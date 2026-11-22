# Work order 280: the merge queue's gate runs its cheap rows first and stops at the first required failure, runs its test rows beside the engine test, and records the load

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches nothing. Run no `cargo build` or `cargo test` yourself and
no whole engine suite; your tests are the tool tests, whose own probe crate is theirs, and the
plan runs below. Run `tools/gate.py` only as `--plan`, as "Done when" does. Never print a whole
large file, diff, log or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Do not run `conduit`, `codex`, `claude` or the dogfood
harness, and sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file
you write is public: no absolute local path, user or machine name, or private repository name.

## What and why

**The owner's answers** of 2026-11-22 ([the decision log](../decisions/log.md)): DEC-060, "Offload
+ try runs (Recommended)": every order's gate runs in full on the Mac before it merges (ruling 80
point 10 unchanged); DEC-062, "Measure first (Recommended)": no machine is bought or rented for
the gate until measured, and each gate run records the machine's load average. Merges wait on
`tools/gate.py`, and CI stays a non-blocking second opinion
([ruling 80](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-80/README.md)).

**Today**, by the coordinator's measurements of 2026-11-22. The gate runs its rows one after
another and runs every row after a failure. One engine order took 1949.9 s to gate (order 269b),
1765.1 s of it the engine test. The five recent gates that also ran every package (orders 265,
257, 267b, 263 and 261) took 1590 to 2804 s: their other rows took 405 to 812 s before the engine
test's 1185.5 to 1991.6 s. Of 31 engine gate runs since 2026-11-21 12:00, 7 failed. Two were
cheap rows, order 265's names check and then its fmt; the fmt failed at 0.8 s and the engine test
still ran. Only engine-test failures stopped early (at 231 to 439 s), since `cargo test` stops at
its first failing test binary. The gate records no load, so load's share of the slowdown is
inferred, not measured. The coordinator's research names this order's options G3b and G3; G1, G2 and the rest
are not (see "Not in this order").

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): none
served and none at risk. The order changes no promise, engine code, package or case, and cuts no
check: a stopped run is a FAIL, and a PASS still needs every required row run and passed.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: CI cancels a matrix's other jobs at its first failure and runs
independent jobs at once (GitHub Actions' `strategy.fail-fast`, on by default; `cargo test` stops
at the first failing binary unless given `--no-fail-fast`), and `getloadavg(3)` is the system's
own load figure. It adds two flags and a scheduler inside `tools/gate.py`, the named Python
exception (HOW-WE-WORK rule 2). Not taken: stopping early by default, since a person's run should
show every failure at once; a bigger share beside the engine test, which is the critical path.

**When it runs.** On `main` once DEC-060 to DEC-062 are logged. Through Conduit's dogfood: one
Codex worker turn (a 4-hour limit; aim for under 2 hours), the two blind reads (DEC-050), and the
gate (DEC-030). By ruling 89, each decision says what must hold; how is yours. The flags' names,
the texts and the budget are the coordinator's, open to the owner's strike. Passing the flag from
the merge queue is the coordinator's, not this order's, as order 223 left `--target-cache`.

**Stop rule.** Stop, change nothing further, and report where:

- a correct result needs a change outside the scope: an engine, package or experiment test, a
  package's packing, `.github/workflows/`, or `engine/.config/nextest.toml`;
- you find a test of one directory that writes inside another directory of the tree, beyond its
  own `target/` and `dist/`: rows run side by side would race on it;
- the tool tests do not pass on BASE.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The flags: `--merge-queue` and `--one-at-a-time`.** Without the first, the gate runs the
   same rows, in the same order, one at a time, with the same commands, environment and lines as
   today, apart from decision 4's line and the new fields: `stopped_at` (null without
   `--merge-queue`) and each row's `started` are in every record. With it, decisions 2 and 3
   apply; with `--one-at-a-time` too, decision 2 alone: one row at a time in its order, with the
   full jobs and threads and no suffixes. `--one-at-a-time` without `--merge-queue` is an
   argument error. `--plan --merge-queue` lists the rows in the order decision 3 starts them,
   with its suffixes, and runs only discovery, as `--plan` does today. `settings` gains
   `merge_queue` and `one_at_a_time` (true or false), and `side_jobs` and `side_test_threads`
   (decision 3; null where it does not apply).

2. **Cheap rows first, and the first required failure stops the run** (G3b).
   - The order: today's rows in today's order, except that the tool tests and every `test` row,
     each Rust directory's and the engine's, move after the last fmt and clippy row. A NOT RUN
     row that has no command keeps its place. The cheap rows (discovery, identity, the docs job's
     checks, whitespace, vector changes, decisions, secrets, every fmt and every clippy) took
     55 to 105 s on order 269b's gate and the five above, of gates of 1590 to 2804 s; the test
     rows took the rest. So a cheap failure costs at most about 2 minutes, not a whole gate.
   - Once a required row fails (FAIL, a timeout among them; rust discovery too), no row starts,
     and every row still running is ended with its process group, as a timeout ends one.
   - A row not started is NOT RUN, its `why`:
     `not run: the merge queue's gate stops at its first required failure, NAME`.
     A row ended is NOT RUN with its `seconds` and its log, its `why`:
     `ended after N s with its process group: the merge queue's gate stops at its first required failure, NAME; its log is partial`.
   - The record gains `stopped_at`: NAME, or null. The last line becomes
     `Gate: FAIL. Failed: ...; required, not run: ...; stopped at NAME under --merge-queue, so K rows did not run (the record names each and why); N passed. Record: ...`.
     The rows the stop left out are counted there, not listed under "required, not run".
   - A stopped run is a FAIL and exits 1. Every merge still passes the whole gate (DEC-060).

3. **Test rows side by side and beside the engine test** (G3).
   - What may not overlap. The engine's tests build and pack packages in each package's own
     `target/` and `dist/` (order 223's R1 kept these builds "as today"). In
     `engine/crates/shell/tests/`, github.rs and s2.rs both pack provider-file, agents.rs packs
     both Agent packages, and the package binaries write their archives with plain `fs::write`
     (`packages/provider-file/src/bin/package.rs`). A package's own test packs into the same
     file and compares two packs
     (`packages/provider-file/tests/package.rs`). So a test row whose directory the engine's
     sources read (it is, is inside, or holds one of the directories `engine_paths` returns,
     those that choose the engine rows today) runs before the engine test starts, never beside
     it. Reading too many costs only time; too few is wrong.
   - The schedule. After the cheap rows, those rows run two at a time, each with the gate's
     `--jobs` and `--test-threads`. When they have all passed, the engine test starts with its
     jobs and threads as today, and the tool tests and every other test row run beside it, two
     at a time, each `cargo` row with `CARGO_BUILD_JOBS` of max(1, jobs // 2) and
     `RUST_TEST_THREADS` of max(1, test_threads // 2): 3 and 2 at the defaults. Their printed and
     recorded commands begin `CARGO_BUILD_JOBS=3 RUST_TEST_THREADS=2`. With no engine test,
     every test row runs two at a time with the full jobs and threads.
   - Why this budget, from the coordinator's measurements. The Mac has 12 cores, 8 performance
     and 4 efficiency. The engine test runs with 6 build jobs and 4 test threads, and is the
     critical path (1185.5 to 1991.6 s above). On order 269b, 9 of its 33 test binaries, 1070 s
     or 67% of its 1596 s of runs, are held by a lock, one test or a package build, and leave
     most of its 4 test slots idle. On the five gates, the test rows of directories the engine
     reads took 191 to 422 s, and the tool tests and the other test rows 152 to 286 s, the
     longest of these 70.1 s. Halved, two at a time, the rows beside the engine test add 6 build
     jobs, the 12 cores' remainder, and end long before its shortest 1185 s; the engine test
     keeps its own jobs and threads. Before it, nothing else of the gate runs, so two rows take
     the full share. Estimated saving on such gates: 250 to 500 s (the 152-286 s hidden, and
     about half of the 191-422 s if two rows at once do not slow each other, which is not
     measured). On an order like 269b, the tool tests' 84.6 s; its agent-claude-code row
     (44.9 s) stays before the engine test. The Mac's load was 18 to 23 when measured, so these
     are shares of a busy machine; decision 4 lets the next order tell. The risk: the research
     counts about 20 timing-sensitive engine tests, and one of the 7 failures above was load
     (order 257's six "serve did not listen", while other suites ran); rows beside the engine
     test add load. Decision 4's figures tell such a failure apart, and `--one-at-a-time` then
     keeps the stop without the side rows.
   - Each row's record gains `started`, a UTC time, or null for a row not started. Rows enter
     the record as they finish. Logs keep their `NN-name.log` names, numbered in decision 2's
     order. A row's line prints when it ends, and no two lines interleave. An interrupt, or
     SIGTERM under `--target-cache`, ends every running row's process group, names each, leaves
     the record incomplete and releases the lock.
   - Under `--plan --merge-queue`, with an engine test chosen, the tool tests' and each test
     row's `WOULD RUN` line ends
     `; before the engine test, since the engine's sources read its directory` or
     `; beside the engine test`, and the rows are listed in the order they would start: the
     first kind, the engine test, then the second kind.
   - Unchanged: the engine row's `cargo test`, CI's nextest profile (`fail-fast = false`),
     checks.yml, and the fingerprint rule of `--target-cache`, taken once per directory before
     its first row.

4. **The load average** (DEC-062), on every run but `--plan`, with or without the flag. The
   gate reads `os.getloadavg()` before its first row and again when it writes the result (PASS,
   FAIL or stopped), and on an interrupt where it can. The record gains `load_average`:
   `cores` (`os.cpu_count()`), `start` and `end`, each a list of the 1-, 5- and 15-minute
   figures to 2 decimals, or null where the call fails or did not happen. One line follows the
   `Environment:` line, such as `Load: 18.10 16.20 15.00 (1, 5 and 15 minutes) on 12 cores at the start; the record also keeps the end's.`
   or, where the call fails, `Load: not available on this machine, so the record holds none.`
   `--plan` prints and records neither.

5. **The documents.**
   - `AGENTS.md`, the gate's paragraph, after the sentence on `--target-cache`: "`--merge-queue`
     runs the cheap rows first, stops at the first required failure and runs test rows side by
     side; every record holds the load average (DEC-062)."
   - `docs/implementation/METHOD_AS_RUN.md`, section Load: "No input reads the load: check by
     hand with" becomes "Each gate record holds the load average at its start and end (DEC-062);
     nothing else reads it: check by hand with".
   - Leave CONTRIBUTING.md, HOW-WE-WORK.md, `docs/decisions/` and `docs/work-orders/README.md`.

6. **Tests**, in `tools/tests/test_gate.py`, with the fake runner unless said. Write them first,
   run them on the unchanged gate, and report each failing line.
   - `test_without_merge_queue_keeps_todays_rows`: the calls, their order, one at a time, and
     their environment as today, for a change choosing a package the engine reads, one it does
     not, the tool tests and the engine; `stopped_at` null; `started` on each row.
   - `test_merge_queue_runs_the_cheap_rows_first`: the same change with the flag; every fmt and
     clippy call before the first test call; the record's `settings`.
   - `test_merge_queue_stops_at_the_first_required_failure`: a failure planted in docs, an fmt
     row, a clippy row, a test row before the engine test, the engine test, and a row beside it.
     No later command starts; each row left out has decision 2's `why`; `stopped_at`; the last
     line; exit 1.
   - `test_merge_queue_ends_the_rows_still_running`: with the real runner, short stand-in
     commands for cargo and `GRACE` patched to 1. A row beside the engine test failing ends the
     engine test's stand-in, and the engine test failing ends the rows beside it, each within
     6 s; their logs are kept.
   - `test_merge_queue_runs_test_rows_beside_the_engine_test`: a runner that records when each
     call starts and ends. Rows the engine reads end before the engine test starts; the tool
     tests and the other package's test overlap it; never more than two run beside it; each
     command's `CARGO_BUILD_JOBS` and `RUST_TEST_THREADS` as decision 3 gives; `started` on each
     row; the plan's suffixes.
   - `test_merge_queue_interrupt_ends_every_running_row`: an interrupt while rows run beside the
     engine test ends each group, says incomplete, and leaves no lock.
   - `test_merge_queue_one_at_a_time_keeps_the_stop`: both flags; one call at a time in decision
     2's order, with the full jobs and threads; a planted fmt failure stops the run; no plan
     suffix; `--one-at-a-time` alone exits 2.
   - `test_load_average_in_every_record`: `os.getloadavg` patched; `start` and `end` for a PASS,
     a FAIL and a stopped run; null and the second line when it raises `OSError`; neither under
     `--plan`.
   - Change an existing test only where it asserts a record's exact fields.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 4f14b1db HEAD; echo "main $?"
git rev-parse HEAD                                   # BASE
git status --short | wc -l | tr -d ' '
grep -c '^## DEC-062, ' docs/decisions/log.md
grep -c "add_argument('--merge-queue'" tools/gate.py
grep -c 'getloadavg' tools/gate.py
grep -cE "^    \('tool tests', " tools/gate.py
grep -c '^def engine_paths(' tools/gate.py
grep -c 'No input reads the load: check by' docs/implementation/METHOD_AS_RUN.md
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1
```

They print `main 0`, BASE, `0`, `1`, `0`, `0`, `1`, `1`, `1`, then a line beginning `OK`.
`/usr/bin/python3` is the Mac's interpreter with PyYAML.

## Scope

Change `tools/gate.py`, `tools/tests/test_gate.py`, AGENTS.md (the gate's paragraph) and
`docs/implementation/METHOD_AS_RUN.md` (the one sentence of decision 5). Write outside these only
under `target/wo280/`, for the checks. Anything else goes in your report.

## Done when

Each line marked "fails today" prints otherwise on BASE; all pass after. The plan runs use the
new `tools/gate.py` against a second worktree of BASE, and build nothing.

```sh
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1  # begins OK
grep -cE 'def test_(merge_queue|load_average|without_merge_queue)' tools/tests/test_gate.py  # 8 or more; fails today (0)
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -k merge_queue 2>&1 | grep -E '^Ran '  # Ran 7 tests or more; fails today (Ran 0 tests)
grep -c -- '--merge-queue' AGENTS.md  # 1 or more; fails today (0)
grep -c 'No input reads the load' docs/implementation/METHOD_AS_RUN.md  # 0; fails today (1)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(tools/gate\.py|tools/tests/test_gate\.py|AGENTS\.md|docs/implementation/METHOD_AS_RUN\.md)$' | wc -l | tr -d ' '  # 0
T=target/wo280/tree
git worktree add -q --detach "$T" HEAD
G='import sys; sys.path.insert(0, "tools"); import gate; sys.exit(gate.main(sys.argv[1], sys.argv[2:]))'
g() { PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -c "$G" "$T" --base HEAD "$@"; }
o() { sed -nE 's/^WOULD RUN (tool tests|engine [a-z]+|rust packages\/provider-(file|keychain) [a-z]+): .*/\1/p' | tr '\n' ','; echo; }
for p in engine/README.md packages/provider-file/README.md packages/provider-keychain/README.md; do echo 'wo280 probe' >> "$T/$p"; done
echo '# wo280 probe' >> "$T/tools/check_roadmap.py"
F='rust packages/provider-file'; K='rust packages/provider-keychain'; E='engine fmt,engine clippy'
test "$(g --plan | o)" = "tool tests,$F fmt,$F clippy,$F test,$K fmt,$K clippy,$K test,$E,engine test,"; echo "today $?"  # today 0
test "$(g --plan --merge-queue | o)" = "$F fmt,$F clippy,$K fmt,$K clippy,$E,$F test,engine test,tool tests,$K test,"; echo "queue $?"  # queue 0; fails today (queue 1)
test "$(g --plan --merge-queue --one-at-a-time | o)" = "$F fmt,$F clippy,$K fmt,$K clippy,$E,tool tests,$F test,$K test,engine test,"; echo "one $?"  # one 0; fails today (one 1)
g --plan --merge-queue | grep -c "^WOULD RUN rust packages/provider-file test: .*; before the engine test, since the engine's sources read its directory$"  # 1; fails today (0)
g --plan --merge-queue | grep -cE '^WOULD RUN (tool tests|rust packages/provider-keychain test): .*; beside the engine test$'  # 2; fails today (0)
g --plan --merge-queue | tail -1  # begins "Gate plan: rust discovery passed"; fails today
git worktree remove --force "$T"; rm -rf target/wo280
```

Report, per decision, what you built and the tests that show it; each new test's failing line on
the unchanged gate; the tool tests' time before and after; your own choices under
`own_choices`; and every check's last line. Stage nothing.

## Not in this order

The research's other options, each a later order or the owner's: G1, the engine tests under
nextest, after its timed comparison; G2, the package builds in one target folder, which changes
order 223's R1 and must carry its staleness rule; the queue's skip-and-continue, which is
Conduit's own work; the PC's try runs and offload (DEC-060); CI's fixes; the vector test's own
threads (`CONDUIT_VECTOR_THREADS`); testing only each package's spec test on a `spec/` change,
which narrows the gate and is the owner's call; batches (DEC-061).

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with the owner's DEC-060 to DEC-067 stacked while they merged.

**Worker-1's turn** (10:22 to 10:37) gave `tools/gate.py` `--merge-queue`: the cheap rows (fmt,
clippy, the document checks) run first, and the first required failure stops the run, the
record naming the row that stopped it and each row that did not run; test rows run beside the
engine test within the stated budget; `--one-at-a-time` keeps the stop and drops the overlap.
Every run records the machine's load average at its start and end (DEC-062). Without the flag
the gate runs today's rows in today's order. The eight regressions failed first; 198 tool tests
pass.

**Two blind reads at once** (10:37 to 10:43): `turn-verifier-1.turn.v1` found two musts (a
failed side row could leave its process group running after the gate returned and released the
cache lock; a row admitted to a worker was recorded as started before its command launched), the
Claude reader a nit. One fix turn (10:43 to 10:49) applied all three, each must reproduced first.
202 tool tests pass and the three plan probes match.

The gate's run on the squash is the coordinator's, at the merge.
