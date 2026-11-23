use super::*;

const RESOLVED: &str =
    include_str!("../../../../conformance/language/registration/values-resolved/case.yaml");

fn fixture(change: impl FnOnce(&mut Vec<Value>, &Context)) -> (State, Context) {
    let c = crate::login_resource_tests::context();
    let case: Value = serde_json::from_str(RESOLVED).unwrap();
    let started = crate::step(
        &State::default(),
        serde_json::from_value(case["steps"][0]["input"].clone()).unwrap(),
        &c,
    )
    .unwrap();
    let mut input = case["steps"][1]["input"].clone();
    change(
        input["request"]["arguments"]["documents"]
            .as_array_mut()
            .unwrap(),
        &c,
    );
    input["evidence"]["ids"] = json!(
        (1..100)
            .map(|i| format!("00000000-0000-4000-a297-000000{i:06}"))
            .collect::<Vec<_>>()
    );
    input["evidence"]["revisions"] = json!(vec!["initial-revision"; 40]);
    let registered =
        crate::step(&started.state, serde_json::from_value(input).unwrap(), &c).unwrap();
    assert_eq!(
        registered.reply["isError"], false,
        "{}",
        registered.reply["structuredContent"]["diagnostics"]
    );
    (registered.state, c)
}

fn object<'a>(state: &'a State, name: &str) -> &'a Value {
    state.objects.iter().find(|o| o["name"] == name).unwrap()
}

fn call(state: &State, c: &Context, serial: usize, tool: &str, args: Value) -> Transition {
    crate::step(
        state,
        serde_json::from_value(json!({
            "kind":"invoke",
            "request":{
                "identity":{"kind":"instance","id":"00000000-0000-4000-8000-000000000005"},
                "tool":tool,"arguments":args
            },
            "evidence":{
                "time":"2026-11-23T12:00:00Z",
                "ids":(1..100).map(|i| format!("00000000-0000-4000-b297-{serial:06}{i:06}")).collect::<Vec<_>>(),
                "revisions":vec!["replacement-revision";40],
                "suffixes":(0..10).map(|i| format!("{serial:03}{i:02}")).collect::<Vec<_>>()
            }
        }))
        .unwrap(),
        c,
    )
    .unwrap()
}

fn retire_nested(state: &mut State, holder: &str, slot: &str) {
    let id = object(state, holder)["id"].clone();
    for o in &mut state.objects {
        if o["nested"]["parent"] == id && o["nested"]["name"] == slot {
            o["state"] = json!("retired");
        }
    }
}

fn replace(state: &State, c: &Context, serial: usize, holder: &str, slot: &str) -> Transition {
    call(
        state,
        c,
        serial,
        "management.register-replacement.v1",
        json!({"instance":{"kind":object(state, holder)["kind"],"name":holder},"nested":slot}),
    )
}

fn replacement_object(t: &Transition) -> &Value {
    assert_eq!(
        t.reply["isError"], false,
        "{}",
        t.reply["structuredContent"]["diagnostics"]
    );
    let id = &t.reply["structuredContent"]["object"]["id"];
    t.state.objects.iter().find(|o| o["id"] == *id).unwrap()
}

fn normalized(object: &Value) -> Value {
    fn visit(value: &mut Value, id: &Value, name: &Value) {
        if value == id {
            *value = json!("generated-id");
        } else if value == name {
            *value = json!("generated-name");
        } else {
            match value {
                Value::Object(map) => map.values_mut().for_each(|v| visit(v, id, name)),
                Value::Array(items) => items.iter_mut().for_each(|v| visit(v, id, name)),
                _ => {}
            }
        }
    }
    let mut result = object.clone();
    visit(&mut result, &object["id"], &object["name"]);
    result
}

fn assert_holder_kept(before: &State, after: &State, name: &str) {
    let mut expected = object(before, name).clone();
    let actual = object(after, name);
    expected["slots"] = actual["slots"].clone();
    assert!(expected == *actual, "only the holder's slot may change");
}

fn assert_refusal(before: &State, t: &Transition, rule: &str) {
    assert_eq!(t.reply["isError"], true);
    assert_eq!(
        t.reply["_meta"]["io.github.shrav-k.conduit/reason"],
        "invalid"
    );
    let diagnostics = t.reply["structuredContent"]["diagnostics"]
        .as_array()
        .unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0]["ruleId"], rule);
    assert!(before.objects == t.state.objects, "object state changed");
    assert!(
        before.installed_packages == t.state.installed_packages,
        "installed packages changed"
    );
    assert!(
        before.tombstones == t.state.tombstones,
        "tombstones changed"
    );
    assert_eq!(before.invocations, t.state.invocations, "no dispatch");
    assert!(t.external_intents.is_empty());
    for scalar in [
        "synthetic-installation-label",
        "/example/install-value-directory",
    ] {
        assert!(!t.reply.to_string().contains(scalar));
        assert!(!serde_json::to_string(&t.records).unwrap().contains(scalar));
    }
}

#[test]
fn value_replacement_node_constraint() {
    let (mut state, c) = fixture(|_, _| {});
    retire_nested(&mut state, "team", "worker");
    // Core fixture: public controls do not store a new held placement on this base.
    let mut other_node = object(&state, "local").clone();
    other_node["id"] = json!("00000000-0000-4000-a297-999999999999");
    other_node["name"] = json!("other-node");
    other_node["metadata"]["name"] = json!("other-node");
    state.objects.push(other_node.clone());
    let parent = state
        .objects
        .iter_mut()
        .find(|o| o["name"] == "team")
        .unwrap();
    parent["entries"]["worker"]["placement"] = json!({"kind":"Resource","id":other_node["id"]});
    let t = replace(&state, &c, 1, "team", "worker");
    assert_refusal(&state, &t, "OBJ-009");
    assert_eq!(
        t.reply["structuredContent"]["diagnostics"],
        json!([{
            "ruleId":"OBJ-009","level":"error",
            "message":{"text":"team-t: /spec/entries/worker/values/home names {kind: Resource, name: home}: the value Resource is tied to node local; the instance is placed on node other-node; the replacement takes the value held at registration: replace worker-t so it accepts it, or register team again to resolve its values anew"},
            "locations":[
                {"logicalLocations":[{"fullyQualifiedName":"/objects/team-t/spec/entries/worker/values/home"}]},
                {"logicalLocations":[{"fullyQualifiedName":"/objects/team/entries/worker/placement"}]}
            ]
        }])
    );
}

#[test]
fn value_replacement_declared_conduit() {
    let (mut state, c) = fixture(|docs, c| {
        let api = docs[0]["apiVersion"].clone();
        docs.extend([
            json!({"apiVersion":api,"kind":"Requirement","metadata":{"name":"marker.v1"},
                "spec":{"inputs":{"tag":{"schema":{"type":"string"}}}}}),
            json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"channel-t"},
                "spec":{"capability":c.qualified("access-control.v1"),"operations":[],
                    "inputs":{"tag":{"schema":{"type":"string"},"to":["/requirements/marker/arguments/tag"]}},
                    "requirements":{"marker":{"requirement":"#marker.v1"}}}}),
            json!({"apiVersion":api,"kind":"Interface","metadata":{"name":"console-t"},
                "spec":{"type":c.qualified("interface.v1"),"conduits":{"channel":{
                    "from":"#channel-t","target":{"kind":"Resource","name":"access"},
                    "values":{"tag":{"kind":"Resource","name":"acct"}}}}}}),
            json!({"apiVersion":api,"kind":"Interface","metadata":{"name":"console"},"from":"#console-t"}),
        ]);
    });
    retire_nested(&mut state, "console", "channel");
    // Closing a declared Conduit removes it; a replacement does not reopen it.
    state
        .objects
        .retain(|o| !(o["kind"] == "Conduit" && o["state"] == "retired"));
    let source = object(&state, "acct")["id"].clone();
    state.objects.retain(|o| o["id"] != source);
    let t = replace(&state, &c, 2, "console", "channel");
    let new = replacement_object(&t);
    assert_eq!(
        new["requirements"]["marker"]["arguments"]["tag"],
        "synthetic-installation-label"
    );
    assert_eq!(
        new["_value_sources"]["/requirements/marker/arguments/tag"],
        json!({"kind":"Resource","id":source})
    );
    assert_holder_kept(&state, &t.state, "console");
    assert_ne!(new["open"], true);
    assert!(t.external_intents.is_empty());
}

#[test]
fn value_replacement_equal_held_sources() {
    let (mut state, c) = fixture(|_, _| {});
    retire_nested(&mut state, "team", "worker");
    let first = replace(&state, &c, 3, "team", "worker");
    let second = replace(&state, &c, 4, "team", "worker");
    assert!(
        normalized(replacement_object(&first)) == normalized(replacement_object(&second)),
        "normalized replacement objects differ"
    );
    assert_holder_kept(&state, &first.state, "team");
    assert_holder_kept(&state, &second.state, "team");
}

#[test]
fn value_replacement_different_held_source() {
    fn scenario(alternative: bool) -> (State, Context) {
        fixture(|docs, _| {
            let mut other = docs[0].clone();
            other["metadata"]["name"] = json!("other-acct");
            docs.push(other);
            if alternative {
                docs[5]["values"]["account"]["name"] = json!("other-acct");
            }
        })
    }
    let (mut left, c) = scenario(false);
    let (mut right, _) = scenario(true);
    retire_nested(&mut left, "team", "worker");
    retire_nested(&mut right, "team", "worker");
    let first = replace(&left, &c, 5, "team", "worker");
    let second = replace(&right, &c, 6, "team", "worker");
    let mut a = normalized(replacement_object(&first));
    let b = normalized(replacement_object(&second));
    assert_eq!(a["config"], b["config"]);
    assert_ne!(
        a["_value_sources"]["/config/account"],
        b["_value_sources"]["/config/account"]
    );
    a["_value_sources"]["/config/account"] = b["_value_sources"]["/config/account"].clone();
    assert_eq!(
        a["_value_sources"], b["_value_sources"],
        "only that source map entry differs"
    );
}

#[test]
fn value_replacement_current_template_checks() {
    fn substitute(value: &mut Value, captures: &BTreeMap<String, Value>) {
        match value {
            Value::String(text) => {
                if let Some(captured) = text.strip_prefix('$').and_then(|name| captures.get(name)) {
                    *value = captured.clone();
                }
            }
            Value::Object(map) => map.values_mut().for_each(|v| substitute(v, captures)),
            Value::Array(items) => items.iter_mut().for_each(|v| substitute(v, captures)),
            _ => {}
        }
    }
    let case: Value = serde_json::from_str(include_str!(
        "../../../../conformance/language/registration/replacement-checks/case.yaml"
    ))
    .unwrap();
    let c = crate::login_resource_tests::context();
    let mut state = State::default();
    let mut captures = BTreeMap::new();
    let mut refusals = 0;
    for step in case["steps"].as_array().unwrap() {
        let mut input = step["input"].clone();
        substitute(&mut input, &captures);
        let t = crate::step(&state, serde_json::from_value(input).unwrap(), &c).unwrap();
        if step["expect"]["reply"]["isError"] == true {
            let expected = &step["expect"]["reply"]["structuredContent"]["diagnostics"];
            assert_refusal(&state, &t, expected[0]["ruleId"].as_str().unwrap());
            assert_eq!(t.reply["structuredContent"]["diagnostics"], *expected);
            refusals += 1;
        } else if step["expect"]["reply"]["isError"] == false {
            assert_eq!(
                t.reply["isError"], false,
                "{}",
                t.reply["structuredContent"]["diagnostics"]
            );
        }
        if let Some(map) = step["capture"].as_object() {
            for (name, pointer) in map {
                let pointer = pointer.as_str().unwrap().strip_prefix("/reply").unwrap();
                captures.insert(name.clone(), t.reply.pointer(pointer).unwrap().clone());
            }
        }
        state = t.state;
    }
    assert_eq!(refusals, 4);
}

#[test]
fn value_replacement_type_schema() {
    let (mut state, c) = fixture(|_, _| {});
    retire_nested(&mut state, "team", "worker");
    let case: Value = serde_json::from_str(RESOLVED).unwrap();
    let args = &case["steps"][1]["input"]["request"]["arguments"];
    let mut ty = args["documents"][2].clone();
    ty["spec"]["schema"]["properties"]["account"] = json!({"type":"integer"});
    let updated = call(
        &state,
        &c,
        7,
        "management.replace.v1",
        json!({
            "template":args["documents"][3],"contracts":[ty],"manifest":args["manifest"],
            "revision":object(&state,"worker-t")["revision"]
        }),
    );
    assert_eq!(updated.reply["isError"], false);
    let t = replace(&updated.state, &c, 8, "team", "worker");
    assert_refusal(&updated.state, &t, "INS-014");
    let d = &t.reply["structuredContent"]["diagnostics"][0];
    assert_eq!(
        d["message"]["text"],
        "team: /values/account names {kind: Resource, name: acct}: the receiving Type schema failed (/properties/account/type) (destination /config/account in template worker-t); the replacement takes the value held at registration: replace worker-t so it accepts it, or register team again to resolve its values anew"
    );
    assert_eq!(
        d["locations"][1]["logicalLocations"][0]["fullyQualifiedName"],
        "/objects/worker-t/spec/config/account"
    );
}

#[test]
fn value_replacement_forwarded_sources() {
    let (mut state, c) = fixture(|docs, c| {
        let api = docs[0]["apiVersion"].clone();
        docs[3]["spec"]["inputs"]["account"]["to"] =
            json!(["/config/account", "/conduits/channel/arguments/tag"]);
        docs[3]["spec"]["conduits"] = json!({"channel":{
            "from":"#forwarded-t","target":{"kind":"Resource","name":"access"}
        }});
        docs.extend([
            json!({"apiVersion":api,"kind":"Requirement","metadata":{"name":"tagged.v1"},
                "spec":{"inputs":{"tag":{"schema":{"type":"string"}}}}}),
            json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"forwarded-t"},
                "spec":{"capability":c.qualified("access-control.v1"),"operations":[],
                    "inputs":{"tag":{"schema":{"type":"string"},"to":["/requirements/tagged/arguments/tag"]}},
                    "requirements":{"tagged":{"requirement":"#tagged.v1"}}}}),
        ]);
    });
    retire_nested(&mut state, "team", "worker");
    let source = object(&state, "acct")["id"].clone();
    state
        .objects
        .retain(|o| o["name"] != "acct" && o["name"] != "home");
    let first = replace(&state, &c, 9, "team", "worker");
    let worker = replacement_object(&first);
    let held_at = "/conduits/channel/arguments/tag";
    assert_eq!(
        worker["_value_sources"][held_at],
        json!({"kind":"Resource","id":source})
    );
    assert_eq!(
        worker["_value_origins"][held_at]["place"],
        json!({"name":"team","pointer":"/values/account"})
    );
    assert!(worker["conduits"]["channel"].get("values").is_none());
    let name = worker["name"].as_str().unwrap().to_owned();
    let worker_id = worker["id"].clone();
    let mut next = first.state;
    next.objects.retain(|o| o["nested"]["parent"] != worker_id);
    let second = replace(&next, &c, 10, &name, "channel");
    let conduit = replacement_object(&second);
    assert_eq!(
        conduit["_value_sources"]["/requirements/tagged/arguments/tag"],
        json!({"kind":"Resource","id":source})
    );
    assert_holder_kept(&next, &second.state, &name);

    // A changed forwarded input must be checked before its scalar could resolve.
    let original = object(&next, "forwarded-t");
    let mut template = json!({
        "apiVersion":"github.com/shrav-k/conduit/v0","kind":"Conduit",
        "metadata":{"name":"forwarded-t"},"spec":original["spec"]
    });
    template["spec"]["inputs"]["tag"]["schema"] =
        json!({"$ref":c.qualified("resource-reference.v1")});
    let case: Value = serde_json::from_str(RESOLVED).unwrap();
    let updated = call(
        &next,
        &c,
        11,
        "management.replace.v1",
        json!({
            "template":template,
            "contracts":[{"apiVersion":"github.com/shrav-k/conduit/v0","kind":"Requirement",
                "metadata":{"name":"tagged.v1"},"spec":{"inputs":{"tag":{"schema":{"type":"string"}}}}}],
            "manifest":case["steps"][1]["input"]["request"]["arguments"]["manifest"],
            "revision":original["revision"]
        }),
    );
    assert_eq!(
        updated.reply["isError"], false,
        "{}",
        updated.reply["structuredContent"]["diagnostics"]
    );
    let mut forwarded = updated.state.clone();
    retire_nested(&mut forwarded, "team", "worker");
    let refused = replace(&forwarded, &c, 12, "team", "worker");
    assert_refusal(&forwarded, &refused, "OBJ-009");
    let refused = replace(&updated.state, &c, 13, &name, "channel");
    assert_refusal(&updated.state, &refused, "OBJ-009");
    let diagnostic = &refused.reply["structuredContent"]["diagnostics"][0];
    assert_eq!(
        diagnostic["message"]["text"],
        format!(
            "team: /values/account names {{kind: Resource, name: acct}}: a reference schema governs this place (input tag in template forwarded-t); the replacement takes the value held at registration: replace forwarded-t so it accepts it, or register {name} again to resolve its values anew"
        )
    );
    assert_eq!(
        diagnostic["locations"][1]["logicalLocations"][0]["fullyQualifiedName"],
        "/objects/forwarded-t/spec/inputs/tag"
    );
}
