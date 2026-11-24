# Work order 315: the engine keeps a usage report's paid usage and each usage signal's newest value (DEC-094, DEC-096, the usage guard's first engine order)

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

**The owner's words.** [DEC-094](../decisions/log.md#dec-094-2026-11-23-usage-is-self-reported-by-packages-and-the-engine-trusts-the-report):
"Conduit specifies what must be reported and what contracts must be upheld. The engine then trusts
this reporting"; it lands with an engine that "computes nothing itself", so the engine adds nothing
up. [DEC-096](../decisions/log.md#dec-096-2026-11-23-a-usage-report-may-carry-paid-usage-state),
"Accept (Recommended)": a report may carry whether paid usage is possible and in use, with no
amount. DEC-120: "I don't want to modify how the models operate just police their usage", so the
engine keeps only what a package reported.
[Order 307](307-limits-carry-paid-usage-and-signals.md) wrote the spec's half (IFC-052,
IFC-053, OBS-002): a report's `paidUsage`, `windowsOmitted`, null `usedPercent` and wider window
IDs, kept in `newest`, and `signals`, the newest value other than null of `usageAllowed` and of
paid usage's `possible` and `inUse`, which no later null, time or purge clears; it listed
`requests-limits-signals` as an expected failure. This order builds that in the reference engine
and removes the entry. The login's admission hook, from the guard's later orders, reads these
signals.

**Today** (`main` at `1e135e61` with order 307's build on top), `limits.rs` keeps each member
whole in its end record and, per instance in `State`'s `limit_observations`, a `newest` without
`paidUsage` or `windowsOmitted` and the windows as reported; a later null `newest` hides an
earlier `false`, and `limits::status` gives no `signals`, so `requests-limits-signals` fails at
its step 18. No package in the tree sends a limits member.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7,
each signal names the record source, instance and invocation that reported it; 6, no inspection
starts a call to fill it, and nothing depends on it (IFC-053); 4, the engine keeps what a package
reported and reads no client's field. None at risk: only an inspection's `limits` gains members.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes:
IFC-053 is a must, as order 307 amended it, and under DEC-094 a login package decides from what the
engine keeps. By existing practice, a Kubernetes Node keeps each condition apart with its own
heartbeat time, and a patch that omits one leaves it standing, as a null leaves a signal. Not
taken: rebuilding signals from records at inspection or open (a purge would shorten them); clearing
a signal by its reset, its age or a later null (DEC-094: its reader judges); a signal per window;
totals.

**When it runs.** It stacks on order 307 and runs on 307's build until 307 merges, then on `main`.
Through Conduit: one Codex worker turn, the blind Codex verifier and Claude reader at once, one fix
turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what must hold; how
is yours. The coordinator's, open to the owner's strike (DEC-029): keeping the signals as engine
state beside `newest`, decision 4's store format, the test names and the row's words.

**Shared files.** Order 314, also on 307, adds two entries to
`conformance/expected-failures.yaml`, adds to IFC-053 an exception for ending a login's admission
report (a later engine order builds it), changes `engine/STANDING.md`, and expects
`requests-limits-signals` still listed. Order 311 and DEC-149's other engine orders change other
rows of `engine/CONFORMANCE.md`, and 311 core `requests.rs` and `records.rs`, which this order
leaves as they are. Whichever merges later rebases and reruns `promise_standing.py --write`; where
314 merged first and IFC-053's row counts its cases, work from the row and list it under
`own_choices`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first IFC-051, IFC-052, IFC-053, OBS-002, OBS-003, OBS-006, REC-002, REC-005, SEC-015 and DL2;
the schemas `limit-report.v1` and `limit-status.v1` (`spec/core/results.yaml`) and `inspect.v1`'s
description; the decisions of orders 307 and 289; and the row `DL8: records` of
`engine/CONFORMANCE.md`. In the core, `limits.rs` whole, `limits_tests.rs` and `State` in `lib.rs`;
in the shell, `limits_tests.rs`, `Store::write` in `store.rs`, and `established`, `open`, `actor`
and `assert_memory_equals_disk` in `tests.rs`.

1. **What `newest` holds** (IFC-053, `limit-status.v1`). Today's fields, and the member's
   `paidUsage` (whole, its nulls kept) and `windowsOmitted` where it gave them; `usageAllowed` as
   given, null or not. The newer member takes the whole place, so one without `paidUsage`,
   `noWindows` or `windowsOmitted` leaves `newest` without it. The windows are kept and listed as
   today, a null `usedPercent` and an ID with digits and hyphens among them.
2. **The signals** (IFC-053, OBS-002). The transition that keeps a member, for the same instances
   as `newest` and the windows (the call's target, and an Agent's login Resource), keeps for each
   of three signals, `usageAllowed` and `paidUsage`'s `possible` and `inUse`, the newest member
   that gave it a value other than null. A signal's entry holds that `value`; for `possible` and
   `inUse`, the member's `paidUsage.resetsAt` as `resetsAt`; and, as a window has them, the
   member's `observedAt`, the end record's `source`, the instance in `object.v1` form and the
   invocation. A null or absent value leaves the signal as it stands, whatever its time. Between a
   kept signal and a member, `keep_latest`'s rule decides: the later instant by `chrono` and the
   digits past the ninth fractional one, never as strings, and at an equal instant the member kept
   later. Each signal is decided apart. No purge, `expire-outputs`, retention, reset time, age,
   retirement or removal changes a signal, and a member dropped under IFC-052 keeps none.
3. **What `inspect.v1` shows** (IFC-053, OBS-002, OBS-006). An observed status gives, beside
   `state`, `newest` and `windows`, `signals` with each kept signal: `usageAllowed`, and under
   `paidUsage` `possible` and `inUse`; a part with none kept is absent. A signal has no `state`
   and never reads `unknown`, whatever its time or reset (OBS-002). An `unknown` status is as
   today, with no `signals`. Every status conforms to the core's `limit-status.v1`, and no other
   inspection, output, record or reply shows a signal.
4. **The store's format** (DL2). The signals are engine state beside `newest`, in
   `limits::Observations`, with `#[serde(default, skip_serializing_if = ...)]`: an entry without
   them reads as one that keeps no signal, and an entry that keeps none is written byte for byte
   as today, so memory equals disk and the format version stays 1. No package has sent a limits
   member (the Start check's eleventh printed line), so no store but one a test made holds an
   entry, and no rule fills signals for an older one. Opening or inspecting a store changes no
   entry.
5. **Nothing else changes** (IFC-051, IFC-052, IFC-053, SEC-015, DEC-094). The outcome, the reply
   and its repeats, the retained result, the decisions, every record and record ID are as today;
   the end record keeps the member whole, and no reply carries it. The check stays the core's
   schema and the repeated `id`, with no check of the engine's own. A read of a secret still
   ignores the member. The engine adds nothing up and compares only decision 2's times; no access,
   dispatch, hold or lifecycle state depends on a signal, and no inspection starts a call.
6. **The case and the row.** Remove `requests-limits-signals`'s entry from
   `conformance/expected-failures.yaml`, and change no case. In the row `DL8: records` of
   `engine/CONFORMANCE.md`, on its one line, after "`unknown` from its reset (IFC-053).", add:
   "From order 315, the newest report also keeps the `paidUsage` and `windowsOmitted` it gave, and
   the engine keeps beside it, as its own state, the newest value other than null of
   `usageAllowed` and of paid usage's `possible` and `inUse`, each with its time and source and,
   for paid usage, its reset time, which `inspect.v1` shows in `signals` and which no later null,
   time or purge clears (IFC-053, DEC-096)." In its evidence cell, after
   `` `requests-limits-latest`, `` add
   `` `requests-limits-paid-usage`, `requests-limits-signals`, ``, and after the link to
   `records.rs` and its comma add:
   "`limits_newest_keeps_paid_usage_and_left_out_windows` and
   `limits_signals_keep_the_newest_value_other_than_null` in [limits_tests.rs] (link to
   crates/core/src/limits_tests.rs), `limits_signals_read_from_an_earlier_engines_store` in
   [limits_tests.rs] (link to crates/shell/src/limits_tests.rs),". Write each "[text] (link to
   target)" as a real Markdown link: the text in square brackets, then at once the target,
   relative to `engine/`, in parentheses. Then run `python3 tools/promise_standing.py --check`,
   and `--write` where it fails.
7. **Tests.** Write them first, run them on the otherwise unchanged code and report each line.
   K is the key `io.github.shrav-k.conduit/limits`. Members are `limit-report.v1` values in the
   form of `report` in `limits_tests.rs`, with neutral IDs such as `example/all/300m`; times are on
   2026-11-21, in UTC unless written otherwise.
   - In `engine/crates/core/src/limits_tests.rs`, with its helpers, from `requests-limits-latest`'s
     state through step 17 (`limits_case(LATEST, login_project(), 17)`), in which `a`'s turns gave
     `usageAllowed` true at 13:01 and null at 13:00, and `b`'s false at 13:02:
     - `limits_newest_keeps_paid_usage_and_left_out_windows` (IFC-052, IFC-053; decisions 1 and
       5): a call on `a` whose member, at 13:05, gives `usageAllowed` false, `paidUsage`
       `{possible: true, inUse: null, resetsAt: <a time>}`, `windowsOmitted` 1 and one window
       `example/all/300m` (300 minutes, `usedPercent` null, `resetsAt` null, `reached`, scope
       null). Its reply holds no K; its end record holds the member unchanged and no
       `limitsDropped`; `a`'s and `login`'s `newest` hold that `paidUsage` and `windowsOmitted`
       1, and their windows list `example/all/300m` as reported among the case's windows by `id`.
       A second call on `a`, at 13:06, whose member names no window, gives `windowsOmitted` 2 and
       no `paidUsage`: `newest` holds `windowsOmitted` 2 and no `paidUsage` or `noWindows`, and
       the windows stay. Every status conforms to `limit-status.v1` (`first_core_failure` gives
       none). Today `newest` holds neither `paidUsage` nor `windowsOmitted`.
     - `limits_signals_keep_the_newest_value_other_than_null` (IFC-053, OBS-002; decisions 2 and
       3): first, `a`'s `signals` hold `usageAllowed` true from its 13:01 turn, `b`'s and
       `login`'s false from `b`'s 13:02 turn, and none holds `signals.paidUsage`. Then four
       members on `a`, kept in this order, each with no window and `noWindows` `no-event`; X is a
       time of yours. `paidUsage` is written (`possible`, `inUse`, `resetsAt`). After each,
       `a`'s signals and `newest` are:

       | Member | At | `usageAllowed` | `paidUsage` | Allowed | Possible | In use | Newest |
       |---|---|---|---|---|---|---|---|
       | M1 | 13:05 | null | (true, null, X) | 13:01 turn's | M1's, X | none | M1 |
       | M2 | 13:04 | false | (false, true, null) | M2's | M1's | M2's, null | M1 |
       | M3 | `2026-11-21T15:04:00+02:00`, M2's instant | true | none | M3's, kept later | M1's | M2's | M1 |
       | M4 | 13:07 | false | (false, false, null) and `amount` 5: IFC-052 drops M4 | M3's | M1's | M2's | M1 |

       `login`'s follow the same rule over `b`'s turn and `a`'s members: `usageAllowed` false from
       M2, then true from M3, and `a`'s paid signals. Each signal names its member's `observedAt`
       as written, the end record's `source`, `a` as `instance` and the member's invocation. A
       `purge.v1` that removes every end record holding `limits`, as
       `limits_status_outlasts_a_purge_and_takes_the_later_of_equal_times` makes, and a serde
       round trip of the state leave every status as it was. Every status conforms to
       `limit-status.v1`. Today no status holds `signals`.
   - In `engine/crates/shell/src/limits_tests.rs`, a guard that passes on the unchanged code too:
     - `limits_signals_read_from_an_earlier_engines_store` (DL2; decision 4): in the store
       `established` leaves (s1's steps 1 to 13), in a temporary directory, close the shell;
       parse `state.json` as `store::Stored`, add a `limit_observations` entry keyed by
       `agent-b`'s ID, read from JSON with no `signals`: `newest` with an `observedAt`, the record
       source, `agent-b` in `object.v1` form as `instance`, an ID of yours as `invocation` and
       `usageAllowed` null, and `windows` `{}`; write it with `Store::write`, as
       `store_roundtrips_document_depth_plus_state_wrappers` does, and open the store again. It
       opens; root's inspection of `agent-b` (its route yours) shows that `newest`, no window and
       no `signals`, and conforms to `limit-status.v1`; memory equals disk
       (`assert_memory_equals_disk`); and the entry, in memory and in `state.json`, holds no
       `signals` after the inspection and after a second open.
   - An existing test changes only where it compares a whole status that now gains `signals`;
     none is expected to. Report each.

## Start check

From the repository root:

```sh
k() { f=$1; shift; for p in "$@"; do grep -cF -- "$p" "$f"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
grep -c '^## DEC-096, ' docs/decisions/log.md  # 1
sed -n '/^## DEC-096, /,/^## DEC-097, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
sed -n '/^  name: limit-report.v1$/,/^---$/p' spec/core/results.yaml | grep -cE '^    (paidUsage:$|windowsOmitted: )'  # 2
sed -n '/^  name: limit-status.v1$/,/^---$/p' spec/core/results.yaml | grep -cE '^    (signal|paidSignal|signals):$'  # 3
grep -c '"id": "requests-limits-signals"' conformance/expected-failures.yaml  # 1
k engine/crates/core/src/limits.rs 'pub struct Observations' 'newest["noWindows"] = reason.clone();' 'fn keep_latest' 'signals' 'paidUsage' 'windowsOmitted'  # 1 1 1 0 0 0
k engine/CONFORMANCE.md '`unknown` from its reset (IFC-053).' 'From order 315,'  # 1 0
grep -cE '^\| IFC-053 \| interface \| must \| 2 \| 1 \| outside \| core \| case passes \|$' engine/STANDING.md  # 1
cat engine/crates/core/src/limits_tests.rs engine/crates/shell/src/limits_tests.rs | grep -cE 'fn limits_(newest|signals)_'  # 0
grep -rlE 'usageAllowed|conduit/limits' packages | wc -l | tr -d ' '  # 0
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"  # engine 0
```

Where a line prints otherwise, stop, change nothing and report it. A fourth or fifth printed line
other than `2` and `3` means order 307's spec is not in the tree; a seventh printed line other
than `1 1 1 0 0 0` means `limits.rs` moved since this order was written; an eleventh other than
`0` means a package sends a limits member, and decision 4 no longer holds.

## Scope

Change `engine/crates/core/src/limits.rs` for decisions 1 to 5, and the core tests, which go in
`limits_tests.rs`; `engine/crates/shell/src/limits_tests.rs`; the one entry of
`conformance/expected-failures.yaml`; the row `DL8: records` of `engine/CONFORMANCE.md`; and
`engine/STANDING.md` by the tool alone. Change an existing test only under decision 7. Change
nothing else: not `spec/`, a case, `docs/` (DEC-094 and DEC-096 stay not done), core
`requests.rs`, `records.rs`, `lib.rs` or `schemas.rs`, the shell's other sources, the loader, the
conformance crate, `packages/`, `tools/`, a `Cargo.toml` or `Cargo.lock`. No promise changes, so
`spec/CHANGES.md` gains no line. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after, with `k` as in the Start check; the
second, guards, passes today and after. Run each existing test you changed by name, once, too.
Run the core's tests and then the whole vectors test, once each, last; never the whole engine
suite.

```sh
ok='^test result: ok\. 1 passed'
for t in limits_newest_keeps_paid_usage_and_left_out_windows limits_signals_keep_the_newest_value_other_than_null; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 twice; 0 today
(cd engine && cargo test --offline --release -p conduit-shell --lib limits_signals_read_from_an_earlier_engines_store 2>&1 | grep -c "$ok")  # 1; 0 today, before it is written
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-limits-signals 2>&1 | grep -c "^PASS requests-limits-signals ")  # 1; 0 today
grep -c '"id": "requests-limits-signals"' conformance/expected-failures.yaml  # 0; 1 today
k engine/CONFORMANCE.md 'From order 315,' '`requests-limits-signals`'  # 1 1; 0 0 today
grep -cE '^\| IFC-053 \| interface \| must \| 2 \| 0 \| outside \| core(, shell)? \| case passes \|$' engine/STANDING.md  # 1; 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-shell a_secret_read_ignores_its_limits_in_its_invocation_end 1  # 1
s conduit-shell memory_equals_disk_after 5  # 1
for p in requests-limits-kept requests-limits-latest requests-limits-paid-usage requests-usage-kept requests-login-resource- records-purge- secret-reads-; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # "N cases passed", N in order: 1 1 1 1 2 2 11
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, twice
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/limits(_tests)?\.rs|engine/crates/shell/src/limits_tests\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/expected-failures\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

Your report gives, per decision, what changed and where; each new core test's failing line on the
unchanged code, then its passing line; the shell test's line on the unchanged code and after;
`requests-limits-signals`'s line before and after; each existing test you changed, and why; your
own choices; and every check's last line. Stage nothing.

## What follows

- **The guard's next orders** (DEC-094, DEC-095, DEC-123): the spec's admission report and hook
  (order 314), then the engine orders that keep a login's report and hold a call on it.
- **The packages** (DEC-124): each login package's hook read, and the next Claude Agent Type from
  its turns' events, report `paidUsage`, `windowsOmitted` and null percentages only from fields
  its client gives, on new Type versions, after this order merges, so decision 4 keeps holding.
- **The CLI** shows a login's `signals` and paid usage beside its windows.
- DEC-094 and DEC-096 stay not done until the guard's orders merge, and no team-seat Agent takes a
  turn before then (DEC-102).

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, stacked on order 307. The usage
guard's engine order E0 (DEC-166 puts the guard's orders first). Written, built, read and fixed
in stage 3 of experiment 028; it is no order of stage 3.

**Worker-2's turn** (16:51 to 17:10) kept the report's paid usage and left-out windows and the
newest non-null value of each signal, so order 307's signals case passes; every check passed.

**Two blind reads** (17:12 to 17:30). The Codex verifier, `root-verifier-2.turn.v1`, passed it
with no finding; the Claude reader (`reader-1`) had no must, a should (what a signal's time is)
and a nit. One fix turn by worker-2 applied both. The gate's run on the squash is the
coordinator's, at the merge.
