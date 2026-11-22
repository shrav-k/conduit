# Work order 268: s2_dependent_call_after_close orders its close by the barrier, not by the clock

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no conformance case changes. The order
needs no network: the only sockets it opens are the loopback ports its tests bind. Every crate it
builds is in the local cache: run cargo offline, and if a build asks for a crate that is not
there, stop and report, and fetch nothing. Run one cargo command at a time.
Never print a whole large file, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential, even one a test made. Do not run
`conduit` outside a test, `npm`, `node`, `codex`, `claude`, the dogfood harness or `tools/gate.py`,
and sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write
is public: no absolute local path, user or machine name, or private repository name.

## What and why

**The failure.** On 2026-11-22 `s2_dependent_call_after_close` (`engine/crates/shell/tests/s2.rs`)
failed in the whole-suite runs of orders 261, 257 and 262, none of which changes an engine file it
reaches, each while another worker's whole suite ran on the same machine:

```text
did not close and release within the aware request deadline: Elapsed(())
```

It passed each of three runs alone. Under
[DEC-046](../decisions/log.md#dec-046-2026-11-21-the-fleet-grows-on-every-account-by-measurement)
more workers share this machine; a test that fails under load stops the gate and the workers.

**Today.** The test arms the stub's barrier on the read of `SYN-1` for 4000 ms. Beside the read,
inside one two-second `timeout`, it waits up to 1500 ms for the barrier to be reached, closes A's
`read` Conduit (a new `conduit` process) and releases the barrier. The two seconds are the aware
package's: `packages/scenario-s2-linear` gives its HTTP requests "a two-second connect and response
timeout". But all three clocks start before the read is sent, so they also count its way to the
stub, which the promise does not time. The failures show only that the way took under 1.5 s (a
slower one fails first at the wait's 408); how the rest split between the way and the close is
unknown. After this order, a failure at decision 2's bound points to the hold (What follows).

**The promise.** V5 of [experiment 013](../../experiments/013-s2-shared-login/README.md) (ACC-034,
ACC-039, IFC-020): an invocation dispatched through a Conduit ends after it closes, and a later
call through it is refused. The barrier makes the order: it holds the read until the test
releases it, and the test closes in between.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2, every
access can be revoked, which the test keeps pinning; none at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: a test of asynchronous work waits on the event it needs and keeps a
timeout only as a guard, not as the measure of the work (Martin Fowler, "Eradicating
Non-Determinism in Tests", 2011). Not taken: raising the two seconds, moot under the package's
own timeout; closing before the barrier is reached, another moment than V5's; changing the
package, outside a test-only order.

**When it runs.** On `main` from #529. Through Conduit's dogfood: one Codex worker turn (aim for
under an hour), a blind Codex verifier and Claude reader, one fix turn, the gate (DEC-046,
DEC-033). By ruling 89, each decision says what must hold; how is yours. The bounds, the
constant's name and the mutation runs are the coordinator's, open to the owner's strike.

**Stop rule.** Stop, change nothing further, and report where:

- the start check prints otherwise (its last line: see below);
- the changed test fails alone, or a mutation run ends otherwise than Done when says: report the
  line, and loosen no other bound;
- a correct fix needs a change outside the scope.

On the unchanged test only (the start check's last line, M1 and M2 today), an `Elapsed` line
may be the load this order fixes: run `t` (Done when) once more and report both lines. Stop only
on another failure line, or on a second `Elapsed` when `pgrep -x cargo | wc -l` printed `0` just
before it; otherwise go on.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **Only `WAIT` times the read's way to the stub.** `/barrier/wait`'s `timeout_ms` is half of
   `WAIT`, the harness's progress deadline (20 s): under the control client's own `WAIT`, so a
   read that never arrives fails with the stub's 408, not `stub request failed`. The arm
   deadline, counted from the arm, covers the way and the hold: `WAIT`, within the stub's limit of
   60 000 ms, so it ends after both other bounds (10 s + 2 s) and never first. Derive both from
   `WAIT`, not as literals.
2. **One bound, on the hold only, from the aware deadline.** After `/barrier/wait` answers
   `reached`, the close and the release run inside one `timeout` of a constant
   `AWARE_REQUEST_DEADLINE`, two seconds, beside `WAIT`, its comment citing the package's response
   timeout; keep the `expect` message. It is the window in which the held read can still
   succeed, and past it the message names the cause where `issue` would fail with none. It starts
   after the wait answers, the package's before the stub is reached, so it never ends first; a
   longer one would catch nothing more.
3. **Every assertion stays, in its order:** `READ` among A's tools before the arm, the `reached`
   and `released` states, `issue(&answer, "SYN-1")`, the succeeded `invocation-ended` record,
   `check_refused` on the later read, `h.count()` of 1, and `finish(.., 1, 1, 1, true)`. The
   comment in `S2::new` on the "two-second in-flight-close window" still holds.

## What to read

The test, `WAIT`, `S2::control`, `S2::root` and `close` in `s2.rs`; the stub's barrier rules
(experiment 013's `linear-stub/README.md`, `src/barrier.rs`); the package README's timeout line;
ACC-034.

## Before you change anything

From the repository root; a line that prints otherwise is the stop rule's:

```sh
git merge-base --is-ancestor 70dd302f HEAD; echo "main $?"
s=engine/crates/shell/tests/s2.rs
grep -c -e '"timeout_ms":4000' -e '"timeout_ms":1500' -e 'timeout(Duration::from_secs(2), async {' "$s"
grep -c 'two-second connect and response timeout' packages/scenario-s2-linear/README.md
git grep -l 'AWARE_REQUEST_DEADLINE' -- engine | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
(cd engine && cargo test --offline --release -p conduit-shell --test s2 s2_dependent_call_after_close 2>&1 | grep -c '^test result: ok\. 1 passed')
```

They print `main 0`, `3`, `1`, `0`, `engine 0`, `0` and `1`.

## Scope

Change only `engine/crates/shell/tests/s2.rs`: `s2_dependent_call_after_close` and the new
constant. Report anything else that needs changing.

## Done when

Each mutation is a temporary edit of the test, run once with `t` below and reverted at once. Run
M1 and M2 on the unchanged test first, for today's line, then all three on the changed one.

- **M1, a slow way to the stub:** the read inside `tokio::join!` becomes
  `async { sleep(Duration::from_secs(5)).await; h.read(&a, "SYN-1").await }`, past today's arm
  deadline. Today it fails with the wait's 408 at about 1.5 s; after, it passes only if both
  bounds left the way.
- **M2, a close that takes no effect:** the line `close(&h, &route).await;` removed. Today and
  after it fails at the same line, `check_refused`'s `unwrap_err`: the later read is answered.
- **M3, a hold past the package's deadline:** `sleep(Duration::from_secs(3)).await;` between the
  close and the release. After, it fails with decision 2's `expect` message.

```sh
ok='^test result: ok\. 1 passed'
t() { (cd engine && cargo test --offline --release -p conduit-shell --test s2 s2_dependent_call_after_close 2>&1); }
for i in 1 2 3; do t | grep -c "$ok"; done  # 1, three times
t | grep -E -m3 'panicked|expect|^test result'  # once per mutation run; as M1 to M3 say
s=engine/crates/shell/tests/s2.rs
grep -c 'AWARE_REQUEST_DEADLINE' "$s"  # 2; today 0
grep -c -e '"timeout_ms":4000' -e '"timeout_ms":1500' -e 'timeout(Duration::from_secs(2), async {' "$s"  # 0; today 3
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vx "$s" | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-shell --test s2 2>&1 | tail -3)  # the whole S2 file, once, last
```

The whole engine suite is the gate's. Report what changed, the start run's line, each mutation
run's line before and after, the three passing lines and every check's last line.

Stage nothing.

## What follows

Named here and not in this order:

- The hold, a `conduit` process and a release, must fit the package's two seconds. If the test
  fails at decision 2's bound, a later order chooses the package's timeout or a close through a
  connected root session.
- The same pattern in other shell tests, unchanged here: `github.rs`'s `WAIT`; the 5-second waits
  in `http_bridge.rs` and `local_process.rs`; `runner.rs`'s `dispatch_deadline_` tests and
  `deadlines_exit_repair_write_and_cancel`; `s2_filter_current_read`.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-2's turn** (02:20 to 02:40): the barrier's arm and wait now take their bounds from
`WAIT` (20 s and 10 s), outside any short timeout, and one two-second bound,
`AWARE_REQUEST_DEADLINE`, starts only once the stub reports the read reached, covering the close
and the release. Every assertion is kept, in its order. The mutation runs gave the results the
order names: a delayed read fails today and passes now; removing the close fails the later read's
refusal before and after; a slow hold fails at the kept bound. The whole s2 file passed, nine
tests, three focused runs alone.

**Two blind reads at once** (02:40 to 02:44): `turn-verifier-2.turn.v1` passed it by its diff
(its own rerun could not build the fixtures read-only); the Claude reader passed it with one nit,
left to a later order: the two derived bounds and the constant could say in a comment why each is
what it is. No fix turn.

The gate's run on the squash is the coordinator's, at the merge.
