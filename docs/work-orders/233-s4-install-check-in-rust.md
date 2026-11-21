# Work order 233: G-8, scenario S4's installation check as an engine test, and its shell script removed

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate, and the only sockets it opens are the loopback
ports that the engine's own tests bind. Every crate it builds is in the local cache already; if a
build asks for one that is not, stop and report, and fetch nothing. Run one cargo command at a
time. Never print a whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n`
or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential. Do not run `conduit`
against a store outside a test, `codex`, `claude` or the dogfood harness, and sign in nowhere.

## What and why

**The step.** The dogfood-gaps stream's step G-8, the port the owner chose in
[DEC-025](../decisions/log.md#dec-025-2026-11-20-the-engine-is-rust-no-private-code): leftovers
-> "Port scenario-s4’s install.sh to Rust". [HOW-WE-WORK.md](../../HOW-WE-WORK.md), rule 2: code
is Rust in the tree, tested against the real binary. The exceptions are the Python checkers in
`tools/` and the Workflow tool's JavaScript. `packages/scenario-s4-effect/tests/install.sh` is the
one file of other code in the tree (the start check counts it).

**What the script does.** It is the installation check of
[order 143](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/143-s4-srv1-effect.md)
(decision 9), run from the repository root:

- it builds the engine (`cargo build --release` in `engine/`);
- it reads the archive's digest from `dist/scenario-s4-effect.tar.gz.sha256`, which an earlier
  `cargo run --release --bin package` made by hand must have written;
- it copies `tests/fixtures/project/*.yaml` to a temporary directory and replaces
  `sha256:PLACEHOLDER` in the copy with that digest;
- it stops if loopback port 38143 already answers, then runs `conduit init` with that port;
- it starts `conduit serve` and waits up to 30 seconds for the port while serve lives;
- it runs `conduit install` of the archive and `conduit submit` of the project, each of which
  must exit 0, then kills serve.

It prints "submit registered templates only" but checks nothing beyond the exit codes. No gate or
CI job runs it. It ran once, in order 143, on 2026-11-12.

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests);
[ruling 92](../proposals/ruling-92/README.md), point 1): 6, nothing starts model work or repeats
an effect implicitly. The test asserts what the script only printed: registering the templates
of an effect server dispatches nothing to it. No other test is at risk: no promise, case or
engine behavior changes.

**Should this exist** (point 3). The check, yes. It is the only place where the engine admits
this package's archive and fixture. The package's own tests check its declarations against
`spec/schemas/`, but not against the engine's loader and core. The existing paradigm is already
in the engine's suite, which packs packages and installs them through the real `conduit` binary:

- `engine/crates/shell/tests/agents.rs` packs `agent-claude-code` and `agent-codex`: first
  `cargo build --bin`, then `cargo run --bin package`. S4's `package` needs the same two steps,
  since it never runs Cargo itself;
- `engine/crates/shell/tests/secrets.rs` packs `provider-file`, offline;
- the gate runs the engine suite when such a package changes (`ENGINE_PREFIXES`;
  [order 218](218-s2-1-secret-read-file-store.md) added `provider-file` to it).

The script, as a second route, no. These routes are not taken (the coordinator's, open to the
owner's strike):

- **A binary in the package.** It would still run by hand, which is how this check went unrun.
- **A test in the package.** It would need the engine's binary. The package can name that only
  through a path dependency on the engine, and it depends on nothing in this repository (order
  143, decision 1), or through a nested engine build.
- **A Cargo build step** (`build.rs`). A build must not start a server or bind a port.
- **Deleting the script with nothing in its place.** The owner chose a port.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for
under 2 hours), then a Codex verifier and a Claude reader, who run only what settles a doubt,
then the gate ([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). By
[ruling 89](../proposals/ruling-89/README.md), each decision says what must hold; how is yours.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report in these
cases:

- a crate is missing from the cache: an `offline` line of the start check prints otherwise, or a
  build asks to fetch;
- the engine refuses the archive or the fixture: report its diagnostic. This order changes
  neither the engine nor the package's code or fixture;
- `submit` sets up, opens or dispatches anything. That is a finding against the fixture's claim;
- a correct result needs a change outside the scope.

A choice that nothing here governs is yours; list it under `own_choices`.

## The decisions

1. **The test.** `scenario_s4_effect_installs_and_registers_templates_only` goes in
   `engine/crates/shell/tests/serve.rs`, using that file's `cli`, `start_server`,
   `terminate_server` and `state` helpers, as `author_commands_require_serve_and_make_recorded_requests`
   does. It is a plain `#[test]`: it runs by default, is not `#[ignore]`, and reads no
   environment variable to decide whether to run. It needs no network beyond loopback.
2. **The archive.** Build it in `packages/scenario-s4-effect`, with that package's own toolchain
   and lock, offline and locked. Build the server binary first (`--bin scenario-s4-effect`), then
   run `--bin package`, as `agents.rs` does, with `--offline` as `secrets.rs` has it.
   - The nested builds use the package's own target directory, whatever the caller's
     environment, as the comment in `rust_command` in `tools/gate.py` says engine tests do.
   - The digest used is the sidecar's. The test requires that it equal the digest `package`
     prints and `conduit_core::packages::bytes_digest` of the archive's bytes.
   - A failed build fails the test, showing the last lines of cargo's stderr. It never skips.
3. **The project.** Copy the fixture's two files into the test's temporary directory and replace
   `sha256:PLACEHOLDER` with the digest. The test requires exactly one occurrence. The fixture in
   the tree does not change.
4. **The store and serve.** Run `init` with `--url http://127.0.0.1:PORT/mcp`. PORT is a free
   loopback port, found by binding port 0 as the file's other tests do, in place of 38143 and the
   probe for a port that already answers. Start serve with `start_server`, whose wait ends if
   serve exits.
5. **What must hold.**
   - `install` with `ARCHIVE#DIGEST` exits 0, and `submit` of the project exits 0.
   - The store then holds the three Resource templates `effect-plain`, `effect-keyed` and
     `effect-ledger`, each with its Type
     (`example.org/conduit/scenario-s4-effect#effect-plain.v1` and so on), and no Resource
     instance of the package's three Types.
   - No `invocation-dispatched` record names an operation of `example.org/conduit/scenario-s4-effect`:
     registration dispatched nothing to the package's items. Such records do name core's
     `install.v1` and `register.v1`, as `author_commands_require_serve_and_make_recorded_requests`
     asserts, so the record read is not empty.
   - Serve stops on SIGTERM with exit 0 (`terminate_server`).

   How the test reads templates and instances, from `state.json` or from `inspect`, is yours.
6. **The script removed.** Delete `packages/scenario-s4-effect/tests/install.sh`. In the
   package's README:
   - The paragraph beginning "The installation fixture pins" gains one sentence. It names the
     engine test and its file, and says that the test builds and packs this package, installs
     the archive into a served store and submits the fixture.
   - The counts table follows its own rule. Its row "Qualification tests and installation
     script | 4 | 959" becomes `| Qualification tests | 3 | 906 |`, since the script counted 53.
     Recount the README row and the Total, which holds 17 files.
7. **The gate.** The engine's tests now read this package. Add `'packages/scenario-s4-effect/'`
   to `ENGINE_PREFIXES` and name it in `ENGINE_WHY` in `tools/gate.py`. Add the case
   `('packages/scenario-s4-effect/x', set(), True)` to `test_reads_and_the_engine_rule` in
   `tools/tests/test_gate.py`, as order 218 did for `provider-file`.

## What this leaves

Report this list, corrected by what you built:

- a standing check that fails on any code outside Rust, the Python in `tools/` and the Workflow
  tool's JavaScript (DEC-025's "then check for non-Rust code"). This order's Done-when checks
  once;
- DEC-025's status line and its ROADMAP row, both the coordinator's;
- S4-H's qualification of dispatch through Conduit, unchanged;
- order 144's installation check (its Done-when, a `bash` invocation) takes this test's form in
  its finishing order, the coordinator's;
- CI's engine shards hold no package crates, so this test, like `secrets.rs`'s offline pack,
  fails there until the job restores or fetches them; CI is non-blocking (ruling 80), the
  coordinator's.

## What to read

- `packages/scenario-s4-effect/tests/install.sh`, `src/bin/package.rs`, `tests/fixtures/project/`
  and the README's "Build and qualification" and "D15 and preparation counts";
- in `engine/crates/shell/tests/`: `agents.rs` (`pack`), `secrets.rs` (`pack`), and `serve.rs`
  (`start_server`, `terminate_server`, `author_commands_require_serve_and_make_recorded_requests`);
- `tools/gate.py` near `ENGINE_PREFIXES` and the `rust_command` comment on the packages' targets.

## Before you change anything

From the repository root. If any line prints otherwise, stop, change nothing and report:

```sh
git merge-base --is-ancestor dd281795 HEAD; echo "main $?"
test -f packages/scenario-s4-effect/tests/install.sh; echo "script $?"
grep -c 'scenario_s4_effect_installs_and_registers_templates_only' engine/crates/shell/tests/serve.rs packages/scenario-s4-effect/README.md
grep -c 'sha256:PLACEHOLDER' packages/scenario-s4-effect/tests/fixtures/project/conduit.yaml
grep -c '"--bin", "scripted-client"' engine/crates/shell/tests/agents.rs
grep -c "'packages/provider-file/'" tools/gate.py
grep -c "('packages/provider-file/x', set(), True)," tools/tests/test_gate.py
grep -c '^| Qualification tests and installation script | 4 | 959 |$' packages/scenario-s4-effect/README.md
find . \( -name .git -o -name target -o -name dist -o -name node_modules \) -prune -o -type f \( -name '*.sh' -o -name '*.bash' -o -name '*.zsh' -o -name '*.py' -o -name '*.js' -o -name '*.mjs' -o -name '*.cjs' -o -name '*.ts' \) -print | grep -v '^\./tools/' | wc -l | tr -d ' '
(cd packages/scenario-s4-effect && cargo metadata --offline --locked --format-version 1 --filter-platform "$(rustc -vV | sed -n 's/^host: //p')" >/dev/null); echo "offline package $?"
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$(rustc -vV | sed -n 's/^host: //p')" >/dev/null); echo "offline engine $?"
git status --short | wc -l | tr -d ' '
```

They print:

- `main 0` and `script 0`;
- two lines ending `:0`;
- `1`, `1`, `1`, `1`, `1` and `1`;
- `offline package 0`, `offline engine 0` and `0`.

An `offline` line printing otherwise means a crate is missing from the cache: stop.

## Scope

Change these:

- `engine/crates/shell/tests/serve.rs`, the new test and any helper it needs;
- `packages/scenario-s4-effect/README.md`, as decision 6 says;
- `packages/scenario-s4-effect/tests/install.sh`, deleted;
- `tools/gate.py`, `ENGINE_PREFIXES` and `ENGINE_WHY` alone;
- `tools/tests/test_gate.py`, that one case alone;
- `engine/STANDING.md`, and only by `promise_standing.py --write`, if its check asks.

Change nothing else. That includes the engine's sources, `engine/Cargo.lock`, any `Cargo.toml`
or `Cargo.lock`, the package's `src/`, its other tests, fixture, `conduit.yaml` and
`contracts.yaml`, and everything under `spec/`, `conformance/`, `docs/`, `experiments/` and
`.github/`, as well as `IN_FORCE.md`, `ROADMAP.md` and `HOW-WE-WORK.md`. Put anything else that
needs changing in your report.

## Done when

Each line marked "fails today" prints otherwise on `main` at `dd281795`; all pass after.

```sh
(cd engine && cargo test --release -p conduit-shell --test serve -- --exact scenario_s4_effect_installs_and_registers_templates_only 2>&1 | grep '^test result:')  # test result: ok. 1 passed; 0 failed; 0 ignored; ...; fails today (ok. 0 passed)
test -e packages/scenario-s4-effect/tests/install.sh; echo "script $?"  # script 1; fails today (script 0)
find . \( -name .git -o -name target -o -name dist -o -name node_modules \) -prune -o -type f \( -name '*.sh' -o -name '*.bash' -o -name '*.zsh' -o -name '*.py' -o -name '*.js' -o -name '*.mjs' -o -name '*.cjs' -o -name '*.ts' \) -print | grep -v '^\./tools/' | wc -l | tr -d ' '  # 0; fails today (1)
(cd tools && PYTHONDONTWRITEBYTECODE=1 python3 -c "import gate; p='packages/scenario-s4-effect/'; print(p in gate.ENGINE_PREFIXES, p in gate.ENGINE_WHY)")  # True True; fails today (False False)
grep -c "('packages/scenario-s4-effect/x', set(), True)" tools/tests/test_gate.py  # 1; fails today (0)
grep -c 'scenario_s4_effect_installs_and_registers_templates_only' packages/scenario-s4-effect/README.md  # 1; fails today (0)
grep -c -i 'installation script' packages/scenario-s4-effect/README.md  # 0; fails today (1)
grep -c '^| Qualification tests | 3 | 906 |$' packages/scenario-s4-effect/README.md  # 1; fails today (0)
grep -c '^| Total | 17 | ' packages/scenario-s4-effect/README.md  # 1; fails today (0)
```

Once the first line passes, show that the test can fail. Make one temporary edit, so that the
copy's pinned digest is wrong by one hex digit. Run the first line again with
`grep -E '^test result:|panicked at'` in place of its grep, report both lines, then revert the
edit.

If you made a new file, run `git add -N` on it before the next block and `git reset -q` on it
after, so that the checks see it; stage nothing. These pass, as on `main`. Run the whole suite
once, last:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd packages/scenario-s4-effect && cargo test --release)  # passes; the README is in the archive
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s tools/tests  # OK
python3 tools/check_docs.py  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
python3 tools/check_decisions.py HEAD WORKTREE  # Errors: 0
python3 tools/promise_standing.py --check  # Errors: 0
git diff --check  # prints nothing
git diff --name-only HEAD | LC_ALL=C sort | tr '\n' ' '  # engine/crates/shell/tests/serve.rs packages/scenario-s4-effect/README.md packages/scenario-s4-effect/tests/install.sh tools/gate.py tools/tests/test_gate.py (and engine/STANDING.md only by the tool)
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite
```

Your report gives:

- each Done-when line's output on `main` and after;
- the deliberate failure's line and assertion;
- the new test's time on a cold package target and on a warm one, which the gate's cost needs;
- how the test tells templates from instances and finds the dispatch records;
- every choice you made yourself, under `own_choices`;
- the corrected list of what this leaves;
- every check's last line.

Stage nothing. The coordinator runs `tools/gate.py`, has the diff read, and merges.

## Record

Not yet run.
