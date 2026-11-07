# Protocol documentation migration — 2026-11-02

This records a documentation migration, not a new set of protocol rulings. Its purpose is to keep
accepted semantics active and accessible while separating author guidance, reference-implementation
choices, syntax proposals, research and historical review. [Current review](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/REVIEW.md) contains
the remaining decisions; it is the only active queue.

## Baselines and preservation

- `4c54263` preserves the merged conceptual/implementation review, research and collaborator PRs.
- `8609a64e35ecdc66a3b8f1494cca84536650fa0f` is the immediate pre-migration baseline, including the
  repository/common-package decisions and protocol-first transition plan.
- `23dbccb` is the mechanical migration checkpoint: 17 file moves, with all 45 Markdown documents
  preserved and relative link destinations rebased. No prose was removed or rewritten at that step.
- The following editorial pass adds navigation/authoring guidance and a current review queue,
  clarifies historical status and fixes stale cross-references. It does not approve candidate syntax,
  deployment mechanisms, a license, an organization transfer or a release scheme.

## Source map

| Previous source | Current destination | Treatment |
| --- | --- | --- |
| `docs/SPEC.md` | [Overview](../../OVERVIEW.md) | Retained summary, accurately titled as an overview rather than a complete released specification. |
| `docs/DESIGN.md` | [Protocol index](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/README.md) | Preserved principles and contract map; clarified public semantics versus reference-runtime selections. |
| `docs/design/*.md` | [Protocol topics](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/README.md#contract-map) | All seven detailed bodies retained; scope/navigation notes added and stale review pointers corrected where needed. |
| `docs/REVIEW.md` | [Closed conceptual review](../reviews/CONCEPTUAL.md) | Preserved C1–C3 rulings and rationale. A new `docs/REVIEW.md` holds only current work. |
| `docs/proposals/implementation-v0/REVIEW.md` | [Closed implementation review](../reviews/IMPLEMENTATION.md) | Preserved I1–I7 references and accepted/deferred boundaries. |
| `docs/proposals/implementation-v0/{README,ARCHITECTURE,ORGANIZATION}.md` | [Implementation design](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/implementation/README.md) | Retained accepted boundaries and candidate mechanisms, explicitly differentiated. No candidate crate tree was scaffolded. |
| `docs/proposals/implementation-v0/AUTHORIZATION.md` | [Authorization representation proposal](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax/AUTHORIZATION.md) | Keeps mixed semantic examples/unapproved representation work with the later authoring packet. |
| `docs/proposals/COLLABORATOR_REVIEW.md` | [Completed collaborator comparison](../collaborator-proposals/REVIEW.md) | Retained adopted refinements and dissent; corrected an index that incorrectly described the PRs as unmerged. |
| `docs/proposals/REPOSITORY.md` | [Historical repository proposal](../reviews/REPOSITORY-2026-11-02.md) | Retained all research-derived alternatives and the transition plan. Open release/owner questions moved to the current queue. |
| `docs/proposals/authoring-syntax/*` | [Authoring proposal packet](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax/README.md) | Existing long examples retained; no syntax promoted by relocation. |
| `docs/research/*` | [Research](../../research/README.md) | Reports, external source pins and limitations retained. Index status corrected; no blanket re-verification of sources claimed. |
| `docs/archive/collaborator-proposals/PR-*` | [Original collaborator proposals](../collaborator-proposals/README.md) | Original arguments/attribution retained; only repository-relative links rebased. |
| `experiments/*` | [Experiments](../../../experiments/README.md) | Remain outside docs. Markdown links rebased; both YAML fixtures unchanged. No new runtime result claimed. |
| Root/docs navigation | [README](../../../README.md), [docs index](../../README.md) | Rewritten reading paths. Detailed accepted rules were not replaced by these summaries. |

The [machine-readable accounting](2026-11-protocol-framing.json) lists every source/destination,
mechanical content hash and exact editorial correction. It is a dated preservation record, not a
permanent decision ledger or a second source of protocol behavior.

## What remains active

The five families and supporting contracts, immutable definitions/typed controls, Pool ownership,
shared authentication, endpoint responsibilities, requirements/grants, recovery, independent authority,
Interface observation, memory/communication rules and conformance boundaries remain in the protocol
topics. Accepted reference-runtime selections remain qualified and linked from implementation design.
Completed-review IDs and rationale remain accessible without reviving their old question queues.

New navigation emphasizes original authoring rather than an integration catalog. The authoring path
uses existing accepted behavior and links illustrative packages without presenting them as installable.
No empty runtime, schema, website or package skeleton was introduced. The repository remains in design.

## Validation and limits

Migration validation passed on 2026-11-02: 49 Markdown documents, 501 local links, all 45 original
documents accounted for, 17 moves checked, both YAML fixtures unchanged, and no whitespace errors.
These are documentation/preservation results; no runtime experiment was performed.

Run from the repository root:

```sh
python3 tools/check_docs.py
python3 tools/check_doc_migration.py
git diff --check
```

Before the editorial commit, use `python3 tools/check_doc_migration.py --working-tree`. The default
historical audit locates the commit that introduced its accounting file; later legitimate documentation
changes therefore do not fail a preservation check for this migration. Full Git history is required.

The migration audit compares all baseline Markdown sources with the mechanical commit, allowing only
deterministic link rebasing. It then verifies that each retained document changed only through recorded
editorial replacements, except the two explicitly identified navigation rewrites. It also checks both
YAML fixtures byte-for-byte. New guidance/review text receives normal editorial and link review.

Local Markdown checks validate paths, anchors, reference links and fences. They do not establish legal
terms, external-source freshness, schema conformance, interoperability or runtime behavior. Research
tests remain historical evidence and experiments remain static/planned. Preservation checking does
not itself prove the design is complete; outstanding choices are explicitly recorded in current review.
