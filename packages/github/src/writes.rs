//! The writer's schemas, fixed upstream table, and effect-aware outcomes.
use super::*;

pub(crate) const TOOLS: [(&str, &[&str]); 4] = [
    (
        "list_pull_requests",
        &["owner", "repo", "state", "head", "base"],
    ),
    (
        "create_pull_request",
        &["owner", "repo", "title", "head", "base", "body"],
    ),
    (
        "pull_request_read",
        &["method", "owner", "repo", "pullNumber"],
    ),
    (
        "merge_pull_request",
        &[
            "owner",
            "repo",
            "pullNumber",
            "merge_method",
            "commit_title",
            "commit_message",
            "expectedHeadSha",
        ],
    ),
];
const CREATE: &str = "pull-request.create.v1";
const MERGE: &str = "pull-request.merge.v1";

fn branch_schema() -> Value {
    json!({"type":"string","minLength":1,"maxLength":255,
        "pattern":"^[A-Za-z0-9._/-]+$",
        "not":{"anyOf":[{"pattern":"^[-/.]"},{"pattern":"[/.]$"},
            {"pattern":"[.]lock$"},{"pattern":"[.][.]|//|/[.]|[.]lock/"}]}})
}

fn title_schema(max: usize) -> Value {
    json!({"type":"string","minLength":1,"maxLength":max,
        "not":{"pattern":"[\r\n\u{000b}\u{000c}\u{0085}\u{2028}\u{2029}]"}})
}

fn schemas(merge: bool) -> (Value, Value) {
    let number = json!({"type":"integer","minimum":1});
    let sha = json!({"type":"string","pattern":"^[0-9a-f]{40}$"});
    let text = json!({"type":"string","maxLength":65536});
    if merge {
        (
            json!({"type":"object","additionalProperties":false,
            "required":["number","expectedHead","title","message"],
            "properties":{"number":number,"expectedHead":sha,"title":title_schema(1024),"message":text}}),
            json!({"type":"object","additionalProperties":false,"required":["sha"],
            "properties":{"sha":sha}}),
        )
    } else {
        (
            json!({"type":"object","additionalProperties":false,"required":["head","title","body"],
            "properties":{"head":branch_schema(),"title":title_schema(256),"body":text}}),
            json!({"type":"object","additionalProperties":false,"required":["number","url"],
            "properties":{"number":number,"url":{"type":"string","pattern":"^https://github[.]com/"}}}),
        )
    }
}

pub(crate) fn tools() -> Vec<Tool> {
    [CREATE, MERGE]
        .into_iter()
        .map(|name| {
            let merge = name == MERGE;
            let (input, output) = schemas(merge);
            serde_json::from_value(
                json!({"name":name,"inputSchema":input,"outputSchema":output,
            "description":"Acts on the Resource's repository and base alone.",
            "annotations":{"idempotentHint":merge,"destructiveHint":false,"openWorldHint":true}}),
            )
            .unwrap()
        })
        .collect()
}

fn branch(value: &str) -> bool {
    (1..=255).contains(&value.len())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._/-".contains(&b))
        && !value.starts_with(['-', '/', '.'])
        && !value.ends_with(['/', '.'])
        && !value.ends_with(".lock")
        && !["..", "//", "/.", ".lock/"]
            .iter()
            .any(|part| value.contains(part))
}

fn sha(value: &Value) -> bool {
    value.as_str().is_some_and(|s| {
        s.len() == 40
            && s.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}

fn number(value: &Value) -> bool {
    value.as_f64().is_some_and(|n| n >= 1.0 && n.fract() == 0.0)
}

fn text(value: &Value, min: usize, max: usize, title: bool) -> bool {
    value.as_str().is_some_and(|s| {
        (min..=max).contains(&s.chars().count())
            && (!title
                || !s.contains([
                    '\r', '\n', '\u{000b}', '\u{000c}', '\u{0085}', '\u{2028}', '\u{2029}',
                ]))
    })
}

struct Input<'a> {
    merge: bool,
    args: &'a Map<String, Value>,
    owner: &'a str,
    repo: &'a str,
    base: &'a str,
}

impl<'a> Input<'a> {
    fn validate(
        request: &'a CallToolRequestParams,
        meta: &'a Value,
        token: &str,
    ) -> Result<Self, String> {
        let merge = match request.name.as_ref() {
            CREATE => false,
            MERGE => true,
            _ => return Err("unknown pull-request writer operation".into()),
        };
        if meta[format!("{PREFIX}operation")] != format!("{ADDRESS}#{}", request.name) {
            return Err("unknown pull-request writer operation metadata".into());
        }
        for key in ["instance", "invocation", "caller", "revision", "conduit"] {
            if meta[format!("{PREFIX}{key}")]
                .as_str()
                .is_none_or(str::is_empty)
            {
                return Err("request metadata requires non-empty instance, operation, invocation, caller, revision and conduit".into());
            }
        }
        let config = &meta[format!("{PREFIX}config")];
        let args_value = json!(request.arguments);
        if contains_token(config, token) || contains_token(&args_value, token) {
            return Err("arguments or configuration contain the active credential".into());
        }
        let invalid_config = "configuration requires repository, a valid base branch and a resolved secret Resource reference";
        let mut read_config = config.clone();
        let fields = read_config.as_object_mut().ok_or(invalid_config)?;
        if fields.len() != 3 {
            return Err(invalid_config.into());
        }
        fields.remove("base");
        repository(&read_config).ok_or(invalid_config)?;
        let base = config["base"]
            .as_str()
            .filter(|base| branch(base))
            .ok_or(invalid_config)?;
        let (owner, repo) = config["repository"]
            .as_str()
            .unwrap()
            .split_once('/')
            .unwrap();
        let args = request.arguments.as_ref().ok_or("arguments are required")?;
        let fields: &[&str] = if merge {
            &["number", "expectedHead", "title", "message"]
        } else {
            &["head", "title", "body"]
        };
        if args.len() != fields.len()
            || fields.iter().any(|key| !args.contains_key(*key))
            || !text(&args["title"], 1, if merge { 1024 } else { 256 }, true)
            || !text(
                &args[if merge { "message" } else { "body" }],
                0,
                65536,
                false,
            )
            || if merge {
                !number(&args["number"]) || !sha(&args["expectedHead"])
            } else {
                args["head"].as_str().is_none_or(|head| !branch(head))
            }
        {
            return Err("arguments do not match the operation's closed schema".into());
        }
        if !merge && args["head"] == base {
            return Err("head equals the configured base".into());
        }
        Ok(Self {
            merge,
            args,
            owner,
            repo,
            base,
        })
    }

    fn arguments(&self, extra: Value) -> Map<String, Value> {
        let mut args = extra.as_object().unwrap().clone();
        args.insert("owner".into(), json!(self.owner));
        args.insert("repo".into(), json!(self.repo));
        args
    }

    fn read_arguments(&self) -> Map<String, Value> {
        self.arguments(if self.merge {
            json!({"method":"get","pullNumber":self.args["number"]})
        } else {
            json!({"state":"open","head":format!("{}:{}",self.owner,self.args["head"].as_str().unwrap()),"base":self.base})
        })
    }

    fn write_arguments(&self) -> Map<String, Value> {
        self.arguments(if self.merge {
            json!({"pullNumber":self.args["number"],"merge_method":"squash",
                "commit_title":self.args["title"],"commit_message":self.args["message"],
                "expectedHeadSha":self.args["expectedHead"]})
        } else {
            json!({"title":self.args["title"],"head":self.args["head"],
                "base":self.base,"body":self.args["body"]})
        })
    }
}

// A readback must have all four typed fields before it can establish no merge.
fn readable(value: &Value) -> bool {
    value["state"].is_string()
        && value["merged"].is_boolean()
        && value["head"]["sha"].is_string()
        && value["base"]["ref"].is_string()
}

fn created(value: &Value, input: &Input<'_>) -> Option<Value> {
    let url = value["url"].as_str()?;
    let path = url.strip_prefix("https://github.com/")?;
    let parts: Vec<_> = path.split('/').collect();
    if parts.len() != 4
        || !parts[0].eq_ignore_ascii_case(input.owner)
        || !parts[1].eq_ignore_ascii_case(input.repo)
        || parts[2] != "pull"
        || parts[3].is_empty()
        || !parts[3].bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let number = parts[3].parse::<u64>().ok().filter(|n| *n >= 1)?;
    Some(json!({"number":number,"url":url}))
}

fn open_number(value: &Value) -> String {
    value["number"].as_u64().filter(|n| *n > 0).map_or_else(
        || "an open pull request".into(),
        |n| format!("pull request #{n}"),
    )
}

impl Front {
    async fn write_effect(
        &self,
        input: &Input<'_>,
        token: &str,
        sent: &AtomicBool,
    ) -> Result<Value, (bool, String)> {
        let read_tool = if input.merge {
            "pull_request_read"
        } else {
            "list_pull_requests"
        };
        let read = self
            .dispatch_raw(token, read_tool, input.read_arguments(), None)
            .await
            .and_then(|r| data(r, token));
        let read = read.map_err(|cause| {
            let context = if input.merge {
                "could not read the pull request".into()
            } else {
                format!(
                    "could not list open pull requests from {} into {}",
                    input.args["head"].as_str().unwrap(),
                    input.base
                )
            };
            (false, format!("{context}; {cause}"))
        })?;
        if input.merge {
            let value = Some(read)
                .filter(readable)
                .ok_or_else(|| (false, "could not read the pull request".into()))?;
            let cause = if value["merged"] == true {
                Some("is already merged".into())
            } else if value["state"] != "open" {
                Some("is closed".into())
            } else if value["base"]["ref"] != input.base {
                Some(format!(
                    "targets {}, not {}",
                    value["base"]["ref"].as_str().unwrap(),
                    input.base
                ))
            } else if value["head"]["sha"] != input.args["expectedHead"] {
                return Err((
                    false,
                    format!(
                        "head is {}, not the expected {}; gate the new head before merging again",
                        value["head"]["sha"].as_str().unwrap(),
                        input.args["expectedHead"].as_str().unwrap()
                    ),
                ));
            } else {
                None
            };
            if let Some(cause) = cause {
                return Err((false, cause));
            }
        } else {
            let value = Some(read).filter(Value::is_array).ok_or_else(|| {
                (
                    false,
                    format!(
                        "could not list open pull requests from {} into {}",
                        input.args["head"].as_str().unwrap(),
                        input.base
                    ),
                )
            })?;
            if let Some(pr) = value.as_array().unwrap().first() {
                return Err((
                    false,
                    format!(
                        "{} from {} into {} is already open",
                        open_number(pr),
                        input.args["head"].as_str().unwrap(),
                        input.base
                    ),
                ));
            }
        }
        let tool = if input.merge {
            "merge_pull_request"
        } else {
            "create_pull_request"
        };
        let result = self
            .dispatch_raw(token, tool, input.write_arguments(), Some(sent))
            .await
            .map_err(|cause| (sent.load(Ordering::Relaxed), cause))?;
        if result.is_error == Some(true) {
            let cause = data(result, token).unwrap_err();
            let read = self
                .dispatch_raw(token, read_tool, input.read_arguments(), None)
                .await
                .and_then(|r| data(r, token));
            if input.merge {
                if let Ok(value) = read
                    && readable(&value)
                {
                    if value["merged"] == true {
                        return Err((
                            true,
                            format!(
                                "{cause}; pull request #{} is now merged",
                                input.args["number"]
                            ),
                        ));
                    }
                    let moved = if value["head"]["sha"] != input.args["expectedHead"] {
                        format!(
                            "; the head is now {}; gate the new head before merging again",
                            value["head"]["sha"].as_str().unwrap()
                        )
                    } else {
                        String::new()
                    };
                    return Err((false, format!("{cause}{moved}")));
                }
            } else if let Ok(Value::Array(pulls)) = read {
                if pulls.is_empty() {
                    return Err((false, cause));
                }
                return Err((
                    true,
                    format!("{cause}; {} is now open", open_number(&pulls[0])),
                ));
            }
            return Err((
                true,
                format!("{cause}; readback could not establish that the write had no effect"),
            ));
        }
        let value = data(result, token).map_err(|cause| (true, cause))?;
        if input.merge {
            if value["merged"] == true && sha(&value["sha"]) {
                return Ok(json!({"sha":value["sha"]}));
            }
        } else if let Some(created) = created(&value, input) {
            return Ok(created);
        }
        Err((
            true,
            "server answer does not confirm the requested effect".into(),
        ))
    }

    pub(super) async fn write_call(
        &self,
        request: CallToolRequestParams,
        meta: &Value,
    ) -> CallToolResponse {
        let sent = AtomicBool::new(false);
        let result = async {
            let token = self.token.as_deref().ok_or_else(|| (false,"GITHUB_PERSONAL_ACCESS_TOKEN is absent or empty".into()))?;
            let input = Input::validate(&request,meta,token).map_err(|cause| (false,cause))?;
            let mut stopping = self.stopping.subscribe();
            if *stopping.borrow() { return Err((false,"pull-request writer is stopping".into())); }
            tokio::select! {
                biased;
                _ = stopping.changed() => Err((sent.load(Ordering::Relaxed),"pull-request writer is stopping".into())),
                result = self.write_effect(&input,token,&sent) => result,
            }
        }.await;
        let response = match result {
            Ok(outputs) => {
                json!({"content":[],"structuredContent":outputs,"isError":false,"_meta":{OUTCOME:"succeeded"}})
            }
            Err((unknown, cause)) => self.write_failure(&request, unknown, &cause),
        };
        serde_json::from_value::<CallToolResult>(response)
            .unwrap()
            .into()
    }

    pub(super) fn write_failure(
        &self,
        request: &CallToolRequestParams,
        unknown: bool,
        cause: &str,
    ) -> Value {
        let merge = request.name == MERGE;
        let args = json!(request.arguments);
        let target = if merge {
            format!("pull request #{}", args["number"])
        } else {
            format!("head {}", args["head"].as_str().unwrap_or("(unspecified)"))
        };
        let end = if unknown {
            if merge {
                "merge is not known; read the pull request through pull-request.read.v1"
            } else {
                "creation is not known; read the pull request through pull-request.list.v1"
            }
        } else if merge {
            "read the pull request through pull-request.read.v1 before trying again; nothing merged"
        } else {
            "read open pull requests through pull-request.list.v1 before trying again; nothing created"
        };
        let mut message = format!("{target}: {cause}; {end}");
        if let Some(token) = self.token.as_deref() {
            message = redacted(&message, token);
        }
        json!({"content":[{"type":"text","text":message}],"isError":true,
            "_meta":{OUTCOME:if unknown {"unknown"} else {"failed"}}})
    }
}
