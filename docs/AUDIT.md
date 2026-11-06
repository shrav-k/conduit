# Plan audit and refinement tracker

Opened: 2026-11-05. Baseline: commit `37de771` (`research/vocabulary-precedents`).

This is the working tracker for the direction the owner set on 2026-11-05 after an external audit
of the plans. It records where things stand, the rulings, the priorities, the action items and
the experiment program. It is the one tracker: the earlier review queue is
superseded and removed. It is not a specification. The specification is [spec/](../spec/README.md), which
alone binds, and a ruling changes nothing until it is written there.

The external audit's report is not kept as a file. What survives of it is the
[findings](archive/plan-audit-history.md#findings) and the other sections marked **Historical**,
which ruling 39 moved to the [history of this tracker](archive/plan-audit-history.md). "The audit" in ruling 1
means it. This document is called the plan audit after it. Later audits have other names: the
consolidation audit, the re-audit of the specification and the audit of the documents outside
`spec/`, each with a record under [proposals/](README.md#records-of-rulings).

## Identifiers

A bare identifier in this document is one of the tracker's own:

| Prefix | Meaning |
| --- | --- |
| **A** | Action item |
| **Q**, **D** | Question and disagreement, in the [findings](archive/plan-audit-history.md#findings) |
| **E** | Probe or comparison |
| **S** | Scenario of the [experiment program](#experiment-program) |
| **P** | Priority, P0 to P3 |
| **LD** | [Layer decision](#layer-decisions) |
| **G** | Condition of the [readiness gate](implementation/INDEPENDENT_BUILDS.md#readiness-gate), G1 to G11 |
| **L** | Language defect, in the [register](LANGUAGE_DEFECTS.md) |

The records of the rulings number their own decisions, and some reuse a letter. An identifier
from a record is therefore written with the record's name, as in "re-audit decision G1".

| Prefix | Meaning | Record |
| --- | --- | --- |
| **PL**, **IN** | Placement and instantiation decisions, written with no hyphen: PL1 to PL8 and IN1 to IN10. With a hyphen, **PL-1** to **PL-10** are something else: the earlier wording of the placement rules, which are now the promises `PLC-001` onward | [placement](proposals/placement/README.md), [instantiation](proposals/instantiation/README.md) |
| **NS** | Decisions on the form of the specification, NS1 to NS4 | [normative-surface](proposals/normative-surface/README.md) |
| **V** | Vocabulary decisions, V1 to V8. The removed [vocabulary map](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md) also numbered its topics V01 to V37 | [vocabulary](proposals/vocabulary/README.md) |
| **RD**, **RR** | Decisions and rules on relationships and access | [relationships](proposals/relationships/README.md) |
| **OD**, **OR** | Decisions and rules on operations, outcomes and revisions | [operations](proposals/operations/README.md) |
| **DD**, **DR** | Decisions and rules of the declarations batch | [declarations](proposals/declarations/README.md) |
| **GD**, **SD**, **GR**, **SR** | Decisions and rules on graphs and on the schema profile | [graphs-and-schemas](proposals/graphs-and-schemas/README.md) |
| **PD**, **PR** | Decisions and rules on packages, first start and conformance | [packages-and-start](proposals/packages-and-start/README.md) |
| **NA** to **NF** | Naming decisions | [naming](proposals/naming/README.md) |
| **CS**, **CC** | Consolidation findings, on the shape of documents and on concepts | [consolidation](proposals/consolidation/README.md) |
| **OL**, **DO**, **RC**, **NI** | Decisions on the four remaining shared conventions | [conventions](proposals/conventions/README.md) |
| **D**, **I**, **X** | Decisions on the shape of documents, on the interface, and on the name the owner controls | [schemas-and-interface](proposals/schemas-and-interface/README.md) |
| **DL** | An item of the [implementation-defined list](../spec/implementation-defined.yaml), DL1 onward | The specification |
| **U**, **Q**, **A**, **G**, **W** | Re-audit decisions, by group. **F** is a follow-up decision | [reaudit](proposals/reaudit/README.md) |
| **C**, **N**, **P**, **D**, **X** | Choices, names, open points, disagreements and accepted changes of the core package and schema batch | [core-and-schemas](proposals/core-and-schemas/README.md) |
| **DA** | Decisions of the audit of the documents outside `spec/` | [docs-audit](proposals/docs-audit/README.md) |

A promise has an identifier of letters, a hyphen and three digits, such as `CON-001`. The
letters name its file; [spec/README.md](../spec/README.md#contents) lists the files. None of
these sets reuses the earlier R, I, C or M ruling sets of the archived reviews.

## How to use this document

- Statuses are `open`, `in progress`, `ruled`, `done`, `answered`, `dropped` or `superseded`,
  optionally followed by a colon and a note. `ruled` means decided by the owner and written as
  promises, with vectors still to come. `answered` is used for a question.
- A ruling is recorded once in the [ruling log](#ruling-log), then applied to the documents it
  affects. The rows of the log are not rewritten; a [note under it](#later-amendments) lists
  which rulings later ones amended.
- Promise ranges and counts in status notes are as first written. Later passes folded some
  promises into others and added more; the current files and counts are in
  [spec/README.md](../spec/README.md#contents).
- Experiment fixtures and dated results live under [experiments](../experiments/README.md).
- A finding against `spec/` goes in the [re-audit record](proposals/reaudit/README.md). The
  [defect register](LANGUAGE_DEFECTS.md) holds the first sweep, which read the protocol topics
  that ruling 39 removed.
- The sections that recorded the audit as it was first made, and the action items that are done
  or superseded, are in the [history of this tracker](archive/plan-audit-history.md).
- When every item is closed, move this file to the [archive](archive/README.md).

## Where things stand

| Item | State on 2026-11-06 |
| --- | --- |
| Specification | A draft, in [spec/](../spec/README.md): 434 numbered promises in three layers (the language, the shared conventions and the implementation interface), with the definitions, the schema profile, the implementation-defined list, the core package as documents, the JSON Schemas of every document and valid and invalid examples. Its [contents table](../spec/README.md#contents) gives the files and the counts. The owner accepted it as it stands at ruling 39. No promise has a conformance vector. |
| How it was checked | Four rounds of cold readers ([re-audit](proposals/reaudit/README.md)): about 170 problems in round 1, counted by class, then 125, 44 and 38 findings in the three reports on `spec/`. No round was clean; every finding is resolved and logged. The interface was checked twice against the published Model Context Protocol ([first](proposals/reaudit/round-2/mcp-citations.md), [second](proposals/reaudit/round-4/mcp-recheck.md)). |
| Secrets | Not in the specification. Ruling 38 gave the direction and ruling 39 asked for research first: [research packet](research/SECRETS.md), [proposal](proposals/secrets/README.md), not ruled. |
| Documents outside the specification | Nothing outside `spec/` binds. What the specification superseded was removed from the tree by ruling 39; the [archive's index](archive/README.md#removed-from-the-tree) lists it, with links to its last version. The historical sections of this tracker are in [its history](archive/plan-audit-history.md). |
| Language defects | 167 recorded in the [first sweep](LANGUAGE_DEFECTS.md), of the protocol topics. Triage by layer: 48 language, 39 shared convention, 30 type contract, 50 runtime profile. Of the 87 in the first two layers, three are closed and 84 are ruled. Of the 50 in the runtime profile, 49 are answered by an item of the implementation-defined list and one is ruled. The 30 type-contract defects wait for their contracts. None has a vector. |
| Readiness gate | G1 to G11, with G7 as ruling 38 restated it. None is met; G1 and G3 are partly met. The state of each, assessed under ruling 39 and confirmed by the owner, is in the [build process](implementation/INDEPENDENT_BUILDS.md). |
| Vocabulary | Ruled ([record](proposals/vocabulary/README.md)) and applied in `spec/` and the overview. The record of old and new spellings (A29) is not written. |
| Build process and brief | Drafted: [independent builds](implementation/INDEPENDENT_BUILDS.md), [engine build brief](implementation/ENGINE_BUILD_BRIEF.md). Not issued. |
| Code | [Experiment 004](../experiments/004-stateright-evaluation/README.md), a Rust model-checking evaluation, and [Experiment 005](../experiments/005-macos-keychain/README.md), a probe of the macOS keychain for the first secret store package, executed and re-run. No engine, no reference model and no package yet. |

### What happens next

1. Pull request 7 applies rulings 39 and 40. A review of the whole design against ruling 40
   ([principles review](proposals/principles-review/README.md)) brings the owner the changes it
   needs; the owner rules on them and on the secret type.
2. The specification is rewritten around provider contracts (A62): the runtime model, the location
   model and the setup graphs are cut down as ruling 40 says, and the secret type is written in.
3. A fifth round of readers reads the rewritten specification, with scenarios across several
   machines, and every confirmed finding is fixed. Then pull request 7 is merged.
4. The owner starts a cold read by Codex. What that reader needs is in the next section,
   [for the next reader](#for-the-next-reader).
5. The conformance vectors (A39), the reference model (A38) and the first providers (A63)
   follow. Convergence is judged on them (ruling 38, point 4). In the
   [order of the experiment program](#experiment-program) the work is at step 2.

### For the next reader

Written for the reader that the owner starts after ruling 39, and for anyone who audits this
repository cold.

**What to read.** `README.md`, then [spec/README.md](../spec/README.md) and
[spec/definitions.md](../spec/definitions.md), then the promises. Only `spec/` binds. The
[overview](OVERVIEW.md) is a summary and does not bind.

**What has been read.** Five rounds of cold readers read `spec/`
([re-audit](proposals/reaudit/README.md) has the counts by round). Every finding was resolved
and logged.

**How the readiness gate now reads (ruling 38, point 4).** The gate of two consecutive clean
rounds of cold reading is withdrawn: four rounds showed that reading alone does not reach it.
The scenario reader found fifteen gaps in each of rounds 3 and 4, where features that were
specified apart meet. Convergence is judged instead on two things: the latest round of readers
reports no unsafe finding, and the reference model and the conformance vectors, once they
exist, agree with the promises. A reader is asked to report every finding, and to mark as
unsafe any that could grant access, lose or repeat an effect, or report a wrong result.

**Known to be open.** Do not report these again unless there is something new:

- Secrets. The specification says nothing of them yet. The direction is ruled (rulings 38 and
  39): a secret is referred to and never written as a literal; only a Resource reads one, and
  only the root Interface may open a Conduit that reads one; the engine may set a credential in
  a process's environment at start. The rest is [proposed](proposals/secrets/README.md), not
  ruled.
- The shape of a diagnostic beyond its `message`, and the full shape of an inspection result.
  They are settled with the conformance vectors.
- No promise has a conformance vector. No reference model and no engine exist.
- Later work is not part of v0 (ruling 38, point 6): no operation arranges, through the
  engine, work that outlasts its invocation.
- A composite Conduit at setup: the decision to open is whole, and each component then opens
  with its own outcome (ruling 39, point 5).

**To look at in particular (ruling 38, point 11).** The interface follows the Model Context
Protocol on errors: a call to a tool that the engine does not present is answered with the
protocol's error, code -32602, and a result that carries none of its operation's declared
outputs has `isError` true. Two things follow that the owner wants examined. The protocol lets
a client keep protocol errors from its model, so an Agent's model may never see that a request
was refused. And an open proposal to the protocol, SEP-2145, would turn an unknown tool into
an error result, which would reverse the first.

**Who decided what.** The next section. In short: rulings 1 to 39 are the owner's, and ruling
39 accepted every logged decision of the coordinator and choice of an editor made before it. A
reader who disagrees with any of them should say so.

**Where findings go.** One report per reader, in a new directory
`docs/proposals/reaudit/round-6/`, in the form of the earlier reports: an ID for each finding,
a severity, the promise IDs, short exact quotes, the situation, and the least change that
fixes it. A reader changes nothing under `spec/`. The owner rules, or the coordinator decides
under ruling 37, and the decision is logged beside the report.

### Who decides what

The owner makes the rulings, logged [below](#ruling-log). In ruling 37 the owner also said that
the findings of the cold readers are to be resolved without waiting for a ruling on each, and
in ruling 38 confirmed the coordinator's reading of that: a finding goes back to the owner only
where settling it would change what Conduit is for. So the specification has changed in three
ways, and the records keep them apart:

- **By ruling.** The rows of the ruling log, each with its record under
  [proposals/](README.md#records-of-rulings).
- **By the coordinator's decision on a finding**, logged with its reason where the finding is
  recorded: under [proposals/reaudit/](proposals/reaudit/README.md) and
  [proposals/docs-audit/](proposals/docs-audit/README.md).
- **By an editor's choice**, where a decision left the wording or a detail open, logged under
  "Choices" in the editor's log.

Ruling 39 accepted every decision of the coordinator and choice of an editor logged before it,
including the choices that the passes applying rulings made before ruling 37. A later one has
the standing of a decision the owner can strike, until a ruling accepts it.

| Word | Who it is |
| --- | --- |
| The coordinator | The Claude session that the owner asked to write the language, and that keeps this tracker. It decides a finding where ruling 37 lets it, starts the readers and editors, checks their work and commits it. It is not the owner. |
| A cold reader | A separate session that the coordinator starts with no context. It is given only `spec/`, or the whole repository, and writes a report, which is kept. It changes nothing. The counts of findings for a round are the counts of these reports. In round 2 the editors of the closing pass and of the last pass also started three readers of their own, whose findings are rows of [the closing log](proposals/reaudit/round-2/log-closing.md) and of [the log of the last pass](proposals/reaudit/round-2/log-tail.md), outside the 125. |
| An editor | A separate session that the coordinator starts to apply decisions to named files. "Editor P" and "Editor S" in round 2 were the editors of the promises and of the core package and schemas. An editor logs each choice it makes, and lists what needs a new decision without deciding it. |

The work is committed and pushed at each milestone, to a branch with its own pull request, as
the owner confirmed in ruling 38.

## Waiting for the owner

One list of everything that only the owner can settle.

| What | Where it is recorded |
| --- | --- |
| The names that a vocabulary audit of the rewritten specification proposes, among them the name of the shared contract of secret stores (ruling 41). | [Ruling 41](proposals/ruling-41/README.md); the audit's record, when it is written |

## Purpose

Ruled 2026-11-05 (ruling 15).

This section is kept in the words of the ruling, except one sentence that ruling 39 amended.
Since rulings 27 to 29 there is no separate grant: a grant is the access an open Conduit carries, to issue one is to open a Conduit, and to
revoke one is to close it. "Provision" is setting up, and a plugin is an implementation. Read
the section with those words.

**The relationship is the product.** Infrastructure languages such as Kubernetes, Docker Compose
and Terraform declare workloads, which do what their specification says. An agent does what it
decides. An agent-first language therefore declares what surrounds the agent: what it may reach,
under which limits, and who answers for what it did. That is why the family that carries grants
gives the project its name.

- **First user:** a single developer running agents from several vendors on their own machine.
- **Teams:** not in v0, but the language must be able to support them later, most likely through
  Interfaces. No rule may assume there is exactly one owner.
- **Enforcement:** a future, optional part of the language. By default a restriction applies to
  what passes through Conduit. An author can opt in to a selected isolation framework through the
  language and plugins, as with Docker, to have it enforced.

What is distinctly Conduit's, and must survive the rework:

- **Establish once, share under restriction.** A login, checkout or memory is owned by a Pool and
  used by agents from any vendor, each under its own limits, with the credential never handed
  over.
- **The Conduit is an object.** Each agent's access to each resource can be inspected, narrowed
  and revoked independently of both ends. (Amended by ruling 39, point 7: the ruling said "the
  grant".)
- **Agents can operate the system.** An agent can provision a resource, start a peer and delegate
  a narrower slice of access, through the same contract a human uses.
- **Nothing implicit spends tokens or repeats an effect.** Actions are explicit, an unknown
  outcome stays unknown, and there is no background reconciler or automatic wake.
- **Private and shared stay separate.** An agent's subscription and context are its own; anything
  shared goes through a Resource.

Not differentiating, and so free to simplify: the graph engine, the package system, the schema
profile, persistence and export. For graphs and schemas the owner's standing instruction
(ruling 23) is to do nothing revolutionary and to rely heavily on existing frameworks and
paradigms; the same applies to packages (ruling 24).

### Preservation tests

Accepted 2026-11-05 (ruling 17). Each rework decision is checked against them.

Since rulings 27 to 29 a grant is the access an open Conduit carries, and "provision" is setting
up; read the tests with those words. The tests themselves are not reworded.

| # | Test |
| --- | --- |
| 1 | An agent is a participant, never a workload: the language does not specify its reasoning. |
| 2 | Every access is a relationship with a grant that can be inspected and revoked. |
| 3 | A shared thing is owned once and never copied to an agent. |
| 4 | Nothing assumes a particular vendor's agent. |
| 5 | Every management action is available to an agent under a grant, and delegation only narrows. |
| 6 | Nothing starts model work or repeats an effect implicitly. |
| 7 | Everything an agent does through Conduit is attributable. |
| 8 | No rule assumes exactly one owner: a human's access is an Interface holding grants. |
| 9 | A restriction always reports whether it is enforced or only mediated, and enforcement is opted into through the language. |

Two additions to the shared conventions were accepted with them. There is a seventh convention,
for test 6: nothing starts model work or repeats an effect implicitly. And the access convention
states, for test 5, that an agent may issue grants to others only within a bound it was given.

## Current direction

- **Conduit innovates only on connection and access** (ruling 40). It owns its documents,
  connection (Conduits), access, accountability (what must be recorded) and the shape of its
  contracts. Everything else is a provider behind a contract, swappable by the user wherever
  possible: secret stores, the runtime that runs implementations, the machines and their
  placement, the source of packages, and later identity, isolation and the storage of records.
  Where an existing tool or standard does the job, Conduit adopts it.
- **It works across machines.** Coordination of agents and resources on several machines is part
  of what Conduit is for. Conduit owns what placement, a lost machine and a failed request mean
  for access; providers own the mechanics. One control plane decides every request in the first
  version; local enforcement with a bound on how long a revocation takes is likely needed in v1.
- **The language is the core; the runtime fulfills its promise.**
- **Conduit is a configuration language, not a behavioral specification.** The language fully
  determines what a set of documents means. Behavior beyond a few shared conventions lives in
  type contracts and a runtime profile ([layered claim](#layered-claim)).
- **Current work makes the language concrete and provable**: the specification, written as
  numbered promises for the language, the shared conventions and the implementation interface;
  schemas; an executable reference model in Rust; public conformance vectors; and model
  checking.
- **Proof of sufficiency** is two independent builds, by fresh sessions of two coding agents
  from different vendors, Codex and Claude. Each sees only the specification and the vectors,
  and never the other's work. They cover the language layer, the shared conventions and the
  implementation interface; one reference engine is built separately. They do not start until
  the [readiness gate](implementation/INDEPENDENT_BUILDS.md#readiness-gate) passes; the owner
  wants high confidence first.
- **Language defects are closed before the builds.** The first sweep found 167 against a stronger
  claim; they were re-triaged by layer after the layer decisions
  ([defect closure plan](archive/plan-audit-history.md#defect-closure-plan)).
- **Any code is Rust**, under `experiments/` until the reference model has a home.
- **The vocabulary was decided at once and applied with the rework** (ruling 16), so that names
  and behavior were fixed together and the text was rewritten once. The pass merged concepts as
  well as renaming them ([vocabulary record](proposals/vocabulary/README.md)).
- **The relationship is the product** ([purpose](#purpose)). Rework decisions are checked against
  the preservation tests.
- **The acceptance scenario is a first proof, not the scope** ([scope guard](archive/plan-audit-history.md#scope-guard)).

## Ruling log

All rulings are the owner's, in this order: rulings 1 to 37 on 2026-11-05, and rulings 38 to
41 on 2026-11-06. Each row is the coordinator's summary of the owner's messages, with the
owner's own words quoted where they are marked as quoted; the owner confirmed that labelling in
ruling 39, point 15. The record linked from a row holds the owner's words in full where the
ruling was given point by point.

| # | Ruling |
| --- | --- |
| 1 | The audit is the basis for action items and priorities. Unneeded reference clones are removed. |
| 2 | Vocabulary stays provisional until the language is shown to be expressive enough. |
| 3 | Exploratory code may be written and run whenever useful, in Rust only. |
| 4 | Claude and Codex collaborating on one project on one machine is the starting acceptance scenario; success there must not narrow the design below its broad scope. |
| 5 | Multi-system support: settle placement in the language first, then build separated components under a simulated network. |
| 6 | The language is the core and the runtime fulfills its promise; whatever makes the language more concise, concrete and useful should be done. |
| 7 | Placement decisions PL1 and PL3 to PL8 accepted, including that a system cut off from management starts no new managed work in v0, that types declare their location needs, and that a declare-lost operation is needed. |
| 8 | Instantiation: a closed, declared input list; structure is never an input; one shape everywhere; explicit inputs with no shorthand; each input names its destinations; the instance stores only the result; name, owner and placement are standard arguments. This also settles PL2. |
| 9 | The nine-scenario experiment program, its order and its parallel work are accepted. Old experiments are rewritten or removed as replacements land and need not be preserved. |
| 10 | Sufficiency must be provable to a user of the protocol. The executable reference model in Rust is accepted. A full rebuild is wanted if proof needs it: Codex and Claude each build the whole engine and do not check each other's work. |
| 11 | The independence rules for the builds are accepted, and the session that wrote the language does not build an engine. |
| 12 | The repository is to be prepared with an explicit build brief, with builders on separate branches. All identifiable language defects are to be closed first, and the builds wait until success is very likely. |
| 13 | The layered claim is adopted: the language determines what declarations mean; behavior beyond shared conventions lives in type contracts and a runtime profile. Layer decisions come next, then the defects are re-triaged. |
| 14 | Layer decisions LD1 to LD5 accepted as recommended. Setup graphs follow the Docker Compose and Terraform model: the language defines the dependency structure and the engine decides how to walk it. The independent builds cover the language layer, the shared conventions and the plugin interface, with one reference engine. Before fixing proceeds, the language's purpose and its agent-first differentiation are to be discussed and then preserved through the rework. |
| 15 | Purpose: the relationship is the product. The first user is a single developer; teams come later and the language must be able to support them, likely through Interfaces. Enforcement is a future, optional part of the language, opted into through the language and plugins that select an isolation framework. |
| 16 | The vocabulary is decided now, folding in the reviewer's simplification pass on pull request 5, and applied alongside the rework to avoid churn. This replaces the earlier ruling that vocabulary comes last. Then the defects are reassessed and fixed. |
| 17 | Accepted: the nine preservation tests; a seventh shared convention and explicit agent delegation in the access convention; the form of the specification (NS1 to NS4); and vocabulary decisions V1 and V3 to V8. V3 also settles that a Conduit is pairwise: one consumer, one target, opened and closed. V2, how a document marks a template, is under discussion: two kinds per family is rejected as cumbersome, since a template and its instance are two variants of one kind and the template flows into the instance. |
| 18 | Vocabulary decision V2: one kind per family. A document that declares inputs and has no `from` is a template; a document with `from` is an instance of it. Every instance therefore comes from a template, and a reference is a kind and a name. |
| 19 | The first batch of fixes, relationships and access, is accepted: decisions RD1 to RD8 and rules RR1 to RR16. A reference may give the name alone where its field accepts exactly one kind. The owner also stated that issuing a Conduit to a live instance must be a core part of the language. |
| 20 | Accepted: Conduits to live instances (CON-012 to CON-014), with consumer and target as standard arguments of the Conduit family; the `layer` field on a promise; and the second batch of fixes, operations, outcomes and revisions (decisions OD1 to OD5 and rules OR1 to OR9). |
| 21 | The third batch of fixes, declarations, is accepted: decisions DD1 to DD5 and rules DR1 to DR13. On DD2 the owner asked what a document is concretely before moving on. |
| 22 | After a concrete explanation of what a document and a submission are, decision DD2 stands as accepted: a submission is a set of documents, checked together, with order derived from their references, and submitting only registers. |
| 23 | The fourth batch of fixes, graphs and the schema profile, is accepted: decisions GD1 and SD1 to SD3 and rules GR1 to GR4 and SR1 to SR5. Every type must state its location needs, including none. Standing instruction: for graphs and schemas, do nothing revolutionary and rely heavily on existing frameworks and paradigms, since they are not what differentiates Conduit. |
| 24 | The fifth batch of fixes, packages, first start and conformance, is accepted except decision PD1: decisions PD2 to PD5 and rules PR1 to PR8. Applying the existing-paradigms instruction to packages is accepted with it. PD1, how a package's exports are named and referred to, needs more research and discussion of naming patterns. |
| 25 | Naming direction: a package's identity is address-like, as in Go, because people will have entirely custom package setups; references to a package's exports are fully qualified, with no aliases; versions are pinned once per project, as Go and Cargo do; contracts having an identity independent of the package release is acceptable; the reference syntax is to be discussed. The owner is concerned that so large an inconsistency was found so late, and asked for an audit of the language for any other consolidation that is needed. |
| 26 | The owner raised that connections must work across package versions: the two ends of a Conduit may come from different versions, instances run different versions, and compatibility should be as wide as requirements allow. Whether packages must be recorded directly in templates and instances is to be discussed. |
| 27 | Accepted: the naming decisions NA to NF and the answers on versions across connections. A reference to a package's export is its address, `#` and the export's name; one manifest per project holds versions and digests; a registered template records its packages; any number of versions can be installed; a contract's version is part of its name, changes only compatibly, and a type can offer several; a breaking package version takes a new address. Consolidation: grants are merged into Conduits, and Pool entries are folded into the mechanism Conduits use. The remaining consolidation findings are to be discussed. |
| 28 | Consolidation rulings: every recommendation of the consolidation audit is accepted except two that are to be discussed. The right to open Conduits to a target is itself an open Conduit to the `access` Resource, so every permission is a Conduit. Open: whether Pool ownership should itself be a Conduit; and the common lifecycle, including whether a closed Conduit can be reopened at all or is simply retired. |
| 29 | Accepted: Pool ownership stays one recorded fact, fixed at creation, that permits nothing; every power over a Pool or its Resources is an open Conduit; a right to open stated for a Pool covers the Resources it owns; a Pool's entries are created with the Pool and "provision" becomes setting up that Resource. Accepted: one forward-only lifecycle for all five families, whose states the owner named registered, ready and retired; a closed Conduit is retired and never opened again; there is no suspended state; setup never replaces a retired declared Conduit or entry Resource, and a replacement is a new instance made only on an explicit request; Conduits keep the verbs open and close; running, paused and idle belong to type contracts. |
| 30 | Accepted: all fourteen decisions on the four remaining shared conventions ([record](proposals/conventions/README.md)). An instance is retired from the moment the request is dispatched, and is unregistered only once its shutdown is confirmed; one authority covers an object's whole life; first-start objects are permanent; a difference between declared and observed state is reported and never repaired without a request; a closed list of events is always recorded, and only `root` may purge; model work starts only through an operation whose contract marks it, never from setup. The owner also ruled the order of what remains: one more batch of rulings, then the promises are consolidated and the whole specification is audited again. |
| 31 | Accepted: the last batch ([record](proposals/schemas-and-interface/README.md)), every recommendation, with one amendment. Documents: one `implementation` map; camelCase keys and `apiVersion`; structured value references from five sources; a runtime is a pair of start and stop operations; an optional `retire` graph; controls are defined only by a type; the Conduit template holds one capability, its operations and one `requirements` map; setting up a Pool sets up its entries; contracts are documents of four kinds. Interface: the Model Context Protocol is the wire, an operation is a tool, Conduit adds keys in `_meta`, a plain server's answers map to outcomes by a fixed table, and the engine presents each consumer its permitted operations as tools. Amendment, at the owner's request: one baseline revision, 2026-07-28, that everyone supports, negotiation above it, and a new version of the specification may raise the baseline. The implementation-defined list of twelve entries is accepted. `conduit.dev` stays a placeholder until the owner names a domain, which must happen before the engine builds. |
| 32 | Accepted: all 37 decisions of the first re-audit ([record](proposals/reaudit/README.md)), as recommended. Among them: every request carries the Conduit and the restrictions that apply, and an onward request names the invocation it serves; a plain server never counts as deduplicating; a delegate's bound limits the policies it may apply; a repeat of an identity is the new attempt; one order among the four reasons; a location's facts are stored configuration; a control is an Operation contract; and the core package, the JSON Schemas and the manifest format are to be written as normative parts of the specification and reviewed as one batch. |
| 33 | Accepted: the 26 follow-up decisions from consolidation pass 1 and the editors' five readings ([record](proposals/reaudit/README.md#follow-up-decisions--accepted)), except F3. The owner ruled that until there is a domain the names come from this repository: the core package is `github.com/shrav-k/conduit/core`, a document's `apiVersion` is `github.com/shrav-k/conduit/v0`, and Conduit's `_meta` keys have the prefix `io.github.shrav-k.conduit/`. This replaces the `conduit.dev` placeholder and settles decision X1 of ruling 31. |
| 34 | The owner asked for a way to change the names later, since they come from the repository only until there is a domain. Done in the repository: `spec/names.yaml` is the one place the names are defined, `python3 tools/set_names.py --check` verifies that every file under `spec/` agrees with it and refuses a retired name, and the same tool rewrites every occurrence for a new domain or repository. Proposed and not yet ruled: a rule in the language that the names belong to a version of the specification, so that a later version can change them. |
| 35 | Accepted: the names belong to a version of the specification. Each version states its names in `spec/names.yaml`, and only a new version may change them, as with the protocol baseline. A document's `apiVersion` says which version it is written for, so an engine that supports two versions loads documents of both. To be written as promises once the editors applying ruling 33 have finished with the files it touches. |
| 36 | The owner asked for the work to be tracked on a pull request ([#6](https://github.com/shrav-k/conduit/pull/6)), and for the vocabulary research pull request ([#5](https://github.com/shrav-k/conduit/pull/5)) to be merged or closed and this branch rebased, keeping a record of the reviewer's contribution and discarding what is no longer needed. Done in the tree: the reviewer's simplification pass is kept as written with a note of its outcome; the precedents and sources are kept as evidence; the candidate tables and walkthroughs, which the rulings replaced, are removed, with links pointing at the last commit that held them. The owner then confirmed the merge: #5 was merged into `main` with a merge commit, which keeps the reviewer's commit under their name, and this branch was rebased onto it. |
| 37 | Accepted: the core package, the document schemas and the manifest format as drafted ([record](proposals/core-and-schemas/README.md)), with five changes: a control that sets a location's facts; requests name objects by reference only; a bound is rejected on a Conduit whose target is not `access` or when applied twice; a package outside the core may not mark an operation as reserved to `root`; and names are unique across kinds within a project. Also accepted: the definitions of the four families, model work, enforced and mediated, project and published. The owner further ruled that findings of the cold readers are to be resolved without waiting for a ruling on each, that the documents outside `spec/` are to be audited once more, and that the repository is to be made ready for a cold read by Codex after these rounds. How each item is applied is in [the instructions](proposals/core-and-schemas/APPLY.md). |
| 38 | After four rounds of the re-audit the coordinator put fourteen points to the owner ([record](proposals/ruling-38/README.md)). Ruled: a secret is referred to and never written as a literal, and a secret type is to be explored before anything is written; the six behaviors of the older design that the specification does not carry are dropped; a composite Conduit opens whole or not at all at setup and its components are independent afterwards; the gate of two clean rounds of cold reading is withdrawn, and convergence is judged on no unsafe finding in the latest round and on the reference model and the vectors; no domain need be named before the engine builds; later work is dropped from v0; a policy that makes an actor's requests to an engine-implemented target unavailable stands, and `apply-policy.v1` warns of it; handing an engine a directory installs no package. Accepted, of the coordinator's decisions: a retired consumer's Conduits stay open for its retirement alone; the resolved form in a step and in an onward request; the interface following the protocol on errors, to be looked at again; and eight changes to what earlier rulings accepted, listed in its record; those that changed what an earlier ruling had accepted are under later amendments. Confirmed: the coordinator's reading of ruling 37, and committing and pushing at each milestone. Pull request 6 is merged by squashing. |
| 39 | Before the cold read by Codex the coordinator put fifteen points to the owner ([record](proposals/ruling-39/README.md)). Ruled: the secret type is not decided: "I feel we need more research into how secrets are handled by cloud providers like AWS and in kubernetes before making a determination"; accepted for it, as direction, that only a Resource reads a secret and only the root Interface may open a Conduit that reads one, that the engine sets a credential in a process's environment at start for servers that take one there, and that an Agent's own provider credential is delivered the same way. Accepted: a composite Conduit opens with a whole decision and an outcome for each component; every logged decision of the coordinator and choice of an editor, before and since ruling 37, and the three corrections to the interface; "the grant is an object" becomes "the Conduit is an object"; an assessed state for each readiness condition, for the owner to confirm; the removal from the tree of the material that the specification superseded, with links pinned to `main`; the historical sections of this tracker moved to the archive; the names of two private repositories removed; the stale branches deleted; the personal address on the merge of pull request 6 left as it is. The sentence on who made the external audit is struck. The rows of this log are labelled as the coordinator's summaries. |
| 40 | Conduit innovates only on connection and access ([record](proposals/ruling-40/README.md)). In the owner's words: "The point of conduit was always to leverage existing infrastructure where possible and only innovate on connection and access"; "We should not innovate on the architecture ourselves just the contracts". Ruled: Conduit owns its documents, connection, access, accountability (what must be recorded) and the shape of its contracts, and everything else is a provider behind a contract, swappable by the user wherever possible, "leaving decisions to the user on architecture and existing tools as long as they meet requirements"; the runtime model becomes a runtime provider contract (run, stop, status); locations stay in the language but a provider reports them, places instances by label and reports a lost machine, while what placement and loss mean for access stays Conduit's; setup graphs reduce to declared dependencies, and an operation built from other operations becomes an ordinary implementation; a request that crosses machines carries cryptographic proof of its sender; one control plane decides every request in the first version, and "a robust version of B [local enforcement] will likely be a requirement for v1"; the changes are made now, before round 5 and the cold read by Codex. |
| 41 | The review of the whole design against ruling 40, and the secret type ([record](proposals/ruling-41/README.md)). Accepted as recommended: every decision of the [principles review](proposals/principles-review/README.md), among them setup and retirement as two hook operations in place of graphs; a runtime provider contract in the shape of CRI; placement by labels, with machines registered from what a provider reports; policies, composite Conduits, requirement composition, `obliges`, re-exports and rename removed from v0; results, cancellation, diagnostics, conditions, records, implementation items, schemas and identity in the standards' own forms; providers as packages that implement core provider capabilities, picked by the user; the secret type as a core capability that each store's package implements, read only by the engine before a run. "Accept all, though I don't necessarily like the name secret reveal for the contract, we should do another vocab audit as well before we finish up pr 7". |

### Later amendments

The rows above are kept as the rulings were given. Later rulings amended some of them:

| Ruling | What it says | Amended by |
| --- | --- | --- |
| 2 | Vocabulary stays provisional. | Replaced by ruling 16; the vocabulary was then ruled (17, 18). |
| 10 | Codex and Claude each build the whole engine. | Narrowed by ruling 14 and layer decision LD4: the two builds cover the language layer, the shared conventions and the implementation interface, and one reference engine is built separately. |
| 14, 15 | "Plugin interface", "plugins". | Vocabulary decision V7 (ruling 17): the implementation interface, and implementations. |
| 15, 17, 19 | Grants, and issuing them. | Rulings 27 to 29: grants are merged into Conduits. The access is what an open Conduit carries, and to issue is to open. |
| 24 | Decision PD1, how a package's exports are named, is held back. | Settled by rulings 25 and 27. |
| 31 | Contracts are documents of four kinds. | Five, with `Schema`, since follow-up decision F21 of the re-audit (ruling 33). |
| 31 | The implementation-defined list has twelve entries. | Fifteen. Ruling 32 added DL13. Decision DA3 of the documents audit, taken under ruling 37, added DL14 when the behaviors that the older text accepted were converted. Round 2 of the re-audit, under ruling 37, added DL15. |
| 31 | `conduit.dev` stays a placeholder. | Replaced by ruling 33: the names come from this repository. |
| 7, 8, 9 | Placement as Conduit's own model: locations registered as Resources with facts, a default location, Conduit's own rules for choosing one. | Ruling 40, point 3b: a provider reports the machines, places instances by label and reports a lost one; what placement and loss mean for access stays Conduit's. |
| 13, 14 (LD2), 23 | Setup graphs, with the static meaning of steps, bindings and dependencies, in the language. | Ruling 40, point 3c: reduced to declared dependencies; an operation built from other operations is an ordinary implementation. |
| 13 | Process layout is part of each engine's runtime profile. | Ruling 40: processes are run by a runtime provider that the user chooses. |
| 31 | Runtimes declared in templates, with start and stop operations and handles the engine holds. | Ruling 40, point 3a: a runtime provider contract (run, stop, status). |
| 15 | "The grant is an object", in the purpose. | "The Conduit is an object", by ruling 39, point 7. |
| 31 | A domain must be named before the engine builds. | Withdrawn by ruling 38, point 5. |
| 19, 20 | Rule RR8 of the relationships record: work that a caller started stays subject to that caller's access whenever it later runs. | Later work is dropped from v0 by ruling 38, point 6: no operation arranges, through the engine, work that outlasts its invocation. ACC-019 states that instead, and IMP-006 is removed. |
| 34 | Proposed: the names belong to a version of the specification. | Accepted as ruling 35. |

Decisions of the coordinator, taken under ruling 37, also changed some of what a ruling had
accepted. The owner accepted each of the six below in ruling 38 (points 10 and 12):

| Ruling | What it says | Changed by the coordinator, and accepted by ruling 38 |
| --- | --- | --- |
| 24 | Decision PD5: two conformance levels. | Three, with Implementation, since row GP16 of the [documents audit](proposals/docs-audit/GAPS.md). |
| 31 | Decision D4: a runtime may be marked `optional: true`. | A runtime carries no such mark; a step that starts it can be optional. [Round 2](proposals/reaudit/round-2/DECISIONS.md). |
| 32 | An operation identity is scoped to one caller and one target. | And to the instance that sent the request (OPS-008). Round 2, finding R3-3. |
| 32 | Decision Q5: the order of reasons is refused, invalid, conflict, unavailable. | Where the decision on access cannot be made, unavailable stands in the place of refused (OPS-023). [Round 3](proposals/reaudit/round-3/DECISIONS.md), finding 3A-1. |
| 37 | The owner's change X1: a control's `changes` is a list of pointers. | A map from an input's name to a pointer. Round 2, finding R2-G2. |
| 37 | Requests name objects by reference only. | A step and an onward request may hold the resolved form. Round 3, finding 3B-2. |

## Layered claim

Ruled 2026-11-05 (ruling 13). An earlier working claim, that the documents determine one
observable outcome for every situation, is the bar for a programming-language specification. It
is withdrawn. Conduit is a configuration language, and by that bar even Kubernetes would fail:
its control plane has one implementation, its conformance suite tests a core subset, and most of
its behavior lives in controllers rather than in the language.

**The claim:** the language fully determines what a set of declarations means: which objects
exist, how they relate and which invariants always hold. How a runtime gets there is the
runtime's contract.

| Layer | What it holds | Comparable part of Kubernetes | How precise |
| --- | --- | --- | --- |
| Language | The five families, template and instance, identity and references, instantiation, placement, Conduits and grants as declared data, schemas | Resource schemas and API conventions | Fully specified; small |
| Shared conventions | The few behaviors every part must agree on, such as operation identity, the outcome set, revisions and what a grant permits | Resource versions, optimistic concurrency, finalizers | Specified as invariants, not step by step |
| Type contracts | How each type behaves: an Agent's conditions (running, paused, idle), a directory's edge cases, message read positions | Each controller's behavior | Owned by the type's author; versioned separately |
| Runtime profile | Engine choices: outage handling, retries, cursors, process layout | The reference control plane's behavior | Decided and tested for the reference engine; other engines may differ |

Behavioral decisions are still made and recorded, in the type contracts and the runtime profile
rather than in the language. The critical concepts are unchanged: Pools owning Resources and
shared logins, Conduits carrying grants, placement, instantiation, explicit actions rather than
a background reconciler, control and observation through the Interface, and plugins over MCP.

The specification holds three layers: language, convention and interface. The third is the
implementation interface of layer decisions LD3 and LD4, which this section calls the plugin
interface. Type contracts and the runtime profile stay outside the specification. This section
and the table of layer decisions below are kept in the words of rulings 13 and 14: read
"declarations" as documents, "grant" as the access an open Conduit carries, "plugin" as
implementation, and "normative surface" as the specification. MCP is the Model Context
Protocol.

Ruling 40 adds one layer beside these: **provider contracts**. A provider is existing
infrastructure that the user chooses (a secret store, a runtime, a set of machines, a source of
packages), reached through a contract that Conduit states and any tool can implement, as
Kubernetes reaches runtimes, storage and networks through CRI, CSI and CNI. The contracts belong
to the specification; the providers do not. What a runtime profile held about process layout and
placement now belongs to the chosen provider.

### Layer decisions

| ID | Decision | Ruling | Status |
| --- | --- | --- | --- |
| LD1 | The test that assigns a rule to a layer. | Language if it can be answered by reading the declarations alone. Shared convention if two independently written parts would break each other, or a safety promise, by disagreeing. Type contract if it matters only to instances of one type. Runtime profile otherwise. | done: accepted as recommended |
| LD2 | Where setup graphs sit. | Keep graphs. The language defines their static meaning (steps, bindings, dependencies, validity) and two invariants: a step never runs before its prerequisites succeed, and setup succeeds only if every required step does. Scheduling, retry, cancellation and partial outputs move to the runtime profile. | done: accepted as recommended |
| LD3 | The list of shared conventions, and whether one engine's internal services must work with another's. | Six conventions: operation identity and outcome set; revision-checked change; no managed access without a recorded grant, checked at the target, with restrictions only narrowing; declarations distinct from observed state, with no effect from registration; single Pool ownership and retire-before-unregister; an inspectable record of every managed operation. Cross-engine interoperation of internal services is not a v0 goal; the plugin interface is the shared wire. | done: accepted as recommended. There are seven conventions since ruling 17 |
| LD4 | What the two independent builds must cover. | The language layer in full (declarations in, resolved objects and diagnostics out), the shared conventions through vectors, and the plugin interface. One reference engine; a second full engine is optional later. | done: accepted as recommended |
| LD5 | Where each layer is written down. | The language and shared conventions in the normative surface as promises with vectors. Each type contract with its type, in a standard library shipped beside the reference engine. The runtime profile with the reference engine. | done: accepted as recommended |

## Priorities

| Priority | Theme | Why it comes here |
| --- | --- | --- |
| P0 | Make the language concrete, complete and free of known defects | The owner's goal, and the precondition for everything else. |
| P1 | Prove the language can be rebuilt and its promises kept | The reference model, vectors, model checking and the two engine builds; native probes run alongside. |
| P2 | Prune and re-baseline, under the scope guard | Uses the evidence to re-sequence what the first slice does not need. |
| P3 | Vocabulary | Moved up by ruling 16: decided now and applied with the rework. |

## Action items

Open, in progress and ruled items are below. Completed and superseded items are in
[the history](archive/plan-audit-history.md#completed-and-superseded-action-items).

Promise ranges and counts in the status notes are as first written; the current files and
counts are in [spec/README.md](../spec/README.md#contents).

### P0 — make the language concrete and complete

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A32 | Define the specification: the exact set of documents from which an engine must be rebuildable. | Goal, Q2, Q10 | The contents list exists and everything outside it is marked as not binding (G1). | in progress: `spec/` exists and [spec/README.md](../spec/README.md) lists its parts. Nothing outside it binds (decision DA1 of the documents audit). Eighteen of the nineteen accepted behaviors that only the older text stated are converted (DA3; [list](proposals/docs-audit/GAPS.md), [log](proposals/docs-audit/log-gaps.md)). What remains is the nineteenth, on secrets (GP3 of the list), which waits for the owner |
| A33 | Build the promise register: restate accepted behavior as numbered promises, each with a vector. | Goal, D8 | Every topic is converted, with word counts before and after (G2). | in progress: every area is written as promises; no vectors; the word counts are not done |
| A42 | Re-triage the defects by layer and close those in the language and shared conventions, per the [defect closure plan](archive/plan-audit-history.md#defect-closure-plan). | Rulings 12, 13 | Readiness condition G7, as ruling 38 restated it: the latest round of readers reports no unsafe finding, and the reference model and the vectors agree with the promises. It was: two consecutive sweeps of the language and shared conventions are clean. | in progress: triage done; 87 of 167 are in the language or shared conventions |
| A51 | First batch of fixes: relationships and access. Eight decisions and sixteen rules are proposed in the [relationships record](proposals/relationships/README.md). | Rulings 15, 17 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled (rulings 19, 20) and written as 41 promises in [spec](../spec/README.md); vectors pending |
| A13 | Second batch of fixes: operations, outcomes and revisions. Five decisions and nine rules are proposed in the [operations record](proposals/operations/README.md). | Q6, L1, L10 | It is ruled, model-checked and swept clean. | in progress: ruled (ruling 20) and written as 27 promises; Experiment 004 model-checked the at-most-once rules as the older text stated them, as an evaluation of the tool and not as evidence about the promises; the promises themselves are not model-checked; vectors and a sweep pending |
| A52 | Third batch of fixes: declarations. Five decisions and thirteen rules, in the [declarations record](proposals/declarations/README.md). | Ruling 14 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled (rulings 21, 22) and written as 45 promises; vectors pending |
| A53 | Fourth batch of fixes: graphs and the schema profile. Four decisions and nine rules, in the [graphs and schemas record](proposals/graphs-and-schemas/README.md). | Ruling 14 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled (ruling 23) and written as 27 promises; vectors pending |
| A54 | Fifth batch of fixes: packages, first start and conformance. Five decisions and eight rules, in the [packages and start record](proposals/packages-and-start/README.md). | Rulings 14, 23 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled except PD1 (ruling 24), which rulings 25 and 27 then settled (A55), and written as 20 promises; vectors pending |
| A59 | Consolidate the promises and audit the whole specification again with cold readers who see only `spec/`. | Ruling 30 | As restated by ruling 38, point 4: the latest round of readers reports no unsafe finding, and the reference model and the conformance vectors agree with the promises. It was: two consecutive rounds with no finding in the language, conventions or interface, which four rounds showed reading alone does not reach. | in progress: round 1 done, with about 170 findings, 11 of them unsafe; 37 decisions accepted (ruling 32) and applied by consolidation pass 1, which changed 212 promises, removed 21 and added 48 ([record](proposals/reaudit/README.md)). The follow-up decisions (rulings 33 to 35) are applied. Round 2 is done: four readers, 125 findings in `spec/`, of which 8 classed as unsafe (7 by a reader, 1 by the coordinator), the decisions in [DECISIONS.md](proposals/reaudit/round-2/DECISIONS.md), and the fixes applied in four passes ([record](proposals/reaudit/round-2/README.md)). Round 3 is done: 44 findings in `spec/`, 4 classed as unsafe, the decisions in [DECISIONS.md](proposals/reaudit/round-3/DECISIONS.md), and the fixes applied ([record](proposals/reaudit/round-3/README.md)). No round has been clean. Round 4 is done: 38 findings in `spec/`, 3 classed as unsafe, the decisions in [DECISIONS.md](proposals/reaudit/round-4/DECISIONS.md), and the fixes applied ([record](proposals/reaudit/round-4/README.md)). No fifth round is started. Ruling 38 withdrew the gate of two clean rounds; the next read is the owner's cold read by Codex, and after it the vectors and the reference model |
| A61 | Audit the documents outside `spec/` and make the repository ready for a cold read. | Ruling 37 | Every decision of the audit is applied, and a cold reader can tell from the entry points what binds. | in progress: audited, and decisions DA1 to DA13 are applied ([record](proposals/docs-audit/README.md), which links the logs). The second half of the done-when was put to the fourth reader of round 2 of the re-audit, who read the whole repository cold and could tell from the entry points what binds ([report](proposals/reaudit/round-2/reader-4.md)). What was done about that reader's other findings, and what is left for the owner, is in [the log](proposals/reaudit/round-2/log-reader-4-fixes.md). The fourth reader of round 3 reported 23 findings, 9 of them misleading, and the fourth reader of round 4 reported 26, 8 of them misleading; each is fixed or is listed for the owner ([round 3](proposals/reaudit/round-3/DECISIONS.md), [round 4](proposals/reaudit/round-4/README.md)). The action is done when such a reader reports nothing that misleads. The cold read by Codex that ruling 37 names is the owner's to start |
| A41 | Fix the uncertain-write rule (L1) with A13. | L1 | The convention states when reissue is safe and a vector covers it. | ruled: ruling 20, OPS-012 to OPS-014; vector pending |
| A12 | Name the target MCP revision and specify the implementation interface and its profile. Whether the wire between an engine's own services is shared is layer decision LD3. | Q2, G9 | An implementation written against the interface works with any conforming engine. | ruled: ruling 31, IFC-001 to IFC-017; the baseline revision is 2026-07-28; vectors and test implementations pending |
| A11 | State what the language requires of caller authentication, then choose the mechanism from the native probes. | Q3 | Both are in the specification. | in progress: the requirement is stated. The engine establishes which instance the consumer is for each request, and no consumer can present as another (IFC-022); what a consumer presents is never sent to an implementation (IFC-030); how a request shows which instance it comes from is item DL13 of the implementation-defined list (re-audit decisions U10 and F2). The mechanism waits for the native probe (A4) |
| A16 | Conduit instance granularity, participation and grant lifetime. | Q5 | Ruled in the layer it is assigned to and swept clean. | ruled: a Conduit has one consumer and one target (ruling 17), and what closes one is ruled (rulings 19, 28, 29); vectors pending |
| A25 | Agent lifecycle, including what survives replacement. | Q8 | Ruled in the layer it is assigned to and swept clean. | ruled by the specification, settled by the coordinator under ruling 37 as ruling 38, point 13, confirmed it, and accepted by ruling 39: a replacement is a new instance, with an ID of its own: LIF-009 for a nested one, and the registration of a new document for any other, as the definition of an ID in `spec/definitions.md` has it. It holds none of the retired instance's Conduits, which closed when that instance was retired (CON-008), and its own declared Conduits are opened by its setup (CON-004). What its runtimes keep is its type's contract (`restoresRuntimes`, CTR-004). Vector pending |
| A17 | State what runs with no frontend attached and what starts it. | Q7 | It is a promise. | open |
| A35 | Specify the declare-lost operation, which is reserved to the root Interface. | Ruling 7 | Its contract, fencing and effects are promises. | ruled: PLC-017, reserved to the root Interface; vector pending |
| A43 | Write the schemas and a strict loader in Rust. | G3 | Every scenario file validates and the negative cases are rejected. | in progress: the schemas are written in `spec/schemas` and accepted (A60); the Rust loader is open |
| A21 | Repair status and consistency defects in the documents. | Q10 | Each item under [known inconsistencies](archive/plan-audit-history.md#known-inconsistencies) is fixed or dropped. | in progress: readiness audit applied 2026-11-05. On 2026-11-06 each of the eleven items was checked against the tree and its state is noted beside it. Three are overtaken, two partly. Five stand as written in files that are marked as not binding or are not edited; whether to drop them is the owner's call. One, a stale branch, waits for the owner |

### P1 — prove the language can be rebuilt and its promises kept

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A38 | Write the executable reference model in Rust for the language layer and the shared conventions: a pure, deterministic library in which every case cites a promise. | Rulings 10, 14 | It passes every vector with no unhandled case (G4). | open |
| A39 | Publish the conformance vectors, with a held-out set. | Ruling 10 | Any engine can run them through an adapter; coverage by promise is reported. | open |
| A44 | Mutation-test the reference model against the vectors. | G6 | No unexplained surviving mutant. | open |
| A40 | Model-check the safety promises with Stateright in small configurations, in the ordinary test run, including two machines with message loss, reordering, a partition and a lost machine (ruling 40). | [Experiment 004](../experiments/004-stateright-evaluation/README.md) | Each safety promise holds or has a recorded reason it cannot be checked this way (G5). | open: tool evaluated |
| A45 | Publish the build base: the orphan branch, the brief as `BUILD.md`, agent instruction files, and a pilot run of the brief on S1. | Ruling 12 | G10 passes and the owner signs off (G11). | open: brief and process drafted |
| A34 | Two independent builds in Rust, by Codex and by Claude, of the language layer, the shared conventions and the implementation interface (LD4). | Rulings 10, 14 | Both pass the public and held-out vectors; every divergence is traced to the language and fixed. | open: blocked by the readiness gate |
| A50 | Build one reference engine, with a standard library of core type contracts and a recorded runtime profile. | LD4, LD5 | It runs the nine scenarios. | open |
| A62 | Write the provider contracts of ruling 40 in the specification: the runtime that runs an instance's implementation (run, stop, status); the machines, their labels and the loss of one; the secret store; the source of packages. | Ruling 40 | Each is a contract any existing tool can implement, and the specification names no backing system. | open: after the principles review |
| A63 | Build the first providers, as packages in Rust under a top-level `packages/` directory, one crate per provider (ruling 40, after the ruling): two for each provider contract, on different existing tools. Candidates: secret stores on the engine's own file and variable, the macOS keychain and 1Password; runtimes on a local process and Docker; machines on the local machine and a second host reached over the network; package sources on Git and a local directory. | Ruling 40 | Each passes the vectors of its contract, and S10 passes with every provider swapped. | open: needs A62. For the keychain store, [Experiment 005](../experiments/005-macos-keychain/README.md) measured, in the user's logged-in session: a binary on an item's lists reads it in milliseconds with no dialog; in the login keychain's format an item added with the `security` tool is readable only by Apple's tools until its partition list is changed, so the package should create its own items or set that list; a rebuild that changes the binary loses access unless it is signed with a stable identity; a locked keychain refuses with the same error code as an untrusted binary. A real background daemon (probe E10) is still untested |
| A64 | Require proof of the sending instance on every request that crosses machines, with the mechanism swappable (mutual TLS, SPIFFE, the authorization of the Model Context Protocol). | Ruling 40, point 3e | Written as a promise; a vector shows a request without proof refused. | open |
| A65 | Design local enforcement for v1: each machine enforces access from configuration pushed to it, with a stated bound on how long a revocation takes to arrive. | Ruling 40, point 3f | A proposal for the owner, before v1. Nothing in v0 may assume that every decision is made in one place. | open |
| A14 | Run the acceptance scenario on a built engine and use it for a week. | Ruling 4 | Each promise is scored exercised, contradicted or never needed. | open |
| A4 | Native launch and identity probe. | E1 | Results recorded for Claude Code and Codex. | open |
| A5 | Four-account coexistence probe, 48 hours. | E2 | Refresh and logout behavior recorded per account. | open |
| A6 | Mediation proxy probe, including a lost response. | E3 | A trace exists for discovery, invocation, refresh and a lost response. | open |
| A7 | Bypass probe, and the honest v0 security claim. | E4, D5 | The claim is a promise. | open |
| A10 | Messaging and next-turn notice probe. | E5 | A two-Agent exchange is recorded with the human prompts it needed. | open |
| A8 | Check subscription terms for the owned model loop. | E11 | The owned loop is confirmed, changed to API keys or dropped. | open |

### P2 — prune and re-baseline, under the scope guard

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A18 | Present promises that no scenario exercised as candidates for later sequencing, each for an owner ruling. Nothing leaves the accepted scope by default. | D1 | Each candidate under [deferrable scope](archive/plan-audit-history.md#deferrable-scope) has a ruling. | open |
| A19 | Run the graph-need comparison and rule on authored graphs in v0. | E8, D6 | Ruling recorded with line and concept counts. | open |
| A20 | Rule on v0 authorization scope. | D5 | The cross-path, issuer-independence and provider-scope questions each have an answer. | open |
| A22 | Keychain probe. | E10 | Results recorded. | open |
| A24 | Adopt a review rule: a new ruling carries a vector that can fail. | D8 | The contributor guide states it. | open |
| A27 | Citation hygiene. | [Reference repositories](archive/plan-audit-history.md#reference-repositories) | The Pi pins are confirmed upstream and the Pi helper probes are retained or their claims removed. | open |

### P3 — vocabulary

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A29 | Apply the chosen names as each area is converted into the specification, recording old and new spellings in the vocabulary map. | A28 | No area is rewritten twice for naming. | in progress: the names are applied in `spec/` as each area was written; the record of old and new spellings is not written |

## Experiment program

Accepted 2026-11-05 (ruling 9).

**Claim to prove:** the [layered claim](#layered-claim), with the provider layer of ruling 40. The
language fully determines what a set of documents means; the shared conventions hold as
invariants, on one machine and across several; everything else is a type contract, a provider or
the runtime profile. And every provider can be swapped: each provider contract has at least two
implementations, built on different existing tools, that pass the same vectors.

| Method | What it shows | Action |
| --- | --- | --- |
| Promise register with must, should and may wording and an explicit implementation-defined list | Every behavior is pinned or openly left to implementations | A33 |
| Executable reference model in Rust | Completeness: an unhandled case is a hole in the language | A38 |
| Public conformance vectors | Any implementation can be checked | A39 |
| Mutation testing of the model against the vectors | Whether the vectors actually pin behavior | A44 |
| Model checking of safety promises under faults | The promises can all hold together | A40 |
| Cold-reader sweeps | Places where two builders would diverge, found cheaply before any build | A42 |
| Two providers for each provider contract, built on different existing tools | The infrastructure really is swappable: the same documents behave the same on either | A63 |
| Multi-machine runs, with a partition and a lost machine | The access promises hold across machines | S3, A40 |
| Independent builds by Codex and by Claude of the language layer, shared conventions and implementation interface | Unambiguity, by the recognized public standard of independent implementations passing one suite | A34 |

| # | Scenario | Mainly exercises |
| --- | --- | --- |
| S1 | Two Agents share a project directory and a message history on one location | All five families, instantiation, communication, inspection |
| S2 | One service login shared by two Agents with different restrictions | Requirements, grants, shared authentication, dependent calls |
| S3 | S1 and S2 across two machines, including a partition and a lost machine. Central to the claim since ruling 40 | Placement by a provider, what a lost machine retires and closes, unknown outcomes across the network, proof of identity across machines |
| S4 | Setup that fails partway, is retried, and loses a response | Graphs, outcome model, recovery |
| S5 | A control change under contention | Revisions, partial confirmation, lost receipt |
| S6 | Access revoked and a Resource retired while requests are in flight | Current-access checks, retirement duties |
| S7 | An outside author packages an existing MCP server and a small code plugin | Packages, contracts, types, location-need declarations |
| S8 | Search over storage returning only what the caller may see | Restrictions on returned content, derived content |
| S9 | First initialization, later start, and explicit repair | Root Interface and foundational setup |
| S10 | S1 and S2 again with every provider swapped: another secret store, another runtime, another package source | Provider contracts (ruling 40) |

The scenarios exercise every layer, so they run on the reference engine with its standard
library of types (A50) and the first providers (A63). The acceptance scenario is S1 and S2 passing there, with their
language-layer and convention parts also passing in both independent builds. Running them with
real Claude and Codex Agents, and the two-Claude, two-Codex account check, are later gates.

Order:

1. Shared foundations, in series: the layer decisions (A48), the form of the specification and
   of a promise (A47), then the defect re-triage (A42).
2. The language and shared conventions, each ruled, checked and swept.
3. Tracer, in series: S1 alone through every method, to fix the file formats for scenarios,
   vectors and promises, on the first provider of each contract.
4. Second providers: a second implementation of each provider contract, on a different existing
   tool, and S10 (A63).
5. Fan-out, in parallel: S2 to S9, one scenario per session, with one integrator. S3 runs on at
   least two real machines.
6. Whole-model checks: model checking, including the safety promises across machines under loss,
   reordering and partition, and mutation testing.
7. The readiness gate, then the two engine builds.
8. Prune.

The vocabulary is no longer a last step: ruling 16 moved it up, and it was decided and applied
with the rework.

In parallel throughout: the native probes, which need only real accounts and elapsed time.

Independence rules for the builds, accepted (ruling 11) and detailed in
[independent builds](implementation/INDEPENDENT_BUILDS.md): each builder is a fresh session that
did not write the language; its only inputs are the specification and the vectors; it cannot
see the other engine, the reference model's source, or anything else in this repository; and the
owner or a neutral harness compares the results.

Removal of the old experiments: 001 was removed on 2026-11-05, once
[spec/schemas/examples](../spec/schemas/examples/) had replaced it as the worked example
(decision DA4 of the documents audit); 002 and 003 were removed by ruling 39.
