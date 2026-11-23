# Work order 289: the engine keeps a call's usage limits in its end record and shows the newest of each window on the Agent and its login Resource (DEC-077, the engine's part)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network, and every crate it builds is
cached: run cargo offline, and if a crate is missing, stop and report, and fetch nothing. Run one
cargo command at a time, and not `tools/gate.py` or the whole engine suite. Never print a whole
large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Run no `conduit` outside a test, no
`codex` or `claude`, and sign in nowhere: every implementation a test meets is scripted. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute
local path, user, machine or account name, or private repository name.

## What and why

**The owner's answer** ([DEC-077](../decisions/log.md#dec-077-2026-11-22-usage-and-limits-advertised-in-v0)):
"an agent must keep track of and advertise its usage, its a core feature of conduit"; then
"Design B in v0 (Recommended)". Order 284 wrote the spec's first part: IFC-052 (a result's
`limits` member kept in its end record, or dropped with a warning), IFC-053 (the newest of each
window in an Agent's and a login Resource's status, `unknown` from its reset), `limit-report.v1`,
`limit-status.v1`, REC-002, OBS-002 and OBS-003 as amended, and two cases listed as expected
failures. This order builds them in the reference engine and removes both entries.

**Today**, read on `main` at `52a29f1a` with order 284 stacked. `requests::complete` takes the
`usage` member (`usage.rs`, [order 262](262-engine-usage-in-the-end-record.md)) and writes it
into the `invocation-ended` record after `complete_call`; nothing reads
`io.github.shrav-k.conduit/limits`. `envelope` builds the reply's `_meta` anew, so the member
already never reaches the caller. `inspect.v1`'s status (`records.rs`) holds no `limits`.

**The read.** A login Resource's operations are dispatched as any Resource's, and PND-005's hold
refuses them while it is held, so a `limits.v1` that its Type lists already reaches its
implementation: this order keeps that call's report like any other, and a test shows it.
`limits.v1` is agent-codex's, not the core's (DEC-045). A Type's `reports` statement and its
check at install are the spec's unwritten second part ("What follows").

**DEC-066 is not built here.** The coordinator's design notes recommend building its condition
route with this order. It does not fit: its interface spec order is unwritten (which conditions a
Type names, the `_meta` key, the clearing at a run's end), an engine order changes nothing under
`spec/`, and this order alone fills one turn. Its engine order follows the spec on this order's
path: the member taken beside usage, the check and warning, and the kept state.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7,
each window names the instance, invocation and record that reported it; 3, one kept entry per
window and instance, the end record its source; 4, any client's windows, under its package's
fixed IDs; 6, an inspection starts no call, and no number permits or holds anything. None at risk:
no outcome, reply, decision or other record changes.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes: IFC-052 and IFC-053 are musts, and DEC-077 puts them in v0. Existing practice gives the form.
Order 262's path for usage applies OpenTelemetry's rule that telemetry never fails the work it
describes. A Kubernetes status keeps the last observation with its time, and it outlives the
events that reported it. Not taken: rebuilding the status from records at each inspection (a
purge would shorten it, and IFC-053 says none does); a record of its own; totals or a hold (68,
DEC-077); DEC-066's route (above).

**When it runs.** On `main` from `52a29f1a` (#563), after order 284, whose `limit-report.v1`
the engine checks against: merged, or rebased onto `52a29f1a` and stacked as one commit. Through
Conduit: one Codex worker turn, the blind Codex verifier and Claude reader at once, one fix turn,
the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what must hold; how is
yours. The coordinator's, open to the owner's strike: the test names, the warnings' words, and
that the newest reports are kept as engine state. It shares core `requests.rs` (the head and tail
of `complete`) and `records.rs` (inspect's status) with other engine orders, and
`engine/STANDING.md` with all of them: the later to merge rebases and reruns
`promise_standing.py --write`.

**Stop rule** (the coordinator's D1). Stop, change nothing further, and report only for a
contradiction between promises (give both), a step that needs a change outside Scope, or a file
outside Scope. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, beside any choice no decision governs.

## The decisions

1. **Which ends keep it** (IFC-052, REC-002). The ends that keep usage under order 262's
   decision 1 keep the limits member, plain or keyed, hooks among them; the same ends keep none.
   The member is the value in the result's `_meta` under `c.meta_prefix` followed by `limits`.
   Every read of a secret ignores it without checking, keeping or warning (SEC-015), as it does
   usage.
2. **Kept or dropped** (IFC-052). The member is checked against `limit-report.v1` of the engine's
   own core (`c.core_environment`), then for a window `id` named twice. Where it passes both,
   `data.limits` is the value, unchanged; otherwise `data.limitsDropped` is decision 3's warning,
   and the record holds no part of the value. The member never fails the step.
3. **The warning** (IFC-052, ruling 52). Usage's form: `ruleId` `IFC-052`, `level` `warning`,
   one logical location whose `fullyQualifiedName` is a JSON Pointer from
   `/_meta/io.github.shrav-k.conduit~1limits`, and no value quoted. Its three forms:
   - **The schema fails:** the first failure in instance-path order, as usage's, its place after
     the key's pointer and its `keywordLocation` in `properties`; the text is "The result's limits
     member was dropped: at P it does not conform to limit-report.v1, failing its W keyword. The
     record of this invocation's end keeps no part of it, and nothing else about the call changes
     (IFC-052)." The place follows `properties` and, through an array index, `items`, to the
     last name the schema fixes (the case's `/windows/0/usedPercent`). `first_core_failure`
     stops at `/windows`: extend it, or add a sibling, in `schemas.rs`. `usage.v1` has no array,
     so usage's places stay as they are.
   - **The schema passes and an `id` repeats:** the pointer ends at `/windows`, with no
     `properties`; the text is "The result's limits member was dropped: at P two windows have the
     same id. The record of this invocation's end keeps no part of it, and nothing else about the
     call changes (IFC-052)."
   - **The check errs:** the key's own pointer and no `properties`, with usage's second text
     naming `limit-report.v1` and IFC-052.
4. **The kept state** (IFC-053, OBS-002). When a member is kept, the same transition updates the
   engine's own state for the call's target, where it is an Agent or a Resource whose Type states
   `signIn`. Where the target is an Agent whose login place names a Resource
   (`login_resource::id`), it updates that Resource's state too. For `newest`, and for each window
   the member names, the member takes the kept entry's place unless the kept entry's `observedAt`
   is a later instant. Compare instants with `chrono`, never strings; at an equal instant, the
   member kept later stands. An entry holds:
   - the window's reported members and the member's `observedAt`;
   - the end record's `source`;
   - the instance whose call it was, in `object.v1` form (kind, name and ID, and `nested` where
     it has one);
   - the invocation;
   - for `newest` only, `usageAllowed` and any `noWindows`.

   A member with no window changes only `newest`. No purge shortens the state: not `purge.v1`,
   not `expire-outputs`, not retention. Neither does an Agent's retirement or removal, for its
   login Resource. The state is one new `State` field in core `lib.rs`, a map keyed by the
   instance's ID, with `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]` so that an
   earlier store still reads. Memory equals disk after the commit (DL2). Name IFC-053 there.
5. **What `inspect.v1` shows** (IFC-053, `limit-status.v1`, OBS-003). Only an inspection of an
   Agent or of a Resource whose Type states `signIn` shows `limits`. With no kept state it is
   `{"state":"unknown","windows":[]}`. Otherwise it is `state` `observed` with `newest` and the
   windows, listed by `id` in Unicode code point order. A window whose `resetsAt` is an instant
   no later than the inspection's evidence time reads `state` `unknown`, keeping its other
   members, until a newer member names it. No other inspection, other operation's output, record
   or reply shows the kept state.
6. **Nothing else changes** (IFC-052, IFC-053, ruling 68). The outcome, the reply and its
   repeats, the retained result, the decisions, every other record and every record ID are what
   they would be without the key. Keeping or dropping allocates no ID. No reply carries the member
   or the warning. The Type's `records` does not gate it (CTR-004). The engine sums nothing and
   compares only decision 4's and 5's times. No access, dispatch, hold or lifecycle state depends
   on it, and no inspection starts a call.
7. **One way** (the least change). A new `engine/crates/core/src/limits.rs`, declared by one line
   in core `lib.rs`, takes the member beside `usage::take` in `complete`. It writes the record and
   the state once the body returns, as usage does, and builds the status for `records.rs`.
   `usage.rs` and `schemas.rs` may share a helper with it if usage's behaviour and words stay as
   they are.
8. **Tests.** Write them first, run them on the otherwise unchanged code and report each failing
   line. K is the key, and the reports are `limit-report.v1` values of the cases' form.
   - In a new `engine/crates/core/src/limits_tests.rs`, declared `#[cfg(test)]` in core `lib.rs`
     as `login_resource_tests` is: build from `requests-limits-latest`'s state with `run_case`,
     as `login_resource_one_call_at_a_time` does. `login_resource_tests.rs`'s helpers may become
     visible to it; nothing else there changes.
     - `a_login_resources_own_read_keeps_its_limits_in_its_status`: give `login`'s held Type a
       read operation, `limits.v1`, that states no `effect` and does not start model work, and
       open a Conduit from root that lists it (how is yours). A call to it whose result carries K:
       its reply holds no K; its `invocation-ended` holds the report; `login`'s status shows the
       report's window, with `instance` `login`; `a`'s and `b`'s statuses are unchanged. With
       `login` held (`sign_in_holds`), the call reports unavailable, is not dispatched, and no
       status changes. Today `login` shows no `limits`.
     - `limits_status_outlasts_a_purge_and_takes_the_later_of_equal_times`. After the case's
       turns, a `purge.v1` of every record leaves `a`'s and `login`'s `limits` as they were. A
       member is kept later at R1's instant, written `2026-11-21T15:01:00+02:00`: it stands on
       `a` for each window it names. A member one second older changes nothing. Today no
       `limits` is shown.
     - `only_agents_and_login_resources_show_limits`: a Conduit, a Pool and a Resource whose Type
       does not state `signIn` show no `limits`. A call to such a Resource keeps its member in
       its end record, and no status shows it; that Resource, its operation and a Conduit to it
       are yours. An Agent that names no login Resource shows its own: run
       `requests-limits-kept`'s state with `run_case`; its `agent` names none. Today the first
       record holds no `limits`.
   - In a new `engine/crates/shell/src/limits_tests.rs`, declared in `tests.rs` as
     `usage_tests.rs` is, a guard that passes today too:
     `a_secret_read_ignores_its_limits_in_its_invocation_end`. Set it up as
     `a_secret_read_ignores_its_usage_in_its_invocation_end` is, with the read's result carrying
     K and a report. No record holds `limits` or `limitsDropped`.
   - An existing test changes only where it compares a whole status of an Agent or a login
     Resource, to add `limits`. Report each.
9. **The cases and documents.** Remove both `requests-limits-` entries from
   `conformance/expected-failures.yaml`, and change no case. In `engine/CONFORMANCE.md`'s DL8
   row, after "and no number decides anything.", add this sentence: "From order 289, it also holds
   in `limits` the limits member, or in `limitsDropped` the IFC-052 warning; the engine keeps the
   newest report of each window, as its own state that no purge shortens, for the Agent or login
   Resource called and for an Agent's login Resource, and `inspect.v1` shows it, `unknown` from
   its reset (IFC-053)." Add `` `requests-limits-kept`, `requests-limits-latest`, `` to its
   evidence cell after `` `requests-usage-kept`, ``. Then run
   `python3 tools/promise_standing.py --write`.

## What to read

The promises IFC-051 to IFC-053, REC-002, REC-005, OBS-002, OBS-003, CTR-004, CTR-013, PND-005
and SEC-015. The schemas `limit-report.v1`, `limit-status.v1`, `record.v1` and `inspect.v1`.
Orders 262 and 284. In the core: `usage.rs`, `schemas.rs` (`first_core_failure`), `requests.rs`
(`complete`, `envelope`), `records.rs` (inspect's status, `purge.v1`), `login_resource.rs`,
`sign_in.rs` (`request_is_held`) and `login_resource_tests.rs`. In the shell: `usage_tests.rs`.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor 52a29f1a HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
grep -c '^- id: IFC-05[23]$' spec/interface/implementation.yaml
grep -cE '^  name: limit-(report|status)\.v1$' spec/core/results.yaml
grep -cE '"id": "requests-limits-(kept|latest)"' conformance/expected-failures.yaml
ls conformance/conventions/requests/limits-*/case.yaml | wc -l | tr -d ' '
git grep -l -e 'IFC-05[23]' -e 'limit-report' -e 'limit-status' -e 'limitsDropped' -- engine/crates | wc -l | tr -d ' '
test -e engine/crates/core/src/limits.rs || test -e engine/crates/core/src/limits_tests.rs || test -e engine/crates/shell/src/limits_tests.rs; echo "new $?"
grep -c 'and no number decides anything.' engine/CONFORMANCE.md
grep -c 'From order 289,' engine/CONFORMANCE.md
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
```

They print `main 0`, `0`, `2`, `2`, `2`, `2`, `0`, `new 1`, `1`, `0` and `engine 0`. Any
other line means stop, change nothing and report it. A `0` in the third to sixth lines means
order 284's spec is not in the tree.

## Scope

Change `engine/crates/core/src/limits.rs` and `limits_tests.rs` (new), core `lib.rs` (two `mod`
lines and decision 4's `State` field), `requests.rs` (`complete` alone), `records.rs` (inspect's
status), `schemas.rs` (decision 3's place), `usage.rs` (a shared helper, if needed) and
`login_resource_tests.rs` (helper visibility). In the shell, change `tests.rs` (its `mod` line)
and `limits_tests.rs` (new). Change the two entries of `conformance/expected-failures.yaml` and
`engine/CONFORMANCE.md`'s DL8 row, and `engine/STANDING.md` only by the tool. Change an existing
test only under decision 8. Change nothing else: not `spec/`, a case, `docs/` (DEC-077 stays not
done), `packages/`, `tools/`, the loader, the conformance crate, the shell's other sources, a
`Cargo.toml` or `Cargo.lock`.

## Done when

The first group prints otherwise today and passes after. The second group are guards, which pass
today and must still pass. Run each existing test you changed by name, once, too. Run the
group's last line, the vectors test, once, last. Do not run the whole engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in a_login_resources_own_read_keeps_its_limits_in_its_status limits_status_outlasts_a_purge_and_takes_the_later_of_equal_times only_agents_and_login_resources_show_limits; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 three times; today 0
for c in requests-limits-kept requests-limits-latest; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$c" 2>&1 | grep -c "^PASS $c "); done  # 1 then 1; today 0 and 0
grep -cE '"id": "requests-limits-(kept|latest)"' conformance/expected-failures.yaml  # 0; today 2
grep -c 'From order 289,' engine/CONFORMANCE.md  # 1; today 0
grep -cE '^\| IFC-05[23] \| interface \| must \| 1 \| 0 \| outside \| [a-z, ]+ \| case passes \|$' engine/STANDING.md  # 2; today 0
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-shell a_secret_read_ignores_its_limits_in_its_invocation_end 1  # 1 once written, on unchanged code too
s conduit-shell usage_tests:: 4  # 1
s conduit-shell memory_equals_disk_after 5  # 1
(cd engine && cargo test --offline --release -p conduit-core --lib 2>&1 | grep -c '^test result: ok\.')  # 1: the core's tests
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
n=$(git ls-files -o --exclude-standard -- engine); [ -z "$n" ] || git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(limits|limits_tests|lib|requests|records|usage|schemas|login_resource_tests)\.rs|engine/crates/shell/src/(tests|limits_tests|server_tests)\.rs|engine/crates/shell/tests/[a-z_]+\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/expected-failures\.yaml)$' | wc -l | tr -d ' '  # 0
[ -z "$n" ] || git reset -q -- $n  # stage nothing
(cd engine && CARGO_NET_OFFLINE=true CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors 2>&1 | grep -c '^test result: ok\.')  # 1: every vector passes
```

Your report gives, per decision, what changed and where: each new test's failing line, then its
passing line; each case's line before and after; each existing test you changed, and why; your
own choices; and every check's last line. Stage nothing.

## What follows

- **The spec's second part** (DEC-077), one turn: a Type with a `startsModelWork` operation, or
  one that states `signIn`, states `reports` (`usage` and `limits`: `turns`, `read` or `none`),
  and is rejected at install without it. `limits: read` answers `limits.v1`, and the status gains
  `not-reported`. A short engine order then checks it at install.
- **DEC-066:** its interface spec order, its engine order on this path, the github release.
- **agent-codex** (U3) and **agent-claude-code** (U4) at a stage boundary (order 277 item 8),
  then **the CLI** (U5), a limits table, and **the dispatcher** (U6), which pauses and reopens
  from the status.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, stacked
on spec order 284.

**Worker-2's turn** (15:03 to 15:23) built DEC-077's engine part: a call's usage limits are kept
in its end record whatever the Type's `records` states, the newest report of each window is
kept on the Agent and its login Resource and shown by `inspect.v1`, a window reads unknown after
its reset, and nothing is summed or acted on. Both of 284's cases pass and leave the expected
failures, with 192 core tests and the whole vectors test.

**Two blind reads.** The Claude reader (15:24 to 15:28) had no must and two nits. The Codex
verifier (`turn-verifier-1.turn.v1`, 15:36 to 15:43) had one must: valid timestamps that differ
beyond nine fractional digits compared equal, so an older report could replace a newer one. One
fix turn, by worker-1 in the same working copy (15:43 to 15:55), applied all three, both
timestamp regressions failing first; 195 core tests and the vectors test pass. The gate's run
on the squash is the coordinator's, at the merge.
