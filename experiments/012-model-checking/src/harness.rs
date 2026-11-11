//! Shared explicit network, fault budgets, and checked exhaustive runs.
use stateright::{Checker, Expectation, HasDiscoveries, Model, Path, Property};
use std::collections::{BTreeSet, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// All tests and all public check helpers serialize through this lock.
static CHECK_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(usize)]
pub enum Fault {
    Reorder,
    Loss,
    Duplication,
    Cut,
    Heal,
    Crash,
    Recovery,
    LostMachine,
    Timer,
}
impl Fault {
    pub const ALL: [Self; 9] = [
        Self::Reorder,
        Self::Loss,
        Self::Duplication,
        Self::Cut,
        Self::Heal,
        Self::Crash,
        Self::Recovery,
        Self::LostMachine,
        Self::Timer,
    ];
    pub fn witness(self) -> &'static str {
        match self {
            Self::Reorder => "harness: reordered",
            Self::Loss => "harness: lost",
            Self::Duplication => "harness: duplicated",
            Self::Cut => "harness: cut",
            Self::Heal => "harness: healed",
            Self::Crash => "harness: crashed",
            Self::Recovery => "harness: recovered",
            Self::LostMachine => "harness: lost for good",
            Self::Timer => "harness: timer",
        }
    }
}

/// A model's single configuration includes its node names, all budgets, and all bounds.
/// A zero budget needs a documented reason in `disabled_reasons`.
#[derive(Clone, Debug)]
pub struct Config {
    pub name: &'static str,
    pub nodes: Vec<&'static str>,
    pub budgets: [u8; 9],
    pub disabled_reasons: [Option<&'static str>; 9],
    pub messages_per_link: usize,
    pub unique_state_ceiling: usize,
    pub timeout: Duration,
    pub bounds: &'static str,
}
impl Config {
    pub fn validate(&self) {
        assert!(!self.name.is_empty() && self.nodes.len() >= 2);
        assert!(!self.bounds.is_empty() && self.messages_per_link > 0);
        assert!((1..=1_000_000).contains(&self.unique_state_ceiling));
        assert!(!self.timeout.is_zero());
        for fault in Fault::ALL {
            if self.budgets[fault as usize] == 0 {
                assert!(
                    self.disabled_reasons[fault as usize].is_some_and(|s| !s.is_empty()),
                    "zero budget needs a reason: {fault:?}"
                );
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Status {
    Running,
    Crashed,
    Lost,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Node<N> {
    pub data: N,
    pub status: Status,
}

/// The ledger is outside all nodes. Fault transitions never mutate it.
/// Each directed link is a sequence: identical messages occupy distinct positions.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct State<N, M, L> {
    pub nodes: Vec<Node<N>>,
    pub links: Vec<Vec<M>>,
    pub cut: BTreeSet<(usize, usize)>,
    pub used: [u8; 9],
    pub ledger: L,
}
pub type System<P> =
    State<<P as Protocol>::Node, <P as Protocol>::Message, <P as Protocol>::Ledger>;
impl<N, M, L> State<N, M, L> {
    pub fn link(&self, src: usize, dst: usize) -> usize {
        src * self.nodes.len() + dst
    }
    pub fn send(&mut self, config: &Config, src: usize, dst: usize, message: M) {
        let link = self.link(src, dst);
        assert!(
            self.links[link].len() < config.messages_per_link,
            "model exceeded its stated queue bound; no transition may be silently pruned"
        );
        self.links[link].push(message);
    }
}

/// Protocols are pure reference models. Crash/recovery receive only the affected node,
/// so they cannot erase external history, queues, or global fault counts.
/// Saved and volatile storage are defined by each protocol's node type.
pub trait Protocol: Clone + Send + Sync + 'static {
    type Node: Clone + Debug + Eq + Hash + Send + Sync;
    type Message: Clone + Debug + Eq + Hash + Send + Sync;
    type Ledger: Clone + Debug + Eq + Hash + Send + Sync;
    type Local: Clone + Debug + Eq + Hash + Send + Sync;
    fn initial(&self, config: &Config) -> System<Self>;
    fn receive(
        &self,
        config: &Config,
        s: &mut System<Self>,
        src: usize,
        dst: usize,
        msg: Self::Message,
    );
    fn timer_enabled(&self, s: &System<Self>, node: usize) -> bool;
    fn timer(&self, config: &Config, s: &mut System<Self>, node: usize);
    fn crash(&self, node: &mut Self::Node);
    fn recover(&self, node: &mut Self::Node);
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Self::Local>);
    fn local_step(&self, config: &Config, s: &mut System<Self>, node: usize, action: Self::Local);
    fn properties(&self) -> Vec<NamedProperty<Self>>;
}

pub struct NamedProperty<P: Protocol> {
    pub property: Property<Harness<P>>,
    /// Promise IDs for later models, or just "harness" for MC1.
    pub promises: &'static [&'static str],
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Action<A> {
    Deliver {
        src: usize,
        dst: usize,
        index: usize,
    },
    Drop {
        src: usize,
        dst: usize,
        index: usize,
    },
    Duplicate {
        src: usize,
        dst: usize,
        index: usize,
    },
    Cut(usize, usize),
    Heal(usize, usize),
    Crash(usize),
    Recover(usize),
    Lose(usize),
    Timer(usize),
    Local(usize, A),
}

struct Guard<S> {
    started: Instant,
    seen: Mutex<HashSet<S>>,
    truncated: Mutex<Option<&'static str>>,
    first_found: Mutex<bool>,
}

#[derive(Clone)]
pub struct Harness<P: Protocol> {
    pub config: Config,
    pub protocol: P,
    guard: Option<Arc<Guard<System<P>>>>,
}
impl<P: Protocol> Harness<P> {
    pub fn new(config: Config, protocol: P) -> Self {
        config.validate();
        let model = Self {
            config,
            protocol,
            guard: None,
        };
        let props = model.properties();
        let mut names = BTreeSet::new();
        for p in &props {
            assert!(
                !p.name.is_empty() && names.insert(p.name),
                "duplicate/empty property name"
            );
            assert!(
                p.expectation != Expectation::Eventually,
                "this harness checks safety and reachability"
            );
        }
        for p in model.protocol.properties() {
            assert!(!p.promises.is_empty() && p.promises.iter().all(|id| !id.is_empty()));
        }
        assert!(props.iter().any(|p| p.expectation == Expectation::Always));
        assert!(
            model
                .protocol
                .properties()
                .iter()
                .any(|p| p.property.expectation == Expectation::Sometimes),
            "protocol must witness the outcomes its safety properties concern"
        );
        model
    }
    fn available(&self, s: &System<P>, fault: Fault) -> bool {
        s.used[fault as usize] < self.config.budgets[fault as usize]
    }
    pub fn describe(&self, action: &Action<P::Local>) -> String {
        let name = |i: usize| self.config.nodes[i];
        match action {
            Action::Deliver { src, dst, index } => format!(
                "{} {} -> {} [position {index}]",
                if *index == 0 {
                    "delivered"
                } else {
                    "delivered out of order"
                },
                name(*src),
                name(*dst)
            ),
            Action::Drop { src, dst, index } => {
                format!("lost {} -> {} [position {index}]", name(*src), name(*dst))
            }
            Action::Duplicate { src, dst, index } => format!(
                "duplicated {} -> {} [position {index}]",
                name(*src),
                name(*dst)
            ),
            Action::Cut(a, b) => format!("cut {} <-> {}", name(*a), name(*b)),
            Action::Heal(a, b) => format!("healed {} <-> {}", name(*a), name(*b)),
            Action::Crash(n) => format!("crashed {}", name(*n)),
            Action::Recover(n) => format!("recovered {}", name(*n)),
            Action::Lose(n) => format!("lost for good {}", name(*n)),
            Action::Timer(n) => format!("timer {}", name(*n)),
            Action::Local(n, a) => format!("{} {a:?}", name(*n)),
        }
    }
}
fn pair(a: usize, b: usize) -> (usize, usize) {
    (a.min(b), a.max(b))
}

impl<P: Protocol> Model for Harness<P> {
    type State = System<P>;
    type Action = Action<P::Local>;
    fn init_states(&self) -> Vec<Self::State> {
        let s = self.protocol.initial(&self.config);
        assert_eq!(s.nodes.len(), self.config.nodes.len());
        assert_eq!(s.links.len(), s.nodes.len() * s.nodes.len());
        assert!(
            s.links
                .iter()
                .all(|q| q.len() <= self.config.messages_per_link)
        );
        vec![s]
    }
    fn actions(&self, s: &Self::State, out: &mut Vec<Self::Action>) {
        for src in 0..s.nodes.len() {
            for dst in 0..s.nodes.len() {
                for index in 0..s.links[s.link(src, dst)].len() {
                    if s.nodes[dst].status == Status::Running
                        && !s.cut.contains(&pair(src, dst))
                        && (index == 0 || self.available(s, Fault::Reorder))
                    {
                        out.push(Action::Deliver { src, dst, index });
                    }
                    if self.available(s, Fault::Loss) {
                        out.push(Action::Drop { src, dst, index });
                    }
                    if self.available(s, Fault::Duplication) {
                        out.push(Action::Duplicate { src, dst, index });
                    }
                }
                if src < dst {
                    if s.cut.contains(&(src, dst)) {
                        if self.available(s, Fault::Heal) {
                            out.push(Action::Heal(src, dst));
                        }
                    } else if self.available(s, Fault::Cut) {
                        out.push(Action::Cut(src, dst));
                    }
                }
            }
            match s.nodes[src].status {
                Status::Running => {
                    if self.available(s, Fault::Crash) {
                        out.push(Action::Crash(src));
                    }
                    if self.available(s, Fault::Timer) && self.protocol.timer_enabled(s, src) {
                        out.push(Action::Timer(src));
                    }
                    let mut local = Vec::new();
                    self.protocol.local_actions(s, src, &mut local);
                    out.extend(local.into_iter().map(|a| Action::Local(src, a)));
                }
                Status::Crashed if self.available(s, Fault::Recovery) => {
                    out.push(Action::Recover(src))
                }
                _ => {}
            }
            if s.nodes[src].status != Status::Lost && self.available(s, Fault::LostMachine) {
                out.push(Action::Lose(src));
            }
        }
    }
    fn next_state(&self, s: &Self::State, a: Self::Action) -> Option<Self::State> {
        let mut next = s.clone();
        let fault = match a {
            Action::Deliver { src, dst, index } => {
                let link = next.link(src, dst);
                let msg = next.links[link].remove(index);
                self.protocol
                    .receive(&self.config, &mut next, src, dst, msg);
                (index > 0).then_some(Fault::Reorder)
            }
            Action::Drop { src, dst, index } => {
                let link = next.link(src, dst);
                next.links[link].remove(index);
                Some(Fault::Loss)
            }
            Action::Duplicate { src, dst, index } => {
                let msg = next.links[next.link(src, dst)][index].clone();
                next.send(&self.config, src, dst, msg);
                Some(Fault::Duplication)
            }
            Action::Cut(a, b) => {
                next.cut.insert(pair(a, b));
                Some(Fault::Cut)
            }
            Action::Heal(a, b) => {
                next.cut.remove(&pair(a, b));
                Some(Fault::Heal)
            }
            Action::Crash(n) => {
                next.nodes[n].status = Status::Crashed;
                self.protocol.crash(&mut next.nodes[n].data);
                Some(Fault::Crash)
            }
            Action::Recover(n) => {
                self.protocol.recover(&mut next.nodes[n].data);
                next.nodes[n].status = Status::Running;
                Some(Fault::Recovery)
            }
            Action::Lose(n) => {
                self.protocol.crash(&mut next.nodes[n].data);
                next.nodes[n].status = Status::Lost;
                Some(Fault::LostMachine)
            }
            Action::Timer(n) => {
                self.protocol.timer(&self.config, &mut next, n);
                Some(Fault::Timer)
            }
            Action::Local(n, a) => {
                self.protocol.local_step(&self.config, &mut next, n, a);
                None
            }
        };
        if let Some(fault) = fault {
            next.used[fault as usize] += 1;
        }
        Some(next)
    }
    fn properties(&self) -> Vec<Property<Self>> {
        let mut props: Vec<_> = self
            .protocol
            .properties()
            .into_iter()
            .map(|p| p.property)
            .collect();
        // Fixed predicates remain function pointers; no environment or iteration order leaks in.
        let witnesses: [fn(&Self, &Self::State) -> bool; 9] = [
            |_, s| s.used[0] > 0,
            |_, s| s.used[1] > 0,
            |_, s| s.used[2] > 0,
            |_, s| s.used[3] > 0,
            |_, s| s.used[4] > 0,
            |_, s| s.used[5] > 0,
            |_, s| s.used[6] > 0,
            |_, s| s.used[7] > 0,
            |_, s| s.used[8] > 0,
        ];
        for fault in Fault::ALL {
            if self.config.budgets[fault as usize] > 0 {
                props.push(Property::sometimes(
                    fault.witness(),
                    witnesses[fault as usize],
                ));
            }
        }
        // BFS otherwise stops expanding a state where all properties have discoveries.
        // This permanently true invariant forces even broken variants to be fully explored.
        props.push(Property::always(
            "harness: exhaustive exploration",
            |_, _| true,
        ));
        props
    }
    fn within_boundary(&self, s: &Self::State) -> bool {
        let Some(guard) = &self.guard else {
            return true;
        };
        if *guard.first_found.lock().unwrap() {
            return false;
        }
        let mut truncated = guard.truncated.lock().unwrap();
        if truncated.is_some() {
            return false;
        }
        if guard.started.elapsed() >= self.config.timeout {
            *truncated = Some("timeout");
            return false;
        }
        let mut seen = guard.seen.lock().unwrap();
        if !seen.contains(s) {
            if seen.len() == self.config.unique_state_ceiling {
                *truncated = Some("unique state ceiling");
                return false;
            }
            seen.insert(s.clone());
        }
        true
    }
}

#[derive(Debug)]
pub struct Report {
    pub unique_states: usize,
    /// Stateright depth includes the initial state (depth 1).
    pub depth: usize,
    pub seconds: f64,
    pub threads: usize,
    pub broken: Vec<&'static str>,
    pub missing_witnesses: Vec<&'static str>,
}
impl Display for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} unique states, depth {}, {:.3} s, {} threads; broken {:?}; missing {:?}",
            self.unique_states,
            self.depth,
            self.seconds,
            self.threads,
            self.broken,
            self.missing_witnesses
        )
    }
}
pub fn threads_from(value: Option<&str>) -> usize {
    value
        .map(|s| {
            s.parse::<usize>()
                .expect("CONDUIT_MC_THREADS must be a positive integer")
        })
        .map(|n| {
            assert!(n > 0);
            n.min(2)
        })
        .unwrap_or(2)
}
pub fn threads() -> usize {
    threads_from(std::env::var("CONDUIT_MC_THREADS").ok().as_deref())
}

/// Exhaustively explores *all* variants, including deliberately broken ones.
/// The helper rejects truncation before interpreting discoveries.
pub fn check_all<P: Protocol>(mut model: Harness<P>) -> Report {
    let _lock = CHECK_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    model.config.validate();
    let guard = Arc::new(Guard {
        started: Instant::now(),
        seen: Mutex::new(HashSet::new()),
        truncated: Mutex::new(None),
        first_found: Mutex::new(false),
    });
    model.guard = Some(guard.clone());
    let threads = threads();
    let checker = model
        .checker()
        .threads(threads)
        .finish_when(HasDiscoveries::AnyOf(BTreeSet::new()))
        .spawn_bfs()
        .join();
    let seconds = guard.started.elapsed().as_secs_f64();
    let truncated = *guard.truncated.lock().unwrap();
    assert!(truncated.is_none(), "run truncated: {truncated:?}");
    assert!(
        seconds < checker.model().config.timeout.as_secs_f64(),
        "run exceeded timeout"
    );
    let discoveries = checker.discoveries();
    let mut report = Report {
        unique_states: checker.unique_state_count(),
        depth: checker.max_depth(),
        seconds,
        threads,
        broken: Vec::new(),
        missing_witnesses: Vec::new(),
    };
    for p in checker.model().properties() {
        match p.expectation {
            Expectation::Always if discoveries.contains_key(p.name) => report.broken.push(p.name),
            Expectation::Sometimes if !discoveries.contains_key(p.name) => {
                report.missing_witnesses.push(p.name)
            }
            _ => {}
        }
    }
    assert!(report.unique_states <= checker.model().config.unique_state_ceiling);
    assert_eq!(report.unique_states, guard.seen.lock().unwrap().len());
    report
}
/// One call checks safety *and* every enabled fault and outcome witness.
pub fn assert_holds<P: Protocol>(model: Harness<P>) -> Report {
    let report = check_all(model);
    assert!(
        report.broken.is_empty(),
        "unexpected safety failures: {:?}",
        report.broken
    );
    assert!(
        report.missing_witnesses.is_empty(),
        "unreached witnesses: {:?}",
        report.missing_witnesses
    );
    report
}

/// A separate one-thread BFS, stopping on the named discovery, never on a witness.
/// Stateright checks finish_when only after a batch of 1500 states. A visitor saves
/// the FIRST counterexample before later states in that batch can replace its trace.
pub fn assert_refuted<P: Protocol>(
    mut model: Harness<P>,
    name: &'static str,
) -> Vec<Action<P::Local>> {
    let _lock = CHECK_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    model.config.validate();
    assert_eq!(model.property(name).expectation, Expectation::Always);
    let condition = model.property(name).condition;
    let pure = model.clone();
    let guard = Arc::new(Guard {
        started: Instant::now(),
        seen: Mutex::new(HashSet::new()),
        truncated: Mutex::new(None),
        first_found: Mutex::new(false),
    });
    model.guard = Some(guard.clone());
    let first = Arc::new(Mutex::new(None));
    let captured = first.clone();
    let control = guard.clone();
    let checker = model
        .checker()
        .threads(1)
        .finish_when(HasDiscoveries::AnyOf(BTreeSet::from([name])))
        .visitor(move |path: Path<System<P>, Action<P::Local>>| {
            let mut first = captured.lock().unwrap();
            if first.is_none() && !condition(&pure, path.last_state()) {
                *first = Some(path.into_actions());
                *control.first_found.lock().unwrap() = true;
            }
        })
        .spawn_bfs()
        .join();
    assert!(
        guard.truncated.lock().unwrap().is_none(),
        "refutation run truncated"
    );
    assert!(guard.started.elapsed() < checker.model().config.timeout);
    assert!(
        checker.discoveries().contains_key(name),
        "expected counterexample: {name}"
    );
    let trace = first
        .lock()
        .unwrap()
        .take()
        .expect("first counterexample recorded");
    print_trace(checker.model(), &trace);
    trace
}
pub fn print_trace<P: Protocol>(model: &Harness<P>, trace: &[Action<P::Local>]) {
    for (i, action) in trace.iter().enumerate() {
        println!("{}. {}", i + 1, model.describe(action));
    }
}
