# Contributing to Conduit

Conduit is a configuration language for the relationships between independent agents and the
things they share. As of 2026-11-10, this repository holds its draft specification, the records
behind it, and the first build: a strict loader, a pure core, the reference model, conformance
vectors and a runnable engine shell. The shell ran real Claude Code and Codex as Agents on S1
through T1, T2 and T7 ([dated results](docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md)).
Increment 4 is in progress: on 2026-11-10 both author runs completed T1 to T7 with real clients
but failed the owner's bar for authoring work. The [build log](docs/proposals/first-build/log-increment-4.md)
records the findings and fixes from orders 025 to 028; the
[roadmap](ROADMAP.md#the-order-of-the-work) puts simplification and a rerun next.
This is not a release or a claim of whole-language conformance.

Read in this order: the [README](README.md), [spec/README.md](spec/README.md),
[spec/definitions.md](spec/definitions.md), then the [plan audit](docs/AUDIT.md) for the state
of the work and the owner's rulings, and the [roadmap](ROADMAP.md) for the order of the work.
The [documentation map](docs/README.md) lists every other document, and the
[reading paths](spec/README.md#reading-paths) of the specification say what to read for writing
documents, for writing contracts and providers, and for building an engine.
[AGENTS.md](AGENTS.md) states the same rules in short form.
For a review of the text, follow the [cold readers' instructions](docs/guides/cold-readers.md).
To run the build, start with [reference-model/](reference-model/README.md#running-it); the
[guides](docs/guides/README.md) cover the owner's setup for later experiments.

## What binds

The specification in [spec/](spec/README.md) alone binds. Nothing under `docs/` or
`experiments/` does, nor does the [roadmap](ROADMAP.md). The design as it stood before the
specification was removed from the tree by ruling 39; the
[archive's index](docs/archive/README.md#removed-from-the-tree) links its last version, as
source material for the type contracts still to be written. A behavior that is missing from
`spec/` is a gap in `spec/`.

As of 2026-11-10, rulings 49 to 59 are accepted; their specification changes remain pending,
while rulings 57 and 58 also govern documentation upkeep. Its [status](spec/README.md) links each:

- [Ruling 55](docs/proposals/ruling-55/README.md): the short form is language, names stay stable and commands with named flags approve dangerous acts.
- [Ruling 56](docs/proposals/ruling-56/README.md): provider-defined enrollment verification, evidence shown by the engine at root and recorded exceptions for weaker checks.
- [Ruling 57](docs/proposals/ruling-57/README.md): historical documents may be edited, condensed or removed once their relevant Conduit information is preserved elsewhere.
- [Ruling 58](docs/proposals/ruling-58/README.md): earlier logged choices accepted with stated exceptions, fixed ruling records and condensation at close.
- [Ruling 59](docs/proposals/ruling-59/README.md): the Agent-owning family's structure, root's default Conduits, discovery and replacement bounds; its name and later authority decisions remain open.

The [language's shape](docs/proposals/language-shape/README.md) and the owner's
[Project direction](docs/proposals/owner-direction/2026-11-10-projects.md) are being investigated
and settled first, so each change is written once. Read those records as decisions or
proposals as marked, rather than as promises already implemented.

## Where work belongs

- **Accepted behavior** is a promise in [spec/](spec/README.md), in the terms of
  [spec/definitions.md](spec/definitions.md). A change needs an owner ruling, recorded in the
  [ruling log](docs/AUDIT.md#ruling-log) of the plan audit with its record under
  `docs/proposals/`. The one other way is a decision on a finding of the re-audit, under
  ruling 37, or a finding of the first build under ruling 46, point 4, logged as the coordinator's
  ([who decides what](docs/AUDIT.md#who-decides-what)).
- **A proposal for a ruling** is a record under `docs/proposals/`, with the options and a
  recommendation. The owner makes the rulings. Do not record a recommendation as accepted.
- **A defect in the language** is fixed in the language, not around it. Findings against `spec/`
  are recorded with the [re-audit](docs/proposals/reaudit/README.md). The
  [defect register](docs/LANGUAGE_DEFECTS.md) holds the first sweep, which read the protocol topics that ruling 39 removed.
- **The state of the work** is tracked in the [plan audit](docs/AUDIT.md), and its order and its
  experiments and probes in the [roadmap](ROADMAP.md).
- **The first build** follows [its increments](docs/implementation/FIRST_BUILD.md) and logs
  findings under [docs/proposals/first-build/](docs/proposals/first-build/README.md), with the
  fix in the specification and a vector. Under [ruling 48](docs/proposals/ruling-48/README.md),
  the coordinator plans and decides and Codex workers carry out
  [work orders](docs/implementation/WORK_ORDERS.md). Workers do not commit or publish; the
  admin session reviews and handles that work.
- **The independent builds** are separate, later work. Their process and the brief each
  builder receives are in [docs/implementation/](docs/implementation/INDEPENDENT_BUILDS.md).
- **Research**, meaning sourced comparisons and the analysis of probe results, goes in
  [docs/research/](docs/research/README.md). Attribute source versions and distinguish inference
  from observed behavior.
- **Evidence** goes in its own directory under [experiments/](experiments/README.md), with its
  scope, instructions and dated results. An expected outcome is never presented as a result.
  Before proposing an experiment or a probe, check the roadmap's
  [register](ROADMAP.md#experiments-and-probes): name the row it serves, or say that it is new
  and what it feeds, and follow [the rules for proposing one](ROADMAP.md#proposing-an-experiment).
- **Code** is Rust, with a lockfile, a pinned toolchain and ignored build output: a probe or an
  evaluation in its own directory under [experiments/](experiments/README.md); a provider
  package, one crate per provider, under [packages/](packages/) (ruling 40), which as of
  2026-11-10 holds the Claude Code and Codex Agent packages; and the first build, its core and
  its shell, in the workspace under
  [reference-model/](reference-model/README.md) (ruling 46). The only exception is the Python
  checkers in `tools/`.
- **History** is in [docs/archive/](docs/archive/README.md): completed reviews, attribution and
  dissent.

Under [ruling 57](docs/proposals/ruling-57/README.md), a document that does not bind may be edited, condensed or removed once every piece of relevant Conduit information in it is preserved in `spec/` or a live document, git keeping every version.

## Review discipline

The specification is the authority. Code, examples and schemas do not redefine it: where an
example or a schema differs from a promise, the promise governs.

Raise an open question with the promise it affects, a concrete example and the choice needed.
Do not reopen a ruled question because an older document uses different words. Separate
mechanical moves from changes of meaning where practical, repair links, and keep source pins.

What the language promises about versions is in
[spec/promises/naming.yaml](spec/promises/naming.yaml) and
[spec/promises/packages.yaml](spec/promises/packages.yaml). The specification itself is a
draft and not a release.

## Vocabulary changes

The vocabulary is ruled (rulings 17, 18 and 27 to 29 of the plan audit). The terms in force are
in [spec/definitions.md](spec/definitions.md). Write new text in them. Changing a term changes
the specification, so it needs a ruling like any other change. Ruling 42 accepted the twenty
names of the [vocabulary audit](docs/proposals/vocabulary-audit/README.md) that ruling 41 asked
for, and [spec/definitions.md](spec/definitions.md#words-the-promises-do-not-use) lists each old
name with its new one.

[docs/VOCABULARY.md](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md) was the map of the working terms from before the
vocabulary ruling. Ruling 39 removed it from the tree, and the link is to its last version. Its
`vxx` anchors are reached through links pinned to that version.

Do not rename terms across the older documents, research or archives. They keep the words they
were written in. Terms that belong to JSON Schema, YAML or the Model Context Protocol keep
their own meanings.

## Checks

Run these from the repository root before each commit:

```sh
python3 tools/check_docs.py          # links, anchors and code fences in every Markdown file
python3 tools/set_names.py --check   # the names under spec/, reference-model/, conformance/ and packages/ agree with spec/names.yaml
python3 tools/check_schemas.py       # schemas, core package and examples; needs PyYAML and jsonschema
python3 tools/check_roadmap.py       # one row per action item, and the roadmap cites each open one
git diff --check                     # whitespace errors
(cd reference-model && cargo test --release)   # the first build: loader, core, vectors and shell
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
- `cargo test --release` in `reference-model/` needs Rust through rustup, which installs on
  first use the toolchain that `reference-model/rust-toolchain.toml` pins. Run it from inside
  the directory, so that the pin applies. For a change to its code, run
  `cargo clippy --all-targets -- -D warnings` there too.

Run `cargo test --release` in each Rust directory you change, under `experiments/`,
`packages/` or `reference-model/`, and in `reference-model/` after any change to `spec/`.

What the checks cover:

- `check_docs.py` validates repository Markdown paths, heading anchors, reference links and
  fenced-code balance without network access. It does not validate external links, and it
  does not check that some document links to each file. `--walk` makes it walk the directory
  in a Git working tree too.
- `set_names.py --check` refuses a retired name, a `_meta` key of Conduit's written with another
  prefix, a value that looks like one of the three names of `spec/names.yaml` and differs from
  it, and a variable that begins as those of `identity` do and is neither. To change the names,
  run it with the new values to see what would change, and add `--write` to apply.
- `check_schemas.py` checks that the files under `spec/core/` and `spec/schemas/` are strict
  YAML within the schema profile, that every example document is accepted or rejected as its
  directory says, and that the core package agrees with itself. Its own header lists each step.
- `check_roadmap.py` checks that each action item, from A1 to the highest, has exactly one row,
  in the plan audit, the roadmap or the plan audit's history; that each one cited has a row;
  that the roadmap's order of the work cites each one not yet done; that each
  directory under `experiments/` is listed in its index and linked from the roadmap; that the
  entry points link the roadmap; and that each state begins with an allowed word. It cannot
  tell whether a state is true. Its own header lists each check.
- `cargo test --release` in `reference-model/` runs the first build's tests: the strict loader
  reads every example under `spec/schemas/examples/` and checks its verdict against the one
  `check_schemas.py` gives, which a fixture records, and reads documents and directories case
  by case. As of 2026-11-10 it also tests the pure core, portable vectors, durable shell,
  runner and HTTP adapter, with scripted clients by default.
  [reference-model/README.md](reference-model/README.md) gives the commands and limits.

The Python and whitespace checks do not establish engine behavior. As of 2026-11-10, the Rust
tests exercise the implemented slice; real-client evidence is recorded separately in the
[increment 3 closeout](docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md).
Passing these checks does not establish whole-language conformance.

The dated migration has a separate historical audit:

```sh
python3 tools/check_doc_migration.py
```

It reads fixed Git objects, so it cannot fail on later changes and is a record of that migration
only. It verifies preservation against the recorded baseline and mechanical-move commit, including explicitly
listed editorial changes. By default it checks the commit that introduced the migration record, so
future edits need not remain frozen. Full Git history containing those commits is needed. See the
[migration record](docs/archive/migrations/2026-11-protocol-framing.md) for working-tree use and limits.

## License and attribution

Original Conduit documentation, contracts and code use the [Apache License, Version 2.0](LICENSE).
The repository includes the unmodified official license text. Preserve applicable copyright,
license and attribution notices when adding or adapting material.

This adoption does not relicense reproduced third-party material. In particular, the historical
collaborator submissions in `docs/archive/collaborator-proposals/PR-001-roadmap.md` and
`docs/archive/collaborator-proposals/PR-002-protocol.md` retain their authorship and source attribution;
this change does not assert an additional license grant from their authors. Quoted source material
in research likewise retains its applicable terms. The archive index records the original sources.

No separate contributor agreement or DCO/sign-off requirement is introduced by this ruling.
Any future contribution procedure remains an explicit project decision.

## Project ownership and publication

One main repository, one format for every package and Apache-2.0 are accepted. The format is
the manifest and the archive that [the package promises](spec/promises/packages.yaml) give;
the older documents call it the common package envelope. Public naming is
deferred: work continues in `shrav-k/conduit` until the owner selects an organization or transfer.
Until there is a domain, the names the specification assigns come from this repository (rulings
33 to 35 of the plan audit). They are defined once, in `spec/names.yaml`, and
`tools/set_names.py` changes them together.

The repository owner retains final authority over the direction of the language. Collaborators
can propose and review changes; merge and release work uses existing repository permissions and responsibilities
explicitly delegated to appointed maintainers. Work that the owner has already asked for needs no
new approval. This does not introduce an additional approval step for already-authorized work, a fixed review quorum or a
formal committee. Governance can evolve through an explicit project decision when useful.

Public support/security contact details and any additional contribution procedures can be selected
when distribution needs them. The earlier
[review](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/REVIEW.md#owner-decisions-before-public-distribution) records these boundaries.
This guide adds no promise about releases.

The [organization and publication plan](docs/proposals/ruling-47/README.md) records the
owner's decision for the transition after a v0 engine passes the full experiment program and
the owner judges it worth publishing. The in-tree Rust convention above applies to prototype
providers. Official integrations will then have their own maintainers and release repositories.
Outside authors can build packages in any language that implements the published contracts.
