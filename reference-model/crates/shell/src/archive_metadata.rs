//! Metadata discovery only, before install-archive admission.
//! The virtual unpack/link helpers below are copied from core/archives.rs because
//! that API requires address/version and does not expose metadata discovery.
//! Keep their semantics aligned; admission still runs in the core.
use conduit_core::archives::Member;
use flate2::read::MultiGzDecoder;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
};

pub(super) fn manifest(bytes: &[u8]) -> crate::Result<serde_json::Value> {
    let files = unpack(bytes).map_err(|(rule, message)| format!("{rule}: {message}"))?;
    let mut manifests = Vec::new();
    for name in ["conduit.yaml", "conduit.yml", "conduit.json"] {
        if let Some(Member::File { bytes, .. }) =
            resolve(&files, name).map_err(|(rule, message)| format!("{rule}: {message}"))?
        {
            let syntax = if name.ends_with(".json") {
                conduit_loader::Syntax::Json
            } else {
                conduit_loader::Syntax::Yaml
            };
            let documents = conduit_loader::read_bytes(syntax, bytes).map_err(|e| e.to_string())?;
            if documents.len() != 1 {
                return Err("expected one package manifest".into());
            }
            manifests.push(documents[0].to_json());
        }
    }
    if manifests.len() != 1 {
        return Err("expected one root manifest: conduit.yaml, conduit.yml or conduit.json".into());
    }
    Ok(manifests.remove(0))
}

type ArchiveResult<T> = std::result::Result<T, (&'static str, String)>;
fn malformed(message: impl ToString) -> (&'static str, String) {
    ("PKG-022", message.to_string())
}
fn unsafe_path(message: impl ToString) -> (&'static str, String) {
    ("MAN-014", message.to_string())
}
fn member_path(path: &[u8]) -> ArchiveResult<String> {
    let s = std::str::from_utf8(path).map_err(malformed)?;
    if s.starts_with('/') || s.split('/').any(|c| c == "..") {
        return Err(unsafe_path(format!("unsafe member {s}")));
    }
    Ok(s.split('/')
        .filter(|s| !s.is_empty() && *s != ".")
        .collect::<Vec<_>>()
        .join("/"))
}
fn link_path(parent: &str, target: &str) -> ArchiveResult<String> {
    if target.starts_with('/') {
        return Err(unsafe_path(format!("absolute link {target}")));
    }
    // Keep `..` until resolution: a preceding component may itself be a link.
    Ok(if parent.is_empty() {
        target.into()
    } else {
        format!("{parent}/{target}")
    })
}
/// Resolve links in memory, including links used as a member's parent. Safety checks
/// allow forward/dangling targets; reads additionally require every parent to exist
/// as a directory, before applying a following `.` or `..` (MAN-011/SUB-010).
fn resolved_path(
    files: &BTreeMap<String, Member>,
    path: &str,
    require_directories: bool,
) -> ArchiveResult<String> {
    let mut path = path.to_owned();
    let mut visited = BTreeSet::new();
    let mut expansions = 0;
    loop {
        if !visited.insert(path.clone()) {
            return Err(malformed(format!("link cycle at {path}")));
        }
        let parts: Vec<_> = path.split('/').collect();
        let mut walked = vec![];
        let mut replacement = None;
        for (i, part) in parts.iter().enumerate() {
            match *part {
                "" | "." => continue,
                ".." => {
                    if walked.pop().is_none() {
                        return Err(unsafe_path(format!("link outside archive: {path}")));
                    }
                    continue;
                }
                _ => walked.push(*part),
            }
            let prefix = walked.join("/");
            if let Some(Member::Link { target }) = files.get(&prefix) {
                replacement = Some(if i + 1 == parts.len() {
                    target.clone()
                } else {
                    format!("{target}/{}", parts[i + 1..].join("/"))
                });
                break;
            }
            if require_directories
                && i + 1 < parts.len()
                && !matches!(files.get(&prefix), Some(Member::Directory))
            {
                return Err(malformed(format!(
                    "{prefix}: intermediate component is not an existing directory"
                )));
            }
        }
        match replacement {
            Some(next) => {
                if expansions == 40 {
                    return Err(malformed("more than 40 link expansions"));
                }
                expansions += 1;
                path = next;
            }
            None => return Ok(walked.join("/")),
        }
    }
}
fn resolve<'a>(
    files: &'a BTreeMap<String, Member>,
    path: &str,
) -> ArchiveResult<Option<&'a Member>> {
    Ok(files.get(&resolved_path(files, path, true)?))
}
fn unpack(bytes: &[u8]) -> ArchiveResult<BTreeMap<String, Member>> {
    let mut tar_bytes = vec![];
    MultiGzDecoder::new(bytes)
        .read_to_end(&mut tar_bytes)
        .map_err(malformed)?;
    if tar_bytes.len() < 1024
        || tar_bytes.len() % 512 != 0
        || !tar_bytes[tar_bytes.len() - 1024..].iter().all(|b| *b == 0)
    {
        return Err(malformed("truncated tar archive"));
    }
    let mut archive = tar::Archive::new(tar_bytes.as_slice());
    let mut files = BTreeMap::new();
    let mut end = 0;
    for entry in archive.entries().map_err(malformed)?.raw(true) {
        let mut entry = entry.map_err(malformed)?;
        let mut name = member_path(&entry.path_bytes())?;
        if entry.header().as_ustar().is_none() {
            return Err(malformed(format!("{name}: DL7 accepts USTAR only")));
        }
        let size = entry.size();
        end = usize::try_from(
            entry
                .raw_file_position()
                .checked_add(
                    size.checked_add(511)
                        .ok_or_else(|| malformed("member size overflow"))?
                        / 512
                        * 512,
                )
                .ok_or_else(|| malformed("member size overflow"))?,
        )
        .map_err(malformed)?;
        if end > tar_bytes.len() {
            return Err(malformed(format!("{name}: truncated member")));
        }
        if let Some((parent, leaf)) = name.rsplit_once('/') {
            let parent = resolved_path(&files, parent, false)
                .map_err(|(rule, m)| (rule, format!("{name}: {m}")))?;
            name = if parent.is_empty() {
                leaf.into()
            } else {
                format!("{parent}/{leaf}")
            };
        }
        let parts: Vec<_> = name.split('/').collect();
        for i in 1..parts.len() {
            let parent = parts[..i].join("/");
            match files.get(&parent) {
                None => {
                    files.insert(parent, Member::Directory);
                }
                Some(Member::Directory) => {}
                _ => return Err(malformed(format!("{name}: parent is not a directory"))),
            }
        }
        let kind = entry.header().entry_type();
        let member = if kind.is_file() {
            let mode = entry.header().mode().map_err(malformed)?;
            let mut bytes = vec![];
            entry.read_to_end(&mut bytes).map_err(malformed)?;
            Member::File { bytes, mode }
        } else if kind.is_dir() {
            Member::Directory
        } else if kind.is_symlink() || kind.is_hard_link() {
            let target = entry
                .link_name_bytes()
                .ok_or_else(|| malformed(format!("{name}: missing link target")))?;
            let target = std::str::from_utf8(&target).map_err(malformed)?;
            let parent = if kind.is_hard_link() {
                ""
            } else {
                name.rsplit_once('/').map_or("", |(p, _)| p)
            };
            let target =
                link_path(parent, target).map_err(|(_, m)| unsafe_path(format!("{name}: {m}")))?;
            if kind.is_hard_link() {
                match resolve(&files, &target)
                    .map_err(|(rule, m)| (rule, format!("{name}: {m}")))?
                {
                    Some(Member::File { bytes, mode }) => Member::File {
                        bytes: bytes.clone(),
                        mode: *mode,
                    },
                    _ => {
                        return Err(malformed(format!(
                            "{name}: hard link target is not an earlier regular file"
                        )));
                    }
                }
            } else {
                // Reject an escaping target even if a later member overwrites this link.
                resolved_path(&files, &target, false)
                    .map_err(|(rule, m)| (rule, format!("{name}: {m}")))?;
                Member::Link { target }
            }
        } else {
            return Err(malformed(format!("{name}: unsupported USTAR entry type")));
        };
        if name.is_empty() {
            if !matches!(member, Member::Directory) {
                return Err(malformed("archive root is not a directory"));
            }
            continue;
        }
        if !matches!(member, Member::Directory)
            && files.keys().any(|p| p.starts_with(&format!("{name}/")))
        {
            return Err(malformed(format!(
                "{name}: cannot replace a nonempty directory"
            )));
        }
        files.insert(name, member);
    }
    if tar_bytes.len().saturating_sub(end) < 1024 || tar_bytes[end..].iter().any(|b| *b != 0) {
        return Err(malformed(
            "tar has no two-block terminator or has trailing content",
        ));
    }
    // Never follow a link on disk. Resolve all retained paths to detect cycles even in
    // non-document content; path traversal was rejected before any names were normalized.
    for name in files.keys() {
        resolved_path(&files, name, false).map_err(|(rule, m)| (rule, format!("{name}: {m}")))?;
    }
    Ok(files)
}
