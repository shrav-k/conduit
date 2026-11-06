//! Evaluation spike: can Stateright check Conduit's outcome and recovery promises?
//!
//! The model is one revision-checked control change with an external effect:
//!
//! ```text
//! caller --Change{op, expected_rev}--> authority --Apply{op}--> provider
//! caller <--Outcome{op, ..}---------- authority <--Applied{op}-- provider
//! ```
//!
//! Two callers each submit one change from revision 0. Messages can be lost, delayed and
//! reordered. Any one actor can crash and recover from its saved storage. A caller retries
//! with the same operation ID. The provider counts how many times each effect was applied.
//!
//! Each variant changes one rule of the authority or provider. The checker explores every
//! reachable state and reports whether a promise can be broken, with the shortest trace.

use stateright::actor::{
    model_timeout, Actor, ActorModel, ActorModelAction, ActorModelState, Id, LossyNetwork, Network, Out,
};
use stateright::{Checker, Expectation, HasDiscoveries, Model};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::time::Instant;

type Op = u8;

/// What the authority does when a caller retries a change whose effect is still unconfirmed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum OnUncertain {
    /// Ask the provider. Confirm if it applied the effect; otherwise stay uncertain.
    InspectOnly,
    /// Ask the provider. Confirm if it applied the effect; otherwise send the effect again.
    InspectThenReissue,
    /// Send the effect again without asking.
    BlindReplay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Rules {
    on_uncertain: OnUncertain,
    /// Record the intent to dispatch in storage before sending the effect.
    durable_intent: bool,
    /// The provider ignores a second request carrying an operation ID it has already applied.
    provider_dedups: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Outcome {
    Confirmed { rev: u8 },
    Conflict,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Msg {
    Change { op: Op, expected_rev: u8 },
    Outcome { op: Op, outcome: Outcome },
    Apply { op: Op },
    Applied { op: Op },
    Query { op: Op },
    Ledger { op: Op, applied: bool },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Timer {
    Retry,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct AuthorityState {
    rev: u8,
    results: BTreeMap<Op, Outcome>,
    /// The one change whose effect has been dispatched but not confirmed, and who asked.
    in_flight: Option<(Op, Id)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum State {
    Caller { outcome: Option<Outcome>, retries_left: u8 },
    Authority(AuthorityState),
    Provider { applied: BTreeMap<Op, u8> },
}

#[derive(Clone)]
enum Node {
    Caller { op: Op, authority: Id },
    Authority { provider: Id, rules: Rules },
    Provider { rules: Rules },
}

const RETRIES: u8 = 2;

impl Actor for Node {
    type Msg = Msg;
    type Timer = Timer;
    type State = State;
    type Storage = State;
    type Random = ();

    fn on_start(&self, _id: Id, storage: &Option<State>, o: &mut Out<Self>) -> State {
        match self {
            Node::Caller { op, authority } => match storage {
                Some(saved) => {
                    if let State::Caller { outcome: None, retries_left } = saved {
                        if *retries_left > 0 {
                            o.set_timer(Timer::Retry, model_timeout());
                        }
                    }
                    saved.clone()
                }
                None => {
                    o.send(*authority, Msg::Change { op: *op, expected_rev: 0 });
                    o.set_timer(Timer::Retry, model_timeout());
                    let state = State::Caller { outcome: None, retries_left: RETRIES };
                    o.save(state.clone());
                    state
                }
            },
            Node::Authority { .. } => storage.clone().unwrap_or(State::Authority(AuthorityState::default())),
            Node::Provider { .. } => storage.clone().unwrap_or(State::Provider { applied: BTreeMap::new() }),
        }
    }

    fn on_msg(&self, _id: Id, state: &mut Cow<State>, src: Id, msg: Msg, o: &mut Out<Self>) {
        match (self, msg) {
            (Node::Caller { op, .. }, Msg::Outcome { op: for_op, outcome }) => {
                if let State::Caller { outcome: None, retries_left } = **state {
                    if for_op == *op {
                        let next = State::Caller { outcome: Some(outcome), retries_left };
                        o.cancel_timer(Timer::Retry);
                        o.save(next.clone());
                        *state.to_mut() = next;
                    }
                }
            }

            (Node::Authority { provider, rules }, msg) => {
                let State::Authority(current) = &**state else { return };
                let mut next = current.clone();
                match msg {
                    Msg::Change { op, expected_rev } => {
                        if let Some(outcome) = next.results.get(&op) {
                            // Already decided: return the recorded result, never re-apply.
                            o.send(src, Msg::Outcome { op, outcome: *outcome });
                        } else if let Some((pending, _)) = next.in_flight {
                            if pending == op {
                                match rules.on_uncertain {
                                    OnUncertain::BlindReplay => o.send(*provider, Msg::Apply { op }),
                                    _ => o.send(*provider, Msg::Query { op }),
                                }
                            }
                            // A different change waits its turn; its caller retries.
                        } else if expected_rev != next.rev {
                            next.results.insert(op, Outcome::Conflict);
                            o.send(src, Msg::Outcome { op, outcome: Outcome::Conflict });
                        } else {
                            next.in_flight = Some((op, src));
                            o.send(*provider, Msg::Apply { op });
                        }
                    }
                    Msg::Applied { op } | Msg::Ledger { op, applied: true } => {
                        if let Some((pending, caller)) = next.in_flight {
                            if pending == op {
                                next.rev += 1;
                                let outcome = Outcome::Confirmed { rev: next.rev };
                                next.results.insert(op, outcome);
                                next.in_flight = None;
                                o.send(caller, Msg::Outcome { op, outcome });
                            }
                        }
                    }
                    Msg::Ledger { op, applied: false } => {
                        if let Some((pending, _)) = next.in_flight {
                            if pending == op && rules.on_uncertain == OnUncertain::InspectThenReissue {
                                o.send(*provider, Msg::Apply { op });
                            }
                        }
                    }
                    _ => {}
                }
                if next != *current {
                    let mut saved = next.clone();
                    if !rules.durable_intent {
                        saved.in_flight = None;
                    }
                    o.save(State::Authority(saved));
                    *state.to_mut() = State::Authority(next);
                }
            }

            (Node::Provider { rules }, msg) => {
                let State::Provider { applied } = &**state else { return };
                match msg {
                    Msg::Apply { op } => {
                        let count = applied.get(&op).copied().unwrap_or(0);
                        if count == 0 || !rules.provider_dedups {
                            let mut applied = applied.clone();
                            applied.insert(op, count + 1);
                            let next = State::Provider { applied };
                            o.save(next.clone());
                            *state.to_mut() = next;
                        }
                        o.send(src, Msg::Applied { op });
                    }
                    Msg::Query { op } => {
                        let was_applied = applied.get(&op).copied().unwrap_or(0) > 0;
                        o.send(src, Msg::Ledger { op, applied: was_applied });
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }

    fn name(&self) -> String {
        match self {
            Node::Caller { op, .. } => format!("caller-{op}"),
            Node::Authority { .. } => "authority".into(),
            Node::Provider { .. } => "provider".into(),
        }
    }

    fn on_timeout(&self, _id: Id, state: &mut Cow<State>, _timer: &Timer, o: &mut Out<Self>) {
        if let (Node::Caller { op, authority }, State::Caller { outcome: None, retries_left }) = (self, &**state) {
            if *retries_left > 0 {
                let next = State::Caller { outcome: None, retries_left: retries_left - 1 };
                o.send(*authority, Msg::Change { op: *op, expected_rev: 0 });
                if retries_left - 1 > 0 {
                    o.set_timer(Timer::Retry, model_timeout());
                }
                o.save(next.clone());
                *state.to_mut() = next;
            }
        }
    }
}

type Sys = ActorModelState<Node, ()>;

fn authority(sys: &Sys) -> &AuthorityState {
    sys.actor_states
        .iter()
        .find_map(|s| if let State::Authority(a) = &**s { Some(a) } else { None })
        .expect("model has an authority")
}

fn ledger(sys: &Sys) -> &BTreeMap<Op, u8> {
    sys.actor_states
        .iter()
        .find_map(|s| if let State::Provider { applied } = &**s { Some(applied) } else { None })
        .expect("model has a provider")
}

fn caller_outcomes(model: &ActorModel<Node, Rules, ()>, sys: &Sys) -> Vec<(Op, Option<Outcome>)> {
    model
        .actors
        .iter()
        .zip(sys.actor_states.iter())
        .filter_map(|(node, state)| match (node, &**state) {
            (Node::Caller { op, .. }, State::Caller { outcome, .. }) => Some((*op, *outcome)),
            _ => None,
        })
        .collect()
}

const NO_DUPLICATE_EFFECT: &str = "an effect is applied at most once per operation";
const ONE_WINNER: &str = "at most one change is confirmed from one revision";
const CONFIRMED_IMPLIES_EFFECT: &str = "a confirmed change has an applied effect";
const CALLER_MATCHES_RECORD: &str = "a caller never holds an outcome the authority did not record";
const SOME_CONFIRMED: &str = "a change can be confirmed to its caller";
const SOME_CONFLICT: &str = "a stale caller can be told of the conflict";

fn build(rules: Rules, callers: u8, max_crashes: usize) -> ActorModel<Node, Rules, ()> {
    let authority_id = Id::from(0usize);
    let provider_id = Id::from(1usize);
    let mut model = ActorModel::new(rules, ())
        .actor(Node::Authority { provider: provider_id, rules })
        .actor(Node::Provider { rules });
    for op in 1..=callers {
        model = model.actor(Node::Caller { op, authority: authority_id });
    }
    model
        // Each send is delivered at most once, in any order, and may be lost.
        .init_network(Network::new_unordered_nonduplicating([]))
        .lossy_network(LossyNetwork::Yes)
        .max_crashes(max_crashes)
        .property(Expectation::Always, NO_DUPLICATE_EFFECT, |_, sys| ledger(sys).values().all(|n| *n <= 1))
        .property(Expectation::Always, ONE_WINNER, |_, sys| {
            authority(sys).results.values().filter(|o| matches!(o, Outcome::Confirmed { .. })).count() <= 1
        })
        .property(Expectation::Always, CONFIRMED_IMPLIES_EFFECT, |_, sys| {
            authority(sys)
                .results
                .iter()
                .all(|(op, o)| !matches!(o, Outcome::Confirmed { .. }) || ledger(sys).get(op).copied().unwrap_or(0) >= 1)
        })
        .property(Expectation::Always, CALLER_MATCHES_RECORD, |model, sys| {
            caller_outcomes(model, sys)
                .into_iter()
                .all(|(op, held)| held.is_none() || authority(sys).results.get(&op).copied() == held)
        })
        .property(Expectation::Sometimes, SOME_CONFIRMED, |model, sys| {
            caller_outcomes(model, sys).into_iter().any(|(_, o)| matches!(o, Some(Outcome::Confirmed { .. })))
        })
        .property(Expectation::Sometimes, SOME_CONFLICT, |model, sys| {
            caller_outcomes(model, sys).into_iter().any(|(_, o)| o == Some(Outcome::Conflict))
        })
}

struct Variant {
    name: &'static str,
    rules: Rules,
    /// The safety promise this variant is expected to break, if any.
    expect_broken: Option<&'static str>,
}

fn variants() -> Vec<Variant> {
    use OnUncertain::*;
    vec![
        Variant {
            name: "inspect-only, durable intent, plain provider",
            rules: Rules { on_uncertain: InspectOnly, durable_intent: true, provider_dedups: false },
            expect_broken: None,
        },
        Variant {
            name: "inspect-then-reissue, durable intent, deduplicating provider",
            rules: Rules { on_uncertain: InspectThenReissue, durable_intent: true, provider_dedups: true },
            expect_broken: None,
        },
        Variant {
            name: "inspect-then-reissue, durable intent, plain provider",
            rules: Rules { on_uncertain: InspectThenReissue, durable_intent: true, provider_dedups: false },
            expect_broken: Some(NO_DUPLICATE_EFFECT),
        },
        Variant {
            name: "blind replay, durable intent, plain provider",
            rules: Rules { on_uncertain: BlindReplay, durable_intent: true, provider_dedups: false },
            expect_broken: Some(NO_DUPLICATE_EFFECT),
        },
        Variant {
            name: "inspect-only, intent not saved, plain provider",
            rules: Rules { on_uncertain: InspectOnly, durable_intent: false, provider_dedups: false },
            expect_broken: Some(NO_DUPLICATE_EFFECT),
        },
    ]
}

struct Report {
    unique_states: usize,
    max_depth: usize,
    seconds: f64,
    broken: Vec<&'static str>,
    missing_witness: Vec<&'static str>,
}

fn describe(model: &ActorModel<Node, Rules, ()>, action: &ActorModelAction<Msg, Timer, ()>) -> String {
    let who = |id: &Id| model.actors[usize::from(*id)].name();
    match action {
        ActorModelAction::Deliver { src, dst, msg } => format!("{} -> {}: {msg:?}", who(src), who(dst)),
        ActorModelAction::Drop(envelope) => {
            format!("LOST {} -> {}: {:?}", who(&envelope.src), who(&envelope.dst), envelope.msg)
        }
        ActorModelAction::Timeout(id, _) => format!("{} retries", who(id)),
        ActorModelAction::Crash(id) => format!("{} CRASHES", who(id)),
        ActorModelAction::Recover(id) => format!("{} recovers from storage", who(id)),
        ActorModelAction::SelectRandom { actor, .. } => format!("{} makes a random choice", who(actor)),
    }
}

/// The shortest trace to the first broken promise.
///
/// A run to completion keeps overwriting a broken promise's trace with later violating states,
/// so it reports a long one. Stopping at the first failure, breadth-first on one thread, gives
/// the shortest.
fn shortest_failure(rules: Rules, callers: u8, max_crashes: usize) -> Option<(&'static str, Vec<String>)> {
    let checker = build(rules, callers, max_crashes)
        .checker()
        .threads(1)
        .finish_when(HasDiscoveries::AnyFailures)
        .spawn_bfs()
        .join();
    let model = checker.model();
    checker.discoveries().into_iter().find_map(|(name, path)| {
        let is_failure = model.properties().iter().any(|p| p.name == name && p.expectation == Expectation::Always);
        is_failure.then(|| (name, path.into_actions().iter().map(|a| describe(model, a)).collect()))
    })
}

/// Explores the whole state space and reports every broken promise and every unreached witness.
fn check(rules: Rules, callers: u8, max_crashes: usize, threads: usize) -> Report {
    let started = Instant::now();
    let checker = build(rules, callers, max_crashes).checker().threads(threads).spawn_bfs().join();
    let seconds = started.elapsed().as_secs_f64();
    let discoveries = checker.discoveries();
    let mut broken = Vec::new();
    let mut missing_witness = Vec::new();
    for property in checker.model().properties() {
        match (property.expectation, discoveries.get(property.name)) {
            (Expectation::Always, Some(_)) => broken.push(property.name),
            (Expectation::Sometimes, None) => missing_witness.push(property.name),
            _ => {}
        }
    }
    Report { unique_states: checker.unique_state_count(), max_depth: checker.max_depth(), seconds, broken, missing_witness }
}

/// Usage: stateright-evaluation [callers] [max-crashed-at-once] [threads]
fn main() {
    let mut args = std::env::args().skip(1).map(|a| a.parse::<usize>().expect("arguments are numbers"));
    let callers = args.next().unwrap_or(2) as u8;
    let max_crashes = args.next().unwrap_or(1);
    let threads = args.next().unwrap_or(1);
    println!(
        "Stateright evaluation: {callers} callers, up to {max_crashes} crashed at once, {threads} thread(s), lossy unordered network\n"
    );
    for variant in variants() {
        let report = check(variant.rules, callers, max_crashes, threads);
        println!("== {}", variant.name);
        println!("   {} unique states, depth {}, {:.2}s", report.unique_states, report.max_depth, report.seconds);
        if report.broken.is_empty() {
            println!("   all safety promises hold");
        }
        for name in &report.broken {
            println!("   BROKEN: {name}");
        }
        if let Some((name, trace)) = shortest_failure(variant.rules, callers, max_crashes) {
            println!("   shortest trace to a broken promise ({name}):");
            for (step, action) in trace.iter().enumerate() {
                println!("     {:>2}. {action}", step + 1);
            }
        }
        for name in &report.missing_witness {
            println!("   NEVER REACHED: {name}");
        }
        let expected = match variant.expect_broken {
            None => report.broken.is_empty(),
            Some(name) => report.broken.contains(&name),
        };
        println!("   matches expectation: {expected}\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every variant behaves as predicted: the sound rule sets keep every promise and each
    /// unsound rule set breaks the duplicate-effect promise.
    #[test]
    fn variants_match_expectations() {
        for variant in variants() {
            let report = check(variant.rules, 2, 1, 1);
            match variant.expect_broken {
                None => assert!(report.broken.is_empty(), "{}: unexpected break {:?}", variant.name, report.broken),
                Some(name) => {
                    assert!(report.broken.contains(&name), "{}: expected a break of '{name}'", variant.name)
                }
            }
            assert!(report.missing_witness.is_empty(), "{}: never reached {:?}", variant.name, report.missing_witness);
        }
    }
}
