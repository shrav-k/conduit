# Work order 221: orders 079 and 080 carried onto `main`, the engine calling runtime providers and reaching their runs (R-1, first part)

**Split on 2026-11-20.** One Codex worker turn has a 4-hour limit, and order 080's first round
alone took about 11 hours, so R-1 is now two orders. This one carries orders 079 and 080 onto
`main` and fixes 080's three verifier musts. [Order 226](226-engine-runtime-provider-local-process-qualified.md)
holds the rest: a url that proves nothing reporting unavailable, a provider run again after a
restart, qualification against `provider-local-process` on the Mac, and the built-in and provider
runs side by side.

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

**What.** The shell serves the provider calls the core already emits, `run.v1`, `stop.v1` and
`inspect-run.v1` (PRV-005, PRV-007 to PRV-009), through the provider's own run on the engine's
runner, and reaches each run behind the provider's gate through `conduit __http-bridge`
(PRV-025, PRV-028, IFC-017), using the store's `https` address. An instance that names no
`runtime` runs on the engine's own runner exactly as on `main` (PRV-003, PRV-006).

**Why.** On `main` the core emits these calls and the shell serves none:
`Executor::provider_call` is the trait's default, so such a setup reports its run failed and its
inspection unknown (`missing` in `engine/crates/shell/src/lib.rs`). `engine/CONFORMANCE.md` says
so (the row "DL16: executor coverage", item 7 of "What v0 does not claim"), and
`engine/STANDING.md` shows 10 of the 30 PRV promises with a passing case, none by a run on a
provider. On 2026-11-20 the owner approved the runtime provider's stream of orders: "Yes, start
R-1 now (Rec.)".

**Should this exist** (ruling 92 point 3). It is built already, off `main`, and this order adds no
mechanism. [Order 079](079-shell-https-address.md) gave the engine its `https` address (merged
into the side branch `s3/m2` as `9b8f3326`). [Order 080](080-shell-runtime-provider-calls.md)
served the provider calls and a bridge to each run (two rounds, checkpointed as `7e54117e` on
`origin/wo/080-shell-runtime-provider-calls`, its Record at `8e13bd81`, three musts left). The
engine's side, calling a provider and reaching its run, is Conduit's by PRV-005, PRV-007 and
IFC-017; the run itself is the provider's, behind the contract. TLS is rustls's, MCP is rmcp's,
and an existing tool terminates TLS at the engine's `https` address (`tailscale serve`,
[ruling 77](../proposals/ruling-77/README.md), point 3).

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1): 2,
since the gate makes a Conduit binding on a server the engine does not run itself (PRV-025); and
7, since each provider call is recorded (PRV-005, REC-001).

**When it runs.** An order of A78's stage 2, sent through Conduit under
[order 211's registration](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/211-a78-stage-2-registration.md): one Codex worker turn, a Codex
verifier and a Claude reader who run only what settles a doubt, then the local gate, which runs
the whole suite (AGENTS.md's work-order rules, DEC-033). It changes nothing under
`experiments/028-a78-dogfood/` and not the harness. By the coordinator's reading, open to the
owner's strike, the owner's answer brings 079 and 080 to `main` now, though `s3/m2` was to merge
after the S1 rerun's pin ([order 120](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/120-plan-providers.md), point 4). How you build what each
decision says must hold is yours ([ruling 89](../proposals/ruling-89/README.md)).

**Stop rule.** Where a promise's wording needs a choice that `spec/` does not make, stop there,
change nothing further, and report the promise, the choice and the options. Do not choose. Stop
and report too if no route builds offline (decision 3), or if carrying 079 or 080 needs a change
under `engine/crates/core/`.

## Findings carried, and how this part handles them

- **Crates missing from the cache.** 080's lockfile takes `rustls-platform-verifier` and
  `webpki-root-certs` through reqwest's `rustls-no-provider` feature; every rustls feature of
  reqwest 0.13.5 needs the verifier, and the Mac's offline cache holds neither crate (the start
  check's fourth line). Decision 3 handles them here.
- **The R-3 blocker** (Agent packages' items are package-relative `command`s, which PRV-007 keeps
  on the engine's own runner). Not touched here; order 226 carries it.
- **PRV-021 and local-process's state** (its owed stops sit in the state directory PRV-029
  removes with its Resource). Not touched here, since no shipped provider runs in this order;
  order 226 carries it.

## The decisions it applies

1. **Order 079, as built.** Apply `git diff 2a9b25ae 9b8f3326 -- engine` to your tree (for
   instance with `git apply -3`), resolving each conflict so that `main`'s later behavior holds and
   079's is added. Its decisions 1 to 8 stand as its file states them; do not rework them.
2. **Order 080, as built, and its last verifier's three musts.** Apply
   `git diff 9b8f3326 7e54117e -- engine` over decision 1 the same way. Its decisions stand as its
   file states them, the bridge `conduit __http-bridge` among them. Where `main` has changed what
   080 relies on since 2026-11-12 (order 214's request metadata, order 209's keyed hooks, the
   sign-in work in `runner.rs` and `lib.rs`, order 218's secret reads and PRV-026 path, order
   219's `/records` route in `server.rs`), `main`'s behavior holds. Report each place where you
   adapted 079 or 080. Then fix the three musts under "The last verifier's findings" in 080's
   Record (`git show 8e13bd81:docs/work-orders/080-shell-runtime-provider-calls.md`), each with a
   regression in `engine/crates/shell/tests/http_bridge.rs`:
   - an empty-data priming event (`id: 0`, `data:`, blank line) before initialization and before
     a later message is skipped, not emitted, while empty fields inside nonempty data are kept:
     `bridge_skips_empty_data_priming_events`;
   - the end of stdin while the initialization response is still pending: the bridge sends no
     later message, deletes any session it was given and exits 0:
     `bridge_exits_zero_on_stdin_eof_during_initialize`;
   - each request carries `Mcp-Method`, and each `tools/call` `Mcp-Name`, as MCP 2026-07-28's
     standard request headers say, with their standard encoding where a name needs it; the test
     gate in `tests/runtime_support/` refuses a request where either is missing or mismatched:
     `bridge_sends_mcp_method_and_name_headers`.
3. **No crate that the cache lacks.** Add no crate that `cargo metadata --offline --locked` cannot
   find. The bridge trusts the store's root alone and needs neither missing crate: take its HTTP
   client off reqwest's TLS features (and do not enable reqwest's internal `__rustls`), and reach
   the run's url through crates already in the cache, such as rustls 0.23 with `ring`,
   tokio-rustls 0.26, hyper 1 and hyper-util 0.1 (the versions the kit's lockfile holds). 080's
   rule stands: no `aws-lc`, OpenSSL or `native-tls` crate, and one rustls, 0.23.
4. **The built-in runner stays the default.** An instance that names no `runtime` runs on the
   engine's own runner as on `main`: the loopback address and audience, no `tls.crt` or
   `tls.key`, and its state directory under PRV-029 (PRV-003, PRV-006, PRV-022). No case changes,
   and no existing test changes but by 079's and 080's own diffs. 080's
   `runtime_provider_failures_discovery_and_tls_refusal_map_reports` and
   `runtime_provider_inspection_repairs_ended_session_and_refuses_changed_unproved_url` keep the
   mapping they assert (unknown or failed); order 226 changes them.
5. **What the documents say.** Update what `engine/CONFORMANCE.md` and `engine/README.md` say of
   runtime providers: the DL13 rows, the row "DL16: executor coverage", the sentences the start
   check's pattern `p` finds, and item 7, which keeps that the SSH runtime provider's isolation is
   unmet and says the engine has not run with that provider. The DL16 row gains one sentence
   beginning "From order 221," saying that the shell serves `run.v1`, `stop.v1` and
   `inspect-run.v1` through a provider's run on its own runner and reaches the run through the
   bridge, shown with a stand-in provider. Name what stays for order 226 with the words "until
   order 226": a url that proves nothing and a missing extension reporting unknown or failed, a
   provider ended at a restart not run again, and no shipped provider run by the engine. Say what
   stays unserved past it: remote connections, secrets for a run on a provider, the engine's own
   inspections (DL5). Run `python3 tools/promise_standing.py --write` where an engine source names
   a promise ID it did not. The coordinator writes 079's and 080's standing at the merge.

## What to read

- PRV-005 to PRV-009, PRV-019, PRV-022 and PRV-024 to PRV-030 in `spec/promises/providers.yaml`;
  IFC-017, IFC-022, IFC-037 and IFC-042 in `spec/interface/implementation.yaml`; DL13 and DL16 in
  `spec/implementation-defined.yaml`.
- 079's and 080's files (`git show 9b8f3326:docs/work-orders/079-shell-https-address.md`, and
  080's at `8e13bd81`), their decisions and 080's Record.
- The shell's `lib.rs` (`Executor`, `drive`, `missing`), `runner.rs`, `identity.rs`, `server.rs`.

## Before you change anything

The five commits below are on the side branches `s3/m2` and
`wo/080-shell-runtime-provider-calls`, not on `main`. Root provides them in your directory before
your turn; a missing one is root's to fetch, not yours: stop and report, and fetch nothing. From
the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor ed1c0477 HEAD; echo "main $?"
for c in 2a9b25ae 396bf304 9b8f3326 7e54117e 8e13bd81; do git cat-file -e "$c^{commit}" 2>/dev/null || echo "missing $c"; done; echo commits
git diff --quiet 396bf304 9b8f3326 -- engine; echo "stacked $?"
git show 7e54117e:engine/Cargo.lock | grep -cE '^name = "(rustls-platform-verifier|webpki-root-certs)"$'
grep -c 'fn provider_call' engine/crates/shell/src/runner.rs
p='Runtime provider calls and remote connections are unserved|Runtime providers and remote connections still have|remote transport or runtime providers, which remain unfinished|It calls no runtime provider|does not yet call that provider|The first build calls no runtime|Runtime providers and remote connections remain unserved'
cat engine/CONFORMANCE.md engine/README.md | grep -cE "$p"
grep -cE '^\| PRV-[0-9]+ \|.*\| case passes \|$' engine/STANDING.md
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine/crates/shell && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "offline $?"
test -e engine/crates/shell/tests/provider_calls.rs; echo "tests $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `commits` alone, `stacked 0` (080's base holds 079's engine tree), `2` (the
two crates the cache lacks), `0`, `5`, `10`, `offline 0`, `tests 1` and `0`. `--filter-platform`
keeps cargo from asking for other platforms' crates, which the cache does not hold either.

## Scope

Change `engine/crates/shell/` (its sources, tests and `Cargo.toml`), `engine/Cargo.lock`,
`engine/CONFORMANCE.md`, `engine/README.md`, and `engine/STANDING.md` only by the tool. Create the
files that 079's and 080's diffs create. Change nothing under `engine/crates/core/`, `spec/`,
`conformance/`, `packages/`, `experiments/`, `tools/`, `docs/`, `engine/crates/loader/` or
`engine/crates/conformance/`, nor `IN_FORCE.md`, `ROADMAP.md` or `docs/AUDIT.md`. No case
changes, so there is no declaration. Anything else goes in your report.

## Done when

Apply decisions 1 and 2's diffs, then write decision 2's three regressions and run them before
fixing: each fails on 080's tree as carried; report each failing line. Then fix the musts. 080's
tests have never run on macOS; report any that fails here and how you fixed it.

Each line in the first group prints otherwise on `main` at `ed1c0477` (shown as "today") and
passes after. The second group are guards, which pass today and must still pass. Run the whole
suite once, last.

```sh
(cd engine && cargo test --release -p conduit-shell --lib https_tests::)   # 15 or more passed; today 0 passed
(cd engine && cargo test --release -p conduit-shell --test cli https_)      # 1 or more passed; today 0 passed
(cd engine && cargo test --release -p conduit-shell --test provider_calls)  # 080's tests pass; today no such test target
(cd engine && cargo test --release -p conduit-shell --test http_bridge)     # 080's tests and the three regressions pass; today no such test target
grep -cE 'fn (bridge_skips_empty_data_priming_events|bridge_exits_zero_on_stdin_eof_during_initialize|bridge_sends_mcp_method_and_name_headers)\(' engine/crates/shell/tests/http_bridge.rs  # 3; today no such file
grep -c 'fn provider_call' engine/crates/shell/src/runner.rs  # 2 or more; today 0
grep -c '^name = "rustls"$' engine/Cargo.lock                 # 1; today 0
cat engine/CONFORMANCE.md engine/README.md | grep -cE "$p"    # 0, with the start check's p; today 5
grep -c 'From order 221,' engine/CONFORMANCE.md                # 1; today 0
cat engine/CONFORMANCE.md engine/README.md | grep -c 'until order 226'  # 1 or more; today 0
```

```sh
(cd engine && cargo fmt --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd engine/crates/shell && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "offline $?"  # offline 0, with the start check's h
grep -cE '^name = "(aws-lc-rs|aws-lc-sys|openssl|openssl-sys|native-tls|rustls-platform-verifier|webpki-root-certs)"$' engine/Cargo.lock  # 0
git add -N engine                             # so the checks see the new files
python3 tools/check_docs.py                   # Errors: 0
python3 tools/promise_standing.py --check     # Errors: 0
python3 tools/check_private_paths.py          # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 0 changes; 0 undeclared; 0 errors; exit 0
git diff --check                              # prints nothing
git diff --name-only HEAD -- spec conformance packages experiments tools docs engine/crates/core engine/crates/loader engine/crates/conformance IN_FORCE.md ROADMAP.md | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, every vector among it; once, last
git reset -q                                  # stage nothing
```

Your report gives, per decision, what changed; where you adapted 079 or 080 to `main`; each
regression's failing line before its fix; the crates the lockfile gained and how each was found
offline; each choice you made yourself, under `own_choices`; and every check's last line. Stage
nothing. The coordinator runs `tools/gate.py`, has a fresh, read-only Opus reader read the diff
([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4), and merges.

## What follows

Order 226, once this merges: the rest of R-1, with R-2 and R-3 and the findings above after it.

## Record

Not yet run.
