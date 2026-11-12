# Work order 129 (A63-RK, owed stops first): no run starts on a machine that owes a stop

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
token or a private key, even one that a test made.

## What and why

Model checking found (MC4-002) that a run whose stop did not succeed may run beside a later
instance's run on the same machine. The owner answered on 2026-11-12: the specification keeps
that as a stated limit, and every runtime provider Conduit ships carries out a stop it still owes
on a machine before it starts any run there, as the kubelet does, and its contract says so;
PRV-021 allows it.

The runtime-provider kit, `packages/runtime-provider-kit/`, already blocks a new start of the
same instance while that instance owes a stop (`src/server.rs`, `state_of`, near line 654). It
does not block a start of another instance on the same machine. This order closes that, in the
kit, for every runtime built on it.

Read first: PRV-021 in `spec/promises/providers.yaml`; the kit's `README.md` (its policies,
`drain` and `drain_owed_stops`) and `src/server.rs` (`execute`, near line 467, `drain`, and
`state_of`).

## The decisions it applies

1. **Owed stops first, machine-wide.** `run.v1` already drains owed stops before it acts. Then,
   after `repeated_run_policy` (`src/server.rs`, near lines 513 to 515) and before the old gate is
   closed (near line 516), so before the epoch and flight handling and before the candidate is
   saved, it checks whether any stop is still owed on the call's machine (the same hostname), for
   another instance, durably or in memory. If one is, the call starts nothing:
   - it answers `failed`, not `unknown`: the provider knows nothing started, and OPS-014 keeps
     `unknown` for an effect that may have happened (the admin's decision, 2026-11-12);
   - its message, the result's text and not a `reason` key (a result that states both an outcome
     and a reason reads as unknown, OPS-007), is an explicit diagnostic (ruling 52): it names
     every owed stop's instance ID on that machine and the machine's hostname, and says that the
     start may be tried again once those stops are carried out.

   A stop in progress counts as owed, since its intent is saved before the backend acts: rely on
   the drain's wait, and never take another instance's lock while holding this one's. Reuse the
   ledger read that the call already makes; if it fails, the answer is `unknown`, as it is today.
   The same instance's own owed stop keeps today's answer through `state_of`. A later `run.v1`,
   once the drain has carried the stops out, starts as usual. Stops owed on other machines do not
   block it. Nothing is staged, started or gated.
2. **The README** says it in the policy list, near `drain`: "A held stop is carried out before any
   new run on that machine starts; a start refused for this names the owed stop, and starts
   nothing." It also says that a runtime's contract must state this rule (the owner's answer to
   MC4-002, 2026-11-12); that on a provider of one machine a single owed stop blocks every new
   run there; and that a stop the startup sweep could not carry out is owed and blocks the machine
   until a drain carries it out.
3. **Tests**, with the fake backend:
   - instance A's stop cannot be carried out and is owed; `run.v1` for instance B on the same
     machine answers `failed`, its message naming A's ID and the hostname and saying the start may
     be tried again once that stop is carried out, with no `reason` key, and starts nothing; a
     repeated `run.v1` for a B already running answers its `url` as before; once the backend can act,
     the next `run.v1` for B drains A's stop and starts B;
   - an owed stop on another machine does not block B: the fake's declaration accepts one
     hostname only, so make A's owed stop on another hostname through `stop.v1`, which does not
     check the declaration, then run B on the fake's machine;
   - the same with the obligation held only in memory (a write failure and a stop failure
     together, as order 124 made it): keep the save failing through B's refused call, so that the
     drain cannot make the obligation durable, and assert that the ledger holds no owed record for
     A while B is refused;
   - two owed stops on the machine: the message names both.
4. **Nothing else changes.** No change to the backend traits, and no change outside the kit.

## Scope

Change only `packages/runtime-provider-kit/`.

## Done when

In `packages/runtime-provider-kit/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`

From the repository root: `python3 tools/check_docs.py`. Run these with `bash`. Each prints
nothing when it passes:

```bash
D=packages/runtime-provider-kit
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
```

`target/` stays ignored and unstaged. Do not stage anything.
