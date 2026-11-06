# Instructions for agents working in this repository

Conduit is a configuration language for the relationships between independent agents and the
things they share. This repository holds the language's design. No Conduit engine exists yet.

**If you were asked to build an engine, stop.** Engine builders work only from the `build-base`
branch and must not read this branch. That branch is not yet published, and the builds have not
started. See [independent builds](docs/implementation/INDEPENDENT_BUILDS.md).

## Read first

1. [spec/README.md](spec/README.md): what the specification is, its three layers, its contents
   and the form of a promise.
2. [spec/definitions.md](spec/definitions.md): the terms in force.
3. [docs/AUDIT.md](docs/AUDIT.md): the state of the work, the owner's direction, the ruling
   log, the action items and the experiment program.

## The goal

The language is the core and the engine keeps its promises. Conduit is a configuration language,
not a behavioral specification: the language fully determines what a set of documents means,
and behavior beyond a few shared conventions belongs to type contracts and a runtime profile. See
the layered claim in [docs/AUDIT.md](docs/AUDIT.md#layered-claim). Work that makes the language more
concise, concrete and provable comes first.

The relationship is the product: an agent decides for itself, so the language declares what it may
reach, under which limits, and who answers for what it did. Conduit innovates only on connection
and access (ruling 40): everything else is a provider behind a contract that any existing tool
can implement, and a design that invents infrastructure instead is wrong. Check any change
against the purpose and preservation tests in [docs/AUDIT.md](docs/AUDIT.md#purpose).

## What binds and what does not

- The specification, [spec/](spec/README.md), alone binds. It holds three layers: the language,
  the shared conventions and the implementation interface, as numbered promises. It is a draft:
  every area is written, and no conformance vector exists yet.
- A behavior that is missing from `spec/` is a gap in `spec/`. No older document supplies it.
- The design as it stood before the specification, under `docs/protocol/`, and the other
  documents that the specification superseded were removed from the tree by ruling 39. The
  [archive's index](docs/archive/README.md#removed-from-the-tree) lists each, with a link to
  its last version. They are source material for the type contracts still to be written, and
  bind nothing.
- The records under `docs/proposals/` say what was decided, by whom and why: the owner's
  rulings, and for the documents audit and the rounds of the re-audit the decisions of the
  coordinating session under ruling 37, which ruling 39 accepted. Where a record and the
  specification differ, the specification governs.
- If you were asked to read or audit this repository cold, start at
  [for the next reader](docs/AUDIT.md#for-the-next-reader): what to read, what is known to be
  open, what to look at first, and where findings go.
- Also not binding: `docs/OVERVIEW.md`, `docs/implementation/`, `docs/research/`,
  `docs/archive/` and everything under `experiments/`. Most predate the specification and say
  so in a note under their title. The files under `docs/archive/` are the exception: only the
  archive's index, `docs/archive/README.md`, carries the note. An archived file that calls the
  protocol topics the active reference, or the review queue current, describes the time it
  was written.
- The vocabulary is ruled (rulings 17, 18 and 27 to 29). The terms in force are in
  [spec/definitions.md](spec/definitions.md); write new text in them. Older documents keep the
  earlier words; do not mass-rename them.

## Where things go

- An owner ruling: the ruling log in `docs/AUDIT.md`, with its record under `docs/proposals/`,
  then the documents it affects.
- Accepted behavior: a promise in `spec/`, in the terms of `spec/definitions.md`. It changes
  by a logged ruling. Under ruling 37, as the owner confirmed it in ruling 38, the session that
  coordinates the re-audit may also settle a cold reader's finding without a ruling on it, and
  logs each such decision under `docs/proposals/reaudit/` as its own
  ([who decides what](docs/AUDIT.md#who-decides-what)). Ruling 39 accepted every such
  decision made before it.
- A defect in the language: fix it in the language, not around it. Findings against `spec/` are
  recorded under `docs/proposals/reaudit/`. `docs/LANGUAGE_DEFECTS.md` is the register of the
  first sweep, which read the protocol topics that ruling 39 removed.
- Evidence: its own directory under `experiments/`, with dated results. Never present an
  expected outcome as a result.

## Rules

- Any code is Rust, with a `Cargo.lock`, a pinned toolchain and ignored build output: a probe or
  an evaluation in its own directory under `experiments/`, and a provider package, one crate per
  provider, under `packages/` (ruling 40). The only exception is the Python checkers in `tools/`.
- The owner makes the rulings. When a choice changes behavior, present the options with a
  recommendation and wait; do not record your recommendation as accepted.
- Do not edit what `docs/archive/` holds or the content of `docs/research/`: they are records
  of their time. New material may be added to the archive, with a line in its index. Ruling 39
  made one exception, to remove the names of two private repositories.
- Do not edit the sibling directories of this repository that hold reference clones, if
  present. They are on the owner's machine, to consult only. Nothing in this repository
  needs them.
- Commit or push only when asked, and work on a branch. One standing instruction exists, and it
  is for the sessions the owner has asked to write the language, not for a reviewer or any
  other session: the owner's instruction of 2026-11-05 is that this work is tracked on a pull request
  (ruling 36), and the owner confirmed in ruling 38 that those sessions commit and push to its
  branch at each milestone. Pull request 6 was merged by squashing after ruling 38. The work of
  ruling 39 is on the branch `docs/pre-codex`, pull request 7. Merges to `main` are squashed,
  with a message written by hand.

## Checks

Run these from the repository root before each commit:

```sh
python3 tools/check_docs.py          # links, anchors and code fences in every Markdown file
python3 tools/set_names.py --check   # the names under spec/ agree with spec/names.yaml
python3 tools/check_schemas.py       # schemas, core package and examples; needs PyYAML and jsonschema
git diff --check                     # whitespace errors
```

What each check needs:

- `check_docs.py` and `set_names.py` need only Python 3. In a Git working tree `check_docs.py`
  asks Git for the Markdown files. Anywhere else, such as a copy downloaded as an archive, it
  walks the directory, skipping `.git/`, `target/` and what `.gitignore` plainly names. Its
  first line of output says which it did.
- `check_schemas.py` needs the Python packages PyYAML and jsonschema. It was last run with
  PyYAML 6.0.1 and jsonschema 4.20.0. To install them:

  ```sh
  python3 -m pip install pyyaml jsonschema
  ```

  If the `python3` that runs it lacks either package, and `/usr/bin/python3` exists and is a
  different interpreter, it runs itself again under that one. If PyYAML is still missing, it
  stops, says that nothing was checked and exits with status 2. If only jsonschema is missing,
  it runs steps 1, 2, 5 and 6 without the one rule that needs a validator, and says that steps
  3 and 4 and that rule were not run. Its last line then begins "Errors: N in the steps that
  ran", and it exits with status 3 when N is 0. It never reports a check it did not run. Its
  first line of output names the interpreter and the package versions it used.
- `git diff --check` needs Git and a Git working tree.

The names the specification assigns (the core package's address, `apiVersion` and the prefix of
its `_meta` keys) are defined once, in `spec/names.yaml`. Never edit one by hand. Run
`python3 tools/set_names.py` with the new values to see what would change, and add `--write` to
rewrite every occurrence together.

Run `cargo test --release` in each Rust experiment directory you change.
