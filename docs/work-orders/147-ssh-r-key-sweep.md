# Work order 147 (SSH-R): a sweep of retired providers' keys

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no other machine. Run one
cargo command at a time, and never print a whole large file or log. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never print a key, a token
or a certificate's key, even one that a test made.

## What and why

[Order 145](145-ssh-r-durable-dir.md) keeps each SSH runtime provider's ledger, owner lock and
private key in its own subdirectory of an operator's `durableDir`, outside the engine's lifecycle,
and [order 146](146-ssh-r-drainer.md)'s drainer deletes the key and `known_hosts` once the
provider's instance is retired and nothing is owed: the admin's rule that no key outlives the last
stop it serves. The drainer catches a retirement only within a minute of the provider's end. A
provider retired later, as after an engine restart with no run of it since (PRV-026), keeps its
key. This order adds a sweep that catches those, whenever another provider starts with the same
`durableDir`. Read orders 145 and 146 and the crate's README.

## The decisions it applies

1. **When it runs.** Every provider start, once it holds its own owner lock and has checked its
   own `state-dir` (order 145, decision 2), looks at the other subdirectories of `durableDir`.
2. **What it deletes.** For each other subdirectory, it tries that subdirectory's owner lock
   without blocking, skipping it where the lock is held, and holds it through what follows. It
   deletes the private key and `known_hosts` there only where all hold: the `state-dir` file names
   a path that is gone (`symlink_metadata` answering not found) while that path's parent still
   exists, as order 146's retirement test has it; and the ledger reads, owing nothing. A missing
   ledger or `state-dir` counts as owing, and the sweep leaves that subdirectory as it is. It
   keeps the ledger and markers, logs each deletion and each skip in its own diagnostics, and
   then releases that lock. It never removes a subdirectory.
3. **What it never does.** It deletes nothing in its own subdirectory, starts no drainer for
   another subdirectory, and waits on no lock. A failure while sweeping one subdirectory is
   logged and skips it, and never stops the provider's start.
4. **Its contract text,** in `contracts.yaml` and the README: the sweep, when it runs and what it
   deletes, and the gap that stays: a provider retired while no drainer watched and with no other
   provider sharing its `durableDir` keeps its key until the operator removes it.
5. **Tests,** in the crate: a sibling with a free lock, a removed state directory under an
   existing parent and nothing owed loses its key and `known_hosts` and keeps its ledger and
   markers; a sibling with a held lock, a stop owed, a present state directory, a state
   directory whose parent is gone too, a missing ledger or a missing `state-dir` keeps them; the
   provider's own subdirectory is never swept; and a failure in one sibling skips it and the
   provider still starts.
6. **What this order does not do:** change the kit or the drainer, start any real process outside
   a test's temporary directory, or reach any machine.

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
