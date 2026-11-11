Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Ownership, grouping, naming and lifecycle in Conduit today, and what a Project would have to hold if Pools are shared

## Key points
1. Only Resources have an owner: exactly one Pool, fixed for life (POOL-001). Agents, Interfaces, Pools, Conduits and templates have none. Everything about them is authorized against `management` (ACC-033 rows 2, 5 and 7), which cannot carry a bound or a filter (CON-028, ACC-030). So today no Interface can be scoped to one project's Agents; only Resources (through their Pool) and opening and closing (through a bound) can be scoped.
2. The Pool is the only scope below the engine. It owns, it authorizes (ACC-033 rows 1, 4, 6, 8, 13 and 14), a bound names it and then covers its later Resources (ACC-029), and it retires as a group (LIF-020, LIF-021). Ruling 29 says ownership permits nothing and power is a Conduit. A Project should follow the same pattern: membership is a stored fact, and authority is a Conduit to the Project.
3. Retiring a Pool:
   - `retire.v1` (authorized by `management`) reports conflict while any owned Resource's shutdown is unconfirmed.
   - `retire-with-resources.v1` (authorized by the Pool) retires the Resources in reverse dependency order, and a failure holds back only the Resources after it.
   - A runtime provider still named by any instance anywhere blocks it (PRV-020).
   - No Agent or Interface is retired. CON-008 closes the Conduits to the retired Resources. Objects that referred to them keep the ID and report it missing (LIF-019).
   - Nothing is ever collected for being unreferenced (LIF-023).
4. Object names are engine-wide per kind, templates and instances together (OBJ-002). They are unique across kinds only within one submission (NAM-027, ruling 37). The only scoped names today are an entry inside its Pool (POOL-014), a declared Conduit's local name inside its consumer (IFC-012), `#name` inside a project or package (NAM-022), and package addresses. Templates are global: `#name` written as `from` resolves among every template that exists (INS-025).
5. "Project" is transient. After registration only each template's pins and the submission digest remain (NAM-009, SUB-008). The manifest is not stored (NAM-010). Which objects a submission made is kept only in records, which root can purge. Any holder of `management` who replaces a template changes what later replacements in other projects get (LIF-009, INS-020).
6. The simplification work had to rebuild project memory in the CLI: an applied-IDs file, drift, `--prune`, and a guard keyed by project. Design D's skeptic found three things:
   - a record kept in the Pool dies with the Pool;
   - one implicit Pool per project widens access through bounds;
   - a project's identity must be an ID the engine checks, not a name, and an existing Pool of the same name must never be adopted.
7. What a Project must hold:
   - a checked ID and a name;
   - its manifest(s) in force (NAM-006), with NAM-010 still true for templates already registered;
   - its submission digests and its address;
   - membership of everything it owns: Agents, Interfaces, Pools, Conduits, and maybe templates;
   - separate references to Pools it uses but does not own;
   - tombstones and applied state that outlive any of its Pools;
   - its declared authority and approvers;
   - a Pool-like lifecycle: refuse retirement while owned objects are live, or retire them in explicit order.
8. Shared Pools: preservation test 3 ("owned once") requires one owner per Pool, a project or the engine level as `system` is, with other projects only referencing it. A reference must grant nothing, as references in `config` and `runtime` grant nothing today. Bound cover and Project authority must follow ownership, never reference. Otherwise every referencing project gains authority over the shared Pool, and over any Resource another project later adds to it.
9. Today's bound limits only the target. ACC-030 says its holder may open a Conduit "for any consumer". Under project-scoped Interfaces, project A's operator could connect project B's Agent to A's Resources. Cross-project Conduits therefore need consent on both sides, mirroring ruling 51's "two halves".
10. Retiring a shared Pool, or its owning project, closes other projects' Conduits to it (CON-008). Whether a referenced Pool should instead block retirement, as PRV-020 and PLC-011 do for runtime providers and nodes, is an open choice for the owner. Package version skew across projects is already fine: NAM-017 does not block a Conduit, and NAM-023 makes the target's version govern.
11. Owner's examples:
   - A skills Pool for all projects fits the `system` precedent: an engine-level owner, referenced by everyone.
   - An auth Pool for one project can be owned by a project today. But only root may open a Conduit that lists `get-secret.v1` (SEC-008), and sign-ins go to root in v0 (ruling 49). Approval of secret access by a project's Interface would change the secret guarantee itself.
12. The owner's "an Interface with authority over one or more projects" maps onto today's model as Conduits from the Interface to Project targets. The project may declare those Conduits, as templates declare Conduits. That needs these changes:
   - ACC-033 rows 2, 5 and 7, which put those families on `management`;
   - CON-028 and the arguments of `bound.v1`;
   - a consumer-side condition in ACC-006 and ACC-030;
   - the registration of a Conduit, which today takes the engine-wide `management`;
   - the free choice of a runtime provider in any Pool (PRV-006).
13. Approvals as pending actions:
   - Rulings 49 to 51 give the mechanism: raised at an Interface, with kind, subject, evidence, expiry and outcome. Ruling 53, decision 2 is still open.
   - Ruling 38 and ACC-019 forbid work that outlasts its invocation. An approved widening should therefore be the approver's own request, fixed by a digest or `expectedId` and decided anew (ACC-036).
   - "Who may approve" then means who holds the authority to perform the change.
   - Narrowing already never waits (LIF-005, ACC-011).
   - Routing an action to a project's approvers requires instances to know their project, which nothing records today.
14. Canonical and concise forms: the manifest's keys (MAN-004) and the standard arguments (OBJ-013, where `owner` is Resource-only) are both closed lists. Project identity therefore needs a manifest key, a standard argument (`project`, or `owner` for all families), or a new kind (OBJ-020). The concise form's `project:` heading maps onto any of these. Project-scoped names would change OBJ-002, OBJ-006, NAM-003 and every reference schema, and would need a rule for referring to a shared Pool from another project.
15. These things stay engine-wide whatever a Project becomes:
   - the first-start objects;
   - nodes and the machine and package-source providers, all owned by `system` (PLC-008, PRV-013);
   - installed packages (`install.v1` on `management`);
   - records and purge, and the operations reserved to root;
   - the trust domain.

## Findings
All paths are relative to the repository root, <home> I only read files and changed nothing. Citations give the promise ID and the file:line where the promise starts.

## 1. Who owns what today, by kind

Only Resources have an owner. Every other family has none. Without an owner, the authorizing target is `management`, which covers the whole engine and cannot be bounded.

| Kind | Owner or holder | Who authorizes its life (ACC-033 rows, spec/conventions/access.yaml:652) | Sources |
| --- | --- | --- | --- |
| Resource | Exactly one Pool, fixed when the Resource is made; no operation changes it | Register: `management` plus the owning Pool if that Pool already exists (row 1). Set up or retire: the Pool (row 4). Inspect: the Pool, or `management` once the Pool is retired (row 6). Reconcile: rows 13 and 14 | POOL-001 (spec/promises/pools.yaml:45), POOL-004 (:84), definitions.md:122 |
| Node, `management`, `access` | The `system` Pool. The engine makes every node; no document makes one | Never retired by request (LIF-022, LIF-034) | PLC-008 (placement.yaml:189), PLC-026 (:446), PLC-028 (:515), FST-001 (first-start.yaml:33), FST-006 (:96) |
| Machine provider, package-source providers | Must be owned by `system`. At most one machine provider, and one provider per URI scheme | Like any other Resource | PRV-013 (providers.yaml:502) |
| Runtime provider | Any Pool. An instance anywhere names it in `runtime` with no Conduit; PRV-004 forbids Conduits to provider operations | It cannot be retired while any instance, inside or outside its Pool, names it (PRV-020, providers.yaml:691) | PRV-006 (:300) |
| Secret Resource | Any Pool; nothing restricts the owner | Only the root Interface may open a Conduit that lists `get-secret.v1`, and only to a Resource or Agent consumer | SEC-008 (secrets.yaml:286), SEC-009 (:309), SEC-022 (:631) |
| Agent | No owner. It has one placement (a node owned by `system`) and an optional `runtime` | `management` for register, set up, retire, inspect and reconcile (rows 2, 5, 7, 14) | OBJ-013 (objects.yaml:230) |
| Interface | No owner and no placement. A non-root Interface has no access until a Conduit it consumes is opened | `management` (rows 2, 5, 7) | ACC-023 (access.yaml:488) |
| Pool | No owner; no Pool owns a Pool. It is never a consumer or a caller | `management` for register, set up, retire and inspect (rows 2, 5, 7). The Pool itself authorizes `retire-with-resources.v1` (row 8) | ACC-028 (access.yaml:535) |
| Conduit | No owner. A declared Conduit is a nested instance that belongs to its consumer (definitions.md:75, CON-002 at conduits.yaml:96). A standalone Conduit belongs to nothing (CON-012, :217) | Register: `management` (row 2). Open and close: `access`, through a bound that limits the target only. It closes when either end is retired (CON-008, :165) and is removed when closed (LIF-035) | ACC-006 (access.yaml:190), ACC-012 |
| Template | No owner and no lifecycle state. It exists until unregistered, and its name is global for its kind | `management` to register, replace, unregister and inspect (rows 2, 7). A submitted document never replaces a template (SUB-007, submissions.yaml:161); only `replace.v1` does, and it hands over a project (SUB-009, :230) | LIF-012 (lifecycle.yaml:311), INS-020 (instantiation.yaml:306) |
| Contract (Type, Capability, Operation, Requirement, Schema) | Not an object. It belongs to a package (`address#name`) or to one submission of a project, identified by digest | It is not registered. Templates keep the address and digest (NAM-009) | SUB-008 (submissions.yaml:176), NAM-009 (naming.yaml:191) |
| Package | The whole engine. Installed by `install.v1` against `management` (row 9); several versions can be installed side by side | | NAM-011 (naming.yaml:228), PKG-006 (packages.yaml:135) |
| Record | The engine. It names the caller, the Conduit, the target and so on, but no project. Only root may purge | | REC-002 (records.yaml:103), REC-004 (:167) |

Precedent: ruling 29 decided that "Pool ownership stays one recorded fact, fixed at creation, that permits nothing; every power over a Pool or its Resources is an open Conduit." The consolidation audit (docs/proposals/consolidation/README.md:58) considered making ownership a Conduit and rejected it because it needed five exceptions. Ruling 30 (OL2, docs/proposals/conventions/README.md) set one authority for an object's whole life, "modelled on Azure resource groups and Kubernetes namespaces". The comment at the head of pools.yaml (pools.yaml:2-4) cites the same two models.

Consequence: no Interface can be given authority over one project's Agents, Interfaces, Pools, Conduits or templates. All of these go through `management`, which carries no bound and supports no filter (CON-028 at conduits.yaml:430, ACC-030 at access.yaml:583). The spec's own remedy is a broker (spec/README.md, "What access does not narrow"). Only Resources (through their Pool) and open/close (through a bound) can be scoped today.

## 2. What retiring a Pool does

- **`retire.v1` on a Pool** is authorized against `management` (row 5). It reports conflict while the Pool owns any Resource whose shutdown is not confirmed, whether that Resource is registered, ready, or retired with its shutdown pending or unknown (LIF-020, lifecycle.yaml:454).
- **`retire-with-resources.v1`** is authorized against the Pool itself, and the Conduit need list only that operation (row 8; LIF-021, lifecycle.yaml:468).
  - It retires the Resources in the reverse of setup order. A Resource is retired before anything it refers to, names in `runtime`, or consumes through a Conduit, among the Pool's own Resources. The order among Resources in a cycle is left to the engine (DL1).
  - A Resource that fails to retire holds back only the Resources ordered after it. Retirements that succeeded are not undone.
  - The Pool's own `retire.v1` is dispatched only after every shutdown is confirmed. Otherwise the outcome is failed, with diagnostics.
  - PRV-020 blocks the retirement of a runtime provider that any instance still names, inside the Pool or outside it. This is a cross-Pool "in use" block. PLC-011 blocks nodes the same way.
- **After the Pool is retired:**
  - Every Conduit to the Pool is closed (CON-008).
  - Nothing can be registered into it: POOL-004 rejects the document (pools.yaml:84) and POOL-017 rejects an entry replacement (:271), each reporting invalid.
  - Inspecting and reconciling its Resources move to `management` (rows 6, 13 and 14). Nothing else is authorized against it.
  - A Pool whose shutdown is not confirmed is retired again with `retire.v1` through `management` (LIF-026).
  - It is removed once its shutdown is confirmed, and its name is freed (LIF-035, lifecycle.yaml:787).
- **Effects outside the Pool:**
  - No Agent, Interface or other Pool is retired. Only what the instance itself holds is stopped (LIF-016, :381).
  - Conduits whose target is a retired Resource close at once (CON-008), so Agents lose that access without being retired.
  - Objects that referred to the Pool keep its ID and report it missing (LIF-019, :438).
  - Nothing is ever collected for being unreferenced (LIF-023, :544).
  - `system` can never be retired (LIF-022, :530).
- **Entries:**
  - An entry's Resource is made with the Pool (POOL-002, :57) and set up as a part of the Pool's setup, in dependency order (POOL-013).
  - That part is authorized by the setup sender's own Conduit to the Pool (POOL-015, :227).
  - A retired entry Resource is replaced only by an explicit `register-replacement.v1` (LIF-008 and LIF-009, lifecycle.yaml:211, :227). The replacement is owned by the same Pool.

## 3. How names are scoped

- **Object names are engine-wide, per kind, with templates and instances counted together** (OBJ-002, objects.yaml:69). Being in different Pools does not separate names: no two Resources may share a name, whichever Pools own them, and no two Agents may either. Generated names are the template's name plus a suffix, unique per kind (OBJ-003, :83).
- **A name is fixed once its object is made** (OBJ-011, :197). A retired object keeps its name until it is removed (LIF-007). After removal the name is free, and holders of the old ID never follow it to the new object (OBJ-010, LIF-035).
- **Within one project or package, names are unique across kinds** (NAM-027, naming.yaml:519). This is ruling 37's "names are unique across kinds within a project". The engine-wide rule is per kind only, so a Pool `s1` and an Agent `s1` from different submissions can coexist. The simplification critic's point 16 ("the default Pool named after the project collides with any key of the same name") therefore holds only within one submission.
- **The only scoped names that exist today:**
  - an entry name inside a Pool, reached from outside as `{kind: Pool, name, entry}` whatever name was generated for the Resource (POOL-014, pools.yaml:213);
  - a declared Conduit's local name inside its consumer, which names that consumer's tools (IFC-012, interface/implementation.yaml:476);
  - `#name` for contracts and implementations inside a project or package (NAM-022, naming.yaml:412);
  - a package address (NAM-001, DNS-based).
- **Templates are not project-scoped.** In a submitted project, `#name` written as `from` resolves among all templates that exist, as a bare name does (INS-025, instantiation.yaml:376). A local reference contains neither `#` nor `/` and is never looked up in a package (NAM-003). A `project/name` form would therefore clash with today's syntax rules; a project-qualified reference would need a new key in `{kind, name}` (OBJ-006) and in every reference schema.
- **Actors name objects by reference only.** `expectedId` is a precondition, not an address (LIF-031, LIF-037).

## 4. What already behaves like a scope

1. **The Pool.** It does four things a scope does: it owns, it authorizes (rows 1, 4, 6, 8, 13 and 14), it is the unit a bound names (ACC-029, access.yaml:547), and it retires as a group (LIF-020, LIF-021). A bound that names a Pool covers the Pool and every Resource it owns, including Resources added after the bound was opened. That cover is a precedent for hierarchical scope.
2. **The `system` Pool and the first-start objects.** They are the engine's own scope: nodes, `management`, `access`, the machine and package-source providers, and root. They are shared by everything, which is the existing precedent for a "for all projects" Pool.
3. **A bound on a Conduit to `access`.** It limits open and close by Pools, individual targets and operations. It limits targets only: the holder "may open, for any consumer" a Conduit to a covered target (ACC-030 observable). Opening also needs the Conduit to be registered first, which takes `management`.
4. **A submitted project.** Its submission is atomic (SUB-003, submissions.yaml:93). Names are unique across kinds within it (NAM-027), `#name` resolves within it, its manifest pins one version per package (NAM-006, naming.yaml:148), its contracts are identified by one digest (SUB-008), and its manifest's `address` names those contracts outside it (MAN-015, manifest.yaml:400). All of this disappears at registration; see section 5.
5. **Nested instantiations.** Declared Conduits and entries belong to the instance that holds them. A replacement is keyed by (instance, nested name) (LIF-009).
6. **A consumer's tool namespace** (IFC-012, IFC-014).
7. **`metadata.annotations`** (OBJ-018, objects.yaml:335). They have no meaning to the language. They are the only place a CLI could stamp a project today, and they are unchecked: the skeptics of design C list "provenance is forged by the client" (simplification/research/synthesis.md, rejected parts of C).
8. **Engine-level scopes:** the trust domain (ruling 51, decision 2) and records.

## 5. "Project" today, and where it disappears

- **The definition** (spec/definitions.md:65, SUB-005, the `register.v1` description in core/operations.yaml): the documents and the one manifest handed over together in one registration, or with a `replace.v1`.
- **What survives registration:**
  - each template's own pins: address, version and digest per package, plus the submission digest and address for its own contracts and the implementation items it selected (NAM-009, naming.yaml:191);
  - each instance's copy of those pins (PKG-005, packages.yaml:119).
- **What is not stored:**
  - The manifest itself. Resolution uses the template's own pins, "not through any manifest in force at that time" (NAM-010, naming.yaml:215).
  - Which objects a registration made. That exists only in the records (REC-001), which root may purge (REC-004), and in the result of `register.v1`.
  - Any link from an Agent, Pool or Conduit back to "its" project.
- **Coupling across projects already exists through templates:**
  - Any holder of `management` can `replace.v1` a template.
  - A later `register-replacement.v1` for a nested instance anywhere uses the template "as it now stands" (LIF-009, INS-020).
  - The synthesis lists this as the replace.v1 hijack (design E, #4) and answers it with one fresh template per instance (synthesis.md, item 1).
- **The simplification work had to rebuild project memory in the CLI** (simplification/README.md; research/synthesis.md items 2 and 4):
  - a tool-written `conduit-project.applied` file of the IDs last applied;
  - drift computed from the inventory and the tombstones;
  - `--prune`, and "Name held by an object the project does not manage: refused".
- **The critic** (research/critic.md, point 2) asks for the revocation guard to be keyed "by (project, agent key, grant key)". Ruling 53 accepted tombstones enforced by the engine at open and register, which have no project key today.
- **Design D's skeptic** (research/design-D-relationship-first.md:460-480) found:
  - "One implicit Pool per project widens access silently". Bounds that name the Pool cover every Resource added later, and it cannot express Resources in different Pools.
  - "The applied record lives with the Pool, so losing the Pool erases the protections". After `retire-with-resources.v1`, the next apply is a first apply that recreates revoked access.
  - "Two directories, or two authors, with the same name share one record."
  - Fixes proposed there: keep the record apart from the Pool, "as a permanent object (LIF-019 style)"; "the project's identity is an ID that is checked, not the Pool's name"; refuse to adopt an existing Pool of the same name.

## 6. What a Project would have to hold

From the gaps above and the owner's direction:

1. **Identity:** an ID that the engine checks, and a name. Whether project names are engine-wide or scoped is open. The manifest's `address` is not an identity, because two submissions may share one (SUB-008).
2. **Its manifest in force, or the history of its manifests.** NAM-006 already puts pins per project, and ruling 53 decision 5 has `conduit install` write them. NAM-010 must still hold: templates keep their own pins, and a stored manifest governs only later registrations into the project. Today each submission's `#name` reaches only its own contracts (NAM-022, SUB-009), so a project registered in several steps needs a rule for contracts across its submissions.
3. **Membership of what it owns, for every family:** Agents, Interfaces, Pools, Conduits (standalone and declared), and templates if they become project-scoped. Its Resources would come through its owned Pools.
4. **References to Pools it uses but does not own.** These must be stored apart from ownership.
5. **Its memory:** tombstones, applied state and the baseline for drift and prune. This must outlive the retirement of any of its Pools and the removal of its objects (design D, finding 5).
6. **Its authority and its approvers:** which Interfaces act on it, and who approves widenings. Both must be declared (owner's direction; ruling 52's "as explicit and descriptive as possible").
7. **Its lifecycle,** modelled on Pools:
   - `retire.v1` refused while owned objects are live (as LIF-020), plus an explicit retire-with-contents across families in reverse dependency order (as LIF-021).
   - Referenced Pools are left alone. The project's own Conduits into them close as their consumers retire (CON-008).
   - Whether its memory survives its removal is open, and so is whether a name with history needs a recreate flag.
8. **Engine-wide things that stay outside every project:**
   - the first-start objects;
   - nodes, the machine provider and package-source providers, all in `system` (PRV-013, PLC-008);
   - installed packages (row 9);
   - records and purge, the operations reserved to root (ACC-032), and the trust domain.

## 7. If Pools are shared by several projects: ownership vs reference

- **Preservation test 3**, "A shared thing is owned once and never copied to an agent" (docs/AUDIT.md, Preservation tests). It implies that a Pool has exactly one owner, either one project or the engine-level scope as `system` is today, and that other projects only reference it. A Pool with several owners would break this test.
- **A reference grants nothing today.** References in `config` and `arguments` (OBJ-021) and `runtime` (PRV-006) need no Conduit to the object they name. The only authority used is `management` at registration. A project's reference to a shared Pool should therefore give that project's Interfaces no authority over the Pool. The authority must come from a Conduit to the Pool, or to the Pool's owning project.
- **Bound cover must follow ownership, never reference.** A bound already covers a Pool's later Resources (ACC-029). If a project bound covered referenced Pools, every project that references a shared Pool would gain authority over it, and over every Resource any other project later adds to it. The same widening appears with any bound that names a shared Pool directly: when one project adds a Resource, every other holder of such a bound covers it at once.
- **Cross-project Conduits need consent on both sides.** Today only the target side is checked (ACC-006, ACC-030), and the consumer can be anyone. Under project-scoped Interfaces, an operator of project A could open a Conduit from project B's Agent to A's Resources, given a Conduit registered by a holder of `management`. A rule that also needs authority over the consumer's project mirrors ruling 51, decision 4 ("a relationship across parties is two halves").
- **Retiring a shared Pool, or its owning project, affects other projects.**
  - CON-008 closes the referencing projects' Conduits to its Resources.
  - LIF-019 leaves their references pointing at a missing ID.
  - LIF-023 never protects an object because something refers to it.
  - The existing "in use, refuse" precedents are PRV-020 for runtime providers and PLC-011 for nodes. Whether a referenced shared Pool blocks retirement in the same way, or the referencing projects simply lose access, is a choice to put to the owner.
- **Version skew between projects is already handled.** Conduits are not blocked by different package versions (NAM-017, "must not"), and the target's version of a contract governs (NAM-023, naming.yaml:450).
- **Records name no project** (REC-002). Auditing per project how a shared Resource is used would need a mapping from consumer to project.
- **The owner's two examples.**
  - A skills Pool for all projects fits the `system` precedent: an engine-level owner, referenced by everyone.
  - An auth Pool for one project: ownership works today, since any Pool may own secret Resources. But SEC-008 lets only root open a Conduit that lists `get-secret.v1`, and sign-ins go to root in v0 (ruling 49). A project Interface could not grant or approve secret access without changing SEC-008 and the README section "What the guarantee for secrets covers" ("only the root Interface's choice ... limits which secret reaches an Agent's process").

## 8. Interfaces with authority over projects, against today's access model

- **Today an Interface's authority is exactly the Conduits it consumes** (ACC-023, test 8: "a human's access is an Interface holding grants"). Interface, Agent and Resource templates may declare Conduits (TPL-001); `root` declares its two.
- **The model already supports one reading of "an Interface with authority over one or more projects":**
  - a Project becomes a target, like a Pool;
  - the project-management operations are authorized against it;
  - an Interface holds one Conduit per project;
  - a bound may name Projects.
  - This keeps ruling 29 (power is a Conduit, ownership permits nothing) and ACC-033's rule that "at any moment one target authorizes a request". The Project would authorize its non-Resource objects, the Pool would keep authorizing Resources, and `management` would keep the engine-wide rows (install, reserved operations, `system`, creating projects).
- **"Projects determine an Interface's permissions"** can be read compatibly: the project declares the Conduits from its Interfaces, as a template declares Conduits or a Pool declares entries, so the declaration lives with the project and the authority is still Conduits.
- **What has to change:**
  - ACC-033 rows 2, 5 and 7, which put Agents, Interfaces, Pools, Conduits and templates on `management`.
  - CON-028, which allows a bound only on Conduits to `access`, and the bound's arguments (`pools`, `targets` and `operations`), if Projects become nameable in a bound.
  - A consumer-side condition in ACC-006 and ACC-030.
  - The registration of a Conduit, which today needs the engine-wide `management` regardless of the scope of its ends.
  - The picking of a runtime provider in another Pool, which `management`'s register authorizes today without any Conduit (PRV-004, PRV-006).

## 9. Approvals of widening as pending actions

- **What the rulings say:**
  - Ruling 49: sign-in prompts go only to the root Interface in v0; more granular later; "fleshing out interfaces ... a lot of the work for v1".
  - Ruling 50, point 3: every action waiting on a person is one mechanism, raised at the Interface, "a control panel".
  - Ruling 51, decision 5: a pending action has a kind, a subject, evidence, an expiry and an outcome.
  - Ruling 53, decision 2 is still open: "how a change is approved without a terminal".
- **No promise today declares who approves anything.** The only fixed approvers are root, for reserved operations (ACC-032), purge (REC-004) and secret Conduits (SEC-008).
- **Routing a pending action to a project's approvers needs the subject's project** (section 6, item 3), which nothing records today.
- **A constraint from ruling 38 and ACC-019** (access.yaml): no operation arranges work that outlasts its invocation, and "work that is to happen later is asked for later, by an actor". An approved widening is therefore best carried out by the approver's own request:
  - the approver opens the Conduit, or confirms a recorded proposal whose content is fixed by a digest and `expectedId`;
  - it is decided anew when made (ACC-036) and recorded for the approver.
  - "Who may approve" then means who holds the authority to perform the change.
- **Narrowing already never waits:** a retirement or a close takes effect when it is dispatched (LIF-005, ACC-011), and the synthesis orders narrowings before widenings.
- **Disclosure:** a pending action must not reveal objects the approver may not inspect (ruling 52; critic point 8).

## 10. Constraints from the canonical documents and the concise form

- The manifest's keys are a closed list (MAN-004, manifest.yaml:149). The standard arguments are a closed list too (OBJ-013): `owner` exists only for Resources. A project identity in canonical documents therefore needs a new manifest key, a new standard argument (`project`, or `owner` for every family), or a new kind. A new kind changes OBJ-020's ten kinds, which is the same scale as adding a family.
- The concise form's `project: <name>` heading (synthesis item 1) maps onto either, and a project boundary is what makes short local names safe.
- Making names project-scoped would change OBJ-002, OBJ-006, NAM-003 and every reference schema. It would also have to define cross-project references to a shared Pool, and to its entries (POOL-014).

## Key files

- spec/promises/pools.yaml
- spec/conventions/access.yaml (ACC-006, ACC-028 to ACC-030, ACC-033)
- spec/conventions/lifecycle.yaml (LIF-007 to LIF-009, LIF-012, LIF-016, LIF-019 to LIF-023, LIF-035)
- spec/promises/objects.yaml (OBJ-002, OBJ-003, OBJ-011, OBJ-013)
- spec/promises/naming.yaml (NAM-006, NAM-009, NAM-010, NAM-022, NAM-027)
- spec/promises/submissions.yaml (SUB-003, SUB-007 to SUB-009)
- spec/promises/providers.yaml (PRV-004, PRV-006, PRV-013, PRV-020)
- spec/promises/secrets.yaml (SEC-008, SEC-009)
- docs/proposals/simplification/research/design-D-relationship-first.md:460-480
- docs/proposals/simplification/research/critic.md
- docs/proposals/simplification/research/synthesis.md
- docs/proposals/ruling-53/README.md

## Unverified
- The prior-art comparisons (Azure resource groups, Kubernetes namespaces, Terraform state and remote state, ownerReferences, server-side apply field managers) come from the repo's own records (pools.yaml header, conventions/README.md OL2, consolidation/README.md, critic.md point 19), and some reflect my own understanding of those tools. I checked none of them against outside sources.
- I read the ruling-log rows for rulings 37, 40, 41 and 48 to 53, and the full records of rulings 49, 50, 51 and 53. I did not open the ruling-37 record (docs/proposals/core-and-schemas/README.md) or the ruling-52 record (docs/proposals/ruling-52/README.md). The point about ruling 37 (names unique across kinds within a project) rests on the ruling-log row and NAM-027.
- Ruling 53's accepted inventory names only the Conduits an object consumes and is the target of. The synthesis's wider inventory (registrant, entries, annotations) may or may not be in scope. I did not check whether any accepted text stores an object's registrant.
- I did not read spec/definitions.md past line 205 (the Ownership and First start rows fall within what I read). Of the enrollment proposal I read only its pending-action passages.
- I did not check what the reference model or shell does about projects beyond a code search. It shows Project only as the transient input to a submission (crates/loader/src/project.rs, crates/core/src/packages.rs).
- I ran no checks or tests. The vector counts quoted in map-mechanisms.md (for example, ACC-033 cited by 78 conformance files) are that paper's figures, not mine.
- The parallel short-form investigation was not in the tree when I looked (no directory for it under docs/proposals/). Whether its form already has a project heading or owner rules is unknown.
- The design options in sections 6 to 9 (a Project as a target like a Pool, consent on both sides, approval as the approver's own request, ownership with references for shared Pools) follow from the promises cited. They are not decided or tested. I estimated no counts of affected promises.
- Package removal: PKG-006 and NAM-011 speak of an installed package being removed, but I found no operation that removes one. I did not settle whether that is a gap.