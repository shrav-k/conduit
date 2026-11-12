//! IFC-012/IFC-033/IFC-038: BASE 1091a71's independent publication computation.
//! Deliberately retains whole-state copies and independent normalization/lookup.
use crate::{Context, Decision, Result, State, access::id, decision, schemas};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

struct PublicationState {
    state: State,
    unnormalizable: BTreeMap<usize, String>,
}

impl PublicationState {
    fn find(&self, predicate: impl Fn(&Value) -> bool) -> Result<Option<&Value>> {
        // IFC-012: an unread object's normalization failure cannot affect publication.
        let Some((index, object)) = self
            .state
            .objects
            .iter()
            .enumerate()
            .find(|(_, o)| predicate(o))
        else {
            return Ok(None);
        };
        match self.unnormalizable.get(&index) {
            Some(error) => Err(error.clone()),
            None => Ok(Some(object)),
        }
    }
}
fn open(path: &Value) -> bool {
    // IFC-012: BASE's ready, open Conduit predicate.
    path["kind"] == "Conduit" && path["open"] == true && path["state"] == "ready"
}
fn values<'a>(v: &'a Value, key: &str) -> impl Iterator<Item = &'a Value> {
    // IFC-012: preserve BASE's operation order.
    v.get(key).and_then(Value::as_array).into_iter().flatten()
}
fn publication_state(state: &State, context: &Context) -> PublicationState {
    // IFC-012/IFC-033: retain BASE's eager, independent whole-state normalization,
    // marking failed objects so only a publication that reads one returns its error.
    let mut view = state.clone();
    let mut unnormalizable = BTreeMap::new();
    for (index, o) in view.objects.iter_mut().enumerate() {
        if o.get("spec").is_some() {
            continue;
        }
        if o["kind"] == "Conduit" {
            for key in ["consumer", "target"] {
                o[key] = id(&o[key]).clone();
            }
            o["local_name"] = o
                .get("local_name")
                .cloned()
                .or_else(|| o["nested"].get("name").cloned())
                .unwrap_or_else(|| o["name"].clone());
        } else {
            match crate::registration::publication_object(o, context) {
                Ok(normalized) => *o = normalized,
                Err(error) => {
                    unnormalizable.insert(index, error);
                }
            }
        }
    }
    PublicationState {
        state: view,
        unnormalizable,
    }
}
fn tools(state: &State, instance: &str, context: &Context) -> Result<Vec<Value>> {
    // IFC-012/IFC-033: BASE publishes every operation with its own state copy.
    let mut tools = vec![];
    for path in state
        .objects
        .iter()
        .filter(|o| open(o) && id(&o["consumer"]) == &json!(instance))
    {
        for op in values(path, "operations") {
            let (published, _) = publish(
                state,
                instance,
                path["id"].as_str().ok_or("missing Conduit ID")?,
                op.as_str().ok_or("missing operation name")?,
                context,
            )?;
            if let Some(tool) = published.get("tool").filter(|v| !v.is_null()) {
                tools.push(tool.clone());
            }
        }
    }
    tools.sort_by_key(|v| v["name"].as_str().unwrap_or("").to_owned());
    Ok(tools)
}
pub fn publish(
    state: &State,
    consumer: &str,
    conduit: &str,
    operation: &str,
    context: &Context,
) -> Result<(Value, Decision)> {
    // IFC-012/IFC-033: BASE's independent Conduit and target lookup, returning
    // normalization errors only for the objects this publication actually reads.
    let view = publication_state(state, context);
    let path = view
        .find(|o| o["id"] == conduit)?
        .ok_or("unknown Conduit")?;
    if path["kind"] != "Conduit"
        || path["consumer"] != consumer
        || path["open"] != true
        || path["state"] != "ready"
        || !path["operations"]
            .as_array()
            .is_some_and(|ops| ops.contains(&json!(operation)))
    {
        return Ok((
            json!({"tool":null,"reason":"refused"}),
            decision(
                "omit-unavailable-tool",
                &["IFC-012", "ACC-001"],
                &[],
                json!({"conduit":conduit}),
                vec![],
            ),
        ));
    }
    let target = view
        .find(|o| o["id"] == path["target"])?
        .ok_or("missing target")?;
    schemas::publish_normalized(path, target, operation, context)
}

// IFC-012/IFC-033/IFC-038: compare every consumer on both states, including errors.
// Reuse each independent reference list for the reference notification computation.
pub(crate) fn check(before: &State, after: &State, context: &Context) -> Result<()> {
    let consumers: BTreeSet<_> = before
        .objects
        .iter()
        .chain(&after.objects)
        .filter(|o| o["kind"] == "Conduit")
        .filter_map(|o| id(&o["consumer"]).as_str())
        .collect();
    let before_view = crate::access::PublicationView::new(before, context);
    let after_view = crate::access::PublicationView::new(after, context);
    let mut reference_changes: std::result::Result<Vec<Value>, (String, String)> = Ok(vec![]);
    for consumer in &consumers {
        let old = tools(before, consumer, context);
        let new = tools(after, consumer, context);
        for (side, view, expected) in [("before", &before_view, &old), ("after", &after_view, &new)]
        {
            let actual = crate::access::tools(view, consumer, context);
            if &actual != expected {
                return Err(format!(
                    "differential tool list differs for consumer {consumer} ({side}; actual: {:?}; reference: {:?})",
                    actual.as_ref().map(Vec::len),
                    expected.as_ref().map(Vec::len),
                ));
            }
        }
        // BASE's ordered consumer comparison, preserving its first error.
        if let Ok(changes) = &mut reference_changes {
            match (old, new) {
                (Ok(old), Ok(new)) => {
                    if old != new {
                        changes.push(json!({"consumer":consumer,"method":"notifications/tools/list_changed"}));
                    }
                }
                (Err(error), _) | (_, Err(error)) => {
                    reference_changes = Err(((*consumer).to_owned(), error));
                }
            }
        }
    }
    let actual = crate::access::tool_list_changes_attributed(before, after, context);
    check_notifications(&consumers, actual, reference_changes)
}

fn check_notifications(
    consumers: &BTreeSet<&str>,
    actual: std::result::Result<Vec<Value>, (String, String)>,
    reference_changes: std::result::Result<Vec<Value>, (String, String)>,
) -> Result<()> {
    // IFC-038: compare both notification lists and attributed errors.
    if actual != reference_changes {
        // IFC-038: the first failing publication retains its own consumer, not the first ID.
        let consumer = match (&actual, &reference_changes) {
            (Err((consumer, _)), _) | (_, Err((consumer, _))) => consumer.as_str(),
            (Ok(actual), Ok(expected)) => consumers
                .iter()
                .find(|consumer| {
                    actual
                        .iter()
                        .filter(|v| v["consumer"] == **consumer)
                        .collect::<Vec<_>>()
                        != expected
                            .iter()
                            .filter(|v| v["consumer"] == **consumer)
                            .collect::<Vec<_>>()
                })
                .copied()
                .unwrap_or("<none>"),
        };
        return Err(format!(
            "differential notifications differ for consumer {consumer}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn publication_fixture() -> (Context, State) {
        // IFC-033: a real tool definition exercises independent target normalization.
        let context = Context {
            builtin: None,
            core_package: "test.core".into(),
            meta_prefix: "test/".into(),
            templates: BTreeMap::new(),
            record_source: "urn:test".into(),
            core_environment: schemas::Environment {
                scopes: BTreeMap::from([(
                    "core".into(),
                    schemas::Scope {
                        address: "test.core".into(),
                        version: None,
                        manifest: BTreeMap::new(),
                        contracts: BTreeMap::from([
                            (
                                "resource.v1".into(),
                                json!({"kind":"Type","metadata":{"name":"resource.v1"},"spec":{}}),
                            ),
                            (
                                "read.v1".into(),
                                json!({"kind":"Operation","metadata":{"name":"read.v1"},"spec":{}}),
                            ),
                        ]),
                        exports: None,
                    },
                )]),
            },
        };
        let state = State {
            objects: vec![
                json!({"kind":"Resource","id":"target","name":"target","type":"test.core#resource.v1"}),
                json!({"kind":"Conduit","id":"path","name":"read","consumer":{"id":"consumer"},
                    "target":{"id":"target"},"state":"ready","open":true,"operations":["test.core#read.v1"]}),
            ],
            ..State::default()
        };
        (context, state)
    }

    #[test]
    fn unrelated_unreadable_resource_preserves_tool_list() {
        // IFC-012: another Resource's unreadable held value cannot poison this list.
        let (context, before) = publication_fixture();
        let expected = tools(&before, "consumer", &context).unwrap();
        assert_eq!(expected.len(), 1);
        let mut after = before.clone();
        after
            .objects
            .push(json!({"kind":"Resource","id":"unrelated",
            "held":"unreadable; publication must not inspect this"}));
        let reference = publication_state(&after, &context);
        assert_eq!(reference.unnormalizable.len(), 1);
        assert_eq!(tools(&after, "consumer", &context), Ok(expected.clone()));
        let view = crate::access::PublicationView::new(&after, &context);
        assert_eq!(
            crate::access::tools(&view, "consumer", &context),
            Ok(expected)
        );
        check(&before, &after, &context).unwrap();
    }

    #[test]
    fn targeted_unreadable_resource_fails_both_tool_lists() {
        // IFC-012/IFC-033: reading the marked target returns its original error on both sides.
        let (context, mut state) = publication_fixture();
        state.objects[0]["held"] = json!("unreadable; publication reads this");
        let expected =
            crate::registration::publication_object(&state.objects[0], &context).unwrap_err();
        assert_eq!(tools(&state, "consumer", &context), Err(expected.clone()));
        let view = crate::access::PublicationView::new(&state, &context);
        assert_eq!(
            crate::access::tools(&view, "consumer", &context),
            Err(expected)
        );
        check(&state, &state, &context).unwrap();
    }

    #[test]
    fn notification_mismatch_names_failing_consumer() {
        // IFC-038: either side's error identifies the failing consumer, even after another ID.
        let consumers = BTreeSet::from(["a-first", "b-failing"]);
        for (actual, expected) in [
            (
                Ok(vec![]),
                Err(("b-failing".into(), "publication failed".into())),
            ),
            (
                Err(("b-failing".into(), "publication failed".into())),
                Ok(vec![]),
            ),
            (
                Err(("b-failing".into(), "different error".into())),
                Err(("b-failing".into(), "publication failed".into())),
            ),
        ] {
            assert_eq!(
                check_notifications(&consumers, actual, expected),
                Err("differential notifications differ for consumer b-failing".into())
            );
        }
    }
}
