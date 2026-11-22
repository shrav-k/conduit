use crate::{Allocation, Context, Result, State, Transition, access, decision, pending, sign_in};
use serde_json::{Value, json};

pub(crate) fn id(agent: &Value) -> Option<&Value> {
    if agent["kind"] != "Agent" || agent.get("spec").is_some() {
        return None;
    }
    let reference = agent["config"].pointer(agent["login_resource_place"].as_str()?)?;
    reference.get("id")
}

pub(crate) fn resource<'a>(state: &'a State, agent: &Value) -> Option<&'a Value> {
    access::object_by_id(state, id(agent)?)
}

pub(crate) fn label(object: &Value) -> String {
    format!(
        "{}/{} ({})",
        object["kind"].as_str().unwrap_or("Object"),
        object["name"].as_str().unwrap_or_default(),
        object["id"].as_str().unwrap_or_default()
    )
}

pub(crate) fn run_diagnostic(state: &State, agent: &Value) -> Option<Value> {
    let resource = resource(state, agent)?;
    let conditions = [
        (agent.get("runtime").is_some(), "the Agent names a runtime"),
        (
            resource.get("runtime").is_some(),
            "the Resource names a runtime",
        ),
        (
            access::id(&agent["placement"]) != access::id(&resource["placement"]),
            "the Agent and the Resource have different placements",
        ),
    ];
    let failures: Vec<_> = conditions
        .into_iter()
        .filter_map(|(failed, detail)| failed.then_some(detail))
        .collect();
    if failures.is_empty() {
        return None;
    }
    let resource = label(resource);
    let conditions = failures.join(", ");
    Some(
        json!({"ruleId":"PRV-029","level":"error","message":{"text":format!(
            "Only the engine's own runner passes the directory of login Resource {resource}, to a run on its node. Setup or hook run cannot proceed: {conditions}."
        )}}),
    )
}

pub(crate) fn binds_directory(agent: &Value, item: &Value) -> bool {
    agent["login_resource_place"].as_str().is_some_and(|place| {
        item["environment"]
            .as_object()
            .is_some_and(|variables| variables.values().any(|binding| binding == place))
    })
}

pub(crate) struct WorkHold {
    pub(crate) rule: &'static str,
    pub(crate) message: String,
    pub(crate) actions: Vec<Value>,
}

pub(crate) fn hold_work(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    agent: &Value,
    route: &Value,
    access_record: &str,
) -> Result<Option<WorkHold>> {
    let Some(resource) = resource(&t.state, agent).cloned() else {
        return Ok(None);
    };
    let held = sign_in::request_is_held(&t.state, &resource, &Value::Null);
    let undone = sign_in::request_is_held_for_sign_in_undone(&t.state, &resource["id"]);
    let time = &a.evidence.time;
    let running = pending::login_notice(&t.state, &resource)
        .filter(|entry| entry["action"]["answer"]["value"] != "acknowledged")
        .and_then(|notice| {
            t.state
                .invocations
                .iter()
                .find(|v| {
                    v["state"] == "in-progress"
                        && v["contract"]["spec"]["startsModelWork"] == true
                        && id(&v["held_target"]) == Some(&resource["id"])
                })
                .map(|running| (notice.clone(), running.clone()))
        });
    if !held && !undone && running.is_none() {
        return Ok(None);
    }
    let rule = if !held && undone {
        "PND-011"
    } else {
        "PND-005"
    };
    let resource_label = label(&resource);
    let mut causes = Vec::new();
    let mut actions = Vec::new();
    if held {
        if let sign_in::ExpectedAccount::Set { pointer, account } =
            sign_in::expected_account(&resource, c)?
        {
            let raised = pending::raise_sign_in(
                &mut t.state,
                &resource,
                account,
                &pointer,
                pending::SignInRequest::Held {
                    access_record_id: access_record,
                    operation: route["operation"].as_str().unwrap_or_default(),
                },
                a,
            )?;
            if let Some(action) = raised {
                cite_action(t, &resource, &action["id"], "raise-login-sign-in-action");
            }
        }
        causes.push(format!("The Agent's login Resource {resource_label} awaits sign-in to its expected account, so the request was not dispatched. Send it again once root's sign-in.v1 for that Resource has signed it in."));
    }
    if undone {
        let prefix = if rule == "PND-011" {
            ""
        } else {
            "Under PND-011, "
        };
        causes.push(format!("{prefix}SignInUndone is False for the Agent's login Resource {resource_label}, so the request was not dispatched. Send it again once an account check within sign-in.v1 or setup finds the expected account or none."));
    }
    if held || undone {
        actions.extend(
            pending::unanswered_sign_in(&t.state, &resource, time)
                .map(|action| action["id"].clone()),
        );
    }
    if let Some((entry, running)) = running {
        let notice = if let Some(id) = pending::unanswered_login_notice(&t.state, &resource, time) {
            id.clone()
        } else {
            let id = pending::store_login_notice(&mut t.state, &resource, entry, a)?;
            cite_action(t, &resource, &id, "renew-login-notice");
            id
        };
        let mut call = "a call of an Agent of it".to_owned();
        if let Some(agent) = access::object_by_id(&t.state, &running["target"])
            && access::may_inspect(&t.state, &route["caller"], agent, c)?
        {
            call = format!(
                "invocation {} of Agent/{}",
                running["id"].as_str().unwrap_or_default(),
                agent["name"].as_str().unwrap_or_default()
            );
        }
        let notice_label = notice.as_str().unwrap_or_default();
        let prefix = if rule == "PND-005" {
            "A call"
        } else {
            "Under PND-005, a call"
        };
        causes.push(format!("{prefix} on the login Resource {resource_label} reported its login lapsed, so its Agents' calls that start model work run one at a time, and {call} is in progress. The request was not dispatched. Send it again once that call ends. Root lifts this by answering notice {notice_label} acknowledged."));
        actions.push(notice);
    }
    let tool = route["tool"].as_str().ok_or("held request has no tool")?;
    t.decisions.push(decision(
        "hold-login-resource-work",
        &["PND-005", "PND-011", "OPS-023", "IFC-025"],
        &[],
        json!({"instance":resource["id"]}),
        vec![],
    ));
    Ok(Some(WorkHold {
        rule,
        message: format!(
            "{tool} reports unavailable under {rule}: {}",
            causes.join(" ")
        ),
        actions,
    }))
}

pub(crate) fn report_lapse(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    agent: &Value,
    resource: &Value,
    invocation: &Value,
    trigger: &sign_in::Trigger,
) -> Result<(String, Vec<Value>)> {
    let resource_label = label(resource);
    let prefix = match trigger {
        sign_in::Trigger::LapsedLogin => "The implementation reports a missing or lapsed login",
        sign_in::Trigger::UrlElicitation => {
            "The implementation asked for a URL elicitation outside sign-in.v1, which reports a missing or lapsed login"
        }
    };
    let sign_in::ExpectedAccount::Set { pointer, account } =
        sign_in::expected_account(resource, c)?
    else {
        return Ok((
            format!(
                "{prefix}, but the expected account of the Agent's login Resource {resource_label} is not set, so no sign-in action is raised."
            ),
            vec![],
        ));
    };
    if let Some(action) = pending::raise_sign_in(
        &mut t.state,
        resource,
        account,
        &pointer,
        pending::SignInRequest::LoginLapse {
            invocation,
            trigger: trigger.label(),
        },
        a,
    )? {
        cite_action(t, resource, &action["id"], "raise-login-sign-in-action");
    }
    let time = &a.evidence.time;
    let mut actions: Vec<_> = pending::unanswered_sign_in(&t.state, resource, time)
        .map(|action| action["id"].clone())
        .into_iter()
        .collect();
    let mut notice = pending::unanswered_login_notice(&t.state, resource, time).cloned();
    if notice.is_none()
        && invocation["contract"]["spec"]["startsModelWork"] == true
        && resource["state"] != "retired"
    {
        let new = pending::login_notice_for_lapse(&mut t.state, resource, agent, invocation, a)?;
        cite_action(t, resource, &new, "raise-login-notice");
        notice = Some(new);
    }
    let mut cause = format!(
        "{prefix}, for the Agent's login Resource {resource_label}. The report places no Agent on hold and may be a refresh collision while the login is good. The Agent may send its request again. If the login is gone, root sends sign-in.v1 for {resource_label} to sign in all its Agents."
    );
    if !actions.is_empty() {
        cause.push_str(" The Resource's sign-in action is pending.");
    }
    if let Some(notice) = notice {
        cause.push_str(&format!(" Until root answers notice {} acknowledged, the login takes one call that starts model work at a time.", notice.as_str().unwrap_or_default()));
        actions.push(notice);
    }
    Ok((cause, actions))
}

fn cite_action(t: &mut Transition, resource: &Value, id: &Value, action: &str) {
    t.decisions.push(decision(
        action,
        &["PND-001", "PND-005", "PND-012"],
        &["DL19"],
        json!({"action":id,"instance":resource["id"]}),
        vec![],
    ));
}
