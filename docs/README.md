# Documentation

Start with [SPEC](SPEC.md) for the system, [DESIGN](DESIGN.md) for the accepted contract map, and
[REVIEW](REVIEW.md) for the three conceptual closure topics. The current discussion is conceptual;
package and YAML syntax will return in a later PR.

| Location | Purpose | Authority |
| --- | --- | --- |
| [SPEC.md](SPEC.md) | Concise model and user journey. | Summary of the accepted direction. |
| [DESIGN.md](DESIGN.md) → [design/](DESIGN.md#contract-map) | Detailed rules divided into model, lifecycle, configuration, Agents, Resources, Pools and Conduits. | Canonical accepted behavior; topic files replace the former monolithic body. |
| [REVIEW.md](REVIEW.md) | Conformance scope, authority across dependent calls and cross-object lifecycle. | One active conceptual queue; recommendations are not rulings. |
| [Collaborator comparison](proposals/COLLABORATOR_REVIEW.md) | PR #1 and #2 against the current branch. | Analysis and proposed integration; no adoption or merge. |
| [Implementation proposal](proposals/implementation-v0/README.md) | Runtime mechanisms and delivery plan; concise I1–I7 status. | Accepted boundaries are marked; specific mechanisms remain proposals. |
| [Authoring proposal](proposals/authoring-syntax/README.md) | Package, contract and complete template examples. | Parked for a future syntax/conformance PR. |
| [Experiments](../experiments/README.md) | Scenarios, draft configuration, expected outcomes and actual checks. | Static evidence only so far; fixtures do not define the protocol by themselves. |
| [Archive](archive/README.md) | Attributed historical proposals, including dissent and source revisions. | Historical arguments, not active requirements. |
| [Research](research/README.md) | Primary sources, comparisons and historical probes. | Evidence and alternatives, not extra requirements. |

Keep an accepted detailed rule in its design topic, a pending semantic choice in REVIEW, an unadopted
mechanism in proposals, a source claim in research, and a test/result in experiments. Preserve rationale
and evidence; remove repeated ruling narration rather than maintaining parallel specifications.
Source-system vocabulary in research is not a competing Conduit vocabulary.

The reviewed baseline is on `main`; later accepted rulings and this cleanup are on the implementation
branch until merged. There is no runnable Conduit implementation. A documentation split or static
YAML check is not a successful runtime or provider experiment.
