# Work order 227: S2-3, S2's dry run on the engine: the Linear stub, the shared login from the file store, two scripted Agents, and S2's vectors V1 to V6

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` each test of decisions 3 and 5 by name, with its
promises and `passes`. This order needs no network: it fetches nothing, and the only sockets it
opens are loopback ports (the engine's, the stub's and the tests' own). Every crate it builds is
in the local cache already; if a build asks for one that is not, stop and report, and fetch
nothing. Run one cargo command at a time. Never print a whole large file, case, fixture or log.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Every secret in this order is a
synthetic marker that a test makes at run time and never prints, the stub's tokens among them.
Do not run `conduit` against a store outside the tests, `codex`, `claude`, `security` or the
dogfood harness; touch no keychain; sign in nowhere.

## What and why

S2's stream, step S2-3, after orders 218 (S2-1) and 224 (S2-2) and before S2-4, the live run
with the owner. [Experiment 013](../../experiments/013-s2-shared-login/README.md) pre-registers
S2: one service login shared by two Agents with different restrictions. Its P1 needs S2 end to
end on the engine with its vectors V1 to V6 passing; its P2 needs real Claude Code and Codex
Agents and the real service, at S2-4.

**The dry run** is S2 end to end with nothing real but the engine.
[The Linear stub](../../experiments/013-s2-shared-login/linear-stub/README.md) stands in for
Linear on loopback. [The Conduit-aware implementation](../../packages/scenario-s2-linear/README.md)
is the service's Resource; its run gets the shared login by order 218's read from
[the file store](../../packages/provider-file/README.md), a secret the Pool owns. Two Agents,
whose clients are the test itself, each with its own token, hold different restrictions of
[the restriction contracts](../../packages/scenario-s2-linear-restrictions/README.md) (013's O3:
explicit issue IDs with a deadline). It shows offline, before the owner's sitting, that what
S2-4 runs works, and finds what does not.

Orders 218 and 224's "What this leaves" give S2-3 V3, V4, SEC-016, SEC-019 and, by 224, the
engine's reads through the keychain store, which move to S2-4 (decision 6).

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
3, a shared thing is owned once and never copied to an Agent (the login is the Pool's and reaches
the Linear run alone); 2, every access is a relationship that can be inspected and revoked (each
Agent's access is a filtered Conduit, closed and restored in the tests); and 7, everything is
attributable (each read and call is recorded with its caller).

**Should this exist** (point 3). Testing against a local stand-in for a service is the
existing practice (LocalStack for AWS), and every part exists: the stub, the aware
implementation, the file store and the pattern of `tests/secrets.rs` and `tests/serve.rs`. The
order adds tests, not a tool. P1 and ruling 92 point 8 make it Conduit's.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for
under 2 hours), a Codex verifier and a Claude reader, who run only what settles a doubt, and the
gate ([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). By
[ruling 89](../proposals/ruling-89/README.md), each decision says what must hold; how is yours.

**Stop rule.** Change nothing under `spec/`. Where a test needs a choice that `spec/` does not
make, or a fix beyond decision 4, stop on that test alone: leave it out, change nothing for it,
report the promise, the choice and the options, and finish the rest. If 3 hours pass with
tests still unwritten, write no more, leave them out under this rule, and report which. Stop
the whole order
where anything would reach a host other than loopback. A choice no promise governs is yours:
list it under `own_choices`.

## The decisions

The shape (engine tests, not portable cases; decision 6's split) is the coordinator's, open to
the owner's strike.

1. **The project.** One Pool owns two entries:
   - `login`, a `file-secret.v1` Resource at a regular file of mode 0600, outside the store,
     holding the stub's synthetic API key;
   - `linear`, a `linear-aware.v1` Resource. Its `baseUrl` is the stub's loopback GraphQL URL,
     and its `apiKey` is `login`'s reference. It declares a Conduit to `login` that lists
     `get-secret.v1` and is not optional.

   The test builds and installs `provider-file`, `scenario-s2-linear-restrictions` (whose
   archive's digest the aware package pins) and `scenario-s2-linear` the way `secrets.rs`'s
   `pack` builds `provider-file`. `scenario-s2-linear`'s package tool needs its server built
   first: run `cargo build --offline --locked --release --bins`, then
   `cargo run --offline --locked --release --bin package`. It runs the stub's binary, built
   offline, with a fixture of synthetic issues SYN-1 to SYN-4. Root's requests go through the
   `conduit` binary, as an author's would; test machinery such as the Agents' tokens may use
   the shell's library, as `serve.rs` and `secrets.rs` do.
2. **The Agents, scripted.** Two Agent instances of a Type that starts no client (the S1
   fixture's metadata-only Agent Types, as `real_npm_serve_s1_tokens_access_t7_and_shutdown`
   uses, or one of yours). Each Agent's client is an MCP client inside the test, at the engine's
   loopback address with that Agent's own token, as that test's `connect` is. No Agent package,
   model or real client runs. Each restriction is an `issue-scope.v1` filter on a Conduit to
   `linear`:
   - `agent-a` reads SYN-1 and SYN-2 (`issue-reading.v1`), with a deadline the test sets;
   - `agent-b` reads SYN-2 and SYN-3, and writes SYN-3 (`issue-writing.v1`), with deadlines
     past the test's end.

   SYN-2 is the shared issue; SYN-4 is in neither scope. Neither Agent has a Conduit to
   `login`, or a tool that reaches it (013's G1).
3. **The tests**, in `engine/crates/shell/tests/s2.rs`. Each runs its own stub and store and
   holds to these rules:
   - The markers are the stub's API keys, old and new, and decoys, compared and never printed.
   - At each test's end, no marker is in any MCP result, tool list or notification an Agent
     received, in any record or inspection, in any byte of any file under the store
     (recursively), or in any stdout or stderr of a `conduit` command the test ran.
   - No record has `linear` as caller: it sends the engine no request (SEC-019).
   - Each run of `linear` follows exactly one read of `login`, recorded without the value, and
     no Agent reads `login`.
   - Waits are bounded, and no deadline wait exceeds 30 seconds. Leave `testClock` unset, so
     deadlines run on the real clock: a set `testClock` is one fixed instant for as long as the
     Resource's configuration stands.
   - The stub's control routes set the service's authorization, rotate the key and hold calls
     at barriers. The new key is `s2-stub-` followed by a v4 UUID, the only form `rotate-key`
     accepts. The test makes it with the shell's `uuid`, since no Cargo.toml changes.
   - No test returns early on an environment variable: each runs in a plain `cargo test`.

   At least these tests, by these names, with these promises:
   - `s2_one_login_two_restrictions` (S2-T1; SEC-010, SEC-018, SEC-019, IFC-019, ACC-001):
     - both Agents read SYN-2 and get the same issue;
     - B updates SYN-3, and the stub's issue changes;
     - A has no update tool, and a call by A of an update tool's name gets a protocol error;
     - A's read of SYN-3 is refused by the filter, and its result carries the restriction;
     - neither refusal reaches the stub.
   - `s2_filter_current_read` (V1, E02; OPS-009, OPS-024, OPS-034, ACC-036, IFC-019):
     - A's keyed read of SYN-1 succeeds before its deadline;
     - after the deadline, the same keyed request is dispatched again as a new attempt and the
       filter refuses it, with no stored output in its result;
     - with the stub's service authorization hiding SYN-2, B's keyed repeat of a read of SYN-2
       that had succeeded is dispatched again. The new attempt is answered failed, so the
       invocation's outcome is unknown, not failed (OPS-024), and no stored output is given;
     - B's keyed repeat of its succeeded update reports conflict, and the stub's count does
       not move;
     - each invocation's outcome is as OPS-024 states.
   - `s2_bound_allows_unfiltered` (V2, E07; ACC-006, ACC-029, ACC-030, CON-022):
     - root registers, with A as consumer, an unfiltered `issue-reading.v1` Conduit and an
       `issue-writing.v1` Conduit to `linear`, neither open. A holds a Conduit to `access` that
       lists `open.v1`, with a bound naming `linear` in `targets` and `read-issue.v1` and
       `list-issues.v1`, by qualified reference, in `operations`;
     - A opens the reading one, and its read of SYN-4 succeeds through it;
     - A's open of the writing one is refused, with the bound in
       `io.github.shrav-k.conduit/restriction`, and that Conduit stays registered, not open
       (ACC-006, ACC-029);
     - inspection shows A's two open Conduits to `linear`, the filter on one alone.
   - `s2_rotate_replace_repair` (V3, S2-T5 and T6; SEC-010, SEC-014, SEC-018, SEC-019,
     CON-008):
     - both Agents read; then the key is rotated at the stub and in `login`'s file;
     - the running `linear` keeps the old key (SEC-014), so its next calls fail, and no Agent
       result holds either key;
     - root retires `linear`, which closes both Agents' Conduits to it, asks for a replacement of
       the Pool's entry by `register-replacement.v1`, and sets the replacement up. Its run
       follows its own read, which gets the new key, and its calls succeed;
     - root restores A's access, then the engine is stopped and started again before B's is
       restored (the interrupted restoration). Inspection and B's call show B without access
       while A reads (D08's nested variant in [the case README](../../conformance/README.md));
     - root repairs B's access explicitly, and both read again;
     - the records name each close, the replacement and each restoration.
   - `s2_secret_read_fails_closed` (V4; SEC-006, SEC-012, SEC-015, SEC-020):
     - with `login`'s file at mode 0644, and then removed, `linear`'s setup reads and the read
       fails. Its run is not started: no process, and the stub's count stays 0. Its run part
       reports unavailable, and no Agent call reaches the stub;
     - `conduit serve`'s own environment holds a decoy under `S2_LINEAR_API_KEY`, and the Pool
       has a second `file-secret.v1` entry holding the right key, which `linear` does not
       reference. Neither is ever read or delivered;
     - with the file back at 0600, a further setup reads and the run starts.
   - `s2_dependent_call_after_close` (V5, its first half; ACC-034):
     - A's read of SYN-1 is held at a stub barrier, and root closes A's Conduit while it is
       held. The aware implementation ends an HTTP request after 2 seconds, so wait for the
       barrier's `reached`, then close A's Conduit and release the barrier within that time;
     - released, the invocation ends succeeded, its outcome is recorded, and A receives the
       result;
     - A's repeat of the read then reports refused, with no stub request;
     - V5's onward half (ACC-039, IFC-020) needs onward requests, which neither S2 row
       sends: report it as not exercised.
   - `s2_cached_tool_refused` (V6; ACC-001, ACC-011, IFC-012, IFC-013, IFC-025, IFC-038):
     - root closes A's Conduit. A's session is told its tool list changed, and the new list
       lacks the tool;
     - A's call by the cached tool name is answered as IFC-012 and ACC-011 state, recorded,
       with no stub request;
     - a call of a name never presented is answered as IFC-012 states for that case;
     - both differ from a tool failure: B's read of an in-scope issue the stub hides fails;
     - B's calls succeed throughout; root restores A's access, and A reads again.
4. **Fixes.** A test may fail because the engine or the aware package departs from a promise.
   Fix that in `engine/crates/core/src/`, `engine/crates/shell/src/` or
   `packages/scenario-s2-linear/src/`. Each fix gets a test of its own that names the promise and
   fails first, and all fixes together stay under 200 lines outside tests. A larger fix, or one
   that needs a choice `spec/` does not make, falls under the stop rule for that test.
5. **SEC-016**, in `engine/crates/shell/src/tests.rs`:
   `a_read_whose_dispatch_record_cannot_be_kept_is_not_dispatched`. The store's test fault
   (`WriteFault`, DL2) fails the commit that would hold a read's dispatch record. Then:
   - the read is not dispatched, and the store's server receives no `get-secret.v1` call;
   - no run is asked for, no value is anywhere, and memory equals disk;
   - the read reports unavailable.

   The last may need a choice SEC-016 and DL10 leave open (DL10's row in
   engine/CONFORMANCE.md: a store write error stops that shell operation as an embedding error,
   and no intent or successful reply is issued for it): what the caller receives when
   nothing can be written, or how the setup in progress ends. If so, test the rest, change
   nothing for it, and report the gap and the options.
6. **What stays for S2-4 and later.** Report this list, corrected by what you built.
   - Real Claude Code and Codex Agents, signed in through Conduit (P2; O2 at the S2 sitting),
     and what each client shows its model (E17, M1).
   - The real workspace's narrow key (S2-O) in the owner's dedicated keychain, read by
     `provider-keychain` once order 224 merges, with A63-OKC's authorization (O4, O5, "A:
     dedicated only"). This corrects 224's "What this leaves": the dry run touches no keychain.
   - D01's plain and adapter rows, D05, D09 on the keychain store, D20 for real Agent
     processes, local and isolated, and M0, in the counted runs S2-R1 to S2-R4.
   - V2's plain-filter half, V5's onward half, and 013's G1 and G3.

## What follows

- In `engine/CONFORMANCE.md`, statement 2, the two sentences "S2 has yet to run. It is to
  qualify the shared login with the Linear implementation (FR-480, G3 verifier's list)." become
  one sentence beginning "From order 227, S2's dry run". It says that the dry run, against the
  Linear stub, delivers the shared login from a Pool's file-store secret to the aware
  implementation's run alone, for two scripted Agents with different restrictions, and that S2
  has still to run with real Agents and the real service (FR-480).
- In `tools/gate.py`, add `packages/scenario-s2-linear/`,
  `packages/scenario-s2-linear-restrictions/` and `experiments/013-s2-shared-login/linear-stub/`
  to `ENGINE_PREFIXES` and `ENGINE_WHY`, since the engine's tests now read them. Add their cases
  in `tools/tests/test_gate.py`.
- At the end of experiment 013's README, under "Amendments since acceptance", add this line:

  ```markdown
  - **2026-11-20, the coordinator's (order 227):** V1 to V6 are engine tests in
    `engine/crates/shell/tests/s2.rs`, on the Linear stub with the file store and two scripted
    Agents, not portable cases: they assert on real processes. V2's plain-filter half and V5's
    onward half are not run there; the keychain, real Agents and the service are S2-4's.
    V4's unkept dispatch record (SEC-016) is the unit test
    `a_read_whose_dispatch_record_cannot_be_kept_is_not_dispatched` in
    `engine/crates/shell/src/tests.rs`.
  ```
- Run `python3 tools/promise_standing.py --write`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 59cf7ce9 HEAD; echo "main $?"
test -e engine/crates/shell/tests/s2.rs; echo "s2 $?"
grep -c 'fn file_store_read_is_run_local_and_decided_again_after_restart' engine/crates/shell/tests/secrets.rs
grep -c 'fn real_npm_serve_s1_tokens_access_t7_and_shutdown' engine/crates/shell/tests/serve.rs
grep -c 'pub(crate) enum WriteFault' engine/crates/shell/src/store.rs
grep -c 'sha256:6380ce0e6a7db56712f457fd3a56aa7a6802ce3e019c56a3433eda440feac43e' packages/scenario-s2-linear/conduit.yaml
grep -E '^\| SEC-01[69] \|' engine/STANDING.md | grep -c '| none | no case, no reason |$'
grep -c -e "'packages/scenario-s2-linear/'" -e "'experiments/013-s2-shared-login/linear-stub/'" tools/gate.py
grep -c 'to run. It is to qualify the shared login' engine/CONFORMANCE.md
for d in engine packages/provider-file packages/scenario-s2-linear packages/scenario-s2-linear-restrictions experiments/013-s2-shared-login/linear-stub; do (cd "$d" && cargo metadata --offline --locked --format-version 1 --filter-platform aarch64-apple-darwin >/dev/null 2>&1); echo "offline $?"; done
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `s2 1`, `1`, `1`, `1`, `1`, `2`, `0`, `1`, `offline 0` five times, and
`0`. An `offline` line that prints otherwise means a crate is missing from the cache.

## Scope

Create `engine/crates/shell/tests/s2.rs` (and a helper under `tests/common/` if shared).
Change `engine/crates/shell/src/tests.rs`, `engine/CONFORMANCE.md`, `tools/gate.py`,
`tools/tests/test_gate.py` and experiment 013's README as above, `engine/STANDING.md` only by
the tool, and, under decision 4 alone, `engine/crates/core/src/`, `engine/crates/shell/src/` and
`packages/scenario-s2-linear/`'s `src/` and `tests/`.

Change nothing under `spec/`, `conformance/`, `docs/`, other packages or `experiments/` beyond
that line, and not `IN_FORCE.md`, `ROADMAP.md`, `engine/Cargo.lock` or any `Cargo.toml`. Report
anything else that needs changing, such as a package README the dry run makes stale.

## Done when

Each line fails on `main` at `59cf7ce9` and passes after. A test left out under the stop rule
leaves its line failing: the order is then not done, and your report says why.

```sh
(cd engine && cargo test --release -p conduit-shell --test s2)  # 7 or more passed, 0 failed; fails today (no test target s2)
(cd engine && cargo test --release -p conduit-shell --test s2 -- --list 2>/dev/null | grep -cE '^s2_(one_login_two_restrictions|filter_current_read|bound_allows_unfiltered|rotate_replace_repair|secret_read_fails_closed|dependent_call_after_close|cached_tool_refused): test$')  # 7; fails today (0)
(cd engine && cargo test --release -p conduit-shell --lib a_read_whose_dispatch_record_cannot_be_kept_is_not_dispatched)  # 1 passed; fails today (0 passed)
grep -E '^\| SEC-016 \|' engine/STANDING.md | grep -cE '\| (core, )?shell \| no case, no reason \|$'  # 1; fails today (0)
grep -o -e "'packages/scenario-s2-linear/'" -e "'packages/scenario-s2-linear-restrictions/'" -e "'experiments/013-s2-shared-login/linear-stub/'" tools/gate.py | sort -u | wc -l | tr -d ' '  # 3; fails today (0)
tr '\n' ' ' < engine/CONFORMANCE.md | grep -c 'From order 227, S2'  # 1; fails today (0)
grep -c 'to run. It is to qualify the shared login' engine/CONFORMANCE.md  # 0; fails today (1)
grep -c "2026-11-20, the coordinator's (order 227)" experiments/013-s2-shared-login/README.md  # 1; fails today (0)
```

Run `git add -N engine/crates/shell/tests/s2.rs` before the next block and `git reset -q` on it
after, so that the checks see it; stage nothing. These pass, as on `main`; run the whole suite
once, last:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd packages/scenario-s2-linear && cargo test --release)  # only where decision 4 changed it
python3 tools/promise_standing.py --check  # Errors: 0
python3 tools/check_docs.py  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
python3 -m unittest discover -s tools/tests  # OK
grep -cE '/usr/bin/security|[Kk]eychain|api\.linear\.app' engine/crates/shell/tests/s2.rs  # 0
git diff --check  # prints nothing
git diff --name-only HEAD -- spec conformance docs IN_FORCE.md ROADMAP.md engine/Cargo.lock | wc -l | tr -d ' '  # 0
git diff --name-only HEAD -- packages experiments ':(exclude)packages/scenario-s2-linear/src' ':(exclude)packages/scenario-s2-linear/tests' ':(exclude)experiments/013-s2-shared-login/README.md' | wc -l | tr -d ' '  # 0
git diff --name-only HEAD -- '*Cargo.toml' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, s2 among it
```

Your report gives each test's result, time and final stub count; each fix with its promise
and test; SEC-016's result, with any gap and its options; every gap or departure found, under
`findings`; your choices, under `own_choices`; decision 6's list, corrected; and every check's
last line.

The verifier's turn follows. The coordinator then runs `tools/gate.py`, has a fresh Claude
reader read the diff ([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4), and
merges.

## Record

Run on 2026-11-20 through Conduit itself, by the dogfood's own Agents, in Pacific time.
Worker-1's first turn (16:24 to 17:24) ran S2 end to end, and all seven tests passed. It found
the IFC-037 gap that order 231 then fixed, and an SCH-015 redaction defect. Root set the turn
aside until 231 merged. The second turn (18:26 to 18:54) rebased onto 231, kept 231's IFC-037
and dropped its own. Root widened the scope to the by-rule snapshot.

`turn-verifier-1.turn.v1` (18:55 to 19:01) found seven musts. The key one: the redaction fix
unmasked four registration checks the engine never had. Five by-rule examples that had errored
only through the defect would have been accepted:
- CTR-006, controls with overlapping pointers (two examples);
- PRV-016, a package-source control that changes its schemes;
- SEC-001, a literal where a secret reference is required;
- SEC-024, a secret header to a non-loopback HTTP endpoint.

Root widened the scope to the engine's resolution and registration code. The third turn (19:01
to 19:49) added the four checks, each naming its promise. Those rows now read `named invalid`,
and equal-defaults stays accepted. The turn also fixed OPS-024's unanswered earlier attempt and
two S2 tests that could pass without showing their claims. It moved V1's deadline past setup,
after one whole-suite run failed on it.

`turn-reader-1.turn.v1` (19:49 to 19:54), the Claude reader, accepted it with a should and a nit:
- The fixes run to about 335 lines outside tests, past decision 4's 200. Root accepted that, for
  the four checks the verifier found (CTR-006, PRV-016, SEC-001, SEC-024) and the OPS-012 and
  OPS-024 outcome tracking.
- The loopback test reads the raw authority, so `http://127.1/` is refused. That is the safe
  side, and it is left to a follow-up.

Root also updated `packages/scenario-s2-linear/README.md`, which still called the engine's
secret read unbuilt. SEC-016's reply during a store write fault stays open for a spec order.
The worker's two options are a transient unavailable with recovery, or a durable unavailable
once writes recover. The gate's run on the squash is the coordinator's, at the merge.
