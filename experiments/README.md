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
| [005 — macOS keychain](005-macos-keychain/README.md) | Probe E10 of the plan audit (A22): whether a macOS keychain can be read with no person present, by a trusted, an untrusted and a rebuilt binary, and when locked. | Executed and reviewed 2026-11-06; [dated observations](005-macos-keychain/README.md#results). |
| [007 — the baseline for S1](007-s1-baseline/README.md) | A70: what S1's tasks, set up and then changed and repaired, cost an author with today's tools, as the bar for the first build's author measurements. | Executed 2026-11-09; [results](007-s1-baseline/results-2026-11-09.md). |
| [011 — short form translation](011-short-form-spike/README.md) | A79: translate S1 in flow and block YAML into today's canonical documents and count authored edits. | Executed 2026-11-10; [results](011-short-form-spike/results-2026-11-10.md). |
| [012 — model checking of the safety promises](012-model-checking/README.md) | A40: the harness for model checks of the safety promises under loss, reordering, partition and a lost machine. | Built 2026-11-11 by work order 104, with a model that tests the harness; [results](012-model-checking/README.md#mc1-dated-results-2026-11-11). Models of the safety promises followed: MC2 to MC7b on 2026-11-11 (orders 105 and 107 to 115), MC2's rerun on 2026-11-15 (order 160), and [G5's account](012-model-checking/G5.md) of the convention-layer safety promises (order 116). |
| [013 — S2: one service login shared by two Agents](013-s2-shared-login/README.md) | S2: one service login shared by two Agents with different restrictions. | [Pre-registered](013-s2-shared-login/README.md) on 2026-11-11 by work order 101; accepted by the owner under ruling 75 D. |
| [014 — S3: S1 and S2 across two machines](014-s3-two-machines/README.md) | S3: S1 and S2 across two machines, with a partition and a lost machine. | [Pre-registered](014-s3-two-machines/README.md) on 2026-11-11 by work order 101; accepted by the owner under ruling 75 D. |
| [015 — S4: a setup that fails partway](015-s4-partial-setup/README.md) | S4: a setup that fails partway, is retried and loses a response. | [Pre-registered](015-s4-partial-setup/README.md) on 2026-11-11 by work order 102; accepted by the owner under ruling 75 D. |
| [016 — S5: a control change under contention](016-s5-control-contention/README.md) | S5: a control change under contention. | [Pre-registered](016-s5-control-contention/README.md) on 2026-11-11 by work order 102; accepted by the owner under ruling 75 D. |
| [017 — S6: access revoked while requests are in flight](017-s6-revoke-in-flight/README.md) | S6: access revoked and a Resource retired while requests are in flight. | [Pre-registered](017-s6-revoke-in-flight/README.md) on 2026-11-11 by work order 102; accepted by the owner under ruling 75 D. |
| [018 — S7: an outside author's package](018-s7-outside-package/README.md) | S7: an outside author packages an existing MCP server and a small code plugin. | [Pre-registered](018-s7-outside-package/README.md) on 2026-11-11 by work order 103; accepted by the owner under ruling 75 D. |
| [019 — S8: search that returns only what the caller may see](019-s8-filtered-search/README.md) | S8: search over storage returning only what the caller may see. | [Pre-registered](019-s8-filtered-search/README.md) on 2026-11-11 by work order 103; accepted by the owner under ruling 75 D. |
| [020 — S9: first start, later start and repair](020-s9-start-and-repair/README.md) | S9: first initialization, later start and explicit repair. | [Pre-registered](020-s9-start-and-repair/README.md) on 2026-11-11 by work order 102; accepted by the owner under ruling 75 D. |
| [021 — S10: every provider swapped](021-s10-swapped-providers/README.md) | S10: S1 and S2 again with every provider swapped. | [Pre-registered](021-s10-swapped-providers/README.md) on 2026-11-11 by work order 103; accepted by the owner under ruling 75 D. |
| [022 — provider-contract vectors](022-provider-contracts/README.md) | A63: the harness and the suites that show each provider passing its contract. | Built on 2026-11-11 by work order 123: the harness, with no suite yet. |
| [025 — A73: 1Password with no person present](025-1password/README.md) | A73: whether a 1Password item can be read with no person present, by a service account and through the desktop app. Stage 1 is a desk note from 1Password's documentation; stage 2, a run on a throwaway vault, comes only if a 1Password store is ordered. | Stage 1 read on 2026-11-16 by work order 190; [results](025-1password/results-2026-11-16.md). Stage 2 not registered. |
| [026 — A83: blind verdicts from spec/ alone](026-a83-blind-verdicts/README.md) | A83: whether a reader with only spec/ gives the engine's verdicts, in four parts. Part E's engine run sends the specification's by-rule examples through the engine's public routes. | Part E's engine run on 2026-11-15 by work order 165; [results](026-a83-blind-verdicts/results-2026-11-15.md). The other parts run after the pin. |
| [027 — A5: sign-in and isolation](027-a5-sign-in/README.md) | A5: whether each Agent is signed in to the account it names, through the Interface, in four stages. Stage 0 probes two facts about the real clients that order 050 needs, with nothing signed in. | Stage 0 run on 2026-11-15 and 2026-11-16; [results](027-a5-sign-in/results-2026-11-16.md): Claude Code passes (a) and (b); Codex fails both, its status naming no account and its login opening a browser past the shim. Before stage 1's first step, order 208's refresh-blocked probe ran on 2026-11-19 on the Linux node with a fabricated login, [results](027-a5-sign-in/results-2026-11-19-refresh-probe.md): with Codex 0.167.1's refresh blocked by the override no run wrote the home's auth.json, and every control run wrote. Stage 1's first step ran on 2026-11-19 on the Mac, [results](027-a5-sign-in/results-2026-11-19-1-step-1.md): the Codex check passed, so `codex.v2` states `signIn` as order 050n writes it; the Claude scratch sign-in, outside A5's gate, passed. The rest of stage 1, and stages 2 and 3, not run. |
| [028 — A78: Conduit runs its own orders](028-a78-dogfood/README.md) | A78: work orders run through Conduit's own Agents, each friction logged as a finding of the first build. Stage 1: orders 1 to 3, sent by root. | Order 042 observed on 2026-11-15 by work order 161, from the engine's records of its turns; [results](028-a78-dogfood/results-2026-11-12.md). Orders 2 and 3 [pre-registered](028-a78-dogfood/README.md#orders-2-and-3-registered-before-order-2-runs) on 2026-11-15 by work order 170; not run. |
| [032 — A85: the gateway probe](032-gateway-probe/README.md) | A85: what Claude Code 2.1.318 and Codex 0.167.1 send to a gateway they are pointed at, and where else they try to go, with fake tokens, against a stub gateway in Rust inside a network namespace that holds loopback alone, on the Linux node. | [Pre-registered](032-gateway-probe/README.md) on 2026-11-18 by work order 205, and amended on 2026-11-19 before its harness. Its harness, [order 205b](../docs/work-orders/205b-gateway-probe-harness.md), written, not yet run; its run (order 205c) not yet written. |

Experiments 004, 008, 009, 010 and 023 left the tree under work order 222;
the [archive index](../docs/archive/README.md#removed-from-the-tree) links their last versions.

Keep draft syntax, static checks, synthetic tests and tests against live providers distinct.
Never present an expected outcome as a result. A result that contradicts a promise is raised
with a concrete counterexample; the fix is a change to the specification, which needs an owner
ruling logged in the [plan audit](../docs/AUDIT.md#ruling-log). New experiments follow the same
pattern, in their own directory, and state no rules of their own.

## Evidence for v0

A75's evidence index keeps the limits of each run with its results:

- [004: Stateright evaluation](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/004-stateright-evaluation/README.md), for A40; bounded tool
  safety, not a check of a Conduit engine.
- [012: model checking](012-model-checking/README.md), for A40 and G5; the harness and its
  self-test of 2026-11-11, then models MC2 to MC7b of the safety promises and
  [G5's account](012-model-checking/G5.md). Each model is of the rules the promises state, not
  of an engine's code, and a bounded check shows no unbounded safety.
- [005: keychain observations of 2026-11-06](005-macos-keychain/README.md#results), for A22;
  the daemon case remains open.
- [007: S1 baseline of 2026-11-09](007-s1-baseline/results-2026-11-09.md), for A70 and A68's
  comparison; [counted setup](007-s1-baseline/setup/) and
  [exact prompts](007-s1-baseline/evidence-2026-11-09/prompts.txt).
- [008: real-client results of 2026-11-09](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/008-real-client/results-2026-11-09.md) and
  [later-turn results](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/008-real-client/results-2026-11-09-later-turn.md), for A71;
  [method as run](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/008-real-client/HARNESS.md).
- [009: native-launch results of 2026-11-09](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/009-native-launch/results-2026-11-09.md), for A4;
  [method as run](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/009-native-launch/HARNESS.md).
- [010: subscription-terms reading of 2026-11-10](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/010-subscription-terms/results-2026-11-10.md),
  for A8, with its [second reading of 2026-11-16](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/010-subscription-terms/results-2026-11-16.md)
  for two subscriptions with each vendor; to be read again before v0 is published.
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
the next condensation, the results linking their last version. Exact prompts and counted artifacts stay while their experiment stays;
then git keeps them. The [archive index](../docs/archive/README.md#removed-from-the-tree)
lists removed records and superseded harness code. Future engine work goes to
[engine/](../engine/README.md), and provider work to `packages/`, including
[Claude Code](../packages/agent-claude-code/README.md) and [Codex](../packages/agent-codex/README.md).
