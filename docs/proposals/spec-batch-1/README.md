# Spec batch 1: the plan

Not binding. The specification alone binds. The coordinator's plan, written on 2026-11-11 by a
workflow: six area planners, an integrator, three skeptics (fidelity to the rulings, safety and
consistency, feasibility against the engine's code) and a finalizer that applied their findings. It
turns the rulings listed in [IN_FORCE.md](../../../IN_FORCE.md) as *spec pending, batch 1* into
promise changes, vectors and work orders. The readings it applies are the coordinator's, and the
owner may strike any; the three questions under "For the owner" wait for the owner. The area
drafts, with the draft text of each new promise, are under [areas/](areas/). This paper condenses
into the records when the batch closes (ruling 58, point 10).

## What batch 1 changes

WHAT BATCH 1 CHANGES
Batch 1 is the spec that must exist before the S1 rerun, written once, plus the build the rerun uses.

Covered in full:
- rulings 49, 50, 51 (except the peer caveat), 52, 53, 55, 56, 59, 60, 63;
- ruling 54 point 1 (Agents never move) and point 3's provenance;
- ruling 62 C1, C2 and C5;
- ruling 66 points 1-4 and 6. Two parts of 66 wait: point 1's "a Fleet, for Pools" waits for batch 2, because no Fleet can be a consumer before then. Point 3's grant answers wait with the review.

Enrollment (50, 51.8, 52.4, 56) is written into the spec in batch 1 but built off the rerun path: it is needed before S3, not before the rerun.

Two ruled items are not in batch 1:
- Ruling 54 points 2-4 (the grant review, the list on close, the marks and keep). Ruling 61 B1 puts group B in batch 2, and B9 and B10 define the review's trigger and who may keep.
- Ruling 62 C4 (peer engines). It goes to the owner as R3: it needs an Interface other than root signed in at the peer, half of what it names only exists in batch 2, and whether the admitted Interface counts as root is unruled.

Ruling 61 is neither planned nor foreclosed.

In plain words, after batch 1:
- **Fleets.** Agents are owned by a Fleet, a new sixth family, the way Resources are owned by Pools. Setting up, retiring, inspecting and reconciling an Agent go through its Fleet. In v0 only root registers Agents, and the engine enforces this (ACC-057).
- **Root's defaults.** The engine gives root a default Conduit to every Pool and every Fleet. The one to `system` is made at first start. Root may close its own Conduits, except the first-start ones and the defaults.
- **Discovery.** `list.v1` lists only what the caller may inspect, with no counts.
- **The short form** is part of the language. Numbered SCT rules translate sectioned documents into unchanged canonical documents. Packages ship standard Conduit templates, and examples use flow style. NAM-024 forbids adding a default to a published Type, so the Agent Types ship as `.v2`.
- **Ruling 53's five additions:**
  - `as`: a local name for a standalone Conduit, unique per consumer;
  - `expectedId` on open.v1 and close.v1;
  - an inventory in inspect.v1, with no counts. It also shows an Interface's scope and a Pool's administrators;
  - tombstones, enforced at open and at register. A tombstone keeps the IDs it stood for. It is shown in full to whoever may inspect what it stood for. Objects the engine makes get none. lift.v1 of `access` removes a tombstone, and removes the tombstones it caused only where the caller's bound covers each one;
  - tool descriptions that name the target and its Type.
- **Access.** Only root opens or registers a Conduit whose consumer is root. A bound limits reconcile. A holder other than root reconciles only its own invocations through a Pool or Fleet. Every Conduit keeps its provenance.
- **Pending actions:**
  - one mechanism, with kind, subject, evidence, expiry and answer. Every action expires, and an expiry dispatches nothing;
  - raised at root and at every Interface whose scope holds the subject. What an Interface may answer is limited by what it holds;
  - kinds: sign-in, setup, notice (62 C5), plus enrollment, machine and machine-removal;
  - evidence codes and fingerprints never appear in a tool result. The person sees them only in the URL elicitation's message or in the engine's own panel.
- **Sign-in:**
  - by URL elicitation, only to a person's Interface, and only for Types that state `signIn`;
  - the expected account is checked, and a sign-in to any other account is undone;
  - a request held by a pending sign-in is final, and nothing is queued.
- **Other people-and-trust items:**
  - a private state directory per instance, which is the Agent's home;
  - refusals explained in two tiers, and the explicitness rule on every result;
  - machine enrollment, where the provider declares the evidence and the engine enforces the minimum;
  - a SPIFFE trust domain, named at first start.

SIZE
**Promises:** 60 new, taking today's 455 to 515. 47 are on the rerun path; 13 are enrollment, in 038b.
- FLEET-001..006
- ACC-044, ACC-053..057
- LIF-040, LIF-041
- OPS-036, FST-007
- SCT-001..009 and SCT-011..016, plus PKG-031
- PND-001..012
- IFC-049, IFC-050, CTR-013, PRV-029
- ENR-001..012

About 120 promises are amended, about 65 of them in substance. Four new promise files: fleets, sections, pending, enrollment.

**Core package:**
- 14 new operations: list, retire-with-agents, lift, answer, sign-in, sign-in-hook, account-hook, sign-out-hook, enroll-machine, admit-machine, refuse-machine, enroll, accept-machine, remove-machine.
- fleet.v1 and fleet-management.v1.
- 10 new templates: fleet, root-pool, root-fleet, pool-manage, pool-view, fleet-manage, fleet-manage-own, fleet-view, retire-pool, retire-fleet.
- 8 new schemas: inventory, tombstone, provenance, fleet-reference, trust-domain, pending-action, evidence-item, refusal.
- About 16 existing contracts changed.

**spec/schemas:** sectioned.yaml (about 10 schemas), the Fleet, `as` and owner branches, and about 30 examples.

**Other spec files:** about 45 rows in definitions.md. DL5, DL6, DL13, DL16, DL18 and a new DL19 (pending-action expiry). One key in names.yaml.

**Spec text:** about 7.4k-9.2k lines over 5 spec orders.

**Vectors:** about 215 new:
- translator ~70
- additions and provenance ~40
- Fleet ~32
- pending and sign-in ~30
- enrollment ~28
- refusals ~14

About 230-280 existing case files change:
- ~170 by a scripted transform (Fleet s1, owners, defaults, renumbered IDs);
- ~45 for refusals (~85 steps);
- 41 set-up requests in 22 directories;
- 6 collision cases;
- 5 lift insertions;
- first start twice (040, and 049a for the enrollment verbs).

**Build:** about 14k-19k lines of Rust including tests, plus about 0.5k of Python tooling. Plan and apply is unestimated until the CLI reference exists (likely 1.5-2.5k). The program's caution applies: estimates run 1.5 to 2 times low.

**Orders:** 27 work orders. Each targets about one Codex hour.
- 5 spec: 036a, 036b, 037, 038a, 038b.
- 11 core lane: 039, 040, 041a, 041b, 042, 046, 047, 048a, 048b, 049a, 049b.
- 11 other lanes: 043a, 043b, 044, 045, 050, 051a, 051b, 051c, 052a, 052b, 053.

Plus four coordinator tasks:
- K1: the MCP citation check;
- K2: the vector-delta classifier;
- K3: the CLI reference;
- K4: the IN_FORCE markers.

Orders 033 (the rename to engine/), 034 and 035 come first. 032 is merged.

## For the owner

Three questions. None blocks tonight's work. R1 is needed by order 053; R2 and R3 only before S3. I have not applied any of them.

R1. How do a person's sign-ins count in the S1 rerun?
- **Why it comes up:** under ruling 49 each real Agent has its own account. So T1 gains an `account:` line per Agent, which puts T1 at about 34 against the baseline's 33. Each Agent also needs one sign-in, for example `conduit sign-in agent-a`. Ruling 63.1 counts a command that carries a name the author chose.
- **Options:**
  - (a) Count each sign-in as an authoring line.
  - (b) List actions a person takes at the panel apart from authoring lines, as ruling 52.6 lists root's Conduits apart. Report the strict count beside it.
  - (c) Run the rerun's Agents on API keys. This skips what ruling 49 tests.
- **Recommendation:** (b), with the `account:` lines counted as authored lines.
- **Not applied:** 63.1 as worded governs until you answer.

R2. Should admitting a machine require the fingerprint the person read on the machine, with the engine comparing it?
- **Why it comes up:** fingerprints now never appear in a tool result (ruling 56.4). But a session driven by a model could still admit a machine without checking it, unless the command carries what the person read. Ruling 56.10 has the fingerprint shown on the machine and in the panel; it does not require typing it in.
- **Options:**
  - (a) Yes. The engine compares, except for a headless enrollment recorded with `--accept-weaker-check`.
  - (b) No. The person compares by eye.
- **Recommendation:** (a). It fits 55 A4 ("the command is the approval") and 56.4.
- **Waiting on it:** the enrollment spec order (038b) and its build. The rerun does not.

R3. Peer engines (62 C4 and ruling 51's caveat): when, and does the admitted Interface count as root?
- **Why it comes up:** you chose (a), the peer's root template names your admitted Interface. But C4 also names appointment, keep and withhold, which exist only in batch 2. Your Interface also needs a sign-in token at the peer, which is optional work here. And no ruling says whether it counts as root.
- **Options:**
  - (a) Batch 1. Admission works by the peer's root opening, for your Interface, a Conduit to `management` from a core template `peer-owner`. While that is open, your Interface gets a copy of every default, and every action raised at root. This adds about 1-2 promises, 1 template, about 10 vectors and about 0.5k lines, and brings the tokens onto the build.
  - (b) Batch 2, written together with appointment, keep and withhold. Batch 1 computes who receives the defaults and pending actions by a rule that today names only root, so nothing is foreclosed.
- **Either way, a sub-question:** does the admitted Interface count as root for these three rules?
  - only root opens root's Conduits (C1);
  - root may close its own (ACC-024);
  - only root registers Agents (59 A1).
- **Recommendation:** (b). Yes for those three rules, but not for the operations reserved to root (declare-lost, purge), which stay with the peer's own root.

READINGS I'VE APPLIED THAT YOU MAY WANT TO STRIKE (no answer needed)

Scope:
1. Grant review, list on close, marks and keep go to batch 2. Ruling 61 B1 puts group B there, and B9/B10 define the review.
2. 66.3's "closing a grant always" waits with the review.
3. A5 (your second accounts) does not gate the rerun. Scripted vectors prove the wrong-account undo first.

Sign-in:
4. A turn held by a pending sign-in reports "unavailable", naming the action, rather than "refused". The request is final and nothing is queued.
5. A sign-in to the wrong account is undone, ends "failed" naming both accounts, and the action stays pending.
6. "Checked" is read strictly: only Types that state `signIn` get sign-ins. Plain OAuth servers behave as today.
7. API-key Agents are not account-checked. Exactly one of `account` and `apiKey` is set.
8. Every pending action expires. The durations are left to the implementation, and expiry dispatches nothing.
9. Codes and fingerprints reach you only in the sign-in prompt or the engine's own panel.

Names and tombstones:
10. Two Conduits of one consumer never share a local name. This flips 4 vectors.
11. Only root registers a Conduit whose consumer is root, beside your C1.
12. A tombstone:
    - is shown in full, closer included, to whoever may inspect what it stood for;
    - is not made for objects the engine makes;
    - when lifted, takes with it only the tombstones it caused that the caller could lift itself.

Templates:
13. Root's defaults include sign-in and answer: 9 tools per Pool, 8 per Fleet. A Resource signs in through its owning Pool.
14. An Agent managing its own Fleet gets a template without sign-in and answer.
15. Two named exceptions to the preservation tests:
    - test 5: sign-in and answer are for Interfaces only;
    - test 2: root's defaults are revoked only by retiring the target.
16. The `fleet:` key works per file, with at most one per file.

Packages and machines:
17. Installing a release that adds a default to a published Type is rejected at install.
18. The core package is edited in place at 0.1.0, as ruling 41 did. This includes two changes to list-machines.v1 and place.v1 that are not additions, recorded as an exception. The alternative is `.v2`.
19. A node is bound to the machine provider's instance and the machine's `id` (O2). The alternative is the provider's Type. This goes beyond 52.4's words and amends PLC-026's provider-neutral sentence.

## The order of the edits

ORDER OF THE SPEC EDITS
Five spec orders. Each core contract is written by the order that writes its promises. So no file in spec/core or spec/schemas cites an ID that does not exist yet (check_schemas step 5 checks citations from spec/core, spec/schemas and manifest.yaml).

036a SPEC-A1, structure and access promises. It goes first because every area cites its names. Internal order:
1. definitions.md rows: Fleet and what it is not; default Conduit; local name and `as`; tombstone; lift; inventory; provenance; scope (C19; defined here because 037 and 038a both use it); frame and framed document; trust domain in the SPIFFE sense. The tombstone key is called `within`. Prose avoids "entry" and "grant".
2. Objects and kinds:
   - OBJ-001, 002, 013 and 020, each written once with `as`, `owner` and Fleet, and "framed";
   - FLEET-001..006;
   - CON-024, CON-027, CTR-004, TPL-001, TPL-002, PLC-006.
3. Access:
   - ACC-024;
   - ACC-033, written whole: rows 1A, 4A, 5, 6A, 7, 8A, 10 (lift), 13, 14, 14A and 15, the retired-Fleet sentences and the ACC-055 notes. Its disclosure sentence stays as it is today;
   - ACC-044 (provenance), ACC-053 (lift), ACC-054 (C1), ACC-055 (C2 and 66.4), ACC-057 (only root registers Agents);
   - ACC-005, 006, 012, 022, 029;
   - OPS-008, 014, 019, 029, and OPS-036 (list.v1);
   - REV-010, REV-014.
4. Lifecycle:
   - LIF-040 and LIF-041 (tombstones);
   - LIF-001, 009, the inventory half of LIF-013, LIF-020 and 021 generalized, 022, 023, 028, 034, 035, 037;
   - CON-008, REC-004;
   - FST-002, 003, 005, 007;
   - PRV-022, DL6, DL13.
5. Interface: IFC-012, IFC-014 (F3), IFC-025, IFC-033.
6. README contents and limits, including the named exceptions to preservation tests 2 and 5.

036b SPEC-A2, the core package and schemas for 036a, plus the staging (C25). It follows 036a because the contracts cite its promises.

037 SPEC-B, the short form. It comes after 036b and runs beside 038a and 039.

038a SPEC-C1: refusals, pending actions, sign-in, the state directory and the network principles. It comes after 036b and K1, runs beside 037, and comes before 040. Internal order:
1. Refusals first, because every later refusal cites them: ACC-056, IFC-050, and the sentences in OPS-005, OPS-023, OPS-029, ACC-001, IFC-012, IFC-016, IFC-025, IFC-032 and LIF-037.
2. The pending core: PND-001..004, PND-009, PND-012, IFC-049, plus IFC-007, 010, 016 and 045.
3. The kinds: sign-in (PND-005, 010, 011, CTR-013, PRV-029, MAN-018, PRV-023, LIF-035, DL16, DL18, names.yaml), setup (PND-006), notice (PND-007, CON-008), and expiry (DL19).
4. The contracts and schemas, and adding answer.v1 and sign-in.v1 to the capabilities and templates 036b wrote.
5. ACC-033's two new rows and its disclosure sentence.
6. ACC-056's row table, written last over every refusing rule in 036a and 038a.

038b SPEC-C2, enrollment. It comes after 038a and the owner's answer to R2, and is off the rerun path.

Why this order:
- 040 builds root's final tool lists once. It must wait for 038a, which writes answer.v1 and sign-in.v1. Those lists are frozen across releases under PKG-031, so they must be final before any engine reads them.
- 039 needs only 036b, so it runs beside 038a and the critical path loses nothing.

CONFLICTS RESOLVED
C1. IDs.
- **Batch-2 reservations.** The brief's reservations keep their meaning and are left for batch 2: IFC-048, OPS-035, ACC-045..052 and FLEET-007..010. The brief's LIF-038 and LIF-039 (the Fleet retire rules) are folded into LIF-020 and LIF-021 (C2) and stay unused. SCT-010 (the synthesis's root-Pool rule, which the brief dropped) stays unused. So no number changes meaning between the brief and the spec.
- **Retired ACC IDs.** ACC-040..043 are retired in access.yaml's head comment and are never reused; check_schemas step 7 would fail them. Batch 1's ACC IDs:
  - ACC-044 provenance (from the brief);
  - ACC-053 lift;
  - ACC-054 C1;
  - ACC-055 C2 and 66.4;
  - ACC-056 refusals;
  - ACC-057 only root registers Agents.
- **Other new IDs:**
  - LIF-040 (tombstone made) and LIF-041 (tombstone enforced);
  - IFC-049 (the URL-action relay) and IFC-050 (the refusal's form);
  - OPS-036 list.v1; FST-007 trust domain; CTR-013 sign-in hooks; PRV-029; PKG-031;
  - SCT-001..009 and SCT-011..016;
  - pending: PND-001..005 the core; PND-006 setup; PND-007 notice; PND-008 the enrollment kind (038b); PND-009 answer.v1; PND-010 sign-in.v1; PND-011 the account check; PND-012 records. The review takes the next free PND number in batch 2.
- Every new ID was checked as free against the retired registers.

C2. LIF-020 and LIF-021 are generalized to Fleets: one rule for Pools and Fleets.

C3. Batch boundary. The grant review, the list on close, the marks, keep.v1 and root-access's keep move to batch 2, along with ACC-050..052 and pending cases 5-14.
- The ground is ruling 61 B1: group B is batch 2, and B9 and B10 define the review's trigger and who keeps.
- Provenance (ACC-044) stays in batch 1.

C4. One sign-in flow (fixes M1, M2, m7, m9 and m16). This flow becomes PND-005, PND-010, PND-011, CTR-013, IFC-016, IFC-045 and IFC-049.
- **Declaring sign-in.** A Type states `signIn: {account: <pointer>}` and requires three core hooks (CTR-013): sign-in-hook.v1, account-hook.v1 and sign-out-hook.v1.
- **When the account check applies.** Only where the value at the pointer is a string. An API-key Agent writes `account: null`, and its `.v2` schema requires exactly one of `account` and `apiKey` to be set.
- **The elicitation capability.** The engine declares URL elicitation only toward implementations whose Type states `signIn` (IFC-045 amended). A plain OAuth server therefore answers exactly as it does today.
- **At set-up.** account-hook.v1 runs as a part of set-up.v1. If it reports no account, or another account, a sign-in action is raised, with an expiry. Until that action is answered, model work on the instance reports unavailable, naming the action.
- **A held request is final.** Nothing is queued, and nothing is dispatched later. The caller sends a new request.
- **An elicitation mid-call.** If a signIn implementation sends a URL elicitation during a call, that call ends unknown (IFC-010) and a sign-in action is raised. From any other implementation, the call ends unknown with a diagnostic saying the Type states no `signIn`, and nothing is raised.
- **sign-in.v1** is called by an Interface through the subject's Fleet, or through a Resource's owning Pool. It runs account-hook.v1 first:
  - if the expected account is already signed in, it succeeds and answers the action;
  - otherwise it runs sign-in-hook.v1 and relays one URL elicitation under IFC-049; on the client's retry it runs account-hook.v1 again;
  - if a different account signed in, sign-out-hook.v1 runs, the outcome is failed naming both accounts, and the action stays pending with the attempt as evidence;
  - if no retry arrives within the DL18 wait, it ends unknown, and no hook runs later.
- **A failed undo.** If the sign-out does not succeed, SignInUndone=False holds back all model work. Only a later account-hook.v1 part, inside sign-in.v1 or set-up, that reports the expected account or none clears it. A reconciliation never clears it.
- **No hidden steps.** Hooks run only as parts of sign-in.v1 or set-up, never on a notice from the implementation.

C5. One evidence schema, evidence-item.v1 `{kind, value, where, proves}`, with all four fields required (56.3). Values of kind `code` or `fingerprint`:
- appear in no tool result, `_meta`, notification, or record of a request;
- reach a person only in the URL elicitation's message to the Interface completing it, or in the engine's own panel (051c);
- are recorded with the answer (56.3).

PND-004 owns the form, and ENR-005 states the enrollment minimum.

C6. Kinds and expiry. The kinds are sign-in, setup and notice (038a), and enrollment, machine and machine-removal (038b). `kind` is a string with documented values.
- Every action has an expiry (51.5). The durations are implementation-defined (DL19), and vectors use the `clock` input.
- On expiry the answer is `expired` and nothing is dispatched. A sign-in is raised again at the next held request or set-up.

C7. The enrollment join action is never in a tool result, record or `_meta` (ENR-003).
- IFC-049 is written for every URL action at a person's Interface: sign-in.v1 now, enroll-machine.v1 in 038b.
- Its requestState is an opaque handle that the engine keeps: single use, bound to the session, and expiring with DL18.
- A command action is shown only in the engine's panel (051c).

C8. Operation lists, final before 040 (PKG-031):
- **pool-management.v1** gains list, sign-in and answer.
- **fleet-management.v1** lists register, inspect, list, set-up, retire, retire-with-agents, reconcile, sign-in and answer. It has no register-replacement.
- **Templates for Interfaces:**
  - pool-manage: list, inspect, set-up, reconcile, answer. No sign-in, because 66.3 limits scoped sign-ins to the Interface's own Fleets' Agents.
  - fleet-manage: list, inspect, set-up, reconcile, sign-in, answer.
  - pool-view and fleet-view: list, inspect.
- **fleet-manage-own**, for an Agent's own Fleet: list, inspect, set-up, reconcile. This fixes M3.
- **Root's defaults:**
  - root-pool: register, register-replacement, inspect, list, set-up, retire, reconcile, sign-in, answer;
  - root-fleet: register, inspect, list, set-up, retire, reconcile, sign-in, answer.
- **Root's first-start Conduits:**
  - root-management gains list and answer in 040, and enroll-machine, admit-machine and refuse-machine in 049a;
  - root-access gains lift.
- core#retire-pool and core#retire-fleet serve the one-time cascade Conduit.

C9. In language/first-start/protected, steps 2-4 become invalid for root under FST-006. The refused coverage moves to a new Interface that holds only `management`.
- In 040 it asserts the refused reason as today; 046 adds the tier-1 assertions.
- conformance/README is corrected.

C10. The snapshot (conformance/fixtures/root-system-state.json) gets one edit: the final lists for root-access-abc02 and root-management-abc01. Nothing else is added.
- conformance/README says the snapshot is a fixture state, not one the engine can reach.
- 040 adds list vectors from a real first start, and 042 adds inventory vectors from one.

C11. First start changes in 040, with the final lists (except the enrollment verbs), root's `system` default and trustDomain.
- 049a adds the enrollment verbs in a second, small pass of 5 cases and the snapshot.
- lift, answer and sign-in answer unavailable, "not built yet, order NNN", until 042, 047 and 048a.
- No stub remains at the rerun.

C12. One `.v2` schema:
- `accountServices`, `model`, `account` (required and nullable) and `apiKey` (a nullable secret reference), with exactly one of the last two set;
- `peerMessaging` on claude-code.v2 only;
- `signIn`, the three hooks and retire-hook.v1.

The fixture agents 0.2.0 (044) has no `signIn` and no hooks. The real packages come in 050.

C13. The cascade is one command, printed and counted under one flag: register, open, call, close, then root lifts its own tombstone. Only engine-made objects are exempt from tombstones (C27).

C14. 036a writes ACC-033 whole. 038a adds the sign-in.v1 and answer.v1 rows and replaces only the disclosure sentence with ACC-056's. It rewrites no other sentence of 036a's.

C15. inspect.v1 is edited by two orders:
- 036b adds the inventory, provenance, an Interface's or Agent's scope, and a Pool's administrators;
- 038a adds status.pending.

LIF-013 is split the same way.

C16. The amendments to rulings 8, 18, 31, 35 and 37 are recorded in IN_FORCE.md and the ledger (34), not in AUDIT.md.

C17. The coordinator updates IN_FORCE.md's markers at each spec merge (K4). C1 and C2, sign-in, pending actions, the trust domain and enrollment become batch 1. The review and keep stay batch 2.

C18. tombstone.v1 holds:
- the IDs and names of what it stood for: both ends for a Conduit;
- the owning Pool or Fleet at the time of closing;
- the provenance, `by`, and `within?`.

Who may see it (fixes m5 and m8):
- a retired instance's tombstone: whoever may inspect through its owning Pool or Fleet (by the kept ID), otherwise through `management`;
- a Conduit's tombstone: whoever may inspect either end as it stood, or through `management`.

It is shown in full to those callers, provenance and `by` included, and never counted. Lift coverage is decided against the kept IDs (59 A7). LIF-041's conflict diagnostic is tier 2 only for callers who may see the tombstone.

C19. "Scope" is defined in 036a: a holder's open Conduits that list inspect.v1, to Pools and Fleets other than `system`, whatever template made them. The holder is an Interface, or an Agent for its own Fleet.

C20. 66.1's "a Fleet, for Pools" waits for batch 2. An Agent's `manages` holds only its own Fleet, through fleet-manage-own.

C21. sign-in.v1 goes through the subject's Fleet for an Agent, and through its owning Pool for a Resource. This keeps one authority (fixes m16). answer.v1 goes through the target that authorizes inspecting the subject.

C22. ACC-055(b) covers Pools and Fleets only, as ruled. The `management` gap is logged.

C23. 037's examples land together with the routing fixes in tools/check_schemas.py and engine/crates/loader/tests/examples.rs.

C24. FLEET-005's clash rule rests on IFC-014 (F3) and on ACC-054's registration clause. Its diagnostic names root's Conduit only to callers who may inspect it.

C25. Staging. The engine reads spec/core and spec/schemas at run time, so a spec order lands only edits that keep the engine's tests green. Every other edit is staged verbatim in docs/work-orders/036b-*/staged/<order>.md:
- **039:** instance.v1's `as`; expectedId on open.v1 and close.v1.
- **040:** the root, root-access and root-management templates, and every document of the Fleet family or naming kind Fleet:
  - fleet.v1;
  - the fleet, root-fleet and retire-fleet templates;
  - fleet-reference;
  - the Fleet branches;
  - the kind enums;
  - the FAMILIES constant in tools/check_schemas.py (line 139);
  - the first-start example.
- **041a:** the Agent's required `owner`, and the project examples.
- **049a:** root-management's enrollment verbs.
- **049b:** list-machines.v1, place.v1 and machine-provider.v1.

036b and 038b apply all staged text in a scratch tree, run check_schemas and check_docs there, and report the result.

C26. Lift (fixes M4).
- lift.v1 takes a neutral `id` and removes the named tombstone.
- A tombstone the named one caused is removed only where the caller's Conduit to `access` would lift it on its own. The rest stay, and the result lists them, tiered per ACC-056.
- There is no root-only lift reservation (m13): ACC-054 already keeps opening and registering root's Conduits with root.

C27. Objects the engine makes (nodes, default Conduits, first-start objects) get no tombstone (m14). A re-admitted machine's node can therefore take its hostname again.

C28. ACC-057 (fixes m6): a registration or register-replacement whose submission holds an Agent is dispatched only where root sends it on its own behalf, in SEC-008's form.

C29. ACC-054 has a second clause, sibling to C1 (fixes m15): only root registers a Conduit whose consumer is root.

C30. File overlap. 037 and 038a both edit definitions.md, spec/README.md and manifest.yaml, each only its own rows and promises; the second to merge rebases. No promise is edited by two orders except:
- ACC-033 (C14);
- LIF-013 (C15);
- LIF-037 and IFC-012, where 036a writes the text and 038a appends sentences only.

CTR-013 names three hooks in every draft.

## The work orders

Numbering continues after 035. Every order carries the standing verifier.

Gate G for every order:
- the checks: check_docs, set_names --check, check_schemas, check_roadmap, `python3 -m unittest discover -s tools/tests`, and git diff --check;
- where Rust changes, in engine/: cargo fmt --check, cargo clippy --all-targets -- -D warnings, and cargo test --release;
- `cargo test --release` in experiments/011 whenever the core or loader crates change (039, 040, 041a, 041b, 042, 043a, 043b);
- the packages' own cargo test in 044 and 050.

Vector changes are reported through K2's delta classifier. Every flip in reason, outcome or accepted is listed and matched to the order's list. No expectation is regenerated from engine output. 033 runs first; 034 and 035 run in parallel.

COORDINATOR TASKS
- **K1, before 038a: the MCP citation check of URL-mode elicitation** (revision 2026-07-28). It covers:
  - input_required, inputRequests, requestState, and the retry with inputResponses;
  - the capability declaration;
  - user-audience annotations;
  - whether a URL elicitation's message is shown to the person and not to the model. If not, device user codes go only to the engine's panel.
- **K2, before 040: tools/vector_delta.py**, with tests. Per case and per step, it lists any flip and anything other than the shapes an order declares.
- **K3, before 052a: the CLI reference for plan and apply**, with golden cases that do not bind.
- **K4: IN_FORCE.md markers** at each spec merge.

SPEC
036a SPEC-A1: structure and access promises. Needs 032 (merged) and 034 (for ledger placement).
- New promises (16): FLEET-001..006, ACC-044, ACC-053, ACC-054, ACC-055, ACC-057, LIF-040, LIF-041, OPS-036, FST-007.
- About 55 amended (see order).
- Also: definitions.md rows, and README limits including the named exceptions to tests 2 and 5.
- About 1.6-2k lines.
- Checks: G, with engine/ untouched and green.
- Two verifiers: Fleet and access; tombstones, lift and inventory.

036b SPEC-A2: core package and schemas for 036a. After 036a.
- New: fleet.v1, fleet-management.v1, list.v1, retire-with-agents.v1, lift.v1 (input `id`).
- The ten templates of C8, without answer and sign-in.
- Schemas: inventory (with scope and administrators), tombstone (C18), provenance, fleet-reference, trust-domain.
- instance.v1, template.v1 and type-spec.v1: the Fleet, `as` and owner branches.
- expectedId; the first-start example; about 10 contracts changed.
- Staging per C25, with the scratch-tree check.
- About 1.5-1.8k lines.
- Checks: G. Verifier: core contracts against all area plans.

037 SPEC-B: the short form. After 036b; parallel with 038a and 039.
- New: SCT-001..009, SCT-011..016, PKG-031.
- Amended: NAM-024 and about 17 more: OBJ-012, 017, 019, 022, 024; INS-001; MAN-003, 007, 011; SUB-004, 005, 010; SCH-018; NAM-027; PKG-018.
- sectioned.yaml; flow-style examples (valid/sectioned-project, valid/sectioned-package, about 12 invalid); the schemas/README row; the routing fixes (C23).
- The `fleet:` key works per file (A6). A file holds at most one. It owns every Agent in the file, and an Agent there naming another owner is invalid.
- SCT-012 expands `manages:` by consumer kind (C8, C20).
- README: the reading path and its limits, plan and apply as tooling, and the counted flags.
- About 1.5-2k lines. Checks: G.

038a SPEC-C1: refusals, pending actions, sign-in. After 036b and K1; parallel with 037 and 039; before 040.
- New (15): ACC-056, IFC-049, IFC-050, PND-001..007, PND-009..012, CTR-013, PRV-029.
- About 45 amended.
- DL16, DL18, DL19; names.yaml `stateDirVariable`, plus tools/set_names.py.
- Core: answer.v1, sign-in.v1, sign-in-hook.v1, account-hook.v1, sign-out-hook.v1, pending-action.v1, evidence-item.v1, refusal.v1, and inspect.v1's status.pending. answer and sign-in are added to the C8 lists.
- ACC-033's two rows and its sentence (C14).
- C4 to C7 are written as stated.
- README limits:
  - ruling 51's decisions 1, 4, 6, 7, 9 and 10 and the network principle;
  - peer mode, named as pending R3;
  - A5 not gating the rerun.
- About 1.8-2.2k lines.
- Checks: G. Verifier: the tiers across every refusing rule, and sign-in against M1, M2, m7 and m9.

038b SPEC-C2: enrollment. After 038a and the owner's answer to R2. Off the rerun path.
- New: ENR-001..012, PND-008.
- PLC-025, PLC-026 (a node is bound to the provider instance and its machine `id`; reading O2), the PRV and FST edits, ACC-032 (the enrollment verbs reserved to root), DL5.
- The six enrollment operations.
- list-machines.v1 and place.v1 edited in place, with the exception recorded in spec/core/README.md (reading).
- ENR-002: the `id` must be derived from the machine's key where the provider offers one.
- ENR-005: at least one item comes from the key or attestation, and `where` is required.
- ENR-003 and ENR-004 per C5 and C7. R2's answer is applied.
- Staged edits for 049a and 049b.
- About 1-1.2k lines. Checks: G plus the scratch tree.

CORE LANE (serial: the orders share registration.rs, access.rs, requests.rs and the runner's lib.rs)
039 E1: `as`, F3, expectedId, descriptions, C1. After 033 and 036b.
- Build:
  - registration.rs: local names and IFC-014's uniqueness;
  - requests.rs: expectedId through LIF-037's path; ACC-054's sender check on open and on register;
  - schemas.rs: IFC-033;
  - lands its staged edits.
- Vectors, 12 new: conduit-as, conduit-as-invalid, as-unique, as-declared, expected-id-open, expected-id-close, expected-id-access-order, e05-description-target, e05-description-bare, root-consumer-open, root-consumer-onward, root-consumer-register.
- Re-expected: the 4 collision cases, tool-collision, e05-local.
- Experiment 011's test is updated and run.
- About 0.45-0.7k lines.

040 E2: the Fleet family, first start, list.v1, stubs. After 039, 038a and K2.
- Build:
  - Fleet added to every family list: the loader's FAMILIES; tools/check_schemas.py; shell lib.rs:285, main.rs:340, server.rs:804, identity.rs:226, author.rs:82 and 328; core schemas.rs:722, requests.rs:115-119, registration.rs:585-589, packages.rs:648-671 (all under engine/ after 033);
  - FLEET-001 and FLEET-006;
  - first start: root's `system` default, from root-pool; the final root lists (C11); trustDomain validated and recorded;
  - the list.v1 handler: may-inspect filter, no counts, `kind`;
  - stubs for lift, answer and sign-in;
  - `conduit init` makes or takes the trust domain; the shell tests' root lists;
  - lands the staged Fleet documents.
- Vectors, about 10 new: fleet-instance, fleet-names, fleet-consumer, first-start root-system, trust-domain, list management, kind-filter, refused-without-list, list-from-first-start.
- Changed once:
  - first-start initial, repeat and lifecycle;
  - protected (C9);
  - s1/whole step 0;
  - case.schema.json;
  - the snapshot (C10);
  - access/fixed-kind-references step 2.
- About 0.9-1.3k lines.

041a E3a: defaults, Fleet s1, owners, and the migration. After 040.
- Build:
  - FLEET-005: defaults for every registered Pool and Fleet. Their objects and records are allocated after every object and record of the submission;
  - FLEET-002, 003, 004: the Agent's owner is required;
  - ACC-057;
  - the runner, as needed.
- Migration:
  - a scripted transform (tools/, with a unittest) that inserts only known shapes: Fleet s1 (last in the fixture), the owners, the defaults' objects and records, renumbered register-step IDs, and grown evidence lists;
  - about 170 case files: 109 S1 registrations, 15 cases with their own Pools, 43 with their own Agents, and first start;
  - K2's report must show only those shapes.
- Experiment 011 is updated.
- Vectors, about 8 new: defaults pool, defaults fleet, close-refused, root-closes-own, agent-owner, agent-owner-retired, register-agent-non-root, defaults-ids-last.
- About 0.5-0.8k lines of Rust, plus about 0.3k for the transform.

041b E3b: authority through Fleets. After 041a.
- Build:
  - ACC-033 rows 1A, 4A, 6A, 8A and 14A; Agents leave rows 5, 7 and 14; the retired-Fleet fallbacks;
  - LIF-020 and LIF-021 generalized; retire-with-agents.v1;
  - CON-024, REV-010, REV-014;
  - the shell's setup loop: Fleet before Agent, and Agents through root's Fleet default.
- Shell-test fallout: tests/changes.rs 118, 168, 375, 797 and 823; tests/serve.rs 260-261; tests/agents.rs 348; src/server_tests.rs 1211.
- 41 set-up requests in 22 directories; changes/t4 step 2; t5 step 5; engine/examples/s1. root-workspace is kept.
- Vectors, about 14 new: fleets routing, routing-retired, ownership-permits-nothing, retire-conflict, retire-with-agents and its partial and authority cases, setup-registered, list pool, list fleet, partial-authority-no-count, bound-fleet-target, manages-agent-own-fleet (canonical form: 4 tools, no sign-in or answer).
- Outcome changes: access/routing step 7, and the setup sender cases.
- About 0.6-0.9k lines.

042 E4: tombstones, lift, inventory, provenance, reconcile limits. After 041b.
- Build:
  - tombstones per C18, C26 and C27, kept in a separate map: on close, on retirement and in close_ends;
  - lift.v1 replaces its stub;
  - LIF-041 at every instantiation, register-replacement, open and setup part;
  - the inventory, with scope and administrators;
  - ACC-055, with the sender read from invocation["sender"] (hooks inherit it);
  - ACC-044 capture;
  - list.v1 shows tombstones by who may see them;
  - the runner's projections;
  - `conduit inspect` reads the inventory.
- Vectors, about 36 new:
  - additions 5, 9-24 and 29-34;
  - provenance: open, setup parts, onward, engine;
  - closed-with-target, bound-kept-id, inventory-reading, inventory-from-first-start, lift-children-uncovered, tombstone-visibility, node-readmit-no-tombstone, inventory-scope, pool-administrators.
- Lift insertions: restore 14, restore-current-template 12, t4 10 and 15, s1/whole 50, root-closes-own.
- About 1-1.5k lines.

046 E5: refusals and explicitness. After 042 and 038a.
- Build:
  - access.rs: what was named; the tier decision; the tier-2 list; closed Conduits from tombstones;
  - a diagnostic on every reason and on engine-decided outcomes, plus the text block;
  - records: rule and tier;
  - server.rs: the -32602 message and data;
  - the CLI prints tier 2 to root;
  - the runner: the new assertions, including protected's tier 1.
- Vectors: 14 new; about 45 cases (about 85 steps) changed.
- About 0.6-0.75k lines.

047 E6: the pending-action core. After 046.
- Build:
  - pending.rs: the model; routing by scope; disclosure;
  - answer.v1 replaces its stub, with answers limited by holdings;
  - the setup kind (computed) and the notice kind (62 C5: raised when a sender other than root retires a Resource that another Fleet's Agents hold Conduits to);
  - expiry for every kind, through `clock`;
  - the `unanswered` and `pendingAction` keys; the list and inspect projections.
- Vectors, about 14: raised-scoped, setup-failed, setup-retried, setup-unknown-reconciled, notice-other-fleet, notice-none, answer-errors, answer-limited-by-holdings, unanswered-hint, expiry-setup, expiry-notice, expiry-answered-dispatches-nothing.
- About 1-1.5k lines.

048a E7a: sign-in in the core. After 047.
- Build:
  - C4 in full: IFC-016 detection only for signIn Types; PND-005; sign-in.v1 with account-hook first; CTR-013 in the loader and core;
  - SignInUndone, cleared only by a check;
  - held requests are final;
  - evidence handled per C5;
  - a scripted signIn fixture Type with the three hooks.
- Vectors, about 18: elicitation-from-agent, elicitation-from-root-direct, form-mode, sign-in-agent, sign-in-resource-through-pool, scoped-own-fleet-only, wrong-account, declined, no-capability, turn-held-final, void, non-https, case-insensitive, apikey-no-check, plain-oauth-unchanged, sign-out-unknown, undone-cleared-by-check, late-completion, expiry-sign-in, user-code-only-in-elicitation.
- About 0.8-1.1k lines.

048b E7b: the shell relay. After 048a.
- Build:
  - server.rs: input_required with the opaque requestState handle (C7), and the retry;
  - runner.rs: declares URL elicitation only toward signIn Types; the multiple round trips toward implementations (rmcp 3.9.0's InputRequiredResult and UrlElicitationParams);
  - shell tests.
- About 0.5-0.8k lines.

049a E8a: enrollment in the core. After 048b and 038b. Off the rerun path; runs beside 050.
- Build:
  - machines.rs; the MachineReport input; provider-call intents;
  - the three verbs; the PND-008 kinds;
  - evidence values only through the panel channel;
  - lands root-management's staged verbs: the second first-start pass, 5 cases and the snapshot, by the transform.
- Vectors: about 20.
- About 1.2-1.5k lines.

049b E8b: the provider fixture. After 049a.
- The fixture provider package and its snapshot; the staged machine-provider edits; CONFORMANCE.md DL5, DL6, DL13.
- Vectors: about 8, plus validate-values.
- About 0.6-0.9k lines.

OTHER LANES
043a L1a: the translator core. After 037 and 040; merges after 041a.
- Build:
  - loader: sectioned.rs, the routing in project.rs, the mapping and redaction in diagnostic.rs;
  - conformance: the `translate` step and the `equivalent` option; case.schema.json.
- Vectors: about 35 (SCT-001..009).
- About 1-1.5k lines.

043b L1b: the Fleet key, `manages` and templates. After 043a, 041b and 044.
- SCT-011..016; s1-promises.yaml; the manifest.
- Vectors: about 35, including fleet-key-two-in-file and sectioned manages-agent-own-fleet.
- About 1-1.5k lines.

044 P1: package fixtures and the NAM-024 check. After 037; parallel with 039-042.
- packages.rs and archives.rs: the comparison at install.
- filesystem 0.1.1 and 0.2.1, with `read` and `read-write`, and their digests.
- agents 0.2.0, with the `.v2` Types per C12 and `turn`.
- Vectors: release-adds-default-rejected, release-adds-optional-accepted, release-new-version-segment-with-default-accepted, agents-v2-defaults. changes/t5 is confirmed unchanged.
- About 0.3-0.5k lines.

045 S1: the state directory. After 038a.
- Created with mode 0700 and keyed by instance ID; CONDUIT_STATE_DIR; removed on removal or unregister, with a failed delete retried; the MAN-018 check.
- Vectors: state-dir-binding, plus shell tests.
- About 0.3k lines.

050 P2: the Agent packages 0.2.0. After 044, 045 and 048b.
- The `.v2` Types (exactly one of account and apiKey set); each Agent's home; the login flows; the three hooks plus the sign-out at retire; `turn`.
- Scripted client tests; READMEs; archives pinned by digest; CONFORMANCE.md; docs/guides/accounts.md.
- Then the coordinator's real-client trials, including the Codex account read (O3).
- About 1.5k lines.

051a C1a: shell commands. After 042.
- `conduit list`; `fleet list`, `fleet inspect` and `fleet retire --with-agents`; `retire <pool> --with-resources` (C13).
- `conduit inspect` renders an Interface's scope and a Pool's administrators.
- The SPIFFE engine-id and the token `sub` (tests.rs 819-872).
- About 0.5-0.7k lines.

051b C1b: the shell's pending-action commands. After 047 and 048b.
- `conduit pending`, `sign-in` and `answer`.
- Client URL elicitation: show the URL and the message, ask consent, open the browser.
- The unanswered hint.
- Optional and off the rerun path: client tokens for Interfaces other than root (DL13).
- About 0.4-0.6k lines.

051c C1c: the panel and machine commands. After 049a; off the rerun path.
- The engine's own panel rendering of evidence, over a root-only channel that no MCP tool exposes.
- `machine enroll`, `admit` and `refuse`, with `--accept-weaker-check` counted, and the fingerprint argument if R2 is answered (a).
- About 0.3-0.5k lines.

052a C2a: plan and apply. After K3, 043b, 042, 039 and 051a.
- Plan and apply are composed of requests, with expectedId on every open and close, and drift read from the inventory.
- `--reopen`, `--recreate`, `--prune`, `--replace` and `--adopt`, each counted and recorded. Narrowing never waits.
- The plan is printed and recorded (55 A4). By default its text is kept beside the file, and its digest is part of every request's idempotency key, so each record links to it. K3 may decide otherwise.

052b C2b: replacement and authoring commands. After 052a.
- Stable-name replacement: close or retire, lift (no flag for tombstones made in the same apply), register, open.
- The plan prints every bound that stops covering the Resource (59 A7). An owner change is shown as a replacement.
- `install` writes pins; `render`; `init` writes a flow-style skeleton; diagnostics as file:line:col with the SCT origin.

053 R: rerun preparation. Last.
- engine/examples/s1 in the short form; docs/guides in flow style; docs/guides/shared-pools.md (62 C5's guidance).
- The measurement plan: 63 #1-#3; 52.6's separate lists; 55 A3's windows without access, measured; R1's answer, or 63.1 as worded until it comes.
- The fresh author's pack (53 #9); experiment 011's dated follow-up; the IN_FORCE markers.
- Gate: no "not built" stub remains (grep), and G is green.

CRITICAL PATH
036a → 036b → 038a → 040 → 041a → 041b → 042 → 046 → 047 → 048a → 048b → 050 → 053.
- 039 runs beside 038a, between 036b and 040.
- Joining before 053: 037 → 043a → 043b → 052a → 052b; 044; 045; 051a; 051b.
- Off the rerun path, before S3: 038b → 049a → 049b → 051c (after R2), and the optional DL13 tokens.
- Hot files: conformance/manifest.yaml, case.schema.json, the runner's lib.rs, registration.rs and requests.rs. Orders merge in critical-path order, and parallel orders rebase. 043a and 043b stay branched until 041b and 044 merge.

## Findings

RULED POINTS THAT CANNOT BE BUILT AS WORDED (each handled by a reading or a deferral; none blocks the batch)
1. **Ruling 66.1 and 66.2: a Fleet writing `manages:`, and managing "acting for its Fleets".** Both need a Fleet consumer and act.v1, which arrive in batch 2 (CON-027 rejects a Fleet consumer today). In batch 2, act gets its own template rather than joining fleet-manage, which PKG-031 freezes.
2. **Ruling 66.3: "closing a grant always".** Deferred with the review (C3).
3. **Ruling 49.3: "checked" sign-ins.**
   - A plain MCP server cannot report who signed in, so only Types that state `signIn` get sign-ins. URL elicitation is declared only toward those Types, so plain OAuth servers behave exactly as today.
   - Codex's account read waits on the trial (O3).
4. **Ruling 49: "turns are refused".** Built as the reason unavailable, naming the action, and final. In the spec, "refused" means not permitted.
5. **Ruling 49.3: a wrong sign-in is "refused and undone".** The sign-in is undone; sign-in.v1 ends failed, naming both accounts; the action stays pending.
6. **Ruling 52.1's single cause sentence.** It is false for SEC-007, ACC-032 and others, so each rule states its own true cause.
7. **Ruling 51's peer caveat and 62 C4.** The peer needs an Interface other than root signed in at the peer (DL13 tokens, optional here). Appointment, keep and withhold, which C4 also names, exist only in batch 2. Whether the admitted Interface counts as root is unruled. This goes to the owner as R3. The README states peer mode as pending.

GAPS
- **Plan and apply** is on the rerun path, and its CLI reference (K3) must come before 052a.
- **The measurement plan** must settle R1, or use 63.1 as worded.
- **K1** must pass before 038a. If URL-mode messages can reach the model, device user codes go only to the engine's panel.
- **Scoped Interfaces** and pending-action routing are proven by vectors only, until the optional DL13 tokens exist.
- **The ACC-055(b) gap:** a holder other than root whose Conduit to `management` lists reconcile.v1 can reconcile any invocation. This is rare in v0 and logged for batch 2.
- **Lift is required for stable-name replacement.** Under A3 and A7 every such replacement needs a holder of lift.v1, so a delegate without it cannot do one.
- **Stubs.** Between 040 and 048a, lift, answer and sign-in answer "not built yet". 053's gate checks that none remain.
- **Root's tool count** grows by the defaults (9 tools per Pool, 8 per Fleet), and later by 3 enrollment tools. The measurement plan lists them apart (52.6).
- **The snapshot** stays a fixture state the engine cannot reach (C10). Vectors from a real first start cover lists and inventories.
- **Limit of 56.4:** a model with shell access as the person's user can run the CLI panel. This is stated as a limit.
- **Estimates** run 1.5 to 2 times low, and plan and apply is unestimated.

WHAT CHANGED FROM THE REVIEWED PLAN (each skeptic finding and its fix)
Fidelity:
- **1:** C4 goes to R3, and 038a writes the README limits for ruling 51.
- **2:** C5 and C7 keep code and fingerprint values out of tool results.
- **3:** ACC-057.
- **4:** C6 gives every action an expiry.
- **5:** O2 becomes a strikeable reading, not a consequence of 52.4.
- **6, 7:** `where` required; ENR-002 strengthened.
- **8:** C21 routes a Resource's sign-in through its owning Pool.
- **9:** scope rendering in 042 and 051a.
- **10:** shared-Pools guidance in 053, and the administrators vector.
- **11:** the plan is recorded (052a).
- **12:** the measurement plan's added items.
- **13:** IFC-049 is generalized.
- **14:** R1 is not applied.
- **15:** the `fleet:` key works per file.
- **16:** the 61 B1 ground.
- **17:** the wrong-account reading is listed.

Safety:
- **M1:** C4 makes held requests final, with hooks only as parts.
- **M2:** IFC-045 declares elicitation only to signIn Types; the outcome stays unknown.
- **M3:** fleet-manage-own.
- **M4:** C26.
- **m5 and m8:** C18.
- **m6:** ACC-057.
- **m7:** clearing by a check.
- **m9:** the check needs a string account.
- **m10:** C7.
- **m11:** a reading.
- **m12 and m13:** named exceptions; no root-only lift.
- **m14:** C27.
- **m15:** C29 and C24.
- **m16:** C21.
- **m17:** C5.
- **m18:** C14 and C30; three hooks.

Feasibility:
- **1:** the C1 renumbering.
- **2:** C11, 041a's transform, K2.
- **3:** 036 split; contracts with their promises; staging per C25 with the scratch tree.
- **4:** 040 and 041b name every file; experiment 011 is in G.
- **5:** splits, and enrollment off the rerun path.
- **6:** 051b after 048b; C9's tier 1 in 046; 049 serial after 048; hot files.
- **7:** M3.
- **8:** G completed; C10's vectors.

COORDINATOR CHOICES APPLIED (logged in to-review-with-the-owner.md, each strikeable)
- **Integration:** C1 to C30, including:
  - the new templates fleet-manage-own, retire-pool and retire-fleet;
  - sign-in-hook.v1;
  - DL19;
  - the opaque requestState;
  - the transform and K2.
- **Additions:**
  - F3: local names unique per consumer;
  - a tombstone hit reports conflict;
  - no purge touches a tombstone;
  - "sent" is OPS-008's sender, and a hook counts as sent with its setup;
  - C2 means coverage of the reconciled request;
  - `within`;
  - register-replacement never lifts.
- **Fleet:**
  - "may inspect" means through any Conduit the caller holds;
  - register-replacement is not in fleet-management.v1;
  - no cap on name length;
  - root-workspace is kept;
  - the defaults' objects and records are allocated last.
- **Short form:** D1-D14, among them:
  - `manages` is a map of name to role, with no default role;
  - `operator` on a Pool is rejected;
  - NAM-024 is enforced at install;
  - new fixture versions rather than re-pinned ones.
- **Pending actions:**
  - the setup kind;
  - sign-in and answer for Interfaces only;
  - C5's sender read literally;
  - an action's ID is that of its invocation or subject.
- **Refusals:** R1-R3 and R5-R10.
  - tier 2 needs inspect authority over every object it names;
  - outcomes are explicit too;
  - accounts are compared ignoring ASCII case;
  - codex.v2 drops peerMessaging;
  - the name CONDUIT_STATE_DIR.
- **Enrollment:** C1-C8 of that area, and O2.

NOT FORECLOSED FOR BATCH 2 (ruling 61, and 62 C4)
- LIF-041 conditions every instantiation, and ACC-054 and ACC-057 use SEC-008's form, so make.v1 adds only "or make".
- lift.v1 takes a neutral `id`, and tombstone.v1 has `within?`.
- ACC-055 already says a Fleet "or its Agents".
- The reservations stay free: IFC-048, OPS-035, ACC-045..052, FLEET-007..010, and the next PND number.
- FLEET-004 says only "never a caller".
- The defaults list their operations explicitly, so act.v1 never joins them.
- `fleets:` and `templates.fleets` stay free.
- Kinds are open strings.
- Recipients of defaults and pending actions are computed by a rule that today names only root, so R3 (a) or (b) fits.
- DL13's single root of trust is scoped "in this version".
- provenance.v1 can gain `actingFor`.
