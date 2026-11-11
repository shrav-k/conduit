# Ruling 70: Agents run code, permissively for now, so that Conduit can build Conduit

Status: in force; amended by 75 (points 1 and 3: the folder is the grant) (ruled by the owner on 2026-11-11). Holds now: [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach), [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed) (spec pending: the Agent packages 0.2.0, order 050). This record says why.

The first build's Agent Types, `codex.v1` and `claude-code.v1`, turn off what an Agent reaches
through its own login, as [ruling 48](../ruling-48/README.md) point 1 decided. They also turn off
the clients' own machine tools: shell, code execution, browser, hooks and plugins. No ruling asked
for that. The packages chose it because a client's own shell reaches around the engine: the S1
baseline (A70) showed that Codex's read-only sandbox still reads any file its user can. The result
was that an Agent under Conduit could not build or test code. The owner: "Wait they can't run a
shell? Aren't they useless then? Agents that can't actually work or run code are pointless, we
have to solve this now".

The coordinator put three options:

- (a) the clients' own tools on, confined to a directory the template declares, as firmly as the
  runtime allows: fully by a container runtime provider, or for writes only by the client's own
  sandbox;
- (b) a shell offered as a Resource behind a Conduit;
- (c) the tools on with nothing confining them.

The coordinator recommended (a). The owner: "A, and for now we should be overly permissive in order
to test conduit, we can work in more strict sandboxing and isolation later. If we want we can use
flags to disable those features for now in order to test the communication framework and core
concepts. We need to be able to dogfood using conduit to build conduit for multiple reasons. First
to speed up development, and second to refine conduit through real usage".

## The decisions

1. **Agents run code.** An Agent that Conduit starts has its client's own tools: shell, code
   execution, file edits and network access. They are on by default in `codex.v2` and
   `claude-code.v2`. The Agent works in a directory its template names, such as a git worktree.
2. **Permissive for now.** In v0 the tools run as the engine's user, with the client's own sandbox
   and approval prompts off. Nothing confines them. The Type's contract says so, and inspection
   shows an Agent's own tools as reach that does not pass through Conduit (ruling 52).
3. **A flag turns them off.** One template input turns an Agent's own tools off, so that a test can
   exercise the communication framework and the core concepts alone. This is the coordinator's
   reading of "use flags to disable those features": the flag disables the Agent's tools, and
   stricter confinement is not a flag in v0.
4. **Confinement later.** Stricter sandboxing and isolation come later, by the route the
   specification already has: a runtime provider that confines its consumer, chosen through
   `runtime` (ACC-027, PRV-006). An example is a container with only the Agent's directory and
   home mounted. A shell offered as a Resource, option (b), stays possible for tools that Agents
   share.
5. **Conduit builds Conduit.** Dogfooding has two purposes, in the owner's words: "to speed up
   development, and second to refine conduit through real usage". The work is ordered so that
   Conduit runs its own work orders (A78) as early as possible.

## What it amends

Nothing that was ruled. Ruling 48 point 1 and ruling 53 point 7 stand: what an Agent reaches
through its own login (connectors, apps, messaging) stays off by default. The change is to the
first build's Agent packages, from 0.2.0 (order 050). `codex.v1` and `claude-code.v1` stay as they
are.
