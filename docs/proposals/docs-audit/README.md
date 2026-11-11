# Audit of the documents outside `spec/`

Status: audit of 2026-11-05 (plan-audit action A61), asked for by the owner in ruling 37 so
that the repository can be read cold by an outside reviewer. The thirteen decisions below are
applied; [Applied](#applied) links the logs. What the readers concluded describes the
repository as it stood before that. Two readers reported:

| Reader | Scope | Report |
| --- | --- | --- |
| Entry points and tracker | `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`, the overview, the review queue, the vocabulary map, the plan audit, the defect register, the implementation notes and the experiment indexes | [report](reader-entry-points.md) |
| Older and supporting text | The protocol topics, every proposal record, the research, the archive, the authoring documents and Experiments 001 to 003, each checked against the specification | [report](reader-older-text.md) |

## What they concluded

This is the state the two readers found on 2026-11-05, before the decisions were applied. It is
not the state now.

- Links, checks and hygiene were clean, and the specification and the ruling log agreed with
  each other.
- The repository was not ready for a cold read. The front door never sent a reader to `spec/`.
  It presented the overview and the protocol topics, which teach the model the rulings
  replaced, as the accepted reference.
- The tracker's status tables were behind its own ruling log.
- Twelve of fifteen proposal records never said that the specification governs, and several
  showed examples that later rulings reversed.
- Nineteen accepted behaviors in the older text were not stated in the specification.

## Decisions taken to fix it

The owner ruled that findings are resolved without a ruling on each (ruling 37). Each decision
below follows a ruling already made; the last column names it. Each is the coordinator's and
not the owner's, and the owner can strike any of them.

Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

| ID | Decision | Follows |
| --- | --- | --- |
| DA1 | The specification alone binds. Nothing under `docs/protocol/` binds; it is source material for the type contracts that are still to be written. A behavior missing from `spec/` is a gap in `spec/`. | Decision NS3 of the form of the specification (ruling 17): a converted topic is replaced by a pointer. Every area is now converted. |
| DA2 | The protocol topics keep their paths, because about 200 links point into them, and each gets a banner that says it is superseded. `docs/protocol/README.md` becomes a short pointer page. | DA1 |
| DA3 | The nineteen accepted behaviors that the specification does not state are converted into promises, or put on the implementation-defined list, in a pass of their own after the editors applying ruling 37 have finished. Six behaviors that appear to have been dropped are listed in the plan audit for the owner. | Accepted text is converted, not re-decided. |
| DA4 | Experiment 001 is removed. Its fixtures use only retired shapes, and `spec/schemas/examples/` is now the worked example. Links to it point at the last commit that held it, `7927d68`. Experiments 002 and 003 stay. | Ruling 9: old experiments go as replacements land. Ruling 36 for the pinned links. |
| DA5 | Every research packet and the archive's index get one line under the title saying that nothing there binds and that it predates the specification. Nothing else in them is edited. | The practice `AGENTS.md` already describes, of a note under a title. |
| DA6 | `docs/authoring/` and the authoring-syntax proposal get the same line on every file. They are not moved. | DA1 |
| DA7 | `docs/OVERVIEW.md` is rewritten from the specification. | It is the second document a reader is sent to, and it teaches the replaced model. |
| DA8 | Every proposal record says, under its status, that it is the record of a ruling, that the specification governs where they differ, and which later rulings amended it. Examples and rules that later rulings reversed get a note beside them. | — |
| DA9 | The plan audit is brought up to ruling 37: its statuses, its counts and the order of its sections. The ruling log's rows are not rewritten; a note under the log lists which rulings later ones amended. Sections that record the audit as it was first made are marked historical. | — |
| DA10 | One reading order everywhere: `README.md`, `spec/README.md`, `spec/definitions.md`, then the plan audit for the state of the work and the rulings. | — |
| DA11 | One description of Conduit everywhere, the specification's own: a configuration language for the relationships between independent agents and the things they share. | Ruling 14, the layered claim. |
| DA12 | The same list of four checks in `AGENTS.md` and `CONTRIBUTING.md`. | — |
| DA13 | The build brief asks each builder for a runtime profile, uses "documents" and "contents list" for what it called declarations and the manifest, says that `tools/` ships on the build base, and says that its tags are revisions of version `v0`. | The Engine conformance level in `spec/README.md`; the definitions. |

## Applied

All thirteen decisions are applied. Each log lists the edits made, file by file.

| Log | Decisions | What it covers |
| --- | --- | --- |
| [Entry points and tracker](log-entry-points.md) | DA1 to DA4, DA7 and DA9 to DA13 | `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`, the overview, the plan audit, the defect register, the build documents and the experiment index |
| [Older and supporting text](log-older-text.md) | DA1 to DA6 and DA8 | The protocol topics, the proposal records, the research packets, the archive's index, the authoring documents and the experiments |
| [Conversion list](GAPS.md) and its [log](log-gaps.md) | DA3 | The nineteen behaviors that the older text accepted and the specification did not state. Eighteen are converted: the specification went from 376 promises to 395, and the implementation-defined list from 13 items to 14. The nineteenth, on secrets (GP3), is left for the owner |

A later reader read the whole repository cold, as the fourth reader of round 2 of the
re-audit. Its [report](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2/reader-4.md) found the entry points consistent and
the tracker two commits behind, and the [log of the fixes](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2/log-reader-4-fixes.md)
says what was done. The fourth readers of rounds 3 and 4 read it again:
[round 3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-3/reader-4.md), 23 findings, and
[round 4](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/reader-4.md), 26.

## Left for the owner

- `docs/AUDIT.md` names a private repository among its references, and says that its links fail
  for outside readers. The name was already public on `main` before this audit. Whether to keep
  it is the owner's call.
- The branch `implementation/rust-runtime-plugins` still exists, and the plan audit has a note
  about it.
- The purpose section says "the grant is an object". Since rulings 27 to 29 the object is the
  Conduit. A note now says how to read it; rewording the sentence is the owner's call.
- Whether readiness condition G7 is judged on two clean rounds of the re-audit.
