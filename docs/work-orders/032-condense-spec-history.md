# Work order 032: the specification without its change narrative

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access only to fetch crates. Never print a
whole large file: use `head`, `tail`, `grep` or a count.

## What and why

[Ruling 58](../proposals/ruling-58/README.md), point 6: the change narrative inside `spec/` (about
47k words in some 149 YAML head comments, with about 834 paths into proposals, and about 6.6k words
of history paragraphs in `spec/README.md`, `spec/core/README.md`, `spec/schemas/README.md`,
`spec/definitions.md` and `spec/schema-profile.md`) is condensed to what each file holds, its
standing, its standards, its open issues and its register of removed IDs; and specification files
carry no change narrative from now on. This is step 3 of [the condensation plan](../proposals/condensation/plan.md),
its extraction E34 (read both, and the plan's [critic](../proposals/condensation/critic.md)). No
promise changes.

**C0**, the commit every "last version" link of this pass points to, is
`f1cb473493fbb7b8667686d6fdf639ae96579ec3`.

## What to do

1. **`spec/README.md`**: its "History" section becomes "Where the reasons are": one line each linking
   the records that hold the reasons (the plan's E34 list), and `docs/proposals/ruling-log.md`
   written as a future link only if it exists when you finish (it may not; then link the ruling log
   in `docs/AUDIT.md`). The other history paragraphs of the five Markdown files above are condensed
   the same way.
2. **Every YAML head comment under `spec/`** keeps: the sentences describing current content; open
   issues; "Standards followed" where present (for example `providers.yaml`, `secrets.yaml`); and a
   **register of removed IDs**, one line each, "Removed: OLD → where it went (ruling NN)" or "OLD:
   nothing replaces it (ruling NN)", covering every promise and DL ID that the head names at C0 and
   that is no longer live, keeping every "not reused" statement. Add one standing line: "This file
   carries no change narrative; the reasons are in the records listed in spec/README.md, 'Where the
   reasons are'." Drop the rest of the narrative. A path into a proposal that survives becomes the C0
   permalink (`https://github.com/shrav-k/conduit/blob/<C0>/<path>`).
3. **Gaps, not deletions.** A dropped sentence that states a behavior no promise holds is reported as
   a finding (a specification gap), and kept until the admin decides.
4. **Gates, each run by script and reported**:
   - for every YAML file, `yaml.safe_load` before equals after (JSON files untouched);
   - every promise's text is unchanged, compared by script;
   - every ID named in a C0 head comment that is not a live ID is named in the new registers or in
     `spec/README.md`'s list of promise files that rulings removed, and the same for DL IDs;
   - "Standards followed" sentences are present;
   - the words before and after, per file and in total.
5. If any digest or fixture that the shell or the vectors read changes, stop and report: do not
   regenerate anything.

## Scope

Change: `spec/` comments and the five Markdown files named, only. Report anything else.

## Done when

The gates pass and are reported. From the repository root: `python3 tools/check_docs.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py` and `git diff --check`; in `reference-model/`,
`cargo test --release` (report the total passed and failed; tests that bind loopback ports may fail
in your sandbox: say so, and the admin reruns them). Do not stage anything.

## Record

Run on 2026-11-10: the worker in 30 minutes, then a fresh verifier in 7. The worker condensed 153
YAML head comments and the five Markdown files: `spec/` went from 219,540 words to 183,415, its head
comments from 47,053 to 19,907. Every YAML file loads to the same data, all 455 promises and 15 DL
records are unchanged, and the 74 retired IDs named at C0 stay named. It kept two passages of
`spec/promises/secrets.yaml`'s head that no promise holds, as the order asked: that a file store
ships in the standard library and a store reading the engine's own variables is not provided
(finding 032-G1), and what a store package's contract states (032-G2); they describe current
content and stay until a change to secrets writes them as promises. The verifier found that
`tools/check_schemas.py` did not read the new register form "OLD: nothing replaces it", so 20
retired IDs had lost their reuse check (032-G3), and one broken sentence in `spec/definitions.md`.
The admin fixed both, with a regression test, which brought the recognized retired IDs back to 72,
and reran the Rust suite outside the sandbox: 163 tests pass, 3 ignored.
