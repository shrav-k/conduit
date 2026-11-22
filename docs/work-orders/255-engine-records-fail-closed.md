# Work order 255: the engine fails closed when a record cannot be kept (REC-007, SEC-016): the reply at once, the record late, and recovery's branch for a read never dispatched

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave `vectors`
empty, since no case changes. This order needs no network: its tests bind loopback ports alone, and
every crate it builds is cached: run cargo offline, and if a crate is missing, stop and report, and
fetch nothing. Run one cargo command at a time. Never print a whole large file, case, diff or log:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`,
never print an environment variable's value, and never read or print a key, a token or any
credential, even one a test made. Do not run `conduit` outside a test, `codex`, `claude`, the
dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user or machine
name, or private repository name.

## What and why

**What.** [DEC-044](../decisions/log.md#dec-044-2026-11-21-records-fail-closed-and-research-before-deciding):
SEC-016, "Unavailable now, record late (Recommended)"; a request whose record cannot be kept,
"Can this work in parallel? Then we should do spec and engine now". Order 254 writes REC-007,
SEC-016 as its case for reads of secrets, OPS-023's order and IMP-007's sentence on a request
never dispatched; this order builds them.

**Today.** The core is pure. `Shell::commit` writes the whole State, records with it, to
`state.json` before it executes an intent or gives a reply (DL2). When `Store::write_state`
fails, at any of its four stages, nothing is dispatched, but the error goes up, and the server
answers `tools/call` with a JSON-RPC internal error holding the write's text and no ruleId; a
caller waiting on an invocation whose completion failed to commit waits until its deadline;
nothing of the failed transition is ever recorded (CONFORMANCE's DL10 row). The test
`a_read_whose_dispatch_record_cannot_be_kept_is_not_dispatched` pins it for SEC-016. At a
restart `secrets::recover` ends every read in progress unknown, one never dispatched among
them.

**The fault hook.** `Store::fault` in `store.rs`, under `cfg(test)`, holds one `WriteFault`
(`Write`, `FileSync`, `Rename`, `DirectorySync`) that the next state write takes. This order may
extend it, under `cfg(test)` alone, to fail writes in a row.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2, no
access is acted on without the record that makes it inspectable, and the caller is told so with
a ruleId; 6, an invocation whose end cannot be recorded ends unknown, so nothing repeats it
implicitly. None at risk: no vector changes, and a commit with no failed write and nothing held
runs as today.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, as order 254 argues (Vault's audit rule). The least change keeps the one seam, the
shell's failed write, answered by one more pure step. Not taken: a journal beside `state.json`
(a second file on the failing store); a timer that retries writes (DL10 runs none; the next
commit retries); a copy of the State per commit, to restore it after a failed rename.

**When it runs.** Order 254 must have merged: the start check stops otherwise. Through Conduit's
dogfood: one Codex worker turn (a 4-hour limit; aim for under 2 hours), a Codex verifier and a
Claude reader, who run only what settles a doubt, and the gate, which runs everything (DEC-030,
DEC-033). Beside the S1 chain, orders 243 to 250, it changes neither `main.rs` nor `help.rs`, and
shares the shell's `lib.rs` and `store.rs`, its edits there kept to `Shell::begin`, `drive`,
`commit` and `Store::write_state`. Order 256 runs now too: both may change core `requests.rs` and
`sign_in.rs`, and both change `engine/CONFORMANCE.md` (256 its DL18 row) and `engine/STANDING.md`.
Whichever merges second rebases and runs `promise_standing.py --write` again. By ruling 89, each
decision says what must hold; how is yours. The coordinator's, open to the owner's strike: the test
names, the cause words, decisions 5 and 6, that 2.2 ends a dispatched invocation unknown at once
(DEC-044 logs "at recovery"), and that held records are lost if the engine stops (REC-007 keeps
each).

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- a promise's wording needs a choice that `spec/` does not make: report the promise, the choice
  and the options; do not choose;
- a correct result needs a change outside the scope;
- a vector fails, `vector_delta.py` shows a change, or a test decision 8 does not name fails;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The writes that count.** A record cannot be kept when a write of the commit returns an
   error, at any stage, an expiry write before the step among them. Decisions 2 to 5 cover the
   inputs that send a request, `invoke` and `register-project` (and `request`, `submit` and
   `access`, which embeddings send), and those that go on with an invocation already dispatched,
   `complete`, `setup-report`, `respond`, `sign-in-wait-ended` and `cancel`. Every other input
   fails as today (`first-start`, `recover`, `list-tools`, `export-records`, `expire-outputs`,
   `builtin-run-stopped`, `builtin-secret-provider-exited`): REC-007 names none of their records
   but `recover`'s ends of invocations, which a failed write leaves in progress for the next
   open's `recover` to end, late.
2. **The fail-closed transition** (REC-007, SEC-016, SEC-012, OPS-022, OPS-023). Where the write
   of transition T, stepped for input X, fails before its rename, the shell executes none of T's
   intents and gives none of its replies, as today, and steps X's fail-closed transition F from
   the State the store holds. In F:
   1. X's own request is not dispatched and ends unavailable, whatever T decided, refused among
      them, with exactly one diagnostic, `level` `error`, `ruleId` `REC-007`, and two records:
      an `access-decision` that it was not dispatched, `reason` `unavailable`, `ruleId`
      `REC-007`, and its `request-ended`. One that cannot be read keeps T's invalid (OPS-023),
      its record kept late too.
   2. The invocation that X goes on with, but a read under 2.3, ends unknown: an error
      diagnostic naming REC-007, and `invocation-ended` and `request-ended` records that say
      unknown; what it serves goes on as for any invocation ended unknown (a setup counts the
      part under OPS-022).
   3. A read of a secret that T would have dispatched, or whose discovery X answers, is not
      dispatched and ends unavailable with 2.1's records naming SEC-016, as `secrets::complete`
      ends one under IFC-037; SEC-012 then holds: its run or connection is not asked for or
      made, and its setup or retirement ends as OPS-022 counts.
   4. F dispatches nothing: a request it would dispatch ends as in 2.1, or 2.3 for a read, so no
      caller waits on F. Its `cancel` and `cancel-setup-parts` intents, which start nothing, are
      sent at once.

   One way: a core input that carries X and marks its step fail-closed, and one place after the
   step that ends each request whose dispatching intent it emitted, until it emits none
   (`requests::reject`, `secrets::end`, `lifecycle::part` and `explain::outcome_cause` are the
   shapes).
3. **The reply at once, the record late.** The shell writes F and, whether or not that write
   succeeds, gives F's replies at once, the primary one and each `request_replies` entry: the
   `tools/call` result is F's, never an internal error. Where F's write fails too, the shell holds
   X with its evidence, and before any later input's step it steps the held inputs fail-closed
   again, in order, from the State the store holds, and writes them before that input's records;
   the step is pure, so the records kept are those whose replies were given, and the later input
   is stepped from the State they give. An input leaves the hold once a write that holds it has
   reached its rename: none is kept twice, none lost while the engine runs. Memory equals disk
   after every write (DL2). If the engine stops first, the held records are lost (decision 8).
4. **The cause words** (IFC-050; ruling 52). `REASON_CAUSES` in `explain.rs` gains `REC-007`,
   "A request is dispatched only once its record is kept; the engine could not keep it, and keeps
   it if its store can be written while it runs.", and `SEC-016`, "A secret is read only once the
   read's record is kept; the engine could not keep it, and keeps it if its store can be written
   while it runs."; `OUTCOME_CAUSES` gains `REC-007`, "The engine could not record how the
   invocation ended, so its outcome is unknown; the engine keeps the record if its store can be
   written while it runs." No test in `explain.rs` quotes them.
5. **A write that fails after its rename** (the coordinator's reading). Where the directory's
   fsync fails, the file holds T, which DL2 reads back. F is then stepped from T: each request
   that T records as dispatched, none of which was sent, ends unknown naming REC-007 as in 2.2,
   T's other replies are given, F dispatches nothing, and decision 3 keeps it. REC-007's "already
   dispatched" reads on a dispatch recorded and not sent, as IMP-007 would at a restart.
6. **A read never dispatched ends unavailable at a restart** (SEC-016, IMP-007; the
   coordinator's reading). In `secrets::recover`, a read in progress whose dispatch the store
   does not record, in phase `deciding` (waiting for a provider restart) or `discover`, ends with
   2.1's two records naming SEC-016, and no `invocation-ended`. A read records its decision with
   its dispatch, so such a read has neither; it passed its access checks and was never sent: the
   "decided and not dispatched" of 254's IMP-007 and SEC-016. A read whose dispatch is recorded
   ends unknown, as today; the setup or retirement it served ends as recovery ends any.
7. **SEC-024's IPv6 spellings** (254's "What follows"). `sec_024_reads_the_host_as_written` in
   `packages.rs` accepts `http://[0:0:0:0:0:0:0:1]/mcp` and `http://[::0:1]/mcp` too, which pass
   today.
8. **Tests**, in `engine/crates/shell/src/tests.rs`, on the hook. Write them first, with the
   hook's extension, run them on the otherwise unchanged code and report each failing line.
   - `a_request_whose_record_cannot_be_kept_reports_unavailable`: on `established`, agent-b's
     `invoke` of `project.file.write.v1`, in three runs, each failing the next two writes at one
     stage, `Write`, `FileSync` or `Rename`: `submit` returns a result with `isError` true,
     `_meta` reason `unavailable` and one diagnostic, `ruleId` `REC-007`; the executor is asked
     for nothing; the store holds no record of it; a `list-tools` then commits, and the store
     holds first the request's two records of 2.1. A tool not presented to agent-b gives the same
     result, not ACC-001's `-32602`. Today `submit` errs.
   - `an_invocation_whose_end_cannot_be_recorded_ends_unknown`: a `project.file.write.v1` begun
     with `Deferred`, the next two writes failing: `complete` with a result returns no error;
     the ticket's reply has the outcome unknown, a `REC-007` diagnostic, not the result's text;
     after a `list-tools`, its `invocation-ended` and `request-ended` say unknown. Today
     `complete` errs.
   - `a_setup_whose_records_cannot_be_kept_asks_for_no_run`: on the SEC-016 test's fixture, one
     write fails as the executor answers the first `inspect-run.v1`: no `run.v1` and no
     `server/discover` is asked for; `submit` returns the setup's final result, outcome unknown,
     naming REC-007, and its records are kept. Today `submit` errs.
   - `a_write_that_fails_after_its_rename_ends_the_undispatched_request_unavailable`: the first test's `invoke`,
     one write failing at `DirectorySync`: outcome unknown naming REC-007, no executor call, and
     the store holds the decision, the dispatch and the end unknown. Today `submit` errs.
   - `a_read_whose_record_cannot_be_kept_ends_unavailable_at_once` replaces
     `a_read_whose_dispatch_record_cannot_be_kept_is_not_dispatched`, keeping its probe and its
     checks of no read, no consumer run and the value nowhere: `submit` returns the setup's final
     result, the read's two records name SEC-016, no `invocation-dispatched` exists, and the
     consumer's run part reports unavailable. Today `submit` errs.
   - `an_undispatched_secret_read_ends_unavailable_at_recovery`: that probe, the next two writes
     failing; the shell is dropped and the store opened again: the read has decision 6's
     records, the setup ends unknown, the value is nowhere. Today the read ends unknown.
   - Change no other test: the guards' fault tests pass unchanged (`commit` alone, and
     `list-tools`, still return the write's error).

   The documents: in `engine/CONFORMANCE.md`'s DL10 row, "A store write/fsync error stops that
   shell operation as an embedding error; no intent or successful reply is issued for that
   operation." becomes "From order 255, a failed store write dispatches nothing: a request not
   yet dispatched reports unavailable (REC-007, SEC-016 for a read of a secret), an invocation
   already dispatched ends unknown, and their records are kept by the next write that succeeds;
   no timer retries, and records held when the engine stops first are lost. A failed write for
   any other input still stops that shell operation as an embedding error." Keep
   `From order 255,` on one line. DL2's dispatch boundary, after "delivering a final reply",
   gains ", but for DL10's failed write". Then run `python3 tools/promise_standing.py --write`.

## What to read

REC-001, REC-007, OPS-022, OPS-023, IMP-007, SEC-012, SEC-016 and IFC-050. The shell's `lib.rs`
(`begin`, `commit`, `persist`, `retention`, `drive`), `store.rs` and `tests.rs`; the core's
`lib.rs` (`Input`), `requests.rs` (`reject`, `recover`), `secrets.rs`, `lifecycle.rs` and
`explain.rs`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
j() { tr '\n' ' ' < "$2" | tr -s ' ' | grep -oF -- "$1" | wc -l | tr -d ' '; }
git merge-base --is-ancestor f7b1af11 HEAD; echo "main $?"
grep -c '^- id: REC-007$' spec/conventions/records.yaml
j 'This is REC-007 for a read of a secret.' spec/promises/secrets.yaml
git grep -l 'REC-007' -- engine/crates | wc -l | tr -d ' '
grep -c 'pub fault: RefCell<Option<WriteFault>>' engine/crates/shell/src/store.rs
grep -c 'fn a_read_whose_dispatch_record_cannot_be_kept_is_not_dispatched' engine/crates/shell/src/tests.rs
grep -c 'A store write/fsync error stops that shell operation as an embedding error' engine/CONFORMANCE.md
git grep -lE 'fn (a_request_whose_record_cannot|an_invocation_whose_end_cannot|a_setup_whose_records_cannot|a_write_that_fails_after_its_rename|a_read_whose_record_cannot|an_undispatched_secret_read)' -- engine | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

Once order 254 has merged they print `main 0`, `1`, `1`, `0`, `1`, `1`, `1`, `0`, `engine 0` and
`0`. On `main` at `f7b1af11` lines 2 and 3 print `0`: 254 has not merged; stop.

## Scope

Change `engine/crates/shell/src/lib.rs`, `store.rs` (the stage a write failed at; the hook under
`cfg(test)`) and `tests.rs` as decision 8 allows; `engine/crates/core/src/` for decisions 2 to 7
and their unit tests; `engine/CONFORMANCE.md`'s DL10 row and DL2's clause; and
`engine/STANDING.md` only by the tool. Change nothing else: not `spec/`, `conformance/`, `docs/`,
`packages/`, `tools/`, the loader, another shell source (`main.rs`, `help.rs` and `server.rs`
among them), `engine/crates/shell/tests/`, any `Cargo.toml` or `Cargo.lock`. Report anything else.

## Done when

Each line of the first group prints otherwise on `main` once order 254 has merged ("today") and
passes after. The second group are guards, which pass today and must still pass. Run the last
line, the whole engine suite, once, last.

```sh
ok='^test result: ok\. 1 passed'
t() { (cd engine && cargo test --offline --release -p conduit-shell --lib "$1" 2>&1 | grep -c "$ok"); }
t a_request_whose_record_cannot_be_kept_reports_unavailable  # 1; today 0
t an_invocation_whose_end_cannot_be_recorded_ends_unknown  # 1; today 0
t a_setup_whose_records_cannot_be_kept_asks_for_no_run  # 1; today 0
t a_write_that_fails_after_its_rename_ends_the_undispatched_request_unavailable  # 1; today 0
t a_read_whose_record_cannot_be_kept_ends_unavailable_at_once  # 1; today 0
t an_undispatched_secret_read_ends_unavailable_at_recovery  # 1; today 0
grep -c 'fn a_read_whose_dispatch_record_cannot_be_kept_is_not_dispatched' engine/crates/shell/src/tests.rs  # 0; today 1
grep -cF 'keeps it if its store can be written while it runs.' engine/crates/core/src/explain.rs  # 2; today 0
grep -cF 'The engine could not record how the invocation ended, so its outcome is unknown' engine/crates/core/src/explain.rs  # 1; today 0
grep -cF '"http://[0:0:0:0:0:0:0:1]/mcp"' engine/crates/core/src/packages.rs  # 1; today 0
grep -c 'From order 255,' engine/CONFORMANCE.md  # 1; today 0
grep -c 'A store write/fsync error stops that shell operation as an embedding error' engine/CONFORMANCE.md  # 0; today 1
grep -cE '^\| REC-007 \| convention \| must \| 0 \| 0 \| outside \| core, shell \| no case, no reason \|$' engine/STANDING.md  # 1; today 0
grep -cE '^\| SEC-016 \| convention \| must \| 0 \| 0 \| outside \| core, shell \| no case, no reason \|$' engine/STANDING.md  # 1; today 0
```

```sh
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-shell memory_equals_disk_after 5  # 1: the four stages and the step failure
s conduit-shell commit_measures_one_line_per_commit 1  # 1
s conduit-shell state_write_fault_is_private 1  # 1
s conduit-core sec_024_reads_the_host_as_written 1  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/(shell/src/(lib|store|tests)|core/src/[a-z_]+)\.rs|engine/(CONFORMANCE|STANDING)\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Your report gives, per decision, what changed and where; each new test's failing line, then its
passing line; your own choices; and every check's last line.

Stage nothing.

## What follows

Named here and not in this order:

- The inputs decision 1 leaves as today, and a store that cannot be read (ACC-037), still an
  embedding error.
- An error after a write that succeeded (`commit`'s second expiry check,
  `clean_state_directories`) still drops that commit's intents and replies, though it is kept.
- Whether REC-006 shows, at the next start, records held when the engine stopped first.
- Found while writing this order: a read that needs discovery sends `server/discover` before its
  access decision is recorded; whether ACC-002's order reaches that call is a later reading.
- DEC-044 is done once this order and the PRV-007 research are.

## Record

Run on 2026-11-21 and 22 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-1's first turn** (21:39 to 21:41) stopped before any edit, as the stop rule asks: two
decisions met REC-007 as order 254 merged it. The coordinator decided both, open to the owner's
strike, and widened the scope to REC-007's observable and its CHANGES line:
- records held to be kept late are lost if the engine stops before a record can be kept, and
  recovery then ends what is left as IMP-007 does; REC-007 now says so;
- a dispatch that a write records but never passes to the implementation, as when the directory's
  sync fails after the rename, was not dispatched: it ends unavailable while the engine runs, its
  end appended, and unknown at a restart. Decision 5 and its test, now
  `a_write_that_fails_after_its_rename_ends_the_undispatched_request_unavailable`, follow.

An implementation's reason with no outcome stands (REC-007 as merged), with a regression.

**Worker-1's second turn** (21:42 to 22:29) built the order with those changes: seven regressions
failed on `main` and pass. The whole engine suite passed once (527 passed).

**Two blind reads at once** (22:30 to 22:37), the owner's answer for A77's window:
`turn-verifier-1.turn.v1` found three musts, each reproduced in a core probe the regressions
missed; the Claude reader passed it with two nits and found none of them. One fix turn (22:37 to
23:09), each must reproduced by a failing regression first:
- recovery ended unknown an ordinary request decided and never dispatched; it now ends
  unavailable under REC-007, its access decision kept;
- an engine-generated discovery rejection could be taken for an implementation's reason; the
  validated reason now travels with the completion;
- a completion whose end record reads back after its rename is kept while the engine runs, though
  its directory's sync failed (the coordinator's decision, open to the owner's strike); DL10 says
  so, and a regression asserts the result given;
- a secret read recorded and not sent names SEC-016, and its cause words say the record was
  written and read back but the write reported a failure.

163 core tests pass; the whole engine suite passed once more (531 passed). The gate's run on the
squash is the coordinator's, at the merge.
