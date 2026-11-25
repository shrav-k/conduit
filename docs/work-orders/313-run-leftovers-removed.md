# Work order 313: a run's directories go when the run ends, and the store's open sweeps run leftovers and orphaned temporary files (DEC-148, DEC-149)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network, and every crate it builds is
cached: run cargo offline, and if a crate is missing, stop and report, and fetch nothing. Run one
cargo command at a time, and not `tools/gate.py` or the whole engine suite. Never print a whole
large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Run no `conduit` outside a test, no
`codex` or `claude`, and sign in nowhere: every implementation a test meets is scripted. Open,
list or sweep no store but one a test makes in a temporary directory: never the dogfood's. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute
local path, user, machine or account name, or private repository name.

## What and why

**The owner's answers** ([DEC-148](../decisions/log.md#dec-148-2026-11-23-package-files-stored-once-by-digest-run-leftovers-removed)):
run leftovers -> "Remove at run end (Recommended)", which lands as "run directories are removed
when the run ends and swept when the store opens". [DEC-149](../decisions/log.md#dec-149-2026-11-23-the-store-is-slimmed-now):
"Yes, order them now (Recommended)", three engine orders; this is the third. The coordinator's
store study (its R4 and order 4) is the design; this order carries what it needs.

**Today**, on `main` with the log through DEC-149, each run the shell starts leaves up to four
entries in the store, each named by its own random UUID, and nothing removes them: `runs/<uuid>`,
the identity directory `Authority::materialize` makes for every `run.v1` intent, the built-in
runner's and a runtime provider's (`lib.rs`, `materialize_identity` and
`materialize_provider_identity`); `work/<uuid>`, the run's working directory (`Inner::start`);
`run-groups/<uuid>`, the engine and group PIDs (`Inner::start`, and `start_bridge` for a bridge
to a provider's run); and `programs/<uuid>`, a fresh copy of an installed package for a run of
one of its relative programs (`materialize_package`). `http-bridges/<uuid>` goes when its
`BridgeDirectory` drops, so only a killed engine leaves one. `atomic_write_inner` (`store.rs`)
writes `.write-<uuid>` and renames it; a kill between the two leaves the file. A long-lived store
measured on 2026-11-23 held 102 entries in each of `runs/`, `work/`, `run-groups/` and
`programs/`, 524 MB, 522 MB of it in `programs/`. Each `runs/` entry holds its instance's token,
valid for 100 years (DL13): a credential copy that nothing reads. `instances/<id>` (PRV-029) goes
with its instance (`clean_state_directories`) and is not this order's.

**Why the open may sweep everything a run left.** At an open no run of the store lives. The
lease admits one shell. A shell's drop stops every process group it registered
(`impl Drop for Shell`), and a group's guard kills the whole group when the engine's end of its
lifetime pipe closes, the engine killed included (`ProcessGroup`, `guard_main`). A provider's
run, which may outlive the engine, keeps its own identity copy: `provider-local-process` and
`provider-ssh-runtime` set `CONDUIT_IDENTITY_DIR` to their own directory, and PRV-022 has a
provider keep those files as it keeps a secret's value. A later bridge to that run gets fresh
files (`bridge_process`).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6, the
removal and the sweep start nothing, stop nothing and send nothing again, and no run is ended in
order to clean it; 7, no record, reply or attribution changes, and no record ever named these
entries. None at risk: no promise names them (DL13 and DL16 are the engine's rows), and
`instances/` is untouched (PRV-029, LIF-035).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes: DEC-148 asks for it, and existing practice gives the form. systemd removes a unit's
`RuntimeDirectory=` when the unit stops; the kubelet removes the directories of pods that no
longer exist; git's prune removes the temporary files an interrupted write leaves. Not taken:
keeping leftovers a while for diagnosis (the owner chose removal at run end); a sweep at every
commit, as `instances/` has (runs live then, while at an open none does); `client-tokens/`, one
file per instance, not per run.

**When it runs.** On `main` with the decision log through DEC-149, or on `main` once the log
merges. Through Conduit: one Codex worker turn, the blind Codex verifier and Claude reader at
once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what
must hold; how is yours. The coordinator's, open to the owner's strike (DEC-029): the test names,
the open as the sweep's one moment, the UUID-name rule, `http-bridges/` and (with order 312)
`packages/`'s `.write-` files in the sweep, and the removal of a provider run's identity copy
once its call holds the files. It fits one turn; were it not to, `programs/` goes second
(decision 5, test 1's planted copy, `a_failed_start_leaves_none_of_its_entries`, and in DL16's
row the words on a `programs` copy and that test's name), and the rest first.

**Shared files.** Order 311 (the held copy dropped) edits `engine/CONFORMANCE.md`'s row
`DL2: storage`, the table line and evidence cell decision 9 edits, and adds a `mod` line to the
shell's `src/tests.rs`; order 312 (package files by digest) edits that row too, and `store.rs`
(the open, where it migrates format version 1), `lib.rs` and `runner.rs` (`materialize_package`,
`command`, `start`). The later to merge rebases, keeps every insertion and evidence addition in a
shared row, and reruns `promise_standing.py --write` if `engine/STANDING.md` changes.

**Stop rule** (the coordinator's D1). Stop, change nothing further, and report only for a
contradiction between promises (give both), a step that needs a change outside Scope, or a file
outside Scope. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, beside any choice no decision governs.

## The decisions

Read first PRV-022, PRV-029 and LIF-035; DL2, DL13 and DL16 in `spec/implementation-defined.yaml`
with their rows in `engine/CONFORMANCE.md` and its paragraph on `store/run-groups`; and each
function the decisions and tests name.

1. **A run's entries.** A run is one process group the shell's runner starts: a built-in run of
   `run.v1` (`Inner::start`) or a bridge to a provider's run (`start_bridge`). A built-in run's
   entries are those made for it alone: the `runs/` identity directory materialized for its
   `run.v1` intent, its `work/` directory, its `run-groups/` file and the `programs/` copy
   `materialize_package` made for its command (decision 5). A bridge's only entry is its
   `run-groups/` file: it starts in the store's own directory, its `http-bridges/` directory goes
   as today when its `BridgeDirectory` drops, and the provider run's `runs/` copy is no run's
   (decision 3). Each run knows its entries' paths; whether the names stay independent or share
   one UUID is yours. Where each entry lives, its mode (0700 for a directory, 0600 for a file)
   and what a run's process finds in it stay as they are.
2. **Removed when the run ends** (DEC-148). A run's entries are removed once its process group has
   been stopped (`ProcessGroup::stop`, which kills the whole group) and its guard reaped, or its
   group is seen absent (`is_absent`); never earlier, so no entry goes while a process of its
   group may run. The task that reaps the guard removes them, whatever `shutdown` sees;
   `shutdown` also removes those of each run `await_stops` sees absent. Entries neither has
   removed when the runner's runtime drops stay for the next open (decision 4). An entry already
   gone counts as removed. This covers every way a run ends in the engine: `stop.v1`, a new run
   for the same subject (the old run's entries go, the new run's stay), `cancel_setup_parts`, a
   retirement or removal that stops the run, a bridge that fails to start or prove, and
   `shutdown`. A run whose program exits on its own while its guard lives ends when the engine
   stops its group, as today: this order adds no stop and observes no new end. A start that fails
   or is cancelled after making an entry removes it, under this rule, before it reports; an
   identity materialized for an intent that no run takes is removed on that path. Removal never
   follows a link (it removes the link) and never reads an entry's contents. A removal that fails
   changes no result, reply, record or state, and the next open retries it: in the runner it adds
   one runner diagnostic naming the directory (`runs`, `work`, `run-groups` or `programs`) and the
   cause, never the entry; in `lib.rs`, where no runner is at hand, one line of that form goes to
   standard error, as the sweep's does.
3. **A provider's run's identity copy** (PRV-022). For a provider call to `run.v1`
   (`materialize_provider_identity`), the call's inputs carry the identity's files, and the
   provider materializes its own copy. The engine's `runs/` entry for it is removed once
   `provider_call` has returned, whatever it returned: no run of the engine owns it.
4. **The sweep at open** (DEC-148). Once per open of a store, while the open holds the lease,
   after the state file has been read and accepted, and before the open's recover commit, the
   shell removes:
   - each entry of `runs/`, `work/`, `run-groups/`, `http-bridges/` and `programs/` whose name is
     a UUID (decision 5);
   - each file or link named `.write-` followed by a UUID in the store's directory,
     `client-tokens/` and `run-groups/`, the three that `atomic_write` writes into and keeps, and
     in `packages/` once order 312 has merged (a `runs/` or `http-bridges/` entry goes whole).

   A name is a UUID here when `Uuid::parse_str` reads it and it equals that UUID's hyphenated
   lower-case form, the form `Uuid::new_v4().to_string()` writes. The sweep removes nothing else:
   no entry under another name, nothing in `instances/` (`clean_state_directories` keeps its
   rule), `npm/`, `npm-downloads/`, `archive-catalog/` or, but for those `.write-` files,
   `packages/`, and no other file. It creates no directory: a missing one is skipped. It never
   follows a link, never reads an entry's contents, makes no commit and writes no record. A
   removal that fails is left for the next open and the open goes on, after one line on standard
   error naming the directory and the cause, never the entry. An open the store refuses before
   the sweep (another shell's lease, an unknown format, a broader mode) removes nothing; one refused
   later, under the lease (`Shell::open`'s checks of the addresses, the authority or the trust
   domain), may follow the sweep, harmlessly: no run of the store lives. `Runner::new` does not
   sweep: a second runner on the same shell never removes a live run's entries.
5. **`programs/` and order 312.** The start check's last line says whether order 312 has merged
   (`1`) or not (`0`). Either way a run's `programs/<uuid>` is one of its entries, and the sweep
   removes those earlier engines left. Where 312 has merged, read its order first and rebase onto
   it: the archive kept at `packages/<sha256>`, from which the copy is made, is no run's, and
   nothing here removes anything in `packages/` but decision 4's `.write-` files; whether the
   sweep precedes 312's migration at open is yours; `a_failed_start_leaves_none_of_its_entries`
   builds its package as 312's tests build one kept by digest, and also asserts that
   `packages/<sha256>` stays; and if the merged 312 removes names in `programs/`, test 1 asserts
   only that none named by a UUID stays.
6. **Nothing else changes** (DEC-148). No outcome, reply, record, record ID, condition, status or
   state member changes, and no inspection shows any of this; no run starts, stops or restarts
   for this order. `clean_state_directories`, `instances/`, `client-tokens/`, the npm caches,
   `commit-measures.jsonl` and every promise stay as they are. The comment above the
   `run-groups/` write keeps its rule (a persisted PID is never used to inspect or kill a process
   after reopening) and drops "Historical".
7. **One way** (the least change). The run keeps its entries' paths beside its group. The task in
   `Inner::start` that reaps the guard removes them after its `child.wait()` and `group.stop()`,
   the task in `start_bridge` after its `child.wait()`, and `shutdown` before the runner's runtime
   drops. Where a start's exit stops its group without reaping the guard (`Inner::start`'s and
   `start_bridge`'s "setup cancelled", and `Inner::start`'s failed `run-groups` write, whose group
   only `ProcessGroup`'s drop stops), it stops the group and waits for the guard first. The intent
   loop in `lib.rs` hands a built-in run's identity directory (`RunIdentity::directory`) to
   `builtin_runner` with the identity, and removes a provider run's after `provider_call`
   returns. The sweep is one function in `store.rs` that `Store::open` calls after `load`. No new
   crate or dependency.
8. **Tests.** Write them first, run them on the otherwise unchanged code and report each failing
   line. No other test's name, a child fixture's included, contains a new test's name. A test
   reads a path from a run's files, never a token or a key, and opens no file inside a `runs/`
   entry. A wait on the unchanged code ends at its deadline; bound each new wait at 60 seconds.
   - In `engine/crates/shell/tests/runner.rs`, beside
     `killed_engine_ends_run_and_descendant_and_does_not_pass_token`:
     - `a_killed_run_leaves_nothing_after_the_next_open`: start `engine_child`, or a child of its
       form, and wait for `ready`; `runs/`, `work/` and `run-groups/` each hold an entry. Kill the
       child and wait until its run's processes are gone, as that test does. Plant what an earlier
       engine would leave: `programs/<uuid>/bin/worker`, `http-bridges/<uuid>/`; plant
       `programs/kept/`; a `work/<uuid>` that is a link to a directory outside the store holding
       a file; a `runs/<uuid>/` holding a link to a file outside the store; and two `work/`
       entries named by the same UUID in upper case and in simple form (no hyphens). Note the
       names in `instances/`. Open the store again: `runs/`, `work/`, `run-groups/` and
       `http-bridges/` hold no entry named by a UUID, and `work/` holds the two other names;
       `programs/` holds only `kept`; both outside files are as planted; `instances/` holds the
       same names. Today `runs/` keeps its entry.
     - `a_running_runs_directories_are_never_removed`: one shell and runner set up `shared` and
       `history`, as `setup` does. For each run, take its `work/` entry from its `cwd.json` and
       its `runs/` entry from `CONDUIT_IDENTITY_DIR` in its `environment.json`, the path only.
       While both run, commits (a call through `shared` and an `inspect.v1`) and a second
       `Runner::new` on the same shell remove no entry. Send `stop.v1` for `history` through
       `builtin_runner`, as the runner tests send `inspect-run.v1`, and wait, with `until_for`,
       until `history`'s `work/` and `runs/` entries are gone and `run-groups/` holds one entry
       fewer; `shared`'s entries stay and its `inspect-run.v1` reads `running`. Drop the runner
       with the shell still open: `runs/`, `work/` and `run-groups/` then hold no entry. Today
       `history`'s entries stay.
   - In `engine/crates/shell/tests/provider_calls.rs`, on its `Fixture` (an https address set):
     - `a_provider_runs_identity_copy_and_bridge_entry_are_removed`: with `Fixture::new("success")`,
       note the names in `runs/` and `run-groups/`, those of the provider server's own run. Set up
       the Agent (`setup`), which succeeds. While the provider's run lives, `runs/` holds the
       noted names alone, and `run-groups/` holds them and one more, the bridge's. Retire the
       Agent, as `runtime_provider_retirement_stops_bridge_and_keeps_provider_stop_outcome` does,
       and wait until `run-groups/` holds the noted names alone; `runs/` still holds them. Today
       `runs/` gains an entry at the setup.
   - In `engine/crates/shell/src/tests.rs`, beside `crash_child`:
     - `a_killed_engine_mid_write_leaves_no_orphan_after_open`: a child of `crash_child`'s form
       opens an established store; one of its state writes stops after its `.write-` file is
       written and fsynced, before the rename (a test-only stage beside `WriteFault`; how is
       yours), and tells the test so. The test sees one `.write-` file named by a UUID in the
       store's directory and kills the child. It plants `client-tokens/.write-<uuid>`,
       `run-groups/.write-<uuid>`, `.write-kept` in the store's directory, and
       `instances/<id>/.write-<uuid>` for an instance of the store. The next open succeeds; its
       state holds nothing of the child's input; the three directories hold no `.write-` file
       named by a UUID; `.write-kept` and the instance's file stay. Then, while a shell holds the
       store, a planted `.write-<uuid>` survives a second open, refused as today ("store already
       has an active shell"), and is gone once the first shell drops and the store opens again.
       Today the child's orphan survives the open.
   - In `engine/crates/shell/src/runner.rs`'s `mod tests`:
     - `a_failed_start_leaves_none_of_its_entries`: a start through `Inner::start` whose subject's
       command is an installed package's relative program, and which fails after
       `materialize_package` has copied it (a runner whose guard program does not exist, as
       `relative_command_uses_installed_bytes_and_keeps_their_mode` makes one, or another failure
       after the copy), returns its error and leaves no `work/` or `programs/` entry. Its state
       is committed before the shell opens (the test-only `Store::write`), built as
       `installed_command_state` builds one around an instance ID, or as decision 5 says once
       312 has merged; how is yours. Today the `work/` and `programs/` entries stay.
   - An existing test changes only where it counted or read an entry of a run that had ended, or
     listed one that a run's end may now remove before it reads it. Report each, and why.
9. **The document** (`engine/CONFORMANCE.md`). Wrap each insertion at its paragraph's width.
   - In the row "DL2: storage", after "After a failure, the file is read again.", add: "From
     order 313, the store's open, under the lease and before its recover commit, removes each
     `.write-` file named by a UUID in the store's directory, `client-tokens/` and
     `run-groups/`, which a write killed before its rename leaves (DEC-148)." Add
     `` `a_killed_engine_mid_write_leaves_no_orphan_after_open` `` to its evidence cell. Where
     312 has merged, name `packages/` too.
   - In the row "DL13: instance proof", after "under the names the run contract supplies.", add:
     "From order 313, that directory is removed when its run ends, or for a provider's run once
     the provider call holds its files, and the store's open removes any left (DEC-148)." Add
     `` `a_provider_runs_identity_copy_and_bridge_entry_are_removed` `` to its evidence cell.
   - In the row "DL16: environment and working directory", after "separate from shared data and
     identity files.", add: "From order 313, it is removed when the run's process group ends,
     never while the group may run, with the run's `run-groups` diagnostic and any `programs`
     copy made for that run alone, and the store's open removes those of runs that ended with an
     earlier engine (DEC-148)." Add `` `a_killed_run_leaves_nothing_after_the_next_open` ``,
     `` `a_running_runs_directories_are_never_removed` `` and
     `` `a_failed_start_leaves_none_of_its_entries` `` to its evidence cell.
   - In the paragraph on `store/run-groups`, change "private historical engine/group PID
     diagnostics" to "one private engine/group PID diagnostic per run", and after "never used to
     recover or terminate a process after restart." add: "From order 313, each is removed when
     its run's group ends, and the store's open removes those left (DEC-148)."

   Then run `python3 tools/promise_standing.py --check`, and `--write` if it fails because a
   promise ID entered the shell's sources.

## Start check

From the repository root. Any line that prints otherwise means stop, change nothing and report it.

```sh
k() { f=$1; shift; for p in "$@"; do tr -s ' \n' '  ' < "$f" | grep -oF -- "$p" | wc -l | tr -d ' '; done | paste -sd' ' -; }
d=engine/crates/shell/src
grep -c -e '^## DEC-148, ' -e '^## DEC-149, ' docs/decisions/log.md  # 2
for n in 148 149; do sed -n "/^## DEC-$n, /,/^## DEC-/p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1
grep -c -e '"runs"' -e '"work"' -e '"run-groups"' -e '"http-bridges"' $d/store.rs  # 0
grep -cF 'let temporary = directory.join(format!(".write-{}", Uuid::new_v4()));' $d/store.rs  # 1
grep -c 'fn engine_child()' engine/crates/shell/tests/runner.rs  # 1
grep -c 'fn crash_child()' $d/tests.rs  # 1
k engine/crates/shell/tests/provider_calls.rs 'struct Fixture {' 'fn runtime_provider_retirement_stops_bridge_and_keeps_provider_stop_outcome()'  # 1 1
grep -rlE 'fn (a_killed_run_leaves_nothing_after_the_next_open|a_running_runs_directories_are_never_removed|a_provider_runs_identity_copy_and_bridge_entry_are_removed|a_killed_engine_mid_write_leaves_no_orphan_after_open|a_failed_start_leaves_none_of_its_entries)\(' engine/crates/shell | wc -l | tr -d ' '  # 0
k engine/CONFORMANCE.md 'After a failure, the file is read again.' 'under the names the run contract supplies.' 'separate from shared data and identity files.' 'private historical engine/group PID diagnostics' 'never used to recover or terminate a process after restart.' 'From order 313,'  # 1 1 1 1 1 0
ls docs/work-orders | grep -c '^312-.*\.md$'  # 0, or 1 once order 312 has merged (decision 5)
```

## Scope

Change, in `engine/crates/shell/src/`: `runner.rs` (decisions 1, 2, 5 and 7 and its test),
`store.rs` (the sweep and the test-only write stage), `lib.rs` (the intent loop's identities),
`identity.rs` (only if `RunIdentity` must carry its directory's ownership) and `tests.rs` (its
test); `engine/crates/shell/tests/runner.rs` (its two tests) and `tests/provider_calls.rs` (its
test); `engine/CONFORMANCE.md` (decision 9); `engine/STANDING.md` only by the tool. An existing
test changes only under decision 8, in `tests/serve.rs`, `tests/provider_calls.rs`,
`tests/local_process.rs`, `tests/http_bridge.rs`, `tests/agents.rs`, `src/server_tests.rs` or
`src/https_tests.rs`. Change nothing else: not the core, the loader, the conformance crate,
`spec/`, a case, `docs/` (DEC-148 and DEC-149 stay not done), `packages/`, `tools/`, a
`Cargo.toml` or `Cargo.lock`. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after. The second group are guards, which pass
today and must still pass. Run each existing test you changed by name, once, too. Run the shell's
`--lib` line and the six `--test` lines once each, after the named ones. Do not run the whole
engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in a_killed_run_leaves_nothing_after_the_next_open a_running_runs_directories_are_never_removed; do (cd engine && cargo test --offline --release -p conduit-shell --test runner "$t" 2>&1 | grep -c "$ok"); done  # 1 twice; today 0
(cd engine && cargo test --offline --release -p conduit-shell --test provider_calls a_provider_runs_identity_copy_and_bridge_entry_are_removed 2>&1 | grep -c "$ok")  # 1; today 0
for t in a_killed_engine_mid_write_leaves_no_orphan_after_open a_failed_start_leaves_none_of_its_entries; do (cd engine && cargo test --offline --release -p conduit-shell --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 twice; today 0
k() { f=$1; shift; for p in "$@"; do tr -s ' \n' '  ' < "$f" | grep -oF -- "$p" | wc -l | tr -d ' '; done | paste -sd' ' -; }
k engine/CONFORMANCE.md 'From order 313,' 'private historical engine/group PID diagnostics' 'one private engine/group PID diagnostic per run'  # 4 0 1; today 0 1 0
k engine/CONFORMANCE.md a_killed_engine_mid_write_leaves_no_orphan_after_open a_provider_runs_identity_copy_and_bridge_entry_are_removed a_killed_run_leaves_nothing_after_the_next_open a_running_runs_directories_are_never_removed a_failed_start_leaves_none_of_its_entries  # 1 1 1 1 1; today 0 0 0 0 0
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p conduit-shell --lib "$1" 2>&1 | grep -c "^test result: ok\. $2 passed"); }
s killed_after_ 2  # 1: the killed-child tests
s memory_equals_disk_after 5  # 1: the write-fault tests
s private_state_directory_is_empty_distinct_and_stable_after_restart 1  # 1: PRV-029's directories stay
s store_rejects_versions_permissions_and_competing_owners 1  # 1: a refused open is still refused
s https_provider_without_address_has_no_identity_but_has_helper 1  # 1: the sweep makes no directory
s relative_command 2  # 1: installed relative programs still run from their copy
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
n=$(git ls-files -o --exclude-standard -- engine); [ -z "$n" ] || git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/shell/src/(runner|store|lib|identity|tests|server_tests|https_tests)\.rs|engine/crates/shell/tests/(runner|serve|provider_calls|local_process|http_bridge|agents)\.rs|engine/(CONFORMANCE|STANDING)\.md)$' | wc -l | tr -d ' '  # 0
[ -z "$n" ] || git reset -q -- $n  # stage nothing
(cd engine && cargo test --offline --release -p conduit-shell --lib 2>&1 | grep -c '^test result: ok\.')  # 1: the shell's own tests
for t in runner provider_calls local_process http_bridge serve agents; do (cd engine && cargo test --offline --release -p conduit-shell --test "$t" 2>&1 | grep -c '^test result: ok\.'); done  # 1 six times: the tests that start runs or read their directories
```

Your report gives, per decision, what changed and where: each new test's failing line on the
unchanged code, then its passing line; which way decision 5 went, and why; each existing test you
changed, and why; your own choices; and every check's last line. Stage nothing.

## What follows

- **Orders 311 and 312**, if not yet merged; 312's next order keeps a program copy per digest.
- **The measure.** Once the three have merged, the coordinator measures a store's shape again, as
  on 2026-11-23. The dogfood's store (experiment 028) sheds its leftovers at its first open under
  an engine built with this order, about 524 MB as measured.
- **The rest of the store's chain:** every reader through the engine, then the state chain
  (DEC-138 to DEC-142 and DEC-144), with DEC-143's gate for the dogfood's store.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main`. Built and read on the team
seats (DEC-179, DEC-180).

**Worker-4's turn** (09:39 to 10:17), on the Business seat, built the removal at a run's end and the
sweep at open, with five regressions.

**Two blind reads.** The Codex verifier, `root-verifier-4.turn.v1`, found a safety must: cleanup
could follow a replaced ancestor symlink and delete entries outside the store. The Claude reader,
`root-reader-a.turn.v1`, had a should and three nits. One fix turn by worker-4 (10:57 to 11:25)
made cleanup work through directory descriptors that never follow a link, with a test that fails
on the earlier code, and applied the rest. The gate's run on the squash is the coordinator's, at
the merge.
