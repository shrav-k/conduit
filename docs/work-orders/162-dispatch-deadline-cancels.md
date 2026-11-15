# Work order 162: a dispatch deadline cancels its call before the call is reported missing

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for the shell's tests that bind a loopback port; use it for nothing else, and reach no other
machine. Run one cargo command at a time, and never print a whole large file or log. End every
process that a test or a probe of yours starts. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

The full review of 2026-11-14 (order 14 of [its orders](../proposals/full-review/orders.md))
found that when a call to a run outlives the shell's call deadline, the runner reports the call
missing and never tells the server to stop
([FR-335](../proposals/full-review/findings/P06-dogfooding.md#fr-335), the engine part;
[FR-336](../proposals/full-review/findings/P06-dogfooding.md#fr-336), step 3). In
`Inner::dispatch_later` (`engine/crates/shell/src/runner.rs:1114-1170`), `tokio::time::timeout`
drops the waiting call when `Config.call_timeout` passes. Lines 1149-1151 then forget the
request's ID and its response slot, and `shell.complete` reports `{"kind":"missing"}`. The server
goes on with work whose answer no one will read, and a model turn behind it runs to its own
deadline. MCP's lifecycle asks a sender whose request has timed out to send a cancellation
notification for it and stop waiting, as a gRPC client cancels a call whose deadline has passed.
The runner already sends that notification when a caller cancels (`notify_cancel`,
`runner.rs:1104-1112`). The coordinator's D55
([decisions](../proposals/full-review/decisions.md)) has the dogfood engine's faults fixed before
the rerun, this one among them; FR-336 puts the fix before the dogfood engine is re-pinned, and
[finding 4-26](../proposals/first-build/log-increment-4.md) names this order. Order 156 changed
the same file: build on `runner.rs` as it is on `main`.

How long a call may run does not change. A deadline per turn set by the caller stays the owner's,
with finding 4-12, and no deadline is raised as the answer to a long turn (D57).
[Ruling 78](../proposals/ruling-78/README.md), point 7, has a later order change the core so that
a call that changes nothing ends failed when no answer comes. This order changes no outcome: the
shell still completes with `missing`, and the core decides what that means.

## The decisions it applies

1. **At the deadline, cancel the request still outstanding, then report the call missing.** In
   `dispatch_later`'s task, tell the deadline's passing apart from a call that ended before it.
   When `call_timeout` passes, the task:
   - takes the attempt's entry from `requests` and removes its response slot, as lines 1149-1151
     do now. The entry is the request that `call` registered (`:1187-1190`): the call itself or,
     for `refresh-tools-and-call`, the `tools/list` still waiting;
   - sends `notifications/cancelled` for that request ID through the entry's peer, and waits for
     the send to return. rmcp 3.9.0 returns it once `Wire::send` (`:1052-1080`) has written and
     flushed the line to the run's standard input;
   - only then calls `shell.complete` with `{"kind":"missing"}`, as now.

   If the deadline passes before `send_cancellable_request` (`:1182-1185`) has returned, there
   is no entry: the request was never queued for the run, and nothing is cancelled. The
   cancellation is sent even when a caller's cancellation was already sent for the attempt
   (`cancel`, `:421-431`; `call`, `:1191-1193`), because the protocol lets a receiver ignore a
   cancellation for a request it no longer has. A call that ended before the deadline keeps lines
   1149-1151 as they are. `call` does not change.
2. **The notification names its cause.** `notify_cancel` gains an optional reason. The deadline's
   cancellation carries `params.reason` `call deadline exceeded` beside `params.requestId`. A
   caller's cancellation still carries no reason, so what it puts on the wire does not change.
3. **The send is bounded, and a send not seen is reported.** The wait for the send is at most 5
   seconds, a constant, as the npm helper waits at most 5 seconds for its exit status
   (`:813-814`). If the send fails or the 5 seconds pass, the task still completes with
   `missing`. It also pushes one fixed line onto `errors`, which `Runner::diagnostics`
   (`:239-243`) returns. Each line names the invocation's ID and no other value (ruling 52). For
   a send that failed:
   `the call of invocation <ID> passed its deadline and its cancellation could not be sent`.
   For a send not done within the 5 seconds:
   `the call of invocation <ID> passed its deadline and its cancellation was not seen sent within 5 seconds`.
   The second does not say the cancellation was never sent: once the wait ends, rmcp may still
   write it. (The coordinator amended this decision after the order ran; see the Record.)
4. **What stays.**
   - A call that ends before its deadline sends no cancellation, whether it was answered or
     failed (a closed transport, a malformed reply, a refresh answered with an error). Its
     completion does not change. A target with no run still answers `Execution::Missing` at once
     (`:1122-1124`).
   - At the deadline the completion stays `missing`, with the original invocation and attempt.
     Nothing is sent again, and an answer that arrives after the deadline is not read.
   - These do not change: `call_timeout` (`engine/crates/shell/src/lib.rs:31`, 120 seconds by
     default), `--call-timeout-seconds`, the session idle limit derived from it and
     `initialize_timeout`. The 5 seconds fall inside the 60 seconds by which the idle limit
     outlasts the deadline (`engine/CONFORMANCE.md:36`). The deadline stays one per intent. No
     input for a per-call deadline is added. Relaying the protocol's progress notifications,
     which FR-335's verifier raised, is not part of this order.
   - A caller's cancellation, setup cancellation and order 156's stops do not change. The core,
     the vectors, `spec/` and `lib.rs` do not change.
5. **One test** in `engine/crates/shell/tests/runner.rs`,
   `dispatch_deadline_cancels_the_held_request_before_the_next_call` (renamed after the order
   ran; see the Record). It loops over two modes of
   the `scripted_server` fixture (`:209-326`), as `deadlines_exit_repair_write_and_cancel`
   (`:772-871`) does. For each mode it opens the shell with
   `Config { call_timeout: Duration::from_secs(1), ..Config::default() }`, calls
   `register(&shell, parent.path(), false, mode)` and sets up `workspace`, `shared` and `agent-b`
   as that test does (`:796-809`). It reads the wire, `shared/wire.jsonl`, with `lines`
   (`:182-188`).
   - `hang`, the existing mode that holds every `tools/call` (`:296-299`): `agent-b` invokes
     `project.file.write.v1` with `{"path":"x","content":"y"}`. The held request is its
     `tools/call`.
   - `repair-list-hang`, a new mode: the fixture leaves every `tools/list` unanswered, in an arm
     beside the `repair-list-error` arm (`:280-287`). As in every mode whose name begins
     `repair`, its first `tools/call` is answered -32020 (`:300`). `agent-b` invokes
     `project.file.read.v1` with `{"path":"x"}`. The core then sends a refresh, and the held
     request is its `tools/list`.

   In each mode the test waits for the reply and asserts that its outcome is `unknown`, which is
   what the core gives a missing completion today. It then begins a second invocation by
   `agent-b`, `project.file.read.v1` with `{"path":"y"}`, waits until `call_count` (`:202-207`)
   is 2, and asserts on the wire:
   - the first `notifications/cancelled` line has `params.requestId` equal to the held request's
     `id`, and `params.reason` equal to `call deadline exceeded`;
   - that line comes before the second `tools/call`. The second invocation begins only after the
     reply has returned, and the run reads its standard input from one pipe, in order. So this
     shows that the cancellation was sent, and before the next call on the run. That it was
     sent before the call was reported missing rests on the code order in `runner.rs`;
   - `tools/call` appears twice, and `tools/list` appears once in `repair-list-hang` and never in
     `hang`, so nothing was sent again.

   It then waits for the second reply, so that no call is left outstanding. The existing tests
   pass unchanged. The `hang` mode of `deadlines_exit_repair_write_and_cancel` now also receives
   a cancellation, which that test does not check.
6. **`engine/CONFORMANCE.md`.** In the row "DL10: process and call deadlines" (line 35), replace
   the clause "dispatch timeout reports missing with the original invocation and attempt and
   never resends a write" with what decisions 1 to 4 do:
   - at a dispatch's deadline the shell sends `notifications/cancelled`, with the reason
     `call deadline exceeded`, for the request still outstanding (the call, or a refresh's
     `tools/list`);
   - it waits at most 5 seconds for the send, then reports missing with the original invocation
     and attempt;
   - it never resends a write and does not read a later answer;
   - a cancellation whose send failed, or was not seen done within 5 seconds, leaves one of
     decision 3's two fixed diagnostics, each naming only the invocation.

   The row's evidence names the new test. Nothing else in the file changes.

## Scope

Change only `engine/crates/shell/src/runner.rs`, `engine/crates/shell/tests/runner.rs` and
`engine/CONFORMANCE.md`.

## Done when

From `engine/`, each passes, one at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-shell --test runner dispatch_deadline_
cargo test --release
```

The third command reports exactly 1 passed. Then show that the test fails with the fix removed.
From `engine/`, run these one at a time:

```bash
cp crates/shell/src/runner.rs target/runner.rs.fixed
git show HEAD:engine/crates/shell/src/runner.rs > crates/shell/src/runner.rs
cargo test --release -p conduit-shell --test runner dispatch_deadline_
cp target/runner.rs.fixed crates/shell/src/runner.rs
cargo test --release -p conduit-shell --test runner dispatch_deadline_
```

Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not
rebuild the crate and would run `main`'s code again. Git ignores `target/`. With `main`'s
`runner.rs`, the test must fail because the wire holds no `notifications/cancelled`. Report the
failing assertion's line. After the restore, the test must pass again, and `git status --short`
must list only the three files in scope. Then, from the repository root, each of these passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

The admin then shows the engine build, the four engine shards and the doctests passing on the
pull request, on Linux. Do not stage anything. Report each choice you make as your own, and every
check's last line.

## Record

Run on 2026-11-15 on the Mac from `main` `f37d76c0`: the worker (1,043 s) and a fresh Codex verifier
(189 s), which found nothing to change in one round but did not rerun the build or the tests. At a
dispatch's deadline, `Inner::dispatch_later` now takes the request still outstanding from
`requests`, sends `notifications/cancelled` for it through its peer with the reason `call deadline
exceeded`, and waits at most 5 seconds (`DEADLINE_CANCEL_TIMEOUT`) for the send before it completes
the call `missing`. A call that ends before its deadline sends nothing. `notify_cancel` gained an
optional reason, which a caller's cancellation leaves out, so what it puts on the wire is unchanged.
A send not seen left one fixed line in `diagnostics()`. The fixture gained a `repair-list-hang`
mode, and one test covered both the held `tools/call` and a refresh's held `tools/list`. DL10's row
in `engine/CONFORMANCE.md` says what the deadline now does. With `main`'s `runner.rs` the test
failed at `tests/runner.rs:827` (`:839` once the rename's comment was added; the second test then
fails at `:900`): the wire held no cancellation.

A fresh read-only Opus reader then verified the order (ruling 79, point 2). It ran the runner tests
twice (15 passed, 2 ignored), clippy and fmt, and the check with the fix removed, which failed at
the new test. It found no must. Its four shoulds were applied before merge:

- The test showed less than its name claimed: the reader's mutant, which sent the cancellation only
after the missing report, still passed it. The test is renamed
`dispatch_deadline_cancels_the_held_request_before_the_next_call`. Its comment, decision 5 and
DL10's evidence now say that it shows the cancellation sent, and before the next call on the run.
That it comes before the missing report rests on the code order at `runner.rs:1151-1187`, where
`dispatch_later` awaits the bounded send before it calls `shell.complete`. - No test reached the
diagnostic. A new fixture mode, `hang-closed-stdin`, records the held `tools/call`, closes its
standard input and sleeps without exiting. In
`dispatch_deadline_reports_a_cancellation_that_could_not_be_sent`, the deadline's write fails, the
outcome is `unknown`, and `diagnostics()` holds exactly the could-not-be-sent line below. With that
line's push removed, the test fails at `tests/runner.rs:900`. - At the coordinator's word, outside
the order's scope, `engine/.config/nextest.toml` lists both tests, and order 156's three `shutdown_`
tests, in the `shell-sensitive` group's `binary(=runner)` list. - One line for both cases made a
send that failed at once read as a timeout (ruling 52). The coordinator amended decision 3: a send
that failed leaves `the call of invocation <ID> passed its deadline and its cancellation could not
be sent`, and a send not done within 5 seconds keeps the original line. Each names only the
invocation. DL10's row and decision 6's last bullet say the same.

After these fixes the runner tests passed twice (16 passed, 2 ignored), and clippy, fmt and the
documentation checks were clean. The focused command in "Done when" now runs two tests.

A second fresh Opus reader read the coordinator's fixes (ruling 79, point 2): it reran the runner
tests twice (16 passed, 2 ignored), clippy and fmt, and found no must; its one wording point, on the
line numbers above, was applied.
