//! Recover the authored dependency order after boundary metadata validation.
use super::Requirement;
use crate::Result;
use serde_json::Value;
use std::{
    collections::{BTreeMap, VecDeque},
    io::Read,
    path::Path,
};

pub(super) fn dependencies(bytes: &[u8], manifest: &Value) -> Result<Vec<Requirement>> {
    let Some(items) = manifest["packages"].as_object() else {
        return Ok(Vec::new());
    };
    if items.len() <= 1 {
        return Ok(items
            .iter()
            .filter_map(|(address, value)| Requirement::read(address, value))
            .collect());
    }
    // JSON values discard mapping order. The archive metadata reader has already
    // validated the members and links. Read the same manifest as a loader Node.
    let members = authored_members(bytes)?;
    for name in conduit_loader::MANIFEST_NAMES {
        let path = resolve(&members, name, true)?;
        let Some(Member::File(bytes)) = members.get(&path) else {
            continue;
        };
        let syntax = conduit_loader::Syntax::of(name).ok_or("unknown manifest syntax")?;
        let nodes = conduit_loader::read_bytes(syntax, bytes).map_err(|error| error.to_string())?;
        let node = &nodes[0];
        if node.to_json() != *manifest {
            return Err("cannot determine the archive manifest's written order".into());
        }
        if let Some(conduit_loader::Node {
            value: conduit_loader::Value::Object(entries),
            ..
        }) = node.get("packages")
        {
            return Ok(entries
                .iter()
                .filter_map(|(address, item)| Requirement::read(address, &item.to_json()))
                .collect());
        }
    }
    Err("cannot determine the archive manifest's written order".into())
}

enum Member {
    File(Vec<u8>),
    Link(String),
    Directory,
}

fn authored_members(bytes: &[u8]) -> Result<BTreeMap<String, Member>> {
    let gzip = flate2::read::MultiGzDecoder::new(bytes);
    let mut tar = tar::Archive::new(gzip);
    let mut members = BTreeMap::new();
    for entry in tar.entries().map_err(|error| error.to_string())?.raw(true) {
        let mut entry = entry.map_err(|error| error.to_string())?;
        let raw = entry.path_bytes();
        let path = std::str::from_utf8(&raw).map_err(|error| error.to_string())?;
        let path = resolve(&members, path, false)?;
        let entry_type = entry.header().entry_type();
        let member = if entry_type.is_symlink() || entry_type.is_hard_link() {
            let raw_target = entry
                .link_name_bytes()
                .ok_or("archive link has no target")?;
            let target = std::str::from_utf8(&raw_target).map_err(|error| error.to_string())?;
            let parent = Path::new(&path)
                .parent()
                .and_then(Path::to_str)
                .unwrap_or("");
            let target = if entry_type.is_hard_link() || parent.is_empty() {
                target.to_owned()
            } else {
                format!("{parent}/{target}")
            };
            if entry_type.is_hard_link() {
                let target = resolve(&members, &target, true)?;
                let Some(Member::File(data)) = members.get(&target) else {
                    return Err("archive hard link has no earlier file".into());
                };
                Member::File(data.clone())
            } else {
                Member::Link(target)
            }
        } else if entry_type.is_file() {
            let mut data = Vec::new();
            entry
                .read_to_end(&mut data)
                .map_err(|error| error.to_string())?;
            Member::File(data)
        } else if entry_type.is_dir() {
            Member::Directory
        } else {
            continue;
        };
        members.insert(path, member);
    }
    Ok(members)
}

fn resolve(members: &BTreeMap<String, Member>, path: &str, follow_last: bool) -> Result<String> {
    let mut pending: VecDeque<String> = components(path).collect();
    let mut resolved = Vec::new();
    let mut links = 0;
    while let Some(part) = pending.pop_front() {
        if part == ".." {
            resolved.pop().ok_or("archive link leaves the root")?;
            continue;
        }
        resolved.push(part);
        if (follow_last || !pending.is_empty())
            && let Some(Member::Link(target)) = members.get(&resolved.join("/"))
        {
            links += 1;
            if links > 40 {
                return Err("archive link expansion limit exceeded".into());
            }
            let mut target: VecDeque<_> = components(target).collect();
            target.append(&mut pending);
            pending = target;
            resolved.clear();
        }
    }
    Ok(resolved.join("/"))
}

fn components(path: &str) -> impl Iterator<Item = String> {
    path.split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .map(str::to_owned)
}
