# The core package

Status: **proposed, not accepted.** Written for decision G1 of the
[re-audit](../../docs/proposals/reaudit/README.md) and awaiting the owner's review as one batch
with [the schemas](../schemas/README.md) and [the manifest's promises](../promises/manifest.yaml).
The record of the batch, with every choice made here and its alternative, is
[docs/proposals/core-and-schemas](../../docs/proposals/core-and-schemas/README.md). Nothing in
this directory binds until it is accepted.

## What this is

The core package is the package built into every engine, at the address
`github.com/shrav-k/conduit/core` (NAM-020). A document refers to it without a manifest item, and
it cannot be installed, listed or replaced (NAM-021). This directory holds it as documents:
everything the promises say it exports, and everything first start needs.

| File | Holds |
| --- | --- |
| [conduit.yaml](conduit.yaml) | The package's manifest: its address, its version, its exports and its one implementation, the engine itself |
| [operations.yaml](operations.yaml) | The 19 Operation contracts: every management verb, the tool that retrieves one's own results, and the control that designates the default location |
| [capabilities.yaml](capabilities.yaml) | The 4 Capability contracts: what `management`, `access`, a Pool and the `system` Pool offer |
| [types.yaml](types.yaml) | The 6 Type contracts: the plain Interface and Pool types, and the types of `system`, of a location, of `management` and of `access` |
| [requirements.yaml](requirements.yaml) | The 2 Requirement contracts: the bound of a right to open, and the shared-host requirement |
| [references.yaml](references.yaml) | The 11 reference schemas: the Schema contracts that make a value a reference (OBJ-021) |
| [results.yaml](results.yaml) | 2 Schema contracts shared by the outputs of the operations |
| [templates.yaml](templates.yaml) | The 7 templates of the first-start objects |

The package also exports the 45 Schema contracts under [spec/schemas](../schemas/README.md),
which are the schemas of documents themselves. They are part of this package because a `$ref`
from one schema to another must be a qualified reference to a Schema contract that a package
exports (SCH-015), and because `register.v1` states its inputs with them.

Every document here validates against those schemas, and `python3 tools/check_schemas.py`
checks it.

## Names

Export names follow NAM-004, and each becomes a tool's name under IFC-012, so they are short.
The convention, which follows the names the promises already use (`local-directory.v1`,
`shared-host.v1`, `location`):

| Kind | Named as | Example |
| --- | --- | --- |
| Operation | The verb, in the imperative, as the commands of `kubectl` and `docker` are named. Words are joined by hyphens | `register.v1`, `set-up.v1` |
| Type, Capability, Requirement, Schema | A noun, with hyphens | `system-pool.v1`, `access-control.v1`, `bound.v1`, `resource-reference.v1` |
| Template | The name of the first-start object made from it, except `location`, which NAM-020 names | `system`, `root` |
| Every contract | Ends in its compatibility version, written `v` and a whole number (CTR-001) | `.v1` |

No two exports share a name, whatever their kinds (PKG-012). That is why the Type of the
`management` Resource is `management.v1` and its capability is `object-management.v1`.

## The operations

One contract stands for one verb. Where a verb applies both to a Resource and to other
instances, the same contract is included in the capability of a Pool and in the capability of
`management`, and ACC-033 says which target authorizes a request from the object it names. A
request through a Conduit to the other target reports refused.

| Operation | Included in | `revision` input | `deduplicates` | Reserved to `root` | Promise |
| --- | --- | --- | --- | --- | --- |
| `register.v1` | `object-management.v1`, `pool-management.v1` | — | yes | — | SUB-006, ACC-033 |
| `replace.v1` | `object-management.v1` | required | yes | — | SUB-007 |
| `rename.v1` | `object-management.v1` | required | yes | — | OBJ-011 |
| `unregister.v1` | `object-management.v1` | required | yes | — | LIF-017 |
| `inspect.v1` | `object-management.v1`, `pool-management.v1` | — | — | — | LIF-013 |
| `set-up.v1` | `object-management.v1`, `pool-management.v1` | — | no | — | LIF-003, POOL-013 |
| `retire.v1` | `object-management.v1`, `pool-management.v1` | — | no | — | LIF-005, TPL-009 |
| `reconcile.v1` | `object-management.v1`, `pool-management.v1` | for a control | yes | — | OPS-014, REV-010 |
| `register-replacement.v1` | `object-management.v1`, `pool-management.v1` | — | yes | — | LIF-009, POOL-003 |
| `install.v1` | `object-management.v1` | — | yes | — | ACC-033, OPS-019 |
| `declare-lost.v1` | `object-management.v1` | — | yes | yes | PLC-017, ACC-032 |
| `purge.v1` | `object-management.v1` | — | yes | yes | REC-004 |
| `retire-with-resources.v1` | `pool-management.v1` | — | no | — | LIF-021 |
| `open.v1` | `access-control.v1` | — | yes | — | ACC-006 |
| `close.v1` | `access-control.v1` | — | yes | — | ACC-012 |
| `apply-policy.v1` | `access-control.v1` | — | yes | — | ACC-031 |
| `remove-policy.v1` | `access-control.v1` | — | yes | — | ACC-031 |
| `retrieve-result.v1` | No capability: presented to every consumer | — | — | — | IFC-024, OPS-016 |
| `set-default-location.v1` | `system-settings.v1` | required | yes | — | PLC-016 |

How to read the columns:

- **`revision` input.** A request's `arguments` are exactly the operation's inputs (IFC-001), so
  an operation that changes a stored object takes the object's current revision as an input.
  One that names a revision that is not current reports conflict (REV-002), and one that names
  none reports invalid (REV-011). Setting up, retiring, opening and closing name no revision:
  they change an instance's lifecycle state, not its definition or configuration.
- **`deduplicates`.** Declared where the whole effect is a change to what the engine itself
  stores, which the engine recognizes by the operation identity, so a repeat after an unknown
  outcome can be dispatched as a new attempt (OPS-012). Not declared where the operation runs
  the graphs of other implementations. `inspect.v1` and `retrieve-result.v1` state no `effect`.
- **Reserved to `root`.** Written `reservedToRoot: true` in the contract. No promise names
  this key; it is a proposal. ACC-032 and CTR-003 say what the mark means.
- No core operation starts model work (IMP-002), so none carries `startsModelWork`.
- Every core operation states `github.com/shrav-k/conduit/core#engine` as its default
  implementation. The engine performs it; no process is started and no request crosses the
  implementation interface.

The tool that IFC-024 always presents is named `retrieve-result.v1`, with no Conduit's name
before it.

## First start

At first start (FST-001, FST-002) the engine instantiates five documents, which are written out
as an example in
[spec/schemas/examples/valid/first-start](../schemas/examples/valid/first-start/first-start.yaml):

| Object | Kind | `from` | Notes |
| --- | --- | --- | --- |
| `system` | Pool | `github.com/shrav-k/conduit/core#system` | Its default-location setting names nothing |
| `local` | Resource | `github.com/shrav-k/conduit/core#location` | Owned by `system`; its own placement (PLC-008); its facts are the engine's to supply |
| `management` | Resource | `github.com/shrav-k/conduit/core#management` | Owned by `system`, placed on `local` |
| `access` | Resource | `github.com/shrav-k/conduit/core#access` | Owned by `system`, placed on `local` |
| `root` | Interface | `github.com/shrav-k/conduit/core#root` | Declares two Conduits, which are instantiated with it |

The two Conduits of FST-002 are the declared Conduits of the template `root`. Their local names
are `management` and `access`, so the root Interface's tools are `management.register.v1`,
`access.open.v1` and so on. `root-access` applies no bound, which is what ACC-022 requires.
`management` and `access` are not entries of the `system` Pool's template: an entry's Resource
gets a generated name (OBJ-003), and FST-001 fixes these two names.

A location's stored facts (PLC-007, PLC-008, PLC-013) are the value of `facts` in its
configuration. The two facts the definitions name are `host` and `localPaths`; a location may
state others.

## Every export

"Requires" names the promise that needs the export to exist. Where it says "none", the export
is a choice of this batch and the record lists it.

| Export | Kind | Requires |
| --- | --- | --- |
| `system` | Pool template | FST-001, FST-005 |
| `location` | Resource template | FST-001, FST-005, PLC-008, NAM-020 |
| `management` | Resource template | FST-001, FST-005 |
| `access` | Resource template | FST-001, FST-005 |
| `root` | Interface template | FST-001, FST-002, FST-005 |
| `root-management` | Conduit template | FST-002, FST-005, ACC-032 |
| `root-access` | Conduit template | FST-002, FST-005, ACC-022 |
| `interface.v1` | Type | TPL-002: the template `root` needs a type; the plain Interface type |
| `pool.v1` | Type | TPL-002: a Pool template needs a type; the plain Pool type |
| `system-pool.v1` | Type | TPL-002, PLC-003, PLC-016 |
| `location.v1` | Type | PLC-008 |
| `management.v1` | Type | TPL-002, ACC-033 |
| `access.v1` | Type | TPL-002, ACC-006, ACC-031 |
| `object-management.v1` | Capability | OPS-019, ACC-033 |
| `access-control.v1` | Capability | OPS-019, ACC-006, ACC-012, ACC-031 |
| `pool-management.v1` | Capability | OPS-019, ACC-033, ACC-028 |
| `system-settings.v1` | Capability | PLC-016, CON-017 |
| `register.v1` | Operation | OPS-019, ACC-033, SUB-006 |
| `replace.v1` | Operation | SUB-007 |
| `rename.v1` | Operation | OBJ-011 |
| `unregister.v1` | Operation | LIF-017 |
| `inspect.v1` | Operation | LIF-013, ACC-033 |
| `set-up.v1` | Operation | OPS-019, ACC-033, LIF-003 |
| `retire.v1` | Operation | OPS-019, ACC-033, LIF-005 |
| `reconcile.v1` | Operation | OPS-014, REV-010 |
| `register-replacement.v1` | Operation | LIF-009, POOL-003 |
| `install.v1` | Operation | OPS-019, ACC-033 |
| `declare-lost.v1` | Operation | PLC-017, ACC-032 |
| `purge.v1` | Operation | REC-004 |
| `retire-with-resources.v1` | Operation | LIF-021 |
| `open.v1` | Operation | ACC-006 |
| `close.v1` | Operation | ACC-012 |
| `apply-policy.v1` | Operation | ACC-031 |
| `remove-policy.v1` | Operation | ACC-031 |
| `retrieve-result.v1` | Operation | IFC-024, OPS-016 |
| `set-default-location.v1` | Operation, a control | PLC-016 |
| `bound.v1` | Requirement | ACC-029, ACC-030 |
| `shared-host.v1` | Requirement | PLC-013, CON-006 |
| `reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006 |
| `interface-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006 |
| `agent-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006 |
| `resource-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006, POOL-005, POOL-014 |
| `pool-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006, POOL-001 |
| `conduit-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006 |
| `actor-reference.v1` | Schema, a reference schema | OBJ-021, ACC-031 |
| `consumer-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-013, CON-027 |
| `target-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-013, CON-027 |
| `entry-reference.v1` | Schema, a reference schema | OBJ-021, POOL-005 |
| `pool-entry-reference.v1` | Schema, a reference schema | OBJ-021, POOL-014 |
| `object.v1` | Schema | none |
| `diagnostic.v1` | Schema | none |
| `name.v1` | Schema of a document's part | OBJ-001, OBJ-022 |
| `export-name.v1` | Schema of a document's part | NAM-004 |
| `contract-name.v1` | Schema of a document's part | CTR-001 |
| `address.v1` | Schema of a document's part | NAM-001 |
| `qualified-reference.v1` | Schema of a document's part | NAM-002 |
| `selection.v1` | Schema of a document's part | NAM-018, NAM-022 |
| `pointer.v1` | Schema of a document's part | OBJ-016 |
| `destination.v1` | Schema of a document's part | OBJ-016, INS-021 |
| `version.v1` | Schema of a document's part | PKG-001 |
| `digest.v1` | Schema of a document's part | PKG-004 |
| `id.v1` | Schema of a document's part | none |
| `revision.v1` | Schema of a document's part | none |
| `schema.v1` | Schema of a document's part | CTR-008, SCH-002 |
| `api-version.v1` | Schema of a document's part | OBJ-017 |
| `envelope.v1` | Schema of a document's part | OBJ-017, OBJ-020 |
| `template-metadata.v1` | Schema of a document's part | OBJ-018 |
| `instance-metadata.v1` | Schema of a document's part | OBJ-018, OBJ-003 |
| `contract-metadata.v1` | Schema of a document's part | OBJ-018, CTR-001 |
| `graph.v1` | Schema of a document's part | GRA-023, GRA-021 |
| `lifecycle-graph.v1` | Schema of a document's part | GRA-023 |
| `step.v1` | Schema of a document's part | GRA-017 |
| `value-binding.v1` | Schema of a document's part | GRA-019 |
| `value-reference.v1` | Schema of a document's part | GRA-020 |
| `template.v1` | Schema of a document | OBJ-012 |
| `template-spec.v1` | Schema of a document's part | TPL-001 |
| `pool-template-spec.v1` | Schema of a document's part | TPL-001 |
| `conduit-template-spec.v1` | Schema of a document's part | CON-016 |
| `template-input.v1` | Schema of a document's part | INS-021 |
| `implementation-map.v1` | Schema of a document's part | TPL-003 |
| `runtime.v1` | Schema of a document's part | TPL-006 |
| `template-controls.v1` | Schema of a document's part | TPL-010 |
| `instance.v1` | Schema of a document | OBJ-013 |
| `template-reference.v1` | Schema of a document's part | INS-002, FST-005 |
| `declared-conduit.v1` | Schema of a document's part | OBJ-014 |
| `pool-entry.v1` | Schema of a document's part | OBJ-014, POOL-012 |
| `conduit-component.v1` | Schema of a document's part | OBJ-014, CON-019 |
| `contract.v1` | Schema of a document | CTR-001 |
| `type-spec.v1` | Schema of a document's part | CTR-004 |
| `capability-spec.v1` | Schema of a document's part | CTR-003 |
| `operation-spec.v1` | Schema of a document's part | CTR-002, CTR-006 |
| `requirement-spec.v1` | Schema of a document's part | CTR-005 |
| `applied-requirement.v1` | Schema of a document's part | NAM-019 |
| `manifest.v1` | Schema of the manifest | NAM-006, PKG-017 to PKG-020, MAN-003 to MAN-012 |
| `implementation-item.v1` | Schema of the manifest's part | PKG-019, PKG-020, MAN-009, MAN-010 |
| `document.v1` | Schema of a document | OBJ-020, SUB-005 |
| `engine` | Implementation | TPL-004 (every core operation has an implementation), CON-022 (it states that it supports `bound.v1`) |

## What this package does not settle

- The full shape of an inspection result and of a diagnostic. `inspect.v1` fixes only the names
  of its outputs and the two values every status carries; `diagnostic.v1` fixes only `message`.
- A control that changes a location's facts. PLC-008 says they are changed only by a control,
  and no promise names one, so this batch writes none.
- How an ID is written in a request. Every operation here names an object by a reference.

Each is listed, with the promises concerned, in the
[record](../../docs/proposals/core-and-schemas/README.md).
