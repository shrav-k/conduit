# Instructions for agents working in this repository

Conduit is a configuration language for the relationships between independent agents and the
things they share. This repository holds the draft language and its first build: a strict loader,
a pure core and a runnable engine. Where the work stands is in
[the tracker](docs/AUDIT.md#where-things-stand).

**If you were asked to build an engine independently, stop.** Independent builders work only from
the `build-base` branch, which is not yet published, and must not read this branch
([independent builds](docs/implementation/INDEPENDENT_BUILDS.md)). The first build in
[engine/](engine/README.md) is not one of them: it is written beside the language (ruling 46).

## Read first

1. [spec/README.md](spec/README.md): what the specification is, its three layers and the form of
   a promise.
2. [spec/definitions.md](spec/definitions.md): the terms in force. Write new text in them.
3. [IN_FORCE.md](IN_FORCE.md): every decision in force that the specification does not hold, and
   the accepted ones not yet written into it.
4. [docs/AUDIT.md](docs/AUDIT.md): the state of the work, what waits for the owner, what is known
   to be open, and the action items.
5. [ROADMAP.md](ROADMAP.md): the order of the work, and every experiment and probe.

## The goal

The language is the core and the engine keeps its promises. Conduit is a configuration language,
not a behavioral specification: the language fully determines what a set of documents means, and
behavior beyond a few shared conventions belongs to type contracts, providers and a conformance
document ([the layered claim](docs/AUDIT.md#layered-claim)). Work that makes the language more
concise, concrete and provable comes first.

The relationship is the product: an agent decides for itself, so the language declares what it may
reach, under which limits, and who answers for what it did. Conduit innovates only on connection
and access (ruling 40): everything else is a provider behind a contract that any existing tool can
implement, and a design that invents infrastructure instead is wrong. Check any change against the
[purpose and preservation tests](docs/AUDIT.md#purpose).

## What holds

In this order (ruling 67):

1. **The specification, [spec/](spec/README.md), alone binds behavior.** A behavior missing from it
   is a gap in it; no other document supplies it. Where any record and the specification differ,
   the specification governs.
2. **[IN_FORCE.md](IN_FORCE.md)** states every other decision in force. A line marked *spec
   pending* tells the session that writes `spec/` what to write; builds and vectors follow `spec/`
   until it is written. A line that differs from the owner's words in a ruling is a defect: the
   owner's words govern until it is fixed.
3. **[The ledger](docs/proposals/ruling-log.md)** says what the owner ruled, when, in the owner's
   words, and each ruling's status.
4. **A ruling's record**, under `docs/proposals/`, says why. Its first line says whether it still
   holds.

Not binding: `ROADMAP.md`, `docs/OVERVIEW.md`, `docs/implementation/`, `docs/research/`,
`docs/archive/` and `experiments/`. Under ruling 57, a document that does not bind may be edited,
condensed or removed once every piece of relevant Conduit information in it is preserved in `spec/`
or a live document; git keeps every version. What left the tree is listed, with its last version,
in [the archive's index](docs/archive/README.md#removed-from-the-tree).

The vocabulary is ruled. Older documents keep earlier words; do not mass-rename them.
[spec/definitions.md](spec/definitions.md#words-the-promises-do-not-use) maps each old name to its
new one. If you were asked to read or audit this repository cold, start at
[the cold readers' instructions](docs/guides/cold-readers.md).

## Where things go

- **An owner ruling**: one change writes its record under `docs/proposals/ruling-NN/`, its row in
  the ledger, the status lines and rows of the rulings it amends, the lines of IN_FORCE.md it
  changes, and any action item. An answer that changes no decision is added to the existing record,
  dated, and gets no new number (ruling 67).
- **Accepted behavior**: a promise in `spec/`, in the terms of `spec/definitions.md`. It changes by
  a logged ruling.
- **A defect in the language**: fix it in the language, not around it.
- **A finding of the first build**: the open increment's log under
  [docs/proposals/first-build/](docs/proposals/first-build/README.md), and the fix in the
  specification with a vector (ruling 46, point 4).
- **Evidence**: its own directory under `experiments/`, with dated results. Never present an
  expected outcome as a result. Before proposing an experiment or a probe, look for it in the
  roadmap's [register](ROADMAP.md#experiments-and-probes), and follow
  [its rules](ROADMAP.md#proposing-an-experiment).

## Rules

- **Code is Rust**, with a `Cargo.lock`, a pinned toolchain and ignored build output: a probe or an
  evaluation in its own directory under `experiments/`, a provider package under `packages/`, one
  crate per provider (ruling 40), and the first build in the workspace under [engine/](engine/README.md)
  (ruling 46). The only exception is the Python checkers in `tools/`.
- **The owner makes the rulings.** When a choice changes behavior, present the options with a
  recommendation and wait; do not record your recommendation as accepted. Who else decides what is
  in [the tracker](docs/AUDIT.md#who-decides-what).
- **Reference clones** in the sibling directories of this repository, if present, are the owner's,
  to consult only. Do not edit them; nothing here needs them.
- **Commits and branches.** Commit or push only when asked, and on a branch. The sessions the owner
  asked to write the language, and the coordinator (ruling 48), commit and push each milestone on a
  pull request of its own; a work order's branch is `wo/NNN-name`
  ([work orders](docs/implementation/WORK_ORDERS.md)). Merges to `main` are squashed, with a message
  written by hand. The history of earlier pull requests is in the ledger and in git.

## If you were given a work order

Under [ruling 48](docs/proposals/ruling-48/README.md), the coordinator's session plans and decides,
and Codex sessions carry out work orders, as [work orders](docs/implementation/WORK_ORDERS.md)
describes. If you were started with an order from `docs/work-orders/`:

- Apply the decisions the order states. Do not reopen them. Where one cannot be applied as written,
  apply the nearest sound reading and say so in your report.
- Change only the files in the order's scope. Report anything else that needs changing.
- Do not commit, push, open a pull request or switch branches: the admin session does.
- Do not edit `~/.claude`, `~/.claude.json`, `~/.codex/config.toml` or any other configuration of
  the owner's, and never read, print or copy a credential.
- Run the order's checks, and the checks below that apply, before you finish.
- Answer in the order's report schema. List apart every choice you made yourself and every check's
  last line, and never report a check you did not run.

## Checks

Run these from the repository root before each commit:

```sh
python3 tools/check_docs.py          # links, anchors and code fences in every Markdown file
python3 tools/check_rulings.py       # ruling ledger, status lines and undated decisions in force
python3 tools/set_names.py --check   # the names under spec/, engine/, conformance/ and packages/ agree with spec/names.yaml
python3 tools/check_schemas.py       # schemas, core package and examples; needs PyYAML and jsonschema
python3 tools/check_roadmap.py       # one row per action item, and the roadmap cites each open one
git diff --check                     # whitespace errors
(cd engine && cargo test --release)   # the first build: loader, core, vectors and shell
```

What each check needs:

- `check_docs.py`, `set_names.py` and `check_roadmap.py` need only Python 3. In a Git working
  tree `check_docs.py` asks Git for the Markdown files. Anywhere else, such as a copy downloaded
  as an archive, it walks the directory, skipping `.git/`, `target/` and what `.gitignore`
  plainly names. Its first line of output says which it did.
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
- `cargo test --release` in `engine/` needs Rust through rustup, which installs on
  first use the toolchain that `engine/rust-toolchain.toml` pins. Run it from inside
  the directory, so that the pin applies. For a change to its code, run
  `cargo clippy --all-targets -- -D warnings` there too. The tests read `spec/`, and check the
  loader's verdict on every example against the verdicts of `check_schemas.py`, which a
  fixture records.

The names the specification assigns (the core package's address, `apiVersion`, the prefix of
its `_meta` keys and, since ruling 43, the names of what `identity` carries) are defined once,
in `spec/names.yaml`. Never edit one by hand. Run
`python3 tools/set_names.py` with the new values to see what would change, and add `--write` to
rewrite every occurrence under `spec/` together, and under `engine/`, `conformance/`
and `packages/`. The code under `engine/` reads the names from
`spec/names.yaml` and writes none of them, so the tool finds nothing to rewrite there. The tool
does not change a document outside them that quotes a name, such as the example in
`docs/OVERVIEW.md`: update it once the tool has run.

Run `cargo test --release` in each Rust directory you change, under `experiments/`,
`packages/` or `engine/`, and in `engine/` after any change to `spec/`.
