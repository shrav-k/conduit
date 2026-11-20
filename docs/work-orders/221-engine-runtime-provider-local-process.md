# Work order 221: the engine runs instances through a runtime provider, local-process first (R-1)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no network:
it reaches no host and fetches no crate, and the only sockets it opens are the loopback ports its
tests bind. Run one cargo command at a time. Never print a whole large file, diff, fixture or log:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never print a key, a token or any
credential, even one a test made. Do not run `conduit` against a store of the owner's, `codex`,
`claude` or the dogfood harness, and sign in nowhere.

## What and why

**What.** An instance whose document names a runtime provider in `runtime` (PRV-006; the
instance's document gives it, not its template) is run by that provider through provider calls
(PRV-005, PRV-007 to PRV-009), and the engine reaches its run behind the provider's gate (PRV-025,
PRV-028, IFC-017), with `packages/provider-local-process` as the first provider, on the Mac. An
instance that names no `runtime` runs on the engine's own runner exactly as on `main` (PRV-003,
PRV-006): the built-in runner stays the default.

**Why.** On `main` the core emits these calls and records them, but the shell serves none:
`Executor::provider_call` is the trait's default, so such a setup reports its run failed and its
inspection unknown (`missing` in `engine/crates/shell/src/lib.rs`). `engine/CONFORMANCE.md` says
so (the row "DL16: executor coverage", item 7 of "What v0 does not claim", "How to get more"),
and `engine/STANDING.md` shows 9 of the 30 PRV promises with a passing case, none by a run on a
provider. On 2026-11-20 the owner approved a third stream of orders, the runtime provider's:
"Yes, start R-1 now (Rec.)". This order is R-1.

**Should this exist** (ruling 92 point 3). The run is an existing tool's, behind the contract:
`packages/provider-local-process` runs Unix processes, and `packages/runtime-provider-kit` gives it
the MCP server and the per-run TLS gate (A63). The engine's side, calling a provider and reaching
its run, is Conduit's by PRV-005, PRV-007 and IFC-017, and it is built already, off `main`:
[order 079](079-shell-https-address.md) gave the engine its `https` address (merged into the side
branch `s3/m2` as `9b8f3326`), and [order 080](080-shell-runtime-provider-calls.md) served the
provider calls and a bridge to each run (two rounds, checkpointed as `7e54117e`, its Record's
`80841bc7` rewritten, on `origin/wo/080-shell-runtime-provider-calls`, its Record at `8e13bd81`,
three musts left). This order carries both onto `main` and finishes them, and adds no mechanism.
TLS is rustls's, MCP is rmcp's, and an existing tool terminates TLS at the engine's `https` address
(`tailscale serve`, [ruling 77](../proposals/ruling-77/README.md), point 3).

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1): 2,
since the gate makes a Conduit binding on a server the engine does not run itself (PRV-025); 6,
since only a setup asks for a run, nothing restarts it, and running a provider again starts no
model work (PRV-007, PRV-026); 7, since each provider call is recorded (PRV-005, REC-001); and 9,
since a runtime provider is the route by which a later one can confine what it runs (PRV-006),
while this version reports every restriction mediated (ACC-027).

**When it runs.** An order of A78's stage 2, sent through Conduit under
[order 211's registration](211-a78-stage-2-registration.md) ("Stage 2's orders"): one Codex worker
turn, then one Codex verifier turn under [the standing verifier order](verify.md), each by
`conduit` call on the Mac, then the local gate and one fresh read (ruling 92 point 4); the
verifier turn is order 211's registered pair (the coordinator's dispatch, open to the owner's
strike). It changes nothing under `experiments/028-a78-dogfood/` and not the harness. By the
coordinator's reading, open to the owner's strike, the owner's answer brings 079 and 080 to `main`
now, though `s3/m2` was to merge after the S1 rerun's pin ([order 120](120-plan-providers.md),
point 4). How you build what each decision says must hold is yours
([ruling 89](../proposals/ruling-89/README.md)).

**Stop rule.** Where a promise's wording needs a choice that `spec/` does not make, stop there,
change nothing further, and report the promise, the choice and the options. Do not choose.

## The decisions it applies

1. **Order 079, as built.** Apply `git diff 2a9b25ae 9b8f3326 -- engine` to your tree (for
   instance with `git apply -3`), resolving each conflict so that `main`'s later behavior holds and
   079's is added. Its decisions 1 to 8 stand as its file states them; do not rework them.
2. **Order 080, as built, and its last verifier's three musts.** Apply
   `git diff 9b8f3326 7e54117e -- engine` over decision 1 the same way. Its decisions stand as its
   file states them, the bridge `conduit __http-bridge` among them. Then fix the three musts under
   "The last verifier's findings" in its Record
   (`git show 8e13bd81:docs/work-orders/080-shell-runtime-provider-calls.md`): an empty-data
   priming event, the end of stdin while initialization waits, and the `Mcp-Method` and
   `Mcp-Name` headers, each with the regression its finding names. Where `main` has changed what
   080 relies on since 2026-11-12 (order 214's request metadata, order 209's keyed hooks, the
   sign-in work in `runner.rs` and `lib.rs`), `main`'s behavior holds. Report each place where you
   had to adapt 079 or 080.
3. **No crate that the cache lacks.** Add no crate that `cargo build --offline` cannot find. 080's
   lockfile takes `rustls-platform-verifier` and `webpki-root-certs` through reqwest's
   `rustls-no-provider` feature; the bridge trusts the store's root alone and needs neither, and
   the Mac's cache may hold neither. Where they are missing, reach the run's url through crates
   that the kit's lockfile already uses, such as rustls 0.23 with `ring`, tokio-rustls 0.26,
   hyper 1 and hyper-util 0.1. 080's rule stands: no `aws-lc`, OpenSSL or `native-tls` crate, and
   one rustls, 0.23. If no route builds offline, stop and report the missing crate.
4. **A run whose url proves nothing** (PRV-028, IFC-017, PRV-009; 080's step-8 findings). Where
   nothing at the url that `run.v1` or `inspect-run.v1` returns proves by TLS that it serves the
   instance, the engine sends nothing there and the part reports unavailable, not unknown or
   failed, and the instance's `Running` condition still says what the provider last reported
   (`True` for `running`), not `Unknown`. So too where a provider's `server/discover` lacks
   Conduit's extension: the call is not sent, the part reports unavailable and the call's record
   names the extension (PRV-005). 080 reports unknown or failed because the core's reports cannot
   say that a run was not reached or a call not sent: give them a way to, in
   `engine/crates/core/src/`, with a fixed diagnostic that names the promise and the instance and
   holds no url, and no change to any case's expectations. Change 080's test
   `runtime_provider_failures_discovery_and_tls_refusal_map_reports` where it asserts the old
   mapping, and say so.
5. **A provider is run again before a call** (PRV-026; 080's step-8 findings). Where the engine is
   to make a provider call to a provider that is not retired and whose run on the engine's own
   runner ended when the engine stopped and has not been asked for since, the engine's own runner
   runs it again first, as it runs an item. If your base already runs a secret Resource again
   under PRV-026 (order 218, S2-1), use that path for providers too and build no second, with each
   secret the provider needs read again through it; where the engine reads no secret, build no
   read, and a provider whose run needs one is not run. Either way, where a read fails or cannot be
   made, nothing is run and the part reports unavailable, with a diagnostic saying why. By the
   coordinator's reading, open to the owner's strike, a provider's run that ended while the
   engine ran is not run again so: the call is not sent and the part reports unavailable, naming
   the provider, until a setup of the provider runs it (PRV-007).
6. **local-process is the first provider, on the Mac.** The tests below build
   `packages/provider-local-process` as its README says (`cargo run --locked --release --bin
   package`, as `pack` in `engine/crates/shell/tests/agents.rs` builds a package), install its
   archive, and register a `local-process.v1` Resource on `local` with its defaults, whose server
   the engine's own runner runs (PRV-002). The Agent that names it in `runtime` is of a test Type
   whose item is a `command` at an absolute path, a scripted MCP server the test builds, as
   `scripted_server` in `tests/runner.rs` is one: local-process runs only absolute programs, and
   PRV-007 keeps a package-relative program, such as `bin/agent-codex`, on the engine's own runner.
   The scripted server records names, paths, the value of `CONDUIT_URL` and whether each identity
   file exists, and never a value of `identity` or of a secret. Change nothing under `packages/`:
   a fault you find in the provider or the kit goes in your report with the test that shows it,
   and if it keeps a Done-when from passing, stop and report.
7. **The built-in runner stays the default.** An instance that names no `runtime` runs on the
   engine's own runner as on `main`: the loopback address and audience, no `tls.crt` or `tls.key`,
   and its state directory under PRV-029 (PRV-003, PRV-006, PRV-022). No case changes, and no
   existing test but by 079's and 080's own changes and the one test decision 4 names.
8. **What follows from the change.** Update what `engine/CONFORMANCE.md` and `engine/README.md`
   say of runtime providers: the row "DL16: executor coverage", the DL13 rows, the sentences that
   the start check's pattern `p` finds, and item 7, which keeps that the SSH runtime provider's
   isolation is unmet and says the engine has not yet run with it. Say what stays unserved:
   remote connections, secrets for a run on a provider, the engine's own inspections (DL5). Run
   `python3 tools/promise_standing.py --write` where an engine source names a promise ID it did
   not. The coordinator writes 079's and 080's standing at the merge.

## What to read

- PRV-002, PRV-005 to PRV-009, PRV-019, PRV-022 and PRV-024 to PRV-030 in
  `spec/promises/providers.yaml`; IFC-017, IFC-022, IFC-037 and IFC-042 in
  `spec/interface/implementation.yaml`; DL13 and DL16 in `spec/implementation-defined.yaml`.
- `packages/provider-local-process/README.md` and `contracts.yaml`, and the kit's README, its
  section "Shared TLS gate".
- The shell's `lib.rs` (`Executor`, `drive`, `missing`), `runner.rs`, `identity.rs`, `server.rs`,
  `tests/agents.rs` (`pack`) and `tests/runner.rs` (`scripted_server`, `running_condition`); the
  core's `lifecycle.rs` (`emit`, `report`).

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 55d889bc HEAD; echo "main $?"
for c in 2a9b25ae 396bf304 9b8f3326 7e54117e 8e13bd81; do git cat-file -e "$c^{commit}" || echo "missing $c"; done; echo commits
git diff --quiet 396bf304 9b8f3326 -- engine; echo "stacked $?"
grep -c 'fn provider_call' engine/crates/shell/src/runner.rs
p='Runtime provider calls and remote connections are unserved|Runtime providers and remote connections still have|remote transport or runtime providers, which remain unfinished|It calls no runtime provider|does not yet call that provider|The first build calls no runtime|Runtime providers and remote connections remain unserved'
cat engine/CONFORMANCE.md engine/README.md | grep -cE "$p"
grep -cE '^\| PRV-[0-9]+ \|.*\| case passes \|$' engine/STANDING.md
(cd packages/provider-local-process && cargo build --offline --locked --release --bin provider-local-process -q); echo "provider $?"
test -e engine/crates/shell/tests/local_process.rs; echo "tests $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `commits` alone (a missing commit is the coordinator's to fetch), `stacked 0`
(080's base holds 079's engine tree), `0`, a number above 0 (`7` on `55d889bc`; order 218 may
reword some), a number of 9 or more, `provider 0`, `tests 1` and `0`.

## Scope

Change `engine/crates/shell/` (its sources, tests and `Cargo.toml`), `engine/Cargo.lock`,
`engine/crates/core/src/` for decisions 4 and 5 alone, `engine/CONFORMANCE.md`,
`engine/README.md`, and `engine/STANDING.md` only by the tool. Create
`engine/crates/shell/tests/local_process.rs` and the files that 079's and 080's diffs create.
Change nothing under `spec/`, `conformance/`, `packages/`, `experiments/`, `tools/`, `docs/`,
`engine/crates/loader/` or `engine/crates/conformance/`, nor `IN_FORCE.md`, `ROADMAP.md` or
`docs/AUDIT.md`. No case changes, so there is no declaration. Anything else goes in your report.

## Done when

Write the six tests below first. Each fails on `main`'s engine (a compile error counts where it
needs 079's or 080's interfaces), and tests 4 and 6 fail again once decisions 1 and 2 are applied
and before decisions 4 and 5: report each failing line. Tests 1 to 5 are in
`engine/crates/shell/tests/local_process.rs`, each with decision 6's Resource and a store whose
`https` address (079's `--https-url`) names a host that is not loopback, which no test reaches:

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
6. `runtime_provider_unproved_url_reports_the_part_unavailable`, in
   `engine/crates/shell/tests/provider_calls.rs`: a stand-in provider (080's `runtime_support`)
   answers `run.v1` with the url of a TLS listener whose certificate the store's root did not
   issue; the listener reads no HTTP request, the run part reports unavailable with decision 4's
   diagnostic, and `Running` is `True` (PRV-028, IFC-017, PRV-009).

Then each passes, with the rest:

```sh
(cd engine && cargo fmt --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd engine && cargo test --release -p conduit-shell --test local_process <name>)  # once per test 1 to 5: 1 passed
(cd engine && cargo test --release -p conduit-shell --test provider_calls)  # 080's tests and test 6 pass
(cd engine && cargo test --release -p conduit-shell --test http_bridge)     # 080's tests and decision 2's regressions pass
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, every vector among it
(cd packages/provider-local-process && cargo test --release)  # unchanged; passes
git add -N engine                             # so the checks see the new files
python3 tools/check_docs.py                   # Errors: 0
python3 tools/promise_standing.py --check     # Errors: 0
python3 tools/check_private_paths.py          # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 0 changes; 0 undeclared; 0 errors; exit 0
git diff --check                              # prints nothing
git diff --name-only HEAD -- spec conformance packages experiments tools docs engine/crates/loader engine/crates/conformance IN_FORCE.md ROADMAP.md | wc -l | tr -d ' '  # 0
cat engine/CONFORMANCE.md engine/README.md | grep -cE "$p"  # 0, with the start check's p; fails today
grep -c 'fn provider_call' engine/crates/shell/src/runner.rs  # 2 or more; fails today (0)
grep -c '^name = "rustls"$' engine/Cargo.lock  # 1; fails today (0)
grep -cE '^name = "(aws-lc-rs|aws-lc-sys|openssl|openssl-sys|native-tls)"$' engine/Cargo.lock  # 0
git reset -q                                  # stage nothing
```

Your report gives, per decision, what changed; where you adapted 079 or 080 to `main`; each
test's failing lines; the crates added and how each was found offline; each choice you made
yourself, under `own_choices`; and every check's last line. Stage nothing. The coordinator runs
`tools/gate.py`, has a fresh, read-only Opus reader read the diff
([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4), and merges.

## What follows

Named here and not in this order:

- **R-2:** the Linux node enrolled and placed through `packages/provider-ssh-runtime` and its
  machine provider (ruling 77; enrollment's core half, order 049a1, is checkpointed for `s3/m2`).
  **R-3:** a Codex Agent on the node.
- Before R-3: the Agent packages' items are package-relative `command`s, which PRV-007 keeps on the
  engine's own runner, so a `codex.v2` Agent that names a provider ends its setup failed. How an
  Agent package ships a program that a provider can run, such as a registry package or a program
  installed at an absolute path, is a choice for the owner.
- local-process keeps its owed stops in the `CONDUIT_STATE_DIR` that the engine's own runner gives
  it, which PRV-029 removes with the provider's Resource, so its statement under PRV-021 does not
  hold past its own removal: a finding for the provider's next order.
- Secrets for a run on a provider (SEC-010's runtime-provider half, PRV-011), after order 218.
- A dogfood Agent that names a provider needs the store's `https` address and its terminator
  (ruling 77, point 3); the coordinator sets them up when one first does.

## Record

Not yet run.
