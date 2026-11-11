Not binding. The [specification](../../../spec/README.md) alone binds. The revised recommendation of the investigation of the owner's Project model, 2026-11-10 (three designs, four skeptics each, a synthesis, a critic and a revision), kept as returned with local paths removed. It is the brief the Project rulings rest on; ruling 59 decides its group A.

## model
RECOMMENDED MODEL (revised after the second critic): Projects that hold Conduits, acted for by forwarding, with what flows through a Project confined to its own Agents, and root's authority made standing

Checked read-only against the repository at a98ffe0. That commit adds rulings 56 and 57 after 22ce848. Ruling 56 point 8 calls a pending action's result its "answer", and this text uses that word. Ruling 57 does not bear on this. Nothing was modified. Apart from reads, the only things run were read-only scans of conformance/.

WHAT THIS REVISION CLOSES (the critic's findings, each with where it is closed)
- M1. make.v1 got past rules worded only for opening.
  - ACC-048 now says that, for every condition on opening a Conduit, a request to make one counts as a request to open it.
  - ACC-045, ACC-046, ACC-032 and SEC-008 are reworded "open or make", and each gets vectors (sections 7 and 9).
- M2. E1's exclusions could be bypassed through a worker.
  - ACC-047(c) forbids register.v1, register-replacement.v1 and retire-with-resources.v1 whatever the consumer.
  - PRJ-006 rejects an E2 whose bound names them (sections 5 and 8).
- M3. Standing turns reach every Conduit a worker holds.
  - This is named in section 13 and shown by computed reach marks (section 11).
  - B7's rationale is corrected.
  - The hard rule for secrets is ruling B13.
- M4. The binding close is replaced by an explicit withhold.
  - The engine never withholds implicitly. withhold.v1 is a separate operation of `access` that a bound can withhold, it is lifted by ruling 55 A7's lift, and it is shown in inspection (section 10, ruling B11).
- D1. An explicit cascade Conduit used to be permanent.
  - ACC-024 is amended: root may close its own Conduits, except the first-start ones and the defaults.
  - A cascade is one command with a named flag (ruling A3(d)).
- D2. Setting up through an E1 when the instance has parts.
  - act.v1 takes `partsThrough`, which names the held E2 that decides the parts.
  - Confinement admits the instance being set up as a part's consumer.
  - For an unnamed part, provenance lists every Conduit that covered it, which fixes H1 F16 (sections 6, 8 and 10).
- D3. A non-root actor could give a Project a Conduit only once. make.v1, sent by an actor on its own behalf, may now name the actor itself or a Project as consumer (section 9, B8).
- D4. Actors could not see the grants they made.
  - An inventory entry is shown wherever the caller may inspect the object at either end.
  - list.v1 on a Project lists what it holds and its withholds (ruling A4, section 11).
- D5. pkg-console had nothing to sign in with.
  - Batch 2's shell order issues tokens to non-root Interfaces.
  - The bridge an Interface forms by acting for more than one Project is named and marked (ruling B4).
- D6. Both unnamed exceptions are gone.
  - The "reconciled only by root" rule is dropped: reconciliation follows the ordinary rule, and the computed mark catches an open recorded later.
  - The bind is now an operation a bound can withhold.
- D7. Ruling 51 decision 5, ruling 30 OL5, ruling 55 A7 (extended) and ruling 56 point 8 are named in section 15.
- D8. Bounds across a stable-name replacement become ruling A7. The examples bound on Pools, which also reads "auth pool for a specific project" literally.
- Minor findings:
  - reserved names and name collisions (section 4);
  - generated names under OBJ-003 and DL15 (section 9);
  - PRJ-008's precedent, and the amendments to IFC-015 and IFC-022 (section 6);
  - OPS-035's reason;
  - only rows 5, 7 and 14 lose Agents;
  - retire dropped from the example's `manage` and `run-pool`;
  - list.v1 over every kind;
  - closing an actor's Conduits, named as intended (section 7);
  - the recount (staging);
  - default-Conduit records in the vectors;
  - the vector question behind B9, now checked (see VECTORS).

Synthesized from M1 (agents act, native), M2 (native, full) and M3 (forwarding), from the twelve skeptic reports, and from both critics.

Notation: core# = github.com/shrav-k/conduit/core#. Real documents write full addresses (ruling 25). Promise IDs are proposals; IFC-048, ACC-044 to ACC-052, OPS-035 and LIF-038 to LIF-039 are free.

What I take from each design:
- From M3: the forwarding mechanism. It amends no ruling, keeps ACC-001, REC-002 and ruling 17 V3 as written, and its skeptics found it sound.
- From all three: the Pool/Project mirror, root's engine-made defaults (`system` included), and provenance.
- From the skeptics and critics, the fixes that recur:
  - turns never travel through a Project's grants, and an actor is never a grantee through its own Project;
  - Conduits to an actor are root's alone;
  - creating a Conduit is a separate operation that a bound can withhold, and every condition on opening applies to it;
  - closing an act-for Conduit is reviewed under ruling 54;
  - the Agent-administration flips in the vectors are named.

What I drop:
- M2's G1 to G4. G1 contradicts ruling 51 and narrows root without saying so.
- M3's agents Conduit (a Project to itself).
- The `projects` item on bound.v1.
- The dispatch broker.
- The automatic binding close of the previous round (M4).
- The rule that only root reconciles an open made under a closed authority (D6).

What stays true in v0, and is stated in spec/README.md:
- A Project organizes authority, attributes acts and contains mistakes made through Conduit. It isolates no execution.
- Every Agent on the engine's own runner, and every client of the owner's, runs as the owner's OS user. It can read any token in the store and present as root (IFC-022, DL16).

1. THE FAMILY
- Project is a sixth family (OBJ-020: eleven kinds). Like a Pool, it is owned by nothing.
- Every Project is an instance of the core template core#project, Type core#project.v1, served by `engine` (MAN-012, TPL-004).
  - It has no config, placement, runtime, hooks, entries or declared Conduits.
  - Its setup has no parts, and its shutdown is confirmed at once (LIF-024).
  - A Type of the family Project from any other package is rejected.
- Its capability is core#project-management.v1, whose operations are all management verbs (CON-024, OPS-019):
  - register.v1, register-replacement.v1, inspect.v1, set-up.v1, retire.v1 and reconcile.v1, about its Agents;
  - list.v1 (section 4);
  - retire-with-agents.v1, the LIF-021 mirror;
  - act.v1 (section 6), added in batch 2 as a further operation, which NAM-024 allows.
- Owning permits nothing. ACC-028 gains: "A Project is never a caller. A Conduit a Project holds is used only through act.v1, by the consumer of an open Conduit to the Project that lists it."
- Interfaces, Pools, Resources, nodes, `system` and the first-start objects stay at the engine level. No Interface is owned.

2. AGENTS ARE OWNED AS RESOURCES ARE (ruling A1)
- Every Agent gives the standard argument `owner: <Project>`.
  - It is required and fixed for life (PRJ-002, the POOL-001 mirror).
  - It names a Project that exists and is not retired, or the document is rejected (PRJ-003, the POOL-004 mirror).
  - An Agent wanted in another Project is declared there anew (ruling 54 point 1). It takes a new name, or the old Agent is retired and its tombstone lifted (ruling 55 A3, A7).
- Ruling 30's OL2 becomes: "its owning Pool for a Resource, its owning Project for an Agent, `management` for everything else".
- ACC-033 gains the Pool rows, mirrored for Agents:
  - 1A. Registering an Agent whose owner is a Project that exists and is not retired: `management` AND that Project, each Conduit listing register.v1. An Agent registered in the same submission as its Project falls under row 2.
  - 3. register-replacement.v1 follows rows 1, 1A or 2.
  - 4A. Setting up or retiring an Agent: its Project, and none where the Project is retired. LIF-026 is mirrored: an Agent of a retired Project whose shutdown is unconfirmed is retired again through `management`.
  - 6A. Inspecting an Agent: its Project, or `management` where the Project is retired.
  - 8A. retire-with-agents.v1: that Project.
  - 14A. Reconciling an invocation dispatched to an Agent: its Project, or `management` where the Project is retired.
  - 15. act.v1: that Project.
  - 16. list.v1: the container itself.
  - Rows 5, 7 and 14 lose Agents. Row 12 (an instance that no longer exists goes to `management`) is unchanged.
  - A Project is registered, set up, retired and inspected through `management`, as a Pool is.
  - "No request is authorized against a retired Pool" gains "or a retired Project".
  - The sentence on objects that do not exist stays: authority over them is `management`'s alone.
- What is mirrored and what is not:
  - Mirrored: POOL-001 and POOL-004 (owner, fixed and live), rows 1, 3, 4, 6, 8 and 14, LIF-020, LIF-021 and LIF-026.
  - Not mirrored:
    - Pool entries (POOL-002 to POOL-017's entry promises): a Project has no entries.
    - ACC-029's coverage: a bound that names a Project (as an item of `targets`) covers the Project alone, never its Agents. A right over a Project's administration therefore never silently covers turns on its Agents.
- Retiring (ruling A2):
  - Plain retire.v1 of a Project that owns an Agent that is not retired, or whose shutdown is not confirmed, reports conflict (LIF-038, the LIF-020 mirror).
  - retire-with-agents.v1 retires each Agent in reverse setup order, then the Project (LIF-039).
- The real narrowing, stated: in v0 nobody who administers only a Project registers anything, because registering needs `management`. A78's workers are therefore fixed slots that root registers (ruling B12).

3. POOLS ARE INDEPENDENT
- POOL-001 to POOL-017 are unchanged. A Pool declares nothing about who uses or administers it.
- "Shared" means only that Conduits to the Pool exist, or that bounds name it. Each such Conduit was opened or made by someone with authority over the Pool (ACC-006): the Conduit is the consent (rulings 27 to 29).
- There is no rule of one administrator per Pool. Instead:
  - inspecting a Pool lists every Conduit to it, through ruling 53's inventory;
  - the guidance and the examples have root alone administer and write a Pool that several Projects use;
  - a rule or a pending action for harm across projects is ruling C5.

4. ROOT'S STANDING AUTHORITY AND DISCOVERY (rulings A3, A4)
- PRJ-005: when a Pool or a Project is registered, the engine instantiates and opens a Conduit from root to it, within that registration.
  - For a Pool it lists every operation of pool-management.v1 except retire-with-resources.v1.
  - For a Project it lists every operation of project-management.v1 except retire-with-agents.v1 and act.v1.
- Names:
  - The name is generated (OBJ-003), and `as` is the target's name.
  - A Pool and a Project may not share a name.
  - `management`, `access` and `system` are reserved as names of Pools and Projects, because root's first-start Conduits use them as local names.
  - Registering a Pool or a Project whose name is already the `as` of one of root's Conduits that are not closed (for example root's operator Conduit to an Agent of that name) reports invalid, naming that Conduit (IFC-014's diagnostic). Nothing is registered.
  - Defaults are exempt from tombstones, so a Pool re-created under a stable name (ruling 55 A3) gets one again.
- A default is shown and recorded as an event on REC-001's closed list. No request closes it, and it closes when its target retires (CON-008).
- ACC-024 amended (ruling A3(d)): a Conduit whose consumer is root is closed only at root's own request, made on its own behalf. It is never closed where it is one of root's first-start Conduits or a default. CON-008 still closes one whose target is retired. So a cascade Conduit that root opens to use once is closed after use: `conduit retire packages --with-agents` opens it, calls it and closes it, and the plan prints all three.
- `system` is included, as a third first-start Conduit of root, from the core template root-system, made at first start (amends FST-002, FST-003 and FST-005, and ruling 30 OL5).
  - With it, root inspects and administers `management`, `access`, nodes and providers through rows 4 and 6.
  - LIF-022 still refuses retiring first-start objects.
- Discovery through core#list.v1 {kind?}: a new operation (additive under NAM-024) on object-management.v1, pool-management.v1 and project-management.v1, authorized against the container (row 16).
  - Through `management`, it lists every kind:
    - Interfaces, Pools, Projects, templates, Conduits, packages and nodes;
    - tombstones and withholds;
    - the instances of a retired Pool or Project, which are inspected through `management` (rows 6 and 6A).
  - A Pool lists its Resources.
  - A Project lists its Agents, the Conduits it holds (with provenance) and the withholds against it.
  - It shows only what the caller may inspect, with no counts.
- Inventory, read with ruling 52's tier 2 (ruling A4):
  - Ruling 53's inventory of an object (the Conduits it consumes and is the target of) shows each entry in full (consumer, target, operations, provenance, marks) wherever the caller may inspect that object.
  - Inspecting a Conduit by name stays `management`'s (row 7), so no row changes.
  - An administrator of a Project therefore sees every Conduit its Agents hold and are the target of, the grants it made among them.
- So root walks the whole engine from its own Conduits. It needs no bypass and writes nothing for its own authority.

5. WHAT A PROJECT HOLDS (ruling B2)
- PRJ-006, language layer: a Conduit whose consumer is a Project is one of two kinds. Anything else is rejected.
  - E1, administration:
    - Its target is a Pool other than `system`, with pool-management.v1.
    - It may list inspect, list, set-up, retire and reconcile.
    - It never lists register.v1, register-replacement.v1 or retire-with-resources.v1.
  - E2, a right to grant:
    - Its target is `access`, with access-control.v1, and it must apply bound.v1.
    - It may list make.v1, open.v1, close.v1 and reconcile.v1. It never lists lift (ruling 55 A7), keep.v1 or withhold.v1.
    - Its bound names none of: register.v1, register-replacement.v1, retire-with-resources.v1, act.v1 or get-secret.v1, nor any operation of access-control.v1 (closes M2 at the language layer).
- A Project may hold several E1s and E2s. Each is revoked on its own, and the call names which one is used (no union, ACC-009).
- Who gives them: anyone whose own right covers them, under unchanged ACC-006 and ACC-030. That means root, or any Interface or Agent within its bound ("as it is or narrower"). They do so by opening a registered one (open.v1) or by making one (make.v1, section 9). Through a Project's own E2, its actors may also make it an E1 (section 8).
- What it never holds:
  - use or turn Conduits, which are v1;
  - Conduits to `management`, `system`, a Project, an Agent or an Interface;
  - get-secret.v1 (SEC-009 gains a Project consumer).
- A Project is presented no server, and no credential names a Project. A request presented from outside the engine as a Project's reports refused (PRJ-009).

6. ACTING FOR A PROJECT, BY FORWARDING (rulings B3, B4)
- The act-for Conduit (E3) runs from an actor to P, with project-management.v1.
  - It lists act.v1, and whichever of inspect, list, set-up and reconcile of P's Agents root chooses. The examples omit retire, so that one call cannot remove a fixed slot.
  - A viewer or operator is a Conduit to P without act.v1. It administers P's Agents through rows 4A, 6A, 14A and 16, and can be delegated within a bound like any administration.
- PRJ-007, language layer: the consumer of a Conduit that lists act.v1 is either an Interface, or an Agent that P owns.
  - An Agent acts only for the Project that owns it. An Interface may act for several (ruling B4).
  - A Project is never such a consumer: one level only.
- act.v1's input is {through, operation, input, partsThrough?}. An actor calls the expanded tool `conduit-core.trees.make.v1 {X}`, and IFC-012 presents it as act.v1 through the actor's E3.
  1. ACC-001 decides it as written:
     - E3 is open and lists act.v1, and row 15 names P;
     - ACC-039 applies: neither the actor nor P is retired;
     - REC-002 records it (caller, E3, consumer actor, target P, act.v1);
     - act.v1's contract records its inputs.
  2. PRJ-008: the engine, as P, sends exactly one onward request through the held Conduit named in `through`, naming the act invocation in `serving`.
     - The engine makes this request within itself, through no server and with no credential, as it already makes its read of a secret through a consumer's Conduit (SEC-010).
     - IFC-015 gains: "for a Project, the engine within itself, and only as act.v1's onward request".
     - IFC-022 gains: "a request whose sender is a Project is never received on a server, and is attributed to that Project by the engine that made it".
     - Setup parts are not the precedent: they are sent as the requester.
  3. That onward request is decided as written:
     - ACC-016: P's own open Conduit to the target, which is always "another target";
     - ACC-006, ACC-012, ACC-030 and ACC-033;
     - ACC-017: it carries the act's caller and E3's filters, of which there are none (CON-022);
     - plus confinement (section 8), since its sender is a Project.
  4. OPS-035:
     - act's outcome and outputs are its onward request's. A repeat of act is answered from act's recorded outcome.
     - The onward idempotency key is derived from the act invocation's ID, so that one act invocation has exactly one onward invocation, whatever attempts act makes (OPS-024).
     - IFC-020 already scopes keys to sender, caller and target. That earlier reason was wrong.
     - Reconciling the onward invocation settles act.
  5. Setup parts (PRJ-008):
     - Where the forwarded operation is set-up.v1 of an instance with declared Conduits, its parts are sent by P (ACC-005). They are decided by the held E2 named in `partsThrough` alone, which must list open.v1 and cover each part. That E2 is recorded as their `openedThrough`.
     - Where none is named, each part reports refused at tier 1, naming the missing input.
     - A tree (fs#local-directory.v1) declares no Conduits, so in the owner's use this never arises.
- What follows from forwarding:
  - Only the Conduits named (`through`, and `partsThrough`) are consulted.
  - E3 is the Conduit of the served invocation. So closing E3, or retiring the actor, stops in-flight onward requests through ACC-034 and ACC-039 as written.
  - `through` naming a Conduit that the target Project does not hold reports invalid.
  - act forwards only management verbs (PRJ-006), so it never starts model work (test 6).
- An actor's direct administration of P's Agents goes through its own Conduit to P (E3 or a viewer), under rows 4A, 6A and 14A. It is not forwarded.
  - Setup parts are decided by the actor's own Conduit to `access` (ACC-005, unchanged).
  - Named limit: an Agent whose required declared Conduits the actor's own right does not cover is set up by root. The short form writes an Agent's Conduits as standalone Conduits, so most Agents have no parts.
- Tools (IFC-012, IFC-013, IFC-014, IFC-033, IFC-038, IFC-048):
  - For E3 with `as: conduit-core`, the actor sees one tool for each operation of each open Conduit P holds, named `<E3 as>.<held as>.<export>`, plus E3's own tools. No `act.v1` tool is shown.
  - The expansion keys on the qualified core#act.v1. The records vectors use an unrelated example.org/repair#act.v1.
  - Descriptions name P, the held Conduit, its target and Type, and its bound (ruling 53).
  - Tools go when E3 or the held Conduit closes, and every E3 holder is notified when P's Conduits change.
  - The local name of an E3, or of a Conduit whose consumer is a Project, is at most 31 characters (31+1+31+1+63 = 127).
- Records: one project action is two records, the act and P's onward request, linked by `serving`. Test 7 reads: actor (the act's consumer), caller, Project (the sender), and both Conduits.

7. APPOINTMENT AND DEPUTIES (rulings B5, B6, B13)
- ACC-045, in SEC-008's form: a request to open or make a Conduit that lists act.v1 is dispatched only where the root Interface sends it on its own behalf.
  - A setup part that would open one is refused unless root sent the setup on its own behalf.
  - Closing one follows ACC-012 like any Conduit, so a bound can grant or withhold it.
- ACC-046: these are dispatched only where root sends them on its own behalf:
  - opening or making any Conduit whose target is an Agent that is the consumer of an open act-for Conduit, for any operation (turns and controls included);
  - setting up or retiring such an Agent, retire-with-agents of a Project that owns one included.
- Root's appointment of an Agent that is already the target of an open Conduit from another consumer is refused, naming those Conduits. Root closes them first.
- Secrets and actors (ruling B13(a)):
  - SEC-008 gains: a Conduit that lists get-secret.v1 is not opened or made to an Agent that the consumer of an open act-for Conduit can take turns on.
  - ACC-046 gains the reverse: a Conduit that lists a startsModelWork operation, and whose consumer is such an actor, is not opened or made to an Agent that consumes a get-secret Conduit.
  - Both bind root too, as SEC-009 does.
- Closing an actor's own Conduits is not reserved. It follows ACC-012: a delegate within its bound can close them. That narrows, and root can undo it by lifting the tombstone (ruling 55 A7). Retiring cannot be undone (forward-only lifecycle), which is why only retiring is root's.

8. CONFINEMENT OF WHAT FLOWS THROUGH A PROJECT (ruling B7)
- ACC-047 applies to every request whose sender is a Project P, that is, to every forwarded request and to the setup parts P sends. Beyond ACC-006 and ACC-012, such a request is dispatched only if the following hold.
  - (a) To make or open, the new Conduit's consumer is one of:
    - P itself, and then its target is a Pool other than `system` and it lists only operations an E1 may list;
    - an Agent P owns that is the consumer of no open act-for Conduit;
    - for a part of a setup P sends, the instance being set up.
  - (b) Its target is a Pool other than `system`, or a Resource that `system` does not own. It is never `access`, `management`, a Project, an Agent or an Interface.
  - (c) Whatever the consumer, it lists none of these: register.v1, register-replacement.v1, retire-with-resources.v1, an operation marked startsModelWork, get-secret.v1, or an operation reserved to root. This closes M2.
  - (d) To close: only a Conduit whose `sentBy` is P. Never root's or anyone else's direct grants, so nothing is laundered.
  - (e) To reconcile: only an invocation whose sender is P or an Agent P owns.
  - (f) P never lifts, keeps or withholds (PRJ-006).
- Refusals:
  - A refusal is tier 1 and reads "the consumer is not within conduit-core" or "not a target a project may grant".
  - Through a Project, a Conduit that does not exist and one outside confinement both report refused before any existence check (OPS-023). There is no oracle across projects.
- What follows:
  - P's actors give P, or P's Agents, use of Resources and administration of Pools within P's bounds, with E1's exclusions applied to every consumer.
  - They cannot:
    - grant to Interfaces (themselves included), to root, to another Project's Agents, or to P's actors;
    - pass on a right to open, or take turns on anyone through P;
    - close grants P did not send.
- Today's delegation is untouched. A request whose sender is an Interface or an Agent is not confined, and conformance/conventions/access/bounds keeps all 17 outcomes.

9. MAKING A CONDUIT (ruling B8)
- ACC-048 adds core#make.v1, a new operation of access-control.v1 (additive under NAM-024).
  - Input: {from: a registered Conduit template, consumer, target, as}.
  - The engine registers and opens one Conduit instance atomically. It is authorized by the bound (ACC-006, ACC-030), not by `management`.
  - It carries no inline template, contracts or manifest.
- Every condition on opening applies (closes M1). For every promise that conditions a request to open a Conduit, a request to make one counts as one: ACC-006, ACC-022, ACC-024, ACC-030, ACC-032, ACC-045, ACC-046, ACC-047, ACC-049, SEC-008 and CON-031.
  - The made instance is checked as any instantiation is, under SEC-009, IFC-014, CON-021, CON-028, PRJ-006, PRJ-007, OBJ-001, OBJ-003 and ruling 53's tombstones.
  - Where any condition refuses, nothing is registered.
- Consumers:
  - A request sent on the actor's own behalf names either the actor itself or a Project. A Project gets only an E1 or E2, under PRJ-006 at instantiation. This closes D3: a non-root Interface gives a Project a Conduit as often as its right allows.
  - A request sent by P is confined by ACC-047.
  - Root makes for any consumer, and only root, on its own behalf, makes one whose consumer is root. Ruling C1 asks whether to apply that last rule to open.v1.
- Names: OBJ-003's form, with `<consumer>-<as>` in place of the template's name, shortened under DL15 so the whole name fits OBJ-001's 63 characters (for example core-w1-tree-x7k2p). No caller chooses a name, squats one or probes with one.
- `expectedId` (ruling 53) does not apply, since the object is new.
- No existing Conduit gains make.v1: it must be listed, and a bound can withhold it.
- It amends ruling 44 point 2 (D04) for Conduits only. A Conduit runs no code, and registering Agents and Resources still needs `management` or a broker.

10. RULING 54: PROVENANCE, REVIEW, KEEP AND WITHHOLD (rulings B9, B10, B11)
- ACC-044, in batch 1: every Conduit opened or made by a request holds the following as state. It is shown on inspection and kept in its tombstone, so that no purge erases it (REC-004).
  - `actor`: the requester, or for a forwarded request the act's consumer;
  - `openedThrough`: the Conduit to `access` that decided it. For a setup part named by `partsThrough`, that E2. For any other setup part, every Conduit to `access` of the sender that covered the part on its own when it was decided, so the record no longer depends on which was opened first (H1 F16);
  - `actingFor`: the act-for Conduit, when forwarded;
  - `sentBy`: the sender.
  - Engine-made Conduits state their `cause` instead (first start, default).
  - Snapshots carry no provenance, and inspection shows none for their Conduits.
- ACC-050: closing a Conduit that a grant was made through triggers the review, however it closes: by request, by retirement or by CON-008. That covers any Conduit to `access`, whoever holds it, and any act-for Conduit, for the grants made under it.
  - The close lists the grants:
    - The `_meta` key io.github.shrav-k.conduit/granted-under, on the invocation within which it closed, lists every grant still open under it.
    - A grant whose `openedThrough` lists others that are still open is listed with them.
    - The key is present only when the list is not empty, so close.v1's outputs stand.
  - The pending action:
    - One pending action is raised at the root Interface (ruling 50's mechanism, at root in v0).
    - It lists the grants transitively.
    - Its expiry is "none", which reads ruling 51 decision 5 for this kind (put in B9).
    - Its answers are "closed" or "kept" for each grant (ruling 56 point 8).
  - Marks are computed, not stored. A Conduit is marked "granted under a closed authority" while every Conduit in its `openedThrough` is closed, or its `actingFor` is closed, and it has not been kept.
    - So a grant that opens later, in flight or reconciled as occurred, is marked, and its opening raises the pending action.
    - Reconciliation follows the ordinary rule, ACC-047(e) for P. No special rule is needed, so D6's exception 6 is gone.
  - Nothing cascades: ACC-030's "closes none" stands.
  - `conduit close --listed` sends individual closes under one named flag (ruling 55 A4).
- ACC-051: keep.v1 is an operation of access-control.v1 that a bound can withhold.
  - It is dispatched only where all of these hold: the keeper's bound covers the grant; the keeper is neither the grant's `actor` nor the consumer of a Conduit in its `openedThrough` or its `actingFor`; and the request is not forwarded.
  - It is recorded and removes the mark. root-access lists it.
- ACC-049, withhold (replaces the automatic binding close, closes M4): withhold.v1 is an operation of access-control.v1 that a bound can withhold.
  - Input: {project, consumer, target}.
  - While a withhold stands, make.v1 and open.v1 sent by that Project, through any Conduit it holds, refuse that consumer and target at tier 1, naming the withhold.
  - It is dispatched only where the requester's bound covers the target, and never forwarded.
  - It is lifted with ruling 55 A7's lift operation, under the same guard as keep. In v0 root holds both, unless it gives them.
  - Nothing withholds implicitly. A close, by root or anyone, never withholds. Root revokes and holds a grant with `conduit close <grant> --withhold`, one command sending close.v1 and withhold.v1.
  - The routine closes at merge and the review's closes withhold nothing, so B12's fixed slot pairings stay usable.
  - Withholds are listed through `management` and in the Project's list.v1. A withhold is the one piece of state about Conduits held outside a Conduit. It refuses, never permits, so it is not a grant object (ruling B11).
- Retirement:
  - A retired Agent's Conduits close (CON-008, ruling 54 point 5).
  - retire-with-agents:
    - the E3s to P close at once, because their target is retired;
    - P's E1s and E2s close at its immediate shutdown;
    - every grant made through P has P or P's Agent as consumer, and closes with them;
    - Pools are untouched.
- Stopping a project now means either:
  - closing its act-for Conduits (the next decision refuses, and in-flight onward requests stop), then answering the pending action they raise; or
  - retiring it with its Agents.
  Root's own direct grants to its actors, such as standing turns on workers, are closed by root separately. The Project's inspection lists them, with what each reaches.

11. INSPECTION
- Inspecting a Project lists:
  - its Agents (list.v1);
  - the Conduits it holds, with their provenance;
  - its actors (the consumers of E3s);
  - the withholds against it;
  - for each Agent, the consumers of Conduits to it that list a startsModelWork operation;
  - "unconfined: runs as the engine's OS user" on every instance on the engine's own runner.
- Reach marks (closes M3's visibility), computed and one hop deep:
  - For every non-root consumer that holds a startsModelWork Conduit to an Agent, inspecting that consumer, the Agent, or any Conduit the Agent holds shows "reachable by <consumer> through <turn Conduit>".
  - The plan prints the same line whenever root's change adds a Conduit to such an Agent, or a turn to an Agent that holds Conduits.
  - Transitive reach is v1 (C12).
- An Interface that acts for more than one Project is marked "bridges: P, Q" (ruling B4).
- Inspecting a Conduit shows its provenance, its mark and any keep.
- Inventory entries follow section 4's reading, so an actor sees the grants it made by inspecting its Project's Agents through its E3.
- Root's refusals are always tier 2 (ruling 52).
- v0 does not filter records by Project (ruling C10).

12. NAMES
- Names stay engine-wide per kind (OBJ-002).
- A Pool and a Project may not share a name, and `management`, `access` and `system` are reserved for both.
- Names of made Conduits are generated (OBJ-003, DL15).
- Per-project names are v1 (ruling C8).
- The old word "project" is folded into "submission" (ruling A5).

13. LIMITS, WRITTEN IN spec/README.md "Limits of this version"
- A Project organizes authority and isolates nothing.
- Every actor in v0 is an unconfined Agent or the owner's own Interface. Every client token sits in a store directory any program of the owner's OS user can read. Confinement, appointment, deputy and secret rules guard against mistakes and injection through Conduit, not against compromise (IFC-022).
- An actor that holds a turn on an Agent wields every Conduit that Agent holds, now and later, root's own direct grants included. Only secrets are ruled out (B13). Everything else is shown, not prevented.
- A shared writable Resource carries instructions between Projects.
- A bound covers a Pool's later Resources (ACC-029) and does not limit filters (ruling 44 D03). A bound item that names a single Resource stops covering it when the Resource is replaced under its name (ruling A7).
- act.v1 is all or nothing over everything P holds, now and later. Anyone who gives P a Conduit, any delegate within its bound included, widens every actor of P.
- An Interface that acts for two Projects carries what it reads between them.
- Outsiders within their bounds may still open or close Conduits for P's Agents (ACC-030, ACC-012).
- A Resource of a shared Pool that a Project sets up gets parts whose provenance is that Project's. Their review then touches everyone, so the guidance has root administer shared Pools.
- An Agent's own means are outside the documents.

14. THE OWNER'S WORDS: how each is honoured, from when, and where it departs
- "I don't think pools themselves should declare who can use them": honoured in full, from batch 1.
- "the interface should still be able to give conduits to projects to manage them directly": honoured from batch 2.
  - Every Interface or Agent whose right covers the Conduit can open or make it for a Project, and can do so again later (B2, B8).
  - The other reading, appointing actors, is root's (B5).
  - Counterexample (H1 F3): ops-lead, whose bound lets it act for packages, makes itself an act-for Conduit. Root later gives packages an E2 over `auth`, and ops-lead gains auth.
- "Doesn't the root interface have discovery and admin power over everything?": made true and standing from batch 1.
  - Root holds defaults to every Pool, `system` included, and to every Project.
  - list.v1 covers every kind.
  - Departure: the cascades are not standing tools, and root uses one per command with a named flag (A3(d)).
- "Projects should be able to still open admin conduits to pools": honoured from batch 2.
  - Through an E2 whose bound names pool-management operations, actors make E1s for P and admin Conduits for P's Agents.
  - Exclusions, each with its counterexample:
    - `system`, because a Project would retire the machine provider;
    - register.v1, because a directory registered at ~/.ssh into a Pool the bound names is covered at once by ACC-029;
    - retire-with-resources, because packages would retire `skills`, which conduit-core uses. This applies through a worker too (M2).
- "What if agents were tied to projects like resources are tied to pools?" and "This matches my intention!": the mirror, from batch 1 (A1, A2), with the parts not mirrored named in section 2.
- "I like the idea of root having conduits to all projects and pools by default also": honoured literally, from batch 1 (A3).
- "No, a new agent needs to be declared in the new project": honoured (PRJ-002).
- "Agents can be given their own conduits to pools and resources": honoured. Root and any delegate can do it today and still can, and P's actors can through P's E2 from batch 2.
  - Departure: not turns through a Project (B7).
- "...revoked individually when a project loses their conduit": honoured, and extended to every authority a grant was made through, act-for Conduits included (B9).
- "Yes the root can deliberately keep it, when the agent is retired the conduits automatically close": honoured. Keep follows ruling 55 A7's pattern, with a self-review guard (B10).
- "Interfaces could have authority over one or multiple projects and this is how to have more granular interfaces": honoured in v0 for the owner's own Interfaces, one or several Projects each, with their own tokens and the bridge marked (B4).
  - Departure: second persons wait for rulings 15 and 49.
- "skills pool for all projects, auth pool for a specific project": honoured literally, from batch 2.
  - Each Project holds an E2 whose bound names the Pools `skills` and `repo`, read only.
  - Only conduit-core's names the Pool `auth`, for the Linear service's operations.
  - "All" means one E2 per Project.
- The owner's earlier "projects also determine an interface's permissions?": a Project determines what an Interface administers through it, never what it uses (C9).

15. PRESERVATION TESTS AND AMENDED RULINGS
- Test 5 exceptions, named:
  - (1) Opening or making act-for is root's (B5).
  - (2) Conduits to an actor, and setting up or retiring an actor, are root's (B6).
  - (3) An Agent acts only for its own Project (B4).
  - (4) Existing Conduits to `management` stop covering Agents (A1). The action stays available to agents through a Conduit to the Project.
  - (5) Keep is root's, if B10(b) is chosen.
  - (6) Closing a Conduit whose consumer is root is root's (ACC-024 amended, A3(d)). It was nobody's before.
- Not exceptions:
  - confinement, which narrows only what flows through P;
  - registering an Agent, which needs `management` AND the Project;
  - make.v1, a new power that removes nothing;
  - withhold, keep and lift, each an operation a bound can grant or withhold;
  - B13's secret rule, which binds root too, as SEC-009 does;
  - reconciliation, which is unchanged.
- Test 2: authority through a Project is revoked per actor (E3) or per held Conduit. Root's defaults and first-start Conduits stay unclosable.
- Test 8: in v0 these depend on root: appointing, deputies, defaults, pending actions at root, and making for root. The language keeps every path, and v1 lifts them by ruling (C6).
- Test 9: a confinement refusal reports its restriction as mediated (ACC-027).
- Tests 1, 3, 4, 6 and 7 hold.
- Rulings amended, each put to the owner:
  - 8 (`owner` becomes a standard argument of Agents);
  - 24 and 39's first start (root-system);
  - 29 (six families, and definitions.md's "family");
  - 30 OL2 (owning Project);
  - 30 OL5 (root's permanent Conduits become three, plus the defaults);
  - ACC-024 as accepted under ruling 39 (root closes its own);
  - 42, with the wording of 25, 27 and 37 (the rename);
  - 44 point 2 (make.v1);
  - 51 decision 5 (no expiry for a review) and 51's peer mode (C4);
  - 53 decision 3 (list.v1 and the inventory reading) and decision 6 (answered by the engine; SCT-010 dropped);
  - 54, extended (B9) and read (B10, B11);
  - 55, its grammar extended (A6), and A7's lift extended to withholds;
  - 56 point 8 (the word "answer", used, not amended).
- Not amended: 15, 17 V3, 27, 28, 40, 49, 50, 52, ACC-001, REC-002, and ACC-030's "for any consumer" for open.v1.

## documents
Abbreviations used only here; the files write every address in full (rulings 25 and 55 A2):
- core# = github.com/shrav-k/conduit/core#
- fs# = example.org/s1/filesystem#
- hist# = example.org/s1/history#
- cc# = example.org/conduit/agent-claude-code#
- cx# = example.org/conduit/agent-codex#
- op# = a 1Password package
- lin# = a Linear MCP package

These are paper documents. Today's loader rejects `project:`, `owner` and kind Project.

Short-form rules assumed, each a numbered SCT rule added to ruling 55's batch:
- A top-level `project: <P>` makes Project P from core#project and sets `owner: P` on every Agent in the file. Identical Project documents from several files of one submission are one.
- `projects: <P>: holds: <k>: {to, from}` makes a Conduit with consumer P and `as: k`, which root opens at apply.
- `acts-for: <P>: {from}` under an actor makes its act-for Conduit to P, with `as: P`, which root opens.
- `operator:` is root's turn Conduit.
- SCT-010 of the short-form synthesis is dropped: the engine makes root's Conduit to each Pool.

A. S1 AS ONE PROJECT: s1.yaml at T1 (20 lines; the lines marked T2 are added at T2)
```yaml
project: s1                                   # the one Project line: Project s1, owner of every Agent below
pools:
  workspace:                                  # not "s1": a Pool and a Project may not share a name
    type: core#pool.v1
resources:
  shared:
    type: fs#local-directory.v1
    owner: workspace
    config: {root: /lab/shared}
  history:                                    # T2
    type: hist#file-history.v1                # T2
    owner: workspace                          # T2
    config: {root: /lab/history}              # T2
agents:
  agent-a:
    type: cc#claude-code.v2
    operator: {capability: cc#conversation.v1, operations: [cc#turn.v1]}
    consumes:
      project: {to: shared, capability: fs#directory.v1, operations: [fs#file.read.v1, fs#directory.list.v1]}
      history: {to: history, capability: fs#directory.v1, operations: [fs#file.read.v1, fs#directory.list.v1, fs#file.write.v1]}   # T2
  agent-b:
    type: cx#codex.v2
    operator: {capability: cx#conversation.v1, operations: [cx#turn.v1]}
    consumes:
      project: {to: shared, capability: fs#directory.v1, operations: [fs#file.read.v1, fs#directory.list.v1, fs#file.write.v1]}
      history: {to: history, capability: fs#directory.v1, operations: [fs#file.read.v1, fs#directory.list.v1, fs#file.write.v1]}   # T2
```
- What the engine makes that nobody writes:
  - root's Conduit to `system`, at first start;
  - root -> workspace (pool-management.v1 without the cascade) and root -> s1 (project-management.v1 without act.v1 and retire-with-agents), within the registration.
- Root sets up `shared` through `workspace.set-up.v1` (row 4), and agent-a and agent-b through `s1.set-up.v1` (row 4A). Their `consumes` are standalone Conduits, so their setup has no parts.
- No name collides: root's local names are management, access, system, workspace, s1, agent-a and agent-b.
- S1 holds no E1, E2 or E3: with one person, root does everything directly.
- The canonical fixture diff:
  - added: one Project document, and `owner: s1` on the two Agents;
  - removed: root's written Conduit to workspace and the template workspace-admin, which root's default replaces.

S1 lines per task, by the measurement plan's primary rule (file lines, plus command lines that carry values). These are paper counts; ruling 45 accepts only a run.

| Task | Baseline (A70) | Short form (ruling 55) | This model | What is written |
|---|---|---|---|---|
| T1 | 33 | 20 | 21 | the 20 lines, `project: s1` among them, then `conduit apply` |
| T2 | 16 | 7 | 7 | the 6 T2 lines, then apply |
| T3 | 0 | 0 | 0 | `conduit list s1` and `conduit inspect agent-a` carry no values |
| T4 | 8 | 2 | 2 | edit the two `project` grants |
| T5 | 3 | 1 | 1 | `conduit install filesystem-0.2.0.tgz`; `shared` is replaced break-before-make under its name (ruling 55 A3) |
| T6 | 2 | 1 | 1 | edit `config.root` |
| T7 | 2 | 2 | 2 | delete agent-a's `project` line, then restore it with `--reopen` (root lifts the tombstone, ruling 55 A7) |
| Total | 64 | 33 | 34 | |

- The result is less work on 5 tasks, the same on T3 and T7, and more on none, so it passes the primary reading.
- With ruling 55's reusable Conduit templates (`from: read`), T1 is about 25 and the total about 38: still better on 5 tasks.
- Risk: at the fresh author's 1.7 times factor, T1 is about 36 against the baseline's 33. That marginal fail is inherited from the short form (20 × 1.7 = 34); the Project adds about 2.
- The alternative of an explicit `owner:` on each Agent plus a `projects:` section (ruling A6(b)) gives T1 24 and a total of 37.
- T5 under ruling A7: no bound in S1 names `shared`, so its replacement drops no coverage.

B. THE OWNER'S REAL USE (v0 after batch 2; the v1 additions are marked)

engine.yaml (root's: the shared objects; no `project:` key)
```yaml
pools:                            # independent; none declares who uses it; root's default Conduit to each is the engine's
  skills: {type: core#pool.v1}    # root alone administers and writes it
  auth:   {type: core#pool.v1}
  repo:   {type: core#pool.v1}
  orders: {type: core#pool.v1}    # one queue per project, never a shared writable one
resources:
  skill-library: {type: fs#local-directory.v1, owner: skills, config: {root: ~/skills}}
  main:          {type: fs#local-directory.v1, owner: repo,   config: {root: ~/conduit-main}}
  linear-key:    {from: op#item, owner: auth, arguments: {item: linear-api}}
  linear:        {from: lin#linear-mcp, owner: auth}   # declares key -> linear-key [core#get-secret.v1]: opened only when root sets it up (SEC-008); the secret goes to this Resource, never to an Agent
templates:
  conduits:
    read:       {capability: fs#directory.v1, operations: [fs#file.read.v1, fs#directory.list.v1]}
    read-write: {capability: fs#directory.v1, operations: [fs#file.read.v1, fs#directory.list.v1, fs#file.write.v1]}
    queue-use:  {capability: hist#history.v1, operations: [hist#read.v1, hist#post.v1]}
    issues:     {capability: lin#issues.v1, operations: [lin#list-issues.v1, lin#get-issue.v1, lin#create-issue.v1, lin#update-issue.v1]}
    cc-turn:    {capability: cc#conversation.v1, operations: [cc#turn.v1]}
    cx-turn:    {capability: cx#conversation.v1, operations: [cx#turn.v1]}
    run-pool:   {capability: core#pool-management.v1, operations: [core#inspect.v1, core#list.v1, core#set-up.v1, core#reconcile.v1]}            # no retire: a fixed slot cannot be registered again by a project
    manage:     {capability: core#project-management.v1, operations: [core#act.v1, core#inspect.v1, core#list.v1, core#set-up.v1, core#reconcile.v1]}
    grant-shared:                 # "skills pool for all projects": read on the Pools skills and repo
      capability: core#access-control.v1
      operations: [core#make.v1, core#close.v1, core#reconcile.v1]
      requirements: {bound: {requirement: core#bound.v1, arguments: {pools: [skills, repo], operations: [fs#file.read.v1, fs#directory.list.v1]}}}
    grant-linear:                 # "auth pool for a specific project": the Pool auth, for Linear's operations only
      capability: core#access-control.v1
      operations: [core#make.v1, core#close.v1, core#reconcile.v1]
      requirements: {bound: {requirement: core#bound.v1, arguments: {pools: [auth], operations: [lin#list-issues.v1, lin#get-issue.v1, lin#create-issue.v1, lin#update-issue.v1]}}}
```
- Why grant-linear names the Pool: naming `auth` covers linear-key too, but linear-key offers only get-secret.v1, which confinement forbids (ACC-047(c)). So this bound reaches the Linear service alone. Unlike a bound naming `linear` by ID, it survives an upgrade that replaces `linear` under its name (ruling A7).

conduit-core.yaml
```yaml
project: conduit-core
projects:
  conduit-core:
    holds:                                            # consumer: the Project; root opens them at apply
      work:   {to: core-work, from: run-pool}           # E1: administers its trees
      trees:  {to: access,    from: grant-core-trees}   # E2: read-write on core-work, plus inspect and set-up of its trees
      shared: {to: access,    from: grant-shared}       # E2: read on skills and repo
      mint:   {to: access,    from: grant-linear}       # E2: Linear, for this project only
pools:
  core-work: {type: core#pool.v1}
resources:
  tree-c1:    {type: fs#local-directory.v1, owner: core-work, config: {root: ~/wt/core-1}}   # tree-c2 and tree-c3 the same; no declared Conduits, so setup has no parts
  core-queue: {type: hist#file-history.v1,  owner: orders,    config: {root: ~/orders/core}}
templates:
  conduits:
    grant-core-trees:             # no open.v1: no tree has setup parts; a Resource with parts would need open.v1 here and partsThrough
      capability: core#access-control.v1
      operations: [core#make.v1, core#close.v1, core#reconcile.v1]
      requirements: {bound: {requirement: core#bound.v1, arguments: {pools: [core-work], operations: [fs#file.read.v1, fs#directory.list.v1, fs#file.write.v1, core#inspect.v1, core#set-up.v1]}}}
agents:
  coordinator:
    type: cc#claude-code.v2
    operator: {from: cc-turn}                         # root's turn
    acts-for: {conduit-core: {from: manage}}          # opened by root alone (ACC-045)
    consumes:
      queue: {to: core-queue, from: queue-use}
      w1:    {to: core-w1, from: cx-turn}             # w2 and w3 the same; standing, opened by root; inspection marks what each reaches
  core-w1:                                            # core-w2 and core-w3 the same: fixed slots, each with its own home, signed in once (ruling 49); none consumes a secret (B13)
    type: cx#codex.v2
    operator: {from: cx-turn}
    consumes:
      main:   {to: main, from: read}
      skills: {to: skill-library, from: read}
      queue:  {to: core-queue, from: queue-use}
```

packages.yaml
```yaml
project: packages
projects:
  packages:
    holds:
      work:   {to: pkg-work, from: run-pool}
      trees:  {to: access,   from: grant-pkg-trees}     # as grant-core-trees, with pools: [pkg-work]
      shared: {to: access,   from: grant-shared}        # read on skills and repo; nothing it holds names auth
pools:
  pkg-work: {type: core#pool.v1}
resources:
  tree-p1:   {type: fs#local-directory.v1, owner: pkg-work, config: {root: ~/wt/pkg-1}}
  pkg-queue: {type: hist#file-history.v1,  owner: orders,   config: {root: ~/orders/pkg}}
agents:
  pkg-w1:
    type: cx#codex.v2
    operator: {from: cx-turn}
    consumes: {main: {to: main, from: read}, skills: {to: skill-library, from: read}, queue: {to: pkg-queue, from: queue-use}}
```

pkg-console.yaml (v0: the owner's own console scoped to packages, with its own loopback token issued by the shell in batch 2)
```yaml
interfaces:
  pkg-console:
    acts-for: {packages: {from: manage}}       # authority over packages only; a second project is a second line, marked "bridges" in inspection
# v1 only: a second person managing packages (rulings 15 and 49)
#  pat:
#    acts-for: {packages: {from: manage}}
```

- Sizes on paper: engine.yaml about 33 lines, conduit-core.yaml about 37, packages.yaml about 17, pkg-console 3. That is about 90 lines once, all of it root's. Root writes nothing for its own authority.
- A78 in v0 costs 0 document lines per order and about 6 tool calls.

The coordinator's flow for order 031 (A78, phase 2: orders 4 and 5)
1. Root posts the order on core-queue and takes a turn on the coordinator.
2. `queue.read.v1`.
3. The coordinator finds a free slot.
   - It calls `conduit-core.list.v1`, then `conduit-core.inspect.v1 {object: core-w1}` through its E3 (row 6A).
   - The inventory shows core-w1's Conduits with their provenance: no tree grant is open, so core-w1 with tree-c1 is free.
   - `conduit-core.work.inspect.v1 {object: tree-c1}` (forwarded through E1 `work`) confirms the tree is ready.
4. `conduit-core.trees.make.v1 {from: read-write, consumer: {kind: Agent, name: core-w1}, target: {kind: Resource, name: tree-c1}, as: tree}`.
   - Two records: the act, and conduit-core's make.v1 serving it.
   - The new Conduit is named core-w1-tree-x7k2p, with provenance {actor: coordinator, openedThrough: [conduit-core's trees], actingFor: coordinator's E3, sentBy: conduit-core}.
5. If the order needs Linear: `conduit-core.mint.make.v1 {from: issues, consumer: {kind: Agent, name: core-w1}, target: {kind: Resource, name: linear}, as: linear}`.
6. `w1.turn.v1 {message: <order 031>}`. The worker works through `tree.*`, `main.*` and `skills.*`, and posts its report with `queue.post.v1`.
7. At merge (root reviews and merges), on root's next turn: `conduit-core.trees.close.v1 {conduit: core-w1-tree-x7k2p}` and `conduit-core.mint.close.v1`.
   - Narrowing never waits, nothing is withheld, and the next order's grant gets a new name.
   - If root does the cleanup itself, it closes without `--withhold`, and the slot stays usable.

What the coordinator is refused in v0, and why

| Request | Result | Why |
|---|---|---|
| a trees make for pkg-w1, or a mint make for any packages Agent | refused, tier 1: "the consumer is not within conduit-core" | ACC-047(a) |
| `conduit-core.shared.make.v1` read-write on main | refused, naming the bound | ACC-006 |
| any make through conduit-core for itself, for root or for pkg-console | refused | ACC-047(a) |
| make.v1 on its own behalf, of an act-for Conduit to packages (the ops-lead path) | no tool: it holds no Conduit to `access`; anyone else is refused | ACC-045 applies to make (ACC-048) |
| a make whose target is core-w2 (a turn), `access`, `management`, `system` or a Project | refused | ACC-047(b) and (c) |
| a make giving core-w1 `skills` with retire-with-resources, or `core-work` with register.v1 | refused | ACC-047(c), whatever the consumer |
| a make for core-w1 on tree-c1 after root withheld that pair | refused, tier 1, naming the withhold | ACC-049 |
| a set-up through `work` of a Resource with declared Conduits and no `partsThrough` | each part refused, naming the missing input | PRJ-008 |
| closing core-w1's `main` Conduit (root's direct grant) | refused, tier 1 | ACC-047(d): not sent by conduit-core |
| retiring, setting up or taking a turn on another actor | refused | root's alone (ACC-046) |
| appointing anyone | refused | ACC-045 |
| keeping, withholding or lifting | no tool | a Project never lists them (PRJ-006) |
| registering a worker or a tree | no tool | rows 1 and 1A need `management` |
| retiring a tree | no tool | `run-pool` and `manage` omit retire |
| anything in packages | refused at tier 1, at act | no act-for Conduit to packages |
| retiring skill-library | no tool | nothing conduit-core holds administers `skills` |

What pkg-console reaches in v0:
- It can:
  - set up, inspect and list packages' Agents (rows 4A, 6A, 16), and see each one's Conduits with their provenance;
  - administer pkg-work's trees through `packages.work.*`, except retiring them;
  - make and close, for pkg-w1 only, read-write on pkg-work and read on skills and repo.
- It cannot:
  - take any turn, or use any Resource, through packages (confinement);
  - reach auth;
  - register;
  - appoint;
  - withhold, keep or lift.
- Its token, like root's, sits in a store directory that any program of the owner's OS user can read, so this is administration mediated by Conduit, not separation from root.

Revocation, worked
- Root closes conduit-core's `trees` (the authority):
  - `granted-under` lists core-w1-tree-x7k2p with its actor and actingFor;
  - one pending action is raised at root, with no expiry, and the grant is marked;
  - root answers "closed" (`conduit close --listed`) or "kept" (`conduit keep`), and each answer is recorded;
  - nothing is withheld, so after root opens a fresh `trees`, the slot pairing core-w1 and tree-c1 can be granted again.
- Root revokes one grant while `trees` is still open, deliberately: `conduit close core-w1-tree-x7k2p --withhold`.
  - That closes the grant and records a withhold for (conduit-core, core-w1, tree-c1).
  - The coordinator's next make for that pair, through any Conduit conduit-core holds, reports refused, naming the withhold.
  - `conduit lift` (ruling 55 A7's operation) ends it.
- Root closes the coordinator's act-for Conduit:
  - every `conduit-core.*` tool goes at the next decision, and in-flight onward requests stop;
  - one pending action lists and marks every grant made acting under it;
  - the coordinator's standing turns on the workers are root's own grants, listed with their reach in conduit-core's inspection; root closes them if it means to stop the coordinator driving workers.
- Root retires packages with its Agents: `conduit retire packages --with-agents`.
  - The command opens root's cascade Conduit, calls retire-with-agents.v1 and closes the Conduit again (ACC-024 amended).
  - pkg-w1 and every packages actor retire, and their Conduits close.
  - pkg-console's act-for to packages closes at once.
  - packages' E1 and E2s close at its immediate shutdown.
  - skills, repo and orders are untouched, and tree-p1 remains in pkg-work.

## rulings
Each ruling is phrased for the owner, with options, a recommendation, the reason, and the owner's words or rulings it touches. They are ordered within each group. Group A must be ruled before the spec batch: ruling 55's short form, ruling 53's five additions and the Project structure, all before the S1 rerun. A5 is applied first in the batch, because the family cannot be added while the word is taken. Group B must be ruled before batch 2: project authority, which lands after the rerun and before A78's coordinator phase. Group C comes later, or is independent of Projects.

=== GROUP A: BEFORE THE SPEC BATCH (structure; before the S1 rerun) ===

A1. Are Agents owned by Projects exactly as Resources are owned by Pools, including registration?
- (a) Exact mirror (recommended).
  - `owner` is required and fixed for life.
  - Registering an Agent needs `management` AND its Project.
  - Setting up, retiring, inspecting and reconciling an Agent go through its Project.
  - Ruling 30's OL2 becomes "its owning Pool for a Resource, its owning Project for an Agent, `management` for everything else".
  - Rows 5, 7 and 14 lose Agents.
  - Not mirrored: a Project has no entries, and a bound that names a Project covers the Project alone, never its Agents.
- (b) The same, but registering an Agent needs only a Conduit to its Project.
- (c) `owner` optional; Agents without one stay with `management`.
- Why (a):
  - It is your "like resources are tied to pools" and ruling 54 point 1.
  - Against (b): whoever administers a Project would choose the program and `remote` URL an Agent runs as the engine's user. Counterexample: an administrator registers pkg-w9 with working directory ~, and root's later turn on it runs that configuration as root (IFC-022).
  - Against (c): it contradicts "each Agent owned by exactly one Project".
  - Covering Agents through their Project would make every right over a Project's administration a right to take turns on its Agents.
- Consequence, stated plainly: in v0 nobody who administers only a Project registers anything, so A78's workers are fixed slots (B12).
- Touches:
  - amends ruling 30 OL2, ruling 29 (six families) and ruling 8;
  - test 5, exception 4. The vectors are access/routing step 7 and setup/sender, sender-onward, sender-no-access and optional step 1, rewritten with a Conduit to the Project.

A2. What does retiring a Project do to its Agents?
- (a) The mirror of a Pool (recommended):
  - plain retire reports conflict while the Project owns an Agent that is not retired, or whose shutdown is not confirmed;
  - retire-with-agents retires each Agent, then the Project.
- (b) Plain retire retires its Agents.
- Why (a): it is the Pool rule (LIF-020, LIF-021) you asked to mirror, and a cascade is asked for by name (ruling 55 A4). The earlier round read the direction's "retiring a Project with its Agents retires them" as (b), so the reversal is put to you explicitly.
- Touches: the direction; LIF-020 and LIF-021.

A3. Root's standing Conduits: to what, listing what, named how, and how root uses a cascade?
- Targets (recommended): every Pool, `system` included, and every Project, each made by the engine within the target's registration. The one to `system` is a third first-start Conduit.
- Contents:
  - (a) every operation;
  - (b) every operation except the two cascades and act (recommended);
  - (c) inspect, list, set-up and reconcile only.
- Names (recommended):
  - generated names, with `as` equal to the target's name;
  - a Pool and a Project may not share a name;
  - `management`, `access` and `system` are reserved for both;
  - registering a Pool or Project whose name is already the `as` of one of root's Conduits reports invalid, naming that Conduit;
  - the defaults are exempt from tombstones.
- (d) How root uses a cascade, given that ACC-024 makes every Conduit whose consumer is root unclosable:
  - (d1) Amend ACC-024 so that root, on its own behalf, may close its own Conduits other than its first-start Conduits and defaults. `--with-agents` and `--with-resources` then open a cascade Conduit, call it and close it in one command, printed in the plan (recommended).
  - (d2) The defaults list the cascades, and the CLI shows them only behind a flag.
  - (d3) Leave it as it is.
- Why:
  - `system`: it is a Pool ("every pool"). Without it root cannot inspect `management`, `access`, nodes or providers.
  - (b): these Conduits are standing tools for whatever drives root, the model-driven admin session included. Counterexample: a report in the queue steers that session into `packages.retire-with-agents.v1`, one call with no flag. retire-with-resources on `system` would retire providers. Root never needs act, because its own `access` is unbounded.
  - Names: without reservation, a Project named `access` collides with root's first-start tools, and the registration would fail for a reason the author never sees.
  - (d1): it alone keeps (b) meaningful.
    - Under (d3), a cascade that fails partway (LIF-021 holds the Pool back while any shutdown is unconfirmed) leaves an unclosable standing cascade tool.
    - Under (d2), the MCP tool stands in front of the model-driven session again.
    - Closing one's own Conduit never locks root out: its unbounded `access` reopens.
- Touches:
  - your "root having conduits to all projects and pools by default" and "admin power over everything";
  - ruling 53 decision 6, answered by the engine;
  - FST-002, FST-003, FST-005 and ruling 30 OL5;
  - ACC-024 (test 5, exception 6, under (d1));
  - test 2.
- Vectors: access/bounds observation 16 (agent-a closing root's c611) stays refused. No vector has root close its own Conduit (checked).

A4. How is the engine discovered, and what does an administrator see?
- Listing:
  - (a) A new operation, list.v1 {kind?}, on `management`, every Pool and every Project, authorized against the container (recommended).
    - `management` lists every kind: Interfaces, Pools, Projects, templates, Conduits, packages, nodes, tombstones, withholds, and the instances of retired containers.
    - A Pool lists its Resources.
    - A Project lists its Agents, the Conduits it holds and its withholds.
    - Each shows only what the caller may inspect, with no counts.
  - (b) inspect.v1 with `object` made optional.
  - (c) No listing.
- Inventory:
  - (i) Ruling 53's inventory of an object shows each Conduit it consumes and is the target of, in full, wherever the caller may inspect that object (recommended).
  - (ii) Only Conduits the caller may inspect by name, which is `management` alone (row 7).
- Why:
  - (a) is an addition, which NAM-024 allows. (b) edits inspect.v1, which requires `object`.
  - (i) is ruling 52's tier 2 ("to a caller that may inspect the target ... which Conduits exist and what they list") applied to the inventory. Under (ii) no one but root sees any entry, so the coordinator and pkg-console never see the grants they made, and closing at merge depends on remembering the names that make returned.
  - (i) changes no ACC-033 row and flips no vector.
- Touches: ruling 53 decision 3 (an extension and a reading); your "discovery ... over everything"; ruling 52; ruling 55 A5's care for published contracts.

A5. What becomes of the existing word "project" (the documents and manifest handed over together)?
- (a) Fold it into "submission", which definitions.md already defines with nearly that meaning, and define a package as a set of documents with its manifest, published as an archive (recommended). Rust identifiers and step kinds such as register-project keep their names until a later sweep.
- (b) A new word, such as "module".
- Why (a):
  - Ruling 42 requires one meaning per word.
  - It is a merge, not a mechanical rename: about 55 promises are reworded.
  - It must come first because batch 1 introduces the family.
- Touches: ruling 42, and the wording of rulings 25, 27 and 37.

A6. How is a Project written in the short form?
- (a) A top-level `project: <name>` in a file makes the Project and is the owner of every Agent in that file (recommended).
  - An Agent in a file without it states `owner:`.
  - Batch 2 adds `projects: <P>: holds:` and `acts-for:` under an actor.
- (b) A `projects:` section, plus `owner:` on every Agent.
- Why (a):
  - S1 costs one line: a total of 34, against the baseline's 64 and the short form's 33; (b) gives 37.
  - The owner is written once, visibly, in the file.
- Risk inherited from the short form: T1 is about 36 against 33 at the fresh author's 1.7 times factor.
- Touches: ruling 55, its grammar extended; ruling 52's measurement plan.

A7. When a Resource is replaced under its name (ruling 55 A3), do bounds that name it follow?
- (a) No (recommended).
  - Bound items keep the ID they resolved to (OBJ-021, unchanged), and the plan of a replacement prints each bound that stops covering the Resource.
  - The guidance and examples bound on Pools, which cover the replacement (ACC-029).
- (b) Bound items that name an individual target are resolved again, by name, at each decision.
- (c) The replacement rewrites the bound items that held the old ID.
- Why (a):
  - It changes no promise.
  - Bounds on Pools are what your "auth pool for a specific project" says anyway: grant-linear names the Pool `auth`, and confinement keeps the key out.
  - (b) lets whatever later takes a name inherit every bound that named it. Ruling 53's tombstones make that deliberate, but a recreated name is not always the same thing.
  - (c) has the engine edit a Conduit's requirements, against "a filter is carried as its template states it, which no request changes" (ACC-017's spirit).
- Under (a), re-making an E2 that named the old Resource raises no review: the grants made under it closed when their target retired (CON-008).
- Touches: ruling 55 A3; OBJ-021; ACC-029; your "auth pool for a specific project".

=== GROUP B: BEFORE BATCH 2 (authority; after the rerun, before A78's coordinator phase) ===

B1. Does project authority ship in v0?
- (a) Yes, as batch 2 after the S1 rerun (recommended).
- (b) No: v0 Projects are ownership, routing, root's defaults, and ruling 54 points 1 and 5.
- Why (a): without it, several of your sentences are not met in v0: "give conduits to projects", "projects open admin conduits to pools", "agents can be given their own conduits" through a project, "interfaces over one or multiple projects", and "skills for all, auth for one". A78 does not need it (B12), so A78 is not the reason.
- Touches: your words above.

B2. What may a Project hold, and who gives it?
- (a) Two kinds of ordinary Conduit (recommended):
  - administration of a Pool other than `system`, never listing register, register-replacement or retire-with-resources;
  - a bounded right to grant on `access`. It must have a bound, never lists lift, keep or withhold, and its bound names none of the excluded operations, nor act or get-secret.
  - Root, or any Interface or Agent whose own right covers them, opens or makes them, under ACC-006 and ACC-030 as written.
- (b) The same, but root alone gives them.
- (c) Also use Conduits.
- Why (a):
  - It honours "the interface should still be able to give conduits to projects to manage them directly" for every Interface with the right, as often as it needs (B8).
  - Each exclusion has a counterexample:
    - `system` holds the machine and package providers;
    - a directory registered at ~/.ssh into a Pool a project's bound names is covered at once (ACC-029);
    - retire-with-resources: packages would retire `skills`;
    - an unbounded right makes the project unlimited.
  - Rejecting the bound's names at the language layer means a Project never holds a right it cannot use (M2).
  - (c) has a Project's actors directing work, the IFC-022 path, so it is v1.
- Touches: your words; the direction's "A Project can hold Conduits"; rulings 27 to 29.

B3. How does an actor use what a Project holds? (The mechanism; it changes what you agree to only through the rulings amended.)
- (a) Forwarding (recommended).
  - The actor asks the Project, through its own act-for Conduit, decided as every request is today.
  - The engine, as the Project, then makes one onward request, from within itself, through the held Conduit named. A setup's parts go through the E2 named in `partsThrough`.
  - That gives two records, linked.
- (b) Native: one request decided by two Conduits together, with one record.
- Why (a):
  - It amends no ruling.
  - Closing the act-for Conduit stops work in flight through the existing chain rules.
  - It is ruling 44's broker pattern.
  - The parts of a forwarded setup are decided by a Conduit the caller names, so provenance never depends on which Conduit was opened first.
  - Cost:
    - two records per action;
    - IFC-015 and IFC-022 gain a sentence for a sender inside the engine;
    - act gains one optional input.
- Touches: the direction's "Actors ... act through the Conduits it holds"; ruling 17 V3 (only (b) amends it); ruling 52.

B4. Who may act for a Project in v0, and for how many?
- (a) Recommended:
  - any Interface you appoint (your own scoped consoles and local hooks), each with its own loopback token, which batch 2's shell issues;
  - any Agent the Project owns.
  - An Interface may act for several Projects, as your words say. An Interface that does is marked "bridges: P, Q" in inspection, and the guidance gives a model-driven console one Project.
  - An Agent acts only for its own Project.
  - People other than you wait for v1.
- (b) Agents only.
- (c) As (a), but one Project per Interface in v0.
- (d) As (a), plus other people. This amends rulings 15 and 49, and is not recommended.
- Why (a):
  - "Interfaces could have authority over one or multiple projects" works in v0 as granular administration.
  - Through a Project, an Interface is granted nothing (B7). What it can carry between Projects is what it reads through root's direct grants. That is named and marked, not forbidden, so (c) would narrow your words for a risk the mark already shows.
  - An Agent acts only for its own Project. Counterexample: one coordinator acting for both projects reads a packages report carrying instructions, makes a Linear grant through conduit-core and relays it into a packages turn.
  - The shell issues client tokens only for root today (reference-model/crates/shell/src/main.rs:178, 234). Every token, root's included, is a file that any program of your OS user can read (lib.rs:280-299). The console's separation from root is therefore mediated, as spec/README.md will say.
- Touches: your "Interfaces could have authority..."; rulings 15 and 49; test 5, exception 3; test 8.

B5. Who appoints someone to act for a Project?
- (a) Opening or making a Conduit that lists act is the root Interface's alone, on its own behalf. Closing follows the ordinary rule, so a bound can grant or withhold it (recommended).
- (b) The Kubernetes `bind` rule: anyone whose bound covers everything the Project holds now.
- (c) Root alone, for opening and for closing.
- Why: acting for a Project reaches whatever it holds later.
  - Counterexample (H1 F3): ops-lead, bound to act for packages, makes itself an act-for Conduit. Root later gives packages a right over `auth`, and ops-lead gains auth.
  - The rule must say "or make", or make.v1 reopens this path (critic M1).
  - (b) fails as the Project grows.
  - Closing narrows.
- Touches: the other reading of "give conduits to projects"; test 5, exception 1.

B6. May anyone but root drive, set up or retire an Agent that acts for a Project?
- (a) No (recommended).
  - Opening or making any Conduit to such an Agent, and setting it up or retiring it, is root's alone.
  - Appointing an Agent that others already hold Conduits to is refused, naming them.
  - Closing an actor's own Conduits stays ordinary.
- (b) Allowed, and marked in inspection.
- Why:
  - Whoever drives an actor acts for its Project. Counterexample: core-claude, given turns on its colleagues, opens or makes a turn on the coordinator and has it make core-claude a Linear grant.
  - Retiring cannot be undone. Closing can, by root, so a delegate stopping an actor is the safe direction.
- Touches: test 5, exception 2; H1 F5.

B7. What may a grant made through a Project be?
- (a) Recommended:
  - It is made to the Project itself (administration of a Pool), to one of its Agents that acts for no Project, or, as a part of a setup the Project sends, to the instance being set up.
  - It targets a Pool other than `system`, or a Resource.
  - Whatever the consumer, it lists nothing that starts model work, no get-secret, no operation reserved to root, and no register, register-replacement or retire-with-resources.
  - A Project closes only grants it sent, reconciles only its own and its Agents' invocations, and never lifts, keeps or withholds.
- (b) Also turns on the Project's own non-actor Agents.
- (c) Use of Resources only.
- Why (a): it reads your "Agents can be given their own conduits to pools and resources" and "Projects ... open admin conduits to pools" literally. Counterexamples:
  - for root: `root -> skill-library [write]` made through a project can never be closed (ACC-024);
  - for an Interface or the actor itself: `pat -> pkg-tree [write]` outlives pat's removal;
  - for another project's Agent: conduit-core's actor mints for pkg-w1;
  - for register and the cascades with an Agent consumer: packages' coordinator gives pkg-w1 retire-with-resources on `skills` and has it retire the Pool (critic M2);
  - against (b): the actor would decide its own reach. It could give itself a turn on any non-actor Agent of P and wield every Conduit root ever gives that Agent, with no decision by root at the time. That is Kubernetes' "create a pod, use any ServiceAccount". The previous text said (a) prevents the token-printing attack. It does not, wherever root itself gives an actor standing turns (B12). That is what B13 rules on.
  - against (c): it narrows your words.
- Touches: your words above; the direction's "to grant its own Agents use"; your "projects determine an interface's permissions" (C9).

B8. How does anyone other than root create a Conduit?
- (a) A new operation of `access`, make.v1 (recommended).
  - It registers and opens one Conduit from a registered template, authorized by the bound, not by `management`, and the engine names it.
  - Every condition on opening applies to making: appointment, deputies, secrets, reserved operations, confinement and withholds.
  - On its own behalf, a non-root actor makes only for itself or for a Project.
  - Only root makes one whose consumer is root.
- (b) An optional `make` input on open.v1.
- (c) No creation: Projects open only what `management` registered.
- (d) As (a), but on its own behalf a non-root actor makes only for itself.
- Why:
  - Under (c) a Project grants once, because a closed Conduit is removed (H1 F1).
  - (b) changes a contract that requires `conduit`, and silently gives every existing right to open the power to create.
  - The "every condition" sentence keeps the rules written for opening from being sidestepped by making (critic M1, H1 F3 and F5).
  - (d) lets a non-root Interface give a Project each Conduit only once, which narrows your "give conduits to projects" (critic D3). Making for a Project is limited to an E1 or E2, under PRJ-006.
- Amends ruling 44 point 2 (D04) for Conduits only.
- Touches: ruling 44; ruling 55 A5 and A7; test 5.

B9. What triggers ruling 54's review, and how is it answered?
- (a) Recommended:
  - Closing any Conduit a grant was made through triggers it, however the Conduit closes. That covers every Conduit to `access` and every act-for Conduit.
  - The close lists the grants.
  - One pending action is raised at root, with expiry "none". This reads ruling 51 decision 5's generic shape for this kind.
  - Its answer for each grant is "closed" or "kept" (ruling 56 point 8).
  - Marks are computed, so grants that open later under a closed authority are marked too.
  - An unnamed setup part records every Conduit that covered it, and is marked only once all of them are closed.
  - "Close all listed" is one command with a named flag.
- (b) Only a Project's Conduits to `access`.
- (c) As (a), but closing an act-for Conduit only informs.
- (d) As (a), with an expiry whose lapse closes the grant.
- Why:
  - Ruling 54 point 3 names "who opened it and through which Conduit". (c) leaves a removed manager's grants unreviewed.
  - (d) is a cascade by timer, against point 2. An expiry that kept the grant would be the forgotten access the ruling prevents.
  - Covering every delegation ends forgotten delegated grants with one rule. This is an extension, named.
- Touches: ruling 54 points 2 and 3; ruling 51 decision 5; ruling 56 point 8; ruling 55 A4; your "revoked individually when a project loses their conduit".

B10. Who may keep a grant under a closed authority?
- (a) An operation of `access` that a bound can withhold (recommended). It is never used by the grant's own `actor`, nor by the holder of an authority it was granted under, and never through a Project. In v0 only root holds it unless root gives it.
- (b) Reserved to root.
- Why:
  - You said root "can" keep, and for tombstones you chose an operation a bound can withhold (ruling 55 A7).
  - The guard's counterexample: a delegate holding two rights to open keeps, through the second, every grant that root's close of the first listed.
  - (b) is a test-5 exception.
- Touches: ruling 54 point 4; ruling 55 A7; test 5.

B11. How does root make an individual revocation of a project's grant stick?
- (a) An explicit withhold (recommended).
  - `conduit close <grant> --withhold` closes the grant and sends withhold.v1, a separate operation of `access` that a bound can withhold.
  - The withhold refuses that Project making or opening the same consumer and target again, until it is lifted with ruling 55 A7's lift.
  - A Project never holds withhold or lift. Withholds are listed in the Project's inspection and through `management`.
- (b) Automatic: root's close of a grant sent by a Project, while its authority is open, binds. (This is the critic's fix.)
- (c) None: only the name is tombstoned. Root narrows or closes the Project's right to stop re-grants, and inspection marks "re-grantable".
- Why (a):
  - Names of made Conduits are fresh, so under (c) an actor undoes root's revocation at once.
  - Under (b), any routine close by root binds silently. Root's own cleanup at merge would block the fixed slot pairing core-w1/tree-c1 for every later order (critic M4).
  - (a) binds only when root says so, in the plan, with a named flag (ruling 55 A4, ruling 52's rule to be explicit). The lift is the operation you already chose, so no root-only lift is added.
- What you agree to: a withhold is state about Conduits held outside any Conduit. Rulings 27 to 29 never considered this, and your direction rejected a list on a Pool as a grant object in disguise. A withhold permits nothing, it only refuses, so it is not a grant object. It is like ruling 53's tombstones, by pair instead of by name.
- Touches: ruling 54 point 3; ruling 55 A7 (extended to withholds); rulings 27 to 29.

B12. How does A78 run in v0?
- (a) Recommended:
  - Fixed worker and tree slots, registered and set up once by root.
  - Orders 1 to 3 dispatched by root after batch 1.
  - Orders 4 and 5 by a coordinator Agent of conduit-core, appointed by root. It grants each order's tree and Linear through conduit-core's Conduits, and drives workers through standing turns that root opened.
  - Inspection shows what each turn reaches.
  - One queue per project.
  - The coordinator's `manage` and the project's `run-pool` omit retire.
- (b) A broker per project, registering a fresh worker and tree per order.
- (c) The coordinator holds `management` registration.
- Why:
  - A fresh Agent per order needs a sign-in per home (rulings 48, 49), and registration through a Project is v1.
  - A78's criterion needs no project authority.
  - (c) is equivalent to root.
  - (b) costs a package, and in M1 its broker crossed projects.
  - Leaving out retire keeps one call from removing a slot a project can never register again.
- Touches: ruling 48 (A78); ruling 44; ruling 49.

B13. May an actor drive an Agent that holds a secret? (new: critic M3)
- (a) No, by rule, plus reach marks for everything else (recommended).
  - SEC-008 gains: a get-secret Conduit is not opened or made to an Agent that the consumer of an open act-for Conduit can take turns on.
  - ACC-046 gains the reverse: a turn whose consumer is an actor is not opened or made to an Agent that consumes a get-secret Conduit.
  - Inspection and the plan show, one hop deep, every Conduit each non-root turn-holder reaches.
- (b) Marks only. The guidance puts secrets in Resources (services), never in Agents that actors drive.
- (c) Nothing beyond naming the limit.
- Why (a):
  - The engine hands a secret to its consumer's run (SEC-010), so whoever can take turns on that Agent can have it print the secret. That is the very attack the previous text used against B7(b), and a standing turn root opened carries it as well as one the actor made.
  - The rule:
    - costs no vector: no vector gives an Agent a get-secret Conduit (checked);
    - costs nothing in your setup, because Linear's key is consumed by the `linear` Resource, not by a worker;
    - binds root too, as SEC-009 does, so it is no test-5 exception.
  - No rule can cover everything else: a worker's read of `main` is reachable by design. So the marks show it, and spec/README.md names it.
- Touches: SEC-008 (ruling 41's secrets); the purpose's "with the credential never handed over"; B7; B12; IFC-022.

=== GROUP C: LATER, OR INDEPENDENT OF PROJECTS ===

C1. Only root opens any Conduit whose consumer is root, open.v1 included.
- (a) Yes, after a vector check (recommended).
- (b) Leave it as a known limit.
- Why: ACC-024 makes such a Conduit unclosable by anyone but root, and a delegate can open one today.

C2. Should a bound limit reconcile for every delegate (H1 F9)?
- (a) Yes, with C1 (recommended).
- (b) Leave it.

C3. Consumer-side consent: may outsiders push Conduits into a Project's Agents?
- (a) v1, as bound.v2 with a consumers item (recommended).
- (b) Now, for Projects only.
- Why: applied now, it flips the bounds vector.

C4. Peer engines (ruling 51): should the owner's Interface admitted with full control get the peer root's defaults, appointment, keep, withhold and pending actions?
- (a) The peer's root template names the admitted Interface for these (recommended).
- (b) v1.

C5. Shared Pools.
- (a) Guidance, every administrator shown, and a pending action when a sender other than root retires a Resource that another project's Agents hold Conduits to (recommended).
- (b) At most one non-root administrator.

C6. People other than the owner acting for Projects (v1). Gated on:
- runtimes that confine;
- DL13 credentials;
- others' sign-ins (ruling 49);
- approvals by others (ruling 55 A4);
- pending actions at their Interfaces.

C7. Registration through a Project in v1: (a) a template and argument cap, or (b) ruling 44's broker. Decide from A78's friction.

C8. Names per Project, with qualified references (v1).

C9. Should a Project set what an Interface may use, not only administer? v1, after C6.

C10. Records filtered by Project, and a `confines` contract field (v1).

C11. Root's tool count (about 70 on your setup): should standing defaults be presented on demand? Measure in the rerun first.

C12. Reach beyond one hop, and for turn-holders who are not actors (v1, with C10).
- A transitive reach report.
- Whether the secret rule of B13 extends to every non-root turn-holder, which would narrow today's delegation.

## staging
V0 VERSUS V1
- v0 batch 1, before the S1 rerun: the structure, with root's defaults, the ACC-024 amendment, list.v1, the inventory reading and provenance.
- v0 batch 2, after the rerun and before A78's coordinator phase: authority.
- v1: group C, chiefly:
  - people other than the owner acting for Projects;
  - registration through Projects;
  - use and turn Conduits held by Projects;
  - consumer-side consent;
  - per-project names, records filtering and transitive reach.

ORDER OF WORK
0. Prerequisites, owed whatever is decided:
   - ruling 53's five additions (about 14 to 19 promises);
   - ruling 55's short form (about 24 to 26), with A7's lift operation;
   - ruling 50's pending action as one mechanism at root, with ruling 56's "answer" (about 5; needed by batch 2);
   - ruling 52's refusal tiers (2 to 4).
1. A Rust spike under experiments/, extending N4's: translate S1 with `project: s1`, register it through the loader and core, and count lines.
2. Group A (A1 to A7) put to the owner. A5 is applied first.
3. Spec batch 1, written once with the short form and ruling 53's additions.
4. Work orders for batch 1, six:
   - (i) the rename in spec/, plus renaming the Rust `pub struct Project` in reference-model/crates/loader/src/project.rs before a Project kind can be added;
   - (ii) the loader: the kind, `owner`, the `project:` key, schema enums, reserved names and the name collisions with root's Conduits;
   - (iii) the core:
     - rows 1A to 16;
     - retire-with-agents;
     - defaults, root-system and the ACC-024 amendment;
     - list.v1 over every kind;
     - the inventory reading;
     - provenance (including a list for unnamed parts);
   - (iv) the shell: list views, root's new tools, and `--with-agents` and `--with-resources` (open, call, close);
   - (v) a conformance recorder that renumbers IDs and emits a reviewed list of every outcome change. No recorder exists, and vectors regenerated from the model's own output cannot catch its bugs;
   - (vi) rewriting the vectors.
5. A78 orders 1 to 3, with root dispatching on fixed slots. Then the fresh Codex author's S1 rerun.
6. Group B (B1 to B13) put to the owner.
7. Spec batch 2.
8. Work orders for batch 2, five or six:
   - the core:
     - act forwarding on requests.rs's onward path (lines 683 to 715), with `partsThrough`;
     - PRJ-008's internal sender;
     - confinement, with clause (c) for every consumer;
     - make.v1 with every opening condition;
     - appointment, deputies and B13;
     - the review, keep and withhold;
     - the reach marks;
   - the shell:
     - a client token for each Interface root registers (today only root's is issued: main.rs:178, 234);
     - expanded tools and notifications;
     - the pending-action panel;
     - `conduit keep`, `close --listed`, `close --withhold` and `lift`;
     - reach and bridge marks in the views;
   - the vectors.
9. A78 orders 4 and 5 through the coordinator. Each friction is logged as a finding of the first build.
10. Group C, each by its own ruling.

SPEC BATCH 1: structure, about 36 to 41 substantive promises on paper
- New, about 11:
  - PRJ-001 (the family and core#project);
  - PRJ-002 (`owner`, fixed);
  - PRJ-003 (a live owner);
  - PRJ-004 (a Project is never a caller; owning permits nothing);
  - PRJ-005 (root's defaults: names, reservations, collisions, tombstone exemption);
  - LIF-038 (retire conflict);
  - LIF-039 (retire-with-agents);
  - ACC-044 (provenance state);
  - one list.v1 promise over every kind;
  - one inventory-visibility promise, folded into ruling 53's inventory promise if that lands in the same batch;
  - the Pool/Project name rule.
- Changed, about 27:
  - objects: OBJ-002, OBJ-013, OBJ-020;
  - conduits and operations: CON-024, CON-027 (a Project as target), OPS-014, OPS-019, OPS-029;
  - access:
    - ACC-022;
    - ACC-024 (root closes its own, defaults and first-start Conduits apart);
    - ACC-028;
    - ACC-029 (a bound naming a Project covers the Project alone; the plan line of A7);
    - ACC-033 (rows 1A, 3, 4A, 5, 6A, 7, 8A, 14, 14A, 16, and its observable);
  - first start: FST-002, FST-003, FST-005;
  - lifecycle: LIF-001, LIF-020, LIF-021, LIF-024, LIF-026;
  - REV-010, CTR-004, PLC-006, TPL-001, TPL-002, TPL-004, IFC-014, REC-001.
- Wording: about 6 promises, plus definitions.md: family, Project, owner, default Conduit, provenance, submission.
- The rename: about 55 promises, 344 occurrences in spec/.
- Core package:
  - project.v1, and project-management.v1 without act.v1;
  - list.v1 on three capabilities; retire-with-agents.v1;
  - the templates project and root-system;
  - project-reference.v1;
  - the schemas: kind enums, a Project branch in instance.v1 and template.v1, and the Agent's required `owner`. That edits instance.v1 in place, as ruling 41 edited the 0.1.0 core package; say so.
- With the prerequisites: about 80 to 90 promises before the rerun.

SPEC BATCH 2: authority, about 40 to 46 substantive promises on paper
- New, about 17:
  - PRJ-006 (what a Project holds, including an E2's bound names);
  - PRJ-007 (who acts for one);
  - PRJ-008 (act forwarding, the internal sender, `partsThrough`);
  - PRJ-009 (no server and no credential for a Project);
  - OPS-035 (act's outcome and its key);
  - ACC-045 (appointment, open or make);
  - ACC-046 (deputies, open or make, B13's reverse clause);
  - ACC-047 (confinement);
  - ACC-048 (make.v1 and "every opening condition applies");
  - ACC-049 (withhold);
  - ACC-050 (the review);
  - ACC-051 (keep.v1);
  - IFC-048 (expanded tools and the 31-character cap);
  - one promise for the review marks;
  - one for the reach and bridge marks;
  - the lift's extension to withholds;
  - the made Conduit's name (OBJ-003 variant).
- Changed, about 23:
  - OBJ-013 and CON-027 (a Project as consumer);
  - ACC-005 (parts sent by a Project; provenance lists);
  - ACC-006 and ACC-030 (make.v1);
  - ACC-012;
  - ACC-032 (open or make);
  - ACC-033 (row 15, and the row-2 exception for make);
  - SEC-008 (open or make; B13) and SEC-009;
  - IFC-012, IFC-013, IFC-015, IFC-022, IFC-033, IFC-038;
  - OPS-023 (the confinement tier and order);
  - LIF-028 and OPS-014 (reconciling a make);
  - CON-031;
  - REC-001;
  - the README limits.
- Core package:
  - act.v1 (with `partsThrough`), added to project-management.v1;
  - make.v1, keep.v1 and withhold.v1, added to access-control.v1 (additive under NAM-024);
  - the templates manage and run-pool;
  - the `_meta` key granted-under.
  - Alternative: land the contracts of make, keep and withhold in batch 1, so that root-access's operation list in the first-start vectors changes once.

TOTALS
- About 76 to 87 substantive promises on paper, about 10 more than the previous count, to cover the critic's fixes. Read them as 100 to 130: estimates in this investigation have run 1.5 to 2 times low.
- Plus about 6 wording promises, about 55 rename promises, and 45 to 55 prerequisites.
- Reference model, about 4.5 to 8k lines:
  - core 2.7 to 4.6k;
  - loader 0.3 to 0.6k, plus the rename;
  - shell 1.0 to 1.8k, including tokens for non-root Interfaces;
  - conformance runner and recorder 0.4 to 0.8k.
- Work orders: 11 to 12, plus 4 to 6 for the prerequisites.

VECTORS THAT CHANGE (367 cases today; from step scans; nothing run)

Batch 1: same input, different outcome, all intended and named:
- conventions/access/routing step 7: permitted becomes refused (agent-a sets up agent-b through `management`).
- conventions/setup/sender, sender-onward, sender-no-access and optional, step 1: becomes refused.
  - Rewrite: the fixture gives agent-b a Conduit to s1 listing set-up.v1.
  - ACC-005 is unchanged for these senders, so their meaning is kept.
- language/first-start/protected steps 2 to 4: refused becomes invalid (FST-006).
  - Rewrite the refused steps with a non-root Interface holding only `management`.
  - Correct conformance/README.md.
- conventions/access/fixed-kind-references step 2: refused becomes invalid.
- The first-start state, in six steps (initial 0, repeat 0 and 1, lifecycle 0, protected 0, s1/whole 0):
  - 7 objects become 8, and 14 records become 16;
  - root-management's operations gain list.v1, and root-access's gain lift.
- Unchanged, checked:
  - s1-pool-authority keeps its four outcomes, and access/bounds all 17;
  - bounds observation 16 (agent-a closing root's c611) stays refused under the amended ACC-024;
  - routing step 9 stays invalid.
  - The scan found no vector in which root closes a Conduit whose consumer is root.

Batch 1: inputs and expectations to rewrite (not a mechanical re-record):
- The 35 steps that set up an Agent through `management.set-up.v1` (agent-a 23, agent-b 10, agent-c 1, a 1), in 17 to 18 case directories, become `s1.set-up.v1`. The shell tests at reference-model/crates/shell/src/main.rs:377-379 change too.
- root-workspace leaves the S1 fixture: 30 `access.open.v1 root-workspace` steps are deleted, and 53 `root-workspace.*` calls become `workspace.*`.
- Each S1-fixture registration step's exact `records` list gains the events of the two defaults, beyond the ID shifts.
- Conduits opened within steps gain provenance fields wherever a case compares their state exactly.
  - About 48 case directories compare Conduit state, most of them already in this set.
  - Snapshots carry no provenance, so cases resting on root-system-state.json alone are not re-recorded for it.
- `owner` becomes required: about 48 cases outside the S1 fixture gain a Project document.
  - About 21 to 25 would otherwise go from accepted to invalid.
  - About 10 or 11 compare exact diagnostic arrays.
  - About 9 would reject for the wrong reason.
  - Also affected: changes/t4 step 2 and t5 step 5, 7 snapshots, spec/schemas/examples/valid/project/instances.yaml, and reference-model/examples/s1.
- IDs shift in the 118 S1-fixture cases (114 hard-code IDs).
- Snapshot rule, recommended: a snapshot need not be what first start makes, so the 197 cases on root-system-state.json are not re-recorded for `system`.
- In all: about 170 to 210 distinct case files, plus 28 to 38 new cases:
  - ownership and owner rejection;
  - the mirrored rows;
  - retire conflict and retire-with-agents;
  - defaults, reservations and collisions;
  - root closing its own cascade Conduit, and refused on a default;
  - list.v1 over each kind;
  - the inventory reading;
  - provenance, including the list for an unnamed part.

Batch 2:
- No existing outcome should change. The provisos:
  - make.v1, keep.v1 and withhold.v1 are new operations;
  - granted-under rides in `_meta` only when not empty;
  - the expansion keys on core#act.v1.
- Checked: B9's claim rests on no vector closing or retiring a Conduit to `access` while grants made through it are open. None does.
  - The only closes in the vectors are root's closes of declared Conduits, in setup/close, close-in-flight, restore, restore-retired-target, changes/t4 and s1/whole. Those Conduits are grants, not authorities.
  - The rest are agent-a's closes of c611 through c610 (bounds 15 and 16), in which c610 stays open.
  - Root's own `access` never closes.
- Checked: B13 rests on no vector giving an Agent a get-secret Conduit. None does: get-secret appears only in access/collision-secret and interface/schemas/e05-secret-omitted.
- root-access's exact operation list in the six first-start steps changes again, for make, keep and withhold, unless their contracts land in batch 1.
- New cases, about 50 to 65:
  - act through each kind of held Conduit, with records at two levels;
  - act refused at tier 1;
  - a forwarded setup with and without `partsThrough`;
  - make.v1 refused for each opening rule: act-for (the H1 F3 path), a turn on an actor (F5), get-secret, a reserved operation, root as consumer, a withheld pair;
  - make.v1 for a Project by a non-root Interface, more than once;
  - generated names at the 63-character limit;
  - each confinement clause, including (c) with an Agent consumer and the absence of an existence oracle;
  - appointment by root only, and each deputy rule;
  - B13 in both directions;
  - withhold and its lift;
  - the review: close by request, by retirement, and of an act-for Conduit; a computed mark for an open made in flight or reconciled later; an unnamed part covered by two Conduits;
  - keep and its guard;
  - expanded tool names and the cap;
  - reach and bridge marks;
  - A78's order flow as one scenario.

THE S1 RERUN
- It runs after batch 1 is built and before batch 2, as ruling 55's "What follows" and ROADMAP row 3 order.
- The author writes `project: s1` and meets one new concept. The paper count is 34 against the baseline's 64.
- It also measures:
  - root's tool count (about 37 on S1, against 23 today);
  - break-before-make windows at T5 and T6;
  - whether T1 holds under 33 at the fresh author's factor.
- The store makes a clean break: re-register from documents (`conduit init` again). Back-filling a required owner would be invented data, and snapshots carry no provenance.

## risks
1. Projects isolate nothing.
- Every Agent on the engine's own runner, and every client of the owner's (pkg-console included), runs as the owner's OS user.
- Every client token sits in a store directory that any program of that user can read, so any of them can present as root (IFC-022, DL16; reference-model/crates/shell/src/lib.rs:280-299).
- Appointment, deputy, secret and confinement rules stop mistakes and injection travelling through Conduit, not compromise.
- spec/README.md and inspection must say so, so that the owner never reads "administrative" as "separated".

2. Data channels carry instructions between projects. Shared writable Resources, Linear issues, skills and a shared queue all do.
- The recommendation is one queue per project, and root alone writing Pools that several projects read.
- Ruling 52's tiers do not cover content. One poisoned report read by a coordinator is a turn in all but name.

3. Turn reach (critic M3, partly closed).
- An actor with a standing turn on an Agent wields every Conduit that Agent holds, now and later, root's direct grants included.
- B13(a) rules out secrets. Everything else is only shown, and only one hop deep: the coordinator driving core-w1, which drives another Agent, is not marked until C12.
- If B13(b) or (c) is chosen, the token-printing path stays open wherever root gives a driven Agent a secret.

4. Act is all or nothing over what a Project holds, now and later.
- Anyone who gives the Project a Conduit, by open or by make, widens every actor of it, any delegate within its bound included.
- Provenance shows who added each Conduit, but nothing asks root.

5. An Interface that acts for two Projects carries what it reads between them (B4(a)).
- It is marked in inspection, not prevented.
- A model-driven console given two Projects is the bridge B4 rejects for Agents.

6. Grants made to an Agent before root appoints it are not self-grants under confinement. Closing its act-for Conduit does not review them; inspection shows them.

7. The estimates run low.
- Read the plan as about 100 to 130 substantive promises across both batches, about 4.5 to 8k lines of Rust, and 11 to 12 work orders, plus prerequisites.
- ACC-033 alone is about 100 lines of dense text.

8. Vectors.
- No recorder exists. About 170 to 210 cases change, with renumbered IDs, plus about 90 rewritten inputs and new exact `records` and provenance in registration steps.
- A recorder that blesses the model's own output would hide its bugs, so each change needs a reviewed list.
- The flip lists come from step scans, not runs.

9. The author's bar. At the fresh author's 1.7 times factor, T1 is about 36 against 33. Only a run (ruling 45) settles it.

10. Root's tools and destructive verbs.
- About 37 tools on S1 and about 70 on the owner's setup, heavy for a model-driven admin session (C11).
- Plain retire of each object stays a standing tool, with no flag at the MCP layer.
- Under the ACC-024 amendment, an injected session could close root's own non-default Conduits. Root reopens them through unbounded `access` (with a tombstone lift), so this costs availability, not authority.

11. Two records per project action. Record volume for project actions doubles, and a refusal comes at two levels. To be measured.

12. The rename is wide.
- "submission" merges two terms and rewords about 55 promises.
- Code: 378 to 517 occurrences in Rust, and about 1,900 in the conformance harness.
- The Rust `Project` type and the runner's step kinds (register-project, 212 steps by this scan) collide with the new family's name until renamed.

13. In-place edits to core contracts: a required `owner` on instance.v1 and new kind enums. Ruling 41 set the precedent, but ruling 55 A5 shows the owner cares, so the batch must say so.

14. Peer engines (ruling 51). Until C4, an owner's Interface admitted to a peer loses standing control of the peer's Agents under the amended OL2.

15. Shared Pools.
- Any non-root administrator of a Pool can retire a Resource that other projects use. CON-008 closes their Conduits (C5).
- A project that sets up a Resource of a shared Pool gives its parts that project's provenance, so the review of that project's E2 touches every project.
- The guidance has root administer shared Pools.

16. Ruling 15 holds by policy, not by the engine. The mechanism that lets the owner's consoles act for projects is the one a second person would use. v0 cannot check who holds a token.

17. State outside Conduits.
- Tombstones (ruling 53) and now withholds (B11) are state about Conduits held outside any Conduit.
- Each refuses and never permits, but the owner has not yet ruled on that class of state, and B11 puts it.
- Per-order grants, withholds and tombstones accumulate in the store.

18. Bounds by ID (A7(a)). A bound that names a single Resource stops covering it after a stable-name replacement. The plan prints it, but an author who ignores the plan finds a project silently without access. The examples bound on Pools.

19. Unverified, and what was checked.
- Nothing was run.
- Not checked:
  - the exact count of exact-array diagnostics;
  - whether the shell's private_directory permissions differ by platform.
- Ruling 53's tombstones are not yet in spec/, so their fields (consumer, target, provenance) are defined here, not verified.
- Checked by reading or scanning, read-only, at a98ffe0:
  - IFC-048, ACC-044 to ACC-052, OPS-035 and LIF-038 to LIF-039 are free;
  - the only removed IFC IDs are IFC-024 and IFC-035;
  - IFC-020 scopes an onward idempotency key to sender, caller and target;
  - bound items are stored as {kind, id} (spec/core/requirements.yaml:53-55, OBJ-021);
  - OBJ-003 and DL15 shorten generated names to fit OBJ-001's 63 characters;
  - pool-management.v1 lists register.v1, register-replacement.v1 and retire-with-resources.v1;
  - SEC-010 hands a secret to its consumer's run;
  - ruling 51 decision 5 gives a pending action an expiry;
  - ruling 30 OL5 makes root's two Conduits permanent;
  - the shell issues client tokens only for root (main.rs:178, 234);
  - no vector has root close its own Conduit, closes a Conduit to `access` while grants made through it are open, or gives an Agent a get-secret Conduit.