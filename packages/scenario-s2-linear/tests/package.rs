use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path, process::Command};

#[test]
fn deterministic_host_archive_contains_only_declarations_readme_and_executable() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let pack = || {
        let output = Command::new(env!("CARGO_BIN_EXE_package"))
            .output()
            .unwrap();
        assert!(output.status.success(), "package build failed");
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    };
    let first = pack();
    let digest = first.split_once("sha256:").unwrap().1;
    let path = root.join("dist/scenario-s2-linear.tar.gz");
    let bytes = fs::read(&path).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), digest);
    assert_eq!(
        fs::read_to_string(path.with_extension("gz.sha256")).unwrap(),
        format!("{digest}\n")
    );
    assert_eq!(pack(), first);
    assert_eq!(fs::read(path).unwrap(), bytes, "archive bytes differ");
    assert_eq!(&bytes[4..8], &[0; 4]);
    assert_eq!(bytes[9], 255);
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(bytes.as_slice()));
    let mut names = Vec::new();
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let name = entry.path().unwrap().display().to_string();
        let header = entry.header();
        assert!(header.as_ustar().is_some());
        assert_eq!(header.uid().unwrap(), 0);
        assert_eq!(header.gid().unwrap(), 0);
        assert_eq!(header.mtime().unwrap(), 0);
        assert_eq!(
            header.mode().unwrap(),
            if name.starts_with("bin/") {
                0o755
            } else {
                0o644
            }
        );
        assert_eq!(header.username().unwrap(), Some(""));
        assert_eq!(header.groupname().unwrap(), Some(""));
        let mut content = Vec::new();
        entry.read_to_end(&mut content).unwrap();
        let source = if name.starts_with("bin/") {
            Path::new(env!("CARGO_BIN_EXE_scenario-s2-linear")).to_path_buf()
        } else {
            root.join(&name)
        };
        assert_eq!(content, fs::read(source).unwrap());
        names.push(name);
    }
    assert_eq!(
        names,
        [
            "bin/scenario-s2-linear",
            "conduit.yaml",
            "contracts.yaml",
            "README.md"
        ]
    );
}
