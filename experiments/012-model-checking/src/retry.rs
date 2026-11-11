//! Harness self-test only: these rules claim no Conduit promise.
use crate::harness::*;
use stateright::Property;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

pub const AT_MOST_ONCE: &str = "harness: at most once";
pub const NO_FALSE_CONFIRMATION: &str = "harness: no false confirmation";
const ID: u8 = 0;
const SENDER: usize = 0;
const RECEIVER: usize = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Saved,
    VolatileIds,
    AnswerFirst,
}
impl Variant {
    pub const ALL: [Self; 3] = [Self::Saved, Self::VolatileIds, Self::AnswerFirst];
    pub fn broken(self) -> Option<&'static str> {
        match self {
            Self::Saved => None,
            Self::VolatileIds => Some(AT_MOST_ONCE),
            Self::AnswerFirst => Some(NO_FALSE_CONFIRMATION),
        }
    }
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Data {
    pub saved_ids: BTreeSet<u8>,
    pub volatile_ids: BTreeSet<u8>,
    pub pending: BTreeSet<u8>,
    pub confirmed: bool,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Request(u8),
    Confirmation(u8),
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Local {
    Apply(u8),
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Ledger {
    pub effects: BTreeMap<u8, u8>,
    /// Each receipt records the effect count *then*, preserving an early false confirmation
    /// even if an effect is applied later or the sender subsequently crashes.
    pub confirmations: Vec<(u8, u8)>,
}
#[derive(Clone)]
pub struct Retry {
    pub variant: Variant,
}

/// One request ID; one initial send plus at most two timer retries and one wire copy.
/// Both nodes may suffer any node fault, and either direction may suffer any wire fault.
pub fn configuration() -> Config {
    Config {
        name: "two machines, all faults",
        nodes: vec!["sender", "receiver"],
        budgets: [1, 1, 1, 1, 1, 1, 1, 1, 2],
        disabled_reasons: [None; 9],
        messages_per_link: 4,
        unique_state_ceiling: 1_000_000,
        timeout: Duration::from_secs(15),
        bounds: "one ID (0), two machines, one initial request, at most two retries and one copy; at most four effects and four confirmations",
    }
}
pub fn model(variant: Variant) -> Harness<Retry> {
    Harness::new(configuration(), Retry { variant })
}
impl Protocol for Retry {
    type Node = Data;
    type Message = Message;
    type Ledger = Ledger;
    type Local = Local;
    fn initial(&self, config: &Config) -> System<Self> {
        let mut state = State {
            nodes: vec![
                Node {
                    data: Data::default(),
                    status: Status::Running
                };
                2
            ],
            links: vec![Vec::new(); 4],
            cut: BTreeSet::new(),
            used: [0; 9],
            ledger: Ledger::default(),
        };
        state.send(config, SENDER, RECEIVER, Message::Request(ID));
        state
    }
    fn receive(
        &self,
        config: &Config,
        s: &mut System<Self>,
        _src: usize,
        dst: usize,
        msg: Message,
    ) {
        match (dst, msg) {
            (RECEIVER, Message::Request(id)) => {
                if s.nodes[dst].data.volatile_ids.insert(id) {
                    if self.variant != Variant::VolatileIds {
                        s.nodes[dst].data.saved_ids.insert(id);
                    }
                    if self.variant == Variant::AnswerFirst {
                        s.nodes[dst].data.pending.insert(id);
                    } else {
                        *s.ledger.effects.entry(id).or_default() += 1;
                    }
                }
                s.send(config, RECEIVER, SENDER, Message::Confirmation(id));
            }
            (SENDER, Message::Confirmation(id)) => {
                s.nodes[SENDER].data.confirmed = true;
                s.ledger
                    .confirmations
                    .push((id, s.ledger.effects.get(&id).copied().unwrap_or(0)));
            }
            _ => unreachable!("messages have fixed destinations"),
        }
    }
    fn timer_enabled(&self, s: &System<Self>, node: usize) -> bool {
        node == SENDER && !s.nodes[node].data.confirmed
    }
    fn timer(&self, config: &Config, s: &mut System<Self>, _node: usize) {
        s.send(config, SENDER, RECEIVER, Message::Request(ID));
    }
    fn crash(&self, node: &mut Data) {
        node.volatile_ids.clear();
        node.pending.clear();
        node.confirmed = false;
    }
    fn recover(&self, node: &mut Data) {
        node.volatile_ids = node.saved_ids.clone();
    }
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        out.extend(
            s.nodes[node]
                .data
                .pending
                .iter()
                .map(|id| Local::Apply(*id)),
        );
    }
    fn local_step(&self, _config: &Config, s: &mut System<Self>, node: usize, action: Local) {
        let Local::Apply(id) = action;
        assert!(s.nodes[node].data.pending.remove(&id));
        *s.ledger.effects.entry(id).or_default() += 1;
    }
    fn properties(&self) -> Vec<NamedProperty<Self>> {
        vec![
            NamedProperty {
                property: Property::<Harness<Self>>::always(AT_MOST_ONCE, |_, s| {
                    s.ledger.effects.values().all(|n| *n <= 1)
                }),
                promises: &["harness"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(NO_FALSE_CONFIRMATION, |_, s| {
                    s.ledger.confirmations.iter().all(|(_, n)| *n > 0)
                }),
                promises: &["harness"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "harness: effect applied",
                    |_, s| !s.ledger.effects.is_empty(),
                ),
                promises: &["harness"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "harness: confirmation received",
                    |_, s| !s.ledger.confirmations.is_empty(),
                ),
                promises: &["harness"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(
                    "harness: receiver lost after effect",
                    |_, s| s.nodes[RECEIVER].status == Status::Lost && !s.ledger.effects.is_empty(),
                ),
                promises: &["harness"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stateright::Model;

    #[test]
    fn saved_ids_hold() {
        let report = assert_holds(model(Variant::Saved));
        println!("Saved: {report}; holds");
    }
    #[test]
    fn volatile_ids_break_only_at_most_once() {
        let report = check_all(model(Variant::VolatileIds));
        assert_eq!(report.broken, vec![AT_MOST_ONCE]);
        assert!(report.missing_witnesses.is_empty());
        println!("VolatileIds: {report}; refuted only at most once");
        let m = model(Variant::VolatileIds);
        let trace = assert_refuted(m.clone(), AT_MOST_ONCE);
        assert_eq!(trace.len(), 5);
        assert!(trace.contains(&Action::Crash(RECEIVER)));
        assert!(trace.contains(&Action::Recover(RECEIVER)));
        let mut no_crash = configuration();
        no_crash.budgets[Fault::Crash as usize] = 0;
        no_crash.budgets[Fault::Recovery as usize] = 0;
        no_crash.disabled_reasons[Fault::Crash as usize] =
            Some("prove the seeded failure requires a crash");
        no_crash.disabled_reasons[Fault::Recovery as usize] = Some("no crash to recover from");
        let report = assert_holds(Harness::new(
            no_crash,
            Retry {
                variant: Variant::VolatileIds,
            },
        ));
        println!("VolatileIds without crash/recovery (control): {report}; holds");
    }
    #[test]
    fn answering_first_breaks_only_no_false_confirmation() {
        let report = check_all(model(Variant::AnswerFirst));
        assert_eq!(report.broken, vec![NO_FALSE_CONFIRMATION]);
        assert!(report.missing_witnesses.is_empty());
        println!("AnswerFirst: {report}; refuted only no false confirmation");
        let m = model(Variant::AnswerFirst);
        let trace = assert_refuted(m.clone(), NO_FALSE_CONFIRMATION);
        assert_eq!(trace.len(), 2);
        let path = stateright::Path::from_actions(&m, m.init_states().remove(0), &trace).unwrap();
        assert!(path.last_state().ledger.effects.is_empty());
    }
    fn step(m: &Harness<Retry>, s: &mut System<Retry>, a: Action<Local>) {
        let mut actions = Vec::new();
        m.actions(s, &mut actions);
        assert!(actions.contains(&a), "action not enabled: {a:?}");
        *s = m.next_state(s, a).unwrap();
    }
    #[test]
    fn queues_hold_across_cuts_crashes_and_permanent_loss() {
        let m = model(Variant::Saved);
        let mut s = m.init_states().remove(0);
        step(&m, &mut s, Action::Cut(0, 1));
        step(&m, &mut s, Action::Timer(0)); // sent while cut, held alongside original
        let held = s.links.clone();
        let mut actions = Vec::new();
        m.actions(&s, &mut actions);
        assert!(!actions.iter().any(|a| matches!(a, Action::Deliver { .. })));
        step(&m, &mut s, Action::Crash(1));
        step(&m, &mut s, Action::Heal(0, 1));
        assert_eq!(s.links, held);
        actions.clear();
        m.actions(&s, &mut actions);
        assert!(
            !actions
                .iter()
                .any(|a| matches!(a, Action::Deliver { dst: 1, .. }))
        );
        step(&m, &mut s, Action::Recover(1));
        step(
            &m,
            &mut s,
            Action::Deliver {
                src: 0,
                dst: 1,
                index: 0,
            },
        );
        let ledger = s.ledger.clone();
        step(&m, &mut s, Action::Lose(1));
        assert_eq!(s.ledger, ledger);
        actions.clear();
        m.actions(&s, &mut actions);
        assert!(!actions.iter().any(|a| matches!(
            a,
            Action::Recover(1)
                | Action::Timer(1)
                | Action::Crash(1)
                | Action::Lose(1)
                | Action::Deliver { dst: 1, .. }
        )));
        step(
            &m,
            &mut s,
            Action::Drop {
                src: 0,
                dst: 1,
                index: 0,
            },
        );
        assert_eq!(s.ledger, ledger);
    }
    #[test]
    fn equal_copies_have_positions_and_fault_budgets_are_global() {
        let m = model(Variant::Saved);
        let mut s = m.init_states().remove(0);
        step(
            &m,
            &mut s,
            Action::Duplicate {
                src: 0,
                dst: 1,
                index: 0,
            },
        );
        assert_eq!(s.links[1].len(), 2);
        step(
            &m,
            &mut s,
            Action::Deliver {
                src: 0,
                dst: 1,
                index: 1,
            },
        );
        assert_eq!(s.links[1].len(), 1);
        assert_eq!(s.used[Fault::Reorder as usize], 1);
        let mut actions = Vec::new();
        m.actions(&s, &mut actions);
        assert!(!actions.iter().any(|a| matches!(
            a,
            Action::Duplicate { .. } | Action::Deliver { index: 1, .. }
        )));
        step(
            &m,
            &mut s,
            Action::Deliver {
                src: 0,
                dst: 1,
                index: 0,
            },
        );
        assert_eq!(s.ledger.effects[&ID], 1);
        step(&m, &mut s, Action::Crash(1));
        step(&m, &mut s, Action::Recover(1));
        assert_eq!(s.used[Fault::Crash as usize], 1);
        actions.clear();
        m.actions(&s, &mut actions);
        assert!(!actions.iter().any(|a| matches!(a, Action::Crash(_))));
    }
    #[test]
    fn zero_reorder_budget_means_fifo_and_metadata_is_present() {
        let mut config = configuration();
        config.budgets[Fault::Reorder as usize] = 0;
        config.disabled_reasons[Fault::Reorder as usize] = Some("FIFO regression");
        let m = Harness::new(
            config,
            Retry {
                variant: Variant::Saved,
            },
        );
        let mut s = m.init_states().remove(0);
        step(&m, &mut s, Action::Timer(0));
        let mut actions = Vec::new();
        m.actions(&s, &mut actions);
        assert!(actions.contains(&Action::Deliver {
            src: 0,
            dst: 1,
            index: 0
        }));
        assert!(!actions.contains(&Action::Deliver {
            src: 0,
            dst: 1,
            index: 1
        }));
        assert!(
            m.protocol
                .properties()
                .iter()
                .all(|p| p.promises == ["harness"])
        );
        for fault in Fault::ALL {
            assert_eq!(
                m.properties().iter().any(|p| p.name == fault.witness()),
                fault != Fault::Reorder
            );
        }
        assert_eq!(threads_from(None), 2);
        assert_eq!(threads_from(Some("1")), 1);
        assert_eq!(threads_from(Some("8")), 2);
        for property in m.protocol.properties() {
            if property.property.expectation == stateright::Expectation::Always {
                assert!(
                    Variant::ALL
                        .iter()
                        .any(|v| v.broken() == Some(property.property.name))
                );
            }
        }
    }
    #[test]
    fn later_effect_and_sender_crash_do_not_erase_false_confirmation() {
        let m = model(Variant::AnswerFirst);
        let mut s = m.init_states().remove(0);
        step(
            &m,
            &mut s,
            Action::Deliver {
                src: 0,
                dst: 1,
                index: 0,
            },
        );
        step(
            &m,
            &mut s,
            Action::Deliver {
                src: 1,
                dst: 0,
                index: 0,
            },
        );
        step(&m, &mut s, Action::Local(1, Local::Apply(ID)));
        step(&m, &mut s, Action::Crash(0));
        assert_eq!(s.ledger.effects[&ID], 1);
        assert_eq!(s.ledger.confirmations, vec![(ID, 0)]);
        assert!(!(m.property(NO_FALSE_CONFIRMATION).condition)(&m, &s));
    }

    #[test]
    fn ceiling_and_timeout_never_count_as_holds() {
        let mut m = model(Variant::Saved);
        m.config.unique_state_ceiling = 1;
        assert!(std::panic::catch_unwind(|| assert_holds(m)).is_err());
        let mut m = model(Variant::Saved);
        m.config.timeout = Duration::from_nanos(1);
        assert!(std::panic::catch_unwind(|| assert_holds(m)).is_err());
    }
}
