# Work order 068 (E0): the test gate, CI split across runners, and a check for private paths

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

The engine's full suite takes 36 to 40 minutes, and CI on `main` is red. The crash tests
`tests::killed_after_commit_before_send` and `tests::killed_after_send_before_completion_commit`
panicked at `engine/crates/shell/src/tests.rs:1204`, "crash child did not reach barrier". Their child
has 30 seconds to reach a barrier, and a slow runner takes longer (finding 4-18 of increment 4). This
order makes the suite fast and reliable to run, and changes no behavior and no outcome: it changes
how tests are run and what they wait for, never what they check.

## What to do

1. **nextest as the gate.** Add `engine/.config/nextest.toml`:
   - a test group with `max-threads = 1` for the shell's crash and timing-sensitive tests, in
     `crates/shell/src/tests.rs`, `crates/shell/tests/runner.rs`, `crates/shell/tests/serve.rs`,
     `crates/shell/tests/cli.rs` and `crates/shell/tests/agents.rs`. Choose them by reading the
     tests, and list your choices in the report;
   - a `slow-timeout` per test that reports a hang without killing a slow but healthy test;
   - **no retries anywhere**. A test that passes only on a retry is a failure.
2. **Bounds on progress, not on speed.** Where a test waits for something to happen and panics if
   it has not happened by a fixed time, raise that bound so that a slow runner cannot fail it. Start
   with the barrier at `src/tests.rs:1197`. That kind of bound guards against a hang; it checks
   nothing about speed. Leave alone any assertion that checks a timing promise, such as a deadline
   the engine must keep. List each bound you changed, with the reason, and each one you left.
3. **CI split across runners.** In `.github/workflows/checks.yml`, the engine's directory:
   - builds once, with `cargo nextest archive --release`, and runs the archive in N shards with
     `--partition count:i/N` on `ubuntu-latest`. Pick N from the measured times, and say why;
   - runs doctests apart, with `cargo test --release --doc`;
   - keeps fmt and clippy as they are;
   - ends in one summary job that passes only if every shard, the doctests and the other Rust
     directories passed, so that the result's name stays `rust`.

   The other Rust directories keep `cargo test --release`. Pin nextest's version. Use no secret, no
   paid runner and no service other than GitHub's. Keep the comments' style: say why each step is
   there.
4. **A check for private paths.** Add `tools/check_private_paths.py` with a unittest in
   `tools/tests/`. It fails on any file Git tracks that contains:
   - a home-directory path, such as `/Users/<name>/` or `/home/<name>/`;
   - a session scratch path under `/private/tmp/` or `/tmp/claude`;
   - `.codex/auth.json`;
   - any name in the comma-separated environment variable `CONDUIT_PRIVATE_NAMES`.

   The variable keeps the owner's names out of the public repository; the check never hard-codes
   one. Allow a listed fixture: `experiments/005-macos-keychain/src/lib.rs` uses the made-up user
   `u`. The unittest must show the check green on a clean tree, and red on a planted path and on a
   planted name from the variable. Run the check in CI's docs job, and add it to AGENTS.md's list of
   checks with one line saying what it needs.

## Scope

Change:

- `engine/.config/`;
- the test files of `engine/crates/shell/`: test code only, never the code under test;
- `.github/workflows/checks.yml`;
- `tools/check_private_paths.py` and `tools/tests/`;
- AGENTS.md's list of checks.

Do not change:

- any non-test Rust source;
- any case, vector or expectation;
- `spec/`, `conformance/`, `packages/` or a record.

Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/check_private_paths.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

In `engine/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo nextest run --release`, twice, both green;
- `cargo test --release`, once.

Report the wall time of each full run, and the totals passed and failed. Tests that bind loopback
ports may fail in your sandbox; if they do, say so, and the admin reruns them. Validate the edited
workflow's YAML by parsing it. Do not stage anything.
