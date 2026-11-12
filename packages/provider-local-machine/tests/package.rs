use sha2::{Digest, Sha256};
use std::{fs, io::Read, process::Command};
#[test]
fn deterministic_host_archive_metadata_and_digest() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let pack = || {
        let o = Command::new(env!("CARGO_BIN_EXE_package"))
            .output()
            .unwrap();
        assert!(o.status.success(), "package build failed");
        String::from_utf8(o.stdout).unwrap().trim().to_string()
    };
    let first = pack();
    let hash = first.split_once("sha256:").unwrap().1;
    let archive_path = root.join("dist/provider-local-machine.tar.gz");
    let path = archive_path.to_str().unwrap();
    let bytes = fs::read(path).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), hash);
    assert_eq!(
        fs::read_to_string(format!("{path}.sha256")).unwrap(),
        format!("{hash}\n")
    );
    assert_eq!(pack(), first);
    assert!(fs::read(path).unwrap() == bytes, "archive bytes differ");
    assert_eq!(&bytes[4..8], &[0; 4]);
    let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(bytes.as_slice()));
    let mut names = Vec::new();
    for e in tar.entries().unwrap() {
        let mut e = e.unwrap();
        let name = e.path().unwrap().display().to_string();
        let h = e.header();
        assert!(h.as_ustar().is_some());
        assert_eq!(h.uid().unwrap(), 0);
        assert_eq!(h.gid().unwrap(), 0);
        assert_eq!(h.mtime().unwrap(), 0);
        assert_eq!(
            h.mode().unwrap(),
            if name.starts_with("bin/") {
                0o755
            } else {
                0o644
            }
        );
        let mut content = Vec::new();
        e.read_to_end(&mut content).unwrap();
        if name.ends_with(".yaml") || name.ends_with(".md") {
            assert!(
                content == fs::read(root.join(&name)).unwrap(),
                "archive document differs"
            );
        }
        names.push(name);
    }
    assert_eq!(
        names,
        vec![
            format!("bin/{}", provider_local_machine::BINARY),
            "conduit.yaml".into(),
            "contracts.yaml".into(),
            "README.md".into()
        ]
    );
}
