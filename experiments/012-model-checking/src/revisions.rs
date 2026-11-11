//! Revision checks and engine-record serialization, independent of engine code.
use crate::harness::*;
use crate::mc2::{self, Part};
use stateright::Property;
use std::collections::BTreeSet;

pub const CHECKED: &str = "REV-002 REV-011: revision checked";
pub const SERIAL: &str = "REV-012: one change at a time";
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Checked,
    Unchecked,
    Concurrent,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Answer {
    Succeeded,
    Unknown,
    Conflict,
    Invalid,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Change { id: u8, revision: Option<u8> },
    Dispatch(u8),
    Reply(u8),
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Data {
    engine: bool,
    revision: u8,          // saved
    pending: BTreeSet<u8>, // saved invocation records
    ended: BTreeSet<u8>,
    recovering: bool,
    stage: u8,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Request {
        id: u8,
        named: Option<u8>,
        current: u8,
        dispatched: bool,
        answer: Option<Answer>,
    },
    Dispatch {
        id: u8,
        named: Option<u8>,
        current: u8,
    },
    Effect(u8),
    End {
        id: u8,
        answer: Answer,
        revision: u8,
        source: &'static str,
    },
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Ledger {
    pub events: Vec<Event>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Local {
    Next,
    Recover,
    LostReply(u8),
}
#[derive(Clone)]
pub struct Revisions {
    pub variant: Variant,
    pub part: Part,
}
pub fn model(variant: Variant, part: Part) -> Harness<Revisions> {
    let mut c = mc2::configuration(part, false);
    c.bounds = "four changes, two initially naming r0 together; then no revision; then first success revision; one instance; at most two timers; three messages per link";
    Harness::new(c, Revisions { variant, part })
}
impl Revisions {
    fn end(&self, s: &mut System<Self>, id: u8, answer: Answer, source: &'static str) {
        let e = &mut s.nodes[1].data;
        assert!(e.pending.remove(&id));
        e.ended.insert(id);
        if answer == Answer::Succeeded {
            e.revision += 1;
        }
        s.ledger.events.push(Event::End {
            id,
            answer,
            revision: e.revision,
            source,
        });
    }
    fn recover_pending(&self, s: &mut System<Self>) {
        if s.nodes[1].data.recovering {
            s.nodes[1].data.recovering = false;
            for id in s.nodes[1].data.pending.clone() {
                self.end(s, id, Answer::Unknown, "recovery");
            }
        }
    }
    fn first_success(s: &System<Self>) -> Option<u8> {
        s.ledger.events.iter().find_map(|e| match e {
            Event::End {
                answer: Answer::Succeeded,
                revision,
                ..
            } => Some(*revision),
            _ => None,
        })
    }
}
impl Protocol for Revisions {
    type Node = Data;
    type Message = Message;
    type Ledger = Ledger;
    type Local = Local;
    fn fault_allowed(&self, f: Fault, src: usize, dst: usize) -> bool {
        if f == Fault::Crash {
            src == 1
        } else {
            mc2::allowed(f, src, dst)
        }
    }
    fn initial(&self, c: &Config) -> System<Self> {
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
        s.nodes[1].data.engine = true;
        for id in 0..2 {
            s.send(
                c,
                0,
                1,
                Message::Change {
                    id,
                    revision: Some(0),
                },
            );
        }
        s
    }
    fn receive(&self, c: &Config, s: &mut System<Self>, _src: usize, dst: usize, msg: Message) {
        if dst == 1 {
            self.recover_pending(s);
        }
        match msg {
            Message::Change { id, revision } => {
                let current = s.nodes[1].data.revision;
                let answer = if self.variant != Variant::Unchecked && revision.is_none() {
                    Some(Answer::Invalid)
                } else if (self.variant != Variant::Unchecked && revision != Some(current))
                    || (self.variant != Variant::Concurrent && !s.nodes[1].data.pending.is_empty())
                {
                    Some(Answer::Conflict)
                } else {
                    None
                };
                s.ledger.events.push(Event::Request {
                    id,
                    named: revision,
                    current,
                    dispatched: answer.is_none(),
                    answer,
                });
                if answer.is_none() {
                    s.nodes[1].data.pending.insert(id);
                    s.ledger.events.push(Event::Dispatch {
                        id,
                        named: revision,
                        current,
                    });
                    s.send(c, 1, 2, Message::Dispatch(id));
                }
            }
            Message::Dispatch(id) => {
                s.ledger.events.push(Event::Effect(id));
                s.send(c, 2, 1, Message::Reply(id));
            }
            Message::Reply(id) => {
                if s.nodes[1].data.pending.contains(&id) {
                    self.end(s, id, Answer::Succeeded, "reply");
                }
            }
        }
    }
    fn timer_enabled(&self, s: &System<Self>, node: usize) -> bool {
        node == 1 && !s.nodes[1].data.pending.is_empty()
    }
    fn timer(&self, _c: &Config, s: &mut System<Self>, _node: usize) {
        self.recover_pending(s);
        for id in s.nodes[1].data.pending.clone() {
            self.end(s, id, Answer::Unknown, "timer");
        }
    }
    fn crash(&self, _node: &mut Data) {}
    fn recover(&self, node: &mut Data) {
        if node.engine {
            node.recovering = true;
        }
    }
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        if node == 0
            && s.links[s.link(0, 1)].is_empty()
            && (s.nodes[0].data.stage == 0
                || (s.nodes[0].data.stage == 1 && Self::first_success(s).is_some()))
        {
            out.push(Local::Next);
        }
        if node == 1 {
            if s.nodes[1].data.recovering {
                out.push(Local::Recover);
            }
            for id in &s.nodes[1].data.pending {
                if s.ledger.events.contains(&Event::Effect(*id))
                    && !s.links.iter().flatten().any(|m| *m == Message::Reply(*id))
                {
                    out.push(Local::LostReply(*id));
                }
            }
        }
    }
    fn local_step(&self, c: &Config, s: &mut System<Self>, node: usize, action: Local) {
        match action {
            Local::Next => {
                let stage = s.nodes[node].data.stage;
                s.nodes[node].data.stage += 1;
                s.send(
                    c,
                    0,
                    1,
                    Message::Change {
                        id: stage + 2,
                        revision: if stage == 0 {
                            None
                        } else {
                            Self::first_success(s)
                        },
                    },
                );
            }
            Local::Recover => self.recover_pending(s),
            Local::LostReply(id) => self.end(s, id, Answer::Unknown, "lost reply"),
        }
    }
    fn properties(&self) -> Vec<NamedProperty<Self>> {
        let mut p = vec![
            NamedProperty {
                property: Property::<Harness<Self>>::always(CHECKED, |_, s| {
                    s.ledger.events.iter().all(|e| match e {
                        Event::Dispatch { named, current, .. } => *named == Some(*current),
                        Event::Request {
                            named,
                            current,
                            dispatched,
                            answer,
                            ..
                        } if named.is_none() => !dispatched && *answer == Some(Answer::Invalid),
                        Event::Request {
                            named,
                            current,
                            dispatched,
                            answer,
                            ..
                        } if *named != Some(*current) => {
                            !dispatched && *answer == Some(Answer::Conflict)
                        }
                        _ => true,
                    })
                }),
                promises: &["REV-002", "REV-011"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(SERIAL, |_, s| {
                    let mut flight = BTreeSet::new();
                    for e in &s.ledger.events {
                        match e {
                            Event::Dispatch { id, .. } => {
                                flight.insert(*id);
                                if flight.len() > 1 {
                                    return false;
                                }
                            }
                            Event::End { id, .. } => {
                                flight.remove(id);
                            }
                            _ => {}
                        }
                    }
                    true
                }),
                promises: &["REV-012"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "REV-012: change succeeded",
                    |_, s| Self::first_success(s).is_some(),
                ),
                promises: &["REV-012"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "REV-011: missing revision requested",
                    |_, s| {
                        s.ledger
                            .events
                            .iter()
                            .any(|e| matches!(e, Event::Request { named: None, .. }))
                    },
                ),
                promises: &["REV-011"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "REV-002: followup requested",
                    |_, s| {
                        s.ledger
                            .events
                            .iter()
                            .any(|e| matches!(e, Event::Request { id: 3, .. }))
                    },
                ),
                promises: &["REV-002"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "REV-012: unknown on timer",
                    |_, s| {
                        s.ledger.events.iter().any(|e| {
                            matches!(
                                e,
                                Event::End {
                                    source: "timer",
                                    ..
                                }
                            )
                        })
                    },
                ),
                promises: &["REV-012"],
            },
        ];
        for (part, name, condition) in [
            (
                Part::Loss,
                "REV-012: unknown on lost reply",
                (|_: &Harness<Self>, s: &System<Self>| {
                    s.ledger.events.iter().any(|e| {
                        matches!(
                            e,
                            Event::End {
                                source: "lost reply",
                                ..
                            }
                        )
                    })
                }) as fn(&Harness<Self>, &System<Self>) -> bool,
            ),
            (
                Part::Crash,
                "REV-012: unknown on recovery",
                (|_: &Harness<Self>, s: &System<Self>| {
                    s.ledger.events.iter().any(|e| {
                        matches!(
                            e,
                            Event::End {
                                source: "recovery",
                                ..
                            }
                        )
                    })
                }) as fn(&Harness<Self>, &System<Self>) -> bool,
            ),
        ] {
            if self.part == part {
                p.push(NamedProperty {
                    property: Property::sometimes(name, condition),
                    promises: &["REV-012", "IMP-007"],
                });
            }
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_runs_match_pre_registration() {
        for v in [Variant::Checked, Variant::Unchecked, Variant::Concurrent] {
            for part in [Part::Timers, Part::Loss, Part::Reorder, Part::Crash] {
                let report = check_all(model(v, part));
                println!("MC2 revisions {v:?}/{part:?}: {report}");
                assert_eq!(
                    report.broken,
                    match v {
                        Variant::Checked => vec![],
                        Variant::Unchecked => vec![CHECKED],
                        Variant::Concurrent => vec![SERIAL],
                    },
                    "pre-registration difference {v:?}/{part:?}"
                );
                assert!(
                    report.missing_witnesses.is_empty(),
                    "{v:?}/{part:?}: {:?}",
                    report.missing_witnesses
                );
            }
        }
    }
    #[test]
    fn shortest_named_refutations() {
        for (v, target) in [(Variant::Unchecked, CHECKED), (Variant::Concurrent, SERIAL)] {
            println!("MC2 revision refutation {v:?}: {target}");
            let trace = assert_refuted(model(v, Part::Timers), target);
            println!("MC2 revision refutation {v:?}: {} actions", trace.len());
        }
    }
}
