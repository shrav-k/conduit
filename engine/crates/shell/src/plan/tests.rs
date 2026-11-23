use super::*;

const API: &str = "github.com/shrav-k/conduit/v0";
const CORE: &str = "github.com/shrav-k/conduit/core";
const DIGEST: &str = "sha256:1111111111111111111111111111111111111111111111111111111111111111";
const OTHER: &str = "sha256:2222222222222222222222222222222222222222222222222222222222222222";
const ID: &str = "00000000-0000-4000-8000-000000000001";

#[derive(Default)]
struct Fixture {
    file: Desired,
    kept: Ledger,
    engine: Engine,
}

impl Default for Desired {
    fn default() -> Self {
        Self {
            project: Project {
                manifest: None,
                documents: vec![],
                directory: None,
            },
            objects: BTreeMap::new(),
            nested: Vec::new(),
            core_address: "github.com/shrav-k/conduit/core".into(),
            core_pin: Value::Null,
            stops: BTreeSet::new(),
            available_archives: BTreeMap::new(),
        }
    }
}

fn object(state: &str, definition: Value) -> ObservedObject {
    ObservedObject {
        id: ID.into(),
        revision: "r1".into(),
        state: state.into(),
        definition,
        packages: vec![],
        status: Value::Null,
        unknown_endings: BTreeMap::new(),
        inventory: Value::Null,
        implementations: Value::Null,
        made_by_engine: false,
    }
}

impl Fixture {
    fn want(&mut self, name: &Name, resolved: ObservedObject) -> &mut DesiredObject {
        let mut document = json!({"apiVersion":API,"kind":name.kind,"metadata":{"name":name.name},"from":"example"});
        if resolved.is_template() {
            document.as_object_mut().unwrap().remove("from");
            document["spec"] = resolved.definition.clone();
        }
        self.file.objects.insert(
            name.clone(),
            DesiredObject {
                document,
                resolved,
                own_tools: None,
                controls: BTreeMap::new(),
                reached_documents: BTreeMap::new(),
                reached_packages: BTreeSet::new(),
                model_work: false,
            },
        );
        self.file.objects.get_mut(name).unwrap()
    }

    fn observed(&mut self, name: &Name, old: ObservedObject) {
        self.kept.objects.insert(name.clone(), old.clone());
        self.engine.objects.insert(name.clone(), old);
    }

    fn text(&self) -> String {
        compare(&self.file, &self.kept, &self.engine).text()
    }

    fn line(&self, expect: &str) {
        let actual = self.text();
        assert!(
            actual.lines().any(|line| line == expect),
            "expected {expect:?}; actual plan: {actual}"
        );
    }

    fn no_changes(&self) {
        assert_eq!(self.text(), "plan\nno changes\n");
    }
}

fn example_pin(digest: &str) -> Pin {
    Pin {
        address: "example.org/release".into(),
        version: Some("1.0.0".into()),
        digest: Some(digest.into()),
    }
}

#[test]
fn review_b_05_manifest_file_url_is_fingerprinted_only_in_plan_text() {
    let mut f = Fixture::default();
    let fetch = "file:///project/archives/release.tar.gz";
    f.file.project.manifest = Some(json!({"apiVersion":API,"address":"example.org/project",
        "packages":{"example.org/release":{"version":"1.0.0","digest":DIGEST,"fetch":fetch}}}));
    f.want(
        &Name::new("Resource", "example--t"),
        object("template", json!({})),
    );
    let plan = compare(&f.file, &f.kept, &f.engine);
    let text = plan.text();
    assert!(!text.contains(fetch), "plan exposes the archive file URL");
    let fingerprint = bytes_digest(fetch.as_bytes());
    assert!(text.contains(&format!(
        r#""fetch":{{"pathSha256":"{}"}}"#,
        &fingerprint[7..]
    )));
    assert_eq!(
        plan.steps[0].arguments["manifest"]["packages"]["example.org/release"]["fetch"],
        fetch
    );
}

fn grant(operations: &[&str]) -> Value {
    json!({
        "consumer":{"kind":"Agent","name":"a"},"target":{"kind":"Resource","name":"shared"},
        "as":"project","from":"example.org/release#read","resolved":{"operations":operations,"capability":"example.org/release#files.v1"},
    })
}

#[test]
fn row_01_package_must_be_installed_at_exact_digest() {
    let mut f = Fixture::default();
    let pin = example_pin(DIGEST);
    read::check_installed(&pin, &f.engine, &mut f.file.stops);
    f.line(&format!("stop Package/example.org/release version=1.0.0 digest={DIGEST} installed=no requires=conduit-install PKG-006"));
    f.file.stops.clear();
    f.engine.installed.insert(example_pin(OTHER));
    read::check_installed(&pin, &f.engine, &mut f.file.stops);
    f.line(&format!(
        "stop Package/example.org/release version=1.0.0 digest={DIGEST} installed={OTHER} PKG-025"
    ));
}

#[test]
fn row_02_archive_absence_and_mismatch_stop() {
    let mut f = Fixture::default();
    let pin = example_pin(DIGEST);
    read::Cache::stop(&pin, &Archive::Absent, true, &mut f.file.stops);
    f.line(&format!("stop Package/example.org/release version=1.0.0 digest={DIGEST} archive=absent requires=conduit-install PKG-004"));
    f.file.stops.clear();
    read::Cache::stop(&pin, &Archive::Mismatch, true, &mut f.file.stops);
    f.line(&format!(
        "stop Package/example.org/release version=1.0.0 digest={DIGEST} archive=mismatch PKG-004"
    ));
}

#[test]
fn row_03_relative_manifest_values_stop() {
    let mut f = Fixture::default();
    f.file.project.manifest = Some(json!({
        "packages":{"example.org/release":{"fetch":"release.tar.gz"}},
        "implementations":{"reader":{"command":["./reader"]}},
    }));
    read::relative_stops(&f.file.project, &mut f.file.stops);
    f.line("stop Manifest/- place=/implementations/reader/command/0 SUB-010");
    f.line("stop Manifest/- place=/packages/example.org~1release/fetch SUB-010");
}

#[test]
fn row_04_secret_stop_has_no_steps_or_differences() {
    // Safety case 15. The CLI regression exercises finding the governed place.
    let mut f = Fixture::default();
    let name = Name::new("Resource", "reader");
    f.want(&name, object("registered", json!({})));
    f.file
        .stops
        .insert("stop Resource/reader place=/spec/config/value SEC-001".into());
    assert_eq!(
        f.text(),
        "plan\nstop Resource/reader place=/spec/config/value SEC-001\nflags []\nstopped\n"
    );
}

#[test]
fn row_05_unknown_looks_ahead_and_preserves_other_stops() {
    // Safety case 11.
    let mut f = Fixture::default();
    let name = Name::new("Resource", "shared");
    let mut old = object("ready", json!({"resolved":{"config":{"root":"before"}}}));
    old.status = json!({"unreconciled":["invocation"]});
    f.observed(&name, old.clone());
    f.want(&name, old.clone());
    f.line("difference Resource/shared unreconciled=invocation OPS-013");
    old.definition["resolved"]["config"]["root"] = json!("after");
    f.want(&name, old);
    f.line("stop Resource/shared setting=/root requires=--replace shared OBJ-011");
    f.line("stop Resource/shared unreconciled=invocation OPS-013");
}

#[test]
fn row_06_engine_defaults_stay_protected() {
    // Safety case 13.
    let mut f = Fixture::default();
    let name = Name::new("Conduit", "root-management");
    let mut old = object("ready", grant(&["read"]));
    old.made_by_engine = true;
    f.observed(&name, old.clone());
    f.no_changes();
    f.want(&name, old);
    assert_eq!(
        f.text(),
        "plan\nstop Conduit/root-management made-by=engine ACC-024\nflags []\nstopped\n"
    );
}

#[test]
fn row_07_foreign_retirement_waits_for_shutdown() {
    // Safety case 3.
    let mut f = Fixture::default();
    let name = Name::new("Agent", "a");
    let old = object("ready", json!({"owner":"team"}));
    f.observed(&name, old.clone());
    f.want(&name, old);
    let retired = f.engine.objects.get_mut(&name).unwrap();
    retired.state = "retired".into();
    retired.status = json!({"shutdown":"unknown"});
    f.line("stop Agent/a state=retired LIF-024");
    f.kept.objects.get_mut(&name).unwrap().state = "retired".into();
    f.line("stop Agent/a state=retired rule=none -");
}

#[test]
fn row_08_registered_is_verified_and_set_up() {
    let mut f = Fixture::default();
    let name = Name::new("Resource", "reader");
    let instance = object("registered", json!({}));
    f.observed(&name, instance.clone());
    f.want(&name, instance);
    f.line(&format!(
        "1 management.inspect.v1 Resource/reader expected={ID} neither flag=- LIF-013"
    ));
    f.line(&format!(
        "2 management.set-up.v1 Resource/reader expected={ID} neither flag=- TPL-012"
    ));
    f.kept.registration_flags.insert(
        ID.into(),
        BTreeSet::from([Approval::new("--replace", "original-reader")]),
    );
    f.line("stop Resource/reader state=registered requires=--replace original-reader LIF-013");
    assert!(compare(&f.file, &f.kept, &f.engine).steps.is_empty());
}

#[test]
fn row_09_new_object_is_registered_and_verified() {
    // Golden cases 01 and 02.
    let mut f = Fixture::default();
    f.want(&Name::new("Pool", "team"), object("registered", json!({})));
    f.line("1 management.register.v1 Pool/team expected=(step 1) adds flag=- SCT-003");
    f.line("2 management.inspect.v1 Pool/team expected=(step 1) neither flag=- LIF-013");
    f.line("3 management.set-up.v1 Pool/team expected=(step 1) neither flag=- TPL-012");
}

#[test]
fn row_10_drifted_identity_requires_adoption() {
    // Safety case 5.
    let mut f = Fixture::default();
    let name = Name::new("Conduit", "project");
    let observed = object("ready", grant(&["read"]));
    f.observed(&name, observed.clone());
    f.kept.objects.get_mut(&name).unwrap().id = "previous".into();
    f.want(&name, object("registered", grant(&["read", "write"])));
    f.line(&format!(
        "stop Conduit/project id={ID} requires=--adopt project OBJ-002"
    ));
    let name = Name::new("Resource", "reader");
    let mut observed = object("template", json!({}));
    f.observed(&name, observed.clone());
    observed.revision = "r2".into();
    f.engine.objects.insert(name.clone(), observed.clone());
    f.want(&name, observed);
    f.line(&format!(
        "stop Resource/reader id={ID} revision=r2 requires=--adopt reader OBJ-002"
    ));
}

#[test]
fn row_11_matching_tombstone_requires_explicit_reopen() {
    // Safety cases 1–4, including a tombstone independent of purged records.
    let mut f = Fixture::default();
    let name = Name::new("Conduit", "new");
    f.want(&name, object("registered", grant(&["read"])));
    f.engine.tombstones.push(json!({
        "id":"stone","object":{"kind":"Conduit","name":"old"},
        "consumer":{"kind":"Agent","name":"a"},"target":{"kind":"Resource","name":"shared"},
        "localName":"old","capability":"example.org/release#files.v1"
    }));
    f.line("stop Conduit/new tombstone=stone requires=--reopen new LIF-041");
    f.engine.tombstones.clear();
    f.observed(
        &Name::new("Conduit", "old"),
        object("ready", grant(&["read"])),
    );
    f.line("stop Conduit/new tombstone=(Conduit/old) requires=--reopen new LIF-041");
}

#[test]
fn row_12_missing_owned_instance_requires_recreate() {
    let mut f = Fixture::default();
    let name = Name::new("Agent", "a");
    let instance = object("ready", json!({}));
    f.kept.objects.insert(name.clone(), instance.clone());
    f.want(&name, instance);
    f.line("stop Agent/a absent=engine requires=--recreate a LIF-040");
}

#[test]
fn row_13_dropped_grant_closes_without_permission() {
    let mut f = Fixture::default();
    f.observed(
        &Name::new("Conduit", "project"),
        object("ready", grant(&["read"])),
    );
    f.line(&format!(
        "1 access.close.v1 Conduit/project expected={ID} narrows flag=- LIF-037"
    ));
    f.line("2 management.list.v1 Tombstone/* expected=- neither flag=- LIF-040");
    f.line("affects bounds=[] closed-conduits=[project]");
}

#[test]
fn row_14_dropped_owner_requires_cascade_permission() {
    let mut f = Fixture::default();
    f.observed(&Name::new("Pool", "team"), object("ready", json!({})));
    f.observed(
        &Name::new("Resource", "reader"),
        object("ready", json!({"owner":"team"})),
    );
    assert_eq!(
        f.text(),
        "plan\nstop Pool/team absent=file requires=--with-resources team LIF-020\nflags []\nstopped\n"
    );
}

#[test]
fn row_15_dropped_instance_requires_prune() {
    let mut f = Fixture::default();
    f.observed(&Name::new("Resource", "reader"), object("ready", json!({})));
    f.observed(
        &Name::new("Resource", "reader--t"),
        object("template", json!({})),
    );
    assert_eq!(
        f.text(),
        "plan\nstop Resource/reader absent=file requires=--prune reader LIF-035\nflags []\nstopped\n"
    );
}

#[test]
fn row_16_dropped_template_requires_prune() {
    let mut f = Fixture::default();
    f.observed(
        &Name::new("Resource", "reader"),
        object("template", json!({})),
    );
    f.line("stop Resource/reader absent=file requires=--prune reader INS-020");
}

#[test]
fn row_17_file_equal_to_engine_wins() {
    let mut f = Fixture::default();
    let name = Name::new("Resource", "reader");
    let instance = object("ready", json!({"resolved":{"config":{"root":"current"}}}));
    f.observed(&name, instance.clone());
    f.kept.objects.get_mut(&name).unwrap().definition["resolved"]["config"]["root"] =
        json!("prior");
    f.want(&name, instance);
    f.no_changes();
}

fn submitted(address: &str, digest: &str) -> Value {
    json!({"address":address,"digest":digest,"documents":[]})
}

#[test]
fn row_18_template_updates_and_marks_hidden_item_uncertainty() {
    let mut f = Fixture::default();
    let tpl = Name::new("Resource", "reader--t");
    let def = json!({"type":"#resource.v1"});
    let mut old = object("template", def);
    old.packages = vec![submitted("example.org/project", DIGEST)];
    old.implementations = json!({"example.org/project#read.v1":{
        "selection":"example.org/project#reader",
        "package":{"address":"example.org/project","digest":DIGEST},
        "item":{"remote":{"headers":[{"name":"X-Example","isSecret":true,"value":"(secret: not shown)"}]}}
    }});
    f.file.project.manifest = Some(json!({"address":"example.org/project"}));
    f.observed(&tpl, old.clone());
    let mut new = old;
    new.packages = vec![submitted("example.org/project", OTHER)];
    new.implementations["example.org/project#read.v1"]["package"]["digest"] = json!(OTHER);
    f.want(&tpl, new);
    f.line(&format!(
        "1 management.replace.v1 Resource/reader--t expected={ID} replaces flag=- SUB-009"
    ));
    f.line("detail item=example.org/project#reader secret=not-shown SEC-021");
}

fn installed(package: Value) -> Archive {
    use conduit_core::archives::InstalledPackage;
    use conduit_core::packages::{Snapshot, SnapshotSource};
    let pin = Pin::from_value(&package).unwrap();
    Archive::Read(Box::new(InstalledPackage {
        package: Snapshot {
            source: SnapshotSource::InstalledArchive,
            address: pin.address,
            version: pin.version.unwrap(),
            digest: pin.digest.unwrap(),
            project: Project {
                manifest: None,
                documents: vec![
                    json!({"kind":"Operation","metadata":{"name":"read.v1"},"spec":{"effect":"reads"}}),
                    json!({"kind":"Capability","metadata":{"name":"files.v1"},"spec":{"operations":["#read.v1"]}}),
                ],
                directory: None,
            },
        },
        files: BTreeMap::new(),
    }))
}

#[test]
fn row_19_equal_execution_retains_held_pins() {
    let mut f = Fixture::default();
    let resource = Name::new("Resource", "shared");
    let conduit = Name::new("Conduit", "project");
    let mut old = object("ready", grant(&["example.org/release#read.v1"]));
    let old_pin = json!({"address":"example.org/release","version":"1.0.0","digest":DIGEST});
    let new_pin = json!({"address":"example.org/release","version":"2.0.0","digest":OTHER});
    old.packages = vec![old_pin.clone()];
    f.observed(&conduit, old.clone());
    old.packages = vec![new_pin.clone()];
    f.want(&conduit, old);
    let mut target = object("ready", json!({}));
    target.packages =
        vec![json!({"address":"example.org/release","version":"0.1.0","digest":OTHER})];
    f.observed(&resource, target.clone());
    f.want(&resource, target);
    f.file
        .available_archives
        .insert(Pin::from_value(&old_pin).unwrap(), installed(old_pin));
    f.file
        .available_archives
        .insert(Pin::from_value(&new_pin).unwrap(), installed(new_pin));
    assert_eq!(
        f.text(),
        "plan\ndifference shared holds release=0.1.0; shared grants retain release=1.0.0 NAM-010\nno changes\n"
    );
}

#[test]
fn row_20_nested_conduit_change_requires_nested_replacement() {
    let mut f = Fixture::default();
    let parent = Name::new("Agent", "a");
    let child = Name::new("Conduit", "nested");
    f.engine.objects.insert(
        parent,
        object(
            "ready",
            json!({"slots":{"conduits":{"project":child.reference()}}}),
        ),
    );
    f.observed(&child, object("ready", grant(&["read"])));
    let mut wider = grant(&["read", "write"]);
    wider["from"] = json!("example.org/release#read-write");
    f.want(&child, object("registered", wider));
    f.line("stop Conduit/nested place=/from requires=--replace nested LIF-009");
    f.want(&child, object("registered", grant(&[])));
    f.line(&format!(
        "4 management.register-replacement.v1 Agent/a expected={ID} neither flag=- LIF-009"
    ));
    f.line("detail nested=project");
    f.line("6 access.open.v1 Conduit/(step 4).object expected=(step 4) widens flag=- LIF-037");
}

#[test]
fn row_21_subset_closes_lists_lifts_and_activates_without_a_flag() {
    let mut f = Fixture::default();
    let name = Name::new("Conduit", "project");
    f.observed(&name, object("ready", grant(&["read", "write"])));
    f.want(&name, object("registered", grant(&["read"])));
    f.line(&format!(
        "1 access.close.v1 Conduit/project expected={ID} narrows flag=- LIF-037"
    ));
    f.line("3 access.lift.v1 Tombstone/project expected=(step 2) neither flag=- ACC-053");
    f.line("4 management.register.v1 Conduit/project expected=(step 4) adds flag=- SCT-007");
    f.line("6 access.open.v1 Conduit/project expected=(step 4) widens flag=- LIF-037");
    let plan = compare(&f.file, &f.kept, &f.engine);
    assert!(matches!(&plan.steps[2].action.binding,
        Some(apply::Binding::Tombstone { closed: 1, listed: 2, object, .. }) if object == ID));
    assert!(plan.steps[4].action.check.is_some());
    assert!(
        plan.steps
            .iter()
            .all(|step| !matches!(step.action.effect, apply::Effect::Unspecified))
    );
}

#[test]
fn row_22_wider_grant_requires_replace() {
    let mut f = Fixture::default();
    let name = Name::new("Conduit", "project");
    f.observed(&name, object("ready", grant(&["read"])));
    f.want(&name, object("registered", grant(&["read", "write"])));
    f.line("stop Conduit/project place=/resolved/operations requires=--replace project OBJ-011");
}

#[test]
fn row_23_control_drift_is_preserved_or_stops() {
    let mut f = Fixture::default();
    let name = Name::new("Resource", "reader");
    let prior = object("ready", json!({"resolved":{"config":{"root":"prior"}}}));
    f.observed(&name, prior.clone());
    f.want(&name, prior.clone());
    f.engine.objects.get_mut(&name).unwrap().definition["resolved"]["config"]["root"] =
        json!("controlled");
    f.line("difference Resource/reader setting=/root changed-by=control REV-007");
    f.file.objects.get_mut(&name).unwrap().resolved.definition["description"] = json!("edit");
    f.line("stop Resource/reader setting=/root changed-by=control REV-007");
}

#[test]
fn row_24_owner_with_members_cannot_be_replaced() {
    let mut f = Fixture::default();
    let name = Name::new("Fleet", "team");
    f.observed(&name, object("ready", json!({})));
    f.engine.objects.insert(
        Name::new("Agent", "a"),
        object("ready", json!({"owner":"team"})),
    );
    f.want(&name, object("registered", json!({"description":"edit"})));
    f.line("stop Fleet/team place=/description LIF-020");
}

#[test]
fn row_25_config_edit_names_available_control_only_when_ready() {
    let mut f = Fixture::default();
    let name = Name::new("Resource", "reader");
    f.observed(
        &name,
        object("ready", json!({"resolved":{"config":{"root":"prior"}}})),
    );
    f.want(
        &name,
        object("registered", json!({"resolved":{"config":{"root":"new"}}})),
    )
    .controls
    .insert(
        "/root".into(),
        BTreeSet::from(["example.org/release#set-root.v1".into()]),
    );
    f.line("stop Resource/reader setting=/root control=example.org/release#set-root.v1 requires=--replace reader OBJ-011");
    f.engine.objects.get_mut(&name).unwrap().state = "registered".into();
    f.line("stop Resource/reader setting=/root requires=--replace reader OBJ-011");
}

#[test]
fn row_26_upgrade_and_downgrade_test_execution_and_hidden_items() {
    // Safety cases 9 and 18.
    let mut f = Fixture::default();
    let name = Name::new("Resource", "reader");
    let mut old = object(
        "ready",
        json!({"resolved":{"type":"example.org/release#resource.v1"}}),
    );
    let mut next = old.clone();
    old.packages = vec![json!({"address":"example.org/release","version":"2.0.0","digest":OTHER})];
    next.packages =
        vec![json!({"address":"example.org/release","version":"1.0.0","digest":DIGEST})];
    let operation = "example.org/release#read.v1";
    let item = json!({"implements":["#read.v1"],"command":["reader"]});
    for instance in [&mut old, &mut next] {
        let identity = &instance.packages[0];
        instance.implementations[operation] = json!({
            "selection":"example.org/release#reader","package":identity,"item":item
        });
        let Archive::Read(mut archive) = installed(identity.clone()) else {
            unreachable!("the simulated release has a readable archive");
        };
        archive.package.project.manifest = Some(json!({"implementations":{"reader":item}}));
        archive.package.project.documents.push(json!({
            "kind":"Type","metadata":{"name":"resource.v1"},"spec":{"offers":["#files.v1"]}
        }));
        if identity["version"] == "2.0.0" {
            archive.package.project.documents[0]["spec"]["inputSchema"] = json!({
                "type":"object","properties":{"offset":{"type":"integer"}}
            });
        }
        f.file
            .available_archives
            .insert(Pin::from_value(identity).unwrap(), Archive::Read(archive));
    }
    f.observed(&name, next.clone());
    f.want(&name, old.clone());
    f.line("stop Resource/reader pin=example.org/release@2.0.0 requires=--replace reader OBJ-011");
    f.observed(&name, old.clone());
    f.want(&name, next);
    f.line("stop Resource/reader pin=example.org/release@1.0.0 requires=--replace reader OBJ-011");
    f.file.project.manifest = Some(json!({"address":"example.org/project"}));
    old.packages = vec![submitted("example.org/project", DIGEST)];
    old.implementations = json!({"example.org/project#read.v1":{
        "selection":"example.org/project#reader","package":{"address":"example.org/project","digest":DIGEST},
        "item":{"remote":{"headers":[{"name":"X-Example","isSecret":true,"default":"(secret: not shown)"}]}}
    }});
    f.observed(&name, old.clone());
    old.packages = vec![submitted("example.org/project", OTHER)];
    old.implementations["example.org/project#read.v1"]["package"]["digest"] = json!(OTHER);
    f.want(&name, old);
    f.line("stop Resource/reader item=example.org/project#reader secret=not-shown requires=--replace reader SEC-021");
}

#[test]
fn row_27_only_held_manifest_address_stops() {
    let mut f = Fixture::default();
    let name = Name::new("Pool", "team");
    let instance = object(
        "ready",
        json!({"resolved":{"type":format!("{CORE}#pool.v1")}}),
    );
    f.observed(&name, instance.clone());
    f.want(&name, instance);
    f.file.project.manifest = Some(json!({"address":"example.org/new"}));
    f.no_changes();

    f.engine.objects.get_mut(&name).unwrap().packages = vec![submitted("example.org/old", DIGEST)];
    assert_eq!(
        f.text(),
        "plan\nstop Manifest/- place=/address rule=none -\nflags []\nstopped\n"
    );
}

#[test]
fn ordering_narrows_before_widens() {
    // T4: B's narrowing is still planned when the independently widening A needs approval.
    let mut f = Fixture::default();
    let a = Name::new("Conduit", "agent-a--project");
    let b = Name::new("Conduit", "agent-b--project");
    f.observed(&a, object("ready", grant(&["read"])));
    f.observed(&b, object("ready", grant(&["read", "write"])));
    f.want(&a, object("registered", grant(&["read", "write"])));
    f.want(&b, object("registered", grant(&["read"])));
    let pool = Name::new("Pool", "team");
    let resource = Name::new("Resource", "new");
    f.want(&resource, object("registered", json!({"owner":"team"})));
    f.want(&pool, object("registered", json!({})));
    let plan = compare(&f.file, &f.kept, &f.engine);
    assert_eq!(plan.steps[0].subject, b);
    assert_eq!(plan.steps[0].class, "narrows");
    let activation: Vec<_> = plan
        .steps
        .iter()
        .filter(|step| step.tool.ends_with(".set-up.v1") || step.tool.ends_with(".open.v1"))
        .map(|step| &step.subject)
        .collect();
    assert_eq!(activation, [&pool, &resource, &b]);
    f.line("stop Conduit/agent-a--project place=/resolved/operations requires=--replace agent-a--project OBJ-011");
}

#[test]
fn own_tools_lines() {
    let mut f = Fixture::default();
    let agent = Name::new("Agent", "a");
    let conduit = Name::new("Conduit", "to-agent");
    let own = f.want(&agent, object("registered", json!({"owner":"s1"})));
    own.own_tools = Some(json!({"stated":false}));
    f.line("detail ownTools Agent/a {\"stated\":false} CTR-014");
    let own = json!({"stated":true,"directory":"/lab/tools","directoryNamed":true,"pointer":"/directory","enforcement":"neither"});
    f.file.objects.get_mut(&agent).unwrap().own_tools = Some(own.clone());
    let mut def = grant(&["example.org/release#turn.v1"]);
    def["target"] = agent.reference();
    f.want(&conduit, object("registered", def)).model_work = true;
    let expected = format!("detail targetOwnTools Agent/a {} CTR-014", safe(&own));
    f.line(&expected);
    let text = f.text();
    assert!(text.contains(&format!(
        "detail operations=[example.org/release#turn.v1]\n{expected}\n"
    )));
    assert!(text.contains("pathSha256"));
    assert!(!text.contains("/lab/tools"));
}

#[test]
fn kept_state_is_read_or_refused() {
    let directory = tempfile::tempdir().unwrap();
    let file = directory.path().join("state.yaml");
    assert!(Ledger::read(&file).unwrap().objects.is_empty());
    let mut valid = json!({"format":1,"files":{},"objects":[["Pool","team",ID,"r1","ready",{},[]]],"tombstones":[],"plans":[],"journals":{}});
    let write = |value: &Value| std::fs::write(&file, value.to_string()).unwrap();
    valid["journals"][DIGEST] = json!({"not":{"decoded":[1]}});
    write(&valid);
    assert_eq!(Ledger::read(&file).unwrap().objects.len(), 1);
    let mut wrong = valid.clone();
    wrong["format"] = json!(2);
    write(&wrong);
    assert!(Ledger::read(&file).unwrap_err().contains("/format"));
    let mut wrong = valid.clone();
    wrong["unexpected"] = json!(true);
    write(&wrong);
    assert!(Ledger::read(&file).unwrap_err().contains("/unexpected"));
    let mut wrong = valid.clone();
    wrong["objects"][0].as_array_mut().unwrap().pop();
    write(&wrong);
    assert!(Ledger::read(&file).unwrap_err().contains("/objects/0"));
    valid["objects"][0][2] = json!("bad");
    write(&valid);
    assert!(Ledger::read(&file).unwrap_err().contains("/objects/0/2"));
}

#[test]
fn review_a_01_dropped_name_reused_by_another_copy() {
    let mut f = Fixture::default();
    let name = Name::new("Conduit", "project");
    let mut observation = object("ready", grant(&["read"]));
    f.observed(&name, observation.clone());
    observation.id = "00000000-0000-4000-8000-000000000002".into();
    f.engine.objects.insert(name, observation);
    f.no_changes();
}

#[test]
fn review_a_02_only_core_bound_arguments_can_prove_narrowing() {
    let mut f = Fixture::default();
    let name = Name::new("Conduit", "project");
    let mut before = grant(&["read"]);
    before["resolved"]["requirements"]["filter"] = json!({
        "requirement":"example.org/release#bound.v1","arguments":{"operations":["read","write"]}
    });
    let mut after = before.clone();
    after["resolved"]["requirements"]["filter"]["arguments"]["operations"] = json!(["read"]);
    f.observed(&name, object("ready", before.clone()));
    f.want(&name, object("registered", after.clone()));
    f.line("stop Conduit/project place=/resolved/requirements/filter/arguments/operations requires=--replace project OBJ-011");
    assert!(compare(&f.file, &f.kept, &f.engine).steps.is_empty());

    for definition in [&mut before, &mut after] {
        definition["resolved"]["requirements"]["filter"]["requirement"] =
            json!(format!("{CORE}#bound.v1"));
    }
    f.observed(&name, object("ready", before));
    f.want(&name, object("registered", after));
    f.line(&format!(
        "1 access.close.v1 Conduit/project expected={ID} narrows flag=- LIF-037"
    ));
}

#[test]
fn review_a_04_removed_retired_instance_waits_for_shutdown() {
    let mut f = Fixture::default();
    let name = Name::new("Resource", "reader");
    let mut observation = object("retired", json!({"owner":"team"}));
    observation.status = json!({"shutdown":"unknown"});
    f.observed(&name, observation);
    assert_eq!(
        f.text(),
        "plan\nstop Resource/reader state=retired rule=none -\nflags []\nstopped\n"
    );
}

#[test]
fn review_a_05_owner_preserves_control_drift_and_blocks_pin_replacement() {
    let mut f = Fixture::default();
    let name = Name::new("Pool", "team");
    let mut kept = object(
        "ready",
        json!({"resolved":{"type":"example.org/release#pool.v1","config":{"limit":1}}}),
    );
    kept.packages =
        vec![json!({"address":"example.org/release","version":"1.0.0","digest":DIGEST})];
    f.observed(&name, kept.clone());
    f.engine.objects.get_mut(&name).unwrap().definition["resolved"]["config"]["limit"] = json!(2);
    f.engine.objects.insert(
        Name::new("Resource", "reader"),
        object("ready", json!({"owner":"team"})),
    );
    f.want(&name, kept.clone());
    f.line("difference Pool/team setting=/limit changed-by=control REV-007");

    let mut wanted = kept;
    wanted.packages =
        vec![json!({"address":"example.org/release","version":"2.0.0","digest":OTHER})];
    f.want(&name, wanted);
    f.line("stop Pool/team setting=/limit changed-by=control REV-007");
    f.line("stop Pool/team pin=example.org/release@2.0.0 LIF-020");
    assert!(!f.text().contains("requires=--replace team"));
}

#[test]
fn review_a_06_nullable_setting_changes_presence() {
    let mut f = Fixture::default();
    let name = Name::new("Resource", "reader");
    let missing = object("ready", json!({"resolved":{"config":{"fixed":true}}}));
    let mut explicit_null = missing.clone();
    explicit_null.definition["resolved"]["config"]["optional"] = Value::Null;
    for (before, after) in [(&missing, &explicit_null), (&explicit_null, &missing)] {
        f.observed(&name, before.clone());
        f.want(&name, after.clone());
        f.line("stop Resource/reader setting=/optional requires=--replace reader OBJ-011");
    }
}

#[test]
fn review_a_07_address_stop_keeps_independent_tombstone_stop() {
    let mut f = Fixture::default();
    let pool = Name::new("Pool", "team");
    let mut held = object("ready", json!({}));
    held.packages.push(submitted("example.org/old", DIGEST));
    f.observed(&pool, held.clone());
    f.want(&pool, held);
    f.file.project.manifest = Some(json!({"address":"example.org/new"}));
    f.want(
        &Name::new("Resource", "reader"),
        object("registered", json!({})),
    );
    f.engine
        .tombstones
        .push(json!({"id":"stone","object":{"kind":"Resource","name":"reader"}}));
    f.line("stop Manifest/- place=/address rule=none -");
    f.line("stop Resource/reader tombstone=stone requires=--recreate reader LIF-041");
}

#[test]
fn review_a_08_runtime_provider_precedes_dependent_and_verification_is_adjacent() {
    let mut f = Fixture::default();
    let consumer = Name::new("Resource", "a-consumer");
    let provider = Name::new("Resource", "z-provider");
    let independent = Name::new("Resource", "b-independent");
    f.want(
        &consumer,
        object(
            "registered",
            json!({"owner":"team","runtime":provider.reference()}),
        ),
    );
    f.want(&provider, object("registered", json!({"owner":"team"})));
    f.want(&independent, object("registered", json!({"owner":"team"})));
    let plan = compare(&f.file, &f.kept, &f.engine);
    let activations: Vec<_> = plan
        .steps
        .windows(2)
        .filter(|pair| pair[1].tool == "team.set-up.v1")
        .map(|pair| {
            assert_eq!(pair[0].tool, "team.inspect.v1");
            assert_eq!(pair[0].subject, pair[1].subject);
            &pair[1].subject
        })
        .collect();
    assert_eq!(activations, [&independent, &provider, &consumer]);
}

#[test]
fn review_248_all_matching_tombstones_need_approval() {
    let mut f = Fixture::default();
    let name = Name::new("Conduit", "new");
    f.want(&name, object("registered", grant(&["read"])));
    for (id, local, target) in [("local", "project", "other"), ("route", "other", "shared")] {
        f.engine.tombstones.push(json!({
            "id":id,"object":{"kind":"Conduit","name":id},
            "consumer":{"kind":"Agent","name":"a"},"target":{"kind":"Resource","name":target},
            "localName":local,"capability":"example.org/release#files.v1"
        }));
    }
    let check = |f: &Fixture, flags: &[Approval]| {
        compare::plan(
            &f.file,
            &f.kept,
            &f.engine,
            &flags.iter().cloned().collect(),
        )
    };
    let stopped = check(&f, &[]);
    assert_eq!(stopped.stops.len(), 2, "both LIF-041 matches need approval");
    let allowed = check(&f, &[Approval::new("--reopen", "new")]);
    assert_eq!(
        allowed
            .steps
            .iter()
            .filter(|s| s.tool.ends_with(".lift.v1"))
            .count(),
        2
    );

    let mut old = grant(&["read"]);
    old["resolved"]["capability"] = json!("example.org/old#files.v1");
    f.observed(&name, object("ready", old));
    let stopped = check(&f, &[Approval::new("--replace", "new")]);
    assert!(stopped.stops.iter().any(|s| s.contains("tombstone=route")));
}

#[test]
fn review_248_incident_nested_conduit_with_held_target_stops_replacement() {
    let mut f = Fixture::default();
    let endpoint = Name::new("Resource", "shared");
    let parent = Name::new("Agent", "a");
    let child = Name::new("Conduit", "nested");
    let before = object(
        "ready",
        json!({"owner":"team","resolved":{"config":{"root":"old"}}}),
    );
    let mut after = before.clone();
    after.definition["resolved"]["config"]["root"] = json!("new");
    f.observed(&endpoint, before);
    f.want(&endpoint, after);
    let parent_view = object(
        "ready",
        json!({
            "slots":{"conduits":{"project":child.reference()}},
            "resolved":{"conduits":{"project":{"from":"files","target":endpoint.reference()}}}
        }),
    );
    f.observed(&parent, parent_view.clone());
    f.want(&parent, parent_view);
    let existing = object("ready", grant(&["read"]));
    f.observed(&child, existing.clone());
    f.want(&child, existing);
    let plan = compare::plan(
        &f.file,
        &f.kept,
        &f.engine,
        &BTreeSet::from([Approval::new("--replace", "shared")]),
    );
    assert!(
        plan.text()
            .contains("stop Conduit/nested place=/target LIF-009"),
        "a surviving parent's held target must not be rebound through ordinary registration"
    );
}

#[test]
fn review_248_replacing_parent_registers_implicit_child_once() {
    let mut f = Fixture::default();
    let parent = Name::new("Agent", "a");
    let child = Name::new("Conduit", "nested");
    let next_child = Name::new("Conduit", "draft-child");
    let before = object(
        "ready",
        json!({"slots":{"conduits":{"project":child.reference()}}}),
    );
    let mut after = before.clone();
    after.definition["annotations"] = json!({"purpose":"replacement"});
    after.definition["slots"]["conduits"]["project"] = next_child.reference();
    f.observed(&parent, before);
    f.want(&parent, after);
    let view = object("ready", grant(&["read"]));
    f.observed(&child, view.clone());
    f.want(&child, view.clone());
    f.file.nested.push(NestedRegistration {
        parent: parent.clone(),
        local: "project".into(),
        name: next_child.clone(),
        resolved: view,
    });
    let plan = compare::plan(
        &f.file,
        &f.kept,
        &f.engine,
        &BTreeSet::from([Approval::new("--replace", "a")]),
    );
    assert!(plan.stops.is_empty());
    let registrations: Vec<_> = plan
        .steps
        .iter()
        .filter_map(|step| {
            if let apply::Effect::Register {
                objects, nested, ..
            } = &step.action.effect
            {
                Some((objects, nested))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(registrations.len(), 1);
    let (objects, nested) = registrations[0];
    assert_eq!(objects.keys().collect::<Vec<_>>(), [&parent]);
    assert_eq!(nested.len(), 1);
    assert_eq!(nested[0].name, next_child);
}

#[test]
fn review_248_affects_uses_effective_coverage_and_held_type() {
    let mut f = Fixture::default();
    let endpoint = Name::new("Resource", "shared");
    let before = object(
        "ready",
        json!({"owner":"old","resolved":{"type":"example.org/release#files.v1"}}),
    );
    let mut after = before.clone();
    after.definition["owner"] = json!("new");
    f.observed(&endpoint, before);
    f.want(&endpoint, after);
    let holder = Name::new("Conduit", "bound");
    f.engine.objects.insert(holder, object("ready", json!({"resolved":{
        "requirements":{"bound":{"requirement":format!("{CORE}#bound.v1"),
            "arguments":{"targets":[endpoint.reference()],"pools":[{"kind":"Pool","name":"new"}]}}}
    }})));
    let plan = compare::plan(
        &f.file,
        &f.kept,
        &f.engine,
        &BTreeSet::from([Approval::new("--replace", "shared")]),
    );
    assert!(
        plan.bounds_added.is_empty(),
        "coverage already existed through targets"
    );
    assert!(plan.affected_bounds.is_empty());

    let agent = Name::new("Agent", "a");
    let pin = json!({"address":"example.org/release","version":"1.0.0","digest":DIGEST});
    let Archive::Read(mut archive) = installed(pin.clone()) else {
        unreachable!()
    };
    archive.package.project.documents.push(json!({
        "kind":"Type","metadata":{"name":"agent.v1"},"spec":{"signIn":{}}
    }));
    let mut before = object(
        "ready",
        json!({"resolved":{"type":"example.org/release#agent.v1"}}),
    );
    before.packages.push(pin.clone());
    let after = object(
        "ready",
        json!({"resolved":{"type":"example.org/no-login#agent.v1"}}),
    );
    f.observed(&agent, before);
    f.want(&agent, after);
    let key = Pin::from_value(&pin).unwrap();
    f.file
        .available_archives
        .insert(key.clone(), Archive::Read(archive.clone()));
    let approved = BTreeSet::from([
        Approval::new("--replace", "a"),
        Approval::new("--replace", "shared"),
    ]);
    let plan = compare::plan(&f.file, &f.kept, &f.engine, &approved);
    assert!(
        plan.signed_out.contains(&agent),
        "the held Type states signIn"
    );
    archive.package.project.documents.last_mut().unwrap()["spec"] = json!({});
    f.file
        .available_archives
        .insert(key, Archive::Read(archive));
    assert!(
        !compare::plan(&f.file, &f.kept, &f.engine, &approved)
            .signed_out
            .contains(&agent)
    );
}

#[test]
fn plan_apply_prune_removes_templates_and_retries() {
    let mut f = Fixture::default();
    let template = Name::new("Conduit", "unused");
    f.observed(&template, object("template", json!({})));
    let approval = BTreeSet::from([Approval::new("--prune", "unused")]);
    let plan = compare::plan(&f.file, &f.kept, &f.engine, &approval);
    assert!(
        plan.text()
            .contains("management.unregister.v1 Conduit/unused"),
        "approved standalone template removal needs unregister.v1"
    );

    let instance = Name::new("Resource", "history");
    let mut old = object("retired", json!({"owner":"team"}));
    old.status = json!({"shutdown":"pending"});
    f.observed(&instance, old);
    // Recover the permission of a dispatched retirement from its kept journal.
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path().join(".conduit");
    std::fs::create_dir_all(base.join("plans")).unwrap();
    std::fs::create_dir_all(base.join("results")).unwrap();
    let recorded = "plan\nflags [--prune history]\nready\n";
    let digest = bytes_digest(recorded.as_bytes());
    let hex = digest.trim_start_matches("sha256:");
    std::fs::write(base.join(format!("plans/{hex}.txt")), recorded).unwrap();
    std::fs::write(
        base.join(format!("results/{hex}.yaml")),
        json!({"attempt":1,"entries":[{
        "tool":"team.retire.v1","flag":"--prune history","retirementDispatched":true,
        "arguments":{"instance":instance.reference(),"expectedId":ID},
        "result":{"outcome":"failed","invocation":"retirement"}}]})
        .to_string(),
    )
    .unwrap();
    std::fs::write(base.join("state.yaml"), json!({"format":1,"objects":[["Resource","history",ID,"r1","retired",{"owner":"team"},[]]],
        "files":{},"plans":[digest],"tombstones":[],"journals":{(digest.clone()):format!(".conduit/results/{hex}.yaml")}}).to_string()).unwrap();
    f.kept = Ledger::read(&base.join("state.yaml")).unwrap();
    f.observed(
        &Name::new("Resource", "history--t"),
        object("template", json!({})),
    );
    let plan = compare::plan(&f.file, &f.kept, &f.engine, &BTreeSet::new());
    assert!(
        plan.text()
            .contains("state=retired requires=--prune history LIF-026")
    );
    let plan = compare::plan(
        &f.file,
        &f.kept,
        &f.engine,
        &BTreeSet::from([Approval::new("--prune", "history")]),
    );
    assert!(plan.stops.is_empty());
    assert_eq!(
        plan.steps
            .iter()
            .map(|s| s.tool.as_str())
            .collect::<Vec<_>>(),
        [
            "team.retire.v1",
            "management.unregister.v1",
            "management.list.v1"
        ]
    );
}

#[tokio::test]
async fn review_249_bare_apply_does_not_open_an_adopted_conduit() {
    let mut f = Fixture::default();
    let name = Name::new("Conduit", "grant");
    let view = object("registered", json!({}));
    f.engine.objects.insert(name.clone(), view.clone());
    f.want(&name, view);
    let directory = tempfile::tempdir().unwrap();
    let plan = compare::plan(
        &f.file,
        &f.kept,
        &f.engine,
        &BTreeSet::from([Approval::new("--adopt", "grant")]),
    );
    assert_eq!(
        apply::execute(directory.path(), &plan, f.kept, async |tool, _, _| {
            assert_eq!(tool, "management.inspect.v1");
            Ok(
                json!({"_meta":{"io.github.shrav-k.conduit/outcome":"succeeded"},
            "structuredContent":{"id":ID,"revision":"r1","declared":{},"packages":[],
                "status":{"state":"registered"}}}),
            )
        })
        .await,
        0
    );
    let kept = Ledger::read(&directory.path().join(".conduit/state.yaml")).unwrap();
    let plan = compare::plan(&f.file, &kept, &f.engine, &BTreeSet::new());
    assert!(
        !plan
            .steps
            .iter()
            .any(|step| step.tool.ends_with(".open.v1")),
        "bare apply must not open a foreign grant taken into the ledger by adoption"
    );
    assert_eq!(
        apply::execute(directory.path(), &plan, kept, async |_, _, _| {
            panic!("bare apply must send nothing for the adopted registered grant")
        })
        .await,
        0
    );
}

#[test]
fn review_249_settled_setting_ignores_old_unknown_record() {
    let mut f = Fixture::default();
    let name = Name::new("Resource", "shared");
    let mut observed = object("ready", json!({"resolved":{"config":{"limit":10}}}));
    observed.status = json!({"unreconciled":[]});
    observed
        .unknown_endings
        .insert("old-unknown".into(), "configure.v1".into());
    f.observed(&name, observed.clone());
    observed.definition["resolved"]["config"]["limit"] = json!(20);
    f.want(&name, observed)
        .controls
        .insert("/limit".into(), BTreeSet::from(["configure.v1".into()]));
    let plan = compare::plan(
        &f.file,
        &f.kept,
        &f.engine,
        &BTreeSet::from([Approval::new("--replace", "shared")]),
    );
    assert!(
        !plan.text().contains("REV-009"),
        "a settled setting must not stop the replacement"
    );
}

#[test]
fn review_249_foreign_retired_object_reports_a_stop() {
    let mut f = Fixture::default();
    let name = Name::new("Resource", "history");
    let mut view = object("retired", json!({}));
    view.status = json!({"shutdown":"confirmed"});
    f.engine.objects.insert(name.clone(), view.clone());
    f.want(&name, view);
    f.line("stop Resource/history state=retired rule=none -");
}

#[test]
fn plan_apply_with_resources_stops_and_cleans_up() {
    let mut f = Fixture::default();
    let pool = Name::new("Pool", "team");
    let member = Name::new("Resource", "history");
    let grant = Name::new("Conduit", "cascade-team");
    f.observed(&pool, object("ready", json!({})));
    let mut old = object("ready", json!({"owner":"team"}));
    old.id = "00000000-0000-4000-8000-000000000002".into();
    old.status = json!({"unreconciled":["unknown-call"]});
    f.observed(&member, old);
    let approved = BTreeSet::from([Approval::new("--with-resources", "team")]);
    let plan = compare::plan(&f.file, &f.kept, &f.engine, &approved);
    assert!(
        plan.text()
            .contains("stop Resource/history unreconciled=unknown-call OPS-013")
    );
    assert!(
        plan.steps
            .iter()
            .any(|step| step.tool == "cascade-team.retire-with-resources.v1"),
        "cascade is planned before the row 5 stop withholds it"
    );

    let mut current = object(
        "ready",
        json!({
            "from":format!("{CORE}#retire-pool"),"consumer":{"kind":"Interface","name":"root"},
            "target":pool.reference(),"as":grant.name
        }),
    );
    current.id = "00000000-0000-4000-8000-000000000003".into();
    f.engine.objects.insert(grant.clone(), current.clone());
    f.engine.objects.get_mut(&member).unwrap().status = Value::Null;
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("state.yaml"),
        json!({"format":1,
        "objects":[["Pool","team",ID,"r1","ready",{},[]],
            ["Resource","history",f.engine.objects[&member].id,"r1","ready",{"owner":"team"},[]]],
        "files":{},"plans":[],"tombstones":[],"journals":{},"cascades":[{
            "conduit":grant.reference(),"id":current.id,"container":pool.reference(),
            "flag":"--with-resources team"}]})
        .to_string(),
    )
    .unwrap();
    f.kept = Ledger::read(&dir.path().join("state.yaml")).unwrap();
    let plan = compare::plan(&f.file, &f.kept, &f.engine, &approved);
    assert!(plan.stops.is_empty());
    assert_eq!(
        plan.steps
            .iter()
            .take(4)
            .map(|step| step.tool.as_str())
            .collect::<Vec<_>>(),
        [
            "management.inspect.v1",
            "access.close.v1",
            "management.list.v1",
            "access.lift.v1"
        ]
    );
    let bare = compare::plan(&f.file, &f.kept, &f.engine, &BTreeSet::new());
    assert!(bare.steps.iter().any(|step| step.tool == "access.close.v1"));
    assert!(!bare.steps.iter().any(|step| step.tool == "access.lift.v1"));
    let template = Name::new("Resource", "history--t");
    let mut template_view = object("template", json!({}));
    template_view.id = "00000000-0000-4000-8000-000000000004".into();
    f.observed(&template, template_view);
    f.engine.objects.remove(&member);
    let resumed = compare::plan(&f.file, &f.kept, &f.engine, &approved);
    assert!(resumed.stops.is_empty());
    assert!(resumed.steps.iter().any(|step| step.subject == template
        && step.tool == "management.unregister.v1"
        && step.action.approvals == approved));
    f.engine.objects.get_mut(&grant).unwrap().id = "another-id".into();
    let changed = compare::plan(&f.file, &f.kept, &f.engine, &approved);
    assert!(
        changed
            .stops
            .iter()
            .any(|stop| stop.starts_with("stop Conduit/cascade-team "))
    );
    assert!(
        !changed
            .steps
            .iter()
            .any(|step| step.subject == grant && step.tool.ends_with(".close.v1"))
    );
}
