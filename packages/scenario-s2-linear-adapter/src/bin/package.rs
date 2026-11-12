use flate2::{Compression, GzBuilder};
use scenario_s2_linear_adapter::BINARY;
use sha2::{Digest, Sha256};
use std::{fs, io, path::Path};

fn package() -> io::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // Consume the adjacent server built by cargo test/build; do not nest Cargo commands.
    let binary = std::env::current_exe()?
        .with_file_name(format!("{BINARY}{}", std::env::consts::EXE_SUFFIX));
    let gzip = GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(Vec::new(), Compression::default());
    let mut archive = tar::Builder::new(gzip);
    for (name, bytes, mode) in [
        (format!("bin/{BINARY}"), fs::read(binary)?, 0o755),
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
        let mut header = tar::Header::new_ustar();
        header.set_size(bytes.len() as u64);
        header.set_mode(mode);
        header.set_uid(0);
        header.set_gid(0);
        header.set_mtime(0);
        header.set_cksum();
        archive.append_data(&mut header, name, bytes.as_slice())?;
    }
    let bytes = archive.into_inner()?.finish()?;
    let digest = format!("{:x}", Sha256::digest(&bytes));
    fs::create_dir_all(root.join("dist"))?;
    let path = root.join(format!("dist/{BINARY}.tar.gz"));
    fs::write(&path, bytes)?;
    fs::write(path.with_extension("gz.sha256"), format!("{digest}\n"))?;
    println!("{BINARY}.tar.gz sha256:{digest}");
    Ok(())
}
fn main() {
    if package().is_err() {
        eprintln!("Package failed; build the adjacent release server binary first");
        std::process::exit(1);
    }
}
