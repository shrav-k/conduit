# Work order 214: the sign-in hook's call names the baseline revision, so a URL reaches the relay

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty, since no case changes. This order has
network access only because the engine's tests bind loopback ports, which the sandbox refuses
without it; use it for nothing else, add no crate, and reach no other machine. Run one cargo
command at a time. Never print a whole large file, case, fixture or log: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Do not
run `claude`, `codex` or any client, and sign in nowhere.

## What and why

**The defect.** The coordinator's dry run of A5's live check, on 2026-11-19, ended `sign-in.v1`
`unknown` under PND-010 for both Agent packages of 0.2.0, with no URL relayed (the readiness
review's delta pass, its blocker B5; the coordinator's notes, private). The engine opens every
provider run with rmcp 3.9.0's legacy handshake, `().serve(transport)` in
`engine/crates/shell/src/runner.rs` (line 623 as written), which negotiates at most `2025-11-25`.
The 0.2.0 packages, on the lanes `wo/050-lane-codex` and `wo/050-lane-claude` until order 050n
merges them, answer the plain `sign-in-hook.v1` call with `CallToolResponse::InputRequired`, URL
mode. rmcp 3.9.0's server refuses that result on any request whose protocol version is older
than `2026-07-28`, with `invalid_request` (`rmcp-3.9.0/src/handler/server.rs:307-311`). The
engine reads the protocol error on a plain hook as unknown (`core/src/requests.rs`, the
`ProtocolError` arms of the completion), and `hook_ended` ends `sign-in.v1` unknown
(`core/src/sign_in.rs:1262`). IFC-009, IFC-010 and IFC-049 say the URL is relayed. No test caught
it: the lanes' sign-in test connects at `Discover { V_2026_07_28 }`, and the engine's sign-in tests
use engine-side scripts, not `runner.rs` against an rmcp server.

**The fix.** The core writes the baseline revision's request metadata on the sign-in hook's call,
as order 209 did for keyed hooks' `server/discover`, so that rmcp's server answers that one
request at `2026-07-28` on the session the legacy handshake opened. Every milestone-1 sign-in goes
through this path, so nothing of the live check or the dogfood's sign-ins runs until it merges.

## The decisions it applies

1. **Route (b), the narrower one** (the coordinator's choice, open to the owner's strike). rmcp
   3.9.0's server honours a request's own `_meta` version on a session that `initialize` opened:
   - `RequestContext::protocol_version` takes the request's
     `io.modelcontextprotocol/protocolVersion` first and the handshake's only without it
     (`src/service.rs:1239-1245`); the serve loop hands the request's `_meta` to the handler
     unchanged (`src/service.rs:1594-1605`).
   - `is_2026_07_28_or_later` is read from that (`src/handler/server.rs:88-90`), which is the
     test at `:307` that refuses `InputRequiredResult`.
   - A request naming a version checks it against the server's supported versions
     (`src/handler/server.rs:91-99`); the default is `KNOWN_VERSIONS` (`:448-450`), which holds
     `V_2026_07_28` (`src/model.rs:204-210`), and neither lane's wrapper overrides it.
   - A request naming `2026-07-28` or later must also hold
     `io.modelcontextprotocol/clientCapabilities` (`src/handler/server.rs:107-128`;
     `src/model/meta.rs:521-533`), and the server then reads the capabilities from it
     (`src/service.rs:1259-1266`), so the Claude wrapper's check for URL elicitation reads
     IFC-045's declaration there.
   - The `CallToolRequest` arm has no legacy check (`src/handler/server.rs:232-245`).
   - Order 209's Record found the same on the wire: after rmcp's default `initialize`,
     negotiated at `2025-11-25`, a `server/discover` carrying both keys was answered.

   Route (a), opening provider sessions with `ClientLifecycleMode::Discover` or `Auto`, is not
   taken. Beyond changing every provider session, `turn.v1` and the runtime providers included,
   it would not work alone: after a discover opener the server requires both keys on every request
   (`src/service.rs:1054-1056`, `src/service/server.rs:660`, `src/handler/server.rs:107-110`),
   and `runner.rs`'s `ExactCall` replaces the params rmcp fills with the core's exact params
   (`runner.rs:1043-1072`), so every call the core writes without them, plain calls and the
   `tools/list` refresh included, would be refused. `Discover` does not fall back to a legacy
   server (`src/service/client.rs:860`), and `Auto` waits up to 10 seconds for one (`:722`).

2. **Blast radius: the `sign-in-hook.v1` call alone.** Its `tools/call` carries, in `_meta`,
   `io.modelcontextprotocol/protocolVersion` `2026-07-28` and
   `io.modelcontextprotocol/clientCapabilities` (IFC-045's URL elicitation, and on a keyed call
   Conduit's extension as today), whether the call is plain or keyed, on its first call and on
   every re-send. The re-sends rebuild the call through `intent` (`core/src/requests.rs`, its
   callers near lines 1861, 2061, 2284 and 2765 as written); `retry_intent` is test-only for now
   (`#[allow(dead_code)]`). So write the keys where every `intent` call site picks them up, in
   `intent` or `dispatch_intent`, not only at the hook's first dispatch. No other call changes:
   the account hook, the sign-out part, setup, `turn.v1`, requests, `tools/list` and
   `server/discover` are sent as today, and every provider session still opens with the legacy
   handshake. IFC-007 lets a plain call carry `io.modelcontextprotocol/protocolVersion`.
   The two signIn-stating implementations of 0.2.0, the wrappers on the lanes, are rmcp `=3.9.0`
   servers on the default `KNOWN_VERSIONS`; no package on `main` states `signIn`. A server that
   declines `2026-07-28` would answer the sign-in hook with a protocol error, read as unknown: no
   worse than today for an `InputRequired` answer, a regression for a complete one, which no
   implementation in the plan has (the coordinator's, open to the owner's strike).

3. **Reading the hook's answer.** On a request at `2026-07-28`, rmcp keeps
   `resultType: "complete"` on a complete result instead of stripping it
   (`src/handler/server.rs:313-319`). The engine already reads a plain result whose `resultType`
   is absent or `complete` alike (`core/src/requests.rs:2117`); a sign-in hook that answers with a
   complete result keeps being read as today. Do not make any reading depend on `resultType`'s
   presence.

4. **The test** (the coordinator's reading of the delta pass's test, open to the owner's strike).
   The 0.2.0 wrappers are on unmerged lanes, not on `main`, and signing in through them needs the
   real clients, which a worker may not run. A stand-in on the same rmcp `=3.9.0` server exercises
   the same refusal and the same runner path, and the dry run repeated after the merge covers the
   real wrappers. An engine test runs a real rmcp 3.9.0 server, an `rmcp::ServerHandler` served
   over stdio from inside the engine's test tree, as a provider through `runner.rs`'s own spawn
   and handshake, with a Type that states `signIn`. Its account hook answers with no account and
   its sign-in hook answers `CallToolResponse::InputRequired` with one URL-mode elicitation whose
   URL matches the stated login (IFC-049). A `sign-in.v1` started at the engine must bring that URL
   to the Interface that asked, in the shell's input-required result (order 048b-relay1's relay).
   It fails on `main` as the dry run did, ending unknown under PND-010, and passes on the fix.
   Beside it, a core test that the sign-in hook's call carries both keys and that the account
   hook's `tools/call` toward the same instance (a plain account hook, or a keyed one's
   `tools/call` after its `server/discover`, which carries the key as today) carries no
   `io.modelcontextprotocol/protocolVersion`, and that a sign-in hook's call built by one of the
   re-sends of decision 2 carries both keys.
   How the server binary is built and found is yours; no new crate, and `engine/Cargo.lock`
   does not change.

5. **Stop and report, change nothing more,** if the test cannot pass without changing
   `runner.rs`'s handshake (the route is the coordinator's), if a case's expectations must change,
   or if a promise needs a word changed (`spec/` is the owner's).

## What to read

- `engine/crates/shell/src/runner.rs`: the spawn and handshake near `().serve(transport)`,
  `Wire` and `ExactCall`, and `Inner::call`.
- `engine/crates/core/src/requests.rs`: `intent` and `dispatch_intent`, the keyed discovery call
  (near the comment on rmcp 3.9.0 and `server/discover`), and the completion's arms for a hook;
  `engine/crates/core/src/sign_in.rs`: `sign_in_hook_part`, `hook_input` and `hook_ended`.
- The tests that drive a sign-in through the Interface in `engine/crates/shell/src/server_tests.rs`,
  and the scripted provider of `engine/crates/shell/tests/runner.rs`, as models.
- rmcp 3.9.0's source in cargo's registry, at the lines decision 1 cites:
  `"${CARGO_HOME:-$HOME/.cargo}"/registry/src/*/rmcp-3.9.0/`.
- IFC-007, IFC-009, IFC-010, IFC-045 and IFC-049 in `spec/interface/implementation.yaml`.

## Before you change anything

From the repository root, check that you find what this order was written against, and stop,
change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 33140121 HEAD; echo "main $?"
grep -c '().serve(transport)' engine/crates/shell/src/runner.rs
grep -rh -c '"io.modelcontextprotocol/protocolVersion":"2026-07-28"' engine/crates/core/src/requests.rs
grep -c 'rmcp = { version = "=3.9.0"' engine/crates/shell/Cargo.toml
grep -c 'InputRequiredResult requires negotiated protocol version 2026-07-28' "${CARGO_HOME:-$HOME/.cargo}"/registry/src/*/rmcp-3.9.0/src/handler/server.rs
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `1`, `1`, `1` and `0`.

## Scope

Change `engine/crates/core/src/` and the engine's tests (`engine/crates/shell/src/` test modules
and `engine/crates/shell/tests/`), and `engine/STANDING.md` only if `promise_standing.py` asks
for it. `runner.rs` changes only in a test module, if at all. Change nothing under `packages/`,
`spec/`, `conformance/`, `docs/` or `experiments/`, and not `engine/Cargo.lock`. Anything else
you find that needs changing goes in your report.

## Done when

Write the test of decision 4 first and run it on the unchanged engine: report its failing line.
Then, from the repository root, each passes:

```sh
(cd engine && cargo test --release)                           # the whole suite, conformance included
(cd engine && cargo test --release -p conduit-shell --test <file> <name>)  # 1 passed
(cd engine && cargo test --release -p conduit-core <name>)                # 1 passed
(cd engine && cargo clippy --all-targets -- -D warnings)
git add -N <each new file>                                    # so the checks below see it
python3 tools/promise_standing.py --check
python3 tools/check_docs.py                                   # Errors: 0
python3 tools/check_private_paths.py                          # Errors: 0
git diff --check                                              # prints nothing
python3 tools/vector_delta.py HEAD WORKTREE --declared-changed  # 0 changes, 0 errors
git diff --name-only HEAD -- packages spec conformance engine/Cargo.lock | wc -l | tr -d ' ' # 0
grep -c '().serve(transport)' engine/crates/shell/src/runner.rs                  # 1
git reset -q -- <each new file>                               # staged nothing
```

The two named tests are decision 4's: the shell's, with the test file and name you chose
(`--lib` in place of `--test <file>` if it sits in a test module), and the core's; write the
names into your report.

Your report lists each test's name and the shell test's failing line on the unchanged engine,
then each passing line; where the core writes the two keys and how it knows the call is the
sign-in hook's; every choice you made yourself; and every check's last line. Stage nothing. The
coordinator then has a fresh, read-only Opus reader read the diff (ruling 79), runs the gate and
merges; it then rebuilds the live check's engine from the merge and repeats the dry run (the
coordinator's notes, private).

## Record

Run on 2026-11-20, ending about 00:43 UTC, on the Windows node from `main` `201005c7` (order 050n's
refresh), in one round (ruling 87): the worker (1792 s) and a fresh Codex verifier (299 s). The
change is two files of Scope. In `engine/crates/core/src/requests.rs`, `dispatch_intent` writes
`io.modelcontextprotocol/protocolVersion` `2026-07-28` into the call's `_meta` when the invocation
is a hook and its operation is the core's qualified `sign-in-hook.v1`, beside the
`clientCapabilities` that IFC-045's URL declaration already writes for a Type that states `signIn`;
every dispatch and re-send is built there, through `intent`, so the first call and each re-send
carry both keys, plain or keyed, and no other call changes. The core test
`sign_in_hook_uses_baseline_revision_on_dispatch_and_resend` checks that the account hook's call
names no revision, that the plain and the keyed sign-in hook's calls carry both keys, and that the
`refresh-tools-and-call` re-send rebuilds the keyed call. In `engine/crates/shell/tests/runner.rs`,
the ignored test `rmcp_sign_in_provider` serves an rmcp 3.9.0 `ServerHandler` over stdio, whose
account hook answers no account and whose sign-in hook answers `InputRequired` with one URL-mode
elicitation; `sign_in_relays_url_from_rmcp_stdio_provider` spawns it through the runner's own spawn
and unchanged legacy handshake, with the plain-hook Type and manifest of the requests fixtures, and
checks that `sign-in.v1` brings the URL to the Interface. The worker's own choices: the fixture as
an ignored test of the same executable, launched by exact name; the plain-hook fixtures with a
command override; a read-only contract to exercise the re-send path, which the real hook's effect
precludes under IFC-046. On the unchanged engine the shell test ended `sign-in.v1` unknown under
PND-010, as the dry run did. The whole suite at default concurrency was killed by SIGKILL in the
conformance vectors' executable (`target/release/deps/vectors-*`); rerun with one harness thread and
two vector workers it passed, 402 passed, 0 failed and 5 ignored; Clippy passed.

The Codex verifier found two musts, two shoulds and a nit: (1) a non-plain sign-in hook now names
the baseline revision without IFC-037's discovery or the extension's declaration; (2) the fixture,
run through libtest, prints libtest's `running 1 test` on the stdout that carries MCP; should 1, the
fixture ignores the request's capabilities, so the test would pass if the URL capability were lost
on the wire; should 2, the fixture lists no tools; and a nit, an absolute path in the worker's
report.

A fresh read-only Opus reader then verified the order (ruling 79). It ran both named tests on the
Mac and on the node (1 passed each); removed the three lines of the fix in place and saw the shell
test fail at `crates/shell/tests/runner.rs:536:9` with `expected a relayed sign-in URL, got outcome
unknown under PND-010` on both machines, and the core test at `requests.rs:2960` (left `Null`, right
`2026-07-28`), restoring with `cp` and `cmp`; and on the node the core suite (148 passed), the
runner tests (18 passed, 3 ignored), the shell's sign-in lib tests (14 passed), the conformance
vectors (2 passed, 295 s), Clippy and `cargo fmt --check` (clean). `promise_standing --check`,
`check_docs` and `check_private_paths` gave Errors: 0, `git diff --check` was clean, the vector
delta 0 changes, nothing changed under `packages/`, `spec/`, `conformance/`, `docs/`,
`experiments/`, `engine/Cargo.lock` or the shell's sources, and `().serve(transport)` still counted
one. It refuted the verifier's must 1 (decision 2 sets the blast radius as the sign-in hook's
call, plain or keyed, and a non-plain call carrying Conduit's keys without discovery is order 209's
recorded gap, neither widened nor in scope), must 2 as a must (the fact holds, but it is the file's
established fixture pattern, the runner skips non-JSON lines, and the 0.2.0 wrappers print no such
line), should 2 (no path that lists tools reaches a sign-in hook) and the nit (the report is not
tracked). It found no must and four shoulds:

- S1: the stand-in sign-in hook ignores its `RequestContext` and elicits whatever the call declares,
  so the test would pass if the URL elicitation capability were dropped between the core and the
  wire (the verifier's should 1).
- S2: the core test proves a re-send carries both keys only through the `refresh-tools-and-call`
  path, which a real sign-in hook never takes, and not the call sent after a keyed hook's
  `server/discover` answer.
- S3: the worker reported the failing line on `main` as `runner.rs:521`; the committed test fails at
  `runner.rs:536:9`.
- S4: `engine/CONFORMANCE.md` does not record that the sign-in hook's call alone names `2026-07-28`
  per request on a session opened by the legacy handshake.

The fixer rebased the branch onto `origin/main` at `6ccb3471` (order 215, written ahead) with the
work-order identity, without conflict, and applied each finding:

- S1: the fixture names its context and, in the sign-in hook's arm, refuses with `invalid_request`
  `no URL elicitation declared` when `context.client_capabilities()` holds no `elicitation.url`.
  With the fix the shell test passes. With the three lines of the fix removed it fails at
  `crates/shell/tests/runner.rs:547:9` (536 before the 11 lines S1 adds) with `expected a relayed
  sign-in URL, got outcome unknown under PND-010`. With the core declaring `elicitation` as `{}` in
  place of `{"url": {}}`, it fails at the same line with the same message, where the fixture as the
  worker wrote it passed.
- S2: the core test also pushes the keyed sign-in hook with its discovery pending, answers its
  `server/discover` with a result that lists Conduit's extension at `v0`, and checks that the call
  sent is a `tools/call` naming `2026-07-28` and equal to the keyed call, IFC-045's URL capability
  and the extension included.
- S3: this Record cites the failing line as `crates/shell/tests/runner.rs:536:9` on the worker's
  commit, and the verifier's SIGKILL by the executable's name only.
- S4: the keyed-hooks bullet of `engine/CONFORMANCE.md` gains the reader's sentence: "From order
  214, the sign-in-hook.v1 call, plain or keyed and on every re-send, carries
  io.modelcontextprotocol/protocolVersion 2026-07-28 beside IFC-045's clientCapabilities, so that an
  rmcp 3.9.0 server opened by the legacy handshake may answer it InputRequired (IFC-007, IFC-010);
  no other call names a revision." This is outside the worker's scope, as the reader noted, and is
  the coordinator's record pass.

The mutations ran in place in the worktree, each restored with `cp` and checked with `cmp`. The
fixer then ran, in the worktree on the Mac with `RUST_TEST_THREADS=4` and
`CONDUIT_VECTOR_THREADS=4`: both named tests (1 passed each), `cargo test --release -p conduit-core`
(148 passed), the runner tests (18 passed, 3 ignored), the shell's sign-in lib tests (14 passed),
`cargo clippy --all-targets -- -D warnings` (clean) and `cargo fmt --all -- --check` (exit 0), then
`python3 tools/check_docs.py` and `python3 tools/check_private_paths.py` (Errors: 0 each) and `git
diff --check` (clean). The gate's run on this order's squash is the coordinator's at the merge.
