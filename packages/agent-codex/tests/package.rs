use sha2::{Digest, Sha256};
use std::{fs, io::Read, process::Command};
#[test]
fn deterministic_host_archive_metadata_and_digest() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let pack = || {
        let o = Command::new(env!("CARGO_BIN_EXE_package"))
            .output()
            .unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        String::from_utf8(o.stdout).unwrap().trim().to_string()
    };
    let first = pack();
    let (path, hash) = first.rsplit_once('#').unwrap();
    let bytes = fs::read(path).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), hash);
    assert_eq!(
        fs::read_to_string(format!("{path}.sha256")).unwrap(),
        format!("{hash}\n")
    );
    assert_eq!(pack(), first);
    assert_eq!(fs::read(path).unwrap(), bytes);
    assert_eq!(&bytes[4..8], &[0; 4]);
    let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(bytes.as_slice()));
    let mut names = Vec::new();
    for e in tar.entries().unwrap() {
        let mut e = e.unwrap();
        let name = e.path().unwrap().display().to_string();
        let h = e.header();
        assert!(h.as_ustar().is_some());
        assert_eq!(h.uid().unwrap(), 0);
        assert_eq!(h.gid().unwrap(), 0);
        assert_eq!(h.mtime().unwrap(), 0);
        assert_eq!(
            h.mode().unwrap(),
            if name.starts_with("bin/") {
                0o755
            } else {
                0o644
            }
        );
        let mut content = Vec::new();
        e.read_to_end(&mut content).unwrap();
        if name.ends_with(".yaml") {
            assert_eq!(content, fs::read(root.join(&name)).unwrap());
        }
        names.push(name);
    }
    assert_eq!(
        names,
        vec![
            format!("bin/{}", agent_adapter::WRAPPER),
            "conduit.yaml".into(),
            "contracts.yaml".into()
        ]
    );
}

#[test]
fn standard_conduit_templates_keep_their_operations() {
    let manifest = include_str!("../conduit.yaml");
    let exports: Vec<_> = manifest
        .split_once("\nexports:\n")
        .unwrap()
        .1
        .lines()
        .take_while(|line| line.starts_with("- "))
        .collect();
    assert!(
        exports.ends_with(&["- turn", "- conversation", "- history"]),
        "exports must end with turn, conversation and history"
    );
    let contracts = include_str!("../contracts.yaml");
    for template in [
        r#"---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn
spec:
  capability: '#conversation.v1'
  operations: ['#turn.v1']
"#,
        r#"---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: conversation
spec:
  capability: '#conversation.v1'
  operations: ['#turn.v1', '#new-conversation.v1']
"#,
        r#"---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: history
spec:
  capability: '#history-reading.v1'
  operations: ['#history.v1']
"#,
    ] {
        assert_eq!(
            contracts.matches(template).count(),
            1,
            "expected one copy of:\n{template}"
        );
    }
}

#[test]
fn codex_login_types_keep_their_shape() {
    let contracts = include_str!("../contracts.yaml");
    let type_document = |name: &str| {
        let name_line = format!("  name: {name}");
        contracts
            .split("\n---\n")
            .find(|document| document.lines().any(|line| line == name_line))
            .unwrap_or_else(|| panic!("missing Type {name}"))
    };
    let login = type_document("codex-login.v1");
    let agent = type_document("codex.v3");
    let logins = |document: &'static str| {
        document
            .split_once("    logins:\n")
            .unwrap()
            .1
            .lines()
            .take_while(|line| line.starts_with("    "))
            .collect::<Vec<_>>()
    };
    assert!(login.contains("kind: Type\n"));
    assert!(login.contains("  family: Resource\n"));
    assert!(login.contains("      account: {type: string}\n    required: [account]\n"));
    assert!(login.contains("    account: /account\n"));
    assert!(logins(login) == logins(type_document("codex.v2")));
    assert!(!login.contains("  offers:"));
    assert!(login.contains("  records: {inputs: false, outputs: false}"));
    for hook in ["sign-in", "account", "sign-out", "retire"] {
        let operation = format!("'github.com/shrav-k/conduit/core#{hook}-hook.v1'");
        assert!(login.contains(&format!("  - {operation}\n")));
        assert!(login.contains(&format!("    {operation}: '#login'\n")));
    }
    assert!(agent.contains("kind: Type\n"));
    assert!(agent.contains("  family: Agent\n"));
    assert!(agent.contains("  toolsDirectory: {directory: /directory}\n"));
    assert!(agent.contains("  loginResource: {place: /login}\n"));
    assert!(agent.contains("login: {$ref: github.com/shrav-k/conduit/core#resource-reference.v1}"));
    assert!(agent.contains(
        "required: [directory, turnDeadlineSeconds, effort, accountServices, model, login]"
    ));
    assert!(!agent.contains("  signIn:") && !agent.contains("      account:"));
    assert!(agent.contains("  offers: ['#conversation.v1']\n"));
    let requires = agent
        .split_once("  requires:\n")
        .unwrap()
        .1
        .split_once("  implementation:\n")
        .unwrap()
        .0;
    assert_eq!(
        requires,
        "  - 'github.com/shrav-k/conduit/core#setup-hook.v1'\n  - 'github.com/shrav-k/conduit/core#retire-hook.v1'\n"
    );
    let manifest = include_str!("../conduit.yaml");
    assert!(manifest.contains("- codex.v2\n- codex-login.v1\n- codex.v3\n"));
    assert!(manifest.contains("AGENT_LOGIN_DIRECTORY: /login\n"));
}

#[test]
fn codex_v4_keeps_its_shape() {
    let contracts = include_str!("../contracts.yaml");
    let doc = |name| {
        let name = format!("  name: {name}");
        contracts
            .split("\n---\n")
            .find(|doc| doc.lines().any(|line| line == name))
            .unwrap_or_else(|| panic!("missing Type {name}"))
    };
    let v4 = doc("codex.v4");
    assert!(v4.contains("  family: Agent\n"));
    assert!(v4.contains("  loginResource: {place: /login}\n"));
    assert!(v4.contains("  toolsDirectory: {directory: /directory}\n"));
    for (field, limit) in [("instructions", 21_841), ("outputSchema", 32_762)] {
        assert!(v4.contains(&format!("      {field}: {{type: [string, 'null'], minLength: 1, maxLength: {limit}, default: null}}\n")));
    }
    assert!(v4.contains("required: [directory, turnDeadlineSeconds, effort, accountServices, model, login, instructions, outputSchema]"));
    assert_eq!(v4.matches(": '#wrapper-v4'\n").count(), 4);
    let turn = doc("turn.v1").split_once("  outputSchema:\n").unwrap().1;
    assert!(turn.contains("      structured: {type: object}\n"));
    assert!(turn.contains("    required: [text, clientTools]\n"));
    let manifest = include_str!("../conduit.yaml");
    assert!(manifest.contains("- codex.v3\n- codex.v4\n"));
    assert!(manifest.contains("      AGENT_INSTRUCTIONS: /instructions\n"));
    assert!(manifest.contains("      AGENT_OUTPUT_SCHEMA: /outputSchema\n"));
}

#[test]
fn codex_v5_keeps_its_shape() {
    let contracts = include_str!("../contracts.yaml");
    let document = |name: &str| {
        contracts
            .split("\n---\n")
            .find(|doc| doc.lines().any(|line| line == format!("  name: {name}")))
            .unwrap_or_else(|| panic!("missing contract {name}"))
    };
    let v5 = document("codex.v5");
    let v4 = document("codex.v4");
    let spec = |doc: &'static str| doc.split_once("\nspec:\n").unwrap().1;
    assert_eq!(
        spec(v5),
        spec(v4)
            .replace(
                "offers: ['#conversation.v1']",
                "offers: ['#conversation.v1', '#history-reading.v1']"
            )
            .replace("#wrapper-v4", "#wrapper-v5")
            .replace("  records:", "    '#history.v1': '#wrapper-v5'\n  records:")
    );
    let capability = document("history-reading.v1");
    assert!(capability.contains("kind: Capability\n"));
    assert!(capability.ends_with("  operations: ['#history.v1']"));
    let operation = document("history.v1");
    assert!(operation.contains("kind: Operation\n"));
    assert!(operation.contains("  startsModelWork: false\n"));
    assert!(!operation.contains("\n  effect:"));
    for shape in [
        "since: {type: integer, minimum: 0, default: 0}",
        "count: {type: integer, minimum: 1, maximum: 50, default: 20}",
        "maxItems: 50",
        "latest: {type: integer, minimum: 0}",
        "enum: [running, completed, failed, interrupted]",
        "required: [index, invocation, caller, message, text, state, startedAt, endedAt]",
        "required: [turns, latest]",
    ] {
        assert!(operation.contains(shape), "missing history shape: {shape}");
    }
    assert_eq!(operation.matches("additionalProperties: false").count(), 3);
}

#[test]
fn history_documents_the_boundary_between_turn_text_and_events() {
    let contracts = include_str!("../contracts.yaml");
    let (_, history) = contracts.split_once("\n  name: history.v1\n").unwrap();
    let (description, _) = history.split_once("\nspec:\n").unwrap();
    let readme = include_str!("../README.md");
    for (source, text) in [("contract", description), ("README", readme)] {
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(
            text.contains("message and final text as written"),
            "{source} must state that turn text is kept as written"
        );
        assert!(
            text.contains("even when the client stream carries them"),
            "{source} must state that excluded events are discarded"
        );
    }
}

#[test]
fn history_readme_uses_sectioned_conduit_forms() {
    let readme = include_str!("../README.md");
    assert!(
        !readme.contains("reader: {from: example.org/conduit/agent-codex#history}"),
        "a sectioned Agent does not accept a reader key"
    );
    assert!(readme.contains("operator: {from: example.org/conduit/agent-codex#history}"));
    let standalone =
        "\nconduits:\n  codex-worker-history:\n    from: example.org/conduit/agent-codex#history\n";
    let (_, example) = readme
        .split_once(standalone)
        .expect("standalone history grant");
    assert!(example.starts_with(concat!(
        "    consumer: {kind: Interface, name: root}\n",
        "    target: {kind: Agent, name: codex-worker}\n",
        "    as: codex-worker-history\n",
    )));
}

#[test]
fn history_documents_configuration_and_retirement_refusals() {
    let (codex_v5, _) = include_str!("../contracts.yaml")
        .split_once("\n  name: codex.v5\n")
        .unwrap()
        .1
        .split_once("\nspec:\n")
        .unwrap();
    let readme = include_str!("../README.md");
    for (source, text) in [("Type", codex_v5), ("README", readme)] {
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(
            text.contains("Configuration refusals, including the home and login-directory checks,"),
            "{source} must name home checks as configuration refusals"
        );
        assert!(text.contains("turns after the retire hook leave no entry"));
    }
}
