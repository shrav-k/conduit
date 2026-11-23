# Work order 286: `pool:` makes the Pool that owns each Resource of its file that states no owner, as `fleet:` does for Agents (DEC-081)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` the eight cases you add, with the promises each tests.
It needs no network: every crate it builds is in the local cache, so run cargo offline and fetch
nothing, one cargo command at a time. Never print a whole large file, case, diff or log: use `head`,
`tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print
an environment variable's value, and never read or print a key, a token or any credential. Do not
run `conduit` outside a test, `npm`, `node`, `codex`, `claude`, the dogfood harness or
`tools/gate.py`, and sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every
file you write is public: no absolute local path, user or machine name, or private repository name.

## What and why

**The owner's answer** ([DEC-081](../decisions/log.md#dec-081-2026-11-22-a-default-pool-by-pool-as-a-fleet-by-fleet)):
the default Pool -> "pool: like fleet: (Recommended)", answering ruling 53 point 6: a Resource that
states no owner belongs to its file's `pool:` Pool, as an Agent belongs to its `fleet:` Fleet
(SCT-011), and `conduit init` writes neither.

**Today.** SCT-001 has no `pool` key and `sectioned-resource.v1` requires `owner`. The core Pool
template `system` is protected (FST-006), so T1
([K3's case 01](../implementation/plan-and-apply/cases/01-t1.yaml)) declares its Pool under `pools`
and gives `owner: team`.
The loader's `translate` (`engine/crates/loader/src/sectioned.rs`) handles `fleet` per file in
`check_file_fleets`, `framed_owner`, `check_owner` and `base_instance`. `conduit init` writes
only `apiVersion` and `packages: {}` (DEC-040): nothing changes there.

**S1's counts.** DEC-081, the owner's (ruling 71 point 4), lands before the S1 rerun's pin
(DEC-038) or not before the rerun. This order rewrites no K3 case or S1 example, whose form stays
legal, and adds no ID to `conformance/s1-promises.yaml`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, a
shared thing is owned once: a Resource keeps one owner, a Pool (POOL-001), now named once per
file; 2, every access is a grant that can be inspected and revoked: the key makes no Conduit, and
root's default Conduit to the Pool is FLEET-005's. At risk: ownership is authority, so the key
reaches only its own file's Resources that state no owner (decision 4).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a Kubernetes object that states no `metadata.namespace` is made in the
context's namespace, and one that states its own keeps it. Not taken (the coordinator's design
notes): an engine-made Pool per project (an implicit object, against SCT-004); rejecting a
stated owner that names another Pool, as SCT-011 does for a Fleet (DEC-081 says "states no
owner", and a file may hold two Pools' Resources); today's form alone.

**When it runs.** On `main` from `c4fa2197` (#559), before the S1 rerun's pin. Through Conduit: one
Codex worker turn (aim for under 2 hours), a blind Codex verifier and Claude reader, one fix turn,
the gate (DEC-046, DEC-033). By ruling 89, each decision says what must hold; how is yours. The
words, SCT-017's number, the template's and cases' names and the reading that a stated owner is
kept are the coordinator's, open to the owner's strike.

**Shared files.** Orders 283 and 284, also from `c4fa2197`, move the counts this order moves
(`spec/core/`, `spec/README.md`), and 284, 285 and 287 rewrite `engine/STANDING.md`; whichever
merges later rebases and recounts, by the tool where one exists.

**Stop rule.** A worker stops, changes nothing further and reports only for: a contradiction between
promises (give both); a step that needs an engine change outside the scope (give the file); or a
file outside Scope. Where this order's wording contradicts a promise it leaves unchanged, the
promise governs: follow it and list it under `own_choices` (the coordinator's D1). A choice no
decision governs is yours: list it under `own_choices`.

## The decisions

Wrap each insertion at its file's width.

1. **SCT-017**, after SCT-016 in `spec/promises/sections.yaml`, layer `language`, level `must`:
   - `when`: "A documents file states `pool: X` in a sectioned document."
   - `then`: "At most one `pool` key occurs in the whole file, across all its documents. X is a
     bare name. The translation makes Pool X from the core template `pool`, and supplies X as the
     owner of every Resource instance defined anywhere in that file, whether sectioned or framed,
     that states no owner. A Resource that states an owner keeps it, whichever Pool it names."
   - `observable`: "A second `pool` key in the file is rejected, the diagnostic naming both.
     Resources in another file receive no owner from this key and must state `owner` unless their
     own file supplies one. A Pool's entries are no Resource instance of the file and keep their
     Pool (POOL-002). Where several files state the same X, the project's translation makes that
     Pool once. A separately authored document named X still clashes under NAM-027. No root
     Conduit is translated: FLEET-005 makes the default. A package accepts no `pool` (MAN-011)."
2. **The short form's other promises**, each "A" becoming "B": SCT-001 `then`, "`apiVersion`,
   `fleet`, `templates`" -> "`apiVersion`, `fleet`, `pool`, `templates`"; SCT-003 `then`,
   "SCT-011 and SCT-016" -> "SCT-011, SCT-016 and SCT-017"; SCT-004 `then`, "the per-file rule
   SCT-011;" -> "the per-file rules SCT-011 and SCT-017;"; SCT-006 `observable`, "a Resource
   requires its Pool `owner`;" -> "a Resource requires its Pool `owner` unless SCT-017 supplies
   it;"; SCT-013 `then`, "and `fleet` values" -> "and `fleet` and `pool` values"; SCT-014
   `observable`, "A supplied Fleet owner maps to `/fleet`;" -> "A supplied Fleet owner maps to
   `/fleet`, and a supplied Pool owner to `/pool`;" and "A Fleet shared by several files" -> "A
   Fleet or Pool shared by several files".
3. **Elsewhere in `spec/`.**
   - MAN-011, "`fleet` and every instance section are rejected" -> "`fleet`, `pool` and every
     instance section are rejected"; SUB-010, "SCT-011 supplies owners per file" -> "SCT-011 and
     SCT-017 supply owners per file"; SCH-018, "available for SCT-011." -> "available for SCT-011
     and SCT-017."
   - `spec/schemas/sectioned.yaml`: `sectioned.v1` gains `pool`, with exactly `fleet`'s schema,
     and `required: [pool]` in its `anyOf`, in the file's block form; `sectioned-resource.v1`
     drops `owner` from both `required` lists, its description becoming "An inline or referenced
     Resource definition. Owner is required unless the file supplies pool." In the head comment
     and `sectioned.v1`'s description, per-file Pool ownership joins the Fleet's, SCT-017 joins
     the promises encoded, and "SCT-001 to SCT-016" becomes "SCT-001 to SCT-017".
   - `spec/core/templates.yaml`, after `fleet`, a Pool template `pool` of `metadata` and `spec`
     alone, `spec` being `type: github.com/shrav-k/conduit/core#pool.v1`, described as "The Pool
     template that a documents file's `pool` key instantiates (SCT-017). It has no inputs,
     entries or declared Conduits, and any project may instantiate it.", with a line citing
     SCT-017 in the head comment's list.
     `- pool` after `- fleet` in `spec/core/conduit.yaml`'s exports. `spec/core/README.md`:
     "| `pool` | Pool template | SCT-017 |" after the `fleet` row; "The 17 templates" -> "The 18
     templates"; "exports 145 documents" -> "exports 146 documents".
   - `spec/definitions.md`: the `section` row, "`fleet` is a per-file key, rather than a map
     section." -> "`fleet` and `pool` are per-file keys, rather than map sections."; the
     `translation` row, "only for `fleet`" -> "only for `fleet` and `pool`"; the `reference` row,
     "fixes Fleet (SCT-007, SCT-011)" -> "fixes Fleet and a `pool` value Pool (SCT-007, SCT-011,
     SCT-017)"; after the `fleet` row: "| `pool` | A per-file short-form key making the named Pool
     from the core template `pool` and supplying it as the owner of every Resource instance
     defined in that file that states none (SCT-017). A Resource that states an owner keeps it. |"
   - `spec/README.md`: after "`conduit init` writes a flow-style skeleton.", "It writes neither
     `fleet` nor `pool`."; "518 in all: 225 of the layer `language`" -> "519 in all: 226 of the
     layer `language`"; the row of `promises/sections.yaml`, "per-file Fleet ownership" ->
     "per-file Fleet and Pool ownership" and "14 promises" -> "15 promises"; "145 documents
     exported" -> "146 documents exported"; "17 templates" -> "18 templates".
   - `spec/CHANGES.md`, under `## [Unreleased]`, after its last item:

     ```text
     - Added: SCT-017, a documents file's `pool: X` makes Pool X from the new core template `pool` and supplies X as the owner of each Resource instance of that file that states none, sectioned or framed; a stated owner is kept (DEC-081).
     - Changed: SCT-001, SCT-003, SCT-004, SCT-006, SCT-013, SCT-014, MAN-011, SUB-010 and SCH-018 name `pool` or SCT-017 beside `fleet` or SCT-011, `sectioned.v1` accepts `pool`, and `sectioned-resource.v1` no longer requires `owner` (DEC-081).
     ```

4. **The catalogue.** `docs/implementation/SAFETY_CATALOGUE.md` gains SCT-017's row after
   SCT-016's, class `access`, quoting `then`'s "supplies X as the owner of every Resource
   instance defined anywhere in that file" (ownership is authority), the line "Work order 286
   added the row of SCT-017, which it wrote." and the counts it moves, as order 254 did for
   REC-007 (`git show f19c95e1`).
5. **The loader** (`sectioned.rs`, `locations.rs`): `pool` per file as `fleet` is, for
   Resources. A second `pool` in a file is an SCT-017 fault at its `/pool`, related to the first,
   worded as `check_file_fleets` words the Fleet's. The Pool is made once across files, `from:
   <core>#pool`, rule SCT-017 at `/pool`, each further file's place added to its sources. A
   sectioned Resource with no `owner` gets X, its translation's `owner` the `/pool` place; a
   framed Resource instance (with `from`) and no `owner` gets X under rule SCT-017; a stated owner
   is kept and compared with nothing. An archive's sectioned document holding `pool` is a MAN-011
   fault, as with `fleet`. `translated_places` treats a framed document's SCT-017 translation as
   SCT-011's. `fleet`'s translation and diagnostics stay byte for byte. `conduit-core`'s
   `sectioned_wire_items` (`access.rs`) lists `pool` too, so a wire item `{apiVersion, pool:
   team}` gets SCT-001's SUB-005 rejection, as a unit test shows. Add `"SCT-017"` to the `rule`
   enum of `conformance/case-schema/expect/translations.json`.
6. **Eight cases**, each in `conformance/language/sectioned/pool-key-<name>/` with its entry in
   that directory's `index.yaml`, ID `sectioned-pool-key-<name>`, in the form of the case named:
   - `supplied` (SCT-017, SCT-006; `fleet-framed`): `pool: team`, a Resource `shared` with no
     owner and `kept` with `owner: other`; the result has Pool `team` from `core#pool`, `shared`
     owned by `team`, `kept` by `other`, with their translations;
   - `framed` (SCT-017; `fleet-framed`): a framed Resource instance with no owner gets `team`
     under rule SCT-017, and one with an owner keeps it;
   - `duplicate-key` (SCT-017, SCT-014; `fleet-duplicate-key`);
   - `file-boundary` (SCT-017, SUB-005; `fleet-file-boundary`);
   - `shared` (SCT-017, SCT-004; `fleet-shared`): two files make Pool `team` once;
   - `located` (SCT-014, SCT-017, OBJ-002; `located-supplied-fleet`): `pool: team` beside a
     registered Pool `team` conflicts under OBJ-002 at `/pool`, its translation's rule SCT-017;
   - `archive` (MAN-011, SCT-017; `archive-install`): an archive's `pool` is rejected at install;
   - `registered` (SCT-017, POOL-001, FLEET-005; `inline-equivalence`): with `pool: library` in
     place of its `pools` member and its Resource's `owner`, the project registers, and the reply
     or an inspection shows Pool `library` from `core#pool`, the Resource owned by it, and root's
     default Conduit to it.

   Write the cases first and run them on the unchanged loader. One existing case changes:
   `sectioned-resource-owner` is still rejected under SUB-005 at the same file and document, now
   by the translated instance's schema, so only its `message_contains` changes, to that schema.
   Declare the eight additions and that change in `docs/work-orders/286-declared.yaml`, in the
   form of [256's](256-declared.yaml) and of `tools/vector_delta.py`'s head, and run
   `python3 tools/promise_standing.py --write`.

## What to read

SCT-001 to SCT-016, POOL-001, POOL-002, OBJ-013, FLEET-005 and MAN-011; the functions named
under Today and `translated_places` in `locations.rs`; decision 6's cases, by step with `sed -n`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor c4fa2197 HEAD; echo "main $?"
sed -n '/^## DEC-081/,$p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
k spec/promises/sections.yaml '`apiVersion`, `fleet`, `templates`' 'the per-file rule SCT-011;' 'a Resource requires its Pool `owner`;' 'A supplied Fleet owner maps to `/fleet`;'
k spec/README.md '518 in all: 225 of the layer `language`' '`conduit init` writes a flow-style skeleton.' '145 documents exported' '17 templates'
grep -c -e '^- id: SCT-017$' -e '^  name: pool$' spec/promises/sections.yaml spec/core/templates.yaml | tr '\n' ' '; echo
grep -cF 'packages: {{}}\n' engine/crates/shell/src/main.rs
find conformance/language/sectioned -name 'pool-key-*' | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `1 1 1 1`, `1 1 1 1`,
`spec/promises/sections.yaml:0 spec/core/templates.yaml:0`, `1`, `0`, `engine 0` and `0`.

## Scope

Change the `spec/` files the decisions name, `docs/implementation/SAFETY_CATALOGUE.md`,
`engine/crates/loader/src/sectioned.rs` and `locations.rs` (a test under
`engine/crates/loader/tests/` only where it asserts what decision 5 changes),
`engine/crates/core/src/access.rs` for decision 5 alone,
`conformance/case-schema/expect/translations.json`, `conformance/language/sectioned/index.yaml`
and `resource-owner/case.yaml`, and `engine/STANDING.md` only by the tool. Create only the eight
case directories and `docs/work-orders/286-declared.yaml`. Change nothing else: not another case,
K3, `conformance/s1-promises.yaml` or `manifest.yaml`, `docs/decisions/`, `IN_FORCE.md`,
`ROADMAP.md`, `packages/`, `tools/`, `experiments/`, another engine crate, `Cargo.toml` or
`Cargo.lock`. Report anything else that needs changing.

## Done when

The first group prints otherwise on `main` today and passes after; the second, guards, passes
both times. Run its last line, the whole engine suite, once, last.

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. sectioned-pool-key- 2>&1 | tail -1)  # 8 cases passed; today none matched
k spec/promises/sections.yaml 'whether sectioned or framed, that states no owner.' 'keeps it, whichever Pool it names.' 'keep their Pool (POOL-002).' '`fleet`, `pool`, `templates`' 'SCT-011, SCT-016 and SCT-017.' 'per-file rules SCT-011 and SCT-017;' 'unless SCT-017 supplies it;' 'and `fleet` and `pool` values' 'a supplied Pool owner to `/pool`;' 'A Fleet or Pool shared by several files'  # 1 ten times; 0 today
echo "$(j spec/promises/manifest.yaml '`fleet`, `pool` and every instance section') $(j spec/promises/submissions.yaml 'SCT-011 and SCT-017 supply owners') $(j spec/promises/schemas.yaml 'available for SCT-011 and SCT-017.')"  # 1 1 1; 0 0 0 today
sed -n '/^  name: sectioned.v1$/,/^---$/p' spec/schemas/sectioned.yaml | grep -c -e '^    pool:$' -e '^    - pool$'  # 2; 0 today
sed -n '/^  name: sectioned-resource.v1$/,/^---$/p' spec/schemas/sectioned.yaml | grep -c '^    - owner$'  # 0; 2 today
sed -n '/^  name: pool$/,/^---$/p' spec/core/templates.yaml | grep -c 'core#pool.v1$'  # 1; 0 today
grep -c '^  - pool$' spec/core/conduit.yaml  # 1; 0 today
k spec/core/README.md '| `pool` | Pool template | SCT-017 |' 'The 18 templates' 'exports 146 documents'  # 1 1 1; 0 0 0 today
k spec/definitions.md '`fleet` and `pool` are per-file keys' 'only for `fleet` and `pool`' 'a `pool` value Pool (SCT-007, SCT-011, SCT-017)' '| `pool` | A per-file short-form key'  # 1 1 1 1; 0 0 0 0 today
k spec/README.md 'It writes neither `fleet` nor `pool`.' '519 in all: 226 of the layer `language`' 'per-file Fleet and Pool ownership' '146 documents exported' '18 templates'  # 1 1 1 1 1; 0 0 0 0 0 today
grep -c '^ *"pool",$' engine/crates/core/src/access.rs  # 1; 0 today
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '(DEC-081)\.$'  # 2; 0 today
grep -c -e '^| SCT-017 |' -e 'Work order 286 added the row of SCT-017' docs/implementation/SAFETY_CATALOGUE.md  # 2; 0 today
grep -c '"SCT-017"' conformance/case-schema/expect/translations.json  # 1; 0 today
test -f docs/work-orders/286-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n=conformance/language/sectioned
git add -N "$n"/pool-key-* docs/work-orders/286-declared.yaml
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. section 2>&1 | tail -1)  # every sectioned case passes
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-loader -p conduit-core --all-targets -- -D warnings)
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0; the no-case ratchet holds
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/check_model_account.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/set_names.py --check | tail -1  # ends Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/286-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(promises/(sections|manifest|submissions|schemas)\.yaml|schemas/sectioned\.yaml|core/(templates\.yaml|conduit\.yaml|README\.md)|definitions\.md|README\.md|CHANGES\.md)|docs/implementation/SAFETY_CATALOGUE\.md|engine/crates/(loader/(src/(sectioned|locations)\.rs|tests/[^/]+\.rs)|core/src/access\.rs)|engine/STANDING\.md|conformance/(case-schema/expect/translations\.json|language/sectioned/(index\.yaml|resource-owner/case\.yaml|pool-key-[a-z-]+/[^/]+))|docs/work-orders/286-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # every test passes
git reset -q -- "$n"/pool-key-* docs/work-orders/286-declared.yaml  # stage nothing
```

Report what changed per decision; each case's failing line on the unchanged loader, then its
passing line; `sectioned-resource-owner`'s message before and after; every check's last line.
Stage nothing.

## What follows

- Before the pin, each a later order's: K3's T1 and a worked S1 in the short form (the
  coordinator's design notes) written with `pool:`, which changes case 01's plan, text and
  digest; the S1 example.
- The coordinator sets DEC-081's Status at the merge.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main`.

**Worker-2's turn** (13:46 to 14:03) carried out the owner's DEC-081: a file's `pool: X` makes
Pool X once per project and supplies X as the owner of each Resource in that file that states
none, as `fleet:` does for Agents (SCT-011, SCT-017); a Resource's own `owner` still governs;
`conduit init` writes neither. The eight new cases failed against the unchanged loader and pass,
with the 69 sectioned cases, 68 loader tests and the whole vectors test.

**Two blind reads at once** (14:04 to 14:09): `turn-verifier-2.turn.v1` found a must (the
specification index still counted 37 Operation contracts) and a should; the Claude reader had no
must, three shoulds and two nits. The order's own Done-when lines named stale counts (146
documents); the right counts, 148 documents and 76 Schema contracts, govern. One fix turn (14:09
to 14:16) applied all seven, the count regression failing first at 37.

What it moves in S1's counts lands before the S1 rerun's pin, as DEC-081 asks. K3's case T1 and
the S1 short form adopt `pool:` in their own orders. The gate's run on the squash is the
coordinator's, at the merge.
