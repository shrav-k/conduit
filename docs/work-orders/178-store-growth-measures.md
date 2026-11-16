# Work order 178: the store's growth, one measures line per commit

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only because the
shell's tests bind loopback ports; use it for nothing else, and reach no other machine. Run one
cargo command at a time, and never print a whole large file, `state.json` or log. End every
process that a test of yours starts. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 28 of [its orders](../proposals/full-review/orders.md))
asks the engine to measure its own store. A78 runs the work orders through Conduit, and its
evidence on the store is "state bytes at start and end, bytes per invocation, commit time against
size, restart time"
([its A78 conditions](../proposals/full-review/experiments.md#a78-the-conditions-per-order)).
[FR-358](../proposals/full-review/findings/P07-engine-build.md#fr-358)'s verifier found that those
numbers can come today only from the dogfood harness, which is outside the repository, and asked
that the engine emit them, a line per commit, so that a slope inside one store can be read from
the tree. [FR-351](../proposals/full-review/findings/P07-engine-build.md#fr-351)'s verifier put
the store's rework behind that measure (its step (c)), and the coordinator's D16
([decisions](../proposals/full-review/decisions.md)) makes it the rule: the store's cost (restart
seconds, commit time, state size) is measured first, and a store order goes ahead of 047 only if
the measure shows a cost to the dogfood or the rerun. The one restart figure on record, 171 to 191
seconds, came from an engine older than order 070, and its cause is not in the tree (FR-351's
verifier).

Today `Shell::commit` (`engine/crates/shell/src/lib.rs` lines 511 to 598) writes the whole State
to `state.json` at every commit, through `persist` (lines 434 to 462) and `Store::write_state`
(`engine/crates/shell/src/store.rs` lines 242 to 268), and keeps no account of the time or the
bytes. Only the `#[ignore]`d test `commit_latency_on_s1` (`engine/crates/shell/tests/agents.rs`
lines 393 to 451) times commits, on S1 and by hand. A78's registration of its orders 2 and 3,
which [order 170](170-a78-pre-registration.md) wrote into
[experiment 028](../../experiments/028-a78-dogfood/README.md) under "Orders 2 and 3: registered
before order 2 runs", says so: "The time each commit takes is not measured: the engine does not
write it yet." This order has the shell append one line of measures to a file in the store's
directory at each commit. It changes no state, record, reply or outcome. The owner ruled on
2026-11-16 that this file is not part of the store that F15 is to close: a line that is not
written is reported and refuses no request ([ruling 80](../proposals/ruling-80/README.md), point 5;
[IN_FORCE.md, topic 7](../../IN_FORCE.md#7-operations-outcomes-revisions-records-and-observation)).
The order decides nothing about the store itself: package files by digest, held contexts, and
SQLite or a log of transitions (FR-351's steps (a), (b) and (d)) wait for what the lines show.
DL2 is implementation-defined, and its row is the coordinator's, as order 070 and FR-351 say
(ruling 46, point 4). A78's orders 2 and 3 run through Conduit now, one at a time (ruling 80,
point 7). The order is best merged before the dogfood engine is re-pinned to `main`, so that they
run on an engine that writes the lines. Amending A78's registration to read them (and to note
that `du -sk` of the store then counts this file too) is the coordinator's, not this order's.
Order 079, parked on `s3/m2`, also changes `Shell::open`: build on `main` as it is.

## The decisions it applies

Line numbers are those of `main` at `d4c8cf8b`; no file under `engine/` has changed since
`260f3cba`. If a line has moved, find it by its words.

1. **One line per commit, appended once the commit is durable.** A commit is one call of
   `Shell::commit` that returns `Ok`: the `recover` commit that `Shell::open` makes (line 218),
   the one each `begin` makes, the ones `drive` makes for completions and setup reports (line
   647), and the ones a test helper makes by calling `commit` itself. After
   `writer.store.remember(stored)` (line 592) and before `commit` returns, still under the writer
   mutex, the shell appends one line to the file `commit-measures.jsonl` in the store directory
   (`writer.store.directory`). So the lines are in commit order. A commit that returns an error
   appends no line, even where it wrote `state.json` before its error. The line is one JSON
   object in compact form, ended by one `\n`, written with a single `write_all` to a file opened
   for appending. It is never fsynced: a crash may lose the last lines, never a commit. The
   append is not part of the time the line reports.
2. **What a line holds.** Exactly these keys, and no other:
   - `at`: the UTC time the line is made, in the form that
     `Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)` gives (as at line 360);
   - `input`: the `kind` of the commit's input, such as `recover`, `invoke`, `complete` or
     `list-tools`;
   - `stateBytes`: the length of the last `state.json` the commit wrote. `write_state` returns the
     number of bytes it wrote; the `#[cfg(test)]` `Store::write` (lines 269 to 273) keeps its
     `Result<()>`;
   - `invocations`: the number of entries in the committed State's `invocations`;
   - `writes`: the number of `state.json` writes the commit made: 1 for its own transition, plus
     1 for each `expire-outputs` transition that its two retention checks (lines 516 and 582)
     committed;
   - `stepMicros`: the summed wall time of the `self.transition` calls of those writes (line 524,
     and lines 498 to 502 in `retention`): the core's step with the shell's evidence, its retry
     and any archive read;
   - `writeMicros`: the summed wall time of the `write_state` calls (line 461): serialization, the
     write, the file's fsync, the rename and the directory's fsync;
   - `commitMicros`: from the moment `commit` holds the writer lock (line 512) until `remember`
     returns (line 592). It holds the two above, and also the retention scans, the cleanup of
     instance directories (line 528), the waiters and the tool-list notices. The wait for the lock
     is not counted;
   - `openMicros`, on the line of the `recover` commit that `Shell::open` makes and on no other:
     from the start of `Shell::open` (line 167) until it calls `commit` (line 218). It holds the
     store's lease, the read and parse of `state.json`, the engine address file and the signing
     authority. The `recover` that `conduit serve` submits as it ends
     (`engine/crates/shell/src/main.rs` line 222) does not carry it;
   - `unwritten`, only as decision 4 says.

   Each time is a whole number of microseconds from `std::time::Instant`, written as a JSON
   integer (saturated at `u64::MAX`). A line names no object, instance, invocation, path or value
   from the state, so that the file can enter the tree as evidence with no redaction.
3. **The file is private to the store.** At each commit the shell opens the file for appending,
   creating it with mode 0600, with `O_NOFOLLOW` and `O_NONBLOCK` (through
   `std::os::unix::fs::OpenOptionsExt::custom_flags`; `libc` is already a dependency). It then
   checks the opened file's own metadata: a regular file with mode 0600, as `private_read`
   (`store.rs` lines 52 to 58) asks of the files it reads. So a symbolic link is not followed and
   a FIFO does not block the writer. The engine never reads, truncates or rotates the file.
   Deleting or moving it between commits is safe, and the next commit makes a new one. No flag or
   `Config` field turns it off. Opening a store reads nothing of it, so a store made before this
   order opens unchanged, and the store's format version stays 1. Put the append in `store.rs`,
   beside `atomic_write`, and the gathering of the measures in `lib.rs`. `persist` keeps its
   parameters, since the test helper `filtered_shell` calls it directly (`tests.rs` line 621) and
   that call must compile against both `main`'s `lib.rs` and yours (decision 6, and Done when);
   it may return what it measured, such as the bytes and the write's time, which that call
   discards. `retention` may take a value that gathers the measures. The open's time and the
   count of decision 4 may live in `Writer` (lines 142 to 145) or in `Store`, as you choose.
4. **A line not appended changes no result, and is reported twice** (ruling 52). If the file
   cannot be opened, is not a regular file with mode 0600, or cannot be written, the commit
   returns what it would have returned, and nothing else changes. The shell then:
   - prints one line to standard error, with the `conduit: ` prefix that `conduit` gives its own
     diagnostics (`engine/crates/shell/src/main.rs` line 416). The library already prints some
     lines to standard error itself, as the runner does with its prefix `runner: `
     (`engine/crates/shell/src/runner.rs` line 337). Write this line with `writeln!` on
     `std::io::stderr()` and ignore the result, not with `eprintln!`, which panics, under the
     writer mutex, when standard error cannot be written. Where the file is not a regular file
     with mode 0600, the line is the first of these two; otherwise it is the second:

     ```text
     conduit: the measures line of a commit of kind <input> was not appended: commit-measures.jsonl is not a regular file with mode 0600
     conduit: the measures line of a commit of kind <input> was not appended: <error>
     ```

     `<input>` is the line's `input`, and `<error>` is the `std::io::Error` of the open, the
     metadata read or the write that failed, as Rust displays it: for an operating-system error,
     its message and its number. Neither names a path, and the line carries no other value. The
     test harness does not capture a line written so, so the cargo commands of Done when show it
     whether or not they pass `--nocapture`;
   - counts the line as unwritten. The next line appended carries `unwritten`: the number of
     lines not appended since the last line appended, or since the shell opened. The count then
     returns to 0. A line has the key only when that number is 1 or more. The count is kept in
     memory: lines not appended before a restart are reported only on standard error.

   A commit that returns an error has no line due, so it is not counted (decision 1).
5. **What stays.** The bytes of `state.json`, its format version 1 and how it is written
   (`atomic_write_inner`, `store.rs` lines 80 to 122); every reply, record, outcome, intent and
   notice; what `conduit` prints on standard output; `Config` and the command line; the core, the
   vectors, `spec/` and `commit_latency_on_s1`. The store failing closed (F15, ruling 78, point
   17) is a later order, SO-F (the full review's order 53), and is not built here.
   `commit-measures.jsonl` is not part of the store that point means
   ([ruling 80](../proposals/ruling-80/README.md), point 5): a line not appended is reported, as
   decision 4 says, and never refuses a request.
6. **Tests**, in `engine/crates/shell/src/tests.rs`. Both compile against `main`'s `lib.rs` and
   `store.rs`: they name the file by its literal name and use no item that this order adds.
   - A new test, `commit_measures_one_line_per_commit`. It reads `commit-measures.jsonl` in the
     store and parses each line as JSON. In order:
     1. It opens a shell on a `temporary()` store (line 12) with `open` (line 77). It asserts
        that the file exists, then that it has mode 0600 and holds one line. That line's `input`
        is `recover`, its `openMicros` is an integer, and it has no `unwritten`.
     2. It submits a first start, as lines 86 to 91 do, then, as root (`actor`, line 304),
        `{"kind":"list-tools","identity":{"kind":"instance","id":<root>}}`. The file holds three
        lines, the second with `input` `first-start` and the third with `list-tools`. The first
        line's keys are exactly decision 2's eight, `at` to `commitMicros`, plus `openMicros`;
        the second's and the third's are exactly the eight. On each, `writes` is 1, `at` parses
        as RFC 3339 and ends in `Z`, and `stepMicros` plus `writeMicros` is at most
        `commitMicros`. On the third, `stateBytes` equals the length of `state.json`, and
        `invocations` equals the length of `shell.state()`'s `invocations`.
     3. It sets the store's fault to `store::WriteFault::Write`, as line 1613 does, and submits
        the same `list-tools`, which fails. The file is byte for byte as it was.
     4. It sets the file's mode to 0644 and submits the `list-tools`, which succeeds. The file is
        byte for byte as it was. It sets the mode back to 0600.
     5. It renames the file to `commit-measures.old` in the store, makes `commit-measures.jsonl`
        a symbolic link to `missing-target` in the store, as
        `store_rejects_versions_permissions_and_competing_owners` does for `writer.lock` (from
        line 1085), and submits the `list-tools`, which succeeds. `missing-target` does not
        exist. It removes the link.
     6. It submits the `list-tools`. The file exists again, has mode 0600 and holds one line,
        whose keys are exactly the eight and `unwritten`, which is 2.
     7. It drops the shell and opens the store again. The file holds two lines. The second has
        `input` `recover`, and its keys are exactly the eight and `openMicros`.
   - In `retention_cap_oldest_first_and_age_survive_restart` (lines 498 to 570), around the
     second `submit` (lines 526 to 531). Just before it, read the file's lines, and count the
     invocations in `shell.state()` that keep an output: those for which `retained_results`
     (`lib.rs` line 666) yields at least one. Just after it, read the file and count again.
     Assert that:
     - every line read before the submit has `writes` 1;
     - the submit appended exactly two lines, the first with `input` `invoke` and the second with
       `input` `complete`;
     - the second's `writes` is at least 2;
     - the two lines' `writes` sum to 2 plus the number of outputs the submit expired: the count
       before, plus 1 for `newer`, less the count after.

     The cap now set expires the oldest kept outputs first. These include the one that the
     registration made by `established` keeps (`engine/crates/core/src/records.rs` line 599).
     So the invoke's retention check before its transition (line 516) can expire outputs, and
     the completion's check after its transition (line 582) expires `older`'s. The sum therefore
     counts both checks. Report the two lines' `writes` and the two counts. Nothing else in the
     test changes.

   Every other test passes unchanged, and nothing else in `tests.rs` changes: `filtered_shell`
   keeps its call of `persist` as it is (decision 3).
7. **`engine/CONFORMANCE.md`.** In the row "DL2: storage" (line 25), add this text at the end of
   the choice's cell, after "The store's parent must exist.", as plain text, without the `>`:

   > After each commit that succeeds, still under the writer mutex, the shell appends one line of
   > compact JSON to `commit-measures.jsonl` in the store directory, with no fsync: `at`, the
   > input's kind as `input`, `stateBytes` and `invocations` after the commit, `writes` (its
   > transition's and each expiry's), and `stepMicros`, `writeMicros` and `commitMicros`; the
   > recover commit that opening makes also gives `openMicros`. A line names no object,
   > invocation or path. The file is created 0600 and never followed through a link, and one that
   > is not a regular file with mode 0600 gets no line; the engine never reads, truncates or
   > rotates it. A line that cannot be appended changes no result: a line naming the input's kind
   > and the cause goes to standard error, and the next line appended carries `unwritten`.

   In the row's last cell, after "killed-child tests before and after send", add a comma and a
   space, the test's name `commit_measures_one_line_per_commit` in backticks, the word "in", and
   a Markdown link whose text is "the shell's tests" and whose target is
   `crates/shell/src/tests.rs`, written as the cell's link to `store.rs` is written. The row
   stays one line, and nothing else in the file changes.

## Scope

Change only `engine/crates/shell/src/lib.rs`, `engine/crates/shell/src/store.rs`,
`engine/crates/shell/src/tests.rs` and `engine/CONFORMANCE.md` (the row of decision 7). No vector
changes, so the order has no declaration file, and no promise changes, so CONTRIBUTING.md rule 8
asks for no case. The code, comments and tests it adds name no promise ID that the shell's sources
do not already name, so `engine/STANDING.md`, which `tools/promise_standing.py` writes from those
sources, does not change. Change nothing under `conformance/`, `spec/` or `experiments/`, no other
crate, `ROADMAP.md`, the handover, or any order's file.

## Done when

From `engine/`, each passes, one at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-shell --lib commit_measures_
cargo test --release -p conduit-shell --lib commit_measures_ -- --nocapture 2>&1 | grep 'conduit: the measures line of a commit of kind list-tools was not appended: '
cargo test --release -p conduit-shell
```

The third command reports exactly 1 passed. The fourth prints exactly two lines: first step 4's,
ending `commit-measures.jsonl is not a regular file with mode 0600`, then step 5's, ending in the
operating system's error for the symbolic link that `O_NOFOLLOW` refused. Neither names a path.
Then show that the test fails with the change removed. From `engine/`, run these one at a time:

```bash
cp crates/shell/src/lib.rs target/lib.rs.fixed
cp crates/shell/src/store.rs target/store.rs.fixed
git show HEAD:engine/crates/shell/src/lib.rs > crates/shell/src/lib.rs
git show HEAD:engine/crates/shell/src/store.rs > crates/shell/src/store.rs
cargo test --release -p conduit-shell --lib commit_measures_
cp target/lib.rs.fixed crates/shell/src/lib.rs
cp target/store.rs.fixed crates/shell/src/store.rs
cargo test --release -p conduit-shell --lib commit_measures_
```

Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not
rebuild the crate and would run `main`'s code again. Git ignores `target/`. With `main`'s two
files, the test must compile and then fail at its assertion that the file exists once the shell
has opened. Report the failing assertion's line. After the restore, the test must pass again.

Then, from the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/promise_standing.py --check
python3 tools/check_private_paths.py
python3 tools/vector_delta.py HEAD
git diff --check
git diff --numstat -- engine/CONFORMANCE.md
grep '^| DL2: storage |' engine/CONFORMANCE.md | grep -o -e 'commit-measures\.jsonl' -e 'commit_measures_one_line_per_commit' -e 'unwritten' | sort -u | wc -l
git status --short
```

`vector_delta.py` must end `Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.`; where
`python3` lacks PyYAML, run it with `/usr/bin/python3`. The `numstat` line is
`1	1	engine/CONFORMANCE.md`, the `grep` pipeline prints 3 (the Mac's `wc` pads it with spaces),
and `git status --short` lists only the four files in scope. Do not stage anything. In your
report, mark each choice you made yourself as your own. Give the failing assertion's line, the two
lines the fourth cargo command printed, the two `writes` and the two counts from the retention
test, and the last line of every check.

Before it merges, a fresh Codex verifier and a fresh read-only Opus reader read the result
([ruling 79](../proposals/ruling-79/README.md), point 2). It merges on a pass of the local gate,
the coordinator's interim gate or `tools/gate.py` once order 193b has merged. CI's engine build,
its four engine shards and the doctests run on the pull request, on Linux, as a second opinion
that the merge does not wait for; a run that fails after the merge is fixed in a follow-up
([ruling 80](../proposals/ruling-80/README.md), point 10).

## Record

Run on 2026-11-16 on the Windows computer from `main` `55d2b85a`: the worker (339 s) and a fresh
Codex verifier (132 s), which found no must or should in one round. Cargo was offline there and
its cache lacked `jsonschema`, so only `cargo fmt --check` ran, and passed: clippy and every cargo
test stopped at dependency resolution, and the verifier read the change without running it. The
shell now appends one line of compact JSON to `commit-measures.jsonl` in the store directory after
each commit that returns `Ok`, still under the writer mutex, with no fsync. A line holds `at`,
`input`, `stateBytes`, `invocations`, `writes`, `stepMicros`, `writeMicros` and `commitMicros`; the
recover commit that `Shell::open` makes also gives `openMicros`. `write_state` returns the bytes it
wrote, `persist` returns those bytes and the write's time, and `retention` adds each expiry's step
and write to the commit's measures. `Writer` keeps the open's time and the count of lines not
appended, which the next line appended carries as `unwritten`. The new
`store::append_commit_measures` creates the file 0600 with `O_NOFOLLOW` and `O_NONBLOCK`, and
refuses one that is not a regular file with mode 0600. A line not appended changes no result: a line
naming the input's kind and the cause goes to standard error with the prefix `conduit: `. The new
test `commit_measures_one_line_per_commit` walks the seven steps of decision 6, and the retention
test now checks the `writes` of both retention checks. As its own choice, the worker added a
`println!` of those numbers to that test. DL2's row in `engine/CONFORMANCE.md` says all of this.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), and ran on the Mac the
order's first clippy and cargo tests. It found all seven decisions applied and no must. Its one
should was that the worker's report lacks the evidence that Done when asks for. On a scratch copy
with `main`'s `lib.rs` and `store.rs`, the new test compiled and failed at its assertion that the
file exists, and passed once the two files were restored with `cp`. The order's `git show HEAD:`
would have restored nothing, since `HEAD` is the worker's checkpoint commit, so it took the two
files from `main`. Three faults it planted also failed the tests: without `O_NOFOLLOW` at `tests.rs`
line 166, without the mode check at line 161, and without counting the first retention check's
writes at line 663 (7 against 14). It found the `println!` harmless and not blocking.

The fix applied that should: this Record carries the evidence from the Mac, and no code changed.
On 2026-11-16 the coordinator's fixer ran Done when on the Mac, on the branch, against `main`
`a771b88b`, which has not touched `engine/` since `55d2b85a`. Each cargo command ran alone from
`engine/` under `env -i`, with `CARGO_NET_OFFLINE=true`. The last line of each check:

- `cargo fmt --check`: exit 0, no output.
- `cargo clippy --all-targets -- -D warnings`: exit 0, no warning, and
  ``Finished `dev` profile [unoptimized + debuginfo] target(s)``.
- `cargo test --release -p conduit-shell --lib commit_measures_`:
  `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 53 filtered out`.
- The same with `-- --nocapture`, through the order's `grep`: exactly the two lines below, step
  4's first and step 5's second. Neither names a path.
- `cargo test --release -p conduit-shell`: exit 0. The library's tests report 54 passed, every
  test binary is ok, and the last line is the doc-tests' `test result: ok. 0 passed`.
- `cargo test --release`, the whole workspace: exit 0, and all 28 test results are ok. The two
  lines below are the only diagnostics in its output.
- The retention test: the submit's `invoke` line has `writes` 8 and its `complete` line 6. Before
  the submit 12 invocations kept an output, and after it 1, so 8 + 6 = 2 + 12 + 1 - 1 = 14.
- The fix-removal check, on a scratch copy made with `git archive HEAD` and given `main`'s
  `lib.rs` and `store.rs`: the test compiled and failed at `crates/shell/src/tests.rs:86:5` with
  `assertion failed: path.exists()`. Once the two files were restored with `cp`, it reported
  `1 passed`. The scratch copy was then removed.
- From the root, with `/usr/bin/python3`: `check_docs.py`, `check_rulings.py`,
  `set_names.py --check`, `check_schemas.py` and `check_private_paths.py` each end `Errors: 0`.
  `promise_standing.py --check` ends `engine/STANDING.md is current; promise rows: 519.` and
  `Errors: 0`. `vector_delta.py origin/main HEAD` and `vector_delta.py HEAD` each end
  `Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.`
- `git diff --check origin/main...HEAD` prints nothing. The `numstat` of `engine/CONFORMANCE.md`
  is one line added and one removed, the `grep` pipeline prints 3, and before this Record
  `git status --short` printed nothing, since the work is in the checkpoint commit.

```text
conduit: the measures line of a commit of kind list-tools was not appended: commit-measures.jsonl is not a regular file with mode 0600
conduit: the measures line of a commit of kind list-tools was not appended: Too many levels of symbolic links (os error 62)
```
