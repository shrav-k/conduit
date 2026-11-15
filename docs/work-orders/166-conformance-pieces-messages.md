# Work order 166: the conformance pieces' last messages: the ROOT's repair, one walk of case-schema/, one message for an emptied region

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none:
every crate it builds is in Cargo's cache, and none of its tests binds a port. Run one cargo
command at a time, and never print a whole large file or log. Never run `env`, `printenv`, `set`
or `export -p`, never print an environment variable's value, and never read or print a key, a
token or any credential.

## What and why

[Order 071](071-conformance-manifest-pieces.md) split the coverage index and the case schema into
pieces, with the tool `examples/pieces.rs` over the module `src/pieces.rs` of the conformance
crate. [Its Record](071-conformance-manifest-pieces.md#round-3-on-the-mac-and-the-merge) leaves
three shoulds of its second fresh Opus reader to a follow-up order. The reader of the admin's last
fix before the merge found a fourth, which the Record does not list. All four are still open on
`main` at `632881df`. Below, `src/`, `tests/` and `examples/` are those of
`engine/crates/conformance/`, and a bare line number is in `src/pieces.rs`.

- **A fault in the merge ROOT reads as the tree's, or carries the tree's repair.** `split` reads
  the ROOT's two files at lines 564-581. A missing or unparsable ROOT file comes back from `read`
  (lines 220-227) as a bare `case.schema.json: missing file` or `manifest.yaml: ...`, and a fault
  in a ROOT entry, from `inline_entries` (lines 344-361), as `manifest.yaml: ...`: neither says it
  is the ROOT's. A ROOT manifest with no cases gets `merge ROOT manifest.yaml has no cases; use a
  tree from before the split` (line 570), which says neither how to find that tree nor where the
  procedure is. A fault in the ROOT schema's regions is prefixed `merge ROOT's
  conformance/case.schema.json, not the tree's:` and then gives the tree's repair from
  `inline_schema` (lines 382-427): that the `oneOf` "must be a list holding the false
  placeholder", and that an element is retired by deleting its piece. A ROOT from before the
  split holds neither a placeholder nor a piece. No test asserts the prefix, so removing it fails
  nothing. The one ROOT message a test reaches is checked only for `has no cases` (line 1075).
- **case-schema/ is walked in two places.** The runner (`src/lib.rs:133-162`, in
  `run_matching_with_threads`) and the tool (`read_tree` in `examples/pieces.rs:8-58`, lines 36-43
  for `case-schema/`) each walk it, skip names that begin with a dot and call
  `schema_piece_path`. Only that predicate is tested
  (`only_json_files_directly_in_the_three_folders_are_pieces`, line 1327): deleting either
  caller's call fails no test.
- **The runner's message for a wrong input placeholder is not asserted.** `assemble` (lines
  443-449) answers `properties.steps.items.properties.input.oneOf needs the single false
  placeholder; ...`. No test reaches it:
  `a_missing_or_mistyped_region_names_the_file_and_the_repair` (line 1303) stops in
  `inline_schema` first. A reverted wording fails nothing.
- **An emptied input `oneOf` gets one of two messages.** Without `--merge`, `oneOf: []` with
  nothing else inline meets the older check (lines 646-652), whose message has no `conformance/`
  prefix, no pointer to the README and no `-- --merge ROOT`. The same `[]` beside an inline
  expectation passes that check and meets the newer one (lines 657-663), built with `repair` and
  `INPUTS_FORM`. Both refuse, but the words a caller reads depend on what else is inline.

[Ruling 52](../proposals/ruling-52/README.md), point 2, asks every diagnostic to say what
happened and why, in full: here, whose file is at fault and what repairs it. No promise governs
these messages; as order 071 says, the runner's form is the admin's to decide. This order changes
no vector, case, piece, manifest or schema file.

This order is dispatched after order 163 merges, which also changes this crate. Line numbers are
those of `main` at `632881df`; if a line has moved, find the place by its words.

## The decisions it applies

1. **A ROOT's fault is named as the ROOT's, with the ROOT's repair** (the first of the Record's
   leftover shoulds). In `split`, every error from reading or checking the ROOT's `manifest.yaml`
   and `case.schema.json` (from `read`, from `inline_entries` and its "has no cases", and from
   `inline_schema`) goes through one function and reads:

   ```text
   merge ROOT's conformance/<file>, not the tree's: <problem>. A merge ROOT holds conformance/manifest.yaml and conformance/case.schema.json from the commit the branch started from (git merge-base HEAD main), before the split: the manifest with its cases, and the schema with its input branches and expectation properties inline. See conformance/README.md, "The coverage index and the case schema in pieces".
   ```

   `<file>` is `manifest.yaml` or `case.schema.json`. `<problem>` is the fault alone, without the
   file name in front of it (`manifest.yaml: `, `case.schema.json: ` or
   `conformance/case.schema.json: `) and without its repair. It is `missing file`; the parser's
   error; for the manifest, `has no cases, so it is from after the split`, or an entry's fault
   as `inline_entries` words it; for the schema, a fault as `inline_schema` words it, such as
   `properties.steps.items.properties.input.oneOf: missing oneOf` or `inline input branch 2 of
   the input oneOf: input branch needs a kind const or enum`. The repair is everything after the
   fault: `INPUTS_FORM`, `EXPECT_FORM`, the advice after a bad branch or property ("give it ...",
   "rename it ...") and `repair`'s closing sentences. None of it goes into a ROOT's message; for
   example, `inline_schema` may return the fault and its repair separately. The tree's own
   messages from these three functions do not change, and neither do `compare`'s.

   A unit test in `src/pieces.rs`, `a_faulty_merge_root_names_the_root_and_its_repair`, runs
   `split` on `split_fixture()` against each of these ROOTs, made from `fixture()`: without
   `case.schema.json`; with `{not json` as `case.schema.json`; with a schema whose `input` is
   `{}`; with `{}` added as an inline input branch; without `manifest.yaml`; and
   `split_fixture()` itself, whose manifest has no `cases`. Each run fails and changes nothing.
   Its message begins with `merge ROOT's conformance/` and the file's name, holds `git merge-base
   HEAD main`, and holds none of `placeholder`, `its piece` and `New inline elements`. For the
   ROOT whose `input` is `{}`, the message begins `merge ROOT's conformance/case.schema.json, not
   the tree's: properties.steps.items.properties.input.oneOf: missing oneOf. A merge ROOT holds`.

2. **One walk of case-schema/** (the second). A new
   `pub fn read_schema_pieces(conformance: &Path) -> Result<Files>` in `src/pieces.rs` is the one
   walk of `case-schema/`: the only code that reads a file under it or checks one with
   `schema_piece_path`. The runner's `paths()` (`src/lib.rs:59-72`), which looks for `case.yaml`
   and `index.yaml` everywhere, does not change. Given the conformance folder,
   `read_schema_pieces` returns each piece's bytes, keyed by its path relative to that folder
   with `/` between names, such as `case-schema/inputs/load.json`. Without `case-schema/` it
   returns an empty map. It reads no file or folder whose name begins with a dot, and nothing
   inside such a folder. Every other file must pass `schema_piece_path`, whose error it returns
   unchanged; an I/O error names the folder or file. It replaces three loops:
   - the runner's: `src/lib.rs:133-162` becomes one call that adds its result to `metadata`;
   - the tool's: in `read_tree`, `visit` skips the folder `case-schema` directly inside the
     conformance folder, as it skips a dot name (lines 19-24), so it reads and checks nothing
     under it. Lines 36-38 and the `case-schema/` clause of line 42 go, and after `visit`,
     `read_tree` adds the call's result to `files`;
   - the loop in `baseline()` in `tests/pieces.rs` (lines 29-35).

   The module's first line says that this function is its one reader of files. Its test is an
   integration test, since the module's unit tests write no file (order 071, step 4).
   `tests/pieces.rs` gains `one_walk_reads_pieces_and_refuses_other_files`. It works in a folder
   `conduit-pieces-walk-<process ID>` under `std::env::temp_dir()`, removed on drop as `Stages`
   is. It writes `case-schema/inputs/load.json`, `case-schema/inputs/.DS_Store` and
   `case-schema/.hidden/notes.txt`; the walk returns `case-schema/inputs/load.json` alone, with
   its bytes. It then adds `case-schema/inputs/load.json.orig`, `case-schema/rules/sub/a.json` and
   `case-schema/README.md`, one at a time, removing each before the next. Each time the walk
   fails, and its message holds that path and `rename or move it`. A folder with no
   `case-schema/` gives an empty map.

   In `conformance/README.md`, section "The coverage index and the case schema in pieces" (lines
   156-157), "Every other file under `case-schema/`, other than one whose name begins with a dot,
   is an error, never skipped." becomes "Every other file under `case-schema/` is an error, never
   skipped; nothing whose name begins with a dot is read, nor anything inside a folder so named."
   Rewrap only those lines, with no line over 89 characters, the width of the lines before them.
   They may become three lines. No other line changes.

3. **The input placeholder's message is asserted** (the third). A unit test,
   `assembly_names_the_input_placeholder_and_its_repair`, sets the input `oneOf` of
   `split_fixture()` to `[]`, then to `[false, false]`. Each time, the error of `assemble` holds
   `properties.steps.items.properties.input.oneOf needs the single false placeholder`, `deleting
   its piece` and `-- --merge ROOT`. The message does not change.

4. **One message for an emptied input `oneOf`** (the reader of the last fix). One helper builds
   it, and both the older check (lines 646-652) and the newer one (lines 657-663) return it:

   `repair(SCHEMA, "", &format!("{INPUTS_PLACE} holds no input branch and is not the single false placeholder; --merge does not restore input branches removed inline; {INPUTS_FORM}"))`

   The words saying that `--merge` does not help stay, because the older check also runs with
   `--merge`. When each check fires does not change. A unit test,
   `an_emptied_input_list_gives_one_message`, sets the input `oneOf` of `split_fixture()` to `[]`
   and runs `split` three ways: with nothing else inline; beside an inline expectation property
   `added`; and with nothing else inline and `fixture()` as ROOT. All three fail with one and the
   same message, which holds `holds no input branch`, `--merge does not restore` and `deleting its
   piece`, and none changes anything. On `main`, the first and third give the older message and
   the second gives the newer, so the test fails there.

5. **What stays.** Every existing test passes unchanged, except `baseline()` in
   `tests/pieces.rs`, as decision 2 says. No `Cargo.toml`, dependency or feature changes. In the
   crate's code and tests, write no literal path that begins with `conformance/` and does not
   exist, such as one ending in `load.json.orig`: `tools/check_docs.py` reports it. Write such a
   path relative to the conformance folder, or build it with `join`.

## Scope

Change only:

- `engine/crates/conformance/src/pieces.rs`;
- `engine/crates/conformance/tests/pieces.rs`;
- in `engine/crates/conformance/src/lib.rs`, the walk that decision 2 replaces (lines 133-162),
  and nothing else;
- in `engine/crates/conformance/examples/pieces.rs`, only `read_tree` (lines 8-58): the skip of
  `case-schema/` in `visit`, the removal of lines 36-38 and of line 42's `case-schema/` clause,
  and the call that decision 2 adds;
- in `conformance/README.md`, only lines 156-157, as decision 2 says.

## Done when

From `engine/`, each passes, one at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-conformance --lib pieces::tests::
cargo test --release -p conduit-conformance --test pieces
cargo run --release -p conduit-conformance --example pieces
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance
```

The third command runs the three new unit tests, among the others, and they pass. The fourth runs the new integration test. The fifth prints `Nothing to move.`, and `git status --short` is the same before and after it. The last runs every vector.

Show that the tests of decisions 1 and 4 fail without their decisions. For each, undo that decision's code in `src/pieces.rs` but keep its test, run the test, restore the code, and report the message of the assertion that failed.

From the repository root, each of these prints nothing:

```bash
git grep -n -e schema_piece_path -e case-schema -- engine/crates/conformance/src/lib.rs
git grep -n schema_piece_path -- engine/crates/conformance/examples engine/crates/conformance/tests
git grep -n read_dir -- engine/crates/conformance/tests/pieces.rs
git grep -n 'schema elements removed inline' -- engine/crates/conformance
```

Each of these prints 1, for the one call:

```bash
grep -c read_schema_pieces engine/crates/conformance/src/lib.rs
grep -c read_schema_pieces engine/crates/conformance/examples/pieces.rs
```

And each of these passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

`git status --short` lists only the five files in scope. Do not stage anything. Report each choice you made yourself, and the last line of every check. Then the admin shows on the pull request that the engine build, the four engine shards and the doctests pass, and a fresh Opus reader reads the order and its diff before it merges ([ruling 79](../proposals/ruling-79/README.md), point 2).
