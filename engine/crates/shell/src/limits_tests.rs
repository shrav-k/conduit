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
