//! Host packaging with deterministic USTAR metadata and gzip header.
use anyhow::{Result, ensure};
use flate2::{Compression, GzBuilder};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn run() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    ensure!(
        Command::new("cargo")
            .current_dir(root)
            .args([
                "build",
                "--locked",
                "--release",
                "--bin",
                agent_adapter::WRAPPER
            ])
            .status()?
            .success(),
        "wrapper build failed"
    );
    // Locate custom Cargo target directories through cargo metadata, rather than assumptions.
    let metadata = Command::new("cargo")
        .current_dir(root)
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()?;
    ensure!(metadata.status.success(), "cargo metadata failed");
    let metadata: serde_json::Value = serde_json::from_slice(&metadata.stdout)?;
    let binary = PathBuf::from(metadata["target_directory"].as_str().unwrap())
        .join("release")
        .join(agent_adapter::WRAPPER);
    let gzip = GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(Vec::new(), Compression::default());
    let mut tar = tar::Builder::new(gzip);
    for (name, bytes, mode) in [
        (
            format!("bin/{}", agent_adapter::WRAPPER),
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
    let path = root.join(format!("dist/{}.tar.gz", agent_adapter::WRAPPER));
    fs::write(&path, &bytes)?;
    fs::write(path.with_extension("gz.sha256"), format!("{digest}\n"))?;
    println!("{}#{digest}", path.display());
    Ok(())
}
fn main() -> Result<()> {
    run()
}
