# Work order 275: a setup that succeeds gives each nested instance it covers, by ID, with its lifecycle state and revision, so apply keeps its ledger from the reply (order 247's finding)

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

**The finding.** Order 247 (`conduit apply`) found: "A parent setup can change an implicit
child's lifecycle, but the setup reply omits individual child outcomes." So apply's ledger keeps
each declared Conduit and each entry's Resource as its registration left it, and an agent caller
cannot tell from the result what became of each (ruling 52: every result as explicit as possible
for agent callers). K3 keeps, in each ledger row, the object's ID, revision and lifecycle state
([Kept state and privacy](../implementation/plan-and-apply/README.md#kept-state-and-privacy)).

**The coordinator's decision**, open to the owner's strike: `set-up.v1` declares one output,
`objects`. A setup that succeeds gives in it each nested instance its first part covers, as
`object.v1` names it, with its lifecycle state and its revision as the request leaves them.

**Read against `main` at `05d711dc`.** `set-up.v1` declares no outputs, so a setup that
succeeds has no `structuredContent` (IFC-025), and `settle` in `lifecycle.rs` ends every setup
with `json!({})` as outputs. The first part opens each declared Conduit (CON-004) or, for a
Pool, sets up each entry's Resource (POOL-013), whose own setup opens that Resource's declared
Conduits; the parts are kept in `status.setup` and the records. No setup advances a revision
(REV-012). `open.v1` and `close.v1` act on the Conduit named alone: the outcome says its state
after (ready; or closed and removed) and neither advances its revision, so neither has an
implicit child, and neither changes here. `finish` puts outputs in the result only where the
invocation's contract, copied at dispatch, declares an `outputSchema`, and keeps them for a
repeat; for an operation that declares outputs, a repeat whose outputs are not kept reports
conflict (OPS-034).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2,
every access can be inspected: the result names, by ID, each declared Conduit a setup opened;
5, every management action is available to an agent: the caller of a setup learns what became of
each nested instance from its result; 6, nothing repeats an effect implicitly: a keyed repeat
answers the kept items and dispatches nothing, and one whose items are not kept reports conflict
and dispatches nothing. Newly shown: to whoever sets up the instance holding them, even through a
Conduit that lists `set-up.v1` and not `inspect.v1`, each nested instance's kind, name, ID, state
and opaque revision (REV-001), which inspection shows only to whoever may inspect it (LIF-013).
The coordinator's reading.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: `docker compose up` reports each container it created, started or found
running, by name, with its state. Not taken: inspections added to apply's plan (K3 would print
each read as a plan step, changing its cases' plans and bytes); the parts themselves as the
output (they are the engine's steps, name no revision, and the records hold them already); only
the instances this setup changed (a further setup would give an empty list where all are ready,
hiding their state; the records say which this setup opened); items for a setup that did not
succeed (IFC-025 gives no outputs then; its diagnostics name the part that stopped it, OPS-022,
and apply stops there).

**When it runs.** On `main` from `05d711dc` (order 264, #547), beside the orders not yet
merged, 270 and 271 among them. Through Conduit's dogfood: one Codex worker turn (4-hour limit;
aim for under 2 hours), a blind Codex verifier and Claude reader, one fix turn, the gate (DEC-046,
DEC-033). By ruling 89, each decision says what must hold; how is yours. The words, the schema's
form, the case's name and steps and the test's name are the coordinator's, open to the owner's
strike.

**Shared files.** 270 and 271 change other operations' text in `spec/core/operations.yaml`, the
core README and one line each of `spec/CHANGES.md`, and 271 `registration.rs`; 247 the shell, K3
and its cases; most orders `engine/STANDING.md`. This order changes, in `operations.yaml`, only
`set-up.v1`'s description and its new output schema; in the core README one sentence of the
setup paragraph; in `CHANGES.md` one line; in `lifecycle.rs` only `settle`, what it calls, and
one test. Whichever merges later rebases and
runs `promise_standing.py --write` again.

**Stop rule.** Stop, change nothing further, and report where:

- a wording needs a choice that this order and `spec/` do not make: report it with the options;
- a correct result needs a change outside the scope: core `records.rs`, `requests.rs`,
  `registration.rs` or `lib.rs`, a shell or loader source, `engine/crates/conformance/`, a
  package, `docs/` beyond this order's files, a `Cargo.toml` or a lockfile;
- an item's state or revision differs from what `inspect.v1` shows for that object right after:
  report the object and both values; invent nothing;
- `check_schemas.py` refuses decision 1's `allOf`: report its line;
- an existing case or test fails, or `vector_delta.py` shows a change other than the added case:
  report the case or test and its step, with the expected and the observed value, and change no
  other case or test; above all a repeat of a succeeded setup that now reports conflict;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The spec text.** Change no promise and no shared schema.
   - `set-up.v1`'s description, after "and dispatches the hook again only as its idempotency key
     allows.", gains:

     ```text
     A setup that succeeds gives in `objects` each nested instance, not retired, of the nested instantiations its first part covers: the declared Conduits of the instance or, for a Pool, the Resource of each entry and that Resource's declared Conduits; the list is empty where there are none, as for a Fleet. Each item names the object as `object.v1` does, `nested` among it, with its `state`, ready or registered, and its `revision`, as the request leaves them, the ones `inspect.v1` then shows (CON-004, POOL-013, REV-012). A setup that does not succeed gives no items (IFC-025). A repeat that OPS-009 answers gives the items as the invocation it repeats left them, and one whose outputs the engine no longer keeps reports conflict (OPS-034).
     ```

   - Its contract gains, between `inputSchema` and `effect`, in the form of `tombstone.v1`'s
     `owner` in `spec/core/results.yaml`:

     ```yaml
       outputSchema:
         type: object
         properties:
           objects:
             type: array
             items:
               allOf:
               - $ref: github.com/shrav-k/conduit/core#object.v1
               - properties:
                   state:
                     enum:
                     - registered
                     - ready
                   revision:
                     $ref: github.com/shrav-k/conduit/core#revision.v1
                 required:
                 - state
                 - revision
         required:
         - objects
     ```

   - `spec/core/README.md`, after "then it dispatches the setup hook (TPL-017).", gains: "A setup
     that succeeds names in `objects` each nested instance of its first part, each entry
     Resource's declared Conduits among them, with its lifecycle state and revision."
   - `spec/CHANGES.md`, under `## [Unreleased]`:

     ```text
     - Added: `set-up.v1`'s output `objects`: for a setup that succeeds, each nested instance of its first part, a declared Conduit or an entry's Resource and that Resource's declared Conduits, with its lifecycle `state` and `revision`, so a client learns what became of each with no separate inspection, and a repeat of one whose outputs the engine no longer keeps reports conflict under OPS-034 (order 247's finding; ruling 52).
     ```

2. **The engine** (decision 1), in `lifecycle.rs` alone, storing nothing new: a setup that
   succeeds ends with `{"objects": [...]}` as its outputs, built from state as it ends: for the
   instance, each of its nested instances that is not retired, of slot `conduits`, or for a Pool
   of slot `entries`, and for a Pool each such entry Resource's own `conduits` too. Each item is
   `kind`, `name`, `id`, `nested` (`instance` the holder's ID, `name` the declaration's, as
   `registration.rs` writes it), `state` and `revision`, the stored values. A setup that does
   not succeed ends with outputs as now. A repeat takes the kept outputs as they are: change
   neither `finish` nor the repeat path. Outputs kept before this order are not rewritten: a
   repeat of such a setup answers `{}`, as kept (OPS-009).
3. **One case**, `setup-children-returned`, in `conformance/conventions/setup/children-returned/`
   with its entry in `conformance/conventions/setup/index.yaml`: portable (from
   `{"recipe":"empty"}`), promises `CON-004`, `LIF-003`, `REV-012` and `OPS-009`. Take the
   register step's form from portable cases that invoke `management.register.v1`, such as
   `sectioned-wire-framed-only`, and the keyed form, the input's `key`, from the portable
   `requests-usage-kept`.
   - First start; then `management.register.v1` through root, with a manifest of address
     `example.org/children` and: a Pool template `team-t` (type
     `github.com/shrav-k/conduit/core#pool.v1`), the Pool `team` from it, an Interface template
     `holder-t` (type `github.com/shrav-k/conduit/core#interface.v1`) that declares a Conduit
     `slot` from `github.com/shrav-k/conduit/core#root-pool` to the Pool `team`, as
     `declared-person` in `conformance/conventions/inventory/e4/documents.json` does, and the
     Interface `holder` from it. Capture `holder`'s ID and the name and ID of `slot`'s instance.
   - `management.set-up.v1` of `holder`, under a key, succeeds; its `objects` has one item: that
     Conduit by name and ID, `nested` naming `holder`'s ID and `slot`, `state` ready, and a
     `revision`, captured.
   - Inspecting that Conduit through root's `management` gives that ID and revision and the state
     ready.
   - The same request under the same key is a repeat (OPS-009): the same invocation and the same
     `objects`.
   - `management.set-up.v1` of `holder` with no key is a further setup; its `objects` gives the
     same item, with the same revision (CON-004, REV-012).
   - `management.set-up.v1` of `team`, which has no entries, succeeds with `objects` empty.

   The case states no revision's value: it compares replies with each other and with inspection
   through captures. A step no portable route reaches is left out and listed under `own_choices`.
   Declare it in `docs/work-orders/275-declared.yaml`, in the form of
   [256's](256-declared.yaml), and run `python3 tools/promise_standing.py --write`.
4. **One core test**, `pool_setup_names_each_entry_and_its_declared_conduits`, in `lifecycle.rs`'s
   tests, in the form of `held_back_pool_part_names_only_its_unmet_dependencies`: a Pool whose
   entry `base` has a ready Resource that holds a ready declared Conduit `link`, and whose entry
   `spare`, marked optional, has a registered Resource whose part does not succeed. The setup
   succeeds, and its kept outputs' `objects` has exactly three items, `base`'s Resource, `link`'s
   Conduit and `spare`'s Resource, each with its `nested` and with the state and revision its
   stored object has: ready, ready and registered.
5. **No other case changes.** Read on `main`: the runner's `projection`
   (`engine/crates/conformance/src/expect.rs`) compares an expected map as a subset of the actual
   one; no set-up step in any case expects `/reply/structuredContent` absent or an empty map
   there, and no case repeats a succeeded setup under its key. 241's harness takes a result's
   members by name and compares only those the case states, so K3's cases and
   `plan-apply-mismatches.txt` stand. The runner checks each succeeded built-in reply and kept
   output against its `outputSchema` (`builtin_output`), so with decision 1 alone a case whose
   setup succeeds fails there. Write decisions 1 and 3 first, run the new case and
   `setup-pool-entry` on the unchanged engine, and report each failing line; then decision 2
   makes both pass.

## What to read

`set-up.v1`, `open.v1` and `close.v1`; `object.v1` and `tombstone.v1`'s `owner`; TPL-012,
CON-004, POOL-013, LIF-003, REV-012, OPS-009, OPS-034 and IFC-025; in `lifecycle.rs`, `advance`'s
declarations part, `settle`, `child_answered` and `setup_fixture`; `finish` and `ended_reply` in
`requests.rs`, to read only; where `prepare` in `registration.rs` writes an item's `nested`; the
case `setup-pool-entry`, by step with `sed -n`; K3's rows 8 and 9.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 05d711dc HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/core/operations.yaml 'and dispatches the hook again only as its idempotency key allows.'
j spec/core/README.md 'then it dispatches the setup hook (TPL-017).'
sed -n '/^  name: set-up.v1$/,/^---$/p' spec/core/operations.yaml | grep -c '^  outputSchema:'
grep -cF 'requests::finish(t, a, c, i, outcome, json!({}), json!([]), cause)' engine/crates/core/src/lifecycle.rs
test -e conformance/conventions/setup/children-returned; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `1`, `0`, `1`, `case 1`, `engine 0` and `0`.

## Scope

Change `spec/core/operations.yaml`, `spec/core/README.md` and `spec/CHANGES.md` as decision 1
says; `engine/crates/core/src/lifecycle.rs` as decisions 2 and 4 say;
`conformance/conventions/setup/index.yaml`; and `engine/STANDING.md` only by the tool. Create
only the case directory and `docs/work-orders/275-declared.yaml`. Change nothing else: not a
promise, another case, `docs/` (K3, its cases and order 247 among it), `packages/`, `tools/`, a
shell or loader source, core `records.rs`, `requests.rs`, `registration.rs` or `lib.rs`,
`engine/crates/conformance/`, `Cargo.toml` or `Cargo.lock`. Report anything else that needs
changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last cargo line, the whole engine suite,
once, last.

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. setup-children-returned 2>&1 | tail -1)  # 1 cases passed; today no cases matched the requested prefix
(cd engine && cargo test --offline --release -q -p conduit-core pool_setup_names_each_entry_and_its_declared_conduits 2>&1 | grep -c '^test result: ok. 1 passed')  # 1; today 0
j spec/core/operations.yaml 'A setup that succeeds gives in `objects` each nested instance, not retired,'  # 1; today 0
sed -n '/^  name: set-up.v1$/,/^---$/p' spec/core/operations.yaml | grep -c 'core#revision.v1$'  # 1; today 0
j spec/core/README.md 'A setup that succeeds names in `objects` each nested instance of its first part,'  # 1; today 0
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- Added: `set-up.v1`.s output `objects`: .*(order 247.s finding; ruling 52)\.$'  # 1; today 0
test -f docs/work-orders/275-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n=conformance/conventions/setup/children-returned
git add -N "$n" docs/work-orders/275-declared.yaml  # so the checks see the new files
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. setup-pool-entry 2>&1 | tail -1)  # 1 cases passed
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. setup-review-dependencies-succeeded 2>&1 | tail -1)  # 1 cases passed
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/275-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(core/(operations\.yaml|README\.md)|CHANGES\.md)|engine/crates/core/src/lifecycle\.rs|engine/STANDING\.md|conformance/conventions/setup/(index\.yaml|children-returned/.+)|docs/work-orders/275-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it, 241's replay among it
git reset -q -- "$n" docs/work-orders/275-declared.yaml  # stage nothing
```

Report, per decision, what changed and where; decision 5's failing lines on the unchanged engine,
then their passing lines; and every check's last line.

Stage nothing.

## What follows

Named here and not in this order:

- **Order 247's ledger then takes children from the reply.** Its decision 8 reads a succeeded
  set-up's result as it reads a register's: each object the result's `objects` names, by ID,
  takes in `.conduit/state.yaml` the state and the revision its item gives, so after rows 8, 9,
  11, 12 or 20 set an instance up, the ledger holds each declared Conduit the setup opened, and
  each entry's Resource it set up, as ready, from the result alone; an item whose ID the ledger
  lacks is an error, as a register's unplanned object is. No inspection joins the plan. K3's
  cases hold no nested instance, so each of their setups gives `objects` empty: their `result`
  lines gain `"objects":[]`, which 247's test 1 allows, since it compares a result line by the
  members the case states. Where `objects` is missing, as in a repeat of a setup from before
  this order, it reads no items. 247's text is not changed here; its rerun cites this order.
- **Found while writing:** a declared Conduit a setup opens has opening provenance (ACC-044)
  naming the part's own invocation, which this result does not give; where 247's ledger keeps
  it for such a Conduit, giving it is a later order's. `retire.v1` closes at once the Conduits
  whose target is the instance, and the cascades retire members, and no result names them; a
  later order's, in this order's form.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main`.

**Worker-2's turn** (08:01 to 08:11) gave `set-up.v1` an output `objects`: for a setup that
succeeds, each nested instance it covers (declared Conduits; for a Pool, its entry Resources and
their declared Conduits; retired ones left out), by ID, with its lifecycle state and revision,
the values inspect.v1 then shows. With the new schema and the unchanged engine,
`setup-children-returned` failed at step 3 and `setup-pool-entry` at step 5; both pass, with 182
core tests and the whole vectors test.

**Two blind reads at once** (08:11 to 08:15): the Claude reader passed it with no findings;
`turn-verifier-2.turn.v1` found a must and a should. The must: a Pool registered before this
order holds the earlier core release's contract, so its fresh setups succeed without `objects`.
The coordinator kept the pin model (DEC-048: an object keeps the release it was registered with)
and had the limit stated rather than the contract a fresh invocation uses changed. One fix turn
(08:16 to 08:28) states it in `set-up.v1`'s description, the core README and the changelog line,
adds a core regression for a target holding the earlier contract, and makes the case's keyed
repeat assert no dispatch. 183 core tests, the case and the vectors test pass.

The gate's run on the squash is the coordinator's, at the merge.
