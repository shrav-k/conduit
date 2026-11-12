use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path, process::Command};

#[test]
fn archive_is_deterministic_and_contains_exact_package_bytes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let pack = || {
        let output = Command::new(env!("CARGO_BIN_EXE_package"))
            .output()
            .unwrap();
        assert!(output.status.success(), "package build failed");
        assert!(output.stderr.is_empty());
    };
    pack();
    let path = root.join("dist/scenario-s4-effect.tar.gz");
    let bytes = fs::read(&path).unwrap();
    let digest = fs::read_to_string(path.with_extension("gz.sha256")).unwrap();
    assert!(digest == format!("{:x}\n", Sha256::digest(&bytes)));
    pack();
    assert!(fs::read(&path).unwrap() == bytes);
    assert!(fs::read_to_string(path.with_extension("gz.sha256")).unwrap() == digest);
    assert!(bytes[4..8] == [0; 4] && bytes[9] == 255);
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(bytes.as_slice()));
    let mut names = Vec::new();
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let name = entry.path().unwrap().display().to_string();
        let h = entry.header();
        assert!(h.as_ustar().is_some());
        assert!(h.uid().unwrap() == 0 && h.gid().unwrap() == 0 && h.mtime().unwrap() == 0);
        assert!(
            h.mode().unwrap()
                == if name.starts_with("bin/") {
                    0o755
                } else {
                    0o644
                }
        );
        let mut content = Vec::new();
        entry.read_to_end(&mut content).unwrap();
        let source = if name.starts_with("bin/") {
            Path::new(env!("CARGO_BIN_EXE_scenario-s4-effect")).to_path_buf()
        } else {
            root.join(&name)
        };
        assert!(content == fs::read(source).unwrap());
        names.push(name);
    }
    assert!(
        names
            == [
                "bin/scenario-s4-effect",
                "conduit.yaml",
                "contracts.yaml",
                "README.md"
            ]
    );
}
