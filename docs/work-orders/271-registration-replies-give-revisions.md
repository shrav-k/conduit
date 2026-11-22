# Work order 271: `register.v1` and `register-replacement.v1` give each object's revision beside its ID, so apply keeps its ledger from the reply (order 247's finding)

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

**The finding.** Order 247 (`conduit apply`) stopped, in its worker's words: "register.v1 returns
each object's kind, name and ID. Revisions are independently generated opaque values. The
kept-state reader requires a revision ... Case 01's fixed plan has no inspection for six
registered objects that its kept state lists with revisions, including Pool/team--t and both
generated root Conduits. Case 02 has the same gap for Resource/history--t. The replay harness
obtains these revisions through separate inspections when it compares kept state. K3 requires
post-print reads to appear as plan steps, and decision 8 requires the exact ledger from the
registration result onward." ([K3](../implementation/plan-and-apply/README.md#kept-state-and-privacy)
keeps format `1`, each ledger row with its ID and revision.)

**The coordinator's decision**, open to the owner's strike: the engine, which assigns each
revision, returns it. Each object item of a registration's reply gives its revision beside its
kind, name and ID, exactly the revision `inspect.v1` then shows for it. Apply keeps the exact
ledger from the reply alone. K3's cases keep their plans, recorded text and digests; a register's
printed `result` line gains each item's `revision`, which 247's test 1 already allows, since it
compares a result line by the members the case states.

**Read against `main` at `1fe9aeb0`.** `replace.v1` already returns `revision`, the template's
revision after the request, the one it had where nothing changed (SUB-009; `records.rs` sets it
from the stored template): nothing changes there. `register-replacement.v1` gives one `object`,
not a list. Both outputs use `object.v1` (kind, name, ID and, for a nested instance, `nested`),
which records, refusals and other results share. The engine takes each revision from its
evidence as it builds the object (`Builder::allocate`; the default Conduits in `prepare`) and
stores it, and `inspect.v1` shows the stored value; the reply leaves it out. A repeat under
OPS-009 answers the kept outputs, which `registration_boundary` and the `register.v1` branch of
`records.rs` take from `t.reply["objects"]`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 5,
every management action is available to an agent: the result now gives the revision that the
caller's next revision-checked change names; 6, nothing repeats an effect implicitly: a repeat
still answers its kept result and dispatches nothing. Newly shown: an opaque revision (REV-001)
to whoever made the object, even through a Conduit that lists `register.v1` and not
`inspect.v1`, as the ID is today. The coordinator's reading.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a Kubernetes create or server-side apply answers with the object as
stored, its `metadata.resourceVersion` among it, so `kubectl apply` needs no second read. Not
taken: inspections added to apply's plan (K3 places each read after printing as a plan step, so
cases 01 and 02 would change their plans and bytes); a ledger without revisions (K3 rows 10 and
16 read a template's revision); `revision` in `object.v1` itself (a record or a refusal that names
an object needs none).

**When it runs.** On `main` from `1fe9aeb0` (order 265, #537), with 266 since merged, beside 267.
Through Conduit's dogfood: one Codex worker turn (4-hour limit; aim for under 2 hours), a blind
Codex verifier and Claude reader, one fix turn, the gate (DEC-046, DEC-033). By ruling 89, each
decision says what must hold; how is yours. The words, the schema's form, the case's name and
steps are the coordinator's, open to the owner's strike.

**Shared files.** 267 changes, as 265 did, `inspect.v1` in `spec/core/operations.yaml`, its
sentence in `spec/core/README.md`, one line of `spec/CHANGES.md`, `registration.rs` beside
`Held::package_list`, `records.rs`'s `inspect.v1` branch, and
`conformance/language/registration/index.yaml`; 266 added one call in `prepare` beside LIF-041's
check, a few lines above this order's edit, and changed `reject`, `records.rs`'s `preflight` and
other core files; 245 and 247 the shell, K3 and its cases; all of them `engine/STANDING.md`.
This order changes, in `operations.yaml`, only `register.v1`'s and `register-replacement.v1`'s
descriptions and output schemas; in the core README one sentence; in `CHANGES.md` one line; in
`registration.rs` only where `prepare` builds `registered` and pushes the default Conduits, and
where `replacement` builds its result; `records.rs`, the shell, K3 and its cases not at all. Its
case goes under `conformance/conventions/changes/`. Whichever merges later rebases and runs
`promise_standing.py --write` again.

**Stop rule.** Stop, change nothing further, and report where:

- a wording needs a choice that this order and `spec/` do not make: report it with the options;
- a correct result needs a change outside the scope: core `records.rs`, `requests.rs`,
  `lifecycle.rs` or `lib.rs`, a shell or loader source, `engine/crates/conformance/`, a package,
  `docs/` beyond this order's files, a `Cargo.toml` or a lockfile;
- the revision of an object that the request made differs from what `inspect.v1` shows right
  after it: report the object and both values; something in the request advanced it; invent
  nothing;
- `check_schemas.py` refuses decision 1's `allOf`: report its line;
- an existing case fails, or `vector_delta.py` shows a change other than the added case: report
  the case and step with the expected and the observed reply; change no other case;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The spec text.** Change no promise, `object.v1` or `replace.v1`.
   - `register.v1`'s description, after "and says of each nested instance which nested
     instantiation made it.", gains:

     ```text
     Each item also gives `revision`, the revision of that object as the request leaves it, the one `inspect.v1` then shows for it (REV-001), for a template, an instance, a nested instance and a default Conduit of FLEET-005 alike. A repeat that OPS-009 answers changes nothing and gives the items of the invocation it repeats, with the revisions that invocation left; where a later change has advanced one (REV-003), `inspect.v1` shows the newer.
     ```

   - `register-replacement.v1`'s description, after "The request is a registration, so one that
     is rejected still carries `diagnostics`.", gains:

     ```text
     `object` gives, beside the new instance's kind, name and ID, its `revision`, the one `inspect.v1` then shows for it (REV-001), as an item of the `objects` of `register.v1` does.
     ```

   - Their output schemas: `register.v1`'s `objects.items` and `register-replacement.v1`'s
     `object`, each now `$ref: github.com/shrav-k/conduit/core#object.v1`, become, indented as
     the `$ref` they replace (the form of `tombstone.v1`'s `owner` in `spec/core/results.yaml`):

     ```yaml
     allOf:
     - $ref: github.com/shrav-k/conduit/core#object.v1
     - properties:
         revision:
           $ref: github.com/shrav-k/conduit/core#revision.v1
       required:
       - revision
     ```

   - `spec/core/README.md`: "which is the one it had where the request changed nothing
     (SUB-009)." becomes "which is the one it had where the request changed nothing (SUB-009);
     `register.v1` and `register-replacement.v1` give each object they made its revision beside
     its ID (REV-001)."
   - `spec/CHANGES.md`, under `## [Unreleased]`:

     ```text
     - Added: `register.v1`'s `objects` and `register-replacement.v1`'s `object`: each item gives `revision`, the object's revision as the request leaves it, which `inspect.v1` then shows, so a client keeps it with no separate inspection (order 247's finding; ruling 52).
     ```

2. **The engine** (decision 1), in `registration.rs` alone, storing nothing new:
   - in `prepare`, each item of `registered` gains `revision`, the stored object's revision as
     the transition commits it: the authored templates and instances, nested ones among them,
     and the default Conduits FLEET-005 makes;
   - in `replacement`, the result's `object` gains the new instance's `revision`;
   - the kept outputs take `t.reply["objects"]` as they are, so a repeat answers the revisions
     too; change neither place. Outputs kept before this order are not rewritten: a repeat of
     such an invocation answers them as kept (OPS-009).

   Change no other output, record or decision.
3. **One case**, `change-revisions-returned`, in `conformance/conventions/changes/revisions-returned/`
   with its entry in `conformance/conventions/changes/index.yaml`: portable (from
   `{"recipe":"empty"}`), promises `REV-001`, `FLEET-005`, `LIF-009` and `OPS-009`. Take the steps'
   form from portable cases that invoke `management.register.v1`, such as
   `sectioned-wire-framed-only`, and the keyed form, the input's `key`, from
   `records-replay-register`, a keyed register and its repeat (internal: take only the form), or
   the portable `requests-usage-kept`.
   - First start; then `management.register.v1` through root, under an idempotency key, with a
     manifest of address `example.org/revisions` and: a Pool template `team-t` (type
     `github.com/shrav-k/conduit/core#pool.v1`), the Pool `team` from it, a Fleet `crew` from
     `github.com/shrav-k/conduit/core#fleet`, an Interface template `holder-t` (type
     `github.com/shrav-k/conduit/core#interface.v1`) that declares a Conduit `slot` from
     `github.com/shrav-k/conduit/core#root-pool` to the Pool `team`, as `declared-person` in
     `conformance/conventions/inventory/e4/documents.json` does, and the Interface `holder` from
     it. Its `objects` has eight items: the two templates, `team`, `crew`, `holder`, the
     instance of `slot`, and root's default Conduits to `team` and to `crew`. Each has a
     `revision`; capture each, with the generated names.
   - Inspecting each of the eight through root's `management` gives the revision its item gave.
   - The same request again under the same key is a repeat (OPS-009): its `objects` give the
     same eight items with the same revisions.
   - Then, in the steps of `e4-tombstone-register-replacement`: open the instance of `slot`,
     close it, lift its tombstone (its ID from `management.list.v1`'s reply or the close's
     `records`, not from state), and `management.register-replacement.v1` with
     `{"instance":{"kind":"Interface","name":"holder"},"nested":"slot"}` succeeds; its `object`
     has a `revision`, and inspecting that Conduit, by the reply's name, gives it.

   The case states no revision's value and no order in which the engine takes them from the
   evidence: it compares each reply with inspection through captures. If no portable route
   reaches a succeeded `register-replacement.v1`, leave those steps out, and `LIF-009` out of the
   case's promises and its index entry; list it under `own_choices`, and say how far it went.
   Declare the case in `docs/work-orders/271-declared.yaml`, in the form of
   [256's](256-declared.yaml), and run `python3 tools/promise_standing.py --write`.
4. **No other case changes.** Read on `main`: the runner's `projection`
   (`engine/crates/conformance/src/expect.rs`) compares an expected map as a subset of the actual
   one, arrays by length; no case expects an item there to be an empty map, and each pointer
   into one is a `capture` or an `absent` of another member (`registration-pool-entries`'
   `nested`). 241's harness compares `objects` the same way (`compare_objects`), so K3's cases
   and `plan-apply-mismatches.txt` stand. The runner also checks each succeeded built-in reply and
   each kept output against its `outputSchema` (`builtin_output`), so with decision 1 alone, a
   case that registers or asks for a replacement (`register-replacement.v1`) fails there. Write
   decisions 1 and 3 first, run the new case and `e4-tombstone-register-replacement` on the
   unchanged engine, and report each failing line; then decision 2 makes both pass.

## What to read

`register.v1`, `register-replacement.v1` and `replace.v1`, with their output schemas;
`object.v1` and `tombstone.v1`'s `owner` in `spec/core/results.yaml`; REV-001, REV-003, REV-012,
OBJ-003, FLEET-005, LIF-009 and OPS-009; in `registration.rs`, `Builder::allocate`, the end of
`prepare` from "Nothing has been published before this point" and `replacement`; in `records.rs`,
`registration_boundary` and the `register.v1` branch; the case `e4-tombstone-register-replacement`,
by step with `sed -n`; K3's "Kept state and privacy".

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 1fe9aeb0 HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/core/operations.yaml 'and says of each nested instance which nested instantiation made it.'
j spec/core/operations.yaml 'The request is a registration, so one that is rejected still carries `diagnostics`.'
j spec/core/operations.yaml 'among the outputs is the template'
j spec/core/README.md 'which is the one it had where the request changed nothing (SUB-009).'
sed -n '/^  name: register.v1$/,/^---$/p' spec/core/operations.yaml | grep -c 'core#revision.v1$'
grep -cF 'json!({"kind":o["kind"],"name":o["name"],"id":o["id"]})' engine/crates/core/src/registration.rs
grep -cF 'registered.push(json!({"kind":"Conduit","name":name,"id":id}));' engine/crates/core/src/registration.rs
grep -cF '"object":{"kind":object["kind"],"name":object["name"],"id":object["id"]}' engine/crates/core/src/registration.rs
test -e conformance/conventions/changes/revisions-returned; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `1`, `1`, `1`, `0`, `1`, `1`, `1`, `case 1`, `engine 0` and `0`.

## Scope

Change `spec/core/operations.yaml`, `spec/core/README.md` and `spec/CHANGES.md` as decision 1
says; `engine/crates/core/src/registration.rs` as decision 2 says;
`conformance/conventions/changes/index.yaml`; and `engine/STANDING.md` only by the tool. Create
only the case directory and `docs/work-orders/271-declared.yaml`. Change nothing else: not a
promise, another case, `docs/` (K3, its cases and order 247 among it), `packages/`, `tools/`, a
shell or loader source, core `records.rs`, `requests.rs`, `lifecycle.rs` or `lib.rs`,
`engine/crates/conformance/`, `Cargo.toml` or `Cargo.lock`. Report anything else that needs
changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last cargo line, the whole engine suite,
once, last.

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. change-revisions-returned 2>&1 | tail -1)  # 1 cases passed; today no cases matched the requested prefix
j spec/core/operations.yaml 'Each item also gives `revision`, the revision of that object as the request leaves it'  # 1; today 0
j spec/core/operations.yaml 'its `revision`, the one `inspect.v1` then shows for it (REV-001), as an item of the `objects` of `register.v1` does.'  # 1; today 0
sed -n '/^  name: register.v1$/,/^---$/p' spec/core/operations.yaml | grep -c 'core#revision.v1$'  # 1; today 0
sed -n '/^  name: register-replacement.v1$/,/^---$/p' spec/core/operations.yaml | grep -c 'core#revision.v1$'  # 1; today 0
j spec/core/README.md 'give each object they made its revision beside its ID (REV-001).'  # 1; today 0
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- Added: `register.v1`.s `objects` and .*(order 247.s finding; ruling 52)\.$'  # 1; today 0
test -f docs/work-orders/271-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n=conformance/conventions/changes/revisions-returned
git add -N "$n" docs/work-orders/271-declared.yaml  # so the checks see the new files
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. e4-tombstone-register-replacement 2>&1 | tail -1)  # 1 cases passed
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/271-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(core/(operations\.yaml|README\.md)|CHANGES\.md)|engine/crates/core/src/registration\.rs|engine/STANDING\.md|conformance/conventions/changes/(index\.yaml|revisions-returned/.+)|docs/work-orders/271-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it, 241's replay among it
git reset -q -- "$n" docs/work-orders/271-declared.yaml  # stage nothing
```

Report, per decision, what changed and where; decision 4's failing lines on the unchanged engine,
then their passing lines; whether the case reached the replacement; and every check's last line.

Stage nothing.

## What follows

Named here and not in this order:

- **Order 247's decision 8 then reads** as written, from the reply alone: each object a register
  made is in `.conduit/state.yaml` from that register's result on, with the ID and the revision
  its item gives and the flag its registration carried; a `replace.v1` step's row takes the
  reply's `revision`, and a `register-replacement.v1` step's (248's) its `object`'s. No inspection
  joins the plan: cases 01 and 02 keep their plans, recorded text and digests, and test 1's
  `kept` holds for Pool/team--t, both root Conduits and Resource/history--t. The journal's
  projection (decision 6) carries the revisions too. 247's text is not changed here; its rerun cites this order. Whether
  241's harness keeps its separate inspections is 247's choice.
- **Found while writing:** `register-replacement.v1`'s `object` gives no `nested`, though
  `object.v1` says an item for an instance a nested instantiation made names it, as
  `register.v1`'s items do. Adding it is a later order's.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-2's turn** (05:53 to 06:07) added `revision` to each object item of `register.v1`'s
reply and to `register-replacement.v1`'s object, the value the transition stores and inspect.v1
then shows, for authored templates, instances, nested instances and the generated default
Conduits; a keyed repeat returns the same. With the new schemas and the unchanged engine, the new
case `change-revisions-returned` and `e4-tombstone-register-replacement` failed as expected; both
pass now, with 180 core tests and the whole vectors test.

**Two blind reads at once** (06:08 to 06:11): both passed it. The verifier's one should, accepted:
the case checked the two default Conduits only as Conduits; a short turn (06:11 to 06:13) asserts
each one's default provenance and target, so each is told apart. All 17 steps pass.

The gate's run on the squash is the coordinator's, at the merge.
