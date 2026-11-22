use super::model::{Action, Ledger, MAX_TRACE_STATES, Operations, SAFETY, World};
use stateright::{Checker, Expectation, HasDiscoveries, Model, UniformChooser};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write;
use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Limits {
    pub unique: usize,
    pub time: Duration,
}

pub const FULL: Limits = Limits {
    unique: 200_000,
    time: Duration::from_secs(300),
};
const SIMULATION_TARGET: usize = 32;

#[derive(Default)]
struct Admission {
    seen: HashSet<u64>,
    truncated: Option<&'static str>,
}

pub struct SearchGuard {
    started: Instant,
    limits: Limits,
    admission: Mutex<Admission>,
    pub target: Option<&'static str>,
    first: Mutex<Option<Trace>>,
    found_first: AtomicBool,
}

impl SearchGuard {
    fn new(limits: Limits, target: Option<&'static str>) -> Arc<Self> {
        Arc::new(Self {
            started: Instant::now(),
            limits,
            admission: Mutex::new(Admission::default()),
            target,
            first: Mutex::new(None),
            found_first: AtomicBool::new(false),
        })
    }

    pub fn capture(&self, name: &'static str, actions: &[Action], ledger: &Ledger) {
        if self.target == Some(name) && !self.found_first.swap(true, Ordering::Relaxed) {
            // Stateright 0.31.0 can replace discoveries while finishing its batch.
            *self.first.lock().unwrap() = Some(Trace {
                actions: actions.to_vec(),
                ledger: ledger.clone(),
            });
        }
    }

    pub fn admit(&self, state: &World) -> bool {
        if self.found_first.load(Ordering::Relaxed) {
            return false;
        }
        let mut admission = self.admission.lock().unwrap();
        if admission.truncated.is_some() {
            return false;
        }
        if self.started.elapsed() >= self.limits.time {
            admission.truncated = Some("deadline");
            return false;
        }
        // Keep fingerprints so the guard does not retain every core snapshot.
        let hash = BuildHasherDefault::<DefaultHasher>::default().hash_one(state);
        if admission.seen.insert(hash) && admission.seen.len() >= self.limits.unique {
            admission.truncated = Some("unique-state ceiling");
            return false;
        }
        true
    }

    fn check(&self, count: usize, depth: usize) -> Result<Duration, String> {
        let elapsed = self.started.elapsed();
        let admission = self.admission.lock().unwrap();
        let truncated = admission
            .truncated
            .or_else(|| (elapsed >= self.limits.time).then_some("deadline"));
        if let Some(reason) = truncated {
            return Err(format!(
                "{reason} reached with {count} states at depth {depth} after {:.3}s. No verdict.",
                elapsed.as_secs_f64(),
            ));
        }
        Ok(elapsed)
    }
}

fn guarded(mut model: Operations, limits: Limits) -> (Operations, Arc<SearchGuard>) {
    let guard = SearchGuard::new(limits, None);
    model.guard = Some(Arc::clone(&guard));
    (model, guard)
}

pub struct FullReport {
    pub refuted: Vec<&'static str>,
    pub missing: Vec<&'static str>,
}

pub fn full(model: Operations, limits: Limits) -> Result<FullReport, String> {
    let (model, guard) = guarded(model, limits);
    let checker = model
        .checker()
        .threads(1)
        .finish_when(HasDiscoveries::AnyOf(BTreeSet::new()))
        .spawn_bfs()
        .join();
    let count = checker.unique_state_count();
    let depth = checker.max_depth();
    let elapsed = guard.check(count, depth)?;
    assert_eq!(
        count,
        guard.admission.lock().unwrap().seen.len(),
        "state counts disagree"
    );
    let discoveries = checker.discoveries();
    let refuted: Vec<_> = SAFETY
        .into_iter()
        .filter(|name| discoveries.contains_key(name))
        .collect();
    let missing: Vec<_> = checker
        .model()
        .properties()
        .into_iter()
        .filter(|p| p.expectation == Expectation::Sometimes && !discoveries.contains_key(p.name))
        .map(|p| p.name)
        .collect();
    let witnesses = checker
        .model()
        .properties()
        .into_iter()
        .filter(|p| p.expectation == Expectation::Sometimes)
        .count();
    let verdicts = SAFETY.map(|name| {
        if refuted.contains(&name) {
            "refuted"
        } else {
            "holds"
        }
    });
    println!(
        "Full {}/{:?}: O {}, W {}, U {} | {count} unique | depth {depth} | {:.3}s | witnesses {}/{}",
        checker.model().configuration.label(),
        checker.model().part,
        verdicts[0],
        verdicts[1],
        verdicts[2],
        elapsed.as_secs_f64(),
        witnesses - missing.len(),
        witnesses,
    );
    Ok(FullReport { refuted, missing })
}

#[derive(Clone, Copy)]
pub enum Search {
    Bfs,
    Simulation(u64),
}

struct Trace {
    actions: Vec<Action>,
    ledger: Ledger,
}

pub fn refutation(mut model: Operations, name: &'static str, search: Search) -> String {
    let guard = SearchGuard::new(FULL, Some(name));
    model.guard = Some(Arc::clone(&guard));
    let builder = model
        .checker()
        .threads(1)
        .finish_when(HasDiscoveries::AnyOf(BTreeSet::from([name])));
    let (count, depth, config, part) = match search {
        Search::Bfs => {
            let checker = builder.spawn_bfs().join();
            (
                checker.unique_state_count(),
                checker.max_depth(),
                checker.model().configuration,
                checker.model().part,
            )
        }
        Search::Simulation(seed) => {
            let checker = builder
                .timeout(FULL.time)
                .target_state_count(128)
                .spawn_simulation(seed, UniformChooser)
                .join();
            (
                checker.state_count(),
                checker.max_depth(),
                checker.model().configuration,
                checker.model().part,
            )
        }
    };
    guard.check(count, depth).unwrap();
    if let Search::Simulation(seed) = search {
        println!(
            "Refutation simulation {}/{part:?}: seed {seed} | {count} visits | depth {depth}",
            config.label()
        );
    }
    let Trace { actions, ledger } = guard
        .first
        .lock()
        .unwrap()
        .take()
        .expect("expected refutation");
    let search = match search {
        Search::Bfs => "bfs".into(),
        Search::Simulation(seed) => format!("simulation seed {seed}"),
    };
    let mut printout = format!(
        "{name} refuted: {}/{part:?}, {search}, {} actions\n",
        config.label(),
        actions.len()
    );
    for (index, action) in actions.iter().enumerate() {
        writeln!(printout, "{}. {action}", index + 1).unwrap();
    }
    writeln!(printout, "Invocation ledger").unwrap();
    write!(printout, "{ledger}").unwrap();
    printout
}

pub fn simulate(model: Operations, seed: u64) {
    let (guarded_model, guard) = guarded(model.clone(), FULL);
    let checker = guarded_model
        .checker()
        .threads(1)
        .timeout(FULL.time)
        .target_state_count(SIMULATION_TARGET)
        .finish_when(HasDiscoveries::AnyOf(BTreeSet::new()))
        .spawn_simulation(seed, UniformChooser)
        .join();
    let visited = checker.state_count();
    let depth = checker.max_depth();
    guard.check(visited, depth).unwrap();
    assert!(visited >= SIMULATION_TARGET);
    assert!(visited < SIMULATION_TARGET + MAX_TRACE_STATES);
    let discoveries = checker.discoveries();
    for property in SAFETY {
        if discoveries.contains_key(property) {
            print!("{}", refutation(model, property, Search::Simulation(seed)));
            panic!("correct configuration refuted in a seeded simulation");
        }
    }
    println!(
        "Simulation {}/{:?}: seed {seed} | {visited} visits | depth {depth}",
        checker.model().configuration.label(),
        checker.model().part
    );
}
