# Audit of the documents outside `spec/`: the older and supporting text

Cold reader's report of 2026-11-05, kept verbatim. It checks the protocol topics, the proposal
records, the research, the archive and the old experiments against the specification. Findings
are not rulings; what was done about each is in the [audit record](README.md).

```text
AUDIT OF THE OLDER AND SUPPORTING TEXT AGAINST spec/ (read-only; nothing edited)

Method: spec/README.md, definitions.md, names.yaml and every promise (id, when, then) were read first. The seven protocol topics were read in full by three parallel readers. I then re-read about 45 of their cited lines, and every one matched. All records, banners and fixtures I read myself. `python3 tools/check_docs.py` passes today (101 files, 1223 local links, 0 errors).

====================================================================
A. docs/protocol/ AGAINST spec/
====================================================================

A1. The claim "accepted behavior not yet converted lives in docs/protocol/" is true of about 20 sentences and unsafe as a description of the directory.
- Where it is made: spec/README.md:4-5, AGENTS.md:36-37, docs/README.md:28.
- By lines, across the seven topics (3,428 lines): about 40% is restated by spec/, 18% flatly contradicts it, 30% is type-contract or runtime-profile material, and 12% is rationale. The real unconverted items are listed under "Gaps in the specification".
- The contradictions sit under a banner that calls them "accepted working contracts".
- The accepted decision NS3 already says what should happen (docs/proposals/normative-surface/README.md:84-85): "When a topic is converted, its file under `docs/protocol/` is replaced by a pointer." spec/README.md:56 says every area has been written, so NS3 is due.
- Fix: change the three claims to "Nothing under docs/protocol/ binds. The few accepted rules not yet in spec/ are listed in <one list>", and create that list from the Gaps section below.

A2. The shared banner on all seven topic files (lines 3-13) is wrong.
- Lines 3-6: "The owner's rulings on placement and instantiation amend this topic and are not yet restated here." Rulings 16 to 37 all amend them, and placement and instantiation are restated as PLC-* and INS-*.
- Line 6: "tracks its conversion into numbered promises". The conversion is done.
- Lines 8 and 13: "Scope: accepted working contracts", "Part of the accepted design contracts".
- Fix (minimum): replace with "Superseded. Predates rulings 16 to 37 of docs/AUDIT.md. spec/ governs; where this file differs it is wrong. It is kept as source material for type contracts and the runtime profile."

A3. docs/protocol/README.md (118 lines): none of it is current.
- :9 "The active protocol reference lives here."
- :15 "a separate future PR will settle the vocabulary" (ruling 16 decided it).
- :20 presents Experiment 001 as "The YAML walkthrough proposes concrete syntax".
- :63-64 "V0 Interfaces provide unrestricted local administration" (contradicts ACC-023).
- :83 "Do not duplicate these rules into another editable specification" (spec/ now exists).
- :107 "C2 accepts Resource dependency grants".
- Fix: rewrite as a pointer page of about 20 lines: spec/ governs; the not-yet-converted list; where type-contract source material is.
- It has 51 inbound links from 37 files, so the path should stay.

A4. MODEL.md (618 lines): about 40% superseded, 15% contradicting, 30% runtime-profile or type-contract, 3% real gaps. Recommendation: archive.
- :70 "Grant | A particular issued authorization ... with its own participation lifetime" (definitions: grant not used; ACC-003, ACC-011).
- :116-117 "every template and instance selects one versioned type contract" for all five families (TPL-002: a Conduit has no type).
- :138-143 "runtime drivers and control handlers refer to a dependency alias plus a public export name" (NAM-002: "no shorter form and no alias"; TPL-003).
- :246-249 re-export "under its own public export name" (MAN-008 observable: "A re-export never gives the export another name").
- :266-267 exact-version imports (NAM-015, NAM-024).
- :470-474 "Interfaces ... are the user's unrestricted local administration surface" (ACC-023, ACC-029, ACC-030).
- :40-48 "Only the five main family names ... are settled ... A separate future PR will review terms".
- Its two most-linked sections (authoring concepts, 13 links; package scope, 8) are the ones the naming and consolidation rulings replaced.
- Inbound: 64 links from 22 files, including docs/VOCABULARY.md 17 and the archived AUTHORING review 12.

A5. CONFIGURATION.md (766 lines): about 50% superseded, 20% contradicting, 20% runtime-profile or tooling. Recommendation: archive.
- :38 "a top-level `form` beside it" (INS-001: "There is no other marker").
- :183 "V0 selects all 19 standard formats" (schema-profile.md: five).
- :147 `multipleOf`, and :263 conditionals "remain available" (both outside the profile).
- :394 "no backward-compatibility promise between releases" (NAM-024, NAM-025).
- :447 "Direct creation from complete instance configuration" (every instance has `from`).
- :457 "A rename uses the normal checked configuration update" (OBJ-011).
- :498 "Configuration submissions explicitly select create or update intent" (OBJ-002: a submission only adds; SUB-007).
- :580-582 "preserve successful writes when another entry fails; there is no atomic batch guarantee" (SUB-003).
- :591-592 authored batch order (SUB-001).
- :709-711 a missing schema rejects a template write (PKG-006 registers and reports).
- :742-745 conformance per declared capability (spec/README: two levels, for an engine).
- Stale "remains open" at :56, :83, :88-89, :301, :449.
- Inbound: 57 links from 16 files. CONTRIBUTING.md:36 and docs/implementation/README.md:56 cite #release-compatibility as a rule.

A6. LIFECYCLE.md (780 lines): about 50% superseded, 15% contradicting, 20% not in spec (half runtime profile by design). Recommendation: archive.
- :63-64 "Direct creation from a complete definition remains possible".
- :71-76 templates "explicitly select other supported controls ... backed by handlers" (TPL-010, CTR-006).
- :180-182 parallel changes are "plugin-defined" (REV-012, DL9).
- :188-189 "Control contracts may explicitly permit safe unconditional operations" (REV-011).
- :322-326 registration batches as ordered graphs (SUB-001, SUB-003).
- :357-358 "graph-input source selects the complete declared input payload, without a separate input-name selector" (GRA-020).
- :498-521 the binding key `reference` (GRA-019, GRA-020 use a map with `source`).
- :651-699 startup as a plugin operation given a "runtime target" (GRA-017, TPL-006, TPL-007).
- :756-757 "leaves retirement incomplete and the instance registered" (LIF-005).
- :618 "provision unselected Pool entries" (POOL-002, POOL-013).
- Inbound: 49 links from 18 files. experiments/004-stateright-evaluation/README.md:16 cites #template-defined-configuration-changes as its source rule; repoint it to REV-002, REV-011 and REV-012.

A7. CONDUITS.md (561 lines): about 45% superseded, 25% contradicting, 20% not in spec (mostly messaging and provider type-contract material). Recommendation: archive, keeping :516-561 and :441-488 as type-contract notes.
- :46 "The last-consumer rule governs when a flow ends" (CON-001, CON-008).
- :210-211 "The composite is itself a Conduit template/instance" (CON-003).
- :237-248 "Join an existing instance" (pairwise, ruling 17).
- :326-382 the whole grant model, for example :348 "ordinary managed access requires a recorded grant on the selected Conduit participation" (ACC-003, ACC-006).
- :25-27 a Conduit template's "own permitted configuration changes" (CON-016).
- :419 "Autonomous maintenance ... uses the Resource's own authority" (IFC-021: a Resource is not an actor).
- :441-444 "Conduit does not require its own contract definition for every Linear/MCP tool" (IFC-012, PKG-020).
- :505-506 "Automatically retry only when repetition is known safe" (IMP-007, OPS-012). Only the next sentence carries the L1 defect note.
- Inbound: 28 links from 16 files. experiments/004 README:15 cites #outcomes-and-recovery; repoint it to OPS-008 to OPS-013.

A8. POOLS.md (218 lines): about 20% superseded, 25% contradicting, 35% type-contract. Recommendation: reduce to "Shared access and authentication" (:141-217), which keeps the anchor that 9 files link to.
- :22-23 "These entries describe what the Pool can provision; listing or adding a template does not create/attach a Resource instance" (POOL-002).
- :57-62 "An entry is a reusable provisioning choice, not a singleton ... two tasks can provision separate worktrees from one entry" (POOL-003).
- :74-75 and :88-89 entries do not deploy with the Pool (POOL-013, TPL-012).
- :106-107 adding entries to a live Pool (INS-013).

A9. RESOURCES.md (264 lines): about 13% superseded, 12% contradicting, 55% type-contract. Recommendation: keep with the A2 banner, and strike :28-51.
- :28 "Resource specs use these fields in both template and instance forms".
- :32 "`spec.type` | A type-contract reference with separate `group`, `name` and `version` fields" (TPL-002, NAM-015).
- :36-51 "four core groups ... accepted".
- :229 "A manager may grant only rights it currently holds" (ACC-006).

A10. AGENT.md (221 lines): about 10% superseded, 8% contradicting, 80% Agent type-contract or reference-engine plan. Recommendation: keep, retitled as input to the Agent type contract.
- AUDIT action A25 (Agent lifecycle) is still open, so "accepted" overstates it.
- :24-26 "direct creation itself is accepted".
- :184 "closed Agents can be explicitly reopened" (LIF-006).
- :164 "unrestricted v0 authority".
- :74-75 "The MCP section below" and :219 "the Resource contracts below" point at sections that are not in this file.

A11. Decision for the owner on the topic files.
- (a) Banner only (A2) plus the pointer README (A3). Breaks nothing and can be done today.
- (b) Carry out NS3: archive MODEL, CONFIGURATION, LIFECYCLE and CONDUITS; reduce POOLS; keep AGENT and RESOURCES as type-contract input. About 200 inbound links need repointing, among them roughly 40 anchors in docs/VOCABULARY.md and links from docs/research/ and docs/archive/, which AGENTS.md says not to edit. Ruling 36 set the precedent of pinned-commit links.
- (c) Delete outright and rely on git history.
- Recommendation: (a) now, before the cold read, and (b) as the next step. I do not recommend (c) while the type contracts are unwritten.

====================================================================
B. docs/proposals/ RECORDS
====================================================================

B1. Only 3 of 15 records say that the specification governs: instantiation:10, relationships:9, vocabulary:8. The rest read as current rule text.
- Fix: one standard line under each status: "Record of a ruling. spec/ governs where they differ. Amended by: <rulings>."
- Needed in: placement, declarations, operations, graphs-and-schemas, packages-and-start, naming, consolidation, conventions, schemas-and-interface, normative-surface.

B2. packages-and-start is the most out of date.
- :1 title "accepted in part"; :3-5 and :21 "PD1 is open". Rulings 25 and 27 settled it, and docs/README.md:42 repeats "Accepted except how a package's exports are named".
- :29 PD5 names the level "Host" (spec: "Engine").
- :38 PR1 "A template's dependencies name a package and an exact version" (NAM-006: the manifest does).
- :41 PR4 "A type's default names its package and version" (NAM-005).
- :43 PR6 "location capabilities".
- :52-63 cites seven promises that no longer exist: PKG-002, 008, 009, 010, 011, 015 and FST-004.

B3. placement has no amendment note.
- :20-22 "PLC-001 to PLC-014" (PLC-014 was folded into CTR-004; the file now runs to PLC-019).
- :52 "References use kind, form and name".
- :71-73 "Enrolling a new system is provisioning that Resource".
- :118-124 "location capabilities ... an execution location publishes the capabilities it offers" (now a schema over stored facts: CTR-005, PLC-007, re-audit G5).
- :135-136 "fails preparation; an optional one is reported" (PLC-013: unavailable, checked at open and on each request).
- :187, :193, :218, :230, :236 write `inputs:` on instances with no note.
- :205, :208 `conduit:reference#/$defs/location` (now `github.com/shrav-k/conduit/core#...-reference.v1`).
- :277-281 "Still to specify" is done.

B4. declarations has no amendment note.
- :26-27 DD3 and DD4 are stated in terms of provisioning entries; ruling 29 removed the verb.
- :45 DR10 "Nothing else is a document" (SUB-005 now admits contracts and the manifest).
- :46 DR11 "A reference from one Pool entry to another is the other entry's name" (POOL-005: `{entry: name}`, and "a bare name ... never means an entry").
- :59 cites POOL-008, which was removed.

B5. operations has no amendment note.
- :38 OR4 "A request carrying an identity the target has already seen is never dispatched again" (ruling 32, decision Q1: a repeat is the new attempt; OPS-012, OPS-024).
- :24 OD2 "the implementation's contract declares that it deduplicates" (now the operation's contract, and never a plain implementation).
- :43 OR9 "uncertain" (spec: unknown).

B6. schemas-and-interface: the example at :61-101 contradicts final shapes with no note.
- `apiVersion: conduit.dev/v0` and `conduit.dev/...` on 13 lines. The only note is the table row at :179.
- :81 `controls: set-read-limit: {schema: ...}` (TPL-010: keyed by the control's qualified reference, with an `inputs` map).
- :98 `conduit.dev/core#resource-reference` (the core export is `resource-reference.v1`).
- :46 D11 "four kinds" (five, with Schema).
- :131-144 twelve entries (thirteen), and DL9 "how many run at once" (removed by F10).
- :20 X1 still recommends keeping `conduit.dev`.
- Fix: a note above the example naming rulings 32 and 33, or regenerate it from spec/schemas/examples/valid/project.

B7. naming.
- :5 "NAM-001 to NAM-017" (now up to 026).
- :124-133 the manifest example has no `apiVersion` (MAN-003) and uses `source:` at :132 (the key is `fetch`: NAM-012, definitions "source").
- :129, :147, :151 `conduit.dev/files`.
- :142 `inputs:` on an instance.
- :82, :94, :105 cite PKG-002 and PKG-008, which were folded into NAM-006 and NAM-011.
- The headings "What it would look like" and "Rules that would follow" still read as a proposal.

B8. relationships has a note, but it does not cover two things.
- :115-123 the example puts `consumer`, `target` and `operations` under `inputs:` (OBJ-013: `consumer` and `target` are top-level, the rest go under `arguments`).
- :72-99 cites nine promises that no longer exist: ACC-004, 007, 010, 013, 020, 021, 025 and CON-009, 010.
- Fix: extend the note, and add "IDs as first written; see each file's header for where a promise went".

B9. consolidation.
- :49 "The core package's address is `conduit.dev/core`" (ruling 33).
- :56 "Entries may refer to each other in a cycle" (POOL-011 rejects this; reversed by D9 of schemas-and-interface:44).
- :59-61 "Still unwritten" was decided by ruling 30.
- :39 "covers exactly the Resources it owns" (ACC-029: the Pool itself too).
- Eight dead promise IDs.
- Fix: one note above "Choices made while writing the rulings".

B10. normative-surface never links to spec/ and differs from it.
- :22, :44, :80 are still worded "Proposed:".
- :31 "The six behaviors" (seven, by the same ruling 17).
- :32 "Plugin interface".
- :60-72 seven writing rules (spec/README.md has five, plus the `layer` field of ruling 20).
- :72 "Every promise names at least one vector".
- Fix: a line "The form as it now stands is spec/README.md#the-form-of-a-promise".

B11. vocabulary.
- :1 the title is "Final vocabulary — proposal" although it is accepted.
- :4 "not yet applied to the text" (it is applied in spec/; docs/README.md:49 repeats the claim).
- :8-11 the amended note lists six changed words as if complete. Rows presented as current also differ: :62 "selection ... by package alias and export name" (NAM-002), :92 provision, :108 grant "issued and revoked", :113 owner "Also the person", :131 observation "source" (now origin).
- Fix: retitle, and end the note with "spec/definitions.md is the current vocabulary; this table is the record".

B12. instantiation.
- :5-7 "they will be restated as numbered promises ... when its form is ruled" (they are INS-001 to INS-023).
- :65 `conduit:reference#/$defs/resource`.
- :100-106 "Still to specify" is done, except the Experiment 001 rewrite, which will not happen.
- The `inputs:` at :79 is covered by the note at :10.

B13. graphs-and-schemas.
- :41 SR3 rejects a `default` "inside ... `items`" (SCH-008 now allows it on a property of the object `items` gives, by F23).
- :62 cites GRA-013, which was folded into GRA-019.

B14. core-and-schemas (in flux; confirm when the editors finish).
- README.md:1, :3 "proposed, not accepted", and docs/README.md:48 "Proposed; not accepted".
- docs/AUDIT.md:95 (ruling 37) and APPLY.md:3 say accepted with five changes.

B15. reaudit folder: correct as a record.
- Its eight logs and reader reports (186 KB) are clearly marked verbatim.
- https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#L90-L91 and the logs write the keys as `conduit.dev/...`; that is fine as history.

====================================================================
C. RESEARCH, ARCHIVE, AUTHORING, EXPERIMENTS
====================================================================

C1. docs/research/README.md.
- :3-5 the banner says only that "the current queue and the first proof layout are historical". It does not say research is not binding or predates spec/.
- :37 "Research supports the [specification](../OVERVIEW.md) and [design contracts](../protocol/README.md)" names the wrong document as the specification.
- :12-15 and the VOCABULARY row still advertise "candidate language for all 37 ... topics and worked examples", which were removed under ruling 36.
- Fix: rewrite the note to "Nothing under docs/research/ binds. Every packet predates the specification in spec/. Where a packet says accepted, current, SPEC.md or DESIGN.md, it means the documents as they stood before 2026-11-05." Also fix :37.

C2. Research packets whose top banner is wrong: they present the old documents as the current design.
- CONDUITS.md:3, ESTABLISHMENT.md:3, PI_CONNECTORS.md:3, PI_RUNTIME.md:3, RESOURCES.md:3: "Current-design reading note: [SPEC.md](../OVERVIEW.md) and [DESIGN.md](../protocol/...) define ...".
- CONFIGURATION.md:4 "Current contracts live in SPEC.md and DESIGN.md".
- CONSTRUCTION.md:3-6 "accepted contracts live in DESIGN.md" and "concrete YAML packet" (Experiment 001).
- RUST_RUNTIME.md:3-4 "the accepted [specification](../OVERVIEW.md)".

C3. Research packets with no non-binding or predates line at the top:
- AUTHORIZATION.md (asserts "recorded grants ... issuer-independent grant lifetime" as selected)
- CLOUD_AUTHORITY.md
- DISTRIBUTED_RUNTIME.md
- ECOSYSTEM_ORGANIZATION.md
- OBSERVABILITY.md
- RUNTIME_ESTABLISHMENT.md ("The owner subsequently accepted approach B"; replaced by decision D4, runtime as a start/stop pair)
- language-protocol-foundations/README.md, FORMATS.md, GRAPH_AUTHORING_REVIEW.md, SOURCES.md, WALKTHROUGHS.md
- vocabulary/PRECEDENTS.md:5 ("Candidate words remain unaccepted", "accepted vocabulary map") and vocabulary/SOURCES.md

Correct today: naming/README.md, vocabulary/README.md, vocabulary/SIMPLIFICATION.md.

Fix for C2 and C3: one identical line under each title. AGENTS.md:58 forbids editing research content, so this needs the owner's permission; notes under a title are already the practice (AGENTS.md:40).

C4. docs/archive/README.md.
- :20-21 "current semantics live in [protocol](../protocol/README.md)" is wrong.
- :6-8 "accepted refinements live in the current design".
- Fix: point to spec/, and add "Nothing here binds." This also needs permission (AGENTS.md:58).
- Links into the archive as if current: docs/README.md:8-9 "Completed rulings remain accepted, with their full records in the archive"; docs/protocol/README.md:104-109 (C1 to C3 "accepts ..."); authoring-syntax/AUTHORIZATION.md:9-10. Each protocol topic's banner line 11 is acceptable ("historical review links").

C5. docs/authoring/README.md (17 KB).
- AGENTS.md does not list it as not binding, and the root README.md table offers it as "Author with the protocol".
- :3-6 the banner is partly wrong: "the naming discussion ... now comes last" (ruling 16).
- Body contradictions: :35-36 "all 19 ... formats"; :54-65 non-atomic create/update batches; :77-78 "alias plus public export name"; :95 the binding key `reference`; :119-120 "Merely listing an entry provisions nothing"; :149 exact-version imports.
- Fix: archive it, or replace the body with a pointer to spec/schemas/examples/valid/. It has 9 inbound links from 7 files.

C6. docs/proposals/authoring-syntax/ (317 KB).
- Only README.md has a banner, and it carries the same wrong clause at :5-6. README:25 calls the protocol topics the "Canonical home".
- The four large files open with claims of acceptance and no direction note:
  - AUTHORIZATION.md:3-6 "accepted semantic model ... Policy and Grant are essential concepts"
  - PACKAGE_WALKTHROUGH.md:3-5 "alias-plus-export references are accepted"
  - AUTHORING.md:3
  - CONTRACTS.md:3
- They have 58 inbound links, so a reader lands on them directly.
- Fix: the standard not-binding line on each, or move the folder to docs/archive/.

C7. Links to removed files.
- No local link is broken.
- Nine pinned-commit links to the removed vocabulary CANDIDATES.md and WALKTHROUGHS.md are intentional (ruling 36): research/vocabulary/README, SOURCES, SIMPLIFICATION, PRECEDENTS; proposals/vocabulary:15; docs/VOCABULARY.md.
- Stale mentions without a link: docs/README.md:20 "research and walkthroughs"; docs/research/README.md:12-15.
- The labels "SPEC.md" and "DESIGN.md" in eight research files name files that no longer exist (the link targets do).

C8. Experiments 001 to 003.
- experiments/README.md:10-11: "The [specification](../docs/OVERVIEW.md) and [design contracts](../docs/protocol/README.md) remain authoritative". Wrong; point to spec/.
- 001 README.md:3-7 has a correct "Superseded" banner. But:
  - :11-12 "coherent proposal against the accepted design".
  - CONTRACTS.md has no banner, and :10 says "the common kind/form/metadata/spec envelope are accepted".
  - templates.yaml:1 and instances.yaml:1 say only "Illustrative schema draft".
  - Every shape in the YAML is retired: `form:`, `plugins:`, `{group, name, version}`, `nodes`, `required: true`, `reference: {from: ...}`, `conduit.local/v0alpha1`.
- 001 is presented as current in:
  - docs/VOCABULARY.md, 16 links, as the example for each term (:22-77)
  - docs/protocol/README.md:20
  - research: CONFIGURATION.md:62 "current YAML packet", RESOURCES.md:409 and :510, CONDUITS.md:840, PI_CONNECTORS.md:572, CONSTRUCTION.md:5
- 002 and 003 have correct banners. 002 is linked by MODEL.md:310 and docs/authoring:181.
- Should they be removed now?
  - 001: yes. Its role as the worked syntax example is filled by spec/schemas/examples/valid/, and ruling 9 says old experiments go as replacements land. Removal breaks 27 links from 15 files (VOCABULARY 16, research 9, archive 3, and others), so repoint them to a pinned commit as ruling 36 did. If it is not removed now, add the banner to CONTRACTS.md and both YAML files, and a pointer to spec/schemas/examples.
  - 002: keep until A23 maps its cases to vectors.
  - 003: keep; its results are cited evidence, and the Rust loader (A43) does not exist.
  - This is an owner decision.

====================================================================
D. DUPLICATION AND DEAD WEIGHT
====================================================================

D1. docs/authoring/README.md:23-65 and :80-115 restate CONFIGURATION.md and LIFECYCLE.md paragraph by paragraph. Both copies are superseded. Archive it (see C5).

D2. docs/protocol/README.md:27-60 (four "accepted direction" paragraphs) duplicates MODEL.md's authoring concepts and the root README. It goes with A3.

D3. placement record :252-268 (PL-1 to PL-10) is, in its own words at :21-22, "the earlier wording of the same rules" as PLC-*. Keep it as a record, and add that the PLC wording governs.

D4. Largest files that no longer serve a cold reader:
- authoring-syntax/AUTHORING.md, 138 KB
- authoring-syntax/CONTRACTS.md, 106 KB
- experiments/002 README, 57 KB
- authoring-syntax/AUTHORIZATION.md, 36 KB
- authoring-syntax/PACKAGE_WALKTHROUGH.md, 33 KB
- the seven protocol topics, 261 KB
- reaudit readers and logs, 186 KB (these are labelled records; fine to keep)

Candidates for docs/archive/: the authoring-syntax folder (breaks 76 inbound links in 20 or more files), docs/authoring/, and the four protocol topics of A11.

D5. Nothing in scope is unlinked. core-and-schemas/APPLY.md is linked only from docs/AUDIT.md:95; link it from the record's README once the editors finish.

D6. docs/research/ is about 800 KB in 30 files. It is evidence and is to be kept; only its banners need work.

====================================================================
E. RETIRED VOCABULARY
====================================================================

Counts are case-insensitive whole words. "issue" also counts issue trackers. "host" is legitimate where it means a location's machine.

| File | grant | issue | revoke | provision | plugin | driver | handler | runtime target | declaration | alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| protocol/MODEL | 28 | 9 | 1 | 1 | 45 | 3 | 4 | 1 | 7 | 5 |
| protocol/CONDUITS | 43 | 25 | 4 | 5 | 25 | 0 | 0 | 0 | 14 | 0 |
| protocol/LIFECYCLE | 7 | 0 | 0 | 2 | 41 | 2 | 16 | 9 | 15 | 0 |
| protocol/CONFIGURATION | 5 | 0 | 0 | 2 | 19 | 0 | 2 | 0 | 7 | 2 |
| protocol/POOLS | 8 | 2 | 2 | 20 | 2 | 0 | 0 | 0 | 3 | 1 |
| protocol/RESOURCES | 19 | 7 | 0 | 1 | 8 | 0 | 0 | 0 | 3 | 0 |
| protocol/AGENT | 1 | 1 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 0 |
| protocol/README | 2 | 0 | 0 | 1 | 11 | 0 | 0 | 0 | 0 | 0 |
| authoring/README | 3 | 0 | 0 | 2 | 10 | 1 | 3 | 0 | 4 | 3 |
| authoring-syntax/AUTHORIZATION | 120 | 57 | 12 | 7 | 15 | 0 | 2 | 0 | 3 | 0 |
| authoring-syntax/AUTHORING | 25 | 9 | 3 | 16 | 114 | 13 | 28 | 4 | 29 | 8 |
| authoring-syntax/CONTRACTS | 27 | 6 | 0 | 5 | 62 | 12 | 14 | 3 | 28 | 9 |
| authoring-syntax/PACKAGE_WALKTHROUGH | 5 | 1 | 0 | 2 | 17 | 9 | 10 | 1 | 9 | 11 |
| experiments/001 (4 files) | 4 | 32 | 1 | 15 | 63 | 21 | 8 | 2 | 13 | 6 |
| experiments/002 README | 14 | 2 | 3 | 1 | 37 | 1 | 2 | 0 | 5 | 5 |
| proposals/relationships | 19 | 16 | 8 | 0 | 0 | 0 | 0 | 0 | 3 | 0 |
| proposals/consolidation | 19 | 9 | 6 | 5 | 0 | 2 | 2 | 0 | 3 | 0 |
| proposals/declarations | 0 | 0 | 0 | 7 | 0 | 0 | 0 | 0 | 4 | 0 |
| proposals/placement | 5 | 5 | 0 | 2 | 0 | 0 | 0 | 0 | 3 | 0 |
| proposals/vocabulary | 3 | 4 | 2 | 4 | 4 | 1 | 1 | 1 | 0 | 1 |
| proposals/normative-surface | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 0 | 1 | 0 |

Other counts:
- "managed reference": LIFECYCLE 1, AUTHORING 1, experiments/002 1.
- "descriptor": LIFECYCLE 2, CONFIGURATION 1, AUTHORING 12, CONTRACTS 9, PACKAGE_WALKTHROUGH 8.
- "location capabilit": placement 1, packages-and-start 1, vocabulary 1.
- "not open": relationships 1 (RD3), consolidation 1.
- "form" as a field: CONFIGURATION 15.
- "Host" for the engine: packages-and-start:29; authoring-syntax/CONTRACTS.md:10 ("References to a host below mean the responsible management or execution service").
- Zero occurrences of these words: graphs-and-schemas, instantiation (apart from "declaration" twice), conventions (apart from "revocation" twice), reaudit README, core-and-schemas.

What a banner covers:
- A banner is enough for the protocol topics, docs/authoring, authoring-syntax, Experiments 001 and 002, and research, because none of them should be read as current.
- A sentence-level note is needed in records a reader will take as current:
  - declarations DD3, DD4, DR11 (provision; bare-name entry reference)
  - placement PL1, PL4 (provisioning; location capabilities)
  - packages-and-start PD5, PR1, PR4, PR6
  - operations OR4, OD2
  - vocabulary rows at :62, :92, :108, :113
  - normative-surface :31-32
- relationships and consolidation are covered by their existing or proposed notes: their retired words are the subject of the ruling.

====================================================================
ONE ROW PER FILE OR FOLDER
====================================================================

| Path | Action | Why |
| --- | --- | --- |
| docs/protocol/README.md | reduce to a pointer page | Nothing in it is current; 51 inbound links need the path |
| docs/protocol/MODEL.md | archive (banner now) | Grant, alias, renaming re-export, type on every family, unrestricted Interfaces |
| docs/protocol/CONFIGURATION.md | archive (banner now) | `form`, 19 formats, non-atomic create/update batches; half is restated by spec |
| docs/protocol/LIFECYCLE.md | archive (banner now) | Old graph value forms, runtimes, controls, retirement state |
| docs/protocol/CONDUITS.md | archive (banner now); keep :516-561 and :441-488 as type-contract notes | Grant, join and participation model |
| docs/protocol/POOLS.md | reduce to :141-217 | The first half is the pre-ruling-29 entry model; the second is authentication type material |
| docs/protocol/RESOURCES.md | banner; strike :28-51 | Mostly type-contract source material |
| docs/protocol/AGENT.md | banner; retitle as Agent type-contract input | 80% type material; A25 is open |
| docs/proposals/placement | banner and notes (B3) | Accepted, amended by rulings 28, 29, 32 without a pointer |
| docs/proposals/instantiation | keep; fix :5-7, :65, :100-106 | Already says the spec governs |
| docs/proposals/relationships | keep; extend the note (B8) | Example shape and dead IDs |
| docs/proposals/operations | banner and note on OR4, OD2 | Amended by ruling 32 |
| docs/proposals/declarations | banner and notes (B4) | Amended by rulings 27, 29, 32 |
| docs/proposals/graphs-and-schemas | banner | Two small amendments |
| docs/proposals/packages-and-start | banner; fix title and status; notes (B2) | Status is wrong; four rules replaced |
| docs/proposals/naming | banner; fix the example (B7) | `source`, `conduit.dev`, `inputs` |
| docs/proposals/consolidation | banner; note on the "Choices" table (B9) | Two choices reversed |
| docs/proposals/conventions | banner (one line) | Otherwise accurate |
| docs/proposals/schemas-and-interface | banner; note above the example (B6) | The example contradicts final shapes |
| docs/proposals/normative-surface | banner; link to spec/README (B10) | Six conventions, seven rules, "Proposed" |
| docs/proposals/vocabulary | retitle; widen the note (B11) | "proposal", "not yet applied" |
| docs/proposals/reaudit/ (10 files) | keep | Correctly labelled records |
| docs/proposals/core-and-schemas/ | keep; fix the status after the editors (B14) | Says "not accepted"; ruling 37 accepted it |
| docs/proposals/authoring-syntax/ (5 files, 317 KB) | banner on each file now; archive (owner) | Four files claim acceptance with no note |
| docs/authoring/README.md | archive, or replace with a pointer | Duplicate of superseded text; status undefined in AGENTS.md |
| docs/research/README.md | fix the banner and :37 | Calls OVERVIEW.md "the specification" |
| docs/research/* (21 packets) | one-line banner each (owner permission) | Eight have a wrong "current design" note; 13 have none |
| docs/research/naming, vocabulary/README, SIMPLIFICATION | keep | Banners correct |
| docs/archive/README.md | fix :20-21 (owner permission) | "current semantics live in protocol" |
| docs/archive/** | keep | History |
| experiments/README.md | fix :10-11 | Names OVERVIEW and protocol as authoritative |
| experiments/001-shared-access/ | delete with pinned-commit links (owner), or banner CONTRACTS.md and both YAML files | Replaced by spec/schemas/examples; every shape is retired |
| experiments/002-runtime-plugins/ | keep | Banner correct; waits on A23 |
| experiments/003-language-probes/ | keep | Banner correct; cited evidence |

====================================================================
GAPS IN THE SPECIFICATION
====================================================================

Accepted behavior found in scope that spec/ does not state:

1. A requirement composition forwards its own inputs to its constituents by value bindings. MODEL.md:155-159, CONDUITS.md:126-138. spec gives constituents plain `arguments` only. Layer: language.
2. No cached or offline permission: new requests are held or refused when current permission cannot be established, even between two instances on one cut-off system. MODEL.md:359-363; placement record :110-114 (ruling 7). spec has ACC-002 and DL4 but no promise that a cached decision is never used. Layer: convention (access).
3. Secrets never appear in configuration, records or events, and a provider credential is never handed to the caller. POOLS.md:164-166, :195; docs/AUDIT.md:128-130 lists this as distinctly Conduit's. spec has no promise on secrets. Layer: convention (records), or the authentication type contract. Owner decision.
4. A Type contract states whether its implementation may restore its own runtime and what content its records keep. schemas-and-interface record :56 (ruling 31). CTR-004 has neither field, yet OBS-005 says "as its type contract allows". Layer: language (contracts).
5. A declared default that fails its own schema is rejected when the schema is accepted. CONFIGURATION.md:270-273. Layer: language.
6. Fractional numbers are finite binary64, with no silent conversion to exact-decimal contracts. CONFIGURATION.md:137-141. Layer: language (schema profile).
7. Defaults are materialized for omitted operation inputs before invocation, the same for code and graph implementations. LIFECYCLE.md:581-590. spec states defaults only for instantiation, and denies them for controls. Layer: language or convention.
8. Each step's resolved arguments are validated before dispatch, and an invalid one is not dispatched. LIFECYCLE.md:571-579. Layer: convention.
9. A value reference whose pointer selects nothing at run time is a resolution error; no null is substituted and no default is taken. LIFECYCLE.md:545-550. Layer: language or convention.
10. Configuration referenced in a graph is captured when the execution begins. LIFECYCLE.md:387-391. Layer: convention, or add to DL1.
11. Graph outputs: prerequisites on an output, publication before the graph ends, one fixed value per attempt. LIFECYCLE.md:458-496. LD2 sent this to the runtime profile, but DL1 does not list it. Layer: runtime profile (add to DL1), or drop explicitly.
12. Location-bound values (absolute paths, credential-store references, local addresses) are interpreted only at the instance's placement. placement record :152-156 and PL-5 at :263. spec has only INS-018. Layer: convention or type contract.
13. The local caller is authenticated separately from upstream authorization, with no token passthrough. CONDUITS.md:490-492. Related to DL13 and open action A11. Layer: interface.
14. Conduit's context is not forwarded to an upstream provider. LIFECYCLE.md:128-131. IFC-007 covers plain servers only. Layer: interface.
15. A requirement states when its guarantee must hold and which end is obliged, and an open Conduit records the implementations responsible. CONDUITS.md:178-184. Layer: language or type contract.
16. A conformance claim for an implementation or a consumer, as opposed to an engine. CONFIGURATION.md:742-751. Layer: interface.
17. Whether a non-instance Model Context Protocol client can use a public surface. MODEL.md:446-448. Layer: interface.
18. A ready Resource is usable while its Pool's setup is incomplete; successful Conduits are kept across a failed setup. POOLS.md:81-86, CONDUITS.md:41-47. Layer: runtime profile (DL11 names the area only).
19. Cancelling an invocation in progress. vocabulary record :98 lists "cancel" as an accepted verb. spec mentions cancellation only for graphs in DL1. Layer: interface or runtime profile.

Apparently dropped rather than missing; each wants a line somewhere saying so:
- A composite's result contract derived by default (MODEL.md:216-221).
- Combined create-and-start (LIFECYCLE.md:62-63).
- Per-request "required features" (LIFECYCLE.md:135-136).
- Replaceable package dependencies (MODEL.md:233-236; docs/AUDIT.md lists them as deferrable).
- Instance export (CONFIGURATION.md:70-80; placement PL-1).
- The expected-ID check on updates (CONFIGURATION.md:516-519; removed by ruling 37's "by reference only").

Type-contract material with no home yet:
- Agent: AGENT.md:40-220 (activation, accounts, isolation, resume, fork, checkpoints).
- Authentication: POOLS.md:143-217.
- Directory and worktree: RESOURCES.md:111-143.
- Memory, sharing and workspace: RESOURCES.md:172-263.
- Messaging: CONDUITS.md:516-561.
- Provider integrations: CONDUITS.md:441-488.
- Interface attachments and observability: MODEL.md:491-497, :545-617.

====================================================================
READINESS
====================================================================

This part of the repository is not ready for an outside cold read today. The specification is clearly marked, but everything around it still describes itself in the present tense as the accepted design, and the entry points send a reader there first: the root README.md, docs/README.md and docs/protocol/README.md. The three biggest risks of a cold reader being misled:

1. docs/protocol/ is declared binding for "what is not yet converted" while about a fifth of it teaches the retired model under a banner that names only two amending rulings: a Grant object, aliases, `form`, joinable Conduits, unrestricted Interfaces, entries provisioned on demand, non-atomic create/update batches. A reader cannot tell the roughly 20 real leftovers from the roughly 600 contradicted lines.
2. Accepted records, which a reader will rightly treat as the authoritative history of each ruling, show examples and rules that later rulings reversed, with no note: `inputs:` on instances, `conduit.dev`, `source:` in a manifest, a bare-name entry reference, entry cycles allowed, "never dispatched again", "PD1 is open", "not yet applied". Twelve of fifteen never say that the specification governs.
3. Experiment 001's fixtures, the authoring path and the authoring-syntax packet are still offered as the worked examples: by docs/VOCABULARY.md, docs/protocol/README.md, the root README.md and eight research packets whose banners call the old documents "the current design" and "SPEC.md". spec/schemas/examples/ is the only current example set, and nothing in these documents points to it.
```
