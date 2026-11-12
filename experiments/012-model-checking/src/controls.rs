//! MC7a: controls, revisions, outcomes and model work; no engine code.
use crate::harness::*;
use stateright::Property;
use std::time::Duration;

const A: usize = 0;
const B: usize = 1;
const E: usize = 2;
const I: usize = 3;
const INITIAL: [u8; 2] = [0, 2];
pub const ADVANCE: &str = "MC7a REV-003: revision iff stored change";
pub const FRESH: &str = "MC7a REV-005: fresh revision";
pub const EXACT: &str = "MC7a REV-007: exact applied and stored values";
pub const UNKNOWN: &str = "MC7a REV-009: unknown preserves storage";
pub const RECON: &str = "MC7a REV-010: reconciliation uses current configuration";
pub const SCHEMA: &str = "MC7a REV-013: invalid reconciliation stays unknown";
pub const CONTENTION: &str = "MC7a REV-014: immediate contention answer";
pub const OUTCOME: &str = "MC7a OPS-001: exactly one outcome";
pub const CLAIM: &str = "MC7a OPS-002 OPS-003 OPS-004: implementation claim";
pub const REASON: &str = "MC7a OPS-005: reason without outcome";
pub const MISSING: &str = "MC7a OPS-007: missing or malformed means unknown";
pub const CANCEL: &str = "MC7a OPS-027: cancellation forwarded and unknown";
pub const REPEAT: &str = "MC7a OPS-034: succeeded output repeat";
pub const WORK: &str = "MC7a IMP-001 IMP-002: only explicit model work";
pub const ONWARD: &str = "MC7a IMP-008: onward needs marked parent";
pub const FENCE: &str = "MC7a ruling 75 A1: absent only after fence";
pub const ABSENT: &str = "MC7a control: no effect after reconciled absence";
pub const LATE: &str = "MC7a support OPS-013: late reply preserves outcome";
const SAFETY: [&str; 18] = [
    ADVANCE, FRESH, EXACT, UNKNOWN, RECON, SCHEMA, CONTENTION, OUTCOME, CLAIM, REASON, MISSING,
    CANCEL, REPEAT, WORK, ONWARD, FENCE, ABSENT, LATE,
];
const WITNESSES: [&str; 33] = [
    "MC7a reach: changing store",
    "MC7a reach: no-op store",
    "MC7a reach: change back",
    "MC7a reach: successful control",
    "MC7a reach: unknown control",
    "MC7a reach: reconciled occurred",
    "MC7a reach: reconciled absent",
    "MC7a reach: stale reconciliation",
    "MC7a reach: missing reconciliation revision",
    "MC7a reach: invalid merge after later change",
    "MC7a reach: contention",
    "MC7a reach: retired while in progress",
    "MC7a reach: retired success",
    "MC7a reach: retired reconciliation",
    "MC7a reach: removed reconciliation",
    "MC7a reach: implementation succeeded",
    "MC7a reach: implementation failed",
    "MC7a reach: implementation uncertain",
    "MC7a reach: partial effect",
    "MC7a reach: malformed result",
    "MC7a reach: cancelled unknown",
    "MC7a reach: cancelled confirmed failed",
    "MC7a reach: late confirmation",
    "MC7a reach: repeat with filtered outputs",
    "MC7a reach: repeat without outputs",
    "MC7a reach: expired outputs",
    "MC7a reach: implicit events",
    "MC7a reach: marked onward",
    "MC7a reach: unmarked onward",
    "MC7a reach: reconciliation under configured fault",
    "MC7a reach: valid partial merge after untouched setting changed",
    "MC7a reach: ordinary reply before end",
    "MC7a reach: cancelled unknown reply before end",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scene {
    Control,
    Schema,
    CurrentMerge,
    Retired,
    Removed,
    Outcomes,
    Cancellation,
    Repeats,
    ModelWork,
}
impl Scene {
    fn control(self) -> bool {
        matches!(
            self,
            Self::Control | Self::Schema | Self::CurrentMerge | Self::Retired | Self::Removed
        )
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Part {
    Base,
    LossCaller,
    ReorderCaller,
    LossReceipt,
    ReorderReceipt,
    LossImplementation,
    ReorderImplementation,
    Crash,
    Silence,
}
impl Part {
    pub const ALL: [Self; 9] = [
        Self::Base,
        Self::LossCaller,
        Self::ReorderCaller,
        Self::LossReceipt,
        Self::ReorderReceipt,
        Self::LossImplementation,
        Self::ReorderImplementation,
        Self::Crash,
        Self::Silence,
    ];
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Correct,
    StaleRevision,
    ReusedRevision,
    Substitute,
    StoreUnknown,
    WrongReconcile,
    OriginalReconcile,
    AcceptInvalid,
    QueueContender,
    DoubleOutcome,
    ReasonForOutcome,
    FalseSuccess,
    PartialFailed,
    UncertainFailed,
    OutcomeForReason,
    TrustMalformed,
    DropCancellation,
    CancelFailed,
    CancelUnknownFailed,
    LeakOutputs,
    RepeatNoOutputsConflict,
    ImplicitWork,
    ClosedWork,
    AllowOnward,
    Unfenced,
    LateConfirmation,
}
impl Variant {
    pub const BROKEN: [Self; 25] = [
        Self::StaleRevision,
        Self::ReusedRevision,
        Self::Substitute,
        Self::StoreUnknown,
        Self::WrongReconcile,
        Self::OriginalReconcile,
        Self::AcceptInvalid,
        Self::QueueContender,
        Self::DoubleOutcome,
        Self::ReasonForOutcome,
        Self::FalseSuccess,
        Self::PartialFailed,
        Self::UncertainFailed,
        Self::OutcomeForReason,
        Self::TrustMalformed,
        Self::DropCancellation,
        Self::CancelFailed,
        Self::CancelUnknownFailed,
        Self::LeakOutputs,
        Self::RepeatNoOutputsConflict,
        Self::ImplicitWork,
        Self::ClosedWork,
        Self::AllowOnward,
        Self::Unfenced,
        Self::LateConfirmation,
    ];
    pub fn scene(self) -> Scene {
        match self {
            Self::AcceptInvalid => Scene::Schema,
            Self::OriginalReconcile => Scene::CurrentMerge,
            Self::DoubleOutcome
            | Self::ReasonForOutcome
            | Self::FalseSuccess
            | Self::PartialFailed
            | Self::UncertainFailed
            | Self::OutcomeForReason
            | Self::TrustMalformed => Scene::Outcomes,
            Self::DropCancellation
            | Self::CancelFailed
            | Self::CancelUnknownFailed
            | Self::LateConfirmation => Scene::Cancellation,
            Self::LeakOutputs | Self::RepeatNoOutputsConflict => Scene::Repeats,
            Self::ImplicitWork | Self::ClosedWork | Self::AllowOnward => Scene::ModelWork,
            _ => Scene::Control,
        }
    }
    pub fn target(self) -> &'static str {
        match self {
            Self::StaleRevision => ADVANCE,
            Self::ReusedRevision => FRESH,
            Self::Substitute => EXACT,
            Self::StoreUnknown => UNKNOWN,
            Self::WrongReconcile | Self::OriginalReconcile => RECON,
            Self::AcceptInvalid => SCHEMA,
            Self::QueueContender => CONTENTION,
            Self::DoubleOutcome | Self::ReasonForOutcome => OUTCOME,
            Self::FalseSuccess | Self::PartialFailed | Self::UncertainFailed => CLAIM,
            Self::OutcomeForReason => REASON,
            Self::TrustMalformed => MISSING,
            Self::DropCancellation | Self::CancelFailed | Self::CancelUnknownFailed => CANCEL,
            Self::LeakOutputs | Self::RepeatNoOutputsConflict => REPEAT,
            Self::ImplicitWork | Self::ClosedWork => WORK,
            Self::AllowOnward => ONWARD,
            Self::Unfenced => ABSENT,
            Self::LateConfirmation => LATE,
            Self::Correct => panic!("no correct refutation"),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Answer {
    Succeeded,
    Failed,
    Unknown,
    Refused,
    Invalid,
    Conflict,
    Unavailable,
    Double,
}
impl Answer {
    fn reason(self) -> bool {
        matches!(
            self,
            Self::Refused | Self::Invalid | Self::Conflict | Self::Unavailable
        )
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Effect {
    None,
    Full,
    Partial,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Reply {
    Succeeded,
    Failed,
    Unknown,
    MissingTag,
    BothTags,
    WrongError,
    InvalidOutput,
    Refused,
    Unavailable,
}
impl Reply {
    fn malformed(self) -> bool {
        matches!(
            self,
            Self::MissingTag | Self::BothTags | Self::WrongError | Self::InvalidOutput
        )
    }
    fn answer(self) -> Answer {
        match self {
            Self::Succeeded => Answer::Succeeded,
            Self::Failed => Answer::Failed,
            Self::Refused => Answer::Refused,
            Self::Unavailable => Answer::Unavailable,
            _ => Answer::Unknown,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    Change,
    Setup,
    Retire,
    Refused,
    Invalid,
    Repeat,
    SucceededRepeat,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Request,
    Probe,
    Contender(Kind),
    ClientResult(u8, Answer),
    Dispatch,
    Reply(Reply),
    Cancel,
    Reconcile {
        occurred: bool,
        revision: Option<u8>,
    },
    Repeat(u8),
    WorkRequest {
        marked: bool,
        open: bool,
        listed: bool,
    },
    WorkCall(bool),
    Onward(bool),
    WorkLeaf,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Data {
    // Engine saved state; attempt identity is fixed, its original sent patch is immutable.
    active: bool,
    ended: Option<Answer>,
    dispatched: bool,
    fenced: bool,
    stored: [u8; 2],
    revision: u8,
    next_revision: u8,
    retired: bool,
    removed: bool,
    unknown_mask: [bool; 2],
    stage: u8,
    cancelling: bool,
    recovery: bool,
    suffix: u8,
    recon_sent: bool,
    // Implementation process, distinct from the engine's durable invocation state.
    job: bool,
    stopped: bool,
    silence_used: bool,
    contender_queued: bool,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            active: false,
            ended: None,
            dispatched: false,
            fenced: false,
            stored: INITIAL,
            revision: 0,
            next_revision: 1,
            retired: false,
            removed: false,
            unknown_mask: [false; 2],
            stage: 0,
            cancelling: false,
            recovery: false,
            suffix: 0,
            recon_sent: false,
            job: false,
            stopped: false,
            silence_used: false,
            contender_queued: false,
        }
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Send {
        src: usize,
        dst: usize,
        message: Message,
    },
    Dispatch {
        patch: [Option<u8>; 2],
    },
    Contender {
        kind: Kind,
        busy: bool,
        answer: Answer,
        dispatched: bool,
        queued: bool,
    },
    Effect {
        effect: Effect,
        applied: [Option<u8>; 2],
        after_absent: bool,
    },
    Report {
        effect: Effect,
        applied: [Option<u8>; 2],
        sent: [Option<u8>; 2],
        uncertain: bool,
        reply: Reply,
    },
    Store {
        before: [u8; 2],
        after: [u8; 2],
        old: u8,
        new: u8,
    },
    End {
        control: bool,
        answer: Answer,
        received: Option<Reply>,
        source: &'static str,
        patch: [Option<u8>; 2],
        applied: [Option<u8>; 2],
        before: [u8; 2],
        after: [u8; 2],
        old: u8,
        new: u8,
        retired: bool,
        unknown: [bool; 2],
    },
    Reconcile {
        occurred: bool,
        fenced: bool,
        retired: bool,
        removed: bool,
        named: Option<u8>,
        patch: [Option<u8>; 2],
        before: [u8; 2],
        after: [u8; 2],
        old: u8,
        new: u8,
        answer: Answer,
        outcome: Answer,
        unknown: [bool; 2],
        faults: [u8; 9],
    },
    Cancel {
        forwarded: bool,
    },
    CancelEnd {
        answer: Answer,
    },
    Received {
        reply: Reply,
        in_progress: bool,
    },
    BadResult {
        answer: Answer,
    },
    Late {
        before: Answer,
        after: Answer,
        confirmation: bool,
    },
    Fence(&'static str),
    Retired,
    Removed,
    Silence,
    LaterChange,
    Repeat {
        case: u8,
        answer: Answer,
        outputs: bool,
        dispatched: bool,
        invocation: Option<u8>,
    },
    SeedSucceeded {
        invocation: u8,
        outputs: bool,
    },
    Implicit {
        kind: u8,
        work: bool,
    },
    Work {
        marked: bool,
        open: bool,
        listed: bool,
        invocation: Option<u8>,
    },
    Onward {
        marked: bool,
        dispatched: bool,
        answer: Answer,
    },
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ledger(Vec<Event>, Part, [Option<bool>; 2], [Option<Answer>; 8]);
fn merge(mut before: [u8; 2], patch: [Option<u8>; 2]) -> [u8; 2] {
    for i in 0..2 {
        if let Some(value) = patch[i] {
            before[i] = value;
        }
    }
    before
}
fn valid(values: [u8; 2]) -> bool {
    values[0] <= values[1]
}
impl Ledger {
    fn has(&self, f: impl Fn(&Event) -> bool) -> bool {
        self.0.iter().any(f)
    }
    fn effect(&self) -> Effect {
        self.0
            .iter()
            .rev()
            .find_map(|x| match x {
                Event::Effect { effect, .. } => Some(*effect),
                _ => None,
            })
            .unwrap_or(Effect::None)
    }
    fn applied(&self) -> [Option<u8>; 2] {
        self.0
            .iter()
            .rev()
            .find_map(|x| match x {
                Event::Effect { applied, .. } => Some(*applied),
                _ => None,
            })
            .unwrap_or([None; 2])
    }
    fn safe(&self, n: usize) -> bool {
        match n {
            0 => self.0.iter().all(|x| match x {
                Event::Store {
                    before,
                    after,
                    old,
                    new,
                } => (before != after) == (old != new),
                _ => true,
            }),
            1 => {
                let mut revisions = vec![0];
                self.0.iter().all(|x| match x {
                    Event::Store { old, new, .. } if old != new => {
                        let fresh = !revisions.contains(new);
                        revisions.push(*new);
                        fresh
                    }
                    _ => true,
                })
            }
            2 => self.0.iter().all(|x| match x {
                Event::End {
                    control: true,
                    answer: Answer::Succeeded,
                    source: "reply",
                    patch,
                    applied,
                    before,
                    after,
                    retired,
                    old,
                    new,
                    ..
                } => {
                    applied == patch
                        && if *retired {
                            before == after && old == new
                        } else {
                            merge(*before, *patch) == *after
                        }
                }
                _ => true,
            }),
            3 => self.0.iter().all(|x| match x {
                Event::End {
                    control: true,
                    answer: Answer::Unknown,
                    before,
                    after,
                    old,
                    new,
                    patch,
                    unknown,
                    ..
                } => before == after && old == new && *unknown == patch.map(|v| v.is_some()),
                _ => true,
            }),
            4 => self.0.iter().all(|x| match x {
                Event::Reconcile {
                    occurred,
                    retired,
                    removed,
                    named,
                    patch,
                    before,
                    after,
                    old,
                    new,
                    answer,
                    outcome,
                    unknown,
                    ..
                } => {
                    if *answer == Answer::Succeeded && *unknown != [false; 2] {
                        return false;
                    }
                    if *retired || *removed || !occurred {
                        before == after
                            && old == new
                            && (*answer != Answer::Succeeded
                                || *outcome
                                    == if *occurred {
                                        Answer::Succeeded
                                    } else {
                                        Answer::Failed
                                    })
                    } else if !valid(merge(*before, *patch)) {
                        true
                    } else if named.is_none() {
                        *answer == Answer::Invalid
                            && before == after
                            && old == new
                            && *outcome == Answer::Unknown
                    } else if *named != Some(*old) {
                        *answer == Answer::Conflict
                            && before == after
                            && old == new
                            && *outcome == Answer::Unknown
                    } else {
                        *answer == Answer::Succeeded
                            && *outcome == Answer::Succeeded
                            && *after == merge(*before, *patch)
                    }
                }
                _ => true,
            }),
            5 => self.0.iter().all(|x| match x {
                Event::Reconcile {
                    occurred: true,
                    retired: false,
                    removed: false,
                    patch,
                    before,
                    after,
                    old,
                    new,
                    answer,
                    outcome,
                    unknown,
                    ..
                } if !valid(merge(*before, *patch)) => {
                    *answer == Answer::Invalid
                        && *outcome == Answer::Unknown
                        && before == after
                        && old == new
                        && *unknown == patch.map(|v| v.is_some())
                }
                _ => true,
            }),
            6 => self.0.iter().all(|x| match x {
                Event::Contender {
                    kind,
                    busy: true,
                    answer,
                    dispatched,
                    queued,
                } => {
                    !dispatched
                        && !queued
                        && *answer
                            == match kind {
                                Kind::SucceededRepeat => Answer::Succeeded,
                                Kind::Refused => Answer::Refused,
                                Kind::Invalid => Answer::Invalid,
                                Kind::Repeat => Answer::Conflict,
                                _ => Answer::Conflict,
                            }
                }
                _ => true,
            }),
            7 => self.0.iter().all(|x| match x {
                Event::End {
                    answer, received, ..
                } => {
                    matches!(received, Some(Reply::Refused | Reply::Unavailable))
                        || matches!(answer, Answer::Succeeded | Answer::Failed | Answer::Unknown)
                }
                _ => true,
            }),
            8 => self.0.iter().all(|x| match x {
                Event::Report {
                    effect,
                    applied,
                    sent,
                    uncertain,
                    reply,
                } => {
                    (!(*uncertain || *effect == Effect::Partial) || *reply == Reply::Unknown)
                        && match reply {
                            Reply::Succeeded => *effect == Effect::Full && applied == sent,
                            Reply::Failed => *effect == Effect::None,
                            _ => true,
                        }
                }
                _ => true,
            }),
            9 => self.0.iter().all(|x| match x {
                Event::Contender {
                    dispatched: false,
                    answer,
                    kind,
                    ..
                } => *kind == Kind::SucceededRepeat || answer.reason(),
                Event::End {
                    received: Some(Reply::Refused | Reply::Unavailable),
                    answer,
                    ..
                } => answer.reason(),
                _ => true,
            }),
            10 => self.0.iter().all(|x| match x {
                Event::BadResult { answer } => *answer == Answer::Unknown,
                Event::End {
                    source: "timer" | "recovery",
                    answer,
                    ..
                } => *answer == Answer::Unknown,
                _ => true,
            }),
            11 => {
                // Confirmation is a received implementation fact preceding the end,
                // independent of the engine's chosen answer. A late reply cannot confirm it.
                let mut confirmed = false;
                self.0.iter().all(|x| match x {
                    Event::Received {
                        reply: Reply::Failed,
                        in_progress: true,
                    } => {
                        confirmed = true;
                        true
                    }
                    Event::Cancel { forwarded } => *forwarded,
                    Event::CancelEnd { answer } => {
                        *answer
                            == if confirmed {
                                Answer::Failed
                            } else {
                                Answer::Unknown
                            }
                    }
                    _ => true,
                })
            }
            12 => self.0.iter().all(|x| match x {
                Event::Repeat {
                    case,
                    answer,
                    outputs,
                    dispatched,
                    invocation,
                } => {
                    !dispatched
                        && !outputs
                        && *invocation == Some(*case)
                        && *answer
                            == if *case == 1 {
                                Answer::Succeeded
                            } else {
                                Answer::Conflict
                            }
                }
                _ => true,
            }),
            13 => self.0.iter().all(|x| match x {
                Event::Implicit { work, .. } => !work,
                Event::Work {
                    marked,
                    open,
                    listed,
                    invocation,
                } => {
                    *marked
                        && *open
                        && *listed
                        && match invocation {
                            Some(21) => self.has(|e| {
                                matches!(
                                    e,
                                    Event::Send {
                                        src: E,
                                        dst: I,
                                        message: Message::WorkCall(true)
                                    }
                                )
                            }),
                            Some(22) => self.has(|e| {
                                matches!(
                                    e,
                                    Event::Send {
                                        src: E,
                                        dst: I,
                                        message: Message::WorkLeaf
                                    }
                                )
                            }),
                            _ => false,
                        }
                }
                _ => true,
            }),
            14 => self.0.iter().all(|x| match x {
                Event::Onward {
                    marked,
                    dispatched,
                    answer,
                } => {
                    if *marked {
                        *dispatched
                    } else {
                        !dispatched && *answer == Answer::Refused
                    }
                }
                _ => true,
            }),
            15 => self.0.iter().all(|x| match x {
                Event::Reconcile {
                    occurred: false,
                    answer: Answer::Succeeded,
                    fenced,
                    ..
                } => *fenced,
                _ => true,
            }),
            16 => !self.has(|x| {
                matches!(
                    x,
                    Event::Effect {
                        after_absent: true,
                        ..
                    }
                )
            }),
            17 => self.0.iter().all(|x| match x {
                Event::Late { before, after, .. } => before == after,
                _ => true,
            }),
            _ => unreachable!(),
        }
    }
    fn witness(&self, n: usize) -> bool {
        match n {
            0 => self.has(|x| matches!(x, Event::Store { before, after, .. } if before != after)),
            1 => self.has(|x| matches!(x, Event::Store { before, after, .. } if before == after)),
            2 => { let stores: Vec<_> = self.0.iter().filter_map(|x| match x { Event::Store { after, .. } => Some(*after), _ => None }).collect(); stores.iter().enumerate().any(|(i, x)| *x == INITIAL && stores[..i].iter().any(|v| *v != INITIAL)) },
            3 => self.has(|x| matches!(x, Event::End { control: true, answer: Answer::Succeeded, .. })),
            4 => self.has(|x| matches!(x, Event::End { control: true, answer: Answer::Unknown, .. })),
            5 | 6 => self.has(|x| matches!(x, Event::Reconcile { occurred, answer: Answer::Succeeded, .. } if *occurred == (n == 5))),
            7 => self.has(|x| matches!(x, Event::Reconcile { occurred: true, named: Some(r), old, answer: Answer::Conflict, .. } if r != old)),
            8 => self.has(|x| matches!(x, Event::Reconcile { occurred: true, retired: false, removed: false, named: None, answer: Answer::Invalid, patch, before, .. } if valid(merge(*before, *patch)))),
            9 => self.has(|x| matches!(x, Event::LaterChange)) && self.has(|x| matches!(x, Event::Reconcile { occurred: true, patch, before, old, named: Some(r), answer: Answer::Invalid, .. } if old == r && !valid(merge(*before, *patch)))),
            10 => self.has(|x| matches!(x, Event::Contender { kind: Kind::Change, busy: true, .. })),
            11 => self.has(|x| matches!(x, Event::Retired)),
            12 => self.has(|x| matches!(x, Event::End { retired: true, answer: Answer::Succeeded, .. })),
            13 => self.has(|x| matches!(x, Event::Reconcile { retired: true, removed: false, answer: Answer::Succeeded, .. })),
            14 => self.has(|x| matches!(x, Event::Reconcile { removed: true, answer: Answer::Succeeded, .. })),
            15 => self.has(|x| matches!(x, Event::Report { reply: Reply::Succeeded, .. })),
            16 => self.has(|x| matches!(x, Event::Report { reply: Reply::Failed, .. })),
            17 => self.has(|x| matches!(x, Event::Report { effect: Effect::None, reply: Reply::Unknown, .. })),
            18 => self.has(|x| matches!(x, Event::Effect { effect: Effect::Partial, .. })),
            19 => self.has(|x| matches!(x, Event::Report { reply, .. } if reply.malformed())),
            20 => self.has(|x| matches!(x, Event::CancelEnd { answer: Answer::Unknown, .. })),
            21 => self.0.iter().enumerate().any(|(i, x)| matches!(x, Event::CancelEnd { answer: Answer::Failed }) && self.0[..i].iter().any(|x| matches!(x, Event::Received { reply: Reply::Failed, in_progress: true }))),
            22 => self.has(|x| matches!(x, Event::Cancel { .. })) && self.has(|x| matches!(x, Event::Late { confirmation: true, .. })),
            23..=25 => self.has(|x| matches!(x, Event::Repeat { case, .. } if *case as usize == n - 23)),
            26 => (0..14).all(|kind| self.has(|x| matches!(x, Event::Implicit { kind: k, .. } if *k == kind))),
            27 | 28 => self.has(|x| matches!(x, Event::Onward { marked, .. } if *marked == (n == 27))),
            30 => self.has(|x| matches!(x, Event::LaterChange)) && self.has(|x| matches!(x, Event::Reconcile { occurred: true, retired: false, removed: false, named: Some(r), old, patch, before, answer: Answer::Succeeded, .. } if r == old && patch.contains(&None) && valid(merge(*before, *patch)) && merge(*before, *patch) != merge(INITIAL, *patch))),
            31 => self.has(|x| matches!(x, Event::Received { reply: Reply::Succeeded | Reply::Failed | Reply::Unknown, in_progress: true })),
            32 => self.0.iter().enumerate().any(|(i, x)| matches!(x, Event::Received { reply: Reply::Unknown, in_progress: true }) && self.0[..i].iter().any(|x| matches!(x, Event::Cancel { .. }))),
            _ => unreachable!(),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Local {
    Finish(Effect, Reply),
    Silence,
    Recover,
    StopRun,
    Retire,
    Remove,
    LaterChange,
    Reconcile(bool, u8),
    Suffix,
    CloseStream,
    RecordCancel,
    Script,
}
#[derive(Clone)]
pub struct Controls {
    pub variant: Variant,
    pub scene: Scene,
    pub part: Part,
}
impl Controls {
    fn patch(&self) -> [Option<u8>; 2] {
        [
            Some(1),
            (!matches!(self.scene, Scene::Schema | Scene::CurrentMerge)).then_some(1),
        ]
    }
    fn send(&self, c: &Config, s: &mut System<Self>, src: usize, dst: usize, message: Message) {
        s.ledger.0.push(Event::Send { src, dst, message });
        s.send(c, src, dst, message);
    }
    fn drained(s: &System<Self>) -> bool {
        s.links.iter().flatten().all(|m| {
            !matches!(
                m,
                Message::Contender(_)
                    | Message::Probe
                    | Message::Request
                    | Message::ClientResult(..)
            )
        })
    }
    fn store(&self, s: &mut System<Self>, after: [u8; 2]) {
        let d = &mut s.nodes[E].data;
        let before = d.stored;
        let old = d.revision;
        if before != after {
            d.stored = after;
            d.revision = match self.variant {
                Variant::StaleRevision => old,
                Variant::ReusedRevision => (d.next_revision - 1) % 2 + 1,
                _ => d.next_revision,
            };
            d.next_revision += 1;
        }
        s.ledger.0.push(Event::Store {
            before,
            after: d.stored,
            old,
            new: d.revision,
        });
    }
    fn end(
        &self,
        c: &Config,
        s: &mut System<Self>,
        answer: Answer,
        source: &'static str,
        received: Option<Reply>,
    ) {
        let before = s.nodes[E].data.stored;
        let old = s.nodes[E].data.revision;
        let retired = s.nodes[E].data.retired;
        let patch = self.patch();
        if self.scene.control()
            && !retired
            && (answer == Answer::Succeeded
                || (answer == Answer::Unknown && self.variant == Variant::StoreUnknown))
        {
            self.store(s, merge(before, patch));
        }
        let reported = if self.variant == Variant::DoubleOutcome && !answer.reason() {
            Answer::Double
        } else {
            answer
        };
        s.nodes[E].data.active = false;
        s.nodes[E].data.ended = Some(answer);
        s.nodes[E].data.unknown_mask = if answer == Answer::Unknown {
            patch.map(|v| v.is_some())
        } else {
            [false; 2]
        };
        if s.nodes[E].data.cancelling {
            s.ledger.0.push(Event::CancelEnd { answer });
        }
        s.ledger.0.push(Event::End {
            control: self.scene.control(),
            answer: reported,
            received,
            source,
            patch,
            applied: s.ledger.applied(),
            before,
            after: s.nodes[E].data.stored,
            old,
            new: s.nodes[E].data.revision,
            retired,
            unknown: s.nodes[E].data.unknown_mask,
        });
        if matches!(self.part, Part::LossReceipt | Part::ReorderReceipt) {
            self.send(c, s, E, A, Message::ClientResult(0, reported));
        }
    }
    fn recover_pending(&self, c: &Config, s: &mut System<Self>) {
        if s.nodes[E].data.recovery {
            s.nodes[E].data.recovery = false;
            if s.nodes[E].data.active {
                self.end(c, s, Answer::Unknown, "recovery", None);
            }
        }
    }
}
pub fn model(variant: Variant, scene: Scene, part: Part) -> Harness<Controls> {
    let mut budgets = [0; 9];
    budgets[Fault::Timer as usize] = 1;
    for f in match part {
        Part::LossCaller | Part::LossReceipt | Part::LossImplementation => vec![Fault::Loss],
        Part::ReorderCaller | Part::ReorderReceipt | Part::ReorderImplementation => {
            vec![Fault::Reorder]
        }
        Part::Crash => vec![Fault::Crash, Fault::Recovery],
        _ => vec![],
    } {
        budgets[f as usize] = 1;
    }
    Harness::new(
        Config {
            name: "MC7a split control faults",
            nodes: vec!["A", "B", "engine", "implementation"],
            budgets,
            disabled_reasons: std::array::from_fn(|i| {
                (budgets[i] == 0).then_some(match Fault::ALL[i] {
                    Fault::Duplication => {
                        "no duplicated dispatch transport; repeated requests are explicit"
                    }
                    Fault::Cut | Fault::Heal => {
                        "partition is outside this order; loss and delay are split"
                    }
                    Fault::LostMachine => {
                        "PLC-017 retirement is explicit; no permanent physical loss"
                    }
                    _ => "enabled in another named MC7a fault part",
                })
            }),
            messages_per_link: 7,
            unique_state_ceiling: 150_000,
            timeout: Duration::from_secs(9),
            bounds: "one control attempt, seven contention requests, two probes, one timer, one silence in Silence, one run-end fence, one actor reconciliation, three stored suffix changes; semantic scenes split; at most seven queued messages/link",
        },
        Controls {
            variant,
            scene,
            part,
        },
    )
}
impl Protocol for Controls {
    type Node = Data;
    type Message = Message;
    type Ledger = Ledger;
    type Local = Local;
    fn gate_remaining_faults(&self) -> bool {
        true
    }
    fn fault_allowed(&self, fault: Fault, src: usize, dst: usize) -> bool {
        match fault {
            Fault::Loss | Fault::Reorder => match self.part {
                Part::LossCaller | Part::ReorderCaller => matches!((src, dst), (A, E) | (B, E)),
                Part::LossReceipt | Part::ReorderReceipt => matches!((src, dst), (E, A) | (E, B)),
                Part::LossImplementation | Part::ReorderImplementation => {
                    matches!((src, dst), (E, I) | (I, E))
                }
                _ => false,
            },
            Fault::Crash | Fault::Recovery | Fault::Timer => src == E && dst == E,
            _ => false,
        }
    }
    fn fault_enabled(&self, s: &System<Self>, f: Fault) -> bool {
        match f {
            Fault::Crash => s.nodes[E].data.active && Self::drained(s),
            Fault::Loss | Fault::Reorder => s.nodes[E].data.stage < 2,
            _ => true,
        }
    }
    fn initial(&self, c: &Config) -> System<Self> {
        let mut s = State {
            nodes: (0..4)
                .map(|_| Node {
                    data: Data::default(),
                    status: Status::Running,
                })
                .collect(),
            links: vec![vec![]; 16],
            cut: Default::default(),
            used: [0; 9],
            ledger: Ledger(vec![], self.part, [None; 2], [None; 8]),
        };
        self.send(c, &mut s, A, E, Message::Request);
        self.send(c, &mut s, A, E, Message::Probe);
        s
    }
    fn receive(&self, c: &Config, s: &mut System<Self>, _: usize, dst: usize, msg: Message) {
        if dst == E {
            self.recover_pending(c, s);
        }
        match msg {
            Message::Request => {
                s.nodes[E].data.active = true;
                s.nodes[E].data.dispatched = true;
                s.nodes[E].data.stage = 1;
                s.ledger.0.push(Event::Dispatch {
                    patch: self.patch(),
                });
                self.send(c, s, E, I, Message::Dispatch);
                self.send(c, s, E, I, Message::Probe);
                for kind in [
                    Kind::Change,
                    Kind::Setup,
                    Kind::Retire,
                    Kind::Refused,
                    Kind::Invalid,
                    Kind::Repeat,
                    Kind::SucceededRepeat,
                ] {
                    self.send(c, s, B, E, Message::Contender(kind));
                }
            }
            Message::ClientResult(id, answer) => {
                s.ledger.3[id as usize] = Some(answer);
            }
            Message::Probe => {
                let accepted = if dst == E {
                    s.nodes[E].data.active
                } else {
                    s.nodes[I].data.job
                };
                // One immutable observation per inspection: unrelated ledger events commute.
                s.ledger.2[usize::from(dst == I)] = Some(accepted);
            }
            Message::Contender(kind) => {
                let busy = s.nodes[E].data.active;
                // The script's barrier keeps ordinary completion behind this entire batch.
                assert!(busy, "contention batch must precede completion/recovery");
                let answer = match kind {
                    Kind::SucceededRepeat => Answer::Succeeded,
                    Kind::Refused => Answer::Refused,
                    Kind::Invalid => Answer::Invalid,
                    _ => Answer::Conflict,
                };
                let answer = if self.variant == Variant::OutcomeForReason && kind == Kind::Refused {
                    Answer::Failed
                } else {
                    answer
                };
                let queued = self.variant == Variant::QueueContender && kind == Kind::Change;
                s.nodes[E].data.contender_queued |= queued;
                s.ledger.0.push(Event::Contender {
                    kind,
                    busy,
                    answer,
                    dispatched: false,
                    queued,
                });
                if matches!(self.part, Part::LossReceipt | Part::ReorderReceipt) {
                    self.send(c, s, E, B, Message::ClientResult(kind as u8 + 1, answer));
                }
            }
            Message::Dispatch if !s.nodes[I].data.stopped => s.nodes[I].data.job = true,
            Message::Dispatch => {}
            Message::Reply(reply) => {
                s.ledger.0.push(Event::Received {
                    reply,
                    in_progress: s.nodes[E].data.active,
                });
                s.nodes[E].data.fenced = true;
                s.ledger.0.push(Event::Fence("answer"));
                if s.nodes[E].data.active {
                    let mut answer = reply.answer();
                    if reply.malformed() && self.variant == Variant::TrustMalformed {
                        answer = Answer::Failed;
                    }
                    if s.nodes[E].data.cancelling && answer != Answer::Failed {
                        answer = Answer::Unknown;
                    }
                    if self.variant == Variant::ReasonForOutcome && reply == Reply::Succeeded {
                        answer = Answer::Conflict;
                    }
                    if self.variant == Variant::CancelUnknownFailed
                        && s.nodes[E].data.cancelling
                        && reply == Reply::Unknown
                    {
                        answer = Answer::Failed;
                    }
                    if reply.malformed() {
                        s.ledger.0.push(Event::BadResult { answer });
                    }
                    self.end(c, s, answer, "reply", Some(reply));
                } else {
                    let before = s.nodes[E].data.ended.unwrap();
                    if self.variant == Variant::LateConfirmation
                        && before == Answer::Unknown
                        && reply == Reply::Failed
                    {
                        s.nodes[E].data.ended = Some(Answer::Failed);
                    }
                    s.ledger.0.push(Event::Late {
                        before,
                        after: s.nodes[E].data.ended.unwrap(),
                        confirmation: reply == Reply::Failed,
                    });
                }
            }
            Message::Cancel => { /* Transport cancellation does not force the implementation to stop. */
            }
            Message::Reconcile { occurred, revision } => {
                let d = &s.nodes[E].data;
                let before = d.stored;
                let old = d.revision;
                let retired = d.retired;
                let removed = d.removed;
                let fenced = d.fenced;
                let patch = self.patch();
                let target = merge(before, patch);
                let answer = if !occurred && !fenced && self.variant != Variant::Unfenced {
                    Answer::Conflict
                } else if occurred
                    && !retired
                    && !removed
                    && ((!valid(target) && self.variant != Variant::AcceptInvalid)
                        || revision.is_none())
                {
                    Answer::Invalid
                } else if occurred && !retired && !removed && revision != Some(old) {
                    Answer::Conflict
                } else {
                    Answer::Succeeded
                };
                if answer == Answer::Succeeded {
                    if occurred && !retired && !removed {
                        self.store(
                            s,
                            if self.variant == Variant::WrongReconcile {
                                INITIAL
                            } else if self.variant == Variant::OriginalReconcile {
                                merge(INITIAL, patch)
                            } else {
                                target
                            },
                        );
                    }
                    s.nodes[E].data.ended = Some(if occurred {
                        Answer::Succeeded
                    } else {
                        Answer::Failed
                    });
                    s.nodes[E].data.unknown_mask = [false; 2];
                }
                s.ledger.0.push(Event::Reconcile {
                    occurred,
                    fenced,
                    retired,
                    removed,
                    named: revision,
                    patch,
                    before,
                    after: s.nodes[E].data.stored,
                    old,
                    new: s.nodes[E].data.revision,
                    answer,
                    outcome: s.nodes[E].data.ended.unwrap(),
                    unknown: s.nodes[E].data.unknown_mask,
                    faults: s.used,
                });
                s.nodes[E].data.stage = 3;
            }
            Message::Repeat(case) => {
                let mut answer = if case == 1 {
                    Answer::Succeeded
                } else {
                    Answer::Conflict
                };
                if self.variant == Variant::RepeatNoOutputsConflict && case == 1 {
                    answer = Answer::Conflict;
                }
                s.ledger.0.push(Event::Repeat {
                    case,
                    answer,
                    outputs: self.variant == Variant::LeakOutputs && case == 0,
                    dispatched: false,
                    invocation: Some(case),
                });
            }
            Message::WorkRequest {
                marked,
                open,
                listed,
            } => {
                if open && listed || self.variant == Variant::ClosedWork {
                    if marked && (!open || !listed) {
                        s.ledger.0.push(Event::Work {
                            marked,
                            open,
                            listed,
                            invocation: Some(20),
                        });
                    } else {
                        self.send(c, s, E, I, Message::WorkCall(marked));
                    }
                }
            }
            Message::WorkCall(marked) => {
                if marked {
                    s.ledger.0.push(Event::Work {
                        marked,
                        open: true,
                        listed: true,
                        invocation: Some(20 + u8::from(marked)),
                    });
                }
                self.send(c, s, I, E, Message::Onward(marked));
            }
            Message::WorkLeaf => s.ledger.0.push(Event::Work {
                marked: true,
                open: true,
                listed: true,
                invocation: Some(22),
            }),
            Message::Onward(marked) => {
                let dispatched = marked || self.variant == Variant::AllowOnward;
                s.ledger.0.push(Event::Onward {
                    marked,
                    dispatched,
                    answer: if dispatched {
                        Answer::Succeeded
                    } else {
                        Answer::Refused
                    },
                });
                if dispatched {
                    self.send(c, s, E, I, Message::WorkLeaf);
                }
            }
        }
    }
    fn timer_enabled(&self, s: &System<Self>, node: usize) -> bool {
        node == E && s.nodes[E].data.active && Self::drained(s) && !s.nodes[E].data.recovery
    }
    fn timer(&self, c: &Config, s: &mut System<Self>, _: usize) {
        self.end(c, s, Answer::Unknown, "timer", None);
    }
    fn crash(&self, d: &mut Data) {
        d.recovery = true;
    }
    fn recover(&self, _: &mut Data) {}
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        let d = &s.nodes[E].data;
        if node == I && s.nodes[I].data.job && Self::drained(s) {
            for (effect, reply) in [
                (Effect::Full, Reply::Succeeded),
                (Effect::None, Reply::Failed),
                (Effect::Partial, Reply::Unknown),
                (Effect::None, Reply::Unknown),
            ] {
                if !matches!(self.scene, Scene::Schema | Scene::CurrentMerge)
                    || effect != Effect::Partial
                {
                    out.push(Local::Finish(effect, reply));
                }
            }
            if self.scene == Scene::Outcomes {
                for reply in [
                    Reply::MissingTag,
                    Reply::BothTags,
                    Reply::WrongError,
                    Reply::InvalidOutput,
                ] {
                    out.push(Local::Finish(Effect::Full, reply));
                }
                for reply in [Reply::Refused, Reply::Unavailable] {
                    out.push(Local::Finish(Effect::None, reply));
                }
            }
            if self.part == Part::Silence && !s.nodes[I].data.silence_used {
                out.push(Local::Silence);
            }
        }
        if node != E {
            return;
        }
        if d.recovery {
            out.push(Local::Recover);
            return;
        }
        if d.active && Self::drained(s) {
            if matches!(self.scene, Scene::Retired | Scene::Removed) && !d.retired {
                out.push(Local::Retire);
            }
            if self.scene == Scene::Cancellation && !d.cancelling {
                out.push(Local::CloseStream);
            }
            if d.cancelling {
                out.push(Local::RecordCancel);
            }
        }
        if d.ended == Some(Answer::Unknown) && !d.fenced && !s.nodes[I].data.stopped {
            out.push(Local::StopRun);
        }
        if self.scene == Scene::Removed && d.retired && !d.removed && !d.active {
            out.push(Local::Remove);
        }
        if d.ended == Some(Answer::Unknown) && !d.recon_sent && self.scene.control() {
            let effect = s.ledger.effect();
            let quiet = s.links.iter().all(Vec::is_empty) && !s.nodes[I].data.job;
            if quiet && d.suffix == 0 && !d.retired {
                out.push(Local::LaterChange);
            }
            if effect != Effect::Partial
                && (!matches!(self.scene, Scene::Schema | Scene::CurrentMerge) || d.suffix > 0)
                && (self.scene != Scene::Removed || d.removed)
            {
                if effect == Effect::Full {
                    for revision in 0..3 {
                        out.push(Local::Reconcile(true, revision));
                    }
                } else {
                    out.push(Local::Reconcile(false, 0));
                }
            }
        }
        if d.ended.is_some() && s.links.iter().all(Vec::is_empty) && !s.nodes[I].data.job {
            if self.scene.control()
                && d.suffix < 3
                && (d.ended != Some(Answer::Unknown) || d.recon_sent)
                && !d.retired
            {
                out.push(Local::Suffix);
            }
            if !self.scene.control() && d.stage < 3 {
                out.push(Local::Script);
            }
        }
    }
    fn local_step(&self, c: &Config, s: &mut System<Self>, _: usize, action: Local) {
        match action {
            Local::Finish(effect, reply) => {
                s.nodes[I].data.job = false;
                let mut applied = match effect {
                    Effect::None => [None; 2],
                    Effect::Full => self.patch(),
                    Effect::Partial => [Some(1), None],
                };
                if self.variant == Variant::Substitute && effect == Effect::Full {
                    applied[0] = Some(0);
                }
                s.ledger.0.push(Event::Effect {
                    effect,
                    applied,
                    after_absent: effect != Effect::None
                        && s.ledger.has(|x| {
                            matches!(
                                x,
                                Event::Reconcile {
                                    occurred: false,
                                    answer: Answer::Succeeded,
                                    ..
                                }
                            )
                        }),
                });
                let reply = match self.variant {
                    Variant::FalseSuccess if effect == Effect::None && reply == Reply::Failed => {
                        Reply::Succeeded
                    }
                    Variant::PartialFailed if effect == Effect::Partial => Reply::Failed,
                    Variant::UncertainFailed
                        if effect == Effect::None && reply == Reply::Unknown =>
                    {
                        Reply::Failed
                    }
                    _ => reply,
                };
                let uncertain =
                    effect == Effect::None && matches!(action, Local::Finish(_, Reply::Unknown));
                s.ledger.0.push(Event::Report {
                    effect,
                    applied,
                    sent: self.patch(),
                    uncertain,
                    reply,
                });
                self.send(c, s, I, E, Message::Reply(reply));
            }
            Local::Silence => {
                s.nodes[I].data.job = false;
                s.nodes[I].data.silence_used = true;
                s.ledger.0.push(Event::Silence);
            }
            Local::Recover => self.recover_pending(c, s),
            Local::StopRun => {
                s.nodes[I].data.stopped = true;
                s.nodes[I].data.job = false;
                s.nodes[E].data.fenced = true;
                s.ledger.0.push(Event::Fence(
                    "run ended; in-flight messages address this ended run",
                ));
            }
            Local::Retire => {
                s.nodes[E].data.retired = true;
                s.ledger.0.push(Event::Retired);
            }
            Local::Remove => {
                s.nodes[E].data.removed = true;
                s.ledger.0.push(Event::Removed);
            }
            Local::LaterChange => {
                self.store(
                    s,
                    if self.scene == Scene::CurrentMerge {
                        [0, 1]
                    } else {
                        [0, 0]
                    },
                );
                s.nodes[E].data.suffix = 1;
                s.ledger.0.push(Event::LaterChange);
            }
            Local::Reconcile(occurred, selector) => {
                s.nodes[E].data.recon_sent = true;
                let d = &s.nodes[E].data;
                let revision = if !occurred || d.retired || d.removed {
                    None
                } else {
                    match selector {
                        0 => Some(d.revision),
                        1 => Some(99),
                        _ => None,
                    }
                };
                self.send(c, s, B, E, Message::Reconcile { occurred, revision });
            }
            Local::Suffix => {
                let stage = s.nodes[E].data.suffix;
                self.store(s, if stage < 2 { [0, 0] } else { INITIAL });
                s.nodes[E].data.suffix += 1;
            }
            Local::CloseStream => {
                s.nodes[E].data.cancelling = true;
                let forwarded = self.variant != Variant::DropCancellation;
                s.ledger.0.push(Event::Cancel { forwarded });
                if forwarded {
                    self.send(c, s, E, I, Message::Cancel);
                }
            }
            Local::RecordCancel => self.end(
                c,
                s,
                if self.variant == Variant::CancelFailed {
                    Answer::Failed
                } else {
                    Answer::Unknown
                },
                "cancel",
                None,
            ),
            Local::Script => {
                let stage = s.nodes[E].data.stage;
                if self.scene == Scene::Repeats {
                    if s.nodes[E].data.suffix < 3 {
                        let case = s.nodes[E].data.suffix;
                        s.nodes[E].data.suffix += 1;
                        s.ledger.0.push(Event::SeedSucceeded {
                            invocation: case,
                            outputs: case != 1,
                        });
                        self.send(c, s, A, E, Message::Repeat(case));
                    } else {
                        s.nodes[E].data.stage = 3;
                    }
                } else if self.scene == Scene::ModelWork {
                    let step = s.nodes[E].data.suffix;
                    if step < 14 {
                        let work = self.variant == Variant::ImplicitWork;
                        s.ledger.0.push(Event::Implicit { kind: step, work });
                        if work {
                            s.ledger.0.push(Event::Work {
                                marked: false,
                                open: false,
                                listed: false,
                                invocation: None,
                            });
                        }
                    } else if step < 18 {
                        let (marked, open, listed) = match step {
                            14 => (false, true, true),
                            15 => (true, true, true),
                            16 => (true, false, true),
                            _ => (true, true, false),
                        };
                        self.send(
                            c,
                            s,
                            A,
                            E,
                            Message::WorkRequest {
                                marked,
                                open,
                                listed,
                            },
                        );
                    } else {
                        s.nodes[E].data.stage = 3;
                    }
                    s.nodes[E].data.suffix += 1;
                } else {
                    assert!(stage < 3);
                    s.nodes[E].data.stage = 3;
                }
            }
        }
    }
    fn properties(&self) -> Vec<NamedProperty<Self>> {
        type Condition = fn(&Harness<Controls>, &System<Controls>) -> bool;
        let safety: [Condition; 18] = [
            |_, s| s.ledger.safe(0),
            |_, s| s.ledger.safe(1),
            |_, s| s.ledger.safe(2),
            |_, s| s.ledger.safe(3),
            |_, s| s.ledger.safe(4),
            |_, s| s.ledger.safe(5),
            |_, s| s.ledger.safe(6),
            |_, s| s.ledger.safe(7),
            |_, s| s.ledger.safe(8),
            |_, s| s.ledger.safe(9),
            |_, s| s.ledger.safe(10),
            |_, s| s.ledger.safe(11),
            |_, s| s.ledger.safe(12),
            |_, s| s.ledger.safe(13),
            |_, s| s.ledger.safe(14),
            |_, s| s.ledger.safe(15),
            |_, s| s.ledger.safe(16),
            |_, s| s.ledger.safe(17),
        ];
        let ids: [&[&str]; 18] = [
            &["REV-003", "REV-004"],
            &["REV-005"],
            &["REV-007"],
            &["REV-009"],
            &["REV-010", "REV-002", "REV-011"],
            &["REV-013"],
            &["REV-014"],
            &["OPS-001"],
            &["OPS-002", "OPS-003", "OPS-004"],
            &["OPS-005"],
            &["OPS-007"],
            &["OPS-027"],
            &["OPS-034", "OPS-009"],
            &["IMP-001", "IMP-002"],
            &["IMP-008"],
            &["OPS-014"],
            &["OPS-014"],
            &["OPS-013"],
        ];
        let mut props: Vec<_> = SAFETY
            .into_iter()
            .zip(safety)
            .zip(ids)
            .map(|((name, condition), promises)| NamedProperty {
                property: Property::always(name, condition),
                promises,
            })
            .collect();
        let witnesses: [Condition; 33] = [
            |_, s| s.ledger.witness(0),
            |_, s| s.ledger.witness(1),
            |_, s| s.ledger.witness(2),
            |_, s| s.ledger.witness(3),
            |_, s| s.ledger.witness(4),
            |_, s| s.ledger.witness(5),
            |_, s| s.ledger.witness(6),
            |_, s| s.ledger.witness(7),
            |_, s| s.ledger.witness(8),
            |_, s| s.ledger.witness(9),
            |_, s| s.ledger.witness(10),
            |_, s| s.ledger.witness(11),
            |_, s| s.ledger.witness(12),
            |_, s| s.ledger.witness(13),
            |_, s| s.ledger.witness(14),
            |_, s| s.ledger.witness(15),
            |_, s| s.ledger.witness(16),
            |_, s| s.ledger.witness(17),
            |_, s| s.ledger.witness(18),
            |_, s| s.ledger.witness(19),
            |_, s| s.ledger.witness(20),
            |_, s| s.ledger.witness(21),
            |_, s| s.ledger.witness(22),
            |_, s| s.ledger.witness(23),
            |_, s| s.ledger.witness(24),
            |_, s| s.ledger.witness(25),
            |_, s| s.ledger.witness(26),
            |_, s| s.ledger.witness(27),
            |_, s| s.ledger.witness(28),
            |_, s| {
                s.ledger.has(|e| match e {
                    Event::Reconcile { faults, .. } => match s.ledger.1 {
                        Part::Base => true,
                        Part::LossCaller | Part::LossReceipt | Part::LossImplementation => {
                            faults[Fault::Loss as usize] > 0
                        }
                        Part::ReorderCaller
                        | Part::ReorderReceipt
                        | Part::ReorderImplementation => faults[Fault::Reorder as usize] > 0,
                        Part::Crash => faults[Fault::Recovery as usize] > 0,
                        Part::Silence => s.ledger.has(|x| matches!(x, Event::Silence)),
                    },
                    _ => false,
                })
            },
            |_, s| s.ledger.witness(30),
            |_, s| s.ledger.witness(31),
            |_, s| s.ledger.witness(32),
        ];
        props.extend(
            WITNESSES
                .into_iter()
                .zip(witnesses)
                .map(|(name, condition)| NamedProperty {
                    property: Property::sometimes(name, condition),
                    promises: &["harness"],
                }),
        );
        if self.part == Part::ReorderCaller {
            props.push(NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "MC7a reach: caller inspection overtook control",
                    |_, s| s.ledger.2[0] == Some(false),
                ),
                promises: &["harness"],
            });
        }
        if self.part == Part::ReorderImplementation {
            props.push(NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "MC7a reach: implementation inspection overtook dispatch",
                    |_, s| s.ledger.2[1] == Some(false),
                ),
                promises: &["harness"],
            });
        }
        if self.part == Part::LossReceipt {
            props.push(NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "MC7a reach: succeeded receipt lost while success stays saved",
                    |_, s| {
                        s.ledger.3[0].is_none()
                            && s.ledger.has(|e| {
                                matches!(
                                    e,
                                    Event::End {
                                        answer: Answer::Succeeded,
                                        ..
                                    }
                                )
                            })
                            && s.ledger
                                .0
                                .iter()
                                .filter(|e| matches!(e, Event::Store { .. }))
                                .count()
                                >= 4
                    },
                ),
                promises: &["harness"],
            });
        }
        if matches!(self.part, Part::LossReceipt | Part::ReorderReceipt) {
            props.push(NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "MC7a reach: caller receives primary result",
                    |_, s| s.ledger.3[0].is_some(),
                ),
                promises: &["harness"],
            });
        }
        props
    }
}

/// Every full run in the registered matrix; mutants run only where their defect is needed.
pub fn runs() -> Vec<(Variant, Scene, Part)> {
    Part::ALL
        .into_iter()
        .map(|p| (Variant::Correct, Scene::Control, p))
        .chain(
            [
                Scene::Schema,
                Scene::CurrentMerge,
                Scene::Retired,
                Scene::Removed,
                Scene::Outcomes,
                Scene::Cancellation,
                Scene::Repeats,
                Scene::ModelWork,
            ]
            .into_iter()
            .map(|s| (Variant::Correct, s, Part::Base)),
        )
        .chain(
            Variant::BROKEN
                .into_iter()
                .map(|v| (v, v.scene(), Part::Base)),
        )
        .collect()
}
/// Registered before execution. Properties not mentioned hold; reach indices are README w0..w32.
pub fn expected(v: Variant, scene: Scene) -> (Vec<&'static str>, Vec<&'static str>) {
    let mut broken = match v {
        Variant::Correct => vec![],
        Variant::Substitute => vec![EXACT, CLAIM],
        Variant::DoubleOutcome => vec![OUTCOME, MISSING],
        Variant::OutcomeForReason => vec![REASON, CONTENTION],
        Variant::Unfenced => vec![FENCE, ABSENT],
        _ => vec![v.target()],
    };
    let mut reached = vec![10, 15, 16, 17, 18, 31];
    if scene.control() {
        reached.extend([0, 1, 2, 3, 4, 5, 6, 7, 8, 29]);
    }
    match scene {
        Scene::Schema => {
            reached.retain(|n| ![5, 7, 8, 18].contains(n));
            reached.push(9);
        }
        Scene::CurrentMerge => {
            reached.retain(|n| *n != 18);
            reached.push(30);
        }
        Scene::Retired => reached.extend([11, 12, 13]),
        Scene::Removed => {
            reached.retain(|n| ![7, 8].contains(n));
            reached.extend([11, 12, 14]);
        }
        Scene::Outcomes => reached.push(19),
        Scene::Cancellation => reached.extend([20, 21, 22, 32]),
        Scene::Repeats => reached.extend([23, 24, 25]),
        Scene::ModelWork => reached.extend([26, 27, 28]),
        _ => {}
    }
    match v {
        Variant::UncertainFailed => reached.retain(|n| *n != 17),
        Variant::FalseSuccess => reached.retain(|n| *n != 16),
        Variant::AcceptInvalid => reached.retain(|n| *n != 9),
        _ => {}
    }
    let mut missing: Vec<_> = WITNESSES
        .into_iter()
        .enumerate()
        .filter_map(|(i, name)| (!reached.contains(&i)).then_some(name))
        .collect();
    broken.sort();
    missing.sort();
    (broken, missing)
}
#[cfg(test)]
mod tests {
    use super::*;
    use stateright::Model;
    #[test]
    fn full_runs_match_registered_table() {
        let mut differences = vec![];
        for (v, scene, part) in runs() {
            let report = check_all(model(v, scene, part));
            println!("MC7a {v:?}/{scene:?}/{part:?}: {report}");
            let mut observed = (report.broken, report.missing_witnesses);
            observed.0.sort();
            observed.1.sort();
            let registered = expected(v, scene);
            let mut recorded = registered.clone();
            if v == Variant::AcceptInvalid {
                // MC7a-001: skipping the schema check also reaches occurred and stale conflict.
                recorded
                    .1
                    .retain(|name| ![WITNESSES[5], WITNESSES[7]].contains(name));
            }
            if observed != registered {
                println!(
                    "MC7a recorded difference {v:?}/{scene:?}/{part:?}: reaches w5 and w7 in addition to registration"
                );
            }
            if observed != recorded {
                println!(
                    "MC7a difference {v:?}/{scene:?}/{part:?}: registered {registered:?}; observed {observed:?}"
                );
                differences.push(format!("{v:?}/{scene:?}/{part:?}"));
            }
        }
        assert!(
            differences.is_empty(),
            "unrecorded MC7a differences {differences:?}"
        );
    }
    #[test]
    fn first_refutations_and_ledger_independence() {
        for v in Variant::BROKEN {
            println!("MC7a first refutation {v:?}: {}", v.target());
            let m = model(v, v.scene(), Part::Base);
            let trace = assert_refuted(m.clone(), v.target());
            println!("MC7a first trace {v:?}: {} actions", trace.len());
            let mut s = stateright::Path::from_actions(&m, m.init_states().remove(0), &trace)
                .unwrap()
                .last_state()
                .clone();
            s.nodes.iter_mut().for_each(|n| n.data = Data::default());
            s.links.iter_mut().for_each(Vec::clear);
            assert!(!(m.property(v.target()).condition)(&m, &s));
        }
    }
    #[test]
    fn metadata_and_fault_windows() {
        for (v, scene, part) in runs() {
            model(v, scene, part).config.validate();
        }
        for name in SAFETY {
            assert!(
                Variant::BROKEN
                    .into_iter()
                    .any(|v| expected(v, v.scene()).0.contains(&name))
            );
        }
        let m = model(Variant::Correct, Scene::Control, Part::Crash);
        let s = m.init_states().remove(0);
        let mut actions = vec![];
        m.actions(&s, &mut actions);
        assert!(
            !actions
                .iter()
                .any(|a| matches!(a, Action::Crash(_) | Action::Timer(_)))
        );
        for src in 0..4 {
            for dst in 0..4 {
                for fault in [Fault::Crash, Fault::Recovery, Fault::Timer] {
                    assert_eq!(
                        m.protocol.fault_allowed(fault, src, dst),
                        src == E && dst == E
                    );
                }
            }
        }
    }
}
