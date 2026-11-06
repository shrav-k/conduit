# Experiments

Experiments are evidence. Nothing here binds: the [specification](../spec/README.md) alone states
accepted behavior. No experiment has run a Conduit engine, because none exists.

Direction since 2026-11-05: experiment code is Rust only, and the nine-scenario program in the
[plan audit](../docs/AUDIT.md#experiment-program) replaces Experiments 001 to 003. Each of those
is removed as its replacement lands.

Experiments live outside `docs/`. Each directory keeps its documentation, configuration examples,
expected outcomes and validation record together.

| Experiment | Purpose | Current evidence |
| --- | --- | --- |
| 001 — shared access | Exercised all five families on paper, with static fixtures. | Removed on 2026-11-05: its fixtures used only shapes the rulings replaced. Its files are in the [last commit that held it](https://github.com/shrav-k/conduit/tree/7927d68ce4eea16ba54f45ef92387fc43f7b37fe/experiments/001-shared-access). For current example documents see [spec/schemas/examples/](../spec/schemas/examples/). |
| [002 — runtime and plugins](002-runtime-plugins/README.md) | A plan to test authored implementations, operations implemented by graphs, composed requirements, access over the Model Context Protocol and recovery. Written before the specification, in the words of that time. | A plan only. No code and no measurements. It stays until its cases are mapped onto promises and vectors (action A23 of the plan audit). |
| [003 — language library probes](003-language-probes/README.md) | Check how JSON Schema and YAML libraries behave, before the schema profile was chosen. | 20 offline cases executed and recorded, in Python. No Conduit parser and no provider calls. |
| [004 — Stateright evaluation](004-stateright-evaluation/README.md) | Evaluate a Rust-embedded model checker for checking safety promises under loss, reordering and crashes. | Executed 2026-11-05 in Rust: five rule sets checked, three seeded defects found, one defect found in an accepted rule. A tool evaluation, not evidence about a Conduit engine. |

Keep draft syntax, static checks, synthetic tests and tests against live providers distinct.
Never present an expected outcome as a result. A result that contradicts a promise is raised
with a concrete counterexample; the fix is a change to the specification, which needs an owner
ruling logged in the [plan audit](../docs/AUDIT.md#ruling-log). New experiments follow the same
pattern, in their own directory, and state no rules of their own.
