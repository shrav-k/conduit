//! The done-when of increment 1 (docs/implementation/FIRST_BUILD.md), on the examples under
//! `spec/schemas/examples/`: every document under `valid/` is accepted, each directory read as
//! a project; every file under `invalid/` is rejected by the step its name gives, the verdict
//! that `tools/check_schemas.py` gives it, a rejection by a schema always under SUB-005; and
//! every document under `invalid-by-rule/` passes the schemas, since its rule comes with the
//! core, the rule of SCH-016 apart, which the loader applies. The documents of the core package
//! under `spec/core/` and `spec/schemas/` are valid too, as step 3 of `check_schemas.py`
//! checks. A file whose name begins with a dot is not read (MAN-011), here or by the tool.

mod common;

use std::collections::BTreeMap;

use common::*;
use conduit_loader::{Document, Role, Syntax, read_file, read_project, read_text, rules};

fn is_manifest_file(file: &std::path::Path) -> bool {
    file.file_name().is_some_and(|name| {
        ["conduit.yaml", "conduit.yml", "conduit.json"]
            .iter()
            .any(|want| name == *want)
    })
}

#[test]
fn the_documents_of_the_core_package_are_valid() {
    let spec = specification();
    assert!(
        spec.contract_names().count() > 0,
        "no Schema contract was read"
    );
    for directory in ["core", "schemas"] {
        let project = read_project(&spec_dir().join(directory), spec)
            .unwrap_or_else(|faults| panic!("spec/{directory}: {faults}"));
        assert!(
            !project.documents.is_empty(),
            "spec/{directory} holds documents"
        );
        // spec/schemas/examples/ lies below spec/schemas/ and is not read (MAN-011).
        let read_from = spec_dir().join(directory);
        assert!(
            project
                .documents
                .iter()
                .all(|d| std::path::Path::new(&d.file).parent() == Some(read_from.as_path()))
        );
    }
    let core = read_project(&spec_dir().join("core"), spec).expect("spec/core");
    assert!(
        core.manifest.is_some(),
        "spec/core holds the core package's manifest"
    );
}

/// `tools/check_schemas.py` finds the examples in the whole tree below each directory, and
/// these tests read the files directly in it, and each directory directly under `valid/` as a
/// project, which MAN-011 reads without what lies below. The two find the same files only
/// while the trees are flat, so a nested example would be checked by the tool and missed here.
#[test]
fn the_trees_of_examples_are_flat() {
    let mut directories = vec![
        examples().join("invalid"),
        examples().join("invalid-by-rule"),
    ];
    directories.extend(directories_in(&examples().join("valid")));
    for directory in directories {
        assert_eq!(
            directories_in(&directory),
            Vec::<std::path::PathBuf>::new(),
            "{} holds a directory, whose examples these tests do not read",
            directory.display()
        );
    }
}

#[test]
fn every_valid_example_is_accepted() {
    let directories = directories_in(&examples().join("valid"));
    assert!(!directories.is_empty());
    for directory in directories {
        // Short-form translation is built in 043a/043b. Check its authored schemas here,
        // without passing a sectioned value through the canonical wire document validator.
        let files = files_in(&directory);
        let sectioned = files
            .iter()
            .filter(|file| !is_manifest_file(file))
            .any(|file| {
                read_file(file)
                    .expect("strict example syntax")
                    .iter()
                    .any(|document| document.root.get("kind").is_none())
            });
        if sectioned {
            for file in files {
                for document in read_file(&file).expect("strict example syntax") {
                    if is_manifest_file(&file) {
                        specification()
                            .validate(&document, Role::Manifest)
                            .unwrap_or_else(|fault| panic!("{}: {fault}", file.display()));
                    } else if document.root.get("kind").is_none() {
                        let schema = specification()
                            .contract_validator("sectioned.v1")
                            .expect("sectioned schema");
                        schema
                            .validate(&document.root.to_json())
                            .unwrap_or_else(|fault| panic!("{}: {fault}", file.display()));
                    } else {
                        specification()
                            .validate(&document, Role::Document)
                            .unwrap_or_else(|fault| panic!("{}: {fault}", file.display()));
                    }
                }
            }
            continue;
        }
        let project = read_project(&directory, specification())
            .unwrap_or_else(|faults| panic!("{}: {faults}", directory.display()));
        // Every file of the directory is read: the manifest, and each document of the others.
        let mut documents = 0;
        for file in files_in(&directory) {
            let read = read_file(&file).unwrap_or_else(|faults| panic!("{faults}"));
            if file.file_name().is_some_and(|n| n == "conduit.yaml") {
                assert!(project.manifest.is_some(), "{}", file.display());
            } else {
                documents += read.len();
            }
        }
        assert_eq!(
            project.documents.len(),
            documents,
            "{}",
            directory.display()
        );
    }
}

/// The verdicts that `tools/check_schemas.py` gives the files under `invalid/`, from the
/// fixture `tests/expected/invalid-verdicts.txt`, which says how they were derived.
fn expected_verdicts() -> BTreeMap<String, String> {
    let fixture = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/expected/invalid-verdicts.txt"),
    )
    .expect("the fixture");
    fixture
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            let (name, step) = line.split_once(' ').expect("a name and a step");
            (name.to_string(), step.trim().to_string())
        })
        .collect()
}

/// The promise a file under `invalid/` is named for: its name begins with the promise's ID.
fn promise_of(name: &str) -> String {
    name.splitn(3, '-').take(2).collect::<Vec<_>>().join("-")
}

#[test]
fn every_invalid_example_is_rejected_by_the_step_its_name_gives() {
    let expected = expected_verdicts();
    let all_files = files_in(&examples().join("invalid"));
    let files: Vec<_> = all_files
        .iter()
        .filter(|file| !name_of(file).contains("-sectioned-"))
        .cloned()
        .collect();
    for file in all_files
        .iter()
        .filter(|file| name_of(file).contains("-sectioned-"))
    {
        let schema = specification()
            .contract_validator("sectioned.v1")
            .expect("sectioned schema");
        let documents = read_file(file).expect("strict sectioned syntax");
        assert!(!documents.is_empty());
        for document in documents {
            assert!(
                !schema.is_valid(&document.root.to_json()),
                "{} was accepted",
                file.display()
            );
        }
    }
    let names: Vec<String> = files
        .iter()
        .map(|f| f.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        names.to_vec(),
        expected.keys().cloned().collect::<Vec<_>>(),
        "the fixture lists exactly the files under invalid/"
    );
    for (file, name) in files.iter().zip(&names) {
        let promise = promise_of(name);
        let step = match read_file(file) {
            Err(faults) => {
                // Rejected when read: the rule is the promise the file is named for.
                assert_eq!(faults.rules(), vec![promise.as_str()], "{name}: {faults}");
                "read"
            }
            Ok(documents) => {
                assert!(!documents.is_empty(), "{name} holds a document");
                let role = if name.contains("-manifest-") {
                    Role::Manifest
                } else {
                    Role::Document
                };
                for document in &documents {
                    let fault = specification()
                        .validate(document, role)
                        .expect_err(&format!("{name}: accepted, and it must be rejected"));
                    assert_eq!(fault.rule, rules::SUB_005, "{name}: {fault}");
                }
                "schema"
            }
        };
        assert_eq!(step, expected[name], "{name}: the step that rejects it");
    }
}

/// What a document of a file under `invalid-by-rule/` is validated as, as `check_schemas.py`
/// has it: a file named `-manifest-` holds a manifest, and in any other a document with no
/// `kind` is the manifest of its set's project.
fn by_rule_role(name: &str, document: &Document) -> Role {
    let embedded_manifest = document.root.get("kind").is_none()
        && document.root.to_json().as_object().is_some_and(|map| {
            map.keys().all(|key| {
                [
                    "apiVersion",
                    "address",
                    "version",
                    "packages",
                    "exports",
                    "implementations",
                ]
                .contains(&key.as_str())
            })
        });
    if name.contains("-manifest-") || embedded_manifest {
        Role::Manifest
    } else {
        Role::Document
    }
}

#[test]
fn every_invalid_by_rule_example_passes_the_schemas() {
    let files = files_in(&examples().join("invalid-by-rule"));
    assert!(!files.is_empty());
    for file in files {
        let name = name_of(&file);
        let documents = read_file(&file).unwrap_or_else(|faults| panic!("{name}: {faults}"));
        for document in &documents {
            specification()
                .validate(document, by_rule_role(&name, document))
                .unwrap_or_else(|fault| panic!("{name}: {fault}"));
        }
    }
}

/// SCH-016, as the coordinator settled finding 1-03: a schema whose pattern is not one of
/// ECMA-262 makes the document that holds it rejected. Of the rules of `invalid-by-rule/`, it is
/// the one the loader applies, in `Specification::check`, as reading a project does: each file
/// there named for SCH-016 is rejected by it alone, and every other passes `check`, since its
/// rule comes with the core.
#[test]
fn check_rejects_the_invalid_by_rule_examples_of_sch_016_alone() {
    let mut named = 0;
    for file in files_in(&examples().join("invalid-by-rule")) {
        let name = name_of(&file);
        let documents = read_file(&file).unwrap_or_else(|faults| panic!("{name}: {faults}"));
        let mut found = Vec::new();
        for document in &documents {
            if let Err(faults) = specification().check(document, by_rule_role(&name, document)) {
                found.extend(faults.rules());
            }
        }
        if promise_of(&name) == rules::SCH_016 {
            named += 1;
            assert_eq!(found, vec![rules::SCH_016], "{name}");
        } else {
            assert_eq!(found, Vec::<&str>::new(), "{name}");
        }
    }
    assert!(
        named > 0,
        "no file under invalid-by-rule/ is named for SCH-016"
    );
}

/// MAN-011, as the coordinator settled finding 1-06: a name that begins with a dot is not read.
/// A directory under `valid/` holds such a file, which would be rejected if it were read, and
/// the project is accepted beside it.
#[test]
fn a_file_whose_name_begins_with_a_dot_is_not_read_in_a_valid_example() {
    let mut found = 0;
    for directory in directories_in(&examples().join("valid")) {
        for entry in std::fs::read_dir(&directory).expect("a directory") {
            let path = entry.expect("an entry").path();
            let name = name_of(&path);
            if !name.starts_with('.') || Syntax::of(&name).is_none() {
                continue;
            }
            found += 1;
            // Only read a canonical project when it actually holds a hidden-file fixture.
            let project = read_project(&directory, specification())
                .unwrap_or_else(|faults| panic!("{}: {faults}", directory.display()));
            let rejected = match read_file(&path) {
                Err(_) => true,
                Ok(documents) => documents
                    .iter()
                    .any(|d| specification().validate(d, Role::Document).is_err()),
            };
            assert!(rejected, "{name} would be accepted if it were read");
            let shown = path.display().to_string();
            assert!(
                project.documents.iter().all(|d| d.file != shown),
                "{name} was read"
            );
            assert!(
                project.manifest.as_ref().is_none_or(|m| m.file != shown),
                "{name} was read"
            );
        }
    }
    assert!(
        found > 0,
        "no valid example holds a file whose name begins with a dot"
    );
}

/// Each document written as JSON, as the array of a JSON file.
fn as_json(documents: &[Document]) -> String {
    let values: Vec<serde_json::Value> = documents.iter().map(|d| d.root.to_json()).collect();
    serde_json::to_string_pretty(&values).expect("JSON")
}

/// SCH-013: a document written in JSON means the same as the equivalent YAML document, with
/// the same objects or the same diagnostics. Each example that can be read is written again as
/// JSON, read back, and validated, and the two give the same values and the same verdicts.
#[test]
fn every_example_means_the_same_in_json() {
    let mut files = Vec::new();
    for directory in directories_in(&examples().join("valid")) {
        files.extend(files_in(&directory));
    }
    files.extend(files_in(&examples().join("invalid")));
    files.extend(files_in(&examples().join("invalid-by-rule")));
    let mut compared = 0;
    for file in files {
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        let Ok(yaml) = read_file(&file) else {
            continue; // rejected when read: there is no value to write as JSON
        };
        let json =
            read_text(Syntax::Json, &as_json(&yaml)).unwrap_or_else(|f| panic!("{name}: {f}"));
        assert_eq!(json.len(), yaml.len(), "{name}");
        for (from_yaml, from_json) in yaml.iter().zip(json) {
            assert!(
                from_yaml.root == from_json,
                "{name}: the JSON form reads as other values"
            );
            let as_document = Document {
                file: from_yaml.file.clone(),
                index: from_yaml.index,
                root: from_json,
            };
            let role = if is_manifest_file(&file) || name.contains("-manifest-") {
                Role::Manifest
            } else {
                Role::Document
            };
            if role == Role::Document && from_yaml.root.get("kind").is_none() {
                let schema = specification()
                    .contract_validator("sectioned.v1")
                    .expect("sectioned schema");
                let verdict = |d: &Document| {
                    schema
                        .iter_errors(&d.root.to_json())
                        .map(|fault| (fault.instance_path().to_string(), fault.to_string()))
                        .collect::<Vec<_>>()
                };
                assert_eq!(verdict(from_yaml), verdict(&as_document), "{name}");
                compared += 1;
                continue;
            }
            let verdict = |d: &Document| {
                specification()
                    .validate(d, role)
                    .map_err(|f| (f.rule, f.pointer, f.message))
            };
            assert_eq!(verdict(from_yaml), verdict(&as_document), "{name}");
            compared += 1;
        }
    }
    assert!(compared > 100, "only {compared} documents were compared");
}
