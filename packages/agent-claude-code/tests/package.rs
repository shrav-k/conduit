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
        exports.ends_with(&[
            "- turn",
            "- conversation",
            "- account-home.v1",
            "- claude-code.v3",
            "- claude-code.v4",
            "- claude-code.v5",
        ]),
        "template exports must immediately precede the new Types"
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
fn account_home_types_keep_their_shape() {
    let contracts = include_str!("../contracts.yaml");
    let document = |name| {
        let name_line = format!("  name: {name}");
        contracts
            .split("\n---\n")
            .find(|doc| doc.lines().any(|line| line == name_line))
            .unwrap_or_else(|| panic!("missing Type {name}"))
    };
    let home = document("account-home.v1");
    let agent = document("claude-code.v3");
    let logins = |doc: &'static str| {
        doc.split_once("    logins:\n")
            .unwrap()
            .1
            .lines()
            .take_while(|line| line.starts_with("    "))
            .collect::<Vec<_>>()
    };
    assert!(home.contains("kind: Type\nmetadata:"));
    assert!(home.contains("  family: Resource\n"));
    assert!(home.contains("      account: {type: string}\n    required: [account]\n"));
    assert!(home.contains("  signIn:\n    account: /account\n"));
    assert!(logins(home) == logins(document("claude-code.v2")));
    assert!(!home.contains("  offers:"));
    assert!(home.contains("  records: {inputs: false, outputs: false}"));
    let requirements = home
        .split_once("  requires:\n")
        .unwrap()
        .1
        .split_once("  implementation:\n")
        .unwrap()
        .0;
    for hook in ["sign-in", "account", "sign-out", "retire"] {
        let operation = format!("'github.com/shrav-k/conduit/core#{hook}-hook.v1'");
        assert!(requirements.contains(&format!("  - {operation}\n")));
        assert!(home.contains(&format!("    {operation}: '#login'\n")));
    }
    assert_eq!(requirements.lines().count(), 4);

    assert!(agent.contains("kind: Type\nmetadata:"));
    assert!(agent.contains("  family: Agent\n"));
    assert!(agent.contains("  toolsDirectory: {directory: /directory}\n"));
    assert!(agent.contains("  loginResource: {place: /login}\n"));
    assert!(agent.contains("login: {$ref: github.com/shrav-k/conduit/core#resource-reference.v1}"));
    assert!(agent.contains("required: [directory, turnDeadlineSeconds, effort, accountServices, model, peerMessaging, login]"));
    assert!(!agent.contains("  signIn:") && !agent.contains("      account:"));
    assert!(agent.contains("  offers: ['#conversation.v1']\n"));
    assert!(agent.contains("  requires: ['github.com/shrav-k/conduit/core#retire-hook.v1']\n"));
    assert_eq!(agent.matches(": '#wrapper-v3'\n").count(), 3);
    let manifest = include_str!("../conduit.yaml");
    assert!(manifest.contains("- conversation\n- account-home.v1\n- claude-code.v3\n"));
    assert!(manifest.contains("AGENT_LOGIN_DIRECTORY: /login\n"));
}

#[test]
fn claude_code_v4_keeps_its_shape() {
    let contracts = include_str!("../contracts.yaml");
    let v4 = contracts
        .split("\n---\n")
        .find(|doc| doc.lines().any(|line| line == "  name: claude-code.v4"))
        .expect("missing Type claude-code.v4");
    for (place, limit) in [("instructions", 32763), ("outputSchema", 32762)] {
        assert!(v4.contains(&format!(
            "      {place}: {{type: [string, 'null'], minLength: 1, maxLength: {limit}, default: null}}"
        )));
    }
    assert!(v4.contains("required: [directory, turnDeadlineSeconds, effort, accountServices, model, peerMessaging, login, instructions, outputSchema]"));
    assert!(v4.contains("  loginResource: {place: /login}\n"));
    assert_eq!(v4.matches(": '#wrapper-v4'\n").count(), 3);
    let turn = contracts
        .split("\n---\n")
        .find(|doc| doc.lines().any(|line| line == "  name: turn.v1"))
        .unwrap();
    assert!(turn.contains("      structured: {type: object}\n"));
    assert!(
        !turn
            .lines()
            .any(|line| line.contains("required:") && line.contains("structured"))
    );
    let manifest = include_str!("../conduit.yaml");
    assert!(manifest.contains("- claude-code.v3\n- claude-code.v4\n"));
    let item = manifest.split_once("  wrapper-v4:\n").unwrap().1;
    assert!(item.contains("AGENT_INSTRUCTIONS: /instructions"));
    assert!(item.contains("AGENT_OUTPUT_SCHEMA: /outputSchema"));
}

#[test]
fn claude_code_v5_keeps_its_shape() {
    let contracts = include_str!("../contracts.yaml");
    let contract = |name| {
        contracts
            .split("\n---\n")
            .find(|doc| doc.lines().any(|line| line == format!("  name: {name}")))
    };
    let v4 = contract("claude-code.v4").unwrap();
    let v5 = contract("claude-code.v5").expect("missing Type claude-code.v5");
    let setup = "github.com/shrav-k/conduit/core#setup-hook.v1";
    let retire = "github.com/shrav-k/conduit/core#retire-hook.v1";
    let addition = "    The setup hook gives the path and version of the client it finds, and fails where the\n    client is missing, gives no version, or is older than the lowest version the package was\n    tested with.\n";
    let expected = v4
        .replace("claude-code.v4", "claude-code.v5")
        .replace("wrapper-v4", "wrapper-v5")
        .replace("\nspec:", &format!("\n{addition}spec:"))
        .replace(
            &format!("requires: ['{retire}']"),
            &format!("requires: ['{setup}', '{retire}']"),
        )
        .replace(
            &format!("    '{retire}': '#wrapper-v5'"),
            &format!("    '{setup}': '#wrapper-v5'\n    '{retire}': '#wrapper-v5'"),
        );
    assert_eq!(v5.trim_end(), expected.trim_end());

    let manifest = include_str!("../conduit.yaml");
    assert!(manifest.contains("- claude-code.v4\n- claude-code.v5\n"));
    let (_, items) = manifest.split_once("  wrapper-v4:\n").unwrap();
    let (v4, v5) = items.split_once("  wrapper-v5:\n").unwrap();
    let expected = v4
        .replace(
            &format!("    - '{retire}'"),
            &format!("    - '{setup}'\n    - '{retire}'"),
        )
        .replace(
            &format!("      '{retire}': retire"),
            &format!("      '{setup}': setup\n      '{retire}': retire"),
        );
    assert_eq!(v5.trim_end(), expected.trim_end());
}
