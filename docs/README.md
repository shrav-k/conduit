# Documentation

This is the map of every document in the repository: what each one is, and whether it is
current.

Conduit is a configuration language for the relationships between independent agents and the
things they share. Its specification is in [../spec/](../spec/README.md), and the specification
alone binds. Everything under `docs/` explains, records or supports it.

Read in this order: the [README](../README.md), [spec/README.md](../spec/README.md),
[spec/definitions.md](../spec/definitions.md), then the [plan audit](AUDIT.md) for the state of
the work and the owner's rulings. Rulings are logged in the plan audit, with a record for each
batch under [proposals/](#records-of-rulings).

## Reading paths

| Goal | Path |
| --- | --- |
| Understand Conduit | [README](../README.md) → [spec/README.md](../spec/README.md) → [spec/definitions.md](../spec/definitions.md). The [overview](OVERVIEW.md) is a plain-language tour, and [spec/schemas/examples/](../spec/schemas/examples/) holds worked documents. |
| Learn the state of the work | [Where things stand](AUDIT.md#where-things-stand) → [action items](AUDIT.md#action-items) in the plan audit. |
| Understand a ruling | [Ruling log](AUDIT.md#ruling-log) → the [record](#records-of-rulings) of that batch → [research](research/README.md) for the evidence. |
| Contribute | [AGENTS.md](../AGENTS.md) → [contributor guide](../CONTRIBUTING.md). |
| Build an engine | Not from this branch. See [independent builds](implementation/INDEPENDENT_BUILDS.md). Builders work from a separate branch, which is not yet published. |
| See how the design got here | [protocol/](protocol/README.md), superseded → [archive](archive/README.md). |

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
| [AUDIT.md](AUDIT.md) | The plan audit. The one tracker: where things stand, the owner's ruling log, priorities, action items and the experiment program. Not a specification: a ruling changes nothing until it is applied to the documents it affects. |
| [OVERVIEW.md](OVERVIEW.md) | A plain-language overview of the language, written from the specification. Not binding. |
| [LANGUAGE_DEFECTS.md](LANGUAGE_DEFECTS.md) | The register of the first sweep: 167 places where `docs/protocol/` would have let two engine builders diverge, and what became of each. Findings against `spec/` are in the [re-audit record](proposals/reaudit/README.md). |
| [../tools/](../tools) | The checkers: `check_docs.py` for links and anchors, `set_names.py` for the names `spec/names.yaml` defines, `check_schemas.py` for the schemas, the core package and the examples, and `check_doc_migration.py`, the record of one past migration. |

### Records of rulings

Each record says what was proposed, what the owner ruled and why. They are listed in the order
of the rulings. Where a record and the specification differ, the specification governs.

| Location | Ruling | What it records |
| --- | --- | --- |
| [proposals/placement/](proposals/placement/README.md) | 7 | Where an instance runs. Accepted; written as promises in `spec/promises/placement.yaml`. |
| [proposals/instantiation/](proposals/instantiation/README.md) | 8 | What a template lets its instantiator supply. Accepted; written as promises in `spec/promises/instantiation.yaml`. |
| [proposals/normative-surface/](proposals/normative-surface/README.md) | 17 | The form of the specification: what it contains and the form of a promise. Accepted; the form as it now stands is in `spec/README.md`. |
| [proposals/vocabulary/](proposals/vocabulary/README.md) | 17, 18 | The vocabulary. Accepted and applied in `spec/`; the terms in force are in `spec/definitions.md`. Not applied to the documents written before it. |
| [proposals/relationships/](proposals/relationships/README.md) | 19, 20 | First batch of fixes: Conduits and access. Accepted; written as promises in `spec/promises/conduits.yaml` and `spec/conventions/access.yaml`. |
| [proposals/operations/](proposals/operations/README.md) | 20 | Second batch of fixes: operation identity, outcomes and revisions. Accepted; written as promises in `spec/conventions/operations.yaml` and `spec/conventions/revisions.yaml`. |
| [proposals/declarations/](proposals/declarations/README.md) | 21, 22 | Third batch of fixes: references, instantiation details, submissions and Pool entries. Accepted; written as promises under `spec/promises/`. |
| [proposals/graphs-and-schemas/](proposals/graphs-and-schemas/README.md) | 23 | Fourth batch of fixes: the static meaning of graphs and the schema profile. Accepted; written as promises that cite existing standards, in `spec/promises/graphs.yaml` and `spec/promises/schemas.yaml`. |
| [proposals/packages-and-start/](proposals/packages-and-start/README.md) | 24 | Fifth batch of fixes: packages, first start and conformance. Accepted; the one point held back, how a package's exports are named, was settled by rulings 25 and 27. |
| [proposals/naming/](proposals/naming/README.md) | 25, 27 | How packages, their exports and contracts are named and referred to. Accepted; written as promises in `spec/promises/naming.yaml`. Evidence in [research/naming/](research/naming/README.md). |
| [proposals/consolidation/](proposals/consolidation/README.md) | 27 to 29 | The audit for one idea with several notations, or one key with several meanings: 26 findings, the owner's rulings on all of them, and the choices made while writing them. Applied to the specification. |
| [proposals/conventions/](proposals/conventions/README.md) | 30 | The four remaining shared conventions: fourteen decisions. Accepted; written as promises under `spec/conventions/`. |
| [proposals/schemas-and-interface/](proposals/schemas-and-interface/README.md) | 31 | The shape of documents and contracts, the implementation interface and the implementation-defined list. Accepted, with the owner's amendment on protocol revisions. |
| [proposals/reaudit/](proposals/reaudit/README.md) | 32, 33 | The re-audit of the whole specification by three cold readers: their reports, the 37 decisions of ruling 32, the follow-up decisions of ruling 33, and the logs of the passes that applied them. |
| [proposals/core-and-schemas/](proposals/core-and-schemas/README.md) | 37 | The core package as documents, the JSON Schemas of every document, and the manifest's format. Accepted with five changes; [how each is applied](proposals/core-and-schemas/APPLY.md). |
| [proposals/docs-audit/](proposals/docs-audit/README.md) | 37 | The audit of the documents outside `spec/`, asked for in ruling 37: two readers' reports and the decisions taken to make the repository ready for a cold read. |

### Process for the builds

| Location | What it is |
| --- | --- |
| [implementation/INDEPENDENT_BUILDS.md](implementation/INDEPENDENT_BUILDS.md) | The process, the independence rules and the readiness gate for the two independent builds. The roles and rules are accepted (ruling 11). The builds have not started. |
| [implementation/ENGINE_BUILD_BRIEF.md](implementation/ENGINE_BUILD_BRIEF.md) | The brief each builder receives. A draft; not issued. |

### Superseded, kept for reference

Nothing here binds. Each of these predates the specification and uses the words from before the
vocabulary ruling.

| Location | What it is |
| --- | --- |
| [protocol/](protocol/README.md) | The design as it stood before it was written as promises. Superseded. It is kept as source material for the type contracts still to be written, and because many links point into it. It is no longer edited. |
| [REVIEW.md](REVIEW.md) | The earlier work queue. Superseded by the plan audit; kept for the links into it. |
| [VOCABULARY.md](VOCABULARY.md) | The map of the working terms from before the vocabulary ruling, with stable `vxx` links into the protocol topics. The terms in force are in `spec/definitions.md`. |
| [authoring/](authoring/README.md) | An authoring guide written against the protocol topics. For current documents see [spec/schemas/examples/](../spec/schemas/examples/). |
| [proposals/authoring-syntax/](proposals/authoring-syntax/README.md) | Earlier alternatives for how documents are written, with long examples. Predates the rulings; the shapes in force are the schemas under `spec/schemas/`. |
| [implementation/README.md](implementation/README.md), [ARCHITECTURE.md](implementation/ARCHITECTURE.md), [ORGANIZATION.md](implementation/ORGANIZATION.md) | Earlier proposals for a reference implementation: mechanisms, crates and milestones. Not an input for engine builders. |

### Evidence and history

| Location | What it is |
| --- | --- |
| [research/](research/README.md) | Dated primary sources, source pins, comparisons and probe results. Evidence for the rulings; nothing in it binds, and every packet predates the specification. |
| [archive/](archive/README.md) | Closed reviews, the accounting of one migration and attributed collaborator proposals, including dissent. History; nothing in it binds. |
| [../experiments/](../experiments/README.md) | Exploratory plans, fixtures and measured evidence. Executed: library probes (003) and a Rust model-checking evaluation (004). Planned only: 002. No experiment has run a Conduit engine, because none exists. |

## Repository boundaries

One main repository is accepted. It uses Apache-2.0 for original material, and permanent public
naming remains deferred under the
[publication policy](../CONTRIBUTING.md#project-ownership-and-publication).

The repository holds no engine code and no release scheme. The top-level directories
`conformance/` and `reference-model/` are planned and do not exist yet; see
[independent builds](implementation/INDEPENDENT_BUILDS.md#repository-layout-and-isolation).
There is one specification, `spec/`, and nothing else is to become a second one. A website, if
one is made, renders these sources.

## Maintaining the design

Accepted behavior is a promise in `spec/`, in the terms of `spec/definitions.md`. A change needs
an owner ruling, recorded in the [ruling log](AUDIT.md#ruling-log) of the plan audit with its
record under `proposals/`. `protocol/` is no longer edited.

Keep a proposal that is not yet ruled under `proposals/`, a source claim in `research/`, and a
test or a result in `../experiments/`. After a ruling, change the promises, the definitions, the
schemas and the examples together, and run the [checks](../CONTRIBUTING.md#checks).

The [migration record](archive/migrations/2026-11-protocol-framing.md) locates the paths used
before 2026-11, and the
[vocabulary-preparation audit](archive/migrations/2026-11-vocabulary-audit.md) records the later
index cleanup and the stable `vxx` hooks. Research and archive documents keep the words they
were written in.
