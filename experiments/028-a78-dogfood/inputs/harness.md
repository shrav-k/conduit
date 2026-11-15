# The dogfood harness: stage 1 of A78

Kept outside the repository; never committed. Under ruling 70, Conduit runs its own work orders
(A78; ruling 61 B12). In stage 1, root dispatches orders one at a time through turns on two Agents
of the `conduit-core` Fleet: `worker-1` and `verifier-1`, both `codex.v2` with their own tools on (their templates name a
directory: ruling 75 B1),
working in the slot clone `../slot-1`. `conduit-order.sh` replaces `../run-order.sh` for those
orders only; the other lanes keep using `run-order.sh`. Every script here runs outside any
sandbox: a sandbox around `conduit serve` is inherited by every Agent (G21).

**In v0 these Agents are you.** They run as your macOS user, with no sandbox and no prompts, and
can reach everything you can, including the store's root token, your keychain (and so GitHub)
and other projects. The guards below are speed bumps, not confinement.

## Files

| File | What it does |
| --- | --- |
| `setup.sh` | Makes the pieces, in four steps: `slots` (the slot clone, pushes disabled), `engine` (a clone at `ENGINE_REF`, built), `agents` (a clone at `AGENTS_REF`, order 076's branch, whose two Agent archives it packs into `archives/`), `store` (initializes `store/`, or `smoke-store/` with `SMOKE=1`, renders the project, installs, submits, restarts once, exports records) |
| `project/` | The `conduit-core` project, from `engine/examples/s1`: the manifest, the templates (`codex-worker`: names the slot as its directory, 4-hour deadline, effort high; `codex-verifier`: the same with 1 hour; the tree Pool, empty in stage 1; the turn Conduit) and the instances (`worker-1`, `verifier-1`, `turn-worker-1`, `turn-verifier-1`, the Pool `trees` and root's route to it). The tree Resource `tree-1` and its npm-run filesystem package come back with orders 4 and 5. `@...@` values are filled by `setup.sh` into `rendered/` |
| `smoke/` | The smoke store's manifest (adds agent-claude-code) and three probe Agents |
| `conduit-order.sh` | Runs one order: `run-order.sh`'s interface and log lines, by `conduit call` |
| `smoke.sh` | Runs the design's six smoke checks against the smoke store |
| `lib.sh` | Shared settings (paths, deadlines, `CALL_TIMEOUT=15000`) and functions: serve start and stop, records export, the call with its watchdog, leftover processes |
| `report.py` | Reads a call's result as an order report and validates it against the order's schema |

Made by the scripts: `engine/`, `agents/`, `archives/`, `pins.env` (what was built), `store/`,
`smoke-store/`, `rendered/`, `logs/` (setup, serve logs), `run/` (pid files, locks, status),
`runs/out-NNN/` (each order's outputs), `runs/smoke-*/`.

## Before the first run

- Order 074 (the session idle limit) is merged, and the engine is pinned to a main that has it.
  Without it every call longer than about 300 seconds is cancelled; `setup.sh engine` warns.
- The Agent packages come from order 076's branch, which holds 050a plus 076
  (`wo/076-agents-v2-folder-grant`, on GitHub; the default `AGENTS_REF`). For a branch only the
  main checkout has, build with `SRC=<main checkout>`.
- `codex`, `cargo`, `node`, `git`, `jq` and `lsof` are on PATH (`claude` and `curl` for the
  smoke test). serve passes its PATH to every Agent's client.
- The owner's main branch is protected on GitHub (the design's owner ruling 1).

## The smoke test (about 1 hour; after 074 merges, with order 076's branch)

```sh
cd <dogfood>
./setup.sh slots engine                    # ENGINE_REF defaults to origin/main
./setup.sh agents                          # AGENTS_REF defaults to origin/wo/076-agents-v2-folder-grant
SMOKE=1 ./setup.sh store                   # throwaway store on port 3918
nohup ./smoke.sh > runs/smoke.log 2>&1 &   # never in the foreground: step 4 takes 20 minutes
cat runs/smoke-*/summary.txt               # when it is done
```

Each step prints `smoke N: PASS|FAIL|SEE`. `./smoke.sh 1 3` runs chosen steps.

1. `worker-1` runs `pwd`, `cargo --version`, `python3` with yaml and jsonschema, and `git status`
   in the slot. It writes a nonce under `$HOME/.cache/conduit-dogfood/` and fetches
   `https://crates.io`: the harness checks the file and the HTTP status, so no inherited sandbox
   confines it.
2. `probe-off` names no directory, so it has codex.v1's tools (ruling 75 B1). It is asked to
   write a file by shell or by any file-editing tool; the file must not appear.
3. `probe-claude` (claude-code.v2) runs `pwd` in the slot with no prompt; whether a write inside
   `.git` is allowed or denied is recorded (the file is removed).
4. A 20-minute `sleep` turn on `worker-1` returns: the session outlives it (074), and the call
   holds past the CLI's 120-second default and the coordinator's 10-minute foreground limit.
5. `probe-short` (60-second deadline) fails with `deadline: no turn boundary within 60 seconds`;
   its next turn answers `session failed at turn 1: deadline...`.
6. A turn that starts `nohup sleep 900 &` returns promptly; what it left in the slot is listed
   (Codex's PTY children escape the run's process-group kill, runner.rs:104-114) and stopped.

Record the results in the increment's log. A FAIL in steps 1, 4 or 5 blocks the first order.

## The first order

The first order needs the engine pinned to a main with both 041b and 074, and a store made by
that engine:

```sh
cd <dogfood>
./setup.sh engine                          # re-pin once 041b and 074 are on main
REINIT=1 ./setup.sh store                  # the real store, port 3917; an old one is moved aside
SCHEMA=004-i2-01-vectors-and-first-start ON_LIMIT=exec \
  nohup ./conduit-order.sh 042-build-tombstones-and-inventory net > runs/out-042.log 2>&1 &
```

While it runs: `cat run/store.status`, `tail runs/out-042/progress.log` (the slot's changed files
every 10 minutes: the turn itself leaves no event trail), `tail runs/out-042.log`.

`ON_LIMIT=exec` is the first order's time limit: when the worker's 4-hour deadline fails the turn,
or `LIMIT` seconds pass (default 15300, after the deadline), the harness finishes the order with
plain `codex exec` on the same branch, keeping the partial work. Set `LIMIT` lower to stop
earlier, for example `LIMIT=$((2 * 7200))` for twice a 2-hour estimate.

After it ends, as today, outside Conduit:

```sh
git -C <main checkout> fetch <slot-1> wo/042-build-tombstones-and-inventory:wo/042-build-tombstones-and-inventory
```

then rebase in a worktree, run the checks, review the diff, push, open the PR and squash-merge.
The slot's branch stays; the next order starts a new one from `BASE`.

## What conduit-order.sh does

`[SCHEMA=] [EXTRA=] [ROUNDS=2] [BASE=origin/main] [JOBS=6] [TT=4] ./conduit-order.sh NNN-name [net]`

1. Takes the slot's lock (one order, smoke test or setup at a time), checks the slot: clean, push disabled, and no
   branch `wo/NNN-name` yet (`FRESH=1` resets one). Fetches, takes the order files from
   `origin/main` through the slot, switches the slot to `wo/NNN-name` at `BASE` and cleans it
   (target/ survives). Prints `base=`.
2. For each worker and verifier turn: stops serve, exports the records (`records-ID-N.jsonl`),
   starts serve with `--call-timeout-seconds 15000` (above both deadlines; not stored, so passed at
   every start), waits for root's two turn tools and fails on any startup setup failure in the
   serve log. The restart gives a fresh conversation.
3. Sends `run-order.sh`'s prompt plus a paragraph on how the turn runs (report as one JSON object
   valid against the schema's absolute path; the build environment to prefix, since the wrapper
   drops it; no commit, push or branch; read AGENTS.md; no credentials) by
   `conduit call --store store turn-worker-1.turn.v1 '{"message": ...}'`, in the background, with a
   watchdog at deadline + 900 seconds and a check that serve is still running.
4. Reads the result: `report.py` takes the turn's text, strips a code fence, and validates it.
   Invalid: one repair turn in the same conversation (no restart) with the validator's errors.
5. Runs the guards, checkpoints the worker's changes (`wip: worker output of order NNN`), then
   the verifier's turn the same way, then `run-order.sh`'s rounds: a must or should finding starts
   another worker round and a fresh verifier, up to `ROUNDS`.
6. On exit: stops serve, exports the final records, appends a line to `measures.jsonl`.

Log lines match `run-order.sh`'s: `base=`, `worker exit=N seconds=S`, `checkpointed`,
`verifier exit=N seconds=S`, `round N: no must or should findings`, `worker round N exit=`,
`checkpointed round N`, `verifier round N exit=`, `done rounds=N final-report=...`. The harness's
own lines begin `dogfood:`. `worker exit=` is the call's status: 0, 1 (failed or unknown: the
cause follows on a `dogfood:` line), 2 (prompt not sent), 124 (watchdog), 125 (serve exited).
Unlike `run-order.sh`, a failed or invalid verifier stops the order instead of reading as no
findings.

Fallbacks:

- `VIA=exec`: `run-order.sh`'s own `codex exec` worker and verifier, with `-C` the slot, on the
  same branch. Guards and validation still run.
- `START=verify`: skips to the verifier. The slot must be on the order's branch, clean, with the
  worker's checkpoint, and `runs/out-NNN/worker-report.json` must be valid.
- `KEEP=1`: runs the worker again on the existing branch, keeping its uncommitted changes, with
  a paragraph telling it so.

## The guards

Run after every turn, repair turns included. A trip stops the order for the admin (a worker's
changes are checkpointed first, so `START=verify` can continue once reviewed); a note is only
logged in `runs/out-NNN/guard/log.txt`.

| Guard | Trips when | Notes |
| --- | --- | --- |
| The slot's HEAD and branch | either moved during the turn: only the harness commits | |
| The verifier changed nothing | `git status` in the slot is not empty after a verifier turn | a rewritten `Cargo.lock` alone is restored and noted |
| Nothing pushed | `wo/NNN-name` changed on GitHub, or any ref on GitHub now points at a commit that exists in the slot and in none of its `origin/*` refs (a commit made in the slot), or GitHub's refs cannot be read | others' pushes and deletions (the admin, other lanes) |
| The main checkout | `conduit/`'s branch changed, its HEAD moved to a commit origin lacks, or its working tree (tracked changes and untracked files) changed; read with `--no-optional-locks`, so it takes no lock | HEAD moved to a commit origin has (a pull). `GUARD_MAIN=warn` turns trips into notes, for when the admin edits `conduit/` during a turn |

The slot's pushes fail at once: its push URL is `file:///dev/null/dogfood-clone-cannot-push`
(the design's `no-push:` is read by git as an ssh host) and a pre-push hook refuses. A push to an
explicit URL with `--no-verify` passes both, and the ref guard catches it only afterwards.

After a failed turn, processes whose working directory is in the slot are listed and stopped;
after a successful one they are listed only.

## Measures and records

Per order, in `runs/out-NNN/`: the prompts, each call's JSON and stderr, the reports, the guard
files, `progress.log`, the engine's records at each restart, `harness.log` and `measures.jsonl`
(worker and verifier seconds, rounds, findings by severity, repair turns, restarts and their
seconds, guard trips and notes, watchdog fires, leftover processes). The admin adds
interventions, and whether tests that bind loopback ports ran, to the increment's log.

## Re-pinning

- After 041b and 074 are on main: `./setup.sh engine`, then `REINIT=1 ./setup.sh store`.
- A new commit on order 076's branch: `./setup.sh agents`, then `REINIT=1 ./setup.sh store`. One store never holds
  two different 0.2.0 archives.
- After order 050: re-initialize; each slot Agent is then signed in once by the owner.
