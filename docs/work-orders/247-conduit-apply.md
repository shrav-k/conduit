# Work order 247: `conduit apply`: the plan kept in `.conduit/`, its steps sent under plan-digest keys with a journal before each send, reads checked, no rollback, an interrupted apply resumed, sign-ins left to the panel, and another engine version stopped (the S1 chain; 052a, second part)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no vector changes. This order needs no
network: it reaches no host and fetches no crate, and the only sockets it opens are the loopback
ports its tests bind. Every crate it builds must be in the local cache already; if a build asks
for one that is not, stop and report, and fetch nothing. Run one cargo command at a time. Never
print a whole large file, diff, case or log: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential, even one a test made. Do not run
`conduit` outside a test, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in
nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public:
no absolute local path, no user or machine name and no private repository name.

## What and why

**What.** An author then carries out S1's T1, T2, T3 (a no-op) and T7's revoke with a bare
`conduit apply`. Order 245's `conduit plan` prints the steps; this order sends them: 052a's
second part in [batch 1's plan](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/README.md#the-work-orders).
Plan and apply are tooling made of existing requests
([DEC-008](../decisions/log.md#dec-008-2026-11-20-plan-and-apply);
[the short form's limits](../../spec/README.md#what-the-short-forms-meaning-covers);
[ruling 76](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-76/README.md),
points 3 to 5). [K3](../implementation/plan-and-apply/README.md), as order 241 left it, gives
the form; the flags are 248's and 249's.

**Read against `main` at `cc3605a3`; where the chain's plan was wrong.**

- Case 03 is a `conduit plan` case; T3's apply is the same no-op, so test 2 runs it as apply.
- The cases' `result` lines abbreviate by hand, and K3 defines no projection: decision 6, test 1.
- `serverInfo` holds the crate version alone, which does not tell two builds apart, and 243's
  context does not cover `--store`: decision 2 reads the live engine, for plan too (K3 O7).
- K3's grammar has no line for a failed read check: decision 5.
- Whether case 01's set-ups raise sign-ins is what 241's replay shows (its decision 5). If they
  do, test 1 checks decision 10's lines. Test 4 checks a sign-in either way.
- Case 07 needs 04 to 06's state, which apply cannot reach before 248: 241's harness replays it.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing repeats an effect implicitly (each request carries its plan's key, is journaled before
it is sent and is never sent again on apply's own initiative); and 2, every access can be
revoked (T7's revoke is a bare apply). None at risk; no promise or vector changes.

**Should this exist** ([rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes, as
existing practice. `terraform apply -auto-approve` prints its plan, applies it with no prompt,
keeps state and stops on a failure with no rollback. Keys are Stripe's `Idempotency-Key`, and
the journal before each send a write-ahead log. A client refuses a server of another build, as Terraform refuses state that a newer
version wrote, reading MCP's own `serverInfo`. Conduit's own part is DEC-008's reading of
narrowing, flags and sign-in. Not taken: a prompt (ruling 55, A4: the command is the approval),
rollback (K3), an automatic retry (the seventh convention), and state anywhere but the working
copy's `.conduit/` (K3).

**When it runs.** Through Conduit's dogfood: one Codex worker turn (4-hour limit; aim for 3
hours; the whole engine suite once, at the end), a Codex verifier and a Claude reader, who run
only what settles a doubt, and the gate (DEC-030, DEC-033). Each decision says what must hold;
how is yours (ruling 89). The names, lines and tests are the coordinator's, open to the owner's
strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further and report where:

- a line of the start check prints otherwise;
- a correct result needs a change to `engine/crates/core`, `engine/crates/loader`,
  `conformance/`, `packages/`, a `Cargo.toml` or the lockfile, or to a plan line that 245's
  done-when fixes (245's `row_08_` test, changed by decision 9, excepted);
- a set-up that raised a sign-in does not succeed, or its result has no `pendingAction`
  (IFC-025): the engine's fault, not apply's;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The command.** `conduit apply [PROJECT_DIR] [--store DIR]`, on order 243's engine context
   as plan. Given one of the seven approval flags, it sends nothing, exits 2 and writes 245's
   line with `conduit apply:`; 248 and 249 remove those lines. `help.rs` gains apply's row in
   245's form: K3's description ([Commands](../implementation/plan-and-apply/README.md#commands)),
   no approval flag, example `apply`.
2. **The engine's version.** Before the first `tools/call` of either `plan` or `apply`, the CLI
   compares its own version with the `serverInfo` of its session's `initialize`. The version is
   the crate version and build commit that `conduit --version` prints, written `VERSION+COMMIT`
   (semver's build metadata). Serve's `get_info` reports it in place of the crate version alone.
   On another version, or a name other than `conduit`, it prints nothing to stdout and one
   stderr line naming both versions and that nothing was sent, writes nothing, and exits 2
   (K3 O7). Keep one check for both commands, 245's if it built one.
3. **Print, then keep.** Apply plans with 245's planner, from the same reads, and prints the
   identical text.
   - `no changes`: send nothing, write nothing, status 0.
   - `stopped`: print `not sent 1..N` where the plan has N steps (no line where it has none, as
     in case 09), write nothing, status 2.
   - `ready`: before step 1, keep under PROJECT_DIR, each file atomic and synced
     ([kept state](../implementation/plan-and-apply/README.md#kept-state-and-privacy)):
     `.conduit/plans/HEX.txt`, the plan's exact bytes, whose SHA-256 is HEX; the journal
     `.conduit/results/HEX.yaml`; and `.conduit/state.yaml`, naming HEX among its plans.
     Where a write fails, print a stderr line naming the path and `not sent K..N` for the steps
     left, send nothing more, and exit 2.
4. **Send.** Steps go in order, one at a time, each waiting for its result.
   - Each step carries `io.github.shrav-k.conduit/idempotencyKey`, `plan:HEX:ATTEMPT:STEP`, in
     `_meta`, and declares Conduit's extension (IFC-023, IFC-036).
   - Its journal entry (step, tool, safe arguments, key and flag) is on disk before the step is
     sent, and its safe result right after it.
   - A reference to an object an earlier step registered takes the ID that step returned
     (K3's `(step N)`).
   - The executor knows no tool. Each step comes from the planner with what to send, which
     earlier result it reads, what a read must match and its effect on the ledger, so 248 and
     249 add acts without touching the executor. Add what 245's steps lack, in the planner.
5. **Check the reads.** Each inspection after a register must match, before execution goes on:
   the ID the register returned, and the resolved definition and held packages the planner
   resolved (K3, "Reads after printing"). On a mismatch:
   - after the step's `result` line, print `verify N failed KIND/NAME place=POINTER`, where
     POINTER is the first place that differs (`/id`, a place in the resolved definition, or
     `/packages`), with no value;
   - then `not sent N+1..M`, and exit 1.
   Nothing more is sent, and the object is neither opened nor set up.
6. **Results and stops.**
   - After each result print `result N OUTCOME JSON`. OUTCOME is the outcome (succeeded, failed
     or unknown) or the reason (refused, invalid, conflict or unavailable).
   - JSON is `structuredContent`'s members other than `records`, `recordGaps` and
     `recordsComplete`, plus `_meta`'s outcome, reason, invocation and pendingAction under their
     unprefixed names. Absolute paths are fingerprinted by the rule 245's plan text uses, in
     sorted-key compact JSON. The journal keeps the same projection.
   - Any result other than succeeded stops apply: print `not sent N+1..M` for the steps left
     (`7..7` for one), and exit 1. Nothing is rolled back or sent again.
   - A transport failure after a send is an unknown delivery. Print `result N unknown {}` and a
     stderr line naming the failure, the step and its key. Keep the entry without a result, and
     exit 1.
   - A local write failure after a send exits 2 and keeps the journal.
7. **The attempt.** A digest with no journal starts at attempt 1. The attempt is kept in the
   journal ([repeats](../implementation/plan-and-apply/README.md#stops-unknown-outcomes-and-repeats)).
   - It rises by one where the digest's last attempt ended with a step unsuccessful.
   - It stays the same otherwise, as where that attempt completed or was cut short: a step sent
     with no result, or a kill or local failure between steps. So an identical plan sends the
     same keys again, and OPS-009 and OPS-012 decide the engine's answer.
8. **The ledger.** `.conduit/state.yaml` has the form 245's planner reads: K3's `kept`, with
   real IDs and each journal named by its digest.
   - It is replaced after each step's result with that step's effect. So a stop, or a kill
     between steps, leaves it exact: each object the apply registered is in it from its
     register's result on, with the flag its registration carried.
   - Nothing under `.conduit/` holds an absolute path, the store's path or a credential.
9. **Resume (K3 row 8).** After a stop, the next bare apply plans from the engine and this
   ledger. An instance this working copy registered that is still registered is inspected,
   verified, then set up or opened, under the flag its registration carried. Nothing is
   registered again.
   - Row 8 is this order's: 245 prints its fallback, and its `row_08_` test checks it. Build
     row 8 in `conduit_shell::plan` and change that test to row 8's steps.
   - A register with no result is not taken over by name: its objects stop under row 10, which
     249's `--adopt` settles after verifying them.
10. **Sign-ins.** For each ID in a succeeded set-up's `pendingAction`, print
    `panel action ID: sign-in required` (ruling 76 point 4; K3 point D). These lines come after
    the last `result` line and any `not sent` line, in the order returned, each ID once. Apply
    answers none, and sends no `sign-in.v1` and no `answer.v1`. Pending sign-ins alone leave
    status 0. In this build a set-up raises only sign-in actions (TPL-012, PND-005).
11. **K3 and the guide.** Add three things to [Text grammar and statuses](../implementation/plan-and-apply/README.md#text-grammar-and-statuses):
    the `verify` line, decision 6's projection, and where the panel lines go. Where
    `docs/guides/plan-and-apply.md` (order 246) exists, add them there too.

## The tests

**End to end**, in a new `engine/crates/shell/tests/apply.rs`, on order 241's harness: its
fixture packages, its scripted filesystem, its ID map, and its seeding of the engine (the replay
of earlier cases' requests) and of `.conduit/state.yaml` (from `applied`), as 245's plan tests
use them. Each test runs the `conduit` binary against a serving engine. Each checks that no file
under `.conduit/` holds the test's temporary directory or the store's path.

1. `apply_case_01_t1`, `apply_case_02_t2`, `apply_case_07_t7_revoke` and
   `apply_case_09_t7_restore_unflagged`. From the case's `applied` and `engine`, a bare
   `conduit apply` prints `printed` byte for byte up to `ready` and after its last `result`
   line; each `result` line has the case's step and OUTCOME, and its JSON holds each member the
   case's line states, compared as 241's harness compares a result (`view` and `tombstones` in
   K3's compact notation, under the map, times not compared). It keeps `recorded` at its path,
   sends `requests` (the tools in order as the engine's records show them, the arguments and
   keys as the journal does), leaves `.conduit/state.yaml` equal to `kept` and exits with
   `exit`, all under the map, each real digest mapped to the case's. For case 09, `.conduit/` stays byte for byte, and the
   engine records no invocation after the reads.
2. `apply_case_03_t3_noop`: case 03, run as `conduit apply`, prints the case's `printed` and
   exits 0. It sends nothing after the reads and leaves `.conduit/` byte for byte.
3. `apply_resumes_after_a_stopped_step`: case 01's input, with one step after the register made
   to fail with a known outcome (how is yours: no sleep, no retry loop, and no switch in the
   binary that only a test uses). The first apply prints that step's `result` line and
   `not sent`, exits 1 and undoes nothing. Once the cause is removed, a second bare apply
   prints no register step, carries out the rest and exits 0; the engine ends as in case 01,
   and state.yaml as its `kept` under the map, with both digests among its plans.
4. `apply_prints_pending_sign_ins_and_answers_none`: an Agent of a fixture Type that states
   signIn, whose account hook reports no account, as `rmcp_sign_in_provider` in
   `tests/runner.rs` answers. The test packs and installs it: no npm and no real client.
   Apply prints one `panel action ID: sign-in required` line, for the ID `conduit pending`
   lists, and exits 0; the records show no `sign-in.v1` or `answer.v1`, the action stays
   unanswered, and a second apply prints `plan`, then `no changes`.
5. `apply_and_plan_stop_on_another_engine_version`: against an endpoint whose `initialize`
   answers `serverInfo.version` as the crate version alone, as `main` serves it, and against
   one with another version, each command exits 2, prints nothing to stdout and names both
   versions on stderr. The endpoint sees no `tools/call`. 245's
   `plan_stops_on_another_engine_version` moves onto this endpoint; report it.
6. `apply_sends_nothing_when_its_state_cannot_be_kept`: with `.conduit/` a directory and a
   regular file at `.conduit/plans`, case 01's apply prints the plan and `not sent 1..15`,
   names `.conduit/plans`, and exits 2. The engine records no invocation after the reads.

**Unit tests**, in the shell's library, with an injected sender and steps of tools the executor
has never seen:

7. `apply_journals_each_step_before_sending_it`: the sender finds each step's entry on disk,
   each result is journaled after it, and later steps carry the IDs earlier ones returned.
8. `apply_stops_on_a_failed_read_check`: a read whose ID, definition or packages differ prints
   the `verify` line and `not sent`, and gives status 1. Nothing more is sent.
9. `apply_attempt_rises_only_after_an_unsuccessful_step`, decision 7's three cases: an
   unknown delivery and a completed attempt keep the attempt and the keys; a refused step
   raises it.

Test 8 holds K3's safety cases 6 (a returned ID differs) and 7 (a definition differs after
printing), and test 4 holds case 17; name each in a comment. Case 8 is 248's.

## What to read

- K3: its sections from "Commands" to "Case format and sequence", the difference table's rows
  5, 8 and 10, O7 and O8; cases 01, 02, 03, 07 and 09.
- IFC-023, IFC-025 and IFC-036; OPS-008, OPS-009, OPS-012 and OPS-034; PND-005.
- 245's planner and `plan` command, 243's context, and 241's harness.
- In `engine/crates/shell/src`: `server.rs` (`client`, `get_info`), `author.rs` (`succeeded`) and
  `store.rs` (its atomic writes). In `tests/runner.rs`: `rmcp_sign_in_provider` and
  `sign_in_relays_url_from_rmcp_stdio_provider`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
grep -l '479ef47083c8' docs/implementation/plan-and-apply/cases/*.yaml | wc -l | tr -d ' '
grep -c '"plan",' engine/crates/shell/src/help.rs
grep -c '"apply",' engine/crates/shell/src/help.rs
grep -c '"version":env!("CARGO_PKG_VERSION")' engine/crates/shell/src/server.rs
test -e engine/crates/shell/tests/apply.rs; echo "apply $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

Once 241 and 245 have merged, these print `main 0`, `0`, `1`, `0`, `1`, `apply 1`, `engine 0`
and `0`. On `main` at `cc3605a3` the second prints `9` (241 has not re-pinned the cases off the
old agents digest) and the third `0` (no `plan` before 245): the order cannot start yet.

## Scope

Change these:
- `engine/crates/shell/`: apply's modules, `main.rs`, `help.rs`, and the planner where
  decisions 4 and 9 say; in `server.rs`, `get_info`'s version and a client that carries
  `_meta`; `tests/apply.rs`, and 241's shared test helpers where a test needs one;
- `engine/STANDING.md`, only as `tools/promise_standing.py --write` regenerates it;
- `docs/implementation/plan-and-apply/README.md`, for decision 11;
- `docs/guides/plan-and-apply.md`, where it exists.

Nothing else, the engine README included. Report anything else that needs changing.

## Done when

Write the tests first and run them on the unchanged code: report each failing line. Then change
the code until all pass. Each line marked "fails today" prints otherwise on `main` at `cc3605a3`,
and also on a `main` with 245 merged; all pass after.

```sh
(cd engine && cargo test --release -p conduit-shell --test apply 2>&1 | grep -c '^test result: ok\. 9 passed')  # 1; fails today (0)
(cd engine && cargo test --release -p conduit-shell --lib apply_ 2>&1 | grep -c '^test result: ok\. 3 passed')  # 1; fails today (0)
grep -c '"apply",' engine/crates/shell/src/help.rs  # 1; fails today (0)
grep -c 'verify N failed' docs/implementation/plan-and-apply/README.md  # 1 or more; fails today (0)
```

These pass, as on `main`. Run the engine's whole suite once, last:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git status --short --untracked-files=all | cut -c4- | grep -vE '^engine/crates/shell/|^engine/STANDING\.md$|^docs/implementation/plan-and-apply/|^docs/guides/plan-and-apply\.md$' | wc -l | tr -d ' '  # 0
(cd engine && cargo test --release)  # every test passes
```

Your report gives what changed for each decision; each test's failing line on the unchanged
code, then its passing line; what decisions 4 and 9 added to 245's planner; and each check's
last line. Stage nothing.

## What follows

Named here and not in this order: the other flags (248, 249), as planner steps this executor
sends unchanged; a plan line tying a row 10 stop to this working copy's own unknown delivery of
a register; and the rerun's preparation (250), with the real Agent packages, whose set-ups raise
the panel actions K3 counts at T1.

## Record

Not yet run.
