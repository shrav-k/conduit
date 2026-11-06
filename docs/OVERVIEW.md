# Conduit overview

Not binding. This is a short tour of the language in ordinary words, written from
[spec/README.md](../spec/README.md) and [spec/definitions.md](../spec/definitions.md). It was
last read against the specification on 2026-11-06, as
[ruling 41](proposals/ruling-41/README.md) left it. The
[specification](../spec/README.md) alone binds, and it governs where this page differs. Each
section links the files that state its rules. A code such as `CON-001` is the identifier of
one promise in them.

## What Conduit is

Conduit is a configuration language for the relationships between independent agents and the
things they share.

Languages such as Kubernetes, Docker Compose and Terraform declare workloads, which do what
their configuration says. An agent decides for itself what to do. So Conduit declares what
surrounds the agent: what it may reach, under which limits, and who answers for what it did.

Conduit innovates only on that: connection and access
([ruling 40](proposals/ruling-40/README.md)). Everything else, such as what runs a program,
the machines it runs on, where packages come from and where secrets are kept, is a provider
behind a contract that any existing tool can implement, and the user picks it. See
[Providers](#providers).

An author writes documents. An engine is a running program that implements the specification.
No engine exists yet.

## The five families

Every object is a template or an instance of one of five families
([definitions](../spec/definitions.md#objects-and-documents)).

| Family | What it is |
| --- | --- |
| Interface | Where a person, or a system outside, acts on the system. It is an actor: it makes requests on its own behalf. |
| Agent | An actor, as an Interface is. Its model being run is called model work. |
| Resource | Something that agents share, such as a directory. It is not an actor: it makes a request only to serve one it received. A provider is a Resource too. |
| Pool | The owner of Resources. It is never a consumer or a caller. |
| Conduit | A relationship between exactly one consumer and exactly one target, for exactly one capability. |

## Documents, templates and instances

Rules: [objects](../spec/promises/objects.yaml),
[instantiation](../spec/promises/instantiation.yaml),
[templates](../spec/promises/templates.yaml) and
[submissions](../spec/promises/submissions.yaml).

A document is one template, instance or contract written out, in YAML or the equivalent JSON.

- A **template** has no `from`. Its `spec` holds everything it declares, including its inputs.
  Each input has a schema and names the places in the `spec` where its value is assigned.
- An **instance** is one identified object made from a template. Its document has `from`, which
  names the template, and the standard arguments of its family: `owner` for a Resource,
  `placement` and `runtime` for an Agent and a Resource, and `consumer` and `target` for a
  Conduit. `owner`, `consumer` and `target` are required. `placement` may be left out, and the
  machines provider then places the instance. `runtime` may be left out, and the engine's own
  runner then runs it (OBJ-013). The document may have `arguments`, which supply values for the
  template's inputs. It has no `spec`. Every instance comes from a template.
- A template of any family but Conduit selects a **type**: a contract that states the schema of
  an instance's `config`, the capabilities its instances offer and their controls.

Documents are handed to the engine together, as a submission. The engine resolves them: it
reads them against what already exists and yields either the objects they make or diagnostics.
Resolving runs nothing. If no document is rejected, the engine registers the templates and the
instances and makes the contracts available to the project; otherwise it registers nothing.
Registering an instance document makes the instance and the instances its template nests in
it. Nothing is set up, and nothing is opened (INS-016).

Where a document refers to another object, the instance stores the kind and the ID of the
object that the reference resolved to, written `{kind, id}`. Only the engine writes that form:
a document never does, and nor does a request that an actor makes on its own behalf, which
names an object by a reference (OBJ-009, LIF-031). A name is fixed once its object is made
(OBJ-011), and a stored reference never comes to point at another object, even where the name
is later given to a new one (OBJ-010).

## Pools, ownership and placement

Rules: [pools](../spec/promises/pools.yaml) and [placement](../spec/promises/placement.yaml).

Every Resource has exactly one owner, which is a Pool. The owner is fixed when the Resource is
made, and no operation changes it (POOL-001). Ownership is a stored fact and permits nothing
(ACC-028).

A Pool template may declare entries. When the Pool is made, one Resource is made for each entry
and is owned by that Pool (POOL-002), and the Pool's setup sets them up in the order of the
references among them (POOL-013). A Resource can also be made on its own, by a document that
names as its owner a Pool that is not retired (POOL-004).

An Agent and a Resource each have exactly one placement: the location where it runs, fixed for
its whole life (PLC-001, PLC-009). Interfaces, Pools and Conduits have none. A location is a
Resource owned by the Pool `system` that stands for one machine, and the engine makes them all:
`local` for its own machine at first start, and one for each other machine that the machines
provider reports (PLC-026). What is known of a machine is its labels, such as
`kubernetes.io/os: linux`, which the provider reports and nothing else sets (PRV-014). Where a
document gives no placement, the machines provider chooses a machine; with none, everything is
placed on `local` (PLC-002, PRV-015). A type or an implementation may state a label selector
under `placement`, which the location's labels must match (PLC-007). An instance runs on the
machine of its location: its runtime provider runs it there, or the run fails and nothing runs
(PRV-019). The engine's own runner runs only on the engine's machine, so an instance placed on
another machine names a runtime provider that runs there (PRV-003, PRV-006).

A machine reported unreachable keeps its instances, and requests to them report unavailable
(PLC-023). A machine reported gone retires its location and every instance on it, as if `root`
had declared the location lost (PLC-024).

## Conduits and access

Rules: [conduits](../spec/promises/conduits.yaml), the
[access convention](../spec/conventions/access.yaml) and
[first start](../spec/promises/first-start.yaml).

A Conduit joins one consumer to one target for one capability (CON-001). The consumer makes
requests through it. The target receives them. A capability is a list of operations. A type
lists the capabilities its instances offer, and a Conduit names one that its target's type
offers. Offering is not permission.

**Every permission is an open Conduit.** While a Conduit is open it permits exactly the
operations it lists, on its target (ACC-003). A request for an operation that no open Conduit
from that consumer to that target lists is refused (ACC-001). There is no separate object for a
permission.

A Conduit comes to exist in one of two ways. A template of an Interface, an Agent or a Resource
can declare it; one Conduit is then made with each instance of that template (CON-002), and
opening it is a part of that instance's setup (CON-004). A template that needs two
capabilities of one target declares two Conduits. Or a Conduit document names an existing
instance as its consumer and another as its target (CON-012).

A request can be limited by restrictions, and a restriction only narrows (ACC-008). A filter is
a requirement that a Conduit's template applies, and it narrows what may pass through that
Conduit. A bound limits a right to open Conduits, as described below. To narrow what an agent
may do, its Conduit is closed and a narrower one is opened (ACC-011).

The engine applies the two requirements of the core package itself, a bound and the shared-host
requirement, and never sends them. Every other requirement is a filter: the engine sends it
with each request to the implementation of the target, which applies it (IFC-018, IFC-019). So
that implementation must state the filter's requirement under `supports` in its manifest, or
the Conduit is not made, and one marked `plain` supports none (CON-022, PKG-020). A filter is
never dropped: a request that carries one which the implementation does not support reports
unavailable (ACC-015).

Inspecting a Conduit reports, for each restriction, whether it is enforced or only mediated
(ACC-027). Mediated means that the restriction applies to requests that pass through the
engine. Enforced means that the runtime provider that runs the consumer also keeps it from
going around the engine, because that provider's contract says it confines what it runs. In
this version every restriction is reported as mediated.

**The right to open Conduits is itself an open Conduit.** Opening and closing are operations of
a Resource named `access`. An actor may open a Conduit only if it is the consumer of an open
Conduit to `access` that lists the opening operation (ACC-006). That Conduit can carry a bound,
which names the Pools, targets and operations it covers. One that carries no bound is
unlimited (ACC-029). A right to open is passed on only as it is or narrower, and since a bound
never limits by filters, whoever holds one may open unfiltered access within it (ACC-030).

Access begins at first start. Five objects then exist: the Pool `system`, the location `local`,
the Resources `management` and `access`, and the Interface `root`, which has an open Conduit to
each of those two Resources (FST-001, FST-002). Any other Interface has no access until a
Conduit of which it is the consumer is opened (ACC-023). These objects are set apart. None of
the five is retired or unregistered (LIF-022), a report that the engine's own machine is
unreachable or gone changes nothing (PLC-023, PLC-024), no request closes a Conduit of which
`root` is
the consumer (ACC-024), and two operations, declaring a location lost and purging records, are
dispatched only when `root` asks (ACC-032). Only `root` opens a Conduit through which the
engine reads a secret for a Resource or an Agent (SEC-008); see [Secrets](#secrets).

Closing a Conduit ends all the access it carried. It is never opened again, and it is removed
at once (ACC-011, LIF-035).

## A worked example

These two documents are quoted from the specification's
[example project](../spec/schemas/examples/valid/project/). The names under `example.org` are
invented. The first is a Conduit template, from `templates.yaml`:

```yaml
# A Conduit template with two requirements: the shared-host requirement, which the engine
# decides when the Conduit is instantiated (PLC-013), and a filter, which the target's
# implementations apply (CON-018). The input `limit` fills an argument of the filter
# (INS-012).
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: directory-read
spec:
  inputs:
    limit:
      schema: {type: integer, minimum: 1}
      to: [/requirements/rate/arguments/limit]
      default: 20
  capability: example.org/files#directory-access.v1
  operations:
    - example.org/files#directory.read.v1
  requirements:
    same-machine:
      requirement: github.com/shrav-k/conduit/core#shared-host.v1
    rate:
      requirement: example.org/files#rate-limit.v1
      arguments:
        limit: 20
```

The second is a Conduit made from it, from `instances.yaml`:

```yaml
# A Conduit instantiated on its own (CON-012). `consumer` and `target` each accept more than
# one kind, so each is written with its kind (OBJ-013).
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: reviewer-scratch
from: directory-read
consumer: {kind: Agent, name: reviewer-1}
target: {kind: Resource, name: scratch}
arguments:
  limit: 10
```

- `directory-read` has no `from`, so it is a template. It names one capability of the package
  `example.org/files` and lists one of its operations.
- It applies two requirements. `shared-host.v1`, from the core package, requires the consumer
  and the target to be placed on the same machine: the engine compares the
  `kubernetes.io/hostname` labels of their two locations, once, when the Conduit is made
  (PLC-013). `rate-limit.v1` is a filter, which the target's implementation applies, and the
  input `limit` fills its argument.
- `reviewer-scratch` has `from`, so it is an instance. Its consumer is the Agent `reviewer-1`,
  its target is the Resource `scratch`, and it supplies 10 for `limit`.
- Registering it makes the Conduit, in the state registered, unless the two are on different
  machines, in which case it is not made. Once the Conduit is opened, `reviewer-1` may ask
  `scratch` for `directory.read.v1` within the rate limit. This Conduit permits nothing else.

## One lifecycle

Rules: the [lifecycle convention](../spec/conventions/lifecycle.yaml) and
[declared against observed state](../spec/conventions/observation.yaml).

Every instance of every family has exactly one lifecycle state, and the state only moves
forward (LIF-001, LIF-006). A template has none.

| State | When an instance has it |
| --- | --- |
| registered | From the moment it is made. |
| ready | Once its setup has succeeded. A Conduit that is ready is open. |
| retired | From the moment a request to retire it is dispatched, whatever the outcome. |

An instance is set up by one invocation of the operation `set-up.v1`, and retired by one of
`retire.v1`. The engine implements both, and carries each out in parts. A setup first opens
the instance's declared Conduits, or for a Pool sets up its entries' Resources; then asks the
runtime provider to run the instance's program, or connects to its remote server; and last
dispatches the setup hook, where the template selects one (TPL-012, TPL-017). Each part begins
only once the required parts before it have succeeded, and a further setup leaves what is
already done as it is. A retirement
dispatches the retire hook, where there is one, and then, whatever came of the hook, asks the
runtime provider to stop the program (TPL-009, TPL-018). For a Conduit the verbs are open and close, in place of set up and
retire, and the operations are `open.v1` and `close.v1`.

Retiring an instance also closes its Conduits. Those of which it is the target are closed at
once. Those of which it is the consumer are closed once its shutdown is confirmed, or at once
where it is declared lost, and until then they permit only the requests of its retirement
(CON-008, ACC-039). An onward request that serves an invocation which came through one of
them is refused from the same moment, unless it belongs to that retirement. Shutdown is the
stopping and releasing of what the retired instance itself holds, and the stop of its run
(LIF-016). It is confirmed once a
retirement of the instance has succeeded (LIF-024). Declaring a location lost, which only
`root` may do, retires the location and every instance placed on it without their shutdown
being confirmed (PLC-017), and a machine reported gone does the same (PLC-024).

Once a retired instance's shutdown is confirmed, it is removed: it no longer exists, its name
is free again, and its records are kept (LIF-035). One whose location was declared lost is
removed by unregistering it (LIF-017).

Whether an instance is running, paused or idle is not a lifecycle state. Such conditions are
reported beside the state, in the shape that Kubernetes gives a condition, and the engine
reports there whether an instance's program is running (LIF-011, PRV-009).

Inspecting an instance returns what was declared, its status and the records about it
(LIF-013). A difference between what is observed and what was declared is reported, and nothing
acts to remove it without an explicit request (OBS-004). A runtime provider may restart a
program that stopped, under its own setting, as systemd and Docker Compose do. That is the
provider's own, and the engine dispatches nothing because of it (PRV-010).

## Operations, outcomes and reasons

Rules: the conventions on [operations](../spec/conventions/operations.yaml),
[revisions](../spec/conventions/revisions.yaml), [records](../spec/conventions/records.yaml) and
[nothing implicit](../spec/conventions/implicit.yaml).

An operation is a callable action with named inputs and outputs, defined by a contract. Every
management verb, such as register, set up, open, close and retire, is itself an operation.

A request asks for one operation on one target. The engine makes its own access decision before
any implementation is invoked (ACC-002). A request that passes the engine's checks is
dispatched, which begins an invocation. The implementation then applies the restrictions it
was sent, and answers refused or unavailable, having performed nothing, where one of them
forbids the request or cannot be applied (IFC-019, IFC-026). Every request ends in one of
seven ways
([definitions](../spec/definitions.md#how-a-request-ends)):

- With an **outcome**: succeeded, failed or unknown. Unknown means it cannot be confirmed that
  the effect occurred or that it did not.
- With a **reason** and no outcome: refused, invalid, conflict or unavailable.

A caller may attach an operation identity to a request so that a repeat is recognized. The
identity is scoped to the caller, the target and the instance that sent the request, and at
most one invocation exists for one of each and one identity (OPS-008). Nothing is dispatched
again unless the caller repeats the request (IMP-007). A repeat of a request whose invocation
succeeded is answered with its outputs, for as long as the engine keeps them; a repeat of one
that has not ended reports conflict, with the engine's ID for the invocation (OPS-009,
OPS-032). A caller that lost a result repeats its request.

What follows an unknown outcome is fixed. A repeat of the request is dispatched as a new
attempt only if the operation's contract declares `deduplicates: true` and the implementation
is not marked `plain` (OPS-012). Where no new attempt is dispatched, the outcome stays unknown
until it is reconciled (OPS-013): an actor records, with what it checked, that the effect
occurred or that it did not, and the outcome becomes succeeded or failed (OPS-014). Reconciling
is itself an operation, `reconcile.v1`, and needs an open Conduit that lists it. For an
invocation of a management verb, that Conduit is to the target that authorized the request
being reconciled. For any other invocation it is to the target that manages the instance the
invocation was dispatched to: the Pool that owns it where it is a Resource, and `management`
otherwise, or where that Pool is retired or the instance no longer exists (ACC-033).

A control is an operation that changes stored settings. It applies exactly the values it is
sent, or fails (REV-007). A request to change an object names the object's current revision,
and one that names another reports conflict (REV-002). While a setup, a retirement or a change
of an instance is in progress, another request for one reports conflict at once (REV-014).

Model work starts only as the effect of an operation whose contract is marked as starting it,
dispatched through an open Conduit that lists it (IMP-001). Setup, retirement and hooks never
start it (IMP-002).

Not every invocation begins with a consumer's request: the engine dispatches the hooks of a
setup or a retirement, and reads secrets, for the caller of that setup or retirement (TPL-017,
SEC-011). Work that outlasts its invocation is not part of this version (ruling 38): once an
invocation has ended, no request is dispatched on its behalf (ACC-019). A request is cancelled
by the protocol's own means (OPS-027). Every access decision, every invocation and every
provider call is recorded (REC-001), as a CloudEvent (REC-002).

## Packages, contracts and the core package

Rules: [contracts](../spec/promises/contracts.yaml), [naming](../spec/promises/naming.yaml),
[packages](../spec/promises/packages.yaml), the [manifest](../spec/promises/manifest.yaml), the
[implementation interface](../spec/interface/implementation.yaml) and the
[core package](../spec/core/README.md).

A contract is a document that states what something is. There are five kinds: `Type`,
`Capability`, `Operation`, `Requirement` and `Schema`. Its name ends in its compatibility
version, such as `.v1`. An Operation contract is written as the Model Context Protocol writes a
tool: its inputs and outputs are the properties of its `inputSchema` and `outputSchema`
(CTR-002).

A package is what an author ships: a manifest and the things it exports, which are only its own
documents. It is identified by an address, such as `example.org/files`, and each release has a
version. A project's manifest lists each package it uses with exactly one version and that
version's digest, and may give a `fetch` URL for its archive. Listing a package does not
install it: a package is made available only by `install.v1`, an operation of `management`, and
registering a project whose manifest lists a package that is not installed is rejected, naming
each one (PKG-006). Every engine fetches `https:` and `file:` URLs, and a package-source
provider can add other schemes (NAM-012, PRV-017). A document reaches an export by a qualified
reference: the address, then `#`, then the export's name. A reference never carries a version.
A project that holds its own contracts or implementations states an address in its manifest
too, and outside the project they are named by it, as a package's are (MAN-015, SUB-008).

An implementation does the work behind an operation. Its item in the manifest reaches it in
exactly one of three ways: a command; a package as the MCP Registry's server.json describes
one, pinned by its digest; or a remote server at a URL (MAN-009, MAN-010). The item can take
variables, arguments or headers from the instance's settings. An existing server that was not
written for Conduit can be one; its item marks it `plain`. An operation built from other
operations is an ordinary implementation: a server that makes onward requests.

The shared wire between an engine and an implementation, and between an engine and a consumer,
is the Model Context Protocol. An operation is one tool, and the engine presents to each
consumer, as tools, the operations its open Conduits list (IFC-012). Conduit's own keys travel
in `_meta` as an extension of the protocol (IFC-037), and an implementation that carries them
receives its instance's settings with every request (IFC-006).

The core package is built into every engine. It exports the templates that first start
instantiates, the contracts of the management verbs, of the hooks and of the provider
capabilities, and the schemas of documents. It needs no item of `packages` in any manifest and
cannot be replaced.

## Providers

Rules: [providers](../spec/promises/providers.yaml).

A provider is a Resource whose type offers one of four capabilities of the core package
(PRV-001):

| Provider | What it does | Where the user picks none |
| --- | --- | --- |
| Runtime provider | Runs an instance's program on the machine of its location, stops it and reports whether it is running, as a container runtime does | The engine's own runner starts the program as a process on the engine's machine, and runs nothing placed on another machine |
| Machines provider | Reports the machines, their labels and whether each is reachable, unreachable or gone, and chooses a machine for an instance | The engine's own machine is the only one, and everything is placed on `local` |
| Package-source provider | Fetches a package's archive for the URL schemes it claims | The engine fetches `https:` and `file:` URLs itself |
| Secret Resource | Reads one secret from a store outside the engine | Nothing: no store is built into the engine |

The user picks a provider by registering a Resource of its type, configured as that provider
needs, such as a Docker context (PRV-001). An Agent or a Resource names the runtime provider
that runs it in its standard argument `runtime`, as a Kubernetes Pod names its runtime class,
and it is fixed for life (PRV-006). The machines provider and the package-source providers are
owned by `system`: one machines provider at a time, and one package-source provider for each
scheme (PRV-013). A runtime provider is not retired while an instance names it (PRV-020). A
user who picks none gets the engine's own (PRV-003).

The engine calls a provider itself. A provider call is not a request: no Conduit permits it, no
Conduit may list its operations, and it is recorded (PRV-004, PRV-005). A program's run is named
by the instance's ID, so one run serves one instance (PRV-007). Which runtime provider ran an
instance, or on which machine, never establishes who sent a request: only what the request
presents does, by a cryptographic proof wherever it crosses machines (PRV-018, IFC-022).

## Secrets

Rules: [secrets](../spec/promises/secrets.yaml). What the guarantee covers, and what it does
not, is stated in [spec/README.md](../spec/README.md#what-the-guarantee-for-secrets-covers).

A document never holds a secret. A place that the core schema `secret-reference.v1` governs
holds a reference to a secret Resource, and a literal there is rejected (SEC-001, SEC-002). A
secret Resource's settings are a locator, which says where its store keeps the value. Each kind
of store is its own type, which gives the form of its locator, and has no control, so nothing
changes a locator once it is written (SEC-005).

Only the engine reads a secret, and only just before it asks a runtime provider to run a
program that needs it, or connects to a remote server that needs it in a header (SEC-010,
SEC-013). It reads through the instance's own Conduit to the secret Resource, and only `root`
may open such a Conduit, for a Resource or an Agent (SEC-008, SEC-009). No instance may ask for
a secret itself: it is presented no tool for it, and a request for it is refused (SEC-007). A
read belongs to the setup or the retirement that asked for the run, and has its caller
(SEC-011). It is recorded without its value; the engine keeps no copy, and passes the value
only to the run, whose runtime provider sets the variable that the implementation item names
(SEC-015, SEC-017, SEC-018, PRV-011). The value is fixed for the life of that run (SEC-014). A
read that fails stops the run, and requests to the instance report unavailable (SEC-012).

One contract serves every store. Five are foreseen, each a package that implements it
([section 4 of the principles review](proposals/principles-review/README.md#4-secrets-reduced)):
Bitwarden (`bws secret get`), 1Password (`op read`), the macOS keychain
(`security find-generic-password`, which fails while the keychain is locked), AWS Secrets
Manager (`GetSecretValue`), and a file store in the standard library beside the reference
engine. A store that needs a token of its own, as Bitwarden and 1Password do, has that token
read as a secret too. Each store's package states in its contract the form of its locator,
versions and caching in its own form, how it reaches its store, and where it must run.

## Three layers, and what is outside them

The specification has three layers ([spec/README.md](../spec/README.md#what-this-is)), and every
promise says which one it belongs to.

| Layer | What it determines |
| --- | --- |
| `language` | What a set of documents means: which objects they make, how those relate, and what is valid. |
| `convention` | The few behaviors every part must agree on, stated as invariants. There are seven: operation identity and outcomes; revision-checked change; access; declared against observed state; ownership and lifecycle; records; and that nothing starts model work or repeats an effect implicitly. |
| `interface` | The implementation interface: the Model Context Protocol as the shared wire, with the keys Conduit adds to it. |

The provider contracts are written in the same three layers. Two things are deliberately
outside them, and neither binds other engines:

- **How one type behaves**, such as the conditions it defines or a directory's edge cases. That
  belongs to the type's contract.
- **How the work is done beyond what the promises fix.** How a program is started, or where an
  instance is placed, belongs to the provider that does it, and its contract says. What stays
  the engine's own, such as retries and how it stores its records, belongs to the engine's
  runtime profile: its documented choice for every item of the
  [implementation-defined list](../spec/implementation-defined.yaml).

A claim of conformance names a version of the specification and one of three levels
([conformance](../spec/README.md#conformance)). An engine claims the Language level, which
covers the layer `language`, or the Engine level, which adds the other two layers and a
runtime profile. An implementation, which is not an engine, claims the Implementation level:
every promise of the layer `interface` whose obligation falls on the implementation.

For everything that is not connection and access, such as schemas, packages, the wire and what
the providers do, the specification adopts existing standards
([schema profile](../spec/schema-profile.md)).

No promise is yet tested by a conformance vector, and there is no reference model. The
[plan audit](AUDIT.md#where-things-stand) tracks the state of the work.

## One user journey

Older documents link to this heading. The journey it described was written before the
specification and no longer holds. See the [worked example](#a-worked-example) above and the
[example project](../spec/schemas/examples/valid/project/).
