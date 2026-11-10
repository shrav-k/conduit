//! Rebuild a repository-authored S1 package archive with reproducible headers.
use flate2::{Compression, GzBuilder};
use std::{fs, path::PathBuf};
fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    assert_eq!(args.len(), 2, "usage: s1-archive PACKAGE_DIRECTORY OUTPUT");
    let directory = PathBuf::from(&args[0]);
    let mut archive = tar::Builder::new(Vec::new());
    for name in ["conduit.yaml", "contracts.yaml"] {
        let bytes = fs::read(directory.join(name)).unwrap();
        let mut header = tar::Header::new_ustar();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o644);
        header.set_uid(0);
        header.set_gid(0);
        header.set_mtime(0);
        header.set_cksum();
        archive
            .append_data(&mut header, name, bytes.as_slice())
            .unwrap();
    }
    let tar = archive.into_inner().unwrap();
    let mut gzip = GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(Vec::new(), Compression::best());
    std::io::Write::write_all(&mut gzip, &tar).unwrap();
    let bytes = gzip.finish().unwrap();
    fs::write(&args[1], &bytes).unwrap();
    println!("{}", conduit_core::packages::bytes_digest(&bytes));
}
