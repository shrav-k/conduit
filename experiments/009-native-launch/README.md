# 009 — Native launch and identity

Status: done on 2026-11-09; [dated results](results-2026-11-09.md) record three attempts,
including two failed Codex attempts and the limits of the observations. Pre-registered on
2026-11-09. This is action item A4 of the
[roadmap](../../ROADMAP.md#experiments-and-probes), probe E1 of the plan audit, which
[ruling 48](../../docs/proposals/ruling-48/README.md) runs before increment 3 of
[the first build](../../docs/implementation/FIRST_BUILD.md#what-increment-3-needs). A Codex
worker builds its harness under [work order 010](../../docs/work-orders/010-native-launch-harness.md).
Under ruling 45's rule, this page states what the probe must show before anything runs.

It runs no Conduit engine. It starts each client as the engine's own runner would start an
Agent, against the stub of experiment 008 standing in for the engine's presented server, so
that increment 3 starts from launches that are known to work.

## Question

Can Claude Code and Codex, unchanged, each be run as an Agent is run on the engine's own runner
(PRV-003, PRV-007, PRV-022, PRV-023, TPL-019), with the owner's configuration kept out, the
reach of their logins turned off, a token of their own read from a file, and their access
changed between turns? What does each take?

## What it must show

For each client, with evidence:

1. **The wrapper.** A program, in Rust, that the runner starts with its standard input and
   output as an MCP server (PRV-007), and that runs the client unchanged for each turn it is
   asked for: Claude Code on its streamed input in one process, Codex by resuming its session,
   as experiment 008's follow-up found. What it takes, in files and lines.
2. **The launch.** With only what PRV-022 and PRV-023 give a run (`CONDUIT_URL`,
   `CONDUIT_IDENTITY_DIR` and its files, `HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `USER`)
   and a private, empty working directory (DL16), the client reaches the stub with the token
   from its own identity directory, through its header helper.
3. **The owner's configuration kept out**, by flags alone: none of the owner's settings, MCP
   servers, hooks or instructions loads, shown by what the client reports at its start.
4. **The reach of the login off** (ruling 48, point 1): for Claude Code, no claude.ai connector
   and no `SendMessage`, `ListAgents` or `RemoteTrigger`, and messages from other sessions
   refused; for Codex, no apps. Shown by the tools each client reports, not by its flags.
5. **Two Agents apart.** Two Agents, one of each client, run at once with tokens that name
   different instances; the stub's log attributes every request to the right one (IFC-022).
6. **Access changed between turns**: a Conduit opened and one closed between a first and a
   second turn reach the client's model at the second, as experiment 008 found.
7. **The login**: which authentication each client reports (ruling 48, point 2: the owner's,
   shared). Whether a separate configuration home per Agent can hold its own login is recorded
   as a step that needs the owner, not attempted.

## Method

1. Work order 010 builds the two wrappers and a runner in Rust, in this directory, reusing
   experiment 008's stub, with tests that need no agent.
2. The admin session runs the runner: each wrapper is started as a run, given its identity
   directory, asked for two turns with a change of access between them, and both Agents also
   run together once. Three runs of each.
3. A work order writes the results from the records, each claim citing its run.

## Limits, stated before the run

- One machine, one user: both Agents run as the owner's operating-system user, so each could
  read the other's token (IFC-022); the probe shows attribution, not isolation (A7).
- Headless only, as in experiment 008.
- The stub stands in for the engine; the launch is the runner's, written for the probe, not
  increment 3's.

## Results

[Results of 2026-11-09](results-2026-11-09.md): three attempts at the nine-scenario matrix;
the third completed all twenty-four turns, after changing how Codex's header helper receives
its identity directory. The records show per-request attribution and changed access on the
second turn; the results state the configuration, login and message-refusal properties still
unshown. [Trimmed evidence](evidence-2026-11-09/README.md) accompanies them, and the
[register](../../ROADMAP.md#experiments-and-probes) records the state.
