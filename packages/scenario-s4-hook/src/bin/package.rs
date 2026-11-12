//! Host packaging with deterministic USTAR metadata and gzip header.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use flate2::{Compression, GzBuilder};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn run() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // A build is explicit: never start a nested Cargo command from package or tests.
    let binary = std::env::current_exe()?
        .parent()
        .unwrap()
        .join(scenario_s4_hook::BINARY);
    if !binary.is_file() {
        return Err("build the release server binary before packaging".into());
    }
    let gzip = GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(Vec::new(), Compression::default());
    let mut tar = tar::Builder::new(gzip);
    for (name, bytes, mode) in [
        (
            format!("bin/{}", scenario_s4_hook::BINARY),
            fs::read(binary)?,
            0o755,
        ),
        (
            "conduit.yaml".into(),
            fs::read(root.join("conduit.yaml"))?,
            0o644,
        ),
        (
            "contracts.yaml".into(),
            fs::read(root.join("contracts.yaml"))?,
            0o644,
        ),
        ("README.md".into(), fs::read(root.join("README.md"))?, 0o644),
    ] {
        let mut h = tar::Header::new_ustar();
        h.set_size(bytes.len() as u64);
        h.set_mode(mode);
        h.set_uid(0);
        h.set_gid(0);
        h.set_mtime(0);
        h.set_cksum();
        tar.append_data(&mut h, name, bytes.as_slice())?;
    }
    let bytes = tar.into_inner()?.finish()?;
    let digest = format!("{:x}", Sha256::digest(&bytes));
    fs::create_dir_all(root.join("dist"))?;
    let path = root.join(format!("dist/{}.tar.gz", scenario_s4_hook::BINARY));
    fs::write(&path, &bytes)?;
    fs::write(path.with_extension("gz.sha256"), format!("{digest}\n"))?;
    println!("scenario-s4-hook.tar.gz sha256:{digest}");
    Ok(())
}
fn main() -> Result<()> {
    run()
}
