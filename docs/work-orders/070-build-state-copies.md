# Work order 070 (E1): stop copying the whole state on every commit

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count. The stores your tests make hold signing keys and tokens:
never print one.

## What and why

Finding 4-18 of increment 4 ([the log](../proposals/first-build/log-increment-4.md)): the shell's
unit tests took 5 minutes 33 seconds and 31 minutes of processor time, and a profile put that time
in copying, parsing and dropping the whole state on each commit. The code shows where. Line
numbers are `main`'s at `cfd7b23`, in `engine/crates/` (nothing under `engine/` has changed on
`main` since):

- `step` copies the whole State for every transition (`core/src/lib.rs:301-303`).
- Publishing copies it again for every tool. `schemas::publish` (`core/src/schemas.rs:987-994`)
  calls `access::publication_state` (`core/src/access.rs:93-112`), which clones the State.
  `tool_list_changes` (`access.rs:184-199`) publishes every tool of every consumer, through
  `tools()` (`access.rs:134-156`), on both sides of each commit.
- Each access evaluation copies it twice (`access.rs:419`, and `:619` through `publish`), and one
  invoke evaluates twice (`core/src/requests.rs:677` and `:861`). `lifecycle.rs:327` and
  `records.rs:281` copy it once more.
- Two trials copy the whole transition (`requests.rs:920` and `:1005`).
- Registration copies it four more times: the `register.v1` preflight's trial
  (`core/src/records.rs:72-79`), the `replace.v1` preflight (`records.rs:110`), and the Builders
  of `register_project` and `replacement` (`core/src/registration.rs:1709` and `:2336`). This order
  leaves these four (step 5).
- The shell parses `state.json` on every commit and on every read of the state, including the
  token check on every request (`shell/src/lib.rs:220-227`, `:278-282`). It copies the state twice
  more per commit (`lib.rs:438`, `:491`), and once more for the test trace (`:497-501`).
  `unfinished()` (`shell/src/server.rs:92-132`) publishes per tool again on every tool call.

This order removes those copies and parses, except registration's four. It changes no outcome,
record, ID, vector or specification text. What a transition decides, what the store holds, and the
bytes `state.json` holds for a given state stay the same. Only how often the state is copied and
parsed changes. The DL2 storage row of [engine/CONFORMANCE.md](../../engine/CONFORMANCE.md)
changes to say how; that is the coordinator's decision, under ruling 46, point 4. The promises
this order must keep:

- IFC-012, IFC-033 and IFC-038: each consumer's tool list, each tool's definition, and the notice
  that a list changed;
- ACC-024, ACC-033, ACC-039 and ACC-054: each access check reads the same facts as today (step 2);
- IMP-007, REC-003 and OPS-013: an invocation is dispatched at most once per attempt; a record is
  removed or altered only by a purge; an unknown outcome stays unknown until it is reconciled. The
  shell's crash tests guard them;
- DL2, DL6, DL10 and DL13, as `engine/CONFORMANCE.md` lines 16-19, 23, 30 and 33 implement them.

**Your base.** This order starts from order 041b's verified checkpoint, not from `main`, and it
merges after 041b. The admin names that checkpoint's commit at dispatch; call it BASE. On 041b's
tree some sites above sit lower: `lib.rs:305`, `access.rs:425`, `lifecycle.rs:343` and `:369`,
`requests.rs:1006`, `registration.rs:1718` and `:2345`, and `server.rs:61` and `:94`. Find each
site by the function named, not by its line.

After 041b merges, the admin rebases this order onto `main`, and a worker round follows. In that
round:

- Recheck every site by name, since 041b's last round may add a copy. Search the core for
  `publication_state`, `access::evaluate`, `state.clone()`, `t.clone()` and `s.clone()`. A
  whole-state copy in `registration.rs`, or in a preflight of `records.rs`, stays, as step 5 says;
  report each one you find.
- Expect conflicts with two orders, and keep their lines:
  - order 072 edits `server.rs` beside `unfinished()`;
  - order 068 edits the shell's tests (`src/tests.rs`, `src/server_tests.rs`, `tests/agents.rs`,
    `tests/cli.rs`, `tests/runner.rs`, `tests/serve.rs`), mostly to raise waits to 600 seconds,
    and adds `engine/.config/nextest.toml`, whose test groups name tests by their paths.
- Rerun every check of "Done when", with BASE as that section says.

## What to do

1. **Measure first, before any other change.**
   - Add `commit_latency_on_s1`, an `#[ignore]`d test in `engine/crates/shell/tests/agents.rs`.
     It sets up `engine/examples/s1` through the set-up loop, as the live test
     `live_s1_scripted_agents_call_engine_with_distinct_run_identities` does. Factor the shared
     steps into helpers. The live test keeps its name and checks exactly what it checks now.
   - It then times 20 commits through `Shell::submit` of each of two inputs: `list-tools` for
     root; and root's `inspect.v1` of `agent-a`, through the Conduit that ACC-033 routes it
     through. The inspection must succeed, and the engine must run it inside the core; if it does
     not, choose an input that does and name it.
   - It prints:
     - the median and the maximum of each, in milliseconds;
     - the wall time of the set-up's one `register-project` submission, which runs registration's
       copies;
     - the byte size of `state.json`, and how many of those bytes the objects take and how many
       the installed packages' files take.
   - In `engine/`, build each target with `--no-run`. Then run each `agents.rs` run below once,
     untimed: its first run also builds `packages/agent-claude-code` and `packages/agent-codex`
     (`pack()`, `agents.rs:67-88`), which `--no-run` does not. Then time these runs:
     - `cargo test --release -p conduit-shell --lib`
     - `cargo test --release -p conduit-shell --test agents`
     - `cargo test --release -p conduit-conformance --test vectors`
     - `cargo test --release -p conduit-shell --test agents -- --ignored --exact commit_latency_on_s1 --nocapture`
   - Record the values of `CARGO_BUILD_JOBS` and `RUST_TEST_THREADS` as in effect ("unset" if
     unset), and the machine's count of logical processors. Print no other variable.
2. **One publication view per call, of objects only** (IFC-012, IFC-033). Replace
   `access::publication_state` with a view that:
   - holds objects only: never records, invocations, installed packages or snapshots;
   - normalizes exactly as now: a Conduit's `consumer` and `target` as IDs, and its `local_name`;
     an instance through `registration::publication_object`; a template unchanged;
   - borrows what it leaves unchanged, and normalizes each object at most once. Normalize lazily
     where you can: only the objects a call looks up. `publication_object` clones and parses each
     instance's held environment and serializes it again, so a view that normalizes every
     instance costs nearly what the copy did;
   - is built at most once per call that publishes or evaluates access, never once per tool. The
     plan says once per transition; this order does not, because a transition changes its
     objects between such calls, and a view kept across a change goes stale. With lazy
     normalization, the two evaluations of one invoke cost only the objects each looks up.

   Laziness, and the `list.v1` change below, mean a call no longer fails because an object it
   never reads fails to normalize. If any vector or test changes because of that, stop and report
   it.

   `schemas::publish` takes the view instead of building one. Each caller builds one view, or is
   given one, and passes it on:
   - `tools()`;
   - `access::evaluate`, which passes it to `evaluate_path` and so to `publish`;
   - `request_for` in `lifecycle.rs`, which also passes it to `access::evaluate` for each
     candidate (`lifecycle.rs:353`) instead of letting it build another;
   - `Input::ToolDefinition` in `step` (`lib.rs:447`);
   - the shell's `unfinished()`, one view per call.

   **`evaluate_path` reads both the transition's State and the view** (ACC-024, ACC-033,
   ACC-039, ACC-054). Today `evaluate` passes the whole publication copy to `evaluate_path` as its
   State (`access.rs:442-443`), so `evaluate_path` reads that copy for everything. Split its reads
   as follows, so that no access decision changes:
   - **From the view: every object read, as today.** These are the selected Conduit and its
     target, the object `routing` names, root and management, the Conduits `current_path` scans,
     and the target passed to `schemas::operation` and `schemas::publish`. Some checks read fields
     that only normalization supplies: `schemas::operation` reads the target's `contract_scope`
     and `schema_environment`, and ACC-024's check reads the named Conduit's `local_name`
     (`access.rs:551`).
   - **From the State: every read of invocations.** These are ACC-039's serving exception
     (`access.rs:482-488`) and `routing`'s `reconcile.v1` branch (`:331-335`, ACC-033), which
     then looks the invocation's object up in the view.
   - **Helpers outside `access.rs` keep taking the transition's State.** One is
     `registration::root_consumed` (`registration.rs:1533`, called at `access.rs:508`, ACC-054).
     It compares only IDs, names and kinds through `access::object`, and normalization changes
     none of them, so its answer is the same.

   A State with only its objects filled in, passed where `evaluate_path` reads invocations, would
   silently change ACC-039 and ACC-033. The transition's State, passed where it reads objects,
   would give `schemas::operation` a target without `contract_scope` or `schema_environment`.
   Neither is allowed.

   **The `list.v1` branch in `records.rs` needs no view.** It reads only `kind`, `name`, `id`,
   `state`, `spec`, `owner` and `from`, and Conduits' `consumer` and `target` through
   `access::id`. Normalization changes none of these. So `access::object`, `may_inspect` and
   `may_inspect_packages` (`records.rs:282`, `:301`, `:319`) may read the view or the transition's
   State; reading the State saves the view. Its installed-packages loop (`records.rs:321`) reads
   the view today, and must read the State.
3. **Tool lists compared cheaply** (IFC-038). `access::tool_list_changes` keeps its signature
   and its results, and stays the call the shell makes. The shell keeps no tool list of its own.
   A consumer's list depends only on its open Conduits, in state order, and on each one's target
   object. Compare those values between the two states first. Compute the two lists, over one
   view per state, only for the consumers whose values differ, and notify when the lists differ.
   Where an open Conduit's target is absent, compute the lists as today, so that the same error
   results.
4. **`step` without a copy for the shell** (DL2's dispatch boundary; DL6).
   - `step(&State, ...)` keeps its signature, its contract that an error leaves the caller's state
     untouched (DL6, `engine/CONFORMANCE.md:23`), and `Transition`'s fields and serialized form.
     The vectors and experiment 011 call it.
   - Add an entry that takes the State by value and runs the same body with no copy. `step`
     becomes a copy followed by that entry, so the vectors check the code the shell runs.
   - On an error, the State given to the new entry is gone. The shell recovers it as step 6 says.
5. **The two trials, and registration's copies.**
   - Register-replacement at `requests.rs:920` and install at `:1005` each copy the whole
     transition to try a change and keep part of it. Copy only what the trial reads and writes,
     or check without changing anything. If either needs a change to registration's or
     installation's logic, keep its copy and report why, with its measured cost.
   - Leave registration's four copies (`records.rs:72-79` and `:110`, `registration.rs:1709` and
     `:2336`). Removing them changes registration's logic. Report them as one finding, with the
     `register-project` time step 1 prints, before and after. A register-replacement still copies
     the State in `replacement`'s Builder after you change its trial: say so in the finding.
6. **The shell keeps the committed state in memory** (DL2's storage row; IMP-007, REC-003,
   OPS-013).
   - **Parse once.** `Store::open` (`store.rs:109-142`) parses `state.json` once and hands the
     result on. `identity.rs` (`:66`, `:100`) and the first commit use it, not a new read.
   - **Readers.** The last commit, its State and its output times, lives in memory, under the
     writer mutex and the process lease. Readers get a shared snapshot of the last committed
     State, such as an `Arc<State>`, never a parse or a deep copy. The readers: `state()`,
     `state_directory`, `materialize_identity`, `client_token`, `verify_token`, `instance_exists`
     (`server.rs:59-67`), `unfinished()` and `preflight` (`server.rs:133-153`). `verify_token`
     runs on every request and keeps DL13's check of the instance's current existence
     (`engine/CONFORMANCE.md:33`): the snapshot it reads is the last commit.
   - **The commit** (`lib.rs:485-557`). It does not re-read the store (`:487`). Before the step it
     copies only what `tool_list_changes` and the consumer-ended notices read, which is the
     objects (`:491`). It runs the new entry of step 4, taking the snapshot when no reader holds
     it and copying it only when one does. `persist` (`:414-440`) serializes from references,
     with no copy into `Stored` (`:438`). `Stored`'s format does not change: the same version,
     fields, order and bytes.
   - **Memory advances only after the write, its fsync, the rename and the directory's fsync all
     succeed.** After any failure, in the step or in the write, the shell drops its in-memory
     copy, and the next access parses `state.json` again, as every access does today. Memory
     then equals disk even when the rename succeeded and the directory's fsync failed. Where the
     shell needs again a state that a step consumed, it parses `state.json` again, which equals
     the committed state: the retry with more evidence (`lib.rs:362`), and the second step that
     adds an archive (`:402`).
   - **Retention** (`lib.rs:442-483`) keeps DL10's two checks, before a request and after a
     transition (`CONFORMANCE.md:30`). Each reads the in-memory state, and writes only when it
     expires an output.
   - **The test trace** (`lib.rs:497-501`) stays test-only. It keeps a transition only while a
     test has turned it on; the tests that read it turn it on.
7. **Tests.**
   - **Memory equals disk after a failure.** In the shell's unit tests, a test-only fault makes a
     commit fail at each of: the temporary file's write; its fsync; the rename; the directory's
     fsync. A further case is a commit whose step returns an error; scripted evidence with too
     few IDs does that, and also takes the retry at `lib.rs:362`. After each failure, the shell's
     State and output times equal a fresh parse of `state.json`; then the next commit succeeds,
     and they equal it again.
   - **The fault belongs to one test's store.** `cargo test` runs the shell's unit tests as
     threads of one process, and `atomic_write` (`store.rs:60-81`) also writes client tokens and
     `engine-url.json`. So the fault is a `#[cfg(test)]` field of the test's own `Store` or
     `Writer`, and it matches only the write of `state.json`. Never a static, a thread-local
     shared across tests, or an environment variable.
   - **These tests need no serial group.** They kill no process and time nothing. Do not add them
     to one: `engine/.config/` is order 068's.
   - **A differential check of old against new.** The tool-list computation changes form, so:
     - add a cargo feature `differential` to `conduit-core`, and a feature of the same name to
       `conduit-conformance` and `conduit-shell` that turns it on. It is off by default, and no
       build without it compiles the check;
     - under it, the by-value entry of `step` (which `step(&State)` calls) keeps a copy of the
       State it was given, as the "before". It then compares, for every consumer of a Conduit in
       either state, each new list with the reference list on both states, and
       `tool_list_changes`'s result with the reference's. Any difference is an error that names
       the consumer. An error on one side and not on the other is a difference;
     - the reference is BASE's computation, unchanged in substance, in its own module: a
       whole-state publication copy per publish (BASE's `access.rs:93-112`), each consumer's
       whole list (`:134-156`), and the lists compared for every consumer (`:184-199`);
     - the reference keeps its own copies of BASE's `publication_state` and of the Conduit and
       target lookup at the head of BASE's `schemas::publish` (`schemas.rs:994-1024`). It may
       share only the unchanged rest of `publish`, which builds a tool from a normalized Conduit
       and target. A reference that shares the new view's code cannot catch a fault in
       normalization;
     - it runs over every transition of the vectors, of the shell's unit tests (S1 whole through
       the shell) and of `agents.rs` (`engine/examples/s1` through the shell);
     - show once that it can fail: plant a fault in the new comparison, run it, see it fail,
       remove the fault, and report the failing line.
   - **Existing tests.** Every existing test and vector passes, and none checks less than before.
     Rename or move no existing test: order 068's `engine/.config/nextest.toml` names tests by
     their paths, the live test of step 1 and the crash tests in `src/tests.rs` among them, and a
     renamed test silently leaves its group. Change an existing test only:
     - to factor the live test's set-up into helpers (step 1);
     - to turn the trace on where a test reads it: `established()` (`tests.rs:216`, `:231`) and
       `s1_whole_through_store_loop_and_scripted_executor` (`:254`, `:288`);
     - where the state accessor's new type requires it.
   - Each changed function says in a comment which promise or DL row it keeps.
8. **The DL2 storage row** (`engine/CONFORMANCE.md:18`). It must say:
   - format version 1 is unchanged;
   - `state.json` is parsed once, when the store opens;
   - the committed State is kept in memory under the mutex and the lease, and readers see the
     last commit;
   - each transition is written to a new file, which is fsynced and renamed over the old one,
     and the directory is fsynced, before memory advances;
   - after a failure, the file is read again.

   Keep the row's other facts: the lease, the modes, unknown formats refused, the store's parent.
9. **Measure after**, on the same base as step 1 (041b's checkpoint), with the same build,
   untimed runs, timed runs and variables. After the rebase onto `main`, do not redo the table,
   unless you redo both columns on the new base: order 071 makes the vectors run in parallel,
   and a column measured after it would credit this order with 071's gain. In the report's
   `summary`, give a table, in numbers only, with no local path or log name:
   - each run's wall time before and after;
   - the latency test's medians and maxima, and the `register-project` time, before and after;
   - `state.json`'s size, its objects' bytes and its package bytes;
   - `CARGO_BUILD_JOBS`, `RUST_TEST_THREADS` and the count of logical processors.

   List every timed run in `checks` too. In the round after the rebase, also report the wall time
   of `cargo nextest run --release` if it runs (see "Done when"); order 068's report holds the
   gate's time before this order.
10. **Declare no vector change.** Write `docs/work-orders/070-declared.yaml`: a comment naming
    this order, then an empty list, `[]`.

## Scope

Change:

- in `engine/crates/core/src/`: `lib.rs`, `access.rs`, `schemas.rs`, `lifecycle.rs`,
  `records.rs`, `requests.rs`, and one new module for the reference under `differential`;
  `registration.rs` and `archives.rs` only where a trial of step 5 needs it, never to remove
  registration's four copies;
- in `engine/crates/shell/src/`: `lib.rs`, `store.rs`, `server.rs`, `identity.rs`, the unit tests
  `tests.rs` and `server_tests.rs`, and `main.rs` and `runner.rs` only for the state accessor's
  type;
- in `engine/crates/shell/tests/`: `agents.rs`, for the latency test and its helpers; the other
  files only for the state accessor's type;
- the `[features]` tables of `engine/crates/core/Cargo.toml`, `engine/crates/shell/Cargo.toml`
  and `engine/crates/conformance/Cargo.toml`, for `differential` only;
- `engine/CONFORMANCE.md`: the DL2 storage row only;
- `docs/work-orders/070-declared.yaml`.

Do not change:

- `spec/`, or anything under `conformance/`: no case, vector, expectation, fixture or manifest
  entry;
- `engine/crates/conformance/src/`: order 071 (S0a) rewrites the runner, and this order needs
  nothing there;
- `engine/crates/loader/`, `packages/` or `experiments/`;
- `engine/.config/`, `.github/`, or the barrier at `engine/crates/shell/src/tests.rs:1197-1204`,
  which are order 068's;
- the name or module of any existing test;
- the store's format: its version, its fields, and the bytes it writes for a state. Package files
  stay in `state.json`. Keeping them by digest outside it would be a format version 2, for a
  separate order: report what step 1 measured about them, and change nothing;
- any crate's dependencies;
- any record: the increment log, `docs/implementation/WORK_ORDERS.md`, `docs/AUDIT.md`.

Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/check_private_paths.py`, if your tree has it
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py BASE --declared docs/work-orders/070-declared.yaml`, whose last
  line reads `Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.` Before the rebase, BASE
  is the commit the admin named at dispatch. After it, BASE is
  `$(git merge-base HEAD origin/main)`
- `git diff --check`

In `engine/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo clippy -p conduit-core --features differential --all-targets -- -D warnings`
- `cargo clippy -p conduit-shell --features differential --all-targets -- -D warnings`
- `cargo test --release`
- `cargo nextest run --release`, if your tree has `engine/.config/nextest.toml`: the gate, with
  no retries, so a flaky result fails it
- `cargo test --release -p conduit-conformance --features differential --test vectors`
- `cargo test --release -p conduit-shell --features differential --lib --test agents`
- the memory-equals-disk tests, by name, among the passes

In `experiments/011-short-form-spike/`: `cargo test --release`.

Report the totals passed and failed, and the table of step 9. In `own_choices`, say what form you
gave the view, whether it normalizes lazily, the shared snapshot, the new entry of `step`, and the
fault's field. Tests that bind loopback ports may fail in your sandbox: say so, and the admin
reruns them. If the crash tests fail at their barrier under load, rerun them with
`RUST_TEST_THREADS=2` and say so. Do not stage anything.
