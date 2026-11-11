# Work order 033: `reference-model/` becomes `engine/`

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has network access only to fetch
crates. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

[Ruling 64](../proposals/ruling-64/README.md), point 4: `reference-model/` is renamed `engine/`,
because [ruling 47](../proposals/ruling-47/README.md) makes v0 the reference engine, which that
directory holds. No code, promise, vector or crate name changes: only the directory's name, every
path that names it, and the live documents' word for it.

## What to do

1. **Move the directory** with `git mv reference-model engine`. Its crates, their names and the
   binary's name stay as they are. Delete nothing; `engine/target/` stays ignored.
2. **Every path that names it**, in every tracked file, becomes `engine/`: Markdown link targets,
   relative paths in YAML and TOML (for example `conformance/language/registration/live-agents/case.yaml`
   and `experiments/011-short-form-spike/Cargo.toml`), `.github/workflows/`, and the tools
   (`tools/set_names.py`'s default roots, `tools/check_docs.py`'s path-mention check and its
   pattern, `tools/check_schemas.py`'s text, `tools/tests/`). Code under the new `engine/` that
   names its own directory changes too.
3. **The live documents' word.** In `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `ROADMAP.md`,
   `docs/README.md`, `docs/OVERVIEW.md`, `docs/AUDIT.md` (its tracker prose, not its ruling log),
   `docs/guides/`, `docs/implementation/`, `conformance/` READMEs, `spec/README.md` and the new
   `engine/`'s own documents, text that calls this directory or its code "the reference model"
   says "the engine" (or "the reference engine" where ruling 47's sense matters). Where a sentence
   means ruling 46's pure core, keep "core". Keep the sense of every sentence.
4. **Records keep their words.** Under `docs/proposals/`, `docs/archive/`, `docs/work-orders/` and
   every experiment's dated results, change only link targets and paths that a check reads; leave
   their prose, link text and quotations as written. Never change an owner's quoted words or a
   ruling's decisions (ruling 64 itself says "`reference-model/` is renamed `engine/`": keep it).
5. **Find what is left.** After the edits, `git grep -n "reference-model"` and
   `git grep -n -i "reference model"` must show only lines in the records of item 4, or lines that
   speak of the old name on purpose. Report the count of each, and list every live-document line
   that remains with the reason.

## Scope

Change: everything item 1 to 4 names. Do not change a promise, a schema, a vector's expected
outcome, a fixture's bytes or a digest. If any digest or fixture would change, stop and report.

## Done when

From the repository root: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py`,
`python3 -m unittest discover -s tools/tests` and `git diff --check` pass; in `engine/`,
`cargo test --release`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`; in
`experiments/011-short-form-spike/`, `cargo test --release`. Report the total passed and failed;
tests that bind loopback ports may fail in your sandbox: say so, and the admin reruns them. Do not
stage anything.
