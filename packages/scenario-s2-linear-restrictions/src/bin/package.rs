//! Deterministic contracts-only USTAR archive with a fixed gzip header.
use anyhow::Result;
use flate2::{Compression, GzBuilder};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let gzip = GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(Vec::new(), Compression::default());
    let mut archive = tar::Builder::new(gzip);
    for name in ["conduit.yaml", "contracts.yaml", "README.md"] {
        let bytes = fs::read(root.join(name))?;
        let mut header = tar::Header::new_ustar();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o644);
        header.set_uid(0);
        header.set_gid(0);
        header.set_mtime(0);
        header.set_cksum();
        archive.append_data(&mut header, name, bytes.as_slice())?;
    }
    let bytes = archive.into_inner()?.finish()?;
    let digest = format!("{:x}", Sha256::digest(&bytes));
    fs::create_dir_all(root.join("dist"))?;
    let path = root.join("dist/scenario-s2-linear-restrictions.tar.gz");
    fs::write(&path, bytes)?;
    fs::write(path.with_extension("gz.sha256"), format!("{digest}\n"))?;
    println!("scenario-s2-linear-restrictions.tar.gz sha256:{digest}");
    Ok(())
}
