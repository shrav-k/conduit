# Work order 149 (SSH-R drainer, finished): the launch generation set by the provider

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no other machine. Run one
cargo command at a time, and never print a whole large file or log. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never print a key, a token
or a certificate's key, even one that a test made.

## What and why

[Order 146](146-ssh-r-drainer.md) built the SSH runtime provider's stop-drainer,
`packages/provider-ssh-runtime/src/drainer.rs`, in two rounds. Your working tree begins at its last
commit. Order 146's decisions all still apply; read it, then `src/drainer.rs`, `src/startup.rs` and
how the provider starts the drainer. Its final verifier left one must: a drainer takes its place in
the order of drainers when it registers (`src/drainer.rs`, `register`, near line 190), and
registration can come later than launch. An older drainer delayed before it registers can take a
higher number than a newer one, and then makes the newer one exit as superseded; the older one's
configuration then serves the next retirement, and stops on a machine added since cannot be drained.

## The decisions it applies

1. **The provider sets the generation.** While it holds the owner lock at startup, before it
   starts the drainer, the provider takes the next launch generation from a durable counter in the
   subdirectory, written atomically, and passes it to the drainer as an argument. A missing
   counter reads as 0; a corrupt one, or one that would overflow, makes the provider refuse to
   start, naming the file, and is never reset, since a reset would let older drainers beat newer
   ones. The drainer never assigns or raises a generation. Remove `register` and
   `drainer-order.lock` (`src/drainer.rs`, near lines 16 to 39 and 186 to 199).
2. **Only a running newer drainer supersedes.** A drainer waiting for `drainer.lock` exits at once
   where the counter is above its own generation, since a newer drainer has been or is being
   started. Each waiter writes its own generation into `newer-drainer` on every poll. The drainer
   that holds `drainer.lock` exits as superseded only where `newer-drainer` names a generation
   higher than its own, so that a newer drainer is actually running; a counter raised by a
   provider whose drainer then failed to start, or that was killed before starting it, never makes
   the holder exit, and the stops keep a drainer.
3. **What may change.** The signatures this needs change: `prepare`'s `start` and
   `Launch.arguments` (`src/startup.rs`, near lines 24, 37, 86 and 92), the drainer binary's
   argument check (`src/bin/provider-ssh-runtime-drain.rs`, near line 7), `drainer::run`, and in
   `tests/drainer.rs` the assertion near line 160 and `worker()` near line 86. Update the contract
   text that describes the order of drainers (`contracts.yaml`, near lines 121 to 126; README, near
   lines 127 to 132 and 240 to 241).
4. **Tests,** by paused tokio time and the manual-poll pattern of `tests/drainer.rs` (near lines
   243 to 266), with two `prepare` calls: an older drainer delayed before its first poll while a
   newer provider starts and its drainer polls first; when the older one resumes, it exits and the
   newer one survives and serves; a counter raised and the drainer's start then failing, the old
   drainer keeping its place and serving; a corrupt counter refused, naming the file; the
   generation always rising across provider starts, also after a restart. Every other test of
   order 146 still passes.

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
