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
logs its findings (below).

| # | Increment | Done when |
| --- | --- | --- |
| 1 | **The strict loader (A43).** It reads a document in YAML or JSON, strictly (SCH-012). It reads a project directory as MAN-001, MAN-011 and SUB-010 say. It validates each document against the schemas under `spec/schemas/`, after bundling the Schema contracts as `tools/check_schemas.py` bundles them. Formats are asserted (SCH-001, SCH-004), and patterns are ECMA-262 (SCH-016). | Every document under `spec/schemas/examples/valid/` is accepted. Every file under `invalid/` is rejected, by the step its name gives: when read, or by a schema. Every document under `invalid-by-rule/` passes the schemas, since its rule comes with the core. The verdicts agree with the schema steps of `check_schemas.py`. SCH-004's uuid grammar and SCH-016's patterns pass cases written for them. `cargo test --release` passes. |
| 2 | **The core on S1 (A38, A39).** First start (FST-001 and after), the registration and resolving of S1's documents, setup and the opening of declared Conduits (CON-004), the access decision (ACC-001), how a request ends (OPS-001), and records. Safety vectors for the cases that S1 touches, E05 of round 6 first. | S1's documents register and resolve on the core. Every decision cites a promise. The safety vectors pass. Each case the core cannot decide from the promises is a logged finding. |
| 3 | **The shell.** The engine's own runner starts a `plain` filesystem MCP server, unchanged. The presented server answers at a loopback `CONDUIT_URL` with tokens. Two real Agents share one directory: one through a Conduit that lists reads, one through a Conduit that lists reads and writes. | Each Agent sees exactly the tools of its open Conduits. A request outside them is refused. Closing one Agent's Conduit leaves the other working. Every request has a record. |
| 4 | **S1 complete.** The message history that S1's two Agents share, the tasks of change and repair below, and the author measurements against A70. | A68's done-when: S1 runs end to end on the reference model and the first providers, its safety vectors pass, and the author measurements are recorded with dated results, beside what they show for round 6's proposed simplifications. Then comes the next read by cold readers. |

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
supplement the scenarios and the vectors; they do not replace conformance.

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
here. Duplicated configuration, per-agent changes of access, inspection effort,
provider-specific code and recovery steps are the baseline's measures in the S1 row of round 6's
table.

## Findings

Each increment logs what it finds in `docs/proposals/first-build/`, one log per increment
(ruling 46, point 4). A finding is one of:

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
