# Increment 4: the coordinator's run of S1's tasks, 2026-11-10

Not binding. The [specification](../../../../spec/README.md) alone binds. The coordinator's run of
T1 to T7 on the engine, with the real Claude Code and Codex, counted as
[the measurement plan](measurement-plan.md) fixed before any number was known, and compared
with the baseline for S1 (A70,
[its results](../../../../experiments/007-s1-baseline/results-2026-11-09.md)). The coordinator
knows the design, so these numbers are a floor on what the language costs today; the fresh
Codex author's run follows. Paths under the run's lab are written `<lab>/`. The author's files,
log and T3 answer are in [coordinator-run/](coordinator-run/log.txt).

## What ran

The engine as work order 025 left it (`main` at its merge), Claude Code 2.1.318 as agent A and
Codex 0.167.0 as agent B through their packages 0.1.1, Node 21.5.0, the filesystem server 2.1.0
and, after T5, 2.2.0, a fresh store and fresh directories. The author began from an empty
project directory and wrote every document; the shipped packages' digests were copied from the
example project and the fixtures' README. Every task ended when the real clients showed its
intended state.

## The result in short

Every task reached its intended state with the real clients, and on four points the build did
better than the baseline: no restart and no lost session for any change; the upgrade gave no
Agent a new tool (the baseline's read-only agent gained two write tools); T3's answer took two
commands; and a revocation reached the running session at its next turn. **But the author wrote
far more**: 332 lines against the baseline's 64, and more on six of the seven tasks. By the bar
the owner set for v0, "less of an author's work than the baseline on most of S1's tasks and much
more on none", **the language as it stands fails**. The reasons are specific and listed under
[what drives the count](#what-drives-the-count).

## Per task

| Measure | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Files edited | 5 | 4 | 0 | 1 | 2 | 2 | 2 | 15 distinct |
| Lines written | 154 | 73 | 0 | 2 | 62 | 29 | 12 | 332 |
| of which digests copied | 3 | 2 | 0 | 0 | 1 | 1 | 1 | 8 |
| of which `---` separators | 14 | 5 | 0 | 0 | 5 | 2 | 0 | 26 |
| Requests made by the author | 4 | 5 | 2 | 8 | 5 | 8 | 3 | 35 |
| Grants made by hand (to Agents) | 2 | 2 | 0 | 0 | 2 | 2 | 1 | 9 |
| Root Interface Conduits (apart) | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 4 |
| Adapters written | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Per-agent changes of access (A / B) | 2 / 2 files | 1 / 1 Conduit | 0 | 1 file shared, 3 / 3 requests | 1 / 1 Conduit | 1 / 1 Conduit | 1 / 0 Conduit | |
| Provider-specific lines | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Contract-address lines (apart) | 23 | 9 | 0 | 2 | 9 | 1 | 1 | 45 |
| Recovery steps that need a person | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| Time without access it should have | none | none | none | A 29 s, B 30 s | none | A and B about 80 s | none (A's was intended) | |
| Observed time | 8 min 33 s | 3 min 39 s | 38 s | 3 min 37 s | 4 min 21 s | 5 min 38 s | 3 min 47 s | 30 min 13 s |
| Mistakes | 1 | 1 | 0 | 0 | 0 | 1 | 0 | 3 |

Counted as the plan says. Lines are non-blank lines written, comments excluded, separators
included. A file copied by the author, such as a manifest, counts as written. Requests are the
author's commands that are not checks: `register`, `submit`, `install`, `replace`, `retire`,
`unregister` and `call` of an operation of the core package. T3's requests are the two
`conduit tools --as`. The baseline counted provider-specific code by the npm server's name and
its tool names; the author's files name neither, and the 45 lines that name a contract's address or a package's,
such as `example.org/s1/filesystem#file.read.v1`, are listed apart, as the plan says.

## Against the baseline, task by task

| Task | Baseline lines | Build lines | Baseline other | Build other | Less, same or more work |
| --- | --- | --- | --- | --- | --- |
| T1 | 33 in 5 files | 154 in 5 files | 3 recovery steps, 3 mistakes | 1 recovery step (a store discarded), 1 mistake | Much more |
| T2 | 16 in 5 files | 73 in 4 files | none | 1 mistake | Much more |
| T3 | 0; 10 items read and 1 command | 0; 2 commands, 2 READMEs | | | Less |
| T4 | 8 in 2 files, 1 restart | 2 in 1 file, 8 requests, about 30 s without access each | a running session kept its old tools | each running session changed at its next turn | Less, by the counting rule on restarts; the requests are the new cost |
| T5 | 3 in 2 files, 2 recovery steps and 1 expected | 62 in 2 files, 5 requests | the read-only agent gained `append_file` and `copy_file` and wrote with them | no Agent gained a tool | More lines; far safer, no recovery |
| T6 | 2 in 2 files | 29 in 2 files, 8 requests | | | Much more |
| T7 | 2 in 1 file | 12 in 2 files, 3 requests | reached the running session at its next turn | the same | More |

Against the better baseline the counting rules name for T4 and T5 (an allow list in each client,
and the server's `--read-only`), the baseline's T4 stays at about 8 lines and its T5 needs no
recovery: the build's T5 safety advantage over a careful baseline is smaller than over A70's run,
and its line count is still larger.

## What drives the count

1. **Each document's frame.** 33 documents, each with `apiVersion`, `kind`, `metadata` and
   `name`, about 130 lines, and 26 separators: half the total. The baseline's files had no such
   frame.
2. **A template and an instance for everything.** The directory, the history, each Agent and each
   Conduit is written twice, a template and an instance from it, though S1 uses each template
   once.
3. **Change is new documents (finding 4-01).** T5 and T6 rewrite a Resource, its template and the
   Conduits to it, because nothing changes an existing instance; T7 submits its Conduit again
   because closing removes it.
4. **Root's own Conduits to its Pool.** The root Interface could neither set up nor register a
   Resource of `workspace` until the author wrote Conduits from root to the Pool listing
   `set-up.v1` and then `register.v1` (mistakes 1 and 2): 35 lines of the total, for authority
   the owner already has.
5. **Tool names follow Conduit names.** Access added after an Agent exists is a standalone
   Conduit with a name of its own, so the history's tools are `history-a.*` and `history-b.*`,
   and after T5 the project's are `project-a.*` and `project-b.*`.

Each is a candidate for round 6's proposed simplifications (A68) and for finding 4-01's decision.

## What surprised the author, and the findings

- **A store that cannot be repaired (4-07).** After the first registration failed to set up the
  shared directory, `conduit serve` also exited with the same error, since it sets up registered
  instances first, so `submit`, the only way to add the missing Conduit, could not run. The store
  was discarded and registration done again.
- **A refusal that does not say why (4-08).** `submit` answered `refused`, naming no operation
  and no Conduit; the author found the missing `register.v1` by reading ACC-033.
- **Inspection (4-09).** `conduit inspect` cannot run while the engine serves, and
  `management.inspect.v1` on an Agent shows its declared Conduits but not the standalone ones it
  consumes. `conduit tools --as` answered T3.
- **Codex's own agent tools (4-06).** Asked to list its tools in T5, Codex named
  `collaboration.spawn_agent`, `collaboration.send_message` and four more, though the package
  disables `multi_agent` to turn sub-agents off (finding 3-05).
- **Unregistering a closed Conduit (4-10).** It reports invalid, because closing removed it; the
  diagnostic names OPS-029 and not that the object is gone.

## Safety and footprint

- Every request has a record: 93 access decisions and 93 request ends; 90 invocations, each
  dispatched and ended once. 16 tokens of the run were compared with the export and none
  appears in it.
- After SIGTERM every process of the engine was gone within about 50 seconds. A count at 30
  seconds showed three matches; the matching pattern also matched the checking shell's own
  command line, which likely explains them and the same observation in increment 3. Not
  confirmed.
- B's write tool was absent after T4, and T5 gave no Agent a tool its Conduit did not list,
  checked by the real clients.

## What this run does not show

- Whether a person new to the language does better or worse: the fresh Codex author's run.
- Times are a model's, as the baseline's were, and include the real clients' turns.
- The second machine, account services, messaging between sessions and the reach inputs set
  true, carried from increment 3, are not part of these tasks.
