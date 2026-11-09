//! Reading a project, handed over as a directory (SUB-010) or as files without one: the one
//! manifest (MAN-001, MAN-002), which files are documents (MAN-011), and that nothing below the
//! directory is read.

mod common;

use std::fs;
use std::path::Path;

use common::*;
use conduit_loader::{FileRole, Syntax, read_project, rules};

fn write(directory: &Path, name: &str, text: &str) {
    if let Some(parent) = directory.join(name).parent() {
        fs::create_dir_all(parent).expect("a directory");
    }
    fs::write(directory.join(name), text).expect("a file");
}

#[test]
fn the_role_of_a_file_is_given_by_its_name() {
    assert_eq!(
        FileRole::of("conduit.yaml"),
        FileRole::Manifest(Syntax::Yaml)
    );
    assert_eq!(
        FileRole::of("conduit.yml"),
        FileRole::Manifest(Syntax::Yaml)
    );
    assert_eq!(
        FileRole::of("conduit.json"),
        FileRole::Manifest(Syntax::Json)
    );
    assert_eq!(
        FileRole::of("templates.yaml"),
        FileRole::Documents(Syntax::Yaml)
    );
    assert_eq!(
        FileRole::of("templates.yml"),
        FileRole::Documents(Syntax::Yaml)
    );
    assert_eq!(
        FileRole::of("templates.json"),
        FileRole::Documents(Syntax::Json)
    );
    for name in [
        "README.md",
        "notes.txt",
        "templates.yaml.bak",
        "templates.YAML",
        "conduit.toml",
        "yaml",
    ] {
        assert_eq!(FileRole::of(name), FileRole::NotRead, "{name}");
    }
    // Only these exact names are the manifest: any other is a file of documents.
    assert_eq!(
        FileRole::of("Conduit.yaml"),
        FileRole::Documents(Syntax::Yaml)
    );
}

#[test]
fn a_directory_is_read_as_one_project() {
    let directory = tempfile::tempdir().expect("a directory");
    let root = directory.path();
    write(root, "conduit.yaml", &manifest());
    write(root, "console.yaml", &interface("console"));
    write(
        root,
        "more.yml",
        &format!("{}---\n{}", interface("viewer"), interface("editor")),
    );
    let project = read_project(root, specification()).unwrap_or_else(|faults| panic!("{faults}"));
    // The manifest is read as the manifest and never as a document (MAN-001).
    assert!(project.manifest.is_some());
    assert_eq!(project.documents.len(), 3);
    assert_eq!(project.directory.as_deref(), Some(root));
}

#[test]
fn a_project_with_no_manifest_is_read() {
    let directory = tempfile::tempdir().expect("a directory");
    write(directory.path(), "console.yaml", &interface("console"));
    let project =
        read_project(directory.path(), specification()).unwrap_or_else(|faults| panic!("{faults}"));
    assert!(project.manifest.is_none());
    assert_eq!(project.documents.len(), 1);
}

#[test]
fn more_than_one_manifest_is_rejected() {
    let directory = tempfile::tempdir().expect("a directory");
    let root = directory.path();
    write(root, "conduit.yaml", &manifest());
    write(
        root,
        "conduit.json",
        &format!("{{\"apiVersion\": \"{}\"}}", api()),
    );
    write(root, "console.yaml", &interface("console"));
    let faults = read_project(root, specification()).expect_err("two manifests");
    assert_eq!(faults.rules(), vec![rules::MAN_002], "{faults}");
    assert!(
        faults.0[0].message.contains("conduit.json, conduit.yaml"),
        "{faults}"
    );
    // Three are rejected the same way.
    write(root, "conduit.yml", &manifest());
    let faults = read_project(root, specification()).expect_err("three manifests");
    assert!(
        faults.0[0]
            .message
            .contains("conduit.json, conduit.yaml, conduit.yml"),
        "{faults}"
    );
    // So are several files submitted without a directory that include two manifests.
    let faults =
        submit(&[("conduit.yaml", &manifest()), ("conduit.yml", &manifest())]).expect_err("two");
    assert_eq!(faults.rules(), vec![rules::MAN_002], "{faults}");
}

#[test]
fn a_manifest_holds_exactly_one_document() {
    for (name, text) in [
        ("conduit.yaml", format!("{}---\n{}", manifest(), manifest())),
        ("conduit.yaml", String::new()),
        ("conduit.yaml", "# nothing but a comment\n".to_string()),
        (
            "conduit.json",
            format!("[{{\"apiVersion\": \"{}\"}}]", api()),
        ),
        ("conduit.json", "[]".to_string()),
    ] {
        let faults = submit(&[(name, &text)]).expect_err("not one document");
        assert_eq!(
            faults.rules(),
            vec![rules::MAN_001],
            "{name} {text:?}: {faults}"
        );
    }
    // A manifest that is one document but not a valid one is rejected by its schema.
    let faults =
        submit(&[("conduit.yaml", &format!("{}kind: Interface\n", manifest()))]).expect_err("kind");
    assert_eq!(faults.rules(), vec![rules::SUB_005], "{faults}");
    assert!(faults.0[0].message.contains("manifest.v1"), "{faults}");
}

#[test]
fn nothing_below_the_directory_is_read() {
    let directory = tempfile::tempdir().expect("a directory");
    let root = directory.path();
    write(root, "console.yaml", &interface("console"));
    // Each of these would be rejected if it were read.
    write(root, "programs/bad.yaml", "a: b: c\n");
    write(root, "nested/conduit.yaml", "not: [a, manifest\n");
    write(root, "other/project.yaml", &interface("Viewer"));
    fs::create_dir_all(root.join("folder.yaml")).expect("a directory with a document's ending");
    let project = read_project(root, specification()).unwrap_or_else(|faults| panic!("{faults}"));
    assert_eq!(project.documents.len(), 1);
    assert!(project.manifest.is_none());
    // The directory below is another project, read on its own.
    let faults =
        read_project(&root.join("other"), specification()).expect_err("an upper-case name");
    assert_eq!(faults.rules(), vec![rules::SUB_005]);
}

#[test]
fn a_file_with_any_other_ending_is_not_read() {
    let directory = tempfile::tempdir().expect("a directory");
    let root = directory.path();
    write(root, "console.yaml", &interface("console"));
    for name in [
        "README.md",
        "notes.txt",
        "console.yaml.bak",
        "draft.YAML",
        "conduit.toml",
    ] {
        write(root, name, "a: b: c\n\u{0}");
    }
    fs::write(root.join("binary.bin"), [0xff, 0xfe, 0x00]).expect("a file");
    let project = read_project(root, specification()).unwrap_or_else(|faults| panic!("{faults}"));
    assert_eq!(project.documents.len(), 1);
}

#[test]
fn every_fault_of_a_project_is_reported() {
    let faults = submit(&[
        ("a.yaml", "a: 1\na: 2\n"),
        ("b.json", "{"),
        ("c.yaml", &interface("Console")),
        ("d.yaml", &interface("console")),
    ])
    .expect_err("three faulty files");
    assert_eq!(
        faults.rules(),
        vec![rules::SCH_012, rules::SUB_005],
        "{faults}"
    );
    assert_eq!(faults.0.len(), 3, "{faults}");
    let files: Vec<_> = faults
        .0
        .iter()
        .map(|f| f.file.clone().unwrap_or_default())
        .collect();
    assert_eq!(files, ["a.yaml", "b.json", "c.yaml"]);
    // A rejection by a schema names the file, the document, the place and the contract.
    let named = &faults.0[2];
    assert_eq!(named.document, Some(1));
    assert_eq!(named.pointer.as_deref(), Some("/metadata/name"));
    assert_eq!(named.position.map(|p| p.line), Some(4));
    assert!(named.message.contains("name.v1"), "{named}");
}

/// What the rejections of a project say, apart from where in a file each is: the rule, the
/// file, the place in the document and the message, in order.
fn verdict(
    faults: &conduit_loader::Diagnostics,
) -> Vec<(String, Option<String>, Option<String>, String)> {
    let mut found: Vec<_> = faults
        .0
        .iter()
        .map(|d| {
            (
                d.rule.to_string(),
                d.file.clone(),
                d.pointer.clone(),
                d.message.clone(),
            )
        })
        .collect();
    found.sort();
    found
}

/// SUB-001: the order in which documents appear, within a file or across files, has no
/// meaning. The same documents in any order give the same objects, or the same diagnostics,
/// apart from which document of its file each is and where in the file.
#[test]
fn the_order_of_the_documents_has_no_meaning() {
    let faulty_name =
        interface("console").replace("  name: console\n", "  name: console\n  name: other\n");
    let faulty_type = format!("{}  type: other\n", interface("viewer"));
    let invalid = interface("Editor");
    let documents = [faulty_name.as_str(), faulty_type.as_str(), invalid.as_str()];
    let mut seen = None;
    for order in [[0, 1, 2], [2, 1, 0], [1, 2, 0]] {
        let text: Vec<&str> = order.iter().map(|i| documents[*i]).collect();
        let text = text.join("---\n");
        let faults = submit(&[("conduit.yaml", &manifest()), ("all.yaml", &text)])
            .expect_err("faulty documents");
        // Two duplicate keys, in two documents, both found when the file is read: the reading
        // goes on past the first. A file rejected when read is not validated.
        assert_eq!(faults.0.len(), 2, "{faults}");
        let found = verdict(&faults);
        assert_eq!(
            seen.get_or_insert_with(|| found.clone()),
            &found,
            "{order:?}"
        );
    }
    // A file that holds no fault when read is validated, and every rejection is reported.
    let valid = interface("editor");
    let documents = [faulty_name.as_str(), invalid.as_str(), valid.as_str()];
    let forward = documents.join("---\n");
    let backward = documents
        .iter()
        .rev()
        .copied()
        .collect::<Vec<_>>()
        .join("---\n");
    let one = submit(&[("all.yaml", &forward)]).expect_err("faulty");
    let other = submit(&[("all.yaml", &backward)]).expect_err("faulty");
    assert_eq!(verdict(&one), verdict(&other));
    // Accepted, the documents are the same, in any order.
    let good = [interface("viewer"), interface("editor")];
    let (one, other) = (
        submit(&[
            ("b.yaml", &good.join("---\n")),
            ("conduit.yaml", &manifest()),
        ])
        .unwrap(),
        submit(&[
            ("conduit.yaml", &manifest()),
            ("b.yaml", &format!("{}---\n{}", good[1], good[0])),
        ])
        .unwrap(),
    );
    assert_eq!(one.documents.len(), 2);
    assert!(
        one.documents
            .iter()
            .all(|d| other.documents.iter().any(|e| e.root == d.root))
    );
    // Across files, the files are read in the order of their names, so the order in which
    // they are handed over changes nothing at all.
    let files = [
        ("b.yaml", interface("viewer")),
        (
            "a.json",
            format!("{{\"apiVersion\": \"{}\", \"kind\": \"Policy\"}}", api()),
        ),
        ("conduit.yaml", manifest()),
        ("c.yaml", "a: 1\na: 2\n".to_string()),
    ];
    let forward: Vec<(&str, &str)> = files.iter().map(|(n, t)| (*n, t.as_str())).collect();
    let mut backward = forward.clone();
    backward.reverse();
    assert_eq!(
        submit(&forward).unwrap_err(),
        submit(&backward).unwrap_err()
    );
}

/// A link is followed as the file system follows it. One that leads nowhere, under a name that
/// MAN-011 reads, is a file that cannot be read: rejected under SUB-010, and never skipped,
/// so that a link named as the manifest cannot leave a project without one.
#[cfg(unix)]
#[test]
fn a_link_is_followed_and_one_that_leads_nowhere_is_rejected() {
    use std::os::unix::fs::symlink;
    let directory = tempfile::tempdir().expect("a directory");
    let root = directory.path();
    let elsewhere = tempfile::tempdir().expect("another directory");
    write(elsewhere.path(), "console.yaml", &interface("console"));
    fs::create_dir_all(elsewhere.path().join("below")).expect("a directory");
    symlink(
        elsewhere.path().join("console.yaml"),
        root.join("linked.yaml"),
    )
    .expect("a link");
    symlink(elsewhere.path().join("below"), root.join("folder.yaml")).expect("a link");
    let project = read_project(root, specification()).unwrap_or_else(|faults| panic!("{faults}"));
    assert_eq!(project.documents.len(), 1);
    symlink(root.join("nowhere.yaml"), root.join("conduit.yaml")).expect("a link");
    symlink(root.join("nowhere.json"), root.join("dangling.json")).expect("a link");
    write(root, "bad.yaml", "a: 1\na: 2\n");
    let faults = read_project(root, specification()).expect_err("dangling links");
    assert_eq!(
        faults.rules(),
        vec![rules::SUB_010, rules::SCH_012],
        "{faults}"
    );
    let named: Vec<_> = faults
        .0
        .iter()
        .filter_map(|d| d.file.as_deref())
        .filter_map(|f| Path::new(f).file_name()?.to_str())
        .collect();
    assert_eq!(
        named,
        ["conduit.yaml", "dangling.json", "bad.yaml"],
        "{faults}"
    );
}

#[test]
fn a_json_file_of_documents_is_read_as_its_items() {
    let as_json = |name: &str| {
        format!(
            "{{\"apiVersion\": \"{api}\", \"kind\": \"Interface\", \"metadata\": {{\"name\": \"{name}\"}}, \"spec\": {{\"type\": \"{core}#interface.v1\"}}}}",
            api = api(),
            core = core()
        )
    };
    let array = format!("[{}, {}]", as_json("console"), as_json("viewer"));
    let project =
        submit(&[("interfaces.json", &array)]).unwrap_or_else(|faults| panic!("{faults}"));
    assert_eq!(project.documents.len(), 2);
    assert_eq!(project.documents[1].index, 2);
    // The YAML form of the same documents gives the same values (SCH-013, SCH-018).
    let yaml = format!("{}---\n{}", interface("console"), interface("viewer"));
    let from_yaml = submit(&[("interfaces.yaml", &yaml)]).unwrap();
    assert!(
        project
            .documents
            .iter()
            .zip(&from_yaml.documents)
            .all(|(a, b)| a.root == b.root)
    );
    // An item that is not a map is a document the schemas reject, as in YAML.
    let faults = submit(&[("odd.json", &format!("[{}, 15]", as_json("console")))]).expect_err("15");
    assert_eq!(faults.rules(), vec![rules::SUB_005]);
    assert_eq!(faults.0[0].document, Some(2));
}

#[test]
fn an_unreadable_directory_is_a_value_not_a_panic() {
    let directory = tempfile::tempdir().expect("a directory");
    let missing = directory.path().join("missing");
    let faults = read_project(&missing, specification()).expect_err("no directory");
    assert_eq!(faults.rules(), vec![rules::SUB_010]);
}
