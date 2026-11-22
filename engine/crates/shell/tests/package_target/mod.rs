use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

pub fn package_target_dir(directory: &Path) -> Option<PathBuf> {
    let cache = std::env::var_os("CONDUIT_TEST_TARGET_CACHE").filter(|path| !path.is_empty())?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let invalid = || -> ! {
        panic!(
            "not a package or experiment Rust directory: {}",
            directory.display()
        )
    };
    let path = directory.canonicalize().unwrap_or_else(|_| invalid());
    let relative = path.strip_prefix(root).unwrap_or_else(|_| invalid());
    let parts: Vec<_> = relative.iter().collect();
    match parts.as_slice() {
        [area, _] if *area == "packages" || *area == "experiments" => {}
        [area, _, _] if *area == "experiments" => {}
        _ => invalid(),
    }
    let mut name = OsString::new();
    for (index, part) in parts.into_iter().enumerate() {
        if index > 0 {
            name.push("-");
        }
        name.push(part);
    }
    Some(PathBuf::from(cache).join(name))
}
