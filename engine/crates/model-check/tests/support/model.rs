use conduit_core::{
    Context, Evidence, Input, State, Transition, access::AccessRequest, requests::Completion,
};
use serde_json::{Value, json};
use stateright::{Model, Property};
use std::collections::BTreeSet;
use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::sync::{Arc, OnceLock};

use super::search::SearchGuard;

// Experiment 012, src/operations.rs, MC2 rerun under order 160.
pub const ONCE: &str = "OPS-018 OPS-008 OPS-012 OPS-024: at most once";
pub const RIGHT: &str = "OPS-012 OPS-014: no wrong outcome";
pub const UNKNOWN: &str = "OPS-013 IMP-007: unknown stays unknown";
pub const SAFETY: [&str; 3] = [ONCE, RIGHT, UNKNOWN];

const MAX_SENDS: u8 = 2;
const MAX_STOPS: u8 = 1;
const MAX_LINK_MESSAGES: usize = 3;
// Each request can add a discovery, a call and an answer. A loss replaces delivery.
pub const MAX_TRACE_STATES: usize = 4 * MAX_SENDS as usize + MAX_STOPS as usize + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Configuration {
    Plain,
    Deduplicating,
    OControl,
    WControl,
    UControl,
}

impl Configuration {
    pub fn label(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Deduplicating => "deduplicating",
            Self::OControl => "o-control",
            Self::WControl => "w-control",
            Self::UControl => "u-control",
        }
    }

    fn aware(self) -> bool {
        self != Self::Plain
    }

    pub fn control(self) -> Option<&'static str> {
        match self {
            Self::OControl => Some(ONCE),
            Self::WControl => Some(RIGHT),
            Self::UControl => Some(UNKNOWN),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    T,
    L,
    R,
    LR,
}

impl Part {
    fn loss(self) -> bool {
        matches!(self, Self::L | Self::LR)
    }

    fn reorder(self) -> bool {
        matches!(self, Self::R | Self::LR)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Attempt {
    invocation: String,
    number: u64,
}

impl Attempt {
    fn from_emission(value: &Value) -> Self {
        Self {
            invocation: value["invocation"]
                .as_str()
                .expect("emitted invocation ID")
                .into(),
            number: value["attempt"].as_u64().expect("emitted attempt number"),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Outcome {
    Succeeded,
    Failed,
    Unknown,
}

impl Outcome {
    fn parse(value: &Value) -> Option<Self> {
        match value.as_str() {
            Some("succeeded") => Some(Self::Succeeded),
            Some("failed") => Some(Self::Failed),
            Some("unknown") => Some(Self::Unknown),
            None if value.is_null() => None,
            _ => panic!("unexpected outcome in core emission"),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Answer {
    Succeeded,
    Failed,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Event {
    Dispatch(Attempt),
    Effect(Attempt),
    AnswerSent(Attempt, Answer),
    AnswerDelivered(Attempt, Answer),
    Outcome(Attempt, Outcome, usize),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Ledger {
    allows_unknown_retry: bool,
    events: Vec<Event>,
}

impl Ledger {
    fn effects(&self) -> usize {
        self.events
            .iter()
            .filter(|event| matches!(event, Event::Effect(_)))
            .count()
    }

    fn outcome(&self, expected: Outcome) -> bool {
        self.events
            .iter()
            .any(|e| matches!(e, Event::Outcome(_, value, _) if *value == expected))
    }

    fn earlier_unanswered(&self, current: &Attempt) -> bool {
        self.events.iter().any(|event| {
            let Event::Dispatch(earlier) = event else {
                return false;
            };
            earlier.invocation == current.invocation
                && earlier.number < current.number
                && !self.events.iter().any(|event| {
                    matches!(event, Event::AnswerDelivered(attempt, _) if attempt == earlier)
                })
        })
    }

    fn once(&self) -> bool {
        self.effects() <= 1
    }

    fn right(&self) -> bool {
        let mut current = None;
        for event in &self.events {
            match event {
                Event::Dispatch(_) => current = None,
                Event::Effect(_) if current == Some(Outcome::Failed) => return false,
                Event::Outcome(_, outcome, effects) => {
                    if (*outcome == Outcome::Succeeded && *effects == 0)
                        || (*outcome == Outcome::Failed && *effects != 0)
                    {
                        return false;
                    }
                    current = Some(*outcome);
                }
                _ => {}
            }
        }
        true
    }

    fn unknown(&self) -> bool {
        let mut unknown = false;
        for event in &self.events {
            match event {
                Event::Dispatch(_) => {
                    if unknown && !self.allows_unknown_retry {
                        return false;
                    }
                    unknown = false;
                }
                Event::Outcome(_, outcome, _) => {
                    if unknown && *outcome != Outcome::Unknown {
                        return false;
                    }
                    unknown = *outcome == Outcome::Unknown;
                }
                _ => {}
            }
        }
        true
    }
}

impl fmt::Display for Ledger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for event in &self.events {
            let (attempt, description) = match event {
                Event::Dispatch(attempt) => (attempt, "dispatch".into()),
                Event::Effect(attempt) => (attempt, "effect applied".into()),
                Event::AnswerSent(attempt, answer) => (attempt, format!("answer sent {answer:?}")),
                Event::AnswerDelivered(attempt, answer) => {
                    (attempt, format!("answer delivered {answer:?}"))
                }
                Event::Outcome(attempt, outcome, effects) => (
                    attempt,
                    format!("outcome {} with {effects} effects", outcome.name()),
                ),
            };
            writeln!(
                f,
                "  invocation {}, attempt {}: {description}",
                attempt.invocation, attempt.number
            )?;
        }
        Ok(())
    }
}

// The fixture holds every other core field. Assert those fields did not change before
// sharing them between explored states. The three mutable fields are kept in full.
#[derive(Debug, PartialEq, Eq)]
struct CoreHistory {
    invocations: Vec<Value>,
    records: Vec<Value>,
    allocated_ids: BTreeSet<String>,
    hash: u64,
}

impl CoreHistory {
    fn take(state: &mut State) -> Self {
        let invocations = std::mem::take(&mut state.invocations);
        let records = std::mem::take(&mut state.records);
        let allocated_ids = std::mem::take(&mut state.allocated_ids);
        let mut hasher = DefaultHasher::new();
        invocations.hash(&mut hasher);
        records.hash(&mut hasher);
        allocated_ids.hash(&mut hasher);
        Self {
            invocations,
            records,
            allocated_ids,
            hash: hasher.finish(),
        }
    }

    fn restore(&self, fixed: &State) -> State {
        State {
            invocations: self.invocations.clone(),
            records: self.records.clone(),
            allocated_ids: self.allocated_ids.clone(),
            ..fixed.clone()
        }
    }
}

impl Hash for CoreHistory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

struct Fixture {
    context: Context,
    fixed: State,
    initial: Arc<CoreHistory>,
    time: String,
    request: AccessRequest,
    key: Option<String>,
}

impl Fixture {
    fn load(aware: bool) -> &'static Self {
        static PLAIN: OnceLock<Fixture> = OnceLock::new();
        static DEDUPLICATING: OnceLock<Fixture> = OnceLock::new();
        let (cache, directory) = if aware {
            (&DEDUPLICATING, "repeat-failed-earlier-unanswered")
        } else {
            (&PLAIN, "unknown-write")
        };
        cache.get_or_init(|| {
            let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
            let path = Path::new("conformance/conventions/requests")
                .join(directory)
                .join("case.yaml");
            let (context, mut fixed, input) =
                conduit_conformance::request_fixture_at(&repository, &path, 2)
                    .unwrap_or_else(|e| panic!("fixture {directory}: {e}"));
            let Input::Invoke {
                evidence,
                request,
                key,
                archive,
            } = input
            else {
                panic!("fixture {directory}: expected an invocation");
            };
            assert!(
                key.is_some() && archive.is_none(),
                "expected a keyed request"
            );
            let initial = Arc::new(CoreHistory::take(&mut fixed));
            Self {
                context,
                fixed,
                initial,
                time: evidence.time,
                request,
                key,
            }
        })
    }

    fn mark(&self, name: &str) -> String {
        format!("{}{name}", self.context.meta_prefix)
    }

    fn evidence(&self, step: u16) -> Evidence {
        Evidence {
            time: self.time.clone(),
            // Separate slots for each successive core step on a path.
            ids: (0..16)
                .map(|slot| format!("00000000-0000-4000-8000-9{step:03}{slot:08}"))
                .collect(),
            revisions: vec![],
            suffixes: vec![],
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct World {
    trace: Option<TracePath>,
    core: Arc<CoreHistory>,
    core_steps: u16,
    sends: u8,
    stops: u8,
    lost: bool,
    reordered: bool,
    calls: Vec<Attempt>,
    answers: Vec<(Attempt, Answer)>,
    discovery: Option<Attempt>,
    waiting: Option<Attempt>,
    first_answer: Option<Answer>,
    used_failure: bool,
    repeat_without_dispatch: bool,
    retried_unknown: bool,
    failed_while_earlier_unanswered: bool,
    pub ledger: Ledger,
}

// The path is diagnostic context, never a source of successor actions or invariants.
#[derive(Clone, Debug, Default)]
struct TracePath(Vec<Action>);

impl PartialEq for TracePath {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Eq for TracePath {}
impl Hash for TracePath {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link {
    ToImplementation,
    ToEngine,
}

impl Link {
    fn label(self) -> &'static str {
        match self {
            Self::ToImplementation => "engine -> implementation",
            Self::ToEngine => "implementation -> engine",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Send,
    Discover,
    Stop,
    ReceiveCall { position: usize, fail: bool },
    ReceiveAnswer { position: usize },
    Lose { link: Link, position: usize },
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let delivery = |position| {
            if position == 0 {
                "delivered"
            } else {
                "delivered out of order"
            }
        };
        match *self {
            Self::Send => write!(f, "caller sent the request"),
            Self::Discover => write!(f, "implementation answered discovery"),
            Self::Stop => write!(f, "engine stopped waiting"),
            Self::ReceiveCall { position, fail } => {
                let choice = if fail { " (choose failure)" } else { "" };
                write!(
                    f,
                    "{} {} [position {position}]{choice}",
                    delivery(position),
                    Link::ToImplementation.label()
                )
            }
            Self::ReceiveAnswer { position } => {
                write!(
                    f,
                    "{} {} [position {position}]",
                    delivery(position),
                    Link::ToEngine.label()
                )
            }
            Self::Lose { link, position } => {
                write!(f, "lost {} [position {position}]", link.label())
            }
        }
    }
}

#[derive(Clone)]
pub struct Operations {
    pub configuration: Configuration,
    pub part: Part,
    fixture: &'static Fixture,
    pub guard: Option<Arc<SearchGuard>>,
}

impl Operations {
    fn safety(&self, state: &World, name: &'static str, holds: bool) -> bool {
        if !holds
            && let Some(guard) = &self.guard
            && let Some(path) = &state.trace
        {
            guard.capture(name, &path.0, &state.ledger);
        }
        holds
    }

    pub fn new(configuration: Configuration, part: Part) -> Self {
        assert!(configuration != Configuration::Plain || !part.reorder());
        assert!(configuration.control().is_none() || part == Part::T);
        Self {
            configuration,
            part,
            fixture: Fixture::load(configuration.aware()),
            guard: None,
        }
    }

    fn observe(&self, world: &mut World, transition: &Transition, caller_send: bool) {
        let mark = |name| self.fixture.mark(name);
        assert!(
            transition.input_required.is_empty(),
            "unexpected input relay"
        );
        for intent in &transition.external_intents {
            assert_eq!(intent["kind"], "dispatch");
            let attempt = Attempt::from_emission(intent);
            match intent["call"]["method"].as_str() {
                Some("server/discover") => {
                    assert!(self.configuration.aware());
                    assert!(world.discovery.replace(attempt).is_none());
                }
                Some("tools/call") => {
                    // Both prepared fixtures declare deduplication. Only one is plain,
                    // so validate the oracle against the contract the core selected.
                    let invocation = transition
                        .state
                        .invocations
                        .iter()
                        .find(|v| v["id"] == attempt.invocation && v["attempt"] == attempt.number)
                        .expect("dispatch has a prepared invocation");
                    assert!(
                        invocation["contract"]["spec"]["deduplicates"] == true,
                        "fixture operation must declare deduplication"
                    );
                    assert_eq!(
                        invocation["plain"],
                        !self.configuration.aware(),
                        "fixture implementation must match the configured plain setting"
                    );
                    let wire = &intent["call"]["params"]["_meta"][mark("idempotencyKey")];
                    if self.configuration.aware() {
                        assert!(
                            wire.as_str() == self.fixture.key.as_deref(),
                            "call correlation changed"
                        );
                    } else {
                        assert!(wire.is_null(), "plain call carried a key");
                    }
                    if let Some(Event::Outcome(_, Outcome::Unknown, _)) = world
                        .ledger
                        .events
                        .iter()
                        .rev()
                        .find(|e| matches!(e, Event::Dispatch(_) | Event::Outcome(_, _, _)))
                    {
                        world.retried_unknown = true;
                    }
                    assert!(
                        world.waiting.replace(attempt.clone()).is_none(),
                        "overlapping core waits"
                    );
                    world.ledger.events.push(Event::Dispatch(attempt.clone()));
                    world.calls.push(attempt);
                }
                _ => panic!("unexpected call intent"),
            }
        }

        let mut outcomes = Vec::new();
        for record in &transition.records {
            if record["type"] == "invocation-ended" {
                let data = &record["data"];
                let outcome =
                    Outcome::parse(&data["outcome"]).expect("ending record has an outcome");
                outcomes.push((Attempt::from_emission(data), outcome));
            }
        }
        let outcome_key = mark("outcome");
        for reply in &transition.request_replies {
            if let Some(outcome) = Outcome::parse(&reply.reply["_meta"][&outcome_key]) {
                let event = (
                    Attempt {
                        invocation: reply.invocation.clone(),
                        number: reply.attempt,
                    },
                    outcome,
                );
                if !outcomes.contains(&event) {
                    outcomes.push(event);
                }
            }
        }
        let meta = &transition.reply["_meta"];
        if let Some(outcome) = Outcome::parse(&meta[&outcome_key]) {
            let invocation = meta[mark("invocation")]
                .as_str()
                .expect("reply invocation ID");
            if !outcomes
                .iter()
                .any(|(attempt, observed)| attempt.invocation == invocation && *observed == outcome)
            {
                let attempt = world
                    .ledger
                    .events
                    .iter()
                    .rev()
                    .find_map(|e| match e {
                        Event::Dispatch(attempt) if attempt.invocation == invocation => {
                            Some(attempt)
                        }
                        _ => None,
                    })
                    .expect("reply names a dispatched invocation")
                    .clone();
                outcomes.push((attempt, outcome));
            }
        }
        if caller_send
            && world.sends > 1
            && transition.external_intents.is_empty()
            && (!outcomes.is_empty() || meta[mark("reason")].is_string())
        {
            world.repeat_without_dispatch = true;
        }
        let effects = world.ledger.effects();
        for (attempt, outcome) in outcomes {
            if !caller_send && world.waiting.as_ref() == Some(&attempt) {
                world.waiting = None;
            }
            world
                .ledger
                .events
                .push(Event::Outcome(attempt, outcome, effects));
        }
    }

    fn receive_call(&self, world: &mut World, position: usize, fail: bool) {
        let attempt = world.calls.remove(position);
        let (answer, applies) = match self.configuration {
            Configuration::OControl => (Answer::Succeeded, true),
            _ if self.configuration.aware() && world.first_answer.is_some() => {
                (world.first_answer.unwrap(), false)
            }
            Configuration::WControl => (Answer::Failed, true),
            _ if fail => (Answer::Failed, false),
            _ => (Answer::Succeeded, true),
        };
        if self.configuration.aware() {
            world.first_answer.get_or_insert(answer);
        }
        world.used_failure |= fail;
        if applies {
            world.ledger.events.push(Event::Effect(attempt.clone()));
        }
        world
            .ledger
            .events
            .push(Event::AnswerSent(attempt.clone(), answer));
        world.answers.push((attempt, answer));
    }

    fn complete(evidence: Evidence, attempt: Attempt, completion: Completion) -> Input {
        Input::Complete {
            evidence,
            invocation: attempt.invocation,
            attempt: attempt.number,
            completion,
        }
    }
}

impl Model for Operations {
    type State = World;
    type Action = Action;

    fn init_states(&self) -> Vec<World> {
        vec![World {
            trace: self
                .guard
                .as_ref()
                .and_then(|guard| guard.target.map(|_| TracePath::default())),
            core: self.fixture.initial.clone(),
            core_steps: 0,
            sends: 0,
            stops: 0,
            lost: false,
            reordered: false,
            calls: vec![],
            answers: vec![],
            discovery: None,
            waiting: None,
            first_answer: None,
            used_failure: false,
            repeat_without_dispatch: false,
            retried_unknown: false,
            failed_while_earlier_unanswered: false,
            ledger: Ledger {
                allows_unknown_retry: self.configuration.aware()
                    && self.configuration != Configuration::UControl,
                events: vec![],
            },
        }]
    }

    fn actions(&self, s: &World, out: &mut Vec<Action>) {
        if s.discovery.is_some() {
            out.push(Action::Discover);
            return;
        }
        if s.sends < MAX_SENDS {
            out.push(Action::Send);
        }
        if s.waiting.is_some() && s.stops < MAX_STOPS {
            out.push(Action::Stop);
        }
        let may_fail = !s.used_failure
            && s.first_answer.is_none()
            && !matches!(
                self.configuration,
                Configuration::OControl | Configuration::WControl
            );
        for (link, length) in [
            (Link::ToImplementation, s.calls.len()),
            (Link::ToEngine, s.answers.len()),
        ] {
            for position in 0..length {
                if position == 0 || self.part.reorder() && !s.reordered {
                    match link {
                        Link::ToImplementation => {
                            out.push(Action::ReceiveCall {
                                position,
                                fail: false,
                            });
                            if may_fail {
                                out.push(Action::ReceiveCall {
                                    position,
                                    fail: true,
                                });
                            }
                        }
                        Link::ToEngine => out.push(Action::ReceiveAnswer { position }),
                    }
                }
                if self.part.loss() && !s.lost {
                    out.push(Action::Lose { link, position });
                }
            }
        }
    }

    fn next_state(&self, state: &World, action: Action) -> Option<World> {
        let mut next = state.clone();
        if let Some(path) = &mut next.trace {
            path.0.push(action);
        }
        let evidence = || self.fixture.evidence(state.core_steps + 1);
        let mut later_failed = None;
        let input = match action {
            Action::Send => {
                next.sends += 1;
                Some(Input::Invoke {
                    evidence: evidence(),
                    request: self.fixture.request.clone(),
                    key: self.fixture.key.clone(),
                    archive: None,
                })
            }
            Action::Discover => {
                let attempt = next.discovery.take().unwrap();
                let extension = self.fixture.mark("implementation-interface");
                Some(Self::complete(
                    evidence(),
                    attempt,
                    Completion::Result {
                        result: json!({"capabilities":{"extensions":{extension:{"revision":"v0"}}}}),
                    },
                ))
            }
            Action::Stop => {
                next.stops += 1;
                Some(Self::complete(
                    evidence(),
                    next.waiting.clone().unwrap(),
                    Completion::Missing,
                ))
            }
            Action::ReceiveCall { position, fail } => {
                next.reordered |= position > 0;
                self.receive_call(&mut next, position, fail);
                None
            }
            Action::ReceiveAnswer { position } => {
                next.reordered |= position > 0;
                let (attempt, answer) = next.answers.remove(position);
                if answer == Answer::Failed
                    && next.waiting.as_ref() == Some(&attempt)
                    && next.ledger.earlier_unanswered(&attempt)
                {
                    later_failed = Some(attempt.clone());
                }
                next.ledger
                    .events
                    .push(Event::AnswerDelivered(attempt.clone(), answer));
                let mut result = json!({"content":[],"isError":answer == Answer::Failed});
                if self.configuration.aware() {
                    result["_meta"][self.fixture.mark("outcome")] = json!(match answer {
                        Answer::Succeeded => "succeeded",
                        Answer::Failed => "failed",
                    });
                }
                Some(Self::complete(
                    evidence(),
                    attempt,
                    Completion::Result { result },
                ))
            }
            Action::Lose { link, position } => {
                next.lost = true;
                match link {
                    Link::ToImplementation => {
                        next.calls.remove(position);
                    }
                    Link::ToEngine => {
                        next.answers.remove(position);
                    }
                }
                None
            }
        };
        if let Some(input) = input {
            let before = next.ledger.events.len();
            let core = state.core.restore(&self.fixture.fixed);
            let mut transition = conduit_core::step(&core, input, &self.fixture.context)
                .unwrap_or_else(|e| panic!("core step failed during {action}: {e}"));
            next.core_steps += 1;
            self.observe(&mut next, &transition, action == Action::Send);
            if let Some(attempt) = later_failed {
                next.failed_while_earlier_unanswered |= next.ledger.events[before..].iter().any(|event| {
                    matches!(event, Event::Outcome(ended, Outcome::Unknown, _) if *ended == attempt)
                });
            }
            next.core = Arc::new(CoreHistory::take(&mut transition.state));
            assert!(
                transition.state == self.fixture.fixed,
                "core changed a shared fixture field"
            );
        }
        assert!(
            next.calls.len() <= MAX_LINK_MESSAGES && next.answers.len() <= MAX_LINK_MESSAGES,
            "link exceeds its registered bound"
        );
        if self.configuration == Configuration::Plain {
            assert!(
                next.calls.len() <= 1 && next.answers.len() <= 1,
                "plain configuration admits reordering"
            );
        }
        Some(next)
    }

    fn properties(&self) -> Vec<Property<Self>> {
        let mut properties = vec![
            Property::<Self>::always(ONCE, |m, s| m.safety(s, ONCE, s.ledger.once())),
            Property::<Self>::always(RIGHT, |m, s| m.safety(s, RIGHT, s.ledger.right())),
            Property::<Self>::always(UNKNOWN, |m, s| m.safety(s, UNKNOWN, s.ledger.unknown())),
            // Stateright can stop expanding when every property has a discovery.
            Property::<Self>::always("harness: continue exhaustive exploration", |_, _| true),
            Property::<Self>::sometimes("effect applied", |_, s| s.ledger.effects() > 0),
            Property::<Self>::sometimes("outcome unknown", |_, s| {
                s.ledger.outcome(Outcome::Unknown)
            }),
            Property::<Self>::sometimes("repeat answered without dispatch", |_, s| {
                s.repeat_without_dispatch
            }),
        ];
        if self.configuration != Configuration::WControl {
            properties.push(Property::<Self>::sometimes("outcome succeeded", |_, s| {
                s.ledger.outcome(Outcome::Succeeded)
            }));
        }
        if self.configuration != Configuration::OControl && self.configuration.aware() {
            properties.push(Property::<Self>::sometimes("outcome failed", |_, s| {
                s.ledger.outcome(Outcome::Failed)
            }));
        }
        if self.part.loss() {
            properties.push(Property::<Self>::sometimes("loss used", |_, s| s.lost));
        }
        if self.part.reorder() {
            properties.push(Property::<Self>::sometimes("reordering used", |_, s| {
                s.reordered
            }));
        }
        if self.configuration.aware() {
            properties.push(Property::<Self>::sometimes(
                "new attempt after unknown",
                |_, s| s.retried_unknown,
            ));
            if self.part != Part::T {
                properties.push(Property::<Self>::sometimes(
                    "OPS-012: later failed, earlier unanswered",
                    |_, s| s.failed_while_earlier_unanswered,
                ));
            }
        }
        properties
    }

    fn within_boundary(&self, state: &World) -> bool {
        self.guard.as_ref().is_none_or(|guard| guard.admit(state))
    }
}
