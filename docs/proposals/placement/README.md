# Placement in the language — accepted direction

Status: owner rulings of 2026-11-05 for
[plan-audit item A31](../../AUDIT.md#p0--make-the-language-concrete-and-complete) (question Q11).
Field and type spellings were illustrative when this was written, under a
[vocabulary boundary](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#vocabulary-review-boundary) that ruling 16 has
since ended. No runtime exists and none of this has been executed.

This is the record of a ruling. The specification in [`spec/`](../../../spec/README.md) governs
where they differ. Amended by:

- ruling 8: PL2 is settled through instantiation, and placement is a standard argument;
- rulings 17 and 18: the word is "location", and a reference is a kind and a name, with no form;
- ruling 23: every type states what it needs of a location, including nothing;
- rulings 27 to 29: a grant is the access an open Conduit carries; "provisioning" is setting up;
  what a type needs of a location is a requirement on placement; an instance writes its values
  under `arguments`; and "owner-only" operations are operations reserved to the root Interface;
- ruling 30: the default location is designated by a control of the `system` Pool;
- ruling 32: a location's facts are stored configuration, a requirement on placement is a schema
  those facts must satisfy, and the shared-host requirement is checked when a Conduit is opened
  and on each request through it;
- ruling 33: built-in schemas are exports of the core package, whose address comes from the
  repository;
- ruling 37: a control of the core package sets a location's facts.

Promise IDs are as first written; the comment at the head of each spec file says where a removed
ID went.

## Ruling status, 2026-11-05

| Decision | Owner ruling |
| --- | --- |
| PL1 — location is a Resource in the system Pool | Accepted |
| PL2 — where placement is declared | Accepted through the [instantiation rulings](../instantiation/README.md): placement is a standard instantiation argument, passed down by Pool entries through the same shape, with the single-location default and optional designated default. Templates never name a concrete location. |
| PL3 — only Agents and Resources carry placement | Accepted, including its consequence for a system cut off from management |
| PL4 — types declare what they need from a location | Accepted. Declarations are needed in v0, not deferred. |
| PL5 and PL6 — co-location as a requirement on host identity; one execution service is one location | Accepted |
| PL7 and PL8 — fixed placement, replacement to move, no reassignment, and a "declare lost" operation | Accepted |

All decisions are accepted and are written as promises in the specification:
[placement](../../../spec/promises/placement.yaml), PLC-001 to PLC-014 as first written. That
file has changed since: the comment at its head says which promises were folded into others and
which were added. Promises PL-1 to PL-10 below are the earlier wording of the same rules, and the
PLC wording governs.

## The gap

The accepted model says objects should eventually run across systems, that one Pool can own
Resources on several systems, and that Pool ownership is not a placement rule
([distribution direction](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#distribution-and-execution)). The language gives
an author no way to say any of it. Five things cannot be written or are ambiguous today:

1. A directory Resource stores an absolute path, but not whose filesystem the path belongs to.
2. An Agent that receives a directory by symlink only works when both are on the same host.
   No Conduit template can require or check that.
3. An authentication Resource backed by an OS credential store is bound to one system's store.
   Nothing records which.
4. Experiment 002 places one Agent on each execution service and spreads one Pool's Resources
   across both. That arrangement cannot be expressed in its YAML; the fixture hard-wires it.
5. "One designated service is authoritative for each instance" has no author-visible form: who
   designates it, where it is recorded and whether it can be inspected are unstated.

The [distributed-runtime research](../../research/DISTRIBUTED_RUNTIME.md) reached the same point
and stopped: the protocol "needs an execution-location identity separate from a mutable address",
without a sixth family, and whether that is deployment configuration, an environment capability
or an existing Resource type "remains open".

## Constraints already accepted

> Later ruling: these are the constraints as they stood on the day. Ruling 18 dropped `form`, so
> a reference is a kind and a name (OBJ-006).

- Five families only; supporting concepts add no family.
- Pool ownership is logical. Pool membership grants nothing.
- Reusable templates do not embed one deployment's identity; composition supplies concrete
  targets at instantiation ([dependency composition](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/POOLS.md#pools)).
- References use kind, form and name and are retained as stable IDs.
- No generic labels or selectors in v0. Nothing is chosen by discovery order.
- Fixed definitions: a changed definition means a replacement instance.
- Unknown ownership never triggers automatic reassignment.
- Declared compatibility is checked before execution; actual readiness is verified when needed.

## Decisions

Each decision lists the options considered and what was accepted. "Execution location" is the
research's working term and is used here without endorsing it.

### PL1 — What an instance is placed on

| Option | Consequence |
| --- | --- |
| A. A Resource of a core type, owned by the system Pool | Reuses identity, enrollment as provisioning, health observation, controls and retirement. No new family. |
| B. Runtime configuration outside the language | Simplest, but multi-system deployment is then invisible to the language, and an engine cannot be rebuilt from the language alone. |
| C. A sixth family | Contradicts the accepted five-family rule. |

**Accepted: A.** An execution location is a unitary capability ("hosts managed instances")
with one owner, which is what a Resource is. Enrolling a new system is provisioning that Resource
through the system Pool, which gives remote enrollment a defined home. Its stable ID is the
location identity; its address is ordinary mutable configuration. First initialization
establishes the local execution location alongside the system Pool. An execution-location
Resource is its own placement, which is the base case.

> Later ruling: ruling 29 removed "provision" as a verb. A location is instantiated by
> registering its document, like any Resource, and is then set up (PLC-008).

### PL2 — Where placement is declared

**Accepted:** follow the existing dependency-composition pattern, through the instantiation shape.

- An Agent or Resource **instance** carries exactly one placement reference, resolved to a stable
  ID and retained like its owner reference.
- Placement is supplied at creation: explicitly by the caller, or for a Pool's Resources through
  that Pool instance's composition entry.
- **Default:** when exactly one execution location is registered, an omitted placement resolves
  to it. A single-machine author writes nothing. When several are registered and none is given,
  creation is rejected before any effect, unless the owner has explicitly designated a default
  location. Without that option, enrolling a second system would make previously valid creation
  requests fail from then on.
- **Templates do not name a concrete location.** They may state what a location must offer (PL4).

Alternative considered: a separate deployment document mapping instances to locations. It keeps
templates clean but adds a document kind and a second place to look; the instance field with a
default achieves the same separation.

### PL3 — What placement means for each family

| Family | Meaning | Placement field |
| --- | --- | --- |
| Agent | Where its executions run and its private state lives. | Yes |
| Resource | Where its managed runtimes run and where its location-bound backing is interpreted. | Yes |
| Conduit | Each endpoint's local behavior runs at that endpoint's placement. The relationship record itself is held by management. | No |
| Pool | A record with no activity of its own; held by management. Each owned Resource has its own placement. | No |
| Interface | The instance record is held by management; frontends attach from anywhere. | No |

Holding Conduit records at management matches the accepted rule that new managed requests wait
when current permission cannot be established. It also answers part of audit question Q5.

Accepted consequence: under the no-offline-grants rule, a system cut off from management starts
no new managed work, even between two instances placed on that same system. Where the Conduit
record is held does not change this, because grants are checked at management. V0 multi-system
support therefore means running across systems while management is reachable. Loosening it
means revisiting bounded offline grants.

### PL4 — Compatibility between a type and a location

**Accepted:** a type contract declares the location capabilities it needs, and an execution
location publishes the capabilities it offers. The declaration is part of v0. Every type states its needs
explicitly, including stating that it has none, so a missing declaration is an error and not an
implied "anywhere" (confirmed by ruling 23; promise PLC-014). Creation checks one against the other for the
explicitly chosen location, before any effect, and names both sides in the diagnostic. For
example, a local-directory type needs local path access; a native Claude Agent type needs that
native runtime installed; OS credential custody needs that credential store.

This is a check on an explicit choice. It is not a scheduler and introduces no selectors.

> Later rulings: rulings 28 and 32 replaced "location capabilities". What a type or an
> implementation needs is a requirement on placement: a schema that the stored facts of the
> location must satisfy (CTR-005, PLC-007). A location publishes nothing; its facts are stored
> configuration, changed only by a control (PLC-008).

### PL5 — Access that only works on a shared host

Some access modes need both endpoints on one host: a symlinked directory, a local socket, a
stdio child. Others are location-transparent because they travel as operations.

**Accepted:** express this as an ordinary reusable relationship requirement (working name
`shared-host`) attached by the access mode that needs it. It is checked at establishment like any
other requirement. A required connection that cannot satisfy it fails preparation; an optional one
is reported. There is no silent fallback to another mode. No new mechanism is needed.

> Later ruling: ruling 32 (decision G13 of the re-audit) changed when and how this is checked.
> The engine checks the shared-host requirement when the Conduit is opened and on each request
> through it. Where it does not hold, the request is not dispatched and reports unavailable
> (PLC-013).

A shared host is necessary but not sufficient. An isolated Agent on the same machine still cannot
see host paths without a bridge. The requirement catches the cross-machine case before any
effect; the access mode's own preparation reports the rest.

This is the first requirement that depends on facts about both endpoints rather than on the
contents of a request, so it also tests whether the requirement model is expressive enough.

### PL6 — What a location is

**Accepted:** one execution service is one location, because that is the unit of
supervision, authority and failure. Two execution services on one machine are two locations, as
in Experiment 002. Each location reports a host identity. Shared-host requirements (PL5) compare
host identity, not location identity, since two services on one machine do share a filesystem.

### PL7 — Location-bound values

**Accepted:** absolute paths, OS credential-store references and local addresses are
interpreted at the instance's placement and nowhere else. Instance export includes the placement
reference, so an exported path is never ambiguous.

### PL8 — Change and failure

**Accepted:**

- Placement is fixed for the life of an instance. A different placement means a replacement
  instance, with backing data reused only where the type supports attachment.
- An unreachable location makes the state of its instances unknown. They are never reassigned.
  Instances at other locations are unaffected.
- A location cannot be unregistered while instances are placed on it.
- A location's address can change through a control without changing its identity.
- Moving an Agent can use the accepted fork-from-checkpoint operation, so its history need not
  be lost.
- **Declare lost.** The accepted retirement rule requires confirmed shutdown duties, which a
  permanently dead system can never give. Its instances and the location itself would otherwise
  stay registered forever. An explicit owner-only operation records that confirmation is
  impossible and lets the affected records be retired with that fact preserved as evidence. This
  is a new semantic; its exact contract is audit action A35.

## Worked example

Two enrolled locations, one Pool spanning both, one Agent on each. The examples use the document
shape and names ruled later the same day: one kind per family, `from` marking an instance, and
"location". The `apiVersion` line is omitted.

> Later rulings changed the shapes in the three examples below. An instance writes the values
> for its template's inputs under `arguments`, not `inputs` (rulings 28 and 29; OBJ-013). The
> schema of a reference to a location is now the core package's export
> `github.com/shrav-k/conduit/core#resource-reference.v1`, not
> `conduit:reference#/$defs/location` (rulings 28 and 33; the names are in
> [`spec/names.yaml`](../../../spec/names.yaml)). A location is made from the core package's
> template, and what is stored about it is its facts (PLC-008). The current worked example is
> [`spec/schemas/examples/valid/`](../../../spec/schemas/examples/valid/).

```yaml
kind: Resource
metadata: {name: laptop}
from: location
owner: system
inputs: {address: local}
---
kind: Resource
metadata: {name: buildbox}
from: location
owner: system
inputs: {address: buildbox.example.invalid:7443}
```

The Pool template names no location. It declares inputs whose values land on its entries, and one
input can land on several:

```yaml
kind: Pool
metadata: {name: project-pool}
spec:
  inputs:
    checkoutPlacement:
      schema: {$ref: "conduit:reference#/$defs/location"}
      to: [/entries/checkout/placement]
    servicePlacement:
      schema: {$ref: "conduit:reference#/$defs/location"}
      to: [/entries/auth/placement, /entries/issues/placement]
  entries:
    checkout: {from: project-directory}
    auth: {from: service-authentication}
    issues: {from: issue-service}
---
kind: Pool
metadata: {name: project-pool-0001}
from: project-pool
inputs:
  checkoutPlacement: laptop
  servicePlacement: buildbox
```

Each Agent instance carries its own placement.

```yaml
kind: Agent
metadata: {name: collaborator-0001}
from: collaborator
placement: laptop
inputs: {account: agent-account-a}
---
kind: Agent
metadata: {name: collaborator-0002}
from: collaborator
placement: buildbox
inputs: {account: agent-account-b}
```

What follows from the rules above:

- `collaborator-0001` can receive `checkout` as a directory: both are on `laptop`.
- `collaborator-0002` cannot. Its directory-mode connection fails the shared-host requirement at
  preparation. It can use a location-transparent mode if the directory type offers one.
- Both Agents reach `issues` through Conduits. The service credential stays with `auth` on
  `buildbox` and never moves to the laptop.
- If `buildbox` becomes unreachable, `auth`, `issues` and `collaborator-0002` report unknown
  state. `checkout` and `collaborator-0001` keep working.

The fixtures show only filters on a Conduit, not other requirements, so no YAML is shown for the
shared-host requirement. That is a separate gap in the existing examples.

## Promises this adds

Each is written so that a fixture can fail it. They are the test list for the simulated-network
engine ([audit option (b)](../../archive/plan-audit-history.md#multi-system-options)).

| ID | Promise |
| --- | --- |
| PL-1 | Every Agent and Resource instance has exactly one resolved placement, retained by stable ID and shown in inspection and export. |
| PL-2 | With one registered location, an omitted placement resolves to it. With several, an omitted placement resolves to the owner-designated default location if one is set, and is otherwise rejected before any effect. Discovery order never selects a location. |
| PL-3 | A template that names a concrete location is rejected at registration. |
| PL-4 | Creation rejects a location that does not offer a capability the type requires, naming both, before any effect. |
| PL-5 | A location-bound value is interpreted only at the instance's placement. |
| PL-6 | A connection whose access mode needs a shared host fails or is reported, per its required or optional marking, when the endpoints' hosts differ. No other mode is substituted. |
| PL-7 | An instance's placement never changes. |
| PL-8 | When a location is unreachable, its instances report unknown state and are not reassigned; instances elsewhere continue. |
| PL-9 | A location with instances placed on it cannot be unregistered, except through the owner-only declare-lost operation, which records that shutdown could not be confirmed. |
| PL-10 | Sharing a location or a host grants no access. |

## Out of scope

Automatic scheduling, selectors, migration of live instances, failover, replication, and the
trust exchange used to enroll a remote system. Enrollment gets a home under PL1; its mechanism
belongs with caller authentication (audit question Q3) and the wire profile (Q2). No remote
support is claimed before a test across two machines.

## Still to specify

> Later rulings: the first and the third are now specified. Declare lost is PLC-017 and
> LIF-017. A requirement that is not a filter, such as the shared-host requirement, is a
> Requirement contract applied in a Conduit template's `requirements` map (CTR-005, CON-018,
> PLC-013). The second no longer applies: there are no location capabilities, only a location's
> stored facts.

The exact contract of the declare-lost operation (audit action A35), the catalog of location
capabilities used by the first scenarios, and the YAML shape of a non-filter requirement such as
the shared-host requirement.
