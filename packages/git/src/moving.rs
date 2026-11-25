//! Journal an in-place move; retained entries never leave the opened root.
use super::{Copy, Git, OpenedDirectory, Result, State, git_output, patch, require};
use crate::{Config, MAX_BYTES, files, objects};
use rustix::{
    fs::{
        AtFlags, FileType, Mode, OFlags, fchmod, fstat, mkdirat, openat, readlinkat, renameat,
        statat, symlinkat, unlinkat,
    },
    io::Errno,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs::{self, File},
    io,
    os::{fd::OwnedFd, unix::ffi::OsStringExt},
    path::{Path, PathBuf},
};

const PATCH_REFUSED: &str = "move refused: the patch does not apply to commit; provide a patch that applies to commit without .git paths; nothing was changed";

fn unchanged(cause: String) -> String {
    if cause.ends_with("; nothing was changed") {
        cause
    } else {
        format!(
            "move refused: {cause}; correct the reported problem and retry the move; nothing was changed"
        )
    }
}

fn entry_error(action: &str, path: &Path, cause: impl std::fmt::Display) -> String {
    format!("{action} {path:?}: {cause}")
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct Inputs {
    commit: String,
    expected_base: String,
    expected_patch_sha256: String,
    patch_sha256: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub(super) struct Pending {
    inputs: Inputs,
    kept: Vec<Kept>,
}

impl Pending {
    pub(super) fn refusal(&self) -> String {
        self.failure("the previous move has not finished; this request cannot proceed")
    }

    fn failure(&self, cause: &str) -> String {
        let cause = cause.strip_prefix("move refused: ").unwrap_or(cause);
        let cause = cause.strip_suffix("; nothing was changed").unwrap_or(cause);
        format!(
            "a move to {} ended part way: {cause}; the copy is left as the record names; repair the reported problem and repeat the move with the same inputs; if it cannot finish, retire the Resource and set it up again (see README, Moving the copy)",
            self.inputs.commit
        )
    }
}

#[derive(Deserialize, Serialize)]
struct Kept {
    // Bytes also represent ignored names that are not UTF-8.
    path: Vec<u8>,
    device: u64,
    inode: u64,
    whole: bool,
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

impl Inputs {
    fn parse(args: &Map<String, Value>) -> Result<(Self, Option<&str>)> {
        if args.keys().any(|key| {
            !matches!(
                key.as_str(),
                "commit" | "expectedBase" | "expectedPatchSha256" | "patch"
            )
        }) {
            return Err("unexpected working-copy arguments".into());
        }
        let hex = |name, length| -> Result<String> {
            args.get(name)
                .and_then(Value::as_str)
                .filter(|value| {
                    value.len() == length
                        && value
                            .bytes()
                            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                })
                .map(str::to_owned)
                .ok_or_else(|| format!("{name} must be {length} lowercase hexadecimal digits"))
        };
        let patch = args
            .get("patch")
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(|| "patch must be a string".to_owned())
            })
            .transpose()?;
        Ok((
            Self {
                commit: hex("commit", 40)?,
                expected_base: hex("expectedBase", 40)?,
                expected_patch_sha256: hex("expectedPatchSha256", 64)?,
                patch_sha256: patch.map(|p| digest(p.as_bytes())),
            },
            patch,
        ))
    }
}

impl State {
    pub(super) fn clean_workspace(&self) -> Result<()> {
        // Setup owns this whole private repository, even after a kill during staging.
        let label = Path::new("repository.git/move-staging");
        match OpenedDirectory::open(&self.git_dir().join("move-staging")) {
            Ok(dir) => {
                clear(&dir.dir, label, &BTreeMap::new())?;
                dir.remove()
                    .map_err(|cause| entry_error("cannot remove", label, cause))
            }
            Err(Errno::NOENT) => Ok(()),
            Err(cause) => Err(entry_error("cannot open", label, cause)),
        }
    }

    pub fn move_copy(&self, config: &Config, args: &Map<String, Value>) -> Result<Value> {
        let mut copy = self
            .copy()
            .map_err(unchanged)?
            .ok_or_else(|| unchanged("instance has not made a working copy".into()))?;
        let mut staged = false;
        let result = (|| {
            let (inputs, supplied_patch) = Inputs::parse(args)?;
            if let Some(pending) = &copy.pending {
                if pending.inputs != inputs {
                    return Err("the inputs do not match the recorded move".into());
                }
            } else if copy.base != inputs.expected_base {
                return Err("move refused: the copy is not at expectedBase; read changes.v1 again; nothing was changed".into());
            }
            let root = self.root(config)?;
            self.clean_workspace()?;
            let workspace = self.git_dir().join("move-staging");
            fs::create_dir(&workspace).map_err(|cause| {
                entry_error(
                    "cannot create",
                    Path::new("repository.git/move-staging"),
                    cause,
                )
            })?;
            staged = true;
            let candidates = if copy.pending.is_some() {
                Vec::new()
            } else {
                let (read, ignored) =
                    patch::take_in(&self.directory, &copy.base, &root, &workspace)?;
                if digest(read["patch"].as_str().unwrap().as_bytes())
                    != inputs.expected_patch_sha256
                {
                    return Err("move refused: the copy's change is not the one expectedPatchSha256 names; read changes.v1 again; nothing was changed".into());
                }
                ignored
            };
            self.finish_move(
                config,
                &root,
                &mut copy,
                &workspace,
                (inputs, supplied_patch),
                candidates,
            )
        })();
        result.map_err(|mut cause| {
            if staged && let Err(cleanup) = self.clean_workspace() {
                cause = format!("{cause}; staging cleanup also failed: {cleanup}");
            }
            match &copy.pending {
                Some(pending) => pending.failure(&cause),
                None => unchanged(cause),
            }
        })
    }

    fn finish_move(
        &self,
        config: &Config,
        root: &OwnedFd,
        copy: &mut Copy,
        workspace: &Path,
        request: (Inputs, Option<&str>),
        candidates: Vec<PathBuf>,
    ) -> Result<Value> {
        let (inputs, supplied_patch) = request;
        let home = self.directory.join("home");
        let git_dir = self.git_dir();
        let git = Git::new(&home, &git_dir)?;
        objects::import(&config.repository, &git_dir.join("objects"))
            .map_err(|_| "repository is unreadable")?;
        git.validate_base(&inputs.commit)?;
        let tree = workspace.join("tree");
        let index = workspace.join("index");
        fs::create_dir(&tree)
            .map_err(|cause| entry_error("cannot create", Path::new("move-staging/tree"), cause))?;
        git.checkout(&inputs.commit, &tree, &index)?;
        let base_index = workspace.join("base.index");
        fs::copy(&index, &base_index).map_err(|cause| {
            entry_error("cannot save", Path::new("move-staging/base.index"), cause)
        })?;
        if let Some(patch) = supplied_patch {
            if patch.len() > MAX_BYTES {
                return Err("move refused: the patch is over 8 MiB; provide a patch of at most 8 MiB; nothing was changed".into());
            }
            if !patch.is_empty() {
                let applied = git_output(
                    git.tree(&tree, &index).args(["apply", "--index"]),
                    Some(patch.as_bytes().to_vec()),
                )?;
                require(&applied, PATCH_REFUSED)?;
            }
        }
        let listed = git_output(git.tree(&tree, &index).args(["ls-files", "-z"]), None)?;
        require(&listed, "cannot list move tree")?;
        let tracked = patch::split_paths(&listed.stdout);
        if tracked.iter().any(|path| {
            path.components()
                .any(|part| files::git_entry(part.as_os_str()))
        }) {
            return Err(PATCH_REFUSED.into());
        }
        if copy.pending.is_none() {
            let mut kept = BTreeMap::new();
            for path in candidates {
                if tracked
                    .iter()
                    .any(|p| p.starts_with(&path) || path.starts_with(p))
                {
                    continue;
                }
                let parent = files::walk(root, path.parent().unwrap(), false).map_err(|cause| {
                    entry_error("cannot open parent of ignored entry", &path, cause)
                })?;
                let stat = statat(
                    &parent,
                    path.file_name().unwrap(),
                    AtFlags::SYMLINK_NOFOLLOW,
                )
                .map_err(|cause| entry_error("cannot inspect ignored entry", &path, cause))?;
                let mut query = Path::new(".").join(&path).into_os_string().into_vec();
                if FileType::from_raw_mode(stat.st_mode) == FileType::Directory {
                    query.push(b'/');
                }
                query.push(0);
                let ignored = git_output(
                    git.tree(&tree, &index)
                        .args(["check-ignore", "--no-index", "-z", "--stdin"]),
                    Some(query),
                )?;
                if !matches!(ignored.status.code(), Some(0 | 1)) {
                    return Err(format!(
                        "cannot read moved tree's ignore rules for {path:?}"
                    ));
                }
                if ignored.status.code() != Some(0) {
                    continue;
                }
                kept.insert(path.clone(), keep(&path, &stat, true));
                for ancestor in path
                    .ancestors()
                    .skip(1)
                    .filter(|p| !p.as_os_str().is_empty())
                {
                    let dir = files::walk(root, ancestor, false).map_err(|cause| {
                        entry_error("cannot open ignored entry's ancestor", ancestor, cause)
                    })?;
                    let stat = fstat(&dir).map_err(|cause| {
                        entry_error("cannot inspect ignored entry's ancestor", ancestor, cause)
                    })?;
                    kept.entry(ancestor.to_owned())
                        .or_insert_with(|| keep(ancestor, &stat, false));
                }
            }
            copy.setup_commit.get_or_insert_with(|| copy.base.clone());
            copy.pending = Some(Pending {
                inputs: inputs.clone(),
                kept: kept.into_values().collect(),
            });
            if let Err(cause) = self.remember(copy) {
                copy.pending = None;
                return Err(cause);
            }
        }
        // View creation uses the commit's saved index, not the applied patch's index.
        git.write_view(&inputs.commit, &tree, &base_index)?;
        let kept = copy
            .pending
            .as_ref()
            .unwrap()
            .kept
            .iter()
            .map(|entry| (PathBuf::from(OsString::from_vec(entry.path.clone())), entry))
            .collect();
        clear(root, Path::new(""), &kept)?;
        let source = files::absolute(&tree)
            .map_err(|cause| entry_error("cannot open", Path::new("move-staging/tree"), cause))?;
        publish(&source, root, Path::new(""))?;
        let (read, _) = patch::take_in(&self.directory, &inputs.commit, root, workspace)
            .map_err(|cause| format!("cannot verify moved copy: {cause}"))?;
        copy.base = inputs.commit;
        // Clean staging before dropping the pending move, including after a killed publication.
        self.clean_workspace()?;
        let pending = copy.pending.take();
        if let Err(cause) = self.remember(copy) {
            copy.pending = pending;
            return Err(cause);
        }
        Ok(json!({"base":copy.base,"files":read["files"]}))
    }
}

fn keep(path: &Path, stat: &rustix::fs::Stat, whole: bool) -> Kept {
    Kept {
        path: path.as_os_str().as_encoded_bytes().to_vec(),
        device: stat.st_dev as u64,
        inode: stat.st_ino,
        whole,
    }
}

fn clear(root: &OwnedFd, parent: &Path, kept: &BTreeMap<PathBuf, &Kept>) -> Result<bool> {
    let mut has_kept = false;
    for (name, kind) in
        files::entries(root).map_err(|cause| entry_error("cannot list", parent, cause))?
    {
        let path = parent.join(&name);
        let retained = kept.get(&path).filter(|entry| {
            statat(root, &name, AtFlags::SYMLINK_NOFOLLOW)
                .is_ok_and(|stat| (stat.st_dev as u64, stat.st_ino) == (entry.device, entry.inode))
        });
        if retained.is_some_and(|entry| entry.whole) {
            has_kept = true;
            continue;
        }
        if kind == FileType::Directory {
            let dir = files::dir_at(root, &name)
                .map_err(|cause| entry_error("cannot open", &path, cause))?;
            // A caller may have replaced an ancestor while repairing a failed move.
            // A descendant still at its recorded inode must survive that repair too.
            let descendant = clear(&dir, &path, kept)?;
            if retained.is_some() || descendant {
                has_kept = true;
            } else {
                files::remove_dir_at(root, &name, &dir)
                    .map_err(|cause| entry_error("cannot remove directory", &path, cause))?;
            }
        } else {
            unlinkat(root, &name, AtFlags::empty())
                .map_err(|cause| entry_error("cannot remove", &path, cause))?;
        }
    }
    Ok(has_kept)
}

fn publish(source: &OwnedFd, target: &OwnedFd, parent: &Path) -> Result<()> {
    for (name, kind) in files::entries(source)
        .map_err(|cause| entry_error("cannot list staged directory", parent, cause))?
    {
        let path = parent.join(&name);
        if kind == FileType::Directory {
            match mkdirat(target, &name, Mode::from_bits_truncate(0o755)) {
                Ok(()) | Err(Errno::EXIST) => {}
                Err(cause) => return Err(entry_error("cannot create directory", &path, cause)),
            }
            let from = files::dir_at(source, &name)
                .map_err(|cause| entry_error("cannot open staged directory", &path, cause))?;
            let to = files::dir_at(target, &name)
                .map_err(|cause| entry_error("cannot open destination directory", &path, cause))?;
            publish(&from, &to, &path)?;
        } else {
            // All target leaves were removed; preserved directories only merge directories.
            let result = match renameat(source, &name, target, &name) {
                Err(Errno::XDEV) => publish_across_devices(source, target, &name, kind),
                result => result.map_err(io::Error::from),
            };
            result.map_err(|cause| entry_error("cannot publish", &path, cause))?;
        }
    }
    Ok(())
}

// Only new checkout entries cross devices. No kept entry is opened or copied here.
fn publish_across_devices(
    source: &OwnedFd,
    target: &OwnedFd,
    name: &std::ffi::OsStr,
    kind: FileType,
) -> io::Result<()> {
    if kind == FileType::Symlink {
        let link = readlinkat(source, name, Vec::new())?;
        return symlinkat(link.as_c_str(), target, name).map_err(io::Error::from);
    }
    if kind != FileType::RegularFile {
        return Err(Errno::INVAL.into());
    }
    let mut input = files::regular_at(source, name)?;
    let stat = fstat(&input)?;
    let fd = openat(
        target,
        name,
        OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::from_bits_truncate(stat.st_mode & 0o777),
    )?;
    let mut output = File::from(fd);
    io::copy(&mut input, &mut output)?;
    fchmod(&output, Mode::from_bits_truncate(stat.st_mode & 0o777))?;
    output.sync_all()
}
