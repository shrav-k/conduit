# Archive

Nothing here binds. The specification is [`spec/`](../../spec/README.md); current work is in
the [tracker](../AUDIT.md), and decisions in the [ledger](../proposals/ruling-log.md).
Git preserves removed source material, attribution and dissent. Archiving is not approval or
evidence that a proposed integration was tested.

## Removed from the tree

One row per removed directory or group. Ruling 39’s last versions remain `e271d1c`;
rulings 57 and 58’s condensation uses C0, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`,
except the earlier removal of Experiment 001 (`de6566a`).

| Path or group | What it was | Source material for | Last version |
| --- | --- | --- | --- |
| `docs/proposals/core-and-schemas/{log-documents.md, log-promises.md, APPLY.md}` | Application instructions and editor logs; choices preserved in the record, with the 22 left-undone items checked against their settlements. | [Core/schema decisions and editor choices](../proposals/core-and-schemas/README.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas) |
| `docs/proposals/docs-audit/{log-entry-points.md, log-older-text.md, log-gaps.md, reader-entry-points.md, reader-older-text.md}` | Reader reports and editor logs; the DA decisions and GAPS register remain, with editor choices preserved in the record. | [DA decisions](../proposals/docs-audit/README.md) and [conversion list](../proposals/docs-audit/GAPS.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit) |
| `docs/protocol/` | The protocol topics: the design as it stood before the specification. Source material for the type contracts still to be written. | Type contracts still to be written; [current specification](../../spec/README.md). | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol) |
| `docs/authoring/` | The authoring guide, written for the protocol topics. | Historical authoring rationale; [current reading paths](../../spec/README.md#reading-paths). | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/authoring) |
| `docs/proposals/authoring-syntax/` | A proposed authoring syntax, never ruled, in the shapes the specification replaced. | Historical syntax alternatives; current shapes are in the specification. | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax) |
| `docs/REVIEW.md` | The review queue, superseded by the plan audit. | [Tracker](../AUDIT.md) and its open action items. | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/REVIEW.md) |
| `docs/VOCABULARY.md` | The map of working terms from before the vocabulary ruling. | [Vocabulary decisions](../proposals/vocabulary/README.md) and [definitions](../../spec/definitions.md). | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md) |
| `docs/implementation/README.md` | A proposal for the reference implementation, from before the rulings. | Historical implementation plans; [live build](../../engine/README.md). | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/implementation/README.md) |
| `docs/implementation/ARCHITECTURE.md` | A proposed runtime architecture, from before the rulings. | Historical implementation plans; [live build](../../engine/README.md). | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/implementation/ARCHITECTURE.md) |
| `docs/implementation/ORGANIZATION.md` | A proposed organization of repositories and delivery, from before the rulings. | [Organization and publication ruling](../proposals/ruling-47/README.md). | [e271d1c](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/implementation/ORGANIZATION.md) |
| `experiments/002-runtime-plugins/` | Experiment 002: a plan only, with no code and no measurements. | Historical proof plan, superseded; no execution evidence. | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/002-runtime-plugins) |
| `experiments/003-language-probes/` | Experiment 003: Python probes of JSON Schema and YAML libraries. | Historical schema/library probes; [current schemas](../../spec/schemas/README.md). | [e271d1c](https://github.com/shrav-k/conduit/tree/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/003-language-probes) |
| `docs/proposals/reaudit/CONSOLIDATION.md` | The first consolidation’s instructions and validation. Its promise-writing rules are in CONTRIBUTING.md, “Writing promises”; the form check is in tools/check_schemas.py. | [Promise-writing rules](../../CONTRIBUTING.md#writing-promises) and form checks. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md) |
| `docs/proposals/reaudit/reader-1-duplicates-and-contradictions.md, docs/proposals/reaudit/reader-2-terms-and-shapes.md, docs/proposals/reaudit/reader-3-undetermined-behavior.md` | Round 1’s cold reports; source for the kept decisions and follow-up in the re-audit record. | [Re-audit decisions and verification](../proposals/reaudit/README.md); [cold readers’ guide](../guides/cold-readers.md); [ledger](../proposals/ruling-log.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/) |
| `docs/proposals/reaudit/log-group-a.md, docs/proposals/reaudit/log-group-b.md, docs/proposals/reaudit/log-group-c.md, docs/proposals/reaudit/log-followups-ab.md, docs/proposals/reaudit/log-followups-c.md` | Round 1’s editor logs; source for the re-audit record’s “Choices that set behavior” and follow-up decisions. | [Re-audit decisions and verification](../proposals/reaudit/README.md); [cold readers’ guide](../guides/cold-readers.md); [ledger](../proposals/ruling-log.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/) |
| `docs/proposals/reaudit/round-2/` | Round 2’s reports, decisions and editor logs; source for the re-audit index and the ledger’s historical R1-/R2-/R3- prefix destinations. | [Re-audit decisions and verification](../proposals/reaudit/README.md); [cold readers’ guide](../guides/cold-readers.md); [ledger](../proposals/ruling-log.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2) |
| `docs/proposals/reaudit/round-3/` | Round 3’s reports, decisions and fix log; source for the re-audit index and the ledger’s historical 3A to 3D prefix destination. | [Re-audit decisions and verification](../proposals/reaudit/README.md); [cold readers’ guide](../guides/cold-readers.md); [ledger](../proposals/ruling-log.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-3) |
| `docs/proposals/reaudit/round-4/` | Round 4’s reports, decisions, fix log and protocol recheck; source for the re-audit index, tracker, cold readers’ protocol guide and ledger’s historical 4A to 4D prefix destination. | [Re-audit decisions and verification](../proposals/reaudit/README.md); [cold readers’ guide](../guides/cold-readers.md); [ledger](../proposals/ruling-log.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4) |
| `docs/proposals/reaudit/round-5/reader-1.md to reader-6.md; docs/proposals/reaudit/round-5/handoff-check.md` | Round 5’s cold reports and handoff check; source for its kept decisions and verification, and the cold readers’ protocol guide. | [Re-audit decisions and verification](../proposals/reaudit/README.md); [cold readers’ guide](../guides/cold-readers.md); [ledger](../proposals/ruling-log.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5) |
| `docs/proposals/reaudit/round-5/log-fix-F1.md to docs/proposals/reaudit/round-5/log-fix-F7.md` | Round 5’s seven area logs; source for the kept integration and the re-audit record’s “Choices that set behavior”. | [Re-audit decisions and verification](../proposals/reaudit/README.md); [cold readers’ guide](../guides/cold-readers.md); [ledger](../proposals/ruling-log.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5) |
| `docs/proposals/reaudit/round-6/log-d06-d12.md, log-conventions.md, log-core.md, log-interface.md, log-promises.md` | Round 6’s five editor logs; alternatives, readings and costs copied to DECISIONS.md, “For the owner”; open items retained with status in log-integration.md. | [Re-audit decisions and verification](../proposals/reaudit/README.md); [cold readers’ guide](../guides/cold-readers.md); [ledger](../proposals/ruling-log.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6) |
| `docs/proposals/reaudit/round-6/log-specdocs.md, log-tracker.md` | Round 6’s specification-document and tracker logs; source for its README’s “Choices that set behavior”; the pointer-comparison question remains integration item 3. | [Re-audit decisions and verification](../proposals/reaudit/README.md); [cold readers’ guide](../guides/cold-readers.md); [ledger](../proposals/ruling-log.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6) |
| `docs/proposals/ruling-38/check.md`, `docs/proposals/ruling-38/handoff-check.md`, `docs/proposals/ruling-38/log-spec.md` | Checks and the specification editor’s log; their changes are recorded in ruling 38. [Record](../proposals/ruling-38/README.md). | [Ruling 38](../proposals/ruling-38/README.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-38) |
| `docs/proposals/ruling-41/PLAN.md`, `docs/proposals/ruling-41/log-G1.md`, `docs/proposals/ruling-41/log-G2.md`, `docs/proposals/ruling-41/log-G3.md`, `docs/proposals/ruling-41/log-G4.md`, `docs/proposals/ruling-41/log-G5.md`, `docs/proposals/ruling-41/log-G6.md`, `docs/proposals/ruling-41/log-G7.md`, `docs/proposals/ruling-41/log-core.md`, `docs/proposals/ruling-41/log-docs.md`, `docs/proposals/ruling-41/log-integration.md` | The rewrite plan and ten editor logs; P1 to P20 and editor choices are preserved in the ruling record, and open decisions remain in place. [Record](../proposals/ruling-41/README.md). | [Ruling 41’s choices](../proposals/ruling-41/README.md); open decisions stay live. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41) |
| `docs/proposals/ruling-42/log-conventions.md`, `docs/proposals/ruling-42/log-core.md`, `docs/proposals/ruling-42/log-docs.md`, `docs/proposals/ruling-42/log-editorial.md`, `docs/proposals/ruling-42/log-integration.md`, `docs/proposals/ruling-42/log-language.md`, `docs/proposals/ruling-42/log-machines.md`, `docs/proposals/ruling-42/log-rename-a.md`, `docs/proposals/ruling-42/log-rename-b.md`, `docs/proposals/ruling-42/log-secrets.md` | Ten editor logs; their choices are preserved in the ruling record. [Record](../proposals/ruling-42/README.md). | [Ruling 42’s choices](../proposals/ruling-42/README.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42) |
| `docs/proposals/ruling-43/log-core.md`, `docs/proposals/ruling-43/log-docs.md`, `docs/proposals/ruling-43/log-integration.md`, `docs/proposals/ruling-43/log-interface.md`, `docs/proposals/ruling-43/log-providers.md`, `docs/proposals/ruling-43/log-secrets.md` | Six editor logs; their choices are preserved in the ruling record, and MX-21 in the ledger’s appendix. [Record](../proposals/ruling-43/README.md). | [Ruling 43’s choices](../proposals/ruling-43/README.md); MX-21 in the ledger appendix. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43) |
| `docs/proposals/first-build/log-increment-1.md`, `docs/proposals/first-build/log-increment-2.md`, `docs/proposals/first-build/log-increment-3.md` | The three closed increment logs: 33 findings in the [settlements register](../proposals/first-build/README.md#settlements-register); D19 and open loader remainders there; lean-vector and mutation evidence in [conformance](../../conformance/README.md#planned-work). | [Settlements register](../proposals/first-build/README.md#settlements-register), A44, G6 and [A79](../../conformance/README.md#planned-work). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-1.md), [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md), [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-3.md) |
| `docs/proposals/first-build/check-plain-errors.md` | Tagged protocol/SDK check; conclusion, provenance and rejected alternatives in [decision 8a’s evidence](../proposals/first-build/increment-2/decisions.md#evidence-for-decision-8a). | [Decision 8a’s evidence](../proposals/first-build/increment-2/decisions.md#evidence-for-decision-8a). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/check-plain-errors.md) |
| `docs/proposals/first-build/increment-3-handoff.md` | Historical increment-2 boundary and shell handoff; today’s [core boundary](../../engine/README.md#the-cores-boundary) is rebuilt from code, with coverage debt in conformance. | [Live core boundary](../../engine/README.md#the-cores-boundary). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3-handoff.md) |
| `docs/proposals/first-build/increment-3/closeout.md` | The real-client closeout plan; its sentinel method and remaining checks are in [How to run the open checks](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#how-to-run-the-open-checks). | [Open-check method](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#how-to-run-the-open-checks). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3/closeout.md) |
| `docs/proposals/first-build/increment-2/plan.md` | The study: frozen [promise inventory](../../conformance/s1-promises.yaml), [proposed safety traces](../../conformance/s1-safety-cases.md), admin decisions, portable runner/code and planned coverage work carry its relevant information. | [Frozen inventory](../../conformance/s1-promises.yaml), [safety traces](../../conformance/s1-safety-cases.md), live decisions and coverage work. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md) |
| `docs/proposals/first-build/increment-2/s1/` | Nine illustrative YAML draft files; maintained fixtures and the [live S1 example](../../engine/examples/s1/README.md) already replaced them. No archive/install or runtime result was claimed by the drafts. | Maintained conformance fixtures and [live S1 example](../../engine/examples/s1/README.md). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/s1) |
| `docs/proposals/simplification/research/` (13 files) | Six maps, five designs, a synthesis and a critic of S1 authoring costs. | [Rejected alternatives, guarantees and measured core-reduction costs](../proposals/simplification/README.md#carried-from-the-papers); [golden plans and critic constraints](../proposals/language-shape/short-form/synthesis.md#carried-from-the-papers). E9's uncertainties and cost effects, E11's measurement questions and E36's tagging to-do were already carried in the increment-4 log, tracker, measurement plan and conformance README. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research) |
| The four `design-N1` to `design-N4` papers and four `inventory-*` papers under `docs/proposals/language-shape/short-form/` (8 files) | Alternative concise forms and their build, construct, prior-art and scenario inventories. The synthesis and critic stay live for spec order 037. | [Carried constraints and tests](../proposals/language-shape/short-form/synthesis.md#carried-from-the-papers), [reduction costs](../proposals/simplification/README.md#what-the-v1-candidate-reductions-lose), and [worker-scale constraints](../proposals/projects/brief.md#worker-structure-and-upgrade-costs). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form) |
| `docs/proposals/language-shape/project/` (10 files) | Four Project designs, four inventories, the synthesis and its critic. | The [Project brief](../proposals/projects/brief.md#carried-from-the-earlier-project-papers): provenance, alternatives and decisions, authority and lifecycle constraints, use cases, prior art and critic constraints, retained beside its revised recommendation. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project) |
| `docs/research/CONDUITS.md` | Access and messaging comparisons, evaluation cases and retained-history conclusions. | History/messaging contracts: L151 to L154, L166 and A10; conclusions at C0 lines 795 to 833 and evaluation cases. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONDUITS.md) |
| `docs/research/RESOURCES.md` | Resource responsibilities, capabilities, type groups and candidate catalog. | A50’s core type contracts; catalog at C0 lines 148 to 250. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/RESOURCES.md) |
| `docs/research/ESTABLISHMENT.md` | Account, database, credential, directory and worktree setup/recovery and fault cases. | Directory/worktree contracts: L85, L86, L92 and L96; C0 lines 333 to 563 and 589 to 600. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/ESTABLISHMENT.md) |
| `docs/research/PI_RUNTIME.md` | Pi durable execution, replay, storage and frontend boundaries. | Agent contract: L77, L79, L81, L87 and L94; sections 3, 4, 6 and 7. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/PI_RUNTIME.md) |
| `docs/research/PI_CONNECTORS.md` | Pi providers, account/context boundaries and history. | Account contract: “Authentication is an account contract”; retained “Session history”. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/PI_CONNECTORS.md) |
| `docs/research/OBSERVABILITY.md` | Observation, capture and telemetry comparisons. | Observation provider, especially section 3. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/OBSERVABILITY.md) |
| `docs/research/AUTHORIZATION.md` | Authorization and cloud authority precedents. | A66: caller rights, required restrictions, revocation consistency and grant administration; [live source summary](../AUDIT.md#p1--prove-the-language-can-be-rebuilt-and-its-promises-kept). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/AUTHORIZATION.md) |
| `docs/research/CLOUD_AUTHORITY.md` | Authorization and cloud authority precedents. | A66: caller rights, required restrictions, revocation consistency and grant administration; [live source summary](../AUDIT.md#p1--prove-the-language-can-be-rebuilt-and-its-promises-kept). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CLOUD_AUTHORITY.md) |
| `docs/research/ECOSYSTEM_ORGANIZATION.md` | Repository, distribution and documentation comparisons. | Five conclusions preserved [below](#repository-and-ecosystem-source-material); organization/publication record [ruling 47](../proposals/ruling-47/README.md). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/ECOSYSTEM_ORGANIZATION.md) |
| `docs/research/CONFIGURATION.md` | Configuration composition and update semantics. | [Prior art still applied](#prior-art-still-applied). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONFIGURATION.md) |
| `docs/research/CONSTRUCTION.md` | Template, implementation and runtime boundaries. | [Prior art still applied](#prior-art-still-applied). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONSTRUCTION.md) |
| `docs/research/RUNTIME_ESTABLISHMENT.md` | Setup readiness, progress and recovery. | [Prior art still applied](#prior-art-still-applied). | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/RUNTIME_ESTABLISHMENT.md) |
| `docs/research/DISTRIBUTED_RUNTIME.md` | Early runtime comparisons and proposed proof layouts. | Historical design research; later layers and provider boundaries supersede its runtime proposals. No new extraction. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/DISTRIBUTED_RUNTIME.md) |
| `docs/research/RUST_RUNTIME.md` | Early runtime comparisons and proposed proof layouts. | Historical design research; later layers and provider boundaries supersede its runtime proposals. No new extraction. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/RUST_RUNTIME.md) |
| `docs/research/language-protocol-foundations/` | Six format, walkthrough, source and probe records. | [Format choice](#format-choice); behavior/structure decisions in the ledger and specification. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/language-protocol-foundations) |
| `docs/research/vocabulary/{README.md, PRECEDENTS.md, SOURCES.md}` | README.md, PRECEDENTS.md and SOURCES.md: vocabulary evidence kept by ruling 36, then removable under rulings 57 and 58. | Vocabulary decisions; [the reviewer’s simplification pass](../research/vocabulary/SIMPLIFICATION.md) stays as written. The JSON source pins and retrieval records also remain. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/vocabulary) |
| `docs/archive/collaborator-proposals/` | Four submitted-proposal and comparison records from allocmemory. | [Unadopted alternatives and why](#not-adopted-from-the-collaborators-proposals-and-why); [attribution](#attribution); accepted review rulings in the ledger. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/collaborator-proposals) |
| `docs/archive/migrations/` | Three framing/vocabulary migration records, including the machine-readable preservation manifest. | Historical migration accounting; the specification and current checks govern new work. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/migrations) |
| `tools/check_doc_migration.py` | Fixed-Git-object checker of the historical framing migration. | Reproduction of that migration only; removed with its records and CONTRIBUTING instructions. | [f1cb473](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/tools/check_doc_migration.py) |
| `docs/archive/reviews/` | Four completed authoring, conceptual, implementation and repository reviews. | [Rulings before the plan audit](../proposals/ruling-log.md#history-rulings-before-the-plan-audit); [repository release/website source material](#repository-and-ecosystem-source-material). | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/reviews) |
| `experiments/001-shared-access` | Shared-access paper scenario and static fixtures, removed before C0. | Later worked schemas and acceptance scenarios; historical evidence, not an engine run. | [f1cb473](https://github.com/shrav-k/conduit/tree/de6566ad8a2ea408876a66ab04930734c7b334de/experiments/001-shared-access) |

Under rulings 57 and 58, work order 066 removed the raw experiment records and superseded
harness code below after checking every removed file against C0: none had changed. The
[experiment index](../../experiments/README.md#evidence-for-v0) keeps the dated results,
methods as run, exact prompts and counted artifacts. Experiments 008 and 009 no longer have
Cargo manifests, so the automatically discovered CI matrix no longer includes either harness.

| Path or group | What it was and what stays | Last version |
| --- | --- | --- |
| `experiments/001-shared-access/` | Experiment 001: exercised all five families on paper, with static fixtures. Removed on 2026-11-05 because its fixtures used only shapes the rulings replaced; spec/schemas/examples/ supplies the current worked examples. | [de6566a](https://github.com/shrav-k/conduit/tree/de6566ad8a2ea408876a66ab04930734c7b334de/experiments/001-shared-access) |
| `experiments/005-macos-keychain/{results.json, results-rerun.json, results.txt, results-rerun.txt}` | Four raw ACL and timing records, tabulated in the README's dated observations. The probe code and safety method stay; A22's daemon case is open. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/005-macos-keychain) |
| `experiments/007-s1-baseline/evidence-2026-11-09/ (except prompts.txt)` | 32 raw run records. The dated results, exact prompts and counted setup/ files stay. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/007-s1-baseline/evidence-2026-11-09) |
| `experiments/008-real-client/{src/, tests/, Cargo.toml, Cargo.lock, rust-toolchain.toml, .gitignore}` | Ten superseded harness files. README, HARNESS.md and both dated result files stay; the engine and Agent packages carry the replacement code. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client) |
| `experiments/008-real-client/evidence-2026-11-09/ (except README.md)` | 818 raw matrix records. The evidence README keeps the trimming and sanitization method. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09) |
| `experiments/008-real-client/evidence-2026-11-09-later-turn/ (except README.md)` | 36 raw later-turn records. The evidence README and dated follow-up results stay; the metrics proposal cites its two source turns at C0. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn) |
| `experiments/009-native-launch/{src/, tests/, Cargo.toml, Cargo.lock, rust-toolchain.toml, .gitignore}` | Thirteen superseded harness files, removed with the 008 crate on which they depended. README, HARNESS.md and dated results stay. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch) |
| `experiments/009-native-launch/evidence-2026-11-09/ (except README.md)` | 29 raw attribution and harness records. The evidence README keeps the trimming and sanitization method; the Claude Code fixture cites its source at C0. | [f1cb473](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch/evidence-2026-11-09) |

## Format choice

YAML 1.2, JSON and JSON Schema 2020-12 were chosen because MCP and OpenAPI 3.2 build on 2020-12. JSON alone was the runner-up. CUE was the strongest alternative, to reconsider only if authoring exposes constraint or generation behavior the model cannot represent. Dhall needs another evaluator; Smithy and WIT need a mapping; a new notation carries the largest burden. [Source at C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/language-protocol-foundations/FORMATS.md#L25-L70)

## Not adopted from the collaborator's proposals, and why

Historical comparison, copied verbatim from the reviewed C0 source. Its terminology and
then-current contracts describe that review; the specification governs today.

## What needs adaptation

| Proposal | Current contract / recommended treatment |
| --- | --- |
| Carry setup/runtime as opaque implementation-defined fields in the first protocol | Conflicts with portable plugin/template authoring. Keep public graph, operation, control and lifecycle meanings. A limited consumer may decline an unsupported execution surface; a template executor cannot claim portability by ignoring its semantics. |
| Exclude plugin ABI with all graph behavior | Separate physical ABI/process plumbing from public invocation and execution semantics. Native ABI details need not be standardized; MCP/profile contracts and declared behavior do. |
| Derive the protocol directly from what Experiment 001 needs | Use 001 as a scenario and 002 for composition/failure coverage. Draft fixture fields are not rulings, and neither fixture alone defines every required semantic boundary. |
| Start with a parallel fixed JSON-RPC method catalog and local socket wire | MCP is already the accepted public plugin/capability base. First map supported standard mechanisms and the necessary Conduit profile; evaluate any additional transport/method surface in the later authoring PR. No new wire is adopted here. |
| Use one short family lifecycle and a closed error list | Shared evidence is useful, but setup, runtime readiness, uncertain effects and domain outcomes cannot all be collapsed into one state. C3 requires confirmation of declared shutdown duties before unregistration while preserving outstanding-effect evidence and shared dependencies; exact results stay extensible and typed. |
| Every request names its actor | A supplied name is attribution data until authenticated by the appropriate binding. Same-OS-user execution does not grant an Agent root management rights. C2 preserves authenticated caller context and applicable restrictions while Resources use their own dependency grants. |
| Require two independent full implementations for the first release | C1 selects independently authored client/plugin conformance first and defers a second complete runtime. The initial proof does not establish runtime-to-runtime portability. |
| No skills | Already no Conduit-owned skills framework. Preserve BYO native skills and explicit Agent activation through permitted Resources. |

The proposals' `v1`/`v2` labels, identifiers, paths and message names are suggestions, not accepted
release targets. Preserve the public behavior first; choose serialization together later.

## Additional roadmap candidates

**Sealmesh:** PR #1 proposes it as a messaging backend and describes its room/identity/retention
behavior. Those claims have not been independently revalidated in this pass. Evaluate the actual
implementation before choosing an adapter. It could fit as a Pool-owned communication Resource
if its declared guarantees match the selected retained-history capability. Adopting it must not
make rooms, GitHub organizations, tickets, Tailscale or remote deployment mandatory core concepts.
Conduit's accepted retain-until-explicit-purge contract is not superseded by a backend default;
capability/retention gaps must remain visible.

**Local root Agent:** this is distinct from the accepted root Interface and its deterministic
setup/recovery operations. A host-administration Agent could be an optional Agent with explicit
Resource access and delegated controls. It need not replace the Interface, own policy, or become a
mandatory model dependency for startup/repair. Model choice and local-model guarantees are later
questions only if that optional use case is selected.

**VM per environment:** a possible isolated-execution implementation, subject to actual host,
credential, filesystem and network tests. Making it the default would reopen the accepted native
default and compatibility tradeoff. Do not adopt that change merely by choosing a sandbox driver.
Remote scheduling and one-VM-per-account/process are not consequences of the five-family model.

## Prior art still applied

- No generic deep-merge of permission lists. [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONFIGURATION.md#L126-L126)
- Labels and selectors as general metadata for choosing sets of objects, such as a Pool's entries, were not adopted; placement's node selectors are (PLC-007). [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONFIGURATION.md#L121-L121). Whole-object revision checks avoid implicit overlay/field ownership. [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONFIGURATION.md#L220-L238)
- Terraform’s “Removing a declaration is not permission to delete the external object” comes from CONFIGURATION, not CONSTRUCTION. [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONFIGURATION.md#L128-L128). CONSTRUCTION retains the corresponding recovery cases. [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CONSTRUCTION.md#L82-L95)
- Avoid a hidden readiness cycle: startup/output guarantees must let migrations run before stronger readiness checks. [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/RUNTIME_ESTABLISHMENT.md#L169-L186)
- Retain progress without claiming readiness, managed effects after migration failure, revalidation of expired handles, explicit retries of valid progress, input snapshots, evidence after frontend disconnects and withholding protected work when authorization cannot be verified. [Failure cases at C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/RUNTIME_ESTABLISHMENT.md#L266-L283)

## Repository and ecosystem source material

Historical proposals, not new decisions. The organization/publication decisions now live in
[ruling 47](../proposals/ruling-47/README.md); remaining work is A75 and A76.

## Conclusions to test, not rules already adopted

1. One source repository can produce several public artifacts. Repository, crate, package,
   executable, process and object-instance boundaries answer different questions.
2. Public protocol independence needs accessible contracts, explicit capability claims and tests
   against external clients/plugins. Merely moving the spec to another repository does not supply it.
3. A standard distribution can bundle a convenient working selection without making its contents
   mandatory for every compatible implementation.
4. Plugin runtime communication and package delivery are distinct protocols. MCP does not decide
   package identity, dependency resolution, releases or executable retention.
5. Website navigation should follow user and author tasks; the repository should follow authority,
   ownership and build/release boundaries. They need not have identical trees.

## 5. Release and delivery proposal

Use one coordinated reference-distribution release initially. It records exactly which protocol
capabilities, package artifacts and platforms were tested together. Core Rust crates can follow that
release without promising stable public APIs for each crate.

Keep logical version domains separate: protocol/contract version, package implementation release and
artifact identity, reference application release, and internal storage-format compatibility. An app
patch should not require changing every capability contract's version. Conversely, two releases claiming
the same shape do not establish identical semantics. Exact version syntax still belongs in the authoring pass.

First distribution: prepared archives for validated macOS targets with a readable inventory and integrity
metadata, plus source build instructions. A Homebrew entry can wrap the same tested artifacts later.
Do not claim Linux/Windows validation from portable Rust code alone. No npm/Python runtime should be
required for the base Rust distribution merely because some extensions use those languages; each such
extension declares its actual prerequisites.

Keep v0's accepted explicit local package installation. A user can download a package release and install
it locally without a hosted Conduit marketplace. Future OCI, Git or other delivery mechanisms can carry
the same package contract. Neither a registry nor automatic cross-language dependency resolution is
needed to prove authoring. External MCP services keep separately reported compatibility; pinning the
Conduit adapter cannot freeze a remote service implementation.

Installing a release must preserve artifacts bound to retained instances. Package installation still
does not instantiate templates, authenticate services or grant access. Test built release artifacts
outside the source workspace so workspace paths cannot hide missing schemas, executables or dependencies.

## 6. Website and documentation experience

Lead with the protocol and original authoring. The earlier product-first navigation was too centered
on installing templates and composing existing integrations. The proposed replacement is:

| Route | First useful experience |
| --- | --- |
| Learn the protocol | Understand the five families, language, contracts and how their relationships express a system. |
| Author with the protocol | Define capabilities, operations and requirements; implement a small plugin; construct templates and package contributions. |
| Protocol reference and conformance | Find exact semantics, representations, MCP mappings, compatibility and observable guarantees. |
| Reference implementation | Execute and inspect the contracts using the Rust runtime and clients; distinguish those mechanisms from protocol requirements. |
| Project and ecosystem | Find source responsibilities, experiments, proposals, research, archived alternatives and supported ancillary packages. |

An introductory example should define a small new capability, supply its implementation, use it in
a Resource template, establish it through a Pool and connect an Agent/Interface through a Conduit.
Inspection and a failure case should demonstrate its guarantees. Reusing existing integrations is a
later application of this language. Defining an extension must not require reading executor SQL or
modifying core dispatch. These are documentation priorities, not a new requirement to implement every
capability before the first authoring example can be reviewed.

Separate tutorials/task guides from exhaustive reference and architecture explanation. Keep research
and dissent reachable under contribution/design context without presenting them as supported features.
Until implementation exists, the site should say protocol design/development and link reviewable examples;
it should not display an installation quickstart that cannot run.

Keep site configuration in the main repo initially; a separate deployment does not require a separate
content repository. Choose a static-site framework after the content paths and version strategy are clear.
Development docs and later released docs must be distinguishable. Package docs should identify the exact
release/contract they describe. Conformance status, platform support and experimental features should be
visible alongside the relevant instructions.

## Attribution

The collaborator `allocmemory` submitted [PR #1](https://github.com/shrav-k/conduit/pull/1)
at `2111b0d` and [PR #2](https://github.com/shrav-k/conduit/pull/2) at `c480e96`, merged as
[`71d6348`](https://github.com/shrav-k/conduit/commit/71d634879be68399c080450f0c2e8fa401d5b8e8) and
[`4b3db58`](https://github.com/shrav-k/conduit/commit/4b3db583fdfbcd5c95c36aefa85bad4d73748b12).
The vocabulary research and simplification pass came through
[PR #5](https://github.com/shrav-k/conduit/pull/5) at `2890ef7`. Original authorship, dissent
and applicable third-party terms remain; Apache-2.0 adoption asserts no additional grant
for reproduced submissions. See [license and attribution](../../CONTRIBUTING.md#license-and-attribution).

Pi reference pins `0441483` and `04025a6` remain in the
[reference-repository record](plan-audit-history.md#reference-repositories), pending A27’s
reconciliation; no new claim of upstream availability is made.
