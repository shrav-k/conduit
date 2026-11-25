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
6. For a coordinator taking over: [the handover](docs/implementation/HANDOVER.md), then
   [the full review](docs/guides/full-review.md) the owner asked for.

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
3. **[The ledger](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-log.md)**
   says what the owner ruled, when, in the owner's words, and each ruling's status.
4. **A ruling's record**, [beside the ledger](https://github.com/shrav-k/conduit/tree/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals),
   says why. Its first line says whether it still holds.

The ledger and the records are archived at `077fa47e`: work order 235 took them out of the tree,
git keeps them, and each link to them names that commit. They hold as items 3 and 4 say until
IN_FORCE.md is retired, and nothing is added to them.

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

- **An owner decision** goes in the [decision log](docs/decisions/log.md), using its
  [entry form](docs/decisions/README.md#the-log), with the lines of IN_FORCE.md it changes and any
  action item. An answer that confirms or corrects a decision is a new entry citing it.
- **Accepted behavior**: a promise in `spec/`, in the terms of `spec/definitions.md`. It changes by
  a logged ruling.
- **A defect in the language**: fix it in the language, not around it.
- **A finding of the first build** goes in the [open log](docs/implementation/first-build/log-increment-4.md)
  and its fix in the specification with a vector (ruling 46, point 4).
- **Evidence**: its own directory under `experiments/`, with dated results. Never present an
  expected outcome as a result. Before proposing an experiment or a probe, look for it in the
  roadmap's [register](ROADMAP.md#experiments-and-probes), and follow
  [its rules](ROADMAP.md#proposing-an-experiment).

## Rules

- **Code**: the engine is Rust, and a package may be written in any language; other code is Rust
  ([DEC-111](docs/decisions/log.md#dec-111-2026-11-23-only-the-runtime-engine-must-be-rust);
  [DEC-132](docs/decisions/log.md#dec-132-2026-11-23-dec-111-frees-packages-only) keeps experiments
  and tooling in Rust), the Python checkers in `tools/` and the Workflow tool's JavaScript apart
  (DEC-025). A package in another language still carries its manifest (`conduit.yaml`) and its
  contracts (`contracts.yaml`), meets the conformance cases of what it provides, and, if it is a
  client, keeps [the client contract](docs/clients.md). Rust code keeps a `Cargo.lock` and a pinned
  toolchain (`rust-toolchain.toml`); code in another language keeps its own lockfile and pins its
  toolchain's version. All of it ignores its build output. Where code goes: a probe in
  `experiments/`, a package in `packages/` (a provider, one per provider, an Agent or an Interface
  such as the web UI; ruling 40), [engine/](engine/README.md) (ruling 46).
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

Under [ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md), the coordinator's session plans and decides,
and Codex sessions carry out work orders, as [work orders](docs/implementation/WORK_ORDERS.md)
describes. If you were started with an order from `docs/work-orders/`:

- Apply the decisions the order states. Do not reopen them. Where one cannot be applied as written,
  apply the nearest sound reading and say so in your report.
- Change only the files in the order's scope. Report anything else that needs changing.
- An order that changes `spec/` writes its changelog lines in its own fragment under
  `spec/changes/`, whatever its scope lists, and adds no count to a README
  ([HOW-WE-WORK section 8](HOW-WE-WORK.md#8-records-where-each-fact-lives)).
- An order that lists a case the build does not pass yet writes the entry in its own fragment
  under `conformance/expected-failures/`, named for its order file, whatever its scope lists;
  an order that makes a listed case pass removes the entry from the fragment that holds it,
  whatever its scope lists, and deletes a fragment that this empties. The order's scope may name
  that fragment
  ([HOW-WE-WORK section 8](HOW-WE-WORK.md#8-records-where-each-fact-lives)).
- Do not commit, push, open a pull request or switch branches: the admin session does.
- Do not edit `~/.claude`, `~/.claude.json`, `~/.codex/config.toml` or any other configuration of
  the owner's, and never read, print or copy a credential.
- Run the order's checks, and the checks below that apply, before you finish.
- Run the whole engine suite at most once, at the end; a verifier or reader runs only what settles
  a doubt. The gate runs everything before each merge
  ([DEC-033](docs/decisions/log.md#dec-033-2026-11-20-the-testing-audit)).
- An order that changes the text of a promise experiment 012 models marks that row of
  `experiments/012-model-checking/account.yaml` "text changed, not rerun", whatever its scope says
  ([DEC-033](docs/decisions/log.md#dec-033-2026-11-20-the-testing-audit)).
- Answer in the order's report schema. List apart every choice you made yourself and every check's
  last line, and never report a check you did not run.

## Checks

Run these from the repository root before each commit:

```sh
python3 tools/check_docs.py          # links, anchors and code fences in every Markdown file
python3 tools/set_names.py --check   # the names under spec/, engine/, conformance/ and packages/ agree with spec/names.yaml
python3 tools/check_schemas.py       # schemas, core package and examples; needs PyYAML and jsonschema
python3 tools/check_roadmap.py       # one row per action item, and the roadmap cites each open one
python3 tools/check_model_account.py # the model-checking account has a row for each convention-layer promise, and none is stale
python3 tools/promise_standing.py --check # engine/STANDING.md agrees with the promises, the case tags, the expected failures and the engine's sources
python3 tools/check_private_paths.py # tracked files contain no private paths, email addresses outside reserved domains, or configured names
git diff --check                     # whitespace errors
(cd engine && cargo test --release)   # the first build: loader, core, vectors and shell
python3 tools/gate.py                # CI's checks, run here for what the branch changes; merges wait on its PASS
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
- `check_private_paths.py` needs Python 3 and Git; `CONDUIT_PRIVATE_NAMES` optionally lists private names, separated by commas.
- `check_model_account.py` needs Python 3 and PyYAML, and runs itself again under
  `/usr/bin/python3` when its `python3` lacks PyYAML, as `check_schemas.py` does. With
  `--history` it also needs Git and the history back to each row's `read`.
- `promise_standing.py` needs Python 3 and PyYAML, and runs itself again under
  `/usr/bin/python3` when its `python3` lacks PyYAML, as `check_model_account.py` does. A
  change to a promise, to a case's `promises`, to the expected failures, to an S1 row or to a
  promise ID in the sources of the loader, core or shell changes `engine/STANDING.md`: run
  the tool with `--write` in the same change, whatever files an order's scope lists, so that
  the file is committed with the change.
- `cargo test --release` in `engine/` needs Rust through rustup, which installs on
  first use the toolchain that `engine/rust-toolchain.toml` pins. Run it from inside
  the directory, so that the pin applies. For a change to its code, run
  `cargo clippy --all-targets -- -D warnings` there too. The tests read `spec/`, and check the
  loader's verdict on every example against the verdicts of `check_schemas.py`, which a
  fixture records.
- `gate.py` needs Python 3 and Git, and what each check it runs needs: the items above
  and, for each Rust directory it runs, the toolchain that directory pins installed
  through rustup and its crates in cargo's cache, since it runs cargo offline and lets
  rustup install nothing. Its discovery runs cargo from the root, which pins no toolchain,
  so it needs rustup's default toolchain too. When its `python3` lacks PyYAML or jsonschema
  it runs itself again under `/usr/bin/python3`, as `check_model_account.py` does. It
  compares the working tree with `origin/main` as last fetched, or with `--base REF`, and
  never fetches, pushes or calls GitHub. It fails a commit since the base whose author or
  committer is neither `Conduit work order <work-orders@conduit.invalid>` nor a contributor that
  `CONTRIBUTORS` lists at the merge base.
  `--target-cache DIR` uses one target directory per Rust directory, rebuilds the tree's own
  crates, and holds `DIR/gate.lock` for the run. Before its first build, with or without that
  option, it sets the time of every file Git lists in the tree, tracked or untracked and not
  ignored, to one instant, so that cargo rebuilds each crate built from the tree over any build
  a target directory held before that instant, whatever times the files carried; the record's
  `freshness` names it (order 402). If listing or setting file times fails, no clippy or test
  command runs: those rows FAIL, or are not run after the merge queue's first failure.
  Without `--target-cache` no gate lock holds, so a build made by another tree's gate, an editor
  or a manual command in a shared target directory after that instant can still be reused.
  Hand builds in DIR's folders bypass the gate's lock and preparation; a clock set back can
  also defeat time comparisons. Cite a `--target-cache` record: its lock and fingerprint
  removal narrow these gaps, but cannot guard against hand builds that bypass the gate.
  `--merge-queue` runs the cheap rows first,
  stops at the first required failure and runs test rows side by side; every record holds the
  load average (DEC-062). The tool tests run with `--all` or when a change
  touches `tools/`, `.github/workflows/`, or a Rust source, `Cargo.toml` or `Cargo.lock` under
  `packages/` or `experiments/`.
  With `--derive-private-names` it derives the private names from the machine it runs on,
  at each run and with no file: the OS account's short and full names, the host names (on
  macOS the computer and local host names too), git's global `user.name` and the local part
  of its `user.email`, the names of the Git repositories beside this one and of their
  remotes, and the host names of the tailnet's peers where `tailscale` is installed. It
  drops a name shorter than 4 characters, one in the origin remote's URL and one on its list
  of public words, and skips a source it cannot read, naming the source. With
  `--private-names-file` it also reads names from that file, of mode 0600 and outside the
  repository, and drops none of them. It gives the names to `check_private_paths.py` alone,
  redacts them from its output and logs, and prints only how many it derived and from which
  sources; the `CONDUIT_PRIVATE_NAMES` of its own environment it does not use.
  It runs gitleaks only when the release that `.github/workflows/checks.yml` pins is on the
  `PATH`, and otherwise says that it did not. Its last line says PASS only when every
  required gate passed, and it writes its record under `target/gate/`. `--plan` runs
  `rust discovery` alone and lists the other gates without running them, and exits 1 when
  discovery does not pass; `--all` runs every Rust directory and the engine. The admin merges
  on its PASS on the Mac, and CI runs after, as a second opinion ([ruling 80](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-80/README.md)).

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
