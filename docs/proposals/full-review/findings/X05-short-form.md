# X05-short-form: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Commit 2eb8f258 (2026-11-12). `sections.yaml` holds 15 promises, SCT-001 to SCT-016 with SCT-010 unused, and none has a vector.

**Is the translation total?** For sectioned content, yes, and single-valued. By hand, the valid sectioned project gives 20 canonical documents (1 Fleet, 6 single-use templates, 6 instances, 7 Conduits), plus root's two defaults at registration. The sectioned package gives 2 Conduit templates. Six constructs of my own each had exactly one translation: `consumes` with three targets, `operator`, `manages`, per-file Fleet ownership, a Pool's entries, and a declared Conduit inside an inline Agent. Generated names cannot collide: authored names hold no `--`, `t`, `operator` and `manages` are reserved, and FLEET-006 stops a Fleet and a Pool sharing a name. The one construct with two outcomes is a framed Agent in a file that states `fleet` (F1).

**Is the meaning canonical?** Yes, by construction (SCT-003, SCT-015).
- The short form is stricter in four places (SCT-008, SCT-011, SCT-012's own-Fleet rule, SCT-013). None is a safety property, because SCT-016 accepts the same Conduits.
- Only the short form has ownership by file boundary and version from the manifest; neither reaches the wire.
- Only the canonical form has unnamed instances, a Conduit name or `as` other than the generated one, inputs on a single-use definition, and annotations on a `consumes` or `operator` Conduit.

**Which untested promises carry the most risk?** No program implements the SCT text. The engine only schema-checks, and experiment 011's translator predates the promises: it has no `manages`, rejects a referenced `operator` (`src/lib.rs:271-274`), and reads one document at a time. So its 14 of 14 covers only S1's constructs. Most risk first: SCT-011, SCT-012, SCT-008, SCT-013 with SCT-004, SCT-015, SCT-014, SCT-002.

**Does experiment 023 show finding 4-11 answered?** No. It is a pre-registered paper forecast, and the records already say so; what follows adds detail.
- The counts are 39 flow, 41 strict and 72 block against 64. The baseline was counted from a run that worked.
- One Codex worker wrote the states in 11 minutes, in a prescribed dense layout.
- No program produced or consumed them: 011 would reject their `operator: {from}`, and they assume `.v2` Types with a turn and sign-in that the fixtures lack.
- Block style fails, a required plan fails, and the routes the specification itself defines total 49, with T5 to T7 much more.
- Fixing the rule first is sound; the unit, a physical line, measures layout (ruling 63 accepted that).
- What the count shows is that frames and single-use templates no longer cost authored lines. Change (4-01) has moved to a CLI that is not built.

**Are two forms worth their cost?** Yes. The canonical form is already the wire and the store, and contracts stay framed, so the short form's cost is its translation. Trim F1's coupling and F3.

**Could an author write S1 from the short form's promises alone?** The documents of every state, yes, given the packages' contracts and the example for the Pool and Interface types. Carrying S1 out, no: nothing in `sections.yaml` registers, sets up, opens or changes anything, and T4 to T7 need the lifecycle and access conventions or the unbuilt CLI. The shipped example cannot register either: it pins filesystem 0.1.0, which exports no `read`.

**What the reader read.** **Read in full:** `spec/promises/sections.yaml`, `spec/schemas/sectioned.yaml`, `spec/promises/objects.yaml`, `spec/schemas/instances.yaml`, `spec/schemas/templates.yaml`, `spec/core/references.yaml`, `spec/core/templates.yaml`, the head and `bound.v1` of `spec/core/requirements.yaml`, the 2 valid and 12 invalid sectioned examples, and `spec/README.md` lines 1-200 and 272-400.

**Read by ID:** INS-001, 002, 024, 025; NAM-002, 003, 004, 013, 018, 022, 027; FLEET-001 to 006; MAN-001, 002, 003, 011; SUB-001 to 010; IFC-012, 014; CON-002, 021, 028; ACC-024, 054, 057; FST-001 to 006; POOL-005, 014; LIF-041; OPS-023; PKG-018, 031; SCH-005, 007, 012, 013.

**Experiments and references:**
- 011: README, the 2026-11-11 evidence, `src/lib.rs` in full, test names only.
- 023: README, and results lines 1-420 of 628.
- Work order 073: its Record and layout rule.
- Plan and apply: lines 1-330 and the open points; not its cases.

**Rulings and state:** records of rulings 53, 55, 59, 60, 62, 63, 66 and 76; 54 skimmed; 64 and 65 from the ledger only (organization, nothing on the short form). Also the short-form lines of IN_FORCE.md, the tracker's known-open list, increment 4's findings 4-01 and 4-11, and the batch 1 area papers by search.

**Not checked:**
- I ran no checker, build or test, so every translation is by hand and the schema was read, not executed.
- 011's 2026-11-10 results, 023's last 200 lines, the plan-and-apply cases and the conformance vectors were not read.
- Engine code was read only for the loader's branch choice and to confirm no translator exists.
- Plan and apply's own faults, the bar's fairness and tombstone lifting belong to other areas.

**Recounted:** 15 promises, all with `vectors: []`; 10 schema contracts in 961 lines; 7 `oneOf` in `sectioned.yaml`; 011's 14 exact states and 14 probes; 023's totals.


## FR-202

**A file's `fleet` key is said to supply the owner of framed Agents, which a framed Agent must itself state**

X05/F1 · contradiction · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-102.

**Promises.** SCT-011, OBJ-013, FLEET-002, SCT-003, SUB-010

**Where.** spec/promises/sections.yaml:185

**Quote.** "supplies X as the owner of every Agent instance defined anywhere in that file, whether sectioned or framed. An explicitly written owner equal to X is accepted; one naming another Fleet is rejected."

**Situation.** A file holds a sectioned document `fleet: s1` and, after `---`, a framed Agent with no `owner`. Reading 1 (SCT-011; SUB-010 at spec/promises/submissions.yaml:274, "SCT-011 supplies owners per file"): the Agent is registered, owned by s1. Translation then changes a framed document, though SCT-003 (sections.yaml:52) hands over "the framed documents read beside them". Reading 2 (OBJ-013 at spec/promises/objects.yaml:201, "`owner`, `consumer` and `target` are required"; FLEET-002 at spec/promises/fleets.yaml:34; instance.v1 at spec/schemas/instances.yaml:101-105; SUB-005): the document fails its schema before any translation, and "supplies" means only the equality check. One builder registers the Agent; the other reports invalid. The plan scoped the key to the sectioned document (docs/proposals/spec-batch-1/areas/fleet.md:262); the promise widened it to the file.

**Counterargument.** SCT-011 is the later and more specific rule and follows ruling 59 A6's words (the key "owns every Agent in the file"), so a builder could read it as an exception to OBJ-013. Mixed files are rare, and S1 has none.

**Least change.** SCT-011, then: "... and supplies X as the owner of every Agent defined in a sectioned document of that file. A framed Agent in that file states its `owner` (OBJ-013). An owner other than X, sectioned or framed, is rejected." SUB-010: "SCT-011 supplies the owners of sectioned Agents per file ...". The framed form stays exactly the wire form, and ruling 59 A6 still holds.

**Precedent.** kubectl's `-n` against a manifest's own `metadata.namespace`: a mismatch is rejected. kubectl also fills a missing namespace; that is reading 1, which would need OBJ-013, FLEET-002 and instance.v1 to change.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction whose fix adds no behavior. Reading 1 changes the framed form and would be the owner's.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/sections.yaml:185-187. SCT-011 reads as reading 1, and so do spec/definitions.md:73 ("supplying its owner to every Agent instance defined in that file (SCT-011)") and spec/promises/submissions.yaml:274 ("SCT-011 supplies owners per file before its boundaries are discarded"). Reading 2 rests on spec/promises/objects.yaml:196-201 (when: "A framed instance document is read"; "`owner`, `consumer` and `target` are required for those families"), spec/promises/fleets.yaml:34 ("An Agent document with no `owner`, or an owner that is not a Fleet, is rejected") and spec/schemas/instances.yaml:101-105 (agent branch requires `owner`). The only stated exception is for sectioned definitions: sections.yaml:111-112 ("an Agent requires its Fleet `owner` unless SCT-011 supplies it"), under SCT-006's when. No text in OBJ-013, FLEET-002 or the instances.yaml header mentions SCT-011 (grep of spec/ for SCT-011: nine hits, none there). No example covers a framed Agent without `owner` in a `fleet` file. Not recorded in docs/AUDIT.md:174-250, docs/guides/cold-readers.md:39-80, the increment logs, the review list, or order 037's Record (docs/work-orders/037-spec-short-form.md:61-74). The wording entered with order 037 (commit 9294c49d). Unsafe mark false is right: either the Agent is owned by the Fleet its own file names, or the project is rejected.

**Verifier's corrections.** Class stands; it could also be filed as ambiguity, because sections.yaml:51-55 lists SCT-011 among the translation rules and applies "SUB-001 to SUB-007 ... and every other check" to the handed-over set, which gives reading 1 a coherent order (supply, then instance.v1). The text therefore leans to reading 1 more than the reviewer says. Who decides: the reviewer's least change removes a behavior SCT-011 states in so many words ("whether sectioned or framed"), and ruling 59 A6 (docs/proposals/ruling-59/README.md:25, "owns every Agent in the file; an Agent elsewhere states `owner:`") implies an Agent in the file need not state it. The other direction is as small: add "unless SCT-011 supplies it" to OBJ-013 and FLEET-002 and say instance.v1 is applied after the supply. The coordinator may fix the contradiction, but the choice of direction changes whether a framed Agent may omit `owner`: log it for the owner to strike and put it on the review list.


## FR-203

**No promise says which form a failed sectioned definition is reported against**

X05/F2 · gap · verifier: weakened, low · decides: coordinator · new

**Promises.** SCT-006, SCT-007, SCT-014, SUB-005

**Where.** spec/promises/sections.yaml:259

**Quote.** "Schema rejections cite SUB-005 and carry `keywordLocation`."

**Situation.** An author writes `agents: {worker: {from: w, owner: f, consume: {...}}}` (a misspelt key), or mixes the two forms of a `consumes` item. Every definition, `consumes` item, `operator` and Conduit member is a `oneOf` of two closed maps (spec/schemas/sectioned.yaml:439, 522, 636, 728, 776, 840, 885). The failing keyword is the `oneOf` at `/agents/worker`; SUB-005 asks only for the place, the contract's name and the failing keyword's location. Engine A reports the `oneOf` and never mentions `consume`. Engine B picks a branch. The first build picks by a rule that exists only in code (engine/crates/loader/src/schemas.rs:441-443: the branch "whose `kind` it has, then one whose type it has, then the one it breaks in the fewest places"). A sectioned member has no `kind`, and `{type, from}` breaks each branch once. A vector that compares keyword locations would make that heuristic binding with no text behind it. Under ruling 52 the author is not told which key is wrong.

**Counterargument.** SUB-005 gives every schema rejection one form, framed documents have the same `anyOf` (instance.v1), and SCT-015 holds engines only to `ruleId` and `level`. Which sub-error to show may fairly be each engine's choice.

**Least change.** Add to SCT-014's observable: "Where a member fails the `oneOf` between its inline and referenced forms, the diagnostic is that of the form its keys select: `type`, or `capability` for a Conduit, a `consumes` item or `operator`, selects the inline form, and `from` the referenced form. It points to the offending key. With both or neither, it points to the member and says that exactly one of the two is required."

**Precedent.** OpenAPI 3's `discriminator`, which names the property that selects a `oneOf` branch; kubectl's strict decoding, which reports the unknown field's path.

**Who decides.** coordinator. Ruling 52 requires explicit diagnostics; ruling 71 point 4 covers a small finding that cannot move S1's counts; rulings 37 and 46 point 4.

**Verifier.** weakened, low. Quote verbatim at spec/promises/sections.yaml:259-260. The seven `oneOf` lines are right (spec/schemas/sectioned.yaml:439, 522, 636, 728, 776, 840, 885), and the engine heuristic is quoted correctly (engine/crates/loader/src/schemas.rs:441-443). Partly decided by the text: the misspelt-key case falls under OBJ-019, spec/promises/objects.yaml:334 ("Resolving returns a diagnostic naming the key") and :341-343 ("Section maps, `templates` family maps, `consumes` and the maps under `manages` have author-named members ... Their member bodies remain closed"). An engine that reports only the `oneOf` and never mentions `consume` breaks OBJ-019. What stays open: a key the specification defines but in the wrong form (`type` with `from`, `capability` with `from`, `config` on a referenced definition), and in every case where the pointer and `keywordLocation` point. SUB-005 (spec/promises/submissions.yaml:68-72) speaks of "the failing keyword's location" in the singular; SCT-014 asks for the field only "where the rejecting check names one" (sections.yaml:247-248). Not recorded: first-build 1-14 (docs/proposals/first-build/README.md:70) and review item 5 (docs/proposals/first-build/to-review-with-the-owner.md:18) settle that SUB-005 is cited, not which branch.

**Verifier's corrections.** Smaller and wider than stated. It is a general underdetermination of SUB-005 for every `anyOf` or `oneOf` (instance.v1 at spec/schemas/instances.yaml:55-62 and the reference schemas have it too), sharpest for sectioned members because they have no `kind`. Drop the misspelt-key example or restate it as a question of pointer and keyword location only. Class gap and decider coordinator stand. The least change fits better as one sentence in SUB-005 (the diagnostic is that of the branch the value's distinguishing key selects), with SCT-014 naming the sectioned keys. What would settle the size: whether sectioned vectors will compare instance pointer and `keywordLocation`, as conformance/README.md:86-92 says existing schema vectors do.


## FR-204

**`manages` writes nothing that `consumes` with the core scope templates does not**

X05/F3 · needless-complexity · verifier: weakened, low · decides: owner · new

**Promises.** SCT-012, SCT-007, SCT-013, SCT-016

**Where.** spec/promises/sections.yaml:203

**Quote.** "Each member T makes Conduit `N--manages--T`, with consumer `{kind: F, name: N}`, target `{kind: Fleet, name: T}` or `{kind: Pool, name: T}`, and `as: T`. For an Interface, role `manage` uses the core template `fleet-manage` or `pool-manage`; role `view` uses `fleet-view` or `pool-view`."

**Situation.** On Interface `console`, `manages: {fleets: {s1: manage}}` and `consumes: {s1: {to: {kind: Fleet, name: s1}, from: github.com/shrav-k/conduit/core#fleet-manage}}` make the same Conduit: same template, consumer, target and `as`. Only its name differs (`console--manages--s1` against `console--s1`). For an Agent the same holds with `fleet-manage-own`. Each is one flow line, so the bar's count (ruling 63) is equal. `manages` costs SCT-012, the schema `manages.v1`, a reserved `consumes` key, a third generated-name shape, and a role word whose template depends on the consumer's family. Its own-Fleet check is not made on the `consumes` or `conduits` spelling of the same Conduit (SCT-016), so it protects nothing. Scope is computed from open Conduits "whatever templates made them" (sections.yaml:217). Experiment 023 left the key out of S1 (docs/work-orders/073-paper-count-flow-style.md:168).

**Counterargument.** The owner ruled the shorthand (ruling 66 point 1). `manage` and `view` read as intent and spare a long core reference. The own-Fleet check catches an author's slip early. Batch 2 (a Fleet holding Pools, acting for a Fleet) may give the key a meaning that no template reference carries.

**Least change.** Remove SCT-012, `manages.v1` and the reserved key `manages` in SCT-013. Show the `consumes` spelling with `core#fleet-manage` in the sectioned example. Keep the five core scope templates and LIF-013's computed scope. If the key is kept, state in SCT-012 that it is a spelling of that `consumes` item.

**Precedent.** A Kubernetes RoleBinding to the built-in ClusterRoles `view`, `edit` and `admin`: a scope is an ordinary binding to a standard role by name, with no separate notation.

**Who decides.** owner. Ruling 66 point 1 ruled the shorthand, and removing a feature is the owner's.

**Verifier.** weakened, low. Quote verbatim at spec/promises/sections.yaml:203-206. The equivalence holds on the text: the five scope templates are exported by the core package (spec/core/conduit.yaml:168-172) and take no inputs (spec/core/templates.yaml:163-236); target-reference.v1 admits Fleet and Pool (spec/core/references.yaml:326-331); SCT-007 gives `N--L`, consumer N, `target` equal to `to`, `as: L` (sections.yaml:124-125); SCT-008 restricts only `management` and `access` (sections.yaml:144-145). No canonical promise confines an Agent's Conduit to its own Fleet, so SCT-012's own-Fleet rule binds only this spelling (sections.yaml:295-298: "Every Conduit that the canonical form accepts may be written here"). docs/work-orders/073-paper-count-flow-style.md:168 is cited correctly. But the headline is the ruled design: docs/proposals/ruling-66/README.md:18-22, "A `manages:` shorthand ... expands at apply into ordinary Conduits, made from standard templates and opened by root"; ledger row at docs/proposals/ruling-log.md:74.

**Verifier's corrections.** Real, but mostly what ruling 66 point 1 ruled knowingly: a shorthand by definition writes nothing the long form cannot. The new content is narrower: (1) SCT-012's restrictions (own Fleet, never `system`) are a lint on one spelling, not a protection; (2) the role word maps to a different template by consumer family. "The bar's count is equal" holds only in lines under flow layout: `s1: manage` is about 10 characters against about 85 for the `consumes` item with its core reference. Class and decider (owner; removing a ruled feature) stand. Put it to the owner as one option in the batched questions, with ruling 66 point 6's remark on shorthands (ruling-66/README.md:41-44) as context, not as a defect.


## FR-205

**SCT-014's authored locations cannot tell two files, or two documents of one file, apart**

X05/F4 · gap · verifier: known, low · decides: coordinator · new

**Promises.** SCT-014, SCT-011, SCT-013, NAM-027, SUB-004

**Where.** spec/promises/sections.yaml:257

**Quote.** "A supplied Fleet owner maps to `/fleet`; an explicitly written owner maps to its authored owner. A Fleet shared by several files names their supplying places."

**Situation.** a.yaml and b.yaml each state `fleet: s1` and each define `agents.x`. The collision must name "both authored places" (sections.yaml:236-237) and the shared Fleet "their supplying places". A diagnostic's only stated location is a JSON Pointer in `logicalLocations[].fullyQualifiedName` (spec/core/results.yaml:73-74, 103); anything else is what "an engine may add". Both places are `/agents/x` and both suppliers are `/fleet`, so an engine keeps every promise and returns the same pointer twice. The same holds for two `fleet` keys in two documents of one file, and for an owner conflict, where `/fleet` does not say which file's. Engines that do add a file will each do it their own way, and a caller that is an agent cannot find the line (ruling 52).

**Counterargument.** `message.text` can name the files, as other promises' diagnostics name a document. The first build's SARIF adapter already keeps physical locations. With one sectioned document per project, as in S1, a pointer is unique because names are unique within a project.

**Least change.** Add to SCT-014's then: "Each authored location also gives the file, as a path relative to the project's directory, and the document's position in it, in that location's SARIF `physicalLocation` (`artifactLocation.uri` and `region.startLine`)." List `physicalLocation` in `diagnostic.v1`.

**Precedent.** SARIF 2.1.0's `physicalLocation`, from the standard that `diagnostic.v1` already reduces; kubectl and Terraform diagnostics name file and line.

**Who decides.** coordinator. Ruling 52, with ruling 71 point 4: a small addition of a diagnostic field that cannot move S1's counts, logged for the owner to strike.

**Verifier.** known, low. Quote verbatim at spec/promises/sections.yaml:256-258; results.yaml lines cited correctly (spec/core/results.yaml:73-75, 103-104). Already a stated non-settlement in spec/: spec/core/README.md:706-707, "How a diagnostic names a document or an object beyond the JSON Pointer of its logical location ... an engine names them." The tracker's known-open list points at that section (docs/AUDIT.md:185-189). The reviewer's fix was considered and rejected under ruling 41: docs/proposals/ruling-41/README.md:73, P16, alternative "`physicalLocation` with a region"; the owner's answer at :42, "Accept all with p2 changed"; and :282, "A diagnostic names the document it is about by no fixed member." Other promises already require naming a file with no fixed member: spec/promises/manifest.yaml:34 ("rejected with a diagnostic naming the file"), spec/promises/submissions.yaml:75-77 ("the diagnostic names the file").

**Verifier's corrections.** Known mark should be known-new-situation, not new. The new situation: only in the short form does a file boundary carry meaning (SCT-011), and a sectioned document has no kind or name, so "names the file's `fleet`" (sections.yaml:189), "names both authored places" (sections.yaml:237) and "names their supplying places" (sections.yaml:258) rest wholly on engine-chosen naming; SCT-014 also does not say which document of a multi-document file a pointer is rooted at. Decider is wrong: listing `physicalLocation` in diagnostic.v1 reverses the owner-accepted P16, so it is the owner's, not a ruling 71 point 4 addition. A coordinator-sized alternative that changes no schema: SCT-014 says the diagnostic names the file and the document's position in it, in the manner of MAN-001 and SUB-005.


## The verifier's notes

Checked at commit 2eb8f258 (2026-11-12), read-only; nothing was built or run. Recounted: spec/promises/sections.yaml holds 15 promises (SCT-001 to SCT-016, SCT-010 unused), each with `vectors: []`. The engine has no sectioned translation: the only mention under engine/crates is loader/tests/examples.rs, so no build settles any of the four. All four quotes are verbatim and all cited lines check out. Result: F1 confirmed (medium), F2 and F3 weakened (low), F4 known (low). Two decider corrections matter most: F1's fix direction should be logged for the owner to strike, since the text and ruling 59 A6 lean to the reading the reviewer rejects; F4's fix reverses ruling 41 P16 and is the owner's. I added no extra findings; I did not verify the reviewer's assessment beyond what the four findings needed (the hand translations, the experiment 023 counts and the experiment 011 claims are unchecked).
