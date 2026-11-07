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
