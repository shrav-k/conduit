Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

## recommendation
## Recommended model in one paragraph

Make **Project** a sixth family. Projects are flat and live at the engine level. Each object has exactly one **home** (a Project or the engine), recorded on the object when it is registered and never changed. A Project "references every object under it" through an inventory computed from those records. It keeps no member list of its own.

- **Authority over a project** is an ordinary Conduit to the Project. It carries a new capability, `project-management.v1`, and a new requirement, `project-bound.v1`. An Interface with authority over three projects holds three Conduits, and each one can be revoked by its own project.
- **Root and `management` enclose every Project**, so root needs no Conduit per project or per Pool. Engine-level objects, `system` included, keep today's rules exactly. An engine with no Projects therefore behaves as it does today.
- **A Pool has one home.** Other projects use it only through **audience entries** that its home grants. Each entry names the receiving projects and Resources, an operation ceiling, and any restrictions every Conduit must carry, and it says explicitly whether it covers projects and Resources made later. An entry grants use and never administration.
- **A Conduit that crosses projects needs two halves:** authority on the consumer's side and on the target's side (ruling 51, decision 4).
- **What waits is decided by authority, not by a classifier of widenings.** An actor acts at once inside its envelope, which is a bounded Conduit that a person opened, and proposes anything beyond it. An approval is a person's answer on a page the engine serves, never a tool call. Narrowing never waits.

This combines the four designs:
- **From P3-flat:** the skeleton (a family, audiences, two halves, authority deciding what waits, no nesting).
- **From P4:** backward compatibility. `management` encloses Projects only, so no existing routing vector flips and `system` keeps its two keys.
- **From P1:** the explicit bound with five dimensions, and explicit `propose.v1` and `approve.v1`.
- **From P2:** role templates and the standing target half.
- **From the twelve skeptic reports:** the fixes below.

## Fixes applied from the skeptics (mapped to their findings)

1. **Root-equivalence in v0.** This was fatal or critical in all four designs (P1 C2, P2 C1, P3 C1, P4 #1 and F1). The fixes:
   - An instance registered through a Project's authority is made only from a template homed in that Project. It takes `runtime` and `placement` from that template and cannot state them itself.
   - Templates of a Project are registered, replaced or unregistered only by engine authority, or by a holder of `templates: all`, which only engine authority can grant.
   - A Project submission may select implementation items only from installed packages. Its own `command` or `remote` items need `management`.
   - The spec README says plainly that in v0 a Project is a **mediated** boundary. A run on the engine's runner can read root's token (IFC-022; shell lib.rs:280-299). Before a second, untrusted person gets authority, three things must exist: a DL13 token path, a store the runs cannot read (a separate OS user, DL16), and a confining runtime.
2. **Approval was a tool, so a model could approve** (P1 C1, P3 C4). Now:
   - `approve.v1` is not presented as a tool (an IFC-012 exception, like `get-secret.v1`). It is answered only on an engine page reached by a URL elicitation, the sign-in pattern of ruling 49.
   - Only Interfaces of a new core Type `person.v1` may hold it.
   - Opening any Conduit to a Project, a Pool, `management` or `access` whose consumer is an Agent is also confirmed by a person, because that envelope is a standing approval.
3. **The consumer half applies everywhere** (P1 C5/C12, P2 C4/C5/C6, P3 C2/M3, P4 F9).
   - Every open, and every registration of a Conduit, needs both halves.
   - An Interface consumer's half belongs to that Interface itself or to root.
   - Only root opens or registers a Conduit whose consumer is root.
   - A bounded Conduit to `access` supplies only the target half.
4. **Template coverage and `replace.v1`** (P1 C6, P2 C7/M1, P3 C3/M4).
   - A bound pins template revisions.
   - Coverage through a template reaches only instances registered through the same Conduit.
   - Replacing a template does not widen any standing bound.
5. **The bound limits every verb** (P1 C13, P3 M4). `retire-with-members.v1` and `includeShared` need an unbounded Conduit, and no core role carries `retire-with-members.v1`.
6. **An approval binds content, not just IDs** (P1 C3/C4/C17, P4 #3).
   - It carries an evidence digest, which is recomputed at the last approval, plus template revisions.
   - Only a closed list of operations may be proposed.
   - The subject is decided by what the approvals jointly permit, never by the final approver's whole authority.
   - Anything that would open a `get-secret.v1` Conduit, declared Conduits included, is root's own act and is never proposable.
7. **Shares go from a Pool to projects, not to actors** (P4 F2).
   - Resources and later coverage are listed explicitly (P1 C7, P3 M8).
   - An entry may carry required restrictions (P1 M3, P2 M7, P3 M7).
   - No ceiling may hold a control or a management, pool or access operation (P1 C14/C15, P2 C12).
   - No Pool whose Resources hold authority Conduits may be shared (P1 C8).
   - A `requestable` entry lets another project propose without probing (P2 M6, P3 M2).
8. **Disclosure.** A reference into another project resolves only through a half the caller holds or an audience entry. Otherwise the answer is the one a missing object gets. Refusals name only targets the caller may inspect, and there is no listing of projects (P1 C11, P2 C23, P3 C6, P4 #13).
9. **Revocation.** Conduits store `openedBy`, objects store `registeredBy`, and a narrowing `revoke.v1` closes an actor's grants in a project transitively (P1 M10, P2 M4, P3 C5, P4 #10).
10. **Retirement race.** `retire-with-members.v1` retires the Project first, so its authority Conduits close and nothing more can register into it. Its members are retired after that (P2 C14, P3 C13, P4 #11).
11. **`management` encloses Projects only.** It does not gain the Pool rows of engine-level Pools, so `system`'s Resources keep both keys and no existing grant silently widens (P1 C10, P2 C20, P3 C10).
12. **Pending-action phishing and fatigue** (P2 C9, P3 C11).
    - Root is prompted only where no other approver covers a part.
    - Sign-in actions show the registrant chain and the program digest.
    - In v0, sign-ins are raised only for instances whose template root registered or approved.
13. **Secrets** (P1 C3, P2 C8, P4 #7). The auth Pool is homed in a root-only project, and only its service Resource is shared. Templates behind `get-secret.v1` consumers can be replaced only by engine authority.
14. **Peers** (P1 C9). Interfaces from another trust domain are marked. Giving one a Conduit to a Project, `management` or `access`, or `approve.v1`, needs root.
15. **Names.** The five first-start names are reserved in every project, there is no fallback lookup, and tombstones are keyed by project ID and by (consumer ID, target ID, capability) (P2 C17/C18, P3 C12, P4 #17).
16. **Cost-skeptic defects.**
    - The record names the Conduit actually used, not "the narrowest".
    - Reconcile routing is stated.
    - An invocation stores its scope when it is made.
    - The store break is stated as a clean break.
17. **Smaller additions.**
    - The caller's project is passed in an implementation's keys (P1 M4).
    - A proposal may pair a close with its replacing open (`replaces`), so re-pointing causes no outage (P1 M6).
    - An optional `sameRegistration` pairing in the bound keeps worker-032 out of tree-033 in A78 (P1 M2, P2 M3).

## Where this follows the owner's direction

- **A project references every object under it:** Agents, Pools and their Resources, Conduits and templates. This is the computed inventory of ruling 53, limited to what the caller may inspect.
- **Pools usable from many projects, and specialized Pools:** a skills Pool shared to all projects, read-only, and an auth Pool whose service only one project reaches.
- **Interfaces with authority over one or more projects:** one Conduit per project. That makes granular Interfaces possible: an admin, an operator, a viewer, and an approve-only control panel for a phone.
- **Projects determine an Interface's permissions:** a project's own file declares the Conduits to it (`managers:` and `approvers:` in the concise form). Its inventory shows them, and either side can revoke them.
- **Who may approve is declared:** approvers are the person Interfaces that hold `approve.v1` on the project, declared like any other authority.

## Where it departs, and why

1. **"Every possible object" stops at the engine.** These stay engine-level: Interfaces, Projects, nodes, everything owned by `system`, the first-start objects, packages and records. Putting `system` in a project would hand out near-root power (FST-001). Kubernetes makes the same split between namespaced and cluster-scoped kinds.
2. **The reference is computed, not stored.** A member list kept on the Project would drift and race; this is why Kubernetes replaced allowlist prune with ApplySet.
3. **"Pools accessible from multiple projects" holds for use, not administration.** A Pool has one home and is never co-administered. Two homes would break preservation test 3 ("owned once"), ACC-033 and ACC-012. Every system in the prior-art study uses one home plus explicit share edges.
4. **Interfaces are principals, not project members, and only root creates them in v0.** If a project homed a person, that project's managers could retire someone whose authority spans other projects. Issuing an identity is an engine act (DL13).
5. **A project never grants itself anything.** "Projects determine permissions" means the grants live with the project and are opened by authority someone already holds. The first such authority always comes from root.
6. **"An auth pool for a specific project" is realized as a root-only home with a one-project audience, not a Pool the project administers.** SEC-008 keeps every secret wiring root's own. If the project administered the Pool, its managers could re-point what reads the secret, and root would become a confused deputy.
7. **Granular Interfaces are mediated in v0.** They bound well-behaved actors and agents and record everything, but they do not isolate a collaborator who can run code on the owner's machines until the three prerequisites in fix 1 exist.
8. **The approvals direction is not in the owner's relayed words, nor in docs/proposals/owner-direction/2026-11-10.md.** I offer it as the coordinator's answer to ruling 53's open decision 2, staged last, and as an explicit decision for the owner. It also departs in two ways:
   - It does not gate widenings by a classifier. Authority decides what waits, because a gate would make every work-order dispatch wait for a person.
   - Agents never approve in v0, a stated exception to preservation test 5.
9. **No nesting or folders.** Inheritance would contradict "projects determine an Interface's permissions": a parent's grant would apply inside a project that cannot see it or close it. Strong team boundaries belong in separate engines (ruling 51 peers).

## Honest size

About 100 promises change in substance (about 40 new, about 60 changed) out of about 455. A further 45 to 55 promises change wording if the word "Project" is kept for the new object. That is larger than making the short form normative (50 to 80), not the same scale. In return, no existing routing outcome flips, and S1 stays at 27 concise lines. It should be decided together with the short-form ruling and written in four stages.

## model
## What a Project is

**A sixth family, `kind: Project`.** OBJ-020 grows to eleven kinds.
- Its Type is `core#project.v1`, which the engine serves, as it serves `management` and `access`. In v0 it is the only Project Type, and a rule makes Project Types core-only, as PLC-028 does for nodes.
- It offers `core#project-management.v1`.
- It is made from the core template `project`. That is an ordinary, instantiable core template, the first one that is not a first-start object (FST-006 is unchanged).
- It has no placement, runtime, hooks or `config`, and runs nothing (TPL-019).
- Its states are registered, ready and retired, the one common lifecycle of ruling 29.
- It is never a consumer or a caller, owns nothing, and is never a member of another Project. Projects are flat.

**Why a family, rather than a Pool Type (P4) or a Resource owned by `system` (P2):**
- A Conduit needs a target with an ID and a lifecycle. Design D's skeptic required a project identity "that is checked, not the Pool's name", and memory that outlives every Pool.
- As a Resource owned by `system`, a Project would be covered by any bound naming `system`, which is the FST-001 hazard.
- As a Pool Type, "Pool" would mean two things (owner of Resources, and home of every family), and `retire-with-resources.v1` would retire Agents. That breaks ruling 42's one-meaning rule.
- Cost: about 10 to 12 more promises than the Pool-Type form (decision 2).

## What it holds, and what is computed

**Stored on the Project:** its ID, name and description. That is all.

**Engine state keyed by the Project's ID**, which outlives the Project until root purges it:
- its tombstones (ruling 53);
- its pending actions;
- the scope of each invocation made through it.

**Stored on each member:** `project` (engine-owned, OBJ-004), `registeredBy` (actor and Conduit), and for Conduits `openedBy`.

**Computed by `inspect.v1` on the Project** (ruling 53's inventory, per project, showing only what the caller may inspect and no counts of the rest):
- members by kind;
- every Conduit a member consumes or is the target of, with its home and `openedBy`;
- the Conduits to the Project, which are its authority, with their bounds and roles;
- the audience entries of its Pools, and the entries that admit it;
- pending actions;
- tombstones.

## Membership, kind by kind

| Kind | Home |
|---|---|
| Agent | Exactly one: the Project named by its submission's manifest key `project:`, or the engine. Fixed for life; moving means replacing. |
| Resource | Its owning Pool's home. POOL-001 is unchanged. In P's submission a Resource must be owned by a Pool homed in P. Only the home registers into a shared Pool. |
| Pool | Exactly one home, a Project or the engine; never two. `system` is engine-only. A Pool reaches other projects only through audience entries. |
| Declared Conduit | Its consumer's home (CON-002). |
| Standalone Conduit | The scope it is registered in, which must be the home of its consumer or its target. A Conduit whose target is a Project is homed in that Project, so a project's authority grants are its own members. Engine-level if both ends are. |
| Template | A Project or the engine. A template of P is registered, replaced and unregistered only by engine authority or by a holder of `templates: all`, which only engine authority grants. It selects implementation items only from installed packages unless it goes through `management`. Instances registered through P's authority come only from P's templates and take `runtime` and `placement` from the template. Through `management`, any template may be used, as today. |
| Interface | Engine only. A principal, never a member; registered only through `management`. Persons use the new core Type `core#person.v1`, which root declares. |
| Project | Engine only. Names are engine-unique; `engine` is reserved. |
| First-start objects, nodes, machine and package-source providers, everything owned by `system` | Engine only (FST-001, PLC-008, PRV-013). Never shareable. |
| Runtime and secret-store providers | Homed like any Resource. A member's `runtime` names a provider of its own project or of the engine. An engine-level instance names only an engine-level provider. |
| Contracts and pins | Per submission, as today (NAM-009, NAM-010, SUB-008). A submission `address` is bound to the first project that registers it, which closes the address-squatting gap. |
| Packages | Engine-wide (`install.v1` through `management`). |
| Records, pending actions, tombstones | Engine state that carries the scopes they concern: the caller's and the target's (REC-002). |

**How membership is stated.**
- In a manifest, the new key `project: <name>` (MAN-004). One submission goes into one scope; a submission without the key makes engine-level objects, exactly as today.
- `register.v1` gains an optional `projectId`, an `expectedId` for the Project, which the CLI writes from its applied record. A recreated namesake is then never written into by accident.
- A Project document may come in the same atomic submission as its members (SUB-003). It is the one document that is not its own member, and it is authorized by `management`.
- A request through a Conduit to P registers into P, and the manifest must agree.
- A package manifest states no `project`.
- `replace.v1`'s `project` must equal the template's own.

## Names

- **Inside a Project, names are unique across kinds.** This is ruling 37 and NAM-027 carried from the submission to the registered Project. Generated names are unique per project, so the concise form's derived Pool can be called `default` in every project.
- **At the engine level, names are unique per kind, as today** (OBJ-002 unchanged for the engine scope).
- **Reserved names:** the five first-start names (`root`, `system`, `local`, `management`, `access`) in every project, and `engine` as a project name.
- **References:**
  - An unqualified `{kind, name}` resolves only in the referrer's own scope, except for the five reserved names and for reference schemas whose kind exists only at engine level (Interface, Project, a node in `placement`), which resolve at the engine.
  - Another project's object is always written `{kind, project, name}`, and an engine object `project: engine`.
  - There is no fallback and no shadowing, avoiding GitHub's "lowest level takes precedence" trap.
- **Disclosure:** a reference into another project resolves only through an audience entry covering the referrer's project or a half the registrant holds. Otherwise the answer is the one a missing object gets (ruling 52, first tier).
- **Requests (LIF-031)** resolve in the scope of the Conduit they travel through. The engine keeps no current project; the CLI supplies one from the file.
- **Tool names** are unaffected: they come from `as` (IFC-012).
- **Fallback (decision 5):** keep global names. That saves about 10 promises and about 1,000 lines, but UC8 fails and a name clash discloses that another project's object exists.

## Sharing Pools: one home, explicit audience

**The audience is a new standard argument of a Pool.** It is a list of entries, each with an engine ID:
```yaml
audience:
  - projects: [packages] | all       # with `all`, laterProjects: true|false is required (no default)
    resources: [main] | all          # with `all`, laterResources: true|false is required
    operations: [<qualified names>]  # the ceiling, compared as names (as ACC-029)
    requirements: [...]              # optional restrictions every Conduit through this entry must carry
    requestable: true|false          # may be named in another project's proposals, even with an empty ceiling
```

**Who decides.** Only the home: the Pool's home Project, or engine authority for an engine-level Pool.
- Entries are set up with the Pool and changed by `share.v1` and `unshare.v1`, which carry `expectedId` and the Pool's revision. They are operations of `project-management.v1` and `access-control.v1`; `pool-management.v1` is unchanged.
- `projects: all` needs engine authority (root).
- Shares are not transitive: RAM's rule that you "can't share a resource that's shared with you".
- Inside one engine the receiving project does not accept a share. Nothing reaches its Agents until its own managers open a Conduit, which is the consumer half.

**What an entry gives the receiving project:**
- the target half, standing, for Conduits from its own members to the covered Resources, within the ceiling and carrying the required restrictions;
- inspection of the covered Resources' name, ID, Type, description and state.

**What it never gives:**
- registering into the Pool, setting up, retiring or reconciling its Resources, retiring the Pool, or changing its audience;
- seeing other projects' Conduits, records or counts;
- sharing the Pool on;
- opening for consumers that are not its members.

**Never in an audience:**
- `system`, or any Pool that owns a node, `management`, `access` or a machine or package-source provider;
- a Pool whose Resources consume Conduits to a Project, a Pool, `management` or `access` (the confused-deputy rule, checked at share and at open);
- in a ceiling: `get-secret.v1`, reserved operations, provider operations, any management, pool, project or access operation, or any control.

**Narrowing never waits.** Removing a project or lowering a ceiling at once closes every Conduit from that project's members that lists an operation outside the new ceiling or lacks a newly required restriction. Each close leaves a tombstone naming the unshare as its cause. A receiving project may leave an entry, or close its own Conduits, at any time.

**Widening through future coverage is explicit.**
- A Resource registered into a Pool whose entry has `laterResources: true` is covered at once. The plan and the result list the receiving projects.
- A project created later falls into `laterProjects: true` entries, and its creation result lists them.

**Root is not capped by audiences** (decision 8). Its unbounded `access` holds every half. Inspection of the Pool shows root-opened cross-project Conduits as "outside the audience".

**The owner's two examples:**
- **Skills:** a Pool in a root-only project `commons`, with `projects: all, laterProjects: true, resources: all, laterResources: true`, read and list only.
- **Auth:** a Pool in `commons` whose entry admits `conduit-core` to the service Resource `linear` only. It never admits the key. Only root opens `get-secret.v1` (SEC-008).

## Lifecycle

**Create.**
- `register.v1` of a Project goes through `management`, which in practice means root. A registration naming an existing Project is rejected: nothing is ever adopted by name.
- If the name has history, the result names the earlier ID, and the CLI then requires `--recreate`.
- `set-up.v1` goes through `management` and only makes the Project ready.

**Change.** Members come and go through ordinary requests routed to P. No object ever changes its home.

**Retire.**
- `retire.v1` on a Project reports conflict while any member is live (the LIF-020 pattern).
- `retire-with-members.v1` is authorized by `management`, or by P through an unbounded Conduit; no core role carries it. In order:
  1. It refuses up front, with nothing done, while a Pool homed in P has live cross-project Conduits through its entries, unless `includeShared: true` is stated.
  2. It retires P itself first. CON-008 then closes every Conduit to P, so all authority over P ends, and no registration into P is accepted.
  3. It retires Agents, then each Pool with its Resources in LIF-021's order, continuing past a failure (ruling 42, R5).
  4. It unregisters P's templates.
  5. It voids P's pending actions.
- Every Conduit with a retired end closes (CON-008), cross-project ones included. The result enumerates them.
- Members left live by a failure fall to `management`.
- It never touches Pools homed elsewhere, engine-level objects, Interfaces, or other projects. Entries that admit P are left naming a retired ID and admit no namesake.

**After removal.**
- The name is freed (LIF-035).
- Tombstones stay, keyed by P's ID, as engine state apart from records, so purges of records do not remove them.
- A new Project with the same name gets a new ID and inherits no authority, entry, approval or tombstone binding.

**Not in v0:** moves, nesting, folders, scoping nodes or runtimes per project (UC5), and project sets.

## authority
## The rule that replaces ACC-033's single authorizer

A management request travels through exactly one Conduit, the one its tool belongs to (IFC-012). It is permitted when:
- that Conduit lists the operation;
- its target is one of the row's authorizing targets for the object;
- any bound it carries covers the object.

The record names that Conduit (REC-002, unchanged). The scopes of a project member are its Pool (Resources only), its Project, and `management`, which encloses every Project. Engine-level objects keep today's rows exactly, so an engine without Projects reads as it does today and no existing routing vector flips.

| # | Request | Object | Through a Conduit listing the operation to |
|---|---|---|---|
| R1 | register.v1 | a Project | `management` |
| R2 | register.v1, register-replacement.v1 | an engine-level object | as today (ACC-033 rows 1 to 3; a live engine Pool still needs `management` AND the Pool) |
| R3 | register.v1, register-replacement.v1 | a submission into P: instances from P's templates, Resources into P's Pools, Conduits homed in P | P or `management`. Templates of P only under `templates: all` |
| R4 | replace.v1, unregister.v1 | a template or retired instance homed in P | P (templates only under `templates: all`) or `management`. First-start objects are still refused |
| R5 | set-up, retire, inspect, reconcile | a Resource of Pool X homed in P | X, P or `management` |
| R6 | the same | a Resource of an engine-level Pool, `system`'s included | as today: the Pool; `management` once the Pool is retired |
| R7 | set-up, retire, inspect, reconcile | an Agent, Pool or standalone Conduit homed in P | P or `management` |
| R8 | the same | an Interface, or an engine-level Agent, Pool or Conduit | `management` (as today) |
| R9 | inspect / set-up, retire / retire-with-members | Project P | P or `management` / `management` / `management`, or P through an unbounded Conduit |
| R10 | retire-with-resources.v1 | Pool X homed in P | X, P or `management` |
| R11 | share.v1, unshare.v1 | an audience entry of Pool X | X's home P, or engine authority for an engine-level Pool. `projects: all` needs engine authority. A receiving project may unshare its own entry |
| R12 | open.v1, close.v1 | a Conduit | the two halves (below) |
| R13 | propose.v1 | a subject | P, for a touched project where the actor holds propose.v1 |
| R14 | approve.v1 | a pending action | approve.v1 on each touched Project, with a bound covering that project's part, or on `access`. Only through the engine's approval page |
| R15 | revoke.v1 | an actor's grants in P | P, through a Conduit whose bound contains the revoked Conduit's; or engine authority |
| R16 | install.v1, declare-lost.v1, purge.v1 | engine | `management`. Reserved operations stay root's own (ACC-032) |
| R17 | a control | an instance | the instance itself (unchanged) |
| R18 | any other operation (a turn, a read) | an instance | a Conduit to that instance. No scope ever covers use (ACC-001, ACC-003, ACC-028) |
| R19 | reconcile.v1 of an invocation | — | any target that would authorize the original request now. Each invocation stores its scope when made, so a removed member's invocations still route |

"Engine authority" means `management` for management verbs, and an unbounded Conduit to `access` for open, close, share, approve and revoke.

**Capabilities:**
- `project-management.v1` (new, 17 operations): register, register-replacement, replace, unregister, inspect, set-up, retire, reconcile, retire-with-resources, retire-with-members, open, close, share, unshare, propose, approve, revoke.
- `object-management.v1` gains `retire-with-members.v1`, for 12 operations.
- `access-control.v1` gains share, unshare, approve and revoke, for 7 operations.
- `pool-management.v1` is unchanged.

## Opening and closing: two halves

Opening Conduit C (consumer c, target t, operations O) is decided by the actor's own Conduits (ACC-016). It is dispatched only if the actor holds both halves.

**Target half**, one of:
- (a) a Conduit to `access` whose bound.v1 covers t and names O (today's ACC-006);
- (b) a Conduit to t's home Project listing open.v1, whose project bound covers t (through targets, pools or templates), names O, and admits c in `consumers`;
- (c) an audience entry of t's Pool that admits c's home project, covers t and O, and whose required restrictions C carries; together with a Conduit to c's home project listing open.v1 whose bound names that Pool in `pools` and covers O and c.

**Consumer half**, one of:
- c is a member of project Q: a Conduit to Q listing open.v1 whose bound covers c and names O;
- c is an Interface: the actor is c itself, or holds an unbounded `access`. Only root opens, or registers, a Conduit whose consumer is root;
- c is an engine-level Agent or Resource: an unbounded `access`. Where t is also engine-level, today's rule stands (any covering right, "for any consumer").

One Conduit may supply both halves; that is the usual case inside one project.

**Registering a Conduit** needs register authority in its home scope, plus:
- the consumer half, when the consumer is outside the registering scope;
- a resolvable target (an audience entry, which may be `requestable`, or the target half), when the target is outside.

This closes the name-squatting and existence-oracle routes (IFC-014 itself is unchanged).

**Closing:**
- Either half that covers C and names its operations may close it.
- The consumer may relinquish its own Conduit, unless the consumer is root.
- Root's Conduits are never closed (ACC-024).
- A Conduit to a Project can be closed only by a holder whose bound contains its bound, by engine authority, or by its consumer.

**Unchanged:**
- `get-secret.v1` is opened only by root, on its own behalf, for a Resource or Agent (SEC-008, SEC-009). Never through a Project, a share or a proposal.
- Reserved operations (ACC-032) and provider operations (PRV-004).
- A retired end gives conflict (CON-031).
- Conduits to `management` and `access` are opened only through `access`, under ACC-030.

**Confirmed by a person.** Opening a Conduit to a Project, a Pool, `management` or `access` whose consumer is an Agent completes only when the actor's person answers an engine elicitation within the request. Such a Conduit is a standing approval.

## `project-bound.v1` (new requirement; CON-028 allows it on Conduits to a Project)

Every input is required, and each is a list or `all`. Nothing is implicit (ruling 52).
- **`templates`:** templates of P, pinned to their revision when the Conduit is opened, that register.v1 and register-replacement.v1 may instantiate. Instances made from them through this same Conduit are covered as consumers and as targets, including those made later. Conduit templates are listed too. `all` also permits registering, replacing and unregistering P's templates, and only engine authority grants it.
- **`consumers`:** the consumers it may give its half for. These are P's members (by name, or by template as above), the token `members`, or qualified objects or projects elsewhere, so that a target half can serve foreign consumers only explicitly.
- **`targets`:** P's members it covers as targets of opens and of management verbs.
- **`pools`:** P's Pools, which cover their Resources now and later, and Pools shared into P, only within their entries.
- **`operations`:** operations the Conduits it opens, registers, closes or proposes may list.
- **`sameRegistration`** (boolean): a Conduit between two objects covered only through `templates` must join objects made by the same registration (the A78 pairing; decision 16).

**How each half is judged:**
- Consumer half: `consumers` and `operations`.
- Target half: targets, pools or templates; `operations`; and `consumers`.
- Management verbs (set-up, retire, inspect, reconcile, retire-with-resources, close): the object must be covered.
- register: the template must be listed.
- `retire-with-members.v1` and `includeShared` need an unbounded Conduit.

"Within", for delegation, means each dimension is a subset, `all` contains everything, and pinned revisions are equal. bound.v1 on `access` is unchanged.

## Delegation and roles

**Opening a Conduit to P** (granting authority in P) needs, as its target half:
- the actor holds open.v1 AND approve.v1 on P, so only approvers make envelopes, or the actor has engine authority;
- the operations are a subset of the actor's;
- the bound is within the actor's;
- an unbounded Conduit comes only from an unbounded holder or from the engine;
- `templates: all` comes only from engine authority;
- approve.v1 is listed only for a `person.v1` Interface consumer.

It also needs the consumer half above. An Agent of Q made an operator in P therefore needs Q's consent too. Revocation does not cascade unless `revoke.v1` is sent.

**Core role templates** (Conduit templates for `project-management.v1`):
- `project-admin`: everything except retire-with-members. Its approve.v1 is usable only by person Interfaces.
- `project-operator`, for Agents: register, register-replacement, inspect, set-up, retire, reconcile, open, close, propose.
- `project-proposer`: inspect, close, propose.
- `project-approver`: inspect, approve. This is the phone control panel.
- `project-viewer`: inspect.

## Root

- First start makes the same five objects and no Project.
- root-management lists 12 operations and root-access lists 7. FST-002 and FST-005 change, and about 21 fixtures that list root's operations are regenerated.
- `management` encloses every Project, and root's unbounded `access` holds every half. Root therefore needs no Conduit to any Project or to any Pool inside one. That removes 40 of the coordinator's 332 S1 lines.
- Root's identity powers are unchanged: ACC-032, SEC-008, ACC-024, LIF-022, and in v0 sign-ins and enrollment.
- New: only root opens or registers a Conduit whose consumer is root.

## A second person (collab over packages)

1. Root registers Interface `collab` from a `person.v1` template through `management`. It gets no access yet (ACC-023).
2. Root opens collab -> Project `packages` from `project-admin`, either unbounded or with a project bound.
3. collab can then:
   - register members from packages' templates;
   - set up, retire, inspect and reconcile packages members;
   - open Conduits among them, and from them into Pools shared to packages within the ceilings;
   - open Conduits for herself (consumer half by self);
   - approve packages' part of proposals;
   - share packages' Pools to named projects;
   - give narrower roles to packages Agents (confirmed by her as a person);
   - revoke what she granted;
   - see packages' inventory only.
4. collab cannot:
   - act in another project;
   - make another Interface a consumer;
   - share to `all`;
   - open `get-secret.v1`;
   - use reserved operations or install;
   - close root's Conduits;
   - retire the Project.

**Prerequisites:** DL13 (a token for an Interface other than root). The shell's `issue_client_token` accepts any instance, but only root is issued one (main.rs:157, :212). Before collab is untrusted, the store must also be unreadable by runs (DL16).

## Revocation and closing between co-managers

- `revoke.v1 {actor: X}` on P is a narrowing and immediate. It closes:
  - X's Conduits to P;
  - every Conduit in P's scope or touching P whose `openedBy` chain leads to X through authority X granted.
- It lists the objects that X registered, which are not retired.
- Equal co-managers can still close each other's member Conduits (ACC-012, kept). A narrower co-manager cannot close a wider one's grant (decision 15).

## Approvals as pending actions

**Authority, not a classifier, decides what waits.**
- An actor acts at once within its Conduits. An envelope is a bounded Conduit that only an approver can open, confirmed by a person.
- Beyond its envelope an actor may only propose.
- The engine needs no general judgement of "widening", except to refuse narrowings in a proposal unless they are paired as `replaces`.

**`propose.v1` on P.** The subject is an ordered list of requests drawn from a closed list: register, register-replacement, set-up, open, share and controls. Never reserved operations, install, or anything that would open a `get-secret.v1` Conduit, declared Conduits included. Each request carries `expectedId`, template revisions and document digests. A proposal also carries `replaces`, a quoted justification, and `expiresIn` up to an engine maximum (a new DL; 24 hours suggested, 7 days at most).

The engine refuses a proposal when:
- the proposer could make it all itself;
- it names anything the proposer may not inspect, other than through a `requestable` entry;
- it holds an unpaired narrowing.

Otherwise it records the pending action:
- `id` and `kind: approval`;
- `scopes`;
- the proposer and its caller chain;
- the subject and its digest;
- **evidence written by the engine:** reach before and after for each consumer, tool-less Conduits included; every declared Conduit a setup would open; the template revision, program, `remote`, runtime and placement of every instance that would run; the entries relied on;
- the evidence digest;
- `needs`: which half each project must still supply;
- the expiry and the outcome.

**Routed** as an elicitation to `person.v1` Interfaces holding approve.v1 on a touched project whose bound covers that project's part.
- It is never raised at an Agent.
- Root sees every action in inventories, but is prompted only where no other approver covers a part or engine authority is needed.
- Each approver sees both ends' qualified names, Types and operations, and nothing else of the other project.

**Approve** happens only on the engine's page. approve.v1 is not a tool (an IFC-012 exception). The answer carries the action ID, the evidence digest, and approve or reject.

**On the last approval needed**, the engine:
1. recomputes the evidence, and reports conflict if its digest differs from the one the approvers saw;
2. rechecks every `expectedId` and template revision;
3. checks that the proposer still holds propose.v1 and that the approvers' Conduits stand;
4. dispatches the requests in order, inside the approving invocation (ACC-019). Each request is decided under the proposer's own Conduits plus the approvers' approve Conduits for the parts each covers. An approve Conduit with bound B permits, inside an approved action only, what a Conduit to P with bound B would permit. Dispatch stops at the first request that does not succeed, with no rollback.

**Records** name the proposer as the actor, with `approvedBy`.

**Rules that hold for every approval:**
- No self-approval: an approver is not the proposer, and its approve Conduit was not opened by the proposer, checked transitively over `openedBy`.
- Any one reject ends the action, and the proposer may withdraw it.
- An action is voided when its proposer retires or loses propose.v1, or a scope retires.
- A pending action reserves nothing. A narrowing is never held back, and a drifted subject gives conflict.
- Agents never hold approve.v1 in v0. This is a stated exception to preservation test 5.

**Sign-ins and enrollment** use the same mechanism.
- The routing promise is written now: to the person approvers of the instance's home project, plus root. Its behaviour stays root-only in v0 (ruling 49).
- A sign-in action shows the scope, `registeredBy`, the program digest and the expected account. In v0 it is raised only for instances whose template root registered or approved.
- Enrollment stays engine-level and root's (ruling 50).
- Peer Interfaces are marked foreign. A Conduit from one to a Project, `management` or `access`, or one listing approve.v1, needs root.

## examples
Notation:
- `core#x` stands for `github.com/shrav-k/conduit/core#x`.
- Every canonical document carries `apiVersion: github.com/shrav-k/conduit/v0`, omitted below.
- Operation names are shortened (`fs#file.read.v1`); the real files qualify them fully (ruling 25).
- The concise syntax belongs to the parallel short-form investigation. Only its content is fixed here.
- All line counts are paper counts. Ruling 45 accepts only a run.

## 1. S1 as one project

**Concise form.** The ruling-53 package file, unchanged. `project: s1` now names a real Project.
```yaml
conduit: v0
project: s1
packages: {...}                          # written by conduit install
resources:
  shared: {type: example.org/s1/filesystem#local-directory.v1, config: {root: <lab>/shared}}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach: {project: {to: shared, ops: [fs#file.read.v1, fs#directory.list.v1]}}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    reach: {project: {to: shared, ops: [fs#file.read.v1, fs#directory.list.v1, fs#file.write.v1]}}
```

**Plan (abridged).** The operator is root.
```
+ Project s1                      engine-level; through management (R1)
+ Pool s1/default                 derived: owns every Resource that names no Pool
+ Resource s1/shared; Agents s1/agent-a, s1/agent-b      through management, which encloses s1 (R3)
+ Conduit agent-a--project, agent-b--project   as project
+ Conduit agent-a--turn, agent-b--turn         root -> Agent [turn.v1]   derived; homed in s1
authority over s1: root (management and access enclose every project): no operator Conduit
approvers of s1: root. Root's own widenings are never pending.     audience: none
```

**Canonical form.** What changes in the coordinator's T1 submission:
```yaml
# conduit.yaml (manifest)
project: s1                       # NEW key (MAN-004): every document below is homed in s1
packages: {...}
---
kind: Project                     # NEW family; the one document that is not its own member
metadata: {name: s1}
from: core#project
---
kind: Pool
metadata: {name: workspace}
from: workspace-pool              # a template homed in s1
---
kind: Resource
metadata: {name: shared}
from: directory
owner: workspace
arguments: {root: <lab>/shared}
# agent-a, agent-b, their declared `project` Conduits and the turn Conduits root -> agent-a/-b: as today.
# REMOVED: template pool-admin (12 lines), Conduit root-workspace (8), admin2.yaml (16),
#          the two fix-up manifests (4). Root's management encloses s1 and its Pool.
```

**Later tasks:**
- T2: `history` is registered into `workspace` through `management` alone. Today that needs `management` and the Pool.
- T4: root closes B's write and opens A's. Nothing is pending.
- T5 and T6: re-points are root's own close and open.
- T7: the revoke leaves tombstones keyed by (s1's ID, agent-a, as `project`) and by (agent-a's ID, shared's ID, capability). The restore names them explicitly.

| | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
|---|---|---|---|---|---|---|---|---|
| A70 baseline | 33 | 16 | 0 | 8 | 3 | 2 | 2 | 64 |
| Recommended, concise | 16 | 6 | 0 | 2 | 1 | 1 | 1 | 27 |
| Recommended, canonical (estimate) | 139 | 56 | 0 | 2 | 63 | 30 | 13 | 303 |
| Coordinator's run today | 154 | 73 | 0 | 2 | 62 | 29 | 12 | 332 |

Canonical arithmetic: T1 is 154 − 22 + 6 (the Project document) + 1 (`project:`); T2 is 73 − 18 + 1; T5, T6 and T7 each add 1 for `project:`. Nothing is pending anywhere in S1, because root makes every widening.

## 2. The owner's real use: commons, conduit-core and packages

**engine.yaml** (no `project:`; root through `management`):
```yaml
kind: Project, metadata: {name: commons},      from: core#project   # root-only home of shared Pools
kind: Project, metadata: {name: conduit-core}, from: core#project
kind: Project, metadata: {name: packages},     from: core#project
---
kind: Interface
metadata: {name: collab}
from: person                       # engine template, type core#person.v1 (new): a person, who may approve
```

**commons** (`project: commons`; nobody but root holds authority here). Canonical for one Pool:
```yaml
kind: Pool
metadata: {name: auth}
from: shared-pool                  # commons template, type core#pool.v1
audience:
  - projects: [conduit-core]
    resources: [linear]            # the service only, never linear-key or onepassword
    operations: [example.org/linear#list-issues.v1, example.org/linear#get-issue.v1,
                 example.org/linear#create-issue.v1, example.org/linear#update-issue.v1]
    requestable: false
  - projects: [packages]
    resources: [linear]
    operations: []                 # packages cannot open; it may only propose
    requestable: true
---
kind: Resource, metadata: {name: onepassword}, from: op-store, owner: auth   # secret-store provider, vault conduit-test
kind: Resource, metadata: {name: linear-key},  from: op-item,  owner: auth, arguments: {item: linear-api}
kind: Resource, metadata: {name: linear},      from: linear-mcp, owner: auth
# linear-mcp declares key -> linear-key [core#get-secret.v1]; it opens only when root sends linear's
# setup (SEC-008, ACC-005). Only root can, since commons has no other authority.
```

The rest of commons, concise:
```yaml
pools:
  skills:
    audience: [{projects: all, laterProjects: true, resources: all, laterResources: true,
                operations: [fs#file.read.v1, fs#directory.list.v1]}]       # projects: all needs root
    resources: {claude-skills: {...root: ~/.claude/skills}, codex-skills: {...root: ~/.codex/skills}}
  repo:
    audience:
      - {projects: [conduit-core, packages], resources: [main],  operations: [fs#file.read.v1, fs#directory.list.v1]}
      - {projects: [conduit-core, packages], resources: [queue], operations: [history#post.v1, history#read.v1],
         requirements: [<a filter, if the history Type offers one, so each project reads only its own entries>]}
    resources: {main: {...read-only checkout}, queue: {...order history}}
```

**conduit-core** (`project: conduit-core`), concise:
```yaml
agents:
  coordinator:                     # claude-code; template homed here, registered by root; runtime and placement fixed by it
    reach:
      queue:  {to: commons:queue, ops: [history#post.v1, history#read.v1]}           # target half: repo's entry
      skills: {to: commons:claude-skills, ops: [fs#file.read.v1, fs#directory.list.v1]}
      linear: {to: commons:linear, ops: [list-issues, get-issue, create-issue, update-issue]}   # auth's entry
  triage:
    reach: {linear: {to: commons:linear, ops: [list-issues, get-issue]}}   # S2: one login, two restrictions
managers:
  coordinator:                     # compiles to Conduit coordinator -> Project conduit-core, homed in conduit-core
    ops: [open, close]
    bound: {templates: [], consumers: [coordinator], targets: [], pools: [],
            operations: [example.org/conduit/agent-codex#turn.v1], sameRegistration: false}
    # conduit-core's consumer half: its coordinator may take turns other projects allow it, and nothing else
```

**packages** (`project: packages`). Canonical for the coordinator's authority. It is homed in packages because its target is the Project.
```yaml
kind: Conduit
metadata: {name: coordinator-in-packages}
spec:
  capability: core#project-management.v1
  operations: [core#register.v1, core#set-up.v1, core#retire.v1, core#inspect.v1, core#reconcile.v1,
               core#open.v1, core#close.v1, core#propose.v1]
  requirements:
    - requirement: core#project-bound.v1
      arguments:
        templates: [pkg-worker, pkg-tree, tree-rw, main-ro, queue-rw, skills-ro, coordinator-turn]  # revisions pinned at open
        consumers: [{template: pkg-worker}, {kind: Agent, project: conduit-core, name: coordinator}]
        targets:   [{template: pkg-worker}, {template: pkg-tree}]
        pools:     [{kind: Pool, name: work}, {kind: Pool, project: commons, name: repo},
                    {kind: Pool, project: commons, name: skills}]
        operations: [fs#file.read.v1, fs#file.write.v1, fs#directory.list.v1,
                     history#post.v1, history#read.v1, example.org/conduit/agent-codex#turn.v1]
        sameRegistration: true
---
kind: Conduit
from: coordinator-in-packages
consumer: {kind: Agent, project: conduit-core, name: coordinator}
target: {kind: Project, name: packages}
as: packages
# Opened by root: the target half needs a packages approver; the consumer half needs conduit-core,
# which collab lacks. The open is person-confirmed (the consumer is an Agent).
---
kind: Conduit
metadata: {name: collab-admin}
from: core#project-admin           # no requirements: unbounded on packages, templates: all included
consumer: {kind: Interface, name: collab}
target: {kind: Project, name: packages}
as: packages
# Opened by root; an Interface consumer's half is the Interface itself or root.
```
The rest of packages, concise: `pools: {work: {}}`; templates `pkg-worker` (codex; placement fixed by root), `pkg-tree` (local-directory, whose `root` input carries the pattern `^<lab>/trees/[a-z0-9-]+$`), and the Conduit templates listed above; Agent `pkg-lead` with `managers: {pkg-lead: {role: project-proposer, bound: {...workers and trees only...}}}`, opened by collab.

**Order 032.** The coordinator does all of this with no approval:
1. One registration through `packages.register.v1`: `pkg-worker-032`, `pkg-tree-032` (in Pool `work`), `worker-032--tree` (tree-rw), `worker-032--main`, `--queue` and `--skills`, and `coordinator--worker-032` (coordinator-turn). All of them are homed in packages. The turn Conduit's consumer half comes from coordinator's conduit-core grant.
2. It opens `worker-032 -> tree-032 [read, list, write]`. The same project and the same registration satisfy `sameRegistration`.
3. It opens `worker-032 -> commons:main [read, list]`, `-> commons:queue`, and `-> commons:codex-skills`. The target half comes from the audience entries; the consumer half from packages.
4. It opens `coordinator -> worker-032 [turn]`. The consumer half is conduit-core's; the target half is packages'.
5. At merge it closes them and retires the worker and the tree. Those are narrowings and are immediate.

**Refused, each with the first-tier answer only:**
- worker-032 reaching tree-033: `sameRegistration`.
- Registering from any template not listed, registering or replacing any template, or registering from a later revision of `pkg-worker` until the grant is renewed.
- Any packages member reaching `linear`: the entry's ceiling is empty.
- A worker writing `main`: above the ceiling.
- Any project actor opening `get-secret.v1`.
- collab acting in conduit-core or commons.
- collab opening a Conduit for `ci-hook` (an Interface not herself).
- collab making the coordinator an operator alone: there is no conduit-core half.
- collab approving commons' part.
- collab retiring Project packages.
- Anyone but root opening a Conduit whose consumer is root.

**An agent proposes.** pkg-lead wants `pkg-worker-032` to read Linear issue titles:
```yaml
operation: core#propose.v1
target: {kind: Project, name: packages}
input:
  subject:
    - {operation: core#register.v1, documents: [sha256:… (Conduit template linear-read: linear list/get),
        sha256:… (Conduit worker-032--linear from linear-read; consumer pkg-worker-032;
                  target {kind: Resource, project: commons, name: linear}; as: linear)]}
    - {operation: core#open.v1, conduit: {kind: Conduit, name: worker-032--linear}}
  justification: "Order 032 needs issue titles for release notes"
  expiresIn: PT24H
# -> succeeded: action pa-01JE…, outcome pending, digest sha256:…, evidence digest sha256:…
#    needs: [{project: packages, half: consumer and registration}, {project: commons, half: target}]
#    evidence: "pkg-worker-032 (packages) would gain linear.list-issues, linear.get-issue on
#               commons/linear (Linear MCP; reached through auth's requestable entry; ceiling for packages: none)"
```
- The engine raises the action at collab, for packages, and at root. Root is prompted because commons has no other approver.
- Each person answers on the engine's page. On the last answer the engine rechecks the evidence digest, the `expectedId`s and the revisions, then dispatches:
  - the register under collab's approve Conduit;
  - the open with its consumer half from collab's approve and its target half from root's.
- The record names actor pkg-lead, `approvedBy` [collab, root].
- **Refused:**
  - pkg-lead approving: it is an Agent, and there is no tool for it;
  - collab approving commons' part;
  - a stale digest;
  - approval after `pkg-worker-032` has retired (voided).
- **Root's better answer:** reject, and give packages' entry in auth a ceiling of `[list-issues, get-issue]` (UC10). That is root's own immediate act, after which packages' managers open it without asking.

**Retiring packages.** Root sends `retire-with-members.v1` through `management`.
- Project packages is retired first. That closes collab-admin, coordinator-in-packages and pkg-lead's grant, and refuses any further registration into it.
- Then the workers, which closes their Conduits into commons and the coordinator's turns; then the trees with Pool `work`; then packages' templates.
- Its pending actions are voided.
- commons, conduit-core, collab and the coordinator are untouched. Entries naming packages now name a retired ID and admit no namesake.

**Note.** In the owner's practice the A78 coordinator is the admin session, which is root. Then coordinator-in-packages is unnecessary. The Agent variant above shows delegation.

## comparison
Sources: the four dossiers, their twelve skeptic reports, and checks against the repo.
- Promise totals are out of about 455 (two cost skeptics' grep of IDs; a third counted 470 unique).
- "Rename" means keeping the word "Project" for the new object and rewording today's meaning.

| | P1: sixth family, chain of scopes | P2: scope as an engine-served Resource | P3: flat Projects, audiences (nesting tested and declined) | P4: Home Projects (Pool Type, minimal) | Recommended (composite, with fixes) |
|---|---|---|---|---|---|
| **Safety, as written** | 2 critical (approval is a tool; a project is not a host boundary), 8 high, 9 medium, 4 low | 1 critical (run a member, become root), 8 high (private Pools bridged; deputies; consumer half skipped through `access` and for Interfaces; replace defeats templates), 12 medium, 2 low | 1 fatal as written (delegated register, then root), 3 high (Interface consumers exempt; replace plus setup as confused deputy; approvals not human), about 9 medium or low | 1 fatal (the same, for every design), 7 high (home granted by any mention; set-up approval swappable; unbounded right smuggled; deputy laundering; Agent-made Interfaces; get-secret laundering; bare names across projects) | Every critical and high finding is closed by a rule (see recommendation). What remains is stated, not fixed: in v0 a project is mediated and needs DL13, a separate OS user and a confining runtime before an untrusted person; "a person" is a client obligation (ruling 49) |
| **Expressiveness** | 10 unacknowledged majors: approved controls cannot be executed; no 032/033 pairing; no filters in shares; no caller project for implementations; `people` home cuts off reach; re-point outage; `widens` has no direction; whole-Pool shares; Agents not shareable; revocation does not stop a delegate. UC5 no; UC8 yes | 8 majors: templates bypassed; example contradicts itself (bridging); no consumer dimension; revocation not one act; approval only for opens; cross-project proposals impossible; no filters; no node scope | 8 majors: approve-only role approves nothing; proposals cannot reach unshared targets; consumer exemption; bound undefined for management verbs; no route for a project admin to delegate; coordinator needs its own project's authority; no filters; placement | Blocking F1 plus majors: shares and homes attach to actors, so managers bridge projects; unshare is N requests; placement. UC8 (names) and the template limit not served | UC1, UC2, UC3, UC4, UC7, UC8, UC9 and UC10 served, including the template cap, the pairing, cross-project proposals through `requestable`, required restrictions on shares, and revoke. UC5 (machine per project) and UC6 (peer per project) deferred to v1. S8 partial: the caller's project is passed, enforcement comes in v1 |
| **Promises touched** | Claimed 75 to 80; counted about 105 (new word) or about 147 (rename) | Claimed 36 to 42; counted 46 to 56 or 93 to 103 | Claimed 66; counted 85 to 90, or about 130 | Claimed 22; counted about 30 (6.5%) | About 100 in substance (about 40 new, of which about 5 are owed to rulings 49 to 51; about 60 changed), plus 45 to 55 worded if renamed |
| **Existing vectors** | 15 to 20 re-expected by hand; about 230 first-start snapshots; store break | 14 steps flip in 5 cases | 13 steps flip in 4 cases | 0 flip | 0 routing outcomes flip (`management` encloses Projects only). About 21 first-start fixtures regenerated (root's operations); stated clean break of lab stores |
| **Rust** | 8,500 to 15,000 | core +2,000 to 3,500, shell and CLI +1,000 to 1,500 | core 4,500 to 7,000; total about 6,000 to 10,000 | core 1,600 to 2,800 | core 5,000 to 8,000; shell 1,200 to 2,500 (DL13, approval page, confirmations); CLI 800 to 1,500; loader and runner 300 to 800. About 7,500 to 13,000 |
| **Work orders** | about 1 increment or more | 5 to 8 | 10 to 13 | 3 to 4, after 3 prerequisites | 11 to 14, in 4 stages, after the shared prerequisites |
| **Concepts** (S1 author / multi-project author) | 0 / +13 to 15 terms | 0 / +12 to 15 | +1 −1 / +6 | 0 / about 8; Pool now has two meanings | +1 (Project) −1 (root's Pool Conduit) / about 8: home, audience, two halves, project bound, role, propose, approve, revoke |
| **Fit with the short-form question** | Good: `project:` names the object, scoped names; heavy | Good: `managers:`, `audience:`, `to: project X`, `default` Pool | Good, the same keys; "authority not a classifier" suits plan and apply | Weak: replaces the derived Pool one for one, but global names make short local names unsafe | Good: one `project:` line; `managers:`, `approvers:` and `audience:` compile to Conduits and entries; scoped names make `default` and local names safe; apply sends widenings beyond the author's envelope as one proposal (ruling 53, decision 2) |
| **Main thing kept from it** | Explicit propose and approve; project bound with five dimensions; per-project tombstones | Role templates; the audience as a standing target half; "no member list on the Project" | The skeleton: family, audiences with explicit `future`, two halves, flat with nesting declined, authority deciding what waits | Engine-without-Projects unchanged; no vector flips; approval equals authority | — |

## decisions
Each decision gives the options and a recommendation. Decisions 1 to 6 should be ruled together with the short-form ruling (ruling 53, decision 1), because both redefine "project", names and references.

1. **Whether and when to adopt Project.**
   - (a) Adopt the composite model, staged A to D, decided with the short form. **Recommended.**
   - (b) Adopt P4 (about 30 promises) as v0 and add the rest later. This writes tombstones, inventory and names twice.
   - (c) Defer until after the short form lands.
   - Note that the change is larger than the short-form move (about 100 promises against 50 to 80), not equal to it.

2. **Form.**
   - (a) A sixth family, `kind: Project`. **Recommended:** explicit (ruling 52), and no second meaning for "Pool".
   - (b) A Pool of `core#project.v1` (P4). About 10 to 12 fewer promises, but "Pool" then means two things and `retire-with-resources.v1` retires Agents.
   - (c) A Resource owned by `system` (P2). Rejected: every bound naming `system` would cover every Project.

3. **The word (ruling 42, one meaning per word).**
   - (a) "Project" for the registered object, with today's meaning folded into "submission", which definitions.md already defines almost identically, and "a package is a published project" reworded. About 45 to 55 promises reworded, plus runner and loader identifiers. **Recommended:** it is the owner's word and the concise heading.
   - (b) Another word for the object (workspace, space). No rename cost, but the concise `project:` key then means something else.

4. **How root gets authority over projects.**
   - (a) `management` and an unbounded `access` enclose every Project. Engine-level Pools, `system` included, keep today's two keys. **Recommended:** root needs no Conduit per project, no existing vector flips, and no existing grant silently widens.
   - (b) `management` also encloses engine-level Pools (P1 to P3). This removes root's Pool Conduits everywhere, but flips 13 to 14 steps and widens co-admins over `system`'s providers.
   - (c) A single authorizer with a derived root Conduit to each Project (P4).

5. **Names.**
   - (a) Unique within a Project across kinds, with qualified cross-project references, the five first-start names reserved, and no fallback. **Recommended:** needed for UC8 and concurrent work orders; stops existence leaking through name clashes.
   - (b) Global names (P4). Saves about 10 promises, 800 to 1,300 core lines and about 15 cases.

6. **Interfaces.**
   - (a) Engine-level principals that only root creates. The consumer half for an Interface is held by that Interface itself or by root. A new core Type `person.v1` marks persons. **Recommended.**
   - (b) Interfaces homed in projects (P1). Their projects' managers could then retire them.

7. **Sharing Pools.**
   - (a) Audience entries on the Pool, from Pool to project, with listed Resources, explicit `laterProjects` and `laterResources`, a ceiling, required restrictions and `requestable`. The home decides; `projects: all` needs root; no acceptance step inside one engine. **Recommended.**
   - (b) Shares as rights given to actors (P4). These bridge projects.
   - (c) Receiving acceptance as a pending action (P1).
   - Also confirm: project-homed Pools may be shared (Shared VPC style). **Recommended: yes.** Cross-cutting Pools belong in a root-only `commons` project.

8. **Does an audience cap root?**
   - (a) No; inspection marks root's Conduits as "outside the audience". **Recommended.**
   - (b) Yes (P2): the audience becomes the complete truth, and root's one-off exceptions are lost.

9. **Conduits across projects.**
   - (a) Two halves for every open and every Conduit registration, with each half judged on its own bound dimensions. A bridge exists only where a target-half bound names foreign consumers explicitly. **Recommended.**
   - (b) One opener covering both halves (P2). This silently joins projects.

10. **Approvals.** Note that this direction is in the computed task, not in the owner's relayed words or the owner-direction record.
    - (a) Adopt at all? **Recommended: yes**, as the answer to ruling 53's open decision 2, staged last.
    - (b) Mechanism: explicit `propose.v1` and `approve.v1` with a pending action. **Recommended**, over the register/open split of P2 and P4, which cannot carry setups or controls and lets proposals be inferred from half-applied state.
    - (c) Approval only as a person's answer on an engine page, never a tool. **Recommended.**
    - (d) No Agent holds approve.v1 in v0, a stated exception to preservation test 5. **Recommended.**
    - (e) Bounded Conduits are standing approvals, opened only by approvers and confirmed by a person when the consumer is an Agent. Root's own grants to Agents are confirmed too. **Recommended.**
    - (f) Expiry: a new DL, 24 hours by default and 7 days at most. **Recommended.**
    - (g) One approver per touched project. **Recommended.** A two-person rule and quorums wait for v1.

11. **Isolation in v0.**
    - (a) State that projects are mediated. Add the template cap: instances registered through P come only from P's templates, which fix runtime and placement; templates of P are registered only by root or a root-granted `templates: all`; implementation items come only from installed packages unless the submission goes through `management`. **Recommended.**
    - (b) Also make three things preconditions for any second, untrusted person: a DL13 token, a store that runs cannot read (a separate OS user), and a confining runtime for project-authorized runs. **Recommended.**

12. **Secrets.**
    - (a) They stay root's. The auth Pool is homed in a root-only project and only its service Resource is shared. Templates behind `get-secret.v1` consumers are replaced only by engine authority. Anything that would open `get-secret.v1` is never proposable. **Recommended.**
    - (b) Project approvers may approve secret access. Rejected: it changes the secret guarantee in spec/README.md.

13. **Retiring a Pool or Project that others use through shares.**
    - (a) Refuse unless `includeShared: true` is stated; then act at once and list everything that closes. **Recommended.**
    - (b) Proceed and list (P2, P3).
    - (c) Refuse until unshared.

14. **Revocation.**
    - (a) Store `openedBy` and `registeredBy`, and add `revoke.v1`, a narrowing that closes an actor's grants in a project transitively. **Recommended.**
    - (b) The CLI composes revocation from the inventory.
    - (c) Revocation cascades by default (against ACC-030).

15. **Closing between co-managers.**
    - (a) A Conduit to P is closed only by a holder whose bound contains its bound, by engine authority, or by its consumer. Member Conduits may be closed by any covering half. **Recommended.**
    - (b) Keep ACC-012 as it is everywhere.
    - (c) Only the opener may close.

16. **Pairing for work orders.**
    - (a) A `sameRegistration` flag on the project bound, so the engine keeps worker-032 out of tree-033. **Recommended:** A78 is the real use.
    - (b) Leave it to the coordinator's code.

17. **Records per scope.**
    - (a) Records are readable by inspectors of the caller's scope and of the target's home. The other side appears by qualified name only. Inputs a Type records are visible only to the caller's project and root. Root sees everything. **Recommended.**
    - (b) Root only, as today in practice.

18. **Tombstones.**
    - (a) Keyed by project ID and by (consumer ID, target ID, capability). They outlive the Project, a namesake inherits nothing, and recreating a name needs an explicit `--recreate`. A tombstone left by a non-approver does not bind an approver's restore. **Recommended.**
    - (b) Keyed by project name, which leaks to namesakes.

19. **Not in v0.** Scoping nodes and runtimes per project (UC5), exposing a project to a peer engine (UC6), project sets, and nesting. **Recommended:** keep projects flat now. A team boundary is a separate engine (ruling 51). Revisit project sets when a second team exists.

20. **Widening through configuration** (`accountServices`, a directory root moved up).
    - (a) In v0, only the template cap. In v1, Types mark the direction in which an input widens. **Recommended.**
    - (b) Type marks now, as new versions of every package.

21. **Order of work.**
    - (a) Write ruling 53's five additions once, in their per-project form, in stage A after the short-form ruling. **Recommended.**
    - (b) Land the package first and add Projects later, writing tombstones, inventory and `expectedId` twice.

## plan
## 0. Before any promise is written

1. **Owner rulings.** Put decisions 1 to 6 (adopt, form, word, enclosure, names, Interfaces) to the owner together with the short-form ruling (ruling 53, decision 1). Decisions 7 to 21 can follow per stage.
2. **Records**, written by the admin session; this run modified nothing:
   - docs/proposals/projects/README.md (this synthesis);
   - docs/proposals/projects/research/ (the four designs, the twelve skeptic reports, the four inventories);
   - the decision list added to to-review-with-the-owner.md.
   Say plainly that the approvals clause is not in the owner's words.
3. **Do not interrupt the current first-build increment.** The Project batches start only after the short-form ruling.

## 1. Prerequisites (owed whatever is decided)

- **P-a.** The pending-action batch of rulings 49 to 51: shape, routing and elicitation. About 5 promises.
- **P-b.** A DL13 rule for issuing a token to an Interface other than root, and the shell path that does it. `issue_client_token` exists in shell/src/lib.rs:297, but only root is issued a token (main.rs:157, :212).
- **P-c.** LIF-021 (`retire-with-resources.v1`) implemented in the reference model. Only its routing exists today (access.rs:280).
- **P-d.** Runs that cannot read the engine's store: a separate OS user (DL16). This is required before any untrusted second person.

## 2. Spec batches

Each batch is written by the admin session and is followed by Codex work orders.

**Stage A: Project, membership, names and memory.** About 45 promises. Written together with ruling 53's five additions, in their per-project form.
- New:
  - PRJ-001: the family, Type and template; Project Types are core-only.
  - PRJ-002: the home rule, kind by kind.
  - PRJ-003: the closed list of engine-only objects.
  - PRJ-004: membership permits nothing.
  - PRJ-005: one submission, one scope; the `projectId` check.
  - PRJ-006: retire and `retire-with-members.v1`, Project first.
  - PRJ-007: removal, the freed name, the namesake rule.
  - PRJ-008: the per-project inventory.
  - PRJ-009: per-project tombstones.
  - PRJ-010: names within a Project across kinds, reserved names, qualified references, resolution with no fallback.
  - The stated clean break of stores.
- Changed: OBJ-001, OBJ-002, OBJ-003, OBJ-006, OBJ-013, OBJ-020, OBJ-021, OBJ-025; NAM-003, NAM-022, NAM-027; INS-001, INS-024, INS-025; TPL-001, TPL-002, TPL-019; LIF-001, LIF-013, LIF-031, LIF-035, LIF-037; MAN-004, MAN-015; SUB-005, SUB-008, SUB-009; POOL-004; CON-008; REC-002; CTR-004; FST-003; definitions.md (family, home, engine scope, submission).
- Plus the wording pass, if decision 3(a) is taken.

**Stage B: authority.** About 30 promises.
- ACC-033 rewritten as rows R1 to R19, with `management` enclosing Projects only.
- PRJ: Conduits to Projects; `project-bound.v1` and its per-half semantics; delegation (only approvers make envelopes); `revoke.v1` and `openedBy` / `registeredBy`; the root-consumer rule; the template cap (P-only templates, runtime and placement fixed by the template, implementations only from packages).
- Two halves: changes to ACC-005, ACC-006, ACC-012, ACC-016, ACC-022, ACC-023, ACC-028, ACC-029, ACC-030, CON-001, CON-012, CON-024, CON-028, OPS-014, OPS-019, OPS-023, OPS-029, POOL-015, PRV-006, REV-010.
- Disclosure: cross-project references resolve only through a half or an audience entry; refusals name only targets the caller may inspect.
- Core package: `project.v1`, `person.v1`, `project-management.v1`, five role templates, `project-bound.v1`, a project reference schema plus an optional `project` in the 12 reference schemas, and new operations in root-management and root-access.
- README: rewrite "What access does not narrow", and add the v0 mediation statement.

**Stage C: audiences.** About 10 promises.
- The audience standard argument, `share.v1` and `unshare.v1`, what is never shareable, the narrowing cascade with tombstones, `includeShared`, `requestable`, required restrictions, and explicit later coverage in plans and results.
- The caller's project passed in implementation keys (IFC-006).

**Stage D: propose and approve,** on top of P-a. About 15 promises.
- PND: the shape; `propose.v1` and its closed list; the evidence and its digest; routing to person approvers with root prompted only when needed; approve.v1 only through the engine page (an IFC-012 exception); dispatch in the approving invocation under joint approvals (ACC-019); no self-approval, checked transitively; expiry (a new DL), voiding and withdrawal; `replaces`; person-confirmed envelopes; sign-in routing written now but root-only in v0; foreign Interfaces.
- Changed: ACC-001, ACC-019, IFC-007, IFC-016, SEC-008 (observable), REC-002.

## 3. Work orders for Codex (about 11 to 14)

| # | Work order | Rough size |
|---|---|---|
| WO-1 | Store format 2 with a clean break; a scope field on every object; the Project family; the manifest key; `projectId` (loader, core, shell) | |
| WO-2 | Scoped names: per-scope name index, qualified references, resolution in the Conduit's scope (core: 73 name lookups; CLI `named()`) | 800 to 1,300 lines |
| WO-3 | Inventory, tombstones and `expectedId` on open and close, per project (ruling 53's additions) | |
| WO-4 | `retire-with-members.v1`, Project first, with `includeShared` (after P-c) | |
| WO-5 | Routing by scope in access.rs (rows R1 to R19); Conduits to Projects; `project-bound.v1`; delegation; `revoke.v1` | |
| WO-6 | Two halves, the root-consumer rule, consumer half at registration, disclosure answers | |
| WO-7 | Template cap: P-only templates, runtime and placement fixed by the template, implementations through `management` only | |
| WO-8 | Audiences, `share.v1` and `unshare.v1`, close cascades, `requestable`, required restrictions | |
| WO-9 | Pending actions and `propose.v1`, with evidence before and after (shared with P-a) | 600 to 900 lines for evidence alone |
| WO-10 | Approval page and elicitation; `person.v1`; envelope confirmation; joint-approval dispatch; records `approvedBy` | |
| WO-11 | DL13 token for a second Interface (P-b) | |
| WO-12 | Concise CLI: `project:`, `managers:`, `approvers:` and `audience:` keys; plan sections for authority, audiences and pending actions; apply sending narrowings directly and widenings beyond the envelope as one proposal | |
| WO-13 and WO-14 | Conformance vectors for stages B to D, if they do not fit inside the orders above | |

Past orders ran about 1,100 to 1,600 Rust lines each. All spike or experiment code is Rust.

## 4. Tests and acceptance

- **Regression gate after WO-5 and WO-6.** All 367 existing conformance cases pass unchanged. The only exception is the regenerated first-start fixtures that list root's operations, about 21 files. Any routing outcome that flips is a defect, because an engine without Projects must read as today.
- **New conformance cases: about 100 to 130**, mostly generated (about 100k to 170k fixture lines). They cover:
  - homes and names, including UC8 and the reserved names;
  - every row from R1 to R19;
  - every dimension of the bound and its effect on each verb;
  - two halves;
  - audiences and their cascades;
  - retirement order and `includeShared`;
  - tombstones by project ID;
  - every PND rule.
- **One negative case for each skeptic counterexample.** These must be refused:
  - a non-root actor opening or registering a Conduit whose consumer is root;
  - an open for an Interface by anyone other than that Interface or root;
  - a bounded `access` holder pushing a Conduit into another project's Agent;
  - replacing a pinned template;
  - instances covered by template from another Conduit;
  - a registration through P that names `runtime` or `placement`, or carries `command` or `remote` items;
  - an existence oracle: the same answer whether the object exists or not;
  - approve.v1 presented as a tool;
  - a puppet chain of approvers;
  - evidence drift after a template is replaced;
  - a subject that would open a declared `get-secret.v1` Conduit;
  - a control in a ceiling;
  - sharing a Pool whose Resource holds a Conduit to a Project;
  - registering into a Project during its retirement;
  - `revoke.v1` leaving the delegate's turns open;
  - and others.
- **Shell tests:** a token for a second Interface; the approval page; confirmation of an envelope for an Agent; after P-d, that a run cannot read client-tokens/.
- **Runs (ruling 45 accepts only a run):**
  - **S1 rerun.** Concise target 27 lines; the canonical form drops root's Pool route (target about 303). The acceptance scenario must not be narrowed.
  - **An A78 run** across two projects plus `commons`. Workers per order, `sameRegistration`, one proposal approved by two persons, retiring `packages`, and every refusal UC9 must make.

## 5. Sequencing summary

1. Short-form ruling together with decisions 1 to 6.
2. Prerequisites P-a to P-c, in parallel with stage A's spec batch.
3. Stage A (WO-1 to WO-4).
4. Stage B (WO-5 to WO-7), with the regression gate.
5. Stage C (WO-8).
6. P-d and P-b, then stage D (WO-9 to WO-11).
7. CLI (WO-12).
8. S1 rerun and the A78 run.

Each stage is useful on its own: A gives project memory, prune and drift; B gives granular Interfaces for trusted people and agents; C gives shared Pools; D gives approvals without a terminal.

## risks
1. **Size and opportunity cost.** About 100 promises in substance, plus 45 to 55 reworded, 7,500 to 13,000 Rust lines and 11 to 14 work orders. That is larger than the short-form move, and it competes for the same window and with the first build's increments. Every skeptic found its design's estimate low by 1.5 to 2 times. Treat these figures as paper counts too. Mitigation: four stages, each useful on its own, and a regression gate that no existing routing outcome flips.

2. **A false sense of isolation.** In v0 a Project is a mediated boundary. Any run on the engine's runner can read root's client token (shell lib.rs:280-299; IFC-022; spec/README.md "programs that run as the engine's operating-system user ... can get past every gate"). That includes a prompt-injected Codex worker that a project manager legitimately registered. The template cap stops a manager from choosing the program, runtime or host paths. It does not stop the program itself. Until DL13, a separate OS user (DL16) and a confining runtime exist, granular Interfaces suit trusted collaborators and agents only. This has to be stated in the README, not left in a footnote.

3. **"A person approves" is a client obligation.** The engine serves the approval page and raises an elicitation, but whether a person or a model clicks depends on the client honouring ruling 49. The owner's root Interface is often driven by an AI admin session. An Interface marked `person.v1` is root's declaration, not something the engine verifies.

4. **The approvals feature may not be what the owner asked for.** It comes from the computed task, not from the owner's words. The mitigation is to stage it last and decide it explicitly (decision 10), so the owner can drop it without unwinding stages A to C.

5. **The access chapter becomes much harder to read and to get right.** Scopes per row, two halves, five bound dimensions plus pairing, per-half judging, delegation of envelopes, and transitive revocation. The "within" checks across dimensions and the `openedBy` tree are classic places for bugs. Mitigation: one negative conformance case per skeptic counterexample, and inventories that show every route to a power.

6. **Churn from scoped names.** References, requests (LIF-031), records, refusals, tombstones, the CLI's `named()`, and 73 name lookups in the core all change. Resolving a name in the scope of the Conduit used is a new idea that authors and agents must learn, and reserved names can still surprise people. The fallback (global names) is cheaper but leaks across projects.

7. **Disclosure leaks remain.** The first tier hides existence only in refusals and resolution. Project and Interface names, contract addresses, tool descriptions, IFC-014 diagnostics and the content of shared Resources can still reveal another project.

8. **Shared writable Pools carry prompt injection across projects.** The `queue` written by packages and read by conduit-core's coordinator is one example. Audiences cap operations, not content. Required restrictions help only where a Type offers a filter, and partitioning by the caller's project needs implementations to honour the new key.

9. **Friction from the safety rules.**
   - Template pinning means every change to `pkg-worker` needs the grant renewed.
   - Template authoring needs root unless root grants `templates: all`.
   - Envelopes for Agents need a person's click.
   - Cross-project grants need both halves, so root is often the only one who can complete them.
   - A narrowed audience closes whole Conduits (ACC-011), so receivers must reopen narrower ones.
   Too much friction pushes operators back toward root, which defeats granular Interfaces.

10. **`management` gains reach over project contents.** Every holder of `management`, not only root, now administers every Project's members and their Pools' Resources. Today only root holds it, but a future co-admin or broker gains more than they would have before. This must be shown in inspection and in any upgrade report (ruling 52).

11. **Revocation gaps.** Without `revoke.v1`, closing a manager leaves its grants open (ACC-030). With it, the transitive tree must be computed correctly, and objects the actor registered keep running until someone retires them.

12. **The store break.** Root's first-start Conduits are permanent and cannot gain the new operations by request, and objects cannot be moved into projects. Lab stores must be re-initialized and S1 rerun. Anything else in use must be recreated.

13. **Interplay with the short form.** If the short form becomes normative with different key semantics or a different unit of authoring, the concise keys and the "submission" rename may need rework. Deciding both together avoids that.

14. **Use cases deferred.** UC5 (a machine per project) and UC6 (a peer per project) are deferred, and machines are not isolated per project. The owner's enrollment and WSL direction still needs v1 work: a node audience that the engine checks after placement, which conflicts with placement being fixed (PLC-009).

15. **Root as the approval bottleneck.** Anything in `commons` or across projects with no other approver comes to root. Fatigue raises the risk of approving phishing-shaped actions, such as a sign-in for the owner's account requested for a collaborator's Agent. Mitigation: prompt root only when needed, show the registrant and the program digest, and in v0 raise sign-ins only for instances root registered or approved.

16. **Vocabulary.** Folding today's "project" into "submission" touches 55 or more promises, 349 Rust identifiers and about 270 runner steps. Inconsistent leftovers would break ruling 42's one-meaning rule. Choosing another word for the new object avoids this, at the cost of the concise `project:` key.