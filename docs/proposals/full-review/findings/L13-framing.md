# L13-framing: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Pinned at 2eb8f258 (2026-11-12). A newcomer cannot learn the model from these two files. They get a first sketch from the Core terms table of definitions.md (39 one-line rows), which is the best entry point in the repository, and nothing firmer without reading the promises.

What obstructs it, in order:
1. The README never introduces the model. "Pool", "Fleet" and "Interface" first appear as words in the Contents table (lines 208 to 225), after X509-SVID (line 44) and the gate (line 48). Its opening section spends about 1,000 words on what is not Conduit's (providers, proofs, gates) before saying what an Agent, a Resource or a Conduit is.
2. Neither file holds an example or a picture. Nothing shows one Agent in a Fleet, one Resource in a Pool and one Conduit between them; examples are only linked.
3. The full rows of definitions.md are digests of the rules, not definitions: 291 rows, 28,239 words, 1,311 citations of 420 of the 519 promises, 27 rows over 200 words ("connection" is 445). Learning a term means reading a convention, and several rows have drifted from the promises (F10, F11).
4. 41% of the README (3,641 of 8,816 words) is limits, some of it rulings and roadmap (F7).

The most serious obstruction is the first: the two files describe the specification (layers, files, proofs, limits) and not the model.

On the six questions:
- Definitions: F10 and F11 give the script's lists. No two definitions circle badly; the conflicts are drift.
- The layer test is not in spec/ and is not stated so that two people would agree: five of ten promises tried disagree with their recorded layer, two more turn on which wording is used (F3).
- The README's account of itself is behind the specification in three ways: the convention layer (F6), the status of `observable` (F1), and ruled tool behavior parked in the limits (F7). It promises more than the promises do where the limits state tool duties, and less where it names two exceptions to the preservation tests (F12).
- Three of the six limit sections are met before reliance; three are not (F9).
- The Contents table is accurate for files and counts; only vector coverage and three sentences are stale (F13).

Two findings are marked unsafe (F1, F2), both with medium-to-high confidence that the gap exists and less that a builder would take the harmful reading. Four need the owner (F2, F3, F6, F7); the rest are the coordinator's.

**What the reader read.** Read in full: docs/guides/full-review.md, AGENTS.md "What holds", spec/README.md (678 lines), spec/definitions.md (467 lines), and docs/AUDIT.md "How to use this document", "Known to be open", "Purpose", "Preservation tests", "Layered claim" and "Layer decisions". Read for novelty: the cold readers' known-open list, IN_FORCE.md topics 1 and 2 and every line that greps matched, and round 6's design critique where it touches these points.

By script, with /usr/bin/python3 and PyYAML, nothing written: parsed all 27 promise files (519 promises); recounted layers, levels, per-file counts, the core package, the schemas, the examples, the implementation-defined list and the manifest's 494 cases; compared the README's table row by row; grepped every "word the promises do not use" against the promises and against all of spec/; counted each defined term's uses; listed identifiers and frequent words absent from definitions.md; measured `then` against `observable`; counted language promises that cite other layers.

Read individually, in whole or in part: about 45 promises, those cited in the findings among them.

Not checked:
- I did not read all 519 promises. The layer trial is ten promises, with about 35 more sampled, so the rate of misfiled layers is an estimate.
- I did not check every definitions row against the promises it cites; about 25 rows were checked.
- The count of observables holding rule-like sentences (324 of 519) and the overlap test on the safety catalogue's 153 observable rows are crude text matches. Only MAN-014, MAN-010, SCH-004 and the four `_meta` keys were confirmed by reading as resting on an observable alone.
- The safety catalogue was written from 502 promises at an earlier commit.
- No checker was run and nothing was built, so links and anchors are unverified.
- Rulings' records were consulted only by grep.

Known and not reported as a finding: the three enrollment verbs and the three provider enrollment operations are in no capability and not implemented by `engine` (spec/core/README.md:135-155; IN_FORCE.md:215, staged for 049a and 049b). Under spec/ as it binds today no node but `local` can be made, while README:446 describes managed enrollment in the present tense.


## FR-140

**The form of a promise does not say whether `observable` binds, and rules rest on it alone**

L13/F1 · ambiguity · unsafe · verifier: confirmed, high · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-153.

**Promises.** MAN-014, PND-002, IFC-025, IFC-006, SCH-004, MAN-010

**Where.** spec/README.md:292

**Quote.** "What can be checked to tell whether the promise was kept."

**Situation.** README:291 calls `then` "The single obligation". The `observable` fields hold 69,486 words against 22,622 in `then`, and the coordinator's safety catalogue takes its safety slice from `observable` in 153 of 394 rows. Reading A: only `then` binds. Reading B: every statement of `observable` binds too. Rules that exist under B only: the key `io.github.shrav-k.conduit/unanswered` (spec/promises/pending.yaml:38, spec/interface/implementation.yaml:734, in no `then`); what "raised" means (PND-002); `signInLogin` and `signInReturn` on a sign-in-hook call (IFC-006); "no shell reads the list" (MAN-010, spec/promises/manifest.yaml:223); "Checking a `uri` makes no request to it" (SCH-004). Steps to harm under A: MAN-014's `then` is "The package is rejected." An engine unpacks an archive whose member is `../../x`, then rejects it and reports invalid: `then` is kept, and a file was written outside the package's place, which only spec/promises/manifest.yaml:320 forbids.

**Counterargument.** `observable` says what is checked to tell that the promise was kept, so failing it is failing the promise; the first build and its vectors (S1-archive-escape) treat it so, and many observable sentences only restate another promise's `then`.

**Least change.** In the table of README, "The form of a promise": "`observable` | What can be checked to tell whether the promise was kept. A statement here of what is made, returned, rejected or never done binds as `then` does. A comparison with another system explains and binds nothing." Then move into `then` the rules that stand in an observable alone, starting with MAN-014, MAN-010, SCH-004 and the four `_meta` keys.

**Precedent.** W3C specifications' conformance clause: everything is normative except sections marked non-normative, examples and notes; ISO/IEC Directives Part 2 separates requirements from notes the same way.

**Who decides.** coordinator. It settles an ambiguity and adds no behavior: the build and the vectors already take reading B (rulings 37 and 46 point 4). Logged for the owner to strike.

**Verifier.** confirmed, high. Quote verbatim at spec/README.md:292; :291 reads "The single obligation. For `must not`, the thing that must not happen." Nothing in spec/ says whether an observable binds. Recounted: `then` 22,622 words, `observable` 69,486; docs/implementation/SAFETY_CATALOGUE.md quotes `observable` in 153 of 394 safety rows. The key `unanswered` occurs only in observables (spec/promises/pending.yaml:38, spec/interface/implementation.yaml:734). MAN-014's `then` is "The package is rejected." and only spec/promises/manifest.yaml:320 says "nothing is written outside the place where the engine keeps the package". "no shell reads the list" is at spec/promises/manifest.yaml:223, in an observable. Not in docs/AUDIT.md "Known to be open" or cold-readers. Earlier record: docs/proposals/reaudit/round-6/design-critique.md:595 "Treat `observable` as part of the obligation review: several important decisions live there rather than in `then`" (D21, accepted as proposed, docs/proposals/reaudit/round-6/DECISIONS.md:125). So known-new-situation is the right mark.

**Verifier's corrections.** 1. A stronger unsafe path than MAN-014: the rule that a browser login's return is never kept stands only in observables. spec/promises/pending.yaml:268 "The value appears in no result, structuredContent, response _meta, notification, diagnostic, log or record"; spec/interface/implementation.yaml:1419 likewise. PND-013's `then` bars the value only from a refused return's diagnostic. Under reading A an engine may record an accepted authorization code: a leaked secret. MAN-014's harm is a file written outside the package's place, which is unsafe only if it overwrites engine state or a credential. 2. The basis "the build and the vectors already take reading B" is not the whole record: ruling 41's editors treated `then` as where a rule must stand (docs/proposals/ruling-41/README.md:158 "Where the work order put a rule only in an observable, it was also stated in `then`"). Text has been written under both readings. 3. The proposed blanket sentence would turn every observable that conflicts with another promise's `then` into a binding contradiction (INS-016, F8). Move the observable-only rules into `then` first, or sweep for such conflicts in the same order. 4. Coordinator may decide, but it changes what the language says, so it belongs on the review list.


## FR-141

**"The engine's own panel" carries secret-bearing evidence and is nowhere defined; the nearest definition contradicts it**

L13/F2 · gap · unsafe · verifier: confirmed, high · decides: owner · new

Same fault, or overlapping: FR-193.

**Promises.** PND-004, PND-002, PND-008, ENR-003, ENR-004, ENR-005, IFC-049

**Where.** spec/promises/pending.yaml:76

**Quote.** "The engine alone renders evidence at its panel. Values of kind code or fingerprint appear in no elicitation message, tool result, _meta, notification or record of a request; they are shown only in the engine's own panel and recorded with the answer."

**Situation.** Seven promises send codes, fingerprints and enrollment join actions that carry a secret to "the engine's own panel" or "root's panel". spec/definitions.md:267 defines only "control panel | An Interface at which a person sees and answers pending actions". The panel cannot be an Interface: nothing shown there may travel in a tool result, `_meta`, notification or elicitation, "even from root's tool results" (PND-002). No promise, definition or implementation-defined item says what the panel is, who may view it, or how the engine knows that the viewer is root or "a person permitted to inspect the answer" (PND-004). Steps to harm: builder A renders it in the engine's terminal; builder B serves it as a page at the engine's address. That page is no request, so IFC-032's check of who asks does not apply. B keeps every promise and shows a join secret (ENR-003) or a device code to whoever opens the page.

**Counterargument.** "Root's panel" implies that only root sees it; in v0 the person's Interface runs on the engine's machine and a process of the engine's operating-system user is already inside the domain of trust (spec/README.md:424). Batch 1's plan called it "a root-only channel that no MCP tool exposes", but that is a plan and `spec/` does not hold it.

**Least change.** A row in definitions.md: "the engine's own panel | The engine's own rendering of pending actions and their evidence to a person, outside the protocol: no call, result, notification or consumer carries it." One sentence added to PND-004's `then`: "The panel is shown only to the engine's operating-system user on the machine the engine runs on; the person who reads it answers as root." Its form is added to the implementation-defined list. The row "control panel" then says that an Interface lists and answers actions and the panel alone shows withheld evidence.

**Precedent.** RFC 8628: the device shows the user code on its own display, outside the client's channel; GnuPG's pinentry and polkit's authentication agent are the same trusted path beside an application.

**Who decides.** owner. Who may see secret-bearing evidence, and by what proof, is behavior and trust model; rulings 50 and 56 fixed that the engine alone shows it, not to whom.

**Verifier.** confirmed, high. Quote verbatim at spec/promises/pending.yaml:76-79. spec/definitions.md:267 defines only "control panel | An Interface at which a person sees and answers pending actions", and :60 says an Interface "is also the control panel for pending actions". spec/README.md:402-407 treats the Interface as control panel and "the engine's own panel" as two things. PND-002's observable withholds codes and fingerprints "even from root's tool results", so the panel is not the protocol. No row, promise or item of spec/implementation-defined.yaml says what the panel is, who may view it or how the viewer is established. Only plans do: docs/proposals/spec-batch-1/README.md:643 "over a root-only channel that no MCP tool exposes" (order 051c, unwritten). Not in AUDIT known-open, cold-readers, IN_FORCE Open or Spec pending lines, or the review list.

**Verifier's corrections.** 1. Promise list: PND-002 does not use the word "panel"; PRV-014 does (spec/promises/providers.yaml:457). The seven are ENR-003, ENR-004, ENR-005, IFC-049, PND-004, PND-008 and PRV-014. 2. The unsafe mark holds on a literal reading only: PND-008's "Only root's own panel shows" names the surface, not who may open it, and IFC-032 covers only requests "on a server it presents under IFC-012". The secret actually exposed is ENR-003's join action; a device code or a fingerprint alone gives little. Since the engine's address must be reachable from runs on other machines (PRV-022's CONDUIT_URL), a page there would be reachable by an unconfined Agent. 3. Owner is right: ruling 56 point 4 says "The engine alone, at the root Interface's control panel" (docs/proposals/ruling-56/README.md:23), so a panel outside every Interface rewords a ruling.


## FR-142

**No test in spec/ assigns a promise to a layer, the three statements of the layers differ, and half of a ten-promise trial disagrees with the recorded layer**

L13/F3 · ambiguity · verifier: confirmed, medium · decides: owner · new

Same fault, or overlapping: FR-042, FR-056, FR-098.

**Promises.** LIF-007, LIF-012, FST-001, FST-002, ENR-005, PND-004, PRV-024, ENR-003, FLEET-005, SUB-006, PLC-007, NAM-016, CON-021, PRV-011, SEC-019, OBJ-023, IFC-014

**Where.** spec/README.md:61

**Quote.** "Everything here is answered by resolving, which reads the documents against what already exists, the lifecycle state and the status of the objects that exist among it, such as the labels fixed for a node under ENR-010, and runs nothing."

**Situation.** The only test is LD1 (docs/AUDIT.md:406; "no other record", IN_FORCE.md:37): "Language if it can be answered by reading the declarations alone. Shared convention if two independently written parts would break each other, or a safety promise, by disagreeing. Type contract if it matters only to instances of one type. Runtime profile otherwise." It has no outcome `interface`, and its third clause sends a runtime provider's duties outside the specification. README:61-63 describes the layers and admits existing state, which LD1 excludes. README:65 says provider contracts are "not a fourth layer"; docs/AUDIT.md:395 says "one layer beside these". Trial, one promise per file. Disagree with the recorded layer: LIF-007 (language; what holds after a retirement; its twin LIF-012 is convention), FST-001 (language; no document is read; FST-002 and FST-003, same `when`, are convention), ENR-005 (language; what a provider reports and the panel shows), PND-004 (interface; same shape as PRV-024, convention), FLEET-005 (convention; which objects a registration makes, restated as language in SUB-006 and INS-016). Turn on the wording: PLC-007, NAM-016 (CON-021 is language). Undecidable: PRV-011. Agree: OBJ-023, IFC-014.

**Counterargument.** The files follow a workable habit: what resolving decides about a document is language, what a request reports or does is convention, what a message holds is interface. The disagreements are a handful of promises filed against that habit, and README:270 says the `layer` field decides.

**Least change.** Under the layer table in README: "A promise is `language` where its `when` is a document, a package or a reference being read, registered, instantiated or installed and its `then` says what is made, what it holds or what is rejected. It is `interface` where its `then` says what a message of the protocol holds. It is `convention` otherwise." Then relayer the promises the test moves, LIF-007, FST-001 and ENR-005 first, and make AUDIT.md:395 agree with README:65.

**Precedent.** Kubernetes separates API conventions (what an object means and what is valid) from each controller's behavior, the comparison the layered claim itself makes.

**Who decides.** owner. LD1 is the owner's ruling 14 and has no other record; a test that adds `interface` and places provider duties amends it. Relayering single promises afterwards is the coordinator's (ruling 46 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/README.md:61. LD1 is at docs/AUDIT.md:406 and has four outcomes, none `interface`. IN_FORCE.md:37 "have no other record than" the layered claim. spec/README.md:65 "not a fourth layer" against docs/AUDIT.md:395 "Ruling 40 adds one layer beside these". Recorded layers checked: LIF-007 language, LIF-012 convention; FST-001 language, FST-002 and FST-003 convention with the same `when`; ENR-005 language ("The panel shows the value exactly as the provider reported it"); PND-004 interface, PRV-024 convention; FLEET-005 convention, restated in SUB-006 and INS-016 (language).

**Verifier's corrections.** 1. "No test in spec/" is overstated: spec/README.md:61 gives a criterion for `language` ("answered by resolving ... and runs nothing") and :65-72 assigns the provider promises by kind. 2. PRV-011 is not undecidable: spec/README.md:69-70 names it as `convention`. 3. FST-001 is defensible as language: the first-start objects "are instances of its templates" (spec/README.md:181). The clear misfit under the README's own criterion is ENR-005; LIF-007 and PND-004 are arguable. So three or four of ten disagree, not five. 4. No builder diverges on behavior, since "A promise's own `layer` field decides" (spec/README.md:270). The cost falls on what the Language level and LD4's independent build cover, and on writers of new promises. Closer to a defect (misfiled promises, unstated criterion) than an ambiguity. 5. Relayering ENR-005 against README:61 needs no owner; replacing LD1's words does.


## FR-143

**Conformance levels turn on whom a promise binds, which no field records; no level covers a package's author**

L13/F4 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-154.

**Promises.** PRV-022, PRV-011, PRV-019, NAM-024, NAM-025, PKG-031, PKG-016, SEC-006

**Where.** spec/README.md:322

**Quote.** "Every promise, whatever its layer, whose obligation falls on the implementation and not on the engine."

**Situation.** The Engine and Implementation levels (README:321-322) are defined by where an obligation falls. A promise has `id`, `layer`, `level`, `when`, `then`, `observable` and `vectors` (README:287-293) and nothing that says whom it binds. `layer` does not say it: a runtime provider's duties are `convention` (PRV-011, PRV-019) and other implementations' duties are `interface` (SEC-006, PKG-016). The writer of a provider must read 293 convention and interface promises to find their own, and PRV-022 binds the engine and the provider in one `then`; two writers will draw different sets. Promises that bind the author of a package are `language` (NAM-025 and PKG-031, both `should`; NAM-024), so the Language level, which only an engine claims, counts duties no engine can keep, and nothing lets a package or a contract claim conformance. The split between `convention` and `interface` decides nothing at any level; it serves only the count by layer (README:303).

**Counterargument.** README:322 lists examples for each level, and the `when` of most promises names its subject ("A runtime provider is given..."), so a careful reader can usually tell.

**Least change.** Add to the form of a promise: "`binds` | Who must keep it: `engine`, `implementation`, `provider` or `author`; more than one where the `then` names each." Reword the levels: Engine, "every promise that binds the engine"; Implementation, "every promise that binds an implementation or a provider"; and add "A package or a project conforms where it keeps every promise that binds its author."

**Precedent.** RFC 9110 section 2.2 states every requirement by role (sender, recipient, client, server); the W3C QA Framework's Specification Guidelines require each requirement to name its class of product.

**Who decides.** coordinator. The field records what each `then` already says and adds no behavior (rulings 37 and 46 point 4); the added sentence on packages is logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at spec/README.md:322; :321 uses the same test ("whose obligation falls on the engine"). Every promise has exactly id, layer, level, when, then, observable, vectors (script over all 519). Both level rows give examples only ("Examples are", "such as"). NAM-025 and PKG-031 are `language`, `should`, and bind a package's author; PKG-016 and SEC-006 are `interface` and bind implementations; PRV-011 and PRV-019 are `convention` and bind a provider. 245 + 48 = 293 convention and interface promises. No list of who is bound exists in spec/README.md, spec/core/README.md or the head of spec/promises/providers.yaml. No earlier record found.

**Verifier's corrections.** 1. The added sentence "A package or a project conforms where..." is a fourth conformance class: a feature, so the owner's. The three levels are ruled (IN_FORCE.md:41, rulings 24 PD5 and 38 point 12a). 2. A `binds` field changes the ruled form of a promise (IN_FORCE.md:40, ruling 17 NS2) and edits 519 records. A lighter fix with the same effect: complete the two lists in the Conformance table, written by a checker. Either way it goes on the review list.


## FR-144

**The Language level is not self-contained: language promises lean on convention and interface promises**

L13/F5 · ambiguity · verifier: weakened, low · decides: coordinator · new

**Promises.** IFC-014, IFC-012, SUB-006, INS-016, FLEET-005, PLC-007, PLC-008, ENR-010, OBJ-013, PLC-002

**Where.** spec/README.md:320

**Quote.** "Every promise whose layer is `language`: documents in, resolved objects or diagnostics out."

**Situation.** By script, 17 of the 226 language promises name a convention or interface promise in their `when` or `then`, and 94 in any field. IFC-014 rejects a Conduit by the tool names of IFC-012 (interface). SUB-006 and INS-016 make and open the default Conduits "under FLEET-005" (convention), which alone gives their operation lists. PLC-007 and PLC-008 read labels and nodes fixed under ENR-010 and ENR-007 (convention). OBJ-013 leans on PLC-002 (convention) for an instance given no placement. Reading A: a Language-level engine keeps the `language` promises only, so it need not compute a tool name or give a default Conduit its operations, and IFC-014 and SUB-006 cannot be checked against it. Reading B: it also keeps whatever a language promise names, so the level takes in parts of FLEET-005, IFC-012, ENR-010 and PLC-002 that the README does not list. Two builders claiming the Language level return different resolved objects for one Pool document.

**Counterargument.** Only an Engine-level engine is being built, and it keeps all three layers, so the difference shows only when a second builder claims the Language level alone (LD4's independent build).

**Least change.** Add to the Language row of the table in README, "Conformance": "A language promise that names a promise of another layer in its `when` or `then` takes in what it names. Where PLC-002 would have a machine provider place an instance, the engine's own machine provider serves (PRV-003)."

**Precedent.** A conformance clause that lists by reference the parts a class must satisfy, as POSIX conformance classes and W3C conformance sections do.

**Who decides.** coordinator. It settles which of two readings holds and adds no behavior: reading B is what SUB-006 and IFC-014 already need (rulings 37 and 46 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/README.md:320. Script confirms 17 language promises cite a convention or interface promise in `when` or `then`, 94 in any field. Deciding text against two of the examples: spec/core/templates.yaml:145-160 holds `root-pool`'s operation list ("operations: - ...#register.v1 ... - ...#answer.v1"), so FLEET-005 is not what "alone gives their operation lists"; and spec/README.md:61 already takes in "the labels fixed for a node under ENR-010".

**Verifier's corrections.** Smaller, and a defect in README:320's description more than an ambiguity. Reading A is not sound where a `then` cites the other promise by ID: IFC-014's "tool names under IFC-012" cannot be applied without IFC-012. What remains: the Language row does not say that it takes in IFC-012's naming rule, FLEET-005's choice of template and local name, and, for an unplaced instance, PLC-002 and PRV-015's provider call, which is not "documents in, resolved objects or diagnostics out". The proposed sentence fits; the coordinator decides.


## FR-145

**The README's account of the convention layer no longer matches it: 245 promises, 113 of them outside the seven conventions' files**

L13/F6 · defect · verifier: confirmed, medium · decides: owner · known-new-situation

Same fault, or overlapping: FR-160.

**Promises.** PRV-010, PRV-019, PRV-021, PND-010, PND-011, PND-013, ENR-001, ENR-006, TPL-012, ACC-033

**Where.** spec/README.md:62

**Quote.** "The few behaviors every part must agree on, stated as invariants. There are seven shared conventions: idempotency keys and outcomes; revision-checked change; access; declared against observed state; ownership and lifecycle; records; and that nothing starts model work or repeats an effect implicitly."

**Situation.** Recount at this commit: 245 convention promises against 226 language (round 6 counted 207 and 202). The seven files under `conventions/` hold 132 of them; 113 sit in 15 of the 19 topic files: providers 26, secrets 15, pending 12, enrollment 10, placement 10, conduits 9 and others. Four bodies of convention-layer behavior belong to none of the seven named conventions: a provider's duties (PRV-010 to PRV-030), the engine's reads of secrets, pending actions and sign-in, and enrollment. Several are procedures and not invariants: TPL-012 orders a setup's parts, PND-010, PND-011 and PND-013 walk a browser login step by step, ACC-033 is a table of 20 rows in one `then`. The layered claim (docs/AUDIT.md:381-382) calls the language "small" and the conventions "few", "Specified as invariants, not step by step". By LD1's own third clause, sign-in and a runtime provider's duties matter only to instances of some types and would be type contracts. The README promises less than the promises hold.

**Counterargument.** Ruling 40 put provider contracts in the specification on purpose, and rulings 49, 50 and 56 ruled sign-in and enrollment; README:65-72 says provider promises fall in the three layers. The count is a reading burden, not proof that a rule is misplaced.

**Least change.** The owner says whether the claim still stands as worded. Then README:62 reads: "What every part must agree on: the seven shared conventions, and what the engine and a provider owe each other under each provider capability, pending actions and enrollment among it." Where the owner holds to "few, as invariants", sign-in's steps and the provider-side duties move to the Type and provider contracts that LD1 names.

**Precedent.** Kubernetes keeps CRI, CSI and CNI as contracts of their own beside the API conventions, the comparison ruling 40 makes.

**Who decides.** owner. The layered claim is ruling 13 in the owner's words; whether it still describes the specification is scope and purpose.

**Verifier.** confirmed, medium. Quote verbatim at spec/README.md:62. Recount: 245 convention, 226 language, 48 interface. 132 convention promises in the seven files of conventions/, 113 in 15 of the 19 topic files (providers 26, secrets 15, pending 12, enrollment 10, placement 10, conduits 9). docs/AUDIT.md:381-382 "Fully specified; small" and "Specified as invariants, not step by step". TPL-012's `then` begins "The setup is these parts, in this order"; ACC-033's `then` is 486 words of numbered rows. Earlier records: docs/guides/full-review.md:116 asks the question; docs/proposals/reaudit/round-6/design-critique.md:588 "\"Few shared conventions\" names seven topics, not a small behavioral specification", at 207 convention promises.

**Verifier's corrections.** The README is not silent on two of the four bodies: spec/README.md:65-72 says what the engine asks of a provider, "by a provider call or by a read of a secret, and what a provider must do are `convention`". Unaccounted for are pending actions and sign-in, enrollment (22 convention promises in pending.yaml and enrollment.yaml), and the convention promises in placement and conduits. Known-new-situation is right. Owner decides whether the claim stands; a sentence at README:62 that only reports where convention promises now sit is the coordinator's.


## FR-146

**"Limits of this version" holds ruled behavior that no promise states, and IN_FORCE counts it as written in spec/**

L13/F7 · contradiction · verifier: weakened, medium · decides: owner · new

**Promises.** PKG-031, SCT-003, SCT-015, CTR-013

**Where.** spec/README.md:333

**Quote.** "They are statements, not promises, and nothing tests them; each point cites the promises that state its parts."

**Situation.** The section states duties that no promise states and that cite rulings, not promises. Plan and apply: "Every dangerous act needs a named flag, counted when used", "A plan prints and counts every widening", "Narrowing never waits" (README:351-361), called "tooling duties" at README:370. Across parties: "A relationship across parties has two halves", "A partner's caller chain is recorded and is not used in access decisions" (README:439-444). Where a package that keeps a login runs (README:426). IN_FORCE.md marks each "In spec/" (lines 77, 79, 95, 194, 218, 219), and AGENTS.md says accepted behavior is "a promise in `spec/`". The README itself says a tool's commands belong to that tool (README:113). So a second `conduit` tool that reopens, prunes or widens without a flag, used with an engine that keeps every promise, breaks nothing that can be claimed or tested: no promise, no level and no vector covers a tool.

**Counterargument.** The paragraphs are honest about being statements; plan and apply are made of existing requests that root is permitted to send, and ruling 76 treats them as tooling.

**Least change.** The owner chooses. (a) Tooling stays outside: the tool sentences move to the tool's reference, the IN_FORCE lines read "Not in spec/: tooling", and README:333 keeps its claim. (b) Tooling binds: the duties become promises with a level "Tool" in the table of "Conformance". Under either, README:333-334 says which sentences of the section state decisions that no promise holds.

**Precedent.** Terraform documents plan and apply with its CLI and not in the configuration language's specification; kubectl's `--prune` and `--force` are in kubectl's reference, not in the API conventions.

**Who decides.** owner. Whether tools are within the specification, with a conformance class, adds or removes a feature; rulings 37 and 46 point 4 give the coordinator only fixes that add no behavior.

**Verifier.** weakened, medium. Quote verbatim at spec/README.md:333-334. Tool duties with no promise at spec/README.md:351-363 and :369-370 ("These are tooling duties"); cross-party sentences at :439-444 cite only "ruling 51, decisions 1, 4, 6, 7, 9 and 10"; :426-427 cites only "ruling 75 C1". IN_FORCE.md:77 "**In spec/**: spec/README.md, \"What the short form's meaning covers\". Plan and apply are made of today's requests"; lines 79, 95, 194, 218 and 219 likewise. AGENTS.md, "Where things go": "Accepted behavior: a promise in `spec/`". spec/README.md:113 "the guide to a tool's commands belongs to that tool". No conformance level covers a tool (spec/README.md:318-322). Not in the review list, AUDIT known-open or the increment logs.

**Verifier's corrections.** Class: not a contradiction in the brief's sense, since no two promises conflict. It is a gap (ruled tool behavior that no promise holds, which AGENTS.md calls a gap in spec/) and a defect in README:333-334: "each point cites the promises that state its parts" is false for the points that cite only rulings. The harm from a second tool is hypothetical in v0; the present cost is that the IN_FORCE "In spec/" lines point at text that says it binds and tests nothing, so the flag rule of rulings 53 point 4 and 55 A4 has no vector. Owner is right. Related and already recorded: IN_FORCE.md:78 holds ruling 76's amendments as spec pending.


## FR-147

**"Register: nothing runs" and INS-016's observable deny the provider call that registration makes to place an instance**

L13/F8 · contradiction · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-097.

**Promises.** INS-016, PRV-015, PLC-002, PRV-026, SUB-006

**Where.** spec/promises/instantiation.yaml:233

**Quote.** "After instantiation no setup has run, no Conduit other than a default made under FLEET-005 has been opened, and no implementation has received a call."

**Situation.** An Agent or a Resource with an item that is run is registered with no placement, and a machine provider Resource exists. PRV-015 (spec/promises/providers.yaml:460): "The engine asks the machine provider with `place.v1`", "a provider call under PRV-005, made while the engine decides the request". After a restart, PRV-026 first has the engine's own runner run that provider again and read its secrets. So a registration starts a process, reads secrets and calls an implementation. INS-016's observable says no implementation has received a call. spec/definitions.md:36 says "register | Store the templates and instances that a submission's documents make; nothing runs." README:61 says the language is answered by resolving, which "runs nothing", while the placement of that instance, part of its resolved form, comes from a provider. A vector written from INS-016's observable fails a conforming engine.

**Counterargument.** The row "resolving" (spec/definitions.md:101) says plainly that placing is not resolving and that the engine asks the provider while it decides the registration; the call comes before the request is dispatched, so "after instantiation" can be read as what instantiation itself causes.

**Least change.** INS-016's observable ends: "and no implementation has received a call, apart from the machine provider's `place.v1` under PRV-015 and what PRV-026 does before it." spec/definitions.md:36 reads: "Store the templates and instances that a submission's documents make; nothing is set up, and the only call is the machine provider's under PRV-015."

**Precedent.** Kubernetes: creating a Pod runs nothing of it, and the scheduler's binding is a separate, named step, the comparison PRV-015's own observable draws.

**Who decides.** coordinator. Two texts that cannot both hold; the fix states what PRV-015 already requires and adds no behavior (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/promises/instantiation.yaml:233-235. PRV-015's `then` at spec/promises/providers.yaml:468 "The engine asks the machine provider with `place.v1`". spec/definitions.md:207 makes a provider call a call that "the engine sends an implementation". spec/definitions.md:36 "nothing runs"; :101 "the engine asks the machine provider while it decides the registration". PRV-026's `then` has the runner "run that provider again" with secrets read, before the call. docs/implementation/SAFETY_CATALOGUE.md:77 takes INS-016's safety slice from this clause: "no implementation has received a call". No earlier record found.

**Verifier's corrections.** It is a contradiction between promises only if observables bind (F1); otherwise PRV-015's `then` governs and INS-016's observable is a wrong check. No builder diverges, since PRV-015 requires the call. It arises only where a machine provider Resource is registered, so first at S3, and a safety vector written from the catalogue's row would fail a conforming engine there. That the call is made "while the engine decides the request ... before it is dispatched" is itself only in PRV-015's observable.


## FR-148

**A reader does not meet the limits on logins, sign-in and enrollment before relying on them**

L13/F9 · defect · verifier: confirmed, low · decides: coordinator · new

**Promises.** PRV-029, CTR-013, PND-005, PND-010, PND-011

**Where.** spec/README.md:426

**Quote.** "Until Agents are confined, a package keeping a Resource's login runs where no unconfined Agent runs; the engine does not check that placement in v0 (ruling 75 C1)."

**Situation.** Three of the six limit sections are reachable from where a reader relies on the thing: the short form (README:114), access (author's step 4; the row "bound") and secrets (author's step 7; two rows). The other three are linked from no reading path, no definitions row and no promise file's head: "Fleets, discovery and tombstones", "Pending actions, sign-in and the network" and records. `promises/pending.yaml` and `promises/enrollment.yaml` (25 promises) are on no reading path, though README:167 says the engine builder's steps follow the Contents table. A Type author who writes `signIn` for a shared login (the S2 case) follows the contract writer's path, reads CTR-013, and never meets README:418-427: the package holds the credential of every login it runs, a process of the same operating-system user reads every state directory, and the placement rule quoted is unchecked. On the first user's one machine, with every Agent unconfined, that rule cannot be met at all. spec/definitions.md:279 calls the state directory "private" and drops PRV-029's "to the engine's operating-system user".

**Counterargument.** The limits are stated in full in the README, an engine builder is told to read the whole section (README:188), and PRV-029's observable repeats the point about the same user.

**Least change.** Add `promises/pending.yaml` to the author's step 6 and the contract writer's step 2, and `promises/enrollment.yaml` to the engine builder's step 5, each followed by "then [Pending actions, sign-in and the network]". Link the rows state directory, login, sign-in, tombstone and record to their limit sections, and write "private to the engine's operating-system user" at definitions.md:279.

**Precedent.** RFC 3552: security considerations are referenced from the feature they limit; Kubernetes documentation places its cautions on the feature's own page.

**Who decides.** coordinator. Links and one restored qualifier; no behavior changes (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/README.md:426-427. Links from reading paths: spec/README.md:114 (short form), :131 (access), :138 (secrets), :188 (the whole section, engine builder only). spec/definitions.md links only #what-access-does-not-narrow and #what-the-guarantee-for-secrets-covers. promises/pending.yaml and promises/enrollment.yaml (13 + 12 promises) appear in no step of spec/README.md:122-189. spec/definitions.md:279 "The engine runner's private persistent directory for one instance" against PRV-029 "private to the engine's operating-system user (mode 0700 on systems with those modes)".

**Verifier's corrections.** "Linked from no reading path" is slightly overstated: spec/README.md:188 sends an engine builder to all the limits, and PND-013's observable points to spec/README.md (spec/promises/pending.yaml:293). The author's and the contract writer's paths do miss them. The sentence on the first user's one machine is a separate, larger point than a missing link: see extra 1.


## FR-149

**Definitions rows that contradict the promises or each other**

L13/F10 · defect · verifier: confirmed, low · decides: coordinator · new

**Promises.** IFC-006, IFC-025, IFC-049, PND-002, PND-003, OPS-036, ACC-055, PRV-029

**Where.** spec/definitions.md:296

**Quote.** "On a request that a consumer or an instance sends to the engine they are `idempotencyKey`, which a consumer may supply (IFC-023), and `serving`, on an onward request (IFC-020)"

**Situation.** definitions.md restates the rules (28,239 words; 1,311 citations of 420 of the 519 promises) and has drifted since batch 1. (1) "Conduit's keys" (line 296) omits four keys the promises put on the wire: `pendingAction` (PND-005), `unanswered` (PND-002, IFC-025), `signInLogin` and `signInReturn` (IFC-006, IFC-049); a builder who takes the row as the list rejects or drops them. (2) Line 222: the promises use "'answer' only as a verb"; line 269 defines "answer" as a noun, and PND-003 uses it so. (3) Line 302: "hold" is "not used for a Conduit or a right"; line 163 defines scope by "a holder", and nine promises say holder, as in "A holder of an open Conduit to `management`" (OPS-036). (4) "evidence" has two rows (270, 322). (5) "definition" has three senses (383; "sectioned definition", 68; a tool's), against the heading's "each with one meaning". (6) Line 78: "A package is a project that has been published", while a project is what is handed over "in one registration" and a package is never registered. (7) "core package" (399) lists none of the Fleet, pending, sign-in or enrollment exports that spec/core holds.

**Counterargument.** The file says the promises govern where a row names one, so no row can change behavior; most rows are accurate.

**Least change.** Correct the seven rows from the promises they cite. For row 1: "...and `serving`, on an onward request (IFC-020), and on a sign-in `signInLogin` and `signInReturn` (IFC-049); on a result ... `pendingAction` and, for an Interface, `unanswered` (IFC-025)." Longer term, shorten rows to the meaning and the governing promise, so that a rule has one text.

**Precedent.** ISO/IEC Directives Part 2: a definition does not take the form of, or contain, a requirement; requirements stay in the body.

**Who decides.** coordinator. Defects in text that the promises already govern; no behavior changes (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/definitions.md:296; the row names none of pendingAction, unanswered, signInLogin, signInReturn (PND-005 `then`; PND-002 and IFC-025 observables; IFC-006 and IFC-049 observables). :222 "and \"answer\" only as a verb" against the row "answer" at :269. :302 "The word is not used for a Conduit or a right" against :163 "A holder's open Conduits" and OPS-036 "A holder of an open Conduit to `management`"; nine promises say holder. "evidence" rows at :270 and :322. :78 "A package is a project that has been published". Recount: 28,239 words, 27 rows over 200 words, "connection" 445; my regex finds 1,309 citations of 419 promises.

**Verifier's corrections.** Item 7 is partly wrong: the row at spec/definitions.md:399 does name "the three sign-in hooks". What it lacks is the Fleet capability and type, the enrollment operations and the schemas inventory, tombstone, provenance, trust-domain, pending-action, evidence-item and refusal that spec/README.md:224-228 lists. Items 2, 4 and 5 are wording nits that change no meaning; the file says the promises govern. Item 1 is the one a builder could act on. FST-007's "holder" is a credential's, not a Conduit's.


## FR-150

**Rule 4 is not kept: terms of art without a row, and "words the promises do not use" that the promises use**

L13/F11 · defect · verifier: confirmed, low · decides: coordinator · new

**Promises.** PND-009, PRV-005, CTR-014, CTR-004, PRV-018, ENR-004, ACC-056, LIF-021, ACC-033, ENR-007

**Where.** spec/README.md:300

**Quote.** "4. Every term is defined in [definitions.md]\(definitions.md)."

**Situation.** By script over the `when`, `then` and `observable` of all 519 promises. Terms of art with no row: panel (7 promises), person (13; "user" has a row, and ENR-007 rests on "the person controls the machine"), client (27, in two senses: the protocol's, and the vendor program that drives an Agent), subject of a pending action (more than 20), tier of ACC-056 (8), cascade (5), live as in "a live existing Pool" (3; three rows of definitions.md use it too), dismissed (4), and the operations `answer.v1` (9), `retire-with-resources.v1` (7) and `retire-with-agents.v1` (3). Old words in use: "setup operation" (PND-009, spec/promises/pending.yaml:115, against definitions.md:461); "policy" as "the provider's records policy" (PRV-005, against line 430); "grant" for access, as in "no Conduit grants it" (CTR-014, spec/promises/contracts.yaml:349; also PRV-018, ENR-004, ENR-009 and LIF-037, against line 421); "declaration" for a Type's own tools (CTR-004, against line 422). Rows no promise uses: control panel, decision point (it says so itself), sign-in hooks. No other old word was found in a promise in its ruled-out sense.

**Counterargument.** Most uses are negations ("grants nothing") or ordinary words, and a reader is not misled by them; a client is the protocol's term.

**Least change.** Add rows for person, client (both senses), subject, cascade and the engine's own panel (F2). Reword the four places: "through `set-up.v1`" in PND-009; "whatever the provider's Type states under `records`" in PRV-005; "no Conduit permits it" in CTR-014; and let line 421 allow "grants nothing". Add the two greps to `tools/` beside the existing checkers.

**Precedent.** ISO/IEC Directives Part 2, terms and definitions: a defined term is used in one sense throughout, and a term that is used is defined.

**Who decides.** coordinator. Vocabulary is ruled (ruling 42's audit) and the promises have slipped from it; restoring it changes no behavior (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/README.md:300. Script over `when`, `then` and `observable`: panel 7, person 13, client 28, cascade 5, dismiss 4, answer.v1 9, retire-with-resources.v1 7, retire-with-agents.v1 3; none has a row in spec/definitions.md. Old words: spec/promises/pending.yaml:115 "only through its setup operation" against spec/definitions.md:461; PRV-005 "whatever the provider's records policy" against :430; spec/promises/contracts.yaml:349 "no Conduit grants it" against :421; CTR-004 "the own tools declaration of CTR-014" against :422. No promise says "control panel", "decision point" or "sign-in hooks".

**Verifier's corrections.** Small additions: ACC-056's observable also says "grants no answering authority" and "grants no request authority"; IFC-016 says "Without that declaration" of signIn; "tier" is in 7 promises by my count, not 8. Most of the "grant" uses are the verb in a negation and mislead nobody. The checker in tools/ is the useful part of the fix.


## FR-151

**The README names two exceptions to the preservation tests; the promises make more**

L13/F12 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** ACC-032, ACC-057, SEC-008, ACC-024, LIF-022, CTR-014, FLEET-005

**Where.** spec/README.md:380

**Quote.** "Two named exceptions to the [preservation tests]\(../docs/AUDIT.md#preservation-tests) apply."

**Situation.** Test 5: "Every management action is available to an agent under a grant". Beyond sign-in and answering, which the README names, the promises withhold from every Agent: the five operations reserved to root, two of them management verbs by spec/definitions.md:204, declaring lost and purging (ACC-032); registering an Agent, the purpose's "start a peer" (ACC-057); and opening any Conduit that lists `get-secret.v1` (SEC-008). Test 2: "Every access is a relationship with a grant that can be inspected and revoked". The README says root's defaults "can be revoked only by retiring their targets"; root's three first-start Conduits cannot be revoked at all, since no request closes them (ACC-024) and their targets are never retired (LIF-022). An Agent's own tools are access that "no Conduit grants", "no restriction limits" and "no record shows" (CTR-014), outside tests 2 and 7. An owner who checks v0 against the purpose from this paragraph undercounts where it departs.

**Counterargument.** Each is ruled (59 A1, 66 point 3, 70) and IN_FORCE.md topic 1 lists most of them; the README's paragraph may have meant only the exceptions that Fleets added.

**Least change.** Replace the paragraph's first sentence with: "These exceptions to the preservation tests apply in this version." Then list, for test 2, root's first-start and default Conduits and an Agent's own tools; for test 5, sign-in and answering, the operations reserved to root (ACC-032), registering an Agent (ACC-057) and Conduits that list `get-secret.v1` (SEC-008); each with its promise.

**Who decides.** coordinator. The text is wrong against the promises; listing what they already do adds no behavior (rulings 37 and 46 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/README.md:380. ACC-032's observable: "The five are declare-lost.v1, purge.v1, enroll-machine.v1, admit-machine.v1 and refuse-machine.v1". ACC-057: dispatched "only where root sends it on its own behalf". SEC-008 likewise. ACC-024 and LIF-022 leave root's three first-start Conduits with no way to close. Already recorded: IN_FORCE.md:21 "Rulings limit this in v0. Only root registers Agents (59 A1) and admits machines (56 point 11) ... Enrollment and secrets stay with root (66 point 3)"; IN_FORCE.md:119 holds the closing exception. The README's sentence comes from batch 1, which meant the two it added (docs/proposals/spec-batch-1/README.md:176).

**Verifier's corrections.** Smaller than stated. Each exception is ruled and most are in IN_FORCE topic 1; what is wrong is only that README:380's "Two" reads as the whole list, and that declaring lost, purging and Conduits listing `get-secret.v1` are named nowhere as limits on test 5. Own tools are no departure from test 7, which covers what an agent does "through Conduit" (docs/AUDIT.md:304): CTR-014 says their reach "does not pass through Conduit", and spec/README.md:82-88 already states them. Coordinator may fix the sentence.


## FR-152

**Coverage is kept in three places that disagree, and three README sentences are stale**

L13/F13 · defect · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-157.

**Promises.** CTR-013, SCH-009

**Where.** spec/README.md:293

**Quote.** "The conformance vectors that test it. See the dated coverage status above."

**Situation.** Recount by script. Files, promise counts (519: 226, 245, 48), core documents (147 exports; 37, 8, 6, 2, 13, 12 and 17), schemas (52 in 9 files), examples (14 and one dot file in 6 directories; 74; 41), implementation-defined items (17) and vectors (494 cases) all match the table. Coverage does not. The `vectors` field is filled in 2 promises of 519 (CTR-013, SCH-009), so rule 5 (README:301) reads 517 as uncovered. The table's counts add to 274 promises; the manifest's case tags cover 292, and five rows lag: access 23 against 25, lifecycle 18 against 25, revisions 4 against 9, conduits 12 against 14, objects 13 against 15. The manifest's `s1_coverage` gives 226. Stale: README:644 "no promise is tested yet" against README:7; README:642 "the four checks that AGENTS.md lists", which lists eight; README:268 "two hold promises of the layer `interface`", where four topic files do (enrollment, packages, pending, secrets). README:167 says the builder's steps follow the Contents table, but they begin with `sections.yaml`, the table's tenth promise file, and skip two.

**Counterargument.** README:193 dates the coverage column (2026-11-10) and sends the reader to the manifest, which calls itself a runner's index and not a promise.

**Least change.** Drop `vectors` from the form of a promise and from the two promises that fill it, and let rule 5 read "A promise that no case of the coverage manifest names is not yet tested." Replace the table's vector counts with a column a checker writes from the manifest. Correct README:268, 642 and 644, and README:167 to "in this order".

**Precedent.** web-platform-tests: each test names the section it tests and the specification carries no list of tests; Kubernetes generates its conformance list from test metadata.

**Who decides.** coordinator. Stale counts and a field the build does not use; no behavior (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/README.md:293. Recount: `vectors` filled in CTR-013 and SCH-009 only. The table's coverage counts sum to 274; the manifest's 494 cases tag 292 promises; five rows lag (access 25 against 23, lifecycle 25 against 18, revisions 9 against 4, conduits 14 against 12, objects 15 against 13); `s1_coverage` has 308 entries, 226 with vectors. spec/README.md:644 "no promise is tested yet" against :7; :642 "the four checks" where AGENTS.md lists eight; :268 "two hold promises of the layer `interface`" where enrollment, packages, pending and secrets do. Promise, core, schema, example and implementation-defined counts match the table.

**Verifier's corrections.** 1. sections.yaml is the table's tenth row and its seventh promise file. 2. The lag is declared: spec/README.md:193 dates the coverage at 2026-11-10. 3. Dropping `vectors` changes the ruled form of a promise (IN_FORCE.md:40, ruling 17 NS2): log it for the owner, or keep the field and have a checker fill it from the manifest. The stale sentences are the coordinator's.


## Further points the verifier noticed

Not verified by a second reader.

- **The limit on where a login-keeping package runs cannot be met on the first user's one machine, and S2's login probe plans against it** (spec/README.md:426). "Until Agents are confined, a package keeping a Resource's login runs where no unconfined Agent runs; the engine does not check that placement in v0". The reviewer put this inside F9; it is a separate point. Agents run unconfined in v0, and the first user has one machine, so the condition has no satisfying placement there. experiments/013-s2-shared-login/plan.md:240-241 has S2-LOG-P keep "the token in its own state directory (75 C1; the engine holds none, SEC-020)" on the same machine, where PRV-029's observable says "A process under the same operating-system user may still read it". No promise states the rule and nothing checks it. Medium. The owner decides (ruling 75 C1 against ruling 70 and the S2 scenario): either the limit says what holds when the condition is unmet, or the probe runs the package apart.
- **The rule that a browser login's return is never kept or logged stands only in observables** (spec/promises/pending.yaml:268). "The value appears in no result, structuredContent, response _meta, notification, diagnostic, log or record" and "The engine holds it in memory only until the hook retry ends" are in PND-013's observable (which starts at line 249); IFC-049 repeats it in its observable (spec/interface/implementation.yaml:1419). PND-013's `then` bars the value only from a refused return's diagnostic. This is the clearest unsafe instance of F1 and the first sentence to move into a `then`. Coordinator's fix; it adds no behavior.

## The verifier's notes

Checked at 2eb8f258 (2026-11-12). All thirteen quotes are verbatim at the cited lines. Result: 10 confirmed (F1, F2, F3, F4, F6, F8, F9, F10, F11, F13), 3 weakened (F5, F7, F12), none refuted, none known. Two high (F1, F2), four medium (F3, F4, F6, F7), seven low.

Recounts that match the reviewer: 519 promises (226 language, 245 convention, 48 interface); 132 convention promises in conventions/ and 113 in 15 topic files; `then` 22,622 words and `observable` 69,486; 17 and 94 language promises citing another layer; manifest 494 cases tagging 292 promises, table sum 274, s1_coverage 226 of 308; safety catalogue 153 of 394 rows quoting `observable`; README limits 3,641 of 8,816 words. Small differences: 1,309 citations of 419 promises in definitions.md by my regex (reviewer: 1,311 of 420); "tier" in 7 promises (reviewer: 8).

Main corrections: F1 has a stronger unsafe path than the one given (PND-013's return), and its blanket fix would make F8 a binding contradiction; F2's promise list swaps PND-002 for PRV-014; F3's "no test in spec/" and its count of five are overstated; F5's claim that FLEET-005 alone gives the default Conduits' operation lists is wrong (spec/core/templates.yaml:145-160); F7 is a gap and a defect, not a contradiction; F10's item 7 is partly wrong; F12's exceptions are already ruled and mostly in IN_FORCE.md:21. On who decides: F4's new conformance class and F13's dropping of the `vectors` field touch ruled text (IN_FORCE.md:40-41) and should be logged for the owner.

The reviewer's assessment holds on the facts I checked: capital-I Interface, Pool and Fleet first appear in the README at lines 225, 224 and 208.

Disclosure: the rules say to create nothing anywhere. I wrote four small helper scripts (load.py, f1.py, show.py and f5.py), with the Python cache directory they left, to the session scratchpad to parse the promise files. Nothing was written, built or run inside the repository root or anywhere else, no git command wrote, and no environment or credential was read. I left the scratch files in place rather than delete them.
