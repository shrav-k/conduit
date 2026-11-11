# 019 — S8: search that returns only what the caller may see

> **For the owner**
> - **Accepted under ruling 75 D on 2026-11-11.** Every item labelled proposed below is accepted,
>   with the answers under "Open for the owner" applied.
> - **Proposed:** P3 — compare returned content with an independent visibility oracle.
> - **Proposed:** P4 — apply visibility to derived content, metadata and cached answers.
> - **Proposed:** P5 — refuse unsupported filters and stop when authorization cannot be judged.
> - **Proposed:** P6 — retain filters and attribution through search's onward storage calls.
> - **Proposed:** M0 — carry the common author measures and selected counting rules.
> - **Proposed:** M1 — count disclosure and missing-content mismatches by caller and surface.
> - **Proposed:** M2 — count authorization-change, cache and onward-call observations.
> - **Proposed:** V1 — `s8-visible-search-results`.
> - **Proposed:** V2 — `s8-derived-content-visibility`.
> - **Proposed:** V3 — `s8-authorization-change-and-cache`.
> - **Proposed:** V4 — `s8-unsupported-or-unavailable-filter`.
> - **Proposed:** V5 — `s8-onward-filter-chain`.
> - **Proposed:** two author runs over the same synthetic fixture and fault schedule.
> - **Open:** Q1 — Which subscription accounts should sign in for the runs?
> - **Open:** Q2 — Which storage/search tools and content-filter contracts should be qualified?
> - **Open:** Q3 — Which rule should govern a derived answer with mixed source permissions?
> - **Open:** Q4 — Accept P3–P6, M0–M2, V1–V5 and the run plan below, or revise them?

## Status

Pre-registered on 2026-11-11 by work order 103. Accepted by the owner on 2026-11-11 under
[ruling 75](../../docs/proposals/ruling-75/README.md), D: its pass conditions, measures,
counting rules and planned vectors, with the answers under "Open for the owner" applied. Ruling
45 fixes what an experiment must show before it runs. No result is recorded here.

## Question

Can search over storage return only what its caller may see, including returned and derived
content, as [S8](../../ROADMAP.md#scenarios) asks? The question concerns the meaning and
implementation of the selected filter contracts, as well as propagation through Conduits.

## What it must show

1. **P1 — ruled:** S8 runs end to end on the reference engine with its vectors passing;
   judge by task evidence and the vector report ([ruling 47](../../docs/proposals/ruling-47/README.md)).
2. **P2 — ruled:** every safety vector added here passes and names its safety promise
   ([ruling 68, point 2](../../docs/proposals/ruling-68/README.md)).
3. **P3 — proposed, for the owner's acceptance:** for the fixed queries, each caller gets exactly
   the expected visible document IDs and text, including empty results. Judge against an oracle
   computed independently from the fixture's permissions, never from the implementation's answer.
4. **P4 — proposed, for the owner's acceptance:** titles, snippets, counts, facets, citations,
   resource links, errors, summaries and cached answers disclose no content or existence derived
   from a source the caller cannot see. Expected counts and ranking use the visible corpus only;
   an authorized control caller still gets its complete expected answers. This tests a proposed
   package-contract rule (Q3), not a content policy already supplied by the core requirements.
5. **P5 — proposed, for the owner's acceptance:** an unsupported filter prevents opening its
   Conduit (CON-022); a filter that cannot be evaluated causes unavailable with no work or
   content (ACC-014, IFC-019). Judge wire answers, records and storage-call counts together.
6. **P6 — proposed, for the owner's acceptance:** search's onward storage requests carry the
   caller and every applicable filter, use search's own storage Conduit, and produce correctly
   attributed records. An absent onward Conduit permits no call; compare ACC-016, ACC-017 and
   IFC-020 with the captured chain. Closing A's search access refuses its next request while B's
   authorized query still works. A cached tool list confers no permission.

## What it covers

- S8 has **no row** in [round 6's allocation](../../ROADMAP.md#round-6s-evidence-and-measurements).
  No E case or D measure is assigned to it. Its filter questions overlap the critique's
  [D05](../../docs/proposals/reaudit/round-6/design-critique.md#d05--a-filter-contract-must-explain-meaning-not-just-advertise-support);
  this does not redefine or claim S2's D05 evidence.
- [contracts.yaml](../../spec/promises/contracts.yaml): CTR-002, CTR-004, CTR-005 — operation,
  Type and Requirement contracts; [schemas.yaml](../../spec/promises/schemas.yaml): SCH-001,
  SCH-009 — validation and defaults, not a definition of search visibility.
- [packages.yaml](../../spec/promises/packages.yaml): PKG-019, PKG-020, PKG-031;
  [manifest.yaml](../../spec/promises/manifest.yaml): MAN-009, MAN-010, MAN-017 — implementations,
  truthful `supports`, bindings and standard Conduit templates.
- [conduits.yaml](../../spec/promises/conduits.yaml): CON-022; and
  [access.yaml](../../spec/conventions/access.yaml): ACC-001, ACC-003, ACC-008, ACC-009,
  ACC-014, ACC-016, ACC-017, ACC-027 — request-local restrictions, onward filters, mediated access.
- [implementation.yaml](../../spec/interface/implementation.yaml): IFC-007, IFC-018–IFC-021,
  IFC-025, IFC-026, IFC-028, IFC-033 — filter wire, errors and self-contained schemas.
- [core/requirements.yaml](../../spec/core/requirements.yaml) defines only bound/shared-host;
  neither defines returned-content visibility. [providers.yaml](../../spec/promises/providers.yaml)
  PRV-007 and [placement.yaml](../../spec/promises/placement.yaml) PLC-007 cover fixture execution.
  [secrets.yaml](../../spec/promises/secrets.yaml) SEC-019 applies if an implementation receives a
  secret; this fixture needs none.
- Feeds A39's missing content/filter vectors and A50's scenario evidence; overlaps S2's
  restrictions. S8's scenario row is its register row; no new row is needed.

## The tasks

1. **S8-T1:** declare storage and search Resources and their contracts; create Agents A and B
   with different visibility filters, plus a root-driven authorized control caller.
2. **S8-T2:** open the intended search and onward storage Conduits, inspect each restriction
   and have the real Agents run the same fixed direct-search queries.
3. **S8-T3:** ask for summaries, snippets, facets, citations and links; compare answers with the
   visible-only oracle, including a query whose only matching document is hidden.
4. **S8-T4:** query before and after the prepared authorization change; repeat the same read
   arguments and key, then query through a fresh invocation and a warmed cache.
5. **S8-T5:** attempt the unsupported-filter configuration and query while authorization data
   is unavailable; inspect the refusal/unavailable evidence without widening access.
6. **S8-T6:** inspect the onward chain, close A's search Conduit, check A's refusal and B's
   continued results, then explicitly restore A's intended access.

Preparation, apart from author tasks: freeze a small synthetic corpus, query list and oracle.
Include A-only, B-only, shared and neither-visible documents, distinctive hidden sentinel strings,
mixed-source summaries, an empty result, pagination and a hidden-only match. The harness changes
one permission version between calls, warms caches under a broader caller before a narrower one,
and makes the authorization source unavailable. It captures an attempted onward call with no
storage Conduit and an unsupported filter. These faults are not author recovery work until the
author responds. No real personal data, credential or live organizational index is used.

## Measures

- **M0 — proposed, for the owner's acceptance (order 103, decision 4):** carry by reference the
  [first build's table](../../docs/implementation/FIRST_BUILD.md#measures), and only “Provider
  work apart”, “A client's concepts count” and “Requests and time without access” from its
  [counting rules](../../docs/implementation/FIRST_BUILD.md#counting-rules-added-after-the-baseline-ran).
  Apply per S8 task; show filter/adapter implementation work separately. Follow the
  [increment-4 ledger model](../../docs/proposals/first-build/increment-4/measurement-plan.md).
- **Ruled counting:** carry [ruling 63's line count](../../docs/proposals/ruling-63/README.md),
  [ruling 68 R1's panel actions apart](../../docs/proposals/ruling-68/README.md), and
  [IN_FORCE.md topic 13's per-run rules](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement).
  The source rules hold; M0's extension is proposed. Do not carry A70's comparison/bar, the
  S1 T3 reach-around rule, its T4/T5 better-baseline rule or its local name `project`.
- **Ruled:** observed time and mistakes precede any numeric target (ruling 46, point 3).
  S8 has no numeric bar.
- **M1 — proposed, for the owner's acceptance:** per caller, query and output surface, list the
  expected and actual visible IDs, missing authorized items, unauthorized items and sentinel
  disclosures. Count each distinct mismatching item/surface once per response; list repetitions
  separately. Counts/facets/ranks and derived outputs get their own oracle checks and source-ID
  sets; absence of a sentinel alone is not evidence of no disclosure. This is an exact fixture
  correctness test, not a statistical quality or numeric performance target.
- **M2 — proposed, for the owner's acceptance:** per scheduled change, record permission version,
  caller, key, invocation ID, cached/fresh route, outcome/reason, onward dispatch count and filters.
  Compare before/after answers and record each stale disclosure or unauthorized dispatch.
  Count manual repair requests and intervals without intended access by M0; record authorization
  lookup/cache waits and each fault's injection/recovery times apart from author time.

## Vectors expected

Every row is **proposed, for the owner's acceptance**; names are proposed, no vectors are written.
All are safety vectors under [the tracker](../../docs/AUDIT.md#how-to-use-this-document) and ruling 68.

| ID / proposed name | Observation and cited promises | Safety promise |
| --- | --- | --- |
| V1 `s8-visible-search-results` | Same query, distinct visible content per caller: ACC-008, IFC-019, CTR-005 | IFC-019 |
| V2 `s8-derived-content-visibility` | Mixed-source summaries, metadata, links and errors obey the accepted filter contract: IFC-019, ACC-017 | IFC-019 |
| V3 `s8-authorization-change-and-cache` | Changed permissions and warmed broad-caller cache reveal nothing newly forbidden; read repeat checked: IFC-019, OPS-009, ACC-001 | IFC-019 |
| V4 `s8-unsupported-or-unavailable-filter` | Unsupported opening rejected; unevaluable filter performs nothing: CON-022, ACC-014, IFC-019 | ACC-014 |
| V5 `s8-onward-filter-chain` | Caller/filters preserved, own onward Conduit required, revoked entry refuses: ACC-001, ACC-016, ACC-017, IFC-020, IFC-028 | ACC-017 |

V1–V3 need the missing concrete filter contract before expected content can be encoded.
OPS-009 is in [operations.yaml](../../spec/conventions/operations.yaml); distinguish read redispatch
from the non-dispatched repeats that ACC-008 exempts from a fresh filter judgment.
Existing [access-plain-registration](../../conformance/conventions/access/plain-registration/case.yaml)
covers rejection of a plain filtered target; [access-s1-plain-filter](../../conformance/conventions/access/s1-plain-filter/case.yaml)
covers an onward carried filter meeting a plain implementation. Neither covers search content.

## Runs

**Proposed, for the owner's acceptance:** two separate runs, coordinator then fresh Codex author,
with identical frozen fixtures, queries and fault schedule; report separately, never average.
The fresh author receives `spec/`, tasks, accepted package contracts and CLI format reference,
without ruling history or a finished project. Run Claude Code and Codex as A and B through the
engine's Agent packages on one Mac; root is the control caller. Record all exact versions and pins.

Use isolated Agent homes, scratch storage/search datasets, local runner, and owner-selected
existing storage/search tools behind their packages (Q2). No paid service or shared service
credential is proposed. Needed packages: the two Agents, storage, search and filter-aware adapters.
Owner presence/time: sign in at start and on lapse, answer any pending actions, and record each
panel action's actual duration apart. No planned background access to personal accounts is needed.

Retain dated results here later: task brief/prompts, corpus and permission snapshots, oracle and
provenance, package pins, authored artifacts/diffs, per-task measures, captured wire/records and
client/model output, fault timeline, vector report and all deviations. Use `<lab>` scratch paths;
retain no credentials or sign-in codes. Fix the oracle before the first timed run.

## What it needs first

Owner-accepted concrete Requirement and search/storage Operation contracts, with two qualified
filter-aware implementations where S2's D05 supplies them; truthful `supports` (CTR-005, CON-022)
is essential. Implement ACC-017/IFC-020 onward identity/filter delivery, IFC-019 application to
returns, OPS-009 read repeats and ACC-014 no-work failure in the reference engine and packages.
The engine's current S1 slice is not this evidence. The owner selects accounts and the tools;
the coordinator supplies the independent corpus/oracle and permission/failure controls.

## What `spec/` does not yet decide

- **G2 — gap:** CTR-005 and IFC-019 require a filter's application but no current Requirement
  defines which stored documents a caller may see, how authorization changes are interpreted,
  or how a returned search hit, snippet, count, rank or link is constrained. The core requirements
  supply no such contract. This is package-contract work, not a proposed new core access language.
- **G3 — gap:** ACC-017 limits returned content through carried filters, but no current contract
  defines derived-content provenance, mixed-source visibility or cache reuse across callers and
  permission versions. Q3 must settle the chosen contract before a content oracle/vector is run.
  [IN_FORCE.md topic 6](../../IN_FORCE.md#6-conduits-and-access) cites the general filter promises;
  neither G2 nor G3 has a line marked spec pending. No task assumes an answer.

## Open for the owner

- **Q1: Which subscription accounts should sign in for the runs?** Options: owner-designated
  dedicated test accounts, or the owner's existing accounts in isolated Agent homes.
  **Recommendation:** dedicated test accounts, completed through Conduit's sign-in flow.
- **Q2:** use local filesystem storage plus SQLite FTS search behind small adapters, or an existing
  hosted storage/search service? **Recommendation:** local existing tools with synthetic data;
  qualify their packages first and count every adapter. This proposes no new search infrastructure.
- **Q3:** derive answers only from sources all visible to the caller, or allow mixed-source answers
  with a separately specified redaction/provenance rule? **Recommendation:** visible sources only,
  with visible-only counts/ranking and caches qualified per caller/authorization version.
- **Q4:** accept the proposed criteria, measures, vectors and run plan, or revise them?
  **Recommendation:** accept only after the concrete filter contracts and oracle are reviewed.

**Answers, under [ruling 75](../../docs/proposals/ruling-75/README.md), D (2026-11-11):**

- Q1: the second subscription accounts. Every account has its own label, and every Agent home
  signs in on its own (D2).
- Q2: the local filesystem and SQLite FTS, behind small adapters that are counted (D1).
- Q3: a derived answer draws only on sources the caller may see in full.
- Q4: accepted.

## Limits

This finite fixture cannot prove arbitrary generated summaries safe or a search engine generally
complete. The deterministic derived-answer check tests the selected contract, not model reasoning
or side channels such as timing. It establishes only mediated reach through Conduit (ACC-027);
an Agent's own filesystem/network and account services require separate analysis. No distributed
index, cross-machine partition or production-scale performance claim is made.
