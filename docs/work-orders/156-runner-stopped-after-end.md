# Work order 156: the runner reports a run stopped only once it has seen it end

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for the shell's tests that bind a loopback port; use it for nothing else, and reach no other
machine. Run one cargo command at a time, and never print a whole large file or log. End every
process that a test or a probe of yours starts. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never print a key, a token or any
credential.

## What and why

The full review of 2026-11-14 (order 13 of [its orders](../proposals/full-review/orders.md))
found that when the engine shuts down, `Runner::drop` (`engine/crates/shell/src/runner.rs:208-235`)
sends SIGKILL to each run's process group and submits `builtin-run-stopped` in the next
statement. That submission sets the instance's `Running` condition to `RunStopped`, so the runner
reports an end it has not seen
([FR-346](../proposals/full-review/findings/P07-engine-build.md#fr-346), the build half;
[FR-332](../proposals/full-review/findings/P06-dogfooding.md#fr-332), the runner half).
`engine/CONFORMANCE.md:41` says so itself: DL16, executor coverage, "it must await an actual end".
The same stop also gives a run no time to end its own work. provider-local-process sends TERM,
waits, sends KILL and confirms; the runner sends KILL at once
([FR-311](../proposals/full-review/findings/P04-providers-built.md#fr-311), its verifier's item
(d); [FR-306](../proposals/full-review/findings/P04-providers-built.md#fr-306), the grace period).
The `.v2` wrappers of the review's order 16 stop their client's process tree on TERM, so they need
that grace. The coordinator's D55 ([decisions](../proposals/full-review/decisions.md)) has the
runner record a stop only once the run's group is empty, and this must land before the dogfood
engine is re-pinned.

[Ruling 78](../proposals/ruling-78/README.md), point 6, drops the fence, so this order is not
about fencing. The core keeps the fence marks it sets on this observation until the order that
removes them, and this order changes nothing in the core. Its job is to make the report true.

## The decisions it applies

1. **TERM, then KILL, as provider-local-process's guard does.** The run guard (`guard_main`,
   `runner.rs:62-116`) ignores TERM itself and restores the ordinary TERM disposition for its
   run's program before exec, as `packages/provider-local-process/src/guard.rs:44-49` and `:79-84`
   do. The engine asks a guard to stop by writing the one byte `S` on the lifetime pipe it already
   holds (`ProcessGroup::lifetime`, `runner.rs:40-44`), the byte provider-local-process's guard
   reads. A failed write is not an error: the guard is then already gone. On `S` the guard sends
   TERM to its group. It then sends KILL to its group as soon as its run's program has exited (at
   once if it had already exited) or the pipe closes, as a container ends when its main process
   exits. The engine keeps the time, not the guard. End-of-file, a read error or any other byte
   KILLs the group at once, as now (`runner.rs:104-114`), so a lost engine still ends its runs.
2. **Absence is seen, never assumed.** A group is absent only when `libc::kill(-group, 0)` fails
   with `ESRCH`. Success or `EPERM` means a member remains: on the Mac, a group whose only member
   is an unreaped zombie answers `EPERM`, and on Linux such a probe succeeds. The guard is the
   engine's own child and keeps the group present until the runner's task that reaps it
   (`runner.rs:548-553`) has done so, so absence can be seen only after that reaping. A reused ID
   can only make a group look present, which is the safe side.
3. **The shutdown.** The work of `Runner::drop` moves into a new `pub fn shutdown(&self)`, which
   runs once: a second call does nothing. `src/main.rs:224-228` prints `diagnostics()` before it
   drops the runner, so `Drop` calls `shutdown` and writes to standard error, with the prefix
   `runner: `, each line that its own call added. A call that does nothing adds none. `shutdown`:
   - under the `cancelled_setups` lock, as `Drop` holds it now (`runner.rs:212`), sets `stopping`,
     takes the run table, and KILLs at once every group in `pending_groups` that is not the group
     of a run it took, submitting nothing for them. `pending_groups` also holds the groups of runs
     already in the run table (`runner.rs:408-413`), so compare with `Arc::ptr_eq`. It then
     releases every lock the runner holds before it starts waiting, so that neither a guard's
     reaping nor a setup still starting waits on it;
   - if the shell is gone, waits for nothing and submits nothing: the shell's own drop has
     already KILLed every group (`lib.rs:653-661`);
   - asks every run's guard to stop, all at once, and waits for all of them against one deadline,
     the grace, not one grace per run. It polls each group every 10 ms, as provider-local-process
     does, and returns as soon as every group has been seen absent;
   - when the grace has passed, KILLs each group not yet seen absent with `ProcessGroup::stop`
     (`runner.rs:46-54`), as now, and waits up to three more seconds, a constant, as
     provider-local-process does when it confirms a stop;
   - submits `builtin-run-stopped` for a run as soon as its group is seen absent, and never for a
     run whose group was not seen absent. The existing line for a submission that fails stays;
   - for each run not seen absent by then, submits nothing, so its status stays as stored. It adds
     to `diagnostics()` one fixed line that names the instance's ID and no other value:
     `the run of instance <ID> was not seen to end within 3 seconds of KILL, so its stop was not recorded`.
4. **The grace.** `Config` in `engine/crates/shell/src/lib.rs:23-45` gains
   `stop_grace: Duration`, 10 seconds by default, the same wait that `docker stop` uses. There is
   no command-line flag. The runner reads the value from the shell's config.
5. **What stays.** These keep their immediate KILL and their answers: `stop.v1`
   (`runner.rs:268-275`), setup cancellation (`:329-347`), a failed initialize and a setup
   cancelled after it (`:519-532`), a replaced run (`:539-546`), the runner's task that reaps a
   guard and stops its group (`:548-553`), the npm helper (`:685-713`), a call whose transport
   closes (`:1104-1111`) and the shell's own drop (`lib.rs:653-661`). `inspect-run.v1` does not
   change. A stop that answers only once the run has ended belongs to SO-B's stop half
   ([FR-219](../proposals/full-review/findings/X07-retries-fence.md#fr-219)), not to this order.
   The core, the vectors and `spec/` do not change. The unit is the run's process group. A process
   that leaves the group, by `setsid` or by joining another group, is neither stopped nor seen, as
   fix F61 of the [decisions](../proposals/full-review/decisions.md) asks DL16 to state.
6. **Three tests** in `engine/crates/shell/tests/runner.rs`. Each opens the shell with
   `Config { stop_grace: …, ..Config::default() }`, establishes as the existing tests do, calls
   `runner.shutdown()` while the shell is still open, then inspects each Resource as
   `runner_exit_observes_owned_runs_as_stopped` (`:325-368`) does. Tests 1 and 3 each add a mode to
   the `scripted_server` fixture (`:203`), used by both of its Resources; test 2 uses `normal`.
   - `shutdown_reports_a_run_that_traps_term_stopped_after_kill`, grace 2 seconds, mode
     `trap-term`: the fixture handles TERM by recording, in a file `term` beside its `pid`, that
     TERM reached it, and keeps serving. `shutdown` takes at least the grace; each Resource's
     `term` file exists; each fixture's PID is gone; and the `Running` condition reads `False`
     with reason `RunStopped` and a `lastTransitionTime` no earlier than the UTC time taken just
     before `shutdown` plus the grace, compared at millisecond precision.
   - `shutdown_reports_a_run_that_exits_on_term_before_the_grace`: the `normal` mode, with a grace
     of 120 seconds. `shutdown` returns in under 60 seconds, and the condition reads `RunStopped`.
   - `shutdown_submits_nothing_for_a_group_not_seen_absent`, grace 1 second, mode
     `zombie-in-group`: before it serves, the fixture spawns `/bin/sleep 0` with null standard
     streams and never waits for it, then calls `libc::setsid()`, so the fixture leaves the run's
     group and session while the exited sleep stays in the group as a zombie that only the fixture
     could reap. After `shutdown`, the condition still reads `True` with reason `RunRunning`, and
     `diagnostics()` holds decision 3's line exactly once for each Resource, naming its ID. A value
     whose drop sends KILL to each fixture's PID, read from its `pid` file, ends the fixtures, so
     that a failed assertion leaves none behind.

   The existing runner tests pass unchanged, among them
   `killed_engine_ends_run_and_descendant_and_does_not_pass_token` (engine loss),
   `dropping_runner_during_initialize_ends_group_with_shell_still_open` (a group still starting)
   and `runner_exit_observes_owned_runs_as_stopped`.
7. **`engine/CONFORMANCE.md`.** In DL16's executor coverage row (line 41), replace the sentence
   that begins "The shell submits that observation only on Runner drop" with what decisions 1 to 5
   do: the observation is still submitted only at the runner's shutdown; TERM, the grace and its
   default, KILL, three seconds to see the group absent by `ESRCH`, and otherwise nothing
   submitted and a line instead; also that a process that left the group is not seen. The row's
   evidence names the three tests. The sentence "The runner stops the run's Unix process group
   immediately with its existing SIGKILL mechanism; no graceful-stop interval is chosen."
   (lines 279-280) is amended to say that this holds for a retirement's stop, and that the
   engine's shutdown gives the grace (DL16). The row's sentence on the core's fence stays.

## Scope

Change only `engine/crates/shell/src/runner.rs`, `engine/crates/shell/src/lib.rs` (the `Config`
field), `engine/crates/shell/tests/runner.rs` and `engine/CONFORMANCE.md`.

## Done when

From `engine/`, each passes, one at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-shell --test runner shutdown_
cargo test --release
```

The third command reports exactly 3 passed, the three new tests. From the repository root, each
passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

The admin then shows the engine build, the four engine shards and the doctests passing on the
pull request, on Linux. Do not stage anything. Report each choice you make as your own, and every
check's last line.

## Record

Run on 2026-11-15 on the Mac from `main` `89aa7894`: the worker (1,263 s) and a fresh Codex
verifier (311 s), which found nothing to change in one round but did not rerun the build or the
tests. `Runner::drop` now calls a new once-only `Runner::shutdown`. It KILLs groups still starting,
releases every lock, and asks each run's guard to stop with `S` on its lifetime pipe. The guard
sends TERM to its group, then KILLs the group once the program has exited or the pipe closes. All
runs share one `Config.stop_grace` (10 s by default, no flag). Then come KILL and up to 3 more
seconds of polling every 10 ms. `builtin-run-stopped` is submitted only for a group whose probe
failed with `ESRCH`. For any other group a fixed line goes to `diagnostics()`, which `Drop` prints
with the prefix `runner: `. Three `shutdown_` tests cover a program that traps TERM, one that exits
on it, and a zombie left in the group. In `engine/CONFORMANCE.md`, DL16's executor coverage row
says all of this, and the retirement paragraph says that its immediate KILL is a retirement's.
A fresh read-only Opus reader then verified the order (ruling 79, point 2). It reran the runner
tests (14 passed), clippy and fmt, and found no must. Both of its shoulds were applied before
merge. The guard's read loop again retries an interrupted read, as it did before, rather than
KILLing its group. DL16's executor row now says that the guard KILLs its group on the program's
exit only once asked to stop. The retirement paragraph is re-wrapped to its width. A second fresh
Opus reader read those fixes and found nothing to change.
