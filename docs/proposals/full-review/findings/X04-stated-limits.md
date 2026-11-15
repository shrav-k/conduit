# X04-stated-limits: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Commit 2eb8f258 (2026-11-12). Area: spec/README.md "Limits of this version" (lines 331-617), the limits in "What this is" (lines 53-54, 82-96), and the fence's limits, which sit only in OPS-014, OPS-018 and OPS-030.

The limits are stated honestly and, apart from F8 and F10, agree with the promises they cite. Most of them are one limit seen from different sides: with no `runtime` named, an Agent whose own tools are on runs as the engine's operating-system user and is inside the engine's domain of trust. The faults are mostly that a limit reaches the user only in this section and in no result (ruling 52).

Ranked by the harm each permits, most first:

1. **Programs of the engine's operating-system user reach every token and state directory** (README:424-427, 500-509; IFC-022, PRV-025, PRV-029, DL13, DL16).
   - Harm: such an Agent can present as root or as any instance, read every Resource login, secret and state directory, and rewrite records. Against a misbehaving Agent, tests 2, 3, 5 and 7 hold nothing.
   - Judgment: honest for v0 and consistent with the promises, but the user meets it in no result and has no recovery (F3, F5, F6).
2. **An Agent's own tools, neither enforced nor mediated** (CTR-014, LIF-013).
   - Harm: reach that no Conduit grants and no record shows.
   - Judgment: right for v0, and shown in inspection and in plans. The exceptions are an undeclared Type (F9) and that plans bind no tool (F8).
3. **Every restriction reported as mediated** (ACC-027).
   - Harm: test 9 is met in form only, because the report never varies. For a consumer inside the domain of trust the word overstates.
   - Judgment: right as a cautious default. It should not stand unqualified for that consumer (F3).
4. **Pending actions, sign-in and the network** (README:400-450).
   - Judgment: mostly right and consistent with PND-005, PND-013 and IFC-049.
   - Three faults: a wrong account that cannot be undone leaves a Resource serving (F1, unsafe); the engine's own panel is undefined (F2, unsafe); the placement rule of ruling 75 C1 is neither checked nor reported (F6).
   - "The person's Interface runs on the engine's machine" binds nothing: IFC-042 accepts a token from any machine.
5. **A run beside a later instance** (README:582-605; LIF-036, PRV-021).
   - Harm: two runs at once, the earlier keeping its secrets.
   - Judgment: right for v0, as a forced deletion in Kubernetes is. The moment it becomes final is silent (F7), and it is filed under secrets, where an author replacing a Pool's entry will not look.
6. **The fence's two limits** (OPS-014; ruling 75's later answers).
   - Judgment: both are right for ordinary requests. Neither appears in the limits section.
   - Faults: a setup hook has no new request (F4). The first limit's remedy, each hop handling its own idempotency, is unavailable to a plain server, which is given no key (IFC-007).
7. **Management cannot be narrowed below whole operations** (README:466-487).
   - Harm: `register.v1` or `replace.v1` trusts its consumer to choose programs; with `set-up.v1` that is code run as the engine's user.
   - Judgment: known, and S1 and S2 measure it. No new fault. A broker cannot register an Agent (ACC-057), and Conduits to Pools and Fleets take no filter either, which the section does not say.
8. **A bound does not limit filters** (README:454-461).
   - Judgment: known and consistent with ACC-029 and ACC-030.
   - In v0 `open.v1` opens only a Conduit already registered (core/operations.yaml:755), and registering one needs `management`. So the section's example needs a registrar until batch 2's `make.v1` makes it fully real.
9. **Records lost or altered outside the engine** (README:607-617).
   - Judgment: a consequence of 1. The owner reviewed it on 2026-11-12 (review list item 29). No new fault.
   - DL8 sets no floor: an engine that can tell no outside loss conforms.
10. **What the guarantee for secrets covers.**
    - Judgment: consistent with SEC-001 to SEC-024.
    - Lesser harms: a secret bound to an argument is visible to every user of the machine, outside the stated domain of trust, with no diagnostic (noted in F6). A rotated value reaches a running instance only by replacing it. A credential lasts as long as its instance (F5).
11. **What the short form's meaning covers.**
    - Judgment: right as an equivalence of registration (SCT-003, SCT-015). The tooling duties stated beside it bind nothing (F8).
12. **Fleets, discovery and tombstones.**
    - Judgment: consistent. The exceptions to the preservation tests are undercounted (F10).

For the owner to look at again:
- **The default domain of trust once any Agent's own tools are on.** Ruling 75 B3 moves the engine to a dedicated user and leaves Agents inside it.
- **ACC-057 against the purpose's "start a peer".** It is open as ruling 62 C7.
- **The fence's second limit for hooks** (F4).
- **Whether the duties of plan and apply bind anything** (F8).

**What the reader read.** Nothing was built or run and no file was changed. I ran no vectors, read no engine code and recounted no numbers.

**Read in full**
- docs/guides/full-review.md; AGENTS.md "What holds"; spec/README.md.
- docs/AUDIT.md: who decides, waiting for the owner, known to be open, purpose, preservation tests, scope guard.
- docs/guides/cold-readers.md, known-open list.
- spec/conventions/records.yaml; spec/promises/pending.yaml, sections.yaml and secrets.yaml.
- spec/implementation-defined.yaml; spec/core/templates.yaml.
- Ruling 75 and ruling 76 records; ruling 42's later answers.

**Read in part**
- spec/conventions/access.yaml: all but ACC-037.
- spec/promises/conduits.yaml: CON-004 to CON-031.
- spec/promises/providers.yaml: PRV-006 and PRV-019 to PRV-030.
- spec/conventions/lifecycle.yaml: LIF-009, LIF-013, LIF-015 to LIF-018, LIF-028, LIF-034 to LIF-041.
- spec/conventions/operations.yaml: OPS-012 to OPS-014, OPS-018, OPS-020 to OPS-024, OPS-028, OPS-030.
- spec/interface/implementation.yaml: IFC-006, IFC-007, IFC-022, IFC-025, IFC-032, IFC-042, IFC-049, IFC-050.
- spec/promises/contracts.yaml (CTR-013, CTR-014), packages.yaml (PKG-030 to PKG-032), templates.yaml (TPL-017), enrollment.yaml (ENR-003, ENR-004), first-start.yaml (FST-002, FST-007).
- spec/core/operations.yaml: open, close, unregister, retire, reconcile, register-replacement.
- spec/core/results.yaml (`inventory.v1`); the export table of spec/core/README.md; parts of spec/definitions.md.
- IN_FORCE.md: topics 6 and 9 and every line marked Open or Spec pending.

**Checked for novelty**
- AUDIT known-open, cold-readers known-open, the brief's queued list, IN_FORCE Open and Spec pending lines.
- By search only: the review list, increment 4's log, G5.md and order 077's Record.

**Not read**
- spec/promises: objects, schemas, naming, instantiation, manifest, submissions, pools, fleets, placement.
- spec/conventions: revisions, observation, implicit.
- Most of the interface file and the schemas.
- A promise in those files that contradicts a limit would have been missed.


## FR-192

**A failed undo of a wrong account holds only model work, so a Resource goes on serving under the wrong login**

X04/F1 · ambiguity · unsafe · verifier: confirmed, high · decides: coordinator · new

Same fault, or overlapping: FR-063.

**Promises.** PND-011, PND-005, PND-010, CTR-013

**Where.** spec/promises/pending.yaml:216

**Quote.** "If sign-out does not succeed, the engine reports SignInUndone=False with its cause and holds back all model work."

**Situation.** The limit at spec/README.md:411-413 repeats this: a failed undo holds model work. Model work is a turn of an Agent's model (spec/definitions.md:225), and sign-in also covers Resources (pending.yaml:166-167). Reading A: the hold is on that instance's model work, as PND-005 words its own hold (pending.yaml:148-150); for a Resource that holds nothing. Reading B: all model work of the engine stops because one sign-out failed. Steps to the harm under A: a ready Resource's login lapses; root signs in again and picks the wrong account; the account check finds another account; sign-out-hook fails; sign-in.v1 ends failed and the action stays pending; the Resource is still ready and signed in to the wrong account; every Agent with a Conduit to it acts in that account, and nothing is refused.

**Counterargument.** Only root signs a Resource in, the condition and the pending action are both reported, and a later account check clears it. At a first setup the failed sign-out part keeps the Resource from becoming ready, so the case needs a Resource that was ready before. Ruling 49's words speak of Agents' model work.

**Least change.** In PND-011's observable: "...and, until that condition clears, dispatches to that instance no request other than the parts of sign-in.v1, of a setup and of its retirement: every other request to it, an operation that starts its model work among them, reports unavailable and names the condition." Repeat the wording in README:411-413 and definitions.md:278.

**Precedent.** Kubernetes readiness: a Pod whose Ready condition is False receives no traffic from its Service until the condition clears.

**Who decides.** coordinator. An ambiguity with an unsafe reading (rulings 37; 46 point 4). Choosing the hold on the instance's own requests adds a hold for Resources, a small feature under ruling 71 point 4 that cannot move S1's counts; log it for the owner to strike, since ruling 49's words name model work only.

**Verifier.** confirmed, high. Quote verbatim at spec/promises/pending.yaml:216-217 (it wraps over two lines). The two readings are real: PND-005 words its own hold per instance, pending.yaml:148-150 "operations that start model work on that instance are not dispatched and report unavailable", while PND-011 says "all model work" with no scope. Nothing else in spec/ scopes it: `SignInUndone` occurs once in spec/ (pending.yaml:216); spec/definitions.md:278 says only "Failed undo holds model work"; spec/README.md:411-412 repeats it. Model work is "A turn of an Agent's model" (spec/definitions.md:225), and sign-in covers Resources (pending.yaml:165-167 "Resources remain root's"), so for a Resource the hold holds nothing. No promise stops dispatch to a ready Resource on this condition: hooks run "only as parts of sign-in.v1 or setup" (pending.yaml:219-220), and IFC-010 and IFC-016 (spec/interface/implementation.yaml:256-258, 429-432) only raise the action. New: experiments/012-model-checking/G5.md:170 leaves PND-011 to MC9; docs/implementation/SAFETY_CATALOGUE.md:347 speaks of model work only; not in docs/AUDIT.md "Known to be open", the review list or IN_FORCE.md:189. The non-binding source took reading A: docs/proposals/spec-batch-1/areas/refusals-and-identity.md:131 "Nothing that starts model work is dispatched to it".

**Verifier's corrections.** Class is ambiguity plus gap. The unsafe harm does not depend on the reading: under reading B new turns stop, but turns already in progress and Interfaces still reach the Resource, so choosing a reading closes nothing for a Resource. The fix has two parts with different deciders. Choosing the per-instance reading is the coordinator's (rulings 37; 46 point 4). Holding every request to an instance whose undo failed is new behavior: it stops a serving Resource. The coordinator may settle it under ruling 71 point 4, but it changes what the language says, so it also goes on the review list; ruling 49's answer (docs/proposals/ruling-49/README.md:24 "refuses and undoes a sign-in to any other") supports it. The same exposure arises with no sign-in.v1 at all: a further setup of a ready Resource whose undo part fails leaves it ready (spec/promises/templates.yaml:215-216, 228-230).


## FR-193

**The engine's own panel, on which the sign-in and enrollment limits rest, is defined by no promise**

X04/F2 · gap · unsafe · verifier: confirmed, high · decides: owner · new

Same fault, or overlapping: FR-141.

**Promises.** PND-004, PND-008, ENR-003, ENR-004, IFC-049, PND-002

**Where.** spec/promises/pending.yaml:76

**Quote.** "The engine alone renders evidence at its panel. Values of kind code or fingerprint appear in no elicitation message, tool result, _meta, notification or record of a request; they are shown only in the engine's own panel and recorded with the answer."

**Situation.** The limit at spec/README.md:402-408 makes the panel the defence against a model-driven console: the person compares the engine panel's own rendering. Codes, fingerprints and join actions that carry a secret (enrollment.yaml:73-74) are barred from every carrier of the wire, so the panel is a surface outside the protocol. No promise, definition (definitions.md:267 calls a control panel an Interface) or implementation-defined item says what it is, over which channel it renders, or how it establishes that the viewer is an Interface at which the action is raised, root alone for PND-008's kinds. A scoped Interface told that a device code is in the engine's panel (implementation.yaml:1369-1370) has no stated panel. Steps: an engine renders its panel as a page on loopback, or in the terminal that a model-driven console also reads; it keeps every promise; a run of another operating-system user, or the console's model, reads a join secret or shows the person a fingerprint of its choosing.

**Counterargument.** In v0 one person sits at the engine's machine, and the natural build prints to that person's terminal, inside the stated domain of trust. Rulings 50 and 56 say the Interface is the control panel and the engine alone shows evidence. A careful builder would not publish it.

**Least change.** One definition and one promise: "The engine's own panel is the engine's own rendering to a person, outside the protocol's wire. It shows an action's evidence and prompt only to a viewer that it has established, under IFC-022, as an Interface at which the action is raised under PND-002, and as root for the kinds of PND-008, over a channel that no run and no other operating-system user of the machine can read." How it is reached becomes an item of the implementation-defined list.

**Precedent.** GnuPG's pinentry: a separate prompt that gpg-agent alone drives, so the calling program never sees the passphrase. RFC 8628 has the device show the user code on its own display.

**Who decides.** owner. It defines a surface and who may see it, which is behavior; ruling 56 (who shows it) and ruling 50 point 3 hold the owner's words.

**Verifier.** confirmed, high. Quote verbatim at spec/promises/pending.yaml:76-79. No text in spec/ says what the panel is, how it is reached or how it knows its viewer. spec/definitions.md:267 defines only "control panel | An Interface at which a person sees and answers pending actions"; `panel` does not occur in spec/implementation-defined.yaml. Yet the promises make it the only surface for these values: pending.yaml:343-344 "Only root's own panel shows their machine evidence, code, fingerprint or join action"; pending.yaml:353-354 "Tools list only ID, kind, subject, expiry and state, never a pending machine's evidence, even at root"; spec/promises/enrollment.yaml:73-74. IFC-049 tells any signing Interface "for device it says the code is in the engine's panel" (spec/interface/implementation.yaml:1369-1370), and PND-010 lets a scoped Interface sign in its Fleet's Agents (pending.yaml:165-166). The only statement of the channel is a plan, which binds nothing: docs/proposals/spec-batch-1/README.md:643 "over a root-only channel that no MCP tool exposes" (order 051c, not written). Not in docs/AUDIT.md "Known to be open", docs/guides/cold-readers.md, the queued list of docs/guides/full-review.md, or any Open or Spec pending line of IN_FORCE.md (lines 183-185, 221 cite the panel as written).

**Verifier's corrections.** The unsafe mark stands, by a sharper path than the one given. ENR-006 check 8 (spec/promises/enrollment.yaml:143 "the request carries fingerprint") admits a machine only on a fingerprint the person read on the machine, and PND-008 keeps that value from root's own tools. If a conforming engine's panel is output that the model at root's console can read, such as a command's output, the model supplies the fingerprint itself and a machine nobody checked becomes a node. The other path, another operating-system user reading a loopback page, is weaker: "root's own panel" can fairly be read to exclude a surface that does not establish root. Decider stays the owner. Ruling 56 point 4's words are "at the root Interface's control panel" (docs/proposals/ruling-56/README.md:23), so the separate surface is itself the coordinator's reading; put the plan's sentence to the owner as the recommended answer.


## FR-194

**No result tells the user that a consumer is inside the engine's domain of trust; its restrictions are still reported as mediated**

X04/F3 · defect · verifier: weakened, medium · decides: owner · known-new-situation

**Promises.** ACC-027, IFC-022, PRV-029, LIF-013, CTR-014, PRV-006

**Where.** spec/conventions/access.yaml:451

**Quote.** "This version reports mediated for every restriction, whatever the consumer's runtime provider is."

**Situation.** Mediated means the restriction applies to requests that pass through the engine (definitions.md:193). A consumer run by the engine's own runner as the engine's operating-system user, which is what an instance gets when it names no runtime, can read root's token and the engine's keys and present as root or as any such instance (implementation.yaml:623-627; README:424-425). Its requests then pass through the engine with none of its own restrictions applied. Inspection reports the same word for it as for a consumer on another machine, and `ownTools.enforcement: neither` in both cases (lifecycle.yaml:273-275); the inventory names no runtime (lifecycle.yaml:289-290). To learn that an Agent can become root, a user must combine `directoryNamed`, an absent `runtime` and DL16 in the conformance document. PRV-029 also calls path isolation among such processes mediated (providers.yaml:978-979), which no enforcement point mediates. Preservation test 9 is met in form only: the report never varies.

**Counterargument.** Mediated is true by its definition for the requests the consumer makes as itself, the limit is stated plainly in README:500-509 and in IFC-022, and reporting the weaker word is the cautious choice. No confining provider ships in v0, so there is nothing to tell apart yet.

**Least change.** In LIF-013: the status of an Agent or Resource that has an item that is run says whether its run is inside the engine's domain of trust: true where the engine's own runner runs it as the engine's operating-system user, under DL16. Inspecting a Conduit whose consumer is such an instance says so beside each restriction. In PRV-029 replace "isolation is mediated" with "the directory is kept apart by its path alone, neither enforced nor mediated".

**Precedent.** Kubernetes shows `privileged` and `hostPID` on the Pod and its Pod Security Admission warns at admission without rejecting; Docker's inspect shows `HostConfig.Privileged`.

**Who decides.** owner. It adds a reported member and changes what the two-valued report of preservation test 9 (ruling 17) tells a user. The PRV-029 wording alone is the coordinator's (ruling 37).

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/access.yaml:451-452. The facts hold: IFC-022 at spec/interface/implementation.yaml:623-627; the limit at spec/README.md:500-509; LIF-013 at spec/conventions/lifecycle.yaml:289-290 "these members name no runtime provider". But the text is not wrong by its own terms. "Mediated" is defined as applying "to requests that pass through an enforcement point" (spec/definitions.md:193), and ACC-027 itself says of this consumer, access.yaml:454-456, "is a consumer that is not confined: it may reach by its own means what a target's implementation reaches". Inspection already shows `enforcement: neither` and the directory, and `runtime` in `declared`. Leaving out a runtime member was a logged decision: docs/work-orders/075-spec-own-tools-reach.md:289-292 "No runtime member yet... A member that names a confining provider comes with the first such provider (ruling 75, B2)". PRV-029 at spec/promises/providers.yaml:977-979 does say "isolation is mediated (ACC-027, DL16)".

**Verifier's corrections.** Class: not a defect of ACC-027. It is a ruling 52 shortfall on a stated limit: no result takes the step from unconfined to able to present as root. What is a defect, and new, is PRV-029's use of the defined word "mediated" for a directory that no enforcement point stands before; that rewording is the coordinator's (ruling 37). The reported member stays the owner's, and the question should say that it reverses order 075's logged choice. The finding is also wider than stated: a Resource's package on the engine's own runner is equally inside the domain of trust and has no `ownTools` member at all, so an Agent-only member would not cover it.


## FR-195

**The fence's second limit names a way on that a setup hook does not have**

X04/F4 · gap · verifier: confirmed, medium · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-225.

**Promises.** OPS-014, OPS-030, OPS-021, OPS-022, OPS-028, TPL-017, LIF-028

**Where.** spec/conventions/operations.yaml:373

**Quote.** "An attempt that none of these events fences, such as one sent to a plain server reached by `remote` that never answered, is never fenced: its invocation can be recorded as having occurred, or stays unknown under OPS-013, and the Agent's way on is a new request."

**Situation.** A setup hook may be dispatched to an item reached by `remote` (templates.yaml:277-279). Its response is lost: the hook ends unknown and nothing can ever fence it, since a `remote` server has no run to stop. No actor sends a hook, and its idempotency key is fixed per instance (operations.yaml:650-652), so there is no new request. OPS-021 never dispatches it again while unknown (operations.yaml:450-452), and every further setup ends unknown (operations.yaml:477-480). An actor who has checked that the hook never ran can neither record that (OPS-030, conflict, for good) nor have it sent again. The two exits are to record the hook or the setup as having occurred, which the actor knows to be false, or to retire the instance and register another, with every Conduit and tombstone that costs. Before order 077 the actor could record failed and the next setup sent a new attempt. The first limit's remedy also fails for a plain server: it is given no key to pass on (IFC-007), so its hop cannot handle its own idempotency.

**Counterargument.** The owner accepted both limits on 2026-11-12, and replacing the instance is an honest way on. A late answer on a connection still open fences the attempt. Hooks on remote items are a minority.

**Least change.** Add to OPS-014's observable and to OPS-030's message: "A hook is sent by no request, so there is no new request for it: every further setup ends unknown under OPS-022 until the hook is recorded as having occurred, where the actor checked that it did; otherwise the instance is retired and another is registered in its place." Whether a Type may declare its hooks deduplicating, so that OPS-012 sends an unknown hook again with OPS-028's key, is a question for the owner.

**Precedent.** Kubernetes: a Pod left Unknown on an unreachable node is force-deleted and replaced. Stripe: a client retries with the same idempotency key and the server answers from the first attempt.

**Who decides.** coordinator. Stating the consequence adds no behavior (rulings 37; 46 point 4). Letting a hook be sent again while unknown adds a feature and goes to the owner in the same ruling as ruling 75 A1.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/operations.yaml:373-376. Each step holds. A hook may go to a `remote` item (spec/promises/templates.yaml:279). "A server reached by `remote` has no run" (operations.yaml:366). OPS-021 at operations.yaml:450-451 "A hook whose outcome is unknown receives no further call however often the instance is set up or retired, until it is reconciled". OPS-022 at operations.yaml:477-479 gives every further setup the outcome unknown "even where it dispatched nothing". OPS-028 at operations.yaml:651 fixes the key "the same in every setup of that instance". OPS-030 at operations.yaml:737-739 repeats "the Agent's way on is a new request". The owner's answer covered an Agent's call only (docs/proposals/ruling-75/README.md:121-124); IN_FORCE.md:150 records the two limits as written. Order 077's file mentions hooks only at lines 217 and 282, neither on this case.

**Verifier's corrections.** Class: the promises do decide the situation (it stays unknown), so this is a misleading statement plus a cost, not an undecided case: the way on that OPS-014 and OPS-030 name does not exist for a hook. The situation is wider than `remote`. A setup hook on an item that is run, whose call the engine stopped waiting for, is fenced only when the process ends or answers (operations.yaml:357-364), and no operation ends the run of an instance short of retiring it (spec/README.md:540-541 says so for a ready one; PRV-007 leaves a running run as it is). On the engine's own runner the person can end the process outside Conduit; under another runtime provider nothing can. Deciders as stated: the sentence is the coordinator's; sending a hook again is the owner's.


## FR-196

**After the stated exposure nothing ends root's credential or replaces the signing keys**

X04/F5 · gap · verifier: confirmed, medium · decides: coordinator · known-new-situation

**Promises.** IFC-022, IFC-032, IFC-042, PRV-022, LIF-034, FST-007

**Where.** spec/README.md:558

**Quote.** "Once the instance is removed it establishes no instance, since the engine checks on every request that the instance it names still exists, which is the only revocation (IFC-022, IFC-032, DL13)."

**Situation.** README:424-425 says a process of the engine's operating-system user can read root's token; README:500-502 says it can read the engine's keys. An unconfined Agent copies either off the machine. The owner retires and unregisters the Agent. The only revocation is removal of the instance a credential names, and root is never retired or unregistered (lifecycle.yaml:698-705); the engine accepts a token at its `https` address from any machine (IFC-042). The lifetime of an Interface's token and any replacement of the signing keys are left to DL13, with no floor, and the trust domain never changes, repeated first starts included (first-start.yaml:105, 111-112). So no promise gives the person any act that ends the copy's access, and the limit states the exposure without saying that recovery means a new engine store. The limit that the person's Interface runs on the engine's machine (README:429) binds nothing either.

**Counterargument.** A same-user program is inside the stated domain of trust, a single developer can wipe the store, and an engine's DL13 choice may issue short tokens through the protocol's authorization.

**Least change.** Add to the limit: "No request ends a credential other than by removing its instance, and root is never removed. After a program of the engine's operating-system user is no longer trusted, the remedy in this version is a new engine store, with a new trust domain." Add to DL13: the conformance document states how long a consumer's token lasts and how the user replaces the signing keys.

**Precedent.** SPIFFE and SPIRE issue short-lived SVIDs and rotate the trust bundle; OAuth 2.0 token revocation, RFC 7009.

**Who decides.** coordinator. Stating the consequence and adding a documented choice add no engine behavior (rulings 37; 46 point 4). A revocation or rotation operation would be a feature for the owner.

**Verifier.** confirmed, medium. Quote verbatim at spec/README.md:558-560. Root is never removed: LIF-034 at spec/conventions/lifecycle.yaml:698-705 "The request is not dispatched, and reports refused". The token works from anywhere: spec/interface/implementation.yaml:1192-1193 "it accepts the token at its `https` address whichever machine a request comes from". DL13 at spec/implementation-defined.yaml:155-158 "the engine's check on every request that the instance still exists is the only revocation. Each root, and each key in `jwks.json`, is kept for as long as a run that holds it lasts". DL13 states no lifetime for an Interface's token. FST-007 at spec/promises/first-start.yaml:104, 111-112 "It never changes." The exposure is stated at spec/README.md:424-425 and 500-502; the consequence for root and the keys is in no record (docs/AUDIT.md:473 and docs/proposals/multi-machine/README.md:210-218 speak of runs' credentials and v1 renewal only).

**Verifier's corrections.** DL13's sentence on keeping each root and key while a run holds it makes the finding slightly stronger than stated: a key cannot be dropped while any instance given it still runs, so replacing keys means replacing those runs. Severity medium, not higher: the exposed programs are inside the stated domain of trust, and the fix is a sentence and one documented choice. Deciders as stated.


## FR-197

**Ruling 75 C1's placement rule is checked and reported by nothing, and the first user's one machine cannot meet it**

X04/F6 · gap · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** PRV-029, CTR-013, IFC-049, PND-010, SEC-010, LIF-013

**Where.** spec/README.md:426

**Quote.** "Until Agents are confined, a package keeping a Resource's login runs where no unconfined Agent runs; the engine does not check that placement in v0 (ruling 75 C1)."

**Situation.** The first user runs everything on one machine (docs/AUDIT.md:262), and Conduit's own work runs with Agents' own tools on (docs/AUDIT.md:333-335). A Resource whose Type states signIn, placed on `local` with no `runtime`, keeps its token in a state directory that every such Agent can read (providers.yaml:977-979). Root signs it in: the message the engine writes names the instance, its node, the host and the account (implementation.yaml:1367-1370) and says nothing of the Agents that share the node. The engine holds every fact needed: which Types state signIn, each Agent's `ownTools.directoryNamed`, each instance's node, hostname label and runtime. The same holds when the engine reads a secret for a run on that runner, and when a template binds a secret to a package argument, which every user of the machine can list (README:531-532; no promise states that part). The purpose's first point, a credential never handed over, then fails with no result saying so.

**Counterargument.** The rule is the owner's own and IN_FORCE records it as unchecked in v0. The engine cannot know the operating-system user under another runtime provider, so a check could only warn. Inspection already shows each Agent's own tools.

**Least change.** A diagnostic that does not reject, as CTR-004 and TPL-004 give one, on `set-up.v1` and `sign-in.v1` of a Resource whose Type states signIn: it names each Agent whose `ownTools.directoryNamed` is true, or whose Type states no `toolsDirectory`, whose node reports the same hostname label and whose runtime is the same. IFC-049's message names them too. The same diagnostic on registering a template whose item binds a secret reference to a package argument.

**Precedent.** Kubernetes Pod Security Admission's warn mode. OpenSSH refuses a private key file that others can read and says why.

**Who decides.** coordinator. A small finding that adds a diagnostic (ruling 71 point 4), in service of a rule the owner already made (ruling 75 C1) and of ruling 52; logged for the owner to strike.

**Verifier.** weakened, medium. Quote verbatim at spec/README.md:426-427. That nothing checks the placement is the recorded limit itself: the same sentence says "the engine does not check that placement in v0", and IN_FORCE.md:194 records "placement away from unconfined Agents, unchecked in v0 (75 C1)". The owner's rule is at docs/proposals/ruling-75/README.md:52-54. The rest holds: PRV-029 at spec/promises/providers.yaml:977-979; IFC-049's message at spec/interface/implementation.yaml:1367-1370 names the instance, node, host and account and no Agent. The plan for S2 knows the setting: experiments/013-s2-shared-login/plan.md:210-211 "unconfined on the Mac as the engine's own runner starts it".

**Verifier's corrections.** Two parts are smaller than stated. "Checked by nothing" is known, at the two places above. "The first user's one machine cannot meet it" is too wide: the rule is met wherever no Agent on the node names a directory, as S1's Agents do not (ruling 75 B1, docs/proposals/ruling-75/README.md:39-42). It cannot be met on one machine once any Agent's own tools are on, since no confining provider ships; that is the new situation. What survives is the ruling 52 point: no result at `set-up.v1` or `sign-in.v1` says that an unconfined Agent shares the Resource's node. Class: a silent limit, not an undecided situation. Decider: the coordinator may add the diagnostic under ruling 71 point 4, but it is a check where IN_FORCE.md:194 and README:427 say there is none, so both lines change with it and it is logged to strike. Unsafe mark: arguable either way; the leak is the stated limit's harm and not one this finding adds, so false can stand.


## FR-198

**Unregistering reports success and nothing else when its last stop failed or was never sent**

X04/F7 · gap · verifier: confirmed, medium · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-014, FR-232.

**Promises.** LIF-036, LIF-017, LIF-009, PRV-021, IFC-025, IFC-050

**Where.** spec/conventions/lifecycle.yaml:765

**Quote.** "the request to unregister asserts that the instance's shutdown is settled in another way, and the outcome of the request is that of the removal."

**Situation.** The limit (README:582-605, filed under secrets) lets a run go on beside a later instance with the values it was given. The moment it becomes final is `unregister.v1`: the stop it asks for ends unknown or failed, or is not dispatched because the provider is not ready, and is then never asked for again (lifecycle.yaml:763-769). The request reports succeeded. `unregister.v1` declares no outputs (core/operations.yaml:258-269), and a result that is not an error carries no `structuredContent` (implementation.yaml:669-670), so the caller is told nothing; the engine keeps no state of the run. The only trace is the record of a provider call on an object that no longer exists. A replacement registered and set up afterwards gets no word either, unless the provider's contract happens to hold the owed stop.

**Counterargument.** The caller already saw the retirement end unknown or failed and is asserting that shutdown is settled elsewhere. The stop's record survives, and every runtime provider that Conduit ships refuses a start beside a stop it still owes (ruling 42's later answer).

**Least change.** Give `unregister.v1` the output `diagnostics`, as `register.v1` has. In LIF-036: "Where the stop ended unknown or failed, or was not dispatched, the result carries a diagnostic that does not reject, naming LIF-036, how the stop ended and the machine, and saying that the run may go on with the values it was given."

**Precedent.** kubectl on a forced deletion: "Immediate deletion does not wait for confirmation that the running resource has been terminated. The resource may continue to run on the cluster indefinitely."

**Who decides.** coordinator. A small finding that adds an output (ruling 71 point 4), required by ruling 52; it cannot move S1's author counts.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/lifecycle.yaml:765-766. `unregister.v1` has no `outputSchema` (spec/core/operations.yaml:244-269: `inputSchema`, `effect`, `deduplicates` only). IFC-025 at spec/interface/implementation.yaml:669-670 "Where it declares no outputs, the result carries no `structuredContent`". LIF-036 at lifecycle.yaml:767-770 "keeps no state of the run, so a stop that the provider never received... is not asked for again. A stop that ended unknown or failed, or was not dispatched, does not prevent removal." IFC-050's observable (implementation.yaml:1343-1346) names only reasons and failed or unknown outcomes, so nothing requires a succeeded result to say how a part ended. The owner's answer of 2026-11-12 (docs/proposals/ruling-42/README.md:275-285) and IN_FORCE.md:245 cover the overlap, not what the result says.

**Verifier's corrections.** None to the substance. One note for the fix: IFC-050's `then` ("says what happened and why in full", implementation.yaml:1339-1341) can be read to require this already, so two builders would differ; the least change may be a sentence in LIF-036 and IFC-050's observable that the result's `content` states how the stop ended, with the `diagnostics` output as the structured form. Adding an output changes a core `.v1` contract's tool definition, which the log entry should say.


## FR-199

**The limits section states duties of plan and apply while saying it holds no promise, and a promise leans on them**

X04/F8 · contradiction · verifier: weakened, low · decides: owner · new

**Promises.** SCT-003, SCT-015, PKG-031, CTR-014, ACC-029

**Where.** spec/README.md:333

**Quote.** "They are statements, not promises, and nothing tests them; each point cites the promises that state its parts."

**Situation.** The first section then states rules that no promise holds: every dangerous act needs a named flag (README:352-353), a plan prints and counts every widening and prints `ownTools` beside each Agent (README:357-361), plans print affected bounds, and it calls these tooling duties (README:369-370). No conformance level has a tool as its claimant (README:312-322), and README:113-120 says a tool's commands belong to the tool. Reading 1: they bind, and are promises with no ID, layer, level or vector. Reading 2: they bind nothing; then IN_FORCE.md:77 marks rulings 53 point 4, 55 A4, 59 A3 and 61 B11 as in spec/ on the strength of text that binds no one, and PKG-031 lets a release widen a standard template because a plan prints any widening (packages.yaml:438-440). An actor who registers over the wire, or with another tool, sees no widening, no `ownTools` line and no bound warning: for those limits the plan is the only moment the user would meet them.

**Counterargument.** Plan and apply are deliberately tooling made of existing requests (ruling 76), the reference under docs/implementation says it binds nothing, and inspection shows operations and `ownTools` to anyone who looks.

**Least change.** Either move the duties to the tool's reference and say here only that no declaration sets up, opens, prunes or lifts, striking the plan clause from PKG-031's observable; or write them as promises that bind a tool and add a conformance level for one. Where a duty must hold for every client, the engine states it in the result of the request, such as a diagnostic on a registration that widens a Conduit.

**Precedent.** Kubernetes moved apply and dry-run into the API server (server-side apply, `dryRun=All`) so that they hold for every client; Terraform's plan is the tool's own and binds no other tool.

**Who decides.** owner. Whether a tool is bound, and by which conformance level, is scope and adds or removes a feature. The coordinator may strike PKG-031's reliance meanwhile (ruling 37).

**Verifier.** weakened, low. Quote verbatim at spec/README.md:333-334. The cited text is as described: README:352-353 "Every dangerous act needs a named flag, counted when used"; README:369-370 "These are tooling duties and limits of the registration equivalence, not further engine operations"; PKG-031 at spec/promises/packages.yaml:438-440 "a plan prints and counts any widening of the resolved Conduit operations before apply"; the three conformance levels at README:312-322 name no tool. Against a contradiction: the section is not a promise, spec/promises/sections.yaml:6 says "Plan and apply remain tooling made of existing requests", and IN_FORCE.md:77 restates the four rulings' decisions in full, so they are in force for Conduit's own tool whatever the section's standing (AGENTS.md:47-50). Ruling 76 point 5 already treats the plan as optional tooling (docs/proposals/ruling-76/README.md:35).

**Verifier's corrections.** Class: not a contradiction. It is an ambiguity of standing (the word "duties" in a section that says nothing in it binds) and one small defect: PKG-031, a promise, states in its observable something that no claimant of any conformance level can show. Both are wording fixes that add no behavior, so the coordinator may make them: say that the paragraph describes Conduit's own tool, and strike or reword the plan clause in PKG-031. The owner is needed only if a tool is to be bound by a conformance level, which ruling 76 does not ask for. PKG-031 is a `should` that keeps the operation list; "lets a release widen" overstates it.


## FR-200

**An Agent Type that declares nothing about its own tools gets the mildest report**

X04/F9 · defect · verifier: confirmed, low · decides: coordinator · known-new-situation

**Promises.** CTR-014, LIF-013, ACC-027

**Where.** spec/conventions/lifecycle.yaml:267

**Quote.** "Where it is false the Type declares nothing about such reach, and the member holds none of the following fields."

**Situation.** README:87-88 says this version reports the reach of own tools as neither enforced nor mediated. The report carries `enforcement: neither` only where the Type states `toolsDirectory` and the value is a string (core/results.yaml:378-381, 402-405). A third-party Agent Type whose client has a shell and that states no `toolsDirectory`, which CTR-014 allows (contracts.yaml:365-366), inspects and plans as `ownTools: {stated: false}`. The owner reads that as no own tools, gives turns on the Agent, and it runs unconfined on the engine's own runner. The Type that declares honestly prints a directory and `neither`; the one that omits the line prints less. The same holds for `directoryNamed: false`, where the text itself says the Agent may still reach outside Conduit (lifecycle.yaml:275-278). Anything that flags Agents by `enforcement` misses both.

**Counterargument.** `stated: false` is literally true, the engine cannot know what a client holds, the Type's description is carried into every tool description, and Conduit's own Types follow ruling 75 B1, under which forgetting the line is safe.

**Least change.** In LIF-013 and `inventory.v1`: `enforcement` is required beside `stated` and is `neither` for every Agent, since this version confines none; where `stated` is false the text says that the Type declares nothing and that the Agent's reach outside Conduit is unknown.

**Precedent.** Kubernetes Pod Security Standards, restricted profile: a field left unset, such as `runAsNonRoot`, does not meet the profile; absence is read as the unsafe case.

**Who decides.** coordinator. A small finding that adds a field's presence (ruling 71 point 4), under ruling 52; it changes no author line of S1.

**Verifier.** confirmed, low. Quote verbatim at spec/conventions/lifecycle.yaml:267-268. The schema agrees: spec/core/results.yaml:380-381 "directory and enforcement are present exactly when directoryNamed is true", and results.yaml:402-405 requires only `stated`. CTR-014 allows the silent Type: spec/promises/contracts.yaml:365-366 "A Type that states no `toolsDirectory` declares nothing about such reach, and inspection says so". spec/README.md:87-88 says without condition "This version reports the reach of own tools as neither enforced nor mediated", which LIF-013 does only where a directory is named. The form was a logged choice, not an open point: docs/work-orders/075-spec-own-tools-reach.md:285-288 "is the admin's reading of ruling 52, point 2". It is not on the review list.

**Verifier's corrections.** Severity low: at this commit no Type under packages/ states `toolsDirectory`, and the only Types that state nothing are the `.v1` ones whose tools are off, for which the mild report is accurate; the harm needs a third-party Type. For the fix, `neither` beside `stated: false` would itself assert something the engine does not know; a value or text that says the reach is undeclared and unknown fits better. README:87-88 should be narrowed to match LIF-013 in the same change.


## FR-201

**The limits name two exceptions to the preservation tests; the promises make more to test 5**

X04/F10 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** ACC-032, ACC-057, SEC-008, ACC-054, PND-009, PND-010, FLEET-005

**Where.** spec/README.md:380

**Quote.** "Two named exceptions to the [preservation tests]\(../docs/AUDIT.md#preservation-tests) apply."

**Situation.** Test 5 says every management action is available to an agent under a grant (docs/AUDIT.md:302). Beside the named exception for sign-in and answers, no Agent can ever hold: the five operations reserved to root (access.yaml:579-580), registering an Agent (access.yaml:897-900), opening a Conduit that reads a secret (secrets.yaml:198-200), or opening a root-consumed Conduit (ACC-054). ACC-057 is stated two sentences earlier as a fact and not as an exception, though it removes "start a peer" from what the purpose says must survive (docs/AUDIT.md:277-278); the broker of README:477-487 cannot register an Agent either, since its request is an onward one. A reader who takes the section as the list of departures from the tests, as the owner would when judging purpose, undercounts them.

**Counterargument.** The sentence sits in the Fleet section and may mean only that section's two; the reservations are older rulings, IN_FORCE.md:25 says root-only powers open later, and registration through a Fleet is already open as ruling 62 C7.

**Least change.** Reword: "These exceptions to the preservation tests apply in this version", and list for test 5 the operations reserved to root (ACC-032), registering an Agent (ACC-057), opening a Conduit that lists `get-secret.v1` (SEC-008) and sign-in and answers (PND-009, PND-010); for test 2, root's defaults; for test 9, that every restriction reports mediated (ACC-027).

**Who decides.** coordinator. A defect of statement whose fix adds no behavior (ruling 37; ruling 46 point 4). Whether ACC-057 should stand against the purpose stays the owner's, with ruling 62 C7.

**Verifier.** weakened, low. Quote verbatim at spec/README.md:380. The reservations exist as cited: ACC-032 at spec/conventions/access.yaml:579-580 "The five are declare-lost.v1, purge.v1, enroll-machine.v1, admit-machine.v1 and refuse-machine.v1"; ACC-057 at access.yaml:893-904; SEC-008 at spec/promises/secrets.yaml:191-200. But they are already recorded against test 5: IN_FORCE.md:21 "Rulings limit this in v0. Only root registers Agents (59 A1) and admits machines (56 point 11)... Enrollment and secrets stay with root (66 point 3)", and IN_FORCE.md:25 says powers given to root alone are to open later. Registration through a Fleet is open as ruling 62 C7 (IN_FORCE.md:178).

**Verifier's corrections.** The substance is known and ruled; what is left is one sentence. Read in its section, "Two named exceptions" can mean that section's two, and the preservation tests are not part of spec/. The fix is the coordinator's rewording so the sentence does not read as the whole list, with a pointer to IN_FORCE.md:21, without restating every reservation in the limits. Listing ACC-027 as an exception to test 9 is not right: the report is made; it only never varies.


## Further points the verifier noticed

Not verified by a second reader.

- **Polling after sign-in-hook cannot tell an account that was already signed in from one the person just chose** (spec/promises/pending.yaml:183-185 (PND-010) with 209-211 (PND-011)). Noticed while checking F1; not searched further, medium confidence. PND-010: "that initial mismatch does not undo the account or end sign-in.v1. Each retry polls account-hook.v1 to check the account after the person's sign-in, under PND-011, until an account appears". PND-011: during polling "a different account is undone by a sign-out-hook.v1 part, sign-in.v1 ends failed". Where the instance is already signed in to another account, the first poll sees that same account before the person has finished. One builder undoes it and ends failed at once, and the person's login then completes with no check, since no hook runs later. Another waits for the account to change. Class ambiguity. For a Resource the unchecked login then serves, as in F1.

## The verifier's notes

Checked at commit 2eb8f258 in the read-only tree; nothing was built, run or changed. All ten quotes are verbatim at the cited lines (several wrap onto the next line), and every secondary line citation I opened matched.

Result: 6 confirmed (F1, F2, F4, F5, F7, F9), 4 weakened (F3, F6, F8, F10), none refuted, none wholly known. Severity: F1 and F2 high; F3, F4, F5, F6 and F7 medium; F8, F9 and F10 low.

The two unsafe marks stand. For F1 the harm is a gap that holds under both readings, so settling the ambiguity alone does not close it. For F2 the strongest path is through ENR-006's typed fingerprint.

The weakened ones share a pattern: the fact is a stated or ruled limit, and what survives is that no result says it at the moment it matters (ruling 52), or a wording fix.
- F3 and F9 both revisit choices logged in order 075 (no runtime member; `stated: false`). Neither is on the review list, so any question to the owner should say so.
- F6's core, that the placement is unchecked, is recorded at spec/README.md:427 and IN_FORCE.md:194.
- F10's reservations are recorded at IN_FORCE.md:21.

I did not re-verify the assessment's ranking as a whole. Its spot claims that I opened hold: `open.v1` opens only a registered Conduit (spec/core/operations.yaml:752-753), and ACC-057 refuses an onward request serving root. One historical claim in F4, that before order 077 an actor could record failed and get a new attempt, was not checked against history.
