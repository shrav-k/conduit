# Work order 071 (S0a): the coverage index in pieces, the runner's seams, and parallel vectors

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

Almost every build order edits the same few places, so orders that could run in parallel
conflict and must merge one after another. Spec batch 1's plan calls them its hot files
([the plan](../proposals/spec-batch-1/README.md), line 665):

- `conformance/manifest.yaml` lists all 413 cases in one `cases` list (lines 3-4714). Each new
  vector appends an entry to it.
- `conformance/case.schema.json` holds every step input as a branch of one `oneOf` list (line 87;
  21 branches), and every expectation as a property of one `expect` object (line 795; 20
  properties). Its `allOf` list (line 949) holds 6 rules that tie input kinds to the
  expectations they require. Later orders add to all three: a `translate` step
  ([short form](../proposals/spec-batch-1/areas/short-form.md), line 160), the inputs of pending
  actions ([pending actions](../proposals/spec-batch-1/areas/pending-actions.md), line 239), and
  `machine-report` and `clock` ([machines](../proposals/spec-batch-1/areas/machines-and-trust.md),
  line 181).
- `engine/crates/conformance/src/lib.rs` runs every step kind and checks every expectation in one
  function, `run_case` (lines 417-1013).

The runner also runs the vectors one after another, inside one test (`tests/vectors.rs`), and
each order runs them several times. Under [ruling 71](../proposals/ruling-71/README.md) the work
now runs in parallel lanes. This order splits these places along their seams, and runs the
vectors in parallel. Order 043a is to be stacked on this order and will write its `translate` step
in the new modules and pieces.

This order changes no vector, expectation, promise or outcome. On a valid tree the runner prints
the same output as today. No promise governs the runner's form: it is the convention in
[conformance/README.md](../../conformance/README.md), adopted by increment 2's decisions 3, 4 and
7, and the admin decides it. Size: L, at its upper end. That is above the plan's M-L, for the
move of the runner's code and a tool that merges as well as splits.

Line numbers are from `main` at cfd7b23; e94c057 changed none of the lines cited. If your tree
moved one, find the place by its content.

**BASE** is the commit this order starts from. In round one it is `HEAD`: you never commit, so
`HEAD` is the commit your tree started from. In a later round, the admin's prompt names BASE.
Write its hash in your report, and use it wherever this order writes BASE.

Read:

- `conformance/README.md`: lines 1-22, "Running" (line 103), "Reviewing vector changes" (line
  119), and "S1 whole and promise inventory (I2-09)" (line 435);
- in `engine/crates/conformance/`: `src/lib.rs`, `src/main.rs`, `tests/vectors.rs`,
  `src/schemas.rs` lines 303-398, and `examples/make_archives.rs`, the precedent for a tool
  written as an example;
- `tools/check_docs.py`: `PATH_MENTION` (lines 197-199) and `path_mentions` (lines 202-243), and
  its test, `tools/tests/test_condensation_checks.py` (lines 148-167);
- `tools/set_names.py`, lines 71-73 and 247-254;
- `python3 tools/vector_delta.py --help`.

## What to do

This order is dispatched after order 068 merges. Before step 1, check that your tree has
`tools/check_private_paths.py` and `engine/.config/nextest.toml`. If either is missing, stop and
report it.

1. **Measure BASE first.** Change nothing before this step is done.
   - From the repository root, copy BASE's tree into ignored build output:
     `mkdir -p engine/target/s0a/base`, then `git archive BASE | tar -x -C engine/target/s0a/base`.
     Run `git archive` from the root: from `engine/` it archives only `engine/`. If the directory
     already holds an older copy, delete it first, except as step 10 says.
   - From `engine/`, build BASE's runner:
     `cargo build --release --manifest-path target/s0a/base/engine/Cargo.toml -p conduit-conformance`.
   - Run it, timed, and keep its output:
     `time cargo run --release --manifest-path target/s0a/base/engine/Cargo.toml -p conduit-conformance > target/s0a/runner-before.txt`.
     The output is long. Never print it: use `wc`, `tail -1` or `cmp`.
   - Time BASE's vectors test:
     `cargo nextest run --release --manifest-path target/s0a/base/engine/Cargo.toml -p conduit-conformance --test vectors`.

   Never commit anything under `target/`. Run one cargo command at a time, so that the times
   compare.

2. **The coverage index in pieces.**
   - A piece is the file `index.yaml` in each directory that holds case directories. The case
     `conformance/conventions/requests/aware-failed/case.yaml` is listed in
     `conformance/conventions/requests/index.yaml`. The case
     `conformance/language/registration/e1/conduit-as/case.yaml` is listed in
     `conformance/language/registration/e1/index.yaml`. At cfd7b23 that gives 18 pieces. Under
     `conformance/`, the name `index.yaml` is kept for pieces.
   - A piece is `{"cases": [...]}`. Its entries are sorted by `id`, in byte order. It is written
     as `serde_json::to_string_pretty` writes it, plus a final newline. That is the form of
     `manifest.yaml`, so each entry's lines are the same bytes as in `manifest.yaml`.
   - Each entry keeps the manifest's members exactly, in the manifest's order: `id`; `case`, still
     relative to `conformance/`; `promises`; and `safety_rows`. Where the manifest has no
     `safety_rows`, the entry has none (85 entries at cfd7b23). Where it has an empty list, the
     entry keeps `[]` (64 entries). An entry with any other member is an error, so that nothing
     is dropped.
   - `manifest.yaml` loses its `cases` member and nothing else. `status`, `s1_coverage` and
     `s1_coverage_source` keep every byte. The links to it still hold: it records the S1
     coverage and its gaps (`docs/AUDIT.md` lines 47, 62, 150, 413 and 436; `spec/README.md`;
     `README.md`; `engine/CONFORMANCE.md`). One sentence becomes false: `spec/README.md` line 193
     says it "lists the cases". Step 8 rewrites that line.

3. **The case schema in pieces.**
   - Each branch of the step input's `oneOf` (`case.schema.json` line 87) moves to
     `conformance/case-schema/inputs/<kind>.json`. The file is named after the branch's kind. A
     branch that lists several kinds is named after the first: `schema-case.json` and
     `register-project.json`.
   - Each property of `expect` (line 795) moves to `conformance/case-schema/expect/<name>.json`.
   - The 6 `allOf` rules (line 949) stay in `case.schema.json`. A later order adds a rule as a
     file of its own, `conformance/case-schema/rules/<name>.json`, holding one rule, and the
     runner appends these files to `allOf` in file-name order. No rule file exists yet; write
     none.
   - Everything else stays in `case.schema.json`: `allOf`, `capture`, `initial` and the required
     members. Where the moved parts were, `case.schema.json` holds placeholders of your choice.
     With them, `case.schema.json` must still be a valid JSON Schema on its own.
   - The pieces, and what remains of `case.schema.json`, keep BASE's key order. The diff of
     `case.schema.json` from BASE only removes the moved parts and adds the placeholders.
   - The runner assembles the schema where it reads it (`lib.rs:81`). The assembled schema must
     equal BASE's `case.schema.json` as a JSON value, once the two lists of input branches are put
     in one order.

4. **The tool.** It is written in Rust, in the conformance crate:
   - its logic, and the assembly of step 3, are a public library module, `src/pieces.rs`, of pure
     functions over file contents, with unit tests that write no file;
   - its command is the example `examples/pieces.rs`. It only reads its arguments and the files,
     calls `pieces`, and writes the result. `examples/make_archives.rs` is the precedent for a
     tool written as an example; unlike it, keep this one thin.

   Keep key order without `preserve_order`. Write the pieces, and rewrite `manifest.yaml` and
   `case.schema.json`, through a small order-keeping value type in `src/pieces.rs`, with its own
   `Deserialize` and `Serialize`, printed by `serde_json`'s pretty printer. Both files are in that
   printer's form today, so every byte outside what moves stays. Add no dependency, and change no
   feature of `serde_json`. Cargo unifies features across the workspace, so `preserve_order` would
   change the key order of every `serde_json::Value` in the core and the shell too. Do not rely on
   a feature this crate does not ask for, such as `raw_value`, which only the loader enables.

   It does three things. Run each from `engine/`.
   - `cargo run --release -p conduit-conformance --example pieces` moves what is left inline into
     the pieces:
     - each entry of `manifest.yaml`'s `cases`, if present, into the piece of its case's
       directory, in `id` order; a new directory gets a new piece;
     - each inline input branch and expectation property of `case.schema.json`, if present, into
       its file of step 3.

     An element that no piece holds is added. One that a piece holds in the same form is
     dropped. One that a piece holds in another form makes the tool fail and change nothing; its
     message names the entry's ID or the file, and `--merge`. With nothing left inline, it changes
     nothing and says so. Its first run is the split itself. A list that holds a placeholder and
     inline elements together, as a merge can leave it, is accepted.
   - `-- --merge ROOT` is a three-way merge of what is left inline. `ROOT` holds a copy, made
     before the split, of the tree the inline lists started from: the tool reads
     `ROOT/conformance/manifest.yaml` and `ROOT/conformance/case.schema.json`, and fails if that
     manifest has no `cases`. For each entry ID, input file and expectation file, compared with
     `ROOT`:
     - unchanged: dropped;
     - added: added, as above;
     - changed: written to its piece if the piece holds `ROOT`'s form, and dropped if the piece
       already holds the new form. Otherwise the tool fails: it changed on both sides;
     - an entry removed: removed from its piece if its case no longer exists and the piece holds
       `ROOT`'s form. If its case still exists, the tool fails, so that a short list can never
       empty the pieces. A piece left empty is deleted;
     - an input branch or expectation property removed: the tool fails. An order that retires a
       kind deletes its piece.

     The tool changes nothing unless the whole merge succeeds. The example passes it the case
     paths that exist. Step 10 uses this mode.
   - `-- --compare ROOT` is the proof of step 9. It compares this tree with the tree at `ROOT`,
     made before the split, and changes nothing.

   The unit tests show that:
   - a small manifest splits into the expected pieces, and its other members keep their bytes;
   - a second run changes nothing;
   - an entry with a new ID joins its piece in `id` order, and one in a new directory makes a new
     piece;
   - an inline entry or schema element that differs from its piece fails without `--merge`,
     changes nothing, and its message names `--merge`;
   - `--merge` applies an addition, a change, and the removal of an entry whose case is gone, and
     deletes a piece left empty. It fails on a change made on both sides, on the removal of an
     entry whose case exists, and on the removal of a schema element;
   - an absent `safety_rows` stays absent, and an empty one stays empty;
   - an entry with an unknown member fails;
   - the repository's `manifest.yaml` and `case.schema.json`, read and written through the
     order-keeping type, give their own bytes; a split schema differs from its input only at the
     placeholders;
   - the assembly appends a rule file to `allOf`;
   - the comparison fails on a changed, missing or extra entry, on a changed `s1_coverage` row,
     and on a changed input branch.

5. **The runner reads the pieces.** `paths()` (`lib.rs:55-65`) gathers every `index.yaml` as it
   gathers every `case.yaml`. The runner keeps its three checks, over the gathered entries:
   - every entry has an ID, and no ID appears twice in all the pieces together (`lib.rs:146-154`);
   - every case has an entry whose `case` is the case's path and whose `promises` equal the
     case's (`lib.rs:165-176`);
   - every entry names a case that exists (`lib.rs:203-205`).

   Where their messages say "coverage index", make them name the piece, and the ID where there
   is one. It adds these checks:
   - `manifest.yaml` has no `cases` member;
   - `case.schema.json` holds no inline input branch or expectation property;
   - a piece is an object whose only member is `cases`, a list that is not empty;
   - a case's entry is in the piece of the directory that holds the case's own directory;
   - entries are in `id` order.

   `validate_s1_coverage` (`lib.rs:222-284`) reads its rows from `manifest.yaml`. It looks up
   vectors in the gathered entries, where it now reads `manifest["cases"]` (`lib.rs:262-267`).
   Every check in it stays. Its unit test (`lib.rs:1015-1046`) keeps every assertion; only the
   shape of its input changes.

   Today only `validate_s1_coverage` (`lib.rs:145`) and the check of the index's IDs
   (`lib.rs:146-154`) run before the cases. The checks of each case (`lib.rs:157-181`) run inside
   the loop that runs the cases (`lib.rs:156-202`), and `ids != inventory_ids` (`lib.rs:203-205`)
   runs after them all. Move the checks of each case, and `lib.rs:203-205`, into a first pass
   over every case, even with an ID prefix. Then run the cases that match (step 7). An invalid
   tree now fails before any case runs.

   Each new failure names the file, the entry's ID where there is one, and what would fix it.
   When `manifest.yaml` still has `cases`, the message names both causes. For example:
   "conformance/manifest.yaml still has `cases`. An entry belongs in the index.yaml of its case's
   directory (conformance/README.md, "The coverage index and the case schema in pieces"). From
   engine/, `cargo run --release -p conduit-conformance --example pieces` moves new entries there;
   a branch made before the split and rebased after it needs `-- --merge ROOT`, as that section
   says." This follows the rule of [ruling 52](../proposals/ruling-52/README.md), point 2, for
   the runner's own callers, the workers.

6. **The step and expectation code in modules.** Move the code. Do not change what it does, or
   any message it gives.
   - The arms of the input dispatch (`lib.rs:462-737`) go into `src/steps/`. Each arm is one
     file, grouping kinds as the arms group them today. The dispatch in `src/steps/mod.rs` has one
     line per arm.
   - The checks that every step's result passes go into `src/checks.rs` (`lib.rs:738-844`):
     evidence formats, decisions' promises, `record.v1`, record types, the retention of records,
     `records::transition`, `condition.v1` and `diagnostic.v1`.
   - The expectations (`lib.rs:848-1003`) and `projection` (`lib.rs:317-350`) go into
     `src/expect.rs`.

   After the move, a later order adds an input kind with:
   - a new file in `src/steps/`;
   - one line in the dispatch;
   - one input piece, and a rule file if the kind requires expectations.

   It adds an expectation with code in `src/expect.rs` and one expectation piece.
   `validate_s1_coverage` may move to `src/pieces.rs`.

   Leave `src/access.rs`, `src/packages.rs`, `src/records.rs` and `src/schemas.rs` as they are:
   order 041b changes `records.rs`. They reach `lib.rs` through `use super::*`, and must compile
   unchanged:
   - `schemas.rs`'s test module `regression_vectors` (lines 362-398) calls
     `run_case(&path, &case, &spec, &context, &known)` and uses `Case` and `read_value`; its test
     at lines 331-335 uses `read_value` and `expand`;
   - `packages.rs` uses `read_value` (lines 77 and 189);
   - all four use the `Result` alias and names from `lib.rs`'s `use` lines.

   So `run_case` keeps its name and signature. `Case`, `read_value`, `expand`, `Result` and those
   imported names stay reachable from `lib.rs`, defined or re-exported there.

7. **The vectors in parallel.** `run_matching` runs the cases that match on threads of
   `std::thread::scope`.
   - The number of threads is the positive integer in `CONDUIT_VECTOR_THREADS`, or
     `std::thread::available_parallelism()` without it. Any other value fails, and the message
     names the variable. `1` runs the cases one after another.
   - `run_suite` and `run_matching` keep their signatures. A new function takes the number of
     threads, so that a test can set it without changing the environment.
   - Reports and failures keep today's order: the sorted order of the case paths (`lib.rs:76`).
     The executable's output is the same bytes whatever the threads and their timing.
   - A panic in one case still fails the run.
   - Threads only read shared values. If one is not `Sync`, give each thread its own copy and
     say so. Do not change the loader or the core.
   - If a case overflows a thread's stack, set the size with `std::thread::Builder::stack_size`,
     and say so.
   - Add a test that runs the cases of one ID prefix, at least ten cases, with one thread and with
     four. It must find the same reports, field by field.

8. **Documents and checkers.**
   - `conformance/README.md`:
     - lines 5-6: the case schema is `case.schema.json` with the pieces under `case-schema/`, and
       later orders extend it as the new section says;
     - lines 13-14: the coverage index is now the pieces, and `manifest.yaml` keeps the S1
       coverage rows;
     - lines 445-448: the `safety_rows` tags are in the pieces;
     - line 503: "manifest attributions" becomes "coverage-index attributions";
     - "Running" (line 103): `CONDUIT_VECTOR_THREADS`, the tool and its flags;
     - a new section, "The coverage index and the case schema in pieces". It says where an order
       adds a case's entry: the piece of its directory, in `id` order; a new directory gets a new
       piece. It says where an order adds an input kind, an expectation and a rule: a new kind's
       rule is a new file under `case-schema/rules/`, and an inline rule's list is not extended.
       The one exception is the last rule's list of kinds exempt from `data.target`. It says how
       the runner assembles the case schema, and gives step 10's procedures.
   - `spec/README.md`, two places only. Change nothing else under `spec/`.
     - Lines 7-8: remove the hand-written count, "There are 413 portable vectors". Every build
       order edits it today. Make the sentence read: "Portable vectors under conformance/ test
       slices of promises, and the manifest records the coverage and its gaps; the runner reports
       how many there are." Keep its two links, on "conformance/" and "manifest".
     - Line 193: replace "lists the cases and the reasons for missing S1 coverage." with "lists
       each S1 promise's vectors, or the reason it has none." Line 192 and its link stay.
   - `tools/set_names.py`, line 73 only: add `CONDUIT_VECTOR_THREADS` to `BUILD_TEST_VARIABLES`,
     as `CONDUIT_TEST_NPM` is. Lines 249-254 report every other `CONDUIT_` name under `engine/`
     and `conformance/`, so without this `set_names.py --check` fails.
   - `tools/check_docs.py`, lines 209 and 217-226: check the `case` paths of every piece, as it
     now checks those of `manifest.yaml`. Keep scanning `manifest.yaml` for path mentions. Extend
     its test (`tools/tests/test_condensation_checks.py`, lines 148-167) with a piece that holds
     a missing case path.
   - `tools/migrate_041a.py`, line 399: its `--declare` mode reads the case IDs from
     `manifest.yaml`'s `cases`. Make it read them from the pieces.

   In the crate's code and tests, write a path under `conformance/` relative to it, as
   `language/registration/e1/index.yaml`, or build it with `join`. `tools/check_docs.py` (lines
   210-239) reports a literal root-relative path that does not exist, such as
   `conformance/x/index.yaml` in a test or `conformance/case-schema/rules` before any rule file
   exists.

9. **The proof that nothing was lost or changed.**
   - From `engine/`, run `cargo run --release -p conduit-conformance --example pieces -- --compare target/s0a/base`.
     It checks that:
     - every entry of BASE's `cases` is in the piece of its case's directory, once;
     - each entry's canonical bytes equal those of BASE's entry. The canonical bytes are
       `serde_json::to_vec` of the parsed entry, whose keys `serde_json` sorts;
     - the pieces hold no other entry;
     - `status`, `s1_coverage` and `s1_coverage_source` are equal in the same sense;
     - the assembled case schema equals BASE's, as step 3 says.

     It ends with one line of counts: pieces; entries; entries with tags, with an empty list and
     without `safety_rows`; `s1_coverage` rows; input branches; and expectation properties. At
     cfd7b23 these are 18 pieces, 413 entries (264, 64 and 85), 308 rows, 21 branches and 20
     properties.
   - `git diff --numstat BASE -- conformance/manifest.yaml` shows 0 lines added: the file only
     lost its `cases` lines (lines 3-4714 at cfd7b23).
   - `git diff BASE -- conformance/case.schema.json` adds only the placeholder lines.
   - From `engine/`, the runner's output equals BASE's, byte for byte:
     - `cargo run --release -p conduit-conformance > target/s0a/runner-after.txt`, then
       `cmp target/s0a/runner-before.txt target/s0a/runner-after.txt`;
     - the same with `CONDUIT_VECTOR_THREADS=1`, into `target/s0a/runner-after-1.txt`.
   - Time the vectors test and the executable as step 1 did: with the default threads, with
     `CONDUIT_VECTOR_THREADS=1` and with `CONDUIT_VECTOR_THREADS=4`. The vectors test is
     `cargo nextest run --release -p conduit-conformance --test vectors`.

10. **On a rebase.**
    - **This branch, rebased over an order that merged first and changed `manifest.yaml`.**
      Order 041b does: its tree today adds 17 entries and changes three. The admin takes the new base's
      `conformance/manifest.yaml` and `conformance/case.schema.json` whole. The next round:
      1. from `engine/`, runs
         `cargo run --release -p conduit-conformance --example pieces -- --merge target/s0a/base`,
         while that directory still holds the old BASE;
      2. then redoes step 1 for the new BASE, which the round's prompt names;
      3. then reruns every check.
    - **A branch made before the split, rebased after it.** Write this procedure into the new
      section of `conformance/README.md`:
      1. Before the rebase, copy `conformance/manifest.yaml` and `conformance/case.schema.json`
         from the commit the branch started from (`git merge-base` of the branch and `main`)
         into ignored build output, under one directory, at those paths.
      2. At a conflict in either file, keep the branch's side of each conflicting hunk (stage 3
         in a rebase), and git's merge elsewhere: run `git merge-file --theirs` over the file's
         three index stages. The `s1_coverage` rows merge as ordinary text.
      3. Run the tool with `-- --merge` on that directory, then the runner.

    Never merge an entry, a piece or a schema branch by hand.

Work in this order.

## Scope

Change:

- `engine/crates/conformance/`:
  - `src/lib.rs`;
  - the new `src/pieces.rs`, `src/steps/`, `src/checks.rs` and `src/expect.rs`;
  - the new `examples/pieces.rs`;
  - the crate's tests;
- in `conformance/`, through the tool only:
  - `manifest.yaml`, losing its `cases` member;
  - the new `index.yaml` pieces;
  - `case.schema.json`;
  - the new `case-schema/inputs/` and `case-schema/expect/`;
- `conformance/README.md`;
- `spec/README.md`, lines 7-8 and 193 only;
- `tools/set_names.py`, line 73 only;
- `tools/check_docs.py`, `tools/tests/test_condensation_checks.py` and `tools/migrate_041a.py`.

Do not change:

- any `case.yaml` or fixture. This order declares no vector change, and writes no declaration
  file;
- any entry's `id`, `case`, `promises` or `safety_rows`;
- any `s1_coverage` row, `conformance/s1-promises.yaml` or `conformance/s1-safety-cases.md`;
- any `allOf` rule, and write no file under `case-schema/rules/`;
- the core, the loader, the shell or `engine/examples/`;
- the runner's `src/access.rs`, `src/packages.rs`, `src/records.rs`, `src/schemas.rs` or the
  output of `src/main.rs`;
- any `Cargo.toml`, dependency or feature, or `engine/Cargo.lock`;
- `engine/.config/` or `.github/`;
- `IN_FORCE.md`, `docs/AUDIT.md`, the rest of `spec/`, the rest of `tools/set_names.py`, or a
  record.

Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/check_private_paths.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py BASE`, with the interpreter that has PyYAML. Its last line is
  `Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.`
- `git diff --numstat BASE -- conformance/manifest.yaml`, which shows 0 lines added
- `git diff BASE -- conformance/case.schema.json`, which adds only the placeholder lines
- `git diff --check`

In `engine/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- the tool, run a second time: it reports nothing to move, and `git status --short` is the same
  before and after it;
- the tool's `--compare target/s0a/base`, whose last line gives the counts of BASE's files. At
  cfd7b23 they are those of step 9;
- both `cmp` commands of step 9, silent;
- the full suite: `cargo nextest run --release`, then `cargo test --release --doc`.

In `experiments/011-short-form-spike/`: `cargo test --release`.

Report:

- BASE's hash;
- the counts line of `--compare`;
- the placeholder lines that `case.schema.json` gained;
- the times of steps 1 and 9, in `summary`, as numbers only. Give the number of threads of each
  run, and what `available_parallelism` gave. Name no local path and no log file;
- whether the vectors test should reserve threads under nextest (`threads-required` in
  `engine/.config/nextest.toml`), and whether the admin's dispatch should set
  `CONDUIT_VECTOR_THREADS` for orders that run side by side, with the times behind it. Both are
  outside this order's scope: report them only;
- the totals passed and failed.

Your report's `vectors` is empty: this order adds none. Tests that bind loopback ports may fail
in your sandbox. If they do, say so, and the admin reruns them. Do not stage anything.
