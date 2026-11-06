# The core package

Status: **accepted by the owner on 2026-11-05** (plan-audit ruling 37), as one batch with
[the schemas](../schemas/README.md) and [the manifest's promises](../promises/manifest.yaml).
It was written for decision G1 of the [re-audit](../../docs/proposals/reaudit/README.md). The
record of the batch, with every choice made here and its alternative, is
[docs/proposals/core-and-schemas](../../docs/proposals/core-and-schemas/README.md), and what
the owner changed is in
[APPLY.md](../../docs/proposals/core-and-schemas/APPLY.md). The decisions of the
[docs audit](../../docs/proposals/docs-audit/GAPS.md) settled the points that batch left open
and changed some descriptions here; they added no export. The promises are the source of
truth: where a document here and a promise differ, the document is wrong.

## What this is

The core package is the package built into every engine, at the address
`github.com/shrav-k/conduit/core` (NAM-020). A document refers to it without a manifest item, and
it cannot be installed, listed or replaced (NAM-021). This directory holds it as documents:
everything the promises say it exports, and everything first start needs.

| File | Holds |
| --- | --- |
| [conduit.yaml](conduit.yaml) | The package's manifest: its address, its version, its exports and its one implementation, the engine itself |
| [operations.yaml](operations.yaml) | The 20 Operation contracts: every management verb, the tool that retrieves one's own results, the control that designates the default location and the control that changes a location's facts |
| [capabilities.yaml](capabilities.yaml) | The 5 Capability contracts: what `management`, `access`, a Pool, the `system` Pool and a location offer |
| [types.yaml](types.yaml) | The 6 Type contracts: the plain Interface and Pool types, and the types of `system`, of a location, of `management` and of `access` |
| [requirements.yaml](requirements.yaml) | The 2 Requirement contracts: the bound of a right to open, and the shared-host requirement |
| [references.yaml](references.yaml) | The 11 reference schemas: the Schema contracts that make a value a reference (OBJ-021) |
| [results.yaml](results.yaml) | 2 Schema contracts shared by the outputs of the operations |
| [templates.yaml](templates.yaml) | The 7 templates of the first-start objects |

The package also exports the 45 Schema contracts under [spec/schemas](../schemas/README.md),
which are the schemas of documents themselves. They are part of this package because a `$ref`
from one schema to another names a Schema contract (SCH-015), and because `register.v1` states
its inputs with them. In all the package exports 98 names, 58 of them Schema contracts, and
has one implementation.

A document of the core package refers to another by its full address, as
`github.com/shrav-k/conduit/core#name`. That is the one exception to the rule that a document
refers to something in its own package as `#name` (NAM-022).

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
| `set-facts.v1` | `location-settings.v1` | required | yes | — | PLC-008 |

How to read the columns:

- **`revision` input.** A request's `arguments` are exactly the operation's inputs (IFC-001), so
  an operation that changes a stored object takes the object's current revision as an input.
  One that names a revision that is not current reports conflict (REV-002), and one that names
  none reports invalid (REV-011). Setting up, retiring, retiring a Pool together with its
  Resources, opening and closing name no revision and advance none: they change an instance's
  lifecycle state, not its definition or configuration (REV-012). Applying and removing a
  policy name none either, because a policy is not configuration (ACC-031).
- **Controls.** `set-default-location.v1` and `set-facts.v1` are controls: each states
  `changes`, declares the input `revision` and the output `settings` (CTR-006), is listed under
  `controls` by its type, and is included in a capability so that a Conduit to the instance can
  list it (OPS-020, CON-017). `revision` is the one input that must always be supplied. Every
  other input is optional, and one that is not supplied leaves its setting as stored (REV-006).
  The result states the settings it confirmed in the output `settings`: an object keyed by the
  pointers of the control's `changes`, with one member for each place confirmed, holding the
  value now stored there (REV-007). `set-facts.v1` returns `settings: {/facts: ...}`.
- **Defaults.** No input of a core operation states a `default` (CTR-002, OPS-026). An input
  that may be left out is marked `optional: true`.
- **`deduplicates`.** Declared where the whole effect is a change to what the engine itself
  stores, which the engine recognizes by the operation identity, so a repeat after an unknown
  outcome can be dispatched as a new attempt (OPS-012). Not declared where the operation runs
  the graphs of other implementations. `inspect.v1` and `retrieve-result.v1` state no `effect`.
- **Reserved to `root`.** Written `reservedToRoot: true` in the contract (CTR-002). ACC-032
  says what the mark means, and only an Operation contract of the core package carries it: any
  other package that holds one is rejected when it is installed, and a project that holds one
  when it is submitted (CTR-009).
- **How a request names things.** An object is named by a reference and never by an ID, and a
  retired instance's name still resolves until the instance is unregistered (LIF-007). An
  invocation and a policy are not objects: `reconcile.v1` names an invocation, and
  `remove-policy.v1` a policy, by the ID the engine gave it.
- **Diagnostics.** `register.v1`, `replace.v1` and `install.v1` declare the output
  `diagnostics`. A request for one of them that ends with a reason carries its diagnostics
  too, in the `structuredContent` of its result (OBJ-023, IFC-025). A rejected installation
  reports invalid, unavailable or conflict as PKG-023 to PKG-025 state.
- No core operation starts model work (IMP-002), so none carries `startsModelWork`.
- Every core operation states `github.com/shrav-k/conduit/core#engine` as its default
  implementation. The engine performs it; no process is started and no request crosses the
  implementation interface.

The tool that IFC-024 always presents is named `retrieve-result.v1`, with no Conduit's name
before it. A caller names its invocation by `operationIdentity` and `target` (OPS-016). Once
the target is unregistered it can no longer be named, so the result can no longer be retrieved
through the tool; the records of the invocation remain (OPS-016, LIF-019). The two words of
its result are also what a direct answer carries in its `_meta`:
`io.github.shrav-k.conduit/state` with the value `inProgress` for a repeat of an invocation
that has not ended (OPS-009), and `io.github.shrav-k.conduit/withheld` with the value true
where content is withheld (ACC-034). IFC-025 states both. The third key of a result is
`io.github.shrav-k.conduit/restriction`, which names the restriction that is why a request
ended with a reason: an implementation gives it under IFC-026, and the engine's own answer
carries it under the same key (IFC-025). No operation of the core package declares any of the
three as an output, because `_meta` is not part of an operation's outputs.

The keys of a request's `_meta` are those of IFC-006: `interfaceVersion`, `instance`,
`operation`, `invocation`, `caller` and `restrictions`, with `operationIdentity`, `conduit` and
`package` where they apply, and `serving` on an onward request (IFC-020), each under the prefix
`io.github.shrav-k.conduit/`. `io.github.shrav-k.conduit/package` is sent only where the
operation, or a requirement that the restrictions name, is a contract of an unpublished
package: such a contract has no qualified reference, so it is named `#name`, and the key
holds the unpublished package's digest (SUB-008).

`inspect.v1` returns `id`, `revision`, `packages`, `declared`, `status` and `records`
(LIF-013). The status of `access` lists the policies in force under `policies`, each with the
`id` that `apply-policy.v1` returned (ACC-031).

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
state others. The argument `facts` sets them when the location is instantiated. After that
they are changed only by the control `set-facts.v1`, which is in the capability
`location-settings.v1` that the location type offers, and so is requested through an open
Conduit to the location that lists it. A location's document states no placement (PLC-008).

## Every export

"Requires" names the promise that needs the export to exist. Where it says "none", the export
is a choice of the batch that the owner accepted, and the record lists it.

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
| `location.v1` | Type | PLC-008, CTR-006 |
| `management.v1` | Type | TPL-002, ACC-033 |
| `access.v1` | Type | TPL-002, ACC-006, ACC-031 |
| `object-management.v1` | Capability | OPS-019, ACC-033 |
| `access-control.v1` | Capability | OPS-019, ACC-006, ACC-012, ACC-031 |
| `pool-management.v1` | Capability | OPS-019, ACC-033, ACC-028 |
| `system-settings.v1` | Capability | PLC-016, CON-017 |
| `location-settings.v1` | Capability | PLC-008, CON-017 |
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
| `set-facts.v1` | Operation, a control | PLC-008 |
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
| `destination.v1` | Schema of a document's part | OBJ-016, INS-021, INS-012 |
| `version.v1` | Schema of a document's part | PKG-001 |
| `digest.v1` | Schema of a document's part | PKG-004 |
| `id.v1` | Schema of a document's part | none |
| `revision.v1` | Schema of a document's part | none |
| `schema.v1` | Schema of a document's part | CTR-008, SCH-002, SCH-008, SCH-015 |
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
| `template-spec.v1` | Schema of a document's part | TPL-001, INS-012 |
| `pool-template-spec.v1` | Schema of a document's part | TPL-001, INS-012 |
| `conduit-template-spec.v1` | Schema of a document's part | CON-016, INS-012 |
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
| `operation-spec.v1` | Schema of a document's part | CTR-002, CTR-006, CTR-009 |
| `requirement-spec.v1` | Schema of a document's part | CTR-005 |
| `applied-requirement.v1` | Schema of a document's part | NAM-019 |
| `manifest.v1` | Schema of the manifest | NAM-006, PKG-017 to PKG-020, MAN-003 to MAN-012 |
| `implementation-item.v1` | Schema of the manifest's part | PKG-019, PKG-020, MAN-009, MAN-010 |
| `document.v1` | Schema of a document | OBJ-020, SUB-005 |
| `engine` | Implementation | TPL-004 (every core operation has an implementation), CON-022 (it states that it supports `bound.v1`) |

## What this package does not settle

- The full shape of an inspection result and of a diagnostic. `inspect.v1` fixes only the names
  of its outputs, the two values every status carries, and that a policy listed in the status
  of `access` has an `id`; `diagnostic.v1` fixes only `message`. Where the result of inspecting
  a Conduit states, for each restriction, enforced or mediated (ACC-027), and for each
  requirement the implementation that applies it (ACC-038), is not fixed either.

It is listed, with the promises concerned, in the
[record](../../docs/proposals/core-and-schemas/README.md). The other point that page once
listed is settled: the result of an invocation whose target has been unregistered can no
longer be retrieved through the tool (OPS-016).

No contract of the core package states `restoresRuntimes`, `records` or `obliges`, and none is
a composition (CTR-004, CTR-005, CTR-010): no core type declares a runtime, and the engine
itself checks both core requirements.
