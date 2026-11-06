# Audit of the documents outside `spec/`: log of the edits to the older and supporting text

Editor's log of 2026-11-05. It records what was changed to carry out decisions DA1 to DA8 of the
[audit record](README.md) in the files that the [older-text report](reader-older-text.md)
covers. Finding numbers (A1 to E) are that report's. Nothing under `spec/` was edited, and no
record's decisions or examples were rewritten: each change is a banner, a status line or a note
beside the text it concerns.

## What changed

### `docs/protocol/` (8 files)

| File | Change | Findings |
| --- | --- | --- |
| `MODEL.md`, `CONFIGURATION.md`, `LIFECYCLE.md`, `CONDUITS.md`, `POOLS.md`, `RESOURCES.md`, `AGENT.md` | The shared banner, the "Scope: accepted working contracts" line and the "Part of the accepted design contracts" line are replaced. The new banner says: superseded; nothing in the file binds; it predates rulings 16 to 37; `spec/` governs and where the file differs it is wrong; what it is kept for; that a behavior stated there and not in `spec/` is a gap in `spec/`; and that its words are the older ones. Bodies and headings are untouched. | A2, A4 to A10, A11 option (a); DA1, DA2 |
| The same, what each is kept for | `MODEL.md`: Interface attachments and observability. `CONDUITS.md`: messaging and provider integrations. `POOLS.md`: shared authentication. `RESOURCES.md`: the directory, worktree, memory and workspace types. `AGENT.md`: the Agent type. `CONFIGURATION.md` and `LIFECYCLE.md`: links only. | The report's list of type-contract material |
| `README.md` | The body is replaced by a pointer page: nothing here binds; the specification is `spec/`; what each topic file is kept for and where its subject is now stated; where rulings are logged; the current worked example. The heading "Contract map" is kept because four files link to its anchor. No other anchor of the old page had a link from outside the banners that were replaced. | A3, D2, C8 (the link that offered Experiment 001 as the walkthrough is gone); DA2 |

### Proposal records (13 files, each `README.md`)

Every record now has, under its status, the line "This is the record of a ruling. The
specification in `spec/` governs where they differ. Amended by: ...", with the later rulings
that changed something in it. Every record that cited promise IDs also has "Promise IDs are as
first written; the comment at the head of each spec file says where a removed ID went."
Finding B1 and decision DA8 are answered by these two lines.

| Record | Further changes | Findings |
| --- | --- | --- |
| `placement` | Status: spellings "were illustrative"; "PLC-001 to PLC-014 as first written", and the PLC wording governs over PL-1 to PL-10. Notes above: the constraints list (`form` dropped), PL1 (provisioning), PL4 (location capabilities), PL5 (when the shared-host requirement is checked), the worked examples (`inputs` is now `arguments`; the reference schema is a core-package export), and "Still to specify". | B3, D3, E |
| `instantiation` | Status: the decisions are written as INS promises, no longer "will be restated". The existing amendment note is replaced by the standard line with rulings 18, 19, 28, 29, 31, 32, 33 and 37. Notes above the example (`arguments`; the reference schema; `type`) and above "Still to specify". | B12 |
| `relationships` | The existing amendment note is replaced by the standard line and widened: rulings 27 to 29, 32 and 37. Note above the Conduit example: `consumer` and `target` are top-level and the rest goes under `arguments`. | B8 |
| `operations` | Notes above the decisions (OD2: the operation's contract declares deduplication, a plain implementation never counts, "grant") and above the rules (OR4 reversed by decision Q1; OR3; OR9 "unknown"). Note under "A stale revision is refused": the reason is conflict. | B5, E |
| `declarations` | Notes above the decisions (DD3 and DD4, stated in terms of provisioning) and above the rules (DR10: contracts and the manifest; DR11: `{entry: name}`). | B4, E |
| `graphs-and-schemas` | Note above the rules: SR3 amended by decision F23. | B13 |
| `packages-and-start` | Title: "accepted", no longer "accepted in part". Status: PD1 was settled by rulings 25 and 27 and nothing is open. "PD1 is open" and "waits on PD1" are put in the past tense. Notes on PD1, PD5 ("Host" is "Engine"), PR1, PR4, PR6 and PR8. The row for PD1 in "Where each is written" says settled. | B2, E |
| `naming` | "NAM-001 to NAM-017 as first written". Note above the examples: `source` is now `fetch`; a manifest carries `apiVersion`; `inputs` is now `arguments`; `conduit.dev` is not a name of the specification. A line under each of the headings "What it would look like" and "Rules that would follow" says it was accepted and that the heading is as first written. | B7 |
| `consolidation` | Note above "Choices made while writing the rulings": the core package's address; entry cycles are rejected; "Still unwritten" was decided by ruling 30. The amended-by line also says that a right to open stated for a Pool covers the Pool itself. | B9 |
| `conventions` | The standard line only, with rulings 32, 33 and 37. | Table row |
| `schemas-and-interface` | Notes on X1 (settled by ruling 33), on D6 and D11 (five kinds of contract), above the example (`conduit.dev`, `controls`, the reference schema), on I2 and I6, and on the list (thirteen entries; DL9). | B6 |
| `normative-surface` | The standard line also links the form as it now stands and says the four decisions were accepted although worded "Proposed". Notes on NS1 (seven conventions; implementation interface), on the writing rules (five, and the `layer` field) and on NS3 (the topics keep their paths with a banner). | B10, E |
| `vocabulary` | Title: "accepted", no longer "proposal". "Not yet applied to the text" is replaced: the names are applied in the specification. The amendment note is widened and ends by naming `spec/definitions.md` as the current vocabulary. A note above each table names the rows that later rulings replaced. | B11, E |

### `docs/proposals/authoring-syntax/` (5 files) and `docs/authoring/README.md` (1 file)

| File | Change | Findings |
| --- | --- | --- |
| `AUTHORING.md`, `AUTHORIZATION.md`, `CONTRACTS.md`, `PACKAGE_WALKTHROUGH.md` | One banner under the title: not binding; predates rulings 13 to 37; uses shapes and words the specification has replaced; the current worked example is `spec/schemas/examples/valid/`. | C6, D4; DA6 |
| `authoring-syntax/README.md` | The direction note is replaced by the same banner, which drops the wrong clause "the naming review it puts first now comes last". Under "Accepted rules to apply", the lead sentence and the column heading no longer call the protocol topics the canonical home. | C6 |
| `docs/authoring/README.md` | The direction note is replaced by the same banner, which drops the wrong clause "the naming discussion ... now comes last". | C5, D1; DA6 |
| `PACKAGE_WALKTHROUGH.md`, `authoring-syntax/README.md` | The link to Experiment 001 points at commit `7927d68`, with "since removed" beside it. | C8; DA4 |

### `docs/research/` (22 files)

| File | Change | Findings |
| --- | --- | --- |
| Twenty-one packets | One line under the title: "Not binding. This research predates the specification in `spec/`; where it says accepted, current, SPEC.md or DESIGN.md, it means the documents as they stood before 2026-11-05." In `CONDUITS.md`, `ESTABLISHMENT.md`, `PI_CONNECTORS.md`, `PI_RUNTIME.md` and `RESOURCES.md` the line replaces the "Current-design reading note". In the other sixteen it is added. | C2, C3, C7; DA5 |
| `README.md` | The banner says that nothing under `docs/research/` binds. The sentence that called `OVERVIEW.md` the specification now names `spec/` and its examples. The two mentions of candidate tables and walkthroughs say they were removed under ruling 36. | C1, C7 |
| `CONDUITS.md`, `CONFIGURATION.md`, `CONSTRUCTION.md`, `ESTABLISHMENT.md`, `PI_CONNECTORS.md`, `RESOURCES.md` | Nine links to Experiment 001 point at commit `7927d68`. | C8; DA4 |
| `naming/README.md`, `vocabulary/README.md`, `vocabulary/SIMPLIFICATION.md` | Not changed: the report found their banners correct. | — |

### `docs/archive/README.md` (1 file)

"Nothing here binds" is added under the title. The two statements about where current text lives
now point to `spec/`, and say that the protocol topics the archived documents link to are
superseded. Finding C4; decision DA5.

### `experiments/`

| Path | Change | Findings |
| --- | --- | --- |
| `001-shared-access/` | Removed with `git rm -r` (four files). | C8; DA4 |
| `002-runtime-plugins/README.md` | Its one link to Experiment 001 points at commit `7927d68`. | DA4 |
| `003-language-probes/` | Not changed. | — |
| `004-stateright-evaluation/README.md` | The two citations of protocol sections are replaced by the promises that now state the rules: OPS-008 to OPS-013 and REV-002, REV-011 and REV-012. A sentence says the rule quoted under Findings is from the older text. | A6, A7 |

Every promise ID cited in an added line was checked against the promise files as they stood when
the work ended.

## Findings not carried out, and why

| Finding | Why |
| --- | --- |
| A1: the claim in `spec/README.md`, `AGENTS.md` and `docs/README.md` that unconverted behavior lives in `docs/protocol/` | Those files belong to other editors. |
| A4 to A10: the contradicted sentences inside the topic files; A8 "reduce `POOLS.md`"; A9 "strike `RESOURCES.md` :28-51"; A10 "retitle `AGENT.md`", and its two references to sections "below" that are in other files | Decision DA2 chose option (a) of A11: a banner only, with bodies and headings untouched. The banner says what each file is kept for. |
| A11 option (b): archive four topics | Not chosen (DA2). |
| B14, B15: `core-and-schemas` and `reaudit` | Outside this editor's files. |
| C2, for `CONFIGURATION.md`, `CONSTRUCTION.md`, `RUST_RUNTIME.md` and `PI_RUNTIME.md` | The wrong sentence is part of the opening paragraph, not a separate banner. The one line is added above it and says how to read it; DA5 allows nothing else to be edited. |
| C5, C6, D4: archive or move `docs/authoring/` and the authoring-syntax folder | DA6: a banner on each file, and they are not moved. |
| C7: the labels "SPEC.md" and "DESIGN.md" in research | DA5: one line only. The line says what the labels mean. |
| C8: `experiments/README.md` | Belongs to the entry-points editor. |
| C8: research sentences that call Experiment 001 "the current YAML packet" | DA5. The links are repointed and the one line says how to read "current". |
| `docs/archive/README.md`: "Current review is the only active question queue" | The instruction limited edits there to statements about where current text lives. The direction note above it already names the plan audit as the tracker. |
| `experiments/002-runtime-plugins/README.md`: the label "specification" on a link to `docs/OVERVIEW.md` | The instruction limited edits there to what the report lists, and it lists nothing for 002. Worth fixing in a later pass. |
| The nineteen gaps and six dropped behaviors under "Gaps in the specification" | DA3: a pass of their own. |

## Links to Experiment 001 that remain in files outside this editor's set

These are the 21 errors that `python3 tools/check_docs.py` reports after the removal.

| File | Links | Whose |
| --- | --- | --- |
| `docs/VOCABULARY.md` | 17, on lines 22, 24, 25, 35, 37, 40, 41 (two), 51, 60, 63, 65, 66, 72, 74, 76 and 77 | The entry-points editor |
| `experiments/README.md` | 1, on line 15 | The entry-points editor |
| `docs/archive/collaborator-proposals/PR-002-protocol.md` | 1, on line 59 | No editor: `docs/archive/` is not edited |
| `docs/archive/reviews/CONCEPTUAL.md` | 1, on line 115 | No editor |
| `docs/archive/reviews/AUTHORING-2026-11-04.md` | 1, on line 747 | No editor |

The three archive links need a decision: repoint them to commit `7927d68`, as ruling 36 did for
the vocabulary research, or exempt them in the checker.

Mentions that are not links, and so are not errors:
`docs/archive/migrations/2026-11-protocol-framing.json` (six paths in its accounting),
`docs/implementation/ORGANIZATION.md` line 92 (a directory listing) and the verbatim
[older-text report](reader-older-text.md).
