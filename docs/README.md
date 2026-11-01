# Documentation

Read the [specification](SPEC.md) first, then the
[YAML walkthrough](../experiments/001-shared-access/README.md). Consult
[design contracts](DESIGN.md) for precise behavior and [review](REVIEW.md) for the boundary
between settled decisions, schema proposals and future work.

For implementation planning, read the [v0 proposal](proposals/implementation-v0/README.md) and its
[grouped review](proposals/implementation-v0/REVIEW.md). Accepted extension boundaries are folded
into DESIGN; unresolved crate, deployment and syntax choices remain explicitly proposed.
The current authoring review formalizes [Policy and Grant](proposals/implementation-v0/AUTHORIZATION.md)
alongside [composable relationship requirements](DESIGN.md#composable-relationship-requirements),
plugins and operations. The existing YAML walkthrough does not yet specify their unresolved syntax.

| Location | Purpose | Authority |
| --- | --- | --- |
| [SPEC.md](SPEC.md) | Concise model, workflow and scope. | Current design summary. |
| [DESIGN.md](DESIGN.md) | Identity, construction, controls, access, lifecycle and domain contracts. | Detailed accepted behavior. |
| [REVIEW.md](REVIEW.md) | Consolidated review outcomes and questions to test with examples. | Review boundary; no parallel decision ledger. |
| [proposals/implementation-v0/](proposals/implementation-v0/README.md) | Concrete runtime architecture, plugin contracts, repository organization and grouped implementation choices. | Proposed implementation; pending rulings are explicit. |
| [experiments/](../experiments/README.md) | Concrete configuration, walkthroughs and acceptance evidence. | Proposed syntax and tests until adopted/validated. |
| [research/](research/README.md) | Primary sources, comparisons and recorded probes. | Evidence, not additional requirements. |

Keep each detailed rule in DESIGN. Experiments link to those rules rather than inventing another
contract. Record actual validation separately from expected outcomes. Research can describe
broader possibilities or another system's behavior without changing Conduit's design.

The reviewed baseline is on `main`; later accepted rulings remain on the implementation branch
until merged. Implementation planning is still documentation only. No live
provider experiments or runtime implementation have been completed by drafting the YAML examples
or implementation proposal.
