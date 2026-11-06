# The core package

Status: **accepted by the owner on 2026-11-05** (plan-audit ruling 37), as one batch with
[the schemas](../schemas/README.md) and [the manifest's promises](../promises/manifest.yaml).
It was written for decision G1 of the [re-audit](../../docs/proposals/reaudit/README.md). The
record of the batch, with every choice made here and its alternative, is
[docs/proposals/core-and-schemas](../../docs/proposals/core-and-schemas/README.md), and what
the owner changed is in
[APPLY.md](../../docs/proposals/core-and-schemas/APPLY.md). The decisions of the
[docs audit](../../docs/proposals/docs-audit/GAPS.md) settled the points that batch left open
and changed some descriptions here; they added no export. The
[decisions of the second re-audit round](../../docs/proposals/reaudit/round-2/DECISIONS.md)
changed contracts and descriptions here and added no export either; what changed is in the
[log of that pass](../../docs/proposals/reaudit/round-2/log-documents.md) and the
[log of its closing pass](../../docs/proposals/reaudit/round-2/log-closing.md). The
[last pass of that round](../../docs/proposals/reaudit/round-2/log-tail.md) added one export,
the operation `start-later-work.v1`. The promises are the source of truth: where a document
here and a promise differ, the document is wrong.

## What this is

The core package is the package built into every engine, at the address
`github.com/shrav-k/conduit/core` (NAM-020). A document refers to it without an item of
`packages` in a manifest, and it cannot be installed, listed or replaced (NAM-021). This
directory holds it as documents:
everything the promises say it exports, and everything first start needs.

| File | Holds |
| --- | --- |
| [conduit.yaml](conduit.yaml) | The package's manifest: its address, its version, its exports and its one implementation, the engine itself |
| [operations.yaml](operations.yaml) | The 21 Operation contracts: every management verb, the tool that retrieves one's own results, the tool that asks for a start of later work, the control that designates the default location and the control that changes a location's facts |
| [capabilities.yaml](capabilities.yaml) | The 5 Capability contracts: what `management`, `access`, a Pool, the `system` Pool and a location offer |
| [types.yaml](types.yaml) | The 6 Type contracts: the basic Interface and Pool types, and the types of `system`, of a location, of `management` and of `access` |
| [requirements.yaml](requirements.yaml) | The 2 Requirement contracts: the bound of a right to open, and the shared-host requirement |
| [references.yaml](references.yaml) | The 11 reference schemas: the Schema contracts that make a value a reference (OBJ-021). Each also accepts the resolved form `{kind, id}`, which only the engine writes |
| [results.yaml](results.yaml) | 2 Schema contracts shared by the outputs of the operations |
| [templates.yaml](templates.yaml) | The 7 templates of the first-start objects |

The package also exports the 45 Schema contracts under [spec/schemas](../schemas/README.md),
which are the schemas of documents themselves. They are part of this package because a `$ref`
from one schema to another names a Schema contract (SCH-015), and because `register.v1` states
its inputs with them. In all the package exports 99 documents, 58 of them Schema contracts,
and has one implementation, which is also an export (PKG-019).

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
| Template | The name of the first-start object made from it, except `location`, which NAM-020 names, and the two Conduit templates, which are named for their consumer and their target | `system`, `root`, `root-access` |
| Every contract | Ends in its compatibility version, written `v` and a whole number (CTR-001) | `.v1` |

No two exports share a name, whatever their kinds (PKG-012). That is why the Type of the
`management` Resource is `management.v1` and its capability is `object-management.v1`.

## The operations

One contract stands for one verb. Where a verb applies both to a Resource and to other
instances, the same contract is included in the capability of a Pool and in the capability of
`management`, and ACC-033 says which target authorizes a request from the object it names. A
request through a Conduit to the other target reports refused.

An instance is set up by one invocation of `set-up.v1` and retired by one of `retire.v1`,
requested of the target ACC-033 names, with the instance as an input. The engine implements
both. A template's `setup` and `retire` graphs are the steps the engine runs for that
instance: they are not implementations selected in the `implementation` map, and they have no
inputs of their own. "The setup operation" and "the retire operation" of an instance mean
these invocations. A Conduit is set up by `open.v1` and retired by `close.v1` in the same way.

| Operation | Included in | `revision` input | `deduplicates` | Reserved to `root` | Promise |
| --- | --- | --- | --- | --- | --- |
| `register.v1` | `object-management.v1`, `pool-management.v1` | — | yes | — | SUB-006, ACC-033 |
| `replace.v1` | `object-management.v1` | required | yes | — | SUB-009 |
| `rename.v1` | `object-management.v1` | required | yes | — | OBJ-026 |
| `unregister.v1` | `object-management.v1` | required | yes | — | LIF-017 |
| `inspect.v1` | `object-management.v1`, `pool-management.v1` | — | — | — | LIF-013 |
| `set-up.v1` | `object-management.v1`, `pool-management.v1` | — | no | — | LIF-003, POOL-013 |
| `retire.v1` | `object-management.v1`, `pool-management.v1` | — | no | — | LIF-005, TPL-009 |
| `reconcile.v1` | `object-management.v1`, `pool-management.v1`, `access-control.v1` | for a control | yes | — | OPS-014, ACC-033, LIF-028, REV-010 |
| `register-replacement.v1` | `object-management.v1`, `pool-management.v1` | — | yes | — | LIF-009, POOL-003 |
| `install.v1` | `object-management.v1` | — | yes | — | ACC-033, OPS-019 |
| `declare-lost.v1` | `object-management.v1` | — | yes | yes | PLC-017, ACC-032 |
| `purge.v1` | `object-management.v1` | — | yes | yes | REC-004 |
| `retire-with-resources.v1` | `pool-management.v1` | — | no | — | LIF-021 |
| `open.v1` | `access-control.v1` | — | yes | — | ACC-006 |
| `close.v1` | `access-control.v1` | — | yes | — | ACC-012 |
| `apply-policy.v1` | `access-control.v1` | — | yes | — | ACC-031 |
| `remove-policy.v1` | `access-control.v1` | — | yes | — | ACC-031 |
| `retrieve-result.v1` | No capability: presented to every instance | — | — | — | IFC-024, OPS-016 |
| `start-later-work.v1` | No capability: presented to every instance | — | no | — | IFC-035, ACC-040 |
| `set-default-location.v1` | `system-settings.v1` | required | yes | — | PLC-016 |
| `set-facts.v1` | `location-settings.v1` | required | yes | — | PLC-021 |

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
  list it (OPS-020, CON-017). `changes` is a map from the name of an input to the JSON Pointer,
  into the configuration, of the setting that input sets: `set-facts.v1` states
  `changes: {facts: /facts}`. `revision` is the one input that must always be supplied. Every
  other input is optional, and one that is not supplied leaves its setting as stored
  (REV-006): the settings a control addressed are those of the inputs it was sent. The result
  states the settings it confirmed in the output `settings`: a map keyed by the pointers
  of the control's `changes`, with one member for each place confirmed, holding the value now
  stored there (REV-007). `set-facts.v1` returns `settings: {/facts: ...}`. The input
  `settings` of `reconcile.v1`, which states the settings of a reconciled control that took
  effect, has the same shape (REV-010).
- **Reconciling.** `reconcile.v1` is in the capability of `management`, of a Pool and of
  `access`, and in no other. An invocation of a management verb is reconciled through the
  target that authorized the request being reconciled, by an open Conduit to that target
  that lists `reconcile.v1`. Any other invocation, of a control or of any other operation,
  is reconciled through the target that manages the instance it was dispatched to: the Pool
  that owns it where it is a Resource, and `management` where it is an Agent, an Interface
  or a Pool (ACC-033, OPS-014). So a control of a location is reconciled through a Conduit
  to `system`, which owns every location, and a control of the `system` Pool through a
  Conduit to `management`. No capability, of this package or of any other, has to include
  `reconcile.v1` beside those three. Where the Pool that owns a Resource is retired, every
  request that the Pool would have authorized is authorized against `management` in its
  place, inspecting the Resource and reconciling its invocations among them (ACC-033).
  Recording that an unknown setup or open occurred makes the
  instance ready or the Conduit open. Recording that a retirement or a close occurred
  confirms the shutdown. Recording that the effect did not occur gives the outcome failed
  and changes nothing else (LIF-028). For a control, the settings stated are stored in the
  current configuration, or nothing is (REV-010). A consumer learns the ID that names the
  invocation from the `io.github.shrav-k.conduit/invocation` of its result (IFC-025), or
  from the output `invocation` of `retrieve-result.v1` (IFC-024).
- **Defaults.** No input of a core operation states a `default` (CTR-002, OPS-026). An input
  that may be left out is marked `optional: true`. Each input of the requirement `bound.v1`
  states `default: []`, and may be left out for that reason alone (OBJ-015).
- **`deduplicates`.** Declared where the whole effect is a change to what the engine itself
  stores, which the engine recognizes by the operation identity, so a repeat after an unknown
  outcome can be dispatched as a new attempt (OPS-012). Not declared where the engine runs the
  steps of a template's graph or invokes other implementations, as it does for `set-up.v1`,
  `retire.v1`, `retire-with-resources.v1` and `start-later-work.v1`. `inspect.v1` and
  `retrieve-result.v1` state no `effect`.
- **Reserved to `root`.** Written `reservedToRoot: true` in the contract (CTR-002). ACC-032
  says what the mark means, and only an Operation contract of the core package carries it: any
  other package that holds one is rejected when it is installed, and a project that holds one
  when it is submitted (CTR-009).
- **How a request names things.** An object is named by a reference and never by an ID, and a
  retired instance's name still resolves until the instance is unregistered (LIF-007). An
  invocation and a policy are not objects: `reconcile.v1` names an invocation, and
  `remove-policy.v1` a policy, by the ID the engine gave it. A request made through a Conduit
  that lists the operation, and that names an object, a policy or an invocation that does not
  exist, reports invalid (OPS-029). So does a request for `set-up.v1` or `retire.v1` that
  names a template or a Conduit (LIF-012).
- **The resolved form of a reference.** Where a value is a reference, the instance stores
  `{kind: <Kind>, id: <ID>}` in its place (OBJ-009, OBJ-021). Every reference schema accepts
  that form, and only the engine writes it. An implementation receives it in its `arguments`
  and in the restrictions it is sent, a graph reads it through a value reference, a control
  returns it in `settings`, and `inspect.v1` shows it under `declared`.
- **Diagnostics.** `register.v1`, `replace.v1`, `register-replacement.v1` and `install.v1`
  declare the output `diagnostics`, for the diagnostics that did not prevent the request.
  What a request that is not dispatched says beyond its reason, it says in `diagnostics` in
  the `structuredContent` of its result, whether or not the operation declares an output of
  that name (IFC-025). A rejected registration carries them so (OBJ-023), and a request to
  retire or to unregister a Pool whose Resources are in the way names those Resources so
  (LIF-020), though `retire.v1` and `unregister.v1` declare no output. A request that is not
  permitted reports refused and says nothing else about the objects it names (OPS-023). A
  rejected installation reports invalid, unavailable or conflict as PKG-023 to PKG-025 state.
- **Retiring.** A request to retire an instance whose shutdown is not confirmed is a further
  retirement (LIF-026), and one to retire an instance whose shutdown is confirmed changes
  nothing and succeeds (LIF-030). The Conduits of which a retired instance is the target are
  closed at once. Those of which it is the consumer are closed once its shutdown is
  confirmed, and until then permit only the requests of its retirement (CON-008, ACC-039).
  Where Resources remain whose shutdown is not confirmed, `retire-with-resources.v1` has the
  outcome failed, and the diagnostics of its result name them (LIF-021). It declares no
  output, because a result whose outcome is failed holds none (IFC-025).
- **Installing.** A package is installed only by `install.v1`. A registration installs none
  of the packages its manifest lists, and `fetch` in an item of `packages` is the place an
  installation of that item fetches from (NAM-012, PKG-006).
- **Published contracts only.** The input `requirement` of `apply-policy.v1`, and each item
  of the argument `operations` of `bound.v1`, is a qualified reference. A policy and a bound
  therefore name contracts of published packages only (ACC-031, ACC-029).
- No core operation starts model work (IMP-002), so none carries `startsModelWork`. None
  arranges later work either, so none carries `arrangesLaterWork` (CTR-002, ACC-019).
  `start-later-work.v1` starts work that another operation arranged, and the marks of the
  two operations it names decide whether the start is dispatched (ACC-040, IMP-006).
- Every core operation states `github.com/shrav-k/conduit/core#engine` as its default
  implementation. The engine performs it; no process is started and no request crosses the
  implementation interface.

The tool that IFC-024 always presents is named `retrieve-result.v1`, with no Conduit's name
before it. An instance names an invocation that began with a request it sent itself by
`operationIdentity` and `target` (OPS-016). An
operation identity, and the retrieval of a result, are scoped to the caller, the target and
the instance that sent the request (OPS-008), and an identity under which the sender has no
invocation reports refused (OPS-015). A retained result belongs to the Conduit its
invocation was dispatched through: its content is delivered only while that Conduit is open,
and never under another one (OPS-017, ACC-034). Once the target is unregistered it can no
longer be named, so the request reports refused, as for any identity under which the sender
can reach no invocation; the records of the invocation remain (OPS-016, LIF-019). The tool's
result holds `state`, `invocation`, `withheld` and `outputs`. `invocation` is the engine's
ID for the invocation asked about, so an instance that lost the result of its request can
still reconcile it. `outputs` is absent where content is withheld, and where the engine no
longer keeps it (OPS-016). `state` and `withheld` are also what a result that answers a
request directly carries in its `_meta`:
`io.github.shrav-k.conduit/state` with the value `inProgress` for a repeat of an invocation
that has not ended (OPS-009), and `io.github.shrav-k.conduit/withheld` with the value true
where content is withheld (ACC-034). IFC-025 states both. A result's `_meta` may also hold
`io.github.shrav-k.conduit/restriction`, which names the restriction that is why a request
ended with a reason: an implementation gives it under IFC-026, and the engine's own result
carries it under the same key (IFC-025). A result that has an outcome, or that answers a
repeat, holds `io.github.shrav-k.conduit/invocation`, the engine's ID for the invocation,
which is the ID that `reconcile.v1` takes. No operation of the core package declares any key
of `_meta` as an output, because `_meta` is not part of an operation's outputs.

A result that the engine gives a consumer has `isError` true where it carries none of the
operation's outputs: where the request ended with a reason, where its outcome is failed or
unknown, where its content is withheld, and where it is a repeat of an invocation that has
not ended, or of one that succeeded and whose outputs the engine no longer keeps. Any other
result holds the outputs in `structuredContent`, which conforms to the tool's `outputSchema`
(IFC-025, IFC-033). The result of an operation that declares no outputs has no content to
withhold. A call that names a tool the engine does not present is answered with no
tool result: the engine returns the protocol's error for invalid parameters, code -32602,
whose `data` holds `io.github.shrav-k.conduit/reason` with the value refused (IFC-012).

The engine always presents a second tool, `start-later-work.v1`, also with no Conduit's name
before it (IFC-035). The implementation that holds later work asks with it for each start,
naming the engine's ID for the invocation that arranged the work, the operation to start
and its arguments. The engine dispatches the start for the caller of the arranging
invocation and through the Conduit that invocation was dispatched through, where that
invocation was dispatched to the instance that asks and is of an operation marked
`arrangesLaterWork`, that Conduit is open and lists the operation, and, where the operation
starts model work, the arranging operation is marked `startsModelWork` too. Where one of
these does not hold, the request reports refused (ACC-040). The result is that of the started invocation, with the
started operation's outputs in `outputs`.

The definition of a tool that the engine presents for an operation of a Conduit holds
`io.github.shrav-k.conduit/restrictions` in its `_meta` where the Conduit carries a filter
whose contract names `consumer` in `obliges`: the list of those filters, in the form a
request's `restrictions` has (IFC-034).

The keys of a request's `_meta` are those of IFC-006: `interfaceVersion`, `instance`,
`operation`, `invocation`, `caller` and `restrictions`, with `operationIdentity` and `conduit`
where they apply, and `serving` on an onward request (IFC-020), each under the prefix
`io.github.shrav-k.conduit/`. There is no key for a package. A contract of a published
package is named on the wire by its qualified reference. A contract of an unpublished
package has no address, so it is named by the digest that identifies its unpublished package
(SUB-008), then `#`, then its name: `<digest>#<name>`. Two contracts of two unpublished
packages are told apart in one request in that way.

`inspect.v1` returns `id`, `revision`, `packages`, `declared`, `status` and `records`
(LIF-013). `packages` is a list. Each item has `address`, `version` and `digest`; the core
package's item has no `digest`, and an unpublished package's has `digest` alone (NAM-009,
PKG-005). `declared` shows each reference in its resolved form, `{kind, id}`. For an open
Conduit it shows, under each requirement the Conduit applies, the implementation that applies it,
named by its qualified reference or, for an implementation of a project that is not
published, as `<digest>#<name>` (ACC-038). The status of `access` lists the policies in
force under `policies`, each with the `id` that `apply-policy.v1` returned (ACC-031).

A policy applies to each request whose caller is one of its `actors`, where that list is
given, and whose target is one of its `targets`, where that list is given. A request to
`apply-policy.v1` gives at least one of the two (ACC-031), and no policy applies to a request
whose caller is the root Interface (ACC-035).

## First start

At first start (FST-001, FST-002) the engine instantiates five documents, which are written out
as an example in
[spec/schemas/examples/valid/first-start](../schemas/examples/valid/first-start/first-start.yaml).
The example shows what first start makes. The engine is handed no such documents, and a
submission that held them would be rejected (FST-006, OBJ-002):

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

The first-start objects are identified by their IDs. None of them is renamed, and nor is
either of the two Conduits of `root` (LIF-022). No second instance is made from the templates
`system`, `management`, `access` and `root`: an
instance document whose `from` names one of the four is rejected (FST-006). The template
`location` is instantiated for every location, and a location's document writes
`owner: system` (PLC-008).

A location's stored facts (PLC-007, PLC-008, PLC-013) are the value of `facts` in its
configuration. The two facts the core package's schemas name are `host` and `localPaths`; a
location may state others. The argument `facts` sets them when the location is instantiated.
After that
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
| `interface.v1` | Type | TPL-002: the template `root` needs a type; the basic Interface type |
| `pool.v1` | Type | TPL-002: a Pool template needs a type; the basic Pool type; CTR-004 |
| `system-pool.v1` | Type | TPL-002, PLC-003, PLC-016 |
| `location.v1` | Type | PLC-008, CTR-006 |
| `management.v1` | Type | TPL-002, ACC-033 |
| `access.v1` | Type | TPL-002, ACC-006, ACC-031 |
| `object-management.v1` | Capability | OPS-019, ACC-033 |
| `access-control.v1` | Capability | OPS-019, ACC-006, ACC-012, ACC-031, ACC-033 |
| `pool-management.v1` | Capability | OPS-019, ACC-033, ACC-028, CTR-004 |
| `system-settings.v1` | Capability | PLC-016, CON-017 |
| `location-settings.v1` | Capability | PLC-021, CON-017 |
| `register.v1` | Operation | OPS-019, ACC-033, SUB-006 |
| `replace.v1` | Operation | SUB-009, SUB-007 |
| `rename.v1` | Operation | OBJ-026, OBJ-011 |
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
| `start-later-work.v1` | Operation | IFC-035, ACC-040, ACC-019 |
| `set-default-location.v1` | Operation, a control | PLC-016 |
| `set-facts.v1` | Operation, a control | PLC-021, PLC-008 |
| `bound.v1` | Requirement | ACC-029, ACC-030, CON-028 |
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
| `object.v1` | Schema | OBJ-003: a result names each instance made, and says which nested instantiation made a nested one. Its shape is a choice |
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
| `template-reference.v1` | Schema of a document's part | INS-025, INS-002, FST-005 |
| `declared-conduit.v1` | Schema of a document's part | OBJ-014 |
| `pool-entry.v1` | Schema of a document's part | OBJ-014, POOL-009 |
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

- The full shape of an inspection result, and the shape of a diagnostic beyond `message`.
  Both are settled with the conformance vectors: what a vector compares fixes the shape.
  `inspect.v1` fixes the names of
  its outputs, the shape of `packages`, the lifecycle state that every status carries, the
  shutdown that the status of a retired instance carries, and that a policy listed in the
  status of `access` has an `id`; `diagnostic.v1` fixes only `message`.
  That `declared` shows each reference as `{kind, id}`, and names under each requirement of a
  Conduit the implementation that applies it, is settled; the keys that hold them are not.
  Not fixed either: where the result of inspecting a Conduit states, for each restriction,
  enforced or mediated (ACC-027); the keys of a record (REC-002); the form of an observation
  with its time and its origin (OBS-002, OBS-003); where conditions appear (LIF-011); where
  the word unknown stands against a setting (REV-009); how the status names a setup and its
  outcome, what retired an instance and which event closed a Conduit (LIF-004, LIF-005,
  CON-008, CON-011); the members that list the actors or targets of a policy (ACC-031); where
  an instance's `from` is reported (FST-005); and how a gap in the records is shown
  (REC-006). Settled: what a request that is not dispatched says beyond its reason, it says
  in diagnostics, so a request to retire or to unregister a Pool names the Resources in the
  way in a diagnostic, and `retire.v1` and `unregister.v1` need no output (LIF-020, IFC-025).
  How a diagnostic names a Resource is part of its shape.

It is listed, with the promises concerned, in the
[record](../../docs/proposals/core-and-schemas/README.md). The other point that page once
listed is settled: a request to retrieve the result of an invocation whose target has been
unregistered reports refused (OPS-016).

No contract of the core package states `restoresRuntimes`, `records` or `obliges`, and none is
a composition (CTR-004, CTR-005, CTR-010). No core type declares a runtime. `bound.v1` is a
filter, and the implementation of the target must always state a filter in `supports`
(CON-022): `engine`, which is the implementation behind `access`, does. `shared-host.v1` does
not narrow requests, and the engine itself checks it (PLC-013). It is the one requirement
that does not narrow requests and may be applied to a Conduit: one that is not a contract of
the core package is rejected (CON-030).
