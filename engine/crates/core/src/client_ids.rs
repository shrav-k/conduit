//! CTR-013: compare a proposed Type's login client IDs with held Types.
use crate::explain::Rejection;
use crate::packages::Snapshot;
use crate::{State, Transition, decision};
use serde_json::{Value, json};
use std::collections::BTreeSet;

fn client_ids<'a>(
    documents: impl IntoIterator<Item = &'a Value>,
) -> impl Iterator<Item = (&'a str, &'a str)> {
    documents
        .into_iter()
        .filter(|doc| doc["kind"] == "Type")
        .flat_map(|doc| {
            doc["metadata"]["name"]
                .as_str()
                .into_iter()
                .flat_map(move |name| {
                    doc["spec"]["signIn"]["logins"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(move |login| Some((name, login["clientId"].as_str()?)))
                })
        })
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Sharing<'a> {
    id: &'a str,
    proposed_name: &'a str,
    held_address: &'a str,
    held_name: &'a str,
    held_digest: &'a str,
    place: String,
}

fn share_with<'a>(
    found: &mut BTreeSet<Sharing<'a>>,
    proposed_address: &str,
    proposed: &[(&'a str, &'a str)],
    held_address: &'a str,
    held_digest: &'a str,
    documents: impl IntoIterator<Item = &'a Value>,
    place: &str,
) {
    if proposed_address == held_address {
        return;
    }
    for (held_name, held_id) in client_ids(documents) {
        for &(name, id) in proposed.iter().filter(|(_, id)| *id == held_id) {
            found.insert(Sharing {
                id,
                proposed_name: name,
                held_address,
                held_name,
                held_digest,
                place: place.into(),
            });
        }
    }
}

fn sharing<'a>(
    address: &str,
    docs: impl IntoIterator<Item = &'a Value>,
    state: &'a State,
    installed: &'a [Snapshot],
) -> BTreeSet<Sharing<'a>> {
    let proposed: Vec<_> = client_ids(docs).collect();
    let mut found = BTreeSet::new();
    if proposed.is_empty() {
        return found;
    }
    for package in installed {
        share_with(
            &mut found,
            address,
            &proposed,
            &package.address,
            &package.digest,
            &package.project.documents,
            &format!("installed package {} {}", package.address, package.version),
        );
    }
    let mut seen = BTreeSet::new();
    for object in &state.objects {
        let project = &object["held"]["contexts"]["project"];
        let Some(held_address) = project["address"].as_str() else {
            continue;
        };
        let Some(digest) = project["digest"].as_str() else {
            continue;
        };
        if held_address == address
            || !object["packages"].as_array().is_some_and(|packages| {
                packages.iter().any(|package| {
                    package["address"] == held_address && package["digest"] == digest
                })
            })
            || !seen.insert((held_address, digest))
        {
            continue;
        }
        let documents = object["held"]["indexes"]["project"]
            .as_object()
            .into_iter()
            .flat_map(|index| index.values());
        share_with(
            &mut found,
            address,
            &proposed,
            held_address,
            digest,
            documents,
            "a submitted project held by a template or instance",
        );
    }
    found
}

fn diagnostics(messages: impl IntoIterator<Item = String>) -> Vec<Value> {
    let mut messages: Vec<_> = messages.into_iter().collect();
    messages.sort_unstable();
    messages
        .into_iter()
        .map(|message| json!({"ruleId":"CTR-013","level":"error","message":{"text":message}}))
        .collect()
}

pub(crate) fn submission_diagnostics<'a>(
    state: &'a State,
    installed: &'a [Snapshot],
    manifest: Option<&Value>,
    documents: impl IntoIterator<Item = &'a Value>,
) -> Vec<Value> {
    let Some(address) = manifest.and_then(|manifest| manifest["address"].as_str()) else {
        return Vec::new();
    };
    diagnostics(sharing(address, documents, state, installed).into_iter().map(|item| {
        format!("Client ID {} is stated by Type {address}#{} in this project and by Type {}#{} in {}, at another address; a submission accepts no confirmation: give the Type another client ID, or have root install the project as a package with confirmClientIds listing {}.", item.id, item.proposed_name, item.held_address, item.held_name, item.place, item.id)
    }))
}

pub(crate) fn install_rejection(
    t: &mut Transition,
    package: &Snapshot,
    installed: &[Snapshot],
    confirmed: &[Value],
) -> Option<Rejection> {
    let found = sharing(
        &package.address,
        &package.project.documents,
        &t.state,
        installed,
    );
    let confirmed: BTreeSet<_> = confirmed.iter().filter_map(Value::as_str).collect();
    let shared: BTreeSet<_> = found.iter().map(|item| item.id).collect();
    let invalid = diagnostics(confirmed.difference(&shared).map(|id| {
        format!("confirmClientIds lists {id}, which no Type of this package shares with a Type at another address; list only shared client IDs.")
    }));
    let (reason, rejected) = if invalid.is_empty() {
        ("conflict", diagnostics(found.into_iter().filter(|item| !confirmed.contains(item.id)).map(|item| {
            format!("Client ID {} is stated by Type {}#{} in this package and by Type {}#{} in {}, at another address; root may confirm the sharing by sending install.v1 for this package with confirmClientIds listing {}.", item.id, package.address, item.proposed_name, item.held_address, item.held_name, item.place, item.id)
        })))
    } else {
        ("invalid", invalid)
    };
    if rejected.is_empty() {
        return None;
    }
    t.decisions.push(decision(
        "reject-install-client-ids",
        &["CTR-013", "OPS-023"],
        &[],
        json!({}),
        vec![],
    ));
    t.reply = json!({"accepted":false,"reason":reason,"diagnostics":rejected});
    Some(Rejection::new(reason, "CTR-013"))
}

pub(crate) fn confirmation_inputs(args: &Value) -> Option<Value> {
    args.get("confirmClientIds")
        .map(|ids| json!({"confirmClientIds":ids}))
}
