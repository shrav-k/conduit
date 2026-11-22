# Work order 262: the engine keeps a call's usage in the record of its end (IFC-051), and drops a malformed one with a warning

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave `vectors`
empty, since no case changes. This order needs no network, and every crate it builds is cached: run
cargo offline, and if a crate is missing, stop and report, and fetch nothing. Run one cargo command
at a time. Never print a whole large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n`
or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential, even one a test made.
Do not run `conduit` outside a test, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and
sign in nowhere: every implementation a test meets is scripted. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user, machine
or account name, or private repository name.

## What and why

**What.** [DEC-047](../decisions/log.md#dec-047-2026-11-21-usage-in-records-and-harnesses-in-role-templates)
logs the owner's answer on usage, "Engine keeps them now (Recommended)". Order 261 writes it into
the spec: IFC-051, the core schema `usage.v1`, `record.v1`'s `usage` and `usageDropped`, REC-002's
sentence, and the case `requests-usage-kept`, listed as an expected failure that this order makes
pass. This order builds IFC-051 in the reference engine, plain and keyed alike. Order 260 has both
Agent packages fill the member; DEC-047 is done once both are.

**Today.** `requests::complete` reads a result's outcome (IFC-009, IFC-010), and `finish` writes the
`invocation-ended` record with the outcome and the outputs `records` keeps. Nothing reads `_meta`'s
`io.github.shrav-k.conduit/usage`: the record holds none, and a malformed one draws no warning.
`envelope` builds the reply's `_meta` anew, so the member already never reaches the caller, and
`requests-usage-kept` passes but for its record assertions (order 261's report). The shell hands a
call's result to the core whole (`runner.rs`), and `export-records`, behind `conduit records` and
`serve`'s records route, returns the stored records whole: neither needs a change. `records.rs`
writes the ends of the engine's own operations, which no implementation answers: unchanged.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7, as
model work through Conduit is attributed with what it used, in the record of the call that used it;
4, as the member names its client and model and assumes no vendor. None at risk: no outcome, reply,
decision or other record changes, and the numbers permit nothing (2, 6).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes: IFC-051 is a must, and DEC-047 asks for it now. Existing practice gives the form:
OpenTelemetry's rule that telemetry never fails the work it describes, so a malformed member is
dropped with a warning; the loader's SUB-005 diagnostics give the warning's pointer and
`keywordLocation`; and `finish_login_lapse` already completes an `invocation-ended` record after
`finish` (its `pendingAction`). The least change reads the member once, in `complete`, and writes it
into that record, leaving `finish` and its callers alone. Not taken: a copy of the member on
the invocation's state (a second place that DL2 writes and expiry must clear); checking it in the
shell (the core reads results); a record of its own (IFC-051 names the end record); relaying the
warning to the caller (nothing else about the call changes).

**When it runs.** Order 261 must have merged, its spec and its case, not its text alone: the engine
checks the member against the `usage.v1` that the shell loads from `spec/core/`, and removes 261's
expected failure. The start check stops otherwise. Through Conduit's dogfood: one Codex worker turn
(a 4-hour limit; aim for under 2 hours), then a Codex verifier and a Claude reader, who read it blind
at once and run only what settles a doubt, one fix turn, and the gate, which runs everything
(DEC-030, DEC-046). It may run beside orders 255 and 258 and the S1 chain (orders 243 to 250). The
hot files it shares are core `requests.rs`, where it changes the head and tail of `complete` alone,
and core `lib.rs` and shell `tests.rs`, one `mod` line each: its code and tests go in files of
their own. It leaves `records.rs`, `secrets.rs` and `finish` alone. It also shares
`engine/CONFORMANCE.md` with 255 (DL8 here, DL10 and DL2 there) and `engine/STANDING.md` with
every engine order: whichever merges second rebases and runs `promise_standing.py --write` again.
By ruling 89, each decision says what must hold; how is yours. The coordinator's, open to the
owner's strike: the test names, the warning's words, that the schema is the engine's own core's,
and decision 1's readings of which ends keep a member.

**Stop rule.** Change nothing under `spec/`, and under `conformance/` only the one expected-failures
entry. Stop, change nothing further, and report where:

- IFC-051's wording needs a choice that `spec/` does not make: report the promise, the choice and
  the options; do not choose;
- a correct result needs a change outside the scope;
- `requests-usage-kept` fails, another vector fails, `vector_delta.py` shows a change, or a test that
  decision 6 does not name fails;
- a crate is missing from the cache, or a step needs the network, a real client or a sign-in.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **Which ends keep it** (IFC-051, REC-002). A `complete` input whose completion is a result
   (`Completion::Result`), and whose step writes the named invocation's `invocation-ended` record,
   keeps that result's member in that record, plain or keyed, a hook's call and a read of a secret
   among them. The member is the value in the result's `_meta` under the key `c.meta_prefix`
   followed by `usage`. Only that record changes: no `request-ended`, no record of the setup,
   retirement or request that the call serves, no record of another step. The coordinator's
   readings: the answer to an invocation whose `discovery` is pending, or to a secret read in its
   `discover` phase, is no result of the call and keeps none; a reason the engine accepts
   (IFC-026, ACC-008, ACC-014) and a late or duplicate completion write no `invocation-ended`, so
   keep none; a reason it reads as unknown keeps the member, as any unknown end does; and a
   cancellation's confirmation, an end that OPS-027 decides, keeps none.
2. **Kept or dropped** (IFC-051). The member is checked against `usage.v1` of the engine's own core
   (`c.core_environment`), never of the target's held scope: the member is Conduit's, not the
   package's. Where it conforms, the record's `data.usage` is the value, unchanged. Where it does
   not, `data.usageDropped` is decision 3's warning, and the record holds no part of the value. The
   member never fails the step: where the core holds no `usage.v1` or the check errs, the member is
   dropped with the warning's second form.
3. **The warning** (IFC-051, SUB-005, ruling 52). One diagnostic: `ruleId` `IFC-051`; `level`
   `warning`; one logical location whose `fullyQualifiedName` is the JSON Pointer of the failing
   place in the result, the key's own pointer, from `c.meta_prefix`, followed by the place in the
   member (for the case,
   `/_meta/io.github.shrav-k.conduit~1usage/models/example-model-b/output`); in `properties`, the
   failing keyword's `keywordLocation`, counted from `usage.v1`'s schema as the loader's
   `locate_keywords` counts SUB-005's (for the case,
   `/properties/models/additionalProperties/properties/output/minimum`); and `message.text`, "The
   result's usage member was dropped: at P it does not conform to usage.v1, failing its W keyword.
   The record of this invocation's end keeps no part of it, and nothing else about the call
   changes (IFC-051).", where P is that pointer and W the last token of `keywordLocation`. Its
   second form, where the engine could not check the member, has the key's own pointer, no
   `keywordLocation` and the text "The result's usage member was dropped: the engine could not
   check it against usage.v1. The record of this invocation's end keeps no part of it, and nothing
   else about the call changes (IFC-051)." One failure is named, the first in instance-path order,
   as the loader's `first_failure` takes it. No value is quoted: not the member's, not the
   validator's message.
4. **Nothing else changes** (IFC-051, REC-002, CTR-004). The outcome, its rule and cause, the reply
   and its repeats, the retained result, the transition's decisions, every other record and every
   record ID are what they would be without the key: keeping or dropping allocates no ID and adds no
   record and no decision, and the engine sums, compares and acts on no number. No reply carries the
   member or the warning. The member is no output: the Type's `records` does not gate it, and
   `expire-outputs` leaves it. Memory equals disk after the commit (DL2): the step's record and the
   State's copy change together, as `finish_login_lapse` changes `pendingAction`.
5. **One way** (the least change). A new `engine/crates/core/src/usage.rs`, declared by one line in
   core `lib.rs`: one function takes the member from a completion, one checks it and writes it into
   the step's ending record. `requests::complete` takes the member and the state decision 1 reads
   at its head, before the secret read's hand-off, and hands them over once its body returns, by
   whichever path; renaming the body to an inner function is one way. `Environment::failure_at`
   in core `schemas.rs` and its callers stay as they are; where it cannot give decision 3's
   failure, one new `pub(crate)` function beside it may, taking the first in instance-path order
   and its `keywordLocation` as the loader's `first_failure` and `locate_keywords` do.
6. **Tests**, with scripted implementations only, in a new `engine/crates/shell/src/usage_tests.rs`,
   declared in `tests.rs` as `fixtures.rs` is and using its helpers. Write them first, run them on
   the otherwise unchanged code and report each failing line. K is `format!("{}usage",
   shell.context.meta_prefix)`; P0 is `/_meta/` then K, `~` escaped as `~0` and `/` as `~1`; U is
   261's,
   `{"client":{"name":"example-client","version":"1.0.0"},"models":{"example-model-a":{"input":1200,"cachedInput":800,"output":300,"reasoning":120},"example-model-b":{"input":40,"output":7}}}`.
   - `a_plain_result_keeps_its_usage_in_the_end_record`: on `established`, agent-b's `invoke` of
     `project.file.write.v1`, twice, with distinct keys and a `Scripted` executor: answered once by
     a result whose `_meta` holds K: U and once by the same result without it. Both replies are
     equal but for the invocation's ID, and neither holds K. In `export-records`, the first
     invocation's `invocation-ended` holds U in `data.usage` and no `usageDropped`, the second's
     holds neither, and no other record holds either. With the shell dropped and the store opened
     again, the export is the same. Today the `usage` assertion fails.
   - `a_keyed_result_keeps_its_usage_in_the_end_record`: on `filtered_shell`, a `filtered_read`
     begun with `Deferred` and completed by `complete_read`'s result with K: U in its `_meta`: the
     reply is what `complete_read` asserts, without K, and the read's `invocation-ended` holds U.
     Today it holds none.
   - `a_secret_read_ignores_its_usage_in_its_invocation_end`: set up as
     `a_read_whose_dispatch_record_cannot_be_kept_is_not_dispatched` does, with no fault and the
     read's result carrying K: U: the read's `invocation-ended` holds U, and its `request-ended`
     and every other record hold neither member. Today it holds none.
   - `a_malformed_usage_is_dropped_with_a_warning`: on `established`, as in the first test, one
     call answered without K and three with K: the string `synthetic-usage-value`; U with a
     top-level `"account":"synthetic-account-label"`; and `{"client":{"name":"","version":"1.0.0"}}`,
     which fails at `/client/name` and, first in path order, at the root. Each of the three replies
     equals the first's but for the invocation's ID; its `invocation-ended` holds no `usage`, and
     holds `usageDropped` with `ruleId` `IFC-051`, `level` `warning`, the pointer P0,
     `keywordLocation` `/type`, `/additionalProperties` and `/required` in turn, and decision 3's
     words; and `synthetic-` is in no record, no reply and nowhere in the State. Today
     `usageDropped` is absent.
   - Change no other test.
7. **The case and the documents.** Remove `requests-usage-kept`'s entry from
   `conformance/expected-failures.yaml`; change no case. In `engine/CONFORMANCE.md`'s DL8 row, after
   "No live record-feed cursor is implemented.", add "From order 262, an `invocation-ended` record
   holds in `usage` the usage member that its call's result carried, plain or keyed, or in
   `usageDropped` the IFC-051 warning that dropped a malformed one; no reply carries either, and no
   number decides anything.", and add `` `requests-usage-kept`, `` to its evidence cell after
   `` `setup-review-provider-records`, ``. Then run `python3 tools/promise_standing.py --write`.

## What to read

IFC-051 and REC-002 as order 261 has them, `usage.v1`, `record.v1` and `diagnostic.v1`; IFC-009,
IFC-010, IFC-025, IFC-026, SUB-005, CTR-004 and OPS-027; order 261's text, its decision 8 above all.
The core's `requests.rs` (`complete`, `finish`, `finish_login_lapse`, `complete_discovery`, `cancel`,
`envelope`), `secrets.rs` (`complete`, `end`), `records.rs` (`export`), `schemas.rs` (`failure_at`)
and `lib.rs` (`Allocation::record`); the loader's `schemas.rs` (`locate_keywords`); the shell's
`tests.rs` (`established`, `Scripted`, `invoke`, `filtered_shell`, `filtered_read`, `Deferred`,
`complete_read`) and `runner.rs` where a result becomes a completion.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
git merge-base --is-ancestor 5a6451e7 HEAD; echo "main $?"
test -f docs/work-orders/261-usage-in-the-end-record.md; echo "order $?"
grep -c '^- id: IFC-051$' spec/interface/implementation.yaml
grep -c '^  name: usage\.v1$' spec/core/results.yaml
j spec/core/results.yaml 'usageDropped: {$ref: github.com/shrav-k/conduit/core#diagnostic.v1}'
grep -c '"id": "requests-usage-kept"' conformance/expected-failures.yaml
test -f conformance/conventions/requests/usage-kept/case.yaml; echo "case $?"
git grep -l -e 'IFC-051' -e 'conduit/usage' -e 'usage\.v1' -e 'usageDropped' -- engine/crates | wc -l | tr -d ' '
test -e engine/crates/core/src/usage.rs || test -e engine/crates/shell/src/usage_tests.rs; echo "new $?"
grep -c 'No live record-feed cursor is implemented.' engine/CONFORMANCE.md
grep -c '^#\[path = "fixtures.rs"\]$' engine/crates/shell/src/tests.rs
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

Once order 261 has merged they print `main 0`, `order 0`, `1`, `1`, `1`, `1`, `case 0`, `0`,
`new 1`, `1`, `1`, `engine 0` and `0`. On `main` at `5a6451e7` the second prints `order 1`, and with
261's text alone the third to seventh print `0` or `case 1`: 261 has not run; stop.

## Scope

Change `engine/crates/core/src/usage.rs` (new), core `lib.rs` (its `mod` line), `requests.rs`
(`complete` alone) and `schemas.rs` (decision 5's function, if needed); `engine/crates/shell/src/tests.rs`
(its `mod` line) and `usage_tests.rs` (new); one entry of `conformance/expected-failures.yaml`;
`engine/CONFORMANCE.md`'s DL8 row; and `engine/STANDING.md` only by the tool. Change nothing else:
not `spec/`, a case, `docs/` (DEC-047 stays not done), `packages/`, `tools/`, the loader, another
core or shell source (`records.rs`, `secrets.rs`, the shell's `lib.rs`, `main.rs`, `runner.rs` and
`server.rs` among them), `engine/crates/shell/tests/`, any `Cargo.toml` or `Cargo.lock`. Report
anything else.

## Done when

Each line of the first group prints otherwise on `main` once order 261 has merged ("today") and
passes after. The second group are guards, which pass today and must still pass. Run the last line,
the whole engine suite, once, last.

```sh
ok='^test result: ok\. 1 passed'
t() { (cd engine && cargo test --offline --release -p conduit-shell --lib "$1" 2>&1 | grep -c "$ok"); }
t a_plain_result_keeps_its_usage_in_the_end_record  # 1; today 0
t a_keyed_result_keeps_its_usage_in_the_end_record  # 1; today 0
t a_secret_read_ignores_its_usage_in_its_invocation_end  # 1; today 0
t a_malformed_usage_is_dropped_with_a_warning  # 1; today 0
grep -c '"id": "requests-usage-kept"' conformance/expected-failures.yaml  # 0; today 1
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-usage-kept 2>&1 | grep -c '^PASS requests-usage-kept ')  # 1; today 0
grep -c 'From order 262,' engine/CONFORMANCE.md  # 1; today 0
grep -cE '^\| IFC-051 \| interface \| must \| 1 \| 0 \| outside \| core(, shell)? \| case passes \|$' engine/STANDING.md  # 1; today 0
grep -cE '^\| REC-002 \| convention \| must \| [0-9]+ \| 0 \| cases \| [a-z, ]+ \| case passes \|$' engine/STANDING.md  # 1; today 0
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-shell memory_equals_disk_after 5  # 1
s conduit-shell retention_cap_oldest_first_and_age_survive_restart 1  # 1
s conduit-shell first_filter_rejection_delivers_core_reply_to_ticket 1  # 1
s conduit-core plain_effectful_error_result_names_ifc010 1  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
n=$(git ls-files -o --exclude-standard -- engine); [ -z "$n" ] || git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(usage|lib|requests|schemas)\.rs|engine/crates/shell/src/(tests|usage_tests)\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/expected-failures\.yaml)$' | wc -l | tr -d ' '  # 0
[ -z "$n" ] || git reset -q -- $n  # stage nothing
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # every test and vector passes
```

Your report gives, per decision, what changed and where; each new test's failing line, then its
passing line; the case's line before and after; your own choices; and every check's last line.

Stage nothing.

## What follows

Named here and not in this order:

- Order 260: both Agent packages fill the member by model, and report a limit's class, window and
  reset in the result. DEC-047 is done once 260 and this order are.
- The metrics batch (DEC-016; rulings 68 and 69), which maps the member to OpenTelemetry's
  `gen_ai.usage.*` and reads it from the records; the safety catalogue's row for IFC-051.
- A cancellation's confirmation keeps no member (decision 1): whether IFC-051 reaches an end that
  OPS-027 decides is a later reading.
- From order 261: a failed result of an implementation that carries Conduit's keys loses its own
  diagnostics (`requests.rs`, the keyed branch); whether IFC-025's "`diagnostics` apart" passes
  them on is for later.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with order 261 stacked as one commit while 261 waited in the merge queue.

**Worker-1's turn** (00:27 to 01:11) built IFC-051 as order 261 merged it, with the coordinator's
amendments during 261's build (open to the owner's strike): every read of a secret is excluded,
the engine checks the member's shape only, and a malformed member's warning stops at the last
member whose name the schema fixes, with its `keywordLocation`. So decision 6's secret-read test
is `a_secret_read_ignores_its_usage_in_its_invocation_end`. Its four tests failed first and
pass; `requests-usage-kept` passes all its steps and leaves the expected failures. The whole
engine suite failed once at `s2_dependent_call_after_close`, which this order does not reach and
which failed the same way in orders 261's and 257's runs under the machine's load; order 268
makes it independent of load.

**Two blind reads at once** (01:13 to 01:20), the owner's answer for A77's window:
`turn-verifier-1.turn.v1` passed it, with 168 core tests and the promoted case; the Claude
reader passed it with two nits. No fix turn: the renamed test is recorded here, and a core test
of the warning's second form (no `usage.v1` to check against) is left to a later order.

The gate's run on the squash is the coordinator's, at the merge.
