//! Shared, explicitly split MC2 bounds. Not engine code.
use crate::harness::{Config, Fault};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Part {
    Timers,
    Loss,
    Reorder,
    Duplication,
    Partition,
    Crash,
    Lost,
}
impl Part {
    pub const ALL: [Self; 7] = [
        Self::Timers,
        Self::Loss,
        Self::Reorder,
        Self::Duplication,
        Self::Partition,
        Self::Crash,
        Self::Lost,
    ];
}
pub fn configuration(part: Part, scope: bool) -> Config {
    let mut budgets = [0; 9];
    budgets[Fault::Timer as usize] = 2;
    for fault in match part {
        Part::Timers => vec![],
        Part::Loss => vec![Fault::Loss],
        Part::Reorder => vec![Fault::Reorder],
        Part::Duplication => vec![Fault::Duplication],
        Part::Partition => vec![Fault::Cut, Fault::Heal],
        Part::Crash => vec![Fault::Crash, Fault::Recovery],
        Part::Lost => vec![Fault::LostMachine],
    } {
        budgets[fault as usize] = 1;
    }
    let mut nodes = vec!["A", "engine", "implementation"];
    if scope {
        nodes.push("B");
    }
    Config {
        name: "MC2 split faults",
        nodes,
        budgets,
        disabled_reasons: std::array::from_fn(|i| {
            (budgets[i] == 0)
                .then_some("split configuration: this fault is enabled in its named part")
        }),
        messages_per_link: 3,
        unique_state_ceiling: 200_000,
        timeout: Duration::from_secs(10),
        bounds: "one caller/target; one key; c1/c2; two equal keyless requests; at most two timers; one of each enabled fault; serialized script suffix; B only in scope",
    }
}
/// Request copies only; dispatch transport delivers each send at most once.
pub fn allowed(fault: Fault, src: usize, dst: usize) -> bool {
    match fault {
        Fault::Duplication => (src == 0 || src == 3) && dst == 1,
        Fault::Cut => (src == 1 && dst == 2) || (src == 2 && dst == 1),
        Fault::Crash => src == 1 || src == 2,
        Fault::LostMachine => src == 2,
        _ => true,
    }
}
