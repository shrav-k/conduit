# The schemas of documents

Status: **accepted by the owner on 2026-11-05** (plan-audit ruling 37), as it stood on that
day, as one batch with [the core package](../core/README.md) and
[the manifest's promises](../promises/manifest.yaml). **Changed since by decisions that the
owner has not ruled on.** It was written for decision G1 of the
[re-audit](../../docs/proposals/reaudit/README.md). The record of the batch is
[docs/proposals/core-and-schemas](../../docs/proposals/core-and-schemas/README.md), and what
the owner changed is in [APPLY.md](../../docs/proposals/core-and-schemas/APPLY.md).

What follows was changed after the owner's ruling, by the coordinator of the re-audit, whose
decisions are logged and can be struck by the owner. None of it is a choice the owner accepted.
The decisions of the [docs audit](../../docs/proposals/docs-audit/GAPS.md) added four things
a contract may hold, which are listed under
[Choices the schemas encode](#choices-the-schemas-encode). The
[decisions of the second re-audit round](../../docs/proposals/reaudit/round-2/DECISIONS.md)
changed seven shapes, which are listed in the same place; what changed in each file is in the
[log of that pass](../../docs/proposals/reaudit/round-2/log-documents.md) and the
[log of its closing pass](../../docs/proposals/reaudit/round-2/log-closing.md). The
[last pass of that round](../../docs/proposals/reaudit/round-2/log-tail.md) changed no shape:
it added three rules that no schema can express, with an example of each. The
[decisions of the third round](../../docs/proposals/reaudit/round-3/DECISIONS.md) changed no
shape of a document: they added three more such rules, with an example of each, and two
examples of a manifest that a schema rejects, and their follow-up took `supports` from an
implementation item that states `graph`; the
[log of that pass](../../docs/proposals/reaudit/round-3/log-fixes.md) says what changed in
each file. The
[decisions of the fourth round](../../docs/proposals/reaudit/round-4/DECISIONS.md) changed one
shape, the `conduit` of a step, which may now name a component of a composite declared
Conduit by a dotted path, and added four rules that no schema can express, with an example of
each; the [log of that pass](../../docs/proposals/reaudit/round-4/log-fixes.md) says what
changed in each file. The promises are the source of truth: where a schema and a
promise differ, the schema is wrong.

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
| [instances.yaml](instances.yaml) | `instance.v1`, `template-reference.v1`, `declared-conduit.v1`, `pool-entry.v1`, `conduit-component.v1` | An instance document of each family; the four forms of the value of `from`; the three nested instantiations |
| [graphs.yaml](graphs.yaml) | `graph.v1`, `lifecycle-graph.v1`, `step.v1`, `value-binding.v1`, `value-reference.v1` | A graph with `steps` and `outputs`; the graph of `setup` or `retire`; a step; a value binding; a value reference |
| [contracts.yaml](contracts.yaml) | `contract.v1`, `type-spec.v1`, `capability-spec.v1`, `operation-spec.v1`, `requirement-spec.v1`, `applied-requirement.v1` | A contract document of each of the five kinds, and a requirement applied somewhere. `operation-spec.v1` carries a control's `changes`, a map from the name of an input to the pointer that input sets (CTR-006). `requirement-spec.v1` carries the form of a constituent of a composed requirement, whose `arguments` are value bindings with the one source `input` (CTR-010) |
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
| `{kind: <Kind>, id: <ID>}`, the resolved form, which only the engine writes | Every reference schema, as its last branch, with the kinds that schema accepts |
| A qualified reference, `<address>#<name>` | `qualified-reference.v1` |
| `#<name>`, or a qualified reference, where a document selects something | `selection.v1` |
| The value of `from`: a bare name or `{kind: <Kind>, name: <name>}` for a template that exists, a qualified reference for a template that a package exports, or `#<name>`, which names a template of the same package and, in a submitted project, resolves as the bare name does (INS-025) | `template-reference.v1`. It is not a selection, and a template's name holds no dot, so its `#<name>` form holds none |

`{entry: <name>}` is accepted by every reference schema that accepts a Resource, because an
entry's argument is checked against the schema of a Resource template's input. Written
anywhere other than in an entry of a Pool template it is rejected (POOL-005), and that is the
engine's check.

### The resolved form of a reference

Where a value is a reference, the instance stores `{kind: <Kind>, id: <ID>}` in its place when
it is made (OBJ-009, OBJ-021): the kind and the ID of the object the reference resolved to.
Every reference schema accepts that form, so the stored configuration still satisfies the
schema of the type that INS-014 checked it against, and a control or a reconciliation that
stores a setting is checked against the same schema (REV-008, REV-013). Only the engine
writes the form. An implementation receives it in its `arguments` and in the restrictions it
is sent, a graph reads it through a value reference, a control returns it in `settings`, and
`inspect.v1` shows it under `declared`. A document never writes it, and nor does a request
that an actor makes on its own behalf: the engine rejects a submitted document that does
(OBJ-009), and such a request reports invalid (LIF-031). A step of a graph and an onward
request may hold it, because the engine handed it to them.

No schema here describes a resolved instance, because `inspect.v1` leaves the shape of
`declared` open, so there is no example file of one. This is what the resolved definition of
the Agent `reviewer-1` of the [example project](examples/valid/project) holds once it is
instantiated, as `inspect.v1` would show it, with an ID invented for the example:

```yaml
# Written in the instance document:   checkout: {kind: Pool, name: team, entry: checkout}
# Stored, and shown under `declared`, as the `target` of the declared Conduit `checkout`:
target: {kind: Resource, id: 0b5c1d7e-2a43-4f6e-9d10-6c8e4f1a2b3c}
```

## Which schema to use

| To check | Use |
| --- | --- |
| A document, whatever its kind | `document.v1` |
| A file named `conduit.yaml` or `conduit.json` | `manifest.v1` |
| Only that a document has `apiVersion` and one of the ten kinds | `envelope.v1` |
| A schema an author wrote | `schema.v1` |

## Closed and open

Every map these schemas describe is closed with `additionalProperties: false`, so a key this
specification does not define is rejected, as OBJ-019 requires. That is also how a field the
engine owns is rejected (OBJ-004), and a `required` key (OBJ-015), a `depends_on` (OBJ-019), a
`consumer` in a template (CON-020), an `owner` or a `placement` in an entry (OBJ-014) and
`optional` on a runtime (TPL-006).

An input's destination is closed in the same spirit (INS-012). The schema of each kind of
template `spec` carries, as one pattern, the list of places a destination may reach:

| Template | A destination is |
| --- | --- |
| Interface, Agent or Resource | `/config` or a place inside it; `/conduits/<name>/target`; `/conduits/<name>/arguments` or a place inside it |
| Pool | `/config` or a place inside it; `/entries/<name>/placement`; `/entries/<name>/arguments` or a place inside it |
| Conduit that is not a composite | `/operations`; `/requirements/<name>/arguments` or a place inside it |
| Composite Conduit | `/components/<name>/arguments` or a place inside it |

Any other destination is rejected by the schema: an item of `operations`, `operations` in a
composite, a whole nested instantiation, a requirement's `requirement`, anything under
`inputs`, and every key INS-012 names as fixed.

The schemas are open where OBJ-019 exempts:

| Place | How it is open |
| --- | --- |
| `config` | Any map. The type's schema checks it (INS-014) |
| `arguments` of an instance, of a nested instantiation and of an applied requirement | Any map. The inputs of the template or of the requirement check it. One that is left out is present and empty, as a `config` that is not written is (INS-011, INS-024) |
| `annotations` | Any map of strings |
| The value of `literal` | Any value |
| The value of the `default` of an input, whether a template, an Operation contract or a Requirement contract declares it | Any value. It is a literal value, which OBJ-019 exempts. The input's own schema checks it (SCH-019) |
| A schema | The names under `properties` and `$defs`, and the values of `const`, `enum`, `default` and `examples`, are the author's. The keywords are not: SCH-002 rejects one outside the profile, and `schema.v1` does |
| A map this specification says is keyed by name, and an `implementation`, `controls` or `tools` map, whose keys are selections | Any key. See the first row of the next table |

`envelope.v1` is the one schema of a map that is not closed. It states only what all ten kinds
share, and is always used together with the closed schema of one kind. The helper schemas
under `$defs` that carry the pattern of a destination are open too, by design: each is
joined with `allOf` to the closed `template-input.v1`. `object.v1` and `diagnostic.v1`, the
two schemas of results in [spec/core](../core/results.yaml), are open so that an engine may
add members.

## What a schema cannot express

These promises are not checked by validating a document against a schema. The engine checks
them. "Tool" says whether `tools/check_schemas.py` applies the rule itself, as far as the
documents in front of it show, in its step 6; an example under
[examples/invalid-by-rule](examples/invalid-by-rule) exists only for those.

| What | Promises | Why a schema cannot | Tool |
| --- | --- | --- | --- |
| The rule for a key of a named map: the name of an input, a step, a declared Conduit, an entry, a runtime, a requirement or a component; of a runtime or a member of `placement` in a Type; of a member of `placement` in an implementation item; and of an input or a constituent of a Requirement; that an Operation's input or output name is not empty; that a key of `implementation`, `controls` or `tools` is a selection, which is a qualified reference or `#name`; that a key of `packages` is an address | OBJ-022, CTR-002, TPL-003, TPL-010, PKG-020, MAN-010, NAM-001, NAM-004, NAM-018 | The profile has no keyword that constrains the keys of a map. `propertyNames` and `patternProperties` are outside it | — |
| The syntax of the file: duplicate keys, anchors, aliases, merge keys, tags, keys that are not strings; numbers with no JSON form; whole numbers out of range; other numbers outside the range of binary64; one document or one array in a JSON file | SCH-012, SCH-017, SCH-006, SCH-021, SCH-018 | They are gone, or cannot be seen, once the file has been read into values | When it reads a file, in step 1, except SCH-018 |
| A standard argument written under `arguments`, and anything else inside an open place | OBJ-013, INS-003 | The place is open by OBJ-019 | — |
| That a reference resolves, and to the right thing: a template or an instance, a kind, a contract of the expected kind, an export; that a `$ref` written as a qualified reference or as `#name` names a Schema contract | OBJ-007 to OBJ-009, INS-002, TPL-002, TPL-005, CON-017, CON-021, CON-027, NAM-003, NAM-007, NAM-022, NAM-028, PKG-018, POOL-014, SCH-015 | It needs the other documents, the objects that exist and the installed packages | Only a selection, a `$ref`, or a `from` written as a qualified reference or as `#name`, that names something in this repository, for its kind, or that names nothing there; SCH-015 and NAM-028 have an example each |
| That a document, or a request that an actor makes on its own behalf, does not write the resolved form of a reference, `{kind, id}` | OBJ-009, LIF-031 | Every reference schema accepts the form, because a stored instance holds it and the same schema governs what is stored | — |
| That no two documents of one project or package share a name, whatever their kinds, and that no implementation has the name of a document | NAM-027 | It needs the other documents, and the manifest | Yes for two documents. For an implementation, in step 5, for the core package and the valid examples |
| That a `default` satisfies the schema that states it, and an input's `default` the input's own `schema`. A default that is a map is checked after the defaults of its own properties are filled in | SCH-019, SCH-010 | A schema of a schema sees the keyword and not whether its value fits. `schema.v1` describes a schema and does not apply it | Yes, where jsonschema is importable and every `$ref` of the schema can be followed inside this repository |
| That a value reference in a constituent's `arguments` names an input the composition declares | CTR-010 | It relates values in different parts of a document. That the source is `input` is in the schema | Yes |
| Other uniqueness across documents, and the length of a name that is put together from several: a tool name, and the name of a component Conduit | OBJ-002, SUB-004, PKG-012, CTR-007, IFC-014, CON-029, MAN-002 | It needs the other documents, or the objects that exist. One case is in a schema: `manifest.v1` gives `exports` `uniqueItems`, so a name that `exports` writes twice is rejected (PKG-012) | — |
| Cycles: among the steps of a graph, among the graphs of one template that invoke one another's operations, among entries, components, constituents and re-exports | GRA-005, POOL-011, CON-019, CTR-005, PKG-014 | It needs the other documents, or relates values in different parts of one | GRA-005 only, for a graph written in place in a template's `implementation` map, or in its type's where the tool has the type's contract, that invokes its own operation |
| What an argument must satisfy, and what the resolved instance holds | INS-003 to INS-009, INS-013, INS-014, INS-019, INS-023, NAM-019, SCH-009, SCH-010 | It needs the template or the requirement, and its schemas | — |
| That two destinations do not overlap, and that a destination's parent is present | INS-010, INS-011 | It relates values in different parts of a document. Which places a destination may reach is in the schemas | — |
| That a Conduit template that is not a composite and writes no `operations` has an input whose destination is `/operations` | CON-016 | The profile cannot say that some member of a map holds a value. The schema does reject such a template that declares no `inputs` at all | Yes |
| That a Conduit template applies `bound.v1` at most once, which is checked when the template is registered, and that a Conduit that applies it has `access` as its target | CON-028 | The profile cannot count the members of a map that hold one value, and the target is in another document | Yes: a template that applies it twice, and a target written in place beside a template the tool has |
| That `bound.v1` is never a constituent of a composition, directly or through other compositions | CTR-011 | A constituent's `requirement` is a selection like any other, and a composition that reaches it through another needs that other's contract | Yes, as far as the tool has the contracts of the constituents |
| That no Operation contract outside the core package is marked `reservedToRoot: true`, and that no capability outside it lists an operation that is | CTR-009, CTR-003 | A schema sees one document and not the package it belongs to | CTR-009 only |
| That a Type of the family Pool offers the core package's capability `pool-management.v1` | CTR-004 | The profile has no keyword that says a list holds a value, and none that makes one key depend on another | Yes, where `offers` names the capability in the core package |
| That a template declares no runtime under a name its type declares, and that a step's `runtime` names a runtime of the template or of its type | TPL-006, GRA-017 | The type's contract is another document | TPL-006 only, where the tool has the type's contract |
| That a location's document states no `placement` | PLC-008 | Only the type of the template says that an instance is a location | Yes, where the tool has the template |
| That `{entry: name}` is written only in an entry of a Pool template | POOL-005 | A reference schema governs an entry's argument and every other place alike | Only in the `placement`, `consumer` and `target` of an instance document and the `target` of a declared Conduit |
| The meaning of a graph: that what a step names exists, that exactly one Conduit that a step's `conduit` names lists the step's operation, that no step names the start or the stop operation of a runtime of its own instance, required inputs, literals against their schemas, declared outputs, which source is allowed in which graph, that no value reference reads an output of a step that names a runtime, handles, model work | GRA-004, GRA-006 to GRA-008, GRA-010, GRA-011, GRA-014, GRA-020, GRA-021, GRA-022, GRA-024, GRA-028, TPL-006, TPL-016, IMP-004, IMP-005, IMP-009 | It needs the operations' contracts, the templates of the declared Conduits, and knowledge of which graph a step is in | GRA-028, GRA-024 and TPL-016 only, for the graphs a template writes, where the tool has the Conduit templates or the type's contract |
| Rules that tie two keys of one map together beyond "exactly one of": a requirement on placement states `schema` and no `inputs`; `tools` has a key for each operation in `implements`; a `graph` item implements exactly one operation, and, which the schema does express, is not marked `plain: true`, gives no `tools` and states no `supports`; each key of a control's `changes` is an input the control declares and each pointer a place its type's schema describes, and a control declares the input `revision` and the output `settings` | CTR-005, PKG-020, GRA-014, CTR-006 | The profile has no `if`, `not` or dependent keyword, and none that constrains the keys of a map | CTR-006 only, in step 5, for a control whose contract is in this repository |
| That a requirement is applied only where its `appliesTo` lists, and a composition only where the `appliesTo` of every constituent lists too | CTR-005 | It needs the requirement's contract and those of its constituents | Yes, for the `requirements` of a Conduit template and the `placement` of a Type, where the tool has the contracts |
| That a requirement that is not a composition and does not narrow requests is applied to a Conduit, itself or as a constituent of a composition, only where it is a contract of the core package | CON-030 | It needs the requirement's contract and those of its constituents | Yes, for the `requirements` of a Conduit template, where the tool has the contracts |
| That a requirement that does not narrow requests states no `obliges` | CTR-005 | The profile has no keyword that makes one key depend on another, and whether a composition narrows is in the contracts of its constituents | Yes, where the tool can tell that the requirement does not narrow |
| That every operation which a template's type requires, which a capability the type offers includes, which the type lists as a control, or which a runtime or a step without `conduit` names has an implementation: one the template selects, or a default of the type or of the operation's contract | TPL-004 | It needs the type's contract, the capabilities' and the operations' | Yes, where the tool has the type's contract, for each operation whose contract it has |
| That the implementations the target has selected, and those selected for the consumer where `obliges` names `consumer`, support a filter: by stating it in `supports`, which for a composition means the composition or every one of its constituents, or, for a graph, through the implementations of its steps | CON-022, CTR-005, PKG-019 | It needs the target's type, the implementations selected and their manifests | — |
| That no instance document is made from the core package's templates `system`, `management`, `access` or `root` | FST-006 | Only resolving `from` says which template it names. The documents of [examples/valid/first-start](examples/valid/first-start) are valid against the schemas for that reason, and a submission that held them is rejected | — |
| Placement, ownership and state | POOL-001 (the owner is a Pool instance), PLC-001 to PLC-007, PLC-018, CON-006, CON-022, CON-025 | They need the objects that exist | — |
| That the core package is not listed in a manifest, and that a manifest that is handed over for registration does not state its address as its own | NAM-021 | The profile has no `not`, and whether a manifest was handed over is not in the manifest. `manifest.v1` keeps a form for the core package's own manifest, which that address selects | Yes, for the manifests of the valid examples |
| That a manifest handed to `replace.v1` holds `packages` alone, beside its `apiVersion` | SUB-009 | The input `manifest` of `replace.v1` is declared with `manifest.v1`, which describes every manifest. Which operation a manifest was handed to is not in the manifest | — |
| That `fetch` is a URI reference | MAN-006 | The profile has no format for a URI reference: `uri` requires a scheme, and a relative reference has none. `manifest.v1` and `install.v1` accept any string that is not empty | — |
| That a relative `fetch`, or a program given as a relative path, has a directory to be resolved against | MAN-006, MAN-010 | Whether a manifest was read from a directory or handed over as a value is not in the manifest | — |
| That the address a package's manifest states is the one it was listed or fetched under | PKG-017 | It needs the manifest that lists the package, or the request that installs it | — |
| That a document does not use one version's `apiVersion` with another version's core address | OBJ-024 | The schemas are those of one version, and state its names alone | — |
| That a `pattern` is a regular expression of ECMA-262 | SCH-016 | Not expressible | — |
| Everything a package's archive must be, which files of a directory are read, and that a package holds no instance document | PKG-004, PKG-021, PKG-022, MAN-001, MAN-002, MAN-011, MAN-013, MAN-014 | It is about files, not a document: a schema sees one document, and not whether it lies in a package or in a submitted project | — |

A schema therefore answers one question: is this document, taken alone, one this specification
could accept. It never says a document is accepted.

## Choices the schemas encode

Where a promise left the shape open, the schema took the smallest reading, and the
[record](../../docs/proposals/core-and-schemas/README.md#b-choices-that-no-promise-states)
lists each. The owner accepted them all, and most are now stated by a promise: `metadata` and
`arguments` may be left out (OBJ-013, OBJ-014); a template narrows a control's input as
`inputs: {<name>: {schema: ...}}` (TPL-010); a schema may be a boolean; a Requirement's input
holds `schema` and may hold `default` and `optional` (CTR-005); and an implementation item
that is not marked `plain: true` holds no `tools` (PKG-020). One more choice of the schemas
was not in that list and is now stated by a promise, on a decision of the third round of the
re-audit that the owner has not ruled on: an implementation item that states `graph` is not
marked `plain: true` and gives no `tools` (PKG-020). The follow-up to that round changed one
shape, on a decision of the same standing: an item that states `graph` no longer states
`supports` (PKG-019). The
[decisions of the fourth round](../../docs/proposals/reaudit/round-4/DECISIONS.md), which
have the same standing, changed one more shape: the `conduit` of a step is the name of a
declared Conduit or that name and the names of components, joined by dots, where it was a
name alone (GRA-024). These remain choices of the
schemas alone: `optional` is a boolean, so `optional: false` is accepted; and a Type's
`family` is one of the four families that have a type. No schema requires `optional`
beside `default`: an input that states a `default` may be left out, whether or not it is
marked optional (OBJ-015).

The decisions of the [docs audit](../../docs/proposals/docs-audit/GAPS.md) added four things,
each stated by a promise:

| What | Schema | Promise |
| --- | --- | --- |
| A Type may hold `restoresRuntimes`, and `records` with `inputs` and `outputs`. Each is a boolean, and one that is left out is false | `type-spec.v1` | CTR-004 |
| An Operation's input that holds a `schema` may hold `default`. An output, and an input that is a handle, may not | `operation-spec.v1` | CTR-002, OPS-026 |
| A Requirement may hold `obliges`, a list drawn from `consumer` and `target` | `requirement-spec.v1` | CTR-005 |
| A constituent of a composed Requirement holds `requirement` and may hold `arguments`, which are value bindings whose only source is `input` | `requirement-spec.v1` | CTR-010 |

The [decisions of the second re-audit round](../../docs/proposals/reaudit/round-2/DECISIONS.md)
changed seven shapes:

| What | Schema | Decision |
| --- | --- | --- |
| A control's `changes` is a map from the name of an input to the JSON Pointer of the setting that input sets. It was a list of pointers | `operation-spec.v1` | R2-G2 |
| Every reference schema also accepts the resolved form `{kind, id}`, which only the engine writes | The 11 reference schemas of [spec/core](../core/references.yaml) | R2-G1 |
| The value of `from` has four forms, and its `#name` form is its own and no longer the form of a selection | `template-reference.v1` | R2-S1 |
| A path segment of an address starts with a letter or a digit | `address.v1`, `qualified-reference.v1` | R2-S9 |
| An input of a composite Conduit template has the `arguments` of a component as its destination, and never `operations` | `conduit-template-spec.v1` | Finding S5 of the second reader, now INS-012 |
| A runtime holds `start` and `stop` and no `optional` | `runtime.v1` | The fix pass, and what it left; TPL-006 |
| An Operation no longer holds `arrangesLaterWork`, which round 2 had added: later work is not part of this version | `operation-spec.v1` | [Ruling 38](../../docs/proposals/ruling-38/README.md), point 6 |

The [decisions of the third re-audit round](../../docs/proposals/reaudit/round-3/DECISIONS.md)
changed no shape of a document. Two core operations gained an optional input, which
`operation-spec.v1` already allowed: `reconcile.v1` the handle `handle` (TPL-015), and
`apply-policy.v1` `version` (ACC-031). The decisions on what that pass left changed one shape
of the manifest:

| What | Schema | Decision |
| --- | --- | --- |
| An implementation item that states `graph` states no `supports`. No graph states it, wherever it is written | `implementation-item.v1` | Point 4 of "The fix pass, and what it left"; PKG-019, CON-022 |

## Examples

| Path | What |
| --- | --- |
| [examples/valid/project](examples/valid/project) | A small submitted project: a manifest; a Schema contract of its own, which a template reaches with a `$ref` written `#name`; a Resource template with an input, a runtime of its own beside the one its type declares, a setup graph whose steps name both, and a narrowed control; a Pool template with two entries, one referring to the other; a Conduit template with two requirements; a composite Conduit template, one of whose components writes its `from` as `#name`; an Agent template with a required and an optional declared Conduit; an Interface template, which writes no `config`; a Conduit template that applies the bound of a right to open; and instance documents of every family, a Conduit to `access` among them |
| [examples/valid/files-package](examples/valid/files-package) | A package: a manifest with `address`, `version`, `exports`, a re-export and three implementations, and one contract of each of the five kinds, two of which use a `$ref` written `#name`. Among them are an Operation whose input states a `default`, a control whose `changes` maps its input to a setting, a Type that declares a runtime and states `restoresRuntimes` and `records`, a Requirement that states `obliges`, and a composed Requirement |
| [examples/valid/first-start](examples/valid/first-start) | The instance documents of the five first-start objects, written out to show what first start makes. Each is valid against the schemas. The engine makes these objects itself and is handed no such document: submitting them is rejected, under FST-006 for the four made from the templates `system`, `management`, `access` and `root`, and under OBJ-002 because the five names are taken |
| [examples/invalid](examples/invalid) | 63 files, each holding one document that a schema rejects, or that is rejected when its file is read, and named for the promise it breaks. A file with `-manifest-` in its name is a manifest; the others are documents |
| [examples/invalid-by-rule](examples/invalid-by-rule) | 24 files, each holding one set of one or more documents and named for the promise the set breaks. Every document in them is valid against the schemas. One file has `-manifest-` in its name and holds a manifest, which is valid against `manifest.v1`. What rejects the set is a rule no schema can express, which the tool applies itself and which the table above marks |

Every name under `example.org` is invented, and every digest in an example is illustrative.
Each directory under `examples/valid` is one project, with its documents beside its manifest
where it has one, as MAN-011 states. A set under `examples/invalid-by-rule` shows one rule and is not a whole
project: it may name objects and packages that are not written out.

## Checking

```sh
python3 tools/check_schemas.py
```

It reads every file as strict YAML, checks every schema against the profile, validates every
document here, in `spec/core` and in `examples/valid`, and checks that every document in
`examples/invalid` is rejected by a schema, or when its file is read. It checks that the core package agrees with
itself, that each control's `changes` maps inputs the control declares to places its type's
schema describes, and that the tables of this page and of the core package's page list what
there is.
It then applies the few rules the table above marks, to the core package and the valid
examples, which must break none, and to each set in `examples/invalid-by-rule`, which the
rule its file is named for must reject. One rule is on manifests, for NAM-021, and is applied
to the manifests of the valid examples and to the one manifest in `examples/invalid-by-rule`.
One of those rules, for SCH-019, validates each
`default` against its schema, and is applied only where jsonschema is importable; where it is
not, the report says that the rule was not applied. Its report says of each rejection whether a
schema or one of its own rules made it. It does not do anything else the table leaves to an
engine.
