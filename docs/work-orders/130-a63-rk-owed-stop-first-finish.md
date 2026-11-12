# Work order 130 (A63-RK, owed stops first, finished): close the window after admission

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
token or a private key, even one that a test made.

## What and why

Order 129 (`docs/work-orders/129-a63-rk-owed-stop-first.md`) made the runtime-provider kit refuse
`run.v1` on a machine that still owes a stop, under the owner's answer to MC4-002: a provider
carries out a stop it owes on a machine before it starts any run there. Its two rounds are spent.
Round two added a per-machine admission lock (`src/server.rs`, `admission`, near line 321): a
stop publishes its obligation under it (near lines 143 to 153), and `run.v1` checks owed stops
under it (near lines 547 to 571). Its final verifier left one must: `run.v1` releases the lock
(near line 572) before it saves its candidate, stages and starts, so a stop of another instance
on the same machine can publish its obligation during those awaits, fail, and the run still
starts. This order starts from order 129's last commit, which is where your working tree begins,
and closes that window. Order 129's decisions all still apply; read it and the kit's README first.

## The decisions it applies

1. **The lock becomes a read-write lock per machine.** A stop publishes its obligation under the
   write side, briefly, as it does now. `run.v1` takes the read side, so starts on one machine do
   not wait for one another.
2. **Check again right before the start.** `run.v1` keeps its first check (it refuses early, before
   any effect). Then, after staging and immediately before calling the backend's `start`, it takes
   the read side again, checks owed stops on the machine again exactly as the first check does
   (durable and in memory, other instances, the same hostname), and holds the read side across the
   backend's `start` call only, releasing it as soon as `start` returns. Staging is not under it,
   so a slow staging never delays a stop.
   Put both checks in one helper. Take the locks in the order they are taken now, the instance
   first and the machine second. Never take a second read while holding the first, and drop the
   read side before matching on `start`'s result: a failed start goes to `failed_start`, whose
   stop takes the write side, which the same task would then wait for forever. The cancellation
   check comes after the second check, under the read side.
3. **A refusal at the second check** leaves nothing behind: the staged material is discarded, no
   child process runs, no gate is opened, and the candidate record is saved as stopped. The answer
   is `failed` with the same explicit message as the first check (each owed stop's instance ID, the
   machine's hostname, and that the start may be tried again once those stops are carried out).
   As on the staging-failure path, cancel the run's token (which ends its deadline task), drop the
   read side, drop the staged material (`Staged` cleans up when dropped), then save the record.
   Do not call `failed_start`: its stop would call the backend, which likely fails as A's did, and
   turn B into an owed stop answering `unknown`.
4. **What the README says, as contract text a runtime carries:** a stop never waits on staging,
   only on the backend's `start` calls already in progress on its machine; and a run is admitted
   only if, at the moment its start begins, no other instance owes a stop on its machine. Replace,
   not just add to, the README's sentence that "a blocked start never delays another instance's
   stop" (near lines 174 and 175): on the same machine it now does, for the length of the `start`
   call.
5. **Test,** deterministic, with the fake backend: B passes the first check and pauses in staging
   (a hook in the fake that the test releases); A's stop then publishes its obligation and fails;
   B resumes, and the second check refuses it: `failed`, its message naming A and the machine;
   the test checks all three of decision 3's conditions: no gate opened, no `start` called and so no
   child running, and B's record saved as stopped, with its staged material dropped. And: a stop of A that comes
   while B's `start` call is in progress waits for that call, then proceeds. In the first test,
   await A's stop with a timeout before releasing B, which shows that staging never delays a stop.
   The fake needs hooks it lacks, which you add in `tests/support` only: a pause in staging, a
   pause in `start` that can end in success, and an observable `Staged` with a count of drops.
6. **Two earlier tests change.** `blocked_start_does_not_delay_stop_call_for_another_instance` and
   its interval twin (`tests/runtime.rs`, near lines 444 to 501) put the other instance on the
   same machine, where its stop now rightly waits for the blocked `start`. Move that instance to
   another hostname, as order 129's other-machine test does, so that they still show that a
   blocked start never delays a stop on another machine; decision 5's second test covers the same
   machine. Keep every other test of orders 122, 124, 126 and 129 passing.
7. **Nothing else changes,** and nothing outside the kit. If a fix needs a choice that `spec/` or
   orders 129 and 124 do not make, take the narrowest, put it behind one function, and report it.

## Scope

Change only `packages/runtime-provider-kit/`.

## Done when

In `packages/runtime-provider-kit/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`, run twice; report both runs' totals and any test that differed

From the repository root: `python3 tools/check_docs.py`. Run these with `bash`. Each prints
nothing when it passes:

```bash
D=packages/runtime-provider-kit
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
```

`target/` stays ignored and unstaged. Do not stage anything.

## Record

Run on 2026-11-12 on the Mac, stacked on order 129's last commit (74b1c85): the worker and a fresh
verifier (240 s), which found nothing to change in one round. The per-machine lock is now
read-write: a stop publishes its obligation under the write side; `run.v1` checks again right
before the backend's `start` and holds the read side across that call only, never across staging,
so a stop waits at most for start calls already in progress on its machine. A refusal there leaves
nothing behind: no gate, no child, the candidate saved as stopped. The two earlier tests that put
the other instance on the same machine now put it on another. The lane coordinator ran the kit's
tests on the result: 72 passed.
