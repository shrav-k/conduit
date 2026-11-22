# Work order 281: the engine's tests build their packages in the gate's cache, changing order 223's R1

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: every crate it builds is in the local cache already. Run cargo offline, and if a build
asks for a crate that is not there, stop and report, and fetch nothing. Run one cargo command at a
time. Never print a whole large file, diff or log: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Do not run `conduit` outside a
test, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each
`python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user or machine name, or private repository name.

## What and why

**Today.** Eight files of the engine's shell tests start cargo builds of ten Rust directories
under `packages/` and `experiments/`, to pack or run them. In `engine/crates/shell/tests/`:
`agents.rs` (`build_archive`, lines 78-99; `scripted_client`, 639-650), `github.rs` (`cargo`,
27-48, for `archive` and `copy_stand_in`), `local_process.rs` (`archive`, 39-67),
`working_copy.rs` (`pack`, 20-49), `secrets.rs` (`pack`, 34-63), `serve.rs`
(`pack_scenario_s4_effect`, 736-781), `plan_apply_support/mod.rs` (`scripted_agents`, 148-196)
and `s2.rs` (`cargo` and `artifacts`, 63-149).

[Order 223](223-gate-speedups.md)'s R1 left these builds where they were: "It does not set
`CARGO_TARGET_DIR`, so a cargo run inside a test (`agents.rs` builds the Agent packages) keeps
building in that package's own `target/`, as today." [Order 230](230-gate-target-cache-fixes.md)
then gave a package's own test row its cache folder through `CARGO_TARGET_DIR`. The engine's test
row kept R1's words (the comment in `rust_command`, `tools/gate.py`: "Engine tests build several
packages and keep their separate targets."). `serve.rs` also passes `--target-dir
<package>/target` whatever the environment, as [order 233](233-s4-install-check-in-rust.md)'s
decision 2 asked. So every gate in a fresh worktree builds the ten directories from empty
`target/` folders.

**The cost**, from the coordinator's measurements of order 269's merge gate:

- the engine test row took 1765.1 s, and about 340-410 s of it went to these builds (an estimate
  from the log's times);
- the two Agent package builds compiled 141 crates each, in 51.8 s and 36.4 s, and the row's log
  holds 287 `Compiling` lines, 5 of them the engine's own;
- the ten folders held 3.2 GB in that worktree.

**The saving expected:** 250-300 s of the engine test row under `cargo test` (the coordinator's
estimate). Registry crates are reused, and only each directory's own path crates rebuild. Not
measured: how much of it the queue's compiler cache already serves when its server runs. The
gate also stops adding those folders to each worktree, though a worker's own runs still do. A
later nextest order needs this, since under nextest every test process repeats these builds.

**How the gate shows it.** This order's own merge gate runs the new `tools/gate.py` and fills the
folders. From the next engine gate on, three figures show it:

- the record's `engine test` row `seconds`, and its printed line `PASS engine test (N s)`, should
  fall by about 250-300 s from the earlier gates' 1185-1765 s at like load. Like load is the
  record's `load_average`, which order 280 adds (DEC-062); until it lands, the coordinator reads
  the load by hand;
- that row's log should count under 20 `Compiling` lines instead of 287, whatever the load. This
  counts only `agents.rs`'s two Agent builds, the only ones whose cargo output reaches the log;
- for the other eight directories: that gate, in a fresh worktree, adds none of the ten `target/`
  folders, and the cache gains `experiments-013-s2-shared-login-linear-stub`, absent today.

The coordinator reads them and records them.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): none
served and none at risk. No promise, case, engine source or package changes. Every test checks
what it checks today; only where its builds land under the gate changes. The gate still runs every
row it runs today, in full on the Mac (DEC-060).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: cargo's own persistent target directory, named by `CARGO_TARGET_DIR`,
which the gate already gives its rows (orders 223 and 230). This order extends the same folders,
and R1's staleness rule, to the builds the engine's tests start. It adds one environment variable,
read in one test helper, and no new tool. Not taken:

- one folder for every directory (`CARGO_TARGET_DIR=<cache>/engine-packages`, considered
  first). Same-named binaries overwrite each other in one folder: both Agent packages build
  `scripted-client`, and `agents.rs` gives each Agent its own (`scripted_client`, compared at
  line 474);
- the compiler cache alone, which still leaves the folders cold and the linking undone.

**When it runs.** On `main` once DEC-060 to DEC-062 are logged. Through Conduit's dogfood: one
Codex worker turn (aim for under an hour). Its tests are the gate's own tool tests and one shell
test run alone, never a whole engine suite. Then the two blind reads, at the same time, who run
only what settles a doubt, with one fix turn for both
([DEC-050](../decisions/log.md#dec-050-2026-11-22-both-reads-on-every-order)), and the gate
(DEC-030, [DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). This is one of
the gate orders DEC-060 lands, and one of the speed-ups DEC-062 measures before any machine is
bought. The variable's name, the folders shared with each package's own rows, and the helper's
refusal are the coordinator's choices, open to the owner's strike (HOW-WE-WORK rule 6). Other
gate orders of the same day also edit `tools/gate.py` and `test_gate.py`: whichever merges later
rebases and keeps both, and against order 280 keeps decision 2's preparation before any row
beside the engine test.

**Stop rule.** Stop, change nothing further, and report where:

- a build site cannot take the folder without a change under `packages/`, such as a package
  binary that finds its build by a fixed `target/` path. Today each `src/bin/package.rs` the
  tests run finds its binary through cargo's metadata or its own path, or packs none;
- the gate cannot prepare a folder that the engine's tests use;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **Order 223's R1 changes, and order 233's decision 2 with it.** The R1 sentence quoted above no
   longer holds for the engine's test row. In its place: under `--target-cache DIR`, the gate's
   `engine test` row runs with `CONDUIT_TEST_TARGET_CACHE=DIR`. Each Rust directory D that an engine
   test builds then builds with `CARGO_TARGET_DIR=DIR/<D, each / as ->`, R1's name for D's folder,
   the one D's own rows use. Order 233's "The nested builds use the package's own target
   directory, whatever the caller's environment" gains "unless `CONDUIT_TEST_TARGET_CACHE` names
   the gate's cache". The rest of R1 stands: the lock, the record, the refusals, the folder names,
   and the gate never cleaning DIR. Order 223's text stays as written; this order is the change.
2. **The gate** (`tools/gate.py`).
   - Only the `engine test` row gets the variable. Its printed and recorded command shows
     `CONDUIT_TEST_TARGET_CACHE='<target-cache>'`, as the package rows show `CARGO_TARGET_DIR`.
     `--plan` shows it too, and creates nothing. The engine's fmt and clippy rows and every
     package and experiment row stay as they are.
   - The variable joins `NAMED`. It is removed from every command, and named on the Environment
     line when the caller's environment has it. So no row, with or without the option, gets a
     value whose folders the gate did not prepare.
   - **R1's staleness rule, carried over.** Before the engine test row's `cargo test` starts, the
     gate removes the path crates' fingerprints from the folder of each discovered Rust directory
     that exists in DIR and that no earlier row of this run prepared. It does this as R1 prepares a
     directory before its first row: the host query, the `--filter-platform` metadata and
     `invalidate_path_fingerprints`. Registry crates are kept. A directory with no folder in DIR
     needs nothing. Every discovered directory, prepared or without a folder, then joins the
     run's prepared set, so no later row prepares it again. Under order 280's `--merge-queue`,
     this finishes before any row beside the engine test starts: an experiment has no clippy row,
     and its test row's preparation would otherwise remove fingerprints in a folder the engine
     test's builds use, outside cargo's lock.
   - A preparation that fails fails the engine test row closed, its why naming the directory and
     the stage, and `cargo test` does not run. The row's seconds include the preparation, as today.
   - The comment in `rust_command` says what the code now does.
3. **The tests' builds** (`engine/crates/shell/tests/`). One helper, `package_target_dir`, in a new
   module `package_target/mod.rs`, is the only code that reads `CONDUIT_TEST_TARGET_CACHE`, and
   that file the only `.rs` file that names it, comments included. When the variable is set and
   not empty, the helper takes a directory's path, names it relative to the repository root, and
   returns `DIR/<that name, each / as ->`. When it is unset or empty, it returns none.
   - Under the variable, every cargo command at the eight build sites runs with `CARGO_TARGET_DIR`
     set to the helper's folder. It must be the environment variable, so that a package's
     `src/bin/package.rs`, and the `cargo build` and `cargo metadata` it starts, see the same
     folder. That covers `agents.rs`'s `cargo metadata` in `scripted_client`. `s2.rs` builds its
     stub's path from the helper: under the variable, `release/s2-linear-stub` in the helper's
     folder, since the engine row's `--target-dir` leaves the test process no `CARGO_TARGET_DIR`;
     without it, lines 140-143 hold. `serve.rs` keeps its
     `env_remove` lines and drops `--target-dir <package>/target` only under the variable.
   - Under the variable, a directory that is not `packages/<name>`, `experiments/<name>` or
     `experiments/<name>/<name>` fails the test, naming the directory. These are a subset of
     `rust_manifests`'s patterns, and the gate prepares only the workspace roots among them; all
     ten directories are roots today.
   - Without the variable, every command is exactly today's. A person's run by hand builds where it
     builds today: in each package's own `target/`, or where that person's `CARGO_TARGET_DIR`
     points, except in `serve.rs`, which keeps `--target-dir <package>/target`.
4. **The tool tests**, in `tools/tests/test_gate.py`, with the fake runner, each name beginning
   `test_target_cache_engine_`. Write them first, run them on the unchanged gate, and report each
   failing line. They cover:
   - the variable on the engine test row alone, its value DIR and shown as `<target-cache>`, in the
     plan and the run, with DIR nowhere in the output, record or logs;
   - before the engine test call, the path fingerprints gone and the registry ones kept in two
     existing package folders, one of them holding a crate reached by path from another
     directory. A folder that the package's own row prepared earlier in the run is not prepared
     again, and nothing is created in DIR for a directory with no folder there;
   - a failing preparation: the engine test row FAILs naming the directory, there is no
     `cargo test` call, and the lock is released;
   - the variable planted in the caller's environment: removed from every command, with or
     without `--target-cache`, and named on the Environment line.
5. **METHOD_AS_RUN.md**, its gate paragraph. After "`--target-cache DIR` uses one target directory
   per Rust directory, rebuilds the tree's own crates and reuses registry crates." add: "The
   engine's tests build the packages they use in the same folders, which the gate names to them
   in `CONDUIT_TEST_TARGET_CACHE`; run by hand, without it, those builds stay in each package's own
   `target/`."

## What to read

In `tools/gate.py`: `NAMED`, `rust_manifests`, `invalidate_path_fingerprints`, and `rust_command`
with its preparation of a directory. In `tools/tests/test_gate.py`: the tests whose names begin
`test_target_cache`. Then the eight build sites above, and `packages/git/src/bin/package.rs` for
how a package binary finds its build.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 4f14b1db HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
grep -cE '^## DEC-06[02], ' docs/decisions/log.md
grep -c 'Engine tests build several packages and keep their separate targets' tools/gate.py
grep -c "^NAMED = frozenset(('CONDUIT_TEST_NPM'" tools/gate.py
git grep -l 'Command::new("cargo")' -- engine/crates/shell/tests | wc -l | tr -d ' '
grep -c 'arg(package.join("target"))' engine/crates/shell/tests/serve.rs
grep -c 'var_os("CARGO_TARGET_DIR")' engine/crates/shell/tests/s2.rs
git grep -l CONDUIT_TEST_TARGET_CACHE -- tools engine | wc -l | tr -d ' '
ls -d packages/git/target 2>/dev/null | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
(cd packages/git && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "git $?"
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py 2>&1 | tail -1
```

They print `main 0`, `0`, `2`, `1`, `1`, `8`, `1`, `1`, `0`, `0`, `engine 0`, `git 0`, and a line
beginning `OK`. `/usr/bin/python3` is the Mac's interpreter with PyYAML.

## Scope

Change `tools/gate.py`, `tools/tests/test_gate.py`, the eight build-site files named above,
`plan_apply_cases.rs` only to declare the new module, and `docs/implementation/METHOD_AS_RUN.md`
as decision 5 says. Create only `engine/crates/shell/tests/package_target/mod.rs`. Change nothing
under `spec/`, `conformance/`, `packages/` or `engine/crates/*/src`, and no other test, no
`Cargo.toml` or lockfile, and no other document. Write outside these only under `target/wo281/` and
`engine/target/`, and, through the shell runs, `packages/git/dist/` and (by hand)
`packages/git/target/`. Report anything else that needs changing.

## Done when

Each line of the first group marked "today" prints otherwise on BASE; every line passes after.
Run the shell runs in the order shown, from a tree where `packages/git/target` does not exist.
The second group are guards, which pass today and must still pass. Run no other engine test: the
gate runs the suite at the merge.

```sh
ok='^test result: ok\. 1 passed'
C="$PWD/target/wo281/cache"
grep -c CONDUIT_TEST_TARGET_CACHE tools/gate.py  # 1 or more; today 0
grep -c 'Engine tests build several packages and keep their separate targets' tools/gate.py  # 0; today 1
grep -cE 'def test_target_cache_engine_' tools/tests/test_gate.py  # 4 or more; today 0
grep -rl --include='*.rs' CONDUIT_TEST_TARGET_CACHE engine/crates | wc -l | tr -d ' '  # 1; today 0
grep -rl --include='*.rs' 'package_target_dir(' engine/crates/shell/tests | wc -l | tr -d ' '  # 9; today 0
sed -n '/shared-login\/linear-stub"/,/release\/s2-linear-stub/p' engine/crates/shell/tests/s2.rs | grep -c 'package_target_dir('  # 1 or more; today 0
grep -c CONDUIT_TEST_TARGET_CACHE docs/implementation/METHOD_AS_RUN.md  # 1; today 0
(unset CARGO_TARGET_DIR; cd engine && CONDUIT_TEST_TARGET_CACHE="$C" cargo test --offline --release -p conduit-shell --test working_copy 2>&1 | grep -c "$ok")  # 1, the test run alone under the gate's variable
test -x "$C/packages-git/release/conduit-git"; echo "cached $?"  # cached 0; today cached 1
ls -d packages/git/target 2>/dev/null | wc -l | tr -d ' '  # 0; today 1
```

```sh
(unset CARGO_TARGET_DIR CONDUIT_TEST_TARGET_CACHE; cd engine && cargo test --offline --release -p conduit-shell --test working_copy 2>&1 | grep -c "$ok")  # 1: run by hand
ls -d packages/git/target/release | wc -l | tr -d ' '  # 1: by hand, the package's own target/, as today
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
git add -N engine/crates/shell/tests/package_target/mod.rs  # so the checks see the new file
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1  # OK
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(tools/(gate\.py|tests/test_gate\.py)|engine/crates/shell/tests/((agents|github|local_process|working_copy|secrets|serve|s2|plan_apply_cases)\.rs|(plan_apply_support|package_target)/mod\.rs)|docs/implementation/METHOD_AS_RUN\.md)$' | wc -l | tr -d ' '  # 0
git reset -q -- engine/crates/shell/tests/package_target/mod.rs; rm -rf target/wo281  # stage nothing
```

Report, per decision, what changed and where; each new tool test's failing line on the unchanged
gate, then its passing line; the shell runs' last lines; your own choices under `own_choices`;
and every check's last line. Stage nothing. The two blind reads follow (DEC-050), then the
gate and the merge.

## What follows

Named here and not in this order:

- The coordinator measures the saving at the second engine gate after the merge, as "How the gate
  shows it" says, and records it.
- Found while writing: run by hand with `CARGO_TARGET_DIR` set, both Agent packages' builds land
  in that one folder. Their `scripted-client` binaries then overwrite each other, and `agents.rs`
  gives both Agents the second. This order leaves runs by hand as they are.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with the owner's DEC-060 to DEC-067 and order 280's first build stacked.

**Worker-2's turns** (10:41 to 10:50, and 11:10 to 11:13 after a right stop) made the package
builds that the engine's tests start use a persistent target folder in the gate's cache, named by
`CONDUIT_TEST_TARGET_CACHE` when the gate runs them, with order 223 R1's staleness rule carried
over (the package path crates' fingerprints removed before each gate) and each shared cache
prepared before any row uses it; a person running the tests by hand keeps each package's own
folder, and `serve.rs`'s folder is fixed. The stop: the names check rejected the new variable,
and the coordinator widened the scope to add it, alone, to `tools/set_names.py`'s build-only
allowlist, as order 243 did for `CONDUIT_CONFIG_DIR`. The five regressions failed first; 203 tool
tests pass.

**One read** (12:19 to 12:24), the Codex verifier's alone under the owner's DEC-071, passed it
with two shoulds and a nit. One short turn (12:24 to 12:28) applied them: the race regression now
fails on a scheduler that admits a companion row before the cache is prepared, the redaction test
covers both cases, and METHOD_AS_RUN states the manual run as it is. 55 gate tool tests and the
names check pass.

**At the merge**, the gate stopped at its first failure, as order 280 now has it: the older test
`test_target_cache_rebuilds_for_a_second_tree_with_an_explicit_host_target` builds a probe crate
with a 60-second bound, and beside the engine test it waited on cargo's locks past that. The
coordinator raised the bound to 600 seconds, still a guard against a hang.

The gate's run on the squash is the coordinator's, at the merge; the step times it records show
the saving.
