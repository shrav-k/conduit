Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Project: use cases the design must serve (what is declared, who may do what, what must be stated, what must be refused)

## Key points
1. Management cannot be scoped today. `register.v1` on `management` trusts its holder with any object anywhere, and even a bounded delegate needs it to register the standalone Conduit it then opens (ACC-030, ACC-033 rows 2/5/7, CON-028). Authority scoped to a project is the heart of the change; without it UC3, UC4 and UC9 cannot be expressed.
2. `bound.v1` limits targets, Pools and operations, but not consumers or templates: its holder may open "for any consumer" (ACC-029, ACC-030). It covers an Agent only by naming it. A project scope also needs consumer, template, Agent-membership and placement dimensions.
3. Membership differs by family. Agents and Resources have one home. A Pool needs a home plus a declared audience, or must be engine-level. Interfaces are engine-level with authority over one or more projects. First-start objects, nodes and system-owned providers belong to no project. "A project references every object" holds only with these distinctions, and UC7 (retire a project, the shared Pool lives on) is the case that tests them.
4. A shared Pool declares its audience: which projects, and the most operations each may be granted. It must say whether "all projects" covers projects made later, as a bound's Pool covers later Resources (ACC-029). Cross-project access needs consent from both sides (consumer's project, target's home), and either side may narrow at once.
5. The engine must decide from declared data whether a change widens. The spec has no notion of widening today (G7). A successor Conduit must be compared with its predecessor, or T5/T6 re-points and close-then-reopen tricks are misclassified. Settings in opaque configuration, such as a directory root or reach flags, need the Type to mark them.
6. Authority and approval are separate. Narrowing needs authority and never waits. Widening needs authority and raises a pending action unless the requester is a declared approver of every scope it touches. The approver's own authority must cover the change, the proposer never approves itself, the approval binds the exact requests with `expectedId` (drift aborts), and both proposer and approver are recorded.
7. S1 must not get heavier. In the concise form the project costs at most one line, root's own widenings raise nothing, and equal re-points are not widenings. Otherwise the design fails the owner's bar that the simplification measured; root's own Conduits were already 22% of the coordinator's lines.
8. Pending actions are routed by project to the declared approvers and stay visible to root: sign-ins for declared accounts, machine admissions, approvals of widenings. Sign-ins stay person-only (ruling 49). Whether an Agent may ever approve is open: preservation test 5 pulls one way, "through the Interface" the other.
9. Placement needs authority. Nodes and runtime providers can be scoped to projects, and the engine must check the scope after the machine provider places an instance. Today PLC-015 and PRV-006 carry no authority, and labels are only reported status. Narrowing a node's scope runs into placement that never changes (PLC-009).
10. A peer engine introduced at project level separates engine-wide trust (root) from what a project exposes (that project's approvers). Each engine keeps its own half (ruling 51 decision 4), and the partner's caller chain is recorded but not used in decisions.
11. Names are global to the engine today (OBJ-002, OBJ-025). UC8 needs names per project, references that resolve in their own project, cross-project references that are always qualified, no "current project" kept by the engine, and a decision on the engine-level names (`root`, `system`, `management`, `access`, `local`, nodes). Tombstones, records, refusals and pending actions must all be keyed by project.
12. Retiring a project must: list and close every Conduit crossing its boundary; leave shared Pools alone; void its pending actions; keep its records and tombstones; and keep a new project of the same name from inheriting authority or reopening revoked grants without fresh approval.
13. The word is taken. "Project" already means a submission's documents and manifest (SUB-005, SUB-008, NAM-022's `#name`), so the old term or the new object must be renamed (ruling 42's one-meaning rule).
14. SEC-008 hard-wires root as the only one who opens a Conduit that reads a secret. An auth Pool for one project still needs root to wire its secret, and whether a project's approvers may ever do so is a separate decision.
15. Existing paradigms fit ruling 40: Kubernetes namespaces with RBAC escalation prevention, Gateway API ReferenceGrant (an audience), GCP Shared VPC and AWS RAM (shared Pools), project node selectors (node scope), Terraform saved plans (approval bound to the change). Pools already are "ownership as namespace" (pools.yaml header), so a Project may be the Pool idea extended to Agents and Pools.

## Findings
Repo: <home> (read only; nothing was modified). Promise IDs are cited as the spec has them. Sketches use placeholder words (`project`, `home`, `audience`, `managers`, `approvers`, `authority`, `envelope`, `scope`). They name what must be stated and are not proposed syntax. Each sketch must be writable both in today's canonical documents and in a concise form (one line per grant, `project:` stated once per file, as in docs/proposals/simplification/research/synthesis.md item 1).

## 0. What today's specification gives, and the gaps every use case runs into

- **G1. Nothing registered remembers a project.** "project" means only the documents and the one manifest handed over in one registration (spec/definitions.md, core terms; SUB-005). A submission only adds objects (OBJ-002), and after registration no object records which submission made it.
- **G2. Management cannot be scoped.** A Conduit to `management` carries no bound (CON-028) and can apply no filter (CON-022, MAN-012). One that lists `register.v1` or `replace.v1` trusts its consumer with any object authorized against `management`, an Agent from any template included (ACC-030; spec/README.md "What access does not narrow"). Registering an Agent, an Interface, a Pool or a standalone Conduit, and setting up, retiring or inspecting an Agent, Interface or Pool, are all authorized against `management` (ACC-033 rows 2, 5 and 7). So even a delegate with a bounded right to open needs `management` to register the standalone Conduit it then opens. The only narrower route is a broker (ACC-030), whose narrowing is its own and not the engine's.
- **G3. A bound limits targets, Pools and operations, never consumers or templates.** `bound.v1` has the inputs `pools`, `targets` and `operations` only (spec/core/requirements.yaml). The holder "may open, for any consumer, a Conduit that lists any operation the bound names" (ACC-030). Naming a Pool covers the Pool and its Resources, including later ones (ACC-029), but never an Agent: an Agent is covered only by naming it as a target.
- **G4. Only Resources have an owner.** POOL-001 gives each Resource exactly one owning Pool, fixed for life. Agents, Interfaces, Pools, Conduits and templates belong to nothing. Ownership "permits nothing" (ACC-028). The pools.yaml header already calls ownership "one stored fact, as a resource's group is in Azure and an object's namespace is in Kubernetes".
- **G5. Names are global to the engine for each kind** (OBJ-002; a clash reports conflict, OBJ-025). Within one submitted project they are unique across kinds (NAM-027, ruling 37). A name is freed only when its object is removed (LIF-035, OBJ-010/011).
- **G6. Placement and runtime carry no authority.** An instance depends on its node through placement alone, with no Conduit (PLC-015). Placement never changes (PLC-009). `runtime` is a bare reference, checked only for kind and state (PRV-006). Labels are reported status that no document sets (definitions, "label"). Machine and package-source providers must be owned by `system` (PRV-013). Runtime and secret-store providers may live in any Pool.
- **G7. The spec has no pending actions and no notion of widening.** Rulings 49 to 51 set the direction (sign-ins, enrollment, one generic pending action raised at the Interface, root only in v0), but no promise exists yet. The spec defines "narrow" (definitions; INS-014) and nothing that classifies a change as widening.
- **G8. Secrets are root-wired.** Only the root Interface, on its own behalf, opens a Conduit that lists `get-secret.v1`, and only for a Resource or Agent consumer (SEC-008).
- **G9. Root sits above everything.** Root's Conduit to `access` has no bound (ACC-022), no request closes root's Conduits (ACC-024), `declare-lost.v1` and `purge.v1` are reserved to root (ACC-032, REC-004), and a new Interface has no access until someone opens a Conduit for it (ACC-023).
- **G10. The word is taken.** "Project", "submitted project", a project's `address` (SUB-008, MAN-015) and `#name` meaning "in the same project" (NAM-022) all mean the document set. Ruling 42's one-word-one-meaning rule forces a rename of either the old term or the new object.

## 1. Requirements every use case relies on

**R1. Membership differs by family.** "A project references every object under it" holds only with these distinctions:

| Object | Relation to projects that the use cases need |
| --- | --- |
| Agent | Exactly one home project; it may hold Conduits into other projects with their consent (UC2) |
| Resource | Through its Pool (POOL-001 unchanged) |
| Pool | A home project plus a declared audience of other projects, or engine-level and referenced by many. UC7 is the case that tells the two apart |
| Conduit | Crosses projects. Both ends' projects see it, either side may narrow it, and widening needs both (UC2, UC3, UC10) |
| Interface | Engine-level, holding authority over one or more projects (UC3). Membership in one project is the alternative, and UC7 tests it |
| Template, contracts, pins | Per project (each project's manifest pins its versions, NAM-006). `install.v1` stays engine-wide and grants nothing (NAM-013) |
| First-start objects, nodes, system-owned providers | In no project (FST-001, PLC-026, PRV-013). First start still makes no project (FST-001 to 003) |

**R2. Authority over a project is one relation that can be inspected and revoked** (preservation test 2). It can be read from the project ("managers") and from the Interface ("authority over"). It is granted only by an actor whose own authority covers it (test 5; ACC-030's "only as it is or narrower"). Today's Conduit to a Pool offering `pool-management.v1` is the existing paradigm to extend.

**R3. The engine, not the CLI, decides from declared data whether a change widens.**

| Widening | Narrowing | Neutral |
| --- | --- | --- |
| `open.v1`, including a declared Conduit at setup (ACC-005). A successor Conduit that lists an operation its predecessor did not, or drops a filter. Opening a right to open, or a wider bound. Registering or setting up an Agent whose declared Conduits will open. Registering from a template outside the project's allowed set. `replace.v1`, which changes programs and `remote` URLs (ACC-030). A control that sets a setting the Type marks as widening (`accountServices` or `peerMessaging` true, ruling 48; a directory root moved up). Adding a project to a Pool's audience or raising its ceiling. Adding a manager or approver. Admitting a machine, or widening a node's project scope. Introducing a peer | `close.v1`, retire, unregister. Removing a project from an audience. Removing a manager or approver (never root, ACC-024). Narrowing a node scope. Dropping a peer | `inspect.v1`, `install.v1` (NAM-013), reconcile |

A successor must be compared with its predecessor. Without that, every T5 or T6 re-point and every "close then reopen with more" is misclassified, one way or the other. Opaque configuration cannot be judged unless the Type marks it.

**R4. Approval is separate from authority.** Narrowing needs authority and never waits. Widening needs authority and, unless the requester is a declared approver of every scope the change touches, raises a pending action. The shape is ruling 51 decision 5: kind, subject, evidence, expiry, outcome.
- The subject is the exact requests, with `expectedId` (LIF-037 and ruling 53). Any drift aborts the action (critic gap 1).
- The evidence is the reach before and after, written by the engine. The proposer's justification is shown only as a quote.
- The approvers are those of the consumer's project and of the target's home, where they differ.
- An approver's own authority must cover the change, and the proposer never approves its own proposal.
- Records attribute both proposer and approver (test 7).
- A pending widening never holds back a narrowing, and no narrowing is bundled into one. In T4, B's narrowing applies at once while A's widening waits.
- Sign-ins stay person-only (ruling 49). Machine admission needs a person in v0 (ruling 50, point 2).

**R5. A shared Pool states its audience**: which projects, and the most operations each may be granted. "All projects" must say whether it covers projects made later, as a bound's Pool covers Resources made later (ACC-029). Cross-project access needs consent from both sides, and either side may close it at once.

**R6. Canonical and concise forms both work.** The concise form states membership once per file. The canonical form states it per registration or per document (like `metadata.namespace`). A reference to another project's object is always qualified. The engine has no "current project": a CLI may supply one, as a kubectl context does, but the request is explicit (ruling 52).

**R7. Explicit, and no more disclosed than allowed** (ruling 52, ruling 53 decision 3c). Every derived object (the Pool per project, the operator's authority, turn Conduits) is recorded and shown. A refusal names the scope that refused it. Inventory, tombstones and records of a shared Pool show a project only what that project may inspect, with no counts of the rest.

## 2. Use cases

### UC1. S1 as one project (scenario S1; FIRST_BUILD tasks T1 to T7)

Sketch:
```
project s1                                  # one line in the concise form
  managers: root  approvers: root           # derived from the registrant, shown in plan and inspection
  pool workspace (home s1; the derived Pool per project)
    shared:  local-directory root=<lab>/shared
    history: file-history    root=<lab>/history
  agent agent-a (claude-code)  reach project->shared [read,list]        history->history [read,list,write]
  agent agent-b (codex)        reach project->shared [read,list,write]  history->history [read,list,write]
  turns: root->agent-a, root->agent-b       # derived
```

Who may do what: root does everything in s1, as today. The agents have only their own Conduits.
- T3: "inspect s1" lists every member, every Conduit a member consumes or is the target of, and the reach flags.
- T4: B's narrowing applies at once. A's widening raises nothing, because root is a declared approver acting itself, and the record says so.
- T5/T6: an equal re-point is not a widening.
- T7: the revoke leaves a tombstone keyed by (s1, agent-a, project). The restore is an explicit widening.

Must state:
- each object's membership, once;
- the project's managers and approvers, derived and shown;
- the derived Pool, operator Conduit and turns as ordinary recorded objects tied to s1 (this answers ruling 53 decision 6);
- drift (members present in the engine but absent from the file) and prune, both limited to members;
- tombstones per project.

Must refuse:
- prune or drift touching non-members (`system`, `local`, `management`, `access`, `root`, nodes);
- adopting an existing object by name (synthesis item 2: a first apply refuses a Pool that already has the project's name);
- a later apply reopening agent-a's revoked grant without an explicit restore (critic gap 2);
- any design that adds author lines to S1. The owner's bar is less work than A70 on most tasks (docs/proposals/simplification/README.md).

Today: no object remembers s1. Root's own Conduits to its Pool and for turns were 72 of the coordinator's 332 lines (22%) (simplification README; coordinator-run/project/instances.yaml).

### UC2. Conduit building Conduit (owner's direction A14 and S2; A78; WORK_ORDERS.md "Running the orders through Conduit")

Sketch:
```
project conduit-core   managers: root  approvers: root
  pool core-work (home)  tree-<order> (rw directory, one per order)
  pool repo (home; audience: packages -> main [read], queue [post,read])
     main  (read-only checkout of main)   queue (message history of orders and reports)
  pool auth (home; audience: conduit-core only)
     onepassword  secret-store provider (service account limited to the test vault)
     linear-key   secret Resource -> vault item       # wired by root only (SEC-008)
     linear       Linear MCP, remote; its header comes from linear-key through its own Conduit
  agent coordinator (claude-code)
     reach: queue [post,read]; linear [list,get,create,update]; skills [read,list]
     authority in conduit-core and packages (packages by packages' consent):
       register workers from template `worker` only; set up and retire them;
       open/close a worker's Conduits to its own tree [rw], main [r], queue, skills [r]; turns to workers
  agent triage (codex)   reach: linear [list,get]   # S2: one login, two restrictions
project packages       managers: root  approvers: root
  pool pkg-work (home)   tree-<order>
  agents pkg-worker-* (codex), several at once: own tree [rw], main [r], queue, skills [r]
pool skills  (audience: all projects, ceiling [read,list]; writes: root)
```

Who may do what:
- The coordinator acts in packages only through authority that packages' managers grant, and they may close it.
- Triage cannot reach `linear-key` and has nothing in packages.
- A worker reaches only its own tree, `main` read-only, the queue and skills read-only.
- No agent is given the Linear credential (owner-direction 2026-11-10, S2 row).

Must state:
- a Pool's home and its audience with a ceiling per project;
- whether "all projects" covers future projects;
- each Agent's home;
- cross-project Conduits and the consent behind them;
- the coordinator's authority scoped by project, by consumer, by template and by operation;
- pins per project with `install` engine-wide;
- concurrent workers whose generated names never collide.

Must refuse:
- any packages member reaching auth, even one with a right to open in packages;
- the coordinator, under its packages authority, opening a Conduit whose consumer is outside the scopes it holds (the consumer dimension missing in G3);
- the coordinator registering an Agent from any template but `worker`, such as one with `accountServices` on, without approval (G2; today only a broker can do this);
- anyone but root opening a Conduit that lists `get-secret.v1` (SEC-008);
- a worker writing skills or `main`.

Today: G2, G3 and G4. A78 has not started (ROADMAP.md, A78 row).

### UC3. A second person who may manage only packages (rulings 49 and 50, test 8)

Sketch:
```
interface collab   (a person; engine-level)  authority: packages
project packages   managers: root, collab   approvers: root, collab
interface ci-hook  (an outside system, e.g. a GitHub webhook)  authority: packages queue [post] only
```

Who may do what:
- Collab may register, set up, retire and inspect packages members.
- Collab may open and close Conduits whose consumer is a packages member and whose target is a packages member or within an audience that admits packages, up to its ceiling.
- Collab approves widenings in packages and receives packages' pending actions: approvals, sign-ins for the declared accounts of packages agents (ruling 49 point 3: a sign-in to another account is refused and undone), and enrollment for packages (UC5).
- Collab may narrow anything into packages, including closing the coordinator's cross-project Conduits.
- Ci-hook shows the owner's "more granular interfaces".

Must state:
- an Interface's authority over a list of projects, with its operations;
- how pending actions are routed: to the declared approvers of the scope, and always visible to root;
- which accounts a person may complete sign-ins for;
- what collab sees: filtered inventory and records, with no counts of the rest.

Must refuse:
- collab acting in conduit-core, or on auth, skills or repo beyond packages' ceiling. The refusal is ruling 52's first tier only and does not disclose whether the object exists;
- collab widening its own authority, or giving ci-hook more than collab holds;
- collab removing root's authority (ACC-024 spirit), or authority it did not grant;
- collab seeing or completing a conduit-core sign-in;
- collab approving alone a widening that touches skills or auth;
- collab retiring or unregistering a shared Pool;
- `declare-lost.v1` and `purge.v1`, which stay root's unless re-scoped deliberately (ACC-032, REC-004).

Today: ACC-023 gives a new Interface nothing. A Conduit to pkg-work covers set-up, retire and inspect of its Resources, but registering packages Agents or any standalone Conduit needs global `management` (G2). Ruling 49 left sign-ins by other people to v1.

### UC4. An agent that may propose changes to its project but not approve widenings (test 5, ruling 53 decision 2)

Sketch:
```
project packages  approvers: collab, root      # pkg-lead is not one
  agent pkg-lead (claude-code)
    may narrow at once: close and retire what it may reach in packages
    envelope (pre-approved): worker trees [rw], main [r], queue, skills [r]
    beyond the envelope: may propose only
```

Flow: pkg-lead proposes `pkg-worker-docker -> linear [list]`. The engine records a pending action:
- kind: approval;
- subject: the exact requests, with `expectedId`;
- the proposer and its caller chain;
- the reach diff, written by the engine;
- approvers: packages {collab, root} and auth {root};
- an expiry.

Collab alone is not enough, because auth's approver is root. Pkg-lead sees the outcome on its next request or inspection and is never woken (test 6; A10).

Must state:
- whether an envelope counts as standing approval;
- the pending action's shape and binding;
- that both proposer and approver are recorded;
- what happens to a proposal whose proposer is retired (voided).

Must refuse:
- pkg-lead approving its own proposal, or any approval by an actor whose approval authority comes from pkg-lead. ACC-030's rule that you pass on only what you hold applies to approval too;
- a split widening: close, then reopen with more, is judged against what was closed;
- a narrowing waiting behind a pending widening, or bundled into one;
- an approval after expiry, or of a changed subject;
- pkg-lead raising a sign-in or enrollment action, or sending any action to an Interface that is not an approver of the scope (no prompt spam; elicitations come only from the engine, ruling 49);
- pkg-lead removing collab's authority;
- a widening disguised as a control, such as turning on a worker's `accountServices`.

Today: nothing proposes or approves. The synthesis has `--yes` never accept a widening (synthesis item 4), and the critic finds that a non-interactive author cannot approve (critic gap 1).

### UC5. A machine enrolled for one project's agents, and one for all (rulings 50 to 52; docs/proposals/enrollment/README.md option C)

Sketch:
```
machine provider: tailscale (system Pool, PRV-013)
enroll win-wsl    scope: packages        # collab requests; a person who approves for packages admits
enroll build-box  scope: all projects    # root only
pool pkg-work: docker (runtime provider, home packages)
```

Who may do what: packages instances run on win-wsl through docker, and any project's instances may run on build-box. The join secret goes only into the action shown to the admitting person (ruling 52, decision 4).

Must state:
- a node's project scope as data the engine enforces, not a label the machine reports;
- which projects may name a runtime provider in `runtime`;
- that widening a scope is a widening;
- what narrowing a scope does to foreign instances already placed there, given that placement is fixed (PLC-009): refuse until they are gone, or retire them;
- who may declare a scoped node lost.

Must refuse:
- a conduit-core instance placed on win-wsl, whether the document gives the placement or `place.v1` chooses it. The engine checks the scope after the provider answers;
- a conduit-core instance naming packages' docker as its `runtime`;
- the machine claiming a scope or label (ruling 52 decision 3: labels are bound by the provider);
- collab enrolling for all projects or for conduit-core;
- network membership treated as a grant (ruling 51 decision 1).

Today: no authority over placement or runtime (G6). What a person checks before admitting is still open (ruling 52 point 5).

### UC6. A peer engine introduced at project level (ruling 51; networking-paradigms.md section 6)

Sketch:
```
main engine:  project packages
  pool peer-win (home packages):  win-build  remote https://<peer engine>
  introduction: peer trust domain (SPIFFE); scope packages; trust approved by root, exposure by packages' approvers
peer engine (its own root): interface partner-main-packages
  the peer's root opens Conduits from it -> builder [build.run.v1]
```

Who may do what: packages agents call `win-build`, and the peer decides each call under its own Conduits. Each side closes its own half (decision 4). Contrast the owner's caveat: their own WSL engine admitting the owner's root with full management is engine-level, not project-level.

Must state:
- the introduction as a pending action on both sides (an invitation with a fingerprint);
- engine-wide trust and project-level exposure as two separate approvals;
- the partner's project scope on each side;
- revocation on each side;
- that the caller chain is recorded and not used in decisions (decision 10);
- that the credentials are held by the engine.

Must refuse:
- conduit-core agents reaching `win-build`;
- the partner acting outside packages;
- collab introducing a peer engine-wide, or giving it management;
- one engine opening a Conduit on the other's objects;
- reachability over the tailnet treated as permission;
- another party's machine as a node (decision 3).

Today: ruling 51's sketch is engine-wide only.

### UC7. Retiring a project while a shared Pool lives on

Retire packages.

| What happens | Objects |
| --- | --- |
| Retired as members | pkg-lead, the workers, pkg-work and peer-win (each with its Resources, LIF-021) |
| Ended or removed | Collab's authority over packages; packages' entries in the skills and repo audiences; win-wsl's scope; pending actions (voided) |
| Closed | Every Conduit a member consumes or is the target of, the coordinator's turns into packages included (CON-008, ACC-039) |
| Unaffected | skills, repo, auth, coordinator, the collab Interface (left with no access), nodes |

In-flight invocations into skills run to their end (ACC-034). Unknown outcomes stay reconcilable by skills' managers (ACC-033 rows 12 to 14). Records and tombstones are kept (purging is root's, REC-004). Retirement is a narrowing, so it waits for no approval. It still goes through explicit plan and apply (ruling 53 decision 4), and the plan lists every Conduit that will close, cross-project ones included (synthesis item 4).

Must state:
- what a project contains and what it only references;
- that a shared Pool outlives any one project that uses it;
- what happens to a Pool whose home is being retired while others use it: refuse, move its home first, or make Pools engine-level;
- that the project's name is freed;
- that a new project with the same name inherits no authority, audience or approval, and whether old tombstones bind it.

Must refuse:
- retiring skills, repo, auth or conduit-core's members as part of packages;
- retiring conduit-core while it is home to Pools others use, without explicit handling;
- registering into packages once its retirement is dispatched (as POOL-004 does for Pools);
- recreating the name to reopen revoked grants without fresh widenings;
- reaching into a peer's half.

Today: there is no group retirement. LIF-021 covers one Pool, and Agents are not grouped.

### UC8. The same name `shared` in two projects

Sketch:
```
conduit-core: resource shared, agent worker
packages:     resource shared, agent worker
coordinator:  as core-shared -> conduit-core/shared ; as pkg-shared -> packages/shared
```

Must state:
- the scope of name uniqueness: per project and kind, or per project across kinds, as NAM-027 has it for documents;
- that a reference resolves in its own project only;
- that a cross-project reference is always qualified;
- what happens to names held at engine level (the first-start objects, nodes): reserve them or qualify them;
- how a request names an object, since LIF-031 references by name and the reference now needs the project;
- that `expectedId` still guards swaps (LIF-037; ruling 53 adds it to open and close);
- that records, refusals, pending actions and tombstones are all keyed by project;
- that tool names are unaffected, since they come from the local name or `as` (IFC-012).

Must refuse:
- an unqualified `shared` in packages resolving to conduit-core's;
- an unqualified request from an actor whose authority spans both projects;
- an ambiguous bound, audience or approval subject;
- two tool prefixes that collide for the coordinator (IFC-014);
- a project member named `root`, `system`, `management`, `access` or `local`, if those names are referenced unqualified;
- telling a packages agent that names conduit-core/shared anything beyond the first-tier refusal.

Today: OBJ-002 and OBJ-025 reject the second `shared` with conflict. The synthesis generates `shared--2`, and the critic (gap 16) notes that the derived Pool's name collides across kinds.

### UC9. Work orders through Conduit, concurrently, in both projects (A78; A77; ruling 48)

Order 031 runs in conduit-core while orders 032 and 033 run in packages. For each order the coordinator, within the project's envelope and with no approval:
- registers a worker from `worker` and a tree in the project's work Pool;
- opens the worker's Conduits: its tree [rw], main [r], queue, skills [r], and a turn.

At merge it closes them and retires the worker and the tree. That is a narrowing and never waits. A worker that needs more, such as network access or Linear, proposes (UC4).

Must state:
- per-order objects as project members, with generated names that do not collide;
- that a worker with a new key is not blocked by a tombstone of a merged worker;
- whether the queue is per project or filtered, since a shared history would show packages workers conduit-core's orders.

Must refuse:
- the worker of 032 reaching tree-033;
- a worker opening any Conduit (it has no right to open);
- dispatch into packages after collab has closed the coordinator's authority;
- reopening a merged worker's Conduits.

### UC10. Promoting auth from one project to two

Later, packages needs to read Linear for release notes. Packages is added to auth's audience with the ceiling `linear [list,get]`. This needs the target side's approver (root, for auth) and the consumer side's approver (collab or root, for packages).

Must refuse:
- the widening approved by collab alone;
- any audience that reaches `linear-key` (SEC-008; ruling 42 D10: a secret Resource's type offers no other capability);
- packages getting write.

The reverse, removing packages from the audience, is a narrowing: packages' Conduits to `linear` close at once.

## 3. Decisions the use cases raise for discussion

- **F1.** Pools: a home project plus an audience, or engine-level and referenced? UC7 tells them apart.
- **F2.** Interfaces: engine-level, or members of a project? UC3 and UC7.
- **F3.** The form of Project:
  - (a) a sixth family, the target of a `project-management` capability as a Pool is of `pool-management.v1`;
  - (b) a stored fact on each object, as namespaces are, with authority through Conduits to the project;
  - (c) the Pool generalized to own Agents and Pools.
  Pools already are "ownership as namespace" (G4).
- **F4.** Is a bound or envelope a standing approval? UC4 and UC9. Without that, every dispatch waits for a person.
- **F5.** May an Agent ever approve? Test 5 (every management action is open to an agent under a grant) pulls one way. The owner's "through the Interface" and ruling 49 (sign-ins only to a person) pull the other.
- **F6.** Which project holds a cross-project Conduit? Consent from both sides, and narrowing by either side.
- **F7.** The scope of names, and engine-level names reserved or qualified (UC8).
- **F8.** Renaming today's "project" (the submission), or naming the new object differently (G10).
- **F9.** How Types mark which settings widen, and what an unmarked setting defaults to.
- **F10.** What narrowing does to placement, which is fixed, and to a home Pool still in use.
- **F11.** Whether "all projects" in an audience includes future projects.
- **F12.** What a shared Pool's records and inventory show each project.

## 4. Prior art that fits ruling 40 (from my own knowledge; not re-read in this session)

| Prior art | What it is analogous to |
| --- | --- |
| Kubernetes namespaces and RoleBindings | Authority scoped to a project; names unique per namespace and kind; Nodes are cluster-scoped |
| Kubernetes RBAC escalation prevention (the `escalate` and `bind` verbs) | No one grants what they do not hold, approvals included |
| Gateway API ReferenceGrant | The target side consents to cross-namespace references, as an audience does |
| GCP Shared VPC (host and service projects), AWS RAM | Shared Pools |
| OpenShift project node selector, Kubernetes PodNodeSelector | A node scoped to projects |
| Kubernetes CSR approval, GitHub required reviewers and CODEOWNERS | Declared approvers |
| Terraform saved plans | An approval bound to the exact change |

## Sources read

- spec/definitions.md
- spec/conventions/access.yaml (ACC-006, 022 to 024, 028 to 030, 032, 033)
- spec/core/requirements.yaml (`bound.v1`)
- spec/core/capabilities.yaml
- spec/promises/pools.yaml, objects.yaml, placement.yaml, providers.yaml, secrets.yaml, naming.yaml, conduits.yaml
- spec/conventions/lifecycle.yaml
- spec/interface/implementation.yaml (IFC-012, 014, 022, 042)
- spec/README.md
- docs/AUDIT.md (ruling log 37 to 53; purpose; preservation tests)
- docs/proposals/ruling-48/README.md through ruling-53/README.md
- docs/proposals/enrollment/README.md and networking-paradigms.md
- docs/proposals/simplification/README.md, research/synthesis.md, research/critic.md, research/map-prior-art.md
- docs/proposals/owner-direction/2026-11-10.md
- ROADMAP.md
- docs/implementation/WORK_ORDERS.md
- docs/proposals/first-build/increment-4/coordinator-run/

## Unverified
- The approvals direction (widening approved as a pending action at the Interface, narrowing never waits, approvers declared) comes from the computed task text. It is not in the owner's relayed words, and I found it recorded nowhere in the repo. It is consistent with ruling 53 decision 2 (open) and with synthesis item 4 ("`--yes` never accepts a widening").
- The prior art in section 4 is from my own knowledge and was not re-read or fetched in this session.
- I did not find how an Interface other than root authenticates. IFC-022 and DL13 issue credentials to instances, and ruling 49 leaves other people's Interfaces to v1. UC3 assumes this gets solved.
- Pending actions do not exist in the spec yet. Their shape comes from ruling 51 decision 5 and the enrollment proposal, neither of which is binding.
- Choices I made for the scenario:
  - Linear and the auth Pool are placed in conduit-core (the task says only "for one").
  - "skills" is read as Claude Code and Codex skill directories.
  - The `repo` Pool (the `main` checkout plus the work-order queue) is my addition, as a shared Pool with a different ceiling per project.
  - The names collab, ci-hook, pkg-lead, triage, win-wsl, build-box and win-build are placeholders.
- What the parallel short-form investigation contains is unknown. The concise sketches follow only the simplification's project file, and its line counts are paper counts (ruling 45 accepts only a run as evidence).
- These are my analysis, not spec statements:
  - how each change is classified as widening or narrowing (R3);
  - the outcomes proposed for narrowing a node's scope while instances sit on it, and for retiring a project that is home to a shared Pool;
  - voiding a retired proposer's proposals;
  - routing pending actions to the approvers of both scopes.
- I did not check who may read records across Pools today, so the requirements on what a shared Pool shows each project (R7, F12) are stated as "must state", not as gaps confirmed in the spec.
- Mapping S3 to S10 onto these use cases is partial: S4, S5 and S8 are not exercised.