# Conduit overview

Not binding. This is a short tour of the language in ordinary words, written from
[spec/README.md](../spec/README.md) and [spec/definitions.md](../spec/definitions.md). It was
last read against the specification on 2026-11-07, as
[ruling 43](proposals/ruling-43/README.md) and the fixes of
[round 5](proposals/reaudit/round-5/DECISIONS.md) of the re-audit left it: with the names that
[ruling 42](proposals/ruling-42/README.md) accepted from the vocabulary audit (among them node,
machine provider, `nodeSelector`, idempotency key, source and conformance document) and its
editorial fixes, and with ruling 43's answer on work across machines, which
[its own section](#work-across-machines) tells. The
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
| Agent | An actor, as an Interface is. A turn of its model is called model work. |
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
  Conduit. `owner`, `consumer` and `target` are required. `placement` may be left out: the
  machine provider then places an instance that has a program to run, and one that has none is
  placed on `local` (PLC-002). `runtime` may be left out, and the engine's own
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
names as its owner a Pool that is not retired (POOL-004). Retiring a Pool with its Resources
retires them in the reverse order. One that cannot be retired, such as one on a machine that
cannot be reached, holds back only the Resources that the order puts after it, every other is
retired, and the Pool is retired only once all of them are, so the same request made again
finishes the work (LIF-021).

An Agent and a Resource each have exactly one placement: the node where it runs, fixed for
its whole life (PLC-001, PLC-009). Interfaces, Pools and Conduits have none. A node is a
Resource owned by the Pool `system` that represents one machine, and the engine makes them all:
`local` for the machine it runs on at first start, and one for each other machine that the machine
provider reports (PLC-026). What is known of a machine is its labels, such as
`kubernetes.io/os: linux`, which the provider reports and nothing else sets (PRV-014). Where a
document gives no placement, the machine provider chooses a machine for an instance that has a
program to run; an instance that has none, and every instance where no machine provider exists,
is placed on `local` (PLC-002, PRV-015). A type or an implementation may state a label selector
under `nodeSelector`, which the node's labels must match (PLC-007). An instance runs on the
machine of its node: its runtime provider runs it there, or the run fails and nothing runs
(PRV-019). The engine's own runner runs only on the engine's machine, so an instance placed on
another machine names a runtime provider that runs there (PRV-003, PRV-006). An instance that
has a program to run and names no runtime provider is placed only where `local`'s hostname
label matches: that label joins its selector, and the machine provider, asked with it, names
the engine's machine or none (PLC-007, PRV-015). Where the machine provider is not ready, or
cannot place an instance now, the request that would make the instance reports unavailable
(PRV-015).

A machine reported unreachable, or left out of the provider's latest report, keeps its
instances, and requests to them report unavailable (PLC-023). The provider reports a machine
gone only where it knows that the machine no longer exists (PRV-014), and the engine then
declares its node lost, which retires the node and every instance on it, as when
`root` declares a node lost (PLC-024).
A node belongs to its machine's hostname and not to the provider that reports it, so
replacing the machine provider retires no node (PLC-026).

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
this version every restriction is reported as mediated. The gate described under
[work across machines](#work-across-machines) does not change that: it keeps other callers
away from a target's server, but does not confine the consumer.

**The right to open Conduits is itself an open Conduit.** Opening and closing are operations of
a Resource named `access`. An actor may open a Conduit only if it is the consumer of an open
Conduit to `access` that lists the opening operation (ACC-006). That Conduit can carry a bound,
which names the Pools, targets and operations it covers. One that carries no bound is
unlimited (ACC-029). A right to open is passed on only as it is or narrower, and since a bound
never limits by filters, whoever holds one may open unfiltered access within it (ACC-030).

Access begins at first start. Five objects then exist: the Pool `system`, the node `local`,
the Resources `management` and `access`, and the Interface `root`, which has an open Conduit to
each of those two Resources (FST-001, FST-002). Any other Interface has no access until a
Conduit of which it is the consumer is opened (ACC-023). These objects are set apart. None of
the five is retired or unregistered (LIF-022), a report that the machine the engine runs on is
unreachable or gone changes nothing (PLC-023, PLC-024), no request closes a Conduit of which
`root` is
the consumer (ACC-024), and two operations, declaring a node lost and purging records, are
dispatched only when `root` asks (ACC-032). Only `root` opens a Conduit through which the
engine reads a secret for a Resource or an Agent (SEC-008); see [Secrets](#secrets).

Closing a Conduit ends all the access it carried. It is never opened again, and it is removed
at once (ACC-011, LIF-035). A request through it reports refused, as one that no Conduit ever
permitted does (ACC-011).

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
  `kubernetes.io/hostname` labels of their two nodes, once, when the Conduit is made
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
retirement of the instance has succeeded (LIF-024). Declaring a node lost retires the node and
every instance placed on it without their shutdown being confirmed: only `root` may ask for it
(PLC-017), and the engine does it where the node's machine is reported gone (PLC-024).

Once a retired instance's shutdown is confirmed, it is removed: it no longer exists, its name
is free again, and its records are kept (LIF-035). One whose shutdown is not confirmed, as where
its node was declared lost or its retire hook keeps failing, may be removed by unregistering it,
which asks its runtime provider once more to stop its program (LIF-017, LIF-036). What the
provider does with a stop that it cannot carry out then, such as carry it out once the machine
comes back, is what its contract states, and it keeps that stop where neither a restart of the
engine nor its own retirement loses it (PRV-021).

Whether an instance is running, paused or idle is not a lifecycle state. Such conditions are
reported beside the state, in the shape that Kubernetes gives a condition, and the engine
reports there whether an instance's program is running (LIF-011, PRV-009).

Inspecting an instance returns what was declared, its status and the records about it
(LIF-013). A difference between what is observed and what was declared is reported, and nothing
acts to remove it without an explicit request (OBS-004). A runtime provider may restart a
program that stopped, under its own restart policy, as systemd and Docker Compose do. That is
the provider's own, and the engine dispatches nothing because of it (PRV-010).

## Operations, outcomes and reasons

Rules: the conventions on [operations](../spec/conventions/operations.yaml),
[revisions](../spec/conventions/revisions.yaml), [records](../spec/conventions/records.yaml) and
[nothing implicit](../spec/conventions/implicit.yaml).

An operation is a callable action with named inputs and outputs, defined by a contract. Every
management verb, such as register, set up, open, close and retire, is itself an operation.

A request asks for one operation on one target. The engine makes its own access decision before
any implementation is invoked (ACC-002). A request that passes the engine's checks is
dispatched, which begins an invocation. The implementation then applies the filters it
was sent, and answers refused or unavailable, having performed nothing, where one of them
forbids the request or cannot be applied (IFC-019, IFC-026). Every request ends in one of
seven ways
([definitions](../spec/definitions.md#how-a-request-ends)):

- With an **outcome**: succeeded, failed or unknown. Unknown means it cannot be confirmed that
  the effect occurred or that it did not.
- With a **reason** and no outcome: refused, invalid, conflict or unavailable.

A caller may attach an idempotency key to a request so that a repeat is recognized. The
key is scoped to the caller, the target and the instance that sent the request, and at most one
invocation exists for one of each and one key (OPS-008). Nothing is dispatched
again unless the caller repeats the request (IMP-007). A repeat of a request whose invocation
succeeded is answered with its outputs for as long as the engine keeps them, where every filter
that applies to the repeat was sent with the invocation, and otherwise reports conflict
(OPS-009, OPS-034); a repeat of one that has not ended reports conflict too (OPS-032). Each
conflict carries the engine's ID for the invocation, and its diagnostic names the promise that
tells the two apart. A caller that lost a result repeats its request.

What follows an unknown outcome is fixed. A repeat of the request is dispatched as a new
attempt only if the operation's contract declares `deduplicates: true` and the implementation
is not marked `plain` (OPS-012). Where no new attempt is dispatched, the outcome stays unknown
until it is reconciled (OPS-013): an actor records, with what it checked, that the effect
occurred or that it did not, and the outcome becomes succeeded or failed (OPS-014). Reconciling
is itself an operation, `reconcile.v1`, and needs an open Conduit that lists it. For an
invocation of a management verb, that Conduit is to the target that authorized the request
being reconciled. For any other invocation it is to the target that manages the instance the
invocation was dispatched to: the Pool that owns it where it is a Resource, and `management`
otherwise, or where that Pool is retired or the instance no longer exists (ACC-033). An actor
finds each invocation whose outcome is unknown and that concerns an instance in `unreconciled`
in the instance's status, even after the records are purged (LIF-013).

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
invocation has ended, no request is dispatched on its behalf (ACC-019). There are two
exceptions, both reads of a secret after the engine restarts: what a remote server's headers
need, read again for the caller of the setup that made the connection (SEC-023), and what a
provider's own server needs, read before the engine runs that provider again, for the caller of
the setup that last ran it (PRV-026); see [Secrets](#secrets). A request is cancelled by the
protocol's own means (OPS-027). Every access decision, every invocation and every
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
one, pinned by the SHA-256 of its file in `fileSha256`, or an OCI image by the digest in its
identifier; or a remote server at a URL (MAN-009, MAN-010). The item can take variables,
arguments or headers from the instance's settings. An existing server that was not
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
| Runtime provider | Runs an instance's program on the machine of its node, behind a gate, stops it and reports whether it is running, as a container runtime does | The engine's own runner starts the program as a process on the engine's machine, talks to it over stdio, and runs nothing placed on another machine |
| Machine provider | Reports the machines, their labels and whether each is reachable, unreachable or gone, and chooses a machine for an instance | The engine's own machine provider reports only the machine the engine runs on, and everything is placed on `local` |
| Package-source provider | Fetches a package's archive for the URL schemes it claims | The engine fetches `https:` and `file:` URLs itself |
| Secret Resource | Reads one secret from a store outside the engine | Nothing: no store is built into the engine |

The user picks a provider by registering a Resource of its type, configured as that provider
needs, such as a Docker context (PRV-001). An Agent or a Resource names the runtime provider
that runs it in its standard argument `runtime`, as a Kubernetes Pod names its runtime class,
and it is fixed for life (PRV-006). The machine provider and the package-source providers are
owned by `system`: one machine provider at a time, and one package-source provider for each
scheme (PRV-013). A runtime provider is not retired while an instance names it (PRV-020). A
user who picks none gets the engine's own (PRV-003). The servers of the runtime, machine and
package-source providers run on the engine's own runner, on the engine's machine (PRV-002), so
they stop when the engine stops, or the part of it that runs them; after that, the engine runs
each again when it first needs to call it, reading its secrets for the caller of the setup that
last ran it (PRV-026).

The engine's own runner passes a program it starts, of the engine's own environment, only
`HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM` and `USER`, as the stdio clients of the Model
Context Protocol's SDKs do, and the further variables that a list the user sets on the runner
names, as systemd's `PassEnvironment=` does. So a token exported in the shell that started the
engine reaches no process that the runner starts unless that list names it (PRV-023). Only the
engine's own runner runs a program given by a relative path, which names a file on the
engine's machine (PRV-007, MAN-010).

The engine calls a provider itself. A provider call is not a request: no Conduit permits it, no
Conduit may list its operations, and it is recorded (PRV-004, PRV-005). It declares Conduit's
extension, and no server marked `plain`, and none reached at a URL, implements a runtime
provider, a machine provider or a package-source provider (PRV-005, PKG-020, PKG-030). A
program's run is named by the instance's ID, so one run serves one instance (PRV-007). Which
runtime provider ran an instance, or on which machine, never establishes who sent a request:
only what the request presents does, by a cryptographic proof wherever it crosses machines
(PRV-018, IFC-022). What a run presents, and the engine's address, are given to it in
`identity`, which the runtime provider keeps as it keeps a secret (PRV-022);
[work across machines](#work-across-machines) says more.

## Secrets

Rules: [secrets](../spec/promises/secrets.yaml). What the guarantee covers, and what it does
not, is stated in [spec/README.md](../spec/README.md#what-the-guarantee-for-secrets-covers).

A document never holds a secret. A place that the core schema `secret-reference.v1` governs
holds a reference to a secret Resource, and a literal there is rejected (SEC-001, SEC-002). A
secret Resource's settings are a locator, which says where its store keeps the value. Each kind
of store is its own type, which gives the form of its locator, and has no control, so nothing
changes a locator once it is written (SEC-005). That type offers nothing but the reading of a
secret: a store package that lists or writes secrets does so through another type (SEC-022).

Only the engine reads a secret, and only just before it asks a runtime provider to run a
program that needs it, or connects to a remote server that needs it in a header (SEC-010,
SEC-013). It reads through the instance's own Conduit to the secret Resource, and only `root`
may open such a Conduit, for a Resource or an Agent (SEC-008, SEC-009). No instance may ask for
a secret itself: it is presented no tool for it, and a request for it is refused (SEC-007). A
read belongs to the setup or the retirement that asked for the run or the connection, and has
its caller (SEC-011). It is recorded without its value; the engine keeps no copy, and passes
the value only to the run, whose runtime provider sets the variable or the argument that the
implementation item names, or sends it in the header (SEC-015, SEC-017, SEC-018, PRV-011). The
value is fixed for the life of that run, so a value rotated in its store arrives only in a
later run (SEC-014). A read that fails stops the run, and requests to the instance report
unavailable (SEC-012).

A header that carries a secret goes, and the server of a secret Resource reached at a URL
stands, only at an `https` URL, or at an `http` URL on a loopback address, such as `localhost`;
and on an engine that can move to another machine or run as several parts, no server reached at
a URL stands at a loopback address at all. A template whose item would break this is rejected
when it is registered (SEC-024).

After the engine restarts, on its machine or on another, or the part of it that holds a
connection to a remote server stops, it no longer holds the header values of that connection.
Before it next sends a request on that connection it reads them again, for the caller of the
setup that made the connection, as the instance's Conduit to the secret Resource decides then,
so closing that Conduit stops the read (SEC-023). It reads a provider's secrets in the same way
before it runs that provider again (PRV-026).

The server of a secret Resource that a runtime provider other than the engine's own runner
runs, or that is reached at a URL and takes Conduit's keys, stands behind a gate like any other
server (see [work across machines](#work-across-machines)), so it admits only requests that
carry the engine's proof, and the value crosses to the engine confidential and unaltered
(PRV-025, IFC-017).

One contract serves every store. Five are foreseen, each a package that implements it
([section 4 of the principles review](proposals/principles-review/README.md#4-secrets-reduced)):
Bitwarden (`bws secret get`), 1Password (`op read`), the macOS keychain
(`security find-generic-password`, which fails while the keychain is locked), AWS Secrets
Manager (`GetSecretValue`), and a file store in the standard library beside the reference
engine. A store that needs a token of its own, as Bitwarden and 1Password do, has that token
read as a secret too. Each store's package states in its contract the form of its locator,
versions and caching in its own form, how it reaches its store, and where it must run.

## Work across machines

Rules: [providers](../spec/promises/providers.yaml), the
[implementation interface](../spec/interface/implementation.yaml),
[secrets](../spec/promises/secrets.yaml) and item DL17 of the
[implementation-defined list](../spec/implementation-defined.yaml).
[Ruling 43](proposals/ruling-43/README.md) decided this part, from the
[proposal on work across machines](proposals/multi-machine/README.md).

**One engine decides.** In this version the engine decides every request, wherever the
instances run, and every request passes through it on its way to a server (ACC-002, PRV-025).
The specification names the two roles as NIST does: the engine is the only decision point,
which decides, and the only enforcement point, which lets a request through only once it is
allowed ([definitions](../spec/definitions.md#access)). The engine itself may run as several
parts, or restart on another machine, where its conformance document says so. A promise then
binds the engine as a whole, and DL17 lists what such an engine must still guarantee: every
decision reads the state as it stands, at most one part acts on any one invocation, runs keep
reaching it at the same address, and it keeps its hostname, so `local` still means one machine
(DL17, PLC-026).

**Who a request comes from.** Every run is given, in `identity`, the engine's address, a
credential that names its instance by its ID, the root certificates with which it checks that
it has reached the real engine, and, for its gate, the keys that check the engine's token and
the ID that the engine's proof names (PRV-022). The names of those variables and files are the
same under every engine, fixed in [names.yaml](../spec/names.yaml), so that a harness or a gate
written once works anywhere. A request that crosses machines carries a cryptographic proof:
under mutual TLS a client certificate, or a signed token in the `Authorization` header, which
is the Model Context Protocol's own authorization (IFC-022, DL13). The machine or the network a
request came from proves nothing (PRV-018). In this version a credential lasts as long as its
instance and nothing renews it. Once the instance is removed the credential opens nothing,
because the engine checks on every request that the instance still exists (DL13, IFC-032).

**The gate.** A server believes what a request says about who is asking and under which
limits. So every server that the engine does not run itself, a plain one reached at a URL apart,
stands behind a gate: the server of a run that any runtime provider but the engine's own runner
makes, and a remote server that takes Conduit's keys (PRV-025). The gate lets a request through
only with proof that the engine sent it, in a form that the server cannot replay to another, and
nothing else on that machine reaches the server, apart from root and the operating-system user
or group that the provider uses there. Before it sends anything, the engine checks that the
server answering is the one it asked for, and the request and its result cross confidential and
unaltered (IFC-017). Conduit ships no gate. The runtime provider builds one from existing tools
and its contract names them: a mutual TLS proxy such as ghostunnel, a service mesh, a proxy that
checks tokens, a bridge to the server's standard input, or `docker run -i` over SSH with the
check at the provider's own end. An address, a VPN, a password that is the same for every
request, and loopback that other programs share are not gates. A remote server that takes
Conduit's keys is its own gate: it checks the engine's proof itself and is reached only over
`https`, and one that cannot do so is used as `plain` (IFC-041, IFC-044). So a local server that
takes Conduit's keys runs over stdio on the engine's own runner, and only a plain one is reached
at an `http` URL on loopback, on an engine that can neither move nor run as several parts.

**What needs none of it.** On one machine, where no runtime provider is named, the engine's own
runner starts each program and talks to it over stdio. Nothing else can reach that channel, so
no gate, certificate or proxy is needed, and such a setup is as it was (PRV-003, PRV-025). An
existing MCP server needs no change: marked `plain`, it receives none of Conduit's keys, and at
a URL it stands behind no gate and is reached as before (PKG-020, IFC-041). Only a header that
carries a secret is limited, as [Secrets](#secrets) says. Run by another runtime provider, such
a server sits behind the gate that the provider supplies, and is still unchanged. An MCP client
connects to the engine as to any server: from another machine, with the protocol's own
authorization or a configured `Authorization` header, and no client certificate, which many
clients cannot present (IFC-042). No document names a gate, a credential or an enforcement
point: an Agent placed on another machine is written as one on the engine's machine is, with
its placement and the runtime provider that runs it there.

**What stays open, said plainly.** Programs that run as the engine's own operating-system user
can read its keys, and so can get past every gate. The runs of the engine's own runner usually
run as that user, and runs that share one user can present as one another. An Agent that should
not be trusted that far is given a runtime provider that runs it apart, such as in a container
([what the guarantee for secrets covers](../spec/README.md#what-the-guarantee-for-secrets-covers)).
A program shipped inside a package runs only on the engine's own runner in this version: a
server meant for another machine is published as a package item pinned by its digest, such as
an OCI image (PRV-007, MAN-010). Where a machine is cut off from the engine, requests to the
instances on it report unavailable once it is reported unreachable, and nothing is retired
(PLC-023); work already sent there may run on until its gate notices the cut (OPS-027). How a
later version may let each machine enforce access itself, and keep working for a bounded time
when cut off, is recorded with the
[proposal](proposals/multi-machine/README.md#q4-the-shape-of-v1-recorded-in-a65); none of it is
part of this version.

## Three layers, and what is outside them

The specification has three layers ([spec/README.md](../spec/README.md#what-this-is)), and every
promise says which one it belongs to.

| Layer | What it determines |
| --- | --- |
| `language` | What a set of documents means: which objects they make, how those relate, and what is valid. |
| `convention` | The few behaviors every part must agree on, stated as invariants. There are seven: idempotency keys and outcomes; revision-checked change; access; declared against observed state; ownership and lifecycle; records; and that nothing starts model work or repeats an effect implicitly. |
| `interface` | The implementation interface: the Model Context Protocol as the shared wire, with the keys Conduit adds to it. |

The provider contracts are written in the same three layers. Two things are deliberately
outside them, and neither binds other engines:

- **How one type behaves**, such as the conditions it defines or a directory's edge cases. That
  belongs to the type's contract.
- **How the work is done beyond what the promises fix.** How a program is started, or where an
  instance is placed, belongs to the provider that does it, and its contract says. What stays
  the engine's own, such as retries and how it stores its records, belongs to the engine's
  conformance document: its documented choice for every item of the [implementation-defined
  list](../spec/implementation-defined.yaml).

A claim of conformance names a version of the specification and one of three levels
([conformance](../spec/README.md#conformance)). An engine claims the Language level, which
covers the layer `language`, or the Engine level, which adds what the other two layers ask of
the engine, and a conformance document. An implementation, which is not an engine, a provider's
among them, claims the Implementation level: every promise whose obligation falls on the
implementation, whatever its layer, such as a runtime provider's gate (PRV-025).

For everything that is not connection and access, such as schemas, packages, the wire and what
the providers do, the specification adopts existing standards
([schema profile](../spec/schema-profile.md)).

No promise is yet tested by a conformance vector, and there is no reference model. The
[plan audit](AUDIT.md#where-things-stand) tracks the state of the work.

## One user journey

Older documents link to this heading. The journey it described was written before the
specification and no longer holds. See the [worked example](#a-worked-example) above and the
[example project](../spec/schemas/examples/valid/project/).
