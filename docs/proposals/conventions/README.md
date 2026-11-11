# The remaining shared conventions — accepted

Status: in force; amended by 41 (item 17); amended by 58 (M41); amended by 59 (A1, OL2); amended by 60 (OL2) (ruled by the owner on 2026-11-05). Holds now: [IN_FORCE.md: 7. Operations, outcomes, revisions, records and observation](../../../IN_FORCE.md#7-operations-outcomes-revisions-records-and-observation), [IN_FORCE.md: 8. Lifecycle, Pools and first start](../../../IN_FORCE.md#8-lifecycle-pools-and-first-start); [LIF-014 to LIF-023](../../../spec/conventions/lifecycle.yaml), [REC-001 to REC-006](../../../spec/conventions/records.yaml), [OBS-001 to OBS-006](../../../spec/conventions/observation.yaml), [IMP-001 to IMP-003, IMP-007, IMP-008](../../../spec/conventions/implicit.yaml). This record says why.

Every
recommendation was accepted. Written as promises in the specification:
[ownership and lifecycle](../../../spec/conventions/lifecycle.yaml) LIF-014 to LIF-023, with
ACC-033 and PLC-016; [observation](../../../spec/conventions/observation.yaml) OBS-001 to OBS-006;
[records](../../../spec/conventions/records.yaml) REC-001 to REC-006; and
[nothing implicit](../../../spec/conventions/implicit.yaml) IMP-001 to IMP-007.

This is the record of a ruling. The specification in [`spec/`](../../../spec/README.md) governs
where they differ. Amended by:

- ruling 32: every request and how it ended is always recorded, dispatched or not, which widens
  RC1; a purge keeps the identity and the outcome of every invocation; a Pool may be retired
  only when the shutdown of every Resource it owns is confirmed, which tightens OL4; an
  instance that was never set up does not run its `retire` graph; an instance that was retired
  because its location was declared lost can be unregistered without its shutdown being
  confirmed (re-audit decision A2, LIF-017), which widens OL3;
- ruling 33: a purge also keeps a digest of each invocation's contents; the `retire` graph
  never starts model work, as for setup; work arranged for later is bound to the Conduit that
  arranged it and never revives, which narrows NI3; where a record or an observation was made
  is its origin, not its source;
- ruling 37: changed LIF-007, LIF-009, LIF-013, REC-004 and ACC-031 to ACC-034.

Promise IDs and ranges are as first written; the comment at the head of each spec file says
where a removed ID went and which were added.

This batch covers the four shared conventions that are still unwritten or partly written:
ownership and lifecycle, declared against observed state, records, and that nothing starts model
work or repeats an effect implicitly. Most of each already exists as accepted text in
[docs/protocol](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/README.md) and only needs restating as promises. The decisions
below are the points where that text is silent, or where the rulings of 2026-11-05 changed what it
rests on.

## Ownership and lifecycle

Restated from accepted text, with no decision needed:

- Retiring stops new work reaching the instance and releases what the instance itself holds, as
  its type contract defines. It does not stop an external service or a shared Resource.
- Unregistering never erases records, backing files or service data. Purging those is separate.
- Other objects' references to an unregistered object are kept and reported unavailable. Nothing
  is cascaded, pruned or retargeted.
- Unregistering names the object's current revision, like any other change.
- There is no automatic retirement and no garbage collection. A Resource's owner cannot change.
- When shutdown can never be confirmed, an authorized actor reconciles the retire operation, as
  for any other operation whose outcome is unknown.

| ID | Decision | Options | Recommendation | Existing practice |
| --- | --- | --- | --- | --- |
| OL1 | When an instance becomes retired. | (1) When the retire operation is dispatched. Access ends at once; the status says whether shutdown is confirmed, pending or unknown. (2) Only when shutdown is confirmed, with a fourth state in between. | (1). A revocation has to take effect immediately, and it keeps three states. | Kubernetes marks an object for deletion at once and lets finalizers delay its removal |
| OL2 | Which Conduit permits inspecting, setting up, retiring and unregistering an object. | (1) One authority for an object's whole life: its owning Pool for a Resource, `management` for everything else, with opening and closing on `access` as ruled. (2) Everything on `management`. (3) A Conduit to the object itself. | (1). It extends the rule already ruled for creating a Resource. | Azure and Kubernetes scope management rights by group or namespace |
| OL3 | When an instance may be unregistered. | (1) Only when it is retired and its shutdown is confirmed. Retiring an instance that was never set up is confirmed at once. (2) Also directly from registered. (3) Unregistering retires first. | (1). One rule, and it matches the accepted text that removal never starts a shutdown. | — |
| OL4 | Retiring a Pool that owns Resources. | (1) Not dispatched, conflict, unless every Resource it owns is retired. One explicit operation retires the Resources and then the Pool, and keeps the Pool if any step does not succeed. (2) Retiring the Pool retires its Resources. | (1). It is the accepted text, restated for the three states. | Azure refuses nothing here and deletes the contents; the accepted text deliberately does not |
| OL5 | Whether the first-start objects can be retired or unregistered. | (1) Never: `system`, `local`, `management`, `access`, `root` and `root`'s two Conduits are permanent. (2) Only `local` may be. | (1). `management` and `access` are placed on `local`, so it could never be emptied anyway. | Kubernetes' `default` and `kube-system` namespaces cannot be deleted |
| OL6 | How the default location is designated. | (1) A control of the `system` Pool, changed under ordinary access. (2) An operation reserved to `root`. (3) No default: with several locations, placement is always written. | (1). It needs nothing new. | Kubernetes marks a default storage class with a setting on an object |

## Declared against observed state

Restated from accepted text, with no decision needed:

- An inspection keeps what was declared, what was observed and what was requested apart.
- Configuration changes only when a control confirms it. A setting is never shown as in force
  while the outcome of the control that set it is unknown.
- Observing starts no model work and permits nothing.

| ID | Decision | Options | Recommendation | Existing practice |
| --- | --- | --- | --- | --- |
| DO1 | What is reported when something cannot be observed now. | (1) Unknown, with the time of the last successful observation. (2) The last value, marked stale. (3) Unknown alone. | (1). | Kubernetes node conditions |
| DO2 | Whether anything acts to make the observed state match the declared one. | (1) No. A difference is reported and corrected only on an explicit request. A type contract may let its implementation keep its own runtime alive, provided that starts no model work and repeats no operation. (2) The engine reconciles continuously. | (1). It is the Compose and Terraform model already chosen, and continuous repair would undo revocations. | Terraform acts only on apply; Kubernetes controllers are the opposite |

## Records

Restated from accepted text, with no decision needed:

- Records are kept until explicitly purged, whether or not any Interface is attached, and
  failing to export them never erases them.
- Records from one source are ordered. No order is promised across sources.
- A gap in the records is reported as a gap.

| ID | Decision | Options | Recommendation | Existing practice |
| --- | --- | --- | --- | --- |
| RC1 | What is always recorded. | (1) A closed list: every access decision, every invocation and how it ended, every change of lifecycle state, every registration and unregistration, and every change of revision. (2) Left to each type. | (1). | Kubernetes audit events; AWS CloudTrail |
| RC2 | What every record holds. | (1) The caller, the instance acting for it if any, the Conduit used, the target, the operation and its identity, the time, and how it ended. Inputs and outputs are not required. (2) Also the inputs and outputs. | (1). Content is governed by each type's recording policy. | Kubernetes audit events hold who, what, when and the response status |
| RC3 | Who may purge records. | (1) An operation reserved to `root`, which itself leaves a record of what was purged. (2) An ordinary operation of `management`. (3) No purge operation in this version. | (1). No actor should be able to erase the trail of its own actions. | CloudTrail and Kubernetes audit logs cannot be deleted through the API they record |

## Nothing starts model work or repeats an effect implicitly

Restated from accepted text and earlier rulings, with no decision needed:

- Registering, instantiating, inspecting and observing start nothing.
- Delivering a message or a notice does not wake an Agent.
- An invocation is re-sent only as the operations convention allows. After an engine restarts,
  nothing that was in flight is sent again.

| ID | Decision | Options | Recommendation | Existing practice |
| --- | --- | --- | --- | --- |
| NI1 | What makes a start of model work explicit. | (1) Model work starts only as the effect of an operation whose contract declares that it starts model work, requested through an open Conduit that lists it. A graph that contains such a step implements an operation that must declare the same, which is checked when the template is registered. (2) A per-type matter. | (1). "Who may cause whom to spend tokens" becomes a declared relationship that can be read from the documents. | Effects declared in a contract, as MCP tool annotations declare read-only and destructive |
| NI2 | Whether setup may start model work. | (1) Never. A setup graph with a step that starts model work is rejected. Ready means prepared; work begins with a request. (2) Allowed when the setup operation declares it. | (1). It keeps "ready" meaning that nothing is running on the owner's account. | Compose and Terraform separate creating from invoking |
| NI3 | Work that starts later or repeatedly, such as on a schedule. | (1) Allowed only when set up by an operation that declares it. Each start is recorded against the actor that set it up and checked against that actor's access at that moment. (2) Not in this version. | (1). It follows from the promise that later work runs under the caller's access as it then stands. | cron jobs run as the user who installed them |

## Choices made while writing the rulings

The owner did not rule on these one by one. Each can be changed by a ruling.

| Choice | Why | Promise |
| --- | --- | --- |
| Shutdown is "pending" whenever the retire operation has neither succeeded nor an unknown outcome, and a retired instance whose shutdown is not confirmed can be retired again. | OL1 named the three words; a failed shutdown has to be retryable. | LIF-014 |
| A request to retire or unregister a first-start object reports refused. | It matches the promise that `root`'s Conduits cannot be closed. | LIF-022 |
| A purge never removes the record of an earlier purge. | Otherwise two purges would erase the trail RC3 exists to keep. | REC-004 |
| An operation without the mark does not start model work. | The same convention as `optional`: one mark, and its absence has one meaning. | IMP-003 |
| Inspecting a Conduit is authorized against `management`, like other objects that no Pool owns. | OL2 gave `access` only opening and closing. | ACC-033 |

## After this batch

One more batch of rulings remains: the schema of each family's `spec`, the implementation
interface, and the list of implementation-defined behavior. After it, the promises are
consolidated: overlapping ones merged, each file reviewed against the definitions, and the whole
specification put back through the cold-reader, consolidation and defect sweeps.
