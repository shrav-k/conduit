use crate::{Config, files, objects};
use rustix::{
    fs::{FlockOperation, flock, renameat},
    io::Errno,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::Write,
    os::{
        fd::OwnedFd,
        unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
    },
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

#[path = "patch.rs"]
mod patch;

type Result<T> = std::result::Result<T, String>;

#[derive(Serialize, Deserialize)]
struct Copy {
    root: PathBuf,
    base: String,
    staging: PathBuf,
    device: u64,
    inode: u64,
}

struct OpenedDirectory {
    parent: OwnedFd,
    name: OsString,
    dir: OwnedFd,
}

impl OpenedDirectory {
    fn open(path: &Path) -> rustix::io::Result<Self> {
        let parent = files::absolute(path.parent().ok_or(Errno::INVAL)?)?;
        let name = path.file_name().ok_or(Errno::INVAL)?.to_owned();
        let dir = files::dir_at(&parent, &name)?;
        Ok(Self { parent, name, dir })
    }

    fn remove(self) -> Result<()> {
        files::remove_dir_at(&self.parent, &self.name, &self.dir)
            .map_err(|_| "cannot safely remove working-copy directory".into())
    }
}

impl Copy {
    fn owns(&self, dir: &OwnedFd) -> Result<bool> {
        let meta = dir
            .try_clone()
            .and_then(|fd| File::from(fd).metadata())
            .map_err(|_| "cannot inspect working-copy directory")?;
        Ok((meta.dev(), meta.ino()) == (self.device, self.inode))
    }
}

pub struct State {
    directory: PathBuf,
    _lock: File,
}

impl State {
    pub fn locked() -> Result<Self> {
        let directory = std::env::var_os("CONDUIT_STATE_DIR")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute() && files::absolute(p).is_ok())
            .ok_or("no usable CONDUIT_STATE_DIR instance state directory")?;
        let lock = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .open(directory.join("copy.lock"))
            .map_err(|_| "cannot open working-copy lock")?;
        flock(&lock, FlockOperation::LockExclusive).map_err(|_| "cannot lock working copy")?;
        Ok(Self {
            directory,
            _lock: lock,
        })
    }

    fn copy_path(&self) -> PathBuf {
        self.directory.join("copy.json")
    }

    fn git_dir(&self) -> PathBuf {
        self.directory.join("repository.git")
    }

    fn copy(&self) -> Result<Option<Copy>> {
        match fs::read(self.copy_path()) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map(Some)
                .map_err(|_| "invalid working-copy state record".into()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(_) => Err("cannot read working-copy state record".into()),
        }
    }

    fn remember(&self, copy: &Copy) -> Result<()> {
        let save = || -> anyhow::Result<()> {
            let mut temp = tempfile::NamedTempFile::new_in(&self.directory)?;
            serde_json::to_writer(&mut temp, copy)?;
            temp.as_file().sync_all()?;
            temp.persist(self.copy_path())?;
            Ok(())
        };
        save().map_err(|_| "cannot save working-copy state record".into())
    }

    fn remove_attempt(&self, copy: &Copy) -> Result<()> {
        match OpenedDirectory::open(&copy.staging) {
            Ok(dir) if copy.owns(&dir.dir)? => dir.remove()?,
            Ok(_) | Err(Errno::NOTDIR | Errno::LOOP) => {
                return Err("setup staging path no longer belongs to this instance".into());
            }
            Err(Errno::NOENT) => {}
            Err(_) => return Err("cannot inspect setup staging path".into()),
        }
        remove_dir(&self.git_dir())?;
        fs::remove_file(self.copy_path())
            .map_err(|_| "cannot remove working-copy state record".into())
    }

    pub fn setup(&self, config: &Config) -> Result<()> {
        let root = normalize_root(&config.root)?;
        if root.starts_with(&self.directory) || self.directory.starts_with(&root) {
            return Err("working-copy root overlaps its state directory".into());
        }
        if let Some(copy) = self.copy()? {
            if let Ok(dir) = files::absolute(&copy.root)
                && copy.owns(&dir)?
            {
                return if root == copy.root && config.commit == copy.base {
                    Ok(())
                } else {
                    Err(
                        "this instance already made a working copy at another root or commit"
                            .into(),
                    )
                };
            }
            self.remove_attempt(&copy)?;
        }
        let prior = empty_root(&root)?;
        let source = config
            .repository
            .canonicalize()
            .map_err(|_| "repository is unreadable")?;
        if !source.is_dir()
            || source == Path::new("/")
            || source.starts_with(&root)
            || root.starts_with(&source)
        {
            return Err("repository is unreadable or overlaps the working-copy root".into());
        }
        let home = self.directory.join("home");
        fs::create_dir_all(&home).map_err(|_| "cannot make the private Git home")?;
        let git_dir = self.git_dir();
        match fs::symlink_metadata(&git_dir) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            _ => return Err("unowned or inaccessible Git directory in instance state".into()),
        }
        let git = Git::new(&home, &git_dir)?;
        git.check_version()?;

        // The checkout is a sibling so publication is a single filesystem rename.
        let stage = tempfile::Builder::new()
            .prefix(".conduit-checkout-")
            .tempdir_in(root.parent().unwrap())
            .map_err(|_| "cannot stage checkout beside working-copy root")?;
        let metadata =
            fs::metadata(stage.path()).map_err(|_| "cannot inspect setup staging directory")?;
        let copy = Copy {
            root: root.clone(),
            base: config.commit.clone(),
            staging: stage.path().to_owned(),
            device: metadata.dev(),
            inode: metadata.ino(),
        };
        self.remember(&copy)?;
        // From here, the journal owns cleanup. Only its recorded inode can be removed.
        let stage = stage.keep();
        let result = (|| {
            let index = git_dir.join("index");
            git.init(&stage, &index)?;
            objects::import(&source, &git_dir.join("objects"))
                .map_err(|_| "repository is unreadable")?;
            git.validate_base(&config.commit)?;
            git.checkout(&config.commit, &stage, &index)?;
            git.write_view(&config.commit, &stage, &index)?;
            // TempDir starts private; the checkout inherits the mode of a supplied empty root.
            if let Some((_, _, mode)) = prior {
                fs::set_permissions(&stage, fs::Permissions::from_mode(mode & 0o777))
                    .map_err(|_| "cannot preserve working-copy root permissions")?;
            }
            if empty_root(&root)? != prior {
                return Err("working-copy root changed during setup".into());
            }
            let parent = files::absolute(root.parent().unwrap())
                .map_err(|_| "cannot open working-copy root parent for publication")?;
            renameat(
                &parent,
                stage.file_name().unwrap(),
                &parent,
                root.file_name().unwrap(),
            )
            .map_err(|_| "cannot publish checkout at working-copy root".into())
        })();
        if result.is_err() {
            self.remove_attempt(&copy)?;
        }
        result
    }

    pub fn root(&self, config: &Config) -> Result<OwnedFd> {
        let copy = self.copy()?.ok_or("instance has not made a working copy")?;
        if normalize_root(&config.root)? != copy.root {
            return Err("configured root is not the working copy this instance made".into());
        }
        let dir = files::absolute(&copy.root)
            .map_err(|_| "working-copy root is missing or contains a symbolic link")?;
        if !copy.owns(&dir)? {
            return Err("configured root is not the working copy this instance made".into());
        }
        Ok(dir)
    }

    pub fn retire(&self, config: &Config) -> Result<()> {
        let Some(copy) = self.copy()? else {
            return Ok(());
        };
        if normalize_root(&config.root)? != copy.root {
            return Err("configured root is not the working copy this instance made".into());
        }
        match OpenedDirectory::open(&copy.root) {
            Ok(dir) if copy.owns(&dir.dir)? => dir.remove()?,
            Ok(_) | Err(Errno::NOTDIR | Errno::LOOP) => {
                return Err("configured root is not the working copy this instance made".into());
            }
            Err(Errno::NOENT) => {}
            Err(_) => return Err("cannot inspect working-copy root for retirement".into()),
        }
        self.remove_attempt(&copy)
    }

    pub fn changes(&self, root: &OwnedFd) -> Result<Value> {
        let copy = self.copy()?.ok_or("instance has not made a working copy")?;
        patch::take(&self.directory, &copy.base, root)
    }
}

fn normalize_root(root: &Path) -> Result<PathBuf> {
    let name = root
        .file_name()
        .ok_or("working-copy root must name a directory below its existing parent")?;
    let parent = root.parent().unwrap();
    files::absolute(parent).map_err(
        |_| "working-copy root parent is missing, not a directory, or contains a symbolic link",
    )?;
    let parent = parent
        .canonicalize()
        .map_err(|_| "working-copy root parent is unreadable")?;
    Ok(parent.join(name))
}

fn empty_root(root: &Path) -> Result<Option<(u64, u64, u32)>> {
    match fs::symlink_metadata(root) {
        Ok(meta) if meta.is_dir() => {
            let dir = files::absolute(root)
                .map_err(|_| "working-copy root is not an accessible directory")?;
            if !files::entries(&dir)
                .map_err(|_| "working-copy root is unreadable")?
                .is_empty()
            {
                return Err("working-copy root is not empty".into());
            }
            Ok(Some((meta.dev(), meta.ino(), meta.mode())))
        }
        Ok(_) => Err("working-copy root is not a directory or is a symbolic link".into()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("working-copy root is unreadable".into()),
    }
}

fn remove_dir(path: &Path) -> Result<()> {
    match OpenedDirectory::open(path) {
        Ok(dir) => dir.remove(),
        Err(Errno::NOENT) => Ok(()),
        Err(_) => Err("cannot safely open working-copy directory for removal".into()),
    }
}

struct Git<'a> {
    home: &'a Path,
    git_dir: &'a Path,
    executable: PathBuf,
    search_path: OsString,
}

impl<'a> Git<'a> {
    fn new(home: &'a Path, git_dir: &'a Path) -> Result<Self> {
        let path = std::env::var_os("PATH").ok_or("Git is missing from PATH")?;
        let cwd =
            std::env::current_dir().map_err(|_| "cannot resolve the runner's Git search path")?;
        let directories: Vec<_> = std::env::split_paths(&path)
            .map(|dir| cwd.join(dir))
            .collect();
        // Resolve relative PATH entries before running Git in the checkout.
        let executable = directories
            .iter()
            .map(|dir| dir.join("git"))
            .find(|git| {
                fs::metadata(git).is_ok_and(|meta| meta.is_file() && meta.mode() & 0o111 != 0)
            })
            .ok_or("Git is missing from PATH")?;
        let search_path =
            std::env::join_paths(&directories).map_err(|_| "invalid runner Git search path")?;
        Ok(Self {
            home,
            git_dir,
            executable,
            search_path,
        })
    }

    fn command(&self) -> Command {
        let mut git = Command::new(&self.executable);
        git.current_dir(self.home)
            .env_clear()
            .env("PATH", &self.search_path)
            .env("HOME", self.home)
            .env("XDG_CONFIG_HOME", self.home)
            .env("LC_ALL", "C")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_ATTR_NOSYSTEM", "1")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_ALLOW_PROTOCOL", "file")
            .env("GIT_DIR", self.git_dir)
            .args([
                "--no-pager",
                "-c",
                "core.hooksPath=/dev/null",
                "-c",
                "core.fsmonitor=false",
                "-c",
                "core.autocrlf=false",
                "-c",
                "core.filemode=true",
                "-c",
                "core.symlinks=true",
                "-c",
                "core.ignorecase=false",
                "-c",
                "core.precomposeunicode=false",
                "-c",
                "core.attributesFile=/dev/null",
                "-c",
                "core.excludesFile=/dev/null",
                "-c",
                "credential.helper=",
            ]);
        git
    }

    fn tree(&self, work_tree: &Path, index: &Path) -> Command {
        let mut command = self.command();
        command
            .current_dir(work_tree)
            .env("GIT_WORK_TREE", work_tree)
            .env("GIT_INDEX_FILE", index);
        command
    }

    fn check_version(&self) -> Result<()> {
        let version = git_output(self.command().arg("--version"), None)?;
        require(&version, "cannot read Git version")?;
        let version = std::str::from_utf8(&version.stdout)
            .unwrap_or("")
            .trim()
            .strip_prefix("git version ")
            .ok_or("cannot read Git version")?;
        let mut components = version.split('.');
        let major = components.next().and_then(|v| v.parse::<u32>().ok());
        let minor = components.next().and_then(|v| v.parse::<u32>().ok());
        if !matches!((major, minor), (Some(2), Some(32..)) | (Some(3..), Some(_))) {
            return Err(format!(
                "Git {version} is too old; Git 2.32 or later is required"
            ));
        }
        Ok(())
    }

    fn init(&self, tree: &Path, index: &Path) -> Result<()> {
        let init = git_output(
            self.tree(tree, index)
                .args(["init", "--quiet", "--template=", "--object-format=sha1"]),
            None,
        )?;
        require(&init, "cannot initialize private Git directory")?;
        self.attributes("* !filter !diff\n")
    }

    fn attributes(&self, content: &str) -> Result<()> {
        let info = self.git_dir.join("info");
        fs::create_dir_all(&info)
            .and_then(|()| fs::write(info.join("attributes"), content))
            .map_err(|_| "cannot initialize Git attributes".into())
    }

    fn validate_base(&self, commit: &str) -> Result<()> {
        let commit_type = git_output(self.command().args(["cat-file", "-t", commit]), None)?;
        if !commit_type.status.success() || commit_type.stdout != b"commit\n" {
            return Err("commit is not in the repository".into());
        }
        let tree = git_output(self.command().args(["ls-tree", "-r", "-z", commit]), None)?;
        require(&tree, "cannot list base tree")?;
        if tree
            .stdout
            .split(|b| *b == 0)
            .any(|entry| entry.starts_with(b"160000 "))
        {
            return Err("the base commit contains a Git link; submodules are unsupported".into());
        }
        Ok(())
    }

    fn write_view(&self, commit: &str, stage: &Path, index: &Path) -> Result<()> {
        let write = || -> std::io::Result<()> {
            let view = stage.join(".git");
            for directory in [
                "info",
                "objects/info",
                "objects/pack",
                "refs/heads",
                "refs/tags",
            ] {
                fs::create_dir_all(view.join(directory))?;
            }
            fs::write(view.join("HEAD"), format!("{commit}\n"))?;
            fs::copy(index, view.join("index"))?;
            fs::copy(
                self.git_dir.join("info/attributes"),
                view.join("info/attributes"),
            )?;
            let mut alternate = self
                .git_dir
                .join("objects")
                .into_os_string()
                .as_encoded_bytes()
                .to_vec();
            alternate.push(b'\n');
            fs::write(view.join("objects/info/alternates"), alternate)?;
            fs::write(
                view.join("config"),
                "[core]\n\
                 repositoryformatversion = 0\n\
                 bare = false\n\
                 logAllRefUpdates = false\n\
                 hooksPath = /dev/null\n\
                 fsmonitor = false\n\
                 autocrlf = false\n\
                 filemode = true\n\
                 symlinks = true\n\
                 ignorecase = false\n\
                 precomposeunicode = false\n\
                 attributesFile = /dev/null\n\
                 excludesFile = /dev/null\n\
                 [credential]\n\
                 helper =\n\
                 [protocol]\n\
                 allow = never\n\
                 [user]\n\
                 name =\n\
                 email =\n",
            )
        };
        write().map_err(|_| "cannot write working-copy Git view".into())
    }

    fn checkout(&self, commit: &str, stage: &Path, index: &Path) -> Result<()> {
        let checkout = git_output(
            self.tree(stage, index).args([
                "-c",
                "advice.detachedHead=false",
                "checkout",
                "--quiet",
                "--detach",
                "--no-recurse-submodules",
                commit,
            ]),
            None,
        )?;
        require(&checkout, "cannot check out base commit")
    }
}

fn git_output(command: &mut Command, input: Option<Vec<u8>>) -> Result<Output> {
    command
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            "Git is missing from PATH"
        } else {
            "cannot start Git"
        }
    })?;
    if let Some(input) = input {
        let mut stdin = child.stdin.take().unwrap();
        return std::thread::scope(|s| {
            let writer = s.spawn(move || stdin.write_all(&input));
            let out = child
                .wait_with_output()
                .map_err(|_| "cannot wait for Git")?;
            if writer
                .join()
                .ok()
                .and_then(std::result::Result::ok)
                .is_none()
                && out.status.success()
            {
                return Err("cannot send paths to Git".into());
            }
            Ok(out)
        });
    }
    child
        .wait_with_output()
        .map_err(|_| "cannot wait for Git".into())
}

fn require(output: &Output, message: &str) -> Result<()> {
    if output.status.success() {
        Ok(())
    } else {
        Err(message.into())
    }
}

#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod scratch_git;

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;

    #[test]
    fn changes_uses_the_open_copy_after_its_path_is_replaced() {
        let scratch = scratch_git::Scratch::new();
        let source = scratch.repository();
        fs::write(source.join("file.txt"), "base\n").unwrap();
        let base = scratch.commit(&source);
        let directory = scratch.path.join("state");
        fs::create_dir(&directory).unwrap();
        let state = State {
            _lock: File::create(directory.join("lock")).unwrap(),
            directory,
        };
        let config = Config {
            repository: source.clone(),
            commit: base.clone(),
            root: scratch.path.join("copy"),
        };
        state.setup(&config).unwrap();
        fs::write(config.root.join("file.txt"), "from owned directory\n").unwrap();
        let root = state.root(&config).unwrap();
        fs::rename(&config.root, scratch.path.join("moved-copy")).unwrap();
        fs::create_dir(&config.root).unwrap();
        fs::write(config.root.join("file.txt"), "from replacement\n").unwrap();

        let changes = state.changes(&root).unwrap();
        let applied = scratch.clone_at(&source, &base);
        scratch.git(&applied, &["apply", "--index"], changes["patch"].as_str());
        assert_eq!(
            fs::read(applied.join("file.txt")).unwrap(),
            b"from owned directory\n"
        );
        assert_eq!(
            fs::read(config.root.join("file.txt")).unwrap(),
            b"from replacement\n"
        );
    }

    #[test]
    fn retirement_keeps_the_parent_descriptor_after_ancestor_replacement() {
        let scratch = tempfile::tempdir().unwrap();
        let scratch = scratch.path().canonicalize().unwrap();
        let parent = scratch.join("parent");
        let outside = scratch.join("outside");
        fs::create_dir_all(parent.join("copy")).unwrap();
        fs::create_dir_all(outside.join("copy")).unwrap();
        fs::write(parent.join("copy/owned"), "owned").unwrap();
        fs::write(outside.join("copy/keep"), "outside").unwrap();
        let directory = OpenedDirectory::open(&parent.join("copy")).unwrap();

        let moved = scratch.join("moved-parent");
        fs::rename(&parent, &moved).unwrap();
        symlink(&outside, &parent).unwrap();
        directory.remove().unwrap();

        assert!(outside.join("copy/keep").is_file());
        assert!(!moved.join("copy").exists());
    }
}
