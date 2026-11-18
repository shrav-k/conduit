//! PND-005 lapse signals and holds, PND-010 checks, and PND-011 sign-out conditions.
use crate::{Context, Result, State, access, explain::Rejection, inventory, pending};
use serde_json::{Value, json};

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

pub(crate) fn clear_sign_in_undone(object: &mut Value, time: &str) -> bool {
    let Some(i) = undone_condition(object) else {
        return false;
    };
    let old = &object["status"]["conditions"][i];
    if old["status"] != "False" {
        return false;
    }
    let condition = json!({
        "type":"SignInUndone","status":"True","reason":"AccountChecked",
        "message":"An account check within setup found the expected account or none.",
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
    time: &str,
    context: &Context,
) -> Result<Option<Rejection>> {
    if route["operation"] != context.qualified("sign-in.v1") {
        return Ok(None);
    }
    let Some(instance) = access::object(state, &route["arguments"]["instance"]) else {
        return Ok(None);
    };
    // PND-010, OPS-023: only root, or an Interface on its own behalf through an Agent's
    // owning Fleet, learns more than the stub's unavailable; a Resource stays root's.
    let root = access::object_by_id(state, &route["caller"])
        .is_some_and(|c| c["from"] == context.qualified("root") && c.get("spec").is_none());
    let owner = instance["kind"] == "Agent"
        && access::object(state, &instance["owner"]).is_some_and(|o| o["id"] == route["target"]);
    if !root
        && !(owner && pending::own_interface(state, &route["caller"], &route["consumer"]).is_some())
    {
        return Ok(None);
    }
    refusal(
        state,
        route,
        instance,
        time,
        expected_account(instance, context)?,
        context,
    )
}

fn refusal(
    state: &State,
    route: &Value,
    instance: &Value,
    time: &str,
    expected: ExpectedAccount<'_>,
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
        return Ok(Some(rejection("invalid", tool, &invalid)));
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
    if let Some(action) = pending::unanswered_sign_in(state, instance, time)
        && let Some(holder) = access::object_by_id(state, &route["caller"])
        && pending::raised_view(action, holder, state, time, context)?.is_some()
        && let Some(id) = action["id"].as_str()
    {
        text.push_str(&format!(" Its pending sign-in action is {id}."));
    }
    Ok(Some(rejection("conflict", tool, &text)))
}

fn rejection(reason: &'static str, tool: &str, cause: &str) -> Rejection {
    let mut rejection = Rejection::new(reason, "PND-010");
    rejection.diagnostics = Some(json!([{
        "ruleId":"PND-010", "level":"error",
        "message":{"text":format!("{tool} reports {reason} under PND-010: {cause}")}
    }]));
    rejection
}

#[cfg(test)]
mod tests {
    use super::*;

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
            [clear_sign_in_undone(&mut state.objects[0], checked)],
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
                clear_sign_in_undone(&mut state.objects[0], "2026-11-12T12:00:03Z"),
                clear_sign_in_undone(&mut state.objects[1], checked),
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
        let assert_refusal = |actual: Option<Rejection>, reason: &str, cause: &str| {
            let actual = actual.unwrap();
            assert_eq!(actual.reason, reason);
            assert_eq!(actual.rule, "PND-010");
            assert_eq!(
                actual.diagnostics,
                Some(json!([{
                    "ruleId":"PND-010","level":"error",
                    "message":{"text":format!("group.sign-in.v1 reports {reason} under PND-010: {cause}")}
                }]))
            );
        };
        let no_sign_in =
            "the instance's Type states no signIn, so it has no expected account to sign in to.";
        let not_string = "the instance's expected account, at its Type's account pointer /account, is not a string.";
        assert_refusal(
            preflight(&state, &route("r"), time, &context).unwrap(),
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
            preflight(&state, &through("v", "f"), time, &context).unwrap(),
            "invalid",
            no_sign_in,
        );
        for (caller, target) in [("w", "f2"), ("v", "f2"), ("s2", "f")] {
            assert!(
                preflight(&state, &through(caller, target), time, &context)
                    .unwrap()
                    .is_none()
            );
        }
        state.objects.extend([
            json!({"id":"p","name":"p","kind":"Pool"}),
            json!({"id":"x","name":"x","kind":"Resource","owner":"p"}),
        ]);
        let mut resource = through("v", "p");
        resource["arguments"]["instance"] = json!({"kind":"Resource","name":"x"});
        assert!(
            preflight(&state, &resource, time, &context)
                .unwrap()
                .is_none()
        );
        let check = |state: &State, caller, expected| {
            refusal(state, &route(caller), &agent, time, expected, &context).unwrap()
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
}
