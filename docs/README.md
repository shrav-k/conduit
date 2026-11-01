# Documentation

Read the [specification](SPEC.md) first, then the
[YAML walkthrough](../experiments/001-shared-access/README.md). Consult
[design contracts](DESIGN.md) for precise behavior and [review](REVIEW.md) for the boundary
between settled decisions, schema proposals and future work.

| Location | Purpose | Authority |
| --- | --- | --- |
| [SPEC.md](SPEC.md) | Concise model, workflow and scope. | Current design summary. |
| [DESIGN.md](DESIGN.md) | Identity, construction, controls, access, lifecycle and domain contracts. | Detailed accepted behavior. |
| [REVIEW.md](REVIEW.md) | Consolidated review outcomes and questions to test with examples. | Review boundary; no parallel decision ledger. |
| [experiments/](../experiments/README.md) | Concrete configuration, walkthroughs and acceptance evidence. | Proposed syntax and tests until adopted/validated. |
| [research/](research/README.md) | Primary sources, comparisons and recorded probes. | Evidence, not additional requirements. |
| [Proposal/](Proposal/README.md) | Roadmap proposal: protocol focus, Sealmesh messaging, local root agent, per-environment VMs. | Proposal for discussion; not accepted design. |

Keep each detailed rule in DESIGN. Experiments link to those rules rather than inventing another
contract. Record actual validation separately from expected outcomes. Research can describe
broader possibilities or another system's behavior without changing Conduit's design.

Current work is documentation only on `main`. No live provider experiments
or runtime implementation have been completed by drafting the YAML examples.
