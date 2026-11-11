Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# P3: Project as a scope (the namespace model). A Project is an engine-served target, like `management` and `access`, whose reach is the objects that name it. It is never a participant and never an owner.

## name
P3: Project as a scope (the namespace model). A Project is an engine-served target, like `management` and `access`, whose reach is the objects that name it. It is never a participant and never an owner.

## thesis
A Project is a **scope, not a participant**. It is an engine-served target whose capability is "management and access, limited to my members". The engine already serves two such targets for the whole engine, `management` and `access`. A Project serves the same operations for one part of the engine.

How it works:
- **Membership** is one fact stored on each member: its scope, which is either the engine or one Project. It is fixed when the object is registered, as a Resource's owner is (POOL-001) and as a Kubernetes object's namespace is. Membership grants nothing (ACC-028). The Project keeps no member list; inspection computes one (ruling 53's inventory).
- **Authority over a project** is an ordinary Conduit to the Project. This is Kubernetes' RoleBinding. Most such Conduits come from four core role templates, which play the part of the `admin`, `edit` and `view` ClusterRoles.
  - An Interface with authority over three projects holds three Conduits. Each project's own authority can revoke its one.
  - Authority over a scope also covers every scope inside it: the engine contains Projects, and a Project contains its Pools. So root's existing `management` covers every project, and nobody needs derived operator Conduits.
- **Pools have one home.**
  - A Pool homed in a project is private to it.
  - A Pool homed at the engine may declare an **audience**: which projects (or `all`, which explicitly includes projects made later) and the most operations each may be given.
  - This is GitHub's organization secret with `selected` repositories. One difference: an audience grants nothing. It only caps what each project's managers may open, as AWS RAM's managed permission does.
- **Approving a widening is the approver's own `open.v1`** of a Conduit someone else registered, pinned by `expectedId`. The pending action is the engine telling the Interfaces whose declared Conduits could make that open. Narrowing is a close or a retirement, and it never waits.
- **Nothing changes for existing documents.** An object that names no project is engine-scoped, which is today's meaning. First start is unchanged.

**My answer to the angle's question.** A scope is enough, but it has to be a *target*, and so an object (a Kubernetes Namespace is an object too). It does not need to be an owner or a sixth family:
- Ownership adds nothing that scope membership does not already give.
- A family adds a kind (OBJ-020) for something that behaves exactly like `management`.
- The price of this design is not a new family. It is that ACC-033's rule "one target authorizes each request" becomes "the narrowest target or any scope enclosing it". That amends ruling 30 (OL2) and ruling 44 (D03/D04: `management` cannot be narrowed), so it needs the owner. I say so plainly.

## model
## 1. What a Project is (not a family)

- **What it is.** A Project is a Resource of a new core type, `core#project.v1`. It is owned by `system`, like `management`, `access` and the nodes. It is made from a new core template `core#project` and served by the engine (spec/core/conduit.yaml adds `project-management.v1` to what the engine implements).
- **What it is not.** It has no run. It declares no Conduit, consumes nothing, never calls, owns nothing, and appears in no `runtime` or `placement`.
- **What it offers.** One capability, `project-management.v1`. Its operations are the existing management verbs:
  - `register.v1`, `register-replacement.v1`, `replace.v1`, `unregister.v1`
  - `inspect.v1`, `set-up.v1`, `retire.v1`, `reconcile.v1`
  - `open.v1`, `close.v1`
  - one new verb, `retire-with-members.v1`

  It offers no `install.v1`, `declare-lost.v1` or `purge.v1`, so the operations reserved to root stay root's (ACC-032).
- **Why a Resource and not a kind:**
  - A Conduit's target may already be a Resource (OBJ-013), so CON-001 and OBJ-020 do not change.
  - `management` and `access` are the precedent: engine-served Resources that exist only as authorizing targets.
  - A bound naming `system` already covers them, so the near-root hazard is not new.
  - A `kind: Project` would read better (ruling 52). It would also cost a sixth family: OBJ-020, definitions, schemas and the target kinds in OBJ-013. I list it as an open question, not as the recommendation.
- **Flat.** Projects do not nest. Moving an object to another project means replacing it.

## 2. Scope: membership is stored on the member

- Every template and instance has exactly one scope: the **engine** or **one Project**.
  - Through a Conduit to a Project P, `register.v1` registers into P implicitly. The target implies the scope, as `kubectl -n` does.
  - Through `management`, `register.v1` takes an optional `project` argument (with an optional `expectedId` of the Project). A document may also state `metadata.project`. A document that states a different project from its request is invalid.
- Scope is fixed for life. No operation changes it. A Resource's scope is always its owning Pool's scope.
- **Scope of each kind:**

| Kind | Scope | Note |
|---|---|---|
| Agent | engine or one project | An engine-scoped Agent is a cross-project actor, such as an engine-wide coordinator |
| Resource | its Pool's | POOL-001 unchanged |
| Pool | engine or one project | Project-homed means private. Engine-homed may declare an `audience`. `system` is engine-scoped and its audience is always empty |
| Conduit | its consumer's project if it has one, otherwise its target's project, otherwise the engine | Declared Conduits stay their consumer's (CON-002) |
| Template | engine or one project | A project's templates can be replaced only by that project's authority, which contains the `replace.v1` hijack |
| Interface | engine only (v0) | Interfaces are principals, like Kubernetes Users. They get authority over projects through Conduits |
| Project | engine only | |
| `system`, `local`, nodes, `management`, `access`, `root`, the machine provider and package-source providers | engine | PLC-008, PRV-013 |
| Runtime and secret-store providers | their Pool's | A member may name in `runtime` only a provider of its own scope or of the engine scope |
| Installed packages, records, trust domain | engine | |

## 3. "A project references every object under it", restated

The owner's direction is met as a computed view:
- Each member records its project.
- `inspect.v1` on a Project returns its members (ruling 53's inventory), limited to what the caller may inspect and with no count of the rest.
- The author's project file lists everything.

I deliberately keep no mutable member list on the Project. Every system studied records membership on the member. Kubernetes replaced its allowlist prune with ApplySet's part-of label because the list drifted and deleted things unexpectedly.

"Every possible object" is the part I challenge. These stay engine-scoped by design: Interfaces, Projects, nodes, the providers owned by `system`, the first-start objects, packages and engine-homed shared Pools.

## 4. Names

- **Recommended, separable from the rest:** names are unique per (scope, kind).
  - An engine-scoped name is reserved in every project, and a project's name for a kind is refused at the engine scope. So an unqualified `{kind, name}` resolves to the one object visible from the referrer's scope, with no precedence and no shadowing (GitHub's "lowest level takes precedence" is the trap).
  - A member of another project is always written `{kind, project, name}`.
  - A request sent through P resolves names in P. One sent through `management` resolves at the engine scope unless it gives `project`.
  - `--` stays reserved for generated names.
  - The concise form's default Pool can be called `default` in every project, which removes the critic's gap 16.
- **Fallback:** keep OBJ-002's engine-global names in v0. That saves about 6 promises and loses "the same project file applied twice under two names".

## 5. Lifecycle

- **Create.** Register a Resource `from: core#project`, `owner: system`. Members may come in the same submission. Root's `management` suffices, because it encloses `system` (see authority).
- **Membership.** An object joins only when it is registered, and never moves.
- **Retire:**
  - `retire.v1` on P, through `management`, reports conflict while any member is not retired, as LIF-020 does for Pools.
  - `retire-with-members.v1`, authorized by P or by `management`, retires the members in reverse dependency order: Agents, then Resources within their Pools in LIF-021's order, then Pools. Templates are left to `unregister.v1`.
  - Every Conduit with a retired end closes (CON-008). That includes Conduits crossing the boundary and every Conduit whose target is P, so all authority over P ends.
  - Then P itself is retired.
- **What a retirement never touches:** engine-homed Pools (even if they share into P), Interfaces (they keep existing with no access to P), engine-scoped Agents, and other projects.
- **Pending actions** whose subject is in P are voided.
- **Removal.** P is removed once it is retired and has no members (LIF-035), and its name is freed.
- **What survives:** records (REC-004) and P's tombstones, kept by project name (see spec plan).
- **A namesake project inherits no authority, audience entry or approval,** because all of these are references frozen to IDs (OBJ-021). It does inherit the tombstones, so reopening a revoked grant still needs an explicit reopen.

## 6. Is a scope enough? Compared with P1 (Project as a family that is target and owner)

I read P1 as: a new family that owns its Agents and Pools and is the target of project authority.

| Question | P3, scope | P1, family and owner |
|---|---|---|
| Is the Project a target of Conduits? | Yes | Yes |
| Owner of its members? | No. A scope fact, which permits nothing | Yes. A second kind of ownership beside POOL-001 |
| New family or kind | None: a Resource of a core type | A sixth family (OBJ-020, definitions, schemas) |
| Today's documents | Unchanged: no project means engine scope | Need an owner, or a default project |
| Shared Pool | Homed at the engine with an audience. Nobody owns it more than once | Owned by one project and shared to others. Retiring the owner affects them |
| Root | Covered by containment. Nothing derived | Needs a Conduit to each project (the derived operator Conduit), unless containment is added too |
| Main core change | ACC-033 becomes a scope chain | ACC-033 gains Project rows (rows 2, 5 and 7 split) |

- **What P1 has that P3 lacks:** a more descriptive `kind:`, and cascade by ownership. P3 offers the cascade explicitly, as `retire-with-members.v1`.
- **What P3 has that P1 lacks:** additivity, no second ownership relation, and the GitHub org-secret shape for shared Pools with no co-ownership.
- **Conclusion:** the only thing a Project must be is an addressable target. Everything else a family would add, a scope already gives.

## sharing
## The model: GitHub organization secrets, with RAM's "grants nothing, caps only"

1. **One home, never several.**
   - A Pool homed in a project (`project: conduit-core`) is private. Its Resources are reached from that project, or from another project only by a Conduit that both sides consent to (see authority).
   - A Pool homed at the engine (no `project`) is the organization-level item.
   - No Pool belongs to two projects. Every system studied rejects multi-homing (Kubernetes, GCP, Azure, HCP), and preservation test 3 says a shared thing is owned once.

2. **Who decides, and how it is declared.**
   - An engine-homed Pool states its audience in its configuration. This uses a new core Pool type, `core#pool.v2`, because `pool.v1`'s schema is `additionalProperties: false`:
     ```yaml
     config:
       audience:
       - projects: all             # every project, those made later included
         operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]
       - projects: [{name: packages}]
         operations: [example.org/s1/history#history.post.v1]
     ```
   - `all` always includes projects made later. Inspection and every plan print it as "all projects, including those made later", following lesson 4 (a grant on a container covers future members, so say so).
   - The audience is changed in place by a new core control `share.v1` on `pool.v2`, sent with the Pool's revision (REV-002). A control is the language's existing in-place change.
   - The control is authorized by a Conduit to the Pool listing `share.v1`, or by an enclosing scope's authority (`management` for an engine Pool, which means root).
   - So **the owner of the shared thing decides** (lesson 2), as GitHub's org owner sets `selected_repository_ids`.

3. **What an audience never contains:**
   - `get-secret.v1` (SEC-008 is unchanged);
   - operations reserved to root;
   - provider operations (PRV-004);
   - management verbs.

   `system`'s audience is always empty, and `share.v1` on `system` reports invalid. So `management`, `access`, nodes and providers can never be shared into a project. That rules out the FST-001 hazard of naming `system`.

4. **What the audience does.**
   - It is the **target's standing half** of every Conduit from a project member to one of the Pool's Resources.
   - It permits nothing by itself.
   - That project's own managers then open individual Conduits from their members, up to the ceiling. These are AWS's "two evaluations": the share is the maximum, and the consumer side grants within it.
   - The cap applies to every opener, **root included**. Then `inspect skills` states completely which projects can ever reach it. Root wanting a one-off exception either raises the ceiling or puts the Resource in another Pool. This loss is listed in what_is_lost.
   - Engine-scoped consumers (Interfaces, engine Agents) are not capped by audiences. They are wired by engine authority, as today.

5. **What a project may do with a Pool shared into it:**
   - open and close Conduits from its own members to the Pool's Resources, within the ceiling;
   - inspect the Pool and its Resources (name, Type, capabilities, its own Conduits into them).

   **What it may not do:**
   - register into the Pool, set up or retire its Resources, or change its audience;
   - see other projects' Conduits into it, or their count (ruling 52; Gateway's "MUST NOT expose");
   - re-share it. Shares are non-transitive: only the home's authority changes the audience (RAM: "can't share a resource that's shared with you");
   - grant an Interface access to it. An Interface is engine-scoped, so that takes engine authority.

   A bound on a Conduit to a Project that names the project covers the project's members, never the Pools shared into it. Cover follows home, never reference (the design D skeptic's widening).

6. **Narrowing never waits.**
   - A `share.v1` that removes a project or lowers its ceiling closes at once every open Conduit from that project's members into the Pool that exceeds the new ceiling, whoever opened it. Each close leaves a tombstone. RAM's removal and the deletion of a Gateway ReferenceGrant behave the same way.
   - A widening `share.v1` opens nothing. It only lets each project's managers act.
   - Within one engine (one trust domain) the consumer project does not have to accept a share, as RAM inside an organization needs no invitation: nothing reaches its Agents until its own managers open a Conduit. Acceptance with an expiry belongs to peers (ruling 51).

7. **When either side retires.**
   - **The shared Pool is retired** (engine authority, a narrowing). CON-008 closes every project's Conduits into it, and LIF-019 leaves their references pointing at a missing ID. The plan lists the closures by project. I recommend this does not block retirement, to keep "narrowing never waits". The PRV-020-style "in use" refusal is the alternative, as an open question.
   - **A consumer project is retired.** Its members' Conduits into the Pool close. Its audience entry keeps P's ID, which is now missing (LIF-019), and admits no namesake.
   - **A project-homed Pool's project is retired.** The Pool retires with it. Nothing else can depend on it, because it was never shared.

8. **Project to project.**
   - In v0, a project-homed Pool is never shared, so there is no Shared VPC-style host project. If two projects need a Pool, it lives at the engine with an audience naming both.
   - Moving a project Pool to the engine later means replacing its Resources, and root rewires any secrets.
   - A single Conduit between members of two projects is allowed, with both halves (see authority).

## The owner's two examples

- **Skills Pool for all projects.** Engine-homed, `audience: [{projects: all, operations: [file.read.v1, directory.list.v1]}]`. Root alone writes to it. Each project's managers give their Agents read access.
- **Auth Pool for one project.** Two correct shapes:
  - (a) homed in `conduit-core`, private;
  - (b) homed at the engine, with `audience: [{projects: [conduit-core], operations: [linear list/get]}]`.

  **Challenge.** In both, the secret Resource stays root-wired. Only root opens a Conduit listing `get-secret.v1` (SEC-008, SEC-009), and sign-ins go to root in v0 (ruling 49). The project can be given the *service* backed by the credential (the Linear MCP Resource), up to its ceiling, but never the secret.

  Letting a project's managers wire secrets would change the secret guarantee itself (README, "What the guarantee for secrets covers"). I recommend against it. Following lesson 5, I recommend (a) only when every manager of `conduit-core` is trusted with everyone's Linear access; otherwise (b) with a narrow ceiling.

## authority
## Principles kept

- Every permission is an open Conduit (rulings 28 and 29, ACC-001 and ACC-003).
- Ownership and membership permit nothing (ACC-028).
- Using a member always needs a Conduit to that member (R16). Authority over P never lets its holder take a turn on P's Agent or read P's Resource.
- An onward request is decided by the sender's own Conduits (ACC-016).

## 1. Authority over a project is a Conduit to it (the RoleBinding)

- An Interface or Agent holding `X -> P` with capability `project-management.v1` acts on P's members with the operations it lists.
- **An Interface over N projects holds N Conduits.** This answers "Interfaces could have authority over one or multiple projects". Each is revocable by that project's authority, never held as one list on the Interface (Lighthouse, RoleBinding).
- **"Projects determine an Interface's permissions".** The project's file declares its managers (the concise `managers:` key compiles to these Conduits), and P's inventory lists them. The authority itself is still the Conduit.
- **Core role templates** (Conduit templates in the core package, which cannot be replaced, and which play the part of ClusterRoles):

| Template | Operations of `project-management.v1` |
|---|---|
| `core#project-admin` | register, register-replacement, replace, unregister, inspect, set-up, retire, reconcile, open, close |
| `core#project-approver` | inspect, open, close, reconcile |
| `core#project-proposer` | inspect, register, unregister, retire, close (it can propose and narrow, but not enact) |
| `core#project-viewer` | inspect |

  A project may also register its own Conduit templates for this capability, its custom roles (a Kubernetes Role).
- `retire-with-members.v1` is in no role template. Root holds it through `management`, or someone grants it explicitly.

## 2. ACC-033 becomes a scope chain (the core change)

**The rule.** A management verb on an object is permitted by an open Conduit listing the operation to the object's **narrowest authorizing target, or to any scope enclosing it**:
- the Pool, for its Resources (as today);
- the object's Project, for a project member;
- `management`, which encloses everything.

**Pools' administrative controls.** `share.v1` follows the same chain. Controls of Agents and Resources stay with the instance itself (row 11).

| Row | Today | P3 |
|---|---|---|
| 1, register a Resource into a live Pool | `management` AND the Pool | A *trust half*: the object's scope target (its Project, or `management`). Plus a *Pool half*: the Pool, or any scope enclosing it. One Conduit to P or to `management` satisfies both. A Pool Conduit alone still does not (unchanged) |
| 2, register any other object; replace; unregister | `management` | The new object's scope target (P, through which it is sent) or `management`. Registering a Project, an Interface or an engine-scoped object: `management` only |
| 3, register-replacement | row 1 or row 2 | The same, as P3 rows 1 and 2 |
| 4, set-up or retire a Resource | the Pool | The Pool, its Project, or `management` |
| 5, set-up or retire an Agent, Interface or Pool | `management` | For a member: P or `management`. For an engine-scoped object: `management` |
| 6 and 7, inspect | Pool / `management` | The narrowest target or any enclosing one. Also: P itself authorizes inspecting P |
| 8, retire-with-resources | the Pool | The Pool or any enclosing scope |
| 9, install and reserved operations | `management` | Unchanged |
| 10, open and close | `access` | `access` (engine, bound as today) **or** a Project (section 3) |
| 11, a control | the instance | Unchanged, except a Pool's controls follow the chain |
| 12 to 14, reconcile | the authorizing target | Any target that would authorize the original request now |

**Consequences:**
- Root's existing `management` now covers every Pool row. Root's own Conduits to its Pools and the derived operator Conduit disappear: 40 of the coordinator's S1 lines, and ruling 53's decision 6 (the operator Conduit) is answered by needing nothing.
- This also widens what *any* `management` holder may do, so it must be stated.
- It amends ruling 30 (OL2, "one authority for an object's whole life"). OL2's own precedent column already cited "Azure and Kubernetes scope management rights by group or namespace".
- It amends ruling 44 D03/D04 ("access to management cannot be narrowed; use a broker"). A Conduit to P *is* `management` narrowed to a scope, so it is the "focused constraint" that spec/README says would come back to the owner.

## 3. Opening and closing through a Project: two halves

`X -> P` listing `open.v1` permits opening Conduit C only if **both halves** are covered by X's Conduits to Projects (X may combine several) or by engine authority.

**Target half**, one of:
- C's target is a member of a project X holds `open.v1` in;
- C's target is a Resource of an engine Pool whose audience admits C's consumer's project, with every operation C lists inside the ceiling (the standing half);
- C's target is P itself, which is delegation (section 4).

**Consumer half**, one of:
- C's consumer is a member of a project X holds `open.v1` in;
- C's consumer is an Interface. A principal may be granted by the target side alone, as a RoleBinding subject is. This is also how a peer engine's admitted Interface is exposed at project level: ruling 51's two halves, with this engine's half held by the project.

**Bound.** If X's Conduit applies a bound, every operation C lists must be named there, and its targets must be covered (ACC-029's rule, applied to the project).

**Never through a Project:**
- a Conduit to `management`, `access`, a node or a provider;
- one listing `get-secret.v1` (SEC-008, which stays root only);
- one listing a reserved operation (ACC-032);
- one listing a provider operation (PRV-004).

**Cross-project consent in v0.** One opener must cover both halves: root, or someone who manages both projects. The coordinator holding admin in both `conduit-core` and `packages` can connect its own projects. Collab, admin of `packages` only, cannot connect a `packages` Agent to a `conduit-core` Resource. Recorded consent per half (two approvers) is an open question for v1.

**Closing (narrowing):** `X -> P` listing `close.v1` closes C when either half is in P: a member consumer, a member target, or an audience-admitted Pool's Resource whose consumer is in P. Either side may cut a cross-project Conduit at once.
- Root's Conduits are never closed (ACC-024, unchanged).
- Co-managers can close each other's grants (ACC-012, kept). Authorship stays a record, not authority.

## 4. Bounds, delegation and the hazards

- **Bounds.** CON-028 changes: a bound may be carried by a Conduit to `access` **or to a Project**. `bound.v1` becomes `bound.v2`, with two new inputs:
  - `templates`: the templates that `register.v1` through P may instantiate from, nested instantiations included. This narrows the create right; lesson 5 says a scope is only as strong as its weakest create right.
  - `delegates` (default false).
  - A Conduit to P with no bound covers all of P, including members made later. This fixes B3 (bounds frozen at instantiation).
- **Delegation.** Opening or closing a Conduit **whose target is P** needs the actor's Conduit to P to carry `delegates: true`. Then the new Conduit's operations must be a subset of the actor's and its bound within the actor's (ACC-030's rule). Who may make managers and approvers is therefore explicit, like Kubernetes' `bind` and `escalate`.
- **Revocation does not cascade** (ACC-030, kept).
- **Hazards ruled out:**
  - `system` is never a member and never in an audience.
  - A bound naming P covers members, never shared-in Pools.
  - Project authority never reaches `management` or `access`.
  - A project member can hold engine authority only if it is made engine-scoped instead. That is a visible choice; the coordinator in the examples is a member.

## 5. Root and a second person

- **Root's documents are unchanged.** Containment makes root's `management` enclose every project. Its unbounded `access` opens anything, except into engine Pools beyond their audiences for project consumers.
- **Identity powers that stay root's:**
  - the reserved operations;
  - `get-secret.v1` opens;
  - ACC-024 protection and permanence;
  - sign-ins and enrollment in v0 (rulings 49 and 50).
- **A second person.** Root registers and sets up Interface `collab` through `management` (engine scope), then opens `collab -> packages` from `core#project-admin`. This presupposes DL13: how a person gets a token. The first build issues tokens only to root and to run instances, so this is a prerequisite, not part of this design.
- **Visibility.**
  - Through P, a holder inspects P's members, the Pools shared into P (only what is described in sharing), and the names of Interfaces and Projects.
  - Refusals across projects are ruling 52's first tier only, and so is registering a Conduit whose target the registrant cannot inspect. Both give the answer a non-existent object gets.
  - This fixes B7, where today visibility is all or nothing.

## approvals
## No new approval object: the register/open split is the pending state

- **A proposal** is a registered, unopened Conduit whose registrant cannot open it.
  - Typically the registrant is a holder of `core#project-proposer`.
  - It may also be a registered Agent whose setup would open declared Conduits the registrant cannot open (ACC-005).
  - A registered Conduit permits nothing (`open.v1` opens only registered Conduits), so a proposal is safe while it waits.
- **Approval** is the approver's own `open.v1` of that Conduit, sent with `expectedId`, the ID it was registered under (ruling 53).
  - It is decided anew at the moment it is sent (ACC-036).
  - It arranges no later work (ruling 38, ACC-019).
  - Its record names both the proposer and the approver (preservation test 7).
- **Declining** is the approver's `unregister.v1` with `expectedId`, which leaves a tombstone.
- **The proposal binds exactly what is approved.**
  - A changed proposal is a new Conduit with a new ID. The old `expectedId` then fails with conflict, so the plan approved is the plan run.
  - `register.v1` gains an input `proposalExpires`, bounded by an engine maximum. An `open.v1` after that time reports conflict and the proposal must be made again. This check happens when a request is decided, not as background work.

## Who may approve is declared

- A project's approvers are the consumers of open Conduits to P that list `open.v1` (`core#project-approver` or `core#project-admin`). Engine authority (root's `access`) covers every project.
- P's inventory prints them as "approvers of P".
- Who may make approvers is declared too: only a holder whose Conduit to P carries `delegates: true`.
- Adding an approver is itself a widening, an open of a Conduit to P, so it is proposed and approved the same way.
- Removing one is a close: immediate, never root (ACC-024).
- **Standing approval.** A Conduit to P with a bound is standing approval within that bound, answering use case F4. The coordinator's `templates: [pkg-worker, pkg-tree]` lets it run work orders without waking anyone.

## The pending action (ruling 51's generic shape, plus what the prior art adds)

| Field | Content |
|---|---|
| kind | `approval` |
| project | the project whose half is missing; both projects where both halves are missing |
| subject | the Conduit, by ID and revision, and `open.v1` |
| evidence | Written by the engine. The tools the consumer gains, as `as.operation`. The target and its scope. The operations and filters. The audience and ceiling the open relies on. The predecessor closed with the same (consumer, `as`), taken from the tombstones, so an approver sees "close then reopen with more" for what it is. The proposer's justification is shown only as a quote |
| proposer | the registrant, with its caller chain (for example root → pkg-lead's turn) |
| approvers | computed: the Interfaces whose declared Conduits cover every missing half, plus root always (rulings 49 and 50, v0) |
| expiry | `proposalExpires` |
| outcome | opened, declined or expired, with the actor |

- **Where it is raised.** At those Interfaces' control panel (ruling 50).
- **Agents with open authority.** An Agent holding open authority is not prompted, since prompts go only to people (ruling 49's spirit). Under preservation test 5 it may still open. The owner can forbid that by never granting `open.v1` to Agents (open question).
- **Disclosure.** An approver sees only what it may inspect. A proposal that names a target its proposer may not inspect is refused when it is registered, so a pending action never leaks another project's objects.

## Narrowing never waits

All of these happen at once, by any holder covering either half: `close.v1`, `retire.v1`, `retire-with-members.v1`, `unregister.v1`, a narrowing `share.v1`, and closing a manager's Conduit to P.
- None of them is ever bundled with a widening.
- A pending widening never holds one back. In S1's T4, B's narrowing is a separate request from A's widening.

## What counts as a widening

Every `open.v1`, every setup that opens declared Conduits, every `share.v1` that adds a project or operations, and every Conduit to a Project. The engine needs no general "is this wider?" comparison:
- In Conduit, every gain of access is already an open.
- Every open needs authority, whatever its predecessor was.

So the tricks that worried the use cases are covered: an equal re-point or a split "close, then reopen with more" by a proposer needs approval, and by root needs none.

**Not covered:** settings that widen reach without an open, such as an Agent's `accountServices: true` (ruling 48). These are bounded by `bound.templates` and by keeping `set-up.v1` out of `project-proposer`. A Type marking which settings widen remains open (F9).

## Two challenges to the owner's direction

1. **"Who may approve" separate from "who may do".** A separate approver list that is not itself the authority to open would be a second source of authority, which ruling 28 rejects. A list that *is* that authority is just `core#project-approver` without `register.v1`. So proposer, approver and admin are three role templates, not a new mechanism.
2. **Two approvers for one cross-project widening** (packages' and auth's). In v0 this needs one actor covering both halves, which in the owner's setup is root. Recording each half's consent separately is ruling 51's "two halves" inside one engine. It is worth doing, but it adds a state between registered and open. I put it to the owner for v1 rather than hide the cost.

Sign-ins and enrollment keep rulings 49 and 50 (root in v0). The Project gives them the routing key for v1: holders of `set-up.v1` on the member's project.

## examples
Operation names are shortened (`file.read.v1` for `example.org/s1/filesystem#file.read.v1`, `core#` for `github.com/shrav-k/conduit/core#`). The real documents write them in full (ruling 25).

## 1. S1 as one project

**Canonical form.** Root registers the Project through `management`:
```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata: {name: s1}
from: core#project
owner: system
```
Then the coordinator's documents are submitted unchanged with `register.v1 {project: s1}` (CLI `conduit submit -p s1`), with two deletions: the template `pool-admin` and the instance `root-workspace`. Root's `management` now encloses s1 and its Pool `workspace`.

Every object becomes a member of s1: Pool `workspace`, Resource `shared`, `agent-a`, `agent-b`, their declared Conduits `project`, and `turn-a` and `turn-b` (whose consumer root is engine-scoped and whose target is a member, so their home is s1). The 8 templates are s1's.

`inspect s1` as root:
```
Project s1 (prj-1a2) ready. Authority: root via management (encloses every project). Approvers: root.
Members: Pool workspace; Resource shared; Agent agent-a, agent-b; Conduit agent-a/project, agent-b/project, turn-a, turn-b; 8 templates.
Shared into s1: none.  Crossing the boundary: turn-a, turn-b (consumer root, engine).  Tombstones: none.
```

**Concise form.** The simplification's file is unchanged: `project: s1` was already one of its 15 lines. The plan differs from the synthesis's only here:
```
+ Project s1                       line 2      root's management encloses it: no operator Conduit derived
+ Pool default (project s1)        rule: default Pool per project
+ Resource shared (owner default)  ...  + Agent agent-a, agent-b  ...  + turns root->agent (rule)  ...
```

## 2. The owner's real use: two projects, a shared skills Pool, a project-only auth Pool

**engine.yaml** (no `project`, so engine scope; root applies it):
```yaml
conduit: v0
pools:
  skills:
    audience: [{projects: all, operations: [file.read.v1, directory.list.v1]}]   # all = including later projects
    resources:
      claude-skills: {type: example.org/s1/filesystem#local-directory.v1, config: {root: ~/.claude/skills}}
      codex-skills:  {type: example.org/s1/filesystem#local-directory.v1, config: {root: ~/.codex/skills}}
  repo:
    audience: [{projects: [conduit-core, packages], operations: [file.read.v1, directory.list.v1, history.post.v1, history.read.v1]}]
    resources:
      main:  {type: ...#local-directory.v1, config: {root: <repo>/main}}       # read-only by ceiling
      queue: {type: example.org/s1/history#file-history.v1, config: {root: <repo>/orders}}
interfaces:
  collab: {type: core#interface.v1}
```

**conduit-core.yaml:**
```yaml
conduit: v0
project: conduit-core
pools:
  auth:                     # homed here: private, never shared
    onepassword: {type: example.org/onepassword#service-account.v1, config: {vault: conduit-test}}
    linear-key:  {type: example.org/onepassword#item.v1, config: {item: linear-api}}
    linear:      {type: example.org/linear#linear-mcp.v1, secrets: {token: linear-key}}  # get-secret Conduit: root only (SEC-008)
  core-work: {}
templates:
  worker: {type: example.org/conduit/agent-codex#codex.v1}
  tree:   {type: ...#local-directory.v1}
agents:
  coordinator:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach:
      queue:  {to: repo/queue, ops: [history.post.v1, history.read.v1]}            # engine Pool, within audience
      skills: {to: skills/claude-skills, ops: [file.read.v1, directory.list.v1]}
      linear: {to: linear, ops: [linear.list-issues.v1, linear.get-issue.v1, linear.create-issue.v1, linear.update-issue.v1]}
      core:   {to: project conduit-core, from: core#project-admin, bound: {templates: [worker, tree]}}
  triage:
    type: example.org/conduit/agent-codex#codex.v1
    reach:
      linear: {to: linear, ops: [linear.list-issues.v1, linear.get-issue.v1]}
```

**packages.yaml:**
```yaml
conduit: v0
project: packages
managers:
  collab:      {from: core#project-admin}
  coordinator: {from: core#project-admin, bound: {templates: [pkg-worker, pkg-tree]}}  # consumer in conduit-core: both halves, root applies
pools:
  pkg-work: {}
templates:
  pkg-worker: {type: example.org/conduit/agent-codex#codex.v1}
  pkg-tree:   {type: ...#local-directory.v1}
agents:
  pkg-lead:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach:
      packages: {to: project packages, from: core#project-proposer}
      queue:    {to: repo/queue, ops: [history.post.v1, history.read.v1]}
```

**What follows:**
- For each work order, the coordinator, with no approval needed:
  - registers `pkg-worker`, and a `pkg-tree` in `pkg-work`, through `packages.register.v1`;
  - opens the worker's Conduits: its tree [read and write], `repo/main` [read], `repo/queue`, `skills/codex-skills` [read];
  - opens its own turn to the worker. The consumer is in conduit-core and the target in packages, and the coordinator holds both halves.
- At merge, it closes those Conduits and retires the worker and tree. That is a narrowing and never waits.
- **Refused:**
  - a packages member reaching `linear` or `linear-key` (no audience, and no conduit-core half);
  - the coordinator registering from any template but its bound's;
  - a worker writing `main` (above the ceiling, even for root while the audience stands);
  - collab touching conduit-core, skills or repo beyond packages' ceiling. Collab gets ruling 52's first tier only, with no disclosure.

## 3. A second Interface limited to one project

```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Interface
metadata: {name: collab}
from: collab-person            # an engine template of type core#interface.v1, declaring no Conduits
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata: {name: collab--packages}
from: core#project-admin
consumer: {kind: Interface, name: collab}
target: {kind: Resource, name: packages}
as: packages
```
Root sends `register.v1` and `set-up.v1` through `management`, then `open.v1` through `access`. Collab's tools are `packages.register.v1`, `packages.inspect.v1`, `packages.open.v1`, and so on.

**Collab may:**
- register, set up and retire packages members, including new Pools and Agents;
- open Conduits between packages members, from packages members to `skills` and `repo` within their ceilings, and from any Interface (collab herself, ci-hook) to packages members;
- close any of them, including the coordinator's Conduit to packages;
- approve packages' proposals.

**Collab may not:**
- make another admin (no `delegates: true`);
- install; declare-lost or purge;
- open `get-secret.v1`;
- touch engine Pools' membership or audience;
- inspect conduit-core;
- close root's Conduits;
- retire the Project packages itself.

**Prerequisite:** DL13 must give collab a token.

## 4. An agent proposing a change

pkg-lead holds `pkg-lead--packages` from `core#project-proposer`. Through `packages.register.v1` it sends:
```yaml
proposalExpires: 2026-11-17T00:00:00Z
documents:
- {apiVersion: github.com/shrav-k/conduit/v0, kind: Conduit, metadata: {name: worker-033-queue}, from: queue-poster,
   consumer: {kind: Agent, name: pkg-worker-033}, target: {kind: Resource, name: queue}, as: queue}
```
It registers, but it does not open, because pkg-lead holds no `open.v1`. The engine raises:
```
pending action pa-91  kind approval  project packages
subject   Conduit worker-033-queue (cdt-4c1), open.v1
evidence  pkg-worker-033 gains queue.history.post.v1, queue.history.read.v1; target queue (engine Pool repo);
          relies on repo's audience for packages [.., history.post.v1, history.read.v1]: within ceiling
proposer  pkg-lead (Agent, packages) via root -> pkg-lead turn inv-77
approvers collab (collab--packages: open.v1), root     expires 2026-11-17T00:00Z
```
Collab approves with `packages.open.v1 {conduit: {kind: Conduit, name: worker-033-queue}, expectedId: cdt-4c1}`. The record names pkg-lead as proposer and collab as approver. pkg-lead sees the outcome the next time it inspects; it is never woken.

**Variant:** pkg-lead proposes `file.write.v1` on `repo/main`. Collab's open is refused, because the operation is above packages' ceiling for `repo`. The pending action says the only route is engine authority raising `repo`'s audience for packages with `share.v1`, which root alone can do and which widens every packages manager's power. That is the explicit choice the owner asked for.

## 5. S1 line counts

| | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
|---|---|---|---|---|---|---|---|---|
| Baseline A70 | 33 | 16 | 0 | 8 | 3 | 2 | 2 | 64 |
| P3, concise form (the simplification's package plus Project) | 16 | 6 | 0 | 2 | 1 | 1 | 1 | **27** |
| P3, canonical form (the coordinator's documents) | ~139 | ~56 | 0 | 2 | 62 | 29 | 12 | ~300 |
| Coordinator today | 154 | 73 | 0 | 2 | 62 | 29 | 12 | 332 |

- **How the canonical form was counted:**
  - T1 = 154 − 22 (`pool-admin` 12, `root-workspace` 8, the t1fix manifest 2) + 6 (the Project document) + 1 (the `-p s1` value).
  - T2 = 73 − 18 (`admin2.yaml` 16, its manifest 2) + 1.
  - T3 to T7 do not change.
- **Approvals.** In S1, root makes every widening (T1, T4, T7's restore), so nothing is ever pending.
- **Honest reading.** P3 adds 0 author lines to S1 and removes none from the concise form; the package already derived root's Conduits at 0 lines. What it removes are 3 derived objects, the operator Conduit's register and open, and the design D defect that the project's memory died with its Pool. P3's value shows in A14/A78 (examples 2 to 4), not in S1.
- All counts are paper counts (ruling 45).

## spec_plan
## Rulings this amends (owner decisions needed)

- **Ruling 30, OL2.** "One authority for an object's whole life" becomes "the narrowest target or any enclosing scope".
- **Ruling 44, D03/D04.** `management` becomes narrowable by scope through Projects. The broker remains for narrowings finer than a project.
- **Ruling 42's one-meaning rule.** Today's "project" (the documents and manifest handed over in one registration) is renamed **submission** (already used in SUB-*). "Project" then means the registered scope.

Rulings 28 and 29 are kept. So are 49 to 53, which this design builds on.

## Promises added: a new file spec/promises/projects.yaml, about 17

| ID | Content |
|---|---|
| PRJ-001 | A Project is a Resource of `core#project.v1`, owned by `system` (refused otherwise, as PRV-013 refuses a misplaced provider), served by the engine. It has no run and declares and consumes no Conduit. |
| PRJ-002 | Every template and instance has exactly one scope, the engine or one Project, fixed for life. A Resource takes its Pool's scope. Interfaces, Projects and `system`'s Resources are engine-scoped. The rule that gives a Conduit's home. |
| PRJ-003 | `register.v1` through P registers into P. Through `management` it takes `project`, with an optional `expectedId` of the Project. A document's `metadata.project` must agree with its request. |
| PRJ-004 | Membership grants nothing, and no member list is stored on a Project. |
| PRJ-005 | A member's references (`config`, `arguments`, `runtime`, `from`) resolve in its own scope or the engine scope. Another project's member may be named only as a Conduit's consumer or target, qualified. |
| PRJ-006 | Scope enclosure for management verbs (the chain under ACC-033). |
| PRJ-007 | Opening through a Project: the two halves, the Interface-subject rule, audiences as the standing target half, and what is never opened through a Project. |
| PRJ-008 | Closing through a Project: either half suffices. |
| PRJ-009 | Delegation of a Project's authority: `delegates: true`, operations a subset, bound within. Symmetric for close. |
| PRJ-010 | An engine Pool's `audience`: its form; `all` includes later projects; what a ceiling never holds; project Pools and `system` have none. |
| PRJ-011 | `share.v1`: a narrowing closes, at once, every Conduit beyond the new ceiling and leaves tombstones. A widening opens nothing. |
| PRJ-012 | The audience caps every opener, root included, for project-member consumers. |
| PRJ-013 | Retiring and removing a Project (`retire.v1` conflict; `retire-with-members.v1` order; engine Pools untouched; pending actions voided). |
| PRJ-014 | The Project's inventory: members, managers and approvers, audiences admitting it, Conduits crossing its boundary. Only what the caller may inspect, no counts. |
| PRJ-015 | Proposal and pending action: kind `approval`; subject, evidence, approvers, expiry, outcome; `open.v1` after expiry is a conflict. |
| PRJ-016 | Tombstones per project (see below). |
| PRJ-017 | A lifecycle request may assert `project`. It reports conflict when the object is not a member, which makes prune safe in the engine. |

## Promises changed: about 25

- **ACC-033.** Restated as the scope chain (rows 1 to 14 as in the authority table).
- **ACC-006 and ACC-012.** Opening and closing through `access` or a Project.
- **ACC-028.** Membership permits nothing; a Conduit to P permits no member's own operation.
- **ACC-029 and ACC-030.** `bound.v2`'s `templates` and `delegates`; bounds on Conduits to Projects.
- **CON-028.** A bound is allowed on Conduits to `access` or to a Project.
- **ACC-022.** Root's `access` stays unlimited except PRJ-012.
- **SEC-008.** Observable: no Project authority opens `get-secret.v1`, and no audience holds it.
- **PRV-006.** `runtime` stays within the instance's scope or the engine's.
- **OBJ-013.** The standard argument `project` for Agent, Pool, Conduit and templates (Resources: through their Pool).
- **If names are scoped:** OBJ-002, OBJ-006 (an optional `project` key in references), OBJ-021, NAM-003, INS-025 (`from` resolves in scope, then engine), LIF-031 (requests name `project` for a member outside the actor's scope), and the reference schemas in spec/core/references.yaml.
- **LIF-013.** The inventory.
- **REC-002.** Records name the scope of the caller and the target.
- **Vocabulary: project becomes submission.** SUB-005, SUB-008, SUB-009, NAM-009, NAM-022, NAM-027, MAN-015, and definitions.md (project, submission, scope, member, audience, and the entries for `management` and `access`).

**Removed:** none. **Kept unchanged:** POOL-001 (owner stays a Pool), CON-001, ACC-024, ACC-032.

## First start

- FST-001 to FST-006 are unchanged in text.
- No project exists at first start (FST-003 holds), and there is no `default` project. A missing `project` means the engine scope, today's meaning.
- Root's two Conduits are unchanged as documents. Their reach changes only through ACC-033: `management` now encloses Pools and Projects.
- The new core templates (`project`, the four `project-*` role templates, a `shared-pool` template of `pool.v2`) are ordinary package exports, not first-start objects.

## Ruling 53's five additions, per project

- **`as`.** Unchanged. It is a local name in the consumer's tool namespace, so scoped object names never reach tools. A Conduit to a Project defaults to `as: <project name>` (tools `packages.register.v1`).
- **`expectedId` on open and close.** It is the binding of an approval (PRJ-015). It is also accepted on `register.v1`'s `project`, so a plan cannot register into a swapped namesake project (design D's "identity is an ID that is checked").
- **The inventory.**
  - `inspect.v1` on P returns PRJ-014.
  - On an engine Pool, it returns its audience and, to engine authority, the Conduits into it grouped by consumer project. A project's holder sees only its own project's.
  - Drift is computed per project from this inventory. Prune uses PRJ-017, so neither can touch a non-member: `system`, `local`, `management`, `access`, `root` or another project.
- **Tombstones.**
  - Keyed by (scope name, kind, name) for instances and (scope name, consumer, `as`) for Conduits.
  - Kept as engine state apart from records (no purge removes them) and enforced at `open.v1` and `register.v1` within that scope.
  - They outlive any Pool and the Project itself, and bind a namesake project. That answers the critic's gap 2 and design D's "the record dies with the Pool".
  - They are cleared only by an explicit reopen from a holder of `open.v1` in that scope.
- **Descriptive tool descriptions.** They name the target, its Type's description, and its scope. For example: "queue: Resource in engine Pool repo, shared with packages: post and read", and "packages: manage the members of project packages".

## Schemas and core contracts (spec/core)

- **types.yaml.** `project.v1` (family Resource, empty schema, offers `project-management.v1`). `pool.v2` (schema `{audience: [...]}`, offers `pool-management.v1`, control `share.v1`). `pool.v1` stays for Pools that are never shared.
- **capabilities.yaml.** `project-management.v1`.
- **operations.yaml.**
  - `register.v1` gains the inputs `project` and `proposalExpires`.
  - Lifecycle operations gain the assertion `project`.
  - `inspect.v1`'s outputs gain the inventory and `scope`.
  - New: `retire-with-members.v1` and the control `share.v1`.
  - The management-verb list (OPS-019, CON-024) gains `retire-with-members.v1`.
- **requirements.yaml.** `bound.v2`.
- **templates.yaml.** `project`, `project-admin`, `project-approver`, `project-proposer`, `project-viewer`.
- **references.yaml.** `project-reference.v1`, a Resource reference whose type must be `project.v1`. The `project` key, if names are scoped.
- **conduit.yaml.** The engine implements `project-management.v1` and `share.v1`.

## Core operations the build needs (reference model)

- **Scope.** A scope field on every object.
- **Routing.** Access routing (crates/core/src/access.rs) as a chain walk.
- **Opens.** The two-half check for opens.
- **Audiences.** Storage and the narrowing cascade.
- **The Project Resource.** Served by the engine.
- **Proposals.** Registrant and expiry kept on a proposal; pending actions computed from the Conduits that cover a proposal.
- **Inventory.** The per-project inventory.
- **Tombstones.** Keyed by scope.

The concise form needs `project:`, `managers:`, `audience:` and `to: project X`.

## cost
These are estimates from reading. Nothing was built or counted by script.

## Specification

- **New promises.** About 17 (projects.yaml).
- **Changed promises.** About 25. About 19 if names stay engine-global, because OBJ-002, OBJ-006, OBJ-021, NAM-003, INS-025 and LIF-031 then stay as they are.
- **Total.** About 36 to 42 of about 455 promises, that is 8 to 9%. On the simplification's scale that is "major": smaller than D (about 50), B (about 60) and C (about 80), and larger than A (about 20 to 26) and the accepted package (about 11 to 14).
- **Core contracts.** About 12 documents: 2 Types, 1 Capability, 2 Operations (`retire-with-members.v1` and the control `share.v1`), changed inputs and outputs of `register.v1` and `inspect.v1`, `bound.v2`, 5 templates, and 1 to 2 reference schemas.

## The expensive part: ACC-033

- **Exposure.** It is the most cited promise: 78 conformance files, per map-mechanisms.md; I did not recount them.
- **Vectors that stay valid.** Most of them, because engine-scoped behaviour is unchanged.
- **Vectors whose outcome flips.** Every vector asserting that a Conduit to `management` alone is refused for a Pool row: registering into a live Pool without the Pool Conduit (row 1's observable), set-up, retire or inspect of a Resource by `management` (rows 4 and 6), and the reconcile routing (rows 13 and 14). I estimate 10 to 20; not counted.
- **New vectors.** About 40 to 60 for PRJ-* (two halves, audience cap and narrowing, delegation, scope refusals with no disclosure, proposals and expiry, tombstones by project), mostly safety cases.

## Reference model and shell

- **Core.** About 2,000 to 3,500 lines:
  - the scope field and its invariants;
  - routing as a chain walk;
  - the two-half check;
  - the audience and the `share.v1` cascade;
  - the Project Resource;
  - proposals and pending actions;
  - inventory and scoped tombstones;
  - qualified names, if adopted.
- **Shell and CLI.** About 1,000 to 1,500 lines: `-p` and the project argument, `managers:`, `audience:` and `to: project` in the concise compiler, the per-project drift and prune, and the pending-action list in the control panel.
- **Work orders.** About 5 to 8, after ruling 53's additions. Those should be written once, in this scoped form, so the inventory and tombstones are not written twice.

## Prerequisites outside this design

- **DL13.** How a second person gets a token. The first build cannot connect a second Interface.
- **The pending-action mechanism** of rulings 49 to 51, which is not in spec/ yet.
- **The short-form decision** (ruling 53, decision 1).

## What it saves

- **S1, concise form.** Nothing in author lines.
- **S1, canonical form.** About 32 lines (root's Pool Conduits).
- **The owner's real use.** One project's file states its managers and audiences in a few lines, where today it needs a broker or engine-wide `management`.
- **Defects removed.** It removes the simplification's CLI-only machinery that skeptics broke: an applied-IDs-only memory, a guard keyed by project name, and an operator Conduit derived by the CLI.

## what_is_lost
1. **ACC-033's "at any moment one target authorizes a request".**
   - Several Conduits can now permit the same request: to the Pool, to the Project, or to `management`.
   - Refusal diagnostics and ruling 52's second tier must list the chain.
   - Revoking one Conduit may not revoke a power that an enclosing one also gives, so the inspection must show every path.
2. **`management`'s present meaning.** A Conduit to `management` that lists `set-up.v1` or `retire.v1` now also covers every Resource of every Pool, engine-wide. Today only root holds one, but this widens every existing and future `management` holder, and it must be said.
3. **Ruling 44's simple position** that management cannot be narrowed and only a broker narrows it. Bounds now carry two meanings: a right to open, and a project role.
4. **Root's one-off cross grants into shared Pools.** Under the audience cap, root cannot give one packages Agent write on `repo/main` without raising packages' whole ceiling, or putting `main` in another Pool. I chose that the audience is the complete truth over flexibility.
5. **Project-to-project sharing.** No project Pool is shared in v0 (no Shared VPC host). Moving a project Pool to the engine later means replacing its Resources, and root rewires any secrets.
6. **Granularity finer than a project inside an audience.** A ceiling is per project, not per consumer. Inside a project, boundaries are weak, as Kubernetes warns: any holder of `register.v1` in P can run any allowed template's program and reach everything shared into P up to its ceilings. `bound.templates` narrows this, but does not remove it.
7. **Two separate approvers for one cross-project widening, in v0.** One actor must cover both halves, which in practice means root.
8. **Simplicity of global names, if scoped names are adopted.**
   - References and requests gain `project`.
   - The engine scope's names become reserved in every project.
   - A conflict can disclose that an engine-scoped name exists, as GCP's global project IDs do.
9. **The word "project" for a submission.** Every SUB-* and NAM-* text and the docs that use it change.
10. **Interfaces as members of a project** (Kubernetes ServiceAccount style). Every Interface is engine-scoped and made by engine authority, so a project's managers cannot mint principals of their own in v0.
11. **Protection between co-managers.** ACC-012 is kept: any holder covering a Conduit can close it, so co-managers can revoke each other, and revocation does not cascade (ACC-030).
12. **Project managers wiring secrets.** It is deliberately kept out: SEC-008 is untouched, so an "auth pool for a project" still needs root for every secret it reads.

## open_questions
Each question carries my recommendation.

1. **Containment, or one authorizer.** May an enclosing scope's Conduit (Project or `management`) authorize what a Pool authorizes?
   - **Recommend yes.** It is the core of the scope model, removes root's Pool Conduits, and is what Kubernetes, Azure and GCP do.
   - **The alternative** is P1-style rows with a derived operator Conduit per project. It keeps ruling 30 intact and costs one derived object per project and Pool.
2. **Project as a Resource of `core#project.v1`, or a `kind: Project`.**
   - **Recommend the Resource** for v0 (no new family).
   - A kind reads better (ruling 52) and costs about 8 to 10 more promises.
3. **Scoped names now, or later.**
   - **Recommend now,** since promises are to be written once. It brings disjoint engine and project names, qualified references and no precedence.
   - **The fallback** is engine-global names in v0.
4. **Does the audience cap root too?**
   - **Recommend yes** (the audience is the complete truth).
   - **The alternative** caps only project authority, and lets root grant outside the audience, shown as "outside the audience" in inspection.
5. **Should retiring a shared Pool, or its project, be refused while other projects hold open Conduits into it,** like PRV-020 and PLC-011?
   - **Recommend no.** It is a narrowing, so it never waits, and the plan lists every closure by project.
6. **Two approvers for one cross-project widening.** Should each half's consent be recorded on the proposal (ruling 51's two halves inside one engine) in v1, or should one actor always cover both?
7. **May an Agent hold `open.v1` on a project, and so approve?**
   - Preservation test 5 says yes.
   - The owner's "through the Interface" suggests pending actions are raised only to people.
   - **Recommend:** Agents may hold it; pending actions are raised only at Interfaces.
8. **A two-person rule or no self-approval** (GitHub's "Prevent self-review"). Not expressible beyond "do not give proposers `open.v1`". Is that enough for v0?
9. **Settings that widen without an open** (`accountServices`, a directory root moved up). Should a Type mark them (F9), or is `bound.templates` with proposers holding no `set-up.v1` enough for v0?
10. **Do tombstones bind a namesake project?**
    - **Recommend yes:** a recreated name cannot silently reopen revoked grants.
    - Clearing them takes an explicit reopen.
11. **Should project-scoped Interfaces exist** (ServiceAccount-like, such as `ci-hook` minted by a project's manager)?
    - **Recommend not in v0.** Issuing identity is an engine-level, trust-domain act (DL13).
12. **Who receives a project member's sign-ins and enrollment prompts?** Rulings 49 and 50 say root in v0. **Recommend for v1:** holders of `set-up.v1` on the member's project, plus root.
13. **Node and runtime scoping per project** (enrollment's decision 5, use case UC5).
    - Out of v0, except that a `runtime` reference stays within the member's scope or the engine's.
    - Is a node audience, mirroring the Pool audience, the right shape later?
14. **Vocabulary.** "Project" for the scope and "submission" for the documents handed over. Or another word for the scope ("space", "workspace")? Ruling 42 requires one meaning per word.
15. **Should `retire-with-members.v1` be in `core#project-admin`?** In Kubernetes a namespace admin cannot delete the namespace. **Recommend no;** root keeps it.
16. **Should the parallel short-form investigation adopt the shape here directly?** That means `project:`, `managers:`, `audience:` and `to: project X`, and the default Pool named `default`.

**Unverified:**
- the count of 78 ACC-033 conformance files, and the estimate of how many vectors flip;
- the source-line estimates;
- my reading of P1 (I have not seen it);
- the rulings 49 to 53 mechanisms, which are not yet in spec/;
- whether `pool.v1` → `pool.v2` and `bound.v1` → `bound.v2` must be new versions or can extend the core package in place (I assume new versions, per ruling 25's versioned names).

## Verdict authority (major)
P3's core idea holds up: the Project as a scope and an engine-served target, membership fixed on each member, one home per Pool, and audiences that grant nothing. Its authority rules do not hold up yet. I found 23 counterexamples: 1 critical, 8 high, 12 medium and 2 low. The four worst: (1) in v0, anyone who may register and set up a running member can act as root; (2) a "private" project Pool such as `auth` can be reached from another project through any actor that holds authority in both, and P3's own coordinator is such an actor; (3) Agents that hold authority in another project can be driven by whoever controls their home project; (4) the check that the consumer's project agrees is skipped when a Conduit is opened through `access` or names an Interface. None of these needs a sixth family or abandons the thesis. All are fixable by changing rules.

I only read files; nothing was modified. Sources: spec/conventions/access.yaml (ACC-001 to ACC-039), spec/promises/{conduits,secrets,providers}.yaml, spec/conventions/lifecycle.yaml, spec/interface/implementation.yaml (IFC-012 to IFC-014), spec/core/{capabilities,operations}.yaml, the records of rulings 49 to 53, simplification/research/design-E-layered.md, and reference-model/crates/shell/src/{lib.rs,runner.rs,server.rs,store.rs}.

## Counterexamples, most severe first

**C1. CRITICAL (in v0): anyone who can run a member can act as root, so projects do not separate people.**
- Why it works:
  - Every restriction is only mediated in this version (ACC-027). Placement and `runtime` carry no authority (PLC-015, PRV-006).
  - The first build starts each run as the same OS user, with only `env_clear` and no sandbox (runner.rs:87).
  - Root's client token sits in the engine's store at client-tokens/<root-id>. It is 0600, but readable by that same user (lib.rs:280-295, server.rs:787, store.rs).
- Steps:
  1. collab, admin of packages only, registers a Codex Agent placed on `local` (or on the managed WSL machine) and sends `packages.set-up.v1`.
  2. Its shell reads root's token and calls the engine as root.
- Neither the coordinator's `bound.templates: [pkg-worker]` (pkg-worker is Codex, which runs shell commands) nor any proposer flow prevents this.
- P3's list of what collab may not do (install, purge, inspect conduit-core) is therefore false in v0.
- This is pre-existing for every Agent, but P3's purpose (a second person limited to one project) depends on closing it.
- Fix:
  - Through a Project, allow register and set-up of an instance that runs only on a runtime provider whose contract confines what it runs. Restrictions then become enforced (ACC-027).
  - Add an engine-set list, per project, of the runtimes and nodes its members may use: a node audience mirroring the Pool audience.
  - Make the store unreadable by runs (a separate OS user).
  - Until then, state plainly (ruling 52) that a Project organizes objects and does not isolate people.

**C2. HIGH: "private" project Pools are not private.**
- P3 lets one opener who covers both halves connect two projects.
- Its own coordinator holds admin in both conduit-core and packages, so it can open `pkg-worker-033 -> linear [list,get,create,update]`:
  - target half: `linear` is a conduit-core member;
  - consumer half: the worker is a packages member.
- P3's example 2 lists exactly this as refused. It is not.
- A trigger exists: packages workers may post to the shared `repo/queue`, which the coordinator reads.
- Root and bounded-`access` holders (C4) can also cross.
- Fix:
  - A project-homed Pool gets an implicit empty audience that caps every opener, root included (symmetric with PRJ-012). Crossing means re-homing the Pool at the engine with an explicit audience.
  - A cross-project open needs two separate acts by two authorities, at least when the opener is an Agent.

**C3. HIGH: Agents holding authority in another project are deputies of whoever can drive them.**
- Steps:
  1. dana, admin of conduit-core only, opens `dana -> coordinator [turn]`. This is allowed: the target is a conduit-core member, and an Interface is an acceptable consumer by the target side alone.
  2. dana then drives the coordinator to register, retire and open in packages.
- packages consented to "coordinator". The effective grantees are everyone conduit-core lets take a turn on it. In Kubernetes terms, binding a Role to another namespace's ServiceAccount gives it to everyone who can run pods as that account.
- Persistence: collab makes `pkg-helper` with admin and `delegates`, and keeps her own turn Conduit to it. Closing collab's Conduit to the project leaves her in control, because revocation does not cascade (ACC-030).
- Fix:
  - Opening a turn or control Conduit to an Agent also needs the half of every Project that Agent holds authority in. Alternatively, only Interfaces and the project's own members may hold Conduits to it.
  - PRJ-014 lists each Agent's effective controllers.
  - Removing a manager also closes that manager's Conduits into the project's members.

**C4. HIGH: the consumer-half check is skipped when opening through `access`.**
- A bound limits targets only, and its holder may open "for any consumer" (ACC-029, ACC-030).
- Steps: root gives dana `access` bounded to `pools: [auth]`, today's way of delegating grants. dana then opens `pkg-worker -> linear` with no consent from packages.
- In the same way, any bounded holder can push Conduits into another project's Agents.
- Fix: PRJ-007's consumer half applies to every open. A consumer that is a project member needs that project's half, or an unbounded right to open.

**C5. HIGH: the rule that any Interface may be made a consumer by the target side alone.**
- collab opens `root -> pkg-x` with `as: linear`. Consequences:
  - (a) No one can ever close it, root included (ACC-024). Only retiring `pkg-x` ends it.
  - (b) IFC-014 counts registered but unopened Conduits, so collab can squat `as` names in any Interface's tool namespace. Root's later `as: linear` then fails to instantiate.
  - (c) packages' tools and descriptions appear in root's namespace, which is often driven by an agent session: a prompt-injection channel into the most powerful principal.
  - (d) collab can expose members to an Interface a peer engine was admitted with (ruling 51), or, holding `delegates`, make it an admin. An outside party then runs code on the owner's nodes.
- Fix:
  - An Interface as consumer needs that Interface's consent (a pending action at its control panel) or engine authority.
  - Only root opens Conduits whose consumer is root.
  - Peer-admitted Interfaces carry an engine-set list of the projects they may be granted in, and are never given `project-management.v1` without root.

**C6. HIGH: registration checks only the target.**
- Steps: a packages proposer registers a Conduit whose consumer is `{Agent, conduit-core, coordinator}`, `as: queue`. That gives it:
  - an oracle for whether the coordinator exists;
  - through IFC-014's diagnostic, which names "the Conduit that has the other", a list of the coordinator's Conduit names;
  - a squat on the name `queue` for the coordinator;
  - a pending action raised to conduit-core's approvers with content packages chose.
- P3 also contradicts itself here: "register through P registers into P", yet "a Conduit's home is its consumer's project".
- Fix:
  - Registration needs the consumer half too.
  - Redact IFC-014's diagnostic to the first tier when the other Conduit is not inspectable.
  - A Conduit registered without consumer-side authority reserves no names.

**C7. HIGH: `replace.v1` defeats `bound.templates`, P3's "standing approval".**
- `replace.v1` keeps the template's kind and name and only advances its revision (operations.yaml). Bound references are frozen IDs (OBJ-021), so the bound still names the replaced template.
- `core#project-admin` includes `replace.v1`. The bounded coordinator can therefore replace `pkg-worker` (new Type, `remote` URL, `accountServices` on, wider declared Conduits) and then register "allowed" workers from it.
- Fix: a bound with `templates` pins each template's ID and revision, and its holder may not replace those templates or register new ones unless the bound lists them.

**C8. HIGH: secrets are root-wired, but everything around them becomes project-administered (shape (a)).**
- Secret Resources and secret stores run programs that receive the store's own credential (SEC-010).
- A conduit-core admin may set them up or retire them, replace their project templates, register new secret Resources into `auth` (one Conduit to the project now satisfies both halves of row 1), and open control Conduits to `linear`.
- Each re-wiring needs root to set up or open. Root becomes the deputy, and the vault's service-account token leaks to a program the admin chose.
- "SEC-008 untouched" holds in letter only.
- Fix:
  - Like PRV-013 for machine providers: secret Resources, secret stores and every consumer of a get-secret Conduit are engine-scoped, with engine templates. Only shape (b) is allowed.
  - Root's pending action shows the program, `remote` and item differences from the predecessor.

**C9. HIGH: root's control panel becomes a surface for phishing and fatigue.**
- In v0 every sign-in goes to root (ruling 49). The account expected is named in configuration that project admins write.
- Steps: collab configures an Agent for the owner's personal Google account. Root completes a sign-in that looks routine. The account check passes, because the configuration names that account.
- P3 also makes root an approver of every proposal in every project, so any proposer can flood root with pending actions.
- Fix:
  - Each pending action names the project, the registrant chain and the program digest.
  - The accounts each project's members may sign in to are an engine-set list.
  - Root sees project proposals but is not prompted, unless the change needs engine authority.
  - Cap each proposer's outstanding proposals.

**C10. MEDIUM: shared writable Pools are channels between projects.**
- `repo/queue` lets packages inject into conduit-core's coordinator, an admin, and lets it read conduit-core's orders.
- Reach through deputies (ACC-016) and credentialed shared brokers (one token, no partition by project) make P3's claim that the audience is "the complete truth" false.
- Fix:
  - Drop that claim. Inventory shows reach through deputies.
  - An audience may carry filters per project.
  - The `serving` chain carries the origin project, and a shared Resource's contract may declare that it partitions by it.
  - The plan flags any Pool one project writes and another project's admin Agent reads.

**C11. MEDIUM: adding a Resource to a shared Pool silently widens every project in its audience.**
- Fix: classify such a registration as a widening for each project in the audience, or let audiences name Resources.

**C12. MEDIUM: audiences may hold controls.**
- Controls are authorized by the instance itself (row 11), so one project could reconfigure a shared Resource for all.
- Fix: no controls in audiences.

**C13. MEDIUM: target-side revocation fails.**
- PRJ-009 makes closing a Conduit to the project need `delegates: true`. collab, who has no `delegates`, therefore cannot close the coordinator's Conduit to packages, although example 3 says she can. Narrowing then waits for root.
- Renaming `as` evades tombstones (design-E 3(b)).
- Fix:
  - Closing any non-root Conduit to the project needs only `close.v1` there.
  - Key tombstones by consumer, target and capability, not by `as`.

**C14. MEDIUM: race during retirement.**
- `retire-with-members.v1` retires the project last, so the managers' authority stays live during the cascade and they can add members mid-way.
- Fix: from dispatch, mark the project retiring. Close its authority Conduits first, refuse registers and opens into it, then retire members.

**C15. MEDIUM: engine objects may depend on project objects.**
- PRJ-005 binds only members, so an engine object may name a project's runtime provider or template. The project's admin then controls code that runs with engine authority, and PRV-020 lets the engine object block the project's retirement.
- Fix: engine-scoped objects reference only engine-scoped objects.

**C16. MEDIUM: which project a cross-project Conduit belongs to, and where its names resolve.**
- `coordinator--packages` is homed in conduit-core, so:
  - its `bound.templates` resolve in conduit-core, where a conduit-core admin can plant a template named `pkg-worker`;
  - packages cannot inspect or unregister a grant over itself;
  - packages' revocation tombstone lives in conduit-core.
- `packages.yaml` writes `managers: coordinator` unqualified. If collab registers an Agent `coordinator` in packages, it captures the grant when root applies the file.
- Fix:
  - Authority Conduits are homed at their target, as a RoleBinding or ReferenceGrant is.
  - Foreign names are written qualified and pinned by ID in the plan.
  - The concise form refuses unqualified foreign names.

**C17. MEDIUM: reserving engine names in every project is an oracle and a squat.**
- Registration conflicts reveal which engine-scoped names exist: Interfaces, engine Agents, Pools that are not shared.
- A project admin can block root from creating an object or a Project of a chosen name. A Project is a Resource, so a project Resource named `billing` blocks a Project `billing`.
- Fix: no fallback lookup. Project documents name engine objects only with an explicit engine qualifier.

**C18. MEDIUM: tombstones keyed by project name.**
- They leak the old project's member names to a namesake project's admins, through refusals.
- Fix: key them by project ID. Show the old tombstones only to engine authority.

**C19. MEDIUM: "narrowing never waits" covers destructive operations.**
- `retire.v1` removes an Agent's home (ruling 49) and runs retire hooks. `core#project-proposer` holds it, so an injected pkg-lead can wipe the project at once.
- Fix: proposers get only `close.v1`. Retiring a member one did not register is a proposal.

**C20. MEDIUM: containment widens `management` over `system`.**
- Brokers and co-admins could set up or retire the machine provider, package-source providers and nodes.
- Fix: keep `system` outside containment.

**C21. MEDIUM: Agents as approvers.**
- The coordinator approves its own widenings and pkg-lead's. With C10's injection, a proposer can in effect approve itself.
- Fix: opening a Conduit someone else proposed needs an Interface, unless the project explicitly declares an Agent approver.

**C22. LOW: P3 contradicts itself.**
- `core#project-approver` cannot decline, because declining is `unregister.v1` and the role lacks it.
- A bound with only `templates` covers nothing under ACC-029's empty-list rule; read the other way, it covers everything.
- `share.v1` is said to be "authorized by management", but `object-management.v1` does not list it.
- "The audience admits C's consumer's project" is undefined when the consumer is an Interface.

**C23. LOW (a decision for the owner): can project B learn that A exists?**
- Yes, through:
  - the project and Interface names P3 lists to every project holder;
  - full audiences returned when an engine Pool is inspected;
  - tool descriptions;
  - the inventory of Conduits crossing the boundary;
  - IFC-014's diagnostics;
  - the contents of shared Resources;
  - name-reservation conflicts.
- Ruling 52's first tier hides existence only in refusals.
- Recommendation: as in Kubernetes, a project holder cannot list projects, sees another project only through a relationship into its own, and sees audiences filtered to itself.

## What survived the attacks
- Membership is fixed and grants nothing.
- No Pool is homed in two projects, and shares are non-transitive.
- `system` is never in a project or an audience.
- A bound naming a project never covers Pools shared into it.
- Only root opens get-secret Conduits, as SEC-008 states.
- Approvals are bound by `expectedId`.
- An audience caps root as well.
- Retiring a project never touches engine Pools.

## Direct answers to the angle's questions
- Who may open a Conduit from a member of A to a Resource in a shared Pool: A's managers, within A's ceiling, and root.
- Can the Pool's owner revoke it: only root, or only coarsely through `share.v1`. Add `close.v1` to Pool authority for Conduits into its own Resources.
- Can an agent approve its own widening: yes, if it holds `open.v1`.
- Can a project's Interface approve a widening into a shared Pool: yes, within the ceiling, and that is sound.

## Process note
The approvals requirement (widenings as pending actions, narrowing never waits, approvers declared) is not in the owner's relayed words or in docs/proposals/owner-direction/2026-11-10.md. P3 should present it as the coordinator's answer to ruling 53's open decision 2, not as the owner's direction.

## Unverified
- C1 rests on reading runner.rs and the token code, not on a run.
- The Kubernetes analogies are from memory.
- I did not check whether any work order adds sandboxing or tokens for a second Interface.

## Verdict expressiveness (major)
P3 can state the namespace shape the owner described: one home per object, Interfaces holding authority over one or more projects, a skills Pool for all projects, an auth Pool for one project, and S1 at no extra author cost. It fails several required refusals, though. Its narrowing tools each lack a dimension. bound.templates is bypassed by replace.v1. Bounds cannot limit the consumer, and Interfaces can be granted access by the target's project alone. Audience ceilings have no filter dimension. An approval can only be an open, so it cannot cover a setup. Cross-project proposals are refused at registration. Nodes and runtime providers have no scope per project. The design's own work-order example (example 2) contradicts itself. Nothing I found is fatal. Each gap can be closed with a targeted addition, but several reopen how bounds are designed (ACC-029/ACC-030).

Method: I read P3 against the use-case inventory UC1-UC10, ROADMAP S2-S10, A78 and WORK_ORDERS.md "Running the orders through Conduit". I checked in the repo, read-only: bound.v1 (spec/core/requirements.yaml: every input `default: []`, and "a Conduit whose bound names nothing covers nothing"), ACC-005, ACC-029, ACC-030 (a bound "does not limit the filters ... may open, for any consumer"), replace.v1 (spec/core/operations.yaml:279; keeps kind and name, any new content), SUB-009, INS-020, OBJ-013, PRV-006, LIF-012, LIF-022, and IFC caller metadata (spec/interface/implementation.yaml:1008: an implementation learns only the caller's and the Conduit's IDs). Severities below are per finding. Overall: major.

== MAJOR: what P3 cannot express, or where a required refusal fails ==

M1. bound.templates is bypassed by replace.v1, so the create right cannot be capped.
- core#project-admin lists replace.v1. replace.v1 keeps the template's kind and name (so its identity and its place in the bound) but accepts any new content: Type, program, `remote`, declared Conduits, settings (operations.yaml:279; SUB-009).
- P3's list of widenings leaves out replace.v1. The use cases' R3 lists it.
- Failure in the design's own example: the coordinator in packages holds `bound: {templates: [pkg-worker, pkg-tree]}`. It replaces pkg-worker with a claude-code template that has `accountServices: true` and declared Conduits, then registers from it.
- This breaks UC2's required refusal "the coordinator registering an Agent from any template but `worker` ... without approval".
- Unbounded project admins (collab) are uncapped anyway. A project-homed Pool has no ceiling, and a Resource's configuration is opaque. Collab can register `local-directory root: ~` (or a runtime provider with host mounts, or a `remote` pointing anywhere) and open members to it. That is an open within collab's authority, so it needs no approval and no audience. Lesson 5 (a scope is only as strong as its weakest create right) is not answered.
- Fix: a template named in any holder's `bound.templates` can be replaced or unregistered only by an unbounded holder, and replace.v1 counts as a widening. Separately, decide whether project-homed Pools get a ceiling, or Types mark settings that widen (F9).

M2. Example 2 contradicts itself, and one actor managing two projects joins them.
- Reading 1: bound.v2 keeps v1's semantics, so absent `operations` and `targets` cover nothing (ACC-029, `default: []`).
  - The coordinator's `bound: {templates: [worker, tree]}` then authorizes no open at all, so "what follows" fails.
  - Its `templates` list also omits the Conduit templates it must register from to make the worker's standalone Conduits (`queue-poster` and similar). Even registering the worker's reach is refused.
- Reading 2: absent dimensions mean unlimited.
  - The coordinator holds open.v1 in both conduit-core and packages, so it covers both halves. It can open pkg-lead -> linear [create, update].
  - That refutes the example's own claim: "Refused: a packages member reaching linear ... (no conduit-core half)".
- In general, an Agent admin of two projects is a bridge between them, and nothing displays it as one.
- Fix: specify bound.v2 semantics for absent dimensions. Have the example list operations, pools and Conduit templates. Have inspection report "X bridges A and B" wherever one actor covers both halves.

M3. The bound has no consumer dimension, and any Interface can be granted access by the target's project alone.
- bound.v2 still limits only targets, operations and templates (B4/G3 are unchanged).
- The consumer half is satisfied whenever the consumer is an Interface.
- Consequences:
  - A bounded Agent admin (the coordinator) can expose its project's members to any Interface: ci-hook, or a peer engine's admitted `partner-main-packages`. No person is involved.
  - UC6's required refusal "the partner acting outside packages" fails against conduit-core's Agent admin.
  - UC2's envelope ("open a worker's Conduits") cannot be stated, because the bound cannot say "consumers: workers only".
  - The pairing "worker-033 with tree-033 only" cannot be stated. UC9's "worker of 032 reaching tree-033" is refused only because workers hold no right to open. The coordinator can still open it.
- Fix: bound.v2 gains `consumers` (by scope or template, or "instances I registered"). An Interface consumer then needs engine authority, or an Interface allow-list on the Project.

M4. Revoking a manager is not one act.
- ACC-030's non-cascade is kept, and the inventory (PRJ-014) records no opener.
- Collab closing coordinator -> packages leaves open the coordinator's turns to existing packages workers, and every Conduit it opened for them.
- UC9's required refusal "dispatch into packages after collab has closed the coordinator's authority" fails until collab finds every crossing Conduit and closes it individually. Who opened each is only in records, which root can purge.
- This is S6 (revocation in flight) at the project level.
- Fix: make the opener engine state, and add a narrowing "close everything X opened in P", or make crossing turns depend on their opener's Conduit to P.

M5. Approval exists only for open.v1, while the owner's direction covers every widening.
- core#project-approver lacks set-up.v1, and the pending action's subject is one Conduit's open.
- These cannot be raised or approved as pending actions:
  - a proposed new instance, such as UC4's pkg-worker-docker or UC9's "a worker that needs more", which needs register, then set-up, then opens;
  - a setup whose declared Conduits open (P3 lists it as a widening, but no role can approve it);
  - widening controls such as `accountServices` (ruling 48), which P3 admits;
  - replace.v1 (M1).
- The evidence lists the target's name and scope, not its configuration. A proposer holding register.v1 can register `tree-033` with `root: ~/.ssh` and propose an innocuous-looking rw open.
- Fix: the pending action's subject is a bundle of requests, each with its expectedId. The approver role covers set-up of proposed instances. The evidence shows the target's resolved configuration.

M6. Cross-project proposals are impossible.
- A proposal that names a target its proposer may not inspect is refused when it is registered, with ruling 52's first-tier answer.
- pkg-lead cannot propose access to conduit-core's `linear`. That is UC4's own flagship flow, with approvers packages {collab, root} and auth {root}. UC10's promotion cannot be requested from the consumer side either.
- P3's example 4 avoids the problem by using `queue`, which is already shared into packages.
- Pending actions can only ask for things the project can already see, so the rest goes out of band, for example over the shared queue.
- Fix: allow a proposal against a name the proposer cannot inspect, routed only to the target side's approvers. The proposer learns pending, declined or expired and nothing about existence. This needs the owner to rule against ruling 52.

M7. Audience ceilings have no filter dimension.
- Bounds do not limit filters (ACC-030 observable, kept), and audiences cap operations only.
- Every project opener may open unfiltered Conduits to a shared Resource.
- S2 ("one login, two restrictions"): E07, the bounded opener dropping a filter, now applies to every project manager.
- S8 ("only what the caller may see") fails across projects. The implementation receives the caller ID but not the caller's project, so a shared search or history Resource cannot partition by project either.
- UC9: P3's single shared `repo/queue` lets packages workers read conduit-core's orders. The only way out is one queue per project.
- Fix: audience entries can require filters, the open check enforces them, and the engine passes the caller's scope in `_meta`.

M8. Placement and runtime have no scope per project (UC5, part of S3 and S10).
- Nodes are engine-scoped, and placement carries no authority. Any project admin places members on any node, so "win-wsl for packages only" cannot be stated.
- A runtime provider homed at the engine can be used by every project whatever its Pool's audience says, because the audience governs Conduits and `runtime` needs no Conduit (PRV-006).
- The owner's WSL and enrollment direction needs both.
- Fix: a node audience, and runtime providers that obey their Pool's audience when an instance is registered (P3's own open question 13).

== MEDIUM: expressible only awkwardly ==

D1. A project becomes a trust boundary, not a product boundary. Laying out A78 under P3 with an Agent coordinator:
- It cannot be engine-scoped: no project can grant an engine Agent the consumer half, so it could not open its own turns to workers.
- It must be admin of its home project to open its own crossing turns. So that home must hold nothing it should not administer, such as auth or triage.
- Merging into `main` needs write in its home project's ceiling, and every opener there can then hand write on `main` to anyone in that project.
- Result: about four or five projects for two products. The clean expression is a coordinator that is an Interface. A78's admin really is an outside session, and Interfaces are neither capped nor limited by halves. The design's example should say so.

D2. Ceilings are per Pool and project, not per consumer or Resource.
- Different access shapes need separate Pools.
- Root cannot make a one-off exception (PRJ-012).
- Promoting a Pool to shared needs recreation: pool.v1 cannot become pool.v2, and a project-homed Pool cannot move to the engine. That means new IDs, closed Conduits and root rewiring secrets. UC10 is badly expressed if auth started in a project.

D3. Narrowing an audience closes whole Conduits.
- There is no change in place (ACC-011), so the still-allowed operations go too.
- Tombstones then block re-applying the project file, and each project must explicitly reopen narrower Conduits.

D4. A project file's `managers:` and crossing reach compile to Conduits whose home is the engine, or the consumer's project (conduit-core for the coordinator).
- So packages' drift and prune (PRJ-017) cannot see or remove them.
- Applying `managers:` needs `delegates` or root, so a project's own admin cannot apply its own file whole.

D5. The proposer role is too broad.
- core#project-proposer holds close.v1 and retire.v1 over every member, and narrowing never waits.
- An injected pkg-lead can retire other agents and close collab's turn Conduits.
- UC4's "may narrow what it may reach" (its own grants only) cannot be stated.

D6. Agents as approvers.
- The approvers are whoever holds open.v1, Agents included (open question 7). Pending actions reach only Interfaces, but pkg-lead can ask the coordinator over the queue, and the coordinator opens.
- Any opener in a scope clears a tombstone, so an Agent can reopen a grant a person revoked.
- "Only people approve" and "a person's revocation is undone only by a person" can be stated only by giving Agents no open.v1, which ends A78's dispatch without waking anyone.

D7. No authority over all projects short of `management`.
- An admin or approver over every current and future project, but not the engine, would be the analog of a ClusterRoleBinding.
- That needs `management`, which also covers install, Interfaces, engine Pools and Projects.
- Audiences have `all`; authority does not.

D8. An engine-scoped Agent cannot be offered to projects as a service.
- Audiences apply only to Pools, so "a reviewer Agent for all projects" needs root for every wiring.
- The consumer-half asymmetry between Interfaces and engine Agents strains preservation test 5.

== MINOR ==

- "retire.v1 on P reports conflict while any member is not retired" is undefined for templates, which have no lifecycle (LIF-012), and for registered Conduits (proposals).
- Registering into P after retire-with-members is dispatched is not refused, which UC7 requires.
- What happens when retire-with-members fails partway is unstated (S4).
- Routing sign-ins and enrollment per project is deferred, as rulings 49 and 50 allow. UC3 only has a key for v1.
- Whether a project holder can read records is unspecified.
- Name squatting: a project can take a name root later wants at the engine scope, and a proposal can take a member's `as` in its tool namespace (IFC-014).
- Nothing limits how many proposals reach the approvers.
- Every project holder sees all Interface names, a peer's admitted Interfaces included.
- If names stay engine-global (the fallback), UC8 fails and the `default` Pool collides, as P3 acknowledges.

== WHAT P3 STATES WELL ==

- UC1 and S1: no extra author lines, no derived operator Conduit, drift and prune limited to members.
- UC3 core: collab as admin over packages, and the refusals across projects without disclosure.
- UC7: retirement leaves shared Pools alone and namesakes inherit nothing.
- UC8, with scoped names.
- The owner's skills and auth examples, with SEC-008 kept.
- S5: share.v1 with a revision.
- S9: first start unchanged.
- S10: as today; swaps are root-heavy but expressible.
- Narrowing never waits.
- One semantic shift worth putting to the owner: an audience acts as standing consent from the target side. So collab alone may give packages members skills read within the ceiling, where UC3 and UC10 assumed root would also approve.

None of this was run. These are paper traces against the spec text and P3's own examples.

## Verdict cost (major)
P3 is coherent and buildable, but it costs more than it says. Its own figure is about 36 to 42 promises (8 to 9%). Counted against the repo, it touches about 46 to 56 promises (10 to 12% of 455) if the scope gets a word other than "project". If today's "project" is renamed "submission" as proposed, it touches about 93 to 103 (20 to 23%). So P3 is about the size of design D (about 50) or larger, not smaller as claimed. The one expensive core change is still ACC-033's scope chain, and it flips only 14 conformance steps in 5 cases. The reference-model and store-migration costs are close to P3's estimates. Recommendation: accept the cost model with five corrections. (1) Choose a new word for the scope rather than renaming "project". (2) Extend pool.v1 and bound.v1 in place rather than adding v2 versions. (3) Fix the restated reconcile rows 12 to 14. (4) Add a consumer-side check when a Conduit is registered through P. (5) Add ruling 37 to the list of rulings amended.

Everything below was checked by reading <home> (read-only) and by scripts over conformance/ and spec/. Nothing was modified.

## Cost table

| Item | P3's claim | Checked | Notes |
|---|---|---|---|
| Promise total | about 455 | 455 | Counted the `- id:` entries in spec/ |
| New promises (PRJ-*) | about 17 | about 18 to 20 | Two rules are missing: one for registering a Conduit through P whose consumer is outside P, and one for a `project` argument that names a Project registered in the same submission. The generic pending-action promises (rulings 49 to 51) are a prerequisite and not counted. |
| Changed promises | about 25 (19 with global names) | about 28 to 36 | P3 misses about 9 to 11 of them (listed below). |
| Vocabulary rename "project" to "submission" | 7 promises plus definitions | 54 promises plus about 58 uses in definitions.md, about 390 uses across spec/, 405 in the reference model, about 1,440 in docs | "Submission" already means something different (definitions.md:67). Renaming also breaks the sentence "A package is a project that has been published" and CTR-001 to 012's "its project is submitted". Choose a new word for the scope instead. |
| Total promises | 36 to 42 (8 to 9%), "smaller than D" | 46 to 56 (10 to 12%) with a new scope word; 93 to 103 (20 to 23%) with the rename | With the rename it is larger than C (about 80). |
| Rulings amended | 30 (OL2), 44 (D03/D04), 42 | Add 37 and 41 | Ruling 37 made CON-028 ("a bound is rejected on a Conduit whose target is not `access`") and defined "project". Ruling 41 made the engine implement exactly three capabilities. Ruling 44 itself foresaw "a list of allowed templates on access to management", which matches `bound.templates`. |
| Core contract documents | about 12 | about 20 to 30 | P3's own list already comes to 14 to 16 (it forgot the `shared-pool` template). Also affected: the descriptions of open.v1 and close.v1, which state the bound rule; the `project` assertion on set-up, retire, unregister and inspect; the exports and `implements` in conduit.yaml; the metadata or instance schemas. Up to 12 reference schemas change if names are scoped. `share.v1` would be the core package's first control (operations.yaml says "The core package has no control"). |
| v2 or in-place versions | assumes pool.v2 and bound.v2 | in place is possible | Ruling 44 added `expectedId` to v1 operations in place, and NAM-024 allows optional additions. pool.v1 can gain `audience` and a control in place, so no Pool needs migrating. bound.v1's inputs state `default: []`, and NAM-024 forbids a default on an added input, so `templates` and `delegates` can be added in place only without defaults (absent means unrestricted, or false). |
| ACC-033 conformance files | 78 | 78 (76 cases, plus manifest.yaml and README) | 59 of them are request cases that stay valid. |
| Vectors whose outcome flips | 10 to 20, including reconcile | 14 steps in 5 cases; no reconcile flips | access/routing steps 1 and 6; s1-pool-authority 1 and 4; entry-authority 0, 2, 4, 6, 8 and 10; fixed-kind-references 2; language/first-start/protected 2 to 4 (refused becomes invalid under FST-006, because `management` now encloses `system`). Also the README paragraph about protected cases, and 1 shell test (crates/shell/tests/cli.rs: inspecting `repair-me` without a Pool Conduit). Reconciles in the cases go through root-workspace, which stays permitted. Cases compare objects and records only on the fields they list, so a new `scope` field breaks nothing. |
| New vectors | 40 to 60 | plausible: about 10 to 15 case files | The existing access cases average about 730 lines (16,135 lines across 22 cases), so expect about 7,000 to 11,000 fixture lines. case.schema.json and the runner need new inputs (inventory, pending actions). |
| Reference model, core | +2,000 to 3,500 lines | plausible: about 20 to 35% on a 10,465-line core | Routing in access.rs (729 lines) is a 70-line first-match table, and roughly doubles. Then registration.rs (scope, `project`, names), lifecycle.rs and retirement.rs (retire-with-members, the `share.v1` cascade), requests.rs (the engine-served Project, inventory) and records.rs. Tombstones and inventory (ruling 53) are not built yet. |
| Reference model, shell and others | +1,000 to 1,500 | plausible | author.rs and main.rs currently look for "no root management route to owning Pool"; this gets simpler. The concise compiler does not exist yet. The conformance crate needs +200 to 400. The loader changes little, since schemas are data. All 4 crates rebuild. |
| Store migration | not costed | low | Objects are untyped values, and a missing scope means the engine scope. New state goes behind `serde(default)` in State and in the shell's Stored (version 1). There are two catches. (a) Every existing holder of `management` silently gains Pool rights on upgrade, including over `system`'s machine and package-source providers and over Projects; ruling 52 calls for an upgrade report. (b) With pool.v2, sharing an existing Pool means retiring and re-registering it and having root rewire its secrets. |
| Reading cost | not costed | about +12 to 15 concepts | Added: scope and membership, the Project target, `project-management.v1`, the containment chain, audience and ceiling, a first core control, two halves, `delegates`, `bound.templates`, 4 role templates, proposals and `proposalExpires`, computed approvers, the approval pending action, `retire-with-members.v1`, qualified and reserved names. Removed: "one target authorizes" (replaced by a harder chain) and root's Pool Conduits (a practice, not a concept). The access chapter gets clearly harder. |

## Promises P3's change list misses

- **OPS-019 and CON-024.** Management verbs are only "of `management`, `access` or a Pool".
- **POOL-015.** An entry's setup is authorized by the Pool Conduit.
- **ACC-005.** A setup's opens are decided only through `access`, yet project admins set up workers.
- **ACC-015.** Requests to `management`, `access` or a Pool that carry a filter.
- **OPS-029, LIF-037 and OPS-023.** These are the disclosure tiers ("management's alone"; "only a Pool's authority reports refused").
- **MAN-012.** It needs changing if `bound.v2` is added or `implements` changes.
- **OBJ-018 or TPL-*.** Metadata is closed (`additionalProperties: false`), and templates have no standard arguments.

## Defects in P3

1. **The restated rows 12 to 14 are wrong.** "Any target that would authorize the original request now" contradicts ACC-033's observable: an invocation that is not a management verb is never reconciled through a Conduit to the instance. Row 14 should read "the owning Pool or any enclosing scope".
2. **Removed members lose their scope.** Membership lives only on the member, so once a member is removed the engine cannot tell which project its invocations belonged to. Only `management` could then reconcile them. Invocations and records must store the scope when they are made.
3. **Proposals can squat names.** Registering through P a Conduit whose consumer is an engine Interface, root included, reserves that consumer's tool names under IFC-014 and the Conduit's name under OBJ-002 while it waits. This needs a check on the consumer side at registration.
4. **Two places for the project.** P3 states it both as `metadata.project` and as an OBJ-013 standard argument. Pick one; both require schema changes.
5. **A paper-count slip.** S1 saves 40 lines gross and 32 net. Its own count of core documents also comes to 14 to 16, not about 12.

## Confirmed claims

- FST-001 to FST-006 are unchanged in text, and first start makes no project.
- POOL-001, CON-001 and OBJ-020 are unchanged, since a Project is a Resource.
- SEC-008, ACC-024 and ACC-032 are kept.
- Pending actions, tombstones and inventory are not in spec/ yet, and are not in the reference model.

## Evidence

- spec/conventions/access.yaml (ACC-033)
- conformance/conventions/access/{routing,s1-pool-authority,entry-authority,fixed-kind-references}/
- conformance/language/first-start/protected/
- reference-model/crates/core/src/access.rs:235-305
- reference-model/crates/shell/src/store.rs