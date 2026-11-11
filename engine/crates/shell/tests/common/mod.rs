use conduit_core::packages::{Snapshot, SnapshotSource};
use conduit_loader::Specification;
use conduit_shell::{Result, boundary};
use std::path::Path;

pub fn snapshots(repository: &Path) -> Result<Vec<Snapshot>> {
    let spec = Specification::read(&repository.join("spec")).map_err(|e| e.to_string())?;
    let fixture =
        boundary::value(&repository.join("conformance/language/registration/s1/fixture.json"))?;
    fixture["snapshots"]
        .as_array()
        .ok_or("missing S1 snapshots")?
        .iter()
        .map(|recipe| {
            let address = recipe["address"]
                .as_str()
                .ok_or("missing snapshot address")?;
            let name = address.rsplit('/').next().ok_or("missing package name")?;
            let archive = std::fs::read(
                repository.join(format!("conformance/fixtures/s1/archives/{name}.tar.gz")),
            )
            .map_err(|e| e.to_string())?;
            if conduit_core::packages::bytes_digest(&archive) != recipe["digest"] {
                return Err("S1 snapshot digest mismatch".into());
            }
            Ok(Snapshot {
                source: SnapshotSource::PreinstalledSnapshot,
                address: address.into(),
                version: recipe["version"].as_str().ok_or("missing version")?.into(),
                digest: recipe["digest"].as_str().ok_or("missing digest")?.into(),
                project: boundary::project(
                    &repository.join(format!("conformance/fixtures/s1/packages/{name}")),
                    &spec,
                )?,
            })
        })
        .collect()
}

/// Explicit library archive installation for test packages, including scripted variants.
/// The portable vectors keep their original snapshots in fixtures instead.
pub fn install(
    shell: &conduit_shell::Shell,
    repository: &Path,
    snapshots: Vec<Snapshot>,
    project: &mut conduit_core::packages::Project,
) {
    use std::collections::BTreeMap;
    let temporary = tempfile::tempdir().unwrap();
    let mut pins: BTreeMap<String, String> = BTreeMap::new();
    let originals = self::snapshots(repository).unwrap();
    for mut snapshot in snapshots {
        let mut manifest = snapshot.project.manifest.take().unwrap();
        for (address, digest) in &pins {
            if manifest["packages"].get(address).is_some() {
                manifest["packages"][address]["digest"] = serde_json::json!(digest);
            }
        }
        let unchanged = originals.iter().any(|old| {
            old.address == snapshot.address
                && old.project.manifest.as_ref() == Some(&manifest)
                && old.project.documents == snapshot.project.documents
        });
        let mut files = vec![("conduit.json".to_string(), manifest)];
        files.extend(
            snapshot
                .project
                .documents
                .into_iter()
                .enumerate()
                .map(|(i, d)| (format!("document-{i}.json"), d)),
        );
        let gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        let mut tar = tar::Builder::new(gzip);
        for (name, value) in files {
            let bytes = serde_json::to_vec(&value).unwrap();
            let mut header = tar::Header::new_ustar();
            header.set_size(bytes.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append_data(&mut header, name, bytes.as_slice())
                .unwrap();
        }
        let mut bytes = tar.into_inner().unwrap().finish().unwrap();
        if unchanged {
            bytes = std::fs::read(repository.join(format!(
                "conformance/fixtures/s1/archives/{}.tar.gz",
                snapshot.address.rsplit('/').next().unwrap()
            )))
            .unwrap();
        }
        let digest = conduit_core::packages::bytes_digest(&bytes);
        let path = temporary.path().join("package.tar.gz");
        std::fs::write(&path, bytes).unwrap();
        boundary::install_package(shell, repository, &format!("{}#{digest}", path.display()))
            .unwrap();
        pins.insert(snapshot.address, digest);
    }
    for (address, digest) in pins {
        if project.manifest.as_ref().unwrap()["packages"]
            .get(&address)
            .is_some()
        {
            project.manifest.as_mut().unwrap()["packages"][&address]["digest"] =
                serde_json::json!(digest);
        }
    }
}
