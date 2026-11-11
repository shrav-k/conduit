# Round 6 of the re-audit

Status: read, decided and applied, 2026-11-07 (plan-audit action A59). The coordinator decided
each finding and each point of the critique, and the owner accepted its recommendations in
[ruling 44](../../ruling-44/README.md). After the ruling the owner also accepted revised
recommendations on D06 and D12, which are applied too ([log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-d06-d12.md)). Pull request 8, on the branch
`audit/round-6-codex`, carries this round's report, its decisions, ruling 44 and the changes
they made.

Round 6 is the owner's cold read by Codex. One reader read the whole repository at `c4d7d33`,
the merge of pull request 7, as [the tracker](../../../AUDIT.md#for-the-next-reader) asked,
and wrote two reports. The round was not clean: two of its six findings were safety-critical.
All six are fixed. The next read by cold readers follows the first build that ruling 44 makes
the next milestone.

| Step | Record |
| --- | --- |
| The six findings, with quotes, situations, counterarguments and least changes | [reader-1.md](reader-1.md) |
| The critique of the design's simplicity and usefulness: 22 points, D01 to D22, the four open points and twenty evidence cases, E01 to E20 | [design-critique.md](design-critique.md) |
| The decisions on every finding, point and open point, with what the independent check found | [DECISIONS.md](DECISIONS.md) |
| The ruling, with what the owner accepted after it | [ruling 44](../../ruling-44/README.md) |
| What each editor changed, its choices and what it left for a decision | The logs beside this record, `log-*.md`, among them [the log of the tracker and the entry points](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-tracker.md) and [the integration's](log-integration.md), which made the groups' files agree, with the [verification](log-integration.md#verification) of the result by two further readers |

| | Round 6 |
| --- | --- |
| Readers | 1, of the whole repository |
| Findings | 6: 3 wrong, 2 of them safety-critical, 2 undetermined, 1 misleading |
| Confirmed by the independent check | 6 |
| Fixed | 6 |
| Points of the critique | 22: 8 accepted and applied, 10 accepted for the experiment program, 2 accepted as revised and applied, 2 with no change |

The counts of promises after the round are in the
[contents table of the specification](../../../../spec/README.md#contents).

## What the reader wrote

The reader's own introduction to the two reports, kept as it was written. "Pending" and
"nothing under `spec/` was changed" describe the report as it was handed over, before the
decisions above.

Started 2026-11-07, on `main` at `c4d7d33`, after the merge of pull request 7, as
[the tracker](../../../AUDIT.md#for-the-next-reader) asks.

The owner subsequently asked for a broader critique of the language's simplicity and usefulness,
with a report and pull request to take back to Claude. This is one reader's report in two parts:

| Part | Scope |
| --- | --- |
| [Findings](reader-1.md) | Six issues for adjudication, with exact quotes, scenarios, counterarguments and least repairs; the tracker's two highlighted MCP questions |
| [Design critique](design-critique.md) | Overall judgment, strengths to preserve, 22 design critiques, concrete simplification alternatives and costs, the four known open points, 20 proposed evidence cases, coverage and handoff |

Start with the design critique's judgment and priority table for the broader review, then use
the findings for the precise defect cases. The design critique distinguishes semantic changes
from improvements to documentation, tooling and experiments. Its proposals are not additional
correctness findings.

The six findings are three wrong (two safety-critical), two undetermined and one misleading.
The expanded adversarial review corrected two earlier `unsafe` labels: those cases show
specific procedures conflicting with general safety promises, not a bad result from an engine
that obeys every promise. Finding 05 has a substantial counterargument and may merit only a
clarification. Neither qualification resolves the outstanding safety cases or certifies a
clean readiness gate.

Three targeted reviewers assisted the expansion and challenged the findings. They shared task
context and do not constitute three independent cold reads. No engine, provider or user trial
was run; the report separates small schema checks and static checks from analytical traces and
proposed experiments.

All findings and recommendations remain pending. Nothing under `spec/` was changed. Decisions
by the owner or coordinator belong beside the report under the existing audit process. This
branch contains only the report; it does not apply its recommendations or update the historical
decision records.

## Choices that set behavior

Copied from the C0 logs, one line per choice, with the source log and item. Adjacent
editorial reasons are kept where they explain the reading. These describe the choices then;
later decisions and promises govern.

- **round-6/log-specdocs, choice 1.** **The history of `spec/definitions.md` moved to its end.** D21 and the work order ask for the current design before history in `spec/README.md` and a table of core terms at the top of `definitions.md`. Ninety lines of history stood at the top of `definitions.md`; the table could not be at the top with them there, so they moved, unchanged, to a new last section, How this file changed, and the head says where they are. No anchor changed.
- **round-6/log-specdocs, choice 2.** **The core terms link sections, not rows.** A Markdown table row has no anchor, and no file in the repository uses HTML anchors, so each line links the section that holds its full row. Six lines name pairs (Interface and Agent; template and instance; outcome and reason; idempotency key and repeat; set up and retire; `management` and `access`) where one line says both.
- **round-6/log-specdocs, choice 3.** **One section on limits in `spec/README.md`.** D03 and D04's statement is a new subsection, What access does not narrow, beside What the guarantee for secrets covers, which became a subsection of the same section, Limits of this version. The anchor of the secrets section is unchanged.
- **round-6/log-specdocs, choice 4.** **The Conformance sentence was changed** to fit TPL-004's hook default, which the record did not name. Without it the README said that no part of an accepted document is ignored.
- **round-6/log-specdocs, choice 5.** **The engine builder's path is the former Reading order, word for word,** so no promise or file moved between the paths of a reader of the Language and the Engine levels.
- **round-6/log-specdocs, choice 6.** **The row is named "bundle (of a tool's schema)"**, because "trust bundle" has its own row and CTR-005 uses "a bundle of limits" in its ordinary sense.
- **round-6/log-specdocs, choice 7.** **The schema profile states pointer comparison for CTR-006 and PRV-016, and cites INS-010 only for the rule that destinations do not overlap.** INS-010 does not say how it compares two destinations; whether it compares by reference tokens too is not settled here (see below).
- **round-6/log-specdocs, choice 8.** **The README's history says that D06 and D12, as revised, are not yet applied, naming A69.** The sentence must change when A69 applies them.
- **round-6/log-specdocs, choice 9.** **IFC-047 was added to the examples of the Implementation level** under Conformance, beside IFC-008, since it binds an implementation and not the engine.
- **round-6/log-tracker, choice 1.** **D06 and D12 are written as settled and not yet applied.** The task said that both wait for the owner on new facts. While this pass ran, the record of ruling 44 gained "After the ruling": the owner answered the revised recommendations with "Agree on both". The tracker follows the record: D06 and D12 are not under "Waiting for the owner", the ruling's row and the later amendments give them as revised, and a new action item, A69, applies them, since this pass's instructions kept every editor off CTR-002's `implementation`, TPL-003, TPL-004 and LIF-031.
- **round-6/log-tracker, choice 2.** **The numbers A68 and A69.** A67 was the last action item in the tracker and its history.
- **round-6/log-tracker, choice 3.** **E15 is placed under S3.** The task named S1, S2, S4, S5 and S10. E15 is a case of partitions and a lost machine, which only S3 runs, as D20 says. D20 is under S2 and S3.
- **round-6/log-tracker, choice 4.** **E18 under S4, E10 and E16 under S5, E17 under S2, E05 under S1.** The critique names no scenario for these; each is placed with the scenario that exercises its mechanism.
- **round-6/log-tracker, choice 5.** **Round 6 is not counted as clean, and readers are asked to mark such a conflict unsafe.** The reader kept "wrong, safety-critical" for 01 and 02 and noted that round 5 used unsafe for such a conflict (M32). DECISIONS.md keeps the reader's labels; the tracker says that round 6 was not clean for G7, and adds to its request to readers that a conflict between a specific procedure and a general safety promise is marked unsafe. That is the stricter of the two readings, and the coordinator can strike it.
- **round-6/log-tracker, choice 6.** **D13 and D17 are "no change".** Ruling 44 does not name them. The critique itself recommends keeping both rules, and puts E16 in the program; D17's author steps are counted by D22's measurements.
- **round-6/log-tracker, choice 7.** **D21 in the entry points links rather than repeats.** `spec/README.md` now has reading paths for the three readers; the root README, the documentation map and the contributor guide link them, as D21 asks for one source of each rule.
- **round-6/log-tracker, choice 8.** **The reader's introduction to round 6 is kept, not rewritten.** It says that everything is pending and that `spec/` was not changed, which was true of the report; a status and a note above it say what came after.
- **round-6/log-tracker, choice 9.** **Round 5's fix log keeps its order.** The check suggested moving the sentence that records V6's settlement after "Recommendation: 1." The pass was to add dated notes only, so a note says the settlement took the recommended option.
- **round-6/log-tracker, choice 10.** **The record's attribution of C42 and X2.** Ruling 44's table cites ruling 39 for both; the check found that ruling 37 accepted them first, and the tracker says so beside the table rather than editing the record, which is not among this pass's files.
- **round-6/log-tracker, choice 11.** **What the check found, written down for readers.** The tracker lists as open the readings of -32602 (IFC-010) and -32020 (IFC-046), which the check named and did not check, and the smaller `isRepeated` gap beside the repeated argument. DECISIONS.md summarizes the check by its verdicts and findings; the check's own results are the coordinator's working notes and are not in the repository.
- **round-6/log-tracker, choice 12.** **Details after the ruling.** DECISIONS.md marks three details that the coordinator fixed in applying the ruling, and lists them for the owner: IFC-047, the obligation on an implementation that carries Conduit's keys, which the check had called the owner's choice; the third case of 6-1-02; and the empty pointer of 6-1-03.
- **round-6/log-tracker, choice 13.** **AGENTS.md says that the first build is not an independent build**, so that an agent asked to work on the reference model is not stopped by the instruction for engine builders.
