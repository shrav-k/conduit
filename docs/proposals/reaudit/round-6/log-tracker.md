# Round 6: log of the tracker and the entry points

Date: 2026-11-07. This pass applies [ruling 44](../../ruling-44/README.md) and round 6's
[decisions](DECISIONS.md) to the tracker, the entry points and the records of the re-audit. It
writes the ruling into the [ruling log](../../../AUDIT.md#ruling-log) and its later amendments,
brings "Where things stand", "What happens next", "For the next reader" and "Waiting for the
owner" up to date, adds two action items and updates nine, places round 6's evidence cases and
measurements in the experiment program, fixes the statements that finding 6-1-06 and the
independent check found stale, and writes DECISIONS.md. The verdicts, evidence and risks of the
independent check were read first.

Files: `docs/AUDIT.md`, `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`,
`docs/implementation/INDEPENDENT_BUILDS.md`, `docs/proposals/reaudit/README.md`,
`docs/proposals/reaudit/round-5/README.md`, `docs/proposals/reaudit/round-5/log-fixes.md`,
`docs/proposals/reaudit/round-6/README.md`, `docs/proposals/reaudit/round-6/DECISIONS.md`
(new) and this log. No file under `spec/` was edited, and no promise. Round 5's records keep
their history: each change there is a dated note or a label that describes a later state, and
the heading "For the owner" of round 5's fix log is kept. Nothing was committed or staged.

## Changes

| # | File | Promise IDs | Finding or critique | What changed, and why |
| --- | --- | --- | --- | --- |
| 1 | `docs/AUDIT.md`, identifiers | none | Ruling 44 | The note above the table says how to read a D or an E written with round 6, and names the records of the D, R and Q of rulings 42 and 43, since "the two records before the last" no longer pointed at them once a row was added. A row for 6-1-, D and E of round 6. Round 6's D01 to D22 and E01 to E20 reuse the plan audit's D and E. |
| 2 | `docs/AUDIT.md`, where things stand | none | 6-1-06; ruling 44 | Specification: round 6's decisions and ruling 44 changed it. How it was checked: the 29 findings of round 5's verification are resolved, 28 by fixes and V6 by the coordinator, as option 1, in PLC-007 and PLC-002; round 6 at `c4d7d33`, six findings, two safety-critical, confirmed by the check and fixed; the text it changed is not read cold. Readiness gate: round 6 reported two safety-critical findings, both fixed, and was not clean, so G7 is next judged on the read after the first build. Code: `check_schemas.py` applies 27 rules from 26 promises, with round 6's two, as its run on the working tree reports. |
| 3 | `docs/AUDIT.md`, what happens next | none | Ruling 44, point 1; D22 | Step 4 names `c4d7d33` as the merge of pull request 7, as round 5's H-05 asked and the check found missing. Step 5: round 6 done, decided, accepted by ruling 44, D06 and D12 revised and not yet applied, pull request 8. Step 6: the first build (A68), with the author measurements, its nature to be discussed with the owner first, and the work at step 3 of the program. Step 7: the next read follows the build. |
| 4 | `docs/AUDIT.md`, for the next reader | none | Ruling 44; D21; the four open points; 6-1-01 | Written for any cold reader; the next read follows the build. Which text: `main` from the merge of pull request 8. What has been read: round 6, one reader started by the owner, with helpers that are not cold reads. The gate: round 6 was not clean, and a reader is asked to mark as unsafe a conflict between a procedure and a safety promise. Known to be open: the two limits of delegation that v0 keeps (D03, D04), what a client and a model see, a request without its key, and D06 and D12 not yet applied. The four open points, each with its decision. Still open: -32602 under IFC-010 and -32020 under IFC-046, which the check named. "To look at in particular": round 6 found no further defect. "To look at after round 6" replaces "after round 5". Who decided what: rulings 1 to 44. Where findings go: `round-7/`. |
| 5 | `docs/AUDIT.md`, who decides what | none | Ruling 44 | A paragraph: ruling 44 accepted the coordinator's recommendations on round 6; the details fixed after the ruling and the editors' choices can be struck by the owner. |
| 6 | `docs/AUDIT.md`, waiting for the owner | none | Ruling 44, point 1 | A row for the nature of the first build. A paragraph under the table: D06 and D12 settled as revised ("Agree on both"), applied under A69. |
| 7 | `docs/AUDIT.md`, ruling log | none | Ruling 44 | The dates line names ruling 44 on 2026-11-07. Row 44: the coordinator's summary, the owner's words quoted, what was accepted, and the owner's answer after the ruling on D06 and D12. |
| 8 | `docs/AUDIT.md`, later amendments | none | D14; D06; D12; 6-1-01 | A row for ruling 41, item 16a, changed by D14, as CTR-012 now reads. A row for choice C42 of ruling 37, changed by D06 as revised, not yet applied. A paragraph: X2 and LIF-031 are not changed, since D12 as revised gives an expected ID instead; and the record's table cites ruling 39 for C42 and X2, which ruling 37 accepted first. Under "Changes that no ruling has accepted", a paragraph: ruling 44 reversed V2-15 of round 5, with the promises that changed. |
| 9 | `docs/AUDIT.md`, action items | none | Ruling 44; D01, D09, D15, D16, D19, D20; the four open points | A68, new: the first build. A69, new: apply D06 and D12 as revised. A59 and A61: round 6. A38: the first build starts it, and it measures the state kept per invocation (D19). A39: safety cases first, E01 to E06, and the portable cause of a machine's loss. A63: D15, D16, D01, and the repeated argument in S7. A64: D20. A65: OPS-028 kept for v0, tested by E20, revisited here. A67: D09. A14: D19's projection. |
| 10 | `docs/AUDIT.md`, experiment program | none | Ruling 44; E01 to E20; D01, D02, D05, D08 to D10, D13, D15, D16, D19, D20, D22 | Step 3 of the order is the first build. A new subsection, "Round 6's evidence and measurements", at the end of the section, places each case and measurement under a scenario, with what the specification now requires of it, and says that none has been run. |
| 11 | `README.md` | none | 6-1-06; D21; ruling 44 | The Specification row gives the current design first and links the limits and the reading paths of `spec/README.md`. A new row, "How it got here", carries the history, says that the 29 findings of round 5's verification are resolved, the last, V6, by the coordinator, adds round 6 and ruling 44, and names the next milestone. The reference model row says it is the next milestone. |
| 12 | `AGENTS.md` | none | Ruling 44 | Pull request 8 carries round 6, its decisions and ruling 44; work after it goes on a new branch. The first build is not one of the independent builds, and its nature is discussed with the owner first. |
| 13 | `CONTRIBUTING.md` | none | D21 | The reading order links the reading paths of `spec/README.md`. |
| 14 | `docs/README.md` | none | D21; ruling 44 | The reading paths say that the current design comes first, and three rows link the paths of `spec/README.md` for an author, a writer of contracts and providers, and an implementer. Rows for round 6 and ruling 44 among the records; "Rounds 2 to 6". |
| 15 | `docs/implementation/INDEPENDENT_BUILDS.md` | none | Ruling 44; 6-1-01 | The paragraph of changes since the assessment, and row G2, give 453 promises, 44 of them in `interface/implementation.yaml`, as counted on the working tree, IFC-047 being added. A paragraph on round 6: two safety-critical findings, fixed; G7 still not met; the interface promises on -32021 and on a tool's definition changed (G9); the first build is the language side's and not one of the two builds. The rows keep the assessment as the owner confirmed it, as H-08 of round 5 had it. |
| 16 | `docs/proposals/reaudit/README.md` | none | 6-1-06 | The intro says that the coordinator settled V6, and adds round 6 and its decisions; step 4 adds round 6, which was not clean. The check found this page stale; the reader had not named it. |
| 17 | `docs/proposals/reaudit/round-5/README.md` | none | 6-1-06; 6-1-01 | A dated note after "The next read is the owner's cold read by Codex"; the label of the verification's step says that V6 was settled; a dated note on V2-15's reversal; a dated note after "Known after the round"; dated notes in rows H-05 and H-07. |
| 18 | `docs/proposals/reaudit/round-5/log-fixes.md` | none | 6-1-06; 6-1-01 | Four dated notes: after the verification's intro, that V6 was settled and all 29 are resolved; after its table, that ruling 44 reversed V2-15, and why; after the counts, that V6's row under "Waiting for the owner" was removed; after "For the owner", on the order of V6's sentences and on V2-15. No row and no heading changed. |
| 19 | `docs/proposals/reaudit/round-6/DECISIONS.md` | none | All | New: one row for each finding, each point of the critique and each open point, with the decision and the reason; what the independent check found, with its verdicts; and what the owner may want to read first. |
| 20 | `docs/proposals/reaudit/round-6/README.md` | none | Ruling 44 | A status, the table of the round's records and its counts above the reader's own introduction, which is kept under "What the reader wrote", its first line included. |

## Choices beyond the record

1. **D06 and D12 are written as settled and not yet applied.** The task said that both wait for
   the owner on new facts. While this pass ran, the record of ruling 44 gained "After the
   ruling": the owner answered the revised recommendations with "Agree on both". The tracker
   follows the record: D06 and D12 are not under "Waiting for the owner", the ruling's row and
   the later amendments give them as revised, and a new action item, A69, applies them, since
   this pass's instructions kept every editor off CTR-002's `implementation`, TPL-003, TPL-004
   and LIF-031.
2. **The numbers A68 and A69.** A67 was the last action item in the tracker and its history.
3. **E15 is placed under S3.** The task named S1, S2, S4, S5 and S10. E15 is a case of
   partitions and a lost machine, which only S3 runs, as D20 says. D20 is under S2 and S3.
4. **E18 under S4, E10 and E16 under S5, E17 under S2, E05 under S1.** The critique names no
   scenario for these; each is placed with the scenario that exercises its mechanism.
5. **Round 6 is not counted as clean, and readers are asked to mark such a conflict unsafe.**
   The reader kept "wrong, safety-critical" for 01 and 02 and noted that round 5 used unsafe
   for such a conflict (M32). DECISIONS.md keeps the reader's labels; the tracker says that
   round 6 was not clean for G7, and adds to its request to readers that a conflict between a
   specific procedure and a general safety promise is marked unsafe. That is the stricter of the
   two readings, and the coordinator can strike it.
6. **D13 and D17 are "no change".** Ruling 44 does not name them. The critique itself
   recommends keeping both rules, and puts E16 in the program; D17's author steps are counted
   by D22's measurements.
7. **D21 in the entry points links rather than repeats.** `spec/README.md` now has reading paths
   for the three readers; the root README, the documentation map and the contributor guide link
   them, as D21 asks for one source of each rule.
8. **The reader's introduction to round 6 is kept, not rewritten.** It says that everything is
   pending and that `spec/` was not changed, which was true of the report; a status and a note
   above it say what came after.
9. **Round 5's fix log keeps its order.** The check suggested moving the sentence that records
   V6's settlement after "Recommendation: 1." The pass was to add dated notes only, so a note
   says the settlement took the recommended option.
10. **The record's attribution of C42 and X2.** Ruling 44's table cites ruling 39 for both; the
    check found that ruling 37 accepted them first, and the tracker says so beside the table
    rather than editing the record, which is not among this pass's files.
11. **What the check found, written down for readers.** The tracker lists as open the readings
    of -32602 (IFC-010) and -32020 (IFC-046), which the check named and did not check, and the
    smaller `isRepeated` gap beside the repeated argument. DECISIONS.md summarizes the check by
    its verdicts and findings; the check's own results are the coordinator's working notes and
    are not in the repository.
12. **Details after the ruling.** DECISIONS.md marks three details that the coordinator fixed
    in applying the ruling, and lists them for the owner: IFC-047, the obligation on an
    implementation that carries Conduit's keys, which the check had called the owner's choice;
    the third case of 6-1-02; and the empty pointer of 6-1-03.
13. **AGENTS.md says that the first build is not an independent build**, so that an agent asked
    to work on the reference model is not stopped by the instruction for engine builders.

## Needs a decision or a later pass

- **A69.** D06 and D12, as the owner accepted them revised, are not applied. When they are,
  these places change with them: step 5 of "What happens next", the row for C42 and the
  paragraph on X2 under the later amendments, the bullet under "Known to be open", A59, A69 and
  E12 in the tracker; the rows for D06 and D12 and the status of DECISIONS.md; and the status
  and the counts of round 6's README.
- **IFC-047.** The independent check said that how an implementation carrying Conduit's keys
  is treated was the owner's to choose, between reading its -32021 as a plain server's and a
  new obligation. The coordinator took the obligation as the case that the ruling's words on
  01 name. The owner may want to confirm it.
- **The severity of a conflict between a procedure and a safety promise.** Choice 5 asks
  readers to mark it unsafe; the coordinator or the owner may prefer the reader's label.
- **The counts.** 453 promises, 202 `language`, 207 `convention` and 44 `interface`, and 27
  rules of the tool from 26 promises, were counted on the working tree while the other editors'
  passes were still open. If a later pass adds or removes a promise or a rule, the counts in
  `docs/AUDIT.md` (the Code row) and `docs/implementation/INDEPENDENT_BUILDS.md` change with it.

## Checks

Run from the repository root on the working tree after this pass, with the other editors'
passes in the same tree:

```text
$ /usr/bin/python3 - <<'PY'   # the validation of CONSOLIDATION.md
453 promises ok
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 186 Markdown files and 2602 local links (offline).
Errors: 0
$ python3 tools/set_names.py --check
Names: corePackage x443, apiVersion x262, metaPrefix x103; of `identity`, CONDUIT_URL x19,
CONDUIT_IDENTITY_DIR x10, ca.crt x11, jwks.json x12, engine-id x14, token x18, tls.crt x16,
tls.key x14. Errors: 0
$ python3 tools/check_schemas.py
5. ... Checked 1 controls, the 5 operations that take no `revision`, the 1 that returns one,
the tables of the two READMEs, and 1037 citations of promises in 115 files. ...
6. Applied 27 rules of this tool, which come from 26 promises, to the core package and 3
example directories. 33 sets of documents in 33 files under examples/invalid-by-rule/ were
each rejected by the rule they are named for; each of their 57 documents is valid against
the schemas.
Errors: 0
$ git diff --check
(no output)
```

`git diff --check` does not read untracked files; DECISIONS.md and this log were checked by
hand for trailing whitespace. No failure was caused by another editor's file.
