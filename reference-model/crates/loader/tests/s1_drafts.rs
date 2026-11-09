//! Work order 003: S1's proposed directories pass the strict loader. This asserts syntax
//! and document shape only; it does not install packages, resolve references or run S1.

mod common;

use common::{directories_in, repository, specification};
use conduit_loader::read_project;

#[test]
fn every_s1_draft_directory_is_accepted() {
    let root = repository().join("docs/proposals/first-build/increment-2/s1");
    let directories = directories_in(&root);
    let names: Vec<_> = directories
        .iter()
        .map(|path| path.file_name().unwrap().to_str().unwrap())
        .collect();
    assert_eq!(names, ["agents", "filesystem", "history", "project"]);
    for directory in directories {
        let project = read_project(&directory, specification())
            .unwrap_or_else(|faults| panic!("{}: {faults}", directory.display()));
        assert!(project.manifest.is_some(), "{}", directory.display());
        assert!(!project.documents.is_empty(), "{}", directory.display());
    }
}
