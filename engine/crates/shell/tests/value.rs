mod package_target;
mod plan_apply_support;

use conduit_shell::{Config, Result, sign_in::Person, value};
use plan_apply_support::{Engine, cli::*, result, sha256};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    path::Path,
    process::{Output, Stdio},
    sync::{Arc, Mutex},
};

const CORE: &str = "github.com/shrav-k/conduit/core";
const API: &str = "github.com/shrav-k/conduit/v0";
const OLD: &str = "fixture old account";
const NEW: &str = "fixture new account";
const HOME: &str = "fixture home directory";
const WARNING: &str = "A value is not secret: the engine keeps it as configuration, conduit inspect and conduit value list --show show it, and conduit records exports it. A password or token goes on a secret Resource, named by a secret reference (SEC-001), never in a value.";
const NOT_TERMINAL: &str = "conduit value set: reads the value from your terminal, and standard input is not a terminal; a script registers a value Resource as root with management.register.v1 (MCP or conduit call); nothing was read or sent";

struct Script<'a> {
    answers: VecDeque<&'a str>,
    transcript: Vec<String>,
    asked: usize,
    invalid_at: Option<usize>,
}

impl Person for Script<'_> {
    fn ask(&mut self, prompt: &str) -> Result<Option<String>> {
        self.asked += 1;
        self.transcript.push(prompt.into());
        if self.invalid_at == Some(self.asked) {
            use std::io::BufRead;
            let mut line = String::new();
            let mut bytes = std::io::Cursor::new([0xff, b'\n']);
            return bytes
                .read_line(&mut line)
                .map(|_| Some(line))
                .map_err(|error| format!("could not read the terminal: {error}"));
        }
        Ok(self.answers.pop_front().map(str::to_owned))
    }

    fn tell(&mut self, line: &str) {
        self.transcript.push(line.into());
    }

    fn open(&mut self, _: &str) -> Result<()> {
        panic!("value never opens a browser")
    }
}

async fn scripted(
    engine: &Engine,
    arguments: &[&str],
    terminal: bool,
    answers: &[&str],
) -> (i32, String, Vec<String>, usize) {
    let mut arguments: Vec<_> = arguments.iter().map(|arg| arg.to_string()).collect();
    arguments.extend([
        "--store".into(),
        engine.directory().to_str().unwrap().into(),
    ]);
    let mut person = Script {
        answers: answers.iter().copied().collect(),
        transcript: Vec::new(),
        asked: 0,
        invalid_at: None,
    };
    let mut output = Vec::new();
    let code = value::run(&arguments, &mut person, terminal, &mut output).await;
    (
        code,
        String::from_utf8(output).unwrap(),
        person.transcript,
        person.asked,
    )
}

fn no_values(output: &str) {
    for text in [OLD, NEW, HOME] {
        assert!(!output.contains(text), "a value appeared on stdout");
    }
}

async fn call(engine: &Engine, tool: &str, arguments: Value) -> Value {
    let reply = result(&engine.call(tool, arguments, None).await);
    assert_eq!(
        reply["outcome"], "succeeded",
        "{tool}: {}",
        reply["diagnostics"]
    );
    reply
}

fn object(name: &str) -> Value {
    json!({"kind":"Resource","name":name})
}

async fn inspect(engine: &Engine, name: &str) -> Value {
    call(engine, "system.inspect.v1", json!({"object":object(name)})).await
}

async fn register(engine: &Engine, name: &str, text: &str, node: Option<&str>) -> Value {
    let mut args = json!({"value":text});
    if let Some(node) = node {
        args["node"] = object(node);
    }
    call(engine, "management.register.v1", json!({"documents":[{
        "apiVersion":API,"kind":"Resource","metadata":{"name":name},"from":format!("{CORE}#value"),
        "owner":"system","arguments":args,
    }]})).await;
    inspect(engine, name).await["id"].clone()
}

async fn worker(engine: &Engine, source: &str) -> Value {
    call(engine, "management.register.v1", json!({
        "manifest":{"apiVersion":API,"address":"example.org/conformance/value-cli-worker"},
        "documents":[
            {"apiVersion":API,"kind":"Type","metadata":{"name":"worker.v1"},"spec":{
                "family":"Resource","schema":{"type":"object","properties":{"account":{"type":"string"}},
                "required":["account"],"additionalProperties":false}
            }},
            {"apiVersion":API,"kind":"Resource","metadata":{"name":"worker-t"},"spec":{
                "type":"#worker.v1","inputs":{"account":{"schema":{"type":"string"},"to":["/config/account"]}}
            }},
            {"apiVersion":API,"kind":"Resource","metadata":{"name":"worker"},"owner":"system","from":"#worker-t",
             "values":{"account":object(source)}}
        ]
    })).await;
    inspect(engine, "worker").await
}

async fn tombstone(engine: &Engine, name: &str) -> Value {
    let listed = call(engine, "management.list.v1", json!({})).await;
    listed["tombstones"]
        .as_array()
        .unwrap()
        .iter()
        .find(|stone| stone["object"]["kind"] == "Resource" && stone["object"]["name"] == name)
        .expect("expected Resource tombstone")
        .clone()
}

fn tombstone_refusal(store: &Path, stone: &Value) -> String {
    let id = stone["id"].as_str().unwrap();
    let old = stone["object"]["id"].as_str().unwrap();
    let name = stone["object"]["name"].as_str().unwrap();
    let pool = stone["owner"]["name"].as_str().unwrap();
    let time = stone["time"].as_str().unwrap();
    let cause = &stone["cause"];
    format!(
        "conduit value set: Tombstone id={id} of Resource/{name} id={old}, owned by Pool/{pool}, retired at {time} (cause {cause}), holds the name; to reuse the name, lift it as root first: conduit call --store '{}' access.lift.v1 '{{\"id\":\"{id}\"}}', then run conduit value set {name} again; nothing was read or sent",
        store.display()
    )
}

fn url(engine: &Engine) -> String {
    Config::for_store(&engine.directory()).unwrap().engine_url
}

fn binary_value(engine: &Engine, args: &[&str]) -> Output {
    let directory = tempfile::tempdir().unwrap();
    let config = tempfile::tempdir().unwrap();
    tokio::task::block_in_place(|| tool(engine, directory.path(), config.path(), args))
}

// Forward root's calls to the serving fixture, changing only the chosen reply.
// This exercises the CLI's real transport and leaves requests after a fault observable.
async fn save_fault(fault: &str) -> (String, String, Vec<String>) {
    use axum::{Json, Router, response::IntoResponse, routing::post};
    let engine = Arc::new(Engine::start().await);
    let old = register(&engine, "acct", OLD, None).await;
    let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))
        .await
        .unwrap();
    let endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
    let forwarded = engine.clone();
    let calls = Arc::new(Mutex::new(Vec::new()));
    let observed = calls.clone();
    let fault = fault.to_owned();
    let app = Router::new().route(
        "/mcp",
        post(move |Json(request): Json<Value>| {
            let engine = forwarded.clone();
            let calls = observed.clone();
            let fault = fault.clone();
            async move {
                let reply = match request["method"].as_str() {
                    Some("initialize") => json!({
                        "protocolVersion": request["params"]["protocolVersion"],
                        "capabilities":{"tools":{}},
                        "serverInfo":{"name":"conduit","version":env!("CARGO_PKG_VERSION")}
                    }),
                    Some("tools/call") => {
                        let tool = request["params"]["name"].as_str().unwrap();
                        calls.lock().unwrap().push(tool.to_owned());
                        if fault == "lift" && tool == "access.lift.v1" {
                            json!({"isError":true,"content":[],"_meta":{
                                "io.github.shrav-k.conduit/outcome":"failed"
                            }})
                        } else {
                            let mut reply = engine
                                .call(tool, request["params"]["arguments"].clone(), None)
                                .await;
                            if tool == "management.list.v1"
                                && reply["structuredContent"]["tombstones"]
                                    .as_array()
                                    .is_some_and(|s| !s.is_empty())
                            {
                                if fault == "missing tombstone" {
                                    reply["structuredContent"]["tombstones"] = json!([]);
                                } else if fault == "list" {
                                    reply = json!({"isError":true,"content":[],"_meta":{
                                        "io.github.shrav-k.conduit/outcome":"failed"
                                    }});
                                }
                            }
                            if fault == "missing ID" && tool == "management.register.v1" {
                                reply["structuredContent"]["objects"] = json!([]);
                            }
                            reply
                        }
                    }
                    _ => return axum::http::StatusCode::ACCEPTED.into_response(),
                };
                Json(json!({"jsonrpc":"2.0","id":request["id"],"result":reply})).into_response()
            }
        }),
    );
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    std::fs::write(
        engine.directory().join("engine-url.json"),
        serde_json::to_vec(&endpoint).unwrap(),
    )
    .unwrap();
    let (code, output, transcript, _) =
        scripted(&engine, &["set", "acct"], true, &[NEW, "y"]).await;
    assert_eq!(code, 1);
    no_values(&output);
    let seen = calls.lock().unwrap().clone();
    let retirement = seen
        .iter()
        .position(|tool| tool == "system.retire.v1")
        .unwrap();
    let seen = seen[retirement..].to_vec();
    let error = transcript
        .last()
        .unwrap()
        .replace(old.as_str().unwrap(), "OLD");
    server.abort();
    let _ = server.await;
    Arc::try_unwrap(engine).ok().unwrap().close().await;
    (output, error, seen)
}

async fn check_save_faults() {
    let before_lift = "; Resource/acct id=OLD is retired and its new value is not registered; instances keep what they hold; check management.list.v1 for its tombstone and lift it as root with access.lift.v1 before running conduit value set acct again";
    let after_lift = "; Resource/acct id=OLD is retired and its new value is not registered; instances keep what they hold; conduit value set acct again registers it";
    let mut errors = Vec::new();
    for (fault, number, tool, reason, ending, remaining) in [
        (
            "lift",
            3,
            "access.lift.v1",
            "did not succeed",
            before_lift,
            "4..5",
        ),
        (
            "list",
            2,
            "management.list.v1",
            "did not succeed",
            before_lift,
            "3..5",
        ),
        (
            "missing tombstone",
            2,
            "management.list.v1",
            "listed no tombstone of Resource/acct id=OLD",
            before_lift,
            "3..5",
        ),
        (
            "missing ID",
            4,
            "management.register.v1",
            "returned no ID for Resource/acct",
            after_lift,
            "5..5",
        ),
    ] {
        let (output, error, calls) = save_fault(fault).await;
        assert!(output.ends_with(&format!("not sent {remaining}\n")));
        assert_eq!(
            calls,
            [
                "system.retire.v1",
                "management.list.v1",
                "access.lift.v1",
                "management.register.v1"
            ][..number]
        );
        if fault.starts_with("missing") {
            assert!(output.contains(&format!(
                "{number} {tool} {} outcome=succeeded\n",
                if number == 2 {
                    "Tombstone/acct"
                } else {
                    "Resource/acct"
                }
            )));
        }
        let wanted =
            format!("conduit value set: {number} {tool} {reason}; nothing more was sent{ending}");
        if error != wanted {
            errors.push(format!("{fault}: expected {wanted:?}, got {error:?}"));
        }
    }
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

async fn check_local_faults(engine: &Engine) {
    let missing = tempfile::tempdir().unwrap();
    let before = engine.write_requests();
    let mut failures = Vec::new();
    for (store, invalid_at, wanted) in [
        (
            missing.path().to_owned(),
            None,
            "store is not initialized; run conduit init",
        ),
        (engine.directory(), Some(1), "could not read the terminal:"),
        (engine.directory(), Some(2), "could not read the terminal:"),
    ] {
        let mut person = Script {
            answers: [NEW, "y"].into(),
            transcript: Vec::new(),
            asked: 0,
            invalid_at,
        };
        let mut output = Vec::new();
        let args = vec![
            "set".into(),
            "acct".into(),
            "--store".into(),
            store.to_str().unwrap().into(),
        ];
        let code = value::run(&args, &mut person, true, &mut output).await;
        let error = person.transcript.last().unwrap();
        assert!(error.contains(wanted));
        assert!(output.is_empty());
        assert_eq!(engine.write_requests(), before);
        if code != 2 || !error.ends_with("; nothing was sent") {
            failures.push(format!("{invalid_at:?}: status={code}, {error}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn value_set_shows_the_value_echoes_it_and_saves_only_on_y() {
    let engine = Engine::start().await;
    check_local_faults(&engine).await;
    let typed = "  fixture \"quoted\" account  ";
    let before = engine.write_requests();
    let (code, output, transcript, asked) =
        scripted(&engine, &["set", "acct"], true, &[typed, "n"]).await;
    assert_eq!((code, asked), (2, 2));
    assert!(output.is_empty());
    assert_eq!(
        transcript,
        [
            WARNING.into(),
            "Type the value of acct, then Enter: ".into(),
            format!("acct = {} (not tied to a node)", json!(typed)),
            format!("Save it as root in the engine at {}? [y/N] ", url(&engine)),
            "conduit value set: not saved: the answer was not y; nothing was sent".into(),
        ]
    );
    assert_eq!(engine.write_requests(), before);
    let (code, output, _, asked) = scripted(&engine, &["set", "acct"], true, &[typed, "YES"]).await;
    assert_eq!((code, asked), (0, 2));
    let instance = inspect(&engine, "acct").await;
    assert_eq!(instance["declared"]["config"]["value"], typed);
    let id = instance["id"].as_str().unwrap();
    assert_eq!(
        output,
        format!(
            "1 management.register.v1 Resource/acct outcome=succeeded\n2 system.set-up.v1 Resource/acct outcome=succeeded\nsaved Resource/acct id={id} node=-\n"
        )
    );
    assert!(!output.contains("quoted"));
    let before = engine.write_requests();
    let (code, output, transcript, asked) = scripted(&engine, &["set", "none"], true, &[]).await;
    assert_eq!((code, asked), (2, 1));
    assert!(output.is_empty());
    assert_eq!(
        transcript.last().unwrap(),
        "conduit value set: no value was read: standard input ended; nothing was sent"
    );
    assert_eq!(engine.write_requests(), before);
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn value_set_replaces_by_retire_lift_and_register() {
    let engine = Engine::start().await;
    let old = register(&engine, "acct", OLD, None).await;
    worker(&engine, "acct").await;
    let (code, output, transcript, asked) =
        scripted(&engine, &["set", "acct"], true, &[NEW, "y"]).await;
    assert_eq!((code, asked), (0, 2));
    let old_id = old.as_str().unwrap();
    assert_eq!(
        transcript[1],
        format!(
            "acct holds {old_value} (not tied to a node) as Resource/acct id={old_id} and fills [Resource/worker /config/account]; each keeps {old_value} until it is replaced (OBJ-010)",
            old_value = json!(OLD)
        )
    );
    assert_eq!(
        transcript[3],
        format!(
            "acct = {} (not tied to a node), replacing {} (not tied to a node)",
            json!(NEW),
            json!(OLD)
        )
    );
    assert_eq!(
        transcript[4],
        format!(
            "Save it as root in the engine at {}? This retires Resource/acct id={old_id}, lifts its tombstone and registers the new value. [y/N] ",
            url(&engine)
        )
    );
    let current = inspect(&engine, "acct").await;
    let id = current["id"].as_str().unwrap();
    assert_ne!(id, old_id);
    assert_eq!(
        output,
        format!(
            "1 system.retire.v1 Resource/acct outcome=succeeded\n2 management.list.v1 Tombstone/acct outcome=succeeded\n3 access.lift.v1 Tombstone/acct outcome=succeeded\n4 management.register.v1 Resource/acct outcome=succeeded\n5 system.set-up.v1 Resource/acct outcome=succeeded\nsaved Resource/acct id={id} node=-\n"
        )
    );
    no_values(&output);
    let held = inspect(&engine, "worker").await;
    assert_eq!(held["declared"]["config"]["account"], OLD);
    assert_eq!(held["values"]["/config/account"]["id"], old);
    let listed = call(&engine, "management.list.v1", json!({})).await;
    assert!(
        listed["tombstones"]
            .as_array()
            .unwrap()
            .iter()
            .all(|stone| stone["object"]["name"] != "acct")
    );
    let before = engine.write_requests();
    let (code, output, _, asked) = scripted(&engine, &["set", "acct"], true, &[NEW]).await;
    assert_eq!((code, asked), (0, 1));
    assert_eq!(
        output,
        format!(
            "unchanged Resource/acct id={id}: the engine holds this value and node; nothing was sent\n"
        )
    );
    no_values(&output);
    assert_eq!(engine.write_requests(), before);
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn value_set_refuses_before_it_reads_a_value() {
    let engine = Engine::start().await;
    register(&engine, "acct", OLD, None).await;
    worker(&engine, "acct").await;
    call(
        &engine,
        "management.register.v1",
        json!({"documents":[{
            "apiVersion":API,"kind":"Resource","metadata":{"name":"value-t"},
            "spec":{"type":format!("{CORE}#value.v1")}
        }]}),
    )
    .await;
    let before = engine.write_requests();
    for (args, terminal, wanted) in [
        (vec!["set","Bad"],true,"conduit value set: \"Bad\" is not a name.v1; nothing was read or sent".to_owned()),
        (vec!["set","x","--node","missing"],true,"conduit value set: root sees no node named \"missing\"; its nodes are [local]; nothing was read or sent".into()),
        (vec!["set","worker"],true,"conduit value set: Resource/worker exists with Type example.org/conformance/value-cli-worker#worker.v1, not value.v1; choose another name; nothing was read or sent".into()),
        (vec!["set","worker-t"],true,"conduit value set: Resource/worker-t exists with Type #worker.v1, not value.v1; choose another name; nothing was read or sent".into()),
        (vec!["set","value-t"],true,"conduit value set: Resource/value-t is a template, not a value instance; choose another name; nothing was read or sent".into()),
        (vec!["set","x"],false,NOT_TERMINAL.into()),
    ] {
        let (code, output, transcript, asked) = scripted(&engine,&args,terminal,&[NEW,"y"]).await;
        assert_eq!((code,asked),(2,0));
        assert!(output.is_empty());
        assert_eq!(transcript,[wanted]);
        assert_eq!(engine.write_requests(), before);
    }
    let dir = tempfile::tempdir().unwrap();
    let config = tempfile::tempdir().unwrap();
    let piped = tokio::task::block_in_place(|| {
        binary(dir.path(), config.path())
            .args(["value", "set", "x", "--store"])
            .arg(engine.directory())
            .stdin(Stdio::piped())
            .output()
            .unwrap()
    });
    assert_eq!(piped.status.code(), Some(2));
    assert!(piped.stdout.is_empty());
    assert_eq!(
        String::from_utf8(piped.stderr).unwrap(),
        format!("{NOT_TERMINAL}\n")
    );
    for (text, wanted) in [
        (
            "",
            "conduit value set: the value is empty; nothing was sent",
        ),
        (
            "é\u{0085}x",
            "conduit value set: the value holds a control character at character 2; a value is one line of text; nothing was sent",
        ),
    ] {
        let (code, output, transcript, asked) =
            scripted(&engine, &["set", "x"], true, &[text, "y"]).await;
        assert_eq!((code, asked), (2, 1));
        assert!(output.is_empty());
        assert_eq!(transcript.last().unwrap(), wanted);
    }
    assert_eq!(engine.write_requests(), before);
    call(
        &engine,
        "system.retire.v1",
        json!({"instance":object("worker")}),
    )
    .await;
    let stone = tombstone(&engine, "worker").await;
    let before = engine.write_requests();
    let (code, output, transcript, asked) =
        scripted(&engine, &["set", "worker"], true, &[NEW, "y"]).await;
    assert_eq!((code, asked), (2, 0));
    assert!(output.is_empty());
    assert_eq!(transcript, [tombstone_refusal(&engine.directory(), &stone)]);
    assert_eq!(engine.write_requests(), before);
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn value_set_sets_up_a_registered_value_and_tells_a_failed_set_up() {
    check_save_faults().await;
    let engine = Engine::start().await;
    let id = register(&engine, "acct", OLD, None).await;
    let id = id.as_str().unwrap();
    let (code, output, transcript, asked) =
        scripted(&engine, &["set", "acct"], true, &[OLD, "y"]).await;
    assert_eq!((code, asked), (0, 2));
    assert_eq!(
        transcript.last().unwrap(),
        &format!(
            "Resource/acct id={id} holds this value and node but is not set up; set it up as root in the engine at {}? [y/N] ",
            url(&engine)
        )
    );
    assert_eq!(
        output,
        format!(
            "1 system.set-up.v1 Resource/acct outcome=succeeded\nsaved Resource/acct id={id} node=-\n"
        )
    );
    no_values(&output);
    let (code, output, _, asked) = scripted(&engine, &["set", "acct"], true, &[OLD]).await;
    assert_eq!((code, asked), (0, 1));
    assert_eq!(
        output,
        format!(
            "unchanged Resource/acct id={id}: the engine holds this value and node; nothing was sent\n"
        )
    );

    let hook = format!("{CORE}#setup-hook.v1");
    call(&engine,"management.register.v1",json!({
        "manifest":{"apiVersion":API,"address":"example.org/conformance/value-cli","implementations":{
            "setup-probe":{"remote":{"type":"streamable-http","url":"https://fixture.invalid/setup"},"implements":[hook]}
        }},
        "documents":[
            {"apiVersion":API,"kind":"Resource","metadata":{"name":"probe-t"},"spec":{
                "type":format!("{CORE}#value.v1"),"config":{"value":NEW},"implementation":{hook:"#setup-probe"}
            }},
            {"apiVersion":API,"kind":"Resource","metadata":{"name":"probe"},"owner":"system","from":"#probe-t"}
        ]
    })).await;
    let observed = inspect(&engine, "probe").await;
    let probe_id = observed["id"].as_str().unwrap();
    engine.fail_next_setup();
    let (code, output, transcript, _) =
        scripted(&engine, &["set", "probe"], true, &[NEW, "y"]).await;
    assert_eq!(code, 1, "{transcript:?}");
    assert!(output.starts_with("1 system.set-up.v1 Resource/probe outcome=failed\n"));
    no_values(&output);
    assert_eq!(
        transcript.last().unwrap(),
        &format!(
            "conduit value set: 1 system.set-up.v1 did not succeed; nothing more was sent; Resource/probe id={probe_id} is registered but not set up; conduit value set probe again sets it up"
        )
    );
    let (code, output, _, _) = scripted(&engine, &["set", "probe"], true, &[NEW, "y"]).await;
    assert_eq!(code, 0);
    assert_eq!(
        output,
        format!(
            "1 system.set-up.v1 Resource/probe outcome=succeeded\nsaved Resource/probe id={probe_id} node=-\n"
        )
    );
    no_values(&output);
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn value_list_names_nodes_and_fills_and_values_only_with_show() {
    let engine = Engine::start().await;
    let acct = register(&engine, "acct", OLD, None).await;
    let home = register(&engine, "home", HOME, Some("local")).await;
    worker(&engine, "acct").await;
    let fills = json!([{"kind":"Resource","name":"worker","place":"/config/account"}]);
    let rows = [
        json!({"fills":fills,"id":acct,"live":true,"name":"acct","node":null}),
        json!({"fills":[],"id":home,"live":true,"name":"home","node":"local"}),
    ];
    for show in [false, true] {
        let arguments: &[&str] = if show {
            &["value", "list", "--show"]
        } else {
            &["value", "list"]
        };
        let output = binary_value(&engine, arguments);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let mut wanted = rows.clone();
        if show {
            wanted[0]["value"] = json!(OLD);
            wanted[1]["value"] = json!(HOME);
        } else {
            no_values(stdin_free(&output));
        }
        assert_eq!(
            stdin_free(&output),
            format!("{}\n{}\n", wanted[0], wanted[1])
        );
    }
    let (code, output, _, _) = scripted(&engine, &["set", "acct"], true, &[NEW, "y"]).await;
    assert_eq!(code, 0);
    no_values(&output);
    let id = inspect(&engine, "acct").await["id"].clone();
    let output = binary_value(&engine, &["value", "list"]);
    assert_eq!(output.status.code(), Some(0));
    no_values(stdin_free(&output));
    assert_eq!(
        stdin_free(&output),
        format!(
            "{}\n{}\n{}\n",
            json!({"fills":fills,"id":acct,"live":false,"name":null,"node":null}),
            json!({"fills":[],"id":id,"live":true,"name":"acct","node":null}),
            rows[1]
        )
    );
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn value_unset_refuses_while_an_instance_names_it() {
    let engine = Engine::start().await;
    register(&engine, "acct", OLD, None).await;
    let home = register(&engine, "home", HOME, None).await;
    worker(&engine, "acct").await;
    let before = engine.write_requests();
    let output = binary_value(&engine, &["value", "unset", "acct"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "conduit value unset: Resource/acct fills [Resource/worker /config/account]; each keeps its value until it is replaced or retired, so a value an instance names is not retired; retire or replace each, then unset again; nothing was sent\n"
    );
    assert_eq!(engine.write_requests(), before);
    let output = binary_value(&engine, &["value", "unset", "home"]);
    assert_eq!(output.status.code(), Some(0));
    no_values(stdin_free(&output));
    let id = home.as_str().unwrap();
    assert_eq!(
        stdin_free(&output),
        format!(
            "1 system.retire.v1 Resource/home outcome=succeeded\nunset Resource/home id={id}: retired; its tombstone holds the name until root lifts it with access.lift.v1; conduit value set home then registers it again\n"
        )
    );
    let list = binary_value(&engine, &["value", "list"]);
    assert_eq!(list.status.code(), Some(0));
    assert!(!stdin_free(&list).contains("\"name\":\"home\""));
    let stone = tombstone(&engine, "home").await;
    let before = engine.write_requests();
    let (code, output, transcript, asked) =
        scripted(&engine, &["set", "home"], true, &[HOME, "y"]).await;
    assert_eq!((code, asked), (2, 0));
    assert!(output.is_empty());
    assert_eq!(transcript, [tombstone_refusal(&engine.directory(), &stone)]);
    assert_eq!(engine.write_requests(), before);
    call(&engine, "access.lift.v1", json!({"id":stone["id"]})).await;
    let (code, output, _, _) = scripted(&engine, &["set", "home"], true, &[HOME, "y"]).await;
    assert_eq!(code, 0);
    assert!(output.starts_with("1 management.register.v1 Resource/home outcome=succeeded\n"));
    no_values(&output);
    let before = engine.write_requests();
    let absent = binary_value(&engine, &["value", "unset", "absent"]);
    assert_eq!(absent.status.code(), Some(2));
    assert!(absent.stdout.is_empty());
    assert_eq!(
        String::from_utf8(absent.stderr).unwrap(),
        "conduit value unset: root sees no value Resource named absent; conduit value list names those it sees; nothing was sent\n"
    );
    assert_eq!(engine.write_requests(), before);
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_is_unchanged_by_values_outside_the_project() {
    let (engine, config, dir, case) = first_case().await;
    let (code, output, _, _) = scripted(&engine, &["set", "extra"], true, &[NEW, "y"]).await;
    assert_eq!(code, 0);
    no_values(&output);
    let out = tokio::task::block_in_place(|| tool(&engine, dir.path(), config.path(), &["plan"]));
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = stdin_free(&out);
    no_values(text);
    assert_eq!(text, case["recorded"]["text"].as_str().unwrap());
    assert_eq!(
        json!(format!("sha256:{}", sha256(text))),
        case["recorded"]["digest"]
    );
    engine.close().await;
}
