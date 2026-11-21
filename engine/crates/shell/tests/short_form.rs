use std::{fs, process::Command};

#[test]
fn short_form_directory_registers_through_the_cli() {
    let temporary = tempfile::tempdir().unwrap();
    let store = temporary.path().join("store");
    let project = temporary.path().join("project");
    fs::create_dir(&project).unwrap();
    fs::write(
        project.join("pools.yaml"),
        r#"apiVersion: github.com/shrav-k/conduit/v0
pools: {library: {type: 'github.com/shrav-k/conduit/core#pool.v1'}}
"#,
    )
    .unwrap();
    let cli = |verb| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_conduit"));
        command.args([verb, "--store"]).arg(&store);
        command
    };
    let init = cli("init").output().unwrap();
    assert!(init.status.success(), "store initialization failed");
    let register = cli("register").arg(&project).output().unwrap();
    assert!(
        register.status.success(),
        "sectioned registration failed: {}",
        String::from_utf8_lossy(&register.stderr)
    );
    let persisted: serde_json::Value =
        serde_json::from_slice(&fs::read(store.join("state.json")).unwrap()).unwrap();
    let objects = persisted["state"]["objects"].as_array().unwrap();
    for name in ["library", "library--t"] {
        assert!(
            objects.iter().any(|object| object["name"] == name),
            "{name}"
        );
    }
}
