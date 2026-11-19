//! A state census: distinct states counted over a harness's transitions, reading no property.
use crate::harness::{Action, Harness, Protocol, System};
use stateright::Model;
use std::collections::{BTreeMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::{Duration, Instant};

pub struct Limits {
    pub states: usize,
    pub seconds: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Stop {
    Exhausted,
    StateCap,
    TimeCap,
}

impl Display for Stop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Exhausted => "exhausted",
            Self::StateCap => "state cap",
            Self::TimeCap => "time cap",
        })
    }
}

pub struct Census {
    pub states: usize,
    pub stop: Stop,
    pub seconds: f64,
    pub classes: Vec<usize>,
    pub projections: Vec<usize>,
    pub kinds: BTreeMap<String, (u64, u64)>,
}

type Prune<'a, P> = dyn Fn(&System<P>, &mut Vec<Action<<P as Protocol>::Local>>) + 'a;
type Projection<'a, P> = dyn Fn(&System<P>) -> u64 + 'a;

fn fingerprint<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn append_variant(label: &mut String, value: &impl Debug) {
    let start = label.len();
    write!(label, "{value:?}").unwrap();
    if let Some(end) = label[start..].find(['(', '{', ' ']) {
        label.truncate(start + end);
    }
}

fn describe_kind<P: Protocol>(
    label: &mut String,
    model: &Harness<P>,
    state: &System<P>,
    action: &Action<P::Local>,
) {
    let initial = |node: usize| model.config.nodes[node].chars().next().unwrap();
    label.clear();
    append_variant(label, action);
    match action {
        Action::Deliver { src, dst, index }
        | Action::Drop { src, dst, index }
        | Action::Duplicate { src, dst, index } => {
            write!(label, " {}->{} ", initial(*src), initial(*dst)).unwrap();
            append_variant(label, &state.links[state.link(*src, *dst)][*index]);
        }
        Action::Local(node, local) => {
            write!(label, " {} ", initial(*node)).unwrap();
            append_variant(label, local);
        }
        _ => {}
    }
}

struct Frame<P: Protocol> {
    state: System<P>,
    actions: std::vec::IntoIter<Action<P::Local>>,
}

pub fn census<P: Protocol>(
    model: &Harness<P>,
    prune: &Prune<'_, P>,
    class: &dyn Fn(&System<P>) -> usize,
    projections: &[&Projection<'_, P>],
    limits: Limits,
) -> Census {
    let started = Instant::now();
    let timeout = Duration::from_secs(limits.seconds);
    let mut seen = HashSet::with_capacity(limits.states);
    let mut distinct: Vec<HashSet<u64>> = projections
        .iter()
        .map(|_| HashSet::with_capacity(limits.states))
        .collect();
    let mut classes = Vec::new();
    let mut kinds = BTreeMap::<String, (u64, u64)>::new();
    let mut stack: Vec<Frame<P>> = Vec::new();
    let mut label = String::new();
    let mut roots = model.init_states().into_iter();

    let stop = loop {
        if seen.len() == limits.states {
            break Stop::StateCap;
        }
        if started.elapsed() >= timeout {
            break Stop::TimeCap;
        }
        let (next, kind) = if let Some(frame) = stack.last_mut() {
            let Some(action) = frame.actions.next() else {
                stack.pop();
                continue;
            };
            describe_kind(&mut label, model, &frame.state, &action);
            let kind = if let Some(counts) = kinds.get_mut(label.as_str()) {
                counts
            } else {
                kinds.entry(label.clone()).or_default()
            };
            kind.0 += 1;
            (model.next_state(&frame.state, action), Some(kind))
        } else if let Some(root) = roots.next() {
            (Some(root), None)
        } else {
            break Stop::Exhausted;
        };
        let Some(state) = next else { continue };
        if !seen.insert(fingerprint(&state)) {
            continue;
        }
        if let Some(kind) = kind {
            kind.1 += 1;
        }
        let index = class(&state);
        if classes.len() <= index {
            classes.resize(index + 1, 0);
        }
        classes[index] += 1;
        for (project, values) in projections.iter().zip(&mut distinct) {
            values.insert(project(&state));
        }
        if seen.len() < limits.states {
            let mut actions = Vec::new();
            model.actions(&state, &mut actions);
            prune(&state, &mut actions);
            stack.push(Frame {
                state,
                actions: actions.into_iter(),
            });
        }
    };
    Census {
        states: seen.len(),
        stop,
        seconds: started.elapsed().as_secs_f64(),
        classes,
        projections: distinct.iter().map(HashSet::len).collect(),
        kinds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn census_counts_as_the_checker() {
        println!("MC9a measurement test census_counts_as_the_checker");
        for part in crate::records::record::Part::ALL {
            let model =
                crate::records::record::model(crate::records::record::Variant::Correct, part);
            let measured = census(
                &model,
                &|_, _| {},
                &|_| 0,
                &[],
                Limits {
                    states: 1_000_000,
                    seconds: 60,
                },
            );
            let checked = crate::harness::check_all(model);
            println!(
                "MC9a calibration Records|Correct/{part:?}|{}|{}",
                measured.states, checked.unique_states
            );
            assert_eq!(measured.stop, Stop::Exhausted, "{part:?}");
            assert_eq!(measured.states, checked.unique_states, "{part:?}");
        }
    }
}
