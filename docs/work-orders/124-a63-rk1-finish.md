# Work order 124 (A63-RK1, finished): close what RK1's final verifier left open

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
token or a private key, even one that a test made.

## What and why

Work order 122 (`docs/work-orders/122-a63-rk1-runtime-kit-server.md`) built
`packages/runtime-provider-kit/`, the MCP server for `runtime-provider.v1` and the backend
interfaces that every runtime provider will build on. Its two rounds are spent, and its final
verifier left three musts and one should, all in how the server keeps its promises about stops
when its state store fails. Owed stops are PRV-021's, a safety promise, and the local-process,
Docker and SSH runtimes will rely on this code. This order starts from order 122's last commit,
which is where your working tree begins, and closes the four. Order 122's decisions all still
apply; read that order and `packages/runtime-provider-kit/README.md` first.

## The decisions it applies

Fix each finding as its fix says, each with a regression test that fails on order 122's code and
passes on yours. Line numbers are those of order 122's last commit in
`packages/runtime-provider-kit/src/server.rs`.

1. **A drain triggered by a call uses that call's context** (near line 376; decision 3 of order
   122). `execute` drains before it applies the current call's context, so an owed stop recorded
   under one configuration and revision is tried with them even when the call that triggers the
   drain carries newer ones. Pass the current call's `config`, `revision` and `machine` into a
   drain that a call triggers, and refresh the matching records with them before any backend
   call. The call's `config` and `revision` are the provider's own (PRV-005), so they replace
   those of every owed record; its `machine` replaces only the machine of records with the same
   hostname, as `call_records` does (near line 536), and never another hostname's. A drain at
   startup or on the interval, with no call, keeps the saved contexts, and does not overwrite a
   fresher context already held for a hostname. Test: an owed stop recorded under revision r1,
   then an `inspect-run.v1` call under r2 with changed configuration; no backend call made after
   it carries r1, and an owed record for another hostname keeps its own machine. Trigger the drain
   with `inspect-run.v1`, not `run.v1`: a `run.v1` after an owed stop answers "start cancelled by
   stop" because its own drain advances the epoch (near lines 284, 369 and 418). Do not change
   that; report it as a finding for order 081.
2. **An obligation survives a write failure and a stop failure together** (near lines 103 to
   108). When saving the owed-stop intent and the backend's stop both fail, the persisted record
   still says nothing is owed, so no later drain retries it. Keep such an obligation in memory
   until it is durably saved, under these rules:
   - it is created only when the backend's stop failed and its intent was not saved; a stop that
     succeeded leaves nothing owed;
   - it counts as owed wherever a saved one does: it blocks a start, and inspection reports
     `unknown` for it, as a saved one does (near line 556);
   - every later drain includes it, on a call and on the interval, and re-checks it after taking
     the lock, as the drain re-reads the ledger (near lines 275 to 283);
   - any later confirmed stop of the same instance on the same hostname clears it.

   Tests: both fail together, then storage recovers, and with no further stop call the next drain
   saves the obligation and stops the run, once for a drain on a call and once on the interval;
   and a start that arrives while it is held is refused.
3. **The startup sweep cleans up every run despite write failures** (near line 161). A failed
   intent write ends the sweep before any stop is tried. Try to stop every run the backend
   reports, whatever an individual write does (as `stop_record` already does: "even on a store
   failure, try to stop the tree"), and gather the failures. Then the server refuses to serve if
   any read or write of the state store failed during recovery, and still serves when the only
   failures are stops whose owed intent was saved. The refusal is returned by
   `RuntimeServer::new` only, as a fixed reason that holds no backend text; the crate's error
   type stays as it is, since the runtimes depend on it. Test: failing writes and several
   reported runs; every run's stop is tried, and construction is refused; and a second test where
   only stops fail, their intents saved, and the server serves.
4. **Recovery keeps the contexts it learns** (should; near line 288). A drain at recovery builds
   a slot and stops a recovered record without remembering its context, so a shutdown after a
   later ledger-read failure tries no stop for that run, against the README ("attempts all known
   trees before reporting a storage failure"). Remember the context of every record found in
   recovery and draining. Test: recover a run that cannot be stopped yet, make it stoppable,
   fail ledger reads, and shut down before any tool call; its stop is tried.
5. **Nothing else changes.** No new behavior beyond these four, no change to the README's
   promises other than what these fixes make true, and no change outside the crate. If a fix
   needs a choice that `spec/` or order 122 does not make, take the narrowest, put it behind one
   function, and report it as a finding.

## Scope

Change only `packages/runtime-provider-kit/`.

## Done when

In `packages/runtime-provider-kit/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`, run twice; report both runs' totals and any test that differed

From the repository root:

- `python3 tools/check_docs.py`
- `python3 tools/set_names.py --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
D=packages/runtime-provider-kit
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
git status --porcelain --ignored $D | grep target/ | grep -v '^!!'
```

`target/` stays ignored and unstaged. Report, for each of the four, the test that shows it fixed.
Do not stage anything.

## Record

Run on 2026-11-11 on the Mac, stacked on order 122's last commit (c06b093): the worker in 7
minutes (445 s) and a fresh verifier in 6 minutes (364 s); then a second worker round in 3
minutes (187 s) and a fresh verifier in 6 minutes (359 s). It closed the four findings, with
nine regression tests that fail on order 122's code. Round one's one should (a drain waited on
unrelated instances' locks) was fixed in round two.

Round two's verifier left one must: `Slot::refresh` read the held context, released the mutex
and took it again to insert, so an interval drain could copy r1, a live call install r2, and the
drain then write r1 back. **The lane coordinator fixed it, an admin-side fix at the admin's word
(a small finding, ruling 71):** `refresh` holds the mutex once across the lookup, the update and
the insert, and stays outside the operation lock. A fresh reader checked the fix and found two
shoulds, which the coordinator also applied: `call_records` writes only this call's candidate
again, not every record, so it cannot restore a context another call installed; and the stop that
a run's lifetime triggers refreshes its record first, so it carries the newest context. Tests:
`interval_refresh_never_writes_back_an_older_context` and the reader's
`contended_interval_refreshes_never_regress_a_live_context` (which failed 30 of 30 runs on the
old code), and `lifetime_stop_uses_the_newest_call_context` (which fails with the refresh line
removed). The reader checked the follow-up too and found nothing to change. The crate's
fmt, clippy and 47 tests pass.

Left as found, a could: a failed start stops with its own call's record, not one refreshed by
another call meanwhile; both come from live calls. For order 081, the worker's finding: a
`run.v1` whose own drain clears an owed stop answers failed, "start cancelled by stop", because
that drain advances the instance's epoch.
