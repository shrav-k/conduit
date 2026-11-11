use crate::{Result, boundary};
use conduit_core::packages::{Snapshot, SnapshotSource};
use conduit_loader::Specification;
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
