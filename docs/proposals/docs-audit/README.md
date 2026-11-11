# Audit of the documents outside `spec/`

Status: audit of 2026-11-05 (plan-audit action A61), asked for by the owner in ruling 37 so
that the repository can be read cold by an outside reviewer. The thirteen decisions below are
applied; [Applied](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/README.md#applied) links the logs. What the readers concluded describes the
repository as it stood before that. Two readers reported:

| Reader | Scope | Report |
| --- | --- | --- |
| Entry points and tracker | `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`, the overview, the review queue, the vocabulary map, the plan audit, the defect register, the implementation notes and the experiment indexes | [report](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-entry-points.md) |
| Older and supporting text | The protocol topics, every proposal record, the research, the archive, the authoring documents and Experiments 001 to 003, each checked against the specification | [report](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md) |

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


## Choices that set behavior

### Entry-point log

Source: [log-entry-points](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/log-entry-points.md#findings-not-carried-out-and-why).

| Finding | Why |
| --- | --- |
| [D6, moving the historical sections to the archive](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-entry-points.md) | The smaller fix was taken: a banner on each. |

### Older-text log

Source: [log-older-text](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/log-older-text.md#findings-not-carried-out-and-why).

| Finding | Why |
| --- | --- |
| [A4 to A10](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md): the contradicted sentences inside the topic files; [A8](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md) "reduce `POOLS.md`"; [A9](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md) "strike `RESOURCES.md` :28-51"; [A10](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md) "retitle `AGENT.md`", and its two references to sections "below" that are in other files | Decision DA2 chose option (a) of [A11](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md): a banner only, with bodies and headings untouched. The banner says what each file is kept for. |
| [A11 option (b)](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md): archive four topics | Not chosen (DA2). |
| [C2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md), for `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONFIGURATION.md`, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONSTRUCTION.md`, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/RUST_RUNTIME.md` and `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/PI_RUNTIME.md` | The wrong sentence is part of the opening paragraph, not a separate banner. The one line is added above it and says how to read it; DA5 allows nothing else to be edited. |
| [C5, C6, D4](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md): archive or move `docs/authoring/` and the authoring-syntax folder | DA6: a banner on each file, and they are not moved. |
| [C7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md): the labels "SPEC.md" and "DESIGN.md" in research | DA5: one line only. The line says what the labels mean. |
| [C8](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md): research sentences that call Experiment 001 "the current YAML packet" | DA5. The links are repointed and the one line says how to read "current". |

### Gaps log

Source: [log-gaps](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/log-gaps.md#choices-made-that-gapsmd-does-not-state).

| Choice | Why |
| --- | --- |
| GP8, GP9 and GP10 are in `graphs.yaml`, with the layer `convention`. GP7 and GP19 are in `operations.yaml` | Their subject is a step and a graph. The two invariants of a graph are already there with that layer |
| GP2 is two promises, ACC-036 and ACC-037 | One obligation to a promise, and the second names a reason |
| GP6 is two promises, SCH-020 and SCH-021 | The same |
| GP15's convention half is in `access.yaml`, beside ACC-027, which is the other thing an inspected Conduit reports | — |
| SCH-019 covers an input's `default` against the input's `schema`, as well as the keyword `default` inside a schema | In this specification an input states its default beside its schema, not inside it. The row's "its template registered" fits only that reading |
| An input of an Operation that states a `default` may be left out whether or not it is marked `optional: true` (OPS-026, GRA-007) | Otherwise GRA-007 would reject the step that the default is for. INS-004 already reads a template's input so |
| A constituent's value reference may add `pointer` | GRA-020 allows it for the source `input`, and the row says value bindings |
| A constituent has its own form inside `requirement-spec.v1`, and no new Schema contract | A new contract would be a new export of the core package |
| No core contract states `obliges`, `restoresRuntimes` or `records`, and no core input states a `default` | No promise says what obliging an end requires of it, so no value for `bound.v1` or `shared-host.v1` could be checked. No core type declares a runtime |
| ACC-037 cites DL4 for whether the engine waits | DL4 already leaves that to the engine, and the two then agree |
| The Implementation level names four promises as examples and does not list them all | Which promises bind an implementation follows from each promise's own wording |
