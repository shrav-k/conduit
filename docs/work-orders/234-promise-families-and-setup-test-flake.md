# Work order 234: two follow-ups: `tools/check_decisions.py` reads promise IDs only in the standing's families, and order 225's setup-view test made deterministic

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate, and the only sockets it opens are the loopback
ports that the engine's own tests bind. Every crate it builds is in the local cache already; if a
build asks for one that is not, stop and report, and fetch nothing. Run one cargo command at a
time. Never print a whole large file, diff, log or list: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Do not run `conduit` outside a
test, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere. Every file
you write is public: write no absolute local path, no user or machine name and no private
repository name.

## What and why

**Two small follow-ups, found on 2026-11-20.**

1. *Promise IDs.* [Order 228](228-decision-framework-gate-checks.md)'s `tools/check_decisions.py`
   takes every token of two to five capitals, a hyphen and three digits as a promise ID. So
   `SHA-256`, `SHA-512`, `AES-256` or `ISO-639` in a `spec/CHANGES.md` line fails as an unknown
   promise, and in the not-done ratchet it makes a decision read as not done. Order 228's Claude
   reader found it (its Record: "left to a follow-up that limits the pattern to the promise
   families"). It fails loudly and lets nothing through, but it would refuse a correct line.
2. *A flaky test.* [Order 225](225-dogfood-cli-gaps.md)'s
   `inspect_setup_prints_one_line_per_instance`, in `engine/crates/shell/tests/serve.rs`, failed
   once in a combined run of the shell's `serve`, `cli`, `provider_calls` and `http_bridge` tests
   on a loaded Mac, and passed alone four times. It failed in its fixture `serve_failed_setups`,
   at `assert!(!submitted.status.success())`: a `conduit submit` of a project whose setup hook
   is a missing program exited 0. That assertion prints nothing, so the run left no steps to
   read. `pending_agent_lists_that_agents_actions_alone` uses the same fixture.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests);
[ruling 92](../proposals/ruling-92/README.md), point 1): none at risk, and no promise, case or
engine code changes. Order 225's `inspect --setup` and `pending --agent` serve 2 (every access
can be inspected) and 7 (everything is attributable); their tests should fail only when those
break. The checker keeps each `spec/` change tied to an owner's decision
([DEC-001](../decisions/log.md#dec-001-2026-11-20-the-decision-framework)).

**Should this exist** (point 3). Yes, both, as existing practice:

- A key that matches a pattern counts only where its namespace exists, as an issue tracker reads
  `ABC-123` in a commit message as an issue only where project `ABC` exists. The namespaces are
  read from `engine/STANDING.md`, which the tool already reads; no list is added. Not taken: a
  deny-list of `SHA`, `AES`, `ISO` and the like, which never ends; and asking writers to
  backtick such tokens, which is a rule nobody would remember.
- A flaky test gets its cause found and removed. A retry or `#[ignore]` would stop it guarding
  225's views, and the gate runs each test once, so a flake fails a merge that changed nothing
  it tests. The owner's words on tests: "how to keep it minimal until v0 is shipped to improve
  velocity" ([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)).

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for
under 2 hours), a Codex verifier and a Claude reader, who run only what settles a doubt, and the
gate, which runs everything (DEC-033). By [ruling 89](../proposals/ruling-89/README.md), each
decision says what must hold; how is yours. The test names and the stress run's size are the
coordinator's, open to the owner's strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- the cause of the flake is outside `engine/crates/shell/tests/serve.rs`: in the shell's or the
  core's code (then the test caught a real defect), or in a helper other test files share. Report
  the cause, its file and line, the forced test's failing line, and the options; do not choose;
- no cause can be shown from the code and forced by construction (decision 5). Report what you
  read and tried, the baseline stress's results and your leading reading, and keep only the
  diagnostics of decision 4;
- a correct result needs a retry, a sleep, a longer bound or timeout, `#[ignore]`, a weaker
  assertion, or the file's tests run one at a time;
- the real tree gives an `error:` line from `tools/check_decisions.py` (report it; do not weaken
  the rule or change the file it names);
- a correct result needs a change outside the scope, a crate that is not in the cache, or the
  network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

### Part 1: promise families

1. **The families.** A promise family is the part before the hyphen of the ID in a promise row of
   `engine/STANDING.md`, the rows the tool reads today. The tool takes the families of BEFORE's
   and AFTER's file together, as one set for the run.
   - A token of today's pattern is a promise ID only where its family is in that set, and never
     `DEC-NNN`.
   - Every rule that reads promise IDs uses that one set: the CHANGES lines (order 228's 3.1), and
     the not-done ratchet and the dormancy report (its 6 and 7), for BEFORE's lines and AFTER's
     alike, so both counts read the same IDs.
   - A line whose only such tokens are outside the families, such as `SHA-256`, is a line with no
     promise ID: it needs a `spec/` anchor or a definitions term, as any such line does, and in
     the ratchet it is done by its text.
   - Where neither snapshot has the file, there are no families, so no token is a promise ID.
   - Nothing else changes: an ID of a known family that is not a row (`ACC-999`) still fails as an
     unknown promise; the `Removed:` rule, the messages, the output lines and the exit codes stay
     as they are. The tool still runs under Python 3.9 (the Mac's `/usr/bin/python3`) and 3.12.
2. **The tests**, in `tools/tests/test_decisions.py`, in its form, on its fixture (whose standing
   holds families `ACC` and `OPS`):
   - `test_changes_line_reads_ids_only_in_standing_families`. Each of these is its own Unreleased
     item:
     - `- Changed: SHA-256, SHA-512, AES-256 and ISO-639 in spec/README.md#overview (DEC-001).`
       gives no `CHANGES lines` finding;
     - the same tokens with no anchor and no term fail with "expected a spec anchor or a
       backticked definitions term" and not "unexpected promise standing";
     - `- Changed: ACC-999 (DEC-001).` still fails with "unexpected promise standing for ACC-999".
   - `test_changes_line_takes_families_from_before_and_after`. A family in BEFORE alone (AFTER's
     standing drops its every row) still reads in a `Removed:` line: removing its last promise
     passes, and naming one that BEFORE lacks fails with "unexpected promise standing for" it. A
     family in AFTER alone (a new row, such as `ZZZ-001`) reads in an `Added:` line: its row
     passes, and `ZZZ-002` fails with that message. Today's tool passes this test; it keeps the
     union from narrowing to one snapshot.
   - `test_not_done_reads_ids_only_in_standing_families`. A line citing DEC-001 (more than 28 days
     before the fixture's `--today`) that names `SHA-256` and `spec/README.md#overview` leaves the
     not-done ratchet's head at 0 and reports no dormant DEC-001 for that line. On today's tool its
     head is 1 and the ratchet fails.

   The `ABC-999` item in `test_changes_line_rejects_wrong_kind_citations_and_subjects` keeps
   failing, now as a line with no subject: that test's count does not change.

### Part 2: the setup-view test

3. **Find the cause from the code.** Read the path from the fixture to the assertion and name
   what lets `submit` exit 0 there, or lets the fixture talk to an engine other than its own, with
   file and line, and why load opens it. Places to start; none is established:
   - The port. `records_store` reserves a port by binding `127.0.0.1:0`, and drops the listener
     when it returns; serve binds the port later. In between, anything on the Mac may take it: the
     file's other tests run in parallel, and other worktrees' suites run on the same machine.
     `wait_for_listener` returns once anything accepts on the port, and looks at its child only
     while connects fail, so it cannot tell its serve from another listener. A serve of another
     store refuses this store's root token, so a stolen port alone makes submit exit non-zero, not
     0. It explains the failure only together with something that lets submit set nothing up.
   - Readiness. Since order 225, serve prints one ready line once it answers (its decision 5);
     `start_server` still polls TCP, and runs serve with stdout to null.
   - What submit counts. `submit` in `engine/crates/shell/src/author.rs` sets up the objects it
     finds in both the registration's reply and `state.json`, read after the reply, and exits 0
     when no step fails, so a submit that sets nothing up exits 0. Its stdout lists each step.
   - The setup's timing. What `set-up.v1` answers while the hook's run is still starting, under
     load: the guard's spawn and `initialize_timeout` in `engine/crates/shell/src/runner.rs`.
4. **Make it deterministic.** Each wait in the fixture ends on an event that proves what the next
   step needs, not on chance or the clock. For example, serve's own ready line proves that this
   serve answers. There is no retry, sleep, longer bound or `#[ignore]`, and the file's tests
   still run in parallel. Every test that uses a changed helper keeps all its assertions, and so
   does each of the two tests. When the fixture's assertions on `submit` and on the Agent's
   `call` fail, they print that command's stdout (the steps) and its stderr.
5. **Prove it.**
   - *Forced.* A test `failed_setups_fixture_forced_cause` in `serve.rs` forces the cause by
     construction: a port the test holds, a hook made slow, or a pause the test controls, but not
     load or chance. It fails on the fixture as it is today; report that failing line. Then
     change the fixture until it passes.
   - *Stress.* The Done-when block's stress run, five rounds of four copies of the whole `serve`
     test binary at once. Run it once with decision 4's diagnostics alone, before the fix, and
     report each failure with its message (the forced test fails all 20 times; the flake itself
     may not show). Run it once after the fix.

## What this leaves

Report this list, corrected by what you built: the same reserve-then-release port pattern in
`agents.rs`, `cli.rs`, `runner.rs` and `secrets.rs` under `engine/crates/shell/tests/`, if the cause is the
port; any defect in the shell's code that the cause shows (decision 3), for its own order.

## What to read

- `tools/check_decisions.py`: `PROMISE`, `promise_ids`, `standing`, `language_decisions`,
  `Check.changes`, `Check.ratchets` and `Check.run`; `tools/tests/test_decisions.py`: its
  fixture's `standing` and the `test_changes_line_` and `test_not_done_` tests.
- `engine/crates/shell/tests/serve.rs`: `records_store`, `start_server`, `wait_for_listener`,
  `spawn_server`, `terminate_server`, `serve_failed_setups`, the two tests that use it, and
  `serve_prints_one_ready_line_once_it_answers`.
- `engine/crates/shell/src/main.rs`: the `serve` arm (startup setups, then the listener and the
  ready line); `engine/crates/shell/src/author.rs`: `submit`, `submit_step`, `succeeded`;
  `engine/crates/shell/src/server.rs`: `client` and `serve_with_ready`;
  `engine/crates/shell/src/runner.rs`: `spawn` and the start of a hook's run.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor dd281795 HEAD; echo "main $?"
grep -cF 'PROMISE = re.compile(r"\b[A-Z]{2,5}-[0-9]{3}\b")' tools/check_decisions.py
grep -c 'def promise_ids(text):' tools/check_decisions.py
grep -c 'def test_changes_line_' tools/tests/test_decisions.py
grep -c 'famil' tools/check_decisions.py tools/tests/test_decisions.py
grep -c '^fn inspect_setup_prints_one_line_per_instance()' engine/crates/shell/tests/serve.rs
grep -c 'assert!(!submitted.status.success());' engine/crates/shell/tests/serve.rs
grep -c 'forced_cause' engine/crates/shell/tests/serve.rs
python3 tools/check_decisions.py HEAD WORKTREE | tail -1
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$(rustc -vV | sed -n 's/^host: //p')" >/dev/null); echo "offline $?"
git status --short | wc -l | tr -d ' '
```

They print:
- `main 0`, `1`, `1`, `6`, then two lines ending `:0`;
- `1`, `1`, `0`, `Errors: 0`, `offline 0` and `0`.

`main` printing otherwise means order 228's tool has not merged: stop. `offline` printing
otherwise means a crate is missing from the cache: stop and report, and fetch nothing.

## Scope

Change `tools/check_decisions.py`, `tools/tests/test_decisions.py` and
`engine/crates/shell/tests/serve.rs`. Change nothing else: not `spec/`, `conformance/`,
`packages/`, `experiments/` or `docs/`, no other tool or test file, nothing under
`engine/crates/*/src/`, no `Cargo.toml` and not `engine/Cargo.lock`. Put anything else that needs
changing in your report.

## Done when

Write the tests of decisions 2 and 5 first, and run them on the unchanged tool and fixture: report
each failing line (the second of decision 2 passes there). Then change the code until all pass. Each
line marked "fails today" prints otherwise on `main` at `dd281795`; all pass after. Run the
stress lines from the repository root, in one shell: once before the fix with decision 4's
diagnostics, and once after the cargo lines pass.

```sh
python3 -m unittest discover -s tools/tests -p test_decisions.py -v 2>&1 | grep -E '^test_[a-z_]*families[a-z_]* ' | grep -c ' \.\.\. ok$'  # 3; fails today (0)
/usr/bin/python3 -m unittest discover -s tools/tests -p test_decisions.py -v 2>&1 | grep -E '^test_[a-z_]*families[a-z_]* ' | grep -c ' \.\.\. ok$'  # 3, under Python 3.9; fails today (0)
(cd engine && cargo test --release -p conduit-shell --test serve -- failed_setups_fixture_forced_cause inspect_setup_prints_one_line_per_instance pending_agent_lists_that_agents_actions_alone 2>&1 | grep -c '^test result: ok\. 3 passed')  # 1; fails today (0: 2 passed)
bin=$(cd engine && cargo test --release -p conduit-shell --test serve --no-run --message-format=json 2>/dev/null | grep -o '"executable":"[^"]*/serve-[^"]*"' | tail -1 | cut -d'"' -f4); log=$(mktemp -d); test -x "$bin"; echo "binary $?"  # binary 0
for round in 1 2 3 4 5; do for copy in 1 2 3 4; do (cd engine/crates/shell && "$bin" --test-threads 4 >"$log/$round-$copy" 2>&1) & done; wait; done
cat "$log"/* | grep -cE '^test (failed_setups_fixture_forced_cause|inspect_setup_prints_one_line_per_instance|pending_agent_lists_that_agents_actions_alone) \.\.\. ok$'  # 60; fails today (40 at most)
cat "$log"/* | grep -cE '^test (failed_setups_fixture_forced_cause|inspect_setup_prints_one_line_per_instance|pending_agent_lists_that_agents_actions_alone) \.\.\. FAILED$'  # 0
cat "$log"/* | grep '^test .* FAILED$' | sort | uniq -c  # report each with its message; another test's failure does not fail this order
```

These pass, as on `main`; run the tool suite and the whole `serve` file once each, last:

```sh
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy -p conduit-shell --all-targets -- -D warnings)
git diff --check  # prints nothing
git diff --name-only HEAD | grep -vxE 'tools/check_decisions\.py|tools/tests/test_decisions\.py|engine/crates/shell/tests/serve\.rs' | wc -l | tr -d ' '  # 0
/usr/bin/python3 -m unittest discover -s tools/tests  # OK
(cd engine && cargo test --release -p conduit-shell --test serve)  # every test passes
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML. The whole engine suite is the gate's:
only `serve.rs` among the engine's files changes. Your report gives:
- each new test's name with its failing line on the unchanged code, then its passing line;
- the flake's cause, with file and line, why load opens it, and how the forced test forces it;
- the two stress runs' counts, and each failure's message;
- the corrected list of what this leaves;
- every choice you made yourself, under `own_choices`;
- every check's last line.

The verifier's turn follows; the coordinator then runs `tools/gate.py`, has a fresh, read-only
Claude reader read the diff ([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4),
and merges.

## Record

Not yet run.
