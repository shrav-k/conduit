# Work order 219: `conduit records` reads the records through serve while it runs, with the output it gives after a stop

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate, and the only sockets it opens are the loopback
ports that the engine's own tests bind, as every run of the suite does. Run one cargo command at
a time. Never print a whole large file, case, fixture or log: use `head`, `tail`, `grep`,
`sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential; a test
may read root's client token from its own temporary store, and prints it nowhere. Do not run
`conduit` against a store outside a test, `codex`, `claude` or the dogfood harness, and sign in
nowhere.

## What and why

**The gap** (the dogfood-gaps stream, step G-1; [ruling 92](../proposals/ruling-92/README.md),
point 9). `conduit records --store DIR` opens the store itself (`Shell::open` in
`engine/crates/shell/src/main.rs`), and while `conduit serve` holds the store the lease refuses
it with "store already has an active shell"; its help says "stop serve first". So the dogfood
must stop the engine to export what its Agents did. Stopping serve ends every run, and the next
start sets every instance up again, between every two orders. `inspect`, `pending` and the
author commands already work while serve runs, through serve (`author::run`, and the fallback in
`main.rs` on the lease's refusal). This order makes `records` do the same, with the same output.

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
7, everything an Agent does through Conduit is attributable, now while it acts and without
stopping it.

**Should this exist** (point 3). No existing tool reads Conduit's records: they are the engine's
own store. The promises that make them Conduit's to serve are REC-001 (each occurrence can be
found), REC-003 (a record can still be read, also once the objects it names are gone), REC-005
(one source's records in the order made), REC-006 (a gap is said, never a complete-looking
result), and DL8 in `spec/implementation-defined.yaml`, under which this engine reads the records
that name no existing object by its export (`engine/CONFORMANCE.md`, the row "DL8: records").
The route is the existing paradigm of a daemon's API: `docker logs` asks the daemon that holds
the logs, `kubectl` reads through the API server; the client does not read the store beside its
holder. Two routes are not taken (the coordinator's, open to the owner's strike):

- The command reading `state.json` beside serve, outside the lease. It would parse the store's
  format and its gaps in a second place (ruling 92 point 7), and has no store to read once the
  engine runs as several parts (DL17).
- An MCP tool for the export. The export is DL8's, an operation of no Type; adding one to the
  management Type changes `spec/`, which is the owner's.

**When it runs.** An order of A78's stage 2, sent through Conduit under
[order 211's registration](211-a78-stage-2-registration.md) ("Stage 2's orders"): one Codex
worker turn and one verifier turn, each by `conduit call`, on the Mac. It changes nothing under
`experiments/028-a78-dogfood/` and not the harness; the dogfood's plan stops serve to export
until this merges, and the coordinator changes that step afterwards, not here.

## The decisions it applies

By [ruling 89](../proposals/ruling-89/README.md), each decision says what must hold; how the
shell does it is yours.

1. **The command.** Where no process holds the store, `conduit records --store DIR` is unchanged:
   it opens the store and exports. Where the lease refuses because the store is held, it asks
   serve at the store's engine address (`Config::for_store`), as `inspect` does, and never opens
   the store. Where the store is held and nothing answers at that address (another `conduit`
   process, such as `register`, holds it), it prints nothing on stdout and exits nonzero with a
   message naming the engine address, saying that another process holds the store and that
   serve does not answer there (ruling 52). Both paths print through one function: each record
   as one JSON line on stdout as today, and, where the export is not complete, today's stderr
   line `records: export has gaps: ...; preserve the available records and check retention`
   and a nonzero exit. So the output is the same by construction.
2. **The route.** Serve answers `GET /records` on its own listener, at the engine address's
   origin. It shares the token verification and the `Origin` comparison of `authenticate` in
   `engine/crates/shell/src/server.rs` (IFC-036, DL12, DL13), and none of its recording: not the
   `record_rejection` of a refused `Origin`, and not the rejection recorded for a non-success
   response to an established identity. `/mcp` keeps both branches unchanged (IFC-036 records a
   refused `Origin` at `/mcp` only). Root's token alone gets HTTP 200 with a JSON
   body equal to the reply of the shell's `export-records` input: `records`, `gaps` and
   `complete`. Serve submits that input to its own shell off the async workers, as
   `authenticate` verifies tokens (`spawn_blocking`). The token of any other instance gets 403,
   no token or one that does not verify gets 401, a refused `Origin` gets 403, and any method
   other than GET gets 405; none of them carries a record. The path `/mcp` and its rules do not
   change.
3. **What a live export is.** It is the last committed state's records, read through the shell
   as every other read is, so a record is in it whole or not at all, in the order REC-005 has.
   A later export's lines begin with an earlier one's unless a purge came between (REC-003,
   REC-004). It is a snapshot, not a feed: DL8's feed and cursor stay unimplemented, and the
   row's sentence "No live record-feed cursor is implemented" stays.
4. **No record of an export.** The route is no MCP request and goes through no Conduit, so
   neither an export nor a refusal at it leaves a record, as the export with serve stopped leaves
   none; REC-001 requires none (the coordinator's, open to the owner's strike). A refusal at
   `/mcp` is recorded as today.
5. **The words.** `engine/crates/shell/src/help.rs`: the description of `records` says it goes
   through serve while serve holds the store, and no longer says "stop serve first" (the one for
   `register` keeps it). `engine/README.md`: its two sentences telling the reader to stop serve
   before exporting records say instead that the export goes through serve while it runs.
   `engine/CONFORMANCE.md`: the row "DL8: records" gains one sentence beginning "From order
   219," saying that while serve holds the store `records` asks serve, root's token alone being
   answered, for the last committed records with the same output, and names the tests of
   decision 6.
6. **The tests**, in `engine/crates/shell/tests/serve.rs`, with its `start_server` and `cli`
   helpers, each running the real binary. The first two stop serve without `stop_server`, whose
   second half runs `conduit inspect --store` once serve has exited, and that inspect, opening
   the store itself, records an `inspect.v1` per object: they send SIGTERM and wait for exit 0,
   as `stop_server`'s first half does, or use a helper of yours that runs no command against the
   store.
   - `records_while_serving_match_records_after_stop`. After `init` and `serve`, requests through
     serve that leave records, among them an object registered and then unregistered, so that a
     record names an object that no longer exists (REC-003; the project and steps of
     `author_commands_require_serve_and_make_recorded_requests` will do). Then, serve running:
     `conduit records` gives A; `conduit inspect` runs; `conduit records` gives B. Serve stops;
     `conduit records` gives C. Each exits 0 with empty stderr; A is not empty and its lines are
     a strict prefix of B's; B equals C byte for byte; and B holds a record of an `inspect.v1`
     dispatched after A was read and a record that names the unregistered object.
   - `records_while_serving_report_a_gap_as_after_stop`. Serve running, a purge through serve
     (`conduit call management.purge.v1` with a `before` that removes some records, REC-004),
     then `conduit records` exits nonzero, its stderr beginning `records: export has gaps:`
     (REC-006). Serve stops, and `conduit records` gives the same stdout byte for byte, the same
     stderr and the same exit code.
   - `records_route_answers_root_alone`. `GET /records` with root's token answers 200, and its
     `records` are the lines `conduit records` prints at that moment. No token answers 401, a
     forged token 401, root's token with an `Origin` the engine does not accept 403, a `POST`
     405, and the token of an instance other than root 403. That token comes with no network:
     before serve starts, the test opens the store in-process (`Shell::open`), calls the public
     `Shell::issue_client_token` for a Pool or Resource instance, drops the shell, and then starts
     serve; or it uses a project-local run implementation. No gating on `CONDUIT_TEST_NPM`; say
     in your report which route and which instance. No refusal's body holds `records`, and the
     count of records in `state.json` is the same before and after the refusals.
   - `records_when_held_without_serve_names_the_address`. The test holds the store's
     `writer.lock` itself (`fs2`'s `try_lock_exclusive` or `libc::flock`, as the lease takes it
     in `engine/crates/shell/src/store.rs`), with no server on the engine port; `conduit records`
     prints nothing on stdout, exits nonzero, and its stderr names the engine address and says
     serve does not answer there.

   Each of the four fails on `main`: the first two because `conduit records` exits nonzero with
   "store already has an active shell", the third because no route answers 200, the fourth
   because its stderr names no engine address.
7. **Stop and report, change nothing more,** if a correct export through serve needs a change
   under `engine/crates/core/`, to a promise or a case, or to a `Cargo.toml` or
   `engine/Cargo.lock`; or if serve's own shutdown (its `recover` submit and any run endings)
   adds a record in the first test's store, so that B cannot equal C.

## What to read

- `engine/crates/shell/src/main.rs`: the `records` arm, and the `inspect` fallback on "store
  already has an active shell".
- `engine/crates/shell/src/author.rs`: `run`, its connection check and its messages.
- `engine/crates/shell/src/server.rs`: `serve` (the `Router`), `authenticate`, and `client`
  (how a client finds root's token).
- `engine/crates/core/src/records.rs`: `export`, read only.
- REC-001 to REC-006 in `spec/conventions/records.yaml`; DL8 in
  `spec/implementation-defined.yaml`; the DL8 rows of `engine/CONFORMANCE.md`.

## Before you change anything

From the repository root, check that you find what this order was written against, and stop,
change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 55d889bc HEAD; echo "main $?"
grep -c 'stop serve first' engine/crates/shell/src/help.rs
grep -c 'route_service("/mcp"' engine/crates/shell/src/server.rs
grep -c '"export-records"' engine/crates/shell/src/main.rs
grep -c 'before exporting records' engine/README.md
grep -c 'No live record-feed cursor is implemented' engine/CONFORMANCE.md
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `2`, `1`, `1`, `2`, `1` and `0`.

## Scope

Change `engine/crates/shell/src/` (`main.rs`, `server.rs`, `help.rs`, `author.rs`, or a module of
your own), `engine/crates/shell/tests/serve.rs`, `engine/README.md` and `engine/CONFORMANCE.md`,
and `engine/STANDING.md` only by `promise_standing.py --write` if its check asks. Change nothing
under `engine/crates/core/`, `engine/crates/loader/`, `engine/crates/conformance/`, `spec/`,
`conformance/`, `packages/`, `tools/`, `docs/` or `experiments/`, and no `Cargo.toml` or
`engine/Cargo.lock`. Anything else you find that needs changing goes in your report.

## Done when

Write the four tests of decision 6 first and run them on the unchanged engine: report each
failing line. Then change the shell until they pass. Each line below marked "fails today" prints
otherwise on `main` at `55d889bc`; all pass after.

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd engine && cargo test --release -p conduit-shell --test serve records_)  # 4 passed; fails today
(cd engine && cargo test --release -p conduit-shell --test cli)             # passes
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite
python3 tools/promise_standing.py --check       # Errors: 0
python3 tools/check_docs.py                     # Errors: 0
python3 tools/check_private_paths.py            # Errors: 0
git diff --check                                # prints nothing
git diff --name-only HEAD -- engine/crates/core engine/crates/loader engine/crates/conformance spec conformance packages tools docs experiments engine/Cargo.lock | wc -l | tr -d ' '  # 0
git diff --name-only HEAD -- '*Cargo.toml' | wc -l | tr -d ' '  # 0
grep -c 'stop serve first' engine/crates/shell/src/help.rs        # 1; fails today (2)
grep -c 'before exporting records' engine/README.md               # 0; fails today (2)
grep -c 'From order 219,' engine/CONFORMANCE.md                   # 1; fails today (0)
grep -c 'route_service("/mcp"' engine/crates/shell/src/server.rs  # 1: /mcp unchanged
grep -c 'No live record-feed cursor is implemented' engine/CONFORMANCE.md  # 1
```

Your report gives each test's name and its failing line on the unchanged engine, then its
passing line; the route as built (path, method, statuses) and where the command's two paths
share their printing; which instance's token the third test refuses and how it set it up, and what the fourth test's
stderr says; every
choice you made yourself, under `own_choices`; and every check's last line. Stage nothing. The
verifier's turn follows; the coordinator then runs `tools/gate.py`, has a fresh, read-only Opus
reader read the diff ([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4), and
merges.

## Record

Run on 2026-11-20 through Conduit itself, by the dogfood's own Agents, which root called by
`conduit call`, in Pacific time. `turn-worker-1.turn.v1` (12:20 to 12:37) built it: `conduit
records` exports through serve while serve holds the store. The order's four regression tests
reproduced the gap on the unchanged engine, and the release suite passed, 408 tests.

`turn-verifier-1.turn.v1` (12:48 to 12:51) was the first verifier under the owner's rule that a
verifier runs only what settles a doubt. It found two musts in the lease fallback: the live
export decoded the whole state to find root's credential, and a missing root credential hid the
no-server diagnostic. The second turn (13:07 to 13:11) fixed both, each with a test that failed
first.

`turn-reader-1.turn.v1` (13:12 to 13:16), the Claude reader, accepted it with one should and two
nits, which the third turn (13:17 to 13:19) settled:
- the Done-when filter `records_` now matches 6 tests, the order's 4 plus the 2 from the
  verifier's fixes;
- the diagnostic for a store started by an earlier build, which wrote no `root-client-id`, now
  says to restart `conduit serve` with this build;
- engine/CONFORMANCE.md no longer says the Host allowlist covers `/records`, and names what it
  checks instead.

The gate's run on the squash is the coordinator's, at the merge.
