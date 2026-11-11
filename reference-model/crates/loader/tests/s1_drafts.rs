//! S1's maintained example passes the strict loader. This asserts syntax
//! and document shape only; it does not install packages, resolve references or run S1.

mod common;

use common::{repository, specification};
use conduit_loader::read_project;

#[test]
fn maintained_s1_example_is_accepted() {
    let directory = repository().join("reference-model/examples/s1");
    let project = read_project(&directory, specification())
        .unwrap_or_else(|faults| panic!("{}: {faults}", directory.display()));
    assert!(project.manifest.is_some(), "{}", directory.display());
    assert!(!project.documents.is_empty(), "{}", directory.display());
}
