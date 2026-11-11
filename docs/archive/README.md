# Archive

Nothing here binds. The specification is [`spec/`](../../spec/README.md).

**Direction note, 2026-11-05.** The active tracker is the [plan audit](../AUDIT.md).

Historical contributions belong here when their arguments should remain available without
competing with the current specification. Preserve attribution, source revisions and dissent.
What was accepted and still holds is written in the specification, and the rulings are logged
in the [plan audit](../AUDIT.md#ruling-log).

- [Collaborator proposals](collaborator-proposals/README.md) — PR #1 and PR #2, with source links and current disposition.

- [Completed conceptual review](reviews/CONCEPTUAL.md) and [implementation review](reviews/IMPLEMENTATION.md) — accepted ruling summaries, linked to the protocol topics, which were then the active reference.
- [Completed authoring review](reviews/AUTHORING-2026-11-04.md) — preserved migration/owner and behavior/structure rulings merged in PR #3, plus the former cumulative authoring index.
- [Repository framing proposal](reviews/REPOSITORY-2026-11-02.md) — research, accepted boundaries and the historical transition plan; remaining proposals were carried to current review.
- [Documentation migration](migrations/2026-11-protocol-framing.md) — source/destination accounting and reproducible preservation checks.
- [Vocabulary-preparation audit](migrations/2026-11-vocabulary-audit.md) — index cleanup, stable definition references and preservation evidence; no new naming decisions.

There is no active question queue here; the tracker is the [plan audit](../AUDIT.md). The
earlier queue, `docs/REVIEW.md`, is superseded and removed (below). Archiving is not approval or
evidence that a proposed integration was tested. Historical prose retains its original context;
current semantics live in the specification, [`spec/`](../../spec/README.md).

The files listed above are kept as they were written and carry no note of their own. Several
say that the protocol reference is authoritative or active, or that the current review is the
active queue. Each such statement describes the time the file was written. The
protocol topics and the review queue are both superseded, and both are removed (below).

## Removed from the tree

Ruling 39 of the [plan audit](../AUDIT.md#ruling-log) removed the documents below on
2026-11-06. The specification had superseded each of them. Git keeps them: every link to one
now points at commit `e271d1c` on `main`, the last that held it, and the table does the same.

| Path | What it was | Last version |
| --- | --- | --- |
| `docs/proposals/core-and-schemas/{log-documents.md, log-promises.md, APPLY.md}` | Application instructions and editor logs; choices preserved in the record, with the 22 left-undone items checked against their settlements. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas) |
| `docs/proposals/docs-audit/{log-entry-points.md, log-older-text.md, log-gaps.md, reader-entry-points.md, reader-older-text.md}` | Reader reports and editor logs; the DA decisions and GAPS register remain, with editor choices preserved in the record. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit) |
| `docs/protocol/` | The protocol topics: the design as it stood before the specification. Source material for the type contracts still to be written. | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol) |
| `docs/authoring/` | The authoring guide, written for the protocol topics. | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/authoring) |
| `docs/proposals/authoring-syntax/` | A proposed authoring syntax, never ruled, in the shapes the specification replaced. | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax) |
| `docs/REVIEW.md` | The review queue, superseded by the plan audit. | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/REVIEW.md) |
| `docs/VOCABULARY.md` | The map of working terms from before the vocabulary ruling. | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md) |
| `docs/implementation/README.md` | A proposal for the reference implementation, from before the rulings. | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/implementation/README.md) |
| `docs/implementation/ARCHITECTURE.md` | A proposed runtime architecture, from before the rulings. | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/implementation/ARCHITECTURE.md) |
| `docs/implementation/ORGANIZATION.md` | A proposed organization of repositories and delivery, from before the rulings. | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/implementation/ORGANIZATION.md) |
| `experiments/002-runtime-plugins/` | Experiment 002: a plan only, with no code and no measurements. | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/002-runtime-plugins) |
| `experiments/003-language-probes/` | Experiment 003: Python probes of JSON Schema and YAML libraries. | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/003-language-probes) |

Under rulings 57 and 58, work order 060 removed the re-audit working papers below after
preserving their relevant information. Their last version is C0, `f1cb473`. The kept
[re-audit index](../proposals/reaudit/README.md#rounds) holds the round summaries; decision
records retain their original row IDs.

| Path or group | What it was and what it is source material for | Last version |
| --- | --- | --- |
| `docs/proposals/reaudit/CONSOLIDATION.md` | The first consolidation’s instructions and validation. Its promise-writing rules are in CONTRIBUTING.md, “Writing promises”; the form check is in tools/check_schemas.py. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md) |
| `docs/proposals/reaudit/reader-1-duplicates-and-contradictions.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/reader-2-terms-and-shapes.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/reader-3-undetermined-behavior.md` | Round 1’s cold reports; source for the kept decisions and follow-up in the re-audit record. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/) |
| `docs/proposals/reaudit/log-group-a.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-group-b.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-group-c.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-followups-ab.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-followups-c.md` | Round 1’s editor logs; source for the re-audit record’s “Choices that set behavior” and follow-up decisions. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/) |
| `docs/proposals/reaudit/round-2/` | Round 2’s reports, decisions and editor logs; source for the re-audit index and the ledger’s historical R1-/R2-/R3- prefix destinations. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2) |
| `docs/proposals/reaudit/round-3/` | Round 3’s reports, decisions and fix log; source for the re-audit index and the ledger’s historical 3A to 3D prefix destination. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-3) |
| `docs/proposals/reaudit/round-4/` | Round 4’s reports, decisions, fix log and protocol recheck; source for the re-audit index, tracker, cold readers’ protocol guide and ledger’s historical 4A to 4D prefix destination. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4) |
| `docs/proposals/reaudit/round-5/reader-1.md to reader-6.md; https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/handoff-check.md` | Round 5’s cold reports and handoff check; source for its kept decisions and verification, and the cold readers’ protocol guide. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5) |
| `docs/proposals/reaudit/round-5/log-fix-F1.md to https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F7.md` | Round 5’s seven area logs; source for the kept integration and the re-audit record’s “Choices that set behavior”. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5) |
| `docs/proposals/reaudit/round-6/log-d06-d12.md, log-conventions.md, log-core.md, log-interface.md, log-promises.md` | Round 6’s five editor logs; alternatives, readings and costs copied to DECISIONS.md, “For the owner”; open items retained with status in log-integration.md. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6) |
| `docs/proposals/reaudit/round-6/log-specdocs.md, log-tracker.md` | Round 6’s specification-document and tracker logs; source for its README’s “Choices that set behavior”; the pointer-comparison question remains integration item 3. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6) |
| `docs/proposals/ruling-38/check.md`, `docs/proposals/ruling-38/handoff-check.md`, `docs/proposals/ruling-38/log-spec.md` | Checks and the specification editor’s log; their changes are recorded in ruling 38. [Record](../proposals/ruling-38/README.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-38) |
| `docs/proposals/ruling-41/PLAN.md`, `docs/proposals/ruling-41/log-G1.md`, `docs/proposals/ruling-41/log-G2.md`, `docs/proposals/ruling-41/log-G3.md`, `docs/proposals/ruling-41/log-G4.md`, `docs/proposals/ruling-41/log-G5.md`, `docs/proposals/ruling-41/log-G6.md`, `docs/proposals/ruling-41/log-G7.md`, `docs/proposals/ruling-41/log-core.md`, `docs/proposals/ruling-41/log-docs.md`, `docs/proposals/ruling-41/log-integration.md` | The rewrite plan and ten editor logs; P1 to P20 and editor choices are preserved in the ruling record, and open decisions remain in place. [Record](../proposals/ruling-41/README.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41) |
| `docs/proposals/ruling-42/log-conventions.md`, `docs/proposals/ruling-42/log-core.md`, `docs/proposals/ruling-42/log-docs.md`, `docs/proposals/ruling-42/log-editorial.md`, `docs/proposals/ruling-42/log-integration.md`, `docs/proposals/ruling-42/log-language.md`, `docs/proposals/ruling-42/log-machines.md`, `docs/proposals/ruling-42/log-rename-a.md`, `docs/proposals/ruling-42/log-rename-b.md`, `docs/proposals/ruling-42/log-secrets.md` | Ten editor logs; their choices are preserved in the ruling record. [Record](../proposals/ruling-42/README.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42) |
| `docs/proposals/ruling-43/log-core.md`, `docs/proposals/ruling-43/log-docs.md`, `docs/proposals/ruling-43/log-integration.md`, `docs/proposals/ruling-43/log-interface.md`, `docs/proposals/ruling-43/log-providers.md`, `docs/proposals/ruling-43/log-secrets.md` | Six editor logs; their choices are preserved in the ruling record, and MX-21 in the ledger’s appendix. [Record](../proposals/ruling-43/README.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43) |

Under rulings 57 and 58, work order 063 condensed the closed first-build increments after
extracting their findings, evidence and runnable open-check method. Increment 4’s log and
raw runs remain live. Last versions below are C0, `f1cb473`.

| Path or group | What it was and what it is source material for | Last version |
| --- | --- | --- |
| `docs/proposals/first-build/log-increment-1.md`, `docs/proposals/first-build/log-increment-2.md`, `docs/proposals/first-build/log-increment-3.md` | The three closed increment logs: 33 findings in the [settlements register](../proposals/first-build/README.md#settlements-register); D19 and open loader remainders there; lean-vector and mutation evidence in [conformance](../../conformance/README.md#planned-work). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-1.md), [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md), [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-3.md) |
| `docs/proposals/first-build/check-plain-errors.md` | Tagged protocol/SDK check; conclusion, provenance and rejected alternatives in [decision 8a’s evidence](../proposals/first-build/increment-2/decisions.md#evidence-for-decision-8a). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/check-plain-errors.md) |
| `docs/proposals/first-build/increment-3-handoff.md` | Historical increment-2 boundary and shell handoff; today’s [core boundary](../../engine/README.md#the-cores-boundary) is rebuilt from code, with coverage debt in conformance. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3-handoff.md) |
| `docs/proposals/first-build/increment-3/closeout.md` | The real-client closeout plan; its sentinel method and remaining checks are in [How to run the open checks](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#how-to-run-the-open-checks). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3/closeout.md) |
| `docs/proposals/first-build/increment-2/plan.md` | The study: frozen [promise inventory](../../conformance/s1-promises.yaml), [proposed safety traces](../../conformance/s1-safety-cases.md), admin decisions, portable runner/code and planned coverage work carry its relevant information. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md) |
| `docs/proposals/first-build/increment-2/s1/` | Nine illustrative YAML draft files; maintained fixtures and the [live S1 example](../../engine/examples/s1/README.md) already replaced them. No archive/install or runtime result was claimed by the drafts. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/s1) |
