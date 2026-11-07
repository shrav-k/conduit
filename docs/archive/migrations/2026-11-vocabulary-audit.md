# Documentation and vocabulary-preparation audit

Date: 2026-11-04. Baseline: [`82e87cd`](https://github.com/shrav-k/conduit/commit/82e87cd18eeb000f5277158e8dd3e0cae100eb1c),
the merge of the authoring behavior/structure review in [PR #3](https://github.com/shrav-k/conduit/pull/3).
This is documentation accounting, not a new specification, naming ruling or runtime test report.

## Findings and changes

| Finding | Change | Preservation boundary |
| --- | --- | --- |
| The active review mixed current work with 801 lines of completed rulings. | [Current review](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/REVIEW.md) now lists next work, representation obligations, deferred scope and closure links. | The entire former body is in the [completed authoring record](../reviews/AUTHORING-2026-11-04.md), with only its title and relative links adjusted. |
| The authoring index repeated 308 lines of accepted rules already maintained in protocol topics. | The [proposal index](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax/README.md) is a reading map and a list of canonical rule sources. | Its full former body is preserved as the archive's [index appendix](../reviews/AUTHORING-2026-11-04.md#preserved-authoring-index). Detailed worked proposals remain in place. |
| Entry pages still described the merged review as future work; the documentation map also retained pre-license migration language. | Updated repository/docs/protocol/authoring/implementation/research/experiment entry points to the current status. | No change to accepted semantics, license, milestones or implementation claims. |
| Provisional terminology had no common naming-review map and heading-based links could move during a rename. | Added [VOCABULARY.md](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md), with 37 topic IDs linked to stable `v01`–`v37` anchors at canonical definitions. | IDs are documentation hooks only. No replacement words, language keywords or contract IDs were selected. |
| The same words describe unrelated things across graphs, registries, runtime values and external protocols. | Added collision examples, affected surfaces and a [naming migration procedure](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md#review-and-migration-procedure), also linked from the contributor guide. | Review one meaning in a complete example; no global replacement across sources, archives or MCP/JSON Schema fields. |

The active review keeps its previously linked section anchors, including the completed merge checkpoint.
The frozen records point to current sources without pretending their historical next steps are still
pending. This audit adds no second decision queue or glossary that competes with the protocol reference.

## Source accounting

| Baseline source | Active destination | Full historical body |
| --- | --- | --- |
| `docs/REVIEW.md` — 801 lines | [REVIEW.md](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/REVIEW.md) — 107 lines at this audit | [Authoring review](../reviews/AUTHORING-2026-11-04.md). |
| `docs/proposals/authoring-syntax/README.md` — 308 lines | [Proposal index](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax/README.md) — 59 lines at this audit | [Index appendix](../reviews/AUTHORING-2026-11-04.md#preserved-authoring-index). |
| Seven canonical topic files | Same paths in [protocol/](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/README.md) | Existing prose retained; stable definition anchors added. Model/index navigation was updated to the completed review and naming map. |
| Existing research, archives and experiments | Same locations | Research reports, source inventories/pins, attributed dissent, YAML fixtures, probe code and recorded results were not rewritten. Active index/status wording was clarified. |

The vocabulary map is deliberately a navigation aid. It links to definitions and examples, identifies
nearby meanings that must stay distinct, and lists affected authoring/wire/documentation surfaces.
The final naming PR should record chosen spellings and decision links there, retain the stable IDs,
and update active examples coherently. Future schema/code work must still demonstrate conformance.

## Verification and limits

Completed checks:

- Read both original source files from the pinned Git baseline. After removing their first title and
  relocating relative Markdown links, verified each complete body appears exactly once in the archive.
  No accepted ruling was dropped merely to shorten an index.
- Verified 37 unique `vxx` anchors across the seven canonical topic files, each with a matching map entry.
  The normal documentation checker follows those definition links and the example links.
- Compared the seven canonical topic bodies with the baseline after removing the new anchors:
  six match exactly; `MODEL.md` differs only in its navigation paragraph linking the completed
  review and naming map. Accepted behavioral text was not rewritten.
- Ran `python3 tools/check_docs.py`: 59 Markdown files and 833 local links, zero errors.
  `git diff --check` also passed. These check local navigation/fences and whitespace, not semantics.
- Ran the existing `python3 tools/check_doc_migration.py`: the earlier frozen migration still passes
  its 45-source, 17-move, two-unchanged-YAML preservation audit with zero errors.

This audit did not re-fetch research sources, revalidate upstream product claims, choose final
vocabulary, implement schemas, execute a Conduit runtime, or rerun unchanged provider experiments.
The repository still distinguishes paper/static fixtures, recorded library probes, planned runtime
tests and actual provider evidence. Existing source dates and test limitations remain meaningful.
