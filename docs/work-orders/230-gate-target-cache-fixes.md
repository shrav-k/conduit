# Work order 230: two defects in the gate's warm build (order 223's R1), found in real merges

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access.

## What and why

Order 223 (merged, #467) gave `tools/gate.py` the option `--target-cache DIR` (R1, the testing
audit's warm build, DEC-033). Its first two real merges, of orders 225 and 224 on 2026-11-20,
both failed with the option on, for two reasons, and the merge queue has turned the option off
until this order lands:

1. **The cached path metadata asks for every platform.** Before a directory's first cached row,
   the gate runs `cargo metadata --format-version 1 --offline` (`tools/gate.py`, the call near
   line 1246) to find its path crates. Without `--filter-platform`, cargo resolves the
   dependencies of every platform, and the offline cache lacks some (the engine's log: "failed to
   download `wasm-bindgen-futures v0.4.80` ... --offline was specified"). So the engine's clippy
   and test rows end with exit 101 in under a second. Order 221's start check already uses
   `--filter-platform "$(rustc -vV | sed -n 's/^host: //p')"` for the same reason.
2. **A package's own archive test compares two different builds.** With `--target-dir`, the test
   binary and `CARGO_BIN_EXE_*` come from the cache, while the test's own inner `cargo` build of
   the package's archive writes to the package's `target/`. `packages/provider-keychain`'s
   `deterministic_host_archive_metadata_and_digest` then fails with "archive binary differs".
   Eleven packages have such a `tests/package.rs`.

**Should this exist:** yes. It fixes a merged order's defect, found by the dogfood, so the warm
build the owner approved can be used at all. It adds nothing new.

## Decisions

1. The cached path metadata call names the host platform with `--filter-platform`, the host
   taken from `rustc -vV` run in that directory (its pinned toolchain), as order 221 does. A
   failure of that host query fails the row closed, as a metadata failure does today.
2. Under `--target-cache`, every test that builds or compares its own package binary or archive
   passes as it does without the option. How is yours. One way is to give a package directory the
   cache through `CARGO_TARGET_DIR` instead of `--target-dir`, so its tests' inner builds share
   it. Read each directory's tests first, `engine/crates/shell/tests/agents.rs` among them, since
   it builds the Agent packages inside the engine's test. Keep R1's guarantees:
   - the lock is always released;
   - the path crates' fingerprints are invalidated before a directory's first row;
   - no build of another worktree can pass a gate.
3. No other behavior of the gate changes. Without `--target-cache`, the gate behaves exactly as
   today.

**Stop rule.** Change nothing under `spec/`, `engine/`, `packages/` or `conformance/`; a fix
that needs one, stop and report.

## Scope

`tools/gate.py`, `tools/tests/test_gate.py`, and this order's row in `docs/work-orders/README.md`
if its text needs one. Nothing else.

## Before you change anything

```sh
git merge-base --is-ancestor 59cf7ce9 HEAD; echo "main $?"
grep -c "'metadata', '--format-version', '1', '--offline'" tools/gate.py
grep -c 'filter-platform' tools/gate.py
ls packages/*/tests/package.rs | wc -l | tr -d ' '
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `0`, `11` and `0`.

## Done when

```sh
grep -c 'filter-platform' tools/gate.py                       # 1 or more; fails today (0)
/usr/bin/python3 -m unittest discover -s tools/tests          # OK, with a new test for each decision
```

Then prove both fixes on real directories with a cache outside the repository:
- `python3 tools/gate.py --target-cache "$C"` on a scratch commit that touches
  `packages/provider-keychain` and one engine file, run twice;
- each run ends `Gate: PASS`, and the second compiles fewer crates.

Report both runs' last lines and their compile counts. Remove the scratch commit, its worktree
and the cache. `git diff --check` prints nothing.

## Record

Written by root on 2026-11-20 after the warm build's first two real merges failed, and run through
Conduit itself, by the dogfood's own Agents, in Pacific time. The text merges with the build, and
it had no separate read.

`turn-worker-2.turn.v1` (15:54 to 16:23) fixed both defects. Its cold and warm scratch gates each
passed 19 checks, and Cargo's compile entries fell from 789 to 5. `turn-verifier-2.turn.v1`
(16:23 to 16:28) found one must, which its own probe reproduced: fingerprints in Cargo's
target-specific profile directories were not invalidated, so an explicit native target could
pass on another tree's build. The second turn fixed it, with a two-tree regression that failed
first. 140 tool tests pass.

`turn-reader-1.turn.v1` (16:34 to 16:38), the Claude reader, accepted it with one should: the
two real cached gates were not repeated after the second turn. Root takes the next real merges,
with the merge queue's `--target-cache` back on, as that proof. A defect there fails a gate and
does not pass one, and the false-pass case has its own regression. The gate's run on the squash
is the coordinator's, at the merge.
