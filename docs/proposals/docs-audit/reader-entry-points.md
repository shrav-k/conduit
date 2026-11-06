# Audit of the documents outside `spec/`: the entry points and the tracker

Cold reader's report of 2026-11-05, kept verbatim apart from an absolute path. It reads the
repository from `README.md` as an outsider would. Findings are not rulings; what was done about
each is in the [audit record](README.md).

```text
All paths are under the repository root. Nothing was edited. Coverage: every listed file was read in full except docs/implementation/ARCHITECTURE.md (lines 1-330 read, the rest by headings and searches), docs/implementation/ORGANIZATION.md (lines 1-60 and 252-412) and the Experiment 001 and 002 READMEs (first 40-50 lines each). All four checkers were run and pass: check_docs (101 Markdown files, 1223 links, 0 errors), set_names --check (0 errors), check_schemas (129 documents valid, 40 invalid rejected, 0 errors), check_doc_migration (0 errors). Counts of spec/ are as of this read; spec/ is in flux.

A. STALE OR FALSE STATEMENTS

A1. docs/AUDIT.md:103 — "355 promises across 21 files: 189 in the language layer … The definitions are not yet brought into line, and the JSON Schemas and the core package are not written."
- The repository has spec/core/, spec/schemas/ and spec/promises/manifest.yaml, and ruling 37 (AUDIT.md:95) accepted them.
- Current count: 369 promises in 22 files (203 language, 137 convention, 29 interface), 13 implementation-defined items.
- Fix: reword to "…core package, JSON Schemas and manifest format drafted and accepted (ruling 37); its changes are being applied. No vectors." Update the count once the spec editors finish, or replace the numbers with "counts: spec/README.md".

A2. docs/AUDIT.md:243 (A60) "drafted and proposed … Awaiting the owner's review"; :242 (A59) "In progress: the definitions, and the core package, schemas and manifest format for the owner's review"; docs/README.md:48 "Proposed; not accepted."
- All three are contradicted by ruling 37.
- Fix: A60 to "accepted: ruling 37, with five changes; being applied per APPLY.md"; A59 to end "…355 promises. Next: round 2."; docs/README row to "Accepted with five changes (ruling 37)."

A3. docs/AUDIT.md:231 (A32) "in progress: form ruled; `spec/` not yet created"; :232 (A33) "open: format ruled".
- spec/ exists and every area is written as promises.
- Fix: A32 "in progress: spec/ exists and spec/README.md lists its parts; G1 not met while accepted behavior remains in docs/protocol". A33 "in progress: every area written as promises; no vectors; word counts not done".

A4. "Only five names are settled / vocabulary comes last" survives in eight places, against rulings 16 to 18 (AUDIT.md:74-76) and AUDIT.md:46:
- AGENTS.md:41-42 "Only the five family names are settled … the vocabulary pass comes last" (also contradicts AGENTS.md:48 "written in the ruled vocabulary")
- README.md:16 "Vocabulary comes last."
- docs/README.md:7 "vocabulary comes last"
- CONTRIBUTING.md:49 "Only the five family names are settled."
- docs/REVIEW.md:18, :30, :33-34, :39
- docs/OVERVIEW.md:184 "The naming pass now comes last"
- docs/VOCABULARY.md:8-11 and :118 "No replacement vocabulary has been accepted in this map yet."
- Fix for AGENTS: "The vocabulary is ruled (rulings 17, 18, 27 to 29). The terms in force are in spec/definitions.md; write new text in them. Older documents keep the earlier words; do not mass-rename them." README: "The vocabulary is ruled and used in the specification." The others are covered by the banners in E3.

A5. docs/README.md:36-37 "Awaits restatement as promises in the protocol reference"; AGENTS.md:16-18 "accepted rulings not yet restated in the protocol topics".
- Placement and instantiation are promises in spec/promises/placement.yaml and instantiation.yaml and will never be restated in docs/protocol.
- Fix rows: "Accepted; written as promises in spec/promises/…. Where the record differs, the specification governs." Replace AGENTS "Read first" item 3 with spec/README.md, then spec/definitions.md.

A6. docs/LANGUAGE_DEFECTS.md cites promise IDs that no longer exist in spec/, in 14 rows:
- :115 FST-004; :116 PKG-015; :121 PKG-002; :123 PKG-009 to 011; :124 PKG-008; :135 INS-015; :136 ACC-025
- :207, :233, :234 CON-010; :208 ACC-004; :209 ACC-007; :219 ACC-013; :228 ACC-020, ACC-021
- docs/AUDIT.md:105, :259, :260 give ranges that include PLC-010, PLC-014 and INS-015, which are also gone.
- Fix: repoint each to its replacement (as LANGUAGE_DEFECTS.md:79 says was done for ACC-010), or add above the table: "IDs are as first written; the comment at the head of each spec file says where a removed ID went." Do this after the spec editors finish.

A7. Action and question statuses in docs/AUDIT.md that the repository contradicts:
- :235 A42 "86 of 167" against 87 at :101 and LANGUAGE_DEFECTS.md:33. Fix: 87.
- :247 A41 "open". L1 is ruled (LANGUAGE_DEFECTS.md:87). Fix: "ruled (ruling 20): OPS-012 to OPS-014; vector pending".
- :250 A16 and :374 Q5 "open". A Conduit is pairwise (ruling 17) and its closing is ruled (rulings 19, 28, 29). Fix: "ruled".
- :253 A35 "Specify the owner-only declare-lost operation … open". PLC-017 exists (spec/definitions.md:236). Fix: "ruled: PLC-017, reserved to the root Interface; vector pending".
- :376 Q6 "open: A13". The outcome model is ruled (ruling 20). Fix: "done: ruling 20".
- :379 Q10 "in progress: A47". A47 is done at :230. Fix: "answered: spec/ (ruling 17); conversion of docs/protocol tracked by A33".
- :254 A43 "open". Fix: "schemas drafted in spec/schemas (A60); Rust loader open".
- :251 A25 and :377 Q8 "open". Ruling 29 settled the lifecycle. Owner to confirm: "ruled in part (ruling 29); the rest waits for the Agent type contract (L69, L76 to L81)".
- :238, :240, :241, :248 give ranges (NAM to 017, IMP to 007, LIF to 023, CTR to 006, IFC to 017) that later passes extended to NAM-026, IMP-009, LIF-026, CTR-008, IFC-029. Fix: one line above the table: "Promise ranges in status notes are as first written; current files and counts are in spec/README.md."

A8. docs/AUDIT.md:104 "Vocabulary | Ruled. Not yet applied to the text."; docs/README.md:49 "Accepted; not yet applied to the text."
- The specification is written in it.
- Fix: "Applied in spec/. Not applied to docs/protocol, the overview and other documents written before it; the old-to-new record (A29) is not written."

A9. Future tense for finished work:
- docs/AUDIT.md:42-44 "they are re-triaged by layer after the layer decisions"
- docs/AUDIT.md:201-202 "After the layer decisions are ruled, each defect is assigned"
- docs/LANGUAGE_DEFECTS.md:15-17 "Each defect below is to be re-triaged", directly above "Triage by layer — Done"
- Fix: past tense.

A10. docs/AUDIT.md:351 "7. Prune and vocabulary."; :393 D7 "The pass comes last and may merge concepts. | done".
- Both contradict ruling 16 and P3 (:222).
- Fix: step 7 "Prune."; D7 "Superseded by ruling 16: decided now and applied with the rework."

A11. docs/AUDIT.md:511 "all 19 enforced formats".
- The profile enforces five (spec/schema-profile.md:63).
- The rest of that paragraph (:511-518) names dropped concepts ("runtime-target source", "registration batches as graphs", "bounded issuance"). Fix: mark it historical (D6).

A12. Superseded words in the live parts of docs/AUDIT.md (not the ruling log):
- "plugin interface" :38, :248, :275, :322. INDEPENDENT_BUILDS.md:22 already says "implementation interface". Fix: replace.
- "normative surface" :33, :37, :231, :249. Fix: "the specification" (decision V8).
- "owner-only" :253. Fix: "reserved to the root Interface".
- "grant" as an object: :116, :131 "**The grant is an object.**", preservation tests 2, 5 and 8 (:152-158), :163, :179-180, :186; "provision" :133.
- These last are standing tests, so do not rewrite them silently. Fix: one line under :147: "Since rulings 27 to 29 a grant is the access an open Conduit carries, and 'provision' is setting up; read the tests with those words."
- Owner decision: whether :131 becomes "The relationship is an object: each Conduit can be inspected, narrowed and closed independently of both ends."

A13. docs/AUDIT.md:181 "Type contracts | How each type behaves: an Agent's lifecycle".
- Ruling 29 makes the lifecycle one shared convention; a type defines conditions.
- Fix: "an Agent's conditions (running, paused, idle)". At :195 (LD3 "Six conventions") append "(seven since ruling 17)".

A14. docs/implementation/ENGINE_BUILD_BRIEF.md:5-6 "The paths it names (`spec/`, `conformance/`) are proposed and do not exist yet."
- spec/ exists. Fix: "`conformance/` does not exist yet."
- Same in INDEPENDENT_BUILDS.md:6 and :59 ("Proposed top-level directories"): mark spec/ as existing.

A15. docs/REVIEW.md:41-42 "alternatives for all 37 topics and walkthroughs"; docs/README.md:20 "[research and walkthroughs](research/vocabulary/README.md)".
- CANDIDATES.md and WALKTHROUGHS.md were removed under ruling 36.
- Fix: "precedents and the reviewer's simplification pass".

A16. AGENTS.md:34-35 and docs/README.md:26 "the language and the shared conventions … A draft being filled area by area".
- spec/README.md:11 has three layers, and :3-4 says every area is written.
- Fix: "the language, the shared conventions and the implementation interface, as numbered promises. A draft: every area is written; no conformance vector exists yet."

A17. docs/README.md:8-9 "Completed rulings remain accepted, with their full records in the archive".
- The 37 rulings are logged in AUDIT.md with records under docs/proposals/.
- Fix: "Rulings are logged in the plan audit, with a record for each batch under proposals/."

A18. AGENTS.md:6-7 "Engine builders work only from the `build-base` branch".
- No such branch exists locally or on origin.
- Fix: append "(not yet published; the builds have not started)".

A19. docs/AUDIT.md:257 (A2) "done: S1 and S2 passing every method".
- It reads as though they pass; nothing has run.
- Fix: "done: named as S1 and S2 (rulings 4, 9); not yet run".

A20. docs/AUDIT.md:103 "a follow-up pass applied 25 more rulings (33 to 35)".
- Fix: "the 26 follow-up decisions (ruling 33) and ruling 35".

B. CONTRADICTIONS

B1. Where accepted behavior is changed.
- CONTRIBUTING.md:10 "Change accepted behavior in docs/protocol/"; docs/README.md:71-74 "Update an accepted rule once in its protocol topic … without restoring a parallel decision ledger"; docs/REVIEW.md:130.
- Against AGENTS.md:46-48 "An owner ruling: the ruling log … Accepted behavior: a promise in `spec/`".
- CONTRIBUTING.md never mentions spec/.
- Fix both to: "Accepted behavior is a promise in spec/, in the terms of spec/definitions.md. A change needs an owner ruling, recorded in the ruling log of docs/AUDIT.md with its record under docs/proposals/. docs/protocol/ is no longer edited."

B2. What binds.
- spec/README.md:4-5 and AGENTS.md:36-37 say behavior "not yet converted" still lives in docs/protocol.
- docs/README.md:28 calls protocol/ "One active home for detailed accepted contracts"; README.md:24 calls it "Accepted behavioral contracts".
- AUDIT.md:103 says every area is ruled and written. Nothing says which protocol text is still in force, and docs/protocol uses the superseded model (`form`, grant, join).
- Owner decision. Option (a): spec/ alone binds; docs/protocol is background and source material for future type contracts; a gap is a defect in spec/. Option (b): keep it binding and list the sections still in force.
- I recommend (a): the build base holds only spec/. Then a one-line banner on docs/protocol/README.md and corrected rows in README.md, docs/README.md and AGENTS.md.

B3. How many layers.
- docs/AUDIT.md:177-182 lists four (two inside the specification); docs/proposals/packages-and-start/README.md:11 says "the specification's two layers"; spec/README.md:11 says "It has three layers".
- Fix: add under AUDIT.md:187: "The specification holds three layers: language, convention and interface (the implementation interface of LD3 and LD4). Type contracts and the runtime profile stay outside it."

B4. What Conduit is.
- README.md:3 and AGENTS.md:3 "protocol and configuration language"; docs/OVERVIEW.md:1-3 "extensible protocol".
- Against AGENTS.md:22-23, AUDIT.md:30 and spec/README.md:9 "a configuration language, not a behavioral specification".
- Fix: use spec/README.md:9's sentence in README.md:3 and AGENTS.md:3. Owner decision: whether "protocol" stays in any description.

B5. docs/OVERVIEW.md teaches the superseded model with no banner, and README.md:23 sends readers there second:
- :17 "template form and a concrete instance form"; :106 "envelope uses `apiVersion`, `kind`, `form`, `metadata` and `spec`" (ruling 18: no `form`; an instance has `from` and no `spec`)
- :40 "Policy / grant" and :116-121 (ruling 27: no grant object)
- :130-131 "Ending the last logical participation ends a Conduit" (ruling 17: pairwise)
- :25, :66 "provisions" (ruling 29); :38 "Plugin"
- Smallest fix: banner "Predates rulings 13 to 37. Where it differs, spec/ governs: there is no `form` field, no grant object, a Conduit has one consumer and one target, and the lifecycle is registered, ready, retired."
- Better (owner decision): rewrite it from spec/README.md and spec/definitions.md.

B6. The build brief uses words the specification bans or redefines.
- docs/implementation/ENGINE_BUILD_BRIEF.md:16, :58 and INDEPENDENT_BUILDS.md:20 say "declarations"; spec/definitions.md:265 says "declaration: Not used." Fix: "documents".
- ENGINE_BUILD_BRIEF.md:29 and INDEPENDENT_BUILDS.md:63, :89 call spec/README.md "the manifest"; manifest is now a defined term (spec/definitions.md:200). Fix: "contents list".
- INDEPENDENT_BUILDS.md:91 (G3) "each family and form". Fix: "each kind".

B7. docs/implementation/ENGINE_BUILD_BRIEF.md:112-121 lists no runtime profile.
- spec/README.md:100 requires one for the Engine level.
- Owner decision: add "PROFILE.md: your choice for every item of spec/implementation-defined.yaml", or state which conformance level builders target.
- Also INDEPENDENT_BUILDS.md:49 tags `spec-v1, spec-v2` while the specification's own version is `v0` (spec/names.yaml:17). Fix: rename the tags (for example `base-1`) or add a sentence that they are revisions of version v0.

B8. docs/implementation/INDEPENDENT_BUILDS.md:48 says the build base holds spec/, conformance/, the brief and "Nothing else".
- spec/ files link into ../docs/ and tell the reader to run tools/check_schemas.py and tools/set_names.py (spec/names.yaml:3, spec/core/README.md:33, spec/schemas/README.md:135).
- Owner decision: ship tools/ on the build base, or strip those references when publishing it.

B9. Three different check lists.
- AGENTS.md:64-68 has check_docs, set_names --check and git diff --check.
- CONTRIBUTING.md:62-65 has check_docs and git diff --check.
- AUDIT.md:243 says spec/core and spec/schemas are "checked by `tools/check_schemas.py`", which neither lists and which needs PyYAML and jsonschema.
- Fix: the same four-line block in both, with "# needs PyYAML and jsonschema" beside check_schemas.

B10. tools/set_names.py does less than the documents say.
- AUDIT.md:92 says it "verifies that every file under `spec/` agrees with it".
- The check (set_names.py:102-130) refuses retired names, checks the prefix on four of the eleven `_meta` keys (:108), and requires each name to occur once. A wrong new value that is not in `retired` passes.
- Fix: reword to "refuses a retired name and a wrong prefix on the keys it recognizes", or extend the tool.
- AGENTS.md:71-72 "run `python3 tools/set_names.py` with the new values, which rewrites every occurrence". Without `--write` it only previews (set_names.py:14, :173). Fix: add `--write`.

B11. Two queues each called the only one.
- docs/REVIEW.md:3 "the only active review entry point"; docs/README.md:31 "The short active work queue"; CONTRIBUTING.md:40 "Use one current review queue"; docs/implementation/README.md:10.
- Against AUDIT.md as the tracker that "governs".
- Fix: see C3 and E3.

B12. docs/LANGUAGE_DEFECTS.md:63-64 defines closed as "the language states the rule and a vector covers it".
- Three rows are closed with no vector (:231, :232, :249), and AUDIT.md:101 says "None has a vector".
- Fix: add "or the question no longer arises, with the reason".

C. NAVIGATION

C1. README.md:20-29 — the "Start here" table has no row for spec/. A cold reader never learns from the front page that a specification exists.
- Fix: first row "[Specification](spec/README.md) | What binds an engine: numbered promises, definitions, schemas and the core package. Draft."

C2. Every reading order skips the specification: docs/README.md:3-4 and :15-18; CONTRIBUTING.md:4-6; AGENTS.md:10-18.
- Fix: put spec/README.md then spec/definitions.md at the head of each.

C3. Competing starting points.
- README.md:22 "Plan audit: Read this first"; docs/README.md:3 "Start with the overview"; CONTRIBUTING.md:4 "Start with the plan audit"; docs/REVIEW.md:3.
- Fix: one order everywhere: README, spec/README.md, spec/definitions.md, docs/AUDIT.md (state and rulings), then the rest as background.

C4. No route from the vocabulary entry points to the glossary in force.
- README.md:28 offers the vocabulary map as "accepted meanings"; docs/VOCABULARY.md never links spec/definitions.md, which has three inbound links in the whole repository.
- Fix: README row "[Definitions](spec/definitions.md) | The terms in force"; demote the map with the banner in E3.

C5. docs/README.md index.
- Rows :26, :36, :37, :48, :49 are wrong (A2, A5, A8, A16).
- No row for tools/ or AGENTS.md.
- The AUDIT row (:32) omits the ruling log.
- ":42 Fifth and last batch" and ":46 The last batch" are each followed by later batches.
- Proposal rows are not in ruling order (vocabulary and normative-surface, rulings 17-18, come last).
- Fix: correct the rows, order them by ruling number, drop "last".

C6. Link text that misdescribes its target (AUDIT.md:497-498 already notes the class; not fixed):
- experiments/README.md:10 "[specification](../docs/OVERVIEW.md)"
- experiments/002-runtime-plugins/README.md:12, same label
- docs/implementation/README.md:27 "[REVIEW](../archive/reviews/IMPLEMENTATION.md)"
- docs/implementation/ARCHITECTURE.md:14 "[DESIGN](../protocol/README.md)"
- Fix: "overview", "closed implementation review", "protocol topics".

C7. AGENTS.md has no inbound link, so a human starting at README.md never sees the do-not-edit rules.
- Fix: one README line: "Rules for agents and contributors, including what not to edit: AGENTS.md."

C8. docs/AUDIT.md:236-246 — the P0 table lists the five fix batches as 1, 5, …, 4, 3, 2 (A51, A54, …, A53, A52, A13). Fix: put them in order.

D. docs/AUDIT.md

D1. Usable as the one tracker?
- As a ruling log, yes. Batch sizes in rulings 19 to 24, 27 to 33 and 37 match their action items and records (37 decisions, 26 follow-ups, 14 convention decisions, 26 consolidation findings).
- As a status tracker, not today: "Where things stand" and about fifteen statuses are behind the log (A1 to A3, A7).

D2. Superseded rulings carry no forward pointer:
- 2 (replaced by 16; only 16 says so)
- 10 "each build the whole engine" (narrowed by 14 and LD4)
- 15, 17, 19 "grant", "issue" (replaced by 27 to 29)
- 31 "contracts are documents of four kinds" (five since follow-up F21 under 33), "twelve entries" (13 since consolidation pass 1), `conduit.dev` (replaced by 33)
- Fix, without touching rows: one sentence under :55 listing these amendments.
- Owner decision: whether ruling 31's "a domain … must happen before the engine builds" still stands after ruling 33.

D3. Accepted against log.
- The only acceptance the tracker understates is ruling 37 (A2).
- Nothing in scope is stated as accepted without a ruling.
- Two rows overstate: :389 D3 "done" while its action A23 is open, and :393 D7 (A10).

D4. Identifiers (:11-14).
- The prefix list omits NS, V, RD/RR, OD/OR, DD/DR, GD/SD/GR/SR, PD/PR, NA to NF, DL, F and X.
- "G1" is both readiness condition (:231) and re-audit decision (:243).
- "PL-n for promises" is stale: the promises are PLC-nnn.
- Fix: complete the list, write "re-audit decision G1", change PL-n to PLC.

D5. Statuses. :18 lists five; the tables also use "ruled:", "round 1 done:", "drafted and proposed" and "answered by". Fix: add `ruled` (decided and written as promises; vectors pending) and normalize the rest.

D6. Sections now historical:
- Defect closure plan (:199-213), complete
- Findings Q and D tables (:364-394), settled except Q3, Q7, D5, D6, D8
- Multi-system options (:430-459), record of ruling 5
- Research and design gaps (:463-480); Known inconsistencies (:482-507); Deferrable scope (:509-518)
- Reference repositories (:520-545)
- Smallest fix: a banner under each: "Historical: recorded at the audit, before rulings 13 to 37. Not maintained."
- Owner option: move them to docs/archive/. Only #multi-system-options and #defect-closure-plan have inbound links (one each).
- Keep the standing probes table (:396-411) live; it is still open work.

D7. Order. The ruling log (:53-95), 37 dense rows of undefined identifiers, comes before anything that explains them. Fix: move "Where things stand" and "Purpose" above it.

D8. Two homes for defects, two convergence tests.
- AUDIT.md:23-24 sends defects to the register, but the re-audit's roughly 170 findings against spec/ live in docs/proposals/reaudit/. The register's sweep table (LANGUAGE_DEFECTS.md:69-81) stops at ruling 31.
- G7 (INDEPENDENT_BUILDS.md:95) counts clean sweeps of the register; A59 (:242) counts clean re-audit rounds.
- Fix: a line in the register: "Covers the first sweep, of docs/protocol. Findings against spec/ are in the re-audit record."
- Owner decision: whether G7 is judged on A59's two clean rounds.

D9. :102 "None of G1 to G11 is met." Plausible, but G9 is arguably met in draft (IFC-001 to IFC-029). Owner call; a one-line state per condition would settle it.

E. FIRST TEN MINUTES

E1. README.md:3-6 — the opening sentence is a list of jargon and gives no reason for the project.
- Fix: lead with spec/README.md:9 ("a configuration language for the relationships between independent agents and the things they share") and the three sentences at AUDIT.md:113-116.

E2. README.md:13-18 — "normative surface", "promise register", "conformance vectors" and "reference model" are used undefined, and the paragraph does not say the specification exists.
- Fix: "The specification is in spec/: about 370 numbered promises, drafted and audited once; none is yet tested by a conformance vector. There is no reference model and no engine."

E3. Banners contradicted by the text under them:
- docs/VOCABULARY.md:3-6 against :8-11 and :118. Fix banner: "This map lists the working terms from before the vocabulary ruling. The terms in force are in spec/definitions.md; the record is proposals/vocabulary/README.md."
- docs/REVIEW.md:8, :13-19 against :28-63. Fix banner: "Superseded as a work queue by the plan audit. Sections below it are kept for the links into them; their statements that vocabulary comes last, that five names are settled and that schemas are a later pass no longer hold."
- docs/implementation/README.md:3-6 against :10; ARCHITECTURE.md:3-6 against :8; ORGANIZATION.md:3-6 against :11. Fix: add to each banner "Not binding. Uses the words from before the vocabulary ruling. The tracker is the plan audit, not REVIEW."

E4. Unexplained on first use: "MCP" (README.md:9), "Codex" (AUDIT.md:36). Fix: expand once in README.md.

E5. Instructions that cannot be followed as written:
- AGENTS.md:6-8, the `build-base` branch (A18)
- AGENTS.md:71-72, set_names without `--write` (B10)
- AGENTS.md:58-59 "sibling directories `../agentwatch` and `../pi-reference`" exist only on the owner's machine. Fix: add "if present".
- CONTRIBUTING.md:60 "From a checkout with Python 3 available": check_schemas also needs two packages (B9).

E6. docs/LANGUAGE_DEFECTS.md — statuses read "implementation-defined: DL1" and DL is defined nowhere in the file; defect text uses handler, plugin, grant, provision, host and join.
- Fix: one line under "Status values": "DLn is an item of spec/implementation-defined.yaml. Defect text keeps the words of the text it was found in."

F. HYGIENE

F1. Clean: no personal absolute paths, session identifiers, TODO or FIXME markers, or temporary files, tracked or untracked. .gitignore covers target/ and .DS_Store.

F2. Uncommitted work the log already cites: docs/AUDIT.md is modified and docs/proposals/core-and-schemas/APPLY.md is untracked, yet ruling 37 links it. Commit both before the cold read.

F3. docs/AUDIT.md:530 names a proprietary repository (`jxp-software/agentwatch`) and says its links fail for outsiders; :533-545 lists nine removed clones, one "cited nowhere". Owner decision: keep, or move to docs/research/README.md or the archive.

F4. docs/AUDIT.md:501-502 is a note about the branch `implementation/rust-runtime-plugins`, which still exists locally and on origin. Owner decision: delete the branch and the note, or keep both.

F5. Patch-note prose: docs/VOCABULARY.md:3-6 ("Superseded in part on the same day … Earlier: …"); docs/REVIEW.md:8 (two unrelated sentences fused). The banners in E3 replace both.

F6. tools/check_schemas.py:2 "Check the proposed JSON Schemas". After ruling 37, drop "proposed".

F7. For the editors of the in-flux record: docs/proposals/core-and-schemas/README.md:226 "The `python3` on this machine's path is Python 3.13" is machine-specific.

TOP TEN FOR A COLD READER

What a cold read from README.md gives today:
- (1) What and why: partly. The what is jargon; the why is only in AUDIT.md#purpose.
- (2) What binds: no. README never mentions spec/ and presents docs/protocol as the accepted contracts.
- (3) State: partly. "Where things stand" exists but is behind ruling 37.
- (4) Reading order: no. Four documents give different starts and none includes spec/.
- (5) Checks: partly. Three lists, none complete; all four checks pass.
- (6) Do-not-touch: only for a reader who finds AGENTS.md.

1. README.md: add the spec/ row first, rewrite the opening and status paragraphs (C1, E1, E2), remove "Vocabulary comes last", link AGENTS.md.
2. Decide and state what binds (B2), and say it identically in README.md, AGENTS.md, docs/README.md and CONTRIBUTING.md.
3. AGENTS.md: replace the "five names" bullet (A4), make "Read first" spec/README.md, definitions.md, then AUDIT.md (A5, C2), fix the spec description (A16), add check_schemas and `--write` (B9, B10).
4. docs/AUDIT.md: bring "Where things stand", A32, A33, A59, A60 and the statuses in A7 up to ruling 37.
5. docs/OVERVIEW.md: banner now; rewrite from spec/ when time allows (B5).
6. CONTRIBUTING.md and docs/README.md "Maintaining the design": accepted behavior is a promise in spec/, changed by a logged ruling (B1); one check list (B9).
7. docs/README.md: fix the five wrong rows, order proposals by ruling, put spec/ in every reading path (C5, C2).
8. docs/VOCABULARY.md, docs/REVIEW.md and the three implementation documents: the banners in E3.
9. docs/AUDIT.md structure: status and purpose above the ruling log, banners on the historical sections, amendment note under the log, complete prefix list (D2, D4, D6, D7).
10. docs/LANGUAGE_DEFECTS.md: repoint the 14 rows with dead promise IDs, define DL, say it covers the first sweep only (A6, E6, D8); and in the build brief replace "declarations" and "manifest" (B6).

Not ready today. Links and checks are clean and hygiene is good, and spec/ and the ruling log are coherent with each other. What blocks a cold read is the front door: README.md, CONTRIBUTING.md and docs/README.md never route a reader to spec/; they present docs/protocol and docs/OVERVIEW.md, which teach the superseded model, as the accepted reference. AGENTS.md says in one section that only five names are settled and in the next that promises are written in the ruled vocabulary. The tracker's status tables say the schemas and core package are unwritten and unaccepted while its own ruling 37 accepts them. One decision is the owner's: whether docs/protocol still binds anything (B2). With that made, items 1 to 8 are about a day of edits to entry documents, none of which touches spec/.
```
