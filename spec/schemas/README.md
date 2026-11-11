# The schemas of documents

Status: draft specification. See [the specification's status](../README.md) for coverage and
pending rulings. The promises are the source of truth: where a schema and a promise differ,
the schema is wrong.

The decision records and their reasons are listed in
[Where the reasons are](../README.md#where-the-reasons-are).

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
`$ref` to point into it. `tools/check_schemas.py` does exactly that, keying each copy by the
contract's name, since it holds the core package's alone. The definition of a tool that the
engine presents is made in the same way, keyed by each contract's qualified reference and the
version of its package, so that no two packages' contracts share a key (IFC-033): it stands
alone, and every `$ref` keeps its target, so the tools of `register.v1` and `replace.v1` hold a
`$defs` at their roots. For a control, an input that the target's template narrows is given
`allOf` of the contract's schema and the template's (TPL-010, IFC-033).

## Files

| File | Schema contracts | What they describe |
| --- | --- | --- |
| [document.yaml](document.yaml) | `document.v1` | Any template, instance or contract document |
| [envelope.yaml](envelope.yaml) | `api-version.v1`, `envelope.v1`, `template-metadata.v1`, `instance-metadata.v1`, `contract-metadata.v1` | `apiVersion`, what every document carries, and `metadata` |
| [templates.yaml](templates.yaml) | `template.v1`, `template-spec.v1`, `pool-template-spec.v1`, `conduit-template-spec.v1`, `template-input.v1`, `implementation-map.v1`, `template-controls.v1` | A template document; the `spec` of an Interface, Agent or Resource template, of a Pool template, and of a Conduit template, each with the closed list of places an input's destination may reach; an input; the `implementation` map; a template's `controls` |
| [instances.yaml](instances.yaml) | `instance.v1`, `template-reference.v1`, `declared-conduit.v1`, `pool-entry.v1` | An instance document of each family, with the standard argument `runtime` of an Agent and a Resource; the four forms of the value of `from`; the two nested instantiations |
| [contracts.yaml](contracts.yaml) | `contract.v1`, `type-spec.v1`, `capability-spec.v1`, `operation-spec.v1`, `requirement-spec.v1`, `applied-requirement.v1` | A contract document of each of the five kinds, and a requirement applied to a Conduit. `operation-spec.v1` carries `inputSchema` and `outputSchema`, with the root CTR-012 allows, the three hints of CTR-002, and a control's `changes`; `type-spec.v1` states that a Pool type offers `pool-management.v1` |
| [schema.yaml](schema.yaml) | `schema.v1` | A schema under the profile, with the three forms of a `$ref`. It is the `spec` of a Schema contract and the value of every `schema` key |
| [manifest.yaml](manifest.yaml) | `manifest.v1`, `implementation-item.v1`, `server-package.v1`, `server-remote.v1` | The manifest of a project and of a package; one implementation item; and the Package and the Remote of the Model Context Protocol Registry's server.json, schema 2025-12-11, the Remote limited to Streamable HTTP |
| [values.yaml](values.yaml) | `name.v1`, `export-name.v1`, `contract-name.v1`, `address.v1`, `qualified-reference.v1`, `selection.v1`, `pointer.v1`, `destination.v1`, `version.v1`, `digest.v1`, `id.v1`, `revision.v1`, `labels.v1` | The small values the others share: names, addresses, the forms of a reference to an export, pointers, versions, digests, an opaque revision and a label selector |

The schemas of a reference to an object are the core package's reference schemas, in
[spec/core/references.yaml](../core/references.yaml), because they are also what makes a value
inside `config` or `arguments` a reference (OBJ-021).

| A reference written as | Schema |
| --- | --- |
| A bare name, where one kind is accepted | `name.v1`, as a branch of `interface-reference.v1`, `agent-reference.v1`, `resource-reference.v1`, `pool-reference.v1`, `conduit-reference.v1` and `template-reference.v1` |
| `{kind: <Kind>, name: <name>}` | `reference.v1`; with fewer kinds, `actor-reference.v1`, `consumer-reference.v1`, `target-reference.v1` and `secret-reference.v1` |
| `{kind: Pool, name: <pool>, entry: <entry>}` | `pool-entry-reference.v1` |
| `{entry: <name>}` | `entry-reference.v1` |
| `{kind: <Kind>, id: <ID>}`, the resolved form, which only the engine writes | Every reference schema, as its last branch, with the kinds that schema accepts |
| A secret reference: a reference to a secret Resource, never a bare name and never the secret | `secret-reference.v1` (SEC-001) |
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
stores a value is checked against the same schema (REV-013, REV-015). Only the engine writes
the form. An implementation receives it in its `arguments`, in the filters it is sent and
in the configuration that every request to it carries (IFC-006), and `inspect.v1` shows it
under `declared`. A document never writes it, and nor does a request that an actor makes on
its own behalf: the engine rejects a submitted document that does (OBJ-009), and such a request
reports invalid (LIF-031). An onward request may hold it, because the engine handed it to the
instance that sends it. `set-up.v1`, `retire.v1`, `inspect.v1` and `unregister.v1` take the
optional input `expectedId`, an `id.v1` and not a reference: it names nothing, and where the
reference resolves to an object with another ID the request reports conflict (LIF-037).

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
| A file named `conduit.yaml`, `conduit.yml` or `conduit.json` | `manifest.v1` |
| Only that a document has `apiVersion` and one of the ten kinds | `envelope.v1` |
| A schema an author wrote | `schema.v1` |

## Closed and open

Every map these schemas describe is closed with `additionalProperties: false`, so a key this
specification does not define is rejected, as OBJ-019 requires. That is also how a field the
engine owns is rejected (OBJ-004), and a `required` key (OBJ-015), a `depends_on` (OBJ-019), a
`consumer` in a template (CON-020), an `owner`, a `placement` or a `runtime` in an entry
(OBJ-014), `runtimes`, `setup` or `retire` in a template (TPL-001), and `inputs` or `outputs` in
an Operation contract (CTR-002). The keys of a map that this specification says is keyed by
name are stated with `propertyNames`: `name.v1` for the members of a template's `inputs`,
`conduits`, `entries` and `requirements` and of a Requirement's `inputs` (OBJ-022), a
selection for the keys of an `implementation`, `controls` or `tools` map (NAM-018), an address
for the keys of `packages` and an export name for those of `implementations` (NAM-001,
NAM-004), and the label syntax for the keys of a label selector.

An input's destination is closed in the same spirit (INS-012). The schema of each kind of
template `spec` carries, as one pattern, the list of places a destination may reach:

| Template | A destination is |
| --- | --- |
| Interface, Agent or Resource | `/config` or a place inside it; `/conduits/<name>/target`; `/conduits/<name>/arguments` or a place inside it |
| Pool | `/config` or a place inside it; `/entries/<name>/placement`; `/entries/<name>/runtime`; `/entries/<name>/arguments` or a place inside it |
| Conduit | `/operations`; `/requirements/<name>/arguments` or a place inside it |

Any other destination is rejected by the schema: an item of `operations`, a whole nested
instantiation, a requirement's `requirement`, anything under `inputs`, and every key INS-012
names as fixed.

The schemas are open where OBJ-019 exempts:

| Place | How it is open |
| --- | --- |
| `config` | Any map. The type's schema checks it (INS-014) |
| `arguments` of an instance, of a nested instantiation and of an applied requirement | Any map. The inputs of the template or of the requirement check it. One that is left out is present and empty, as a `config` that is not written is (INS-011, INS-024) |
| `annotations` | Any map of strings |
| The value of the `default` of a template's input or of a Requirement's input | Any value. It is a literal value, which OBJ-019 exempts. The input's own schema checks it (SCH-019) |
| A schema, `inputSchema` and `outputSchema` among them | The names under `properties`, `dependentRequired`, `dependentSchemas` and `$defs`, the patterns under `patternProperties`, and the values of `const`, `enum`, `default` and `examples`, are the author's. The keywords are not: `schema.v1` rejects one outside the profile under SUB-005, as SCH-002 requires |
| An item's `package` and `remote` | As open as server.json's own schema is, which governs them (MAN-010), save that the `url` of the `remote` of an item that is not `plain` begins with `https://` (IFC-044) |
| The keys of an item's `environment`, `packageArguments` and `headers` | Any non-empty string: they are the names that the Package or the Remote declares, or for a `command` a variable's name. The schema rejects one key alone: `Authorization`, in any case, among the `headers` of an item that is not `plain` (IFC-043) |

`envelope.v1` is the one schema of a document that is not closed. It states only what all ten
kinds share, and is always used together with the closed schema of one kind. The helper schemas
under `$defs` that carry the pattern of a destination are open too, by design: each is
joined with `allOf` to the closed `template-input.v1`. The schemas of results, statuses and
records in [spec/core](../core/results.yaml), `object.v1`, `diagnostic.v1`, `condition.v1`
and `record.v1`, are open so that an engine may add members.

## What a schema cannot express

These promises are not checked by validating a document against a schema. The engine checks
them. "Tool" says whether `tools/check_schemas.py` applies the rule itself, as far as the
documents in front of it show, in its step 6; an example under
[examples/invalid-by-rule](examples/invalid-by-rule) exists only for those.

| What | Promises | Why a schema cannot | Tool |
| --- | --- | --- | --- |
| The syntax of the file: a file that is not UTF-8, or not YAML or JSON; duplicate keys, anchors, aliases, merge keys, tags, keys that are not strings; numbers with no JSON form; whole numbers out of range, and numbers read as whole binary64 values out of range; other numbers outside the range of binary64; one document or one array in a JSON file | SUB-005, SCH-012, SCH-017, SCH-006, SCH-021, SCH-018 | They are gone, or cannot be seen, once the file has been read into values | When it reads a file, in step 1, a JSON file as SCH-018 reads it |
| A standard argument written under `arguments`, and anything else inside an open place | OBJ-013, INS-003 | The place is open by OBJ-019 | — |
| That a reference resolves, and to the right thing: a template or an instance, a kind, a contract of the expected kind, an export; that a `$ref` written as a qualified reference or as `#name` names a Schema contract | OBJ-007 to OBJ-009, INS-002, TPL-002, TPL-005, CON-017, CON-021, CON-027, NAM-003, NAM-007, NAM-022, NAM-028, PKG-018, POOL-014, SCH-015 | It needs the other documents, the objects that exist and the installed packages | Only a selection, a `$ref`, or a `from` written as a qualified reference or as `#name`, that names something in this repository, for its kind, or that names nothing there; SCH-015 and NAM-028 have an example each |
| That a document, or a request that an actor makes on its own behalf, does not write the resolved form of a reference, `{kind, id}` | OBJ-009, LIF-031 | Every reference schema accepts the form, because a stored instance holds it and the same schema governs what is stored | — |
| That no two documents of one project or package share a name, whatever their kinds, and that no implementation has the name of a document | NAM-027 | It needs the other documents, and the manifest | Yes for two documents. For an implementation, in step 5, for the core package and the valid examples |
| That a `default` satisfies the schema that states it, and a template's or a Requirement's input's `default` the input's own `schema`. A default that is a map is checked after the defaults of its own properties are filled in | SCH-019, SCH-010 | A schema of a schema sees the keyword and not whether its value fits. `schema.v1` describes a schema and does not apply it | Yes, where jsonschema is importable and every `$ref` of the schema can be followed inside this repository |
| Other uniqueness across documents, and the length of a tool name | OBJ-002, SUB-004, PKG-012, CTR-007, IFC-014, MAN-002 | It needs the other documents, or the objects that exist. One case is in a schema: `manifest.v1` gives `exports` `uniqueItems`, so a name that `exports` writes twice is rejected under SUB-005, as PKG-012 requires | — |
| Cycles among entries | POOL-011 | It relates values in different parts of a document | — |
| What an argument must satisfy, and what the resolved instance holds | INS-003 to INS-009, INS-013, INS-014, INS-019, INS-023, NAM-019, SCH-009, SCH-010 | It needs the template or the requirement, and its schemas | — |
| That two destinations do not overlap, and that a destination's parent is present | INS-010, INS-011 | It relates values in different parts of a document. Which places a destination may reach is in the schemas | — |
| That a Conduit template that writes no `operations` has an input whose destination is `/operations` | CON-016 | The profile has no keyword that says that some member of a map holds a value. The schema does reject such a template that declares no `inputs` at all | Yes |
| That a Conduit template applies `bound.v1` at most once, which is checked when the template is registered, and that a Conduit that applies it has `access` as its target | CON-028 | The profile cannot count the members of a map that hold one value, and the target is in another document | Yes: a template that applies it twice, and a target written in place beside a template the tool has |
| That no Operation contract outside the core package is marked `reservedToRoot: true`, and that no capability outside it lists an operation that is | CTR-009, CTR-003 | A schema sees one document and not the package it belongs to | CTR-009 only |
| That each key of a control's `changes` is an input the control declares and each pointer a place its type's schema describes; that no two of its pointers are equal or lie one inside another, compared as reference tokens, the empty pointer lying over every other; that its `inputSchema` lists `revision` under `required` and that it declares no output `settings` | CTR-006 | It relates the control to its type, which lists it, and the values of `changes` to each other | In step 5, for a control whose contract is in this repository. The overlap of pointers in step 6, for every Operation contract that holds `changes`, with two examples |
| That `{entry: name}` is written only in an entry of a Pool template | POOL-005 | A reference schema governs an entry's argument and every other place alike | Only in the `placement`, `runtime`, `consumer` and `target` of an instance document and the `target` of a declared Conduit |
| That every operation which a template's type requires, which a capability the type offers includes, or which the type lists as a control has an implementation: one the template selects or a default of the type, unless the engine implements it, as it does each operation that the core package's `engine` lists; no Operation contract states a default (CTR-002) | TPL-004 | It needs the type's contract, the capabilities' and the operations' | Yes, where the tool has the type's contract, for each operation whose contract it has |
| That no key of a template's `implementation` map, nor of a Type's, is an operation that the core package's implementation `engine` lists | TPL-003 | It needs the core package's manifest | Yes, for the templates and Types that the tool reads |
| That the implementations a template selects include at most one item that is run, and none for an Interface or a Pool | TPL-019 | It needs the type's contract and the manifests of the implementations | Yes, for two items, where the tool has the type's contract and the manifests |
| That a selected item's `nodeSelector` names no label that its type's `nodeSelector`, or another selected item's, names with another value | PLC-025 | It needs the type's contract and the items' manifests | Yes, as far as the tool has them |
| That a selected item's `environment`, `packageArguments` and `headers` point at places that the type's schema requires, with every member on the way required; which it confines, by `type`, `enum` or `const`, in itself or in each branch of its `anyOf` or `oneOf`, to a string, a number, a boolean or null, unless `secret-reference.v1` governs the place; and, for an input that the Package or the Remote marks `isRequired: true`, no null | MAN-017 | It needs the type's schema and the item's manifest | Yes, following `properties` and `required`, and the branches of `anyOf` and `oneOf` at the place, and not a `$ref` |
| That a name in an item's `environment`, `packageArguments` or `headers` is one that its Package or its Remote declares | MAN-010 | The profile cannot relate the keys of one map to the values inside another | — |
| That no item that a template selects binds, in its `environment`, a variable that `identity` carries, compared without regard to case | MAN-018 | The item alone is not rejected: the rule is on the template that selects it | Yes, with the names that [spec/names.yaml](../names.yaml) fixes, for each item the tool has |
| That a header that carries a secret goes, and an item that a template selects for `get-secret.v1` stands, only at an `https` URL, or at an `http` URL whose host is `localhost`, an IPv4 address in `127.0.0.0/8` or the IPv6 address `::1`; and that on an engine that can restart on another machine or run as several parts, no `remote` item stands at such a host | SEC-024, DL17 | Which place holds a secret is in the type's schema, the URL in the manifest, and whether the engine can move in its conformance document | Yes, for a header that carries a secret at an `http` URL whose host is not a loopback address, finding the place as MAN-017's rule does |
| That the implementations the target has selected support a filter, by stating it in `supports` | CON-022, PKG-019 | It needs the target's type, the implementations selected and their manifests | — |
| That no instance document is made from the core package's templates `system`, `node`, `management`, `access` or `root` | FST-006 | Only resolving `from` says which template it names | Yes, for a `from` that is a qualified reference into the core package. Not for the documents of [examples/valid/first-start](examples/valid/first-start), which show what first start makes and are valid against the schemas for that reason |
| That `secret-reference.v1` governs a place only as its own schema, as `items`, or beside `{type: null}` under `anyOf` or `oneOf` | SEC-001 | The profile cannot say which schemas stand beside a `$ref` to a given contract | Yes, for every schema a document writes; a `$ref` to a definition that holds it is not followed |
| That no template but the core package's `node` has the type `node.v1` | PLC-028 | Only the template's name and its package say which template it is | Yes, for a `type` that is the core package's qualified reference |
| That a secret reference names a Resource whose type offers `secret-provider.v1` | SEC-002 | It needs the object the reference resolves to | — |
| That a Type that offers a provider capability is of the family Resource and offers no other | PRV-001 | The profile could state it with `if` and `contains`; `type-spec.v1` leaves the core package's provider capabilities to this tool's rule | Yes |
| That a Type that offers `secret-provider.v1` offers no other capability | SEC-022 | The profile could state it with `if`, `contains` and `maxItems`; `type-spec.v1` leaves the core package's capabilities to this tool's rule | Yes |
| That a plain implementation implements no operation of `runtime-provider.v1`, `machine-provider.v1` or `package-source.v1` | PKG-020 | The profile could state it with `if` and `contains`; `implementation-item.v1` leaves the core package's operations to this tool's rule | Yes, for the manifests of the valid examples and those in `examples/invalid-by-rule` |
| That an item reached by `remote` implements no operation of `runtime-provider.v1`, `machine-provider.v1` or `package-source.v1` | PKG-030 | As for PKG-020 | Yes, for the manifests of the valid examples and those in `examples/invalid-by-rule` |
| That a Type that offers `secret-provider.v1` lists no control, and that one that offers `package-source.v1` requires `schemes` and lists no control whose `changes` holds `/schemes`, a pointer inside it or the empty pointer | SEC-005, PRV-016 | The profile could state the first two with `if` and `contains`, and `type-spec.v1` does not yet; the third needs the contracts of the controls | For a control of a package-source provider's Type whose `changes` reaches `/schemes`, where the tool has the control's contract, compared as reference tokens |
| That no Conduit lists an operation of `runtime-provider.v1`, `machine-provider.v1` or `package-source.v1`, and that one that lists `get-secret.v1` has no Interface as its consumer | PRV-004, SEC-009 | Which capability is a provider capability is in the core package, and the consumer is in another document | PRV-004, for a Conduit template |
| That a runtime provider, the machine provider and a package-source provider are given no placement but `local` and no `runtime`, and that `runtime` names a runtime provider that is not retired | PRV-002, PRV-006 | It needs the template's type and the object that `runtime` resolves to | — |
| Placement, ownership and state | POOL-001 (the owner is a Pool instance), PLC-001, PLC-002, PLC-007, PLC-018, PRV-015, CON-006, CON-025 | They need the objects that exist, and the labels that the machine provider reported | — |
| That the core package is not listed in a manifest, and that a manifest that is handed over for registration does not state its address as its own | NAM-021 | Whether a manifest was handed over is not in the manifest. `manifest.v1` keeps a form for the core package's own manifest, which that address selects | Yes, for the manifests of the valid examples |
| That a submitted project that holds a contract or lists an implementation states `address`, and not that of an installed package | MAN-015, MAN-016 | Whether the project holds a contract is in its other documents, and what is installed is not in the manifest | MAN-015, for a manifest that lists an implementation and for the valid examples |
| That the template, the contracts and the manifest handed to `replace.v1` are resolved as one submitted project, and that the template has the kind and the name of the template it replaces | SUB-009, SUB-008 | It needs the template being replaced, and what is installed. The schemas do check that `contracts` holds contract documents alone | — |
| That a relative `fetch`, or a program given as a relative path, has a directory to be resolved against | MAN-006, MAN-010 | Whether a manifest was read from a directory or handed over as a value is not in the manifest. The schemas do check that `fetch` is a URI reference with no user information | — |
| That the address a package's manifest states is the one it was listed or fetched under | PKG-017 | It needs the manifest that lists the package, or the request that installs it | — |
| That a document does not use one version's `apiVersion` with another version's core address | OBJ-024 | The schemas are those of one version, and state its names alone | — |
| That a `pattern`, and a key of `patternProperties`, uses only SCH-016's interoperable tokens and is a regular expression of ECMA-262 in Unicode mode | SCH-016 | `schema.v1` can say that a pattern is a string, and no keyword of the profile says that a string is a regular expression | Yes: a small tokenizer checks the token list, then `re` compiles it after `ecma` translates the end anchor and character classes |
| Everything a package's archive must be, which files of a directory are read, and that a package holds no instance document | PKG-004, PKG-021, PKG-022, MAN-001, MAN-002, MAN-011, MAN-013, MAN-014 | It is about files, not a document: a schema sees one document, and not whether it lies in a package or in a submitted project | Only that a file whose name begins with a dot is not read, in the trees of examples it reads |

A schema therefore answers one question: is this document, taken alone, one this specification
could accept. It never says a document is accepted.

## Choices the schemas encode

The schemas encode the following shapes: `metadata` and `arguments` may be left out
(OBJ-013, OBJ-014); a template narrows a control's input as
`inputs: {<name>: {schema: ...}}` (TPL-010); a schema may be a boolean; a Requirement's input
holds `schema` and may hold `default` and `optional` (CTR-005); and an implementation item
that is not marked `plain: true` holds no `tools` (PKG-020). These remain choices of the
schemas alone: `optional` and `plain` are booleans, so `optional: false` and `plain: false` are
accepted; and a Type's `family` is one of the four families that have a type. No schema
requires `optional` beside `default`: an input that states a `default` may be left out, whether
or not it is marked optional (OBJ-015).

The table lists further shapes the schemas encode. Their reasons are in the
[decision records](../README.md#where-the-reasons-are).

| What | Schema | Promise |
| --- | --- | --- |
| An operation with no inputs, or with no outputs, states no `inputSchema` or no `outputSchema` | `operation-spec.v1` | CTR-002, IFC-033 |
| `server-package.v1` and `server-remote.v1` are server.json's Package and Remote with their descriptions, examples and defaults left out; the Remote's `url` is also a `uri` | `server-package.v1`, `server-remote.v1` | MAN-010 |
| A diagnostic's `ruleId` is a promise ID in form, and a logical location's `fullyQualifiedName` is a JSON Pointer | `diagnostic.v1` | IFC-025 |
| A record's `data` holds the fields of REC-002 under the names that IFC-006 gives Conduit's keys, with `consumer`, `target`, `outcome` and `reason` beside them, the invocation served in `serving`, as IFC-020 names its key, and the inputs and outputs that the type's `records` keeps in `inputs` and `outputs`; the record's time is the CloudEvent's `time` | `record.v1` | REC-002, IFC-020, CTR-004 |
| `nested` in a result gives the ID of the instance that holds the nested instantiation and the instantiation's name; an actor names that instance in `register-replacement.v1` by its reference, and never by that ID | `object.v1` | LIF-009, LIF-031 |
| A label's key and value have the syntax of Kubernetes's label keys and values | `labels.v1` | PLC-007, PRV-014 |

## Examples

| Path | What |
| --- | --- |
| [examples/valid/project](examples/valid/project) | A small submitted project, whose manifest states `address` because it holds contracts and lists implementations: a Schema contract of its own, which a template reaches with a `$ref` written `#name`; a Resource template with an input and a narrowed control, which selects a setup hook; a Pool template with two entries, one referring to the other, whose input supplies their placement, and whose items, a program inside a package, run on the engine's own runner; a Conduit template with the shared-host requirement and a filter; an Agent template with a required and an optional declared Conduit; an Interface template, which writes no `config`; a Conduit template that applies the bound of a right to open, naming an operation of the project by its address; a runtime provider's type and template; the type of a secret Resource, a secret Resource and the Conduit through which it is read; a type whose configuration holds a secret reference, with a plain `command` item whose `environment` binds it; and instance documents of every family, an Agent that names its `runtime` among them. `bom.json` begins with one byte order mark, which is ignored. Beside them lies `.notes.yaml`, whose name begins with a dot, so that it is not read (MAN-011), and which would be rejected if it were |
| [examples/valid/files-package](examples/valid/files-package) | A package: a manifest with `address`, `version`, `exports` and three implementations, reached by `command`, by a server.json `package` pinned by its `fileSha256` and by a plain `remote`, each binding a value of the configuration, the `command` and the `remote` with label selectors that name one label with different values; and one contract of each of the five kinds, two of which use a `$ref` written `#name`. Among them are Operation contracts in `inputSchema` form, one with a `default`, one with a hint, one with `oneOf` at the root of its `inputSchema` and two with a handle, a control whose `changes` maps its input to a setting, a Type with a label selector, the default implementations of the operations it needs, since no Operation contract states one, and `records`, and a Requirement that is a filter |
| [examples/valid/first-start](examples/valid/first-start) | The instance documents of the five first-start objects, written out to show what first start makes. Each is valid against the schemas. The engine makes these objects itself and is handed no such document: submitting them is rejected, under FST-006 for the five made from the templates `system`, `node`, `management`, `access` and `root`, and under OBJ-002 because the five names are taken |
| [examples/invalid](examples/invalid) | 62 files, each holding one document that a schema rejects, or that is rejected when its file is read, and named for the promise it breaks. A file with `-manifest-` in its name is a manifest; the others are documents. One is a JSON file, whose value is a string |
| [examples/invalid-by-rule](examples/invalid-by-rule) | 38 files, each holding one set of one or more documents and named for the promise the set breaks. Every document in them is valid against the schemas. Four files have `-manifest-` in their names and hold a manifest, which is valid against `manifest.v1`. Three others hold, beside their documents, the manifest of their project, a document with no `kind`, whose items their templates select. What rejects the set is a rule no schema can express, which the tool applies itself and which the table above marks |

Every name under `example.org` is invented, and every digest and hash in an example is
illustrative. Each directory under `examples/valid` is one project, with its documents beside
its manifest where it has one, as MAN-011 states; a file there whose name begins with a dot is
not one of them. A set under `examples/invalid-by-rule` shows one rule and is not a whole
project: it may name objects and packages that are not written out.

## Checking

```sh
python3 tools/check_schemas.py
```

It reads every file of documents as UTF-8 and as strict YAML or JSON, by the ending of its name,
and no file whose name begins with a dot. It checks every schema against the profile, validates
every document here, in `spec/core` and in `examples/valid`, and checks that every document in
`examples/invalid` is rejected by a schema, or when its file is read. It checks that the core
package agrees with itself, that each control's `changes` maps inputs the control declares to
places its type's schema describes and that its `inputSchema` requires `revision`, and that the
tables of this page and of the core package's page list what there is. It then applies the rules
the table above marks, to the core package and the valid examples, which must break none, and to
each set in `examples/invalid-by-rule`, which the rule its file is named for must reject. Four
rules are on manifests, for NAM-021, MAN-015, PKG-020 and PKG-030, and are applied to the
manifests of the valid examples and to the manifests in `examples/invalid-by-rule`. The rules
that follow a template's selections into an implementation item, for TPL-019, PLC-025, MAN-017,
MAN-018 and SEC-024, read the items from the manifests of the core package and of the valid
examples, and from the manifest that a set in `examples/invalid-by-rule` holds beside its
documents. One of
the rules, for SCH-019, validates each `default` against its schema, and is applied only where
jsonschema is importable; where it is not, the report says that the rule was not applied. Its
report says of each rejection whether a schema or one of its own rules made it. It also checks
that `run.v1` holds the names of `identity` that `spec/names.yaml` fixes, its files closed to
them. It does not do
anything else the table leaves to an engine.
