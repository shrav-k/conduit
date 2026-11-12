//! Synthetic MCP peer. This process's stdout/stderr are captured, never copied to reports.
use provider_contracts::names;
use serde_json::{Value, json};
use std::io::{self, BufRead, Write};
fn main() {
    if std::env::args().nth(1).as_deref() == Some("--version") {
        let name = std::env::args()
            .nth(2)
            .unwrap_or("scripted-provider".into());
        let version = std::env::args().nth(3).unwrap_or("1.0.0".into());
        println!("{}", json!({"name":name,"version":version}));
        return;
    }
    let contract = std::env::args().nth(1).unwrap_or_default();
    let mode = std::env::var("FIXTURE_MODE").unwrap_or_default();
    if mode == "exit" {
        return;
    }
    let stdin = io::stdin();
    let mut out = io::stdout().lock();
    let mut repaired = false;
    let mut calls = 0;
    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            break;
        };
        let Ok(request) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let Some(id) = request.get("id") else {
            continue;
        };
        let method = request["method"].as_str().unwrap_or("");
        let mut extensions = json!({names::extension():{"revision":"v0"}});
        if mode == "no-extension" {
            extensions = json!({});
        }
        if mode == "wrong-revision" {
            extensions = json!({names::extension():{"revision":"v9"}});
        }
        let mut response = match method {
            "initialize" => {
                let revision = if mode == "baseline" || mode == "revision-modern" {
                    "2026-07-28"
                } else {
                    "2025-11-25"
                };
                json!({"jsonrpc":"2.0","id":id,"result":{"protocolVersion":revision,"capabilities":{"tools":{},"extensions":extensions},"serverInfo":{"name":"scripted","version":"1.0.0"}}})
            }
            "server/discover" => {
                json!({"jsonrpc":"2.0","id":id,"result":{"resultType":"complete","supportedVersions":["2025-11-25","2026-07-28"],"capabilities":{"tools":{},"extensions":extensions},"ttlMs":0,"cacheScope":"private"}})
            }
            "tools/list" => json!({"jsonrpc":"2.0","id":id,"result":{"tools":[]}}),
            "tools/call" => {
                calls += 1;
                let p = &request["params"];
                let name = p["name"].as_str().unwrap_or("");
                let op = name;
                let m = &p["_meta"];
                let secret = contract == "secret-provider.v1";
                let good = m[names::key("instance")].is_string()
                    && m[names::key("config")].is_object()
                    && m[names::key("revision")]
                        .as_str()
                        .is_some_and(|s| !s.is_empty())
                    && !name.contains('#')
                    && m[names::key("operation")] == names::operation(op)
                    && m["io.modelcontextprotocol/clientCapabilities"]["extensions"]
                        [names::extension()]["revision"]
                        == "v0"
                    && (mode != "baseline"
                        || m["io.modelcontextprotocol/protocolVersion"] == "2026-07-28")
                    && (!secret
                        || ["caller", "invocation", "conduit", "filters"]
                            .iter()
                            .all(|k| m.get(names::key(k)).is_some()))
                    && (secret
                        || ["conduit", "filters", "idempotencyKey"]
                            .iter()
                            .all(|k| m.get(names::key(k)).is_none()));
                if !good {
                    json!({"jsonrpc":"2.0","id":id,"error":{"code":-32602,"message":"fixture metadata check failed"}})
                } else {
                    let output = match op {
                        "run.v1" => json!({"url":"https://example.invalid/mcp"}),
                        "stop.v1" => json!({}),
                        "inspect-run.v1" => json!({"state":"running"}),
                        "list-machines.v1" => json!({"machines":[]}),
                        "place.v1" => json!({"hostname":p["arguments"]["hostname"]}),
                        "fetch-package.v1" => json!({"archive":"dGVzdA=="}),
                        "get-secret.v1" => json!({"value":"synthetic-store-value"}),
                        _ => json!({}),
                    };
                    let mut r = json!({"jsonrpc":"2.0","id":id,"result":{"content":[],"structuredContent":output,"isError":false,"_meta":{names::key("outcome"):"succeeded"}}});
                    let per_call = p["arguments"]["fixture_mode"].as_str().unwrap_or(&mode);
                    match per_call {
                        "input-required" if calls == 1 => {
                            r["result"] =
                                json!({"resultType":"input_required","requestState":"again"});
                        }
                        "call-count" => {
                            r["result"]["structuredContent"]["calls"] = json!(calls);
                        }
                        mode if mode.starts_with("revision-") || mode == "repair-then-revision" => {
                            let revision = &m["io.modelcontextprotocol/protocolVersion"];
                            if calls == 1 && mode == "repair-then-revision"
                                || calls == 2 && mode == "revision-then-repair"
                            {
                                r = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32020,"message":"refresh"}});
                            } else if calls == 1
                                || ![
                                    "revision-success",
                                    "revision-discovery",
                                    "revision-modern",
                                    "revision-malformed-content",
                                ]
                                .contains(&mode)
                            {
                                r = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32022,"message":"revision unsupported"}});
                                if mode != "revision-discovery" {
                                    r["error"]["data"] = json!({"supported": if mode == "revision-no-shared" {
                                        json!(["2099-01-01", "2025-11-25"])
                                    } else if mode == "revision-modern" {
                                        json!(["2025-06-18", "2026-07-28", "2025-11-25"])
                                    } else if mode == "revision-mixed-supported" {
                                        json!([false, "2025-06-18"])
                                    } else {
                                        json!(["2025-06-18", "2024-11-05", "2025-11-25"])
                                    }});
                                }
                            } else {
                                let expected = if mode == "revision-discovery" {
                                    "2026-07-28"
                                } else if mode == "revision-modern" {
                                    "2025-11-25"
                                } else {
                                    "2025-06-18"
                                };
                                if revision != expected {
                                    r = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32602,"message":"retry revision differs"}});
                                } else if mode == "revision-malformed-content" {
                                    r["result"]["content"] = json!(42);
                                }
                            }
                        }
                        "malformed-content" => {
                            r["result"]["content"] = json!(42);
                        }
                        "repair" if !repaired => {
                            repaired = true;
                            r = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32020,"message":"parameter headers need refresh"}});
                        }
                        "repair-always" => {
                            r = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32020,"message":"parameter headers need refresh"}});
                        }
                        "failed" | "unknown" => {
                            r["result"]["_meta"][names::key("outcome")] = json!(per_call);
                            r["result"]["isError"] = json!(true);
                            r["result"]
                                .as_object_mut()
                                .unwrap()
                                .remove("structuredContent");
                        }
                        "refused" | "unavailable" => {
                            r["result"]["_meta"] = json!({names::key("reason"):per_call,names::key("restriction"):m[names::key("filters")][0]});
                            r["result"]["isError"] = json!(true);
                        }
                        "both" => {
                            r["result"]["_meta"][names::key("reason")] = json!("refused");
                        }
                        "disagree" => {
                            r["result"]["isError"] = json!(true);
                        }
                        "neither" => {
                            r["result"]["_meta"] = json!({});
                        }
                        "invalid-outcome" => {
                            r["result"]["_meta"][names::key("outcome")] = json!("done");
                        }
                        "bad-schema" => {
                            r["result"]["structuredContent"] = json!({"state":42});
                        }
                        "malformed" => {
                            r["result"] = json!({"content":42});
                        }
                        "protocol-error" => {
                            r = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32021,"message":"required capability absent","data":{"requiredCapabilities":{"extensions":{names::extension():{"revision":"v0"}}}}}});
                        }
                        "unqualified-error" => {
                            r = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32021,"message":"unqualified"}});
                        }
                        "echo-marker" => {
                            r["result"]["content"] =
                                json!([{"type":"text","text":p["arguments"]["marker"]}]);
                        }
                        "log-marker" => {
                            eprintln!("{}", p["arguments"]["marker"].as_str().unwrap_or(""));
                        }
                        "state" => {
                            let dir = std::env::var(names::STATE_DIR).unwrap();
                            let path = std::path::Path::new(&dir).join("fixture-counter");
                            let n = std::fs::read_to_string(&path)
                                .ok()
                                .and_then(|v| v.parse::<u64>().ok())
                                .unwrap_or(0)
                                + 1;
                            std::fs::write(path, n.to_string()).unwrap();
                            r["result"]["structuredContent"]["count"] = json!(n);
                        }
                        "echo" => {
                            r["result"]["structuredContent"]["echo"] =
                                p["arguments"]["value"].clone();
                        }
                        "environment" => {
                            r["result"]["structuredContent"]["binding"] =
                                json!(std::env::var("FIXTURE_BINDING").ok());
                            r["result"]["structuredContent"]["allowed"] =
                                json!(std::env::var("FIXTURE_ALLOWLIST").ok());
                            r["result"]["structuredContent"]["ambient_absent"] =
                                json!(std::env::var("AMBIENT_SENTINEL").is_err());
                        }
                        _ => (),
                    }
                    r
                }
            }
            _ => {
                json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"unknown method"}})
            }
        };
        // Keep JSON serialization local to the captured process.
        response
            .as_object_mut()
            .unwrap()
            .insert("id".into(), id.clone());
        writeln!(out, "{response}").unwrap();
        out.flush().unwrap();
    }
}
