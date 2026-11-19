//! PND-005 lapse signals and holds, PND-010 checks, and PND-011 sign-out conditions.
use crate::{
    Allocation, Context, InputRequired, Result, State, Transition,
    access::{self, AccessRequest},
    decision,
    explain::{self, Rejection},
    inventory, pending, requests,
};
use chrono::{DateTime, SecondsFormat, TimeDelta};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use url::Url;

pub(crate) enum ExpectedAccount<'a> {
    NoSignIn,
    NotString { pointer: String },
    Set { pointer: String, account: &'a str },
}

pub(crate) fn request_is_held(state: &State, target: &Value, contract: &Value) -> bool {
    target.get("spec").is_none()
        && match target["kind"].as_str() {
            Some("Resource") => true,
            Some("Agent") => contract["spec"]["startsModelWork"] == true,
            _ => false,
        }
        && target["id"]
            .as_str()
            .is_some_and(|id| state.sign_in_holds.contains_key(id))
}

fn undone_condition(object: &Value) -> Option<usize> {
    object["status"]["conditions"]
        .as_array()?
        .iter()
        .position(|condition| condition["type"] == "SignInUndone")
}

pub(crate) fn request_is_held_for_sign_in_undone(state: &State, target: &Value) -> bool {
    access::object_by_id(state, target).is_some_and(|object| {
        matches!(object["kind"].as_str(), Some("Resource" | "Agent"))
            && object.get("spec").is_none()
            && undone_condition(object)
                .is_some_and(|i| object["status"]["conditions"][i]["status"] == "False")
    })
}

pub(crate) fn set_sign_in_undone_false(
    object: &mut Value,
    outcome: &str,
    hook: &Value,
    time: &str,
) {
    let condition = json!({
        "type":"SignInUndone",
        "status":"False",
        "reason":if outcome == "failed" {"SignOutFailed"} else {"SignOutUnknown"},
        "message":format!(
            "The sign-out hook ended {outcome} after setup's account check found an account other than the expected one, so that account may still be signed in. Requests to the instance other than the parts of sign-in.v1, of a setup and of its retirement report unavailable under PND-011 until an account check finds the expected account or none."
        ),
        "lastTransitionTime":time,
        "source":format!("urn:uuid:{}", hook.as_str().unwrap())
    });
    write_undone_condition(object, undone_condition(object), condition, time);
}

pub(crate) fn clear_sign_in_undone(object: &mut Value, time: &str, within: &str) -> bool {
    let Some(i) = undone_condition(object) else {
        return false;
    };
    let old = &object["status"]["conditions"][i];
    if old["status"] != "False" {
        return false;
    }
    let condition = json!({
        "type":"SignInUndone","status":"True","reason":"AccountChecked",
        "message":format!("An account check within {within} found the expected account or none."),
        "lastTransitionTime":time,"source":old["source"]
    });
    write_undone_condition(object, Some(i), condition, time);
    true
}

fn write_undone_condition(object: &mut Value, index: Option<usize>, mut new: Value, time: &str) {
    if let Some(i) = index {
        let old = &object["status"]["conditions"][i];
        if old["status"] == new["status"] {
            new["lastTransitionTime"] = old["lastTransitionTime"].clone();
        } else {
            let past = json!({"condition":old,"endedAt":time});
            if !object["past_conditions"].is_array() {
                object["past_conditions"] = json!([]);
            }
            object["past_conditions"].as_array_mut().unwrap().push(past);
        }
        object["status"]["conditions"][i] = new;
    } else {
        if !object["status"]["conditions"].is_array() {
            object["status"]["conditions"] = json!([]);
        }
        object["status"]["conditions"]
            .as_array_mut()
            .unwrap()
            .push(new);
    }
}

pub(crate) fn expected_account<'a>(
    instance: &'a Value,
    context: &Context,
) -> Result<ExpectedAccount<'a>> {
    if !matches!(instance["kind"].as_str(), Some("Resource" | "Agent"))
        || instance.get("spec").is_some()
        || instance.get("held").is_none()
    {
        return Ok(ExpectedAccount::NoSignIn);
    }
    let ty = inventory::instance_type(instance, context)?;
    let Some(pointer) = ty["spec"]["signIn"]["account"].as_str() else {
        return Ok(ExpectedAccount::NoSignIn);
    };
    let pointer = pointer.to_owned();
    Ok(
        match instance["config"].pointer(&pointer).and_then(Value::as_str) {
            Some(account) => ExpectedAccount::Set { pointer, account },
            None => ExpectedAccount::NotString { pointer },
        },
    )
}

const ACCOUNT_NOT_STRING: &str = "The instance's expected account is not a string, so sign-in.v1 cannot check it and signs nothing in.";
const RETIRED_DURING_CHECK: &str = "The instance was retired while its account check was in progress, so sign-in.v1 signs nothing in and answers no sign-in action.";
const RETIRED_DURING_HOOK: &str = "The instance was retired while its sign-in hook was in progress, so sign-in.v1 relays nothing and signs nothing in.";
const ACCOUNT_HOOK_NOT_RESOLVED: &str = "No implementation of account-hook.v1 is resolved for the instance, so its account cannot be checked and sign-in.v1 signs nothing in.";

pub(crate) fn begin(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    let reference = &t.state.invocations[i]["arguments"]["instance"];
    let Some(instance) = access::object(&t.state, reference).cloned() else {
        return end_sign_in(
            t,
            a,
            c,
            i,
            "failed",
            "PND-010",
            "The request's instance names no object, so sign-in.v1 has no account to check and signs nothing in.",
        );
    };
    if !matches!(expected_account(&instance, c)?, ExpectedAccount::Set { .. }) {
        return end_sign_in(t, a, c, i, "failed", "PND-010", ACCOUNT_NOT_STRING);
    }
    if instance["state"] == "retired" {
        return end_sign_in(
            t,
            a,
            c,
            i,
            "failed",
            "PND-003",
            "The instance is retired, so sign-in.v1 signs nothing in and dispatches no part.",
        );
    }
    let invocation = &mut t.state.invocations[i];
    invocation["object"] = instance["id"].clone();
    invocation["sign_in"] = json!({"phase":"account-pending","parts":[]});
    if !requests::setup_sign_in_hook(
        t,
        a,
        c,
        i,
        &instance,
        "account-hook.v1",
        &["PND-010", "PND-011", "CTR-013", "ACC-018", "IMP-002"],
        None,
    )? {
        return end_sign_in(t, a, c, i, "failed", "PND-011", ACCOUNT_HOOK_NOT_RESOLVED);
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
pub(crate) enum FirstCheck {
    Expected(String),
    NoAccount,
    Other,
    NotChecked,
}

pub(crate) fn first_check(outcome: &str, outputs: &Value, expected: &str) -> FirstCheck {
    if outcome != "succeeded" {
        return FirstCheck::NotChecked;
    }
    match outputs.get("account") {
        Some(Value::Null) => FirstCheck::NoAccount,
        Some(Value::String(found)) if found.eq_ignore_ascii_case(expected) => {
            FirstCheck::Expected(found.clone())
        }
        Some(Value::String(_)) => FirstCheck::Other,
        _ => FirstCheck::NotChecked,
    }
}

/// IFC-049: the person's answer to the one URL input request, under its key `url` in the
/// retry's inputResponses; None for anything else.
pub(crate) fn url_action(responses: &Value) -> Option<&'static str> {
    let responses = responses.as_object()?;
    if responses.len() != 1 {
        return None;
    }
    let action = responses.get("url")?.get("action")?.as_str()?;
    ["accept", "decline", "cancel"]
        .into_iter()
        .find(|a| *a == action)
}

/// PND-011: while polling, no account, or the account that the check before
/// sign-in-hook.v1 found, compared ignoring ASCII case, counts as no account yet.
pub(crate) fn no_account_yet(found: &Value, first: &Value) -> bool {
    match (found.as_str(), first.as_str()) {
        (None, _) => found.is_null(),
        (Some(found), Some(first)) => found.eq_ignore_ascii_case(first),
        (Some(_), None) => false,
    }
}

/// PND-010 and PND-011: what a poll while sign-in.v1 waits after its URL action found.
#[derive(Debug, PartialEq)]
pub(crate) enum Poll {
    Expected(String),
    NoneYet,
    Other,
    NotKnown,
}

/// A poll that failed or ended unknown counts as no account yet (PND-010); one that states
/// a reason, or whose account cannot be read, is not known.
pub(crate) fn poll_check(outcome: &str, outputs: &Value, expected: &str, first: &Value) -> Poll {
    if matches!(outcome, "failed" | "unknown") {
        return Poll::NoneYet;
    }
    if outcome != "succeeded" {
        return Poll::NotKnown;
    }
    match outputs.get("account") {
        Some(Value::String(found)) if found.eq_ignore_ascii_case(expected) => {
            Poll::Expected(found.clone())
        }
        Some(found) if no_account_yet(found, first) => Poll::NoneYet,
        Some(Value::String(_)) => Poll::Other,
        _ => Poll::NotKnown,
    }
}

pub(crate) fn part_answered(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    part: &Value,
    outcome: &str,
) -> Result<()> {
    let invocation = &mut t.state.invocations[i];
    let name = match invocation["sign_in"]["phase"].as_str() {
        Some("sign-in-pending") => return hook_ended(t, a, c, i, &part["id"], outcome),
        Some("poll-pending") if part["id"] == invocation["sign_in"]["poll"] => "poll",
        Some("account-pending") => "account",
        _ => return Ok(()),
    };
    invocation["sign_in"]["parts"]
        .as_array_mut()
        .unwrap()
        .push(json!({"name":name,"outcome":outcome,"invocation":part["id"]}));
    let Some(object) = t
        .state
        .objects
        .iter()
        .position(|o| o["id"] == invocation["object"])
    else {
        return end_sign_in(t, a, c, i, "failed", "PND-003", RETIRED_DURING_CHECK);
    };
    let instance = t.state.objects[object].clone();
    let ExpectedAccount::Set { pointer, account } = expected_account(&instance, c)? else {
        return end_sign_in(t, a, c, i, "failed", "PND-010", ACCOUNT_NOT_STRING);
    };
    let check = first_check(outcome, &part["retained"]["outputs"], account);
    if matches!(check, FirstCheck::Expected(_) | FirstCheck::NoAccount)
        && clear_sign_in_undone(&mut t.state.objects[object], &a.evidence.time, "sign-in.v1")
    {
        t.decisions.push(decision(
            "clear-sign-in-undone",
            &["PND-011", "PND-010"],
            &[],
            json!({"instance":instance["id"]}),
            vec![],
        ));
    }
    if instance["state"] == "retired" {
        return end_sign_in(t, a, c, i, "failed", "PND-003", RETIRED_DURING_CHECK);
    }
    if name == "poll" {
        return poll_answered(t, a, c, i, part, outcome, account);
    }
    match check {
        FirstCheck::Expected(found) => return signed_in(t, a, c, i, &found),
        FirstCheck::NotChecked if !matches!(outcome, "failed" | "unknown") => {
            return end_sign_in(
                t,
                a,
                c,
                i,
                "failed",
                "PND-011",
                &format!(
                    "The account check before sign-in-hook.v1 ended {outcome}, so the account is not known and sign-in.v1 completes nothing; send sign-in.v1 again."
                ),
            );
        }
        _ => {}
    }
    let retired_by = c.qualified("retire.v1");
    let retiring = t.state.invocations.iter().any(|v| {
        v["operation"] == retired_by && v["state"] == "in-progress" && v["object"] == instance["id"]
    });
    if retiring {
        return end_sign_in(
            t,
            a,
            c,
            i,
            "failed",
            "PND-003",
            "A retirement of the instance is in progress, so sign-in.v1 starts no login and signs nothing in.",
        );
    }
    if matches!(check, FirstCheck::Other) {
        let invocation = t.state.invocations[i].clone();
        if let Some(action) = pending::raise_sign_in(
            &mut t.state,
            &instance,
            account,
            &pointer,
            pending::SignInRequest::SignIn {
                invocation: &invocation,
            },
            a,
        )? {
            t.state.invocations[i]["sign_in"]["raised"] = action["id"].clone();
            t.decisions.push(decision(
                "raise-sign-in-action",
                &["PND-001", "PND-005", "PND-010", "PND-012"],
                &["DL19"],
                json!({"action":action["id"],"instance":instance["id"]}),
                vec![],
            ));
        }
    }
    let login = t.state.invocations[i]["asked_login"]
        .as_str()
        .map(str::to_owned);
    t.state.invocations[i]["sign_in"]["first_account"] =
        json!(part["retained"]["outputs"]["account"].as_str());
    t.state.invocations[i]["sign_in"]["check"] = unexplained_check(outcome);
    if !requests::setup_sign_in_hook(
        t,
        a,
        c,
        i,
        &instance,
        "sign-in-hook.v1",
        &["PND-010", "IFC-006", "ACC-018", "IMP-002"],
        login.as_deref(),
    )? {
        return end_sign_in(
            t,
            a,
            c,
            i,
            "failed",
            "PND-010",
            "No implementation of sign-in-hook.v1 is resolved for the instance, so no login can be started and sign-in.v1 signs nothing in.",
        );
    }
    t.state.invocations[i]["sign_in"]["phase"] = json!("sign-in-pending");
    Ok(())
}

fn poll_answered(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    part: &Value,
    outcome: &str,
    account: &str,
) -> Result<()> {
    let invocation = &t.state.invocations[i];
    match poll_check(
        outcome,
        &part["retained"]["outputs"],
        account,
        &invocation["sign_in"]["first_account"],
    ) {
        Poll::Expected(found) => signed_in(t, a, c, i, &found),
        Poll::NoneYet => {
            let id = invocation["id"]
                .as_str()
                .ok_or("sign-in has no ID")?
                .to_owned();
            t.state.invocations[i]["sign_in"]["check"] = unexplained_check(outcome);
            t.decisions.push(decision(
                "ask-sign-in-again",
                &["IFC-049", "PND-010", "PND-011"],
                &["DL18"],
                json!({"invocation":id,"hook":part["id"]}),
                vec![],
            ));
            relay_output(t, &id, &a.evidence.time, c)
        }
        Poll::Other => {
            t.decisions.push(decision(
                "end-sign-in-other-account",
                &["PND-011"],
                &[],
                json!({"invocation":invocation["id"],"hook":part["id"]}),
                vec![],
            ));
            end_sign_in(
                t,
                a,
                c,
                i,
                "failed",
                "PND-011",
                "While polling, the account check found an account that is neither the expected one nor the one the first check found; undoing it with a sign-out-hook.v1 part is not built yet (order 048a-5gh), so sign-in.v1 ends failed and nothing was signed in.",
            )
        }
        Poll::NotKnown => end_sign_in(
            t,
            a,
            c,
            i,
            "failed",
            "PND-011",
            &format!(
                "The account check while polling after the person's sign-in reported {outcome}, so the account is not known and sign-in.v1 completes nothing; send sign-in.v1 again."
            ),
        ),
    }
}

/// PND-011: an account check that ended failed or unknown, which the relay's message explains.
fn unexplained_check(outcome: &str) -> Value {
    match outcome {
        "failed" | "unknown" => json!(outcome),
        _ => Value::Null,
    }
}

pub(crate) fn waiting_for_retry(v: &Value, attempt: u64) -> bool {
    v["sign_in"]["phase"] == "retry-pending" && v["sign_in_wait"]["attempt"] == attempt
}

pub(crate) fn retried(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    responses: &Value,
) -> Result<()> {
    let invocation = &t.state.invocations[i];
    match url_action(responses) {
        Some("accept") => {
            let Some(instance) = access::object_by_id(&t.state, &invocation["object"])
                .filter(|o| o["state"] != "retired")
                .cloned()
            else {
                return end_sign_in(
                    t,
                    a,
                    c,
                    i,
                    "failed",
                    "PND-003",
                    "The instance was retired while sign-in.v1 waited for the person, so it polls nothing and signs nothing in.",
                );
            };
            if !requests::setup_sign_in_hook(
                t,
                a,
                c,
                i,
                &instance,
                "account-hook.v1",
                &["PND-010", "PND-011", "IFC-049", "ACC-018", "IMP-002"],
                None,
            )? {
                return end_sign_in(t, a, c, i, "failed", "PND-011", ACCOUNT_HOOK_NOT_RESOLVED);
            }
            let invocation = &mut t.state.invocations[i];
            let hook = &t.reply["hook"];
            invocation["sign_in"]["phase"] = json!("poll-pending");
            invocation["sign_in"]["poll"] = hook.clone();
            t.decisions.push(decision(
                "poll-sign-in-account",
                &["IFC-049", "PND-010", "PND-011"],
                &[],
                json!({"invocation":invocation["id"],"hook":hook}),
                vec![],
            ));
            Ok(())
        }
        Some(action @ ("decline" | "cancel")) => {
            t.decisions.push(decision(
                "end-sign-in-declined",
                &["PND-010", "PND-013", "IFC-049"],
                &[],
                json!({"invocation":invocation["id"],"action":action}),
                vec![],
            ));
            let did = if action == "decline" {
                "declined"
            } else {
                "cancelled"
            };
            end_sign_in(
                t,
                a,
                c,
                i,
                "failed",
                "PND-010",
                &format!(
                    "The person {did} the URL action on a retry, so no part was dispatched and nothing the retry carried was passed on."
                ),
            )
        }
        _ => {
            t.decisions.push(decision(
                "end-sign-in-malformed-response",
                &["IFC-049"],
                &[],
                json!({"invocation":invocation["id"]}),
                vec![],
            ));
            end_sign_in(
                t,
                a,
                c,
                i,
                "failed",
                "IFC-049",
                "The retry's inputResponses held no response under url whose action is accept, decline or cancel, and its requestState was used, so no part was dispatched and nothing the retry carried was passed on.",
            )
        }
    }
}

pub(crate) fn sign_in_hook_part(state: &State, part: &Value, c: &Context) -> bool {
    part["operation"] == c.qualified("sign-in-hook.v1")
        && part.get("setup_parent").is_some_and(|parent| {
            state
                .invocations
                .iter()
                .any(|v| v["id"] == *parent && v["operation"] == c.qualified("sign-in.v1"))
        })
}

pub(crate) fn hook_url_action(
    result: &Value,
) -> std::result::Result<(String, Option<String>), (&'static str, &'static str)> {
    if result["resultType"] != "input_required" {
        return Err(("asks for input", "resultType"));
    }
    let (key, request) = result["inputRequests"]
        .as_object()
        .filter(|requests| requests.len() == 1)
        .and_then(|requests| requests.iter().next())
        .ok_or(("exactly one input request", "inputRequests"))?;
    if request["method"] != "elicitation/create" {
        return Err(("an elicitation", "method"));
    }
    let params = &request["params"];
    if params["mode"] != "url" {
        return Err(("URL mode", "params.mode"));
    }
    if !params["message"].is_string() {
        return Err(("a message", "params.message"));
    }
    if !params["url"].is_string() {
        return Err(("a URL", "params.url"));
    }
    Ok((
        key.clone(),
        result["requestState"].as_str().map(str::to_owned),
    ))
}

pub(crate) fn hook_answer_cause(check: &str, field: &str) -> String {
    format!(
        "The sign-in hook's answer is not one URL elicitation, so nothing was relayed and the instance is not signed in: it fails the check \"{check}\" at the field {field}."
    )
}

pub(crate) fn url_match(
    url: &str,
    logins: &[Value],
    asked: Option<&str>,
) -> std::result::Result<String, (&'static str, &'static str, Option<String>)> {
    let url = Url::parse(url).map_err(|_| ("parses as a URL", "params.url", None))?;
    if !url.username().is_empty() || url.password().is_some() {
        return Err(("no user information", "userinfo", None));
    }
    if url.fragment().is_some() {
        return Err(("no fragment", "fragment", None));
    }
    let mut query = BTreeMap::new();
    for (name, value) in url.query_pairs() {
        if query.insert(name, value).is_some() {
            return Err(("no repeated parameter", "query", None));
        }
    }
    let same_endpoint: Vec<_> = logins
        .iter()
        .filter(|login| {
            login["authorization"]
                .as_str()
                .and_then(|authorization| Url::parse(authorization).ok())
                .is_some_and(|authorization| {
                    authorization.origin() == url.origin() && authorization.path() == url.path()
                })
        })
        .collect();
    if same_endpoint.is_empty() {
        return Err(("a stated login", "origin and path", None));
    }
    if let Some(name) = asked
        && !same_endpoint
            .iter()
            .any(|login| login["name"].as_str() == Some(name))
    {
        return Err(("the asked-for login", "origin and path", Some(name.into())));
    }

    let parameter = |name| query.get(name).map(|value| value.as_ref());
    let mut matched = None;
    let mut latest_failure = None;
    let exactly_one = ("exactly one login", "params.url", None);
    for login in same_endpoint {
        let (Some(name), Some(kind @ ("device" | "redirect" | "page" | "service"))) =
            (login["name"].as_str(), login["return"].as_str())
        else {
            continue;
        };
        if asked.is_some_and(|asked| asked != name) {
            continue;
        }
        match check_login_url(login, kind, url.query().is_some(), parameter) {
            Ok(()) => {
                if matched.replace(name).is_some() {
                    return Err(exactly_one);
                }
            }
            Err((row, check, field)) => {
                // Keep the first login in the Type's order when rows tie.
                if latest_failure.is_none_or(|(previous, _, _, _)| row > previous) {
                    latest_failure = Some((row, check, field, name));
                }
            }
        }
    }
    if let Some(name) = matched {
        return Ok(name.into());
    }
    Err(match latest_failure {
        Some((_, check, field, name)) => (check, field, Some(name.into())),
        None => exactly_one,
    })
}

fn check_login_url<'a>(
    login: &Value,
    kind: &str,
    has_query: bool,
    parameter: impl Fn(&'static str) -> Option<&'a str>,
) -> std::result::Result<(), (u8, &'static str, &'static str)> {
    if kind == "device" {
        return if has_query {
            Err((7, "no query", "query"))
        } else {
            Ok(())
        };
    }
    if !parameter("client_id").is_some_and(|client| Some(client) == login["clientId"].as_str()) {
        return Err((8, "the client ID", "client_id"));
    }
    if parameter("response_type") != Some("code") {
        return Err((9, "response type code", "response_type"));
    }
    let redirect_matches = parameter("redirect_uri").is_some_and(|uri| match kind {
        "redirect" => redirect_uri_matches(uri, &login["redirect"]),
        _ => login["page"].as_str() == Some(uri),
    });
    if !redirect_matches {
        return Err((10, "the redirect URI", "redirect_uri"));
    }
    if kind == "service" {
        return Ok(());
    }
    if parameter("code_challenge_method") != Some("S256") {
        return Err((11, "S256", "code_challenge_method"));
    }
    if !parameter("code_challenge").is_some_and(|challenge| {
        challenge.len() == 43
            && challenge
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_'))
    }) {
        return Err((12, "a code challenge", "code_challenge"));
    }
    if !parameter("state").is_some_and(|state| {
        state.len() >= 22
            && state
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~'))
    }) {
        return Err((13, "a state", "state"));
    }
    Ok(())
}

fn redirect_uri_matches(uri: &str, redirect: &Value) -> bool {
    let (Some(host), Some(path)) = (redirect["host"].as_str(), redirect["path"].as_str()) else {
        return false;
    };
    let Some(port) = uri
        .strip_prefix("http://")
        .and_then(|rest| rest.strip_prefix(host))
        .and_then(|rest| rest.strip_prefix(':'))
        .and_then(|rest| rest.strip_suffix(path))
    else {
        return false;
    };
    if port.starts_with('0') || !port.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let Ok(port) = port.parse::<u16>() else {
        return false;
    };
    let ports = &redirect["ports"];
    if ports == "any" {
        return port >= 1024;
    }
    // Every u16 fits exactly in f64, including a whole-valued JSON decimal.
    ports.as_array().is_some_and(|ports| {
        ports
            .iter()
            .any(|stated| stated.as_f64() == Some(f64::from(port)))
    })
}

pub(crate) fn url_match_cause(check: &str, field: &str, login: Option<&str>) -> String {
    match login {
        Some(name) => format!(
            "The sign-in hook's URL does not match the login \"{name}\" of the instance's Type, so nothing was relayed and the instance is not signed in: it fails the check \"{check}\" at the field {field}."
        ),
        None => format!(
            "The sign-in hook's URL does not match exactly one login of the instance's Type, so nothing was relayed and the instance is not signed in: it fails the check \"{check}\" at the field {field}."
        ),
    }
}

pub(crate) fn keep_hook_input(part: &mut Value, key: &str, state: Option<&str>) {
    let mut input = json!({"key":key});
    if let Some(state) = state {
        input["request_state"] = json!(state);
    }
    part["hook_input"] = input;
}

pub(crate) fn accepts_return(login: &Value, plain: bool) -> bool {
    matches!(login["return"].as_str(), Some("redirect" | "page"))
        && login["carried"] == true
        && !plain
}

pub(crate) fn wait_end(time: &str) -> Option<String> {
    Some(
        DateTime::parse_from_rfc3339(time)
            .ok()?
            .checked_add_signed(TimeDelta::minutes(10))?
            .to_utc()
            .to_rfc3339_opts(SecondsFormat::AutoSi, true),
    )
}

pub(crate) fn relay_request(message: &str, url: &str) -> Value {
    json!({"method":"elicitation/create","params":{"mode":"url","message":message,"url":url}})
}

pub(crate) fn login_meta(login: &Value, node: &Value, accepts: bool) -> Value {
    json!({"name":login["name"],"return":login["return"],"node":node,"acceptsReturn":accepts})
}

pub(crate) struct RelayFacts<'a> {
    pub kind: &'a str,
    pub name: &'a str,
    pub type_name: &'a str,
    pub package: &'a str,
    pub node: &'a str,
    pub host: &'a str,
    pub account: &'a str,
    pub login: &'a Value,
    pub accepts: bool,
    pub check: Option<&'a str>,
}

pub(crate) fn relay_message(f: &RelayFacts<'_>) -> String {
    let RelayFacts {
        kind,
        name,
        type_name,
        package,
        node,
        host,
        account,
        login,
        accepts,
        check,
    } = f;
    let ending = match login["return"].as_str() {
        Some("device") => format!(
            "The code to enter at {host} is shown in the engine's panel, never here; nothing comes back through this Interface."
        ),
        Some("service") => {
            let page = login["page"].as_str().unwrap_or_default();
            format!(
                "The service's own page {page} receives the login's return, so nothing comes back through this Interface."
            )
        }
        Some("redirect") if *accepts => {
            let rhost = login["redirect"]["host"].as_str().unwrap_or_default();
            let rpath = login["redirect"]["path"].as_str().unwrap_or_default();
            format!(
                "After you sign in, the browser is sent to an address on {rhost} with the path {rpath}. Where the browser runs on the node {node}'s machine, the login completes there; elsewhere, that address is what you bring back, and your Interface carries it or asks you to paste it."
            )
        }
        Some("page") if *accepts => {
            let page = login["page"].as_str().unwrap_or_default();
            format!(
                "After you sign in, the page {page} shows a code: that code is what you bring back, and you paste it into your Interface."
            )
        }
        _ => format!(
            "Nothing comes back through this Interface: complete the login on the node {node}'s machine, at it or over a connection Conduit manages, by remote desktop or by SSH, pasting the code or forwarding the port."
        ),
    };
    let check = check
        .map(|check| format!("The last account check ended {check}, so whether the account is signed in is not known; it counts as no account yet (PND-011). "))
        .unwrap_or_default();
    format!(
        "Sign in the {kind} {name} (Type {type_name}, package {package}) on the node {node} at {host}, as the account {account}. {check}{ending}"
    )
}

pub(crate) fn keep_sign_in_wait(v: &mut Value, login: &str, url: &str, plain: bool) {
    v["sign_in_wait"] = json!({"login":login,"url":url,"plain":plain});
}

pub(crate) fn relay_output(
    t: &mut Transition,
    sign_in: &str,
    time: &str,
    c: &Context,
) -> Result<()> {
    let i = t
        .state
        .invocations
        .iter()
        .position(|v| v["id"] == sign_in)
        .ok_or("sign-in invocation missing")?;
    let v = &t.state.invocations[i];
    let instance = access::object(&t.state, &v["object"]).ok_or("sign-in instance missing")?;
    let w = &v["sign_in_wait"];
    let login_name = w["login"].as_str().ok_or("sign-in wait missing")?;
    let url = w["url"].as_str().ok_or("sign-in wait missing")?;
    let plain = w["plain"].as_bool().ok_or("sign-in wait missing")?;
    let ExpectedAccount::Set { account, .. } = expected_account(instance, c)? else {
        return Err("PND-010: sign-in expected account missing".into());
    };
    let (ty, package) = inventory::instance_type_and_package(instance, c)?;
    let login = ty["spec"]["signIn"]["logins"]
        .as_array()
        .and_then(|logins| logins.iter().find(|login| login["name"] == login_name))
        .ok_or("sign-in login missing")?;
    let node = inventory::object(
        access::object(&t.state, &instance["placement"]).ok_or("sign-in node missing")?,
    );
    let parsed = Url::parse(url).map_err(|_| "sign-in URL did not parse")?;
    let host = parsed.host_str().ok_or("sign-in URL host missing")?;
    let accepts = accepts_return(login, plain);
    let expires = w["expires"]
        .as_str()
        .map(str::to_owned)
        .or_else(|| wait_end(time))
        .ok_or("sign-in expiry could not be determined")?;
    let attempt = v["attempt"].as_u64().ok_or("sign-in attempt missing")?;
    let meta = login_meta(login, &node, accepts);
    let check = v["sign_in"]["check"].as_str().map(str::to_owned);
    let message = relay_message(&RelayFacts {
        kind: instance["kind"]
            .as_str()
            .ok_or("sign-in instance kind missing")?,
        name: instance["name"]
            .as_str()
            .ok_or("sign-in instance name missing")?,
        type_name: ty["metadata"]["name"]
            .as_str()
            .ok_or("sign-in Type name missing")?,
        package: &package,
        node: node["name"].as_str().ok_or("sign-in node missing")?,
        host,
        account,
        login,
        accepts,
        check: check.as_deref(),
    });
    let output = InputRequired {
        invocation: sign_in.to_owned(),
        attempt,
        request: relay_request(&message, url),
        meta: json!({format!("{}signInLogin", c.meta_prefix):meta}),
        expires: expires.clone(),
    };
    let mut facts =
        json!({"invocation":sign_in,"attempt":attempt,"login":login_name,"expires":expires});
    if let Some(check) = check {
        facts["check"] = json!(check);
    }
    let v = &mut t.state.invocations[i];
    v["sign_in_wait"]["attempt"] = json!(attempt);
    v["sign_in_wait"]["expires"] = json!(expires);
    v["sign_in_wait"]["meta"] = meta;
    v["sign_in"]["phase"] = json!("retry-pending");
    t.input_required.push(output);
    t.decisions.push(decision(
        "relay-sign-in-url",
        &["IFC-049", "IFC-010", "PND-010"],
        &["DL18"],
        facts,
        vec![],
    ));
    t.reply = json!({"pending":true,"invocation":sign_in});
    Ok(())
}

pub(crate) fn hook_input(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    result: &Value,
) -> Result<()> {
    let part = &t.state.invocations[i];
    let parent = &part["setup_parent"];
    let hook = part["id"].clone();
    let p = t
        .state
        .invocations
        .iter()
        .position(|v| v["id"] == *parent)
        .ok_or("missing sign-in parent")?;
    let invocation = t.state.invocations[p]["id"].clone();
    let parent = &t.state.invocations[p];
    let drop_cause = if parent["state"] != "in-progress" {
        Some(
            "The sign-in hook asked for input after its sign-in.v1 had ended, so nothing was relayed and no sign-in action was raised.",
        )
    } else if parent["sign_in"]["phase"] != "sign-in-pending" {
        Some(
            "The sign-in hook asked for input again after its URL request was relayed, so nothing was relayed again and sign-in.v1 goes on waiting.",
        )
    } else {
        None
    };
    if let Some(cause) = drop_cause {
        finish_hook_input(t, a, c, i, cause)?;
        t.decisions.push(decision(
            "drop-sign-in-hook-answer",
            &["IFC-010", "OPS-027"],
            &[],
            json!({"invocation":invocation,"hook":hook}),
            vec![],
        ));
        return Ok(());
    }

    let action = hook_url_action(result);
    let matched = match (active_hook_subject(t, p), action) {
        (Some(instance), Ok((key, state))) => {
            let url = result["inputRequests"][&key]["params"]["url"]
                .as_str()
                .unwrap();
            let logins = stated_logins(instance, c)?;
            let asked = t.state.invocations[p]["asked_login"].as_str();
            Some(
                url_match(url, &logins, asked)
                    .map(|login| (login, key, state, url))
                    .map_err(|(check, field, login)| HookAnswerFault::Url(check, field, login)),
            )
        }
        (Some(_), Err((check, field))) => Some(Err(HookAnswerFault::Form(check, field))),
        (None, _) => None,
    };
    let fault = match matched {
        Some(Ok((login, key, state, url))) => {
            let plain = t.state.invocations[i]["plain"] == true;
            keep_hook_input(&mut t.state.invocations[i], &key, state.as_deref());
            keep_sign_in_wait(&mut t.state.invocations[p], &login, url, plain);
            return relay_output(
                t,
                invocation.as_str().ok_or("sign-in has no ID")?,
                &a.evidence.time,
                c,
            );
        }
        Some(Err(fault)) => Some(fault),
        None => None,
    };
    finish_hook_input(
        t,
        a,
        c,
        i,
        "The hook asked for input that sign-in.v1 did not relay, so whether the login it began is still waiting is not known.",
    )?;
    record_hook_part(t, p, &hook, "unknown");
    match fault {
        Some(fault) => refuse_hook_answer(t, a, c, p, &hook, fault),
        None => end_sign_in(t, a, c, p, "failed", "PND-003", RETIRED_DURING_HOOK),
    }
}

fn finish_hook_input(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    cause: &str,
) -> Result<()> {
    let diagnostic =
        explain::outcome_diagnostic(&t.state.invocations[i], "unknown", "IFC-010", cause)?;
    requests::finish(
        t,
        a,
        c,
        i,
        "unknown",
        json!({}),
        json!([]),
        Some(vec![diagnostic]),
    )
}

fn active_hook_subject(t: &Transition, i: usize) -> Option<&Value> {
    t.state
        .objects
        .iter()
        .find(|o| o["id"] == t.state.invocations[i]["object"])
        .filter(|o| o["state"] != "retired")
}

fn record_hook_part(t: &mut Transition, i: usize, hook: &Value, outcome: &str) {
    t.state.invocations[i]["sign_in"]["parts"]
        .as_array_mut()
        .unwrap()
        .push(json!({"name":"sign-in","outcome":outcome,"invocation":hook}));
}

enum HookAnswerFault {
    Form(&'static str, &'static str),
    Url(&'static str, &'static str, Option<String>),
}

fn refuse_hook_answer(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    hook: &Value,
    fault: HookAnswerFault,
) -> Result<()> {
    let (check, field, login, cause) = match fault {
        HookAnswerFault::Form(check, field) => {
            (check, field, None, hook_answer_cause(check, field))
        }
        HookAnswerFault::Url(check, field, login) => {
            let cause = url_match_cause(check, field, login.as_deref());
            (check, field, login, cause)
        }
    };
    let mut facts = json!({
        "invocation":t.state.invocations[i]["id"],"hook":hook,"check":check,"field":field
    });
    if let Some(login) = login {
        facts["login"] = json!(login);
    }
    t.decisions.push(decision(
        "refuse-sign-in-hook-answer",
        &["IFC-049", "IFC-010"],
        &[],
        facts,
        vec![],
    ));
    end_sign_in(t, a, c, i, "failed", "IFC-049", &cause)
}

fn hook_ended(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    hook: &Value,
    outcome: &str,
) -> Result<()> {
    record_hook_part(t, i, hook, outcome);
    if active_hook_subject(t, i).is_none() {
        return end_sign_in(t, a, c, i, "failed", "PND-003", RETIRED_DURING_HOOK);
    }
    if outcome == "succeeded" {
        return refuse_hook_answer(
            t,
            a,
            c,
            i,
            hook,
            HookAnswerFault::Form("asks for input", "resultType"),
        );
    }
    let (outcome, cause) = match outcome {
        "failed" => ("failed", "The sign-in hook ended failed, so no login was started and sign-in.v1 signs nothing in.".into()),
        "unknown" => ("unknown", "The sign-in hook ended unknown, so whether it began a login is not known, and sign-in.v1 signs nothing in.".into()),
        reason => ("failed", format!("The sign-in hook reported {reason}, so no login was started and sign-in.v1 signs nothing in.")),
    };
    end_sign_in(t, a, c, i, outcome, "PND-010", &cause)
}

pub(crate) fn answer_and_lift(
    state: &mut State,
    instance: &str,
    invocation: &Value,
    time: &str,
) -> (Option<Value>, bool) {
    let action = pending::unanswered_sign_in(state, &json!({"id":instance}), time).cloned();
    let answered = action.map(|action| {
        pending::store_answer(
            state,
            action,
            pending::invocation_answer("signed-in", invocation, time),
        )
    });
    let lifted = state.sign_in_holds.remove(instance).is_some();
    (answered, lifted)
}

pub(crate) fn end_hook_wait(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    why: &str,
) -> Result<()> {
    let parent = t.state.invocations[i]["id"].clone();
    let operation = c.qualified("sign-in-hook.v1");
    let cause =
        format!("The sign-in.v1 invocation it serves ended {why}, so its retry is never sent.");
    for p in 0..t.state.invocations.len() {
        let part = &t.state.invocations[p];
        if part["setup_parent"] == parent
            && part["operation"] == operation
            && part["state"] == "in-progress"
            && part.get("hook_input").is_some()
        {
            let hook = part["id"].clone();
            let diagnostic = explain::outcome_diagnostic(part, "unknown", "PND-010", &cause)?;
            requests::finish(
                t,
                a,
                c,
                p,
                "unknown",
                json!({}),
                json!([]),
                Some(vec![diagnostic]),
            )?;
            record_hook_part(t, i, &hook, "unknown");
        }
    }
    Ok(())
}

pub(crate) fn signed_in(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    found: &str,
) -> Result<()> {
    end_hook_wait(t, a, c, i, "succeeded")?;
    let invocation = t.state.invocations[i].clone();
    let instance = invocation["object"]
        .as_str()
        .ok_or("sign-in subject has no ID")?;
    let (action, lifted) = answer_and_lift(&mut t.state, instance, &invocation, &a.evidence.time);
    requests::finish(
        t,
        a,
        c,
        i,
        "succeeded",
        json!({"account":found}),
        json!([]),
        None,
    )?;
    t.state.invocations[i]["sign_in"]["phase"] = json!("ended");
    if let Some(action) = action {
        link_action(t, &invocation["id"], &action["id"])?;
        pending::record_answer(t, a, c, &action)?;
        t.decisions.push(decision(
            "answer-sign-in-signed-in",
            &["PND-003", "PND-010", "PND-011", "PND-012"],
            &["DL19"],
            json!({"action":action["id"]}),
            vec![],
        ));
    }
    if lifted {
        t.decisions.push(decision(
            "lift-sign-in-hold",
            &["PND-005", "PND-010"],
            &[],
            json!({"instance":instance}),
            vec![],
        ));
    }
    Ok(())
}

fn end_sign_in(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    outcome: &str,
    rule: &str,
    cause: &str,
) -> Result<()> {
    end_hook_wait(t, a, c, i, &format!("{outcome} under {rule}"))?;
    let diagnostic = explain::outcome_diagnostic(&t.state.invocations[i], outcome, rule, cause)?;
    requests::finish(
        t,
        a,
        c,
        i,
        outcome,
        json!({}),
        json!([]),
        Some(vec![diagnostic]),
    )?;
    let invocation = &mut t.state.invocations[i];
    invocation["sign_in"]["phase"] = json!("ended");
    if let Some(action) = invocation["sign_in"].get("raised") {
        let (id, action) = (invocation["id"].clone(), action.clone());
        link_action(t, &id, &action)?;
    }
    Ok(())
}

fn link_action(t: &mut Transition, invocation: &Value, action: &Value) -> Result<()> {
    let ended = t
        .records
        .iter_mut()
        .rev()
        .find(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == *invocation)
        .ok_or("sign-in invocation has no ending record")?;
    ended["data"]["pendingAction"] = json!([action]);
    let stored = t
        .state
        .records
        .iter_mut()
        .rev()
        .find(|r| r["id"] == ended["id"])
        .ok_or("sign-in invocation ending record absent from state")?;
    *stored = ended.clone();
    Ok(())
}

pub(crate) enum Trigger {
    LapsedLogin,
    UrlElicitation,
}

impl Trigger {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::LapsedLogin => "lapsed-login",
            Self::UrlElicitation => "url-elicitation",
        }
    }

    pub(crate) fn rule(&self) -> &'static str {
        match self {
            Self::LapsedLogin => "PND-005",
            Self::UrlElicitation => "IFC-016",
        }
    }

    pub(crate) fn cause(&self, expected: &ExpectedAccount<'_>, retired: bool) -> &'static str {
        match (self, expected, retired) {
            (Self::UrlElicitation, ExpectedAccount::Set { .. }, true) => {
                "The implementation asked for a URL elicitation outside sign-in.v1 after the instance retired."
            }
            (Self::LapsedLogin, ExpectedAccount::Set { .. }, true) => {
                "The implementation reports a missing or lapsed login after the instance retired."
            }
            (Self::UrlElicitation, ExpectedAccount::Set { .. }, false) => {
                "The implementation asked for a URL elicitation outside sign-in.v1, and the instance awaits sign-in to its expected account under PND-005."
            }
            (Self::UrlElicitation, ExpectedAccount::NoSignIn, _) => {
                "The implementation asked for a URL elicitation outside sign-in.v1, and the instance's Type states no signIn, so no sign-in action is raised."
            }
            (Self::UrlElicitation, ExpectedAccount::NotString { .. }, _) => {
                "The implementation asked for a URL elicitation outside sign-in.v1, and the instance's expected account is not set, so no sign-in action is raised."
            }
            (Self::LapsedLogin, ExpectedAccount::Set { .. }, false) => {
                "The implementation reports a missing or lapsed login, and the instance awaits sign-in to its expected account."
            }
            (Self::LapsedLogin, ExpectedAccount::NoSignIn, _) => {
                "The implementation reports a missing or lapsed login, but the instance's Type states no signIn, so no sign-in action is raised."
            }
            (Self::LapsedLogin, ExpectedAccount::NotString { .. }, _) => {
                "The implementation reports a missing or lapsed login, but the instance's expected account is not set, so no sign-in action is raised."
            }
        }
    }
}

pub(crate) fn url_elicitation(result: &Value) -> bool {
    fn is_url(request: &Value) -> bool {
        request["method"] == "elicitation/create" && request["params"]["mode"] == "url"
    }
    result["resultType"] == "input_required"
        && match &result["inputRequests"] {
            Value::Object(requests) => requests.values().any(is_url),
            Value::Array(requests) => requests.iter().any(is_url),
            _ => false,
        }
}

pub(crate) fn lapse_diagnostic(result: &Value) -> bool {
    result["structuredContent"]["diagnostics"]
        .as_array()
        .is_some_and(|diagnostics| {
            diagnostics
                .iter()
                .any(|d| d["ruleId"] == "PND-005" && d["level"] == "error")
        })
}

pub(crate) fn preflight(
    state: &State,
    route: &Value,
    request: &AccessRequest,
    time: &str,
    context: &Context,
) -> Result<Option<Rejection>> {
    if route["operation"] != context.qualified("sign-in.v1") {
        return Ok(None);
    }
    let reference = &route["arguments"]["instance"];
    let Some(instance) = access::object(state, reference) else {
        return Err("sign-in.v1 reached its preflight from a caller or for an instance that access does not permit (ACC-033 row 16, PND-010)".into());
    };
    // PND-010, ACC-033 row 16: access permits only root, or an Interface on its own behalf
    // through an Agent's owning Fleet; should another request reach here, fail, never dispatch.
    let root = access::object_by_id(state, &route["caller"])
        .is_some_and(|c| c["from"] == context.qualified("root") && c.get("spec").is_none());
    let owner = instance["kind"] == "Agent"
        && access::object(state, &instance["owner"]).is_some_and(|o| o["id"] == route["target"]);
    if !root
        && !(owner && pending::own_interface(state, &route["caller"], &route["consumer"]).is_some())
    {
        return Err("sign-in.v1 reached its preflight from a caller or for an instance that access does not permit (ACC-033 row 16, PND-010)".into());
    }
    if reference.get("id").is_some() {
        let tool = route["tool"].as_str().ok_or("sign-in route has no tool")?;
        return Ok(Some(rejection(
            "invalid",
            "LIF-031",
            tool,
            reference,
            "an own-behalf request must name the instance by reference instead of supplying its resolved ID.",
        )));
    }
    let expected = expected_account(instance, context)?;
    let logins = if matches!(expected, ExpectedAccount::Set { .. }) {
        stated_logins(instance, context)?
    } else {
        Vec::new()
    };
    let names = stated_login_names(&logins);
    if let Some(rejection) = refusal(
        state, route, instance, time, expected, request, &names, context,
    )? {
        return Ok(Some(rejection));
    }

    let others = other_sign_in_ports(state, instance, context)?;
    if others.is_empty() {
        return Ok(None);
    }
    let asked = request
        .sign_in_login
        .as_ref()
        .and_then(|v| v["name"].as_str());
    let own = may_start_fixed_ports(&logins, asked);
    fixed_port_refusal(state, route, instance, &own, &others, time, context)
}

fn stated_logins(instance: &Value, context: &Context) -> Result<Vec<Value>> {
    let mut ty = inventory::instance_type(instance, context)?;
    Ok(match ty["spec"]["signIn"]["logins"].take() {
        Value::Array(logins) => logins,
        _ => Vec::new(),
    })
}

fn stated_login_names(logins: &[Value]) -> Vec<String> {
    logins
        .iter()
        .filter_map(|login| login["name"].as_str().map(str::to_owned))
        .collect()
}

pub(crate) fn may_start_fixed_ports(logins: &[Value], asked: Option<&str>) -> BTreeSet<u64> {
    logins
        .iter()
        .filter(|login| {
            login["return"] == "redirect"
                && asked.is_none_or(|name| login["name"].as_str() == Some(name))
        })
        .filter_map(|login| login["redirect"]["ports"].as_array())
        .flatten()
        .filter_map(|value| {
            value.as_u64().or_else(|| {
                // JSON Schema accepts whole-valued decimals as integers. Ports fit exactly in f64.
                let port = value.as_f64()?;
                ((0.0..=f64::from(u16::MAX)).contains(&port) && port.fract() == 0.0)
                    .then_some(port as u64)
            })
        })
        .collect()
}

pub(crate) fn other_sign_in_ports(
    state: &State,
    instance: &Value,
    c: &Context,
) -> Result<Vec<(Value, BTreeSet<u64>)>> {
    let operation = c.qualified("sign-in.v1");
    let mut others = Vec::new();
    for invocation in &state.invocations {
        if invocation["state"] != "in-progress"
            || invocation["operation"] != operation
            || invocation["object"] == instance["id"]
        {
            continue;
        }
        let Some(other) = invocation
            .get("object")
            .and_then(|id| access::object_by_id(state, id))
        else {
            continue;
        };
        if !matches!(expected_account(other, c)?, ExpectedAccount::Set { .. }) {
            continue;
        }
        let logins = stated_logins(other, c)?;
        let ports = may_start_fixed_ports(&logins, invocation["asked_login"].as_str());
        others.push((other["id"].clone(), ports));
    }
    Ok(others)
}

pub(crate) fn fixed_port_refusal(
    state: &State,
    route: &Value,
    instance: &Value,
    own: &BTreeSet<u64>,
    others: &[(Value, BTreeSet<u64>)],
    time: &str,
    context: &Context,
) -> Result<Option<Rejection>> {
    if own.is_empty() {
        return Ok(None);
    }
    let Some((id, ports)) = others.iter().find(|(_, ports)| !own.is_disjoint(ports)) else {
        return Ok(None);
    };
    let shared: Vec<_> = own.intersection(ports).map(u64::to_string).collect();
    let label = if shared.len() == 1 { "port" } else { "ports" };
    let ports = shared.join(", ");
    let tool = route["tool"].as_str().ok_or("sign-in route has no tool")?;
    let mut cause = format!(
        "another sign-in.v1 in progress may start a login on the fixed {label} {ports}, which a login this request may start also states, and one fixed port serves one login at a time; send sign-in.v1 again once that sign-in ends."
    );
    if let Some(other) = access::object_by_id(state, id)
        && let Some(action) = visible_sign_in_action(state, other, &route["caller"], time, context)?
    {
        cause.push_str(&format!(
            " The other instance's pending sign-in action is {action}."
        ));
    }
    Ok(Some(rejection(
        "unavailable",
        "PND-010",
        tool,
        instance,
        &cause,
    )))
}

fn request_member_refusal(
    route: &Value,
    instance: &Value,
    request: &AccessRequest,
    names: &[String],
    context: &Context,
) -> Option<Rejection> {
    let tool = route["tool"].as_str()?;
    if !declares_url_elicitation(request) {
        return Some(rejection(
            "invalid",
            "IFC-049",
            tool,
            instance,
            r#"the request does not declare URL elicitation in its client capabilities, and sign-in.v1 relays a login only by URL elicitation; send it again with {"elicitation": {"url": {}}} in io.modelcontextprotocol/clientCapabilities."#,
        ));
    }
    if request.sign_in_return.is_some() {
        let key = format!("{}signInReturn", context.meta_prefix);
        return Some(rejection(
            "invalid",
            "PND-013",
            tool,
            instance,
            &format!(
                "the request carries {key} on a first sign-in.v1 request, which takes no return; a return travels only on a retry of sign-in.v1 in the invocation its requestState binds."
            ),
        ));
    }
    let login = request.sign_in_login.as_ref()?;
    let stated_logins = names
        .iter()
        .map(|name| json!(name).to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let cause = match login["name"].as_str() {
        Some(name) if names.iter().any(|stated| stated == name) => {
            let hook = context.qualified("sign-in-hook.v1");
            let item = &instance["resolved_implementations"][&hook]["item"];
            if item["plain"] == true && !requests::keyed_hook(item, &hook) {
                return Some(rejection(
                    "invalid",
                    "PND-010",
                    tool,
                    instance,
                    &format!(
                        "signInLogin asks for the login {}, but the instance's sign-in-hook.v1 is a plain hook, its implementation item marked plain and not listing sign-in-hook.v1 under keyed (PKG-020), which gets no login name (IFC-007); send the request again without signInLogin, and the hook chooses among the stated logins {stated_logins}.",
                        json!(name)
                    ),
                ));
            }
            return None;
        }
        Some(name) => format!(
            "signInLogin asks for the login {}, which the instance's Type does not state",
            json!(name)
        ),
        None => "signInLogin must hold the name of one login the instance's Type states".into(),
    };
    Some(rejection(
        "invalid",
        "PND-010",
        tool,
        instance,
        &format!("{cause}; its stated logins are {stated_logins}."),
    ))
}

pub(crate) fn declares_url_elicitation(request: &AccessRequest) -> bool {
    request
        .client_capabilities
        .as_ref()
        .is_some_and(|capabilities| capabilities["elicitation"]["url"].is_object())
}

#[allow(clippy::too_many_arguments)]
fn refusal(
    state: &State,
    route: &Value,
    instance: &Value,
    time: &str,
    expected: ExpectedAccount<'_>,
    request: &AccessRequest,
    names: &[String],
    context: &Context,
) -> Result<Option<Rejection>> {
    let tool = route["tool"].as_str().ok_or("sign-in route has no tool")?;
    let invalid = match expected {
        ExpectedAccount::NoSignIn => Some(
            "the instance's Type states no signIn, so it has no expected account to sign in to."
                .into(),
        ),
        ExpectedAccount::NotString { pointer } => Some(format!(
            "the instance's expected account, at its Type's account pointer {pointer}, is not a string."
        )),
        ExpectedAccount::Set { .. } => None,
    };
    if let Some(invalid) = invalid {
        return Ok(Some(rejection(
            "invalid", "PND-010", tool, instance, &invalid,
        )));
    }
    if let Some(rejection) = request_member_refusal(route, instance, request, names, context) {
        return Ok(Some(rejection));
    }
    if route["arguments"]
        .get("expectedId")
        .is_some_and(|id| *id != instance["id"])
    {
        return Ok(Some(Rejection::new("conflict", "LIF-037")));
    }

    let operations = [
        (context.qualified("sign-in.v1"), "sign-in.v1"),
        (context.qualified("set-up.v1"), "setup"),
        (context.qualified("retire.v1"), "retirement"),
    ];
    let Some(what) = state.invocations.iter().find_map(|v| {
        if v["state"] != "in-progress" || v["object"] != instance["id"] {
            return None;
        }
        operations
            .iter()
            .find(|(op, _)| v["operation"] == *op)
            .map(|(_, what)| *what)
    }) else {
        return Ok(None);
    };

    let mut text = format!(
        "the instance's earlier {what} has been dispatched and has not ended; send sign-in.v1 again once it ends."
    );
    if let Some(id) = visible_sign_in_action(state, instance, &route["caller"], time, context)? {
        text.push_str(&format!(" Its pending sign-in action is {id}."));
    }
    Ok(Some(rejection(
        "conflict", "PND-010", tool, instance, &text,
    )))
}

fn visible_sign_in_action<'a>(
    state: &'a State,
    instance: &Value,
    caller: &Value,
    time: &str,
    context: &Context,
) -> Result<Option<&'a str>> {
    if let Some(action) = pending::unanswered_sign_in(state, instance, time)
        && let Some(holder) = access::object_by_id(state, caller)
        && pending::raised_view(action, holder, state, time, context)?.is_some()
    {
        return Ok(action["id"].as_str());
    }
    Ok(None)
}

pub(crate) fn rejection(
    reason: &'static str,
    rule: &'static str,
    tool: &str,
    instance: &Value,
    cause: &str,
) -> Rejection {
    let kind = instance["kind"].as_str().unwrap_or("Object");
    let name = instance["name"]
        .as_str()
        .or_else(|| instance["id"].as_str())
        .unwrap_or_default();
    let mut rejection = Rejection::new(reason, rule);
    rejection.diagnostics = Some(json!([{
        "ruleId":rule, "level":"error",
        "message":{"text":format!("{tool} reports {reason} under {rule} for {kind}/{name}: {cause}")}
    }]));
    rejection
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Evidence;
    use crate::explain::tests::{evidence, fixture, transition};

    fn url_action() -> Value {
        json!({
            "resultType":"input_required",
            "inputRequests":{"sign-in":{"method":"elicitation/create","params":{
                "mode":"url","message":"Open the device page.","url":"https://example.org/device"
            }}},
            "requestState":"opaque-hook-state-0001"
        })
    }

    fn fixture_logins() -> Vec<Value> {
        vec![
            json!({
                "name":"browser","return":"redirect","authorization":"https://example.org/authorize",
                "clientId":"placeholder-signed-agent","carried":true,
                "redirect":{"host":"127.0.0.1","ports":[8400,8401],"path":"/callback"}
            }),
            json!({
                "name":"device","return":"device","authorization":"https://example.org/device",
                "clientId":"placeholder-signed-agent"
            }),
        ]
    }

    #[test]
    fn sign_in_relay_message_and_wait() {
        let logins = fixture_logins();
        let [browser, device] = logins.as_slice() else {
            panic!("expected browser and device logins");
        };
        let page = json!({
            "name":"code","return":"page","authorization":"https://example.org/authorize",
            "clientId":"placeholder-signed-agent","page":"https://example.org/code","carried":true
        });
        let service = json!({
            "name":"service","return":"service","authorization":"https://example.org/authorize",
            "clientId":"placeholder-signed-agent","page":"https://example.org/done"
        });
        let mut local = browser.clone();
        local["carried"] = json!(false);
        for (login, plain, accepts) in [
            (browser, false, true),
            (browser, true, false),
            (&local, false, false),
            (&page, false, true),
            (device, false, false),
            (&service, false, false),
        ] {
            assert_eq!(accepts_return(login, plain), accepts);
        }
        for (login, accepts, message) in [
            (
                browser,
                true,
                "Sign in the Agent signed (Type signed-agent.v1, package example.org/sign-in-lapse) on the node local at example.org, as the account primary-account. After you sign in, the browser is sent to an address on 127.0.0.1 with the path /callback. Where the browser runs on the node local's machine, the login completes there; elsewhere, that address is what you bring back, and your Interface carries it or asks you to paste it.",
            ),
            (
                browser,
                false,
                "Sign in the Agent signed (Type signed-agent.v1, package example.org/sign-in-lapse) on the node local at example.org, as the account primary-account. Nothing comes back through this Interface: complete the login on the node local's machine, at it or over a connection Conduit manages, by remote desktop or by SSH, pasting the code or forwarding the port.",
            ),
            (
                &page,
                true,
                "Sign in the Agent signed (Type signed-agent.v1, package example.org/sign-in-lapse) on the node local at example.org, as the account primary-account. After you sign in, the page https://example.org/code shows a code: that code is what you bring back, and you paste it into your Interface.",
            ),
            (
                device,
                false,
                "Sign in the Agent signed (Type signed-agent.v1, package example.org/sign-in-lapse) on the node local at example.org, as the account primary-account. The code to enter at example.org is shown in the engine's panel, never here; nothing comes back through this Interface.",
            ),
            (
                &service,
                false,
                "Sign in the Agent signed (Type signed-agent.v1, package example.org/sign-in-lapse) on the node local at example.org, as the account primary-account. The service's own page https://example.org/done receives the login's return, so nothing comes back through this Interface.",
            ),
        ] {
            let actual = relay_message(&RelayFacts {
                kind: "Agent",
                name: "signed",
                type_name: "signed-agent.v1",
                package: "example.org/sign-in-lapse",
                node: "local",
                host: "example.org",
                account: "primary-account",
                login,
                accepts,
                check: None,
            });
            assert_eq!(actual, message);
            assert!(!actual.contains("Open the device page"));
        }
        assert_eq!(
            relay_message(&RelayFacts {
                kind: "Agent",
                name: "signed",
                type_name: "signed-agent.v1",
                package: "example.org/sign-in-lapse",
                node: "local",
                host: "example.org",
                account: "primary-account",
                login: device,
                accepts: false,
                check: Some("failed"),
            }),
            "Sign in the Agent signed (Type signed-agent.v1, package example.org/sign-in-lapse) on the node local at example.org, as the account primary-account. The last account check ended failed, so whether the account is signed in is not known; it counts as no account yet (PND-011). The code to enter at example.org is shown in the engine's panel, never here; nothing comes back through this Interface."
        );
        assert_eq!(
            relay_request("M", "https://example.org/x"),
            json!({"method":"elicitation/create","params":{"mode":"url","message":"M","url":"https://example.org/x"}})
        );
        let node = json!({"kind":"Resource","name":"local","id":"n1"});
        assert_eq!(
            login_meta(browser, &node, true),
            json!({"name":"browser","return":"redirect","node":{"kind":"Resource","name":"local","id":"n1"},"acceptsReturn":true})
        );
        assert_eq!(
            wait_end("2026-11-12T12:00:00Z").as_deref(),
            Some("2026-11-12T12:10:00Z")
        );
        assert_eq!(
            wait_end("2026-11-12T23:55:00+02:00").as_deref(),
            Some("2026-11-12T22:05:00Z")
        );
        assert_eq!(wait_end("soon"), None);
    }

    #[test]
    fn sign_in_polling_answers() {
        for (responses, action) in [
            (json!({"url":{"action":"accept"}}), Some("accept")),
            (
                json!({"url":{"action":"accept","content":{}}}),
                Some("accept"),
            ),
            (json!({"url":{"action":"decline"}}), Some("decline")),
            (json!({"url":{"action":"cancel"}}), Some("cancel")),
            (json!({"url":{"action":"ACCEPT"}}), None),
            (json!({"url":{}}), None),
            (json!({"url":"accept"}), None),
            (json!({}), None),
            (json!({"login":{"action":"accept"}}), None),
            (
                json!({"url":{"action":"accept"},"login":{"action":"accept"}}),
                None,
            ),
            (json!(null), None),
        ] {
            assert_eq!(super::url_action(&responses), action);
        }
        for (found, first, none_yet) in [
            (json!(null), json!("other-account"), true),
            (json!(null), json!(null), true),
            (json!("other-account"), json!("other-account"), true),
            (json!("OTHER-ACCOUNT"), json!("other-account"), true),
            (json!("third-account"), json!("other-account"), false),
            (json!("other-account"), json!(null), false),
            (json!(7), json!(null), false),
        ] {
            assert_eq!(no_account_yet(&found, &first), none_yet);
        }
        for (outcome, outputs, first, poll) in [
            (
                "succeeded",
                json!({"account":"Primary-Account"}),
                json!(null),
                Poll::Expected("Primary-Account".into()),
            ),
            (
                "succeeded",
                json!({"account":null}),
                json!("other-account"),
                Poll::NoneYet,
            ),
            (
                "succeeded",
                json!({"account":null}),
                json!(null),
                Poll::NoneYet,
            ),
            (
                "succeeded",
                json!({"account":"OTHER-ACCOUNT"}),
                json!("other-account"),
                Poll::NoneYet,
            ),
            ("failed", json!({}), json!(null), Poll::NoneYet),
            ("unknown", json!({}), json!(null), Poll::NoneYet),
            (
                "succeeded",
                json!({"account":"third-account"}),
                json!("other-account"),
                Poll::Other,
            ),
            (
                "succeeded",
                json!({"account":"other-account"}),
                json!(null),
                Poll::Other,
            ),
            ("unavailable", json!({}), json!(null), Poll::NotKnown),
            (
                "succeeded",
                json!({"account":7}),
                json!(null),
                Poll::NotKnown,
            ),
        ] {
            assert_eq!(
                poll_check(outcome, &outputs, "primary-account", &first),
                poll
            );
        }
    }

    #[test]
    fn sign_in_url_match() {
        const CC: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
        const ST: &str = "Wt3qK8xZp_LmN2vB-c.Y7h~R";
        const DEVICE: &str = "https://example.org/device";
        let mut logins = fixture_logins();
        logins.extend([
            json!({
                "name":"loopback","return":"redirect",
                "authorization":"https://auth.example.net/oauth/authorize",
                "clientId":"loopback-client","carried":true,
                "redirect":{"host":"localhost","ports":"any","path":"/auth/callback"}
            }),
            json!({
                "name":"code-page","return":"page",
                "authorization":"https://pages.example.com/oauth/authorize",
                "clientId":"page-client","page":"https://pages.example.com/oauth/code/callback",
                "carried":false
            }),
            json!({
                "name":"service","return":"service",
                "authorization":"https://pages.example.com/oauth/authorize",
                "clientId":"service-client","page":"https://service.example.com/done?from=login"
            }),
        ]);
        let pkce = format!("code_challenge={CC}&code_challenge_method=S256&state={ST}");
        let redirect = "http%3A%2F%2F127.0.0.1%3A8401%2Fcallback";
        let browser = format!(
            "https://example.org/authorize?response_type=code&client_id=placeholder-signed-agent&redirect_uri={redirect}&scope=openid&{pkce}"
        );
        let loopback = format!(
            "https://auth.example.net/oauth/authorize?response_type=code&client_id=loopback-client&redirect_uri=http%3A%2F%2Flocalhost%3A1455%2Fauth%2Fcallback&{pkce}"
        );
        let page_uri = "https%3A%2F%2Fpages.example.com%2Foauth%2Fcode%2Fcallback";
        let service_uri = "https%3A%2F%2Fservice.example.com%2Fdone%3Ffrom%3Dlogin";
        let page = format!(
            "https://pages.example.com/oauth/authorize?response_type=code&client_id=page-client&redirect_uri={page_uri}&{pkce}"
        );
        let service = format!(
            "https://pages.example.com/oauth/authorize?response_type=code&client_id=service-client&redirect_uri={service_uri}"
        );

        for (url, asked, login) in [
            (DEVICE.into(), None, "device"),
            (DEVICE.into(), Some("device"), "device"),
            (browser.clone(), None, "browser"),
            (browser.clone(), Some("browser"), "browser"),
            (browser.replace("8401", "8400"), None, "browser"),
            (
                browser.replace("scope=openid", "prompt=login"),
                None,
                "browser",
            ),
            (loopback.clone(), None, "loopback"),
            (loopback.replace("1455", "1024"), None, "loopback"),
            (loopback.replace("1455", "65535"), None, "loopback"),
            (page.clone(), None, "code-page"),
            (service.clone(), None, "service"),
        ] {
            assert_eq!(url_match(&url, &logins, asked), Ok(login.into()), "{url}");
        }
        let mut decimal_port = logins[0].clone();
        decimal_port["redirect"]["ports"] = json!([8401.0]);
        assert_eq!(
            url_match(&browser, &[decimal_port], None),
            Ok("browser".into())
        );
        let reject = |url: &str, asked, check, field, login: Option<&str>| {
            assert_eq!(
                url_match(url, &logins, asked),
                Err((check, field, login.map(str::to_owned))),
                "{url}"
            );
        };
        reject("not a url", None, "parses as a URL", "params.url", None);
        for url in [
            "https://user@example.org/device",
            "https://user:secret@example.org/device",
            "https://:secret@example.org/device",
        ] {
            reject(url, None, "no user information", "userinfo", None);
        }
        for suffix in ["#x", "#"] {
            reject(
                &format!("{DEVICE}{suffix}"),
                None,
                "no fragment",
                "fragment",
                None,
            );
        }
        reject(
            &format!("{browser}&state={ST}"),
            None,
            "no repeated parameter",
            "query",
            None,
        );
        for url in [
            "https://example.org/devices",
            "http://example.org/device",
            "https://example.org:8443/device",
        ] {
            reject(url, None, "a stated login", "origin and path", None);
        }
        reject(
            DEVICE,
            Some("browser"),
            "the asked-for login",
            "origin and path",
            Some("browser"),
        );
        for suffix in ["?user_code=WDJB-MJHT", "?"] {
            reject(
                &format!("{DEVICE}{suffix}"),
                None,
                "no query",
                "query",
                Some("device"),
            );
        }

        let reject_browser =
            |url: &str, check, field| reject(url, None, check, field, Some("browser"));
        for url in [
            browser.replace("placeholder-signed-agent", "another-client"),
            browser.replace("client_id=placeholder-signed-agent&", ""),
        ] {
            reject_browser(&url, "the client ID", "client_id");
        }
        reject_browser(
            &browser.replace("response_type=code", "response_type=token"),
            "response type code",
            "response_type",
        );
        for uri in [
            "http%3A%2F%2F127.0.0.1%3A8402%2Fcallback",
            "http%3A%2F%2F127.0.0.1%3A08401%2Fcallback",
            "http%3A%2F%2Flocalhost%3A8401%2Fcallback",
            "http%3A%2F%2F127.0.0.1%3A8401%2Fcallback%2Fx",
            "https%3A%2F%2F127.0.0.1%3A8401%2Fcallback",
            "http%3A%2F%2F127.0.0.1%3A8401%2Fcallback%3Fx%3D1",
        ] {
            reject_browser(
                &browser.replace(redirect, uri),
                "the redirect URI",
                "redirect_uri",
            );
        }
        reject_browser(
            &browser.replace("code_challenge_method=S256", "code_challenge_method=plain"),
            "S256",
            "code_challenge_method",
        );
        let short_cc = &CC[..CC.len() - 1];
        for challenge in [short_cc.into(), format!("{short_cc}%3D")] {
            reject_browser(
                &browser.replace(CC, &challenge),
                "a code challenge",
                "code_challenge",
            );
        }
        for state in [
            ST[..ST.len() - 3].into(),
            format!("{}%21", &ST[..ST.len() - 1]),
        ] {
            reject_browser(&browser.replace(ST, &state), "a state", "state");
        }
        reject(
            &loopback.replace("1455", "1023"),
            None,
            "the redirect URI",
            "redirect_uri",
            Some("loopback"),
        );
        reject(
            &page.replace(page_uri, service_uri),
            None,
            "the redirect URI",
            "redirect_uri",
            Some("code-page"),
        );
        reject(
            &service.replace("response_type=code&", ""),
            None,
            "response type code",
            "response_type",
            Some("service"),
        );
        reject(
            &service,
            Some("code-page"),
            "the client ID",
            "client_id",
            Some("code-page"),
        );
        reject(
            &page.replace("client_id=page-client&", ""),
            None,
            "the client ID",
            "client_id",
            Some("code-page"),
        );
        reject(
            &page.replace(
                page_uri,
                "https%3A%2F%2FPAGES.example.com%2Foauth%2Fcode%2Fcallback",
            ),
            None,
            "the redirect URI",
            "redirect_uri",
            Some("code-page"),
        );

        let mut second_device = logins[1].clone();
        second_device["name"] = json!("device-again");
        assert_eq!(
            url_match(DEVICE, &[logins[1].clone(), second_device], None),
            Err(("exactly one login", "params.url", None))
        );
        assert_eq!(
            url_match_cause("no query", "query", Some("device")),
            "The sign-in hook's URL does not match the login \"device\" of the instance's Type, so nothing was relayed and the instance is not signed in: it fails the check \"no query\" at the field query."
        );
        assert_eq!(
            url_match_cause("a stated login", "origin and path", None),
            "The sign-in hook's URL does not match exactly one login of the instance's Type, so nothing was relayed and the instance is not signed in: it fails the check \"a stated login\" at the field origin and path."
        );
    }

    #[test]
    fn sign_in_hook_answer_form() {
        let action = url_action();
        let with_state = |state: &str| Ok(("sign-in".into(), Some(state.into())));
        assert_eq!(
            hook_url_action(&action),
            with_state("opaque-hook-state-0001")
        );
        let mut changed = action.clone();
        changed["requestState"] = json!(" opaque/state+=\u{e9} ");
        assert_eq!(
            hook_url_action(&changed),
            with_state(" opaque/state+=\u{e9} ")
        );
        changed.as_object_mut().unwrap().remove("requestState");
        assert_eq!(hook_url_action(&changed), Ok(("sign-in".into(), None)));
        changed["requestState"] = json!(7);
        assert_eq!(hook_url_action(&changed), Ok(("sign-in".into(), None)));
        changed = action.clone();
        changed["inputRequests"]["sign-in"]["params"]["elicitationId"] = json!("");
        assert_eq!(
            hook_url_action(&changed),
            with_state("opaque-hook-state-0001")
        );

        let request = &action["inputRequests"]["sign-in"];
        for (pointer, value, check, field) in [
            (
                "/resultType",
                json!("complete"),
                "asks for input",
                "resultType",
            ),
            (
                "/inputRequests",
                json!([request]),
                "exactly one input request",
                "inputRequests",
            ),
            (
                "/inputRequests",
                json!({"sign-in":request,"again":request}),
                "exactly one input request",
                "inputRequests",
            ),
            (
                "/inputRequests/sign-in/method",
                json!("sampling/createMessage"),
                "an elicitation",
                "method",
            ),
            (
                "/inputRequests/sign-in/params/mode",
                json!("form"),
                "URL mode",
                "params.mode",
            ),
            (
                "/inputRequests/sign-in/params",
                json!("url"),
                "URL mode",
                "params.mode",
            ),
            (
                "/inputRequests/sign-in/params/url",
                json!(7),
                "a URL",
                "params.url",
            ),
        ] {
            let mut changed = action.clone();
            *changed.pointer_mut(pointer).unwrap() = value;
            assert_eq!(hook_url_action(&changed), Err((check, field)));
        }
        let mut changed = action.clone();
        changed.as_object_mut().unwrap().remove("resultType");
        assert_eq!(
            hook_url_action(&changed),
            Err(("asks for input", "resultType"))
        );
        changed = action.clone();
        changed["inputRequests"]["sign-in"]["params"]
            .as_object_mut()
            .unwrap()
            .remove("message");
        assert_eq!(
            hook_url_action(&changed),
            Err(("a message", "params.message"))
        );
        assert_eq!(
            hook_answer_cause("URL mode", "params.mode"),
            r#"The sign-in hook's answer is not one URL elicitation, so nothing was relayed and the instance is not signed in: it fails the check "URL mode" at the field params.mode."#
        );
        let mut part = json!({"id":"h1"});
        keep_hook_input(&mut part, "sign-in", Some("opaque-hook-state-0001"));
        assert_eq!(
            part,
            json!({"id":"h1","hook_input":{"key":"sign-in","request_state":"opaque-hook-state-0001"}})
        );
        keep_hook_input(&mut part, "sign-in", None);
        assert_eq!(part, json!({"id":"h1","hook_input":{"key":"sign-in"}}));
    }

    fn hook_part_fixture() -> (Transition, Context) {
        let (_, c) = fixture();
        let mut environment = c.core_environment.clone();
        let contracts = &mut environment.scopes.get_mut("core").unwrap().contracts;
        contracts.get_mut("type.v1").unwrap()["spec"]["signIn"] =
            json!({"account":"/account","logins":fixture_logins()});
        let sign_in_hook = json!({
            "kind":"Operation","metadata":{"name":"sign-in-hook.v1"},
            "spec":{"effect":"Start a login"}
        });
        contracts.insert("sign-in-hook.v1".into(), sign_in_hook.clone());
        let indexes = json!({"core":contracts});
        let instance = json!({"id":"s1","name":"s1","kind":"Agent","owner":"f","placement":"n1",
            "type":c.qualified("type.v1"),
            "held":{"environment":environment,"indexes":indexes,"contexts":{},"scope":"core"},
            "config":{"account":"primary-account"}});
        let held_target = crate::registration::publication_object(&instance, &c).unwrap();
        let mut parent = json!({
            "id":"g1","plain":false,"contract":{"spec":{}},"state":"in-progress",
            "attempt":1,"operation":c.qualified("sign-in.v1"),"target":"f",
            "held_target":{"id":"f","kind":"Fleet"},"sender":"r","caller":"r","object":"s1",
            "sign_in":{"phase":"sign-in-pending","parts":[
                {"name":"account","outcome":"succeeded","invocation":"k1"}
            ]}
        });
        parent["named_subject"] = json!("Agent/s1");
        let part = json!({
            "id":"h1","plain":true,"contract":sign_in_hook,
            "state":"in-progress","attempt":1,
            "operation":c.qualified("sign-in-hook.v1"),"target":"s1",
            "held_target":held_target,"sender":"r","caller":"r","setup_parent":"g1","serving":"g1"
        });
        let state = State {
            objects: vec![
                json!({"id":"r","kind":"Interface","from":c.qualified("root")}),
                instance,
                json!({"id":"n1","kind":"Resource","name":"local"}),
            ],
            invocations: vec![parent, part],
            ..State::default()
        };
        (transition(state), c)
    }

    fn assert_device_relay(t: &Transition, c: &Context) {
        let parent = &t.state.invocations[0];
        let part = &t.state.invocations[1];
        assert_eq!(part["state"], "in-progress");
        assert_eq!(
            part["hook_input"],
            json!({"key":"sign-in","request_state":"opaque-hook-state-0001"})
        );
        assert_eq!(parent["state"], "in-progress");
        assert_eq!(parent["sign_in"]["phase"], "retry-pending");
        assert_eq!(parent["sign_in_wait"]["url"], "https://example.org/device");
        assert_eq!(parent["sign_in_wait"]["expires"], "2026-11-09T13:10:00Z");
        assert_eq!(t.reply, json!({"pending":true,"invocation":"g1"}));
        assert_eq!(t.input_required.len(), 1);
        let entry = &t.input_required[0];
        assert_eq!(
            entry.request["params"],
            json!({
                "mode":"url",
                "message":"Sign in the Agent s1 (Type type.v1, package test.core) on the node local at example.org, as the account primary-account. The code to enter at example.org is shown in the engine's panel, never here; nothing comes back through this Interface.",
                "url":"https://example.org/device"
            })
        );
        assert_eq!(
            entry.meta[format!("{}signInLogin", c.meta_prefix)],
            json!({"name":"device","return":"device","node":{"kind":"Resource","name":"local","id":"n1"},"acceptsReturn":false})
        );
        assert_hook_input_private(t);
        let entry = serde_json::to_string(entry).unwrap();
        for value in ["opaque-hook-state-0001", "Open the device page."] {
            assert!(!entry.contains(value));
        }
    }

    fn assert_hook_input_private(t: &Transition) {
        assert!(t.state.pending_actions.is_empty());
        assert!(t.state.sign_in_holds.is_empty());
        assert!(t.external_intents.is_empty());
        assert!(
            !t.decisions
                .iter()
                .any(|d| d.action == "raise-sign-in-action")
        );
        let facts: Vec<_> = t.decisions.iter().map(|d| &d.facts).collect();
        let surfaced = serde_json::to_string(&json!([
            t.reply,
            t.records,
            t.request_replies,
            t.external_intents,
            t.state.records,
            facts
        ]))
        .unwrap();
        for value in [
            "opaque-hook-state-0001",
            "example.org/device",
            "Open the device page.",
        ] {
            assert!(!surfaced.contains(value));
        }
    }

    #[test]
    fn sign_in_hook_input_kept_nowhere() {
        let (base, c) = hook_part_fixture();
        let complete = |t: &mut Transition, result| {
            requests::complete(
                t,
                &evidence(),
                "h1",
                1,
                requests::Completion::Result { result },
                &c,
            )
            .unwrap();
        };
        let absent = |t: &Transition| {
            assert_hook_input_private(t);
            assert!(t.input_required.is_empty());
        };
        let part_ended = |t: &Transition| {
            let part = &t.state.invocations[1];
            assert_eq!(part["state"], "unknown");
            assert!(part.get("hook_input").is_none());
            let cause = part["cause"].as_array().unwrap();
            assert_eq!(cause.len(), 1);
            assert_eq!(cause[0]["ruleId"], "IFC-010");
        };
        let sign_in_failed = |t: &Transition, rule, cause: &str| {
            let parent = &t.state.invocations[0];
            assert_eq!(parent["state"], "failed");
            assert_eq!(parent["sign_in"]["phase"], "ended");
            assert_eq!(
                parent["sign_in"]["parts"]
                    .as_array()
                    .unwrap()
                    .last()
                    .unwrap(),
                &json!({"name":"sign-in","outcome":"unknown","invocation":"h1"})
            );
            let diagnostics = t.reply["structuredContent"]["diagnostics"]
                .as_array()
                .unwrap();
            assert_eq!(diagnostics.len(), 1);
            assert_eq!(diagnostics[0]["ruleId"], rule);
            assert!(
                diagnostics[0]["message"]["text"]
                    .as_str()
                    .unwrap()
                    .ends_with(cause)
            );
        };

        let mut plain = base.clone();
        complete(&mut plain, url_action());
        assert_device_relay(&plain, &c);

        let mut keyed = base.clone();
        keyed.state.invocations[1]["plain"] = json!(false);
        complete(&mut keyed, url_action());
        assert_device_relay(&keyed, &c);
        assert_eq!(keyed.state.invocations[0]["sign_in_wait"]["plain"], false);

        let mut malformed = base.clone();
        let mut action = url_action();
        action["inputRequests"]["sign-in"]["params"]["mode"] = json!("form");
        complete(&mut malformed, action);
        part_ended(&malformed);
        sign_in_failed(
            &malformed,
            "IFC-049",
            &hook_answer_cause("URL mode", "params.mode"),
        );
        absent(&malformed);

        let mut cancelled = base.clone();
        cancelled.state.invocations[0]["state"] = json!("unknown");
        cancelled.state.invocations[0]["ended"] =
            json!({"time":evidence().time,"outcome":"unknown"});
        let parent = cancelled.state.invocations[0].clone();
        complete(&mut cancelled, url_action());
        part_ended(&cancelled);
        assert_eq!(cancelled.state.invocations[0], parent);
        assert!(
            cancelled
                .decisions
                .iter()
                .any(|d| d.action == "drop-sign-in-hook-answer")
        );
        absent(&cancelled);

        let mut waiting = base;
        waiting.state.invocations[0]["sign_in"]["phase"] = json!("retry-pending");
        keep_hook_input(
            &mut waiting.state.invocations[1],
            "sign-in",
            Some("opaque-hook-state-0001"),
        );
        let parent = waiting.state.invocations[0].clone();
        complete(&mut waiting, url_action());
        part_ended(&waiting);
        assert_eq!(waiting.state.invocations[0], parent);
        assert!(
            waiting
                .decisions
                .iter()
                .any(|d| d.action == "drop-sign-in-hook-answer")
        );
        absent(&waiting);
    }

    #[test]
    fn sign_in_url_mismatch_kept_nowhere() {
        let (base, c) = hook_part_fixture();
        let complete = |t: &mut Transition, result| {
            requests::complete(
                t,
                &evidence(),
                "h1",
                1,
                requests::Completion::Result { result },
                &c,
            )
            .unwrap();
        };
        let assert_ended = |t: &Transition, rule, cause: &str| {
            let parent = &t.state.invocations[0];
            let part = &t.state.invocations[1];
            assert_eq!(part["state"], "unknown");
            assert!(part.get("hook_input").is_none());
            let part_cause = part["cause"].as_array().unwrap();
            assert_eq!(part_cause.len(), 1);
            assert_eq!(part_cause[0]["ruleId"], "IFC-010");
            assert_eq!(parent["state"], "failed");
            assert_eq!(parent["sign_in"]["phase"], "ended");
            assert_eq!(
                parent["sign_in"]["parts"]
                    .as_array()
                    .unwrap()
                    .last()
                    .unwrap(),
                &json!({"name":"sign-in","outcome":"unknown","invocation":"h1"})
            );
            let diagnostics = t.reply["structuredContent"]["diagnostics"]
                .as_array()
                .unwrap();
            assert_eq!(diagnostics.len(), 1);
            assert_eq!(diagnostics[0]["ruleId"], rule);
            assert!(
                diagnostics[0]["message"]["text"]
                    .as_str()
                    .unwrap()
                    .ends_with(cause)
            );
            assert!(t.state.pending_actions.is_empty());
            assert!(t.state.sign_in_holds.is_empty());
            assert!(t.input_required.is_empty());
            assert!(
                !t.decisions
                    .iter()
                    .any(|d| d.action == "raise-sign-in-action")
            );
            let facts: Vec<_> = t.decisions.iter().map(|d| &d.facts).collect();
            let surfaced = serde_json::to_string(&json!([
                t.reply,
                t.records,
                t.request_replies,
                t.external_intents,
                t.state.records,
                facts
            ]))
            .unwrap();
            for value in [
                "WDJB-MJHT",
                "user_code",
                "example.org/device",
                "opaque-hook-state-0001",
                "Open the device page.",
            ] {
                assert!(!surfaced.contains(value), "{value}");
            }
        };
        let mut device_with_code = url_action();
        device_with_code["inputRequests"]["sign-in"]["params"]["url"] =
            json!("https://example.org/device?user_code=WDJB-MJHT");
        for (asked, answer, check, field, login) in [
            (None, device_with_code, "no query", "query", "device"),
            (
                Some("browser"),
                url_action(),
                "the asked-for login",
                "origin and path",
                "browser",
            ),
        ] {
            let mut t = base.clone();
            if let Some(asked) = asked {
                t.state.invocations[0]["asked_login"] = json!(asked);
            }
            complete(&mut t, answer);
            assert_ended(&t, "IFC-049", &url_match_cause(check, field, Some(login)));
            let refused: Vec<_> = t
                .decisions
                .iter()
                .filter(|d| d.action == "refuse-sign-in-hook-answer")
                .map(|d| &d.facts)
                .collect();
            assert_eq!(
                refused,
                [&json!({
                    "invocation":"g1","hook":"h1","check":check,"field":field,"login":login
                })]
            );
            assert!(!t.decisions.iter().any(|d| d.action == "relay-sign-in-url"));
        }

        let mut matched = base;
        complete(&mut matched, url_action());
        assert_device_relay(&matched, &c);
        let relayed = matched
            .decisions
            .iter()
            .find(|d| d.action == "relay-sign-in-url")
            .unwrap();
        assert_eq!(
            relayed.facts,
            json!({"invocation":"g1","attempt":1,"login":"device","expires":"2026-11-09T13:10:00Z"})
        );
        assert!(
            !matched
                .decisions
                .iter()
                .any(|d| d.action == "refuse-sign-in-hook-answer")
        );
    }

    #[test]
    fn sign_in_hook_not_dispatched_while_retiring() {
        let (mut t, c) = hook_part_fixture();
        let mut environment = c.core_environment.clone();
        let contracts = &mut environment.scopes.get_mut("core").unwrap().contracts;
        contracts.get_mut("type.v1").unwrap()["spec"]["signIn"] = json!({"account":"/account"});
        let account_hook = json!({"kind":"Operation","metadata":{"name":"account-hook.v1"},
            "spec":{"outputSchema":{"type":"object","required":["account"],
                "properties":{"account":{"type":["string","null"]}}}}});
        contracts.insert("account-hook.v1".into(), account_hook.clone());
        let indexes = json!({"core":contracts});
        let instance = &mut t.state.objects[1];
        instance["type"] = json!(c.qualified("type.v1"));
        instance["held"] =
            json!({"environment":environment,"indexes":indexes,"contexts":{},"scope":"core"});
        let mut check = t.state.invocations.pop().unwrap();
        check["id"] = json!("k1");
        check["operation"] = json!(c.qualified("account-hook.v1"));
        check["plain"] = json!(false);
        check["contract"] = account_hook;
        check["held_target"] = crate::registration::publication_object(instance, &c).unwrap();
        t.state.invocations[0]["sign_in"] = json!({"phase":"account-pending","parts":[]});
        t.state.invocations.push(check);
        t.state.invocations.push(json!({
            "id":"r1","operation":c.qualified("retire.v1"),"state":"in-progress","object":"s1"
        }));
        let outcome = format!("{}outcome", c.meta_prefix);
        let result = json!({"_meta":{outcome:"succeeded"},"content":[],"structuredContent":{"account":null}});
        requests::complete(
            &mut t,
            &evidence(),
            "k1",
            1,
            requests::Completion::Result { result },
            &c,
        )
        .unwrap();
        assert_eq!(t.state.invocations[0]["state"], "failed");
        let diagnostics = t.reply["structuredContent"]["diagnostics"]
            .as_array()
            .unwrap();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0]["ruleId"], "PND-003");
        assert!(t.external_intents.is_empty());
        assert!(
            !t.state
                .invocations
                .iter()
                .any(|v| v["operation"] == c.qualified("sign-in-hook.v1"))
        );
    }

    #[test]
    fn sign_in_first_check_ignores_ascii_case() {
        use FirstCheck::{Expected, NoAccount, NotChecked, Other};
        for (outcome, outputs, check) in [
            (
                "succeeded",
                json!({"account":"PRIMARY-ACCOUNT"}),
                Expected("PRIMARY-ACCOUNT".into()),
            ),
            (
                "succeeded",
                json!({"account":"primary-account"}),
                Expected("primary-account".into()),
            ),
            ("succeeded", json!({"account":null}), NoAccount),
            ("succeeded", json!({"account":"other-account"}), Other),
            (
                "succeeded",
                json!({"account":"pr\u{0131}mary-account"}),
                Other,
            ),
            ("succeeded", json!({}), NotChecked),
            ("failed", json!({}), NotChecked),
            ("unknown", json!({"account":"primary-account"}), NotChecked),
        ] {
            assert_eq!(first_check(outcome, &outputs, "primary-account"), check);
        }
    }

    #[test]
    fn sign_in_success_answers_and_lifts_the_hold() {
        let (_, context) = crate::explain::tests::fixture();
        let time = "2026-11-12T12:00:00Z";
        let evidence = Evidence {
            time: time.into(),
            ids: vec!["first-action".into()],
            revisions: vec![],
            suffixes: vec![],
        };
        let mut allocation = Allocation {
            evidence: &evidence,
            cursor: 0,
        };
        let instance = json!({"id":"s1","name":"s1","kind":"Agent","owner":"f",
            "config":{"account":"primary-account"}});
        let mut state = State {
            objects: vec![
                json!({"id":"r","kind":"Interface","from":context.qualified("root")}),
                instance.clone(),
            ],
            records: vec![json!({"type":"invocation-ended","data":{"invocation":"c1"}})],
            ..State::default()
        };
        let action = pending::raise_sign_in(
            &mut state,
            &instance,
            "primary-account",
            "/account",
            pending::SignInRequest::Invocation {
                invocation: &json!({"id":"c1","operation":"example.org/sign-in-hold#work.v1"}),
                trigger: "lapsed-login",
            },
            &mut allocation,
        )
        .unwrap()
        .unwrap();
        assert_eq!(state.sign_in_holds["s1"], json!({"since":time}));
        let invocation = json!({"id":"g1","caller":"r"});
        let answered_at = "2026-11-12T12:00:01Z";
        let mut answered_state = state.clone();
        let mut expected = action.clone();
        expected["answer"] =
            json!({"value":"signed-in","actor":"r","invocation":"g1","at":answered_at});
        assert_eq!(
            answer_and_lift(&mut answered_state, "s1", &invocation, answered_at),
            (Some(expected.clone()), true)
        );
        assert_eq!(
            pending::unanswered_sign_in(&answered_state, &instance, answered_at),
            None
        );
        assert!(!answered_state.sign_in_holds.contains_key("s1"));
        let id = action["id"].as_str().unwrap();
        assert_eq!(answered_state.pending_actions[id]["raisedBy"], "c1");
        assert_eq!(answered_state.pending_actions[id]["action"], expected);
        let before_repeat = serde_json::to_value(&answered_state).unwrap();
        assert_eq!(
            answer_and_lift(
                &mut answered_state,
                "s1",
                &invocation,
                "2026-11-12T12:00:02Z"
            ),
            (None, false)
        );
        assert_eq!(serde_json::to_value(answered_state).unwrap(), before_repeat);

        let expiry = "2026-11-19T12:00:00Z";
        assert_eq!(action["expiry"], expiry);
        assert_eq!(
            answer_and_lift(&mut state, "s1", &invocation, expiry),
            (None, true)
        );
        let expired = pending::action_named(&state, id, Some(expiry), &context).unwrap();
        assert_eq!(expired["answer"], json!({"value":"expired","at":expiry}));
        assert!(!state.sign_in_holds.contains_key("s1"));
    }

    #[test]
    fn sign_in_undone_condition_and_hold() {
        let time = "2026-11-12T12:00:00Z";
        let later = "2026-11-12T12:00:01Z";
        let checked = "2026-11-12T12:00:02Z";
        let running = json!({"type":"Running","status":"True"});
        let mut state = State {
            objects: vec![
                json!({"kind":"Agent","id":"s1","status":{"conditions":[running]}}),
                json!({"kind":"Agent","id":"s2"}),
                json!({"kind":"Resource","id":"x1"}),
                json!({"kind":"Agent","id":"t1","spec":{}}),
            ],
            ..State::default()
        };
        set_sign_in_undone_false(&mut state.objects[0], "failed", &json!("h1"), time);
        assert_eq!(
            state.objects[0]["status"]["conditions"],
            json!([running,{
                "type":"SignInUndone","status":"False","reason":"SignOutFailed",
                "message":"The sign-out hook ended failed after setup's account check found an account other than the expected one, so that account may still be signed in. Requests to the instance other than the parts of sign-in.v1, of a setup and of its retirement report unavailable under PND-011 until an account check finds the expected account or none.",
                "lastTransitionTime":time,"source":"urn:uuid:h1"
            }])
        );
        let held = |state: &State, id: &str| request_is_held_for_sign_in_undone(state, &json!(id));
        assert_eq!([held(&state, "s1"), held(&state, "s2")], [true, false]);
        set_sign_in_undone_false(&mut state.objects[0], "unknown", &json!("h2"), later);
        let unknown = state.objects[0]["status"]["conditions"][1].clone();
        assert_eq!(unknown["reason"], "SignOutUnknown");
        assert_eq!(unknown["source"], "urn:uuid:h2");
        assert_eq!(unknown["lastTransitionTime"], time);
        assert_eq!(state.objects[0].get("past_conditions"), None);
        state.objects[2]["status"]["conditions"] = json!([unknown]);
        state.objects[3]["status"]["conditions"] = json!([unknown]);
        assert_eq!([held(&state, "x1"), held(&state, "t1")], [true, false]);
        assert_eq!(
            [clear_sign_in_undone(
                &mut state.objects[0],
                checked,
                "setup"
            )],
            [true]
        );
        assert_eq!(
            state.objects[0]["status"]["conditions"][1],
            json!({"type":"SignInUndone","status":"True","reason":"AccountChecked",
                "message":"An account check within setup found the expected account or none.",
                "lastTransitionTime":checked,"source":"urn:uuid:h2"})
        );
        assert_eq!(
            state.objects[0]["past_conditions"],
            json!([{"condition":unknown,"endedAt":checked}])
        );
        let cleared = state.objects[0].clone();
        assert_eq!(
            [
                held(&state, "s1"),
                clear_sign_in_undone(&mut state.objects[0], "2026-11-12T12:00:03Z", "setup"),
                clear_sign_in_undone(&mut state.objects[1], checked, "setup"),
            ],
            [false, false, false]
        );
        assert_eq!(state.objects[0], cleared);
        assert_eq!(state.objects[1].get("status"), None);
    }

    #[test]
    fn held_requests_by_family_and_model_work() {
        let held_agent = json!({"kind":"Agent","id":"s1"});
        let other_agent = json!({"kind":"Agent","id":"s2"});
        let resource = json!({"kind":"Resource","id":"x1"});
        let template = json!({"kind":"Agent","id":"t1","spec":{}});
        let hold = json!({"since":"2026-11-12T12:00:00Z"});
        let state = State {
            objects: vec![
                held_agent.clone(),
                other_agent.clone(),
                resource.clone(),
                template.clone(),
            ],
            sign_in_holds: [
                ("s1".into(), hold.clone()),
                ("x1".into(), hold.clone()),
                ("t1".into(), hold),
            ]
            .into(),
            ..State::default()
        };
        let model = json!({"spec":{"startsModelWork":true}});
        let plain = json!({"spec":{"effect":"A fixture effect."}});
        assert_eq!(
            [
                request_is_held(&state, &held_agent, &model),
                request_is_held(&state, &held_agent, &plain),
                request_is_held(&state, &resource, &plain),
                request_is_held(&state, &resource, &model),
                request_is_held(&state, &other_agent, &model),
                request_is_held(&state, &template, &model),
            ],
            [true, false, true, true, false, false]
        );
    }

    #[test]
    fn sign_in_login_refused_for_a_plain_hook() {
        let (_, context) = crate::explain::tests::fixture();
        let hook = context.qualified("sign-in-hook.v1");
        let route = json!({"tool":"group.sign-in.v1"});
        let names = ["browser".to_owned(), "device".to_owned()];
        let check = |item, login: Option<Value>| {
            let mut request = json!({
                "identity":{"kind":"instance","id":"r"},
                "tool":route["tool"],
                "arguments":{"instance":{"kind":"Agent","name":"signed"}},
                "clientCapabilities":{"elicitation":{"url":{}}}
            });
            if let Some(login) = login {
                request["signInLogin"] = login;
            }
            let instance = json!({
                "kind":"Agent","name":"signed",
                "resolved_implementations":{&hook:{"item":item}}
            });
            request_member_refusal(
                &route,
                &instance,
                &serde_json::from_value(request).unwrap(),
                &names,
                &context,
            )
            .map(|r| (r.reason, r.rule, r.diagnostics.unwrap()))
        };
        let plain = json!({"plain":true,"tools":{}});
        for (name, text) in [
            (
                "browser",
                r#"group.sign-in.v1 reports invalid under PND-010 for Agent/signed: signInLogin asks for the login "browser", but the instance's sign-in-hook.v1 is a plain hook, its implementation item marked plain and not listing sign-in-hook.v1 under keyed (PKG-020), which gets no login name (IFC-007); send the request again without signInLogin, and the hook chooses among the stated logins "browser", "device"."#,
            ),
            (
                "phone",
                r#"group.sign-in.v1 reports invalid under PND-010 for Agent/signed: signInLogin asks for the login "phone", which the instance's Type does not state; its stated logins are "browser", "device"."#,
            ),
        ] {
            assert_eq!(
                check(plain.clone(), Some(json!({"name":name}))),
                Some((
                    "invalid",
                    "PND-010".to_owned(),
                    json!([{"ruleId":"PND-010","level":"error","message":{"text":text}}])
                ))
            );
        }
        assert_eq!(check(plain, None), None);
        assert_eq!(
            check(
                json!({"plain":true,"tools":{},"keyed":[hook]}),
                Some(json!({"name":"browser"}))
            ),
            None
        );
        assert_eq!(
            check(json!({"command":["x"]}), Some(json!({"name":"device"}))),
            None
        );
    }

    #[test]
    fn sign_in_request_member_refusals() {
        let (_, context) = crate::explain::tests::fixture();
        let route = json!({
            "tool":"group.sign-in.v1",
            "operation":context.qualified("sign-in.v1"),
            "caller":"r",
            "arguments":{"instance":{"kind":"Agent","name":"agent-template"}}
        });
        let names = ["browser".to_owned(), "device".to_owned()];
        let url = json!({"elicitation":{"url":{}}});
        let returned = json!({
            "value":"http://127.0.0.1:49152/callback?code=not-a-code&state=not-a-state",
            "via":"paste"
        });
        let request = |members: Value| {
            let mut request = json!({
                "identity":{"kind":"instance","id":"r"},
                "tool":route["tool"],
                "arguments":route["arguments"]
            });
            request
                .as_object_mut()
                .unwrap()
                .extend(members.as_object().unwrap().clone());
            serde_json::from_value::<AccessRequest>(request).unwrap()
        };
        let assert_diagnostic = |actual: Rejection, rule: &str, text: &str| {
            assert_eq!(actual.reason, "invalid");
            assert_eq!(actual.rule, rule);
            let diagnostics = actual.diagnostics.unwrap();
            assert_eq!(
                diagnostics,
                json!([{"ruleId":rule,"level":"error","message":{"text":text}}])
            );
            diagnostics
        };
        let missing_text = r#"group.sign-in.v1 reports invalid under IFC-049 for Agent/agent-template: the request does not declare URL elicitation in its client capabilities, and sign-in.v1 relays a login only by URL elicitation; send it again with {"elicitation": {"url": {}}} in io.modelcontextprotocol/clientCapabilities."#;
        let return_text = format!(
            "group.sign-in.v1 reports invalid under PND-013 for Agent/agent-template: the request carries {}signInReturn on a first sign-in.v1 request, which takes no return; a return travels only on a retry of sign-in.v1 in the invocation its requestState binds.",
            context.meta_prefix
        );
        let unknown_login_text = |asked| {
            format!(
                r#"group.sign-in.v1 reports invalid under PND-010 for Agent/agent-template: signInLogin asks for the login "{asked}", which the instance's Type does not state; its stated logins are "browser", "device"."#
            )
        };
        let malformed_login_text = r#"group.sign-in.v1 reports invalid under PND-010 for Agent/agent-template: signInLogin must hold the name of one login the instance's Type states; its stated logins are "browser", "device"."#;
        let instance = &route["arguments"]["instance"];
        let check = |members: Value| {
            request_member_refusal(&route, instance, &request(members), &names, &context)
        };
        for (members, rule, text) in [
            (json!({}), "IFC-049", missing_text),
            (
                json!({"clientCapabilities":{"elicitation":{}}}),
                "IFC-049",
                missing_text,
            ),
            (
                json!({"clientCapabilities":{"elicitation":{"url":true}}}),
                "IFC-049",
                missing_text,
            ),
            (
                json!({"clientCapabilities":url,"signInReturn":returned}),
                "PND-013",
                &return_text,
            ),
            (
                json!({"clientCapabilities":url,"signInLogin":{"name":"phone"}}),
                "PND-010",
                &unknown_login_text("phone"),
            ),
            (
                json!({"clientCapabilities":url,"signInLogin":"browser"}),
                "PND-010",
                malformed_login_text,
            ),
            (
                json!({"clientCapabilities":url,"signInLogin":{"name":"BROWSER"}}),
                "PND-010",
                &unknown_login_text("BROWSER"),
            ),
            (
                json!({"signInReturn":returned,"signInLogin":{"name":"phone"}}),
                "IFC-049",
                missing_text,
            ),
            (
                json!({"clientCapabilities":url,"signInReturn":returned,"signInLogin":{"name":"phone"}}),
                "PND-013",
                &return_text,
            ),
        ] {
            let diagnostics = assert_diagnostic(check(members).unwrap(), rule, text).to_string();
            if rule == "PND-013" {
                assert!(!diagnostics.contains("not-a-code"));
                assert!(!diagnostics.contains("not-a-state"));
            }
        }
        for members in [
            json!({"clientCapabilities":url}),
            json!({"clientCapabilities":url,"signInReturn":null}),
            json!({"clientCapabilities":url,"signInLogin":{"name":"browser"}}),
            json!({"clientCapabilities":url,"signInLogin":{"name":"device"}}),
        ] {
            assert!(check(members).is_none());
        }
        let state = State {
            objects: vec![
                json!({"id":"r","kind":"Interface","from":context.qualified("root")}),
                json!({"id":"f","name":"f","kind":"Fleet"}),
                json!({
                    "id":"t","name":"agent-template","kind":"Agent","revision":"t1",
                    "metadata":{"name":"agent-template"},
                    "spec":{"type":"#signed-agent.v1"},"resolved_implementations":{},
                    "held":{},"packages":[]
                }),
            ],
            ..State::default()
        };
        let actual = preflight(
            &state,
            &route,
            &request(json!({"clientCapabilities":url,"signInLogin":{"name":"phone"}})),
            "2026-11-12T12:00:00Z",
            &context,
        )
        .unwrap()
        .unwrap();
        assert_diagnostic(
            actual,
            "PND-010",
            "group.sign-in.v1 reports invalid under PND-010 for Agent/agent-template: the instance's Type states no signIn, so it has no expected account to sign in to.",
        );
    }

    #[test]
    fn sign_in_refusals_before_dispatch() {
        let (_, context) = crate::explain::tests::fixture();
        let time = "2026-11-12T12:00:00Z";
        let agent = json!({"id":"s1","name":"signed","kind":"Agent","owner":"f"});
        let mut state = State {
            objects: vec![
                json!({"id":"r","kind":"Interface","from":context.qualified("root")}),
                json!({"id":"v","kind":"Interface"}),
                json!({"id":"w","kind":"Interface"}),
                json!({"id":"management","kind":"Resource","from":context.qualified("management")}),
                json!({"id":"f","name":"f","kind":"Fleet"}),
                agent.clone(),
                json!({"id":"s2","kind":"Agent","owner":"f"}),
                json!({"id":"scope","kind":"Conduit","consumer":"v","target":"f",
                    "open":true,"state":"ready","operations":[context.qualified("inspect.v1")]}),
            ],
            ..State::default()
        };
        let route = |caller| {
            json!({"tool":"group.sign-in.v1","caller":caller,
            "operation":context.qualified("sign-in.v1"),
            "arguments":{"instance":{"kind":"Agent","name":"signed"}}})
        };
        let request: AccessRequest = serde_json::from_value(json!({
            "identity":{"kind":"instance","id":"r"},
            "tool":"group.sign-in.v1",
            "arguments":{"instance":{"kind":"Agent","name":"signed"}},
            "clientCapabilities":{"elicitation":{"url":{}}}
        }))
        .unwrap();
        let assert_refusal = |actual: Option<Rejection>, reason: &str, cause: &str| {
            let actual = actual.unwrap();
            assert_eq!(actual.reason, reason);
            assert_eq!(actual.rule, "PND-010");
            assert_eq!(
                actual.diagnostics,
                Some(json!([{
                    "ruleId":"PND-010","level":"error",
                    "message":{"text":format!("group.sign-in.v1 reports {reason} under PND-010 for Agent/signed: {cause}")}
                }]))
            );
        };
        let no_sign_in =
            "the instance's Type states no signIn, so it has no expected account to sign in to.";
        let not_string = "the instance's expected account, at its Type's account pointer /account, is not a string.";
        assert_refusal(
            preflight(&state, &route("r"), &request, time, &context).unwrap(),
            "invalid",
            no_sign_in,
        );
        let through = |caller: &'static str, target: &'static str| {
            let mut route = route(caller);
            route["consumer"] = json!(caller);
            route["target"] = json!(target);
            route["arguments"]["expectedId"] = json!("stale");
            route
        };
        state
            .objects
            .push(json!({"id":"f2","name":"f2","kind":"Fleet"}));
        assert_refusal(
            preflight(&state, &through("v", "f"), &request, time, &context).unwrap(),
            "invalid",
            no_sign_in,
        );
        for (caller, target) in [("w", "f2"), ("v", "f2"), ("s2", "f")] {
            assert!(preflight(&state, &through(caller, target), &request, time, &context).is_err());
        }
        state.objects.extend([
            json!({"id":"p","name":"p","kind":"Pool"}),
            json!({"id":"x","name":"x","kind":"Resource","owner":"p"}),
        ]);
        let mut resource = through("v", "p");
        resource["arguments"]["instance"] = json!({"kind":"Resource","name":"x"});
        assert!(preflight(&state, &resource, &request, time, &context).is_err());
        let check = |state: &State, caller, expected| {
            refusal(
                state,
                &route(caller),
                &agent,
                time,
                expected,
                &request,
                &[],
                &context,
            )
            .unwrap()
        };
        let unset = || ExpectedAccount::NotString {
            pointer: "/account".into(),
        };
        let set = || ExpectedAccount::Set {
            pointer: "/account".into(),
            account: "primary-account",
        };
        assert_refusal(check(&state, "r", unset()), "invalid", not_string);
        assert!(check(&state, "r", set()).is_none());

        let in_progress = |op| {
            json!({"id":"earlier","operation":context.qualified(op),
            "object":"s1","state":"in-progress"})
        };
        let busy = |what| {
            format!(
                "the instance's earlier {what} has been dispatched and has not ended; send sign-in.v1 again once it ends."
            )
        };
        for (op, what) in [
            ("set-up.v1", "setup"),
            ("retire.v1", "retirement"),
            ("sign-in.v1", "sign-in.v1"),
        ] {
            state.invocations = vec![in_progress(op)];
            assert_refusal(check(&state, "r", set()), "conflict", &busy(what));
        }
        state.invocations = vec![in_progress("set-up.v1")];
        assert_refusal(
            check(&state, "r", ExpectedAccount::NoSignIn),
            "invalid",
            no_sign_in,
        );
        assert_refusal(check(&state, "r", unset()), "invalid", not_string);
        state.pending_actions.insert(
            "action-id".into(),
            json!({
                "action":{"id":"action-id","kind":"sign-in","subject":[{"id":"s1"}],
                    "evidence":[],"expiry":"2026-11-19T12:00:00Z"},
                "dispatched":null,"raisedBy":"call-id"
            }),
        );
        let setup_busy = busy("setup");
        let with_action = format!("{setup_busy} Its pending sign-in action is action-id.");
        for caller in ["r", "v"] {
            assert_refusal(check(&state, caller, set()), "conflict", &with_action);
        }
        assert_refusal(check(&state, "w", set()), "conflict", &setup_busy);
        state.pending_actions.get_mut("action-id").unwrap()["action"]["expiry"] = json!(time);
        assert_refusal(check(&state, "r", set()), "conflict", &setup_busy);
    }

    #[test]
    fn sign_in_fixed_port_refusal() {
        let logins = [
            json!({"name":"browser","return":"redirect",
                "redirect":{"host":"127.0.0.1","ports":[8400,8401],"path":"/callback"}}),
            json!({"name":"device","return":"device"}),
            json!({"name":"any-port","return":"redirect",
                "redirect":{"host":"127.0.0.1","ports":"any","path":"/cb"}}),
            json!({"name":"pasted","return":"page","page":"https://example.org/code"}),
            json!({"name":"second","return":"redirect",
                "redirect":{"host":"localhost","ports":[8402,8401],"path":"/other"}}),
        ];
        let all = BTreeSet::from([8400, 8401, 8402]);
        let browser = BTreeSet::from([8400, 8401]);
        let second = BTreeSet::from([8401, 8402]);
        let empty = BTreeSet::new();
        assert_eq!(may_start_fixed_ports(&logins, None), all);
        assert_eq!(may_start_fixed_ports(&logins, Some("browser")), browser);
        assert_eq!(may_start_fixed_ports(&logins, Some("second")), second);
        for name in ["device", "any-port", "pasted", "phone"] {
            assert_eq!(may_start_fixed_ports(&logins, Some(name)), empty);
        }
        assert_eq!(may_start_fixed_ports(&[], None), empty);
        let mut decimal_ports = logins[0].clone();
        decimal_ports["redirect"]["ports"] = json!([8400.0, 8401.0]);
        assert_eq!(may_start_fixed_ports(&[decimal_ports], None), browser);

        let (_, context) = crate::explain::tests::fixture();
        let time = "2026-11-12T12:00:00Z";
        let expiry = "2026-11-19T12:00:00Z";
        let agent = json!({"id":"s1","name":"signed","kind":"Agent","owner":"f"});
        let mut state = State {
            objects: vec![
                json!({"id":"r","kind":"Interface","from":context.qualified("root")}),
                json!({"id":"v","kind":"Interface"}),
                json!({"id":"w","kind":"Interface"}),
                json!({"id":"management","kind":"Resource","from":context.qualified("management")}),
                json!({"id":"f","name":"f","kind":"Fleet"}),
                agent.clone(),
                json!({"id":"s2","kind":"Agent","owner":"f"}),
                json!({"id":"scope","kind":"Conduit","consumer":"v","target":"f",
                    "open":true,"state":"ready","operations":[context.qualified("inspect.v1")]}),
            ],
            ..State::default()
        };
        state.pending_actions.insert(
            "action-id".into(),
            json!({
                "action":{"id":"action-id","kind":"sign-in","subject":[{"id":"s2"}],
                    "evidence":[],"expiry":expiry},
                "dispatched":null,"raisedBy":"call-id"
            }),
        );
        let check = |caller, time, own: &BTreeSet<u64>, others: &[(Value, BTreeSet<u64>)]| {
            let route = json!({"tool":"group.sign-in.v1","caller":caller});
            fixed_port_refusal(&state, &route, &agent, own, others, time, &context).unwrap()
        };
        let assert_unavailable = |actual: Option<Rejection>, ports, visible| {
            let actual = actual.unwrap();
            assert_eq!(actual.reason, "unavailable");
            assert_eq!(actual.rule, "PND-010");
            let mut text = format!(
                "group.sign-in.v1 reports unavailable under PND-010 for Agent/signed: another sign-in.v1 in progress may start a login on the fixed {ports}, which a login this request may start also states, and one fixed port serves one login at a time; send sign-in.v1 again once that sign-in ends."
            );
            if visible {
                text.push_str(" The other instance's pending sign-in action is action-id.");
            } else {
                assert!(
                    !actual
                        .diagnostics
                        .as_ref()
                        .unwrap()
                        .to_string()
                        .contains("action-id")
                );
            }
            assert_eq!(
                actual.diagnostics,
                Some(json!([{"ruleId":"PND-010","level":"error","message":{"text":text}}]))
            );
        };
        let others = [(json!("s2"), second.clone())];
        for caller in ["r", "v"] {
            assert_unavailable(check(caller, time, &browser, &others), "port 8401", true);
        }
        assert_unavailable(check("w", time, &browser, &others), "port 8401", false);
        assert_unavailable(check("r", expiry, &browser, &others), "port 8401", false);
        assert_unavailable(
            check(
                "w",
                time,
                &all,
                &[(json!("s1"), empty.clone()), (json!("s2"), second.clone())],
            ),
            "ports 8401, 8402",
            false,
        );
        assert_unavailable(
            check(
                "w",
                time,
                &browser,
                &[
                    (json!("s1"), BTreeSet::from([8400])),
                    (json!("s2"), second.clone()),
                ],
            ),
            "port 8400",
            false,
        );
        assert!(check("r", time, &BTreeSet::from([8400]), &others).is_none());
        assert!(check("r", time, &empty, &others).is_none());
        assert!(check("r", time, &browser, &[]).is_none());
    }
}
