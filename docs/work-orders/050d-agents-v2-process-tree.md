# Work order 050d (P2d): the `.v2` wrappers stop their client's whole process tree (the full review's order 16)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only for tests that
bind a loopback port; the one crate it adds is already in each package's `Cargo.lock`, so cargo
should fetch nothing (decision 1 checks it offline), and you use the network for nothing else.
Run one cargo command at a time. Never print a whole large file: use `head`, `tail`, `grep`,
`sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Never read or print any process's environment or
arguments, never run a command that lists every process on the machine, such as `ps -e`, `ps aux`
or `top` (a `ps` in this order names its PIDs with `-p`), and never signal a process that a test
of this order did not start. The wrapper's table read of decision 3, which keeps only the fields it
names, is what this order builds, and its tests run it. Never run the real
`codex` or `claude` programs, not even for their help or version, and sign in nowhere. The tests
use each package's scripted client, in temporary directories only.

## What and why

**Where this order starts.** Your tree is the branch that the coordinator names as `BASE`, not
`main`: `wo/050-agents-0.2.0`, which holds orders 050a, 076 and 078, 050a's Record and order 050c
with its Record, rebased onto `main`. `BASE`'s tip holds this order file, committed: the
coordinator rebases the branch onto `main` after this order's text merges, and sets `BASE` to the
branch's tip. Before you change anything, check from the repository root that you find there what
that gives, and stop, change nothing and report if any check prints otherwise:

```bash
git merge-base --is-ancestor 408d5f17 "$BASE"; echo "main $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/050d-agents-v2-process-tree.md >/dev/null; echo "order $?"
git status --short | wc -l
grep -c '^## Record' docs/work-orders/050a-agents-v2-own-tools.md docs/work-orders/050c-agents-v2-client-home.md
grep -c '^pub fn client_home' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs
grep -cE 'process_group|setsid|killpg|SIGTERM|libc' packages/agent-codex/src/lib.rs packages/agent-codex/src/main.rs packages/agent-claude-code/src/lib.rs packages/agent-claude-code/src/main.rs
grep -c '^libc' packages/agent-codex/Cargo.toml packages/agent-claude-code/Cargo.toml
grep -A1 '^name = "libc"$' packages/agent-codex/Cargo.lock packages/agent-claude-code/Cargo.lock | grep -c 'version = "0.2.191"'
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'
diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs | grep -c '^[<>]'
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs
git diff --quiet b9515838 "$BASE" -- packages/agent-codex packages/agent-claude-code; echo "packages $?"
```

They print `main 0`, `head 0`, `order 0` and `0`; then `1` for each order file (the two Records);
`1` for each `lib.rs` (050c's `client_home`); `0` for each of the four sources (no group, signal or
`libc` yet); `0` for each `Cargo.toml`; `2` (`libc` 0.2.191 is locked in both packages, through
`tokio`); `2` (the two `lib.rs` differ in line 17 alone, the `CLAUDE` constant); `0` (the two
`main.rs` are one file); `13` for each `wrapper.rs`; and `packages 0`. The rebase changed no
package file, since no commit on `main` after the cut touches the two packages, so the line numbers
below, those of `b9515838`, hold in your tree. The last check, `packages 0`, is what checks that
claim.

**What it builds.** A `.v2` wrapper ends a failed, cancelled or timed-out turn by killing the
client's process alone (`Session::shutdown`, `src/lib.rs:723-730`: `s.child.kill()` then
`s.child.wait()`), and it has no handler for TERM, so the engine runner's stop ends the wrapper and
leaves the client's children to the run's process group. In the dogfood smoke test, Codex's
terminal children outlived that group's kill ([FR-332](../proposals/full-review/findings/P06-dogfooding.md#fr-332),
draft row 4-23). Then a turn can be recorded failed, and its repeat dispatched, while processes of
the first turn still write in the folder. This order makes every `.v2` wrapper own its client's
whole process tree. The one behavior, in both packages: every `.v2` stop of the client (a failed
turn, a deadline, a cancellation, the wrapper's TERM and the end of its input) ends every process
of that tree that the wrapper can find, and the answer that reports the stop comes only once each
of them is seen ended or is named as having outlived it. The `.v1` launch and stop do not change.

It carries [the full review's order 16](../proposals/full-review/orders.md): FR-332's wrapper half
as its verifier corrected it, that is, the client's descendants found by a walk of parent links,
with no process's environment read (the marker variable of FR-332's least fix would read the
environment of every process of the user, against the standing rule); the fourth change of
[FR-336](../proposals/full-review/findings/P06-dogfooding.md#fr-336)'s step 2, "the wrapper owning
its client's process tree"; and [FR-337](../proposals/full-review/findings/P06-dogfooding.md#fr-337)'s
probe P3, with its verifier's TERM added, as a test: a background job, a new session and a double
fork, ended by deadline, by cancellation and by TERM, leave no survivor, or the result names each
one. The other changes of FR-336's step 2 are not here: the failure as fields is order 050e's (the
full review's order 17). FR-332's runner half is order 156, merged; its `spec/` sentence on
OPS-014 is on the owner's review list, not here.

From the targeted review of the 0.2.0 sign-in surface it carries M64 as frozen ("order 16's stop of
the client's whole process tree. Freeze as is"), and the start of item 7 of
[`claude-code.v2`](../proposals/review-signin-2026-11-16/README.md#050-claude-codev2) and
[`codex.v2`](../proposals/review-signin-2026-11-16/README.md#050-codexv2) (M65, V18): "the
survivors of order 16's stop are named where #17's fields live and, on TERM, in the wrapper's own
log". Here they are named in the failed turn's text, the place a failure lives until 050e, and on
TERM on the wrapper's stderr; 050e moves the first into its fields and leaves the second. The
coordinator reads M65's "the wrapper's own log" as the wrapper's stderr, open to the owner's
strike: the wrapper keeps no log, and under the engine's own runner, which discards the wrapper's
stderr, nothing keeps that line. The coordinator puts that reading on
[the owner's review list](../proposals/first-build/to-review-with-the-owner.md) with this order's
Record.

**What 172b changed, and why the client gets a group of its own.** Order
[172b](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/172b-so-b-reconcile-records-stop-ends.md) (its Record; Phase 0's flag F2 of the targeted
review says it bears on order 16) narrowed what a stop must end. PRV-030
(`spec/promises/providers.yaml:987-1013`): "A run's processes are every process that the provider
started for the ID and every process that those started. The provider stops a run by a unit that
the operating system ends whole, such as a process group ... Its contract names that unit and which
processes of a run can leave it, which a stop that succeeded may leave running." PRV-008
(`:294-297`): "A stop succeeds only once no process of a run of the ID still runs in the unit by
which the provider stops it". For the engine's own runner the unit is the run's process group,
led by its guard, and DL16 (`engine/CONFORMANCE.md`, rows "DL16: run state and restart" and
"DL16: executor coverage") says "A process that left the group is neither stopped nor seen", and
that at shutdown the guard sends TERM, waits for the program's exit or `stop_grace` (10 seconds by
default), then KILLs the group and submits `builtin-run-stopped` only once it has seen the group
absent. So the runner's stop does not need the wrapper, but it does not reach a process that left
its group, and Codex's terminal children are such processes.

The wrapper cannot end its own group without ending itself, so it cannot stop a double-forked
process that stays in the run's group at a turn's deadline or cancellation, when the run goes on.
The coordinator therefore takes FR-332's least fix as written: each `.v2` launch starts the client
as the leader of a new process group (decision 2), which the wrapper can end whole. That group is
outside the runner's group, a process that left the runner's unit under PRV-030, so the README
names it (decision 9): the runner's TERM reaches only the wrapper, which then ends the client's
tree before it exits, and the guard's KILL follows the wrapper's exit, so the runner's stop is
recorded after the client's tree ended or was named. Where the wrapper is killed without warning
(the engine's loss, which closes the guard's lifetime pipe, or the runner's KILL at the end of a
`stop_grace` shorter than the wrapper's stop), the client's tree is neither stopped nor seen. Today
the same KILL ends the client and leaves only the children that left the group. That trade is the
coordinator's, open to the owner's strike (see "Who decided what").

**What the branch already does, so that you do not redo it.** In each package's `src/lib.rs` (the
two files are one source; line 17's `CLAUDE` selects the client) and `src/main.rs` (one file):

- `Stream::spawn` (`lib.rs:534-556`) launches the client with `kill_on_drop(true)`, stdin and
  stdout piped and stderr discarded, in the wrapper's own process group, after `client_home` for
  `.v2` (order 050c). A spawn error fails the turn with `client did not start`.
- `Session::turn` (`:575-605`) wraps `run_turn` in the deadline; on any error it records the cause
  in `failed`, calls `shutdown` and fails with
  `client turn K failed: CAUSE; session cannot restart implicitly`. Later turns are refused with
  `session failed at turn K: CAUSE; implicit restart refused` before any launch.
- `Session::cancel` (`:715-722`) records `cancelled`, calls `shutdown` and returns
  `client turn K failed: cancelled; session cannot restart implicitly`. `main.rs:44-50` calls it
  when the call's cancellation token fires during a turn, dropping the turn's future first.
- `Session::shutdown` (`:723-730`) closes the client's stdin, kills the client and reaps it.
  `main.rs:86-91` serves the wrapper over stdio and calls `shutdown` once its input ends; `main`
  (`:95-101`) prints `Agent wrapper failed` on stderr when `run` fails.
- A Codex turn launches a new process, a resumed one after the first, and a successful Codex turn
  waits for that process to exit; a Claude Code conversation is one process, launched at its first
  turn.
- The scripted client (`tests/fixtures/client.rs`) writes `client-started` in its working
  directory when it starts; `BLOCK` writes `blocked` and waits for a file `release`.

## The decisions it applies

1. **The crate and the constants.** In each package's `Cargo.toml`, add `libc = "0.2"` to
   `[dependencies]`, after `anyhow`. The only change cargo makes to `Cargo.lock` is `"libc"` in the
   package's own dependency list: 0.2.191 is locked already. After the `Cargo.toml` edit, your
   first cargo command in each package is `cargo metadata --offline --format-version 1 >/dev/null`;
   a failure there is reported, not worked around. Add to `src/lib.rs`, after
   `CREDENTIAL_STORE`:

   ```rust
   /// A .v2 stop sends TERM, then KILL after this grace (the full review's order 16).
   pub const STOP_GRACE: Duration = Duration::from_secs(2);
   /// How long a .v2 stop waits after KILL before it names what outlived it.
   pub const KILL_WAIT: Duration = Duration::from_secs(1);
   /// The note of a stop that could not read the process table.
   pub const TREE_UNCONFIRMED: &str =
       "client tree: the process table could not be read, so the stop is unconfirmed";
   ```

   and a private `POLL` of 20 milliseconds. The two durations together stay well under the engine
   runner's default `stop_grace`, so that on TERM the wrapper ends its stop before the runner's
   KILL.

2. **Each `.v2` launch leads a new process group.** In `Stream::spawn`, for `.v2` alone, call
   `process_group(0)` on the command (Unix), so that the client's process group ID is its PID. Pass
   `Stream::spawn` the session's `ClientTree` (decision 3), and, under the tree's lock, refuse the
   launch with `client did not start` where the tree is closed; otherwise record the new group's
   ID, the client's PID, with its leader's start time, read for that one PID right after the spawn
   and under the lock (one entry through decision 3's `process`, not a table sample), before the
   lock is released. Where that one read returns `Ok(None)` or `Err`, record the group with no
   start time.
   A `.v1` launch is unchanged: no new group, nothing recorded, the same command as at `BASE`.

3. **The client's tree.** Add `#[derive(Clone, Default)] pub struct ClientTree`, an
   `Arc<std::sync::Mutex<...>>` holding the recorded group IDs, each with its leader's start time,
   the processes seen, each by PID and start time, and `closed`. `Session` gains the field
   `tree: ClientTree`, made in `Session::new`, and `pub fn tree(&self) -> ClientTree`, which
   returns a clone. The lock is never held across an `.await`.

   The table read is a private `fn processes() -> std::io::Result<Vec<Process>>`, with each
   `Process` holding a PID, a parent PID, a process group ID, a start time (`u64`) and whether it
   is a zombie, and nothing else, and the same read for one PID is a private
   `fn process(pid: i32) -> std::io::Result<Option<Process>>`:
   - The parse of a stat line is a platform-neutral private
     `fn parse_stat(pid: i32, text: &str) -> Option<Process>`, compiled on every system: after the
     line's last `)`, the fields state, parent PID, process group and start time (the 1st, 2nd,
     3rd and 20th there; fields 3, 4, 5 and 22 of `proc(5)`), a zombie being state `Z` or `X`. A
     unit test in `src/lib.rs` (`#[cfg(test)] mod tests`, one `#[test]`) parses fixed stat lines:
     a name holding `) `, a zombie, and field 22. Where it is unused outside tests (any system but
     Linux), it carries `#[cfg_attr(not(target_os = "linux"), allow(dead_code))]`, so that clippy
     passes there.
   - On Linux, under `cfg(target_os = "linux")` alone, the entries of `/proc` whose names are all
     digits, and for each the file `/proc/PID/stat` alone, through `parse_stat`. An entry that
     cannot be read or parsed is skipped: its process ended.
   - On macOS, `libc::proc_listallpids` and, for each PID,
     `libc::proc_pidinfo(pid, libc::PROC_PIDTBSDINFO, 1, ...)` (an arg of 1 also returns a
     zombie, with `pbi_status` `SZOMB`) into a `libc::proc_bsdinfo`: `pbi_ppid`, `pbi_pgid`, a
     start time of `pbi_start_tvsec * 1_000_000 + pbi_start_tvusec`, and a zombie where
     `pbi_status` is `libc::SZOMB`. A PID whose call fails is skipped.
   - Elsewhere, `Err` of kind `Unsupported`.

   `process(pid)` returns `Ok(None)` where no such PID is readable, and `Err` only where the table
   cannot be read at all (`Unsupported`).

   It reads no other file under `/proc/PID`, no `environ` or `cmdline` and no `KERN_PROCARGS2`,
   and runs no program. The name that the stat line and `proc_bsdinfo` carry is skipped: it is
   never stored, compared or printed. That is the correction to FR-332: what the wrapper knows of
   a process is its place in the tree and its start time, never its environment or arguments.

   **What a stop finds**, from one table read. Each read first forgets every recorded group G for
   which the read shows a process with PID G, live or zombie, whose start time differs from the
   recorded one, and every recorded group with no live member: a group's ID is kept from reuse
   only while the group has a member. A group with no start time is forgotten only when it has no
   live member. Only then does it find, from the groups still recorded: the
   live processes (not zombies) whose process group is a recorded group with a start time, and the
   live processes seen before whose PID and start time still match; then, repeated until it adds
   none, every live process whose parent PID is that of one already found. Every process found is
   added to those seen, and a seen process that the read does not show live is dropped. The
   wrapper's own PID is never found, and the wrapper never signals its own process group.

4. **The stop.** Add `pub async fn stop(&self, close: bool) -> Option<String>` to `ClientTree`,
   which does, in this order:
   - With `close`, set `closed`, under the lock, before the first read, so that no launch starts
     after it.
   - With no recorded group and no process seen, return `None` without reading.
   - A group with no start time is never signalled as a group, and its members are found only
     through the parent links of the other groups found; the stop then appends `TREE_UNCONFIRMED`
     where such a group still has a live member (at the read that ends the stop: after the
     survivor note and `; `, or alone where there is none).
   - Read and find (decision 3). Nothing found: return `None`, or `TREE_UNCONFIRMED` where a
     recorded group with no start time still has a live member.
   - Send TERM to each recorded group with a start time (`libc::kill(-group, SIGTERM)`) and to
     each process found outside them (`libc::kill(pid, SIGTERM)`).
   - Every `POLL`, until `STOP_GRACE` has passed since the TERM: read and find; nothing found,
     return `None`, or `TREE_UNCONFIRMED` where a recorded group with no start time still has a
     live member; send TERM to each process found that has not had it (by PID and start time).
   - Then send KILL to each recorded group with a start time and each process found, and every
     `POLL`, until `KILL_WAIT` has passed: read and find; nothing found, return `None`, or
     `TREE_UNCONFIRMED` where a recorded group with no start time still has a live member; send
     KILL to each process found that has not had it.
   - Return `survivor_note` of the PIDs still found.
   - Where any read fails, signal no group whose leader's start time cannot be confirmed: each
     signal of this path goes only to a recorded group whose leader a read of that one PID
     (`process`) shows with its recorded start time. Where a read fails before the first TERM, send
     TERM to those groups, wait `STOP_GRACE`, send them KILL, wait `KILL_WAIT`, and return
     `Some(TREE_UNCONFIRMED.into())`; where it can send TERM to none, return that note at once,
     without waiting. A read that fails after the first TERM goes on from the step it reached:
     KILL at `STOP_GRACE` after the first TERM, then `KILL_WAIT`, then `TREE_UNCONFIRMED`.

   Add `pub fn survivor_note(pids: &[i32]) -> Option<String>`: `None` for no PID, otherwise
   `client tree: PIDs P1, P2 outlived the stop`, with the PIDs in ascending order, separated by
   `, `. The note holds PIDs and fixed words alone: no process name, path, argument or client text.

5. **Where a `.v2` stop runs.** `Session::shutdown` becomes `pub async fn shutdown(&mut self) ->
   Option<String>`. It closes the client's stdin as now, then, where `self.config.v2`, runs
   `let note = self.tree.stop(false).await;` (for `.v1`, `None`), then kills and reaps the client
   as now, sets `stream` to `None` and returns the note. It stops the tree for `.v2` whether or not
   a stream is open, so that what an earlier Codex launch left running is stopped too. For `.v1`
   it does exactly what it does at `BASE`.
   - `Session::turn`'s failure text becomes
     `client turn K failed: CAUSE; session cannot restart implicitly`, followed, where the
     shutdown returned a note, by `; ` and the note. `failed` keeps the cause alone, so the
     refusal of later turns does not change.
   - `Session::cancel` returns the same form, with `cancelled` as the cause.
   - A successful turn stops nothing.

   So the failed answer of a `.v2` turn ends the call only after the client's tree is seen ended
   or named (P3: "no stop or fence is recorded before that"). OPS-014's text still holds the fence
   until the later order that [ruling 78](../proposals/ruling-78/README.md) point 6 names drops
   it, and this order relies on neither the fence nor its removal.

6. **TERM and the end of input**, in `main.rs`, for `.v2` alone. Keep `config.v2` before the
   config moves into the session, and take the session's tree before the session goes into its
   `Mutex`. Where `.v2`, create `tokio::signal::unix::signal(SignalKind::terminate())` before
   serving (an error fails `run`, as now), and spawn a task that, at its first signal, runs
   `tree.stop(true).await`, writes `{WRAPPER}: {note}` on stderr where it returns a note, and
   exits with status 143. It needs no lock on the session, which a turn may hold. At the end of
   input, where `shutdown` returns a note, write the same line on stderr. A `.v1` wrapper installs
   no handler and writes nothing new. The two `main.rs` stay one file.

7. **The scripted client.** In both packages' `tests/fixtures/client.rs`, make the same change: a
   message `TREE`, handled where `BLOCK` is, before any event is emitted. It starts the background
   job and the new session with stdin, stdout and stderr null; sh with stdin and stderr null and
   stdout piped, the grandchild's own descriptors redirected as shown:
   - the background job: `sleep 60`, as its own child;
   - the new session: `sleep 60`, as its own child, whose `pre_exec` (unsafe) calls
     `libc::setsid()` and sets TERM to `SIG_IGN`, which the exec keeps, so that only KILL ends it
     and a stop must find it outside the client's group;
   - the double fork: `sh -c 'sleep 60 </dev/null >/dev/null 2>&1 & echo $!'`, whose output gives
     the grandchild's PID; the client waits for `sh` to exit, so the grandchild's parent is gone
     and it stays in the client's group.

   Then it writes `tree.json` in its working directory, through a temporary name and a rename,
   holding `{"client":C,"background":B,"session":S,"doubleFork":D}`, and waits for `release` as
   `BLOCK` does. The two copies differ after this change only as they do at `BASE`.

8. **The test**, in each package's `tests/wrapper.rs`, with the same name in both:
   `v2_client_tree_stop_leaves_no_survivor`. Its observer is `ps`, independent of the wrapper's
   own table read: `ps -o pid=,ppid=,pgid=,stat= -p LIST` for the planted PIDs, a PID counting as
   ended where `ps` shows no line for it or a state beginning `Z`. A drop guard records each planted
   PID's `ps -o lstart= -p PID` when it is planted, and at the end, pass or fail, sends KILL to each
   one still shown with the same `lstart`, so a failing run leaves nothing and no reused PID is
   signalled. Three `.v2` cases and a `.v1` check, each in the package's own client and from a
   fresh temporary parent with `config(parent, false, false)` and `prepare(&c)` before the session
   starts; the three cases set `v2`. Each case and the `.v1` check prints `tree.json`'s four PIDs on stderr, with `eprintln!` before the stop (cargo test
   shows it on failure), and the report gives them:
   - **Shape.** In each case, once `tree.json` exists, `ps` shows the four PIDs live, with the
     client's process group equal to its PID and not the test process's group (decision 2); the
     background job's parent the client and its group the client's; the new session's group its
     own PID; and the double fork's group the client's and its parent not the client.
   - **Deadline.** `timeout` 2 seconds. Start `session.turn("TREE")` as a pinned future, race it
     against waiting for `tree.json` (it must not finish first), check the shape, then await the
     same future. It fails with a text that begins
     `client turn 1 failed: deadline: no turn boundary within 2 seconds (input turnDeadlineSeconds); session cannot restart implicitly`;
     at the moment it returns, each of the four PIDs is ended or named in the text, and the text
     holds a note only where a PID is not ended. The turn returns within 2 seconds plus
     `STOP_GRACE`, `KILL_WAIT` and 3 seconds. A next `TEST` turn is refused as
     `session failed at turn 1: deadline: ...`, as at `BASE`.
   - **Cancellation.** `timeout` 60 seconds. Race the turn against `tree.json` as above, drop the
     turn's future, then `session.cancel().await`: its text begins
     `client turn 1 failed: cancelled; session cannot restart implicitly`, and each PID is ended or
     named in it, as above.
   - **TERM.** Start the wrapper binary as `stdio_v2_defaults_and_invalid_configuration_remain_callable`
     does, with `AGENT_ACCOUNT_SERVICES` `false`, `AGENT_TURN_DEADLINE_SECONDS` `60` and
     `CONDUIT_STATE_DIR` the config's, stderr piped, and `process_group(0)`, as the runner's guard
     leads its run's group. Send a `TREE` turn (`send_request_with_option`, as the cancellation
     test does), wait for `tree.json`, check the shape, then send TERM to the wrapper's group
     (`libc::kill(-pid, SIGTERM)`), as the guard does. The wrapper exits within 10 seconds with
     status 143; each PID is ended or named on a stderr line that begins with the package's
     `WRAPPER`; and stderr holds no `synthetic-test-token` and no API key's value.
   - **`.v1`.** In the same test (the count stays 14), a `.v1` check: a session with `v2` unset
     and `timeout` 2 seconds runs `TREE`. Once `tree.json` exists, `ps` shows the client's
     process group equal to the test process's group (`libc::getpgrp()`).
     The failed text then equals
     `client turn 1 failed: deadline: no turn boundary within 2 seconds; session cannot restart implicitly`
     exactly, with no `client tree:` note. The drop guard then KILLs what the `.v1` stop leaves.

   The test also asserts `survivor_note(&[])` is `None` and `survivor_note(&[43, 42])` is
   `Some("client tree: PIDs 42, 43 outlived the stop")`. The test count goes from 13 to 14. Every
   other test keeps passing unchanged: tests that call `session.shutdown().await;` as a statement
   need no change. No test sleeps longer than the waits above, and none reads or writes outside its
   temporary directories.

9. **The READMEs**, both. Write no absolute local path and no client version.
   - In the paragraph that begins "The runner supplies a private empty working directory",
     "after it the client ends and the turn fails." becomes "after it the client ends, for `.v2`
     with its whole process tree, and the turn fails.", and "Active cancellation ends the client"
     becomes "Active cancellation ends the client, for `.v2` with its whole process tree,", each
     "whole process tree" a link to the new section's anchor, `the-v2-clients-process-tree`.
   - In `## Turn boundaries and failures (both versions)`, after the failed call's text, add that a
     `.v2` call's text then holds `; client tree: PIDs P1, P2 outlived the stop` where processes
     outlived the stop, or `; client tree: the process table could not be read, so the stop is
     unconfirmed`; and "Cleanup ends the client after the cause is selected." becomes "Cleanup ends
     the client after the cause is selected; for `.v2` it first stops the client's whole process
     tree."
   - Add, right before `## The wrapper-v2-path item` in `agent-codex/README.md` and before
     `## Turn boundaries and failures (both versions)` in `agent-claude-code/README.md`, a section
     `## The .v2 client's process tree` that says, in the package's own client's terms: each
     `.v2` launch starts the client as the leader of a new process group; every stop (a failed
     turn, a deadline, a cancellation, the wrapper's TERM and the end of its input) finds every
     live process of each group the wrapper started and every process reached from them by parent
     links, sends TERM, then KILL after two seconds, and waits one second more; of each process it
     reads its PID, parent, process group, start time and whether it is a zombie, from
     `/proc/PID/stat` on Linux and `proc_pidinfo` on macOS, and never a process's environment or
     arguments; the failed turn is answered only after the stop, and names in its text each PID
     that outlived it; on TERM the wrapper writes that note on its stderr, which the engine's own
     runner discards, and exits with status 143; a successful turn stops nothing, so what a turn's
     tools leave running stays until the next stop; and the `.v1` launch and stop are 0.1.1's. Its
     limits, stated as such: the wrapper reads the tree only at a stop and samples nothing between
     stops, so a process that left the client's group and whose parent links to the
     tree were gone before a stop's read, such as a double fork that also starts a new session, is
     neither stopped nor named; a client that exits before the stop, as at an exit or EOF failure,
     leaves any child in a session of its own neither stopped nor named; the client's group lies
     outside the run's process group, the unit by which the engine's own runner stops a run
     (PRV-030, DL16), so where the wrapper is killed without warning, at the engine's loss or by
     the runner's KILL after a `stop_grace` shorter than three seconds, the client's tree is
     neither stopped nor seen; under another runtime provider, what reaches the client's group is
     what that provider's contract states; and where the process table cannot be read (any system
     but Linux and macOS), the stop signals no group whose leader's start time it cannot confirm,
     and says it is unconfirmed.

10. **What this order does not do.** No change to `conduit.yaml`, `contracts.yaml`,
    `src/bin/package.rs` or the other tests, and none under `engine/`, `conformance/`, `spec/` or
    `docs/`. No failure fields (050e), no change to the `.v1` launch or stop, no un-poisoning, and
    no sampling of the tree between stops. The descriptions stay as they are (Q1 is 050f's and
    later orders'). The engine's test
    `live_s1_scripted_agents_call_engine_with_distinct_run_identities`
    (`engine/crates/shell/tests/agents.rs`) fails on this branch by design until order 050n moves
    `conformance/fixtures/agents/` to 0.2.0, as [order 050a](050a-agents-v2-own-tools.md) records.
    It is the one expected failure on `wo/050-agents-0.2.0`, and it is not this order's: neither
    you nor the verifier counts it against this order. Do not run the engine's tests and do not
    change it.

## Who decided what

The carrier is the full review's order 16, under D55, which stays the coordinator's, open to the
owner's strike ([ruling 82](../proposals/ruling-82/README.md#what-it-amends), the section after
point 6: "The rest of D55 and D80 stays the coordinator's"); ruling 82 point 5 struck only D55's
un-refusing for `.v2`. M65 is carried by item 7 of the review's must-state lists, which the owner
accepted with ruling 82. M64, which freezes order 16's stop as is, is a verdict of the targeted
review (verdicts.md, section G) that no must-state item traces to, and it decides nothing beyond
order 16. The split of order
050 into 050b to 050o, this order's place on `wo/050-agents-0.2.0` after 050c and its base are the
coordinator's, open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)), and so is
the replacement of the queue's "not stacked on 078" for order 16, since order 050 builds on 078's
branch. Everything else here is the coordinator's too, each open to the owner's strike: the client
of a `.v2` launch leading a new process group (`process_group(0)`), with the trade stated in the
README's limits (where the wrapper is killed without warning, the client's tree outlives it); the
walk of parent links and what it reads; the two-second grace and the one-second wait; signalling found processes by PID
with their start time checked; recording each group with its leader's start time, forgetting a
group whose leader's PID shows another start time, and signalling no group whose leader's start
time cannot be confirmed; the survivor note, its words and its place in the failed turn's text
until 050e; the reading of M65's "the wrapper's own log" as the wrapper's stderr, which nothing
keeps under the engine's own runner; the stderr line and status 143 on TERM; stopping nothing at a
successful turn and no sampling of the tree between stops, stated in the README as a limit; and
the README's wording. Apply them as
written and report them under `decisions` as the coordinator's.

## Scope

Change only, in `packages/agent-codex/` and `packages/agent-claude-code/`: `Cargo.toml`,
`Cargo.lock`, `src/lib.rs`, `src/main.rs`, `tests/wrapper.rs`, `tests/fixtures/client.rs` and
`README.md`. Nothing else, in the packages or outside them. The build's ignored output (`target/`,
`dist/`) may change.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

Each prints what is said beside it:

```bash
git status --short | grep -vE '^ M packages/agent-(codex|claude-code)/(Cargo\.toml|Cargo\.lock|src/lib\.rs|src/main\.rs|tests/wrapper\.rs|tests/fixtures/client\.rs|README\.md)$' | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml; echo $?   # 0
git diff "$BASE" -- packages/agent-codex/Cargo.lock | grep -c '^[-+] '   # 1
git diff "$BASE" -- packages/agent-claude-code/Cargo.lock | grep -c '^[-+] '   # 1
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'   # 2
diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs | grep -c '^[<>]'   # 0
diff packages/agent-codex/tests/fixtures/client.rs packages/agent-claude-code/tests/fixtures/client.rs | grep -c '^[<>]'   # 7
grep -c 'process_group(0)' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs   # 1 each
grep -cE '/environ|"environ"|cmdline|PROCARGS|pbi_comm|pbi_name' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs packages/agent-codex/tests/fixtures/client.rs packages/agent-claude-code/tests/fixtures/client.rs   # 0 each
grep -c 'the-v2-clients-process-tree' packages/agent-codex/README.md packages/agent-claude-code/README.md   # at least 2 each
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs   # 14 each
grep -rnE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

No comment in those four files names `environ`, `cmdline`, `PROCARGS`, `pbi_comm` or `pbi_name`,
so that the count of 0 holds for a correct change: say in words, such as "reads no argument", what
the table read leaves out.

In each of `packages/agent-codex/` and `packages/agent-claude-code/`, one command at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

`cargo test --release` reports, for `tests/wrapper.rs`, 14 passed and 0 failed, for the library's
unit tests 1 passed and 0 failed (none at `BASE`, decision 3's `parse_stat` test), and every other
test target as at `BASE`. Report the totals, the time `v2_client_tree_stop_leaves_no_survivor`
took, the planted PIDs each case printed, each failed turn's text and each stderr line that test
saw, and the digest that `package`
prints: it is a host build's. Say on which system the tests ran: `parse_stat` is compiled and
tested on every system, but a run compiles and tests only that system's table read (decision 3),
so say that the other system's was not run.

Then show that the test fails without decision 5's stop. In `packages/agent-codex/`:

```bash
cp src/lib.rs target/lib.rs.fixed
```

Make one edit to `src/lib.rs`: in `Session::shutdown`, replace `self.tree.stop(false).await` with
`None`; it still compiles, with at most a warning, which `cargo test` allows. Then run
`cargo test --release --test wrapper v2_client_tree_stop`; it must fail, in the deadline case, at
the assertion that a planted PID is ended or named. Restore with `cp target/lib.rs.fixed src/lib.rs`
(not `mv`, so that cargo rebuilds), rerun the same command, which passes, and check that
`cmp target/lib.rs.fixed src/lib.rs` prints nothing. Check with
`ps -o pid= -p LIST` on the PIDs the failing run printed, or that its guard recorded, that none is
left running.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, and give the last line of every check and each count above.
Tests that bind loopback ports may fail in your sandbox; if they do, say so, and the admin reruns
them.

The admin then stages any new file and runs `python3 tools/check_private_paths.py` and
`git diff --cached --check`. This order is a checkpoint on `wo/050-agents-0.2.0`: it does not merge
alone, and its Record is written at the foot of this file on that branch, as order 050c's is. A
fresh Codex verifier and a fresh read-only Opus reader read the result before the checkpoint stands
([ruling 79](../proposals/ruling-79/README.md), point 2). The local gate that
[ruling 80](../proposals/ruling-80/README.md) has a merge wait on, `tools/gate.py`, runs at order
050n's merge, read with the expected engine failure known.

## Record

Run on 2026-11-18, from about 06:18 to 07:02 UTC, on the Mac from `BASE` `wo/050-agents-0.2.0` at
`b92b3575` (the second reader's shoulds and the coordinator's rulings on order 050c), in two rounds:
the worker (1075 s) and a fresh Codex verifier (439 s), then the worker again (644 s) and a second
fresh Codex verifier (483 s). The change is the fourteen files of Scope, the seven it names in each
package. Each `.v2` launch starts the client as the leader of a new process group and records the
leader's PID and start time; every `.v2` stop (a failed turn, a deadline, a cancellation, the
wrapper's TERM and the end of its input) reads the process table, from `/proc/PID/stat` on Linux
and `proc_pidinfo` on macOS, finds the live members of each recorded group and every process
reached from them by parent links, sends TERM, then KILL after two seconds, and waits one second
more. The failed turn's text gains `; client tree: PIDs P1, P2 outlived the stop` where processes
outlived it, or the unconfirmed note where the table could not be read; on TERM the wrapper writes
that note on its stderr and exits with status 143. Each package gains `libc`, the library its
`parse_stat` unit test, the scripted client its `TREE` message, the wrapper suite
`v2_client_tree_stop_leaves_no_survivor` (13 tests to 14), and both READMEs decision 9's texts and
the section `## The .v2 client's process tree`. The worker applied decisions 1 to 10 as written. Its
own choices: ordered collections for PIDs and groups and a monotonic clock for the deadlines; a
macOS PID buffer sized from the system's estimate and grown when filled, and a Linux reader that
tolerates non-UTF-8 bytes in the unused process name; and a PID-scoped `ps` preflight before
planting, with a guard that signals a PID only when `ps` confirms its recorded start time. Its
sandbox denied PID-scoped `ps`, so the tree test stopped at the preflight in both packages (13
wrapper tests passed in each) and neither the regression nor the mutation proof ran; and
`cargo metadata --offline` exited 101 in both packages, since `wasm-bindgen-futures` 0.4.80 is not
in the cache. The first verifier, whose sandbox also denied `ps`, found five musts and two shoulds:
a Linux `/proc` enumeration error was dropped, so an incomplete table could report a clean stop; a
process that left its group between the read and the group signal was credited with TERM it never
got; end of input could return 0 after the TERM task had the signal; one failed start-time lookup
left later planted PIDs outside the guard; the regression and mutation results were missing; the
TERM case sampled `ps` twice; and the READMEs left out a group whose leader's start time was never
recorded. Round 2 propagated the enumeration error, tracked group and direct TERM apart, raced TERM
against serving in a biased `tokio::select!`, which it marked as its nearest sound reading of
decision 6, attempted all four lookups with a `tree-cleanup` request to the scripted client when one
failed, took one snapshot after the TERM exit, and documented the missing start time. The second
verifier found four musts and one should: the select drops the serving future, and with it the
session and its client, before the TERM stop's first read; a readable `/proc` with no PID entries
counts as a complete table; the fixture acknowledged cleanup while the orphan could still be live;
the regression and mutation results were still missing; and the `.v1` check asserts only the
group and the text, not that the client ended and its three descendants were left.

A fresh read-only Opus reader then read the result (ruling 79, point 2). On a scratch copy on
macOS arm64, where PID-scoped `ps` is allowed, it ran fmt and clippy (clean) and
`cargo test --release` in each package: 14 wrapper tests passed and 0 failed, the library's unit
test 1, every other target as at `BASE`; `v2_client_tree_stop_leaves_no_survivor` took 10.39 s
(Codex) and 10.21 s (Claude Code), with the deadline, cancellation and `.v1` texts as the order
gives them, TERM's status 143 with empty stderr, and no planted PID left. The Codex mutation failed
at the ended-or-named assertion, the restored source passed and `cmp` was silent. The digests
equalled round 2's. The Linux table read was neither compiled nor run, since no Linux target is
installed. It found one must and three shoulds:

- R1: decision 6 is not applied as written. The biased `select!` drops the serving future before
  the TERM handler runs, which releases the session that owns the client's `Child`, spawned with
  `kill_on_drop(true)`, so an idle client can be killed before the stop's first read and a child in
  a session of its own then loses its parent link and survives with no name.
- R2: on Linux, a readable `/proc` with no PID entries is taken as a complete table, so the stop
  forgets every group and reports clean where it should say the stop is unconfirmed.
- R3: the fixture's cleanup path wrote `tree-cleaned` even where it never read the orphan's start
  time and so could not signal it, so a failing run could leave a `sleep 60` running.
- R4: decision 9's sentence was not applied word for word: the READMEs kept the old full stop and
  added "For `.v2` it first stops the client's whole process tree." as a sentence of its own.

The fixer kept the branch on `wo/050-agents-0.2.0` at `b92b3575`, which had not moved, and applied
each finding, the same in both packages:

- R1: in `main.rs`, the session goes into its `Arc<Mutex<_>>` in `run()`'s scope, and only a clone
  moves into the serving future, so the session and its client live until `exit(143)`; the TERM
  handler still takes no lock, and the two `main.rs` stay one file.
- R2: the Linux `processes()` returns `NotFound` when the table holds no entry for the wrapper's
  own PID, which reaches decision 4's unconfirmed fallback; the two `lib.rs` still differ by 2
  lines.
- R3: where the scripted client cannot read the orphan's start time, it sends KILL to the orphan at
  once, kills its two direct children (beyond R3's text, so that a failing run leaves nothing), and
  panics before writing `tree.json`; on a cleanup request it signals all three, waits for both
  children and for up to half a second for the orphan to be gone, and writes `tree-cleaned` only
  when all three are confirmed ended, exiting 1 without it otherwise.
- R4: both READMEs read "Cleanup ends the client after the cause is selected; for `.v2` it first
  stops the client's whole process tree."

Of the second verifier's findings, the first three musts are R1 to R3, fixed, and the fourth is met
by the reader's and the fixer's runs; its should on the `.v1` check was not applied, since the
reader did not take it up and the order asks for neither assertion, and stays with the coordinator.

The fixer then ran, from the worktree's root with `BASE` set to `wo/050-agents-0.2.0`,
`python3 tools/check_docs.py` and `python3 tools/check_private_paths.py` (Errors: 0 each),
`git diff --check wo/050-agents-0.2.0...HEAD` (clean), and the text checks of "Done when", which
printed `0`, `0`, `1`, `1`, `2`, `0`, `7`, `1` and `1`, `0` in each of the four files, `2` and `2`,
`14` and `14`, and `0`. On macOS arm64, in each package, `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings` exited 0, and `cargo test --release` gave, for Codex, 14
wrapper tests passed and 0 failed, the library's unit test 1, the manifest test 1 and the package
test 1; for Claude Code, 14 wrapper tests passed and 0 failed, the library's unit test 1 and the
package test 1. With `--nocapture`, `v2_client_tree_stop_leaves_no_survivor` took 10.20 s (Codex)
and 10.42 s (Claude Code). The Codex run printed the planted PIDs (client, background, session,
double fork) deadline 20218, 20229, 20230, 20241; cancellation 20351, 20362, 20363, 20374; `.v1`
20479, 20490, 20491, 20502; and TERM 20598, 20609, 20610, 20621. The Claude Code run printed
deadline 24323, 24343, 24344, 24355; cancellation 24501, 24521, 24522, 24533; `.v1` 24929, 24949,
24950, 24962; and TERM 25229, 25249, 25250, 25262. In both, the deadline turn failed with
`client turn 1 failed: deadline: no turn boundary within 2 seconds (input turnDeadlineSeconds); session cannot restart implicitly`,
the next turn was refused as
`session failed at turn 1: deadline: no turn boundary within 2 seconds (input turnDeadlineSeconds); implicit restart refused`,
the cancellation failed with `client turn 1 failed: cancelled; session cannot restart implicitly`,
the `.v1` check's text was
`client turn 1 failed: deadline: no turn boundary within 2 seconds; session cannot restart implicitly`,
and the wrapper exited on TERM with status 143 and empty stderr, so no failed text held a
`client tree:` note. `ps -o pid= -p` on the sixteen PIDs of each run printed nothing. The Codex
mutation, `self.tree.stop(false).await` replaced by `None` in `Session::shutdown`, failed in the
deadline case at `tests/wrapper.rs:517`, "each live planted PID must be ended or named in the stop
report. Live PIDs: [20865, 20866, 20877]"; `ps` on its four PIDs printed nothing afterwards;
restored with `cp`, the test passed and `cmp` printed nothing. The package build printed
`dist/agent-codex.tar.gz#bddacd7efa6c09cf67d4f48ac4af8eecd4291fce9304ee13a1249ff3eebff268` and
`dist/agent-claude-code.tar.gz#b0f0656b755ce8dbdeddd93d41889e91db292b1c6326a9e362dc80f90026600f`,
both new with R1 and R2; they are a host build's. The Linux table read, with R2's check, was
neither compiled nor run. The engine's expected failure,
`live_s1_scripted_agents_call_engine_with_distinct_run_identities`, was not run. The gate's run,
with that failure known, is the coordinator's, on the squash at order 050n's merge.

A second fresh read-only Opus reader read the fixed result on a scratch clone of `169c8786`. Every
check of "Done when" held there, with the tree test at 10.27 s (Codex) and 10.26 s (Claude Code)
and the digests equal to the ones above, and it found no must and two shoulds:

- R5: the run times above were each one second short of the run log's, which gives 1075 s, 439 s,
  644 s and 483 s.
- R6: decision 6 was still not applied as written. The biased `select!` drops the serving future
  at TERM, and with it rmcp's drop guard, which cancels a turn in flight; that turn's cancellation
  then ran `Session::cancel`, and so a second stop of the tree, `stop(false)`, at the same moment
  as the TERM task's `stop(true)`. An instrumented scratch copy logged both stops in the same
  microsecond.

The coordinator chose decision 6 as written over the `select!` race, since two stops of one tree
must not run at once. This round applied both shoulds, the same in both packages:

- R5: the four run times read 1075 s, 439 s, 644 s and 483 s, as the log gives them.
- R6: `main.rs` spawns the TERM task, which at its first signal sets a shared `AtomicBool`, runs
  `tree.stop(true).await`, writes `{WRAPPER}: {note}` where the stop returns one and exits with
  status 143; `serve.await` is the only main path, so serving goes on and TERM cancels no turn.
  At the end of input, where the flag is set, the wrapper neither stops the tree nor writes the
  note, and awaits `std::future::pending()`, so that the TERM task owns the exit. The two `main.rs`
  stay one file.
- Beyond R6's text, in `lib.rs`: the spawned task alone still let a turn in flight run a second
  stop, since TERM's stop ends the client, the turn reads the end of its output and fails, and the
  failed turn's `Session::shutdown` runs `stop(false)`. An instrumented scratch copy of the Claude
  Code package with only the `main.rs` change logged `stop running close=true` and, 1.8 ms later,
  `stop running close=false` in the same wrapper. So `ClientTree` gains a second field, an async
  lock that one stop holds while it runs, and `stop(false)` waits for good where TERM's stop has
  begun: before the lock where it has already begun, and after the lock where TERM's began while
  this stop was waiting for it. A stop that began before TERM runs to its end and TERM's stop then
  runs after it. The two `lib.rs` still differ by 2 lines. With the change, the same instrumented
  run logged, in the TERM case, `stop running close=true`, then `stop entered close=false` and
  `stop close=false parked before the lock: TERM's stop has begun` 1.8 ms later, then the TERM
  task's exit with status 143 about 2 s after, and no second running stop. The lock, and the parking
  of every stop(false) once TERM's stop has begun, are this round's own choice, beyond decisions 4
  and 6 as written. A TERM that arrives during a running stop now waits for it, at most STOP_GRACE +
  KILL_WAIT, before its own stop. The coordinator accepts them, since two stops of one tree must not
  run at once, open to the owner's strike.

This round's runs, on macOS arm64 from the worktree's root and in each package, with PID-scoped
`ps` allowed: `cargo fmt --check` and `cargo clippy --offline --all-targets -- -D warnings` exited
0; `cargo test --offline --release` gave, for Codex, 14 wrapper tests passed and 0 failed, the
library's unit test 1, the manifest test 1 and the package test 1, and for Claude Code, 14 wrapper
tests passed and 0 failed, the library's unit test 1 and the package test 1. With `--nocapture`,
`v2_client_tree_stop_leaves_no_survivor` took 10.21 s (Codex) and 10.22 s (Claude Code). The Codex
run printed the planted PIDs (client, background, session, double fork) deadline 58753, 58773,
58774, 58785; cancellation 58909, 58920, 58921, 58932; `.v1` 59050, 59061, 59062, 59073; and TERM
59176, 59187, 59188, 59199. The Claude Code run printed deadline 59346, 59368, 59369, 59380;
cancellation 59491, 59511, 59512, 59523; `.v1` 59629, 59649, 59650, 59661; and TERM 59757, 59777,
59778, 59789. Both gave the deadline, retry, cancellation and `.v1` texts above word for word, and
TERM's status 143 with empty stderr. `ps -o pid= -p` on the thirty-two PIDs and on the instrumented
runs' planted PIDs printed nothing. The package build printed
`dist/agent-codex.tar.gz#8d76e2204107c10f9473a913d79468a7f301afb93e308d27e71556f94d775dba` and
`dist/agent-claude-code.tar.gz#124ba40ef33ce4beec9fd8a8eefd5ca8334334333fca01f1124c18a24e1cee83`,
both new with R6; they are a host build's. `python3 tools/check_docs.py` and
`python3 tools/check_private_paths.py` gave Errors: 0 each, and
`git diff --check wo/050-agents-0.2.0...HEAD` was clean. The Linux table read was neither compiled
nor run, and the engine's expected failure,
`live_s1_scripted_agents_call_engine_with_distinct_run_identities`, was not run.
