# Work order 141 (A63-RK): a start after its own owed stop, under order 081

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
token or a private key, even one that a test made.

## What and why

Order 081 settled one run per instance in `spec/promises/providers.yaml`: PRV-019 (a start where
the provider cannot tell whether an earlier run of the ID is running starts none and ends
unknown), PRV-021 (a stop held and carried out later, and a contract that carries out every owed
stop on a machine before starting any run there) and PRV-030 (a stop succeeds only once no run of
the ID is running or being started, and no run starts after it until `run.v1` next asks). Read
them, then the runtime-provider kit's README, `packages/runtime-provider-kit/`, and
`src/server.rs`. PRV-021's last sentences (near lines 661 to 664) say that, under such a contract,
a start that finds a stop still owed fails, naming it, without telling the start's own instance
from another's. Where the start's own earlier run may still be running, PRV-019 makes the call
unknown, and the admin's decision 2 below follows PRV-019; the admin corrects PRV-021's sentence
in a spec order. Report the difference as a finding, and change nothing in `spec/`.

The kit has one fault against them, found when order 081 was written. `run.v1` reads its
instance's epoch (`src/server.rs`, `execute`, near line 534), then drains owed stops before the
call. Any drain, this call's, another call's or the interval's, that takes up a stop this
instance owes from an earlier `stop.v1` advances the instance's epoch (`drain`, near line 439,
through `successful_stop_policy`) before it tries the stop, whether the stop then succeeds or
not. Where it succeeds, the start then answers `failed`, "start cancelled by stop" (near line
602), though nothing stopped it after it began. Where it fails, the start answers `unknown`,
"run may still be running" (`repeated_run_policy`, near line 211), before that check, without
naming the stop that holds it.

## The decisions it applies (the admin's, 2026-11-12)

The admin also decided, on the same day, the two extensions in decisions 1 to 3 that a fresh review
of this order raised: every drain cancels a start's flight without advancing its epoch, so only
`stop.v1` and shutdown do; and where the start's own state cannot be read, the answer is
`unknown`, naming every owed stop on that machine, `failed` for another instance's stop holding
only where the start's own state is readable (OPS-014's honesty, applied alike; it settles order
137's finding that such an answer named no stop). The admin's next spec order puts the same three
cases into PRV-021's text.

1. **A drain does not cancel a start.** A stop owed before this `run.v1` began, whichever drain
   carries it out, does not cancel this start: it proceeds. A `stop.v1` for the instance that
   arrives while the start is in flight still cancels it, as PRV-030 requires. `stop.v1` advances
   the epoch itself before any await (near line 536), and a drain can only advance it in the window
   before `run.v1` takes the instance's operation lock (near line 545). So at the drain (near line
   439), cancel the instance's flight, its token and serial, without advancing the epoch: only
   `stop.v1` (near line 536) and shutdown (near line 464) advance it. Keep the change to that one
   function; take no new lock across an await.
2. **The start's own owed stop, not carried out: `unknown`.** The earlier run may still be running,
   so the honest outcome is `unknown` (PRV-019, OPS-014). The message keeps "run may still be
   running" at its start, then names the owed stop's instance ID and the machine's hostname, and
   says the start may be tried again once that stop is carried out (ruling 52). `state_of` (near
   lines 756 to 774) folds an owed stop and an unreadable state into one `Unknown`, so take the owed
   records from the call's records (near lines 573 to 581), which include stops held only in memory
   (`slot.pending()`), and name each with its own hostname, which may differ from the call's
   machine. Where the state is unknown, the message also names every other instance's owed stop on
   the call's machine. Where nothing is owed, the message stays as now.
3. **Another instance's owed stop on the machine: `failed`,** naming it, exactly as order 129 made
   it, where the start's own state can be read; unchanged. Where it cannot, as on an unreachable
   machine, the start answers `unknown` under decision 2, naming the stop.
4. **The contract text.** The README states all three: what a start does when its own drain carries
   out its owed stop; the two answers when an owed stop cannot be carried out, its own (`unknown`)
   and another instance's (`failed` where the start's own state can be read, else `unknown`), each
   naming the stop and the machine; and that a `stop.v1` arriving while a start is in flight cancels
   it, while a drain's stop owed before the start began does not. Its sentence that "a queued start
   overtaken by a stop is cancelled too" (near line 180) says `stop.v1`. Replace the README's
   sentence that order 081 settles the specification text (near line 172) with a reference to
   PRV-019, PRV-021 and PRV-030 as order 081 wrote them.
5. **Tests,** deterministic, with the fake backend in `tests/support` (add hooks there only):
   - an instance with an owed stop that the start's own drain carries out: the start proceeds and
     succeeds, and exactly one stop and one start reach the backend, in that order;
   - the same, with the stop failing: `unknown`, the message naming the instance and the machine,
     no staging and no start;
   - a `stop.v1` arriving while the start is paused in staging, and one while it is paused in
     `start`: the start is cancelled and nothing runs after the stop succeeds;
   - an owed stop, with the start's own drain paused in the backend's `stop` (`pause_next_stop`,
     `stop_entered`): a `stop.v1` arrives, then the drain resumes; the start fails, and no start
     reaches the backend after the stop succeeds; and another call's drain carrying out an owed
     stop in the window before the start takes its operation lock does not cancel it. The
     existing `stop_overtakes_run_paused_in_initial_ledger_read` (`tests/runtime.rs`, near line
     658) stays as a guard; the test near line 1448, which expects "start cancelled by stop" for
     exactly decision 1's case, is replaced. A call-order hook in `tests/support` shows the stop
     reaching the backend before the start;
   - another instance's owed stop on the machine: `failed`, naming it, as order 129's tests have
     it.

   Keep every other test of orders 122, 124, 126, 129, 130 and 138 passing.
6. **Dependents.** `packages/provider-local-process/` and `packages/provider-ssh-runtime/` build
   and pass their tests with no change to any of their files.
   The kit's version stays 0.1.0 and it gains no dependency.

## Scope

Change only `packages/runtime-provider-kit/`.

## Done when

In `packages/runtime-provider-kit/`: `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`. In each dependent named in
decision 6, changing nothing there: `cargo clippy --all-targets -- -D warnings` and
`cargo test --release`; then, from the repository root, `git status --porcelain packages/` shows
changes under `packages/runtime-provider-kit/` only. From the repository root also:
`python3 tools/check_docs.py` and `python3 tools/set_names.py --check`. Run these with `bash`. Each
prints nothing when it passes:

```bash
D=packages/runtime-provider-kit
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
git status --porcelain --ignored $D | grep -E "target/" | grep -v '^!!'
```

`target/` stays ignored and unstaged. Report each choice you make as your own. Do not stage
anything.
