use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::time::Duration;

use refresh_probe::{Arm, BLOCK, Case, Form, Hooks, OVERRIDE, Run, Scratch};
use serde_json::{Value, json};

#[test]
fn environment_is_cleared_for_both_arms() {
    // This is the only test in this integration-test process. There are no
    // other threads accessing its environment when this mutation happens.
    unsafe { std::env::set_var("REFRESH_PROBE_CANARY", "excluded") };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/test-scratch");
    fs::create_dir_all(&root).unwrap();
    let scratch = Scratch::new(&root, "environment").unwrap();
    for (arm, expected) in [
        (
            Arm::Blocked,
            json!({"environment":["CODEX_HOME",OVERRIDE,"HOME","PATH"],"override":BLOCK}),
        ),
        (
            Arm::Control,
            json!({"environment":["CODEX_HOME","HOME","PATH"],"override":null}),
        ),
    ] {
        let observed = RefCell::new(Value::Null);
        let hooks = Hooks {
            prepare: &|_, home, _| fs::write(home.join("fake-script.json"), b"{}"),
            inspect: &|home| {
                let record = fs::read_to_string(home.join("fake-record.jsonl"))?;
                *observed.borrow_mut() = serde_json::from_str(record.lines().next().unwrap())?;
                Ok(())
            },
        };
        let run = Run {
            client: Path::new(env!("CARGO_BIN_EXE_fake-app-server")),
            scratch: scratch.path(),
            logs: scratch.path(),
            case: Case::P1,
            arm,
            repetition: 1,
            form: Form::Issuer,
            hold: Duration::ZERO,
        };
        let report = run.execute(Some(&hooks));
        assert!(report["fault"].is_null());
        assert_eq!(observed.into_inner(), expected);
    }
}
