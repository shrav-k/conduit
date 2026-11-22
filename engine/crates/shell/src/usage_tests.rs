use super::*;

fn usage() -> Value {
    json!({
        "client": {"name":"example-client","version":"1.0.0"},
        "models": {
            "example-model-a": {"input":1200,"cachedInput":800,"output":300,"reasoning":120},
            "example-model-b": {"input":40,"output":7}
        }
    })
}

fn usage_key(shell: &Shell) -> String {
    format!("{}usage", shell.context.meta_prefix)
}

fn with_usage(shell: &Shell, mut result: Value, usage: Value) -> Value {
    result["_meta"][usage_key(shell)] = usage;
    result
}

fn export(shell: &Shell) -> Value {
    shell
        .submit(json!({"kind":"export-records"}), &MissingExecutor)
        .unwrap()
}

fn ending<'a>(export: &'a Value, id: &str) -> &'a Value {
    export["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|record| record["type"] == "invocation-ended" && record["data"]["invocation"] == id)
        .expect("missing invocation-ended record")
}

fn assert_no_usage(record: &Value) {
    assert!(record["data"].get("usage").is_none());
    assert!(record["data"].get("usageDropped").is_none());
}

fn plain_results(
    shell: &Shell,
    directory: &Path,
    members: Vec<Option<Value>>,
) -> Vec<(String, Value)> {
    let count = members.len();
    let executor = Scripted {
        replies: Mutex::new(
            members
                .into_iter()
                .map(|member| {
                    let mut result =
                        json!({"isError":false,"content":[{"type":"text","text":"written"}]});
                    if let Some(member) = member {
                        result = with_usage(shell, result, member);
                    }
                    json!({"kind":"complete","completion":{"kind":"result","result":result}})
                })
                .collect(),
        ),
        directory: directory.into(),
        dispatched: AtomicUsize::new(0),
    };
    let replies = (0..count)
        .map(|index| {
            let reply = shell
                .submit(
                    invoke(
                        shell,
                        "agent-b",
                        "project.file.write.v1",
                        Some(&format!("usage-{index}")),
                    ),
                    &executor,
                )
                .unwrap();
            assert_eq!(
                reply["_meta"][format!("{}outcome", shell.context.meta_prefix)],
                "succeeded"
            );
            assert!(reply["_meta"].get(usage_key(shell)).is_none());
            let mut reply = reply;
            let id = reply["_meta"]
                .as_object_mut()
                .unwrap()
                .remove(&format!("{}invocation", shell.context.meta_prefix))
                .unwrap();
            (id.as_str().unwrap().into(), reply)
        })
        .collect();
    assert_eq!(executor.dispatched.load(Ordering::SeqCst), count);
    replies
}

#[test]
fn a_plain_result_keeps_its_usage_in_the_end_record() {
    let temp = temporary();
    let shell = established(temp.path());
    let reported = usage();
    let replies = plain_results(&shell, temp.path(), vec![Some(reported.clone()), None]);
    let (id, reply) = &replies[0];
    assert_ne!(*id, replies[1].0);
    assert_eq!(*reply, replies[1].1);

    let before = export(&shell);
    let end = ending(&before, id);
    assert_eq!(
        end["data"].get("usage"),
        Some(&reported),
        "plain end is missing the reported usage"
    );
    assert!(end["data"].get("usageDropped").is_none());
    for record in before["records"].as_array().unwrap() {
        if record["id"] != end["id"] {
            assert_no_usage(record);
        }
    }
    shell
        .submit(
            json!({"kind":"expire-outputs","invocation":id}),
            &MissingExecutor,
        )
        .unwrap();
    assert_eq!(export(&shell), before);
    drop(shell);
    assert_eq!(export(&open(temp.path())), before);
}

#[test]
fn a_keyed_result_keeps_its_usage_in_the_end_record() {
    let temp = temporary();
    let shell = filtered_shell(temp.path());
    let ticket = shell
        .begin(filtered_read(&shell, "usage"), &Deferred)
        .unwrap();
    let id = ticket.invocation().unwrap();
    let reported = usage();
    let result = with_usage(
        &shell,
        json!({
            "isError":false,"structuredContent":{"text":"read"},
            "_meta":{format!("{}outcome",shell.context.meta_prefix):"succeeded"}
        }),
        reported.clone(),
    );
    shell
        .complete(
            id,
            1,
            json!({"kind":"result","result":result}),
            &MissingExecutor,
        )
        .unwrap();
    let reply = ticket.try_reply().unwrap().unwrap();
    assert_eq!(reply["structuredContent"]["text"], "read");
    assert!(reply["_meta"].get(usage_key(&shell)).is_none());
    let exported = export(&shell);
    assert_eq!(
        ending(&exported, id)["data"].get("usage"),
        Some(&reported),
        "keyed end is missing the reported usage"
    );
}

#[test]
fn a_secret_read_ignores_its_usage_in_its_invocation_end() {
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
            let result = with_usage(
                self.shell,
                json!({
                    "isError":false,"structuredContent":{"value":""},
                    "_meta":{format!("{}outcome",self.shell.context.meta_prefix):"succeeded"}
                }),
                self.member.clone(),
            );
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

    for member in [usage(), json!("malformed usage")] {
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
        assert!(reply["_meta"].get(usage_key(&shell)).is_none());
        let exported = export(&shell);
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
            assert_no_usage(record);
        }
    }
}

#[test]
fn a_malformed_usage_is_dropped_with_a_warning() {
    let temp = temporary();
    let shell = established(temp.path());
    let mut extra = usage();
    extra["account"] = json!("synthetic-account-label");
    let mut model = usage();
    model["models"]["synthetic-model"] = json!({"input":40,"output":-8675309});
    let replies = plain_results(
        &shell,
        temp.path(),
        vec![
            None,
            Some(json!("synthetic-usage-value")),
            Some(extra),
            Some(json!({"client":{"name":"","version":"1.0.0"}})),
            Some(model),
        ],
    );
    let exported = export(&shell);
    assert_no_usage(ending(&exported, &replies[0].0));
    let base = format!(
        "/_meta/{}",
        usage_key(&shell).replace('~', "~0").replace('/', "~1")
    );
    for ((id, reply), (at, keyword)) in replies[1..].iter().zip([
        ("", "/type"),
        ("", "/additionalProperties"),
        ("", "/required"),
        (
            "/models",
            "/properties/models/additionalProperties/properties/output/minimum",
        ),
    ]) {
        assert_eq!(*reply, replies[0].1);
        let data = &ending(&exported, id)["data"];
        assert!(data.get("usage").is_none());
        let warning = data
            .get("usageDropped")
            .expect("malformed end is missing its usageDropped warning");
        let pointer = format!("{base}{at}");
        let last = keyword.rsplit('/').next().unwrap();
        assert_eq!(
            *warning,
            json!({
                "ruleId":"IFC-051","level":"warning",
                "locations":[{"logicalLocations":[{"fullyQualifiedName":pointer}]}],
                "properties":{"keywordLocation":keyword},
                "message":{"text":format!("The result's usage member was dropped: at {pointer} it does not conform to usage.v1, failing its {last} keyword. The record of this invocation's end keeps no part of it, and nothing else about the call changes (IFC-051).")}
            })
        );
    }
    for text in ["synthetic-", "8675309"] {
        assert!(!exported.to_string().contains(text));
        assert!(
            replies
                .iter()
                .all(|(_, reply)| !reply.to_string().contains(text))
        );
        assert!(
            !serde_json::to_string(shell.state().unwrap().as_ref())
                .unwrap()
                .contains(text)
        );
    }
}
