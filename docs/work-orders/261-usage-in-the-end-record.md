# Work order 261: a call's usage is kept in the record of its end (IFC-051), ahead of the metrics batch

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the case this order adds, an expected failure. It needs no network and changes no engine
code; if a crate is missing from the cache, stop and fetch nothing. Run one cargo command at a
time, and not `tools/gate.py`. Never print a whole large file, case, diff or list: use `head`,
`tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.
Run no `conduit` outside a test, no `codex` or `claude`, and sign in nowhere. Prefix each
`python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user, machine or account name, or private repository name.

## What and why

**What.** [DEC-047](../decisions/log.md) logs the owner's answer on usage, "Engine keeps them
now (Recommended)": each turn's result carries the tokens its client reported, the model and the
client version, the engine keeps them in that call's end record, and numbers decide nothing in
the engine. It amends [DEC-016](../decisions/log.md#dec-016-2026-11-20-metrics)'s order, the
metrics batch, for this one measure, and gives ruling 92 point 4's "every run records its
tokens" its record (HOW-WE-WORK rule 4). This order writes the spec; orders 262 and 260 follow.

**The shape** is [ruling 68](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-68/README.md#the-decisions)'s
M1: an Agent reports what only it knows "in one `_meta` key", and the engine "checks the
report, takes whose it is from its own record, and keeps it in that call's end record"; a bad
report "is dropped and the drop recorded" (its proposal).
Here the declaration is one core schema, `usage.v1`; the record's target says whose the numbers
are, so the member names no account; per-package declarations and the account attribute
stay the batch's ([69](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-69/README.md#the-decisions)).

**Today.** No key of Conduit's names usage. The engine reads a result's outcome, reason and
restriction, or a `plain` one's `isError` and `content` (IFC-009, IFC-010); an end record holds
the outcome and the outputs `records` keeps (REC-002). Both Agent packages are
`plain`, and in stage 2 "No result reports tokens"
([the results](../../experiments/028-a78-dogfood/results-2026-11-21-stage-2.md)). A failed
`.v2` turn names its class, `usage-limit` among them, in its `content`; order 260 adds the
window and reset time that DEC-046's pause reads from the result.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1):
serves 7, as model work through Conduit is attributed with what it used; 4, as the member names
its client and fixes what each count means. None at risk: the numbers permit nothing (2, 6).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice; REC-002 and 68's M1 make the record Conduit's. Model APIs report usage
with each response: OpenAI's Responses API, whose `input_tokens` hold `cached_tokens` and
`output_tokens` hold `reasoning_tokens`, the meanings the member takes; Anthropic's Messages
API; Codex's `turn.completed`; Claude Code's `result`, whose `modelUsage` is keyed by model, as
the member is. OpenTelemetry's GenAI conventions name the same counts (`gen_ai.usage.*`), for
the metrics batch to map (68 M4); its API does not throw when used incorrectly: telemetry never
fails the work it describes, so a malformed member is dropped with a warning. Not taken: the
options DEC-047 declined; relaying the member to the caller (68's v0 default); a field for the
limit.

**When it runs.** Through Conduit's dogfood: one Codex worker turn; a Codex verifier and a
Claude reader read it blind at once, running only what settles a doubt; one fix turn; the gate
(DEC-030, DEC-046). By ruling 89, each decision says what must hold; how is yours. The key's
name, the per-model map, the counts' meanings, `usageDropped`, the case and a limit's lack of a
field are the coordinator's, open to the owner's strike (HOW-WE-WORK rule 6).

**Stop rule.** Change only the files in Scope. Stop, change nothing further, and report where
the set wording contradicts a promise no decision changes (give both), a check refuses a line
as given (give its error), the case fails on the unchanged engine without its record assertions
or, with them, other than at the first, another case or test fails, a step needs an engine
change, or the start check prints otherwise. A choice no decision governs is yours: list it
under `own_choices`.

## The decisions

Wrap each insertion at its file's width.

1. **IFC-051,** in `spec/interface/implementation.yaml`, after IFC-027:

   ```yaml
   - id: IFC-051
     layer: interface
     level: must
     when: >-
       An implementation, whether or not it is marked `plain`, ends a call for a request or a
       hook with a result from which its outcome is read, under IFC-009 or IFC-010, and the
       result's `_meta` holds `io.github.shrav-k.conduit/usage`.
     then: >-
       Where its value conforms to the core schema `usage.v1`, the record of the invocation's
       end holds that value, unchanged, in `usage`. Where it does not, that record holds no part
       of it, and holds in `usageDropped` one diagnostic whose `level` is `warning` and whose
       `ruleId` is the ID of this promise. Nothing else about the call changes.
     observable: >-
       The value says what the call's model work used, by client and model, with the meanings
       that `usage.v1` gives its counts. An implementation may carry it on any such result, one
       that states failed or unknown among them, and answers for its numbers: the engine checks
       their form against `usage.v1` and never their truth. The warning says that the value was
       dropped and why: it names the failing place by its JSON Pointer in the result, carries in
       `properties` the failing keyword's `keywordLocation`, as SUB-005 has it, and quotes no
       value. The call's outcome, its result, its other records and every decision are what
       they would be without the key: nothing depends on it, and the engine sums, compares and
       acts on none of its numbers. The result that the engine gives the consumer does not
       carry it. The record holds it whatever the type's `records` states, since the value is
       no output (CTR-004), and the record is read as any other (LIF-013). The value names no
       account: the record names the target whose call it was (REC-002). A call that ends on its
       client's usage limit is recorded as any other ending: the limit's class, window and time
       of reset are the implementation's own report in its result, and are no part of this value.
   ```

2. **REC-002's observable,** after "and the outputs where `outputs` is true.": "The record of an
   invocation's end also holds the usage member that its result carried, or the warning that
   dropped one, under IFC-051, whatever `records` states." `spec/definitions.md`'s row
   "Conduit's keys" gains after "conflict (IFC-025).": "A result, plain or not, may also carry
   `usage`, kept in the record of the invocation's end and never relayed (IFC-051)."

3. **`usage.v1`,** in `spec/core/results.yaml` after `record.v1`:

   ```yaml
   ---
   apiVersion: github.com/shrav-k/conduit/v0
   kind: Schema
   metadata:
     name: usage.v1
     description: >-
       What one call's model work used, as a result reports it under
       `io.github.shrav-k.conduit/usage` and the record of its end keeps it (IFC-051): the
       client and its version, and, keyed by each model's name as the client gives it, or as
       the implementation asked for it where the client names none, its tokens: `input`, every
       input token, cached ones among them; `cachedInput`, those of `input` read from the
       client's cache; `output`, every output token, reasoning among them; and `reasoning`,
       those of `output` spent on reasoning; `cachedInput` and `reasoning` where the client
       reports them. A reader adds neither part to its whole. Nothing in it names an account
       or is a secret.
   spec:
     type: object
     properties:
       client:
         type: object
         properties:
           name: {type: string, minLength: 1, maxLength: 256}
           version: {type: string, minLength: 1, maxLength: 256}
         required: [name, version]
         additionalProperties: false
       models:
         type: object
         minProperties: 1
         maxProperties: 16
         propertyNames: {minLength: 1, maxLength: 256}
         additionalProperties:
           type: object
           properties:
             input: {type: integer, minimum: 0}
             cachedInput: {type: integer, minimum: 0}
             output: {type: integer, minimum: 0}
             reasoning: {type: integer, minimum: 0}
           required: [input, output]
           additionalProperties: false
     required: [client, models]
     additionalProperties: false
   ```

   - `record.v1`'s `data.properties` gain, after `outputs`,
     `usage: {$ref: github.com/shrav-k/conduit/core#usage.v1}` and
     `usageDropped: {$ref: github.com/shrav-k/conduit/core#diagnostic.v1}`; its description
     ends "An invocation's end record holds in usage its result's member, and in usageDropped
     the warning that dropped one (IFC-051)."; the head's list gains `usage.v1`, IFC-051.
   - `spec/core/conduit.yaml` exports `usage.v1` after `record.v1`. `spec/core/README.md`: "11
     Schema contracts: an object as a result names it, a diagnostic, a condition, a record,"
     becomes "12 Schema contracts: an object as a result names it, a diagnostic, a condition, a
     record, a call's usage,"; after `record.v1`'s row, "| `usage.v1` | Schema | IFC-051: what
     one call's model work used, by client and model |".

4. **A usage-limit ending** gains no field: IFC-051's last sentence says how it is recorded.

5. **`spec/README.md`, Contents:** the `interface/implementation.yaml` row reads "Draft; 46
   promises; vectors test slices of 24 promises"; the `core/results.yaml` row names `usage.v1`
   after `record.v1` and "a call's usage" after "a record", and reads "Draft; 12 contracts";
   the totals sentence gains one promise in all and one of the layer `interface`.

6. **`spec/CHANGES.md`,** under `## [Unreleased]`, after its last item:

   ```text
   - Added: IFC-051, a result, plain or not, may carry one usage member in `_meta`, which the record of the invocation's end keeps unchanged, or drops with a warning naming IFC-051 where it is malformed; it changes nothing else (DEC-047).
   - Added: `usage.v1`, the core schema of IFC-051's member, and `record.v1`'s `usage` and `usageDropped` (DEC-047).
   - Changed: REC-002, the record of an invocation's end holds its usage member or the warning that dropped it, under IFC-051, whatever `records` states (DEC-047).
   ```

7. **IN_FORCE.md, topic 7.** The line beginning "*Spec pending, the metrics batch after batch
   2*" ends, on the same line: "One measure is in spec/ ahead of the batch, as DEC-047 amends
   DEC-016's order: a call's usage, as its implementation reports it, kept in the record of its
   end and deciding nothing (IFC-051)." No bullet is added.

8. **The case,** `requests-usage-kept`, at `conformance/conventions/requests/usage-kept/`,
   listed in that directory's `index.yaml` with `promises` `["IFC-051", "REC-002"]`; layers
   `convention` and `interface`, profile `first-build`, modelled on
   `requests-plain-model-work`: a `plain` implementation of an operation marked
   `startsModelWork: true`. Let U be
   `{"client":{"name":"example-client","version":"1.0.0"},"models":{"example-model-a":{"input":1200,"cachedInput":800,"output":300,"reasoning":120},"example-model-b":{"input":40,"output":7}}}`.
   Three requests end with a plain result whose `_meta` holds `io.github.shrav-k.conduit/usage`:
   - U on a complete result: it ends succeeded, its reply is what it would be without the key,
     with no `/reply/_meta/io.github.shrav-k.conduit~1usage`, and its `invocation-ended` record
     holds U in `usage`;
   - U on an error result: it ends unknown under IFC-010, and its record holds U in `usage`;
   - U with `"output": -1` for `example-model-b`, on a complete result: it ends succeeded; its
     record holds no `usage`, and holds `usageDropped` with `ruleId` `IFC-051`, `level`
     `warning` and the location
     `/_meta/io.github.shrav-k.conduit~1usage/models/example-model-b/output`.

   Each step's `required_promises` names only what the unchanged engine cites, as the model
   case's do (IFC-010, IFC-025 for a request), never IFC-051.
   Write it first without the records' `usage` and `usageDropped` assertions: it passes on the
   unchanged engine, so the key changes no outcome, reply or other record; report that line.
   Then add them, and list the case in `conformance/expected-failures.yaml`, in `id` order,
   with the reason "IFC-051: the first build keeps no usage member in the record of an
   invocation's end and drops none with a warning; order 262 makes it pass and removes this
   entry"; the runner reports an expected failure naming the first request's missing `usage`.
   Declare the case in `docs/work-orders/261-declared.yaml` (`kind: added`), as
   [244's](244-declared.yaml) does. Then run `python3 tools/promise_standing.py --write`.

## What to read

IFC-009, IFC-010, IFC-025, IFC-026, REC-002, CTR-004 and SUB-005; `record.v1`, `diagnostic.v1`;
[CONTRIBUTING.md](../../CONTRIBUTING.md#writing-promises), rule 8; `conformance/README.md`,
"Expected failures"; `requests-plain-model-work`, by step; `requests.rs`'s `Completion::Result`.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor 5a6451e7 HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
sed -n '/^## DEC-047/,/^## DEC-048/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
git grep -l -e 'IFC-051' -e 'conduit/usage' -e 'usage\.v1' -- spec conformance engine tools | wc -l | tr -d ' '
grep -c 'Draft; 45 promises; vectors test slices of 23 promises' spec/README.md
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/conventions/records.yaml 'and the outputs where `outputs` is true. A record has the form'
j IN_FORCE.md 'usage is not relayed to the calling Agent in v0; numbers decide nothing (68).'
grep -c '"id": ' conformance/expected-failures.yaml
test -f conformance/conventions/requests/plain-model-work/case.yaml; echo "model $?"
test -e conformance/conventions/requests/usage-kept; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
```

They print `main 0`, `0`, `1`, `0`, `1`, `1`, `1`, `3`, `model 0`, `case 1` and `engine 0`.
Another count than `3` on the eighth means another order changed the expected failures: carry
on, adding one.

## Scope

Change `spec/interface/implementation.yaml`, `spec/conventions/records.yaml`,
`spec/definitions.md` (one row), `spec/core/results.yaml`, `spec/core/conduit.yaml`,
`spec/core/README.md`, `spec/README.md` (Contents alone), `spec/CHANGES.md` and `IN_FORCE.md`,
as the decisions say; the new `conformance/conventions/requests/usage-kept/`, one entry of that
directory's `index.yaml` and one of `conformance/expected-failures.yaml`; the new
`docs/work-orders/261-declared.yaml`; and `engine/STANDING.md` by the tool. Change nothing else:
not `docs/decisions/log.md` (DEC-047 stays not done), the engine, `packages/`, `tools/` or
`experiments/` (IFC-051 is of the layer `interface`, so experiment 012's account takes no row);
report what else needs changing.

## Done when

Each line prints otherwise on `main` today and passes after:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
grep -c '^- id: IFC-051$' spec/interface/implementation.yaml  # 1; 0 today
k spec/interface/implementation.yaml 'holds `io.github.shrav-k.conduit/usage`.' 'holds in `usageDropped` one diagnostic whose `level` is `warning`' 'and are no part of this value.'  # 1 1 1; 0 0 0 today
j spec/conventions/records.yaml 'or the warning that dropped one, under IFC-051, whatever `records` states.'  # 1; 0 today
grep -c '^  name: usage\.v1$' spec/core/results.yaml  # 1; 0 today
k spec/core/results.yaml 'usage: {$ref: github.com/shrav-k/conduit/core#usage.v1}' 'usageDropped: {$ref: github.com/shrav-k/conduit/core#diagnostic.v1}'  # 1 1; 0 0 today
k spec/README.md 'Draft; 46 promises; vectors test slices of 24 promises' 'Draft; 12 contracts |'  # 1 1; 0 0 today
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '(DEC-047)\.$'  # 3; 0 today
j IN_FORCE.md 'One measure is in spec/ ahead of the batch, as DEC-047 amends'  # 1; 0 today
grep -c '"id": "requests-usage-kept"' conformance/expected-failures.yaml  # 1; 0 today
grep -cE '^\| IFC-051 \| interface \| must \| 1 \| 1 \| outside \| none \| expected failure \|$' engine/STANDING.md  # 1; 0 today
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-usage-kept 2>&1 | grep -c '^EXPECTED FAILURE requests-usage-kept ')  # 1; 0 today
```

Guards, which pass today and must still pass. Run the engine's suite once, last:

```sh
export PYTHONDONTWRITEBYTECODE=1
n=conformance/conventions/requests/usage-kept
git add -N "$n" docs/work-orders/261-declared.yaml
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0; its no-case ratchet shows head equal to base
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/check_model_account.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/set_names.py --check | tail -1  # ends Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/261-declared.yaml; echo "exit $?"  # ends 0 undeclared; 0 errors. then exit 0
test "$(git show HEAD:IN_FORCE.md | grep -cE '^\s*[-*] ')" -eq "$(grep -cE '^\s*[-*] ' IN_FORCE.md)"; echo "bullets $?"  # bullets 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(interface/implementation\.yaml|conventions/records\.yaml|definitions\.md|core/(results|conduit)\.yaml|core/README\.md|README\.md|CHANGES\.md)|IN_FORCE\.md|engine/STANDING\.md|conformance/(expected-failures\.yaml|conventions/requests/(index\.yaml|usage-kept/[^/]+))|docs/work-orders/261-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # every test passes, the new case an expected failure
git reset -q -- "$n" docs/work-orders/261-declared.yaml  # stage nothing
```

Report each changed sentence before and after; the case's steps; its line without the record
assertions and its lines with them; every check's last line.

## What follows

- Order 262: IFC-051 in the engine, plain and keyed alike, removing the expected failure.
  Order 260: both Agent packages fill the member by model, each turn's own use where a client
  reports a running total (68), Claude's `input` summing the cache reads and writes its client
  counts apart, and report a limit's class, window and reset in the result.
  DEC-047 is done once both are.
- The metrics batch (DEC-016; 68, 69); the safety catalogue's row for IFC-051, the proof lane's.
- Found while writing: a failed result of an implementation that carries Conduit's keys loses
  its own diagnostics (`requests.rs`, the keyed branch), so a keyed package's limit report would
  not reach the caller; whether IFC-025's "`diagnostics` apart" passes them on is for later.

## Record

Not yet run.
