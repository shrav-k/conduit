# Spec batch 1, area: fleet

Not binding. An area planner's draft for [the batch 1 plan](../README.md); where the plan differs, the plan governs.

## Scope

The Fleet family: rulings 59 and 60, with ruling 54's ownership points (1 and 5). It covers:
- the kind `Fleet`, `core#fleet`, `core#fleet.v1` and `core#fleet-management.v1`;
- FLEET- promises that mirror POOL-001 and POOL-004;
- Agent ownership and ACC-033 rows 1A, 4A, 6A, 8A and 14A, plus `list.v1` (row 15);
- retire-with-agents;
- root's standing Conduits, `system` included, and the ACC-024 amendment;
- `list.v1` and the inventory reading;
- bound items keeping the IDs they resolved to;
- names: no Pool and Fleet share a name, and `management`, `access` and `system` are reserved;
- what a Fleet is not.

Repository: this one. Nothing in it was modified.

## Promise and core changes

The promise IDs are provisional; the coordinator allocates IDs across the batch-1 areas. I number FLEET- promises in sequence and leave FLEET-007 onward for batch 2, where the brief's PRJ-006 to PRJ-009 (holdings, acting for, forwarding, no server) land as FLEET-007 to FLEET-010. No spec file carries change narrative (work order 032).

## A. NEW PROMISES: spec/promises/fleets.yaml (new file)

**FLEET-001** (language, must). The family.
- When: a Fleet is instantiated, or a template or Type of the family Fleet is read.
- Then:
  - A Fleet is an instance of the core template `fleet`, whose Type is `fleet.v1`.
  - No other template or Type of the family exists. One in a submitted project or a package is rejected.
  - Its document has `apiVersion`, `kind`, `from` and optionally `metadata`. It has no standard argument and no `arguments`.
- Observable:
  - A Fleet has no config, placement, `runtime`, run, hooks, entries or declared Conduits.
  - It offers `fleet-management.v1`, which the engine implements with no selection (TPL-004).
  - It is registered, set up, retired and inspected through `management` (ACC-033 rows 2, 5 and 7).
  - Its setup has no parts. Its shutdown is confirmed once its retirement succeeds, or at once if it was never set up (LIF-024).
  - Each of these is rejected with a diagnostic naming it, and its registration reports invalid (OBJ-023):
    - a Fleet document that writes `owner`, `placement`, `runtime` or `arguments`;
    - a template of kind Fleet;
    - a Type whose family is Fleet, outside the core package.
  - It owns Agents and nothing else. It contains no Pool and no machine.
- This follows the brief's section 1, carried under A1.

**FLEET-002** (language, must). Ownership; mirrors POOL-001 and ruling 54 point 1.
- When: an Agent is instantiated.
- Then: it has exactly one owner, a Fleet instance, and no operation changes it.
- Observable:
  - An Agent document with no `owner`, or whose `owner` is not a Fleet, is rejected with a diagnostic.
  - `owner` is a bare name. Its kind is fixed by the new `fleet-reference.v1` (OBJ-006).
  - Inspecting the Agent at any later time reports the same owner.
  - An Agent wanted in another Fleet is a new Agent with its own ID, declared there:
    - under a name no existing Agent has; or
    - under the old name, only once the old Agent is retired and removed and its tombstone lifted (rulings 53 and 55 A3 and A7).
  - No request moves an Agent.

**FLEET-003** (language, must). A live owner; mirrors POOL-004.
- When: an Agent document names a Fleet as owner.
- Then: where the Fleet is not retired, the Agent is instantiated and owned by it. Where the Fleet is retired, the document is rejected.
- Observable:
  - The diagnostic names the Agent and the Fleet and says the Fleet is retired. The Fleet owns the same Agents as before, and the registration reports invalid (OBJ-023).
  - Registering an Agent into a live Fleet needs `management` and that Fleet (row 1A).
  - With the Fleet in the same submission, it needs `management` alone (row 2).

**FLEET-004** (convention, must not). Owning permits nothing; mirrors ACC-028.
- When: a Fleet owns an Agent.
- Then (must not): that ownership permits any request.
- Observable:
  - A Fleet is never a caller. No credential names a Fleet, and no server is presented to one.
  - An actor whose only open Conduit is to the Fleet can use the Fleet's own operations. It reports refused on every operation of the Fleet's Agents, a turn among them: running an Agent's turns takes a Conduit to the Agent.
  - A Fleet applies nothing to its Agents together. Every request names one Agent; retire-with-agents dispatches one retirement per Agent.
  - A bound that names a Fleet covers the Fleet alone (ACC-029).
- Batch 2 adds "a Conduit a Fleet holds is used only through `act.v1`" and keeps "never a caller", so this does not foreclose ruling 61's forwarding (B3).

**FLEET-005** (convention, must). Root's standing Conduits (A3).
- When: a Pool or a Fleet is registered.
- Then:
  - Within that registration, the engine instantiates and opens one Conduit whose consumer is root and whose target is the new Pool or Fleet.
  - It is made from a core template:
    - `root-pool` lists `register.v1`, `register-replacement.v1`, `inspect.v1`, `list.v1`, `set-up.v1`, `retire.v1` and `reconcile.v1`, but not `retire-with-resources.v1`;
    - `root-fleet` lists `register.v1`, `inspect.v1`, `list.v1`, `set-up.v1`, `retire.v1` and `reconcile.v1`, but not `retire-with-agents.v1`.
  - Its name is generated under OBJ-003, and its local name (`as`) is the target's name.
  - `system` instead has root's declared Conduit `system`, made at first start (FST-002).
- Observable:
  - `register.v1`'s result lists it with its name and ID. Inspection shows it open, with cause "default". Its registration and opening are recorded within that registration.
  - No request closes it (ACC-024). It closes when its target retires (CON-008).
  - No tombstone refuses making one, so a Pool or Fleet re-made under a stable name gets its own.
  - If root already has a Conduit, not closed, whose local name equals the target's name, nothing is registered. The registration reports invalid with a diagnostic naming that Conduit.
  - It lists exactly the operations printed above. An operation added to a capability later (NAM-024), such as `act.v1`, is not listed.
- C4's peer engines extend "root" to the admitted Interface; that is the peer area's change to this promise.

**FLEET-006** (language, must). Names (A3).
- When: a document would make a Pool or a Fleet instance.
- Then: its name is none of `management`, `access` and `system`, and no Pool instance and Fleet share a name.
- Observable:
  - A reserved name is rejected, and the registration reports invalid (OBJ-023). Where it is also a clash, as for a Pool named `system`, OPS-023 orders the reasons.
  - A Fleet named like an existing Pool instance, or the reverse, is rejected as an OBJ-002 clash and reports conflict (OBJ-025), naming both. Within one project, NAM-027 already rejects it.
  - Names stay engine-wide per kind. A Pool or Fleet may share a name with an Agent, Interface or Resource, subject to FLEET-005's clash rule.

## B. NEW PROMISE: list.v1

ID provisional: OPS-035, with `act.v1` taking the next free ID in batch 2. Convention, must.
- When: an actor asks for `list.v1` through an open Conduit to `management`, a Pool or a Fleet that lists it.
- Then, what each container lists:
  - through `management`: objects of every kind (templates included), the installed packages and the tombstones;
  - through a Pool: the Resources it owns;
  - through a Fleet: the Agents it owns.
- Then, what is shown:
  - only the items the caller may inspect, under ACC-033, through any open Conduit it holds;
  - no count, total, cursor or position, and nothing from which one could be read;
  - an optional `kind` input narrows the list.
- Observable:
  - Authorized against the container (ACC-033 row 15).
  - Each item has kind, name, ID, whether it is a template, and its state.
  - An Agent of a live Fleet appears through `management` only to a caller that may inspect it through that Fleet.
  - A retired container's Conduits are closed, so its remaining instances are listed through `management`.
  - Batch 2 adds optional output members: the Conduits a Fleet holds and its withholds.

## C. CHANGED PROMISES

Objects:
- OBJ-001: six families.
- OBJ-020: eleven kinds.
- OBJ-013:
  - `owner` is a standard argument of a Resource and an Agent, required for both;
  - a Fleet has no standard argument;
  - a Conduit's target may be a Fleet; its consumer may not, in batch 1.

Conduits:
- CON-024: the management verbs a Fleet authorizes are dispatched while the Fleet is registered.
- CON-027: a Pool or a Fleet given as a consumer is rejected. Batch 2 changes this for Fleets.

Contracts and templates:
- CTR-004: a Type of the family Fleet exists only in the core package.
- TPL-001 and TPL-002: a Fleet template holds `type` only and selects a Type of its family.

Placement:
- PLC-006: a Fleet gives no placement.

Lifecycle:
- LIF-001: Fleets report one lifecycle state like every instance.
- LIF-020, generalized: "a Pool that owns a Resource, or a Fleet that owns an Agent, whose shutdown is not confirmed" reports conflict, naming each.
- LIF-021, generalized: "retire a Pool with its Resources, or a Fleet with its Agents":
  - the order puts an Agent before each Agent of the Fleet that it refers to or is the consumer of a Conduit to;
  - the `runtime` clause stays Resource-only;
  - the operation is `retire-with-agents.v1`, requested of the Fleet;
  - the alternative is new LIF-038 and LIF-039, as the brief has it; I recommend generalizing, which gives one rule and no duplicated 40-line text.
- LIF-022 and LIF-034: "closing one of root's declared Conduits or a default is refused under ACC-024".
- LIF-023: an Agent is retired with its Fleet under LIF-021.
- LIF-037, OPS-029 and the description of `inspect.v1`: "no owning Pool or Fleet".

Access:
- ACC-012: point its root sentence at the amended ACC-024.
- ACC-024, amended, keeping the "must not" form. Closing a Conduit whose consumer is root is refused unless:
  - root asks on its own behalf; and
  - the Conduit is neither one of root's declared `management`, `access` and `system` nor a default.

  CON-008 still closes these when their target is retired. Root closing its own Conduit never locks it out, since its unbounded `access` (ACC-022) opens another.
- ACC-029 gains:
  - "A bound that names a Fleet in `targets` covers the Fleet alone and none of its Agents."
  - "An item that names an individual target holds the ID it resolved to (OBJ-021), so it covers no object made later under that name, a Resource replaced under its name included; naming its Pool covers the replacement." This is A7.
- ACC-033:
  - `when` gains retiring a Fleet with its Agents, and listing.
  - Row 1A: registering an Agent whose owner is a live Fleet needs `management` and that Fleet, each listing `register.v1`.
  - Row 2 covers an Agent in the same submission as its Fleet. Row 3 is unchanged, since Agents have no entries.
  - Row 4A: setting up or retiring an Agent goes through its Fleet, and through none where the Fleet is retired.
  - Row 5: an Interface, a Pool or a Fleet; Agents are removed.
  - Row 6A: inspecting an Agent goes through its Fleet, or through `management` where the Fleet is retired.
  - Row 7: Agents are removed and Fleets added.
  - Row 8A: `retire-with-agents.v1` goes through that Fleet.
  - Row 13: "a retired Pool or Fleet".
  - Row 14: Agents are removed and Fleets added.
  - Row 14A: an invocation dispatched to an Agent is reconciled through its Fleet, or through `management` where the Fleet is retired.
  - Row 15: `list.v1` goes through the container.
  - "No request is authorized against a retired Pool or a retired Fleet."
  - The observable mirrors the Pool sentences, including "a retired Fleet whose shutdown is not confirmed is retired again through `management`" (LIF-026).
  - The brief also says that "an Agent of a retired Fleet whose shutdown is unconfirmed is retired again through `management`". That state cannot arise, because a Fleet retires only once every Agent's shutdown is confirmed, so I do not write it.

Operations, revisions and results:
- OPS-014 and OPS-019: `fleet-management.v1` joins the management verbs, and the reconcile target names the Fleet.
- REV-010: an Agent's control is reconciled through its Fleet.
- REV-014: `retire-with-agents.v1` holds back the Fleet's other setups, retirements and changes.
- IFC-025: the LIF-020 diagnostics name the Agents in the way.

First start:
- FST-002: root has three declared Conduits, `management`, `access` and `system`, all open.
- FST-003: no Fleet and no other Conduit at first start.
- FST-005: the template list gains `root-pool`.

## D. CORE PACKAGE (spec/core/)

These are in-place edits of the unreleased 0.1.0, following ruling 41's precedent; spec/core/README.md says so in its open issues.

Types:
- `fleet.v1`:
  - `family: Fleet`, with an empty schema;
  - offers `fleet-management.v1` and requires its operations.
- `pool.v1` and `management.v1` require `list.v1`.

Capabilities:
- `fleet-management.v1` lists `register`, `inspect`, `list`, `set-up`, `retire`, `retire-with-agents` and `reconcile`.
  - I drop the brief's `register-replacement.v1`, since no row authorizes it against a Fleet.
  - Batch 2 adds `act.v1` under NAM-024.
- `object-management.v1` and `pool-management.v1` gain `list.v1`, and their descriptions change.

Operations:
- `list.v1`, as in B.
- `retire-with-agents.v1`, mirroring `retire-with-resources.v1`, with no inputs and no outputs.
- `close.v1`'s description: the root sentence follows ACC-024.
- `set-up.v1`, `retire.v1`, `inspect.v1` and `reconcile.v1`: their descriptions name the Fleet's authority.

Templates:
- `fleet`, of kind Fleet, with `type: core#fleet.v1`.
- `root-pool` and `root-fleet`, Conduit templates.
- `root` declares a third Conduit, `system: {from: root-pool, target: Pool system}`.
- Optional, decided with ruling 66's standard templates: cascade templates `retire-pool` and `retire-fleet`, each listing only its cascade, for the CLI's one-time Conduit.
- Do not name a template `root-system`: the conformance snapshot already holds an instance named `root-system`, and the clash would confuse readers.

References and results:
- New `fleet-reference.v1`.
- `target-reference.v1`, `reference.v1` and `object.v1` gain Fleet.
- `consumer-reference.v1` and `actor-reference.v1` stay unchanged until batch 2.

Manifest (conduit.yaml):
- the new exports;
- `engine` implements `list.v1` and `retire-with-agents.v1`.

## E. SCHEMAS (spec/schemas/)

- The `kind` enums gain Fleet in the envelope and document schemas.
- `instance.v1`:
  - a Fleet branch with no `owner`, `placement`, `runtime` or `arguments`;
  - the Agent branch requires `owner` as a `fleet-reference.v1`.
- `template.v1`: a Fleet branch holding `type` only, so that the core template validates. FLEET-001 rejects any other.
- `type-spec.v1`: the family enum gains Fleet.
- Examples:
  - `valid/project/instances.yaml` gains a Fleet and the owners;
  - `valid/first-start` gains root's `system` Conduit;
  - new invalid examples: an Agent with no owner, an Agent whose owner is a Pool, a Fleet with a placement, a Fleet template in a project, a Type of family Fleet, a reserved Pool name.

## F. definitions.md

- Rows changed: family (six), kind (eleven), Agent (owned by exactly one Fleet), owner (a Pool or Fleet instance), standard argument, root Interface (three declared Conduits; closes only its own others), management verb, and the core-terms table.
- New row, default Conduit.
- New row, Fleet: "One of the six families: what owns Agents, as a Pool owns Resources (FLEET-002)."
  - Ownership permits nothing. Administering an Agent goes through a Conduit to its Fleet, and running its turns takes a Conduit to the Agent.
  - A Fleet runs nothing, applies nothing to its Agents together (they are never configured, upgraded or started as one) and holds no count of copies.
  - It is engine-local, has no placement, contains no Pool, is not a group of machines (a machine is a node) and is never a caller.
  - An Agent never changes Fleet.
  - Outside products are cited by name.
- "Words the promises do not use" gains "fleet operator", "the fleet" meaning all Agents, and "dispatch" applied to a Fleet.

## G. spec/README.md

- Contents: a fleets.yaml row.
- Limits:
  - A Fleet organizes administration and isolates nothing (IFC-022).
  - Nobody who administers only a Fleet registers an Agent: row 1A needs `management` too.
  - A bound item naming a single Resource stops covering it after a replacement under its name. Bound on Pools.

## H. FOR THE SHORT-FORM AREA (A6): an SCT rule I supply

- A top-level `fleet: <name>` makes the Fleet from `core#fleet` and gives `owner: <name>` to every Agent in that document. Such an Agent writes no `owner`; if it does, it is rejected at that key.
- Without `fleet:`, each Agent writes `owner:` as a bare name.
- Two documents of one project with the same `fleet:` make one Fleet, a narrow exception to NAM-027.
- `fleet:` makes the Fleet. To add Agents to a Fleet registered earlier, a document writes `owner:`.
- `operator:` on a Pool is rejected, and [SCT-010](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N4-minimal-normative.md#L140) is not written: the engine makes root's Conduits.
- The reserved short-form names already cover FLEET-006.

## I. CLI (stated in the plan's or the CLI's reference)

- Commands:
  - `conduit list [--kind]`;
  - `conduit fleet list|inspect`;
  - `conduit retire <pool> --with-resources` and `conduit fleet retire <f> --with-agents`.
- Each cascade is one command that registers and opens a one-time Conduit from root, calls the cascade and closes it. The plan prints all of them, and the flags join ruling 55 A4's counted named flags.
- An apply that would change an Agent's owner is shown as a replacement under a flag, never a move.
- The plan of a replacement prints every bound that stops covering the Resource (A7).

## J. READINGS THE COORDINATOR DECIDES (each within the ruling)

1. "May inspect" in `list.v1` means through any Conduit the caller holds. Recommended: it matches "root walks" and ruling 52's tier 2.
2. Generalize LIF-020 and LIF-021, or add LIF-038 and LIF-039.
3. Omit `register-replacement.v1` from `fleet-management.v1`.
4. Template names: `root-pool`, `root-fleet` and the cascade templates.
5. The core package is edited in place at 0.1.0. A bump would change the core version that inspection outputs show in many vectors.
6. Do not cap Fleet names at 31 characters now. In batch 2, `acts-for` can take an explicit `as`.

## Vectors

## NEW CASES (about 28)

The paths are proposals.

language/registration:
- fleet-instance: a Fleet from `core#fleet` is accepted. These are rejected with a diagnostic and report invalid:
  - a Fleet that writes placement, runtime, owner or arguments;
  - a Fleet template in a project;
  - a Type of family Fleet in a project.

  Shows FLEET-001, CTR-004, PLC-006 and OBJ-013.
- agent-owner: an Agent with no owner, with a Pool as owner, or with a missing Fleet as owner reports invalid. Inspection later shows the same owner. Shows FLEET-002.
- agent-owner-retired: registering an Agent into a retired Fleet reports invalid, and the Fleet's Agents are unchanged. Shows FLEET-003.
- fleet-names, which shows FLEET-006 and FLEET-005's clash rule:
  - a Pool or Fleet named `management`, `access` or `system` reports invalid;
  - a Fleet named like an existing Pool instance, and the reverse, reports conflict naming both;
  - the same within one project fails NAM-027;
  - a Pool named like the `as` of root's operator Conduit reports invalid, naming that Conduit.
- fleet-consumer: a Conduit with a Fleet as consumer is rejected; a Fleet as target is accepted. Shows CON-027. It is marked to change in batch 2.

conventions/fleets:
- routing: shows ACC-033 rows 1A, 4A, 6A and 14A.
  - Registering an Agent into an existing Fleet with `management` alone is refused, with the Fleet alone is refused, and with both is permitted.
  - With the Fleet in the same submission, `management` alone is permitted.
  - Set-up, retire, inspect and reconcile of an Agent through its Fleet are permitted.
  - The same through `management`, or through another Fleet, is refused.
- routing-retired: once the Fleet is retired, its Agents are inspected and their invocations reconciled through `management` (rows 6A, 14A and 12).
- ownership-permits-nothing: the Fleet's own operations are permitted; a turn on its Agent through it is refused. Shows FLEET-004.
- retire-conflict: shows LIF-020 for a Fleet.
  - Plain retire of a Fleet with a ready Agent reports conflict naming it.
  - An Agent retired with its shutdown pending is still in the way.
- retire-with-agents, which shows LIF-021, ruling 54 point 5 and CON-008:
  - an Agent that consumes a turn Conduit to another Agent retires first;
  - the Fleet retires last;
  - the Conduits of each retired Agent close at both ends.
- retire-with-agents-partial: an unknown outcome holds back only the Agents ordered after it. The outcome is failed with diagnostics, and a repeat finishes.
- retire-with-agents-authority: a Conduit listing only `retire-with-agents.v1` suffices; through `management` the request is refused. Shows row 8A.
- setup-registered: management verbs to Agents are dispatched while the Fleet is registered, and the Fleet's own setup has no parts. Shows CON-024.

conventions/defaults:
- pool-default and fleet-default: show FLEET-005.
  - The registration result lists root's Conduit, with `as` equal to the target's name.
  - It is open, lists the exact operations, and its records belong to the registration.
- close-refused: root closing a default or a declared first-start Conduit is refused, and so is a non-root actor closing root's Conduit. Shows ACC-024.
- root-closes-own: root registers and opens a one-time Conduit listing `retire-with-resources.v1`, uses it on a non-empty Pool whose cascade fails, and closes it, which is permitted. Shows the amended ACC-024.
- closed-with-target: a default closes when its target retires. A Pool re-made under the same name gets a new default. Shows CON-008 and the tombstone exemption; written with the tombstone area.

conventions/first-start:
- root-system: root inspects `local`, `management` and a provider through `system.inspect.v1`. Retiring `management` through `system` is refused (LIF-034). No `system.retire-with-resources.v1` tool exists.

conventions/list:
- management, pool and fleet: each container lists what row 15 gives it.
- partial-authority-no-count: a non-root holder of one Fleet sees only its Agents. The result has no count or cursor.
- kind-filter.
- refused-without-list: a Conduit that does not list `list.v1` is refused.

conventions/access:
- bound-fleet-target: a bound naming a Fleet lets its holder open a Conduit to the Fleet and refuses one to the Fleet's Agent. Shows ACC-029.
- bound-kept-id: shows A7.
  - A bound naming `shared`; `shared` is retired and re-registered under its name; opening to the new `shared` is refused.
  - A bound naming its Pool is permitted.

conventions/inspection:
- inventory-reading: a holder of a Conduit to the Fleet listing `inspect.v1` sees every Conduit the Agent consumes and is the target of, in full. Inspecting one of those Conduits by name through the Fleet is refused (row 7).

language/short-form (if the A6 rule is placed in my area):
- fleet-key: the key sets every Agent's owner; an Agent that also writes `owner:` is rejected; a document without the key requires `owner:`; two documents with the same `fleet:` make one Fleet.

## EXISTING VECTORS WHOSE OUTCOME CHANGES

Verified by reading the case files:
- conformance/conventions/access/routing:
  - step 7 (observation index 6): agent-a's `manage.set-up.v1` of the Agent agent-b through `management` goes from permitted to refused (row 4A);
  - step 9 (observation 8): `manage.inspect.v1` of the missing Agent stays invalid, since objects that do not exist stay `management`'s.
- conformance/language/first-start/protected, steps 2 to 4: root registers the documents node.yaml, management.yaml and access.yaml, all owned by `system`, through root-management.
  - Today they report refused: row 1 needs `system`, and these steps start from first start.
  - With root's `system` Conduit listing `register.v1`, they become invalid under FST-006.
  - Recommendation: keep the refused coverage with a non-root Interface that holds only `management`, add the invalid steps for root, and correct conformance/README.md.
- conformance/conventions/first-start/initial, "Exactly five ready first-start objects and two open declared Conduits": becomes three declared Conduits. The object and record counts grow; the brief's scan gives 7 to 8 objects and 14 to 16 records.
  - repeat and the s1/whole first-start state change the same way.
  - lifecycle and lifecycle-pool-target: outcomes stay refused (LIF-034).
- conformance/conventions/setup/sender and optional: their requests.json call `admin.set-up.v1` on the Agent agent-a through a Conduit to `management`, which goes from permitted to refused. The brief's scan says sender-onward and sender-no-access do the same; I did not open those two.
  - Rewrite: give agent-b a Conduit to Fleet s1 listing `set-up.v1`. ACC-005 is unchanged.
- conformance/conventions/access/bounds, step 17 (observation 16): agent-a's `c610.close.v1` of c611 stays refused under the amended ACC-024.

From the brief's scan, not opened by me:
- access/fixed-kind-references step 2: from refused to invalid.
- s1-pool-authority keeps its four outcomes, and access/bounds all 17.

## INPUTS TO REWRITE (same meaning)

- Agent set-up through `management`: my scan found 41 requests in 22 case directories, counting echoed replies:
  - `management.set-up.v1` on agent-a 23 times, agent-b 10, agent-c 1 and a 1;
  - `admin.set-up.v1` on agent-a 4 times;
  - `manage.set-up.v1` on agent-b once.

  These become `s1.set-up.v1` through root's default to Fleet s1, or through a fixture Conduit to the Fleet. The shell tests at reference-model/crates/shell/src/main.rs:377-379 change too.
- The S1 fixture (conformance/fixtures/s1/project/instances.yaml) gains Fleet s1 and `owner: s1` on agent-a and agent-b.
  - Each registration step's exact `records` gain the default Conduits' events.
  - Cases outside the S1 fixture with Agents gain a Fleet document. The brief estimates about 48 cases, with about 21 to 25 that would otherwise go from accepted to invalid.
  - Also: changes/t4 step 2, changes/t5 step 5, spec/schemas/examples/valid/project/instances.yaml and reference-model/examples/s1.

Recommendations to limit the churn, each to be confirmed by the recorder:
- Keep the fixture's `root-workspace` Conduit in batch 1. It stays legal, since its tools `root-workspace.*` differ from the default's `workspace.*`, and keeping it avoids rewriting about 83 steps.
- Allocate the defaults' IDs after the submission's own objects, and put the Fleet document last, so that most of the 114 cases that hard-code IDs keep them.

Snapshot rule (coordinator decision):
- conformance/fixtures/root-system-state.json is used by 198 case directories. It already holds a fixture Conduit named `root-system` from root to `system`, listing `register.v1` and `retire.v1`, with ID ...050.
- Recommendation: leave it unchanged. conformance/README.md states that a snapshot is the case's established initial state and need not be what first start makes. The new first-start state is tested by the first-start cases above.

## BATCH-2 FLIPS TO EXPECT

- fleet-consumer: rejection becomes acceptance for the two kinds ruling 61 allows.
- root-access's and the Fleet's operation lists gain the new operations, unless the contracts land early.

## Build

Paths are under reference-model/, soon engine/.

Loader, about 0.1k to 0.3k lines:
- the FAMILIES constants in crates/loader/src/authored.rs:33 and schemas.rs:62 gain Fleet;
- schema-driven checks pick up instance.v1 and template.v1 from spec/;
- `owner` handling for Agents.

Core, about 1.2k to 2.0k lines:
- registration.rs:
  - the `owner` standard argument for Agents: the standards list at about line 981, and the owner-kind check at about 1004;
  - FLEET-001's core-only template and Type rule;
  - FLEET-003's live owner;
  - FLEET-006's names: shared Pool and Fleet namespace, reserved names, root local-name clash;
  - making and opening the default Conduit inside the registration transition, with its records and IDs allocated last.
- lib.rs `first_start`, at about lines 615 to 690: root's third declared Conduit, `system`.
- access.rs routing, at about lines 236 to 293:
  - rows 1A, 4A, 6A, 8A, 14A and 15;
  - rows 5, 7 and 14 lose Agents;
  - the retired-Fleet fallbacks;
  - the ACC-024 amendment in the close path (root's own request; never its declared Conduits or defaults).
- retirement.rs and lifecycle.rs: LIF-020 and LIF-021 generalized to owned Agents, with the ordering by references and Conduits, and `retire-with-agents.v1`.
- requests.rs: the `list.v1` handler (container members, may-inspect filter, no counts) and the inventory reading, if that area does not take it.
- records.rs: the defaults' events.

Shell, about 0.4k to 0.8k lines:
- commands: `conduit list`, `conduit fleet list|inspect|retire --with-agents`, `conduit retire <pool> --with-resources`;
- each cascade registers, opens, calls and closes, treating "already removed" as done, and prints its plan lines;
- the setup loop in main.rs, at about line 340, puts Fleet before Agent and sets Agents up through root's Fleet default, not `management`;
- identity.rs:226, lib.rs:285 and server.rs:804 do not get Fleet: no token and no server, per FLEET-004;
- the presented root server shows the `system.*`, default and `list.v1` tools.

Conformance runner, about 0.1k to 0.3k lines: comparing list results; probably no new step kind.

Vectors:
- about 28 new cases;
- the rewrites above, made through the batch's recorder order, which the brief lists as order (v) and which does not exist yet.

Suggested work orders for this area:
- F1, spec: fleets.yaml, the changed promises, spec/core, spec/schemas with examples, definitions.md and the README. About 600 to 900 lines of YAML and Markdown.
  - Checks: check_schemas, set_names --check, check_docs, and cargo test in reference-model.
- F2, engine: loader and core.
- F3, shell.
- F4, vectors: through the recorder order.

Ruling 61's batch 2 builds on this: `act.v1` on `fleet-management.v1`, a Fleet as consumer, and the forwarding path in requests.rs.

## Ruling points

## WHERE EACH RULING POINT IS IMPLEMENTED

Ruling 59:
- A1, ownership as Pools own Resources:
  - FLEET-002 and FLEET-003;
  - ACC-033 rows 1A, 4A, 6A and 14A; rows 5, 7 and 14 lose Agents;
  - FLEET-004, the ACC-028 mirror, and ACC-029's "a Fleet covers the Fleet alone";
  - ruling 30's authority in ACC-033's text and in definitions.md.
- A2, retire as a Pool: LIF-020 and LIF-021 generalized (or LIF-038 and LIF-039), `retire-with-agents.v1`, and row 8A.
- A3:
  - FLEET-005: the defaults from `root-pool` and `root-fleet`, generated names with `as` equal to the target's name, the operations listed except cascades and `act.v1`, and the tombstone exemption;
  - FST-002, FST-003 and FST-005, and root's declared `system` Conduit;
  - the ACC-024 amendment, with LIF-022, LIF-034, ACC-012 and `close.v1`;
  - FLEET-006: no Pool and Fleet share a name, and `management`, `access` and `system` are reserved;
  - the CLI's one-time cascade Conduit and its named flags.
- A4: the `list.v1` promise, row 15, the contract on three capabilities, and the inventory reading (ruling 52's tier 2 applied to ruling 53's inventory).
- A5: closed by ruling 60; nothing is renamed.
- A6: the SCT rule for `fleet:` handed to the short-form area (changes, H).
- A7: ACC-029's resolved-ID sentence, the plan line, and Pool bounds in the examples and README limits.

Ruling 60:
- the names: `kind: Fleet`, `core#fleet`, `fleet.v1`, `fleet-management.v1`, FLEET- and `conduit fleet`;
- ruling 30's line;
- the definition row stating what a Fleet is not, and the prose rules;
- "project" keeps its meaning, so nothing is reworded;
- S1's Conduit keeps its local name `project`;
- the enrollment proposal's use of "fleet" is reworded when it is next condensed (documentation only).

Ruling 54:
- point 1: FLEET-002 (no request moves an Agent; another Fleet means a new Agent);
- point 5: CON-008 already closes a retired Agent's Conduits; the retire-with-agents vector shows it.
- Points 2 to 4 belong to the provenance and review area.

## READINGS FOR THE COORDINATOR (none departs from a ruling)

- A1's "In v0 only root registers Agents": I read it as the consequence of row 1A needing `management`, not as a new reservation. The brief's section 2 calls it "the real narrowing, stated". It is a one-line confirmation for the owner's morning list; a hard reservation would be a new refusal.
- Fleet-only-core templates and Types (FLEET-001): taken from the brief's section 1 under A1.
- The brief's `register-replacement.v1` in `fleet-management.v1` is dead, since no row authorizes it against a Fleet. I recommend omitting it.
- The brief's "an Agent of a retired Fleet with unconfirmed shutdown is retired again through `management`" cannot arise. The real mirror is the Fleet itself, retired again under LIF-026.

## FINDINGS: RULED POINTS THAT NEED ANOTHER AREA'S RULE

1. A3's clash rule (a Pool or Fleet named like the `as` of one of root's open Conduits is rejected) is not given by IFC-014, which compares tool names only. A default to Pool x (`x.inspect.v1`) and root's operator Conduit to Agent x (`x.turn.v1`) share no tool name.
   - The `as` area must state that no two Conduits of one consumer that are not closed share a local name. FLEET-005 relies on it, and so does the reverse case.
2. Ruling 66 point 1 lets "a Fleet, for Pools" write `manages:`. That makes the Fleet a consumer, which batch 1 rejects (CON-027) and ruling 61 B2 opens only in batch 2.
   - The scoped-Interfaces area should defer that form to batch 2.
3. Under the amended ACC-024, root's one-time cascade Conduit and the re-made defaults must not trip ruling 53's tombstones.
   - The tombstone area must exempt defaults (A3, from the brief) and must key tombstones so that a cascade Conduit, with a generated name and no `as`, is not refused when retried.

## Dependencies

## BEFORE THIS AREA

- Work order 032 (spec history condensed) merged, so that new text carries no change narrative.
- Ruling 53's `as`: the promise and `instance.v1`'s field, with local-name uniqueness per consumer (finding 1). Defaults and FLEET-005 and FLEET-006 depend on it.
- Ruling 53's tombstones: the exemption for defaults, and the cascade Conduit (finding 3).
- Ruling 53's inventory in `inspect.v1`: the A4(i) reading is applied there. One order should write `inspect.v1` once.
- Ruling 55's sectioned-document grammar (SCT): needed for the `fleet:` key, [SCT-010](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N4-minimal-normative.md#L140) dropped, and `operator:` on a Pool rejected.
- ID allocation across the batch-1 areas by the coordinator: FLEET-, OPS-035 or OBS-, and ACC-044 onward.
- ACC-033 needs a single writer. Ruling 62's C1 and C2 (bound limits reconcile; only root opens Conduits whose consumer is root, next to ACC-024) and ruling 66 point 4 (reconcile by sender) edit the same rows.
  - Recommendation: one spec order writes ACC-033 and ACC-024 with all three inputs.
- The vector recorder order, before F4's rewrites.

## WHAT THIS AREA BLOCKS

- The S1 rerun. The fixture and the author's file need `fleet: s1`, `owner`, root's defaults and Agent setup through the Fleet. It also measures root's tool count (C11: about 37 on S1).
- Ruling 66: `manages:` needs `fleet-management.v1` and the core's standard manage and view templates (ruling 63 point 3), and scope membership by ownership.
  - Proposed: manage lists `inspect`, `list`, `set-up` and `reconcile`, plus `act.v1` in batch 2; view lists `inspect` and `list`.
- Ruling 62 C5: "another Fleet's Agents" needs ownership.
- Ruling 62 C4: peer engines extend FLEET-005 to the admitted Interface.
- Ruling 54's provenance and review: `cause: default` on engine-made Conduits; a retired Agent's Conduits close.
- Pending actions (rulings 50 and 56, scoped by ruling 66) follow Fleet membership.
- A78 orders 1 to 3: fixed slots registered in Fleets.

## RULING 61, BATCH 2: NOT FORECLOSED

- `act.v1` can be added to `fleet-management.v1` under NAM-024, and the defaults list their operations explicitly, so they never gain it.
- FLEET-004 says only "never a caller", and the Fleet gets no credential.
- CON-027 and OBJ-013 relax Fleet consumers then.
- `list.v1`'s output takes optional members for held Conduits and withholds.
- ACC-033 row 16 is left for `act.v1`.
- FLEET-007 onward is left for holdings, acting for, forwarding and no server.
- The ACC-024 amendment is the one ruling 61 assumes.
