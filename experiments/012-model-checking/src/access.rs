//! MC3: access decisions and invocation chains, independent of engine code.
use crate::harness::*;
use stateright::Property;
use std::time::Duration;

const A: usize = 0;
const R: usize = 1;
const E: usize = 2;
const T: usize = 3;
const U: usize = 4;
pub const CURRENT: &str = "ACC-001 ACC-002 ACC-011 ACC-036: current access";
pub const OPEN: &str = "CON-013: open permits dispatch";
pub const FLIGHT: &str = "ACC-034: in flight runs to its end";
pub const END_CONTROL: &str = "ACC-034 OPS-013 IMP-007: control preserves recorded end";
pub const ONWARD: &str = "ACC-016 ACC-017 ACC-019 ACC-034: target owns onward access";
pub const UNREADABLE: &str = "ACC-037: unreadable is unavailable";
pub const READY: &str = "CON-024: registered is unavailable";
pub const REFUSED: &str = "MC3: refused after close";
pub const COMPLETED: &str = "MC3: outcome after close";
pub const FORWARDED: &str = "MC3: onward dispatched";
pub const CHAIN_REFUSED: &str = "MC3: onward refused after chain closed";
pub const CANNOT_READ: &str = "MC3: unavailable unreadable";
pub const NOT_READY: &str = "MC3: unavailable registered";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Part {
    Base,
    Timers,
    ReorderA,
    ReorderService,
    LossA,
    LossService,
    Copies,
    EngineCrash,
    TargetCrash,
    Cut,
    Window,
}
impl Part {
    pub const ALL: [Self; 11] = [
        Self::Base,
        Self::Timers,
        Self::ReorderA,
        Self::ReorderService,
        Self::LossA,
        Self::LossService,
        Self::Copies,
        Self::EngineCrash,
        Self::TargetCrash,
        Self::Cut,
        Self::Window,
    ];
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Correct,
    Reuse,
    CloseRefused,
    Borrow,
    Chain,
    DropFilter,
    Caller,
    Ended,
    ReadOpen,
    Registered,
}
impl Variant {
    pub const BROKEN: [Self; 9] = [
        Self::Reuse,
        Self::CloseRefused,
        Self::Borrow,
        Self::Chain,
        Self::DropFilter,
        Self::Caller,
        Self::Ended,
        Self::ReadOpen,
        Self::Registered,
    ];
    pub fn part(self) -> Part {
        match self {
            Self::Reuse => Part::Copies,
            Self::Ended => Part::Timers,
            Self::ReadOpen => Part::Window,
            _ => Part::Base,
        }
    }
    pub fn target(self) -> &'static str {
        match self {
            Self::Reuse => CURRENT,
            Self::CloseRefused => FLIGHT,
            Self::Borrow | Self::Chain | Self::DropFilter | Self::Caller | Self::Ended => ONWARD,
            Self::ReadOpen => UNREADABLE,
            Self::Registered => READY,
            Self::Correct => panic!("correct has no seeded failure"),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Answer {
    Outcome,
    Refused,
    Unavailable,
    Invalid,
    Conflict,
    Unknown,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Outcome {
    Progress,
    Succeeded,
    Unknown,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum End {
    Reply,
    Timer,
    Recovery,
    TargetCrash,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Open,
    Close(u8),
    Request(u8),
    Invoke(u8),
    Onward(u8),
    ToU(u8),
    OnwardAnswer(u8, Answer),
    Reply(u8),
    ReplyU(u8),
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Data {
    engine: bool,
    version: u8, // saved: 0 initial, 1 c2 open, 2 c1 removed, 3 cT removed
    ready: bool, // lifecycle readiness survives implementation crashes
    window: u8,  // 0 unused, 1 unreadable, 2 restored
    onward_index: [Option<Outcome>; 2], // saved child invocations
    index: [Option<Outcome>; 2], // saved keyed invocations, op has effect and no outputs
    pending_recovery: bool,
    pending_target_crash: bool,
    jobs: [bool; 2], // T's volatile active calls
    timer_choice: Option<u8>,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Change {
    conduit: u8,
    open: bool,
    version: u8,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Decision {
    version: u8,
    read_version: u8,
    readable: bool,
    ready: bool,
    open: bool,
    prior: Option<Outcome>,
    dispatched: bool,
    answer: Option<Answer>,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct OnwardRequest {
    version: u8,
    caller: usize,
    filter: bool,
    conduit: u8,
    closed: bool,
    ended: bool,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Onward {
    end: Option<End>,
    version: u8,
    readable: bool,
    caller: usize,
    filter: bool,
    conduit: u8, // 2 cT, 3 cAU
    closed: bool,
    ended: bool,
    dispatched: bool,
    answer: Option<Answer>,
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Invocation {
    dispatch_version: Option<u8>,
    end: Option<End>,
    onward: Option<Onward>,
    request: Option<OnwardRequest>,
    reply: Option<Answer>, // original request's answer at T reply receipt
    closed_at_reply: bool,
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Ledger {
    changes: Vec<Change>,
    decisions: [Vec<Decision>; 2],
    invocations: [Invocation; 2],
}
impl Ledger {
    fn flight(&self) -> bool {
        self.invocations
            .iter()
            .all(|i| i.reply.is_none_or(|a| a == Answer::Outcome))
    }
    fn forwarded(&self) -> bool {
        self.invocations
            .iter()
            .any(|i| i.onward.as_ref().is_some_and(|o| o.dispatched))
    }
    fn chain_refused(&self) -> bool {
        self.invocations.iter().any(|i| {
            i.onward
                .as_ref()
                .is_some_and(|o| o.closed && o.answer == Some(Answer::Refused))
        })
    }
    fn completed(&self) -> bool {
        self.invocations
            .iter()
            .any(|i| i.closed_at_reply && i.reply == Some(Answer::Outcome))
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Local {
    SetupT,
    Unreadable,
    Readable,
    Recover,
    SelectTimer(u8),
    TargetUnknown,
}
#[derive(Clone)]
pub struct Access {
    pub variant: Variant,
    pub part: Part,
}
fn service(src: usize, dst: usize) -> bool {
    matches!(
        (src, dst),
        (E, T) | (T, E) | (E, U) | (U, E) | (T, U) | (U, T)
    )
}
fn open(version: u8, key: usize) -> bool {
    if key == 0 { version < 2 } else { version >= 1 }
}
pub fn model(variant: Variant, part: Part) -> Harness<Access> {
    let mut budgets = [0; 9];
    for f in match part {
        Part::ReorderA | Part::ReorderService => vec![Fault::Reorder],
        Part::LossA | Part::LossService => vec![Fault::Loss],
        Part::Copies => vec![Fault::Duplication],
        Part::EngineCrash | Part::TargetCrash => vec![Fault::Crash, Fault::Recovery],
        Part::Cut => vec![Fault::Cut],
        _ => vec![],
    } {
        budgets[f as usize] = 1;
    }
    if part == Part::Timers {
        budgets[Fault::Timer as usize] = 2;
    }
    Harness::new(
        Config {
            name: "MC3 split access faults",
            nodes: vec!["A", "R", "engine", "T", "U"],
            budgets,
            disabled_reasons: std::array::from_fn(|i| {
                (budgets[i] == 0)
                    .then_some("fault split; healing and permanent loss are outside MC3")
            }),
            messages_per_link: 3,
            unique_state_ceiling: 190_000,
            timeout: Duration::from_secs(10),
            bounds: "two keyed top-level requests, one wire copy, three FIFO access changes; one onward per invocation, one unreadable window, two timers",
        },
        Access { variant, part },
    )
}
impl Access {
    fn finish(&self, s: &mut System<Self>, key: usize, answer: Answer, end: End) {
        let outcome = if answer == Answer::Outcome {
            Outcome::Succeeded
        } else {
            Outcome::Unknown
        };
        s.nodes[E].data.index[key] = Some(outcome);
        s.ledger.invocations[key].end = Some(end);
        let original = s.ledger.decisions[key]
            .iter_mut()
            .find(|d| d.dispatched)
            .unwrap();
        if original.answer.is_none() {
            original.answer = Some(answer);
        }
    }
    fn finish_onward(&self, c: &Config, s: &mut System<Self>, k: usize, answer: Answer, end: End) {
        s.nodes[E].data.onward_index[k] = Some(if answer == Answer::Outcome {
            Outcome::Succeeded
        } else {
            Outcome::Unknown
        });
        let onward = s.ledger.invocations[k].onward.as_mut().unwrap();
        onward.end = Some(end);
        onward.answer = Some(answer);
        s.send(c, E, T, Message::OnwardAnswer(k as u8, answer));
    }
    fn recover_ledger(&self, c: &Config, s: &mut System<Self>) {
        if s.nodes[E].data.pending_recovery {
            for k in 0..2 {
                if s.nodes[E].data.index[k] == Some(Outcome::Progress) {
                    self.finish(s, k, Answer::Unknown, End::Recovery);
                }
            }
            for k in 0..2 {
                if s.nodes[E].data.onward_index[k] == Some(Outcome::Progress) {
                    self.finish_onward(c, s, k, Answer::Unknown, End::Recovery);
                }
            }
            s.nodes[E].data.pending_recovery = false;
        }
        self.observe_target_crash(s);
    }
    fn observe_target_crash(&self, s: &mut System<Self>) {
        if s.nodes[T].data.pending_target_crash {
            for k in 0..2 {
                if s.nodes[E].data.index[k] == Some(Outcome::Progress) {
                    self.finish(s, k, Answer::Unknown, End::TargetCrash);
                }
            }
            // Keep the observation pending while T is down: another ready-target
            // dispatch may arrive before recovery. Recovery itself cannot erase it.
            if s.nodes[T].status == Status::Running {
                s.nodes[T].data.pending_target_crash = false;
            }
        }
    }
}
impl Protocol for Access {
    type Node = Data;
    type Message = Message;
    type Ledger = Ledger;
    type Local = Local;
    fn fault_allowed(&self, fault: Fault, src: usize, dst: usize) -> bool {
        match fault {
            Fault::Reorder => {
                if self.part == Part::ReorderA {
                    src == A && dst == E
                } else {
                    service(src, dst)
                }
            }
            Fault::Loss => {
                if self.part == Part::LossA {
                    src == A && dst == E
                } else {
                    service(src, dst)
                }
            }
            Fault::Duplication => src == A && dst == E,
            Fault::Crash => src == if self.part == Part::EngineCrash { E } else { T },
            Fault::Cut => (src, dst) == (E, T),
            _ => false,
        }
    }
    fn initial(&self, c: &Config) -> System<Self> {
        let mut s = State {
            nodes: (0..5)
                .map(|n| Node {
                    data: Data {
                        engine: n == E,
                        ..Data::default()
                    },
                    status: Status::Running,
                })
                .collect(),
            links: vec![vec![]; 25],
            cut: Default::default(),
            used: [0; 9],
            ledger: Ledger::default(),
        };
        for msg in [Message::Open, Message::Close(0), Message::Close(2)] {
            s.send(c, R, E, msg);
        }
        for key in 0..2 {
            s.send(c, A, E, Message::Request(key));
        }
        s
    }
    fn receive(&self, c: &Config, s: &mut System<Self>, _src: usize, dst: usize, msg: Message) {
        if dst == E {
            self.recover_ledger(c, s);
        }
        match msg {
            Message::Open | Message::Close(_) => {
                let (conduit, is_open) = match msg {
                    Message::Open => (1, true),
                    Message::Close(k) => (k, false),
                    _ => unreachable!(),
                };
                s.nodes[E].data.version += 1;
                s.ledger.changes.push(Change {
                    conduit,
                    open: is_open,
                    version: s.nodes[E].data.version,
                });
                if self.variant == Variant::CloseRefused
                    && conduit == 0
                    && s.nodes[E].data.index[0] == Some(Outcome::Progress)
                {
                    let d = s.ledger.decisions[0]
                        .iter_mut()
                        .find(|d| d.dispatched)
                        .unwrap();
                    d.answer = Some(Answer::Refused);
                }
            }
            Message::Request(key) => {
                let k = key as usize;
                let data = &s.nodes[E].data;
                let prior = data.index[k];
                let readable = data.window != 1;
                let current_open = open(data.version, k);
                let reused = self.variant == Variant::Reuse
                    && s.ledger.decisions.iter().flatten().any(|d| d.dispatched);
                let read_version = if reused {
                    s.ledger
                        .decisions
                        .iter()
                        .flatten()
                        .find(|d| d.dispatched)
                        .unwrap()
                        .read_version
                } else {
                    data.version
                };
                let access = if !readable && self.variant != Variant::ReadOpen {
                    Some(Answer::Unavailable)
                } else if !current_open
                    && !reused
                    && !(self.variant == Variant::ReadOpen && !readable)
                {
                    Some(Answer::Refused)
                } else if !data.ready && self.variant != Variant::Registered {
                    Some(Answer::Unavailable)
                } else {
                    None
                };
                let answer = access.or(match prior {
                    Some(Outcome::Progress) => Some(Answer::Conflict),
                    Some(Outcome::Succeeded) => Some(Answer::Outcome),
                    Some(Outcome::Unknown) => Some(Answer::Unknown),
                    None => None,
                });
                let dispatched = answer.is_none();
                s.ledger.decisions[k].push(Decision {
                    version: data.version,
                    read_version,
                    readable,
                    ready: data.ready,
                    open: current_open,
                    prior,
                    dispatched,
                    answer,
                });
                if dispatched {
                    s.nodes[E].data.index[k] = Some(Outcome::Progress);
                    s.ledger.invocations[k].dispatch_version = Some(s.nodes[E].data.version);
                    s.send(c, E, T, Message::Invoke(key));
                }
            }
            Message::Invoke(key) => {
                let k = key as usize;
                s.nodes[T].data.jobs[k] = true;
                // Emission survives loss of the request before the engine decides it.
                s.ledger.invocations[k].request = Some(OnwardRequest {
                    version: s.nodes[E].data.version,
                    caller: A,
                    filter: k == 0,
                    conduit: 2,
                    closed: !open(s.nodes[E].data.version, k) || s.nodes[E].data.version == 3,
                    ended: s.ledger.invocations[k].end.is_some(),
                });
                s.send(c, T, E, Message::Onward(key));
            }
            Message::Onward(key) => {
                let k = key as usize;
                let data = &s.nodes[E].data;
                let closed = !open(data.version, k) || data.version == 3;
                let ended = data.index[k] != Some(Outcome::Progress);
                let conduit = if self.variant == Variant::Borrow && data.version == 3 {
                    3
                } else {
                    2
                };
                let own_open = conduit == 3 || data.version < 3;
                let chain_open = if self.variant == Variant::Chain || conduit == 3 {
                    true
                } else {
                    !closed
                };
                let readable = data.window != 1;
                let answer = if !readable {
                    Some(Answer::Unavailable)
                } else if !own_open || !chain_open || (ended && self.variant != Variant::Ended) {
                    Some(Answer::Refused)
                } else {
                    None
                };
                let dispatched = answer.is_none();
                s.ledger.invocations[k].onward = Some(Onward {
                    end: None,
                    version: data.version,
                    readable,
                    caller: if self.variant == Variant::Caller {
                        T
                    } else {
                        A
                    },
                    filter: k == 0 && self.variant != Variant::DropFilter,
                    conduit,
                    closed,
                    ended,
                    dispatched,
                    answer,
                });
                if let Some(a) = answer {
                    s.send(c, E, T, Message::OnwardAnswer(key, a));
                } else {
                    s.nodes[E].data.onward_index[k] = Some(Outcome::Progress);
                    s.send(c, E, U, Message::ToU(key));
                }
            }
            Message::ToU(key) => {
                // U supports the carried filter; its reply is relayed by the engine.
                s.send(c, U, E, Message::ReplyU(key));
            }
            Message::ReplyU(key) => {
                let k = key as usize;
                if s.nodes[E].data.onward_index[k] == Some(Outcome::Progress) {
                    self.finish_onward(c, s, k, Answer::Outcome, End::Reply);
                }
            }
            Message::OnwardAnswer(key, _) => {
                if s.nodes[T].data.jobs[key as usize] {
                    s.nodes[T].data.jobs[key as usize] = false;
                    s.send(c, T, E, Message::Reply(key));
                }
            }
            Message::Reply(key) => {
                let k = key as usize;
                if s.nodes[E].data.index[k] == Some(Outcome::Progress) {
                    self.finish(s, k, Answer::Outcome, End::Reply);
                }
                let original = s.ledger.decisions[k].iter().find(|d| d.dispatched).unwrap();
                s.ledger.invocations[k].reply = original.answer;
                s.ledger.invocations[k].closed_at_reply = !open(s.nodes[E].data.version, k);
            }
        }
    }
    fn timer_enabled(&self, s: &System<Self>, node: usize) -> bool {
        node == E && s.nodes[E].data.timer_choice.is_some() && !s.nodes[E].data.pending_recovery
    }
    fn timer(&self, c: &Config, s: &mut System<Self>, _node: usize) {
        let k = s.nodes[E].data.timer_choice.take().unwrap() as usize;
        if k < 2 {
            if s.nodes[E].data.index[k] == Some(Outcome::Progress) {
                self.finish(s, k, Answer::Unknown, End::Timer);
            }
        } else if s.nodes[E].data.onward_index[k - 2] == Some(Outcome::Progress) {
            self.finish_onward(c, s, k - 2, Answer::Unknown, End::Timer);
        }
    }
    fn crash(&self, node: &mut Data) {
        node.jobs = [false; 2];
        node.timer_choice = None;
        if node.engine {
            node.pending_recovery = true;
        } else {
            node.pending_target_crash = true;
        }
    }
    fn recover(&self, _node: &mut Data) {}
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        if node != E {
            return;
        }
        let data = &s.nodes[E].data;
        if data.pending_recovery {
            out.push(Local::Recover);
            return;
        }
        if s.nodes[T].data.pending_target_crash
            && (s.nodes[T].status == Status::Running
                || data.index.contains(&Some(Outcome::Progress)))
        {
            out.push(Local::TargetUnknown);
            return;
        }
        if !data.ready && s.nodes[T].status == Status::Running {
            out.push(Local::SetupT);
        }
        if self.part == Part::Window {
            match data.window {
                0 => out.push(Local::Unreadable),
                1 => out.push(Local::Readable),
                _ => {}
            }
        }
        if self.part == Part::Timers
            && data.timer_choice.is_none()
            && s.used[Fault::Timer as usize] < 2
        {
            for k in 0..2 {
                if data.index[k] == Some(Outcome::Progress) {
                    out.push(Local::SelectTimer(k as u8));
                }
                if data.onward_index[k] == Some(Outcome::Progress) {
                    out.push(Local::SelectTimer((k + 2) as u8));
                }
            }
        }
    }
    fn local_step(&self, c: &Config, s: &mut System<Self>, _node: usize, action: Local) {
        match action {
            Local::SetupT => s.nodes[E].data.ready = true,
            Local::Unreadable => s.nodes[E].data.window = 1,
            Local::Readable => s.nodes[E].data.window = 2,
            Local::Recover => self.recover_ledger(c, s),
            Local::TargetUnknown => self.observe_target_crash(s),
            Local::SelectTimer(k) => s.nodes[E].data.timer_choice = Some(k),
        }
    }
    fn properties(&self) -> Vec<NamedProperty<Self>> {
        vec![
            NamedProperty {
                property: Property::<Harness<Self>>::always(CURRENT, |_, s| {
                    s.ledger.decisions.iter().flatten().all(|d| {
                        (!d.dispatched || d.version == d.read_version)
                            && (!d.readable
                                || d.open
                                || (!d.dispatched && d.answer == Some(Answer::Refused)))
                    })
                }),
                promises: &["ACC-001", "ACC-002", "ACC-011", "ACC-036"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(OPEN, |_, s| {
                    s.ledger
                        .decisions
                        .iter()
                        .flatten()
                        .all(|d| !d.open || !d.readable || !d.ready || d.dispatched)
                }),
                promises: &["CON-013"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(FLIGHT, |_, s| s.ledger.flight()),
                promises: &["ACC-034"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(END_CONTROL, |_, s| {
                    s.ledger.invocations.iter().all(|i| {
                        i.reply.is_none_or(|a| {
                            if matches!(i.end, Some(End::Timer | End::Recovery | End::TargetCrash))
                            {
                                a == Answer::Unknown
                            } else {
                                a == Answer::Outcome
                            }
                        })
                    })
                }),
                promises: &["ACC-034", "OPS-013", "IMP-007"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(ONWARD, |_, s| {
                    s.ledger.invocations.iter().enumerate().all(|(k, i)| {
                        i.onward.as_ref().is_none_or(|o| {
                            !o.dispatched
                                || (o.conduit == 2
                                    && o.version < 3
                                    && o.caller == A
                                    && o.filter == (k == 0)
                                    && !o.closed
                                    && !o.ended)
                        })
                    })
                }),
                promises: &["ACC-016", "ACC-017", "ACC-019", "ACC-034"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(UNREADABLE, |_, s| {
                    s.ledger.decisions.iter().flatten().all(|d| {
                        d.readable || (!d.dispatched && d.answer == Some(Answer::Unavailable))
                    }) && s.ledger.invocations.iter().all(|i| {
                        i.onward.as_ref().is_none_or(|o| {
                            o.readable || (!o.dispatched && o.answer == Some(Answer::Unavailable))
                        })
                    })
                }),
                promises: &["ACC-037"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(READY, |_, s| {
                    s.ledger.decisions.iter().flatten().all(|d| {
                        !d.open
                            || !d.readable
                            || d.ready
                            || (!d.dispatched && d.answer == Some(Answer::Unavailable))
                    })
                }),
                promises: &["CON-024"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(REFUSED, |_, s| {
                    s.ledger
                        .decisions
                        .iter()
                        .flatten()
                        .any(|d| d.version >= 2 && !d.open && d.answer == Some(Answer::Refused))
                }),
                promises: &["ACC-011"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(COMPLETED, |_, s| {
                    s.ledger.completed()
                }),
                promises: &["ACC-034"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(FORWARDED, |_, s| {
                    s.ledger.forwarded()
                }),
                promises: &["ACC-016", "ACC-017"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(CHAIN_REFUSED, |_, s| {
                    s.ledger.chain_refused()
                }),
                promises: &["ACC-034"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(CANNOT_READ, |_, s| {
                    s.ledger
                        .decisions
                        .iter()
                        .flatten()
                        .any(|d| !d.readable && d.answer == Some(Answer::Unavailable))
                }),
                promises: &["ACC-037"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(NOT_READY, |_, s| {
                    s.ledger
                        .decisions
                        .iter()
                        .flatten()
                        .any(|d| d.open && !d.ready && d.answer == Some(Answer::Unavailable))
                }),
                promises: &["CON-024"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stateright::Model;
    #[test]
    fn target_crash_observation_survives_recovery_before_onward() {
        let m = model(Variant::Correct, Part::TargetCrash);
        let mut s = m.init_states().remove(0);
        for action in [
            Action::Local(E, Local::SetupT),
            Action::Deliver {
                src: A,
                dst: E,
                index: 0,
            },
            Action::Deliver {
                src: E,
                dst: T,
                index: 0,
            },
            Action::Crash(T),
            Action::Recover(T),
            Action::Deliver {
                src: T,
                dst: E,
                index: 0,
            },
        ] {
            let mut available = vec![];
            m.actions(&s, &mut available);
            assert!(available.contains(&action), "unavailable: {action:?}");
            s = m.next_state(&s, action).unwrap();
        }
        assert!(s.nodes[E].data.ready);
        assert_eq!(s.ledger.invocations[0].end, Some(End::TargetCrash));
        assert_eq!(s.ledger.decisions[0][0].answer, Some(Answer::Unknown));
        let onward = s.ledger.invocations[0].onward.as_ref().unwrap();
        assert!(onward.ended);
        assert!(!onward.dispatched);
        assert_eq!(onward.answer, Some(Answer::Refused));
        assert!(s.links[s.link(E, U)].is_empty());
    }
    fn expected(v: Variant, part: Part) -> (Vec<&'static str>, Vec<&'static str>) {
        let mut broken = vec![];
        if part == Part::Copies {
            broken.push(OPEN);
        }
        if matches!(part, Part::Timers | Part::EngineCrash) {
            broken.push(FLIGHT);
        }
        match v {
            Variant::Reuse => broken.push(CURRENT),
            Variant::CloseRefused => {
                broken.push(FLIGHT);
                broken.push(END_CONTROL);
            }
            Variant::Borrow
            | Variant::Chain
            | Variant::DropFilter
            | Variant::Caller
            | Variant::Ended => broken.push(ONWARD),
            Variant::ReadOpen => broken.push(UNREADABLE),
            Variant::Registered => broken.push(READY),
            Variant::Correct => {}
        }
        let mut missing = vec![];
        if part != Part::Window || v == Variant::ReadOpen {
            missing.push(CANNOT_READ);
        }
        if v == Variant::CloseRefused {
            missing.push(COMPLETED);
        }
        if v == Variant::Registered {
            missing.push(NOT_READY);
        }
        broken.sort();
        broken.dedup();
        missing.sort();
        (broken, missing)
    }
    #[test]
    fn full_runs_match_pre_registration() {
        for (v, part) in Part::ALL
            .into_iter()
            .map(|p| (Variant::Correct, p))
            .chain(Variant::BROKEN.into_iter().map(|v| (v, v.part())))
        {
            let report = check_all(model(v, part));
            println!("MC3 {v:?}/{part:?}: {report}");
            let mut broken = report.broken;
            let mut missing = report.missing_witnesses;
            broken.sort();
            missing.sort();
            let registered = expected(v, part);
            let mut observed_table = registered.clone();
            if part == Part::TargetCrash {
                // MC3-004: T's crash can also precede a late reply.
                observed_table.0.push(FLIGHT);
                observed_table.0.sort();
                println!(
                    "MC3 difference Correct/TargetCrash: in-flight expected holds, observed refuted (unknown after target crash)"
                );
            }
            if v == Variant::ReadOpen {
                // MC3-003: the unchanged table missed the separate not-ready path.
                observed_table.1.retain(|name| *name != CANNOT_READ);
                println!(
                    "MC3 difference ReadOpen/Window: unreadable witness expected missing, observed reached (target registered)"
                );
            }
            assert_eq!(
                (broken, missing),
                observed_table,
                "MC3 table mismatch {v:?}/{part:?}"
            );
        }
    }
    #[test]
    fn first_named_refutations() {
        for v in Variant::BROKEN {
            println!("MC3 refutation {v:?}: {}", v.target());
            let m = model(v, v.part());
            let trace = assert_refuted(m.clone(), v.target());
            println!("MC3 refutation {v:?}: {} actions", trace.len());
            let path =
                stateright::Path::from_actions(&m, m.init_states().remove(0), &trace).unwrap();
            let mut s = path.last_state().clone();
            s.nodes.iter_mut().for_each(|n| n.data = Data::default());
            assert!(!(m.property(v.target()).condition)(&m, &s));
        }
        println!("MC3 control refutation CloseRefused: {END_CONTROL}");
        assert_refuted(model(Variant::CloseRefused, Part::Base), END_CONTROL);
        for (part, property) in [
            (Part::Copies, OPEN),
            (Part::Timers, FLIGHT),
            (Part::EngineCrash, FLIGHT),
            (Part::TargetCrash, FLIGHT),
        ] {
            println!("MC3 literal promise refutation {part:?}: {property}");
            assert_refuted(model(Variant::Correct, part), property);
        }
    }
}
