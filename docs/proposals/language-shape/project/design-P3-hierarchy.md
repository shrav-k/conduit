Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Flat Projects under the engine. Nested hierarchy tested and declined for v0. Authority is inherited only from the engine and along the ownership path. Pools have one home and explicit audiences.

## name
Flat Projects under the engine. Nested hierarchy tested and declined for v0. Authority is inherited only from the engine and along the ownership path. Pools have one home and explicit audiences.

## thesis
Conduit already has a two-level hierarchy. The engine (`root`, `management`, `access`, `system`) plays the part of a GCP organization, and its authority covers everything. This design makes the second level real and stops there. A Project is a sixth family: a scoped `management` plus a scoped `access` over its own members. There are no folders and no nesting.

Tested against S1, the owner's real use, multi-team use and the prior art, nesting with authority inheritance does not earn its complexity in v0. It gains two things, and both are multi-team v1 needs: zero-line authority over a growing set of projects, and delegated creation of projects. It costs five things, and each one cuts against a ruling:
- silent coverage of future projects × their future Pools × their future Resources (ruling 52, explicit);
- union grants that a child cannot refuse, because Conduit has no deny and ruling 41 removed policies;
- approver sets that a project cannot narrow;
- moves, which would be the language's first in-place re-homing (POOL-001, PLC-009);
- an ancestry walk inside every decision, refusal tier (ruling 52) and inventory (ruling 53).

Most decisively, inheritance contradicts the owner's own sentence, "projects determine an Interface's permissions". Under inheritance a parent's grant applies inside the project, yet the project can neither see it among its own Conduits nor close it.

So the rules are:
1. Each object has one home, fixed at registration and recorded on the object.
2. Authority follows home only: engine → Project → its Pools → their Resources. It never follows a share.
3. A Pool reaches other projects only through audience entries that its home grants, each with an operation ceiling.
4. A Conduit that crosses projects needs both halves (ruling 51, decision 4).
5. An approval is a proposal routed to Interfaces declared as approvers. It runs under the approver's own authority, so nobody approves beyond what they hold.

Strong team boundaries belong in separate engines (ruling 51 peers), as AWS uses accounts and Kubernetes uses clusters. If many teams ever share one engine, the safe residue of folders is a v1 "project set": explicit, not nested, and joined only by a two-sided, approved widening whose evidence lists everything the join confers.

## model
Notation: `core#x` means `github.com/shrav-k/conduit/core#x`. Promise IDs are today's. New IDs (PRJ-, PND-, POOL-02x) are proposals.

1. SCOPES
There are exactly two kinds of scope.
- **The engine.** It is the root and the only parent, as today.
- **Projects.** Projects are flat. There are no folders, Projects never nest, and there is no default Project.

An object registered without a project is engine-level. It means exactly what it means today and is authorized exactly as today. Every document written so far stays valid, and nothing changes for an engine with no Project.

2. WHAT A PROJECT IS
- A sixth family, `kind: Project`, instantiated from the core template `project`.
- Its Type is `core#project.v1`, which the engine serves (as it serves `management` and `access`). The Type offers one capability, `core#project-management.v1`.
- A Project has no placement, no runtime, no owner and no inputs in v0, and there are no user Project Types.
- Its states are registered, ready and retired (ruling 29's one lifecycle, now covering six families).
- Its one standard argument is `authority`: Conduits whose target is this Project, nested in it as a Pool's entries are nested in the Pool. They are registered with the Project and opened at its setup by the sender's right to open (ACC-005 pattern). This is the literal form of "projects determine an Interface's permissions": the project document says who acts on it, and each grant is still a Conduit.
- Project names are unique engine-wide. `engine` is reserved.

Alternative considered: a Resource of type `core#project.v1` owned by `system`, like `management`. It is cheaper (no change to OBJ-020, the families or the target kinds). But every bound naming `system` would then cover every Project, and a project is not "a shared thing". See open questions.

3. MEMBERSHIP: RECORDED ON THE MEMBER, NEVER AS A LIST ON THE PROJECT
- Each registered object stores its scope, fixed at registration (as POOL-001 fixes a Resource's owner). The scope comes from one new manifest key, `project: <name>`. Every document of that submission is homed in that Project. A manifest without the key registers engine-level objects.
- The Project's inventory is computed from its members. It is never a stored member list, which would drift and race (Kubernetes ApplySet replaced allowlist prune for this reason).
- "A project references every object under it" holds in two places: for the author (one concise file per project lists everything under `project:`) and in inspection (the Project's inventory).
- Challenge to the owner's wording: not every object can be a member. Interfaces, nodes, providers, packages and first-start objects stay engine-level. Shared Pools are referenced, not owned.

| Kind | Relation to projects |
|---|---|
| Agent | Exactly one scope (a Project or the engine), fixed. |
| Resource | The scope of its owning Pool. POOL-001 is unchanged. It has no `project` key. |
| Pool | One home (a Project or the engine), fixed, plus an `audience` of shares (section "sharing"). Shares are not membership. No Pool owns a Pool. |
| Standalone Conduit | Registered into the scope of its consumer or of its target (the manifest's), and named there. It appears in the inventories of both ends' scopes. Its scope gives it no authority. |
| Declared Conduit | Belongs to its consumer (CON-002), so it has the consumer's scope. A Project's `authority` Conduits belong to the Project. |
| Template | Homed where it was registered. `from` resolves among the document's own scope, engine-level templates (qualified) and package exports, never another project's templates. `replace.v1` on a project template needs that project's authority, so the cross-project replace.v1 hijack (synthesis E #4) is confined to one project. |
| Interface | Engine-only. It is a principal and never a member. Its authority over a project is exactly its Conduits to that Project or to that Project's members. If a project's admins could retire a person's Interface, "authority over several projects" would become ambiguous. |
| Nodes, machine and package-source providers, `system`, `management`, `access`, `root` | Engine-only (FST-001, PLC-008, PRV-013). |
| Runtime and secret-store providers | May be in any Pool. An instance may name, in `runtime` (or as a secret Resource's store), only a provider in its own scope or at engine level. Naming one grants use without a Conduit (PRV-004, PRV-006), so a project must not borrow another project's provider. |
| Packages | Installed engine-wide (`install.v1` on `management`). Pins stay per submission manifest (NAM-006, NAM-010). |
| Records | Engine-level. Each record also names the caller's scope and the target's scope (REC-002). |
| Pending actions | Engine state, keyed by the scopes their subject touches. |

4. NAMES
- Names are unique per (scope, kind). Two projects may each have `shared` and `worker`.
- An unqualified reference in a document resolves only in that document's scope. A reference into another scope carries `project: <name>`, or `project: engine` for an engine-level Pool, Resource, Agent or template.
- Interfaces and nodes exist only at engine level, so they are referenced unqualified, as `root` is today.
- In a request, an unqualified reference resolves in the scope of the Conduit the request travels through. A request through a Conduit to Project P resolves in P. A request through `management` or `access` resolves at engine level, and a project member must then be qualified.
- The engine keeps no "current project". There is no fallback and no "nearest wins". No name ever shadows another (GitHub's lowest-level-precedence pitfall).
- Tool names are unaffected: they come from the local name or from `as` (IFC-012, ruling 53).
- Fallback if this is too costly: keep names global (OBJ-002 as is) and have the CLI prefix them. That is cheaper, but it allows squatting across projects and puts mangled names into records and refusals.

5. LIFECYCLE
**Create.**
- `register.v1` of a Project is engine-level: it goes through `management`, so only root in v0. The Project may come alone or in the same submission as its members, which is atomic (SUB-003).
- `set-up.v1` makes it ready and opens its `authority` Conduits.
- `register.v1` accepts `expectedId` for the Project, so a client that pinned a Project's ID never writes into a recreated namesake (design D skeptic: the identity is a checked ID, never a name).

**Change.**
- Members come and go through ordinary requests routed to the Project.
- No object ever changes scope. Membership is immutable, so restructuring means new objects, as Kubernetes namespaces require.

**Retire.**
- `retire.v1` on a Project reports conflict while any member is live (the LIF-020 pattern).
- `retire-with-contents.v1` works in this order, narrowings first:
  1. close every Conduit that crosses the boundary, and end the project's shares out;
  2. retire the Agents;
  3. retire each Pool with its Resources, in LIF-021 order, going on past a failure (ruling 42, R5);
  4. unregister the project's templates;
  5. void its pending actions.
- Conduits to the Project close (CON-008).
- Pools shared in, and engine-homed Pools, are never touched (Compose `external`).
- It is a narrowing, so it never waits for approval. Its result lists every cross-project Conduit it closed.

**What survives.**
- The retired Project is kept, not removed under LIF-035, as the holder of its memory: tombstones, records until root purges them, and the applied baseline if that moves into the engine. Its name stays reserved until root explicitly unregisters it.
- A new Project with the same name gets a new ID. It inherits no authority, audience entry, approval or tombstone binding. The planner shows the old project's tombstones and requires `--recreate`.
- This answers the design D skeptic's finding that "losing the Pool erases the protections".

## sharing
ONE HOME, EXPLICIT AUDIENCE
A shared Pool is never multi-homed. Two homes would break ACC-033's single authorizer for the Pool's rows, would let two projects retire and close each other's things, and would break preservation test 3 ("owned once").

A Pool has one home, the engine or one Project, plus an `audience`. The audience is a list of entries, each with its own ID:
`{projects: [<names>] | all, future: true|false, resources: [<names>] (optional; omitted means every Resource of the Pool, including later ones), operations: [<qualified names>]}`.
`future` is required when `projects: all` and has no default (ruling 52). It says whether projects created later are included, which is RAM's `Principal: *` lesson.

**Who decides.** Only the home: the owner of the shared thing grants, never the consumer. Audience entries are:
- declared in the Pool document, as a new standard argument `audience`, and set up with the Pool;
- changed afterwards by `share.v1` and `unshare.v1`, new operations of `pool-management.v1`. These are authorized by a Conduit to the Pool, to the Pool's home Project, or to `management`, and both carry `expectedId`.

**What a share is.** It is the target half of a cross-project Conduit, given in advance as standing consent. The consumer half is the consumer project's own act: no member Conduit exists until an actor holding `open.v1` on the audience project opens one for its own member. So there is no separate "accept" step inside one engine. Across engines, ruling 51's two halves and an invitation with an expiry apply (v1).

A share gives the audience project's authority holders exactly two things:
- the right to open, for that project's members (Agents or Resources) as consumers, Conduits to the covered Resources that list only operations in the ceiling;
- inspection of the covered Resources' name, Type, description and state, so they can be wired.

**What an audience project may not do:**
- register into the Pool, set it up, retire it or its Resources, or inspect its other audience entries or other projects' Conduits or records to it;
- open a Conduit for a consumer outside itself;
- list an operation beyond the ceiling;
- share the Pool on. Shares are non-transitive: RAM's "you can't share a resource that's shared with you".

A bound on a Conduit to the audience project may name a shared-in Pool only within its ceiling.

**What is never shareable:**
- `system`, and any Pool that owns a node, `management`, `access` or a machine or package-source provider. This removes the FST-001 hazard from projects altogether.
- No ceiling may list `get-secret.v1` (SEC-008, SEC-009), an operation reserved to root (ACC-032) or a provider operation (PRV-004).

**Narrowing is immediate.**
- `unshare.v1`, or narrowing a ceiling, closes at once every Conduit from the dropped project's members to the covered Resources that lists an operation beyond what remains. Root's own Conduits are exempt (ACC-024).
- The result lists every Conduit closed, and tombstones are written (ruling 53).
- Order is respected (Zanzibar's "new enemy" problem): a Resource added after an unshare is never covered for the dropped project.

**Retirement on either side.**
- If the Pool or its home Project retires, CON-008 closes every Conduit to its Resources, other projects' included. There is no refusal, because narrowing never waits, but the plan and the result enumerate the cross-project Conduits that close.
- This is why a Pool meant for everyone should be engine-homed and retired only by engine-level authority, not homed in a project whose retirement would end it for all.
- If an audience project retires, its entries end, and its members' Conduits close as the members retire.

**Future coverage, made explicit.** Two cases widen without a new request: a Resource added to a shared Pool is covered at once for every audience entry with `resources` omitted (the ACC-029 precedent), and `all, future: true` covers projects created later. Both are stated in the entry. The result of creating a Project lists every share it falls into.

**The owner's two examples.**
- **Skills Pool for all projects.** Engine-homed (the `system` precedent: an engine-level owner, referenced by all), with audience `{projects: all, future: true, operations: [file.read.v1, directory.list.v1]}`. Writes stay with engine-level authority (root).
- **Auth Pool for one project.** Homed in that project, with no audience.
  - Challenge: ownership by a project does not let the project's admins wire secrets. SEC-008 keeps opening any Conduit that lists `get-secret.v1` root's, and the Linear MCP's declared Conduit to its key opens only when root sends the setup (ACC-005).
  - Whoever may register Agents and open Conduits in that project reaches everything in it (the weakest-create-right lesson). Recommended: put `auth` in a project whose only authority is root. Share the service Resource, for example `{projects: [packages], resources: [linear], operations: [<Linear list/get ops>]}`, and never the key.

**The hierarchical alternative, tested and rejected.** "Pools attached at a level are visible to projects below" is an implicit audience: `projects: descendants(F), future: true`, with no stated ceiling. It is worse than an audience in three ways:
1. The engine is the root of every tree and holds `system`. Position-based visibility would make `management`, `access`, nodes and providers visible to every project unless excepted, and one missing exception is near-root.
2. The home and the audience become the same thing, so a Pool cannot be homed with its maintainers while reaching projects elsewhere. Explicit shares are still needed across subtrees, so there would be two paths to the same access under different controls (Terraform's `tfe_outputs` pitfall).
3. No line in any plan shows that a newly created project can now reach the folder's Pools.

## authority
1. WHERE AUTHORITY IS INHERITED, AND WHERE NOT
| Edge | Carries authority? | Why |
|---|---|---|
| Engine (`management`, an unbounded `access`) → everything | Yes, for all management and open/close rows. Never for use (R16). | It already exists: root can do all of this today in two steps. Making it explicit removes root's own Pool Conduits, 40 of the coordinator's 332 S1 lines. It is the one GCP-organization edge kept. |
| Project → its members, and through its Pools → their Resources | Yes, for project-management.v1. Never for use (ACC-028). | The ownership path. It mirrors ACC-029's Pool → later Resources. |
| Pool → its Resources | Yes (today) | Unchanged |
| Project → Pools shared into it | No management. Only the right to open, within the ceiling, for its own members as consumers. | Authority follows home, never reference. |
| Folder → Projects | Does not exist | See section 7 |
| Membership, ownership | Never | Ruling 29, ACC-028 |

2. A PROJECT IS AN AUTHORIZING TARGET
`core#project-management.v1`, served by the engine, offers:
- register.v1, replace.v1, unregister.v1, register-replacement.v1;
- inspect.v1, set-up.v1, retire.v1, reconcile.v1;
- retire-with-contents.v1 (new);
- open.v1, close.v1;
- propose.v1 (new), approve.v1 (new).

Operations are reused contracts, as pool-management.v1 already reuses register.v1.

A Conduit to Project P is the RoleBinding. Its operation list is the role, and its optional bound narrows it. An Interface or Agent "with authority over several projects" holds one Conduit per project, and each project's authority makes and closes its own (Lighthouse pattern). Core ships Conduit templates `project-admin` (every operation except propose.v1), `project-operator` (no approve.v1, for Agents), `project-viewer` (inspect.v1) and `project-approver` (inspect.v1, approve.v1). Like `root-management`, these are templates, not aliases (ruling 25).

3. ROUTING (ACC-033 REWRITTEN BY SCOPE)
A request is permitted when an open Conduit from the actor to any one of the row's targets lists the operation. The authorizer recorded is the narrowest that permits. A refusal names every target tried (ruling 52, first tier).
- (a) register, replace, unregister and register-replacement of an object in P (a Resource is in P when its Pool's home is P): P, or `management`.
  - Registering into an engine-level live Pool: `management`. Today this needs `management` AND the Pool; the Pool half is dropped because `management` now covers Pool rows. This is a deliberate change.
- (b) Engine-level objects: Projects, Interfaces, and engine-level Agents, Pools, Conduits and templates. Also `install.v1`: `management`. Reserved operations: `management` and root only (ACC-032, unchanged).
- (c) set-up, retire, inspect and reconcile of P's Agents, Pools, Conduits and templates: P or `management`. Of the Resources of P's Pools: the Pool, P, or `management`.
- (d) Resources of engine-level Pools: the Pool or `management`.
- (e) retire-with-resources.v1 on a Pool: the Pool, its home P, or `management`. share.v1 and unshare.v1: the same targets.
- (f) Project P itself (inspect, retire, retire-with-contents): P or `management`.
- (g) propose.v1 and approve.v1: P (section "approvals"). Root may always approve.
- (h) open.v1 and close.v1: see section 4.
- (i) A control: the instance itself (unchanged).
- (j) Reconcile: the target that authorized the original request (rows 12 to 14 pattern, with P where P authorized).

4. OPENING AND CLOSING: TWO HALVES
Opening Conduit C (consumer c, target t, operations O) is dispatched only if both halves hold.

**TARGET HALF.** The actor holds one of:
- (i) a Conduit to `access` whose bound covers t and names O (today's ACC-006);
- (ii) a Conduit to t's home Project listing open.v1, with any bound on it covering t and O;
- (iii) where t's Pool has an audience entry for c's home project whose ceiling covers t and O: a Conduit to c's home project listing open.v1.

**CONSUMER HALF.** Required only when c is an Agent or a Resource homed in a project other than t's scope. The actor must also hold open.v1 on c's home project, or an unbounded `access`.

Engine-level consumers and Interface consumers need no consumer half. This keeps every existing document's meaning (ACC-030's "for any consumer" still holds where no project is involved).

The consumer half closes gap B4. A project admin of P2 may not give P1's Agent a write channel into P2, which would let P1's data be exfiltrated to P2's admin.

The root-only rules stay: ACC-032; SEC-008 and SEC-009 (only root, on its own behalf, opens a Conduit that lists get-secret.v1); PRV-004; CON-031. Closing follows ACC-012: any holder covering the target half may close. Conduits are never changed in place (ACC-011).

5. DELEGATION AND BOUNDS
- Opening a Conduit to Project P (handing out authority in P) is dispatched only if:
  - every operation it lists is one the actor's own Conduit to P lists;
  - its bound is within the actor's bound;
  - an unbounded one comes only from an unbounded holder or from engine level;
  - approve.v1 is listed only for an Interface consumer.
  This is ACC-030 applied to Projects: delegation only narrows (test 5).
- CON-028 now allows bound.v1 on Conduits to `access` or to a Project.
- On a Project Conduit, the bound's `pools` and `targets` must be P's members or shared-in Pools within their ceilings. `system` can never be named, so the near-root hazard of FST-001 cannot arise through a Project.
- bound.v1 gains a fourth list, `templates`. It does two things:
  - it limits register and register-replacement to instances `from` those templates;
  - it covers, as targets, every instance made from them, present and future, as `pools` covers future Resources.
- `templates` fixes B3 (bounds frozen to IDs cannot cover a future Agent) and replaces the broker for "register workers of one approved template" (ACC-030's broker paragraph stays valid but is no longer the only way).
  - Example: coordinator → Project packages, listing [register, set-up, retire, inspect, open, close, reconcile], bound {templates: [pkg-worker], pools: [pkg-work, engine:skills, conduit-core:repo], operations: [file.*, turn.v1, …]}.
  - Config is not a loophole. A template fixes the inputs it does not declare, so a worker's `accountServices` cannot be set unless `pkg-worker` exposes it as an input. The existing paradigm, the template, is the approved shape.

6. ROOT, `management` AND A SECOND PERSON
**Root.**
- Root's two first-start Conduits are unchanged.
- `management` now covers every project row and every Pool row (engine as the one parent).
- Root's identity powers stay root's: ACC-032 (declare-lost, purge), SEC-008 (secret Conduits), ACC-024 (its Conduits never closed), LIF-022, and in v0 the routing of sign-ins and enrollment (rulings 49 and 50).
- Project admins cannot touch root's authority, because it is not a Conduit to their Project.

**A second person.**
- Root registers an Interface `collab` (engine-level, through `management`, ACC-023: no access yet).
- Root then opens collab → Project packages from `project-admin`, either declared in the Project's `authority` or standalone.
- The build must also issue collab a token (DL13). Today only root and run instances get one, so this is a shell work item.

**Overlap and revocation.**
- Every holder in P covers every Conduit in P and can close it (ACC-012). This is accepted inside one project.
- Revocation does not cascade (ACC-030). New: each Conduit stores `openedBy`, the actor whose request opened it. The inventory shows it, so "close everything collab opened in packages" is one explicit request the planner can compose.

7. THE HIERARCHY VARIANT, TESTED: THE ESCALATION RISKS INHERITANCE BRINGS
Variant H: Folders nest up to 4 deep under the engine. A Conduit to folder F covers every object in F's subtree, present and future (GCP union allow). Pools attached at F are visible below. Approvers declared at F apply below. move.v1 needs rights on both parents.

Risks, in order of severity:
- E0. The one edge this design keeps is already the largest: any holder of `management` or an unbounded `access` administers every project, present and future. It is kept only because it already exists and is root's; giving either to anyone but a deliberate co-root is warned against.
- E1. Union grants a child cannot refuse.
  - There is no deny, because ruling 41 removed policies, so a folder grant applies in every descendant. The project can neither see it among its Conduits nor close it. That falsifies "projects determine an Interface's permissions".
  - GCP needed deny policies, org-policy constraints and PAM to compensate. Each is a rule mechanism outside Conduits.
- E2. Compounded future coverage: future projects × their future Pools × their future Agents and Resources, with no plan line for any of them (ruling 52). ACC-029 accepts this for one Pool's Resources only.
- E3. Position-based visibility includes the engine level (`system`: `management`, `access`, nodes, providers). One missed exception gives every project near-root reach.
- E4. The weakest create right, multiplied. A folder admin who may create projects and Agents reaches, through Agents it makes, every Pool attached at F and above. An auth Pool attached at a folder is reachable by every descendant registrant.
- E5. Approver union. A project cannot narrow its approvers, so one compromised folder approver approves widenings everywhere below.
- E6. Moves are a widening and a narrowing at once. They need rights on both parents and an approval in the destination, and they close Conduits into the old parent's Pools. They would be the language's first in-place re-homing (against POOL-001 and PLC-009).
- E7. Structural overlap. Every ancestor's holders cover every descendant's Conduits (ACC-012), so revocation contests are built in.
- E8. Orphans scale with the subtree. Removing a folder admin leaves everything it opened across N projects live.
- E9. Delegation depth. "Within" becomes subtree containment over IDs frozen at instantiation (OBJ-021). A re-parent or a containment slip escalates across a whole subtree.
- E10. Explanation and disclosure. Every decision, the second refusal tier (ruling 52), the inventory (ruling 53) and pending-action routing must compute effective authority over the ancestry. "Who can reach X" is no longer readable from X's Conduits and its home.
- E11. Shadowing pressure. Nesting invites nearest-wins resolution of names and settings, which Conduit must refuse; most of nesting's convenience goes with it.
- E12. Blast radius. Retiring a folder retires every project below (as deleting an Azure resource group deletes its contents), and cross-tree shares die at once.

**Verdict.** Inheritance does not earn this in v0. The owner's real use is one person, 2 to 10 projects and maybe one collaborator. Flat projects cost one explicit, approved line per extra project per cross-cutting grant, and those lines are exactly what ruling 52 wants seen.

**What would make one level of grouping safe in v1** (a "project set"):
- no nesting;
- a project joins only by a two-sided widening (the set's authority, and the project's approver), and its evidence lists every Conduit and share the join confers;
- leaving is an immediate narrowing that either side may make;
- set shares carry ceilings;
- approvers are never inherited.
This also gives delegated project creation (register.v1 of Projects through a Conduit to a set, the creator consenting on both sides). Nothing in v0 precludes it: it adds a target kind and a `sets` entry to audiences and bounds.

## approvals
PRINCIPLE: AUTHORITY, NOT A CLASSIFIER, DECIDES WHAT WAITS
An actor acts at once within its Conduits. Within its envelope a request is pre-approved by whoever opened that envelope, who could only give what it held (ACC-030). Beyond its envelope, an actor may only propose.

So the engine never has to decide in general whether a change "widens" (gap G7, F9), and no Type has to mark widening settings. The cost is stated honestly: a Conduit that lists a control or replace.v1 is a standing approval of whatever those can do. ACC-030 already says this for replace.v1.

Three rights, each declared as operations on a Conduit to Project P, so each is inspectable, revocable and narrowed only by delegation:
- **act:** the operations listed, such as open.v1 or register.v1;
- **propose:** propose.v1;
- **approve:** approve.v1, listed only on a Conduit whose consumer is an Interface.

"Who may approve is itself declared" is therefore literal: it is the set of Interfaces holding approve.v1 on P, plus root always. The owner's sketch of a granular Interface is expressible this way, for example a phone control panel holding only inspect.v1 and approve.v1 on one project.

NARROWING NEVER WAITS
- An actor that may close, retire, unregister, unshare or retire-with-contents does so at once.
- A narrowing is never bundled into a proposal: propose.v1 refuses a subject that contains a narrowing the proposer could make itself. A pending widening never holds a narrowing back. (In T4, B's narrowing applies at once while A's widening goes ahead.)
- Recommended small addition: any consumer other than root may close a Conduit it consumes (relinquish), so an Agent can always narrow itself.

THE FLOW (one generic pending action, rulings 50 and 51, decision 5)

1. **Propose.** `P.propose.v1` takes:
   - `subject`: the exact requests, each with its operation, its target reference with `expectedId`, and the digests of the documents it would register;
   - `justification`: shown only as a quote;
   - optionally, an `expiry` shorter than the maximum.

   The engine refuses the proposal (not dispatched) when:
   - the proposer could make every request in it itself (it should just act);
   - it contains a narrowing the proposer could make;
   - it names an object in a scope the caller may not inspect. The answer then is the first-tier refusal only, so a proposal cannot be used to probe whether objects exist.

   Otherwise the engine records the pending action:
   - **kind:** approval;
   - **scopes:** every scope whose half the subject needs;
   - **proposer:** with its caller chain;
   - **subject**, with its digest;
   - **evidence**, written by the engine: reach before and after for each consumer, and which half each scope must supply;
   - **expiry**, and **outcome:** pending.

   The propose.v1 invocation ends at once (ACC-019, ruling 38: no work outlasts an invocation).

2. **Route.** The action is raised at every Interface holding approve.v1 on each scope in `scopes`, and at root always, through Interface notifications or elicitation. It is never raised at an Agent. It is also visible in the Project's inventory to callers who may inspect it. It does not disclose objects that a given approver may not inspect. An approver of one half sees the other half only as "needs approval in a scope you cannot inspect" (ruling 52).

3. **Approve.** An approver calls `P.approve.v1` with the action's ID as `expectedId` and a decision, approve or decline.
   - The engine re-decides every request of the subject anew (ACC-036) under the approvers' own authority at that moment. Each half must be covered by the authority of an approver of that half's scope. Root covers all halves alone.
   - If any `expectedId` in the subject no longer matches, the outcome is conflict and nothing is dispatched (drift aborts; critic gap 1).
   - The requests are dispatched in order, stopping at the first that does not succeed, with no rollback, as apply works.
   - Records attribute the proposer and every approver (test 7).
   - An approver can never approve beyond what it could do itself, because the change is decided under its authority (Kubernetes' bind/escalate rule).

4. **No self-approval.** The proposer never approves its own action, and neither does an Interface whose approve.v1 was opened by the proposer. For an Interface proposer such as ci-hook this rule is explicit.

5. **Expiry and voiding.**
   - The engine default is 24 hours, and the maximum, 7 days, is stated per engine (Kubernetes CSR; RAM invitations).
   - An action is voided when its proposer, any subject object or any of its scopes is retired.
   - Expired and voided actions are kept as records.

6. **Cross-project.** Approvals of both halves may come from two Interfaces, for example collab for packages and root for conduit-core. The action executes when the last required half is approved and both are still valid.

SIGN-INS AND ENROLLMENT
- These are the same pending-action mechanism with kind sign-in or enrollment.
- In v0 they are routed to root alone, as rulings 49 and 50 rule. Nodes are engine-level, so enrollment stays root's.
- In v1, a sign-in for a declared account of an Agent in P could also be routed to P's approvers who are persons (ruling 49's "others authenticate"). The routing key already exists because every Agent records its scope.

SECRETS
SEC-008 is unchanged: no project approver can approve a get-secret Conduit, because none holds the authority to open one. Changing that would change the secret guarantee itself, and it is left to the owner.

WHY NOT GATE ALL WIDENINGS BY AUTHORITY HOLDERS
"Every widening waits for an approver, even when the actor holds the authority" is expressible without a new mechanism: list propose.v1 and close.v1 but not open.v1. That holder narrows at once and can only propose widenings. A blanket gate would make every coordinator dispatch in UC9 wait for a person, and would add a y/N step to root's own S1 work (the critic's decision 2).

## examples
Notation: `v0` = `apiVersion: github.com/shrav-k/conduit/v0`; `core#` = `github.com/shrav-k/conduit/core#`. Concise forms follow the simplification's project file (synthesis item 1). Their syntax belongs to the parallel short-form investigation; only the information content is fixed here.

A. S1 AS ONE PROJECT

Concise (the package's T1, 15 file lines, unchanged):
```yaml
conduit: v0
project: s1                    # now names a real Project (engine-held), not only a CLI label
resources:
  shared: {type: example.org/s1/filesystem#local-directory.v1, config: {root: <lab>/shared}}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach:
      project: {to: shared, ops: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
  agent-b: ...                 # as the package, + file.write.v1
```
Plan (abridged), operator root:
```
+ Project s1                   registered through management; no authority entries: root covers s1 from the engine level
+ Pool s1/workspace            derived default Pool, home s1
+ Resource s1/shared, Agents s1/agent-a, s1/agent-b
+ Conduit agent-a--turn, agent-b--turn   root -> Agent [turn.v1]   derived: authority over a project never permits use (ACC-028)
+ Conduit agent-a--project as project ... ; agent-b--project ...
  (no s1--admin: root's `management` covers s1 and its Pools, ACC-033 rows a and c)
```

Canonical (T1 excerpt):
```yaml
# conduit.yaml
v0
project: s1                                  # NEW manifest key (MAN-004 + MAN-0xx)
packages: {...}
---
v0
kind: Project                                # NEW family
metadata: {name: s1}
from: core#project
---
v0
kind: Pool
metadata: {name: workspace}
from: workspace-pool
---
v0
kind: Resource
metadata: {name: shared}
from: directory
owner: workspace
arguments: {root: <lab>/shared}
# Agents, the two grant Conduits and the two turn Conduits as today.
# GONE: template pool-admin and Conduit root-workspace (root -> Pool workspace).
```
This removes, in canonical form and with no CLI default, the first mistake both S1 authors made: "no root management route to owning Pool" (map-attribution, row 5; coordinator mistake 1).

B. THE OWNER'S REAL USE: TWO PROJECTS, A SHARED SKILLS POOL, A PROJECT-ONLY AUTH POOL

Engine-level file (no `project:` line):
```yaml
conduit: v0
interfaces:
  collab: {type: core#interface.v1}            # a second person, engine-level
pools:
  skills:                                      # home: engine
    audience:
      - {projects: all, future: true, ops: [example.org/fs#file.read.v1, example.org/fs#directory.list.v1]}
    resources:
      claude-skills: {type: example.org/fs#local-directory.v1, config: {root: ~/.claude/skills}}
      codex-skills:  {type: example.org/fs#local-directory.v1, config: {root: ~/.codex/skills}}
```
Canonical for the shared Pool:
```yaml
v0
kind: Pool
metadata: {name: skills}
from: skills-pool                              # engine-level template
audience:                                      # NEW standard argument of a Pool (POOL-020)
  - projects: all
    future: true                               # required with `all`; no default
    operations: [example.org/fs#file.read.v1, example.org/fs#directory.list.v1]
```

conduit-core file:
```yaml
conduit: v0
project: conduit-core
pools:
  auth:                                        # no audience: project-only
    resources:
      onepassword: {type: example.org/1password#store.v1, config: {vault: conduit-test}}   # secret-store provider
      linear-key:  {type: example.org/1password#secret.v1, store: onepassword, config: {item: linear-api}}
      linear:      {type: example.org/linear#mcp.v1, secrets: {token: linear-key}}
      # linear's declared Conduit to linear-key lists get-secret.v1; it opens only when root sends the setup (SEC-008, ACC-005)
  repo:
    audience:
      - {projects: [packages], resources: [main], ops: [file.read.v1, directory.list.v1]}
      - {projects: [packages], resources: [queue], ops: [queue.post.v1, queue.read.v1]}
    resources: {main: {...}, queue: {...}}
agents:
  coordinator:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach:
      skills: {to: engine:claude-skills, ops: [file.read.v1, directory.list.v1]}   # target half: skills' audience; consumer half: conduit-core (root)
      linear: {to: linear, ops: [linear.issues.list.v1, linear.issues.create.v1]}
    authority:
      packages: {template: project-operator, bound: {templates: [pkg-worker], pools: [pkg-work, engine:skills, conduit-core:repo]}}
      # a Conduit coordinator -> Project packages: target half from packages' authority, consumer half from conduit-core's; root holds both
```
Canonical cross-scope references (from conduit-core documents):
```yaml
target: {kind: Resource, name: claude-skills, project: engine}
target: {kind: Project, name: packages}
```

packages file:
```yaml
conduit: v0
project: packages
authority:
  collab: project-admin                        # compiles to the Project's `authority` entry; collab can approve
pools:
  pkg-work: {}
templates:
  pkg-worker: {type: example.org/conduit/agent-codex#codex.v1}   # no accountServices input: workers cannot widen by config
```
Canonical for the Project:
```yaml
v0
kind: Project
metadata: {name: packages}
from: core#project
authority:                                     # NEW: Conduits to this Project, nested in it
  collab:
    from: core#project-admin
    consumer: {kind: Interface, name: collab}
```

What each party can do:
- The coordinator registers `pkg-worker` instances in packages, gives each its own tree and read-only access to main, skills and the queue, takes turns, and retires them, all without approval.
- Workers reach only their own Conduits. No packages member can reach auth.
- Only root wires linear-key.

C. A SECOND INTERFACE LIMITED TO ONE PROJECT
`collab`, holding only the `project-admin` Conduit to packages, together with a narrower outside system:
```yaml
v0
kind: Conduit
metadata: {name: ci-hook-queue}
from: queue-post                               # registered into packages by collab (target in packages)
consumer: {kind: Interface, name: ci-hook}
target: {kind: Resource, name: queue, project: conduit-core}
# Refused: queue is homed in conduit-core, and packages' audience entry lets
# packages' admins open Conduits only for packages' own members.
# Root must open it, or conduit-core must grant ci-hook directly.
```

Collab may:
- register, set up, retire and inspect packages members;
- open and close Conduits for packages members, to packages members, and to skills or repo within their ceilings;
- delegate a subset of its operations (e.g. `project-viewer` to ci-hook);
- close the coordinator's Conduit to packages;
- approve packages' halves of pending actions;
- see the packages inventory, filtered, with no counts.

Refused:
- any request in conduit-core, or on auth (first-tier refusal only, which does not disclose existence);
- any write to skills or repo;
- widening its own Conduit, or giving ci-hook more than it holds;
- any get-secret Conduit;
- declare-lost and purge;
- retiring skills or repo;
- approving alone a change whose target half is in conduit-core;
- seeing any conduit-core sign-in.

D. AN AGENT PROPOSING A CHANGE
pkg-lead (Agent, home packages) holds `pkg-lead -> Project packages` listing [inspect.v1, close.v1, retire.v1, propose.v1]. It may narrow at once and propose widenings. It calls `packages.propose.v1`:
```json
{"subject": [
  {"operation": "core#register.v1", "documents": ["sha256:9f1c… (Conduit docker-linear: consumer {kind: Agent, name: pkg-worker-docker}, target {kind: Resource, name: linear, project: conduit-core}, ops [linear.issues.list.v1], as: linear)"]},
  {"operation": "core#open.v1", "conduit": {"kind": "Conduit", "name": "docker-linear"}}],
 "justification": "release notes need issue titles"}
```
The engine records:
```yaml
id: pa-0193
kind: approval
scopes: [packages, conduit-core]
proposer: {kind: Agent, name: pkg-lead, project: packages}   # plus caller chain
subject: sha256:…
evidence:
  pkg-worker-docker: {before: [tree rw, main r, queue post/read, skills r], after: [+ linear linear.issues.list.v1 (conduit-core/auth; not shared to packages)]}
  halves: {consumer: packages (approve.v1: collab, root), target: conduit-core (approve.v1: root)}
expiry: 2026-11-11T10:00Z
outcome: pending
```
- collab approves → the packages half is recorded. root approves → the engine re-decides: the register through packages under collab's authority, the open with its target half under root's and its consumer half under collab's. The outcome is approved, and the records name pkg-lead, collab and root.
- If collab alone approves and the action expires → outcome expired, nothing dispatched.
- If pkg-worker-docker was replaced meanwhile → conflict.
- Root's better answer: decline, and instead add an audience entry `{projects: [packages], resources: [linear], ops: [linear.issues.list.v1, linear.issues.get.v1]}`, which is root's own immediate act. collab can then open the Conduit itself.

E. S1 LINE COUNTS PER TASK (paper counts by the measurement plan's rule; ruling 45 accepts only a run)
| | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
|---|---|---|---|---|---|---|---|---|
| Baseline A70 | 33 | 16 | 0 | 8 | 3 | 2 | 2 | 64 |
| Package (synthesis) | 16 | 6 | 0 | 2 | 1 | 1 | 1 | 27 |
| This design, concise | 16 | 6 | 0 | 2 | 1 | 1 | 1 | 27 |
| Hierarchy variant H, concise | 16 | 6 | 0 | 2 | 1 | 1 | 1 | 27 |

Why no line is added:
- The package already counted `project: s1`.
- Root is engine-level, so its widenings in T4 and T7 act at once and raise nothing.
- Equal re-points in T5 and T6 are ordinary close and open by root.
- Tombstones are keyed by (s1's ID, consumer, local name).
- The design removes one derived object (`s1--admin`) and adds a real one (Project s1), so the plan shows the same number of things.

Canonical delta against the coordinator's run:
- −40 lines of root-to-Pool route (T1 −22, T2 −18; map-attribution category 4);
- +4 for the Project document (T1);
- +1 `project:` manifest line per submission.
That gives about T1 137 and T2 56, roughly 300 in all against 332.

H adds nothing to S1 (default parent: the engine) except concepts for a fresh author: folder, parent, inherited authority, inherited visibility.

F. FLAT AGAINST NESTED ON LARGER SHAPES (paper)
| Shape | Flat (this design) | Nested (H) | Verdict |
|---|---|---|---|
| Owner today: conduit-core, packages, skills, auth | 0 lines beyond the files above | +1 folder line | Flat |
| 8 package projects, collab admin of all, repo shared to all 8 | 8 authority lines and 1 audience line listing 8. A new project costs +2 lines, each a recorded widening | About 3 lines. A new project costs +1 line `parent:`, and collab and the audience widen silently | H saves about 10 lines and loses explicitness (E1, E2) |
| Two teams on one engine, each creating its own projects | Every new project needs root in v0 | A folder admin creates projects | H wins; answered by separate engines (ruling 51) or v1 project sets |
| Retire a team | Per project, enumerated | Folder cascade across N projects (E12) | Flat |

## spec_plan
Sequence: write this after the short-form ruling (ruling 53, decision 1) and together with ruling 53's five additions, so that each of the five is written once, per scope.

NEW PROMISES (about 28)

projects.yaml (PRJ):
- PRJ-001 What a Project is: kind, core template `project`, Type core#project.v1 served by the engine, project-management.v1, engine-unique names, `engine` reserved.
- PRJ-002 Scope: every Agent, Pool, standalone Conduit and template has exactly one scope, from its submission's manifest `project` key or the engine, fixed for life. A Resource takes its Pool's scope; a declared Conduit its consumer's; a Project's `authority` Conduits belong to the Project.
- PRJ-003 Engine-only kinds and objects: Interfaces, nodes, Projects, first-start objects, packages, records. A manifest naming a project and holding such a document (other than its own Project document) is rejected.
- PRJ-004 Scope permits nothing (extends ACC-028).
- PRJ-005 One submission registers into one scope. A Project and its members may come in one atomic submission (SUB-003).
- PRJ-006 `authority` entries are registered with the Project and opened at its setup by the sender's right to open (ACC-005 pattern).
- PRJ-007 retire.v1 of a Project reports conflict while any member is live (LIF-020 pattern).
- PRJ-008 retire-with-contents.v1: its order and its going on past a failure (LIF-021 and R5 pattern).
- PRJ-009 A retired Project is kept with its name, tombstones and memory until root unregisters it (an exception to LIF-035).
- PRJ-010 Inspecting a Project returns its inventory: members, authority Conduits with `openedBy`, shares out and in, boundary Conduits, pending actions and tombstones. Only what the caller may inspect, with no counts.
- PRJ-011 Delegation in a Project, as in ACC-030: operations within the actor's; bound within the actor's; approve.v1 only for Interface consumers.
- PRJ-012 `expectedId` on the Project in register.v1.

pools.yaml:
- POOL-020 `audience`: its entries, its future flag, declaration in the document, share.v1 and unshare.v1.
- POOL-021 What is never shareable, and what a ceiling may never list.
- POOL-022 A share is the target half only and permits nothing. Shares are non-transitive.
- POOL-023 Unsharing, or narrowing a ceiling, closes the dependent Conduits at once and lists them.
- POOL-024 A Pool's home is fixed, and a member registered into a Pool must share its scope.

pending.yaml (PND), absorbing rulings 49 to 51:
- PND-001 Shape: kind, scopes, proposer, subject, evidence, expiry, outcome.
- PND-002 propose.v1 and when it is refused.
- PND-003 Routing to approve.v1 Interfaces and to root, never to Agents, with disclosure limits.
- PND-004 approve.v1: re-decided under the approvers' authority, halves, drift gives conflict, dispatched in order, recorded.
- PND-005 No self-approval.
- PND-006 Expiry and voiding.
- PND-007 A narrowing is never held or bundled.
- PND-008 Sign-ins and enrollment go to root alone in v0.

Elsewhere:
- MAN-0xx The `project` key.
- ACC-0xx The consumer half.
- ACC-0xx Relinquish (optional).
- CON-0xx `openedBy` stored and inspected.

CHANGED PROMISES (about 38)
- **Objects.**
  - OBJ-002: unique per (scope, kind).
  - OBJ-006, OBJ-021: references gain `project`.
  - OBJ-013: Pool `audience`, Project `authority`.
  - OBJ-020: eleven kinds, six families.
  - OBJ-025: the conflict names the scope.
- **Naming.**
  - NAM-003: unqualified references resolve in their own scope.
  - NAM-022, NAM-027: "project" becomes "submission".
- **Submissions and manifest.** SUB-005 and definitions.md (the term), SUB-008 (the address is not an identity), MAN-004 (adds `project`).
- **Instantiation.** INS-025 (`from` within scope), INS-020 and LIF-009 (a replacement uses the template in its own scope).
- **Lifecycle.** LIF-013 (inventory), LIF-031 (references in a request resolve in the scope of the Conduit used), LIF-035 (the Project exception).
- **Conduits.**
  - CON-008: also closes on unshare.
  - CON-012: registered into the consumer's or the target's scope.
  - CON-028: bound.v1 on Conduits to `access` or to a Project.
- **Access.**
  - ACC-005, ACC-006, ACC-012: the two halves.
  - ACC-022: root unlimited, Projects included.
  - ACC-028: wording.
  - ACC-029: bound semantics on a Project, plus `templates`.
  - ACC-030: delegation through Projects; "for any consumer" kept only where no project is involved.
  - ACC-033: rewritten by scope. "Any one of the row's targets", the narrowest recorded; `management` covers every Pool row; row 1's double requirement removed.
- **Pools.** POOL-004 (registration into a Pool of another scope is refused), POOL-015.
- **Providers and secrets.**
  - PRV-006, and the store reference of secret Resources: providers only in the instance's own scope or at engine level.
  - SEC-008: unchanged in substance, with a note that no project authority can open get-secret.
- **Records, operations and Interfaces.**
  - REC-002: records carry scopes.
  - OPS-023: a refusal names the targets tried.
  - IFC-007 and IFC-016: the elicitations of ruling 49, delivered as pending actions.
  - IFC-033: a tool's description names the target's project when it differs from the consumer's.
- **First start and contracts.**
  - FST-003: "or Project".
  - FST-005: core also exports the templates `project`, `project-admin`, `project-operator`, `project-viewer` and `project-approver`.
  - CTR-004: pool-management.v1 now holds share.v1 and unshare.v1, served by the engine.

REMOVED
None. ACC-033's row 1 requirement for both `management` and the Pool, and ACC-030's broker as the only narrowing of `management`, are superseded in their text rather than removed as promises.

FIRST START
The five objects and root's two Conduits are unchanged. No Project exists at first start, and there is no default Project. `management` widens in meaning (it covers every project row and every Pool row), so root needs no Conduit to any Pool or Project. Engines that never use projects behave exactly as today, apart from that widening of `management`.

RULING 53's FIVE ADDITIONS, PER SCOPE
- **`as`:** unchanged. It is a local name per consumer, and tool names never include a project.
- **`expectedId` on open and close:** also on approve.v1 (the action's ID), share.v1 and unshare.v1, inside a propose.v1 subject, and on the Project in register.v1.
- **Inventory in inspect.v1:**
  - every object reports its scope;
  - inspecting a Project lists its members, boundary Conduits, audiences, pending actions and tombstones;
  - an object's inventory shows Conduits from other scopes only where the caller may inspect those scopes;
  - no counts. Scoped admins finally see their own project and not the engine (B7).
- **Tombstones:**
  - keyed by (scope ID, kind, name) for instances and (scope ID, consumer, local name) for Conduits;
  - enforced at open and register within the scope;
  - kept by the retired Project (PRJ-009);
  - a recreated name has a new ID, and the planner shows the old tombstones.
- **Tool descriptions:** name the target, its Type's description and, when it is cross-scope, its project (ruling 52).

SCHEMAS
- manifest (+`project`);
- the reference schemas (+`project`);
- a Project instance schema (`authority` map);
- an audience entry schema;
- bound.v1 (+`templates`);
- a pending-action schema;
- inputs and outputs of propose.v1, approve.v1, share.v1, unshare.v1 and retire-with-contents.v1;
- the inspect.v1 output (scope, inventory, audience, `openedBy`, pending actions, tombstones);
- record.v1 extension data (scopes).

CORE OPERATIONS AND CAPABILITIES
- **New capability:** project-management.v1 = register, replace, unregister, register-replacement, inspect, set-up, retire, retire-with-contents (new), reconcile, open, close, propose (new), approve (new).
- **pool-management.v1:** adds share.v1 and unshare.v1.
- **Unchanged:** object-management.v1 (its reach widens through ACC-033) and access-control.v1.

## cost
Paper estimates. No experiment was run.

**Specification.** About 28 new and 38 changed promises, roughly 66 of about 460 (about 14%). That is a major change, in the range of the simplification's designs B (about 60) and D (about 50). It cannot go through as a quiet batch: it needs rulings on the vocabulary (ruling 42), on a sixth family (ruling 29's lifecycle), on ACC-033 and on ACC-030.

**Conformance.**
- ACC-033 is cited by 78 conformance files (map-mechanisms' figure). Their meaning for engine-level objects is unchanged, but their expected refusal wording changes.
- About 5 to 15 vectors invert. The clearest case is ACC-033's observable "registering a Resource into a live Pool through `management` without a Pool Conduit is refused", which now succeeds.
- About 70 to 100 new vectors: scope and naming, the two halves, audiences and their cascades, the `templates` bound, delegation in a Project, every PND rule, retiring a Project, and disclosure in inventories and refusals.

**Reference model and build (Rust).**
- Core: about 3,000 to 4,500 source lines plus about 3,000 test lines. That covers scope stored on every object, a per-scope name index, routing by scope in `access.rs`, the two halves, audiences with their close cascades, bound `templates`, pending-action state and its decisions, `openedBy`, and filtered inventories.
- Shell: about 800 to 1,500 lines. Pending actions reach Interfaces as elicitations or tools. A token can be issued to a second Interface (DL13; impossible in the first build today).
- CLI: about 400 to 800 lines. The concise `project:`, `authority:` and `audience:` keys, `conduit approve`, and project-aware plans.
- About 8 to 10 work orders after the short-form decision.

**Author-facing.**
- S1 concise: unchanged at 27 lines (against 64 for the baseline).
- S1 canonical: about −30 lines net. It also removes the "no route to owning Pool" mistake both authors made.
- A fresh author doing S1 meets one new concept (Project) and loses one (root's Pool Conduit).
- An author using sharing or approvals meets about four: scope, audience, propose and approve.

**What the hierarchy variant would add on top.**
- About 20 to 30 further promises: Folder or `parent`, subtree cover in routing, bounds and audiences, approver union, moves or their refusal, path-qualified names, folder retirement.
- About 1,500 to 2,500 more core lines and 40 to 60 more vectors, one per escalation case E1 to E12.
- A permanent cost in every decision, refusal and inventory, all of which must walk the ancestry.
- For S1 and the owner's use today, it saves nothing.

## what_is_lost
What the recommended flat design loses, honestly:
1. **No zero-line authority over a growing set of projects.** An admin of N projects holds N Conduits, and every new project needs its grants and audience entries written. Each is a recorded widening, which ruling 52 wants seen but which costs lines and approvals at scale. At 8 package projects that is about 10 lines more than a folder would need.
2. **No delegated project creation in v0.** Registering a Project needs `management`, so root creates every project. A team lead cannot create projects for their team until v1 project sets, or until the team gets an engine of its own.
3. **No moves.** An Agent, Pool or template never changes project. Restructuring means new objects: an Agent's session is lost and its tombstones stay in the old scope.
4. **`management` becomes the whole engine admin.**
   - It now covers every Pool row, so ACC-033 row 1's Pool consent for `management` holders is gone. A co-admin given `management` can set up and retire every Resource, though still not use it or open Conduits without `access`.
   - This is deliberate (the engine is the one parent), but it widens a first-start object's meaning.
5. **ACC-033's single-authorizer observable becomes "any one of the row's targets, the narrowest recorded".** Refusals must name several targets.
6. **Per-scope names complicate references.**
   - A cross-scope reference must carry `project:`, and `engine` is reserved.
   - LIF-031's resolution by the scope of the Conduit used is a new idea that authors and Agents must learn.
   - Unqualified references to engine-level Pools from a project no longer work. The skills Pool must be written `engine:` in every project file.
7. **Test 5 narrows.** If approve.v1 is Interface-only, an Agent can act and propose but never approve. That is the owner's "through the Interface", but it is a deliberate exception to "every management action is available to an agent under a grant".
8. **Authority, not a classifier, decides what waits.** A Conduit that lists a control or replace.v1 pre-approves whatever those can do. Authors must keep widening settings out of a template's inputs; the engine will not catch a widening made by config.
9. **The two halves remove a freedom ACC-030 gave today.** A bounded right to open no longer works "for any consumer" once consumers live in other projects.
10. **Still root's in v0:**
    - secret wiring (SEC-008), so an "auth pool for a project" still needs root for every secret-reading instance and every replacement of one;
    - sign-in and enrollment delivery (rulings 49 and 50), so the owner's granular Interfaces do not yet see their projects' sign-ins;
    - placement: nodes and runtime choice are not project-scoped, so any project's instance may land on any node (UC5 unmet).
11. **Interfaces cannot be created by a project's admins.** A ci-hook for one project needs root.
12. **From the hierarchy, deliberately given up:**
    - per-subtree administration;
    - attaching a Pool once for a whole branch;
    - folder-level approvers;
    - a natural home for a team boundary inside one engine. The design sends that to separate engines (ruling 51 peers), which costs a second engine and two-halves relationships between them.

## open_questions
For the owner:
1. **Do you foresee several teams with separate admins sharing ONE engine?** If not, flat Projects suffice, and separate trust means separate engines (ruling 51 peers). If yes, plan v1 "project sets": no nesting, a join is a two-sided approved widening whose evidence lists what it confers, leaving is immediate, approvers are never inherited. Recommended: flat now, and decide on sets when a second team exists.
2. **Vocabulary (ruling 42, one meaning per word).** Keep "Project" for the registered scope, the owner's word as in GCP and HCP, and rename today's term to "submission" (the documents and manifest handed over in one registration; NAM-022 and NAM-027 follow)? Or give the new object another name ("workspace", "space")? Recommended: Project, and rename the old term.
3. **The Project's form.** A sixth family (recommended: clearer definitions and lifecycle), or a Resource of type `core#project.v1` owned by `system`, as `management` is? The second is about 5 to 8 promises cheaper, but every bound that names `system` would then cover every Project.
4. **Names.** Unique per (scope, kind) with qualified cross-scope references (recommended)? Or kept global (OBJ-002 unchanged), with CLI prefixes, which is cheaper but allows squatting across projects and mangles names in records?
5. **Should `management` cover every Pool row** (the engine as the one parent, removing root's Pool Conduits)? Or should Pool consent stay, with root holding a derived Conduit to each Pool?
6. **May an Agent ever hold approve.v1?** Test 5 pulls one way; "handled through the Interface" and ruling 49 pull the other. Recommended: Interfaces only.
7. **Approval quorum and expiry.** One approver per half (recommended)? Default 24 hours and maximum 7 days per engine (recommended)? Should a project be able to require two persons?
8. **"All projects" audiences.** `future` is required, with no default (recommended)? Should creating a project list every share it falls into and require acknowledgement?
9. **Retiring a Pool or project that others use through shares.** Proceed and enumerate (recommended: narrowing never waits)? Or refuse until unshared, as RAM advises?
10. **Small additions.** Self-relinquish (a consumer may close its own Conduit, root excepted; recommended)? `openedBy` stored on every Conduit (recommended)? An explicit "close everything X opened in P" request, rather than cascading revocation?
11. **Interfaces homed in a project.** Should a project's admins be able to create outside-system Interfaces (ci-hook) for their own project? Recommended: v1, after DL13 is specified.
12. **Project-scoped nodes and runtime providers (UC5).** Should they come in v1? The engine would check the scope after the machine provider places an instance, and narrowing a node's scope would meet fixed placement (PLC-009).
13. **Routing sign-ins to a project's person-approvers** (ruling 49's "others authenticate"): when, and how is a person's Interface tied to a person (DL13)?
14. **A secret Conduit approved by a project's approver.** Should this ever be allowed? It would change SEC-008 and the README's secret guarantee. Recommended: no in v0.
15. **Should the applied baseline move into the engine?** Today the synthesis keeps the CLI's applied-IDs file as project memory; it could become state on the Project, so that drift and prune need no committed file.
16. **May one submission span scopes?** Recommended: no. One manifest, one scope, as ApplySet's "at most one set".
17. **Projects across peer engines.** Exposing one project to a peer (UC6) means engine-wide trust approved by root, and exposure approved by that project's approvers, each engine keeping its own half. Is that v1?
18. **Short-form interaction.** Does the parallel short-form investigation adopt `project:` as the manifest key and `authority:`, `audience:` and `approvers:` as file keys? Its answer may move where these keys live, but not what they mean.

Unverified:
- Every count here is a paper estimate. The S1 rows reuse the synthesis's counts, and the canonical delta uses map-attribution's category-4 split (pool-admin 12 + root-workspace 8 + manifest 2 in T1; admin2 16 + manifest 2 in T2).
- The figure of 78 conformance files citing ACC-033 is quoted, not recounted.
- The prior-art claims about GCP, AWS, Azure, Kubernetes, GitHub and HCP come from the provided research, not from fresh reading.
- That a template fixing an undeclared input prevents config widening rests on today's rule that arguments exist only for declared inputs. I did not re-read INS for exceptions.
- My count of 470 `- id:` entries under spec/ differs from the simplification's 455. I did not reconcile them.

## Verdict authority (major)
The flat, one-home, explicit-audience shape holds up. Single home, non-transitive shares, owner-granted audiences, never-shareable `system`, narrowing first and SEC-008 unchanged all survived attack. As written, though, a Project is not a security boundary. I found 4 high-severity breaks and about 9 medium or low ones. One break is fatal as written: delegated register.v1 lets any project admin, or an Agent under a `templates` bound, run code as the engine's OS user, read root's token and become root. Every break has a fix that adds to the design without changing its shape or bringing back nesting.

Scope: I read spec/ and the reference model in <home> (read-only) and attacked the "Flat Projects" design. Severities are for the design as written. Attack traces use the design's own Example B to D actors: root, collab (project-admin of packages), coordinator (homed in conduit-core, operator in packages), pkg-lead, ci-hook.

== HIGH (fatal as written) ==
C1. Delegated registration lets a project admin run code as the engine's OS user, which makes them root.
- Trace, step 1: collab holds register.v1 and replace.v1 on packages (project-admin). It submits into packages a manifest with an implementation item `command: [...]`. MAN-004 and MAN-015 allow `implementations`, and definitions.md:41 says an implementation is reached by command, package or remote.
- Step 2: collab registers and sets up an Agent from it with no `runtime`. PRV-003 (spec/promises/providers.yaml:201) then runs it on the engine's own runner "as the operating-system user ... usually the engine's own and can then read what the engine makes its proof with".
- Step 3: the process reads `<store>/client-tokens/<root id>`. The shell writes it there (reference-model/crates/shell/src/lib.rs:280-299, main.rs:157). It then acts as root: it approves, opens get-secret Conduits and administers every project.
- A variant with no custom code: register a shipped `local-directory.v1` Resource in packages with `config.root` set to the engine's store directory, and give a packages Agent `file.read.v1` on it.
- The `templates` bound does not stop this. OBJ-013 (spec/promises/objects.yaml:230): "A standard argument is accepted whatever inputs the template declares", so `runtime` and `placement` are always the instance author's choice. So the design's claim "Config is not a loophole" is false.
- collab can also `replace.v1` the pkg-worker template, which hands over programs and remote URLs (definitions.md:94). The coordinator then registers the escaping workers for collab.
- The same applies to any managed node (ruling 51), such as the owner's WSL machine. The design lists unscoped placement under "Lost" as a missing convenience; it is an escalation to root.
- Fix:
  (a) A registration authorized by a Project (rather than `management`) may not use the engine's own runner. It must name a runtime provider from a per-Project `runtimes` cap that root grants, and that provider must confine (ACC-027, "enforced").
  (b) Implementation items and contracts in a project submission need root's approval (a pending action), or come only from packages installed through install.v1.
  (c) Let a template fix `runtime` and `placement`, and let bound `templates` pin them.
  (d) Normatively, no run may read the engine's store or tokens: a separate OS user, as DL16 allows.
  (e) Path-valued config of Types that read the host is confined by the runtime, or is engine-level only.
  Until this is fixed, project authority over register.v1 is root by another name.

== HIGH ==
C2. The consumer-half exemption lets anyone inject tools into root and hand authority to outside Interfaces.
- The design requires no consumer half when the consumer is an Interface or an engine-level object.
- Trace 1: collab registers in packages a Resource whose Operation contract carries a prompt-injection `metadata.description`. IFC-033 (spec/interface/implementation.yaml:1068) makes that the tool description. collab then opens root → packages:evil: the target half is satisfied, and no consumer half is required.
  - Root's Interface, in practice the admin AI session, now presents collab's tool.
  - ACC-024 then forbids anyone, root included, from closing it (spec/conventions/access.yaml:500). It goes only if the target is retired.
- Trace 2: collab, or the coordinator (an Agent holding open.v1 on packages), opens a Conduit to Project packages for ci-hook or for a peer engine's Interface (ruling 51), with the ops it holds. An outside party now administers packages, and by C1 owns the host. Neither root nor the other side consented. This breaks ruling 51 decisions 3 and 4 in spirit.
- Trace 3 (tool-name squatting): IFC-014 (implementation.yaml:544) reserves tool names for Conduits that are registered but not yet open. A target-side admin can register Conduits whose consumer is another project's Agent or root under names that consumer will later need. That is denial of service with no consent.
- Fix:
  - Require the consumer half whenever the consumer's scope differs from the target's, and treat Interfaces and the engine level as their own scope.
  - The consumer half for an Interface is engine-level `access` covering it, or the Interface's own acceptance as a pending action. Only root opens Conduits whose consumer is root.
  - Registering a Conduit with an out-of-scope consumer also needs the consumer half.
  - Agents may not open Conduits to a Project for Interface consumers.

C3. A template replace plus setup by the sender's authority makes a confused deputy across projects and into secrets.
- Background: ACC-005 (access.yaml:168) opens a setup's declared Conduits under the sender's right to open. LIF-009 (spec/conventions/lifecycle.yaml:227) and INS-020 instantiate "as that template stands". Bound `templates` covers present and future instances by template ID.
- Trace A: the coordinator holds authority in both conduit-core and packages (UC2). collab replaces pkg-worker to declare a Conduit to `{project: conduit-core, name: tree-031}` with rw.
  - The coordinator registers and sets up a worker.
  - Target half: the coordinator's conduit-core authority. Consumer half: its packages authority.
  - Result: collab's worker writes into conduit-core.
- Trace B: the declared Conduit targets conduit-core:linear-key and lists get-secret.v1. Any root-sent setup opens it, with SEC-008 satisfied (secrets.yaml:286). Root-sent setups include root's apply, an approval whose subject includes set-up, or register-replacement plus setup. That pierces the design's recommended "auth project whose only authority is root".
- Fix:
  - Bound `templates` entries pin the template revision or digest. After replace.v1 they cover nothing new until the grantor re-grants.
  - A setup opens declared Conduits that cross scope or list get-secret.v1 only when the sender registered that template revision, or approved it by digest.
  - Store `registeredBy` and the template revision. Plans and approval evidence list every declared Conduit that a setup would open, with qualified targets.

C4. Approvals are not guaranteed to be human, and approve.v1's authority is undefined.
- (a) approve.v1 is a callable operation. "Never raised at an Agent" checks the family, not whether a person is present.
  - The owner's root Interface is driven by the admin AI session, and collab's Interface could be driven by an AI too.
  - Any run on the engine's runner can also become root (C1).
  - So in the owner's real setup an AI proposes and an AI approves, including the coordinator's own widenings.
- (b) The approval "re-decides under the approver's own authority". But project-approver (inspect.v1 and approve.v1), and the "phone control panel", hold no register or open, so they can approve nothing.
  - If instead approve.v1 confers authority to perform the subject, it is an unbounded admin right hidden behind a "viewer+approver" name: pkg-lead (propose) plus the phone (approve) = full admin of packages.
- (c) The no-self-approval rule checks only the direct opener, so a chain of puppets passes it. Two-person rules cannot be enforced, because an Interface is not a person (DL13).
- Fix:
  - approve.v1 carries its own bound (ops, targets, templates). It is delegated only within the ops the delegator holds (bind/escalate).
  - An approval is accepted only as the response to an engine-raised URL or form elicitation tied to the action digest (the sign-in pattern of ruling 49), never as a tool call.
  - The no-self-approval check is transitive over `openedBy`.
  - Mark which Interfaces are persons.

== MEDIUM ==
C5. Revoking a collaborator does not stick.
- Root closes collab → packages. A ci-admin that collab delegated before then does either of:
  - register-replacement plus open of the Project's declared `authority.collab` entry. A closed Conduit is removed (LIF-035), and LIF-009 replaces removed nested instances.
  - Opens a standalone collab → packages Conduit under a new `as`, which escapes tombstones keyed by local name.
- collab's registered Agents keep running collab's programs and remote URLs with their Conduits. `openedBy` records only the direct opener.
- Fix:
  - One engine operation, "revoke X from P", that closes the transitive `openedBy` tree of Conduits to P and enumerates objects `registeredBy` X.
  - The authority tombstone is keyed by (P, consumer), and only the closer's level may lift it.

C6. Project B can learn that A exists (ruling 52).
- (a) register.v1 resolves cross-scope references (OBJ-021). Registering a packages Conduit that targets `{project: conduit-core, name: X}` succeeds or fails by existence, so it is an oracle for any project or object.
- (b) "A refusal names every target tried" discloses the Pool, the project and existence. Ruling 52's first tier (docs/proposals/ruling-52/README.md) requires the same answer for absent and present.
- (c) propose.v1 is refused when it names an uninspectable object, which makes Example D impossible. If relaxed, the evidence "conduit-core/auth; not shared to packages" and `scopes: [packages, conduit-core]` sit in packages' inventory for collab and pkg-lead to read.
- (d) IFC-014 diagnostics name the other Conduit.
- Fix:
  - A cross-scope reference resolves only through an audience entry covering the registrant's scope, or to an engine-level target. Otherwise the answer is the same not-found.
  - Refusals name only targets the caller may inspect.
  - Evidence is filtered per viewer.
  - Make targets "requestable" through an audience entry with an empty ceiling, so that proposals become possible without probing.

C7. Approvals across the two halves are blind.
- Each approver sees the other half only as "a scope you cannot inspect".
- So collab can approve a write channel from its Agent to an unknown target, which is the B4 exfiltration the consumer half exists to stop. A target approver can likewise expose a Resource to an unknown consumer.
- Fix: proposing discloses to both halves the qualified name, Type and operations of both ends, as RAM shows principals.

C8. Owners cannot see or cut access across the boundary.
- Inventory filtering hides inbound Conduits from scopes the target admin cannot inspect, so collab cannot see who reaches packages:queue.
- Closing needs the target half, so packages cannot close its own Agent's Conduit into Q when root opened it through `access`. It can only retire the Agent.
- Fix:
  - Target-side inspectors always see inbound Conduits, with the consumer's qualified name.
  - Holders of either half may close.

C9. Target half (iii) never checks the actor's bound.
- A bounded operator in packages, for example one with `pools: [pkg-work]`, can open Conduits to every shared-in Resource within the ceilings.
- A bound naming `conduit-core:repo` must be intersected with the entry's `resources` list.
- Bound `templates` must be limited to P's instances.
- Fix: (iii) requires the bound to cover t and O. Coverage = bound ∩ audience entry ∩ P membership.

C10. `management` covering every Pool row silently widens existing grants.
- A holder of management register.v1 with no `access` can now register package-source or machine providers into `system`: supply chain and placement.
- management retire.v1 can now retire every Resource, secret stores included. This conflicts with ruling 52.
- Fix: `system` keeps Pool consent (root only). The migration lists every non-root Conduit to `management` whose meaning widens.

C11. Sign-in phishing (ruling 49).
- The expected account is named in configuration, which collab writes. Every v0 sign-in reaches root.
- If root completes "sign in owner@… for packages/pkg-x", collab's Agent (collab's program) holds the owner's session.
- Fix:
  - The sign-in action shows scope, `registeredBy` and program digest.
  - In v0, refuse sign-ins for instances not registered by root, or limit accounts to an allowlist per Project that root declares.

== LOW ==
C12. Engine names can be spoofed in project scopes.
- A packages Resource can be named `management`, `access` or `local`, a Pool `system`, an Agent `root`. These read as engine objects in plans, evidence and elicitations.
- Fix: reserve first-start names, node names and `engine` in every scope, and always show scope/kind/name.

C13. Retirement race.
- No retiring state exists, so delegates keep registering, and approvals keep dispatching, into P during retire-with-contents. UC7 required this to be refused.
- Fix: dispatch moves P to retiring, and register, open, propose and approve into P are refused from then on.

C14. Audience entries in documents name projects by name.
- After packages is retired and recreated, re-applying root's engine file quietly re-shares to the namesake.
- Fix: store IDs. The planner treats a re-resolved name as a widening that needs `--recreate`.

C15. Evidence goes stale over the 7-day window.
- Fix: recompute it on display, and have approve.v1 carry the digest of the evidence that was shown.

C16. Control flows through shares.
- queue [post] shared to packages is read by conduit-core's coordinator, which holds authority. Authority does not follow a share, but prompt injection does.
- Fix:
  - share.v1 evidence flags write-like ceilings into Resources that authority-holding Agents of the home read.
  - Tool descriptions mark the other project's content.

C17. Routing gap.
- A Project is an engine-level object, so by (i) and (ii) opening a Conduit to P needs `access`. That contradicts collab delegating project-viewer to ci-hook.
- Fix: add an explicit row saying P's open.v1 covers Conduits to P, under section 5's "within" rule.

== Probes asked, and what held ==
- An Agent in A to a Resource in a shared Pool: only A's admins may open it, within the entry, given fix C9. The home can revoke at once by close, unshare or retire, and this held. A Pool's owner can always revoke.
- A project's Interface approving a widening into a shared Pool: beyond the ceiling it needs the home's approver, and this held (modulo C7). An Agent cannot approve, only as a family check (see C4).
- Moves: refused, which held. register-replacement and template replace are the effective moves (C3, C5).
- In-flight requests run to the end (ACC-034). Narrowing first held.
- Shares are non-transitive. The never-shareable list held. SEC-008 is unchanged except through C1 and C3.
- Rejecting nesting is sound. Every break above applies equally, or worse, to variant H.

Severity is "major", not "fatal": C1 is fatal as written, but every fix adds to the design without changing its shape, and the design should not go to the owner without C1 to C4 fixed.

## Verdict expressiveness (major)
Major gaps, none fatal. The flat Project design can state S1, UC1, UC8, UC10 and the owner's two Pool examples. It cannot yet state granular Interfaces or approvals as written. Three of its own examples contradict its own rules: an approve-only Interface can approve nothing, Example D is refused by propose.v1's own rule, and collab's delegation has no route. It also has two holes. Engine-level and Interface consumers skip the consumer half. And a bound on a Conduit to a Project does not limit the management operations, so `project-operator` can rewrite the very template its bound is pinned to. A share's ceiling lists operations only, so a share cannot cap restrictions (S8, A78's queue). UC5 and UC6 cannot be expressed in v0. Each problem has a local fix.

Scope: I checked the design against UC1-UC10, S2-S10 (ROADMAP.md:133-142) and A78 (ROADMAP.md:71; docs/implementation/WORK_ORDERS.md:118-127). I verified spec claims in spec/conventions/access.yaml (ACC-005:168, ACC-012:287, ACC-030:583), spec/promises/templates.yaml:262 and instantiation.yaml (INS-014), docs/proposals/ruling-53/README.md and docs/proposals/simplification/research/synthesis.md:80. Nothing was modified. Each finding is graded MAJOR, MODERATE or MINOR.

== MAJOR ==
M1. An approve-only Interface approves nothing (internal contradiction).
- Rule: approve.v1 re-decides each half "under the approvers' own authority".
- Clash: the `project-approver` template (inspect.v1 + approve.v1) and the design's own "phone control panel" example hold no open.v1 or register.v1. So the authority they approve under covers nothing.
- Failure: a person given only approval rights on conduit-core can never approve the target half of a proposal (UC4, UC10). The owner's granular Interface fails exactly where it is meant to work.
- Fix: give approve.v1 its own envelope, for example a bound on the approver's Conduit that states what it may approve. Or drop the template and say that approvers must hold act rights.

M2. Proposals cannot reach what is not already shared, and approvers decide blind.
- Rule: propose.v1 is refused when the subject names an object in a scope the caller may not inspect.
- Example D: pkg-lead names conduit-core:linear, which is not shared to packages, so the design's own example is refused.
- Example D's evidence also shows collab "conduit-core/auth; not shared to packages". That contradicts the disclosure rule, under which a one-half approver sees only "needs approval in a scope you cannot inspect". If that rule holds, collab approves the consumer half without knowing the source. That is the exfiltration and prompt-injection risk the consumer half exists to stop.
- Two non-root admins cannot combine halves:
  - collab cannot name conduit-core objects;
  - dana (a conduit-core admin) cannot name packages members;
  - an audience grants per project, never per consumer (an entry names projects, not Agents).
  So "let pkg-worker-docker alone read X in conduit-core" needs root, or a share that lets any packages admin wire any packages member.
- Fix: let a proposal name a target by a reference its home has published (an invitation or an audience entry with a `consumers` list). Show the consumer-side approver the target's Type and description.

M3. The consumer half exempts engine-level and Interface consumers, which reopens B4.
- Rule: "Engine-level consumers and Interface consumers need no consumer half."
- Failure: any holder of open.v1 on P can wire root's engine-level Agents, and any Interface, into P's Resources and into Pools shared into P. That includes the coordinator under a bounded `project-operator` Conduit, unless its bound limits consumers, and bounds never do. Root never consents.
- UC2 must refuse "opening a Conduit whose consumer is outside the scopes it holds". It fails for every engine-level consumer.
- Fix: treat the engine as a scope with its own half (`management` or an unbounded `access`). For an Interface consumer, either require consent from that Interface, or state the exemption and its consequence plainly.

M4. A bound on a Conduit to a Project is defined only for opening and registration.
- The design does not say how a bound limits set-up, retire, replace.v1, unregister, retire-with-contents, share.v1 or inspect for a Project's members.
- `project-operator` is "every operation except approve.v1", so it includes replace.v1, unregister.v1 and retire-with-contents.v1.
- Failure in Example B: the coordinator holds `project-operator` bounded to `templates: [pkg-worker]`. It can replace.v1 pkg-worker itself, for example to turn accountServices on or change the program. The bound follows the template's ID, so the coordinator then creates workers it was never approved for. This defeats the "config is not a loophole" argument and the UC2/UC4 refusals.
- The same coordinator can retire-with-contents packages.
- retire.v1 on P covers both P's members and P itself (rows c and f). So "retire workers, never the project" cannot be stated.
- Fix:
  - define a bound's effect on every row: targets limit retire, set-up and inspect; `templates` also limits replace.v1;
  - split retire of the Project into its own operation;
  - make `project-operator` exclude replace.v1, unregister.v1, retire-with-contents.v1 and share.v1.

M5. A project admin's delegation has no route.
- A Conduit whose consumer is an Interface and whose target is Project P has two engine-level ends. Interfaces and Projects are engine-only (PRJ-003).
- So its registration falls under row (b), `management`, and only root can make it. The Project's `authority` entries live on the Project document, which also only `management` registers.
- Failure: Example C's "collab delegates project-viewer to ci-hook" cannot be done by collab. This is B6 come back.
- Fix: state that a Conduit whose target is P is registered in P's scope, and that P authorizes its register and unregister.

M6. The coordinator needs authority over its own project to take turns with its workers (A78, UC9).
- Each order's turn Conduit, coordinator (conduit-core) → worker (packages), crosses scopes. Its consumer half needs open.v1 on the coordinator's own home, conduit-core.
- So in A78 the coordinator must hold open rights in conduit-core, the project that holds auth. Otherwise it needs a hack: a Conduit listing open.v1 with an empty bound. Whether a bound limits the consumer half at all is not stated.
- Fix: the consumer half holds when the actor is the consumer itself and consents to its own reach. Alternatively, define the consumer half against a bound that names consumers.

M7. A ceiling lists operations only, never restrictions.
- ACC-029 and ACC-030 confirm that a bound does not limit filters. An audience ceiling has the same gap, so a Pool's home cannot require a filter on other projects' consumers.
- Failures:
  - S2 across projects: the home cannot cap the restriction placed on the shared login.
  - S8: a shared search Resource indexing Resources that were never shared leaks their derived content to every audience project. Nothing ties derived content to scope.
  - A78/UC9: the shared queue shows packages workers conduit-core's orders. The only workaround is one queue per project.
- Fix: add an optional `restrictions` list to audience entries that every cross-project Conduit must carry. State that derived content stays in its source's scope, or that a share of it must name its sources.

M8. Placement is not scoped (UC5, S3; the design admits this).
- Nodes are engine-only, so any project's instance may land on any node. "A machine for one project" cannot be stated.
- A runtime provider can serve one project or all of them, never a subset, because providers cannot be in an audience.
- declare-lost and repair of a lost machine's instances are root's alone, so collab cannot recover packages after a machine loss.
- Keep it as a stated v1 item, but the plan should not call UC5 covered.

== MODERATE ==
N1. A proposal binds IDs but not content (S5).
- expectedId does not change when a template is replaced in place or when a control's revision moves. A proposal to register `from: pkg-worker` can therefore be approved after pkg-worker was rewritten.
- The engine's evidence ("reach before and after") cannot describe widenings made by settings: controls, a replace.v1 that changes `remote`, a directory root moved up.
- Fix: bind template digests and instance revisions in the subject, and say plainly in the evidence that a change is opaque.

N2. The approval flow has no answer for the outcome model (S4, S6).
- It does not cover:
  - a lost approve.v1 response, and whether repeating it is idempotent;
  - an unknown outcome partway through a dispatch with no rollback, and who reconciles it;
  - proposals whose proposer's Conduit was closed; they are voided only when the proposer is retired;
  - whether a recorded half stays valid after its approver's Conduit closes before the last half arrives.

N3. Only Pools can be shared.
- Agents cannot be shared: no shared reviewer Agent, and no standing consent for messages or turns from a worker to the coordinator. Each such Conduit needs an actor holding both projects.
- Interfaces have no home project, so audiences never apply to them. collab cannot read the shared repo or skills, and ci-hook cannot post to the queue, without root.

N4. approve.v1 is "Interface only", but nothing tells a person from an outside system (DL13). ci-hook could approve. The owner's "through the Interface" probably means a person.

N5. Co-admins can revoke each other.
- collab can close dana's project-admin Conduit, and root's own `authority` entries on the Project. That leaves the document drifting from the engine.
- UC3's "must refuse removing authority it did not grant" fails. Only root's Conduits are protected.

N6. The owner's three files depend on each other in a cycle.
- repo's audience names packages, packages' wiring names conduit-core:repo, and the coordinator's authority names Project packages.
- With one scope per submission, applying them takes several passes and is not atomic, so a first apply partly succeeds.

N7. An auth Pool inside a working project is unsafe.
- Any conduit-core holder of replace.v1 or register-replacement can redirect a secret-reading Resource. For example, a template replace changes linear's `remote`.
- The only gate is root's next routine setup (SEC-008), and the evidence cannot show the change.
- The design's fix is a root-only auth project plus a share. That is a workaround, not the owner's literal "auth pool for a specific project".

N8. Several things stay root's (admitted; ruling 49 fixes the sign-in part).
- install.v1 and creating Interfaces: a project admin cannot add a package (S7) or a ci-hook (UC3).
- Sign-ins and enrollment for a project's Agents (UC3, UC5).

== MINOR ==
- First-start names are not reserved inside projects. A project Resource named `access` or `management` captures unqualified references in that project's documents and acts as a decoy. Engine objects must be written `project: engine`.
- A cross-project standalone Conduit may be registered into either end's scope. So the other end can squat the consumer's local names (`as`), and which scope's tombstone applies is unclear.
- Where a package template's fixed references resolve is not specified.
- NAM-022 keeps `#name` per submission, so a project registered in several submissions cannot share its own local contracts.
- A `templates` bound that names an engine-level or package template: whether it covers instances in other projects is not specified.
- Engine-level Agents cannot propose (propose.v1 exists only on a Project).
- A quorum, or a two-person rule for root, cannot be stated.
- A holder that may only propose cannot re-point a Conduit atomically. The subject may not include a close it could make itself, so it must close first (an outage) or open first (an overlap, and it is not woken to finish).
- A retired Project keeps its name reserved, while UC7 asked for it to be freed. The design chose this deliberately; record the divergence.
- How a project admin reads its project's records is not specified.
- Inconsistencies in the examples:
  - Example C says ci-hook-queue is "registered into packages (target in packages)", but its target is in conduit-core.
  - The coordinator's `templates: [pkg-worker]` bound omits the per-order tree template, so it could not register the trees. A tree's root can be limited only by narrowing that template's input schema (templates.yaml:262), for example with a path pattern. That works but is fragile.

== COVERAGE ==
| Case | Verdict | Blocking findings |
|---|---|---|
| UC1, S1 | Yes (27 concise lines) | |
| UC2 | Mostly | M3, M4, M6, N7 |
| UC3 | Partly | M5, N3, N5, N8 |
| UC4 | Partly | M1, M2, N1 |
| UC5 | No (deferred to v1) | M8 |
| UC6 | No (deferred to v1) | |
| UC7 | Mostly; no re-homing, so the home's admin cuts every audience at once | N6 |
| UC8 | Yes | minor: decoy names |
| UC9, A78 | With workarounds | M6, M7 (queue), tree template, sign-ins root-only |
| UC10 | Yes, by root's audience entry | |
| S2 | Yes within a project | M7 across projects |
| S3 | No project-scoped placement | M8 |
| S4 | Partly | N2 |
| S5 | Partly | N1 |
| S6 | Good on unshare and retire cascades | N2 |
| S7 | Partly | N8, minor: package template scope |
| S8 | No across projects | M7 |
| S9 | Unchanged; `management`'s wider meaning inverts some vectors; repair is root's | |
| S10 | Fine | minor: providers shareable to one project or all |

Unverified:
- Ruling 53's tombstone and `as` rules are not in spec/ yet. I judged them from the ruling-53 record and synthesis.md:80.
- I did not check IFC-014's precedence for local names that collide across scopes.
- I did not trace how a records read is authorized today.

## Verdict cost (major)
The cost is affordable but major, and about 25 to 40% above the design's own figures. The figures it cites check out: 78 conformance files cite ACC-033, there are 470 promise IDs, and 13 steps invert, which is within its 5 to 15. What it undercounts is four things: the sixth family has to be added everywhere families are listed, names per scope touch 77 lookups in the core, renaming today's "project" to "submission" changes the wording of about 46 more promises, and the conformance cases are far bigger than "3,000 test lines". Revised totals: about 85 to 90 promises change in substance (18 to 19% of 470), or about 130 counting the rename; the core grows by 4,500 to 7,000 lines (45 to 65% of today's); 80 to 120 new conformance cases; 10 to 13 work orders. Two parts are logically inconsistent and their cost is not counted: refusals that name targets can disclose objects, and an approval decides one request under the authority of two actors. The four parts can be built in stages: Project and routing, then sharing, then approvals, then names.

Everything here was read in <home> (read only). I ran no build or tests. Line and vector counts come from the tree and from git history.

COST TABLE (design's claim | what I checked | my estimate)

| Item | Design | Checked | Estimate |
|---|---|---|---|
| Promises today | about 460 to 470 | 470 unique `- id:` entries under spec/, no duplicates. The simplification's 455 is stale | 470 |
| New promises | about 28 | Pending actions, tombstones, inventory and the second refusal tier are not in spec/ yet. ruling-53/README.md says to write them only after the short-form investigation. So the PND rules and ruling 53's five additions are owed anyway, partly | 31 to 34. Add: Project Types are core-only (like PLC-028 for nodes); a package manifest must not carry `project`; the rule by which an approval decides a request under two approvers; disclosure of the reach that existing `management` holders gain at upgrade |
| Promises changed in substance | about 38 (41 listed) | Missed, because each lists families or depends on one actor deciding: OBJ-001, INS-001, INS-024, TPL-001 (the Project's `authority` slot, like `entries`), TPL-002, TPL-019, LIF-001, CTR-004 (family enum, schemas/contracts.yaml:163), ACC-001 and ACC-016 (joint approver authority), ACC-019 (pending state is not later work), IFC-014 (tool-prefix collisions across scopes), PRV-020, CON-031 | 53 to 56 |
| Wording only: "project" renamed "submission" (open question 2) | NAM-022, NAM-027, SUB-005 and definitions only | The word appears in 56 promises. Also definitions.md (58 uses), schemas/README (23), core/operations.yaml (13), core/README (8), the example folder schemas/examples/valid/project/. In code: packages::Project, Input::RegisterProject, `scope == "project"`, and the stored key `held.contexts.project`. 279 conformance cases use the runner inputs `register-project` and `resolve-project` | About 46 more promises, wording only. All of it is avoided if the new object gets another name |
| Share of the spec touched | 66 (14%) | | 85 to 90 in substance (18 to 19%); about 130 with the rename (28%) |
| Core package exports (61 today) | listed | All 7 core templates today are first-start objects, protected by FST-006. `project` and the four `project-*` Conduit templates would be the first core templates that user documents instantiate | About 16 new: a Type, a capability, 5 operations, 5 templates, 3 or 4 schemas. About 16 changed: 11 reference schemas plus the target enum, bound.v1, the inputs and outputs of inspect.v1 and register.v1, pool-management.v1, record.v1 |
| spec/schemas | listed | Kind enum in document.yaml and envelope.yaml, `family` in contracts.yaml, instances.yaml:148, templates.yaml:131, manifest.yaml | 6 files, plus valid and invalid examples checked by tools/check_schemas.py |
| Vectors that invert | 5 to 15 | 13 steps in 4 cases. conventions/access/entry-authority steps 0,2,4,6,8,10: `management` alone inspecting, setting up or retiring a Resource of a live Pool, refused today. conventions/access/routing steps 0 and 5. conventions/access/s1-pool-authority steps 0 and 3; its title "existing owner needs two paths" becomes false. language/first-start/protected steps 2 to 4: refused becomes invalid under FST-006. The paragraph in conformance/README.md also changes | 13, near the top of the design's range. Other ACC-033 citers stay valid, because assertions are required-field projections, so added scope fields do no harm. Exception: cases asserting `absent` diagnostics break if a refusal names targets |
| ACC-033 citers | 78 | Exactly 78: 76 case.yaml files, plus manifest.yaml and README.md | Their meaning holds for engine-level objects |
| New conformance cases | 70 to 100, about 3,000 test lines | The core has no unit tests; its tests are conformance cases. Today 367 cases cover 249 promises. Case size: conventions/access 26.8k lines over 18 cases, setup 45.9k over 39. Past orders: 014 added 18 cases and 26k lines, 016 added 39 and 43k, 017 added 16 and 26k | 80 to 120 cases, about 100k to 170k lines of case data, mostly generated. That is 3 to 5 orders that carry vectors |
| Core (10,465 lines today; access.rs 729) | 3,000 to 4,500 lines | `object()` (access.rs:34) matches names with no scope, and 77 name lookups across core depend on that. The invocation stores a boolean `management` where it would need the authorizing target | 4,500 to 7,000. Project family, setup and retire-with-contents 700 to 1,000. Names per scope, qualified references and resolution in the Conduit's scope (LIF-031) 800 to 1,300. ACC-033 by scope, the two halves, delegation, bound `templates`, CON-028 600 to 900. Audiences and their close cascades 500 to 800. Pending actions (propose as a dry run, the reach before and after, expiry, voiding, approve with joint halves) 1,200 to 1,800. openedBy, filtered inventory, tombstones per scope 400 to 700. Records 150 to 300 |
| Loader | not costed | Schema-driven. FAMILIES is a constant in authored.rs:33 and schemas.rs:62 | 100 to 250 |
| Conformance runner | not costed | | 200 to 500: a time-advance input for expiry, scopes in snapshots |
| Shell | 800 to 1,500 | A token is issued only for root (main.rs:157, :212) and for run instances (lib.rs:262-299). DL13 is unspecified. The CLI's `named()` takes KIND/NAME and rejects `/` | 1,000 to 2,000, plus a DL13 specification first |
| CLI | 400 to 800 | No plan or apply and no concise form exist yet. author.rs is 284 lines | 400 to 800, on top of an authoring layer not yet built (ruling 53 decision 4; short form pending) |
| Store migration | not costed | shell/src/store.rs: VERSION = 1, matched exactly, with no migration path. State has deny_unknown_fields. Objects are untyped JSON values | A v2 reader: no scope means engine, plus empty lists for pending actions and tombstones; 100 to 200 lines. The semantic problem: every existing non-root Conduit to `management` silently gains every Pool row, including retiring the machine and package-source providers that `system` owns. Ruling 52 requires that to be disclosed at upgrade. In practice only S1 lab stores exist |
| Work orders | 8 to 10 | Past rate is about 1,100 to 1,600 Rust lines per order (014: 1,158; 016: 1,605; 017: 1,255) | 10 to 13, counting 2 or 3 passes to write the spec |
| Reading cost | +1 concept for S1 (−1 root Pool Conduit); about +4 for sharing and approvals | definitions.md has 320 rows and 26.9k words | S1 as claimed. Sharing and approvals add 6: scope, audience with ceiling and `future`, the two halves, propose and approve, bound `templates`, openedBy. About 10 new definition rows; "submission" too if renamed |

FLAWS THAT CHANGE THE COST
1. Refusals that name targets can disclose objects. "A refusal names every target tried" conflicts with OPS-023 ("says nothing else about the objects it names") and with the design's own non-disclosure claim. A request travels through exactly one Conduit (Request.conduit), so the targets "tried" can only be the ones that would have authorized it. Those come from the object's home, so naming them reveals which project an object lives in. The rule needs wording that takes targets only from the caller's own Conduits.
2. An approval decides one request under two actors' authority. One approver supplies the target half and another the consumer half. Today one actor's Conduits decide each request (ACC-001, ACC-016), and REC-002 records one caller and one Conduit. This needs a new promise, a change to record.v1, and new code.
3. propose.v1 must test the whole subject as a dry run, documents included, and must compute the reach before and after for each consumer. That is a new computation of effective reach, about 600 to 900 lines on its own.
4. The family addition is undercounted, so the Resource-form alternative (open question 3) saves about 10 to 12 promises, not 5 to 8.

CHEAPER OPTIONS (savings against the estimate)
- Keep names global (open question 4): saves about 10 promises, 800 to 1,300 core lines and about 15 cases. The price is name squatting across projects, which expectedId and tombstones only partly prevent.
- Give the new object another name (open question 2): saves about 46 wording-only edits and the identifier churn in code.
- Keep Pool consent (open question 5): no inversions, and ACC-033's "one target authorizes" stays true. The price: canonical S1 keeps root's Pool Conduit, so the −40 lines are not saved.
- Stage the work. (A) Project family, scope, routing and inventory, written together with ruling 53's additions: about 50 promises and 2,800 to 4,200 core lines. (B) Audiences and the two halves: about 15 promises and 900 to 1,400 lines. (C) Propose and approve, after ruling 53 decision 2: about 15 promises and 1,200 to 1,800 lines. Pending actions for sign-ins and enrollment are owed by rulings 49 and 50 whatever is decided.

KEY FILES
spec/conventions/access.yaml (ACC-033 and its observable "at any moment one target authorizes")
reference-model/crates/core/src/access.rs (routing at :235, row-1 Pool check at :445-455, `object()` at :34)
reference-model/crates/shell/src/store.rs
conformance/conventions/access/entry-authority/observations.json
conformance/conventions/access/routing/observations.json
conformance/conventions/access/s1-pool-authority/observations.json
conformance/language/first-start/protected/case.yaml

NOT VERIFIED: the S1 line deltas (taken from map-attribution); the prior-art claims; whether shell tests beyond a grep would invert (none found asserting a refusal from `management` alone).