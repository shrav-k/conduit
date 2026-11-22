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
        exports.ends_with(&["- turn", "- conversation"]),
        "exports must end with turn and conversation"
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
