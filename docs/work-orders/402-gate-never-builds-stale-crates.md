# Work order 402: the gate builds every crate from the branch's own files, whatever their times (DEC-172)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network: the tests' crate has no
dependencies and cargo runs offline; if a build asks for a crate, stop and report, and fetch
nothing. Run cargo only through the tool tests, one test command at a time, and not
`tools/gate.py` or the whole engine suite. Never print a whole large file, case, diff or log: use
`head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`,
never print an environment variable's value, and never read or print a key, a token or any
credential, even one a test made. Run no `conduit`, `codex` or `claude`, and sign in nowhere.
Open, copy or read no store and no target directory but one a test makes in a temporary
directory: never the gate's cache or the dogfood's. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user,
machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-172](../decisions/log.md#dec-172-2026-11-23-the-gates-stale-build-cache-is-fixed-next):
whether the gate's stale-cache fix runs before the guard chain ends -> "Yes, next (Recommended)",
landing in "the order that keeps the gate from building a branch against a stale copy of a crate
from its shared target directory (order 278's merge met it)", ahead of the guard's remaining
orders; it amends
[DEC-166](../decisions/log.md#dec-166-2026-11-23-the-usage-guards-orders-come-first-on-both-workers)
for this one order.

**Today.** Cargo judges a crate fresh by file times: a unit whose record is newer than every file it
read is reused, and rustc is not called. Two worktrees of this repository give a path crate the same
unit (cargo hashes its path from the workspace root), so a folder they share holds one build of it,
and the second tree reuses the first's wherever its own files carry older times: a rebase leaves
them on each file it does not rewrite, a copy that keeps times (`rsync -a`, `cp -p`) on each file it
copies. Under `--target-cache DIR`, since [order 223](223-gate-speedups.md), the gate removes every
path crate's fingerprints from each folder of DIR before a run's first build there (since
[order 281](281-engine-test-package-builds-in-the-gate-cache.md) also the folders the engine's tests
build packages in), so those folders rebuild the tree's crates whatever the times; `CargoCacheTests`
checks the removal. Without the option, the rows build in each directory's own `target/`, or in a
folder that the caller's `CARGO_TARGET_DIR` or cargo's configuration names, which several worktrees
may share, and the engine's tests build packages in their own folders or in that one: in each, a
build newer than a changed file is reused; the record names no freshness step.

**278's case.** On 2026-11-23 the coordinator's account of order 278's merge reads that the gate's
shared cache folder held `main`'s older `conduit-core`, newer than 278's sources, and that a build
used it: errors on names 278 itself changes. Touching 278's core sources showed 278's own error,
order 289's `limit_observations` missing from its exhaustive State view. The gate's one record for
that head, kept privately, shows its own run rebuilt the core from the branch
(`Checking conduit-core`), as order 223's removal makes it, and failed on that own error (E0027).
The build that reused `main`'s core was therefore one the gate had not prepared; the mechanism is
the same in any shared folder, and the gate prepares only DIR's. A later build of `main` in a new
folder, without the compiler cache, passed all 47 test binaries: no merged order slipped through.

**What it costs.** Under `--target-cache`, which every merge uses, nothing more is built: each
folder a run uses rebuilds the tree's own crates already (order 223). In the queue's last three
engine records (orders 287, 297 and 299, kept privately) that was the engine clippy row, 8.6 to
11.0 s, and the engine test's build, cargo's first `Finished` at 1 min 49 s to 3 min 47 s; both
stay. The step sets about 3,400 files' times (38.6 MiB on `main`), well under a second, and Git
re-reads them once at its next `git status`. Without the option, a rerun on a tree whose folders
hold its earlier build rebuilds the tree's crates, as a first run or one after a rebase pays
today; and after any run with a clippy or test row, a later build outside the gate in that tree,
by hand or by an editor, in any folder, rebuilds them once: each up to the engine's minutes, less
what the compiler cache serves. The tool tests gain four builds of a one-file crate.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): none
served and none at risk: no promise, case, engine source or package changes. A gate PASS comes to
stand for the branch's own files, so each merged order's tests test that order.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice. Cargo compares file times; its unstable `-Zchecksum-freshness` compares
contents where times cannot be trusted, as in CI. On a stable toolchain, builds that share or
restore a target folder set the sources' times after a restore, or drop the workspace's own
crates, as the rust-cache action keeps none and order 223 keeps none in DIR's folders. This order
takes the first: one step that every folder cargo may use sees, those the gate cannot name
included. Not taken:

- `-Zchecksum-freshness`: unstable as this is written, and the engine pins the stable 1.97.1,
  where it needs a bootstrap override that nothing in the gate uses.
- a target folder per tree, keyed by its hash: each new tree rebuilds every registry crate and
  keeps a full folder, which needs eviction; the compiler cache caches neither incremental builds
  nor crates that link.
- removing fingerprints in every folder: the gate cannot name every one (a caller's
  `CARGO_TARGET_DIR`, cargo's configuration, a package's own `target/`). Order 223's removal
  stays for DIR's folders, where it holds even against a clock set back.
- each file at its last commit's time, as `git restore-mtime` sets it: still older than a later
  build from other files, which is 278's case.
- only the files the branch changes: a shared folder may hold crates built from another branch's
  changes, at paths this branch never touched.
- `CARGO_INCREMENTAL=0` or the compiler cache: neither acts on a crate cargo judges fresh.

**When it runs.** On the decision log's branch that holds DEC-172, or on `main` once it merges,
ahead of the guard's remaining orders. Through Conduit: one Codex worker turn, the blind Codex
verifier and Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling
89, each decision says what must hold; how is yours. The coordinator's, open to the owner's
strike (DEC-029): the mechanism, applying it without `--target-cache` too, keeping order 223's
removal beside it, the record's key and words, and the test names.

**Shared files.** Other gate orders, 400 among them, edit `tools/gate.py` and `test_gate.py`:
whichever merges later rebases and keeps both.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside
Scope that must change. Stop and report, too, where the first test of decision 6 passes on the
unchanged gate (give its output's last lines): the case is then not reproduced. Where this order's
wording contradicts a promise it does not change, the promise governs: apply it and list it under
`own_choices`, with every choice no decision governs.

## The decisions

Read first: in `tools/gate.py`, `run_gate` from the target-cache claim to the end of the Rust
rows, with `prepare_cache`, `invalidate_path_fingerprints` and `rust_command`; in
`tools/tests/test_gate.py`, `FakeRunner`, `GateTests.setUp` and `invoke`, `CargoCacheTests`, and
the tests whose names begin `test_target_cache`.

1. **What holds** (DEC-172). Every crate a run builds from the tree is built from the tree's files
   as they stand, over any build that a target folder held before the run's instant, in whatever
   folder cargo builds it: one of DIR's, a directory's own `target/`, one that a caller's
   `CARGO_TARGET_DIR` or cargo's configuration names, or one the engine's tests build packages in.
   It holds whatever times the files carried before the run, with or without `--target-cache`.
   Registry and git crates are reused as today.
2. **The mechanism: one instant for every file.** Once in a run, after rust discovery has chosen
   at least one clippy or test row and before the first of them starts, the gate takes one
   instant T, later than its claim of `DIR/gate.lock` where it takes one, and sets the
   modification time, and the access time with it, of every regular file that
   `git ls-files --cached --others --exclude-standard` lists in the tree to T: the whole tree,
   since the engine's crates read files outside `engine/` as they compile (`spec/names.yaml`).
   Nothing else: no ignored file, so nothing under a `target/`; no directory; no symbolic link,
   nor what one names; nothing outside the tree. A listed path that no longer exists is skipped.
   Setting follows no link (`follow_symlinks=False`, say): a path swapped for one after its check
   stays inside the tree. Cargo then finds the tree's files newer than any unit built before T, in
   every folder, and rebuilds the tree's crates once in each. With no clippy or test row, as on a
   branch that changes only documents, and under `--plan`, no time changes. Order 223's removal
   stays.
3. **It fails closed.** Where `git ls-files` fails, or a listed file's times cannot be set for any
   reason but its absence, no clippy or test row runs a command in that run: each FAILs, its why
   `cannot list the tree's files: git ls-files exit <N>` or
   `cannot set file times: <the path, relative to the root>: <the error>`, redacted as every why
   is. The fmt, Python and tool-test rows run as they would, and the merge queue stops at the
   first such row as at any required failure.
4. **The record and the output name it.** The record holds `freshness`: `null` until the step,
   and in a run without one; after it,
   `{"mechanism": "file times set to one instant before the first build", "time": T, "files": N}`,
   T in the form of the record's `started` and N the number of distinct paths set, and `"error"`
   holding decision 3's text where it applies. The output gains one line when the step ends,
   beginning `Freshness: `: N, T, and that cargo therefore rebuilds each crate from the tree over
   any build a target folder held before T; or decision 3's error.
5. **AGENTS.md and METHOD_AS_RUN.md**, at their width, `(order 402)` kept on one line.
   - In `AGENTS.md`'s gate item, after "holds `DIR/gate.lock` for the run." add: "Before its first
     build, with or without that option, it sets the time of every file Git lists in the tree,
     tracked or untracked and not ignored, to one instant, so that cargo rebuilds each crate built
     from the tree over any build a target directory held before that instant, whatever times the
     files carried; the record's `freshness` names it (order 402)."
   - In `docs/implementation/METHOD_AS_RUN.md`, after "The gate holds `DIR/gate.lock` for the
     run." add: "Before its first build, with or without the option, the gate sets the time of
     every file Git lists in the tree, tracked or untracked and not ignored, to one instant, so
     that cargo rebuilds each crate built from the tree over any build a target directory held
     before that instant, whatever times the files carried (order 402). A build by hand in one of
     DIR's folders gets neither this nor the gate's rebuild of the tree's own crates, and can
     reuse a crate another tree built there."
6. **Tests**, in `tools/tests/test_gate.py`. Write them first, run each on the otherwise unchanged
   gate and report its line. The two that build with the real cargo live in a class of their own
   beside `CargoCacheTests`, keeping the caller's environment (`GateTests.setUp` leaves `PATH` at
   `os.defpath`, where cargo is not found, and `invoke` stubs `shutil.which`), and make the
   fixture as `GateTests` does. They skip, build offline and bound each build as
   `CargoCacheTests` does, the copy's build and the gate's row alike in the environment
   `gate.scrub_environment()` gives. A wrapper captures `gate.main`'s output, as `invoke` does, so
   no printed text breaks the ` ... ok` line.
   - `test_freshness_rebuilds_over_a_newer_build_in_a_shared_target_directory` (decisions 1 and 2;
     278's case): the fixture holds `experiments/gate-probe`, its own workspace, a crate named
     `gate-probe` (as the fake runner's metadata names it) with no dependencies, whose one test
     passes on the base; the branch changes it to fail with a message naming the branch. A copy of
     the base tree outside the repository passes `cargo test --release` with `CARGO_TARGET_DIR`
     naming a shared folder; the branch's probe files then get a time before that build, as 278's
     sources had. `gate.main` runs on the branch without `--target-cache`, the caller's
     `CARGO_TARGET_DIR` naming that folder, the probe's test row on the real cargo (other commands
     may stay fake). The row is FAIL, its log holding the branch's message. On the unchanged gate
     it is PASS, cargo reusing the copy's build: report that failing line.
   - `test_freshness_rebuilds_over_a_newer_build_in_the_target_cache` (decision 1; 278's own
     setting): the same, with `--target-cache DIR` and the copy building in
     `DIR/experiments-gate-probe`, the folder the gate gives that directory. The row is FAIL with
     the branch's message. It passes on the unchanged gate too, by order 223's removal: report
     that line. It keeps the case covered should either mechanism change.
   - `test_freshness_sets_each_listed_file_once_before_the_first_build` (decisions 2 and 4; the
     fake runner; one method, each case a `subTest`), without and with `--merge-queue`: the
     fixture holds a package with clippy and test rows, a tracked file outside it, an untracked
     file, an ignored file under `target/`, and an untracked symbolic link to a file outside the
     tree, all with past times. At the first clippy or test command, every regular file Git lists
     holds one time, not before `started`, which `freshness.time` names; the ignored file, the
     link and its target keep theirs; at the end no listed file's time differs from then.
     `freshness` holds decision 4's mechanism and `files` the count of those files; the output
     holds one `Freshness: ` line. A branch that changes only `docs/note.md`, which no Rust
     directory, `ENGINE_ROOTS` or `RUST_READS` reaches, and a `--plan` run, change no time, and
     the first's `freshness` is `null`.
   - `test_freshness_fails_closed_on_a_file_it_cannot_set` (decision 3; the fake runner): setting
     one listed file's times fails with a permission error. Every clippy and test row FAILs with
     decision 3's why naming that file, the runner sees no clippy or test command, the fmt rows
     run (without `--merge-queue`), `freshness.error` holds the same text, and the last line is
     `Gate: FAIL`.
   - An existing test changes only where it compares a whole record or the output's lines, which
     the new key and line change. Report each.

## Start check

From the repository root:

```sh
k() { f=$1; shift; for p in "$@"; do grep -cF -- "$p" "$f"; done | paste -sd' ' -; }
git merge-base --is-ancestor 863ab1b7 HEAD; echo "gate $?"  # gate 0
git status --short | wc -l | tr -d ' '  # 0
k tools/gate.py 'def invalidate_path_fingerprints(target, names):' "names = {package['name'] for package in packages if package['source'] is None}" "'ls-files', '--cached', '--others'," 'utime' 'freshness'  # 1 1 1 0 0
k tools/tests/test_gate.py 'def test_target_cache_rebuilds_for_a_second_tree_with_an_explicit_host_target' 'def test_freshness_'  # 1 0
k AGENTS.md "rebuilds the tree's own" 'order 402'  # 1 0
k docs/implementation/METHOD_AS_RUN.md 'The gate holds `DIR/gate.lock` for the run.' 'order 402'  # 1 0
command -v cargo rustc | wc -l | tr -d ' '  # 2
```

Where a line prints otherwise, stop, change nothing and report it. A third line other than
`1 1 1 0 0` means the gate's preparation moved since this order was written.

## Scope

Change `tools/gate.py` for decisions 1 to 4, `tools/tests/test_gate.py` for decision 6, and the
one sentence each in `AGENTS.md` and `docs/implementation/METHOD_AS_RUN.md` for decision 5. Change
an existing test only under decision 6. Change nothing else: not `engine/`, `packages/`,
`experiments/`, `spec/`, `conformance/`, `.github/`, another tool or another document, and not
`docs/decisions/` (DEC-172 stays not done). No promise changes, so `spec/CHANGES.md` gains no
line. Write outside the tree only in temporary directories the tests make. Report anything else
that needs changing.

## Done when

The first group prints otherwise today and passes after; the second, guards, passes today and
after. Run each new test by name, once, then the whole tool tests once, last.

```sh
export PYTHONDONTWRITEBYTECODE=1
u() { /usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -k "$1" -v 2>&1 | grep -c ' \.\.\. ok$'; }
for t in test_freshness_rebuilds_over_a_newer_build_in_a_shared_target_directory test_freshness_rebuilds_over_a_newer_build_in_the_target_cache test_freshness_sets_each_listed_file_once_before_the_first_build test_freshness_fails_closed_on_a_file_it_cannot_set; do u "$t"; done  # 1 four times; 0 today
grep -c 'file times set to one instant before the first build' tools/gate.py  # 1 or more; 0 today
for f in AGENTS.md docs/implementation/METHOD_AS_RUN.md; do grep -c 'order 402' "$f"; done  # 1 twice; 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
u() { /usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -k "$1" -v 2>&1 | grep -c ' \.\.\. ok$'; }
for t in test_target_cache_rebuilds_for_a_second_tree_with_an_explicit_host_target test_target_cache_engine_prepares_discovered_directories_once test_target_cache_invalidates_transitive_path_fingerprints_once; do u "$t"; done  # 1 three times
for t in check_docs check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, twice
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(tools/(gate\.py|tests/test_gate\.py)|AGENTS\.md|docs/implementation/METHOD_AS_RUN\.md)$' | wc -l | tr -d ' '  # 0
/usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1 | cut -d' ' -f1  # OK, the whole tool tests, once, last
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML. Your report gives, per decision, what
changed and where; each new test's line on the unchanged gate, then its passing line; each
existing test you changed, and why; your own choices; and every check's last line. Stage nothing.

## What follows

- `-Zchecksum-freshness`, once stable on the pinned toolchain, compares contents: a gate order
  can then retire this step and order 223's removal.
- The coordinator reads the next two queue records by numbers: `freshness.files`, the engine
  clippy row's seconds and the engine test's first `Finished`, expected as What it costs gives.
- A build another tree or a hand makes in a shared folder after the run's instant, or with the
  clock set back, can still be reused: in DIR's folders the lock and order 223's removal narrow
  this, elsewhere nothing does. The coordinator's kit, and order 401's queue after it, build in
  DIR's folders only through the gate.
- The compiler cache server's stall is not this order's: a gate order of its own makes the gate
  name it.
- DEC-172's status is the coordinator's once this order merges.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` with the decision log through
DEC-175. The first order built and read on the team seats (DEC-179, DEC-180).

**Worker-3's turn** (09:39 to 09:57), on the Business seat, built the freshness step and its
regressions; the stale-build regression fails on the unchanged gate and passes with the change.

**Two blind reads.** The Codex verifier, `root-verifier-3.turn.v1`, passed it with no finding; the
Claude reader, `root-reader-a.turn.v1` on the Team seat in its first read, had two shoulds (say what
to do when the step fails; name every gap) and three nits, one the coordinator's untracked order
text. One fix turn by worker-3 (10:09 to 10:19) applied them. The gate's run on the squash is the
coordinator's, at the merge.
