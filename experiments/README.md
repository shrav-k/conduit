# Experiments

Experiments are evidence. Nothing here binds: the [specification](../spec/README.md) alone states
accepted behavior. The first build's real-engine runs are indexed below with the earlier probes.

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
| 002 — runtime and plugins | A plan to test authored implementations, operations implemented by graphs, composed requirements, access over the Model Context Protocol and recovery, written before the specification. | Removed on 2026-11-06; [archive](../docs/archive/README.md#removed-from-the-tree). |
| 003 — language library probes | Checked how JSON Schema and YAML libraries behave, before the schema profile was chosen. | Removed on 2026-11-06; [archive](../docs/archive/README.md#removed-from-the-tree). |
| [004 — Stateright evaluation](004-stateright-evaluation/README.md) | Evaluate a Rust-embedded model checker for checking safety promises under loss, reordering and crashes. | Executed 2026-11-05; [results](004-stateright-evaluation/README.md#results). |
| [005 — macOS keychain](005-macos-keychain/README.md) | Probe E10 of the plan audit (A22): whether a macOS keychain can be read with no person present, by a trusted, an untrusted and a rebuilt binary, and when locked. | Executed and reviewed 2026-11-06; [dated observations](005-macos-keychain/README.md#results). |
| [007 — the baseline for S1](007-s1-baseline/README.md) | A70: what S1's tasks, set up and then changed and repaired, cost an author with today's tools, as the bar for the first build's author measurements. | Executed 2026-11-09; [results](007-s1-baseline/results-2026-11-09.md). |
| [008 — what a real client shows its model](008-real-client/README.md) | A71: what Claude Code and Codex, unchanged, receive, pass to their models and do next when the engine's presented server answers as IFC-012, IFC-013, IFC-025 and IFC-038 say, through a stub in Rust. | Done 2026-11-09; [results](008-real-client/results-2026-11-09.md) and [later-turn results](008-real-client/results-2026-11-09-later-turn.md). |
| [009 — native launch and identity](009-native-launch/README.md) | A4: whether Claude Code and Codex, unchanged, can each be run as an Agent on the engine's own runner, with the owner's configuration out, their logins' reach off, a token of their own, and access changed between turns. | Done 2026-11-09; [results](009-native-launch/results-2026-11-09.md). |
| [010 — subscription terms](010-subscription-terms/README.md) | A8: whether the engine may run Claude Code and Codex headless as Agents on a subscription. | Done 2026-11-10; [results](010-subscription-terms/results-2026-11-10.md). |
| [011 — short form translation](011-short-form-spike/README.md) | A79: translate S1 in flow and block YAML into today's canonical documents and count authored edits. | Executed 2026-11-10; [results](011-short-form-spike/results-2026-11-10.md). |
| [012 — model checking of the safety promises](012-model-checking/README.md) | A40: the harness for model checks of the safety promises under loss, reordering, partition and a lost machine. | Built 2026-11-11 by work order 104, with a model that tests the harness; [results](012-model-checking/README.md#results). No Conduit promise is checked yet. |
| [013 — S2: one service login shared by two Agents](013-s2-shared-login/README.md) | S2: one service login shared by two Agents with different restrictions. | [Pre-registered](013-s2-shared-login/README.md) on 2026-11-11 by work order 101; awaits the owner's acceptance. |
| [014 — S3: S1 and S2 across two machines](014-s3-two-machines/README.md) | S3: S1 and S2 across two machines, with a partition and a lost machine. | [Pre-registered](014-s3-two-machines/README.md) on 2026-11-11 by work order 101; awaits the owner's acceptance. |
| [015 — S4: a setup that fails partway](015-s4-partial-setup/README.md) | S4: a setup that fails partway, is retried and loses a response. | Pre-registration being written by work order 102; it awaits the owner's acceptance. |
| [016 — S5: a control change under contention](016-s5-control-contention/README.md) | S5: a control change under contention. | Pre-registration being written by work order 102; it awaits the owner's acceptance. |
| [017 — S6: access revoked while requests are in flight](017-s6-revoke-in-flight/README.md) | S6: access revoked and a Resource retired while requests are in flight. | Pre-registration being written by work order 102; it awaits the owner's acceptance. |
| [018 — S7: an outside author's package](018-s7-outside-package/README.md) | S7: an outside author packages an existing MCP server and a small code plugin. | [Pre-registered](018-s7-outside-package/README.md) on 2026-11-11 by work order 103; awaits the owner's acceptance. |
| [019 — S8: search that returns only what the caller may see](019-s8-filtered-search/README.md) | S8: search over storage returning only what the caller may see. | [Pre-registered](019-s8-filtered-search/README.md) on 2026-11-11 by work order 103; awaits the owner's acceptance. |
| [020 — S9: first start, later start and repair](020-s9-start-and-repair/README.md) | S9: first initialization, later start and explicit repair. | Pre-registration being written by work order 102; it awaits the owner's acceptance. |
| [021 — S10: every provider swapped](021-s10-swapped-providers/README.md) | S10: S1 and S2 again with every provider swapped. | [Pre-registered](021-s10-swapped-providers/README.md) on 2026-11-11 by work order 103; awaits the owner's acceptance. |

Keep draft syntax, static checks, synthetic tests and tests against live providers distinct.
Never present an expected outcome as a result. A result that contradicts a promise is raised
with a concrete counterexample; the fix is a change to the specification, which needs an owner
ruling logged in the [plan audit](../docs/AUDIT.md#ruling-log). New experiments follow the same
pattern, in their own directory, and state no rules of their own.

## Evidence for v0

A75's evidence index keeps the limits of each run with its results:

- [004: Stateright evaluation](004-stateright-evaluation/README.md), for A40; bounded tool
  safety, not a check of a Conduit engine.
- [012: model checking](012-model-checking/README.md), for A40 and G5; the harness and its
  self-test of 2026-11-11. No Conduit promise is checked yet.
- [005: keychain observations of 2026-11-06](005-macos-keychain/README.md#results), for A22;
  the daemon case remains open.
- [007: S1 baseline of 2026-11-09](007-s1-baseline/results-2026-11-09.md), for A70 and A68's
  comparison; [counted setup](007-s1-baseline/setup/) and
  [exact prompts](007-s1-baseline/evidence-2026-11-09/prompts.txt).
- [008: real-client results of 2026-11-09](008-real-client/results-2026-11-09.md) and
  [later-turn results](008-real-client/results-2026-11-09-later-turn.md), for A71;
  [method as run](008-real-client/HARNESS.md).
- [009: native-launch results of 2026-11-09](009-native-launch/results-2026-11-09.md), for A4;
  [method as run](009-native-launch/HARNESS.md).
- [010: subscription-terms reading of 2026-11-10](010-subscription-terms/results-2026-11-10.md),
  for A8; to be read again before v0 is published.
- The first build's [increment-3 closeout results](../docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md)
  and increment 4's [measurement plan](../docs/proposals/first-build/increment-4/measurement-plan.md),
  [coordinator report](../docs/proposals/first-build/increment-4/coordinator-run-2026-11-10.md)
  and [fresh-author report](../docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md).
  These real-engine runs stay beside their first-build records. Their raw runs stay until the
  S1 rerun on the simplified language; counted project files and scripts stay as evidence
  afterward.

## Keeping evidence

Results are self-contained; a harness's method and flags as run, and each evidence directory's
README, stay with them; raw run records are committed with their results and leave the tree in
the next condensation, the results linking their last version. Exact prompts and counted
artifacts stay as evidence. The [archive index](../docs/archive/README.md#removed-from-the-tree)
lists removed records and superseded harness code. Future engine work goes to
[engine/](../engine/README.md), and provider work to `packages/`, including
[Claude Code](../packages/agent-claude-code/README.md) and [Codex](../packages/agent-codex/README.md).
