//! MC5b: reads, run lifetimes and remote headers, independent of engine code.
use crate::harness::*;
use stateright::Property;
use std::time::Duration;

const E: usize = 0;
const K: usize = 1;
const I: usize = 2;
const H: usize = 3;
const O: usize = 4;
const P: u8 = 0;
const INSTANCE: u8 = 1;
const R: u8 = 2;
pub const PATH: &str = "SEC-007 SEC-010 SEC-011 SEC-020: one read path";
pub const STOP: &str = "SEC-012 SEC-016: failed read stops its work";
pub const VALUES: &str = "SEC-015 SEC-017 SEC-018: values stay where sent";
pub const FIXED: &str = "SEC-014 PRV-026: fixed run and provider re-read";
pub const HEADER: &str = "SEC-013 SEC-023: remote header";
pub const SAFETY: [&str; 5] = [PATH, STOP, VALUES, FIXED, HEADER];
const HTTP_WORK: &str = "MC5b: HTTP dispatches match work entries";
const WITNESSES: [&str; 12] = [
    "MC5b: unknown read",
    "MC5b: record unavailable",
    "MC5b: I requests secret",
    "MC5b: O requests secret",
    "MC5b: I locally restarted",
    "MC5b: engine restarted",
    "MC5b: P run again",
    "MC5b: P closed refusal",
    "MC5b: R closed refusal",
    "MC5b: HTTP after restart on old connection with new read",
    "MC5b: redirect",
    "MC5b: late answer discarded",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Part {
    Base,
    ReadFault,
    RecordFault,
    Recovery,
    RecoveryReadFault,
    RecoveryRecordFault,
    CloseP,
    ClosePEarly,
    CloseR,
    CloseREarly,
    LossK,
    ReorderK,
    LossH,
    ReorderH,
}
impl Part {
    pub const ALL: [Self; 14] = [
        Self::Base,
        Self::ReadFault,
        Self::RecordFault,
        Self::Recovery,
        Self::RecoveryReadFault,
        Self::RecoveryRecordFault,
        Self::CloseP,
        Self::ClosePEarly,
        Self::CloseR,
        Self::CloseREarly,
        Self::LossK,
        Self::ReorderK,
        Self::LossH,
        Self::ReorderH,
    ];
    fn recovery(self) -> bool {
        matches!(
            self,
            Self::Recovery
                | Self::RecoveryReadFault
                | Self::RecoveryRecordFault
                | Self::CloseP
                | Self::ClosePEarly
                | Self::CloseR
                | Self::CloseREarly
                | Self::ReorderK
        )
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Correct,
    Onward,
    Borrow,
    Caller,
    Environment,
    EngineAnswer,
    FailedRun,
    Unrecorded,
    CachedP,
    RecordValue,
    RefreshI,
    ClosedP,
    CachedHeader,
    Redirect,
}
impl Variant {
    pub const BROKEN: [Self; 13] = [
        Self::Onward,
        Self::Borrow,
        Self::Caller,
        Self::Environment,
        Self::EngineAnswer,
        Self::FailedRun,
        Self::Unrecorded,
        Self::CachedP,
        Self::RecordValue,
        Self::RefreshI,
        Self::ClosedP,
        Self::CachedHeader,
        Self::Redirect,
    ];
    pub fn part(self) -> Part {
        match self {
            Self::Caller | Self::EngineAnswer | Self::CachedP | Self::CachedHeader => {
                Part::Recovery
            }
            Self::FailedRun => Part::ReadFault,
            Self::Unrecorded => Part::RecordFault,
            Self::ClosedP => Part::CloseP,
            _ => Part::Base,
        }
    }
    pub fn target(self) -> &'static str {
        match self {
            Self::Onward | Self::Borrow | Self::Caller | Self::Environment | Self::EngineAnswer => {
                PATH
            }
            Self::FailedRun | Self::Unrecorded | Self::ClosedP => STOP,
            Self::CachedP | Self::RecordValue => VALUES,
            Self::RefreshI => FIXED,
            Self::CachedHeader | Self::Redirect => HEADER,
            Self::Correct => panic!("no mutant"),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Outcome {
    Succeeded,
    Reason,
    Failed,
    Unknown,
    Unavailable,
    Refused,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Place {
    Secrets,
    Runner,
    Header,
    Record,
    Cache,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Work {
    Run(u8),
    Connection,
    Http(u8),
}
fn owner(read: u8) -> u8 {
    match read {
        0 | 5 => P,
        1 | 6 => INSTANCE,
        _ => R,
    }
}
fn work(read: u8) -> Work {
    match read {
        0 | 5 => Work::Run(read),
        1 | 6 => Work::Run(1),
        2 => Work::Connection,
        _ => Work::Http(read - 3),
    }
}
fn caller(instance: u8) -> u8 {
    10 + instance
}
fn invocation(instance: u8) -> u8 {
    100 + instance
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Read(u8),
    Value(u8),
    Http(u8),
    Redirect(u8),
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Data {
    // Saved engine state: script, access, invocation status and connection metadata, no values.
    stage: u8,
    epoch: u8,
    sync: bool,
    closed: [bool; 3],
    status: [u8; 7],
    p_asked: bool,
    i_running: bool,
    connection: bool,
    connection_epoch: u8,
    p_attempted: bool,
    inspected: bool,
    read_fault: bool,
    record_fault: bool,
    // Volatile engine state; I's node separately keeps I's run value across P's death.
    p_live: bool,
    p_run: Option<u8>,
    header: Option<u8>,
    values: [Option<u8>; 7],
    local_restart: bool,
    run_value: Option<u8>,
    redirect_sent: bool,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Read {
        id: u8,
        instance: u8,
        caller: u8,
        invocation: u8,
        work: Work,
        conduit: u8,
        open: bool,
        dispatched: bool,
        record_kept: bool,
    },
    End {
        id: u8,
        outcome: Outcome,
        responder: usize,
    },
    Get {
        sender: usize,
        conduit: u8,
        serving: Option<u8>,
        dispatched: bool,
        answer: Outcome,
    },
    Use {
        // None denotes the mutant's injected engine-environment value.
        read: Option<u8>,
        work: Work,
        place: Place,
    },
    Work {
        work: Work,
        read: u8,
    },
    Http {
        id: u8,
        read: u8,
        url: u8,
        connection_epoch: u8,
    },
    Call {
        run: bool,
        p_live: bool,
        p_run: Option<u8>,
    },
    Close(u8),
    LocalRestart {
        read: u8,
    },
    Restart,
    RecordLost(u8),
    Redirect(u8),
    Discard(u8),
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Ledger(Vec<(u8, Event)>);
impl Ledger {
    fn add(&mut self, epoch: u8, event: Event) {
        self.0.push((epoch, event));
    }
    fn read(&self, id: u8) -> Option<(u8, &Event)> {
        self.0.iter().find_map(|(e, x)| {
            matches!(x, Event::Read { id: r, .. } if *r == id).then_some((*e, x))
        })
    }
    fn outcome(&self, id: u8) -> Option<Outcome> {
        self.0.iter().find_map(|(_, x)| {
            if let Event::End { id: r, outcome, .. } = x {
                (*r == id).then_some(*outcome)
            } else {
                None
            }
        })
    }
    fn has(&self, predicate: impl Fn(u8, &Event) -> bool) -> bool {
        self.0.iter().any(|(e, x)| predicate(*e, x))
    }
    fn path(&self) -> bool {
        self.0.iter().all(|(_, x)| match x {
            Event::Read {
                id,
                instance,
                caller: c,
                invocation: inv,
                conduit,
                open,
                dispatched,
                ..
            } => {
                *instance == owner(*id)
                    && *c == caller(*instance)
                    && *inv == invocation(*instance)
                    && (!dispatched || (*conduit == *instance && *open))
            }
            Event::Get {
                dispatched, answer, ..
            } => !dispatched && *answer == Outcome::Refused,
            Event::End {
                outcome: Outcome::Succeeded,
                responder,
                ..
            } => *responder == K,
            Event::Use { read: None, .. } => false,
            _ => true,
        })
    }
    fn stop(&self) -> bool {
        self.0.iter().all(|(_, x)| match x {
            Event::Read {
                dispatched,
                open,
                record_kept,
                id,
                ..
            } => {
                (!dispatched || (*open && *record_kept))
                    && (*record_kept || self.outcome(*id) == Some(Outcome::Unavailable))
            }
            Event::Work { read, .. } => self.outcome(*read) == Some(Outcome::Succeeded),
            _ => true,
        })
    }
    fn http_work_matches_dispatches(&self) -> bool {
        let work = self.0.iter().filter_map(|(epoch, event)| match event {
            Event::Work {
                work: Work::Http(id),
                read,
            } => Some((*epoch, *id, *read)),
            _ => None,
        });
        let dispatches = self.0.iter().filter_map(|(epoch, event)| match event {
            Event::Http { id, read, .. } => Some((*epoch, *id, *read)),
            _ => None,
        });
        work.eq(dispatches)
    }
    fn values(&self) -> bool {
        self.0.iter().all(|(epoch, x)| match x {
            Event::Use {
                read: Some(id),
                work: dest,
                place,
            } => {
                let allowed = match place {
                    Place::Secrets | Place::Runner => {
                        *dest == work(*id) && matches!(dest, Work::Run(_))
                    }
                    Place::Header => {
                        owner(*id) == R && matches!(dest, Work::Connection | Work::Http(_))
                    }
                    _ => false,
                };
                allowed
                    && self.read(*id).is_some_and(|(e, _)| {
                        // I's provider-held run value may survive the engine's restart.
                        *dest == Work::Run(1) || e == *epoch
                    })
            }
            _ => true,
        })
    }
    fn fixed(&self) -> bool {
        self.0
            .iter()
            .enumerate()
            .all(|(index, (epoch, x))| match x {
                Event::Read {
                    instance: INSTANCE, ..
                } => !self.0[..index].iter().any(|(_, e)| {
                    matches!(
                        e,
                        Event::Work {
                            work: Work::Run(1),
                            ..
                        }
                    )
                }),
                Event::Work {
                    work: Work::Run(5),
                    read,
                } => self.read(*read).is_some_and(|(e, r)| {
                    e == *epoch
                        && e > 0
                        && matches!(
                            r,
                            Event::Read {
                                open: true,
                                dispatched: true,
                                ..
                            }
                        )
                }),
                Event::Call { p_live, p_run, .. } if *epoch > 0 => {
                    *p_live
                        && p_run.is_some_and(|id| {
                            self.0[..index].iter().any(|(e, x)| {
                            *e == *epoch
                                && matches!(x, Event::Work { work: Work::Run(r), .. } if *r == id)
                        })
                        })
                }
                _ => true,
            })
    }
    fn header(&self) -> bool {
        self.0.iter().enumerate().all(|(index, (epoch, x))| match x {
            Event::Work { work: Work::Connection, read } => self.read(*read).is_some_and(|(e, r)| e == *epoch && matches!(r, Event::Read { instance: R, dispatched: true, open: true, .. })),
            Event::Http { id, read, url, .. } => *url == 0 && self.read(*read).is_some_and(|(e, r)| {
                e == *epoch && self.outcome(*read) == Some(Outcome::Succeeded)
                    && matches!(r, Event::Read { instance: R, conduit: R, open: true, dispatched: true, .. })
                    && (*read == 2 || *read == *id + 3 || self.0[..index].iter().any(|(ep, event)| *ep == *epoch && matches!(event, Event::Http { read: previous, .. } if previous == read)))
            }),
            _ => true,
        })
    }
    fn witness(&self, n: usize) -> bool {
        match n {
        0 => self.has(|_, x| matches!(x, Event::End { outcome: Outcome::Unknown, .. })),
        1 => self.has(|_, x| matches!(x, Event::RecordLost(_))),
        2 | 3 => self.has(|_, x| matches!(x, Event::Get { sender, .. } if *sender == if n == 2 { I } else { O })),
        4 => self.has(|_, x| matches!(x, Event::LocalRestart { .. })),
        5 => self.has(|_, x| matches!(x, Event::Restart)),
        6 => self.has(|e, x| e > 0 && matches!(x, Event::Work { work: Work::Run(5), .. })),
        7 | 8 => self.has(|_, x| matches!(x, Event::Read { id, open: false, dispatched: false, .. } if if n == 7 { *id == 5 } else { matches!(*id, 3 | 4) })),
        9 => self.has(|e, x| matches!(x, Event::Http { read, connection_epoch, .. } if e > *connection_epoch && *read >= 3) && self.read(match x { Event::Http { read, .. } => *read, _ => unreachable!() }).is_some_and(|(re, _)| re == e)),
        10 => self.has(|_, x| matches!(x, Event::Redirect(_))),
        11 => self.has(|_, x| matches!(x, Event::Discard(_))),
        _ => unreachable!(),
    }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Local {
    Advance,
    RecoverLedger,
    ReadFault(Outcome),
    RecordFault,
    Close(u8),
    RestartI,
}
type Condition = fn(&Harness<Secrets>, &System<Secrets>) -> bool;

#[derive(Clone)]
pub struct Secrets {
    variant: Variant,
    part: Part,
}
pub fn model(variant: Variant, part: Part) -> Harness<Secrets> {
    let mut budgets = [0; 9];
    if part.recovery() {
        budgets[Fault::Crash as usize] = 1;
        budgets[Fault::Recovery as usize] = 1;
    }
    if matches!(part, Part::LossK | Part::LossH) {
        budgets[Fault::Loss as usize] = 1;
    }
    if matches!(part, Part::ReorderK | Part::ReorderH) {
        budgets[Fault::Reorder as usize] = 1;
    }
    Harness::new(
        Config {
            name: "MC5b split secret faults",
            nodes: vec!["engine (P server)", "K", "I node (P part)", "h (R)", "O"],
            budgets,
            disabled_reasons: std::array::from_fn(|i| {
                (budgets[i] == 0).then_some("split fault configuration; duplication, partition, permanent loss and timers outside MC5b")
            }),
            messages_per_link: 7,
            unique_state_ceiling: 150_000,
            timeout: Duration::from_secs(10),
            bounds: "one P setup, one I run and local restart, one R connection, two HTTP requests, one engine restart, one provider rerun, seven distinct reads, one faulted read and one failed record in separate splits",
        },
        Secrets { variant, part },
    )
}
impl Secrets {
    fn log(&self, s: &mut System<Self>, event: Event) {
        s.ledger.add(s.nodes[E].data.epoch, event);
    }
    fn current(&self, s: &System<Self>) -> Option<u8> {
        let d = &s.nodes[E].data;
        if d.epoch > 0
            && d.p_asked
            && !d.p_live
            && !d.p_attempted
            && (d.i_running || (d.stage == 1 && d.status[1] == 2))
        {
            return Some(5);
        }
        match d.stage {
            0 => Some(0),
            1 => Some(1),
            3 => Some(2),
            4 | 5 if d.connection && d.header.is_none() => Some(d.stage - 1),
            _ => None,
        }
    }
    fn begin(&self, c: &Config, s: &mut System<Self>, id: u8, record_kept: bool) {
        let instance = owner(id);
        let conduit = if self.variant == Variant::Borrow && id == 1 {
            P
        } else {
            instance
        };
        let open = !s.nodes[E].data.closed[conduit as usize];
        let dispatched = (open || (self.variant == Variant::ClosedP && id == 5))
            && (record_kept || self.variant == Variant::Unrecorded);
        self.log(
            s,
            Event::Read {
                id,
                instance,
                caller: if self.variant == Variant::Caller && id == 5 {
                    caller(INSTANCE)
                } else {
                    caller(instance)
                },
                invocation: invocation(instance),
                work: work(id),
                conduit,
                open,
                dispatched,
                record_kept,
            },
        );
        if !record_kept {
            self.log(s, Event::RecordLost(id));
        }
        s.nodes[E].data.status[id as usize] = if dispatched { 1 } else { 3 };
        if dispatched {
            s.send(c, E, K, Message::Read(id));
        }
        // Record failure is answered unavailable even in the deliberately dispatching mutant.
        if !dispatched || !record_kept {
            self.log(
                s,
                Event::End {
                    id,
                    outcome: if !record_kept {
                        Outcome::Unavailable
                    } else {
                        Outcome::Refused
                    },
                    responder: E,
                },
            );
        }
        if self.variant == Variant::EngineAnswer && id == 5 && dispatched {
            self.log(
                s,
                Event::End {
                    id,
                    outcome: Outcome::Succeeded,
                    responder: E,
                },
            );
            s.nodes[E].data.status[id as usize] = 2;
            s.nodes[E].data.values[id as usize] = Some(id);
        }
    }
    fn use_value(&self, s: &mut System<Self>, id: u8, dest: Work, place: Place) {
        self.log(
            s,
            Event::Use {
                read: Some(id),
                work: dest,
                place,
            },
        );
    }
    fn apply(&self, s: &mut System<Self>, id: u8) {
        let success = s.nodes[E].data.status[id as usize] == 2;
        if success || (self.variant == Variant::FailedRun && id == 1) {
            let value = s.nodes[E].data.values[id as usize].unwrap_or(id);
            // A header re-read only prepares the connection value. HTTP work is
            // logged when the request is actually dispatched, in advance().
            if !matches!(work(id), Work::Http(_)) {
                self.log(
                    s,
                    Event::Work {
                        work: work(id),
                        read: value,
                    },
                );
            }
            match id {
                0 | 5 => {
                    self.use_value(s, value, work(id), Place::Runner);
                    s.nodes[E].data.p_asked = true;
                    s.nodes[E].data.p_live = true;
                    s.nodes[E].data.p_run = Some(id);
                }
                1 => {
                    self.log(
                        s,
                        Event::Call {
                            run: true,
                            p_live: s.nodes[E].data.p_live,
                            p_run: s.nodes[E].data.p_run,
                        },
                    );
                    self.use_value(s, value, Work::Run(1), Place::Secrets);
                    s.nodes[I].data.run_value = Some(value);
                    s.nodes[E].data.i_running = true;
                    if self.variant == Variant::Environment {
                        self.log(
                            s,
                            Event::Use {
                                read: None,
                                work: Work::Run(1),
                                place: Place::Secrets,
                            },
                        );
                    }
                }
                2 => {
                    self.use_value(s, value, Work::Connection, Place::Header);
                    s.nodes[E].data.connection = true;
                    s.nodes[E].data.connection_epoch = s.nodes[E].data.epoch;
                    s.nodes[E].data.header = Some(value);
                }
                3 | 4 => s.nodes[E].data.header = Some(value),
                _ => {}
            }
        }
        s.nodes[E].data.values[id as usize] = None;
        if id == 5 {
            s.nodes[E].data.p_attempted = true;
        } else if id <= 2 || (!success && id <= 4) {
            s.nodes[E].data.stage += 1;
        }
    }
    fn advance(&self, c: &Config, s: &mut System<Self>) {
        if let Some(id) = self.current(s) {
            match s.nodes[E].data.status[id as usize] {
                0 => self.begin(c, s, id, true),
                2 | 3 => {
                    if id == 1 && s.nodes[E].data.status[1] == 2 && !s.nodes[E].data.p_live {
                        // A failed provider re-read prevents the dependent run as well.
                        if s.nodes[E].data.epoch > 0
                            && s.nodes[E].data.p_asked
                            && !s.nodes[E].data.p_attempted
                        {
                            return;
                        }
                        s.nodes[E].data.stage += 1;
                    } else {
                        self.apply(s, id);
                    }
                }
                _ => {}
            }
            return;
        }
        let d = &s.nodes[E].data;
        if d.epoch > 0 && d.i_running && !d.inspected && (d.p_live || d.p_attempted) {
            if d.p_live {
                self.log(
                    s,
                    Event::Call {
                        run: false,
                        p_live: true,
                        p_run: s.nodes[E].data.p_run,
                    },
                );
            }
            s.nodes[E].data.inspected = true;
            return;
        }
        match s.nodes[E].data.stage {
            2 => {
                s.nodes[E].data.stage += 1;
            }
            4 | 5 => {
                if let Some(read) = s.nodes[E].data.header {
                    let id = s.nodes[E].data.stage - 4;
                    self.log(
                        s,
                        Event::Work {
                            work: Work::Http(id),
                            read,
                        },
                    );
                    self.use_value(s, read, Work::Http(id), Place::Header);
                    self.log(
                        s,
                        Event::Http {
                            id,
                            read,
                            url: 0,
                            connection_epoch: s.nodes[E].data.connection_epoch,
                        },
                    );
                    s.send(c, E, H, Message::Http(id));
                }
                s.nodes[E].data.stage += 1;
            }
            6 | 7 => {
                let onward = s.nodes[E].data.stage == 7;
                self.log(
                    s,
                    Event::Get {
                        sender: if onward { O } else { I },
                        conduit: if onward { 3 } else { INSTANCE },
                        serving: onward.then_some(103),
                        answer: Outcome::Refused,
                        dispatched: onward && self.variant == Variant::Onward,
                    },
                );
                if onward && self.variant == Variant::Onward {
                    s.send(c, E, K, Message::Read(7));
                }
                s.nodes[E].data.stage += 1;
            }
            _ => {}
        }
    }
}
impl Protocol for Secrets {
    type Node = Data;
    type Message = Message;
    type Ledger = Ledger;
    type Local = Local;
    fn fault_allowed(&self, fault: Fault, src: usize, dst: usize) -> bool {
        match fault {
            Fault::Crash | Fault::Recovery => src == E,
            Fault::Loss | Fault::Reorder => match self.part {
                Part::LossK | Part::ReorderK => matches!((src, dst), (E, K) | (K, E)),
                Part::LossH | Part::ReorderH => matches!((src, dst), (E, H) | (H, E)),
                _ => false,
            },
            _ => false,
        }
    }
    fn initial(&self, c: &Config) -> System<Self> {
        State {
            nodes: c
                .nodes
                .iter()
                .map(|_| Node {
                    data: Data::default(),
                    status: Status::Running,
                })
                .collect(),
            links: vec![vec![]; c.nodes.len() * c.nodes.len()],
            cut: Default::default(),
            used: [0; 9],
            ledger: Ledger::default(),
        }
    }
    fn receive(&self, c: &Config, s: &mut System<Self>, _src: usize, dst: usize, msg: Message) {
        // Recovery observation is atomic with the first engine action, including delivery.
        if dst == E && s.nodes[E].data.sync {
            self.local_step(c, s, E, Local::RecoverLedger);
        }
        match msg {
            Message::Read(id) if id < 7 => s.send(c, K, E, Message::Value(id)),
            Message::Value(id) => {
                if s.nodes[E].data.status[id as usize] != 1 {
                    self.log(s, Event::Discard(id));
                } else {
                    self.log(
                        s,
                        Event::End {
                            id,
                            outcome: Outcome::Succeeded,
                            responder: K,
                        },
                    );
                    s.nodes[E].data.status[id as usize] = 2;
                    s.nodes[E].data.values[id as usize] = Some(id);
                    if self.variant == Variant::RecordValue {
                        self.use_value(s, id, work(id), Place::Record);
                    }
                }
            }
            Message::Http(id) => {
                if !s.nodes[H].data.redirect_sent {
                    s.nodes[H].data.redirect_sent = true;
                    self.log(s, Event::Redirect(id));
                    s.send(c, H, E, Message::Redirect(id));
                }
            }
            Message::Redirect(id) if self.variant == Variant::Redirect => {
                if let Some(read) = s.nodes[E].data.header {
                    self.log(
                        s,
                        Event::Work {
                            work: Work::Http(id),
                            read,
                        },
                    );
                    self.log(
                        s,
                        Event::Http {
                            id,
                            read,
                            url: 1,
                            connection_epoch: s.nodes[E].data.connection_epoch,
                        },
                    );
                    self.use_value(s, read, Work::Http(id), Place::Header);
                }
            }
            _ => {}
        }
    }
    fn timer_enabled(&self, _: &System<Self>, _: usize) -> bool {
        false
    }
    fn timer(&self, _: &Config, _: &mut System<Self>, _: usize) {}
    fn crash(&self, d: &mut Data) {
        d.sync = true;
        d.p_live = false;
        d.p_run = None;
        d.values = [None; 7];
        if self.variant != Variant::CachedHeader {
            d.header = None;
        }
    }
    fn recover(&self, d: &mut Data) {
        d.epoch += 1;
    }
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        if node == I
            && s.nodes[E].data.stage == 2
            && s.nodes[E].data.i_running
            && !s.nodes[I].data.local_restart
        {
            out.push(Local::RestartI);
        }
        if node != E {
            return;
        }
        let d = &s.nodes[E].data;
        if d.sync {
            out.push(Local::RecoverLedger);
            return;
        }
        if let Some(id) = self.current(s) {
            if d.status[id as usize] != 1 {
                out.push(Local::Advance);
            }
            if d.status[id as usize] == 1
                && matches!(self.part, Part::ReadFault | Part::RecoveryReadFault)
                && !d.read_fault
            {
                for outcome in [Outcome::Reason, Outcome::Failed, Outcome::Unknown] {
                    out.push(Local::ReadFault(outcome));
                }
            }
            if d.status[id as usize] == 0
                && matches!(self.part, Part::RecordFault | Part::RecoveryRecordFault)
                && !d.record_fault
            {
                out.push(Local::RecordFault);
            }
        } else if (d.stage < 8 && (d.stage != 2 || !d.i_running || s.nodes[I].data.local_restart))
            || (d.epoch > 0 && d.i_running && !d.inspected)
        {
            out.push(Local::Advance);
        }
        if ((self.part == Part::CloseP && d.stage >= 3)
            || (self.part == Part::ClosePEarly && d.stage < 3))
            && !d.closed[P as usize]
        {
            out.push(Local::Close(P));
        }
        if ((self.part == Part::CloseR && d.stage >= 3)
            || (self.part == Part::CloseREarly && d.stage < 3))
            && !d.closed[R as usize]
        {
            out.push(Local::Close(R));
        }
    }
    fn local_step(&self, c: &Config, s: &mut System<Self>, _: usize, action: Local) {
        match action {
            Local::Advance => self.advance(c, s),
            Local::RecoverLedger => {
                self.log(s, Event::Restart);
                for id in 0..7 {
                    let status = s.nodes[E].data.status[id];
                    if status == 1 {
                        self.log(
                            s,
                            Event::End {
                                id: id as u8,
                                outcome: Outcome::Unknown,
                                responder: E,
                            },
                        );
                        s.nodes[E].data.status[id] = 3;
                    } else if status == 2 && s.nodes[E].data.values[id].is_none() {
                        // A succeeded read not yet used belongs to interrupted setup work.
                        s.nodes[E].data.status[id] = 3;
                    }
                }
                if self.variant == Variant::CachedP && s.nodes[E].data.p_asked {
                    self.use_value(s, 0, Work::Run(5), Place::Cache);
                    s.nodes[E].data.status[5] = 2;
                    s.nodes[E].data.values[5] = Some(0);
                }
                s.nodes[E].data.sync = false;
            }
            Local::ReadFault(outcome) => {
                let id = self.current(s).unwrap();
                self.log(
                    s,
                    Event::End {
                        id,
                        outcome,
                        responder: if outcome == Outcome::Failed { K } else { E },
                    },
                );
                s.nodes[E].data.status[id as usize] = 3;
                s.nodes[E].data.read_fault = true;
            }
            Local::RecordFault => {
                let id = self.current(s).unwrap();
                s.nodes[E].data.record_fault = true;
                self.begin(c, s, id, false);
            }
            Local::Close(instance) => {
                s.nodes[E].data.closed[instance as usize] = true;
                self.log(s, Event::Close(instance));
            }
            Local::RestartI => {
                let read = s.nodes[I].data.run_value.unwrap();
                s.nodes[I].data.local_restart = true;
                self.log(s, Event::LocalRestart { read });
                self.use_value(s, read, Work::Run(1), Place::Secrets);
                if self.variant == Variant::RefreshI {
                    self.begin(c, s, 6, true);
                }
            }
        }
    }
    fn properties(&self) -> Vec<NamedProperty<Self>> {
        let mut properties = vec![
            NamedProperty {
                property: Property::<Harness<Self>>::always(PATH, |_, s| s.ledger.path()),
                promises: &["SEC-007", "SEC-010", "SEC-011", "SEC-020"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(STOP, |_, s| s.ledger.stop()),
                promises: &["SEC-012", "SEC-016"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(VALUES, |_, s| s.ledger.values()),
                promises: &["SEC-015", "SEC-017", "SEC-018"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(FIXED, |_, s| s.ledger.fixed()),
                promises: &["SEC-014", "PRV-026"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(HEADER, |_, s| s.ledger.header()),
                promises: &["SEC-013", "SEC-023"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(HTTP_WORK, |_, s| {
                    s.ledger.http_work_matches_dispatches()
                }),
                promises: &["harness"],
            },
        ];
        // Function pointers keep the harness's property conditions pure and ledger-only.
        let conditions: [Condition; 12] = [
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
        ];
        for (name, condition) in WITNESSES.into_iter().zip(conditions) {
            properties.push(NamedProperty {
                property: Property::sometimes(name, condition),
                promises: &["harness"],
            });
        }
        properties
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stateright::Model;
    // This table is registered before the first model-checker run; README has its full rows.
    fn expected(v: Variant, part: Part) -> (Vec<&'static str>, Vec<&'static str>) {
        let broken: Vec<_> = match v {
            Variant::Correct => vec![],
            Variant::Borrow => vec![PATH],
            Variant::ClosedP => vec![PATH, STOP, FIXED],
            Variant::CachedP => vec![VALUES, FIXED],
            Variant::RecordValue => vec![VALUES],
            Variant::CachedHeader => vec![VALUES, HEADER],
            _ => vec![v.target()],
        };
        let reached = |i| match i {
            0 => part == Part::ReadFault || part.recovery(),
            1 => matches!(part, Part::RecordFault | Part::RecoveryRecordFault),
            2..=4 | 10 => true,
            5 | 6 | 9 => part.recovery(),
            7 => matches!(part, Part::CloseP | Part::ClosePEarly) && v != Variant::ClosedP,
            8 => part == Part::CloseR,
            11 => part == Part::ReadFault || part.recovery(),
            _ => false,
        };
        (
            broken,
            WITNESSES
                .into_iter()
                .enumerate()
                .filter_map(|(i, n)| (!reached(i)).then_some(n))
                .collect(),
        )
    }
    #[test]
    fn full_runs_match_pre_registration() {
        for (v, p) in Part::ALL
            .into_iter()
            .map(|p| (Variant::Correct, p))
            .chain(Variant::BROKEN.into_iter().map(|v| (v, v.part())))
        {
            let report = check_all(model(v, p));
            println!("MC5b {v:?}/{p:?}: {report}");
            let mut observed = (report.broken, report.missing_witnesses);
            let mut registered = expected(v, p);
            // MC5b-001: preserve the original table and check its observed collateral failure.
            if v == Variant::Unrecorded {
                registered.0.push(HEADER);
                println!(
                    "MC5b difference Unrecorded/RecordFault: remote header expected holds, observed refuted"
                );
            }
            // MC5b-002: retaining the header prevents the fresh-read witness itself.
            if v == Variant::CachedHeader {
                registered.1.push(WITNESSES[9]);
                println!(
                    "MC5b difference CachedHeader/Recovery: fresh HTTP witness expected reached, observed unreachable"
                );
            }
            observed.0.sort();
            observed.1.sort();
            registered.0.sort();
            registered.1.sort();
            assert_eq!(observed, registered, "MC5b table mismatch {v:?}/{p:?}");
        }
    }
    #[test]
    fn first_named_refutations() {
        for v in Variant::BROKEN {
            println!("MC5b refutation {v:?}: {}", v.target());
            let m = model(v, v.part());
            let trace = assert_refuted(m.clone(), v.target());
            let path =
                stateright::Path::from_actions(&m, m.init_states().remove(0), &trace).unwrap();
            let mut state = path.last_state().clone();
            state
                .nodes
                .iter_mut()
                .for_each(|n| n.data = Data::default());
            assert!(!(m.property(v.target()).condition)(&m, &state));
            println!("MC5b first trace {v:?}: {} actions", trace.len());
        }
    }
    #[test]
    fn collateral_header_refutation() {
        println!("MC5b collateral Unrecorded: {HEADER}");
        let trace = assert_refuted(model(Variant::Unrecorded, Part::RecordFault), HEADER);
        println!("MC5b collateral first trace: {} actions", trace.len());
    }
    #[test]
    fn metadata_and_saved_state_contain_no_secret_values() {
        for part in Part::ALL {
            model(Variant::Correct, part).config.validate();
        }
        assert_eq!(SAFETY.len(), 5);
        let protocol = Secrets {
            variant: Variant::Correct,
            part: Part::Recovery,
        };
        let mut data = Data {
            values: [Some(0); 7],
            header: Some(2),
            p_live: true,
            connection: true,
            ..Data::default()
        };
        protocol.crash(&mut data);
        assert_eq!(data.values, [None; 7]);
        assert_eq!(data.header, None);
        assert!(data.connection);
        assert!(!data.p_live);
    }
}
