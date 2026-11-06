# Audit of the documents outside `spec/`

Status: audit of 2026-11-05 (plan-audit action A61), asked for by the owner in ruling 37 so
that the repository can be read cold by an outside reviewer. Two readers reported:

| Reader | Scope | Report |
| --- | --- | --- |
| Entry points and tracker | `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`, the overview, the review queue, the vocabulary map, the plan audit, the defect register, the implementation notes and the experiment indexes | [report](reader-entry-points.md) |
| Older and supporting text | The protocol topics, every proposal record, the research, the archive, the authoring documents and Experiments 001 to 003, each checked against the specification | [report](reader-older-text.md) |

## What they concluded

- Links, checks and hygiene are clean, and the specification and the ruling log agree with each
  other.
- The repository is not ready for a cold read. The front door never sends a reader to `spec/`.
  It presents the overview and the protocol topics, which teach the model the rulings replaced,
  as the accepted reference.
- The tracker's status tables are behind its own ruling log.
- Twelve of fifteen proposal records never say that the specification governs, and several show
  examples that later rulings reversed.
- Nineteen accepted behaviors in the older text are not stated in the specification.

## Decisions taken to fix it

The owner ruled that findings are resolved without a ruling on each (ruling 37). Each decision
below follows a ruling already made; the last column names it.

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

## Left for the owner

- `docs/AUDIT.md` names a private repository among its references, and says that its links fail
  for outside readers. The name was already public on `main` before this audit. Whether to keep
  it is the owner's call.
- The branch `implementation/rust-runtime-plugins` still exists, and the plan audit has a note
  about it.
- The purpose section says "the grant is an object". Since rulings 27 to 29 the object is the
  Conduit. A note now says how to read it; rewording the sentence is the owner's call.
- Whether readiness condition G7 is judged on two clean rounds of the re-audit.
