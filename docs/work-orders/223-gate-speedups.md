# Work order 223: the gate's speed-ups: a warm build cache, short waits in the gate's test, tool tests on their inputs, experiment 011 on its own changes, CI's macOS jobs nightly, and the 041a migration retired

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: `tools/gate.py` runs cargo offline and fetches nothing; if a build asks for a crate that
is not in the local cache, stop and report. Run one cargo command, and one gate, at a time. Never
print a whole large file, diff, log or list: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Do not run `conduit` against a store,
`codex`, `claude` or the dogfood harness, and sign in nowhere. Every file you write is public:
write no absolute local path, no user or machine name and no private repository name.

## What and why

**What.** On 2026-11-20 the owner approved, "Yes, all (Rec.)", the speed-ups of the coordinator's
testing audit (private). They are numbered here as in the owner's summary: R1 a warm build, R2
short waits in the gate's own test, R3 the tool tests on their inputs, R4 experiment 011 off
engine changes, R5 CI's macOS jobs nightly, and R7 a cleanup.

**Why.** The gate runs its rows one after another, so a second cut from a row is a second cut
from a merge. From 17 gate records: a docs-only gate takes about 114 s, 71-91 s of it the tool
tests, about 62 s of those `test_gate.py`, and about 27 s of that three fixed 10-second waits.
An engine gate takes 493-873 s and a spec gate 1383-2166 s cold, mostly compiling: order 216's
rerun of the same 50 rows in a warm target directory compiled 617 s less. Experiment 011 adds
31-77 s cold to each engine gate. 43 of CI's 45 failed runs since 11-16 failed only in macOS
jobs. Together, about 1 to 1.4 hours a day of serial gate time. What could slip, by the audit:
under R3, a Python update on the Mac that breaks a tool test shows at the next `tools/` change;
under R4, a break in the short form, which v0 does not need; under R5, a Mac-only break in a
package the branch did not change, found the next morning.

**Preservation tests** ([ruling 92](../proposals/ruling-92/README.md), point 1): none served and
none at risk. The order changes no promise, engine code, package or case, and cuts no check v0
needs.

**Should this exist** (point 3). Yes. Each piece is existing practice: a cargo target directory
shared between builds (`--target-dir`), checks filtered by the paths a change touches, and
scheduled CI runs. It adds one mechanism, the cache option and its lock, inside `tools/gate.py`,
the named Python exception to the rule that code is Rust; no shell or other script.

**When it runs.** Through Conduit, as A78's stage 2 sends its orders (the coordinator's route,
open to the owner's strike). Order 218 also edits `ENGINE_PREFIXES` and `test_gate.py`:
whichever merges second rebases and keeps both. The coordinator runs the gate, has one fresh read
of the diff (ruling 92), and merges.

## The decisions

**R1. A warm build: `--target-cache DIR`.**
- Each Rust directory the gate runs, and the engine, builds in `DIR/<its path, each / as ->`
  (`DIR/engine`, `DIR/packages-provider-file`). The gate adds `--target-dir` with that path to
  its `cargo clippy` and `cargo test` commands; the row's printed and recorded command shows it
  as `--target-dir <target-cache>/<name>`, as the secrets row shows `<temporary>`, and DIR
  itself appears only in the one printed line and in `settings.target_cache`. It does not set
  `CARGO_TARGET_DIR`, so a cargo run inside a test (`agents.rs` builds the Agent packages) keeps
  building in that package's own `target/`, as today. If two directories map to one name,
  `rust discovery` fails, naming both.
- DIR is made if missing. A DIR inside the gate's root, or one that is not a directory, is
  refused: the gate prints `Gate: NOT RUN:` with the reason and exits 2.
- What is stale. Cargo reuses a registry crate by its version, features, profile and toolchain.
  It judges the tree's own crates by file times, which a cache shared by worktrees cannot trust:
  a worktree made or edited before another gate's build looks older than that build, and would
  reuse binaries that hold the other worktree's paths. So before a directory's first clippy or
  test row, the gate removes from that directory's cache the fingerprints
  (`debug/.fingerprint/<name>-*` and `release/.fingerprint/<name>-*`) of every package whose
  source is a path: the directory's own members and those of each directory it depends on by
  path, transitively, as discovery's metadata names them. Those rebuild from this tree; registry
  crates are reused. Removing more only costs time; removing less is wrong.
- The lock. The merge queue runs one gate at a time, so two gates never share DIR at once. The
  gate makes that certain: at start it creates `DIR/gate.lock` exclusively, holding its process
  ID, head and start time, and removes it however it ends: PASS, FAIL, an interrupt or an error.
  If the file exists, the gate does not start. It prints `Gate: NOT RUN:`, names the lock, the
  process ID in it when there is one and whether that process runs, says to remove the file if
  no gate runs, exits 2, and removes nothing it did not create.
- `--plan` takes no lock and creates nothing; its rows show the `--target-dir` they would use.
- The record: `settings` gains `target_cache`, DIR as printed (redacted as the branch is), or
  null. One printed line names DIR and says the tree's own crates rebuild and registry crates
  are reused.
- Without the option, the gate runs the same commands with the same environment and prints the
  same lines as today; only the record's `target_cache: null` is new.
- The gate never cleans DIR; removing DIR makes the next gate cold, nothing else. Passing the
  option from the merge queue is the coordinator's, not this order's.

**R2. The waits.** `test_a_command_past_its_timeout_is_ended_with_its_children` in
`tools/tests/test_gate.py` runs its trap, escape and detach modes against `GRACE = 10` and
`PIPE_WAIT = 10` unshortened. Patch both to 1 with `with mock.patch.object(gate, 'GRACE', 1),
mock.patch.object(gate, 'PIPE_WAIT', 1):` around the trap, plain, escape and detach runs only, as
`test_the_real_readers_take_planted_values` patches `DERIVE_TIMEOUT`. The two interrupt parts
after them keep the real values, since the second one's `threading.Timer(3, ...)` must fire
while `run_command` still waits PIPE_WAIT on the pipe. The three modes' bounds, now
`assertLess(elapsed, 15)`, become 8 s or less, below the real 10, so that a runner that waits a
fixed 10 s instead of `GRACE` or `PIPE_WAIT` fails. No assertion is removed. The gate's own
values stay 10.

**R3. The tool tests on their inputs.** The `tool tests` row runs when the branch changes a path
under `tools/` or `.github/workflows/`, or a file named `*.rs`, `Cargo.toml` or `Cargo.lock`
under `packages/` or `experiments/`, and with `--all`. Those are what the tool tests read from
the real tree: `checks.yml`, and the tracked `.rs` and `Cargo.toml` files that
`test_reads_table_agrees_with_the_tree` reads; the other modules read temporary folders.
Otherwise the row is NOT RUN and not required, keeps its name, command and place (so its
comparison with CI's docs job holds), and its why says that the branch changes none of those
paths and that CI's docs job runs the tool tests on each pull request. The last line lists it
among the rows not run. CI's docs job is unchanged. If a tool test reads another path of the real
tree, add that path to the trigger and say so under `own_choices`. Of 186 commits since 11-14,
26 match: 86 % of gates skip the row.

**R4. Experiment 011 on its own changes.** Two routes choose it today: its path dependency on
`engine/crates/loader` and `core` (from cargo metadata, not a table) on any engine change, and
its two `RUST_READS` rows on `spec/` and `conformance/fixtures/s1/`. Add a table to
`tools/gate.py`, `OWN_PATHS_ONLY`, mapping a directory to its reason; its one entry,
`experiments/011-short-form-spike`: a finished experiment (A79's result is recorded), kept
because open order 049a1 runs its tests; CI's rust job still tests it on each pull request. A
directory in it is chosen only when the branch changes one of its own paths, or by `--all`. Where
a dependency or a read would have chosen it, the gate prints a NOT RUN row, not required, named
`rust experiments/011-short-form-spike`, whose why names that route and the reason; the
directory is not listed in `rust unreached`. `RUST_READS` keeps its two rows for 011, which say
what it reads, so `test_reads_table_agrees_with_the_tree` holds; that test also checks that each
key of the new table is a directory in the tree. 011 stays in the tree.

**R5. CI's macOS jobs nightly.** These jobs move from `.github/workflows/checks.yml` to
`.github/workflows/nightly.yml`, their steps and timeouts unchanged: `macos-package` (`macos
(<directory>)`, one per package under `packages/`, 12 today) and `macos-engine` (`macos (engine:
loader, core, conformance)`). nightly.yml lists the packages for the first itself, as
`rust-dirs` does today; `rust-dirs` keeps `macos_only` (`tools/gate.py` reads it) and loses its
`packages` output and the check on it. Their cache saves run on the nightly's own events, not on
`push`. The `rust` job's `needs` and its check line lose the two macOS jobs; `engine-doc` stays
as it is. Each file's header comment says where its jobs run. No check becomes required
([ruling 80](../proposals/ruling-80/README.md)). In `docs/implementation/METHOD_AS_RUN.md`, the
sentences on the macOS jobs say what this order changed. The report names the effect on A77's
claim 1, whose macOS leg leaves the checks workflow; `docs/implementation/WORK_ORDERS.md` stays as
written.

**R7. The 041a migration retires.** Delete `tools/migrate_041a.py` and
`tools/tests/test_migrate_041a.py`, the one-time migration of 11-11, with `git rm -q`. No gate
row, CI step or AGENTS.md line names them; `check_docs.py`'s check of path mentions in tools
loses the script's mention of `docs/work-orders/041a-declared.yaml`. That declaration stays:
order 077, which stays for order 144's reading, names it, and order 222's decision 3 keeps it;
it leaves when they do. Mentions in work-order text stay as written. The sentence order 222's
decision 3 adds to `docs/work-orders/README.md`, that the declaration stays "for the tool", is
the coordinator's to fix when 222's work merges.

**AGENTS.md.** The gate's paragraph gains a sentence on `--target-cache` (one target directory
per Rust directory, the tree's own crates rebuilt, the lock) and one on when the tool tests run.
METHOD_AS_RUN.md's gate paragraph says the same and when experiment 011 runs; CONTRIBUTING.md's
`gate.py` item says when the tool tests and 011 run, and its sentence on the `rust` check no
longer counts the macOS jobs among its jobs.

**Tests.** In `tools/tests/test_gate.py`, with the fake runner:
- names beginning `test_target_cache`: each clippy and test row's `--target-dir`, run with
  DIR's path and shown as `<target-cache>/<name>`, none on fmt;
  without the option, the same calls as today; a held lock refused with exit 2 and no command
  run; no lock left after a pass, a fail and an interrupt; the record's `target_cache`, redacted;
  a path package's fingerprint removed and a registry crate's kept; a DIR inside the root
  refused; two directories mapping to one name failing discovery;
- a name beginning `test_tool_tests`: the row run for a change of each trigger kind and for
  `--all`, and NOT RUN with its why for `docs/x.md`, `packages/p/README.md`, `engine/src/x.rs`
  and `spec/x.yaml`;
- a name holding `own_paths_only`: a listed directory chosen by its own change, NOT RUN with
  the route for a dependency and for a read, and absent from `rust unreached`.

## Not in this order

- The two process rules and the roadmap's v0 table, which the coordinator writes; the audit's
  narrowing of spec gates by test file; its R8, timing the workers' own runs; the coordinator's
  retired stopgap gate script, which is outside the tree.
- Any cut of a check v0 needs. The vectors, promise standing, the model account, experiment 012,
  the provider harness (experiment 022 and the provider packages) and the secret and private-path
  checks stay as they are.
- CI's `engine-doc` job, which the audit noted builds for 0 doctests: the owner did not approve
  dropping it, and it stays.

## Stop rule

Change nothing under `spec/`, `engine/`, `conformance/` or `packages/`. If a speed-up needs a
change there, stop, change nothing further, and report it.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor b49e8629 HEAD; echo "main $?"
git rev-parse HEAD                                   # BASE
git status --short | wc -l | tr -d ' '
grep -cE '^(GRACE|PIPE_WAIT) = 10$' tools/gate.py
grep -c "'tool tests'" tools/gate.py
grep -c 'runs-on: macos-latest' .github/workflows/checks.yml
grep -cE 'engine-doc|test --release --doc' .github/workflows/checks.yml
ls tools/migrate_041a.py tools/tests/test_migrate_041a.py | wc -l | tr -d ' '
grep -lw REPOSITORY tools/tests/*.py
(cd packages/provider-local-directory && cargo metadata --offline --locked --format-version 1 >/dev/null); echo "offline $?"
/usr/bin/time -p /usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py 2>&1 | grep -E '^(OK|real)'
/usr/bin/time -p /usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -k test_a_command_past_its_timeout 2>&1 | grep -E '^(OK|real)'
```

They print `main 0`, BASE, `0`, `2`, `2`, `2`, `5`, `2`, `tools/tests/test_gate.py`, `offline 0`,
then `OK` and the module's time before (about 60 s), then `OK` and the timeout test's time
before (about 40 s). `/usr/bin/python3` is the Mac's interpreter with PyYAML; a `python3`
without it skips or fails some tool tests.

## Scope

Change `tools/gate.py`, `tools/tests/test_gate.py`, `.github/workflows/checks.yml`,
`.github/workflows/nightly.yml`, AGENTS.md (the gate's paragraph),
`docs/implementation/METHOD_AS_RUN.md` (its gate paragraph at R3, R4 and `--target-cache`, and
the sentences R5 names) and CONTRIBUTING.md (its `gate.py` item at R3 and R4, and the sentence
on the `rust` check and the macOS jobs), and delete the two files of R7. Write
outside these only under `target/wo223/`, for the checks. Anything else goes in your report.

## Done when

Each line marked "fails today" prints otherwise on BASE; all pass after. The checks of R1, R3
and R4 run the new `tools/gate.py` against a second worktree of BASE.

```sh
/usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1     # OK
python3 tools/check_docs.py | tail -1                                   # Errors: 0
python3 tools/check_private_paths.py | tail -1                          # Errors: 0
git diff --check                                                        # prints nothing
git diff --name-only HEAD -- spec engine conformance packages | wc -l | tr -d ' '  # 0
grep -cE 'def test_(target_cache|tool_tests|.*own_paths_only)' tools/tests/test_gate.py  # 3 or more; fails today (0)
grep -c -- '--target-cache' AGENTS.md                                   # 1 or more; fails today (0)
# R2
grep -oE "patch\.object\(gate, '(GRACE|PIPE_WAIT)'" tools/tests/test_gate.py | wc -l | tr -d ' '  # 2 or more; fails today (0)
grep -cE '^(GRACE|PIPE_WAIT) = 10$' tools/gate.py                       # 2
/usr/bin/time -p /usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -k test_a_command_past_its_timeout 2>&1 | grep -E '^(OK|real)'  # OK; real at least 20 s below the start check's; fails today
# R5
grep -c 'runs-on: macos-latest' .github/workflows/checks.yml            # 0; fails today (2)
grep -c 'runs-on: macos-latest' .github/workflows/nightly.yml           # 2; fails today (0)
grep -cE 'macos-package|macos-engine' .github/workflows/checks.yml      # 0; fails today (5)
grep -cE 'engine-doc|test --release --doc' .github/workflows/checks.yml  # 5
grep -c 'macos_only=" experiments/005-macos-keychain "' .github/workflows/checks.yml  # 1
/usr/bin/python3 -c "import sys, yaml; [yaml.safe_load(open(f)) for f in sys.argv[1:]]; print('yaml ok')" .github/workflows/checks.yml .github/workflows/nightly.yml  # yaml ok
# R7
ls tools/migrate_041a.py tools/tests/test_migrate_041a.py 2>/dev/null | wc -l | tr -d ' '  # 0; fails today (2)
git grep -l migrate_041a -- . ':!docs/work-orders' | wc -l | tr -d ' '  # 0; fails today (2)
test -f docs/work-orders/041a-declared.yaml; echo "kept $?"            # kept 0
# A second worktree of BASE, run by the new gate
T=target/wo223/tree; C="$PWD/target/wo223/cache"
git worktree add -q --detach "$T" HEAD
G='import sys; sys.path.insert(0, "tools"); import gate; sys.exit(gate.main(sys.argv[1], sys.argv[2:]))'
g() { /usr/bin/python3 -c "$G" "$T" --base HEAD "$@"; }
# R3
echo 'wo223 probe' >> "$T/README.md"
g --plan | grep -c '^NOT RUN tool tests: '                              # 1; fails today (0)
echo '# wo223 probe' >> "$T/tools/check_roadmap.py"
g --plan | grep -c '^WOULD RUN tool tests: '                            # 1
git -C "$T" checkout -q -- .
# R4
echo 'wo223 probe' >> "$T/engine/README.md"
g --plan | grep -c '^WOULD RUN rust experiments/011-short-form-spike'   # 0; fails today (1)
g --plan | grep -c '^NOT RUN rust experiments/011-short-form-spike'     # 1; fails today (0)
echo 'wo223 probe' >> "$T/experiments/011-short-form-spike/README.md"
g --plan | grep -c '^WOULD RUN rust experiments/011-short-form-spike test: '  # 1
git -C "$T" checkout -q -- .
# R1
echo 'wo223 probe' >> "$T/packages/provider-local-directory/README.md"
g --target-cache "$C" | tail -1                                         # begins "Gate: PASS."; fails today
grep -c '^ *Compiling ' "$T"/target/gate/logs/*/*-provider-local-directory-test.log  # N1, cold: report it
g --target-cache "$C" | tail -1                                         # begins "Gate: PASS."
grep -c '^ *Compiling ' "$T"/target/gate/logs/*/*-provider-local-directory-test.log  # below N1: report it
grep -c '^ *Compiling provider-local-directory ' "$T"/target/gate/logs/*/*-provider-local-directory-test.log  # 1: the tree's own crate rebuilt
ls "$C"                                                                 # packages-provider-local-directory alone: no gate.lock
grep -c '"target_cache": "' "$T"/target/gate/*.json                     # 1
git -C "$T" checkout -q -- .
: > "$C/gate.lock"; g --target-cache "$C" | tail -1; rm -f "$C/gate.lock"  # begins "Gate: NOT RUN:" and names the lock
g --plan --target-cache "$PWD/$T/target/cache" | tail -1                # begins "Gate: NOT RUN:": inside the gate's root
git worktree remove --force "$T"; rm -rf target/wo223
python3 tools/gate.py | tail -1                                         # begins "Gate: PASS."
```

`git rm` stages the deletions; stage nothing else. Report, per decision, what you built and the
tests that show it; the start check's and the final times of `test_gate.py` and of the timeout
test; N1 and N2 and the
Rust rows' seconds in the two R1 runs; your own choices under `own_choices`; and every check's
last line.

## Record

Run on 2026-11-20 through Conduit itself, by the dogfood's own Agents, in Pacific time.
Worker-2's first turn (14:01 to 14:20) built R1 to R5 and R7; the gate passed.
`turn-verifier-2.turn.v1` (14:20 to 14:26) found four musts:
- the warm cache could miss a path crate reached transitively from outside the workspace;
- the lock was visible before its cleanup was registered;
- SIGTERM could end a cached gate without releasing the lock;
- the order's own cache probe, under `target/`, tripped `tools/set_names.py`.

For the fourth, root widened the scope to `tools/set_names.py`, so that its test of the `target`
component is relative to the repository root. The second turn (14:27 to 14:41) fixed all four,
each with a regression that failed first. Both prescribed cache runs then passed, as did 135 tool
tests, and the timeout test fell from 39 s to 12 s.

`turn-reader-1.turn.v1` (14:42 to 14:47), the Claude reader, accepted it with two shoulds:
- the `rust` job's comment no longer names the macOS jobs;
- its second, unstaged deletions, was only the state of the reader's copy, since the merged
  branch commits both.

The gate's run on the squash is the coordinator's, at the merge. It runs under the old rows,
since this change takes effect on the next merge.
