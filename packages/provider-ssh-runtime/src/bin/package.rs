//! Host packaging with deterministic USTAR metadata and gzip header.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
macro_rules! ensure {
    ($condition:expr, $message:expr) => {
        if !$condition {
            return Err($message.into());
        }
    };
}
use flate2::{Compression, GzBuilder};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn run() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // Build the provider once with cargo build --release --bins before packaging.
    // Never nest a cargo command inside this cargo-run binary.
    let binary = std::env::current_exe()?.with_file_name(provider_ssh_runtime::BINARY);
    ensure!(
        binary.is_file(),
        "build provider-ssh-runtime with cargo build --release --bins first"
    );
    let drainer = binary.with_file_name(provider_ssh_runtime::startup::DRAINER);
    ensure!(
        drainer.is_file(),
        "build provider-ssh-runtime-drain with cargo build --release --bins first"
    );
    let gzip = GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(Vec::new(), Compression::default());
    let mut tar = tar::Builder::new(gzip);
    for (name, bytes, mode) in [
        (
            format!("bin/{}", provider_ssh_runtime::BINARY),
            fs::read(binary)?,
            0o755,
        ),
        (
            format!("bin/{}", provider_ssh_runtime::startup::DRAINER),
            fs::read(drainer)?,
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
    let path = root.join(format!("dist/{}.tar.gz", provider_ssh_runtime::BINARY));
    fs::write(&path, &bytes)?;
    fs::write(path.with_extension("gz.sha256"), format!("{digest}\n"))?;
    println!("{}#{digest}", path.display());
    Ok(())
}
fn main() -> Result<()> {
    run()
}
