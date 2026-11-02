# Documentation

Start with [SPEC](OVERVIEW.md) for the system, [DESIGN](protocol/README.md) for the accepted contract map, and
[REVIEW](archive/reviews/CONCEPTUAL.md) for the completed conceptual review. The current discussion is
[repository, documentation and distribution structure](archive/reviews/REPOSITORY-2026-11-02.md), before package and
YAML syntax return in a later authoring/conformance PR.

The language and public contracts lead the documentation. The
[transition plan](archive/reviews/REPOSITORY-2026-11-02.md#8-preserving-the-design-through-the-transition) explains how
to preserve accepted semantics, research and dissent while developing the reference and authoring
paths. Existing design topics remain authoritative until their deliberate migration; proposed file
locations or examples do not replace them automatically.

| Location | Purpose | Authority |
| --- | --- | --- |
| [SPEC.md](OVERVIEW.md) | Concise model and user journey. | Summary of the accepted direction. |
| [DESIGN.md](protocol/README.md) → [design/](protocol/README.md#contract-map) | Detailed rules divided into model, lifecycle, configuration, Agents, Resources, Pools and Conduits. | Canonical accepted behavior; topic files replace the former monolithic body. |
| [REVIEW.md](archive/reviews/CONCEPTUAL.md) | Conformance scope, authority across dependent calls and cross-object lifecycle. | Completed C1–C3 rulings; detailed syntax remains deferred. |
| [Repository review](archive/reviews/REPOSITORY-2026-11-02.md) | Organization, source boundaries, packages, releases and website. | One main repository and common package envelope accepted; documentation, layout and release proposals remain under review. |
| [Collaborator comparison](archive/collaborator-proposals/REVIEW.md) | PR #1 and #2 against the reviewed design. | Adopted refinements, unadopted alternatives and attributed dissent. |
| [Implementation proposal](implementation/README.md) | Runtime mechanisms and delivery plan; concise I1–I7 status. | Accepted boundaries are marked; specific mechanisms remain proposals. |
| [Authoring proposal](proposals/authoring-syntax/README.md) | Package, contract and complete template examples. | Parked for a future syntax/conformance PR. |
| [Experiments](../experiments/README.md) | Scenarios, draft configuration, expected outcomes and actual checks. | Static evidence only so far; fixtures do not define the protocol by themselves. |
| [Archive](archive/README.md) | Attributed historical proposals, including dissent and source revisions. | Historical arguments, not active requirements. |
| [Research](research/README.md) | Primary sources, comparisons and historical probes. | Evidence and alternatives, not extra requirements. |

Keep an accepted detailed rule in its design topic, a pending semantic choice in REVIEW, an unadopted
mechanism in proposals, a source claim in research, and a test/result in experiments. Preserve rationale
and evidence; remove repeated ruling narration rather than maintaining parallel specifications.
Source-system vocabulary in research is not a competing Conduit vocabulary.

This documentation contains the reviewed conceptual design and attributed collaborator proposals.
There is no runnable Conduit implementation. A documentation split or static
YAML check is not a successful runtime or provider experiment.
