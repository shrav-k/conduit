# Work order 163: A24 at entry: a new promise lands with a case that can fail

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order needs no network access: its tests bind
no port, and every crate comes from Cargo's cache. Run one cargo command at a time, and never print
a whole large file or log: the runner's output is long, so use `wc`, `tail -1` or `cmp`. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 18 of [its orders](../proposals/full-review/orders.md))
found that nothing ties new text in `spec/` to a check. Of the 64 promises added since vectors
exist, one, CTR-014, entered with a case that cites it. The four orders that wrote only text added
60 ([FR-471](../proposals/full-review/findings/G2-size-brake.md#fr-471) and
[FR-473](../proposals/full-review/findings/G2-size-brake.md#fr-473), as their verifier corrected
them). Action item A24 in [the tracker](../AUDIT.md), open since the plan audit, reads "a new ruling
carries a vector that can fail". Its done-when is that the contributor guide states it.

The verifier's correction sets the form. If a promise had to wait for a passing check before it
entered `spec/`, ruled text would be held out of `spec/`. A tag can also cite a promise in a case
that asserts nothing. So the text merges with a case that cites the promise and can fail. Until
the build passes the case, it is listed as an expected failure, in the way test262 adds tests
before engines ship the feature. The coordinator adopted this as D40 and F128 of
[the decisions](../proposals/full-review/decisions.md), logged for the owner to strike. It changes
no behavior of the language and caps nothing. [Ruling 78](../proposals/ruling-78/README.md#the-decisions),
point 3.1, closed v0's list of features, and that is the brake on what enters v0. This rule only
makes each promise cost a case.

Today the runner has no such list: every case that fails fails the whole run
(`engine/crates/conformance/src/lib.rs:329-354`). This order adds the list. The runner reports a
listed case that fails apart from the passes and never as a pass. The order also states the rule
in `CONTRIBUTING.md`. The review's orders 19 (SO-A) and 21 (SO-B) will land their new and changed
cases through the list, and its order 22 removes SO-B's entries once the build passes them.

## The decisions it applies

1. **The list.** Create `conformance/expected-failures.yaml`. Write it as serde_json's pretty
   printer writes it, plus a final newline, as the coverage-index pieces are written. Its form:
   - It holds `{"cases": []}` today. The file always exists.
   - Each entry is an object with exactly two members, in this order. `id` names a case in the
     tree. `reason` says what the build lacks and which order is to make the case pass. Both are
     nonempty strings.
   - Entries are sorted by `id` in byte order, and no ID appears twice.
2. **The runner checks the whole list before any case runs.** Add a private module,
   `src/expected.rs`, with a pure function that takes the file's value and the set of every case
   ID and returns a map from ID to reason. `run_matching_with_threads` reads the file with
   `read_value`. It does so after the check that every index entry names a case (`lib.rs:283-292`),
   once `ids` holds every case ID, and before it selects the matching cases (`:293`), so the whole
   list is checked whatever the prefix. Each of these fails the run:
   - a missing or unreadable file;
   - a value that is not an object whose one member is `cases`, or a `cases` that is not a list;
   - an entry that is not an object with exactly `id` and `reason` as nonempty strings;
   - a duplicate ID, or IDs out of byte order;
   - an ID that names no case.

   Each message begins `conformance/expected-failures.yaml: `, names the entry's ID where there is
   one, and says what would fix it. For an ID that names no case, it says to remove the entry or
   restore the case. This follows `pieces::repair` and
   [ruling 52](../proposals/ruling-52/README.md), point 2, but leaves out `repair`'s hint about the
   pieces tool.
3. **Four outcomes.** In the loop over results (`lib.rs:329-346`):
   - a case that passes and is not listed gives a `CaseReport`, as now;
   - a case that fails and is listed gives an `ExpectedFailure` (decision 4) and never a
     `CaseReport`;
   - a case that fails and is not listed gives a failure line, as now;
   - a case that passes and is listed gives this failure line:
     `<id> [<promises>]: passes, but conformance/expected-failures.yaml lists it as an expected failure (<reason>); remove its entry`.

   The rules around the loop:
   - A panic still fails the whole run (`:326`), whether or not the case is listed.
   - The error at `:347-354` keeps its form, `<n> cases passed; <k> failed` and then the lines.
     When the run also had expected failures, its first line reads
     `<n> cases passed; <m> expected failures; <k> failed`.
   - "no cases matched the requested prefix" (`:355-357`) is returned only when the run has
     neither a pass nor an expected failure.
4. **The result reports expected failures apart.** In `lib.rs`, next to `CaseReport` (`:43-49`),
   add two public structs, each deriving `Debug` and `PartialEq`:
   - `ExpectedFailure`, with `id`, `promises: Vec<String>`, `reason` and `error`. `error` is the
     error that `run_case` returned, unchanged.
   - `RunReport`, with `passed: Vec<CaseReport>` and `expected_failures: Vec<ExpectedFailure>`.

   `run_suite`, `run_matching` and `run_matching_with_threads` return `Result<RunReport>` in place
   of `Result<Vec<CaseReport>>`. Their arguments do not change. Both lists keep the sorted
   case-path order. `CaseReport` does not change, and neither do `run_case` and its signature,
   since `schemas.rs`'s regression test calls it.
5. **The executable prints them apart.** `src/main.rs` prints, in this order:
   - `PASS` lines for `passed`, as now;
   - for each expected failure, the line `EXPECTED FAILURE <id> (<promises joined by ", ">): <reason>`,
     followed by its error, with each line of the error indented by two spaces;
   - `D19` lines for `passed` only, as now;
   - a last line: `<n> cases passed` when there is no expected failure, so that today's output
     stays the same to the byte, and `<n> cases passed; <m> expected failures` otherwise.

   The exit status stays 0 on `Ok` and 1 on `Err`.
6. **The crate's callers.** In `tests/vectors.rs`:
   - `all_portable_cases_pass` looks for each required ID among the IDs of `passed` and
     `expected_failures` together, so that a required case that a specification order lists
     still counts as present;
   - `one_and_four_threads_report_the_same_fields` asserts `passed.len() >= 10` and compares the
     two `RunReport`s whole.
7. **Tests.**
   - Unit tests in `src/expected.rs`:
     - `{"cases": []}` gives an empty map;
     - two sorted entries give both;
     - each failure in decision 2 other than a missing or unreadable file fails with a message
       that begins `conformance/expected-failures.yaml: `.
   - A new file, `tests/expected_failures.rs`, with three tests. Each test:
     - copies the repository's `spec/` and `conformance/` into a fresh directory under
       `std::env::temp_dir()`, named with the test's name and the process ID and removed on drop,
       as `tests/pieces.rs` does with `Stages` (lines 49-54 and 64-68);
     - writes the list it needs into the copy;
     - runs `run_matching_with_threads` on the copy, with the prefix `loader-leading-bom` and 1
       thread.

     To *break* the case means to change the copy of the case `loader-leading-bom`, at
     `language/loader/bom/case.yaml` under `conformance/`. Read it as JSON, assert that the value
     at `/steps/0/expect/documents/0/a` is `"text"`, and set it to `"changed by the test"`. Then
     write the case back with `serde_json::to_string_pretty` and a final newline.
     - `a_listed_failing_case_is_reported_apart`: break the case and list it with a reason. The
       library call:
       - returns `Ok`, with `passed` empty;
       - gives one expected failure: ID `loader-leading-bom`, promises `SUB-005` and `SCH-013`,
         the reason, and an error that begins `step 1`.

       Then run the executable, `env!("CARGO_BIN_EXE_conduit-conformance")`, with the copy's root
       and the prefix as its arguments. It must:
       - exit 0;
       - print no line that begins `PASS `;
       - print the line `EXPECTED FAILURE loader-leading-bom (SUB-005, SCH-013): <reason>`;
       - end with the line `0 cases passed; 1 expected failures`.
     - `an_unlisted_failure_fails_the_run`: break the case and write the empty list. The call
       returns `Err`, and the message contains `0 cases passed; 1 failed` and
       `loader-leading-bom [SUB-005, SCH-013]: step 1`.
     - `a_listed_case_that_passes_fails_the_run`: list the case without breaking it. The call
       returns `Err`, and the message contains `loader-leading-bom`, the reason and
       `remove its entry`.

     Build every path inside the copy with `join`, never as one literal path from the repository
     root, as order 071 asked. `tools/check_docs.py` reports any literal path under `conformance/`
     that does not exist.
8. **The rule, in `CONTRIBUTING.md`.** Add rule 8 under "Writing promises", after rule 7 (lines
   136-138). Wrap it as its neighbours are wrapped, and make `conformance/expected-failures.yaml`
   in it a link to the new section of decision 9 (target `conformance/README.md#expected-failures`).
   The text:

   > 8. A new promise enters `spec/` in the same change as a case under `conformance/` that cites
   > it and can fail: a step of the case asserts what the promise says, so that an engine that
   > broke the promise would fail the case. Where the first build does not pass the case yet, the
   > change lists the case's ID, with its reason, in `conformance/expected-failures.yaml`. The
   > runner reports a listed case apart and never as a pass, and fails the run when a listed case
   > passes, so the change that makes the build pass it removes the entry. A case that a change to
   > `spec/` rewrites, and that the build does not pass yet, is listed the same way. A
   > convention-layer safety promise also gets a named property in a model of experiment 012,
   > beside the case and never in its place, unless its row in that experiment's model-checking
   > account says why no model checks it (ruling 78, point 3.2). No order writes promises alone.
   > The rule holds for the coordinator's own additions and for every fix that adds a promise ID.
   > A change that adds no promise ID is exempt. The rule makes each promise cost a case, and it
   > limits nothing in number.
9. **`conformance/README.md`.**
   - In "Running" (lines 115-116), after the sentence that ends "mismatched expectation", add one
     sentence. It says that a case listed in `expected-failures.yaml` is reported apart, with its
     reason and error, and never as a pass, and it links the new section.
   - Add a section, "## Expected failures", between "The coverage index and the case schema in
     pieces" and "Reviewing vector changes" (line 202). It states:
     - what the file is for, with a link to `CONTRIBUTING.md`'s "Writing promises" (rule 8);
     - the file's form (decision 1);
     - the checks the runner makes before any case runs (decision 2);
     - the four outcomes and that a panic still fails the run (decision 3);
     - that the list excuses only a failure of the case's run: a listed case is still validated
       against the case schema and checked against its index entry and the known promises before
       any case runs, as every case is, and a fault there still fails the run;
     - the executable's lines (decision 5);
     - that a listed case keeps its coverage-index entry: its tags say what the case tests, not
       that the build passes it.

## Scope

Change only:

- `engine/crates/conformance/src/lib.rs`, `src/main.rs` and the new `src/expected.rs`;
- `engine/crates/conformance/tests/vectors.rs` and the new `tests/expected_failures.rs`;
- the new `conformance/expected-failures.yaml`;
- `conformance/README.md`, as decision 9 says;
- `CONTRIBUTING.md`, rule 8 only.

Do not change `spec/`, any case, piece, fixture or schema piece, `src/pieces.rs`, any `Cargo.toml`
or `engine/Cargo.lock`, `tools/`, `docs/AUDIT.md` or `IN_FORCE.md`. This order adds no checker of
which promises a change adds; report anything else that needs changing.

## Done when

Before you change anything, from `engine/`:

```bash
mkdir -p target
cargo run --release -p conduit-conformance > target/o163-before.txt
```

After the change, run each of these from `engine/`, one at a time. Each must pass:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-conformance --test expected_failures
cargo test --release -p conduit-conformance
cargo run --release -p conduit-conformance > target/o163-after.txt
cmp target/o163-before.txt target/o163-after.txt
```

The third command reports exactly 3 passed, the three new tests. `cmp` prints nothing.

Then show that the tests fail without the change:

1. In decision 3's loop, look each case up in an empty map in place of the list.
2. Run the third command again. It must report 2 failed and 1 passed: the failures are
   `a_listed_failing_case_is_reported_apart` and `a_listed_case_that_passes_fails_the_run`.
3. Restore the line and run the command again. It must report 3 passed.

Report the last line of each run.

From the repository root, each of these passes:

```bash
python3 tools/check_docs.py
python3 tools/set_names.py --check
python3 tools/check_private_paths.py
git diff --check
```

Do not stage anything. Report each choice you make as your own, and every check's last line.

`git diff --check` and `check_private_paths.py` read tracked files only. So the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. On
the pull request, the admin shows the engine build, the four engine shards and the doctests
passing. A fresh Codex verifier and a fresh read-only Opus reader read the result before it
merges ([ruling 79](../proposals/ruling-79/README.md), point 2).

## Record

Run on 2026-11-15 on the Mac from `main` `f37d76c0`: the worker (876 s) and a fresh Codex verifier
(245 s), which found nothing to change in one round. Its read-only sandbox kept it from rerunning
the three tests that copy the tree and the check that they fail without the change, so it read their
saved logs instead; it ran the four validator unit tests, the executable on the BOM case, the format
and whitespace checks and the docs, names and private-path checkers. The order added
`conformance/expected-failures.yaml`, empty today, and a private module, `src/expected.rs`, that
checks the whole list against every case ID before any case runs. The runner reports a listed case
that fails as an `ExpectedFailure`, apart from the passes and never as one, and fails the run when a
listed case passes; `run_suite`, `run_matching` and `run_matching_with_threads` now return a
`RunReport`. The executable prints an `EXPECTED FAILURE` line with the case's error indented beneath
it, and with the empty list its output is unchanged to the byte (`494 cases passed`).
`CONTRIBUTING.md` gains rule 8 under "Writing promises", and `conformance/README.md` a section,
"Expected failures". A fresh read-only Opus reader then verified the order (ruling 79, point 2). It
reran the crate's tests (lib 38, core_boundary 7, expected_failures 3, pieces 3, vectors 2), clippy,
the executable, the check that the new tests fail without the change (2 failed and 1 passed, then 3
passed once restored), the docs and private-path checkers and `git diff --check`, probed the list's
diagnostics on a scratch copy, and found no must. This fix applied its three shoulds. Each
diagnostic of the list now says what is wrong before the repair: the actual type of the root, of
`cases`, of an entry or of its `id` or `reason`, the members an object lacks or has unknown, an
empty `id` or `reason`, the earlier entry a duplicate repeats and the ID an out-of-order entry
follows. An entry without an ID is named by its position, as `entry 2: `. The unit tests assert each
message whole, and the README's sentence on the diagnostics says the same. The executable's test
checks that its last line is exactly `0 cases passed; 1 expected failures`. The "Running" paragraph
is re-wrapped to its width, with one short line where its long link is kept whole. A24's row in
`docs/AUDIT.md` still reads open; updating it was outside this order's scope.

A second fresh Opus reader read the coordinator's fixes (ruling 79, point 2): it reran the
conformance tests, clippy and fmt and found no must. Its two points were applied: a duplicate
named after a later entry is now tested, and this Record names the docs check.
