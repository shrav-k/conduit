use conduit_conformance::run_matching_with_threads;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

const ID: &str = "loader-leading-bom";
const REASON: &str = "the build lacks the changed BOM expectation; order 163 test fixture";

struct Repository(PathBuf);

impl Drop for Repository {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    std::fs::create_dir(destination).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let target = destination.join(entry.file_name());
        if path.is_dir() {
            copy_tree(&path, &target);
        } else {
            std::fs::copy(path, target).unwrap();
        }
    }
}

impl Repository {
    fn new(name: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "conduit-expected-failures-{name}-{}",
            std::process::id()
        ));
        std::fs::create_dir(&directory).unwrap();
        let repository = Self(directory);
        let original = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        for name in ["spec", "conformance"] {
            copy_tree(&original.join(name), &repository.0.join(name));
        }
        repository
    }

    fn list(&self, cases: Value) {
        write_value(
            &self.0.join("conformance").join("expected-failures.yaml"),
            &json!({"cases": cases}),
        );
    }

    fn break_case(&self) {
        let path = self
            .0
            .join("conformance")
            .join("language")
            .join("loader")
            .join("bom")
            .join("case.yaml");
        let mut case: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let value = case.pointer_mut("/steps/0/expect/documents/0/a").unwrap();
        assert_eq!(value, "text");
        *value = json!("changed by the test");
        write_value(&path, &case);
    }
}

fn write_value(path: &Path, value: &Value) {
    std::fs::write(path, serde_json::to_string_pretty(value).unwrap() + "\n").unwrap();
}

#[test]
fn a_listed_failing_case_is_reported_apart() {
    let repository = Repository::new("a_listed_failing_case_is_reported_apart");
    repository.break_case();
    repository.list(json!([{"id": ID, "reason": REASON}]));
    let report = run_matching_with_threads(&repository.0, ID, 1).unwrap();
    assert!(report.passed.is_empty());
    assert_eq!(report.expected_failures.len(), 1);
    let failure = &report.expected_failures[0];
    assert_eq!(failure.id, ID);
    assert_eq!(failure.promises, ["SUB-005", "SCH-013"]);
    assert_eq!(failure.reason, REASON);
    assert!(failure.error.starts_with("step 1"));

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_conduit-conformance"))
        .arg(&repository.0)
        .arg(ID)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.lines().any(|line| line.starts_with("PASS ")));
    assert!(
        stdout
            .lines()
            .any(|line| { line == format!("EXPECTED FAILURE {ID} (SUB-005, SCH-013): {REASON}") })
    );
    for line in failure.error.lines() {
        assert!(stdout.lines().any(|printed| printed == format!("  {line}")));
    }
    assert_eq!(
        stdout.lines().last(),
        Some("0 cases passed; 1 expected failures")
    );
}

#[test]
fn an_unlisted_failure_fails_the_run() {
    let repository = Repository::new("an_unlisted_failure_fails_the_run");
    repository.break_case();
    repository.list(json!([]));
    let error = run_matching_with_threads(&repository.0, ID, 1).unwrap_err();
    assert!(error.contains("0 cases passed; 1 failed"));
    assert!(error.contains("loader-leading-bom [SUB-005, SCH-013]: step 1"));
}

#[test]
fn a_listed_case_that_passes_fails_the_run() {
    let repository = Repository::new("a_listed_case_that_passes_fails_the_run");
    repository.list(json!([{"id": ID, "reason": REASON}]));
    let error = run_matching_with_threads(&repository.0, ID, 1).unwrap_err();
    assert!(error.contains(ID));
    assert!(error.contains(REASON));
    assert!(error.contains("remove its entry"));
}
