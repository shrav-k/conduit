use super::*;

#[test]
fn admission_reports_read_from_an_earlier_engines_store() {
    let temp = temporary();
    let shell = established(temp.path());
    let disk: Value =
        serde_json::from_slice(&fs::read(temp.path().join("state.json")).unwrap()).unwrap();
    assert!(disk["state"].get("admission_reports").is_none());
    let root = actor(&shell, "root");
    shell
        .submit(
            json!({"kind":"invoke","request":{
                "identity":{"kind":"instance","id":root},"tool":"management.inspect.v1",
                "arguments":{"object":{"kind":"Interface","name":"root"}}
            }}),
            &MissingExecutor,
        )
        .unwrap();
    let mut disk: Value =
        serde_json::from_slice(&fs::read(temp.path().join("state.json")).unwrap()).unwrap();
    assert!(disk["state"].get("admission_reports").is_none());
    assert_memory_equals_disk(&shell);
    drop(shell);
    let report = json!({
        "admit":true,"reason":"within-levers","lever":null,"window":null,
        "paidUsage":{"possible":null,"inUse":null,"resetsAt":null},"cannotTell":null,
        "lifts":null,"observedAt":"2026-11-21T12:00:00Z",
        "source":{"client":null,"method":"example-read"},"validForSeconds":3600,"singleUse":false
    });
    let id = "00000000-0000-4000-a317-000000000001";
    let entry = json!({"report":report,"invocation":"00000000-0000-4000-a317-000000000002",
        "keptAt":"2026-11-21T13:01:00Z","ended":null});
    disk["state"]["admission_reports"] = json!({id:entry});
    let stored: store::Stored = serde_json::from_value(disk).expect("read the new report state");
    let store = Store::open(temp.path()).unwrap();
    store.write(&stored).unwrap();
    drop(store);
    for _ in 0..2 {
        let shell = open(temp.path());
        assert_eq!(
            serde_json::to_value(shell.state().unwrap().as_ref()).unwrap()["admission_reports"][id],
            entry
        );
        let disk: Value =
            serde_json::from_slice(&fs::read(temp.path().join("state.json")).unwrap()).unwrap();
        assert_eq!(disk["state"]["admission_reports"][id], entry);
        assert_memory_equals_disk(&shell);
    }
}
