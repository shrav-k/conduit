# The core package

Status: **accepted by the owner as it stands at
[ruling 39](../../docs/proposals/ruling-39/README.md)** (2026-11-06), which accepted every
logged decision of the coordinator and choice of an editor, and **changed by
[ruling 41](../../docs/proposals/ruling-41/README.md)** of the same day; see
[the specification's README](../README.md). Plan-audit ruling 37 first accepted it on
2026-11-05, as one batch with [the schemas](../schemas/README.md) and
[the manifest's promises](../promises/manifest.yaml). It was written for decision G1 of the
[re-audit](../../docs/proposals/reaudit/README.md). The record of the batch, with every choice
made here and its alternative, is
[docs/proposals/core-and-schemas](../../docs/proposals/core-and-schemas/README.md), and what
the owner changed is in [APPLY.md](../../docs/proposals/core-and-schemas/APPLY.md).

What follows was changed after ruling 37, on the logged decisions of the coordinator of the
re-audit, and on [ruling 38](../../docs/proposals/ruling-38/README.md) of 2026-11-06.
The decisions of the [docs audit](../../docs/proposals/docs-audit/GAPS.md) settled the points
that batch left open and changed some descriptions here; they added no export. The
[decisions of the second re-audit round](../../docs/proposals/reaudit/round-2/DECISIONS.md),
of the [third](../../docs/proposals/reaudit/round-3/DECISIONS.md) and of the
[fourth](../../docs/proposals/reaudit/round-4/DECISIONS.md) changed contracts and
descriptions; what changed is in the logs that each names. Ruling 38 removed the export
`start-later-work.v1`, because later work is not part of this version; what it changed is in
the [log of that pass](../../docs/proposals/ruling-38/log-spec.md).

[Ruling 40](../../docs/proposals/ruling-40/README.md) and ruling 41, which accepted the
[principles review](../../docs/proposals/principles-review/README.md), made Conduit innovate
only on connection and access: everything else is a provider behind a contract that the user
picks. The [work order](../../docs/proposals/ruling-41/PLAN.md) applied them here, and the
[log of that pass](../../docs/proposals/ruling-41/log-core.md) lists every change. In short:
16 exports were removed, the rename, retrieval, policy, default-location and location-facts
operations, two capabilities, the type of `system` and the schemas of graphs, runtimes and
composite components; 20 were added, the two hooks, the provider capabilities and their
operations, the operation of a secret store and the schemas of a handle, a condition, a
record, a secret reference, a label selector and server.json's Package and Remote; and every
Operation contract is written as the Model Context Protocol writes a tool. The names that the
review and the work order coined, `secret-reveal.v1` and `reveal-secret.v1` among them, are
not final: the vocabulary audit that ruling 41 orders proposes names before pull request 7 is
finished. The [coverage check](../../docs/proposals/ruling-41/log-integration.md#coverage-check)
of the rewrite then changed the description of `run.v1`, which runs the item only on the
machine that `location` names, and of `bound.v1`, whose `operations` are compared as names,
and no longer says that a control requires `revision` alone.

The promises are the source of truth: where a document here and a promise differ, the document
is wrong.

## What this is

The core package is the package built into every engine, at the address
`github.com/shrav-k/conduit/core` (NAM-020). A document refers to it without an item of
`packages` in a manifest, and it cannot be installed, listed or replaced (NAM-021). This
directory holds it as documents:
everything the promises say it exports, and everything first start needs.

| File | Holds |
| --- | --- |
| [conduit.yaml](conduit.yaml) | The package's manifest: its address, its version, its exports and its one implementation, the engine itself |
| [operations.yaml](operations.yaml) | The 23 Operation contracts: every management verb, the setup hook and the retire hook, the operations of the three provider capabilities that the engine invokes by provider calls, and the one operation of a secret store |
| [capabilities.yaml](capabilities.yaml) | The 7 Capability contracts: what `management`, `access` and a Pool offer, and the four provider capabilities: a runtime provider, the machines provider, a package-source provider and a secret store |
| [types.yaml](types.yaml) | The 5 Type contracts: the basic Interface and Pool types, and the types of a location, of `management` and of `access` |
| [requirements.yaml](requirements.yaml) | The 2 Requirement contracts: the bound of a right to open, and the shared-host requirement |
| [references.yaml](references.yaml) | The 12 reference schemas: the Schema contracts that make a value a reference (OBJ-021), a secret reference among them (SEC-001). Each also accepts the resolved form `{kind, id}`, which only the engine writes |
| [results.yaml](results.yaml) | 5 Schema contracts: an object as a result names it, a diagnostic, a handle, a condition and a record |
| [templates.yaml](templates.yaml) | The 7 templates that first start instantiates: those of the five first-start objects and of the two Conduits of `root` |

The package also exports the 41 Schema contracts under [spec/schemas](../schemas/README.md),
which are the schemas of documents themselves. They are part of this package because a `$ref`
from one schema to another names a Schema contract (SCH-015), and because `register.v1` states
its inputs with them. In all the package exports 102 documents, 58 of them Schema contracts,
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
| Type, Capability, Requirement, Schema | A noun, with hyphens | `location.v1`, `access-control.v1`, `bound.v1`, `resource-reference.v1` |
| Template | The name of the first-start object made from it, except `location`, which NAM-020 names, and the two Conduit templates, which are named for their consumer and their target | `system`, `root`, `root-access` |
| Every contract | Ends in its compatibility version, written `v` and a whole number (CTR-001) | `.v1` |

No two exports share a name, whatever their kinds (PKG-012). That is why the Type of the
`management` Resource is `management.v1` and its capability is `object-management.v1`.

## The operations

Every Operation contract is written as the Model Context Protocol writes a tool (CTR-002): the
properties of its `inputSchema` are its inputs and those of its `outputSchema` its outputs,
an input or an output is optional where `required` does not list it (OBJ-015), and the root of
each holds `type: object` and may hold `properties`, `required` and `additionalProperties`, and
nothing else (CTR-012). An operation with no inputs or no outputs states no `inputSchema` or no
`outputSchema`. The engine derives a
tool's definition from the contract (IFC-033).

One contract stands for one verb. Where a verb applies both to a Resource and to other
instances, the same contract is included in the capability of a Pool and in the capability of
`management`, and ACC-033 says which target authorizes a request from the object it names. A
request through a Conduit to the other target reports refused.

An instance is set up by one invocation of `set-up.v1` and retired by one of `retire.v1`,
requested of the target ACC-033 names, with the instance as an input. The engine implements
both, and neither is selected in an `implementation` map (TPL-008). What a setup does is
TPL-012: it opens the instance's declared Conduits, or for a Pool sets up the Resources of its
entries (CON-004, POOL-013); then it asks for the instance's run (PRV-007) and connects to
each `remote` item (IFC-041); then it dispatches the setup hook (TPL-017). What a retirement
does is TPL-009: the retire hook (TPL-018), then the stop of the run (PRV-008). A Conduit is
set up by `open.v1` and retired by `close.v1`.

### The management verbs

| Operation | Included in | `revision` input | `deduplicates` | Reserved to `root` | Promise |
| --- | --- | --- | --- | --- | --- |
| `register.v1` | `object-management.v1`, `pool-management.v1` | — | yes | — | SUB-006, ACC-033, PKG-006, MAN-015 |
| `replace.v1` | `object-management.v1` | required | yes | — | SUB-009 |
| `unregister.v1` | `object-management.v1` | required | yes | — | LIF-012, LIF-017 |
| `inspect.v1` | `object-management.v1`, `pool-management.v1` | — | — | — | LIF-013 |
| `set-up.v1` | `object-management.v1`, `pool-management.v1` | — | no | — | LIF-003, TPL-012 |
| `retire.v1` | `object-management.v1`, `pool-management.v1` | — | no | — | LIF-005, TPL-009 |
| `reconcile.v1` | `object-management.v1`, `pool-management.v1`, `access-control.v1` | for a control that took effect | yes | — | OPS-014, ACC-033, LIF-028, REV-010, OPS-031 |
| `register-replacement.v1` | `object-management.v1`, `pool-management.v1` | — | yes | — | LIF-009, POOL-003 |
| `install.v1` | `object-management.v1` | — | yes | — | ACC-033, OPS-019, PRV-017 |
| `declare-lost.v1` | `object-management.v1` | — | yes | yes | PLC-017, ACC-032 |
| `purge.v1` | `object-management.v1` | — | yes | yes | REC-004 |
| `retire-with-resources.v1` | `pool-management.v1` | — | no | — | LIF-021 |
| `open.v1` | `access-control.v1` | — | yes | — | ACC-006 |
| `close.v1` | `access-control.v1` | — | yes | — | ACC-012 |

### The hooks, and the operations of providers

| Operation | Included in | Invoked by | Promise |
| --- | --- | --- | --- |
| `setup-hook.v1` | No capability: a template selects it | The engine, as the last part of a setup, through no Conduit | TPL-017, OPS-028 |
| `retire-hook.v1` | No capability: a template selects it | The engine, within a retirement, before the stop | TPL-018, OPS-028 |
| `run.v1` | `runtime-provider.v1` | The engine, by a provider call | PRV-007, PRV-011, PRV-012 |
| `stop.v1` | `runtime-provider.v1` | The engine, by a provider call | PRV-008 |
| `inspect-run.v1` | `runtime-provider.v1` | The engine, by a provider call | PRV-009 |
| `list-machines.v1` | `machine-provider.v1` | The engine, by a provider call | PRV-014 |
| `place.v1` | `machine-provider.v1` | The engine, by a provider call | PRV-015 |
| `fetch-package.v1` | `package-source.v1` | The engine, by a provider call | PRV-017 |
| `reveal-secret.v1` | `secret-reveal.v1` | The engine's reads alone, which are requests through the reader's own Conduit | SEC-005 to SEC-007, SEC-010 |

None of these nine states a default implementation, and `engine` implements none of them
(MAN-012, SEC-020): a template selects an implementation for each that it needs, or its type's
default does (TPL-004). A provider call is not a request: no Conduit permits it, no access
decision is made for it, and it is recorded (PRV-005). So no Conduit lists an operation of
`runtime-provider.v1`, `machine-provider.v1` or `package-source.v1` (PRV-004). A Conduit may
list `reveal-secret.v1`, and only the root Interface opens one, for a Resource or an Agent
(SEC-008); no actor's request for it is dispatched and no tool is presented for it (SEC-007,
IFC-012). None of the provider operations, nor `reveal-secret.v1`, states an `effect`, apart
from `run.v1` and `stop.v1`.

### How to read the columns

- **`revision` input.** A request's `arguments` are exactly the operation's inputs (IFC-001), so
  an operation that changes a stored object takes the object's current revision as an input.
  One that names a revision that is not current reports conflict (REV-002), and one that names
  none reports invalid (REV-011). A revision is an opaque string, compared only for equality
  (REV-001). Setting up, retiring, retiring a Pool together with its Resources, opening and
  closing name no revision and advance none: they change an instance's lifecycle state, not
  its definition or configuration (REV-012). `replace.v1` returns the template's revision
  after the request in the output `revision`, which is the one it had where the request
  changed nothing (SUB-009).
- **Controls.** The core package has none. A control of another package states `changes`, a
  map from the name of an input to the JSON Pointer, into the configuration, of the setting
  that input sets; its `inputSchema` lists `revision` under `required`, beside any other
  input, which may still be left out (REV-006); and it declares no output `settings`
  (CTR-006). The tool presented for it lists `revision` alone as required (IFC-033). It
  applies exactly the values it is sent or fails, and the engine checks them against the
  type's schema before it dispatches (REV-007, REV-015).
- **Reconciling.** `reconcile.v1` is in the capability of `management`, of a Pool and of
  `access`, and in no other. A management verb is every operation of the first table
  (CON-024). An invocation of one is reconciled through the target that authorized the request
  being reconciled, by an open Conduit to that target that lists `reconcile.v1`. Any other
  invocation, of a control, of a hook or of any other operation, is reconciled through the
  target that manages the instance it was dispatched to: the Pool that owns it where it is a
  Resource, and `management` where it is an Agent, an Interface or a Pool (ACC-033, OPS-014).
  Where a Pool is retired, two of the requests that the Pool would have authorized are
  authorized against `management` in its place: inspecting a Resource the Pool owns, and
  reconciling an invocation. Where the instance of an invocation no longer exists, the
  invocation is reconciled through `management`, and the reconciliation records the outcome
  and stores nothing (ACC-033). Recording that an unknown setup or open occurred makes the
  instance ready or the Conduit open. Recording that a retirement or a close occurred confirms
  the shutdown, and the instance is removed (LIF-028, LIF-035). Recording that the effect did
  not occur gives the outcome failed and changes nothing else. An actor tells whether a run is
  running from the instance's condition `Running` (PRV-009): no handle is held, and
  `reconcile.v1` takes none. For a control that took effect, on an instance that exists, the
  request names `revision`, and the values the control was sent are stored (REV-010). A
  reconciliation that stores nothing names no `revision`, and one that does reports invalid
  (OPS-031). A request that names an invocation whose outcome is not unknown reports conflict
  (OPS-030). A consumer learns the ID that names the invocation from the
  `io.github.shrav-k.conduit/invocation` of its result, or of a repeat of its request (IFC-025,
  OPS-032).
- **Defaults.** No input of a core operation states a `default` (CTR-002, OPS-026). An input
  that may be left out is one that `required` does not list. Each input of the requirement
  `bound.v1` states `default: []`, and may be left out for that reason alone (OBJ-015).
- **`deduplicates`.** Declared where the whole effect is a change to what the engine itself
  stores, which the engine recognizes by the operation identity, so a repeat after an unknown
  outcome can be dispatched as a new attempt (OPS-012). Not declared where the engine dispatches
  other work, as it does for `set-up.v1`, `retire.v1` and `retire-with-resources.v1`.
  `inspect.v1` states no `effect`.
- **Reserved to `root`.** Written `reservedToRoot: true` in the contract (CTR-002). ACC-032
  says what the mark means, and only an Operation contract of the core package carries it: any
  other package that holds one is rejected when it is installed, and a project that holds one
  when it is submitted (CTR-009).
- **How a request names things.** A request that an actor makes on its own behalf names an
  object by a reference and never by an ID, and one that holds the resolved form in its place
  reports invalid (LIF-031). A retired instance's name still resolves until the instance is
  removed (LIF-007, LIF-035). An invocation is not an object: `reconcile.v1` names it by the ID
  the engine gave it. A request made through a Conduit that lists the operation, and that
  names an object or an invocation that does not exist, reports invalid (OPS-029). So does a
  request for `set-up.v1` or `retire.v1` that names a template or a Conduit (LIF-032).
  `register-replacement.v1` names the instance that holds a nested instantiation in
  `instance`, and the name of the declared Conduit or the entry in `nested` (LIF-009, LIF-033).
- **The resolved form of a reference.** Where a value is a reference, the instance stores
  `{kind: <Kind>, id: <ID>}` in its place (OBJ-009, OBJ-021). Every reference schema accepts
  that form, and only the engine writes it. An implementation receives it in its `arguments`,
  in the restrictions it is sent and in the configuration that every request to it carries
  (IFC-006), and `inspect.v1` shows it under `declared`. An onward request may therefore hold
  it in an input of a core operation, because the engine handed it to the instance that sends
  it; a request that an actor makes on its own behalf may not (LIF-031).
- **Diagnostics.** `register.v1`, `replace.v1`, `register-replacement.v1` and `install.v1`
  declare the output `diagnostics`, for the diagnostics that did not prevent the request. What
  a request that is not dispatched says beyond its reason, it says in `diagnostics` in the
  `structuredContent` of its result, whether or not the operation declares an output of that
  name, and a result that reports invalid or conflict carries there an error diagnostic whose
  `ruleId` is the ID of the promise that decided it (IFC-025). A diagnostic is SARIF 2.1.0's
  `result`, reduced (`diagnostic.v1`). A rejected registration carries them so (OBJ-023), and a
  request to retire a Pool whose Resources are in the way names those Resources so (LIF-020),
  though `retire.v1` declares no output. A request that is not permitted reports refused and
  says nothing else about the objects it names, and so does one whose access decision cannot
  be made, which reports unavailable (OPS-023, ACC-037). A request to install that installs
  nothing reports invalid, unavailable or conflict as PKG-023 to PKG-025 and PKG-029 state.
- **Retiring and removing.** A request to retire an instance whose shutdown is not confirmed
  is a further retirement (LIF-026). Once the shutdown of an instance is confirmed it is
  removed, with no request, and its records are kept (LIF-035, LIF-019): a request that names
  it afterwards reports invalid (OPS-029). `unregister.v1` removes a template (LIF-012), or a
  retired instance whose location was declared lost (LIF-017). The Conduits of which a retired
  instance is the target are closed at once. Those of which it is the consumer are closed once
  its shutdown is confirmed, and until then permit only the requests of its retirement
  (CON-008, ACC-039). `retire-with-resources.v1` retires the Resources in the reverse of the
  order of setup, and declares no output, because a result whose outcome is failed holds none
  (LIF-021, IFC-025).
- **Installing.** A package is installed only by `install.v1`. A registration installs none
  of the packages its manifest lists, and one whose manifest lists a package that is not
  installed at that version and digest is rejected (PKG-006); so is an installation whose
  manifest does (PKG-028). `fetch` is a URI reference with no user information (MAN-006). The
  engine fetches `https:` and `file:` URLs itself, and a package-source provider that claims
  another scheme fetches it with `fetch-package.v1` (NAM-012, PRV-017). A request for an address
  and version that is already installed is decided from what is installed, and fetches nothing
  (PKG-025, PKG-027). One whose `fetch` is a relative reference reports invalid, because a
  request has no directory (PKG-023).
- **Bounds.** Each item of the argument `operations` of `bound.v1` is a qualified reference,
  which reaches a contract of an installed package or of a submitted project, named
  `address#name` with the address its manifest states (ACC-029, SUB-008, MAN-015). The engine
  applies a bound itself and never sends it, as it does the shared-host requirement (CTR-005,
  CON-018). A right to open trusts its consumer with every operation the bound names on every
  target it covers (ACC-030).
- No core operation starts model work (IMP-002), so none carries `startsModelWork`. No
  operation arranges, through the engine, work that outlasts its invocation, and no contract
  carries a mark for it (ACC-019, CTR-002).
- Every management verb states `github.com/shrav-k/conduit/core#engine` as its default
  implementation. The engine performs it; no process is started and no request crosses the
  implementation interface.

## Results and Conduit's keys

A result that the engine gives a consumer has `isError` true where the request ended with a
reason or its outcome is failed or unknown, and in no other case. Any other result holds the
outputs in `structuredContent`, which conforms to the tool's `outputSchema` and is an empty map
where the operation declares no outputs (IFC-025, IFC-033). A result whose outcome is succeeded
holds its outputs even where the Conduit through which its invocation was dispatched was closed
before it ended (ACC-034). A repeat of an invocation that has not ended, or of one that
succeeded and whose outputs the engine no longer keeps, reports conflict, and its result holds
the engine's ID for the invocation (OPS-032); a caller that lost a result repeats its request
with the same operation identity (OPS-009). A call that names a tool the engine does not
present is answered with no tool result: the engine returns the protocol's error for invalid
parameters, code -32602, whose `data` holds `io.github.shrav-k.conduit/reason` with the value
refused (IFC-012). A request that the protocol has a server reject with an error of its own is
answered with that error and no tool result (IFC-036).

A result's `_meta` holds `io.github.shrav-k.conduit/outcome` where the request has an outcome,
and `io.github.shrav-k.conduit/reason` where it ended with a reason, with
`io.github.shrav-k.conduit/restriction` where a restriction is why; a result that has an
outcome, or that answers a repeat under OPS-032, holds `io.github.shrav-k.conduit/invocation`,
the ID that `reconcile.v1` takes (IFC-025). No operation of the core package declares any key
of `_meta` as an output, because `_meta` is not part of an operation's outputs.

Conduit's keys travel as Conduit's extension of the protocol,
`io.github.shrav-k.conduit/interface`, at its revision `v0`, which the engine declares on each
request to an implementation that carries them (IFC-037). Their `_meta` keys are those of
IFC-006: `instance`, `operation`, `invocation`, `caller`, `restrictions`, `config`, which is the
instance's configuration, and `revision`, its revision, with `operationIdentity` and `conduit`
where they apply, each under the prefix `io.github.shrav-k.conduit/`. Every request the engine
sends has a caller. `serving` is not one of them: it is a key of an onward request that an
instance sends to the engine (IFC-020), and the engine does not send it on. A contract of a
package, or of a submitted project, is named on the wire by its qualified reference, and a
submitted project's is `address#name` with the address its manifest states (SUB-008). A provider
call carries the keys that PRV-005 names.

`inspect.v1` returns `id`, `revision`, `packages`, `declared`, `status` and `records` (LIF-013).
`packages` is a list. A package's item has `address`, `version` and `digest`; the core package's
item has no `digest`; and the item of a submitted project has `address`, the digest that
identifies the contracts of that submission and `version` where its manifest states one
(NAM-009, PKG-005). `declared` shows each reference in its resolved form, `{kind, id}`. For an
open Conduit it shows, under each requirement the Conduit applies, the implementation that
applies it, named by its qualified reference; for a requirement of the core package that is
`github.com/shrav-k/conduit/core#engine` (ACC-038). The status holds the lifecycle state, the
shutdown of a retired instance (LIF-014, LIF-024) and `conditions`, each in the shape of
Kubernetes `metav1.Condition`, `condition.v1` (LIF-011, OBS-002, OBS-003), among them the
condition `Running` of an instance whose item is run (PRV-009). The status of a location holds
the labels last reported for its machine (PLC-026). `records` is a list of `record.v1`, each a
CloudEvent whose `data` holds the fields of a record (REC-002).

## Providers

A provider is a Resource whose type offers one of the four provider capabilities, and no other
of them (PRV-001): `runtime-provider.v1` runs an instance's implementation item, stops it and
reports its state; `machine-provider.v1` reports the machines, their labels and their states,
and places instances; `package-source.v1` fetches archives for the URL schemes it claims; and
`secret-reveal.v1` is the contract of a secret store. The user picks one by registering a
Resource of its type. An Agent or a Resource names the runtime provider that runs it in its
standard argument `runtime` (PRV-006); the machines provider and the package-source providers
are owned by `system`, at most one machines provider and one package-source provider for each
scheme (PRV-013). The engine's own runner, its own machine and its own `https:` and `file:`
fetching serve where no provider of a kind does (PRV-003); they are part of the engine and are
not selected as an implementation, so `engine` states none of them. A runtime provider runs an
item on the machine that `location` names in `run.v1`, or ends the run failed, so the engine's
own runner runs only instances placed on `local` (PRV-019). A runtime provider, the
machines provider and a package-source provider run on the engine's own runner, on `local`
(PRV-002).

A secret Resource's type is its store's own Type, whose schema is the form of the locator and
which lists no control (SEC-005). A place that `secret-reference.v1` governs holds a reference
to a secret Resource and never the secret itself (SEC-001, SEC-002). The engine reads a secret
before each run it asks for, through the reading instance's own open Conduit to the secret
Resource, and passes the value in the run's `secrets` (SEC-010, SEC-011); it keeps no copy and
puts the value nowhere else (SEC-017, SEC-018).

## First start

At first start (FST-001, FST-002) the engine instantiates five documents, which are written out
as an example in
[spec/schemas/examples/valid/first-start](../schemas/examples/valid/first-start/first-start.yaml).
The example shows what first start makes. The engine is handed no such documents, and a
submission that held them would be rejected (FST-006, OBJ-002):

| Object | Kind | `from` | Notes |
| --- | --- | --- | --- |
| `system` | Pool | `github.com/shrav-k/conduit/core#system` | Of the basic Pool type; owns the locations, `management`, `access`, and the machines provider and package-source providers |
| `local` | Resource | `github.com/shrav-k/conduit/core#location` | Owned by `system`; its own placement; the location of the machine the engine runs on, with an empty configuration and the labels reported for that machine (PLC-008, PLC-026) |
| `management` | Resource | `github.com/shrav-k/conduit/core#management` | Owned by `system`, placed on `local` |
| `access` | Resource | `github.com/shrav-k/conduit/core#access` | Owned by `system`, placed on `local` |
| `root` | Interface | `github.com/shrav-k/conduit/core#root` | Declares two Conduits, which are instantiated with it |

The two Conduits of FST-002 are the declared Conduits of the template `root`. Their local names
are `management` and `access`, so the root Interface's tools are `management.register.v1`,
`access.open.v1` and so on. `root-access` applies no bound, which is what ACC-022 requires.
`management` and `access` are not entries of the `system` Pool's template: an entry's Resource
gets a generated name (OBJ-003), and FST-001 fixes these two names.

The first-start objects are identified by their IDs, and a name is fixed once its object is
made (OBJ-011). No instance document is made from the templates `system`, `location`,
`management`, `access` and `root` (FST-006). The engine alone instantiates `location`: for
`local` at first start, and for every other machine that the machines provider reports and no
location stands for, naming the location from the machine's hostname (PLC-026). A location
offers no capability and lists no control, so nothing changes it once it is made apart from its
lifecycle state and the labels reported for it (PLC-008).

## Every export

"Requires" names the promises that need the export to exist. No row says "none".

| Export | Kind | Requires |
| --- | --- | --- |
| `system` | Pool template | FST-001, FST-005 |
| `location` | Resource template | FST-001, FST-005, PLC-008, PLC-026, NAM-020 |
| `management` | Resource template | FST-001, FST-005 |
| `access` | Resource template | FST-001, FST-005 |
| `root` | Interface template | FST-001, FST-002, FST-005 |
| `root-management` | Conduit template | FST-002, FST-005, ACC-032 |
| `root-access` | Conduit template | FST-002, FST-005, ACC-022 |
| `interface.v1` | Type | TPL-002: the template `root` needs a type; the basic Interface type |
| `pool.v1` | Type | TPL-002: a Pool template needs a type; the basic Pool type, and the type of `system`; CTR-004 |
| `location.v1` | Type | PLC-008, PLC-026 |
| `management.v1` | Type | TPL-002, ACC-033 |
| `access.v1` | Type | TPL-002, ACC-006 |
| `object-management.v1` | Capability | OPS-019, ACC-033 |
| `access-control.v1` | Capability | OPS-019, ACC-006, ACC-012, ACC-033 |
| `pool-management.v1` | Capability | OPS-019, ACC-033, ACC-028, CTR-004 |
| `runtime-provider.v1` | Capability | PRV-001, PRV-006, PRV-007 |
| `machine-provider.v1` | Capability | PRV-001, PRV-013, PRV-014, PRV-015 |
| `package-source.v1` | Capability | PRV-001, PRV-016, PRV-017 |
| `secret-reveal.v1` | Capability | SEC-002, SEC-005, PRV-001 |
| `register.v1` | Operation | OPS-019, ACC-033, SUB-006 |
| `replace.v1` | Operation | SUB-009, SUB-007 |
| `unregister.v1` | Operation | LIF-012, LIF-017 |
| `inspect.v1` | Operation | LIF-013, ACC-033 |
| `set-up.v1` | Operation | OPS-019, ACC-033, LIF-003, TPL-012 |
| `retire.v1` | Operation | OPS-019, ACC-033, LIF-005, TPL-009 |
| `reconcile.v1` | Operation | OPS-014, REV-010, OPS-031 |
| `register-replacement.v1` | Operation | LIF-009, POOL-003 |
| `install.v1` | Operation | OPS-019, ACC-033 |
| `declare-lost.v1` | Operation | PLC-017, ACC-032 |
| `purge.v1` | Operation | REC-004 |
| `retire-with-resources.v1` | Operation | LIF-021 |
| `open.v1` | Operation | ACC-006 |
| `close.v1` | Operation | ACC-012 |
| `setup-hook.v1` | Operation | TPL-017, TPL-003 |
| `retire-hook.v1` | Operation | TPL-018, TPL-003 |
| `run.v1` | Operation | PRV-007, PRV-011 |
| `stop.v1` | Operation | PRV-008 |
| `inspect-run.v1` | Operation | PRV-009 |
| `list-machines.v1` | Operation | PRV-014 |
| `place.v1` | Operation | PRV-015 |
| `fetch-package.v1` | Operation | PRV-017 |
| `reveal-secret.v1` | Operation | SEC-005, SEC-006, SEC-010 |
| `bound.v1` | Requirement | ACC-029, ACC-030, CON-028 |
| `shared-host.v1` | Requirement | PLC-013, CON-006 |
| `reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006 |
| `interface-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006 |
| `agent-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006 |
| `resource-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006, POOL-005, POOL-014 |
| `pool-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006, POOL-001 |
| `conduit-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-006 |
| `actor-reference.v1` | Schema, a reference schema | OBJ-021 |
| `consumer-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-013, CON-027 |
| `target-reference.v1` | Schema, a reference schema | OBJ-021, OBJ-013, CON-027 |
| `entry-reference.v1` | Schema, a reference schema | OBJ-021, POOL-005 |
| `pool-entry-reference.v1` | Schema, a reference schema | OBJ-021, POOL-014 |
| `secret-reference.v1` | Schema, a reference schema | OBJ-021, SEC-001, SEC-002 |
| `object.v1` | Schema | OBJ-003, LIF-009: a result names each instance made, and says which nested instantiation made a nested one. Its shape is a choice |
| `diagnostic.v1` | Schema | IFC-025, OBJ-008, PKG-006: SARIF 2.1.0's `result`, reduced |
| `handle.v1` | Schema | CTR-002, IFC-011 |
| `condition.v1` | Schema | LIF-011, OBS-002, PRV-009 |
| `record.v1` | Schema | REC-002, LIF-013 |
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
| `id.v1` | Schema of a document's part | OBJ-009, IFC-006: an object and an invocation each have an ID. Its type is a choice |
| `revision.v1` | Schema of a document's part | REV-001, REV-011 |
| `label-selector.v1` | Schema of a document's part | PLC-007, CTR-004, MAN-010, PRV-014 |
| `schema.v1` | Schema of a document's part | CTR-008, SCH-002, SCH-008, SCH-015 |
| `api-version.v1` | Schema of a document's part | OBJ-017 |
| `envelope.v1` | Schema of a document's part | OBJ-017, OBJ-020 |
| `template-metadata.v1` | Schema of a document's part | OBJ-018 |
| `instance-metadata.v1` | Schema of a document's part | OBJ-018, OBJ-003 |
| `contract-metadata.v1` | Schema of a document's part | OBJ-018, CTR-001 |
| `template.v1` | Schema of a document | OBJ-012 |
| `template-spec.v1` | Schema of a document's part | TPL-001, INS-012 |
| `pool-template-spec.v1` | Schema of a document's part | TPL-001, INS-012, POOL-009 |
| `conduit-template-spec.v1` | Schema of a document's part | CON-016, INS-012 |
| `template-input.v1` | Schema of a document's part | INS-021 |
| `implementation-map.v1` | Schema of a document's part | TPL-003 |
| `template-controls.v1` | Schema of a document's part | TPL-010 |
| `instance.v1` | Schema of a document | OBJ-013, PRV-006 |
| `template-reference.v1` | Schema of a document's part | INS-025, INS-002, FST-005 |
| `declared-conduit.v1` | Schema of a document's part | OBJ-014 |
| `pool-entry.v1` | Schema of a document's part | OBJ-014, POOL-009 |
| `contract.v1` | Schema of a document | CTR-001 |
| `type-spec.v1` | Schema of a document's part | CTR-004 |
| `capability-spec.v1` | Schema of a document's part | CTR-003 |
| `operation-spec.v1` | Schema of a document's part | CTR-002, CTR-006, CTR-009, CTR-012 |
| `requirement-spec.v1` | Schema of a document's part | CTR-005 |
| `applied-requirement.v1` | Schema of a document's part | NAM-019, CON-018 |
| `manifest.v1` | Schema of the manifest | NAM-006, NAM-021, PKG-017 to PKG-020, MAN-003 to MAN-012 |
| `implementation-item.v1` | Schema of the manifest's part | PKG-019, PKG-020, MAN-009, MAN-010 |
| `server-package.v1` | Schema of the manifest's part | MAN-010, PKG-019 |
| `server-remote.v1` | Schema of the manifest's part | MAN-010, IFC-041 |
| `document.v1` | Schema of a document | OBJ-020, SUB-005 |
| `engine` | Implementation | TPL-004 (every management verb has an implementation), MAN-012 (it states none of `command`, `package` and `remote` and no `supports`) |

## What this package does not settle

- The full shape of an inspection result. `inspect.v1` fixes the names of its outputs, the
  shape of `packages`, the lifecycle state that every status carries, the shutdown that the
  status of a retired instance carries, that its conditions are `condition.v1` and that its
  records are `record.v1`. That `declared` shows each reference as `{kind, id}`, and names under
  each requirement of a Conduit the implementation that applies it, is settled; the keys that
  hold them are not. Not fixed either: where the result of inspecting a Conduit states, for
  each restriction, enforced or mediated (ACC-027); the key that holds a location's labels in
  its status (PLC-026); where the word unknown stands against a setting (REV-009); how the
  status names a setup and its outcome, what retired an instance and which event closed a
  Conduit (LIF-004, LIF-005, CON-008, CON-011); where an instance's `from` is reported
  (FST-005); and how a gap in the records is shown (REC-006). These are settled with the
  conformance vectors: what a vector compares fixes the shape.
- How a diagnostic names a document or an object beyond the JSON Pointer of its logical
  location, and the `source` and `type` of a record's CloudEvent: an engine names them.
- The keys of `secrets` in `run.v1` are the JSON Pointers of the places read, which the item's
  `environment` and `arguments` bind to names; a value that is not a string at a place an item
  binds is not settled either. Both are open questions of the
  [log of this pass](../../docs/proposals/ruling-41/log-core.md#open-questions).

No contract of the core package states `records`, and no core type offers a provider
capability or declares a run of its own: the engine's own runner, machine and fetching are part
of the engine (PRV-003). `bound.v1` and `shared-host.v1` are applied by the engine itself and
never sent, so no implementation states either in `supports` (CTR-005, CON-022).
