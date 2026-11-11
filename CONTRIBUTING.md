# Contributing to Conduit

Conduit is a configuration language for the relationships between independent agents and the
things they share. As of 2026-11-10, this repository holds its draft specification, the records
behind it, and the first build: a strict loader, a pure core, conformance
vectors and a runnable engine shell. The shell ran real Claude Code and Codex as Agents on S1
through T1, T2 and T7 ([dated results](docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md)).
Increment 4 is in progress: on 2026-11-10 both author runs completed T1 to T7 with real clients
but failed the owner's bar for authoring work. The [build log](docs/proposals/first-build/log-increment-4.md)
records the findings and fixes from orders 025 to 028; the
[roadmap](ROADMAP.md#the-order-of-the-work) puts simplification and a rerun next.
This is not a release or a claim of whole-language conformance.

Read in this order: the [README](README.md), [spec/README.md](spec/README.md),
[spec/definitions.md](spec/definitions.md), then the [plan audit](docs/AUDIT.md) for the state
of the work, and the [roadmap](ROADMAP.md) for the order of the work.
The [documentation map](docs/README.md) lists every other document, and the
[reading paths](spec/README.md#reading-paths) of the specification say what to read for writing
documents, for writing contracts and providers, and for building an engine.
[AGENTS.md](AGENTS.md) states the same rules in short form.
For a review of the text, follow the [cold readers' instructions](docs/guides/cold-readers.md).
To run the build, start with [engine/](engine/README.md#running-it); the
[guides](docs/guides/README.md) cover the owner's setup for later experiments.

## What binds

The specification in [spec/](spec/README.md) alone binds. Nothing under `docs/` or
`experiments/` does, nor does the [roadmap](ROADMAP.md). The design as it stood before the
specification was removed from the tree by ruling 39; the
[archive's index](docs/archive/README.md#removed-from-the-tree) links its last version, as
source material for the type contracts still to be written. A behavior that is missing from
`spec/` is a gap in `spec/`.

[IN_FORCE.md](IN_FORCE.md) holds decisions in force outside the specification, including
accepted language decisions marked *spec pending*. Builds and vectors follow `spec/` until those
changes are written. The [ledger](docs/proposals/ruling-log.md) records what was ruled and each
ruling's status; the [records](docs/README.md#records-of-rulings) say why. The owner's words govern
if a line of IN_FORCE.md differs from them ([ruling 67](docs/proposals/ruling-67/README.md)).

## Where work belongs

- **Accepted behavior** is a promise in [spec/](spec/README.md), in the terms of
  [spec/definitions.md](spec/definitions.md). A change needs an owner ruling, recorded in the
  [ledger](docs/proposals/ruling-log.md), with its record under `docs/proposals/`. The one other
  way is a decision on a finding of the re-audit, under
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
  [engine/](engine/README.md) (ruling 46). The only exception is the Python
  checkers in `tools/`.
- **History** is in [docs/archive/](docs/archive/README.md): completed reviews, attribution and
  dissent.

Under [ruling 57](docs/proposals/ruling-57/README.md), a document that does not bind may be edited, condensed or removed once every piece of relevant Conduit information in it is preserved in `spec/` or a live document, git keeping every version.

## Keeping decisions and documents current

Under [ruling 67](docs/proposals/ruling-67/README.md), one change writes a new ruling's record,
its ledger row in `docs/proposals/ruling-log.md`, the status lines and ledger rows of every ruling
it amends or replaces, IN_FORCE.md's affected lines, and any action item. A specification batch
turns its *spec pending* lines into pointers. An answer that confirms or clarifies a ruling
without changing a decision is added to the existing record, dated, and to its ledger row; it
gets no new ruling number.

A proposal record says under its status that it records a ruling, that the specification governs
where they differ, and which later rulings amended it. Put a note beside examples or rules a
later ruling reversed (DA8 of the [documents audit](docs/proposals/docs-audit/README.md)). Keep
one reading order across entry points (DA10), one description of Conduit, the specification's
own (DA11), and the same check list in AGENTS.md and this guide (DA12).

For removal or condensation, first preserve every relevant piece in `spec/` or a live document:
a ruling and its reasons; what was put to the owner, with the options; an open item; the reason
an alternative was rejected; v0 evidence and the method as run; or instructions. Verify the
extraction against the pinned source before removing it. Keep a file that the specification or a
ruling cites as its reasoning, condensing it in place. Preserve the owner's verbatim words,
decision tables and cited option essays; keep dissent in the archive index. Each removal gets an
[archive row](docs/archive/README.md) with what it was, what it is source material for and its
last version at a pinned commit; repoint incoming links and retarget live instructions. In this
condensation, leave surviving text as written except link targets, one-line superseded or
accepted notes, and live instructions retargeted to where the work now goes.

Dated results, the method as run, prompts and counted artifacts remain as v0 evidence. Raw run
records leave the tree once their results are merged, with links to their last version in git;
increment 4's runs stay until the S1 rerun (ruling 58, point 5). At the close of a batch,
increment or investigation, condense working papers into registers and records (point 10).

Some facts still stand in two places, and no check covers them; the coordinator keeps each pair
in step. A68's action and step 3 both name the pieces of the first build. A63's candidates and
the rows of A22 and A73 both name the stores the probes serve. The index of experiments and
the roadmap's rows of Experiments 001 to 005 both say what each did. The priority P1 says that
the native probes run beside it, as the row "In parallel" does.

## Writing promises

1. A promise keeps the exact form `id`, `layer`, `level`, `when`, `then`, `observable`,
   `vectors: []`, with folded `>-` text indented four spaces and lines under 97 characters.
   `layer` is `language`, `convention` or `interface`. `level` is `must`, `must not`, `should`
   or `may`.
2. IDs are never reused or renumbered. A new promise takes the next number never used in that
   file's prefix; check the head comment's removed-ID register first.
3. A removed promise is deleted and named in its file's head comment with the ruling that
   removed it and where its content went, or "nothing replaces it". The register form is
   `Removed: OLD → NEW (ruling NN).` or `OLD: nothing replaces it (ruling NN).`
4. No promise moves between files. Where content moves, remove the old ID and give the content
   a new ID in its destination. Change a promise's layer in place when that is the ruled change.
5. Every promise about a request that is not dispatched names exactly one of refused, invalid,
   conflict and unavailable, in the order of OPS-023.
6. A reference to another promise by ID must still exist after the pass.
7. A rewrite keeps everything in the promise that the accepted change does not name, apart from
   sentences that cite a removed ID or a term that goes, which are cut or pointed at the ID
   named in its place.

A specification head comment holds what the file holds, its standing, reading notes, the
standards it follows, open issues and its register of removed IDs. It carries no change narrative
(ruling 58, point 6); see [the form of a promise](spec/README.md#the-form-of-a-promise).

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

One word for one idea, and one idea for one word. Use plain technical words for authoring and
operation; flow imagery only around Conduits. Words belonging to a type or profile are settled
with that type or profile ([vocabulary principles](docs/proposals/vocabulary/README.md#principles)).
Before offering a proposed shape, check it against the CS/CC list of the
[consolidation audit](docs/proposals/consolidation/README.md).

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
  `cargo clippy --all-targets -- -D warnings` there too.

Run `cargo test --release` in each Rust directory you change, under `experiments/`,
`packages/` or `engine/`, and in `engine/` after any change to `spec/`.

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
- `cargo test --release` in `engine/` runs the first build's tests: the strict loader
  reads every example under `spec/schemas/examples/` and checks its verdict against the one
  `check_schemas.py` gives, which a fixture records, and reads documents and directories case
  by case. As of 2026-11-10 it also tests the pure core, portable vectors, durable shell,
  runner and HTTP adapter, with scripted clients by default.
  [engine/README.md](engine/README.md) gives the commands and limits.

The Python and whitespace checks do not establish engine behavior. As of 2026-11-10, the Rust
tests exercise the implemented slice; real-client evidence is recorded separately in the
[increment 3 closeout](docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md).
Passing these checks does not establish whole-language conformance.

## License and attribution

Original Conduit documentation, contracts and code use the [Apache License, Version 2.0](LICENSE).
The repository includes the unmodified official license text. Preserve applicable copyright,
license and attribution notices when adding or adapting material.

This adoption does not relicense reproduced third-party material. The collaborator's
[PR #1](https://github.com/shrav-k/conduit/pull/1) at `2111b0d` and
[PR #2](https://github.com/shrav-k/conduit/pull/2) at `c480e96`, merged as `71d6348` and
`4b3db58`, retain their authorship and source attribution, as does the vocabulary research of
[PR #5](https://github.com/shrav-k/conduit/pull/5) at `2890ef7`. Their history remains in git;
the [archive index](docs/archive/README.md) preserves the source pointers. This does not assert an
additional license grant from their authors. Quoted source material retains its applicable terms.

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
