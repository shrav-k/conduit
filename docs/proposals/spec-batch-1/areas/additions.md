# Spec batch 1, area: additions

Not binding. An area planner's draft for [the batch 1 plan](../README.md); where the plan differs, the plan governs.

## Scope

Ruling 53's five core additions and ruling 55 A7's lift, plus ruling 62 C1 and C2 and ruling 66 point 4. All paths below are relative to the repository root (read-only; nothing was modified).

The five additions are:
- `as`, a local name for a standalone Conduit;
- `expectedId` on open.v1 and close.v1;
- an inventory in inspect.v1 that gives no count of what it hides;
- tombstones that the engine enforces at open and register;
- tool descriptions that name the target and its Type.

They are written scope-ready: each tombstone key carries an optional `within` field that, when absent, means the engine. Visibility of the inventory follows the inspection rows of ACC-033, which the Fleet area extends, so Fleets need no rewrite. The lift is the new operation `lift.v1` of `access`, which a bound can withhold.

The ruling 62 and 66 points:
- C1: only root opens a Conduit whose consumer is root.
- C2: a bound limits reconcile through `access`.
- Ruling 66 point 4: a holder other than root reconciles through a Pool or a Fleet only the invocations it, its Fleet or its Fleet's Agents sent.

Grounding: rulings 53, 55, 59, 60, 61 (read only so as not to foreclose it), 62 and 66; docs/proposals/simplification/ (README, [research/synthesis.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/synthesis.md), [research/critic.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/critic.md)); docs/proposals/language-shape/README.md and short-form/synthesis.md (F2, F3, decision 5); docs/proposals/projects/brief.md (sections 4, 9, 10, 15, staging, group C); experiments/011-short-form-spike/results-2026-11-10.md; and the current text of every promise named below.

## Promise and core changes

PROMISE IDs. The Fleet brief has already taken LIF-038/039 (retire conflict, retire-with-agents), ACC-044 to ACC-052 and IFC-048. This area takes LIF-040, LIF-041 and ACC-053 to ACC-055. The batch editor makes the final numbering. The engine-wide names in spec/names.yaml are unchanged: refusals reuse the existing `io.github.shrav-k.conduit/restriction` key (IFC-026).

A. NEW PROMISES (drafts)

LIF-040, tombstones made (convention, must). Proposed file: spec/conventions/lifecycle.yaml.
- When: a Conduit is closed, whether by request or under CON-008; or an instance that is not a Conduit is retired.
- Then: the engine makes a tombstone and keeps it as its own state, apart from the records, until it is lifted under ACC-053.
  - A Conduit's tombstone holds its consumer and target, each by kind and name, its local name, its capability and its operations.
  - Another instance's tombstone holds its kind and name, and its owner where it had one.
  - Every tombstone also holds: the ID of the closed or retired object; the time; an ID the engine gives the tombstone; and its cause. The cause is the invocation of the request that closed or retired the object. For a Conduit closed because an end was retired or declared lost, the cause is that end's tombstone, or the cause that PLC-024 states.
  - A tombstone holds no `within`: its names are names of the engine (OBJ-002), and it applies to every request the engine decides.
- Observable:
  - A Conduit the engine makes without a request leaves no tombstone. That covers root's first-start Conduits and the defaults made with each Pool and Fleet (ruling 59 A3).
  - No purge removes a tombstone; only a lift does.
  - The keys are names, not IDs, so a consumer or target replaced under the same name still matches. This closes critic gap 2 under ruling 55 A3.
  - The inventory of either end shows a Conduit's tombstone (LIF-013). `management`'s list.v1 shows every tombstone (Fleet area).
  - The caller of the request that made a tombstone is shown only to root and to that caller.
  - A tombstone is not a record and gives no count of anything.

LIF-041, tombstones enforced (convention, must).
- When, either of these:
  - A request would instantiate a Conduit (as a document of a registration, as a nested instantiation of an instance being registered, or under LIF-009), or would open one (open.v1, or a setup part under ACC-005). A tombstone holds the same consumer and either the same local name, or the same target and capability.
  - A request would instantiate an instance whose kind and name a tombstone holds.
- Then: the request is not dispatched and reports conflict, as OBJ-025 does for a taken name. Its error diagnostic names the tombstone's ID, what it holds and its cause, and says that lift.v1 lifts it. Nothing changes.
- Observable:
  - The check comes after the access decision and after every check that gives invalid (OPS-023). A caller that is not permitted reports refused and learns nothing.
  - Giving a Conduit another name or local name does not escape it: the target-and-capability key still matches.
  - An open Conduit is not refused, because reopening changes nothing.
  - A Conduit the engine makes without a request is never refused.
  - register-replacement.v1 does not lift a tombstone.
  - The engine enforces this against every client, Agents included. That keeps preservation test 5.

ACC-053, lifting (convention, must; ruling 55 A7).
- When: an actor asks to lift a tombstone.
- Then: the request is the operation lift.v1 of `access`, and it names the tombstone by its ID. It is dispatched only if the actor consumes an open Conduit to `access` that lists lift.v1, and whose bound covers what the tombstone holds:
  - for a Conduit's tombstone: its target and every operation it listed (as ACC-012 requires for closing it), or ACC-030's within-bound rule where that Conduit was one to `access`;
  - for another instance's tombstone: the instance as a target under ACC-029, with a Resource also covered through its owning Pool.
  - A tombstone of a Conduit whose consumer is root is lifted only at root's own request.
  - Lifting removes the tombstone and every tombstone whose cause it is (the CON-008 cascade).
- Observable:
  - An actor whose Conduit to `access` does not list lift.v1 reports refused, whatever its bound.
  - ACC-030 passes lift.v1 on only to an actor whose own Conduit lists it. So a bound can withhold the lift (A7, preservation test 5).
  - An ID that names no tombstone reports invalid (OPS-029) for an unbounded holder. A bounded holder reports refused and learns nothing.
  - A lift registers nothing and opens nothing, and names no revision (REV-012). It is recorded with its input.
  - It is reconciled through `access` (row 13, ACC-055).

ACC-054, ruling 62 C1 (convention, must; in SEC-008's form).
- When: a request is made to open a Conduit whose consumer is the root Interface.
- Then: it is dispatched only where root sent it on its own behalf. Otherwise it is not dispatched and reports refused.
- Observable:
  - A delegate reports refused whatever its bound. So does an onward request that serves root.
  - The Conduit stays registered.
  - With ACC-024 (the closing side, as ruling 59 A3 amends it, written by the Fleet area), every Conduit whose consumer is root is opened and closed at root's request alone.
  - Registration is unchanged (row 2).
  - The refusal names the restriction (IFC-026, ruling 52 tier 1).
  - It is phrased so that batch 2's B8 only adds "or make".

ACC-055, ruling 62 C2 and ruling 66 point 4 (convention, must).
- When: an actor other than root asks to reconcile an invocation (OPS-014) through a Conduit to `access` that carries a bound, or through a Conduit to a Pool or a Fleet.
- Then:
  - Through `access`: the request is dispatched only if the bound covers what the reconciled request needed it to cover. That is the target and every operation of the Conduit it opened or closed (ACC-006, ACC-012), ACC-030's rule where that Conduit is one to `access`, or the tombstone it lifted (ACC-053).
  - Through a Pool or a Fleet: the request is dispatched only if the invocation was sent by the actor, or, through a Fleet, by that Fleet or by an Agent it owns.
  - Otherwise it reports refused.
- Observable:
  - A hook's invocation counts as sent by the instance that sent the setup or retirement within which the hook was dispatched.
  - The sender is what OPS-008 keeps.
  - The check is part of the access decision. A refused reconciliation therefore says nothing of the invocation's outcome: refused comes before OPS-030's conflict.
  - Root, an unbounded Conduit to `access`, and reconciliation through `management` are not limited.
  - The Fleet-as-sender clause stays inert until batch 2's forwarding.

B. AMENDED PROMISES

spec/promises/objects.yaml:
- OBJ-013:
  - A Conduit's standard arguments become `consumer`, `target` and an optional `as`: its local name, a name under OBJ-022.
  - `as` written under `arguments`, or on another family, is rejected with a diagnostic naming the key.
  - A declared Conduit's local name stays its key; `as` is for a Conduit instantiated on its own.
- OBJ-002: the observable notes that a name held by no object may still be held by a tombstone (LIF-041: conflict).

spec/interface/implementation.yaml:
- IFC-012, last sentence: a Conduit instantiated on its own has its `as` as its local name, or its own name where it gives none.
- IFC-014 (F3 of the short-form synthesis):
  - A Conduit is not instantiated where its local name equals that of another Conduit of the same consumer that is not closed, declared or instantiated on its own, whatever tools either has.
  - The tool-name clause stays (batch 2's expanded tools still need it).
  - The carve-out "where the two would have a tool name in common" becomes unconditional.
  - The diagnostic names the local name and the other Conduit.
  - Ruling 59 A3's collisions with root's `as` rest on this rule.
- IFC-033: the tool `description` becomes, separated by blank lines:
  1. the contract's `metadata.description`, where one is stated;
  2. `Target: <Kind> <name>, of Type <qualified reference of the Type the target holds>.`
  3. that Type's `metadata.description`, where one is stated.
  - It discloses only the kind, name, Type and Type description of the consumer's own target: no config, owner or placement.
- IFC-038: checked and unchanged. An instance's name and Type are fixed, so a description changes only when a Conduit opens or closes, and IFC-038 already notifies that.

spec/conventions/lifecycle.yaml:
- LIF-037: `when` adds open.v1 and close.v1, whose reference is the input `conduit`.
- LIF-013: an instance's inspection gains a fourth part, `inventory`:
  - the Conduits it consumes and the Conduits whose target it is, each in full: the Conduit as object.v1, consumer, target, local name, capability, operations and state;
  - the tombstones of Conduits at either end;
  - no count of anything.
  - It is shown to any caller permitted to inspect the instance under ACC-033 (ruling 59 A4's reading of ruling 53's "listing only what the caller may inspect").
  - It is absent for templates and for Conduits.
- LIF-009: a register-replacement that a tombstone matches reports conflict (LIF-041), and it does not lift. This drops the short-form synthesis's F2 acknowledgement, which A7 rules out.
- LIF-035: a name can be given to another object once no tombstone holds it.
- LIF-028: cross-reference to ACC-055.

spec/conventions/access.yaml:
- ACC-006: cross-references to ACC-054 (consumer is root) and LIF-041.
- ACC-005: a part that a tombstone refuses reports conflict, and OPS-022 then applies.
- ACC-012: closing leaves a tombstone (LIF-040). ACC-024 itself is the Fleet area's.
- ACC-022: the observable says root's Conduit to `access` lists lift.v1, so root can lift any tombstone.
- ACC-033, co-edited with the Fleet area:
  - `when` adds "to lift a tombstone";
  - row 10 becomes "opening or closing a Conduit, or lifting a tombstone: `access` (ACC-006, ACC-012, ACC-053)";
  - an observable sentence: an object's inventory is part of its inspection, so no row is needed for the Conduits listed there;
  - rows 13 and 14 (and the Fleet area's 14A) gain "for an actor other than root, ACC-055 states a further condition".

spec/promises/conduits.yaml:
- CON-008: every closing leaves a tombstone except for Conduits the engine makes without a request.

spec/conventions/records.yaml:
- REC-004: a purge does not touch tombstones.

spec/conventions/operations.yaml:
- OPS-008: the kept state names "the instance that sent its request" explicitly. The reference model already stores `sender`, hooks included.
- OPS-014: cross-reference to ACC-055.

About 20 promises are amended, about 13 of them in substance, plus the 5 new ones. This is within the 14 to 19 that the short-form synthesis estimated for the additions with F2 to F4, plus C1, C2 and point 4.

C. CORE PACKAGE AND SCHEMAS

spec/core/operations.yaml:
- open.v1 and close.v1 gain the optional `expectedId: core#id.v1`, an addition NAM-024 allows. Their descriptions add the root-consumer rule, tombstone conflicts and tombstones made.
- inspect.v1 gains the optional output `inventory: core#inventory.v1`.
- reconcile.v1's description gains the bound and sender limits.
- New lift.v1:
  - input `{id: core#id.v1}`, required. The input name is deliberately neutral, so that batch 2's withholds can be lifted without a lift.v2.
  - effect: removes the tombstone and the tombstones it caused;
  - `deduplicates: true`; engine-implemented.
- The header notes are updated: the expectedId list, lift as an operation of `access`, and no revision for lift.

spec/core/capabilities.yaml: access-control.v1 lists lift.v1, and its description changes.

spec/core/templates.yaml: root-access lists lift.v1.

spec/core/results.yaml:
- New inventory.v1: `{consumes: [...], targetOf: [...], tombstones: [tombstone.v1]}`. Each Conduit item has `conduit` (object.v1), `consumer`, `target`, `localName`, `capability`, `operations` and `state`. It is extensible, so ruling 54's provenance and review marks can be added.
- New tombstone.v1:
  - `id`, `object` (object.v1), `cause` `{invocation?, tombstone?}`, `time`;
  - for a Conduit, also `consumer`, `target`, `localName`, `capability`, `operations`;
  - for an instance, `owner?`;
  - `by?`, shown only to root and the closer;
  - `within?`, absent meaning the engine.

spec/core/conduit.yaml: exports lift.v1, inventory.v1 and tombstone.v1; `engine` implements lift.v1.

spec/schemas/instances.yaml: the Conduit instance gains `as: {$ref: core#name.v1}`, the field experiment 011 found rejected. Add examples:
- valid: a project with `as`;
- invalid: `as` under arguments, `as` uppercase, `as` on a Resource.
Update the loader-verdict fixture that the tests record.

Core version: in-place edit of 0.1.0 (as ruling 41 did) or a new release. This is one decision for the whole batch, shared with the Fleet area.

D. spec/definitions.md
- "local name": amended to include `as`.
- New "tombstone", "lift" and "inventory".
- "ID": expectedId now also on open.v1 and close.v1.
- "the instance that sent a request": now also limits reconciliation through a Pool or a Fleet (ACC-055).
- The key is `within`, not "scope", because ruling 66 uses "scope" for an Interface's managed set and ruling 42 wants one meaning per word. Prose avoids "grant" (definitions.md: not used for access).

E. spec/README.md limits: tombstones accumulate until lifted. Every stable-name replacement (ruling 55 A3) needs a lift by an actor whose Conduit to `access` lists lift.v1.

F. Not written here, but relied on: ACC-024's amendment, defaults, object.v1's Fleet kind, list.v1 listing tombstones, and the Fleet inspect and reconcile rows (Fleet area); provenance in tombstones and inventory (ruling 54 area); refusal tiers (ruling 52 area); the short form's translation to `as` and NAM-024's no-default rule (short-form area).

## Vectors

NEW CASES. All are under conformance/, about 34 in all.

`as`
1. language/registration/conduit-as: a standalone Conduit with `as: project` registers, and agent-a's list-tools shows `project.*` (OBJ-013, IFC-012).
2. language/registration/conduit-as-invalid: `as` under `arguments`, uppercase, 64 characters, or on a Resource. Each is rejected with a diagnostic naming the key.
3. conventions/access/as-unique: a second Conduit of agent-a with `as: project` and another capability, so no tool in common, is invalid under IFC-014. The diagnostic names the local name and the other Conduit.
4. conventions/access/as-declared: a standalone Conduit whose `as` equals a declared Conduit's key, with no tool in common, is invalid.
5. conventions/access/as-after-close: after root closes `project`, IFC-014 no longer counts it, but the tombstone gives conflict (LIF-041). After a lift, the new Conduit registers and opens, and the tools are `project.*` again.

expectedId
6. conventions/records/expected-id-open:
   - a matching expectedId opens the Conduit;
   - a mismatch (a decoy registered under the name, or the name reused after a close and lift) gives conflict naming both IDs, and nothing opens.
7. conventions/records/expected-id-close: a mismatch gives conflict and the Conduit stays open; a match closes it.
8. conventions/records/expected-id-access-order:
   - a delegate whose bound does not cover, with a wrong expectedId, gets refused with no ID shown;
   - root naming a removed Conduit with expectedId gets invalid (OPS-029).

Inventory
9. conventions/records/inventory: root inspects agent-a. `consumes` lists project and history in full; `targetOf` and `tombstones` are as expected.
10. conventions/records/inventory-pool-holder:
    - a non-root holder of a Pool Conduit listing inspect.v1 inspects `shared` and sees both Agents' Conduits in full;
    - no count member appears anywhere;
    - inspecting agent-a is refused and says nothing.
11. conventions/records/inventory-tombstones: after root closes agent-a's project Conduit, the inventories of agent-a and of shared both list the tombstone with its ID and key. `by` is shown to root and absent for the Pool holder.
12. conventions/records/inventory-template: inspecting a template, or a Conduit, gives no `inventory`.

Tombstones
13. conventions/access/tombstone-register-replacement: root closes a declared Conduit. register-replacement then gives conflict; capture `structuredContent.diagnostics[0].properties.tombstone`. Root lifts. register-replacement succeeds, and set-up opens the Conduit.
14. conventions/access/tombstone-second-key: after X (agent-a, `as` p, shared, directory.v1) closes:
    - registering Y (agent-a, `as` q, shared, directory.v1) gives conflict;
    - a twin registered before the close gives conflict at open.
15. conventions/access/tombstone-instance:
    - a retired and removed Resource's name, registered again, gives conflict;
    - the lift lets it register;
    - the Conduits that the retirement closed are lifted with it, because their cause is its tombstone.
16. conventions/access/tombstone-replaced-consumer: agent-a is retired and re-registered under its name. Its Conduits give conflict until the retirement's tombstone is lifted (critic gap 2).
17. conventions/records/tombstone-purge: after root's purge, the tombstone still blocks (REC-004).
18. conventions/access/tombstone-order:
    - a registrant that is not permitted gets refused and learns nothing;
    - a permitted registrant whose document is also invalid gets invalid;
    - otherwise conflict.
19. conventions/setup/tombstone-part: a declared Conduit registered before a matching tombstone. Its setup part gives conflict, and the setup does not succeed (CON-004, OPS-022).

Lift
20. conventions/access/lift-root: root lifts; the request is recorded with its input.
21. conventions/access/lift-bound:
    - a delegate whose bound covers the target and operations lifts successfully;
    - a bound that does not cover gives refused, naming the bound;
    - a Conduit to `access` that does not list lift.v1 gives refused.
22. conventions/access/lift-unknown: an ID naming no tombstone gives invalid for an unbounded holder and refused for a bounded one.
23. conventions/access/lift-root-consumer: a tombstone of a Conduit whose consumer is root is refused to a delegate and succeeds for root.
24. conventions/access/lift-pass-on: a delegate whose Conduit to `access` lacks lift.v1 cannot open one that lists it (ACC-030). This shows that a bound withholds the lift.

Descriptions
25. interface/schemas/e05-description-target: a Type with a description gives the contract description, then the target line, then the Type description.
26. interface/schemas/e05-description-bare: no contract description and no Type description gives the target line alone.

C1
27. conventions/access/root-consumer-open: a delegate whose bound covers the target and operations opens a Conduit whose consumer is root and gets refused (tier 1, restriction ACC-054). Root's own open succeeds.
28. conventions/access/root-consumer-onward: an onward request that serves root is refused.

C2
29. conventions/requests/reconcile-access-bound: a delegate holds a Conduit to `access` listing reconcile.v1, with a bound on Pool workspace and file.read.v1.
    - Reconciling an open inside the bound passes the access decision and then gives conflict (OPS-030, the outcome is not unknown).
    - Reconciling one outside the bound gives refused, naming the bound.
    - No invocation of unknown outcome is needed. This avoids extending the runner to produce one.
30. conventions/requests/reconcile-access-root: root reconciles any such invocation and passes the access decision (control).

Ruling 66 point 4
31. conventions/requests/reconcile-sender-own: agent-b holds a Conduit to the Pool listing reconcile.v1 and reconciles its own unknown write: succeeded.
32. conventions/requests/reconcile-sender-other: agent-b reconciling agent-a's write gets refused. If that write is already reconciled, the answer is still refused, not conflict.
33. conventions/requests/reconcile-sender-hook: agent-b reconciles the hook of a setup it sent: permitted.
34. conventions/requests/reconcile-sender-fleet: after the Fleet core lands, a holder of a Conduit to Fleet F reconciles an invocation an Agent of F sent (permitted) and one another Fleet's Agent sent (refused). This case moves to the Fleet order if that order lands later.

EXISTING VECTORS WHOSE OUTCOME OR INPUTS CHANGE (each verified by reading)

F3:
- conventions/access/collision-missing-contract, collision-secret, collision-control-type and collision-control-template: step 0 goes from `accepted: true` to invalid (IFC-014). In each, documents.json has a standalone Conduit `c` that shares its consumer's declared Conduit `c` and has no tool in common.
  - Their titles ("No tool collision ... even with two unopened Conduits") and expected state change.
- conventions/access/tool-collision stays invalid. Its two exact diagnostic texts stay the same if the tool-name diagnostic is still emitted where tools collide; otherwise they are re-recorded.

Tombstones without a lift (A7):
- conventions/setup/restore, step 14;
- conventions/setup/restore-current-template, step 12 (lifecycle-fixture);
- conventions/changes/t4, steps 10 and 15;
- conventions/s1/whole, step 50 (YAML).
- In each, root closes a declared Conduit and then calls register-replacement, which today succeeds and would report conflict.
- Rewrite: insert root's capture and `access.lift.v1` before each register-replacement. The later outcomes are then unchanged.

Descriptions:
- interface/schemas/e05-local: the asserted description "Local path tool" changes. Its fixture needs the target's name and Type.

lift on root-access:
- The exact list of root-access operations gains lift.v1 in conventions/first-start/initial, repeat and lifecycle, language/first-start/protected, and conventions/s1/whole step 0. These are the same steps where the Fleet area adds list.v1 and root-system.
- fixtures/root-system-state.json: root-access-abc02 gains lift.v1. This is a snapshot input, needed by the inserted lift steps. No case asserts its operation list, and no case lists root's tools.

CHECKED UNCHANGED
- C1: scan of all 367 cases plus their requests and observations files.
  - 81 calls to open, close or reconcile tools; 20 are not by root.
  - 15 opens and 2 closes of c611 by agent-a in conventions/access/bounds. The consumer is agent-a in every open. Observation 16, agent-a closing the root-consumed c611, stays refused.
  - 3 calls to `admin.reconcile.v1` by agent-b in requests/reconcile-true, reconcile-false and restart-interrupted. All are refused because no such tool is presented.
  - So C1, C2 and point 4 change no outcome. Bounds observation 14, a Conduit to `access` listing reconcile.v1, stays refused under ACC-030.
- Tombstones: scan for a register or open after a close or retire found 8 cases. Four keep their outcome because invalid comes before conflict:
  - setup/restore-missing-template step 12;
  - setup/restore-retired-target step 12;
  - changes/retirement step 9;
  - setup/close steps 18 and 19.
- records/expected-id builds its name reuse from a snapshot, so no tombstone exists and nothing changes.
- inventory (an added output) and expectedId on open and close (an added input): assertions are projections, and no vector asserts the schemas of the open or close tools. Expect no change; confirm by running.

ALSO
- experiments/011-short-form-spike: its 14 exact translations should load once `as` lands. Re-run it as a check. Its test that asserts 14 rejections will then fail; update it or keep it as dated evidence.
- The engine's shell tests list root's tools (crates/shell/tests/agents.rs and serve.rs) and gain access.lift.v1.

## Build

Paths assume order 033's rename (reference-model/ to engine/) has landed.

TWO WORK ORDERS AFTER THE BATCH'S SPEC ORDER
WO-1 is split off because it unblocks the short-form translator early; the two can be merged.

WO-1, core additions part 1: `as` with IFC-014 F3, expectedId on open and close, IFC-033 descriptions, and ACC-054 (C1).
- engine/crates/core/src/registration.rs: `as` becomes `local_name`, and the local-name uniqueness check is added to IFC-014. Today local_name is set only for first-start Conduits (lib.rs:707-714) and read in registration.rs around line 1906.
- requests.rs: expectedId on open and close, reusing LIF-037's path (records.rs:32, lifecycle.rs:80, requests.rs:519), and ACC-054's check on the sender.
- access.rs: the tools are unchanged; they already key on local_name.
- schemas.rs: IFC-033's description (schemas.rs:1107-1113 today copies only the contract description). It needs the description of the target's held Type.
- Loader: none. It validates against spec/schemas/instances.yaml.
- Vectors: new 1-8 and 25-28; the 4 collision cases, tool-collision and e05-local re-expected.
- Size: about 250-400 core lines, plus about 150-250 lines of unit tests.

WO-2: tombstones, lift, inventory and the reconcile limits.
- core/src/lib.rs: State gains a separate `tombstones` map, so that state projections and snapshots without it survive.
- retirement.rs: instance tombstones at dispatch of a retirement; CON-008 cascade tombstones in close_ends.
- requests.rs:
  - tombstones on close;
  - the lift.v1 route;
  - LIF-041 at open;
  - ACC-055 in the reconcile path around lines 900-1090, using invocation["sender"], which hooks already inherit at line 1536.
- registration.rs: LIF-041 at every instantiation, including nested instantiations and register-replacement.
- lifecycle.rs: the inventory in inspect, and LIF-041 on setup parts.
- access.rs: bound coverage for lift and for reconcile through `access`.
- The defaults exemption hooks into the Fleet core's default making.
- engine/crates/conformance: tombstones in snapshots and state projections, and capture from diagnostic properties (check that it exists). No producer of unknown-outcome opens is needed (see vectors 29 and 31-32).
- engine/crates/shell: `conduit inspect` and reach read the inventory in place of order 027's CLI-only view. The plan and apply flags `--reopen`, `--recreate` and `--replace` map to lift.v1 but belong to the short-form or shell order.
- Vectors: new 9-24 and 29-34; the restore, t4 and s1/whole lift insertions; the root-access lists in first-start cases and the snapshot.
- Size: about 450-700 core lines, about 250-400 lines of tests, about 80-200 runner lines and about 50-150 shell lines.

TOTALS
- Engine: about 0.7-1.1k core lines plus tests.
- Vectors: about 34 new case directories and about 12-14 changed files.
- Spec: 5 new promises, about 20 amended (about 13 in substance), 1 new operation, 2 new result schemas, 1 schema field and 4 definitions entries.
- Gate: every check in AGENTS.md, and `cargo test --release` plus clippy in engine/.
- Each order's report lists every existing vector whose outcome changed, against the lists above.

## Ruling points

IMPLEMENTED, WITH WHERE
Ruling 53 #3, the five additions:
- (a) `as`: OBJ-013, IFC-012, IFC-014, instances.yaml, and definitions' "local name". Experiment 011 found it is needed first: instance.v1 is a closed schema.
- (b) expectedId on open.v1 and close.v1: LIF-037, operations.yaml, and definitions' "ID". It extends ruling 44's D12 list. make.v1 (batch 2) takes no expectedId, because its object is new.
- (c) The inventory without counts: LIF-013, inspect.v1, inventory.v1 and ACC-033's observable. It is read with ruling 59 A4: entries are shown in full to whoever may inspect the object, which is ruling 52's tier 2. No count field exists.
- (d) Tombstones enforced by the engine at open and register: LIF-040 and LIF-041, plus cross-notes in CON-008, ACC-005, ACC-006, ACC-012, LIF-009, LIF-035, OBJ-002 and REC-004. They use F2's two Conduit keys and F2's cascade cause.
- (e) Tool descriptions naming the target and its Type: IFC-033. IFC-038 was checked and is unchanged.

Ruling 55 A7: lift.v1, a separate operation of access-control.v1, in ACC-053, operations.yaml, capabilities.yaml, the root-access template and ACC-033 row 10. ACC-030 (passing it on only as it is or narrower) and the bound's coverage make it withholdable. Batch 2's PRJ-006 can exclude it from Fleets.

Ruling 55 A3 and A4 (interplay): every stable-name replacement is close or retire, then lift, then register, then open. The plan sends the lift without a flag for tombstones made in the same apply. `--reopen` and `--recreate` are lift.v1 requests counted as flags. Those flags are the shell area's.

Ruling 59 A3:
- Defaults are exempt (LIF-040 and LIF-041).
- IFC-014 F3 supplies the collision rule for root's `as`.

Ruling 62:
- C1: ACC-054, plus open.v1's description. It is the sibling of ACC-024 as amended, which the Fleet area writes.
- C2: ACC-055(a), plus reconcile.v1's description, OPS-014 and LIF-028.

Ruling 66 point 4: ACC-055(b), ACC-033 rows 13, 14 and 14A, and OPS-008's sender. This amends ACC-033's reconcile rows for holders other than root, as ruled.

Ruling 52 points 1 and 2: every new refusal and conflict carries an explicit diagnostic or restriction, without disclosure beyond tier 2.

Ruling 61 (batch 2), not foreclosed:
- B8: LIF-041 conditions any instantiation and ACC-054 uses SEC-008's form, so make.v1 adds only "or make".
- B11: lift.v1's input is a neutral `id`, and tombstone.v1 has `within?`.
- B10: the keep guard is added for withholds only; tombstones keep A7's plain rule.
- B7: ACC-055 already words "a Fleet or its Agents".
- IFC-048's expanded tools extend IFC-033 and keep IFC-014's tool-name clause.

FINDINGS AND DECISIONS FOR THE COORDINATOR OR THE OWNER
None is unimplementable. Four need a decision or the owner's awareness.

1. The one-time cascade Conduit (ruling 59 A3, d1). Root opens, uses and closes it, and each close leaves a tombstone on (root, Pool, pool-management.v1). The next cascade would then report conflict.
   - Recommended: the cascade command lifts its own tombstone within the same command, with all four requests printed in the plan.
   - Alternative: exempt Conduits whose consumer is root. This loses the guard against a stale working copy re-widening root's own Conduits, such as turn Conduits.
   - Decide with the Fleet planner.
2. Ruling 66 point 4 names Pools and Fleets only. A holder other than root with a Conduit to `management` that lists reconcile.v1 still reconciles anyone's invocations. This is flagged as a possible gap; it is rare in v0.
3. Under A7 combined with A3, any actor that replaces an object under a stable name must hold lift.v1. A bounded delegate without it cannot do stable-name replacements. This is consistent with the ruling; the owner should know.
4. F3 (local-name uniqueness regardless of tools) comes from the short-form synthesis's adopted fixes rather than an explicit owner answer. It flips 4 vectors. It is recorded as a coordinator decision under rulings 53 and 55 that the owner may strike. Ruling 59 A3's name rule depends on it.

READINGS RECORDED AS COORDINATOR CHOICES
- A tombstone hit reports conflict, mirroring OBJ-025 and LIF-037, and not refused.
- `by` (who closed or retired) is shown only to root and to that caller, as in F2.
- Retention lasts until lifted; there is no purge route. Growth goes in the README limits.
- "It sent" means the sending instance of OPS-008, not the caller of an onward request. A hook counts as sent by the sender of its setup or retirement.
- C2's "a bound limits reconcile" means the reconciled request's own coverage. The brief's source, [inventory-authority.md](../../projects/brief.md#unverified-authority-gaps), found no text on this.
- The key is named `within`, not "scope", which ruling 66 uses for an Interface's managed set (ruling 42).
- register-replacement no longer acknowledges a tombstone: F2's proposal conflicts with A7.
- Ruling 53's literal "listing only what the caller may inspect", read with ACC-033 row 7, would list nothing except to holders of `management`. Ruling 59 A4 settles it as implemented. No action is needed.
- Not included, open for the owner: adding a Conduit's own `metadata.description` to tool descriptions (useful under ruling 52's explicitness, but not ruled).
- For the peer area (ruling 62 C4): whether a peer's admitted Interface counts as root for C1.

## Dependencies

MUST LAND BEFORE OR WITH THIS AREA
- Order 033 (reference-model/ becomes engine/), for the paths.
- One decision for the whole batch on the core package's version: edit 0.1.0 in place or publish a new release. The Fleet area raises the same question for `owner` on Agents.
- The Fleet area (rulings 59 and 60), written in the same spec order:
  - the definition of the engine-made defaults (for the tombstone exemption);
  - ACC-024 amended (C1's closing counterpart);
  - ACC-033's new rows: 1A, 4A, 6A, 8A, 14A, 16, and the rows that lose Agents. ACC-033 is co-edited, so one editor writes it, or the edits are sequenced; this area's text is additive sentences.
  - object.v1's Fleet kind;
  - list.v1 listing tombstones through `management`;
  - Fleets as reconcile targets (needed for vector 34);
  - root-management's list.v1 and root-system, in the same first-start vectors and snapshot.
- The ruling 52 area: the tier wording of refusals (C1, ACC-055, lift) and the explicit diagnostics.
- The ruling 54 area: provenance (ACC-044) kept in tombstones, and provenance and review marks added to inventory items. The schemas here leave room for them.

BLOCKS
- The short-form translator: its translations carry `as` (experiment 011: `as` is needed first). WO-1 should land first, and experiment 011 should then be re-run.
- The shell's plan and apply under ruling 53 #4 and ruling 55 A4:
  - expectedId on every open and close;
  - drift detection from the inventory;
  - `--reopen`, `--recreate` and in-apply lifts on stable-name replacement.
- The Fleet area's name collision with root's `as` (relies on F3).
- Ruling 66's `manages:` expansion. Its Conduits carry `as`, their inspection shows the scope through the inventory, and scoped reconciliation uses ACC-055.
- Ruling 62 C5's "every administrator shown in inspection" uses the inventory's targetOf.
- Ruling 52's tier 2 "Conduits open or closed": closed Conduits are knowable only through tombstones.
- The S1 rerun, which needs T7 restore through `--reopen`, T5 and T6 stable-name replacements, and T2's history convention reaching Agents through descriptions.
- Batch 2: make.v1 (tombstones and C1 apply), withhold with lift (B11), keep (B10), a Fleet's own reconciliation (B7) and expanded tool descriptions (IFC-048).
