# 018 — S7: an outside author's package

> **For the owner**
> - **Accepted under ruling 75 D on 2026-11-11.** Every item labelled proposed below is accepted,
>   with the answers under "Open for the owner" applied.
> - **Proposed:** P3 — install and use the outside author's two packages without history.
> - **Proposed:** P4 — retain repeated literal arguments and reject an ambiguous binding.
> - **Proposed:** P5 — check label selectors and the plain-server/filter boundary.
> - **Proposed:** M0 — carry the common author measures and selected counting rules.
> - **Proposed:** M1 — apply S2's D01 to this author's package, with separate author costs.
> - **Proposed:** M2 — count packaging corrections and repeated-argument workarounds.
> - **Proposed:** V1 — `s7-outside-package-install-use`.
> - **Proposed:** V2 — `s7-repeated-argument-binding`.
> - **Proposed:** V3 — `s7-label-selector-admission`.
> - **Proposed:** V4 — `s7-plain-filter-refused`.
> - **Proposed:** one outside-author run and a separate non-Rust publication check.
> - **Proposed:** a named synthetic fixture read as the task shared by D01 variants.
> - **Open:** Q1 — Which subscription accounts should sign in for the runs?
> - **Open:** Q2 — Codex or Claude as the outside author?
> - **Open:** Q3 — Which npm server with a repeated named argument should be qualified?
> - **Open:** Q4 — Which language and repository should hold the non-Rust implementation?
> - **Open:** Q5 — Accept P3–P5, M0–M2, V1–V4 and the run plan below, or revise them?

## Status

Pre-registered on 2026-11-11 by work order 103. Accepted by the owner on 2026-11-11 under
[ruling 75](../../docs/proposals/ruling-75/README.md), D: its pass conditions, measures,
counting rules and planned vectors, with the answers under "Open for the owner" applied. Ruling
45 fixes what an experiment must show before it runs. No result is recorded here.

Its order-level plan is [plan.md](plan.md), written by work order 119; the plan changes nothing accepted here.

## Question

Can an outside author package an existing MCP server and a small code plugin, using packages,
contracts, types and label selectors for placement, as [S7](../../ROADMAP.md#scenarios) asks?
Label selectors replaced “location needs” in [ruling 41, item 8a](../../docs/proposals/ruling-41/README.md).

## What it must show

1. **P1 — ruled:** S7 runs end to end on the reference engine with its vectors passing;
   judge by the task evidence and vector report ([ruling 47](../../docs/proposals/ruling-47/README.md)).
2. **P2 — ruled:** every safety vector added here passes, with its safety promise named;
   judge the vector report under [ruling 68, point 2](../../docs/proposals/ruling-68/README.md).
3. **P3 — proposed, for the owner's acceptance:** the fresh author produces installable packages
   for the unchanged server and a small plugin, then real Agents use their exported operations.
   Installation, operation results and records agree with the pinned artifacts and contracts;
   the author consults no history. Any extra guidance is recorded as intervention.
4. **P4 — proposed, for the owner's acceptance:** a verified real server command repeats a named
   argument; literal occurrences arrive in order, while a map binding that names two occurrences
   is rejected under MAN-010. No occurrence-selection syntax is assumed. Record whether existing
   packaging conventions suffice, or leave the author task blocked with its exact need.
5. **P5 — proposed, for the owner's acceptance:** a compatible selector admits the package's
   instance, a conflicting Type/item selector rejects its template, and a nonmatching node
   rejects its instance. A filter aimed at the unchanged plain server is not instantiated.
   Compare diagnostics and absence of execution with PLC-007, PLC-025 and CON-022.
6. **P6 — accepted by the owner:** the package-authoring check includes a non-Rust implementation
   ([IN_FORCE.md topic 14](../../IN_FORCE.md#14-publication-and-organization), ruling 47).
   Retain an installation/use record and its implementation language. This is a publication
   check building on S7, **not a condition of v0**; its location is Q4's answer below.

## What it covers

- [Round 6's table](../../ROADMAP.md#round-6s-evidence-and-measurements) assigns S7 no E01–E20
  case. It assigns D01 and the real repeated-argument package (MAN-010, A63).
- [packages.yaml](../../spec/promises/packages.yaml): PKG-004, PKG-018–PKG-020, PKG-022,
  PKG-026, PKG-031 — archive pinning, exports, implementations, plain and shipped Conduit templates.
- [manifest.yaml](../../spec/promises/manifest.yaml): MAN-001, MAN-004, MAN-007, MAN-009,
  MAN-010, MAN-011, MAN-013, MAN-017 — manifest, layout and bindings.
- [contracts.yaml](../../spec/promises/contracts.yaml): CTR-001–CTR-005; and
  [schemas.yaml](../../spec/promises/schemas.yaml): SCH-001, SCH-013 — contracts and their schemas.
- [placement.yaml](../../spec/promises/placement.yaml): PLC-007, PLC-025; and
  [providers.yaml](../../spec/promises/providers.yaml): PRV-003, PRV-007, PRV-012 — selectors
  and execution, including the relative-command restriction.
- [conduits.yaml](../../spec/promises/conduits.yaml): CON-022; [access.yaml](../../spec/conventions/access.yaml):
  ACC-001, ACC-003, ACC-014, ACC-027; [implementation.yaml](../../spec/interface/implementation.yaml):
  IFC-007, IFC-010, IFC-019, IFC-033, IFC-045 — plain qualification and mediated access.
- [core/requirements.yaml](../../spec/core/requirements.yaml) supplies bound and shared-host,
  not a generic content filter. [secrets.yaml](../../spec/promises/secrets.yaml), SEC-019,
  constrains a Conduit-aware plugin given a secret; this run proposes synthetic data only.
- Feeds A63's integration evidence and A75/A76's later publication check; overlaps S2's D01.
  S7's scenario row is its register row; no new row is needed.

## The tasks

1. **S7-T1:** from the permitted brief, select and pin an eligible existing npm server;
   describe its tools as Operation, Capability and Type contracts and map them with `plain`.
2. **S7-T2:** build its Conduit package archive, exports and standard Conduit templates;
   install it, register instances and have the two Agents perform the agreed fixture task.
3. **S7-T3:** package a small deterministic code plugin for that same task, with a Conduit-aware
   implementation and truthful `supports`; qualify the plain, adapter and aware variants by D01.
4. **S7-T4:** declare compatible placement labels, inspect the result, then submit the prepared
   conflicting-selector and nonmatching-node examples and explain their diagnostics.
5. **S7-T5:** package the repeated named arguments literally, then attempt the ambiguous binding;
   explain the rejection and record any supported workaround without adding language.
6. **S7-T6:** hand the artifacts to a clean consumer project, repeat installation and use,
   and explain each Agent's access through Conduit from inspection and records.
7. **S7-T7:** after Q4 is settled, repeat the plugin authoring check with the selected non-Rust
   implementation, counted separately as the publication check.

Preparation, apart from author tasks: the harness supplies synthetic fixture data and expected
answers, a clean engine store, pinned upstream bytes, incompatible selector examples and an
ambiguous-binding example. It injects no real credential and gives the author no finished package.
**Proposed, for the owner's acceptance:** the common task is reading the exact text of a named
synthetic fixture through each D01 variant; the plugin returns that fixture deterministically.

## Measures

- **M0 — proposed, for the owner's acceptance (order 103, decision 4):** carry by reference the
  [first build's table of measures](../../docs/implementation/FIRST_BUILD.md#measures) and exactly
  its counting rules “Provider work apart”, “A client's concepts count” and “Requests and time
  without access” in [Counting rules](../../docs/implementation/FIRST_BUILD.md#counting-rules-added-after-the-baseline-ran).
  Apply them per S7 task, including package-author work; list package implementation work apart
  without hiding it from the outside author's total. The
  [increment-4 plan](../../docs/proposals/first-build/increment-4/measurement-plan.md) models the evidence ledger.
- **Ruled counting:** carry [ruling 63's line count](../../docs/proposals/ruling-63/README.md),
  with [ruling 68 R1's panel actions apart](../../docs/proposals/ruling-68/README.md), and the
  per-run rules of [IN_FORCE.md topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement).
  These source rules hold; M0's extension to S7 is proposed. No A70 comparison/bar, S1 T3
  reach-around rule, T4/T5 better-baseline rule or local name `project` is carried.
- **Ruled:** record observed time and mistakes before setting any numeric target (ruling 46,
  point 3). S7 has no numeric bar.
- **M1 — proposed, for the owner's acceptance:** use **D01**, defined only in
  [S2's pre-registration](../013-s2-shared-login/README.md), for this outside author's package.
  Attach its qualification table and evidence to S7, with costs per variant and task; do not
  redefine D01 here. Its final definition must be available and accepted before this run.
- **M2 — proposed, for the owner's acceptance:** count each rejected installation/submission,
  diagnostic, correction and extra author intervention by task; list each repeated-argument
  workaround in files, lines and changed argv positions. Keep unchanged upstream code, new
  adapter code and plugin code separate; a blocked attempt is recorded, never called a pass.

## Vectors expected

Every row is **proposed, for the owner's acceptance**; names are proposed, not created vectors.
Safety follows [the tracker's definition](../../docs/AUDIT.md#how-to-use-this-document) and ruling 68.

| ID / proposed name | Observation and promises | Safety; safety promise |
| --- | --- | --- |
| V1 `s7-outside-package-install-use` | Pinned archive installs; exports reach the mapped tool: PKG-004, PKG-018–PKG-020, PKG-026, MAN-009 | Yes; PKG-026 (altered bytes rejected) |
| V2 `s7-repeated-argument-binding` | Literal repetitions retained; ambiguous binding rejected: MAN-010 | Yes; MAN-010 (no wrong argument bound) |
| V3 `s7-label-selector-admission` | Matching node accepted; contradictory selectors and wrong node rejected: PLC-007, PLC-025 | Yes; PLC-007 (no incompatible placement) |
| V4 `s7-plain-filter-refused` | Plain item supports no filter; filtered Conduit rejected: PKG-020, CON-022, IFC-007 | Yes; CON-022 (no unenforceable access opened) |

Existing partial coverage: [digest-mismatch](../../conformance/language/packages/digest-mismatch/case.yaml),
[package-binding](../../conformance/language/packages/package-binding/case.yaml),
[placement-label-darwin](../../conformance/language/registration/placement-label-darwin/case.yaml),
and [access-plain-registration](../../conformance/conventions/access/plain-registration/case.yaml).
They do not establish a real outside author's success or repeated-argument qualification.

## Runs

**Proposed, for the owner's acceptance:** one fresh outside-author session, recommended Codex,
with only `spec/`, package format/contracts, task brief and the CLI format reference. Isolate its
checkout from repository history and these decision records, as increment 4 isolated its author.
The coordinator prepares/checks fixtures and keeps interventions; it is not the outside author.
One S7-T1–T6 run plus a separately reported T7 publication run; failed attempts remain evidence.

Use one Mac, reference engine/CLI and its local runner, Node/npm and pinned npm stdio artifacts;
Claude Code and Codex as the two consuming Agents, each in its own home. Record exact versions,
hashes, commands and the run's starting revision before timing. The packages under test, the two
Agent packages and any fixture upstream are the complete service inventory; no shared login is needed.
Owner presence is needed for account sign-ins and any pending action, at start and on lapse;
record actual panel time separately, with no assumed duration or unattended-login claim.

Candidates are **unverified, chosen from remembered CLI shapes, without network access**:
`mcp-remote` (recommended: repeated `--header`, npm stdio front end, requires a disposable HTTP MCP
upstream with synthetic headers); `@playwright/mcp` (candidate repeated `--init-script`, npm stdio,
requires browser binaries and a synthetic page). Verify the package, exact argv and repeats before
selection; neither recollection is evidence. A Docker-only candidate needs Docker and runtime
work beyond today's npm stdio runner (DL16), and cannot silently replace an eligible npm candidate.

Keep dated results beside this file: brief, prompts, authored files/diffs, package hashes, argv
capture, diagnostics, actual client/model observations, request/record IDs, vector report and
per-task count ledger. Use `<lab>` for scratch paths; retain no credential values or sign-in codes.

## What it needs first

Install/archive admission and bindings (PKG-026, MAN-010), selector checks (PLC-007, PLC-025),
plain tool mapping (PKG-020, IFC-007) and real Agent adapters must work in the pinned engine.
The plugin and adapter must implement their declared contracts, including IFC-019 if filtered.
D01's definition from order 101, the owner-accepted fixture task and verified eligible upstream
are prerequisites. The owner supplies accounts and completes sign-ins; the coordinator prepares
an isolated brief and capture of synthetic argv. Q4 must precede any non-Rust code placement.

## What `spec/` does not yet decide

- **G1 — gap:** MAN-010 rejects a name matching multiple arguments; no promise binds one
  occurrence or its several values via `isRepeated`. [IN_FORCE.md topic 11](../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items)
  marks this **Open**, not spec pending. S7 records the real need; no task assumes a new binding.
- The non-Rust implementation's location is a repository/owner decision, not missing language:
  topic 14 accepts the publication check while topic 13 and AGENTS.md keep this repository's code Rust.

## Open for the owner

- **Q1: Which subscription accounts should sign in for the runs?** Options: owner-designated
  dedicated test accounts, or the owner's existing accounts in isolated Agent homes.
  **Recommendation:** dedicated test accounts, completed through Conduit's sign-in flow.
- **Q2:** outside author Codex or Claude? **Recommendation:** fresh Codex, matching increment 4;
  either gets the identical restricted brief.
- **Q3:** `mcp-remote`, `@playwright/mcp`, or another verified npm stdio server with repeats?
  **Recommendation:** first qualify `mcp-remote` with a synthetic local upstream; if ineligible,
  return a verified candidate and its dependencies before starting the author run.
- **Q4:** TypeScript or Python, in an author-owned repository or an explicit in-tree exception?
  **Recommendation:** TypeScript/npm in an author-owned repository with pinned evidence retained
  here later; the existing code rule is not waived by this proposal.
- **Q5:** accept the proposed criteria, measures, vectors and run plan, or amend them before use?
  **Recommendation:** accept subject to candidate qualification and the final D01 definition.

**Answers, under [ruling 75](../../docs/proposals/ruling-75/README.md), D (2026-11-11):**

- Q1: the second subscription accounts. Every account has its own label, and every Agent home
  signs in on its own (D2).
- Q2: a fresh Codex session is the outside author.
- Q3: qualify `mcp-remote` first.
- Q4: TypeScript on npm, in a repository the author owns; this repository's Rust rule stands.
- Q5: accepted.

## Limits

One outside author and a small fixture do not establish broad MCP compatibility or general
author usability. Plain servers cannot enforce arbitrary filters. This does not show provider
substitution, remote placement, OS confinement or safety outside the domain of trust; ACC-027
reports restrictions as mediated. The non-Rust publication check does not add a v0 gate.
