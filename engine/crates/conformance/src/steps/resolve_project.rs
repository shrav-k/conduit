use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let base = step.base;
    let case = step.case;
    let input = step.input;
    let kind = step.kind;
    let state = step.state;
    let context = step.context;
    let spec = step.spec;
    let registration = packages::Registration {
        evidence: &input["evidence"],
        request: &input["request"],
        order: usize::from(input["reverse"] == true),
        framed: false,
    };
    let next: State;
    let actual = {
        let mut t = packages::run(
            base,
            input["file"].as_str().ok_or("missing context fixture")?,
            &case.files,
            state,
            spec,
            context,
            (kind != "resolve-project").then_some(registration),
        )?;
        if kind == "register-project"
            && let Some(alias) = input["equivalent"].as_str()
        {
            let framed = packages::run(
                base,
                alias,
                &case.files,
                state,
                spec,
                context,
                Some(packages::Registration {
                    framed: true,
                    ..registration
                }),
            )?;
            let operation = format!("{}#register.v1", context.core_package);
            let original = comparable_registration(&t, &operation)?;
            let other = comparable_registration(&framed, &operation)?;
            for ((sectioned, framed), place) in original
                .into_iter()
                .zip(other)
                .zip(["/reply", "/state", "/records"])
            {
                if let Some(pointer) = difference(&sectioned, &framed, place) {
                    return Err(format!(
                        "SCT-015: equivalent registration differs at {pointer}"
                    ));
                }
            }
        }
        if kind == "registration-orders" {
            let original = serde_json::to_value(&t).map_err(|e| e.to_string())?;
            for order in [1, 2] {
                let other = packages::run(
                    base,
                    input["file"].as_str().unwrap(),
                    &case.files,
                    state,
                    spec,
                    context,
                    Some(packages::Registration {
                        order,
                        ..registration
                    }),
                )?;
                if serde_json::to_value(other).map_err(|e| e.to_string())? != original {
                    return Err("SUB-001: registration differs by document/file order".into());
                }
            }
            t.reply["orders_compared"] = json!(3);
        }
        next = t.state.clone();
        serde_json::to_value(t).map_err(|e| e.to_string())?
    };
    Ok((actual, next))
}

fn comparable_registration(
    transition: &conduit_core::Transition,
    operation: &str,
) -> Result<[Value; 3]> {
    let mut reply = transition.reply.clone();
    diagnostic_identity(reply.get_mut("diagnostics"));
    let mut state = serde_json::to_value(&transition.state).map_err(|e| e.to_string())?;
    if let Some(invocations) = state["invocations"].as_array_mut() {
        for invocation in invocations
            .iter_mut()
            .filter(|v| v["boundary"] == "register-project")
        {
            diagnostic_identity(invocation.pointer_mut("/retained/outputs/diagnostics"));
        }
    }
    let mut records = json!(transition.records);
    for series in [&mut state["records"], &mut records] {
        if let Some(items) = series.as_array_mut() {
            for record in items
                .iter_mut()
                .filter(|v| v["data"]["operation"] == operation)
            {
                diagnostic_identity(record.pointer_mut("/data/outputs/diagnostics"));
            }
        }
    }
    Ok([reply, state, records])
}

fn diagnostic_identity(value: Option<&mut Value>) {
    if let Some(diagnostics) = value.and_then(Value::as_array_mut) {
        for diagnostic in &mut *diagnostics {
            *diagnostic = json!({"ruleId": diagnostic["ruleId"], "level": diagnostic["level"]});
        }
        diagnostics.sort_by(|a, b| {
            (a["ruleId"].as_str(), a["level"].as_str())
                .cmp(&(b["ruleId"].as_str(), b["level"].as_str()))
        });
    }
}

fn difference(left: &Value, right: &Value, pointer: &str) -> Option<String> {
    match (left, right) {
        (Value::Object(a), Value::Object(b)) => a
            .keys()
            .chain(b.keys())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .find_map(|key| {
                let place = format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"));
                match (a.get(key), b.get(key)) {
                    (Some(a), Some(b)) => difference(a, b, &place),
                    _ => Some(place),
                }
            }),
        (Value::Array(a), Value::Array(b)) => a
            .iter()
            .zip(b)
            .enumerate()
            .find_map(|(index, (a, b))| difference(a, b, &format!("{pointer}/{index}")))
            .or_else(|| {
                (a.len() != b.len()).then(|| format!("{pointer}/{}", a.len().min(b.len())))
            }),
        _ => (left != right).then(|| pointer.into()),
    }
}
