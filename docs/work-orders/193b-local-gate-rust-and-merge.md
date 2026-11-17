# Work order 193b: the local gate's Rust checks, and merges that wait on the gate

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
Run no `cargo build` and no `cargo test`: the only cargo commands are the `cargo locate-project`
and `cargo metadata --no-deps` that the gate's discovery runs, offline, which build and fetch
nothing. Run the gate one run at a time. Install nothing, gitleaks included. Never print a whole
large file or log. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential.

## What and why

[Ruling 80](../proposals/ruling-80/README.md), point 10, has a local gate run the checks on the
coordinator's machine before each merge, and the merge wait for it, while GitHub's CI keeps
running on every pull request as a free second opinion and the Linux leg that no merge waits for.
As the option the owner chose said, the merge script and the method as run switch to the gate.
The coordinator split order 193 in two. [Order 193a](193a-local-gate.md) built `tools/gate.py`:
the range and the tree, the commit identity, the Python checks of CI's `docs` job with the
private names from a file, the pinned gitleaks, the lines, the record and `--plan`. It runs no
Rust directory and not the engine, and says so in a gate `rust and engine`, NOT RUN.

This part adds them, and switches the documents to the gate. Today the method as run has the
admin run AGENTS.md's checks and, for a Rust directory, its tests, fmt and clippy by hand, then
"wait for CI" ([section 7](../implementation/METHOD_AS_RUN.md#7-merge-and-the-record), lines 271
to 286), which ruling 80 ended. Which Rust directories a branch reaches is left to the admin's
eye. Since order 187, CI's macOS jobs run the packages' clippy and release tests and the engine's
loader, core and conformance tests, but no merge waits for them now, and section 5 still has the
experiments, the engine's shell and the engine's clippy run on the Mac by hand (lines 220 to 223).
`CONTRIBUTING.md`, the handover and the tracker still speak of the gate as to come, with order
193. [D13](../proposals/full-review/decisions.md) has a merge that changes how orders are checked
update the method as run in the same pull request.

What it leaves to others:

- The coordinator's private merge script switches to the gate outside the tree, where
  [D62](../proposals/full-review/decisions.md) keeps the runners.
- The nightly job's npm tests need the network, order 188's differential vectors take hours, and
  [order 189](189-package-tests-repeated-linux.md)'s repeated runs are for Linux: all stay CI's.
  The gate runs each check once, on the Mac.
- Whether a check becomes required on `main`: ruling 80 makes none required for now.
  IN_FORCE.md, topic 15, already states the local gate (ruling 80); this order changes no line of
  it, nor of ruling 80's record or the ledger.
- Where the private names are held stays open (ruling 80, its facts); the documents name the file
  the gate reads only by the method's variable.
- Making the gate a CI job, or CI a required check: the owner's "formalize them into GitHub
  processes later".

Read first: `tools/gate.py` and `tools/tests/test_gate.py` as order 193a left them;
`.github/workflows/checks.yml`, the jobs `rust-dirs`, `rust-dir`, `engine-build`, `engine-tests`,
`engine-doc` and `changed-packages`; `.github/workflows/nightly.yml`;
`engine/.config/nextest.toml`; the variables and sections 2, 5 and 7 of the method as run; and
order 189's decision 1, which selects package crates for CI's repeated runs much as decision 2
selects directories here.

## The decisions it applies

The gate is the owner's (ruling 80, point 10, answer 13). Its design below is the coordinator's
(rulings 37, 38 and 46, point 4), logged here and open to the owner's strike. Line numbers are
those of `main` at `d4c8cf8b`. If a line has moved, find it by its words. Each block below is
indented to sit in its list item: write its lines without that indent. Order 193a's decisions
stand except where this order changes them.

1. **Three options.** The usage becomes
   `python3 tools/gate.py [--base REF] [--plan] [--private-names-file FILE] [--all] [--jobs N] [--test-threads N]`:
   - `--all`: every Rust directory and the engine run, whatever the branch changes.
   - `--jobs N`, by default 6, and `--test-threads N`, by default 4: cargo's build jobs and the
     tests' threads, the method's `JOBS` and `TT` on the Mac. Each is a positive integer;
     anything else is argparse's usage error, exit 2.

   The gate finds `cargo` with `shutil.which`, which a test patches, as it finds `gitleaks`.
   What order 193a's decision 1 lets it run gains cargo: the gate now runs read-only Git
   commands, the checkers, cargo and gitleaks, and nothing else.
2. **Which Rust directories run.**
   - **Discovery**, as the `rust-dirs` job finds them (lines 184 to 200): each manifest matching
     `packages/*/Cargo.toml`, `packages/*/*/Cargo.toml`, `experiments/*/Cargo.toml` or
     `experiments/*/*/Cargo.toml` in the working tree, skipping a path with a `target`
     directory in it, and kept only when
     `cargo locate-project --workspace --message-format plain --manifest-path <manifest>`, run
     from the root, prints that manifest's own absolute path, the two compared as
     `Path.resolve()` gives them. Its directory is linted when it is under `packages/`, and built
     unless the line `macos_only="..."` of checks.yml (line 185) names it; a checks.yml without
     that line fails discovery, and the failure names the file. `engine/` is not discovered: it
     has its own rule below. The job also fails when it finds no crate under `packages/` (lines
     205 to 208), which guards the matrix of the macOS jobs; the gate has no such matrix and does
     not.
   - **Path dependencies**: for each discovered directory and for `engine/`,
     `cargo metadata --no-deps --format-version 1 --offline --manifest-path <dir>/Cargo.toml`,
     run from the root. Every dependency of every package it lists, normal, dev and build alike,
     that has a `path` is a path dependency. Strip the root, resolved as `Path.resolve()` gives
     it, from that absolute path; a path outside the root fails discovery, and the failure names
     it. A path inside the same directory adds nothing. Any other belongs to the discovered
     directory, or to `engine`, that holds it. On `main` today,
     `experiments/011-short-form-spike` depends on `engine`; `packages/provider-local-process`
     and `packages/provider-ssh-runtime` on `packages/runtime-provider-kit`; and
     `packages/scenario-s2-linear` and `packages/scenario-s2-linear-adapter` on
     `experiments/013-s2-shared-login/linear-stub`.
   - These commands make one required gate, `rust discovery`, which runs first, before
     `identity`. A command that fails, or a fault named above, fails it. With no `cargo` on the
     `PATH` it is NOT RUN with `cargo was not found on the PATH; install Rust through rustup`,
     still required. Either way no Rust or engine gate can be chosen: the gate records order
     193a's gate `rust and engine`, NOT RUN and not required, now with the why
     `rust discovery did not pass, so no Rust directory or engine gate was chosen`, and the
     result is FAIL. When discovery passes, `rust and engine` is not recorded.
   - **Selection.** A discovered directory runs when, the first that applies:
     1. the branch changes a path inside it, with the reason
        `the branch changes <N> of its paths`;
     2. it depends by path on a directory of item 1, or on `engine` when the branch changes a
        path under `engine/`, with the reason
        `it depends by path on <dir>, which the branch changes`, or, through other directories,
        `it depends by path, through <dir>, on <dir>, which the branch changes`;
     3. the table below says it reads a path the branch changes, with the reason
        `it reads <path> (<file>), which the branch changes`;
     4. `--all` was given, with the reason `--all was given`.

     Paths compare as plain prefixes ending in `/`, or whole for an entry that names a file;
     never as patterns.
   - **The engine** runs when the branch changes a path under `engine/`, `spec/` or
     `conformance/`, or under `packages/agent-claude-code/` or `packages/agent-codex/`, which the
     shell's tests build (`pack` and `scripted_client` in
     `engine/crates/shell/tests/agents.rs`), with the reason
     `the branch changes <N> paths under <the prefixes that matched, joined by commas>`.
     It also runs when it depends by path on a directory of item 1 (none today), and with
     `--all`. Its running for `spec/`, `conformance/` or an Agent package alone does not run a
     directory that depends on it.
   - **What each directory reads**, a module constant: each entry a directory, a path from the
     root that it reads, the file in the directory that shows it, and the text in that file that
     shows it. Write it with exactly these entries:

     | Directory | Reads | Shown in | By the text |
     | --- | --- | --- | --- |
     | `packages/provider-file` | `spec/` | `tests/schema_drift.rs` | `"../../spec` |
     | `packages/provider-local-directory` | `spec/` | `tests/schema_drift.rs` | `"../../spec` |
     | `packages/provider-local-machine` | `spec/` | `tests/schema_drift.rs` | `"../../spec` |
     | `packages/provider-local-process` | `spec/` | `tests/package.rs` | `"../../spec` |
     | `packages/provider-local-process` | `tools/check_schemas.py` | `tests/package.rs` | `import check_schemas` |
     | `packages/provider-ssh-runtime` | `spec/` | `tests/package.rs` | `"../../spec` |
     | `packages/runtime-provider-kit` | `spec/` | `tests/schema_drift.rs` | `"../../spec` |
     | `packages/scenario-s2-linear` | `spec/` | `tests/schema_drift.rs` | `"../../spec` |
     | `packages/scenario-s2-linear` | `packages/scenario-s2-linear-restrictions/contracts.yaml` | `tests/schema_drift.rs` | `"../scenario-s2-linear-restrictions/contracts.yaml` |
     | `packages/scenario-s2-linear-adapter` | `spec/` | `tests/package.rs` | `"../../spec` |
     | `packages/scenario-s2-linear-adapter` | `packages/scenario-s2-linear-restrictions/contracts.yaml` | `tests/package.rs` | `"../scenario-s2-linear-restrictions/contracts.yaml` |
     | `packages/scenario-s2-linear-restrictions` | `spec/` | `tests/schema_drift.rs` | `"../../spec` |
     | `packages/scenario-s4-effect` | `spec/` | `tests/declarations.rs` | `"../../spec` |
     | `experiments/011-short-form-spike` | `spec/` | `src/evidence.rs` | `.join("spec")` |
     | `experiments/011-short-form-spike` | `conformance/fixtures/s1/` | `src/evidence.rs` | `.join("conformance/fixtures/s1")` |
     | `experiments/022-provider-contracts` | `spec/` | `tests/harness.rs` | `.join("spec/` |

     CI's `rust-dir` runs every directory on every pull request, so it runs these tests after a
     change under `spec/`, or to the restrictions' `contracts.yaml`, alone; the gate must too.
     The test of decision 5, item 5, keeps the table true.
3. **The Rust gates**, after `secrets`, for the chosen directories in byte order of path, then
   `rust unreached`, then the engine's. Each runs inside its directory, so that its
   `rust-toolchain.toml` applies, and runs whatever the one before it gave. Each that runs is
   required.
   - A linted directory, as `rust-dir`'s steps (lines 247 to 255): `rust <dir> fmt`,
     `cargo fmt --check`; `rust <dir> clippy`, `cargo clippy --all-targets -- -D warnings`; and
     `rust <dir> test`, `cargo test --release`.
   - An experiment: `rust <dir> test` alone, as `rust-dir` checks no experiment's formatting or
     clippy.
   - A directory that is not built: one gate `rust <dir>`, NOT RUN and not required,
     `listed in macos_only of checks.yml, which CI does not build; the gate does not run it either`.
     Today that is `experiments/005-macos-keychain`, which probes the Mac's keychains, a store:
     the Mac could build it, but the gate runs no store. A linted directory listed there also
     gets `rust <dir> fmt`, `cargo fmt --check`, as `rust-dir` runs it for one (line 248); none is
     listed today.
   - The engine: `engine fmt`, `cargo fmt --check`; `engine clippy`,
     `cargo clippy --all-targets -- -D warnings`; and `engine test`, `cargo test --release`,
     which runs every test of CI's four shards and the doctests that `engine doctests` runs
     apart. It runs them as AGENTS.md does, not under nextest, so
     `engine/.config/nextest.toml`, which runs the shell's timing-sensitive tests one at a time,
     does not apply: on a busy Mac one of them can fail where CI's shards pass. The gate does not
     retry; the admin reads the log, and a Record that cites a second run says so.
   - Every command of the gate now runs with these set, besides order 193a's removals:
     `CARGO_NET_OFFLINE=true`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_TERM_COLOR=never` and
     `CARGO_BUILD_JOBS=<jobs>`, and for each `cargo test` `RUST_TEST_THREADS=<test threads>`,
     recorded in its command, as in `RUST_TEST_THREADS=4 cargo test --release`. So cargo takes
     crates from its cache alone and rustup installs no toolchain: one missing on the Mac fails
     the gate with cargo's or rustup's own message, and its fix, `cargo fetch` or
     `rustup toolchain install` in the directory, is the caller's.
   - The discovered directories that do not run make one gate, `rust unreached`, NOT RUN and not
     required:
     `the branch changes nothing in these, in a directory they depend on by path, or in a path they read: <dirs, joined by commas>`.
     It is left out when every directory runs. An engine that does not run makes one gate,
     `engine`, NOT RUN and not required:
     `the branch changes nothing under engine/, spec/, conformance/, packages/agent-claude-code/ or packages/agent-codex/`.
   - Timeouts, as the jobs have them: 5 minutes for each command of discovery (`rust-dirs`) and
     90 for each cargo command (`rust-dir` and the engine's jobs), ended as order 193a's
     decision 8 ends a command, with its timeout words.
4. **The frame of order 193a, changed so.**
   - The gates run one at a time: `rust discovery`, `identity`, the Python gates, `secrets`, the
     Rust gates and `rust unreached`, then the engine's gates or `engine`.
   - The header's Environment line becomes
     `Environment: cargo runs with CARGO_NET_OFFLINE=true, RUSTUP_AUTO_INSTALL=0, CARGO_TERM_COLOR=never and CARGO_BUILD_JOBS=<J>, and cargo test with RUST_TEST_THREADS=<T>. Removed from every command: <names>, and <K> variables named for a credential.`
   - `rust discovery`'s command reads
     `cargo locate-project and cargo metadata --no-deps --offline, for <N> manifests`, and its
     reason `the Rust gates are chosen from it`. Its log holds each command's output in turn. A
     Rust or engine gate's reason is decision 2's. `rust unreached`, `rust and engine`, and
     `engine` when it does not run, have no command: their line is `NOT RUN <name>: <why>`, and
     their record's `command` and `directory` are null. A Rust or engine gate's line ends its
     command with ` in <dir>`, and its record's `directory` is that directory.
   - Required are now also `rust discovery`, run or not, and every Rust and engine gate that
     runs; a `rust discovery` that is NOT RUN is named in the last line among the required gates
     not run.
   - The record gains `settings`: `{"jobs": <J>, "test_threads": <T>, "all": <bool>}`, and its
     `environment.set` maps each variable this decision sets to its value, `RUST_TEST_THREADS`
     among them. Its `format` stays 1.
   - `--plan` also runs `rust discovery`, printing its line, and then lists the Rust and engine
     gates as a run would choose them. It exits 1 when `rust discovery` fails or is not run.
5. **Tests**, in `tools/tests/test_gate.py`. Make the fake of order 193a's tests answer
   discovery's commands (no manifest, and an engine with no path dependency, unless a test needs
   more), and change what a test of 193a asserts only where this order changes it: the header's
   Environment line, the gates' order, and `rust and engine`, which now appears only when
   discovery did not pass. Then add these, each under 193a's rules for tests; item 5 reads the
   repository, and none runs cargo. The fake carries discovery's cargo commands too. Each fails
   when the behaviour it covers is taken out of `tools/gate.py`.
   1. `test_rust_directories_are_found_as_ci_finds_them`: manifests at `packages/p`,
      `packages/w`, `packages/w/m` (a member, for which the fake `cargo locate-project` prints
      `packages/w`'s manifest), `packages/p/target`, `experiments/e`, `experiments/f/g` and
      `experiments/005-macos-keychain`, with a checks.yml whose `macos_only` line names the
      last. Discovery finds exactly `experiments/005-macos-keychain` (not built),
      `experiments/e` and `experiments/f/g` (built, not linted), and `packages/p` and
      `packages/w` (built and linted). With a checks.yml that lacks the line, `rust discovery`
      fails, its line names `.github/workflows/checks.yml`, the `rust and engine` line says no
      Rust or engine gate was chosen, and the result is FAIL. With no `cargo` on the `PATH`,
      `rust discovery` is NOT RUN with decision 2's words, the last line names it among the
      required gates not run, and `main` returns 1; with `--plan` too it returns 1. With
      `--plan` and discovery passing, the fake sees only `cargo locate-project`,
      `cargo metadata` and `gitleaks version`.
   2. `test_dependents_by_path_run_transitively`: `packages/b` depends by path on `packages/a`,
      `packages/c` on `packages/b` as a dev-dependency, and `packages/d` on nothing, as the fake
      `cargo metadata` says. A change to `packages/a/src/lib.rs` runs `a` because of the change,
      `b` and `c` because of their dependencies, with decision 2's reasons naming `packages/a`
      (and `packages/b` for `c`), and leaves `d` in `rust unreached`. A dependency path outside
      the root fails `rust discovery` with a line that names it; one inside its own directory
      chooses nothing.
   3. `test_reads_and_the_engine_rule`: with the table of decision 2 replaced by two entries,
      `packages/e` reading `spec/` and `packages/f` reading `packages/g/contracts.yaml`: a change
      to `spec/x.yaml` runs `packages/e` and the engine, and not `packages/f`; one to
      `packages/g/contracts.yaml` runs `packages/g` and `packages/f`, and not the engine; one to
      `packages/g/README.md` runs `packages/g` alone; one to `specs/x.yaml` runs nothing. Each
      of `engine/x`, `conformance/x`, `packages/agent-claude-code/x` and
      `packages/agent-codex/x` runs the engine; `docs/x.md` and `tools/x.py` run no Rust gate,
      and the `rust unreached` and `engine` lines give decision 3's words. A `packages/h` that
      the fake `cargo metadata` gives a path dependency under `engine/` runs on a change to
      `engine/x`, with the reason `it depends by path on engine, which the branch changes`, and
      not on one to `packages/agent-codex/x`. With `--all` every directory and the engine run,
      with the reason `--all was given`.
   4. `test_each_kind_of_directory_gets_ci_commands_and_environment`: a package, an
      experiment, the macOS-only probe, a package that the `macos_only` line also names, and the
      engine all chosen, with `--jobs 5 --test-threads 3`, and the environment holding
      `CONDUIT_TEST_NPM`, `SCR_AGENT_CODEX`, `RUSTUP_TOOLCHAIN`, `RUSTFLAGS` and
      `EXAMPLE_API_KEY`, each set to a planted value. The fake sees, in the package's directory,
      `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and
      `cargo test --release`; in the experiment's, `cargo test --release` alone; nothing in the
      probe's, whose gate is NOT RUN and not required; `cargo fmt --check` alone in the listed
      package's; and the three in `engine`. Every cargo command's environment holds
      `CARGO_NET_OFFLINE=true`, `RUSTUP_AUTO_INSTALL=0` and `CARGO_BUILD_JOBS=5`, each
      `cargo test`'s `RUST_TEST_THREADS=3`; none holds a planted variable; the record's
      `settings` and `environment.set` hold those values; and no planted value appears in the
      output. `--jobs 0` and `--test-threads x` raise `SystemExit` with code 2.
   5. `test_reads_table_agrees_with_the_tree`: on the repository itself, each entry of decision
      2's table names a directory with a `Cargo.toml`, and its "Shown in" file there is tracked
      and holds its "By the text". Each directory under `packages/` or `experiments/` with a
      tracked `Cargo.toml` and a tracked `.rs` file holding `"../../spec` or `.join("spec` has
      an entry reading `spec/`; each with one holding `"../../conformance` or
      `.join("conformance` has an entry reading a path under `conformance/`; and each with one
      holding `"../<name>/`, where `<name>` ends the path of another such directory beside it,
      has an entry reading a path under that directory. On `main` at `d4c8cf8b`, 12
      directories read `spec/`, one reads under `conformance/`, and two read under
      `packages/scenario-s2-linear-restrictions/`.
6. **The documents** (D13). Each text named here is given under
   [the texts of decision 6](#the-texts-of-decision-6), by its letter. Rewrap a changed paragraph
   to lines of at most 100 characters, and change no other word.
   - In `docs/implementation/METHOD_AS_RUN.md`, the variables' table gains row V after the row
     of `<FINDINGS_FILE>` (line 30).
   - In the same file, section 7, lines 271 to 286, from "The admin reads the diff" to "runs on
     pull requests and pushes to `main`:", become text A. The line after them, which begins
     with `secrets` in backticks, and every line after it, do not change.
   - In the same file, section 5, lines 220 to 223, from "Orders built on Windows" to "still
     run on the Mac by hand. 131", become the three lines of text B. Line 224, which begins
     "failed to compile on macOS", does not change.
   - In `AGENTS.md`, in the block under "Run these from the repository root before each
     commit", after the line `(cd engine && cargo test --release)` (line 131), add text C, one
     line, and leave the others as they are. Under "What each check needs", after the item for
     `cargo test --release` in `engine/` (lines 165 to 170), add text D.
   - In `CONTRIBUTING.md`, add text C to the block under "Run these from the repository root
     before each commit", after its `(cd engine && cargo test --release)` line (line 205), so
     that the two lists agree on it ([DA12](../proposals/docs-audit/README.md) of the
     documents audit). Under "Settings this repository relies on", the sentence from "The checks
     are to run as local gates" (line 293) to "and the merge waits for it." (line 295) becomes
     text E1, and the sentence from "Since then the only name check is" (line 315) to "can hold
     the names is open." (line 319) becomes text E2. Nothing else in that section changes.
   - In `docs/implementation/HANDOVER.md`, lines 72 to 81, the item that begins "**Each
     merge** needs the local gate to pass", become text F. Nothing else in that file changes:
     its lane coordinator's row already says that a merge waits for the local gate (ruling 80).
   - In `docs/AUDIT.md`, in the row of D11 (line 178), the words G1 and G2 replace the words
     quoted with them. Nothing else in the row changes.

### The texts of decision 6

Row V, for the variables of the method as run:

```markdown
| `<NAMES_FILE>` | the file of private names, of mode 0600 and outside the repository, that `tools/gate.py --private-names-file` reads; where the names are held is open ([ruling 80](../proposals/ruling-80/README.md)) | yes | in the private note, once the names are held |
```

Text A, for section 7 of the method as run:

````markdown
The admin reads the diff, report and every worker choice, then squashes with the guard:
```sh
git fetch origin && git rebase origin/main && git merge-base --is-ancestor origin/main HEAD && git reset --soft origin/main && git -c user.name="Conduit work order" -c user.email=work-orders@conduit.invalid commit
git diff --name-only HEAD^ HEAD
```
The squash commits with the checkpoints' identity, not the handover's plain `git commit`.
Check the second command's files against scope; shared remote refs can move between
commands. Then, on the Mac, with the tree clean, run the gate
([ruling 80](../proposals/ruling-80/README.md), point 10), giving it the file of private
names once they are held:
```sh
python3 tools/gate.py --private-names-file <NAMES_FILE>
```
It compares the branch with `origin/main` as last fetched, and never fetches, pushes or
calls GitHub. It fails a commit since `origin/main` whose author or committer is not the
work-order identity, as a plain rebase leaves one, and names the commit. For what the
branch changes it runs the checks of CI's `checks` workflow on the Mac: the `docs` job's
Python checks in that job's order, with the private names given to
`tools/check_private_paths.py` alone; the vector changes against the declarations the
branch adds or changes, the tools' unit tests and `git diff --check`; `cargo fmt --check`,
clippy with `-D warnings` and `cargo test --release` in each package that the branch
changes, that depends by path on a directory it changes, or that reads a path it changes,
and the tests alone of each such experiment; the engine's fmt, clippy and release tests
when `engine/`, `spec/`, `conformance/` or an Agent package changes; and gitleaks on the
branch's commits when the release that checks.yml pins is on the `PATH`. Cargo runs
offline. The gate prints one line for each gate, PASS, FAIL or NOT RUN with its command
and why, and a last line that says PASS only when every required gate passed, and writes
its record to `target/gate/<HEAD>.json`. `--plan` lists the gates without running them;
`--all` runs every Rust directory and the engine. A FAIL is fixed on the branch, which is
squashed again, and the gate runs again.

The merge waits on the gate, not on GitHub CI. Only after the gate's PASS, push
`wo/NNN-name`, open its pull request with the gate's last line and its record's gates in
the description, and squash-merge with a message written by hand, its subject in the form
of orders merged since 151: `Work order NNN: title (#PR)`. The record cited names the
pushed commit as its head, and says that the tree was clean, that the branch contained
`origin/main` and, once the names are held, that they were configured. Where it says that
gitleaks did not run, no secret scan ran before the push: GitHub's push protection stops
only the patterns it knows, and the pull request's `secrets` job scans after. The gate's
log of the vector changes lists each flip for the reader ([reviewing vector
changes](../../conformance/README.md#reviewing-vector-changes)).
[checks.yml](../../.github/workflows/checks.yml) still runs on pull requests and pushes to
`main`, as a second opinion and the Linux leg that nothing waits on (ruling 80). A run that
fails after a merge is fixed in a follow-up, an order or a coordinator's fix read by a
fresh Opus reader, whose Record names the run. Its jobs:
````

Text B, for section 5 of the method as run:

```markdown
Orders built on Windows passed the changed crate's tests, fmt and clippy on the Mac before
merge. Since order 193b the gate of [section 7](#7-merge-and-the-record) runs them there,
for each Rust directory the branch reaches and the engine; CI's macOS jobs (187) run after. 131
```

Text C, the line for the blocks of checks in AGENTS.md and CONTRIBUTING.md:

```text
python3 tools/gate.py                # CI's checks, run here for what the branch changes; merges wait on its PASS
```

Text D, the item for AGENTS.md:

```markdown
- `gate.py` needs Python 3 and Git, and what each check it runs needs: the items above
  and, for each Rust directory it runs, the toolchain that directory pins installed
  through rustup and its crates in cargo's cache, since it runs cargo offline and lets
  rustup install nothing. When its `python3` lacks PyYAML or jsonschema it runs itself
  again under `/usr/bin/python3`, as `check_model_account.py` does. It compares the
  working tree with `origin/main` as last fetched, or with `--base REF`, and never
  fetches, pushes or calls GitHub. It fails a commit since the base whose author or
  committer is not `Conduit work order <work-orders@conduit.invalid>`. It reads the
  private names from the file that `--private-names-file` names, of mode 0600 and outside
  the repository, gives them to `check_private_paths.py` alone, and prints none; the
  `CONDUIT_PRIVATE_NAMES` of its own environment it does not use. It runs gitleaks only
  when the release that `.github/workflows/checks.yml` pins is on the `PATH`, and
  otherwise says that it did not. Its last line says PASS only when every required gate
  passed, and it writes its record under `target/gate/`. `--plan` lists the gates without
  running them; `--all` runs every Rust directory and the engine. The admin merges on its
  PASS on the Mac, and CI runs after, as a second opinion
  ([ruling 80](docs/proposals/ruling-80/README.md)).
```

Text E1, for CONTRIBUTING.md's settings:

```markdown
The checks run as local gates before merge in `tools/gate.py` (orders 193a and 193b), and
the merge waits for its PASS on the Mac, as [AGENTS.md](AGENTS.md#checks) describes.
```

Text E2, for CONTRIBUTING.md's settings:

```markdown
Since then the only name check is the `private paths` gate of `tools/gate.py` before each
merge ([the method as run](docs/implementation/METHOD_AS_RUN.md#7-merge-and-the-record)),
which gives the checker the names from a file the coordinator names, of mode 0600 and
outside the repository; until the names are held it checks the built-in patterns alone, and
its record says so. Whether one of Conduit's secret stores can hold the names is open.
```

Text F, for the handover:

```markdown
- **Each merge** needs a PASS from `tools/gate.py` on the coordinator's machine and a clean
  fresh verifier ([ruling 80](../proposals/ruling-80/README.md), point 10;
  [the method as run](METHOD_AS_RUN.md#7-merge-and-the-record)). The private handoff note
  names the merge script that runs the gate, as it does the runners'. GitHub's CI keeps
  running on every pull request, as a second opinion and the Linux leg, but no merge waits
  for it, and a run that fails after a merge is fixed in a follow-up. For an order built on
  Windows, the gate runs the changed crates' tests, fmt and clippy on the Mac
  ([the method as run](METHOD_AS_RUN.md#5-on-the-windows-computer)). The merge is the
  guarded squash ([the squash guard](#standing-rules-learned-this-session)).
```

G1, for the tracker's row of D11, in place of "the checks are to run as local gates before merge
in `tools/gate.py`, which order 193 is to build, and until it lands the coordinator runs an
interim local gate before each merge, which the merge waits for;":

```text
the checks run as local gates before merge in `tools/gate.py` (orders 193a and 193b), and the merge waits for its PASS on the Mac;
```

G2, in the same row, in place of the passage from "and the coordinator's local run before each
merge is the only name check;" to "where the names are to be held is open." (on `main` at
`d4c8cf8b` it reads "only the seven patterns" in between; order 196 may have made that "only
the built-in patterns"):

```text
and the gate's `private paths` gate before each merge is the only name check, with the names from a file the coordinator names once they are held; where the names are to be held is open.
```

## Scope

- Changed: `tools/gate.py` and `tools/tests/test_gate.py` (decisions 1 to 5);
  `docs/implementation/METHOD_AS_RUN.md`, one row of its variables and sections 5 and 7; in
  `AGENTS.md`, one line and one item of the Checks section; in `CONTRIBUTING.md`, one line of its
  Checks block and two sentences of its settings; in `docs/implementation/HANDOVER.md`, one item;
  in `docs/AUDIT.md`, two passages of D11's row (decision 6).

Change nothing else: nothing under `.github/`, no other tool, no file under `spec/`,
`conformance/`, `engine/`, `packages/` or `experiments/`, and not IN_FORCE.md, the ledger, ruling
80's record or the roadmap. The gate writes only under `target/`, which Git ignores.

## Done when

From the repository root, each passes:

```sh
/usr/bin/python3 -m unittest discover -s tools/tests     # OK
/usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -v 2>&1 | grep -c -E '^test_(rust_directories_are_found_as_ci_finds_them|dependents_by_path_run_transitively|reads_and_the_engine_rule|each_kind_of_directory_gets_ci_commands_and_environment|reads_table_agrees_with_the_tree) .* ok$'   # prints 5
/usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -v 2>&1 | grep -c -E '^test_[a-z_]+ .* ok$'   # prints 15
/usr/bin/python3 tools/gate.py --plan --base 0e53c233 | grep -E '^(WOULD RUN|NOT RUN) (rust|engine)' | sed -E 's/:.*//'   # the first list below
python3 tools/gate.py --plan --base 0e53c233 | grep -c '^WOULD RUN rust experiments/011'   # prints 0 on main at d4c8cf8b; 1 once a commit that changes spec/ or engine/ is on your tree (orders 171a, 171b, 172a, 172b and 178 do): report it
python3 tools/gate.py --plan --base 86b8fe4a~1 | grep -E '^WOULD RUN (rust|engine) ' | sed -E 's/:.*//'   # the second list below
python3 tools/gate.py --plan --base 86b8fe4a~1 | grep -F 'WOULD RUN rust experiments/011-short-form-spike test:' | grep -c 'depends by path on engine'   # prints 1
python3 tools/gate.py; echo "exit $?"                    # the last line below, then exit 0
python3 -c 'import json, subprocess; h = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip(); r = json.load(open("target/gate/" + h + ".json")); print(r["result"], r["tree"]["clean"], r["environment"]["private_names_configured"], r["settings"], [g["name"] for g in r["gates"] if g["result"] == "NOT RUN"])'   # PASS False False {'jobs': 6, 'test_threads': 4, 'all': False} ['identity', 'secrets', 'rust unreached', 'engine']
! grep -q -F "$(git rev-parse --show-toplevel)" target/gate/*.json && echo "no absolute path"
grep -c -F 'python3 tools/gate.py --private-names-file <NAMES_FILE>' docs/implementation/METHOD_AS_RUN.md   # prints 1
grep -c -F 'wait for CI' docs/implementation/METHOD_AS_RUN.md   # prints 0
grep -c -F 'order 193 is to build' CONTRIBUTING.md docs/AUDIT.md   # each 0
grep -c -F 'once order 193 lands it' docs/implementation/HANDOVER.md   # prints 0
grep -c -E '^python3 tools/gate\.py ' AGENTS.md CONTRIBUTING.md   # each 1
python3 tools/check_docs.py                              # Errors: 0
python3 tools/check_private_paths.py                     # Errors: 0
git diff --check
git diff --quiet -- .github && echo ".github unchanged"
git status --short
```

On `main` at `d4c8cf8b` with this order's changes, the first `--plan` prints exactly:

```text
WOULD RUN rust packages/provider-local-machine fmt
WOULD RUN rust packages/provider-local-machine clippy
WOULD RUN rust packages/provider-local-machine test
WOULD RUN rust packages/provider-local-process fmt
WOULD RUN rust packages/provider-local-process clippy
WOULD RUN rust packages/provider-local-process test
NOT RUN rust unreached
WOULD RUN engine fmt
WOULD RUN engine clippy
WOULD RUN engine test
```

since `0ed26306` (order 187) changed a test in each of those two packages, and `4a37c3ba` (order
188) changed `conformance/README.md`, which runs the engine with the reason
`the branch changes 1 paths under conformance/`. On `main` at `d4c8cf8b` nothing since
`0e53c233` changed `engine/` or `spec/`, so experiment 011, which depends on the engine and reads
`spec/`, does not run. Orders 171a, 171b, 172a and 172b change `spec/`, and order 178 `engine/`:
once one of them is on your tree, both lists gain every directory that reads `spec/` or depends
on the engine, experiment 011 among them. That is not a fault: report each line added with the
commit that adds it. The second prints exactly:

```text
WOULD RUN rust experiments/011-short-form-spike test
WOULD RUN rust packages/provider-local-machine fmt
WOULD RUN rust packages/provider-local-machine clippy
WOULD RUN rust packages/provider-local-machine test
WOULD RUN rust packages/provider-local-process fmt
WOULD RUN rust packages/provider-local-process clippy
WOULD RUN rust packages/provider-local-process test
WOULD RUN engine fmt
WOULD RUN engine clippy
WOULD RUN engine test
```

since `86b8fe4a` (order 183) changed `engine/CONFORMANCE.md`, and experiment 011 depends by path
on the engine's loader and core. Order 193a's own commit changes neither list. If `main` has moved
further, a commit since may add lines to either list: report each difference with the commit and
path that cause it.

The full run on your tree runs no cargo command but discovery's. Its last line begins
`Gate: PASS.`, names `identity` and `secrets` (the branch adds no commit, or gitleaks is not
installed), `rust unreached` and `engine` among the gates not run, and goes on with the sentence
on uncommitted paths, since you commit nothing; then `exit 0`. `git status --short` must list
exactly these seven paths, all changed: `AGENTS.md`, `CONTRIBUTING.md`, `docs/AUDIT.md`,
`docs/implementation/HANDOVER.md`, `docs/implementation/METHOD_AS_RUN.md`, `tools/gate.py` and
`tools/tests/test_gate.py`.

Then show that each new test can fail. For each of the 5, from the repository root, one at a
time: copy `tools/gate.py` to `target/gate.py.orig`, take out or change the one behaviour the test
covers (a line or two), run that test alone and see it fail, then copy `target/gate.py.orig`
back; `cmp tools/gate.py target/gate.py.orig` must then print nothing. Git ignores `target/`.

Do not stage anything. In your report, mark each choice you made yourself as your own; give the
full output of the gate's run and of each `--plan` above (they are short), each change you made
to a test of order 193a and why, for each new test the change that made it fail, and the last
line of every check.

After the report:

- Before it merges, a fresh Codex verifier and a fresh read-only Opus reader read the result
  ([ruling 79](../proposals/ruling-79/README.md), point 2).
- This order's own merge is the first that waits on the gate alone: the admin runs
  `python3 tools/gate.py --private-names-file <NAMES_FILE>` on the squash, or without the option
  while the names are not held, cites its record in the pull request, and merges on its PASS.
  The Record gives that run's gates and times, the engine's among them when it runs, and the
  coordinator switches its private merge script to the gate.

## Record

Run on 2026-11-17, from about 00:27 to 01:07 UTC, on the Mac from `main` `f08a73ea` (order 193a's
merge), in two rounds: the worker (1510 s) and a fresh Codex verifier (198 s), then the worker again
(433 s) and a second fresh Codex verifier (251 s). The change is the seven files of Scope.
`tools/gate.py` gains `--all`, `--jobs` and `--test-threads`; a first gate, `rust discovery`, which
finds the Rust directories as `rust-dirs` does and their path dependencies, with
`cargo locate-project` and `cargo metadata --no-deps --offline` run from the root; decision 2's
selection, with its table as `RUST_READS`; decision 3's Rust gates, `rust unreached`, and the
engine's three gates or `engine`; the environment that cargo runs with and the record's `settings`;
and a `--plan` that runs discovery and exits 1 when it does not pass. `tools/tests/test_gate.py` has
the fake answer discovery's commands, changes order 193a's tests where this order changes what they
assert (the `Environment:` line, the gates' order and `rust and engine`), and adds decision 5's five
tests. The documents take row V and texts A and B in the method as run, C and D in AGENTS.md, C, E1
and E2 in CONTRIBUTING.md, F in the handover, and G1 and G2 in the tracker. The worker's own
choices: a breadth-first search for a dependency's route, in byte order on ties; invalid metadata,
and a path dependency inside the repository that no discovered directory or engine holds, fail
discovery; the count of manifests includes the engine's; and the plan's last line said that
discovery was checked. On the worker's tree, both plans of "Done when" exceeded the order's lists
only by the lines of the commits since `d4c8cf8b` that change `engine/`, `spec/` or experiment 012
(orders 178, 171a, 172a and 172b): experiment 011 in the first list, and experiments 012 and 022 and
eight more packages' fmt, clippy and test in both, as the order foresaw. The first verifier found
one must: a path dependency outside the repository put its absolute path in the record's `why`,
against order 193a's rule of no absolute path in the record. Round 2 gave `DiscoveryError` a
path-free `record_reason`, kept the path in the line and the log, and extended decision 5's test 2
to check the record. The second verifier found nothing.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), on the branch before its
rebase. It ran the gate's tests with `-v` (15 ok lines, the five new among them), the tool tests
(123, OK), both plans of "Done when" (37 and 36 lines, supersets of the order's lists by the lines
of orders 171a, 172a, 172b and 178; at `d4c8cf8b` the gate's selection gives the order's lists
exactly), the full gate (its last line began
`Gate: PASS. 13 passed; not run: secrets, rust unreached, engine.`, exit 0; no cargo command ran but
discovery's 41), the record's summary, `no absolute path`, the order's greps, `check_docs` and
`check_private_paths` (Errors: 0 each) and `git diff --check origin/main...HEAD` (clean); `.github`
was unchanged and the diff held the seven files of Scope. It also ran a `--plan` with no cargo on
the `PATH` (exit 1); with scratch scripts it compared the texts of decision 6 with the files
(verbatim, E1 and E2 after the rewrap) and the order's table with `RUST_READS` (16 rows, identical);
and on scratch copies it ran a probe in which `cargo metadata` exits 101, and eight removals, each
of which failed its test, and a ninth, R1's, which did not. It found no must and eight shoulds:

- R1: in decision 5's test 1, the check that the failed discovery's line names
  `.github/workflows/checks.yml` searched the whole output, where the `secrets` line always names
  that file, so it could not fail.
- R2: the plan's last line said `Only rust discovery was checked` when discovery did not run, and
  counted discovery, passed or failed, among the gates that would run.
- R3: for a discovery command that fails, the line's `its last line:` repeated the gate's own words
  for the exit, and cargo's message reached only the tail on stderr.
- R4: texts A and D said that `--plan` lists the gates without running them, though it runs
  `rust discovery` and exits 1 when that does not pass.
- R5: text D did not say that discovery, which runs cargo from the root, where no toolchain is
  pinned, needs rustup's default toolchain (inferred from rustup's behaviour, not reproduced).
- R6: CONTRIBUTING.md's lists of what each check needs and what the checks cover had no item for
  `gate.py`.
- R7: a path dependency inside the repository that no discovered directory or engine holds fails
  discovery, the worker's own choice, and no test covered it.
- R8: the usage printed `BASE`, `PRIVATE_NAMES_FILE`, `JOBS` and `TEST_THREADS`, not decision 1's
  `REF`, `FILE` and `N`.

The fixer rebased the branch onto `origin/main` at `0db2951c` (order 171b) with the work-order
identity, without conflict, and applied each finding:

- R1: decision 5's test 1 takes the `FAIL rust discovery` line and checks that it holds
  `macos_only could not be read from .github/workflows/checks.yml`.
- R2: the plan's last line begins `Gate plan: rust discovery passed;`, `failed;` or `was not run;`,
  then counts the other gates, `<W> other gates would run, <N> would not.`, and ends
  `No other gate was checked, and no record was written.` The test
  `test_plan_lists_the_gates_and_runs_none` expects it, and decision 5's test 1 checks its start
  when cargo is missing and when a discovery command fails.
- R3: `DiscoveryError` carries `from_command` for a command's exit or timeout, and the gate adds its
  own words to the discovery log only for the other faults (no `macos_only` line, a path outside the
  repository, invalid metadata, a dependency no directory holds), so that `its last line:` is
  cargo's. The same test runs the gate with `cargo metadata` exiting 101 and checks for
  `exit 101; its last line: error: failed to parse manifest; log`.
- R4: in text D of AGENTS.md and text A of the method as run, `--plan` now "runs `rust discovery`
  alone and lists the other gates without running them, and exits 1 when discovery does not pass",
  and the rest of each paragraph is rewrapped.
- R5: text D gains the sentence "Its discovery runs cargo from the root, which pins no toolchain, so
  it needs rustup's default toolchain too."
- R6: CONTRIBUTING.md gains an item for `gate.py` in each list: what it needs, the default toolchain
  among it, with a link to AGENTS.md for the rest; and what it covers, with a link to the method as
  run's section 7.
- R7: decision 5's test 2 gives `packages/d` a path dependency at `tools/crate`, and checks exit 1
  and `path dependency has no discovered directory: tools/crate` in the output and in the record's
  `why`. This differs from CI: `changed-packages` matches such a dependency by its own directory as
  a prefix and goes on, where the gate fails discovery, and with it the run.
- R8: `--base` takes the metavar `REF`, `--private-names-file` `FILE`, and `--jobs` and
  `--test-threads` `N`; `--help` prints
  `[--base REF] [--plan] [--private-names-file FILE] [--all] [--jobs N] [--test-threads N]`.

On scratch copies, each fix taken out failed its test: R1's message without the file's name, R2's
former last line, R3's words added for every fault, and R7's dependency skipped. The fixer then ran,
from the worktree's root under `env -i` with a minimal `PATH` and `HOME`, on the files of this
commit before it was made: `/usr/bin/python3 -m unittest tools.tests.test_gate -v` (15 ok lines,
OK), `/usr/bin/python3 -m unittest discover -s tools/tests` (123, OK), `python3 tools/check_docs.py`
and `python3 tools/check_private_paths.py` (Errors: 0 each), `git diff --check origin/main...HEAD`
and `git diff --check` (clean), and `/usr/bin/python3 tools/gate.py`, which exited 0 with the last
line `Gate: PASS. 13 passed; not run: secrets, rust unreached, engine.`, then the record's path and
the sentence on the uncommitted paths. Its `rust discovery` passed on 21 manifests, cargo ran only
discovery's 41 commands, `identity` passed on the branch's 2 commits, and `secrets` did not run,
since gitleaks is not on that `PATH`. `--plan` ended
`Gate plan: rust discovery passed; 12 other gates would run, 3 would not.` and exited 0; with no
cargo on the `PATH` it ended
`Gate plan: rust discovery was not run; 12 other gates would run, 2 would not.` and exited 1. The
gate's run on this order's squash, which this merge waits on, and the switch of the coordinator's
private merge script are the coordinator's at the merge.

The coordinator applied the second reader's two shoulds: the `gate.py` item of CONTRIBUTING.md's
"What the checks cover" no longer lists the commit identity among CI's checks, which do not check
it, and this Record says which of the reader's comparisons ran with scratch scripts and which on
scratch copies. At the merge the coordinator ran the gate on a trial squash, `9908637a691c`, which
differs from the merged squash by this paragraph alone: its last line began `Gate: PASS. 14
passed; not run: rust unreached, engine.`, with `rust discovery` PASS on 21 manifests, `identity`
PASS on 1 commit and `secrets` run, gitleaks 8.24.2 on the coordinator's `PATH`; the interim gate,
which still gates this merge, printed `GATE PASS`. The coordinator's merge script switches to the
gate after this merge.
