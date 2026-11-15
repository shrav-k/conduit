# Work order 152: the engine's CI stands alone, and CI builds every crate

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates;
use it for nothing else, and reach no other machine. Run one cargo command at a time, and never
print a whole large file or log. Never run `env`, `printenv`, `set` or `export -p`, and never print
an environment variable's value.

## What and why

The full review of 2026-11-14 (order 3 of [its orders](../proposals/full-review/orders.md))
found two gaps in `.github/workflows/checks.yml`
([FR-493](../proposals/full-review/findings/G4-who-verifies.md#fr-493), gates 3 and 4;
[FR-352](../proposals/full-review/findings/P07-engine-build.md#fr-352)):

- The engine's test shards and doctests need the job `rust-dir`, which is one matrix over the
  engine, every package and every experiment. A failure in any package skips every engine shard:
  since pull request 260 a flaky SSH-R test hid the engine's results more than once, and order 077
  merged on local evidence.
- The matrix finds `Cargo.toml` one level below `packages/` and `experiments/` only, so
  `experiments/013-s2-shared-login/linear-stub` and `experiments/015-s4-partial-setup/scripted-client`
  (orders 135, 142 and 148) have never been built or tested by CI.

## The decisions it applies

1. **The engine gets its own build job.** A new job builds the engine alone: the toolchain, the
   cache, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, the nextest archive and
   its upload, as the `engine` leg of `rust-dir` does now. `rust-dir`'s matrix no longer holds
   `engine`. `engine-tests` and `engine-doc` need the new job only, never `rust-dir`, so that no
   package or experiment can skip them. The engine's cache key stays the same, so that `engine-doc`
   still restores it.
2. **The summary gate stays one name.** The job `rust` still needs every Rust job, the new one
   included, and still fails when any of them failed, was skipped or was cancelled: its stable
   name is what a branch rule can require.
3. **The matrix finds crates one level deeper.** Under `packages/` and `experiments/`, a directory
   whose `Cargo.toml` sits one or two levels down is a matrix entry, except under any `target/`.
   An experiment's entries are built and tested, never linted, as now; a package's are linted too.
   A crate that is a member of a workspace above it is not listed twice: list only directories
   whose `Cargo.toml` is not a workspace member (no such crate exists today; say so if you find
   one).
4. **The two crates CI never built pass their own tests.** In each, `cargo test --release` passes.
   Where one does not build or a test fails, make the least fix inside that crate and say what it
   was; an experiment is evidence, so change no expected result, and if a test fails because of
   what it shows, not because of its code, stop and report it instead of changing it.

## Scope

Change only `.github/workflows/checks.yml`, and, where decision 4 needs it, files inside
`experiments/013-s2-shared-login/linear-stub/` and `experiments/015-s4-partial-setup/scripted-client/`.

## Done when

- `/usr/bin/python3` loads `.github/workflows/checks.yml` with PyYAML, and a short check you write
  and run (and do not commit) shows: no job that `engine-tests` or `engine-doc` needs is the matrix
  job or any job that builds a package or an experiment; `rust` needs every Rust job; and the
  discovery step, run as a shell script from the repository root, lists both nested crates and no
  `target/` directory, and lists `engine` in no matrix entry.
- In each nested crate: `cargo test --release` passes. In `engine/`: `cargo fmt --check` passes.
- From the repository root: `python3 tools/check_docs.py` and `git diff --check`.

The admin then shows, on the pull request, the new engine job, the four shards, the doctests and
both nested crates passing. Do not stage anything. Report each choice you make as your own, and
every check's last line.

## Record

Run on 2026-11-14 on the Mac from `main` `b1aacc08`: the worker (216 s) and a fresh Codex verifier
(164 s), which found nothing to change in one round. The engine builds in its own job,
`engine build` (fmt, clippy, the nextest archive), and the four engine shards and the doctests
need only it, so no package or experiment can skip them; the `rust` gate needs every Rust job and
fails on any that failed, was skipped or was cancelled. Discovery lists `Cargo.toml` one or two
levels under `packages/` and `experiments/`, skips `target/` and, by `cargo locate-project
--workspace`, any workspace member: 19 entries, among them the two nested crates, whose release
tests passed unchanged (14 and 17 tests). A fresh read-only Opus reader verified the order before
it merged (ruling 79, point 2) and found nothing to change.
