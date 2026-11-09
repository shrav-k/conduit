# Work orders: the admin session and Codex

Not binding. The specification in [spec/](../../spec/README.md) alone binds. This page says how
the work on Conduit is divided between the coordinator's session and Codex sessions, as
[ruling 48](../proposals/ruling-48/README.md) asks, and what the first round with real agents
taught about doing it without a person present.

## Roles

| Who | Does | Does not |
| --- | --- | --- |
| The owner | Rules: on behavior, on what Conduit is for, on the order of the work. Can strike any decision of the coordinator | |
| The admin session, the coordinator (Claude Code) | Plans the work and writes each order: what it applies, its scope, its done-when, its report. Makes the decisions an order applies, under rulings 37, 38 and 46, point 4, each logged and strikeable. Reviews what comes back, runs the checks, commits, opens and merges the pull request. Puts to the owner what is the owner's | Spend its own session on work that an order can state |
| A Codex worker | Carries out one order in a working tree of its own: a build increment, an experiment, a sweep of the promises, a verification of another order's result, or a fresh read | Commit, push or open a pull request. Decide what the order leaves to the admin: it reports instead |

What goes to Codex is work whose result the admin can check: a done-when that the checks show,
counts that cite their evidence, or findings that cite the text. What stays with the admin is
whatever chooses: a decision on a finding, a measure or its counting rule, what a result
means, and what goes to the owner.

## An order

An order is a file `docs/work-orders/NNN-name.md`, written before it runs, as ruling 45's rule
asks of an experiment. It holds:

1. **What and why**: the goal, in two sentences, and the records it comes from.
2. **The decisions it applies**, each with its source. A worker applies them and does not
   reopen them. Where one cannot be applied as written, it applies the nearest sound reading
   and says so in its report.
3. **What to read**: the files and the promises, by path and ID.
4. **Scope**: the files it may change. Anything else it needs changed goes in its report.
5. **Done when**: the checks that must pass, each a command.
6. **The report**: a JSON Schema, given to Codex with `--output-schema`, so that what comes
   back is a value the admin reads, not prose. Every order's report says, at least, what it
   changed, what it could not do, which choices it made itself, and each check's last line.
7. **Rules**: those of [AGENTS.md](../../AGENTS.md#if-you-were-given-a-work-order), which every
   Codex session in this repository reads, and any of the order's own.

## Dispatch

One order, one working tree on a branch `wo/NNN-name` from `main`, outside the repository's
own checkout. The admin starts Codex headless in it:

```sh
CARGO_NET_OFFLINE=true codex exec \
  --ignore-user-config --strict-config --disable apps \
  -s workspace-write --add-dir "$HOME/.cargo" \
  -C <tree> -c 'model_reasoning_effort="high"' \
  --output-schema <order>.schema.json -o <report>.json --json \
  - < docs/work-orders/NNN-name.md > <events>.jsonl
```

Each part is there for a reason that this round found:

- `--ignore-user-config` keeps the owner's own Codex configuration out, and its MCP servers.
  Codex still signs in with the owner's subscription (ruling 48, point 2).
- `--strict-config` stops at a key that Codex does not know. A70 lost a task to an override
  that Codex accepted without a warning and ignored (mistake 2 of its results).
- `--disable apps` turns off the apps of the ChatGPT login (ruling 48, point 1).
- `workspace-write` lets Codex write in its tree and nowhere else, with no network. Cargo's
  own cache is the one other place it writes, and with `CARGO_NET_OFFLINE` every crate comes
  from that cache.
- The order goes in on standard input (`-`), so that Codex never waits on an open input: one
  of A70's runs did, for five minutes.
- `-o` and `--output-schema` give the report as a file; `--json` keeps every event, which the
  admin reads only when a report needs explaining. Codex's events do not show every command it
  runs (A70, surprise 7), so a claim that matters is checked against the files, not the
  events.

## Checking what comes back

1. **A verifier**: a second, fresh Codex session, in a read-only sandbox, with the order, the
   diff and a schema of its own. It tries to refute each change against the decisions and the
   texts they cite. A reviewer reading cold found the most consequential faults of this round:
   a contradiction across nine promises, and a wrong claim about Codex in the coordinator's own
   notes.
2. **The admin**: runs every check itself, reads the diff and the verifier's findings, scans
   for private paths and names, and decides what to fix, to reorder or to put to the owner.
3. **One more round at most**: the admin sends the worker back once with the verifier's
   findings it accepts. If that does not settle them, the admin finishes the work itself.

## Fresh reads

A fresh read is an order whose worker sees only what the order names: a working tree of the
build base, or of `spec/` alone, and no history of rulings. Ruling 48, point 4 gives Codex
the fresh author of increment 4. A fresh reader's report lists where it had to guess.

## What this round taught

From A70, increment 1 and the review of both:

- **Brief in decisions, not in hopes.** Each order states the decisions, and the worker's
  report states its own choices apart, so that the admin sees every choice made.
- **A skeptic per angle pays for itself.** Every review of this round changed a result: the
  loader's three reviewers found four defects, A70's auditor corrected six counts, and the
  last review found two of the coordinator's own errors.
- **Confirm a connection with a real call.** Codex's debug output hides MCP tools, and its
  configuration can fail silently: A4, A71 and increment 3 confirm each Agent's connection by
  a tool call that the server records.
- **Keep the owner's machine out.** No order edits the owner's agent configuration or reads a
  credential. A worker's own state, such as Codex's sessions, is the agent's.
- **Close the input, pin the flags, write the report to a file.** The dispatch above is the
  sum of this round's harness faults. Two orders started at the same moment once raced for
  Git's configuration while making their working trees; `git worktree add --no-track` avoids
  it.
- **Pre-register before the harness, amend before the run.** Order 002's review found that the
  pre-registration of experiment 008 named tools otherwise than IFC-012 and left its matrix
  open; it was amended, with a dated note, before any run.
- **Count the coordinator's own choices as risks.** Counting rules fixed after a result is
  seen are the easiest to challenge, so the order that sets them says so.

## Running the orders through Conduit

Once increment 3 runs, the orders are to go through Conduit itself (A78): each order's working
tree is a directory Resource, the queue of orders and their reports is the message history,
and each worker is an Agent that the engine runs with a Conduit to its own tree that lists
reads and writes, and one to `main` that lists reads. The admin opens a worker's Conduits when
it dispatches the order and closes them when it merges. That is S1 in daily use, so each
friction is a finding of the first build, logged as one. When S2 runs, opening the pull
request moves behind a Conduit to GitHub too, and the token for it stays with the engine
rather than going to any worker.

## The record

| Order | What | Worker | Verifier | What the admin changed | State |
| --- | --- | --- | --- | --- | --- |
| [001](../work-orders/001-review-fixes.md) | The coordinator's fixes from the review of increment 1 and A70, and ruling 48, point 3 | 21 minutes; every decision applied as written; 36 files; one point left to the admin | 5 minutes; one `must`: `check_schemas.py` crashed on a repeat count past Python's limit | Fixed the crash, with a regression case; settled the point left to it, SCH-015's rule for a reference to the wrong kind | done on 2026-11-09 |
| [002](../work-orders/002-real-client-harness.md) | The harness of experiment 008 (A71) | 17 minutes, then 25 on a second round; a stub on the official Rust SDK and a runner, with tests that need no agent | 5 minutes; five `must` and one `should` in the stub, all fixed on the second round | Amended the pre-registration before any run: tools named as IFC-012 names them, and the matrix cut to 68 runs. Sent the worker back to use the owner's login rather than a fresh configuration and an API key. Added Codex's approval of the stub's server, which the second round missed. Ran one real run per client before the matrix | done on 2026-11-09 |
| [003](../work-orders/003-increment-2-study.md) | A study for increment 2, the core on S1 | 17 minutes; the plan, the drafts of S1 and a test; six decisions for the admin, two candidate findings, nine proposed orders | 4 minutes; two `must`, a promise's strength misstated and a safety case's refusal misdefined, and one `should`, a weak case | Sent the worker back once with the findings, which it fixed in 5 minutes; [decided](../proposals/first-build/increment-2/decisions.md) the six points and the two findings | done on 2026-11-09 |
| [004](../work-orders/004-i2-01-vectors-and-first-start.md) | I2-01: the vectors' format, the core's crates, and first start | 17 minutes, then a second round; `conduit-core`, `conduit-conformance`, the format and 11 vectors; one point left to the admin | 5 minutes; two `must`: unauthorized requests reached a diagnostic before their refusal, and records took the named object for the target | Settled the point left to it (two leading byte order marks are not YAML) and sent the worker back once with it and the findings; 13 vectors pass | done on 2026-11-09 |
| [005](../work-orders/005-real-client-results.md) | The results of experiment 008 (A71) | | | | written |
