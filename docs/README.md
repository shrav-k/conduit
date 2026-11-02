# Documentation

The protocol and language lead this repository. Start with the [overview](OVERVIEW.md), then read
the [protocol reference](protocol/README.md) or follow the [authoring path](authoring/README.md).
There is one [review entry point](REVIEW.md). The post-migration review is complete; the next phase
is a concrete authoring/conformance walkthrough, informed first by the
[language/protocol comparison](research/language-protocol-foundations/README.md).
Completed conceptual and implementation rulings
remain accepted; their earlier review summaries live in the archive.

## Reading paths

| Goal | Path |
| --- | --- |
| Understand Conduit | [Overview](OVERVIEW.md) → [model](protocol/MODEL.md) → [lifecycle](protocol/LIFECYCLE.md). |
| Define something new | [Authoring](authoring/README.md) → relevant [capability/object contracts](protocol/README.md#contract-map) → clearly marked [draft examples](proposals/authoring-syntax/README.md). |
| Implement the protocol | [Configuration and conformance](protocol/CONFIGURATION.md) → [Conduit authority/requirements](protocol/CONDUITS.md) → [experiment obligations](../experiments/002-runtime-plugins/README.md). |
| Build the reference runtime | [Implementation boundaries](implementation/README.md) → [architecture candidates](implementation/ARCHITECTURE.md) → [organization candidates](implementation/ORGANIZATION.md). |
| Understand a decision or contribute | [Current review](REVIEW.md) → [research](research/README.md) / [archived rationale](archive/README.md) → [contributor guide](../CONTRIBUTING.md). |

## Source responsibilities

| Location | Authority and purpose |
| --- | --- |
| [OVERVIEW.md](OVERVIEW.md) | Concise summary of the accepted model; detailed topics govern behavior. |
| [protocol/](protocol/README.md) | One active home for detailed accepted contracts. Explicit reference-runtime selections remain qualified. This is a working reference, not a released schema or a claim of tested conformance. |
| [authoring/](authoring/README.md) | How an original author moves from a contract to an implementation, graph and template. No unapproved syntax or unimplemented installation instructions. |
| [implementation/](implementation/README.md) | Accepted first-implementation boundaries and marked mechanism proposals. Moving a proposal here does not ratify its candidate crates or libraries. |
| [REVIEW.md](REVIEW.md) | Settled migration/owner decisions and next authoring refinements, with engineering and deferred scope separated. |
| [proposals/authoring-syntax/](proposals/authoring-syntax/README.md) | Preserved representation alternatives and complete examples for the next syntax/conformance PR. |
| [research/](research/README.md) | Dated primary sources, source pins, comparisons and actual probes. Historical recommendations are not an active specification or question queue. |
| [archive/](archive/README.md) | Closed reviews, transition accounting and attributed collaborator proposals, including dissent. Accepted behavior also stays accessible in the active reference. |
| [../experiments/](../experiments/README.md) | Exploratory fixtures, plans and measured evidence outside the documentation tree. Includes executed language-library probes; Conduit runtime experiments remain static/planned. |

## Repository boundaries

One main repository is accepted, with independently consumable protocol/conformance, reference-runtime
and extension surfaces. A common package envelope can contain plugins, templates or both. Neither
decision requires a shared version or one installation containing everything.

This migration establishes documentation sources and navigation. It does not create an organization,
choose a license, approve a release scheme or scaffold a code tree. Future machine-readable protocol
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
preservation. The [contributor guide](../CONTRIBUTING.md) documents local link/anchor checks. Research
and archive wording retains historical terminology where that is necessary to explain the source.
