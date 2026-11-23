use super::*;
use crate::login_resource_tests::{complete, invoke, project, run_case, subject};

const LATEST: &str =
    include_str!("../../../../conformance/conventions/requests/limits-latest/case.yaml");
const KEPT: &str =
    include_str!("../../../../conformance/conventions/requests/limits-kept/case.yaml");
const KEY: &str = "io.github.shrav-k.conduit/limits";

fn login_project() -> packages::Project {
    project(
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/conduit.json"
        ),
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/documents.json"
        ),
    )
}

fn with_read(mut project: packages::Project) -> packages::Project {
    let manifest = project.manifest.as_mut().unwrap();
    let api = manifest["apiVersion"].clone();
    manifest["implementations"]["reader"] = json!({"implements":["#limits.v1"],"plain":true,"tools":{"#limits.v1":"limits"},
               "command":["test-reader"]});
    manifest["implementations"]["resource"]["implements"]
        .as_array_mut()
        .unwrap()
        .push(json!("#limits.v1"));
    let login_type = project
        .documents
        .iter_mut()
        .find(|doc| doc["metadata"]["name"] == "login-holder.v1")
        .unwrap();
    login_type["spec"]["offers"] = json!(["#reading.v1"]);
    login_type["spec"]["implementation"]["#limits.v1"] = json!("#resource");

    project.documents.extend([
        json!({"apiVersion":api,"kind":"Operation","metadata":{"name":"limits.v1"},
               "spec":{"inputSchema":{"type":"object"}}}),
        json!({"apiVersion":api,"kind":"Capability","metadata":{"name":"reading.v1"},
               "spec":{"operations":["#limits.v1"]}}),
        json!({"apiVersion":api,"kind":"Type","metadata":{"name":"ordinary.v1"},
               "spec":{"family":"Resource","schema":{"type":"object"},"offers":["#reading.v1"],
                       "implementation":{"#limits.v1":"#reader"}}}),
        json!({"apiVersion":api,"kind":"Resource","metadata":{"name":"ordinary-t"},
               "spec":{"type":"#ordinary.v1","config":{}}}),
        json!({"apiVersion":api,"kind":"Resource","metadata":{"name":"ordinary"},
               "from":"#ordinary-t","owner":"logins"}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"reading"},
               "spec":{"capability":"#reading.v1","operations":["#limits.v1"]}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"read-login"},
               "from":"#reading","consumer":{"kind":"Interface","name":"root"},
               "target":{"kind":"Resource","name":"login"}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"read-ordinary"},
               "from":"#reading","consumer":{"kind":"Interface","name":"root"},
               "target":{"kind":"Resource","name":"ordinary"}}),
    ]);
    project
}

fn limits_case(
    source: &str,
    project: packages::Project,
    steps: usize,
) -> (State, Context, BTreeMap<String, Value>) {
    let mut fixture: Value = serde_json::from_str(source).unwrap();
    for step in fixture["steps"].as_array_mut().unwrap() {
        // The helper runs reply checks and captures. The tests inspect limits themselves.
        if let Some(capture) = step["capture"].as_object_mut() {
            capture.retain(|_, path| path.as_str().unwrap().starts_with("/reply/"));
        }
        if let Some(reply) = step["expect"]["reply"].as_object_mut()
            && reply
                .get("structuredContent")
                .is_some_and(|content| content["status"].get("limits").is_some())
        {
            reply.remove("structuredContent");
        }
    }
    run_case(&fixture.to_string(), project, steps)
}

fn root_call(state: &State, c: &Context, serial: u32, tool: &str, arguments: Value) -> Transition {
    invoke(
        state,
        c,
        serial,
        &subject(state, "Interface", "root")["id"],
        tool,
        arguments,
    )
}

fn inspect(state: &State, c: &Context, serial: u32, route: &str, kind: &str, name: &str) -> Value {
    let inspection = root_call(
        state,
        c,
        serial,
        &format!("{route}.inspect.v1"),
        json!({"object":{"kind":kind,"name":name}}),
    );
    assert_eq!(inspection.reply["isError"], false, "inspect {kind}/{name}");
    assert!(inspection.external_intents.is_empty());
    inspection.reply["structuredContent"]["status"].clone()
}

fn open_read(state: &State, c: &Context, serial: u32, route: &str) -> State {
    let opened = root_call(state, c, serial, "access.open.v1", json!({"conduit":route}));
    assert_eq!(
        opened.reply["isError"], false,
        "open {route}: {:?}",
        opened.reply
    );
    opened.state
}

fn ready_resource(state: &mut State, name: &str) {
    let resource = state
        .objects
        .iter_mut()
        .find(|object| object["kind"] == "Resource" && object["name"] == name)
        .unwrap();
    // The runner is scripted, so the test supplies the Resource's readiness.
    resource["state"] = json!("ready");
}

fn report(observed_at: &str, used: u32) -> Value {
    json!({
        "client":{"name":"example-client","version":"1.0.0"},
        "method":"scripted","observedAt":observed_at,"usageAllowed":true,
        "windows":[{"id":"example/short","windowMinutes":5,"usedPercent":used,
                    "resetsAt":"2026-11-21T13:07:00Z","state":"allowed","scope":null}]
    })
}

fn finish_call(call: &Transition, c: &Context, serial: u32, report: &Value) -> Transition {
    assert_eq!(call.external_intents.len(), 1, "dispatch the scripted call");
    complete(
        &call.state,
        c,
        serial,
        &call.reply["invocation"],
        json!({"resultType":"complete","isError":false,"structuredContent":{},
               "_meta":{format!("{}outcome", c.meta_prefix):"succeeded", KEY:report}}),
    )
}

fn ending<'a>(ended: &'a Transition, call: &Transition) -> &'a Value {
    ended
        .records
        .iter()
        .find(|record| {
            record["type"] == "invocation-ended"
                && record["data"]["invocation"] == call.reply["invocation"]
        })
        .expect("missing invocation-ended record")
}

#[test]
fn a_login_resources_own_read_keeps_its_limits_in_its_status() {
    let (mut state, c, _) = limits_case(LATEST, with_read(login_project()), 17);
    ready_resource(&mut state, "login");
    let mut state = open_read(&state, &c, 201, "read-login");
    let a = inspect(&state, &c, 202, "workers", "Agent", "a");
    let b = inspect(&state, &c, 203, "workers", "Agent", "b");
    let login = subject(&state, "Resource", "login")["id"].clone();
    let report = report("2026-11-21T13:04:00Z", 70);

    let call = root_call(&state, &c, 204, "read-login.limits.v1", json!({}));
    assert_eq!(call.external_intents.len(), 1);
    let requests::Completion::Result { result } = requests::advertised_extension(&c) else {
        unreachable!();
    };
    let call = complete(&call.state, &c, 213, &call.reply["invocation"], result);
    let ended = finish_call(&call, &c, 205, &report);
    assert!(ended.reply["_meta"].get(KEY).is_none());
    let status = inspect(&ended.state, &c, 206, "logins", "Resource", "login");
    let window = &status["limits"]["windows"][0];
    assert_eq!(
        window["usedPercent"], 70,
        "login's own read is shown in its status"
    );
    let record = ending(&ended, &call);
    assert_eq!(record["data"]["limits"], report);
    assert_eq!(
        window["instance"],
        json!({"kind":"Resource","name":"login","id":login})
    );
    assert_eq!(window["invocation"], call.reply["invocation"]);
    assert_eq!(window["source"], record["source"]);
    assert_eq!(
        inspect(&ended.state, &c, 207, "workers", "Agent", "a")["limits"],
        a["limits"]
    );
    assert_eq!(
        inspect(&ended.state, &c, 208, "workers", "Agent", "b")["limits"],
        b["limits"]
    );

    state = ended.state;
    state.sign_in_holds.insert(
        login.as_str().unwrap().into(),
        json!({"since":"2026-11-21T13:00:00Z"}),
    );
    let held = root_call(&state, &c, 209, "read-login.limits.v1", json!({}));
    assert_eq!(
        held.reply["_meta"][format!("{}reason", c.meta_prefix)],
        "unavailable"
    );
    assert!(held.external_intents.is_empty());
    assert_eq!(
        inspect(&held.state, &c, 210, "logins", "Resource", "login")["limits"],
        status["limits"]
    );
    assert_eq!(
        inspect(&held.state, &c, 211, "workers", "Agent", "a")["limits"],
        a["limits"]
    );
    assert_eq!(
        inspect(&held.state, &c, 212, "workers", "Agent", "b")["limits"],
        b["limits"]
    );
}

#[test]
fn limits_status_outlasts_a_purge_and_takes_the_later_of_equal_times() {
    let (state, c, _) = limits_case(LATEST, login_project(), 17);
    let before = inspect(&state, &c, 221, "workers", "Agent", "a")["limits"].clone();
    let login = inspect(&state, &c, 222, "logins", "Resource", "login")["limits"].clone();
    assert_eq!(
        before["state"], "observed",
        "a's turns are shown in its status"
    );
    let purged = root_call(
        &state,
        &c,
        223,
        "management.purge.v1",
        json!({"before":"2026-11-21T13:04:00Z"}),
    );
    assert_eq!(purged.reply["isError"], false);
    assert!(
        purged
            .state
            .records
            .iter()
            .all(|record| record["data"].get("limits").is_none())
    );
    let reloaded: State =
        serde_json::from_str(&serde_json::to_string(&purged.state).unwrap()).unwrap();
    assert_eq!(reloaded, purged.state);
    assert_eq!(
        inspect(&reloaded, &c, 224, "workers", "Agent", "a")["limits"],
        before
    );
    assert_eq!(
        inspect(&reloaded, &c, 225, "logins", "Resource", "login")["limits"],
        login
    );

    let call = root_call(&reloaded, &c, 226, "call-a.turn.v1", json!({}));
    let ended = finish_call(&call, &c, 227, &report("2026-11-21T15:01:00+02:00", 83));
    let later = inspect(&ended.state, &c, 228, "workers", "Agent", "a")["limits"].clone();
    assert_eq!(later["newest"]["invocation"], call.reply["invocation"]);
    assert_eq!(later["windows"][0]["usedPercent"], 83);
    assert_eq!(later["windows"][1], before["windows"][1]);
    assert_eq!(
        inspect(&ended.state, &c, 229, "logins", "Resource", "login")["limits"],
        login
    );

    let call = root_call(&ended.state, &c, 230, "call-a.turn.v1", json!({}));
    let ended = finish_call(&call, &c, 231, &report("2026-11-21T15:00:59+02:00", 99));
    assert_eq!(
        inspect(&ended.state, &c, 232, "workers", "Agent", "a")["limits"],
        later
    );
    assert_eq!(
        inspect(&ended.state, &c, 233, "logins", "Resource", "login")["limits"],
        login
    );
}

#[test]
fn limits_preserves_subnanosecond_report_order() {
    let (state, c, _) = limits_case(LATEST, login_project(), 17);
    let call = root_call(&state, &c, 251, "call-a.turn.v1", json!({}));
    let newer = report("2026-11-21T13:01:00.0000000002Z", 83);
    let ended = finish_call(&call, &c, 252, &newer);
    assert_eq!(ending(&ended, &call)["data"]["limits"], newer);
    let latest = inspect(&ended.state, &c, 253, "workers", "Agent", "a")["limits"].clone();
    assert_eq!(latest["newest"]["invocation"], call.reply["invocation"]);
    assert_eq!(latest["windows"][0]["usedPercent"], 83);

    let call = root_call(&ended.state, &c, 254, "call-a.turn.v1", json!({}));
    let older = report("2026-11-21T15:01:00.0000000001+02:00", 99);
    let ended = finish_call(&call, &c, 255, &older);
    assert_eq!(ending(&ended, &call)["data"]["limits"], older);
    assert!(
        inspect(&ended.state, &c, 256, "workers", "Agent", "a")["limits"] == latest,
        "an older report kept second must leave the newest report and window unchanged"
    );

    let call = root_call(&ended.state, &c, 257, "call-a.turn.v1", json!({}));
    let equal = report("2026-11-21T15:01:00.000000000200+02:00", 91);
    let ended = finish_call(&call, &c, 258, &equal);
    let latest = inspect(&ended.state, &c, 259, "workers", "Agent", "a");
    assert_eq!(
        latest["limits"]["newest"]["invocation"],
        call.reply["invocation"]
    );
    assert_eq!(latest["limits"]["windows"][0]["usedPercent"], 91);
}

#[test]
fn limits_preserves_subnanosecond_reset_time() {
    let (state, c, _) = limits_case(LATEST, login_project(), 17);
    let reset = "2026-11-21T15:07:00.0000000002+02:00";
    let mut observed = report("2026-11-21T13:02:00Z", 83);
    observed["windows"][0]["resetsAt"] = json!(reset);
    let call = root_call(&state, &c, 261, "call-a.turn.v1", json!({}));
    let ended = finish_call(&call, &c, 262, &observed);
    assert_eq!(ending(&ended, &call)["data"]["limits"], observed);
    let agent = subject(&ended.state, "Agent", "a");
    let before =
        limits::status(&ended.state, agent, "2026-11-21T13:07:00.0000000001Z", &c).unwrap();
    assert!(
        before["windows"][0]["state"] == "allowed",
        "a window must remain allowed at an inspection immediately before its precise reset"
    );
    let at_reset =
        limits::status(&ended.state, agent, "2026-11-21T13:07:00.000000000200Z", &c).unwrap();
    assert_eq!(at_reset["windows"][0]["state"], "unknown");
    assert_eq!(at_reset["windows"][0]["resetsAt"], reset);
}

#[test]
fn repeated_limits_warning_names_indices_without_quoting_the_id() {
    let (state, c, _) = limits_case(LATEST, login_project(), 17);
    let mut duplicate = report("2026-11-21T13:02:00Z", 83);
    let first = duplicate["windows"][0].clone();
    let mut other = first.clone();
    other["id"] = json!("example/daily");
    duplicate["windows"]
        .as_array_mut()
        .unwrap()
        .extend([other, first]);
    let call = root_call(&state, &c, 271, "call-a.turn.v1", json!({}));
    let ended = finish_call(&call, &c, 272, &duplicate);
    let data = &ending(&ended, &call)["data"];
    assert!(data.get("limits").is_none());
    let warning = &data["limitsDropped"];
    assert!(warning.get("properties").is_none());
    assert_eq!(
        warning["locations"][0]["logicalLocations"][0]["fullyQualifiedName"],
        "/_meta/io.github.shrav-k.conduit~1limits/windows"
    );
    let text = warning["message"]["text"].as_str().unwrap();
    assert!(text.contains("indices 0 and 2 have the same id"));
    assert!(!text.contains("example/short"));
}

#[test]
fn only_agents_and_login_resources_show_limits() {
    let (mut state, c, _) = limits_case(LATEST, with_read(login_project()), 10);
    ready_resource(&mut state, "ordinary");
    let state = open_read(&state, &c, 241, "read-ordinary");
    let report = report("2026-11-21T13:01:00Z", 30);
    let call = root_call(&state, &c, 242, "read-ordinary.limits.v1", json!({}));
    let ended = finish_call(&call, &c, 243, &report);
    assert_eq!(
        ending(&ended, &call)["data"]["limits"],
        report,
        "an ordinary Resource's call keeps its report in its end record"
    );
    for (serial, route, kind, name) in [
        (244, "management", "Conduit", "read-ordinary"),
        (245, "management", "Pool", "logins"),
        (246, "logins", "Resource", "ordinary"),
    ] {
        assert!(
            inspect(&ended.state, &c, serial, route, kind, name)
                .get("limits")
                .is_none()
        );
    }
    for (serial, route, kind, name) in [
        (247, "workers", "Agent", "a"),
        (248, "workers", "Agent", "b"),
        (249, "logins", "Resource", "login"),
    ] {
        assert_eq!(
            inspect(&ended.state, &c, serial, route, kind, name)["limits"],
            json!({"state":"unknown","windows":[]})
        );
    }

    let (state, c, captures) = limits_case(
        KEPT,
        project(
            include_str!(
                "../../../../conformance/conventions/explain/failed-model-turn-content/conduit.json"
            ),
            include_str!(
                "../../../../conformance/conventions/explain/failed-model-turn-content/documents.json"
            ),
        ),
        usize::MAX,
    );
    assert!(login_resource::id(subject(&state, "Agent", "agent")).is_none());
    let status = inspect(&state, &c, 250, "fleet", "Agent", "agent");
    assert_eq!(status["limits"]["state"], "observed");
    assert_eq!(
        status["limits"]["newest"]["invocation"],
        captures["failedTurn"]
    );
}
