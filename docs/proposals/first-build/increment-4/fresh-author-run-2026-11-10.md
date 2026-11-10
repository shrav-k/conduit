# Increment 4: a fresh Codex author's run of S1's tasks, 2026-11-10

Not binding. The [specification](../../../../spec/README.md) alone binds. A Codex session that had
never seen the project did T1 to T7 on the engine with the real Claude Code and Codex (ruling 48,
point 4), counted as [the measurement plan](measurement-plan.md) says and set beside the
[coordinator's run](coordinator-run-2026-11-10.md) and the baseline (A70). Its files, log and
report are in [fresh-author-run/](fresh-author-run/log.txt); its scripts are kept with a `.txt`
suffix, since the repository holds no code but Rust and the checkers in `tools/`.

## What it was given

A directory holding only: the specification (`spec/`), a page of S1's tasks, a page on the
engine's commands and the five package archives with their digests (written by the coordinator,
stating what the engine ships and nothing of S1's design), the two Agent packages' READMEs, and
empty `shared/`, `history/` and `shared-new/` directories. The engine was already serving an
empty store, started by the coordinator outside Codex's sandbox, so the author worked through
`install`, `submit`, `replace`, `retire`, `unregister`, `call` and `tools`. Codex 0.167.0 ran with
high reasoning effort in its workspace sandbox, with loopback network.

## The result in short

The author finished every task with the real clients, and read no ruling history and edited no
provider: the first of [the success criteria](../../../implementation/FIRST_BUILD.md#success-criteria)
holds. It took 45 minutes 48 seconds, made 10 mistakes and logged 14 places where the
specification or the commands left it unsure. It wrote 646 lines in 46 documents, in JSON, and
130 lines of Python in 7 scripts to drive the commands, which the measures count as adapters. In
T6 two of its turns ran past the engine's turn deadline and ended `unknown`, their replies lost,
and before T7 it recreated both Agents, losing their conversations.

## Beside the coordinator and the baseline

| Measure | Baseline (A70) | Coordinator | Fresh author |
| --- | --- | --- | --- |
| Lines written | 64 | 332 (YAML) | 646 (JSON) |
| Files edited | 7 | 15 | 46 |
| Adapters written | 0 | 0 | 7 scripts, 130 lines |
| Mistakes | 7 | 3 | 10 |
| Recovery steps that need a person | 6, and 1 expected | 1 | 2: Agents recreated after T6, a stage rerun |
| Sessions lost | restarts in T4 and T5 | none | both Agents', at T7 |
| Observed time | 25 min 25 s | 30 min 13 s | 45 min 48 s |

| Task | Baseline lines | Coordinator lines | Fresh author lines | Fresh author's time |
| --- | --- | --- | --- | --- |
| T1 | 33 | 154 | 263 in 19 files | 5 min 1 s |
| T2 | 16 | 73 | 67 in 5 files | 4 min 2 s |
| T3 | 0 | 0 | 0 (a 50-line answer) | 3 min 24 s |
| T4 | 8 | 2 | 32 in 2 files | 3 min 23 s |
| T5 | 3 | 62 | 112 in 8 files | 5 min 49 s |
| T6 | 2 | 29 | 44 in 3 files | 11 min 6 s |
| T7 | 2 | 12 | 128 in 9 files (Agents recreated) | 13 min 3 s |

JSON written with one key per line takes more lines than the coordinator's YAML for the same
document; by documents, the fresh author wrote 46 against the coordinator's 33. Either way, both
runs fail the owner's bar for v0: the fresh author wrote more than the baseline on every task
that needed writing, and the coordinator on all but T4.

## What the fresh author ran into

- **The same first mistake as the coordinator.** It omitted the root Interface's Conduit to the
  new Pool, so its Resource could not be set up (finding 4-11).
- **Turns that outlast the engine's deadline (4-12).** Asked for several operations in one turn,
  both Agents ran past the 180-second turn deadline; the turns ended `unknown` though their
  nested reads and writes completed, and their final replies were lost. It could not tell from
  `reconcile.v1`'s one `occurred` value what to attest of a whole turn, kept them unknown, and
  recreated the Agents.
- **Results that exit zero (4-13).** `conduit call` exited 0 for results that were `isError` or
  `unknown`, contrary to the commands page; and `submit` registered and partly set up before
  exiting non-zero, so success and failure both had to be read from its output.
- **Discovering the commands (4-14).** No `--help`; `tools --as` takes a bare name and works only
  once the instance is set up, and its error does not say so; the Agent packages' READMEs still
  show the old `register --package` syntax.
- **Intermittent timeouts (4-15).** Initialization of the commands' client timed out at times,
  with calls in parallel and in sequence, with no cause given.
- **Stale status (4-16).** Inspection showed earlier `RunStopped` conditions beside a ready
  state while calls succeeded.
- **Codex's own agent tools.** Codex's native `list_agents` worked, confirming finding 4-06.
- **The history is a convention.** Posting is writing a file: nothing enforces append-only,
  author or order, as the baseline also found.

## What this shows

A newcomer can do S1 with the specification and the commands alone, which is the success
criterion; but it costs more than the coordinator, who knew the design, and far more than the
baseline. The costs are the ones the coordinator listed (each document's frame, a template and
an instance for everything, change as new documents, root's Conduits to its own Pool, standalone
Conduits' tool names), and three the coordinator did not meet: the commands' behaviour on
failure, long turns, and discovery. The fresh author's 130 lines of scripts suggest the commands
are not yet an author's tool but a programmer's.
