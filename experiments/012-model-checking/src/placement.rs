//! MC4: placement and provider obligations, read from the promises, not engine code.
use crate::harness::*;
use stateright::{Model, Property};
use std::time::Duration;

const E: usize = 0;
const M: usize = 1;
const A: usize = 2;
pub const ONE: &str = "PRV-007 PRV-010 PRV-014: at most one live run";
pub const UNREACH: &str = "PLC-023: unreachable is not gone";
pub const LOST: &str = "PLC-017 PLC-024 PLC-027: lost shutdown unknown";
pub const STOPS: &str = "PRV-008 PRV-021 PRV-027: stops kept and addressed";
pub const PLACED: &str = "PLC-002 PRV-015 PRV-019: runs only where placed";
pub const REPORT: &str = "PRV-009: status rests on a true report";
pub const READY: &str = "PLC-019: setup waits for a ready node";
pub const NEW: &str = "PLC-026: returned machine gets a new node";
pub const SERVERS: &str = "PRV-002: provider servers end with engine";
const RETURN: &str = "MC4: unreachable then reachable";
const GONE: &str = "MC4: provider gone";
const ROOT: &str = "MC4: root declaration";
const LATE: &str = "MC4: late stop";
const CUT_REQUEST: &str = "MC4: unavailable request during cut";
const RESTART: &str = "MC4: run exits and restarts";
const REPLACEMENT: &str = "MC4: replacement placed";
const NEW_NODE: &str = "MC4: returned machine new node";
const UNKNOWN: &str = "MC4: unknown run prevents run call";
const NOT_READY: &str = "MC4: setup unavailable while registered";
const RETIRED_KEEPER: &str = "MC4: stop carried after provider retirement";
#[cfg(test)]
const SAFETY: [&str; 9] = [
    ONE, UNREACH, LOST, STOPS, PLACED, REPORT, READY, NEW, SERVERS,
];
const WITNESSES: [&str; 11] = [
    RETURN,
    GONE,
    ROOT,
    LATE,
    CUT_REQUEST,
    RESTART,
    REPLACEMENT,
    NEW_NODE,
    UNKNOWN,
    NOT_READY,
    RETIRED_KEEPER,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Part {
    Base,
    Partition,
    PartitionReply,
    PartitionRestartRace,
    PartitionReplyRestartRace,
    PartitionRetire,
    Combined,
    CombinedRestartRace,
    LossA,
    LossM2,
    ReorderA,
    ReorderM2,
    Gone,
    Declared,
}
impl Part {
    pub const ALL: [Self; 14] = [
        Self::Base,
        Self::Partition,
        Self::PartitionReply,
        Self::PartitionRestartRace,
        Self::PartitionReplyRestartRace,
        Self::PartitionRetire,
        Self::Combined,
        Self::CombinedRestartRace,
        Self::LossA,
        Self::LossM2,
        Self::ReorderA,
        Self::ReorderM2,
        Self::Gone,
        Self::Declared,
    ];
}
impl Part {
    fn combined(self) -> bool {
        matches!(self, Self::Combined | Self::CombinedRestartRace)
    }
    fn consumer_partition(self) -> bool {
        matches!(
            self,
            Self::Partition
                | Self::PartitionReply
                | Self::PartitionRestartRace
                | Self::PartitionReplyRestartRace
        )
    }
    #[cfg(test)]
    fn reply_partition(self) -> bool {
        matches!(self, Self::PartitionReply | Self::PartitionReplyRestartRace)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Correct,
    UnreachableGone,
    UnknownRun,
    VolatileStop,
    WrongStop,
    ConfirmLost,
    LostHook,
    WrongMachine,
    StaleRunning,
    EarlySetup,
    ReuseNode,
    ConfirmLaterLost,
    BadSetupAnswer,
}
impl Variant {
    pub const BROKEN: [Self; 12] = [
        Self::UnreachableGone,
        Self::UnknownRun,
        Self::VolatileStop,
        Self::WrongStop,
        Self::ConfirmLost,
        Self::LostHook,
        Self::WrongMachine,
        Self::StaleRunning,
        Self::EarlySetup,
        Self::ReuseNode,
        Self::ConfirmLaterLost,
        Self::BadSetupAnswer,
    ];
    pub fn part(self) -> Part {
        match self {
            Self::UnreachableGone | Self::UnknownRun | Self::StaleRunning => Part::Partition,
            Self::VolatileStop => Part::Combined,
            Self::ConfirmLost | Self::ConfirmLaterLost | Self::LostHook | Self::ReuseNode => {
                Part::Declared
            }
            _ => Part::Base,
        }
    }
    pub fn target(self) -> &'static str {
        match self {
            Self::UnreachableGone => UNREACH,
            Self::UnknownRun => ONE,
            Self::VolatileStop | Self::WrongStop => STOPS,
            Self::ConfirmLost | Self::ConfirmLaterLost | Self::LostHook => LOST,
            Self::WrongMachine => PLACED,
            Self::StaleRunning => REPORT,
            Self::EarlySetup | Self::BadSetupAnswer => READY,
            Self::ReuseNode => NEW,
            Self::Correct => unreachable!(),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Health {
    #[default]
    Omitted,
    Reachable,
    Unreachable,
    Gone,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RunState {
    #[default]
    Unknown,
    Running,
    Stopped,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Answer {
    Outcome,
    Unavailable,
    Unknown,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Reason {
    Request,
    Root,
    Gone,
    Silence,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InspectFor {
    Setup,
    Retire,
    FinishRetire,
    Observe,
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Data {
    // Engine saved script and object state. Provider server fields are volatile.
    stage: u8,
    node: u8,
    node_ready: bool,
    node_lost: bool,
    lost_through: u8,
    starting: [bool; 3],
    retire_hook_pending: bool,
    recreated: bool,
    exists: [bool; 3],
    setup: [bool; 3],
    retired: [bool; 3],
    placements: [u8; 3],
    epoch: u8,
    providers: bool,
    recovered: bool,
    health: Health,
    seen: Option<(u8, Health)>,
    inspected: [Option<RunReport>; 3],
    inspect_pending: Option<(usize, InspectFor)>,
    inspect_answered: bool,
    request_ends: [Option<Answer>; 2],
    volatile_stops: [bool; 3],
    provider_retired: bool,
    // M2's provider part: durable obligations, saved independently of M1 servers.
    kept: [bool; 3],
    runs: [u8; 3],
    returned: bool,
    restart_pending: bool,
    // Consumer script sends at most two independent requests.
    sent: bool,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RunReport {
    instance: u8,
    value: RunState,
    truth: RunState,
    time: u8,
    epoch: u8,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Run(u8, u8),
    RunReply(u8),
    Request(u8),
    Dispatch(u8),
    Reply(u8),
    Answer(u8, Answer),
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Place {
        node: u8,
        machine: u8,
        selected: u8,
    },
    Setup {
        ready: bool,
        unreachable: bool,
        dispatched: bool,
        answer: Answer,
    },
    RunAsked {
        machine: u8,
        latest: RunState,
        unreachable: bool,
        lost: bool,
        hook: bool,
    },
    RunAnswer(Answer),
    Start {
        machine: u8,
        family_live: u8,
        root_risk: bool,
        restart: bool,
    },
    Exit,
    StopAsked {
        named: u8,
        machine: u8,
        answer: Answer,
        obligation: u8,
    },
    StopDone {
        named: u8,
        late: bool,
        obligation: u8,
        provider_retired: bool,
    },
    Retire {
        lost: bool,
        reason: Reason,
        setup: bool,
        hook: bool,
        confirmed: bool,
    },
    Hook {
        lost: bool,
        unreachable: bool,
        restart_only: bool,
    },
    Secret {
        unreachable: bool,
        lost: bool,
        restart_only: bool,
    },
    Inspect(RunReport),
    Status {
        value: RunState,
        report: Option<RunReport>,
        time: Option<u8>,
        unreachable: bool,
    },
    Request {
        id: u8,
        dispatched: bool,
        answer: Option<Answer>,
        unreachable: bool,
        cut: bool,
    },
    RequestAnswer(u8, Answer), // engine's immutable recorded end
    CallerAnswer(u8, Answer),
    ImplementationReply(u8),
    Removed,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MachineReport {
    value: Health,
    truth: Health,
    epoch: u8,
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Ledger {
    // Per-subject ordering preserves all relevant history and commutes independent events.
    histories: [Vec<Event>; 3],
    requests: [Vec<Event>; 2],
    machine: Vec<MachineReport>,
    nodes: Vec<(u8, bool)>, // (registered ID, already retired when registered)
    declarations: Vec<Reason>,
    lost_subjects: Vec<(Reason, u8, [bool; 3])>,
    retired_nodes: Vec<u8>,
    lifecycle: Vec<(bool, bool)>, // engine running, local provider servers running
    live: [u8; 3],
    obligations: u8,
    root_risk: bool,
    quiescent: bool,
    saw_cut: bool,
    saw_return: bool,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Local {
    Report(Health),
    Script(u8),
    Send,
    Exit,
    Restart,
    Carry(u8),
    Recover,
    ReturnMachine,
    ObserveServers,
    FinishRetire,
    InspectAnswer,
    UseInspect,
    SetupReturnedNode,
    RecreateAfterSilence,
    Quiescent,
}
#[derive(Clone)]
pub struct Placement {
    pub variant: Variant,
    pub part: Part,
}
pub fn configuration(part: Part) -> Config {
    let mut budgets = [0; 9];
    let faults: &[Fault] = match part {
        Part::Partition
        | Part::PartitionReply
        | Part::PartitionRestartRace
        | Part::PartitionReplyRestartRace
        | Part::PartitionRetire
        | Part::Declared => &[Fault::Cut, Fault::Heal],
        Part::Combined | Part::CombinedRestartRace => {
            &[Fault::Cut, Fault::Heal, Fault::Crash, Fault::Recovery]
        }
        Part::LossA | Part::LossM2 => &[Fault::Loss],
        Part::ReorderA | Part::ReorderM2 => &[Fault::Reorder],
        Part::Gone => &[Fault::LostMachine],
        _ => &[],
    };
    for f in faults {
        budgets[*f as usize] = 1;
    }
    Config {
        name: "MC4 two machines, split faults",
        nodes: vec!["engine+providers (M1)", "runs+keeper (M2)", "A (M1)"],
        budgets,
        disabled_reasons: std::array::from_fn(|i| {
            (budgets[i] == 0).then_some(match Fault::ALL[i] {
                Fault::Duplication => {
                    "provider calls sent once over local stdio; no wire duplication"
                }
                Fault::Timer => "reports and kept stops are untimed local actions",
                _ => "fault isolated in another named MC4 configuration",
            })
        }),
        messages_per_link: 6,
        unique_state_ceiling: 190_000,
        timeout: Duration::from_secs(10),
        bounds: "M1 engine and A; M2 runs; I,J,I-prime; at most two requests (one in consumer partition parts); one exit/restart; one node replacement; finite 17-step script; one of each enabled fault; six messages/link",
    }
}
pub fn model(variant: Variant, part: Part) -> Harness<Placement> {
    Harness::new(configuration(part), Placement { variant, part })
}
impl Placement {
    fn reachable(s: &System<Self>) -> bool {
        s.nodes[M].status == Status::Running && !s.cut.contains(&(E, M))
    }
    fn actual(s: &System<Self>) -> Health {
        if s.nodes[M].status == Status::Lost {
            Health::Gone
        } else if Self::reachable(s) {
            Health::Reachable
        } else {
            Health::Unreachable
        }
    }
    fn unavailable(s: &System<Self>) -> bool {
        s.nodes[E].data.health != Health::Reachable
    }
    fn event(s: &mut System<Self>, i: usize, event: Event) {
        match &event {
            Event::Request { id, .. }
            | Event::RequestAnswer(id, _)
            | Event::CallerAnswer(id, _)
            | Event::ImplementationReply(id) => s.ledger.requests[*id as usize].push(event),
            _ => s.ledger.histories[i].push(event),
        }
    }
    fn end_request(&self, c: &Config, s: &mut System<Self>, id: u8, answer: Answer) {
        if s.nodes[E].data.request_ends[id as usize].is_none() {
            s.nodes[E].data.request_ends[id as usize] = Some(answer);
            Self::event(s, 0, Event::RequestAnswer(id, answer));
            s.send(c, E, A, Message::Answer(id, answer));
        }
    }
    fn request_inspect(&self, s: &mut System<Self>, i: usize, purpose: InspectFor) {
        assert!(s.nodes[E].data.inspect_pending.is_none());
        s.nodes[E].data.inspect_pending = Some((i, purpose));
        s.nodes[E].data.inspect_answered = false;
    }
    fn inspect(&self, s: &mut System<Self>, i: usize) -> RunState {
        let value = if !Self::reachable(s) {
            RunState::Unknown
        } else if s.nodes[M].data.runs[i] > 0 {
            RunState::Running
        } else if s.nodes[E].data.starting[i] {
            RunState::Unknown
        } else {
            RunState::Stopped
        };
        let truth = if s.nodes[M].status != Status::Lost && s.nodes[M].data.runs[i] > 0 {
            RunState::Running
        } else {
            RunState::Stopped
        };
        let time = s.ledger.histories[i]
            .iter()
            .filter(|e| matches!(e, Event::Inspect(_)))
            .count() as u8
            + 1;
        let r = RunReport {
            instance: i as u8,
            value,
            truth,
            time,
            epoch: s.nodes[E].data.epoch,
        };
        s.nodes[E].data.inspected[i] = Some(r);
        Self::event(s, i, Event::Inspect(r));
        self.status(s, i);
        value
    }
    fn status(&self, s: &mut System<Self>, i: usize) {
        let unavailable = Self::unavailable(s);
        let r = s.nodes[E].data.inspected[i];
        let value = if unavailable && self.variant != Variant::StaleRunning {
            RunState::Unknown
        } else {
            r.map_or(RunState::Unknown, |r| r.value)
        };
        Self::event(
            s,
            i,
            Event::Status {
                value,
                report: r,
                time: r.map(|r| r.time),
                unreachable: unavailable,
            },
        );
    }
    fn ask_run(&self, c: &Config, s: &mut System<Self>, i: usize, hook: bool, latest: RunState) {
        let unavailable = Self::unavailable(s);
        let lost = s.nodes[E].data.placements[i] != 0
            && s.nodes[E].data.placements[i] <= s.nodes[E].data.lost_through;
        Self::event(
            s,
            i,
            Event::Secret {
                unreachable: unavailable,
                lost,
                restart_only: false,
            },
        );
        Self::event(
            s,
            i,
            Event::RunAsked {
                machine: 2,
                latest,
                unreachable: unavailable,
                lost,
                hook,
            },
        );
        s.nodes[E].data.starting[i] = true;
        s.send(c, E, M, Message::Run(i as u8, 2));
        if !Self::reachable(s) {
            Self::event(s, i, Event::RunAnswer(Answer::Unknown));
        }
    }
    fn setup(&self, _c: &Config, s: &mut System<Self>, i: usize) {
        let d = &s.nodes[E].data;
        if !d.exists[i] || d.retired[i] {
            return;
        }
        let ready = d.node_ready && d.placements[i] == d.node;
        let unavailable = Self::unavailable(s);
        let dispatched = !unavailable && (ready || self.variant == Variant::EarlySetup);
        Self::event(
            s,
            i,
            Event::Setup {
                ready,
                unreachable: unavailable,
                dispatched,
                answer: if !ready && self.variant == Variant::BadSetupAnswer {
                    Answer::Unknown
                } else if dispatched {
                    Answer::Outcome
                } else {
                    Answer::Unavailable
                },
            },
        );
        if dispatched {
            s.nodes[E].data.setup[i] = true;
            self.request_inspect(s, i, InspectFor::Setup);
        }
    }
    fn stop(&self, s: &mut System<Self>, i: usize) {
        if !s.nodes[E].data.setup[i] {
            return;
        }
        s.nodes[M].data.restart_pending = false;
        let named = if self.variant == Variant::WrongStop && i == 0 {
            1
        } else {
            i
        };
        let answer = if Self::reachable(s) && s.nodes[M].data.runs[named] == 0 {
            Answer::Outcome
        } else {
            Answer::Unknown
        };
        let obligation = s.ledger.obligations;
        s.ledger.obligations += 1;
        Self::event(
            s,
            i,
            Event::StopAsked {
                named: named as u8,
                machine: 2,
                answer,
                obligation,
            },
        );
        if answer == Answer::Unknown {
            if self.variant == Variant::VolatileStop {
                s.nodes[E].data.volatile_stops[named] = true;
            } else {
                s.nodes[M].data.kept[named] = true;
            }
        } else {
            Self::event(
                s,
                i,
                Event::StopDone {
                    named: named as u8,
                    late: false,
                    obligation,
                    provider_retired: s.nodes[E].data.provider_retired,
                },
            );
        }
    }
    fn retire(&self, _c: &Config, s: &mut System<Self>, i: usize, reason: Reason) {
        if !s.nodes[E].data.exists[i] {
            return;
        }
        let setup = s.nodes[E].data.setup[i];
        let hook = i == 0 || i == 2;
        let lost = s.nodes[E].data.placements[i] != 0
            && s.nodes[E].data.placements[i] <= s.nodes[E].data.lost_through;
        let unavailable = Self::unavailable(s);
        // PLC-023 holds back only what an instance on a node that represents the machine would
        // need. A lost instance's node represents no machine (PLC-026), so PLC-023 never covers
        // it: PLC-027 keeps its hook and that hook's run from being asked for, and its stop is
        // asked for all the same (PRV-008, PRV-027), whatever the machine's health.
        let held = !lost && unavailable;
        let mut confirmed = !setup;
        if setup && reason == Reason::Request && !held {
            if hook && !lost {
                self.request_inspect(s, i, InspectFor::Retire);
            } else {
                self.stop(s, i);
            }
        }
        if lost
            && setup
            && (self.variant == Variant::ConfirmLost
                || (self.variant == Variant::ConfirmLaterLost && reason == Reason::Request))
        {
            confirmed = true;
        }
        if lost && setup && hook && self.variant == Variant::LostHook {
            Self::event(
                s,
                i,
                Event::Hook {
                    lost,
                    unreachable: unavailable,
                    restart_only: false,
                },
            );
        }
        Self::event(
            s,
            i,
            Event::Retire {
                lost,
                reason,
                setup,
                hook,
                confirmed,
            },
        );
        s.nodes[E].data.retired[i] = true;
    }
    fn declare(&self, c: &Config, s: &mut System<Self>, reason: Reason) {
        if s.nodes[E].data.node == 0 || s.nodes[E].data.node_lost {
            return;
        }
        s.nodes[E].data.node_lost = true;
        s.nodes[E].data.lost_through = s.nodes[E].data.node;
        s.nodes[E].data.node_ready = false;
        if reason == Reason::Root {
            s.ledger.root_risk = true;
        }
        s.ledger.declarations.push(reason);
        s.ledger
            .lost_subjects
            .push((reason, s.nodes[E].data.node, s.nodes[E].data.exists));
        s.ledger.retired_nodes.push(s.nodes[E].data.node);
        for i in 0..3 {
            self.retire(c, s, i, reason);
        }
    }
    fn place(&self, s: &mut System<Self>, i: usize) {
        let d = &mut s.nodes[E].data;
        if d.exists[i] || d.node == 0 || d.node_lost || d.health != Health::Reachable {
            return;
        }
        d.exists[i] = true;
        d.placements[i] = d.node;
        let node = d.node;
        Self::event(
            s,
            i,
            Event::Place {
                node,
                machine: 2,
                selected: 2,
            },
        );
    }
    fn pending(s: &System<Self>, i: usize) -> bool {
        s.nodes[M].data.kept[i] || s.nodes[E].data.volatile_stops[i]
    }
    fn regular_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        let d = &s.nodes[E].data;
        if node == E {
            if d.recovered {
                out.push(Local::Recover);
                return;
            }
            if !d.provider_retired && d.seen != Some((d.epoch, Self::actual(s))) {
                out.push(Local::Report(Self::actual(s)));
                if Self::actual(s) == Health::Unreachable {
                    out.push(Local::Report(Health::Omitted));
                }
            }
            // Reports are not timers: the snapshot key changes only on physical change/restart.
            let waiting_run = s
                .links
                .iter()
                .flatten()
                .any(|m| matches!(m, Message::Run(..) | Message::RunReply(..)));
            let waiting_consumer = s
                .links
                .iter()
                .flatten()
                .any(|m| !matches!(m, Message::Run(..) | Message::RunReply(..)));
            let window_done = d.stage != 6
                || (s.nodes[A].data.sent && (!waiting_consumer || Self::unavailable(s)));
            let call_done = Self::unavailable(s) || !waiting_run;
            // Keep the healed consumer exchange in its own script phase, before exit/retire.
            let consumer_done =
                !self.part.consumer_partition() || d.stage != 8 || !waiting_consumer;
            let heal_stage = match self.part {
                Part::Combined | Part::CombinedRestartRace | Part::PartitionRetire => 13,
                Part::Declared => 15,
                _ => 7,
            };
            let in_window = (s.cut.is_empty() || d.stage != heal_stage)
                && !(s.used[Fault::Heal as usize] > 0
                    && d.seen != Some((d.epoch, Self::actual(s))));
            if d.inspect_pending.is_none()
                && d.retire_hook_pending
                && (!waiting_run || Self::unavailable(s))
            {
                out.push(Local::FinishRetire);
            }
            if d.node > 1 && !d.node_lost && !d.node_ready {
                out.push(Local::SetupReturnedNode);
            }
            let recreate = self.variant == Variant::UnreachableGone
                && d.node > 1
                && d.node_ready
                && d.inspect_pending.is_none()
                && !d.recreated
                && d.health == Health::Reachable;
            if recreate {
                out.push(Local::RecreateAfterSilence);
            }
            if d.inspect_pending.is_some() {
                out.push(if d.inspect_answered {
                    Local::UseInspect
                } else {
                    Local::InspectAnswer
                });
            }
            let retire_done = d.inspect_pending.is_none()
                && !d.retire_hook_pending
                && (d.stage != 12 || Self::unavailable(s) || !(0..3).any(|i| Self::pending(s, i)));
            if d.stage < 17
                && (d.stage != 0 || d.node > 0)
                && window_done
                && consumer_done
                && call_done
                && retire_done
                && in_window
                && (d.node <= 1 || d.node_lost || d.node_ready)
                && !recreate
            {
                out.push(Local::Script(d.stage));
            }
        }
        if node == A
            && s.ledger.lifecycle.last()
                != Some(&(s.nodes[E].status == Status::Running, d.providers))
        {
            out.push(Local::ObserveServers);
        }
        if node == A && d.stage == 6 && !s.nodes[A].data.sent {
            out.push(Local::Send);
        }
        if node == M {
            if d.stage == 8 && s.nodes[M].data.runs[0] > 0 {
                out.push(Local::Exit);
            }
            let restart_window = match self.part {
                Part::Partition | Part::PartitionReply | Part::Combined => d.stage == 9,
                Part::PartitionRestartRace
                | Part::PartitionReplyRestartRace
                | Part::CombinedRestartRace => {
                    d.retire_hook_pending
                        || (d.inspect_pending == Some((0, InspectFor::Retire))
                            && d.inspect_answered)
                }
                _ => true,
            };
            if restart_window
                && s.nodes[M].data.restart_pending
                && s.nodes[M].data.runs[0] == 0
                && !Self::pending(s, 0)
            {
                out.push(Local::Restart);
            }
            if Self::reachable(s) {
                for i in 0..3 {
                    if Self::pending(s, i) {
                        out.push(Local::Carry(i as u8));
                    }
                }
            }
        }
        // A new machine under the old hostname, not recovery of a permanently lost node.
        if node == E
            && s.nodes[M].status == Status::Lost
            && d.seen == Some((d.epoch, Health::Gone))
            && !s.nodes[M].data.returned
        {
            out.push(Local::ReturnMachine);
        }
    }
    fn all_idle(&self, s: &System<Self>) -> bool {
        if !Self::reachable(s) || s.links.iter().any(|q| !q.is_empty()) {
            return false;
        }
        let mut actions = vec![];
        for n in 0..3 {
            if s.nodes[n].status == Status::Running {
                self.regular_actions(s, n, &mut actions);
            }
        }
        if !actions.is_empty() {
            return false;
        }
        // A future fault action also prevents quiescence. Consult the same harness graph.
        let h = model(self.variant, self.part);
        let mut enabled = vec![];
        h.actions_without_quiescent(s, &mut enabled);
        enabled.is_empty()
    }
}
// Avoid recursively asking the harness for the Quiescent observation action itself.
impl Harness<Placement> {
    fn actions_without_quiescent(&self, s: &System<Placement>, out: &mut Vec<Action<Local>>) {
        let mut copy = s.clone();
        copy.ledger.quiescent = true;
        self.actions(&copy, out);
    }
}
impl Protocol for Placement {
    type Node = Data;
    type Message = Message;
    type Ledger = Ledger;
    type Local = Local;
    fn gate_remaining_faults(&self) -> bool {
        true
    }
    fn fault_enabled(&self, s: &System<Self>, fault: Fault) -> bool {
        let stage = s.nodes[E].data.stage;
        match fault {
            Fault::Cut => {
                let dispatched = s.ledger.requests.iter().flatten().any(|e| {
                    matches!(
                        e,
                        Event::Request {
                            dispatched: true,
                            ..
                        }
                    )
                });
                let phase = match self.part {
                    Part::Partition | Part::PartitionRestartRace => !dispatched,
                    Part::PartitionReply | Part::PartitionReplyRestartRace => dispatched,
                    _ => true,
                };
                phase
                    && stage
                        == if self.part == Part::Declared {
                            14
                        } else if matches!(
                            self.part,
                            Part::Combined | Part::CombinedRestartRace | Part::PartitionRetire
                        ) {
                            11
                        } else {
                            6
                        }
            }
            Fault::Heal => {
                stage
                    == match self.part {
                        Part::Combined | Part::CombinedRestartRace | Part::PartitionRetire => 13,
                        Part::Declared => 15,
                        _ => 7,
                    }
                    && (!self.part.combined() || s.used[Fault::Recovery as usize] > 0)
            }
            Fault::Crash => stage == 13 && !s.cut.is_empty(),
            Fault::LostMachine => stage == 10,
            _ => true,
        }
    }
    fn fault_allowed(&self, fault: Fault, src: usize, dst: usize) -> bool {
        let m2_link = (src == E && dst == M) || (src == M && dst == E);
        let a_link = (src == A && dst == E) || (src == E && dst == A);
        match fault {
            Fault::Cut | Fault::Heal => m2_link,
            Fault::Crash | Fault::Recovery => src == E && dst == E,
            Fault::LostMachine => src == M && dst == M,
            Fault::Loss => {
                if self.part == Part::LossA {
                    a_link
                } else {
                    m2_link
                }
            }
            Fault::Reorder => {
                if self.part == Part::ReorderA {
                    a_link
                } else {
                    m2_link
                }
            }
            Fault::Duplication | Fault::Timer => false,
        }
    }
    fn initial(&self, _: &Config) -> System<Self> {
        let mut s = State {
            nodes: (0..3)
                .map(|_| Node {
                    data: Data::default(),
                    status: Status::Running,
                })
                .collect(),
            links: vec![vec![]; 9],
            cut: Default::default(),
            used: [0; 9],
            ledger: Ledger::default(),
        };
        s.nodes[E].data.providers = true;
        s.ledger.lifecycle.push((true, true));
        s
    }
    fn receive(&self, c: &Config, s: &mut System<Self>, _: usize, dst: usize, msg: Message) {
        match msg {
            Message::Run(i, machine) => {
                let i = i as usize;
                let machine = if self.variant == Variant::WrongMachine {
                    1
                } else {
                    machine
                };
                s.nodes[M].data.runs[i] += 1;
                s.ledger.live[i] += 1;
                let family_live = s.ledger.live[0] + s.ledger.live[2];
                Self::event(
                    s,
                    i,
                    Event::Start {
                        machine,
                        family_live,
                        root_risk: s.ledger.root_risk,
                        restart: false,
                    },
                );
                s.send(c, M, E, Message::RunReply(i as u8));
            }
            Message::RunReply(i) => {
                Self::event(s, i as usize, Event::RunAnswer(Answer::Outcome));
                s.nodes[E].data.starting[i as usize] = false;
            }
            Message::Request(id) => {
                let unavailable = Self::unavailable(s);
                let dispatched = !unavailable
                    && s.nodes[E].data.exists[0]
                    && s.nodes[E].data.setup[0]
                    && !s.nodes[E].data.retired[0];
                Self::event(
                    s,
                    0,
                    Event::Request {
                        id,
                        dispatched,
                        answer: (!dispatched).then_some(Answer::Unavailable),
                        unreachable: unavailable,
                        cut: !s.cut.is_empty(),
                    },
                );
                if dispatched {
                    s.send(c, E, M, Message::Dispatch(id));
                } else {
                    self.end_request(c, s, id, Answer::Unavailable);
                }
            }
            Message::Dispatch(id) => {
                s.send(c, M, E, Message::Reply(id));
            }
            Message::Reply(id) => {
                Self::event(s, 0, Event::ImplementationReply(id));
                self.end_request(c, s, id, Answer::Outcome);
            }
            Message::Answer(id, answer) => {
                debug_assert_eq!(dst, A);
                Self::event(s, 0, Event::CallerAnswer(id, answer));
            }
        }
    }
    fn timer_enabled(&self, _: &System<Self>, _: usize) -> bool {
        false
    }
    fn timer(&self, _: &Config, _: &mut System<Self>, _: usize) {
        unreachable!()
    }
    fn crash(&self, d: &mut Data) {
        // Only M1's crash erases its volatile server state. M2 loss ends runs on return.
        d.providers = false;
        d.recovered = true;
        d.health = Health::Omitted;
        d.seen = None;
        d.inspected = [None; 3];
        d.inspect_answered = false;
        d.volatile_stops = [false; 3];
    }
    fn recover(&self, d: &mut Data) {
        d.providers = !d.provider_retired;
        d.epoch += 1;
    }
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        self.regular_actions(s, node, out);
        if node == E && !s.ledger.quiescent && self.all_idle(s) {
            out.push(Local::Quiescent);
        }
    }
    fn local_step(&self, c: &Config, s: &mut System<Self>, _: usize, action: Local) {
        match action {
            Local::Recover => {
                s.nodes[E].data.recovered = false;

                for i in 0..3 {
                    if s.nodes[E].data.exists[i] {
                        self.status(s, i);
                    }
                }
            }
            Local::Report(value) => {
                let truth = Self::actual(s);
                let before = s.nodes[E].data.health;
                s.nodes[E].data.seen = Some((s.nodes[E].data.epoch, truth));
                s.nodes[E].data.health = value;
                s.ledger.machine.push(MachineReport {
                    value,
                    truth,
                    epoch: s.nodes[E].data.epoch,
                });
                if matches!(value, Health::Unreachable | Health::Omitted) {
                    s.ledger.saw_cut = true;
                }
                if matches!(before, Health::Unreachable | Health::Omitted)
                    && value == Health::Reachable
                    && s.ledger.saw_cut
                {
                    s.ledger.saw_return = true;
                }
                if value == Health::Gone {
                    self.declare(c, s, Reason::Gone);
                } else if self.variant == Variant::UnreachableGone && value != Health::Reachable {
                    self.declare(c, s, Reason::Silence);
                } else if matches!(value, Health::Reachable | Health::Unreachable)
                    && (s.nodes[E].data.node == 0 || s.nodes[E].data.node_lost)
                {
                    let old = s.nodes[E].data.node;
                    let reuse = old != 0 && self.variant == Variant::ReuseNode;
                    let d = &mut s.nodes[E].data;
                    d.node = if reuse { old } else { old + 1 };
                    d.node_ready = false;
                    d.node_lost = false;
                    s.ledger.nodes.push((d.node, reuse));
                }
                for i in 0..3 {
                    if s.nodes[E].data.exists[i] {
                        self.status(s, i);
                    }
                }
                // A dispatched request that can no longer be observed ends unknown, not failed.
                if value != Health::Reachable {
                    for id in 0..2 {
                        if s.ledger.requests[id as usize].iter().any(|e| {
                            matches!(
                                e,
                                Event::Request {
                                    dispatched: true,
                                    ..
                                }
                            )
                        }) {
                            self.end_request(c, s, id, Answer::Unknown);
                        }
                    }
                }
            }
            Local::Script(stage) => {
                assert_eq!(stage, s.nodes[E].data.stage);
                match stage {
                    0 => self.place(s, 0),
                    1 => self.place(s, 1),
                    2 => self.setup(c, s, 0), // registered-node probe
                    3 => {
                        if !s.nodes[E].data.node_lost {
                            s.nodes[E].data.node_ready = true;
                        }
                    }
                    4 => self.setup(c, s, 0),
                    5 => self.setup(c, s, 1),
                    6 => self.setup(c, s, 0), // explicit further setup, including unknown inspection
                    7 => {
                        self.request_inspect(s, 0, InspectFor::Observe);
                    }
                    8 | 9 => {} // exit/restart may instead be skipped
                    10 => {}
                    11 => self.retire(c, s, 0, Reason::Request),
                    12 => {
                        if s.nodes[E].data.exists[0] {
                            self.stop(s, 0);
                            Self::event(s, 0, Event::Removed);
                            s.nodes[E].data.exists[0] = false;
                        }
                    }
                    13 => {
                        self.place(s, 2);
                        self.setup(c, s, 2);
                    }
                    14 => {
                        if self.part == Part::Declared {
                            self.declare(c, s, Reason::Root);
                        }
                        self.retire(c, s, 1, Reason::Request);
                        self.stop(s, 1);
                        s.nodes[E].data.exists[1] = false;
                        Self::event(s, 1, Event::Removed);
                    }
                    15 => {
                        self.retire(c, s, 2, Reason::Request);
                        self.stop(s, 2);
                        s.nodes[E].data.exists[2] = false;
                        Self::event(s, 2, Event::Removed);
                    }
                    16 => {
                        s.nodes[E].data.provider_retired = true;
                        s.nodes[E].data.providers = false;
                    }
                    _ => unreachable!(),
                }
                s.nodes[E].data.stage += 1;
            }
            Local::Send => {
                s.nodes[A].data.sent = true;
                let count = if self.part.consumer_partition() { 1 } else { 2 };
                for id in 0..count {
                    s.send(c, A, E, Message::Request(id));
                }
            }
            Local::Exit => {
                s.nodes[M].data.runs[0] = 0;
                s.ledger.live[0] = 0;
                Self::event(s, 0, Event::Exit);
                s.nodes[M].data.restart_pending = true;
                s.nodes[E].data.stage = 9;
            }
            Local::Restart => {
                s.nodes[M].data.runs[0] = 1;
                s.ledger.live[0] = 1;
                let family_live = s.ledger.live[0] + s.ledger.live[2];
                Self::event(
                    s,
                    0,
                    Event::Start {
                        machine: 2,
                        family_live,
                        root_risk: s.ledger.root_risk,
                        restart: true,
                    },
                );
                s.nodes[M].data.restart_pending = false;
            }
            Local::Carry(i) => {
                let i = i as usize;
                s.nodes[M].data.kept[i] = false;
                s.nodes[E].data.volatile_stops[i] = false;
                s.nodes[M].data.runs[i] = 0;
                s.ledger.live[i] = 0;
                // One carried stop discharges all prior asks for the same run.
                let asks: Vec<_> = s
                    .ledger
                    .histories
                    .iter()
                    .enumerate()
                    .flat_map(|(subject, es)| {
                        es.iter().filter_map(move |e| match e {
                            Event::StopAsked {
                                named,
                                answer: Answer::Unknown,
                                obligation,
                                ..
                            } if *named == i as u8 => Some((subject, *obligation)),
                            _ => None,
                        })
                    })
                    .collect();
                for (subject, obligation) in asks {
                    if !s.ledger.histories[subject].iter().any(
                        |e| matches!(e, Event::StopDone { obligation: n, .. } if *n == obligation),
                    ) {
                        Self::event(
                            s,
                            subject,
                            Event::StopDone {
                                named: i as u8,
                                late: true,
                                obligation,
                                provider_retired: s.nodes[E].data.provider_retired,
                            },
                        );
                    }
                }
            }
            Local::ReturnMachine => {
                s.nodes[M].status = Status::Running;
                s.nodes[M].data.returned = true;
                // A new machine cannot restart a run that belonged to the vanished one.
                s.nodes[M].data.restart_pending = false;
                for i in 0..3 {
                    if s.nodes[M].data.runs[i] > 0 {
                        Self::event(s, i, Event::Exit);
                    }
                }
                s.nodes[M].data.runs = [0; 3];
                s.ledger.live = [0; 3];
            }
            Local::RecreateAfterSilence => {
                s.nodes[E].data.recreated = true;
                self.place(s, 2);
                self.setup(c, s, 2);
            }
            Local::SetupReturnedNode => {
                s.nodes[E].data.node_ready = true;
            }
            Local::FinishRetire => {
                s.nodes[E].data.retire_hook_pending = false;
                self.request_inspect(s, 0, InspectFor::FinishRetire);
            }
            Local::InspectAnswer => {
                let (i, _) = s.nodes[E].data.inspect_pending.unwrap();
                self.inspect(s, i);
                s.nodes[E].data.inspect_answered = true;
            }
            Local::UseInspect => {
                let (i, purpose) = s.nodes[E].data.inspect_pending.take().unwrap();
                s.nodes[E].data.inspect_answered = false;
                let r = s.nodes[E].data.inspected[i].unwrap().value;
                let lost = s.nodes[E].data.placements[i] <= s.nodes[E].data.lost_through;
                if !Self::unavailable(s) && !lost {
                    match purpose {
                        InspectFor::Setup | InspectFor::Retire => {
                            if r == RunState::Stopped
                                || (r == RunState::Unknown && self.variant == Variant::UnknownRun)
                            {
                                self.ask_run(c, s, i, purpose == InspectFor::Retire, r);
                                if purpose == InspectFor::Retire {
                                    s.nodes[E].data.retire_hook_pending = true;
                                }
                            } else if purpose == InspectFor::Retire && r == RunState::Running {
                                Self::event(
                                    s,
                                    i,
                                    Event::Hook {
                                        lost: false,
                                        unreachable: false,
                                        restart_only: false,
                                    },
                                );
                            }
                            if purpose == InspectFor::Retire && !s.nodes[E].data.retire_hook_pending
                            {
                                self.stop(s, i);
                            }
                        }
                        InspectFor::FinishRetire => {
                            if r == RunState::Running {
                                Self::event(
                                    s,
                                    i,
                                    Event::Hook {
                                        lost: false,
                                        unreachable: false,
                                        restart_only: false,
                                    },
                                );
                            }
                            self.stop(s, i);
                        }
                        InspectFor::Observe => {}
                    }
                }
            }
            Local::ObserveServers => {
                s.ledger.lifecycle.push((
                    s.nodes[E].status == Status::Running,
                    s.nodes[E].data.providers,
                ));
            }
            Local::Quiescent => {
                s.ledger.quiescent = true;
            }
        }
    }
    fn properties(&self) -> Vec<NamedProperty<Self>> {
        let mut p: Vec<NamedProperty<Self>> = vec![
            NamedProperty {
                property: Property::<Harness<Self>>::always(ONE, |_, s| {
                    s.ledger
                        .machine
                        .iter()
                        .all(|r| r.value != Health::Gone || r.truth == Health::Gone)
                        && s.ledger
                            .histories
                            .iter()
                            .flatten()
                            .chain(s.ledger.requests.iter().flatten())
                            .all(|e| match e {
                                Event::Start {
                                    family_live,
                                    root_risk,
                                    ..
                                } => *family_live <= 1 || *root_risk,
                                Event::RunAsked { latest, .. } => *latest != RunState::Unknown,
                                Event::Hook { restart_only, .. }
                                | Event::Secret { restart_only, .. } => !restart_only,
                                _ => true,
                            })
                }),
                promises: &["PRV-007", "PRV-010", "PRV-014"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(UNREACH, |_, s| {
                    s.ledger
                        .histories
                        .iter()
                        .flatten()
                        .chain(s.ledger.requests.iter().flatten())
                        .all(|e| match e {
                            Event::Retire {
                                reason: Reason::Silence,
                                ..
                            } => false,
                            Event::Request {
                                unreachable: true,
                                dispatched,
                                answer,
                                ..
                            } => !dispatched && *answer == Some(Answer::Unavailable),
                            Event::RunAsked { unreachable, .. }
                            | Event::Secret { unreachable, .. } => !unreachable,
                            _ => true,
                        })
                }),
                promises: &["PLC-023"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(LOST, |_, s| {
                    s.ledger.lost_subjects.iter().all(|(reason, node, subjects)| {
                        s.ledger.retired_nodes.contains(node) && subjects.iter().enumerate().all(|(i, exists)| !exists || s.ledger.histories[i].iter().any(|e| matches!(e, Event::Retire { reason: r, .. } if r == reason)))
                    }) && s.ledger.histories.iter().flatten().chain(s.ledger.requests.iter().flatten()).all(|e| match e {
                        Event::Retire {
                            lost: true,
                            setup: true,
                            hook: true,
                            confirmed,
                            ..
                        } => !confirmed,
                        Event::Hook { lost, .. }
                        | Event::Secret { lost, .. }
                        | Event::RunAsked { lost, .. } => !lost,
                        _ => true,
                    })
                }),
                promises: &["PLC-017", "PLC-024", "PLC-027"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(STOPS, |_, s| {
                    s.ledger.histories.iter().enumerate().all(|(i, es)| es.iter().all(|e| match e {
                Event::StopAsked { named, machine, obligation, answer } => *named == i as u8 && *machine == 2 && (!s.ledger.quiescent || *answer != Answer::Unknown || es.iter().any(|e| matches!(e, Event::StopDone { obligation: n, named: j, .. } if n == obligation && *j == i as u8))),
                Event::StopDone { named, .. } => *named == i as u8,
                _ => true,
            }))
                }),
                promises: &["PRV-008", "PRV-021", "PRV-027"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(PLACED, |_, s| {
                    s.ledger.histories.iter().all(|es| {
                        es.iter().all(|e| match e {
                            Event::Place {
                                machine, selected, ..
                            } => machine == selected,
                            Event::Start { machine, .. } | Event::RunAsked { machine, .. } => {
                                *machine == 2
                            }
                            _ => true,
                        })
                    })
                }),
                promises: &["PLC-002", "PRV-015", "PRV-019"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(REPORT, |_, s| {
                    s.ledger
                        .histories
                        .iter()
                        .flatten()
                        .chain(s.ledger.requests.iter().flatten())
                        .all(|e| match e {
                            Event::Status {
                                value,
                                report,
                                time,
                                unreachable,
                            } => {
                                (*value == RunState::Unknown
                                    || (!unreachable
                                        && report.is_some_and(|r| {
                                            r.value == *value && r.truth == r.value
                                        })))
                                    && *time == report.map(|r| r.time)
                                    && report.is_none_or(|r| {
                                        s.ledger.histories[r.instance as usize].iter().any(
                                        |e| matches!(e, Event::Inspect(original) if *original == r),
                                    )
                                    })
                            }
                            _ => true,
                        })
                }),
                promises: &["PRV-009", "OBS-003"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(READY, |_, s| {
                    s.ledger
                        .histories
                        .iter()
                        .flatten()
                        .chain(s.ledger.requests.iter().flatten())
                        .all(|e| match e {
                            Event::Setup {
                                ready: false,
                                dispatched,
                                answer,
                                ..
                            } => !dispatched && *answer == Answer::Unavailable,
                            _ => true,
                        })
                }),
                promises: &["PLC-019"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(NEW, |_, s| {
                    s.ledger.nodes.iter().enumerate().all(|(i, (id, retired))| {
                        !retired && s.ledger.nodes[..i].iter().all(|(old, _)| old != id)
                    })
                }),
                promises: &["PLC-026"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(SERVERS, |_, s| {
                    s.ledger
                        .lifecycle
                        .iter()
                        .all(|(engine, servers)| *engine || !servers)
                }),
                promises: &["PRV-002"],
            },
        ];
        type Predicate = fn(&Harness<Placement>, &System<Placement>) -> bool;
        let predicates: [Predicate; 11] = [
            |_, s| s.ledger.saw_return,
            |_, s| s.ledger.declarations.contains(&Reason::Gone),
            |_, s| s.ledger.declarations.contains(&Reason::Root),
            |_, s| {
                s.ledger
                    .histories
                    .iter()
                    .flatten()
                    .any(|e| matches!(e, Event::StopDone { late: true, .. }))
            },
            |_, s| {
                s.ledger.requests.iter().flatten().any(|e| {
                    matches!(
                        e,
                        Event::Request {
                            cut: true,
                            dispatched: false,
                            answer: Some(Answer::Unavailable),
                            ..
                        }
                    )
                })
            },
            |_, s| {
                s.ledger.histories[0]
                    .iter()
                    .any(|e| matches!(e, Event::Start { restart: true, .. }))
            },
            |_, s| {
                s.ledger.histories[2]
                    .iter()
                    .any(|e| matches!(e, Event::Place { .. }))
            },
            |_, s| {
                s.ledger
                    .nodes
                    .iter()
                    .any(|(id, retired)| *id > 1 && !retired)
            },
            |_, s| {
                s.ledger.histories[0].iter().any(|e| {
                    matches!(
                        e,
                        Event::Inspect(RunReport {
                            value: RunState::Unknown,
                            ..
                        })
                    )
                }) && !s.ledger.histories[0].iter().any(|e| {
                    matches!(
                        e,
                        Event::RunAsked {
                            latest: RunState::Unknown,
                            ..
                        }
                    )
                })
            },
            |_, s| {
                s.ledger.histories[0].iter().any(|e| {
                    matches!(
                        e,
                        Event::Setup {
                            ready: false,
                            dispatched: false,
                            answer: Answer::Unavailable,
                            ..
                        }
                    )
                })
            },
            |_, s| {
                s.ledger
                    .histories
                    .iter()
                    .flatten()
                    .chain(s.ledger.requests.iter().flatten())
                    .any(|e| {
                        matches!(
                            e,
                            Event::StopDone {
                                provider_retired: true,
                                ..
                            }
                        )
                    })
            },
        ];
        for (name, condition) in WITNESSES.into_iter().zip(predicates) {
            p.push(NamedProperty {
                property: Property::sometimes(name, condition),
                promises: &["harness"],
            });
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn expected(v: Variant, part: Part) -> (Vec<&'static str>, Vec<&'static str>) {
        let mut broken = match v {
            Variant::Correct => vec![],
            Variant::UnreachableGone => vec![UNREACH, ONE],
            _ => vec![v.target()],
        };
        let mut reached = vec![LATE, RESTART, REPLACEMENT, NOT_READY, RETIRED_KEEPER];
        if matches!(
            part,
            Part::Partition
                | Part::PartitionReply
                | Part::PartitionRestartRace
                | Part::PartitionReplyRestartRace
                | Part::Combined
                | Part::CombinedRestartRace
                | Part::PartitionRetire
                | Part::Declared
        ) {
            reached.extend([RETURN, CUT_REQUEST, UNKNOWN]);
        }
        if part == Part::PartitionRetire || part.reply_partition() {
            reached.retain(|n| *n != CUT_REQUEST);
        }
        if part == Part::Gone {
            reached.extend([GONE, NEW_NODE]);
        }
        if part == Part::Declared {
            reached.extend([ROOT, NEW_NODE]);
        }
        if v == Variant::EarlySetup {
            reached.retain(|n| *n != NOT_READY);
        }
        if v == Variant::ReuseNode {
            reached.retain(|n| *n != NEW_NODE);
        }
        let mut missing: Vec<_> = WITNESSES
            .into_iter()
            .filter(|n| !reached.contains(n))
            .collect();
        broken.sort();
        missing.sort();
        (broken, missing)
    }
    #[test]
    fn full_runs_match_pre_registration() {
        let mut mismatches = vec![];
        for (v, part) in Part::ALL
            .into_iter()
            .map(|p| (Variant::Correct, p))
            .chain(Variant::BROKEN.into_iter().map(|v| (v, v.part())))
        {
            let report = check_all(model(v, part));
            println!("MC4 {v:?}/{part:?}: {report}");
            let mut broken = report.broken;
            broken.sort();
            let mut missing = report.missing_witnesses;
            missing.sort();
            let observed = (broken, missing);
            let registered = expected(v, part);
            if observed != registered {
                println!(
                    "MC4 difference {v:?}/{part:?}: expected {registered:?}, observed {observed:?}"
                );
            }
            let mut recorded = registered;
            // MC4-002: replacement can overlap a removed ID with an unknown stop.
            // MC4-009: independent provider restart races the retire-hook run call.
            if !matches!(part, Part::Partition | Part::PartitionReply) {
                recorded.0.push(ONE);
            }
            // MC4-007: stopping J for I leaves I live when its replacement starts.
            if v == Variant::WrongStop {
                recorded.0.push(ONE);
            }
            recorded.0.sort();
            recorded.0.dedup();
            // MC4-003: the retirement windows occur after A's exchange.
            if matches!(
                part,
                Part::Combined | Part::CombinedRestartRace | Part::PartitionRetire | Part::Declared
            ) {
                recorded.1.push(CUT_REQUEST);
            }
            // MC4-004: treating silence as loss really registers a fresh returned node.
            if v == Variant::UnreachableGone {
                recorded.1.retain(|n| *n != NEW_NODE);
            }
            // MC4-008: root's cut is after the explicit inspection window.
            if part == Part::Declared {
                recorded.1.push(UNKNOWN);
            }
            // MC4-006: transport loss or a not-yet-reported gone machine also yields unknown.
            if matches!(part, Part::LossM2 | Part::Gone) {
                recorded.1.retain(|n| *n != UNKNOWN);
            }
            // MC4-005: the unknown-run mutant never takes the blocking branch in this window.
            if v == Variant::UnknownRun {
                recorded.1.push(UNKNOWN);
            }
            // MC4-010: the before-dispatch cut cannot overlap I's inspection window.
            if matches!(part, Part::Partition | Part::PartitionRestartRace) {
                recorded.1.push(UNKNOWN);
            }
            // MC4-011: this mutant never answers the blocked setup unavailable.
            if v == Variant::BadSetupAnswer {
                recorded.1.push(NOT_READY);
            }
            recorded.1.sort();
            recorded.1.dedup();
            if observed != recorded {
                mismatches.push(format!("{v:?}/{part:?}: {observed:?}"));
            }
        }
        assert!(
            mismatches.is_empty(),
            "MC4 observed table mismatches: {mismatches:?}"
        );
    }
    #[test]
    fn literal_restart_property_refutation() {
        println!("MC4 literal refutation Correct/Base: {ONE}");
        assert_refuted(model(Variant::Correct, Part::Base), ONE);
    }
    #[test]
    fn literal_replacement_property_refutation() {
        println!("MC4 literal refutation Correct/PartitionRetire: {ONE}");
        assert_refuted(model(Variant::Correct, Part::PartitionRetire), ONE);
    }
    #[test]
    fn first_named_refutations_read_only_ledger() {
        for v in Variant::BROKEN {
            let m = model(v, v.part());
            println!("MC4 refutation {v:?}: {}", v.target());
            let trace = assert_refuted(m.clone(), v.target());
            println!("MC4 refutation {v:?}: {} actions", trace.len());
            let mut s = stateright::Path::from_actions(&m, m.init_states().remove(0), &trace)
                .unwrap()
                .last_state()
                .clone();
            if v == Variant::ConfirmLaterLost {
                assert!(s.ledger.histories[2].iter().any(|e| matches!(
                    e,
                    Event::Retire {
                        lost: true,
                        reason: Reason::Root,
                        confirmed: false,
                        ..
                    }
                )));
                assert!(s.ledger.histories[2].iter().any(|e| matches!(
                    e,
                    Event::Retire {
                        lost: true,
                        reason: Reason::Request,
                        confirmed: true,
                        ..
                    }
                )));
            }
            if v == Variant::BadSetupAnswer {
                assert!(s.ledger.histories[0].iter().any(|e| matches!(
                    e,
                    Event::Setup {
                        ready: false,
                        dispatched: false,
                        answer: Answer::Unknown,
                        ..
                    }
                )));
            }
            s.nodes.iter_mut().for_each(|n| n.data = Data::default());
            assert!(!(m.property(v.target()).condition)(&m, &s));
        }
    }
    fn take(m: &Harness<Placement>, s: &mut System<Placement>, action: Action<Local>) {
        let mut enabled = vec![];
        m.actions(s, &mut enabled);
        assert!(enabled.contains(&action), "disabled: {action:?}");
        *s = m.next_state(s, action).unwrap();
    }
    fn drain(m: &Harness<Placement>, s: &mut System<Placement>) {
        loop {
            let mut enabled = vec![];
            m.actions(s, &mut enabled);
            let Some(a) = enabled
                .into_iter()
                .find(|a| matches!(a, Action::Deliver { .. }))
            else {
                break;
            };
            take(m, s, a);
        }
    }
    fn inspect_steps(m: &Harness<Placement>, s: &mut System<Placement>) {
        if s.nodes[E].data.inspect_pending.is_some() {
            take(m, s, Action::Local(E, Local::InspectAnswer));
            take(m, s, Action::Local(E, Local::UseInspect));
        }
    }
    fn advance_to(m: &Harness<Placement>, s: &mut System<Placement>, target: u8) {
        while s.nodes[E].data.stage < target {
            let stage = s.nodes[E].data.stage;
            if stage == 6 && !s.nodes[A].data.sent {
                take(m, s, Action::Local(A, Local::Send));
            }
            inspect_steps(m, s);
            drain(m, s);
            if s.nodes[E].data.retire_hook_pending {
                take(m, s, Action::Local(E, Local::FinishRetire));
                inspect_steps(m, s);
            }
            for i in 0..3 {
                if Placement::pending(s, i) {
                    take(m, s, Action::Local(M, Local::Carry(i as u8)));
                }
            }
            take(m, s, Action::Local(E, Local::Script(stage)));
        }
    }
    #[test]
    fn cut_heal_late_reply_preserves_unknown_end() {
        let m = model(Variant::Correct, Part::PartitionReply);
        let mut s = m.init_states().remove(0);
        take(
            &m,
            &mut s,
            Action::Local(E, Local::Report(Health::Reachable)),
        );
        advance_to(&m, &mut s, 6);
        inspect_steps(&m, &mut s);
        drain(&m, &mut s);
        take(&m, &mut s, Action::Local(A, Local::Send));
        take(
            &m,
            &mut s,
            Action::Deliver {
                src: A,
                dst: E,
                index: 0,
            },
        );
        take(
            &m,
            &mut s,
            Action::Deliver {
                src: E,
                dst: M,
                index: 0,
            },
        );
        take(&m, &mut s, Action::Cut(E, M));
        take(
            &m,
            &mut s,
            Action::Local(E, Local::Report(Health::Unreachable)),
        );
        take(
            &m,
            &mut s,
            Action::Deliver {
                src: E,
                dst: A,
                index: 0,
            },
        );
        take(&m, &mut s, Action::Local(E, Local::Script(6)));
        inspect_steps(&m, &mut s);
        take(&m, &mut s, Action::Heal(E, M));
        take(
            &m,
            &mut s,
            Action::Local(E, Local::Report(Health::Reachable)),
        );
        take(
            &m,
            &mut s,
            Action::Deliver {
                src: M,
                dst: E,
                index: 0,
            },
        );
        drain(&m, &mut s);
        assert_eq!(s.nodes[E].data.request_ends[0], Some(Answer::Unknown));
        assert_eq!(
            s.ledger.requests[0]
                .iter()
                .filter(|e| matches!(e, Event::RequestAnswer(..)))
                .count(),
            1
        );
        assert!(s.ledger.requests[0].contains(&Event::ImplementationReply(0)));
        assert!(s.ledger.requests[0].contains(&Event::CallerAnswer(0, Answer::Unknown)));
        assert!(!s.ledger.requests[0].contains(&Event::CallerAnswer(0, Answer::Outcome)));
    }
    #[test]
    fn provider_restart_between_inspect_and_run_refutes_one() {
        let m = model(Variant::Correct, Part::Base);
        let mut s = m.init_states().remove(0);
        take(
            &m,
            &mut s,
            Action::Local(E, Local::Report(Health::Reachable)),
        );
        advance_to(&m, &mut s, 8);
        take(&m, &mut s, Action::Local(M, Local::Exit));
        advance_to(&m, &mut s, 11);
        take(&m, &mut s, Action::Local(E, Local::Script(11)));
        take(&m, &mut s, Action::Local(E, Local::InspectAnswer));
        assert_eq!(
            s.nodes[E].data.inspected[0].unwrap().value,
            RunState::Stopped
        );
        take(&m, &mut s, Action::Local(M, Local::Restart));
        take(&m, &mut s, Action::Local(E, Local::UseInspect));
        drain(&m, &mut s);
        assert_eq!(s.ledger.live[0], 2);
        s.nodes.iter_mut().for_each(|n| n.data = Data::default());
        assert!(!(m.property(ONE).condition)(&m, &s));
    }
    #[test]
    fn ready_requires_unavailable_for_every_blocked_setup() {
        let m = model(Variant::Correct, Part::Base);
        for answer in [Answer::Outcome, Answer::Unknown, Answer::Unavailable] {
            let mut s = m.init_states().remove(0);
            s.ledger.histories[0].push(Event::Setup {
                ready: false,
                unreachable: false,
                dispatched: false,
                answer,
            });
            assert_eq!(
                (m.property(READY).condition)(&m, &s),
                answer == Answer::Unavailable
            );
        }
    }
    #[test]
    fn lost_confirmation_is_checked_after_declaration() {
        let m = model(Variant::Correct, Part::Base);
        let mut s = m.init_states().remove(0);
        s.ledger.histories[2].push(Event::Retire {
            lost: true,
            reason: Reason::Root,
            setup: true,
            hook: true,
            confirmed: false,
        });
        assert!((m.property(LOST).condition)(&m, &s));
        s.ledger.histories[2].push(Event::Retire {
            lost: true,
            reason: Reason::Request,
            setup: true,
            hook: true,
            confirmed: true,
        });
        assert!(!(m.property(LOST).condition)(&m, &s));
    }
    #[test]
    fn retire_hook_run_finishes_before_hook_and_stop() {
        let m = model(Variant::Correct, Part::Base);
        let mut s = m.init_states().remove(0);
        take(
            &m,
            &mut s,
            Action::Local(E, Local::Report(Health::Reachable)),
        );
        while s.nodes[E].data.stage < 11 {
            let stage = s.nodes[E].data.stage;
            if stage == 6 {
                take(&m, &mut s, Action::Local(A, Local::Send));
            }
            inspect_steps(&m, &mut s);
            drain(&m, &mut s);
            if stage == 8 {
                take(&m, &mut s, Action::Local(M, Local::Exit));
            } else {
                take(&m, &mut s, Action::Local(E, Local::Script(stage)));
            }
        }
        take(&m, &mut s, Action::Local(E, Local::Script(11)));
        inspect_steps(&m, &mut s);
        assert!(s.nodes[E].data.retire_hook_pending);
        assert!(
            !s.ledger.histories[0]
                .iter()
                .any(|e| matches!(e, Event::Hook { .. } | Event::StopAsked { .. }))
        );
        let mut actions = vec![];
        m.actions(&s, &mut actions);
        assert!(!actions.contains(&Action::Local(E, Local::FinishRetire)));
        assert!(!actions.contains(&Action::Local(E, Local::Script(12))));
        drain(&m, &mut s);
        take(&m, &mut s, Action::Local(E, Local::FinishRetire));
        inspect_steps(&m, &mut s);
        let hook = s.ledger.histories[0]
            .iter()
            .position(|e| matches!(e, Event::Hook { .. }))
            .unwrap();
        let stop = s.ledger.histories[0]
            .iter()
            .position(|e| matches!(e, Event::StopAsked { .. }))
            .unwrap();
        assert!(hook < stop);
    }
    #[test]
    fn further_retirement_after_gone_asks_stop() {
        let m = model(Variant::Correct, Part::Gone);
        let mut s = m.init_states().remove(0);
        take(
            &m,
            &mut s,
            Action::Local(E, Local::Report(Health::Reachable)),
        );
        advance_to(&m, &mut s, 10);
        take(&m, &mut s, Action::Lose(M));
        take(&m, &mut s, Action::Local(E, Local::Report(Health::Gone)));
        take(&m, &mut s, Action::Local(E, Local::Script(10)));
        let asked = |s: &System<Placement>| {
            s.ledger.histories[0]
                .iter()
                .filter(|e| matches!(e, Event::StopAsked { .. }))
                .count()
        };
        let before = asked(&s);
        let earlier = s.ledger.histories[0].len();
        take(&m, &mut s, Action::Local(E, Local::Script(11)));
        assert_eq!(asked(&s), before + 1);
        assert_eq!(s.nodes[E].data.health, Health::Gone);
        assert!(s.nodes[E].data.inspect_pending.is_none());
        assert!(
            !s.ledger.histories[0].iter().skip(earlier).any(|e| matches!(
                e,
                Event::Hook {
                    restart_only: false,
                    ..
                } | Event::RunAsked { .. }
                    | Event::Secret { .. }
            ))
        );
    }
    #[test]
    fn retirement_on_declared_lost_node_asks_stop_when_unreachable() {
        let m = model(Variant::Correct, Part::Declared);
        for report in [Health::Unreachable, Health::Omitted] {
            let mut s = m.init_states().remove(0);
            take(
                &m,
                &mut s,
                Action::Local(E, Local::Report(Health::Reachable)),
            );
            advance_to(&m, &mut s, 14);
            inspect_steps(&m, &mut s);
            drain(&m, &mut s);
            take(&m, &mut s, Action::Cut(E, M));
            take(&m, &mut s, Action::Local(E, Local::Report(report)));
            let asked = |s: &System<Placement>| {
                s.ledger.histories[1]
                    .iter()
                    .filter(|e| matches!(e, Event::StopAsked { .. }))
                    .count()
            };
            let before = asked(&s);
            take(&m, &mut s, Action::Local(E, Local::Script(14)));
            assert!(s.nodes[E].data.node_lost);
            // The retirement asks for the stop, and so does the unregistration that follows.
            assert_eq!(asked(&s), before + 2);
        }
    }
    #[test]
    fn omitted_report_registers_no_node() {
        let m = model(Variant::Correct, Part::Declared);
        let mut s = m.init_states().remove(0);
        take(
            &m,
            &mut s,
            Action::Local(E, Local::Report(Health::Reachable)),
        );
        advance_to(&m, &mut s, 14);
        inspect_steps(&m, &mut s);
        drain(&m, &mut s);
        take(&m, &mut s, Action::Cut(E, M));
        take(&m, &mut s, Action::Local(E, Local::Script(14)));
        assert!(s.nodes[E].data.node_lost);
        let nodes = s.ledger.nodes.len();
        take(&m, &mut s, Action::Local(E, Local::Report(Health::Omitted)));
        assert_eq!(s.ledger.nodes.len(), nodes);
        assert!(s.nodes[E].data.node_lost);
    }
    #[test]
    fn metadata_and_fault_scopes() {
        for part in Part::ALL {
            let m = model(Variant::Correct, part);
            let mut s = m.init_states().remove(0);
            let mut enabled = vec![];
            m.actions(&s, &mut enabled);
            assert!(
                !enabled
                    .iter()
                    .any(|a| matches!(a, Action::Cut(..) | Action::Crash(..) | Action::Lose(..)))
            );
            if part.combined() {
                s.nodes[E].data.stage = 11;
                enabled.clear();
                m.actions(&s, &mut enabled);
                assert!(enabled.contains(&Action::Cut(E, M)));
                s.cut.insert((E, M));
                s.nodes[E].data.stage = 13;
                enabled.clear();
                m.actions(&s, &mut enabled);
                assert!(enabled.contains(&Action::Crash(E)));
                assert!(!enabled.contains(&Action::Heal(E, M)));
                s.used[Fault::Recovery as usize] = 1;
                enabled.clear();
                m.actions(&s, &mut enabled);
                assert!(enabled.contains(&Action::Heal(E, M)));
            }
            assert_eq!(
                m.protocol
                    .properties()
                    .iter()
                    .filter(|p| p.property.expectation == stateright::Expectation::Always)
                    .count(),
                SAFETY.len()
            );
            for src in 0..3 {
                for dst in 0..3 {
                    assert_eq!(
                        m.protocol.fault_allowed(Fault::Recovery, src, dst),
                        src == E && dst == E
                    );
                    assert_eq!(
                        m.protocol.fault_allowed(Fault::Heal, src, dst),
                        (src == E && dst == M) || (src == M && dst == E)
                    );
                    assert!(!m.protocol.fault_allowed(Fault::Timer, src, dst));
                }
            }
        }
    }
}
