# 007 — The baseline for S1, with today's tools

Status: pre-registered on 2026-11-09, not yet run. This is action item A70 of the
[roadmap](../../ROADMAP.md#experiments-and-probes), which ruling 45 accepted from X3 of pull
request 10, from `allocmemory`. [Ruling 46](../../docs/proposals/ruling-46/README.md) made its
comparison one of the success criteria of the first build. Under ruling 45's rule, this page
states what the experiment must show and how each measure is counted before anything runs.

It runs no Conduit engine, and it is not conformance evidence. It describes no other product's
quality: it measures what one deliberately simple setup, written by hand, costs on S1's tasks.

## Question

What does an author do to set up S1, and then to change and repair it, with the tools people
use today? The tools are a coding agent's own permission settings, its configuration of MCP
servers and, where a shared service must run, a Compose file. The answer is the bar against
which the first build's author measurements are compared (A68).

## What it must show

For each of the tasks T1 to T7, the count of every measure, with the evidence behind each
count. The tasks and the measures are defined once, in the pre-registration of the
[first build](../../docs/implementation/FIRST_BUILD.md): see
[the tasks](../../docs/implementation/FIRST_BUILD.md#the-tasks) and
[the measures](../../docs/implementation/FIRST_BUILD.md#measures). This experiment uses them
unchanged, so that the two sets of results can be compared.

It does not have to show that today's tools are worse. A result in which they cost less is a
result, and the first build is judged against it.

## Method

1. **The setup is fixed before the run, and recorded here before any task is timed.** The agents
   are the two that S1 runs in the build: Claude Code and Codex (ruling 46, point 2), or two
   instances of Claude Code if Codex cannot be used. The directory is served by the reference
   filesystem MCP server, unchanged, which each agent starts from its own configuration. Access
   is limited by each agent's own permission settings and by the directories given to the
   server. The message history is the simplest shared means that both agents can read and
   append to with the same server. A Compose file is used only if a task needs a long-running
   shared service. Every choice made at this step, and its reason, goes in the results.
2. **One author does every task, in order, from a clean machine state**, and keeps a log as it
   goes: each file it creates or changes, each line, each concept it needs and when, each
   mistake and how it was found, and the wall time of each task. Generated files are listed and
   not counted.
3. **Each task is checked by running the agents**, so that a count is made only of a setup that
   works. A failure during a task is part of the result: the steps to recover are counted as
   recovery steps that need a person.
4. **T3, what each agent can reach, is answered from the files alone**, and then checked against
   what the agents can actually do.
5. **The counts are made from the log and the files**, by the definitions in the first build's
   measures, and every count cites its evidence.

## Limits, stated before the run

- The author is an AI agent that knows Conduit's design. The same author does the build's
  tasks, so the comparison is between two setups by one author. Neither result stands for a
  newcomer's effort.
- One run of each task, on one machine. The counts are exact, but the times are observations,
  not measurements of a population.
- Codex is not installed on the machine at pre-registration. It needs the owner to install it
  and sign in. Until then the baseline cannot use it, and the results say which agents were
  used.

## Results

None yet. Results go in this directory with their date, and the
[register](../../ROADMAP.md#experiments-and-probes) records the state.
