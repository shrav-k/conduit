//! PND-005 lapse signals and holds, PND-010 checks, and PND-011 sign-out conditions.
use crate::{
    Allocation, Context, Result, State, Transition,
    access::{self, AccessRequest},
    decision,
    explain::{self, Rejection},
    inventory, pending, requests,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;

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

pub(crate) fn begin(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    let reference = &t.state.invocations[i]["arguments"]["instance"];
    let Some(instance) = access::object(&t.state, reference).cloned() else {
        return end_failed(
            t,
            a,
            c,
            i,
            "PND-010",
            "The request's instance names no object, so sign-in.v1 has no account to check and signs nothing in.",
        );
    };
    if !matches!(expected_account(&instance, c)?, ExpectedAccount::Set { .. }) {
        return end_failed(t, a, c, i, "PND-010", ACCOUNT_NOT_STRING);
    }
    if instance["state"] == "retired" {
        return end_failed(
            t,
            a,
            c,
            i,
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
    )? {
        return end_failed(
            t,
            a,
            c,
            i,
            "PND-011",
            "No implementation of account-hook.v1 is resolved for the instance, so its account cannot be checked and sign-in.v1 signs nothing in.",
        );
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

pub(crate) fn part_answered(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    part: &Value,
    outcome: &str,
) -> Result<()> {
    let invocation = &mut t.state.invocations[i];
    if invocation["sign_in"]["phase"] != "account-pending" {
        return Ok(());
    }
    invocation["sign_in"]["parts"]
        .as_array_mut()
        .unwrap()
        .push(json!({"name":"account","outcome":outcome,"invocation":part["id"]}));
    let Some(object) = t
        .state
        .objects
        .iter()
        .position(|o| o["id"] == invocation["object"])
    else {
        return end_failed(t, a, c, i, "PND-003", RETIRED_DURING_CHECK);
    };
    let instance = t.state.objects[object].clone();
    let ExpectedAccount::Set { pointer, account } = expected_account(&instance, c)? else {
        return end_failed(t, a, c, i, "PND-010", ACCOUNT_NOT_STRING);
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
        return end_failed(t, a, c, i, "PND-003", RETIRED_DURING_CHECK);
    }
    let (found, cause) = match check {
        FirstCheck::Expected(found) => return signed_in(t, a, c, i, &found),
        FirstCheck::NotChecked if matches!(outcome, "failed" | "unknown") => {
            t.decisions.push(decision(
                "end-sign-in-before-relay",
                &["PND-010", "PND-011"],
                &[],
                json!({"invocation":t.state.invocations[i]["id"],"found":"none","check":outcome}),
                vec![],
            ));
            return end_failed(
                t,
                a,
                c,
                i,
                "PND-010",
                &format!(
                    "The account check before sign-in-hook.v1 ended {outcome}, which counts as no account (PND-010, PND-011); sign-in.v1 would go on to start a login through sign-in-hook.v1 and relay it under IFC-049, which this build does not yet do (order 048a-5e), so it ends failed and nothing was signed in."
                ),
            );
        }
        FirstCheck::NotChecked => {
            return end_failed(
                t,
                a,
                c,
                i,
                "PND-011",
                &format!(
                    "The account check before sign-in-hook.v1 ended {outcome}, so the account is not known and sign-in.v1 completes nothing; send sign-in.v1 again."
                ),
            );
        }
        FirstCheck::NoAccount => (
            "none",
            "The account check before sign-in-hook.v1 found no account; sign-in.v1 would go on to start a login through sign-in-hook.v1 and relay it under IFC-049 (PND-010), which this build does not yet do (order 048a-5e), so it ends failed and nothing was signed in.",
        ),
        FirstCheck::Other => {
            let retired_by = c.qualified("retire.v1");
            let retiring = t.state.invocations.iter().any(|v| {
                v["operation"] == retired_by
                    && v["state"] == "in-progress"
                    && v["object"] == instance["id"]
            });
            if !retiring {
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
            (
                "other",
                "The account check before sign-in-hook.v1 found an account other than the expected one; sign-in.v1 would go on to start a login through sign-in-hook.v1 and relay it under IFC-049 (PND-010), which this build does not yet do (order 048a-5e), so it ends failed, that account stays as it is, and nothing was signed in.",
            )
        }
    };
    t.decisions.push(decision(
        "end-sign-in-before-relay",
        &["PND-010", "PND-011"],
        &[],
        json!({"invocation":t.state.invocations[i]["id"],"found":found}),
        vec![],
    ));
    end_failed(t, a, c, i, "PND-010", cause)
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

pub(crate) fn signed_in(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    found: &str,
) -> Result<()> {
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

fn end_failed(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    rule: &str,
    cause: &str,
) -> Result<()> {
    let diagnostic = explain::outcome_diagnostic(&t.state.invocations[i], "failed", rule, cause)?;
    requests::finish(
        t,
        a,
        c,
        i,
        "failed",
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
