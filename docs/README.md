# Documentation

The protocol and language lead this repository. Start with the [overview](OVERVIEW.md), then read
the [protocol reference](protocol/README.md) or follow the [authoring path](authoring/README.md).
There is one [review entry point](REVIEW.md). The [plan audit](AUDIT.md) records the direction
set on 2026-11-05: the language is the core, current work is the normative surface, promise register,
schemas, reference model and conformance vectors, and vocabulary comes last. The [vocabulary map](VOCABULARY.md) supplies stable
links to definitions and examples. Completed rulings remain accepted, with their full records in
the [archive](archive/README.md). Research supplies evidence, not another question queue.

## Reading paths

| Goal | Path |
| --- | --- |
| Understand Conduit | [Overview](OVERVIEW.md) → [model](protocol/MODEL.md) → [lifecycle](protocol/LIFECYCLE.md). |
| Define something new | [Authoring](authoring/README.md) → relevant [capability/object contracts](protocol/README.md#contract-map) → clearly marked [draft examples](proposals/authoring-syntax/README.md). |
| Implement the protocol | [Configuration and conformance](protocol/CONFIGURATION.md) → [Conduit authority/requirements](protocol/CONDUITS.md) → [independent builds](implementation/INDEPENDENT_BUILDS.md) and the [experiment program](AUDIT.md#experiment-program). |
| Build the reference runtime | [Implementation boundaries](implementation/README.md) → [architecture candidates](implementation/ARCHITECTURE.md) → [organization candidates](implementation/ORGANIZATION.md). |
| Understand a decision or contribute | [Current review](REVIEW.md) → [research](research/README.md) / [archived rationale](archive/README.md) → [contributor guide](../CONTRIBUTING.md). |
| Review names without changing meaning | [Vocabulary map](VOCABULARY.md) → its stable definition link → [research and walkthroughs](research/vocabulary/README.md) → [naming procedure](VOCABULARY.md#review-and-migration-procedure). |

## Source responsibilities

| Location | Authority and purpose |
| --- | --- |
| [../spec/](../spec/README.md) | The specification: the language and the shared conventions, as numbered promises. A draft being filled area by area; it governs what it covers. |
| [OVERVIEW.md](OVERVIEW.md) | Concise summary of the accepted model; detailed topics govern behavior. |
| [protocol/](protocol/README.md) | One active home for detailed accepted contracts. Explicit reference-runtime selections remain qualified. This is a working reference, not a released schema or a claim of tested conformance. |
| [authoring/](authoring/README.md) | How an original author moves from a contract to an implementation, graph and template. No unapproved syntax or unimplemented installation instructions. |
| [implementation/](implementation/README.md) | Accepted first-implementation boundaries and marked mechanism proposals, plus the [independent-build process](implementation/INDEPENDENT_BUILDS.md) and the [engine build brief](implementation/ENGINE_BUILD_BRIEF.md). Moving a proposal here does not ratify its candidate crates or libraries. |
| [REVIEW.md](REVIEW.md) | The short active work queue, with engineering and deferred scope separated; completed rulings link to their archive. |
| [AUDIT.md](AUDIT.md) | Working tracker for the 2026-11-05 plan audit: findings, experiments and prioritized action items. Not a specification; it changes nothing until a ruling is applied to the affected document. |
| [LANGUAGE_DEFECTS.md](LANGUAGE_DEFECTS.md) | Register of places where the accepted text would let two engine builders diverge. Each is fixed in the language. |
| [VOCABULARY.md](VOCABULARY.md) | Naming review map with stable definition IDs, examples and change surfaces. Navigation and distinctions, not a second specification or approved glossary. |
| [proposals/authoring-syntax/](proposals/authoring-syntax/README.md) | Preserved representation alternatives and complete examples for the schema work. Predates the 2026-11-05 rulings. |
| [proposals/placement/](proposals/placement/README.md) | Accepted direction for expressing where an instance runs. Awaits restatement as promises in the protocol reference. |
| [proposals/instantiation/](proposals/instantiation/README.md) | Accepted direction for what a template lets its instantiator supply. Awaits restatement as promises in the protocol reference. |
| [proposals/relationships/](proposals/relationships/README.md) | First batch of fixes: Conduits, grants and access. Accepted, and written as promises in the specification. |
| [proposals/operations/](proposals/operations/README.md) | Second batch of fixes: operation identity, outcomes and revisions. Accepted, and written as promises in the specification. |
| [proposals/declarations/](proposals/declarations/README.md) | Third batch of fixes: references, instantiation details, submissions and Pool entries. Accepted, and written as promises in the specification. |
| [proposals/graphs-and-schemas/](proposals/graphs-and-schemas/README.md) | Fourth batch of fixes: the static meaning of graphs and the schema profile. Accepted, and written as promises that cite existing standards. |
| [proposals/packages-and-start/](proposals/packages-and-start/README.md) | Fifth and last batch of fixes: packages, first start and conformance. Accepted except how a package's exports are named. |
| [proposals/naming/](proposals/naming/README.md) | How packages, their exports and contracts are named and referred to. Accepted. Evidence in [research/naming/](research/naming/README.md). |
| [proposals/consolidation/](proposals/consolidation/README.md) | Audit for one idea with several notations, or one key with several meanings. Findings, the owner's rulings on all 26, and the choices made while writing them. Applied to the specification. |
| [proposals/conventions/](proposals/conventions/README.md) | The four remaining shared conventions: fourteen decisions, all accepted and written as promises. |
| [proposals/schemas-and-interface/](proposals/schemas-and-interface/README.md) | The last batch: the shape of documents and contracts, the implementation interface, and the implementation-defined list. Accepted, with the baseline-and-negotiation ruling on protocol revisions. |
| [proposals/reaudit/](proposals/reaudit/README.md) | The re-audit of the whole specification by three cold readers: their reports, how each class of finding is handled, and 37 decisions, all accepted. [Consolidation pass 1](proposals/reaudit/CONSOLIDATION.md) applies them. |
| [proposals/core-and-schemas/](proposals/core-and-schemas/README.md) | The core package as documents, the JSON Schemas of every document, and the manifest's format: what was drafted, 98 choices no promise states, 14 open points with recommendations, and 32 places where the promises disagree or forced a guess. Proposed; not accepted. |
| [proposals/vocabulary/](proposals/vocabulary/README.md) | The final vocabulary, combining the naming research, the reviewer's simplification pass and the 2026-11-05 rulings. Accepted; not yet applied to the text. |
| [proposals/normative-surface/](proposals/normative-surface/README.md) | The accepted form of the specification: what it contains and the form of a promise and a vector. |
| [research/](research/README.md) | Dated primary sources, source pins, comparisons and actual probes. Historical recommendations are not an active specification or question queue. |
| [archive/](archive/README.md) | Closed reviews, transition accounting and attributed collaborator proposals, including dissent. Accepted behavior also stays accessible in the active reference. |
| [../experiments/](../experiments/README.md) | Exploratory fixtures, plans and measured evidence outside the documentation tree. Includes executed language-library probes and an executed Rust tool evaluation (004). No Conduit engine experiment has run. |

## Repository boundaries

One main repository is accepted, with independently consumable protocol/conformance, reference-runtime
and extension surfaces. A common package envelope can contain plugins, templates or both. Neither
decision requires a shared version or one installation containing everything.

The repository uses Apache-2.0 for original material; permanent public naming remains deferred under
the [publication policy](../CONTRIBUTING.md#project-ownership-and-publication). No runtime code tree
or release scheme is created by organizing documentation. Future machine-readable protocol
artifacts and conformance tests should link to these behavioral sources; they should not introduce
another prose specification. Runtime code, package source and website build configuration can be added
when their first reviewed slice needs them. A website should render the canonical source material;
its framework, URLs and released-version navigation remain later publication work.

## Maintaining the design

Update an accepted rule once in its protocol topic. Put a proposed semantic change in the current
review with its affected guarantees and a concrete example. Keep an unadopted mechanism in implementation
design or proposals, a source claim in research, and a test/result in experiments. After a ruling,
update the relevant contract and examples together without restoring a parallel decision ledger.

Use the [migration record](archive/migrations/2026-11-protocol-framing.md) to locate old paths and verify
preservation. The [vocabulary-preparation audit](archive/migrations/2026-11-vocabulary-audit.md) records
the later index cleanup and stable hooks. The [contributor guide](../CONTRIBUTING.md) documents local link/anchor checks. Research
and archive wording retains historical terminology where that is necessary to explain the source.
