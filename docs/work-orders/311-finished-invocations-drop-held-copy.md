# Work order 311: a finished invocation drops its held copy of the target and keeps its contract (DEC-149, the store's first slimming)

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
copy or read no store but one a test makes in a temporary directory: never the dogfood's. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute
local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-149](../decisions/log.md#dec-149-2026-11-23-the-store-is-slimmed-now):
whether to slim today's store before the state chain -> "Yes, order them now (Recommended)",
landing in three engine orders now, the first "a finished invocation drops its held copy of the
target (its contract stays, which inspection reads)".
[DEC-145](../decisions/log.md#dec-145-2026-11-23-finished-requests-are-remembered-for-the-stores-life-for-now):
"Forever for now": OPS-008 and OPS-009 stand, and each finished invocation's slimmed entry is kept
for the store's life. This order is R1 of the coordinator's store study, which is not in the tree.

**Today.** Measured on the dogfood store on 2026-11-23, by names and sizes alone: `state.json` is
707 MB, 580 MB of it in 1,007 invocation entries. The one entry sampled, a call to a core target,
is 195 KB, 190 KB of it `held_target`, nearly all the core's schema environment. Every commit
rewrites all of it; the last 200 took a median 1.7 s to write. In the core, `invoke_inner` and
`dispatch_hook` (`requests.rs`) put `held_target`, the target's publication view, and
`implementation` into each entry, and nothing removes them. Both are read while the entry is in
progress or at recovery. After the end, only `record_content` (`requests.rs`) reads `held_target`,
for each record that names the invocation, to keep the inputs or outputs the held Type's
`records` keeps (CTR-004), or `lift.v1`'s inputs; no core Type states `records`, and the shell
reads neither field. `inspect.v1` builds `status.settings` from every entry's
`contract.spec.changes` (`records.rs`). `restore_or_remove_invocation` restores an entry from
`previous` but leaves the new attempt's two fields.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing repeats an effect implicitly: the index entry stays, so a repeat after the end is answered
from it or reports conflict (OPS-009); 7, everything is attributable: every record holds what it
holds today, and inspection still names the control that left a setting unknown (REV-010). Both
are at risk were a field read after the end dropped; decisions 1 and 2 keep each such field.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: Kubernetes' server-side apply calls the kept copy of what was last applied
(`last-applied-configuration`) legacy. Not taken: dropping `contract`, which inspection reads;
keeping only a `changes` map in its place (later, once environments are kept by digest); a window
after which a repeat is a new request (DEC-145); dropping an unreconciled unknown entry's copy, or
one a later record redacts with (decision 2).

**When it runs.** On the decision log's branch that holds DEC-148 and DEC-149, or on `main` once it
merges. Through Conduit: one Codex worker turn, the blind Codex verifier and Claude reader at once,
one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what must
hold; how is yours. The coordinator's, open to the owner's strike (DEC-029): that `implementation`
is dropped with the held copy, decision 2's exception, the test names, the measure and the rows'
words.

**Shared files.** DEC-149's two other orders change the shell's store and runner,
`engine/CONFORMANCE.md` and `engine/STANDING.md`, and other engine orders change core
`requests.rs` and `records.rs`: whichever merges later rebases and reruns
`promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first OPS-008, OPS-009, OPS-012, OPS-014, OPS-024, OPS-034, REV-007, REV-009, REV-010,
REC-002, CTR-004 and IMP-007, and DL2 and DL10 with their rows in `engine/CONFORMANCE.md`; in the
core, the functions named under Today and in decision 3, and the control tests of `requests.rs`
from `settings_fixture` to `later_control_settles_unknown_setting`.

1. **What an ended entry keeps** (OPS-008, OPS-009, REV-009, REV-010, DEC-145). An entry has ended
   when its state is neither `in-progress` nor `unknown`, or a reconciliation settled it (OPS-014).
   An ended entry holds no `implementation`, and no `held_target` but under decision 2. Every
   other field stays as today, `contract`, `retained`, `cause` and the control's fields among them.
   An entry in progress, or unknown and not yet reconciled, keeps both. A `previous` snapshot holds
   what its entry held.
2. **The records exception** (REC-002, CTR-004). An ended entry keeps `held_target`, and drops
   `implementation` all the same, where a later record of it may hold content: the held Type's
   `records` states `inputs: true` or `outputs: true`, or the operation is `lift.v1`. One function
   says so, used by this decision and by `record_content`'s own check, so the two cannot drift.
   It fails nothing: where it cannot read the held Type, it answers that `held_target` stays. It
   runs at each open for each ended entry still holding `held_target`, so keep it cheap. Where an
   ended entry holds no `held_target`, `record_content` keeps no inputs or outputs for its
   records, as today for a Type that keeps neither, so each such record is unchanged.
3. **When the two are dropped** (DL2). The transition that ends an entry, reconciles it, or
   restores it from `previous` to an ended state drops them after its last record for that entry,
   so every record it writes is as today. Today an entry ends in `finish`, in the reconciliation
   within `invoke_inner`, in `restore_or_remove_invocation`, at the end of a read of a secret or of
   a provider's restart in `secrets.rs`, and in the register-project entry, written ended. Name in
   your report each path you changed. Any other transition changes no entry beyond what it changes
   today: `held_copy_repeat_after_the_end_answers_as_before` checks it for repeats.
4. **Compaction at open** (DL2, DL10, IMP-007). The recover transition, made at each open, also
   drops the two fields, under decisions 1 and 2, from every ended entry, after it has ended
   interrupted work as today. Compaction never fails the recover: an entry whose held Type cannot
   be read keeps `held_target`, and the open goes on. A store an earlier engine wrote is compacted
   at its first open; a second recover changes no entry. Its decision names DL10 beside DL2; its
   reply, records and what it ends unknown are as today. The format version stays 1.
5. **Nothing a caller sees changes** (OPS-008, OPS-009, OPS-024, OPS-034, REV-009, REV-010,
   REC-002). Every reply, record, record ID and inspection is as today; no retained output, record
   or allocated ID is added or removed; no shell source but the tests changes.
6. **`engine/CONFORMANCE.md` and `engine/README.md`,** at their width.
   - In the row `DL2: storage`, after "the next line appended carries `unwritten`.", add: "From
     order 311, an invocation that has ended, an unknown one once reconciled, holds no
     `implementation` and no `held_target`, the copy of its target taken at dispatch: the
     transition that ends, reconciles or restores it drops both after its last record of it, and
     `held_target` stays only where a later record of it may keep inputs or outputs (CTR-004,
     REC-002). Its `contract` and the state OPS-008 names stay. The recover transition at each open
     drops both from entries an earlier engine wrote, so a store is compacted at its first open
     after the change; the format version stays 1 (DEC-149)." At the end of its
     evidence cell add
     `` , `held_copy_compacted_when_a_store_opens` in `its tests` (in `engine/crates/shell/src/held_copy_tests.rs`) ``.
   - In the row `DL10: retained results`, "No key expires and invocation state is never
     compacted." becomes "No key expires. From order 311, an invocation that has ended drops its
     held copy of the target and its implementation, as the row `DL2: storage` says; nothing else
     in invocation state is compacted, so its index entry, its `contract`, which inspection reads
     (REV-009, REV-010), and its retained results, which expire only as above, stay, and repeats
     are answered as before (OPS-008, OPS-009, DEC-145)." At the end of its evidence cell add
     `` , the `held_copy_` tests in `requests.rs` (in `engine/crates/core/src/requests.rs`) ``.
   - In `engine/README.md`, "there is no compaction or key expiry." becomes "there is no key
     expiry, and no compaction but the held copy an ended invocation drops (order 311)."

   Then run `python3 tools/promise_standing.py --check`, and `--write` where it fails.
7. **Tests.** Write them first, run them on the otherwise unchanged code and report each failing
   line. "As it stood in progress" means the two fields copied from the entry before the
   transition that ended it, as an earlier engine kept them. "As on the unchanged code" means
   captured by running the same steps there first and kept in the test as its expected value.
   - In `requests.rs`'s test module, with the control tests' fixture (`control_request_fixture`,
     `invoke_control`, `answer_control`, `inspect_settings`) where it serves, or one of yours:
     - `held_copy_dropped_at_each_end` (OPS-008, DL2): calls that end succeeded and failed hold
       neither field, and every other key and value of each equals its entry as on the unchanged
       code; one that ends unknown equals it whole; reconciling it (`reconcile.v1`,
       `occurred: true`) leaves neither field. Today each ended entry holds both.
     - `held_copy_end_record_unchanged` (REC-002, CTR-004, decision 2): with the Type stating
       `records: {inputs: true, outputs: true}`, a call that ends succeeded keeps `held_target`
       and holds no `implementation`; the ending transition's records equal those as on the
       unchanged code; its repeat's `request-ended` holds the repeat's inputs as today. With a
       Type that keeps neither, the entry holds neither field and the records hold no inputs or
       outputs. Today the entry holds `implementation`.
     - `held_copy_repeat_after_the_end_answers_as_before` (OPS-008, OPS-009, OPS-034, decision 3):
       a call whose operation declares outputs ends succeeded; its repeat is answered from the
       retained outputs and dispatches nothing; after `expire-outputs`, its repeat reports conflict
       under OPS-034 naming the invocation. Each repeat's reply, records and request replies equal
       those of the same repeat on the same state with the two fields put back as they stood in
       progress, and each repeat, on either state, leaves `state.invocations` equal to its input
       (OPS-034: "the invocation's state is unchanged"). Today the ended entry holds both.
     - `held_copy_inspection_reads_ended_controls` (REV-009, REV-010, REV-007): a control ends
       unknown, and inspection shows `/limit` unknown naming it; once reconciled, its entry holds
       neither field and keeps `contract`, and inspection still shows `/limit` unknown; a later
       control with the current revision succeeds, holds neither field, and inspection no longer
       shows `/limit`, as in `later_control_settles_unknown_setting`. Today both entries hold the
       two fields.
     - `held_copy_restored_after_a_refused_redispatch` (OPS-024, REV-009): a control fails; its
       repeat is dispatched again and that attempt refused, as in
       `filtered_redispatch_restores_failed_outcome_diagnostic`, or left unsent under REC-007 by a
       restart while its discovery is pending (either; yours). The restored entry is failed, holds
       neither field, keeps its `contract`, and `status.settings` equals what it was before the
       repeat. Today it holds the new attempt's two fields.
     - `held_copy_recover_compacts_earlier_entries` (DL2, DL10, IMP-007): a state as an earlier
       engine wrote it, each entry holding the two fields as it stood in progress: ones that ended
       succeeded, failed and reconciled; one that ended succeeded whose held Type states
       `records: {inputs: true}`; one that ended whose `held_target`'s `type` its environment
       lacks; one unknown; one in progress that is not a control of the inspected target.
       `Recover` succeeds: the first three lose the two fields and nothing else; the next two lose
       `implementation` alone; the unknown one is unchanged; the one in progress ends unknown as
       today and keeps both. A second `Recover` changes no entry. A repeat of the succeeded call is
       then answered from its retained outputs, and `status.settings` is as before the first
       `Recover`. Today `Recover` leaves the ended entries as they were.
   - In a new `engine/crates/shell/src/held_copy_tests.rs`, declared in `tests.rs` as
     `usage_tests.rs` is, each in a store the test makes in a temporary directory:
     - `held_copy_compacted_when_a_store_opens` (DL2, IMP-007): root calls `management.list.v1`
       with a key, which succeeds, and repeats it. Close the shell; put back into that entry of
       `state.json` a `held_target` and an `implementation` as an earlier engine wrote them (the
       target's publication view with the core environment, and `{"source":"engine"}`); open the
       store again. That entry holds neither field, memory equals disk, and the open's `recover`
       line in `commit-measures.jsonl` gives a `stateBytes` below the file's size before the open;
       a second open changes no entry; a repeat with the key gives the reply of the repeat before
       the close, and its `request-ended` data equal that repeat's but for IDs and times. Today the
       entry keeps both.
     - `held_copy_dropped_on_every_end_path` (DL2, REC-002): in the store `established` leaves
       (s1's steps 1 to 13), every ended entry holds no `implementation`, and holds `held_target`
       exactly where decision 2's function says so (s1's filesystem and history Types state
       `records: inputs: true`); the register-project entry, an `open.v1` entry and a `set-up.v1`
       entry are among them. Today each holds `implementation`.
     - `held_copy_store_measure` (DL2): in a store opened and first-started as
       `commit_measures_one_line_per_commit` does, root calls `management.list.v1` 20 times, each
       with its own key. The test prints one line to standard error,
       `held-copy measure: invocations=N stateBytes=B`, from the last line of
       `commit-measures.jsonl`, then asserts that no ended entry of `state.json` holds
       `implementation`, or `held_target` but where decision 2's function says so (no core Type
       keeps inputs or outputs). Today the assertion fails, after the line.
   - An existing test changes only where it asserts that an ended entry holds `held_target` or
     `implementation`, or compares a whole state that holds one. Report each.
8. **The measure** (DEC-149, DEC-143). Run `held_copy_store_measure` once on the unchanged code,
   right after writing it, and once at the end:

   ```sh
   (cd engine && cargo test --offline --release -p conduit-shell --lib held_copy_store_measure -- --nocapture 2>&1 | grep '^held-copy measure: ')
   ```

   Report both lines and the change in `stateBytes` per invocation: expect about 190 KB per call
   before, the core environment, and a few KB after.

## Start check

From the repository root:

```sh
k() { f=$1; shift; for p in "$@"; do grep -cF -- "$p" "$f"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
grep -c '^## DEC-149, ' docs/decisions/log.md  # 1
sed -n '/^## DEC-149, /,/^## DEC-150, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
k engine/crates/core/src/requests.rs '"held_target":target,"implementation":implementation' 'v["held_target"].clone()' 'fn restore_or_remove_invocation'  # 2 1 1
k engine/crates/core/src/records.rs 'invocation["contract"]["spec"]["changes"]'  # 1
k engine/CONFORMANCE.md 'invocation state is never compacted' 'From order 311,'  # 1 0
grep -c 'there is no compaction or key expiry.' engine/README.md  # 1
grep -c 'fn held_copy_' engine/crates/core/src/requests.rs  # 0
test -e engine/crates/shell/src/held_copy_tests.rs; echo "new $?"  # new 1
grep -cE '^\| (OPS-008|OPS-009|OPS-034|REV-009|REV-010|IMP-007) \| .* \| case passes \|$' engine/STANDING.md  # 6
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"  # engine 0
```

Where a line prints otherwise, stop, change nothing and report it. A fifth line other than `2 1 1`
means the core's dispatch or `record_content` moved since this order was written.

## Scope

Change `engine/crates/core/src/` for decisions 1 to 5 and the core tests, which go in
`requests.rs`'s test module; `engine/crates/shell/src/tests.rs` (its `mod` line) and
`held_copy_tests.rs` (new); `engine/CONFORMANCE.md`'s rows `DL2: storage` and
`DL10: retained results`; `engine/README.md`'s one sentence; and `engine/STANDING.md` by the tool
alone. Change an existing test only under decision 7. Change nothing else: not `spec/`,
`conformance/`, `docs/` (DEC-149 stays not done), the shell's other sources, the loader, the
conformance crate, `packages/`, `tools/`, a `Cargo.toml` or `Cargo.lock`. No promise changes, so
`spec/CHANGES.md` gains no line. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after; the second, guards, passes today and
after. Run each existing test you changed by name, once, too. Run the core's tests and then the
whole vectors test, once each, last; never the whole engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in held_copy_dropped_at_each_end held_copy_end_record_unchanged held_copy_repeat_after_the_end_answers_as_before held_copy_inspection_reads_ended_controls held_copy_restored_after_a_refused_redispatch held_copy_recover_compacts_earlier_entries; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 six times; 0 today
for t in held_copy_compacted_when_a_store_opens held_copy_dropped_on_every_end_path held_copy_store_measure; do (cd engine && cargo test --offline --release -p conduit-shell --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 three times; 0 today
grep -c 'From order 311,' engine/CONFORMANCE.md  # 2; 0 today
grep -c 'invocation state is never compacted' engine/CONFORMANCE.md  # 0; 1 today
grep -c 'there is no compaction or key expiry' engine/README.md  # 0; 1 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-shell memory_equals_disk_after 5  # 1
s conduit-shell killed_after_ 2  # 1
s conduit-shell retention_ 3  # 1
s conduit-shell commit_measures_one_line_per_commit 1  # 1
for p in change-unknown-setting fleets-reconcile- requests-records- requests-declared-outputs requests-plain-content requests-reconcile- requests-repeat-failed-earlier-answered requests-restart-interrupted requests-usage-kept requests-limits- requests-key-scope requests-defaults records-purge- secret-reads-; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # "N cases passed", N in order: 1 3 4 1 1 8 1 1 1 2 1 1 2 11
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
n=engine/crates/shell/src/held_copy_tests.rs; git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, twice
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/[a-z_]+\.rs|engine/crates/shell/src/(tests|held_copy_tests)\.rs|engine/(CONFORMANCE|STANDING|README)\.md)$' | wc -l | tr -d ' '  # 0
git reset -q -- $n  # stage nothing
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

Your report gives, per decision, what changed and where, and each path decision 3 changed; each
new test's failing line on the unchanged code, then its passing line; the two `held-copy measure:`
lines and the change per invocation; each existing test you changed, and why; your own choices;
and every check's last line. Stage nothing.

## What follows

- **DEC-149's other two orders** (DEC-148): archives kept once by digest; run leftovers removed.
- **Environments kept once by digest** (the study's R2): the held copies that stay become
  references, and a control's `changes` map can replace `contract`.
- **The dogfood store** (DEC-143) is compacted at its first open under this order, at a stage
  boundary or by a dated amendment; the coordinator reads that open's measure line, by numbers.
- DEC-149's status is the coordinator's once its three orders merge.

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, on `main` with the decision log
through DEC-157. Written, built, read and fixed in stage 3 of experiment 028; it is no order of
stage 3.

**Worker-2's turn** (15:31 to 15:56) built the drop and the compaction at open, with nine
regression tests written first. The measure on a store of 20 invocations fell from 4,345,951 to
133,991 bytes, about 210 KB saved for each.

**Two blind reads** (16:00 to 16:25). The Codex verifier, `root-verifier-2.turn.v1`, passed it
with no finding; the Claude reader (`reader-1`) had no must, two shoulds and a nit, one of them a
case where `record_content` newly failed. By the coordinator's ruling the old fallback was
restored, with a regression test. One fix turn by worker-2 applied the rest. The gate's run on the
squash is the coordinator's, at the merge.
