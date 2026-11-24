# Work order 306: only the engine must be Rust; a package may be written in any language (DEC-111)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
is empty, as this order adds no case. It needs no network, changes no source and runs no cargo
command, and not `tools/gate.py`. Never print a whole large file, case, diff or list: use `head`,
`tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.
Run no `conduit` outside a test, no `codex` or `claude`, and sign in nowhere. Prefix each
`python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user, machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-111](../decisions/log.md#dec-111-2026-11-23-only-the-runtime-engine-must-be-rust):
"The web ui can be built from anything, conduit packages do not need to be in rust, only the
runtime engine must be". It lands in HOW-WE-WORK's language rule: the engine is Rust; a package
may be written in any language; it amends DEC-025. The web UI is a package (DEC-101, DEC-109,
DEC-110), the first that need not be Rust.

**Where the rule holds today.** HOW-WE-WORK.md is a draft: until AGENTS.md "What holds" names
it, the rules hold where they are stated today (its header). So the rule changes in HOW-WE-WORK
rule 2 and in each place that states it now. A grep of the tree for "Rust", "rust-only", "Rust
only" and "in Rust" found these, each saying that packages, or all code, are Rust:

| File and line | Says |
| --- | --- |
| `HOW-WE-WORK.md:22` | rule 2: "Code is Rust in the tree, tested against the real binary" |
| `AGENTS.md:88-92` | "**Code is Rust**, with a `Cargo.lock`, …, a provider package under `packages/`, one crate per provider (ruling 40)" |
| `CONTRIBUTING.md:73-79` | "**Code** is Rust, with a lockfile, …; a provider package, one crate per provider, under packages/" |
| `CONTRIBUTING.md:416-417` | "The in-tree Rust convention above applies to prototype providers." |
| `IN_FORCE.md:92` | "Providers and the Claude and Codex Agent types are self-contained Rust crates under `packages/` through v0's validation." |
| `IN_FORCE.md:347` | "Exploratory code needs no permission, and all code is Rust (3)." |
| `IN_FORCE.md:378` | ruling 92 point 2, the source of rule 2: "code the coordinator writes is Rust in the tree, tested against the real binary, …, the Python checkers in `tools/` excepted" |
| `docs/clients.md:204-205` | "Code here is Rust, apart from `tools/`'s Python checkers and the Workflow tool's JavaScript (DEC-025)." |
| `docs/AUDIT.md:493` | A63: "as packages in Rust under a top-level `packages/` directory, one crate per provider (ruling 40, after the ruling)" |
| `docs/implementation/INDEPENDENT_BUILDS.md:89` | "The first providers, one Rust crate per provider (ruling 40)" |

`packages/README.md` does not exist. `spec/` names no language. No check enforces Rust for a
package: `tools/*.py`, `tools/tests/` and `.github/workflows/` hold no language check, and
`rust_manifests` in `tools/gate.py` and the discovery step of `checks.yml` find Rust directories
by their `Cargo.toml` and fail nothing that has none.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 4,
nothing assumes a particular vendor's agent; read with ruling 40 (everything but connection and
access is a provider behind a contract any existing tool can implement), nothing assumes one
language's package either. None is at risk: the rule is about the work, not Conduit's behavior,
and a package in any language meets the same contracts and promises (DEC-101).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: Terraform's core is Go, and each provider is a plugin process behind a gRPC
protocol, its language its author's; the Language Server Protocol and the Model Context Protocol
fix the wire and leave each server's language to its author. Not taken: a list of allowed
languages, or a check that refuses one; widening the rule to `tools/`, the Workflow tool's
JavaScript, the coordinator's tooling or experiments.

**When it runs.** On `main`, which holds DEC-111 since #581, or on the decision log's branch,
which changes only the log. Through Conduit: one Codex worker turn, the blind Codex verifier and
Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, the
decision says what must hold; how is yours. The words and these choices are the coordinator's,
open to the owner's strike (DEC-029): the rule changes everywhere the table names, not in
HOW-WE-WORK.md alone; other code stays Rust, experiments and tooling included; each file keeps its
own named exceptions (`tools/`'s Python checkers everywhere, the Workflow tool's JavaScript where
the text names it now); a package in any language keeps the hygiene asked of Rust code, a
lockfile if it has dependencies, a pinned toolchain and ignored build output; the existing
packages stay Rust crates; and no `packages/README.md` is created. Read literally, "only the
runtime engine must be" frees experiments and tooling too; this order reads it within its
question, the web page's script, and the packages the answer names, and keeps DEC-025 for all
other code. Where DEC-086 says DEC-025's rule "covers the engine, the packages and the tooling",
DEC-111 takes the packages out.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), a file outside Scope that must change, or
a check under `tools/` or `.github/workflows/` that fails a package for its language (give its
file and line; the start check finds none). Where this order's wording contradicts a promise it
does not change, the promise governs: apply it and list it under `own_choices`, with every choice
no decision governs.

## The decisions

Wrap each insertion at its file's width. Every other sentence of each paragraph stays as it is.

1. **HOW-WE-WORK.md rule 2** (line 22), one line, becomes:

   ```text
   2. **No private code.** The engine is Rust, and a package may be written in any language; other code in the tree is Rust, and the Python checkers in `tools/` and the Workflow tool's JavaScript are the named exceptions. Code in the tree is tested against the real binary, private values live only in configuration outside the tree, and a gap in the engine is a finding and an order, never a script. (DEC-025, DEC-028, DEC-111, ruling 92 pt 2) `by review`
   ```

2. **AGENTS.md, "Rules"**: the bullet "**Code is Rust**" (lines 88-92) becomes the text below,
   which wraps at width 100 to exactly its five lines, the longest 97 characters (the file is over
   its 150-line cap and must not grow):

   ```text
   - **Code**: the engine is Rust, and a package may be written in any language; other code is Rust ([DEC-111](docs/decisions/log.md#dec-111-2026-11-23-only-the-runtime-engine-must-be-rust)), the Python checkers in `tools/` and the Workflow tool's JavaScript apart (DEC-025). Each keeps a lockfile if it has dependencies, a pinned toolchain and ignored build output: a probe in `experiments/`, a package in `packages/` (ruling 40), [engine/](engine/README.md) (ruling 46).
   ```

3. **CONTRIBUTING.md**: the bullet "**Code** is Rust" (lines 73-79) becomes the text below, and
   the sentence "The in-tree Rust convention above applies to prototype providers." (lines
   416-417) is removed:

   ```text
   - **Code**: the engine is Rust, and a package may be written in any language; other code is Rust ([DEC-111](docs/decisions/log.md#dec-111-2026-11-23-only-the-runtime-engine-must-be-rust)). Each keeps a lockfile if it has dependencies, a pinned toolchain and ignored build output: a probe or an evaluation in its own directory under [experiments/](experiments/README.md); a provider package, one per provider, under [packages/](packages/) (ruling 40), which as of 2026-11-10 holds the Claude Code and Codex Agent packages; and the first build, its core and its shell, in the workspace under [engine/](engine/README.md) (ruling 46). The only exception is the Python checkers in `tools/`.
   ```

4. **IN_FORCE.md**, in place, adding no bullet:
   - line 92, its two sentences: "Providers and the Claude and Codex Agent types are
     self-contained packages under `packages/` through v0's validation. The published package
     format depends on neither the language nor the repository layout, and a package, in the tree
     or by an outside author, may be written in any language (40, 47; DEC-111)."
   - line 347, its first sentence: "Exploratory code needs no permission (3). The engine is Rust,
     and a package may be written in any language; other code is Rust (3, as DEC-111 amends it)."
   - line 378, ruling 92 point 2: "code the coordinator writes is Rust in the tree, tested
     against the real binary" becomes "code the coordinator writes is Rust in the tree, or any
     language in a package (DEC-111), tested against the real binary".
5. **`docs/clients.md`** (lines 204-205): the sentence "Code here is Rust, …" becomes "Here the
   engine is Rust, and a package may be written in any language; other code is Rust, apart from
   `tools/`'s Python checkers and the Workflow tool's JavaScript (DEC-025, DEC-111)."
6. **`docs/AUDIT.md`** A63 (line 493): "as packages in Rust under a top-level `packages/`
   directory, one crate per provider (ruling 40, after the ruling)" becomes "as packages under a
   top-level `packages/` directory, one per provider, each in any language (ruling 40, after the
   ruling; DEC-111)". **`docs/implementation/INDEPENDENT_BUILDS.md`** (line 89): "The first
   providers, one Rust crate per provider (ruling 40)" becomes "The first providers, one package
   per provider, each in any language (ruling 40, DEC-111)".
7. **Kept as they stand**, also found by the grep:
   - `docs/implementation/METHOD_AS_RUN.md:8`, "the tree's code is Rust": why the coordinator's
     runners stay outside the tree; the coordinator's tooling keeps what its text says.
   - `experiments/README.md:12` and `experiments/019-s8-filtered-search/plan.md:70`: experiments
     stay Rust. (`experiments/README.md:14-15` is under What follows.)
   - The engine, which stays Rust: `docs/implementation/INDEPENDENT_BUILDS.md:21`, `:88` and
     `:103`, `docs/implementation/ENGINE_BUILD_BRIEF.md:17`, `docs/implementation/FIRST_BUILD.md:11`,
     `docs/AUDIT.md:484`, `:485`, `:490` and `:533`, `ROADMAP.md:53` and `:140`,
     `engine/README.md:7`, `README.md:36` and `:38`.
   - Facts about packages that are Rust crates: `packages/agent-claude-code/README.md:7`,
     `packages/agent-codex/README.md:8`, `docs/implementation/WORK_ORDERS.md:1097`.
   - How the checks build Rust directories, not which code is Rust: `AGENTS.md:174-224`,
     `CONTRIBUTING.md:232-248`, `:291` and `:324-336`, `docs/clients.md:104` and `:214`.
   - Records: `docs/decisions/log.md`, `docs/research/model-access-gateways.md:165`,
     `docs/work-orders/` and `docs/archive/`.

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
grep -c '^## DEC-111, ' docs/decisions/log.md  # 1
sed -n '/^## DEC-111, /,/^## DEC-112, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
echo "$(j HOW-WE-WORK.md 'Code is Rust in the tree') $(j AGENTS.md '**Code is Rust**') $(j CONTRIBUTING.md '**Code** is Rust') $(j CONTRIBUTING.md 'The in-tree Rust convention') $(j IN_FORCE.md 'self-contained Rust crates') $(j IN_FORCE.md 'all code is Rust') $(j IN_FORCE.md 'code the coordinator writes is Rust in the tree, tested') $(j docs/clients.md 'Code here is Rust') $(j docs/AUDIT.md 'as packages in Rust') $(j docs/implementation/INDEPENDENT_BUILDS.md 'one Rust crate per provider')"  # 1 1 1 1 1 1 1 1 1 1
grep -c '^2\. \*\*No private code\.\*\* Code is Rust in the tree, .*(DEC-025, DEC-028, ruling 92 pt 2) `by review`$' HOW-WE-WORK.md  # 1
grep -rliE 'non-rust|rust only|rust-only|must be rust|only rust' tools .github | wc -l | tr -d ' '  # 0
ls packages/README.md 2>/dev/null | wc -l | tr -d ' '  # 0
echo "$(wc -l < AGENTS.md | tr -d ' ') $(grep -cE '^[[:space:]]*[-*] ' IN_FORCE.md)"  # 225 344
```

## Scope

Change, as the decisions say and in the lines named and their paragraph's rewrap alone:
`HOW-WE-WORK.md`, `AGENTS.md`, `CONTRIBUTING.md`, `IN_FORCE.md`, `docs/clients.md`,
`docs/AUDIT.md` and `docs/implementation/INDEPENDENT_BUILDS.md`. Not the decision log (DEC-111's Status line is the
coordinator's), `spec/`, `engine/`, `packages/`, `experiments/`, `tools/`, `.github/`,
`docs/work-orders/` or `docs/archive/`, and no new file. Where a check fails on a line outside
these, report it and change nothing else.

## Done when

Each line prints otherwise before this order and passes after, with `j` as above:

```sh
echo "$(j HOW-WE-WORK.md 'Code is Rust in the tree') $(j AGENTS.md '**Code is Rust**') $(j CONTRIBUTING.md '**Code** is Rust') $(j CONTRIBUTING.md 'The in-tree Rust convention') $(j IN_FORCE.md 'self-contained Rust crates') $(j IN_FORCE.md 'all code is Rust') $(j IN_FORCE.md 'code the coordinator writes is Rust in the tree, tested') $(j docs/clients.md 'Code here is Rust') $(j docs/AUDIT.md 'as packages in Rust') $(j docs/implementation/INDEPENDENT_BUILDS.md 'one Rust crate per provider')"  # ten 0s; ten 1s before
for f in HOW-WE-WORK.md AGENTS.md CONTRIBUTING.md IN_FORCE.md docs/clients.md; do j "$f" 'engine is Rust, and a package may be written in any language; other code'; done | paste -sd' ' -  # 1 1 1 1 1; 0 0 0 0 0 before
grep -c '^2\. \*\*No private code\.\*\* The engine is Rust, .*(DEC-025, DEC-028, DEC-111, ruling 92 pt 2) `by review`$' HOW-WE-WORK.md  # 1; 0 before
for f in AGENTS.md CONTRIBUTING.md; do j "$f" '(docs/decisions/log.md#dec-111-2026-11-23-only-the-runtime-engine-must-be-rust)'; done | paste -sd' ' -  # 1 1; 0 0 before
echo "$(j IN_FORCE.md 'a package, in the tree or by an outside author, may be written in any language (40, 47; DEC-111).') $(j IN_FORCE.md 'or any language in a package (DEC-111), tested against the real binary') $(j docs/clients.md '(DEC-025, DEC-111).') $(j docs/AUDIT.md 'one per provider, each in any language (ruling 40, after the ruling; DEC-111)') $(j docs/implementation/INDEPENDENT_BUILDS.md 'one package per provider, each in any language (ruling 40, DEC-111)')"  # 1 1 1 1 1; 0 0 0 0 0 before
j IN_FORCE.md 'self-contained packages under `packages/` through v0'  # 1; 0 before
```

Guards, which pass before and must still pass. No cargo command: the order changes nothing under
`engine/`, `spec/`, `conformance/`, `packages/`, `experiments/` or `tools/`, so the gate chooses
no Rust row and no tool tests for it:

```sh
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # head equals base; Errors: 0
python3 tools/check_decisions.py HEAD WORKTREE | grep -c ': HOW-WE-WORK form:'  # 2, lines 38 and 40, as before
for t in check_docs check_roadmap check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, five times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 0 changes; 0 flips; 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vxE '(HOW-WE-WORK|AGENTS|CONTRIBUTING|IN_FORCE)\.md|docs/(AUDIT|clients)\.md|docs/implementation/INDEPENDENT_BUILDS\.md' | wc -l | tr -d ' '  # 0
echo "$(wc -l < AGENTS.md | tr -d ' ') $(grep -cE '^[[:space:]]*[-*] ' IN_FORCE.md)"  # 225 344, or fewer lines in AGENTS.md
```

Report each changed sentence, its file and line, and every check's last line.

## What follows

- DEC-111's Status line: the coordinator's, at the merge, naming the pull request; the rule is
  all it lands in.
- The gate and CI find a package's tests by its `Cargo.toml`, so a package in another language
  gets no test row, and nothing fails. The first such package's order (the web UI package of
  DEC-109 and DEC-110, if it is not Rust) adds its row; that is a `tools/` and
  `.github/workflows/` order.
- Experiment 018 (S7): its open Q4, where a non-Rust implementation lives, and its plan's line
  "No TypeScript is written by an order in this repository" are S7's to settle in DEC-111's
  light; `experiments/021-s10-swapped-providers/README.md:196`, "Provider packages are Rust
  crates, one per provider", is S10's description. The coordinator places both when those
  scenarios are planned.
- `experiments/README.md:14-15`, "A provider package is not an experiment: it goes under a
  top-level `packages/` directory, one crate per provider (ruling 40)", describes today's
  packages; it is the experiments index's to update, with the experiments' next order.
- Once AGENTS.md "What holds" names HOW-WE-WORK.md, rule 2 is the language rule's one home, and
  AGENTS.md, CONTRIBUTING.md and IN_FORCE.md cite it instead of restating it (HOW-WE-WORK rule 7).

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` at its start.

**Worker-2's turn**, on the primary ChatGPT account, applied decisions 1 to 6 to seven files.

**Two blind reads.** The Codex verifier, `root-verifier-2.turn.v1`, passed it with no finding. The
Claude reader, `root-reader-a.turn.v1`, on the Team seat, passed it with two shoulds and two nits:
the rule did not say what still binds a package in another language; it cited DEC-111 alone for
"other code is Rust", which is DEC-132's narrowing; it made `Cargo.lock` conditional for Rust code;
and its list of places named only provider packages. The coordinator fixed all four in
`AGENTS.md` and `CONTRIBUTING.md` (ruling 87), keeping the decisions' prescribed phrase, and reran
every Done-when line. The gate's run on the squash is the coordinator's, at the merge.
