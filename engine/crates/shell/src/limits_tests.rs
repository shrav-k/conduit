use super::*;

#[test]
fn a_secret_read_ignores_its_limits_in_its_invocation_end() {
    struct StoreServer<'a> {
        shell: &'a Shell,
        member: Value,
    }
    impl Executor for StoreServer<'_> {
        fn dispatch(&self, intent: &Value) -> Result<Execution> {
            if let Some(discovery) = scripted_discovery(intent) {
                return Ok(discovery);
            }
            assert_eq!(intent["call"]["params"]["name"], "get-secret.v1");
            let result = json!({
                "isError":false,"structuredContent":{"value":""},
                "_meta":{format!("{}outcome",self.shell.context.meta_prefix):"succeeded",
                         format!("{}limits",self.shell.context.meta_prefix):self.member}
            });
            Ok(Execution::Complete(
                json!({"kind":"result","result":result}),
            ))
        }

        fn builtin_runner(&self, intent: &Value, _: Option<&RunIdentity>) -> Result<Execution> {
            let report = match intent["operation"]
                .as_str()
                .unwrap()
                .rsplit_once('#')
                .unwrap()
                .1
            {
                "inspect-run.v1" => json!({"kind":"inspect-run","state":"stopped"}),
                "run.v1" => json!({"kind":"run","outcome":"succeeded"}),
                _ => panic!("unexpected runner operation"),
            };
            Ok(Execution::SetupReport(report))
        }
    }

    let report = json!({
        "client":{"name":"example-client","version":"1.0.0"},"method":"scripted",
        "observedAt":"2026-11-21T13:00:00Z","usageAllowed":null,"windows":[],"noWindows":"no-event"
    });
    for member in [report, json!("malformed limits")] {
        let temp = temporary();
        let shell = open(temp.path());
        shell
            .submit(
                json!({"kind":"first-start","machine":local_machine(BTreeMap::new()).unwrap()}),
                &MissingExecutor,
            )
            .unwrap();
        let root = actor(&shell, "root");
        let spec = conduit_loader::Specification::read(&repository().join("spec")).unwrap();
        let project = boundary::project(
            &repository().join("conformance/conventions/secret-reads/project"),
            &spec,
        )
        .unwrap();
        let register = shell
            .submit(
                json!({"kind":"invoke","request":{
                    "identity":{"kind":"instance","id":root},"tool":"management.register.v1",
                    "arguments":{"documents":project.documents,"manifest":project.manifest}
                }}),
                &MissingExecutor,
            )
            .unwrap();
        assert_eq!(register["isError"], false);

        let reply = shell
            .submit(
                json!({"kind":"invoke","request":{
                    "identity":{"kind":"instance","id":root},"tool":"management.set-up.v1",
                    "arguments":{"instance":{"kind":"Pool","name":"demo"}}
                }}),
                &StoreServer {
                    shell: &shell,
                    member,
                },
            )
            .unwrap();
        assert_eq!(
            reply["_meta"][format!("{}outcome", shell.context.meta_prefix)],
            "succeeded"
        );
        assert!(
            reply["_meta"]
                .get(format!("{}limits", shell.context.meta_prefix))
                .is_none()
        );
        let exported = shell
            .submit(json!({"kind":"export-records"}), &MissingExecutor)
            .unwrap();
        let records = exported["records"].as_array().unwrap();
        let read = records
            .iter()
            .find(|record| {
                record["type"] == "invocation-ended"
                    && record["data"]["operation"]
                        == format!("{}#get-secret.v1", shell.context.core_package)
            })
            .expect("missing secret read end");
        assert_eq!(read["data"]["outcome"], "succeeded");
        for record in records {
            assert!(record["data"].get("limits").is_none());
            assert!(record["data"].get("limitsDropped").is_none());
        }
    }
}

#[test]
fn limits_signals_read_from_an_earlier_engines_store() {
    // DL2: opening and inspecting an older observation neither fills nor writes signals.
    let temp = temporary();
    let shell = established(temp.path());
    let agent = actor(&shell, "agent-b");
    let entry = json!({"newest":{"observedAt":"2026-11-21T13:00:00Z",
        "source":shell.context.record_source,
        "instance":{"kind":"Agent","name":"agent-b","id":agent},
        "invocation":"00000000-0000-4000-8000-000000315001","usageAllowed":null},
        "windows":{}});
    drop(shell);
    let mut stored: store::Stored =
        serde_json::from_slice(&fs::read(temp.path().join("state.json")).unwrap()).unwrap();
    stored.state.limit_observations.insert(
        agent.clone(),
        serde_json::from_value(entry.clone()).unwrap(),
    );
    let store = Store::open(temp.path()).unwrap();
    store.write(&stored).unwrap();
    drop(store);
    for _ in 0..2 {
        let shell = open(temp.path());
        let root = actor(&shell, "root");
        let fleet = actor(&shell, "s1");
        let snapshot = shell.state().unwrap();
        assert_eq!(
            serde_json::to_value(&snapshot.limit_observations[&agent]).unwrap(),
            entry
        );
        let route = snapshot
            .objects
            .iter()
            .find(|o| {
                o["kind"] == "Conduit"
                    && o["consumer"] == root
                    && o["target"] == fleet
                    && o.get("spec").is_none()
            })
            .unwrap()["local_name"]
            .as_str()
            .unwrap();
        let reply = shell
            .submit(
                json!({"kind":"invoke","request":{
            "identity":{"kind":"instance","id":root},"tool":format!("{route}.inspect.v1"),
            "arguments":{"object":{"kind":"Agent","name":"agent-b"}}}}),
                &MissingExecutor,
            )
            .unwrap();
        assert_eq!(reply["isError"], false);
        let status = &reply["structuredContent"]["status"]["limits"];
        assert_eq!(
            status,
            &json!({"state":"observed","newest":entry["newest"],"windows":[]})
        );
        let spec = conduit_loader::Specification::read(&repository().join("spec")).unwrap();
        assert!(
            spec.contract_validator("limit-status.v1")
                .unwrap()
                .is_valid(status)
        );
        assert_memory_equals_disk(&shell);
        let memory = shell.state().unwrap();
        assert_eq!(
            serde_json::to_value(&memory.limit_observations[&agent]).unwrap(),
            entry
        );
        let disk: store::Stored =
            serde_json::from_slice(&fs::read(temp.path().join("state.json")).unwrap()).unwrap();
        assert_eq!(
            serde_json::to_value(&disk.state.limit_observations[&agent]).unwrap(),
            entry
        );
        let disk: Value =
            serde_json::from_slice(&fs::read(temp.path().join("state.json")).unwrap()).unwrap();
        assert_eq!(disk["state"]["limit_observations"][&agent], entry);
    }
}
