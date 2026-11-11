use serde_json::{Value, json};
use short_form_spike::{evidence::*, translate};

const CORE: &str = "github.com/shrav-k/conduit/core";
const API: &str = "github.com/shrav-k/conduit/v0";

#[test]
fn exact_s1_translations_register_with_as_and_styles_agree() {
    let h = Harness::new().unwrap();
    for task in STAGES {
        let flow = h.check("flow", task).unwrap();
        let block = h.check("block", task).unwrap();
        assert!(flow.documents.iter().any(|d| d.get("as").is_some()));
        for document in flow.documents.iter().filter(|d| d.get("as").is_some()) {
            let name = document["metadata"]["name"].as_str().unwrap();
            assert_eq!(instance(&flow, name)["local_name"], document["as"]);
        }
        access(&flow, task).unwrap();
        access(&block, task).unwrap();
        assert_eq!(flow.manifest, block.manifest, "{task}");
        assert_eq!(flow.documents, block.documents, "{task}");
        assert_eq!(flow.objects, block.objects, "{task}");
        assert_eq!(
            flow.documents.len(),
            if task == "t1" {
                14
            } else if task == "t7-revoked" {
                17
            } else {
                18
            }
        );
    }
}

#[test]
fn upgrades_and_new_directory_change_only_replacement_definitions() {
    let h = Harness::new().unwrap();
    for style in STYLES {
        let t4 = h.check(style, "t4").unwrap();
        let t5 = h.check(style, "t5").unwrap();
        let t6 = h.check(style, "t6").unwrap();
        assert_eq!(
            stable_changes(&t4, &t5, "t5").unwrap(),
            ["shared", "shared--t"]
        );
        assert_eq!(
            stable_changes(&t5, &t6, "t6").unwrap(),
            ["shared", "shared--t"]
        );
    }
}

#[test]
fn t7_restores_identical_canonical_state_and_counts_both_edits() {
    let h = Harness::new().unwrap();
    for style in STYLES {
        let t6 = h.check(style, "t6").unwrap();
        let revoked = h.check(style, "t7-revoked").unwrap();
        let restored = h.check(style, "t7").unwrap();
        let a = by_name(&t6.documents);
        let b = by_name(&revoked.documents);
        assert_eq!(
            a.keys().filter(|n| !b.contains_key(*n)).collect::<Vec<_>>(),
            [&"agent-a--project".to_owned()]
        );
        assert_eq!(t6.documents, restored.documents);
        assert_eq!(
            counts(style).unwrap()[6][0],
            if style == "flow" { 2 } else { 10 }
        );
    }
}

#[test]
fn every_translation_table_row_and_opaque_values() {
    let source = json!({
        "templates": {
            "interfaces": {"person-template": {"type": "example.org/test#person.v1", "inputs": {"label": {"schema": {"type":"string"}, "to":["/config/label"]}}, "description": "reused"}},
            "conduits": {"read": {"capability": "example.org/test#files.v1", "operations": ["example.org/test#read.v1"]}}
        },
        "interfaces": {"person": {"from": "person-template", "arguments": {"label": "shared"}, "description": "actor", "annotations": {"example.org/note":"shared"}, "operator": {"capability":"example.org/test#conversation.v1", "operations": []}, "consumes": {"files": {"to": {"kind":"Resource", "name":"shared"}, "from":"read", "arguments": {"literal": "shared"}}}}},
        "agents": {"worker": {"type": "example.org/test#agent.v1", "config": {"to":"shared"}, "consumes": {"files": {"to": {"kind":"Resource","name":"shared"}, "capability":"example.org/test#files.v1", "operations": ["example.org/test#read.v1"], "requirements": {"limit":{"requirement":"example.org/test#limit.v1","arguments":{"to":"shared"}}}}}}},
        "resources": {"shared": {"type":"example.org/test#resource.v1", "owner":"workspace", "placement":"local", "runtime":"runner", "config":{"name":"shared"}, "conduits": {"nested":{"from":"read", "target":{"kind":"Resource","name":"shared"}}}}},
        "pools": {
            "workspace": {"type": format!("{CORE}#pool.v1"), "entries":{"nested":{"from":"resource-template"}}},
            "private": {"from":"pool-template", "operator":null}
        },
        "conduits": {
            "root-read": {"from":"read", "consumer":{"kind":"Interface","name":"root"}, "target":{"kind":"Resource","name":"shared"}, "as":"files"},
            "worker-read": {"capability":"example.org/test#files.v1", "operations":[], "consumer":{"kind":"Agent","name":"worker"}, "target":{"kind":"Pool","name":"workspace","entry":"nested"}}
        }
    });
    let docs = translate(&source, API, CORE).unwrap();
    let d = by_name(&docs);
    assert_eq!(d["person-template"]["metadata"]["description"], "reused");
    assert!(d["person-template"]["spec"].get("description").is_none());
    assert_eq!(
        d["person"]["arguments"],
        source["interfaces"]["person"]["arguments"]
    );
    assert_eq!(
        d["person"]["metadata"]["annotations"],
        source["interfaces"]["person"]["annotations"]
    );
    assert_eq!(
        d["person--operator"]["consumer"],
        json!({"kind":"Interface","name":"root"})
    );
    assert_eq!(d["person--operator"]["as"], "person");
    assert_eq!(d["person--files"]["from"], "read");
    assert!(!d.contains_key("person--files--t"));
    assert_eq!(
        d["worker--files--t"]["spec"]["requirements"]["limit"]["arguments"]["to"],
        "shared"
    );
    assert_eq!(d["worker--t"]["spec"]["config"]["to"], "shared");
    assert_eq!(
        d["shared--t"]["spec"]["conduits"],
        source["resources"]["shared"]["conduits"]
    );
    assert_eq!(d["shared"]["owner"], "workspace");
    assert_eq!(d["shared"]["runtime"], "runner");
    assert_eq!(
        d["workspace--t"]["spec"]["entries"],
        source["pools"]["workspace"]["entries"]
    );
    assert_eq!(
        d["workspace--operator--t"]["spec"]["operations"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    assert!(!d.contains_key("private--operator"));
    assert_eq!(d["worker-read"]["target"]["entry"], "nested");
    assert!(docs.iter().all(|d| d["apiVersion"] == API));
    let mut framed = source;
    framed["apiVersion"] = json!("example.org/version");
    assert!(
        translate(&framed, API, CORE)
            .unwrap()
            .iter()
            .all(|d| d["apiVersion"] == "example.org/version")
    );
}

#[test]
fn reject_ambiguous_shorthand_and_unsafe_grants() {
    for source in [
        json!({"fleet":"s1", "pools":{}}),
        json!({"pools":{"root":{"type":"x"}}}),
        json!({"pools":{"a--b":{"type":"x"}}}),
        json!({"pools":{"same":{"type":"x"}}, "agents":{"same":{"type":"x"}}}),
        json!({"resources":{"shared":{"type":"x"}}}),
        json!({"agents":{"a":{"from":"b", "type":"x"}}}),
        json!({"agents":{"a":{"type":"x", "inputs":{}}}}),
        json!({"agents":{"a":{"type":"x", "consumes":{"t":{"to":{"kind":"Resource","name":"shared"}, "from":"read"}}}}}),
        json!({"agents":{"a":{"type":"x", "consumes":{"r":{"to":"shared", "from":"read"}}}}}),
        json!({"agents":{"a":{"type":"x", "consumes":{"r":{"to":{"kind":"Resource","name":"management"}, "from":"read"}}}}}),
        json!({"agents":{"a":{"type":"x", "consumes":{"r":{"to":{"kind":"Resource","name":"access"}, "from":"read"}}}}}),
        json!({"conduits":{"r":{"from":"read", "operations":[], "consumer":{"kind":"Agent","name":"a"}, "target":{"kind":"Resource","name":"shared"}}}}),
        json!({"pools":{"a":{"type":"x","consumes":{}}}}),
        json!({"agents":{ "a".repeat(62): {"type":"x"}}}),
    ] {
        assert!(translate(&source, API, CORE).is_err(), "{source}");
    }
}

#[test]
fn strict_reader_rejects_duplicate_keys_before_translation() {
    assert!(parse("pools: {}\npools: {}\n").is_err());
    assert!(parse("pools: {s1: {type: x}}\n---\npools: {}\n").is_err());
}

#[test]
fn line_edits_charge_replacements_deletions_and_separators() {
    assert_eq!(edits("", "# comment\n\na: 1\n---\nb: 2\n"), 3);
    assert_eq!(edits("a: 1\nb: 2\nc: 3\n", "a: 1\nb: 4\nc: 3\n"), 1);
    assert_eq!(edits("a\nb\nc\n", "a\nc\n"), 1);
    assert_eq!(edits("a\nc\n", "a\nb\nc\n"), 1);
    assert_eq!(edits("a\nb\n", "c\nd\ne\n"), 3);
    for style in STYLES {
        let count = counts(style).unwrap();
        assert_eq!(count[2], [0, 0, 0]);
        assert_eq!(count.iter().map(|r| r[1]).sum::<usize>(), 11);
        assert_eq!(count.iter().map(|r| r[2]).sum::<usize>(), 4);
    }
}

#[test]
fn stable_comparison_detects_unrelated_changes() {
    let h = Harness::new().unwrap();
    let t5 = h.check("flow", "t5").unwrap();
    let mut t6 = h.check("flow", "t6").unwrap();
    t6.documents
        .iter_mut()
        .find(|d| d["metadata"]["name"] == "agent-b--project")
        .unwrap()["target"] = Value::Null;
    assert!(stable_changes(&t5, &t6, "t6").is_err());
}
