# Ruling 42: log of the renames for machines and placement

Date: 2026-11-06. This is the account of the editor who applied six of the names that
[ruling 42](README.md) accepted from the [vocabulary audit](../vocabulary-audit/README.md):
N2, N3, N8, N9, N10 and N11, as each subsection of
[Names for the owner](../vocabulary-audit/README.md#names-for-the-owner) says what changes with
it. The names are the owner's. The wording that carries them out is the editor's, and each
choice listed under [Choices beyond the record](#choices-beyond-the-record) stands unless the
coordinator or the owner strikes it. Nothing was committed.

Not applied here: the other names (N1, N4 to N7, N12 to N20) and the editorial fixes E1 to E23,
which another stage applies, so the text keeps `secret-reveal.v1`, `reveal-secret.v1`, operation
identity, origin and the rest; and D5 and D20, which the owner did not rule and which no rename
here touches. Where an editorial fix concerns a sentence that a rename reached (E10, E12, E13,
E23), the sentence is left for that fix, as [What was kept](#what-was-kept) says.

Scope: every file under `spec/`, `tools/check_schemas.py` and `docs/OVERVIEW.md`. No other
file was changed but this log. Each changed YAML file has a line at the end of its head comment
naming ruling 42's names, this log and what changed; the Markdown files say it in their head
paragraphs (`spec/README.md`, `spec/definitions.md`, `spec/core/README.md`,
`spec/schemas/README.md`, `docs/OVERVIEW.md`) and the checker in the comment above `RULES`.

How it was done: the tree was copied aside, a first pass replaced the word location and the
phrase machines provider outside the head comments and the `# ---` section comments, and then
every occurrence of every old name was read by hand, in the copy, against what it names. The
result was applied to the working tree as one patch, after checking that no other job had
changed these files since the copy was taken, and the two example files were renamed with
`git mv`, which stages those two renames.

## Checks

Run from the repository root after the renames.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | 439 promises ok |
| Every promise has exactly the seven fields, its text fields folded with `>-` | Yes |
| A YAML line over 96 characters under `spec/` | None |
| `python3 tools/check_docs.py` | 0 errors in 150 Markdown files and 1985 local links, with this log written |
| `python3 tools/set_names.py --check` | 0 errors |
| `python3 tools/check_schemas.py` | 0 errors. The rules `node-type` (PLC-028) and `label-conflict` (PLC-025) reject their examples under the new names |
| `git diff --check` | Clean |

No promise was added, removed or moved, and no ID changed. 70 promises in 17 files changed, each
only by these names, as the head comment of each file lists:

| File | Promises changed |
| --- | --- |
| `promises/placement.yaml` | 19: PLC-001, PLC-002, PLC-005, PLC-007, PLC-008, PLC-011 to PLC-013, PLC-015, PLC-017 to PLC-020, PLC-023 to PLC-028 |
| `promises/providers.yaml` | 13: PRV-001 to PRV-003, PRV-006 to PRV-008, PRV-013 to PRV-015, PRV-018 to PRV-021 |
| `conventions/lifecycle.yaml` | 13: LIF-007, LIF-009, LIF-010, LIF-015, LIF-017, LIF-018, LIF-022 to LIF-024, LIF-026, LIF-034 to LIF-036 |
| `conventions/access.yaml` | 3: ACC-032, ACC-033, ACC-037 |
| `promises/first-start.yaml` | 3: FST-001, FST-005, FST-006 |
| `promises/manifest.yaml` | 3: MAN-009, MAN-010, MAN-012 |
| `promises/pools.yaml` | 3: POOL-004, POOL-009, POOL-016 |
| `interface/implementation.yaml` | 2: IFC-017, IFC-037 |
| `promises/conduits.yaml` | 2: CON-007, CON-008 |
| `promises/objects.yaml` | 2: OBJ-019, OBJ-022 |
| `promises/contracts.yaml`, `packages.yaml`, `instantiation.yaml`, `templates.yaml` | 1 each: CTR-004, PKG-019, INS-018, TPL-018 |
| `conventions/observation.yaml`, `records.yaml`, `revisions.yaml` | 1 each: OBS-003, REC-005, REV-014 |

`implementation-defined.yaml` changed DL5. The counts below are of occurrences under `spec/`,
in `tools/check_schemas.py` and in `docs/OVERVIEW.md`, before and after, leaving out the lines
that this pass wrote to record the renames.

## The names

### N10: location becomes node, `location.v1` becomes `node.v1`, "stand for" becomes "represent"

| What | Changed | Kept |
| --- | --- | --- |
| The word location, locations, Location | 269 | 31 |
| `location.v1`, in prose, in qualified references and as an export | 22 | 5 |
| "stands for", "stand for", "for which `local` stands", "no location stands" | 54 | 9 |
| The core template `location`, `declare-lost.v1`'s input `location`, the example project's input `location`, and the checker's strings | 32 | 3 |

Files: every promise file listed above but `manifest.yaml` and `objects.yaml`;
`implementation-defined.yaml`; `spec/README.md`, `spec/definitions.md`; `spec/core/README.md`,
`conduit.yaml`, `operations.yaml`, `references.yaml`, `requirements.yaml`, `templates.yaml` and
`types.yaml`; `spec/schemas/README.md`, `contracts.yaml` and `values.yaml`; the examples
`valid/first-start/first-start.yaml`, `valid/project/instances.yaml`,
`valid/project/templates.yaml`, `valid/files-package/contracts.yaml`, and under
`invalid-by-rule/` both PLC-025 files and the two renamed files; `tools/check_schemas.py`;
`docs/OVERVIEW.md`.

What changed with it, as the subsection asks:

- A value tied to a location is a value tied to a node (PLC-020 and the row), "declare a
  location lost" is "declare a node lost", and `declare-lost.v1`'s input `location` is `node`.
- The core package's template `location` is `node` (its manifest, `templates.yaml`, FST-005,
  FST-006, PLC-026, PLC-028, the export tables of the two READMEs, the first-start table and
  the checker's `FIRST_START_TEMPLATES`).
- `tools/check_schemas.py`: the rule `location-type` is `node-type`, its variable and message
  name `node.v1` and the template `node`, and the comment of `first-start-from` names `node`.
- Two example files were renamed with `git mv`:
  `invalid-by-rule/FST-006-instance-from-location.yaml` to `FST-006-instance-from-node.yaml`,
  and `invalid-by-rule/PLC-028-template-of-the-location-type.yaml` to
  `PLC-028-template-of-the-node-type.yaml`. The checker names no example file, so nothing else
  follows; the records that cite the old names (ruling 41's work order and integration log)
  keep them.
- `spec/definitions.md`: the row location is node, and says that a node represents its machine
  as Kubernetes represents a machine by a Node, that the policies are not Kubernetes' (it evicts
  the Pods of an unreachable Node and lets a user label a Node, and Conduit does neither), and
  that a node is not a YAML node. The row stand for is represent, the row location type is node
  type, and the row value tied to a location is value tied to a node.
- Where a promise names Kubernetes' own node, it now writes Node, so that it is not read as
  Conduit's: PLC-023 (a Node's condition `Ready`), PRV-014 (a Kubernetes Node), PRV-021 (what
  its Node should no longer run) and IFC-037 (the kubelet registers no Node). PLC-025 writes the
  `scheduling.nodeSelector` of a RuntimeClass, as PLC-007 does, where it said "the node
  selector".

### N2: the machines provider becomes the machine provider

| What | Changed | Kept |
| --- | --- | --- |
| machines provider, also across a line break | 88 | 13 |
| the engine's own machine, as the built-in provider, now the engine's own machine provider | 13 | 1 |
| the engine's own machine, as the computer, now the machine the engine runs on | 4 | 0 |
| the engine's own runner, machine and fetching, now runner, machine provider and fetching | 5 | 0 |

Files: `docs/OVERVIEW.md`, `spec/README.md`, `spec/definitions.md`, `lifecycle.yaml`,
`observation.yaml`, `implementation-defined.yaml`, `first-start.yaml`, `manifest.yaml` (MAN-012),
`placement.yaml`, `pools.yaml`, `providers.yaml`, `spec/core/README.md`, `capabilities.yaml`,
`conduit.yaml`, `operations.yaml`, `templates.yaml`, `types.yaml`, `spec/schemas/README.md`, and
three examples.

The capability stays `machine-provider.v1`. The row title is "the engine's own runner, machine
provider and fetching", and the row provider adds that these are part of the engine and are no
Resource (PRV-003), as the subsection asks. "The machine the engine runs on" stays for the
computer, and "own runner" and "fetching" stay. The section heading of `operations.yaml`, "The
operations of the machines provider", was renamed too: it describes the file, and records no
ruling.

### N3: `run.v1`'s input `location` becomes `machine`

10 occurrences in 4 files: `run.v1`'s input, its `required`, its description and its effect in
`spec/core/operations.yaml`; PRV-007 (three), PRV-019 and PLC-027; and the section on providers
of `spec/core/README.md`. The input holds `hostname` and `labels`, as before. The rows run and
runtime provider now say that `run.v1` gives the machine in `machine`. "The machine that
`location` names" became "the machine given in `machine`" in PRV-019's `then` and in the effect
of `run.v1`, and "the machine of the instance's node" in PRV-007's observable (see the choices
below). PRV-007's observable says that `machine` always holds the machine of a node that is not
retired.

### N8: `label-selector.v1` becomes `labels.v1`

14 occurrences changed in 9 files: `values.yaml` (the contract and its line under "Promises
encoded"), `contracts.yaml` (`type-spec.v1`), `manifest.yaml` (`implementation-item.v1` and the
core form of `manifest.v1`), `operations.yaml` (`run.v1`'s `machine.labels`,
`list-machines.v1`'s `machines[].labels` and `place.v1`'s `nodeSelector`), the core manifest's
exports, the export tables of `spec/core/README.md` and `spec/schemas/README.md` (two rows),
OBJ-022 and the row label selector. One kept: the head comment of `values.yaml` that records
that ruling 41 added `label-selector.v1`.

The description of `labels.v1` now reads as the subsection recommends: a map from a label to its
value, the shape of Kubernetes labels and of a `nodeSelector`, which holds the labels reported
for a machine and a label selector. The sentence "An empty map selects every location" left the
description, since it is wrong for reported labels; the rule stays in CTR-004 and in the row
label selector. "Label selector" stays in the prose for the role.

### N9: the key `placement` of a Type or an item, and `place.v1`'s `selector`, become `nodeSelector`

45 occurrences of the key `placement` changed, and 3 of `selector` (`place.v1`'s input, its
`required` and its description), in 16 files: `type-spec.v1` (`contracts.yaml`),
`implementation-item.v1` and the core form of `manifest.v1` (`manifest.yaml`), `place.v1`
(`operations.yaml`), the comment of `types.yaml`, CTR-004, MAN-009, MAN-010, MAN-012, OBJ-019,
PKG-019, PLC-007, PLC-025, the rows type, implementation item and label selector,
`spec/schemas/README.md` (the row for PLC-025), `docs/OVERVIEW.md`, the checker's
`label-conflict` (code, comment and messages), and the examples
`valid/files-package/conduit.yaml` (both items), `valid/files-package/contracts.yaml`,
`invalid-by-rule/PLC-025-type-and-item-name-one-label.yaml` and
`invalid/CTR-004-records-as-a-list.yaml`, whose Type wrote `placement: {}` and would otherwise
have been rejected for an unknown key as well as for its `records`.

`placement` now means only the standard argument. Every other `placement` was read and kept:
an instance's `placement` (OBJ-013, PLC-001 and the rest, the schemas of instances, the
examples), an entry's `placement` as a destination (INS-012, POOL-009, PLC-005, the
checker's `entry-reference`), and the word placement in prose. The rows say that the selector is
matched against the labels reported for the machine of a node, as the subsection asks.

### N11: "counts as declared lost" becomes "the engine declares the node lost"

5 phrases removed in 3 files: "counts from then on as declared lost" in PLC-024 and in the rows
declare lost and unreachable, gone; "or counts as declared lost, under PLC-024" in PLC-027; and
"declared lost in this sense" in LIF-017. What was written in their place:

- PLC-024 `then`: the engine declares the node that represents the machine lost, with the effect
  that PLC-017 states where the root Interface declares a node lost. Its observable compares it
  with the root Interface's declaration, where it compared it with "a declaration of loss".
- PLC-017's observable ends: the engine declares a node lost in the same way where the machine
  provider reports its machine gone, under PLC-024. PLC-027's `when` reads "declared lost, under
  PLC-017 or PLC-024". LIF-017: such a node "was declared lost by the engine".
- The row declare lost now defines the act, retiring a node and every instance on it without
  their shutdown being confirmed: the root Interface does it with `declare-lost.v1`, and the
  engine does it itself when its machine is reported gone. `declare-lost.v1`'s description and
  `docs/OVERVIEW.md` say the same.

PLC-011 and CON-008, which name both causes, were read and left as they are: what they say is
true under either wording.

### The words the promises do not use

`spec/definitions.md` lists the earlier words in five new rows: location, `location.v1`,
location type and stand for; machines provider and the engine's own machine;
`label-selector.v1`, `placement` of a Type or an item and `selector` of `place.v1`;
`run.v1`'s `location`; and counts as declared lost. Each says what replaced it and which name of
the audit did.

## What was kept

Every remaining occurrence of an old name under `spec/`, in the checker and in the overview,
found by searching for each, and why it stays.

| Where | Old word | Why |
| --- | --- | --- |
| Head comments of `placement.yaml` (ruling 41's summary of the file, its consolidation, round 2, ruling 41, integration and ruling 42 lines), `providers.yaml` (ruling 41's description of the file, its standards, its coverage check and ruling 42's line), `first-start.yaml`, `pools.yaml`, `lifecycle.yaml`, `revisions.yaml`, `implementation-defined.yaml`, and `core/types.yaml`, `templates.yaml`, `operations.yaml`, `requirements.yaml`, `capabilities.yaml`, `values.yaml` | location, `location.v1`, machines provider, stand for, `location` | They record what an earlier pass did, in the words of its time |
| `# ---` section comments of `placement.yaml` (two) and `providers.yaml` (one) | location | They record what ruling 41 and its coverage check added |
| `spec/definitions.md`, head paragraph: the rows the coverage check, the docs audit and ruling 42's docs editor changed | location | The same: a record of earlier passes |
| `spec/definitions.md`, the rows "facts, location fact, host, ..." and "default location, `set-default-location.v1`, ..." | location fact, `location-settings.v1`, default location, `set-default-location.v1` | Names that ruling 41 removed, listed as such |
| `spec/definitions.md`, the five new rows | every old word | They list the old words as words the promises do not use |
| `spec/core/README.md`, ruling 41's paragraph ("default-location and location-facts operations", "the machine that `location` names") | location, `location` | Record of ruling 41's pass and its coverage check |
| `spec/core/results.yaml` (`diagnostic.v1`: "each logical location of `locations`", the key `locations`), `spec/schema-profile.md`, `spec/schemas/README.md` and the row diagnostic ("logical location"), `spec/core/README.md` ("its logical location") | location, `locations` | SARIF's term, which the task keeps |
| SCH-015 (`schemas.yaml`: "Each of the last two stands for the schema") | stands for | A `$ref`, not a node and its machine. E12 rewrites it as "references" |
| `spec/core/README.md`, "One contract stands for one verb" | stands for | Not the term. E13 rewrites it as "One contract for one verb" |
| `tools/check_schemas.py`, the docstrings of the `$ref` rewriting and of the schemas that `schema` "stands for" | stands for | A `$ref` again, in a checker's comment; the same sense as SCH-015 |
| `spec/README.md`, "the node lifecycle and well-known labels of Kubernetes" | node | Kubernetes' own term, and the sentence says so |
| PRV-021, "the runtime provider plays the part of their node agents" | node | The kubelet and the Nomad client, which "their" ties to Kubernetes and Nomad |
| JSON Schema's "instance location", YAML's nodes and MCP's terms | | None of them occurs under `spec/` in a form that this pass touched |

The other names and the editorial fixes are not applied, so these stay as they are for the
stage that applies them: the row node type (E10 removes it, and `types.yaml`'s description
"The node type."), NAM-020 cited for the template `node` in the core README and in
`templates.yaml` (E23), and the sentences that call the coined names not final, in
`spec/README.md`, `spec/definitions.md` and the core README, which now also say which names are
applied.

## Choices beyond the record

1. **"The engine's own machine" for the computer.** Where the phrase meant the machine the
   engine runs on, not the built-in provider, it became "the machine the engine runs on", which
   N2 keeps for the computer: in `place.v1`'s description, in the section on providers of the
   core README and twice in `docs/OVERVIEW.md`. Renaming it "the engine's own machine provider"
   there would have been wrong.
2. **"That machine" in N3.** The subsection says "the machine that `location` names" becomes
   "that machine". In PRV-019's `then` and in `run.v1`'s effect there is no machine before it to
   point back to, so they say "the machine given in `machine`"; in PRV-007's observable, which
   is read apart from its `then`, "the machine of the instance's node".
3. **The description of `labels.v1`.** Written from the subsection's recommendation, without
   the empty-map sentence (see N8).
4. **A Kubernetes Node.** Five sentences that name Kubernetes' node now write Node, and PLC-025
   writes `scheduling.nodeSelector` (see N10). Without it "node" in those sentences would read
   as Conduit's.
5. **The example project's input.** The Pool template `workspace` in `valid/project/` had an
   input named `location`, which supplies its entries' placement, and the instance `team` gave
   it. Both are now `node`, so that the example uses the name in force.
6. **The checker.** The rule name `location-type` became `node-type`, and its variable
   `location` became `node_type`. The rule's promise, PLC-028, did not change.
7. **The words-not-used rows** (see above) are this editor's wording.
8. **The node row** says that a node "is not a YAML node", as the subsection asks, and that
   Kubernetes' policies differ, in two cases taken from PLC-023 and PLC-026.

## Search for the old names after the renames

Each search covers `spec/`, `tools/check_schemas.py` and `docs/OVERVIEW.md`, across line
breaks. What it finds, apart from the records of this pass and the words-not-used rows, is the
table under [What was kept](#what-was-kept).

| Old name | Remaining | Where |
| --- | --- | --- |
| location, locations | 31 | Head comments and section comments recording earlier rulings (20), the definitions head paragraph (3), the two rows of names that ruling 41 removed (2), and SARIF's logical locations and `locations` (6) |
| `location.v1` | 5 | Head comments of `types.yaml` (2) and `first-start.yaml`, `set-default-location.v1` in `operations.yaml`'s head and in the row of removed names |
| `location` in backticks | 3 | Head comments of `templates.yaml` and `operations.yaml`, and ruling 41's paragraph of the core README |
| stand for, stands for | 9 | Head comments (5), SCH-015, the core README's "One contract stands for one verb", and two docstrings of the checker |
| machines provider | 13 | Head comments only |
| the engine's own machine, as the built-in | 1 | The head comment of `providers.yaml` that describes the file as ruling 41 wrote it |
| `label-selector.v1` | 1 | The head comment of `values.yaml` |
| `placement` as a key of a Type, an item or a manifest's core item | 0 | Every remaining `placement` is the standard argument, an entry's destination or prose |
| `place.v1`'s `selector` | 0 | |
| counts as declared lost, in this sense (of a node) | 0 | |
