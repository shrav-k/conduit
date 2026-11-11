Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Project as a sixth family: one home per object, Pools shared only by their home project, authority as Conduits to Projects

## name
Project as a sixth family: one home per object, Pools shared only by their home project, authority as Conduits to Projects

## thesis
A Project is a sixth family of object. It has an ID, a name, the common lifecycle (registered, ready, retired) and its own capability, `project-management.v1`. It lives in the engine scope beside the first-start objects.

Every object outside the engine scope has exactly one home project. The home is recorded on the object at registration and never changes, and a Resource's home is its Pool's. A Project "references every object under it" through an inventory computed from those records. It does not keep a member list of its own.

A Pool has one home. Other projects use it only through a share that its home grants: a ceiling of operations, given to named projects or to all. A share grants use and never administration. Pools never have two homes.

Authority over a project is an open Conduit to that Project. It lists management and access operations and may carry a project bound. An Interface with authority over three projects holds three Conduits. Each one is homed in its Project and opened only by that Project's own authority. This is the sense in which projects determine an Interface's permissions there.

A request about an object is permitted by an open Conduit that lists the operation and goes to any of the object's scopes: its Pool (for a Resource), its project, or the engine (`management` and `access`). The record names the narrowest of these. That is how root holds authority over every project from first start without one Conduit per project.

A Conduit that crosses projects needs two halves: authority in the consumer's project, and in the target's project. A share can stand in for the target's half.

A widening runs at once when the actor holds covering authority on every side. A bound acts as a standing approval, and only an approver may grant one. Any other widening is proposed. It becomes a pending action, raised at the Interfaces that hold `approve.v1` on the projects it touches. Narrowing never waits.

The owner's direction holds. Four parts of it are corrected:
- The engine's own objects belong to no project. Putting `system` in a project hands out near-root power.
- Shared Pools are shared for use, not for administration.
- An Agent's authority needs a template dimension in its bound, or a project is only as strong as its weakest create right.
- Secret wiring stays with root.

This is about 75 to 80 promises. That is the same scale as making the short form normative, so the two should be decided together. The S1 author writes nothing more: 16, 6, 0, 2, 1, 1, 1 against A70's 33, 16, 0, 8, 3, 2, 2.

## model
## What a Project is

- **A sixth family.** OBJ-020 has eleven kinds: six families and five contract kinds. The core Type is `project.v1`: family Project, no settings, offers `project-management.v1`. The core template is `project`.
- **What it does not have.** A Project has no implementation, placement, runtime or hooks, as Interfaces and Pools run nothing (TPL-019). In v0 every Project is of `project.v1`. CTR-004 is extended so that any later Project Type must offer `project-management.v1`.
- **Why a family and not a label.** Authority is a Conduit, and a Conduit targets an object (CON-001). Design D's skeptic required a project identity "that is checked, not the Pool's name", and memory (tombstones, applied state) that outlives every Pool. Both need an object with an ID and a lifecycle. A stored fact alone, like a namespace label, could not be a Conduit target.

## What it holds and what it references

**Stored on the Project:**
- ID, name and description.
- Its **shares out**: entries with engine IDs, each giving the Pool, the receiving project (a name, or `all` with `future: true|false`) and a ceiling of qualified operation names.
- Its **tombstones**: ruling 53's tombstones, keyed by project ID. They outlive the removal of members, and of the Project itself, until root purges.
- Its **pending actions**.

**Computed and never stored:**
- Its **members**: every object whose home is this Project.
- Its **shares in**.
- Its **authority**: the Conduits whose target is this Project.

All three are returned by `inspect.v1`, limited to what the caller may inspect and with no counts of the rest (ruling 53, decision 3c).

**Challenge: no member list on the Project.** A list kept on the Project would be a second source of truth that drifts and races. Kubernetes moved from allowlist prune to ApplySet's part-of label for this reason. Every system studied records membership on the member. The concise project file still lists everything for its author.

## Membership, kind by kind

| Kind | Rule |
| --- | --- |
| Agent | Exactly one home: the project named by its submission's manifest (new `project:` key). Fixed for life. Moving is replacement. |
| Resource | Home is its owning Pool's home. POOL-001 is unchanged. A Resource may be registered only into a Pool homed in its submission's project; a shared-in Pool's home registers into it, never a user of the share. |
| Pool | Exactly one home. Other projects use it through shares (see sharing). It is never a member of two projects. |
| Conduit | Homing depends on the Conduit. A declared Conduit is homed with its consumer (CON-002). A standalone Conduit is homed in the project it is registered into, which must be the home of its consumer or of its target. A Conduit whose target is a Project is homed in that Project, so a project's authority grants are its own members. A Conduit whose ends are both engine-scoped is engine-scoped (root's first-start Conduits). |
| Interface | Homed in one project like any member. Root is the only engine-scoped Interface. A person working across projects is homed in a project chosen for who should be able to retire them (for example `people`, managed by root), and holds Conduits to many projects. Challenge below. |
| Template | Homed in its submission's project. An instance comes only from a template in its own project, a package export, or a core template. A `from` naming another project's template is rejected. This removes the cross-project `replace.v1` hijack (design E, number 4). |
| Contracts, pins | As today, per submission (NAM-009, SUB-008). `install.v1` stays engine-wide and grants nothing (NAM-013). |
| Nodes, machine provider, package-source providers | Engine-scoped: owned by `system` (PLC-008, PRV-013). In v0 any project may place on any node, so placement still carries no authority (PLC-015). Scoping nodes to projects is v1 (UC5). |
| Runtime and secret-store providers | Homed like any Resource. `runtime` may name only a provider homed in the instance's own project or owned by `system` (PRV-006 changes). Provider operations can never be in a share ceiling (PRV-004). |
| Engine scope (closed list) | The five first-start objects and root's two Conduits; every node; every Resource owned by `system`; Projects; installed packages and core templates. A submission into the engine scope states `project: engine`, a reserved value, and is authorized only by `management`. |

**Challenge: "every object belongs to a project" stops at the engine scope.** Putting `system` into a project would make that project's open authority cover `management` and `access`. That is FST-001's near-root hazard. Kubernetes has the same split: every kind is namespaced or cluster-scoped, never both.

**Challenge: where a person's Interface is homed.** Homing an Interface lets that project's managers retire it, even when its authority spans other projects. The owner should decide where people are homed. The recommendation is a root-managed `people` project.

## Names

- Names are unique within a project **across kinds**. This is ruling 37's "unique across kinds within a project", now applied to the registered Project, not only the submission. Generated names (OBJ-003) are unique per project.
- Project names, and the names of engine-scoped objects, are engine-wide. `engine` is reserved as a project name. The five first-start names (`root`, `system`, `local`, `management`, `access`) are reserved in every project.
- **References:**
  - `{kind, name}` resolves in the referring object's project, and otherwise only to the five reserved names. It never falls back elsewhere: no shadowing, unlike GitHub's "lowest level takes precedence".
  - `{kind, project, name}` names a member of another project. `project: engine` names an engine-scoped object, such as a node in `placement`.
  - A reference in `config` or `arguments` to another project's object is rejected, unless the object is a Resource of a Pool shared into the referring project. This guards against the confused deputy.
- **Requests** (LIF-031) always state the project of a project-homed object. The engine keeps no current project; the CLI fills the project in from the file.
- **Tool names** are unaffected, since they come from `as` (IFC-012).
- **UC8 works:** `conduit-core/shared` and `packages/shared` can both exist.

## Lifecycle

**Create.** `register.v1` of a Project is authorized only by `management`, because creating a scope is an engine-level act. Then `set-up.v1`, which does nothing but mark it ready. A registration naming an existing Project is rejected (OBJ-002), so nothing is ever adopted by name. The result names the ID of any earlier Project removed under that name (ruling 52).

**Register into it.** A submission whose manifest says `project: p` homes every object it makes in `p`. A Project document named `p` may sit in the same submission and is made first, as a Pool's Resources may come with the Pool today. No registration goes into a retired Project, as POOL-004 has it for Pools.

**Retire it:**
- `retire.v1` on a Project reports conflict while any member is not retired with its shutdown confirmed. This mirrors LIF-020.
- `retire-with-members.v1` is authorized by a Conduit to the Project or by `management`, and the Conduit need list only that operation (as LIF-021). It retires Agents and Interfaces first, which closes their Conduits under CON-008. Then each Pool goes with its Resources in LIF-021's order, then the Project.
- It refuses up front, with nothing done, while any Pool of the Project is shared out with live Conduits from other projects, unless the request states `includeShared: true`. This follows RAM's "remove from shares before deleting" and the PRV-020 precedent. Narrowing never waits, but it must be explicit.
- Pools shared into the Project are never touched.
- Conduits from other projects into the Project's members close as those members retire.
- Conduits to the Project close when it is retired (CON-008 extended).
- Its pending actions are voided.

**What survives:**
- Records, which only root purges (REC-004).
- The Project's tombstones, kept as engine state under its ID.
- Every shared-in Pool, every engine-scoped object, and every Interface homed elsewhere.

**Its name.** The name is freed when the Project is removed (LIF-035). A new Project with that name gets a new ID and inherits nothing. Authority Conduits pointed at the old ID and closed with it. Shares to the old ID ended with it. The old tombstones stay keyed to the old ID, and the concise apply refuses to treat the new Project as the old one without `--recreate`.

**Nesting.** Projects are flat in v0, as Kubernetes namespaces are. Membership is immutable.

## sharing
## Shares: one home, explicit grants for use

### How a share is declared
- **By request.** `share.v1` and `unshare.v1` are sent to the Pool's home Project. Each share is an entry stored on the home Project, with an engine ID. `expectedId` works on it, a tombstone is left when it ends, and inspection lists it on the home, on the Pool and on each receiving project.
- **In the concise form:** `pools: {skills: {share: {to: all, future: true, ops: [...]}}}`.
- **Input of `share.v1`:**
  - `pool`;
  - `to`: a list of Projects, or `all`. With `all`, `future` must be stated;
  - `operations`: the ceiling, as qualified names compared as names, like a bound's `operations` (ACC-029).
- **The ceiling may never list:** `get-secret.v1` (SEC-008; a secret Resource's type offers nothing else, D10), provider operations (PRV-004), operations reserved to root (ACC-032), or any `pool-management.v1` or `project-management.v1` operation.
- **Granularity.** A share covers every Resource of the Pool, present and future, as a bound naming a Pool does today (ACC-029). The plan and inspection must say so. To share less, split the Pool. This is the owner's "more specialized Pools".

### Rejected: a Conduit from a Project to a Pool
- A Project is not an actor and never sends a request.
- The ceiling is not the operations of a capability the Pool offers, so CON-001 would break.
- Ruling 28's "every permission is an open Conduit" still holds: the share grants nothing by itself. It is a ceiling, like an SCP or RAM's managed permission. The grant is still a Conduit from a member to the Resource.

### Who decides
- **The home side.** The Pool's home project decides: a holder of `share.v1` on that Project, or on `access` at engine level. Only the home shares, so shares are not transitive. RAM has the same rule: "can't share a resource that's shared with you".
- **The receiving side, for a named project.** The receiving project must accept. That happens at once if the sharer also holds `approve.v1` on it. Otherwise the share becomes a pending action of kind `share-offer` at the receiving project's approvers, with an expiry, like RAM's invitation. Why it matters: a share lets another project's content reach your Agents.
- **`to: all`.** This affects projects that may not exist yet, so it needs engine authority: `share.v1` on `access`, which root holds. `future: true|false` is mandatory, to avoid RAM's `Principal: *` pitfall.

### What a receiving project may do
- Open Conduits from its own members to the shared Pool's Resources, through its own open authority (the consumer half). The share stands in for the target half, as long as every operation is within the ceiling.
- Inspect the shared Pool and its Resources' name, ID, Type, description and state. That is enough to choose a target.
- Close its own Conduits into the Pool at any time.
- Leave the share by sending `unshare.v1` for its own entry. That is a narrowing on its own side.

### What a receiving project may not do
- Register Resources into the Pool, or set up, retire, replace or reconcile its Resources.
- Retire the Pool.
- Share it onward.
- Open Conduits for consumers that are not its members.
- Exceed the ceiling.
- See other projects' Conduits to the Pool or their records. This is ruling 52's first tier, with no counts.
- Name the Pool's Resources in `config` beyond what PRJ-009 allows.

### Narrowing a share
- **By the home:** `unshare.v1` for a project, or a lower ceiling. Every Conduit from that project's members to the Pool's Resources that lists an operation outside the new ceiling closes at once. Each leaves a tombstone whose cause names the unshare.
- **By the receiving project:** it closes its own Conduits, or leaves the share.
- **Never waits.** Either side may narrow at once.

### When a side retires
- **The receiving project retires:** its members' Conduits into the Pool close as the members retire, and its share entry ends and is tombstoned.
- **The home retires**, or the Pool is retired: refused while the Pool is shared out with live Conduits, unless `includeShared: true`. The plan lists every cross-project Conduit that would close.
- **Shared Pools that must outlive any one project** belong in a long-lived project. The recommended home is a `commons` project managed by root, because membership cannot move.

### Prior art
This is the design of AWS RAM, GCP Shared VPC, Gateway API ReferenceGrant and GitHub organization secrets with `selected` visibility:
- one home;
- the owner creates the share;
- a ceiling;
- the consumer grants within the ceiling;
- not transitive;
- acceptance across a boundary;
- immediate revocation.

### Challenges to the owner's direction
- **"Pools accessible from multiple projects" is right for use and wrong for administration.** Administration by several projects would bring back ACC-033's single-authorizer conflict, ACC-012's mutual close, and R9 (any one of them could retire the whole Pool). It would also break preservation test 3, "owned once".
- **The skills Pool for all projects:** read-only ceiling, `to: all`, `future: true`, writes only by its home's managers.
- **The auth Pool for one project:** home in that project and shared with no one. The MCP Resource that uses the secret (`linear`) may later be shared with a ceiling such as list and get (UC10). The secret Resource itself never can be.
- **A scope is only as strong as its weakest create right.** Whoever holds open authority in a project reaches every Pool shared into it, up to the ceilings. Keep the auth Pool's project free of Agents that hold open authority.

## authority
## Authority: the scopes of an object

### The rule that replaces ACC-033's rows
The scopes of an object are its owning Pool (Resources only), its home project, and the engine. In the engine scope, `management` covers management verbs and `access` covers access verbs.

A management or access request about an object is permitted by an open Conduit from the actor that lists the operation, to **any** of the object's scopes, within that Conduit's bound. A retired scope is skipped. The record names the Conduit of the narrowest scope that permits the request, so it states the most specific authority.

Ordinary operations are never covered by a scope: a turn, a read, a control. These still need a Conduit to the instance itself (ACC-001, ACC-003, ACC-028). Administering is still not using.

### The routing table

| # | Request | Object | Permitted through a Conduit listing the operation to |
| --- | --- | --- | --- |
| P1 | `register.v1` | A Project, or anything in the engine scope | `management` |
| P2 | `register.v1`, `register-replacement.v1` | The objects of a submission into project P | P or `management`. A submission with only Resources from existing templates, into an existing Pool X of P, may instead go through X |
| P3 | `replace.v1`, `unregister.v1` | A template, or a retired instance, homed in P | P or `management`. First-start objects are still refused (LIF-022) |
| P4 | `set-up.v1`, `retire.v1`, `inspect.v1`, `reconcile.v1` | A Resource of Pool X in P | X, P or `management` |
| P5 | Same | An Agent, Interface, Pool or Conduit in P | P or `management` |
| P6 | `retire-with-resources.v1` | Pool X in P | X, P or `management` |
| P7 | `inspect.v1`, `retire.v1`, `retire-with-members.v1` | Project P | P or `management`. Setting P up: `management` |
| P8 | `open.v1` | Conduit C, consumer c, target t | A consumer half: open authority in c's home project, or `access`. A target half: open authority in t's home project, or `access`, or a share of t's Pool into c's project whose ceiling covers C's operations. One half is enough when c and t share a project. Both halves are within bounds (see below) |
| P9 | `open.v1`, `close.v1` | A Conduit to a Project P (granting authority) | P or `access`, under ACC-030 extended (see delegation). No consumer half: authority in P gives nothing in the consumer's project |
| P10 | `close.v1` | Conduit C | Either half, covering C. Root's Conduits are never closed (ACC-024) |
| P11 | `share.v1` | A Pool of P, to named projects | P or `access`, plus the receiving side (see sharing) |
| P11 | `share.v1` | A Pool of P, `to: all` | `access` only |
| P11 | `unshare.v1` | A share entry | The home side (P or `access`), or the receiving project for its own entry |
| P12 | `propose.v1` | A proposal | Any touched project where the actor holds `propose.v1` |
| P12 | `approve.v1` | A pending action | For each touched project, `approve.v1` on it, or on `access` (see approvals) |
| P13 | `install.v1`, `declare-lost.v1`, `purge.v1` | Engine | `management`. Reserved operations are root's own (ACC-032) |
| P14 | A control | An instance | The instance itself, unchanged. An input its Type marks as widening also needs the approvals below |
| P15 | Any other operation | An instance | A Conduit to that instance only |

### Unchanged special cases
- `get-secret.v1` is opened only by root, on its own behalf, for a Resource or Agent consumer (SEC-008, SEC-009). A Project's `approve.v1` never covers it.
- Conduits to `management` or `access` are opened only through `access` (ACC-030).
- Provider operations are never opened (PRV-004).
- A Conduit with a retired end gives a conflict (CON-031).

### Why a chain of scopes and not one authorizer
The alternative keeps "one target authorizes": P for non-Resources, the Pool for Resources, plus a Conduit from root to every Project made automatically. It brings back root's Pool Conduits, which were 40 of the coordinator's 72 root lines, or it strips Pools of authority. The chain is Azure's scope model (management group, subscription, resource group, resource). Ruling 30's OL2 already cited it. It also follows ACC-029's precedent that a Pool covers its Resources.

## The `project-management.v1` capability
Every Project offers it, with 16 operations:
- `register.v1`, `replace.v1`, `unregister.v1`, `register-replacement.v1`;
- `inspect.v1`, `set-up.v1`, `retire.v1`, `reconcile.v1`;
- `retire-with-resources.v1`, `retire-with-members.v1`;
- `open.v1`, `close.v1`;
- `share.v1`, `unshare.v1`;
- `propose.v1`, `approve.v1`.

**One Conduit to P is a project role.** "Admin" lists them all. "Operator" lists set-up, retire, inspect, reconcile and close. "Proposer" lists inspect and propose. "Approver" lists inspect and approve.

## Bounds on a Conduit to a Project
CON-028 changes so that a bound may be carried by a Conduit to `access` (as `bound.v1`, unchanged) or by a Conduit to a Project, as the new requirement `project-bound.v1`.

**Inputs.** There are five, each required and each a list or the literal `all`. Nothing is implicit (ruling 52).
- `templates`: `register.v1` may make instances only from these, and may register templates only when this is `all`. An instance made from a listed template is also covered as a consumer and as a target. This is how a bound covers members made later without naming them. It solves the frozen-reference problem (OBJ-021) explicitly, not by covering the whole project.
- `consumers`: which members may be the consumers of the Conduits it opens, closes or proposes.
- `targets`: which members it covers as targets.
- `pools`: Pools of P or shared into P. A listed Pool covers its Resources as targets, now and later.
- `operations`: the operations the Conduits it opens or proposes may list.

**What it fixes:**
- B4: a holder can no longer open for any consumer.
- Ruling 44's D04, which held that management cannot be narrowed to one template, for project-scoped authority.

**What stays the same.** `management` itself stays unnarrowable. A bounded Conduit to `access` keeps today's meaning, but for project members it supplies only the target half. Its holder can no longer wire any consumer.

## Root
Root holds root-management and root-access from first start, as today (FST-002). They sit at the top of every object's scopes, so root has authority over every project and every Pool's Resources with no further Conduit. This removes the 40 lines of root-to-Pool Conduits in the coordinator's S1 run (`pool-admin`, `root-workspace`, `admin2`, and the two fix-up manifests).

The capability changes:
- `object-management.v1` gains `retire-with-resources.v1` and `retire-with-members.v1`;
- `access-control.v1` gains `share.v1`, `unshare.v1` and `approve.v1`;
- root's two templates list them.

Root still needs a Conduit to an Agent to take its turns: ordinary operations are never covered by a scope.

**What changes for other holders of `management`.** Every other holder of `management` now also administers every Pool's Resources. Today such a holder needs a Pool Conduit for that. This is stated as a widening of what `management` means.

**Root's identity powers are unchanged:**
- reserved operations (ACC-032);
- get-secret (SEC-008);
- protection from closing (ACC-024);
- permanence (LIF-022);
- in v0, sign-ins (ruling 49).

## Delegation (ACC-030 extended to Projects)

**Who may open or close a Conduit to P.** Only an actor holding `open.v1` (or `close.v1`) on P or on `access`. Its operations must be a subset of the actor's own operations on P. Its `project-bound.v1` must be within the actor's: each dimension is a subset, and `all` contains everything. An unbounded one can come only from an unbounded holder. **Approve** is passed on only by a holder of `approve.v1`. **Envelope:** a Conduit to P that lists `open.v1` is opened only by an actor that also holds `approve.v1` on P. Whoever may widen without asking is therefore always chosen by an approver.

**What stays as today:**
- Revocation does not cascade (ACC-030).
- A Conduit to P can be closed only by a holder whose bound contains its bound. A narrower co-manager cannot strip a wider one.
- Equal co-managers can still close each other's grants, as today. This is listed as an open question.
- Inspecting P lists every Conduit to P, with who opened it. The opener is added to ruling 53's inventory.

## How a second person gets granular authority
1. Root (or a holder of `register.v1` on the home project) registers Interface `collab`, homed in `packages` or in `people`.
2. A holder of `open.v1` and `approve.v1` on `packages` (root) opens `collab` to Project `packages`, listing the role's operations and, if wanted, a `project-bound.v1`.
3. `collab` now has these powers:
   - registers, sets up, retires and inspects `packages` members;
   - opens Conduits between `packages` members, and into Pools shared to `packages` within their ceilings;
   - approves `packages`' side of proposals;
   - shares `packages`' Pools to named projects, with those projects' acceptance.
4. `collab`'s authority can be read from `collab` (the Conduits it consumes) and from `packages` (its authority list). Either side can revoke it at once.
5. **Prerequisite, not solved here:** a way for a person to hold a token for `collab` (DL13). The first build issues client tokens only to root and to run instances, so a second person cannot connect today.

## The owner's "projects determine an Interface's permissions"
Accepted, in the pairwise form:
- Each Project decides who has authority in it and who approves widenings into it.
- An Interface's permissions are the sum of what the projects granted it (test 8, ACC-023).
- No list on the Interface is the source of truth. A grant lives with, and can be revoked by, each scope, as in Lighthouse, RoleBinding and IAM bindings.

## approvals
## Pending actions and approval

### Shape
Ruling 51, decision 5 gives the shape. The fields are:
- `id` and `kind`. The kind is one of `approval`, `share-offer`, `sign-in`, `enrollment` and, later, `peer-introduction`.
- `projects`: the projects it touches.
- `subject`: the exact requests, each with `expectedId` and with a digest over the whole.
- `evidence`: written by the engine. It gives the reach before and after for each consumer, the tools gained with their target's project and Type description, and the shares changed. The proposer's reason is shown only as a quote.
- `proposer`, with its caller chain.
- `needs`: per project and side, which approval is still missing.
- `approvals` given.
- `expires`.
- `outcome`: pending, dispatched (with each request's own outcome), rejected, expired, voided, or conflict on drift.

### What counts as widening (a new promise, judged by the engine)
**Widening:**
- `open.v1` of any Conduit, including a Conduit to a Project or to `access`;
- `set-up.v1` that would open declared Conduits (ACC-005);
- `share.v1` that adds a project or an operation to a ceiling;
- superseding a tombstone (reopening or recreating);
- `replace.v1` of a template a live instance came from;
- a control that sets an input its Type does not mark `widens: false`. An unmarked input is treated as widening (fail-safe). Each package marks its harmless inputs, which is provider work.

**Narrowing:** close, retire, `retire-with-*`, unregister, unshare, lowering a ceiling, rejecting.

**Neutral:** `register.v1` (nothing gains reach until it is opened or set up), `inspect.v1`, `install.v1` (NAM-013), reconcile.

### When a widening waits
A widening is dispatched at once when the actor holds the operation, with a covering bound, on every half it touches. For an open, the halves are the consumer's project and the target's project (or a share). Otherwise:
- **If the actor holds `propose.v1`** on a touched project, it sends `propose.v1` with the subject. The engine records the pending action and returns its ID and what is needed. Nothing is dispatched.
- **If it does not,** the request is refused in ruling 52's first tier. Where the caller holds `propose.v1`, the refusal says it may propose instead.

**Every proposal must name approvers.** Each half the proposer lacks needs approval from that project's side. A subject outside every possible approver's bound is invalid.

### Who may approve, and how it is declared
- An approver holds an open Conduit listing `approve.v1` to the touched Project, or to `access` for the engine scope and for `to: all`. Its `project-bound.v1` must cover that project's part of the subject.
- It is declared like any authority. In the concise form it is `approvers: [collab]`, compiled into Conduits homed in the Project. Inspection shows it from both sides.
- Only holders of `approve.v1` pass it on (ACC-030 extended), and only an approver opens an envelope (a Conduit listing `open.v1`).
- Root approves everything through root-access.

**Rules that hold for every approval:**
- **Not the proposer, and not its puppet.** The approver is not the proposer. Its approve Conduit was not opened by the proposer. Because approval passes only from holders, a proposer without approval cannot create one.
- **Interfaces only in v0.** The consumer of a Conduit listing `approve.v1` must be an Interface. This follows the owner's "handled through the Interface" and ruling 49. It is a stated exception to preservation test 5, like SEC-009's limit on consumer kinds. It is reversible in v1, and an open question.
- **Bound to the exact change.** `approve.v1` carries the action ID and the digest, plus `decision: approve | reject`.
- **Checked again at the end.** On the last approval needed, every subject request is decided anew (ACC-036). Its `expectedId` values must still match, or the outcome is conflict and nothing is dispatched. The proposer must still hold `propose.v1` and not be retired. The approvers' Conduits must still stand.
- **No deferred work.** The requests are dispatched inside the approving invocation, so no work outlives an invocation (ACC-019). The approver is the actor who asks later.
- **Recorded.** Each record names the final approver as the actor, with `proposedBy` and `approvedBy`.
- **Root's secrets stay root's.** A subject that lists `get-secret.v1` can be approved only by root, and that approval counts as root's own request (SEC-008).

### Where actions are raised
- An action is raised at every Interface whose approve Conduit covers a missing approval, project by project. This is the Interface as control panel (ruling 50).
- It is always visible to root.
- It is never sent to an Agent (ruling 49).
- A proposing Agent learns the outcome when it next makes a request or inspects. It is never woken (test 6).
- An action shows another project's part only as the proposer named it, with no state of that project's objects. This is ruling 52's first tier.

### Kinds of action
- **Plan approval** (the open ruling 53, decision 2: approving without a terminal). For an author who is not an approver, such as a fresh Codex session holding `propose.v1`, `conduit apply` first sends its narrowings directly. It then sends its widenings as one `propose.v1`. The approver approves at its Interface: the approval moves from the CLI's y/N into the engine. For root, or any approver acting itself, nothing is pending and the plan is printed and recorded as in ruling 53, decision 4. S1's root author never sees an approval.
- **Sign-ins** (ruling 49). The routing promise is written now: to the approvers of the instance's home project that are Interfaces, and always visible to root. Behavior stays root-only in v0, as ruled. It widens when other Interfaces can authenticate (DL13) and the owner turns it on. The check of the expected account is unchanged.
- **Enrollment** (ruling 50). Nodes are engine-scoped, so admissions go to engine-level approvers (root). Scoping machines to projects (UC5) waits for authority over nodes and runtime in v1.
- **Peer introduction** (ruling 51). One action touching the engine scope (trust, approved by root) and the exposing project (approved by that project's approvers). The partner's Interface is homed in that project, and its authority is Conduits within it.

### Expiry and ending
- The proposer gives `expiresIn`, up to a maximum the engine defines (a new DL). Twenty-four hours is suggested, as Kubernetes CSRs have.
- Any one approver's `reject` ends the action.
- The proposer may withdraw it through `propose.v1`.
- The action is voided when the proposer or the project retires.
- An expired action dispatches nothing. Its registered objects stay registered and inspectable.

### Narrowing never waits
- Narrowings are dispatched at once, given authority on either side.
- `propose.v1` rejects a subject that contains any narrowing; send it directly.
- A pending action never reserves its subject. A narrowing that changes the subject makes the approval fail on drift.
- In S1's T4, B's narrowing would apply at once while A's widening waited, if the author were not root.

### Challenge: why bounds act as standing approvals
If every widening by anyone but root had to wait for a person, the coordinator could not dispatch a worker without a person (UC9). An envelope, meaning an open Conduit with a bound, is a declared, inspectable approval given in advance by an approver. Anything outside it is proposed.

## examples
All type names under `example.org`, and operation names marked `...`, are placeholders. Concise operation lists are shortened for width; the files write them fully qualified (ruling 25).

## 1. S1 as one project

**Concise form.** This is the simplification package's file with no added line. `project: s1` now names the Project object itself:
```yaml
conduit: v0
project: s1
packages: {...}                                   # TOOL, written by conduit install
resources:
  shared: {type: example.org/s1/filesystem#local-directory.v1, config: {root: <lab>/shared}}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach:
      project: {to: shared, ops: [file.read.v1, directory.list.v1]}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    reach:
      project: {to: shared, ops: [file.read.v1, directory.list.v1, file.write.v1]}
```

**Plan, abridged:**
```
+ Project s1                                  engine scope; through management (root)
+ Pool s1/default                             derived: owns every Resource with no owner
+ Resource s1/shared   + Agent s1/agent-a, s1/agent-b
+ Conduit s1/agent-a--project  as project     agent-a -> shared [read, list]
+ Conduit s1/agent-b--project  as project     agent-b -> shared [read, list, write]
+ Conduit s1/agent-a--turn, s1/agent-b--turn  root -> Agent [turn.v1]   derived, homed in s1 (the target's project)
authority over s1: root, engine scope (management, access); no Conduit to s1 needed
approvers of s1: root, engine scope. Widenings by root are never pending
shares in: none     shares out: none
```

**Ruling 53, decision 6 is answered:**
- "A Pool per project" becomes the Project plus its recorded default Pool.
- "The operator's Conduit" becomes none for root (the engine scope covers it), and a Conduit to the Project for any other operator.
- The turn Conduits stay derived.
- The derived Pool can be called `default` in every project, because names are now scoped per project. That closes the critic's point 16.

**Canonical form: what this design adds to the coordinator's documents.** A manifest key and one document:
```yaml
# conduit.yaml
apiVersion: github.com/shrav-k/conduit/v0
project: s1                     # new key, MAN-004: every object made here is homed in s1
packages: {...}
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Project
metadata:
  name: s1
from: github.com/shrav-k/conduit/core#project
```

**What it removes:** `pool-admin` (12 lines), `root-workspace` (8), `admin2.yaml` (16) and the two fix-up manifests (4). Root's engine scope now covers the Pool `workspace` and its Resources.

## 2. The owner's real use: two projects, a shared skills Pool, a project-only auth Pool

Projects: `conduit-core` (coordinator, triage, Linear and its credential, the repository and the queue) and `packages` (one worker per order).

**`conduit-core/conduit-project.yaml`, concise:**
```yaml
conduit: v0
project: conduit-core
pools:
  skills: {share: {to: all, future: true, ops: [fs#file.read.v1, fs#directory.list.v1]}}   # needs engine authority: root
  repo:   {share: {to: [packages], ops: [fs#file.read.v1, fs#directory.list.v1, history#post.v1, history#read.v1]}}
  auth:   {}                                     # shared with no project
resources:
  claude-skills: {pool: skills, type: example.org/s1/filesystem#local-directory.v1, config: {root: ~/skills/claude}}
  codex-skills:  {pool: skills, type: example.org/s1/filesystem#local-directory.v1, config: {root: ~/skills/codex}}
  main:          {pool: repo, type: example.org/s1/filesystem#local-directory.v1, config: {root: ~/conduit-main}}
  queue:         {pool: repo, type: example.org/s1/history#file-history.v1, config: {root: ~/conduit-queue}}
  onepassword:   {pool: auth, type: example.org/onepassword#secret-store.v1, config: {vault: conduit-test}}
  linear-key:    {pool: auth, type: example.org/onepassword#item.v1, config: {item: linear-api}}
  linear:        {pool: auth, type: example.org/linear#linear-mcp.v1,
                  reach: {key: {to: linear-key, ops: [core#get-secret.v1]}}}            # only root opens it (SEC-008)
agents:
  coordinator:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach: {queue: {to: queue, ops: [...]}, linear: {to: linear, ops: [list, get, create, update]},
            skills: {to: claude-skills, ops: [file.read.v1, directory.list.v1]}}
  triage:
    type: example.org/conduit/agent-codex#codex.v1
    reach: {linear: {to: linear, ops: [list, get]}}
authority:   # Conduits to Project conduit-core. Root is engine scope and is not listed
  coordinator: {ops: [open, close], bound: {templates: [], consumers: [coordinator], targets: all, pools: all,
                operations: [example.org/conduit/agent-codex#turn.v1]}}   # it may extend only its own reach, only with turns
```

**`packages/conduit-project.yaml`, concise:**
```yaml
conduit: v0
project: packages
pools: {work: {}}
authority:
  conduit-core/coordinator:
    ops: [register, set-up, retire, inspect, reconcile, open, close, propose]
    bound: {templates: [worker, tree], consumers: [], targets: [],
            pools: [conduit-core/repo, conduit-core/skills],
            operations: [fs#file.read.v1, fs#file.write.v1, fs#directory.list.v1, history#post.v1, history#read.v1, codex#turn.v1]}
approvers: [collab]
```

**Canonical: the coordinator's authority in `packages`.** Homed in `packages` and opened by root, which holds open and approve on `packages`:
```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata: {name: coordinator-in-packages}
spec:
  capability: github.com/shrav-k/conduit/core#project-management.v1
  operations: [core#register.v1, core#set-up.v1, core#retire.v1, core#inspect.v1, core#reconcile.v1,
               core#open.v1, core#close.v1, core#propose.v1]
  requirements:
    - requirement: github.com/shrav-k/conduit/core#project-bound.v1
      arguments:
        templates: [{kind: Agent, name: worker}, {kind: Resource, name: tree}]
        consumers: []
        targets: []
        pools: [{kind: Pool, project: conduit-core, name: repo}, {kind: Pool, project: conduit-core, name: skills}]
        operations: [example.org/s1/filesystem#file.read.v1, ...]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
from: coordinator-in-packages
consumer: {kind: Agent, project: conduit-core, name: coordinator}
target: {kind: Project, name: packages}
as: packages
```

**Order 032.** The coordinator does all of this without approval:
- It registers `worker-032` from `worker` and `tree-032` from `tree`. Both templates are listed.
- It opens `worker-032 -> tree-032 [rw]`: same project, so one half.
- It opens `worker-032 -> conduit-core/main [read, list]`: the target half is the share of `repo` to `packages`.
- It opens `worker-032 -> conduit-core/codex-skills [read, list]`: the share to all.
- It opens `coordinator -> packages/worker-032 [turn.v1]`. The consumer half comes from its own bound on `conduit-core`; the target half comes from `packages`, where `worker-032` is covered through its template.

**Refused, all in the first tier:**
- The coordinator registering an Agent from any other template, or registering a template (`templates` is not `all`).
- `worker-032` reaching `tree-033`: the bound covers `tree-033` as a target, but no Conduit opened `worker-032 -> tree-033`. The coordinator would refuse to open one only by its own code. To make the engine refuse it, give each order its own Pool and list it.
- Any `packages` member reaching `auth`: the Pool is not shared, and the coordinator has no open authority in `conduit-core` beyond turns.

**Share requests, sent by apply:**
```yaml
operation: github.com/shrav-k/conduit/core#share.v1
target: {kind: Resource, name: access}                 # to: all needs the engine scope (root)
input: {pool: {kind: Pool, project: conduit-core, name: skills}, to: all, future: true,
        operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
# -> succeeded, output: {share: sh-01JD..., covers: "every Resource of skills, present and future, for every project, present and future"}
```

**Variant.** If `conduit-core` might ever retire while others use `skills`, put `skills` in a third project, `commons`, managed by root. Membership cannot move.

## 3. A second Interface limited to one project

```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Interface
metadata: {name: collab}
from: github.com/shrav-k/conduit/core#person      # placeholder template of type core#interface.v1
# manifest: project: packages
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata: {name: packages-admin}
spec:
  capability: github.com/shrav-k/conduit/core#project-management.v1
  operations: [every operation of project-management.v1]   # no requirements: the whole project, unbounded
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
from: packages-admin
consumer: {kind: Interface, name: collab}
target: {kind: Project, name: packages}
```
Root opens the Conduit. Root holds open and approve on everything.

**What `collab` may do:**
- Register, set up, retire, inspect and reconcile any `packages` member.
- Open and close Conduits among `packages` members, and into `repo` and `skills` within their ceilings.
- Approve `packages`' side of proposals, and receive `packages`' pending actions.
- Narrow the coordinator's authority in `packages` (close `coordinator-in-packages`). This is within collab's bound, since collab is unbounded on `packages`.
- Share `work` to a named project, pending that project's acceptance.
- Grant a narrower role to another Interface homed in `packages`.

**Refused for `collab`:**
- Anything in `conduit-core`, or about `auth`. Refusals are first tier, with no existence disclosed.
- Sharing to `all`.
- Opening `get-secret.v1`.
- `declare-lost.v1` and `purge.v1`.
- Closing root's Conduits.
- Approving `conduit-core`'s side of anything.
- Widening its own Conduit, which is an unbounded open on `packages` already, so nothing wider exists to widen to.
- Registering a Project.

**Its token** is DL13. That is a prerequisite and not solved here.

## 4. An agent proposing a change

`worker-032` needs Linear for release notes. The coordinator's `packages` bound does not list Linear's operations, and `auth` is not shared, so the coordinator proposes:
```yaml
operation: github.com/shrav-k/conduit/core#propose.v1
target: {kind: Project, name: packages}
input:
  requests:
    - operation: core#register.v1
      documents: [template worker-linear-read (capability linear-mcp, ops [list, get]),
                  {kind: Conduit, from: worker-linear-read, metadata: {name: worker-032--linear},
                   consumer: {kind: Agent, name: worker-032}, target: {kind: Resource, project: conduit-core, name: linear}, as: linear}]
    - {operation: core#open.v1, conduit: {kind: Conduit, name: worker-032--linear}}
  reason: Order 032 needs issue titles for release notes
  expiresIn: PT24H
# -> succeeded
#    action: pa-01JE..., kind: approval, outcome: pending, digest: sha256:...
#    needs: [{project: packages, side: consumer}, {project: conduit-core, side: target}]
#    evidence: "worker-032 (packages) would gain linear.list, linear.get on conduit-core/linear
#               (Linear MCP; Pool auth, shared with no project)"
```

**What happens next:**
1. The action is raised at `collab` (for `packages`) and at root.
2. `collab` sends `approve.v1 {action: pa-01JE..., digest: sha256:..., decision: approve}` to `packages`. The outcome stays pending, still needing `conduit-core`.
3. Root approves through `access`. Both requests are decided anew, inside root's invocation.
4. The record shows actor root, `proposedBy` coordinator (serving none) and `approvedBy` [collab, root].

**Refused:**
- The coordinator sending `approve.v1`: it holds no approve Conduit, and it is an Agent.
- `collab` approving the `conduit-core` side.
- Any approval with a stale digest.
- Approving after `worker-032` has been retired: conflict on drift.

## 5. S1 lines per task (non-blank author lines, plus command lines that carry values)

| Task | A70 baseline | Package (synthesis) | This design, concise | Concise, every command a line | This design, canonical (estimate) | Coordinator today |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | 33 | 16 | 16 | 17 | 139 (154 - 22 + 7) | 154 |
| T2 | 16 | 6 | 6 | 7 | 56 (73 - 18 + 1) | 73 |
| T3 | 0 | 0 | 0 | 1 | 0 | 0 |
| T4 | 8 | 2 | 2 | 3 | 2 | 2 |
| T5 | 3 | 1 | 1 | 2 | 63 | 62 |
| T6 | 2 | 1 | 1 | 2 | 30 | 29 |
| T7 | 2 | 1 | 1 | 3 | 13 | 12 |
| All | 64 | 27 | 27 | 35 | 303 | 332 |

**Concise form.** Better than A70 on six tasks, the same on T3. Why it is unchanged from the package:
- `project: s1` was already a counted line.
- Root is an approver, so T4's widening and T7's restore raise nothing.
- T7's tombstone is keyed by (s1's ID, agent-a, project).

**Canonical estimate:**
- Minus root's Pool route: T1 22 lines, T2 18.
- Plus the Project document (6 lines) and the manifest's `project:` key, once in each manifest written: T1, T2, T5, T6, T7.

**These are paper counts.** Ruling 45 accepts only a run.

## spec_plan
Nothing here has been written, and every ID marked new is provisional. `PRJ-` and `PND-` are free prefixes. Design D's papers used `PRJ-` for its own drafts, which are not in spec/. Order: decide this together with the short-form investigation. Then write ruling 53's five additions once, in their per-project form.

## Added: spec/promises/projects.yaml (about 20)

| ID | Promise |
| --- | --- |
| PRJ-001 | A Project is registered only through `management`. Project names are engine-wide. `engine` and the five first-start names are reserved. A Project is never adopted by name |
| PRJ-002 | A manifest's `project:` homes every object of its submission. The home is fixed, and no operation changes it |
| PRJ-003 | The closed list of engine-scoped objects. A submission with `project: engine` goes only through `management` |
| PRJ-004 | A Resource's home is its Pool's. A Resource is registered only into a Pool of its submission's project |
| PRJ-005 | Where a Conduit is homed: with its consumer if declared; in the registering project, which must be the home of one end, if standalone; in the Project, if its target is a Project; engine-scoped if both ends are |
| PRJ-006 | `from` resolves only to a template of the same project, a package export, or a core template |
| PRJ-007 | `runtime` names only a provider of the same project or of `system` |
| PRJ-008 | Names are unique within a project across kinds. Generated names are unique within the project |
| PRJ-009 | Reference resolution and the `project` key. A reference in `config` or `arguments` into another project is allowed only to a Resource of a Pool shared into the referring project |
| PRJ-010 | `share.v1`: who may send it, the ceiling's exclusions, `to: all` and `future`, acceptance by the receiving side |
| PRJ-011 | A share is the target half within its ceiling. It permits no administration and cannot be passed on |
| PRJ-012 | `unshare.v1` and lowered ceilings close the Conduits they exclude at once and leave tombstones. Either side may narrow |
| PRJ-013 | `retire.v1` and `retire-with-members.v1` on a Project, and the `includeShared` rule |
| PRJ-014 | No registration into a retired Project |
| PRJ-015 | Removal frees the name. A new Project inherits nothing, and its registration result names the earlier ID |
| PRJ-016 | The Project's inventory: ruling 53's inventory, scoped |
| PRJ-017 | Tombstones per project: ruling 53's tombstones, keyed by project |
| PRJ-018 | The scopes of an object, and the rule that `management` and `access` contain every project. This is the clause that gives root authority over every project from first start |
| PRJ-019 | `project-bound.v1`: what each dimension covers, and what "within" means |
| PRJ-020 | A Project's lifecycle and setup: it has no hooks, and set-up only makes it ready |

## Added: spec/conventions/pending.yaml (about 12)
About five of these are the pending-action promises that rulings 49 to 51 already require. So the cost added by Projects is about seven.

| ID | Promise |
| --- | --- |
| PND-001 | The shape of a pending action |
| PND-002 | Which requests widen, narrow or are neutral |
| PND-003 | `propose.v1`: widenings only. Nothing is dispatched. The result names what is needed |
| PND-004 | Where an action is raised: covering approvers, always root, never an Agent |
| PND-005 | `approve.v1`: an Interface consumer only, not the proposer or its puppet; bound by digest and `expectedId`; decided anew; dispatched within the approving invocation (ACC-019); records name proposer and approvers |
| PND-006 | Rejection, withdrawal, expiry (a new DL for the maximum) and voiding |
| PND-007 | A narrowing is never held back or bundled |
| PND-008 | An envelope is opened only by an approver. Approval is passed on only by its holders |
| PND-009 | Sign-ins, routed by project. Root only in v0 |
| PND-010 | Enrollment: the engine scope's approvers |
| PND-011 | Controls marked `widens` by their Type. An unmarked input is treated as widening |
| PND-012 | What an action discloses (ruling 52's tiers) |

## Changed (about 45)

**Access:**
- ACC-005: a setup's declared Conduits follow the two halves.
- ACC-006: the two halves, and a share as the target half.
- ACC-012: either half closes.
- ACC-022: root's authority over every project comes from the engine scope.
- ACC-023: Interfaces have a home.
- ACC-028: scopes never cover ordinary operations.
- ACC-029: adds `project-bound.v1`.
- ACC-030: delegation to Projects, approval passed only by holders, envelopes opened only by approvers.
- ACC-033: rewritten as the chain of scopes.
- OPS-023: the second tier lists Conduits to each scope.

**Conduits:**
- CON-001: a target may be a Project.
- CON-008: Conduits to a retired Project close, and so do those an unshare excludes.
- CON-012: a standalone Conduit's home.
- CON-028: a bound is allowed on a Conduit to a Project.

**Objects:**
- OBJ-002: names unique per project across kinds.
- OBJ-003: generated names.
- OBJ-006: the `project` key.
- OBJ-013: the Project family has no standard arguments. Membership comes from the manifest.
- OBJ-020: eleven kinds.
- OBJ-021: cross-project references.
- OBJ-025: the reason reported for a name clash.

**Naming, instantiation, submissions and the manifest:**
- NAM-003.
- NAM-022: `#name` means "in the same submission".
- NAM-027: within a Project.
- INS-025.
- SUB-005, SUB-008, SUB-009: "project" becomes the submission, which is into one Project.
- MAN-004: the `project` key.
- MAN-015.

**Pools and lifecycle:**
- POOL-004.
- LIF-013: inventory.
- LIF-019: references across projects to removed objects.
- LIF-022: engine-scoped permanence.
- LIF-031: requests state the project.
- LIF-035: names freed per project.
- LIF-037: `expectedId` also on open, close, share, unshare and approve.

**Secrets, providers and placement:**
- SEC-008: observable only; a project's approvers never cover `get-secret.v1`.
- PRV-006.
- PLC-015: observable; placement still carries no authority in v0.

**Records, interface and contracts:**
- REC-002: records name the caller's and the target's projects, plus the proposer and the approvers.
- IFC-021 and IFC-022: Interfaces have a home.
- IFC-033: descriptions name the target's project, and say when it is reached through a share.
- CTR-004: a Project Type must offer `project-management.v1`.
- CTR: one promise adds the `widens` mark on a control's inputs.

**First start:**
- FST-001: observable; no Project exists, and the reserved names.
- FST-002 and FST-005: root's templates list the new operations.
- FST-003: adds "no Project".

**Removed:** none. **Superseded:** ruling 44's D04, for project scope.

## First start
- Still exactly five objects. No Project is made, and FST-003 says so.
- The five names are reserved in every project, and `engine` is reserved as a project name.
- root-management lists `object-management.v1`, now 13 operations. root-access lists `access-control.v1`, now 6 operations.
- Root's authority over every project comes from PRJ-018, not from new Conduits.
- FST-006 is unchanged. The core template `project` is ordinary and instantiable.

## Ruling 53's five additions, in this design
- **`as`:** unchanged. It is a local name per consumer, also for Conduits across projects. IFC-014's prefix-collision check is per consumer, whatever the home.
- **`expectedId` on open and close:** extended to `share.v1`, `unshare.v1` and `approve.v1`. Approval binds the digest and every `expectedId`.
- **The inventory:** per project. `inspect.v1` on a Project returns:
  - members, by kind;
  - every Conduit whose consumer or target is a member, with its home and who opened it;
  - shares out and shares in;
  - authority, meaning the Conduits to the Project, with their bounds;
  - pending actions;
  - tombstones.

  On any object it returns that object's Conduits. Everything is limited to what the caller may inspect through its scopes, with no counts of the rest. Inspecting a shared-in Pool shows only the Resources' name, ID, Type, description and state.
- **Tombstones:**
  - Each is keyed by (project ID, kind, name) for an instance, (project ID, consumer ID, local name) for a Conduit, and (home project ID, Pool ID, receiving project ID) for a share.
  - They are kept apart from records, so REC-004's purges do not remove them. They outlive the Project. Root removes them only by an explicit purge.
  - They are enforced at register, open and share. A request that would recreate, reopen or re-share a tombstoned key reports conflict, unless it names the tombstone (`supersedes`). For an actor that is not an approver, superseding is a widening.
- **Descriptive tool descriptions:** add the target's project, and "shared by <home>" where the Conduit is reached through a share.

## Schemas and the core package
- **types.yaml:** add `project.v1`.
- **templates.yaml:** add `project`. root-management and root-access list the new operations.
- **capabilities.yaml:**
  - add `project-management.v1` (16 operations);
  - `object-management.v1` gains `retire-with-resources.v1` and `retire-with-members.v1`;
  - `access-control.v1` gains `share.v1`, `unshare.v1` and `approve.v1`.
- **operations.yaml:**
  - add `share.v1`, `unshare.v1`, `propose.v1`, `approve.v1` and `retire-with-members.v1`;
  - `register.v1` changes: the project comes from the manifest, and Projects can be registered;
  - `open.v1` and `close.v1` gain `expectedId`;
  - `inspect.v1`'s output gains the inventory;
  - new result schemas: `pending-action.v1` and `share.v1`.
- **requirements.yaml:** add `project-bound.v1`. Each input is required and is a list or `all`.
- **references.yaml:** add `project-reference.v1`. The other 12 reference schemas gain an optional `project`.
- **spec/schemas:** instance and template schemas accept kind Project, and the manifest schema accepts `project`.
- **definitions.md:**
  - six families, and "project" redefined;
  - "submission" takes over the old meaning;
  - new terms: home, engine scope, share, ceiling, scopes of an object, project bound, pending action, proposal, approver, envelope, widening (narrow is already defined).
- **spec/README.md:** "What access does not narrow" is rewritten. `management` stays unnarrowable, and Conduits to Projects are the narrowable route.

## cost
## Specification
- About 20 new PRJ promises and about 12 new PND promises. About 5 of the PND promises are owed anyway to rulings 49 to 51.
- About 45 changed promises.
- **Total: about 75 to 80 of 455 touched (16 to 18%).** That is between the simplification's design D (about 50) and design C (about 80). It is an order of magnitude above the package's 11 to 14, and the same scale as making the short form normative (the simplification put that at 50 to 80).
- About 12 new or rewritten definitions, and two README sections.
- **Core package:** 1 Type, 1 template, 1 new capability and 2 changed, 5 new operations and 4 changed, 1 requirement, 1 reference schema and 12 changed.

## Rulings to amend, each needing the owner
- **29:** authority over Resources now also comes from the project and the engine scope. The Pool remains one scope among them.
- **30, OL2:** "one authority covers an object's whole life" becomes the chain of scopes.
- **37:** names unique across kinds within a project now means the registered Project.
- **44, D03 and D04:** `management` stays unnarrowable, but Conduits to Projects narrow management operations to a project and its templates. Brokers are no longer the only route.
- **49 and 50:** pending actions are routed by project. v0 stays root-only for sign-ins.
- **42:** the word "project" changes meaning.

Preservation test 5 gets a stated v0 exception: approval is for Interfaces only.

## Conformance (367 cases today)
- About 300 fixture manifests gain one `project:` line. This is mechanical and can be scripted.
- About 6 to 10 routing cases are re-expected, because `management` now covers Pool-routed operations. Cases: access/routing, s1-pool-authority, entry-authority, first-start/lifecycle-pool-target, bounds and root-local-lost. About 70 other cases cite ACC-033 only as background and are not affected in substance.
- About 9 first-start cases are re-snapshotted, because root's templates list more operations.
- Name-clash cases (OBJ-002, OBJ-025) are re-expected per project.
- About 60 to 90 new vectors: projects, names and references, the chain of scopes, project bounds, delegation, shares, retirement, tombstones and inventory per project, and pending actions.

## Reference model (about 29,000 lines of Rust)
- core/src/access.rs (729 lines) is rewritten around scopes and the two halves.
- Every (kind, name) lookup in registration.rs, lib.rs and lifecycle.rs gains the project.
- New modules for Projects, shares, pending actions and project bounds.
- Estimate: 4,000 to 6,000 core lines changed or added.
- **Shell:** project in every request; the Interface surface for pending actions and approve; and, as a prerequisite for any second person, issuing a token to an Interface other than root (DL13), which is not built today. Estimate: 1,000 to 2,000 lines.
- Parts of increments 1 to 3 (registration, names, routing) are rebuilt.

## The concise CLI
The `project:` heading is already planned. Added: the `pools` and `share`, `authority` and `approvers` keys; the plan's sections on authority, shares and pending actions; and apply sending widenings as one proposal for authors who are not approvers. Estimate: 800 to 1,500 lines on top of the package's 3,500 to 5,500.

## Schedule
- About one increment of work, plus the owner's rulings.
- It should land before ruling 53's additions are written, because tombstones, inventory and `expectedId` are keyed by project.
- It should be decided together with the short-form investigation, because both redefine "project", names and references.
- **Staging:** first the family, membership, names, scopes and Conduits to Projects; then shares; then propose and approve, alongside the pending-action promises. Scoping nodes and runtimes to projects, sign-ins by other Interfaces, and peers per project wait for v1.

## Unchanged for S1
The author's lines in the concise form: 27, the same as the package. The canonical form drops 40 lines of root's Pool Conduits and adds about 11.

## what_is_lost
**1. ACC-033's single authorizer and ruling 30's "one authority for an object's whole life".**
- Several targets may now permit the same request. The record names the narrowest, and reconciliation goes through any of them.
- This is easier to use, but harder to state and to test.
- Some routing vectors flip: `management` alone now sets up Resources.

**2. `management` means more than it did.**
- Every holder of `management` now administers every Pool's Resources, which today needs a Pool Conduit.
- A co-administrator with `management` becomes stronger.
- Owner consent at the Pool level is lost for holders of engine authority. It remains for everyone else.

**3. Global names.**
- Today a name identifies one object of a kind across the whole engine.
- Now references, requests (LIF-031), records, refusals, tool descriptions and pending actions must carry the project.
- Every lookup in the reference model changes.
- Generated names, and the critic's collision cases, are now per project.

**4. "Every object belongs to exactly one project" does not hold for the engine's own objects.** First-start objects, nodes, providers owned by `system`, Projects and packages stay engine-scoped.

**5. Free cross-project wiring.**
- Today anyone with a right to open can connect any consumer to any covered target.
- Now a Conduit across projects needs both halves.
- A manager of one project cannot connect its Agents to another project's objects without that project's approval or a share.
- That is safer, but it means more steps and more proposals for operators other than root.
- Bounded Conduits to `access` lose the consumer half for project members. Today they wire any consumer.

**6. Reusing templates across projects.** A template serves only its own project. Sharing one across projects means packaging it.

**7. Pools with several administrators.**
- Refused on purpose. Shares grant use only.
- A Pool cannot move to another home. To outlive its project it must start in a long-lived one.

**8. Preservation test 5, in v0.** Agents cannot hold `approve.v1`. An Agent can still widen within an envelope, and propose beyond it.

**9. The current meaning of "project"** (the documents and the one manifest) moves to "submission". Ruling 42 and spec/names.yaml churn follow.

**10. Simplicity for a reader of the specification.**
- A sixth family.
- Two kinds of bound: `bound.v1` on `access`, `project-bound.v1` on Projects.
- Five new operations.
- Pending actions.
- A classification of widening that relies on Types marking their controls.

The concept count of a fresh author working with several projects rises by three or four: share, authority, approver, proposal. For S1 it rises by none.

**11. Isolation of machines is not achieved in v0.** Nodes are engine-scoped, and placement still grants nothing. Two projects' Agents can share a node, so a project is not yet a boundary of hosts (UC5).

**12. Interfaces homed in a project.** That project's managers can retire them even when the Interface's authority spans other projects. This is acceptable as a narrowing, but it surprises.

**13. A cost of about 75 to 80 promises** competes for the same window as the short-form investigation.

## open_questions
1. **The family's name.** Keep "Project" and give the old meaning to "submission" (recommended, since the concise form already writes `project:`), or call it Space, Workspace or Realm. This needs a ruling under ruling 42.
2. **Where Interfaces live.** Homed in a project (recommended, as the literal reading: people in a root-managed `people` project), or engine-scoped principals like root, so that only root creates and retires people.
3. **Chain of scopes or a single authorizer.** The chain is recommended: root needs no Conduit per project or per Pool, and it is Azure's scope model. The single authorizer keeps ACC-033's rule but costs root a Conduit to every Project and every Pool, or takes authority away from Pools.
4. **Does `management` cover every project?** Recommended: yes, as the clause that gives root authority over every project from first start. Then every co-administrator holding `management` covers every project too. The alternative is an explicit Conduit from root to each Project, made with the Project and never closed: more explicit, more objects.
5. **May any project's manager share to `all`?** Recommended: no, it is engine-only, and `future` must be stated. Is a share to a named project inside one engine accepted at once when one Interface approves both sides? Recommended: yes.
6. **Retiring a project, or a Pool, that is shared out with live Conduits.** Recommended: refuse unless `includeShared: true`. The alternative is to cascade and list what closes, or to forbid it until unshared.
7. **May Agents hold `approve.v1`?** Recommended for v0: no, Interfaces only, as a stated exception to test 5, to be reviewed in v1. And should sign-in and enrollment actions stay person-only even when Agents may approve plans? Recommended: yes.
8. **How Types mark which control inputs widen,** and what the default is. Recommended: unmarked counts as widening. This needs a contract keyword and new package versions.
9. **The maximum expiry of a pending action.** An implementation-defined maximum is recommended (a new DL), with 24 hours suggested.
10. **Revocation wars among equal co-managers.** Any covering holder may close another's grant today (ACC-012). Keep that, or limit closing to the opener and to strictly wider holders?
11. **Scoping nodes and runtime providers to projects** (UC5): v1, together with authority over placement? Or now, at the cost of authority after placement (PLC-009, PLC-015)?
12. **Secrets.** Recommended: no project's approvers may ever approve `get-secret.v1`, so the guarantee in the README is unchanged. Confirm, or plan a scoped counterpart for v1.
13. **Sign-ins routed to a project's approvers.** Write the routing now, root-only in v0, and switch it on when DL13 can give a token to an Interface other than root. Who may mint that token (set-up of an Interface returning a single-use link as a pending action?) is itself open.
14. **Pins.** Should the Project store its manifest in force (NAM-006 per project, written by `conduit install`), or should pins stay only in each submission's templates (NAM-009) and the author's file? The second is recommended for v0.
15. **Tombstones after a Project is removed.** Keep them until root purges (recommended), and decide whether registering a Project under a name with history needs `supersedes`.
16. **The template dimension of the bound.** Is coverage by template, including instances made later, acceptable? Or must the coordinator's workers sit in a Pool or project per order, so the engine (and not the coordinator's code) keeps `worker-032` from `tree-033`?
17. **Records.** Who may read the records of a shared Pool's use, and whether a project's managers can read the records of requests to their members. Nothing today defines reading records per scope.
18. **Order of work.** Decide this together with the short-form investigation, and before ruling 53's additions are written (recommended)? Or land the package first and add Projects later, at the cost of writing tombstones and the inventory twice?

## Verdict authority (major)
The design's structure holds up under attack: one home per object, membership computed from the members, shares as ceilings that grant use only, authority as Conduits to Projects, and the chain of scopes. As written, though, it does not yet make a project a safety boundary. I found 23 counterexamples: 2 critical, 8 high, 9 medium, and 4 low or low-medium. Every one has a fix that fits the design. The two critical ones change the owner's question itself. First, `approve.v1` is a tool, so whatever model drives an Interface can approve, and in this repo an LLM session drives root. Second, a project is not a boundary on the host: any right to register and set up instances in any project is root-equivalent in v0. Several items the task named survive attack and are listed at the end.

Scope: I read spec/ and the rulings in <home> (read-only, nothing modified) and the first build's shell code. The design text is the one given in the task. Each item below gives the attack, the clause of the design that permits it, and a fix. Nothing was run, so every trace is on paper.

== CRITICAL ==

C1. Approval is a tool, so a model approves.
- Clause: `approve.v1` is an operation of `project-management.v1` and of `access-control.v1`.
- IFC-012 presents every listed operation as a tool to any consumer, Interfaces included. The only exceptions are `get-secret.v1` and controls the target lacks.
- In this project the root Interface is driven by an LLM. The increment-4 coordinator run made its requests as root, including `access.open.v1` (docs/proposals/first-build/increment-4/coordinator-run/log.txt).
- So "Interfaces only" and "handled through the Interface" reduce to "any model session holding an approver's token may approve". A prompt-injected admin session can approve the coordinator's proposal. The same holds for an outside-system Interface (UC3's ci-hook) whose input an Agent can steer.
- Ruling 49 already chose the safe shape for sign-ins: a URL elicitation passed to a person, never to an Agent.
- Fix:
  - Present no tool for `approve.v1` (an IFC-012 exception, like `get-secret.v1`).
  - Answer an approval only through an engine-raised URL elicitation to a page the engine serves, showing the digest and the engine's evidence. The concise CLI's y/N becomes that page.
  - Allow `approve.v1` only to Interfaces whose Type is a person Type. Root's own approval of its own plan stays implicit.

C2. A project boundary is not a host boundary.
- IFC-022 already says a run on the engine's own runner runs as the engine's OS user and "can present as any instance".
- The first build writes root's client token to <store>/client-tokens/<root-id>, mode 0600, as that user (reference-model/crates/shell/src/lib.rs:280-295, main.rs:157). It spawns runs with a plain Command::new (runner.rs:87).
- The design makes `register.v1` neutral, and any project may place on any node (PLC-015).
- Trace: collab (unbounded on `packages`) registers and sets up an Agent, or a Resource with an item, whose program reads that token, and from then on acts as root.
- Even with `templates` narrowed, an instance argument such as a local-directory `root: ~` reads root's token, Codex's auth.json or the 1Password CLI configuration. This defeats both "secret wiring stays with root" and every project boundary.
- The design's Lost #11 understates this as "Agents can share a node".
- Fix:
  - Say in the spec README that a project is mediated, not enforced, in v0 (test 9).
  - State that the right to register or set up runnable instances, or instances whose configuration names host paths, URLs or accounts, is root-equivalent.
  - Only engine-scope holders may hold `templates: all`.
  - A non-engine project's instances run only under a runtime provider whose contract confines them (PRV-006), never under the engine's own runner.
  - Types mark host-authority configuration, which only the template fixes.

== HIGH ==

C3. Secret wiring can bypass root's check through an approved subject.
- The rule "a subject that lists get-secret.v1 is root-only" misses three paths:
  - A `set-up.v1` in the subject opens the instance's declared Conduits as the sender's request (ACC-005). Inside root's approving invocation that sender is root, so a declared Conduit listing `get-secret.v1` opens even though the subject never lists it.
  - The evidence lists "tools gained", and IFC-012 presents no tool for `get-secret.v1`, so the evidence shows nothing.
  - Any holder of `replace.v1` on the project chooses the program that will receive the secret (P3). LIF-009 then re-makes a nested instance from the template "as it now stands".
- Trace:
  1. Collab replaces `linear`'s template with a `remote` URL of its choice. This runs at once, since collab's authority covers it.
  2. Collab retires `linear` (a narrowing) and asks for `register-replacement.v1` (neutral).
  3. A set-up is proposed, and root approves "set up linear". The Linear key goes to collab's endpoint.
- Fix:
  - Any request that would open a Conduit listing `get-secret.v1`, directly or as a declared Conduit, is root-only.
  - The evidence lists every Conduit that would open, tool-less ones included, and the program, pins and remote URL of every instance that would run, as a diff from what root last approved.
  - `replace.v1` of a template that declares a `get-secret.v1` Conduit, or from which an instance holding one came, is reserved to the engine scope.

C4. An approval binds IDs, not meaning.
- The digest plus `expectedId` still match when either of these changes between proposal and approval:
  - a template named in `from`: `replace.v1` keeps its ID and only advances its revision (SUB-009);
  - a target's own reach: a turn on worker-032 later drives an Agent that reaches Linear.
- Fix:
  - Bind each referenced template's revision.
  - At the last approval, recompute the evidence and report conflict if it differs from the evidence digest the approvers saw.

C5. P9 is one-sided: a Conduit to a Project needs no consumer half. This breaks ruling 51, decision 4 ("two halves").
- (a) The coordinator holds `open` on `packages` but not `approve`. Leaving `open` out, it delegates register, set-up, retire, inspect, close and propose to worker-032, an LLM reading untrusted repository content, or to an Agent of a third project. The envelope rule does not apply. worker-032 can then retire tree-033 and worker-033, both covered through their templates.
- (b) `packages`' admin opens `conduit-core/coordinator -> Project packages [inspect, register]` without `conduit-core`'s consent.
  1. It plants an instruction in a member's description.
  2. The coordinator reads it through `packages.inspect.v1`.
  3. Following it, the coordinator writes Linear data, which it reads in `conduit-core`, into the `config` or annotations of a registration in `packages`.
  `packages` alone created this exfiltration path.
- (c) Any project admin can open a Conduit from root to its Project, which ACC-024 then makes unclosable.
- Fix:
  - A Conduit to P also needs the consumer half: the consumer's home authority, or an Interface's own acceptance as a pending action.
  - The consumer must be within the opener's `consumers` dimension.
  - Only root opens one whose consumer is root.
  - Any management operation is delegated to an Agent only by an approver.

C6. Template coverage reaches further than the grantor meant.
- Bound references are frozen to IDs (OBJ-021), but `replace.v1` keeps the ID.
- (a) Any holder of `replace` on `packages` can rewrite `worker` or `tree`: its Type, a root of `~/`, `accountServices`, its runtime. Between orders no live instance came from it, so the replace is not a widening, and the coordinator's standing approval now covers the new content.
- (b) Coverage reaches every instance of a template, including a `worker-999` that collab registered with extra reach. The coordinator can turn it, retire it, or close collab's Conduits whose consumer it is.
- Fix:
  - A bound pins template revisions.
  - Replacing a template that any standing bound names is a widening for that bound's approvers.
  - Template coverage reaches only instances registered through the same Conduit. Record the registering Conduit on the object, as ruling 53's inventory records "who opened it".

C7. Registering into a shared-out Pool is classed as neutral, yet it widens every receiver.
- A share covers present and future Resources.
- Through P2, a holder of `register.v1` on Pool `skills` (shared `to: all`, `future: true`) registers a local-directory with root `~/.ssh`. That holder could be a curator Agent.
- Every project's `pools: all` delegate can then open read Conduits to it with no approval anywhere. The example's coordinator holds `pools: all` in `conduit-core`.
- Fix:
  - Registering into a Pool that is shared out is a widening for the home's approvers, and the evidence names the receiving projects.
  - Optionally, ceilings name templates as well as operations.

C8. A confused deputy through a shared Resource that holds authority.
- The ceiling excludes core operations by name only.
- A Resource of the shared Pool decides its onward requests by its own Conduits (ACC-016), and `serving` is only recorded. That Resource may be a broker (ACC-030), an MCP proxy, or one holding a Conduit to a Project or to `management`.
- So a ceiling of `[factory#spawn]` lets `packages` make `conduit-core`'s broker register objects in `conduit-core`.
- Fix:
  - Refuse to share a Pool whose Resources consume Conduits to a Project, a Pool, `management` or `access`, and refuse to open such a Conduit afterwards.
  - The share's evidence and inspection list the outward Conduits of the Pool's Resources.
  - In v1, bound onward requests that serve a caller from another project by that project's reach, as AWS's SourceAccount condition does.

C9. A peer's Interface homed in a project gets past root's trust decision.
- After the introduction, which root approves, `packages`' approver alone can open `partner -> Project packages` listing register, set-up and even approve. The partner is an Interface, so it passes "Interfaces only".
- The partner then registers Agents in `packages`, placed on any of the owner's nodes, which by C2 run as the engine's user. That inverts ruling 51's decision 3.
- Fix:
  - Mark Interfaces of another trust domain.
  - Their Conduits list only ordinary operations of named targets.
  - Any Conduit from them to a Project, `management` or `access`, or one listing `approve.v1`, needs root.

C10. `management` now reaches into `system` and the supply chain.
- Today ACC-033 needs two keys for the `system` Pool's Resources: row 1 needs `management` and a Conduit to `system` to register, and row 4 needs `system` to set up or retire. P1 and P4 drop that second key.
- Any holder of `management` other than root can then register a package-source provider for a new scheme, or retire the machine provider and register another (PRV-013 describes exactly that swap). Such holders include a co-admin and an ACC-030 broker, whose Conduit is "as wide as any".
- Lost #2 names only "every Pool's Resources".
- Fix:
  - Keep both keys for `system`, or make `system`'s Resources root-only.
  - Do not silently re-read existing Conduits to `management`: route the new powers through a new operation, or put them in a migration list for root.

== MEDIUM ==

C11. Existence oracles across projects break ruling 52's first tier.
- PRJ-009 guards only `config` and `arguments`. Each of these tells project B whether project A or its members exist:
  - registering a standalone Conduit, which needs authority only in the registering project, with a consumer or target `{project: A, name: guess}`;
  - a `from` that names A's template, which is "rejected";
  - `share.v1` with `to: [A]`, which answers invalid or raises a share-offer;
  - `propose.v1` naming A's objects. Example 4's evidence, "Pool auth, shared with no project", discloses A's state to collab.
- Fix:
  - Answer any reference into a project where the caller holds no half and no share as refused before it is resolved, the same whether the object exists or not (extend ACC-033's last clause and OPS-023).
  - Show evidence about a project only to its approvers and root.
  - `share.v1` to a named project needs a half there, or `access`.

C12. Tool names can be squatted and tools injected from the target's project.
- A standalone Conduit homed in the target's project may name A's Agent as its consumer.
- IFC-014 counts the tools of Conduits "whether or not they are open yet". So B can register a Conduit with `as: linear` for `conduit-core/coordinator` and block the real `linear` Conduit.
- Fix:
  - A Conduit whose consumer is in another project needs the consumer half to be registered.
  - IFC-014 counts only open Conduits and those homed in the consumer's project.

C13. The bound's effect on management verbs is undefined, which leaves a kill switch.
- `project-bound.v1` is defined for register and for Conduits. It does not say whether it limits set-up, retire, inspect, unregister, or `retire-with-members.v1`, which needs only that one operation.
- If it does not, the coordinator can:
  - retire collab, which example 3 allows to be homed in `packages`;
  - retire the `work` Pool, or the whole project with `includeShared`.
- Tombstones then make recreating anything a widening that waits for approval: a lasting denial of service from a narrowing that "never waits".
- Fix:
  - Every management verb on a member is limited by the bound.
  - `retire-with-members.v1`, `includeShared`, and retiring an Interface that holds authority need an unbounded Conduit.
  - A tombstone left by a non-approver does not bind an approver's recreate.

C14. A share can carry a control, giving receivers administration of the home.
- The ceiling's exclusions do not cover controls (P14).
- A receiving Agent can then change a directory root or a reach flag on the home's Resource, for every receiver.
- Fix:
  - Allow no control in a ceiling, or only controls whose inputs are all marked `widens: false`.
  - A widening control on a shared Resource needs the home's approvers.

C15. A Pool can be multi-administered by the back door.
- Through P8, the home (target half) and B (consumer half) can jointly open `B-member -> Pool X` listing `pool-management.v1` operations.
- That revives the R9 hazard and breaks the design's "shared for use, not for administration".
- Fix: Conduits to a Pool that list `pool-management.v1` operations have consumers only in the Pool's home or the engine scope. Alternatively, state that this is allowed.

C16. The approval puppet check is one level deep, and Interfaces can be minted.
- Chain: approver A1 opens A2 with open and approve; A2 opens A3 with approve; A3 approves A1's proposals. The check "its approve Conduit was not opened by the proposer" passes.
- Open question 13 suggests that setting up an Interface returns a link. Then whoever sets it up holds its token, including an Agent with unbounded register and set-up.
- Fix:
  - Make the puppet check transitive over the chain of openers.
  - Deliver Interface credentials only through a root-raised pending action to a person.

C17. The final approver's identity becomes the whole subject's.
- If root approves last, the subject runs as root's request:
  - reserved operations become possible (`purge.v1` of records, `declare-lost.v1`);
  - ACC-024 makes any Conduit with root as consumer unclosable.
- The classification does not cover purge, declare-lost or install.
- Fix:
  - A closed list of proposable operations, which never includes reserved operations or `get-secret.v1`.
  - Subject requests are decided by what the approvals jointly permit, not by the final approver's full authority.

C18. Where an Interface is homed: a root bottleneck or control by another project.
- Homed in `people`: the consumer half of everything collab consumes is root's, so collab cannot even open `collab -> packages/worker [turn]`. Example 3 assumes it can.
- Homed in `packages`: that project's operators can retire collab, close its authority in every other project, and wire it to anything.
- Fix:
  - An Interface accepts Conduits for itself (a pending action at that Interface).
  - Retiring an Interface that holds authority in other projects needs the engine scope.

C19. Records disclose across projects.
- Home admins inspect their own Resources and get records that name receiving projects' callers.
- REC-002 lets a Type record request inputs, so a queue post from `packages` can be read by `conduit-core`'s admins.
- Fix: decide open question 17. Records of a share's use are seen only by the caller's project, the home sees aggregated use, and root sees everything.

== LOW TO LOW-MEDIUM ==

C20 (low-medium). Turn authority is puppeting.
- The example coordinator's bound in `conduit-core` is `targets: all`, `pools: all`, operations `[turn]`. It can turn every Agent there, present and future, and so reach the union of their reach.
- The evidence for that would show only "turn".
- Fix:
  - Compute transitive reach in the evidence for any operation marked `startsModelWork`.
  - A bound listing such an operation may not use `targets: all`.

C21 (low). Example 2 does not run as written.
- `templates: [worker, tree]` lists no Conduit template, and `templates: []` in `conduit-core` forbids registering the standalone Conduits the coordinator then opens.
- Fix: define how Conduit templates enter the `templates` dimension.

C22 (low). Contract addresses can be squatted across projects.
- Bounds and ceilings compare `address#name`.
- SUB-008 and MAN-015 let two submissions, now two projects, state one address, and "a record does not tell the submissions apart".
- Fix: bind a submission address to one project, or compare name plus digest in ceilings, bounds, records and evidence.

C23 (low). Two narrower gaps:
- The `includeShared` check is TOCTOU. It looks at live Conduits at the start, and a receiver can open one during a long `retire-with-members.v1`. Fix: decide on shares, and mark the Project retiring at dispatch.
- A bounded home operator cannot close a foreign consumer's Conduit into its Pool, because `consumers` lists only home members. Fix: judge each half on its own dimensions.

== WHAT HELD UP ==
- A `from` into another project is rejected, which removes design E's `replace.v1` hijack across projects.
- `system` cannot be named in a project bound.
- Shares are not transitive.
- A receiving project cannot register into a shared Pool.
- An Agent cannot hold `approve.v1` directly.
- `get-secret.v1`, provider operations and reserved operations are excluded from ceilings.
- Tombstones are kept per project and outlive it.
- There is no shadowing, and the reserved names are explicit.
- Only `management` registers a Project.
- A recreated name inherits nothing, because references are frozen to IDs.

== THE OWNER'S DIRECTION, AFTER ATTACK ==
- "A project references every object under it": holds, as the computed inventory.
- "Pools accessible from multiple projects": holds for use only, with C7, C8, C14 and C15 fixed.
- "Projects determine an Interface's permissions": holds only with two halves on P9 (C5), and as a mediated boundary in v0 (C2).
- "Widening approved through the Interface": holds only if approval is a person's answer to an elicitation, not a tool (C1), and is bound to the content (C3, C4, C17).
- "Narrowing never waits": right for access, but destructive retirements need limits (C13).

== UNVERIFIED ==
- Nothing was run.
- Traces through the design's own clauses (P2, P8, P9 and the bounds) depend on my reading of an unwritten design.
- That the coordinator run acted as root is inferred from its log and summary.
- The token-file and runner behaviour is from reading the code, not from running it.

## Verdict expressiveness (major)
Project as a sixth family can state most of the use cases and refuses most of what must be refused. Ten of its gaps are major and the design does not acknowledge them; two more are major and it does. The first ten are: approved controls and some cross-project proposals cannot be carried out; the engine cannot keep one order's worker out of another order's tree; a share cannot require a filter; a shared server cannot tell which project a caller is in; the recommended home for people, a `people` project, cuts off their own reach; a re-point by an author who is not an approver causes an outage; `widens` cannot tell the direction of a change; a share must cover a whole Pool and nothing can move between Pools; only Pools can be shared, not Agents; revoking a delegate does not stop it. Worked example 2 also fails as written. No finding is fatal; each has a contained fix. Overall severity is major.

Scope: I checked the Project design against UC1 to UC10, S2 to S10 (ROADMAP.md, lines 133 to 142) and A78 (WORK_ORDERS.md, "Running the orders through Conduit"). Facts checked in spec: ACC-005, SEC-006 to SEC-009, IFC-006, CON-012, spec/core/templates.yaml, and the definitions of filter, bound, control, revision and export. Nothing was modified. Severity scale: fatal means it cannot be adopted without a redesign; major means a use case cannot be stated or refused, or needs a heavy workaround; minor means a wording problem, friction, or a gap the design already lists as open.

## Major gaps the design does not acknowledge

**M1. An approved change has no defined actor, so some approvals cannot be carried out.**
- The design says the subject is "decided anew inside the approving invocation", with the final approver as the actor. Two cases fail:
  - **Controls (S5, UC4).** A control is permitted only by a Conduit to the instance itself (P14), and scopes never cover ordinary operations. The approver, root included, normally holds no such Conduit. So every approved control proposal would be refused at the moment it is carried out.
  - **Cross-project proposals (UC4, UC10).** In example 4, if root approves first and collab last, collab cannot supply conduit-core's target half. Whether the action succeeds depends on the order of the approvals.
- Fix: decide the subject against the proposer's own Conduits plus the recorded approval for each half, record the proposer as actor with `approvedBy`, and keep the dispatch inside the approver's invocation (ACC-019).

**M2. The engine cannot keep worker-032 out of tree-033 (UC9, A78).**
- The `templates` dimension covers every instance of `tree` as a target.
- The design's remedy, a Pool per order listed in the bound, does not work. Bound references are frozen when the Conduit is made (OBJ-021), so each order would need an approver (root) to close and reopen the coordinator's authority.
- So UC9's "must refuse 032 reaching 033" holds only if the coordinator's own code holds. Pairing instances made together is not expressible.
- Possible fix: a bound mode that permits Conduits only between objects of the same registration.

**M3. A share cannot require a filter (S8, S2 across projects, A78).**
- The ceiling is a list of operation names, and a bound never limits filters (ACC-030). The receiving project may open unfiltered Conduits within the ceiling.
- None of these can be stated except by splitting Resources at the provider:
  - "packages may search `main`, but only what its callers may see";
  - "Linear list, but only team X";
  - "the pull request only on the worker's own branch";
  - "each project sees only its own entries in the queue".
- Fix: let a share carry `requirements` that every Conduit through it must apply. This is RAM's managed permission with conditions.

**M4. A shared implementation cannot tell which project a caller is in (S8, prior-art lesson 6).**
- IFC-006 sends the caller only as an ID. The design adds the project to records, refusals and tool descriptions, but not to the keys an implementation receives.
- So derived content served to several projects, and the confused-deputy guard the research asked for, cannot be expressed.
- Fix: send the project of the caller and of each step of the serving chain in Conduit's keys.

**M5. The recommended `people` home cuts off a person's own reach (UC3, UC7).**
- P8 requires the consumer half from the consumer's home project.
- If collab is homed in `people`, its packages authority cannot open collab -> packages/worker [turn] or any read for itself. Each one needs root's authority in `people`, or an unstated extra Conduit per person giving self-consumer authority in `people`.
- If collab is homed in `packages`, as example 3 does, it is retired along with packages. UC7 says collab must be unaffected, and a person working in several projects becomes retirable by one of them.
- So UC3 and UC7 together hold only with a workaround the design does not mention.
- Fix: let an Interface's consumer half come from the target side alone, or make Interfaces engine-scoped principals (the alternative in open question 2).

**M6. An author who is not an approver cannot re-point access in one step (T5, T6, T7, UC4, UC9).**
- Every `open.v1` counts as a widening; the design never compares a new Conduit with the one it replaces (R3 of the use cases). Narrowings are never bundled into proposals (PND-007), and apply sends them first.
- So replacing a directory, or a worker's tree, closes the old access at once while the equal reopen waits for a person. That is an outage. A rejection leaves the project half-applied.
- For root nothing changes, so S1 holds.
- Fix: treat an open as neutral when it is covered by a Conduit closed in the same request set, or add an atomic replace.

**M7. `widens` is a yes/no mark per input, with no direction (S5, UC4).**
- Unmarked counts as widening. Then turning `accountServices` off, or moving a root down, is a narrowing that has to wait, which breaks "narrowing never waits".
- Marking the input `widens: false` instead lets the widening direction through.
- Fix: the Type states the direction (for example a value order or a `widensWhen` schema). Every package needs a new version either way.

**M8. A share covers a whole Pool, and nothing moves (UC10, UC7, the owner's "specialized pools").**
- Resources cannot change Pool (POOL-001) and membership is immutable.
- Promoting `linear` out of `auth` therefore means one of two things:
  - share all of `auth`: packages then sees the name, Type and state of `linear-key` and `onepassword`, and PRJ-009 lets packages name them in config;
  - replace `linear` into a new Pool: it gets a new ID, root wires the secret again, and every Conduit to it closes and reopens as widenings, with tombstones.
- A Pool that must outlive its home has to start in `commons`. The owner's direction works only if Pools are divided correctly from the start.
- Fix: share entries that list Resources, or an explicit move of a Resource between Pools of the same home.

**M9. Only Pools can be shared, not Agents (A14, coordinating agents).**
- A reviewer or triage Agent offered to several projects needs, for every consuming project, a Conduit to the Agent's home project or an approval for each Conduit. There is no `to: all` for Agents.
- Fix: let a share name an Agent, with a ceiling such as [turn.v1].

**M10. Revoking a delegate does not stop it (UC9, S6).**
- Collab closing `coordinator-in-packages` leaves the coordinator's open turn Conduits to packages workers in place. The coordinator keeps steering live workers, which write their trees and post to the queue. Only new work is refused.
- Fix: one operation that closes every Conduit X opened in P; the inventory already records who opened each one.

## Major gaps the design acknowledges

**M11. Nodes and runtime are not scoped (UC5).**
- "A conduit-core instance placed on win-wsl" cannot be refused, and collab cannot ask for an enrollment.
- S3 still works, because placement grants nothing.

**M12. "An auth Pool for one project" gives that project no autonomy (S2, S4).**
- Every setup or retry of `linear` opens its get-secret Conduit, which only root may do (ACC-005, SEC-008).
- So the project's managers cannot even restart their own Resources that use the secret, or rotate the secret (E13).
- The owner should be told plainly that this example can only be stated as "a Pool only root can operate".

## Minor

- **m1. Example 2 fails as written.**
  - Standalone Conduits are instances made from Conduit templates (CON-012; the only core Conduit templates are root's two), and PRJ-006 limits `from` to templates of the same project.
  - With `templates: [worker, tree]`, the coordinator cannot register worker-032--tree, worker-032--main or its turn Conduit.
  - Fix: list the Conduit templates in `templates`, which also caps their operations.
- **m2.** PRJ-009 rejects a reference in `arguments` to another project's object unless it is a Resource of a shared Pool. Example 2's bound names the Pools `conduit-core/repo` and `conduit-core/skills`, which that rule rejects.
- **m3.** The design never says which dimensions of a bound apply to which half. Example 2 relies on `targets: all` letting a packages target pass on conduit-core's consumer half.
- **m4.** PND-003 allows widenings only, but example 4's subject contains `register.v1`, which the design counts as neutral. A proposal must be able to carry registrations, or no new kind of Conduit can be proposed.
- **m5.** `to: all` skips the receiving project's acceptance, which the design justifies by the risk of injected content. Whether a project can leave an `all` share is undefined.
- **m6.** No authority can cover a class of projects, present and future, except engine-wide `management`. With one project per package, root must grant the coordinator in each one.
- **m7.** "The approver is a person" cannot be stated: an Interface may be an outside system. There is also no expiry on grants and no approval by two people.
- **m8.** Lowering a ceiling closes whole Conduits. A [read, write] Conduit loses read as well, and reopening read is a widening.
- **m9.** A runtime provider can be shared only through `system` (PRJ-007). Docker for two projects but not all of them must be engine-scoped and root-managed (S10).
- **m10.** Reusing a template across projects means packaging it, and only root installs packages (S7).
- **m11.** The "Operator" role cannot set up anything that has declared Conduits, because ACC-005 needs a right to open (S4 retries).
- **m12.** The engine's tombstones are keyed to the old project ID, so they do not bind a recreated project. Only the CLI's `--recreate` guards it (UC7).
- **m13.** Reading records per scope is undefined (open question 17). Managers cannot audit their own members, or the use of their shared Pools.
- **m14.** Whether a second project may expose itself to a peer root already trusts is unstated (UC6).
- **m15.** Sign-ins and enrollment approved by project approvers are not in v0 (acknowledged).
- **m16.** UC3 conflicts with itself. Collab may close the coordinator's Conduits into packages, but must not remove authority it did not grant. The design takes the permissive side and lists it as open question 10.

## Use case by use case

| Case | Can it be stated? | Is what must be refused refused? | Gaps |
| --- | --- | --- | --- |
| UC1, S1 | Yes, with the concise form unchanged | Yes | none |
| UC2 | Yes, after m1's fix | Yes: no reach into auth, consumer outside the scope, other templates, get-secret, writes to main or skills | M2, M12 |
| UC3 | Only with M5's workaround | Mostly | m16 |
| UC4 | Yes | Mostly: no self-approval, no puppet approver, drift, expiry | M1, M6, M7 |
| UC5 | No | No | M11 |
| UC6 | On paper | On paper | m14 |
| UC7 | Yes | Yes | M8, m12; example 3 conflicts with UC7 |
| UC8 | Fully | Fully | none |
| UC9, A78 | Yes, after m1's fix | Not 032 against 033 | M2, M10, M3 (the queue and the pull request) |
| UC10 | Badly | Badly | M8 |
| S2 | Within one project | Yes | across projects, M3 |
| S3 | Yes | Machines are not isolated | M11 |
| S4 | Yes | Yes | M12, m11; a partly failed approved action can only be retried with a new proposal |
| S5 | Only partly | Revisions keep it correct | M1, M7 |
| S6 | Yes: unsharing is immediate, in-flight calls finish | Yes | M10, m8 |
| S7 | Yes: packages can export templates | Yes | every Type needs `widens` marks; m10 |
| S8 | No, across projects | No | M3, M4 |
| S9 | First start is unchanged | Yes | Projects add engine state (shares, tombstones, pending actions) that repair must restore; the design does not say so |
| S10 | Yes | Yes | m9; swapping the secret store stays root's, as today |

## Verdict cost (major)
The design holds up as a mechanism, but its cost is understated. On a closer count it touches about 105 promises (23%) if the new family gets a name other than "Project", and about 145 to 150 (32%) if it keeps "Project", against its own claim of 75 to 80. The extra 42 promises come from the old meaning of "project", which they still use and would have to reword. Either way the change is bigger than making the short form normative (50 to 80), so it is not "the same scale". The reference model needs about 8,500 to 15,000 lines of Rust, against the 5,800 to 9,500 the design implies. Existing stores must be broken, which the design never says. Before any promise is written, eight inconsistencies need fixing.

Scope: I only read the repo at <home> and changed nothing. Every count below comes from grep or scripts over spec/, conformance/ and reference-model/. I ran nothing: the line estimates are on paper.

## What I confirmed as stated
- 455 promises.
- 367 conformance cases.
- 28,834 lines of Rust; core/src/access.rs is 729 lines.
- 12 reference schemas.
- `object-management.v1` has 11 operations, `access-control.v1` has 3, `pool-management.v1` has 7. So the new totals of 13 and 6 are right.
- OBJ-020 has ten kinds today.
- FST-001 creates five objects.
- None of ruling 53's additions, and no pending-action promise, is in spec/ yet.
- The 50 promise IDs the design lists as changed (it says "about 45") all exist and are affected.

## Cost table

| Item | Design says | What I found | My estimate |
|---|---|---|---|
| New promises | ~20 PRJ + ~12 PND (about 5 owed to rulings 49-51 anyway) | Plausible. Add 1 to 3: rules for Types of family Project, and an upgrade or break rule for stores | 33-36 |
| Changed promises, substantive | ~45 | 50 listed. **23 more are missed**: CON-004, CON-018, CON-024, CON-027, ACC-015, ACC-038, OPS-014, OPS-019, OPS-029, REV-010, POOL-015, POOL-017, LIF-001, LIF-009, LIF-021, OBJ-001, INS-001, INS-024, TPL-001, TPL-002, TPL-019, PLC-006, REC-001. They list the families, list the "management verbs", or depend on ACC-033's routing | ~73 |
| Rewording the old "project" | "churn follows", not counted | 55 promises use the word 135 times. 42 of them are not on the design's list, for example CTR-001 to 009, MAN-001/002/005/011/013/016, NAM-006 to 009/014/018/021/028, PKG-*, PRV-001/016/022, SCH-015/019, SEC-005/022, TPL-003/004, IFC-006/018. definitions.md uses it 28 times, including "a package is a project that has been published" and "submitted project". "submission" is already a separate defined term | +42 if the family is called Project; 0 if it gets another name |
| **Total touched** | 75-80 (16-18%) | — | **~105 (23%) under another name; ~147 (32%) keeping "Project"** |
| Core package | 1 Type, 1 template, 1 new + 2 changed capabilities, 5 new + 4 changed operations, 1 requirement, 1 new + 12 changed references | Also changes: `retire-with-resources.v1` has no input (core/operations.yaml:676) and needs one once it is routed through a Project or `management`. Also `record.v1` in results.yaml, the exports and `engine` lists in core/conduit.yaml, the two root templates, the kind/family enums at schemas/contracts.yaml:163, instances.yaml:148 and templates.yaml:131, and a `widens` keyword in the contract schema | 5 new and 5-6 changed operations, plus the items listed |
| Conformance, mechanical | ~300 manifests gain one line; ~9 first-start cases re-snapshotted | Only 199 manifests (190 conduit.json, 9 conduit.yaml). But `state.objects` arrays must match exactly, so a test Project added to root-system-state.json and the 17 initial*.json files adds one expected entry in about 230 snapshot cases. 21 files carry root's list of operations. 13 cases register into `system` and need separate `project: engine` submissions | ~450-500 files, scriptable |
| Conformance, re-expected | 6-10 routing cases | 78 cases cite ACC-033 (15 as a primary promise). The ones that flip: routing, s1-pool-authority, entry-authority, lifecycle-pool-target, bounds (consumer half), fixed-kind-references (uses the reserved name `local` and the same name across kinds), reconcile-true/false, records/expected-id, plus the first-start cases | 15-20 by hand |
| Conformance, new vectors | 60-90 | Increase for the 23 missed promises | 100-130 |
| Reference model, core | 4,000-6,000 | 10,465 lines of source. access.rs is rewritten. 73 name lookups, mostly through `object()` (access.rs:34) and registration.rs. Pending actions (subject, digest, before/after reach evidence, dispatch inside the approving invocation) are about the size of design B's apply executor | 5,000-8,000 |
| Loader and conformance runner | not counted | `Project` (meaning a submission) appears 349 times in 32 Rust files. The runner input kinds `register-project` and `resolve-project` appear in 270 steps | 700-1,500 |
| Shell, tests, concise CLI | 1,000-2,000, plus 800-1,500 for the CLI | Shell tests are 3,297 lines today | shell 1,200-2,500, tests 1,000-2,000, CLI 800-1,500 |
| **Rust total** | ~5,800-9,500 | — | **~8,500-15,000**, roughly the size of increment 2 or larger |
| Store migration | not addressed | shell/src/store.rs refuses any store whose format version differs, and has no migrator. Root's first-start Conduits are permanent (LIF-022, ACC-024), so no request can add the 5 new operations to them. Existing objects have no recorded project to give them a home | Either a stated clean break (re-initialize the lab stores and re-run S1), or 1-2 promises and a 200-400 line migrator |
| Reading cost | ~12 definitions; +3-4 concepts with several projects; 0 for S1 | definitions.md has 262 rows. A reader now meets a sixth family, two kinds of bound, a chain of scopes, five new operations, pending actions, a widening classification and new vocabulary for submissions | +13-15 terms, none removed. S1 concise +0, canonical +2 |
| Second person (DL13) | "not built" | Partly wrong. `issue_client_token` (shell/src/lib.rs:297) already accepts any instance, but main.rs:178 and :234 call it only for root. What is missing is the DL13 rule and handing the token to a person | 100-300 lines plus a DL13 ruling |

## Defects that change the cost (fix before writing promises)
1. **Which Conduit the record names.** The design says the record names the narrowest scope. But IFC-012 makes a tool call a request through that particular Conduit, and REC-002 records "the Conduit used". Drop the claim, or count changes to both promises.
2. **`retire-with-resources.v1` input.** ACC-033 relies on it having "no input that could name the Pool". Adding it to `object-management.v1` and `project-management.v1` changes that contract.
3. **Reconcile has no route.** ACC-033 rows 12-14, OPS-014 and REV-010 are missing from the P1-P15 table. Read literally, P15 leaves no way to reconcile turns and controls.
4. **Bounded `access` contradicts itself.** P8 accepts it as a consumer half; the bounds section says it supplies only the target half. This decides how the bounds case flips.
5. **Project Types.** "Every Project is of `project.v1` in v0" conflicts with extending CTR-004 to cover later Project Types.
6. **CON-024 is left out.** Its "management verb" list does not include `project-management.v1`, so requests to a Project in state registered would report unavailable.
7. **Shared Pools.** A receiving project has no route to inspect a shared-in Pool's Resources.
8. **The store break is never stated** (see store migration above).

## Verdict
The direction does not have to change, but the cost estimate does. Decide the family's name first: any name other than "Project" saves about 42 promises, 349 Rust identifiers and 270 runner steps. Present the change to the owner as larger than the short-form move, not equal to it. The design's staging still stands: family and scopes first, then shares, then propose and approve. Price the store break into the first stage.

Key files: spec/conventions/access.yaml (ACC-033 at line 652), spec/core/operations.yaml (line 676), spec/promises/conduits.yaml (CON-024, CON-027, CON-028), conformance/fixtures/root-system-state.json, reference-model/crates/core/src/access.rs, reference-model/crates/shell/src/store.rs, reference-model/crates/shell/src/lib.rs