# The core package

Status: draft specification. See [the specification's status](../README.md) for coverage and
pending rulings. The promises are the source of truth: where a document here and a promise differ,
the document is wrong.

The decision records and their reasons are listed in
[Where the reasons are](../README.md#where-the-reasons-are).

## What this is

The core package is the package built into every engine, at the address
`github.com/shrav-k/conduit/core` (NAM-020). A document refers to it without an item of
`packages` in a manifest, and it cannot be installed, listed or replaced (NAM-021). This
directory holds it as documents:
everything the promises say it exports, and everything first start needs.

| File | Holds |
| --- | --- |
| [conduit.yaml](conduit.yaml) | The package's manifest: its address, its version, its exports and its one implementation, the engine itself |
| [operations.yaml](operations.yaml) | The 31 Operation contracts: every management verb, the setup, retire and sign-in hooks, the operations of the three provider capabilities that the engine invokes by provider calls, and the one operation of a secret Resource |
| [capabilities.yaml](capabilities.yaml) | The 8 Capability contracts: what `management`, `access`, Pools and Fleets offer, and the four provider capabilities: a runtime provider, the machine provider, a package-source provider and a secret Resource |
| [types.yaml](types.yaml) | The 6 Type contracts: the basic Interface, Pool and Fleet types, and the types of a node, of `management` and of `access` |
| [requirements.yaml](requirements.yaml) | The 2 Requirement contracts: the bound of a right to open, and the shared-host requirement |
| [references.yaml](references.yaml) | The 13 reference schemas: the Schema contracts that make a value a reference (OBJ-021), a secret reference among them (SEC-001). Each also accepts the resolved form `{kind, id}`, which only the engine writes |
| [results.yaml](results.yaml) | 12 Schema contracts: an object as a result names it, a diagnostic, a handle, a condition, a record, an inventory, a tombstone, opening provenance and a trust domain |
| [templates.yaml](templates.yaml) | The 17 templates: first-start objects and Conduits, defaults, scoped administration and explicit cascades |

The package also exports the 51 Schema contracts under [spec/schemas](../schemas/README.md),
which are the schemas of documents themselves. They are part of this package because a `$ref`
from one schema to another names a Schema contract (SCH-015), and because `register.v1` states
its inputs with them. In all the package exports 140 documents, 76 of them Schema contracts,
and has one implementation, which is also an export (PKG-019).

A document of the core package refers to another by its full address, as
`github.com/shrav-k/conduit/core#name`. That is the one exception to the rule that a document
refers to something in its own package as `#name` (NAM-022).

Every document here validates against those schemas, and `python3 tools/check_schemas.py`
checks it.

The unreleased core remains version 0.1.0. Runtime-sensitive contract edits are staged for
build orders 039, 040 and 041a under [036b-staged](../../docs/work-orders/036b-staged/).
Those files contain exact patches; only the contracts present here are active. Sign-in and pending-action contracts are present; 038b supplies enrollment.

## Names

`serving` and `unreconciled` are working names.

Export names follow NAM-004, and each becomes a tool's name under IFC-012, so they are short.
The convention, which follows the names the promises already use (`local-directory.v1`,
`shared-host.v1`, `node`):

| Kind | Named as | Example |
| --- | --- | --- |
| Operation | The verb, in the imperative, as the commands of `kubectl` and `docker` are named. Words are joined by hyphens | `register.v1`, `set-up.v1` |
| Hook | Named for the invocation it belongs to, then `-hook` | `setup-hook.v1`, `retire-hook.v1` |
| Type, Capability, Requirement, Schema | A noun, with hyphens | `node.v1`, `access-control.v1`, `bound.v1`, `resource-reference.v1` |
| Template | The name of the first-start object made from it, except `node`, which FST-005 names; Conduit templates name their consumer and target or their use | `system`, `root`, `root-access` |
| Every contract | Ends in its compatibility version, written `v` and a whole number (CTR-001) | `.v1` |

No two exports share a name, whatever their kinds (PKG-012). That is why the Type of the
`management` Resource is `management.v1` and its capability is `object-management.v1`.

## The operations

Every Operation contract is written as the Model Context Protocol writes a tool (CTR-002): the
properties of its `inputSchema` are its inputs and those of its `outputSchema` its outputs, an
input or an output is optional where `required` does not list it (OBJ-015), and the root of each
holds `type: object`. The root of an operation that is not a control may hold any other keyword
of the profile, as the protocol lets a tool's `inputSchema` do, such as a `oneOf` whose branches
say which inputs are present; that of a control may hold beside `type` only `properties`,
`required` and `additionalProperties`, so that every input but `revision` may be left out
(CTR-012, REV-006). The root of every core operation holds no keyword but `type`, `properties`
and `required`. An operation with no inputs or no outputs states no `inputSchema` or no
`outputSchema`.

The engine derives a tool's definition from the contract as the target holds it (IFC-033). The
definition stands alone: where a schema refers to a Schema contract, the definition holds a copy
of that contract's schema under the `$defs` of its root, keyed by the contract's qualified
reference and, where one is stated, the version of the package or project that holds it, so that
no two packages' contracts collide, with each `$ref` rewritten to point to what it pointed to
where it was written and every keyword beside a `$ref` kept. Each contract is copied once, so
one that refers to itself, directly or through others, needs no more. A contract whose schema
holds no `$ref`, referred to with no keyword beside the `$ref`, may stand in place of the
reference instead. The tools of `register.v1` and `replace.v1`, whose inputs refer to the
schemas of documents, so hold a `$defs` at their roots. For a control, an input that the
target's template narrows is given `allOf` of the contract's schema and the template's, and a
control that the target does not have has no tool (TPL-010, TPL-013, IFC-012).

One contract for one verb. Where a verb applies both to a Resource and to other
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
| `register.v1` | `object-management.v1`, `pool-management.v1`, `fleet-management.v1` | — | yes | — | SUB-006, ACC-033, PKG-006, MAN-015, MAN-018, SEC-024 |
| `replace.v1` | `object-management.v1` | required | yes | — | SUB-009, SUB-008, PKG-006, MAN-015, MAN-016 |
| `unregister.v1` | `object-management.v1` | required | yes | — | LIF-012, LIF-017, LIF-036, LIF-037 |
| `inspect.v1` | `object-management.v1`, `pool-management.v1`, `fleet-management.v1` | — | — | — | LIF-013, LIF-037 |
| `set-up.v1` | `object-management.v1`, `pool-management.v1`, `fleet-management.v1` | — | no | — | LIF-003, TPL-012, LIF-037 |
| `retire.v1` | `object-management.v1`, `pool-management.v1`, `fleet-management.v1` | — | no | — | LIF-005, TPL-009, LIF-037 |
| `reconcile.v1` | `object-management.v1`, `pool-management.v1`, `fleet-management.v1`, `access-control.v1` | for a control that took effect | yes | — | OPS-014, ACC-033, LIF-028, REV-010, OPS-031 |
| `register-replacement.v1` | `object-management.v1`, `pool-management.v1` | — | yes | — | LIF-009, POOL-003 |
| `install.v1` | `object-management.v1` | — | yes | — | ACC-033, OPS-019, PRV-017 |
| `declare-lost.v1` | `object-management.v1` | — | yes | yes | PLC-017, ACC-032 |
| `purge.v1` | `object-management.v1` | — | yes | yes | REC-004 |
| `retire-with-resources.v1` | `pool-management.v1` | — | no | — | LIF-021 |
| `open.v1` | `access-control.v1` | — | yes | — | ACC-006 |
| `close.v1` | `access-control.v1` | — | yes | — | ACC-012 |
| `list.v1` | `object-management.v1`, `pool-management.v1`, `fleet-management.v1` | — | — | — | OPS-036, ACC-033 |
| `retire-with-agents.v1` | `fleet-management.v1` | — | no | — | LIF-021 |
| `lift.v1` | `access-control.v1` | — | yes | — | ACC-053 |
| `answer.v1` | `object-management.v1`, `pool-management.v1`, `fleet-management.v1` | — | yes | — | PND-009, ACC-033 |
| `sign-in.v1` | `pool-management.v1`, `fleet-management.v1` | — | no | — | PND-010, PND-011, IFC-049 |

### The hooks, and the operations of providers

| Operation | Included in | Invoked by | Promise |
| --- | --- | --- | --- |
| `setup-hook.v1` | No capability: a template selects it | The engine, as the last part of a setup, through no Conduit | TPL-017, OPS-028 |
| `retire-hook.v1` | No capability: a template selects it | The engine, within a retirement, before the stop | TPL-018, OPS-028 |
| `sign-in-hook.v1` | No capability: required by a signIn Type | The engine, as a part of sign-in or setup | CTR-013, PND-011 |
| `account-hook.v1` | No capability: required by a signIn Type | The engine, as a part of sign-in or setup | CTR-013, PND-011 |
| `sign-out-hook.v1` | No capability: required by a signIn Type | The engine, as a part of sign-in or setup | CTR-013, PND-011 |
| `run.v1` | `runtime-provider.v1` | The engine, by a provider call | PRV-007, PRV-011, PRV-012, PRV-022, PRV-025, PRV-028 |
| `stop.v1` | `runtime-provider.v1` | The engine, by a provider call | PRV-008, PRV-021, PRV-027, LIF-036 |
| `inspect-run.v1` | `runtime-provider.v1` | The engine, by a provider call | PRV-009, PRV-027, PRV-028, IFC-017 |
| `list-machines.v1` | `machine-provider.v1` | The engine, by a provider call | PRV-014 |
| `place.v1` | `machine-provider.v1` | The engine, by a provider call | PRV-015 |
| `fetch-package.v1` | `package-source.v1` | The engine, by a provider call | PRV-017 |
| `get-secret.v1` | `secret-provider.v1` | The engine's reads alone, which are requests through the instance's own Conduit | SEC-005 to SEC-007, SEC-010, SEC-023, PRV-026 |

No Operation contract states a default implementation (CTR-002), and `engine` implements none
of these nine (MAN-012, SEC-020): a template selects an implementation for each that it needs,
or its type's default does (TPL-004). A provider call is not a request: no Conduit permits it, no access
decision is made for it, and it is recorded (PRV-005). So no Conduit lists an operation of
`runtime-provider.v1`, `machine-provider.v1` or `package-source.v1` (PRV-004). A Conduit may
list `get-secret.v1`, and only the root Interface opens one, for a Resource or an Agent
(SEC-008); no actor's request for it is dispatched and no tool is presented for it (SEC-007,
IFC-012). None of the provider operations, nor `get-secret.v1`, states an `effect`, apart
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
  that input sets, in which no two pointers are equal and none lies inside another, compared
  as reference tokens, the empty pointer lying over every other; its `inputSchema` lists
  `revision` under `required`, beside any other input, which may still be left out (REV-006);
  and it declares no output `settings` (CTR-006). The tool presented for it lists `revision`
  alone as required, and shows each input that the target's template narrows with both
  schemas (IFC-033). It applies exactly the values it is sent or fails, and the engine checks
  them against the type's schema before it dispatches (REV-007, REV-015).
- **Reconciling.** `reconcile.v1` is in the capability of `management`, of a Pool, of a Fleet and of
  `access`, and in no other. A management verb is every operation of the first table
  (CON-024). An invocation of one is reconciled through the target that authorized the request
  being reconciled, by an open Conduit to that target that lists `reconcile.v1`. Any other
  invocation, of a control, of a hook or of any other operation, is reconciled through the
  target that manages the instance it was dispatched to: the Pool that owns it where it is a
  Resource, its owning Fleet where it is an Agent, and `management` where it is an Interface, Pool or Fleet (ACC-033, OPS-014).
  Where a Pool is retired, two of the requests that the Pool would have authorized are
  authorized against `management` in its place: inspecting a Resource the Pool owns, and
  reconciling an invocation. Where the instance of an invocation no longer exists, the
  invocation is reconciled through `management`, and the reconciliation records the outcome
  and stores nothing (ACC-033). Recording that an unknown setup or open occurred makes the
  instance ready or the Conduit open, unless the instance or the Conduit's consumer has been
  retired. Recording that a retirement or a close occurred confirms the shutdown, and the
  instance is removed (LIF-028, LIF-035). Recording that the effect did not occur gives the
  outcome failed and changes nothing else. An actor tells whether a run is running from the
  instance's condition `Running` (PRV-009): no handle is held, and `reconcile.v1` takes none.
  For a control that took effect, on an instance that exists and is not retired, the request
  names `revision`, and the values the control was sent are stored (REV-010). A
  reconciliation that stores nothing names no `revision`, and one that does reports invalid
  (OPS-031). A request that names an invocation whose outcome is not unknown reports conflict
  (OPS-030). A consumer learns the ID that names the invocation from the
  `io.github.shrav-k.conduit/invocation` of its result, or of a repeat of its request (IFC-025,
  OPS-032, OPS-034), and finds each invocation whose outcome is unknown and that concerns an
  instance in `unreconciled` in its status, which no purge shortens (LIF-013).
  A bound limits reconciliation through `access`. Through a Pool, a holder other than root
  reconciles only invocations it sent. Through a Fleet, it reconciles invocations it sent or
  that the Fleet or an Agent the Fleet owns sent; a Fleet is never a sender in this version
  (FLEET-004). Root is exempt only as the consumer of the authorizing Conduit (ACC-055).
- **Defaults.** No input of a core operation states a `default` (CTR-002, OPS-026). An input
  that may be left out is one that `required` does not list. Each input of the requirement
  `bound.v1` states `default: []`, and may be left out for that reason alone (OBJ-015).
- **`deduplicates`.** Declared where the whole effect is a change to what the engine itself
  stores, which the engine recognizes by the idempotency key, so a repeat after an unknown
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
  `set-up.v1`, `retire.v1`, `inspect.v1`, `unregister.v1`, `open.v1` and `close.v1` take the optional input
  `expectedId`, the ID the actor expects the object it names to have, as Kubernetes's
  `preconditions.uid`: it names nothing and grants nothing, and where the reference resolves to
  an object with another ID, as after a name is given to another object (LIF-035), the request
  reports conflict and changes nothing (LIF-037).
  `register-replacement.v1` names the instance that holds a nested instantiation in
  `instance`, and the name of the declared Conduit or the entry in `nested` (LIF-009, LIF-033).
- **The resolved form of a reference.** Where a value is a reference, the instance stores
  `{kind: <Kind>, id: <ID>}` in its place (OBJ-009, OBJ-021). Every reference schema accepts
  that form, and only the engine writes it. An implementation receives it in its `arguments`,
  in the filters it is sent and in the configuration that every request to it carries
  (IFC-006), and `inspect.v1` shows it under `declared`. An onward request may therefore hold
  it in an input of a core operation, because the engine handed it to the instance that sends
  it; a request that an actor makes on its own behalf may not (LIF-031).
- **Diagnostics.** `register.v1`, `replace.v1`, `register-replacement.v1` and `install.v1`
  declare the output `diagnostics`, for the diagnostics that did not prevent the request. What
  a request that is not dispatched says beyond its reason, it says in `diagnostics` in the
  `structuredContent` of its result, whether or not the operation declares an output of that
  name, and every reason carries there an error diagnostic whose
  `ruleId` is the ID of the promise that decided it (IFC-025). A diagnostic is SARIF 2.1.0's
  `result`, reduced (`diagnostic.v1`). A rejected registration carries them so (OBJ-023), and a
  request to retire a Pool whose Resources are in the way names those Resources so (LIF-020),
  though `retire.v1` declares no output. A request that is not permitted carries ACC-056's refusal explanation. If its access decision cannot be made, it carries one ACC-037 diagnostic naming no object (OPS-023, IFC-050). A request to install that installs
  nothing reports invalid, unavailable or conflict as PKG-023 to PKG-025 and PKG-029 state.
- **Retiring and removing.** A request to retire an instance whose shutdown is not confirmed
  is a further retirement (LIF-026). Once the shutdown of an instance is confirmed it is
  removed, with no request, and its records are kept (LIF-035, LIF-019): a request that names
  it afterwards reports invalid (OPS-029). `unregister.v1` removes a template (LIF-012), or a
  retired instance (LIF-017), and for such an instance asks once more for the stop of its run
  (LIF-036). The Conduits of which a retired instance is the target are closed at once. Those
  of which it is the consumer are closed once its shutdown is confirmed or it is unregistered,
  and until then permit only the requests of its retirement (CON-008, ACC-039).
  `retire-with-resources.v1` retires the Resources in the reverse of the order of setup; where
  one cannot be retired, only those that the order puts after it wait, every other one is
  retired and the Pool is not, so a repeat finishes (LIF-021). It declares no output, because
  a result whose outcome is failed holds none (IFC-025).
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
  `address#name` with the address its manifest states (ACC-029, SUB-008, MAN-015). It is
  compared as a name, so it covers the operation of that name in every submission at that
  address, and in a package installed there once no submission at it is held (PKG-029). The
  engine applies a bound itself and never sends it, as it does the shared-host requirement
  (CTR-005, CON-018). A right to open trusts its consumer with every operation the bound names
  on every target it covers (ACC-030).
- No core operation starts model work (IMP-002), so none carries `startsModelWork`. No
  operation arranges, through the engine, work that outlasts its invocation, and no contract
  carries a mark for it (ACC-019, CTR-002).
- Every management verb is implemented by the engine as a rule: `engine` lists it in
  conduit.yaml, no Operation contract states a default implementation (CTR-002), and no
  template or type selects one for it (TPL-003, TPL-004). The engine performs it; no process is
  started and no request crosses the implementation interface.

## Results and Conduit's keys

A result that the engine gives a consumer has `isError` true where the request ended with a reason
or its outcome is failed or unknown, and in no other case. Any other result holds the declared
outputs in `structuredContent`, which conforms to the tool's `outputSchema`. Where the operation
declares no outputs, the tool has no `outputSchema` and a successful result has no
`structuredContent`, preserving plain content (IFC-025, IFC-033). A result whose outcome is
succeeded holds its outputs even where the Conduit through which its invocation was dispatched was
closed before it ended (ACC-034). A repeat of an invocation that has not ended reports conflict, and
its result holds the engine's ID for the invocation (OPS-032); so does a repeat of one that
succeeded, of an operation that declares outputs, where the engine does not keep its outputs, and
the `ruleId` of the error diagnostic tells the two apart (OPS-034). A caller that lost a result
repeats its request with the same idempotency key (OPS-009). Where a filter applies to a repeat of
an invocation that succeeded, the stored outputs are not given: an operation that states no
`effect`, is not a control and starts no model work is dispatched again as a new attempt, so that
its filters decide afresh; any other reports conflict where it declares outputs (OPS-034), and is
answered from its outcome where it declares none (OPS-009). A repeat that is not dispatched is
judged by no filter. A Conduit applies no filter where the engine implements an operation it lists
(CON-022), and a request for an operation of the core package that carries one reports unavailable
(MAN-012). A request through a Conduit that has been closed reports refused, and one that names the
closed Conduit itself reports invalid (ACC-011, OPS-029). A call that names a tool the engine does
not present is answered as the protocol prescribes for an unknown tool at the revision in use. At
the baseline revision that is no tool result: the engine returns the protocol's error for invalid
parameters, code -32602, whose `data` holds `io.github.shrav-k.conduit/reason` with the value
refused, or invalid where the call names the tool that an open Conduit would give a control that its
target does not have, where the target holds the control's contract (TPL-013). At a revision at
which a tool result answers it, that result carries the reason as any other does (IFC-012, IFC-025).
A request that the protocol has a server reject with an error of its own is answered with that error
and no tool result (IFC-036).

A result's `_meta` holds `io.github.shrav-k.conduit/outcome` where the request has an outcome,
and `io.github.shrav-k.conduit/reason` where it ended with a reason, with
`io.github.shrav-k.conduit/restriction` where a restriction is why; a result that has an
outcome, or that answers a repeat under OPS-032 or OPS-034, holds
`io.github.shrav-k.conduit/invocation`, the ID that `reconcile.v1` takes (IFC-025). No operation
of the core package declares any key of `_meta` as an output, because `_meta` is not part of an
operation's outputs.

Conduit's keys travel as Conduit's extension of the protocol,
`io.github.shrav-k.conduit/implementation-interface`, at its revision `v0`, which the engine
declares on each request to an implementation that carries them and on each provider call to a
provider's implementation item, once the implementation's discovery result lists the extension:
where it does not, the request or the call is not dispatched and reports unavailable (IFC-037).
The engine's own server lists the extension in its discovery result, and a request to the engine
that carries any of Conduit's keys and does not declare it reports invalid (IFC-036). Their
`_meta` keys are those of IFC-006: `instance`, `operation`, `invocation`, `caller`,
`filters`, `config`, which is the instance's configuration, and `revision`, its revision,
with `idempotencyKey` and `conduit` where they apply, each under the prefix
`io.github.shrav-k.conduit/`. Every request the engine sends has a caller. `serving` is not one
of them: it is a key of an onward request that an instance sends to the engine (IFC-020), and
the engine does not send it on. A contract of a package, or of a submitted project, is named on
the wire by its qualified reference, and a submitted project's is `address#name` with the
address its manifest states (SUB-008). A provider call carries the keys that PRV-005 names.

To a run's `url`, and to an item reached by `remote` that is not `plain`, the engine sends
nothing, `server/discover` among it, until the server's side has proved who it is, and it
presents on every HTTP request the proof of an enforcement point: to such a `remote` item, only
at an `https` URL, by mutual TLS or by the protocol's own authorization, which no such item may
also name among its `headers` (IFC-017, IFC-041, IFC-043, PRV-025, PRV-028); such an item
stands at an `https` URL, which `implementation-item.v1` states (IFC-044). The engine's own
address at an `https` URL always accepts a token as the protocol's authorization, whichever
machine the request comes from (IFC-042).

`inspect.v1` returns `id`, `revision`, `packages`, `declared`, `status` and `records`,
plus `inventory` for instances other than Conduits, opening `provenance` for Conduits,
`inventory.scope` for Interfaces and Agents, and `inventory.administrators` for Pools (LIF-013, ACC-044).
The `system` status also holds `trustDomain` (FST-007).
`packages` is a list. A package's item has `address`, `version` and `digest`; the core package's
item has no `digest`; and the item of a submitted project has `address`, the digest that
identifies the contracts of that submission and `version` where its manifest states one
(NAM-009, PKG-005). `declared` shows each reference in its resolved form, `{kind, id}`. For an
open Conduit it shows, under each requirement the Conduit applies, the implementation that
applies it, named by its qualified reference; for a requirement of the core package that is
`github.com/shrav-k/conduit/core#engine` (ACC-038). The status holds the lifecycle state, the
shutdown of a retired instance (LIF-014, LIF-024) and `conditions`, each in the shape of
Kubernetes `metav1.Condition`, `condition.v1` (LIF-011, OBS-002, OBS-003), among them the
condition `Running` of an instance whose item is run (PRV-009), and `unreconciled`, the engine's
ID for each invocation whose outcome is unknown and that was dispatched to the instance or names
it (LIF-013). The status of a node holds the labels last reported for
its machine (PLC-026). `records` is a list of `record.v1`, each a CloudEvent whose `data` holds
the fields of a record (REC-002).

## Providers

A provider is a Resource whose type offers one of the four provider capabilities, and no other
of them (PRV-001): `runtime-provider.v1` runs an instance's implementation item, stops it and
reports its state; `machine-provider.v1` reports the machines, their labels and their states,
and places instances; `package-source.v1` fetches archives for the URL schemes it claims; and
`secret-provider.v1`, which the type of a secret Resource offers, gives the value of the secret
that the Resource's locator names. The user picks one by registering a
Resource of its type. An Agent or a Resource names the runtime provider that runs it in its
standard argument `runtime` (PRV-006); the machine provider and the package-source providers
are owned by `system`, at most one machine provider and one package-source provider for each
scheme (PRV-013). A package-source provider claims the schemes it was made with for its whole
life, because its type lists no control whose `changes` reaches `/schemes` (PRV-016): a
provider that is to claim other schemes is a new instance, which PRV-013 admits as it admits
any. The engine's own runner, its own machine provider and its own `https:` and
`file:` fetching serve where no provider of a kind does (PRV-003); they are part of the engine
and are not selected as an implementation, so `engine` states none of them. A runtime provider
runs an item on the machine that `run.v1` gives in `machine`, or ends the run failed, so the
engine's own runner runs only instances placed on `local` (PRV-019), and an instance that has
an item that is run and names no `runtime` is placed only on the machine the engine runs on,
whose `kubernetes.io/hostname` its label selector then also holds (PLC-007). A runtime provider, the
machine provider and a package-source provider run on the engine's own runner, on `local`
(PRV-002). The engine asks no runtime provider other than its own runner to run a `command` item
whose program is a relative path (PRV-007). Its own runner passes a process the variables
`HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM` and `USER` of the engine's environment or, on an
operating system on which a process needs others in their place, such as Windows, those that
DL16 names, each only where its value does not begin with `()`; the further variables that a
list set on the runner names; and what the item binds or `identity` gives, which wins; and
nothing else of the engine's environment (PRV-023). A runtime provider is given, in `run.v1`'s
`identity`, what the run needs to reach the engine and prove which instance it is, under names
fixed for every engine: the variables `CONDUIT_URL`, the engine's address, and
`CONDUIT_IDENTITY_DIR`, a directory, and in that directory the files `ca.crt` and `jwks.json`,
the trust bundle, `engine-id`, the engine's credential ID, `token`, a token in the form of a
JWT-SVID, and, where the forms of proof of DL13 need them or a gate stands before the run,
`tls.crt` and `tls.key`, a certificate in the form of an X509-SVID and its private key
(PRV-022).
It carries them, and the run's secrets, to the run's machine only over a channel that
authenticates that machine and keeps them confidential, which its contract names, and keeps
them as it keeps a secret's value (PRV-011, PRV-022); the engine's own runner delivers them in
the same way. A credential lasts as long as its instance, and nothing renews it in this version
(DL13). No template selects an item that binds either variable in its `environment` (MAN-018).
The server of every run that a runtime provider makes stands behind its gate, which admits a
request only with the proof of an enforcement point, in this version the engine, and keeps
everything else on the run's machine from the server; the engine sends nothing to the run's
`url`, or to the latest one that `inspect-run.v1` reported, until what answers there has proved
by TLS, with the run's certificate, that it serves the instance (PRV-025, PRV-028, IFC-017).
The engine's own runner reaches its process over stdio, and no gate stands before it.
`stop.v1` and `inspect-run.v1` are given the run's machine as `run.v1` is, so that a provider
that keeps no state of its own finds the run (PRV-027). What a runtime provider does with a
stop that it cannot carry out now, such as carry it out once it can, its contract states, with
where it keeps that stop meanwhile, which neither a restart of the engine nor the provider's
own retirement loses (PRV-021), and the engine asks for the stop once more when an actor
unregisters a retired instance (LIF-036). The servers of the runtime, machine and
package-source providers run on the engine's own runner, so they end when the engine does, or
the part of it that holds them; once it has restarted, or that part has stopped, the engine
runs each again when it first needs it, reading its secrets for the caller of the setup that
last asked for its run, and so it does a secret Resource that its own runner ran, before the
first read through it (PRV-026).
A provider call declares Conduit's extension (PRV-005, IFC-037), so no plain implementation
implements an operation of `runtime-provider.v1`, `machine-provider.v1` or `package-source.v1`
(PKG-020).

A secret Resource's type is its store's own Type, whose schema is the form of the locator, which
lists no control (SEC-005) and which offers no other capability (SEC-022). A place that
`secret-reference.v1` governs holds a reference to a secret Resource and never the secret itself
(SEC-001, SEC-002). The engine reads a secret for an instance before each run it asks for, and
before each connection to a `remote` item whose headers carry one, through that instance's own
open Conduit to the secret Resource, and passes the value in the run's `secrets` or
sends it as the header (SEC-010, SEC-011, SEC-013); it keeps no copy and puts the value nowhere
else (SEC-017, SEC-018). Where the engine has lost a connection's values, as when it restarts,
it reads again each secret Resource that the setup which made the connection read, for that
setup's caller, decided by the instance's own Conduit and its filters as they stand then
(SEC-023). A header that carries a secret, and the value that an implementation of
`get-secret.v1` reached by `remote` returns, cross only to an `https` URL, or to an `http` URL
whose host is a loopback address on an engine that, as configured, can neither restart on
another machine nor run as several parts. A template is rejected whose `remote` item would carry
one anywhere else, or that stands at a loopback host on an engine that can restart on another
machine or run as several parts, whatever its headers; an engine configured so after such a
template was registered sends nothing to the item (SEC-024, SEC-013, IFC-041, DL17).

## Pending actions and account hooks

answer.v1 and sign-in.v1 require an Interface acting on its own behalf (PND-009,
PND-010). Their capability lists do not add them to an Agent's tools. pool-manage has
answer only; fleet-manage has both; view templates and fleet-manage-own have neither.
Root's defaults have both. object-management has answer for subjects inspected there.
Sign-in hooks are required by a Type stating signIn, implemented by its selected client,
and run only as parts for the initiating Interface's caller and filters (CTR-013).

inspect.v1 adds status.pending and list.v1 adds pending for Interfaces at which an action
is raised (PND-002). Codes and fingerprints are never serialized in these views or other
wire messages. The engine shows them in its own panel and records them with the answer.
The URL exchange is IFC-049: integrity-protected requestState, retries that poll, and
URL capability errors -32021. No completion notification drives a hook.

## First start

At first start (FST-001, FST-002) the engine instantiates five documents, which are written out
as an example in
[spec/schemas/examples/valid/first-start](../schemas/examples/valid/first-start/first-start.yaml).
The example shows what first start makes. The engine is handed no such documents, and a
submission that held them would be rejected (FST-006, OBJ-002):

| Object | Kind | `from` | Notes |
| --- | --- | --- | --- |
| `system` | Pool | `github.com/shrav-k/conduit/core#system` | Of the basic Pool type; owns the nodes, `management`, `access`, and the machine provider and package-source providers |
| `local` | Resource | `github.com/shrav-k/conduit/core#node` | Owned by `system`; its own placement; the node that represents the machine the engine runs on, with an empty configuration and the labels reported for that machine (PLC-008, PLC-026) |
| `management` | Resource | `github.com/shrav-k/conduit/core#management` | Owned by `system`, placed on `local` |
| `access` | Resource | `github.com/shrav-k/conduit/core#access` | Owned by `system`, placed on `local` |
| `root` | Interface | `github.com/shrav-k/conduit/core#root` | Declares three Conduits, which are instantiated with it |

The three Conduits of FST-002 are the declared Conduits of the template `root`. Their local names
are `management`, `access` and `system`, so the root Interface's tools are `management.register.v1`,
`access.open.v1` and so on. `root-access` applies no bound, which is what ACC-022 requires.
`management` and `access` are not entries of the `system` Pool's template: an entry's Resource
gets a generated name (OBJ-003), and FST-001 fixes these two names.

The first-start objects are identified by their IDs, and a name is fixed once its object is
made (OBJ-011). No instance document is made from the templates `system`, `node`,
`management`, `access` and `root` (FST-006). The engine alone instantiates `node`: for
`local` at first start, and for every other machine that the machine provider reports and no
node represents, naming the node from the machine's hostname (PLC-026). A node
offers no capability and lists no control, so nothing changes it once it is made apart from its
lifecycle state and the labels reported for it (PLC-008).

## Every export

"Requires" names the promises that need the export to exist. No row says "none".

| Export | Kind | Requires |
| --- | --- | --- |
| `system` | Pool template | FST-001, FST-005 |
| `node` | Resource template | FST-001, FST-005, PLC-008, PLC-026 |
| `management` | Resource template | FST-001, FST-005 |
| `access` | Resource template | FST-001, FST-005 |
| `root` | Interface template | FST-001, FST-002, FST-005 |
| `root-management` | Conduit template | FST-002, FST-005, ACC-032 |
| `root-access` | Conduit template | FST-002, FST-005, ACC-022 |
| `interface.v1` | Type | TPL-002: the template `root` needs a type; the basic Interface type |
| `pool.v1` | Type | TPL-002: a Pool template needs a type; the basic Pool type, and the type of `system`; CTR-004 |
| `node.v1` | Type | PLC-008, PLC-026 |
| `management.v1` | Type | TPL-002, ACC-033 |
| `access.v1` | Type | TPL-002, ACC-006 |
| `object-management.v1` | Capability | OPS-019, ACC-033 |
| `access-control.v1` | Capability | OPS-019, ACC-006, ACC-012, ACC-033 |
| `pool-management.v1` | Capability | OPS-019, ACC-033, ACC-028, CTR-004 |
| `runtime-provider.v1` | Capability | PRV-001, PRV-006, PRV-007 |
| `machine-provider.v1` | Capability | PRV-001, PRV-013, PRV-014, PRV-015 |
| `package-source.v1` | Capability | PRV-001, PRV-016, PRV-017 |
| `secret-provider.v1` | Capability | SEC-002, SEC-005, SEC-022, PRV-001 |
| `register.v1` | Operation | OPS-019, ACC-033, SUB-006 |
| `replace.v1` | Operation | SUB-009, SUB-007 |
| `unregister.v1` | Operation | LIF-012, LIF-017, LIF-036, LIF-037 |
| `inspect.v1` | Operation | LIF-013, ACC-033, LIF-037 |
| `set-up.v1` | Operation | OPS-019, ACC-033, LIF-003, TPL-012, LIF-037 |
| `retire.v1` | Operation | OPS-019, ACC-033, LIF-005, TPL-009, LIF-037 |
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
| `run.v1` | Operation | PRV-007, PRV-011, PRV-022, PRV-025, PRV-028 |
| `stop.v1` | Operation | PRV-008, PRV-021, PRV-027, LIF-036 |
| `inspect-run.v1` | Operation | PRV-009, PRV-027, IFC-017 |
| `list-machines.v1` | Operation | PRV-014 |
| `place.v1` | Operation | PRV-015 |
| `fetch-package.v1` | Operation | PRV-017 |
| `get-secret.v1` | Operation | SEC-005, SEC-006, SEC-010, SEC-023, PRV-026 |
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
| `answer.v1` | Operation | PND-009, ACC-033: answer a setup or notice, as an Interface |
| `sign-in.v1` | Operation | PND-010, PND-011, IFC-049: sign in and check the expected account |
| `sign-in-hook.v1` | Operation | CTR-013: begin login, as a part only |
| `account-hook.v1` | Operation | CTR-013, PND-011: report the client account, as a part only |
| `sign-out-hook.v1` | Operation | CTR-013, PND-011: undo the client sign-in, as a part only |
| `pending-action.v1` | Schema | PND-001 to PND-004: action, evidence, expiry and answer |
| `evidence-item.v1` | Schema | PND-004: kind, value, where and proves |
| `refusal.v1` | Schema | ACC-056: the two disclosure tiers |
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
| `labels.v1` | Schema of a document's part | PLC-007, CTR-004, MAN-010, PRV-014 |
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
| `implementation-item.v1` | Schema of the manifest's part | PKG-019, PKG-020, MAN-009, MAN-010, IFC-043, IFC-044 |
| `server-package.v1` | Schema of the manifest's part | MAN-010, PKG-019 |
| `server-remote.v1` | Schema of the manifest's part | MAN-010, IFC-041 |
| `document.v1` | Schema of a document | OBJ-020, SUB-005 |
| `sectioned.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `sectioned-templates.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `sectioned-interface.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `sectioned-agent.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `sectioned-resource.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `sectioned-pool.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `sectioned-conduit.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `consumes-item.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `sectioned-operator.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `manages.v1` | Schema, sectioned documents | SCT-001, SCT-003 |
| `fleet-management.v1` | Capability | FLEET-001, FLEET-004, ACC-033, OPS-019 |
| `provenance.v1` | Schema | ACC-044, REC-004 |
| `tombstone.v1` | Schema | LIF-040, LIF-041, ACC-053, PLC-024 |
| `inventory.v1` | Schema | LIF-013, ACC-044, LIF-040 |
| `trust-domain.v1` | Schema | FST-007 |
| `list.v1` | Operation | OPS-036, ACC-033, LIF-040 |
| `lift.v1` | Operation | ACC-053, LIF-040 |
| `retire-with-agents.v1` | Operation | LIF-020, LIF-021, REV-014 |
| `root-pool` | Conduit template | FLEET-005, ACC-024 |
| `pool-manage` | Conduit template | LIF-013, ACC-033, ACC-055 |
| `pool-view` | Conduit template | LIF-013, ACC-033 |
| `fleet-manage` | Conduit template | LIF-013, ACC-033, ACC-055 |
| `fleet-manage-own` | Conduit template | LIF-013, ACC-033, ACC-055 |
| `fleet-view` | Conduit template | LIF-013, ACC-033 |
| `retire-pool` | Conduit template | LIF-021 |
| `fleet.v1` | Type | FLEET-001, FLEET-004 |
| `fleet` | Fleet template | FLEET-001 |
| `root-fleet` | Conduit template | FLEET-005, ACC-024 |
| `retire-fleet` | Conduit template | LIF-021 |
| `fleet-reference.v1` | Schema, a reference schema | FLEET-002, OBJ-006, OBJ-021 |
| `engine` | Implementation | TPL-004 (the engine implements every operation it lists, with no selection), TPL-003 (no template or type selects one of them), MAN-012 (it states none of `command`, `package` and `remote` and no `supports`) |

## What this package does not settle

- The full shape of an inspection result. `inspect.v1` fixes the names of its outputs, the
  shape of `packages`, the lifecycle state that every status carries, the shutdown that the
  status of a retired instance carries, that its conditions are `condition.v1` and that its
  records are `record.v1`. That `declared` shows each reference as `{kind, id}`, and names under
  each requirement of a Conduit the implementation that applies it, is settled; the keys that
  hold them are not. Not fixed either: where the result of inspecting a Conduit states, for
  each restriction, enforced or mediated (ACC-027); the key that holds a node's labels in
  its status (PLC-026); where the word unknown stands against a setting (REV-009); how the
  status names a setup and its outcome, what retired an instance and which event closed a
  Conduit (LIF-004, LIF-005, CON-008, CON-011); where an instance's `from` is reported
  (FST-005); and how a gap in the records is shown (REC-006). These are settled with the
  conformance vectors: what a vector compares fixes the shape.
- How a diagnostic names a document or an object beyond the JSON Pointer of its logical
  location, and the `source` and `type` of a record's CloudEvent: an engine names them.
- What the engine puts in `run.v1`'s `identity` beyond the names that
  [spec/names.yaml](../names.yaml) fixes: whether it gives `tls.crt` and `tls.key` to a run on
  its own runner, under the forms of proof the engine offers, the ID by which a credential
  names the instance, and where the keys that sign it are held. They belong to the engine's
  mechanism of proof, DL13 of the [implementation-defined list](../implementation-defined.yaml).

No contract of the core package states `records`, and no core type offers a provider
capability or declares a run of its own: the engine's own runner, machine provider and fetching
are part of the engine (PRV-003). `bound.v1` and `shared-host.v1` are applied by the engine
itself and never sent, so no implementation states either in `supports` (CTR-005, CON-022).
