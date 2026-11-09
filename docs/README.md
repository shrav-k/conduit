# Documentation

This is the map of every document in the repository: what each one is, and whether it is
current.

Conduit is a configuration language for the relationships between independent agents and the
things they share. Its specification is in [../spec/](../spec/README.md), and the specification
alone binds. Everything under `docs/` explains, records or supports it.

Read in this order: the [README](../README.md), [spec/README.md](../spec/README.md),
[spec/definitions.md](../spec/definitions.md), then the [plan audit](AUDIT.md) for the state of
the work and the owner's rulings, and the [roadmap](../ROADMAP.md) for the order of the work.
Rulings are logged in the plan audit, with a record for each batch under
[proposals/](#records-of-rulings).

## Reading paths

These paths give the design as it stands first. The history of how it got here comes last, in
the records of rulings, and no path needs it. The paths through the specification itself are
in [its reading paths](../spec/README.md#reading-paths), and the rows below link them rather
than repeat them.

| Goal | Path |
| --- | --- |
| Understand Conduit | [README](../README.md) → [spec/README.md](../spec/README.md) → [spec/definitions.md](../spec/definitions.md). The [overview](OVERVIEW.md) is a plain-language tour, and [spec/schemas/examples/](../spec/schemas/examples/) holds worked documents. |
| Write documents, as an author | The [overview](OVERVIEW.md) → the specification's [path for an author of documents](../spec/README.md#an-author-of-documents), with [what access does not narrow](../spec/README.md#what-access-does-not-narrow). |
| Write a contract or a provider | The specification's [path for a writer of contracts and providers](../spec/README.md#a-writer-of-contracts-and-providers). |
| Implement the specification: the reference model, the vectors or a first provider | The specification's [path for an engine builder](../spec/README.md#an-engine-builder). The builders of the two independent engines do not read this branch (next row). |
| Learn the state of the work | [Roadmap](../ROADMAP.md) → [where things stand](AUDIT.md#where-things-stand) → [action items](AUDIT.md#action-items) in the plan audit. |
| Propose an experiment or a probe | [Experiments and probes](../ROADMAP.md#experiments-and-probes) in the roadmap → [proposing an experiment](../ROADMAP.md#proposing-an-experiment) → [experiments/](../experiments/README.md). |
| Understand a ruling | [Ruling log](AUDIT.md#ruling-log) → the [record](#records-of-rulings) of that batch → [research](research/README.md) for the evidence. |
| Contribute | [AGENTS.md](../AGENTS.md) → [contributor guide](../CONTRIBUTING.md). |
| Build an engine | Not from this branch. See [independent builds](implementation/INDEPENDENT_BUILDS.md). Builders work from a separate branch, which is not yet published. |
| See how the design got here | The [archive](archive/README.md), whose index lists, with links, the documents that the specification superseded and that ruling 39 removed from the tree, and the [history of the plan audit](archive/plan-audit-history.md). |

## What each document is

### What binds

| Location | What it is |
| --- | --- |
| [../spec/](../spec/README.md) | The specification: the language, the shared conventions and the implementation interface, as numbered promises, with definitions, JSON Schemas of the documents, example documents and the core package. A draft: every area is written, and no conformance vector exists yet. It alone binds. |

### Entry points and tracker

| Location | What it is |
| --- | --- |
| [../README.md](../README.md) | What Conduit is, what exists, what binds and where to start. |
| [../AGENTS.md](../AGENTS.md) | The rules for agents and contributors: what binds, where things go, what not to edit, and the checks. |
| [../CONTRIBUTING.md](../CONTRIBUTING.md) | Where each kind of work belongs, the checks, the license and project ownership. |
| [../ROADMAP.md](../ROADMAP.md) | The roadmap: the order of the work, with the state of each step, every experiment and probe, planned, running or done, with what it feeds, and the experiment program with its scenarios. Not binding. |
| [AUDIT.md](AUDIT.md) | The plan audit, which tracks the work with the roadmap: where things stand, the owner's ruling log, what waits for the owner, priorities, and the action items that are not experiments or probes. Not a specification: a ruling changes nothing until it is applied to the documents it affects. |
| [OVERVIEW.md](OVERVIEW.md) | A plain-language overview of the language, written from the specification. Not binding. |
| [LANGUAGE_DEFECTS.md](LANGUAGE_DEFECTS.md) | The register of the first sweep: 167 places where the protocol topics, since removed, would have let two engine builders diverge, and what became of each. Findings against `spec/` are in the [re-audit record](proposals/reaudit/README.md). |
| [../tools/](../tools) | The checkers: `check_docs.py` for links and anchors, `set_names.py` for the names `spec/names.yaml` defines, `check_schemas.py` for the schemas, the core package and the examples, `check_roadmap.py` for the rows of the action items in the plan audit and the roadmap, and `check_doc_migration.py`, the record of one past migration. The first four are the checks run before each commit; [what each needs](../CONTRIBUTING.md#checks) is in the contributor guide. |

### Records of rulings

Each record says what was proposed, what was decided, by whom and why. Most hold rulings of
the owner. The documents audit and rounds 2 to 4 of the re-audit hold what the coordinator
decided under ruling 37, which ruling 39 accepted; their ruling column names the ruling that
asked for the work. The record of ruling 41 also holds the work order of the rewrite, whose
planner's choices the owner accepted with P2 changed, and the logs of its editors. The editors'
choices came after ruling 39, no ruling has accepted them, and the owner can strike any of them.
They are listed in the order of the rulings. Where a record and the specification differ, the
specification governs.

| Location | Ruling | What it records |
| --- | --- | --- |
| [proposals/placement/](proposals/placement/README.md) | 7 | Where an instance runs. Accepted; written as promises in `spec/promises/placement.yaml`. Changed by rulings 40 and 41: the machine provider reports the machines and their labels, and places an instance that is given no placement. Ruling 42 renamed location the node. |
| [proposals/instantiation/](proposals/instantiation/README.md) | 8 | What a template lets its instantiator supply. Accepted; written as promises in `spec/promises/instantiation.yaml`. |
| [proposals/normative-surface/](proposals/normative-surface/README.md) | 17 | The form of the specification: what it contains and the form of a promise. Accepted; the form as it now stands is in `spec/README.md`. |
| [proposals/vocabulary/](proposals/vocabulary/README.md) | 17, 18 | The vocabulary. Accepted and applied in `spec/`; the terms in force are in `spec/definitions.md`. Not applied to the documents written before it. |
| [proposals/relationships/](proposals/relationships/README.md) | 19, 20 | First batch of fixes: Conduits and access. Accepted; written as promises in `spec/promises/conduits.yaml` and `spec/conventions/access.yaml`. |
| [proposals/operations/](proposals/operations/README.md) | 20 | Second batch of fixes: operation identity, outcomes and revisions. Accepted; written as promises in `spec/conventions/operations.yaml` and `spec/conventions/revisions.yaml`. Ruling 42 renamed operation identity the idempotency key. |
| [proposals/declarations/](proposals/declarations/README.md) | 21, 22 | Third batch of fixes: references, instantiation details, submissions and Pool entries. Accepted; written as promises under `spec/promises/`. |
| [proposals/graphs-and-schemas/](proposals/graphs-and-schemas/README.md) | 23 | Fourth batch of fixes: the static meaning of graphs and the schema profile. Accepted; the schema profile is written as promises that cite existing standards, in `spec/promises/schemas.yaml`. The graph promises were in `spec/promises/graphs.yaml`, which ruling 41 removed: an instance's own setup and retirement work is a setup hook and a retire hook. [spec/README.md](../spec/README.md#promise-files-that-rulings-removed) lists the IDs that file held. |
| [proposals/packages-and-start/](proposals/packages-and-start/README.md) | 24 | Fifth batch of fixes: packages, first start and conformance. Accepted; the one point held back, how a package's exports are named, was settled by rulings 25 and 27. |
| [proposals/naming/](proposals/naming/README.md) | 25, 27 | How packages, their exports and contracts are named and referred to. Accepted; written as promises in `spec/promises/naming.yaml`. Evidence in [research/naming/](research/naming/README.md). |
| [proposals/consolidation/](proposals/consolidation/README.md) | 27 to 29 | The audit for one idea with several notations, or one key with several meanings: 26 findings, the owner's rulings on all of them, and the choices made while writing them. Applied to the specification. |
| [proposals/conventions/](proposals/conventions/README.md) | 30 | The four remaining shared conventions: fourteen decisions. Accepted; written as promises under `spec/conventions/`. |
| [proposals/schemas-and-interface/](proposals/schemas-and-interface/README.md) | 31 | The shape of documents and contracts, the implementation interface and the implementation-defined list. Accepted, with the owner's amendment on protocol revisions. Its runtimes declared in templates and its `retire` graph were replaced under rulings 40 and 41, by a runtime provider and a retire hook. |
| [proposals/reaudit/](proposals/reaudit/README.md) | 32, 33 | The re-audit of the whole specification, round 1, by three cold readers: their reports, the 37 decisions of ruling 32, the follow-up decisions of ruling 33, and the logs of the passes that applied them. Rounds 2 to 6 are rows of this table, below. |
| [proposals/core-and-schemas/](proposals/core-and-schemas/README.md) | 37 | The core package as documents, the JSON Schemas of every document, and the manifest's format. Accepted with five changes; [how each is applied](proposals/core-and-schemas/APPLY.md). |
| [proposals/docs-audit/](proposals/docs-audit/README.md) | 37 | The audit of the documents outside `spec/`, asked for in ruling 37: two readers' reports and the thirteen decisions taken to make the repository ready for a cold read, all applied. With it are the [list](proposals/docs-audit/GAPS.md) of the behaviors that the older text accepted and the specification lacked, and the logs of the edits: [entry points](proposals/docs-audit/log-entry-points.md), [older text](proposals/docs-audit/log-older-text.md) and [the specification](proposals/docs-audit/log-gaps.md). |
| [proposals/reaudit/round-2/](proposals/reaudit/round-2/README.md) | 30, 37 | Round 2 of the re-audit, done, with its fixes applied. Ruling 30 asked for the specification to be audited again, and ruling 37 says how the findings are resolved. Four cold readers, three of the specification alone and one of the whole repository: their reports, the decisions, the logs of what was done about them, and the check of the interface against the Model Context Protocol. |
| [proposals/reaudit/round-3/](proposals/reaudit/round-3/README.md) | 30, 37 | Round 3 of the re-audit: the same four readers' roles, on the specification as round 2 left it. Their reports, the decisions, and the log of the fix pass. |
| [proposals/reaudit/round-4/](proposals/reaudit/round-4/README.md) | 30, 37 | Round 4 of the re-audit: the reports, the second check of the interface against the Model Context Protocol, the decisions, and the log of the fix pass. |
| [proposals/ruling-38/](proposals/ruling-38/README.md) | 38 | The fourteen points put to the owner after round 4, with the ruling on each in the owner's words, and the log of what it changed in `spec/`. |
| [proposals/ruling-39/](proposals/ruling-39/README.md) | 39 | The fifteen points put to the owner before the cold read by Codex, with the ruling on each in the owner's words. |
| [proposals/ruling-40/](proposals/ruling-40/README.md) | 40 | Conduit innovates only on connection and access, and everything else is a provider behind a contract: the points put to the owner, with the ruling on each in the owner's words, and the owner's answers on where provider packages live and on Experiment 005. |
| [proposals/principles-review/](proposals/principles-review/README.md) | 40, 41 | The review of the whole design against ruling 40: 31 decisions, the alignments the coordinator would make, and the secret type reduced to a contract that any store implements (section 4). Accepted as recommended by ruling 41, and applied in the rewrite of `spec/`. |
| [proposals/secrets/](proposals/secrets/README.md) | 38, 39, 41 | The secret type, revised after the research that ruling 39 asked for ([research packet](research/SECRETS.md)). Settled by ruling 41 as section 4 of the principles review reduces it, and where the two differ, section 4 governs; written as promises in `spec/promises/secrets.yaml`. The proposal is the reasoning and the record of the options. |
| [proposals/ruling-41/](proposals/ruling-41/README.md) | 41 | The ruling on the principles review and the secret type, in the owner's words, and the owner's answer on the planner's choices for the rewrite. With it are the [work order](proposals/ruling-41/PLAN.md) of the rewrite, the logs of its editors and the [integration log](proposals/ruling-41/log-integration.md), whose questions the [open questions after the rewrite](proposals/ruling-41/open-decisions.md) and rulings 42 and 43 settled. |
| [proposals/vocabulary-audit/](proposals/vocabulary-audit/README.md) | 42 | The vocabulary audit of the rewritten specification, which ruling 41 asked for: twenty names with their precedents, accepted by ruling 42, and the editorial fixes. |
| [proposals/ruling-42/](proposals/ruling-42/README.md) | 42 | The ruling on the questions the rewrite left open ([open questions](proposals/ruling-41/open-decisions.md)) and on the names, in the owner's words, with the logs of the pass that applies it. |
| [proposals/multi-machine/](proposals/multi-machine/README.md) | 42 | How Conduit works across machines: answers to D5 and D20 and five smaller questions, with v0 and v1 kept apart, after the [research on work across machines](research/MULTI_MACHINE.md). Accepted, with two revisions the coordinator made in explaining it, by ruling 43. |
| [proposals/ruling-43/](proposals/ruling-43/README.md) | 43 | The ruling on work across machines, in the owner's words, with the logs of the pass that applies it. |
| [proposals/reaudit/round-5/](proposals/reaudit/round-5/README.md) | 37, 38, 41 | Round 5 of the re-audit, which ruling 41 ordered: six readers of the specification as rulings 40 to 43 rewrote it, among them one of the whole repository and one who checked the interface against the Model Context Protocol. Their reports, the coordinator's decisions, the logs of the fix pass and the verification of the fixes. No ruling has accepted its decisions, and the owner can strike any of them. |
| [proposals/reaudit/round-6/](proposals/reaudit/round-6/README.md) | 37, 38, 44 | Round 6 of the re-audit, the owner's cold read by Codex: one reader of the whole repository at `c4d7d33`, with six findings and a critique of the design's simplicity and usefulness in 22 points. With them are the coordinator's [decisions](proposals/reaudit/round-6/DECISIONS.md), which ruling 44 accepted, and the logs of the pass that applied them. |
| [proposals/ruling-44/](proposals/ruling-44/README.md) | 44 | The ruling on round 6 and on building next, in the owner's words: the next milestone is the first build, and the coordinator's recommendations on the findings and the critique are accepted. |
| [proposals/ruling-45/](proposals/ruling-45/README.md) | 45 | The ruling on two proposals of the collaborator `allocmemory`, pull requests 9 and 10, in the owner's words: what was kept of each, and where it went, and the pass that decides how the roadmap is shown, with [its log](proposals/ruling-45/log-roadmap.md). |

### Process for the builds

| Location | What it is |
| --- | --- |
| [implementation/INDEPENDENT_BUILDS.md](implementation/INDEPENDENT_BUILDS.md) | The process, the independence rules and the readiness gate for the two independent builds. The roles and rules are accepted (ruling 11). The builds have not started. |
| [implementation/ENGINE_BUILD_BRIEF.md](implementation/ENGINE_BUILD_BRIEF.md) | The brief each builder receives. A draft; not issued. |

### Evidence and history

| Location | What it is |
| --- | --- |
| [research/](research/README.md) | Dated primary sources, source pins, comparisons and probe results. Evidence for the rulings; nothing in it binds, and every packet but the naming research, the secrets research and the research on work across machines predates the specification. |
| [archive/](archive/README.md) | Closed reviews, the accounting of one migration, attributed collaborator proposals including dissent, the [history of the plan audit](archive/plan-audit-history.md), and the [list of documents removed from the tree](archive/README.md#removed-from-the-tree) with links to their last versions. History; nothing in it binds. |
| [../experiments/](../experiments/README.md) | Measured evidence. Two experiments, both in Rust: a model-checking evaluation (004) and a probe of the macOS keychain, a candidate store for the first providers (005); the index says what became of 001 to 003. No experiment has run a Conduit engine, because none exists. Every experiment and probe, planned, running or done, has its row in the roadmap's [register](../ROADMAP.md#experiments-and-probes). |

## Repository boundaries

One main repository is accepted. It uses Apache-2.0 for original material, and permanent public
naming remains deferred under the
[publication policy](../CONTRIBUTING.md#project-ownership-and-publication).

The repository holds no engine code and no release scheme. The top-level directories
`conformance/`, `reference-model/` and `packages/` are planned and do not exist yet; see
[independent builds](implementation/INDEPENDENT_BUILDS.md#repository-layout-and-isolation).
`packages/` is to hold the first providers, one Rust crate per provider (ruling 40).
There is one specification, `spec/`, and nothing else is to become a second one. A website, if
one is made, renders these sources.

## Maintaining the design

Accepted behavior is a promise in `spec/`, in the terms of `spec/definitions.md`. A change needs
an owner ruling, recorded in the [ruling log](AUDIT.md#ruling-log) of the plan audit with its
record under `proposals/`. The one other way is a decision on a finding of the re-audit, under
ruling 37, which is logged as the coordinator's
([who decides what](AUDIT.md#who-decides-what)).

Keep a proposal that is not yet ruled under `proposals/`, a source claim in `research/`, and a
test or a result in `../experiments/`. After a ruling, change the promises, the definitions, the
schemas and the examples together, and run the [checks](../CONTRIBUTING.md#checks).

The [migration record](archive/migrations/2026-11-protocol-framing.md) locates the paths used
before 2026-11, and the
[vocabulary-preparation audit](archive/migrations/2026-11-vocabulary-audit.md) records the later
index cleanup and the stable `vxx` hooks. Research and archive documents keep the words they
were written in.
