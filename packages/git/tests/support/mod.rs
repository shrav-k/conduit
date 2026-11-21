use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub struct Scratch {
    _temp: tempfile::TempDir,
    pub path: PathBuf,
    home: PathBuf,
}

impl Scratch {
    pub fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().canonicalize().unwrap();
        let home = path.join("home");
        fs::create_dir(&home).unwrap();
        Self {
            _temp: temp,
            path,
            home,
        }
    }

    pub fn git(&self, directory: &Path, args: &[&str], input: Option<&str>) -> String {
        assert!(directory.canonicalize().unwrap().starts_with(&self.path));
        let mut command = Command::new("git");
        command
            .args(args)
            .current_dir(directory)
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap())
            .env("HOME", &self.home)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_AUTHOR_NAME", "Conduit test")
            .env("GIT_AUTHOR_EMAIL", "test@conduit.invalid")
            .env("GIT_COMMITTER_NAME", "Conduit test")
            .env("GIT_COMMITTER_EMAIL", "test@conduit.invalid")
            .stdin(if input.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().unwrap();
        if let Some(input) = input {
            child
                .stdin
                .take()
                .unwrap()
                .write_all(input.as_bytes())
                .unwrap();
        }
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    }

    pub fn repository(&self) -> PathBuf {
        let repository = self.path.join("source");
        fs::create_dir(&repository).unwrap();
        self.git(
            &repository,
            &["init", "--quiet", "--template=", "--object-format=sha1"],
            None,
        );
        repository
    }

    pub fn commit(&self, repository: &Path) -> String {
        self.git(repository, &["add", "--all"], None);
        self.git(
            repository,
            &["commit", "--quiet", "-m", "scratch commit"],
            None,
        );
        self.git(repository, &["rev-parse", "HEAD"], None)
            .trim()
            .to_owned()
    }

    pub fn clone_at(&self, repository: &Path, base: &str) -> PathBuf {
        let applied = self.path.join("applied");
        assert!(repository.canonicalize().unwrap().starts_with(&self.path));
        self.git(
            &self.path,
            &[
                "clone",
                "--quiet",
                "--no-checkout",
                "--template=",
                "--no-hardlinks",
                "--",
                repository.to_str().unwrap(),
                applied.to_str().unwrap(),
            ],
            None,
        );
        self.git(&applied, &["checkout", "--quiet", "--detach", base], None);
        applied
    }
}
