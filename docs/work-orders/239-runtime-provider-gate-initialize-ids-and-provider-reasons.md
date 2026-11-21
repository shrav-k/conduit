# Work order 239: order 080's last three findings: the kit's gate initializes a run's server once and frees a cancelled ID in a later session, and a provider's reason reads as unknown

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate, and the only sockets it opens are the loopback
ports its tests bind. Every crate it builds is in the local cache already: run cargo offline, and
if a build asks for a crate that is not there, stop and report, and fetch nothing. Run one cargo
command at a time. Never print a whole large file, diff, log or list: use `head`, `tail`, `grep`,
`sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential, even one a test made.
Do not run `conduit` outside a test, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and
sign in nowhere. Every file you write is public: write no absolute local path, no user or machine
name and no private repository name.

## What and why

**The findings.** Order 080's Record
([at `8e13bd81`](https://github.com/shrav-k/conduit/blob/8e13bd812eaeaed969898dc67cf48ba1aeccc3c2/docs/work-orders/080-shell-runtime-provider-calls.md#record))
lists seven findings. Orders 226 and 231 closed four; order
[226](226-engine-runtime-provider-local-process-qualified.md)'s "What follows" carries the other
three, which this order takes, in 080's words:

1. "The gate forwards a repeated `initialize` to the run's persistent stdio server. The bridge
   always sends a fresh one, so for servers that cannot be initialized twice the gate must answer
   a repeated initialization itself."
2. "A static review found that the kit keeps cancelled request IDs and refuses their reuse with
   409. This was not tested."
3. "IFC-026 gives no reason form for provider calls."

**Each still holds on `main` at `c1d09afc`.**

1. `State::http` in `packages/runtime-provider-kit/src/gate/session.rs` ends the old session on
   every `initialize` and writes the new one to the run's stdio; the kit's README says "the
   runtime must use a server that tolerates this". The engine starts a new bridge, and so a new
   MCP client that sends `initialize`, each time it reaches a run again: `run.v1` answering with
   an existing run (PRV-019), or an inspection that finds the run `running` with no live bridge
   or at a new url (PRV-009, IFC-017). MCP has initialization be "the first interaction between
   client and server" ([lifecycle](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle)),
   so a stdio server may refuse a second. The bridge then fails to initialize, and the part
   reports unavailable while the run runs.
2. The same `State` keeps one `cancelled` set for the channel's life and answers an empty 409 to
   a request with an ID in it, in any later session. The engine's client, rmcp 3.9's, numbers
   each session's requests from 0, and the bridge ends on an empty 409. So once a request is cut,
   as when a setup is cancelled during a bridge's initialization (`initialize` is ID 0) or a
   bridge ends with a call in flight, every later bridge to that run fails at that ID. The gate
   also sends `notifications/cancelled` for a cut `initialize`, which MCP forbids: "The
   `initialize` request MUST NOT be cancelled by clients"
   ([cancellation](https://modelcontextprotocol.io/specification/2025-11-25/basic/utilities/cancellation)).
3. `provider_result` in `engine/crates/shell/src/runner.rs` reads a result with
   `reason` refused or unavailable and no `outcome` as "performed nothing", so `run.v1` reports
   failed. `engine/CONFORMANCE.md`'s DL16 row under "Choices still required" says the reason form
   "needs a contract decision".

**What `spec/` settles for 3** (the coordinator's reading, open to the owner's strike). IFC-026
gives a reason only where "a filter it was sent forbids the call, or it cannot apply a filter it
was sent", and says "An implementation gives a reason only in those two cases". A provider call
carries no `filters` (PRV-005). So no provider result holds a reason under IFC-026, and OPS-007,
by which PRV-005 reads a provider's result, makes one that "states neither an outcome nor a
reason under IFC-026" unknown. Its claim to have performed nothing is not relied on, as IFC-009
treats a result that contradicts itself. No contract decision is needed to read it so; whether a
provider call should have a reason form of its own is the owner's question (What follows).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); ruling 92, point 1): 4,
nothing assumes a particular vendor's agent: the gate no longer needs a server that tolerates a
second `initialize`; 6, nothing repeats an effect implicitly: a late reply to a cancelled
request still answers nothing later, and a `run.v1` whose provider may have started a run is not
read as failed, so a further setup does not take it as never run. No promise or case changes.

**Should this exist** (point 3). Yes: each is the least change that keeps a promise already
written, by existing practice. A proxy that shares one server among many clients answers later
`initialize`s with the first result and keeps the clients' request IDs apart, as language-server
multiplexers such as ra-multiplex do; MCP itself keeps an ID unique only "within the same
session" ([requests](https://modelcontextprotocol.io/specification/2025-11-25/basic)). Not taken:
one stdio server per session; the engine's bridge renumbering its client's IDs; a spec change
giving provider calls a reason form.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for under
2 hours), a Codex verifier, a Claude reader and the gate (DEC-030, DEC-033). By ruling 89, each
decision says what must hold; how is yours. The test names are the coordinator's, open to the
owner's strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- a promise's wording needs a choice that `spec/` does not make: report the promise, the choice
  and the options; do not choose;
- you find wording in `spec/` that gives a provider's reason another meaning than decision 3's;
- a correct result needs a change outside the scope: to the engine's bridge (`http_bridge.rs`),
  `engine/crates/core/`, `provider-local-process` or `provider-ssh-runtime`, a `Cargo.toml` or a
  lockfile;
- a test cannot show its decision without a sleep beyond the waits its file already uses.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

### The kit's gate

1. **A run's server is initialized once** (finding 1). Per channel:
   - The run's stdio receives one `initialize`, the first a session sends, and one
     `notifications/initialized`. The gate never sends `notifications/cancelled` for an
     `initialize`; where that request's HTTP response was cut, the server's answer, when it comes,
     is kept and sent to no one.
   - Every later `initialize`, after a replacement, a DELETE or a bridge that ended, still ends
     the old session and gets a fresh session ID, as today, and the gate answers it itself,
     sending the run nothing: with the server's result to the first, under the later request's
     own `id`, in the format its `Accept` selects. Its `notifications/initialized` is forwarded
     only if none has reached the run since the first `initialize` was answered; any other is
     answered 202 and not forwarded.
   - A later `initialize` that comes before the server has answered the first gets that answer
     when it comes, within the bounds any request has. Where the server answered the first with an
     error, nothing was initialized: the next `initialize` is forwarded as the first was.
2. **A cancelled ID is free in a later session** (finding 2).
   - A request whose ID was cancelled, or was pending when its session ended, is accepted in a
     later session: no 409.
   - No two requests that may still be answered share an ID on the run's stdio, so a late reply to
     a cancelled request never answers a later one. One way: forward a request whose ID the
     channel has cancelled under an ID of the gate's own, and give its reply the request's ID
     back.
   - A request whose ID the channel never cancelled is forwarded byte for byte, as today. A cut
     `initialize`'s ID counts here as cancelled, since its answer may still come. A
     `notifications/cancelled`, whether the session or the gate sends it, names the ID the run was
     sent.
   - Within one session nothing changes: an active duplicate ID, or one that session cancelled,
     still gets an empty 409.

### The engine's provider calls

3. **A provider's reason reads as unknown** (finding 3; PRV-005, OPS-007). In `provider_result`,
   a result that holds `<metaPrefix>reason` and no `<metaPrefix>outcome` is unknown, whatever the
   reason's value or `isError`, and the shell leaves one fixed diagnostic that names IFC-026 and
   PRV-005 and holds no result content (ruling 52). The error -32021 of IFC-008 and IFC-047 still
   says the provider performed nothing, as OPS-007 has it. So `run.v1` answered with a reason
   reports unknown, not failed; `stop.v1` and `inspect-run.v1` report unknown, as today.

### Tests and documents

4. **Tests.** Write these first and run them on the unchanged code.
   - `later_sessions_initialize_without_reaching_the_run`, in the kit's `src/gate/tests.rs`: a
     session initializes with ID 1 and the run answers; a replacing `initialize` with ID 0, and one
     with ID 9 after a DELETE, are each answered 200 with that result under their own ID and with a
     new session ID, and the run's stdio receives neither them nor their
     `notifications/initialized`. On a second channel, a first `initialize` whose response is
     dropped before the run answers puts no `notifications/cancelled` on stdio, and a later
     session's `initialize` gets the run's answer once the run writes it, and that session's
     `notifications/initialized` reaches stdio once. Today the later ones are forwarded, and the
     cut one is cancelled and its ID refused.
   - `cancelled_id_is_free_in_a_later_session`, in `src/gate/tests.rs`: a `tools/call` with ID 7
     is cut in one session; in the next, a `tools/call` with ID 7 is accepted and reaches stdio;
     the run's late reply to the first, written after it, reaches no one, and its reply to the
     second answers the second. Today the second gets 409.
   - `runtime_provider_reason_reads_as_unknown`, in `engine/crates/shell/tests/provider_calls.rs`:
     the stand-in provider (`scripted_runtime_provider`, in a new mode) answers `run.v1` with
     reason refused, `isError` true and no outcome, and in a second setup with unavailable; each
     setup's outcome is unknown, and the runner's diagnostics name IFC-026. Today failed.
   - `local_process_new_bridge_initializes_the_run_once`, in
     `engine/crates/shell/tests/local_process.rs`, with the real provider and the kit's gate: an
     Agent that names the provider takes a turn; the test ends the engine's bridge to the run
     alone, as a crash would, and leaves the run (the runner's `run-groups` record, or the
     observing guard, can name the bridge's process group); a further `set-up.v1` reaches the
     run through a new bridge, a turn succeeds, and the scripted server's `methods` file holds
     `initialize` once. Today twice.
   - Existing tests change only where they assert what decisions 1 and 3 change: a second
     `initialize` on one channel in `src/gate/tests.rs`
     (`byte_bridge_sessions_and_two_reply_formats`,
     `replacement_delete_and_close_cancel_requests`,
     `buffer_overflow_ends_session_and_keeps_channel`);
     `dropped_response_is_cancelled_even_if_reply_precedes_queued_cancel` in `session.rs`, whose
     request becomes a `tools/call` in an initialized session, keeping its assertion that the cut
     request's `notifications/cancelled` reaches stdio even when the reply is processed first (its
     check of internal state may follow the new structure); and the two rows with reason refused
     and unavailable in `runner.rs`'s
     `provider_reading_rejects_contradictions_and_bad_outputs_and_distinguishes_no_effect`, which
     now expect unknown. Change no other existing test.
5. **The documents.**
   - The kit's README, "Shared TLS gate": make true each sentence decisions 1 and 2 change,
     among them "before forwarding the new initialize", "Thus a persistent stdio server sees a
     second initialize ...", the `notifications/cancelled` one, "POST bytes are forwarded as one
     stdio line", "pass through to the server", "inspects only JSON envelope fields", and the
     whole 409 bullet.
   - `engine/CONFORMANCE.md`: drop the sentence that begins "Provider reason-only replies" from
     the DL16 row under "Choices still required", and add to the row "DL16: runs of runtime
     providers" one sentence beginning "From order 239,": a provider's result with a reason in
     place of an outcome reads as unknown (IFC-026, PRV-005, OPS-007), and the kit's gate
     initializes a run's server once, answering each later bridge's `initialize` itself.
   - Run `python3 tools/promise_standing.py --write` where an engine source names a promise ID it
     did not.

## What to read

- IFC-009, IFC-026 and IFC-027 in `spec/interface/implementation.yaml`; OPS-007 and OPS-022 in
  `spec/conventions/operations.yaml`; PRV-005, PRV-009, PRV-019 and PRV-028 in
  `spec/promises/providers.yaml`.
- The kit's README, section "Shared TLS gate", and `src/gate.rs`, `src/gate/session.rs` and
  `src/gate/tests.rs`.
- The shell's `runner.rs` (`provider_result`, `provider_later`, `inspect_bridge`,
  `start_bridge`), `tests/provider_calls.rs` and `tests/local_process.rs`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor c1d09afc HEAD; echo "main $?"
grep -c 'let _ = self.output.send(data);' packages/runtime-provider-kit/src/gate/session.rs
grep -c 'runtime must use a server that tolerates this' packages/runtime-provider-kit/README.md
grep -c "remain reserved for the channel's lifetime" packages/runtime-provider-kit/README.md
grep -c 'before forwarding the new initialize' packages/runtime-provider-kit/README.md
grep -c 'IDs cancelled earlier on this channel' packages/runtime-provider-kit/README.md
grep -c 'their reason form needs a contract decision' engine/CONFORMANCE.md
grep -c 'From order 239,' engine/CONFORMANCE.md
grep -cE 'fn (later_sessions_initialize_without_reaching_the_run|cancelled_id_is_free_in_a_later_session)\(' packages/runtime-provider-kit/src/gate/tests.rs
grep -c 'fn runtime_provider_reason_reads_as_unknown' engine/crates/shell/tests/provider_calls.rs
grep -c 'fn local_process_new_bridge_initializes_the_run_once' engine/crates/shell/tests/local_process.rs
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for d in engine packages/runtime-provider-kit packages/provider-local-process packages/provider-ssh-runtime; do (cd "$d" && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "offline $? $d"; done
git status --short | wc -l | tr -d ' '
```

They print `main 0`, six lines `1`, four lines `0`, four lines `offline 0` with their directory,
and `0`. An `offline` line that prints otherwise means a crate is missing from the cache: stop
and report, and fetch nothing.

## Scope

Change the kit's `src/gate.rs`, `src/gate/` and `README.md`; `engine/crates/shell/src/runner.rs`
for decisions 3 and 4 alone; `engine/crates/shell/tests/provider_calls.rs` and
`tests/local_process.rs` for decision 4; `engine/CONFORMANCE.md`; and `engine/STANDING.md` only
by the tool. Change nothing else: not `spec/`, `conformance/`, `docs/`, `experiments/`, `tools/`,
another package, `engine/crates/core/` or another engine file, no `Cargo.toml` and no
`Cargo.lock`. Put anything else that needs changing in your report.

## Done when

Each line of the first group prints otherwise on `main` at `c1d09afc` ("today") and passes after.
The second group are guards, which pass today and must still pass; run its last three, the whole
suites, once, last.

```sh
ok='^test result: ok\. 1 passed'
(cd packages/runtime-provider-kit && cargo test --offline --release --lib later_sessions_initialize_without_reaching_the_run 2>&1 | grep -c "$ok")  # 1; today 0
(cd packages/runtime-provider-kit && cargo test --offline --release --lib cancelled_id_is_free_in_a_later_session 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-shell --test provider_calls runtime_provider_reason_reads_as_unknown 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-shell --test local_process local_process_new_bridge_initializes_the_run_once 2>&1 | grep -c "$ok")  # 1; today 0
grep -c 'runtime must use a server that tolerates this' packages/runtime-provider-kit/README.md  # 0; today 1
grep -c "remain reserved for the channel's lifetime" packages/runtime-provider-kit/README.md  # 0; today 1
grep -c 'before forwarding the new initialize' packages/runtime-provider-kit/README.md  # 0; today 1
grep -c 'IDs cancelled earlier on this channel' packages/runtime-provider-kit/README.md  # 0; today 1
grep -c 'their reason form needs a contract decision' engine/CONFORMANCE.md  # 0; today 1
grep -c 'From order 239,' engine/CONFORMANCE.md  # 1; today 0
```

```sh
(cd packages/runtime-provider-kit && cargo fmt --check)
(cd packages/runtime-provider-kit && cargo clippy --offline --all-targets -- -D warnings)
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
(cd engine && cargo test --offline --release -p conduit-shell --lib provider_reading_rejects 2>&1 | grep -c "$ok")  # 1, with the first group's ok
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py                   # Errors: 0
python3 tools/promise_standing.py --check     # Errors: 0
python3 tools/check_private_paths.py          # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 0 changes; 0 undeclared; 0 errors; exit 0
git diff --check                              # prints nothing
git diff --name-only HEAD | grep -vE '^(packages/runtime-provider-kit/(src/gate\.rs|src/gate/|README\.md)|engine/crates/shell/(src/runner\.rs|tests/(provider_calls|local_process)\.rs)|engine/(CONFORMANCE|STANDING)\.md)' | wc -l | tr -d ' '  # 0
(cd packages/runtime-provider-kit && cargo test --offline --release)  # the kit's whole suite
(cd packages/provider-local-process && cargo test --offline --release)  # the provider's, on the changed kit
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Your report gives, per decision, what changed and how the gate keeps requests apart on the run's
stdio; each new test's failing line on the unchanged code, then its passing line; each existing
test you changed, and why; your own choices; and every check's last line.

Stage nothing. The verifier's turn follows; then the gate, which also runs
`provider-ssh-runtime`'s suite on the changed kit, a read-only Claude reader, and the merge.

## What follows

Named here and not in this order:

- Whether a provider call should have a reason form of its own, such as unavailable where a
  provider cannot serve now: a change to PRV-005 and IFC-026, the owner's.
- Found while writing this order: `stop.v1` answered with -32021 reports unknown (080's mapping),
  where OPS-007 gives the error of IFC-008 failed; a follow-up.
- IFC-001's `tools/list` over HTTP for instance dispatch, the rest of 080's discovery finding, as
  `engine/CONFORMANCE.md`'s DL16 row says.
- R-2 and R-3, as order 226's "What follows" names them.

## Record

Not yet run.
