# Conduit overview

Not binding. This is a short tour of the language in ordinary words, written from
[spec/README.md](../spec/README.md) and [spec/definitions.md](../spec/definitions.md) as they
stood on 2026-11-06. The [specification](../spec/README.md) alone binds, and it governs where
this page differs. Each section links the files that state its rules. A code such as `CON-001`
is the identifier of one promise in them.

## What Conduit is

Conduit is a configuration language for the relationships between independent agents and the
things they share.

Languages such as Kubernetes, Docker Compose and Terraform declare workloads, which do what
their configuration says. An agent decides for itself what to do. So Conduit declares what
surrounds the agent: what it may reach, under which limits, and who answers for what it did.

An author writes documents. An engine is a running program that implements the specification.
No engine exists yet.

## The five families

Every object is a template or an instance of one of five families
([definitions](../spec/definitions.md#objects-and-documents)).

| Family | What it is |
| --- | --- |
| Interface | Where a person, or a system outside, acts on the system. It is an actor: it makes requests on its own behalf. |
| Agent | An actor, as an Interface is. Its model being run is called model work. |
| Resource | Something that agents share, such as a directory. It is not an actor: it makes a request only to serve one it received. |
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
  `placement` for an Agent and a Resource, and `consumer` and `target` for a Conduit. `owner`,
  `consumer` and `target` are required. `placement` may be left out, and a rule then chooses
  it. The document may have `arguments`, which supply values for the template's inputs
  (OBJ-013). It has no `spec`. Every instance comes from a template.
- A template of any family but Conduit selects a **type**: a contract that states the schema of
  an instance's `config`, the capabilities its instances offer and their controls.

Documents are handed to the engine together, as a submission. The engine resolves them: it
reads them against what already exists and yields either the objects they make or diagnostics.
Resolving runs nothing. If no document is rejected, the engine registers the templates and the
instances and makes the contracts available to the project; otherwise it registers nothing.
Registering an instance document makes the instance and the instances its template nests in
it. Nothing is set up, and nothing is opened (INS-016).

Where a document refers to another object, the instance stores the kind and the ID of the
object that the reference resolved to, written `{kind, id}`. Only the engine writes that form.
A later rename of that object does not make the reference point at another one (OBJ-009,
OBJ-010).

## Pools, ownership and placement

Rules: [pools](../spec/promises/pools.yaml) and [placement](../spec/promises/placement.yaml).

Every Resource has exactly one owner, which is a Pool. The owner is fixed when the Resource is
made, and no operation changes it (POOL-001). Ownership is a stored fact and permits nothing
(ACC-028).

A Pool template may declare entries. When the Pool is made, one Resource is made for each entry
and is owned by that Pool (POOL-002). A Resource can also be made on its own, by a document
that names a Pool as its owner (POOL-004).

An Agent and a Resource each have exactly one placement: the location where it runs. A location
is itself a Resource, owned by the Pool `system`. Interfaces, Pools and Conduits have none.

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
opening it is a step of that instance's setup (GRA-022, CON-004). Or a Conduit document names
an existing instance as its consumer and another as its target (CON-012).

A request can be limited by restrictions. A filter is carried by one Conduit and narrows what
may pass through it. A policy is not carried by a Conduit: it is applied by a request that
names actors, targets or both. It applies to each request whose
caller is one of those actors, where any are named, and whose target is one of those targets,
where any are named (ACC-031). A restriction only narrows (ACC-008).

Inspecting a Conduit reports, for each restriction, whether it is enforced or only mediated
(ACC-027). Mediated means that the restriction applies to requests that pass through the
engine. Enforced means that an isolation plugin also keeps the consumer from going around it.
This version specifies no isolation plugin. So in this version nothing is enforced: every
restriction is reported as mediated.

**The right to open Conduits is itself an open Conduit.** Opening and closing are operations of
a Resource named `access`. An actor may open a Conduit only if it is the consumer of an open
Conduit to `access` that lists the opening operation (ACC-006). That Conduit can carry a bound,
which names the Pools, targets and operations it covers. One that carries no bound is
unlimited (ACC-029). A right to open is passed on only as it is or narrower (ACC-030).

Access begins at first start. Five objects then exist: the Pool `system`, the location `local`,
the Resources `management` and `access`, and the Interface `root`, which has an open Conduit to
each of those two Resources (FST-001, FST-002). Any other Interface has no access until a
Conduit of which it is the consumer is opened (ACC-023).

Closing a Conduit ends all the access it carried, and it is never opened again (ACC-011,
LIF-006).

## A worked example

These two documents are quoted from the specification's
[example project](../spec/schemas/examples/valid/project/). The names under `example.org` are
invented. The first is a Conduit template, from `templates.yaml`:

```yaml
# A Conduit template with two requirements: the shared-host requirement and a filter. The
# input `limit` fills an argument of the filter (INS-012).
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
- It applies two requirements. `shared-host.v1`, from the core package, requires the consumer's
  location and the target's location to be on the same host. `rate-limit.v1` is a filter, and
  the input `limit` fills its argument.
- `reviewer-scratch` has `from`, so it is an instance. Its consumer is the Agent `reviewer-1`,
  its target is the Resource `scratch`, and it supplies 10 for `limit`.
- Registering it makes the Conduit, in the state registered. Once the Conduit is opened,
  `reviewer-1` may ask `scratch` for `directory.read.v1` within the two requirements. This
  Conduit permits nothing else.

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
`retire.v1`. The engine implements both. A template's `setup` and `retire` graphs are the steps
that the engine runs for the instance within those invocations (TPL-008). For a Conduit the
verbs are open and close, in place of set up and retire, and the operations are `open.v1` and
`close.v1`. A retired instance still exists until it is unregistered, which needs its shutdown
to be confirmed or its location to have been declared lost (LIF-017).

Whether an instance is running, paused or idle is not a lifecycle state. A type may define such
conditions, and they are reported beside the state (LIF-011).

Inspecting an instance returns what was declared, its status and the records about it
(LIF-013). A difference between what is observed and what was declared is reported, and nothing
acts to remove it without an explicit request (OBS-004). There is one exception: an
implementation may restore its own runtime, where its type's contract allows it. To restore a
runtime is to keep alive what is behind the handle that the instance holds. The handle stays
valid, and nothing is dispatched because of it (OBS-005).

## Operations, outcomes and reasons

Rules: the conventions on [operations](../spec/conventions/operations.yaml),
[revisions](../spec/conventions/revisions.yaml), [records](../spec/conventions/records.yaml) and
[nothing implicit](../spec/conventions/implicit.yaml).

An operation is a callable action with named inputs and outputs, defined by a contract. Every
management verb, such as register, set up, open, close and retire, is itself an operation.

A request asks for one operation on one target. The engine makes its own access decision before
any implementation is invoked (ACC-002). A request that passes the engine's checks is
dispatched, which begins an invocation. Every request ends in exactly one of seven ways
([definitions](../spec/definitions.md#how-a-request-ends)):

- With an **outcome**: succeeded, failed or unknown. Unknown means it cannot be confirmed that
  the effect occurred or that it did not.
- With a **reason** and no outcome: refused, invalid, conflict or unavailable.

A caller may attach an operation identity to a request so that a repeat is recognized. The
identity is scoped to the caller, the target and the instance that sent the request, and at
most one invocation exists for one of each and one identity (OPS-008). Nothing is dispatched
again unless the caller repeats the request (IMP-007).

What follows an unknown outcome is fixed. A repeat of the request is dispatched as a new
attempt only if the operation's contract declares `deduplicates: true` and the implementation
is not marked `plain` (OPS-012). Where no new attempt is dispatched, the outcome stays unknown
until it is reconciled (OPS-013): an actor records, with what it checked, that the effect
occurred or that it did not, and the outcome becomes succeeded or failed (OPS-014). Reconciling
is itself an operation, `reconcile.v1`, and needs an open Conduit that lists it. For an
invocation of a management verb, that Conduit is to the target that authorized the request
being reconciled. For any other invocation it is to the target that manages the instance the
invocation was dispatched to: the Pool that owns it where it is a Resource, and `management`
otherwise (ACC-033).

A control is an operation that changes stored settings. A request to change an object names the
object's current revision, and one that names another reports conflict (REV-002).

Model work starts only as the effect of an operation whose contract is marked as starting it,
dispatched through an open Conduit that lists it (IMP-001). Setup never starts it. Every access
decision and every invocation is recorded (REC-001).

## Packages, contracts and the core package

Rules: [contracts](../spec/promises/contracts.yaml), [naming](../spec/promises/naming.yaml),
[packages](../spec/promises/packages.yaml), the
[implementation interface](../spec/interface/implementation.yaml) and the
[core package](../spec/core/README.md).

A contract is a document that states what something is. There are five kinds: `Type`,
`Capability`, `Operation`, `Requirement` and `Schema`. Its name ends in its compatibility
version, such as `.v1`.

A package is what an author ships: a manifest and the things it exports. It is identified by an
address, such as `example.org/files`, and each release has a version. A project's manifest lists
each package it uses with exactly one version and that version's digest. A document reaches an
export by a qualified reference: the address, then `#`, then the export's name. A reference
never carries a version.

An implementation does the work behind an operation. It is reached by a command, by a URL or as
a graph, which is a set of steps ordered only by their dependencies. An existing server that
was not written for Conduit can be one; its implementation item in the manifest marks it
`plain`.

The shared wire between an engine and an implementation, and between an engine and a consumer,
is the Model Context Protocol. An operation is one tool, and the engine presents to each
consumer, as tools, the operations its open Conduits list (IFC-012).

The core package is built into every engine. It exports the templates of the first-start
objects, the contracts of the management verbs and the schemas of documents. It needs no item
of `packages` in any manifest and cannot be replaced.

## Three layers, and what is outside them

The specification has three layers ([spec/README.md](../spec/README.md#what-this-is)), and every
promise says which one it belongs to.

| Layer | What it determines |
| --- | --- |
| `language` | What a set of documents means: which objects they make, how those relate, and what is valid. |
| `convention` | The few behaviors every part must agree on, stated as invariants. There are seven: operation identity and outcomes; revision-checked change; access; declared against observed state; ownership and lifecycle; records; and that nothing starts model work or repeats an effect implicitly. |
| `interface` | The implementation interface: the Model Context Protocol as the shared wire, with the keys Conduit adds to it. |

Two things are deliberately outside it, and neither binds other engines:

- **How one type behaves**, such as the conditions it defines or a directory's edge cases. That
  belongs to the type's contract.
- **How an engine gets from documents to a running system**, such as scheduling, retries and
  process layout. That belongs to the engine's runtime profile: its documented choice for every
  item of the [implementation-defined list](../spec/implementation-defined.yaml).

A claim of conformance names one of three levels
([conformance](../spec/README.md#conformance)). An engine claims the Language level, which
covers the layer `language`, or the Engine level, which adds the other two layers and a
runtime profile. An implementation, which is not an engine, claims the Implementation level:
every promise of the layer `interface` whose obligation falls on the implementation.

For graphs, schemas and packages, which are not what distinguishes Conduit, the specification
adopts existing standards ([schema profile](../spec/schema-profile.md)).

No promise is yet tested by a conformance vector, and there is no reference model. The
[plan audit](AUDIT.md#where-things-stand) tracks the state of the work.

## One user journey

Older documents link to this heading. The journey it described was written before the
specification and no longer holds. See the [worked example](#a-worked-example) above and the
[example project](../spec/schemas/examples/valid/project/).
