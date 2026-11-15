# Work order 153: the model-checking account, checked by a tool

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order needs no network access, and it runs no
cargo command. Never print a whole large file: read `G5.md`, the safety catalogue, the manifest and
the promise files with `grep`, `sed -n` or a script that prints counts. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 6 of [its orders](../proposals/full-review/orders.md)) found
that nothing ties a promise to the model run that holds it
([FR-449](../proposals/full-review/findings/G1a-model-checking-register.md#fr-449)). G5's account
is a paper table, and no test reads `spec/` against it. Since the text the models read,
specification orders have changed 49 of its 217 rows, 17 of them holds, and no row moved
([FR-448](../proposals/full-review/findings/G1a-model-checking-register.md#fr-448) as its verifier
corrected it; [FR-446](../proposals/full-review/findings/G1a-model-checking-register.md#fr-446) row
0). Reading four of the nine models, the review also found that 28 of their 33 holds rows rest on a
property that cannot fail or says less than the promise
([FR-456](../proposals/full-review/findings/G1b-model-properties.md#fr-456),
[FR-469](../proposals/full-review/findings/G1b-model-properties.md#fr-469)).

This order makes the account a data file with a row for every convention-layer promise, and marks
in it what changed and what was graded. It adds a check, run by CI, that fails when a promise has
no row or when a row stands on text its model did not read, so that a specification order cannot
leave the account stale (D29 and D31 of [the coordinator's decisions](../proposals/full-review/decisions.md)).
The readiness gate's G5 row is next assessed from the counts the check prints
([FR-453](../proposals/full-review/findings/G1a-model-checking-register.md#fr-453)), and the reruns,
MC2's first, record their results in the file.

## The decisions it applies

1. **One data file, with a row for each convention-layer promise.** Create
   `experiments/012-model-checking/account.yaml`: a YAML list with one mapping for each promise
   that has `layer: convention` in `spec/promises/`, `spec/conventions/` or `spec/interface/` (245
   today). G5's 217 rows come first, in G5's order; the other 28 follow, sorted by ID.
   - A row's keys are `id` and `class`, then `standing`, `was`, `model`, `read`, `digest`,
     `outside` and `note` as decisions 2 to 4 allow, and no others.
   - Each of G5's rows keeps its model cell, without backticks and with no `model` key where G5 has
     "—". It also keeps its note word for word; the only additions are the sentences that decisions
     5 and 6 append.
   - Two comment lines at the head of the file say that it is not binding, that it is G5's account,
     and that `tools/check_model_account.py` checks it.
   - Write the file with a script that you run and do not commit.
   ([FR-449](../proposals/full-review/findings/G1a-model-checking-register.md#fr-449) part 1.)
2. **A row's class comes from the safety catalogue.** In `docs/implementation/SAFETY_CATALOGUE.md`:
   - `safety` where the ID is a table row under "## Safety promises";
   - `not safety` where it is a table row under "## The others";
   - `uncatalogued` otherwise.

   Today that gives 217, 14 and 14. The 14 uncatalogued are ENR-001, ENR-002, ENR-004, ENR-006 to
   ENR-012, PKG-032, PND-008, PND-013 and PRV-030. Only a safety row has a `standing` and a `note`,
   and a safety row must have both. Classing the 14 is the review's order 41, not this one
   ([FR-448](../proposals/full-review/findings/G1a-model-checking-register.md#fr-448); D30).
3. **A closed list of standings** (D31;
   [FR-469](../proposals/full-review/findings/G1b-model-properties.md#fr-469) as its verifier
   advised):
   - The model-run standings, each with `model`, `read` and `digest` required:
     - `searched`: the property relates events of different steps, so an order of events or a
       fault that the search explores could refute it.
     - `restated`: restated at decision time; the property reads what the decision computed in the
       same step.
     - `holds, not graded`: held in its model and not yet graded. It is graded at that model's next
       run, where each mutant also runs with every interleaving fault switched off. A property that
       this single schedule refutes is a one-step check, and its rows become `restated` or
       `static`.
     - `contradiction: F`, `gap: F`, `ambiguity: F` and `property over-broad: F`, as G5 classes
       them. F is a model finding that matches `MC[0-9]+[a-z]?-[0-9]{3}`, such as `MC4-009`.
   - `text changed, not rerun`: the row had a model-run standing, and its promise's text has
     changed since `read`. `was` holds the standing it had; `model`, `read` and `digest` are
     required and stay as they were.
   - `static`: decided in one step, or held in its model only by a property that nothing the
     search varies can reach (FR-456's mark K). A vector or a contract test shows it, not a model.
     `outside` may list repository paths of evidence outside `conformance/manifest.yaml`. The
     manifest's own cases are counted from the manifest and never copied into the account.
   - `not modelled`: no property checks its safety slice; the note says why.
   - `left to MC9`: assigned to a model not yet written.

   `model` is optional on the last three standings. It is written as G5 writes it: `module::key`,
   with any further keys after commas (for example `gates::H, C, R`). `module` must name a file
   `experiments/012-model-checking/src/<module>.rs`.
4. **What each model read, and the digest.**
   - Every row with a model-run standing has `read: 5ed438b21a3a22c08b46b7bb9297f3cf6ef027f0`
     (order 041a). Orders 105 to 113, which built the nine models, were each written while that
     commit was the last to change `spec/`. The next such commit, `c8e9aa20` (order 038b),
     rewrote 25 promises after MC2 to MC5b had merged. The MC6, MC7a and MC7b models merged after
     it, and the finishing orders 114 and 115 were written after it, so those models may have read
     it. The older commit is kept for every model so that no change is missed: a row it flags
     wrongly is cleared by a rerun, never by moving `read`. This follows the corrections of
     FR-448's verifier and of
     [FR-446](../proposals/full-review/findings/G1a-model-checking-register.md#fr-446) row 0.
   - `digest` is the SHA-256, in lowercase hexadecimal, of the UTF-8 bytes of
     `json.dumps({"level": L, "observable": O, "then": T, "when": W}, sort_keys=True, ensure_ascii=False, separators=(",", ":"))`.
     Each value is the promise's field as PyYAML's `safe_load` gives it, or `null` where the field
     is absent, so a line folded differently changes nothing. The digest is taken from the
     promise's text at `read`, in whichever of the three directories the promise sat then. Level
     is included because FR-448's comparison includes it.
5. **The 33 rows already graded**
   ([FR-456](../proposals/full-review/findings/G1b-model-properties.md#fr-456) as its verifier
   corrected it; [FR-458](../proposals/full-review/findings/G1b-model-properties.md#fr-458) to
   [FR-461](../proposals/full-review/findings/G1b-model-properties.md#fr-461)). Append
   "Graded from holds by FR-NNN." to the note of each row this decision moves. FR-NNN is the
   finding for the row's module: FR-458 for records, FR-459 for gates, FR-460 for secrets and
   FR-461 for revisions. Any sentence given below follows it.
   - `searched`: PRV-026, PRV-028, SEC-012, SEC-017 and SEC-023.
     - PRV-028: "Restates the send guard; the takeover is excluded by current()'s oracle; proof
       loss and reordering are searched; order 081's clause on an existing run is not checked."
     - PRV-026 and SEC-023: "Searched for a Conduit open or closed; no filter."
   - `restated`: REV-002, SEC-010 and SEC-016.
   - `static`, each keeping its `model`: PRV-003, PRV-011, PRV-013, PRV-018, PRV-022, PRV-023,
     PRV-024, PRV-025, PRV-029, SEC-007, SEC-011, SEC-013, SEC-014, SEC-015, SEC-018, SEC-020,
     OBS-001, OBS-003, OBS-004, OBS-006, REC-001, REC-004, REC-005 and REV-011.
     - `outside` for three of them: PRV-023 `engine/crates/shell/tests/runner.rs`; PRV-024
       `engine/crates/conformance/src/records.rs`; PRV-029 `engine/crates/shell/src/tests.rs` and
       `engine/crates/shell/tests/changes.rs`.
     - PRV-025: "SSH-R's README states PRV-025 unmet until each Agent has a user of its own
       (ruling 77)."
   - `not modelled`: REV-012, with: "J states less than the slice: no property counts the changes
     stored per named revision, and by reading, the correct model applies two changes that name
     r0 (FR-461). MC2 revisions' rerun registers that property, expected to be refuted (the
     review's decision D32)."
   - The other 81 rows that G5 calls holds (operations 6, access 9, placement 11, lifecycle 37,
     controls 18) become `holds, not graded`, and their notes do not change. Do not grade them by
     reading.
6. **The seven rows of ruling 75 A1.** G5 gives OPS-008, OPS-012, OPS-014, OPS-018, OPS-021,
   OPS-024 and OPS-028 as "holds under ruling 75 A1; refuted as written: MC2-001". They become
   `contradiction: MC2-001`, with this appended to the note: "Held only under ruling 75 A1's
   fence, which ruling 78 point 6 withdrew."
   ([Ruling 78](../proposals/ruling-78/README.md#the-decisions), point 6.) Every other standing
   stays as G5 gives it.
7. **Changed text is marked, not hidden.** After decisions 5 and 6, every row with a model-run
   standing whose promise's present text has a different digest becomes `text changed, not
   rerun`, with `was` set to the standing it had; its note does not change. This applies to every
   model-run standing, not to holds alone, because a finding made against text that has since
   changed also waits for a rerun (FR-446 row 0). Expect these 29 rows: PKG-024, PKG-025,
   PKG-027, PLC-017, PLC-026, LIF-025, PLC-023, PLC-024, PRV-008, PRV-015, PRV-019, LIF-009,
   LIF-028, OPS-009, OPS-034, REV-010, PRV-007, PRV-010, PRV-014, PRV-021, LIF-036, PRV-028,
   LIF-019, LIF-024, OPS-014, OPS-018, OPS-024, REC-003 and REC-006. If yours differ, report
   which rows and why; never change a row to reach this list.
8. **The tool, `tools/check_model_account.py`.** It uses Python 3.9 or later, the standard
   library and PyYAML, and has `main(root, argv)` so that tests can run it on a temporary tree.
   - **What it reads.** The three spec directories: each YAML file directly inside them whose top
     level is a list, where each item with an `id` is a promise. Also the catalogue,
     `conformance/manifest.yaml` (`cases[].promises`) and `account.yaml`.
   - **Errors.** It reports each error as one line that names the promise:
     - a convention-layer promise with no row, a second row for one ID, or a row whose ID is not a
       convention-layer promise in `spec/`;
     - a class that the catalogue contradicts;
     - a standing outside decision 3's list, a key that decision 3 does not allow on that
       standing, a required key that is missing, a `was` that is not a model-run standing, a
       `model` whose module has no file, or an `outside` path that does not exist;
     - a row with a model-run standing whose `digest` differs from the digest of its promise's
       present text. The line says to rerun the model or to mark the row `text changed, not
       rerun`.
   - **`--history`.** With this flag it also recomputes each row's `digest` from the text at
     `read`, using `git ls-tree` and `git show`. It reports an error where the two differ or where
     the promise is absent at that commit.
   - **Output.** Before any error it prints: the rows by class; the uncatalogued IDs; the count of
     each standing present, each finding counted separately and `text changed, not rerun` broken
     down by `was`; and the number of static rows that no manifest case names and no `outside`
     path supports. Its last line is `Errors: N`. It exits 0 when N is 0 and 1 otherwise.
   - **Without PyYAML.** Run as a script without PyYAML, it runs itself again under
     `/usr/bin/python3`, as `check_schemas.py` does. If PyYAML is still missing, it says that
     nothing was checked and exits 2. Importing the module never runs it again.
9. **Its tests, `tools/tests/test_model_account.py`**, in the style of `test_private_paths.py`,
   each on a small temporary tree:
   - a clean tree passes with `Errors: 0`;
   - a changed `then` on a `holds, not graded` row fails, naming the row, and passes once the row
     is `text changed, not rerun` with its `was`;
   - the same words folded onto other lines leave the digest unchanged;
   - a missing row, a row for a promise not in `spec/`, a class the catalogue contradicts and an
     unknown standing each fail;
   - on a Git tree of two commits, `--history` passes when the digest is that of the text at
     `read`, and fails when it is that of the later text.

   The module raises `unittest.SkipTest` when PyYAML is missing, so that the suite still runs
   under an interpreter without it.
10. **Where it runs.**
    - In the `docs` job of `.github/workflows/checks.yml`, add one step after `check_rulings.py`,
      named and run as `python3 tools/check_model_account.py --history`, with
      `if: ${{ !cancelled() }}` like the steps beside it. The job's full-history checkout serves
      `--history`.
    - In AGENTS.md's Checks, add one line to the block, after the `check_roadmap.py` line,
      `python3 tools/check_model_account.py # the model-checking account has a row for each convention-layer promise, and none is stale`,
      with its comment aligned with the others.
    - Under "What each check needs" in AGENTS.md, add one bullet after the `check_private_paths.py`
      bullet: `check_model_account.py` needs Python 3 and PyYAML, and runs itself again under
      `/usr/bin/python3` when its `python3` lacks PyYAML, as `check_schemas.py` does. With
      `--history` it also needs Git and the history back to each row's `read`.
11. **G5.md points to the file.** In `experiments/012-model-checking/G5.md`:
    - Under its Status line, add one line: since work order 153 the rows are in
      a Markdown link to `account.yaml`, the file beside G5.md, which `tools/check_model_account.py` checks and whose counts it
      prints; the rest of the file is the account of 2026-11-11.
    - "## Counts" becomes "## Standings". Keep its first paragraph, amended only where it describes
      the old standings. Its standings table and the paragraph after it give way to decision 3's
      list. Keep its table of refuted or contested promises, introduced as of 2026-11-11.
    - "## The account" keeps its two paragraphs, amended only where they describe the table's
      columns. Its table gives way to one sentence pointing to `account.yaml`.
    - Nothing else in `G5.md` changes.

## Scope

Create `experiments/012-model-checking/account.yaml`, `tools/check_model_account.py` and
`tools/tests/test_model_account.py`. Change `experiments/012-model-checking/G5.md`, the `docs` job
of `.github/workflows/checks.yml`, and the Checks section of `AGENTS.md`. Change nothing else: not
`spec/`, the safety catalogue, the model crate, `IN_FORCE.md` or the roadmap.

## Done when

From the repository root, each of these passes:

```bash
python3 tools/check_model_account.py
/usr/bin/python3 tools/check_model_account.py --history
/usr/bin/python3 -m unittest discover -s tools/tests -p 'test_model_account.py'
/usr/bin/python3 -m unittest discover -s tools/tests
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

The new test file runs under `/usr/bin/python3` with no test skipped. The tool's counts should be:

- rows: 245 (safety 217, not safety 14, uncatalogued 14);
- standings: `searched` 4, `restated` 3, `holds, not graded` 71, `contradiction: MC2-001` 4,
  `contradiction: MC6-001` 1, `contradiction: MC6-002` 2, `gap: MC4-009` 1, `gap: MC7b-002` 1,
  `property over-broad: MC3-001` 1, `property over-broad: MC3-002` 1, `text changed, not rerun` 29,
  `static` 77, `not modelled` 12 and `left to MC9` 10;
- `text changed, not rerun` by `was`: `holds, not graded` 10, `searched` 1,
  `contradiction: MC2-001` 3, `contradiction: MC6-001` 1, `contradiction: MC6-002` 1,
  `gap: MC4-002` 2, `gap: MC4-009` 3, `gap: MC7b-002` 3, `ambiguity: MC4-001` 3 and
  `ambiguity: MC7b-001` 2;
- static rows with no case and no evidence outside the manifest: 24.

Also check that `/usr/bin/python3` with PyYAML loads `.github/workflows/checks.yml` and that its
`docs` job has the new step. If a test that this order did not add fails under
`/usr/bin/python3`, report it with its last line and do not change it.

Do not stage anything. In your report, give each choice you made as your own, any count that
differs from those above with its rows, the number of tests the new file ran, and every check's
last line.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the three new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`,
and shows the `docs` and `secrets` jobs passing on the pull request.
