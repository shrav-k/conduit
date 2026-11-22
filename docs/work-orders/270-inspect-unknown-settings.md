# Work order 270: inspection shows each setting a control left unknown, after reconciliation too, so that plan's row 5 sees it (order 245's finding)

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

**The finding.** Order 245's verifier: "The planner expects status.settings. The current
inspect.v1 implementation reports unreconciled invocation IDs but emits no setting-level unknown
state. REV-009 and REV-010 require the setting to remain unknown after reconciliation." K3's row 5
([the plan-and-apply reference](../implementation/plan-and-apply/README.md)) stops a step that
would reach "an instance with a setting reported unknown", printing
`stop KIND/NAME setting=POINTER state=unknown REV-009`, and plan reads only what inspection shows
(step 2).

**What `spec/` and the engine hold today.** REV-009: the settings an unknown control addressed "are
reported as unknown", those "of the inputs the request supplied, at the pointers the control's
`changes` gives for those inputs", and "stay reported unknown after reconciliation, under REV-010",
which "records the outcome alone". Under OBJ-002 no document changes an existing instance, and after
it is made only controls change its configuration (definitions, configuration); an instance that
replaces another is another instance. Where the word unknown stands is open: `spec/core/README.md`,
"What this package does not settle", lists "where the word unknown stands against a setting
(REV-009)", to be settled with the vectors. In the engine, `records.rs`'s `inspect.v1` branch builds
`status.unreconciled` from the invocations whose state is `unknown`; the `reconcile.v1` branch of
`requests.rs` (where its rules gain REV-010) sets a control's state to `succeeded`; no code marks a
setting, so after reconciliation nothing shows it. `engine/CONFORMANCE.md`: "Inspection does not yet
report individual settings as unknown (REV-009)." The shell still answers a control as unfinished
(same file), so no kept store holds an unknown control; the core ends one as any invocation
(`unsafe_parameter_header_errors_name_ifc046` ends one unknown).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2,
every access can be inspected: inspection shows which settings a control may have changed; 5,
every management action is available to an agent: an agent learns from one inspection which
settings it cannot trust, without the records a purge removes (REC-004); 6, nothing repeats an
effect implicitly: plan stops a step over an unknown setting rather than replace or resend over
it. Newly shown, to whoever may inspect the instance (ACC-033): the control's name and the
invocation's ID, which the records already hold, and no value. None at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: Kubernetes server-side apply's `metadata.managedFields` names, for the
fields of an object, the manager and operation that last set them, holding the field paths and
not their values. Not taken: plan reading the control's records, which a purge removes; keeping
the ID in `unreconciled` after reconciliation, against OPS-014 and LIF-013; showing the value
sent, which REV-010 does not store.

**When it runs.** On `main` from #536. Through Conduit's dogfood: one Codex worker turn (4-hour
limit), then a Codex verifier and a Claude reader at once, blind, who run only what settles a
doubt; one fix turn; the gate (DEC-046, DEC-033). By ruling 89, each decision says what must
hold; how is yours. The member's name and form, the reading of when a setting leaves it (a later
success, an ancestor pointer included; a failure keeps it; a running new attempt counts by its
earlier one), and the test and case names are the coordinator's, open to the owner's strike.

**Shared files.** Orders 245, 265 and 267, not merged, change `inspect.v1`'s description and schema,
`spec/core/README.md`, `records.rs`'s `inspect.v1` branch and `requests.rs`, at other lines. Change
`records.rs` only in that branch, beside `status["unreconciled"]`, and `requests.rs` only in the
`reconcile.v1` branch and where an attempt is dispatched, and either's tests. Whichever merges
second rebases and runs `promise_standing.py --write` again.

**Stop rule.** Stop, change nothing further, and report where:

- a wording needs a choice that this order and `spec/` do not make: report it with the options;
- a correct result needs a change outside the scope: a promise, a shell or loader source,
  `engine/crates/conformance/`, a package, a `Cargo.toml` or a lockfile;
- a control cannot be sent from a portable case's inputs, or ends otherwise than decision 4
  says: report the step and its reply; keep decision 3's tests;
- an existing case fails, or `vector_delta.py` shows a change other than the added case: report
  the case and step with the expected and the observed reply; change no other case;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The spec text.** Change no promise.
   - `inspect.v1`'s description in `spec/core/operations.yaml`, after "its machine binding and
     enrollment (PLC-008, PLC-026, ENR-010).", gains:

     ```text
     The status of an instance holds, in `settings`, each setting that a control whose outcome is unknown addressed (REV-009), under its JSON Pointer: `state` `unknown`, the control's qualified reference as `control` and the engine's ID for the invocation as `invocation`, and no value. A setting stays there after reconciliation (REV-010), and leaves once a later control that was sent a value for it, or for a setting that holds it, succeeds other than by reconciliation (REV-007); a later control whose outcome is unknown and that addresses it takes its place, and one that fails changes nothing. No document changes an existing instance (OBJ-002), so an instance that replaces another holds none of its settings. `settings` is absent where no setting is unknown, and no purge shortens it.
     ```

   - Its `outputSchema`, in `status`'s `properties`, after `unreconciled`:

     ```yaml
               settings:
                 type: object
                 propertyNames:
                   $ref: github.com/shrav-k/conduit/core#pointer.v1
                 additionalProperties:
                   type: object
                   properties:
                     state:
                       const: unknown
                     control:
                       $ref: github.com/shrav-k/conduit/core#qualified-reference.v1
                     invocation:
                       $ref: github.com/shrav-k/conduit/core#id.v1
                   required:
                   - state
                   - control
                   - invocation
                   additionalProperties: false
     ```

   - `spec/core/README.md`: after "that was dispatched to the instance or names it (LIF-013).",
     add "`settings` holds each setting a control left unknown, by its JSON Pointer, with the
     control and the invocation, until a later control settles it (REV-009, REV-010)."; in "What
     this package does not settle", delete "where the word unknown stands against a setting
     (REV-009); ".
   - `spec/CHANGES.md`, under `## [Unreleased]`:

     ```text
     - Added: `inspect.v1`'s status member `settings`: under the JSON Pointer of each setting that a control whose outcome is unknown addressed (REV-009), `state` `unknown`, the control and the invocation, and no value, kept after reconciliation (REV-010) until a later control settles it (order 245's finding; ruling 52).
     ```

2. **The engine** (decision 1). An instance's inspection reads its control invocations (those
   whose contract has `changes`, dispatched to it) in the order they were dispatched:
   - one whose outcome is unknown, or was unknown and was reconciled, sets for each input of its
     `changes` present in its arguments the entry at that input's pointer to
     `{"state":"unknown","control":OPERATION,"invocation":ID}`; an input it was not sent gives
     none;
   - one that succeeded otherwise removes each entry at, or inside, the pointer of an input it
     was sent (a JSON Pointer inside another extends it by whole tokens; `""` holds every
     pointer);
   - one that failed, or whose first attempt has not ended, changes nothing;
   - a new attempt (OPS-012) is stored over the earlier one: until it ends, the invocation counts
     by the earlier attempt's outcome, and it is read where its latest attempt was dispatched.

   The entry holds no value, sent or stored. `settings` is present only where an entry remains, so a
   template, and an instance with none, inspect as today. One way: mark the invocation in the
   `reconcile.v1` branch, so a reconciled control is told from one that succeeded, number each
   dispatch as `set-up.v1`'s `dispatched` does, and build the map beside `status["unreconciled"]`.
   The `inspect-object` decision may cite REV-009 too. Change no other output, record or decision.
3. **Two unit tests**, in `requests.rs`'s tests (from `call_fixture` with a control's spec and
   `inspect_setup`) or `records.rs`'s, as you find simplest. Write them first, run them on the
   unchanged code, and report each failing line.
   - `unknown_settings_inspected_after_reconciliation`: a control whose `changes` is
     `{"limit":"/limit","label":"/label"}`, sent `limit` alone, ends unknown: `status.settings`
     is exactly `{"/limit":{"state":"unknown","control":…,"invocation":…}}`, the configuration
     and revision are unchanged and `unreconciled` names it; after `reconcile.v1` with
     `occurred: true`, `settings` is the same and `unreconciled` is empty. Today: no `settings`.
   - `later_control_settles_unknown_setting`: after that, a later control sent `limit` that
     fails keeps the entry; one that ends unknown takes its place, naming its invocation; one
     that succeeds removes it, and `settings` is absent. A control whose `changes` is
     `{"all":""}` ending unknown gives the key `""`, which a later success at `/limit` keeps, and
     an unknown entry at `/limit` is removed by a later success at `""`. Today: no `settings`.
4. **One case**, `change-unknown-setting`, in `conformance/conventions/changes/unknown-setting/`
   with its `index.yaml` entry: portable (from `{"recipe":"empty"}`), promises `REV-009`,
   `REV-010`, `OPS-014` and `LIF-013`. Take the steps' form from `requests-usage-kept` and the
   control from `requests-sign-in-hold`'s `set-limit.v1`, in a Type with no sign-in. Write it
   first, run it on the unchanged engine, and report the failing line.
   - First start; register a project whose Type's schema has `limit` and `label`, its control
     `configure.v1` with `changes` `{"limit":"/limit","label":"/label"}`, implemented by a plain
     item; an instance of it; root's Conduit to it listing the control; open and set it up.
   - Send the control `limit` 20 alone at the instance's revision R; the provider's answer is
     `missing`, and it ends unknown with invocation I.
   - Inspect: `status.settings` is `{"/limit":{"state":"unknown","control":Q,"invocation":I}}`,
     Q the control's qualified reference, with no `/label`; `unreconciled` is `[I]`; the revision
     R and the configuration as registered.
   - Reconcile I with `occurred: true` through the tool ACC-033 names; inspect: the same
     `settings`, `unreconciled` `[]`.
   - Send the control `limit` 30 at R; it succeeds; inspect: no `settings` (an `absent` pointer).

   Declare it in `docs/work-orders/270-declared.yaml`, in the form of
   [256's](256-declared.yaml).
5. **The documents.** `engine/CONFORMANCE.md`: "Inspection does not yet report individual
   settings as unknown (REV-009)." becomes "From order 270, inspection's `status.settings` shows
   each setting a control left unknown, after reconciliation too (REV-009,
   `change-unknown-setting`)." Keep `From order 270,` on one line. Run
   `python3 tools/promise_standing.py --write`.

## What to read

`inspect.v1`, `pointer.v1`; REV-007, REV-009, REV-010, OBJ-002, OPS-012, OPS-014, LIF-013 and
CTR-006; in `records.rs`, the `inspect.v1` branch; in `requests.rs`, the `reconcile.v1` branch,
the redispatch beside `previous`, `call_fixture`, `inspect_setup` and
`unsafe_parameter_header_errors_name_ifc046`; the cases
`requests-usage-kept` and `fleets-reconcile-control`, by step with `sed -n`; K3's step 2 and row 5.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 37c10930 HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/core/operations.yaml 'its machine binding and enrollment (PLC-008, PLC-026, ENR-010).'
j spec/core/operations.yaml 'The status of an instance holds, in `settings`'
grep -c '^          unreconciled:$' spec/core/operations.yaml
grep -c '^          settings:$' spec/core/operations.yaml
j spec/core/README.md 'where the word unknown stands against a setting (REV-009);'
j spec/core/README.md 'that was dispatched to the instance or names it (LIF-013).'
grep -c 'Inspection does not yet report individual settings' engine/CONFORMANCE.md
grep -cF 'status["unreconciled"] = json!(' engine/crates/core/src/records.rs
grep -cF 'rules.push("REV-010");' engine/crates/core/src/requests.rs
git grep -lE 'fn (unknown_settings_inspected_after_reconciliation|later_control_settles_unknown_setting)\(' -- engine | wc -l | tr -d ' '
test -e conformance/conventions/changes/unknown-setting; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `0`, `1`, `0`, `1`, `1`, `1`, `1`, `1`, `0`, `case 1`, `engine 0` and
`0`.

## Scope

Change `spec/core/operations.yaml`, `spec/core/README.md` and `spec/CHANGES.md` as decision 1
says; `engine/crates/core/src/records.rs` and `requests.rs` as the shared-files note bounds;
`conformance/conventions/changes/index.yaml`; `engine/CONFORMANCE.md` as decision 5 says; and
`engine/STANDING.md` only by the tool. Create only the case directory and
`docs/work-orders/270-declared.yaml`. Change nothing else: not a promise, another case, `docs/`
(K3 among it), `packages/`, `tools/`, `experiments/`, a shell or loader source,
`engine/crates/conformance/`, `Cargo.toml` or `Cargo.lock`. Report anything else that needs
changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last cargo line, the whole engine suite,
once, last.

```sh
ok='^test result: ok\. 1 passed'
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
(cd engine && cargo test --offline --release -p conduit-core --lib unknown_settings_inspected_after_reconciliation 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-core --lib later_control_settles_unknown_setting 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. change-unknown-setting 2>&1 | tail -1)  # 1 cases passed; today no cases matched the requested prefix
j spec/core/operations.yaml 'The status of an instance holds, in `settings`, each setting that a control whose outcome is unknown addressed (REV-009)'  # 1; today 0
grep -c '^          settings:$' spec/core/operations.yaml  # 1; today 0
j spec/core/README.md '`settings` holds each setting a control left unknown'  # 1; today 0
j spec/core/README.md 'where the word unknown stands against a setting (REV-009);'  # 0; today 1
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- Added: `inspect.v1`.s status member `settings`.*(order 245.s finding; ruling 52)\.$'  # 1; today 0
grep -c 'From order 270,' engine/CONFORMANCE.md  # 1; today 0
grep -c 'Inspection does not yet report individual settings' engine/CONFORMANCE.md  # 0; today 1
test -f docs/work-orders/270-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n=conformance/conventions/changes/unknown-setting
git add -N "$n" docs/work-orders/270-declared.yaml  # so the checks see the new files
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_model_account.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/270-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(core/(operations\.yaml|README\.md)|CHANGES\.md)|engine/crates/core/src/(records|requests)\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/conventions/changes/(index\.yaml|unknown-setting/[^/]+)|docs/work-orders/270-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- "$n" docs/work-orders/270-declared.yaml  # stage nothing
```

Report, per decision, what changed and where; each new test's and the case's failing line on the
unchanged code, then its passing line; and every check's last line.

Stage nothing. The verifier's turn follows; then the gate, a read-only Claude reader, and the
merge.

## What follows

Named here and not in this order:

- **Plan's row 5 then reads it.** In K3's step 2, order 245's planner reads `status.settings`:
  each member whose `state` is `unknown` gives `stop KIND/NAME setting=POINTER state=unknown
  REV-009` where a step of the plan would reach the instance, and otherwise
  `difference KIND/NAME setting=POINTER state=unknown REV-009`, beside `unreconciled`'s
  `OPS-013` lines. Once the invocation is reconciled, `unreconciled` no longer names it, and the
  setting's line alone holds the step until a control the author sends settles it; plan and
  apply send none (row 5). Order 245 reports that half of row 5 not yet built until this merges.
- **Found while writing:** a control that succeeds still stores no value, and nothing checks the
  values before its dispatch (REV-007 and REV-015 have no case), while the shell answers a
  control as unfinished. So on a serving engine no setting becomes unknown yet; building control
  dispatch, a settled setting's stored value with it, is a later order's.

## Record

Not yet run.
