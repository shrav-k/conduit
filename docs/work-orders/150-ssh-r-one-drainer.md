# Work order 150 (SSH-R drainer, simplified): one drainer, reading the newest configuration

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no other machine. Run one
cargo command at a time, and never print a whole large file or log. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never print a key, a token
or a certificate's key, even one that a test made.

## What and why

[Order 146](146-ssh-r-drainer.md) built the SSH runtime provider's stop-drainer, and
[order 149](149-ssh-r-drainer-finish.md) tried to make the newest drainer, with the newest
configuration, the one that serves, by launch generations and announcements. Each round of each
order found another race between drainers handing over (order 149's last verifier: an obsolete
waiter can take the lock and erase a newer drainer's announcement). Your working tree begins at
order 149's last commit. This order replaces that machinery with a simpler rule, under which no
two drainers ever compete: at most one drainer exists per subdirectory, and it reads the newest
configuration each time it takes the owner lock. Orders 145 and 146's other decisions all still
apply; read them, then `src/drainer.rs`, `src/startup.rs` and the drainer tests.

## The decisions it applies

1. **The configuration travels by file.** The provider writes its configuration atomically to
   `drainer-config`, mode 0600, in the subdirectory, once `SshBackend::new` has succeeded and
   before `RuntimeServer::new`, so that a provider that refuses at startup (a mistyped key, say)
   never overwrites a working configuration. It writes only the places its configuration schema
   declares, each a path, a reference or a setting, never a secret value (no place of the schema
   is a secret place, SEC-001; the private key is a path to a file, never its contents). The 0600
   mode is defence in depth, not the only guard; a test checks that no value written is a secret
   marker the test placed elsewhere. A failed write refuses startup, naming the file, after the
   drainer has started (order 146, decision 1). The drainer reads `drainer-config` each time it
   takes the owner lock and builds its backend from it; its own environment keeps only `PATH`.
   Where `drainer-config` is missing or invalid, it logs that, naming the file, releases
   `drainer.lock` and then the owner lock, and exits.
2. **At most one drainer, kept alive by the provider.** The drainer holds `drainer.lock` for its
   whole life: while it waits for the owner lock, while it drains and while it yields. On every
   exit it releases `drainer.lock` before it releases the owner lock. A drainer holding
   `drainer.lock` never exits while it waits, except where its subdirectory or ledger is gone; any
   other error it logs and retries. The provider starts a drainer at startup as order 146 says,
   and then, for its whole life, tries `drainer.lock` without blocking at every poll (once a
   second): where it is free, it releases it at once and starts a drainer, logging a failed start
   to its diagnostics; where it is held, a drainer is alive or about to be. So a drainer killed
   while the provider runs, one that died before taking `drainer.lock`, or a lock the kernel
   released late, is replaced within a poll. A drainer started while another holds `drainer.lock`
   exits at once.
3. **Remove the generations, and what they replaced.** Remove the launch counter, the generation
   argument, the `newer-drainer` announcements, `handoff.lock`, `drainer-order` and every
   supersede path, with their tests and the contract text that describes them. This replaces
   order 146's decision 1 where it gives the drainer the `SSH_RUNTIME_*` values in its
   environment, decision 2's wait for `drainer.lock` and its `newer-drainer` file, decision 7's
   newest drainer superseding an older one, and decision 8's supersede test. Keep order 146's
   yield (a yielding drainer keeps `drainer.lock` and waits again), its bound, its watch for
   retirement and its exit order.
4. **Tests,** by paused tokio time and manual polls: a provider starting while a drainer waits
   starts none, and that drainer, once it takes the owner lock, serves with the newest
   `drainer-config` (a machine added at the second start is drained); a provider that refuses
   after its backend fails leaves the previous `drainer-config` in place; a provider starting with
   no drainer alive starts one; a drainer aborted while it waits and a provider runs is replaced
   within one poll; a drainer started while another holds `drainer.lock` exits at once; a drainer
   that exits releases `drainer.lock` before the owner lock; a drainer that yields keeps
   `drainer.lock` and serves again; a drainer killed at each point, while it waits, while it
   drains and while it yields, after which the next provider start brings up exactly one new
   drainer and the owed stops are still carried out; a missing or invalid `drainer-config`
   logged by name; and every other test of orders 145 and 146 still passes.
5. **Its contract text,** in `contracts.yaml` and the README: one drainer per subdirectory, kept
   alive by the provider's poll, the configuration it reads and when, and what holds
   `drainer.lock`.

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

The provider's shell scripts stay POSIX, since the tests run them on macOS too. `target/` and
`dist/` stay ignored and unstaged. Report each choice you make as your own. Do not stage anything.


## Record

Run on 2026-11-12 on the Windows computer (WSL), stacked on order 149's last commit (f803d96): the
worker (648 s) and a fresh verifier (125 s), which found nothing to change in one round. At most one
drainer exists per subdirectory: it holds `drainer.lock` for its whole life, releasing it before the
owner lock on every exit, and the provider, for its whole life, tries `drainer.lock` once a second
and starts a drainer wherever it is free, so a drainer that dies is replaced within a poll. The
provider writes its configuration, paths and references only, to `drainer-config` after its backend
starts, and the drainer reads it each time it takes the owner lock. The launch generations,
announcements, `handoff.lock` and `drainer-order` of orders 146 and 149 are removed. On the Mac,
with two test threads: `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` clean,
all 63 tests pass, and two archive builds give the same digest
(`sha256:8f3639aac01afa7feb11006b129077e4d136ea44ef61afea729ebdcd250431ab`). The lane coordinator's
review found nothing to change.

The worker's own choices: `drainer-config` as JSON in the schema's names and types, validated by
the configuration's own parser; a keep-alive task beside the provider's startup, service and
shutdown; an ordered guard that releases the two locks in order, also on cancellation.
