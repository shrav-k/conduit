# Ruling 45: a collaborator's two proposals, and the roadmap

Status: in force (ruled by the owner on 2026-11-09). Holds now: [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement), [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

On 2026-11-08 the collaborator `allocmemory` opened two draft pull requests against `main` at
`2ca84f3`:

- [pull request 9](https://github.com/shrav-k/conduit/pull/9), a plain definition of Conduit at
  the head of the README and a one-page list of its components;
- [pull request 10](https://github.com/shrav-k/conduit/pull/10), a roadmap of experiments toward
  a first runnable slice, which it called v0.0.1.

The coordinator reviewed both against the specification, the rulings and the experiment
program. Four reviewers each read with one lens, and every finding went to a second agent that
tried to refute it. None was refuted, and some were narrowed. The coordinator then put five
points to the owner, each with a recommendation.

The owner's answer: "Let's do as you recommend, but it seems from the overlap our roadmap for
the experiments was not easily discoverable. After you merge these two do a pass over the docs
and decide the cleanest way to display the roadmap".

## What was found

- **Pull request 9.**
  - It replaced the one description of Conduit that DA11 of the
    [documents audit](../docs-audit/README.md) fixes everywhere, the specification's own, in
    the README only.
  - Its plain wording promised more than this version gives. "Exactly what each agent can
    reach": every restriction is mediated in this version (ACC-027). "Agents use services
    without ever seeing the secrets": the root Interface may open an Agent a Conduit to a secret
    Resource (SEC-008 to SEC-010).
  - Its components page restated rules in its own words beside the core-terms glossary that
    ruling 44 (D21) had just added, and had already drifted from the specification. It said a
    gate stands before every server the engine does not run itself, leaving out a `plain` server
    reached by `remote`, and that a secret is passed "never to an Agent".
- **Pull request 10.**
  - It put the owner's discussion of the first build after two rounds of experiments, which
    ruling 44 put next. It never scheduled the conformance vectors or the reference model.
  - Its starting scope had one Agent, where S1 has two. It reached the Agent over stdio, where
    an Agent reaches the engine's address with a token (IFC-042). It kept a secret that no
    server in the slice used.
  - Its five experiments, X1 to X5, each duplicated an item already in the
    [experiment program](../../AUDIT.md#experiment-program) or the action items, in whole or in
    part. The overlap is what the owner took as a sign that the roadmap is hard to find.

## The points and the answers

| # | Point | Answer |
| --- | --- | --- |
| 1 | The description of Conduit | **a**: the specification's sentence stays first, and one concrete sentence from pull request 9 follows it, corrected to what this version gives. No change to DA11. |
| 2 | The components page | **a**: not kept. What it named and the glossary lacked becomes six rows of the core terms in `spec/definitions.md`: project, root Interface, gate, node, manifest and core package. |
| 3 | Pull request 10 | **a**: closed. What is kept goes into the experiment program and the action items, credited to `allocmemory`. |
| 4 | Its experiments | As recommended. X3 and X5 run now, X4 is the first piece of the build, X1 runs later, and X2 is dropped. |
| 5 | Who does the work | **a**: the coordinator revises pull request 9 on its branch and merges it, credited to its author. |

On point 4:

- **X3, the baseline for S1, is kept and runs now (A70).** S1's tasks are done with the tools
  people use today, before the tracer. It needs no engine, and it gives the first build a bar
  to beat. It counts what the S1 row of the program counts, on the setup and on the
  change-and-repair tasks of D08. In pull request 10 it was set beside D08: D08 is the chapter
  on change and repair, and the baseline is the separate comparison of round 6's critique, in
  the S1 row.
- **X5, what a real client shows its model, is kept and runs now (A71).** It is an early run of
  E17 against a stub of the engine's presented server. It needs no engine. "Gated" and "over
  stdio" are dropped: no gate stands before what the engine reaches over stdio, and an Agent
  reaches the engine's address with a token.
- **X4 is the first piece of the build (A43).** It is restated, because a schema validator
  cannot match `tools/check_schemas.py` exactly: the documents under
  `spec/schemas/examples/invalid-by-rule/` are valid against the schemas by design.
- **X1, a probe of 1Password with no person present, runs later (A63).** It runs beside the
  build, under the safety rules of Experiment 005.
- **X2, an author's effort counted on paper, is dropped.** The first build takes the author
  measurements of D22 on every run (A68), and with no engine there is nothing to run the
  documents on.

The scope that pull request 10 proposed for v0.0.1 is an input to the discussion of the first
build (A68), with the corrections above. The name "v0.0.1" is not kept, since the repository
has no release scheme.

Accepted as a rule of the experiment program: each experiment states, before it runs, what it
must show and how each of its measures is counted. Pull request 10 proposed it, and round 6's
D16 asked it of the comparison for S10.

## What follows

1. Pull request 9 was revised as answered and merged by squashing on 2026-11-09, as `0e937ee`.
2. Pull request 10 is closed, with a comment that says where each kept part went.
3. A70 and A71 are new action items, and A43 and A63 note what this ruling adds to them.
4. **The roadmap.** The coordinator makes a pass over the documents and decides the cleanest way
   to show the roadmap, so that a reader finds the order of the work and each experiment's state
   without reading the tracker (A72).
