//! conduit sign-in: one sign-in.v1 through serve, its URL action shown, consented to and retried (order 051b-2).
use crate::Result;
use rmcp::{
    ClientServiceExt, RoleClient, ServiceError,
    model::{
        CallToolRequestParams, CallToolResponse, ClientConfig, ElicitationCapability,
        ProtocolVersion, UrlElicitationCapability,
    },
    service::{ClientLifecycleMode, RunningService},
    transport::{
        StreamableHttpClientTransport, streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

pub trait Person {
    fn ask(&mut self, prompt: &str) -> Result<Option<String>>;
    fn tell(&mut self, line: &str);
    fn open(&mut self, url: &str) -> Result<()>;
}

pub struct Terminal;

impl Person for Terminal {
    fn ask(&mut self, prompt: &str) -> Result<Option<String>> {
        eprint!("{prompt}");
        io::stderr()
            .flush()
            .map_err(|error| format!("could not flush the terminal: {error}"))?;
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .map_err(|error| format!("could not read the terminal: {error}"))?;
        if bytes == 0 {
            return Ok(None);
        }
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        Ok(Some(line))
    }

    fn tell(&mut self, line: &str) {
        eprintln!("{line}");
    }

    fn open(&mut self, url: &str) -> Result<()> {
        let mut command = browser_command(url);
        let status = command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|error| error.to_string())?;
        if status.success() {
            Ok(())
        } else {
            let program = command.get_program().to_string_lossy();
            Err(format!("{program} exited with {status}"))
        }
    }
}

fn browser_command(url: &str) -> Command {
    #[cfg(target_os = "macos")]
    let mut command = Command::new("open");
    #[cfg(target_os = "windows")]
    let mut command = Command::new("rundll32");
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let mut command = Command::new("xdg-open");

    #[cfg(target_os = "windows")]
    command.arg("url.dll,FileProtocolHandler");
    command.arg(url);
    command
}

pub(crate) fn shown(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_control() {
            escaped.extend(c.escape_unicode());
        } else {
            escaped.push(c);
        }
    }
    escaped
}

pub(crate) struct UrlAction {
    pub key: String,
    pub message: String,
    pub url: String,
    pub state: String,
}

pub(crate) fn url_action(result: &Value) -> Result<UrlAction> {
    let parse = || {
        let requests = result["inputRequests"].as_object()?;
        if requests.len() != 1 {
            return None;
        }
        let (key, request) = requests.iter().next()?;
        let params = &request["params"];
        if request["method"] != "elicitation/create" || params["mode"] != "url" {
            return None;
        }
        let message = params["message"].as_str()?;
        let url = params["url"].as_str()?;
        if url::Url::parse(url).ok()?.scheme() != "https" {
            return None;
        }
        let state = result["requestState"]
            .as_str()
            .filter(|state| !state.is_empty())?;
        Some(UrlAction {
            key: key.clone(),
            message: message.into(),
            url: url.into(),
            state: state.into(),
        })
    };
    parse().ok_or_else(|| "the engine's input request is not one URL elicitation with an https URL and a requestState; nothing was opened and nothing more was sent, so this sign-in waits until its URL action's wait ends (DL18)".into())
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Consent {
    Open,
    Decline,
    Cancel,
    Again,
}

pub(crate) fn consent(line: Option<&str>) -> Consent {
    match line.map(str::trim) {
        Some(line) if line.eq_ignore_ascii_case("y") || line.eq_ignore_ascii_case("yes") => {
            Consent::Open
        }
        Some(line) if line.eq_ignore_ascii_case("n") || line.eq_ignore_ascii_case("no") => {
            Consent::Decline
        }
        Some(_) => Consent::Again,
        None => Consent::Cancel,
    }
}

pub(crate) fn waited(line: Option<&str>) -> &'static str {
    match consent(line) {
        Consent::Decline | Consent::Cancel => "cancel",
        Consent::Open | Consent::Again => "accept",
    }
}

fn answer_url(
    action: &UrlAction,
    opened: &mut Option<String>,
    person: &mut dyn Person,
) -> Result<&'static str> {
    if opened.as_deref() != Some(action.url.as_str()) {
        loop {
            let line = person
                .ask("Open this URL in your browser? Type y to open it, or n to decline: ")?;
            match consent(line.as_deref()) {
                Consent::Again => continue,
                Consent::Decline => return Ok("decline"),
                Consent::Cancel => return Ok("cancel"),
                Consent::Open => break,
            }
        }
        if let Err(error) = person.open(&action.url) {
            person.tell(&format!(
                "conduit sign-in: the browser could not be opened ({error}); open the URL above yourself."
            ));
        }
        *opened = Some(action.url.clone());
    }
    let line = person.ask("Press Enter once you have signed in, or type n to stop: ")?;
    Ok(waited(line.as_deref()))
}

pub(crate) fn subject(args: &[String]) -> Result<Value> {
    let [argument] = args else {
        return Err(format!(
            "sign-in: expected 1 positional argument(s), received {}; run conduit sign-in --help for usage and an example",
            args.len()
        ));
    };
    let object = crate::author::named(argument)?;
    let kind = object["kind"].as_str().unwrap_or("?");
    if !matches!(kind, "Agent" | "Resource") {
        return Err(format!(
            "sign-in: {argument} is a {kind}, and sign-in.v1 signs in only an Agent or a Resource; nothing was sent"
        ));
    }
    Ok(object)
}

async fn connect(
    directory: &Path,
    prefix: &str,
) -> Result<RunningService<RoleClient, ClientConfig>> {
    let config = crate::Config::for_store(directory)?;
    let bytes = crate::store::private_read(&directory.join("state.json"))?;
    let mut decoder = serde_json::Deserializer::from_slice(&bytes);
    decoder.disable_recursion_limit();
    let state = Value::deserialize(&mut decoder).map_err(|_| "invalid store")?;
    let id = state["state"]["objects"]
        .as_array()
        .ok_or("missing objects")?
        .iter()
        .find(|o| o.get("spec").is_none() && o["kind"] == "Interface" && o["name"] == "root")
        .and_then(|o| o["id"].as_str())
        .ok_or("tools for root: instance does not exist; submit its instance document first (use conduit inspect to see registered objects)")?;
    let token = String::from_utf8(
        crate::store::private_read(&directory.join("client-tokens").join(id))
            .map_err(|_| "tools for root: instance exists but has no presented tools yet (client identity is unavailable); set it up with conduit call through its management Conduit, then open its access Conduits")?,
    )
    .map_err(|_| "invalid client token file")?;

    let mut handler = ClientConfig::default();
    let url_elicitation = ElicitationCapability::new().with_url(UrlElicitationCapability::new());
    handler.capabilities.elicitation = Some(url_elicitation);
    handler.capabilities.extensions = Some(BTreeMap::from([(
        format!("{prefix}implementation-interface"),
        serde_json::Map::from_iter([("revision".into(), json!("v0"))]),
    )]));

    let failed = |error: &str| {
        format!(
            "conduit sign-in could not open a session with the engine at {}: {error}; run conduit serve --store {} and check its diagnostics",
            config.engine_url,
            directory.display()
        )
    };
    let transport = StreamableHttpClientTransport::with_client(
        reqwest::Client::builder()
            .retry(reqwest::retry::never())
            .build()
            .map_err(|error| failed(&error.to_string()))?,
        StreamableHttpClientTransportConfig::with_uri(config.engine_url.clone())
            .auth_header(token)
            .reinit_on_expired_session(false),
    );
    match tokio::time::timeout(
        Duration::from_secs(10),
        handler.serve_with_lifecycle(
            transport,
            ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            },
        ),
    )
    .await
    {
        Ok(result) => result.map_err(|error| failed(&error.to_string())),
        Err(_) => Err(failed("timed out")),
    }
}

async fn rounds(
    service: &RunningService<RoleClient, ClientConfig>,
    tool: &str,
    arguments: Value,
    meta: Option<Value>,
    label: &str,
    person: &mut dyn Person,
) -> Result<Value> {
    let mut opened: Option<String> = None;
    let mut first = json!({"name": tool, "arguments": arguments});
    if let Some(meta) = meta {
        first["_meta"] = meta;
    }
    let mut request: CallToolRequestParams =
        serde_json::from_value(first).map_err(|error| error.to_string())?;

    loop {
        let retry = request.request_state.is_some();
        let result = match service.call_tool_once(request).await {
            Err(ServiceError::McpError(error)) => {
                let data = serde_json::to_value(&error).map_err(|error| error.to_string())?;
                println!("{}", json!({"error": data}));
                if error.message.starts_with("requestState expired")
                    || error
                        .message
                        .starts_with("requestState is not one this engine issued")
                {
                    return Err("the engine no longer holds this sign-in's state: its URL action's wait ended (DL18) or the engine restarted, so nothing more was sent; run conduit sign-in again".into());
                }
                return Err(format!(
                    "sign-in.v1 through {tool} was answered with error {}: {}; nothing more was sent",
                    error.code.0,
                    shown(&error.message)
                ));
            }
            Err(error) if retry => {
                return Err(format!(
                    "sign-in.v1 through {tool} failed on a retry: {error}; the retry was not sent again, since the engine may have taken it, so this sign-in may still wait in the engine until its URL action's wait ends (DL18): run conduit pending to see its sign-in action, and conduit sign-in again once that wait has ended"
                ));
            }
            Err(error) => {
                return Err(format!(
                    "sign-in.v1 through {tool} failed: {error}; nothing more was sent"
                ));
            }
            Ok(CallToolResponse::Complete(result)) => {
                return serde_json::to_value(result).map_err(|error| error.to_string());
            }
            Ok(CallToolResponse::InputRequired(result)) => {
                serde_json::to_value(result).map_err(|error| error.to_string())?
            }
            Ok(_) => {
                return Err(format!(
                    "sign-in.v1 through {tool} was answered with a task, which conduit sign-in does not read; nothing more was sent"
                ));
            }
        };

        let action = url_action(&result)?;
        let already_opened = opened.as_deref() == Some(action.url.as_str());
        if already_opened {
            person.tell(&format!(
                "conduit sign-in: the engine has not yet found the expected account signed in on {label}; the URL below is the one already opened."
            ));
        } else {
            person.tell(&format!(
                "conduit sign-in: the engine asks you to open a URL to sign in {label}."
            ));
        }
        person.tell(&format!("Message: {}", shown(&action.message)));
        person.tell(&format!("URL: {}", shown(&action.url)));
        if let Some((_, login)) = result["_meta"]
            .as_object()
            .and_then(|meta| meta.iter().find(|(key, _)| key.ends_with("/signInLogin")))
        {
            if let Some(name) = login["name"].as_str() {
                person.tell(&format!("Login: {}", shown(name)));
            }
            if login["acceptsReturn"] == true {
                person.tell(&format!("conduit sign-in: the engine reports that this login takes a return carried back through the Interface (acceptsReturn), but this command carries none (order 051b-3, with the keyed release): where the browser is not on the machine where {label} runs, the login cannot complete through this command."));
            }
        }

        let answer = answer_url(&action, &mut opened, person)?;
        request = serde_json::from_value(json!({
            "name": tool, "arguments": arguments, "requestState": action.state,
            "inputResponses": {action.key: {"action": answer}},
        }))
        .map_err(|error| error.to_string())?;
    }
}

fn label(object: &Value) -> String {
    let kind = object["kind"].as_str().unwrap_or("?");
    let name = object["name"].as_str().unwrap_or("?");
    format!("{kind}/{name}")
}

pub async fn sign_in(
    directory: &Path,
    object: &Value,
    login: Option<&str>,
    prefix: &str,
    person: &mut dyn Person,
) -> Result<Value> {
    let tool = crate::author::route(directory, object, "sign-in.v1")?;
    let meta = login.map(|login| json!({format!("{prefix}signInLogin"): {"name": login}}));
    let service = connect(directory, prefix).await?;
    let arguments = json!({"instance": {"kind": object["kind"], "name": object["name"]}});
    let response = rounds(&service, &tool, arguments, meta, &label(object), person).await;
    let shutdown = service
        .cancel()
        .await
        .map_err(|_| "client shutdown failed".to_string());
    response.and_then(|value| shutdown.map(|_| value))
}

pub async fn run(
    directory: &Path,
    args: &[String],
    login: Option<&str>,
    repository: &Path,
    person: &mut dyn Person,
) -> Result<()> {
    let object = subject(args)?;
    let names = crate::boundary::value(&repository.join("spec/names.yaml"))?;
    let prefix = names["metaPrefix"]
        .as_str()
        .ok_or("missing metadata prefix")?;
    let result = sign_in(directory, &object, login, prefix, person).await?;
    println!("{result}");
    crate::author::note_unanswered(&result);
    let label = label(&object);
    crate::author::ensure_success(&result, &format!("sign-in.v1 of {label} through serve"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_action_takes_one_https_url_elicitation() {
        let valid = json!({
            "resultType": "input_required",
            "inputRequests": {"url": {"method": "elicitation/create", "params": {
                "mode": "url", "message": "Sign in", "url": "https://example.org/login",
                "elicitationId": ""
            }}},
            "requestState": "s"
        });
        let action = url_action(&valid).unwrap();
        assert_eq!(action.key, "url");
        assert_eq!(action.message, "Sign in");
        assert_eq!(action.url, "https://example.org/login");
        assert_eq!(action.state, "s");

        let mut invalid = valid.clone();
        invalid["inputRequests"]["other"] = valid["inputRequests"]["url"].clone();
        assert!(url_action(&invalid).is_err());
        for (path, value) in [
            ("/inputRequests/url/params/mode", "form"),
            ("/inputRequests/url/params/url", "http://example.org/login"),
            ("/inputRequests/url/method", "sampling/createMessage"),
        ] {
            let mut invalid = valid.clone();
            *invalid.pointer_mut(path).unwrap() = json!(value);
            assert!(url_action(&invalid).is_err(), "{path} = {value}");
        }
        let mut invalid = valid;
        invalid.as_object_mut().unwrap().remove("requestState");
        assert!(url_action(&invalid).is_err());
    }

    #[test]
    fn answers_are_read_strictly() {
        assert_eq!(consent(Some("y")), Consent::Open);
        assert_eq!(consent(Some(" YES ")), Consent::Open);
        assert_eq!(consent(Some("n")), Consent::Decline);
        assert_eq!(consent(Some("No")), Consent::Decline);
        assert_eq!(consent(None), Consent::Cancel);
        assert_eq!(consent(Some("")), Consent::Again);
        assert_eq!(consent(Some("maybe")), Consent::Again);

        assert_eq!(waited(Some("")), "accept");
        assert_eq!(waited(Some("done")), "accept");
        assert_eq!(waited(None), "cancel");
        assert_eq!(waited(Some("n")), "cancel");
        assert_eq!(waited(Some("NO")), "cancel");
    }

    #[test]
    fn shown_escapes_control_characters() {
        assert_eq!(shown("a\u{1b}[31mb\nc"), "a\\u{1b}[31mb\\u{a}c");
        let url = "https://example.org/a?b=c";
        assert_eq!(shown(url), url);
    }

    #[test]
    fn subject_takes_one_agent_or_resource() {
        assert_eq!(
            subject(&["Agent/a".into()]).unwrap(),
            json!({"kind": "Agent", "name": "a"})
        );
        assert!(
            subject(&["Fleet/f".into()])
                .unwrap_err()
                .contains("signs in only an Agent or a Resource")
        );
        assert!(
            subject(&[])
                .unwrap_err()
                .contains("expected 1 positional argument(s), received 0")
        );
    }

    #[test]
    fn browser_command_passes_the_url_as_one_argument() {
        let url = "https://example.org/a?b=c&d=e";
        let command = browser_command(url);
        let args: Vec<_> = command.get_args().collect();
        #[cfg(target_os = "macos")]
        {
            assert_eq!(command.get_program(), "open");
            assert_eq!(args, [url]);
        }
        #[cfg(target_os = "windows")]
        {
            assert_eq!(command.get_program(), "rundll32");
            assert_eq!(args, ["url.dll,FileProtocolHandler", url]);
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            assert_eq!(command.get_program(), "xdg-open");
            assert_eq!(args, [url]);
        }
    }
}
