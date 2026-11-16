# Work order 194: the restart tests wait for the second start

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
because the package's tests bind loopback ports; use it for nothing else, and reach no other
machine. Run one cargo command at a time, and never print a whole large file or log. End every
process that a test of yours starts. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.

## What and why

Two tests of `packages/provider-local-process` count the lines of the fixture's file `starts`
before the fixture they have just started again has written to it. This order makes them wait for
the expected count, then check it exactly.

The macOS job for this package, which [order 187](187-ci-macos-leg.md) added, failed on
`killed_provider_fences_descendants_and_restart_keeps_state` in 3 of the 9 recent runs that ran
it, as [order 189's Record](189-package-tests-repeated-linux.md#record) says. One of them was
order 189's hosted check, on a scratch pull request that carried order 189's branch and one
comment line in `packages/runtime-provider-kit/tests/runtime.rs`, a file this package does not
build. There the test panicked at `tests/runtime.rs:372:5`, the assertion that `starts` holds
two lines. In the same run this package's tests passed all 5 of order 189's repeated runs on
Linux.

The cause is a race in the test, not in the provider:

- The fixture, `tests/fixtures/run.rs`, appends a line to the file `starts` in its state
  directory when it starts, then spawns its child, then writes `pids.json`.
- The provider answers `run.v1` with `succeeded` without waiting for any of that. Its guard
  (`src/guard.rs`) reports ready as soon as it has spawned the program; the backend's `start`
  (`src/backend.rs`) returns on that report; and the kit
  (`packages/runtime-provider-kit/src/server.rs`) then opens the gate and answers. Nothing is
  exchanged with the program first.
- The test's first start waits for the fixture through `ready`, which polls for `pids.json`. Its
  second start, after the restart, does not wait: it reads `starts` at once and asserts two
  lines. `ready` cannot be reused there, because the first start's `pids.json` stays in the
  state directory, which the test keeps on purpose. When the read comes first, it finds one line
  and the assertion fails.

The same pattern is in `killed_guard_cannot_confirm_stop_or_admit_a_duplicate_after_restart`: a
`run.v1` that starts the fixture again after the stop is confirmed, then an immediate read that
expects two lines. On `main` at `d4c8cf8b` the two assertions begin at lines 372 and 480 of
`tests/runtime.rs`. Find each site by reading the file; do not trust these line numbers if `main`
has moved.

The coordinator checked this reading on a scratch copy of `main`. With a 500 ms sleep added at the
fixture's start, those two tests failed, at lines 372 and 480, and the other six tests of
`tests/runtime.rs` passed. With decision 1's and decision 2's changes as well, all eight passed.
The full review's D12 ([decisions](../proposals/full-review/decisions.md)) has every fault that
reaches `main` get a test named in the fixing order's Record, and a lock handoff tested
deterministically, "never repetition alone". So the done-when shows the race and its fix with that
scratch delay, not with a loop of runs. Under [ruling 80](../proposals/ruling-80/README.md), point
10, no merge waits for CI, and a run that fails after a merge is fixed in a follow-up; this order
is that follow-up for the macOS leg.

## The decisions it applies

1. **Wait for the expected count, then check it exactly.** Add this helper to `tests/runtime.rs`,
   right after `ready`:

   ```rust
   /// Waits until the fixture has recorded `expected` starts, then returns the count read.
   async fn starts(root: &Path, id: &str, expected: usize) -> usize {
       let path = state(root, id).join("starts");
       let read = || {
           fs::read_to_string(&path)
               .map(|s| s.lines().count())
               .unwrap_or(0)
       };
       let _ = timeout(WAIT, async {
           while read() < expected {
               sleep(Duration::from_millis(10)).await;
           }
       })
       .await;
       read()
   }
   ```

   At each of the two sites above, where a `run.v1` answered `succeeded` starts the fixture again
   and is followed by a count of `starts` that expects 2, replace the `assert!` and its immediate
   read with:

   ```rust
       assert_eq!(
           starts(root.path(), ID, 2).await,
           2,
           "the second run.v1 succeeded, so the fixture must have started twice"
       );
   ```

   The message says what was expected, and `assert_eq!` shows the count read, so that a failure
   explains itself ([ruling 52](../proposals/ruling-52/README.md), point 2). The timeout ends the
   wait, not the test: the exact `assert_eq!` decides, so one start after the wait fails, and so
   does a third start.

2. **Keep the reads that assert no new start.** Two sites assert `== 1` after a `run.v1` that must
   not start the fixture again. In `real_run_gate_repeat_inspect_environment_and_group_stop` it
   follows the repeated `run.v1` that returns the same URL. In
   `killed_guard_cannot_confirm_stop_or_admit_a_duplicate_after_restart` it follows the `run.v1`
   answered `unknown` after the restart. Polling cannot prove that no start will come, so keep
   each immediate read, `fs::read_to_string(...).unwrap().lines().count()`, as it is. Write each
   site as `assert_eq!(<that read>, 1, "<message>")`, so that a failure shows the count, with these
   messages, in this order:
   - `"the repeated run.v1 returned the same URL, so the fixture must not have started again"`
   - `"run.v1 was answered unknown, so the fixture must not have started again"`

   The check in `declaration_refusals_and_path_safe_instance` that `starts` does not exist is not
   a count. Leave it unchanged.

3. **Nothing else changes.** The provider's code, the fixture and every other test stay as they
   are. The done-when's scratch edit to the fixture is reverted before you finish. If you find
   that the fixture or the provider is at fault rather than the test, stop and report it with the
   evidence; do not change them.

## Scope

- `packages/provider-local-process/tests/runtime.rs`.

Nothing else: no change to the package's `src/`, to `tests/fixtures/run.rs` (apart from the
done-when's scratch edit, which is reverted), to its `Cargo.toml` or `Cargo.lock`, or to
`packages/runtime-provider-kit/`. Report anything else that seems to need changing.

## Done when

Run this order on the Mac: the race showed on macOS runners. Work in
`packages/provider-local-process` unless a step says otherwise, and run each command alone.

First, before you change `tests/runtime.rs`, show the race. Make this scratch edit to
`tests/fixtures/run.rs`: right after the line `let state = Path::new(&state);`, add

```rust
    std::thread::sleep(std::time::Duration::from_millis(500));
```

so that the fixture writes its line in `starts` late. Then run:

```sh
cargo test --release --test runtime 2>&1 | grep -E '^test |panicked at|^test result'
```

It reports 6 passed and 2 failed. The two failed tests are
`killed_provider_fences_descendants_and_restart_keeps_state` and
`killed_guard_cannot_confirm_stop_or_admit_a_duplicate_after_restart`, and each panics at its
assertion that `starts` holds two lines (lines 372 and 480 on `main` at `d4c8cf8b`). Report the
`panicked at` lines. If any other test fails, or either of the two passes, stop and report.

Then make decisions 1 and 2's changes, keep the scratch edit, and run the same command. It reports
8 passed and 0 failed.

Then revert the scratch edit, and run these one at a time. Each passes:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
```

In the last, the `runtime` test binary reports 8 passed, as on `main`.

Each of these prints the count after its `#`:

```sh
grep -c 'async fn starts(root: &Path, id: &str, expected: usize) -> usize' tests/runtime.rs  # 1
grep -c 'starts(root.path(), ID, 2).await' tests/runtime.rs      # 2
grep -c 'so the fixture must have started twice' tests/runtime.rs  # 2
grep -c 'so the fixture must not have started again' tests/runtime.rs  # 2
grep -c '== 2$' tests/runtime.rs                                   # 0
grep -c 'join("starts")' tests/runtime.rs                          # 4
```

`pgrep -f 'release/test-ru[n]'` prints nothing: no fixture process is left. (The brackets keep
the pattern from matching the shell that runs it.)

From the repository root, each of these passes:

```sh
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

`git status --short` lists only `packages/provider-local-process/tests/runtime.rs`, and
`git diff -- packages/provider-local-process/tests/fixtures/run.rs` prints nothing. Do not stage
anything. Report each choice you made yourself, and every check's last line.

Before the merge, the coordinator's local gate runs (ruling 80, point 10), and a fresh Opus reader
reads the result ([ruling 79](../proposals/ruling-79/README.md)). After the merge, the Record names
the two tests, as D12 asks.

## Record

Run on 2026-11-16, from about 20:59 to 21:07 UTC, on the Mac from `main` `a771b88b`: the worker
(368 s) and a fresh Codex verifier (113 s), which found no must or should in one round but did not
rerun cargo or the scratch delay in its read-only sandbox. Only
`packages/provider-local-process/tests/runtime.rs` changed. A new helper, `starts`, right after
`ready`, polls the fixture's `starts` file every 10 ms until it holds the expected count of lines
or `WAIT` (8 s) runs out, then returns the count it reads. The two tests that start the fixture
again after a restart, `killed_provider_fences_descendants_and_restart_keeps_state` and
`killed_guard_cannot_confirm_stop_or_admit_a_duplicate_after_restart`, now wait through it and
check with `assert_eq!` that the count is exactly 2, with the message "the second run.v1
succeeded, so the fixture must have started twice". These are the tests D12 asks this Record to
name: on macOS their read could come before the second start had written its line. The two reads
that assert no new start, in `real_run_gate_repeat_inspect_environment_and_group_stop` and after
the `run.v1` answered `unknown` in the second test above, stay immediate and are now `assert_eq!`
with the order's messages. The provider, the fixture and the other tests are unchanged. With the
order's 500 ms scratch delay in the fixture, `main`'s file failed those two tests at lines 372 and
480 (6 passed, 2 failed), and with the change all eight passed; the delay was then reverted. The
worker could not run `pgrep`, which its sandbox denied.

A fresh read-only Opus reader then verified the order (ruling 79, point 2). It ran
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` (again in a fresh target
directory) and `cargo test --release` (the `runtime` binary 8 passed, `package.rs` 3), the six grep
counts (1, 2, 2, 2, 0 and 4), `pgrep -f 'release/test-ru[n]'` (nothing), `check_docs` and
`check_private_paths` (Errors: 0 each) and `git diff --check origin/main...HEAD` (clean). On a
scratch copy of `main` at `ecabe372` with the branch's file, `main`'s private-path check, with
order 196's email pattern, and `check_docs` also found no error. On that copy it repeated the
order's check: with the delay, `main`'s file failed at lines 372 and 480 and the change passed all
eight. It made the second start wait 10 s, longer than `WAIT`, and both restart tests failed at
lines 389 and 496, left 1 and right 2. It made the fixture write its line twice, and the two kept
reads failed at lines 262 and 464 with a count of 2, and the wait at line 389 with 4. So every
assertion the change touches can fail, and each failure explains itself (ruling 52, point 2). It
found no must and no should. Its minor notes are left as they are: the helper's doc comment, which
the order prescribed word for word, does not say that the wait also ends after `WAIT`;
`unwrap_or(0)` would show an unreadable file as a count of 0, which is unlikely once `ready` has
confirmed the first start; and the worker's last line for `cargo test --release` is the doc tests'
line, not the `runtime` binary's.

With no finding, the fixer changed no code. It rebased the branch onto `main` at `ecabe372`
(orders 178, 179 and 196) without conflict; none of them touches `provider-local-process` or
`runtime-provider-kit`. In `packages/provider-local-process`, under `env -i`, it then ran the
order's "Done when" on the rebased branch, one cargo command at a time. With the scratch delay,
`main`'s `tests/runtime.rs` again failed at lines 372 and 480 (6 passed, 2 failed) and the branch's
passed all eight. With the delay reverted, `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings` passed, `cargo test --release` passed (`runtime` 8
passed, `package.rs` 3), the six grep counts were 1, 2, 2, 2, 0 and 4, and
`pgrep -f 'release/test-ru[n]'` printed nothing after each run. From the repository root,
`check_docs` and `check_private_paths` reported Errors: 0, and `git diff --check origin/main...HEAD`
was clean. The order's "Done when" has no loop of runs: under D12 it shows the race and its fix
with the scratch delay, so no repeated run was made.
