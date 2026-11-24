//! Recover the authored dependency order after boundary metadata validation.
use super::Requirement;
use crate::Result;
use crate::boundary::archive_metadata::{resolve, unpack};
use conduit_core::archives::Member;
use serde_json::Value;

pub(super) fn dependencies(bytes: &[u8], manifest: &Value) -> Result<Vec<Requirement>> {
    let Some(items) = manifest["packages"].as_object() else {
        return Ok(Vec::new());
    };
    if items.len() <= 1 {
        return Ok(items
            .iter()
            .filter_map(|(address, value)| Requirement::read(address, value))
            .collect());
    }
    // JSON values discard mapping order. The archive metadata reader has already
    // validated the members and links. Read the same manifest as a loader Node.
    let members = unpack(bytes).map_err(|(rule, message)| format!("{rule}: {message}"))?;
    for name in conduit_loader::MANIFEST_NAMES {
        let Some(Member::File { bytes, .. }) =
            resolve(&members, name).map_err(|(rule, message)| format!("{rule}: {message}"))?
        else {
            continue;
        };
        let syntax = conduit_loader::Syntax::of(name).ok_or("unknown manifest syntax")?;
        let nodes = conduit_loader::read_bytes(syntax, bytes).map_err(|error| error.to_string())?;
        let node = &nodes[0];
        if node.to_json() != *manifest {
            return Err("cannot determine the archive manifest's written order".into());
        }
        if let Some(conduit_loader::Node {
            value: conduit_loader::Value::Object(entries),
            ..
        }) = node.get("packages")
        {
            return Ok(entries
                .iter()
                .filter_map(|(address, item)| Requirement::read(address, &item.to_json()))
                .collect());
        }
    }
    Err("cannot determine the archive manifest's written order".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::{Compression, write::GzEncoder};
    use std::io::Write;

    const MANIFEST: &[u8] = b"apiVersion: github.com/shrav-k/conduit/v0
address: example.org/fixture
version: 1.0.0
packages:
  example.org/b:
    version: 1.0.0
    digest: sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
  example.org/a:
    version: 1.0.0
    digest: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
";

    fn member(
        name: &[u8],
        kind: u8,
        data: &[u8],
        target: &[u8],
        amend: impl FnOnce(&mut tar::Header),
    ) -> Vec<u8> {
        let mut header = tar::Header::new_ustar();
        header.set_entry_type(tar::EntryType::new(kind));
        header.set_mode(0o644);
        header.set_uid(0);
        header.set_gid(0);
        header.set_mtime(0);
        header.set_size(data.len() as u64);
        // Supply the name bytes directly so the fixture can exercise the reader's validation.
        header.as_mut_bytes()[..name.len()].copy_from_slice(name);
        header.as_mut_bytes()[157..157 + target.len()].copy_from_slice(target);
        amend(&mut header);
        header.set_cksum();
        let mut bytes = header.as_bytes().to_vec();
        bytes.extend_from_slice(data);
        bytes.resize(bytes.len().div_ceil(512) * 512, 0);
        bytes
    }

    fn file(name: &[u8], data: &[u8]) -> Vec<u8> {
        member(name, b'0', data, b"", |_| {})
    }

    fn link(name: &[u8], target: &[u8], kind: u8) -> Vec<u8> {
        member(name, kind, b"", target, |_| {})
    }

    fn terminated(members: impl IntoIterator<Item = Vec<u8>>) -> Vec<u8> {
        let mut bytes: Vec<_> = members.into_iter().flatten().collect();
        bytes.extend([0; 1024]);
        bytes
    }

    fn gzip(bytes: &[u8]) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(bytes).unwrap();
        encoder.finish().unwrap()
    }

    fn link_chain(expansions: usize) -> Vec<u8> {
        let mut name = "conduit.yaml".to_owned();
        let mut members = Vec::new();
        for index in 1..=expansions {
            let target = format!("link-{index}");
            members.push(link(name.as_bytes(), target.as_bytes(), b'2'));
            name = target;
        }
        members.push(file(name.as_bytes(), MANIFEST));
        gzip(&terminated(members))
    }

    #[test]
    fn install_and_metadata_paths_read_archives_alike() {
        let manifest = conduit_loader::read_bytes(conduit_loader::Syntax::Yaml, MANIFEST).unwrap()
            [0]
        .to_json();
        let tar_with = |extra: Vec<_>| {
            terminated(std::iter::once(file(b"conduit.yaml", MANIFEST)).chain(extra))
        };
        let archive_with = |extra| gzip(&tar_with(extra));
        let baseline = tar_with(vec![]);
        let mut bad_crc = gzip(&baseline);
        let crc = bad_crc.len() - 8;
        bad_crc[crc] ^= 1;
        let mut gzip_junk = gzip(&baseline);
        gzip_junk.extend_from_slice(b"junk");
        let short_end = gzip(&baseline[..baseline.len() - 512]);
        let mut tar_junk = baseline.clone();
        tar_junk.extend([0; 1536]);
        tar_junk[baseline.len()] = 1;

        let malformed = Some("PKG-022: ");
        let unsafe_name = Some("MAN-014: ");
        let rows = [
            ("1 bad CRC", bad_crc, malformed),
            ("2 trailing non-gzip bytes", gzip_junk, malformed),
            ("3 one zero block", short_end, malformed),
            ("4 trailing tar content", gzip(&tar_junk), malformed),
            (
                "5 truncated member",
                archive_with(vec![member(b"extra", b'0', b"", b"", |header| {
                    header.set_size(4096);
                })]),
                malformed,
            ),
            (
                "6 GNU header",
                archive_with(vec![member(b"extra", b'0', b"", b"", |header| {
                    header.as_mut_bytes()[257..265].copy_from_slice(b"ustar  \0");
                })]),
                malformed,
            ),
            (
                "7 PAX extension",
                archive_with(vec![member(b"extra", b'x', b"", b"", |_| {})]),
                malformed,
            ),
            (
                "8 FIFO entry",
                archive_with(vec![member(b"extra", b'6', b"", b"", |_| {})]),
                malformed,
            ),
            (
                "9 non-octal mode",
                archive_with(vec![member(b"extra", b'0', b"", b"", |header| {
                    header.as_mut_bytes()[100..108].copy_from_slice(b"00000x0\0");
                })]),
                malformed,
            ),
            (
                "10 non-UTF-8 path",
                archive_with(vec![file(b"extra-\xff", b"")]),
                malformed,
            ),
            (
                "11 parent segment",
                archive_with(vec![file(b"dir/../extra", b"")]),
                unsafe_name,
            ),
            (
                "12 unused escaping symlink",
                archive_with(vec![link(b"extra", b"../outside", b'2')]),
                unsafe_name,
            ),
            (
                "13 absolute hard link",
                archive_with(vec![link(b"extra", b"/conduit.yaml", b'1')]),
                unsafe_name,
            ),
            (
                "14 unused link cycle",
                archive_with(vec![
                    link(b"first", b"second", b'2'),
                    link(b"second", b"first", b'2'),
                ]),
                malformed,
            ),
            ("15 41 expansions", link_chain(41), malformed),
            ("15 40 expansions", link_chain(40), None),
            (
                "16 missing component before parent segment",
                gzip(&terminated([
                    file(b"manifest.yaml", MANIFEST),
                    link(b"conduit.yaml", b"missing/../manifest.yaml", b'2'),
                ])),
                malformed,
            ),
            (
                "17 member below file",
                archive_with(vec![file(b"parent", b""), file(b"parent/child", b"")]),
                malformed,
            ),
            (
                "18 replace nonempty directory",
                archive_with(vec![file(b"parent/child", b""), file(b"parent", b"")]),
                malformed,
            ),
            (
                "19 regular file at root",
                archive_with(vec![file(b".", b"")]),
                malformed,
            ),
            ("20 written order", gzip(&baseline), None),
        ];

        let mut mismatches = Vec::new();
        for (row, archive, rule) in rows {
            let metadata = crate::boundary::install_manifest(&archive);
            let install = dependencies(&archive, &manifest).map(|requirements| {
                requirements
                    .into_iter()
                    .map(|requirement| requirement.pin.address)
                    .collect::<Vec<_>>()
            });
            eprintln!(
                "{row}: metadata {:?}; install {install:?}",
                metadata.as_ref().map(|observed| observed == &manifest),
            );
            let agrees = match (&metadata, &install, rule) {
                (Ok(value), Ok(order), None) => {
                    value == &manifest && order == &["example.org/b", "example.org/a"]
                }
                (Err(metadata), Err(install), Some(prefix)) => {
                    metadata.starts_with(prefix) && install == metadata
                }
                _ => false,
            };
            if !agrees {
                mismatches.push(row);
            }
        }
        assert!(
            mismatches.is_empty(),
            "archive readers disagree with the table on: {}",
            mismatches.join(", ")
        );
    }
}
