# The first build

Not binding. The specification in [spec/](../../spec/README.md) alone binds. This page is the
pre-registration of the first build (A68), which ruling 44 made the next milestone and
[ruling 46](../proposals/ruling-46/README.md) shaped. Under ruling 45's rule it states, before
any part runs, what the build must show and how each measure is counted. Where the build stands
is in step 3 of the [roadmap](../../ROADMAP.md#the-order-of-the-work), not here.

## What it is

One codebase in Rust, under `reference-model/`, in two parts (ruling 46, point 1):

- **The core, the reference model (A38).** A pure, deterministic library. Every decision it
  makes cites the promise that requires it, and a case that no promise decides is a hole in the
  language, logged as a finding. The conformance vectors drive it (A39). They are kept under
  `conformance/`, safety cases first.
- **The shell.** It makes the core runnable on one machine. It runs servers on the engine's own
  runner (PRV-003). It serves each consumer the tools of its open Conduits at the engine's
  address, with a token (IFC-012, IFC-042, DL13). It keeps the records (REC-001). The shell is
  the first slice of the reference engine (A50), and S2 to S9 later run on the same codebase.

S1's providers are the engine's built-ins: its own runner, the local machine and fetching from
a local directory (ruling 46, point 5). Its real Agents are Claude Code and Codex, or two
instances of Claude Code if one of them cannot reach an MCP server over HTTP with a token
(point 2). The build uses existing crates, among them the official Rust SDK of the Model
Context Protocol and an existing JSON Schema validator.

## The theses, and what would change the language

| Thesis | What in the build tests it | If it fails |
| --- | --- | --- |
| Declaring relationships costs an author less than configuring each agent | S1's author measurements against the baseline for S1 (A70), on the setup and on the tasks of change and repair | Simplify the documents: fewer concepts, better defaults. A68 records what each simplification proposed by round 6 would have saved |
| The language fully determines what a set of documents means | The core cites a promise for every decision | Each hole, ambiguity or contradiction is a finding, fixed in the promises with a vector |
| Unchanged MCP clients work behind Conduit | The probe of a real client (A71), then Claude Code and Codex in S1 | Change what the engine presents to a consumer (the implementation interface) |
| Access belongs to the relationship, not to the agent | Two Conduits to one directory, opened and closed independently | A finding against the Conduit promises |
| The safety promises hold | The safety vectors pass; model checking follows in step 6 | A defect in the promises or in the core, logged as a finding |

## Increments

Each increment is a branch and a pull request, merged by squashing once its checks pass. Each
logs its findings (below). Since [ruling 48](../proposals/ruling-48/README.md), Codex workers
carry out the increments as [work orders](WORK_ORDERS.md) that the coordinator writes, and the
coordinator reviews and merges them.

| # | Increment | Done when |
| --- | --- | --- |
| 1 | **The strict loader (A43).** It reads a document in YAML or JSON, strictly (SCH-012). It reads a project directory as MAN-001, MAN-011 and SUB-010 say. It validates each document against the schemas under `spec/schemas/`, after bundling the Schema contracts as `tools/check_schemas.py` bundles them. Formats are asserted (SCH-001, SCH-004), and patterns are ECMA-262 (SCH-016). | Every document under `spec/schemas/examples/valid/` is accepted. Every file under `invalid/` is rejected, by the step its name gives: when read, or by a schema. Every document under `invalid-by-rule/` passes the schemas, since its rule comes with the core. The verdicts agree with the schema steps of `check_schemas.py`. SCH-004's uuid grammar and SCH-016's patterns pass cases written for them. `cargo test --release` passes. |
| 2 | **The core on S1 (A38, A39).** First start (FST-001 and after), the registration and resolving of S1's documents, setup and the opening of declared Conduits (CON-004), the access decision (ACC-001), how a request ends (OPS-001), and records. Safety vectors for the cases that S1 touches, E05 of round 6 first. | S1's documents register and resolve on the core. Every decision cites a promise. The safety vectors pass. Each case the core cannot decide from the promises is a logged finding. |
| 3 | **The shell.** The engine's own runner starts a `plain` filesystem MCP server, unchanged. The presented server answers at a loopback `CONDUIT_URL` with tokens. Two real Agents share one directory: one through a Conduit that lists reads, one through a Conduit that lists reads and writes. It meets [what increment 3 needs](#what-increment-3-needs). A4 runs before it. | The engine serves each Agent exactly the tools of its open Conduits, shown by a vector (IFC-012), and each client's own list is recorded apart, as evidence on the third thesis with A71. A request outside them is refused. Closing one Agent's Conduit leaves the other working. Every request has a record. The results say that access is mediated (ACC-027) and list the routes around the engine, as A70's T3 did. |
| 4 | **S1 complete.** The message history that S1's two Agents share, the tasks of change and repair below, and the author measurements against A70. Then a fresh Codex author, with only the specification and S1's tasks, does T1 to T7 (ruling 48, point 4). | A68's done-when: S1 runs end to end on the reference model and the first providers, its safety vectors pass, and the author measurements are recorded with dated results, beside what they show for round 6's proposed simplifications, the coordinator's and the fresh author's apart. Then comes the next read by cold readers. |

### What increment 3 needs

The review of A70 and increment 1 found what running a real Agent on the engine's own runner
takes. Each item is the coordinator's decision, logged and strikeable, apart from those that
ruling 48 settled.

1. **A wrapper per client.** The runner reaches an Agent over its standard input and output,
   as an MCP server (PRV-007, TPL-019). Neither Claude Code 2.1.318 nor Codex 0.167.0 ships a
   server that runs a turn: `claude mcp serve` exposes Claude Code's tools, and Codex has no
   such command. Each Agent type therefore wraps its client, by an existing means such as the
   Claude Agent SDK or an adapter of the Agent Client Protocol. The clients stay unchanged as
   consumers of the engine's server.
2. **One long-lived session per Agent**, so that the live parts of T4 and T7 are tested, not
   one process per prompt. Experiment 008 found that a change of a client's tools reaches its
   model at the next turn, not within the turn in which it happens: Claude Code's streamed
   session took it at its next turn, and Codex took it when its session was resumed, which
   starts a new process. So each wrapper hands its client a change of access at a turn's
   boundary, and Codex's wrapper resumes its session for each turn unless A4 shows another
   way.
3. **The owner's configuration kept out**, by the flags that A70 used: for Claude Code
   `--setting-sources`, `--mcp-config` and `--strict-mcp-config`; for Codex
   `--ignore-user-config` and `--strict-config`.
4. **An Agent's own reach off by default** (ruling 48, point 1): for Claude Code, the claude.ai
   connectors, `SendMessage`, `ListAgents` and `RemoteTrigger` denied, and messages from other
   sessions refused; for Codex, `--disable apps`. An input of the Agent's template turns each
   on for that Agent.
5. **The login** (ruling 48, point 2): the owner's, shared, until A4 and A5 give each Agent a
   home of its own.
6. **The token from its file.** Each client reads `token` from `CONDUIT_IDENTITY_DIR` by its
   own header helper, Claude Code's `headersHelper` and Codex's `http_headers_helper`, so
   neither needs the token in a variable. Experiment 009 found that Codex runs its helper with
   none of the run's environment and rejects `env_vars` for an HTTP server, so Codex's Agent
   type writes the identity directory, not the token, into the helper's command line.
7. **One approval for the engine's server**, in the client's own terms (an allow rule in Claude
   Code, `default_tools_approval_mode = "approve"` in Codex), so that the engine's decision is
   the only one made per call.
8. **A private, empty working directory** per Agent, outside the shared directory and outside
   the other Agent's identity directory.
9. **Each connection confirmed by a tool call** that the engine records, never by a client's
   debug output.
10. **What experiment 009 left unshown, shown before increment 3 claims it**: that Codex's
    apps are off, which its events never list; that the owner's settings, hooks and
    instructions load in neither client, beyond the servers and tools each reports; that
    Claude Code refuses a message from another session; and which login each client used.
    Each needs evidence of its own, such as a sentinel the client would load if it could.

## Success criteria

Accepted by ruling 46, point 3: round 6's suggested criteria for the first author trial, with
the comparison against A70.

- A competent reader can complete the checked S1 path without reading ruling history or
  editing a provider.
- Closing one agent's access leaves the other working.
- The user can identify and resolve uncertainty without guessing that an effect failed.
- When S2 runs: its restrictions differ as intended without copying the shared service
  credential to agents.
- The author measurements are compared with A70's, on the same tasks.

Observed time and mistakes are recorded before any numeric target is set. These criteria
supplement the scenarios and the vectors; they do not replace conformance. The first is tested
by a fresh Codex author in increment 4 (ruling 48, point 4), since the coordinator, who does
the tasks first, knows the design.

## The tasks

The same tasks are done in the build and in the baseline for S1 (A70), so that the two can be
compared.

| Task | What the author does |
| --- | --- |
| T1 | Two agents share one project directory: agent A may only read it, agent B may read and write it. |
| T2 | The two agents share a message history: each may post and read. |
| T3 | Answer, from what is declared or configured, what each agent can reach. |
| T4 | Change a live setting: agent A may now also write, and agent B may now only read. |
| T5 | Upgrade the server behind the directory to a newer version. |
| T6 | Replace the directory with another, keeping both agents' access as intended. |
| T7 | Revoke agent A's access to the directory, check that agent B still works, then restore agent A's access. |

T1 and T2 are S1's setup. T4 to T7 are the chapter on change and repair of D08 of round 6, with
T7 the success criterion on closing one agent's access.

## Measures

Each measure is counted per task, the same way in the build and in A70. Nothing generated by a
tool counts as authored.

| Measure | How it is counted |
| --- | --- |
| Files edited | Each file that the author creates or changes. |
| Lines written | Each non-blank line that the author writes or changes, comments excluded. |
| Concepts met | Each distinct concept that the author must use or understand to finish the task, listed when first needed. For the build these are terms of `spec/definitions.md`; for the baseline, each tool's own. |
| Grants made by hand | Each separate statement that gives one agent access to one thing: a Conduit, a permission rule in an agent's settings, an entry for an MCP server in an agent's configuration. |
| Adapters written | Each wrapper, shim or script the author writes so that one tool fits another, counted in files and lines. |
| Duplicated configuration | Each value, such as a path, a server's definition or a credential, written in more than one place. |
| Per-agent changes of access | The number of places edited to change what one agent can reach. |
| Inspection effort | The files and commands that a person must read or run to answer T3. |
| Provider-specific code | Each line in the author's files that names a specific backing tool, such as Docker. |
| Recovery steps that need a person | Each manual action needed after a failure to get back to the intended state. |
| Observed time and mistakes | The wall time of each task, and each mistake made and how it was found. |

The first five measures include D22's; D22's "files edited" and "concepts met" are counted as
here.

### Counting rules added after the baseline ran

The review of 2026-11-09 found ways in which the measures, as written above, could flatter the
build. The coordinator added these rules on that day, after A70's numbers were known, which
makes them the rules most open to challenge; the owner can strike any.

- **Provider work apart.** What the build ships as packages, such as the filesystem server's
  contracts and its item, the type of the message history and the two Agent types with their
  launch configuration and wrappers, is counted by the same measures and listed apart, as
  provider work. S1's own project, its templates, instances and Conduits, is the author's.
  A70's harness flags, which it did not count, are noted as the same kind of work.
- **A client's concepts count.** The build also counts each concept of Claude Code or Codex
  that its author must use or understand, listed apart from the terms of
  `spec/definitions.md`, unless an Agent type that the build ships carries it.
- **Requests and time without access.** For each task the build lists apart every request the
  author makes and every interval in which an Agent has no access, the trace that D08 asks
  for; A70's restarts and lost sessions are listed beside them.
- **T4 and T5 against the better baseline too.** They are compared with A70's numbers and also
  with an allow list (Codex's `enabled_tools`, a per-tool allow in Claude Code) and with the
  server's `--read-only`. T4's restart counts against the baseline only if a run in a trusted
  workspace repeats it.
- **T3 covers reach around Conduit.** The build's answer to T3 also gives what each Agent
  reaches by its own means, its shell, its built-in tools, its account's services and its
  messaging, and counts what it takes to find, as A70's answer was held to. Duplicated configuration, per-agent changes of access, inspection effort,
provider-specific code and recovery steps are the baseline's measures in the S1 row of round 6's
table.

## Findings

Each increment logs what it finds in
[docs/proposals/first-build/](../proposals/first-build/README.md), one log per increment (ruling
46, point 4). A finding is one of:

- a **gap**: a case that no promise decides;
- an **ambiguity**: a promise that two readings would build differently;
- a **contradiction**: two promises that cannot both hold;
- **needless complexity**: a part of the language that the task did not need, or that cost more
  than it gave.

Each row of the log gives the finding, the IDs of the promises, what the build did and the fix.
The coordinator fixes a finding in the same pull request, in the promises and with a vector, and
logs the fix as its own; the owner can strike it. A finding whose fix adds or removes a feature,
or changes what Conduit is for, goes to the owner instead, and the build records what it assumed
in the meantime.
