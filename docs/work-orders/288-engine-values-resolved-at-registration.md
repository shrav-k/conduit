# Work order 288: the engine resolves `values:` from root's value Resources at registration and keeps each instance's source map (DEC-076, the engine's first part)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; list under
`vectors` the five cases you add, with the promises each tests. It needs no network: run cargo
offline; if a crate is missing, stop and report, and fetch nothing. Run one cargo command at a
time. Never print a whole large file, case, diff or log. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential, even one a test made. Do not run `conduit` outside a test, `codex`, `claude`,
the dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user or
machine name, account, or private repository name; every value in a case is fake.

## What and why

**The owner's answer** ([DEC-076](../decisions/log.md#dec-076-2026-11-22-installation-values-are-valuev1-resources)):
the durable form of private values -> "value.v1 Resources (Recommended)". Order 283 wrote it into
the spec. This order builds the engine's part except LIF-009's replacement, which a second engine
order builds (What follows): the whole does not fit one turn.

**Today** (read on `main` at `52a29f1a` with 283's text):

- The engine reads the core package from `spec/core` as it runs, so the template `value` and
  `value.v1` load with 283, and the loader admits `values` by `instance.v1`; nothing else reads
  it. `registration.rs`'s `Builder::expand` takes inputs from `arguments` alone (INS-003 to
  INS-005 in its `apply`), so an input given under `values` takes its `default`, or else reports
  INS-004 when required or is left out. A value Resource's `node` resolves as any Resource
  reference; nothing checks its Type or the Resource's owner.
- ACC-057 refuses an Agent at three places, `access::evaluate_path`, `registration::register` and
  `registration::prepare`, each by `kind == "Agent"`: a sender other than root that holds ACC-033's
  Conduits registers a value Resource.
- `REFUSAL_CAUSES` (`explain.rs`) and ACC-056's table are compared verbatim by
  `refusal_causes_match_spec_table_verbatim` (the conformance crate's `checks.rs`).
- `prepare` copies each nested instance's `_argument_values` into the holder's nested
  instantiation for a later replacement; `records.rs`'s `inspect.v1` branch builds `declared`
  from the stored object less its internal fields.
- No path stores a control's values: a succeeded control leaves `config` and `revision` as they
  were, and `engine/CONFORMANCE.md` calls control dispatch unfinished.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing repeats an effect implicitly: a value is written once, at registration, and retiring a
value Resource or registering another under its name changes no instance (OBJ-010). At risk: 3,
a shared thing is owned once and never copied to an agent: each instance holds a copy as
configuration, as 283 decided, and its source map names the one owner; 5, every management
action is available to an agent: root alone registers a value Resource (ACC-057), so no one
plants a value under a name in use; anyone may name one.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: Kubernetes server-side apply keeps `managedFields` beside an object, a
record of which manager set each field, and Helm keeps the values a release was installed with
apart from its rendered manifests (`helm get values`). Not taken: resolving at each run, as
`envFrom` does, which changes an instance without a request (test 6, OBJ-010); falling back to an
input's `default` where a value is missing, which OBJ-009 forbids.

**When it runs.** Order 283 must have merged: the start check stops otherwise. On `main` from
`52a29f1a` (#563). Through Conduit: one Codex worker turn (a 4-hour limit; aim for under 2 hours),
a blind Codex verifier and Claude reader (DEC-071), one fix turn, the gate (DEC-046, DEC-033). By
ruling 89, each decision says what must hold; how is yours. The case IDs and steps, the `ruleId`
of each check, the reading for controls and the split are the coordinator's, open to the owner's
strike.

**Shared files.** Core `registration.rs`, `access.rs`, `explain.rs` and `records.rs`,
`spec/CHANGES.md`, `engine/CONFORMANCE.md` and `engine/STANDING.md`: whichever order merges later
rebases and reruns `promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only where two promises contradict (give
both), a step needs a change outside the scope (give its failing line), or a file outside Scope
must change. Where this order's wording contradicts a promise it does not change, the promise
governs: build to the promise and list it under `own_choices`, with every other choice no decision
governs.

## The decisions

1. **A value Resource is root's alone (ACC-057).** A registration or `register-replacement.v1`
   that would make a Resource of the core Type `value.v1`, from the core template `value`, from a
   template of that Type registered or submitted, or as a Pool entry's Resource, is dispatched
   only where root sends it on its own behalf in SEC-008's form. Otherwise it is refused under
   ACC-057 at each of the three places, in ACC-056's step (3), before ACC-033 and every validity
   check; it registers nothing, and its diagnostic locates that document as an Agent's is. A
   template of that Type makes no Resource and stays ACC-033's alone.
   - ACC-056's row and `REFUSAL_CAUSES` change together, to: "| ACC-057 | Only root on its own
     behalf registers a submission containing an Agent or a value Resource. |"
   - `spec/CHANGES.md`, under `## [Unreleased]`, after its last item:

     ```text
     - Changed: ACC-056, the cause for ACC-057 names a value Resource beside an Agent (DEC-076).
     ```

2. **A value Resource's own checks** (`value.v1`'s description), each invalid under OBJ-023: a
   `node` that does not resolve to a Resource of the core Type `node.v1`, the diagnostic naming the
   document, `/config/node`, the authored place that supplies it (`/arguments/node` from the
   template `value`) and the reference as written; an owner other than `system`, naming `/owner`
   and the owner as supplied, or the entry and its Pool where nesting gives the owner.
3. **`values` beside `arguments`** (INS-003, INS-004, OBJ-013, OBJ-014), in an instance document
   of any family but Fleet and in a declared Conduit or a Pool's entry. A name under `values` that
   is no declared input, or one input under both sections, is invalid under INS-003, the
   diagnostic naming the input and its section or both, with the JSON Pointer of each offending
   entry. An input given under either section is supplied as an argument is (INS-004). A nested
   instantiation's entries are checked when its holder is instantiated, never when its template
   is registered.
4. **Resolution** (OBJ-009, OBJ-021, INS-005), at OBJ-009's moment. An entry
   `{kind: Resource, name}` resolves only where (1) it names a Resource of the core Type
   `value.v1` that `system` owns and that is not retired; (2) that Resource states no `node`, or
   its resolved node is the resolved placement of every instance whose input the value supplies,
   so an Interface, a Pool or a Conduit takes none tied to a node, and a holder is not supplied by
   a value that only its nested definition holds; (3) no reference schema governs the input or any
   place the value reaches, through every destination, forwarding into nested and applied
   requirement arguments included, checked before a scalar could resolve as a reference; and (4)
   the value satisfies the input's schema. It then fills each destination as INS-006 assigns an
   argument.
   - A failing check rejects the document, which reports invalid (OBJ-023), with `ruleId` INS-005
     for (4) and OBJ-009 otherwise: no other source, no `default`, no optional input taken as left
     out. The diagnostic names the original document, the entry's place (`/values/<input>`, or its
     pointer in the document holding the nested instantiation), the value Resource as written and
     the failing check. Its locations give the entry's place and, where the failure is at a
     receiving input or destination, that place's pointer in the receiving definition. Where no
     Resource has the name, its message gives the value Resource's registration by root as the
     remedy.
   - No diagnostic or record about an entry holds the value, in `message.text` or `properties`;
     the value shows only where configuration does.
5. **The source map** (INS-013). Each instance a value filled keeps, outside `declared`, a map
   from each place filled, a JSON Pointer into its `declared`, to the value Resource's resolved
   form `{kind: Resource, id}`. A held nested instantiation holds the scalar under `arguments` and
   no `values`; the holder's map gives that argument's source at its pointer (such as
   `/entries/worker/arguments/account`), whether the entry was written there or the holder's own
   input forwarded it, and the nested instance maps each place in its own definition. `inspect.v1`
   gives `values`, the map, exactly while it has entries, as its `outputSchema` states; a value
   Resource's `declared` shows `config.value` and `config.node` as configuration.
6. **Controls and replacement** (REV-007, LIF-009): two declared limits, no contradiction to
   build through. (a) As read on `main`, no control stores a value, so none removes a source map
   entry; a setting a control leaves unknown at a mapped place keeps its entry, beside
   `status.settings`. (b) A LIF-009 replacement is made from the held scalar as today, carrying
   no source, and no held argument keeps its node constraint, document, entry pointer or
   reference as written: INS-013's and INS-017's parts that the second part builds.
   `engine/CONFORMANCE.md` gains one paragraph beginning "From order 288," that says what this
   order built and both limits.
7. **Five portable cases**, from `{"recipe":"empty"}`, written first and run on the unchanged
   engine, each first failing step reported. Where a value must not show, use `absent`
   `{text, under: ["/reply", "/records"]}`, as `sections-secret-framed` does. Take a project's
   Types and templates from `registration-login-place`, a second Interface acting through
   Conduits root opened from `setup-action-by-scope`, and a Resource's retirement from
   `fleets-retire-login-resource`. Each rejection states its `ruleId` and locations.
   - `registration-values-resolved` (OBJ-009, OBJ-013, OBJ-014, INS-004, INS-013): root registers
     value Resources `acct`, a string, and `home`, a path with `node` `local`, from `core#value`. A
     Resource template `worker-t` takes `account` and `home` into its config; a Pool template
     `team-t` forwards its input `account` to its entry `worker`, from `#worker-t`, whose own
     `values` give `home`; the Pool `team` gives `account` under `values`. Inspecting `team` gives
     both scalars under `declared.entries.worker.arguments`, no `values` there, and `values`
     mapping both argument pointers to `acct` and `home`; inspecting the entry's Resource gives both
     under `config` and `values` mapping `/config/account` and `/config/home`; inspecting `acct`
     gives `config.value` and no `values`.
   - `registration-values-unresolved` (OBJ-009, INS-003): invalid for an entry naming no Resource,
     naming `management`, naming a retired value Resource, and naming no Resource for an optional
     input with a `default`; for a name under `values` that is no input; and for one input under
     both sections.
   - `registration-values-misplaced` (OBJ-009, OBJ-021, INS-005): invalid for a value tied to
     `local` given to an Interface's input; for an input whose schema is `resource-reference.v1`;
     for a string input whose destination the Type's schema governs by a reference schema; and for
     a value failing its input's schema, whose synthetic value is absent.
   - `registration-value-resource-checks` (OBJ-023, OBJ-009): invalid for a value Resource whose
     `node` names `management` and for one owned by another Pool; one from a submitted template of
     Type `value.v1` registers.
   - `access-value-resource-root-only` (ACC-057, ACC-056): an Interface other than root holding
     ACC-033 row 1's Conduits, to `management` and `system` each listing `register.v1`, registers a
     plain Resource owned by `system`, and is refused under ACC-057 for a value Resource from
     `core#value` and for one from a submitted template of that Type, neither registering; root
     then registers the first.

   If no portable route retires a value Resource, leave that step out and list it under
   `own_choices`. Index the cases in `conformance/language/registration/index.yaml` and
   `conformance/conventions/access/index.yaml`, declare them in
   `docs/work-orders/288-declared.yaml` in the form of [271's](271-declared.yaml), and run
   `python3 tools/promise_standing.py --write`.

## What to read

OBJ-009, OBJ-010, OBJ-013, OBJ-014, OBJ-021, OBJ-023, INS-003 to INS-006, INS-012, INS-013,
ACC-033, ACC-056, ACC-057, PLC-002, PLC-020 and REV-007 as 283 left them; `value.v1`, `value`,
`value-entries.v1` and `inspect.v1`; in `registration.rs`, `Builder::expand`, `resolve_instance`
and `prepare`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 52a29f1a HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
sed -n '/^## DEC-076/,/^## DEC-077/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
grep -c '^  name: value.v1$' spec/core/types.yaml  # from order 283
grep -c '^  name: value$' spec/core/templates.yaml
grep -c '^  name: value-entries.v1$' spec/schemas/instances.yaml
sed -n '/^  name: inspect.v1$/,/^---$/p' spec/core/operations.yaml | grep -c '^      values:$'
j spec/promises/objects.yaml 'A `values` entry must name a Resource of the core Type `value.v1`'
j spec/promises/instantiation.yaml 'Its source map keys each place a value filled'
j spec/conventions/access.yaml 'or a Resource of the core Type `value.v1`.'
grep -cF '| ACC-057 | Only root on its own behalf registers a submission containing an Agent. |' spec/conventions/access.yaml
grep -cF '"Only root on its own behalf registers a submission containing an Agent.",' engine/crates/core/src/explain.rs
git grep -l -e 'value-entries' -e 'value\.v1' -- engine/crates | wc -l | tr -d ' '
test -e conformance/language/registration/values-resolved; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, ten lines `1`, `0`, `case 1`, `engine 0` and `0`.

## Scope

Change `engine/crates/core/src/` for decisions 1 to 6 and their tests; ACC-056's one row in
`spec/conventions/access.yaml` and `spec/CHANGES.md` (decision 1); `engine/CONFORMANCE.md`
(decision 6); the two `index.yaml` files; `engine/STANDING.md` by the tool. Create only the five
case directories and `docs/work-orders/288-declared.yaml`. Change nothing else: not another
promise, a `spec/core` or `spec/schemas` document, another case, the shell, the loader,
`engine/crates/conformance/`, `packages/`, `tools/`, `docs/decisions/` or a Cargo file. Report
anything else.

## Done when

The first group prints otherwise on `main` today and passes after; the second, guards, passes
today and must still pass. Run the core's tests and then the whole vectors test once, last; never
the whole engine suite.

```sh
for p in registration-value access-value-resource-root-only; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # 4 and 1 cases passed; today FAIL no cases matched the requested prefix
grep -cF '| ACC-057 | Only root on its own behalf registers a submission containing an Agent or a value Resource. |' spec/conventions/access.yaml  # 1; today 0
grep -cF '"Only root on its own behalf registers a submission containing an Agent or a value Resource.",' engine/crates/core/src/explain.rs  # 1; today 0
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- Changed: ACC-056, .*(DEC-076)\.$'  # 1; today 0
grep -c 'From order 288,' engine/CONFORMANCE.md  # 1; today 0
test -f docs/work-orders/288-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
new=$(git status --short -uall | sed -n 's/^?? //p'); git add -N $new  # so the checks see them
for p in register-agent-non-root registration-arguments- registration-nested-reference- registration-pool-entries registration-entry- e4-tombstone-register-replacement; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # each passes, as today
(cd engine && cargo test --offline --release -p conduit-conformance --lib refusal_causes_match_spec_table_verbatim 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_model_account.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/288-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(conventions/access\.yaml|CHANGES\.md)|engine/crates/core/src/[a-z_]+\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/(language/registration/(index\.yaml|values?-[a-z-]+/.+)|conventions/access/(index\.yaml|value-resource-root-only/.+))|docs/work-orders/288-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $new  # stage nothing
```

Report, per decision, what changed and where; each case's failing step on the unchanged engine,
then its passing line; each diagnostic's message for a rejected entry; and every check's last
line. Stage nothing.

## What follows

- **The engine's second part** (S, stacked on this, beside the CLI's): a LIF-009 replacement takes
  the held scalar and carries its source to each place it fills, the holder keeping for each held
  argument the original node constraint, document, entry pointer and reference as written; it
  checks that node against the replacement's placement and the current template's inputs and
  destinations (INS-003, INS-005, INS-014, OBJ-009), resolving no name; INS-017 with held sources;
  a case for each.
- **The CLI's** (M, stacked, next): `conduit value set`, `list` and `unset`; the plan's value lines
  and its stop naming `conduit value set <name>`, with no value (K3); records redacted by value
  name on export; whether an instance holding the same value with no source map is unchanged
  (stage 3's logins).
- **Found while writing:** no control stores its values (REV-007 stands `no case, no reason`);
  control storage, when built, removes a mapped place's entry (INS-013).
- Then the dogfood's and the short form's, as order 283 lists them.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, stacked
on spec order 283 (merged during the turn as #565).

**Worker-2's turn** (14:29 to 15:00) built DEC-076's engine part: root alone registers a
`value.v1` Resource (ACC-057), with its node and owner checked; `values:` entries resolve at
registration with OBJ-009's checks and no fallback; the instance keeps a source map, shown by
`inspect.v1`. The five cases failed on the unchanged engine and pass, with the six regression
groups, 190 core tests and the whole vectors test.

**Two reads.** The Claude reader (15:01 to 15:06) had one must, two shoulds and two nits. The
first Codex verifier read was stopped when the coordinator rebuilt its working copy for another
order during it, and the order was read again blind by `turn-verifier-1.turn.v1` (15:07 to
15:16): four musts, on a noncanonical array index passing the reference-schema check, a
requirement member's failure naming the wrong value Resource, a template's own node reported
at the wrong place, and a forwarded value's duplicate pointing at an argument never authored.
One fix turn (15:21 to 15:37) applied all nine, seven regressions failing first: the five cases,
the regressions, 197 core tests and the vectors test pass. The node and owner checks keep
OBJ-009 (the worker's choice, open to the owner's strike). The two declared limits stand for the
second part. The gate's run on the squash is the coordinator's, at the merge.
