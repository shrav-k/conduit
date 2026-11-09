# Experiments

Experiments are evidence. Nothing here binds: the [specification](../spec/README.md) alone states
accepted behavior. No experiment has run a Conduit engine, because none exists.

Every experiment and probe, planned, running, done or dropped, has one row in the
[register](../ROADMAP.md#experiments-and-probes) of the roadmap, with its state and what it
feeds. This index holds those that have a directory here. Check the register before proposing
an experiment, and follow [the rules for proposing one](../ROADMAP.md#proposing-an-experiment),
which also say how a directory is numbered.

Direction since 2026-11-05: experiment code is Rust only, and the program of scenarios S1 to
S10 in the [roadmap](../ROADMAP.md#scenarios) replaces Experiments 001 to 003, which are
removed. A provider package is not an experiment: it goes under a top-level `packages/`
directory, one crate per provider (ruling 40).

Experiments live outside `docs/`. Each directory keeps its documentation, configuration examples,
expected outcomes and validation record together.

| Experiment | Purpose | Current evidence |
| --- | --- | --- |
| 001 — shared access | Exercised all five families on paper, with static fixtures. | Removed on 2026-11-05: its fixtures used only shapes the rulings replaced. Its files are in the [last commit that held it](https://github.com/shrav-k/conduit/tree/de6566ad8a2ea408876a66ab04930734c7b334de/experiments/001-shared-access). For current example documents see [spec/schemas/examples/](../spec/schemas/examples/). |
| 002 — runtime and plugins | A plan to test authored implementations, operations implemented by graphs, composed requirements, access over the Model Context Protocol and recovery, written before the specification. | Removed by ruling 39: a plan only, with no code and no measurements, in shapes the specification replaced. Its [last version](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/002-runtime-plugins/README.md). |
| 003 — language library probes | Checked how JSON Schema and YAML libraries behave, before the schema profile was chosen. | Removed by ruling 39: 20 offline cases in Python, of libraries and not of Conduit; the schema profile now settles what they probed. Its [last version](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/003-language-probes). |
| [004 — Stateright evaluation](004-stateright-evaluation/README.md) | Evaluate a Rust-embedded model checker for checking safety promises under loss, reordering and crashes. | Executed 2026-11-05 in Rust: five rule sets checked, three seeded defects found, one defect found in an accepted rule. A tool evaluation, not evidence about a Conduit engine. |
| [005 — macOS keychain](005-macos-keychain/README.md) | Probe E10 of the plan audit (A22): whether a macOS keychain can be read with no person present, by a trusted, an untrusted and a rebuilt binary, and when locked. | Executed 2026-11-06 in Rust, on two throwaway keychains, never the login keychain. A binary on an item's lists read it silently in under 6 ms; an unlisted binary, a rebuilt binary and a locked keychain were refused at once, with errSecAuthFailed. In the login keychain's format, an item added with the `security` tool was refused to an ad-hoc-signed binary until its partition list was changed with the keychain's password. A review ran it again the same day, with the same outcome in every case. A daemon was not tested. |
| [007 — the baseline for S1](007-s1-baseline/README.md) | A70: what S1's tasks, set up and then changed and repaired, cost an author with today's tools, as the bar for the first build's author measurements. | Executed 2026-11-09 with Claude Code and Codex: T1 to T7 done, checked by running both agents and counted, in its [results](007-s1-baseline/results-2026-11-09.md). Setup took 49 lines in 7 files and 13 concepts. Each later change of access was one file per agent. The upgrade gave the read-only agent two new write tools until the server's `--read-only` replaced a list of tool names. No engine ran. |
| [008 — what a real client shows its model](008-real-client/README.md) | A71: what Claude Code and Codex, unchanged, receive, pass to their models and do next when the engine's presented server answers as IFC-012, IFC-013, IFC-025 and IFC-038 say, through a stub in Rust. | Done on 2026-11-09: [dated results](008-real-client/results-2026-11-09.md), 68 runs accounted for; the [later-turn follow-up](008-real-client/results-2026-11-09-later-turn.md) ran 12 two-turn sessions that day; follow-up worker classifications await admin review. |
| [009 — native launch and identity](009-native-launch/README.md) | A4: whether Claude Code and Codex, unchanged, can each be run as an Agent on the engine's own runner, with the owner's configuration out, their logins' reach off, a token of their own, and access changed between turns. | Pre-registered 2026-11-09; its harness is built by work order 010; not yet run. |

Keep draft syntax, static checks, synthetic tests and tests against live providers distinct.
Never present an expected outcome as a result. A result that contradicts a promise is raised
with a concrete counterexample; the fix is a change to the specification, which needs an owner
ruling logged in the [plan audit](../docs/AUDIT.md#ruling-log). New experiments follow the same
pattern, in their own directory, and state no rules of their own.
