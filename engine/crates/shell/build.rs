use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8(output.stdout).ok()?.trim_end().into())
}

fn git_path(name: &str) -> Option<PathBuf> {
    git(&["rev-parse", "--path-format=absolute", "--git-path", name]).map(PathBuf::from)
}

fn watch(path: &Path) {
    if path.exists() {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

fn watch_ref_chain(reftable: bool) {
    let mut reference = String::from("HEAD");
    let mut visited = HashSet::new();
    while visited.insert(reference.clone()) {
        if let Some(path) = git_path(&reference) {
            if path.exists() {
                watch(&path);
            } else if !reftable
                && reference != "HEAD"
                && let Some(directory) = path.ancestors().skip(1).find(|path| path.is_dir())
            {
                // Updating a packed branch can create a loose ref without changing packed-refs.
                watch(directory);
            }
        }
        let Some(next) = git(&["symbolic-ref", "--quiet", "--no-recurse", &reference]) else {
            break;
        };
        reference = next;
    }
    if visited.len() > 1
        && let Some(packed) = git_path("packed-refs")
    {
        watch(&packed);
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let reftable = git(&["rev-parse", "--show-ref-format"]).as_deref() == Some("reftable");
    watch_ref_chain(reftable);
    if reftable {
        for option in ["--git-common-dir", "--git-dir"] {
            if let Some(directory) = git(&["rev-parse", "--path-format=absolute", option]) {
                watch(&PathBuf::from(directory).join("reftable/tables.list"));
            }
        }
    }

    let commit = git(&["rev-parse", "HEAD"])
        .filter(|hash| hash.len() == 40 && hash.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=CLI_GIT_COMMIT={commit}");
}
