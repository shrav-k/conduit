# Instructions for agents working in this repository

Conduit is a configuration language for the relationships between independent agents and the
things they share. As of 2026-11-10, this repository holds the draft language and the first
build: a strict loader, a pure core on S1, and a runnable engine shell.
The shell ran real Claude Code and Codex as Agents through S1's T1, T2 and T7; the
[closeout](docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md) states the
results and limits. Increment 4 is in progress as of 2026-11-10: both author runs completed
T1 to T7 with real clients but failed the owner's bar for authoring work. The
[log](docs/proposals/first-build/log-increment-4.md) records the findings and fixes from
orders 025 to 028. The language's shape and Projects are being settled with the owner before
the simplification is built and S1 rerun, as the [roadmap](ROADMAP.md#the-order-of-the-work) states.

**If you were asked to build an engine, stop.** Engine builders work only from the `build-base`
branch and must not read this branch. As of 2026-11-10, that branch is not yet published, and
the independent builds have not started. See
[independent builds](docs/implementation/INDEPENDENT_BUILDS.md). The first build in step 3 of the
[roadmap](ROADMAP.md#the-order-of-the-work) is not one of those builds: it is written beside the
language, as ruling 46 shaped it, one codebase with a pure core and a thin
shell that makes it runnable. Its increments and what each must show are in
[the first build](docs/implementation/FIRST_BUILD.md).

## Read first

1. [spec/README.md](spec/README.md): what the specification is, its three layers, its contents
   and the form of a promise.
2. [spec/definitions.md](spec/definitions.md): the terms in force.
3. [docs/AUDIT.md](docs/AUDIT.md): the state of the work, the owner's direction, the ruling
   log and the action items.
4. [ROADMAP.md](ROADMAP.md): the order of the work, and every experiment and probe with its
   state and what it feeds.

## The goal

The language is the core and the engine keeps its promises. Conduit is a configuration language,
not a behavioral specification: the language fully determines what a set of documents means,
and behavior beyond a few shared conventions belongs to type contracts, providers and a
conformance document. See the layered claim in [docs/AUDIT.md](docs/AUDIT.md#layered-claim).
Work that makes the language more concise, concrete and provable comes first.

The relationship is the product: an agent decides for itself, so the language declares what it may
reach, under which limits, and who answers for what it did. Conduit innovates only on connection
and access (ruling 40): everything else is a provider behind a contract that any existing tool
can implement, and a design that invents infrastructure instead is wrong. Check any change
against the purpose and preservation tests in [docs/AUDIT.md](docs/AUDIT.md#purpose).

## What binds and what does not

- The specification, [spec/](spec/README.md), alone binds. It holds three layers: the language,
  the shared conventions and the implementation interface, as numbered promises. It is a draft:
  every area is written; as of 2026-11-10, [vectors](conformance/README.md) test slices of its
  promises on S1, with missing coverage recorded in the [manifest](conformance/manifest.yaml).
  As of 2026-11-10, rulings 49 to 59 are accepted; their specification changes remain pending,
  while rulings 57 and 58 also govern documentation upkeep. Its [status](spec/README.md) lists
  them. The language's shape is being settled first so
  each change is written once. The owner's [Project direction](docs/proposals/owner-direction/2026-11-10-projects.md)
  guides that investigation; it supplies no missing promise.

  - [Ruling 55](docs/proposals/ruling-55/README.md): the short form is language, names stay stable and commands with named flags approve dangerous acts.
  - [Ruling 56](docs/proposals/ruling-56/README.md): provider-defined enrollment verification, evidence shown by the engine at root and recorded exceptions for weaker checks.
  - [Ruling 57](docs/proposals/ruling-57/README.md): the prohibition on editing historical documents is revoked; relevant Conduit information must be preserved.
  - [Ruling 58](docs/proposals/ruling-58/README.md): earlier logged choices accepted with stated exceptions, fixed ruling records and condensation at close.
  - [Ruling 59](docs/proposals/ruling-59/README.md): the Agent-owning family's structure, root's default Conduits, discovery and replacement bounds; its name and later authority decisions remain open.

- A behavior that is missing from `spec/` is a gap in `spec/`. No older document supplies it.
- The design as it stood before the specification, under `docs/protocol/`, and the other
  documents that the specification superseded were removed from the tree by ruling 39. The
  [archive's index](docs/archive/README.md#removed-from-the-tree) lists each, with a link to
  its last version. They are source material for the type contracts still to be written, and
  bind nothing.
- The records under `docs/proposals/` say what was decided, by whom and why: the owner's
  rulings, and for the documents audit and the rounds of the re-audit the decisions of the
  coordinating session under ruling 37. Ruling 58 accepted logged coordinator decisions, editor
  choices and work-order decisions made before ruling 57, except review-list items 15 to 19 and
  increment 4's open decisions; those exceptions and later choices remain open to the owner.
  [Who decides what](docs/AUDIT.md#who-decides-what) lists
  where they are logged. Where a record and the specification differ, the specification
  governs.
- If you were asked to read or audit this repository cold, start at
  [the cold readers' instructions](docs/guides/cold-readers.md), which link
  [for the next reader](docs/AUDIT.md#for-the-next-reader): what to read, what is known to be
  open, what to look at first, and where findings go.
- Also not binding: `ROADMAP.md`, `docs/OVERVIEW.md`, `docs/implementation/`, `docs/research/`,
  `docs/archive/` and everything under `experiments/`. Most predate the specification and say
  so in a note under their title. The files under `docs/archive/` are the exception: only the
  archive's index, `docs/archive/README.md`, carries the note. An archived file that calls the
  protocol topics the active reference, or the review queue current, describes the time it
  was written.
- The vocabulary is ruled (rulings 17, 18 and 27 to 29). The terms in force are in
  [spec/definitions.md](spec/definitions.md); write new text in them. Older documents keep the
  earlier words; do not mass-rename them. Ruling 42 accepted the twenty names of the
  [vocabulary audit](docs/proposals/vocabulary-audit/README.md) that ruling 41 asked for, and
  [spec/definitions.md](spec/definitions.md#words-the-promises-do-not-use) lists each old name
  with its new one.

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
- A finding of the first build: its increment's log under
  [docs/proposals/first-build/](docs/proposals/first-build/README.md), and the fix in the
  specification with a vector (ruling 46, point 4).
- Evidence: its own directory under `experiments/`, with dated results. Never present an
  expected outcome as a result. Before proposing or starting an experiment or a probe, look for
  it in the roadmap's [register](ROADMAP.md#experiments-and-probes), and follow
  [its rules](ROADMAP.md#proposing-an-experiment).

## Rules

- Any code is Rust, with a `Cargo.lock`, a pinned toolchain and ignored build output: a probe or
  an evaluation in its own directory under `experiments/`, a provider package, one crate per
  provider, under `packages/` (ruling 40), and the first build, its core and its shell, in the
  workspace under [engine/](engine/README.md) (ruling 46). The only exception
  is the Python checkers in `tools/`.
- The owner makes the rulings. When a choice changes behavior, present the options with a
  recommendation and wait; do not record your recommendation as accepted.
- Under [ruling 57](docs/proposals/ruling-57/README.md), a document that does not bind may be edited, condensed or removed once every piece of relevant Conduit information in it is preserved in `spec/` or a live document, git keeping every version.
- Do not edit the sibling directories of this repository that hold reference clones, if
  present. They are on the owner's machine, to consult only. Nothing in this repository
  needs them.
- Commit or push only when asked, and work on a branch. One standing instruction exists, and it
  is for the sessions the owner has asked to write the language, not for a reviewer or any
  other session: the owner's instruction of 2026-11-05 is that this work is tracked on a pull
  request (ruling 36), and the owner confirmed in ruling 38 that those sessions commit and push
  to its branch at each milestone. Pull request 6 was merged by squashing after ruling 38. Pull
  request 7, on the branch `docs/pre-codex`, carries the work of rulings 39 to 43, the rewrite
  of the specification around provider contracts, the vocabulary audit, work across machines
  and [round 5](docs/proposals/reaudit/round-5/README.md) of the readers. It was merged by
  squashing on 2026-11-07. Pull request 8, on the branch `audit/round-6-codex`, carried
  [round 6](docs/proposals/reaudit/round-6/README.md), the owner's cold read by Codex, with its
  decisions and [ruling 44](docs/proposals/ruling-44/README.md). It was merged by squashing on
  2026-11-07, as `2ca84f3`. Pull request 9, a proposal of the collaborator `allocmemory`, was
  revised under ruling 45 and merged by squashing on 2026-11-09, as `0e937ee`; pull request 10,
  its other proposal, was closed. Pull request 11 carries
  [ruling 45](docs/proposals/ruling-45/README.md) and was merged by squashing the same day, as
  `d46a52b`. Pull request 12, on the branch `docs/roadmap`, moved the roadmap to `ROADMAP.md`
  (A72) and was merged by squashing the same day, as `f3521e7`. Pull request 13 carries
  [ruling 46](docs/proposals/ruling-46/README.md), which shaped the first build, and was merged
  by squashing the same day, as `cdf3236`; each of the build's increments is a branch and a
  pull request of its own. Pull request 15 carries the results of the baseline for S1 (A70),
  merged the same day as `0544b24`, and pull request 16, on the branch `build/loader`,
  increment 1, the strict loader, merged as `d27d5ef`. A work order's branch is `wo/NNN-name`
  ([work orders](docs/implementation/WORK_ORDERS.md)). Later work goes on a new branch, with
  its own pull request. Merges to `main` are squashed, with a message written by hand.

## If you were given a work order

Under [ruling 48](docs/proposals/ruling-48/README.md), the coordinator's session plans and
decides, and Codex sessions carry out work orders, as
[work orders](docs/implementation/WORK_ORDERS.md) describes. If you were started with an order
from `docs/work-orders/`:

- Apply the decisions the order states. Do not reopen them. Where one cannot be applied as
  written, apply the nearest sound reading and say so in your report.
- Change only the files in the order's scope. Report anything else that needs changing.
- Do not commit, push, open a pull request or switch branches: the admin session does.
- Do not edit `~/.claude`, `~/.claude.json`, `~/.codex/config.toml` or any other configuration
  of the owner's, and never read, print or copy a credential.
- Run the order's checks, and the checks below that apply, before you finish.
- Answer in the order's report schema. List apart every choice you made yourself and every
  check's last line, and never report a check you did not run.

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
