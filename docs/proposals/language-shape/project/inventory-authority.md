Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# How authority works today in Conduit, and the parts a Project design must change: root, management, access, Pools, bounds and delegation

## Key points
1. Every permission is an open Conduit (ACC-001, ACC-003; rulings 28-29), and ownership permits nothing (ACC-028). Using a Resource (a Conduit to it) and administering it (a Conduit to its Pool) are already separate, so "pools usable from many projects" needs no change for use. The hard part is administration.
2. Three targets authorize all management (ACC-033): `management` (object-management.v1, 11 ops), `access` (open, close, reconcile) and each Pool (pool-management.v1, 7 ops). One target decides each request, except registering a Resource into a live Pool, which needs both `management` and that Pool.
3. Only Resources have an owner (POOL-001), and OL2 (ruling 30) routes everything else to `management`. Agents, Interfaces, Pools, Conduits and templates have no group or namespace, which is exactly the gap a Project would fill.
4. `management` cannot be scoped. It takes no bound (CON-028) and no filter (CON-022, MAN-012). register, replace, unregister, set-up, retire, inspect and install on it act engine-wide (ACC-030 observable). Today the only narrowing is a broker, which is code, not the engine.
5. A bound (bound.v1: pools, targets, operations; ACC-029) covers future Resources of a named Pool but never a future Agent, Interface or Pool, because references are frozen to IDs when the Conduit is made (OBJ-021). It does not constrain the consumer or filters.
6. Delegation only narrows (ACC-030): a new bound must be within the actor's, an unbounded one comes only from an unbounded holder, and access ops come only from the actor's own list. Revocation does not cascade, and any covering holder can close anyone's Conduit (ACC-012), so overlapping project admins can revoke each other.
7. Re-delegation is gated by registration. open.v1 opens only a registered Conduit, and registering a standalone Conduit needs engine-wide management register.v1 (R2). That register/open split is also today's natural "pending approval" state.
8. Root holds root-management (all 11 ops) and root-access (unbounded; ACC-022), so it can create anything and open anything. It still needs a Conduit of its own for every Pool, control and ordinary op, which was 22% of S1's authored lines.
9. Some root powers are identity checks, not Conduits: declare-lost and purge (ACC-032), opening get-secret Conduits (SEC-008/009, and through ACC-005 any setup that opens one), protection from closing (ACC-024), permanence (LIF-022), and per rulings 49-50 every sign-in and pending action. A project-scoped "auth pool" needs root for every secret-reading instance.
10. A second Interface is created by register.v1 and set-up.v1 through `management`, has no access until a Conduit is opened (ACC-023), and gets authority only from Conduits root opens. How a person gets its token is DL13 (implementation-defined). The first build issues client tokens only to root and to run instances, so a second person cannot connect today.
11. Visibility is engine-wide. Inspecting any non-Resource needs `management` (ACC-033 row 7), so ruling 53's inventory and ruling 52's second refusal tier give a scoped admin all or nothing for Agents and Conduits.
12. Names are engine-global per kind (OBJ-002) and requests address objects by name (LIF-031). Two projects cannot reuse an Agent name, and decoy names are possible. Project must decide scoped versus global names.
13. "Project" is not remembered today. Only a template's manifest address and contract digest survive (NAM-009, SUB-008), the address is optional (MAN-015) and may be shared. The one project-like scope that exists is `address#name` in a bound's operations.
14. To realize "an Interface's authority over one or more projects" while keeping ruling 28 and CON-001, the natural design is a Conduit from an Interface to a Project, routed by new ACC-033 rows. The main alternative, a `projects` argument on bound.v1, scopes only `access`. Either way the change touches ACC-033 (78 conformance files), CON-028, ACC-029/030, ACC-012, FST-001..006 and OBJ-013. A Pool in several projects conflicts with ACC-033's one-authorizer rule unless Pool rows stay with the Pool.
15. The hazard to rule out: naming Pool `system` in any bound covers `management` and `access` (FST-001), which is near-root. A shared or system Pool must never become a project member by default.

## Findings
Scope: everything below comes from reading spec/ in <home> The main sources were spec/conventions/access.yaml (all 29 ACC promises), spec/promises/first-start.yaml, spec/core/{capabilities,templates,requirements,types,operations,conduit}.yaml, spec/promises/{conduits,pools,secrets,objects,submissions}.yaml, OPS-019, OPS-023, LIF-022, IFC-012, IFC-021, IFC-022, IFC-032, IFC-042, DL13 and REC-002. I also read the records of rulings 41 and 49-53 (plus 28-30 and 37 in the AUDIT.md log), the simplification proposal and its research, and the routing code of the first build (reference-model/crates/core/src/access.rs and crates/shell). Nothing was modified.

== 1. Ground rules every Project design inherits ==
- Every permission is an open Conduit. There is no grant object (rulings 27-29). A Conduit has one consumer, one target and one capability (CON-001), and it permits exactly the operations it lists (ACC-003). Without such a Conduit a request is refused (ACC-001).
- The decision is made anew for every request, from the Conduits as they stand at that moment (ACC-036). The engine makes it before the implementation is called (ACC-002).
- Ownership permits nothing (ACC-028, ruling 29). A Pool is never a caller. A Conduit to a Pool permits only the Pool's own operations, and every operation on the Pool's Resources is refused unless the actor has a Conduit to that Resource. So administering a Resource (through its Pool) and using it (through a Conduit to the Resource) are already separate today.
- Only Resources have an owner. POOL-001 gives a Resource exactly one owner, which is a Pool, fixed for its life. Agents, Interfaces, Pools, Conduits and templates are owned by nothing (OBJ-013 lists no `owner` for them).
- One authority covers an object's whole life (ruling 30, decision OL2 in docs/proposals/conventions/README.md:52): the owning Pool for a Resource, `management` for everything else, and `access` for opening and closing. The precedent cited there is Azure resource groups and Kubernetes namespaces.
- At any moment one target authorizes a request (ACC-033, observable). Rows 1 and 3 are the only exceptions: they need both `management` and the Pool.
- An onward request is decided by the sending instance's own Conduits, never by the caller's (ACC-016). Setup parts are authorized by the Conduits of whoever sent the setup (ACC-005, POOL-015).
- An actor is an Interface or an Agent (IFC-021). Both hold authority the same way (preservation test 5).

== 2. The three authorizing targets ==
- `management`: a Resource of type `management.v1`, owned by `system`. It offers `object-management.v1` with 11 operations: register.v1, replace.v1, unregister.v1, inspect.v1, set-up.v1, retire.v1, reconcile.v1, register-replacement.v1, install.v1, declare-lost.v1 and purge.v1 (spec/core/capabilities.yaml:110-134).
- `access`: a Resource of type `access.v1`, owned by `system`. It offers `access-control.v1`: open.v1, close.v1 and reconcile.v1 (capabilities.yaml:136-148).
- Every Pool: every Pool type must offer `pool-management.v1` (CTR-004). Its operations are register.v1, register-replacement.v1, inspect.v1, set-up.v1, retire.v1, retire-with-resources.v1 and reconcile.v1 (capabilities.yaml:150-170). The Pool has no unregister.v1, replace.v1 or install.v1.
- The engine itself implements all of these operations (conduit.yaml, `engine`). It states no `supports`, so no Conduit to these three targets can carry a filter (CON-022, MAN-012), and a carried filter makes the request unavailable (ACC-015).
- A bound can be carried only by a Conduit to `access` (CON-028). As a result, a Conduit to `management` can be narrowed by nothing except the list of operations it carries (ACC-030 observable; spec/README.md:337-372 "What access does not narrow").

== 3. Routing table (ACC-033 rows plus the promises around them) ==
| # | Request | Object / state | Authorizing target (Conduit must list the op) | Promises |
|---|---|---|---|---|
| R1 | register.v1 | Resource whose owner is an existing, non-retired Pool | `management` AND that Pool, both listing register.v1. The reference model routes the request through `management` and checks the Pool Conduit as a second precondition (access.rs:445-455) | ACC-033(1), register.v1 contract |
| R2 | register.v1 | Any other object: Agent, Interface, Pool (its entries' Resources included), standalone Conduit (CON-012), any template, a Resource whose Pool is in the same submission | `management` alone | ACC-033(2) |
| R3 | replace.v1 (a template), unregister.v1 (a template or a retired instance) | Any family | `management` alone. First-start objects are refused whoever asks | ACC-033(2), SUB-009, LIF-017, LIF-022, LIF-034 |
| R4 | register-replacement.v1 | Resource of a Pool's entry (Pool live) / declared Conduit | `management` + Pool / `management` | ACC-033(3), LIF-009, POOL-003, POOL-017 |
| R5 | set-up.v1, retire.v1 | Resource | Owning Pool; nothing if the Pool is retired | ACC-033(4), POOL-007 |
| R6 | set-up.v1, retire.v1 | Agent, Interface, Pool | `management`. The setup's parts are then authorized by the sender's other Conduits: declared Conduits by the sender's Conduit to `access` (ACC-005, CON-004), a Pool's entries by the sender's Conduit to the Pool (POOL-015), and a declared Conduit listing get-secret.v1 only when root sent the setup (SEC-008) | ACC-033(5) |
| R7 | inspect.v1 | Resource | Owning Pool; `management` if that Pool is retired | ACC-033(6) |
| R8 | inspect.v1 | Agent, Interface, Pool, Conduit, template, or a non-existent object | `management` alone. A caller with only Pool authority is refused before any diagnostic | ACC-033(7), OPS-023 |
| R9 | retire-with-resources.v1 | The Pool that is the target | That Pool; the Conduit need list only this op | ACC-033(8), LIF-021 |
| R10 | install.v1 | Packages, engine-wide | `management` | ACC-033(9) |
| R11 | declare-lost.v1, purge.v1 | Reserved operations | `management`, and the actor must be root acting on its own behalf. No Conduit for another consumer may list them | ACC-032, ACC-033(9), CTR-003, CTR-009 |
| R12 | open.v1, close.v1 | Conduit to any target except `access` | `access`. The actor's bound must cover the target and name every operation the Conduit lists. Root's Conduits are never closed (ACC-024). get-secret.v1 only for root on its own behalf, with a Resource or Agent consumer (SEC-008, SEC-009). Reserved ops only with root as consumer (ACC-032). Never provider ops (PRV-004). Conflict if an end is retired (CON-031) | ACC-006, ACC-012, ACC-029, ACC-033(10) |
| R13 | open.v1, close.v1 | Conduit to `access` (delegation) | `access`. The new bound must be within the actor's bound, and the access ops it lists must be ones the actor's own Conduit lists. The actor's bound need not cover `access` | ACC-030 |
| R14 | A control | An instance | The instance itself | ACC-033(11) |
| R15 | reconcile.v1 | Instance gone / management verb / other invocation | `management` / whichever target authorized the original request (`management` if that was a retired Pool) / the Pool that owns the Resource, or `management` for an Agent, Interface or Pool | ACC-033(12-14), OPS-014 |
| R16 | Any other operation of an instance (an Agent's turn, a Resource's read, and so on) | Target instance | A Conduit to that instance. A Conduit to its Pool does not permit it | ACC-001, ACC-003, ACC-028 |
| R17 | get-secret.v1 | Secret Resource | Never an actor's or an onward request. Only the engine reads it, through the instance's own Conduit | SEC-007, SEC-010, ACC-001 |
| R18 | run/stop/inspect-run, list-machines/place, fetch-package | Providers | No Conduit; only provider calls | PRV-004, PRV-005 |

== 4. The root Interface: what it can do and why ==
- First start creates exactly five objects (FST-001): Pool `system`, node `local`, Resources `management` and `access` (both owned by `system`) and Interface `root`. Root has exactly two open Conduits, the declared `management` and `access` from templates root-management and root-access (FST-002; spec/core/templates.yaml:93-145). Nothing else exists (FST-003).
- root-management lists all 11 operations of object-management.v1, the reserved ones included. root-access lists open, close and reconcile with no bound, so it is unlimited (ACC-022; ACC-029: "applies no bound.v1 = unlimited"). An empty bound is different: it covers nothing.
- Root can therefore register, replace, unregister, inspect, set up and retire any non-Resource, install packages, and open or close any Conduit to any non-retired target for any offered operation. ACC-022 lists the exceptions: ACC-032, PRV-004 and SEC-009.
- Root cannot act directly on Pool-routed rows (R1's Pool half, R4, R5, R7, R9, R15 for Resources), controls (R14) or ordinary operations (R16). Each time, it must first register and open a Conduit for itself, which it always can.
  - This holds even for the `system` Pool: inspecting `management` or `access` routes to `system` (R7), and root holds no Conduit to `system`.
  - This cost was 72 of 332 lines (22%) in the coordinator's S1 run (docs/proposals/simplification/README.md; research/map-mechanisms.md section c).
- Powers tied to root's identity rather than to a Conduit:
  - Reserved operations declare-lost and purge (ACC-032). The engine checks the caller's identity, and root cannot pass them on.
  - Opening any Conduit that lists get-secret.v1 (SEC-008). Only root, only on its own behalf, and the consumer must be a Resource or an Agent (SEC-009). A setup opens such a declared Conduit only when root sent the setup (ACC-005).
  - Root's Conduits are never closed by a request (ACC-024, ACC-012), and root is never retired (LIF-022). Its targets `management` and `access` are first-start objects and also never retired, so CON-008 never closes them either.
  - Rulings 49 and 50 send sign-in elicitations and every pending action to root alone in v0. Ruling 52 gives root the second tier of a refusal. None of these is in spec/ yet: a grep finds no "pending action".
- Why: rulings 28-29 make the right to open a Conduit to `access`. Ruling 30 (OL5) makes first-start objects permanent. RR11 says only root has access at first start. Ruling 37 X4 makes the reserved mark core-only. Ruling 39/41 makes only root open a get-secret Conduit. ACC-024 prevents locking out the only fully empowered actor. Ruling 51 lets a peer engine admit the owner's root as an Interface holding Conduits to that engine's `management` and `access`.

== 5. What `access` (the Resource) is ==
- A first-start Resource of type `access.v1`, owned by `system`. The engine serves it, and its one capability is access-control.v1: open.v1, close.v1 and reconcile.v1.
- Every open and close is a request to `access` (ACC-006, ACC-012, ACC-033 row 10). Holding a Conduit to the target is never enough (ACC-006).
- An open Conduit to `access` that lists open.v1 is a "right to open" (definitions.md:164). Its only limit is the bound it may carry (bound.v1). Only a Conduit to `access` may carry one (CON-028).
- open.v1 opens only a Conduit that is already registered (operations.yaml, open.v1). Registering a standalone Conduit is a management request (R2).
  - So proposing a relationship (register) and enacting it (open) are separate requests, which different actors may make.
  - Declared Conduits are registered with their consumer (CON-002) and opened at its setup by the sender's right to open (ACC-005).
- reconcile.v1 on `access` settles opens and closes whose outcome is unknown (ACC-033 row 13).
- A close takes effect at once, ends all the access the Conduit carried, and removes the Conduit (ACC-011, LIF-035). To narrow access you close a Conduit and open a narrower one (ACC-011). There is no change in place.

== 6. Bounds and delegation ==
- bound.v1 (spec/core/requirements.yaml:69-99; ACC-029) takes three lists: `pools`, `targets` and `operations`.
  - Naming a Pool covers the Pool and every Resource it owns, including Resources made later. A target is covered only when named.
  - `operations` holds qualified names compared as strings. An item `address#name` covers that operation in every submission at that address and in a package installed there later.
  - `pools` and `targets` are reference schemas, resolved when the Conduit is instantiated and stored as {kind, id} (OBJ-021; requirements.yaml comment).
  - So a bound can never cover an Agent, Interface or Pool created after it. The only growth it allows is new Resources in a Pool it names. Design C's skeptic hit this exact wall (research/design-C-agent-centric.md:660).
- Opening a non-access Conduit needs the target covered and every operation named (ACC-006). Closing needs the same (ACC-012).
- A bound does not constrain the consumer: within it, the holder may open a Conduit for any consumer (ACC-030 observable).
- A bound does not constrain filters: the holder may open an unfiltered Conduit even where its own Conduit to that target is filtered (ACC-029, ACC-030, ACC-009; README "What access does not narrow").
- Delegation (ACC-030): a Conduit to `access` is opened or closed only if its bound is within the actor's bound.
  - "Within" means every Pool and target it names is covered by the actor's bound, and every operation it names is one the actor's bound names.
  - An unbounded Conduit can be opened or closed only by an unbounded holder.
  - The access ops it lists must be ones the actor lists.
  - Delegation therefore only narrows (preservation test 5). Root can create a co-admin with an unbounded right to open.
- Revocation does not cascade. Closing a delegate's Conduit to `access` closes none of the Conduits it opened (ACC-030).
- Any holder whose bound covers a Conduit can close it, even one it did not open (ACC-012 observable). Overlapping bounds mean delegates can revoke each other's grants.
- There is no flag that stops re-delegation. What gates it in practice is that a Conduit to `access` must first be registered, which needs management register.v1 (engine-wide) or a declared Conduit.
- Hazard: a bound that names Pool `system` covers `management`, `access`, every node and every machine or package-source provider (FST-001, PRV-013). Its holder can open Conduits to `management` for anyone, except for reserved ops and get-secret.v1.
- Narrower access to `management` goes only through a broker: a trusted Resource whose own Conduit to `management` is as wide as any, so the narrowing is the broker's code and not the engine's (ACC-030 observable; definitions.md:166). README says a "focused constraint" comes back to the owner if brokers prove clumsy (ruling 44).

== 7. Giving a second Interface (a second person) authority over only some objects ==
How it is created today:
1. Root sends register.v1 to `management` (R2). The documents are an Interface template (kind Interface, type core#interface.v1 or another Interface Type, which may declare `conduits`) and an instance, e.g. `alice`. Declared Conduits are made with it in state registered (CON-002).
2. Root sets alice up through `management` (R6). The setup opens alice's declared Conduits, authorized by root's own right to open (ACC-005, CON-004). Alternatively root registers standalone Conduits (CON-012) and opens them (ACC-006); CON-013 opens them without setting alice up again.
3. alice has no access until a Conduit is opened for her (ACC-023). An Interface has no placement (PLC-006) and no run (TPL-019).
4. Identity: the engine decides which instance a token names (IFC-022, IFC-042). How a person comes to hold a token for alice is DL13, which is implementation-defined.
   - In the first build, client tokens are written only for root (crates/shell/src/main.rs:157 and :212) and for instances whose run identity is materialized (crates/shell/src/lib.rs:262-295). `--as` reads client-tokens/<id> (server.rs:779-788).
   - I found no path that issues a token to a second Interface, so a second person cannot connect to the first build at all.

The best scoping available today, for a "project" made of Pool P and Agents a1 and a2:
- alice→P listing the pool-management operations. This gives real, growing authority over P's Resources (R1 half, R4, R5, R7, R9, R15).
- alice→`access` with bound {pools:[P], targets:[a1, a2], operations:[...]}.
- Conduits from alice to a1 and a2 for their own operations (for example turns).
- Any authority to create objects needs alice→`management`, which is engine-wide.

What blocks finer scoping:
1. B1, `management` cannot be scoped. It takes no bound (CON-028) and no filter (CON-022, MAN-012). Any listed operation applies engine-wide (ACC-030 observable):
   - register any Agent, Interface, Pool, Conduit or template (R2);
   - replace any template, including other people's (R3); unregister any template or retired instance;
   - set up or retire every Agent, Interface and Pool (R6);
   - inspect every non-Resource (R8); install packages (R10).
   - alice cannot create Agents in her project without being able to retire everyone's Agents. The only way out is a broker.
2. B2, no grouping for non-Resources. Agents, Interfaces, Pools, Conduits and templates belong to no Pool (POOL-001, OBJ-013), so nothing like a namespace exists for them. OL2 routes them all to `management`.
3. B3, bounds are frozen at instantiation (OBJ-021). Every new Agent or Pool in alice's project means root must close her Conduit to `access` and open a wider one (ACC-011).
4. B4, bounds don't scope the consumer or filters. alice can give any Agent, including one of another project, unfiltered access to anything in her bound (ACC-030).
5. B5, overlap and orphans.
   - Overlap: anyone whose bound covers a Conduit can close it (ACC-012). A shared "skills" Pool named in several project admins' bounds lets each close the others' grants. A Conduit to the shared Pool listing retire-with-resources.v1 lets any one of them retire the whole Pool (R9).
   - Orphans: revoking alice leaves her grants live (ACC-030).
   - Authorship: who opened a Conduit lives only in records (REC-002 caller), not as stored authority. Finding those Conduits needs inspect.v1 on Conduits (R8), which is engine-wide.
6. B6, delegation is gated by global registration. Opening needs a registered Conduit, and registering a standalone Conduit is management register.v1 (R2). A Pool-scoped admin cannot even propose a new grant without engine-wide register.
7. B7, visibility is engine-wide. Inspecting Agents, Conduits and Pools is R8 (`management`). Ruling 53's inventory ("only what the caller may inspect") and ruling 52's second tier ("a caller that may inspect the target") inherit this, so a scoped admin sees all of it or none of it for non-Resources.
8. B8, some powers belong to root's identity alone.
   - alice can never open a get-secret Conduit (SEC-008), and cannot set up an instance whose declared Conduit lists get-secret unless root opened that Conduit first (ACC-005, CON-004). An "auth pool for a specific project" therefore needs root for every secret-reading instance and every replacement of one.
   - Sign-ins and pending actions go to root only (rulings 49, 50). Reserved operations are root's (ACC-032).
   - Only root's Conduits are protected from closing (ACC-024). alice's can be closed by any covering delegate.
9. B9, names are engine-global per kind (OBJ-002), and requests name objects by name (LIF-031).
   - Two projects cannot both have an Agent `reviewer`, and a standalone Conduit's name is its tool prefix (IFC-012).
   - A scoped registrant can plant a decoy Conduit under an expected name (design-D skeptic trace, research/design-D-relationship-first.md:497). Ruling 53's expectedId on open and close only partly fixes this.
10. B10, person-to-Interface identity is unspecified (DL13), and the first build does not implement it for a second Interface.
- What works today: Pool Conduits are a genuine, growing per-group authority for Resources, and a co-admin (unbounded `access` plus `management` minus the reserved ops) is possible. Scoped admin is what is missing.

== 8. What "project" is today ==
- A project is "the documents and the one manifest handed over together in one registration" (definitions.md:65; SUB-005). Nothing registered remembers it.
- A template keeps the project's manifest `address` (if any), its version, and the digest of that submission's contracts (NAM-009, SUB-008). But an `address` is needed only if the project holds contracts or implementations (MAN-015). Two submissions may share one address (SUB-008). Instances keep no project.
- NAM-027 (ruling 37, X5) makes names unique across kinds within one project's documents. That applies to one submission, not to registered objects.
- A bound can already name a project's operations by `address#name` (ACC-029), the only project-like scope that exists.
- No spec object holds "registrant", project membership, per-project tombstones or drift. The simplification put all of these in the CLI and the planned core additions (research/synthesis.md:29-100), and kept the derived "Pool per project plus an operator Conduit" in the CLI (ruling 53, decision 6, still open).

== 9. The promises a Project design must change, matched to the owner's direction ==
- "Project references every object including pools, agents":
  - Membership must be a reference, not ownership, to keep ruling 29 (ownership permits nothing) and POOL-001 (one owner per Resource).
  - It needs a new family or kind, or a standard argument (OBJ-013), and decisions on FST-001..006: is there a default or system project, and does `system` belong to one?
  - Conduits join objects that may sit in different projects, so the design must say whose project a cross-project Conduit belongs to. Ruling 51 decision 1 says "the target's owner authorizes each request", and today the right to open is judged on the target side (bound covers the target).
- "Pools accessible from multiple projects":
  - For use, nothing is needed. Any consumer with a Conduit to a Resource uses it, whatever its Pool (ACC-028).
  - For administration, multi-membership collides with ACC-033's single-authorizer rule and with the mutual close of ACC-012.
  - The design must choose. Either Pool-routed rows stay with the Pool and a project grants only the right to open Conduits to it, or ACC-033 changes to let any of several authorizers decide. ACC-033 is cited by 78 conformance files (map-mechanisms.md).
- "Projects determine an Interface's permissions; Interfaces have authority over one or more projects":
  - This is the missing namespace scope for `management` (B1, B2, B3, B7). Keeping ruling 28 (every permission is an open Conduit) and CON-001 (pairwise) points to a Conduit from an Interface to a Project as a new authorizing target, one per project for multiple projects.
  - It would need new ACC-033 rows routing register, set-up, retire, inspect and unregister of the project's Agents, Interfaces, Pools, Conduits and templates to the Project. It must also leave `management`'s rows intact, or carve the project's objects out of them.
  - Alternatively, bound.v1 gains a `projects` argument that covers later members, plus a consumer constraint. That scopes `access` only and leaves B1 unsolved.
  - CON-028, ACC-029, ACC-030, ACC-012 and the inventory and refusal disclosure tiers all change. Root-only identity powers need project-scoped counterparts or a deliberate statement that they stay root's: SEC-008 for an auth pool, rulings 49 and 50 for pending actions, ACC-024 for protecting an admin's grants.
  - Names (OBJ-002, LIF-031, IFC-012) must either become project-scoped or stay global, decided explicitly.
- "Approvals of widenings are pending actions; narrowing never waits; who may approve is declared":
  - Today a widening is an open (ACC-006, ACC-030), dispatched at once if the actor holds a covering right to open. A narrowing is a close (ACC-011, ACC-012), also at once, so "narrowing never waits" already holds.
  - The register/open split is the natural pending state: a Conduit sitting in state registered. Today the approver is whoever holds a covering right to open. Declaring approvers separately would be new.
  - The design-E skeptic's confused-deputy trace (root opens a Conduit someone else registered; research/design-E-layered.md:454-458) shows the approver must see exactly what it approves, which is the job of ruling 53's expectedId and inventory.
- Short form: whatever Project is must be expressible both as canonical documents (a document or standard argument plus Conduits) and as the concise form's derived "Pool per project plus operator Conduit" (synthesis.md:29-38). That derivation would then become the Project object itself and the operator's Conduit to it.

## Unverified
- How a bound judges a reconcile.v1 request made through a Conduit to `access`: I found no text. ACC-029 and ACC-030 speak only of covering the target and operations of the Conduit being opened or closed.
- Registering another Resource of type core#management.v1 or core#access.v1 from a user project: no promise I found forbids it. FST-001 and ACC-033 give meaning only to the first-start objects, so such a Resource would authorize nothing. Not tested.
- Which Conduit a register.v1 request for a Resource into a live Pool actually travels through: ACC-033 row 1 requires both. The reference model sends it through `management` and checks the Pool Conduit as a second precondition (reference-model/crates/core/src/access.rs:445-455). The spec text does not say which carries the request.
- That the first build has no way to issue a token to a second Interface: verified by grep of crates/shell (client_token is called only for root at init and serve, and when a run's identity is materialized). I did not run it, and did not check whether a work order is adding such a path.
- Not yet in spec/ (checked by grep): rulings 49 and 50 (pending actions and sign-ins to root), ruling 52's two-tier refusal, and ruling 53's five additions (as, expectedId on open and close, inventory, tombstones, descriptive tool descriptions). What I say about them comes from the ruling records only.
- "ACC-033 is cited by 78 conformance files" and "72 of 332 lines (22%) were root's own Conduits" are quoted from the simplification research (map-mechanisms.md, README.md), not recounted.
- The design consequences in section 9 of the findings (a Conduit to a Project as a new authorizing target, a `projects` argument on bound.v1, and their costs) are my analysis of the hinges. No ruling or proposal has written them.