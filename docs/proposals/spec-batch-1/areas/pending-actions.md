# Spec batch 1, area: pending-actions

Not binding. An area planner's draft for [the batch 1 plan](../README.md); where the plan differs, the plan governs.

## Scope

The generic pending action and ruling 54's revocation machinery. This covers one mechanism for every action that waits on a person, as rulings 50 (point 3), 51 (decision 5) and 56 (decisions 3, 4, 7 and 8) give it: kind, subject, evidence, expiry and answer. It has five kinds in batch 1: sign-in, enrollment (shape only), the ruling 54 review, setup, and the ruling 62 C5 notice. Each action is routed to root and to scoped Interfaces, and what each Interface can answer is limited by what it holds (ruling 66, decision 3). URL elicitation goes to a person's Interface and never to an Agent, which changes IFC-007, IFC-010, IFC-016, IFC-025 and IFC-045 (ruling 49). Ruling 54 contributes provenance, the list of grants on close, computed marks, the review, and keep.v1. Ruling 61 (B9, B10, B11) is not planned here and is kept open (see ruling_points). Promise IDs follow docs/proposals/projects/brief.md where it gave them: ACC-044, ACC-050, ACC-051 and ACC-052, which the brief checked as free. New PND- IDs go in a new topic file. The coordinator assigns final IDs across areas.

## Promise and core changes

A. NEW TOPIC FILE spec/promises/pending.yaml (prefix PND; layers convention and interface; spec/README.md contents table and status updated). Drafts:

PND-001 (convention, must), what a pending action is. When the engine holds something that waits on a person (a sign-in, a machine's admission, the review of grants under a closed authority, a setup that did not succeed, or a notice of Conduits closed by another's retirement), it is a pending action. A pending action has:
- an ID;
- a `kind`;
- a `subject`: the objects it is about, each in resolved form {kind, id, name};
- `evidence` (PND-004);
- an `expiry`, or none;
- once answered, an `answer`: its value, the actor that gave it, the invocation and the time. A review also has `items`, each answered on its own.
Observable:
- The form is the core schema pending-action.v1.
- A review's ID is its closed authority's ID, and a setup action's ID is its setup invocation's ID. Others get a new engine ID. This keeps computed kinds free of allocations, so vectors stay deterministic.
- An answer is never an outcome (OPS-001; ruling 56 point 8).
- An instance has at most one unanswered sign-in and one setup action at a time.
- A pending action permits nothing: being raised at an Interface gives it no access.
- `kind` and answer values are strings whose v0 values are listed. Later kinds (removal, introduction, approval) are added without a .v2.

PND-002 (convention, must), where it is raised. An unanswered action is raised at:
- the root Interface;
- on a peer engine, the Interface that the root template names (ruling 62 C4);
- every Interface whose scope contains one of its subjects (ruling 66 decision 3). The scope is computed from Conduits alone. Recommended reading: the Interface is the consumer of an open Conduit, to a Pool or a Fleet other than `system`, that lists inspect.v1, and that Pool or Fleet is the subject, owns it, or owns an end of it where the subject is a Conduit.
An action is never raised at an Agent. "Raised" means:
- it is listed in the `pending` output of list.v1 and in inspect.v1's `status.pending` for its subject, where the caller is such an Interface;
- every result the engine gives such an Interface's own request carries `io.github.shrav-k.conduit/unanswered`, the number raised at it, when that number is not zero.
Disclosure: an Interface other than root is shown only the subjects and evidence about objects it may inspect, with no count of the rest (ruling 53). Root is shown everything (ruling 52, tier 2). Agents' list and inspect results carry no pending actions.

PND-003 (convention, must), answers and expiry. An answer is the effect of an operation, decided like any request under the access convention by the Conduits the actor holds:
- sign-in: sign-in.v1, which answers `signed-in`;
- enrollment: admit-machine.v1 or refuse-machine.v1, root alone (ruling 56 decision 11), which answer `admitted` or `refused`;
- review: each item answered `closed`, however the grant closes (close.v1, CON-008 or retirement), or `kept` by keep.v1;
- setup: `retried` (a further set-up.v1 is dispatched), `ready` (reconciled as having occurred), `retired`, or `dismissed` by answer.v1;
- notice: `acknowledged` by answer.v1.
Anything that touches a secret, and machine enrollment, stays root's (ruling 66 decision 3). A sign-in whose instance is retired reads `void`. An action whose expiry has passed reads as answered `expired` by no actor. Nothing is dispatched because of an expiry (ACC-019, OBS-004). An answer to an action that is already answered, expired or void reports conflict. Review, setup, notice and sign-in have no expiry: ruling 54 says the review holds "until each is closed or kept", and B9 agrees. Enrollment's expiry is its enrollment's.

PND-004 (convention and interface, must), evidence and prompts.
- Evidence is a list of items {kind: code | fingerprint | attestation | fact, value, where?, proves} (ruling 56 decision 3). The engine writes it from what it observed, or from what an implementation or provider reported through a call the engine made, never from a model's turn.
- Evidence goes only in results to Interfaces at which the action is raised: in structuredContent, and in a content block annotated with audience user. It is recorded with the answer.
- A URL that a person is to open is a prompt, not evidence. A prompt goes only by URL-mode elicitation under IFC-049: never in content or structuredContent, never to an Agent or a Resource, never recorded, and held only until its own expiry or the action's answer (ruling 49 point 1; ruling 52 point 4).
- An enrollment action in command form is finding F3.

PND-005 (convention, must), the sign-in kind. It is raised when:
- (a) an implementation answers a call with a URL-mode elicitation that the engine does not pass on (IFC-016); the subject is the call's instance, and the evidence names the request that hit it; or
- (b) an instance's implementation reports, as its Type's contract states, that its sign-in is missing or lapsed; the subject is that instance.
Evidence: the account expected, from configuration (ruling 49 point 3); the method; for a device flow, the user code (ruling 56 decision 7); and each attempt with its result. Answers: `signed-in`, or `void`. While a sign-in is pending for an Agent, a request for an operation of it that starts model work is not dispatched. It reports unavailable (CON-024 and OPS-023's last place), with the action's ID in `pendingAction` (finding F2).

PND-006 (convention, must), the review kind (ruling 54 point 3). There is one review per closed Conduit to `access` while any open Conduit is marked under ACC-052 and that Conduit was the last of its authorities to close.
- Subject: the closed authority, read from its tombstone, and each item.
- Evidence facts: the authority (its consumer, target, bound, how it closed, by whom, and the invocation); each item (consumer, target, operations and provenance); and, for an item that is itself a Conduit to `access`, the grants open through it. Those are informational and are not items.
- It is raised at root and at Interfaces whose scope contains an item's consumer or target.
- It is computed from state and adds no record. Its history is the records of the close, the keeps and the closes.

PND-007 (convention, must), the setup kind (finding F5). There is one action per instance whose latest set-up.v1 invocation has outcome failed or unknown and that is not retired.
- Subject: the instance.
- Evidence: the invocation, its outcome, its reason and diagnostics, the parts that did not succeed, and the unreconciled IDs.
- Answers: retried, ready, retired or dismissed.
- It is computed and adds no record.

PND-008 (convention, must), the notice kind (ruling 62 C5). It is raised when the retirement of a Resource, sent by an instance other than root, closes under CON-008 a Conduit whose consumer is an Agent owned by a Fleet other than the one that owns the sender. Where the sender is not an Agent, every Fleet counts as other. Batch 2 extends "the sender's Fleet".
- There is one notice per such Fleet. Its subject is the Resource and that Fleet's closed Conduits.
- Evidence: the Resource, its Pool, the sender, the caller, the invocation and the closed Conduits.
- It is raised at root and at Interfaces whose scope contains that Fleet. It is answered `acknowledged`.
- The retirement's result carries each notice's ID in `pendingAction`.
- It is stored, with an engine ID allocated within the retirement.

PND-009 (convention, must), the enrollment kind: shape only. It is raised at root, and at the C4 Interface, alone. It is answered by admit or refuse. Its expiry is the enrollment's. Its evidence follows the enrollment promises (the provider declares the method, and the floor applies). The enrollment area may add a `removal` kind (enrollment decision 6), answered through answer.v1 by root.

PND-010 (convention, must), answer.v1 {action, answer}; no outputs.
- It is dispatched only where an Interface asks on its own behalf, through an open Conduit listing it, to the target that authorizes inspecting the action's subject: the owning Pool for a Resource, the owning Fleet for an Agent or for a notice's Fleet, and `management` otherwise.
- No Conduit whose consumer is not an Interface lists it. Such an open reports refused, as ACC-032 has it for reserved operations (finding F6).
- An answer value that the kind does not take reports invalid, with a diagnostic naming the operation that answers that kind (close.v1 or keep.v1 for a review; sign-in.v1; admit-machine.v1).
- An action not raised at the caller reports refused before invalid (OPS-023), so the caller learns nothing of whether it exists.

PND-011 (interface and convention, must), sign-in.v1 {instance, expectedId?} → {account}. It is implemented by the engine.
- It is dispatched only where an Interface asks on its own behalf. It is authorized against the owning Fleet for an Agent; for a Resource it is authorized only for the root Interface, through `management`.
- The engine gets the sign-in's URL elicitation in one of two ways. For an Agent, it calls the operation that the instance's Type declares for signing in (a part). For a sign-in raised by an implementation's elicitation, it uses the prompt it holds.
- It passes the elicitation under IFC-049. Once the person has accepted and the sign-in has completed, it asks who signed in, as the Type's contract states.
- The expected account gives succeeded, and the action is answered `signed-in`. Another account is undone by a sign-out part, the outcome is failed, the result names the expected account and the one seen, and the action stays pending with the attempt.
- Declined or cancelled gives failed.
- A caller whose request did not declare URL-mode elicitation reports invalid.
- It starts no model work, and no record holds the URL.

PND-012 (convention, must), records. A request that raises or answers an action has a record naming the action's ID. The record of an answer holds the kind and the evidence as shown. No record holds a prompt. Raising adds no record type of its own: the computed kinds add none.

B. NEW ACCESS PROMISES (spec/conventions/access.yaml)

ACC-044 (convention, must), provenance. When a Conduit is opened, it holds its provenance from then on, and in its tombstone after it is removed.
- For a Conduit a request opened, provenance is `actor` (the caller), `sentBy` (the sending instance), `invocation` (that of open.v1, or of the setup whose part opened it) and `openedThrough`. `openedThrough` is the Conduit through which open.v1 was dispatched. For a part under ACC-005, it is every open Conduit to `access` of the sender that would have permitted the part on its own when it was decided, in ID order (H1 F16).
- For a Conduit the engine opened, provenance is `cause`: first-start, or default (ruling 59 A3).
- Reconciling an open as having occurred gives the provenance of that invocation. Nothing changes provenance afterwards.
Observable:
- inspect.v1 and inventory entries show it.
- Purge does not touch it: it is state, and REC-004 is amended.
- No record field is added.
- Snapshots carry none.
- A Conduit with no `openedThrough` is never under a closed authority.
- Schema provenance.v1. Batch 2 adds `actingFor` as an optional property under NAM-024.

ACC-050 (convention, must), closing an authority. When a Conduit to `access` closes, however it closes (close.v1, retirement, CON-008, declare-lost or unregister), every grant still open that names it in `openedThrough` is listed in `io.github.shrav-k.conduit/grantedUnder` of the result of the invocation within which it closed. The key is present only when the list is not empty. Each entry gives consumer, target and operations, and any other open authorities that still cover it.
- None is closed (ACC-030 stands).
- A caller other than root sees only the grants it may inspect, and no count.
- Grants that this close leaves marked become items of the review PND-006.
- An open dispatched before the close and ending after it, or reconciled as having occurred later, is marked at once and joins the review.

ACC-051 (convention, must), keep.v1 {conduit, expectedId?}.
- It is dispatched only if the actor consumes an open Conduit to `access` that lists keep.v1 and whose bound covers the Conduit's target and names every operation the Conduit lists, as ACC-012 has it for close.
- For a Conduit listing get-secret.v1, it is dispatched only for root on its own behalf (ruling 66: "anything touching a secret").
- On a Conduit that is not marked, it reports conflict. On one already kept, it succeeds and changes nothing.
- Effect: the Conduit is recorded `kept` {by, invocation, at}, the mark is removed for good, and its review item is answered `kept`.
- A bound withholds it.
- It is passed on only under ACC-030.
- It grants nothing.
Batch 2 adds B10's guard as one clause.

ACC-052 (convention, must), the mark. An open Conduit whose `openedThrough` is not empty and wholly closed, and that has not been kept, is granted under a closed authority.
- Inspection and inventory show the mark `granted-under-closed-authority`.
- It is computed, not stored.
- Batch 2 adds "or its actingFor is closed".

C. NEW INTERFACE PROMISE

IFC-049 (interface, must). Proposed as the next free ID after IFC-048, which the brief reserves for batch 2. Within sign-in.v1 requested by an Interface whose request declared URL-mode elicitation:
- The engine answers with the protocol's input_required result. It holds exactly one URL-mode elicitation: a message naming the instance, the account expected and what follows, the `url` and the `elicitationId`. It also holds a requestState that only the engine can read.
- The client's retry carrying the response continues the same invocation.
- The engine passes only an `https` URL.
- No content holds the URL.
- A task's inputRequests are handled the same way.
- The protocol mechanics must pass the citation check (dependencies).

D. CHANGED PROMISES
- IFC-016: replace "answers none ... and passes none on" with: none except a URL-mode elicitation. One that arises in a part of sign-in.v1 goes under IFC-049. Any other goes to no one, raises PND-005(a), and is read as IFC-010 reads it. v0 has one path: even root's direct call is not relayed inline, so the account is always checked.
- IFC-045: calls for requests and hooks also declare elicitation in URL mode only, never form mode. Provider calls declare it not.
- IFC-007: the observable now cites IFC-045. A server that needs form-mode elicitation or another capability still answers -32021, read per IFC-010. One asking for a URL elicitation is read under IFC-016.
- IFC-010: observable only. The table is unchanged (input_required stays unknown); a URL elicitation in it raises a sign-in, named in `pendingAction`.
- IFC-025: three result keys: `grantedUnder`, `pendingAction` and `unanswered` (Interfaces only). The brief wrote granted-under; camelCase matches the existing keys.
- ACC-005: the part's `openedThrough`.
- ACC-012 and ACC-030 (observable): closing lists, marks and raises the review.
- ACC-033 (merged into the Fleet area's rewrite): rows for keep.v1 (`access`, ACC-051), sign-in.v1 (the owning Fleet for an Agent; `management`, root alone, for a Resource) and answer.v1 (PND-010).
- CON-008 (observable): an authority closed by retirement lists its grants in the confirming invocation's result. A retired Agent's Conduits close and answer their items `closed` (ruling 54 point 5). A Resource retirement may raise PND-008.
- REC-001 and REC-002: cross-references to PND-012.
- REC-004: provenance, keeps and pending actions are state that purge does not touch.
- FST-002 and FST-003: root's Conduits' operation lists, together with the Fleet area's edit.
- LIF-013, through inspect.v1: `status.pending` and `status.marks`.

E. CORE PACKAGE (edited in place, as NAM-024 allows: added operations and optional outputs; the batch says so, as ruling 41 did)
- operations.yaml:
  - new keep.v1, sign-in.v1 and answer.v1;
  - inspect.v1 output gains `provenance` and `kept` for Conduits, and `status.marks` and `status.pending`;
  - close.v1, open.v1 and retire.v1 descriptions;
  - list.v1 (the Fleet area's new operation) output gains `pending`.
- capabilities.yaml: access-control.v1 gains keep.v1. object-management.v1 gains sign-in.v1 and answer.v1. pool-management.v1 gains answer.v1. fleet-management.v1 gains sign-in.v1 and answer.v1.
- templates.yaml: root-access gains keep.v1, beside ruling 55 A7's lift, so the list changes once. root-management gains sign-in.v1 and answer.v1, beside list.v1. Ruling 66's manage template for an Interface lists sign-in.v1 and answer.v1 on Fleets and answer.v1 on Pools. No template for an Agent lists either.
- results.yaml: pending-action.v1, evidence.v1 and provenance.v1.

F. definitions.md: new rows for pending action, raised at, answer, evidence (item kinds), prompt, the five kinds, provenance, authority (of a grant), granted under a closed authority, keep, review and control panel. The rows for Interface, root Interface and Conduit's keys are amended, and "outcome" gets a note that an answer is not one.

G. implementation-defined.yaml: DL18, how long sign-in.v1 waits for a sign-in to complete.

H. spec/README.md limits:
- A pending action reaches a person only if the client shows it.
- URL elicitation relies on the client honouring the protocol.
- A model-driven console can misrepresent evidence, so the person compares against the panel's own rendering.

I. Size: about 17 new promises (PND-001 to PND-012, ACC-044 and ACC-050 to ACC-052, IFC-049) and about 16 changed, plus about 14 definitions rows. This is more than the brief's "about 5": ruling 54's review, keep and marks moved from batch 2 into this batch, and sign-in.v1 and answer.v1 are included.

## Vectors

EXISTING CASES THAT CHANGE (verified by reading). Five cases assert root's Conduits' exact `operations` arrays:
- conformance/conventions/first-start/initial/case.yaml (lines about 165 to 199)
- conformance/conventions/first-start/repeat/case.yaml
- conformance/conventions/first-start/lifecycle/case.yaml
- conformance/language/first-start/protected/case.yaml
- conformance/conventions/s1/whole/case.yaml
In each, root-access gains @core#keep.v1, and root-management gains @core#sign-in.v1 and @core#answer.v1. Make it the same single edit as list.v1 (ruling 59) and lift (ruling 55 A7), so the five cases change once.

CHECKED AND UNCHANGED
- conventions/requests/read-input-required and write-input-required: their input_required completion carries only content and resultType, with no inputRequests, so no sign-in is raised and IFC-010's reading stands.
- read-capability-error, write-capability-error and aware-capability-undeclared: the -32021 names `sampling`, not elicitation.
- No case asserts clientCapabilities, so IFC-045's declaration changes no case. Shell tests that build outgoing calls are checked by the work order.
- conventions/access/bounds steps 13 to 17: access-fixture decisions only. c611's close is never executed, so no grantedUnder appears.
- Setup cases ending failed or unknown (setup/cancel, hook-unknown, restore, review-conditions-builtin, review-conditions-provider, run-failed, run-unknown-run, run-unknown, run-partial, review-dependencies-failed): the setup kind is computed and adds no record or state. `unanswered` is an extra `_meta` member, which the runner's map projections allow. No case asserts an empty `_meta` (grep) or names the key in `absent`.
- Provenance is new Conduit state. State assertions are map projections, so none fails, and no record field is added.
- Snapshots (fixtures/root-system-state.json and the ten initial.json files) need not be what first start makes (the brief's snapshot rule), and none uses keep, sign-in or answer.
- Recommended: add provenance assertions to about 3 existing opens (setup/base, access/routing, s1/whole), not a mass re-record.

NEW CASES (about 36)

conventions/access (ruling 54):
1. provenance-open (ACC-044): a delegate opens through its access Conduit; inspection shows actor, sentBy, openedThrough and invocation.
2. provenance-setup-parts (ACC-044, ACC-005): two covering access Conduits, both listed in ID order.
3. provenance-onward (ACC-044, ACC-016): a broker opens on a caller's behalf; actor is the caller and sentBy is the broker.
4. provenance-engine (ACC-044): first-start Conduits show cause first-start, and defaults show cause default (with the Fleet area).
5. close-lists-grants (ACC-050, ACC-052, IFC-025): root closes a delegate's access Conduit. grantedUnder lists two grants, both stay open and are marked, and the review is listed at root.
6. close-lists-covered: a grant under [a, b]. Closing a lists it with b, unmarked; closing b marks it, and the review is b's.
7. review-closed (PND-006): closing the items answers them, and the review leaves the list.
8. keep-root (ACC-051): keep removes the mark and answers the item `kept`; a repeat changes nothing; keeping an unmarked Conduit is conflict.
9. keep-bound (ACC-051): refused when the bound does not cover the grant (naming the bound), when keep.v1 is not listed, or for a get-secret grant kept by anyone but root. No case keeps by the grant's own actor or by an authority holder, so B10 later flips nothing.
10. close-by-retirement (ACC-050, CON-008): an Agent holding an access Conduit is retired; the confirming result carries grantedUnder.
11. retired-grantee (CON-008, ruling 54 point 5): the grant closes and its item reads `closed`.
12. marked-in-flight (ACC-052): an open dispatched before the close and ending after it is marked.
13. marked-reconciled (ACC-052): an unknown open reconciled as having occurred after the close is marked.
14. review-transitive (PND-006): sub-grants appear in the evidence and are not items.

conventions/pending:
15. raised-scoped (PND-002): a scoped Interface sees only its Fleet's items, with no count. Root sees all; an Agent's list shows none.
16. answer-limited (PND-003, ACC-012): a scoped Interface without access is refused close and keep; root answers. This case encodes the provisional reading of F1.
17. setup-failed (PND-007, PND-010): the action's ID is the invocation; root dismisses it; a repeat is conflict.
18. setup-retried: a further setup answers `retried`; when it succeeds nothing is pending.
19. setup-unknown-reconciled: reconciled as having occurred gives `ready`.
20. notice-other-fleet (PND-008): an Agent of F retires a Resource that G's Agent uses. One notice for G, listed at root and at G's scope; the retire result carries pendingAction; G's manager acknowledges; a repeat is conflict.
21. notice-none: no notice when root is the sender, or when only F's own Agents are affected.
22. answer-errors (PND-010): answering a review is invalid, naming close.v1 and keep.v1. An unknown ID through management is invalid; through a scope it is refused, with no existence oracle. Opening an Agent Conduit that lists answer.v1 is refused.
23. unanswered-hint (PND-002, IFC-025): present while non-zero, absent at zero, never on Agents' results.
24. expiry (PND-003): written jointly with the enrollment area, since enrollment is the one kind that expires.

interface (ruling 49):
25. url-elicitation-agent (IFC-016, IFC-010, PND-004, PND-005): an Agent's call meets a Resource's URL elicitation. The reply carries no URL, the outcome is unknown, pendingAction is set, the sign-in subject is the Resource, and the records hold no URL.
26. url-elicitation-root-direct: not relayed inline; a sign-in is raised.
27. form-elicitation (IFC-016, IFC-045): nothing is relayed and no action is raised.
28. capabilities-declared (IFC-045, IFC-007): calls declare URL-mode elicitation only. Needs a runner intent field, or else a shell test.
29. sign-in-agent (PND-011, IFC-049): input_required with one https URL elicitation; accept; matching account gives succeeded {account}; the action is answered; the records hold no URL.
30. sign-in-wrong-account: the undo part runs, the outcome is failed, the reason names expected and seen, and the action stays pending with the attempt.
31. sign-in-declined: failed, and the action stays pending.
32. sign-in-no-capability: invalid.
33. sign-in-authority: allowed for F's manager on F's Agent; refused for G's Agent, for a Resource asked by a non-root caller, and for an Agent caller.
34. turn-held (PND-005): unavailable with pendingAction; after sign-in, the turn is dispatched.
35. sign-in-void: retiring the Agent answers its sign-in `void`.
36. sign-in-non-https: the URL is not passed and the outcome is failed.

Runner work: input kinds for an implementation's input_required that carries inputRequests (through `complete`), the Interface's retry with inputResponses, and account reports; projections of pending actions; case.schema.json.

## Build

The crates are named as of today; reference-model/ is about to be renamed engine/. Expect about 2.4k to 4.0k lines of Rust plus about 36 cases. The brief warns that estimates in this program run 1.5 to 2 times low, so read this as up to about 6k.

CORE (about 1.45k to 2.35k):
- New module crates/core/src/pending.rs (about 600 to 900): the pending-action model and kinds; computed review and setup views; stored sign-in and notice; scope routing computed from Conduits; disclosure filtering; answer.v1; expiry evaluation; the unanswered count; projections into list.v1 and inspect.v1.
- access.rs (+250 to 400):
  - provenance capture for direct opens, setup parts (every covering Conduit to access, in ID order) and onward requests;
  - the computed mark;
  - keep.v1's decision;
  - ACC-033 rows for keep, sign-in and answer;
  - the rule that only Interfaces hold sign-in.v1 and answer.v1.
- requests.rs (+300 to 500):
  - detect a URL-mode elicitation in an implementation's input_required or a task's inputRequests, raise a sign-in, and set the pendingAction reply key;
  - sign-in.v1 as an engine-implemented invocation with parts: a login call to the implementation, relay state, the account check, and a sign-out on mismatch;
  - hold model work while a sign-in is pending (unavailable).
- retirement.rs and lifecycle.rs (+150 to 250): collect grantedUnder on close.v1 and on CON-008 closes, including closes confirmed by later invocations; raise C5 notices on Resource retirement.
- lib.rs (+100 to 200): State gains pending actions, and Conduit objects gain provenance and kept; new Input kind for the Interface's elicitation response; reply `_meta` keys.
- records.rs (+50 to 100): records naming action IDs, and answer records holding the evidence shown.

SHELL (about 0.75k to 1.3k):
- server.rs (+250 to 450): serve input_required with a URL elicitation for sign-in.v1. Accept retries with inputResponses and an engine-minted, signed requestState. Check that the consumer declared URL elicitation. Content annotated for the user audience for evidence. The `_meta` keys.
- runner.rs (+200 to 350): declare URL-mode elicitation on calls (IFC-045). Parse implementations' URL elicitations. Act as the multi-round-trip client toward the implementation for the sign-in part. Never pass an elicitation on otherwise.
- main.rs, help.rs, author.rs and lib.rs (+300 to 500): the CLI commands `conduit pending`, `conduit sign-in <instance>`, `conduit keep <conduit>`, `conduit close --listed <authority>` (a named flag counted when used, ruling 55 A4) and `conduit answer <action> acknowledged|dismissed`. Client support for URL elicitation: show the full URL, ask consent, open the browser. Print grantedUnder after a close, and the unanswered hint.
- Non-root Interfaces need their own client tokens (DL13; only root's is issued today, per the brief at main.rs:178 and 234). That work is shared with the ruling 66 and Fleet areas.

CONFORMANCE CRATE (+200 to 400): runner inputs and projections, case.schema.json and the manifest.

LOADER: nothing; pending actions need no authoring. tools/check_schemas.py picks up the new prefix and schemas.

SUGGESTED WORK ORDERS (4):
1. Provenance, marks, list on close and keep.v1, with cases 1 to 14. Needs the Fleet and tombstone orders.
2. Pending-action core: shape, routing, disclosure, the review, setup and notice kinds, answer.v1 and the list and inspect projections, with cases 15 to 24. Needs the ruling 66 scope templates.
3. URL elicitation and sign-in.v1 across core and the shell's server and runner, with cases 25 to 36. Needs the sign-in area's Agent Type v2 login, account and sign-out operations (or a fixture Type) and the MCP citation check.
4. The shell panel CLI and its shell tests.

The spec text (section A to H of changes) is written first by the coordinator, in one pass with the batch.

## Ruling points

IMPLEMENTED, AND WHERE
- R49 point 1 (URL elicitation only, to a person's Interface, never to an Agent; IFC-007 and IFC-016 change): PND-004, PND-011, IFC-049, IFC-016, IFC-045, IFC-007 and IFC-010.
- R49 point 2, as amended by R66 decision 3: PND-002 and PND-011. Agents' sign-ins can be completed by their Fleet's managers; Resources' sign-ins are root's.
- R49 point 3 (account checked; a wrong account refused and undone): PND-011's result and PND-005's attempts. The check's mechanics belong to the sign-in area.
- R49 "What follows" (a lapsed login is a pending sign-in, and turns are held, naming it): PND-005(b) and the hold rule.
- R50 point 3 (one mechanism; the Interface as control panel): PND-001 to PND-004, list.v1 and inspect.v1, the `unanswered` key and `conduit pending`.
- R51 decision 5 (kind, subject, evidence, expiry and outcome, now "answer" under R56 decision 8): PND-001, PND-003, PND-004 and pending-action.v1.
- R52 points 1 and 2 (explicit, with disclosure limits): PND-002's disclosure rule, PND-010's refusal before invalid, the `pendingAction` and `grantedUnder` keys, and the notice reported to its sender.
- R52 point 4 (the join secret only in the action shown at root): PND-004's prompt rule. See F3.
- R54 point 2 (individual revocation, nothing cascades): ACC-050, with ACC-030 standing.
- R54 point 3 (provenance; the list on close; the pending action; marks until each grant is closed or kept): ACC-044, ACC-050, ACC-052, PND-006 and IFC-025.
- R54 point 4 (root keeps; the keep is recorded and the mark removed): ACC-051, keep.v1 and root-access.
- R54 point 5 (a retired Agent's Conduits close): CON-008, existing, plus case 11. Point 1 (Agents never move) is the Fleet area's.
- R55 A4 (the command is the approval; named flags counted): `close --listed` and `keep` in the CLI.
- R55 A7 (lift is an operation of `access` that a bound can withhold): keep.v1 follows the same pattern and lands with lift in root-access.
- R56 decisions 3, 4, 7, 8 and 11: PND-004 (the evidence form, shown only by the engine and only to Interfaces, user audience), PND-005 (account expected and user code), "answer" throughout, and PND-009 (root alone admits).
- R62 C5 (shared Pools): PND-008. R62 C4 (peers): PND-002 routes to the Interface the root template names.
- R66 decision 3: PND-002 to PND-004 and PND-010 to PND-011. R66 decision 4 (reconcile only one's own invocations) limits how a setup action can be answered; it is written in the ruling 66 area.
- R53 (inventory with no counts; tombstones): disclosure without counts. Tombstones carry provenance, and the review reads the closed authority from its tombstone.

R61 (BATCH 2) KEPT OPEN
- B9: the mark and review rules are written over "authorities". Batch 2 adds act-for Conduits as one clause in ACC-050 and ACC-052, plus the optional `actingFor` on provenance.v1, which NAM-024 allows. No expiry for reviews agrees with B9.
- B10: keep.v1 is already an operation of `access` that a bound can withhold. Its guard is one added clause, and no batch-1 case keeps by the grant's own actor or an authority holder, so no outcome flips.
- B11: withhold is untouched. B7 ("a Fleet closes only grants it sent") reads `sentBy`.

FINDINGS FOR THE OWNER
- F1. Ruling 66 says a scoped Interface may answer by "closing a grant always". But a scope "never covers ... grants of its own" and holds no Conduit to `access`, so under ACC-012 it can close nothing. Two readings:
  - (a) Closing is always an answer open to it, decided by what it holds. In v0 that is only explicit grants of `access`; batch 2's act-for adds a route. Recommended, and cases 16 and 33 assume it.
  - (b) The manage expansion adds a close-only Conduit to `access`, bounded to the scope.
- F2. Ruling 49 says an Agent's turns are "refused". In the spec's words, refused means not permitted, and it carries ruling 52's disclosure tiers. A turn held back by a pending sign-in is unavailable (OPS-023, CON-024). Recommended: unavailable, naming the action.
- F3. An enrollment action in command form carries a join secret that is not a URL, so ruling 49's "URL elicitation and no other kind" cannot carry it. Showing it at root puts it in result content that a model-driven root console reads, which is what ruling 56 decision 4 guards against. Recommended: content annotated for the user audience, the shell printing it only to a terminal, and the URL form required wherever the provider offers one (ruling 52 point 4 already prefers it).
- F4 (for the sign-in area). An arbitrary MCP server offers no way to ask who signed in, so ruling 49 point 3's check is possible only for Types that declare an account operation.
- F5. No ruling names the setup kind. It rests on ruling 50 point 3 ("every action waiting on a person"). It was scoped as computed so that it changes no existing case. It is the coordinator's addition, to confirm.
- F6. Preservation test 5 has two named exceptions: sign-in.v1 and answer.v1 are dispatched only for Interfaces. They follow from ruling 49 and from ruling 50's "waiting on a person". Closing and keeping stay available to Agents under a grant.
- F7. Ruling 51 decision 5 gives every action an expiry. Reviews, setups, notices and sign-ins have none, as ruling 54 implies and B9 states. Expiry is optional; this amends the reading of decision 5, as the brief listed.
- F8. C5's "sender other than root" is read literally (the sending instance), so root acting through a broker raises a notice. This is harmless.
- F9. answer.v1 and sign-in.v1 add one or two tools per Pool and Fleet to root's defaults, since ruling 59 A3 lists every operation except the cascades and act-for. Measure this in the rerun with C11.

## Dependencies

MUST LAND BEFORE OR WITH THIS AREA
1. The Fleet structure (rulings 59 and 60). This area needs:
   - fleet-management.v1 and Agent ownership;
   - list.v1, whose output this area extends with `pending`;
   - the ACC-033 rewrite, into which this area's rows merge;
   - root's default Conduits (cause default);
   - the ACC-024 amendment (root may close its own non-default access Conduits, which then trigger reviews);
   - the edits to FST-002 and FST-003 and the first-start cases, made as one edit.
2. Ruling 53 and ruling 55 A7:
   - tombstones, which must keep a closed Conduit's provenance, bound and close details;
   - the inventory, whose entries show provenance and marks;
   - expectedId;
   - the lift operation, which lands in root-access with keep.v1.
3. Ruling 66: the scope definition and the standard manage and view templates. The manage template lists answer.v1 and sign-in.v1; no Agent template lists them. Also needed: client tokens for non-root Interfaces (DL13 and the shell), without which routing to scoped Interfaces cannot be exercised. That token work is shared with the Fleet area.
4. Ruling 52's refusal tiers (OPS-023 changes), which answer.v1, keep.v1 and sign-in.v1 refusals cite.
5. Ruling 62: C1 (only root opens a Conduit whose consumer is root), C2 with ruling 66 decision 4 (the reconcile limits that shape answers to a setup action), and C4 (the peer root template that names the admitted Interface).
6. The sign-in area (ruling 49's "What follows"):
   - the Agent Types v2 contract operations for login (returning a URL elicitation), account status and sign-out;
   - the expected-account configuration key;
   - a home per Agent;
   - for Resources, a declared account operation (F4).
7. An MCP citation check of revision 2026-07-28, in the style of docs/proposals/reaudit/round-2/mcp-citations.md, covering URL-mode elicitation: input_required with inputRequests and requestState; the retry with inputResponses; how the capability is declared; elicitation-complete or URL-required signals; content annotations for the user audience. It must pass before IFC-049, IFC-016 and IFC-045 are final.

THIS AREA BLOCKS
- The enrollment area: its pending action, evidence presentation and expiry case (S3).
- Experiment A5 (signing in second accounts through the engine).
- The S1 rerun, if per-Agent homes land in batch 1: every S1 Agent then raises a sign-in at first setup, and the measurement plan must say how `conduit sign-in <agent>` counts under ruling 63 decision 1.
- Batch 2's B9 to B11 extensions.
- A78's coordinator phase, which relies on the panel.
- C6 and v1 Interfaces for other people, whose routing is already by scope.

INTERNAL ORDER: provenance (ACC-044), then the mark, list on close and keep (ACC-050 to ACC-052), then the pending-action core (PND-001 to PND-004, PND-010 and PND-012), then the computed kinds (review and setup) and the notice, then sign-in (PND-005, PND-011, IFC-049 and the IFC changes), then the shell panel.
