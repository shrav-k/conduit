# Round 5, reader 5, the whole repository

Read cold at commit `d0c425f` on the branch `docs/pre-codex`, on 2026-11-07, with a clean
working tree when the read began. Paths are relative to the root of the repository. Nothing
was edited but this file.

## Opening

**What I read.** In full: `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `CLAUDE.md`,
`docs/README.md`, `docs/AUDIT.md`, `docs/OVERVIEW.md`, `spec/README.md`, `spec/definitions.md`,
`spec/names.yaml`, `docs/LANGUAGE_DEFECTS.md` (head and triage), both files under
`docs/implementation/`, `experiments/README.md`, `docs/archive/README.md`,
`docs/research/README.md`, the record of round 1 (`docs/proposals/reaudit/README.md`), the
README of round 4, the records of rulings 41, 42 and 43, and the integration log of ruling 43.
In part, as the entry points sent me: the "Still needs a decision" and "Needs a decision"
sections of the integration logs of rulings 41 and 42, `docs/proposals/ruling-41/open-decisions.md`
(head and "Left for round 5"), `docs/proposals/ruling-42/log-language.md`, the multi-machine
proposal (head and the lines on credentials), the head of every record under
`docs/proposals/`, `docs/proposals/docs-audit/GAPS.md`, the findings in
`docs/archive/plan-audit-history.md`, the docstrings of `tools/set_names.py` and
`tools/check_schemas.py`, and about 40 promises printed from the YAML (every one I cite below).

**The counts, checked with a script.** I parsed every promise file with PyYAML and counted.
Every count in the contents table of `spec/README.md` is exact: 449 promises in 23 files, 202
`language`, 206 `convention`, 41 `interface`, no ID used twice; 16 items in
`implementation-defined.yaml` (DL1 to DL17, no DL14); 61 documents under `spec/core/` and 41
Schema contracts in 8 files under `spec/schemas/`, so 102 exports, as the core manifest lists,
plus the implementation `engine`; 23 Operation, 7 Capability, 5 Type, 2 Requirement, 12 + 5
Schema contracts and 7 templates; 7 valid example files in 3 directories, 56 invalid and 28
invalid-by-rule. The defect register's 167 rows split 48 / 39 / 30 / 50 by layer, and 13 closed
and 74 ruled in the first two layers, 5 closed, 14 ruled and 31 implementation-defined in the
runtime layer, 30 open in the type layer, as `docs/AUDIT.md` says. Every promise ID and DL item
cited in the entry documents exists, apart from IDs cited as removed. `check_docs.py` (159
files, 2253 links), `set_names.py --check` and `check_schemas.py` each report `Errors: 0`.
The counts that disagree are outside `spec/`: the total of promises in three entry documents
(5-5-04) and the size of the implementation-defined list in the tracker (5-5-05).

**Overall judgment.** The entry points still do their main job: from `README.md` a stranger
learns that only `spec/` binds, that there is no engine, model or vector, and where the
tracker is; `spec/README.md` is exact. The faults are of currency and provenance. The tracker
and the README describe the repository as ruling 42 left it, not ruling 43; the tracker's note
for the next reader says a fifth round has read `spec/` when none has; several pointers send a
reader to lists of open questions that later records settled, while the few items that are
still open, among them a list kept for this round, are named nowhere a reader is sent; bare
identifiers in the tracker collide with its own; and the standing of decisions taken after
ruling 39 cannot be read from the tracker. Two findings are in the specification itself: a
configuration value that no rule of conversion covers (5-5-02), and conformance levels that
leave a provider's own duties unclaimed (5-5-03). Nothing I found is unsafe.

| Severity | Count | Findings |
| --- | --- | --- |
| contradiction | 1 | 5-5-01 |
| undetermined | 2 | 5-5-02, 5-5-03 |
| wrong | 2 | 5-5-04, 5-5-05 |
| misleading | 7 | 5-5-06 to 5-5-12 |
| minor | 2 | 5-5-13, 5-5-14 |

## Findings

### 5-5-01. contradiction. The tracker says five rounds have read `spec/`; no fifth round exists

- **Promise IDs.** None; `docs/AUDIT.md` and `docs/proposals/reaudit/round-4/README.md`.
- **Quotes.** `docs/AUDIT.md`, "For the next reader": "**What has been read.** Five rounds of
  cold readers read `spec/` ([re-audit] has the counts by round)". The same file, "Where things
  stand": "Four rounds of cold readers"; A59: "No fifth round is started."; "What happens next",
  step 3: "A fifth round of readers reads the rewritten specification". "Where findings go":
  "in a new directory `docs/proposals/reaudit/round-6/`". Round 4's README: "No fifth round is
  started. ... the next read is the owner's cold read by Codex".
- **Situation.** At `d0c425f` there is no `docs/proposals/reaudit/round-5/`, and the re-audit
  record has counts for rounds 1 to 4 only. `git blame` dates the "Five rounds" sentence and
  the `round-6` path to 2026-11-06 09:35, before rulings 40 to 43: they were written ahead of
  the work. The section that AGENTS.md sends every cold reader to first therefore tells a
  reader that the rewritten specification has been read cold, which README.md and the rest of
  the tracker deny, and sends a reader at this commit to a `round-6` directory while `round-5`
  does not exist. Round 4's README still names the Codex read as the next read, although
  ruling 41 put a fifth round before it.
- **Least change.** Write "What has been read" as "Four rounds ... read `spec/` before the
  rewrite; the fifth is [round 5]" (and make it five only once round 5's record exists), give
  the directory of the next report as the next free `round-N`, and add to round 4's status that
  ruling 41 ordered a fifth round before the Codex read.

### 5-5-02. undetermined. A configuration value that is an object or an array can reach a variable, an argument or a header, and no rule converts it

- **Promise IDs.** MAN-017, PRV-007, IFC-041, MAN-010.
- **Quotes.** MAN-017 rejects a place "at which that schema admits an object or an array, by
  `type`, `enum` or `const`". PRV-007 gives each value "as a string: a string as it is, a number
  in the form that RFC 8785 gives it, and a boolean as `true` or `false`", and "A place that
  holds null gives no variable and no argument". IFC-041 converts a header's value "as PRV-007
  converts a value".
- **Situation.** A Type's schema is `{type: object, properties: {opts: {}}, required: [opts]}`,
  or states `required: [opts]` with no `properties.opts`. An item binds `environment: {OPTS:
  /opts}`. The place is required, and its schema admits an object, but not "by `type`, `enum` or
  `const`", so MAN-017 accepts the template (and so does `check_schemas.py`, whose `admits`
  reads only those three keywords). An instance is registered with `config: {opts: {a: 1}}`.
  At setup PRV-007 has no rule for an object: one engine passes `OPTS={"a":1}` (RFC 8785),
  another ends the part failed, a third leaves the variable out. The same holds for a header
  under IFC-041. The records know this: `docs/proposals/ruling-42/log-language.md` says such a
  place "admits every value and is not caught. For a reader of round 5", and
  `docs/proposals/ruling-43/log-integration.md` calls MAN-017's check "not this ruling's, and
  still open". No entry point lists it (5-5-08).
- **Least change.** MAN-017 rejects a bound place unless every value its schema admits is a
  string, a number, a boolean or null (a `type` or `enum`/`const` limited to those, in the
  schema and in every branch of `anyOf`/`oneOf`), as its own observable argues ("a variable, an
  argument or a header holds a string"); the checker's rule follows. The other way, PRV-007
  states that an object or an array is given in the form RFC 8785 gives it.

### 5-5-03. undetermined. No conformance level covers what a runtime provider or a machine provider must itself do

- **Promise IDs.** PRV-011, PRV-014, PRV-019, PRV-021, PRV-022, PRV-025, PRV-028; SEC-006;
  `spec/README.md`, "Conformance".
- **Quotes.** `spec/README.md`: "what a provider must do are `convention`, as in PRV-005,
  PRV-007, PRV-011, PRV-022 and SEC-010". The Engine level covers "every promise whose layer is
  `convention` or `interface`"; the Implementation level covers "Every promise whose layer is
  `interface` and that binds an implementation". PRV-025 (`convention`): "The server stands
  behind a gate". AUDIT A63: "Each passes the vectors of its contract".
- **Situation.** A runtime-provider package is an implementation, not an engine, so it claims
  the Implementation level, which covers only `interface` promises. Its own duties, to gate its
  runs (PRV-025), prove the run's certificate at its `url` (PRV-028), keep secrets and
  `identity` as stated (PRV-011, PRV-022), run only on the named machine (PRV-019) and keep a
  stop it owes (PRV-021), are all `convention`, so an Implementation-level claim says nothing
  about them; a machine provider's duty to report `gone` only where it knows (PRV-014) is the
  same. They are instead inside the Engine level, which an engine claims for promises that only
  the provider the user picks can keep. A store's duty, by contrast, is `interface` (SEC-006)
  and is covered. Two runtime-provider packages that each claim conformance may differ on
  whether a gate stands before their runs, and a user cannot tell from either claim; A63's
  "vectors of its contract" have no level to report against.
- **Least change.** Add a Provider level, claimed by a package that implements a provider
  capability, covering every promise whose obligation falls on the provider (among them
  PRV-010 to PRV-012, PRV-014, PRV-019, PRV-021, PRV-022, PRV-025 and PRV-028, and SEC-006 for a
  store), and say that the Engine level covers the engine's side of the provider contracts
  only.

### 5-5-04. wrong. Three entry documents give 440 promises; the specification holds 449

- **Promise IDs.** None; counts.
- **Quotes.** `README.md`: "Every area of the language is written as numbered promises, 440 of
  them." `docs/AUDIT.md`, "Where things stand": "440 numbered promises in three layers".
  `docs/implementation/INDEPENDENT_BUILDS.md`: "`spec/` holds 440 promises, 38 of them in"
  `interface/implementation.yaml`, and in row G2 "`spec/` holds 440 promises". `spec/README.md`:
  "449 in all: 202 of the layer `language`, 206 of the layer `convention` and 41 of the layer
  `interface`".
- **Situation.** My count of the YAML gives 449, 202 / 206 / 41, and 41 promises in
  `interface/implementation.yaml`. 440 is the count after ruling 42 (its record: "The
  specification then held 440 promises"); ruling 43's record says "The specification then held
  449 promises". The README and the tracker both link the contents table that contradicts them.
- **Least change.** 449 in `README.md` and `docs/AUDIT.md`; "449 promises, 41 of them in"
  in the changes paragraph of `INDEPENDENT_BUILDS.md`. Better, have the README and the tracker
  name no total and point to the contents table, so the next ruling cannot leave them behind.

### 5-5-05. wrong. The tracker says the implementation-defined list has fourteen items; it has sixteen

- **Promise IDs.** DL16, DL17.
- **Quotes.** `docs/AUDIT.md`, "Later amendments", row for ruling 31: "Ruling 41 removed DL14,
  ... which leaves fourteen." `spec/README.md`: "Draft; 16 items".
- **Situation.** Ruling 42 (D8) added DL16 and ruling 43 added DL17; the row stops at ruling 41.
  A reader who checks a conformance document against "fourteen" misses two items.
- **Least change.** End the row: "... which leaves fourteen; ruling 42 (D8) added DL16 and ruling
  43 DL17, which makes sixteen."

### 5-5-06. misleading. The README, AGENTS.md and the tracker describe the specification as ruling 42 left it

- **Promise IDs.** SEC-010, SEC-013, SEC-023, PRV-026 (for the last quote).
- **Quotes.** `README.md`: "| Part | State on 2026-11-06 |" and "Rulings 40 to 42 then had it
  rewritten around provider contracts"; ruling 43 and work across machines are not mentioned.
  `AGENTS.md`: "Pull request 7, on the branch `docs/pre-codex`, carries the work of rulings 39
  to 42". `docs/AUDIT.md`, "Where things stand", Specification: "it was rewritten around
  provider contracts under rulings 40 and 41"; Secrets: "a core capability that each store's
  package implements, read only by the engine before a run". `docs/OVERVIEW.md`: "last read
  against the specification on 2026-11-06, as ruling 43 left it".
- **Situation.** The commit that applied ruling 43 (`d0c425f`, 2026-11-07) touched neither
  `README.md` nor `docs/AUDIT.md`'s state table. A reader of the README learns nothing of the
  gate, the two forms of proof or DL17. The Secrets row is false of the text: the engine also
  reads a secret before it connects to a `remote` item (SEC-013), again where it has lost a
  connection's values (SEC-023), and before it runs a provider again after a restart (PRV-026).
  The overview was changed by ruling 43's verification, dated 2026-11-07.
- **Least change.** In the README, date the row 2026-11-07, write "Rulings 40 to 43", and add
  one sentence on ruling 43 (work across machines: a gate before every server the engine does
  not run, two standard forms of proof). In AGENTS.md, "rulings 39 to 43". In the tracker,
  name rulings 42 and 43 in the Specification row, and write the Secrets row as "read only by
  the engine, for a run or a connection, and again after a restart (SEC-010, SEC-013, SEC-023,
  PRV-026)". Date the overview 2026-11-07.

### 5-5-07. misleading. Entry points send a reader to lists of open questions that later records settled

- **Promise IDs.** None; pointers.
- **Quotes.** `docs/AUDIT.md`, A62: "the questions the rewrite left open are in [its log]
  (proposals/ruling-41/log-integration.md#still-needs-a-decision)". `spec/README.md`: the
  coverage check's "log lists what it fixed and what waits on a decision"; ruling 42's
  verification "lists what still needs a decision"; ruling 43's verification "lists what still
  needs a decision". `docs/README.md`, ruling-41 row: "whose coverage check lists what waits on a
  decision".
- **Situation.** The list that A62 links (D1 to D20) was answered in
  `ruling-41/open-decisions.md` and by rulings 42 and 43; the "Still needs a decision" lists of
  rulings 42 and 43 were settled in the "After the ruling" sections of their records (items 1,
  3 and 4; N1 to N4 and items 6 to 8) and in the table "The items that waited on this ruling".
  None of the linked lists says that it was settled or where. A reader following the tracker or
  the specification's README reads some sixty questions as open, and cannot find the one or
  two that are (5-5-08) without reading every record.
- **Least change.** Point each of these at the record that settled the list (open-decisions,
  the "After the ruling" sections of rulings 42 and 43), in the past tense: "the questions it
  left open were settled in ...". Add one line under each "Still needs a decision" heading
  saying where it was settled.

### 5-5-08. misleading. What was left for round 5, and what is still open, is named nowhere a reader is sent

- **Promise IDs.** MAN-017, IFC-004, IFC-008, IFC-037 to IFC-039, CTR-012, REV-010, TPL-004,
  CTR-004, PLC-024, OPS-028; DL16.
- **Quotes.** `docs/proposals/ruling-41/open-decisions.md`: "## Left for round 5", "To check
  against the protocol (R28): the extension negotiation, `listChanged` and `cacheScope`, and
  the reserved trace keys", then five points "For a reader". `ruling-42/log-language.md`: "For a
  reader of round 5" and "Left for the check against the protocol in round 5 (R28)".
  `ruling-43/log-integration.md`: MAN-017's check "still open". `ruling-42/log-integration.md`:
  "**Hardening the engine's own environment** (X2), optional and the owner's". `docs/AUDIT.md`,
  "Waiting for the owner": "Nothing at present."
- **Situation.** No entry point, and not the tracker's "Known to be open", "What happens next"
  step 3 or "Where findings go", mentions these lists. The check of the interface against the
  protocol that two earlier rounds had (a separate reader with the published protocol) is
  pending for this round by R28, yet step 3 describes the round as readers "with scenarios
  across several machines" only. MAN-017's gap is still open by ruling 43's own log (5-5-02).
  X2, an option the record says is the owner's, is neither listed under "Waiting for the owner"
  nor said to be settled by ruling 43's Q5. A reader who follows the entry points cannot do the
  work the records set aside for this round.
- **Least change.** Add to "For the next reader" a bullet "Left for round 5:" linking
  `open-decisions.md#left-for-round-5` and the two log sections, and name the R28 check in step
  3; list MAN-017 under "Known to be open"; list X2 under "Waiting for the owner", or say that
  Q5 settles it.

### 5-5-09. misleading. Bare identifiers in the tracker name the tracker's own D and Q items and also the questions of rulings 42 and 43

- **Promise IDs.** None; identifiers.
- **Quotes.** `docs/AUDIT.md`: "A bare identifier in this document is one of the tracker's own",
  with "**Q**, **D** | Question and disagreement". "Waiting for the owner": "Ruling 43 settled
  the last open question, D5 and D20." A20: "| A20 | Rule on v0 authorization scope. | D5 |",
  status "open". Ruling 42 row: "a request through a closed Conduit reports refused (D6); the
  engine's own runner passes six variables ... (D8)". A65: "ruling 43 (Q4)". Ruling 43 row:
  "(Q1)". `INDEPENDENT_BUILDS.md`, G8: "disagreements D5, D6 and D8 (A20, A19, A24)".
- **Situation.** The tracker's own D5 is "The authorization machinery exceeds what is
  enforceable natively" (open, under A20); D6 is the graph language; D8 the review queue; Q1
  "What does v0 do that a plain MCP configuration cannot?"; Q4 "Who is the independent
  author?". The D5, D6, D8, D20 and Q1 to Q5 of the ruling rows are the numbers of
  `ruling-41/open-decisions.md` and of the multi-machine proposal. So "Ruling 43 settled ... D5"
  reads as closing A20's source, which is open, and G8's "D5, D6 and D8" cannot be told from the
  ruling-42 row's. The identifiers table has no row for those two records, against its own rule
  that a record's identifier is written with the record's name.
- **Least change.** Write them with the record's name ("question D5 of the open questions after
  the rewrite", "Q4 of the multi-machine proposal"), or add two rows to the identifiers table:
  "D1 to D20, R1 to R28: the open questions after the rewrite" and "Q1 to Q5: the multi-machine
  proposal", and say that in the ruling rows of 42 and 43 they are those.

### 5-5-10. misleading. The tracker does not say which decisions made after ruling 39 stand unaccepted, or where they are

- **Promise IDs.** None; provenance.
- **Quotes.** `docs/AUDIT.md`, "For the next reader": "In short: rulings 1 to 43 are the owner's.
  Ruling 39 accepted every logged decision of the coordinator and choice of an editor made before
  it, and ruling 41 the planner's choices for the rewrite. A reader who disagrees with any of
  them should say so." "Who decides what": "A later one has the standing of a decision the owner
  can strike, until a ruling accepts it." Ruling 42 row: "The owner struck none of the
  coordinator's 43 decisions on the rest". `spec/README.md` says the coordinator's decisions are
  "logged in GAPS.md and in the decisions of round 2, of round 3 and of round 4".
- **Situation.** Since ruling 39 the specification changed by the coordinator's 43 decisions in
  `ruling-41/open-decisions.md`, its three decisions after ruling 42 and those after ruling 43
  (each in the "After the ruling" section of the ruling's record), the twelve choices of
  ruling 43's verification, the choices of each editing group in the logs of rulings 41 to 43,
  and names "coined ... for the owner to strike". The tracker's summary names none of these. "Struck none"
  is not "accepted", and the tracker does not say which it is, so a reader cannot tell whether
  the 43 decisions stand as the owner's or as the coordinator's. `spec/README.md`'s list of where
  the coordinator's decisions are logged stops at round 4. A reader asked to "say so" if they
  disagree has no list to disagree with.
- **Least change.** In "Who decides what", add one paragraph: the coordinator's decisions since
  ruling 39 are in open-decisions.md and in the "After the ruling" sections of rulings 42 and
  43; editors' choices are under "Choices" in each log of rulings 41 to 43; ruling 42's "struck
  none" leaves the 43 decisions [accepted | strikeable] (the owner to say). Extend
  `spec/README.md`'s list with the same three places.

### 5-5-11. misleading. Records and indexes still call ruled proposals unruled

- **Promise IDs.** None; statuses.
- **Quotes.** `docs/proposals/vocabulary-audit/README.md`: "Status: proposed 2026-11-06, for the
  owner. ... This is a proposal. Nothing in it is ruled." `docs/research/README.md`: the
  multi-machine proposal, "which is not ruled". `docs/README.md`: "every packet but the naming
  and secrets research predates the specification". `principles-review/README.md` and
  `secrets/README.md`: "> **Ruled.**" followed by "Status: **proposed, not ruled.**".
  `docs-audit/GAPS.md`: "## Left for the owner", "- GP3, how a value is marked as secret."
- **Situation.** Ruling 42 accepted all twenty names and the editorial fixes of the vocabulary
  audit, which AGENTS.md and CONTRIBUTING.md link as accepted; the audit itself says nothing in
  it is ruled, with no banner. Ruling 43 ruled the multi-machine proposal, and
  `research/MULTI_MACHINE.md` also postdates the specification. Two records carry both "Ruled"
  and "proposed, not ruled" at their head. GP3 was settled by ruling 41 (`secrets.yaml`); GAPS.md
  is linked from `spec/README.md` as a log of decisions.
- **Least change.** Give the vocabulary audit the banner the other two have ("Ruled. Ruling 42
  accepted N1 to N20 and E1 to E23"); write "which ruling 43 accepted" in the research index and
  "the naming, secrets and multi-machine research" in `docs/README.md`; change the two
  "Status:" lines to "Was:"; add "settled by ruling 41" after GP3.

### 5-5-12. misleading. A behavior that rulings 42 and 43 accepted was withdrawn by a verifier's choice that no list records

- **Promise IDs.** PRV-022, DL13.
- **Quotes.** Ruling 42, D17, accepted: "a credential as a file that the runtime provider may
  refresh in place". The multi-machine proposal, accepted by ruling 43: "PRV-022 already lets a
  provider replace the file in place, and v1 uses that" and "Credentials are per instance, by
  ID, in files that PRV-022 lets a provider replace in place." PRV-022's observable now:
  "replacing a file's content in place while the run lasts ... is left for renewing a credential
  on the run's machine, which this version does not do". Ruling 43's verification, MX-21:
  "PRV-022's leave to replace a file's content in place moved from its `then` to its observable".
- **Situation.** What two rulings accepted, and what the accepted v1 design (A65) relies on, was
  removed from PRV-022's obligation by a finding of the verification. MX-21 is not among the
  verification's twelve listed "Choices", not among the coordinator's decisions in ruling 43's
  record, and not in the tracker's "Later amendments", whose ruling 42 row still lists D17 as
  accepted without the change. A reader comparing ruling 42 with PRV-022 sees an unexplained
  reversal; a provider author reading ruling 42 would expect the permission.
- **Least change.** Add a row to "Later amendments": "42 (D17) | a provider may refresh a
  credential file in place | Withdrawn for v0 by ruling 43's verification (MX-21): a credential
  lasts as its instance; renewal in place is A65's", and list MX-21 among the verification's
  choices for the owner to strike.

### 5-5-13. minor. Three action items are out of date

- **Promise IDs.** None; `docs/AUDIT.md`, A21, A22, A29.
- **Quotes.** A21: "One, a stale branch, waits for the owner". "Waiting for the owner": "Nothing
  at present." A22: "| A22 | Keychain probe. | E10 | Results recorded. | open |". A29: "recording
  old and new spellings in the vocabulary map".
- **Situation.** The branch is `implementation/rust-runtime-plugins`; ruling 39, point 12,
  deleted the stale branches, and `git branch -a` shows only `main` and `docs/pre-codex` here, so
  A21 both waits on nothing and contradicts "Nothing at present". Experiment 005 is probe E10 and
  recorded its results on 2026-11-06 (A63 says only the daemon case is untested), yet A22 is
  "open" with no pointer. A29's target, the vocabulary map, was removed by ruling 39, while
  `spec/definitions.md`, "Words the promises do not use", now records old and new names.
- **Least change.** A21: drop the branch clause. A22: "in progress: Experiment 005; a
  background daemon is untested". A29: name `spec/definitions.md#words-the-promises-do-not-use`
  as where the spellings are recorded, or close it.

### 5-5-14. minor. AGENTS.md says the names tool rewrites every occurrence; it rewrites `spec/` and two planned directories

- **Promise IDs.** None; `AGENTS.md`, `tools/set_names.py`.
- **Quotes.** `AGENTS.md`: "add `--write` to rewrite every occurrence together". The tool's
  docstring: "rewrite every file under the given roots (default: spec, conformance,
  reference-model)".
- **Situation.** `docs/OVERVIEW.md` quotes the example project, "quoted from the specification's
  example project", with `apiVersion: github.com/shrav-k/conduit/v0` and the core address. A
  rename by the tool leaves the overview's quotation differing from the file it claims to quote.
  Nothing binding breaks, but the instruction overstates what the tool does.
- **Least change.** In AGENTS.md: "to rewrite every occurrence under `spec/` (and `conformance/`
  and `reference-model/` once they exist); then update the overview's quoted example by hand".
