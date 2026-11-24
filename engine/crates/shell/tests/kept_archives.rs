use conduit_core::{
    Input,
    packages::{Catalog, Project, bytes_digest},
    step_owned,
};
use conduit_shell::{Config, MissingExecutor, Shell, boundary};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

const ADDRESS: &str = "example.org/kept";
const VERSION: &str = "1.0.0";

fn open(directory: &Path) -> conduit_shell::Result<Shell> {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    Shell::open(
        directory,
        boundary::context(&repository)?,
        Config::default(),
    )
}

fn start(directory: &Path) -> Shell {
    let shell = open(directory).unwrap();
    shell
        .submit(
            json!({"kind":"first-start","machine":conduit_shell::local_machine(BTreeMap::new()).unwrap()}),
            &MissingExecutor,
        )
        .unwrap();
    shell
}

fn archive(payload_size: usize) -> Vec<u8> {
    let manifest = serde_json::to_vec(
        &json!({"apiVersion":"github.com/shrav-k/conduit/v0","address":ADDRESS,"version":VERSION}),
    )
    .unwrap();
    let payload = vec![b'x'; payload_size];
    pack(&[
        ("conduit.json", &manifest, 0o644),
        ("payload", &payload, 0o755),
    ])
}

fn pack(members: &[(&str, &[u8], u32)]) -> Vec<u8> {
    let gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    let mut tar = tar::Builder::new(gzip);
    for &(name, bytes, mode) in members {
        let mut header = tar::Header::new_ustar();
        header.set_mode(mode);
        header.set_uid(0);
        header.set_gid(0);
        header.set_mtime(0);
        header.set_size(bytes.len() as u64);
        tar.append_data(&mut header, name, bytes).unwrap();
    }
    tar.into_inner().unwrap().finish().unwrap()
}

fn input(shell: &Shell, bytes: &[u8], digest: &str) -> Value {
    json!({"kind":"install-archive","snapshots":[],"builtin":shell.context.builtin,
           "archive":{"name":"kept.tar.gz","address":ADDRESS,"version":VERSION,
           "digest":digest,"directory":"file:///archive/kept/",
           "completion":{"status":"complete","bytes":bytes}}})
}

fn public_install(shell: &Shell, source: &Path, bytes: &[u8], digest: &str) -> Value {
    fs::write(source, bytes).unwrap();
    let root = shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|object| object["kind"] == "Interface" && object["name"] == "root")
        .unwrap()["id"]
        .clone();
    shell
        .submit(
            json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},
        "tool":"management.install.v1","arguments":{"address":ADDRESS,"version":VERSION,
        "digest":digest,"fetch":url::Url::from_file_path(source).unwrap().to_string()}}}),
            &MissingExecutor,
        )
        .unwrap()
}

fn copy(directory: &Path, digest: &str) -> PathBuf {
    directory
        .join("packages")
        .join(digest.strip_prefix("sha256:").unwrap())
}

#[test]
fn plan_fixture_archives_survive_kept_copy_reload() {
    let temp = tempfile::tempdir().unwrap();
    let directory = temp.path().join("store");
    let mut shell = start(&directory);
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../conformance/fixtures");
    for name in [
        "install-archives/filesystem-0.1.1.tar.gz",
        "s1/archives/agents-0.2.1.tar.gz",
    ] {
        let path = fixtures.join(name);
        let bytes = fs::read(&path).unwrap();
        let digest = bytes_digest(&bytes);
        let args = boundary::install_arguments(&format!(
            "{}#{}",
            path.display(),
            digest.strip_prefix("sha256:").unwrap()
        ))
        .unwrap();
        let mut request = input(&shell, &bytes, &digest);
        request["archive"]["address"] = args["address"].clone();
        request["archive"]["version"] = args["version"].clone();
        let reply = shell.submit(request, &MissingExecutor).unwrap();
        assert_eq!(reply["changed"], true, "{name}: {reply}");
        assert_eq!(fs::read(copy(&directory, &digest)).unwrap(), bytes);
        let packages = shell.state().unwrap().installed_packages.clone();
        drop(shell);
        shell = open(&directory).unwrap();
        assert_eq!(
            shell.state().unwrap().installed_packages,
            packages,
            "{name}"
        );
    }
}

#[test]
fn an_archive_directory_with_special_mode_bits_cannot_receive_an_install() {
    let temp = tempfile::tempdir().unwrap();
    let directory = temp.path().join("store");
    let shell = start(&directory);
    let archives = directory.join("packages");
    fs::create_dir(&archives).unwrap();
    fs::set_permissions(&archives, fs::Permissions::from_mode(0o2700)).unwrap();
    assert_eq!(
        fs::metadata(&archives).unwrap().permissions().mode() & 0o7777,
        0o2700
    );

    let bytes = archive(1);
    let digest = bytes_digest(&bytes);
    let result = shell.submit(input(&shell, &bytes, &digest), &MissingExecutor);
    assert!(
        result.is_err(),
        "install accepted a packages directory of mode 2700"
    );
    assert!(result.unwrap_err().contains("0700"));
    assert!(!copy(&directory, &digest).exists());
    drop(shell);
    let reopened = open(&directory).unwrap();
    assert!(reopened.state().unwrap().installed_packages.is_empty());
}

#[test]
fn platforms_require_recognized_go_os_and_arch_values() {
    let temp = tempfile::tempdir().unwrap();
    let shell = start(&temp.path().join("catalog"));
    let catalog = Catalog::new(
        shell.context.builtin.clone().unwrap(),
        vec![],
        &shell.context.core_package,
    )
    .unwrap();
    let mut accepted = Vec::new();

    for pair in ["unknown/arm64", "linux/unknown", "linux/arm64"] {
        let mut manifest = json!({
            "apiVersion":"github.com/shrav-k/conduit/v0","address":ADDRESS,
            "implementations":{"server":{"implements":["#probe.v1"],
                "platforms":[pair],"command":["bin/server"]}}
        });
        let operation = json!({"apiVersion":"github.com/shrav-k/conduit/v0",
            "kind":"Operation","metadata":{"name":"probe.v1"},"spec":{}});
        let resolved = catalog
            .resolve(&Project {
                manifest: Some(manifest.clone()),
                documents: vec![operation.clone()],
                directory: Some("file:///project/platforms/".into()),
            })
            .unwrap()
            .reply;

        manifest["version"] = json!(VERSION);
        manifest["exports"] = json!(["probe.v1"]);
        let archive = pack(&[
            (
                "conduit.json",
                &serde_json::to_vec(&manifest).unwrap(),
                0o644,
            ),
            (
                "probe.json",
                &serde_json::to_vec(&operation).unwrap(),
                0o644,
            ),
            (&format!("platforms/{pair}/bin/server"), b"program", 0o755),
        ]);
        // Use independent installs so a wrongly accepted pair cannot occupy the next one's pin.
        let store = start(&temp.path().join(pair.replace('/', "-")));
        let installed = store
            .submit(
                input(&store, &archive, &bytes_digest(&archive)),
                &MissingExecutor,
            )
            .unwrap();
        for reply in [resolved, installed] {
            let succeeded = reply["accepted"].as_bool().unwrap();
            accepted.push(succeeded);
            if !succeeded && pair != "linux/arm64" {
                let diagnostics = reply["diagnostics"].as_array().unwrap();
                assert_eq!(diagnostics.len(), 1);
                assert_eq!(diagnostics[0]["ruleId"], "MAN-010");
                let message = diagnostics[0]["message"]["text"].as_str().unwrap();
                assert!(message.contains("server") && message.contains(pair));
            }
        }
    }
    assert_eq!(accepted, [false, false, false, false, true, true]);
}

#[test]
fn install_keeps_the_archive_outside_state_json() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("kept.tar.gz");
    let tiny = archive(1);
    let bytes = archive(65_536);
    let digest = bytes_digest(&bytes);
    let marker = ["120"; 24].join(",");
    for (name, public) in [("public", true), ("boundary", false)] {
        let install = |shell: &Shell, bytes: &[u8], digest: &str| {
            if public {
                let reply = public_install(shell, &source, bytes, digest);
                assert_eq!(reply["isError"], false);
            } else {
                let reply = shell
                    .submit(input(shell, bytes, digest), &MissingExecutor)
                    .unwrap();
                assert_eq!(reply["changed"], true);
            }
        };
        let control_dir = temp.path().join(format!("{name}-control"));
        let control = start(&control_dir);
        let control_file = control_dir.join("state.json");
        let control_before = fs::metadata(&control_file).unwrap().len();
        install(&control, &tiny, &bytes_digest(&tiny));
        let control_growth = fs::metadata(&control_file).unwrap().len() - control_before;
        eprintln!("tiny {name} archive: state.json grew by {control_growth} bytes");
        drop(control);

        let directory = temp.path().join(name);
        let shell = start(&directory);
        let state_file = directory.join("state.json");
        let before = fs::metadata(&state_file).unwrap().len();
        install(&shell, &bytes, &digest);
        let growth = fs::metadata(&state_file).unwrap().len() - before;
        eprintln!("{name}: state.json grew by {growth} bytes");
        let stored = fs::read(&state_file).unwrap();
        assert!(
            !stored
                .windows(marker.len())
                .any(|part| part == marker.as_bytes()),
            "{name}: state.json contains the payload's member bytes"
        );
        let added_growth = growth.saturating_sub(control_growth);
        eprintln!("{name} archive size added {added_growth} bytes of state.json growth");
        assert!(
            added_growth <= 65_536,
            "{name} archive added {added_growth} bytes"
        );
        let archive_file = copy(&directory, &digest);
        assert_eq!(
            fs::read(&archive_file).expect("installed archive copy is absent"),
            bytes
        );
        assert_eq!(
            fs::metadata(&archive_file).unwrap().permissions().mode() & 0o777,
            0o600
        );
        assert_eq!(
            fs::metadata(directory.join("packages"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o700
        );
        let packages = shell.state().unwrap().installed_packages.clone();
        assert_eq!(
            packages[0].files["payload"],
            conduit_core::archives::Member::File {
                bytes: vec![b'x'; 65_536],
                mode: 0o755,
            }
        );
        drop(shell);
        assert_eq!(
            open(&directory)
                .unwrap()
                .state()
                .unwrap()
                .installed_packages,
            packages
        );
    }
}

#[test]
fn a_missing_or_altered_kept_archive_stops_the_store_opening() {
    let temp = tempfile::tempdir().unwrap();
    let directory = temp.path().join("store");
    let shell = start(&directory);
    let bytes = archive(65_536);
    let digest = bytes_digest(&bytes);
    let reply = shell
        .submit(input(&shell, &bytes, &digest), &MissingExecutor)
        .unwrap();
    assert_eq!(reply["changed"], true);
    drop(shell);

    let archive_file = copy(&directory, &digest);
    let mut altered = fs::read(&archive_file).expect("installed archive copy is absent");
    altered[0] ^= 1;
    fs::write(&archive_file, &altered).unwrap();
    let error = open(&directory)
        .err()
        .expect("altered archive opened the store");
    let found = bytes_digest(&altered);
    for wanted in [
        ADDRESS,
        VERSION,
        archive_file.to_str().unwrap(),
        &digest,
        &found,
    ] {
        assert!(
            error.contains(wanted),
            "archive error does not name {wanted}"
        );
    }
    fs::remove_file(&archive_file).unwrap();
    let error = open(&directory)
        .err()
        .expect("missing archive opened the store");
    for wanted in [ADDRESS, VERSION, archive_file.to_str().unwrap(), "absent"] {
        assert!(
            error.contains(wanted),
            "archive error does not name {wanted}"
        );
    }
    fs::create_dir(&archive_file).unwrap();
    let error = open(&directory).err().expect("directory opened the store");
    assert!(error.contains("directory"));
    fs::remove_dir(&archive_file).unwrap();
    fs::write(&archive_file, &bytes).unwrap();
    fs::set_permissions(&archive_file, fs::Permissions::from_mode(0o644)).unwrap();
    let error = open(&directory)
        .err()
        .expect("wrong archive mode opened the store");
    assert!(error.contains("0644"));
    fs::set_permissions(&archive_file, fs::Permissions::from_mode(0o600)).unwrap();
    assert_eq!(
        open(&directory)
            .unwrap()
            .state()
            .unwrap()
            .installed_packages
            .len(),
        1
    );
}

#[test]
fn members_kept_in_state_json_before_order_278_still_load() {
    let temp = tempfile::tempdir().unwrap();
    let directory = temp.path().join("store");
    let shell = start(&directory);
    let bytes = archive(65_536);
    let digest = bytes_digest(&bytes);
    let parsed: Input = serde_json::from_value(input(&shell, &bytes, &digest)).unwrap();
    let transition = step_owned((*shell.state().unwrap()).clone(), parsed, &shell.context).unwrap();
    assert_eq!(transition.reply["changed"], true);
    let expected = transition.state.installed_packages;
    drop(shell);
    let state_file = directory.join("state.json");
    let read_state = || serde_json::from_slice::<Value>(&fs::read(&state_file).unwrap()).unwrap();
    let mut stored = read_state();
    stored["state"]["installed_packages"] = json!(expected);
    fs::write(&state_file, serde_json::to_vec(&stored).unwrap()).unwrap();
    let shell = open(&directory).unwrap();
    assert_eq!(shell.state().unwrap().installed_packages, expected);
    shell
        .submit(json!({"kind":"export-records"}), &MissingExecutor)
        .unwrap();
    assert!(read_state()["state"]["installed_packages"] == json!(expected));
    assert!(!copy(&directory, &digest).exists());
}
