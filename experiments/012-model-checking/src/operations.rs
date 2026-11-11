//! Operation identities, attempts and reconciliation, read from the promises.
use crate::harness::*;
use crate::mc2::{self, Part};
use stateright::Property;
use std::collections::{BTreeMap, BTreeSet};

pub const ONCE: &str = "OPS-018 OPS-008 OPS-012 OPS-024: at most once";
pub const RIGHT: &str = "OPS-012 OPS-014: no wrong outcome";
pub const UNKNOWN: &str = "OPS-013 IMP-007: unknown stays unknown";
pub const REPEAT: &str = "OPS-009 OPS-032: repeat answered without dispatch";
pub const CONTENT: &str = "OPS-010: different contents conflict";
pub const KEYLESS: &str = "OPS-011: keyless stay apart";
pub const SCOPE: &str = "OPS-008: sending instances stay apart";
pub const RECON_OK: &str = "OPS-014 OPS-030: reconciled succeeded";
pub const RECON_FAIL: &str = "OPS-014 OPS-030: reconciled failed";
const A: usize = 0;
const ENGINE: usize = 1;
const IMPLEMENTATION: usize = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Plain,
    Deduplicating,
    Fenced,
    BlindReplay,
    VolatileIndex,
    TimerFailed,
    LateReply,
    KeyOnly,
    RepeatSucceeded,
}
impl Variant {
    pub const ALL: [Self; 9] = [
        Self::Plain,
        Self::Deduplicating,
        Self::Fenced,
        Self::BlindReplay,
        Self::VolatileIndex,
        Self::TimerFailed,
        Self::LateReply,
        Self::KeyOnly,
        Self::RepeatSucceeded,
    ];
    pub fn target(self) -> Option<&'static str> {
        match self {
            Self::Plain | Self::BlindReplay | Self::VolatileIndex => Some(ONCE),
            Self::TimerFailed => Some(RIGHT),
            Self::LateReply => Some(UNKNOWN),
            Self::KeyOnly => Some(CONTENT),
            Self::RepeatSucceeded => Some(REPEAT),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Outcome {
    Progress,
    Unknown,
    Succeeded,
    Failed,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Source {
    Reply,
    Timer,
    LostReply,
    ImplementationCrash,
    LostMachine,
    Recovery,
    Reconcile,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Answer {
    Succeeded,
    Failed,
    Conflict,
    Unknown,
    Invalid,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Request {
    pub sender: u8,
    pub contents: u8,
    pub keyless: bool,
    pub script_request: u8, // observer label, not operation contents or identity
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Request(Request),
    Dispatch { id: u8, contents: u8, attempt: u8 },
    Reply { id: u8, attempt: u8 },
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Entry {
    contents: u8,
    attempt: u8,
    outcome: Outcome,
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Data {
    engine: bool,
    index: BTreeMap<u8, Entry>, // saved except in the deliberately volatile variant
    applied: BTreeSet<u8>,      // implementation's saved deduplication index
    stage: u8,                  // A's saved script counter
    recovery_pending: bool,
    next_keyless: u8,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Dispatch {
        contents: u8,
        first_contents: u8,
        attempt: u8,
    },
    Effect {
        contents: u8,
        attempt: u8,
    },
    ReplyEmitted {
        attempt: u8,
    },
    Outcome {
        value: Outcome,
        effects_then: usize,
        source: Source,
    },
    Answer {
        value: Answer,
        source: Source,
    },
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Decision {
    request: Request,
    id: u8,
    first_contents: Option<u8>,
    before: Option<Outcome>,
    dispatched: bool,
    answer: Option<Answer>,
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Ledger {
    deduplicates: bool, // immutable contract fact, independent of the engine index
    // Per-identity histories retain every event, while independent identities commute.
    pub histories: BTreeMap<u8, Vec<Event>>,
    decisions: BTreeMap<u8, Vec<Decision>>,
}
impl Ledger {
    fn effects(&self, id: u8) -> usize {
        self.histories
            .get(&id)
            .into_iter()
            .flatten()
            .filter(|e| matches!(e, Event::Effect { .. }))
            .count()
    }
    fn first(&self, id: u8) -> Option<u8> {
        self.histories.get(&id)?.iter().find_map(|e| match e {
            Event::Dispatch { contents, .. } => Some(*contents),
            _ => None,
        })
    }
    fn current(&self, id: u8) -> Option<Outcome> {
        self.histories.get(&id)?.iter().rev().find_map(|e| match e {
            Event::Dispatch { .. } => Some(Outcome::Progress),
            Event::Outcome { value, .. } => Some(*value),
            _ => None,
        })
    }
    fn record(&mut self, id: u8, value: Outcome, source: Source) {
        let effects_then = self.effects(id);
        self.histories.entry(id).or_default().push(Event::Outcome {
            value,
            effects_then,
            source,
        });
    }
    fn source(&self, wanted: Source) -> bool {
        self.histories.values().flatten().any(|e| {
            matches!(e,
            Event::Outcome { value: Outcome::Unknown, source, .. } if *source == wanted)
        })
    }
    fn reconciled(&self, wanted: Outcome) -> bool {
        self.histories.values().flatten().any(|e| {
            matches!(e,
            Event::Outcome { value, source: Source::Reconcile, .. } if *value == wanted)
        })
    }
    fn keyless_apart(&self) -> bool {
        let witnessed = |id: u8, decisions: &[Decision], label| {
            decisions.iter().any(|d| {
                d.id == id
                    && d.dispatched
                    && d.request.keyless
                    && d.request.script_request == label
                    && self.histories.get(&id).into_iter().flatten().any(|event| {
                        matches!(event, Event::Effect { contents, .. } if *contents == d.request.contents)
                    })
            })
        };
        self.decisions.iter().any(|(&first, ds)| {
            witnessed(first, ds, 1)
                && self
                    .decisions
                    .iter()
                    .any(|(&second, ds)| first != second && witnessed(second, ds, 2))
        })
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Local {
    Next,
    Reconcile(u8),
    Unknown(u8, Source),
    Recover,
}
#[derive(Clone)]
pub struct Operations {
    pub variant: Variant,
    pub part: Part,
    pub scope: bool,
}
pub fn model(variant: Variant, part: Part, scope: bool) -> Harness<Operations> {
    Harness::new(
        mc2::configuration(part, scope),
        Operations {
            variant,
            part,
            scope,
        },
    )
}
impl Operations {
    fn decide(&self, s: &mut System<Self>, id: u8, value: Outcome, source: Source) {
        s.nodes[ENGINE].data.index.get_mut(&id).unwrap().outcome = value;
        s.ledger.record(id, value, source);
        let answer = match value {
            Outcome::Succeeded => Answer::Succeeded,
            Outcome::Failed => Answer::Failed,
            Outcome::Unknown => Answer::Unknown,
            Outcome::Progress => unreachable!(),
        };
        s.ledger
            .histories
            .entry(id)
            .or_default()
            .push(Event::Answer {
                value: answer,
                source,
            });
    }
    fn recovered(&self, s: &mut System<Self>) {
        if s.nodes[ENGINE].data.recovery_pending {
            s.nodes[ENGINE].data.recovery_pending = false;
            let ids: Vec<_> = s.nodes[ENGINE]
                .data
                .index
                .iter()
                .filter_map(|(id, e)| (e.outcome == Outcome::Progress).then_some(*id))
                .collect();
            for id in ids {
                self.decide(s, id, Outcome::Unknown, Source::Recovery);
            }
        }
    }
    fn send_request(&self, c: &Config, s: &mut System<Self>, r: Request) {
        s.send(c, r.sender as usize, ENGINE, Message::Request(r));
    }
    fn queued_dispatch(s: &System<Self>, id: u8) -> bool {
        s.links
            .iter()
            .flatten()
            .any(|m| matches!(m, Message::Dispatch { id: x, .. } if *x == id))
    }
    fn quiescent(s: &System<Self>) -> bool {
        s.links.iter().all(Vec::is_empty)
            && s.nodes[ENGINE]
                .data
                .index
                .values()
                .all(|e| e.outcome != Outcome::Progress)
    }
}
impl Protocol for Operations {
    type Node = Data;
    type Message = Message;
    type Ledger = Ledger;
    type Local = Local;
    fn fault_allowed(&self, f: Fault, src: usize, dst: usize) -> bool {
        mc2::allowed(f, src, dst)
    }
    fn initial(&self, c: &Config) -> System<Self> {
        let n = c.nodes.len();
        let mut s = State {
            nodes: vec![
                Node {
                    data: Data::default(),
                    status: Status::Running
                };
                n
            ],
            links: vec![vec![]; n * n],
            cut: BTreeSet::new(),
            used: [0; 9],
            ledger: Ledger {
                deduplicates: self.variant == Variant::Deduplicating,
                ..Ledger::default()
            },
        };
        s.nodes[ENGINE].data.engine = true;
        s.nodes[ENGINE].data.next_keyless = 2;
        self.send_request(
            c,
            &mut s,
            Request {
                sender: 0,
                contents: 1,
                keyless: false,
                script_request: 0,
            },
        );
        if self.scope {
            self.send_request(
                c,
                &mut s,
                Request {
                    sender: 3,
                    contents: 1,
                    keyless: false,
                    script_request: 0,
                },
            );
        }
        s
    }
    fn receive(&self, c: &Config, s: &mut System<Self>, _src: usize, dst: usize, msg: Message) {
        if dst == ENGINE {
            self.recovered(s);
        }
        match msg {
            Message::Request(request) => {
                let id = if request.keyless {
                    let id = s.nodes[ENGINE].data.next_keyless;
                    s.nodes[ENGINE].data.next_keyless += 1;
                    id
                } else {
                    u8::from(request.sender != 0)
                };
                let mut d = Decision {
                    request,
                    id,
                    first_contents: s.ledger.first(id),
                    before: s.ledger.current(id),
                    dispatched: false,
                    answer: None,
                };
                let entry = s.nodes[ENGINE].data.index.get(&id).cloned();
                let dispatch = if let Some(e) = &entry {
                    if e.contents != request.contents && self.variant != Variant::KeyOnly {
                        d.answer = Some(Answer::Conflict);
                        false
                    } else {
                        match e.outcome {
                            Outcome::Progress => {
                                d.answer = Some(Answer::Conflict);
                                false
                            }
                            Outcome::Succeeded if self.variant != Variant::RepeatSucceeded => {
                                d.answer = Some(Answer::Succeeded);
                                false
                            }
                            Outcome::Unknown
                                if !matches!(
                                    self.variant,
                                    Variant::Deduplicating | Variant::BlindReplay
                                ) =>
                            {
                                d.answer = Some(Answer::Unknown);
                                false
                            }
                            _ => true,
                        }
                    }
                } else {
                    true
                };
                if dispatch {
                    d.dispatched = true;
                    let attempt = entry.map_or(1, |e| e.attempt + 1);
                    let first_contents = s.ledger.first(id).unwrap_or(request.contents);
                    s.ledger
                        .histories
                        .entry(id)
                        .or_default()
                        .push(Event::Dispatch {
                            contents: request.contents,
                            first_contents,
                            attempt,
                        });
                    s.nodes[ENGINE].data.index.insert(
                        id,
                        Entry {
                            contents: request.contents,
                            attempt,
                            outcome: Outcome::Progress,
                        },
                    );
                    s.send(
                        c,
                        ENGINE,
                        IMPLEMENTATION,
                        Message::Dispatch {
                            id,
                            contents: request.contents,
                            attempt,
                        },
                    );
                }
                s.ledger.decisions.entry(id).or_default().push(d);
            }
            Message::Dispatch {
                id,
                contents,
                attempt,
            } => {
                let fresh = s.nodes[IMPLEMENTATION].data.applied.insert(id);
                if self.variant != Variant::Deduplicating || fresh {
                    s.ledger
                        .histories
                        .entry(id)
                        .or_default()
                        .push(Event::Effect { contents, attempt });
                }
                // Deduplicating an attempt suppresses its effect, not its reply.
                s.ledger
                    .histories
                    .entry(id)
                    .or_default()
                    .push(Event::ReplyEmitted { attempt });
                s.send(c, IMPLEMENTATION, ENGINE, Message::Reply { id, attempt });
            }
            Message::Reply { id, attempt } => {
                if s.nodes[ENGINE].data.index.get(&id).is_some_and(|e| {
                    e.attempt == attempt
                        && (e.outcome == Outcome::Progress
                            || (e.outcome == Outcome::Unknown
                                && self.variant == Variant::LateReply))
                }) {
                    self.decide(s, id, Outcome::Succeeded, Source::Reply);
                    // The engine gives its result to the caller atomically; caller wire omitted.
                }
            }
        }
    }
    fn timer_enabled(&self, s: &System<Self>, node: usize) -> bool {
        (node == A && s.nodes[A].data.stage <= 1 && s.links[s.link(A, ENGINE)].len() < 2)
            || (node == ENGINE
                && s.nodes[ENGINE]
                    .data
                    .index
                    .values()
                    .any(|e| e.outcome == Outcome::Progress))
    }
    fn timer(&self, c: &Config, s: &mut System<Self>, node: usize) {
        if node == A {
            s.nodes[A].data.stage = 1;
            self.send_request(
                c,
                s,
                Request {
                    sender: 0,
                    contents: 1,
                    keyless: false,
                    script_request: 0,
                },
            );
        } else {
            self.recovered(s);
            let ids: Vec<_> = s.nodes[ENGINE]
                .data
                .index
                .iter()
                .filter_map(|(id, e)| (e.outcome == Outcome::Progress).then_some(*id))
                .collect();
            for id in ids {
                self.decide(
                    s,
                    id,
                    if self.variant == Variant::TimerFailed {
                        Outcome::Failed
                    } else {
                        Outcome::Unknown
                    },
                    Source::Timer,
                );
            }
        }
    }
    fn crash(&self, node: &mut Data) {
        if node.engine && self.variant == Variant::VolatileIndex {
            node.index.clear();
        }
    }
    fn recover(&self, node: &mut Data) {
        if node.engine {
            node.recovery_pending = true;
        }
    }
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        if node == A && (1..=4).contains(&s.nodes[A].data.stage) && Self::quiescent(s) {
            out.push(Local::Next);
        }
        if node != ENGINE {
            return;
        }
        if s.nodes[ENGINE].data.recovery_pending {
            out.push(Local::Recover);
            return;
        }
        for (&id, e) in &s.nodes[ENGINE].data.index {
            if e.outcome == Outcome::Unknown
                && (self.variant != Variant::Fenced
                    || s.ledger.effects(id) > 0
                    || !Self::queued_dispatch(s, id))
            {
                out.push(Local::Reconcile(id));
            }
            if e.outcome == Outcome::Progress {
                match s.nodes[IMPLEMENTATION].status {
                    Status::Crashed => out.push(Local::Unknown(id, Source::ImplementationCrash)),
                    Status::Lost => out.push(Local::Unknown(id, Source::LostMachine)),
                    _ => {}
                }
                let reply = s.links.iter().flatten().any(|m| {
                    matches!(m,
                    Message::Reply { id: x, attempt } if *x == id && *attempt == e.attempt)
                });
                let emitted = s.ledger.histories.get(&id).into_iter().flatten().any(
                    |event| matches!(event, Event::ReplyEmitted { attempt } if *attempt == e.attempt),
                );
                if emitted && !reply {
                    out.push(Local::Unknown(id, Source::LostReply));
                }
            }
        }
    }
    fn local_step(&self, c: &Config, s: &mut System<Self>, node: usize, action: Local) {
        match action {
            Local::Next => {
                let stage = s.nodes[node].data.stage;
                s.nodes[node].data.stage += 1;
                if stage >= 2 {
                    self.send_request(
                        c,
                        s,
                        Request {
                            sender: 0,
                            contents: if stage == 2 { 2 } else { 1 },
                            keyless: stage >= 3,
                            script_request: stage.saturating_sub(2),
                        },
                    );
                }
            }
            Local::Recover => self.recovered(s),
            Local::Unknown(id, source) => self.decide(s, id, Outcome::Unknown, source),
            Local::Reconcile(id) => self.decide(
                s,
                id,
                if s.ledger.effects(id) > 0 {
                    Outcome::Succeeded
                } else {
                    Outcome::Failed
                },
                Source::Reconcile,
            ),
        }
    }
    fn properties(&self) -> Vec<NamedProperty<Self>> {
        let mut p = vec![
            NamedProperty {
                property: Property::<Harness<Self>>::always(ONCE, |_, s| {
                    (0..2).all(|id| s.ledger.effects(id) <= 1)
                }),
                promises: &["OPS-018", "OPS-008", "OPS-012", "OPS-024"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(RIGHT, |_, s| {
                    s.ledger.histories.values().flatten().all(|e| match e {
                        Event::Outcome {
                            value: Outcome::Succeeded,
                            effects_then,
                            ..
                        } => *effects_then > 0,
                        Event::Outcome {
                            value: Outcome::Failed,
                            effects_then,
                            ..
                        } => *effects_then == 0,
                        _ => true,
                    })
                }),
                promises: &["OPS-012", "OPS-014"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(UNKNOWN, |_, s| {
                    s.ledger.histories.values().all(|events| {
                        let mut unknown = false;
                        for e in events {
                            match e {
                                Event::Dispatch { .. } => {
                                    if unknown && !s.ledger.deduplicates {
                                        return false;
                                    }
                                    unknown = false;
                                }
                                Event::Outcome { value, source, .. } => {
                                    if unknown
                                        && *value != Outcome::Unknown
                                        && *source != Source::Reconcile
                                    {
                                        return false;
                                    }
                                    unknown = *value == Outcome::Unknown;
                                }
                                _ => {}
                            }
                        }
                        true
                    })
                }),
                promises: &["OPS-013", "IMP-007"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(REPEAT, |_, s| {
                    s.ledger.decisions.values().flatten().all(|d| {
                        if d.first_contents != Some(d.request.contents) {
                            return true;
                        }
                        match d.before {
                            Some(Outcome::Succeeded) => {
                                !d.dispatched && d.answer == Some(Answer::Succeeded)
                            }
                            Some(Outcome::Progress) => {
                                !d.dispatched && d.answer == Some(Answer::Conflict)
                            }
                            _ => true,
                        }
                    })
                }),
                promises: &["OPS-009", "OPS-032"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(CONTENT, |_, s| {
                    s.ledger.decisions.values().flatten().all(|d| {
                        d.first_contents
                            .is_none_or(|first| first == d.request.contents)
                            || (!d.dispatched && d.answer == Some(Answer::Conflict))
                    })
                }),
                promises: &["OPS-010"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(KEYLESS, |_, s| {
                    s.ledger.keyless_apart()
                }),
                promises: &["OPS-011"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(RECON_OK, |_, s| {
                    s.ledger.reconciled(Outcome::Succeeded)
                }),
                promises: &["OPS-014", "OPS-030"],
            },
        ];
        if self.variant != Variant::Fenced || self.part == Part::Loss {
            p.push(NamedProperty {
                property: Property::<Harness<Self>>::sometimes(RECON_FAIL, |_, s| {
                    s.ledger.reconciled(Outcome::Failed)
                }),
                promises: &["OPS-014", "OPS-030"],
            });
        }
        if self.scope {
            p.push(NamedProperty {
                property: Property::<Harness<Self>>::sometimes(SCOPE, |_, s| {
                    [0, 1].iter().all(|id| {
                        s.ledger.decisions.get(id).is_some_and(|ds| {
                            ds.first()
                                .is_some_and(|d| d.dispatched && d.before.is_none())
                        })
                    })
                }),
                promises: &["OPS-008"],
            });
        }
        let sources: &[(Source, &str)] = &[
            (Source::Timer, "OPS-013: unknown on timer"),
            (Source::LostReply, "OPS-013: unknown on lost reply"),
            (
                Source::ImplementationCrash,
                "OPS-013: unknown on implementation crash",
            ),
            (Source::LostMachine, "OPS-013: unknown on lost machine"),
            (Source::Recovery, "IMP-007: unknown on recovery"),
        ];
        for &(source, name) in sources {
            let enabled = match source {
                Source::Timer => self.variant != Variant::TimerFailed,
                Source::LostReply => self.part == Part::Loss,
                Source::ImplementationCrash => self.part == Part::Crash,
                Source::LostMachine => self.part == Part::Lost,
                Source::Recovery => {
                    self.part == Part::Crash && self.variant != Variant::VolatileIndex
                }
                _ => false,
            };
            if enabled {
                let condition: fn(&Harness<Self>, &System<Self>) -> bool = match source {
                    Source::Timer => |_, s| s.ledger.source(Source::Timer),
                    Source::LostReply => |_, s| s.ledger.source(Source::LostReply),
                    Source::ImplementationCrash => {
                        |_, s| s.ledger.source(Source::ImplementationCrash)
                    }
                    Source::LostMachine => |_, s| s.ledger.source(Source::LostMachine),
                    Source::Recovery => |_, s| s.ledger.source(Source::Recovery),
                    _ => unreachable!(),
                };
                p.push(NamedProperty {
                    property: Property::sometimes(name, condition),
                    promises: &["OPS-013", "IMP-007"],
                });
            }
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stateright::Model;
    fn step(m: &Harness<Operations>, s: &mut System<Operations>, a: Action<Local>) {
        let mut actions = Vec::new();
        m.actions(s, &mut actions);
        assert!(actions.contains(&a), "action not enabled: {a:?}");
        *s = m.next_state(s, a).unwrap();
    }
    fn lost_reply_enabled(m: &Harness<Operations>, s: &System<Operations>) -> bool {
        let mut actions = Vec::new();
        m.actions(s, &mut actions);
        actions.contains(&Action::Local(ENGINE, Local::Unknown(0, Source::LostReply)))
    }
    #[test]
    fn lost_deduplicated_retry_reply_becomes_unknown() {
        let m = model(Variant::Deduplicating, Part::Loss, false);
        let mut s = m.init_states().remove(0);
        let deliver = |src, dst| Action::Deliver { src, dst, index: 0 };
        step(&m, &mut s, deliver(A, ENGINE));
        assert!(!lost_reply_enabled(&m, &s)); // no reply has been emitted
        step(&m, &mut s, Action::Timer(ENGINE));
        step(&m, &mut s, Action::Timer(A));
        step(&m, &mut s, deliver(A, ENGINE)); // attempt 2 is current
        step(&m, &mut s, deliver(ENGINE, IMPLEMENTATION)); // attempt 1 applies
        step(&m, &mut s, deliver(IMPLEMENTATION, ENGINE)); // stale reply ignored
        assert_eq!(s.nodes[ENGINE].data.index[&0].outcome, Outcome::Progress);
        assert!(!lost_reply_enabled(&m, &s)); // old attempt's completion is insufficient
        step(&m, &mut s, deliver(ENGINE, IMPLEMENTATION)); // deduplicated attempt 2
        assert_eq!(s.ledger.effects(0), 1);
        assert!(!lost_reply_enabled(&m, &s)); // current reply is still queued
        step(
            &m,
            &mut s,
            Action::Drop {
                src: IMPLEMENTATION,
                dst: ENGINE,
                index: 0,
            },
        );
        assert_eq!(s.used[Fault::Timer as usize], 2);
        step(
            &m,
            &mut s,
            Action::Local(ENGINE, Local::Unknown(0, Source::LostReply)),
        );
        assert_eq!(s.ledger.current(0), Some(Outcome::Unknown));
        step(&m, &mut s, Action::Local(ENGINE, Local::Reconcile(0)));
        assert!(s.ledger.reconciled(Outcome::Succeeded));
        assert_eq!(s.ledger.effects(0), 1);
    }
    #[test]
    fn keyless_witness_requires_two_dispatched_identities_with_effects() {
        let m = model(Variant::Deduplicating, Part::Timers, false);
        let mut s = m.init_states().remove(0);
        let decision = |label, dispatched| Decision {
            request: Request {
                sender: 0,
                contents: 1,
                keyless: true,
                script_request: label,
            },
            id: 2,
            first_contents: None,
            before: None,
            dispatched,
            answer: None,
        };
        let witness = m.property(KEYLESS).condition;
        s.ledger.histories.insert(
            2,
            vec![Event::Effect {
                contents: 1,
                attempt: 1,
            }],
        );
        s.ledger
            .decisions
            .insert(2, vec![decision(1, true), decision(2, false)]);
        assert!(!witness(&m, &s)); // second request accidentally answered as a repeat
        s.ledger.decisions.get_mut(&2).unwrap()[1].dispatched = true;
        assert!(!witness(&m, &s)); // even two dispatches cannot share one identity/effect
        let mut second = s.ledger.decisions.get_mut(&2).unwrap().pop().unwrap();
        second.id = 3;
        s.ledger.decisions.insert(3, vec![second]);
        assert!(!witness(&m, &s)); // separate identity alone does not supply its effect
        s.ledger.histories.insert(
            3,
            vec![Event::Effect {
                contents: 1,
                attempt: 1,
            }],
        );
        assert!(witness(&m, &s));
        s.ledger.decisions.get_mut(&3).unwrap()[0].dispatched = false;
        assert!(!witness(&m, &s)); // effects must belong to dispatched scripted requests
    }
    fn expected(v: Variant) -> Vec<&'static str> {
        let mut e = match v {
            Variant::Deduplicating | Variant::Fenced => vec![],
            Variant::Plain | Variant::BlindReplay => vec![ONCE],
            Variant::VolatileIndex => vec![ONCE, REPEAT, CONTENT],
            Variant::TimerFailed => vec![ONCE, RIGHT],
            Variant::LateReply => vec![ONCE, UNKNOWN],
            Variant::KeyOnly => vec![ONCE, CONTENT],
            Variant::RepeatSucceeded => vec![ONCE, REPEAT],
        };
        e.sort();
        e
    }
    #[test]
    fn full_runs_match_pre_registration() {
        for v in Variant::ALL {
            let parts = match v {
                Variant::Plain | Variant::Deduplicating | Variant::Fenced => Part::ALL.to_vec(),
                Variant::VolatileIndex | Variant::TimerFailed => vec![Part::Crash],
                _ => vec![Part::Timers],
            };
            for part in parts {
                let report = check_all(model(v, part, false));
                println!("MC2 operations {v:?}/{part:?}: {report}");
                let mut observed = report.broken.clone();
                observed.sort();
                let registered = expected(v);
                let mut accepted_observation = registered.clone();
                if matches!(v, Variant::BlindReplay | Variant::VolatileIndex) {
                    accepted_observation.push(UNKNOWN);
                    accepted_observation.sort();
                    println!(
                        "MC2 difference {v:?}/{part:?}: preregistered unknown stays unknown H; observed F (unauthorized plain replay)"
                    );
                }
                assert_eq!(
                    observed, accepted_observation,
                    "unrecorded pre-registration difference {v:?}/{part:?}; registered {registered:?}"
                );
                assert!(
                    report.missing_witnesses.is_empty(),
                    "{v:?}/{part:?}: {:?}",
                    report.missing_witnesses
                );
            }
        }
        let report = assert_holds(model(Variant::Deduplicating, Part::Timers, true));
        println!("MC2 operations Deduplicating/Scope: {report}");
    }
    #[test]
    fn shortest_named_refutations() {
        for v in Variant::ALL {
            if let Some(target) = v.target() {
                let part = if matches!(v, Variant::VolatileIndex | Variant::TimerFailed) {
                    Part::Crash
                } else {
                    Part::Timers
                };
                println!("MC2 refutation {v:?}: {target}");
                let trace = assert_refuted(model(v, part, false), target);
                println!("MC2 refutation {v:?}: {} actions", trace.len());
                // The external history must retain a failure even if all engine indexes vanish.
                let m = model(v, part, false);
                let path =
                    stateright::Path::from_actions(&m, m.init_states().remove(0), &trace).unwrap();
                let mut state = path.last_state().clone();
                state.nodes[ENGINE].data.index.clear();
                state.nodes[IMPLEMENTATION].data.applied.clear();
                assert!(!(m.property(target).condition)(&m, &state));
                if v == Variant::Plain {
                    assert_eq!(trace.len(), 7);
                }
                if v == Variant::VolatileIndex {
                    assert!(trace.contains(&Action::Crash(ENGINE)));
                    assert!(trace.contains(&Action::Recover(ENGINE)));
                }
                if matches!(v, Variant::BlindReplay | Variant::VolatileIndex) {
                    println!("MC2 difference refutation {v:?}: {UNKNOWN}");
                    let trace = assert_refuted(model(v, part, false), UNKNOWN);
                    println!("MC2 difference refutation {v:?}: {} actions", trace.len());
                }
            }
        }
    }
}
