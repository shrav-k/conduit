# L01-access: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The core of the access convention is sound and small: every permission is an open Conduit, each request is decided anew, the right to open is itself a bounded Conduit, and an onward request runs under the sender's own Conduit. A builder can follow most `then` clauses without asking. It is not minimal: 35 promises carry 6,401 words of observables that largely restate other promises, and ACC-033 (a table of some 20 rows) and ACC-056 (59 cause rows, with wire detail in a convention promise) are each several obligations in one record. The weakest point is the two-tier refusal: which promise refuses is not determined, the specific cause contradicts tier 1's independence from the named object, and a bounded holder of `open.v1` can tell from the reason whether any Conduit name exists (F2 to F4). The one finding with an unsafe reading is a Pool named in a bound's `targets` (F1). I would cut the hook-filter text, the retired-owner fallbacks and the restatements, and reduce ACC-056's table to deciding rules in a stated order. implicit.yaml is clear, but its two central observables rest on a record that does not exist, and whether an Agent's requests during a turn serve that turn, which decides who answers for them, is left to each Agent package (F5, F6). observation.yaml is the cleanest of the three and rightly layered; its only fault is that OBS-004's `then` says more than PLC-024 and PRV-010 allow (F7).

**What the reader read.** Tree: commit 2eb8f258 (2026-11-12), read-only; nothing was built or run, and nothing was changed.

Read in full, line by line: spec/conventions/access.yaml (1-1004), spec/conventions/implicit.yaml (1-94), spec/conventions/observation.yaml (1-82), docs/guides/full-review.md, AGENTS.md, spec/core/requirements.yaml.

spec/definitions.md: lines 1-261, 347-378 and 408-467 (including "Words the promises do not use") in full; lines 262-346 and 379-407 (pending actions, implementations, providers, secrets, configuration, packages) with each row cut at about 500 characters. spec/README.md: lines 24-97 (the three layers) and 281-495 (form of a promise, conformance, limits through "What access does not narrow") in full; lines 495-618 only searched.

Known-open records checked: docs/AUDIT.md lines 120-159, 174-246 and 289-311; docs/guides/cold-readers.md lines 39-81; IN_FORCE.md lines 100-146 in full and its Open and Spec pending lines by search; ruling 52's record, lines 1-60.

Cited promises opened in full: OPS-005, 007, 008, 009, 012, 013, 014, 022, 023, 024, 025, 029, 030, 032, 034; CON-004, 008, 018, 022, 028, 031; LIF-005, 010, 011, 017, 020, 021, 024, 026, 035, 040, 041 and part of LIF-013; IFC-012, 013, 018, 019, 020, 021, 028, 050; SEC-007 to SEC-011; PRV-004, PRV-014; PLC-023, PLC-024; POOL-004, POOL-015; FLEET-003 to FLEET-005; REV-014; REC-001, REC-002; CTR-002; DL17. In spec/core: `refusal.v1`, `condition.v1`, `lift.v1`, `target-reference.v1`, the `root` template, `interface.v1`, and the lists of operations and capabilities.

Not read: the rest of spec/core, spec/schemas, the other promise files beyond the promises listed, IFC-016, IFC-022, IFC-025 to IFC-027 and IFC-032 beyond searches, and the ledger beyond ruling 52's row. Vectors were only searched (conformance/conventions/access/bounds and current-tools). In the engine I read only engine/crates/core/src/access.rs lines 60-200, to see which reading the first build took; it is not used as authority.

Counts recomputed with /usr/bin/python3 and PyYAML: 519 promises (226 language, 245 convention, 48 interface); access.yaml 35, implicit.yaml 5, observation.yaml 5; ACC-056's table 59 rows.


## FR-001

**A Pool named in a bound's `targets`: what it covers, and whether a bound naming it in `pools` is within it**

L01/F1 · ambiguity · unsafe · verifier: confirmed, high · decides: coordinator · new

Same fault, or overlapping: FR-121.

**Promises.** ACC-029, ACC-030, ACC-006, ACC-012, ACC-053

**Where.** spec/conventions/access.yaml:521

**Quote.** "A new bound is within an old one when every Pool and every individual target it names is covered by the old one, as ACC-029 states, and every operation it names is one the old one names."

**Situation.** `bound.v1`'s `targets` takes `target-reference.v1`, which admits a Pool (spec/core/references.yaml:316-331). ACC-029 says what a Fleet in `targets` covers (access.yaml:509-510) and is silent on a Pool there. Reading 1: a Pool in `targets` is an individual target and covers that Pool alone. Reading 2: "Naming a Pool covers the Pool itself and every Resource that Pool owns" (access.yaml:480) holds whichever argument names it. Harm under reading 2: an author writes targets [Pool shared, Resource r1] with a Resource operation, meaning r1 alone, and the delegate may open Conduits to every Resource of `shared`. Harm under reading 1 with ACC-030's `then` as written: D holds bound {targets: [Pool shared], operations: [read]}; D asks to open for itself a Conduit to `access` with {pools: [shared], operations: [read]}; "every Pool it names is covered by the old one" is true, since the old bound covers the Pool itself, and the first observable sentence ("may name a Pool only if the old one names that Pool", line 535) is true too; the new bound now covers every Resource of `shared`. The same observable's invariant ("covers nothing that the bound of the actor that opened it does not cover", lines 536-537) forbids that, so `then` and observable disagree. The first build took reading 1 and a stricter rule the text does not state: new `pools` must be in old `pools` (engine/crates/core/src/access.rs:88-110). No vector names a Pool in `targets` (conformance/conventions/access/bounds).

**Counterargument.** The invariant sentence in ACC-030's observable lets a careful builder derive the safe rule, and an author who wants a Pool's Resources writes `pools`, so a Pool in `targets` is rare.

**Least change.** ACC-029 `then`, after "An individual target is covered when the bound names it.": "A Pool that `targets` names is an individual target: it covers that Pool alone and none of its Resources, as a Fleet does." ACC-030 `then`: "A new bound is within an old one when every Pool that its `pools` names is one that the old one's `pools` names, every individual target it names is covered by the old one, as ACC-029 states, and every operation it names is one the old one names." One vector for each sentence.

**Precedent.** AWS IAM: a statement on a bucket's ARN does not cover the objects under it, which have their own ARN pattern. Kubernetes RBAC: a rule on the `namespaces` resource grants nothing on the resources inside a namespace.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity whose fix adds no behavior, since it writes down the reading the first build already keeps. It changes what ACC-029 says, so it goes on the review list for the owner to strike.

**Verifier.** confirmed, high. Quote verbatim at spec/conventions/access.yaml:521-523. Nothing in spec/ decides a Pool in `targets`: `bound.v1` takes `target-reference.v1` for `targets` (spec/core/requirements.yaml:68-72), which admits `Pool` (spec/core/references.yaml:316-331); CON-028 (spec/promises/conduits.yaml:370-383) rejects nothing of the kind. Reading 2 has more literal support than the reviewer cites: access.yaml:480 "Naming a Pool covers the Pool itself and every Resource that Pool owns"; access.yaml:484-485 "whose bound names a Pool can open a Conduit to that Pool, and to a Resource instantiated in that Pool afterwards"; spec/README.md:462 "A bound that names a Pool covers every Resource that the Pool owns"; spec/definitions.md:186. Reading 1 rests on the three-way split "It names Pools, individual targets and operations" (access.yaml:479-480) and the Fleet sentence (509-510). The first build is reading 1 plus an unstated rule: engine/crates/core/src/access.rs:88-94 (`covers`: a `targets` item matches by ID only) and 99-101 (`subset`: every new `pools` item must be in old `pools`). ACC-030's `then` ends "This is the whole condition" (access.yaml:527), so under reading 1 the literal `then` admits old {targets:[Pool shared]} to new {pools:[shared]}; only the invariant at 535-537 forbids it. Not recorded: no hit in docs/AUDIT.md Known to be open (174-246), docs/guides/cold-readers.md, IN_FORCE.md, the first-build logs, the review list, G5.md or the ledger.

**Verifier's corrections.** One factual correction: the statement "No vector names a Pool in `targets`" holds only for conformance/conventions/access/bounds. conformance/conventions/inventory/e4/documents.json has three bounds that do (`no-lift`, `kept-other`, and `delegate-access`, which names Pool `store` in both `targets` and `pools`), used by e4 cases pool-bound-removal-lift, pool-bound-removal-reconcile and reconcile-access-opening-operations. In the one I read through (pool-bound-removal-lift) the Pool in `targets` covers a Conduit to the Pool itself, which both readings give; I did not find one that tests a Resource of a Pool named only in `targets`, and did not check all three step by step. This shows the case is written in practice, not rare. Unsafe mark stands, with this precision: the widening needs either reading 2 against an author who meant the Pool alone, or reading 1 with ACC-030's `then` taken as the whole condition and the invariant sentence of its observable not enforced. Class and who decides stand (coordinator, review list). The least change should also correct spec/README.md:462, spec/definitions.md:186, access.yaml:484-486 and the description at spec/core/requirements.yaml:53-54, which all say a bound that "names a Pool" covers its Resources without saying in which argument.


## FR-002

**ACC-056 requires the specific refusing rule at tier 1 and also that tier 1 not depend on the named object's existence, owner or state**

L01/F2 · contradiction · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-209.

**Promises.** ACC-056, ACC-054, SEC-008, ACC-024, ACC-029, ACC-039, ACC-034, IFC-050, OPS-023

**Where.** spec/conventions/access.yaml:919

**Quote.** "Nothing in tier 1 depends on whether an object exists, who owns it or its state."

**Situation.** The sentence before it gives every caller "the cause in the refusing rule's row" and the `then` opens with "whose ruleId is the refusing promise" (line 915); IFC-050 makes the ruleId observable (spec/interface/implementation.yaml:1349). Take an actor whose bounded Conduit to `access` lists `open.v1` and who may inspect nothing. It asks to open Conduit `n`. The refusing rule is ACC-054 where n's consumer is root (row at line 963), SEC-008 where n lists `get-secret.v1` (line 998), ACC-029 where the bound does not cover n's target (line 954); on a close, ACC-024 where n is a root default (line 952). ACC-039 and ACC-034 likewise tell a Resource that a Conduit in the chain it serves has a retired consumer or was closed. So the tier-1 ruleId and cause do vary with the object's existence, consumer and state. Builder A emits the specific rule (explicit, and discloses). Builder B emits ACC-001 to a caller without inspection authority (hides, and is terse). ACC-056 also states no order among refusing rules that apply together, for example a bound that does not cover and a consumer that is root, so two builders of kind A still differ; OPS-023 orders reasons, not rules.

**Counterargument.** The observable says the rows "describe requirements, not hidden facts about a named object's state" (lines 934-935): no cause text names the fact, and a holder of `open.v1` is a trusted delegate. But which row is chosen conveys the fact, and the same observable says a cause "never discloses an inferred owner, retirement state or chain beyond what the caller may learn".

**Least change.** Add to ACC-056's `then`: "Refusing rules are tried in this order: the caller's own authority (ACC-001, ACC-033, IFC-021, IFC-028); its bound (ACC-029, ACC-030); then a rule that reads an object the request names (ACC-024, ACC-054, ACC-057, SEC-008, LIF-034). The first that refuses is the refusing promise. Where it refuses only because of the consumer, the owner or the state of an object that the caller may not inspect under ACC-033, tier 1 gives ACC-001 and its cause, and the record keeps the refusing rule." The quoted sentence then holds as written.

**Precedent.** Amazon S3 answers 403 AccessDenied in place of 404 NoSuchKey to a caller without s3:ListBucket. RFC 9110 section 15.5.4 lets a server answer 404 in place of 403 to hide that a resource exists. OPS-029 already does this for a Pool-only holder.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction inside one promise. Ruling 52 points 1 and 2 already give the rule to apply: explicit unless it discloses what the caller may not learn. No feature is added.

**Verifier.** confirmed, medium. Quote verbatim at access.yaml:919-921. The same `then` says the ruleId "is the refusing promise" (915) and "It states the specific cause of that rule; the no-open-Conduit statement belongs to ACC-001" (918-919); IFC-050 makes it observable: "the ruleId, reason and refusal.v1 fields are observable" (spec/interface/implementation.yaml:1349). Rules whose `when` reads facts of an object the caller may not inspect have their own rows: ACC-024 (952), ACC-029 (954, via `pools` it depends on who owns the target), ACC-034 (958), ACC-039 (961), ACC-054 (963), SEC-008 (998; spec/promises/secrets.yaml:196-200). Clearest case: a Resource serving a chain gets ACC-039 where a chain consumer is retired and ACC-034 where a chain Conduit closed (access.yaml:792-798, 694-699), against "A cause never discloses an inferred owner, retirement state or chain beyond what the caller may learn" (936-937). No order among co-applying rules: OPS-023 orders the four reasons only (spec/conventions/operations.yaml:493-496); ACC-056's rows give partial hints only (988 "needs ACC-033 authority before existence is examined", 984). Ruling 52 point 1 and IN_FORCE.md:143 give tier 1 one generic statement; the per-rule cause came with order 038a. Not in any known-open record; G5.md:254 says ACC-056 is "not modelled".

**Verifier's corrections.** Narrow two points. (a) ACC-054 and SEC-008 read the named Conduit's consumer and operations, which are strictly neither its existence, its owner nor its lifecycle state; the literal breaches of the quoted sentence are ACC-029 through `pools` (owner of the target), ACC-039 and ACC-034 (state in the chain), ACC-024 (first-start or default provenance) and existence itself through OPS-029. (b) Builder B (ACC-001 for every caller without inspection authority) is not a sound reading, because access.yaml:918-919 reserves the no-open-Conduit statement to ACC-001; the real divergence between builders is the order among rules that refuse together, which overlaps F4. Who decides: stating an order is the coordinator's. The masking half of the least change alters the observable ruleId and fixes what a bounded delegate may learn, which is a disclosure choice under ruling 52 point 2; the no-behavior alternative is to reword the sentence to what the rows keep (no tier-1 field other than the refusing rule and its cause is computed from a resolved object). Put the choice between the two on the review list.


## FR-003

**A bounded holder of `open.v1` or `close.v1` learns from the reason alone whether any Conduit name exists**

L01/F3 · contradiction · verifier: weakened, medium · decides: coordinator · new

**Promises.** OPS-029, ACC-006, ACC-012, ACC-053, ACC-056, OPS-036, LIF-013

**Where.** spec/conventions/operations.yaml:702

**Quote.** "A bound is judged against what exists, and does not make such a request report refused: a request for `open.v1` that names no Conduit reports invalid whatever the bound of the actor's Conduit to `access`."

**Situation.** Steps. D, an Agent or a delegated Interface, holds a Conduit to `access` that lists `open.v1` under the bound {pools: [team-a]}. D asks to open `worker--payroll--t`; generated names join authored keys by SCT-013's fixed rules (spec/definitions.md:70), so names can be guessed. No such Conduit: invalid, by the quoted sentence. One exists outside the bound: refused, naming the bound (ACC-006, access.yaml:116-119). Repeated, the reason maps which consumer is connected to which target across the engine, which LIF-013's inventory and OPS-036's listing show only to whoever may inspect. `close.v1` behaves the same. Two texts decide the same kind of case the other way: ACC-053 for `lift.v1` ("refused to a bounded holder, which learns nothing", access.yaml:844-845) and OPS-029 itself for a Pool-only holder ("learns nothing of whether a name exists outside that Pool", operations.yaml:700-701). ACC-056's "the diagnostic is identical whether a name exists or not" (access.yaml:929-930) is then true of the diagnostic and false of the reason.

**Counterargument.** A holder of `open.v1` has to tell a mistyped name from a bound, and invalid is the more explicit answer under ruling 52 point 2. Conduit names are not secrets, and the sentence is deliberate.

**Least change.** Replace the quoted sentence in OPS-029 with: "A request for `open.v1` or `close.v1` that names no Conduit reports invalid where the actor's Conduit to `access` has no bound, and refused under ACC-029 where it has one, as ACC-053 states for `lift.v1`: a bounded holder learns nothing of a name outside its bound."

**Precedent.** ACC-053's own rule for a lift. Amazon S3's 403 in place of 404 for a caller without s3:ListBucket.

**Who decides.** coordinator. Rulings 37 and 46 point 4: two promises decide one kind of case in opposite ways, and the fix adds no feature. It changes a reported reason, so it is logged for the owner to strike; the owner may prefer the reverse, with a lift reporting invalid.

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/operations.yaml:702-705. The text decides the case and it is deliberate: open.v1 naming no Conduit is invalid whatever the bound; ACC-006 agrees (access.yaml:125-126) and ACC-012 (218-219). An existing Conduit outside the bound is refused naming the bound (access.yaml:116-119), so a bounded holder tells the two apart. The sibling rules hide existence: operations.yaml:699-701 "so that a caller with only a Pool's authority learns nothing of whether a name exists outside that Pool"; access.yaml:844-845 "refused to a bounded holder, which learns nothing". Names are guessable (spec/definitions.md:70, `worker--files--t`). The Pool-only rule was a coordinator decision the owner kept: docs/proposals/first-build/to-review-with-the-owner.md:14 "names are global, so the old text let a Pool's caller learn whether a name exists outside its Pool" (finding 2-08, docs/proposals/first-build/README.md:82). That decision did not consider a bounded holder of `access`; no record covers it. Ruling 52 point 1: "a target that does not exist gets the same answer where the caller has no authority over it" (docs/proposals/ruling-52/README.md:24).

**Verifier's corrections.** Class: not a contradiction. OPS-029's sentence governs open.v1 and close.v1 and ACC-053 governs lift.v1, so both hold together; ACC-056 applies only to a request that reports refused (access.yaml:912), so its "identical whether a name exists or not" is not breached, as the reviewer concedes. It is a defect: one rule discloses what the specification's own rule for the two like cases, and the accepted reason for finding 2-08, withhold. Note also that a tombstone ID is engine-given and unguessable while a Conduit name is guessable, so the hidden case is the harmless one. Not unsafe. Who decides: the fix reverses a deliberate sentence and changes a reported reason; finding 2-08 is the precedent for the coordinator doing so under ruling 46 point 4 with a review-list entry, and the owner may prefer invalid for explicitness (ruling 52 point 2).


## FR-004

**Which promise is "the refusing promise" is not determined for common refusals**

L01/F4 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-126, FR-158, FR-210.

**Promises.** ACC-056, ACC-001, ACC-011, ACC-006, ACC-033, IFC-012, IFC-013, IFC-050, OPS-009, OPS-032, OPS-034, CTR-013, REC-001

**Where.** spec/conventions/access.yaml:915

**Quote.** "Its result carries one error diagnostic, whose ruleId is the refusing promise and whose properties.refusal has the form refusal.v1."

**Situation.** One event, four candidate rules. An Agent calls the tool of a Conduit that was closed. IFC-012: "reports refused under ACC-001, whether the name was never presented or its Conduit has been closed" (spec/interface/implementation.yaml:316-319). IFC-013: "reports refused under ACC-011" (344-345), as OPS-009, OPS-032, OPS-034 and LIF-026 say (operations.yaml:242). ACC-056's table has rows for ACC-001, ACC-011, IFC-012 and IFC-013, each with a different cause. A second event: an actor with no Conduit to `access` asks to open one. ACC-001, ACC-006, ACC-033 row 10 and IFC-012 (no such tool is presented) all fit. The table gives rows to promises that define no refusal of their own (ACC-018, ACC-036, ACC-038, OPS-036, PND-007; ACC-023's row speaks of root, which ACC-023 is not about) and has none for CTR-013's refusal ("Only root may give that input; another caller giving it reports refused", spec/promises/contracts.yaml:319). IFC-050 makes the ruleId observable and REC-001 records it, so vectors will fix whichever the first build chose (it never emits ACC-011), and an independent builder cannot derive it from the text. A ruleId of ACC-011 would also tell the caller that the name once had a Conduit, which IFC-012 says tier 1 does not disclose.

**Counterargument.** The observable already says "Promises that restate a refusal reuse the deciding access rule's cause; the ruleId remains the deciding rule" (lines 935-936), so a builder can treat ACC-001 as the rule for any request through no open Conduit.

**Least change.** In ACC-056 keep a row only for a promise whose own `then` refuses, remove the rows of promises that restate (ACC-011, ACC-023, ACC-036, ACC-038, IFC-013 among them), add CTR-013, and state: "A request that no open Conduit permits, whatever became of an earlier one, is refused by ACC-001." Change "refused under ACC-011" to ACC-001 in IFC-013, OPS-009, OPS-032, OPS-034 and LIF-026. F2's order settles rules that apply together.

**Precedent.** SARIF 2.1.0, which `diagnostic.v1` reduces: a result has one `ruleId`, and each rule is defined once in the tool's rule list. RFC 9457 problem details: one `type` per occurrence.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity whose fix adds no behavior.

**Verifier.** confirmed, medium. Quote verbatim at access.yaml:915-916. Closed Conduit, two rules for one event: spec/interface/implementation.yaml:317-319 "reports refused under ACC-001, whether the name was never presented or its Conduit has been closed" and 331-332 "Tier 1 discloses no difference between a name never presented and a tool whose Conduit closed"; against implementation.yaml:344-345 "reports refused under ACC-011", and the same in access.yaml:689, operations.yaml:242, 796, 861 and lifecycle.yaml:564. ACC-011's own observable says such a request "is decided under ACC-001" (access.yaml:191). The table gives ACC-001, ACC-011, IFC-012 and IFC-013 four different causes (941, 946, 969, 970). Rows for promises that define no refusal: ACC-018 (949), ACC-036 (959), ACC-038 (960), OPS-036 (989), PND-007 (991); ACC-023's row speaks of root (951) while ACC-023 is about a non-root Interface (406-415). CTR-013 refuses with its own ruleId and has no row: spec/promises/contracts.yaml:319 "Only root may give that input; another caller giving it reports refused", 323-324 "Each diagnostic names CTR-013". The first build has no `ACC-011` string in any .rs file under engine/crates. Order 038a's record notes one earlier table miss (IMP-008) and nothing on this; no known-open record has it.

**Verifier's corrections.** Add one more instance the reviewer did not list: ACC-024's observable says a protected close "reports refused under ACC-012" (access.yaml:428), yet ACC-024 has its own row (952) and ACC-012's row also covers it (947 "protected Conduits cannot close"). Precision: for the closed-Conduit case the text does point one way (ACC-001: implementation.yaml:317-319 and 331-332, access.yaml:191 and 935-936), so the six "under ACC-011" citations are stray copies and the direction of the fix is determined; the cases with no deciding text are the co-applying rules (ACC-006 against ACC-029 for a bound, ACC-012 against ACC-024, ACC-001 against IFC-012 for an unknown tool spelling). Class and who decides stand.


## FR-005

**Whether a request an Agent makes during a turn serves that turn is left to the Agent's implementation**

L01/F5 · ambiguity · verifier: confirmed, medium · decides: owner · known-new-situation

**Promises.** ACC-016, ACC-017, ACC-034, ACC-039, IMP-008, IFC-020, IFC-021, REC-002

**Where.** spec/conventions/access.yaml:292

**Quote.** "An instance, while serving an invocation, requests an operation on another target."

**Situation.** Interface A holds a Conduit to Agent B that lists a marked `turn` operation and applies filter F. B holds an unfiltered Conduit to directory D. During the turn B's client calls D's read. Reading 1, from this `when`, the definition of onward request and IFC-020 ("An instance makes an onward request in order to serve an invocation it received"): the call serves the turn, must name it, carries A as caller and F, stops once A's Conduit is closed (ACC-034) or A is retired (ACC-039), and its record names the turn in `serving`. Reading 2, from IFC-021's observable ("An Interface or an Agent that names none makes the request on its own behalf and is its caller", spec/interface/implementation.yaml:585-586): B chooses. Naming nothing, the call is B's own, F is not carried, closing A's Conduit stops nothing before the turn ends, and no record links what B did to the turn or to A. The shipped Agent packages never name `serving` (no occurrence under packages/agent-codex or packages/agent-claude-code), so the first build is reading 2; a package that proxies the model's calls and stamps `serving` is reading 1. The two give different callers in records (preservation test 7) and different force to a filter on a Conduit to an Agent. ACC-017's observable ("An effect or content that they forbid to the caller is not produced or returned") does not say that it fails to hold for an Agent's own requests, and neither does "What access does not narrow".

**Counterargument.** An Agent is an actor that decides for itself (preservation test 1), IFC-021 states reading 2 in plain words, and a stock client cannot add `_meta` to each call. IN_FORCE already holds, spec pending for batch 2, that inspection shows what each turn-holder reaches one hop out.

**Least change.** State the limit in ACC-017's observable and in README "What access does not narrow": "A request that an Agent makes during a turn and that names no invocation in `io.github.shrav-k.conduit/serving` is its own: it carries neither the caller nor the filters of the turn, the close of the Conduit through which the turn was dispatched does not stop it, and its record names no invocation served." Put one question to the owner: should the engine record, beside such a request, the marked invocation then in progress on that Agent, with no effect on access.

**Precedent.** OAuth 2.0 Token Exchange (RFC 8693) keeps delegation, with its `act` claim naming the acting party, apart from impersonation. W3C Trace Context links a caused request to its cause without changing its authorization.

**Who decides.** owner. It settles who answers for an Agent's work during a turn, which is purpose and behavior; ruling 37 leaves to the owner what changes what Conduit is for. The coordinator may state the limit meanwhile under ruling 46 point 4.

**Verifier.** confirmed, medium. Quote verbatim at access.yaml:292 (ACC-017's `when`; ACC-016's is the same at 276). The engine's rule for each form is fixed: spec/interface/implementation.yaml:585-586 "An Interface or an Agent that names none makes the request on its own behalf and is its caller"; spec/conventions/implicit.yaml:89-90 allows a marked request "when it serves none and an actor makes it on its own behalf". What obliges an Agent's implementation is only IFC-020's `when` (implementation.yaml:556 "makes an onward request in order to serve an invocation it received"); no text in spec/ says whether a model's call during a turn serves the turn (no hit for requests during a turn in spec/). No `serving` occurs under packages/agent-codex or packages/agent-claude-code. The limits sections do not state it (spec/README.md:452-487 has no word on turns). Known part: spec/conventions/lifecycle.yaml:282-284 "through the target's model work the holder reaches whatever the target's own tools reach. It shows one hop only, and does not show what the target reaches through its own Conduits"; IN_FORCE.md:200 (ruling 61 B13, spec pending batch 2); IN_FORCE.md:178 (62 C12, reach beyond one hop, Open).

**Verifier's corrections.** Known-new-situation is the right mark: reach through a turn is recorded and ruled (61 B13); what is new is that the caller, the carried filters, the stop on close (ACC-034, ACC-039) and the record link all turn on whether the Agent's client stamps `serving`, which no promise decides. Class is sound only at the Implementation level (two package authors diverge); an engine builder does not diverge, so for the engine it is an unstated limit. Not unsafe: the request is still attributed to the Agent (preservation test 7), and a filter on a Conduit to an Agent needs the Agent's implementation to state `supports`, which neither shipped package was checked for. Owner decides, as stated.


## FR-006

**IMP-001 and IMP-002 are checked against a record that no promise requires, and the cases that matter are an Agent package's**

L01/F6 · defect · verifier: confirmed, medium · decides: coordinator · new

**Promises.** IMP-001, IMP-002, IMP-003, REC-001, TPL-017, PRV-010

**Where.** spec/conventions/implicit.yaml:28

**Quote.** "Every record of model work starting names such an invocation, its caller and its Conduit."

**Situation.** REC-001 lists what is recorded and ends "Nothing else is required to be recorded" (spec/conventions/records.yaml:31-32). A start of model work is not in the list and `record.v1` has no such event. The only record that can stand for it is the dispatch of a marked operation, so this observable and IMP-002's ("No record of model work starting follows any of those events", implicit.yaml:42) hold for every engine whatever an Agent's implementation does. The cases IMP-002 names that an engine cannot see are the implementation's: a client that resumes its session when the runtime provider restarts the run, one that starts a turn inside `setup-hook.v1`, one that wakes its model when an unmarked operation delivers a message. Each breaks IMP-002, and no record, observable or vector can show it; the vectors that cite IMP-002 check only that the engine dispatches no marked operation. No interface promise states the duty for an implementation, and IMP-002 is not among the Implementation-level examples (spec/README.md:322). TPL-017 reasons from the contract alone: "`setup-hook.v1` carries no mark of model work, so the hook starts none" (spec/promises/templates.yaml:289).

**Counterargument.** The Implementation level covers "every promise, whatever its layer, whose obligation falls on the implementation", so a package's author is bound already, and an engine cannot see inside a client by design.

**Least change.** IMP-001 observable: "The record of each dispatched invocation of an operation marked `startsModelWork: true` names its caller and its Conduit, under REC-002, and the engine dispatches such an operation only for a request." IMP-002 observable: "None of those events is followed by the dispatch of a marked operation that no request asked for. An Agent's implementation begins or continues a turn of its model only within a call for a marked operation, which its package's contract tests show." Add IMP-002 to the Implementation-level examples in README "Conformance".

**Precedent.** The Model Context Protocol treats a tool's annotations as hints that a client does not trust from an untrusted server, so the property is shown by testing the server. The provider-contract harness (experiment 022) is the existing way a package shows a promise that binds it.

**Who decides.** coordinator. Rulings 37 and 46 point 4: the observable cannot be checked as written, and the fix adds no behavior.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/implicit.yaml:28; IMP-002's at 42-43. REC-001 lists the recorded events and ends "Nothing else is required to be recorded" (spec/conventions/records.yaml:31-32); a start of model work is not among them (records.yaml:17-21) and `record.v1` has no such member (spec/core/results.yaml:175-206). "record of model work" occurs nowhere else in spec/. IMP-002's `when` names cases only an implementation can break: "an operation not marked as starting model work is invoked on an Agent, whatever it delivers to the Agent" (implicit.yaml:36-37). IMP-002 is not among the Implementation-level examples (spec/README.md:322). TPL-017 reasons from the contract: spec/promises/templates.yaml:289. Coverage: conformance/manifest.yaml:7467-7470 gives IMP-001 no vector ("Base Agent types offer no model-work operation and launch no wrapper"); IMP-002's five vectors are setup-base, setup-hook-succeeded, setup-hook-failed, setup-hook-unknown and s1-whole (7472-7479). G5.md:309-310 holds both only in a bounded engine model. No known-open record has the point.

**Verifier's corrections.** The duty already falls on an Agent's implementation through IMP-001's `then` and the Implementation level (spec/README.md:322 "Every promise, whatever its layer, whose obligation falls on the implementation and not on the engine"), so the defect is the observable, not a missing obligation. In the least change, the clause "which its package's contract tests show" adds a conformance duty on packages that no promise has now; keep the rest under rulings 37 and 46 point 4 and either drop that clause or log it as a ruling 71 point 4 small addition for the owner to strike. IMP-001's missing vector is itself a recorded coverage gap (the manifest's reason line), so only the undefined record and the untestable implementation cases are new.


## FR-007

**OBS-004 forbids anything acting on an observed difference, and three promises have something act**

L01/F7 · contradiction · verifier: weakened, low · decides: coordinator · new

**Promises.** OBS-004, PLC-024, PRV-010, PRV-026, LIF-011, OPS-014

**Where.** spec/conventions/observation.yaml:61

**Quote.** "Anything acts to remove the difference without an explicit request."

**Situation.** With no request: a runtime provider restarts a stopped run under its restart policy (PRV-010, cited in OBS-004's own observable); the engine runs a provider again after it restarts (PRV-026, also cited); and, not cited, the engine retires a node and every instance placed on it, closing their Conduits for good, when the machine provider reports the machine gone (PLC-024, spec/promises/placement.yaml:345-353: "No actor asks for it, and no request is made"). The observable narrows the obligation to "No setup, control, open or retire is dispatched because of it", which PLC-024 meets only because it "dispatches nothing". A reader of the convention, whose file head says a difference is "reported, never repaired", does not learn that one provider report ends every instance on a node, while OPS-014 says the runs may still be alive ("Declaring a node lost fences nothing"). G5 holds OBS-004 only in the narrowed form ("drift dispatches no setup, control, open or retirement").

**Counterargument.** Retiring what is gone records a loss and repairs nothing; PLC-024 is ruled (ruling 40 point 3b) and follows Kubernetes, and the observable is exact, so no builder is misled about what to build.

**Least change.** OBS-004 `then`: "The engine dispatches a setup, a control, an open or a retirement because of it." Add to its observable: "One report changes lifecycle state with no request: a machine reported gone, under PLC-024."

**Precedent.** Terraform reports drift in `plan` and changes nothing until `apply`. Kubernetes' cloud node controller deletes the Node of an instance the cloud reports gone, which PLC-024 follows.

**Who decides.** coordinator. Rulings 37 and 46 point 4: the `then` says more than the specification keeps; behavior is unchanged.

**Verifier.** weakened, low. Quote verbatim at spec/conventions/observation.yaml:61; head comment at 1-2 "a difference between them is reported, never repaired". The same promise's observable narrows the obligation and names two of the three cases itself: observation.yaml:64-68 "No setup, control, open or retire is dispatched because of it. A runtime provider that restarts a run does so under its own restart policy, under PRV-010, and the engine dispatches nothing because of it" and PRV-026. PRV-010 is a `may` (spec/promises/providers.yaml:326-333). PLC-024: spec/promises/placement.yaml:347-353 "the node and every instance placed on it are retired" and "No actor asks for it, and no request is made. It takes effect at once and dispatches nothing". OPS-014's sentence exists at spec/conventions/operations.yaml:365. G5.md:301 holds OBS-004 as "O4: drift dispatches no setup, control, open or retirement." Not in any known-open record (the tracker's PLC-024 item is its record field, A39).

**Verifier's corrections.** Class: defect, not contradiction. The `then` is wider than its own observable, which is the only place a builder or a vector reads the obligation, so no two builders diverge and nothing unsafe follows. PRV-010 is the one literal clash (a provider acting on a stopped run with no request), and the promise already carves it out. PLC-024 is weaker evidence than stated: it changes lifecycle state, leaves the declared part alone (OBS-001) and dispatches nothing, so whether it "removes the difference" is itself arguable; what would settle it is one sentence saying whether retirement on a `gone` report counts as acting on a difference. The useful part of the fix is the cross-reference from OBS-004 to PLC-024. Coordinator decides, as stated.


## FR-008

**Access promises decide three cases that their own cited promises make unreachable**

L01/F8 · needless-complexity · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-212, FR-259.

**Promises.** ACC-018, ACC-015, ACC-017, ACC-033, ACC-057, LIF-020, LIF-021, LIF-035, POOL-004, FLEET-003, IFC-018, IFC-019

**Where.** spec/conventions/access.yaml:342

**Quote.** "In this version no hook carries a filter: `set-up.v1`, `retire.v1` and `retire-with-resources.v1` are implemented by `engine`, which states no `supports`, under MAN-012, so no Conduit through which they are requested applies a filter, under CON-022"

**Situation.** (1) Hooks carrying filters. ACC-018's `then` has a hook carry its invocation's filters, and its observable ends that the list "is therefore an empty list". ACC-015's hook clause (lines 250-251), ACC-017, IFC-018, IFC-019 and four definition rows repeat a case that cannot occur. (2) A member of a retired owner. LIF-020 refuses to retire a Pool or Fleet that owns a member whose shutdown is not confirmed; LIF-021 dispatches the container's retirement only once every member's shutdown is confirmed; LIF-035 removes a member at that moment; POOL-004, FLEET-003 and POOL-017 reject registration into a retired owner. So no Resource or Agent exists whose owner is retired, yet ACC-033 rows 4, 4A, 6, 6A, 14, 14A and 17 and lines 653-660 decide for one ("and none where that Pool is retired", "Inspection of its remaining members"). Row 13's fallback, for the container's own retirement and the cascade, is reachable and stays. (3) ACC-057's `when` covers `register-replacement.v1` making an Agent (line 897); LIF-009 makes only a declared Conduit or a Pool entry's Resource. A builder implements each branch, and a vector can reach none without a hand-made snapshot.

**Counterargument.** The hook text is forward-looking: a later engine-implemented verb may support a filter. The retired-owner rows make ACC-033 total without leaning on LIF-020 and on DL17's atomic commit. Unreachable rows cost little.

**Least change.** ACC-018: keep "A hook is sent the filters of the invocation it belongs to, which in this version is an empty list" and delete the hook clause of ACC-015 and the hook sentences of ACC-017 and IFC-019. ACC-033: replace the retired-owner fallbacks of rows 4, 4A, 6, 6A, 14, 14A and 17 and lines 653-660 with "A retired Pool or Fleet owns nothing that exists (LIF-020, LIF-021); its own retirement, and the cascade's, are reconciled through `management`." ACC-057: drop "or `register-replacement.v1`".

**Precedent.** The specification's own rule 2 for a promise: stated as something that can be checked (spec/README.md:298).

**Who decides.** coordinator. Rulings 37 and 46 point 4: no state reaches the text, so removing it removes no feature. Logged for the owner to strike, who may prefer to keep the hook text for v1.

**Verifier.** confirmed, low. Quote verbatim at access.yaml:342-344; the observable ends "is therefore an empty list" (346-347). (2) A member cannot exist under a retired owner: spec/conventions/lifecycle.yaml:413-414 "A member that is registered or ready, or retired with shutdown pending or unknown, prevents retirement of its owning Pool or Fleet"; 423-424 "the container's retirement is dispatched only once every member's shutdown is confirmed"; 442 "A member whose shutdown is confirmed has been removed (LIF-035)"; a Pool or Fleet has no placement (spec/promises/placement.yaml:75-81), so PLC-017 and PLC-024 retire neither; registration into a retired owner is rejected (spec/promises/pools.yaml:60-61, fleets.yaml:48). Yet access.yaml:605-611, 625-630, 634-635 and 653-655 decide for such a member. (3) access.yaml:897 against lifecycle.yaml:149-150 "The nested instantiation is a declared Conduit or a Pool's entry", spec/core/operations.yaml:556-557, and FLEET-005's `root-fleet` list, which omits `register-replacement.v1` (spec/promises/fleets.yaml:82-83). No record or ruling on any of the three.

**Verifier's corrections.** Lines 653-660 are only partly unreachable. Reachable and to keep: "Every Conduit to a retired Pool or Fleet is closed under CON-008" (653), the reconciliation of the cascade and of the container's own retirement (655-657), and the refusal of a cascade or setup addressed to a retired owner (658-660). Unreachable: "its remaining members" (653-655) and the retired-owner fallbacks of rows 4, 4A, 6, 6A, 14, 14A and, by reference, 17. spec/promises/pools.yaml:64-66 ("the Pool owns the same Resources as before") presumes the same state. For (1) the text declares itself vacuous in this version, so the cost is reading, not building; removing forward-looking hook text may be the owner's preference, as the reviewer says. Class and who decides stand.


## FR-009

**Observables restate other promises and carry commentary, and the copies have begun to differ**

L01/F9 · needless-complexity · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-032.

**Promises.** ACC-030, ACC-008, ACC-011, ACC-015, ACC-018, ACC-019, ACC-039, IMP-007, OPS-009, OPS-024, OPS-034, CON-008

**Where.** spec/conventions/access.yaml:552

**Quote.** "A narrower provisioning, such as registering instances of one approved template and no other, goes through a broker: a Resource with a trusted operation whose own contract and configuration allow only that"

**Situation.** Recounted at this commit: access.yaml's 35 promises hold 2,345 words of `then` and 6,401 of `observable`, and each cites eight other promises on average. The broker passage (lines 543-561) is in the observable of a promise about bounds on `access`; no test can observe it, and it is written again in README "What access does not narrow" and in two definition rows. The test for dispatching a filtered repeat again is written out in 11 promises (ACC-008 and IMP-007 here; OPS-009 and OPS-024 own it). What a Conduit permits while its consumer is retired is told in 11 (ACC-003, ACC-006, ACC-011, ACC-039, CON-008, LIF-024 among them). The copies already differ: IFC-012 refuses a closed Conduit's call under ACC-001 and IFC-013 under ACC-011 (F4); ACC-015 lists "`management`, of `access` or of a Pool" (lines 267-268) and ACC-018 "`set-up.v1`, `retire.v1` and `retire-with-resources.v1`" (lines 342-343), both without the Fleet and `retire-with-agents.v1` that batch 1 added. A builder must reconcile every copy, and each ruling must edit all of them.

**Counterargument.** Each promise can be read alone, which cold readers asked for; the restatements closed findings of earlier rounds and mostly agree.

**Least change.** Add a rule 6 to README "The form of a promise": "An observable states what is checked for this promise's own obligation. It cites another promise by its ID and does not restate it." Then cut ACC-030 lines 543-561 (README holds them), the repeat rules from ACC-008 and IMP-007 (leave "OPS-009, OPS-024 and OPS-034 decide a repeat"), and the retirement account from ACC-011.

**Precedent.** IETF practice: a rule is stated once and other sections cite it by number, as RFC 9110 defines each status code once.

**Who decides.** coordinator. Rulings 37 and 46 point 4: no behavior changes. Logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at access.yaml:552-555. Recount at this commit with PyYAML: access.yaml has 35 promises, 2,345 words of `then`, 6,401 of `observable`, 7.9 other promises cited per promise; ACC-056's observable is 1,156 words with 59 cause rows; ACC-033 has 22 rows. The filtered-repeat test (no `effect`, not a control, not `startsModelWork`) is written out in 11 promises by a loose match (ACC-008, IMP-007, OPS-007, OPS-009, OPS-024, IFC-007, IFC-010, IFC-016, IFC-033, IFC-045, IFC-046). The broker passage (access.yaml:543-561) recurs at spec/README.md:476-487 and spec/definitions.md:147 and 187. Copies that differ: IFC-012 against IFC-013 (see F4); access.yaml:266-268 "made of `management`, of `access` or of a Pool" (no Fleet); 342-343 lists `set-up.v1`, `retire.v1` and `retire-with-resources.v1` (no `retire-with-agents.v1`, which LIF-021 names at lifecycle.yaml:454). The form of a promise has five rules and none on restatement (spec/README.md:295-301). No known-open record or ruling covers it; ruling 44 point 2 asks only that v0 "says plainly" the two limits (docs/proposals/ruling-44/README.md:26-31), which the README section does.

**Verifier's corrections.** Counts: the retired-consumer account appears in about 15 promises by my match, not 11, and IFC-033 in the list of 11 defines `readOnlyHint` in the same terms and is not a repeat rule; neither changes the finding. Cutting ACC-030:543-561 is compatible with ruling 44 only while spec/README.md:452-487 keeps the two limits; say so in the log entry. Class and who decides stand.


## Further points the verifier noticed

Not verified by a second reader.

- **ACC-017 and ACC-018 leave out the sign-in hooks that batch 1 added** (spec/conventions/access.yaml:297). ACC-017's `then` carries filters "for a hook, from the invocation of `set-up.v1` or `retire.v1` that it belongs to" (297-298) and ACC-018's `when` is "a setup hook or a retire hook, under TPL-017 or TPL-018" (321). spec/definitions.md:178 defines the chain with "`set-up.v1`, `retire.v1` or `sign-in.v1`" and cites ACC-017 for it, and PND-011 says the sign-in hooks run "for that invocation's caller and filters, through no Conduit" (spec/promises/pending.yaml:219-220). So no access promise says that a request made by the implementation of `sign-in-hook.v1`, `account-hook.v1` or `sign-out-hook.v1` is an onward request naming the hook's invocation, or that ACC-015 applies to such a hook. Gap by drift, low, vacuous in this version for the same reason as F8 (1); it belongs with F9's list of copies that differ. Coordinator (rulings 37 and 46 point 4).

## The verifier's notes

Checked at 2eb8f258d8c6 (2026-11-12) in the review worktree, read-only; nothing was built or run beyond grep, sed, read-only git and /usr/bin/python3 for counts.

Read in full: spec/conventions/access.yaml, implicit.yaml, observation.yaml, docs/guides/full-review.md, the review list (to-review-with-the-owner.md) and ruling 52's record. Read in part, at the cited places: spec/definitions.md, spec/README.md (form of a promise, Conformance, Limits), spec/core/requirements.yaml, references.yaml, results.yaml and operations.yaml, the cited promises in operations.yaml, lifecycle.yaml, records.yaml, placement.yaml, providers.yaml, secrets.yaml, pending.yaml, fleets.yaml, pools.yaml, contracts.yaml, conduits.yaml and interface/implementation.yaml, docs/AUDIT.md (who decides, waiting, known to be open, purpose), cold-readers.md (known to be open), IN_FORCE.md by search, G5.md by search, rulings 44, 59 and 61 by search, engine/crates/core/src/access.rs:40-140, and the e4 inventory vectors.

Result: 7 confirmed, 2 weakened (F3 and F7, both on class), none known, none refuted. F1 is the only unsafe finding and the only high. One factual error found in the reviewer's text: F1's claim that no vector names a Pool in `targets` is true only of the bounds directory; three bounds in conformance/conventions/inventory/e4/documents.json do.

Not done: I did not walk every step of the three e4 cases that use those bounds, so "no vector distinguishes the two readings of F1" is not established, only that the one case read through uses the Pool as the target itself. I did not check whether either shipped Agent package states `supports` (bears on F5's filter example). The first-build claims were checked by search of engine/crates only, not by running it.
