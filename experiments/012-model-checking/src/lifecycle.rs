//! MC6: bounded lifecycle reference model. All predicates use the external send ledger.
use crate::harness::*;
use stateright::Property;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

const R: usize = 0;
const E: usize = 1;
const H: usize = 2;
const I: usize = 0;
const T: usize = 1;
const C: usize = 2;
const C2: usize = 3;
const D: usize = 4;
const RT: usize = 5;
const P: usize = 6;
const E1: usize = 7;
const E2: usize = 8;
const NEW: usize = 9;
const J: usize = 10;
const K: usize = 11;
const N: usize = 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Part {
    Setup,
    Removal,
    Pool,
}
impl Part {
    pub const ALL: [Self; 3] = [Self::Setup, Self::Removal, Self::Pool];
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Faults {
    Base,
    LossR,
    ReorderR,
    LossH,
    ReorderH,
    Crash,
    Timer,
    Silence,
    ShutdownTimer,
}
impl Faults {
    pub const ALL: [Self; 8] = [
        Self::Base,
        Self::LossR,
        Self::ReorderR,
        Self::LossH,
        Self::ReorderH,
        Self::Crash,
        Self::Timer,
        Self::Silence,
    ];
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Correct,
    Regress,
    EarlyPart,
    RepeatHook,
    WrongKey,
    FalseSuccess,
    Rollback,
    KeepConduit,
    AutoReplace,
    IgnoreTombstone,
    EarlyRemove,
    EarlyPool,
    OtherData,
    Revive,
    Unfenced,
    HookPending,
    PreserveReferences,
    TimeConfirms,
    CloseStops,
    AdoptTemplate,
    StopBeforeHook,
    KeepConfirmed,
    DelayRetirement,
    UnsolicitedRetirement,
    WrongHookInstance,
    CurrentTemplateHook,
    UnknownAtDispatch,
    KeepLatestUnknown,
}
impl Variant {
    pub const BROKEN: [Self; 27] = [
        Self::Regress,
        Self::EarlyPart,
        Self::RepeatHook,
        Self::WrongKey,
        Self::FalseSuccess,
        Self::Rollback,
        Self::KeepConduit,
        Self::AutoReplace,
        Self::IgnoreTombstone,
        Self::EarlyRemove,
        Self::EarlyPool,
        Self::OtherData,
        Self::Revive,
        Self::Unfenced,
        Self::HookPending,
        Self::PreserveReferences,
        Self::TimeConfirms,
        Self::CloseStops,
        Self::AdoptTemplate,
        Self::StopBeforeHook,
        Self::KeepConfirmed,
        Self::DelayRetirement,
        Self::UnsolicitedRetirement,
        Self::WrongHookInstance,
        Self::CurrentTemplateHook,
        Self::UnknownAtDispatch,
        Self::KeepLatestUnknown,
    ];
    pub fn run(self) -> (Part, Faults) {
        match self {
            Self::EarlyPool => (Part::Pool, Faults::Base),
            Self::TimeConfirms => (Part::Removal, Faults::ShutdownTimer),
            Self::Unfenced => (Part::Setup, Faults::Timer),
            Self::AdoptTemplate | Self::UnsolicitedRetirement | Self::WrongHookInstance => {
                (Part::Setup, Faults::Base)
            }
            Self::Regress
            | Self::EarlyPart
            | Self::RepeatHook
            | Self::WrongKey
            | Self::FalseSuccess
            | Self::Rollback => (Part::Setup, Faults::Base),
            _ => (Part::Removal, Faults::Base),
        }
    }
    pub fn target(self) -> usize {
        match self {
            Self::Regress | Self::DelayRetirement | Self::UnsolicitedRetirement => 0,
            Self::EarlyPart => 1,
            Self::RepeatHook => 2,
            Self::WrongKey | Self::WrongHookInstance | Self::CurrentTemplateHook => 3,
            Self::FalseSuccess | Self::StopBeforeHook => 4,
            Self::Rollback | Self::CloseStops => 5,
            Self::KeepConduit | Self::PreserveReferences => 6,
            Self::AutoReplace => 7,
            Self::IgnoreTombstone => 8,
            Self::EarlyRemove
            | Self::KeepConfirmed
            | Self::UnknownAtDispatch
            | Self::KeepLatestUnknown => 9,
            Self::EarlyPool => 10,
            Self::OtherData | Self::AdoptTemplate => 11,
            Self::Revive => 12,
            Self::Unfenced => 13,
            Self::HookPending | Self::TimeConfirms => 9,
            Self::Correct => unreachable!(),
        }
    }
}
const SAFETY: [&str; 17] = [
    "MC6 L: lifecycle and observation",
    "MC6 S: setup order and authority",
    "MC6 H: hook dispatch eligibility",
    "MC6 K: hook key and caller",
    "MC6 O: aggregate outcome",
    "MC6 B: no rollback",
    "MC6 C: Conduit closure and retired access",
    "MC6 R: replacement is explicit",
    "MC6 T: tombstone precedence",
    "MC6 D: shutdown and removal",
    "MC6 P: Pool dependencies",
    "MC6 F: unrelated state retained",
    "MC6 Q: reconciliation cannot revive",
    "MC6 A: fenced attempts",
    "MC6 literal: failed hook keeps pending",
    "MC6 literal: removal changes no referrer",
    "MC6 A2: hook effect at most once",
];
const IDS: [&[&str]; 17] = [
    &[
        "LIF-001", "LIF-003", "LIF-004", "LIF-005", "LIF-006", "LIF-010", "LIF-015", "LIF-023",
    ],
    &["TPL-012", "CON-004", "ACC-005"],
    &["OPS-021", "TPL-017", "TPL-018", "LIF-026"],
    &["OPS-028", "ACC-018"],
    &["OPS-022", "TPL-009"],
    &["LIF-027", "CON-014"],
    &["CON-008", "CON-023", "CON-031", "ACC-039"],
    &["LIF-008", "LIF-009", "LIF-029", "POOL-003", "POOL-017"],
    &["LIF-040", "LIF-041"],
    &[
        "LIF-014", "LIF-017", "LIF-018", "LIF-024", "LIF-025", "LIF-035", "LIF-036",
    ],
    &["LIF-020", "LIF-021", "POOL-007", "POOL-013"],
    &["LIF-016", "LIF-019", "INS-020"],
    &["LIF-028"],
    &["OPS-021", "OPS-028"],
    &["LIF-024"],
    &["LIF-019", "CON-008"],
    &["OPS-021", "OPS-028"],
];
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Outcome {
    #[default]
    None,
    Progress,
    Succeeded,
    Failed,
    Unknown,
    Conflict,
    Invalid,
    Refused,
    Unavailable,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Life {
    #[default]
    Registered,
    Ready,
    Retired,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Shutdown {
    #[default]
    Pending,
    Unknown,
    Confirmed,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Hook {
    outcome: Outcome,
    attempt: u8,
    caller: u8,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Instance {
    exists: bool,
    life: Life,
    shutdown: Shutdown,
    setup: bool,
    hooks: [Hook; 2],
    latest: Option<u16>,
    confirmed: bool,
    definition: u8,
    // The creation-time resolution is immutable, including across template changes.
    resolved_definition: u8,
}
impl Default for Instance {
    fn default() -> Self {
        Self {
            exists: true,
            life: Life::Registered,
            shutdown: Shutdown::Pending,
            setup: false,
            hooks: [Hook::default(); 2],
            latest: None,
            confirmed: false,
            definition: 1,
            resolved_definition: 1,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Op {
    Setup(usize),
    Retire(usize),
    Open(usize),
    Close(usize),
    Unregister(usize),
    ReplaceC(bool, bool),
    NewResource(usize),
    Lift,
    ReplaceTemplate,
    RemoveTemplate,
    ReconcileHook(usize, bool),
    ReconcileSetup(usize),
    ReconcileRetire(usize, bool, bool),
    ReconcileOpen,
    Observe(usize),
    Call(bool),
    OwnRequest,
    Cascade,
    Noop,
}
impl Op {
    fn instance(self) -> usize {
        match self {
            Self::Setup(i)
            | Self::Retire(i)
            | Self::Open(i)
            | Self::Close(i)
            | Self::Unregister(i)
            | Self::Observe(i)
            | Self::ReconcileHook(i, _)
            | Self::ReconcileSetup(i)
            | Self::ReconcileRetire(i, _, _) => i,
            Self::Cascade => P,
            Self::Call(_) => T,
            _ => I,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum WorkPart {
    Open,
    Run,
    SetupHook,
    RetireHook,
    Stop,
    Entry1,
    Entry2,
    Member2,
    Member1,
    Container,
    Call,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Request {
    id: u16,
    op: Op,
    caller: u8,
    parent: Option<u16>,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Work {
    request: Request,
    next: u8,
    waiting: bool,
    parts: Vec<(WorkPart, Outcome, bool)>,
    result: Outcome,
    effect: bool,
    before: Life,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Dispatch {
    id: u16,
    instance: usize,
    part: WorkPart,
    attempt: u8,
    outcome: Outcome,
    caller: u8,
    key: (usize, u8),
    selected_definition: u8,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Request(Request),
    Dispatch(Dispatch),
    Reply(Dispatch, bool),
    Answer(u16),
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Data {
    instances: [Instance; N],
    work: BTreeMap<u16, Work>,
    stage: usize,
    ack: BTreeSet<u16>,
    next_child: u16,
    recovery: bool,
    tomb: [bool; N],
    slot: usize,
    entries: [usize; 2],
    template: u8,
    runs: [bool; N],
    backing: [bool; N],
    records: [bool; N],
    shared: bool,
    effects: [[u8; 2]; N],
    jobs: Vec<Dispatch>,
    silences: u8,
    observations: Vec<Request>,
    stops: [u8; N],
    // A volatile cache is intentionally dispensable: it never decides dispatch.
    cache: bool,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    State {
        i: usize,
        before: Life,
        after: Life,
        why: Op,
        exists: bool,
        shutdown: Shutdown,
    },
    Decision {
        op: Op,
        exists: bool,
        life: Life,
        consumer: Life,
        target: Life,
        open: bool,
        tomb: bool,
        members: bool,
        answer: Outcome,
        dispatched: bool,
    },
    ActorRequest(Request),
    Begin(Request, u8),
    Observation {
        i: usize,
        unavailable: bool,
        reported: Option<Life>,
        before: Life,
        after: Life,
    },
    EffectApplied(Dispatch),
    RetirementDispatch {
        request: Request,
        setup: bool,
        hook_prior: Outcome,
        after: Life,
    },
    Existence {
        i: usize,
        life: Life,
        shutdown: Shutdown,
        exists: bool,
    },
    Part {
        dispatch: Dispatch,
        previous: Vec<(WorkPart, Outcome, bool)>,
        prior: Outcome,
        setup: bool,
        retired: bool,
    },
    Applied {
        dispatch: Dispatch,
        count: u8,
    },
    End {
        request: Request,
        parts: Vec<(WorkPart, Outcome, bool)>,
        outcome: Outcome,
        before: Life,
        after: Life,
    },
    Retire {
        i: usize,
        setup: bool,
        before: Life,
        shutdown: Shutdown,
    },
    Removed {
        i: usize,
        explicit: bool,
        confirmed: bool,
        stop: bool,
        closed: Vec<usize>,
    },
    Closed {
        run_before: bool,
        run_after: bool,
        i: usize,
        why: Op,
        consumer: Life,
        after: Life,
    },
    Tombstone {
        i: usize,
        kept: bool,
        engine_made: bool,
    },
    Replacement {
        conduit: bool,
        old: usize,
        new: usize,
        explicit: bool,
        life: Life,
        definition: u8,
        owner_definition: u8,
        template: u8,
        tomb: bool,
    },
    Reconciled {
        op: Op,
        invocation: Option<u16>,
        unknown: bool,
        occurred: bool,
        fenced: bool,
        latest: bool,
        before: Life,
        after: Life,
        shutdown_before: Shutdown,
        shutdown_after: Shutdown,
        retired_consumer: bool,
    },
    Retained {
        definition: u8,
        before_definition: u8,
        template: u8,
        records: bool,
        data: bool,
        unrelated: bool,
    },
    PoolPart {
        part: WorkPart,
        predecessor: bool,
        ready: bool,
        dispatched: bool,
        confirmed: [bool; 2],
        owner_retired: bool,
    },
    Onward {
        retirement: bool,
        retired: bool,
        open: bool,
        dispatched: bool,
    },
    Ends {
        i: usize,
        target_only: bool,
        remaining: Vec<usize>,
    },
    Silence(Dispatch),
    Recovery,
    Timer,
    Tick {
        before: Shutdown,
        after: Shutdown,
    },
    Quiescent,
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Ledger {
    events: Vec<Event>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Local {
    Send,
    Advance(u16),
    Observe(usize, bool),
    Complete(usize),
    Silence(usize),
    Recover,
    ReconcileHook(usize, bool),
    Quiescent,
}
#[derive(Clone)]
pub struct Lifecycle {
    pub part: Part,
    pub faults: Faults,
    pub variant: Variant,
}

pub fn model(part: Part, faults: Faults, variant: Variant) -> Harness<Lifecycle> {
    let mut budgets = [0; 9];
    match faults {
        Faults::LossR | Faults::LossH => budgets[Fault::Loss as usize] = 1,
        Faults::ReorderR | Faults::ReorderH => budgets[Fault::Reorder as usize] = 1,
        Faults::Crash => {
            budgets[Fault::Crash as usize] = 1;
            budgets[Fault::Recovery as usize] = 1;
        }
        Faults::Timer | Faults::Silence | Faults::ShutdownTimer => {
            budgets[Fault::Timer as usize] = 1
        }
        Faults::Base => {}
    }
    Harness::new(
        Config {
            name: "MC6 split lifecycle faults",
            nodes: vec!["R", "engine", "H"],
            budgets,
            disabled_reasons: std::array::from_fn(|n| {
                (budgets[n] == 0).then_some(match Fault::ALL[n] {
            Fault::Duplication => "copy faults belong to MC2; the dispatch transport has no copied sends in this model",
            Fault::Cut | Fault::Heal => "partition and machine placement belong to MC4; MC6 explores individual losses",
            Fault::LostMachine => "machine loss belongs to MC4; all three lifecycle nodes survive",
            _ => "split by fault: enabled in the named MC6 configuration; Base is the no-fault control",
        })
            }),
            messages_per_link: 6,
            unique_state_ceiling: 190_000,
            timeout: Duration::from_secs(10),
            bounds: "three nodes; twelve instance IDs; two entry slots; at most two simultaneous root requests; 64 root requests, 32 children, one silence, one of each enabled fault; six messages per link",
        },
        Lifecycle {
            part,
            faults,
            variant,
        },
    )
}
impl Lifecycle {
    fn script(&self) -> Vec<Vec<Op>> {
        match self.part {
            Part::Setup => vec![
                vec![Op::Setup(I), Op::Setup(J)],
                vec![Op::Setup(I)],
                vec![Op::Setup(I)],
                vec![Op::ReconcileHook(I, false)],
                vec![Op::ReconcileSetup(I)],
                vec![Op::Setup(I)],
                vec![Op::Setup(J)],
                vec![Op::ReplaceTemplate],
                vec![Op::Close(C)],
                vec![Op::Lift],
                vec![Op::ReplaceC(true, true)],
                vec![Op::Setup(I)],
                vec![Op::RemoveTemplate],
                vec![Op::Observe(I)],
            ],
            Part::Removal => vec![
                vec![Op::Open(C)],
                vec![Op::Setup(I)],
                vec![Op::ReconcileHook(I, false)],
                vec![Op::Setup(I)],
                vec![Op::Setup(J)],
                vec![Op::ReconcileSetup(J)],
                vec![Op::ReplaceC(true, true), Op::Unregister(I)],
                vec![Op::Close(C)],
                vec![Op::Setup(I)],
                vec![Op::ReplaceC(false, true)],
                vec![Op::ReplaceC(true, false)],
                vec![Op::ReplaceC(true, true)],
                vec![Op::Lift],
                vec![Op::ReplaceTemplate],
                vec![Op::ReplaceC(true, true)],
                vec![Op::ReplaceC(true, true)],
                vec![if matches!(self.faults, Faults::Crash | Faults::Timer) {
                    Op::Open(C2)
                } else {
                    Op::Noop
                }],
                vec![Op::Retire(I)],
                vec![Op::Retire(I)],
                vec![Op::Retire(I)],
                vec![Op::ReconcileHook(I, true)],
                vec![Op::OwnRequest],
                vec![Op::Open(C2)],
                vec![Op::Setup(I)],
                vec![Op::ReconcileSetup(I)],
                vec![Op::ReconcileOpen],
                vec![Op::Call(false)],
                vec![Op::Unregister(I)],
                vec![Op::Setup(I)],
                vec![Op::Setup(T)],
                vec![Op::Call(false), Op::Retire(T)],
                vec![Op::Call(false)],
                vec![Op::Retire(J)],
                vec![Op::Retire(J)],
                vec![Op::ReconcileRetire(J, false, false)],
                vec![Op::Retire(J)],
                vec![Op::ReconcileRetire(J, true, false)],
                vec![Op::ReconcileRetire(J, true, true)],
                vec![Op::Retire(K)],
                vec![Op::RemoveTemplate],
                vec![Op::Observe(T)],
            ],
            Part::Pool => vec![
                vec![Op::Setup(E2), Op::Setup(J)],
                vec![Op::Setup(P)],
                vec![Op::Setup(P)],
                vec![Op::NewResource(0)],
                vec![Op::Retire(P)],
                vec![Op::Cascade],
                vec![Op::ReconcileRetire(E2, true, true)],
                vec![Op::NewResource(1)],
                vec![Op::Cascade],
                vec![Op::ReconcileRetire(E1, true, true)],
                vec![Op::Cascade],
                vec![Op::NewResource(0)],
                vec![Op::Retire(P)],
            ],
        }
    }
    fn log(s: &mut System<Self>, e: Event) {
        s.ledger.events.push(e);
    }
    fn consumer(i: usize) -> Option<usize> {
        match i {
            C | C2 | D => Some(I),
            _ => None,
        }
    }
    fn target(i: usize) -> usize {
        if i == D { K } else { T }
    }
    fn conduits() -> [usize; 4] {
        [C, C2, D, RT]
    }
    fn state(&self, s: &mut System<Self>, i: usize, before: Life, why: Op) {
        let a = s.nodes[E].data.instances[i];
        Self::log(
            s,
            Event::State {
                i,
                before,
                after: a.life,
                why,
                exists: a.exists,
                shutdown: a.shutdown,
            },
        );
    }
    fn existence(s: &mut System<Self>, i: usize) {
        let a = s.nodes[E].data.instances[i];
        Self::log(
            s,
            Event::Existence {
                i,
                life: a.life,
                shutdown: a.shutdown,
                exists: a.exists,
            },
        );
    }
    fn close(&self, s: &mut System<Self>, i: usize, why: Op) {
        let a = s.nodes[E].data.instances[i];
        if !a.exists {
            return;
        }
        let consumer = Self::consumer(i).map_or(Life::Ready, |n| s.nodes[E].data.instances[n].life);
        let run_before = s.nodes[H].data.runs[I];
        if self.variant == Variant::CloseStops
            && matches!(why, Op::Close(_))
            && Self::consumer(i) == Some(I)
        {
            s.nodes[H].data.runs[I] = false;
        }
        let run_after = s.nodes[H].data.runs[I];
        let d = &mut s.nodes[E].data;
        d.instances[i].life = Life::Retired;
        d.instances[i].shutdown = Shutdown::Confirmed;
        d.instances[i].confirmed = true;
        d.instances[i].exists = false;
        d.tomb[i] = true;
        Self::log(
            s,
            Event::Closed {
                run_before,
                run_after,
                i,
                why,
                consumer,
                after: s.nodes[E].data.instances[I].life,
            },
        );
        Self::log(
            s,
            Event::Tombstone {
                i,
                kept: true,
                engine_made: false,
            },
        );
        self.state(s, i, a.life, why);
        Self::existence(s, i);
    }
    fn close_ends(&self, s: &mut System<Self>, i: usize, target_only: bool, why: Op) -> Vec<usize> {
        let mut closed = vec![];
        for c in Self::conduits() {
            let match_end = if target_only {
                Self::target(c) == i
            } else {
                Self::consumer(c) == Some(i)
            };
            if match_end && s.nodes[E].data.instances[c].exists {
                if self.variant == Variant::KeepConduit
                    || (self.variant == Variant::PreserveReferences && !target_only)
                {
                    continue;
                }
                self.close(s, c, why);
                closed.push(c);
            }
        }
        let remaining = Self::conduits()
            .into_iter()
            .filter(|&n| {
                s.nodes[E].data.instances[n].exists
                    && if target_only {
                        Self::target(n) == i
                    } else {
                        Self::consumer(n) == Some(i)
                    }
            })
            .collect();
        Self::log(
            s,
            Event::Ends {
                i,
                target_only,
                remaining,
            },
        );
        closed
    }
    fn remove(&self, s: &mut System<Self>, i: usize, explicit: bool, stop: bool) {
        if self.variant == Variant::KeepConfirmed && !explicit {
            return;
        }
        let before = s.nodes[E].data.instances[i];
        let expected: Vec<_> = Self::conduits()
            .into_iter()
            .filter(|&c| Self::consumer(c) == Some(i) && s.nodes[E].data.instances[c].exists)
            .collect();
        let closed = self.close_ends(s, i, false, Op::Unregister(i));
        s.nodes[E].data.instances[i].exists = false;
        if self.variant == Variant::OtherData {
            s.nodes[E].data.backing[i] = false;
        }
        Self::log(
            s,
            Event::Removed {
                i,
                explicit,
                confirmed: before.confirmed,
                stop,
                closed: closed.clone(),
            },
        );
        // Log the remaining edges as a send-time observation, independently of the removal list.
        for c in expected {
            let open = s.nodes[E].data.instances[c].exists;
            Self::log(
                s,
                Event::Onward {
                    retirement: false,
                    retired: true,
                    open,
                    dispatched: open,
                },
            );
        }
        Self::log(
            s,
            Event::Retained {
                definition: s.nodes[E].data.instances[i].definition,
                before_definition: before.resolved_definition,
                template: s.nodes[E].data.template,
                records: s.nodes[E].data.records[i],
                data: s.nodes[E].data.backing[i],
                unrelated: s.nodes[E].data.shared,
            },
        );
    }
    fn decision(&self, s: &mut System<Self>, op: Op, answer: Outcome, dispatched: bool) {
        let d = &s.nodes[E].data;
        let a = d.instances[match op {
            Op::NewResource(entry) => d.entries[entry],
            Op::ReplaceC(_, _) => d.slot,
            _ => op.instance(),
        }];
        let c = d.instances[d.slot];
        Self::log(
            s,
            Event::Decision {
                op,
                exists: a.exists,
                life: a.life,
                consumer: d.instances[if matches!(op, Op::NewResource(_)) {
                    P
                } else {
                    I
                }]
                .life,
                target: d.instances[T].life,
                open: c.exists && c.life == Life::Ready,
                tomb: d.tomb[C] || d.tomb[C2],
                members: d.entries.iter().all(|&i| d.instances[i].confirmed),
                answer,
                dispatched,
            },
        );
    }
    fn answer(&self, c: &Config, s: &mut System<Self>, r: Request) {
        if r.parent.is_none() {
            s.send(c, E, R, Message::Answer(r.id));
        }
    }
    fn start(&self, c: &Config, s: &mut System<Self>, r: Request) {
        let i = r.op.instance();
        let a = s.nodes[E].data.instances[i];
        let reject = match r.op {
            Op::Setup(_) | Op::Retire(_) | Op::Unregister(_) | Op::Open(_) | Op::Close(_)
                if !a.exists =>
            {
                Some(Outcome::Invalid)
            }
            Op::Setup(_) if a.life == Life::Retired => Some(Outcome::Conflict),
            Op::Unregister(_) if a.life != Life::Retired => Some(Outcome::Conflict),
            Op::Retire(P)
                if self.variant != Variant::EarlyPool
                    && !s.nodes[E]
                        .data
                        .entries
                        .iter()
                        .all(|&n| s.nodes[E].data.instances[n].confirmed) =>
            {
                Some(Outcome::Conflict)
            }
            Op::Open(_)
                if a.life != Life::Ready
                    && (Self::consumer(i)
                        .is_some_and(|n| s.nodes[E].data.instances[n].life == Life::Retired)
                        || s.nodes[E].data.instances[Self::target(i)].life == Life::Retired) =>
            {
                Some(Outcome::Conflict)
            }
            _ => None,
        };
        if let Some(outcome) = reject {
            self.decision(s, r.op, outcome, false);
            self.answer(c, s, r);
            return;
        }
        match r.op {
            Op::Setup(_)
            | Op::Retire(_)
            | Op::Unregister(_)
            | Op::Open(_)
            | Op::Cascade
            | Op::Call(_) => {
                if let Op::Call(retirement) = r.op {
                    let root = s.nodes[E].data.instances[RT];
                    let access = root.exists
                        && root.life == Life::Ready
                        && s.nodes[E].data.instances[T].life != Life::Retired;
                    let permitted = access && s.nodes[E].data.instances[T].life == Life::Ready;
                    self.decision(
                        s,
                        r.op,
                        if access && s.nodes[E].data.instances[T].life == Life::Registered {
                            Outcome::Unavailable
                        } else if permitted {
                            Outcome::Progress
                        } else {
                            Outcome::Refused
                        },
                        permitted,
                    );
                    Self::log(
                        s,
                        Event::Onward {
                            retirement,
                            retired: false,
                            open: root.exists,
                            dispatched: permitted,
                        },
                    );
                    if !permitted {
                        self.answer(c, s, r);
                        return;
                    }
                } else {
                    self.decision(s, r.op, Outcome::Progress, true);
                }
                if matches!(r.op, Op::Setup(_)) {
                    s.nodes[E].data.instances[i].setup = true;
                }
                if matches!(r.op, Op::Retire(_)) {
                    let d = &mut s.nodes[E].data;
                    if self.variant != Variant::DelayRetirement {
                        d.instances[i].life = Life::Retired;
                    }
                    d.instances[i].latest = Some(r.id);
                    d.instances[i].confirmed = a.confirmed || !a.setup;
                    d.instances[i].shutdown = if d.instances[i].confirmed {
                        Shutdown::Confirmed
                    } else if self.variant == Variant::UnknownAtDispatch {
                        Shutdown::Unknown
                    } else {
                        Shutdown::Pending
                    };
                    d.tomb[i] = true;
                    Self::log(
                        s,
                        Event::RetirementDispatch {
                            request: r,
                            setup: a.setup,
                            hook_prior: a.hooks[1].outcome,
                            after: s.nodes[E].data.instances[i].life,
                        },
                    );
                    Self::log(
                        s,
                        Event::Tombstone {
                            i,
                            kept: true,
                            engine_made: false,
                        },
                    );
                    Self::log(
                        s,
                        Event::Retire {
                            i,
                            setup: a.setup,
                            before: a.life,
                            shutdown: s.nodes[E].data.instances[i].shutdown,
                        },
                    );
                    self.state(s, i, a.life, r.op);
                    self.close_ends(s, i, true, r.op);
                }
                s.nodes[E].data.work.insert(
                    r.id,
                    Work {
                        request: r,
                        next: 0,
                        waiting: false,
                        parts: vec![],
                        result: Outcome::Progress,
                        effect: false,
                        before: a.life,
                    },
                );
                Self::log(s, Event::Begin(r, a.resolved_definition));
                if matches!(r.op, Op::Retire(_)) && !a.setup {
                    self.finish(c, s, r.id, None);
                }
            }
            Op::Close(n) => {
                self.close(s, n, r.op);
                self.answer(c, s, r);
            }
            Op::ReplaceC(auth, valid) => {
                let old = s.nodes[E].data.slot;
                let live = s.nodes[E].data.instances[old].exists;
                let tomb = s.nodes[E].data.tomb[C] || s.nodes[E].data.tomb[C2];
                let result = if !auth {
                    Outcome::Refused
                } else if !valid {
                    Outcome::Invalid
                } else if live || (tomb && self.variant != Variant::IgnoreTombstone) {
                    Outcome::Conflict
                } else {
                    Outcome::Succeeded
                };
                self.decision(s, r.op, result, result == Outcome::Succeeded);
                if result == Outcome::Succeeded {
                    self.replace(s, true, old, C2, true);
                }
                self.answer(c, s, r);
            }
            Op::NewResource(entry) => {
                let old = s.nodes[E].data.entries[entry];
                let owner = s.nodes[E].data.instances[P];
                let result = if owner.life == Life::Retired {
                    Outcome::Invalid
                } else if s.nodes[E].data.instances[old].life != Life::Retired {
                    Outcome::Conflict
                } else {
                    Outcome::Succeeded
                };
                self.decision(s, r.op, result, result == Outcome::Succeeded);
                if result == Outcome::Succeeded {
                    self.replace(s, false, old, NEW, true);
                    s.nodes[E].data.entries[entry] = NEW;
                }
                self.answer(c, s, r);
            }
            Op::Lift => {
                s.nodes[E].data.tomb[C] = false;
                s.nodes[E].data.tomb[C2] = false;
                self.answer(c, s, r);
            }
            Op::ReconcileHook(n, retirement) => {
                self.reconcile_hook(s, n, retirement);
                self.answer(c, s, r);
            }
            Op::ReconcileSetup(n) => {
                self.reconcile(s, r.op, n, false, true, true);
                self.answer(c, s, r);
            }
            Op::ReconcileRetire(n, latest, occurred) => {
                self.reconcile(s, r.op, n, true, latest, occurred);
                self.answer(c, s, r);
            }
            Op::ReconcileOpen => {
                self.reconcile(s, r.op, C2, false, true, true);
                self.answer(c, s, r);
            }
            Op::ReplaceTemplate | Op::RemoveTemplate => {
                s.nodes[E].data.template += 1;
                if self.variant == Variant::AdoptTemplate {
                    s.nodes[E].data.instances[I].definition = s.nodes[E].data.template;
                }
                let a = s.nodes[E].data.instances[I];
                Self::log(
                    s,
                    Event::Retained {
                        definition: a.definition,
                        before_definition: a.resolved_definition,
                        template: s.nodes[E].data.template,
                        records: true,
                        data: true,
                        unrelated: true,
                    },
                );
                if self.variant == Variant::UnsolicitedRetirement && r.op == Op::RemoveTemplate {
                    s.nodes[E].data.instances[I].life = Life::Retired;
                    self.state(s, I, a.life, r.op);
                }
                self.answer(c, s, r);
            }
            Op::Observe(_) => s.nodes[E].data.observations.push(r),
            Op::OwnRequest => {
                let open = s.nodes[E].data.instances[D].exists;
                let retired = s.nodes[E].data.instances[I].life == Life::Retired;
                Self::log(
                    s,
                    Event::Onward {
                        retirement: false,
                        retired,
                        open,
                        dispatched: open && !retired,
                    },
                );
                self.answer(c, s, r);
            }
            Op::Noop => self.answer(c, s, r),
        }
    }
    fn replace(&self, s: &mut System<Self>, conduit: bool, old: usize, new: usize, explicit: bool) {
        s.nodes[E].data.instances[new] = Instance::default();
        s.nodes[E].data.instances[new].definition = s.nodes[E].data.template;
        s.nodes[E].data.instances[new].resolved_definition = s.nodes[E].data.template;
        if conduit {
            s.nodes[E].data.slot = new;
        }
        Self::log(
            s,
            Event::Replacement {
                conduit,
                old,
                new,
                explicit,
                life: Life::Registered,
                definition: s.nodes[E].data.template,
                owner_definition: s.nodes[E].data.instances[I].definition,
                template: s.nodes[E].data.template,
                tomb: if conduit {
                    s.nodes[E].data.tomb[C]
                } else {
                    false
                },
            },
        );
    }
    fn plan(&self, s: &System<Self>, w: &Work) -> Vec<WorkPart> {
        match w.request.op {
            Op::Setup(P) => vec![
                WorkPart::Entry1,
                WorkPart::Entry2,
                WorkPart::Run,
                WorkPart::SetupHook,
            ],
            Op::Setup(I) => {
                if self.variant == Variant::EarlyPart {
                    vec![WorkPart::SetupHook, WorkPart::Open, WorkPart::Run]
                } else {
                    vec![WorkPart::Open, WorkPart::Run, WorkPart::SetupHook]
                }
            }
            Op::Setup(_) => vec![WorkPart::Run, WorkPart::SetupHook],
            Op::Retire(i) if s.nodes[E].data.instances[i].setup => {
                if self.variant == Variant::StopBeforeHook {
                    vec![WorkPart::Run, WorkPart::Stop, WorkPart::RetireHook]
                } else {
                    vec![WorkPart::Run, WorkPart::RetireHook, WorkPart::Stop]
                }
            }
            Op::Retire(_) => vec![],
            Op::Unregister(_) => vec![WorkPart::Stop],
            Op::Open(_) => vec![WorkPart::Open],
            Op::Call(_) => vec![WorkPart::Call],
            Op::Cascade => {
                if self.variant == Variant::EarlyPool {
                    vec![WorkPart::Container, WorkPart::Member2, WorkPart::Member1]
                } else {
                    vec![WorkPart::Member2, WorkPart::Member1, WorkPart::Container]
                }
            }
            _ => unreachable!(),
        }
    }
    fn result(parts: &[(WorkPart, Outcome, bool)]) -> Outcome {
        if parts.iter().all(|p| p.1 == Outcome::Succeeded) {
            Outcome::Succeeded
        } else if parts.iter().any(|p| p.1 == Outcome::Unknown || p.2) {
            Outcome::Unknown
        } else {
            Outcome::Failed
        }
    }
    fn add_part(
        &self,
        s: &mut System<Self>,
        id: u16,
        part: WorkPart,
        value: Outcome,
        effect: bool,
    ) {
        let w = s.nodes[E].data.work.get_mut(&id).unwrap();
        w.parts.push((part, value, effect));
        w.effect |= effect;
        w.next += 1;
        w.waiting = false;
        if matches!(w.request.op, Op::Setup(_)) && value != Outcome::Succeeded {
            w.next = 99;
        }
        if w.request.op == Op::Cascade && value != Outcome::Succeeded {
            w.next = 99;
        }
    }
    fn advance(&self, c: &Config, s: &mut System<Self>, id: u16) {
        let w = s.nodes[E].data.work[&id].clone();
        let i = w.request.op.instance();
        let plan = self.plan(s, &w);
        if w.next as usize >= plan.len() {
            self.finish(c, s, id, None);
            return;
        }
        let part = plan[w.next as usize];
        if matches!(w.request.op, Op::Setup(_))
            && s.nodes[E].data.instances[i].life == Life::Retired
        {
            self.add_part(s, id, part, Outcome::Conflict, false);
            return;
        }
        match part {
            WorkPart::Open => {
                let n = if matches!(w.request.op, Op::Open(_)) {
                    i
                } else {
                    s.nodes[E].data.slot
                };
                let a = s.nodes[E].data.instances[n];
                if !a.exists {
                    if self.variant == Variant::AutoReplace {
                        self.replace(s, true, n, C2, false);
                    }
                    self.add_part(s, id, part, Outcome::Conflict, false);
                } else if a.life == Life::Ready {
                    self.add_part(s, id, part, Outcome::Succeeded, false);
                } else {
                    self.dispatch_log(
                        s,
                        Dispatch {
                            id,
                            instance: n,
                            part,
                            attempt: 0,
                            outcome: Outcome::Succeeded,
                            caller: w.request.caller,
                            key: (n, 0),
                            selected_definition: a.definition,
                        },
                        &w,
                        Outcome::None,
                    );
                    // The engine's stored open effect and the invocation's confirmation are separate.
                    // An interrupted explicit open remains unconfirmed until reconciliation.
                    if matches!(w.request.op, Op::Setup(_)) {
                        let before = a.life;
                        s.nodes[E].data.instances[n].life = Life::Ready;
                        self.state(s, n, before, Op::Open(n));
                    }
                    self.add_part(s, id, part, Outcome::Succeeded, true);
                }
            }
            WorkPart::Entry1
            | WorkPart::Entry2
            | WorkPart::Member2
            | WorkPart::Member1
            | WorkPart::Container => {
                let entries = s.nodes[E].data.entries;
                let cascade = w.request.op == Op::Cascade;
                let n = match part {
                    WorkPart::Entry1 | WorkPart::Member1 => entries[0],
                    WorkPart::Entry2 | WorkPart::Member2 => entries[1],
                    _ => P,
                };
                let a = s.nodes[E].data.instances[n];
                let confirmed = entries.map(|n| s.nodes[E].data.instances[n].confirmed);
                let predecessor = match part {
                    WorkPart::Entry2 => w
                        .parts
                        .iter()
                        .any(|p| p.0 == WorkPart::Entry1 && p.1 == Outcome::Succeeded),
                    WorkPart::Member1 => confirmed[1],
                    WorkPart::Container => confirmed.into_iter().all(|v| v),
                    _ => true,
                };
                let skip = if cascade {
                    a.confirmed
                } else {
                    a.life == Life::Ready
                };
                Self::log(
                    s,
                    Event::PoolPart {
                        part,
                        predecessor,
                        ready: a.life == Life::Ready,
                        dispatched: !skip,
                        confirmed,
                        owner_retired: s.nodes[E].data.instances[P].life == Life::Retired,
                    },
                );
                if !predecessor && self.variant != Variant::EarlyPool {
                    self.add_part(s, id, part, Outcome::Conflict, false);
                } else if skip {
                    self.add_part(s, id, part, Outcome::Succeeded, false);
                } else if !cascade && (a.life == Life::Retired || !a.exists) {
                    self.add_part(s, id, part, Outcome::Conflict, false);
                } else {
                    let child = s.nodes[E].data.next_child;
                    s.nodes[E].data.next_child += 1;
                    s.nodes[E].data.work.get_mut(&id).unwrap().waiting = true;
                    self.start(
                        c,
                        s,
                        Request {
                            id: child,
                            op: if cascade { Op::Retire(n) } else { Op::Setup(n) },
                            caller: w.request.caller,
                            parent: Some(id),
                        },
                    );
                }
            }
            _ => {
                if part == WorkPart::Run {
                    let retire = matches!(w.request.op, Op::Retire(_));
                    let prior = s.nodes[E].data.instances[i].hooks[1].outcome;
                    if s.nodes[H].data.runs[i]
                        || (retire
                            && matches!(
                                prior,
                                Outcome::Succeeded | Outcome::Unknown | Outcome::Progress
                            ))
                    {
                        self.add_part(s, id, part, Outcome::Succeeded, false);
                        return;
                    }
                }
                if part == WorkPart::RetireHook
                    && w.parts
                        .iter()
                        .any(|p| p.0 == WorkPart::Run && p.1 != Outcome::Succeeded)
                {
                    self.add_part(s, id, part, Outcome::Unknown, false);
                    return;
                }
                let hook = match part {
                    WorkPart::SetupHook => Some(0),
                    WorkPart::RetireHook => Some(1),
                    _ => None,
                };
                let prior = hook.map_or(Outcome::None, |h| {
                    s.nodes[E].data.instances[i].hooks[h].outcome
                });
                if hook.is_some()
                    && matches!(
                        prior,
                        Outcome::Succeeded | Outcome::Unknown | Outcome::Progress
                    )
                    && self.variant != Variant::RepeatHook
                {
                    self.add_part(s, id, part, prior, false);
                    return;
                }
                let attempt = hook.map_or(0, |h| s.nodes[E].data.instances[i].hooks[h].attempt + 1);
                let outcome = self.provider_outcome(s, i, part, attempt);
                let target =
                    if hook.is_some() && self.variant == Variant::WrongHookInstance && i == I {
                        J
                    } else {
                        i
                    };
                let key = (
                    target,
                    if self.variant == Variant::WrongKey {
                        w.request.caller
                    } else {
                        u8::from(part == WorkPart::RetireHook)
                    },
                );
                let dispatch = Dispatch {
                    id,
                    instance: target,
                    part,
                    attempt,
                    outcome,
                    caller: w.request.caller,
                    key,
                    selected_definition: if hook.is_some()
                        && self.variant == Variant::CurrentTemplateHook
                    {
                        s.nodes[E].data.template
                    } else {
                        s.nodes[E].data.instances[target].definition
                    },
                };
                self.dispatch_log(s, dispatch, &w, prior);
                if let Some(h) = hook {
                    s.nodes[E].data.instances[i].hooks[h] = Hook {
                        outcome: Outcome::Progress,
                        attempt,
                        caller: w.request.caller,
                    };
                }
                s.nodes[E].data.work.get_mut(&id).unwrap().waiting = true;
                s.send(c, E, H, Message::Dispatch(dispatch));
            }
        }
    }
    fn dispatch_log(&self, s: &mut System<Self>, dispatch: Dispatch, w: &Work, prior: Outcome) {
        let a = s.nodes[E].data.instances[w.request.op.instance()];
        Self::log(
            s,
            Event::Part {
                dispatch,
                previous: w.parts.clone(),
                prior,
                setup: a.setup,
                retired: a.life == Life::Retired,
            },
        );
    }
    fn provider_outcome(&self, s: &System<Self>, i: usize, part: WorkPart, attempt: u8) -> Outcome {
        if part == WorkPart::Run {
            let prior = s.nodes[E]
                .data
                .work
                .values()
                .filter(|w| w.request.op == Op::Setup(i) && w.result != Outcome::Progress)
                .count();
            if ((self.part == Part::Setup && i == J) || (self.part == Part::Pool && i == E2))
                && prior == 0
            {
                return Outcome::Failed;
            }
        }
        if part == WorkPart::Stop && i == T && self.part == Part::Removal {
            return Outcome::Failed;
        }
        if part == WorkPart::Stop
            && i == J
            && self.part == Part::Removal
            && s.nodes[E].data.instances[i].hooks[1].attempt >= 1
        {
            return Outcome::Unknown;
        }
        if part == WorkPart::SetupHook {
            if i == I && attempt == 1 {
                return if self.part == Part::Setup {
                    Outcome::Failed
                } else {
                    Outcome::Unknown
                };
            }
            if (i == I && attempt == 2 && self.part == Part::Setup)
                || (i == J && self.part == Part::Removal && attempt == 1)
            {
                return Outcome::Unknown;
            }
        }
        if part == WorkPart::RetireHook && i == I && attempt == 2 {
            return Outcome::Unknown;
        }
        if part == WorkPart::RetireHook && attempt == 1 {
            return if i == E2 {
                Outcome::Unknown
            } else {
                Outcome::Failed
            };
        }
        Outcome::Succeeded
    }
    fn finish(&self, c: &Config, s: &mut System<Self>, id: u16, forced: Option<Outcome>) {
        let w = s.nodes[E].data.work[&id].clone();
        if w.result != Outcome::Progress {
            return;
        }
        let r = w.request;
        let i = r.op.instance();
        let mut outcome = forced.unwrap_or_else(|| Self::result(&w.parts));
        if self.variant == Variant::FalseSuccess && outcome == Outcome::Unknown {
            outcome = Outcome::Succeeded;
        }
        s.nodes[E].data.work.get_mut(&id).unwrap().result = outcome;
        if self.variant == Variant::DelayRetirement && matches!(r.op, Op::Retire(_)) {
            s.nodes[E].data.instances[i].life = Life::Retired;
        }
        let before = s.nodes[E].data.instances[i];
        if matches!(r.op, Op::Setup(_) | Op::Open(_))
            && outcome == Outcome::Succeeded
            && before.life != Life::Retired
        {
            s.nodes[E].data.instances[i].life = Life::Ready;
        }
        if self.variant == Variant::Regress && matches!(r.op, Op::Observe(_)) {
            s.nodes[E].data.instances[i].life = Life::Registered;
        }
        if matches!(r.op, Op::Retire(_)) {
            let a = &mut s.nodes[E].data.instances[i];
            a.confirmed |= outcome == Outcome::Succeeded || !a.setup;
            a.shutdown = if a.confirmed {
                Shutdown::Confirmed
            } else if outcome == Outcome::Unknown {
                Shutdown::Unknown
            } else {
                Shutdown::Pending
            };
            if self.variant == Variant::HookPending
                && w.parts
                    .iter()
                    .any(|p| p.0 == WorkPart::RetireHook && p.1 == Outcome::Failed)
            {
                a.shutdown = Shutdown::Pending;
            }
        }
        let after = s.nodes[E].data.instances[i];
        Self::log(
            s,
            Event::End {
                request: r,
                parts: w.parts.clone(),
                outcome,
                before: w.before,
                after: after.life,
            },
        );
        self.state(s, i, before.life, r.op);
        if matches!(r.op, Op::Retire(_)) {
            Self::log(
                s,
                Event::Retire {
                    i,
                    setup: after.setup,
                    before: after.life,
                    shutdown: after.shutdown,
                },
            );
            if after.confirmed || self.variant == Variant::EarlyRemove {
                self.remove(s, i, false, false);
            }
            Self::existence(s, i);
        }
        if matches!(r.op, Op::Unregister(_)) {
            let stop = w.parts.iter().any(|p| p.0 == WorkPart::Stop);
            self.remove(s, i, true, stop);
        }
        if self.variant == Variant::Rollback
            && matches!(r.op, Op::Setup(_))
            && outcome != Outcome::Succeeded
        {
            Self::log(
                s,
                Event::Closed {
                    run_before: s.nodes[H].data.runs[I],
                    run_after: s.nodes[H].data.runs[I],
                    i: C,
                    why: r.op,
                    consumer: before.life,
                    after: Life::Retired,
                },
            );
            s.nodes[E].data.instances[C].exists = false;
        }
        if let Some(parent) = r.parent {
            let pw = s.nodes[E].data.work[&parent].clone();
            let part = self.plan(s, &pw)[pw.next as usize];
            let result = if pw.request.op == Op::Cascade && !s.nodes[E].data.instances[i].confirmed
            {
                Outcome::Unknown
            } else {
                outcome
            };
            self.add_part(s, parent, part, result, w.effect);
        } else {
            self.answer(c, s, r);
        }
    }
    fn timeout_work(&self, s: &System<Self>, w: &Work) -> bool {
        w.result == Outcome::Progress
            && (matches!(w.request.op, Op::Open(_))
                || (w.waiting
                    && self.plan(s, w).get(w.next as usize).is_some_and(|p| {
                        matches!(
                            p,
                            WorkPart::Run
                                | WorkPart::SetupHook
                                | WorkPart::RetireHook
                                | WorkPart::Stop
                                | WorkPart::Call
                        )
                    })))
    }
    fn fenced(s: &System<Self>, i: usize, h: usize) -> bool {
        let matches = |d: &Dispatch| {
            d.instance == i
                && d.part
                    == if h == 0 {
                        WorkPart::SetupHook
                    } else {
                        WorkPart::RetireHook
                    }
        };
        !s.links
            .iter()
            .flatten()
            .any(|m| matches!(m, Message::Dispatch(d) if matches(d)))
            && !s.nodes[H].data.jobs.iter().any(matches)
    }
    fn reconcile_hook(&self, s: &mut System<Self>, i: usize, retirement: bool) {
        let h = usize::from(retirement);
        let prior = s.nodes[E].data.instances[i].hooks[h];
        if prior.outcome != Outcome::Unknown {
            return;
        }
        let occurred = s.nodes[H].data.effects[i][h] > 0;
        let fenced = Self::fenced(s, i, h);
        if !occurred && !fenced && self.variant != Variant::Unfenced {
            return;
        }
        s.nodes[E].data.instances[i].hooks[h].outcome = if occurred {
            Outcome::Succeeded
        } else {
            Outcome::Failed
        };
        let a = s.nodes[E].data.instances[i];
        Self::log(
            s,
            Event::Reconciled {
                op: Op::ReconcileHook(i, retirement),
                invocation: None,
                unknown: true,
                occurred,
                fenced,
                latest: true,
                before: a.life,
                after: a.life,
                shutdown_before: a.shutdown,
                shutdown_after: a.shutdown,
                retired_consumer: false,
            },
        );
    }
    fn reconcile(
        &self,
        s: &mut System<Self>,
        op: Op,
        i: usize,
        retirement: bool,
        latest: bool,
        occurred: bool,
    ) {
        let matching: Vec<_> = s.nodes[E]
            .data
            .work
            .iter()
            .filter(|(_, w)| {
                w.result == Outcome::Unknown
                    && w.request.op
                        == if retirement {
                            Op::Retire(i)
                        } else if op == Op::ReconcileOpen {
                            Op::Open(i)
                        } else {
                            Op::Setup(i)
                        }
            })
            .map(|(&id, _)| id)
            .collect();
        let Some(id) = (if latest {
            matching.last()
        } else {
            matching.first()
        })
        .copied() else {
            return;
        };
        let before = s.nodes[E].data.instances[i];
        let is_latest = before.latest == Some(id);
        let retired_consumer =
            Self::consumer(i).is_some_and(|n| s.nodes[E].data.instances[n].life == Life::Retired);
        // Negative reconciliations require all dispatches of the invocation to be fenced.
        let fenced = !s
            .links
            .iter()
            .flatten()
            .any(|m| matches!(m, Message::Dispatch(d) if d.id == id))
            && !s.nodes[H].data.jobs.iter().any(|d| d.id == id);
        if !occurred
            && (!fenced
                || s.ledger
                    .events
                    .iter()
                    .any(|e| matches!(e, Event::EffectApplied(d) if d.id == id)))
        {
            return;
        }
        s.nodes[E].data.work.get_mut(&id).unwrap().result = if occurred {
            Outcome::Succeeded
        } else {
            Outcome::Failed
        };
        if retirement {
            let a = &mut s.nodes[E].data.instances[i];
            if occurred {
                a.confirmed = true;
                a.shutdown = Shutdown::Confirmed;
            } else if is_latest && self.variant != Variant::KeepLatestUnknown {
                a.shutdown = if a.confirmed {
                    Shutdown::Confirmed
                } else {
                    Shutdown::Pending
                };
            }
        } else if occurred
            && ((before.life != Life::Retired && !retired_consumer)
                || self.variant == Variant::Revive)
        {
            s.nodes[E].data.instances[i].life = Life::Ready;
        }
        let after = s.nodes[E].data.instances[i];
        Self::log(
            s,
            Event::Reconciled {
                op,
                invocation: Some(id),
                unknown: true,
                occurred,
                fenced,
                latest: is_latest,
                before: before.life,
                after: after.life,
                shutdown_before: before.shutdown,
                shutdown_after: after.shutdown,
                retired_consumer,
            },
        );
        if retirement && after.confirmed {
            self.remove(s, i, false, false);
        }
        if retirement {
            Self::existence(s, i);
        }
    }
    fn complete(&self, c: &Config, s: &mut System<Self>, index: usize, silence: bool) {
        let d = s.nodes[H].data.jobs.remove(index);
        let i = d.instance;
        // H's first J stop can report unknown without applying an effect.
        // This makes the earlier negative reconciliation truthful as well as fenced.
        let no_effect = self.part == Part::Removal
            && i == J
            && d.part == WorkPart::Stop
            && d.outcome == Outcome::Unknown
            && s.nodes[H].data.stops[i] == 0;
        if d.part == WorkPart::Stop {
            s.nodes[H].data.stops[i] += 1;
        }
        let effect = !no_effect
            && d.outcome != Outcome::Failed
            && match d.part {
                WorkPart::Run => !s.nodes[H].data.runs[i],
                WorkPart::Stop => s.nodes[H].data.runs[i],
                WorkPart::SetupHook | WorkPart::RetireHook => true,
                _ => false,
            };
        if effect {
            if matches!(s.nodes[E].data.work[&d.id].request.op, Op::Retire(_)) {
                Self::log(s, Event::EffectApplied(d));
            }
            match d.part {
                WorkPart::Run => s.nodes[H].data.runs[i] = true,
                WorkPart::Stop => s.nodes[H].data.runs[i] = false,
                WorkPart::SetupHook | WorkPart::RetireHook => {
                    let h = usize::from(d.part == WorkPart::RetireHook);
                    s.nodes[H].data.effects[i][h] += 1;
                    Self::log(
                        s,
                        Event::Applied {
                            dispatch: d,
                            count: s.nodes[H].data.effects[i][h],
                        },
                    );
                    if h == 1 && i == I {
                        let open = s.nodes[E].data.instances[D].exists;
                        Self::log(
                            s,
                            Event::Onward {
                                retirement: true,
                                retired: s.nodes[E].data.instances[i].life == Life::Retired,
                                open,
                                dispatched: open,
                            },
                        );
                    }
                }
                _ => {}
            }
        }
        if silence {
            s.nodes[H].data.silences += 1;
            Self::log(s, Event::Silence(d));
        } else {
            s.send(c, H, E, Message::Reply(d, effect));
        }
    }
}

impl Protocol for Lifecycle {
    type Node = Data;
    type Message = Message;
    type Ledger = Ledger;
    type Local = Local;
    fn gate_remaining_faults(&self) -> bool {
        true
    }
    fn fault_enabled(&self, s: &System<Self>, f: Fault) -> bool {
        if f != Fault::Crash {
            return true;
        }
        let stage = s.nodes[R].data.stage;
        match self.part {
            Part::Setup => stage == 1,
            Part::Removal => stage == 17 || stage == 31,
            Part::Pool => stage == 6,
        }
    }
    fn fault_allowed(&self, f: Fault, src: usize, dst: usize) -> bool {
        match f {
            Fault::Loss | Fault::Reorder => match self.faults {
                Faults::LossR | Faults::ReorderR => {
                    (src == R && dst == E) || (src == E && dst == R)
                }
                Faults::LossH | Faults::ReorderH => {
                    (src == E && dst == H) || (src == H && dst == E)
                }
                _ => false,
            },
            Fault::Crash | Fault::Recovery | Fault::Timer => src == E,
            _ => false,
        }
    }
    fn initial(&self, _: &Config) -> System<Self> {
        let mut s = State {
            nodes: vec![
                Node {
                    data: Data::default(),
                    status: Status::Running
                };
                3
            ],
            links: vec![vec![]; 9],
            cut: BTreeSet::new(),
            used: [0; 9],
            ledger: Ledger::default(),
        };
        let d = &mut s.nodes[E].data;
        d.backing = [true; N];
        d.records = [true; N];
        d.shared = true;
        d.next_child = 128;
        d.slot = C;
        d.entries = [E1, E2];
        d.template = 1;
        d.instances[C2].exists = false;
        d.instances[NEW].exists = false;
        d.instances[D].life = Life::Ready;
        d.instances[RT].life = Life::Ready;
        s
    }
    fn receive(&self, c: &Config, s: &mut System<Self>, _: usize, _: usize, msg: Message) {
        match msg {
            Message::Request(r) => self.start(c, s, r),
            Message::Dispatch(d) => s.nodes[H].data.jobs.push(d),
            Message::Answer(id) => {
                s.nodes[R].data.ack.insert(id);
            }
            Message::Reply(d, effect) => {
                let Some(w) = s.nodes[E].data.work.get(&d.id).cloned() else {
                    return;
                };
                if w.result != Outcome::Progress
                    || !w.waiting
                    || self.plan(s, &w).get(w.next as usize) != Some(&d.part)
                {
                    return;
                }
                if matches!(d.part, WorkPart::SetupHook | WorkPart::RetireHook) {
                    let h = usize::from(d.part == WorkPart::RetireHook);
                    s.nodes[E].data.instances[d.instance].hooks[h].outcome = d.outcome;
                }
                self.add_part(s, d.id, d.part, d.outcome, effect);
            }
        }
    }
    fn timer_enabled(&self, s: &System<Self>, node: usize) -> bool {
        if node != E || s.nodes[E].data.recovery {
            return false;
        }
        if self.faults == Faults::ShutdownTimer {
            let a = s.nodes[E].data.instances[I];
            return a.exists && a.life == Life::Retired && a.shutdown == Shutdown::Unknown;
        }
        let stage = s.nodes[R].data.stage;
        if self.faults == Faults::Silence {
            return s.ledger.events.iter().any(|e| matches!(e, Event::Silence(d) if s.nodes[E].data.work.get(&d.id).is_some_and(|w| w.result == Outcome::Progress && w.waiting && self.plan(s, w).get(w.next as usize) == Some(&d.part))));
        }
        let window = match self.part {
            Part::Setup => stage == 2,
            Part::Removal => stage == 17,
            Part::Pool => stage == 2,
        };
        window
            && s.nodes[E]
                .data
                .work
                .values()
                .any(|w| self.timeout_work(s, w))
    }
    fn timer(&self, c: &Config, s: &mut System<Self>, _: usize) {
        if self.faults == Faults::ShutdownTimer {
            let before = s.nodes[E].data.instances[I].shutdown;
            if self.variant == Variant::TimeConfirms {
                s.nodes[E].data.instances[I].confirmed = true;
                s.nodes[E].data.instances[I].shutdown = Shutdown::Confirmed;
            }
            let after = s.nodes[E].data.instances[I].shutdown;
            Self::log(s, Event::Tick { before, after });
            if self.variant == Variant::TimeConfirms {
                self.remove(s, I, false, false);
            }
            return;
        }
        let id = s.nodes[E]
            .data
            .work
            .iter()
            .find(|(_, w)| self.timeout_work(s, w))
            .map(|(&id, _)| id)
            .unwrap();
        let w = s.nodes[E].data.work[&id].clone();
        Self::log(s, Event::Timer);
        if matches!(w.request.op, Op::Open(_)) {
            self.finish(c, s, id, Some(Outcome::Unknown));
        } else {
            let part = self.plan(s, &w)[w.next as usize];
            if matches!(part, WorkPart::SetupHook | WorkPart::RetireHook) {
                let h = usize::from(part == WorkPart::RetireHook);
                s.nodes[E].data.instances[w.request.op.instance()].hooks[h].outcome =
                    Outcome::Unknown;
            }
            self.add_part(s, id, part, Outcome::Unknown, false);
        }
    }
    fn crash(&self, d: &mut Data) {
        d.cache = false;
        d.recovery = true;
    }
    fn recover(&self, _: &mut Data) {}
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        if node == R {
            let stage = s.nodes[R].data.stage;
            let script = self.script();
            if stage < script.len()
                && (self.part != Part::Pool
                    || stage != 2
                    || (s.nodes[H].data.jobs.is_empty()
                        && s.links[s.link(E, H)].is_empty()
                        && s.links[s.link(H, E)].is_empty()))
                && (stage == 0
                    || (0..script[stage - 1].len()).all(|j| {
                        s.nodes[R]
                            .data
                            .ack
                            .contains(&((stage as u16 - 1) * 2 + j as u16))
                    }))
            {
                out.push(Local::Send);
            }
        }
        if node == E {
            if s.nodes[E].data.recovery {
                out.push(Local::Recover);
                return;
            }
            for index in 0..s.nodes[E].data.observations.len() {
                out.push(Local::Observe(index, false));
                out.push(Local::Observe(index, true));
            }
            for (&id, w) in &s.nodes[E].data.work {
                if w.result == Outcome::Progress && !w.waiting {
                    out.push(Local::Advance(id));
                }
            }
            if s.nodes[R].data.stage == self.script().len()
                && s.links.iter().all(Vec::is_empty)
                && s.nodes[H].data.jobs.is_empty()
                && s.nodes[E].data.observations.is_empty()
                && s.nodes[E]
                    .data
                    .work
                    .values()
                    .all(|w| w.result != Outcome::Progress)
                && !s.ledger.events.contains(&Event::Quiescent)
            {
                out.push(Local::Quiescent);
            }
        }
        if node == H {
            for (i, d) in s.nodes[H].data.jobs.iter().enumerate() {
                out.push(Local::Complete(i));
                if self.faults == Faults::Silence
                    && s.nodes[H].data.silences == 0
                    && s.nodes[R].data.stage <= 2
                    && matches!(d.part, WorkPart::SetupHook | WorkPart::RetireHook)
                {
                    out.push(Local::Silence(i));
                }
            }
        }
    }
    fn local_step(&self, c: &Config, s: &mut System<Self>, _: usize, a: Local) {
        match a {
            Local::Send => {
                let stage = s.nodes[R].data.stage;
                for (j, op) in self.script()[stage].iter().enumerate() {
                    let r = Request {
                        id: stage as u16 * 2 + j as u16,
                        op: *op,
                        caller: (stage % 2 + 1) as u8,
                        parent: None,
                    };
                    Self::log(s, Event::ActorRequest(r));
                    s.send(c, R, E, Message::Request(r));
                }
                s.nodes[R].data.stage += 1;
            }
            Local::Observe(index, unavailable) => {
                let r = s.nodes[E].data.observations.remove(index);
                let i = r.op.instance();
                let before = s.nodes[E].data.instances[i].life;
                if self.variant == Variant::Regress {
                    s.nodes[E].data.instances[i].life = Life::Registered;
                }
                let after = s.nodes[E].data.instances[i].life;
                Self::log(
                    s,
                    Event::Observation {
                        i,
                        unavailable,
                        reported: if unavailable { None } else { Some(before) },
                        before,
                        after,
                    },
                );
                self.state(s, i, before, r.op);
                self.answer(c, s, r);
            }
            Local::Advance(id) => self.advance(c, s, id),
            Local::Complete(i) => self.complete(c, s, i, false),
            Local::Silence(i) => self.complete(c, s, i, true),
            Local::Recover => {
                s.nodes[E].data.recovery = false;
                Self::log(s, Event::Recovery);
            }
            Local::ReconcileHook(i, h) => self.reconcile_hook(s, i, h),
            Local::Quiescent => {
                for i in 0..N {
                    Self::existence(s, i);
                }
                Self::log(s, Event::Quiescent);
            }
        }
    }
    fn properties(&self) -> Vec<NamedProperty<Self>> {
        let mut out = vec![];
        for n in 0..SAFETY.len() {
            out.push(NamedProperty {
                property: Property::always(SAFETY[n], SAFETY_FN[n]),
                promises: IDS[n],
            });
        }
        for (name, predicate) in REACH {
            out.push(NamedProperty {
                property: Property::sometimes(name, predicate),
                promises: &["harness"],
            });
        }
        out
    }
}

type Predicate = fn(&Harness<Lifecycle>, &System<Lifecycle>) -> bool;
fn every(s: &System<Lifecycle>, predicate: impl Fn(&Event) -> bool) -> bool {
    s.ledger.events.iter().all(predicate)
}
fn any(s: &System<Lifecycle>, predicate: impl Fn(&Event) -> bool) -> bool {
    s.ledger.events.iter().any(predicate)
}
fn setup_order(s: &System<Lifecycle>, e: &Event) -> bool {
    let Event::Part {
        dispatch: d,
        previous,
        retired,
        ..
    } = e
    else {
        return true;
    };
    let Some(r) = s.ledger.events.iter().find_map(|e| match e {
        Event::Begin(r, _) if r.id == d.id => Some(*r),
        _ => None,
    }) else {
        return false;
    };
    let Op::Setup(i) = r.op else {
        return true;
    };
    if *retired || previous.iter().any(|p| p.1 != Outcome::Succeeded) || d.caller != r.caller {
        return false;
    }
    let has = |part| {
        previous
            .iter()
            .any(|p| p.0 == part && p.1 == Outcome::Succeeded)
    };
    match d.part {
        WorkPart::Open => i == I && previous.is_empty(),
        WorkPart::Run => match i {
            I => has(WorkPart::Open),
            P => has(WorkPart::Entry1) && has(WorkPart::Entry2),
            _ => previous.is_empty(),
        },
        WorkPart::SetupHook => has(WorkPart::Run),
        _ => false,
    }
}
// Inspect only earlier ledger entries: a later hook cannot justify an earlier stop,
// and a later retirement request cannot authorize an earlier state transition.
fn retirement_transitions(s: &System<Lifecycle>) -> bool {
    let mut requested = BTreeSet::new();
    let mut actors = BTreeMap::new();
    let mut dispatched = BTreeSet::new();
    let mut cascades = BTreeSet::new();
    let mut confirmed = BTreeSet::new();
    let mut closed = BTreeSet::new();
    for event in &s.ledger.events {
        match event {
            Event::ActorRequest(r) => {
                requested.insert(r.op);
                actors.insert(r.id, *r);
            }
            Event::Begin(r, _) if r.op == Op::Cascade => {
                if actors.get(&r.id) != Some(r) {
                    return false;
                }
                cascades.insert(r.id);
            }
            Event::RetirementDispatch { request, after, .. } => {
                if *after != Life::Retired
                    || match request.parent {
                        None => actors.get(&request.id) != Some(request),
                        Some(id) => !cascades.contains(&id),
                    }
                {
                    return false;
                }
                dispatched.insert(request.op.instance());
            }
            Event::Begin(r, _) if matches!(r.op, Op::Retire(_)) => {
                if !dispatched.contains(&r.op.instance()) {
                    return false;
                }
            }
            Event::Retire {
                i,
                shutdown: Shutdown::Confirmed,
                ..
            }
            | Event::Existence {
                i,
                shutdown: Shutdown::Confirmed,
                ..
            } => {
                confirmed.insert(*i);
            }
            Event::Reconciled {
                op: Op::ReconcileRetire(i, _, _),
                shutdown_after: Shutdown::Confirmed,
                ..
            } => {
                confirmed.insert(*i);
            }
            Event::Closed { i, why, .. } => {
                let consumer = Lifecycle::consumer(*i);
                let permitted = match why {
                    Op::Close(n) => i == n && requested.contains(why),
                    Op::Retire(n) => Lifecycle::target(*i) == *n && dispatched.contains(n),
                    Op::Unregister(n) => {
                        consumer == Some(*n) && (requested.contains(why) || confirmed.contains(n))
                    }
                    _ => false,
                };
                if !permitted {
                    return false;
                }
                closed.insert(*i);
            }
            Event::State {
                i,
                before,
                after: Life::Retired,
                why,
                ..
            } if *before != Life::Retired => {
                let permitted = match why {
                    Op::Retire(n) if i == n => dispatched.contains(i),
                    _ => closed.remove(i),
                };
                if !permitted {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}
fn retirement_order(s: &System<Lifecycle>) -> bool {
    let mut retirements = BTreeMap::new();
    let mut hooks = BTreeSet::new();
    let mut stops = BTreeSet::new();
    for event in &s.ledger.events {
        match event {
            Event::RetirementDispatch {
                request,
                setup,
                hook_prior,
                ..
            } => {
                retirements.insert(request.id, (*setup, *hook_prior));
            }
            Event::Part {
                dispatch: d,
                previous,
                ..
            } => {
                let Some((setup, prior)) = retirements.get(&d.id) else {
                    continue;
                };
                if !setup {
                    return false;
                }
                let has = |part| previous.iter().any(|p| p.0 == part);
                match d.part {
                    WorkPart::RetireHook => {
                        if stops.contains(&d.id)
                            || !previous
                                .iter()
                                .any(|p| p.0 == WorkPart::Run && p.1 == Outcome::Succeeded)
                        {
                            return false;
                        }
                        hooks.insert(d.id);
                    }
                    WorkPart::Stop => {
                        // A retained hook still contributes its outcome; neither failure
                        // nor unknown nor retention waives the required stop.
                        let eligible = matches!(prior, Outcome::None | Outcome::Failed);
                        let run_ok = previous
                            .iter()
                            .any(|p| p.0 == WorkPart::Run && p.1 == Outcome::Succeeded);
                        if !has(WorkPart::RetireHook)
                            || (eligible && run_ok && !hooks.contains(&d.id))
                        {
                            return false;
                        }
                        stops.insert(d.id);
                    }
                    _ => {}
                }
            }
            Event::End { request, parts, .. } => {
                if let Some((setup, _)) = retirements.get(&request.id) {
                    if *setup {
                        if !stops.contains(&request.id)
                            || !parts.iter().any(|p| p.0 == WorkPart::Stop)
                        {
                            return false;
                        }
                    } else if !parts.is_empty() {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }
    true
}
// The owner is established before its hook, independently of the dispatch's target.
fn hook_owners(s: &System<Lifecycle>) -> bool {
    let mut owners = BTreeMap::new();
    for e in &s.ledger.events {
        match e {
            Event::Begin(r, definition) => {
                owners.insert(r.id, (*r, *definition));
            }
            Event::Part { dispatch: d, .. }
                if matches!(d.part, WorkPart::SetupHook | WorkPart::RetireHook) =>
            {
                let Some((r, definition)) = owners.get(&d.id) else {
                    return false;
                };
                let part = match r.op {
                    Op::Setup(_) => WorkPart::SetupHook,
                    Op::Retire(_) => WorkPart::RetireHook,
                    _ => return false,
                };
                if d.instance != r.op.instance()
                    || d.part != part
                    || d.selected_definition != *definition
                    || d.caller != r.caller
                    || d.key != (r.op.instance(), u8::from(part == WorkPart::RetireHook))
                {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

// Only dispatch and completion events establish which retirement is still active.
fn active_shutdowns(s: &System<Lifecycle>) -> bool {
    let mut latest = BTreeMap::new();
    let mut confirmed = BTreeSet::new();
    for e in &s.ledger.events {
        match e {
            Event::RetirementDispatch { request, setup, .. } => {
                latest.insert(request.op.instance(), (request.id, true));
                if !setup {
                    confirmed.insert(request.op.instance());
                }
            }
            Event::End {
                request, outcome, ..
            } if matches!(request.op, Op::Retire(_)) => {
                let i = request.op.instance();
                if latest.get(&i).is_some_and(|&(id, _)| id == request.id) {
                    latest.insert(i, (request.id, false));
                }
                if *outcome == Outcome::Succeeded {
                    confirmed.insert(i);
                }
            }
            Event::Reconciled {
                op: Op::ReconcileRetire(i, _, _),
                occurred,
                latest: is_latest,
                shutdown_after,
                ..
            } => {
                if *occurred {
                    confirmed.insert(*i);
                }
                if !occurred
                    && *is_latest
                    && *shutdown_after
                        != if confirmed.contains(i) {
                            Shutdown::Confirmed
                        } else {
                            Shutdown::Pending
                        }
                {
                    return false;
                }
            }
            _ => {}
        }
        let value = match e {
            Event::Retire { i, shutdown, .. }
            | Event::State { i, shutdown, .. }
            | Event::Existence { i, shutdown, .. } => Some((*i, *shutdown)),
            Event::Reconciled {
                op, shutdown_after, ..
            } => Some((op.instance(), *shutdown_after)),
            _ => None,
        };
        if let Some((i, shutdown)) = value
            && latest.get(&i).is_some_and(|&(_, active)| active)
            && shutdown
                != if confirmed.contains(&i) {
                    Shutdown::Confirmed
                } else {
                    Shutdown::Pending
                }
        {
            return false;
        }
    }
    true
}

fn latest_negative(s: &System<Lifecycle>) -> bool {
    let mut latest = None;
    let mut outcomes = BTreeMap::new();
    let mut effects = BTreeSet::new();
    for e in &s.ledger.events {
        match e {
            Event::RetirementDispatch { request, .. } if request.op == Op::Retire(J) => {
                latest = Some(request.id)
            }
            Event::End {
                request, outcome, ..
            } => {
                outcomes.insert(request.id, *outcome);
            }
            Event::EffectApplied(d) => {
                effects.insert(d.id);
            }
            Event::Reconciled {
                op: Op::ReconcileRetire(J, _, _),
                invocation: Some(id),
                unknown: true,
                occurred: false,
                fenced: true,
                latest: true,
                shutdown_before: Shutdown::Unknown,
                ..
            } if latest == Some(*id)
                && outcomes.get(id) == Some(&Outcome::Unknown)
                && !effects.contains(id) =>
            {
                return true;
            }
            _ => {}
        }
    }
    false
}

const SAFETY_FN: [Predicate; 17] = [
    |_, s| {
        every(s, |e| match e {
            Event::State {
                before, after, why, ..
            } => {
                after >= before
                    && (!matches!(why, Op::Observe(_)) || before == after)
                    && (!matches!(why, Op::Close(_)) || *after == Life::Retired)
            }
            Event::Observation {
                unavailable,
                reported,
                before,
                after,
                ..
            } => before == after && (!unavailable || reported.is_none()),
            Event::Reconciled { before, after, .. } => after >= before,
            Event::End {
                request,
                outcome,
                before,
                after,
                ..
            } if matches!(request.op, Op::Setup(_) | Op::Open(_)) => {
                if *outcome == Outcome::Succeeded {
                    *after == Life::Ready
                } else {
                    after == before
                }
            }
            Event::Decision {
                op: Op::Setup(_),
                exists,
                life: Life::Retired,
                answer,
                dispatched,
                ..
            } => {
                !dispatched
                    && *answer
                        == if *exists {
                            Outcome::Conflict
                        } else {
                            Outcome::Invalid
                        }
            }
            _ => true,
        }) && retirement_transitions(s)
    },
    |_, s| every(s, |e| setup_order(s, e)),
    |_, s| {
        every(s, |e| match e {
            Event::Part {
                dispatch: d, prior, ..
            } if matches!(d.part, WorkPart::SetupHook | WorkPart::RetireHook) => {
                matches!(prior, Outcome::None | Outcome::Failed)
            }
            _ => true,
        }) && retirement_order(s)
    },
    |_, s| hook_owners(s),
    |_, s| {
        every(s, |e| match e {
            Event::End {
                request,
                parts,
                outcome,
                ..
            } if matches!(request.op, Op::Setup(_) | Op::Retire(_)) => {
                *outcome == Lifecycle::result(parts)
            }
            _ => true,
        }) && retirement_order(s)
    },
    |_, s| {
        every(s, |e| match e {
            Event::Closed {
                why: Op::Setup(_), ..
            } => false,
            Event::Closed {
                why: Op::Close(_),
                consumer,
                after,
                run_before,
                run_after,
                ..
            } => consumer == after && (*consumer != Life::Ready || run_before == run_after),
            _ => true,
        })
    },
    |_, s| {
        every(s, |e| match e {
            Event::Ends { remaining, .. } => remaining.is_empty(),
            Event::Decision {
                op: Op::Open(_),
                life,
                consumer,
                target,
                answer,
                dispatched,
                exists,
                ..
            } if *exists
                && *life != Life::Ready
                && (*consumer == Life::Retired || *target == Life::Retired) =>
            {
                !dispatched && *answer == Outcome::Conflict
            }
            Event::Decision {
                op: Op::Open(_),
                exists: true,
                life: Life::Registered,
                consumer,
                target: Life::Registered,
                tomb: false,
                dispatched,
                ..
            } if *consumer != Life::Retired => *dispatched,
            Event::Decision {
                op: Op::Call(_),
                target: Life::Retired,
                answer,
                dispatched,
                ..
            } => !dispatched && *answer == Outcome::Refused,
            Event::Onward {
                retirement: false,
                retired: true,
                dispatched,
                ..
            } => !dispatched,
            _ => true,
        })
    },
    |_, s| {
        every(s, |e| match e {
            Event::Replacement {
                old,
                new,
                explicit,
                life,
                definition,
                template,
                ..
            } => old != new && *explicit && *life == Life::Registered && definition == template,
            Event::Decision {
                op: Op::ReplaceC(true, true),
                exists: true,
                answer,
                dispatched,
                ..
            } => !dispatched && *answer == Outcome::Conflict,
            Event::Decision {
                op: Op::NewResource(_),
                life,
                consumer,
                answer,
                dispatched,
                ..
            } => {
                if *consumer == Life::Retired {
                    !dispatched && *answer == Outcome::Invalid
                } else if *life != Life::Retired {
                    !dispatched && *answer == Outcome::Conflict
                } else {
                    true
                }
            }
            _ => true,
        })
    },
    |_, s| {
        every(s, |e| match e {
            Event::Decision {
                op: Op::ReplaceC(auth, valid),
                tomb,
                answer,
                dispatched,
                ..
            } => {
                if !auth {
                    !dispatched && *answer == Outcome::Refused
                } else if !valid {
                    !dispatched && *answer == Outcome::Invalid
                } else if *tomb {
                    !dispatched && *answer == Outcome::Conflict
                } else {
                    true
                }
            }
            Event::Tombstone {
                kept, engine_made, ..
            } => *engine_made || *kept,
            _ => true,
        })
    },
    |_, s| {
        every(s, |e| {
            match e {
            Event::Tick { before, after } => before == after,
            Event::Existence { life: Life::Retired, shutdown: Shutdown::Confirmed, exists, .. } => !exists,
            Event::End { request, parts, .. } if matches!(request.op, Op::Retire(_)) && s.ledger.events.iter().any(|e| matches!(e, Event::Begin(Request { op: Op::Setup(i), .. }, _) if *i == request.op.instance())) => parts.iter().any(|p| p.0 == WorkPart::Stop),
            Event::Retire {
                setup: false,
                shutdown,
                ..
            } => *shutdown == Shutdown::Confirmed,
            Event::Removed {
                explicit,
                confirmed,
                stop,
                ..
            } => {
                if *explicit {
                    *stop
                } else {
                    *confirmed
                }
            }
            Event::Part {
                dispatch: d,
                setup: false,
                ..
            } => !matches!(d.part, WorkPart::RetireHook | WorkPart::Stop),
            Event::Decision {
                op: Op::Unregister(_),
                life,
                answer,
                dispatched,
                ..
            } if *life != Life::Retired => {
                !dispatched && matches!(answer, Outcome::Conflict | Outcome::Invalid)
            }
            _ => true,
        }
        }) && shutdowns(s, false)
            && active_shutdowns(s)
    },
    |_, s| {
        every(s, |e| {
            match e {
            Event::Begin(Request { parent: Some(parent), .. }, _) => s.ledger.events.iter().any(|e| matches!(e, Event::Begin(Request { id, op: Op::Setup(P) | Op::Cascade, .. }, _) if id == parent)),
            Event::PoolPart {
                part,
                predecessor,
                ready,
                dispatched,
                confirmed,
                ..
            } => {
                (!dispatched || *predecessor)
                    && (!matches!(part, WorkPart::Entry1 | WorkPart::Entry2)
                        || !ready
                        || !dispatched)
                    && (*part != WorkPart::Container || confirmed.iter().all(|v| *v))
            }
            Event::Decision {
                op: Op::Retire(P),
                members: false,
                dispatched,
                answer,
                ..
            } => !dispatched && *answer == Outcome::Conflict,
            _ => true,
        }
        })
    },
    |_, s| {
        every(s, |e| {
            match e {
            Event::Part { dispatch: d, .. } if d.part == WorkPart::Stop => s.ledger.events.iter().any(|e| matches!(e, Event::Begin(r, _) if r.id == d.id && r.op.instance() == d.instance)),
            Event::Retained {
                definition,
                before_definition,
                records,
                data,
                unrelated,
                ..
            } => definition == before_definition && *records && *data && *unrelated,
            _ => true,
        }
        })
    },
    |_, s| {
        every(s, |e| match e {
            Event::Reconciled {
                op,
                occurred,
                latest,
                before,
                after,
                shutdown_before,
                shutdown_after,
                retired_consumer,
                ..
            } => match op {
                Op::ReconcileSetup(_) | Op::ReconcileOpen if *occurred => {
                    if *before == Life::Retired || *retired_consumer {
                        before == after
                    } else {
                        *after == Life::Ready
                    }
                }
                Op::ReconcileRetire(_, _, _) => {
                    if *occurred {
                        *shutdown_after == Shutdown::Confirmed
                    } else if *latest {
                        *shutdown_after == Shutdown::Pending
                    } else {
                        shutdown_before == shutdown_after
                    }
                }
                _ => true,
            },
            _ => true,
        })
    },
    |_, s| {
        every(s, |e| match e {
            Event::Reconciled {
                occurred: false,
                fenced,
                ..
            } => *fenced,
            Event::Applied { count, .. } => *count <= 1,
            _ => true,
        })
    },
    |_, s| shutdowns(s, true),
    |_, s| {
        every(s, |e| match e {
            Event::Removed { closed, .. } => closed.is_empty(),
            _ => true,
        })
    },
    |_, s| {
        every(s, |e| match e {
            Event::Applied { count, .. } => *count <= 1,
            _ => true,
        })
    },
];
fn shutdowns(s: &System<Lifecycle>, literal: bool) -> bool {
    let mut last = BTreeMap::new();
    for e in &s.ledger.events {
        match e {
            Event::End {
                request,
                parts,
                outcome,
                ..
            } if matches!(request.op, Op::Retire(_)) => {
                last.insert(
                    request.op.instance(),
                    (
                        *outcome,
                        parts
                            .iter()
                            .any(|p| p.0 == WorkPart::RetireHook && p.1 == Outcome::Failed),
                    ),
                );
            }
            Event::Retire {
                i,
                shutdown,
                before: Life::Retired,
                ..
            } => {
                if let Some((outcome, failed_hook)) = last.remove(i) {
                    let expected = if literal && failed_hook {
                        Shutdown::Pending
                    } else {
                        match outcome {
                            Outcome::Succeeded => Shutdown::Confirmed,
                            Outcome::Unknown => Shutdown::Unknown,
                            _ => Shutdown::Pending,
                        }
                    };
                    if *shutdown != expected {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }
    true
}
const REACH: [(&str, Predicate); 44] = [
    ("MC6 reach: setup succeeded", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::End {
                    request: Request {
                        op: Op::Setup(_),
                        ..
                    },
                    outcome: Outcome::Succeeded,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: setup failed without effects", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::End {
                    request: Request {
                        op: Op::Setup(_),
                        ..
                    },
                    outcome: Outcome::Failed,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: setup unknown", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::End {
                    request: Request {
                        op: Op::Setup(_),
                        ..
                    },
                    outcome: Outcome::Unknown,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: failed hook new attempt", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Part {
                    prior: Outcome::Failed,
                    dispatch: Dispatch {
                        part: WorkPart::SetupHook,
                        ..
                    },
                    ..
                }
            )
        })
    }),
    ("MC6 reach: unknown hook reused", |_, s| {
        any(
            s,
            |e| matches!(e, Event::End { request, parts, .. } if matches!(request.op, Op::Setup(_)) && parts.iter().any(|p| p.0 == WorkPart::SetupHook && p.1 == Outcome::Unknown) && !s.ledger.events.iter().any(|e| matches!(e, Event::Part { dispatch, .. } if dispatch.id == request.id && dispatch.part == WorkPart::SetupHook))),
        )
    }),
    ("MC6 reach: observed unknown", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::State {
                    why: Op::Observe(_),
                    ..
                }
            )
        })
    }),
    ("MC6 reach: explicit ready close", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Closed {
                    why: Op::Close(_),
                    consumer: Life::Ready,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: tombstone conflict", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Decision {
                    op: Op::ReplaceC(true, true),
                    tomb: true,
                    answer: Outcome::Conflict,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: tombstone refused before conflict", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Decision {
                    op: Op::ReplaceC(false, _),
                    tomb: true,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: tombstone invalid before conflict", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Decision {
                    op: Op::ReplaceC(true, false),
                    tomb: true,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: explicit new Conduit", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Replacement {
                    conduit: true,
                    explicit: true,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: retired setup conflict", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Decision {
                    op: Op::Setup(_),
                    exists: true,
                    life: Life::Retired,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: removed setup invalid", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Decision {
                    op: Op::Setup(_),
                    exists: false,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: unopened retired consumer", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Decision {
                    op: Op::Open(_),
                    exists: true,
                    life: Life::Registered,
                    consumer: Life::Retired,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: unknown setup reconciled ready", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Reconciled {
                    op: Op::ReconcileSetup(_),
                    before: Life::Registered,
                    after: Life::Ready,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: unknown setup reconciled retired", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Reconciled {
                    op: Op::ReconcileSetup(_),
                    before: Life::Retired,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: unknown open reconciled retired", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Reconciled {
                    op: Op::ReconcileOpen,
                    retired_consumer: true,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: earlier retirement reconciled failed", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Reconciled {
                    op: Op::ReconcileRetire(_, _, _),
                    occurred: false,
                    latest: false,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: retirement reconciled confirmed", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Reconciled {
                    op: Op::ReconcileRetire(_, _, _),
                    occurred: true,
                    shutdown_after: Shutdown::Confirmed,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: never set up retired", |_, s| {
        any(s, |e| matches!(e, Event::Retire { setup: false, .. }))
    }),
    ("MC6 reach: target retired in flight", |_, s| {
        let mut flight = BTreeSet::new();
        for e in &s.ledger.events {
            match e {
                Event::Part {
                    dispatch:
                        Dispatch {
                            id,
                            part: WorkPart::Call,
                            ..
                        },
                    ..
                } => {
                    flight.insert(*id);
                }
                Event::End {
                    request:
                        Request {
                            id,
                            op: Op::Call(_),
                            ..
                        },
                    ..
                } => {
                    flight.remove(id);
                }
                Event::Retire { i: T, .. } if !flight.is_empty() => return true,
                _ => {}
            }
        }
        false
    }),
    ("MC6 reach: Pool setup twice", |_, s| {
        s.ledger
            .events
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    Event::Begin(
                        Request {
                            op: Op::Setup(P),
                            ..
                        },
                        _
                    )
                )
            })
            .count()
            == 2
    }),
    ("MC6 reach: Pool retirement blocked", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Decision {
                    op: Op::Retire(P),
                    members: false,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: fresh Resource name", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Replacement {
                    conduit: false,
                    new: NEW,
                    tomb: false,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: H silent", |_, s| {
        any(s, |e| matches!(e, Event::Silence(_)))
    }),
    ("MC6 reach: quiescent", |_, s| {
        s.ledger.events.contains(&Event::Quiescent)
    }),
    ("MC6 reach: closed declaration slot", |_, s| {
        any(
            s,
            |e| matches!(e, Event::End { request: Request { op: Op::Setup(I), .. }, parts, .. } if parts.iter().any(|p| p.0 == WorkPart::Open && p.1 == Outcome::Conflict)),
        )
    }),
    ("MC6 reach: replacement opened by setup", |_, s| {
        any(
            s,
            |e| matches!(e, Event::Part { dispatch: Dispatch { instance: C2, part: WorkPart::Open, id, .. }, .. } if s.ledger.events.iter().any(|e| matches!(e, Event::Begin(Request { id: n, op: Op::Setup(I), .. }, _) if n == id))),
        )
    }),
    ("MC6 reach: retired Pool entry invalid", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Decision {
                    op: Op::NewResource(_),
                    consumer: Life::Retired,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: unknown retirement hook retained", |_, s| {
        any(
            s,
            |e| matches!(e, Event::End { request, parts, .. } if matches!(request.op, Op::Retire(_)) && parts.iter().any(|p| p.0 == WorkPart::RetireHook && p.1 == Outcome::Unknown) && !s.ledger.events.iter().any(|e| matches!(e, Event::Part { dispatch, .. } if dispatch.id == request.id && dispatch.part == WorkPart::RetireHook))),
        )
    }),
    ("MC6 reach: retire hook reconciled", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Reconciled {
                    op: Op::ReconcileHook(_, true),
                    ..
                }
            )
        })
    }),
    ("MC6 reach: active unregistration conflict", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Decision {
                    op: Op::Unregister(_),
                    life: Life::Ready,
                    answer: Outcome::Conflict,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: retire hook onward", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Onward {
                    retirement: true,
                    retired: true,
                    open: true,
                    dispatched: true
                }
            )
        })
    }),
    ("MC6 reach: ordinary retired consumer refused", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Onward {
                    retirement: false,
                    retired: true,
                    open: true,
                    dispatched: false
                }
            )
        })
    }),
    ("MC6 reach: timer after unknown shutdown", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::Tick {
                    before: Shutdown::Unknown,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: failed retirement", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::End {
                    request: Request {
                        op: Op::Retire(_),
                        ..
                    },
                    outcome: Outcome::Failed,
                    ..
                }
            )
        })
    }),
    ("MC6 reach: changed template retained resolution", |_, s| {
        any(
            s,
            |e| matches!(e, Event::Retained { before_definition, template, .. } if before_definition != template),
        )
    }),
    ("MC6 reach: stop after failed retire hook", |_, s| {
        any(
            s,
            |e| matches!(e, Event::Part { dispatch: Dispatch { part: WorkPart::Stop, .. }, previous, .. } if previous.iter().any(|p| p.0 == WorkPart::RetireHook && p.1 == Outcome::Failed)),
        )
    }),
    ("MC6 reach: stop after unknown retire hook", |_, s| {
        any(
            s,
            |e| matches!(e, Event::Part { dispatch: Dispatch { part: WorkPart::Stop, .. }, previous, .. } if previous.iter().any(|p| p.0 == WorkPart::RetireHook && p.1 == Outcome::Unknown)),
        )
    }),
    ("MC6 reach: stop after retained retire hook", |_, s| {
        any(
            s,
            |e| matches!(e, Event::Part { dispatch: Dispatch { id, part: WorkPart::Stop, .. }, previous, .. } if previous.iter().any(|p| p.0 == WorkPart::RetireHook) && s.ledger.events.iter().any(|e| matches!(e, Event::RetirementDispatch { request, hook_prior: Outcome::Unknown | Outcome::Succeeded, .. } if request.id == *id))),
        )
    }),
    ("MC6 reach: confirmed instance removed", |_, s| {
        any(
            s,
            |e| matches!(e, Event::Existence { i, life: Life::Retired, shutdown: Shutdown::Confirmed, exists: false } if !Lifecycle::conduits().contains(i)),
        )
    }),
    ("MC6 reach: retirement at dispatch", |_, s| {
        any(s, |e| {
            matches!(
                e,
                Event::RetirementDispatch {
                    after: Life::Retired,
                    ..
                }
            )
        })
    }),
    (
        "MC6 reach: unavailable observation reported unknown",
        |_, s| {
            any(s, |e| {
                matches!(
                    e,
                    Event::Observation {
                        unavailable: true,
                        reported: None,
                        ..
                    }
                )
            })
        },
    ),
    (
        "MC6 reach: latest retirement reconciled not occurred",
        |_, s| latest_negative(s),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use stateright::Model;
    fn registered(
        part: Part,
        faults: Faults,
        variant: Variant,
    ) -> (Vec<&'static str>, Vec<&'static str>) {
        let mut broken = match part {
            Part::Setup => vec![],
            Part::Removal => vec![14, 15],
            Part::Pool => vec![14],
        };
        if variant != Variant::Correct {
            broken.push(variant.target());
        }
        if matches!(variant, Variant::KeepConduit | Variant::PreserveReferences) {
            broken.retain(|&n| n != 15);
        }
        if variant == Variant::HookPending {
            broken.retain(|&n| n != 14);
        }
        if variant == Variant::StopBeforeHook {
            broken.push(2);
        }
        if matches!(variant, Variant::RepeatHook | Variant::Unfenced) {
            broken.push(16);
        }
        broken.sort_unstable();
        broken.dedup();
        let mut reached = match part {
            Part::Setup => vec![0, 1, 2, 3, 4, 5, 14, 25],
            Part::Removal => vec![
                0, 1, 2, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 25,
            ],
            Part::Pool => vec![0, 1, 18, 19, 21, 22, 23, 25],
        };
        if faults == Faults::Silence {
            reached.push(24);
        }
        if matches!(faults, Faults::Timer | Faults::Silence) && part == Part::Pool {
            reached.push(2);
        }
        if faults == Faults::Crash && part == Part::Removal {
            reached.retain(|&n| n != 13);
        }
        if faults == Faults::Timer && part == Part::Removal {
            reached.push(16);
        }
        reached.extend(match part {
            Part::Setup => vec![27],
            Part::Removal => vec![26, 29, 30, 31, 32, 33],
            Part::Pool => vec![28],
        });
        if part == Part::Removal {
            reached.push(35);
        }
        if faults == Faults::ShutdownTimer {
            reached.push(34);
        }
        reached.extend(match part {
            Part::Setup => vec![36],
            Part::Removal => vec![36, 37, 38, 39, 40, 41],
            Part::Pool => vec![37, 38, 40, 41],
        });
        match variant {
            Variant::StopBeforeHook => {
                reached.retain(|&n| ![37, 38, 39].contains(&n));
            }
            Variant::KeepConfirmed => {
                reached.retain(|&n| n != 40);
            }
            Variant::TimeConfirms => {
                reached.retain(|&n| ![11, 13, 29, 30, 32, 33].contains(&n));
            }
            Variant::FalseSuccess => {
                reached.retain(|&n| n != 2 && n != 14);
            }
            Variant::Rollback => {
                reached.retain(|&n| n != 3 && n != 4 && n != 14);
            }
            Variant::RepeatHook => {
                reached.retain(|&n| n != 4);
            }
            Variant::AutoReplace => {
                reached.retain(|&n| n != 10);
            }
            Variant::EarlyRemove => {
                reached.retain(|&n| ![11, 13, 17, 18].contains(&n));
            }
            Variant::Revive => {
                reached.retain(|&n| n != 12);
            }
            Variant::EarlyPool => {
                reached.retain(|&n| ![18, 19, 23, 25].contains(&n));
            }
            _ => {}
        }
        (
            broken.into_iter().map(|n| SAFETY[n]).collect(),
            (0..REACH.len())
                .filter(|n| !reached.contains(n))
                .map(|n| REACH[n].0)
                .collect(),
        )
    }
    // Accepted observations beside, never in place of, the registered table.
    fn observed(
        part: Part,
        faults: Faults,
        variant: Variant,
    ) -> (Vec<&'static str>, Vec<&'static str>) {
        let (mut broken, missing) = registered(part, faults, variant);
        let mut reached: BTreeSet<_> = (0..REACH.len())
            .filter(|&n| !missing.contains(&REACH[n].0))
            .collect();
        if part == Part::Setup {
            reached.insert(10);
            if variant != Variant::Rollback {
                reached.insert(6);
            }
        }
        if part == Part::Pool && matches!(faults, Faults::Timer | Faults::Silence) {
            reached.insert(4);
        }
        match variant {
            Variant::TimeConfirms => {
                reached.extend([11, 13, 29, 30, 32, 33]);
            }
            Variant::RepeatHook => {
                broken.push(SAFETY[13]);
                reached.remove(&14);
            }
            Variant::Rollback => {
                broken.push(SAFETY[0]);
                reached.extend([3, 14, 26]);
            }
            Variant::IgnoreTombstone => {
                reached.remove(&7);
            }
            Variant::EarlyRemove => {
                for n in [29, 30, 32, 33, 38, 39] {
                    reached.remove(&n);
                }
            }
            Variant::StopBeforeHook => {
                // The truthful first negative now needs a stop before any effect.
                // This control instead performs the hook after that stop.
                reached.remove(&17);
                reached.insert(18);
            }
            Variant::WrongHookInstance => {
                reached.remove(&4);
            }
            Variant::EarlyPool => {
                reached.extend([18, 25]);
            }
            Variant::Revive => {
                broken.push(SAFETY[0]);
                broken.retain(|&name| name != SAFETY[15]);
                reached.insert(27);
            }
            _ => {}
        }
        if part != Part::Pool {
            reached.insert(42);
        }
        if part == Part::Removal {
            reached.insert(43);
        }
        if variant == Variant::KeepLatestUnknown {
            broken.push(SAFETY[12]);
        }
        (
            SAFETY.into_iter().filter(|n| broken.contains(n)).collect(),
            (0..REACH.len())
                .filter(|n| !reached.contains(n))
                .map(|n| REACH[n].0)
                .collect(),
        )
    }
    fn rows() -> Vec<(Part, Faults, Variant)> {
        let mut rows = vec![];
        for part in Part::ALL {
            for faults in Faults::ALL {
                rows.push((part, faults, Variant::Correct));
            }
        }
        rows.push((Part::Removal, Faults::ShutdownTimer, Variant::Correct));
        for v in Variant::BROKEN {
            let (p, f) = v.run();
            rows.push((p, f, v));
        }
        rows
    }
    fn take(m: &Harness<Lifecycle>, s: &mut System<Lifecycle>, action: Action<Local>) {
        let mut actions = vec![];
        m.actions(s, &mut actions);
        assert!(actions.contains(&action), "disabled action {action:?}");
        *s = m.next_state(s, action).unwrap();
    }
    fn drive_to(
        m: &Harness<Lifecycle>,
        s: &mut System<Lifecycle>,
        done: impl Fn(&System<Lifecycle>) -> bool,
    ) {
        for _ in 0..500 {
            if done(s) {
                return;
            }
            let mut actions = vec![];
            m.actions(s, &mut actions);
            let a = actions
                .into_iter()
                .find(|a| matches!(a, Action::Deliver { index: 0, .. } | Action::Local(_, _)))
                .expect("script can advance");
            *s = m.next_state(s, a).unwrap();
        }
        panic!("bounded script failed to reach its checkpoint");
    }
    #[test]
    fn pool_timer_expires_child_and_accepts_late_reply() {
        let m = model(Part::Pool, Faults::Timer, Variant::Correct);
        let mut s = m.init_states().remove(0);
        drive_to(&m, &mut s, |s| {
            s.nodes[E]
                .data
                .work
                .values()
                .any(|w| w.request.parent.is_some() && w.waiting)
        });
        let child = s.nodes[E]
            .data
            .work
            .values()
            .find(|w| w.request.parent.is_some() && w.waiting)
            .unwrap()
            .clone();
        let parent = child.request.parent.unwrap();
        assert!(!m.protocol.timeout_work(&s, &s.nodes[E].data.work[&parent]));
        assert!(m.protocol.timeout_work(&s, &child));
        take(&m, &mut s, Action::Timer(E));
        assert_eq!(s.nodes[E].data.work[&child.request.id].next, 99);
        take(
            &m,
            &mut s,
            Action::Local(E, Local::Advance(child.request.id)),
        );
        assert_eq!(s.nodes[E].data.work[&parent].parts[0].1, Outcome::Unknown);
        drive_to(&m, &mut s, |s| s.ledger.events.contains(&Event::Quiescent));
        assert!((m.property(SAFETY[10]).condition)(&m, &s));
    }
    #[test]
    fn flight_witness_uses_invocation_ids_after_an_earlier_end() {
        let m = model(Part::Removal, Faults::Base, Variant::Correct);
        let mut s = m.init_states().remove(0);
        let call = |id| Event::Part {
            dispatch: Dispatch {
                id,
                instance: T,
                part: WorkPart::Call,
                attempt: 0,
                outcome: Outcome::Succeeded,
                caller: 1,
                key: (T, 0),
                selected_definition: 1,
            },
            previous: vec![],
            prior: Outcome::None,
            setup: true,
            retired: false,
        };
        s.ledger.events = vec![
            call(10),
            Event::End {
                request: Request {
                    id: 10,
                    op: Op::Call(false),
                    caller: 1,
                    parent: None,
                },
                parts: vec![],
                outcome: Outcome::Succeeded,
                before: Life::Ready,
                after: Life::Ready,
            },
            call(11),
            Event::Retire {
                i: T,
                setup: true,
                before: Life::Ready,
                shutdown: Shutdown::Pending,
            },
        ];
        assert!((REACH[20].1)(&m, &s));
        s.ledger.events.remove(2);
        assert!(!(REACH[20].1)(&m, &s));
    }
    #[test]
    fn hook_dispatch_is_bound_to_owning_request_and_saved_definition() {
        let m = model(Part::Setup, Faults::Base, Variant::Correct);
        let mut s = m.init_states().remove(0);
        drive_to(&m, &mut s, |s| {
            any(s, |e| {
                matches!(
                    e,
                    Event::Part {
                        dispatch: Dispatch {
                            part: WorkPart::SetupHook,
                            ..
                        },
                        ..
                    }
                )
            })
        });
        assert!(hook_owners(&s));
        let index = s
            .ledger
            .events
            .iter()
            .position(|e| {
                matches!(
                    e,
                    Event::Part {
                        dispatch: Dispatch {
                            part: WorkPart::SetupHook,
                            ..
                        },
                        ..
                    }
                )
            })
            .unwrap();
        for mutation in 0..3 {
            let mut broken = s.clone();
            let Event::Part { dispatch, .. } = &mut broken.ledger.events[index] else {
                unreachable!()
            };
            match mutation {
                0 => {
                    dispatch.instance = J;
                    dispatch.key.0 = J;
                }
                1 => dispatch.selected_definition += 1,
                _ => {
                    dispatch.part = WorkPart::RetireHook;
                    dispatch.key.1 = 1;
                }
            }
            assert!(!hook_owners(&broken), "mutation {mutation}");
        }
        s.ledger.events.retain(|e| !matches!(e, Event::Begin(_, _)));
        assert!(!hook_owners(&s));
    }

    #[test]
    fn unavailable_observation_has_a_report_and_preserves_lifecycle() {
        let m = model(Part::Setup, Faults::Base, Variant::Correct);
        let mut s = m.init_states().remove(0);
        drive_to(&m, &mut s, |s| !s.nodes[E].data.observations.is_empty());
        assert!(!(REACH[42].1)(&m, &s));
        take(&m, &mut s, Action::Local(E, Local::Observe(0, true)));
        assert!((REACH[42].1)(&m, &s));
        assert!((SAFETY_FN[0])(&m, &s));
        let index = s
            .ledger
            .events
            .iter()
            .position(|e| matches!(e, Event::Observation { .. }))
            .unwrap();
        for mutation in 0..3 {
            let mut broken = s.clone();
            let Event::Observation {
                unavailable,
                reported,
                after,
                ..
            } = &mut broken.ledger.events[index]
            else {
                unreachable!()
            };
            match mutation {
                0 => *unavailable = false,
                1 => *reported = Some(Life::Ready),
                _ => *after = Life::Registered,
            }
            if mutation < 2 {
                assert!(!(REACH[42].1)(&m, &broken));
            }
            if mutation > 0 {
                assert!(!(SAFETY_FN[0])(&m, &broken));
            }
        }
    }

    #[test]
    fn active_retirement_checks_every_shutdown_carrier_and_confirmation() {
        let m = model(Part::Removal, Faults::Base, Variant::Correct);
        let mut s = m.init_states().remove(0);
        drive_to(&m, &mut s, |s| {
            any(s, |e| {
                matches!(e, Event::RetirementDispatch { setup: true, .. })
            })
        });
        assert!(active_shutdowns(&s));
        let i = I;
        let events = [
            Event::Retire {
                i,
                setup: true,
                before: Life::Retired,
                shutdown: Shutdown::Unknown,
            },
            Event::State {
                i,
                before: Life::Retired,
                after: Life::Retired,
                why: Op::Retire(i),
                exists: true,
                shutdown: Shutdown::Unknown,
            },
            Event::Existence {
                i,
                life: Life::Retired,
                shutdown: Shutdown::Unknown,
                exists: true,
            },
            Event::Reconciled {
                op: Op::ReconcileHook(i, true),
                invocation: None,
                unknown: true,
                occurred: false,
                fenced: true,
                latest: true,
                before: Life::Retired,
                after: Life::Retired,
                shutdown_before: Shutdown::Unknown,
                shutdown_after: Shutdown::Unknown,
                retired_consumer: false,
            },
        ];
        for event in events {
            let mut broken = s.clone();
            broken.ledger.events.push(event);
            assert!(!active_shutdowns(&broken));
        }
        let r = Request {
            id: 600,
            op: Op::Retire(J),
            caller: 1,
            parent: None,
        };
        for confirmation in 0..3 {
            let mut confirmed = m.init_states().remove(0);
            if confirmation == 1 {
                confirmed.ledger.events.push(Event::End {
                    request: Request { id: 599, ..r },
                    parts: vec![],
                    outcome: Outcome::Succeeded,
                    before: Life::Retired,
                    after: Life::Retired,
                });
            } else if confirmation == 2 {
                confirmed.ledger.events.push(Event::Reconciled {
                    op: Op::ReconcileRetire(J, false, true),
                    invocation: Some(599),
                    unknown: true,
                    occurred: true,
                    fenced: true,
                    latest: false,
                    before: Life::Retired,
                    after: Life::Retired,
                    shutdown_before: Shutdown::Unknown,
                    shutdown_after: Shutdown::Confirmed,
                    retired_consumer: false,
                });
            }
            confirmed.ledger.events.push(Event::RetirementDispatch {
                request: r,
                setup: confirmation != 0,
                hook_prior: Outcome::None,
                after: Life::Retired,
            });
            confirmed.ledger.events.push(Event::Retire {
                i: J,
                setup: confirmation != 0,
                before: Life::Retired,
                shutdown: Shutdown::Confirmed,
            });
            assert!(active_shutdowns(&confirmed));
        }
    }

    #[test]
    fn latest_negative_retirement_is_unknown_fenced_and_effect_free() {
        let m = model(Part::Removal, Faults::Base, Variant::Correct);
        let mut s = m.init_states().remove(0);
        drive_to(&m, &mut s, latest_negative);
        assert!((SAFETY_FN[9])(&m, &s));
        assert!((REACH[17].1)(&m, &s));
        let index = s
            .ledger
            .events
            .iter()
            .position(|e| {
                matches!(
                    e,
                    Event::Reconciled {
                        op: Op::ReconcileRetire(J, true, false),
                        ..
                    }
                )
            })
            .unwrap();
        for mutation in 0..4 {
            let mut broken = s.clone();
            let Event::Reconciled {
                fenced,
                latest,
                shutdown_after,
                invocation,
                ..
            } = &mut broken.ledger.events[index]
            else {
                unreachable!()
            };
            let id = invocation.unwrap();
            match mutation {
                0 => *fenced = false,
                1 => *latest = false,
                2 => *shutdown_after = Shutdown::Unknown,
                _ => {
                    let dispatch = broken
                        .ledger
                        .events
                        .iter()
                        .find_map(|e| match e {
                            Event::Part { dispatch, .. } if dispatch.id == id => Some(*dispatch),
                            _ => None,
                        })
                        .unwrap();
                    broken
                        .ledger
                        .events
                        .insert(index, Event::EffectApplied(dispatch));
                }
            }
            if mutation == 2 {
                assert!(!active_shutdowns(&broken));
            } else {
                assert!(!latest_negative(&broken));
            }
        }
        let mut applied = s.clone();
        let Event::Reconciled {
            invocation: Some(id),
            ..
        } = applied.ledger.events[index]
        else {
            unreachable!()
        };
        applied.ledger.events.truncate(index);
        applied.nodes[E].data.work.get_mut(&id).unwrap().result = Outcome::Unknown;
        let dispatch = applied
            .ledger
            .events
            .iter()
            .find_map(|e| match e {
                Event::Part { dispatch, .. } if dispatch.id == id => Some(*dispatch),
                _ => None,
            })
            .unwrap();
        applied.ledger.events.push(Event::EffectApplied(dispatch));
        let before = applied.clone();
        m.protocol.reconcile(
            &mut applied,
            Op::ReconcileRetire(J, true, false),
            J,
            true,
            true,
            false,
        );
        assert_eq!(
            applied, before,
            "an applied effect forbids a negative record"
        );
        drive_to(&m, &mut s, |s| s.ledger.events.contains(&Event::Quiescent));
        assert!((REACH[18].1)(&m, &s));
        // Every negative retirement record agrees with H's applied-effect ledger.
        for e in &s.ledger.events {
            if let Event::Reconciled {
                op: Op::ReconcileRetire(_, _, _),
                invocation: Some(id),
                occurred: false,
                ..
            } = e
            {
                assert!(!any(
                    &s,
                    |e| matches!(e, Event::EffectApplied(d) if d.id == *id)
                ));
            }
        }
        // The stop-before-hook control can choose a successful first stop; it must
        // really stop J, rather than borrowing the unknown/no-effect choice.
        let control = model(Part::Removal, Faults::Base, Variant::StopBeforeHook);
        let mut stopped = control.init_states().remove(0);
        drive_to(&control, &mut stopped, |s| {
            any(s, |e| {
                matches!(
                    e,
                    Event::End {
                        request: Request {
                            op: Op::Retire(J),
                            ..
                        },
                        ..
                    }
                )
            })
        });
        assert!(!stopped.nodes[H].data.runs[J]);
        assert!(any(&stopped, |e| matches!(
            e,
            Event::EffectApplied(Dispatch {
                instance: J,
                part: WorkPart::Stop,
                outcome: Outcome::Succeeded,
                ..
            })
        )));
    }

    #[test]
    fn full_runs_match_registered_table() {
        let mut differences = vec![];
        for (part, faults, variant) in rows() {
            let report = check_all(model(part, faults, variant));
            println!("MC6 {part:?}/{faults:?}/{variant:?}: {report}");
            let registered = registered(part, faults, variant);
            let observed = observed(part, faults, variant);
            if registered != observed {
                println!(
                    "MC6 recorded table difference {part:?}/{faults:?}/{variant:?}; see MC6-004"
                );
            }
            if report.broken != observed.0 {
                differences.push(format!(
                    "{part:?}/{faults:?}/{variant:?} safety: {:?}, expected {:?}",
                    report.broken, observed.0
                ));
            }
            if report.missing_witnesses != observed.1 {
                differences.push(format!(
                    "{part:?}/{faults:?}/{variant:?} missing: {:?}, expected {:?}",
                    report.missing_witnesses, observed.1
                ));
            }
        }
        assert!(
            differences.is_empty(),
            "unrecorded table differences:\n{}",
            differences.join("\n")
        );
    }
    #[test]
    fn first_refutations_are_ledger_only() {
        for v in Variant::BROKEN {
            let (part, faults) = v.run();
            let m = model(part, faults, v);
            let name = SAFETY[v.target()];
            println!("MC6 first refutation {part:?}/{faults:?}/{v:?}: {name}");
            let trace = assert_refuted(m.clone(), name);
            println!("MC6 first refutation {v:?}: {} actions", trace.len());
            let path =
                stateright::Path::from_actions(&m, m.init_states().remove(0), &trace).unwrap();
            let mut last = path.last_state().clone();
            for n in &mut last.nodes {
                n.data = Data::default();
            }
            last.links.iter_mut().for_each(Vec::clear);
            assert!(!(m.property(name).condition)(&m, &last));
        }
        println!("MC6 unfenced effect control first refutation");
        let _ = assert_refuted(
            model(Part::Setup, Faults::Timer, Variant::Unfenced),
            SAFETY[16],
        );
        for n in [14, 15] {
            println!("MC6 literal first refutation: {}", SAFETY[n]);
            let _ = assert_refuted(
                model(Part::Removal, Faults::Base, Variant::Correct),
                SAFETY[n],
            );
        }
    }
}
