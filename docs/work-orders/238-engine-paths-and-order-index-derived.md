# Work order 238: two shared lines that serialize merges: the gate's engine paths derived from the tree, and the order index taken from each order's first line

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate. It builds nothing of the tree: it runs
`cargo metadata` and `cargo locate-project`, offline, in the start check, inside the gate's
`--plan` and in the tool tests, whose cache test also builds a probe crate in a temporary
directory. Never print a whole large file, diff or log. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Do not run `conduit`, `codex`, `claude` or the dogfood harness, and sign in
nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Run the tool tests with
`/usr/bin/python3`, which has PyYAML; the `python3` on the `PATH` may not.

**The git rule.** Beyond this order's checks (`git merge-base`, `git show`, `git ls-files`,
`git status`, `git diff`, `git add -N`, `git reset`, and the gate's `--plan`), run git only on
repositories a test makes under its temporary directory, or as `git ls-files` on this
repository, as `tools/tests/test_gate.py` does today.

## What and why

**The gap** (the dogfood-gaps stream;
[ruling 92](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-92/README.md),
point 9; [DEC-027](../decisions/log.md#dec-027-2026-11-20-conduit-builds-conduit-directly-through-conduit)).
Two shared lines made each merge of 2026-11-20 and 11-21 wait for the one before it:

1. Each order whose engine test builds or reads a package adds its directory by hand to
   `ENGINE_PREFIXES` and `ENGINE_WHY` in `tools/gate.py`, and a row to
   `test_reads_and_the_engine_rule` in `tools/tests/test_gate.py`. The tuple and the sentence are
   wrapped, so any addition rewraps them: orders 218, 229, 227, 226, 233 and 232 each changed
   them, and each conflicted with the one merged before.
2. Each order's text adds a row at the end of the table in `docs/work-orders/README.md`, so
   each order text conflicts with the one merged before it.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, as existing practice. Build tools choose what to test from the graph they already read,
not from a hand list beside it (Bazel's `rdeps`, Nx's `affected`); the gate already reads
cargo's path dependencies this way, and the engine's own sources name what its tests build.
adr-tools' `adr generate toc` and Python's PEP 0 build an index from each record's title; a
hand-kept table restates the title, against one place per fact (rule 7). Not taken (the
coordinator's, open to the owner's strike): a generated table committed in the README, since
each order would still add a line that conflicts; a declared table in a shared file, for the
same reason.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): none
served and none at risk. No promise, case, engine source or package changes. It serves the
dogfood, so that Conduit's orders can merge in parallel.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (4-hour limit; aim for under
1.5 hours; the whole tool suite once, at the end), a Codex verifier and a Claude reader
([ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md)),
who run only what settles a doubt, and the gate
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). Python under `tools/` is
the named exception to Rust
([DEC-025](../decisions/log.md#dec-025-2026-11-20-the-engine-is-rust-no-private-code)); use the
standard library alone. By
[ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md),
each decision says what must hold; how is yours. The names are the coordinator's, open to the
owner's strike.

## The decisions

1. **The engine's paths, derived.** At each run the gate takes the paths whose change runs the
   engine from the tree it checks: `engine/`, `spec/` and `conformance/`, then each directory
   under `packages/` or `experiments/` that the engine's Rust sources build or read, with every
   directory those depend on by path (transitively, from the map discovery already builds; add
   no cargo call), in byte order of each path with its trailing `/`. What must hold:
   - No directory under `packages/` or `experiments/` is named in `tools/gate.py`'s engine rule.
     A new package that a new engine test builds needs no line in `tools/gate.py` or
     `tools/tests/test_gate.py`.
   - On this tree the paths are today's fourteen, in today's order; the `why` of the engine row
     is today's words for them ("the branch changes nothing under A, B, ... or Z"), character for
     character; and the engine row's `reason` is as today ("the branch changes N paths under P,
     Q"). Which rows run for any change is as today.
   - A read is never missed silently: a tool test fails where an engine Rust source names, by a
     literal path, a directory under `packages/` or `experiments/` (the repository's top-level
     ones; `"conformance/fixtures/s1/packages/filesystem"` in `make_archives.rs` names a directory
     under `conformance/`) that the derived paths lack.
   - Where discovery does not pass, the rows are as today (`RUST_WHY`).

   How the reads are found is yours. The coordinator checked one way against today's tree. In
   the string literals of the tracked `engine/**/*.rs`, a Rust directory is read where a literal
   is its path or holds it followed by `/` (`"packages/git"`,
   `"experiments/013-s2-shared-login/linear-stub"`, `"../../../../packages/git/tests/..."`); and a
   directory directly under `packages/` is read where a literal is its name in a file that also
   holds the literal `"packages"` (as `agents.rs` and `s2.rs` join theirs). That gives ten; path
   dependencies add `packages/runtime-provider-kit` (through `provider-local-process`), and the
   linear stub again (through `scenario-s2-linear`): today's eleven, exactly. Keep the name rule
   to `packages/`: `agents.rs` also holds `"scripted-client"`, a binary of the Agent packages,
   which is the last segment of `experiments/015-s4-partial-setup/scripted-client`, a directory
   the engine does not read. A declared line beside each reader, checked against its literals, is
   another way. A table in a shared file is not.
2. **The tests of decision 1**, in `tools/tests/test_gate.py`, at least these, by these names
   (no other test's name holds `test_engine_paths_`):
   - `test_engine_paths_keep_todays_rows`: a table of today's fourteen paths, written once. On
     this repository's tree each is among the derived paths, and a change under each selects the
     engine row with the reason `the branch changes 1 paths under P`. A change to `docs/x.md`,
     `tools/x.py` or `specs/x.yaml`, or under each of today's twelve other Rust directories
     (`experiments/005-macos-keychain`, `011-short-form-spike`, `012-model-checking`,
     `015-s4-partial-setup/scripted-client`, `022-provider-contracts`,
     `027-a5-sign-in/client-facts` and `027-a5-sign-in/refresh-probe`; `packages/provider-keychain`,
     `provider-local-directory`, `provider-local-machine`, `provider-ssh-runtime` and
     `scenario-s2-linear-adapter`), selects no engine row. The words the gate writes for the
     fourteen are `ENGINE_WHY` as at `c1d09afc`, character for character. A new package needs no
     row here. It may run `cargo metadata --offline` on this tree, as discovery does.
   - `test_engine_paths_take_a_new_read_with_no_edit`: in a temporary tree, a new package and an
     engine test whose source names it select the engine row for a change in the package; with
     that test removed, they do not. Nothing in the gate is patched for it.
   - `test_engine_paths_agree_with_the_tree`: decision 1's never-missed rule, on this repository.

   `test_reads_and_the_engine_rule` drops its rows for directories under `packages/` and
   `experiments/` that only the engine reads; its other cases stay. Change other tests only where
   they read `ENGINE_PREFIXES` or `ENGINE_WHY`, and say which.
3. **The order index, from each order's first line.**
   - **The rule.** Each top-level file of `docs/work-orders/` whose name starts with a digit and
     ends `.md` has the first line `# Work order ID: WHAT` or `# Work order ID (LABEL): WHAT`;
     its file name starts with `ID-`, `WHAT` is not empty and holds no `|`, and no two files
     share an ID. All 62 order files on `c1d09afc` with this one keep it (the coordinator
     checked). Files in subdirectories, `README.md` and `verify.md` are not order files.
   - **The index.** `python3 tools/check_docs.py --work-orders` prints `| Order | What |`,
     `| --- | --- |`, then one row per order file in file-name order: its ID as a Markdown link to
     the file, then its `WHAT`, as the README's rows are today; and exits 0. Where a file breaks the rule it prints each error and exits 1. It and `--walk`
     may be given together or alone; any other argument is refused, as today.
   - **The check.** `python3 tools/check_docs.py` with no option also fails, one error per file
     and naming it, counted in its last line, `Errors: N`: where an order file's first line breaks
     the rule, and where a table row of `docs/work-orders/README.md` links a top-level order file.
     An order adds no row there. A tree without `docs/work-orders/` has no order files and no
     error (`tools/tests/test_condensation_checks.py` runs `check()` on one).
   - **The README** keeps its prose, its sections and its three rows without links (048a7, 048a8
     and 050o), and drops every row that links an order file. In the paragraph above the table,
     "One row for each order file in this directory, in file-name order." becomes one sentence:
     each order file's first line is its index entry, which `python3 tools/check_docs.py
     --work-orders` prints as a table. "they have rows without links where their names sort"
     becomes "they have the rows below, without links". Nothing else in the README changes.
   - **Ruling 90.** Its point 3 has step A add "a check that each order file has one row in its
     directory's index"
     ([ruling 90](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-90/README.md)).
     By the coordinator's reading, open to the owner's strike, that row is now each file's first
     line, checked here, and the index is the one the checker prints.
4. **The tests of decision 3**, in a new file under `tools/tests/`, at least these, by these
   names (no other test's name holds `test_work_order_index_`):
   - `test_work_order_index_from_first_lines`: in a temporary tree with three orders, one of them
     labelled, `--work-orders` prints their rows in file-name order; a fourth order file then
     shows in the index with no change to the README, and the check passes.
   - `test_work_order_index_refuses_bad_lines_and_rows`: a file with no heading, an ID that is
     not its file's, an empty `WHAT`, a `WHAT` with `|`, two files with one ID, and a README row
     linking an order file are each one error naming the file.

## What this leaves

Report this list, corrected by what you built:

- `RUST_READS` stays a hand table: each package's `spec/` row is still a shared line, sorted, so
  two package orders conflict only where their rows meet. Deriving it the same way is a later
  order.
- An order that makes the engine read one of today's twelve other Rust directories moves it in
  `test_engine_paths_keep_todays_rows`; a new package needs nothing.
- A branch rebased after this merges drops its row in the README; `check_docs.py` reports it.
- Order 212, not yet run, plans hand rows, live and merged tables, and a one-row rule in
  `check_docs.py` (its sections 13 and 15). The coordinator refreshes it to this form before it
  runs.
- An index split into live and merged orders (a Record filled or "Not yet run."), if wanted, is
  a later option of the checker.

## Stop rule

Stop, change nothing further, and report the gap and the options it leaves, without choosing,
where: a start-check line prints otherwise (a crate missing from the offline cache among them);
no derivation you find gives today's fourteen paths exactly; an order file on your base breaks
decision 3's rule (do not edit it); or a correct result needs a change outside the scope, such as
an engine source, `.github/workflows/`, `PYTHON_GATES` or another tool.

A choice that no promise governs is yours: list it under `own_choices`.

## What to read

`tools/gate.py` (`select_rust`, `discover_rust`, `show`, `ENGINE_PREFIXES`, `ENGINE_WHY`,
`RUST_READS`); `tools/tests/test_gate.py` (`test_reads_and_the_engine_rule`,
`test_reads_table_agrees_with_the_tree`, and the two that assert `ENGINE_WHY`);
`tools/check_docs.py` (its arguments and its error form); `docs/work-orders/README.md`; and the
path literals of `engine/crates/shell/tests/agents.rs`, `github.rs`, `local_process.rs`, `s2.rs`,
`secrets.rs`, `serve.rs` and `working_copy.rs`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor c1d09afc HEAD; echo "main $?"
grep -c '^ENGINE_PREFIXES = (' tools/gate.py
grep -cE 'packages/(agent-claude-code|agent-codex|git|github|provider-file|provider-local-process|runtime-provider-kit|scenario-s4-effect)/|linear-stub/' tools/gate.py
grep -c -- '--work-orders' tools/check_docs.py
test "$(grep -c '^| \[' docs/work-orders/README.md)" = "$(ls docs/work-orders/[0-9]*.md | wc -l | tr -d ' ')"; echo "rows $?"
grep -cE '^\| (048a7|048a8|050o) \|' docs/work-orders/README.md
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for d in engine $(git ls-files '*Cargo.toml' | grep -E '^(packages|experiments)/' | xargs -n1 dirname); do (cd "$d" && RUSTUP_AUTO_INSTALL=0 cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1) || echo "missing $d"; done | wc -l | tr -d ' '
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, a number above 0 (9 at `c1d09afc`), `0`, `rows 0`, `3`, `0` and `0`.
A `missing` count above 0 means a crate or a pinned toolchain is not on this machine: stop.

## Scope

Change `tools/gate.py`, `tools/tests/test_gate.py` and `tools/check_docs.py` (its docstring
too); create one test file under `tools/tests/`; change `docs/work-orders/README.md` as decision
3 says. Change nothing else: not `engine/`, `spec/`, `conformance/`, `packages/`,
`experiments/`, `.github/`, `AGENTS.md`, `HOW-WE-WORK.md`, another order file or another tool.
Put anything else that needs changing in your report.

## Done when

Each line in the first group prints otherwise on `main` at `c1d09afc` (shown as "fails today")
and passes after.

```sh
grep -cE 'packages/(agent-claude-code|agent-codex|git|github|provider-file|provider-local-process|runtime-provider-kit|scenario-s4-effect)/|linear-stub/' tools/gate.py  # 0; fails today (9)
/usr/bin/python3 -m unittest discover -s tools/tests -k test_engine_paths_ -v 2>&1 | grep -c ' \.\.\. ok$'  # 3; fails today (0)
/usr/bin/python3 -m unittest discover -s tools/tests -k test_work_order_index_ -v 2>&1 | grep -c ' \.\.\. ok$'  # 2; fails today (0)
grep -c '^| \[' docs/work-orders/README.md  # 0; fails today (62 or more)
test "$(python3 tools/check_docs.py --work-orders | grep -c '^| \[')" = "$(ls docs/work-orders/[0-9]*.md | wc -l | tr -d ' ')"; echo "index $?"  # index 0; fails today (index 1)
```

These pass, as on `main`. The first shows the derived paths end to end, with discovery's real
path dependencies; if an order merged since `c1d09afc` added an engine read, its line names that
directory too, in byte order, and you say so.

```sh
w=$(git show c1d09afc:tools/gate.py | /usr/bin/python3 -c 'import ast,sys; t=ast.parse(sys.stdin.read()); print(next(ast.literal_eval(n.value) for n in t.body if isinstance(n, ast.Assign) and getattr(n.targets[0], "id", "") == "ENGINE_WHY"))')
/usr/bin/python3 tools/gate.py --plan --base HEAD 2>&1 | grep -cxF "NOT RUN engine: $w"  # 1
grep -cE '^\| (048a7|048a8|050o) \|' docs/work-orders/README.md  # 3
git add -N tools                               # so the checks see the new test file
python3 tools/check_docs.py                    # Errors: 0
python3 tools/check_private_paths.py           # Errors: 0
/usr/bin/python3 -m unittest discover -s tools/tests  # OK, the whole tool suite, once
git diff --check                               # prints nothing
git diff --name-only HEAD | grep -vE '^(tools/gate\.py|tools/check_docs\.py|tools/tests/[^/]+\.py|docs/work-orders/README\.md)$' | wc -l | tr -d ' '  # 0
git reset -q                                   # stage nothing
```

Your report gives, per decision, what you built and its tests; how the engine's reads are found
and why that misses none on this tree; the `--plan` line; `own_choices`; the corrected list of
what this leaves; and every check's last line.

## Record

Not yet run.
