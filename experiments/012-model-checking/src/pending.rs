//! MC9a reference models: pending actions and sign-in as written.
use crate::harness::{Config, Fault, Harness, NamedProperty, Node, Protocol, State, System};
use stateright::Property;
use std::time::Duration;

const Q: usize = 0;
const E: usize = 1;
const H: usize = 2;

fn config(name: &'static str, fault: Option<Fault>, timer_reason: &'static str) -> Config {
    let mut config = Config {
        name,
        nodes: vec!["Q/requests", "E/engine", "H/instance"],
        budgets: [0; 9],
        disabled_reasons: [
            Some("reordering isolated in ReorderQ"),
            Some("loss isolated in LossQ"),
            Some("duplication isolated in CopyQ"),
            Some("a cut and its repair are MC4's"),
            Some("no cut to heal"),
            Some("a crash of E isolated in Crash; a crash of H is not modelled"),
            Some("no crash to recover from"),
            Some("a lost node is MC4's"),
            Some(timer_reason),
        ],
        messages_per_link: 8,
        unique_state_ceiling: 400_000,
        timeout: Duration::from_secs(15),
        bounds: "MC9a: three nodes Q, E and H; at most 8 messages per directed link; finite scripts and one-use local choices; see the README's MC9a pre-registration",
    };
    for fault in fault
        .into_iter()
        .chain((fault == Some(Fault::Crash)).then_some(Fault::Recovery))
    {
        config.budgets[fault as usize] = 1;
        config.disabled_reasons[fault as usize] = None;
    }
    config
}

fn initial<P: Protocol>(ledger: P::Ledger) -> System<P>
where
    P::Node: Default,
{
    State {
        nodes: (0..3)
            .map(|_| Node {
                data: P::Node::default(),
                status: crate::harness::Status::Running,
            })
            .collect(),
        links: vec![Vec::new(); 9],
        cut: Default::default(),
        used: [0; 9],
        ledger,
    }
}

fn fault_allowed(fault: Fault, src: usize, dst: usize) -> bool {
    match fault {
        Fault::Loss | Fault::Duplication => matches!((src, dst), (Q, E) | (E, Q)),
        Fault::Reorder => (src, dst) == (Q, E),
        Fault::Crash | Fault::Recovery | Fault::Timer => (src, dst) == (E, E),
        Fault::Cut | Fault::Heal | Fault::LostMachine => false,
    }
}

fn bracket_on_link<M>(link: &[M], stage: impl Fn(&M) -> Option<u8>) -> bool {
    matches!(link, [a, b] if stage(a).is_some() && stage(a) == stage(b))
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Caller {
    R,
    X,
    Y,
    G,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResultKind {
    Succeeded,
    Failed,
    Unknown,
    Refused,
    Invalid,
    Conflict,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Restart {
    crashed: bool,
    recovered: bool,
}

// The first protocol step after either harness fault logs it before doing its own work.
fn restart_events(restart: &mut Restart) -> Restart {
    std::mem::take(restart)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Attempt {
    key: u8,
    number: u8,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Script {
    stage: usize,
    sent: u16,
    replied: u16,
}
impl Script {
    fn unsent(self, key: u8) -> bool {
        self.sent & (1 << key) == 0
    }

    fn sent(&mut self, key: u8) {
        self.sent |= 1 << key;
    }

    fn reply(&mut self, key: u8) {
        self.replied |= 1 << key;
    }

    fn has_reply(self, key: u8) -> bool {
        self.replied & (1 << key) != 0
    }

    fn advance(&mut self, stages: &[&[u8]]) {
        if stages
            .get(self.stage)
            .is_some_and(|stage| !stage.is_empty() && stage.iter().all(|&key| self.has_reply(key)))
        {
            self.stage += 1;
        }
    }
}

fn drained<P: Protocol>(s: &System<P>) -> bool {
    s.links.iter().all(Vec::is_empty)
}

type Condition<P> = fn(&Harness<P>, &System<P>) -> bool;

fn properties<P: Protocol>(
    safety: &[&'static str],
    safety_promises: &[&'static [&'static str]],
    reach: &[&'static str],
    reach_promises: &'static [&'static str],
    conditions: &[Condition<P>],
) -> Vec<NamedProperty<P>> {
    assert_eq!(safety.len() + reach.len(), conditions.len());
    safety
        .iter()
        .zip(safety_promises)
        .zip(conditions)
        .map(|((&name, &promises), &condition)| NamedProperty {
            property: Property::always(name, condition),
            promises,
        })
        .chain(
            reach
                .iter()
                .zip(&conditions[safety.len()..])
                .map(|(&name, &condition)| NamedProperty {
                    property: Property::sometimes(name, condition),
                    promises: reach_promises,
                }),
        )
        .collect()
}

macro_rules! ledger_predicates {
    ($($safety:ident),+; $($reach:ident),+; $quiescent:expr $(; $($tail:ident),+)?) => {
        &[
            $(|_, state| $safety(&state.ledger)),+,
            $(|_, state| $reach(&state.ledger)),+,
            $quiescent,
            $($(|_, state| $tail(&state.ledger)),+,)?
        ]
    };
}

pub mod actions {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Configuration {
        Base,
        LossQ,
        ReorderQ,
        CopyQ,
        Crash,
    }
    impl Configuration {
        pub const ALL: [Self; 5] = [
            Self::Base,
            Self::LossQ,
            Self::ReorderQ,
            Self::CopyQ,
            Self::Crash,
        ];
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Variant {
        Correct,
        WrongScope,
        AgentSees,
        VolatileScope,
        SkipRaisedAt,
        AnswerTwice,
        AnswerExpired,
        VolatileAnswers,
        ForgetOnPurge,
        RecomputeDismissed,
    }
    impl Variant {
        pub const BROKEN: [Self; 9] = [
            Self::WrongScope,
            Self::AgentSees,
            Self::VolatileScope,
            Self::SkipRaisedAt,
            Self::AnswerTwice,
            Self::AnswerExpired,
            Self::VolatileAnswers,
            Self::ForgetOnPurge,
            Self::RecomputeDismissed,
        ];

        pub fn configuration(self) -> Configuration {
            match self {
                Self::Correct => panic!("Correct has no single registered row"),
                Self::VolatileScope | Self::VolatileAnswers => Configuration::Crash,
                Self::AnswerTwice => Configuration::CopyQ,
                _ => Configuration::Base,
            }
        }

        pub fn target(self) -> &'static str {
            match self {
                Self::Correct => panic!("Correct has no target"),
                Self::WrongScope | Self::AgentSees | Self::VolatileScope => SAFETY[0],
                Self::SkipRaisedAt => SAFETY[1],
                Self::AnswerTwice | Self::AnswerExpired | Self::VolatileAnswers => SAFETY[2],
                Self::ForgetOnPurge => SAFETY[3],
                Self::RecomputeDismissed => SAFETY[4],
            }
        }
    }

    pub const SAFETY: [&str; 5] = ["MC9a S02", "MC9a A09", "MC9a A03", "MC9a D12", "MC9a U06"];
    pub const REACH: [&str; 15] = [
        "MC9a setup-action",
        "MC9a agent-sees-none",
        "MC9a agent-open-refused",
        "MC9a scoped-listed",
        "MC9a unscoped-empty",
        "MC9a machine-at-root",
        "MC9a unscoped-refused",
        "MC9a invalid-answer",
        "MC9a scope-closed",
        "MC9a retried",
        "MC9a answer-then-expiry",
        "MC9a expiry-then-answer",
        "MC9a purge-then-conflict",
        "MC9a repeat-answered",
        "MC9a quiescent",
    ];
    const PROMISES: [&str; 6] = [
        "PND-002", "PND-008", "PND-009", "PND-003", "PND-012", "PND-006",
    ];

    #[derive(Clone)]
    pub struct Actions {
        pub variant: Variant,
    }

    pub fn model(v: Variant, c: Configuration) -> Harness<Actions> {
        use super::actions::Configuration::*;
        let (name, fault) = match c {
            Base => ("MC9a actions Base", None),
            LossQ => ("MC9a actions LossQ", Some(Fault::Loss)),
            ReorderQ => ("MC9a actions ReorderQ", Some(Fault::Reorder)),
            CopyQ => ("MC9a actions CopyQ", Some(Fault::Duplication)),
            Crash => ("MC9a actions Crash", Some(Fault::Crash)),
        };
        Harness::new(
            config(
                name,
                fault,
                "no timer in this part: DL18's end is sign-in's",
            ),
            Actions { variant: v },
        )
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Operation {
        Setup,
        List,
        OpenCandidate,
        Answer { public_id: u8, dismissed: bool },
        CloseInspection,
        Purge,
    }

    use super::Caller::{G, R, X, Y};
    use super::actions::Operation::{Answer, CloseInspection, List, OpenCandidate, Purge, Setup};
    const U1: u8 = 0;
    const U2: u8 = 10;
    const STAGES: &[&[u8]] = &[
        &[U1],
        &[1],
        &[2],
        &[3, 4],
        &[5],
        &[6],
        &[7],
        &[8],
        &[9],
        &[11],
        &[U2],
        &[12],
        &[13],
        &[14],
        &[15],
    ];
    const REQUESTS: [(Caller, Operation); 16] = [
        (R, Setup),
        (G, List),
        (G, OpenCandidate),
        (X, List),
        (Y, List),
        (R, List),
        (
            Y,
            Answer {
                public_id: U1,
                dismissed: true,
            },
        ),
        (
            X,
            Answer {
                public_id: U1,
                dismissed: false,
            },
        ),
        (R, CloseInspection),
        (X, List),
        (R, Setup),
        (
            X,
            Answer {
                public_id: U1,
                dismissed: true,
            },
        ),
        (
            R,
            Answer {
                public_id: U2,
                dismissed: true,
            },
        ),
        (R, Purge),
        (
            R,
            Answer {
                public_id: U2,
                dismissed: true,
            },
        ),
        (R, List),
    ];

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Identity {
        Machine,
        Setup { invocation: u8, dispatch: u8 },
    }
    impl From<Attempt> for Identity {
        fn from(id: Attempt) -> Self {
            Self::Setup {
                invocation: id.key,
                dispatch: id.number,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum AnswerKind {
        Dismissed,
        Retried,
        Expired,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Decision {
        Dispatch(Attempt),
        Reason(ResultKind, Option<&'static str>, Option<Identity>),
        Repeat(Attempt, ResultKind),
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Conduit {
        XInspection,
        AnswerFleet(Caller),
        AgentInspectFleet,
        AgentOpenAccessBound,
        Candidate,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct Listing {
        machine: bool,
        setup: Option<Identity>,
    }
    impl Listing {
        fn count(self) -> Option<u8> {
            let count = u8::from(self.machine) + u8::from(self.setup.is_some());
            (count != 0).then_some(count)
        }
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Event {
        Sent(u8),
        Opened(Conduit),
        Closed(Conduit),
        Decided(u8, Decision),
        Ended(Attempt, ResultKind),
        PartDispatched(Attempt),
        PartResult(Attempt, ResultKind, bool),
        Raised(Identity),
        Answered(Identity, AnswerKind, Option<Attempt>),
        Expired(Identity),
        ExpiryFinished,
        Result(u8, ResultKind, Option<Listing>, Option<u8>),
        Purged,
        Crashed,
        Recovered,
    }
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Ledger(Vec<Event>);

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct Invocation {
        id: Attempt,
        end: Option<ResultKind>,
        retained_list: Option<Listing>,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct SetupEnding {
        identity: Identity,
        inspection_at_ending: bool,
    }
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Data {
        script: Script,
        restart: Restart,
        inspection_open: bool,
        scope_index_lost: bool,
        invocations: Vec<Invocation>,
        endings: Vec<SetupEnding>,
        answers: Vec<(Identity, AnswerKind)>,
        records: Vec<Attempt>,
        expired: Option<Identity>,
    }
    impl Data {
        fn visible(&self, caller: Caller, for_list: bool, v: Variant) -> Listing {
            let setup = self.endings.last().and_then(|ending| {
                let identity = ending.identity;
                let answer = self.answer(identity);
                let recomputed = for_list
                    && v == Variant::RecomputeDismissed
                    && answer == Some(AnswerKind::Dismissed);
                let scoped = caller == X
                    && if for_list && v == Variant::WrongScope {
                        ending.inspection_at_ending
                    } else {
                        self.inspection_open && !(for_list && self.scope_index_lost)
                    };
                ((answer.is_none() || recomputed)
                    && (caller == R
                        || scoped
                        || (for_list && caller == G && v == Variant::AgentSees)))
                    .then_some(identity)
            });
            Listing {
                machine: caller == R,
                setup,
            }
        }

        fn answer(&self, id: Identity) -> Option<AnswerKind> {
            self.answers
                .iter()
                .rev()
                .find(|(identity, _)| *identity == id)
                .map(|(_, a)| *a)
                .or_else(|| (self.expired == Some(id)).then_some(AnswerKind::Expired))
        }

        fn resolve(&self, public_id: u8) -> Option<Identity> {
            self.endings.iter().rev().find_map(|ending| {
                matches!(ending.identity, Identity::Setup { invocation, .. } if invocation == public_id)
                    .then_some(ending.identity)
            })
        }

        fn lose_answers(&mut self) {
            self.answers.clear();
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Message {
        Request(u8),
        Reply(u8),
        ReplyRepeat,
        Setup(Attempt),
        SetupFailed(Attempt),
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Local {
        Send(u8),
        Expire(Identity),
        RecordRestart,
    }

    impl Actions {
        fn tests_raised_at(&self) -> bool {
            self.variant != Variant::SkipRaisedAt
        }

        fn emit(
            &self,
            c: &Config,
            s: &mut System<Self>,
            key: u8,
            outcome: ResultKind,
            list: Option<Listing>,
        ) {
            let caller = REQUESTS[key as usize].0;
            let count = s.nodes[E].data.visible(caller, false, self.variant).count();
            s.ledger.0.push(Event::Result(key, outcome, list, count));
            s.send(c, E, Q, Message::Reply(key));
        }

        fn dispatch(s: &mut System<Self>, key: u8) -> Attempt {
            let e = &mut s.nodes[E].data;
            let id = Attempt {
                key,
                number: e.invocations.len() as u8,
            };
            e.invocations.push(Invocation {
                id,
                end: None,
                retained_list: None,
            });
            s.ledger.0.push(Event::Decided(key, Decision::Dispatch(id)));
            id
        }

        fn end(
            &self,
            c: &Config,
            s: &mut System<Self>,
            id: Attempt,
            outcome: ResultKind,
            list: Option<Listing>,
        ) {
            let e = &mut s.nodes[E].data;
            let inv = &mut e.invocations[id.number as usize];
            inv.end = Some(outcome);
            inv.retained_list = list;
            e.records.push(id);
            if REQUESTS[id.key as usize].1 == Setup {
                let identity = id.into();
                e.endings.push(SetupEnding {
                    identity,
                    inspection_at_ending: e.inspection_open,
                });
                s.ledger.0.push(Event::Raised(identity));
            }
            s.ledger.0.push(Event::Ended(id, outcome));
            self.emit(c, s, id.key, outcome, list);
        }

        fn record_answer(
            s: &mut System<Self>,
            id: Identity,
            kind: AnswerKind,
            actor: Option<Attempt>,
        ) {
            s.nodes[E].data.answers.push((id, kind));
            s.ledger.0.push(Event::Answered(id, kind, actor));
        }

        fn request(&self, c: &Config, s: &mut System<Self>, key: u8) {
            let (caller, op) = REQUESTS[key as usize];
            let e = &s.nodes[E].data;
            let identity = match op {
                Answer { public_id, .. } => e.resolve(public_id),
                _ => None,
            };
            let raised_at =
                identity.is_some() && (caller == R || (caller == X && e.inspection_open));
            let reason = match op {
                OpenCandidate => Some((ResultKind::Refused, "PND-009")),
                Answer { .. } if self.tests_raised_at() && !raised_at => {
                    Some((ResultKind::Refused, "PND-009"))
                }
                Answer {
                    dismissed: false, ..
                } => Some((ResultKind::Invalid, "PND-009")),
                _ => None,
            };
            if let Some((reason, rule)) = reason {
                s.ledger.0.push(Event::Decided(
                    key,
                    Decision::Reason(reason, Some(rule), None),
                ));
                self.emit(c, s, key, reason, None);
                return;
            }

            let prior = e
                .invocations
                .iter()
                .rev()
                .find(|i| i.id.key == key)
                .copied();
            let answer_twice = self.variant == Variant::AnswerTwice
                && matches!(op, Answer { .. })
                && prior.is_some_and(|i| i.end == Some(ResultKind::Succeeded));
            if let Some(prior) = prior.filter(|_| !answer_twice) {
                let repeat = match prior.end {
                    None => Some(Decision::Reason(
                        ResultKind::Conflict,
                        Some("OPS-032"),
                        None,
                    )),
                    Some(result @ (ResultKind::Succeeded | ResultKind::Unknown)) => {
                        Some(Decision::Repeat(prior.id, result))
                    }
                    _ => None,
                };
                if let Some(repeat) = repeat {
                    s.ledger.0.push(Event::Decided(key, repeat));
                    // A conflict copy cannot answer the script's still-outstanding original request.
                    if let Some(outcome) = prior.end {
                        self.emit(c, s, key, outcome, prior.retained_list);
                    } else {
                        let count = e.visible(caller, false, self.variant).count();
                        s.ledger
                            .0
                            .push(Event::Result(key, ResultKind::Conflict, None, count));
                        s.send(c, E, Q, Message::ReplyRepeat);
                    }
                    return;
                }
            }

            match op {
                Setup => {
                    let retried = e
                        .endings
                        .last()
                        .map(|end| end.identity)
                        .filter(|&id| e.answer(id).is_none());
                    let id = Self::dispatch(s, key);
                    if let Some(pending) = retried {
                        Self::record_answer(s, pending, AnswerKind::Retried, Some(id));
                    }
                    s.ledger.0.push(Event::PartDispatched(id));
                    s.send(c, E, H, Message::Setup(id));
                }
                List => {
                    let list = e.visible(caller, true, self.variant);
                    let id = Self::dispatch(s, key);
                    self.end(c, s, id, ResultKind::Succeeded, Some(list));
                }
                CloseInspection | Purge => {
                    let id = Self::dispatch(s, key);
                    let e = &mut s.nodes[E].data;
                    if op == CloseInspection {
                        e.inspection_open = false;
                        s.ledger.0.push(Event::Closed(Conduit::XInspection));
                    } else {
                        e.records.clear();
                        if self.variant == Variant::ForgetOnPurge {
                            e.lose_answers();
                        }
                        s.ledger.0.push(Event::Purged);
                    }
                    self.end(c, s, id, ResultKind::Succeeded, None);
                }
                Answer { .. } => {
                    let identity = identity.expect("the script names an already computed setup");
                    let answer = e.answer(identity);
                    if !answer_twice
                        && answer.is_some()
                        && !(self.variant == Variant::AnswerExpired
                            && answer == Some(AnswerKind::Expired))
                    {
                        s.ledger.0.push(Event::Decided(
                            key,
                            Decision::Reason(ResultKind::Conflict, Some("PND-003"), Some(identity)),
                        ));
                        self.emit(c, s, key, ResultKind::Conflict, None);
                    } else {
                        let id = Self::dispatch(s, key);
                        Self::record_answer(s, identity, AnswerKind::Dismissed, Some(id));
                        self.end(c, s, id, ResultKind::Succeeded, None);
                    }
                }
                OpenCandidate => unreachable!(),
            }
        }

        fn sync_restart(&self, c: &Config, s: &mut System<Self>) {
            let restart = restart_events(&mut s.nodes[E].data.restart);
            if restart.crashed {
                s.ledger.0.push(Event::Crashed);
            }
            if restart.recovered {
                s.ledger.0.push(Event::Recovered);
                let open: Vec<_> = s.nodes[E]
                    .data
                    .invocations
                    .iter()
                    .filter(|i| i.end.is_none())
                    .map(|i| i.id)
                    .collect();
                for id in open {
                    self.end(c, s, id, ResultKind::Unknown, None);
                }
            }
        }
    }

    impl Protocol for Actions {
        type Node = Data;
        type Message = Message;
        type Ledger = Ledger;
        type Local = Local;

        fn gate_remaining_faults(&self) -> bool {
            true
        }

        fn fault_allowed(&self, fault: Fault, src: usize, dst: usize) -> bool {
            fault != Fault::Timer && fault_allowed(fault, src, dst)
        }

        fn fault_enabled(&self, s: &System<Self>, fault: Fault) -> bool {
            fault != Fault::Reorder
                || bracket_on_link(&s.links[s.link(Q, E)], |m| {
                    matches!(m, Message::Request(3 | 4)).then_some(3)
                })
        }

        fn initial(&self, _: &Config) -> System<Self> {
            let mut s = initial::<Self>(Ledger(vec![
                Event::Raised(Identity::Machine),
                Event::Opened(Conduit::XInspection),
                Event::Opened(Conduit::AnswerFleet(R)),
                Event::Opened(Conduit::AnswerFleet(X)),
                Event::Opened(Conduit::AnswerFleet(Y)),
                Event::Opened(Conduit::AgentInspectFleet),
                Event::Opened(Conduit::AgentOpenAccessBound),
            ]));
            s.nodes[E].data.inspection_open = true;
            s
        }

        fn receive(&self, c: &Config, s: &mut System<Self>, _: usize, _: usize, m: Message) {
            self.sync_restart(c, s);
            match m {
                Message::Request(key) => self.request(c, s, key),
                Message::Reply(key) => {
                    let q = &mut s.nodes[Q].data.script;
                    q.reply(key);
                    q.advance(STAGES);
                }
                Message::ReplyRepeat => {}
                Message::Setup(id) => s.send(c, H, E, Message::SetupFailed(id)),
                Message::SetupFailed(id) => {
                    let late = s.nodes[E].data.invocations[id.number as usize]
                        .end
                        .is_some();
                    s.ledger
                        .0
                        .push(Event::PartResult(id, ResultKind::Failed, late));
                    if !late {
                        self.end(c, s, id, ResultKind::Failed, None);
                    }
                }
            }
        }

        fn crash(&self, node: &mut Data) {
            node.restart.crashed = true;
            if self.variant == Variant::VolatileScope {
                node.scope_index_lost = true;
            }
            if self.variant == Variant::VolatileAnswers {
                node.lose_answers();
            }
        }

        fn recover(&self, node: &mut Data) {
            node.restart.recovered = true;
        }

        fn timer_enabled(&self, _: &System<Self>, _: usize) -> bool {
            false
        }

        fn timer(&self, _: &Config, _: &mut System<Self>, _: usize) {
            unreachable!()
        }

        fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
            let data = &s.nodes[node].data;
            match node {
                Q => {
                    if let Some(keys) = STAGES.get(data.script.stage) {
                        out.extend(
                            keys.iter()
                                .filter(|&&key| data.script.unsent(key))
                                .copied()
                                .map(Local::Send),
                        );
                    }
                }
                E => {
                    if data.restart.recovered {
                        out.push(Local::RecordRestart);
                    } else if data.expired.is_none() && s.nodes[Q].data.script.stage < STAGES.len()
                    {
                        out.extend(data.endings.iter().filter_map(
                            |ending| match ending.identity {
                                id @ Identity::Setup { invocation: U2, .. } => {
                                    Some(Local::Expire(id))
                                }
                                _ => None,
                            },
                        ));
                    }
                }
                _ => {}
            }
        }

        fn local_step(&self, c: &Config, s: &mut System<Self>, _: usize, action: Local) {
            self.sync_restart(c, s);
            match action {
                Local::Send(key) => {
                    s.nodes[Q].data.script.sent(key);
                    s.ledger.0.push(Event::Sent(key));
                    s.send(c, Q, E, Message::Request(key));
                }
                Local::Expire(id) => {
                    let was_unanswered = s.nodes[E].data.answer(id).is_none();
                    s.nodes[E].data.expired = Some(id);
                    s.ledger.0.push(Event::Expired(id));
                    if was_unanswered {
                        Self::record_answer(s, id, AnswerKind::Expired, None);
                    }
                    s.ledger.0.push(Event::ExpiryFinished);
                }
                Local::RecordRestart => {}
            }
        }

        fn properties(&self) -> Vec<NamedProperty<Self>> {
            properties::<Self>(
                &SAFETY,
                &[
                    &["PND-002", "PND-008"],
                    &["PND-009"],
                    &["PND-003"],
                    &["PND-012"],
                    &["PND-006"],
                ],
                &REACH,
                &PROMISES,
                ledger_predicates!(s02, a09, a03, d12, u06;
                    setup_action, agent_sees_none, agent_open_refused, scoped_listed, unscoped_empty,
                    machine_at_root, unscoped_refused, invalid_answer, scope_closed, retried,
                    answer_then_expiry, expiry_then_answer, purge_then_conflict, repeat_answered;
                    |_, s| s.nodes[Q].data.script.stage == STAGES.len() && drained::<Self>(s)),
            )
        }
    }

    // Reconstruct pending state from the public history. This index lives only during an oracle.
    #[derive(Default)]
    struct History {
        inspection: bool,
        machine_raised: bool,
        succeeded: Vec<Attempt>,
        answer_routes: Vec<Caller>,
        latest_setup: Option<Identity>,
        endings: Vec<Identity>,
        answers: Vec<(Identity, AnswerKind)>,
        expired: Option<Identity>,
    }
    impl History {
        fn observe(&mut self, event: Event) {
            match event {
                Event::Opened(Conduit::XInspection) => self.inspection = true,
                Event::Closed(Conduit::XInspection) => self.inspection = false,
                Event::Opened(Conduit::AnswerFleet(caller)) => self.answer_routes.push(caller),
                Event::Raised(id @ Identity::Setup { .. }) => self.endings.push(id),
                Event::Raised(Identity::Machine) => self.machine_raised = true,
                Event::Ended(id, ResultKind::Succeeded) => self.succeeded.push(id),
                Event::Ended(attempt, ResultKind::Failed | ResultKind::Unknown)
                    if REQUESTS[attempt.key as usize].1 == Setup =>
                {
                    self.latest_setup = Some(attempt.into())
                }
                Event::Answered(id, kind, _) => self.answers.push((id, kind)),
                Event::Expired(id) => self.expired = Some(id),
                _ => {}
            }
        }

        fn raised_at(&self, caller: Caller, id: Option<Identity>) -> bool {
            match id {
                Some(Identity::Machine) => caller == R && self.machine_raised,
                Some(Identity::Setup { .. }) => caller == R || (caller == X && self.inspection),
                None => false,
            }
        }

        fn resolve(&self, public_id: u8) -> Option<Identity> {
            self.endings.iter().rev().copied().find(
                |&id| matches!(id, Identity::Setup { invocation, .. } if invocation == public_id),
            )
        }

        fn answered(&self, id: Identity) -> bool {
            self.expired == Some(id) || self.answers.iter().any(|&(identity, _)| identity == id)
        }

        fn listed(&self, caller: Caller) -> Listing {
            Listing {
                machine: self.raised_at(caller, Some(Identity::Machine)),
                setup: self
                    .latest_setup
                    .filter(|&id| self.raised_at(caller, Some(id)) && !self.answered(id)),
            }
        }

        fn answer_target(&self, key: u8) -> Option<Identity> {
            let (caller, op) = REQUESTS[key as usize];
            let Answer {
                dismissed: true, ..
            } = op
            else {
                return None;
            };
            self.requested_identity(key).filter(|&id| {
                self.answer_routes.contains(&caller) && self.raised_at(caller, Some(id))
            })
        }

        fn requested_identity(&self, key: u8) -> Option<Identity> {
            match REQUESTS[key as usize].1 {
                Answer { public_id, .. } => self.resolve(public_id),
                _ => None,
            }
        }

        fn conflict_or_repeat(&self, key: u8, decision: Decision) -> bool {
            match decision {
                Decision::Reason(ResultKind::Conflict, _, _) => true,
                Decision::Repeat(id, ResultKind::Succeeded) => {
                    id.key == key && self.succeeded.contains(&id)
                }
                _ => false,
            }
        }
    }

    fn s02(ledger: &Ledger) -> bool {
        let mut history = History::default();
        for &event in &ledger.0 {
            if let Event::Result(key, outcome, list, count) = event {
                let (caller, op) = REQUESTS[key as usize];
                let expected = history.listed(caller);
                if count != expected.count()
                    || (op == List && outcome == ResultKind::Succeeded && list != Some(expected))
                {
                    return false;
                }
            }
            history.observe(event);
        }
        true
    }

    fn a09(ledger: &Ledger) -> bool {
        let mut history = History::default();
        for &event in &ledger.0 {
            if let Event::Decided(key, decision) = event {
                let (caller, op) = REQUESTS[key as usize];
                match op {
                    Answer { public_id, .. } => {
                        let raised = history.raised_at(caller, history.resolve(public_id));
                        if (!raised
                            && !matches!(decision, Decision::Reason(ResultKind::Refused, _, None)))
                            || (matches!(decision, Decision::Dispatch(_))
                                && !history.answer_routes.contains(&caller))
                        {
                            return false;
                        }
                    }
                    OpenCandidate
                        if decision
                            != Decision::Reason(ResultKind::Refused, Some("PND-009"), None) =>
                    {
                        return false;
                    }
                    _ => {}
                }
            }
            if event == Event::Opened(Conduit::Candidate) {
                return false;
            }
            history.observe(event);
        }
        true
    }

    fn a03(ledger: &Ledger) -> bool {
        let mut history = History::default();
        let mut expiring = false;
        for &event in &ledger.0 {
            match event {
                Event::Expired(_) => expiring = true,
                Event::ExpiryFinished => expiring = false,
                Event::Decided(_, Decision::Dispatch(_)) | Event::PartDispatched(_) if expiring => {
                    return false;
                }
                Event::Decided(key, decision)
                    if history
                        .answer_target(key)
                        .is_some_and(|id| history.answered(id))
                        && !history.conflict_or_repeat(key, decision) =>
                {
                    return false;
                }
                Event::Answered(id, kind, actor)
                    if history.answers.iter().any(|&(earlier, _)| earlier == id)
                        || (kind == AnswerKind::Expired
                            && (actor.is_some() || history.expired != Some(id)))
                        || (kind == AnswerKind::Dismissed
                            && actor
                                .is_none_or(|a| history.requested_identity(a.key) != Some(id))) =>
                {
                    return false;
                }
                _ => {}
            }
            history.observe(event);
        }
        true
    }

    fn d12(ledger: &Ledger) -> bool {
        let mut history = History::default();
        let mut answered_at_purge = Vec::new();
        for &event in &ledger.0 {
            match event {
                Event::Purged => {
                    answered_at_purge = history.answers.iter().map(|&(id, _)| id).collect()
                }
                Event::Decided(key, decision)
                    if history
                        .answer_target(key)
                        .is_some_and(|id| answered_at_purge.contains(&id))
                        && !history.conflict_or_repeat(key, decision) =>
                {
                    return false;
                }
                Event::Answered(id, _, _) if answered_at_purge.contains(&id) => return false,
                _ => {}
            }
            history.observe(event);
        }
        true
    }

    fn u06(ledger: &Ledger) -> bool {
        let mut history = History::default();
        for (i, &event) in ledger.0.iter().enumerate() {
            match event {
                Event::Result(key, ResultKind::Succeeded, list, _) => {
                    let (caller, op) = REQUESTS[key as usize];
                    if (op == List
                        && caller == R
                        && list.is_none_or(|list| list.setup != history.listed(R).setup))
                        || list.and_then(|list| list.setup).is_some_and(|id| {
                            history.answers.contains(&(id, AnswerKind::Dismissed))
                                || history.expired == Some(id)
                        })
                    {
                        return false;
                    }
                }
                Event::Ended(id, ResultKind::Failed | ResultKind::Unknown)
                    if REQUESTS[id.key as usize].1 == Setup =>
                {
                    if ledger.0[..i].last() != Some(&Event::Raised(id.into())) {
                        return false;
                    }
                }
                Event::Decided(key, Decision::Dispatch(id))
                    if REQUESTS[key as usize].1 == Setup =>
                {
                    if let Some(pending) = history.listed(R).setup
                        && ledger.0.get(i + 1)
                            != Some(&Event::Answered(pending, AnswerKind::Retried, Some(id)))
                    {
                        return false;
                    }
                }
                _ => {}
            }
            history.observe(event);
        }
        true
    }

    fn listed(ledger: &Ledger, key: u8, test: impl Fn(Listing, Option<u8>) -> bool) -> bool {
        ledger.0.iter().any(|event| matches!(*event,
            Event::Result(k, ResultKind::Succeeded, Some(list), count) if k == key && test(list, count)))
    }

    fn setup_action(l: &Ledger) -> bool {
        listed(l, 5, |list, _| {
            matches!(list.setup, Some(Identity::Setup { invocation: U1, .. }))
        })
    }
    fn agent_sees_none(l: &Ledger) -> bool {
        listed(l, 1, |list, count| {
            list.count().is_none() && count.is_none()
        })
    }
    fn agent_open_refused(l: &Ledger) -> bool {
        l.0.contains(&Event::Decided(
            2,
            Decision::Reason(ResultKind::Refused, Some("PND-009"), None),
        )) && !l.0.contains(&Event::Opened(Conduit::Candidate))
    }
    fn scoped_listed(l: &Ledger) -> bool {
        listed(l, 3, |list, count| {
            count == Some(1) && matches!(list.setup, Some(Identity::Setup { invocation: U1, .. }))
        })
    }
    fn unscoped_empty(l: &Ledger) -> bool {
        listed(l, 4, |list, count| {
            list.count().is_none() && count.is_none()
        })
    }
    fn machine_at_root(l: &Ledger) -> bool {
        listed(l, 5, |list, _| list.machine) && listed(l, 3, |list, _| !list.machine)
    }
    fn reason(l: &Ledger, key: u8, reason: ResultKind) -> bool {
        l.0.iter().any(|e| matches!(*e, Event::Decided(k, Decision::Reason(r, _, _)) if k == key && r == reason))
    }
    fn unscoped_refused(l: &Ledger) -> bool {
        reason(l, 6, ResultKind::Refused)
    }
    fn invalid_answer(l: &Ledger) -> bool {
        reason(l, 7, ResultKind::Invalid)
    }
    fn scope_closed(l: &Ledger) -> bool {
        l.0.contains(&Event::Closed(Conduit::XInspection))
            && listed(l, 9, |list, count| {
                list.count().is_none() && count.is_none()
            })
            && reason(l, 11, ResultKind::Refused)
    }
    fn retried(l: &Ledger) -> bool {
        l.0.iter().any(|e| {
            matches!(
                e,
                Event::Answered(
                    Identity::Setup { invocation: U1, .. },
                    AnswerKind::Retried,
                    Some(Attempt { key: U2, .. })
                )
            )
        })
    }
    fn answer_then_expiry(l: &Ledger) -> bool {
        l.0.iter().enumerate().any(|(i, e)| {
            let Event::Expired(id @ Identity::Setup { invocation: U2, .. }) = e else { return false };
            l.0[..i].iter().any(|event| matches!(event, Event::Answered(other, AnswerKind::Dismissed, _) if id == other))
                && !l.0.get(i+1).is_some_and(|event| matches!(event, Event::Answered(other, AnswerKind::Expired, _) if id == other))
        })
    }
    fn expiry_then_answer(l: &Ledger) -> bool {
        let mut history = History::default();
        for &event in &l.0 {
            if let Event::Decided(12, Decision::Reason(ResultKind::Conflict, _, Some(id))) = event
                && history.resolve(U2) == Some(id)
                && history.answers.contains(&(id, AnswerKind::Expired))
                && !history.answers.contains(&(id, AnswerKind::Dismissed))
            {
                return true;
            }
            history.observe(event);
        }
        false
    }
    fn purge_then_conflict(l: &Ledger) -> bool {
        l.0.contains(&Event::Purged) && reason(l, 14, ResultKind::Conflict)
    }
    fn repeat_answered(l: &Ledger) -> bool {
        l.0.iter().any(|e| {
            matches!(*e, Event::Decided(key, Decision::Repeat(id, ResultKind::Succeeded))
            if matches!(REQUESTS[key as usize].1, Answer { .. })
                && l.0.contains(&Event::Ended(id, ResultKind::Succeeded)))
        })
    }

    #[cfg(test)]
    mod census {
        use super::*;
        use crate::measure::{Limits, census};

        #[test]
        #[ignore = "measurement for order 204b; run through its run script"]
        fn census_rows() {
            println!("MC9a measurement test census_rows");
            let rows = Configuration::ALL
                .into_iter()
                .map(|c| (Variant::Correct, c))
                .chain(Variant::BROKEN.into_iter().map(|v| (v, v.configuration())))
                .chain(
                    Variant::BROKEN
                        .into_iter()
                        .filter(|v| v.configuration() != Configuration::Base)
                        .map(|v| (v, Configuration::Base)),
                );
            for (v, c) in rows {
                let measured = census(
                    &model(v, c),
                    &|_, _| {},
                    &|_| 0,
                    &[],
                    Limits {
                        states: 200_000_000,
                        seconds: 180,
                    },
                );
                println!(
                    "MC9a census Actions|{v:?}/{c:?}|{}|{}|{:.1}",
                    measured.states, measured.stop, measured.seconds
                );
            }
        }
    }
}

pub mod sign_in {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Configuration {
        Base,
        LossQ,
        ReorderQ,
        CopyQ,
        Crash,
        Wait,
    }
    impl Configuration {
        pub const ALL: [Self; 6] = [
            Self::Base,
            Self::LossQ,
            Self::ReorderQ,
            Self::CopyQ,
            Self::Crash,
            Self::Wait,
        ];
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Variant {
        Correct,
        NoConflict,
        Unavailable,
        CopyAsNew,
        ConflictForever,
        WrongAccountKept,
        UndoInitial,
        ReturnTwice,
        ReplayState,
        SavedReturn,
        LeakDiagnostic,
        ExpiryLifts,
        RetainHold,
        VolatileHold,
        RaiseTwice,
    }
    impl Variant {
        pub const BROKEN: [Self; 14] = [
            Self::NoConflict,
            Self::Unavailable,
            Self::CopyAsNew,
            Self::ConflictForever,
            Self::WrongAccountKept,
            Self::UndoInitial,
            Self::ReturnTwice,
            Self::ReplayState,
            Self::SavedReturn,
            Self::LeakDiagnostic,
            Self::ExpiryLifts,
            Self::RetainHold,
            Self::VolatileHold,
            Self::RaiseTwice,
        ];
        pub fn configuration(self) -> Configuration {
            match self {
                Self::Correct => panic!("Correct has no single registered row"),
                Self::CopyAsNew | Self::ReplayState => Configuration::CopyQ,
                Self::ConflictForever | Self::SavedReturn | Self::VolatileHold => {
                    Configuration::Crash
                }
                _ => Configuration::Base,
            }
        }
        pub fn target(self) -> &'static str {
            match self {
                Self::Correct => panic!("Correct has no target"),
                Self::NoConflict | Self::Unavailable | Self::CopyAsNew | Self::ConflictForever => {
                    SAFETY[0]
                }
                Self::WrongAccountKept | Self::UndoInitial => SAFETY[1],
                Self::ReturnTwice | Self::ReplayState | Self::SavedReturn => SAFETY[2],
                Self::LeakDiagnostic => SAFETY[3],
                Self::ExpiryLifts | Self::RetainHold | Self::VolatileHold => SAFETY[4],
                Self::RaiseTwice => SAFETY[5],
            }
        }
    }

    pub const SAFETY: [&str; 6] = [
        "MC9a X10", "MC9a K11", "MC9a R13", "MC9a V14", "MC9a H05", "MC9a P01",
    ];
    pub const REACH: [&str; 27] = [
        "MC9a setup-raises",
        "MC9a held",
        "MC9a caller-refused",
        "MC9a conflict-sign-in",
        "MC9a conflict-retirement",
        "MC9a retire-during-sign-in",
        "MC9a return-passed",
        "MC9a already-returned",
        "MC9a wrong-account-undone",
        "MC9a undo-failed",
        "MC9a first-account-kept",
        "MC9a signed-in",
        "MC9a expiry-then-signed-in",
        "MC9a signed-in-then-expiry",
        "MC9a new-after-expiry",
        "MC9a lapse-raises",
        "MC9a void",
        "MC9a restart-ends-sign-in",
        "MC9a restart-overlap",
        "MC9a repeat-conflict",
        "MC9a replay-rejected",
        "MC9a wait-ended",
        "MC9a quiescent",
        "MC9a removed-refused",
        "MC9a wrong-account-preserved",
        "MC9a shutdown-unknown",
        "MC9a undo-interrupted",
    ];
    const PROMISES: [&str; 6] = [
        "PND-010", "PND-011", "PND-013", "PND-014", "PND-005", "PND-001",
    ];

    #[derive(Clone)]
    pub struct SignIn {
        pub variant: Variant,
    }
    pub fn model(v: Variant, c: Configuration) -> Harness<SignIn> {
        use super::sign_in::Configuration::*;
        let (name, fault) = match c {
            Base => ("MC9a sign-in Base", None),
            LossQ => ("MC9a sign-in LossQ", Some(Fault::Loss)),
            ReorderQ => ("MC9a sign-in ReorderQ", Some(Fault::Reorder)),
            CopyQ => ("MC9a sign-in CopyQ", Some(Fault::Duplication)),
            Crash => ("MC9a sign-in Crash", Some(Fault::Crash)),
            Wait => ("MC9a sign-in Wait", Some(Fault::Timer)),
        };
        Harness::new(
            config(
                name,
                fault,
                "DL18's end isolated in Wait; these slices give no other timer",
            ),
            SignIn { variant: v },
        )
    }

    const SETUP: u8 = 0;
    const EARLY_TURN: u8 = 1;
    const Y_SIGN: u8 = 2;
    const R_SIGN: u8 = 3;
    const X_SIGN: u8 = 4;
    const W2: u8 = 5;
    const LATE_TURN: u8 = 6;
    const RETIRE: u8 = 7;
    const FINAL_SIGN: u8 = 8;
    const STAGES: &[&[u8]] = &[
        &[SETUP],
        &[EARLY_TURN],
        &[Y_SIGN],
        &[R_SIGN, X_SIGN],
        &[],
        &[W2],
        &[LATE_TURN],
        &[RETIRE, FINAL_SIGN],
    ];

    use super::Caller::{R, X, Y};
    fn caller(key: u8) -> Caller {
        match key {
            SETUP | R_SIGN | W2 | RETIRE => R,
            Y_SIGN => Y,
            _ => X,
        }
    }
    fn sign_in(key: u8) -> bool {
        matches!(key, Y_SIGN | R_SIGN | X_SIGN | W2 | FINAL_SIGN)
    }
    fn turn(key: u8) -> bool {
        matches!(key, EARLY_TURN | LATE_TURN)
    }
    fn excludes_sign_in(key: u8) -> bool {
        sign_in(key) || matches!(key, SETUP | RETIRE)
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    pub enum Account {
        #[default]
        None,
        LowerE,
        Other,
    }
    impl Account {
        fn expected(self) -> bool {
            match self {
                Self::None => "",
                Self::LowerE => "e",
                Self::Other => "O",
            }
            .eq_ignore_ascii_case("E")
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Lifecycle {
        Registered,
        Ready,
        Retired,
        Removed,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum ActionKind {
        Setup,
        Lapse,
        Replacement,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct ActionId {
        kind: ActionKind,
        number: u8,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Answer {
        SignedIn,
        Void,
        Expired,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Return {
        V1,
        V2,
        V3,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct RequestState {
        id: Attempt,
        round: u8,
    }
    impl RequestState {
        fn value(self) -> Return {
            if self.round == 2 {
                Return::V2
            } else if self.id.key == W2 {
                Return::V3
            } else {
                Return::V1
            }
        }
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Part {
        SetupAccount,
        Setup,
        FirstAccount,
        Login,
        Paste,
        InitialPoll,
        FinalPoll,
        Undo,
        Turn,
        Stop,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum PartResult {
        Account(Account),
        Login,
        Paste,
        Succeeded,
        UndoFailed,
        Lapse,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Interruption {
        Restart,
        Wait,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Detail {
        None,
        Account(Account),
        Wrong {
            expected: &'static str,
            found: Account,
        },
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct Ending {
        outcome: ResultKind,
        detail: Detail,
        interruption: Option<Interruption>,
    }
    impl Ending {
        fn ordinary(outcome: ResultKind) -> Self {
            Self {
                outcome,
                detail: Detail::None,
                interruption: None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct Report {
        reason: ResultKind,
        rule: &'static str,
        action: Option<ActionId>,
        invocation: Option<Attempt>,
        undone: bool,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Decision {
        Dispatch(Attempt),
        Reason(Report),
        Repeat(Attempt, ResultKind),
        Retired,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Conduit {
        SignInFleet(Caller),
        InspectFleet(Caller),
        TurnAgent(Caller),
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum StateCheck {
        Accepted,
        Consumed,
        Ended,
        Mismatch,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Emission {
        Result(u8, Option<Return>),
        Diagnostic(u8, &'static str, Option<Account>, Option<Return>),
        Record(Attempt, Option<Return>),
        ToAgent(Attempt, Option<Return>),
        HookRetry(Attempt, Part, Return),
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Event {
        Opened(Conduit),
        Sent(u8),
        Request(u8),
        Decided(u8, Decision),
        Ended(Attempt, Ending),
        Lifecycle(Lifecycle),
        PartDispatched(Attempt, Part),
        PartResult(Attempt, Part, PartResult, bool),
        LoginStarted(Attempt),
        PersonCompleted(Attempt, Account),
        ProgramTookReturn(Attempt),
        ProgramFinished(Attempt),
        Raised(ActionId),
        Answered(ActionId, Answer, Option<Attempt>),
        Expired(ActionId),
        Issued(RequestState, Caller),
        RetrySent(RequestState, Caller, Return),
        Checked(RequestState, Caller, StateCheck),
        ReturnPassed(RequestState, Part, Return),
        ReturnRefused(RequestState, Option<&'static str>),
        Emitted(Emission),
        Crashed,
        Recovered,
    }

    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Ledger(Vec<Event>);
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    struct Requests {
        script: Script,
        w_ended: Option<ResultKind>,
        input: Option<RequestState>,
        pasted: u32,
    }
    impl Requests {
        fn progress(&mut self) {
            self.script.advance(STAGES);
            if self.script.stage == 4
                && let Some(outcome) = self.w_ended
            {
                self.script.stage = if outcome == ResultKind::Succeeded {
                    6
                } else {
                    5
                };
            }
        }

        fn pasted(self, token: RequestState) -> bool {
            self.pasted & paste_bit(token) != 0
        }

        fn driving(self, token: RequestState) -> bool {
            (self.script.stage == 4 && matches!(token.id.key, R_SIGN | X_SIGN))
                || (self.script.stage == 5 && token.id.key == W2)
        }
    }
    fn paste_bit(token: RequestState) -> u32 {
        1 << (2 * token.id.number + token.round - 1)
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct Action {
        id: ActionId,
        answer: Option<Answer>,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct Invocation {
        id: Attempt,
        end: Option<Ending>,
        initial: Account,
        wrong: Option<Account>,
        issued: u8,
        consumed: u8,
        return_passed: bool,
        pending_return: Option<(RequestState, Return)>,
        can_offer: bool,
        poll_ready: bool,
        initial_poll: bool,
        final_poll: bool,
        waiting: bool,
    }
    impl Invocation {
        fn new(id: Attempt) -> Self {
            Self {
                id,
                end: None,
                initial: Account::None,
                wrong: None,
                issued: 0,
                consumed: 0,
                return_passed: false,
                pending_return: None,
                can_offer: false,
                poll_ready: false,
                initial_poll: false,
                final_poll: false,
                waiting: false,
            }
        }
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct Program {
        id: Attempt,
        choice: Option<Account>,
        took_return: bool,
        waiting: bool,
    }
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Data {
        q: Requests,
        restart: Restart,
        lifecycle: Option<Lifecycle>,
        fleet_inspection: bool,
        invocations: Vec<Invocation>,
        first_sign_in: Option<Attempt>,
        stale_conflict: bool,
        actions: Vec<Action>,
        expired: Option<ActionId>,
        hold: bool,
        undone_false: bool,
        offered_second: bool,
        records: Vec<Attempt>,
        account: Account,
        programs: Vec<Program>,
        undo_job: Option<Attempt>,
        turn_job: Option<Attempt>,
        undo_failed_once: bool,
        lapsed_once: bool,
    }
    impl Data {
        fn inv(&self, id: Attempt) -> &Invocation {
            &self.invocations[id.number as usize]
        }
        fn inv_mut(&mut self, id: Attempt) -> &mut Invocation {
            &mut self.invocations[id.number as usize]
        }
        fn unanswered(&self) -> Option<ActionId> {
            self.actions
                .iter()
                .rev()
                .find(|a| a.answer.is_none())
                .map(|a| a.id)
        }
        fn dispatched_retirement(&self) -> bool {
            matches!(
                self.lifecycle,
                Some(Lifecycle::Retired | Lifecycle::Removed)
            )
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Reply {
        Decision,
        End(ResultKind, bool),
        Input(RequestState, bool),
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Message {
        Request(u8),
        Retry(RequestState, Caller, Return),
        Reply(u8, Reply, bool),
        Hook(Attempt, Part),
        HookRetry(Attempt, Return),
        PartResult(Attempt, Part, PartResult),
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Local {
        Send(u8),
        SendRetry(RequestState),
        CompletePage(Attempt, Account),
        Expire(ActionId),
        PassReturn(Attempt),
        OfferSecond(Attempt, bool),
        ApplyReturn(Attempt),
        Poll(Attempt, Part),
        Undo(Attempt, bool),
        Turn(Attempt, bool),
        RecordRestart,
    }
    impl SignIn {
        fn refuses_overlap(&self) -> bool {
            self.variant != Variant::NoConflict
        }

        fn emit(c: &Config, s: &mut System<Self>, key: u8, reply: Reply, script_reply: bool) {
            s.ledger.0.push(Event::Emitted(Emission::Result(key, None)));
            s.send(c, E, Q, Message::Reply(key, reply, script_reply));
        }

        fn reason(c: &Config, s: &mut System<Self>, key: u8, report: Report, script_reply: bool) {
            s.ledger
                .0
                .push(Event::Decided(key, Decision::Reason(report)));
            if !report.rule.is_empty() {
                s.ledger.0.push(Event::Emitted(Emission::Diagnostic(
                    key,
                    report.rule,
                    None,
                    None,
                )));
            }
            if report.undone {
                s.ledger.0.push(Event::Emitted(Emission::Diagnostic(
                    key,
                    "PND-011: SignInUndone=False",
                    None,
                    None,
                )));
            }
            Self::emit(c, s, key, Reply::Decision, script_reply);
        }

        fn simple_reason(
            c: &Config,
            s: &mut System<Self>,
            key: u8,
            reason: ResultKind,
            rule: &'static str,
        ) {
            Self::reason(
                c,
                s,
                key,
                Report {
                    reason,
                    rule,
                    action: None,
                    invocation: None,
                    undone: false,
                },
                true,
            );
        }

        fn part(c: &Config, s: &mut System<Self>, id: Attempt, part: Part) {
            s.ledger.0.push(Event::PartDispatched(id, part));
            if part == Part::Turn {
                s.ledger.0.push(Event::Emitted(Emission::ToAgent(id, None)));
            }
            s.send(c, E, H, Message::Hook(id, part));
        }

        fn lifecycle(s: &mut System<Self>, state: Lifecycle) {
            s.nodes[E].data.lifecycle = Some(state);
            s.ledger.0.push(Event::Lifecycle(state));
        }

        fn answer(s: &mut System<Self>, id: ActionId, answer: Answer, actor: Option<Attempt>) {
            s.nodes[E].data.actions[id.number as usize].answer = Some(answer);
            s.ledger.0.push(Event::Answered(id, answer, actor));
        }

        fn answer_all(s: &mut System<Self>, answer: Answer, actor: Attempt) {
            let pending: Vec<_> = s.nodes[E]
                .data
                .actions
                .iter()
                .filter(|a| a.answer.is_none())
                .map(|a| a.id)
                .collect();
            for id in pending {
                Self::answer(s, id, answer, Some(actor));
            }
        }

        fn raise(s: &mut System<Self>, kind: ActionKind) -> ActionId {
            let e = &mut s.nodes[E].data;
            let id = ActionId {
                kind,
                number: e.actions.len() as u8,
            };
            e.actions.push(Action { id, answer: None });
            e.hold = true;
            s.ledger.0.push(Event::Raised(id));
            id
        }

        fn end(c: &Config, s: &mut System<Self>, id: Attempt, ending: Ending) {
            let e = &mut s.nodes[E].data;
            let primary = e
                .invocations
                .iter()
                .find(|i| i.id.key == id.key)
                .is_some_and(|i| i.id == id);
            let inv = e.inv_mut(id);
            inv.end = Some(ending);
            inv.pending_return = None;
            e.records.push(id);
            s.ledger.0.push(Event::Ended(id, ending));
            s.ledger.0.push(Event::Emitted(Emission::Record(id, None)));
            let drives_w = e.first_sign_in == Some(id);
            Self::emit(c, s, id.key, Reply::End(ending.outcome, drives_w), primary);
        }

        fn success(&self, c: &Config, s: &mut System<Self>, id: Attempt, account: Account) {
            let pending = s.nodes[E].data.unanswered();
            Self::answer_all(s, Answer::SignedIn, id);
            let e = &mut s.nodes[E].data;
            if self.variant != Variant::RetainHold || pending.is_some() || e.expired.is_none() {
                e.hold = false;
            }
            Self::end(
                c,
                s,
                id,
                Ending {
                    outcome: ResultKind::Succeeded,
                    detail: Detail::Account(account),
                    interruption: None,
                },
            );
        }

        fn dispatch(c: &Config, s: &mut System<Self>, key: u8) {
            let e = &mut s.nodes[E].data;
            let id = Attempt {
                key,
                number: e.invocations.len() as u8,
            };
            if matches!(key, R_SIGN | X_SIGN) && e.first_sign_in.is_none() {
                e.first_sign_in = Some(id);
            }
            e.invocations.push(Invocation::new(id));
            s.ledger.0.push(Event::Decided(key, Decision::Dispatch(id)));
            let part = match key {
                SETUP => Part::SetupAccount,
                RETIRE => {
                    Self::answer_all(s, Answer::Void, id);
                    Self::lifecycle(s, Lifecycle::Retired);
                    Part::Stop
                }
                EARLY_TURN | LATE_TURN => Part::Turn,
                _ => Part::FirstAccount,
            };
            Self::part(c, s, id, part);
        }

        fn request(&self, c: &Config, s: &mut System<Self>, key: u8) {
            s.ledger.0.push(Event::Request(key));
            let e = &s.nodes[E].data;
            if sign_in(key)
                && (caller(key) == Y
                    || (caller(key) != R
                        && (!e.fleet_inspection || e.lifecycle == Some(Lifecycle::Removed))))
            {
                Self::simple_reason(c, s, key, ResultKind::Refused, "OPS-023");
                return;
            }
            if let Some(prior) = e.invocations.iter().rev().find(|i| i.id.key == key) {
                match prior.end.map(|end| end.outcome) {
                    None => {
                        if sign_in(key) && self.variant == Variant::CopyAsNew {
                            Self::dispatch(c, s, key);
                        } else {
                            Self::reason(
                                c,
                                s,
                                key,
                                Report {
                                    reason: ResultKind::Conflict,
                                    rule: "OPS-032",
                                    action: None,
                                    invocation: Some(prior.id),
                                    undone: false,
                                },
                                false,
                            );
                        }
                        return;
                    }
                    Some(outcome @ (ResultKind::Succeeded | ResultKind::Unknown)) => {
                        s.ledger
                            .0
                            .push(Event::Decided(key, Decision::Repeat(prior.id, outcome)));
                        Self::emit(c, s, key, Reply::End(outcome, false), false);
                        return;
                    }
                    _ => {}
                }
            }

            if sign_in(key) {
                let overlap = e
                    .invocations
                    .iter()
                    .any(|i| excludes_sign_in(i.id.key) && i.end.is_none());
                if self.refuses_overlap() && (overlap || e.stale_conflict) {
                    let reason = if self.variant == Variant::Unavailable {
                        ResultKind::Unavailable
                    } else {
                        ResultKind::Conflict
                    };
                    Self::reason(
                        c,
                        s,
                        key,
                        Report {
                            reason,
                            rule: "PND-010",
                            action: e.unanswered(),
                            invocation: None,
                            undone: false,
                        },
                        true,
                    );
                    return;
                }
                if e.lifecycle == Some(Lifecycle::Retired)
                    && !e
                        .invocations
                        .iter()
                        .any(|i| i.id.key == RETIRE && i.end.is_none())
                {
                    s.ledger.0.push(Event::Decided(key, Decision::Retired));
                    Self::emit(c, s, key, Reply::Decision, true);
                    return;
                }
            }
            if turn(key) {
                if e.lifecycle != Some(Lifecycle::Ready) {
                    Self::simple_reason(c, s, key, ResultKind::Unavailable, "CON-024");
                    return;
                }
                if e.hold || e.undone_false {
                    let undone = e.undone_false;
                    let pending = e.unanswered();
                    let action =
                        if (e.hold && pending.is_none()) || self.variant == Variant::RaiseTwice {
                            Some(Self::raise(s, ActionKind::Replacement))
                        } else {
                            pending
                        };
                    Self::reason(
                        c,
                        s,
                        key,
                        Report {
                            reason: ResultKind::Unavailable,
                            rule: if action.is_some() { "PND-005" } else { "" },
                            action,
                            invocation: None,
                            undone,
                        },
                        true,
                    );
                    return;
                }
            }
            Self::dispatch(c, s, key);
        }

        fn issue(c: &Config, s: &mut System<Self>, id: Attempt, round: u8) {
            let e = &mut s.nodes[E].data;
            e.inv_mut(id).issued |= 1 << round;
            let token = RequestState { id, round };
            s.ledger.0.push(Event::Issued(token, caller(id.key)));
            let primary = e
                .invocations
                .iter()
                .find(|i| i.id.key == id.key)
                .is_some_and(|i| i.id == id);
            let drives = e.first_sign_in == Some(id) || id.key == W2;
            Self::emit(c, s, id.key, Reply::Input(token, drives), primary);
        }

        fn retry(
            &self,
            c: &Config,
            s: &mut System<Self>,
            token: RequestState,
            principal: Caller,
            value: Return,
        ) {
            let inv = s.nodes[E].data.inv_mut(token.id);
            let check = if inv.end.is_some() {
                StateCheck::Ended
            } else if principal != caller(inv.id.key) || inv.issued & (1 << token.round) == 0 {
                StateCheck::Mismatch
            } else if inv.consumed & (1 << token.round) != 0 {
                StateCheck::Consumed
            } else {
                StateCheck::Accepted
            };
            s.ledger.0.push(Event::Checked(token, principal, check));
            if check != StateCheck::Accepted {
                s.ledger.0.push(Event::ReturnRefused(token, None));
                return;
            }
            if self.variant != Variant::ReplayState {
                inv.consumed |= 1 << token.round;
            }
            if self.variant != Variant::ReturnTwice
                && !inv.return_passed
                && let Some((buffered, buffered_value)) = inv.pending_return.take()
            {
                // A further accepted retry while a return is buffered: E passes the buffered
                // return on first, so this retry's check finds a return passed on.
                Self::pass(c, s, buffered, buffered_value);
            }
            let inv = s.nodes[E].data.inv_mut(token.id);
            if inv.return_passed && self.variant != Variant::ReturnTwice {
                s.ledger
                    .0
                    .push(Event::ReturnRefused(token, Some("already-returned")));
                s.ledger.0.push(Event::Emitted(Emission::Diagnostic(
                    token.id.key,
                    "PND-013",
                    None,
                    (self.variant == Variant::LeakDiagnostic).then_some(value),
                )));
                if token.round == 2 {
                    s.nodes[E].data.inv_mut(token.id).poll_ready = true;
                }
            } else {
                inv.pending_return = Some((token, value));
            }
        }

        fn pass(c: &Config, s: &mut System<Self>, token: RequestState, value: Return) {
            s.nodes[E].data.inv_mut(token.id).return_passed = true;
            s.ledger
                .0
                .push(Event::PartDispatched(token.id, Part::Paste));
            s.ledger
                .0
                .push(Event::ReturnPassed(token, Part::Login, value));
            s.ledger.0.push(Event::Emitted(Emission::HookRetry(
                token.id,
                Part::Login,
                value,
            )));
            s.send(c, E, H, Message::HookRetry(token.id, value));
        }

        fn hook(&self, c: &Config, s: &mut System<Self>, id: Attempt, part: Part) {
            let home = &mut s.nodes[H].data;
            let result = match part {
                Part::SetupAccount | Part::FirstAccount | Part::InitialPoll | Part::FinalPoll => {
                    PartResult::Account(home.account)
                }
                Part::Login => {
                    home.programs.push(Program {
                        id,
                        choice: None,
                        took_return: false,
                        waiting: true,
                    });
                    s.ledger.0.push(Event::LoginStarted(id));
                    PartResult::Login
                }
                Part::Undo => {
                    home.undo_job = Some(id);
                    return;
                }
                Part::Turn if id.key == LATE_TURN => {
                    home.turn_job = Some(id);
                    return;
                }
                Part::Turn | Part::Setup | Part::Stop => PartResult::Succeeded,
                Part::Paste => unreachable!(),
            };
            s.send(c, H, E, Message::PartResult(id, part, result));
        }

        fn part_result(
            &self,
            c: &Config,
            s: &mut System<Self>,
            id: Attempt,
            part: Part,
            result: PartResult,
        ) {
            let e = &mut s.nodes[E].data;
            let late = e.inv(id).end.is_some();
            s.ledger.0.push(Event::PartResult(id, part, result, late));
            if late {
                return;
            }
            if let PartResult::Account(account) = result
                && (account.expected() || account == Account::None)
            {
                e.undone_false = false;
            }
            match (part, result) {
                (Part::SetupAccount, PartResult::Account(account)) => {
                    if !account.expected() && e.unanswered().is_none() {
                        Self::raise(s, ActionKind::Setup);
                    }
                    Self::part(c, s, id, Part::Setup);
                }
                (Part::Setup, PartResult::Succeeded) => {
                    Self::lifecycle(s, Lifecycle::Ready);
                    Self::end(c, s, id, Ending::ordinary(ResultKind::Succeeded));
                }
                (Part::FirstAccount, PartResult::Account(account)) => {
                    e.inv_mut(id).initial = account;
                    if account.expected() {
                        self.success(c, s, id, account);
                    } else {
                        if account == Account::Other
                            && e.unanswered().is_none()
                            && !e.dispatched_retirement()
                        {
                            Self::raise(s, ActionKind::Replacement);
                        }
                        Self::part(c, s, id, Part::Login);
                    }
                }
                (Part::Login, PartResult::Login) => {
                    Self::issue(c, s, id, 1);
                }
                (Part::InitialPoll | Part::FinalPoll, PartResult::Account(account)) => {
                    let initial = e.inv(id).initial;
                    if account.expected()
                        || (account != Account::None
                            && account != initial
                            && self.variant == Variant::WrongAccountKept)
                    {
                        self.success(c, s, id, account);
                    } else if account != Account::None
                        && (account != initial || self.variant == Variant::UndoInitial)
                    {
                        e.inv_mut(id).waiting = false;
                        e.inv_mut(id).wrong = Some(account);
                        Self::part(c, s, id, Part::Undo);
                    } else {
                        e.inv_mut(id).waiting = true;
                    }
                }
                (Part::Paste, PartResult::Paste) => {
                    if e.inv(id).issued & (1 << 2) != 0 {
                        e.inv_mut(id).poll_ready = true;
                    } else {
                        e.inv_mut(id).can_offer = true;
                    }
                }
                (Part::Undo, PartResult::Succeeded | PartResult::UndoFailed) => {
                    let wrong = e.inv(id).wrong.unwrap();
                    if result == PartResult::UndoFailed {
                        e.undone_false = true;
                        s.ledger.0.push(Event::Emitted(Emission::Diagnostic(
                            id.key,
                            "PND-011: SignInUndone=False (sign-out failed)",
                            None,
                            None,
                        )));
                    }
                    Self::end(
                        c,
                        s,
                        id,
                        Ending {
                            outcome: ResultKind::Failed,
                            detail: Detail::Wrong {
                                expected: "E",
                                found: wrong,
                            },
                            interruption: None,
                        },
                    );
                }
                (Part::Turn, PartResult::Lapse) => {
                    s.ledger.0.push(Event::Emitted(Emission::Diagnostic(
                        id.key, "PND-005", None, None,
                    )));
                    if e.unanswered().is_none() && !e.dispatched_retirement() {
                        Self::raise(s, ActionKind::Lapse);
                    }
                    Self::end(c, s, id, Ending::ordinary(ResultKind::Failed));
                }
                (Part::Turn, PartResult::Succeeded) => {
                    Self::end(c, s, id, Ending::ordinary(ResultKind::Succeeded))
                }
                (Part::Stop, PartResult::Succeeded) => {
                    Self::lifecycle(s, Lifecycle::Removed);
                    Self::end(c, s, id, Ending::ordinary(ResultKind::Succeeded));
                }
                _ => unreachable!(),
            }
        }

        fn sync_restart(&self, c: &Config, s: &mut System<Self>) {
            let restart = restart_events(&mut s.nodes[E].data.restart);
            if restart.crashed {
                s.ledger.0.push(Event::Crashed);
            }
            if restart.recovered {
                s.ledger.0.push(Event::Recovered);
                let open: Vec<_> = s.nodes[E]
                    .data
                    .invocations
                    .iter()
                    .filter(|i| i.end.is_none())
                    .map(|i| (i.id, i.pending_return))
                    .collect();
                for (id, pending_return) in open {
                    if sign_in(id.key) && self.variant == Variant::ConflictForever {
                        s.nodes[E].data.stale_conflict = true;
                    }
                    Self::end(
                        c,
                        s,
                        id,
                        Ending {
                            outcome: ResultKind::Unknown,
                            detail: Detail::None,
                            interruption: Some(Interruption::Restart),
                        },
                    );
                    if let Some((token, value)) = pending_return {
                        Self::pass(c, s, token, value);
                    }
                }
            }
        }

        fn waiting_for_login(s: &System<Self>) -> Option<Attempt> {
            s.nodes[E]
                .data
                .invocations
                .iter()
                .find(|inv| sign_in(inv.id.key) && inv.end.is_none() && inv.waiting)
                .map(|inv| inv.id)
        }

        fn received_reply(s: &mut System<Self>, key: u8, reply: Reply) {
            let q = &mut s.nodes[Q].data.q;
            match reply {
                Reply::Decision => q.script.reply(key),
                Reply::End(outcome, drives_w) => {
                    if drives_w && q.w_ended.is_none() {
                        q.w_ended = Some(outcome);
                    }
                    if q.input.is_some_and(|token| token.id.key == key) {
                        q.input = None;
                    }
                    q.script.reply(key);
                }
                Reply::Input(token, drives) => {
                    if drives
                        && !q.pasted(token)
                        && (if key == W2 {
                            !q.script.has_reply(W2)
                        } else {
                            q.w_ended.is_none()
                        })
                    {
                        q.input = Some(token);
                    }
                    if key != W2 {
                        q.script.reply(key);
                    }
                }
            }
            q.progress();
        }
    }

    impl Protocol for SignIn {
        type Node = Data;
        type Message = Message;
        type Ledger = Ledger;
        type Local = Local;

        fn gate_remaining_faults(&self) -> bool {
            true
        }

        fn fault_allowed(&self, fault: Fault, src: usize, dst: usize) -> bool {
            fault_allowed(fault, src, dst)
        }

        fn fault_enabled(&self, s: &System<Self>, fault: Fault) -> bool {
            fault != Fault::Reorder
                || bracket_on_link(&s.links[s.link(Q, E)], |m| match m {
                    Message::Request(R_SIGN | X_SIGN) => Some(3),
                    Message::Request(RETIRE | FINAL_SIGN) => Some(7),
                    _ => None,
                })
        }

        fn initial(&self, _: &Config) -> System<Self> {
            let mut s = initial::<Self>(Ledger(vec![
                Event::Lifecycle(Lifecycle::Registered),
                Event::Opened(Conduit::SignInFleet(R)),
                Event::Opened(Conduit::SignInFleet(X)),
                Event::Opened(Conduit::SignInFleet(Y)),
                Event::Opened(Conduit::InspectFleet(X)),
                Event::Opened(Conduit::TurnAgent(X)),
            ]));
            let e = &mut s.nodes[E].data;
            e.lifecycle = Some(Lifecycle::Registered);
            e.fleet_inspection = true;
            s
        }

        fn receive(&self, c: &Config, s: &mut System<Self>, _: usize, _: usize, m: Message) {
            self.sync_restart(c, s);
            match m {
                Message::Request(key) => self.request(c, s, key),
                Message::Retry(token, principal, value) => {
                    self.retry(c, s, token, principal, value)
                }
                Message::Reply(key, reply, true) => Self::received_reply(s, key, reply),
                Message::Reply(_, _, false) => {}
                Message::Hook(id, part) => self.hook(c, s, id, part),
                Message::HookRetry(id, _) => {
                    s.nodes[H]
                        .data
                        .programs
                        .iter_mut()
                        .find(|p| p.id == id)
                        .unwrap()
                        .took_return = true;
                    s.ledger.0.push(Event::ProgramTookReturn(id));
                    s.send(
                        c,
                        H,
                        E,
                        Message::PartResult(id, Part::Paste, PartResult::Paste),
                    );
                }
                Message::PartResult(id, part, result) => self.part_result(c, s, id, part, result),
            }
        }

        fn crash(&self, node: &mut Data) {
            node.restart.crashed = true;
            if self.variant != Variant::SavedReturn {
                for inv in &mut node.invocations {
                    inv.pending_return = None;
                }
            }
            if self.variant == Variant::VolatileHold {
                node.hold = false;
            }
        }

        fn recover(&self, node: &mut Data) {
            node.restart.recovered = true;
        }

        fn timer_enabled(&self, s: &System<Self>, node: usize) -> bool {
            node == E && Self::waiting_for_login(s).is_some()
        }

        fn timer(&self, c: &Config, s: &mut System<Self>, _: usize) {
            let id = Self::waiting_for_login(s).unwrap();
            let link = s.link(E, H);
            s.links[link].retain(|m| {
                !matches!(m,
                Message::Hook(other, _) | Message::HookRetry(other, _) if *other == id)
            });
            Self::end(
                c,
                s,
                id,
                Ending {
                    outcome: ResultKind::Unknown,
                    detail: Detail::None,
                    interruption: Some(Interruption::Wait),
                },
            );
        }

        fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
            let data = &s.nodes[node].data;
            let q = s.nodes[Q].data.q;
            match node {
                Q => {
                    if let Some(keys) = STAGES.get(q.script.stage) {
                        out.extend(
                            keys.iter()
                                .filter(|&&key| q.script.unsent(key))
                                .copied()
                                .map(Local::Send),
                        );
                    }
                    if let Some(token) = q.input.filter(|t| q.driving(*t))
                        && s.nodes[H]
                            .data
                            .programs
                            .iter()
                            .any(|p| p.id == token.id && p.choice.is_some())
                    {
                        out.push(Local::SendRetry(token));
                    }
                }
                E => {
                    if data.restart.recovered {
                        out.push(Local::RecordRestart);
                        return;
                    }
                    if data.expired.is_none() && q.script.stage < STAGES.len() {
                        out.extend(data.actions.iter().map(|a| Local::Expire(a.id)));
                    }
                    for inv in data.invocations.iter().filter(|i| i.end.is_none()) {
                        if inv.pending_return.is_some() {
                            out.push(Local::PassReturn(inv.id));
                        }
                        let program = s.nodes[H].data.programs.iter().find(|p| p.id == inv.id);
                        let waiting = program.is_some_and(|p| p.waiting);
                        if !inv.initial_poll && inv.return_passed {
                            out.push(Local::Poll(inv.id, Part::InitialPoll));
                        }
                        if inv.can_offer {
                            out.push(Local::OfferSecond(inv.id, false));
                            if !data.offered_second && waiting {
                                out.push(Local::OfferSecond(inv.id, true));
                            }
                        }
                        if inv.poll_ready && inv.waiting && !inv.final_poll && !waiting {
                            out.push(Local::Poll(inv.id, Part::FinalPoll));
                        }
                    }
                }
                H => {
                    if let Some(token) = q.input.filter(|t| q.driving(*t) && t.round == 1)
                        && data
                            .programs
                            .iter()
                            .any(|p| p.id == token.id && p.choice.is_none() && p.waiting)
                    {
                        out.push(Local::CompletePage(token.id, Account::LowerE));
                        if token.id.key != W2 {
                            out.push(Local::CompletePage(token.id, Account::Other));
                        }
                    }
                    out.extend(
                        data.programs
                            .iter()
                            .filter(|p| p.waiting && p.took_return)
                            .map(|p| Local::ApplyReturn(p.id)),
                    );
                    if let Some(id) = data.undo_job {
                        out.push(Local::Undo(id, true));
                        if !data.undo_failed_once {
                            out.push(Local::Undo(id, false));
                        }
                    }
                    if let Some(id) = data.turn_job {
                        out.push(Local::Turn(id, true));
                        if !data.lapsed_once {
                            out.push(Local::Turn(id, false));
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        fn local_step(&self, c: &Config, s: &mut System<Self>, _: usize, action: Local) {
            self.sync_restart(c, s);
            match action {
                Local::Send(key) => {
                    s.nodes[Q].data.q.script.sent(key);
                    s.ledger.0.push(Event::Sent(key));
                    s.send(c, Q, E, Message::Request(key));
                }
                Local::SendRetry(token) => {
                    let q = &mut s.nodes[Q].data.q;
                    q.input = None;
                    q.pasted |= paste_bit(token);
                    let principal = caller(token.id.key);
                    let value = token.value();
                    s.ledger.0.push(Event::RetrySent(token, principal, value));
                    s.send(c, Q, E, Message::Retry(token, principal, value));
                }
                Local::CompletePage(id, account) => {
                    let program = s.nodes[H]
                        .data
                        .programs
                        .iter_mut()
                        .find(|p| p.id == id)
                        .unwrap();
                    program.choice = Some(account);
                    s.ledger.0.push(Event::PersonCompleted(id, account));
                }
                Local::Expire(id) => {
                    let e = &mut s.nodes[E].data;
                    e.expired = Some(id);
                    s.ledger.0.push(Event::Expired(id));
                    if e.actions[id.number as usize].answer.is_none() {
                        if self.variant == Variant::ExpiryLifts {
                            e.hold = false;
                        }
                        Self::answer(s, id, Answer::Expired, None);
                    }
                }
                Local::PassReturn(id) => {
                    let (token, value) = s.nodes[E].data.inv_mut(id).pending_return.take().unwrap();
                    Self::pass(c, s, token, value);
                }
                Local::OfferSecond(id, offer) => {
                    let e = &mut s.nodes[E].data;
                    e.inv_mut(id).can_offer = false;
                    if offer {
                        e.offered_second = true;
                        Self::issue(c, s, id, 2);
                    } else {
                        e.inv_mut(id).poll_ready = true;
                    }
                }
                Local::Poll(id, part) => {
                    let inv = s.nodes[E].data.inv_mut(id);
                    match part {
                        Part::InitialPoll => inv.initial_poll = true,
                        Part::FinalPoll => inv.final_poll = true,
                        _ => unreachable!(),
                    }
                    Self::part(c, s, id, part);
                }
                Local::ApplyReturn(id) => {
                    let home = &mut s.nodes[H].data;
                    let program = home.programs.iter_mut().find(|p| p.id == id).unwrap();
                    home.account = program.choice.expect("the person completed this program");
                    program.waiting = false;
                    s.ledger.0.push(Event::ProgramFinished(id));
                }
                Local::Undo(id, succeeded) => {
                    let home = &mut s.nodes[H].data;
                    home.undo_job = None;
                    let result = if succeeded {
                        home.account = Account::None;
                        PartResult::Succeeded
                    } else {
                        home.undo_failed_once = true;
                        PartResult::UndoFailed
                    };
                    s.send(c, H, E, Message::PartResult(id, Part::Undo, result));
                }
                Local::Turn(id, succeeded) => {
                    let home = &mut s.nodes[H].data;
                    home.turn_job = None;
                    let result = if succeeded {
                        PartResult::Succeeded
                    } else {
                        home.lapsed_once = true;
                        home.account = Account::None;
                        PartResult::Lapse
                    };
                    s.send(c, H, E, Message::PartResult(id, Part::Turn, result));
                }
                Local::RecordRestart => {}
            }
        }

        fn properties(&self) -> Vec<NamedProperty<Self>> {
            properties::<Self>(
                &SAFETY,
                &[
                    &["PND-010"],
                    &["PND-011"],
                    &["PND-013"],
                    &["PND-014"],
                    &["PND-005"],
                    &["PND-001", "PND-005"],
                ],
                &REACH,
                &PROMISES,
                ledger_predicates!(x10, k11, r13, v14, h05, p01;
                    setup_raises, held, caller_refused, conflict_sign_in, conflict_retirement,
                    retire_during_sign_in, return_passed, already_returned, wrong_account_undone,
                    undo_failed, first_account_kept, signed_in, expiry_then_signed_in,
                    signed_in_then_expiry, new_after_expiry, lapse_raises, void, restart_ends_sign_in,
                    restart_overlap, repeat_conflict, replay_rejected, wait_ended;
                    |_, s| s.nodes[Q].data.q.script.stage == STAGES.len() && drained::<Self>(s);
                    removed_refused, wrong_account_preserved, shutdown_unknown, undo_interrupted),
            )
        }
    }

    // Rebuild the lifecycle, action history and currently open dispatches from events.
    #[derive(Default)]
    struct History {
        lifecycle: Option<Lifecycle>,
        fleet_scope: bool,
        sign_in_routes: u8,
        turn_route: bool,
        attempts: Vec<(Attempt, Option<Ending>)>,
        raised: Vec<ActionId>,
        answered: Vec<(ActionId, Answer)>,
        expired: Option<ActionId>,
        held: bool,
        undo_failed: bool,
        retirement: bool,
        expired_while_held: bool,
        released_after_expiry: bool,
    }
    impl History {
        fn observe(&mut self, event: Event) {
            match event {
                Event::Opened(Conduit::InspectFleet(X)) => self.fleet_scope = true,
                Event::Opened(Conduit::SignInFleet(caller)) => {
                    self.sign_in_routes |= 1 << caller as u8
                }
                Event::Opened(Conduit::TurnAgent(X)) => self.turn_route = true,
                Event::Lifecycle(lifecycle) => self.lifecycle = Some(lifecycle),
                Event::Decided(key, Decision::Dispatch(id)) => {
                    self.attempts.push((id, None));
                    self.retirement |= key == RETIRE;
                }
                Event::Ended(id, end) => {
                    self.attempts[id.number as usize].1 = Some(end);
                    if sign_in(id.key) && end.outcome == ResultKind::Succeeded {
                        self.released_after_expiry =
                            self.held && self.expired_while_held && self.pending().is_none();
                        self.held = false;
                        self.expired_while_held = false;
                    }
                }
                Event::Raised(id) => {
                    self.raised.push(id);
                    self.held = true;
                    self.expired_while_held = false;
                    self.released_after_expiry = false;
                }
                Event::PartResult(_, _, PartResult::Account(account), false)
                    if account.expected() || account == Account::None =>
                {
                    self.undo_failed = false;
                }
                Event::Answered(id, answer, _) => self.answered.push((id, answer)),
                Event::Expired(id) => {
                    self.expired_while_held |= self.held && self.pending() == Some(id);
                    self.expired = Some(id);
                }
                Event::Emitted(Emission::Diagnostic(
                    _,
                    "PND-011: SignInUndone=False (sign-out failed)",
                    _,
                    _,
                )) => self.undo_failed = true,
                _ => {}
            }
        }

        fn sign_in_access(&self, key: u8) -> bool {
            let principal = caller(key);
            self.sign_in_routes & (1 << principal as u8) != 0
                && (principal == R
                    || (self.fleet_scope
                        && principal == X
                        && self.lifecycle != Some(Lifecycle::Removed)))
        }

        fn previous(&self, key: u8) -> Option<(Attempt, Option<Ending>)> {
            self.attempts
                .iter()
                .rev()
                .copied()
                .find(|(id, _)| id.key == key)
        }

        fn active(&self, matches: impl Fn(u8) -> bool) -> bool {
            self.attempts
                .iter()
                .any(|(id, end)| end.is_none() && matches(id.key))
        }

        fn pending(&self) -> Option<ActionId> {
            self.raised.iter().rev().copied().find(|id| {
                self.expired != Some(*id)
                    && !self.answered.iter().any(|(answered, _)| answered == id)
            })
        }
    }

    fn x10(ledger: &Ledger) -> bool {
        let mut history = History::default();
        for &event in &ledger.0 {
            if let Event::Decided(key, decision) = event
                && sign_in(key)
            {
                let valid = if !history.sign_in_access(key) {
                    matches!(
                        decision,
                        Decision::Reason(Report {
                            reason: ResultKind::Refused,
                            rule: "OPS-023",
                            action: None,
                            invocation: None,
                            ..
                        })
                    )
                } else {
                    match history.previous(key) {
                        Some((id, None)) => matches!(decision, Decision::Reason(Report {
                            reason: ResultKind::Conflict, rule: "OPS-032", action: None, invocation: Some(other), ..
                        }) if other == id),
                        Some((id, Some(end))) if end.outcome == ResultKind::Succeeded => {
                            if end.detail == Detail::None {
                                matches!(decision, Decision::Reason(Report {
                                    reason: ResultKind::Conflict, rule: "OPS-034", action: None,
                                    invocation: Some(other), ..
                                }) if other == id)
                            } else {
                                decision == Decision::Repeat(id, ResultKind::Succeeded)
                            }
                        }
                        Some((id, Some(end))) if end.outcome == ResultKind::Unknown => {
                            decision == Decision::Repeat(id, ResultKind::Unknown)
                        }
                        _ if history.active(excludes_sign_in) => matches!(decision,
                            Decision::Reason(Report {
                                reason: ResultKind::Conflict, rule: "PND-010", action,
                                invocation: None, ..
                            }) if action == history.pending()),
                        _ if history.lifecycle == Some(Lifecycle::Retired) => {
                            decision == Decision::Retired
                        }
                        _ => matches!(decision, Decision::Dispatch(_)),
                    }
                };
                if !valid {
                    return false;
                }
            }
            history.observe(event);
        }
        true
    }

    #[derive(Clone, Copy, Default)]
    struct AccountCheck {
        initial: Account,
        expected: Option<Account>,
        wrong: Option<Account>,
        sign_out: bool,
        sign_out_result: bool,
        undo_failed: bool,
        undo_reported: bool,
        signed_in: bool,
        pending_at_success: Option<ActionId>,
        signed_in_actions: u32,
        ended: bool,
    }

    fn k11(ledger: &Ledger) -> bool {
        let mut checks = Vec::<AccountCheck>::new();
        let mut history = History::default();
        for (i, &event) in ledger.0.iter().enumerate() {
            match event {
                Event::Decided(_, Decision::Dispatch(_)) => checks.push(AccountCheck::default()),
                Event::PartResult(id, part, result, late) if sign_in(id.key) => {
                    let check = &mut checks[id.number as usize];
                    if check.ended != late {
                        return false;
                    }
                    if !late {
                        match (part, result) {
                            (
                                Part::FirstAccount | Part::InitialPoll | Part::FinalPoll,
                                PartResult::Account(account),
                            ) => {
                                if part == Part::FirstAccount {
                                    check.initial = account;
                                }
                                if account.expected() {
                                    check.expected = Some(account);
                                    // The step that incorporates this result is the success step.
                                    check.pending_at_success = history.pending();
                                } else if part != Part::FirstAccount
                                    && account != Account::None
                                    && account != check.initial
                                {
                                    check.wrong = Some(account);
                                }
                            }
                            (Part::Undo, PartResult::Succeeded | PartResult::UndoFailed) => {
                                check.sign_out_result = true;
                                check.undo_failed = result == PartResult::UndoFailed;
                            }
                            _ => {}
                        }
                    }
                }
                Event::PartDispatched(id, part) if sign_in(id.key) => {
                    let check = &mut checks[id.number as usize];
                    if check.ended || (part == Part::Undo && check.wrong.is_none()) {
                        return false;
                    }
                    check.sign_out |= part == Part::Undo;
                }
                Event::Emitted(Emission::Diagnostic(
                    key,
                    "PND-011: SignInUndone=False (sign-out failed)",
                    _,
                    _,
                )) => {
                    // Credit the attempt whose failed sign-out result this step incorporated.
                    if let Some(Event::PartResult(id, Part::Undo, PartResult::UndoFailed, false)) =
                        i.checked_sub(1).map(|j| ledger.0[j])
                        && id.key == key
                    {
                        checks[id.number as usize].undo_reported = true;
                    }
                }
                Event::Answered(action, Answer::SignedIn, Some(id)) => {
                    let check = &mut checks[id.number as usize];
                    if !sign_in(id.key) || check.expected.is_none() || check.ended {
                        return false;
                    }
                    check.signed_in = true;
                    check.signed_in_actions |= 1 << action.number;
                }
                Event::Answered(_, Answer::SignedIn, None) => return false,
                Event::Ended(id, end) if sign_in(id.key) => {
                    let check = &mut checks[id.number as usize];
                    if check.ended || (check.undo_failed && !check.undo_reported) {
                        return false;
                    }
                    if end.interruption.is_some() {
                        if end.outcome != ResultKind::Unknown || check.signed_in {
                            return false;
                        }
                    } else if let Some(expected) = check.expected {
                        if end.outcome != ResultKind::Succeeded
                            || end.detail != Detail::Account(expected)
                            || history.pending().is_some()
                            || check.pending_at_success.is_some_and(|action| {
                                check.signed_in_actions & (1 << action.number) == 0
                            })
                        {
                            return false;
                        }
                    } else if let Some(wrong) = check.wrong {
                        if !check.sign_out
                            || !check.sign_out_result
                            || end.outcome != ResultKind::Failed
                            || check.signed_in
                            || end.detail
                                != (Detail::Wrong {
                                    expected: "E",
                                    found: wrong,
                                })
                        {
                            return false;
                        }
                    } else if end.outcome == ResultKind::Succeeded {
                        return false;
                    }
                    check.ended = true;
                }
                _ => {}
            }
            history.observe(event);
        }
        true
    }

    fn r13(ledger: &Ledger) -> bool {
        let mut issued = Vec::new();
        let mut accepted = Vec::new();
        let mut passed = Vec::<Attempt>::new();
        let mut ended = Vec::new();
        let mut login = Vec::new();
        let mut latest_check = None;
        for (i, &event) in ledger.0.iter().enumerate() {
            match event {
                Event::Issued(token, caller) => issued.push((token, caller)),
                Event::LoginStarted(id) => login.push(id),
                Event::Ended(id, _) => ended.push(id),
                Event::Checked(token, caller, check) => {
                    if check == StateCheck::Accepted {
                        if !issued.contains(&(token, caller))
                            || accepted.contains(&token)
                            || ended.contains(&token.id)
                        {
                            return false;
                        }
                        // This refusal is part of the same retry decision, not an eventual reply.
                        if passed.contains(&token.id)
                            && ledger.0.get(i + 1)
                                != Some(&Event::ReturnRefused(token, Some("already-returned")))
                        {
                            return false;
                        }
                        accepted.push(token);
                    }
                    latest_check = Some((token, check));
                }
                Event::ReturnPassed(token, part, _) => {
                    if !accepted.contains(&token)
                        || ended.contains(&token.id)
                        || passed.contains(&token.id)
                        || part != Part::Login
                        || !login.contains(&token.id)
                    {
                        return false;
                    }
                    passed.push(token.id);
                }
                Event::ReturnRefused(token, reason) => match latest_check {
                    Some((checked, StateCheck::Accepted)) if checked == token => {
                        if !passed.contains(&token.id) || reason != Some("already-returned") {
                            return false;
                        }
                    }
                    Some((checked, _)) if checked == token && reason.is_none() => {}
                    _ => return false,
                },
                // A PND-013 diagnostic belongs to an already-returned refusal; a retry that
                // requestState's checks reject reports none.
                Event::Emitted(Emission::Diagnostic(key, "PND-013", _, _))
                    if !matches!(i.checked_sub(1).map(|j| ledger.0[j]),
                        Some(Event::ReturnRefused(token, Some("already-returned")))
                        if token.id.key == key) =>
                {
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    fn v14(ledger: &Ledger) -> bool {
        ledger.0.iter().all(|event| match event {
            Event::Emitted(
                Emission::Diagnostic(_, _, _, value)
                | Emission::Result(_, value)
                | Emission::Record(_, value)
                | Emission::ToAgent(_, value),
            ) => value.is_none(),
            Event::Emitted(Emission::HookRetry(_, part, _)) => *part == Part::Login,
            _ => true,
        })
    }

    fn h05(ledger: &Ledger) -> bool {
        let mut history = History::default();
        for (i, &event) in ledger.0.iter().enumerate() {
            if let Event::Decided(key, decision) = event
                && turn(key)
            {
                if history.held
                    && (matches!(decision, Decision::Dispatch(_))
                        || (history.lifecycle == Some(Lifecycle::Ready)
                            && (!matches!(decision,
                            Decision::Reason(Report {
                                reason: ResultKind::Unavailable, rule: "PND-005", action: Some(id), undone, ..
                            }) if history.pending() == Some(id) && undone == history.undo_failed)
                                || !held_diagnostics(ledger, i, key, history.undo_failed))))
                {
                    return false;
                }
                if key == LATE_TURN
                    && history.released_after_expiry
                    && history.turn_route
                    && history.lifecycle == Some(Lifecycle::Ready)
                    && !matches!(decision, Decision::Dispatch(_))
                {
                    return false;
                }
            }
            history.observe(event);
        }
        true
    }

    // The diagnostics a held turn emits up to its result: PND-005's without an account, and
    // PND-011's when sign-out failure also holds the instance.
    fn held_diagnostics(ledger: &Ledger, i: usize, key: u8, undone: bool) -> bool {
        let turn_emissions = || {
            ledger.0[i + 1..]
                .iter()
                .take_while(|event| !matches!(event, Event::Emitted(Emission::Result(..))))
        };
        let pnd005 = turn_emissions().any(|event| {
            matches!(event, Event::Emitted(Emission::Diagnostic(k, "PND-005", None, _)) if *k == key)
        });
        let disclosed = turn_emissions().any(|event| {
            matches!(
                event,
                Event::Emitted(Emission::Diagnostic(_, "PND-005", Some(_), _))
            )
        });
        let pnd011 = turn_emissions().any(|event| {
            matches!(event,
                Event::Emitted(Emission::Diagnostic(k, "PND-011: SignInUndone=False", None, _))
                if *k == key)
        });
        pnd005 && !disclosed && (!undone || pnd011)
    }

    fn p01(ledger: &Ledger) -> bool {
        let mut history = History::default();
        let mut held_after_expiry = false;
        for (i, &event) in ledger.0.iter().enumerate() {
            match event {
                Event::Request(key) if turn(key) => {
                    held_after_expiry = history.lifecycle == Some(Lifecycle::Ready)
                        && history.held
                        && history.expired.is_some()
                        && history.pending().is_none()
                        && !history.retirement;
                }
                Event::Decided(key, _) if turn(key) && held_after_expiry => {
                    if !matches!(ledger.0.get(i.wrapping_sub(1)), Some(Event::Raised(_))) {
                        return false;
                    }
                }
                Event::Raised(id) => {
                    if history.pending().is_some() || history.retirement {
                        return false;
                    }
                    let before = ledger.0.get(i.wrapping_sub(1));
                    let source = match id.kind {
                        ActionKind::Setup => matches!(before, Some(Event::PartResult(
                            Attempt { key: SETUP, .. }, Part::SetupAccount, PartResult::Account(account), false
                        )) if !account.expected()),
                        ActionKind::Lapse => matches!(
                            (ledger.0.get(i.wrapping_sub(2)), before),
                            (
                                Some(Event::PartResult(
                                    Attempt { key: LATE_TURN, .. },
                                    Part::Turn,
                                    PartResult::Lapse,
                                    false
                                )),
                                Some(Event::Emitted(Emission::Diagnostic(
                                    LATE_TURN,
                                    "PND-005",
                                    _,
                                    _
                                )))
                            )
                        ),
                        ActionKind::Replacement => {
                            matches!(
                                before,
                                Some(Event::PartResult(
                                    _,
                                    Part::FirstAccount,
                                    PartResult::Account(Account::Other),
                                    false
                                ))
                            ) || (history.expired.is_some()
                                && matches!((before, ledger.0.get(i + 1)),
                            (Some(Event::Request(key)), Some(Event::Decided(other,
                                Decision::Reason(Report { rule: "PND-005", action: Some(action), .. }))))
                                if turn(*key) && key == other && *action == id))
                        }
                    };
                    if !source {
                        return false;
                    }
                }
                Event::PartResult(
                    _,
                    Part::FirstAccount,
                    PartResult::Account(Account::Other),
                    false,
                ) if !history.retirement && history.pending().is_none() => {
                    if !matches!(ledger.0.get(i + 1), Some(Event::Raised(_))) {
                        return false;
                    }
                }
                Event::PartResult(
                    Attempt { key: LATE_TURN, .. },
                    Part::Turn,
                    PartResult::Lapse,
                    false,
                ) if !history.retirement
                    && history.pending().is_none()
                    && !matches!(ledger.0.get(i + 2), Some(Event::Raised(_))) =>
                {
                    return false;
                }
                _ => {}
            }
            history.observe(event);
        }
        !history.retirement || history.pending().is_none()
    }

    fn setup_raises(l: &Ledger) -> bool {
        l.0.iter().any(|e| {
            matches!(
                e,
                Event::Raised(ActionId {
                    kind: ActionKind::Setup,
                    ..
                })
            )
        })
    }
    fn held(l: &Ledger) -> bool {
        l.0.iter().any(|e| {
            matches!(*e, Event::Decided(key, Decision::Reason(Report {
            reason: ResultKind::Unavailable, rule: "PND-005", action: Some(_), .. })) if turn(key))
        })
    }
    fn caller_refused(l: &Ledger) -> bool {
        l.0.iter().any(|e| {
            matches!(
                e,
                Event::Decided(
                    Y_SIGN,
                    Decision::Reason(Report {
                        reason: ResultKind::Refused,
                        ..
                    })
                )
            )
        }) && !l
            .0
            .iter()
            .any(|e| matches!(e, Event::Decided(Y_SIGN, Decision::Dispatch(_))))
    }
    fn occurred_with(l: &Ledger, test: impl Fn(Event, &History) -> bool) -> bool {
        let mut history = History::default();
        for &event in &l.0 {
            if test(event, &history) {
                return true;
            }
            history.observe(event);
        }
        false
    }
    fn overlap_conflict(event: Event) -> bool {
        matches!(event, Event::Decided(key, Decision::Reason(Report {
            reason: ResultKind::Conflict, rule: "PND-010", ..
        })) if sign_in(key))
    }
    fn conflict_sign_in(l: &Ledger) -> bool {
        occurred_with(l, |e, h| overlap_conflict(e) && h.active(sign_in))
    }
    fn conflict_retirement(l: &Ledger) -> bool {
        occurred_with(l, |e, h| {
            overlap_conflict(e) && h.active(|key| key == RETIRE)
        })
    }
    fn retire_during_sign_in(l: &Ledger) -> bool {
        occurred_with(l, |e, h| {
            matches!(e, Event::Decided(RETIRE, Decision::Dispatch(_))) && h.active(sign_in)
        })
    }
    fn return_passed(l: &Ledger) -> bool {
        l.0.iter().any(|e| matches!(e, Event::ReturnPassed(..)))
    }
    fn already_returned(l: &Ledger) -> bool {
        l.0.iter()
            .any(|e| matches!(e, Event::ReturnRefused(_, Some("already-returned"))))
    }
    fn wrong_account_undone(l: &Ledger) -> bool {
        l.0.iter().enumerate().any(|(i, event)| {
            let Event::Ended(
                id,
                Ending {
                    outcome: ResultKind::Failed,
                    detail:
                        Detail::Wrong {
                            found: Account::Other,
                            ..
                        },
                    ..
                },
            ) = event
            else {
                return false;
            };
            l.0[..i].contains(&Event::PartResult(
                *id,
                Part::Undo,
                PartResult::Succeeded,
                false,
            ))
        })
    }
    fn undo_failed(l: &Ledger) -> bool {
        l.0.iter().any(|e| {
            matches!(
                e,
                Event::Emitted(Emission::Diagnostic(
                    _,
                    "PND-011: SignInUndone=False (sign-out failed)",
                    _,
                    _
                ))
            )
        })
    }
    fn first_account_kept(l: &Ledger) -> bool {
        l.0.iter().enumerate().any(|(i, e)| {
            let Event::PartResult(
                id @ Attempt { key: W2, .. },
                Part::InitialPoll,
                PartResult::Account(Account::Other),
                false,
            ) = e
            else {
                return false;
            };
            l.0[..i].contains(&Event::PartResult(
                *id,
                Part::FirstAccount,
                PartResult::Account(Account::Other),
                false,
            )) && l.0.get(i + 1) != Some(&Event::PartDispatched(*id, Part::Undo))
        })
    }
    fn signed_in(l: &Ledger) -> bool {
        l.0.iter().enumerate().any(|(i, event)| {
            let Event::Ended(
                id,
                Ending {
                    outcome: ResultKind::Succeeded,
                    detail: Detail::Account(Account::LowerE),
                    ..
                },
            ) = event
            else {
                return false;
            };
            l.0[..i].iter().any(
                |ev| matches!(ev, Event::Answered(_, Answer::SignedIn, Some(actor)) if actor == id),
            )
        })
    }
    fn expiry_then_signed_in(l: &Ledger) -> bool {
        occurred_with(l, |e, h| {
            matches!(e, Event::Decided(LATE_TURN, Decision::Dispatch(_)))
                && h.released_after_expiry
                && h.lifecycle == Some(Lifecycle::Ready)
                && h.turn_route
        })
    }
    fn signed_in_then_expiry(l: &Ledger) -> bool {
        l.0.iter().enumerate().any(|(i, e)| {
            let Event::Expired(id) = e else { return false };
            l.0[..i]
                .iter()
                .any(|e| matches!(e, Event::Answered(other, Answer::SignedIn, _) if other == id))
                && l.0.get(i + 1) != Some(&Event::Answered(*id, Answer::Expired, None))
        })
    }
    fn new_after_expiry(l: &Ledger) -> bool {
        occurred_with(l, |e, h| {
            matches!(e, Event::Decided(key, Decision::Reason(Report {
            reason: ResultKind::Unavailable, rule: "PND-005",
            action: Some(ActionId { kind: ActionKind::Replacement, .. }), ..
        })) if turn(key))
                && h.expired.is_some()
        })
    }
    fn lapse_raises(l: &Ledger) -> bool {
        l.0.iter().any(|e| {
            matches!(
                e,
                Event::Raised(ActionId {
                    kind: ActionKind::Lapse,
                    ..
                })
            )
        })
    }
    fn void(l: &Ledger) -> bool {
        l.0.iter()
            .any(|e| matches!(e, Event::Answered(_, Answer::Void, _)))
    }
    fn interrupted_sign_in(e: Event, event: Interruption) -> bool {
        matches!(e, Event::Ended(id, Ending { outcome: ResultKind::Unknown, interruption: Some(interruption), .. })
            if sign_in(id.key) && interruption == event)
    }
    fn restart_ends_sign_in(l: &Ledger) -> bool {
        l.0.iter()
            .any(|&e| interrupted_sign_in(e, Interruption::Restart))
    }
    fn restart_overlap(l: &Ledger) -> bool {
        let mut waiting = Vec::new();
        let mut restarted = Vec::new();
        for &e in &l.0 {
            match e {
                Event::LoginStarted(id) => waiting.push(id),
                Event::ProgramFinished(id) => waiting.retain(|&p| p != id),
                Event::Ended(id, _) if interrupted_sign_in(e, Interruption::Restart) => {
                    restarted.push(id)
                }
                Event::Decided(key, Decision::Dispatch(_))
                    if sign_in(key) && restarted.iter().any(|old| waiting.contains(old)) =>
                {
                    return true;
                }
                _ => {}
            }
        }
        false
    }
    fn repeat_conflict(l: &Ledger) -> bool {
        occurred_with(l, |e, h| {
            matches!(e, Event::Decided(key, Decision::Reason(Report {
            reason: ResultKind::Conflict, rule: "OPS-032", action: None, invocation: Some(id), ..
        })) if sign_in(key) && h.previous(key) == Some((id, None)))
        })
    }
    fn replay_rejected(l: &Ledger) -> bool {
        occurred_with(l, |e, h| {
            matches!(e, Event::Checked(token, _, StateCheck::Consumed)
            if h.previous(token.id.key) == Some((token.id, None)))
        })
    }
    fn wait_ended(l: &Ledger) -> bool {
        l.0.iter()
            .any(|&e| interrupted_sign_in(e, Interruption::Wait))
    }
    fn removed_refused(l: &Ledger) -> bool {
        occurred_with(l, |e, h| {
            matches!(
                e,
                Event::Decided(
                    FINAL_SIGN,
                    Decision::Reason(Report {
                        reason: ResultKind::Refused,
                        rule: "OPS-023",
                        ..
                    })
                )
            ) && h.lifecycle == Some(Lifecycle::Removed)
        })
    }
    fn wrong_account_preserved(l: &Ledger) -> bool {
        let mut w2_unknown = false;
        let mut history = History::default();
        for (i, &event) in l.0.iter().enumerate() {
            w2_unknown |= matches!(
                event,
                Event::Ended(
                    Attempt { key: W2, .. },
                    Ending {
                        outcome: ResultKind::Unknown,
                        ..
                    }
                )
            );
            if let Event::PartResult(
                id @ Attempt {
                    key: FINAL_SIGN, ..
                },
                Part::FirstAccount,
                PartResult::Account(Account::Other),
                false,
            ) = event
                && w2_unknown
            {
                let mut next = i + 1;
                if history.pending().is_none() {
                    if !matches!(l.0.get(next), Some(Event::Raised(_))) {
                        history.observe(event);
                        continue;
                    }
                    next += 1;
                }
                if l.0.get(next) == Some(&Event::PartDispatched(id, Part::Login)) {
                    return true;
                }
            }
            history.observe(event);
        }
        false
    }
    fn shutdown_unknown(l: &Ledger) -> bool {
        occurred_with(l, |e, h| {
            e == Event::Decided(FINAL_SIGN, Decision::Retired)
                && h.lifecycle == Some(Lifecycle::Retired)
                && h.previous(RETIRE).is_some_and(|(_, end)| {
                    end.is_some_and(|end| end.outcome == ResultKind::Unknown)
                })
        })
    }
    fn undo_interrupted(l: &Ledger) -> bool {
        let mut outstanding = Vec::new();
        for &e in &l.0 {
            match e {
                Event::PartDispatched(id, Part::Undo) => outstanding.push(id),
                Event::PartResult(id, Part::Undo, _, _) => outstanding.retain(|&open| open != id),
                Event::Ended(id, _)
                    if interrupted_sign_in(e, Interruption::Restart)
                        && outstanding.contains(&id) =>
                {
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    #[cfg(test)]
    mod census {
        use super::*;
        use crate::harness::{Action as Step, check_all, threads};
        use crate::measure::{Census, Limits, census};
        use std::hash::{DefaultHasher, Hash, Hasher};
        use std::panic::{AssertUnwindSafe, catch_unwind};
        use std::time::Instant;

        const STATE_CAP: usize = 200_000_000;
        type Projection = fn(&System<SignIn>) -> u64;

        #[derive(Clone, Copy, PartialEq)]
        enum Ablation {
            Expiry,
            PageAsE,
            PageAsOther,
            OneInput,
            UndoSucceeds,
            NoLapse,
            BracketInOrder,
            InternalFirst,
        }

        impl Ablation {
            fn prune(self, s: &System<SignIn>, actions: &mut Vec<Step<Local>>) {
                match self {
                    Self::Expiry => {
                        actions.retain(|a| !matches!(a, Step::Local(E, Local::Expire(_))))
                    }
                    Self::PageAsE => actions.retain(|a| {
                        !matches!(a, Step::Local(H, Local::CompletePage(_, Account::Other)))
                    }),
                    Self::PageAsOther => {
                        let other_pages: Vec<_> = actions
                            .iter()
                            .filter_map(|a| match a {
                                Step::Local(H, Local::CompletePage(id, Account::Other)) => {
                                    Some(*id)
                                }
                                _ => None,
                            })
                            .collect();
                        actions.retain(|a| {
                            !matches!(a, Step::Local(H, Local::CompletePage(id, Account::LowerE))
                                     if other_pages.contains(id))
                        });
                    }
                    Self::OneInput => actions
                        .retain(|a| !matches!(a, Step::Local(E, Local::OfferSecond(_, true)))),
                    Self::UndoSucceeds => {
                        actions.retain(|a| !matches!(a, Step::Local(H, Local::Undo(_, false))))
                    }
                    Self::NoLapse => {
                        actions.retain(|a| !matches!(a, Step::Local(H, Local::Turn(_, false))))
                    }
                    Self::BracketInOrder => {
                        let script = s.nodes[Q].data.q.script;
                        actions.retain(|a| match a {
                            Step::Local(Q, Local::Send(X_SIGN)) => !script.unsent(R_SIGN),
                            Step::Local(Q, Local::Send(FINAL_SIGN)) => !script.unsent(RETIRE),
                            _ => true,
                        });
                    }
                    Self::InternalFirst => {
                        if let Some(first) = actions.iter().position(|a| {
                            matches!(
                                a,
                                Step::Local(E, Local::PassReturn(_) | Local::Poll(_, _))
                                    | Step::Local(H, Local::ApplyReturn(_))
                            )
                        }) {
                            let action = actions.swap_remove(first);
                            actions.clear();
                            actions.push(action);
                        }
                    }
                }
            }
        }

        fn count(
            v: Variant,
            c: Configuration,
            suffix: Option<&str>,
            ablations: &[Ablation],
            projections: &[Projection],
            limits: Limits,
        ) -> (String, Census) {
            let mut row = format!("{v:?}/{c:?}");
            if let Some(suffix) = suffix {
                row.push_str(", ");
                row.push_str(suffix);
            }
            let projections: Vec<_> = projections
                .iter()
                .map(|project| project as &dyn Fn(&System<SignIn>) -> u64)
                .collect();
            let measured = census(
                &model(v, c),
                &|s, actions| {
                    for ablation in ablations {
                        ablation.prune(s, actions);
                    }
                },
                &|s| s.nodes[Q].data.q.script.stage,
                &projections,
                limits,
            );
            println!(
                "MC9a census Sign-in|{row}|{}|{}|{:.1}",
                measured.states, measured.stop, measured.seconds
            );
            (row, measured)
        }

        fn print_stages(row: &str, measured: &Census) {
            let stages: Vec<_> = (0..=8)
                .map(|i| measured.classes.get(i).copied().unwrap_or(0).to_string())
                .collect();
            println!("MC9a stages Sign-in|{row}|{}", stages.join(","));
        }

        #[test]
        #[ignore = "measurement for order 204b; run through its run script"]
        fn census_correct() {
            println!("MC9a measurement test census_correct");
            for c in Configuration::ALL {
                let (row, measured) = count(
                    Variant::Correct,
                    c,
                    None,
                    &[],
                    &[],
                    Limits {
                        states: STATE_CAP,
                        seconds: 600,
                    },
                );
                if c == Configuration::Base {
                    for (kind, (transitions, new_states)) in &measured.kinds {
                        println!("MC9a kind Sign-in|{row}|{kind}|{transitions}|{new_states}");
                    }
                }
                print_stages(&row, &measured);
            }
        }

        fn ablated(suffix: &str, ablations: &[Ablation]) {
            let (row, measured) = count(
                Variant::Correct,
                Configuration::Base,
                Some(suffix),
                ablations,
                &[],
                Limits {
                    states: STATE_CAP,
                    seconds: 300,
                },
            );
            print_stages(&row, &measured);
        }

        #[test]
        #[ignore = "measurement for order 204b; run through its run script"]
        fn census_without() {
            println!("MC9a measurement test census_without");
            for (suffix, ablation) in [
                ("without expiry", Ablation::Expiry),
                ("page as e only", Ablation::PageAsE),
                ("page as O only", Ablation::PageAsOther),
                ("one input request", Ablation::OneInput),
                ("sign-out succeeds", Ablation::UndoSucceeds),
                ("no lapse", Ablation::NoLapse),
                ("bracket in order", Ablation::BracketInOrder),
                ("internal steps first", Ablation::InternalFirst),
            ] {
                ablated(suffix, &[ablation]);
            }
        }

        #[test]
        #[ignore = "measurement for order 204b; run through its run script"]
        fn census_fixed() {
            println!("MC9a measurement test census_fixed");
            let choices = [
                (Ablation::Expiry, "choices fixed, with expiry"),
                (Ablation::PageAsE, "choices fixed, with either page"),
                (
                    Ablation::OneInput,
                    "choices fixed, with a second input request",
                ),
                (
                    Ablation::UndoSucceeds,
                    "choices fixed, with a failed sign-out",
                ),
                (Ablation::NoLapse, "choices fixed, with a lapse"),
                (
                    Ablation::BracketInOrder,
                    "choices fixed, with either bracket order",
                ),
            ];
            let fixed = choices.map(|(ablation, _)| ablation);
            ablated("choices fixed", &fixed);
            for (returned, suffix) in choices {
                let ablations: Vec<_> = fixed.into_iter().filter(|a| *a != returned).collect();
                ablated(suffix, &ablations);
            }
            let mut internal_first = fixed.to_vec();
            internal_first.push(Ablation::InternalFirst);
            ablated("choices fixed, internal steps first", &internal_first);
        }

        fn fingerprint(value: &impl Hash) -> u64 {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            hasher.finish()
        }

        fn without_ledger(s: &System<SignIn>) -> u64 {
            fingerprint(&(&s.nodes, &s.links, &s.cut, &s.used))
        }

        fn ledger_unordered(s: &System<SignIn>) -> u64 {
            let mut events: Vec<_> = s.ledger.0.iter().map(fingerprint).collect();
            events.sort_unstable();
            fingerprint(&(&s.nodes, &s.links, &s.cut, &s.used, &events))
        }

        #[test]
        #[ignore = "measurement for order 204b; run through its run script"]
        fn census_projections() {
            println!("MC9a measurement test census_projections");
            let (_, measured) = count(
                Variant::Correct,
                Configuration::Base,
                Some("projections"),
                &[],
                &[without_ledger, ledger_unordered, |s| fingerprint(&s.ledger)],
                Limits {
                    states: 100_000_000,
                    seconds: 600,
                },
            );
            for (projection, values) in ["without the ledger", "ledger unordered", "ledger alone"]
                .into_iter()
                .zip(measured.projections)
            {
                println!("MC9a projection Sign-in|Correct/Base|{projection}|{values}");
            }
        }

        fn mutant(v: Variant, c: Configuration) {
            count(
                v,
                c,
                None,
                &[],
                &[],
                Limits {
                    states: STATE_CAP,
                    seconds: 180,
                },
            );
        }

        #[test]
        #[ignore = "measurement for order 204b; run through its run script"]
        fn census_mutants_own() {
            println!("MC9a measurement test census_mutants_own");
            for v in Variant::BROKEN {
                mutant(v, v.configuration());
            }
        }

        #[test]
        #[ignore = "measurement for order 204b; run through its run script"]
        fn census_mutants_base() {
            println!("MC9a measurement test census_mutants_base");
            for v in Variant::BROKEN
                .into_iter()
                .filter(|v| v.configuration() != Configuration::Base)
            {
                mutant(v, Configuration::Base);
            }
        }

        fn throughput(ceiling: usize) {
            let mut model = model(Variant::Correct, Configuration::Base);
            model.config.unique_state_ceiling = ceiling;
            model.config.timeout = Duration::from_secs(600);
            let threads = threads();
            let start = Instant::now();
            let result = catch_unwind(AssertUnwindSafe(|| check_all(model)));
            let seconds = start.elapsed().as_secs_f64();
            let ended = match result {
                Ok(report) => format!("exhausted at {}", report.unique_states),
                Err(payload) => {
                    let message = payload
                        .downcast_ref::<String>()
                        .map(String::as_str)
                        .or_else(|| payload.downcast_ref::<&str>().copied());
                    assert!(message.is_some_and(|text| text.contains("run truncated")));
                    "truncated".to_owned()
                }
            };
            println!(
                "MC9a throughput Sign-in|Correct/Base|{ceiling}|{threads}|{seconds:.3}|{ended}"
            );
        }

        #[test]
        #[ignore = "measurement for order 204b; run through its run script"]
        fn throughput_400k() {
            println!("MC9a measurement test throughput_400k");
            throughput(400_000);
        }

        #[test]
        #[ignore = "measurement for order 204b; run through its run script"]
        fn throughput_1m() {
            println!("MC9a measurement test throughput_1m");
            throughput(1_000_000);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harness::{Report, assert_reached, assert_refuted, check_all};
    use stateright::{Expectation, Model, Path};

    type Verdict = (Vec<&'static str>, Vec<&'static str>);
    const RECORDED_DIFFERENCES: &[(&str, &[&str], &[&str])] = &[];

    fn sign_in_rows() -> impl Iterator<Item = (sign_in::Variant, sign_in::Configuration)> {
        use super::sign_in::{Configuration as C, Variant as V};
        C::ALL
            .into_iter()
            .map(|c| (V::Correct, c))
            .chain(V::BROKEN.into_iter().map(|v| (v, v.configuration())))
            .chain(
                V::BROKEN
                    .into_iter()
                    .filter(|v| v.configuration() != C::Base)
                    .map(|v| (v, C::Base)),
            )
    }
    fn actions_rows() -> impl Iterator<Item = (actions::Variant, actions::Configuration)> {
        use super::actions::{Configuration as C, Variant as V};
        C::ALL
            .into_iter()
            .map(|c| (V::Correct, c))
            .chain(V::BROKEN.into_iter().map(|v| (v, v.configuration())))
            .chain(
                V::BROKEN
                    .into_iter()
                    .filter(|v| v.configuration() != C::Base)
                    .map(|v| (v, C::Base)),
            )
    }

    mod sign_in_table_data {
        use super::sign_in::{Configuration as C, REACH, Variant as V};
        use super::*;

        pub fn registered(v: V, c: C) -> Verdict {
            let refuted = if v == V::Correct || (c == C::Base && v.configuration() != C::Base) {
                vec![]
            } else {
                vec![v.target()]
            };
            let removed: &[usize] = match v {
                V::NoConflict | V::Unavailable => &[3, 4],
                V::CopyAsNew => &[19],
                V::ConflictForever => &[18, 24],
                V::WrongAccountKept => &[8, 9, 10],
                V::UndoInitial => &[10],
                V::ReturnTwice => &[7],
                V::ReplayState => &[20],
                V::ExpiryLifts => &[14],
                V::RetainHold => &[12],
                _ => &[],
            };
            let missing = REACH
                .into_iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    let reached = matches!(i, 0..=16 | 22 | 23)
                        || match c {
                            C::CopyQ => matches!(i, 19 | 20),
                            C::Crash => matches!(i, 17 | 18 | 24 | 25 | 26),
                            C::Wait => matches!(i, 21 | 24),
                            _ => false,
                        };
                    (!reached || removed.contains(&i)).then_some(name)
                })
                .collect();
            (refuted, missing)
        }
    }

    mod actions_table_data {
        use super::actions::{Configuration as C, REACH, SAFETY, Variant as V};
        use super::*;

        pub fn registered(v: V, c: C) -> Verdict {
            let refuted = if v == V::Correct || (c == C::Base && v.configuration() != C::Base) {
                vec![]
            } else {
                match v {
                    V::VolatileAnswers => vec![SAFETY[0], SAFETY[2], SAFETY[3], SAFETY[4]],
                    V::ForgetOnPurge => vec![SAFETY[0], SAFETY[2], SAFETY[3]],
                    V::RecomputeDismissed => vec![SAFETY[0], SAFETY[4]],
                    _ => vec![v.target()],
                }
            };
            let removed: &[usize] = match v {
                V::WrongScope => &[8],
                V::AgentSees => &[1],
                V::SkipRaisedAt => &[6, 8, 9],
                V::AnswerTwice => &[13],
                V::AnswerExpired => &[11],
                _ => &[],
            };
            let missing = REACH
                .into_iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    ((i == 13 && c != C::CopyQ) || removed.contains(&i)).then_some(name)
                })
                .collect();
            (refuted, missing)
        }
    }

    fn observed(
        part: &str,
        label: &str,
        report: &Report,
        safety: &[&str],
        reach: &[&str],
    ) -> Verdict {
        let safeties: String = safety
            .iter()
            .map(|&n| if report.broken.contains(&n) { 'F' } else { 'H' })
            .collect();
        let witnesses: String = reach
            .iter()
            .map(|&n| {
                if report.missing_witnesses.contains(&n) {
                    '-'
                } else {
                    '+'
                }
            })
            .collect();
        println!(
            "MC9a row {part}|{label}|{}|{}|{:.3}|{safeties}|{witnesses}",
            report.unique_states, report.depth, report.seconds
        );
        assert!(
            !report
                .missing_witnesses
                .iter()
                .any(|name| !reach.contains(name)),
            "MC9a table mismatch: {part} {label}"
        );
        (report.broken.clone(), report.missing_witnesses.clone())
    }

    fn recorded(label: &str) -> Option<Verdict> {
        RECORDED_DIFFERENCES
            .iter()
            .find_map(|&(row, broken, missing)| {
                (label == row).then(|| (broken.to_vec(), missing.to_vec()))
            })
    }

    fn own_differences(part: &str) -> usize {
        RECORDED_DIFFERENCES
            .iter()
            .filter(|entry| {
                entry
                    .0
                    .split_once('|')
                    .is_some_and(|(owner, _)| owner == part)
            })
            .count()
    }

    fn sorted(mut verdict: Verdict) -> Verdict {
        verdict.0.sort_unstable();
        verdict.1.sort_unstable();
        verdict
    }

    fn check_row(part: &str, label: &str, actual: Verdict, registered: Verdict) -> bool {
        let row = format!("{part}|{label}");
        let difference = recorded(&row);
        let differs = difference.is_some();
        if differs {
            println!("MC9a difference {row}");
        }
        assert_eq!(
            sorted(actual),
            sorted(difference.unwrap_or(registered)),
            "MC9a table mismatch: {part} {label}"
        );
        differs
    }

    #[test]
    #[ignore = "MC9a waits for its dated revision after order 204b's measurement"]
    fn sign_in_table() {
        use super::sign_in::*;
        println!("MC9a table Sign-in");
        let mut differences = 0;
        for (v, c) in sign_in_rows() {
            let label = format!("{v:?}/{c:?}");
            let actual = observed("Sign-in", &label, &check_all(model(v, c)), &SAFETY, &REACH);
            differences += usize::from(check_row(
                "Sign-in",
                &label,
                actual,
                sign_in_table_data::registered(v, c),
            ));
        }
        assert_eq!(differences, own_differences("Sign-in"));
    }

    #[test]
    #[ignore = "MC9a waits for its dated revision after order 204b's measurement"]
    fn actions_table() {
        use super::actions::*;
        println!("MC9a table Actions");
        let mut differences = 0;
        for (v, c) in actions_rows() {
            let label = format!("{v:?}/{c:?}");
            let actual = observed("Actions", &label, &check_all(model(v, c)), &SAFETY, &REACH);
            differences += usize::from(check_row(
                "Actions",
                &label,
                actual,
                actions_table_data::registered(v, c),
            ));
        }
        assert_eq!(differences, own_differences("Actions"));
    }

    fn first<P: Protocol>(label: &str, model: Harness<P>, target: &'static str)
    where
        P::Node: Default,
    {
        if recorded(label).is_some_and(|(broken, _)| !broken.contains(&target)) {
            println!("MC9a first {label}|{target}|not refuted");
            return;
        }
        println!("MC9a trace {label}|{target}");
        let trace = assert_refuted(model.clone(), target);
        let path = Path::from_actions(&model, model.init_states().remove(0), &trace).unwrap();
        let mut state = path.last_state().clone();
        for node in &mut state.nodes {
            node.data = P::Node::default();
        }
        for link in &mut state.links {
            link.clear();
        }
        assert!(
            !(model.property(target).condition)(&model, &state),
            "safety must read only the ledger"
        );
        println!("MC9a first {label}|{target}|{} actions", trace.len());
    }

    #[test]
    #[ignore = "MC9a waits for its dated revision after order 204b's measurement"]
    fn sign_in_first_refutations() {
        use super::sign_in::*;
        println!("MC9a test sign_in_first_refutations");
        for v in Variant::BROKEN {
            let c = v.configuration();
            first(&format!("Sign-in|{v:?}/{c:?}"), model(v, c), v.target());
        }
    }

    #[test]
    #[ignore = "MC9a waits for its dated revision after order 204b's measurement"]
    fn actions_first_refutations() {
        use super::actions::*;
        println!("MC9a test actions_first_refutations");
        for v in Variant::BROKEN {
            let c = v.configuration();
            first(&format!("Actions|{v:?}/{c:?}"), model(v, c), v.target());
        }
    }

    fn trace_differences<P: Protocol>(
        model: Harness<P>,
        row: &str,
        expected: Verdict,
        observed: Verdict,
    ) {
        for property in observed.0.iter().filter(|name| !expected.0.contains(name)) {
            println!("MC9a trace {row}|{property}");
            let count = assert_refuted(model.clone(), property).len();
            println!("MC9a difference trace {row}|{property}|{count} actions");
        }
        for witness in expected.1.iter().filter(|name| !observed.1.contains(name)) {
            println!("MC9a trace {row}|{witness}");
            let count = assert_reached(model.clone(), witness).len();
            println!("MC9a difference trace {row}|{witness}|{count} actions");
        }
        for witness in observed.1.iter().filter(|name| !expected.1.contains(name)) {
            println!("MC9a difference trace {row}|{witness}|not reached");
        }
        for property in expected.0.iter().filter(|name| !observed.0.contains(name)) {
            println!("MC9a difference trace {row}|{property}|not refuted");
        }
    }

    #[test]
    #[ignore = "MC9a waits for its dated revision after order 204b's measurement"]
    fn difference_traces() {
        println!("MC9a test difference_traces");
        for (v, c) in sign_in_rows() {
            let row = format!("Sign-in|{v:?}/{c:?}");
            if let Some(actual) = recorded(&row) {
                trace_differences(
                    sign_in::model(v, c),
                    &row,
                    sign_in_table_data::registered(v, c),
                    actual,
                );
            }
        }
        for (v, c) in actions_rows() {
            let row = format!("Actions|{v:?}/{c:?}");
            if let Some(actual) = recorded(&row) {
                trace_differences(
                    actions::model(v, c),
                    &row,
                    actions_table_data::registered(v, c),
                    actual,
                );
            }
        }
    }

    #[test]
    #[ignore = "MC9a waits for its dated revision after order 204b's measurement"]
    fn finding_witnesses() {
        use super::sign_in::*;
        println!("MC9a test finding_witnesses");
        for (c, reach) in [
            (Configuration::Crash, REACH[18]),
            (Configuration::Base, REACH[5]),
            (Configuration::Base, REACH[12]),
        ] {
            let row = format!("Sign-in|Correct/{c:?}");
            if recorded(&row).is_some_and(|(_, missing)| missing.contains(&reach)) {
                println!("MC9a witness {row}|{reach}|not reached");
            } else {
                println!("MC9a trace {row}|{reach}");
                let count = assert_reached(model(Variant::Correct, c), reach).len();
                println!("MC9a witness {row}|{reach}|{count} actions");
            }
        }
    }

    #[test]
    fn configuration_and_property_inventory() {
        println!("MC9a test configuration_and_property_inventory");
        fn inventory<P: Protocol>(model: Harness<P>, safety: &[&str], reach: &[&str]) {
            model.config.validate();
            assert_eq!(model.config.unique_state_ceiling, 400_000);
            assert_eq!(model.config.timeout, Duration::from_secs(15));
            assert_eq!(model.config.messages_per_link, 8);
            assert!(model.protocol.gate_remaining_faults());
            let properties = model.protocol.properties();
            assert_eq!(properties.len(), safety.len() + reach.len());
            for (p, (&name, expectation)) in properties.iter().zip(
                safety
                    .iter()
                    .zip(std::iter::repeat(Expectation::Always))
                    .chain(reach.iter().zip(std::iter::repeat(Expectation::Sometimes))),
            ) {
                assert_eq!(
                    (p.property.name, &p.property.expectation),
                    (name, &expectation)
                );
            }
        }
        for c in sign_in::Configuration::ALL {
            inventory(
                sign_in::model(sign_in::Variant::Correct, c),
                &sign_in::SAFETY,
                &sign_in::REACH,
            );
        }
        for c in actions::Configuration::ALL {
            inventory(
                actions::model(actions::Variant::Correct, c),
                &actions::SAFETY,
                &actions::REACH,
            );
        }
    }
}
