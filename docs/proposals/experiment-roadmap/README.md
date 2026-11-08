# Experiment roadmap toward v0.0.1

Status: a proposal from Santiago, 2026-11-08, against `main` at `2ca84f3`. Not ruled: the owner
decides. It adds a working track on top of the [experiment program](../../AUDIT.md#experiment-program)
(scenarios S1 to S10, evidence cases E01 to E20) and [ruling 44](../ruling-44/README.md), which
makes the first build the next milestone. It changes no promise, scenario or action item.

## The path

1. **Spec plan** (Santiago). Pick the promises and the slice of S1 that the first experiments
   cover, and write down what each experiment must show before it runs.
2. **Basic experiments and test results.** Run them in Rust under `experiments/`, each in its own
   directory with dated results. Expected outcomes stay labeled as expected until they run.
3. **More experiments.** The proposals below, plus whatever step 2 turns up.
4. **Conclusions with Shravan.** Review the results together. Any change to the language goes
   through an owner ruling in the [ruling log](../../AUDIT.md#ruling-log).
5. **Start building v0.0.1.** Its scope is agreed in step 4. A starting proposal is at the end.

## Proposed experiments

These are not yet in the program. Each is small, runs on one Mac, and feeds a case the program
already names.

| # | Question | Feeds |
| --- | --- | --- |
| X1 | Can 1Password be read with no person present, the way [005](../../../experiments/005-macos-keychain/README.md) showed for the Keychain: by a service account, a desktop-app session, and when locked? | A second secret store for S10 and D09 |
| X2 | How much does an author write for S1 using only instance documents on top of packaged templates? Count files, lines and concepts. | D22's author measurements |
| X3 | The same S1 setup with tools people use today: a coding agent's own permission settings plus its MCP config, and a Compose file. Count duplicated credentials, per-agent edits and recovery steps. | D08's manually configured baseline |
| X4 | Does a Rust JSON Schema validator accept and reject exactly what `tools/check_schemas.py` does on every file in `spec/schemas/examples/`? | A first `conduit check` command, and the reference model (A38) |
| X5 | With a real coding agent behind a gated MCP server on one machine over stdio, what does the agent see when a call is refused, a Conduit is closed, or an outcome is unknown? | An earlier, cheaper look at E17 |

## v0.0.1: a starting proposal

The smallest thing a person can run and see Conduit work, as a first slice of the S1 tracer:

- `conduit check <dir>`: validates a project's documents against the schemas and the rules no
  schema can express.
- One node, the engine's own runner, stdio only.
- One Agent, one directory Resource served by a `plain` MCP server, and one Conduit that lists
  read operations. Anything not listed is refused.
- One secret Resource in the Keychain, passed to a server's run and never to the Agent.
- A record of every request.

Out of scope for v0.0.1: several machines, second providers, filters and controls. Whether
v0.0.1 is the reference model of ruling 44 or a separate demonstration beside it is a question
for step 4: [AGENTS.md](../../../AGENTS.md) says the nature of that first build is to be
discussed with the owner before it starts.
