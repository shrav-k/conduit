//! Bounded searches of request attempts through the real core.
mod support;

use std::sync::Mutex;
use std::time::Duration;
use support::model::{Configuration, ONCE, Operations, Part, RIGHT, UNKNOWN};
use support::search::{FULL, Limits, Search, full, refutation, simulate};

static SEARCH_LOCK: Mutex<()> = Mutex::new(());
const SEEDS: [u64; 3] = [17, 237, 20261121];

fn correct(configuration: Configuration, parts: &[Part]) {
    let _lock = SEARCH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for &part in parts {
        let model = Operations::new(configuration, part);
        let report = full(model.clone(), FULL).unwrap();
        if let Some(&property) = report.refuted.first() {
            print!("{}", refutation(model, property, Search::Bfs));
            panic!("correct configuration refuted {property}");
        }
        assert!(
            report.missing.is_empty(),
            "unreached witnesses: {:?}",
            report.missing
        );
    }
}

fn control(configuration: Configuration, property: &'static str) {
    let _lock = SEARCH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let model = Operations::new(configuration, Part::T);
    let report = full(model.clone(), FULL).unwrap();
    assert_eq!(report.refuted, [property]);
    assert!(
        report.missing.is_empty(),
        "unreached witnesses: {:?}",
        report.missing
    );
    print!("{}", refutation(model, property, Search::Bfs));
}

#[test]
fn plain_holds() {
    correct(Configuration::Plain, &[Part::T, Part::L]);
}

#[test]
fn deduplicating_holds() {
    correct(
        Configuration::Deduplicating,
        &[Part::T, Part::L, Part::R, Part::LR],
    );
}

#[test]
fn seeded_simulations_hold() {
    let _lock = SEARCH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for (configuration, part) in [
        (Configuration::Plain, Part::L),
        (Configuration::Deduplicating, Part::LR),
    ] {
        for seed in SEEDS {
            simulate(Operations::new(configuration, part), seed);
        }
    }
}

#[test]
fn at_most_once_control_refutes() {
    control(Configuration::OControl, ONCE);
}

#[test]
fn no_wrong_outcome_control_refutes() {
    control(Configuration::WControl, RIGHT);
}

#[test]
fn unknown_stays_unknown_control_refutes() {
    control(Configuration::UControl, UNKNOWN);
}

#[test]
fn printouts_reproduce() {
    let _lock = SEARCH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let model = Operations::new(Configuration::WControl, Part::T);
    for search in [Search::Bfs, Search::Simulation(SEEDS[1])] {
        let first = refutation(model.clone(), RIGHT, search);
        let second = refutation(model.clone(), RIGHT, search);
        assert_eq!(first, second);
        print!("{first}");
    }
}

#[test]
fn truncated_search_gives_no_verdict() {
    let _lock = SEARCH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let model = Operations::new(Configuration::Plain, Part::T);
    for (limits, reason) in [
        (Limits { unique: 2, ..FULL }, "unique-state ceiling"),
        (
            Limits {
                time: Duration::ZERO,
                ..FULL
            },
            "deadline",
        ),
    ] {
        let Err(error) = full(model.clone(), limits) else {
            panic!("truncated search returned a verdict");
        };
        assert!(
            error.starts_with(reason) && error.ends_with("No verdict."),
            "{error}"
        );
        println!("{error}");
    }
}
