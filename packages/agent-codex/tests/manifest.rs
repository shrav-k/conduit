use std::{fs, path::Path};

#[test]
fn wrapper_v2_path_changes_only_the_program() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = fs::read_to_string(root.join("conduit.yaml")).unwrap();
    let mut items: Vec<(&str, Vec<&str>)> = Vec::new();
    let mut in_implementations = false;
    for line in manifest.lines() {
        if !in_implementations {
            in_implementations = line == "implementations:";
            continue;
        }
        if !line.is_empty() && !line.starts_with(' ') {
            break;
        }
        if let Some(name) = line
            .strip_prefix("  ")
            .and_then(|s| s.strip_suffix(':'))
            .filter(|s| !s.is_empty() && !s.chars().any(char::is_whitespace))
        {
            items.push((name, Vec::new()));
        } else if let Some((name, lines)) = items.last_mut() {
            assert!(
                line.is_empty() || line.starts_with("   "),
                "{name}: unexpected item line {line:?}"
            );
            lines.push(line);
        } else {
            panic!("wrapper: expected item header before line {line:?}");
        }
    }

    let expected_names = ["wrapper", "wrapper-v2", "wrapper-v2-path"];
    for (index, expected) in expected_names.iter().enumerate() {
        assert_eq!(
            items.get(index).map(|(name, _)| *name),
            Some(*expected),
            "{expected}: item header line {} differs",
            index + 1
        );
    }
    assert_eq!(
        items.len(),
        expected_names.len(),
        "wrapper-v2-path: expected end after item, extra header line {:?}",
        items.get(expected_names.len()).map(|(name, _)| name)
    );

    let archive_command = format!("    command: [bin/{}]", agent_adapter::WRAPPER);
    let path_command = format!("    command: [{}]", agent_adapter::WRAPPER);
    assert!(
        !agent_adapter::WRAPPER.contains('/'),
        "wrapper-v2-path: command line {path_command:?} must contain a program name without '/'"
    );
    let original = &items[1].1;
    let by_name = &items[2].1;
    let command_index = original
        .iter()
        .position(|line| line.trim_start().starts_with("command:"))
        .expect("wrapper-v2: missing command line");
    assert_eq!(
        original[command_index],
        archive_command,
        "wrapper-v2: command line {} differs",
        command_index + 1
    );
    for index in 0..original.len().max(by_name.len()) {
        let expected = if index == command_index {
            Some(path_command.as_str())
        } else {
            original.get(index).copied()
        };
        assert_eq!(
            by_name.get(index).copied(),
            expected,
            "wrapper-v2-path: item line {} differs from wrapper-v2 (only command may change)",
            index + 1
        );
    }

    let contracts = fs::read_to_string(root.join("contracts.yaml")).unwrap();
    for (index, line) in contracts.lines().enumerate() {
        assert!(
            !line.contains("wrapper-v2-path"),
            "wrapper-v2-path: contracts.yaml line {} selects the opt-in item: {line:?}",
            index + 1
        );
    }
}
