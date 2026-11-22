# Work order 283: installation values are `value.v1` Resources, named under `values:` (DEC-076, the spec)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors` is
empty. It needs no network and changes no engine code; if a crate is missing from the cache, stop
and fetch nothing. Run one cargo command at a time, and not `tools/gate.py`. Never print a whole
large file, diff or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Run no `conduit` outside a test, no `codex` or `claude`, and
sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is
public: no absolute local path, user or machine name, account, or private repository name.

## What and why

**The owner's answer** ([DEC-076](../decisions/log.md#dec-076-2026-11-22-installation-values-are-valuev1-resources)):
the durable form of private values -> "value.v1 Resources (Recommended)". Today an account or a
directory reaches a document as a `<...>` placeholder filled into a copy outside the repository
(DEC-068). This order writes the spec.

**Today.** OBJ-009 resolves a reference to an identity; nothing copies another object's content
into configuration, and INS-013 keeps no trace of where a value came from. The engine resolves,
so all clients get one answer (SCT-004).

**No case.** This order adds no promise ID: the rules go into the promises whose moment and
checks they are, so CONTRIBUTING's rule 8 asks no case and the no-case ratchet holds; the engine
order adds the cases. ACC-057 has no modelled row to mark (DEC-033).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing repeats an effect implicitly: a changed value changes no instance until it is replaced.
At risk: 3, a shared thing is owned once and never copied to an agent: one value Resource owns
it, but each instance naming it, an Agent too, holds a copy as configuration, as an argument's
is, and its source map names the owner; 5, every management action is available to an agent:
only root registers a value Resource (ACC-057), so no one else plants a value under a name in
use; anyone may name one.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: a Kubernetes ConfigMap beside a Secret, named by `configMapKeyRef`
and, made immutable, changed by delete and recreate; GitHub Actions `vars` beside `secrets`.
Not taken (the coordinator's design notes): a values file the CLI fills before plan (tfvars),
which MCP submitters cannot use; store references for values and secrets alike (`op://`), which
treat an account as a secret; `${ENV}`, `--var` or a prompt; a new promise file.

**When it runs.** On `main` from `c4fa2197` (#559). Through Conduit: one Codex worker turn, a
blind Codex verifier and Claude reader (DEC-071), one fix turn, the gate (DEC-046, DEC-033). By
ruling 89, each decision says what must hold; how is yours. The words, the template `value`, the
name `value-entries.v1`, a scalar value, `inspect.v1`'s `values` output and folding the rules
into existing promises are the coordinator's, open to the owner's strike.

**Shared files.** If a spec order sharing these files merges first, this one rebases.

**Stop rule.** Stop only for a contradiction between promises (give both), a step that needs an
engine change outside this scope (give its failing line), or a file outside Scope. Where this
order's wording contradicts a promise it does not change, the promise governs: write the wording
to fit, and list it under `own_choices` with every other choice no decision governs.

## The decisions

Wrap each insertion at its file's width.

1. **`value.v1`** (`spec/core/types.yaml`), a Type after `node.v1`, with a line in the header.
   Family `Resource`; it offers nothing, requires nothing and lists no control. Its schema: an
   object with `value`, required, a string, a number or a boolean, and `node`, optional, a
   `resource-reference.v1`; nothing else. Its description says: one installation value, such as
   an account, that a document names under `values` and never holds (OBJ-009); owned by `system`,
   registered by root alone (ACC-057); it runs nothing and changes only by retirement and a new
   registration; `node` names the node at which the value has its meaning (PLC-020), its own
   placement deciding nothing, and one naming anything but a node is rejected at registration;
   a value is not secret, and a secret stays on a secret Resource.
2. **The template `value`** (`spec/core/templates.yaml`): a Resource template of `value.v1` with
   the inputs `value`, to `/config/value`, and `node`, optional, to `/config/node`, each with its
   place's schema; a line in the header.
   - `spec/core/conduit.yaml` exports `value`, `value.v1` and `value-entries.v1`, each in its
     group.
   - `spec/core/README.md`, "Every export": "| `value` | Resource template | OBJ-009, ACC-057 |",
     "| `value.v1` | Type | OBJ-009, INS-013, ACC-057 |" and "| `value-entries.v1` | Schema of a
     document's part | INS-003, OBJ-009 |". Its counts become 150 documents, 77 Schema
     contracts, 52 under `spec/schemas`, 7 Type contracts (its row adds "of a value") and 18
     templates. Its line of `inspect.v1`'s outputs adds "`values` for an instance that a `values`
     entry filled" (INS-013).
3. **`value-entries.v1`** (`spec/schemas/instances.yaml`), a Schema after `pool-entry.v1`: an
   object each of whose members, named for an input, is `{kind: Resource, name: <name.v1>}`, both
   required and nothing else, so neither a bare name nor the resolved form. `values`, with that
   schema, stands beside each of the file's seven `arguments`. The header says what it encodes
   and that the engine checks the rest (INS-003, OBJ-009); `spec/schemas/README.md`'s row for
   `instances.yaml` lists it after `pool-entry.v1`.
4. **INS-003** (`spec/promises/instantiation.yaml`). `when`: "supplies, under `arguments`, a
   value" becomes "supplies, under `arguments` or under `values`, a value", and ", or supplies one
   input under both" ends it. `observable`, at its end: "Where an input is under both, the
   diagnostic names the input and both sections."
5. **INS-004.** `when`: "omits an input" becomes "omits, under both `arguments` and `values`, an
   input". `observable`, at its end: "An input given under either section is supplied, under
   INS-006 to INS-009 and INS-019, as an argument is."
6. **INS-005.** `when`: "An argument does not satisfy" becomes "An argument, or the value of the
   value Resource that a `values` entry names, does not satisfy". `observable`, at its end: "For a
   `values` entry, it names the value Resource and never holds the value (OBJ-009)."
7. **OBJ-009** (`spec/promises/objects.yaml`).
   - `then`, at its end: "A `values` entry is a reference too: it is resolved at that moment, and
     the instance stores the value that its value Resource holds at each destination of its
     input, as INS-006 assigns an argument, and the entry's resolved form only in its source map
     (INS-013)."
   - `observable`, at its end: "A `values` entry is resolved only where (1) it names a Resource
     of the core Type `value.v1` that `system` owns and that is not retired; (2) that Resource
     states no `node`, or its `node` is the instance's placement, so an instance with no
     placement takes no value tied to a node; (3) no reference schema governs the input's schema
     or any of its destinations, so a value never stands for an object or a secret, whose place
     takes a reference to a secret Resource alone (SEC-001); and (4) its value satisfies the
     input's schema (INS-005). Otherwise the instance document is rejected and the registration
     reports invalid under OBJ-023, and nothing falls back to the input's `default` or takes an
     optional input as left out. The diagnostic names the document, the place `/values/<input>`,
     the value Resource as written and the check that fails, and, where no value Resource has
     that name, gives as the remedy its registration by root. No diagnostic about a `values`
     entry, and no record of the registration, holds the value; each names the value Resource.
     From then on the value is configuration, shown and recorded where configuration is
     (`inspect.v1`, REC-002). The instance keeps the value it was made with: retiring the value
     Resource, or registering another under its name, changes no instance, under OBJ-010, and an
     instance takes a new value only by its replacement."
   - OBJ-013, `then`: "and, under `arguments`, values" becomes "and, under `arguments` or
     `values` (OBJ-009), values". OBJ-014, `then`: "may hold `arguments`, `target`" becomes "may
     hold `arguments`, `values`, `target`", and "may hold `arguments` and" becomes "may hold
     `arguments`, `values` and".
8. **INS-013.** `then`, after "The arguments are not kept as a separate layer.": "Nor are its
   `values` entries, apart from its source map: each place that one filled, as a JSON Pointer,
   with the resolved form of its value Resource (OBJ-009)." `observable`: "other than the
   template it came from." becomes "other than the template it came from; its source map says
   which came from `values`, and `inspect.v1` shows it under `values`."
9. **ACC-057** (`spec/conventions/access.yaml`). `when`: "would make an Agent." becomes "would
   make an Agent, or a Resource of the core Type `value.v1`." `observable`: "every Agent in a
   submission" becomes "every Agent and every such Resource in a submission". In `register.v1`'s
   description (`spec/core/operations.yaml`), "only root may submit an Agent on its own behalf"
   becomes "only root may submit an Agent or a value Resource on its own behalf". ACC-056's table
   stays: an engine test reads its causes verbatim.
10. **`inspect.v1`.** Description, after "`declared` shows each reference in its resolved form,
    `{kind, id}`.": "The optional output `values` is present for an instance that a `values`
    entry filled: its source map, each place a JSON Pointer into `declared`, with its value
    Resource in the resolved form. The value itself shows at its place in `declared`, as an
    argument does, and a value Resource's `declared` shows its `value` and `node` (INS-013)."
    `outputSchema`: `values`, an object whose names are `pointer.v1` and whose members are
    `{kind: Resource, id: <id.v1>}`, both required and nothing else.
11. **`spec/definitions.md`**, in "Objects and documents", after the `argument` row: "| value
    Resource, `values`, source map | A Resource of the core Type `value.v1`, holding one
    installation value, owned by `system` and registered by root alone (ACC-057). A document names
    it under `values` as `{kind: Resource, name}`; the engine writes the value in when the
    instance is made, never at a reference place, and keeps a source map from each place filled
    to the value Resource (OBJ-009, INS-013). A value is not secret. |". In the `instance` row,
    "`metadata` and `arguments`" becomes "`metadata`, `arguments` and `values`"; in the `core
    package` row, "of a node, of `management`" becomes "of a node, of a value, of `management`".
12. **`spec/README.md`**, after "Plan is available and never required.": "A plan resolves each
    `values` entry from the value Resources it inspects, under OBJ-009, and a plan and its record
    name each value by its value Resource and hold no value. Where an entry does not resolve, the
    plan stops at its document, naming the place, the value Resource and `conduit value set
    <name>`: that command registers a value Resource as root, from the person's terminal,
    showing the value as typed and saving it only on y."
13. **`spec/CHANGES.md`**, under `## [Unreleased]`, after its last item:

    ```text
    - Added: `value.v1`, the core Type of a value Resource, one installation value that may be tied to a node and that root alone registers (ACC-057); the core template `value`; and `value-entries.v1`, a document's `values` (DEC-076).
    - Changed: OBJ-013, OBJ-014, INS-003, INS-004 and INS-005, an instance document or a nested instantiation may give an input under `values` as `{kind: Resource, name}`, never under `arguments` too, and it is then supplied as an argument is (DEC-076).
    - Changed: OBJ-009, a `values` entry resolves to a value Resource not retired, tied to no node or the instance's, at no reference place and passing the input's schema, or the document is rejected, with no fallback to a default and no value in the diagnostic (DEC-076).
    - Changed: INS-013 and `inspect.v1`, the instance keeps a source map from each place a value filled to its value Resource, shown under `values` (DEC-076).
    ```

## What to read

The promises named, OBJ-010, OBJ-021, OBJ-023, INS-006 to INS-009, INS-019, PLC-001, PLC-020,
SEC-001, CTR-013 and REC-002; `node.v1` and `node`; the header of `spec/core/references.yaml`.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor c4fa2197 HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
sed -n '/^## DEC-076/,/^## DEC-077/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
k spec/promises/objects.yaml 'reports is LIF-031.' 'and, under `arguments`, values' 'may hold `arguments`, `target`' 'may hold `arguments` and'
k spec/promises/instantiation.yaml 'supplies, under `arguments`, a value' 'omits an input' 'An argument does not satisfy' 'The arguments are not kept as a separate layer.' 'other than the template it came from.'
k spec/conventions/access.yaml 'would make an Agent.' 'This applies to every Agent in a submission'
k spec/core/operations.yaml 'only root may submit an Agent on its own behalf' '`declared` shows each reference in its resolved form, `{kind, id}`.'
k spec/core/README.md 'exports 145 documents, 75' 'the 51 Schema' 'The 6 Type' 'The 17 templates' 'of a node, of `management`' '`inventory.administrators` for Pools'
k spec/definitions.md 'of a node, of `management`' 'may have `metadata` and `arguments`,'
j spec/README.md 'Plan is available and never required.'
git grep -l -e 'value\.v1' -e 'value-entries' -e '^  name: value$' -- spec | wc -l | tr -d ' '
grep -c '^ *arguments:' spec/schemas/instances.yaml
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

Up to the `git grep` line they print `main 0` and then only `1`s; then `0`, `7`, `engine 0` and
`0`. Where a line prints otherwise, its text has moved: work from where it stands now, and list
it under `own_choices`.

## Scope

Change, as the decisions say, `spec/core/types.yaml`, `templates.yaml`, `conduit.yaml`,
`operations.yaml` and `README.md`, `spec/schemas/instances.yaml` and `README.md`,
`spec/promises/instantiation.yaml` and `objects.yaml`, `spec/conventions/access.yaml`,
`spec/definitions.md`, `spec/README.md` and `spec/CHANGES.md`. Change nothing else: not
`engine/` (no promise is added, so `engine/STANDING.md` needs no write), `conformance/`,
`experiments/`, `docs/`, `packages/` or `tools/`. Report what else needs changing.

## Done when

Each line of the first group prints otherwise on `main` at `c4fa2197` ("today") and passes after:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
grep -c '^  name: value.v1$' spec/core/types.yaml  # 1; 0 today
grep -c '^  name: value$' spec/core/templates.yaml  # 1; 0 today
grep -cE '^  - value(\.v1|-entries\.v1)?$' spec/core/conduit.yaml  # 3; 0 today
grep -cE '^\| `value(\.v1|-entries\.v1)?` \|' spec/core/README.md  # 3; 0 today
k spec/core/README.md 'exports 150 documents, 77' 'the 52 Schema' 'The 7 Type' 'The 18 templates' 'of a node, of a value,' '`values` for an instance that a `values` entry filled'  # 1 six times; 0 today
grep -c '^  name: value-entries.v1$' spec/schemas/instances.yaml  # 1; 0 today
grep -c 'core#value-entries\.v1' spec/schemas/instances.yaml  # 7; 0 today
j spec/schemas/README.md '`pool-entry.v1`, `value-entries.v1`'  # 1; 0 today
k spec/promises/instantiation.yaml 'under `arguments` or under `values`, a value' 'or supplies one input under both.' 'names the input and both sections.' 'omits, under both `arguments` and `values`' 'An input given under either section is supplied' 'that a `values` entry names, does not satisfy' 'Nor are its `values` entries' 'its source map says which came from `values`'  # 1 eight times; 0 today
k spec/promises/objects.yaml 'A `values` entry is a reference too' 'that `system` owns and that is not retired' 'so a value never stands for an object or a secret' 'nothing falls back to the input' 'no record of the registration, holds the value' 'changes no instance, under OBJ-010' 'under `arguments` or `values` (OBJ-009), values' 'may hold `arguments`, `values`, `target`' 'may hold `arguments`, `values` and'  # 1 nine times; 0 today
k spec/conventions/access.yaml 'or a Resource of the core Type `value.v1`.' 'every Agent and every such Resource in a submission'  # 1 1; 0 0 today
k spec/core/operations.yaml 'submit an Agent or a value Resource on its own behalf' 'The optional output `values` is present for an instance'  # 1 1; 0 0 today
sed -n '/^  name: inspect.v1$/,/^---$/p' spec/core/operations.yaml | grep -c '^      values:$'  # 1; 0 today
k spec/definitions.md '| value Resource, `values`, source map |' 'of a node, of a value, of `management`' '`metadata`, `arguments` and `values`'  # 1 three times; 0 today
k spec/README.md 'name each value by its value Resource and hold no value' 'and `conduit value set <name>`'  # 1 1; 0 0 today
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '(DEC-076)\.$'  # 4; 0 today
```

Guards, which pass today and must still pass. Run the engine's suite once, last: it reads
`spec/core` and `spec/schemas`.

```sh
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0; its no-case ratchet shows head equal to base
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/check_model_account.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/set_names.py --check | tail -1  # ends Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^spec/(core/(types|templates|conduit|operations)\.yaml|(core|schemas)/README\.md|schemas/instances\.yaml|promises/(instantiation|objects)\.yaml|conventions/access\.yaml|definitions\.md|README\.md|CHANGES\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # every test passes
```

Report each changed sentence before and after, the new documents, every check's last line and
the ratchet's. Stage nothing.

## What follows

DEC-076's orders, in sequence:

- **The engine's** (M): `value.v1`, its `node` check and the template built; OBJ-009's checks
  with the source map; ACC-057 for value Resources, with ACC-056's row and `REFUSAL_CAUSES`
  together; `inspect.v1`'s `values`; a case for each amended promise. It may split.
- **The CLI's** (M, stacked): `conduit value set`, `list` and `unset`; the plan's value lines and
  stops (K3), with no value; records redacted by value name on export. It decides whether an
  instance holding the same value with no source map is unchanged (stage 3's logins).
- **The dogfood's** (S): stage 3's files name `account` and `directory` under `values:`, and
  DEC-068's copies outside the repository retire.
- **The short form's** (S, spec, if wanted): `values` in `sectioned.v1` (SCT-004).

DEC-076 is done once these and this order merge.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main`.

**Worker-2's turn** (13:23 to 13:33) wrote the owner's DEC-076 in spec/: the core Type and
template `value.v1`; a document's `values:` beside `arguments:`, in instances, declared Conduits
and Pool entries; resolution at registration with its checks and no fallback to a default; the
source map an instance keeps and inspect.v1 shows; registration of a value Resource by root
alone; and plan's behavior. All 16 Done-when lines failed first and pass; the no-case ratchet
stays at 105.

**Two blind reads at once** (13:34 to 13:39): `turn-verifier-2.turn.v1` found six musts and three
shoulds, the Claude reader two musts, five shoulds and a nit, all where `values:` meets promises
the order named or did not: a value forwarded through a nested instantiation, a replacement's
held values, a value's node against a placement, the rejection naming the exact section and
place, INS-017's equality, the published counts, and the glossary. One fix turn (13:39 to 13:45)
applied all seventeen, its readings listed as its own choices: a replacement holds the resolved
scalar with its source and node; the node check applies to each instance that receives the
value; a control's change to a mapped value updates the source map under REV-007.

The engine, CLI and dogfood orders follow. The gate's run on the squash is the coordinator's, at
the merge.
