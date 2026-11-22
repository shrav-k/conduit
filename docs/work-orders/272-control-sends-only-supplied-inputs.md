# Work order 272: a control is sent only the inputs its caller supplied, and no omitted input's default (order 270's finding)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` the one case you add, with the promises it tests.
This order needs no network: it reaches no host and fetches no crate. Every crate it builds is in
the local cache already: run cargo offline, and if a build asks for a crate that is not there,
stop and report, and fetch nothing. Run one cargo command at a time. Never print a whole large
file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Do not run `conduit` outside a test,
`npm`, `node`, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere.
Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no
absolute local path, user or machine name, or private repository name.

## What and why

**The finding.** Order [270](270-inspect-unknown-settings.md)'s worker: "Dispatch normalization
still inserts declared defaults for omitted control inputs." Order 270 worked around it: its
`status.settings` counts only the inputs the caller supplied (`control_inputs`, read by
`control_input_supplied` in `requests.rs`), and the call still carries the default.

**What `spec/` holds.** No promise is silent, so this order changes no spec text:

- REV-006: where a control's input "is not supplied", its setting "is left as stored. No default
  is applied to it."
- OPS-026 fills an omitted input's default only where "the operation is not a control"; its
  observable: "A control applies no default to an input that is not supplied [...]. A value that
  is supplied to a control is filled as any supplied value is." SCH-009 fills the absent
  properties of a supplied map, and REV-015 checks a control "once the defaults of the supplied
  values are filled in under OPS-026".
- IFC-001: a call "carries exactly the operation's inputs as its `arguments`"; REV-007: a control
  "has applied exactly the values it was sent". So a default the engine sends is a value the
  implementation must apply, which can change a setting the caller did not name.
- REV-009: the unknown settings are those "of the inputs the request supplied".
- OPS-009 compares contents "after the defaults are filled in under OPS-026". OPS-026 fills no
  omitted input of a control, so for a control whose `label` defaults to `"default label"`,
  `{limit: 20}` and `{limit: 20, label: "default label"}` are different contents, and a repeat of
  one under the other's key reports conflict (OPS-010).

**What the engine does today.** `access::evaluate` fills the arguments with
`env.fill(&scope, &schema, &request.arguments, contract["spec"].get("changes").is_none(), …)`.
For a control the flag is false, which takes `schemas.rs`'s whole-value fill (`filled`), the one
SCH-009 uses for a configuration: it inserts the default of every absent root property. The
filled arguments become the route's, the invocation's `arguments`, the `tools/call` the
implementation receives (`requests.rs`, `"arguments":v["arguments"]`) and the contents OPS-009
compares (DL2's digest). Every other caller of `Environment::fill` keeps its defaults, as its
promise asks: `registration.rs` for a configuration (SCH-009, INS-024), a template's input
(INS-006, INS-007) and a requirement's input (NAM-019), `schemas.rs`'s check of a default
(SCH-010, SCH-019), and `lib.rs`'s `EvaluateSchema`, whose `supplied` and `operation-inputs`
modes vectors use. An operation that is not a control keeps OPS-026's default for an omitted
input. The shell still answers a control as unfinished (`engine/CONFORMANCE.md`), so no serving
engine dispatches one and no kept store holds a control's arguments.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing repeats an effect implicitly: a control no longer resets a setting its caller did not
name; 2, every access can be inspected: the invocation's record shows what its caller sent, and
`status.settings` names those inputs alone. None at risk: no promise or existing case changes.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: JSON Schema 2020-12 makes `default` an annotation, which validation
does not apply (Validation, section 9.2), and a JSON Merge Patch leaves each member it does not
name as it is (RFC 7396). Not taken: dropping the defaults inside a supplied value too, which
OPS-026's last sentence, SCH-009 and REV-015 fill, a promise change only the owner can make;
keeping order 270's count alone, while the implementation is still sent the default; spec text,
since REV-006 and OPS-026 already say it.

**When it runs.** On `main` once order 270's build has merged: the start check stops otherwise.
Through Conduit's dogfood: one Codex worker turn (4-hour limit; aim for under an hour), then a
Codex verifier and a Claude reader at once, blind, who run only what settles a doubt; one fix
turn; the gate (DEC-046, DEC-033). By ruling 89, each decision says what must hold; how is yours.
The reading that a control's repeat compares only the inputs supplied, keeping order 270's
`control_inputs`, and the test and case names are the coordinator's, open to the owner's strike.

**Shared files.** Order 271 also adds an entry to `conformance/conventions/changes/index.yaml` and
changes rows of `engine/STANDING.md`; order 267 changes `requests.rs` at `record_content` and in
its tests, and rows of `engine/STANDING.md`. Whichever merges later rebases and runs
`promise_standing.py --write` again.

**Stop rule.** Stop, change nothing further, and report where:

- a correct result needs a change outside the scope: a promise, a shell or loader source,
  `engine/crates/conformance/`, a package, a `Cargo.toml` or a lockfile;
- the change would alter what another caller of `Environment::fill` fills, or what an operation
  that is not a control is sent;
- an existing case fails, or `vector_delta.py` shows a change other than the added case: report
  the case and step with the expected and the observed reply; change no other case;
- an existing test other than the one decision 3 names fails: report it and its assertion;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The dispatch** (REV-006, OPS-026). Where the requested operation is a control (its contract
   has `changes`), the arguments `access::evaluate` builds hold exactly the inputs the request
   supplied: no input the request left out is added, whatever its `default`. Each supplied input
   is still filled as any supplied value is: a map it holds takes the defaults of its absent
   properties under SCH-009, as today. The same arguments are validated, compared under OPS-009,
   stored on the invocation, recorded and sent in the `tools/call`. One way: add a method beside
   `fill`, for a control's inputs, that runs its `operation_inputs` branch without inserting an
   absent input's default, and call it where `changes` is present; `fill`'s signature and its
   callers stay as they are. Change no other output, record or decision.
2. **Order 270's count stays.** `control_inputs` and `control_input_supplied` are unchanged; they
   then name the inputs the invocation's `arguments` hold. In `finish`, the comment that retries
   "can supply different inputs with the same normalized arguments" no longer holds for a
   control: say that a retry of a control has its first attempt's inputs.
3. **Tests**, in `requests.rs`'s tests, on order 270's `control_request_fixture`, whose control
   `configure.v1` gives `label` the default `"default label"`. Write them first, run them on the
   unchanged code, and report each failing line.
   - `control_dispatch_sends_only_supplied_inputs`: the fixture's request, `revision` and `limit`
     20, is stored with `arguments` exactly `{"revision":"initial-revision","limit":20}`, and the
     `tools/call` the engine then sends carries exactly those `arguments`. With an input
     `options`, `{type: object, properties: {depth: {type: integer, default: 1}}}`, added to the
     contract and its `changes` in the test, a request that supplies `options: {}` is stored and
     sent with `options` `{"depth":1}` (OPS-026's last sentence). Today `label` is there.
   - Order 270's `successful_control_retry_leaves_the_previously_supplied_setting_unknown`
     becomes `control_repeat_writing_out_a_default_conflicts`: its first attempt, sent `label`
     `"default label"`, ends unknown; its repeat under the same key, which omits `label`, now
     reports conflict with a diagnostic naming OPS-010, makes no attempt and sends no call; and
     `status.settings` still holds `/limit` and `/label` for the first invocation. Today the
     repeat is attempt 2.
   - Change no other existing test.
4. **One case**, `change-omitted-input`, in `conformance/conventions/changes/omitted-input/` with
   its `index.yaml` entry: portable (from `{"recipe":"empty"}`), promises `REV-006` and
   `OPS-026`. Take its steps from order 270's `change-unknown-setting`, through the setup, with
   the control's `inputSchema` giving `label` the default `"default label"`. Then send the
   control `limit` 20 alone at the instance's revision R: the step's `external_intents` hold the
   one `tools/call` to the plain item, whose `arguments` hold `revision` R and `limit` 20, and an
   `absent` pointer `/external_intents/0/call/params/arguments/label`. No later step is needed.
   Write it first, run it on the unchanged engine, and report the failing line. Declare it in
   `docs/work-orders/272-declared.yaml`, in the form of [256's](256-declared.yaml).
5. **The documents.** `engine/CONFORMANCE.md`: after "(REV-009, `change-unknown-setting`).", add
   "From order 272, a control is sent only the inputs its caller supplied, each filled as any
   supplied value is (REV-006, OPS-026, `change-omitted-input`)." Keep `From order 272,` on one
   line. Run `python3 tools/promise_standing.py --write`.

## What to read

REV-006, REV-007, REV-009, REV-015, OPS-009, OPS-010, OPS-026, SCH-009, IFC-001 and CTR-012;
`Environment::fill` and `filled` in `schemas.rs`; `access::evaluate` at its `env.fill`; in
`requests.rs`, the `tools/call` built from `v["arguments"]`, the OPS-010 conflict beside
`old["digest"]`, `control_inputs`, `control_input_supplied`, `finish`, and order 270's tests on
`control_request_fixture`; the cases `change-unknown-setting` and `requests-write-success`'s
dispatch step, by step with `sed -n`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 5b83f31a HEAD; echo "main $?"
r=engine/crates/core/src/requests.rs
grep -c 'fn control_input_supplied(' "$r"
grep -c 'fn successful_control_retry_leaves_the_previously_supplied_setting_unknown(' "$r"
grep -cF '"label":{"type":"string","default":"default label"}' "$r"
grep -cF 'contract["spec"].get("changes").is_none(),' engine/crates/core/src/access.rs
grep -c 'From order 270,' engine/CONFORMANCE.md
grep -c 'From order 272,' engine/CONFORMANCE.md
grep -cE '^\| REV-006 \| convention \| must \| 0 \| 0 \| outside \| none \| no case, no reason \|$' engine/STANDING.md
git grep -lE 'fn (control_dispatch_sends_only_supplied_inputs|control_repeat_writing_out_a_default_conflicts)\(' -- engine | wc -l | tr -d ' '
test -e conformance/conventions/changes/omitted-input; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `1`, `1`, `1`, `1`, `0`, `1`, `0`, `case 1`, `engine 0` and `0`.

## Scope

Change `engine/crates/core/src/access.rs` at its fill, `schemas.rs` beside `Environment::fill`, and
`requests.rs` in decision 2's comment and its tests; `conformance/conventions/changes/index.yaml`;
`engine/CONFORMANCE.md` as decision 5 says; and `engine/STANDING.md` only by the tool. Create only
the case directory and `docs/work-orders/272-declared.yaml`. Change nothing else: not `spec/`,
another case, `docs/`, `packages/`, `tools/`, `experiments/`, a shell or loader source,
`engine/crates/conformance/`, `Cargo.toml` or `Cargo.lock`. Report anything else that needs
changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last cargo line, the whole engine suite,
once, last.

```sh
ok='^test result: ok\. 1 passed'
r=engine/crates/core/src/requests.rs
(cd engine && cargo test --offline --release -p conduit-core --lib control_dispatch_sends_only_supplied_inputs 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-core --lib control_repeat_writing_out_a_default_conflicts 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. change-omitted-input 2>&1 | tail -1)  # 1 cases passed; today no cases matched the requested prefix
grep -c 'fn successful_control_retry_leaves_the_previously_supplied_setting_unknown(' "$r"  # 0; today 1
grep -c 'can supply different inputs with the same normalized arguments' "$r"  # 0; today 1
grep -c 'From order 272,' engine/CONFORMANCE.md  # 1; today 0
grep -cE '^\| REV-006 \| convention \| must \| 1 \| 0 \| outside \| [a-z]+ \| case passes \|$' engine/STANDING.md  # 1; today 0
test -f docs/work-orders/272-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
ok='^test result: ok\. 1 passed'
n=conformance/conventions/changes/omitted-input
git add -N "$n" docs/work-orders/272-declared.yaml  # so the checks see the new files
(cd engine && cargo test --offline --release -p conduit-core --lib defaulted_control_arguments_do_not_mark_or_clear_omitted_settings 2>&1 | grep -c "$ok")  # 1
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. change-unknown-setting 2>&1 | tail -1)  # 1 cases passed
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/272-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(access|schemas|requests)\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/conventions/changes/(index\.yaml|omitted-input/[^/]+)|docs/work-orders/272-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- "$n" docs/work-orders/272-declared.yaml  # stage nothing
```

Report, per decision, what changed and where; each new test's and the case's failing line on the
unchanged code, then its passing line; the test you changed, and why; and every check's last
line.

Stage nothing. The verifier's turn follows; then the gate, a read-only Claude reader, and the
merge.

## What follows

Named here and not in this order:

- **Order 270's workaround then.** `control_inputs` names the same inputs as the invocation's
  `arguments`. It should stay: it reads the request, which is what REV-009 names, so an argument
  the engine might add later cannot widen what inspection reports unknown, and it costs one list
  per control invocation. Dropping it for `arguments` alone is a cleanup no promise asks for.
- **Found while writing:** when a control's success comes to store its values (order 270's "What
  follows"), it stores only the inputs it was sent (REV-006, REV-007), and its REV-015 check fills
  the supplied values alone; with this order the invocation's `arguments` already hold just those.

## Record

Not yet run.
