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

    fn fragments(&self) -> PathBuf {
        self.0.join("conformance").join("expected-failures")
    }

    fn fragment(&self, name: &str, cases: Value) {
        write_value(&self.fragments().join(name), &json!({"cases": cases}));
    }

    fn list(&self, cases: Value) {
        for entry in std::fs::read_dir(self.fragments()).unwrap() {
            let entry = entry.unwrap();
            if entry.file_name() != "README.md" {
                std::fs::remove_file(entry.path()).unwrap();
            }
        }
        if !cases.as_array().unwrap().is_empty() {
            self.fragment("001-a.yaml", cases);
        }
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
    assert!(error.contains(&fragment_path("001-a.yaml")));
}

fn fragment_path(name: &str) -> String {
    format!("conformance/expected-failures/{name}")
}

fn diagnostic(path: &str, problem: &str) -> String {
    format!("{path}: {problem}. See conformance/README.md, \"Expected failures\".")
}

#[test]
fn fragments_are_read_as_one_list() {
    let repository = Repository::new("fragments_are_read_as_one_list");
    repository.break_case();
    repository.list(json!([]));
    // The other case exists but does not match the requested prefix.
    repository.fragment("002-b.yaml", json!([{"id": ID, "reason": REASON}]));
    repository.fragment(
        "001-a.yaml",
        json!([{"id": "loader-depth-limit", "reason": "not run"}]),
    );
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
            .any(|line| line == format!("EXPECTED FAILURE {ID} (SUB-005, SCH-013): {REASON}"))
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
fn a_case_listed_twice_is_refused() {
    let repository = Repository::new("a_case_listed_twice_is_refused");
    repository.break_case();
    let entry = json!({"id": ID, "reason": REASON});
    repository.list(json!([entry]));
    repository.fragment("002-b.yaml", json!([entry]));
    assert_eq!(
        run_matching_with_threads(&repository.0, ID, 1).unwrap_err(),
        diagnostic(
            &fragment_path("002-b.yaml"),
            &format!(
                "{ID}: is listed again as entry 1, after entry 1 of {}; keep only one entry for this ID",
                fragment_path("001-a.yaml")
            )
        )
    );
    repository.list(json!([entry, entry]));
    assert_eq!(
        run_matching_with_threads(&repository.0, ID, 1).unwrap_err(),
        diagnostic(
            &fragment_path("001-a.yaml"),
            &format!(
                "{ID}: is listed again as entry 2, after entry 1; keep only one entry for this ID"
            )
        )
    );
}

#[test]
fn the_directory_holds_only_fragments() {
    let repository = Repository::new("the_directory_holds_only_fragments");
    repository.list(json!([]));
    let refused = "not a fragment; a fragment is a regular file directly in conformance/expected-failures/, named as its order's file with .yaml for .md (DEC-190)";
    let retired = repository
        .0
        .join("conformance")
        .join("expected-failures.yaml");
    write_value(&retired, &json!({"cases": []}));
    assert_eq!(
        run_matching_with_threads(&repository.0, ID, 1).unwrap_err(),
        diagnostic(
            "conformance/expected-failures.yaml",
            "retired; move its entries to a fragment under conformance/expected-failures/ named for the order (DEC-190)"
        )
    );
    std::fs::remove_file(retired).unwrap();
    for name in ["Order.yaml", "001.json", "index.yaml", "case.yaml"] {
        let path = repository.fragments().join(name);
        std::fs::write(&path, "not a document").unwrap();
        assert_eq!(
            run_matching_with_threads(&repository.0, ID, 1).unwrap_err(),
            diagnostic(&fragment_path(name), refused)
        );
        std::fs::remove_file(path).unwrap();
    }
    let sub = repository.fragments().join("sub");
    std::fs::create_dir(&sub).unwrap();
    std::fs::write(sub.join("001.yaml"), "not a document").unwrap();
    assert_eq!(
        run_matching_with_threads(&repository.0, ID, 1).unwrap_err(),
        diagnostic(&fragment_path("sub"), refused)
    );
    std::fs::remove_dir_all(sub).unwrap();
    repository.fragment("001-a.yaml", json!([]));
    assert_eq!(
        run_matching_with_threads(&repository.0, ID, 1).unwrap_err(),
        diagnostic(
            &fragment_path("001-a.yaml"),
            "the root's cases is empty; delete the fragment"
        )
    );
    repository.list(json!([]));
    let before = run_matching_with_threads(&repository.0, ID, 1).unwrap();
    std::fs::write(repository.fragments().join("README.md"), "prose, not JSON").unwrap();
    std::fs::write(repository.fragments().join(".hidden.yaml"), "not JSON").unwrap();
    let after = run_matching_with_threads(&repository.0, ID, 1).unwrap();
    assert_eq!(before, after);
    std::fs::remove_dir_all(repository.fragments()).unwrap();
    assert_eq!(
        run_matching_with_threads(&repository.0, ID, 1).unwrap_err(),
        diagnostic(
            "conformance/expected-failures/",
            "not found; restore the directory with its README.md"
        )
    );
}
