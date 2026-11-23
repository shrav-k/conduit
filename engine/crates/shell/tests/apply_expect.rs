mod package_target;
mod plan_apply_support;

use conduit_shell::boundary;
use plan_apply_support::cli::*;
use plan_apply_support::{
    Engine, Symbols, cases, installs, replay_while, repository, result, sha256,
};
use serde_json::json;
use std::{fs, path::Path, process::Output};

fn command(engine: &Engine, dir: &Path, config: &Path, args: &[&str]) -> Output {
    tokio::task::block_in_place(|| tool(engine, dir, config, args))
}

fn unchanged<T>(engine: &Engine, dir: &Path, run: impl FnOnce() -> T) -> T {
    let kept = dir.join(".conduit");
    let snapshot = || kept.exists().then(|| files(&kept));
    let before = snapshot();
    let requests = non_read_requests(engine);
    let output = run();
    assert!(snapshot() == before, ".conduit changed");
    assert!(
        non_read_requests(engine) == requests,
        "non-read requests changed"
    );
    output
}

fn stderr(output: &Output) -> &str {
    std::str::from_utf8(&output.stderr).unwrap()
}

#[track_caller]
fn plan_digest(output: &Output, code: i32) -> &str {
    assert_eq!(output.status.code(), Some(code), "plan exit status");
    let line = stderr(output);
    let (digest, _) = line
        .strip_prefix("conduit plan: digest ")
        .and_then(|message| message.split_once(';'))
        .expect("plan stderr has no digest line");
    assert_eq!(
        line,
        format!(
            "conduit plan: digest {digest}; conduit apply --expect {digest}, given the same \
             project, store and approval flags, sends nothing unless the plan it computes has \
             this digest\n"
        )
    );
    assert_eq!(digest, format!("sha256:{}", sha256(stdin_free(output))));
    digest
}

#[track_caller]
fn refusal_digest<'a>(output: &'a Output, old: &str, status: &str) -> &'a str {
    assert_eq!(output.status.code(), Some(2), "refused apply exit status");
    let line = stderr(output);
    let (new, _) = line
        .strip_prefix("conduit apply: the plan computed now has digest ")
        .and_then(|message| message.split_once(','))
        .expect("apply stderr has no digest refusal");
    let tail = match status {
        "ready" => format!("review it, then rerun this command with --expect {new} to send it"),
        "stopped" => format!(
            "it is stopped, so --expect {new} sends nothing either: resolve each stop line, run \
             conduit plan with the approval flags apply will be given, and give apply the digest \
             it prints"
        ),
        "no changes" => "it has no changes, so there is nothing to send".to_owned(),
        _ => unreachable!(),
    };
    assert_eq!(
        line,
        format!(
            "conduit apply: the plan computed now has digest {new}, not {old} as --expect gives; \
             nothing was sent or kept; stdout holds that plan; the digest covers what it is \
             computed from: the engine, the project files, .conduit/ and the approval flags given; \
             no earlier plan is kept, so the lines that changed are not named; {tail}\n"
        )
    );
    assert_ne!(new, old);
    new
}

fn assert_refused_plan(output: &Output, plan: &str) {
    let steps = plan
        .lines()
        .filter(|line| line.starts_with(|ch: char| ch.is_ascii_digit()))
        .count();
    let tail = stdin_free(output)
        .strip_prefix(plan)
        .expect("refusal does not print the live plan");
    if steps == 0 {
        assert!(tail.is_empty());
    } else {
        assert_eq!(tail, format!("not sent 1..{steps}\n"));
    }
}

async fn at_case(id: &str, mut test: impl FnMut(&Engine, &Path, &Path)) {
    let cases = cases();
    let target = cases.iter().position(|case| case["id"] == id).unwrap();
    let replay = replay_while(&cases[..=target], |engine, case, symbols| {
        if case["id"] != id {
            return true;
        }
        let dir = tempfile::tempdir().unwrap();
        let config = tempfile::tempdir().unwrap();
        cache_fixture(config.path(), id[..2].parse().unwrap());
        project_fixture(dir.path(), case, Some(symbols));
        test(engine, dir.path(), config.path());
        false
    })
    .await;
    assert!(replay.lines.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_prints_its_digest_on_stderr() {
    let (engine, config, dir, case) = first_case().await;
    let (ready, stopped) = unchanged(&engine, dir.path(), || {
        (
            command(&engine, dir.path(), config.path(), &["plan"]),
            command(
                &engine,
                dir.path(),
                config.path(),
                &["plan", "--replace", "nothing"],
            ),
        )
    });
    assert!(stdin_free(&ready) == case["recorded"]["text"].as_str().unwrap());
    assert!(stdin_free(&stopped).contains("stop Flag/--replace operand=nothing use=none -\n"));

    let invalid = tempfile::tempdir().unwrap();
    let context = boundary::context(&repository()).unwrap();
    let api = &context.builtin.as_ref().unwrap().manifest.as_ref().unwrap()["apiVersion"];
    let reference = format!("{}#secret-reference.v1", context.core_package);
    let documents = json!([
        {"apiVersion":api,"kind":"Type","metadata":{"name":"config.v1"},"spec":{
            "family":"Resource","schema":{"type":"object","properties":{"value":{"$ref":reference}}}
        }},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"example--t"},"spec":{
            "type":"#config.v1","config":{"value":"invalid-reference-example"}
        }}
    ]);
    fs::write(
        invalid.path().join("conduit.json"),
        json!({"apiVersion":api,"address":"example.org/demo","packages":{}}).to_string(),
    )
    .unwrap();
    fs::write(invalid.path().join("documents.json"), documents.to_string()).unwrap();
    let secret_stop = binary(invalid.path(), config.path())
        .arg("plan")
        .output()
        .unwrap();
    assert_eq!(
        stdin_free(&secret_stop),
        "plan\nstop Resource/example--t place=/spec/config/value SEC-001\nflags []\nstopped\n"
    );
    let digest = plan_digest(&ready, 0);
    assert_eq!(digest, case["recorded"]["digest"].as_str().unwrap());
    plan_digest(&stopped, 2);
    let secret_digest = plan_digest(&secret_stop, 2);
    let secret_with_flag = binary(invalid.path(), config.path())
        .args(["plan", "--replace", "nothing"])
        .output()
        .unwrap();
    assert_eq!(stdin_free(&secret_with_flag), stdin_free(&secret_stop));
    assert_eq!(plan_digest(&secret_with_flag, 2), secret_digest);

    let unguarded = binary(invalid.path(), config.path())
        .arg("apply")
        .output()
        .unwrap();
    assert_eq!(unguarded.status.code(), Some(2));
    assert_refused_plan(&unguarded, stdin_free(&secret_stop));
    assert!(stderr(&unguarded).is_empty());
    for expected in [secret_digest, digest] {
        let applied = binary(invalid.path(), config.path())
            .args(["apply", "--expect", expected])
            .output()
            .unwrap();
        assert_eq!(applied.status.code(), Some(2));
        assert_refused_plan(&applied, stdin_free(&secret_stop));
        if expected == secret_digest {
            assert!(stderr(&applied).is_empty());
        } else {
            assert_eq!(refusal_digest(&applied, expected, "stopped"), secret_digest);
        }
    }
    assert!(!invalid.path().join(".conduit").exists());

    let archives = installs();
    let hash = archives[0].args["digest"]
        .as_str()
        .unwrap()
        .strip_prefix("sha256:")
        .unwrap();
    fs::remove_file(config.path().join("packages").join(hash)).unwrap();
    let (cache_stop, cache_with_flag) = unchanged(&engine, dir.path(), || {
        (
            command(&engine, dir.path(), config.path(), &["plan"]),
            command(
                &engine,
                dir.path(),
                config.path(),
                &["plan", "--replace", "nothing"],
            ),
        )
    });
    assert!(stdin_free(&cache_stop).contains("archive=absent requires=conduit-install PKG-004"));
    assert_eq!(stdin_free(&cache_with_flag), stdin_free(&cache_stop));
    assert_eq!(
        plan_digest(&cache_with_flag, 2),
        plan_digest(&cache_stop, 2)
    );
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn apply_sends_the_plan_whose_digest_it_was_given() {
    let (engine, config, dir, case) = first_case().await;
    let plan = command(&engine, dir.path(), config.path(), &["plan"]);
    let digest = plan_digest(&plan, 0);
    assert_eq!(digest, case["recorded"]["digest"].as_str().unwrap());
    applied_case(
        &engine,
        dir.path(),
        config.path(),
        &case,
        &Symbols::default(),
        &["--expect", digest],
    );
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn apply_refuses_a_plan_the_project_changed_and_sends_nothing() {
    let (engine, config, dir, _) = first_case().await;
    let dir = dir.path();
    let config = config.path();
    let original = command(&engine, dir, config, &["plan"]);
    let old = plan_digest(&original, 0);

    let path = dir.join("s1.yaml");
    let mut project = boundary::value(&path).unwrap();
    project["agents"]["agent-b"]["consumes"]["project"]["from"] =
        json!("example.org/s1/filesystem#read");
    fs::write(path, project.to_string()).unwrap();
    let (refused, current) = unchanged(&engine, dir, || {
        (
            command(&engine, dir, config, &["apply", "--expect", old]),
            command(&engine, dir, config, &["plan"]),
        )
    });
    let new = refusal_digest(&refused, old, "ready");
    assert_eq!(new, plan_digest(&current, 0));
    assert_refused_plan(&refused, stdin_free(&current));

    let applied = command(&engine, dir, config, &["apply", "--expect", new]);
    assert_eq!(applied.status.code(), Some(0));
    assert!(stderr(&applied).is_empty());
    let tail = stdin_free(&applied)
        .strip_prefix(stdin_free(&current))
        .unwrap();
    assert!(tail.starts_with("result 1 succeeded "));
    let hex = new.strip_prefix("sha256:").unwrap();
    let kept = fs::read_to_string(dir.join(format!(".conduit/plans/{hex}.txt"))).unwrap();
    assert!(kept == stdin_free(&current));
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn apply_refuses_a_plan_the_engine_changed_and_sends_nothing() {
    let (engine, config, dir, case) = first_case().await;
    let dir = dir.path();
    let config = config.path();
    let original = command(&engine, dir, config, &["plan"]);
    let old = plan_digest(&original, 0);

    let original_arguments = &case["requests"][0]["arguments"];
    let pool_template = &original_arguments["documents"][0];
    assert_eq!(pool_template["metadata"]["name"], "team--t");
    let registered = result(
        &engine
            .call(
                "management.register.v1",
                json!({"documents":[pool_template],"manifest":original_arguments["manifest"]}),
                None,
            )
            .await,
    );
    assert_eq!(registered["outcome"], "succeeded");
    let (refused, current) = unchanged(&engine, dir, || {
        (
            command(&engine, dir, config, &["apply", "--expect", old]),
            command(&engine, dir, config, &["plan"]),
        )
    });
    let new = refusal_digest(&refused, old, "stopped");
    assert_eq!(new, plan_digest(&current, 2));
    assert!(stdin_free(&current).lines().any(|line| {
        line.starts_with("stop Pool/team--t ") && line.ends_with("requires=--adopt team--t OBJ-002")
    }));
    assert_refused_plan(&refused, stdin_free(&current));
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn apply_expect_on_a_no_op() {
    let cases = cases();
    let no_op = cases[2]["recorded"]["digest"].as_str().unwrap();
    let old = cases[0]["recorded"]["digest"].as_str().unwrap();
    at_case("03-t3-noop", |engine, dir, config| {
        let matched = unchanged(engine, dir, || {
            command(engine, dir, config, &["apply", "--expect", no_op])
        });
        assert_eq!(matched.status.code(), Some(0), "matched no-op exit status");
        assert_eq!(stdin_free(&matched), "plan\nno changes\n");
        assert!(stderr(&matched).is_empty());
        let refused = unchanged(engine, dir, || {
            command(engine, dir, config, &["apply", "--expect", old])
        });
        assert_eq!(refusal_digest(&refused, old, "no changes"), no_op);
        assert_refused_plan(&refused, "plan\nno changes\n");
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn expect_covers_the_approval_flags() {
    at_case("04-t4", |engine, dir, config| {
        let approved = command(
            engine,
            dir,
            config,
            &["plan", "--replace", "agent-a--project"],
        );
        let digest = plan_digest(&approved, 0);
        let (refused, stopped) = unchanged(engine, dir, || {
            (
                command(engine, dir, config, &["apply", "--expect", digest]),
                command(engine, dir, config, &["plan"]),
            )
        });
        assert_eq!(
            refusal_digest(&refused, digest, "stopped"),
            plan_digest(&stopped, 2)
        );
        assert!(stdin_free(&stopped).contains("requires=--replace agent-a--project "));
        assert_refused_plan(&refused, stdin_free(&stopped));

        let applied = command(
            engine,
            dir,
            config,
            &["apply", "--replace", "agent-a--project", "--expect", digest],
        );
        assert_eq!(applied.status.code(), Some(0));
        assert!(stderr(&applied).is_empty());
        let hex = digest.strip_prefix("sha256:").unwrap();
        let kept = fs::read_to_string(dir.join(format!(".conduit/plans/{hex}.txt"))).unwrap();
        assert!(kept == stdin_free(&approved));
    })
    .await;
}

#[test]
fn expect_usage_errors_read_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let digest = cases().remove(0)["recorded"]["digest"]
        .as_str()
        .unwrap()
        .to_owned();
    let hex = digest.strip_prefix("sha256:").unwrap();
    let upper = format!("sha256:{}", hex.to_ascii_uppercase());
    let short = &digest[..digest.len() - 1];
    let missing = "conduit apply: --expect needs sha256:HEX, the digest conduit plan prints; \
                   nothing was read or sent\n";
    let invalid = |value: &str| {
        format!(
            "conduit apply: --expect {value:?} is not sha256: and 64 lowercase hex digits; \
         nothing was read or sent\n"
        )
    };
    for (args, line) in [
        (vec!["apply", "--expect"], missing.to_owned()),
        (vec!["apply", "--expect", "--store"], missing.to_owned()),
        (vec!["apply", "--expect", hex], invalid(hex)),
        (vec!["apply", "--expect", &upper], invalid(&upper)),
        (vec!["apply", "--expect", short], invalid(short)),
        (
            vec!["apply", "--expect", &digest, "--expect", &digest],
            "conduit apply: --expect is given twice; give one digest; nothing was read or sent\n"
                .to_owned(),
        ),
        (
            vec!["plan", "--expect", &digest],
            "conduit plan: --expect is apply's; conduit plan prints the digest to give it; \
             nothing was read\n"
                .to_owned(),
        ),
    ] {
        let output = binary(dir.path(), dir.path()).args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(stderr(&output), line);
    }
    let help = binary(dir.path(), dir.path())
        .args(["apply", "--expect", "--help"])
        .output()
        .unwrap();
    assert_eq!(help.status.code(), Some(0));
    assert!(stdin_free(&help).starts_with("conduit apply [--store DIR] "));
    assert!(stderr(&help).is_empty());
    assert!(files(dir.path()).is_empty());
}
