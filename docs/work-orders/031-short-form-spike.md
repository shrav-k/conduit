# Work order 031: a spike of the short form's translation, on S1

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access only to fetch crates. Never print a
whole large file: use `head`, `tail`, `grep` or a count.

## What and why

[Ruling 55](../proposals/ruling-55/README.md) made the short form part of the language: a sectioned
document, defined by numbered translation rules into today's canonical documents, which stay the
kernel, the wire and the stored form. Before any promise is written, this spike checks the claims
the [short-form investigation](../proposals/language-shape/short-form/synthesis.md) made on paper:
that S1, written in the short form, translates into documents that today's loader and core accept,
and how many lines an author writes. Its grammar is in that file's section "grammar" (around line
118), with ruling 55's decisions applied:

- grants refer to Conduit templates for reuse (`from: read`), and operation names stay fully
  qualified wherever they are written (ruling 25);
- a reference names its kind (ruling 19 is kept): write `to: {kind: Resource, name: shared}`;
- the frame is optional; the version comes from the manifest;
- names stay stable: a replacement retires the old object and makes the new one under the same name.

The Fleet family of rulings 59 to 62 does not exist in today's core, so this spike writes no
`fleet:` key; it records how many lines `fleet: s1` would add.

## What to do

1. **A new experiment**, `experiments/011-short-form-spike/`, in Rust (its own `Cargo.toml`,
   `Cargo.lock`, a pinned toolchain, ignored build output), depending by path on the loader and core
   crates of `reference-model/`. Its README states, before the results, what it must show and how
   each line is counted: non-blank lines an author writes, separators included, comments excluded,
   each command line that carries a value the author chose counted apart (the
   [measurement plan](../proposals/first-build/increment-4/measurement-plan.md)).
2. **A translator** from the sectioned document to canonical documents, implementing the translation
   table of the grammar section as amended above. It is a spike, not the reference implementation:
   keep it small, and say in the README what it leaves out.
3. **S1's states in the short form.** Write the author's files for S1 after each of T1, T2, T4, T5,
   T6 and T7 (the tasks are in `docs/implementation/FIRST_BUILD.md`), in two styles: flow style, as
   the synthesis wrote it, and idiomatic block YAML, one key per line. Use the shipped packages'
   addresses and digests from `conformance/fixtures/s1/`.
4. **Check each state**: translate it, and register the result on a fresh state through today's
   loader and core, as the conformance runner does for a submitted project. Each must be accepted.
   For T5 and T6, also check that the translation of the new state differs from the old only by the
   objects a stable-name replacement would retire and make.
5. **Count**, per task and per style, the lines an author writes to go from the previous state, and
   compare with the baseline (33, 16, 0, 8, 3, 2, 2) and the coordinator's run (154, 73, 0, 2, 62,
   29, 12). Put the dated results in `experiments/011-short-form-spike/results-2026-11-10.md`, with
   every limit stated: a spike's paper count is not a run (ruling 45).
6. **The register.** Add the experiment's row to the roadmap's register as A79, with its state, and
   its line in `experiments/README.md`.

## Scope

Create: `experiments/011-short-form-spike/`. Change: `ROADMAP.md` (the register row and a citation
so `tools/check_roadmap.py` passes), `docs/AUDIT.md` (A79's action-item row only, if the roadmap's
rules require one), `experiments/README.md`. Do not change `spec/`, `reference-model/` or
`conformance/`.

## Done when

Each S1 state in both styles translates and is accepted by today's loader and core, or each failure
is reported with its diagnostic; the counts and their limits are in the dated results. From the
repository root: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass; in
the experiment, `cargo test --release`, `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check` pass. Report the total passed and failed. Do not stage anything.

## Record

Run on 2026-11-10: the worker in 14 minutes, then a fresh verifier in 4, which found nothing to fix
and reproduced the results. The exact translation is refused by today's kernel because it emits
`as`, one of ruling 53's additions not yet in the specification (finding 031-01); with `as` dropped,
all fourteen states register. In flow style S1 costs 55 lines on paper against the baseline's 64,
better on two tasks and worse on T1 and T7; in block style 97. The admin reran the spike's tests:
9 pass.
