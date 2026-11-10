//! Reproduce repository-authored installation fixtures. Never fetches or runs a provider.
use flate2::{Compression, GzBuilder};
use std::{collections::BTreeMap, fs, io::Write, path::Path};

fn member(name: &str, data: &[u8], kind: u8, target: &str) -> Vec<u8> {
    let mut h = tar::Header::new_ustar();
    h.set_mode(0o644);
    h.set_uid(0);
    h.set_gid(0);
    h.set_mtime(0);
    h.set_size(data.len() as u64);
    h.set_entry_type(tar::EntryType::new(kind));
    // Deliberately bypass set_path's traversal guard for rejection fixtures.
    h.as_mut_bytes()[..100].fill(0);
    h.as_mut_bytes()[..name.len()].copy_from_slice(name.as_bytes());
    if !target.is_empty() {
        h.set_link_name(target).unwrap();
    }
    h.set_cksum();
    let mut bytes = h.as_bytes().to_vec();
    bytes.extend(data);
    bytes.resize(bytes.len().div_ceil(512) * 512, 0);
    bytes
}
fn archive(mut members: BTreeMap<String, (Vec<u8>, u8, String)>) -> Vec<u8> {
    let mut tar = vec![];
    for (name, (data, kind, target)) in &mut members {
        tar.extend(member(name, data, *kind, target));
    }
    tar.extend([0; 1024]);
    gzip(&tar)
}
fn gzip(tar: &[u8]) -> Vec<u8> {
    let mut gz = GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(vec![], Compression::best());
    gz.write_all(tar).unwrap();
    gz.finish().unwrap()
}
fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let out = root.join("conformance/fixtures/install-archives");
    let check = std::env::args().any(|a| a == "--check");
    if !check {
        fs::create_dir_all(&out).unwrap();
    }
    let package = root.join("conformance/fixtures/s1/packages/filesystem");
    let manifest = fs::read(package.join("conduit.yaml")).unwrap();
    let docs = fs::read(package.join("contracts.yaml")).unwrap();
    let base = BTreeMap::from([
        (
            "conduit.yaml".into(),
            (manifest.clone(), b'0', String::new()),
        ),
        ("contracts.yaml".into(), (docs.clone(), b'0', String::new())),
    ]);
    let mut generated = BTreeMap::new();
    for (name, changed) in [
        (
            "wrong-address",
            String::from_utf8(manifest.clone())
                .unwrap()
                .replace("example.org/s1/filesystem", "example.org/s1/another"),
        ),
        (
            "wrong-version",
            String::from_utf8(manifest.clone())
                .unwrap()
                .replace("version: 0.1.0", "version: 0.2.0"),
        ),
        (
            "missing-dependency",
            String::from_utf8(manifest.clone()).unwrap()
                + "\npackages:\n  example.org/missing:\n    version: 1.0.0\n    digest: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n",
        ),
    ] {
        let mut entries = base.clone();
        entries.get_mut("conduit.yaml").unwrap().0 = changed.into_bytes();
        generated.insert(name.to_string(), archive(entries));
    }
    // Reuse the actual document's apiVersion rather than writing an assigned name here.
    let api = String::from_utf8(docs)
        .unwrap()
        .lines()
        .find(|l| {
            l.split_once(':')
                .is_some_and(|(key, _)| key == "apiVersion")
        })
        .unwrap()
        .to_owned();
    let instance =
        format!("{api}\nkind: Pool\nmetadata:\n  name: unwanted\nfrom: unused\n").into_bytes();
    for (name, path, data, kind, target) in [
        ("instance", "instance.yaml", instance, b'0', ""),
        (
            "dot-entry",
            "._contracts.yaml",
            b"not a document\0\xff".to_vec(),
            b'0',
            "",
        ),
        ("traversal", "../escape", b"outside".to_vec(), b'0', ""),
        ("absolute", "/escape", b"outside".to_vec(), b'0', ""),
        ("link-escape", "escape", vec![], b'2', "../outside"),
        ("link-absolute", "escape", vec![], b'2', "/outside"),
        ("hardlink-escape", "escape", vec![], b'1', "../outside"),
        ("symlink", "linked.yaml", vec![], b'2', "contracts.yaml"),
        ("dangling", "missing.yaml", vec![], b'2', "absent"),
        ("link-cycle", "cycle", vec![], b'2', "cycle/next"),
        (
            "invalid-document",
            "invalid.yaml",
            b"kind: invented\n".to_vec(),
            b'0',
            "",
        ),
        (
            "nested-document",
            "nested/invalid.yaml",
            b"not a document\0\xff".to_vec(),
            b'0',
            "",
        ),
        (
            "program",
            "bin/server",
            b"repository-authored inert program bytes\n".to_vec(),
            b'0',
            "",
        ),
    ] {
        let mut entries = base.clone();
        entries.insert(path.into(), (data, kind, target.into()));
        // A root link to an existing document replaces it; avoid duplicate document names.
        if name == "symlink" {
            let contracts = entries.remove("contracts.yaml").unwrap();
            entries.insert("data/contracts".into(), contracts);
            entries.get_mut("linked.yaml").unwrap().2 = "data/contracts".into();
        }
        generated.insert(name.to_string(), archive(entries));
    }
    let nested = base
        .iter()
        .map(|(k, v)| (format!("package/{k}"), v.clone()))
        .collect();
    generated.insert("nested-manifest".into(), archive(nested));
    let mut indirect = base.clone();
    indirect.insert("a".into(), (vec![], b'2', ".".into()));
    indirect.insert("escape".into(), (vec![], b'2', "a/../outside".into()));
    generated.insert("indirect-link-escape".into(), archive(indirect));
    // Lookup must visit every directory, even when `..` later removes its name.
    for (name, prefix, kind) in [
        ("manifest-file-parent", "payload", Some(b'0')),
        ("manifest-missing-parent", "absent", None),
        ("manifest-directory-parent", "payload", Some(b'5')),
    ] {
        let mut entries = base.clone();
        let manifest = entries.remove("conduit.yaml").unwrap();
        entries.insert("manifest".into(), manifest);
        if let Some(kind) = kind {
            entries.insert(prefix.into(), (vec![], kind, String::new()));
        }
        entries.insert(
            "conduit.yaml".into(),
            (vec![], b'2', format!("{prefix}/../manifest")),
        );
        generated.insert(name.into(), archive(entries));
    }
    for expansions in [39, 40, 41] {
        let mut entries = base.clone();
        let manifest = entries.remove("conduit.yaml").unwrap();
        entries.insert("manifest".into(), manifest);
        entries.insert("conduit.yaml".into(), (vec![], b'2', "link-01".into()));
        for i in 1..expansions {
            let target = if i + 1 == expansions {
                "manifest".into()
            } else {
                format!("link-{:02}", i + 1)
            };
            entries.insert(format!("link-{i:02}"), (vec![], b'2', target));
        }
        generated.insert(format!("link-expansions-{expansions}"), archive(entries));
    }
    // A hard link to an earlier regular file supplies the manifest at the root.
    let hard = BTreeMap::from([
        ("a-manifest".into(), (manifest, b'0', String::new())),
        ("conduit.yaml".into(), (vec![], b'1', "a-manifest".into())),
        ("contracts.yaml".into(), base["contracts.yaml"].clone()),
    ]);
    generated.insert("hardlink".into(), archive(hard));
    generated.insert("not-gzip".into(), b"not a gzip archive".to_vec());
    generated.insert("not-tar".into(), gzip(b"not a tar archive"));
    for (name, bytes) in &generated {
        let path = out.join(format!("{name}.tar.gz"));
        if check {
            assert_eq!(fs::read(&path).unwrap(), *bytes, "{}", path.display());
        } else {
            fs::write(path, bytes).unwrap();
        }
        println!("{name}: {}", conduit_core::packages::bytes_digest(bytes));
    }
    let digests: BTreeMap<_, _> = generated
        .iter()
        .map(|(name, bytes)| {
            (
                format!("{name}.tar.gz"),
                conduit_core::packages::bytes_digest(bytes),
            )
        })
        .collect();
    let mut digest_bytes = serde_json::to_vec_pretty(&digests).unwrap();
    digest_bytes.push(b'\n');
    let digest_path = out.join("digests.json");
    if check {
        assert_eq!(fs::read(digest_path).unwrap(), digest_bytes);
    } else {
        fs::write(digest_path, digest_bytes).unwrap();
    }
    println!(
        "{} reproducible archive fixtures {}",
        generated.len(),
        if check { "verified" } else { "written" }
    );
}
