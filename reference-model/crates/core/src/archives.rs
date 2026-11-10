//! Archive admission from supplied bytes only. No archive is extracted onto a filesystem.
use crate::{Context, Result, Transition, decision, packages::*};
use conduit_loader::{
    Document, FileRole, Names, Specification, Syntax, read_bytes, read_submitted_files,
};
use flate2::read::MultiGzDecoder;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArchiveInput {
    pub name: String,
    pub address: String,
    pub version: String,
    pub digest: String,
    /// The shell's eventual package directory, as an absolute file URL; never read here.
    pub directory: Option<String>,
    pub completion: FetchCompletion,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case", deny_unknown_fields)]
pub enum FetchCompletion {
    Complete { bytes: Vec<u8> },
    Unavailable,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Member {
    File { bytes: Vec<u8>, mode: u32 },
    Directory,
    Link { target: String },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstalledPackage {
    pub package: Snapshot,
    /// A virtual unpacked directory, retaining programs and other non-document files too.
    pub files: BTreeMap<String, Member>,
}
fn diagnostic(rule: &str, message: impl Into<String>) -> Value {
    json!({"ruleId":rule,"level":"error","message":{"text":message.into()}})
}
fn reject(t: &mut Transition, diagnostics: Vec<Value>) {
    let mut rules: BTreeSet<_> = diagnostics
        .iter()
        .filter_map(|d| d["ruleId"].as_str())
        .collect();
    rules.insert("PKG-023");
    t.decisions.push(decision(
        "reject-archive",
        &rules.into_iter().collect::<Vec<_>>(),
        &[],
        json!({}),
        vec![],
    ));
    t.reply = json!({"accepted":false,"reason":"invalid","diagnostics":diagnostics});
}
fn specification(builtin: &Project, context: &Context) -> Result<Specification> {
    // The same supplied built-in schema documents as resolution uses. Reconstitute the
    // strict loader's pure interface, without reading spec/ or assigned names here.
    let bytes = serde_json::to_vec(&builtin.documents).map_err(|e| e.to_string())?;
    let documents: Vec<_> = read_bytes(Syntax::Json, &bytes)
        .map_err(|e| e.to_string())?
        .into_iter()
        .enumerate()
        .map(|(i, root)| Document {
            file: "builtin.json".into(),
            index: i + 1,
            root,
        })
        .collect();
    let names = Names {
        core_package: context.core_package.clone(),
        api_version: builtin
            .documents
            .first()
            .and_then(|d| d["apiVersion"].as_str())
            .ok_or("built-in documents have no apiVersion")?
            .into(),
    };
    Specification::from_documents(names, &documents).map_err(|e| e.to_string())
}

/// PKG-004/026 precede decompression. Publication is atomic after every loader/resolver check.
pub(crate) fn install(
    t: &mut Transition,
    input: ArchiveInput,
    snapshots: Vec<Snapshot>,
    builtin: Project,
    context: &Context,
) -> Result<()> {
    let mut diagnostics = vec![];
    if !input.digest.strip_prefix("sha256:").is_some_and(|h| {
        h.len() == 64
            && h.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    }) {
        diagnostics.push(diagnostic(
            "PKG-004",
            format!("{}: invalid digest {}", input.name, input.digest),
        ));
    }
    if semver::Version::parse(&input.version).is_err() {
        diagnostics.push(diagnostic(
            "PKG-001",
            format!("{}: invalid version {}", input.name, input.version),
        ));
    }
    if input.address == context.core_package {
        diagnostics.push(diagnostic(
            "NAM-021",
            format!("cannot install at {}", input.address),
        ));
    }
    if !diagnostics.is_empty() {
        reject(t, diagnostics);
        return Ok(());
    }
    let mut entries = catalog_entries(&t.state, snapshots);
    // Validate supplied catalog metadata even on an already-installed path.
    Catalog::new(builtin.clone(), entries.clone(), &context.core_package)?;
    // PKG-029 reserves a submitted address only while an existing object selects
    // that project's contracts or implementations. A retained but unused context
    // alone is not a holder, and catalog package selections do not reserve it.
    if t.state.objects.iter().any(|object| {
        object["held"]["contexts"]["project"]["address"] == input.address
            && object["packages"]
                .as_array()
                .is_some_and(|packages| packages.iter().any(|p| p["address"] == input.address))
    }) {
        t.decisions.push(decision(
            "submitted-project-address-held",
            &["PKG-029", "MAN-016"],
            &[],
            json!({"address":input.address}),
            vec![],
        ));
        t.reply = json!({"accepted":false,"reason":"conflict","diagnostics":[]});
        return Ok(());
    }
    if let Some(package) = entries
        .iter()
        .find(|s| s.address == input.address && version_equal(&s.version, &input.version))
    {
        let same = package.digest == input.digest;
        t.decisions.push(decision(
            "already-installed",
            &[if same { "PKG-027" } else { "PKG-025" }, "PKG-003"],
            &[],
            json!({"address":input.address,"digest":package.digest}),
            vec![],
        ));
        t.reply = if same {
            json!({"accepted":true,"outcome":"succeeded","diagnostics":[],"changed":false})
        } else {
            json!({"accepted":false,"reason":"conflict","diagnostics":[]})
        };
        return Ok(());
    }
    let bytes = match input.completion {
        FetchCompletion::Unavailable => {
            t.decisions.push(decision(
                "archive-unavailable",
                &["PKG-024"],
                &[],
                json!({"archive":input.name}),
                vec![],
            ));
            t.reply = json!({"accepted":false,"reason":"unavailable","diagnostics":[]});
            return Ok(());
        }
        FetchCompletion::Complete { bytes } => bytes,
    };
    let digest = bytes_digest(&bytes);
    if digest != input.digest {
        reject(
            t,
            vec![diagnostic(
                "PKG-026",
                format!(
                    "{}: expected {}, archive has {digest}",
                    input.name, input.digest
                ),
            )],
        );
        return Ok(());
    }
    t.decisions.push(decision(
        "verify-archive-digest",
        &["PKG-004", "PKG-026"],
        &[],
        json!({"archive":input.name,"digest":digest}),
        vec![],
    ));
    let files = match unpack(&bytes) {
        Ok(files) => files,
        Err((rule, message)) => {
            reject(
                t,
                vec![diagnostic(rule, format!("{}: {message}", input.name))],
            );
            return Ok(());
        }
    };
    t.decisions.push(decision(
        "unpack-archive",
        &["PKG-022", "MAN-013", "MAN-014"],
        &["DL7"],
        json!({"archive":input.name,"format":"ustar"}),
        vec![],
    ));
    let mut selected = vec![];
    for name in files
        .keys()
        .filter(|n| !n.contains('/') && FileRole::of(n) != FileRole::NotRead)
    {
        match resolve(&files, name) {
            Ok(Some(Member::File { bytes, .. })) => selected.push((name.clone(), bytes.clone())),
            Ok(Some(Member::Directory)) => {}
            _ => diagnostics.push(diagnostic(
                "SUB-010",
                format!("{name}: cannot be read as a regular file"),
            )),
        }
    }
    if !selected
        .iter()
        .any(|(n, _)| matches!(FileRole::of(n), FileRole::Manifest(_)))
    {
        diagnostics.push(diagnostic(
            "MAN-013",
            format!("{}: no manifest at archive root", input.name),
        ));
    }
    if !diagnostics.is_empty() {
        reject(t, diagnostics);
        return Ok(());
    }
    let spec = specification(&builtin, context)?;
    let loaded = match read_submitted_files(&selected, &spec) {
        Ok(project) => project,
        Err(errors) => {
            reject(t, errors.0.iter().map(loader_diagnostic).collect());
            return Ok(());
        }
    };
    for document in &loaded.documents {
        if document.root.get("from").is_some() {
            diagnostics.push(diagnostic(
                "MAN-011",
                format!(
                    "{} document {}: an archive holds an instance",
                    document.file, document.index
                ),
            ));
        }
    }
    let project = Project {
        manifest: loaded.manifest.map(|d| d.root.to_json()),
        documents: loaded.documents.iter().map(|d| d.root.to_json()).collect(),
        directory: input.directory,
    };
    let manifest = project
        .manifest
        .as_ref()
        .expect("selected a manifest and loader accepted");
    if manifest["address"] != input.address
        || !version_equal(manifest["version"].as_str().unwrap_or(""), &input.version)
    {
        diagnostics.push(diagnostic(
            "PKG-017",
            format!(
                "{}: asked for {} {}, manifest states {} {}",
                input.name, input.address, input.version, manifest["address"], manifest["version"]
            ),
        ));
    }
    if !diagnostics.is_empty() {
        reject(t, diagnostics);
        return Ok(());
    }
    let package = Snapshot {
        source: SnapshotSource::InstalledArchive,
        address: input.address,
        version: input.version,
        digest,
        project,
    };
    entries.push(package.clone());
    // Resolve as a package scope, not as a submitted project: exports, dependencies and
    // every document are checked with the same semantics as the existing catalog.
    let resolution = Catalog::new(builtin, entries, &context.core_package)?.resolve(&Project {
        manifest: None,
        documents: vec![],
        directory: None,
    })?;
    t.decisions.extend(resolution.decisions);
    if resolution.reply["accepted"] != true {
        let mut diagnostics = resolution.reply["diagnostics"]
            .as_array()
            .ok_or("resolution diagnostics missing")?
            .clone();
        for d in &mut diagnostics {
            if d["ruleId"] == "PKG-006" {
                d["ruleId"] = json!("PKG-028");
            }
        }
        reject(t, diagnostics);
        return Ok(());
    }
    t.decisions.push(decision(
        "install-archive",
        &[
            "NAM-013", "PKG-017", "PKG-023", "PKG-028", "MAN-011", "SUB-005",
        ],
        &["DL7"],
        json!({"address":package.address,"version":package.version,"digest":package.digest}),
        vec![],
    ));
    t.reply = json!({"accepted":true,"outcome":"succeeded","diagnostics":resolution.reply["diagnostics"],"changed":true,"package":package});
    t.state
        .installed_packages
        .push(InstalledPackage { package, files });
    Ok(())
}
fn loader_diagnostic(d: &conduit_loader::Diagnostic) -> Value {
    let mut v = diagnostic(d.rule, d.message.to_string());
    v["properties"] = json!({"file":d.file,"document":d.document});
    let mut location = json!({});
    if let Some(pointer) = &d.pointer {
        location["logicalLocations"] = json!([{"fullyQualifiedName":pointer}]);
    }
    if let Some(file) = &d.file {
        location["physicalLocation"] = json!({"artifactLocation":{"uri":file}});
    }
    if let Some(position) = d.position {
        location["physicalLocation"]["region"] =
            json!({"startLine":position.line,"startColumn":position.column});
    }
    v["locations"] = json!([location]);
    if let Some(k) = d.keyword_location() {
        v["properties"]["keywordLocation"] = json!(k);
    }
    if let Some(k) = d.absolute_keyword_location() {
        v["properties"]["absoluteKeywordLocation"] = json!(k);
    }
    v
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
