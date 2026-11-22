mod package_target;
mod plan_apply_support;

use conduit_loader::{Syntax, read_text};
use plan_apply_support::{Install, cases, installs, reference, replay, safe_projection, sha256};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

type ArchivePins<'a> = BTreeMap<(&'a str, &'a str), &'a str>;

fn archive_pins(archives: &[Install]) -> ArchivePins<'_> {
    archives
        .iter()
        .map(|archive| {
            let pin = &archive.pin;
            (
                (
                    pin["address"].as_str().unwrap(),
                    pin["version"].as_str().unwrap(),
                ),
                pin["digest"].as_str().unwrap(),
            )
        })
        .collect()
}

fn check_pin(
    address: &str,
    version: &str,
    digest: &str,
    pins: &ArchivePins<'_>,
) -> Result<(), String> {
    match pins.get(&(address, version)) {
        Some(&expected) if digest == expected => Ok(()),
        Some(expected) => Err(format!(
            "archive pin {address} {version}: expected {expected}, found {digest}"
        )),
        None => Err(format!("unknown archive pin {address} {version}")),
    }
}

fn check_pins(value: &Value, parent: &str, pins: &ArchivePins<'_>, case: &str) {
    if let Some(fields) = value.as_object() {
        if let (Some(version), Some(digest)) = (value["version"].as_str(), value["digest"].as_str())
        {
            let address = value["address"].as_str().unwrap_or(parent);
            check_pin(address, version, digest, pins)
                .unwrap_or_else(|error| panic!("{case}: {error}"));
        }
        for (key, item) in fields {
            check_pins(item, key, pins, case);
        }
    } else if let Some(items) = value.as_array() {
        for item in items {
            check_pins(item, parent, pins, case);
        }
    }
}

fn check_manifest_pins(syntax: Syntax, text: &str, pins: &ArchivePins<'_>) -> Result<(), String> {
    let documents = read_text(syntax, text).map_err(|error| error.to_string())?;
    let [manifest] = documents.as_slice() else {
        return Err("expected one manifest".into());
    };
    let manifest = manifest.to_json();
    let packages = manifest["packages"]
        .as_object()
        .ok_or("expected a packages map")?;
    for (address, pin) in packages {
        let version = pin["version"]
            .as_str()
            .ok_or_else(|| format!("{address}: missing version"))?;
        let digest = pin["digest"]
            .as_str()
            .ok_or_else(|| format!("{address}: missing digest"))?;
        check_pin(address, version, digest, pins)?;
    }
    Ok(())
}

fn check_embedded_pins(case: &Value, pins: &ArchivePins<'_>) -> Result<(), String> {
    let authored = case["files"]["conduit.yaml"]
        .as_str()
        .ok_or("missing authored manifest")?;
    check_manifest_pins(Syntax::Yaml, authored, pins)
        .map_err(|error| format!("conduit.yaml: {error}"))?;

    for (index, line) in case["recorded"]["text"]
        .as_str()
        .unwrap()
        .lines()
        .enumerate()
    {
        let result = if let Some(text) = line.strip_prefix("detail manifest=") {
            check_manifest_pins(Syntax::Json, text, pins)
        } else if let Some(detail) = line.strip_prefix("detail filesystem=") {
            let mut fields = detail.split_whitespace();
            match (
                fields.next(),
                fields
                    .next()
                    .and_then(|field| field.strip_prefix("digest=")),
            ) {
                (Some(version), Some(digest)) => {
                    check_pin("example.org/s1/filesystem", version, digest, pins)
                }
                _ => Err("missing filesystem version or digest".into()),
            }
        } else {
            continue;
        };
        result.map_err(|error| format!("plan line {}: {error}", index + 1))?;
    }
    Ok(())
}

#[test]
fn plan_apply_rejects_a_wrong_embedded_archive_pin() {
    let archives = installs();
    let pins = archive_pins(&archives);
    let cases = cases();
    let agents = ("example.org/s1/agents", "0.2.1");
    let filesystem = ("example.org/s1/filesystem", "0.2.1");
    for (index, pointer, (address, version), wrong_pin) in [
        (0, "/files/conduit.yaml", agents, filesystem),
        (0, "/recorded/text", agents, filesystem),
        (
            4,
            "/recorded/text",
            filesystem,
            ("example.org/s1/filesystem", "0.1.1"),
        ),
    ] {
        let mut case = cases[index].clone();
        let expected = pins[&(address, version)];
        let wrong = pins[&wrong_pin];
        let text = case.pointer_mut(pointer).unwrap();
        assert!(text.as_str().unwrap().contains(expected));
        *text = Value::String(text.as_str().unwrap().replacen(expected, wrong, 1));
        let error = check_embedded_pins(&case, &pins).unwrap_err();
        assert!(
            error.ends_with(&format!(
                "archive pin {address} {version}: expected {expected}, found {wrong}"
            )),
            "{error}"
        );
    }
}

#[test]
fn plan_apply_cases_are_consistent() {
    let cases = cases();
    let archives = installs();
    let pins = archive_pins(&archives);
    let readme = fs::read_to_string(reference().join("README.md")).unwrap();
    let mut previous = &Value::Null;
    let mut plans: Vec<String> = Vec::new();
    let mut journals = BTreeMap::new();
    let mut requests_total = 0;
    for case in &cases {
        let id = case["id"].as_str().unwrap();
        check_pins(case, "", &pins, id);
        check_embedded_pins(case, &pins).unwrap_or_else(|error| panic!("{id}: {error}"));
        let text = case["recorded"]["text"].as_str().unwrap();
        let printed = case["printed"].as_str().unwrap();
        let digest = sha256(text);
        let qualified = format!("sha256:{digest}");
        let path = &case["recorded"]["path"];
        let expected_path = format!(".conduit/plans/{digest}.txt");
        assert_eq!(
            case["recorded"]["digest"], qualified,
            "{id}: recorded digest"
        );
        assert!(
            path.is_null() || path == &expected_path,
            "{id}: recorded path expected {expected_path}"
        );
        let result_text = printed
            .strip_prefix(text)
            .unwrap_or_else(|| panic!("{id}: printed plan differs from recorded text"));
        assert_eq!(
            &case["applied"], previous,
            "{id}: applied differs from preceding kept"
        );
        let requests = case["requests"].as_array().unwrap();
        requests_total += requests.len();
        let actual: Vec<_> = result_text
            .lines()
            .filter(|line| line.starts_with("result "))
            .collect();
        assert_eq!(actual.len(), requests.len(), "{id}: printed results");
        let mut expected_output = String::new();
        for (i, (request, actual)) in requests.iter().zip(actual).enumerate() {
            let step = i + 1;
            let key = format!("plan:{digest}:1:{step}");
            assert_eq!(request["idempotencyKey"], key, "{id}: step {step} key");
            let result = &request["result"];
            let outcome = result["outcome"].as_str().unwrap();
            let expected = format!("result {step} {outcome} {}", safe_projection(result));
            assert_eq!(actual, expected, "{id}: step {step} printed result");
            expected_output.push_str(&expected);
            expected_output.push('\n');
            if let Some(actions) = result["pendingAction"].as_array() {
                for action in actions {
                    let action = action.as_str().unwrap();
                    expected_output.push_str(&format!("panel action {action}: sign-in required\n"));
                }
            }
        }
        assert_eq!(
            result_text, expected_output,
            "{id}: printed results and panel actions"
        );

        if path.is_null() {
            assert_eq!(
                case["kept"]["files"], case["applied"]["files"],
                "{id}: no-write case changed file hashes"
            );
        } else {
            for (file, contents) in case["files"].as_object().unwrap() {
                let sha = sha256(contents.as_str().unwrap());
                assert_eq!(case["kept"]["files"][file], sha, "{id}: kept file {file}");
            }
            plans.push(qualified.clone());
            journals.insert(qualified.clone(), format!("safe:case:{id}/requests"));
        }
        assert_eq!(
            case["kept"]["plans"],
            serde_json::json!(plans),
            "{id}: kept plans"
        );
        assert_eq!(
            case["kept"]["journals"],
            serde_json::json!(journals),
            "{id}: kept journals"
        );

        for name in ["files", "printed", "recorded"] {
            let serialized = case[name].to_string();
            for suffix in serialized.split("sha256:").skip(1) {
                let candidate = suffix.get(..64).unwrap_or("");
                if candidate.len() == 64 && candidate.bytes().all(|b| b.is_ascii_hexdigit()) {
                    let pin = format!("sha256:{candidate}");
                    assert!(
                        pins.values().any(|known| **known == pin) || pin == qualified,
                        "{id}: unknown {name} digest {pin}"
                    );
                }
            }
        }
        let columns: Vec<_> = readme
            .lines()
            .find(|line| line.starts_with(&format!("| [{id}]")))
            .unwrap()
            .split('|')
            .map(str::trim)
            .collect();
        for (column, count) in [
            (4, requests.len()),
            (5, text.lines().count()),
            (6, printed.lines().count()),
        ] {
            assert_eq!(
                columns[column].parse::<usize>().unwrap(),
                count,
                "{id}: count in column {column}"
            );
        }
        previous = &case["kept"];
    }
    assert_eq!(requests_total, 60);
}

#[tokio::test]
async fn plan_apply_cases_replay_in_chain_order() {
    let replay = replay(&cases()).await;
    let expected: BTreeSet<_> = include_str!("expected/plan-apply-mismatches.txt")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect();
    let extra: Vec<_> = replay.lines.difference(&expected).collect();
    let missing: Vec<_> = expected.difference(&replay.lines).collect();
    assert!(
        extra.is_empty() && missing.is_empty(),
        "unexpected mismatches: {extra:?}\nmissing mismatches: {missing:?}"
    );
}
