# Work order 030: the checks and code dependencies the condensation needs

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access only to fetch crates. Never print a
whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Ruling 58 accepted a plan to condense the repository's historical documents
([the plan](../proposals/condensation/plan.md), its [critic](../proposals/condensation/critic.md),
[ruling 58](../proposals/ruling-58/README.md)). Removals come in later orders. This first order
removes nothing: it gives the checks the condensation relies on, and moves the two things that code
and tests read out of files the later orders remove. The plan's step 1 is this order, with the
critic's correction to it applied below.

**C0**, the commit every "last version" link of this pass points to, is
`f1cb473493fbb7b8667686d6fdf639ae96579ec3`.

## What to do

1. **`tools/check_schemas.py`, a new step** (the plan's E15, without its third check): every promise
   under `spec/promises/`, `spec/conventions/` and `spec/interface/` has exactly the seven fields,
   its layer and level take the allowed values, IDs are unique across the three, and no live ID is a
   retired ID named in a head comment's register of removed IDs or in `spec/README.md`'s list of
   promise files that rulings removed. The rules come from
   `docs/proposals/reaudit/CONSOLIDATION.md` lines 136 to 151. If current text fails, report each
   failure and do not fix it.
2. **`tools/check_docs.py`, permalinks.** A link of the form
   `https://github.com/shrav-k/conduit/(blob|tree)/<sha>/<path>#<anchor>` is checked: the path with
   `git cat-file -e <sha>:<path>`, the anchor against the headings of `git show <sha>:<path>`, using
   the same anchor rules as for local links. Outside a Git tree with that history, the check is
   skipped with a notice. Other external links stay unchecked.
3. **A path-mention check**, in `tools/check_docs.py` or a new step: repository-relative paths named
   in `tools/*.py`, `conformance/manifest.yaml` and `reference-model/` sources must exist. Report
   what fails today; fix only what this order's items 4 and 5 move.
4. **The frozen S1 set** (the plan's E14): `conformance/s1-promises.yaml` holds the 308 promise IDs
   of `docs/proposals/first-build/increment-2/plan.md` section 2, verbatim. Point
   `reference-model/crates/conformance/src/lib.rs` (around lines 145 to 147 and 206 to 260),
   `conformance/manifest.yaml` (around line 6883) and `conformance/README.md` (around line 399) at it.
   Remove the loader test `reference-model/crates/loader/tests/s1_drafts.rs`, or point it at
   `reference-model/examples/s1`; repoint `conformance/fixtures/s1/README.md` line 7.
5. **S1's proposed safety traces** (the plan's E36, as the critic corrected it): copy
   `docs/proposals/first-build/increment-2/plan.md` section 3 (around lines 515 to 594) verbatim to
   `conformance/s1-safety-cases.md`, headed as **the safety traces proposed for S1 in increment 2**.
   Do not say it defines the manifest's `safety_rows` tags: the manifest uses about 21 tag values, and
   only four are trace names of that section, so no one has yet decided what "safety vector" means.
   Add no check on `safety_rows`. Record the mismatch, with the counts, in your report as a finding;
   the admin carries it to the open items.

## Scope

Change: `tools/`, `conformance/`, `reference-model/`. Report anything else that needs changing.

## Done when

The new checks run and report; the S1 set and the safety traces are in `conformance/`, and nothing
reads them from `docs/proposals/`. From the repository root: `python3 tools/check_docs.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py` (its new step may report
failures in today's text: report them, and the step's own exit status is decided by the admin),
`python3 tools/check_roadmap.py` and `git diff --check`; in `reference-model/`,
`cargo test --release`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`. Report
the total passed and failed across all test binaries. Do not stage anything.
