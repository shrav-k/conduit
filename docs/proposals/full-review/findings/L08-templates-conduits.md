# L08-templates-conduits: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The document rules of this slice (INS-001 to INS-025, TPL-001 to TPL-005 and TPL-019, CON-016 to CON-028, POOL-001 to POOL-012) are clear, testable and rightly `language`. I found nothing by which a promise-keeping engine opens more than a document declares, except through the undefined word "live" on ACC-033's owner rows (F1). The weakest point is the life of nested instances: a declared Conduit holds its target by ID, its setup part reports conflict once that Conduit is closed, and no replacement can be made when the target is gone, which strands the consumer (F3). The records already hold the general point as finding 4-01 and as a later candidate for removing declared Conduits. The form is not minimal: `observable` fields carry rules, cross-references and rationale (TPL-018's runs to 26 lines), and in four places they state a rule no `then` holds or contradict their own `then` (F2, F4, F7, F9), while TPL-012, TPL-018, CON-008, FLEET-001 and FLEET-005 each state several obligations. Layering is mostly right; FLEET-005 is the one promise in the wrong layer, and INS-016 and INS-017 claim a purity that placement by the machine provider breaks. I would cut the `/operations` input destination with INS-023, which no core template, example or package uses. I would state once the twins that the Pool and Fleet split produces (POOL-001 and FLEET-002, POOL-004 and FLEET-003, ACC-028 and FLEET-004, POOL-003 and LIF-029, CON-011 and CON-014). A Fleet's template, `from` and setup carry nothing (one legal template, a setup with no parts), though the family is ruled (59, 60). On ruling 52, a setup that succeeds says nothing of an optional part that failed.

**What the reader read.** Tree: the repository root at 2eb8f258 (2026-11-12), read-only; no build, no cargo, nothing written.

Read in full, line by line:
- spec/promises/templates.yaml (359 lines), instantiation.yaml (372), conduits.yaml (410), pools.yaml (254), fleets.yaml (115)
- spec/definitions.md (467 lines, including "Words the promises do not use")
- spec/README.md (678 lines)
- docs/guides/full-review.md
- AGENTS.md, including "What holds"

Known-open records checked:
- docs/AUDIT.md lines 120-246 (who decides; waiting for the owner; known to be open) and its unsafe test
- docs/guides/cold-readers.md lines 36-192
- IN_FORCE.md lines 60-110, plus a search of every line marked Open or Spec pending; not read in full
- Searched for my slice's promise IDs: increment 4's log (4-01, 4-04, 4-07 and 4-11 read), the review list, G5.md, round 6's design critique, LANGUAGE_DEFECTS.md, the Fleet brief, the simplification record and the short-form synthesis

Cited promises opened and read whole:
- OBJ-002, -003, -013 to -016, -019, -023, -025
- SUB-002, -003, -006
- SCT-001, -005 to -009, -011
- NAM-018, -022
- CTR-003, -007
- PLC-002, -005, -013, -018
- PRV-007, -015, -026
- LIF-003 to -005, -008, -009, -013 to -021, -023, -029, -035, -040, -041
- ACC-003, -005, -006, -012 (then and observable), -024, -028, -033, -057
- OPS-022, -023
- REC-001
- IFC-012 (first 30 lines), -014, -025, -050
- PND-005, -011

Core package and schemas opened:
- The `set-up.v1` contract and the head of `retire.v1`, and which core operations declare an `outputSchema`
- The templates `root-pool` and `root-fleet`
- `reference.v1` and `conduit-template-spec.v1`
- spec/schemas/README.md lines 120-200, and the head comment of spec/schemas/templates.yaml

Cited but not opened; I relied on how the slice and definitions.md quote them:
- LIF-024 to -028, LIF-036
- ACC-029, -030, -039, -054
- OPS-029
- PLC-007, -017, -023, -024
- CTR-004, -006, -013
- SCH-009, PKG-031, FST-001, FST-002

Not read:
- The rest of the conventions, providers.yaml, placement.yaml, secrets.yaml and the interface file beyond the promises listed
- Engine code and experiments
- Vectors, apart from one vector's expected reason and a search for uses of `/operations`
- Ruling records, apart from ruling 59's decision table and five ledger rows


## FR-091

**"Live existing" owner is undefined in FLEET-003 and ACC-033 rows 1 and 1A; one sound reading drops the owner's Conduit**

L08/F1 · ambiguity · unsafe · verifier: weakened, low · decides: coordinator · new

**Promises.** FLEET-003, ACC-033, POOL-004, CON-024

**Where.** spec/promises/fleets.yaml:52

**Quote.** "Registering into a live existing Fleet needs both `management` and that Fleet (ACC-033 row 1A). With the Fleet in the same submission, it needs `management` alone (row 2)."

**Situation.** "Live" decides which row of ACC-033 authorizes a registration (spec/conventions/access.yaml:595-598: "a live existing Pool", "a live existing Fleet") and is defined nowhere; definitions.md uses it in the row `management` without a row of its own. The lifecycle states are registered, ready and retired. Reading A: live means not retired. Reading B: live means ready; the slice itself heads CON-012 to CON-014 "Conduits to live instances" (spec/promises/conduits.yaml:155), and CON-013 and CON-014 open with "whose consumer is ready". Steps to the harm under B: (1) Pool `shared` is registered and never set up; nothing needs its setup, because management verbs are dispatched while a Pool or Fleet is registered (CON-024, conduits.yaml:333-338), and a Fleet's setup has no parts (FLEET-001). (2) An actor is the consumer of a Conduit to `management` listing `register.v1` and of none to `shared`. (3) It registers a Resource with `owner: shared`. Row 1 does not fit a Pool that is not ready, row 2 ("any other object") does, and `management` alone authorizes it. (4) The Resource, with implementation items the actor chose, is covered by every bound that names `shared` (ACC-029) and is set up by `shared`'s administrators. The narrowing by owner is gone for every Pool not yet set up. For Agents ACC-057 leaves only root in this version, so the harm today is on Pools.

**Counterargument.** Row 2 lists what falls to `management` alone as "whose owner is retired or is in the same submission", ACC-033 ends "No request is authorized against a retired Pool or Fleet", POOL-004 and FLEET-003 write "retired" in their `then`, and spec/README.md:475 says "a Pool that exists and is not retired". A careful builder reaches reading A. But that README section is "statements, not promises", and rule 4 of the form of a promise requires every term to be defined.

**Least change.** In ACC-033 rows 1 and 1A, FLEET-003's observable and the row `management` of definitions.md, write "a Pool (a Fleet) that exists before the submission and is not retired" in place of "a live existing Pool (Fleet)". Or add one row to definitions.md: "live | Said of an instance that exists and is not retired, whether its state is registered or ready."

**Precedent.** Kubernetes admits objects into a Namespace in every phase but Terminating, and names the phase rather than using an informal word.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity whose fix adds no behavior; spec/README.md:475 already states the intended reading.

**Verifier.** weakened, low. Quote verbatim at spec/promises/fleets.yaml:52-54. "live" has no row in spec/definitions.md, against rule 4, spec/README.md:300 ("Every term is defined in definitions.md"). But the spec's own usage fixes the meaning as not retired, so reading B (live = ready) is not a sound second reading: (1) spec/definitions.md:275 says "the latest failed or unknown setup of a live instance ... (PND-006)" where PND-006's `when` is "An instance is not retired and its latest set-up.v1 invocation ended failed or unknown" (spec/promises/pending.yaml:300); an instance whose only setup failed is registered, not ready, and is called live. (2) ACC-033's observable splits registrations in two with nothing between: "Registration into a live existing owner needs both Conduits: one to `management` alone is insufficient. Registration into a retired owner needs only `management`" (spec/conventions/access.yaml:640-643). (3) Row 2 lists what falls to `management` alone as "whose owner is retired or is in the same submission" (access.yaml:599-600) and the table ends "No request is authorized against a retired Pool or Fleet" (access.yaml:636). (4) OPS-036 contrasts "An Agent of a live Fleet" with "A retired owner's Conduits are closed" (spec/conventions/operations.yaml:882-883). (5) POOL-004's `then` uses "Where the Pool is not retired" (spec/promises/pools.yaml:60). The only support for reading B is a comment heading (spec/promises/conduits.yaml:155), which also heads CON-012, a promise about "an existing instance". Step 3 of the harm ("Row 1 does not fit a Pool that is not ready") therefore rests on no promise. Not recorded in docs/AUDIT.md known-open, cold-readers.md, IN_FORCE.md, the increment logs or G5.

**Verifier's corrections.** Class: defect of form (an undefined term, rule 4), not an ambiguity with a sound unsafe reading. Unsafe mark: remove. Severity low. Least change stands as the reviewer wrote it, second alternative preferred: one row in definitions.md ("live: said of an instance that exists and is not retired, whether registered or ready"); the word is also used at access.yaml:641, operations.yaml:882 and definitions.md:147 and 275. Coordinator decides (rulings 37; 46 point 4). What would settle it fully: that row.


## FR-092

**A Pool and a Fleet of one name: FLEET-006 says conflict; OBJ-023, OBJ-025, FLEET-005 and OPS-023 give invalid**

L08/F2 · contradiction · verifier: confirmed, medium · decides: coordinator · new

**Promises.** FLEET-006, FLEET-005, OBJ-023, OBJ-025, OPS-023, IFC-014

**Where.** spec/promises/fleets.yaml:110

**Quote.** "A Fleet named like an existing Pool instance, or the reverse, reports conflict under OBJ-025, with a diagnostic naming both objects."

**Situation.** Pool `build` exists and is not retired. A submission registers Fleet `build`. (a) OBJ-025's `when` covers only a document "rejected under OBJ-002, because it would make an object whose kind and name an existing object has" (spec/promises/objects.yaml:456-457); a Pool and a Fleet are two kinds. OBJ-023 gives invalid for "any cause other than the clash with an existing name that OBJ-002 states" (objects.yaml:420-423). By the `when` and `then` texts FLEET-006's clash is invalid; by FLEET-006's observable it is conflict. (b) Root holds its default Conduit to Pool `build` with local name `build`; the Fleet's default would take the same local name, and FLEET-005's observable says "IFC-014 rejects the registration: nothing is registered and it reports invalid" (fleets.yaml:91-93). OPS-023 reports invalid before conflict (spec/conventions/operations.yaml:493-495). So for every Pool or Fleet that is not retired the texts give invalid, and FLEET-006's conflict could show only against a retired one whose default is closed. The build's vector expects conflict with ruleId OBJ-025 (conformance/conventions/fleets/register-name-clash/case.yaml:145 and 163). Two builders, and a vector author, diverge on the reason and the ruleId.

**Counterargument.** The intent is plain: definitions.md gives conflict where a document "clashes with the name of an object that exists", and OBJ-002's observable points to FLEET-006. Either reason rejects the whole submission and the diagnostic names both objects; only the reason word and the ruleId differ.

**Least change.** Decide one reason in the `then` texts. Smallest: widen OBJ-025's `when` to "rejected under OBJ-002, or under FLEET-006 because a Pool instance and a Fleet would share a name", except the same in OBJ-023, and add to FLEET-005's observable: "Where the other Conduit is root's default to a Pool or Fleet of that name, FLEET-006's clash is the one reported." The alternative is to have FLEET-006 report invalid, as its reserved-name clause does, and change the vector.

**Precedent.** Kubernetes: 409 AlreadyExists for a name already taken, 422 Invalid for a document that breaks another rule; HTTP 409 Conflict.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction whose fix adds no behavior; it picks between two of the four existing reasons.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/fleets.yaml:110-111. (a) OBJ-025's `when` covers only a document "rejected under OBJ-002, because it would make an object whose kind and name an existing object has" (spec/promises/objects.yaml:456-457) and its observable adds "Every other cause of rejection reports invalid under OBJ-023" (objects.yaml:462-463); OBJ-023's `when` is "for any cause other than the clash with an existing name that OBJ-002 states" (objects.yaml:420-421). OBJ-002's `when` needs kind and name to match (objects.yaml:32-33) and its observable sets FLEET-006 apart: "FLEET-006 also forbids a Pool and a Fleet from sharing a name" (objects.yaml:40-41). So by the `when` and `then` texts a Pool/Fleet clash is invalid; by FLEET-006's observable it is conflict. (b) FLEET-005's observable: "If root already has a Conduit that is not closed with that local name, IFC-014 rejects the registration: nothing is registered and it reports invalid" (fleets.yaml:91-93); a Pool that is not retired always has root's default with `as` equal to its name (fleets.yaml:84-85; ACC-024, spec/conventions/access.yaml:422-426), and OPS-023 puts invalid before conflict (spec/conventions/operations.yaml:493-495). The vector expects conflict with ruleId OBJ-025 (conformance/conventions/fleets/register-name-clash/case.yaml:145, 163). No record of this in docs/AUDIT.md known-open, cold-readers.md, IN_FORCE.md (line 70 only cites FLEET-006 as in spec), the increment logs or G5 (FLEET-005 is "not modelled", experiments/012-model-checking/G5.md:118).

**Verifier's corrections.** As stated. One addition: part (b) rests on a point the text also leaves open, whether IFC-014's check on the default counts as a second cause when the Fleet's own document is already rejected (FLEET-005's `when` is "A Pool or Fleet is registered", fleets.yaml:77). The fix should say which; the reviewer's sentence for FLEET-005's observable does. The vector cannot show it: its snapshot holds Pool `occupied` with no root default (see extra). Coordinator decides (rulings 37; 46 point 4). The spec's own definition of conflict, "clashes with something that exists, such as a name" (spec/definitions.md:237), favors widening OBJ-025 over changing FLEET-006 to invalid.


## FR-093

**Once the target of a required declared Conduit is retired, its consumer can never complete another setup, so its run can never be asked for again**

L08/F3 · defect · verifier: weakened, medium · decides: owner · known-new-situation

**Promises.** CON-004, LIF-008, LIF-009, TPL-012, OPS-022, PRV-007, PRV-026, CON-008

**Where.** spec/promises/conduits.yaml:56

**Quote.** "Where the nested instantiation has no Conduit that is not retired, the part reports conflict. Setup instantiates no Conduit."

**Situation.** Agent `worker`, ready, on the engine's own runner, declares the required Conduit `repo` to Resource `repo-1`. `repo-1` is retired and another Resource made in its place, as a rotated secret or a package upgrade requires (finding 4-01). CON-008 closes `repo`. No replacement can be registered: LIF-009's observable says "a request for a new declared Conduit whose target is retired, under CON-025, or no longer exists, under OBJ-009, reports invalid" (spec/conventions/lifecycle.yaml:172-174), since the nested instantiation holds the old ID. An actor gives `worker` a standalone Conduit to the new Resource, the route 4-01 names for keeping an Agent and its sign-in. Then the engine restarts. "An Agent's run on that runner is not run again so: a further setup asks for it" (spec/promises/providers.yaml:894-895). That setup comes first to `repo`'s part, which reports conflict for good; `repo` is not optional, so the run's part is never begun (TPL-012) and the setup never succeeds (OPS-022). `worker` stays ready with no run, and no request changes that; only retiring it and registering another does, with a lift for its name and a new sign-in. The same follows a crash that the provider's restart policy does not mend, and an actor's deliberate close of a required declared Conduit: revoking one relationship also ends the consumer's ability to be run again.

**Counterargument.** This is what "required" and LIF-008 mean (nothing gives back access that was taken away), and IDs are fixed on purpose (OBJ-010). The records already say that replacing a target forces its holders to be replaced (4-01; the short-form synthesis's cascades), and whether declared Conduits earn their place is a recorded later candidate. An author who expects a target to change can mark the Conduit optional.

**Least change.** The owner's choice among: (a) CON-004's part counts as succeeded where the consumer has an open Conduit whose local name is the declared Conduit's name, so a standalone Conduit registered with that `as`, after the lift, stands in its place; this adds no operation. (b) `register-replacement.v1` takes `target` for a declared Conduit whose stored target is retired or gone, authorized as registering that Conduit. (c) Keep the rule, state it under Limits of this version, and have the plan print each consumer that a retirement strands.

**Precedent.** Kubernetes: a Pod names a ConfigMap, Secret or claim by name, resolved when its container starts, so a replaced object of that name is picked up at the next start; systemd's `Requires=` names a unit, not one instance of it.

**Who decides.** owner. It changes behavior (what a setup needs, or a new input) and belongs with finding 4-01, which the owner holds until S1's bar is judged (ruling 71 point 3).

**Verifier.** weakened, medium. Quote verbatim at spec/promises/conduits.yaml:56-57. The chain holds for a retired target: CON-008 closes the Conduit at once and it is removed (conduits.yaml:109-115); LIF-009's observable: "a request for a new declared Conduit whose target is retired, under CON-025, or no longer exists, under OBJ-009, reports invalid" (spec/conventions/lifecycle.yaml:172-174), the reference being stored by ID (spec/promises/objects.yaml:127-129); TPL-012 begins the run only after every required earlier part (spec/promises/templates.yaml:217-218); PRV-026: "An Agent's run on that runner is not run again so: a further setup asks for it" (spec/promises/providers.yaml:894-895). CON-004's and LIF-008's observables speak as if a replacement were always possible ("until a replacement is registered under LIF-009", conduits.yaml:61-62; lifecycle.yaml:121-122). Already recorded in general: finding 4-01 (docs/proposals/first-build/log-increment-4.md:12: "T5 and T6 need a new Resource and either new Agents, losing their sessions, or standalone Conduits") and docs/proposals/language-shape/short-form/synthesis.md:528-529 ("Replacing a target forces their holders to be replaced too. For Agents, replacement loses the home and the sign-in"). The new situation is real: 4-01's standalone-Conduit route does not keep the Agent past its next setup or an engine restart.

**Verifier's corrections.** Known-new-situation is right; keep it under 4-01, owner decides (ruling 71 point 3). Two corrections. (1) Class: gap, as 4-01 is classed, not defect; the promises are consistent and can be followed, and what is missing is any route by which a required declared Conduit gets a new target. The one wrong text is the pair of observables above, which could say "where LIF-009 can make one". (2) Drop the leg on an actor's deliberate close: there the target still exists, so a replacement can be registered under LIF-009 after the tombstone is lifted (lifecycle.yaml:182-186, LIF-041, ACC-053) and the next setup opens it; that the consumer cannot be set up while a required Conduit stays revoked is what required means. Scope note: the short form makes only standalone Conduits (SCT-007, spec/promises/sections.yaml:122-124), so this reaches canonical templates and package templates that declare Conduits.


## FR-094

**A setup that succeeds past a failed optional part has nowhere to report it**

L08/F4 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** CON-004, POOL-013, OPS-022, IFC-025, IFC-050, LIF-004

**Where.** spec/promises/conduits.yaml:66

**Quote.** "A declared Conduit marked optional that is not opened does not keep the setup from succeeding, and the setup reports it as not opened."

**Situation.** Agent `worker` declares the optional Conduit `scratch`. An actor sets `worker` up; the `open.v1` part for `scratch` reports refused (the actor's bound does not cover the target) or conflict (a tombstone matches, LIF-041). Every other part succeeds, so the outcome is succeeded (OPS-022). `set-up.v1` declares no `outputSchema` (spec/core/operations.yaml:433-446), and IFC-025 says "Where it declares no outputs, the result carries no `structuredContent`, except for error diagnostics" (spec/interface/implementation.yaml:669-670), which it gives only to a result that ends with a reason. LIF-004 puts a setup into the status only where it "fails, or has an unknown outcome, or is not dispatched". So the result says succeeded and nothing else. The caller, often an agent, learns that `scratch` is not open, and why, only by inspecting the Conduit and reading the part's record. POOL-013 has the same sentence for an optional entry ("is reported as not set up", spec/promises/pools.yaml:173-174). One builder puts prose in `content` (DL12), another adds a status member, a third reports nothing; ruling 52 asks every result to say what happened and why.

**Counterargument.** The part is a request of its own and is recorded with its reason (REC-001), and inspection shows the Conduit still registered. Nothing is lost; it is only not said in the result.

**Least change.** Give `set-up.v1` the output `diagnostics`, as `register.v1` has, and add to OPS-022: "The result of a setup that succeeds holds in `diagnostics` one diagnostic of level `warning` for each optional part that did not succeed, naming the part, its reason or outcome, and the promise that decided it."

**Precedent.** SARIF 2.1.0 `level: warning`, which `diagnostic.v1` already carries and `register.v1` already returns for diagnostics that do not reject (OBJ-008); the Kubernetes API's warnings on a successful response (KEP-1693).

**Who decides.** coordinator. Ruling 71 point 4: a small finding that adds a feature (one output), which cannot move S1's author counts; logged for the owner to strike. The explicitness rule is ruling 52's.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/conduits.yaml:66-67; the twin is at spec/promises/pools.yaml:173-174 ("is reported as not set up"). No text says where: `set-up.v1` declares no `outputSchema` (spec/core/operations.yaml:411-446, inputs and `effect` only); IFC-025: "Where it declares no outputs, the result carries no `structuredContent`, except for error diagnostics as stated below" (spec/interface/implementation.yaml:669-670), and those are for results that end with a reason (implementation.yaml:704-707); LIF-004's `when` is "An instance's setup fails, or has an unknown outcome, or is not dispatched" (spec/conventions/lifecycle.yaml:63); OPS-022 only counts the outcome (spec/conventions/operations.yaml:465-467). The only use of level warning in spec/ is the enum in spec/core/results.yaml:87. Not in docs/AUDIT.md known-open, cold-readers.md, IN_FORCE.md (lines 142-144 hold ruling 52 as in spec), the increment logs (4-13 is the CLI's exit code) or G5 (optional setup parts are absent from the model, G5.md:99).

**Verifier's corrections.** As stated; class gap, not unsafe. On the fix: a warning in a succeeded result is allowed by IFC-025 only if `set-up.v1` declares the output, which gives the tool an `outputSchema`; check that against review item 11 (docs/proposals/first-build/to-review-with-the-owner.md:47), which removed `structuredContent` from no-output tools because a real client then hid `content`. An alternative with no change to the wire: widen LIF-004 so the instance's status names each optional part of the latest setup that did not succeed, with its reason. Either adds a small feature: coordinator under ruling 71 point 4, logged for the owner to strike; it moves no S1 author count.


## FR-095

**After one part of a setup fails, whether the other parts at its position are still begun is not decided**

L08/F5 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** TPL-012, CON-004, POOL-013, OPS-022, LIF-027, LIF-021

**Where.** spec/promises/templates.yaml:217

**Quote.** "A part is begun only once every part that comes before it in that order, and is not marked optional, has succeeded."

**Situation.** An Agent's template declares Conduits `a` and `b`, neither optional. The setting-up actor's bound covers `b`'s target and not `a`'s, so `a`'s part reports refused (ACC-005). Reading A: `b` is still opened, since nothing at an earlier position failed (TPL-012 says of the run and the connections that "none of them waits for another", templates.yaml:226); the setup fails with `b` open, and LIF-027 closes nothing. Reading B: the engine stops at the first failed part and `b` stays registered; which part is first is the engine's own iteration order. The same for a Pool whose entries `x` and `y` do not refer to each other: POOL-013 says only that a part is "begun only once the parts for the entries it refers to have succeeded" (spec/promises/pools.yaml:166-167). After the failed setup, which Conduits are open, so which tools the consumer is presented (IFC-012), and which Resources are ready, differ by engine. No item of implementation-defined.yaml covers it; DL1 covers only the order of a cascade.

**Counterargument.** "The setup is these parts" can be read to oblige every part that nothing holds back, which is reading A. Under either reading only what was declared and permitted is opened or set up.

**Least change.** Add to TPL-012's `then`, in the words LIF-021 uses for a cascade: "A part that does not succeed holds back only the parts that come after its position and, among a Pool's entries, those that refer to it; every other part is begun." If reading B is wanted instead, add it as an item of implementation-defined.yaml.

**Precedent.** Terraform's graph walk continues every node that does not depend on a failed one; LIF-021 ("holds back only the members ordered after it; every other retirement proceeds").

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity; the fix states for setup the rule LIF-021 already states for retirement and adds no feature.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/templates.yaml:217-218. "Only once" is a necessary condition; no text obliges a part to begin when its predecessors succeeded, and none says whether a failed part holds back parts at its own position. The no-wait sentence covers only the second position: "The run and the connections stand at one position in the order: none of them waits for another" (templates.yaml:225-226). POOL-013 has the same form ("begun only once the parts for the entries it refers to have succeeded", spec/promises/pools.yaml:166-167), and so has the contract of `set-up.v1` (spec/core/operations.yaml:413-415). LIF-021 states the rule for retirement only (spec/conventions/lifecycle.yaml:430-432). spec/implementation-defined.yaml has no item on it. Not in docs/AUDIT.md known-open, cold-readers.md, the increment logs or G5 (G5.md:99 and 113 report TPL-012 and POOL-013 as holding in a model without optional parts).

**Verifier's corrections.** As stated, with one consequence the reviewer missed that raises its weight: the two readings give different outcomes for the same documents and state. OPS-022: "failed only if no part had an effect, and otherwise unknown" and "unknown if any part succeeded with an effect" (spec/conventions/operations.yaml:465-466, 474); `open.v1` states an effect (spec/core/operations.yaml:785). So with `a` refused: reading A (b still opened) ends unknown; reading B (stop at `a`) ends failed; reading B with `b` taken first ends unknown. The fix should be stated with that in view. Coordinator decides (rulings 37; 46 point 4); the LIF-021 wording is the right model.


## FR-096

**A Conduit's `operations` can resolve to absent, or list one operation twice, and no promise decides either**

L08/F6 · gap · verifier: weakened, low · decides: coordinator · new

**Promises.** CON-016, CON-017, INS-008, INS-012, INS-023, IFC-014

**Where.** spec/promises/conduits.yaml:205

**Quote.** "It requires `operations` unless one of its inputs has `operations` as a destination, and it may leave out `requirements`, and then applies none."

**Situation.** (1) A Conduit template writes `capability`, no `operations`, and `inputs: {ops: {schema: {type: array}, to: [/operations], optional: true}}`. CON-016 accepts it. An instance supplies no `ops`. INS-008: "Nothing is assigned. Each of its destinations keeps the value the template gives it, or is absent if the template gives none" (spec/promises/instantiation.yaml:112-113). The resolved Conduit has no `operations`. INS-023 checks only a list "that an input's argument or default gives it". One builder rejects the instance document, another stores an empty list (ACC-003: "A Conduit that lists no operations permits nothing"), a third stores it absent; inspection and the tombstone, which keeps "operations" (LIF-040), then differ. (2) `operations` lists the same operation twice. `conduit-template-spec.v1` sets no `uniqueItems` (spec/schemas/templates.yaml:266-268), and CTR-007 decides a twice-listed name for a Capability only. IFC-014's "one of its tool names ... would equal another tool's name" either rejects the Conduit or, read with IFC-012's one tool for each operation, does not.

**Counterargument.** Neither case widens access: ACC-003 gives an absent or empty list nothing, and a duplicate adds no operation. Authors seldom write either.

**Least change.** Add to CON-016: "An input whose destination is `/operations`, in a template that writes no `operations`, states a `default` or is not marked `optional: true`; a template that breaks this is rejected." Add to CON-017: "No operation is listed twice; a list that does so, written or given by an input, is rejected", with `uniqueItems: true` in `conduit-template-spec.v1`. For the owner, if wanted: remove the destination `/operations` and INS-023 altogether; no core template, example or package uses it (only the two vectors under conformance/language/registration/operations-input-*), and the short form's inline `consumes` already makes one template for each Conduit.

**Precedent.** Kubernetes RBAC: a PolicyRule's `verbs` is required and not empty; JSON Schema `uniqueItems`, which `manifest.v1` already uses for `exports` (PKG-012); CTR-007.

**Who decides.** coordinator. Rulings 37 and 46 point 4: rejecting an undetermined document adds no feature. Removing the destination would remove a feature and is the owner's.

**Verifier.** weakened, low. Quote verbatim at spec/promises/conduits.yaml:205-206. Part 1 is decided: INS-008's `then` is "Nothing is assigned. Each of its destinations keeps the value the template gives it, or is absent if the template gives none" (spec/promises/instantiation.yaml:112-113), so the resolved Conduit has no `operations`, no promise rejects the instance document (INS-004 covers only inputs not marked optional, instantiation.yaml:57-60; INS-023 only a list an argument or default gives, instantiation.yaml:319-320), and ACC-003 gives it nothing: "A Conduit that lists no operations permits nothing" (spec/conventions/access.yaml:65-66). A builder who rejects breaks INS-008. Part 2 stands: `conduit-template-spec.v1` sets no `uniqueItems` on `operations` (spec/schemas/templates.yaml:266-268), CTR-007 covers a Capability only (spec/promises/contracts.yaml:194-197), and IFC-014's "would equal another tool's name" (spec/interface/implementation.yaml:353-354) read with IFC-012's "exactly the operations that its open Conduits list" (implementation.yaml:283-284) does not say whether a list that repeats an operation is rejected. The claim that nothing but two vectors uses the destination `/operations` checks out (only conformance/language/registration/operations-input-* and one invalid example). Not recorded anywhere I searched.

**Verifier's corrections.** Smaller than stated. Part 1 is not undecided; what remains is that the output schemas require the list for a Conduit (`tombstone.v1`, spec/core/results.yaml:354-361; the inventory items, results.yaml:437-443 and 471-477), so an absent list has to be shown as `[]` and no promise says absent reads as empty. One sentence in CON-016 or INS-008 settles it, or the reviewer's rule that such an input states a `default`. Part 2 is a small gap with no effect on access. Coordinator for both; removing the destination `/operations` and INS-023 removes a feature and is the owner's.


## FR-097

**INS-016 and INS-017 state instantiation as pure, while POOL-016 and PRV-015 place by a provider call within it**

L08/F7 · contradiction · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-147.

**Promises.** INS-016, INS-017, POOL-016, PLC-002, PRV-015, PRV-026

**Where.** spec/promises/instantiation.yaml:233

**Quote.** "After instantiation no setup has run, no Conduit other than a default made under FLEET-005 has been opened, and no implementation has received a call."

**Situation.** A Pool template has the entry `cache`, whose Resource has an item that is run; no input supplies its placement, and a machine provider that reports two matching reachable machines is registered. Registering the Pool: POOL-016 has the Resource "placed under PLC-002, by the machine provider under PRV-015", and PRV-015 has the engine call `place.v1` "while the engine decides the request that would instantiate the instance" (spec/promises/providers.yaml:468 and 480-481). The machine provider's implementation receives a call during instantiation, and PRV-026 may first run that provider again and read its secrets. INS-016's observable says no implementation has received a call. Registering the same template with the same arguments again, the provider may name the other machine; the two instances differ in `placement`, which INS-017 does not except: "The two results are the same, apart from generated names and IDs" (instantiation.yaml:245). Both are `language` promises, and a vector written from either fails on an engine that keeps POOL-016.

**Counterargument.** definitions.md says in its row "resolving" that placing an instance "is not resolving", and INS-016's `then` speaks only of objects, setups and openings; only its observable and INS-017's list of exceptions overreach. With the engine's own machine provider there is one machine and both hold.

**Least change.** INS-016's observable: "... and no implementation has received a call, apart from the provider call of `place.v1` under PRV-015 and the run that PRV-026 makes for it." INS-017's `then`: "... apart from generated names, IDs and a placement that the machine provider chose under PLC-002."

**Precedent.** Kubernetes: `spec.nodeName`, filled by the scheduler, is not part of what a manifest determines; Terraform marks such values "known after apply".

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction whose fix adds no behavior.

**Verifier.** confirmed, low. Quote verbatim at spec/promises/instantiation.yaml:233-234; INS-017's `then` at instantiation.yaml:245. POOL-016: "placed under PLC-002, by the machine provider under PRV-015" (spec/promises/pools.yaml:224). PRV-015: "The engine asks the machine provider with `place.v1`" (spec/promises/providers.yaml:468), "a provider call under PRV-005, made while the engine decides the request that would instantiate the instance, before it is dispatched" (providers.yaml:480-481), and the provider "names by id one machine" among those that match (providers.yaml:469-470) with nothing that makes the choice repeatable. PLC-002 puts the chosen node's ID in the resolved instance (spec/promises/placement.yaml:44). So a vector written from INS-016's observable or INS-017's `then` fails an engine that keeps PRV-015 with a provider of two matching machines. Not in docs/AUDIT.md known-open, cold-readers.md, IN_FORCE.md, the increment logs or G5; docs/LANGUAGE_DEFECTS.md:196 only lists both as ruled.

**Verifier's corrections.** As stated; contradiction, not unsafe, coordinator. Only INS-016's observable overreaches, its `then` does not; INS-017's `then` does. The fix follows text the spec already has: spec/definitions.md:101 says "Placing an instance that is given no placement is not resolving: the engine asks the machine provider while it decides the registration". Low because PRV-015 leaves no doubt about what an engine does; the cost is a wrong vector and a Language-level claim that placement breaks.


## FR-098

**FLEET-005 is layered `convention` but decides which objects a registration makes and when it is rejected, and states five things**

L08/F8 · defect · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-042, FR-056, FR-142.

**Promises.** FLEET-005, INS-016, SUB-006, IFC-014

**Where.** spec/promises/fleets.yaml:79

**Quote.** "Within that registration the engine instantiates and opens one default Conduit from root to the new Pool or Fleet."

**Situation.** The `language` layer is "which objects they make, how those relate, and what is valid" (spec/README.md:61), and the Language level covers only promises whose layer is `language` (spec/README.md:320). INS-016 and SUB-006, both `language`, say that a registration makes "root's default Conduits under FLEET-005", but what that Conduit is (its template, consumer, target, generated name and `as`) stands only in FLEET-005, which is `convention`. The rule that the registration of a Pool `web` is rejected where root already has a Conduit with local name `web`, for example an `operator` Conduit to an Agent `web` (SCT-009), stands only in FLEET-005's observable (fleets.yaml:91-93); IFC-014's `then` says only "The Conduit is not instantiated", and the default has no document for SUB-003 to reject. A Language-level reader must take the content of a `language` obligation from a `convention` promise and a rejection from an observable. The one promise also states the instantiating, the opening, two operation lists that repeat spec/core/templates.yaml:145-160 and 260-274, the name and the local name, against rule 1 of the form of a promise, so two texts must be kept equal.

**Counterargument.** INS-016 and SUB-006 pull the default into the language by citing it, no engine claims the Language level alone, and the collision is rare. The lists in the promise make it readable without opening the core package.

**Least change.** Split it. `language` (a new ID): "A registration that makes a Pool or Fleet also makes one Conduit from the core template `root-pool` or `root-fleet`, with root as its consumer, the new instance as its target, a name under OBJ-003 and `as` equal to the target's name; where IFC-014 does not instantiate it, the document that makes the Pool or Fleet is rejected." `convention` (FLEET-005 keeps its ID): "The engine opens that Conduit within the registration, and no request closes it." Leave the operation lists to the core templates.

**Precedent.** POOL-002, which states in one `language` promise that a Pool is not instantiated where one of its nested Resources cannot be.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect of layer and form; the split adds no behavior.

**Verifier.** confirmed, low. Quote verbatim at spec/promises/fleets.yaml:79-80; FLEET-005 is `layer: convention` (fleets.yaml:74). The language layer is "which objects they make, how those relate, and what is valid" (spec/README.md:61) and the Language level covers "Every promise whose layer is `language`" (spec/README.md:320). INS-016 and SUB-006, both `language`, take the default's content from it (spec/promises/instantiation.yaml:229-230; spec/promises/submissions.yaml:100). The rejection of the whole registration stands only in observables: "IFC-014 rejects the registration: nothing is registered and it reports invalid" (fleets.yaml:92-93), while IFC-014's `then` is only "The Conduit is not instantiated" (spec/interface/implementation.yaml:356) and FLEET-005's `then` says the engine instantiates one. The two operation lists repeat spec/core/templates.yaml:151-160 and 266-274, which I checked are equal today. Rule 1 is at spec/README.md:297. No record treats this as open; docs/proposals/spec-batch-1/areas/fleet.md drafted it as convention.

**Verifier's corrections.** As stated. The part with weight is the rejection rule that no `then` holds (POOL-002's "If any of them cannot be instantiated, the Pool is not instantiated", spec/promises/pools.yaml:33-34, is the model); the layer and the several obligations are form. The split adds no behavior: coordinator (rulings 37; 46 point 4). It should be done together with F2, since both turn on the same sentence of FLEET-005's observable.


## FR-099

**CON-031's `when` and `then` give conflict for a retired target; its own observable and CON-008 give invalid**

L08/F9 · defect · verifier: confirmed, low · decides: coordinator · new

**Promises.** CON-031, CON-008, LIF-035, OPS-029, OPS-023

**Where.** spec/promises/conduits.yaml:392

**Quote.** "An actor asks to open a Conduit that is not open and whose consumer or whose target is retired."

**Situation.** Conduit `c` is registered and not open; its target Resource is retired. CON-008 closes `c` "at once" and LIF-035 removes it, so no Conduit that is not open and whose target is retired exists to be named. CON-031's `then` ("The request is not dispatched, and reports conflict", conduits.yaml:395) covers the case all the same, while its observable says such a request "names a Conduit that no longer exists, and reports invalid under OPS-029" (conduits.yaml:397-399), and OPS-023 puts invalid before conflict. A vector written from the `when` and `then` expects conflict; an engine that keeps CON-008 answers invalid. The row "open" of definitions.md repeats the wrong half: "either of whose ends is retired is not opened (CON-031)".

**Counterargument.** The observable explains the case, and an engine that keeps CON-008 can only answer invalid; a builder who reads the whole promise is not misled.

**Least change.** CON-031's `when`: "An actor asks to open a Conduit that is not open and whose consumer is retired." Keep the observable's sentence on a retired target, and in the row "open" of definitions.md write "whose consumer is retired".

**Precedent.** Kubernetes: a request that names a deleted object is NotFound, never Conflict.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect whose fix adds no behavior.

**Verifier.** confirmed, low. Quote verbatim at spec/promises/conduits.yaml:392-393; `then` at conduits.yaml:395. The promise's own observable denies the retired-target case: "A Conduit whose target is retired is closed already, under CON-008, and so removed, under LIF-035: a request to open it names a Conduit that no longer exists, and reports invalid under OPS-029" (conduits.yaml:397-399). CON-008: "A Conduit whose target is retired ... is closed at once" and "A closed Conduit is removed" (conduits.yaml:109-115); CON-025 makes none for a retired end (conduits.yaml:345-347). So no Conduit that is not open and has a retired target exists, and that half of the `when` is a dead case whose `then` (conflict) contradicts the observable (invalid). OPS-023 also puts invalid first (spec/conventions/operations.yaml:493-495). No record: IN_FORCE.md:107 lists CON-031 as in spec; G5.md:135 reports it as holding.

**Verifier's corrections.** As stated, but small: an engine that keeps CON-008 can only answer invalid, so builders do not diverge; the cost is a `when` that a vector author cannot stage. One correction: the row "open" of spec/definitions.md:168 ("either of whose ends is retired is not opened") is true as written, since such a Conduit is not opened under either reason; changing it is optional. Coordinator (rulings 37; 46 point 4).


## Further points the verifier noticed

Not verified by a second reader.

- **The name-clash vector starts from a state that FLEET-005 never produces** (conformance/language/fleets/names/state.json:144-146 (used by conformance/conventions/fleets/register-name-clash/case.yaml:20)). The snapshot holds Pool `occupied` in the state ready and Fleet `taken`, and root has only its three first-start Conduits: no default Conduit with local name `occupied` or `taken`. Under FLEET-005 (spec/promises/fleets.yaml:79-85) every registered Pool or Fleet has one, and ACC-024 lets no request close it. So the vector shows conflict with ruleId OBJ-025 only in a state no engine reaches, and cannot show which reason is reported when FLEET-006's clash and the IFC-014 collision of the two defaults are both present (finding F2, part b). When F2 is fixed, the vector's state should include the default.
- **CON-004's and LIF-008's observables promise a remedy that LIF-009 withholds** (spec/promises/conduits.yaml:60-63; spec/conventions/lifecycle.yaml:121-124). Both say the part reports conflict "until a replacement is registered under LIF-009", and CON-004 adds "That replacement is in the state registered, so the next setup opens it". LIF-009's observable says no replacement is made where the declared Conduit's target is retired or gone (lifecycle.yaml:172-174). This is the textual side of F3: whatever the owner rules there, the two observables should say "where LIF-009 can make one". A wording fix the coordinator may make.

## The verifier's notes

Checked at commit 2eb8f258 in the read-only tree; nothing was built, run or changed. Every one of the nine quotes is verbatim at its cited line.

Result: 5 confirmed (F2, F4, F5, F7, F8, F9 are confirmed; that is 6), 3 weakened (F1, F3, F6), none known outright, none refuted. Severities: medium for F2, F3, F4, F5; low for F1, F6, F7, F8, F9. No finding survives as unsafe: F1's unsafe mark depends on reading "live" as "ready", which the spec's own usage excludes (definitions.md:275 with pending.yaml:300; access.yaml:640-643).

The two that matter most for builders are F2 (conflict against invalid for a Pool and Fleet of one name, with a vector that expects conflict from an unreachable state) and F5 (parts at one position after a failure), where I found the readings also differ in the setup's outcome, failed against unknown, under OPS-022. F3 is the owner's and belongs with finding 4-01; its new content is that 4-01's standalone-Conduit route does not keep an Agent past its next setup.

What I read: the five slice files in full (templates, instantiation, conduits, pools, fleets); the cited promises ACC-003, ACC-024, ACC-033, OBJ-002, OBJ-009, OBJ-023, OBJ-025, OPS-022, OPS-023, IFC-012, IFC-014, IFC-025, IFC-050, LIF-004, LIF-008, LIF-009, LIF-021, LIF-027, LIF-029, LIF-040, PLC-002, PRV-007, PRV-009, PRV-010, PRV-015, PRV-026, SUB-003, SUB-006, SCT-007, CTR-007; the relevant rows of definitions.md; the layers, levels, form of a promise and the start of "Limits of this version" in spec/README.md; `set-up.v1`, `root-pool` and `root-fleet` in spec/core; `conduit-template-spec.v1` and the result schemas. For newness: docs/AUDIT.md "Known to be open" in full, cold-readers.md "What is known to be open" in full, the increment 4 log's findings, the whole review list, G5.md by search for each promise, IN_FORCE.md and the ledger by search, the full-review brief's queued list.

Not read: the other promise files beyond the cited promises, the rest of spec/README.md's limits sections beyond searches, the reaudit rounds' full decision logs (searched only for CON-031 and the retired-target case), and ruling records other than 52 and 71 (searched 59 and 60 for the name clash). I did not look for further faults beyond the two in extra.
