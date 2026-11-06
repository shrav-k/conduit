# The schemas of documents

Status: **accepted by the owner on 2026-11-05** (plan-audit ruling 37), as one batch with
[the core package](../core/README.md) and
[the manifest's promises](../promises/manifest.yaml). It was written for decision G1 of the
[re-audit](../../docs/proposals/reaudit/README.md). The record of the batch is
[docs/proposals/core-and-schemas](../../docs/proposals/core-and-schemas/README.md), and what
the owner changed is in [APPLY.md](../../docs/proposals/core-and-schemas/APPLY.md). The
promises are the source of truth: where a schema and a promise differ, the schema is wrong.

## What this is

One JSON Schema for each thing an author writes or an engine must accept. Each is written in
YAML, in JSON Schema 2020-12, using only what [the schema profile](../schema-profile.md)
permits. The profile excludes `$schema` and `$id`, so no file carries either.

Each schema is a Schema contract (CTR-008) that the core package exports. That is not
decoration. The profile allows a `$ref` three forms: `#` and a JSON Pointer into the same
schema; a qualified reference to a Schema contract that a package exports; and `#` and a name,
for a Schema contract of the same project or package (SCH-015). Only the last two let one
schema use another, so the shared parts are contracts. The core package's own documents write
its address in full, the exception NAM-022 allows them, so each is referred to as
`github.com/shrav-k/conduit/core#<name>`. That also lets an operation state its inputs with
them: `register.v1` takes a list of `document.v1`.

To use the schemas with an ordinary JSON Schema 2020-12 validator, put the `spec` of every
Schema contract here and in [spec/core](../core/README.md) under one `$defs`, and rewrite each
`$ref` to point into it. `tools/check_schemas.py` does exactly that.

## Files

| File | Schema contracts | What they describe |
| --- | --- | --- |
| [document.yaml](document.yaml) | `document.v1` | Any template, instance or contract document |
| [envelope.yaml](envelope.yaml) | `api-version.v1`, `envelope.v1`, `template-metadata.v1`, `instance-metadata.v1`, `contract-metadata.v1` | `apiVersion`, what every document carries, and `metadata` |
| [templates.yaml](templates.yaml) | `template.v1`, `template-spec.v1`, `pool-template-spec.v1`, `conduit-template-spec.v1`, `template-input.v1`, `implementation-map.v1`, `runtime.v1`, `template-controls.v1` | A template document; the `spec` of an Interface, Agent or Resource template, of a Pool template, and of a Conduit template, single or composite, each with the closed list of places an input's destination may reach; an input; the `implementation` map; a runtime; a template's `controls` |
| [instances.yaml](instances.yaml) | `instance.v1`, `template-reference.v1`, `declared-conduit.v1`, `pool-entry.v1`, `conduit-component.v1` | An instance document of each family; the value of `from`; the three nested instantiations |
| [graphs.yaml](graphs.yaml) | `graph.v1`, `lifecycle-graph.v1`, `step.v1`, `value-binding.v1`, `value-reference.v1` | A graph with `steps` and `outputs`; the graph of `setup` or `retire`; a step; a value binding; a value reference |
| [contracts.yaml](contracts.yaml) | `contract.v1`, `type-spec.v1`, `capability-spec.v1`, `operation-spec.v1`, `requirement-spec.v1`, `applied-requirement.v1` | A contract document of each of the five kinds, and a requirement applied somewhere |
| [schema.yaml](schema.yaml) | `schema.v1` | A schema under the profile, with the three forms of a `$ref`. It is the `spec` of a Schema contract and the value of every `schema` key |
| [manifest.yaml](manifest.yaml) | `manifest.v1`, `implementation-item.v1` | The manifest of a project and of a package, and one implementation item |
| [values.yaml](values.yaml) | `name.v1`, `export-name.v1`, `contract-name.v1`, `address.v1`, `qualified-reference.v1`, `selection.v1`, `pointer.v1`, `destination.v1`, `version.v1`, `digest.v1`, `id.v1`, `revision.v1` | The small values the others share: names, addresses, the forms of a reference to an export, pointers, versions and digests |

The schemas of a reference to an object are the core package's reference schemas, in
[spec/core/references.yaml](../core/references.yaml), because they are also what makes a value
inside `config` or `arguments` a reference (OBJ-021).

| A reference written as | Schema |
| --- | --- |
| A bare name, where one kind is accepted | `name.v1`, as a branch of `interface-reference.v1`, `agent-reference.v1`, `resource-reference.v1`, `pool-reference.v1`, `conduit-reference.v1` and `template-reference.v1` |
| `{kind: <Kind>, name: <name>}` | `reference.v1`; with fewer kinds, `actor-reference.v1`, `consumer-reference.v1`, `target-reference.v1` |
| `{kind: Pool, name: <pool>, entry: <entry>}` | `pool-entry-reference.v1` |
| `{entry: <name>}` | `entry-reference.v1` |
| A qualified reference, `<address>#<name>` | `qualified-reference.v1` |
| `#<name>`, or a qualified reference, where a document selects something | `selection.v1` |

`{entry: <name>}` is accepted by every reference schema that accepts a Resource, because an
entry's argument is checked against the schema of a Resource template's input. Written
anywhere other than in an entry of a Pool template it is rejected (POOL-005), and that is the
engine's check.

## Which schema to use

| To check | Use |
| --- | --- |
| A document, whatever its kind | `document.v1` |
| A file named `conduit.yaml` or `conduit.json` | `manifest.v1` |
| Only that a document has `apiVersion` and one of the ten kinds | `envelope.v1` |
| A schema an author wrote | `schema.v1` |

## Closed and open

Every object these schemas describe is closed with `additionalProperties: false`, so a key this
specification does not define is rejected, as OBJ-019 requires. That is also how a field the
engine owns is rejected (OBJ-004), and a `required` key (OBJ-015), a `depends_on` (OBJ-019), a
`consumer` in a template (CON-020) and an `owner` in an entry (POOL-012).

An input's destination is closed in the same spirit (INS-012). The schema of each kind of
template `spec` carries, as one pattern, the list of places a destination may reach:

| Template | A destination is |
| --- | --- |
| Interface, Agent or Resource | `/config` or a place inside it; `/conduits/<name>/target`; `/conduits/<name>/arguments` or a place inside it |
| Pool | `/config` or a place inside it; `/entries/<name>/placement`; `/entries/<name>/arguments` or a place inside it |
| Conduit | `/operations`; `/requirements/<name>/arguments` or a place inside it; `/components/<name>/arguments` or a place inside it |

Any other destination is rejected by the schema: an item of `operations`, a whole nested
instantiation, a requirement's `requirement`, anything under `inputs`, and every key INS-012
names as fixed.

The schemas are open where OBJ-019 exempts, and in one place more, the last row:

| Place | How it is open |
| --- | --- |
| `config` | Any object. The type's schema checks it (INS-014) |
| `arguments` of an instance, of a nested instantiation and of an applied requirement | Any object. The inputs of the template or of the requirement check it |
| `annotations` | Any map of strings |
| The value of `literal` | Any value |
| The value of the `default` of a template's input and of a Requirement's input | Any value. The input's own schema checks it. OBJ-019 does not name this place among those it exempts, so read strictly it rejects a key inside a `default` that is a map. The schemas leave it open, and the difference is reported in the record |
| A schema | The names under `properties` and `$defs`, and the values of `const`, `enum`, `default` and `examples`, are the author's. The keywords are not: SCH-002 rejects one outside the profile, and `schema.v1` does |
| A map this specification says is keyed by name, and an `implementation`, `controls` or `tools` map, whose keys are selections | Any key. See the first row of the next table |

`envelope.v1` is the one object schema that is not closed. It states only what all ten kinds
share, and is always used together with the closed schema of one kind.

## What a schema cannot express

These promises are not checked by validating a document against a schema. The engine checks
them. "Tool" says whether `tools/check_schemas.py` applies the rule itself, as far as the
documents in front of it show, in its step 6; an example under
[examples/invalid-by-rule](examples/invalid-by-rule) exists only for those.

| What | Promises | Why a schema cannot | Tool |
| --- | --- | --- | --- |
| The rule for a key of a named map: the name of an input, a step, a declared Conduit, an entry, a runtime, a requirement or a component; that an Operation's input or output name is not empty; that a key of `implementation`, `controls` or `tools` is a selection, which is a qualified reference or `#name`; that a key of `packages` is an address | OBJ-022, CTR-002, TPL-003, TPL-010, PKG-020, NAM-001, NAM-004, NAM-018 | The profile has no keyword that constrains the keys of a map. `propertyNames` and `patternProperties` are outside it | — |
| The syntax of the file: duplicate keys, anchors, aliases, merge keys, tags, keys that are not strings; numbers with no JSON form; whole numbers out of range; one document or one array in a JSON file | SCH-012, SCH-017, SCH-006, SCH-018 | They are gone, or cannot be seen, once the file has been read into values | When it reads a file, in step 1, except SCH-018 |
| A standard argument written under `arguments`, and anything else inside an open place | OBJ-013, INS-003 | The place is open by OBJ-019 | — |
| That a reference resolves, and to the right thing: a template or an instance, a kind, a contract of the expected kind, an export; that a `$ref` written as a qualified reference or as `#name` names a Schema contract | OBJ-007 to OBJ-009, INS-002, TPL-002, TPL-005, CON-017, CON-021, CON-027, NAM-003, NAM-007, NAM-022, PKG-018, POOL-014, SCH-015 | It needs the other documents, the objects that exist and the installed packages | Only a selection or a `$ref` that names something in this repository, for its kind; SCH-015 has an example |
| That no two documents of one project or package share a name, whatever their kinds | NAM-027 | It needs the other documents | Yes |
| Other uniqueness across documents | OBJ-002, SUB-004, PKG-012, CTR-007, IFC-014, MAN-002 | It needs the other documents, or the objects that exist | — |
| Cycles | GRA-005, POOL-011, CON-019, CTR-005, PKG-014 | It needs the other documents | — |
| What an argument must satisfy, and what the resolved instance holds | INS-003 to INS-009, INS-013, INS-014, INS-019, INS-023, NAM-019, SCH-009, SCH-010 | It needs the template or the requirement, and its schemas | — |
| That two destinations do not overlap, and that a destination's parent is present | INS-010, INS-011 | It relates values in different parts of a document. Which places a destination may reach is in the schemas | — |
| That a Conduit template that is not composed and writes no `operations` has an input whose destination is `/operations` | CON-016 | The profile cannot say that some member of a map holds a value. The schema does reject such a template that declares no `inputs` at all | Yes |
| That a Conduit that applies `bound.v1` has `access` as its target, and applies it once | CON-028 | The target is in another document, and the profile cannot count the members of a map that hold one value | Yes: a template that applies it twice, and a target written in place beside a template the tool has |
| That no Operation contract outside the core package is marked `reservedToRoot: true` | CTR-009 | A schema sees one document and not the package it belongs to | Yes |
| That a location's document states no `placement` | PLC-008 | Only the type of the template says that an instance is a location | Yes, where the tool has the template |
| That `{entry: name}` is written only in an entry of a Pool template | POOL-005 | A reference schema governs an entry's argument and every other place alike | Only in the `placement`, `consumer` and `target` of an instance document and the `target` of a declared Conduit |
| The meaning of a graph: that what a step names exists, required inputs, literals against their schemas, declared outputs, which source is allowed in which graph, handles, model work | GRA-004, GRA-006 to GRA-008, GRA-010, GRA-011, GRA-014, GRA-021, GRA-022, GRA-024, TPL-006, IMP-004, IMP-005, IMP-009 | It needs the operations' contracts, and knowledge of which graph a step is in | — |
| Rules that tie two keys of one object together beyond "exactly one of": a requirement on placement states `schema` and no `inputs`; `tools` has a key for each operation in `implements`; a `graph` item implements exactly one operation; a control's `changes` are places its type's schema describes | CTR-005, PKG-020, GRA-014, CTR-006 | The profile has no `if`, `not` or dependent keyword | CTR-006 only, in step 5, for a control whose contract is in this repository |
| Placement, ownership and state | POOL-001 (the owner is a Pool instance), PLC-001 to PLC-007, PLC-018, CON-006, CON-022, CON-025 | They need the objects that exist | — |
| That the core package is not listed in a manifest | NAM-021 | The profile has no `not` | — |
| That a `pattern` is a regular expression of ECMA-262 | SCH-016 | Not expressible | — |
| Everything a package's archive must be, and which files of a directory are read | PKG-004, PKG-021, PKG-022, MAN-001, MAN-002, MAN-011, MAN-013, MAN-014 | It is about files, not a document | — |

A schema therefore answers one question: is this document, taken alone, one this specification
could accept. It never says a document is accepted.

## Choices the schemas encode

Where a promise left the shape open, the schema took the smallest reading, and the
[record](../../docs/proposals/core-and-schemas/README.md#b-choices-that-no-promise-states)
lists each. The owner accepted them all, and most are now stated by a promise: `metadata` and
`arguments` may be left out (OBJ-013, OBJ-014); a template narrows a control's input as
`inputs: {<name>: {schema: ...}}` (TPL-010); a schema may be a boolean. These remain choices
of the schemas alone: `optional` and `plain` are booleans, so `optional: false` is accepted; a
Requirement's input holds `schema` and may hold `default` and `optional`; and a Type's
`family` is one of the four families that have a type.

## Examples

| Path | What |
| --- | --- |
| [examples/valid/project](examples/valid/project) | A small project that is not published: a manifest; a Schema contract of its own, which a template reaches with a `$ref` written `#name`; a Resource template with an input, a runtime, a setup graph and a narrowed control; a Pool template with two entries, one referring to the other; a Conduit template with two requirements; a composite Conduit template; an Agent template with a required and an optional declared Conduit; an Interface template; a Conduit template that applies the bound of a right to open; and instance documents of every family, a Conduit to `access` among them |
| [examples/valid/files-package](examples/valid/files-package) | A package: a manifest with `address`, `version`, `exports`, a re-export and three implementations, and one contract of each of the five kinds, two of which use a `$ref` written `#name` |
| [examples/valid/first-start](examples/valid/first-start) | The instance documents of the five first-start objects |
| [examples/invalid](examples/invalid) | 50 files, each holding one document that a schema rejects, or that is refused when its file is read, and named for the promise it breaks. A file with `-manifest-` in its name is a manifest; the others are documents |
| [examples/invalid-by-rule](examples/invalid-by-rule) | 8 files, each holding one set of one or more documents and named for the promise the set breaks. Every document in them is valid against the schemas. What rejects the set is a rule no schema can express, which the tool applies itself and which the table above marks |

Every name under `example.org` is invented, and every digest in an example is illustrative.
Each directory under `examples/valid` is one project, with its documents beside its manifest, as
MAN-011 states. A set under `examples/invalid-by-rule` shows one rule and is not a whole
project: it may name objects and packages that are not written out.

## Checking

```sh
python3 tools/check_schemas.py
```

It reads every file as strict YAML, checks every schema against the profile, validates every
document here, in `spec/core` and in `examples/valid`, and checks that every document in
`examples/invalid` is rejected by a schema. It checks that the core package agrees with
itself and that the tables of this page and of the core package's page list what there is.
It then applies the few rules the table above marks, to the core package and the valid
examples, which must break none, and to each set in `examples/invalid-by-rule`, which the
rule its file is named for must reject. Its report says of each rejection whether a schema or
one of its own rules made it. It does not do anything else the table leaves to an engine.
