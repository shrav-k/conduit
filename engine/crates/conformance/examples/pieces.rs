//! File I/O for the pure pieces module.
use conduit_conformance::pieces::{self, Files};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, String>;

fn read_tree(root: &Path) -> Result<(Files, BTreeSet<String>)> {
    fn visit(
        root: &Path,
        directory: &Path,
        files: &mut Files,
        cases: &mut BTreeSet<String>,
    ) -> Result<()> {
        for entry in
            std::fs::read_dir(directory).map_err(|e| format!("{}: {e}", directory.display()))?
        {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.is_dir() {
                visit(root, &path, files, cases)?;
            } else {
                let relative = path
                    .strip_prefix(root)
                    .map_err(|e| e.to_string())?
                    .to_string_lossy()
                    .into_owned();
                if path.file_name().is_some_and(|n| n == "case.yaml") {
                    cases.insert(relative.clone());
                }
                if relative == "manifest.yaml"
                    || relative == "case.schema.json"
                    || path.file_name().is_some_and(|n| n == "index.yaml")
                    || relative.starts_with("case-schema/")
                        && path.extension().is_some_and(|e| e == "json")
                {
                    files.insert(
                        relative,
                        std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?,
                    );
                }
            }
        }
        Ok(())
    }
    let mut files = Files::new();
    let mut cases = BTreeSet::new();
    let directory = root.join("conformance");
    visit(&directory, &directory, &mut files, &mut cases)?;
    Ok((files, cases))
}
fn run() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let (files, cases) = read_tree(&root)?;
    let (mode, base) = match args.as_slice() {
        [] => (None, None),
        [flag, directory] if flag == "--merge" || flag == "--compare" => {
            (Some(flag), Some(read_tree(Path::new(directory))?.0))
        }
        _ => return Err("usage: pieces [--merge ROOT | --compare ROOT]".into()),
    };
    if mode.is_some_and(|m| m == "--compare") {
        println!("{}", pieces::compare(&files, base.as_ref().unwrap())?);
        return Ok(());
    }
    let changes = pieces::split(&files, base.as_ref(), &cases)?;
    if changes.is_empty() {
        println!("Nothing to move.");
    } else {
        for (name, bytes) in &changes {
            let path = root.join("conformance").join(name);
            match bytes {
                Some(bytes) => {
                    std::fs::create_dir_all(path.parent().ok_or("piece has no parent")?)
                        .map_err(|e| e.to_string())?;
                    std::fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
                }
                None => {
                    std::fs::remove_file(&path).map_err(|e| format!("{}: {e}", path.display()))?
                }
            }
        }
        println!("Updated {} files.", changes.len());
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
