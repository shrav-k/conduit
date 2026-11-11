Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Home Projects (minimal change): a Project is a Pool of a new core Type that is home to objects of every family

## name
Home Projects (minimal change): a Project is a Pool of a new core Type that is home to objects of every family

## thesis
The smallest change that gives the owner granular Interfaces and shared Pools is to turn the CLI's derived "Pool per project" into a real object. A Project is a Pool whose Type is a new core Type, `core#project.v1`. It owns Resources directly, exactly as the derived default Pool did, so that Pool disappears. It is also the recorded home of every Agent, Interface, Pool, standalone Conduit and template registered under it. For its members it takes over from `management` as the authorizing target, so `management` stays the authority only for objects that belong to no project.

Authority over a project uses only existing mechanisms: an ordinary Conduit to the Project listing a new core capability, `project-management.v1`, plus an ordinary bounded right to open (a Conduit to `access`) whose `pools` names the Project. An Interface with authority over three projects holds three such pairs. Sharing a Pool is a bounded right to open that names that Pool, given to another project's managers. Use stays exactly as today: a Conduit to a Resource, whatever its home (ACC-028).

Five things are unchanged: the shape of bound.v1, CON-028, the Conduit family, names (still engine-wide per kind, OBJ-002) and first start. An engine whose documents name no project behaves exactly as today, so no existing vector breaks.

Membership is recorded on each member, once per submission, through one new manifest key, `project`. The engine stores it and it is fixed for life, as POOL-001's owner is. The Project's inspection computes its member list; the Project does not hold one.

The access rules need one new idea, home versus share, to make projects isolating. To open a Conduit for a member of a project, the actor needs a right that names that project. A right that names only a shared Pool lets its holder open Conduits into that Pool. It never lets the holder close other projects' Conduits.

Approvals reuse the existing split between registering and opening. Someone who may register in a project but may not open a Conduit leaves a proposal: a registered Conduit that is not open. The engine raises it as a pending action at the Interfaces whose existing rights would let them open it. Approving means opening it with `expectedId`. Rejecting means closing it. Narrowing never waits.

In the concise form S1 stays at 16, 6, 0, 2, 1, 1 and 1 lines (27, against the baseline's 33, 16, 0, 8, 3, 2 and 2, 64 in all). It is the ruling-53 package with the Project in place of the derived Pool.

This design does not serve per-project names, scoping of nodes and placement, an audience a Pool declares itself, two-party consent across projects (root bridges), registration limited to particular templates, or classifying configuration changes as widenings.

I challenge four parts of the owner's direction:
- A project should not hold a list of everything under it. Each object records its project, and the list is computed.
- A Pool has exactly one home. Being "accessible from multiple projects" means shares, not several owners.
- "Projects determine an Interface's permissions" is circular if the project grants itself. It works only as: the Conduits to a project are part of that project's documents, and its existing managers (root first) open them.
- "Who may approve" should not be a separate list beside authority. Approvers are exactly the holders of the authority to make the change, so no approval can exceed the approver's own authority.

## model
WHAT A PROJECT IS
- Not a new family or kind. A Project is an instance of the Pool family whose Type is the new core Type `github.com/shrav-k/conduit/core#project.v1`. The core package exports a template `project` (kind Pool, that type, config {}), so one instance document makes one.
- Every Pool rule still applies to it:
  - it owns Resources (POOL-001);
  - it is a Conduit target (OBJ-013);
  - a bound's `pools` may name it (ACC-029);
  - it retires as a group (LIF-020, LIF-021);
  - nothing can be registered into it once retired (POOL-004);
  - a Conduit to it permits only its own operations (ACC-028).
- What it adds over a plain Pool: it is the home of objects of every family. Only `project.v1` has this power. A Pool of `pool.v1` or any other Type still owns only Resources.
- Why a Pool Type and not a sixth family: a sixth family would change OBJ-020 (ten kinds), the kind lists in every reference schema, the 28 places in spec/ that mention "the five families" or similar, the target kinds in OBJ-013, and bound.v1's arguments. As a Pool Type it needs none of that. The price is vocabulary: "a Project is a Pool", and `retire-with-resources.v1` on a Project retires its members (open question 2).

WHAT IT HOLDS, AND WHAT IT ONLY REFERENCES
- It holds nothing as a list. Each object it is home to records it, in an engine-owned field `project` that inspect.v1 returns, fixed for the object's life (OBJ-004 adds it to the engine-owned fields).
- Inspecting the Project computes and returns:
  - its members;
  - the Conduits whose target is the Project (who manages it);
  - the rights to open whose bound names it (who may act inside it);
  - its tombstones.
  The inventory of ruling 53 is filtered to what the caller may inspect, with no count of the rest. This gives the owner "a project references every object under it", read from the engine, without a second, mutable list (the lesson of Kubernetes ApplySet).
- What it references without holding: Pools homed elsewhere that its members use through Conduits. Use needs no membership. A reference grants nothing, and bound cover follows home, never use.

HOW MEMBERSHIP IS STATED
- The manifest gains one key, `project: <name>` (MAN-004). Every template and instance that a submission makes is a member of that Project, recorded by the engine.
- A submission with no `project` makes engine-level objects, as every submission does today.
- The Project the manifest names either exists already, or is made by the same submission. In that case the Pool document making it is the one document that is not its own member, and the request is authorized by `management`, as row 2 authorizes a Resource whose Pool is in the same submission today.
- A package's manifest states no `project`: packages are installed, not registered.
- `replace.v1` hands over a manifest. Its `project` must be the template's own: no object changes project. To move an object, register a new one and retire the old.
- In the concise form this is the existing `project: <name>` heading, one line per file.

MEMBERSHIP BY KIND (P is a Project)
- Resource: its home is its owner if the owner is a Project, otherwise its owner's home.
  - A Resource in P's submission must be owned by P itself or by a Pool whose home is P. Otherwise the document is rejected.
  - Ownership is unchanged (POOL-001: one Pool, fixed).
  - A Pool's entries share the Pool's home.
- Agent: home is the submission's Project, or none (engine-level).
- Interface:
  - A person's Interface is normally engine-level, created by root, with authority over one or more projects through Conduits.
  - A project may be home to an Interface for an outside system, for example a CI hook for `packages`. Retiring the project retires that Interface.
  - root is engine-level.
- Pool: an ordinary Pool's home is the submission's Project or none. A Project's home is always none: projects are flat and do not nest. `system` has no home.
- Conduit:
  - A declared Conduit shares its consumer's home.
  - A standalone Conduit's home is the submission's Project. In P's submission, one of its ends must be P or a member of P. Alternatively it may be a Conduit to `access` whose bound names nothing outside P. Otherwise it is rejected.
  - An engine-level standalone Conduit (from a submission with no project) may join anything. This is how root wires things across projects.
- Template:
  - Home is the submission's Project or none. replace.v1 and unregister.v1 of P's templates are authorized by P. This confines the replace.v1 hijack (design E #4) to the one project.
  - An instance in P may be made `from` a template of P, an engine-level template, or a package's template. A template of another project is rejected (INS-025).
- Contracts (`#name`, `address`): unchanged. They belong to the submission (SUB-008, NAM-022).
- Runtime provider: an instance in P may name in `runtime` a provider homed in P or an engine-level one (PRV-006). So PRV-020's in-use block never couples two projects.
- Nodes, `management`, `access`, `root`, `system`, and the machine and package-source providers: always engine-level (FST-001, PLC-008, PRV-013). Placement is unrestricted, as today.
- Installed packages and records: engine-level. A tombstone (ruling 53) records the project of what it marks.

NAMING SCOPE
- Unchanged: names are unique per kind across the whole engine (OBJ-002, OBJ-025), and a Project's name is a Pool name.
- A reference to an object in another project is the same bare name it is today. There is no qualified project/name form and no engine-held "current project".
- Two projects cannot both have an Agent called `worker`. This is the deliberate cost of minimal change (UC8 is not served).
- Mitigations:
  - ruling 53's `as` keeps object names out of tool names (IFC-012);
  - the concise compiler may prefix keys with the project name, shown in the plan;
  - a name collision reports conflict and so reveals that the name exists. Contents and inspection stay per project.

LIFECYCLE
- Create: register a Pool `from: core#project`. This is engine-level, so it is authorized by `management` (row 2) and is root's in practice. Setup goes through `management` too (row 5).
- Members are then registered, set up, retired, inspected and reconciled through the Project (routing in the authority field).
- Retiring:
  - `retire.v1` on the Project (through `management`) reports conflict while any member's shutdown is unconfirmed (LIF-020 extended to members).
  - `retire-with-resources.v1` on the Project (authorized by the Project itself, row 8) retires every member, of every family, in the reverse of setup order (LIF-021 extended): Agents before what they consume, member Pools with their Resources, member Interfaces, then member standalone Conduits are closed. Then the Project's own retire.v1 runs.
  - A failure holds back only what depends on it.
  - Registration into a Project once its retirement is dispatched is rejected (POOL-004 extended to every family).
- What survives a retired project:
  - engine-level objects and the members of other projects (only their Conduits into the retired members close, CON-008);
  - Interfaces that held authority over it, which lose those Conduits;
  - records (purged only by root, REC-004);
  - tombstones, which are engine state kept apart from records, keyed by the Project's ID.
- After shutdown the Project is removed and its name is freed (LIF-035).
- A new Project with the same name has a new ID. It inherits no Conduit, share or manager. Whether the old tombstones bind it is open question 12.
- A Pool homed in the retired project is retired with it, even if other projects used it; their Conduits into it close. Hence the advice to home shared Pools in a long-lived project such as `commons`.

WHERE THIS DEPARTS FROM THE OWNER'S WORDS, AND WHY
1. "Project references every object": membership is recorded on the member, and the project's list is computed (prior-art lesson 11, preservation test 2).
   - Not every object can be a member: the first-start objects, nodes, the providers in `system`, installed packages and records stay engine-level.
   - A shared Pool is referenced by many projects but is the member of one, or of none.
2. "Pools accessible from multiple projects": for use this is already true (ACC-028). For administration a Pool has one home.
   - Several homes would break preservation test 3 ("owned once"), ACC-033's single authorizer and ACC-012.
   - In every system the prior-art research covered, a shared thing has one home plus explicit share edges.
3. "Projects determine an Interface's permissions": read so that the Conduits granting authority over P may be registered in P's own documents, appear in P's inventory, and are opened by P's existing managers. The first manager is always someone who already holds authority, root at the start.
   - A project cannot grant itself anything.

## sharing
USE NEEDS NOTHING NEW
Any consumer with an open Conduit to a Resource uses it, whatever that Resource's home (ACC-028). `pkg-worker-1` (home `packages`) reading `skills-dir` (home `commons`) is one ordinary Conduit. That Conduit's home is `packages`, since its consumer is a member there.

HOME AND SHARE: THE ONE NEW ACCESS IDEA
- A right to open (a Conduit to `access` with a bound) holds the home of P when:
  - it is unbounded (root); or
  - its `pools` names the Project P; or
  - for an engine-level object, it covers that object as ACC-029 does today.
- A right that names an ordinary Pool, or names individual targets, only shares them.
  - Under ACC-029, naming Project P covers P, every member of P, and every Resource owned by a Pool homed in P, including those made later. Naming an ordinary Pool covers that Pool and its Resources, as today.
- Sharing a Pool = the holder of its home opens, for another actor, a right whose bound names that Pool. The bound's `operations` are the ceiling.
  - Example: root, holding every home, opens for `collab` the right `share-skills`: {pools: [skills], operations: [file.read.v1, directory.list.v1]}.

WHO DECIDES
- The holder of the shared Pool's home:
  - root, for an engine-level Pool;
  - the managers of the Pool's home project. In practice root, because the receiving actor is usually another project's manager, and opening a right for an actor needs that actor's home (see the authority field).
- The receiving project consents implicitly: a share lets its holder open Conduits only for consumers whose home it holds. A share therefore reaches a project's Agents only through that project's own managers.
- An alternative where the consumer side must also approve is not provided (see what is lost, item 4).

HOW IT IS DECLARED
- As an ordinary Conduit to `access` whose template applies bound.v1 naming the Pool. One share per Pool keeps the ceilings per Pool: bound.v1's `operations` list is flat across everything the bound names.
- Shares appear in the inventory of the holder (Conduits it consumes). Through the inventory extension, they also appear when the shared Pool is inspected, as "rights whose bound names it" (open question 14).
- In the concise form, one line per holder: `shares: {skills: [file.read.v1, directory.list.v1]}`.

WHAT A PROJECT MAY DO WITH A SHARED POOL, through a manager holding the project's home and a share
- Open Conduits from its own members to the shared Pool's Resources, listing operations the share names.
- Close those Conduits (it holds the consumer's home), and inspect them (it may inspect through either end's authority).
- Pass a narrower share on, but only to actors whose home it holds. ACC-030's within rule plus the consumer-home rule make sharing non-transitive across projects, like RAM's "can't share a resource that's shared with you".

WHAT IT MAY NOT DO
- Register into the shared Pool, set it up, retire it, inspect it or its Resources. Those stay with the Pool, the Pool's home, or `management`.
- Open a Conduit listing an operation outside the ceiling.
- Open a Conduit to the Pool for another project's members.
- Close another project's Conduits into the shared Pool. This is the overlap hazard of today's ACC-012, removed for Pools homed in a project.
  - It is not removed for engine-level Pools, which keep today's rules for backward compatibility. Hence the advice to home every shared Pool in a project such as `commons`.

THE OWNER'S TWO EXAMPLES
- Skills Pool for all projects: Pool `skills`, homed in project `commons` (or engine-level). root or a `commons` manager adds Resources. Every project manager receives a `share-skills` right with read and list.
  - "All projects" is explicit per holder: a new project's managers get a new share. Nothing silently covers future projects (prior-art lesson 4).
- Auth Pool for one project: Pool `auth`, homed in `conduit-core`, with no shares. Only holders of `conduit-core`'s home can open Conduits into it.
  - Secret wiring stays root's: SEC-008 is unchanged, so only root opens `linear -> linear-key [get-secret.v1]`.
  - Weakest create right: anyone holding `conduit-core`'s register and home can create an Agent and open it to `linear`. If fewer people should reach auth, home it in its own project.

WHEN EITHER SIDE RETIRES
- The shared Pool, or its home project, retires: CON-008 closes every Conduit into it, from every project. Shares then name a retired Pool and cover nothing new (LIF-019, IDs not followed). Nothing blocks the retirement (LIF-023).
  - Whether a Pool that other projects use should block its home's retirement, as PRV-020 and PLC-011 do, is open question 5.
  - The plan (ruling 53, decision 4) lists every Conduit that will close, other projects' included.
- A consumer project retires: its members' Conduits into the shared Pool close as the members retire. The shared Pool is untouched. Shares held by that project's engine-level managers stay until someone closes them: an orphan, as revocation does not cascade (ACC-030).

## authority
THE AUTHORITY OF AN OBJECT, A(x)
A(x) is x's home Project, where x has one that is not retired. Otherwise it is `management`. That covers engine-level objects, Projects themselves, the first-start objects, and members of a retired Project.

ACC-033's single-authorizer rule stays: in each row below, one target decides, or two where today's row 1 already needs two. For an engine with no Projects, A(x) is always `management` and every row reads exactly as today.

NEW ROUTING TABLE (extends ACC-033; the numbers are today's rows)
| # | Request | Authorized against (the Conduit must list the operation) |
|---|---|---|
| 1 | register.v1 of a Resource into an existing, live Pool Q | The submission's authority (P if the manifest names an existing Project P, otherwise `management`) AND Q. Where Q is P itself, one Conduit (S1's T2: s1 alone, where today it needs `management` and s1) |
| 2 | register.v1 of anything else | P for a submission naming an existing Project P. `management` for no project, or where the submission makes P itself |
| 2' | replace.v1 of a template; unregister.v1 of a template or retired instance | A(the object) |
| 3 | register-replacement.v1 | As 1 or 2 for the instance it makes |
| 4 | set-up.v1 / retire.v1 of a Resource | Its owning Pool, unchanged; that Pool may be a Project |
| 5 | set-up.v1 / retire.v1 of an Agent, Interface or Pool | A(x). For a Project: `management` |
| 6 | inspect.v1 of a Resource | Its owning Pool; A(that Pool) once the Pool is retired |
| 7 | inspect.v1 of an Agent, Interface, Pool or template | A(x). A Project is inspected through itself (new: its inventory is its managers' view) |
| 7' | inspect.v1 of a Conduit | A(the Conduit), A(its consumer) or A(its target), whichever the caller holds. This is the one deliberate "any of" in the table: both sides of a cross-project Conduit must see it. A non-existent object stays `management`'s, refused before any diagnostic (OPS-023) |
| 8 | retire-with-resources.v1 | The Pool itself. For a Project it retires every member (LIF-021 extended) |
| 9 | install.v1, declare-lost.v1, purge.v1 | `management`, unchanged (reserved operations stay root's, ACC-032) |
| 10 | open.v1 / close.v1 | `access`, under the home rules below |
| 11 | a control | The instance, unchanged |
| 12-14 | reconcile.v1 | `management` becomes A(x), x being the instance the invocation concerned, as recorded. Pool-routed cases are unchanged |

OPEN AND CLOSE (ACC-006, ACC-012, ACC-030, with the home/share idea from the sharing field)
- Open (ACC-006): unchanged, the right the request travels through must cover the target and name every operation. Added: where the consumer is a member of a Project, some open right of the actor must hold that Project's home.
  - An engine-level consumer keeps today's rule, "for any consumer".
  - Recommended small addition: a Conduit whose consumer is root is opened only by root, since ACC-024 makes it unclosable.
- Close (ACC-012): for a Conduit with an end in a Project, the actor needs a right that names every operation the Conduit lists and holds the home of its target or of its consumer. Either side may narrow; a share alone never closes. For Conduits whose ends are both engine-level, today's rule stands. Root's Conduits are never closed (ACC-024).
- Rights to open (ACC-030):
  - Open: the new bound must be within the actor's, as today, and the actor must hold the consumer's home.
  - Close: within, and the actor must hold the consumer's home or the home of a Project the bound names. So `packages`' managers can close the coordinator's right that names `packages`, but not its `share-skills`.
- CON-028 is unchanged: a bound still lives only on a Conduit to `access`.

ROOT
- Unchanged: root-management (all 11 operations) and root-access (unbounded, so it holds every home) (FST-002, ACC-022).
- Root creates Projects (row 2 through `management`), installs packages, and alone wires get-secret (SEC-008) and the reserved operations.
- Because one target decides, root still needs a Conduit to each Project to act inside it, as it needed one to each Pool. In the concise form that Conduit is derived (rule D2, now `<project>--manage`, listing the nine operations of project-management.v1). ACC-024 makes it permanent until the project is retired.

HOW AN INTERFACE GETS AUTHORITY OVER ONE OR MORE PROJECTS
For each project P, two ordinary Conduits:
- (a) X -> P, from a template whose capability is `core#project-management.v1`: register, register-replacement, replace, unregister, inspect, set-up, retire, retire-with-resources and reconcile, or any subset. This is "manages P" for objects.
- (b) X -> `access`, whose bound names P in `pools`, with the operations X may grant inside P. This is "acts inside P", and it holds P's home.
  - Granting other people management of P needs the nine project-management operations in this bound too. A manager who should not create co-managers gets a bound without them.
- Over three projects, X holds three such pairs. Each pair is revocable by that project's managers: (a) because they hold P's home and name its operations; (b) because its bound names P.
- Both Conduits may be registered in P's own submission, because (a)'s target is P and (b)'s bound names only P. So P's file lists its managers, and they appear in P's inventory. That is the safe reading of "projects determine an Interface's permissions".

A SECOND PERSON WITH AUTHORITY OVER ONE PROJECT
1. root registers an engine-level Interface `collab` (`from` a template of type core#interface.v1).
2. root registers and opens collab -> packages (all nine operations) and collab -> access with bound {pools: [packages], operations: [the operations used in packages and the nine]}.
3. Optionally, root opens collab -> access `share-skills`.

collab can then:
- register, set up, retire and inspect members of `packages`, with no `management` at all (gaps B1, B2 and B6 are closed for members);
- open Conduits among `packages` members, and from them into skills, within read and list;
- see only `packages` and the Conduits that touch it (B7 is closed per project).

collab cannot:
- touch `conduit-core`;
- install packages;
- open get-secret Conduits;
- close root's Conduits;
- create Projects.

How collab's person gets a token is DL13, and the first build issues none to a second Interface. That is unchanged here and is a precondition.

DELEGATION AND BOUNDS
- Delegation still only narrows (ACC-030). collab may make the Agent pkg-lead a sub-manager: pkg-lead -> packages [register.v1, inspect.v1] and pkg-lead -> access {pools: [packages], operations: [file.read.v1, directory.list.v1]}.
- A bound naming a Project covers that Project's future members. This closes gap B3, the bound frozen at instantiation, for project members.
- A bound naming `system` remains near-root, as today. A Project never contains `system` or any engine-level object, so naming a Project never covers them.
- Revocation does not cascade, as today.
- Two managers of one project can still close each other's grants inside it. Across projects they cannot.

## approvals
PRINCIPLE
Approval is not a separate power. A change that widens access needs an actor who holds the authority to make it.
- An actor that holds it acts, and its request is its own approval, recorded as its caller (REC-002).
- An actor that lacks it can only propose. The proposal is the existing register/open split: registering is authorized by the Project, and opening needs a covering right to open.
- So "who may approve is declared" through the same Conduits that declare authority (X -> P plus a right naming P), shown in the Project's inventory.
- No one approves beyond their own authority (ACC-030, the Kubernetes escalate/bind rule), and no separate approver list can drift from authority.

WHAT A PROPOSAL IS (one new promise)
1. A Conduit registered and not open: an `open` proposal.
2. An instance registered and not set up: a `set-up` proposal. Its declared Conduits open at setup, authorized by the setting-up actor's rights (ACC-005).

Either is raised as a pending action of the mechanism of rulings 50 and 51 (kind, subject, evidence, outcome), with:
- subject: the object's ID;
- evidence, written by the engine:
  - its consumer, target, capability, operations and filters, or its template and declared Conduits;
  - its registrant and caller chain, from the register invocation's record;
  - its project;
  - the consumer's tools before and after, from ruling 53's inventory;
- outcome: open, set up, or closed and retired.

ROUTING
- The pending action is shown at every Interface that could perform it now:
  - for an open: holds a right covering the target with the operations named, and holds the consumer's home;
  - for a set-up: holds a Conduit to A(x) listing set-up.v1.
- root always qualifies.
- Agents are not shown pending actions; the owner's "through the Interface", ruling 50's control panel. An Agent that holds the authority could still perform the change, since preservation test 5 says management is open to an agent under a grant. Its bound is its standing approval: the "envelope" of UC4 and UC9 is just its right to open.
- Disclosure: an Interface sees a pending action only where it may inspect the subject under rows 7 and 7' (ruling 52).

APPROVING AND REJECTING
- Approve: the approver's own `open.v1` or `set-up.v1`, with ruling 53's `expectedId` set to the proposal's ID.
  - It is decided anew (ACC-036) against the approver's rights.
  - A swapped or re-registered subject reports conflict, so the plan approved is the plan run.
  - The request is recorded with the approver as caller; the registrant is recorded on the register invocation.
- Reject: `close.v1` (for a Conduit) or `retire.v1` (for an instance), by anyone holding a home of an end. Both are narrowings, so they are immediate and leave a ruling-53 tombstone.

SELF-APPROVAL
A proposer who could perform the change never needed approval. A proposer who could not cannot approve, because approving is performing. "No self-approval" therefore holds by construction, with one caveat: if someone later widens the proposer's own rights, it may open its own earlier proposal. That is acting within its new authority, not approving.

PLAN AND APPLY (ruling 53 decision 4)
- The CLI's plan lists each step's authority.
- For an operator who holds everything (root in S1), apply sends the steps and nothing is pending. This is why S1 gains no lines.
- For an operator who lacks a step, for example the coordinator applying a worker that needs Linear, apply registers that step and reports it as pending, and waits for nothing (ruling 38, ACC-019: no work outlasts its invocation).
- The confirmation `y/N` stays the operator's own consent, not an approval of anyone else's proposal. `--yes` never opens another actor's proposal.

SIGN-INS AND ENROLLMENT
- v0 is unchanged: every sign-in and admission goes to root (rulings 49 and 50).
- This design makes the routing possible for later: a sign-in for an Agent in P could go to the Interfaces holding P's set-up.v1, because every Agent now records its project (open question 9).
- Enrollment concerns nodes, which are engine-level, so it stays with root and holders of `management`.

NARROWING NEVER WAITS
close.v1, retire.v1, retire-with-resources.v1 and rejecting a proposal each take effect when dispatched (ACC-011, LIF-005). Who may narrow: any actor holding the home of either end with the operations named, or the object's authority. A pending widening never holds back a narrowing, because it is just a registered object.

NOT PROVIDED (see what is lost)
- an expiry on proposals;
- two-party approval, where both projects must approve;
- a quorum;
- classifying configuration or control changes, such as `accountServices: true` or replace.v1 of P's templates, as widenings.

## examples
Canonical documents below omit `apiVersion: github.com/shrav-k/conduit/v0`, which every document carries. Qualified operation names are shortened where marked (`fs#` = example.org/s1/filesystem, `core#` = github.com/shrav-k/conduit/core). Concise forms are illustrative only; the parallel short-form investigation owns the syntax.

=== 1. S1 AS ONE PROJECT ===

Concise (the ruling-53 package's file, unchanged):
```yaml
conduit: v0
project: s1
packages: {...}                    # TOOL-written by conduit install
resources:
  shared: {type: fs#local-directory.v1, config: {root: <lab>/shared}}
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

Compiled canonical, abridged:
```yaml
# conduit.yaml
project: s1                         # NEW MAN-004 key: every document below is a member of s1
packages: {...}
---
kind: Pool                          # the Project, made in this submission, so row 2: management
metadata: {name: s1}
from: core#project
---
kind: Resource
metadata: {name: shared}
from: shared                        # one-off template, also a member of s1
owner: s1                           # the Project owns it directly: no separate default Pool
---
kind: Conduit
metadata: {name: s1--manage}        # derived, rule D2; replaces root -> Pool s1
from: s1--manage                    # capability core#project-management.v1, all nine operations
consumer: {kind: Interface, name: root}
target: {kind: Pool, name: s1}
# turns (derived, rule D3) and the two `as project` grants are as in the synthesis
```

The plan differs from the synthesis in three lines:
- `+ Project s1 (core#project)` replaces `+ Pool s1 rule D1`;
- `s1--manage project-management.v1 [9 ops]` replaces `s1--admin pool-management.v1 [5 ops]`;
- set-up of agent-a and agent-b is authorized by s1 (row 5), not `management`.

Effects on later tasks:
- T2: history is registered into s1 through s1--manage alone (row 1, Q = P).
- T7: the tombstone is keyed (project s1, consumer agent-a, local name project).
- T3: `conduit reach` or `inspect s1` lists members, Conduits and rights.

S1 lines per task (paper counts, ruling 45):
| | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
|---|---|---|---|---|---|---|---|---|
| Baseline A70 | 33 | 16 | 0 | 8 | 3 | 2 | 2 | 64 |
| This design, concise | 16 | 6 | 0 | 2 | 1 | 1 | 1 | 27 |

The counts are identical to the ruling-53 package: `project: s1` was already line 2, and the Project replaces derived objects one for one. Root's own Conduits stay derived (0 author lines).

Canonical only, if the concise form is not what is counted: +1 line (`project:`) in each submission's manifest. In T1, roughly 6 lines fewer, because the Pool is made `from: core#project` with no Pool template. Roughly: coordinator T1 154 -> about 149, T2 73 -> 74, T5 to T7 +1 each. Estimates, not recounted.

=== 2. THE OWNER'S REAL USE: conduit-core and packages, skills shared, auth for conduit-core only ===

Root's engine-level submission (manifest without `project`, authorized by `management`):
```yaml
kind: Pool, metadata: {name: commons},      from: core#project
kind: Pool, metadata: {name: conduit-core}, from: core#project
kind: Pool, metadata: {name: packages},     from: core#project
---
kind: Interface
metadata: {name: collab}                    # engine-level person; see example 3
from: person                                # template: spec.type core#interface.v1
---
kind: Conduit
metadata: {name: manage-project}            # template
spec:
  capability: core#project-management.v1
  operations: [core#register.v1, core#register-replacement.v1, core#replace.v1, core#unregister.v1,
               core#inspect.v1, core#set-up.v1, core#retire.v1, core#retire-with-resources.v1, core#reconcile.v1]
---
kind: Conduit
metadata: {name: share-skills}              # template: the skills share, read and list only
spec:
  capability: core#access-control.v1
  operations: [core#open.v1, core#close.v1]
  requirements:
    bound:
      requirement: core#bound.v1
      arguments: {pools: [skills], operations: [fs#file.read.v1, fs#directory.list.v1]}
---
# instances: root -> commons, conduit-core, packages (manage-project), derived in the concise form
kind: Conduit, from: share-skills, consumer: {kind: Interface, name: collab}, target: {kind: Resource, name: access}
kind: Conduit, from: share-skills, consumer: {kind: Agent, name: coordinator}, target: {kind: Resource, name: access}
kind: Conduit, from: right-in-core, consumer: {kind: Agent, name: coordinator}, target: {kind: Resource, name: access}
#   right-in-core: bound {pools: [conduit-core], operations: [file ops, turn.v1, the nine]}
```

commons (`project: commons`):
```yaml
kind: Pool, metadata: {name: skills}, from: skills-pool        # spec.type core#pool.v1; home commons
kind: Resource, metadata: {name: skills-dir}, from: skills-dir, owner: skills   # local-directory, root ~/skills
```

conduit-core (`project: conduit-core`):
```yaml
kind: Pool, metadata: {name: auth}, from: auth-pool            # the project-only auth Pool
kind: Resource, metadata: {name: linear-key}, from: op-secret, owner: auth   # 1Password item in the test vault
kind: Resource, metadata: {name: linear}, from: linear-mcp, owner: auth      # remote; header from linear-key
kind: Conduit, from: read-secret, consumer: {kind: Resource, name: linear}, target: {kind: Resource, name: linear-key}  # get-secret.v1: root opens (SEC-008)
kind: Resource, metadata: {name: core-tree}, from: core-tree, owner: conduit-core
kind: Agent, metadata: {name: coordinator}, from: coordinator  # claude-code
kind: Conduit, from: linear-rw,  consumer: coordinator, target: linear     # list, get, create, update
kind: Conduit, from: tree-rw,    consumer: coordinator, target: core-tree
kind: Conduit, from: skills-ro,  consumer: coordinator, target: skills-dir # crosses into commons; home conduit-core
kind: Agent, metadata: {name: triage}, from: triage            # codex
kind: Conduit, from: linear-ro,  consumer: triage, target: linear          # list, get
```

packages (`project: packages`):
```yaml
kind: Resource, metadata: {name: pkg-tree}, from: pkg-tree, owner: packages
kind: Agent, metadata: {name: pkg-worker-1}, from: worker      # codex
kind: Conduit, from: tree-rw,   consumer: pkg-worker-1, target: pkg-tree
kind: Conduit, from: skills-ro, consumer: pkg-worker-1, target: skills-dir
# collab's authority, registered in packages' own file (target is packages; the bound names only packages):
kind: Conduit, from: manage-project, consumer: {kind: Interface, name: collab}, target: {kind: Pool, name: packages}
kind: Conduit, from: right-in-packages, consumer: {kind: Interface, name: collab}, target: {kind: Resource, name: access}
#   right-in-packages: bound {pools: [packages], operations: [file ops, turn.v1, the nine]}
```

Concise, illustrative: three project files plus root's, adding these lines over today's form:
- `project:` once per file;
- `manages: [packages]` under collab, which derives the two Conduits;
- `shares: {skills: [file.read.v1, directory.list.v1]}` per holder.

Outcomes:
- Refused:
  - pkg-worker-1 -> linear: no right covers auth.
  - collab or coordinator opening anything into skills listing file.write.v1: share-skills names read and list only.
  - collab closing coordinator -> skills-dir: collab holds neither conduit-core's nor commons' home. Today's ACC-012 would permit this.
- Retiring packages (root, retire-with-resources.v1 on packages):
  - pkg-worker-1 and pkg-tree are retired;
  - pkg-worker-1 -> skills-dir and collab -> packages close;
  - skills, commons, conduit-core, auth and collab itself are untouched;
  - collab's right-in-packages now names a retired Pool and covers nothing new (open question 6).

=== 3. A SECOND INTERFACE LIMITED TO ONE PROJECT: collab over packages ===

Permitted, each with no Conduit to `management`:
- register (packages submission) Agent pkg-worker-2 from `worker`, a Resource pkg-tree-2 owned by packages, and Conduit pkg-worker-2 -> pkg-tree-2 (row 2, packages);
- set up both (rows 4 and 5, packages);
- open pkg-worker-2 -> pkg-tree-2 [read, list, write] (target covered and operations named by right-in-packages; consumer home held);
- open pkg-worker-2 -> skills-dir [read, list] (through share-skills; consumer home held through right-in-packages);
- inspect packages (inventory: members, managers, rights naming packages);
- close any Conduit with an end in packages whose operations its bound names;
- make pkg-lead a sub-manager (example 4).

Refused:
- inspect coordinator: refused before any diagnostic (row 7, conduit-core);
- register into conduit-core, or a Resource owned by skills;
- retire skills;
- install.v1;
- open linear -> linear-key (SEC-008);
- close root's s1-style manage Conduit to packages (ACC-024);
- open a right to open for coordinator naming packages: the consumer's home is conduit-core, so only root can;
- create a Project;
- `declare-lost.v1` and `purge.v1`.

=== 4. AN AGENT PROPOSING A CHANGE: pkg-lead ===

pkg-lead (an Agent, home packages) holds two Conduits, both opened by collab:
- pkg-lead -> packages [register.v1, inspect.v1];
- pkg-lead -> access {pools: [packages], operations: [fs#file.read.v1, fs#directory.list.v1]}, its envelope.

Steps:
1. pkg-lead registers, in a packages submission, Conduit `w1-write`: pkg-worker-1 -> pkg-tree [file.write.v1]. Authorized by packages (row 2). State: registered.
2. pkg-lead's open of w1-write reports refused (its bound does not name file.write.v1), naming the bound.
3. The engine raises a pending action:
   - kind `open`; subject w1-write's ID; project packages;
   - evidence: consumer, target and operations; registrant pkg-lead; tools of pkg-worker-1 before and after;
   - shown to root and collab, the Interfaces whose rights could open it.
4. collab sends open.v1 {conduit: w1-write, expectedId: <ID>}. It is decided anew and recorded with collab as caller. pkg-lead learns the outcome on its next inspect; nothing wakes it.
   - Or collab sends close.v1, which removes w1-write and leaves a tombstone.
5. Had pkg-lead proposed pkg-worker-1 -> linear [list], registering is still allowed (consumer in packages), but only root can open it: no one else covers auth and holds packages' home. collab is not shown it, since it may not inspect linear's side... collab may inspect it through row 7' (consumer in packages), but collab could not perform it, so it is routed to root only.
6. Narrowing: pkg-lead may close pkg-worker-1 -> pkg-tree [read, list] at once. Once w1-write is open, pkg-lead cannot close it, because its bound does not name write.
7. A swap between proposal and approval (w1-write closed and re-registered under the same name) makes collab's open report conflict on `expectedId`.

## spec_plan
ADDED (about 5 promises, in pools.yaml as POOL-018 onward, or a short projects section)
- POOL-018, home: every template and instance a submission makes has as home the Project its manifest names, or none.
  - The engine stores it as `project`, fixed for life, returned by inspect.v1.
  - A Resource's home is its owner if the owner is a Project, otherwise its owner's home. Declared Conduits and entries share their holder's home.
  - A Project's home is none: projects are flat.
  - The first-start objects, nodes and Pools owned by `system` have none.
- POOL-019, what a project's submission may hold: Resources owned by P or by a Pool homed in P; standalone Conduits with an end that is P or homed in P, or a Conduit to `access` whose bound names nothing outside P. Otherwise rejected (invalid).
- POOL-020, only a Pool whose Type is core#project.v1 is a home. Its retirement rules for members: LIF-020 and LIF-021 extended, and registration into a retired Project rejected for every family.
- ACC-040, home and share: an actor holds the home of an object as the authority field defines. This is used by ACC-006, ACC-012 and ACC-030.
- One pending-action promise (proposal = registered Conduit not open, or registered instance not set up; routing; outcome). It goes in the batch that writes rulings 50 and 51's pending action, which is not in spec/ yet.
- Optional: a Conduit whose consumer is root is opened only by root.

CHANGED (about 17)
- ACC-033: introduce A(x). Rows 1, 2, 3, 5, 6 (retired fallback), 7 (a Project inspected through itself, and row 7' for a Conduit through any end's authority) and 12 to 14 substitute A(x) for `management`. Row 8 covers a Project's members. The observable gains a sentence: with no Projects, every row reads as before.
- ACC-029: naming a Project covers it, its members, and the Resources of Pools homed in it, including later ones.
- ACC-006: the consumer's home where the consumer is a project member. ACC-005's observable gains the same condition for the parts of a setup.
- ACC-012: home rule for closing Conduits with an end in a Project.
- ACC-030: consumer home on open; closing a right needs the consumer's home or the home of a Project its bound names. "For any consumer" is limited to engine-level consumers.
- CTR-004: a Pool Type offers pool-management.v1, or, for core#project.v1 alone, project-management.v1.
- MAN-004: the `project` key. PKG-017 or MAN-011: a package's manifest states none.
- OBJ-004: `project` is an engine-owned field.
- INS-025: `from` of a project member resolves only to a template of its Project, an engine-level template or a package's.
- PRV-006: `runtime` of a project member names a provider homed in its Project or engine-level.
- LIF-020, LIF-021: Projects retire with members of every family.
- POOL-004, POOL-017: a retired Project accepts no registration.
- SUB-009: replace.v1's manifest `project` equals the template's.
- LIF-013 and inspect.v1's outputSchema: `project`.
- definitions.md:
  - new rows: project (the registered object), home, share, project-management.v1;
  - changed rows: Pool, owner, bound, `management`, submission.
- The old meaning of "project" (documents plus manifest) is renamed. Recommended: "submission", which already exists in definitions.md:67. This is about 300 occurrences in about 45 files, editorial only.

REMOVED: none.

FIRST START: no change.
- FST-001 to FST-006 stand. First start makes no Project. FST-003 already forbids any other Pool.
- Every first-start object is engine-level, and no Project can ever contain one.
- The new core template `project` is not a first-start template, and FST-006's list is unchanged.
- No promise needs a default or system project. `system` stays the engine's Pool, never a home.

CORE CONTRACTS AND SCHEMAS
- types.yaml: Type `project.v1`, family Pool, `offers: [project-management.v1]`, config {}.
- capabilities.yaml: `project-management.v1` = register.v1, register-replacement.v1, replace.v1, unregister.v1, inspect.v1, set-up.v1, retire.v1, retire-with-resources.v1, reconcile.v1. There is no install.v1 and no reserved operation.
- templates.yaml: Pool template `project`.
- Optionally a Conduit template `manage-project` listing all nine, as `root-management` lists its eleven.
- schemas/manifest.yaml: `project`, a name under OBJ-001.
- inspect.v1 outputSchema: `project`.
- bound.v1: no schema change; only its comment and ACC-029.
- No new operation: everything reuses register, replace, unregister, inspect, set-up, retire, retire-with-resources, reconcile, open and close.

THE FIVE ADDITIONS OF RULING 53
- `as`: unchanged. It keeps engine-global object names out of every tool name, which is what makes keeping global names tolerable.
- expectedId on open.v1 and close.v1: becomes the binding of an approval. The approver opens the proposal by its ID, and a swap reports conflict.
- Inventory in inspect.v1, now per project:
  - inspecting a Project lists its members, the Conduits whose target is the Project, and the rights whose bound names it;
  - inspecting any object lists the Conduits of which it is an end, each shown only if the caller may inspect it (row 7');
  - no counts of the rest.
  - One extra sentence lets a Pool's inventory list the rights whose bound names it, so shares are visible.
- Tombstones, per project:
  - each mark carries the project of what it marks, and is keyed (project ID, consumer, local name) for Conduits and (project ID, kind, name) for instances;
  - enforced at open and register as accepted;
  - listed by inspecting the Project;
  - kept as engine state after the Project is removed.
- Descriptive tool descriptions: unchanged; may add the target's project name.

## cost
SPECIFICATION
- About 17 promises changed and 5 added, about 22 of roughly 455 to 470, or 5%.
  - For comparison, ruling 53's core additions are about 11 to 14 (and still unwritten). Design D was about 50 and design C about 80.
  - The pending-action promise is shared with the batch that writes rulings 50 and 51.
- 3 core contracts: a Type, a Capability, a template; optionally a 4th, `manage-project`.
- About 8 definition rows. One manifest schema key and one output field.
- If the old term is renamed: an editorial pass over about 300 occurrences of "project" in about 45 files, mechanical, no behavior change. If the new object takes another word, this cost is zero (open question 1).

VECTORS AND CONFORMANCE
- No existing vector should change. With no Project, A(x) is `management` and every routing, coverage and close rule reads as today.
  - ACC-033's 78 conformance files keep their meaning. That figure is quoted from map-mechanisms.md, not recounted.
- New vectors: about 30 to 40, covering:
  - each substituted ACC-033 row with a Project;
  - row 7' inspection from either end;
  - home versus share for open, close and delegation;
  - Project coverage of later members;
  - POOL-019 rejections;
  - `from` and `runtime` confinement;
  - retire-with-members order and failure hold-back;
  - registration into a retired Project;
  - the manifest key and the replace.v1 project check;
  - proposals and their routing.

REFERENCE MODEL (Rust, crates/core and loader)
- a `project` field in the store;
- the manifest key in the loader;
- an `authority_of` function in access.rs, which replaces the hard-coded `management` routing;
- the holds-home check across the actor's rights to open;
- coverage of Projects;
- retire-with-members, generalizing LIF-021's ordering;
- the inventory per project;
- proposals as pending actions.

Estimate: about 1,000 to 1,800 core lines plus tests. Paper estimate, not checked against the code beyond access.rs:445-455.

CLI AND SHELL
- The ruling-53 compiler derives a Project in place of a Pool. The operator Conduit lists project-management.v1. A small change.
- New concise lines `manages:` and `shares:`, each deriving ordinary Conduits shown in the plan: about 200 to 400 lines.
- The plan shows each step's authority and reports steps it could only register as pending.
- Issuing tokens to a second Interface (DL13) is needed before example 3 can run. It is not costed here, as it is separate work.

WORK ORDERS: about 3 to 4.
- spec batch;
- core routing and coverage;
- loader, store and inventory;
- CLI derivations.
Sequencing: after the short-form investigation decides the form, so ruling 53's additions and this design are written once.

S1: no change in the concise form (27 lines). Canonical: +1 line per submission manifest, about −6 in T1.

## what_is_lost
This list says which use cases are not served, or only partly served, and what the design gives up for being minimal.

1. Names per project (UC8, not served).
   - Names stay unique across the engine (OBJ-002). Two projects cannot both have `shared` or `worker`.
   - A name collision reports conflict and so reveals that the name exists in another project, though not what holds it. Gateway API's "MUST NOT expose existence" holds for contents, not for names.
   - Decoy names are still possible inside a project. `expectedId` guards approvals, and `as` keeps names out of tools.
2. No audience declared on the Pool (UC2, UC10, partly served).
   - Sharing is one right to open per holder, opened by the holder of the Pool's home. The Pool does not list "shared with projects X, Y, and all future ones". A new project's managers each need a new share.
   - This is explicit and avoids silent coverage of future projects, but it costs a line per holder per shared Pool. The share is visible from the Pool only through the inventory extension (open question 14).
3. Ceilings are per bound, and flat. bound.v1's `operations` apply to everything the bound names, so one share per Pool is needed to give different ceilings. There is no per-target ceiling inside one bound.
4. No two-party consent across projects (UC2, UC10).
   - A cross-project Conduit, or a right for another project's member, needs one actor who holds both homes: root in practice. There is no handshake object like ReferenceGrant or a RAM invitation that lets each side consent separately and expire.
   - UC10 ("collab alone must not suffice") holds, but only because root does it. UC2's coordinator working in `packages` needs root to grant it. `packages`' managers can then close that right (served), but they cannot grant it.
5. Project registration trusts its holder with every template it may use (UC2, UC9 "only from `worker`" not served).
   - A holder of P's register.v1 may use any template of P, any engine-level template and any package's, with any configuration, including turning on an Agent's `accountServices`, and any runtime provider of P. A broker is still the only way to narrow provisioning.
   - Weakest create right: P's register plus P's home gives the combined reach of every Pool shared into P.
6. Only open and set-up are widenings. Turning on a reach flag by a control or in configuration, or replace.v1 of P's templates (new programs or `remote` URLs for later replacements in P), raises no pending action (R3 of the use cases, not served).
7. Approvals are minimal (UC4, partly served).
   - No expiry: a proposal persists until it is closed or retired.
   - No quorum.
   - No approval required from the approvers of both sides.
   - No separate approver list. Anyone who holds the authority approves, including an Agent that holds it, which is not shown the pending action but may still act.
   - If the owner wants approvers to be persons only, Agents must be given no right that covers widenings.
8. Placement and nodes are not scoped (UC5, not served). Any project may place on any node, and nodes, enrollment and the machine provider stay engine-level. Only runtime providers are confined.
9. Peer engines at project level (UC6) are served only as far as a partner's Interface can be given Conduits to one project of the peer. Introducing the peer and trusting it stay engine-wide, under ruling 51.
10. Secrets stay root's (SEC-008). An auth Pool homed in a project still needs root for every Conduit that lists get-secret.v1. Sign-ins and enrollment stay with root in v0 (rulings 49 and 50).
11. Revocation does not cascade, as today. Closing collab's rights leaves the Conduits collab opened. Two managers of one project can still close each other's grants inside it.
12. Engine-level shared Pools keep today's overlap. Holders of a share can close each other's Conduits into them. Only Pools homed in a project gain the protection of home versus share.
13. A Pool homed in a project dies with it, and others' access ends without a block (LIF-023). Moving an object between projects is impossible; it must be replaced.
14. Installing packages stays root's, through `management`. A project's managers cannot bring in a new package version.
15. Vocabulary.
   - "A Project is a Pool" puts a member that holds more than Resources into the Pool family.
   - `retire-with-resources.v1` now retires members of every family.
   - The old meaning of "project" must be renamed, or the new object needs another word.
16. Root still needs one Conduit to each Project to act inside it, because one target decides each request. It costs nothing in the concise form (derived), but it is one more document in the canonical form.

## open_questions
1. The word. Two options:
   - Call the new object "project", the owner's word and the concise heading, and rename today's sense (documents plus manifest) to "submission": about 300 occurrences, editorial only.
   - Keep today's sense and call the new object something else, at zero rename cost.
   Ruling 42's one-meaning rule forces one or the other. Recommended: the rename.
2. A Pool Type or a sixth family?
   - The Pool Type is what makes this design minimal: bound.v1, Conduit targets, POOL-001, LIF-020 and LIF-021 are all reused.
   - A sixth family reads more clearly ("a Project is not a Pool") but costs OBJ-020, every reference schema's kinds and about 28 passages on families.
   - If the Pool Type is chosen, should `retire-with-resources.v1` be renamed `retire-with-members.v1`?
3. Root's Conduit to each project.
   - Keep it, derived and recorded, so the single authorizer stays.
   - Or give root an identity power: `management` authorizes for root whatever a Project authorizes, as ACC-032 does for the reserved operations. This saves one canonical document per project and breaks "one target decides".
4. Should every Pool other than `system` be homed in a project? That would remove item 12's overlap for engine-level shared Pools. It costs nothing for S1, which has one project, but changes what a projectless engine may do.
5. Should a Pool that other projects use block its home project's retirement, as PRV-020 and PLC-011 block it for providers and nodes? Or do other projects simply lose access, with the plan listing every Conduit that will close?
6. Should a right to open whose bound names a retired Project be closed automatically? Today a bound that names a retired Pool is simply inert.
7. Who may give access to an engine-level consumer? Today anyone with a covering right may, for any consumer. Recommended at least: a Conduit whose consumer is root is opened only by root, since ACC-024 makes it unclosable.
8. The pending action (ruling 53, decision 2, still open):
   - an expiry for proposals?
   - should Agents see pending actions?
   - should a proposal also be closed when its registrant is retired?
9. Routing of sign-ins and enrollment. In v1, should a sign-in for a member Agent go to the Interfaces that hold its project's set-up.v1 (ruling 49's "more granular in the future")? Enrollment stays engine-level, unless nodes become scoped (UC5).
10. Should a project's managers be able to install packages? install.v1 grants nothing (NAM-013), but it changes what every project can reach by qualified reference.
11. Templates across projects. Is "own project, engine-level or package" the right rule for `from`, or should projects be able to mark a template as usable by others (a template share)?
12. Tombstones and a recreated name. Should a new Project with an old name see the old Project's tombstones and require an explicit `--reopen`? (Design D's skeptic.) They are kept by Project ID either way.
13. Self-declared managers. Is it acceptable that a Project's own file registers the Conduits that make Interfaces its managers, opened by its existing managers, which is this design's reading of "projects determine an Interface's permissions"? Or must every new manager come from root?
14. Showing shares. Should inspecting a Pool list the rights whose bound names it, beyond ruling 53's accepted inventory (Conduits it consumes and is the target of)?
15. Which of the use cases this design leaves unserved matter for v0? Per-project names (UC8), node scope (UC5), template-limited registration (UC2 and UC9), two-party consent (UC10) and widening through configuration (R3). Each could be added later as its own ruling on top of this design, without undoing it:
   - names: a qualified `project/name` reference, changing OBJ-002 and NAM-003;
   - nodes: a scope on node.v1 checked after place.v1;
   - template-limited registration: a `templates` argument on a right;
   - two-party consent: a consumer-side acceptance as a pending action;
   - configuration: Type-marked widening settings.

## approvals_note
unused

## Verdict authority (major)
Home Projects keeps a sound shape: one home per object, shares as separate edges, A(x) routing, a flat Project fixed for life, and a computed member list. As written, though, it is not safe. Finding 1 is fatal to its isolation claim in v0, and it hits every Project design that lets a project's managers register Agents: an Agent run by the engine's own runner can act as root. Seven more findings are high severity, mainly a "home" that any mention grants, set-up approvals that can be swapped, unbounded rights that can be smuggled in, approvals laundered through deputy Agents and Agent-made Interfaces, get-secret laundering, and references resolved by bare name across projects. Each finding has a fix that keeps the design's shape. Overall: major, salvageable, not ready to rule on.

Scope: I read spec/ in <home> and changed nothing. Main sources:
- spec/conventions/access.yaml (ACC-005, 006, 012, 016, 022 to 024, 029, 030, 032, 033, 039)
- spec/README.md:374-400 ("What the guarantee for secrets covers")
- spec/promises/first-start.yaml (FST-006)
- spec/core/templates.yaml (root-access)
- spec/conventions/lifecycle.yaml (LIF-009, LIF-021)
- spec/promises/secrets.yaml (SEC-008, SEC-010)
- spec/promises/providers.yaml (PRV-006, PRV-020)
- spec/promises/pools.yaml, conduits.yaml, objects.yaml
- spec/interface/implementation.yaml (IFC-041)
- the records of rulings 49 to 53, and the design-D and design-E skeptic traces in the simplification research.

Severities: F = fatal as written, H = high, M = medium, L = low.

1. [F] A project manager becomes root through the engine's own runner. This breaks "collab cannot touch conduit-core", and it applies to every Project design that delegates registration.
- Trace:
  - collab (or pkg-lead, which holds packages register.v1) registers an Agent in packages that gives no `runtime`, for example Codex with a shell (row 2, packages).
  - collab sets it up (row 5, packages). PRV-006 runs it on the engine's own runner, usually as the engine's operating-system user (DL16).
  - spec/README.md:386-395 says such programs "can get past every gate: they can read the engine's keys and make its proof". It also says that runs sharing one user "can present as one another".
  - ACC-027: every restriction is only mediated in this version. Placement is unrestricted, so the Agent can also read conduit-core's trees, other Agents' homes (ruling 49) and other runs' environments directly.
- Why it is new: today only a holder of `management` can register an Agent, and that holder is root-equivalent anyway. The design gives registration to project managers, Agents among them.
- Fix:
  - Add a promise: an instance registered under a Project's authority must name a runtime provider whose contract confines it (homed in P or engine-level). An instance on the engine's runner, or on a provider that does not confine, needs `management`. S1 is unaffected, because root holds `management`.
  - Say plainly that project isolation is mediated only, until confining providers and node scope (UC5) exist.
  - Enrollment and node admission must never route to project authority while placement is engine-wide.

2. [H] Any bound that names P grants P's home, and the home can come from a different right than the one that covers the target.
- Trace (the owner's real use, UC2):
  - root gives the coordinator `turn-packages` with bound {pools: [packages], operations: [turn.v1]}, so it can dispatch work to packages' workers. Under the definition of home, the coordinator now holds packages' home.
  - The coordinator registers, in a conduit-core submission, `pkg-worker-1 -> core-tree [read, write]`. POOL-019 accepts it, because the target is a conduit-core member.
  - The coordinator opens it. right-in-core covers the target; "some open right" (turn-packages) holds the consumer's home.
  - Worse, it opens for pkg-worker-1 a right whose bound is {pools: [conduit-core], ...}. ACC-030's within holds against right-in-core, and the consumer's home is held through turn-packages.
  - pkg-worker-1, a packages Agent that collab can prompt, now holds conduit-core's home. It can open Conduits for conduit-core members and close their Conduits.
  - collab cannot close that planted right. The close rule needs it to be within collab's bound, which names only packages.
- Result: collab reaches conduit-core, and neither packages' managers nor root approved anything.
- Fix:
  - The one right a request travels through must both cover the target with its operations and hold the consumer's home.
  - Home must be explicit. Either the actor also holds X -> P listing a project-management operation, or bound.v1 gains a separate `homes` argument. Naming a Pool or a Project in `pools` should only cover.
  - Closing any Conduit or right whose consumer's home the actor holds needs only that home (see 9).

3. [H] A set-up approval can be swapped after it is shown. `expectedId` binds the instance, not its parts.
- Trace:
  - collab registers pkg-worker-2. The template `worker` declares c1 -> access with bound {packages, read}.
  - collab's own setup cannot open every part, so the set-up proposal is raised to root. The evidence shows c1 bounded.
  - Before root approves, collab replaces the template that c1 is made from (replace.v1 of P's templates, authorized by P) so that it applies no bound.
  - collab closes the registered c1 (within, home held) and asks for register-replacement.v1. LIF-009 makes the new instance from the template "as it now stands".
  - root approves with expectedId = pkg-worker-2's ID, which still matches. ACC-005 opens the new c1 with root's unbounded right, so pkg-worker-2 now holds an unbounded right to open.
- Same pattern:
  - a declared Conduit to `management`;
  - a declared get-secret Conduit, which SEC-008 opens only when root sent the setup;
  - a member Pool's entries, whose replacements take the Resource template's new config or `remote` URL.
- Fix:
  - An approval carries a digest of the evidence: each part's ID, consumer, target, capability, operations, requirements, bound and template digest.
  - A setup under approval opens only the parts in that digest. Any difference reports conflict.
  - Simpler: there is no set-up proposal at all. Each widening part becomes its own open proposal.

4. [H] An unbounded right can pass as one confined to the project.
- Trace:
  - FST-006 bars `from` of system, node, management, access and root, but not core#root-access.
  - In packages, pkg-lead registers `kind: Conduit, from: core#root-access, consumer: pkg-lead, target: access`.
  - POOL-019's "a Conduit to access whose bound names nothing outside P" is satisfied vacuously, because the Conduit has no bound.
  - The evidence lists "filters", but a bound is a requirement and not a filter, so it is not shown.
  - root approves what looks like a right inside the project. pkg-lead becomes root-equivalent for open and close, apart from ACC-032 and SEC-008.
- Also: declared Conduits escape POOL-019 entirely ("shares its consumer's home").
- Fix:
  - A Conduit to `access` homed in P must apply bound.v1, naming at least one object in P and nothing outside P.
  - POOL-019 applies to declared Conduits too.
  - The evidence shows the bound and how its coverage changes.
  - A project member never receives an unbounded right through a proposal.

5. [H] Approval can be laundered through a deputy Agent.
- Trace: pkg-lead sends a turn to an Agent that holds a wider right (a worker, or the coordinator in UC9). That Agent's model calls open.v1 on pkg-lead's proposal.
- ACC-016 decides an onward request by the deputy's own Conduits, and ruling 51 decision 10 records the caller chain without using it.
- So "no self-approval by construction" fails. The owner's "approvals handled through the Interface" fails too, because any Agent with a right to open is an approver that is never shown the pending action.
- Fix:
  - A Conduit registered by another actor is opened only by an Interface acting on its own behalf.
  - At the least, an open is refused where the chain of `serving` includes the proposal's registrant.
  - An Agent's right to open opens only Conduits that the Agent itself registered, or that are declared by instances it set up.

6. [H, depends on DL13] Agents can make Interfaces, so "Interface" no longer means a person.
- Trace:
  - pkg-lead registers an Interface homed in packages (row 2, packages; the design allows member Interfaces "for an outside system").
  - collab approves its setup, and its declared right opens within collab's bound.
  - It is now an approver that is shown packages' pending actions. Under open question 9 it would also receive sign-ins, which ruling 49 reserves to persons.
  - Revocation does not cascade, so it outlives collab's own rights.
  - If DL13 issues it a credential, a project manager has made a principal, possibly bound to an outside party. Ruling 51 keeps introductions engine-wide.
- Fix:
  - Registering an Interface needs `management` (Interfaces stay engine-level), or a member Interface may receive no right to open and no project-management Conduit.
  - Pending actions and sign-ins route only to Interfaces that root marked as persons.

7. [H] get-secret laundering: the evidence omits where the secret goes.
- Trace:
  - The coordinator, which holds conduit-core's register, registers `linear2` from its own conduit-core template. The template has `remote: https://attacker`, or a `runtime` provider the coordinator registered, or a lookalike secret Resource whose locator points at another vault item (SEC-005).
  - It proposes linear2 -> linear-key [get-secret.v1]. root, the only opener under SEC-008, sees a Conduit like the existing one and approves.
  - SEC-010 and IFC-041 then deliver the value to the coordinator's provider or URL.
- Today staging this needs `management`. The design gives it to project managers.
- Fix:
  - The evidence of any proposal shows the consumer's runtime provider (its home and who registered it), its `remote` URLs and template digest, and the secret's store and locator.
  - The consumer of a get-secret Conduit must be made from an engine-level or package template, and run on an engine-level runtime provider.

8. [H] Bare names resolve across projects, and confinement only runs one way.
- (a) Name squatting:
  - conduit-core's file references `skills-dir` by bare name. Once commons retires and removes it (LIF-035 frees the name), collab registers its own `skills-dir` in packages.
  - root's next apply re-registers coordinator -> skills-dir. root holds every home, so the open needs no approval.
  - The coordinator now reads skills controlled by collab, and skills are instructions to an Agent.
- (b) Runtime: the restriction applies only to members. An engine-level instance naming `runtime: docker` can resolve to a provider homed in packages, which then receives the run's secrets under SEC-010. PRV-020 also couples the two projects' retirements.
- (c) Templates: an engine-level instance may be made `from` a template of P, and P's managers can replace.v1 it. Root's later replacements then take that template as it now stands.
- Fix:
  - The stored resolved form carries the home.
  - A bare name in a document resolves only among the same project's members and engine-level objects. A reference across projects is qualified as {kind, name, project}.
  - Engine-level instances may name only engine-level runtime providers and engine-level or package templates.
  - The plan shows each target's project, and tombstones also key the target ID.

9. [M] Narrowing can wait, or be impossible.
- packages' managers cannot close their own members' Conduits or rights to foreign targets unless their bound names the foreign operations, or is wider under the within rule. Example: collab cannot close pkg-worker-1 -> linear [list].
- The rejecter of a proposal faces the same rule, so unrejectable proposals persist with no expiry. They are loaded guns for whoever is widened later.
- The approvals text says "or the object's authority" may narrow, but routing row 10 has no such path.
- Fix:
  - Closing a Conduit, a right or a registered proposal whose consumer's home the actor holds needs only that home. Closing a proposal that was never opened needs the home of either end.
  - Proposals expire.

10. [M] An unshare does not revoke.
- Closing collab's share-skills leaves collab's sub-shares (ACC-030 does not cascade) and every Conduit opened under it.
- commons' managers cannot close shares of their own Pool. The close rule asks for "the home of a Project the bound names", and skills is a Pool, not a Project.
- Fix:
  - Holding the home of any object a bound covers is enough to close that bound.
  - Check shares when each request is decided (ACC-036), so a Conduit across projects needs a live share covering it, as Gateway API recalculates on deleting a ReferenceGrant. Or cascade the close to Conduits from that project into the Pool.

11. [M] Retirement can be raced.
- LIF-021 dispatches the Project's own retire.v1 last, so "rejected once its retirement is dispatched" leaves a window.
- Managers' Conduits close only at the end. A lower manager can keep registering members, which keeps P unretirable through LIF-020's conflict, or open new Conduits during the retirement.
- Fix: dispatching retire-with-members marks P as retiring.
  - At once, P rejects every registration, setup and open into it.
  - The Conduits to P and the rights naming P close first, then the members retire.

12. [M] A project manager can create Projects.
- Row 2 authorizes a submission naming P "by P", while a Project's own home is "always none".
- collab can therefore register `from: core#project`, or any Pool template of Type project.v1, from packages' file. That makes engine-level Projects and lets collab squat project names, contrary to "collab cannot create Projects".
- Fix: POOL-019 rejects a Pool of Type project.v1 in any submission that names a project.

13. [M] Disclosure: project B can learn that A exists.
- Registering a Conduit or a `config` reference to a foreign name succeeds or fails on existence, and the capability check reveals the target's Type.
- Through row 7', the registrant inspects its own proposal and gets the target's ID and, under ruling 53, the Type's description.
- Set-up evidence shows A's managers the objects of B that their declared Conduits name.
- Inventory: a bound that names two projects either leaks the other project or hides co-managers.
- This breaks ruling 52's first tier: "a target that does not exist gets the same answer".
- Fix:
  - A reference outside the project reports the first tier unless it is qualified and the registrant holds a right covering it.
  - The evidence and inspection show a foreign end only as "a target in another project".
  - The inventory shows only the part of a bound that covers P.

14. [M] Confused deputy through shared services.
- A Resource homed in commons, such as an index or MCP server, with Conduits into several projects serves any caller. ACC-016 decides its onward requests by its own Conduits, and no one checks the project the request serves (CVE-2021-25740 shape).
- Fix:
  - The plan and inventory flag any instance that consumes Conduits into more than one project.
  - Only root opens such a bridge.
  - Optionally, an engine-evaluated `serving-project` requirement, which reopens ruling 51 decision 10.

15. [M] Shares widen silently.
- A share's ceiling is a list of operation names, and a bound covers later Resources (ACC-029).
- A commons manager who adds `/` or `$HOME` as a local-directory Resource to skills widens every holder's reach. Registration raises no pending action.
- Fix:
  - Shares name targets.
  - Or registering into a Pool that a foreign share names raises a pending action to whoever granted the share, and the plan lists every share that comes to cover the new Resource.

16. [M] Root is exposed as a consumer and as an approver.
- "An engine-level consumer keeps today's rule", so collab can open Conduits whose consumer is root. ACC-024 makes them unclosable.
- They present tools and Type descriptions authored by packages to root's client. Here that client is an AI admin session.
- Pending-action evidence carries names, descriptions and tool lists written by the attacker.
- Fix:
  - Make "a Conduit whose consumer is root is opened only by root" mandatory.
  - Render evidence as quoted data with its provenance.
  - A person confirms approvals of proposals made by project actors.
  - Limit how fast an actor can raise proposals.

17. [L] Tombstones can be evaded.
- Keyed by (project ID, consumer, local name), a tombstone is evaded by registering the same relationship from the other end's project, or under a new `as`.
- A recreated project has a new ID, sees none of the old tombstones, and a re-apply restores revoked grants (design D finding 5 again).
- Fix:
  - Also key tombstones by (consumer ID, target ID, capability).
  - Recreating a project requires an explicit flag, or project names are never reused.

18. [L, plausible] Peers.
- A packages manager registers a Resource whose `remote` is the peer engine.
- IFC-041 sends the engine's proof, not the instance's, and the peer records the caller chain without using it. If the peer admitted "the owner's root" by the main engine's proof, packages acts as root there.
- Fix:
  - Bind peer admission to root's own credential.
  - Make a `remote` to a peer's address engine-level.

What survived my attacks:
- collab cannot close coordinator -> skills-dir.
- Retired rights are inert, because references are held by ID.
- Projects are flat, with no nesting or moves.
- Root's Conduits stay protected (ACC-024).
- ACC-032 and SEC-008 still hold for openings.
- A(x) keeps a single authorizer.
- In-flight invocations end correctly (ACC-034, ACC-039).

Not recorded in the repository: the approvals clause comes from the computed task, not from the owner's relayed words, so challenging it costs less.

Unverified:
- How DL13 would issue a credential to an Interface made by a project manager (finding 6).
- How a peer authenticates the owner's root (finding 18).

I ran nothing. Every trace is worked out on paper from the promises and the design text.

## Verdict expressiveness (major)
The design can state S1 (27 lines on paper), the owner's two Pool examples and an object-level "collab over packages". It cannot refuse what granular Interfaces exist to refuse, for three reasons. (1) Registering and setting up inside a project is root-equivalent. A project's submission may carry its own implementations, and they run on the engine's own runner. A Resource's configuration can also point at another project's files or URLs (aliasing). (2) Shares and homes attach to actors, not projects, so anyone who manages two projects bridges them. (3) Closing a share revokes nothing that was opened under it. All three can be fixed by additions, but they must land before v0 claims granular Interfaces. Finding 1 applies to every design that scopes register.v1. Severity: major, with finding 1 blocking until amended.

SKEPTIC OF EXPRESSIVENESS: the Home Projects design, checked against UC1-UC10, S2-S10 and A78. I only read the repo <home> and changed nothing.

== BLOCKING (fatal unless amended; applies to every design that scopes register.v1) ==

F1. Registering and setting up in a project is root-equivalent, so no project manager is confined to its project.
The design allows three routes:
- (a) Own implementations. A project's submission may carry contracts and `implementations`. The design keeps "contracts belong to the submission" (MAN-015, SUB-008).
  - An item reached by `command` runs on the engine's own runner for any instance that gives no `runtime` (PRV-006). That runner runs it "as the operating-system user ... usually the engine's own, which is inside the domain of trust and can make the engine's proof" (spec/definitions.md:267).
  - The design confines `runtime` to providers homed in the project or engine-level ones. It leaves "none" (the engine's runner) and placement open.
- (b) Configuration aliasing. S1's own `directory` template takes `root` as a free input into /config/root (docs/proposals/first-build/increment-4/coordinator-run/project/templates.yaml:14-17). The type local-directory.v1 has a free `path` (spec/schemas/examples/valid/files-package/contracts.yaml:209).
  - Scenario: collab holds packages' register.v1 and set-up.v1, plus right-in-packages with the file operations.
    1. In a packages submission, collab registers Resource `alias` from `directory`, with root set to conduit-core's tree, to the `main` checkout, or to `/`, owned by packages.
    2. collab sets it up (row 4, through packages).
    3. collab opens collab -> alias [file.write.v1], which right-in-packages covers.
  - Every step is permitted. The same works with `remote` URLs.
- (c) Agents from any template, with any configuration and the reach flags on. The design acknowledges this (Lost #5).

Consequences:
- Example 3's "collab cannot touch conduit-core" holds for objects, not for what they denote.
- The design cannot refuse:
  - UC3: collab acting in conduit-core;
  - UC2 and UC9: a worker writing `main`, and the coordinator registering from any template other than `worker`;
  - UC5: confinement of machines.
- In A78 the coordinator is an Agent. A prompt-injected coordinator that holds a project's register.v1 and set-up.v1 becomes root.
- The `management` row of definitions.md already says that register.v1 "trusts its consumer ... to choose the contracts and the implementation items, programs and remote URLs". The design gives that trust to every project manager without saying so.

Fix (additive; S1 stays at 27 lines because root holds both):
- (i) Templates, contracts and implementations homed in P are registered only through `management`. P's register.v1 covers instances only.
- (ii) Members of P are made only `from` templates of P. That makes P's template set its cap (prior-art lesson 8). It also delivers "from `worker` only" now, rather than in a later ruling.
- (iii) Those templates must confine configuration through input schemas, for example a `pattern` on `root`. Limiting registration to templates is not enough on its own, because `directory`'s input is free.
- (iv) A member of P may not use the engine's own runner unless its template, written by root, says so.
- (v) The evidence of a set-up proposal shows the configuration and the implementation items.

== MAJOR ==

F2. Shares and homes attach to actors, not projects, so a manager of two projects bridges them.
- How the rule composes: an open travels through one right, which must cover the target. Holding the consumer's home may come from "some open right" of the actor.
- (a) "skills shared with packages but not with experiments" cannot be stated when collab manages both. share-skills lets collab wire skills into every project it manages, and also into any engine-level consumer.
- (b) Take right-in-packages {pools: [packages], operations: [read, write]} and right-in-secret {pools: [secret-proj], operations: [read]}.
  - collab can open secret-proj-agent -> pkg-tree [write]: through right-in-packages, with the consumer's home held through right-in-secret.
  - collab can also open pkg-agent -> a secret-proj Resource [read].
  - No share authorized either flow.
  - The design's claim that a cross-project Conduit needs "one actor who holds both homes: root in practice" is false in the owner's own case of an Interface "over one or multiple projects".
- The owner's examples are Pool-to-project ("skills pool for all projects, auth pool for a specific project"). The design delivers Pool-to-actor.
- Close does not compose the same way. It needs one right that both names the operations and holds a home. An actor can therefore open a Conduit it cannot close, for example one whose operations come from a share but are absent from its home right: a widening it cannot undo.
- Fix, keeping bound.v1's shape: the one right a request travels through must cover the target and also hold the consumer's home, for open and close alike.
  - A share then names the receiving project: {pools: [skills, packages], operations: [read, list]}.
  - A right that names two projects becomes the explicit bridge.
  - Cost: one share per (Pool, receiving project, holder).

F3. Unsharing is not one narrowing, and sharing widens silently.
- Closing a share closes nothing that was opened under it (ACC-030, revocation does not cascade).
  - UC10 needs "remove packages from auth's audience: packages' Conduits to linear close at once". UC7 needs packages' place in the audiences to end.
  - Here the owner must find and close each Conduit into the Pool whose consumer is homed in packages, plus each holder's share, in the right order (shares first). That is N requests, not one.
- Orphans: after packages retires, collab's share-skills still lets it open skills Conduits for engine-level consumers, collab included.
- Silent widening: every Resource added later to a shared Pool, and every member Pool added later to a Project, becomes reachable at once by every holder of a share or home (ACC-029). No pending action is raised and no consumer consents. The design says "nothing silently covers future projects", but future Resources are covered silently.
- Fix:
  - Rule `unshare` as a plan step that is shown and ordered: close the shares, then every Conduit into the Pool whose consumer's home is that project.
  - Or rule a scoped cascade, which needs a ruling against ACC-030.
  - Either way, the plan for registering into a shared Pool or a Project lists who gains reach.

F4. Placement, nodes and the engine's runner are not scoped (S3, UC5).
- The design acknowledges this (Lost #8), but not its interaction with F1: any project manager can run arbitrary programs on any enrolled machine.
- This can be deferred only while v0 runs on the owner's own machines.

== MODERATE ==

F5. Restrictions cannot be part of a share's ceiling (S2, E07, UC10).
- Bounds limit no filters (ACC-029, ACC-030). So "packages reads Linear only for team PKG" cannot be stated, and collab may open an unfiltered Conduit.
- This was accepted for co-administrators. With projects, the gap now crosses a trust boundary.

F6. Home is all-or-nothing within a project.
- Holding P's home needs a bound that names P, and that bound covers every member, auth's Resources included.
- "A manager of conduit-core who cannot reach auth" can be stated only with a non-obvious trick: a home right with an empty operations list plus a separate covering right. The F2 fix removes that trick.
- Ceilings are flat per bound, so op lists cannot separate Resources of the same Type, such as a secrets directory and a code tree.

F7. "Manages P" does not cover the Resources of P's member Pools.
- Rows 1, 4 and 6 keep the owning Pool as the authorizer. collab -> packages therefore does not set up, retire or inspect a Resource of pkg-work. Each manager needs a further Conduit to each member Pool, and `inspect packages` leaves those Resources out.
- Yet retire-with-resources.v1 on the Project retires them.
- So B3 is closed for rights to open but reopens for management.
- Fix: for rows 4 and 6, "the owning Pool, or A(that Pool)". Or advise that Projects own their Resources directly.

F8. Proposals are inferred from state.
- "Registered and not open" and "registered and not set up" also describe:
  - the window between register and open in every apply (ruling 53's plan and apply) and in every A78 dispatch;
  - a setup that failed partway or whose outcome is unknown (S4);
  - a staged object.
  Each of these raises a pending action at root, and an approver can race a retry.
- No bundle: a worker with four standalone Conduits means four independent approvals, so partial approval is possible.
- The evidence leaves out:
  - the bound of a proposed right to open (it lives in the template's requirements);
  - the configuration and implementation of a proposed instance;
  - what a named Pool's coverage gains between proposal and approval (`expectedId` binds the Conduit, not its coverage).
- A proposer cannot withdraw a proposal that lists operations outside its own bound, because close.v1 needs them.
- There is no expiry, and nothing voids a retired proposer's proposals (UC4's must-refuse).
- Fix:
  - raise the action from an explicit marker, or from the refused open itself;
  - add a group ID;
  - complete the evidence;
  - let a registrant close its own unopened registrations;
  - add an expiry.

F9. Engine-level consumers keep "for any consumer".
- Any project manager can open Conduits from any engine-level Interface or Agent (other people's, a CI system's, root's engine-level Agents) into its project or a shared Pool. Their content then reaches another party's agent without that party's consent (prior-art lesson 6; UC2's must-refuse).
- A manager can also open X -> P [the nine operations] for any engine-level X, making a half-manager without root.
- This is inconsistent with rights to open, which need the consumer's home.
- Fix: opening for an engine-level consumer requires covering that consumer.

F10. A project's identity is a name in the manifest.
- A file stating `project: packages` joins whatever Project now has that name, including one recreated with a new ID. Whether old tombstones bind it is open question 12.
- So the engine refuses neither of these: UC1's "adopting by name" and UC7's "recreating the name to reopen revoked grants".
- Fix: an optional `expectedId` on the manifest's `project`, written by the CLI as pins are (ruling 53, decision 5).

F11. Retiring a home project while other projects use its Pools is not refused (UC7; open question 5).
- retire-with-resources.v1 on a Project is authorized by the Project alone. Any manager holding that one operation can remove `commons`, and every shared Pool in it, from every project.
- Fix: a block in the manner of PRV-020 and PLC-011 while other projects' Conduits are open, unless a flag is given.

F12. Names stay global: UC8 is not served, which is acknowledged. Not acknowledged:
- A project manager can squat a name engine-wide, for example a future project's name, a derived `<project>--manage`, or a template name another project's file uses. That project's next apply then reports conflict.
- Registering a reference to another project's object discloses that it exists, and its kind (ruling 52; Gateway API's MUST NOT).
- Templates cannot be shared across projects (INS-025). A78's single `worker` template must be either engine-level, where only root replaces it and does so for both projects, or duplicated under two global names.

== MINOR ==

13. Root-only powers remain:
- secrets (SEC-008), so "an auth pool for a project" needs root for every consumer and every rotation;
- install (S7);
- sign-ins (v0).
All are acknowledged.

14. Runtime providers can be shared only at engine level (PRV-006 as confined), not from `commons`. A shared runtime for S10 therefore keeps the overlap hazard the design advises against.

15. The design does not say that a Pool of type core#project.v1 inside a project's submission is rejected. If such a document were accepted, it would be an engine-level object authorized by P. POOL-019 should reject it.

16. Approvers cannot be narrower than authority. Root is always shown every pending action. "collab approves, root only on escalation" cannot be stated, and "persons only" can be stated only by withholding rights.

17. Moving an object to another project is impossible. Promoting a Pool to `commons` means recreating it, its Resources and every consumer's Conduits.

18. The Project's inventory does not list:
- its shared-in Pools;
- its members' Conduits across projects;
- the shares its managers hold.
"References every object" needs the CLI to aggregate member inventories.

19. Tombstones, kept as engine state, have no removal operation (S9; purge.v1 does not touch them).

20. "27 lines" is a paper count (ruling 45).

21. Example 4, step 5 contradicts itself. The rule should read: an action is shown only where the Interface could perform it AND may inspect it.

== COVERAGE ==

| Case | Served? | Gaps |
|---|---|---|
| UC1 (S1) | Served | F10, F19 |
| UC2 (A14, A78) | Partly | F1 is blocking; F2; the coordinator needs root to act in packages (acknowledged) |
| UC3 | Object-level only | F1, F2; co-managers can close each other's grants; sign-in routing deferred |
| UC4 | Partly | F8; controls and configuration are not widenings (acknowledged) |
| UC5 | Not served | F4 |
| UC6 | Partly | Trust stays engine-wide |
| UC7 | Mostly | F3 (orphans), F10, F11 |
| UC8 | Not served | F12 |
| UC9 | Partly | The envelope works; F1(ii, iii) for "from `worker`" and tree paths; F12 for templates |
| UC10 | Partly | Root only; F3, F5 |
| S2 | Partly | F5; secrets stay root's |
| S3 | Scoping not served | F4 combined with F1 |
| S4 | Partly | F8 conflates partial or unknown setups with proposals |
| S5 | Partly | Control widenings unclassified |
| S6 | Partly | In-flight requests fine; no cascade on unshare (F3) |
| S7 | Partly | install is root's; F1(a) bypasses it through submission implementations |
| S8 | Mostly | Records follow inspect; derived content across projects not addressed |
| S9 | Mostly | Unchanged; F19 |
| S10 | Mostly | F14 |
| A78 | Partly | Runnable with root plus the coordinator; its safety claims (no write to `main`, `worker` only, a confined coordinator) are unenforceable until F1 is fixed |

WHAT THE DESIGN GETS RIGHT:
- S1 costs nothing extra.
- One home plus shares, with membership recorded on the member.
- B1, B2, B6 and B7 are closed for members.
- Approval is authority: no list that drifts, no self-approval, and a widening split into close-then-reopen cannot escape, because the opener needs the authority.
- Root and first start are unchanged, and narrowing is immediate.

RECOMMENDATION: keep the design as the base and amend F1-F3 before it claims granular Interfaces. F1's fix (only `management` registers templates and implementations; members only from P's templates with confining input schemas; no engine runner) is required of every Project design.

UNVERIFIED:
- I did not run F1(a) against the reference model: whether the first build accepts `implementations` in an ordinary submission and runs a `command` item for an Agent that gives no `runtime`.
- I did not recount the 78 conformance files citing ACC-033.
- I did not open the records of rulings 37, 48 and 52.
- The scenarios behind F2 and F9 rest on the design's own wording ("some open right" for open; "a right that ... and holds" for close). The design's author may intend otherwise.

## Verdict cost (minor)
The cost claims hold up in order of magnitude. Home Projects is the cheapest design so far, and I confirmed that it breaks no existing vector and leaves first start unchanged. The design underestimates the work by about 1.5x: about 30 promises are touched, not 22, across 6 core files, not 3 or 4, and the reference model needs about 1,600 to 2,800 core lines, not 1,000 to 1,800, because LIF-021, ruling 53's additions and pending actions are not built yet. It also misses some promises, includes one schema change it can avoid, and leaves 4 small rules unwritten. Nothing here blocks the design.

I only read the repo (<home>) and changed nothing. Counts below are measured unless marked as an estimate.

COST TABLE (the design's claim, then what I found)
| Item | Design | Checked |
|---|---|---|
| Promises in spec/ | 455-470 | 455 exactly (grep of ids) |
| Promises changed | ~17 | ~22-24 for behavior, plus ~5 wording-only for the rename (NAM-027, SUB-005, SUB-008, MAN-015, ACC-029 prose) |
| Promises added | ~5 | ~6-8 |
| Share touched | ~5% | ~6.5% (~30 of 455). Still far below design D (~50) and design C (~80) |
| Core files | 3-4 contracts | 6 files: types.yaml, capabilities.yaml, templates.yaml, conduit.yaml (exports), operations.yaml (inspect outputSchema plus the descriptions of register, replace, unregister, retire-with-resources, open and close), core/README.md (8 mentions of pool-management) |
| Schemas | manifest key + output field | spec/schemas/manifest.yaml. Also schemas/contracts.yaml:198 if CTR-004 changes as proposed (avoidable, see M1) |
| spec/README.md | not costed | "What access does not narrow" (README.md:337-372): all 4 bullets change ("for any consumer"; management narrowable only by a broker) |
| definitions.md | ~8 rows | ~8-9 rows, agreed |
| Renaming "project" | ~300 in ~45 files | spec: 300 in 50 files, confirmed. Outside spec, optional: the runner's fixture key "project" in 268 conformance files, 349 identifier uses in 33 reference-model files (loader/src/project.rs), 1,427 in docs/ (records, do not rewrite) |
| Existing vectors invalidated | 0 | 0 confirmed structurally (see V) |
| ACC-033 vectors | 78 | 76 case.yaml files. All keep their meaning |
| New vectors | 30-40 | 30-40 is reasonable. A convention case averages ~1,430 lines (200,531 lines over 140 cases), so ~45-60k fixture lines. Today's base is thin: ACC-012 and ACC-029 have 1 vector each (access-bounds); LIF-021 has none and is missing from the coverage index |
| Reference-model core | 1,000-1,800 lines | 1,600-2,800 (see R) |
| Store migration | n/a | No format migration. But existing deployments cannot be moved into projects (see S) |
| First start | no change | Confirmed: FST-001..006 stand; all 7 core templates are first-start ones; FST-006's list is unchanged |
| S1 lines | 27 concise | Baseline 64 and package 27 confirmed (simplification/README.md:19,82). The canonical +1/-6 is a paper estimate |
| Work orders | 3-4 | 3-4 for Project itself, after 3 prerequisites (P) |

V. WHY NO EXISTING VECTOR BREAKS (confirmed)
- required_promises is checked as a subset (crates/conformance/src/lib.rs:893), so new cited IDs are harmless.
- Maps are compared as projections (conformance/README.md:78), so an added `project` field is tolerated. stateUnchanged compares objects before and after.
- Fixtures refer to core symbolically (@core#…) with no digest, so new core contracts change no fixture.
- MAN-004 has no portable vector (manifest.yaml:5438).
- No fixture manifest uses a `project` key. The `"project"` in registration/*/fixture.json is the runner's own field.
- With no Project, A(x) is always `management`.

R. REFERENCE MODEL (crates are loader, core, conformance, shell; core is 10,465 lines)
- access.rs (729 lines), ~250-400 lines of change:
  - routing() returns a single route and checks `route.id == path.target` (:420), so row 7' needs a set of acceptable targets;
  - covers() and subset() (:76-92) need the state to resolve homes;
  - a holds-home check across the actor's rights.
- registration.rs (2,061 lines), ~300-500: record `project`; POOL-019; confinement of `from` and `runtime`; the SUB-009 check; POOL-004 for every family (today :573-584 checks only Resources).
- LIF-021 is NOT implemented. access.rs:280 only routes it, and retirement.rs implements only LIF-020's conflict. So retire-with-members is new code, not a generalization: ~400-700.
- The inventory, tombstones, `as` and expectedId on open/close are not in spec/ or the model. The only inventory is a CLI supplement (shell/author.rs:18). The per-project versions add ~200-400 on top of ruling 53's own work.
- Pending actions exist in neither spec/ nor the model: ~400-800 lines plus delivery through the shell.
- Loader: the manifest key is schema-driven, ~20-50 lines.
- CLI: there is no concise compiler yet (author.rs derives no Pool), so "a small change" means new code.
- DL13 is a precondition. Client tokens are issued only to root (shell/main.rs:157 and :212), so collab cannot connect at all.

S. STORE MIGRATION
- No format migration. Objects are serde_json::Value (core lib.rs State); a missing `project` reads as engine-level; new State fields can take serde(default). `deny_unknown_fields` applies only to the top level of Stored.
- Membership is fixed for life, so existing objects can never become members; they must be re-registered.
- An existing Pool named `s1` blocks creating Project `s1` (OBJ-002). The design says never adopt an existing Pool, so existing deployments must be retired and recreated.

READING COST
- Added, about 8 concepts: Project (a Pool Type); home; share; the `project` field; A(x); project-management.v1; row 7' (any of three authorizers); a proposal (pending action).
- Changed meaning: Pool (one Type is now home to every family); retire-with-resources (now retires members of every family); "project" renamed to "submission".
- Removed: none in spec/. The CLI-derived Pool disappears, and the broker stops being the only way to narrow `management`.
- ACC-033 grows from 14 rows to 16 plus A(x); its long observable has ~20 sentences that name `management`.
- More tools: each Conduit to a Project publishes up to 9 tools (IFC-012), each right to open 2-3. Root with 3 projects goes from 14 to ~41 management tools.

M. MISSED OR AVOIDABLE
1. The CTR-004 change can be avoided. type-spec.v1 requires a Pool Type's offers to contain pool-management.v1 (schemas/contracts.yaml:198). Let project.v1 offer both capabilities instead. One rule is then needed: only core#project.v1 may offer project-management.v1, otherwise a package's Pool Type could claim it.
2. Promises not in the list:
   - CON-024, OPS-019 and TPL-004, which list the management capabilities by name;
   - ACC-030's "within" rule ("may name a Pool only if the old one names that Pool") must accept a Project as covering its member Pools;
   - ACC-028's observable, and ACC-005.
3. Row 4 is unchanged, so a manager cannot set up the Resources of a member Pool through P. It needs a Conduit to each member Pool. This works, but it contradicts "set up … through the Project" and adds derived Conduits.
4. When a proposal arises is ambiguous:
   - If "registered and not open" is the trigger, every ordinary register-then-open raises one at root for a moment.
   - If a refused open is the trigger, a refusal gains an effect, which is new (ACC-002).
   One sentence fixes it.
5. Templates of a retired Project have no stated fate. Templates have no lifecycle (LIF-012), so their names stay taken engine-wide (OBJ-002).
6. Open takes coverage from one right and the home from any right, while close needs both in one right. Also, under ACC-016 the right that holds the home must belong to the sending instance.

P. PREREQUISITES (shared work; each needed before the Project work orders)
- the ruling-53 core additions (spec and model);
- the pending-action batch (rulings 49-51);
- DL13 tokens for a second Interface;
- LIF-021 in the model.