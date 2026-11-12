# Work order 146 (SSH-R): the stop-drainer that outlives the provider

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no other machine. Run one
cargo command at a time, and never print a whole large file or log. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never print a key, a token
or a certificate's key, even one that a test made.

## What and why

The SSH runtime provider, `packages/provider-ssh-runtime/`, keeps a stop it could not carry out as
owed, and carries it out at its next contact with the machine (PRV-021). Its server runs on the
engine's own runner: it ends when the engine stops, and does not run again once the provider is
retired (PRV-002, PRV-020, PRV-026). [PROVIDERS.md](../implementation/PROVIDERS.md)'s decision D
settles an external stop-drainer that survives the server's retirement and retries over SSH to
stop the recorded units at the next contact. [Order 145](145-ssh-r-durable-dir.md) moved the
provider's state into a durable subdirectory outside the engine's lifecycle; this order builds
the drainer on it. Read PRV-021 in `spec/promises/providers.yaml`, the kit's README on
`drain_owed_stops` and exclusive ownership (`packages/runtime-provider-kit/README.md`), and the
crate's README, `src/main.rs`, `src/backend.rs` and `src/ssh.rs`. The engine's runner ends a
provider by killing its whole process group (`engine/crates/shell/src/runner.rs`,
`ProcessGroup::stop`), with no close of its input first, so no code of the provider runs after
its server is stopped.

## The decisions it applies

1. **The drainer starts with the provider.** Right after the provider takes its owner lock and
   checks `state-dir` (order 145, decision 2), before anything else that can refuse (the key's
   check, the part, `known_hosts`, the kit's recovery), it starts the drainer,
   `bin/provider-ssh-runtime-drain`, the sibling of its own executable, as a new session that
   outlives it (`setsid`), with standard input from `/dev/null` and both its standard output and its
   standard error appended to the drainer's log in the durable subdirectory, so that it holds no
   pipe of the engine's open. Where the drainer cannot be started, the provider refuses to start,
   naming the drainer: a provider serving with no drainer would leave its stops with none once it is
   killed. The drainer's environment is cleared and given only the `SSH_RUNTIME_*` names that the
   configuration reads and `PATH`: never `CONDUIT_URL`, `CONDUIT_IDENTITY_DIR` or a secret. It gets
   the durable subdirectory and the `CONDUIT_STATE_DIR` value as arguments. It leaves the engine's
   runner's process group by design (DL16 places such escape outside the runner's confinement); the
   contract says so.
2. **One drainer, then the owner lock.** The drainer first takes `drainer.lock` in the subdirectory,
   so that one drainer at most waits or works there. A drainer that waits for `drainer.lock` writes
   a `newer-drainer` file; a drainer that holds `drainer.lock` while it only waits for the owner
   lock exits when it finds that file, so that the newest drainer, with the newest configuration,
   serves the next retirement (a machine added since an older drainer started would otherwise be
   unknown to it). The drainer that takes `drainer.lock` removes the file. Then it waits for the
   owner lock, which the kernel releases when the provider exits, however it ends. Both waits poll
   with a non-blocking `flock` under tokio's time, never a blocking `flock` on a blocking thread.
   Holding the owner lock, it removes any `yield` file and builds the backend with the lock it holds
   (order 145, decision 6).
3. **What it does.** It reads the ledger afresh. Where the subdirectory or the ledger is gone, it
   logs that and exits, never reading it as "nothing owed". Where a stop is owed, it calls the kit's
   `drain_owed_stops` and reads the ledger afresh again, until nothing is owed, waiting between
   attempts with a backoff that starts at the drain interval and doubles to at most five minutes; an
   `Err` (an unreadable ledger) counts as a failed attempt. It stops trying once `maxLifetime` plus
   `stopTimeout` has passed since it last took the owner lock, by when systemd has ended every run
   it could still owe a stop for, after one last attempt; any stop still owed waits in the ledger
   for the provider's next start. It starts no run, serves no call and opens no gate.
4. **No key outlives the last stop it serves (the admin's decision, 2026-11-12).** Once nothing is
   owed, whether at first or after draining, and also when it stops trying, the drainer watches for
   at most a minute whether the provider's instance was retired: its `CONDUIT_STATE_DIR` gone
   (`symlink_metadata` answering not found) while that path's parent still exists, so that a moved
   or unmounted store never counts. It never canonicalizes or requires that path at its start, since
   it may already be gone. Where the instance was retired, nothing is owed and the subdirectory's
   `state-dir` still holds the path it was given, it deletes the private key and `known_hosts` from
   the subdirectory and exits: the ledger and markers stay, for the operator. Otherwise it exits
   keeping them, for the provider's next start.
5. **Yielding to a provider.** A provider that starts while a drainer holds the owner lock writes a
   `yield` file in the subdirectory, rewriting it on every poll, and waits at most 10 seconds for
   the lock, since it has not yet answered `initialize`. Before refusing, it deletes its `yield`
   file; then it refuses, naming the owner lock, as now. A drainer checks for the file at least once
   a second, also while it waits between attempts, and on finding it drops its in-flight
   `drain_owed_stops` at once (each stop's intent is saved before it is tried) and yields; a drainer
   that yielded tries the owner lock again, without blocking, after 11 seconds, and resumes where it
   is free; where it is held, it goes back to waiting as in decision 2, and never exits on a held
   lock, since the holder may be a sweep's brief try (order 147). The provider's wait for the lock
   is an asynchronous function of the crate's library on tokio's time, so that the tests run both
   sides in one paused runtime.
6. **Its exit order.** Before it releases the owner lock, the drainer drops its in-flight attempt
   and its backend, whose SSH masters are killed only when dropped, so that none acts beside the
   next owner; it never calls `process::exit` while they live.
7. **Its contract text,** in `contracts.yaml` and the README, replaces the deferral: the drainer,
   when it starts, waits and ends, its log, the locks and the yield; that it leaves the engine's
   runner's process group and survives only that group's kill, not a stop of the engine's whole
   service cgroup or a reboot, after which owed stops wait for the provider's next start; the key's
   deletion; the newest drainer superseding an older one; and that a provider retired while no
   drainer watched keeps its key until order 147's sweep, or the operator, removes it.
   PROVIDERS.md's note that decision D's drainer is deferred is outside your scope: the lane
   coordinator lifts it when this order merges.
8. **Tests,** with the fake remote and the kit's store, in the crate, by paused tokio time, never
   real sleeps:
   - a provider starts the drainer right after its owner lock and `state-dir` check, before a
     refusal it then makes (through a seam that records the start, its arguments and its
     environment, holding no `CONDUIT_` name); a drainer that cannot be started makes the provider
     refuse, naming it; a newer drainer supersedes an older one that only waits;
   - the drainer: it waits for `drainer.lock`, then the owner lock; with nothing owed it watches and
     exits; with a stop owed it carries it out once the machine answers, after failed attempts with
     growing waits; it stops at the bound after one last attempt, leaving the stop owed; with the
     subdirectory or ledger removed, it logs that and exits;
   - the key: with nothing owed and the state directory removed under an existing parent, it deletes
     the key and `known_hosts` and keeps the ledger and markers; with the state directory present,
     or its parent gone too, or `state-dir` changed, it keeps them; a provider that never ran a run,
     retired, has its key deleted;
   - yield: a provider starting while a drainer holds the lock gets it within the bound, the drainer
     dropping an attempt in flight (the fake gains a hold on a stop, as it has on staging); a
     drainer that yielded and finds the lock held goes back to waiting; a provider that gives up
     removes its `yield` file, and the drainer resumes; a stale `yield` file does not stop the next
     drainer; never two owners at once;
   - the drainer starts no run (the fake records no `systemd-run`), and the archive holds its binary
     and is the same, byte for byte, when built twice.
9. **What this order does not do:** the sweep of other subdirectories (order 147), change the
   kit, start any real process outside a test's temporary directory, test over a real `sshd` or
   `systemd`, or reach any machine.

## Scope

Change only `packages/provider-ssh-runtime/`.

## Done when

In the crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --release` (a skipped test is untested, never a pass), and
`cargo run --release --bin package` twice, with the same digest. From the repository root:
`python3 tools/check_docs.py` and `python3 tools/set_names.py --check`. Run these with `bash`.
Each prints nothing when it passes:

```bash
D=packages/provider-ssh-runtime
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rln -E "PRIVATE KEY|BEGIN CERTIFICATE" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
```

The provider's shell scripts stay POSIX, since the tests run them on macOS too (order 140's
Record). `target/` and `dist/` stay ignored and unstaged. Report each choice you make as your own.
Do not stage anything.


## Record

Run on 2026-11-12 on the Windows computer (WSL), from base e03ca3e: the worker (1,198 s) and a fresh
verifier (174 s), then a second worker round (289 s) and a fresh verifier (199 s). Both verifiers
found a race in how drainers handed over: round one's, an older waiting drainer superseding the
newest; round two's, a drainer's place set when it registered, which can come later than its
launch. [Order 149](149-ssh-r-drainer-finish.md) tried launch generations and found two more such
races, and [order 150](150-ssh-r-one-drainer.md) replaced the whole handoff (see its Record). The
drainer itself stands as this order built it: started with the provider, waiting on the owner
lock, draining owed stops with a backoff to its bound, yielding to a starting provider, and
deleting the key and `known_hosts` once the instance is retired and nothing is owed.

The worker's finding: the configuration allows a `drainInterval` above five minutes, while the
backoff starts at that interval and is capped at five minutes; left for SSH-R's loopback-tier
order.
