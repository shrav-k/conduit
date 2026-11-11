# Plan audit: history

Not binding. This file holds the sections of the [plan audit](../AUDIT.md) that record how the
work stood at the audit of 2026-11-05 and in the days after, and the action items that are done
or superseded. Ruling 39 moved them here on 2026-11-06, so that the tracker holds what is
current. They are kept as they were written, apart from links, which point where they did,
and the names of two private repositories, which ruling 39 removed. Sections marked
**Historical** were not maintained after the rulings that replaced their concepts.

## Completed and superseded action items

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A47 | Owner rulings on the [form of the specification](../proposals/normative-surface/README.md): NS1 to NS4. | A32, A33 | — | done: ruling 17 |
| A48 | Owner rulings on the [layer decisions](../AUDIT.md#layer-decisions) LD1 to LD5. | Ruling 13 | — | done: ruling 14 |
| A49 | State the language's purpose and agent-first differentiation, and turn it into tests applied to every rework decision. | Ruling 14 | The purpose and the tests are recorded and the owner has accepted them. | done: [purpose](../AUDIT.md#purpose) and the nine preservation tests accepted (rulings 15, 17) |
| A55 | Naming patterns: research how existing systems name packages and refer to their contents, then decide how Conduit does, across packages, exports, types and capabilities. | Ruling 24, L27 | The naming scheme is ruled and written as promises. | done: ruled (rulings 25, 27) and written as promises NAM-001 to NAM-017; vectors pending |
| A56 | Consolidation audit: find every place where one idea has several unrelated mechanisms, notations or keys, or one key has several meanings. | Ruling 25 | Each finding is consolidated, or recorded as a deliberate distinction. | done: all 26 findings ruled (rulings 27 to 29) and applied to the specification in one pass; the [consolidation record](../proposals/consolidation/README.md#rulings) lists the rulings and the choices made while writing them, which the owner has not ruled on one by one |
| A57 | Write the four remaining shared conventions: ownership and lifecycle, declared against observed state, records, and nothing implicit. | Ruling 29 | Each is a file of promises under `spec/conventions/`. | done: ruling 30; written as LIF-014 to LIF-023, ACC-033, PLC-016, OBS-001 to OBS-006, REC-001 to REC-006 and IMP-001 to IMP-007 ([record](../proposals/conventions/README.md)) |
| A58 | Decide the schema of each family's `spec` and of contracts and manifests, the implementation interface, and the implementation-defined list. | Ruling 30 | Each is written in `spec/`; the readiness conditions G2 and G9 can then be worked on. | done: ruling 31; written as TPL-001 to TPL-012, CTR-001 to CTR-006, IFC-001 to IFC-017, the implementation-defined list, and additions to the object, graph, Conduit, Pool and package promises ([record](../proposals/schemas-and-interface/README.md)). The JSON Schemas are A60. |
| A60 | Write the concrete surface that the first re-audit found missing (re-audit decision G1): the core package as documents, the JSON Schemas of every document, and the manifest's format. | Ruling 32 | Each is a normative part of `spec/`, checked by `tools/check_schemas.py`, and accepted by the owner. | done: accepted (ruling 37) with five changes, which are applied as [the instructions](../proposals/core-and-schemas/README.md#outcome) set out. Written: `spec/core/`, `spec/schemas/` with valid and invalid examples, and `spec/promises/manifest.yaml` ([record](../proposals/core-and-schemas/README.md)) |
| A1 | Permit exploratory code. | Q9 | — | done: ruling 3 |
| A2 | Name the acceptance scenario. | Q1 | — | done: named as S1 and S2 (rulings 4, 9); not yet run. Real Agents and the four-account check are later gates |
| A9 | Decide how v0 honors multi-system support. | D4, Q11 | — | done: ruling 5 |
| A31 | Settle placement in the language. | Q11 | — | done: ruling 7; written as promises in [placement.yaml](../../spec/promises/placement.yaml) |
| A36 | Settle instantiation arguments. | Q12 | — | done: ruling 8; written as promises in [instantiation.yaml](../../spec/promises/instantiation.yaml) |
| A37 | Redesign the experiment set. | Ruling 9 | — | done: see [experiment program](../AUDIT.md#experiment-program) |
| A26 | Define the independent author. | Q4 | — | done: a fresh session that did not write the language (ruling 11) |
| A46 | Apply the repository readiness audit. | Ruling 12 | — | done: 2026-11-05, in the entry documents, agent instruction files, status notes and ignore rules |
| A3 | Cold-author test. | E0 | — | superseded: by the cold-reader sweeps of A42 |
| A30 | Breadth scenarios unlike the acceptance scenario. | Scope guard | — | done: S3 and S8 in the suite |
| A15 | Recovery and outage proofs under a simulated network. | E7 | — | superseded: by A40 for the model and A34 for the engines |
| A23 | Map Experiment 002's cases onto promises and vectors, then remove it. | D3, ruling 9 | Every case is covered by a vector or dropped with a reason. | superseded: ruling 39 removed Experiment 002, a plan with no code; the scenario program (A14) covers what it planned |
| A28 | Decide the final vocabulary, with merging of concepts permitted, folding in the reviewer's simplification pass. | D7, ruling 16 | Decisions V1 to V8 of the [vocabulary proposal](../proposals/vocabulary/README.md) are ruled. | done: rulings 17 and 18 |

## Defect closure plan

> **Historical.** Recorded at the audit, before rulings 13 to 37. Not maintained. The plan is
> complete: the triage it describes is done.

The register's 167 defects were found against the withdrawn claim. After the layer decisions were
ruled, each defect was assigned to a layer:

| Layer | What happens to the defect |
| --- | --- |
| Language or shared convention | Fixed in the language, with a promise and a vector. Only these count for readiness condition G7. |
| Type contract | Recorded against that type and decided when its contract is written. |
| Runtime profile | Decided for the reference engine, recorded and tested there. |
| None | Closed with the reason, where the layering makes the question disappear. |

The triage is recorded in the [defect register](../LANGUAGE_DEFECTS.md#triage-by-layer): 48
language, 39 shared convention, 30 type contract and 50 runtime profile. The estimate made before
triage, about a third in the first two layers, was low; the figure is about half.

## Behaviors not carried into the specification

For the owner. The audit of the documents outside `spec/` (A61) compared the older text with the
specification. It found nineteen accepted behaviors that the specification did not state. The
[conversion list](../proposals/docs-audit/GAPS.md) says how each is written as a promise, a field
or an item of the implementation-defined list (decision DA3). Eighteen are converted, and the
[log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/log-gaps.md) lists every promise that was changed or added for them.

One of the nineteen is not converted, because it needs a decision first: secrets never appear
in configuration, records or events, and a credential is never handed to a caller (GP3 of the
conversion list). Stating it needs a way to mark a value as secret. Ruling 38 gave the
direction: a secret is referred to and never written as a literal. The secret type is
[proposed](../proposals/secrets/README.md) and not yet ruled.

The audit also found six behaviors that the older text accepted and that appear to have been
dropped rather than missed. The owner confirmed all six as dropped in ruling 38, point 2. They
stay listed here as the record of what was dropped. The links show the older text
as it stood at commit `de6566a` on `main`, the last before the specification was merged.

| # | Behavior the older text accepted | Where |
| --- | --- | --- |
| 1 | For an operation composed of others, the public result contract is derived by default from its declared public output bindings and its constituents' contracts. | [MODEL.md, lines 216 to 221](https://github.com/shrav-k/conduit/blob/de6566ad8a2ea408876a66ab04930734c7b334de/docs/protocol/MODEL.md#L211-L216) |
| 2 | One explicit action may combine creation and start, reporting their outcomes separately. | [LIFECYCLE.md, lines 62 to 63](https://github.com/shrav-k/conduit/blob/de6566ad8a2ea408876a66ab04930734c7b334de/docs/protocol/LIFECYCLE.md#L57-L58) |
| 3 | Each request states the features it requires, and the receiver checks that it supports them before acting. | [LIFECYCLE.md, lines 135 to 137](https://github.com/shrav-k/conduit/blob/de6566ad8a2ea408876a66ab04930734c7b334de/docs/protocol/LIFECYCLE.md#L130-L132) |
| 4 | A caller can replace a package's dependency through a binding the package explicitly exposes. This document already lists it under [deferrable scope](#deferrable-scope). | [MODEL.md, lines 233 to 236](https://github.com/shrav-k/conduit/blob/de6566ad8a2ea408876a66ab04930734c7b334de/docs/protocol/MODEL.md#L228-L231) |
| 5 | An instance can be exported: its current stored configuration with its fixed definition and explicit dependencies, as an input for making a fresh instance. | [CONFIGURATION.md, lines 70 to 80](https://github.com/shrav-k/conduit/blob/de6566ad8a2ea408876a66ab04930734c7b334de/docs/protocol/CONFIGURATION.md#L65-L75); [rule PL-1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/placement/README.md#promises-this-adds) of the [placement record](../proposals/placement/README.md) |
| 6 | An update to an existing object names the object's expected stable ID as well as the version it was based on. Ruling 37 made requests name objects by reference only. | [CONFIGURATION.md, lines 516 to 519](https://github.com/shrav-k/conduit/blob/de6566ad8a2ea408876a66ab04930734c7b334de/docs/protocol/CONFIGURATION.md#L511-L514) |

## Findings

> **Historical.** The questions and disagreements are worded as the audit recorded them, before
> rulings 13 to 37. Their statuses were brought up to ruling 37; the open ones are tracked by
> the action items they name. The [standing probes](#standing-probes-and-comparisons) are live.

### Needs clarification

| ID | Question | Detail | Status |
| --- | --- | --- | --- |
| Q1 | What does v0 do that a plain MCP configuration cannot? | Sharing one service with two Agents takes about 260 lines across six documents in the Experiment 001 fixture, against about five lines today. | done: S1 and S2 name it, one login shared under per-Agent restriction, plus shared history |
| Q2 | What is the protocol, normatively? | No MCP revision is named in the protocol reference, the profile keys are unselected and the wire between services is open. | answered: ruling 31, for the wire to implementations and consumers; the wire between an engine's own services is not shared (LD3) |
| Q3 | How does a native Agent prove who it is? | Every access rule rests on an authenticated originating caller, but [owner and client recognition](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#interfaces-and-the-user-workflow) is left open. | in progress: A11 |
| Q4 | Who is the independent author? | The project has one owner and the sessions that wrote the language. | done: A26 |
| Q5 | What is the granularity of a Conduit instance? | Per consumer and target, or shared; grant lifetime for a crashed Agent. | answered: a Conduit has one consumer and one target (ruling 17), and what closes one is ruled (rulings 19, 28, 29) |
| Q6 | What is the outcome model? | Prerequisites require successful completion, but no representation of success, failure or uncertainty exists. | answered: ruling 20 |
| Q7 | What runs when no frontend is attached? | Evidence must persist, yet no always-running process is required. | open: A17 |
| Q8 | What survives Agent replacement? | A replacement does not automatically acquire private context or grants. | answered: A25 |
| Q9 | When may code be written? | — | done: ruling 3 |
| Q10 | Which text is normative? | The proposals folder holds about 120 "Accepted" paragraphs outside the protocol reference. | answered: the specification in `spec/`, and nothing else (ruling 17; decision DA1 of the documents audit). Eighteen of the nineteen behaviors that only the older text stated are converted; the last is tracked by A32 |
| Q11 | How does the language say where an instance runs? | — | done: ruling 7 |
| Q12 | What may an instantiator supply, and how does a template say so? | — | done: ruling 8 |

### Disagreements

| ID | Disagreement | Outcome | Status |
| --- | --- | --- | --- |
| D1 | Risk was tested in the wrong order: the native-Agent and account assumptions were scheduled last. | The native probes run in parallel with the language work. | done |
| D2 | "The language is the primary product" was asserted, not argued. | Ruled, and made testable by the two independent builds. | done |
| D3 | Experiment 002 was v0, not a spike. | Its cases are mapped onto promises and vectors, then it is removed (A23). | open: decided, and the mapping is A23 |
| D4 | The first proof was distributed before anything ran on one machine. | Placement in the language, then separated components under a simulated network. | done |
| D5 | The authorization machinery exceeds what is enforceable natively. | To be ruled under A20, informed by the bypass probe (A7). | open |
| D6 | The graph language is verbose and weak at once. | To be decided by the graph-need comparison (A19). | open |
| D7 | A vocabulary pass before code would churn again after code. | The pass comes last and may merge concepts. | superseded: by ruling 16, the vocabulary was decided at once and applied with the rework |
| D8 | The review process generated its own queue. | A new ruling carries a vector that can fail (A24). | open |

### Standing probes and comparisons

These need no engine. Code is Rust and may be thrown away afterwards.

| ID | Question | Method | Decision it drives | Status |
| --- | --- | --- | --- | --- |
| E1 | Can native Agents be launched with private configuration and a Conduit endpoint? | Start Claude Code and Codex with separate configuration homes and a per-Agent token. | If not, no real Agent exists until the owned loop. | open |
| E2 | Do four accounts coexist? | Two Claude and two Codex subscriptions under one macOS user for 48 hours. | If they collide, Agents need separate OS users or containers. | open |
| E3 | Does mediation work? | A small proxy holding one provider login, serving two Agents with different allowlists, including a lost response. | Whether the operation-identity convention survives clients that send none. | open |
| E4 | What can a native Agent bypass? | Instruct an Agent to reach data it is denied; record every path that works. | The honest security claim. | open |
| E5 | Does messaging progress without wake? | History tools plus a next-turn notice through Agent hooks. | Whether automatic wake moves into v0. | open |
| E8 | Are authored graphs needed? | Build three walkthroughs twice: configuration only, and as authored graphs. | Whether authored graphs stay in v0. | open |
| E10 | Does Keychain work unattended? | Access from a daemon, a rebuilt unsigned binary and a locked session. | Credential custody design. | open |
| E11 | May the owned loop use subscriptions? | Desk check of current terms. | Whether the owned loop is viable as planned. | open |

Earlier experiment ideas E0, E6, E7, E9 and E12 are superseded by A42, A14, A40, A43 and A34.

## Scope guard

The acceptance scenario is the first thing that must work. It does not define what Conduit is.

- Passing the scenario removes nothing from the accepted model. A capability is not dropped or
  narrowed because the scenario does not exercise it.
- "Never exercised by a scenario" is evidence about sequencing, not about scope. Moving anything
  to later scope, or dropping it, takes an explicit owner ruling (A18).
- The object model is checked against scenarios unlike the acceptance scenario before any
  pruning (S3 to S9), so that a design fitted to one scenario is caught early.
- Simplifications made to reach the scenario sooner are recorded here as shortcuts with the
  general behavior they stand in for, so they can be replaced rather than inherited.

| Shortcut taken for the first slice | General behavior it stands in for | Status |
| --- | --- | --- |
| — | — | — |

## Multi-system options

> **Historical.** Recorded at the audit, before rulings 13 to 37. Not maintained.

Kept as the record of ruling 5. "Multi-system support" has two parts, and the plans at the
time of the audit covered only the first.

**Runtime.** Can parts of Conduit run in separate processes or machines and survive each other's
failure? The accepted first proof answers by building one management service and two execution
services as separate processes on one machine, each with its own store.

**Language.** Can an author say that this Resource runs on one system and that Agent on another?
At the time of the audit, no: Pool ownership was explicitly not placement and nothing else selected
a host or execution service. The [placement record](../proposals/placement/README.md) now answers it.

The audit's original objection (D4) was to the build order, not the goal. Starting with three
processes means the first code must solve lost replies between services, stale views and
unreachable policy before any single feature works, and a failing test cannot say whether a rule
or the plumbing is wrong. The opposite risk is equally real: code written as one process tends to
assume shared memory and then cannot be split.

| Option | What is built first | What it proves early | What it cannot prove | Main risk |
| --- | --- | --- | --- | --- |
| (a) Keep the accepted layout | Management and two executors as real processes | Real process death, separate stores, real reconnection | Nothing about the language unless Q11 is settled | Slow first result; rule defects hidden behind plumbing defects |
| (b) Separated components, simulated network | The same components, each with its own store, exchanging only serialized messages, run in one process under a simulated network that can drop, delay, reorder and partition, and can crash a component | Every cross-service rule: outage admission, lost receipts, stale coordinator, retained results, deterministically and repeatably | OS-level behavior: orphaned child processes, file locks, helper supervision, a real second machine | A component quietly sharing state. Mitigated by allowing only the message boundary and by running the same code as real processes as the next step |
| (c) Settle placement in the language first | A written answer to Q11, checked against the two-system breadth scenario | That the language can express a multi-system deployment | Any runtime behavior | None beyond the time it takes; combines with (a) or (b) |

Under every option the accepted evidence rule stands: no remote support is claimed before a test
across two machines.

Owner ruling, 2026-11-05: (c), then (b). After that, the same code runs as real processes, then
across two machines. Option (c) is the [placement record](../proposals/placement/README.md).

## Supporting detail

### Research and design gaps

> **Historical.** Recorded at the audit, before rulings 13 to 37. Not maintained.

- The Rust runtime research says its runtime, worker and persistence choices remain
  recommendations until Experiment 002 measures them. SQLite, helper processes and MCP as the
  plugin invocation base are accepted without that experiment having run.
- The research asks for a stated, tested consistency contract for revocation. The protocol says
  managed requests use new rules immediately and gives none.
- The research takeaway from policy engines is to keep decisions out of service code. The
  accepted text has the governed plugin evaluate its own restriction.
- MCP 2026-07-28 removes stream resumption, replaces server-initiated elicitation with multiple
  round trips, and deprecates dynamic client registration. None of these is reflected in the
  interaction-wait, deferred-work or shared-login designs.
- stdio servers take credentials from the environment. The planned stdio bridge for native
  clients makes that the relevant case, and the research does not analyse it.
- The only executed evidence before the audit was a set of Pi helper probes kept outside the
  repository, 13 offline render checks whose fixtures were not retained, and the Experiment 003
  library probe. None tests Conduit behavior. Experiment 004 has since run a model checker over
  one promise group.

### Known inconsistencies

> **Historical.** Recorded at the audit, before rulings 13 to 37. The items are as first
> written. Each was checked against the tree on 2026-11-06, and the sentence in italics after
> it says what was found. Action A21 tracks what is left.

- `proposals/authoring-syntax/AUTHORING.md` lines 433–436 declare a singular runtime with output
  references, against the accepted named runtime map and explicit start operation.
  *Stands as written, now near line 440. The file is marked as not binding (decision DA6 of the
  documents audit). The shape of a runtime in force is TPL-006 and TPL-007.*
- The two package entry-document examples (authoring proposal and package walkthrough) use
  different fields, and neither carries a format version.
  *Stands as written. Both files are marked as not binding (DA6). The manifest's format in
  force is in `spec/promises/manifest.yaml`.*
- Control declarations differ between the package walkthrough and `instances.yaml`.
  *Overtaken. `instances.yaml` was a file of Experiment 001, which is removed (DA4), and the
  walkthrough is marked as not binding. Controls in force are CTR-006 and TPL-010.*
- Templates in Experiment 001 hard-code predicted instance names, so they are per-deployment
  documents rather than reusable templates.
  *Overtaken. Experiment 001 is removed (DA4).*
- Operations that create instances, open Conduits and start runtimes are presented as ordinary
  plugin exports, with no plugin-to-host interface described.
  *Overtaken in the specification. Registering and opening are Operation contracts of the core
  package, which the engine itself implements (OPS-019, MAN-012). A runtime's start is an
  operation that returns a handle (TPL-006, TPL-007). The older documents that present them as
  plugin exports are marked as not binding.*
- The conceptual review relies on rulings R1–R7, which are defined nowhere in the current tree.
  *Stands as written. The file is in the archive, which is not edited.*
- `research/ESTABLISHMENT.md` has `<session-workspace>`, an artifact of a Session-to-Agent
  replacement whose intended text is not known.
  *Stands as written, at line 389. The content of a research packet is not edited.*
- Rulings R1, R5 and R6 are cited in the lifecycle topic and defined nowhere.
  *Stands as written, at lines 222 to 228 of `protocol/LIFECYCLE.md`. The topic is superseded
  and is not edited.*
- Bare labels such as `[DESIGN]`, `[REVIEW]` and "specification" point at files that have since
  moved; the link targets resolve but the labels mislead.
  *Partly overtaken. The older files that carry one now say under their title that they do
  not bind, apart from the files of the archive, where only the index says so. The note on each
  research packet says what "SPEC.md" and "DESIGN.md" meant. The label in Experiment 002 that
  called the overview the specification was corrected on 2026-11-06. The other labels stand.*
- About 30 links point into Experiments 001 to 003 and will break when those are removed; the
  vocabulary map holds 17 of them.
  *Partly overtaken. The links to Experiment 001 now point at commit `de6566a` on `main`, the
  last there that held it (DA4). About twenty links still point into Experiments 002 and 003, and break when
  those are removed (A23, A43).*
- The local branch `implementation/rust-runtime-plugins` contains no code and its name could
  mislead.
  *Still so: the branch exists locally and on the remote. Left for the owner.*

Fixed on 2026-11-05: stale status and sequence statements in the entry documents; the overview's
date; the corrupted citation URL in `research/CONFIGURATION.md`; "a Agent-local"; the two shapes
used for a Pool entry; the placement record's proposal wording; and the note that the migration
checker cannot fail.

### Deferrable scope

> **Historical.** Recorded at the audit, before rulings 13 to 37. Not maintained. Several of
> the concepts it names were later replaced or removed by rulings.

Accepted for v0 but possibly not needed by the first slice, to be confirmed by the scenarios (A14): all 19 enforced
formats; re-exports and replaceable dependencies; multiple named runtimes and the runtime-target
source; the invocation-metadata source; the object and list binding forms; graph-backed
operations and public-output prerequisites; requirement composition and recursion; live
ownership-scoped bindings and bounded issuance; partial control confirmation; registration
batches as graphs; human-interaction wait retention; snapshot-to-feed handoff; edit-mode capture
and symlink rules; conditional-default rejection; persistence, custody and observation as plugin
contracts; type contracts for Pool, Conduit and Interface.

## Reference repositories

> **Historical.** Recorded at the audit, before rulings 13 to 37. Not maintained.

No clone held uncommitted work, stashes or local-only branches, and nothing in this repository
reads a sibling directory. All citations of these repositories are commit-pinned GitHub URLs.

Kept beside this repository, in two sibling directories, through the first
implementation slice. Consult only; never edit them:

| Repository | Pin | Why |
| --- | --- | --- |
| A private repository (name removed under ruling 38) | `04025a6` | The only Rust code near the planned stack: MCP server, Claude Code hooks, room sequencing. Proprietary, so consult only. Its links will likely fail for readers outside that organization. |
| `earendil-works/pi` | `0441483` | Named frontend reference. This pin is a detached shallow HEAD; confirm it resolves upstream. A sixth cited commit, `642e30b`, was never in the clone. |

Removed on 2026-11-05; re-clone at the pin if needed:

| Repository | Pin |
| --- | --- |
| `xerj-org/xerj` | `63b6aef` |
| `tilt-dev/tilt` | `f3acf27` |
| A second private repository of the same organization (name removed under ruling 38) | `b49a754` |
| `aaalexliu/pstack-pi` | `c87e65f` |
| `earendil-works/pi-chat` | `9adbd29` |
| `jayzeng/pi-memory` | `1e276a7` |
| `josephkern/pi-memory` | `312627f` |
| `nicobailon/pi-messenger` | `09937ed` |
| `desek/agent-observability` | `bab5337` (cited nowhere in this repository) |
