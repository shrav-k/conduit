# Work order 226: the engine runs instances on `provider-local-process`, qualified on the Mac (R-1, second part)

**Split from [order 221](221-engine-runtime-provider-local-process.md) on 2026-11-20.** One Codex
worker turn has a 4-hour limit, so R-1 is two orders. 221 carries orders 079 and 080 onto `main`;
this order holds the rest. It starts only on a `main` that holds 221's build: the start check
says.

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no network:
it reaches no host and fetches no crate, and the only sockets it opens are the loopback ports its
tests bind. Your turn has a 4-hour limit; plan to finish in about two. Run one cargo command at a
time. Never print a whole large file, diff, fixture or log: use `head`, `tail`, `grep`, `sed -n`
or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never print a key, a token or any credential, even one a test made. Do not
run `conduit` against a store of the owner's, `codex`, `claude` or the dogfood harness, and sign in
nowhere.

## What and why

**What.** An instance whose document names a runtime provider in `runtime` (PRV-006; the
instance's document gives it, not its template) runs on `packages/provider-local-process`, on the
Mac, beside an instance on the engine's own runner. A url that proves nothing reports the part
unavailable (PRV-028, PRV-009), and a provider whose run ended with the engine runs again before a
call (PRV-026), by order 218's path.

**Why.** After 221, 080's mapping holds: where nothing at a run's url proves the instance, or a
provider's discovery lacks Conduit's extension, the part reports unknown or failed, and `Running`
becomes `Unknown` where the provider said `running`, against PRV-009 (080's step-8 findings). A
provider ended at a restart is not run again (PRV-026). And no shipped provider has been run by the
engine: 080's tests use a stand-in, `scripted_runtime_provider`, with the test gate in
`tests/runtime_support/`. `engine/CONFORMANCE.md` names each with the words "until order 226".

**Should this exist** (ruling 92 point 3). The run is an existing tool's:
`provider-local-process` runs Unix processes, and `packages/runtime-provider-kit` gives it the MCP
server and the per-run TLS gate (A63). Reporting unavailable and running a provider again are the
engine's by PRV-028, PRV-009 and PRV-026. The run-again path exists already, order 218's for
secret providers, and this order reuses it, building no second. The rest is tests.

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1): 2,
since the real kit's gate makes a Conduit binding on a server the engine does not run itself
(PRV-025); 6, since only a setup asks for a run, nothing restarts it, and running a provider again
starts no model work (PRV-007, PRV-026); and 9, since a runtime provider is the route by which a
later one can confine what it runs (PRV-006), while this version reports every restriction
mediated (ACC-027).

**When it runs.** An order of A78's stage 2, sent through Conduit under
[order 211's registration](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/211-a78-stage-2-registration.md): one Codex worker turn, a Codex
verifier and a Claude reader who run only what settles a doubt, then the local gate, which runs
the whole suite (AGENTS.md's work-order rules, DEC-033). It changes nothing under
`experiments/028-a78-dogfood/` and not the harness. How you build what each decision says must
hold is yours ([ruling 89](../proposals/ruling-89/README.md)).

**Stop rule.** Where a promise's wording needs a choice that `spec/` does not make, stop there,
change nothing further, and report the promise, the choice and the options. Do not choose. Stop
and report too if a build asks for a crate the cache lacks.

## Findings carried, and how this part handles them

- **The R-3 blocker.** The Agent packages' items are package-relative `command`s, which PRV-007
  keeps on the engine's own runner, so a `codex.v2` Agent that names a provider ends its setup
  failed. This order's tests use an absolute program (decision 3) and solve nothing here. Decision
  6's sentence says so; how a package ships a program a provider can run is the owner's choice,
  before R-3.
- **PRV-021 and local-process's state.** local-process keeps its owed stops in the
  `CONDUIT_STATE_DIR` that the engine's own runner gives it, which PRV-029 removes with the
  provider's Resource, so its statement under PRV-021 does not hold past its own removal. Change
  nothing under `packages/`, and retire no provider Resource in the tests: report it as a finding
  for the provider's next order.
- **Crates missing from the cache.** 221 took the bridge off reqwest's TLS and its lockfile holds
  neither `rustls-platform-verifier` nor `webpki-root-certs`. This order adds no crate and changes
  no `Cargo.toml` or `engine/Cargo.lock`; the start check shows the shell and local-process
  resolve offline.

## The decisions it applies

1. **A run whose url proves nothing** (PRV-028, IFC-017, PRV-009). Where nothing at the url that
   `run.v1` or `inspect-run.v1` returns proves by TLS that it serves the instance, the engine sends
   nothing there and the part reports unavailable, not unknown or failed, and the instance's
   `Running` condition still says what the provider last reported (`True` for `running`, and for
   a `run.v1` that succeeded, as the core records it today), not `Unknown`. So too where a
   provider's `server/discover` lacks Conduit's extension: the call is not sent, the part reports
   unavailable and the call's record names the extension (PRV-005).
   The core's reports cannot say today that a run was not reached or a call not sent: give them a
   way to, in `engine/crates/core/src/`, with a fixed diagnostic that names the promise and the
   instance and holds no url, and no change to any case's expectations. Change 080's tests
   `runtime_provider_failures_discovery_and_tls_refusal_map_reports` and
   `runtime_provider_inspection_repairs_ended_session_and_refuses_changed_unproved_url` where they
   assert the old mapping (unknown or failed, and `Running` `Unknown`), and say so.
2. **A provider run again before a call** (PRV-026). Where the engine is to make a provider call
   to a provider that is not retired and whose run on the engine's own runner ended when the engine
   stopped and has not been asked for since, the engine's own runner runs it again first, as it
   runs an item. Order 218 built that path for secret providers (`provider_runner_stopped`,
   `restart_if_needed` and `recover` in `engine/crates/core/src/secrets.rs`): serve a provider call
   through it, generalized where it names a read, and build no second. Each secret the provider's
   own run needs is read again through 218's reads. Where a read fails or the run cannot be made,
   nothing is sent and the part reports unavailable, with a diagnostic saying why. By the
   coordinator's reading, open to the owner's strike, a provider's run that ended while the engine
   ran is not run again so: the call is not sent and the part reports unavailable, naming the
   provider, until a setup of the provider runs it (PRV-007), as 218 treats a secret provider's.
3. **local-process is the first provider, on the Mac.** The tests build
   `packages/provider-local-process` as `engine/crates/shell/tests/secrets.rs` builds
   `provider-file` (`cargo run --offline --locked --release --bin package`, with
   `CARGO_NET_OFFLINE`), install its archive, and register a `local-process.v1` Resource on `local`
   with its defaults, whose server the engine's own runner runs (PRV-002). The Agent that names it
   in `runtime` is of a test Type whose item is a `command` at an absolute path, a scripted MCP
   server the test builds, as `scripted_server` in `tests/runner.rs` is one: local-process runs
   only absolute programs. The scripted server records names, paths, the value of `CONDUIT_URL`
   and whether each identity file exists, and never a value of `identity` or of a secret. Change
   nothing under `packages/`: a fault you find in the provider or the kit goes in your report with
   the test that shows it, and if it keeps a Done-when from passing, stop and report.
4. **The built-in runner stays the default.** An instance that names no `runtime` runs on the
   engine's own runner as on `main`: the loopback address and audience, no `tls.crt` or
   `tls.key`, and its state directory under PRV-029 (PRV-003, PRV-006, PRV-022). No case changes,
   and no existing test but the two decision 1 names.
5. **The gate.** The engine's tests now build `packages/provider-local-process`, and through it
   `packages/runtime-provider-kit`: add both to `ENGINE_PREFIXES` and `ENGINE_WHY` in
   `tools/gate.py`, with their test in `tools/tests/test_gate.py`, as order 218 added
   `packages/provider-file/`.
6. **What the documents say.** In `engine/CONFORMANCE.md` and `engine/README.md`, every place
   that says "until order 226" says what is served now. The row "DL16: executor coverage" gains
   one sentence beginning "From order 226," naming local-process as run by the engine on the Mac
   (the tests below), the unproved url and missing extension reporting unavailable, and a provider
   run again after a restart; it says that a package-relative `command` stays on the engine's own
   runner (PRV-007). Item 7 keeps that the SSH runtime provider's isolation is unmet and that the
   engine has not run with that provider. Say what stays unserved: remote connections, secrets for
   a run on a provider, the engine's own inspections (DL5). Run
   `python3 tools/promise_standing.py --write` where an engine source names a promise ID it did
   not.

## What to read

- PRV-002, PRV-005 to PRV-009, PRV-019, PRV-021, PRV-022 and PRV-024 to PRV-030 in
  `spec/promises/providers.yaml`; IFC-017 and IFC-037 in `spec/interface/implementation.yaml`;
  DL16 in `spec/implementation-defined.yaml`.
- `packages/provider-local-process/README.md` and `contracts.yaml`, and the kit's README, its
  section "Shared TLS gate".
- The core's `lifecycle.rs` (`emit`, `report`) and `secrets.rs` (218's path); the shell's
  `runner.rs`, `http_bridge.rs`, `tests/secrets.rs`, `tests/runner.rs` (`scripted_server`,
  `running_condition`) and `tests/provider_calls.rs` with `tests/runtime_support/`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor ed1c0477 HEAD; echo "main $?"
test -f engine/crates/shell/tests/provider_calls.rs; echo "221 $?"
grep -c 'From order 221,' engine/CONFORMANCE.md
cat engine/CONFORMANCE.md engine/README.md | grep -c 'until order 226'
grep -c 'fn restart_if_needed' engine/crates/core/src/secrets.rs
test -e engine/crates/shell/tests/local_process.rs; echo "tests $?"
grep -cE "'packages/(provider-local-process|runtime-provider-kit)/'" tools/gate.py
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine/crates/shell && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "offline $?"
(cd packages/provider-local-process && cargo metadata --offline --locked --format-version 1 >/dev/null); echo "provider $?"
git status --short | wc -l | tr -d ' '
```

With 221 merged they print `main 0`, `221 0`, `1`, a number above 0, `1`, `tests 1`, `0`,
`offline 0`, `provider 0` and `0`. On `main` at `ed1c0477`, before 221, the second to fourth
print `221 1`, `0` and `0`: 221 is not merged, so stop. If `offline` or `provider` prints
otherwise, a crate is missing from the cache: stop and report.

## Scope

Change `engine/crates/core/src/` for decisions 1 and 2 alone, `engine/crates/shell/src/`,
`engine/crates/shell/tests/` (`provider_calls.rs` and `runtime_support/` for test 6 and decision
1's change), and
`engine/CONFORMANCE.md`, `engine/README.md`, and `engine/STANDING.md` only by the tool; create
`engine/crates/shell/tests/local_process.rs`; change `tools/gate.py` and
`tools/tests/test_gate.py` for decision 5. Change nothing under `spec/`, `conformance/`,
`packages/`, `experiments/`, `docs/`, `engine/crates/loader/` or `engine/crates/conformance/`, no
`Cargo.toml` and not `engine/Cargo.lock`, nor `IN_FORCE.md`, `ROADMAP.md` or `docs/AUDIT.md`. No
case changes, so there is no declaration. Anything else goes in your report.

## Done when

Write the six tests below first. Each fails on `main` today (no such test target), and tests 4
and 6 fail on 221's merge too, before decisions 1 and 2: report each failing line. Tests 1 to 5
are in `engine/crates/shell/tests/local_process.rs`, each with decision 3's Resource and a store
whose `https` address (079's `--https-url`) names a host that is not loopback, which no test
reaches:

1. `local_process_runs_an_agent_turn`: an Agent that names the Resource in `runtime` is set up, and
   its operation, called through a Conduit from root, answers with the scripted server's reply;
   its run's `CONDUIT_STATE_DIR` is `instances/<SHA-256 of its ID>` under the provider's own state
   directory, not the store's `instances/<its ID>`, so the provider ran it; its `CONDUIT_URL` is
   the store's `https` address; `tls.crt` and `tls.key` exist in its identity directory; and no
   record or diagnostic holds a value of `identity` (PRV-007, PRV-019, PRV-022, PRV-024).
2. `local_process_gate_admits_only_the_engine`: at the url the provider returned for that run (the
   test learns it and the store's root without reading a key or a token), a POST of `initialize`
   with no `Authorization` and one with a bearer token the test signed with a key of its own are
   each answered 401, and the scripted server sees neither; the certificate served there chains to
   the store's root and names the Agent's SPIFFE ID alone (PRV-025, PRV-028).
3. `local_process_run_state_follows_the_provider`: after setup, `Running` is `True` with a `source`
   naming the provider (PRV-009); once the run's program exits on its own, nothing starts it again
   (the provider's contract), and a further `set-up.v1` finds it `stopped`, asks `run.v1`, and a
   turn succeeds on the new run; retiring the Agent asks `stop.v1`, the retirement succeeds and
   removes the Agent, no process of either run is left, and the provider's private
   `runs/<SHA-256 of the ID>` directory is gone (PRV-008, PRV-030).
4. `local_process_provider_runs_again_after_restart`: the shell and runner are dropped, which ends
   the provider's server and its runs; reopened, a setup of the Agent alone runs the provider again
   before its `inspect-run.v1`, which reports `stopped`, and a new run takes a turn (PRV-026).
5. `builtin_and_local_process_runs_side_by_side`: in one store, an Agent of the same Type that
   names no `runtime` and one that names the Resource each take a turn; the first's run has the
   loopback `CONDUIT_URL`, no `tls.crt` or `tls.key`, and its state directory at
   `instances/<its ID>` in the store (PRV-003, PRV-022, PRV-029); the second's is as in test 1.
   This is the no-regression check.
6. `runtime_provider_unproved_url_reports_the_part_unavailable`, in
   `engine/crates/shell/tests/provider_calls.rs`: a stand-in provider (`scripted_runtime_provider`
   in `provider_calls.rs`, as its `self-signed` mode does) answers `run.v1` with the url of a TLS
   listener whose certificate the store's root did not issue; the listener reads no HTTP request,
   the run part reports unavailable with decision 1's diagnostic, and `Running` is `True`, as the
   core records a `run.v1` that succeeded (`Report::Run` in `lifecycle.rs`), not `Unknown`
   (PRV-028, IFC-017).

Each line in the first group prints otherwise on `main` at `ed1c0477` (shown as "today") and
passes after. The second group are guards, which pass today and must still pass. Run the whole
suite once, last.

```sh
(cd engine && cargo test --release -p conduit-shell --test local_process <name>)  # once per test 1 to 5: 1 passed; today no such test target
(cd engine && cargo test --release -p conduit-shell --test provider_calls runtime_provider_unproved_url_reports_the_part_unavailable)  # 1 passed; today no such test target
(cd engine && cargo test --release -p conduit-shell --test provider_calls)  # 080's tests, as decision 1 changes two, and test 6 pass; today no such test target
grep -c "'packages/provider-local-process/'" tools/gate.py  # 1 or more; today 0
grep -c "'packages/runtime-provider-kit/'" tools/gate.py    # 1 or more; today 0
grep -c 'From order 226,' engine/CONFORMANCE.md              # 1; today 0
```

```sh
(cd engine && cargo fmt --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
cat engine/CONFORMANCE.md engine/README.md | grep -c 'until order 226'  # 0 (1 or more on 221's merge)
git add -N engine                             # so the checks see the new file
python3 tools/check_docs.py                   # Errors: 0
python3 tools/promise_standing.py --check     # Errors: 0
python3 tools/check_private_paths.py          # Errors: 0
python3 -m unittest discover -s tools/tests   # OK
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 0 changes; 0 undeclared; 0 errors; exit 0
git diff --check                              # prints nothing
git diff --name-only HEAD -- spec conformance packages experiments docs engine/crates/loader engine/crates/conformance engine/Cargo.lock IN_FORCE.md ROADMAP.md '*Cargo.toml' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, every vector among it; once, last
git reset -q                                  # stage nothing
```

Your report gives, per decision, what changed; how 218's path now serves a provider call; each
test's failing lines; each fault found in the provider or the kit, and the PRV-021 finding; each
choice you made yourself, under `own_choices`; and every check's last line. Stage nothing. The
coordinator runs `tools/gate.py`, has a fresh, read-only Opus reader read the diff
([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4), and merges.

## What follows

Named here and not in this order:

- **R-2:** the Linux node enrolled and placed through `packages/provider-ssh-runtime` and its
  machine provider (ruling 77; enrollment's core half, order 049a1, is checkpointed for `s3/m2`).
  **R-3:** a Codex Agent on the node, after the owner chooses how an Agent package ships a program
  a provider can run (the R-3 blocker above), such as a registry package or a program at an
  absolute path.
- The PRV-021 finding, for local-process's next order.
- Secrets for a run on a provider (SEC-010's runtime-provider half, PRV-011).
- 080's other step-8 findings, as its Record lists them (instance dispatch reads no
  `server/discover`, a repeated `initialize` at the gate, the kit's cancelled IDs, IFC-026's
  reason form).
- A dogfood Agent that names a provider needs the store's `https` address and its terminator
  (ruling 77, point 3); the coordinator sets them up when one first does.

## Record

Run on 2026-11-20 through Conduit itself, by the dogfood's own Agents, in Pacific time.
Worker-1's first turn (17:55 to 18:12) wrote the order's tests and stopped, correctly, at the
shipped provider's qualification. The Mac's OS hostname is mixed case, the engine sends the
canonical lowercase name (PRV-027), and `provider-local-process` compared them exactly. Root
widened the scope to that provider's hostname comparison. Root also rebased the work onto order
231, which routes provider calls through discovery.

Worker-2 took it over (19:19 to 19:57). The provider now compares and advertises the canonical
hostname. The shipped provider passed all five Mac integration tests, and the one whole engine
run passed 472.

`turn-verifier-2.turn.v1` (19:57 to 20:06) found four musts:
- a repeated URL accepted without fresh TLS proof;
- an MCP initialization failure overwriting Running;
- no restart after an upgrade from order 221's engine;
- the provider and the engine canonicalizing differently.

The second turn (20:06 to 20:19) fixed all four, each with a regression that failed first.
Where stored history cannot show that a run ended at shutdown, root decided that the part needs
explicit setup, and its diagnostic says so.

`turn-reader-1.turn.v1` (20:19 to 20:24), the Claude reader, accepted it with no findings.
PRV-021 past the provider's own removal (F2) is left to the next provider order. The gate's run
on the squash is the coordinator's, at the merge.
