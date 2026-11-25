//! Snapshot and verify a patch without giving Git a path to the served copy.
use super::{Git, Result, git_output, require};
use crate::{MAX_BYTES, files};
use rustix::fs::FileType;
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    ffi::OsString,
    fs::{self, File},
    io::Read,
    os::{
        fd::OwnedFd,
        unix::{
            ffi::{OsStrExt, OsStringExt},
            fs::PermissionsExt,
        },
    },
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn nul_paths<'a>(paths: impl IntoIterator<Item = &'a Path>) -> Vec<u8> {
    paths
        .into_iter()
        .flat_map(|p| p.as_os_str().as_bytes().iter().copied().chain([0]))
        .collect()
}

pub(super) fn split_paths(bytes: &[u8]) -> BTreeSet<PathBuf> {
    bytes
        .split(|b| *b == 0)
        .filter(|b| !b.is_empty())
        .map(|b| OsString::from_vec(b.to_vec()).into())
        .collect()
}

fn changed_files(bytes: &[u8]) -> Result<Vec<Value>> {
    let Some(bytes) = bytes.strip_suffix(&[0]) else {
        return if bytes.is_empty() {
            Ok(Vec::new())
        } else {
            Err("invalid Git change list".into())
        };
    };
    let mut fields = bytes.split(|b| *b == 0);
    let mut changed = Vec::new();
    while let Some(code) = fields.next() {
        let path = fields
            .next()
            .and_then(|p| std::str::from_utf8(p).ok())
            .ok_or("changed path is not valid UTF-8")?;
        let change = match code {
            b"A" => "added",
            b"M" | b"T" => "modified",
            b"D" => "deleted",
            _ => return Err("unexpected Git change status".into()),
        };
        changed.push((path, change));
    }
    changed.sort_unstable();
    Ok(changed
        .into_iter()
        .map(|(path, change)| json!({"path":path,"change":change}))
        .collect())
}

fn read_patch(command: &mut Command) -> Result<String> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "cannot start Git diff")?;
    let mut bytes = Vec::new();
    let read = child
        .stdout
        .take()
        .unwrap()
        .take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut bytes);
    if read.is_err() || bytes.len() > MAX_BYTES {
        let _ = child.kill();
    }
    let status = child.wait().map_err(|_| "cannot wait for Git diff")?;
    if bytes.len() > MAX_BYTES {
        return Err("Git patch exceeds 8 MiB".into());
    }
    if read.is_err() || !status.success() {
        return Err("cannot read Git patch".into());
    }
    String::from_utf8(bytes).map_err(|_| "Git patch is not valid UTF-8".into())
}

pub(super) fn take(state: &Path, base: &str, root: &OwnedFd) -> Result<Value> {
    take_in(state, base, root, state).map(|(read, _)| read)
}

pub(super) fn take_in(
    state: &Path,
    base: &str,
    root: &OwnedFd,
    workspace: &Path,
) -> Result<(Value, Vec<PathBuf>)> {
    let home = state.join("home");
    let scratch =
        tempfile::tempdir_in(workspace).map_err(|_| "cannot make private patch workspace")?;
    let private_git = scratch.path().join("repository.git");
    let git = Git::new(&home, &private_git)?;
    let baseline = scratch.path().join("base");
    let baseline_index = scratch.path().join("base.index");
    let snapshot = scratch.path().join("snapshot");
    let ignore = scratch.path().join("ignore");
    for dir in [&baseline, &snapshot, &ignore] {
        fs::create_dir(dir).map_err(|_| "cannot make private patch worktree")?;
    }
    git.init(&baseline, &baseline_index)?;
    // Both paths are package-owned; move snapshots may be nested inside move staging.
    fs::write(
        private_git.join("objects/info/alternates"),
        [
            state.join("repository.git/objects").as_os_str().as_bytes(),
            b"\n",
        ]
        .concat(),
    )
    .map_err(|_| "cannot share the private base objects")?;
    git.checkout(base, &baseline, &baseline_index)?;

    let mut cmd = git.command();
    let base_tree = git_output(cmd.args(["ls-tree", "-r", "--name-only", "-z", base]), None)?;
    require(&base_tree, "cannot list base tree")?;
    let tracked = split_paths(&base_tree.stdout);
    let mut ignored = Vec::new();
    let paths = capture_paths(
        &git,
        &ignore,
        &snapshot,
        &baseline_index,
        root,
        &tracked,
        &mut ignored,
    )?;
    let mut edited = Vec::new();
    for path in &paths {
        if !tracked.contains(path) || !same_entry(&snapshot, &baseline, path)? {
            edited.push(path.as_path());
        }
    }
    let removed: Vec<_> = tracked.difference(&paths).map(PathBuf::as_path).collect();
    let candidates = Candidate {
        git: &git,
        base,
        baseline: &baseline,
        baseline_index: &baseline_index,
        snapshot: &snapshot,
        paths: &paths,
        edited: &edited,
        removed: &removed,
    };
    let read = match candidates.verified(scratch.path(), false) {
        Ok(value) => Ok(value),
        Err(first) => candidates.verified(scratch.path(), true).map_err(|second| {
            if first == second {
                first
            } else {
                format!("{first}. A raw-byte patch could not be verified either: {second}")
            }
        }),
    }?;
    Ok((read, ignored))
}

struct Candidate<'a> {
    git: &'a Git<'a>,
    base: &'a str,
    baseline: &'a Path,
    baseline_index: &'a Path,
    snapshot: &'a Path,
    paths: &'a BTreeSet<PathBuf>,
    edited: &'a [&'a Path],
    removed: &'a [&'a Path],
}

impl Candidate<'_> {
    fn verified(&self, scratch: &Path, raw: bool) -> Result<Value> {
        let attempt = tempfile::tempdir_in(scratch)
            .map_err(|_| "cannot make patch verification workspace")?;
        let index = attempt.path().join("candidate.index");
        let mut cmd = self.git.tree(self.baseline, &index);
        require(
            &git_output(cmd.args(["read-tree", self.base]), None)?,
            "cannot seed temporary Git index",
        )?;

        let mut entries = Vec::new();
        for path in self.removed {
            add_index_entry(
                &mut entries,
                "0 0000000000000000000000000000000000000000",
                path,
            );
        }
        for path in self.edited {
            let (mode, object) = self.blob(path, raw)?;
            add_index_entry(&mut entries, &format!("{mode} {object}"), path);
        }
        if !entries.is_empty() {
            let mut cmd = self.git.tree(self.baseline, &index);
            let update = git_output(
                cmd.args(["update-index", "-z", "--index-info"]),
                Some(entries),
            )?;
            require(&update, "cannot stage private snapshot in Git")?;
        }
        let mut cmd = self.git.tree(self.baseline, &index);
        let indexed = git_output(cmd.args(["ls-files", "-z"]), None)?;
        require(&indexed, "cannot verify temporary Git index")?;
        if &split_paths(&indexed.stdout) != self.paths {
            return Err("temporary Git index could not represent all working-copy paths".into());
        }
        let mut cmd = self.git.tree(self.baseline, &index);
        let info = git_output(
            cmd.args([
                "diff",
                "--cached",
                "--name-status",
                "-z",
                "--no-renames",
                self.base,
                "--",
            ]),
            None,
        )?;
        require(&info, "cannot list changed files")?;
        let changed = changed_files(&info.stdout)?;
        let mut cmd = self.git.tree(self.baseline, &index);
        let patch = read_patch(cmd.args([
            "diff",
            "--cached",
            "--binary",
            "--full-index",
            "--no-renames",
            "--no-ext-diff",
            "--no-textconv",
            "--no-color",
            "--src-prefix=a/",
            "--dst-prefix=b/",
            self.base,
            "--",
        ]))?;

        let checkout = attempt.path().join("verification");
        fs::create_dir(&checkout).map_err(|_| "cannot make private verification checkout")?;
        let check_index = attempt.path().join("verification.index");
        self.git.checkout(self.base, &checkout, &check_index)?;
        if !patch.is_empty() {
            let mut cmd = self.git.tree(&checkout, &check_index);
            let applied = git_output(
                cmd.args(["apply", "--index"]),
                Some(patch.as_bytes().to_vec()),
            )?;
            require(
                &applied,
                "Git cannot apply the patch to a clean checkout of the base",
            )?;
        }
        let actual = tree_files(&checkout)?;
        if let Some(path) = self.paths.symmetric_difference(&actual).next() {
            return Err(not_exact(path));
        }
        for path in self.paths {
            if !same_entry(self.snapshot, &checkout, path)? {
                return Err(not_exact(path));
            }
        }
        Ok(json!({"base":self.base,"files":changed,"patch":patch}))
    }

    fn blob(&self, path: &Path, raw: bool) -> Result<(&'static str, String)> {
        let file = self.snapshot.join(path);
        let meta = fs::symlink_metadata(&file).map_err(|_| "cannot inspect a captured file")?;
        let mut cmd = self.git.tree(self.baseline, self.baseline_index);
        cmd.args(["hash-object", "-w", "--stdin"]);
        let (mode, output) = if meta.is_symlink() {
            let link = fs::read_link(file).map_err(|_| "cannot read a captured symbolic link")?;
            (
                "120000",
                git_output(
                    cmd.arg("--no-filters"),
                    Some(link.as_os_str().as_bytes().to_vec()),
                )?,
            )
        } else {
            if raw {
                cmd.arg("--no-filters");
            } else {
                cmd.arg(format!("--path={}", path.display()));
            }
            let input = File::open(file).map_err(|_| "cannot open a captured file")?;
            let output = cmd
                .stdin(input)
                .output()
                .map_err(|_| "cannot hash a captured file")?;
            (
                if executable(&meta) {
                    "100755"
                } else {
                    "100644"
                },
                output,
            )
        };
        require(&output, "cannot hash a captured file")?;
        let object = std::str::from_utf8(&output.stdout)
            .map_err(|_| "invalid Git blob ID")?
            .trim();
        if object.len() != 40 || !object.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("invalid Git blob ID".into());
        }
        Ok((mode, object.to_owned()))
    }
}

fn add_index_entry(buffer: &mut Vec<u8>, entry: &str, path: &Path) {
    buffer.extend_from_slice(entry.as_bytes());
    buffer.push(b'\t');
    buffer.extend_from_slice(path.as_os_str().as_bytes());
    buffer.push(0);
}

fn not_exact(path: &Path) -> String {
    format!("Git cannot reproduce working-copy path {path:?} exactly")
}

// Git records a file as 100755 or 100644 only, from its owner's execute bit, so that bit alone
// decides whether two entries match.
fn executable(meta: &fs::Metadata) -> bool {
    meta.permissions().mode() & 0o100 != 0
}

fn same_entry(left: &Path, right: &Path, path: &Path) -> Result<bool> {
    let left = left.join(path);
    let right = right.join(path);
    let compare = || -> std::io::Result<bool> {
        let a = fs::symlink_metadata(&left)?;
        let b = fs::symlink_metadata(&right)?;
        if a.is_symlink() && b.is_symlink() {
            return Ok(fs::read_link(left)? == fs::read_link(right)?);
        }
        if !a.is_file() || !b.is_file() || executable(&a) != executable(&b) || a.len() != b.len() {
            return Ok(false);
        }
        let (mut left, mut right) = (File::open(left)?, File::open(right)?);
        let mut a = [0; 8192];
        let mut b = [0; 8192];
        let mut remaining = left.metadata()?.len();
        while remaining > 0 {
            let len = remaining.min(a.len() as u64) as usize;
            left.read_exact(&mut a[..len])?;
            right.read_exact(&mut b[..len])?;
            if a[..len] != b[..len] {
                return Ok(false);
            }
            remaining -= len as u64;
        }
        Ok(true)
    };
    compare().map_err(|_| {
        format!("cannot compare working-copy path {path:?} with the verification checkout")
    })
}

fn tree_files(tree: &Path) -> Result<BTreeSet<PathBuf>> {
    let mut directories = vec![PathBuf::new()];
    let mut files = BTreeSet::new();
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(tree.join(&directory))
            .map_err(|_| "cannot list the verification checkout")?
        {
            let entry = entry.map_err(|_| "cannot read the verification checkout")?;
            let path = directory.join(entry.file_name());
            if entry
                .file_type()
                .map_err(|_| "cannot inspect the verification checkout")?
                .is_dir()
            {
                directories.push(path);
            } else {
                files.insert(path);
            }
        }
    }
    Ok(files)
}

fn capture_paths(
    git: &Git<'_>,
    ignore: &Path,
    snapshot: &Path,
    index: &Path,
    root: &OwnedFd,
    tracked: &BTreeSet<PathBuf>,
    omitted: &mut Vec<PathBuf>,
) -> Result<BTreeSet<PathBuf>> {
    let parents: BTreeSet<_> = tracked
        .iter()
        .flat_map(|path| path.ancestors().skip(1))
        .collect();
    let mut pending = vec![(PathBuf::new(), false)];
    let mut result = BTreeSet::new();
    while let Some((parent, parent_ignored)) = pending.pop() {
        let dir =
            files::walk(root, &parent, false).map_err(|_| "cannot open working-copy directory")?;
        let entries = files::entries(&dir).map_err(|_| "cannot read working-copy directory")?;
        if !parent.as_os_str().is_empty()
            && !parent_ignored
            && entries.iter().any(|(name, _)| files::git_entry(name))
        {
            return Err(format!("nested .git entry in {parent:?}"));
        }
        if !parent_ignored {
            // Git sees directory types and the ignore files in a private tree. It
            // never opens an entry under the served copy, including ignore files.
            for (name, kind) in &entries {
                let target = ignore.join(&parent).join(name);
                if *kind == FileType::Directory && !files::git_entry(name) {
                    fs::create_dir(target)
                        .map_err(|_| "cannot stage a directory for Git ignore matching")?;
                } else if name == ".gitignore" && *kind == FileType::RegularFile {
                    files::capture(&dir, name, *kind, &target)
                        .map_err(|_| "cannot capture a working-copy ignore file")?;
                }
            }
        }
        let candidates: Vec<_> = entries
            .into_iter()
            .filter(|(name, _)| !files::git_entry(name))
            .map(|(name, kind)| (parent.join(&name), name, kind))
            .collect();
        let ignored = if candidates.is_empty() || parent_ignored {
            BTreeSet::new()
        } else {
            let mut cmd = git.tree(ignore, index);
            let relative: Vec<_> = candidates
                .iter()
                .map(|(path, _, _)| Path::new(".").join(path))
                .collect();
            let queried = git_output(
                cmd.args(["check-ignore", "--no-index", "-z", "--stdin"]),
                Some(nul_paths(relative.iter().map(PathBuf::as_path))),
            )?;
            if !matches!(queried.status.code(), Some(0 | 1)) {
                return Err("cannot read Git ignore rules".into());
            }
            split_paths(&queried.stdout)
                .into_iter()
                .map(|path| path.strip_prefix(".").map(Path::to_owned).unwrap_or(path))
                .collect()
        };
        for (path, name, kind) in candidates {
            let excluded = parent_ignored || ignored.contains(&path);
            if excluded && !tracked.contains(&path) && !parents.contains(path.as_path()) {
                omitted.push(path.clone());
            }
            if kind == FileType::Directory {
                if !excluded || parents.contains(path.as_path()) {
                    utf8_path(&path)?;
                    fs::create_dir(snapshot.join(&path))
                        .map_err(|_| "cannot stage a snapshot directory")?;
                    pending.push((path, excluded));
                }
            } else if !excluded || tracked.contains(&path) {
                utf8_path(&path)?;
                let target = snapshot.join(&path);
                let captured =
                    if !parent_ignored && name == ".gitignore" && kind == FileType::RegularFile {
                        fs::hard_link(ignore.join(&path), target)
                    } else {
                        files::capture(&dir, &name, kind, &target)
                    };
                captured.map_err(|_| {
                    format!("cannot capture working-copy path {path:?} without following links")
                })?;
                result.insert(path);
            }
        }
    }
    Ok(result)
}

fn utf8_path(path: &Path) -> Result<()> {
    path.to_str()
        .map(|_| ())
        .ok_or_else(|| "working-copy path is not valid UTF-8".into())
}
