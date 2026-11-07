# Re-audit, round 5: log of the fix pass for area F7

Date: 2026-11-07. Area F7, the tracker and the entry documents, applies items M100 to M111 of
[DECISIONS.md](DECISIONS.md) to the documents outside `spec/`. It changes no promise, so no
promise ID is changed, added or removed here, and no file under `spec/` is edited.

## Applied

| Item | Files | Promise IDs | What changed |
| --- | --- | --- | --- |
| M100 | `docs/AUDIT.md` | none | "What has been read": four rounds read `spec/` before the rewrite under rulings 40 to 43, and round 5 read the rewrite, with a link to its record. "How it was checked" says the four rounds read before the rewrite, and adds round 5 with the third check against the protocol. A59 drops "No fifth round is started." and says that ruling 41 ordered the round and that round 5 read the text on 2026-11-07. "Where findings go" calls `round-6/` the next after round 5. |
| M100 | `README.md` | none | The Specification row says that round 5 read the rewrite before pull request 7 is merged, with a link to its decisions. |
| M100 | `docs/proposals/reaudit/round-4/README.md` | none | One sentence added at the end of the Status paragraph: ruling 41 later ordered a fifth round, with a link to round 5. Nothing else changed. |
| M101 | `README.md`, `docs/AUDIT.md` | none | The totals "440" are dropped; the README links the contents table of `spec/README.md`, which the tracker's row already linked. |
| M101 | `docs/implementation/INDEPENDENT_BUILDS.md` | none | The paragraph of changes since the assessment and row G2 give 450 promises, 41 of them in `interface/implementation.yaml`, the counts that [DECISIONS.md](DECISIONS.md#items-that-change-one-text) gives once PKG-030 (M25) is in. When this pass ran, the validation counted 449, since the pass of area F2 had not yet added PKG-030, and the interface file held 41. The handoff recount confirms them. Beside it, the same paragraph's "the rewrite has not been read cold (G7)", which round 5 made false as it did the README's sentence under M100, now says that round 5 read the rewrite and reported ten unsafe findings, with a link to its decisions. |
| M102 | `docs/AUDIT.md` | none | Later amendments, row of ruling 31: ruling 42 (D8) added DL16 and ruling 43 (Q2) added DL17, which makes sixteen, and round 5 removed DL11 (M69), which leaves fifteen. M69 is decided fix, so the first word stays "Fifteen." The identifiers follow the ruling number, as M106 writes them. |
| M103 | `README.md` | none | Dated 2026-11-07; "Rulings 40 to 43"; a sentence on ruling 43 after the sentence on secrets. It says "the engine's address", the term that M93 defines, for the decision's "the engine's own address", and "a gate that admits only a request that proves the engine sent it" for "admits only proof that the engine sent the request". |
| M103 | `AGENTS.md` | none | "rulings 39 to 43", with work across machines. The paragraph's last line, which was over 120 characters, is rewrapped. |
| M103 | `docs/AUDIT.md` | none | "Where things stand" dated 2026-11-07; the Specification row names rulings 40 to 43 and links the records of rulings 42 and 43; the Secrets row lists every read (SEC-010, SEC-013, SEC-023, PRV-026). |
| M104 | `docs/AUDIT.md` | none | A62 says that the questions the rewrite left open were settled in the open questions after the rewrite and by rulings 42 and 43. |
| M104 | `docs/README.md` | none | The row of `proposals/ruling-41/` says that the open questions after the rewrite and rulings 42 and 43 settled the integration log's questions. |
| M104 | `docs/proposals/ruling-41/log-integration.md` | none | One dated line under "Needs a decision" (R1 to R28) and one under "Still needs a decision" (D1 to D20), as decided. Nothing else changed. |
| M104 | `docs/proposals/ruling-42/log-integration.md` | none | One dated line under "Needs a decision" and one under "Still needs a decision", as decided. The first also says that round 5 fixes MAN-017's check (M30), which DECISIONS.md decides, so that the line does not leave it open. |
| M104 | `docs/proposals/ruling-43/log-integration.md` | none | One dated line under "Still needs a decision", as decided. |
| M105 | `docs/AUDIT.md` | none | Step 3 of "What happens next" names the check against the protocol that R28 left for round 5. "For the next reader" gains the paragraph "Left for a reader by the records", which also says that round 5 fixes MAN-017's check, which two of the linked lists name (M30). M30 is decided fix, so no bullet is added to "Known to be open". The row of "Waiting for the owner" names the record of D5 and D20, and says that Q5 of the multi-machine proposal, with a link to its section, settled item 7 of ruling 42's verification, X2. |
| M106 | `docs/AUDIT.md` | none | "Identifiers" gains the sentence on the rows of rulings 42 and 43, and two rows: D and R, the open questions after the rewrite, and Q, the multi-machine proposal. |
| M107 | `docs/AUDIT.md` | none | "Who decides what" gains the paragraph that lists where the decisions and choices made since ruling 39 are logged; "Who decided what" says that those have the standing of decisions the owner can strike. |
| M107 | `AGENTS.md` | none | The bullet on `docs/proposals/` says that the coordinator's decisions and the editors' choices made since ruling 39 can each be struck, and links "who decides what". |
| M108 | `docs/proposals/vocabulary-audit/README.md` | none | A banner "Ruled." and "Status:" becomes "Was:". The banner says that ruling 42 accepted N1 to N20 and left the editorial fixes, E1 to E23, to the coordinator, who applied them: ruling 42's record says "The 23 editorial fixes of the vocabulary audit are the coordinator's to apply", not that the ruling accepted them, so the decided "accepted ... and the editorial fixes" would overstate it. |
| M108 | `docs/proposals/principles-review/README.md`, `docs/proposals/secrets/README.md` | none | "Status:" becomes "Was:" on the line under the banner. |
| M108 | `docs/research/README.md` | none | The index says that ruling 43 accepted the multi-machine proposal. No packet is edited. |
| M108 | `docs/README.md` | none | The row of `research/` names the research on work across machines among the packets that do not predate the specification. |
| M108 | `docs/proposals/docs-audit/GAPS.md` | none | One line after the list under "Left for the owner": ruling 38, point 2, dropped the six behaviors, and ruling 41 settled GP3. Nothing else changed. |
| M109 | `docs/AUDIT.md` | none | Later amendments, first table, a last row for ruling 42 (D17) and MX-21. |
| M109 | `docs/proposals/ruling-43/log-integration.md` | none | Item 13 added to "Choices of the verification", as decided. |
| M110 | `docs/AUDIT.md` | none | A21: the stale branch was deleted under ruling 39, point 12 (`git branch -a` shows only `main` and `docs/pre-codex`). A22 is in progress, with Experiment 005's results of 2026-11-06 and the daemon untested. A29 and the Vocabulary row point to "Words the promises do not use" in `spec/definitions.md`; A29 stays in progress. |
| M111 | `AGENTS.md` | none | "Checks": `--write` rewrites under `spec/`, and under `conformance/` and `reference-model/` once they exist, and a document outside them that quotes a name, such as the example in `docs/OVERVIEW.md`, is updated by hand. `CONTRIBUTING.md` already says only "add `--write` to apply", and is unchanged. |

## Not applied here

The change list given to this pass also held changes to files under `spec/`. They are outside
F7's files and belong to the passes of their own areas (F1 for M01 to M22, F6 for M90,
M94, M97 and M98), which were editing `spec/` while this pass ran. This pass did not edit them.

| Item | Files | Promise IDs | Why not applied |
| --- | --- | --- | --- |
| M01 | `spec/README.md` (three edits) | SEC-024, SEC-006, SEC-013, IFC-041, IFC-007 | Under `spec/`; area F1. |
| M02 | `spec/implementation-defined.yaml`, `spec/README.md` | DL17, SEC-024, IFC-041 | Under `spec/`; area F1. |
| M03 | `spec/implementation-defined.yaml`, `spec/definitions.md` | DL17, PRV-007, TPL-018 | Under `spec/`; area F1. |
| M06 | `spec/definitions.md` | TPL-019, PRV-007 | Under `spec/`; area F1. |
| M07 | `spec/implementation-defined.yaml` | DL16, PRV-003, PRV-012 | Under `spec/`; area F1. |
| M08 | `spec/definitions.md` | PRV-002, PRV-026 | Under `spec/`; area F1. |
| M09 | `spec/definitions.md`, `spec/implementation-defined.yaml`, `spec/README.md` | PRV-026, PRV-002, DL17 | Under `spec/`; area F1. |
| M16 | `spec/README.md` | ACC-027 | Under `spec/`; area F1. |
| M19 | `spec/definitions.md` | PKG-029 | Under `spec/`; area F1. |
| M22 | `spec/definitions.md` | PRV-015, PLC-002 | Under `spec/`; area F1. |
| M90 | `spec/interface/implementation.yaml`, `spec/implementation-defined.yaml` | IFC-042, DL13 | Under `spec/`; area F6, with M47 and M53 of area F4 replacing both edits. |
| M94 | `spec/interface/implementation.yaml` | IFC-037 | Under `spec/`; area F6. |
| M97 | `spec/interface/implementation.yaml`, `spec/implementation-defined.yaml`, `spec/core/README.md` | IFC-041, DL13 | Under `spec/`; area F6. |
| M98 | `spec/core/operations.yaml` | none | Under `spec/`; area F6. |

## Checks

Run from the repository root after the edits:

- The validation of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): 449 promises ok.
- `python3 tools/check_docs.py`: 0 errors before this log was written. At the last run, with
  this log, 167 Markdown files and 2317 local links, and 1 error, not in F7's files:
  `spec/core/README.md` line 118 links `round-5/log-fix-F5.md`, which the pass of area F5 has
  not yet written. It is noted here, not fixed.
- `python3 tools/set_names.py --check`: 0 errors.
- `python3 tools/check_schemas.py`: 0 errors.
- `git diff --check`: clean.

The other areas' passes were editing `spec/` at the same time, so these results are of the tree
as it stood when this pass ran.
