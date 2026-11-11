# Spec batch 1, area: machines-and-trust

Not binding. An area planner's draft for [the batch 1 plan](../README.md); where the plan differs, the plan governs.

## Scope

Enrollment of managed machines (rulings 50, 52 and 56): enroll.v1, the operations that admit, refuse and remove a machine, the pending machine state, nodes bound to the provider's machine id, labels named by the enrollment, the verification minimum and evidence. Also ruling 51's trust domain, named at first start in SPIFFE form, and its principle that network membership grants nothing. Peers (ruling 62 C4) are covered only as far as batch 1 must not foreclose them. Planned read-only from spec/ and rulings 49 to 56, 59 to 63 and 66; the enrollment, networking-paradigms, verification-methods and sign-in proposals; the Fleet brief, group C; the reference model; and the conformance vectors.

## Promise and core changes

A. NEW FILE spec/promises/enrollment.yaml, prefix ENR (coordinator choice C1). tools/check_schemas.py takes prefixes from the files, so no tool change is needed. Add a row to spec/README "Contents". Drafts follow.

ENR-001 (convention, must). Opening an enrollment. When the root Interface asks for `enroll-machine.v1`, and a machine provider that is not retired is in force and ready:
- The engine records an enrollment with: its ID, the provider, the labels it names, its expiry, the evidence it requires beyond the minimum (`require`), and `headless` and `acceptWeakerCheck` as given.
- It asks the provider with `enroll.v1`, inside the request's invocation.
- If the evidence kinds the provider declares meet the minimum (ENR-005) and every requirement, it raises one pending action of kind `enrollment` at the root Interface. That action holds the provider's action.
- The result gives the enrollment's ID and the pending action's ID, and no part of the action.

What can be observed:
- With only the engine's own machine provider, the request reports unavailable, with a diagnostic naming the missing machine provider (PRV-003). The same holds when the provider is not ready.
- These inputs report invalid, with a diagnostic naming the input: a label key that is `kubernetes.io/hostname`, `os` or `arch`, or lies under `kubernetes.io/` or `k8s.io/`; an expiry that is past, or further ahead than DL5 allows; `acceptWeakerCheck` without `headless`.
- `require: [attestation]`, or `headless` without `acceptWeakerCheck`, where the provider cannot give attestation: the provider call ends failed with a diagnostic naming the kinds. The request's outcome is failed, no action is shown and no pending action is raised.
- An `enroll.v1` call that does not succeed leaves no open enrollment.
- No input lowers the minimum (ruling 56 #5).
- `acceptWeakerCheck` is recorded and counted as a named flag (ruling 55 A4).

ENR-002 (convention, must). What the provider does in `enroll.v1`:
- It returns one action for a person: a `url` to open, a `command` to run on the new machine, or one of these for each value of `kubernetes.io/os`.
- With it: a `message`; whether the action `carriesSecret`; an expiry no later than the enrollment's; and the evidence kinds it will report for a machine that joins.
- Any secret is single-use and lapses at that expiry.
- A machine that joins by it joins pending. Where the provider's network supports it (Tailscale device approval), the machine cannot send or receive traffic until `accept-machine.v1`.
- The machine carries the enrollment's labels, bound by the provider so that the machine cannot change them.
- Where the provider can give an action that carries no secret, it gives that one (ruling 52 #4).
- Its contract states: the means (for example tags); the form of the secret; how it pins its coordination server; how it attributes a joined machine to the enrollment; which evidence items it gives and where on the machine each is shown; and whether `id` is derived from the machine's key (ruling 51 #8).

ENR-003 (convention, must not). The join action:
- The engine puts no part of the action from `enroll.v1` in any record, inspection result, observation, diagnostic, result or `_meta`, whatever the provider's type states under `records`. This parallels PRV-024 and SEC-018.
- It shows the action only in its own rendering at the root Interface's control panel, and only while the enrollment is open and no machine has joined.
- A `url` action is delivered as a URL elicitation, the same shape as ruling 49's sign-in.
- A pending action listed through a tool gives its kind, subject, expiry and state, never the action.

ENR-004 (convention, must). A machine that joined (`list-machines.v1` reports it `pending`):
- No node represents it, and none is made except by admission. The provider never places an instance on it.
- If its `enrollment` names an open enrollment of this provider, the engine attaches it to that enrollment's pending action. From then on the action shows the machine's hostname, `id`, labels and evidence, and whether it can be admitted (ENR-006). It no longer shows the join action.
- Otherwise the engine raises a pending action of kind `machine` at the root Interface, saying why the machine cannot be admitted.
- Nothing is granted to the machine (ENR-011).

ENR-005 (language, must). Evidence:
- Evidence is a list of `evidence-item.v1`, each with `kind`, `value`, `where` (where on the machine the person finds it, for example "`tailscale status`") and `proves`.
- Kinds:
  - `fingerprint`: derived from the machine's key; its value is 6 to 8 characters, the same on the machine and in the panel (ruling 56 #10).
  - `attestation`: a platform or hardware statement that the provider verified.
  - `code`: a code for one transaction, such as a device-flow user code; not derived from a key.
  - `fact`: something the machine reports about itself.
- Only `fingerprint` and `attestation` meet the minimum. Facts alone never do.
- Every answer to a pending action that shows evidence is recorded with the evidence shown (ruling 56 #3).

ENR-006 (convention, must). What admission requires. `admit-machine.v1` names the enrollment and the machine `id` that was shown. It is dispatched only if every check below holds. Otherwise it reports conflict, or invalid for check 7, with one diagnostic for each failed check:
1. The provider in force reports that `id` as `pending`, attributed to this enrollment.
2. The root Interface opened the enrollment through that provider; its `enroll.v1` call succeeded; it is open and not expired; and no other machine has joined by it. Two machines on one enrollment make neither admissible.
3. The machine's hostname is not that of `local`, of a node that is not retired, or of another pending machine.
4. Its labels are exactly the three descriptive ones plus those the enrollment named.
5. Its evidence holds at least one `fingerprint` or `attestation`.
6. If the enrollment requires attestation or is `headless`, an `attestation` item is present, unless `acceptWeakerCheck` was given.
7. (Owner point O1.) If there is no attestation, or the enrollment requires `fingerprint`, and it is not a headless enrollment that accepted the weaker check, the request carries `fingerprint`, and it equals the reported value.

Any other actor gets refused (ACC-032). A machine that fails any check can only be refused.

ENR-007 (convention, must). Admitting:
- The engine calls `accept-machine.v1`. Once that succeeds, it registers a node from core `node`, owned by `system` and named as PLC-026 names one, and sets it up.
- The node represents the pair (provider, `id`, hostname). Its labels follow ENR-010.
- The pending action is answered `admitted`. The answer's record holds the enrollment, the `id`, the evidence shown and any weaker-check flag. The enrollment closes.
- The registration is part of root's request and needs no Conduit to `system`. PLC-028 and FST-006 still hold: no actor registers a node.
- Admitting asserts that the user controls the machine (ruling 51's caveat). Another party's machine is never a node: what runs there is reached by `remote` or acts through an Interface.
- If the accept call fails, there is no node and the action can still be answered. If its outcome is unknown, the request's outcome is unknown and there is no node; a new request may try again.
- A repeat is answered from what the engine kept (OPS-009). Admitting an enrollment that was refused or has expired reports conflict.

ENR-008 (convention, must). Refusing (`refuse-machine.v1` names an enrollment, a machine `id`, or both):
- For each named machine, or each machine that joined by the named enrollment, that no node represents, the engine calls `remove-machine.v1`.
- An open enrollment that is named closes as `refused`, so a machine that joins by it later cannot be admitted.
- A `machine` or `machine-removal` pending action is answered `refused` or `removed`.
- A machine that a node represents reports conflict, naming the node.

ENR-009 (convention, must). Expiry, when an enrollment's expiry passes without an answer (coordinator choice C3):
- It closes as `expired`, and its action is no longer shown.
- A machine that joins by it, or had joined and was not admitted, is raised as kind `machine`.
- The engine asks the provider for nothing. Removal is always root's answer; this follows the later-work rule of ACC-019.

ENR-010 (convention, must). A node's labels:
- `kubernetes.io/hostname`, `os` and `arch` as last reported, plus the enrollment's labels, fixed at admission.
- For `local`: the same three, plus labels configured at first start (DL6). This keeps the existing `example.org/role` vector valid.
- No report adds, changes or removes any other label. A report that differs leaves the labels as they are and adds a condition.
- This is ruling 52 #4's narrow exception to "no document and no operation sets one".
- A Linux environment inside another operating system, such as WSL2 on Windows, is a machine of its own and reports `linux`; A74 is to confirm this.

ENR-011 (convention, must not). Network membership grants nothing (ruling 51 #1):
- Joining the provider's network, being accepted there, or being admitted as a node establishes no instance and permits no request.
- Admission makes one node: no Conduit, no credential.
- Who sent a request is established only by what the request presents (IFC-022, PRV-018). Only an open Conduit permits.

ENR-012 (convention, must). The removal question:
- When a node is retired, whether declared lost by root (PLC-017) or retired by a request (PLC-011, coordinator choice C4), and the provider still reports its machine, the engine raises a pending action of kind `machine-removal` at root.
- Nothing is removed until `refuse-machine.v1` answers it (`removed`). The generic dismissal answers `kept`.
- A report of `gone` raises no such action.

FST-007 (convention, must). The trust domain (ruling 51 #2):
- At first start the engine names a SPIFFE trust domain: lowercase letters, digits, `.`, `-` and `_`, at most 255 characters.
- It is unique: either configured by the user (for example a DNS name the user controls) or generated from at least 128 random bits.
- It is recorded in the status of `system` as `trustDomain` (coordinator choice C5) and never changes.
- Every credential that DL13 describes names its holder by a SPIFFE ID in this domain: `spiffe://<td>/conduit/engine` and `spiffe://<td>/conduit/instance/<id>` (coordinator choice C6). `/conduit/node/<id>` is reserved for v1.
- `engine-id` holds the engine's SPIFFE ID. Restarts, moves (DL17) and repeated first starts keep the domain. A shared fixed name such as `conduit.local` is never generated.

B. EXISTING PROMISES CHANGED
- PLC-026 is rewritten:
  - Nodes are made only at first start (`local`) and on admission.
  - A node represents (provider Resource, `id`, hostname) until it is retired. `local` represents the engine's hostname alone and is never pending.
  - A report that gives a node's hostname under another `id`, or its `id` under another hostname, does not represent that node. The node counts as omitted (PLC-023) and gets a condition.
  - A machine that is reported but not admitted gets no node.
  - The sentences "whichever machine provider reports it" and "a later provider ... reports the same node" are removed (owner point O2).
- PLC-008: the node's status holds `machine {provider, id, hostname}`, the enrollment and the labels.
- PLC-017: declaring a node lost raises the ENR-012 question.
- PLC-023: adds the mismatch case.
- PLC-024: a machine that comes back needs a new enrollment; `gone` raises no removal question.
- PLC-028 and FST-006: "the engine makes every node, at first start and on admission".
- FST-001: `local`'s other labels come from configuration; cites FST-007.
- PRV-003: the engine's own machine provider enrolls nothing.
- PRV-005: the three new provider calls are made inside root's invocations; the record of `enroll.v1` holds no action.
- PRV-013: replacing the provider closes its open enrollments, and nodes stay omitted until each machine is re-enrolled.
- PRV-014: `id` is required, opaque and provider-scoped; a new state `pending`, with `enrollment` and `evidence`; the labels rule follows ENR-010; adds the WSL2 sentence.
- PRV-015: `place.v1` names a machine by `id`, never a pending one.
- PRV-018: observable cites ENR-011.
- PRV-022: `engine-id` holds the SPIFFE ID.
- ACC-032: lists five reserved operations.
- REC-001: enrollment opened, closed or expired, and the answer recorded with its evidence, written jointly with the pending-action area.
- DL5: the longest an enrollment may last (suggested 24h).
- DL6: how the trust domain is chosen and kept.
- DL13: SPIFFE IDs required. "One root of trust" is scoped to "in this version", so a peer's bundle can be added in v1. `/conduit/node/<id>` is reserved.

C. CORE PACKAGE
- Root verbs (`reservedToRoot`, on `object-management.v1`, engine-implemented, not `deduplicates`):
  - `enroll-machine.v1`. Inputs: labels; expires (date-time, required); require (array of fingerprint or attestation); headless; acceptWeakerCheck. Outputs: enrollment, pendingAction.
  - `admit-machine.v1`. Inputs: enrollment and machine (both required); fingerprint (6 to 8 characters). Output: node (`object.v1`).
  - `refuse-machine.v1`. Inputs: enrollment and/or machine.
- Provider operations (on `machine-provider.v1`):
  - `enroll.v1`. Inputs: enrollment, labels, expires, require, headless. Outputs: action, message, carriesSecret, expires, evidence (kinds).
  - `accept-machine.v1`. Inputs: id, enrollment.
  - `remove-machine.v1`. Input: id.
- Changed contracts:
  - `list-machines.v1`: `id` required; state `pending`; `enrollment`; `evidence`.
  - `place.v1`: output `id`.
  - `declare-lost.v1`: description changes.
- Capabilities: `object-management.v1` and `machine-provider.v1` gain three operations each.
- `management.v1` requires the three root verbs. `node.v1` and the templates `node`, `system` and `root-management` get new descriptions; `root-management` lists the three verbs.
- New schemas: `evidence-item.v1` in results.yaml and `trust-domain.v1` in values.yaml. conduit.yaml exports grow by 8. The core/README table is updated.
- Names: the provider-side verb avoids "approve" (ruling 55 A4) and "authorize" (an access term), coordinator choice C2.
- `list-machines.v1` and `place.v1` are edited in place. This is safe because packages/ holds no machine provider (only the two Agent packages). The batch record must say so (brief item 13); the alternative is `.v2` (coordinator choice C8).

D. DEFINITIONS.MD
- New rows: enrollment; admit; refuse (a machine); pending (the machine state, distinct from a shutdown that is pending and from a pending action); evidence and its items; machine ID; trust domain.
- Vocabulary clashes:
  - Ruling 42's N19 renamed the secrets sense of "trust domain" to "domain of trust". Reword that words-not-used row: "trust domain" now has only the SPIFFE sense.
  - Ruling 41 removed "facts". The facts row must say that `fact` returns only as an evidence kind.
- Changed rows: node (a machine the user controls; another party's machine is never a node); represent; machine; machine provider; label; `system`; reserved to the root Interface (five operations); root Interface; the engine's credential ID; credential; `identity`; provider call.

E. spec/README
- Status: rulings 50, 51, 52 and 56 written; peer mode not yet.
- "What this is" gains the network principle.
- Contents gets the new file and new counts.
- Limits: peers (introduction, two halves, peer evidence) are v1.

F. reference-model/CONFORMANCE.md: rows for DL5, DL6 and DL13.

G. Documents to tidy afterwards: merge the three enrollment files under condensation/plan.md; reword "Fleet" (fleetdm) in the enrollment README under ruling 60; mark docs/guides/second-machine.md test-only.

## Vectors

NEW PORTABLE CASES
Under conformance/conventions/enrollment/ unless noted. They need:
- a snapshot fixture with a machine-provider Resource owned by `system`;
- a fixture package declaring a Type that offers `machine-provider.v1`;
- new case-schema inputs `machine-report` and `clock` (`clock` is shared with the pending-action area);
- first-start evidence gaining `trustDomain`.

1. enroll-own-provider: unavailable, naming the missing machine provider; no intent and no pending action (ENR-001, PRV-003).
2. enroll-reserved: a non-root actor gets refused for each of the three verbs. Root opening, for another consumer, a Conduit that lists `admit-machine.v1` also gets refused (ACC-032).
3. enroll-opens: a provider-call intent for `enroll.v1` with the enrollment ID, labels, expires and require. Its completion holds a command with a marker secret. Then a pending action of kind `enrollment` is raised, the result holds only the two IDs, and the marker is absent from records, state and the result (ENR-001 to ENR-003, PRV-005).
4. enroll-invalid-inputs: reserved label keys, expiry past or too far ahead, acceptWeakerCheck without headless. Each reports invalid with no intent.
5. enroll-evidence-unmet: require attestation, and the provider fails. Also a completion that declares only `fact`. Each ends failed, with no action shown.
6. enroll-headless: headless without the flag, against a provider with no attestation, fails. With the flag, the enrollment opens and the flag is recorded.
7. join-attaches: a report of a pending machine attributed to E. The action shows the machine and its evidence, and no node is made (ENR-004, PLC-026).
8. admit: the fingerprint matches, so an `accept-machine.v1` intent follows. On completion a node is ready, with owner `system`, its name from the hostname, status `machine {provider,id,hostname}` and labels of the three plus the enrollment's. The answer `admitted` is recorded with its evidence, the enrollment closes, and no Conduit is added (ENR-006, ENR-007, ENR-010, ENR-011).
9. admit-fingerprint: a missing or wrong fingerprint reports invalid, with no intent (O1).
10. admit-attestation: attestation present and no fingerprint given; the machine is admitted.
11. admit-not-admissible: one step for each case: no enrollment; expired; another provider's; two machines on one enrollment; `local`'s hostname; an admitted node's hostname under another id; labels that differ; evidence of facts only. Each reports conflict, with a diagnostic naming the check. Refusal still works.
12. admit-accept-failed and admit-accept-unknown: no node; the first leaves the action answerable, the second gives outcome unknown.
13. admit-repeat: a repeat with the same key is answered from what was kept. Admitting after a refusal reports conflict.
14. refuse-pending: a `remove-machine.v1` intent; answered `refused`; no node.
15. refuse-cancel: an open enrollment is refused before any machine joins; a later join is raised as kind `machine` and cannot be admitted.
16. refuse-represented: refusing a machine that a node represents reports conflict naming the node.
17. expiry-before-join and expiry-after-join: driven by `clock`. Answered `expired`, with no provider intent; the machine is raised as kind `machine` (ENR-009).
18. conventions/nodes/no-node-without-admission: a reachable machine that was never admitted gets no node. This reverses today's PLC-026.
19. nodes/id-mismatch: the node is omitted, requests to an instance placed on it report unavailable (PLC-023), a condition is added and no new node is made.
20. nodes/labels-drift: a reported extra label leaves the node's labels unchanged and adds a condition.
21. nodes/declare-lost-removal: `machine-removal` is raised with no remove intent; refusing then gives a remove intent, answered `removed` (PLC-017, ENR-012).
22. nodes/retire-removal: same as 21 for a retirement by request (C4).
23. nodes/gone: lost, with no removal question (PLC-024).
24. nodes/readmit: the machine of a lost node gets no node until a new enrollment and admission make a new one, with a generated name under OBJ-003.
25. nodes/provider-replaced: the nodes stay omitted (O2, PRV-013).
26. nodes/place-by-id: `place.v1` naming an admitted id gives that node; naming a pending id reports unavailable (PRV-015).
27. first-start/trust-domain: the evidence is recorded in `system`'s status. Malformed values are rejected as invalid first-start evidence: uppercase, empty, a `spiffe://` prefix, more than 255 characters. A repeated first start keeps the domain (FST-007).
Plus validate-values cases for `evidence-item.v1` (fingerprint length, kind enum) and `trust-domain.v1`.

SHELL TESTS (not portable)
- `engine-id` is `spiffe://<td>/conduit/engine`.
- A token's `sub` is `spiffe://<td>/conduit/instance/<id>`.
- The authority's domain equals `system.status.trustDomain`.
- A token from another domain is refused.
- `conduit machine enroll` with no provider prints the unavailable diagnostic.
- With a fake provider returning a secret marker, a scan of the store, records and tool results finds nothing.

EXISTING VECTORS WHOSE OUTCOME CHANGES (checked by reading them)
- Five cases need first-start `evidence.trustDomain`, an expected `system.status.trustDomain`, and the `root-management` operation list (an exact array that today ends at `declare-lost.v1` and `purge.v1`) extended by three:
  - conformance/conventions/first-start/initial/case.yaml (list at line ~177)
  - conformance/conventions/first-start/repeat/case.yaml (~169)
  - conformance/conventions/first-start/lifecycle/case.yaml (~172)
  - conformance/language/first-start/protected/case.yaml (~179)
  - conformance/conventions/s1/whole/case.yaml (~217)
  The list.v1 area edits the same arrays: make one pass.
- conformance/case.schema.json: the first-start evidence uses `additionalProperties: false`, so it must gain `trustDomain`.
- reference-model/crates/shell/src/tests.rs lines 819 to 872 assume the `.conduit.local` suffix and the `/instance/` path.
- conformance/manifest.yaml: new entries, and the PLC-026 and PRV-014 rows re-described.

UNCHANGED
- conformance/fixtures/root-system-state.json and the other snapshot fixtures holding the old operation list: they are snapshots, not first-start claims. Adding `trustDomain` to the shared fixture is optional.
- No vector holds a node other than `local` (checked across all fixtures).
- The PLC-007 label vectors use `local`'s labels from configuration, which ENR-010 keeps.

## Build

BATCH 1 BUILD: spec, the core and vectors, and a thin shell. Paths are under reference-model/, or engine/ after the rename.
- Spec order: enrollment.yaml (about 13 ENR promises, ~700 lines), FST-007, about 20 edited promises, 9 core contracts (6 new, 3 changed), 2 schemas, definitions and README. About 1,300 lines of diff.
- crates/core:
  - new machines.rs (~1,200 to 1,600 lines): the enrollment store; folding in a `machine-report` (pending, attach, mismatch, omitted); admissibility checks with explicit diagnostics; admit, refuse and expire; the removal question; node binding.
  - lib.rs (~120): Input::MachineReport, `trustDomain` in first-start evidence with validation, recorded on `system`.
  - requests.rs (~200): the three verbs as engine-implemented, plus provider-call intents and completions for enroll, accept and remove.
  - access.rs (~10): the reserved list.
  - registration.rs and lifecycle.rs (~100): node creation reused from first start.
  - records.rs (~60): records of answers with their evidence, and no action.
- crates/conformance: input kinds and schema (~200), the 27 cases, a machine-provider fixture package, and a snapshot.
- crates/shell:
  - identity.rs, main.rs and lib.rs (~150): `conduit init` generates or takes the trust domain and passes it as first-start evidence; the authority reads it from state; SPIFFE engine ID and paths. Stores from before batch 1 are declared unsupported; the rerun starts from a fresh init.
  - CLI `conduit machine enroll|admit|refuse` (~200, with tests), including the counted `--accept-weaker-check` flag.
  - CONFORMANCE.md rows.
- Estimate: about 2,000 to 2,800 lines of Rust plus about 27 vectors, in three Codex orders: spec; core and vectors; shell.

DEFERRED TO AN S3-PREP ORDER (not needed for the S1 rerun)
- The shell executor serving machine-provider calls: a stdio server under its own runner (PRV-002, PRV-026). The shell serves no provider calls today (CONFORMANCE.md DL16).
- The DL5 polling loop and expiry clock in the shell.
- Panel rendering, once the pending-action area defines the channel.
- packages/machine-tailscale (Rust, about 1,000 to 1,500 lines): an OAuth client with the auth_keys scope; a one-off, tagged, non-preapproved key; the device list mapped to pending, tags mapped to labels, and a node-key fingerprint of 8 characters; accept through /device/{id}/authorized; remove by delete.
- A74 facts it must confirm:
  - the node key is visible both on the machine and through the API;
  - how a device is attributed to its enrollment key (if the provider cannot attribute one, the machine reports no enrollment and fails closed);
  - WSL2 reporting `linux`;
  - mirrored networking.

## Ruling points

WHERE EACH RULING POINT IS IMPLEMENTED
- R50.1 (enrollment is an operation of the machine provider that returns an action): `enroll.v1` and `enroll-machine.v1`, ENR-001 and ENR-002.
- R50.2 (a node only after a person confirms): PLC-026, ENR-006 and ENR-007.
- R50.3 (one mechanism, a control panel): pending kinds `enrollment`, `machine` and `machine-removal` (ENR-001, ENR-004, ENR-012), raised at root only, since `system` is never in a scope (R66.1 and R66.3: "enrollment ... stay[s] root's").
- R51.1 (network membership grants nothing): ENR-011, definitions, the spec/README principle, PRV-018.
- R51.2 (trust domain): FST-007, DL6, DL13, PRV-022, `system`'s status.
- R51.3 and the caveat (another party's machine is never a node): the definition of node, and ENR-007's observable that admitting asserts the user controls the machine.
- R51.4, 6, 7, 9 and 10: nothing to write in v0. They are carried into the batch record and the spec/README limits.
- R51.5 (a generic pending action): consumed from the pending-action area.
- R51.8 (a required, opaque machine id): `list-machines.v1` and PRV-014.
- R52.4, the enrollment decisions:
  - option C with a secret-free link: ENR-002;
  - a node bound to the id: PLC-026;
  - labels from the enrollment: ENR-010 and PRV-014;
  - the secret only in root's panel: ENR-003;
  - refusing removes the machine and declaring lost asks first: ENR-008 and ENR-012;
  - WSL2 reports `linux`: PRV-014, with A74 to confirm.
- R56.1 (the provider chooses the method, in its contract): ENR-002 and ENR-005.
- R56.2 (the minimum): ENR-006, checks 1 to 5.
- R56.3 (the evidence shape): `evidence-item.v1` and ENR-005.
- R56.4 (the engine alone shows it): ENR-003, ENR-004 and the panel dependency.
- R56.5 (upward only): `require` and `headless`, with no input that lowers the minimum.
- R56.6 (Tailscale): the S3-prep package and A74.
- R56.7 (the same evidence field for peers and sign-ins): the shared `evidence-item.v1`.
- R56.8 (the word "answer"): used throughout.
- R56.9 (a machine with no screen): `headless` plus `acceptWeakerCheck`, recorded and counted (also R55 A4).
- R56.10 (a fingerprint of 6 to 8 characters): ENR-005.
- R56.11 (root alone): `reservedToRoot`. Ruling 61's B2 and B7 already keep Fleets away from `system` and root-reserved operations, so batch 2 is not foreclosed.

R62 C4: WHAT BATCH 1 MUST NOT FORECLOSE (nothing about peers is written)
1. A pending action's `kind` must be additive, either an open string with documented values or extensible under NAM-024, so a later `peer-introduction` kind needs no .v2. The set of Interfaces an action is raised at must be computed by a rule (root plus scopes, per R66), so "Interfaces the root template names" can be added later.
2. Root's default Conduits must carry engine-made provenance (ruling 54), so ACC-024's protection can extend to an admitted Interface's defaults. `as` must stay unique per consumer (the IFC-014 amendment in the short-form synthesis).
3. Write nothing that closes the core `root` template to a later additive input.
4. DL13's single root of trust is scoped "in this version". IFC-022 is untouched. The unique domain and SPIFFE IDs let a peer's bundle validate foreign IDs without renaming any.
5. `fingerprint` and `code` already carry a trust-root fingerprint and a PAKE or short-authentication-string code.
6. A node is a machine the user controls, so peer machines never enter placement.

FINDINGS AND QUESTIONS FOR THE OWNER
- O1 (recommend yes). Should `admit-machine.v1` carry the fingerprint the person read on the machine, with the engine comparing it? Without this, a model-driven session holding root's tools could admit a machine blind. It fits R55 A4 ("the command is the approval") and R56.4. R56.10 itself only says the fingerprint is shown in both places, so this adds a requirement.
- O2 (recommend accepting, as a consequence of R51.8 and R52.4). Binding a node to the provider and its id means that replacing the machine provider leaves every node omitted until each machine is enrolled again. This reverses PLC-026's provider-neutral sentence, which ruling 42 accepted. The alternative is to bind to the provider's Type.
- O3 (information). Ruling 51 supports peer mode, but it cannot land in batch 1: admitting the owner's Interface on a peer needs tokens for Interfaces other than root (batch 2), and full introduction with trust-root evidence is v1 (R56.7). Nothing is unimplementable as ruled.

COORDINATOR CHOICES (the owner may strike any)
- C1: the ENR file.
- C2: the operation names.
- C3: expiry removes nothing at the provider.
- C4: the removal question is also raised on a retirement by request.
- C5: the trust domain is recorded on `system`.
- C6: SPIFFE IDs are required.
- C7: labels are only the three descriptive ones plus the enrollment's (a provider's own attested labels wait for v1).
- C8: `list-machines.v1` and `place.v1` are edited in place.
- Vocabulary: the later rulings govern over N19's "trust domain" and ruling 41's "facts"; definitions.md is reworded to match.

## Dependencies

MUST LAND BEFORE THIS AREA, OR WITH IT
1. The pending-action area (rulings 49, 50, 51 #5, 56 #4 and #8, 66 #3). It must provide:
   - `pending-action.v1` with an additive `kind`, `subject`, `evidence` (as `evidence-item.v1` defined here; whichever area lands first writes the schema), an optional `expires`, and `answer` with who answered and when;
   - the raising rule;
   - a person-facing panel channel rendered by the engine that no tool result carries: a URL elicitation to root's client, or an engine page behind root's credential, with a `url` action delivered like ruling 49's sign-in;
   - a generic dismissal (used for `kept`);
   - REC-001 additions;
   - the conformance input `clock`.
2. The explicit-refusals area (OPS-023's two tiers and ruling 52's explicitness rule), for the form of the refusals and diagnostics on the three verbs.
3. Ruling 53's descriptive tool descriptions, for the three new root tools.
4. Ruling 59's root defaults and list.v1. They edit the same `root-management` operation list and the same five first-start vectors: do one combined pass.
5. The decision on when reference-model/ is renamed engine/.

WHAT THIS AREA BLOCKS
- The S1 rerun only through first start: the trust domain must land before the rerun's `conduit init`, so no store needs migrating. Root also gains three tools (C11 counts root's Conduits apart).
- S3, which adds the Windows machine through enrollment rather than by hand, together with the S3-prep order and A74's facts.
- v1 peer introduction and C4, both built on the trust domain and the evidence field.
- Nothing in batch 2: Fleet authority (ruling 61) never touches `system` or root-reserved operations.

ORDER WITHIN THIS AREA
1. FST-007, DL13 and the shell's trust domain (small and independent: land it first).
2. The ENR promises and the PLC and PRV edits, plus the core contracts.
3. The core model and vectors, once the pending-action core exists.
4. The shell CLI.
5. Later: S3-prep (provider-call executor, polling, panel, Tailscale package).
6. Documentation upkeep: merge the enrollment records (condensation plan), reword the fleetdm use of "Fleet" (ruling 60), and update second-machine.md.
