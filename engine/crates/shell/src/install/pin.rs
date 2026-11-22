//! Edit just the manifest pin's span and verify the result with the loader.
use super::{Failure, Pin, Requirement, precedence};
use conduit_loader::{Document, MANIFEST_NAMES, Node, Position, Role, Specification, Syntax};
use fs2::FileExt;
use serde_json::json;
use std::{
    cmp::Ordering,
    fs::{self, File, Metadata, OpenOptions},
    io::{self, Write},
    ops::Range,
    os::unix::{
        ffi::OsStrExt,
        fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
    },
    path::{Path, PathBuf},
};

pub(super) struct Manifest {
    pub(super) requirements: Vec<Requirement>,
    update: Option<Update>,
    missing: Option<PathBuf>,
}

struct Update {
    file: PathBuf,
    original: String,
    metadata: Metadata,
    changed: String,
    line: String,
    pin: Pin,
}

impl Manifest {
    pub(super) fn read(repository: &Path, pin: Option<&Pin>) -> Result<Self, Failure> {
        let directory = std::env::current_dir()
            .map_err(|_| Failure::Stop("cannot locate the project directory".into()))?;
        let mut found = None;
        for name in MANIFEST_NAMES {
            let file = directory.join(name);
            match fs::symlink_metadata(&file) {
                Ok(metadata) => {
                    if found.is_some() {
                        return Err(stop(
                            &file,
                            pin,
                            "multiple-manifests",
                            "leave only one of conduit.yaml, conduit.yml and conduit.json",
                        ));
                    }
                    if !metadata.is_file() {
                        return Err(stop(
                            &file,
                            pin,
                            "not-a-regular-file",
                            "give the manifest as a regular file in the project directory",
                        ));
                    }
                    found = Some((file, metadata));
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(stop(&file, pin, "unreadable", &error.to_string())),
            }
        }
        let Some((file, metadata)) = found else {
            return if pin.is_some() {
                Ok(Self {
                    requirements: Vec::new(),
                    update: None,
                    missing: Some(directory),
                })
            } else {
                Err(Failure::Stop("no install.v1 sent: restore needs conduit.yaml, conduit.yml or conduit.json in the current directory (MAN-001)".into()))
            };
        };
        let original = fs::read_to_string(&file)
            .map_err(|error| stop(&file, pin, "unreadable", &error.to_string()))?;
        let spec = Specification::read(&repository.join("spec"))
            .map_err(|_| Failure::Stop("cannot read the manifest specification".into()))?;
        let name = file
            .file_name()
            .and_then(|name| name.to_str())
            .expect("known manifest name");
        let syntax = Syntax::of(name).expect("known manifest syntax");
        let root = parse(&original, name, syntax, &spec)
            .map_err(|error| stop(&file, pin, "loader-rejected", &error))?;
        let packages = root.get("packages");
        let entries = packages.and_then(mapping).unwrap_or_default();
        let Some(pin) = pin else {
            let requirements = entries
                .iter()
                .filter_map(|(address, node)| Requirement::read(address, &node.to_json()))
                .collect();
            return Ok(Self {
                requirements,
                update: None,
                missing: None,
            });
        };
        let old = packages.and_then(|packages| packages.get(&pin.address));
        if old.is_some_and(|item| item.get("fetch").is_some()) {
            return Err(stop(
                &file,
                Some(pin),
                "pin-has-fetch",
                "remove or edit the authored fetch explicitly",
            ));
        }
        let (changed, message) = edit(&original, syntax, packages, old, pin)
            .map_err(|error| stop(&file, Some(pin), "unsupported-layout", error))?;
        let updated = parse(&changed, name, syntax, &spec)
            .map_err(|error| stop(&file, Some(pin), "loader-rejected-edit", &error))?;
        let mut intended = root.to_json();
        intended["packages"][&pin.address] = json!({"version":pin.version,"digest":pin.digest});
        if updated.to_json() != intended {
            return Err(stop(
                &file,
                Some(pin),
                "unsupported-layout",
                "the edit would change another manifest value",
            ));
        }
        if changed != original {
            OpenOptions::new()
                .write(true)
                .open(&file)
                .and_then(|_| writable_directory(&directory))
                .map_err(|error| stop(&file, Some(pin), "not-writable", &error.to_string()))?;
        }

        let line = format!("pin {message} file={name}");
        Ok(Self {
            requirements: Vec::new(),
            update: Some(Update {
                file,
                original,
                metadata,
                changed,
                line,
                pin: pin.clone(),
            }),
            missing: None,
        })
    }

    pub(super) fn finish(self, dry_run: bool) -> Result<(), Failure> {
        if let Some(directory) = self.missing {
            println!("pin none: no manifest in {:?}", directory);
        }
        if let Some(update) = self.update {
            if !dry_run {
                update.write().map_err(|error| {
                    let name = update.file.file_name().unwrap_or_default().to_string_lossy();
                    println!("stop Manifest/- file={name} cause=left-unwritten pin={} digest={} requires=edit -", update.pin.description(), update.pin.digest);
                    Failure::Stop(format!("{}; the manifest was left unwritten and successful installs were not rolled back", crate::user_config::escape_line_breaks(&error.to_string())))
                })?;
            }
            println!("{}", update.line);
        }
        Ok(())
    }
}

fn stop(file: &Path, pin: Option<&Pin>, cause: &str, explanation: &str) -> Failure {
    if let Some(pin) = pin {
        let name = file.file_name().unwrap_or_default().to_string_lossy();
        println!(
            "stop Manifest/- file={name} cause={cause} pin={} digest={} requires=edit -",
            pin.description(),
            pin.digest
        );
    }
    Failure::Stop(format!(
        "no install.v1 sent: {file:?}: {}",
        crate::user_config::escape_line_breaks(explanation)
    ))
}

fn parse(text: &str, name: &str, syntax: Syntax, spec: &Specification) -> Result<Node, String> {
    let mut nodes = conduit_loader::read_text(syntax, text).map_err(|errors| {
        errors
            .0
            .iter()
            .map(|d| d.rule)
            .collect::<Vec<_>>()
            .join(", ")
    })?;
    if nodes.len() != 1 || (syntax == Syntax::Json && !text.trim_start().starts_with('{')) {
        return Err("MAN-001 requires one manifest object".into());
    }
    let document = Document {
        file: name.into(),
        index: 1,
        root: nodes.pop().expect("one document"),
    };
    spec.check(&document, Role::Manifest).map_err(|errors| {
        errors
            .0
            .iter()
            .map(|d| d.rule)
            .collect::<Vec<_>>()
            .join(", ")
    })?;
    Ok(document.root)
}

fn mapping(node: &Node) -> Option<&[(String, Node)]> {
    match &node.value {
        conduit_loader::Value::Object(items) => Some(items),
        _ => None,
    }
}

fn writable_directory(directory: &Path) -> io::Result<()> {
    let bytes = std::ffi::CString::new(directory.as_os_str().as_bytes())?;
    // Check access without creating a scratch file during a dry run.
    if unsafe { libc::access(bytes.as_ptr(), libc::W_OK | libc::X_OK) } == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

impl Update {
    fn verify(&self) -> io::Result<()> {
        let now = fs::symlink_metadata(&self.file)?;
        if !now.is_file()
            || now.dev() != self.metadata.dev()
            || now.ino() != self.metadata.ino()
            || now.mode() != self.metadata.mode()
            || now.modified()? != self.metadata.modified()?
            || fs::read_to_string(&self.file)? != self.original
        {
            return Err(io::Error::other("the manifest changed since preflight"));
        }
        let directory = self.file.parent().expect("absolute manifest path");
        if MANIFEST_NAMES.iter().any(|name| {
            let candidate = directory.join(name);
            candidate != self.file && fs::symlink_metadata(candidate).is_ok()
        }) {
            return Err(io::Error::other(
                "another manifest appeared since preflight",
            ));
        }
        Ok(())
    }

    fn write(&self) -> io::Result<()> {
        self.verify()?;
        if self.changed == self.original {
            return Ok(());
        }
        let directory = self.file.parent().expect("absolute manifest path");
        let temporary = directory.join(format!(".conduit-manifest-{}", uuid::Uuid::new_v4()));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temporary)?;
        let result = (|| {
            file.write_all(self.changed.as_bytes())?;
            file.set_permissions(fs::Permissions::from_mode(self.metadata.mode() & 0o7777))?;
            file.sync_all()?;
            // The project directory remains the same inode across manifest replacements.
            let project = File::open(directory)?;
            FileExt::lock_exclusive(&project)?;
            self.verify()?;
            fs::rename(&temporary, &self.file)?;
            // Once renamed the selected file has been written. There is no old
            // manifest left to report as unchanged if the directory sync fails.
            if let Err(error) = project.sync_all() {
                eprintln!(
                    "conduit install: manifest was replaced, but its directory could not be synced: {error}"
                );
            }
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result
    }
}

type LayoutError = &'static str;
const LAYOUT: LayoutError =
    "use a block packages map or an empty packages: {} in YAML, or a conduit.json object";

fn edit(
    text: &str,
    syntax: Syntax,
    packages: Option<&Node>,
    old: Option<&Node>,
    pin: &Pin,
) -> Result<(String, String), LayoutError> {
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let mut changed = text.to_owned();
    let message = if let Some(old) = old {
        let version = old.get("version").and_then(Node::as_str).ok_or(LAYOUT)?;
        let digest = old.get("digest").and_then(Node::as_str).ok_or(LAYOUT)?;
        if syntax == Syntax::Yaml {
            yaml_items(text, packages.ok_or(LAYOUT)?)?;
        }
        if version == pin.version && digest == pin.digest {
            return Ok((
                changed,
                format!(
                    "unchanged {} version={} digest={}",
                    pin.address, pin.version, pin.digest
                ),
            ));
        }
        let version_span = scalar_span(text, old.get("version").ok_or(LAYOUT)?)?;
        let digest_span = scalar_span(text, old.get("digest").ok_or(LAYOUT)?)?;
        let mut edits = [
            (version_span, pin.version.as_str()),
            (digest_span, pin.digest.as_str()),
        ];
        edits.sort_by_key(|(span, _)| span.start);
        let span = edits[0].0.start..edits[1].0.end;
        let middle = text.get(edits[0].0.end..edits[1].0.start).ok_or(LAYOUT)?;
        changed.replace_range(span, &format!("{}{middle}{}", edits[0].1, edits[1].1));
        let movement = match compare_versions(version, &pin.version).ok_or(LAYOUT)? {
            Ordering::Less => "moved up",
            Ordering::Greater => "moved down",
            Ordering::Equal => "moved",
        };
        format!(
            "{movement} {} version={version}->{} digest={digest}->{}",
            pin.address, pin.version, pin.digest
        )
    } else {
        let item = format!(
            "{}: {{version: {}, digest: '{}'}}",
            pin.address, pin.version, pin.digest
        );
        match (syntax, packages) {
            (Syntax::Yaml, Some(packages))
                if mapping(packages).is_some_and(|items| items.is_empty()) =>
            {
                let at = offset(text, packages.position)?;
                let end = flow_end(text, at)?;
                let before = &text[line_start(text, at)..at];
                if before.trim_end() != "packages:" {
                    return Err(LAYOUT);
                }
                let start = line_start(text, at) + "packages:".len();
                changed.replace_range(start..end, &format!("{newline}  {item}"));
            }
            (Syntax::Yaml, Some(packages)) => {
                let (indent, block, last) = yaml_items(text, packages)?;
                let base = " ".repeat(indent);
                let insertion = if block {
                    format!(
                        "{newline}{base}{}:{newline}{base}  version: {}{newline}{base}  digest: '{}'",
                        pin.address, pin.version, pin.digest
                    )
                } else {
                    format!("{newline}{base}{item}")
                };
                changed.insert_str(last, &insertion);
            }
            (Syntax::Yaml, None) => {
                let prefix = if text.ends_with('\n') { "" } else { newline };
                changed.push_str(&format!("{prefix}packages:{newline}  {item}{newline}"));
            }
            (Syntax::Json, packages) => {
                let item = format!(
                    "{}: {{\"version\": {}, \"digest\": {}}}",
                    json!(pin.address),
                    json!(pin.version),
                    json!(pin.digest)
                );
                if let Some(packages) = packages {
                    let items = mapping(packages).ok_or(LAYOUT)?;
                    let (at, inserted) = if let Some((_, last)) = items.last() {
                        (
                            flow_end(text, offset(text, last.position)?)?,
                            format!(", {item}"),
                        )
                    } else {
                        let start = offset(text, packages.position)?;
                        if text.as_bytes().get(start) != Some(&b'{') {
                            return Err(LAYOUT);
                        }
                        (start + 1, item)
                    };
                    changed.insert_str(at, &inserted);
                } else {
                    let end = text.trim_end().len().checked_sub(1).ok_or(LAYOUT)?;
                    if text.as_bytes().get(end) != Some(&b'}') {
                        return Err(LAYOUT);
                    }
                    changed.insert_str(end, &format!(", \"packages\": {{{item}}}"));
                }
            }
        }
        format!(
            "added {} version={} digest={}",
            pin.address, pin.version, pin.digest
        )
    };
    Ok((changed, message))
}

fn offset(text: &str, position: Position) -> Result<usize, LayoutError> {
    let mut lines = text.split_inclusive('\n');
    let first = position.line.checked_sub(1).ok_or(LAYOUT)?;
    let start: usize = (0..first)
        .map(|_| lines.next().map(str::len).ok_or(LAYOUT))
        .sum::<Result<usize, _>>()?;
    let column = position.column.checked_sub(1).ok_or(LAYOUT)?;
    let line = lines.next().ok_or(LAYOUT)?;
    line.char_indices()
        .nth(column as usize)
        .map(|(at, _)| start + at)
        .ok_or(LAYOUT)
}

fn line_start(text: &str, at: usize) -> usize {
    text[..at].rfind('\n').map_or(0, |index| index + 1)
}

fn line_end(text: &str, at: usize) -> usize {
    let end = text[at..].find('\n').map_or(text.len(), |index| at + index);
    if text.as_bytes().get(end.wrapping_sub(1)) == Some(&b'\r') {
        end - 1
    } else {
        end
    }
}

fn indent(text: &str, at: usize) -> usize {
    text[line_start(text, at)..at]
        .bytes()
        .take_while(|byte| *byte == b' ')
        .count()
}

fn yaml_items(text: &str, packages: &Node) -> Result<(usize, bool, usize), LayoutError> {
    let items = mapping(packages).ok_or(LAYOUT)?;
    let mut style = None;
    let mut end = 0;
    for (name, item) in items {
        let at = offset(text, item.position)?;
        let flow = text.as_bytes().get(at) == Some(&b'{');
        let (indent, last) = if flow {
            let start = line_start(text, at);
            let left = text[start..at].trim();
            if left.strip_suffix(':').map(str::trim) != Some(name.as_str()) {
                return Err(LAYOUT);
            }
            let end = flow_end(text, at)?;
            if line_start(text, end) != start {
                return Err(LAYOUT);
            }
            (indent(text, at), line_end(text, end))
        } else {
            let children = mapping(item).ok_or(LAYOUT)?;
            let (_, first) = children.first().ok_or(LAYOUT)?;
            let (_, last) = children.last().ok_or(LAYOUT)?;
            let first = offset(text, first.position)?;
            let header = text[..line_start(text, first)]
                .lines()
                .rev()
                .find(|line| {
                    let content = line.trim_start();
                    !content.is_empty() && !content.starts_with('#')
                })
                .ok_or(LAYOUT)?;
            let key = header.split_once('#').map_or(header, |(key, _)| key);
            if key.trim().strip_suffix(':').map(str::trim) != Some(name.as_str()) {
                return Err(LAYOUT);
            }
            (
                header.bytes().take_while(|byte| *byte == b' ').count(),
                line_end(text, offset(text, last.position)?),
            )
        };
        if indent == 0 || style.is_some_and(|old| old != (indent, !flow)) {
            return Err(LAYOUT);
        }
        style = Some((indent, !flow));
        end = last;
    }
    let (indent, block) = style.ok_or(LAYOUT)?;
    Ok((indent, block, end))
}

fn scalar_span(text: &str, node: &Node) -> Result<Range<usize>, LayoutError> {
    let at = offset(text, node.position)?;
    let first = text.as_bytes()[at];
    if first == b'\'' || first == b'"' {
        let end = quoted_end(text.as_bytes(), at)?;
        return Ok(at + 1..end - 1);
    }
    let length = text[at..]
        .bytes()
        .take_while(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'+' | b':')
        })
        .count();
    if length == 0 {
        return Err(LAYOUT);
    }
    Ok(at..at + length)
}

fn quoted_end(bytes: &[u8], start: usize) -> Result<usize, LayoutError> {
    let quote = bytes[start];
    let mut at = start + 1;
    while at < bytes.len() {
        if bytes[at] == quote {
            if quote == b'\'' && bytes.get(at + 1) == Some(&quote) {
                at += 2;
                continue;
            }
            return Ok(at + 1);
        }
        if quote == b'"' && bytes[at] == b'\\' {
            at += 1;
        }
        at += 1;
    }
    Err(LAYOUT)
}

fn flow_end(text: &str, start: usize) -> Result<usize, LayoutError> {
    let bytes = text.as_bytes();
    if bytes.get(start) != Some(&b'{') {
        return Err(LAYOUT);
    }
    let mut depth = 0;
    let mut at = start;
    while at < bytes.len() {
        match bytes[at] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(at + 1);
                }
            }
            b'\'' | b'"' => {
                at = quoted_end(bytes, at)?;
                continue;
            }
            _ => {}
        }
        at += 1;
    }
    Err(LAYOUT)
}

fn compare_versions(left: &str, right: &str) -> Option<Ordering> {
    fn parts(version: &str) -> Option<([u64; 3], Option<&str>)> {
        let version = precedence(version);
        let (core, pre) = version
            .split_once('-')
            .map_or((version, None), |(a, b)| (a, Some(b)));
        let mut digits = core.split('.');
        let mut number = [0; 3];
        for slot in &mut number {
            *slot = digits.next()?.parse().ok()?;
        }
        digits.next().is_none().then_some((number, pre))
    }

    let (left, left_pre) = parts(left)?;
    let (right, right_pre) = parts(right)?;
    Some(left.cmp(&right).then_with(|| match (left_pre, right_pre) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(left), Some(right)) => {
            let mut left = left.split('.');
            let mut right = right.split('.');
            let numeric = |value: &str| value.bytes().all(|byte| byte.is_ascii_digit());
            loop {
                let order = match (left.next(), right.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (Some(a), Some(b)) => match (numeric(a), numeric(b)) {
                        (true, true) => a.len().cmp(&b.len()).then_with(|| a.cmp(b)),
                        (true, false) => Ordering::Less,
                        (false, true) => Ordering::Greater,
                        (false, false) => a.cmp(b),
                    },
                };
                if order != Ordering::Equal {
                    return order;
                }
            }
        }
    }))
}
