# Round 6 of the re-audit

Status: read, decided and applied, 2026-11-07 (plan-audit action A59). The coordinator decided
each finding and each point of the critique, and the owner accepted its recommendations in
[ruling 44](../../ruling-44/README.md). After the ruling the owner also accepted revised
recommendations on D06 and D12, which are not yet applied (A69 of the
[tracker](../../../AUDIT.md#action-items)). Pull request 8, on the branch
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
| What each editor changed, its choices and what it left for a decision | The logs beside this record, `log-*.md`, among them [the log of the tracker and the entry points](log-tracker.md) and [the integration's](log-integration.md), which made the groups' files agree |

| | Round 6 |
| --- | --- |
| Readers | 1, of the whole repository |
| Findings | 6: 3 wrong, 2 of them safety-critical, 2 undetermined, 1 misleading |
| Confirmed by the independent check | 6 |
| Fixed | 6 |
| Points of the critique | 22: 8 accepted and applied, 10 accepted for the experiment program, 2 accepted as revised and not yet applied, 2 with no change |

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
