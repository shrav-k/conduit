use super::{FileStore, NeverGate, call, support::*};
use rmcp::ServiceExt;
use runtime_provider_kit::{RunInput, RuntimeBackend, RuntimeServer, Unavailable, names};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    os::unix::fs::symlink,
    path::PathBuf,
    sync::{Arc, atomic::Ordering},
    time::Duration,
};
use tokio_util::sync::CancellationToken;

const PROGRAM: &str = "platforms/linux/arm64/bin/server";
const RUN_PATH: &str = "/usr/bin:/bin:/sbin";

struct Package {
    bytes: Vec<u8>,
    digest: String,
    url: String,
}

impl Package {
    fn new(f: &Fixture) -> Self {
        let gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        let mut tar = tar::Builder::new(gzip);
        let program = b"#!/bin/sh\nexit 0\n";
        let mut header = tar::Header::new_ustar();
        header.set_mode(0o755);
        header.set_size(program.len() as u64);
        tar.append_data(&mut header, PROGRAM, program.as_slice())
            .unwrap();
        let bytes = tar.into_inner().unwrap().finish().unwrap();
        let digest = format!("sha256:{:x}", Sha256::digest(&bytes));
        let path = f._root.path().join("engine copy.tar.gz");
        fs::write(&path, &bytes).unwrap();
        let path = path
            .to_str()
            .unwrap()
            .replace('%', "%25")
            .replace(' ', "%20");
        Self {
            bytes,
            digest,
            url: format!("file://localhost{path}"),
        }
    }

    fn request(&self, id: &str, size: usize) -> Value {
        let mut request = serde_json::to_value(call(id)).unwrap();
        request["name"] = json!("run.v2");
        let args = &mut request["arguments"];
        args["item"]["command"] = json!(["bin/server", "--flag"]);
        args["item"]["platforms"] = json!(["linux/arm64"]);
        args["program"] = json!(PROGRAM);
        args["archive"] = json!({"digest":self.digest,"size":size,"urls":[self.url]});
        request
    }

    fn input(&self, id: &str, size: usize) -> RunInput {
        serde_json::from_value(self.request(id, size)["arguments"].clone()).unwrap()
    }

    fn kept(&self, f: &Fixture) -> PathBuf {
        f.remote
            .path()
            .join(f.backend.part())
            .join("archives")
            .join(self.digest.strip_prefix("sha256:").unwrap())
    }

    fn transfers(&self, f: &Fixture) -> usize {
        f.fake
            .calls
            .lock()
            .unwrap()
            .iter()
            .filter(|call| call.input == self.bytes)
            .count()
    }
}

#[tokio::test]
async fn run_v2_carries_the_archive_once_and_runs_the_pair_program() {
    let f = Fixture::with_run_path(RUN_PATH);
    let package = Package::new(&f);
    let ctx = context();
    let cancel = CancellationToken::new();
    let first = f
        .backend
        .stage(&ctx, &package.input(ID, package.bytes.len()), &cancel)
        .await;
    assert!(first.is_ok(), "a valid run.v2 archive must reach staging");
    let second = f
        .backend
        .stage(&ctx, &package.input(OTHER, package.bytes.len()), &cancel)
        .await
        .unwrap();
    let paths = [f.directory(ID), f.directory(OTHER)].map(|dir| dir.join("package").join(PROGRAM));
    assert_ne!(paths[0], paths[1]);
    assert!(paths.iter().all(|p| p.is_file()));
    assert!(fs::read(package.kept(&f)).unwrap() == package.bytes);
    assert_eq!(
        fs::read_dir(package.kept(&f).parent().unwrap())
            .unwrap()
            .count(),
        1
    );
    assert_eq!(package.transfers(&f), 1);
    let mut channels = Vec::new();
    for (id, staged) in [(ID, first.unwrap()), (OTHER, second)] {
        channels.push(
            f.backend
                .start(&ctx, id, staged, &cancel, Duration::from_secs(30))
                .await
                .unwrap(),
        );
    }
    {
        let calls = f.fake.calls.lock().unwrap();
        let starts: Vec<_> = calls
            .iter()
            .map(|c| words(c.args.last().unwrap()))
            .filter(|command| command.iter().any(|word| word == "--pipe"))
            .collect();
        assert_eq!(starts.len(), 2);
        for (start, path) in starts.iter().zip(paths) {
            assert!(start.ends_with(&[path.to_str().unwrap().to_owned(), "--flag".into()]));
        }
    }
    for id in [ID, OTHER] {
        f.backend.stop(&ctx, id).await.unwrap();
    }
    drop(channels);
    assert!(package.kept(&f).is_file());
}

#[tokio::test]
async fn run_v2_mismatch_runs_nothing_and_names_both_values() {
    let f = Fixture::with_run_path(RUN_PATH);
    let package = Package::new(&f);
    let ctx = context();
    let wrong_size = package.bytes.len() + 1;
    let result = f
        .backend
        .stage(
            &ctx,
            &package.input(ID, wrong_size),
            &CancellationToken::new(),
        )
        .await;
    let expected = format!(
        "archive on machine {} has {} bytes; the call gave {wrong_size}",
        ctx.machine.hostname,
        package.bytes.len()
    );
    assert!(
        result.err() == Some(Unavailable::because(expected)),
        "size mismatch must name both lengths"
    );
    assert!(!f.directory(ID).join("package").exists());
    assert!(!package.kept(&f).exists());

    let kept = package.kept(&f);
    let mut changed = package.bytes.clone();
    changed[0] ^= 1;
    let found = format!("sha256:{:x}", Sha256::digest(&changed));
    fs::create_dir_all(kept.parent().unwrap()).unwrap();
    fs::write(&kept, changed).unwrap();
    let server = RuntimeServer::new(
        Arc::new(f.backend.clone()),
        Arc::new(FileStore::new(&f.durable)),
        Arc::new(NeverGate),
        f.config.options(),
    )
    .await
    .unwrap();
    let (client_io, server_io) = tokio::io::duplex(1024 * 1024);
    let serving = tokio::spawn(async move { server.serve_transport(server_io).await });
    let client = ().serve(client_io).await.unwrap();
    let answer = client
        .call_tool(serde_json::from_value(package.request(OTHER, package.bytes.len())).unwrap())
        .await
        .unwrap();
    let answer = serde_json::to_value(answer).unwrap();
    assert_eq!(answer["_meta"][names::key("outcome")], "failed");
    let expected = format!(
        "staging failed: archive on machine {} has digest {found}; the call gave {}",
        ctx.machine.hostname, package.digest
    );
    assert!(
        answer["content"][0]["text"] == expected,
        "digest mismatch must name both digests"
    );
    assert!(!f.directory(OTHER).join("package").exists());
    assert!(f.fake.sessions.lock().unwrap().is_empty());
    assert!(!kept.exists());
    assert_eq!(package.transfers(&f), 1);
    client.cancel().await.unwrap();
    serving.await.unwrap().unwrap();
}

#[tokio::test]
async fn run_v2_verifies_and_unpacks_one_unnamed_snapshot() {
    let f = Fixture::with_run_path(RUN_PATH);
    let package = Package::new(&f);
    let _staged = f
        .backend
        .stage(
            &context(),
            &package.input(ID, package.bytes.len()),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
    let calls = f.fake.calls.lock().unwrap();
    let check = calls
        .iter()
        .filter(|call| call.args.last().is_some_and(|arg| arg.contains("/bin/sh")))
        .filter_map(|call| std::str::from_utf8(&call.input).ok())
        .find(|script| script.contains("expected_digest="))
        .unwrap();
    assert!(
        !check.contains("copy=$directory/archive.gz"),
        "archive verification must not materialize a writable snapshot pathname"
    );
    assert_eq!(check.matches("base64 < \"$kept\"").count(), 1);
    for pipeline in [
        "stream_archive | LC_ALL=C wc",
        "stream_archive | LC_ALL=C sha256sum",
        "stream_archive | { gzip",
    ] {
        assert!(check.contains(pipeline));
    }
}

#[tokio::test]
async fn run_v2_quarantines_mismatch_until_removal_succeeds() {
    const RETRY: &str = "cccccccc-cccc-4ccc-8ccc-cccccccccccc";
    for same_length in [false, true] {
        let f = Fixture::with_run_path(RUN_PATH);
        let package = Package::new(&f);
        let ctx = context();
        let cancel = CancellationToken::new();
        let kept = package.kept(&f);
        let changed = vec![0; if same_length { package.bytes.len() } else { 1 }];
        fs::create_dir_all(kept.parent().unwrap()).unwrap();
        fs::write(&kept, &changed).unwrap();
        f.fake.block_archive_removal.store(true, Ordering::SeqCst);
        let first = f
            .backend
            .stage(&ctx, &package.input(ID, package.bytes.len()), &cancel)
            .await;
        let expected = if same_length {
            format!(
                "archive on machine {} has digest sha256:{:x}; the call gave {}",
                ctx.machine.hostname,
                Sha256::digest(&changed),
                package.digest
            )
        } else {
            format!(
                "archive on machine {} has 1 bytes; the call gave {}",
                ctx.machine.hostname,
                package.bytes.len()
            )
        };
        assert!(first.err() == Some(Unavailable::because(expected)));
        assert!(kept.is_file());
        let second = f
            .backend
            .stage(&ctx, &package.input(OTHER, package.bytes.len()), &cancel)
            .await;
        let expected = format!(
            "kept archive {} on machine {} could not be removed; it is ineligible until removal succeeds",
            kept.display(),
            ctx.machine.hostname
        );
        assert!(
            second.err() == Some(Unavailable::because(expected)),
            "known-bad cache must be refused until removal succeeds"
        );
        assert_eq!(package.transfers(&f), 0);
        f.fake.block_archive_removal.store(false, Ordering::SeqCst);
        let _staged = f
            .backend
            .stage(&ctx, &package.input(RETRY, package.bytes.len()), &cancel)
            .await
            .unwrap();
        assert_eq!(package.transfers(&f), 1);
        assert!(fs::read(&kept).unwrap() == package.bytes);
    }
}

#[tokio::test]
async fn run_v2_names_a_verified_archive_that_cannot_be_unpacked() {
    let f = Fixture::with_run_path(RUN_PATH);
    let mut package = Package::new(&f);
    package.bytes.truncate(package.bytes.len() - 8);
    package.digest = format!("sha256:{:x}", Sha256::digest(&package.bytes));
    fs::write(f._root.path().join("engine copy.tar.gz"), &package.bytes).unwrap();
    let ctx = context();
    let result = f
        .backend
        .stage(
            &ctx,
            &package.input(ID, package.bytes.len()),
            &CancellationToken::new(),
        )
        .await;
    let expected = format!(
        "archive {} on machine {} could not be unpacked into {}/package (gzip or tar failed); the kept copy is unchanged",
        package.digest,
        ctx.machine.hostname,
        f.directory(ID).display()
    );
    assert!(
        result.err() == Some(Unavailable::because(expected)),
        "unpack failure must name the digest, node, directory and cause"
    );
    assert!(fs::read(package.kept(&f)).unwrap() == package.bytes);
    assert!(f.fake.sessions.lock().unwrap().is_empty());
}

#[tokio::test]
async fn run_v2_reports_the_path_and_step_for_cache_failures() {
    for bad_kept in [true, false] {
        let f = Fixture::with_run_path(RUN_PATH);
        let package = Package::new(&f);
        let kept = package.kept(&f);
        let ctx = context();
        let expected = if bad_kept {
            fs::create_dir_all(&kept).unwrap();
            format!(
                "kept archive {} on machine {} is not a regular file; remove it",
                kept.display(),
                ctx.machine.hostname
            )
        } else {
            fs::create_dir_all(kept.parent().unwrap().parent().unwrap()).unwrap();
            fs::write(kept.parent().unwrap(), b"ordinary file").unwrap();
            format!(
                "archive transfer to {} on machine {} failed: could not create the archive directory",
                kept.display(),
                ctx.machine.hostname
            )
        };
        let result = f
            .backend
            .stage(
                &ctx,
                &package.input(ID, package.bytes.len()),
                &CancellationToken::new(),
            )
            .await;
        assert!(result.err() == Some(Unavailable::because(expected)));
        assert!(!f.directory(ID).join("package").exists());
    }
}

#[tokio::test]
async fn run_v2_names_a_missing_tool() {
    // Resolve the same relative PATH entry from the generation that systemd uses.
    let f = Fixture::with_run_path("../../../../tools");
    let tools = f.remote.path().join("tools");
    fs::create_dir(&tools).unwrap();
    for name in ["wc", "gzip", "tar"] {
        symlink(format!("/usr/bin/{name}"), tools.join(name)).unwrap();
    }
    let package = Package::new(&f);
    let result = f
        .backend
        .stage(
            &context(),
            &package.input(ID, package.bytes.len()),
            &CancellationToken::new(),
        )
        .await;
    assert!(
        result.err()
            == Some(Unavailable::because(
                "machine synthetic-node lacks sha256sum on the run PATH"
            )),
        "missing sha256sum must be named"
    );
    assert_eq!(package.transfers(&f), 0);
    assert!(!package.kept(&f).exists());
}
