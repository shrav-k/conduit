# Plan audit and refinement tracker

Opened: 2026-11-05. Baseline: commit `37de771` (`research/vocabulary-precedents`).

This is the working tracker for the direction the owner set on 2026-11-05 after an external audit
of the plans. It records the rulings, the priorities, the action items and the experiment
program. It is not a specification and amends no contract by itself. The
[current review](REVIEW.md) remains the review entry point and defers to this tracker where they
differ.

Identifier prefixes: **A** action item, **Q** question, **D** disagreement, **E** experiment,
**S** scenario, **LD** layer decision, **G** readiness-gate condition, **L** language
defect ([register](LANGUAGE_DEFECTS.md)). The placement and instantiation records use **PL** and
**IN** for decisions and **PL-n** for promises. None reuses the earlier R, I, C or M ruling sets.

## How to use this document

- Statuses are `open`, `in progress`, `done`, `dropped` or `superseded`, optionally followed by a
  colon and a note.
- A ruling is recorded once in the [ruling log](#ruling-log), then applied to the documents it
  affects.
- Experiment fixtures and dated results live under [experiments](../experiments/README.md).
- A place where the accepted text is undetermined, contradictory or unsafe goes in the
  [defect register](LANGUAGE_DEFECTS.md), not here.
- When every item is closed, move this file to the [archive](archive/README.md).

## Current direction

- **The language is the core; the runtime fulfills its promise.**
- **Conduit is a configuration language, not a behavioral specification.** The language fully
  determines what a set of declarations means. Behavior beyond a few shared conventions lives in
  type contracts and a runtime profile ([layered claim](#layered-claim)).
- **Current work makes the language concrete and provable**: a defined normative surface, a
  promise register for the language and shared conventions, schemas, an executable reference
  model in Rust, public conformance vectors and model checking.
- **Proof of sufficiency** is independent builds by fresh Codex and Claude sessions that see only
  the normative surface and the vectors, and never each other's work. They cover the
  language layer, the shared conventions and the plugin interface; one reference engine is built
  separately. They do not start until the
  [readiness gate](implementation/INDEPENDENT_BUILDS.md#readiness-gate) passes; the owner wants
  high confidence first.
- **Language defects are closed before the builds.** The first sweep found 167 against a stronger
  claim; they are re-triaged by layer after the layer decisions
  ([defect closure plan](#defect-closure-plan)).
- **Any code is Rust**, under `experiments/` until the reference model has a home.
- **Vocabulary is decided now and applied with the rework** (ruling 16), so that names and
  behavior are fixed together and the text is rewritten once. The pass merges concepts as well
  as renaming them ([vocabulary proposal](proposals/vocabulary/README.md)).
- **The relationship is the product** ([purpose](#purpose)). Rework decisions are checked against
  the preservation tests.
- **The acceptance scenario is a first proof, not the scope** ([scope guard](#scope-guard)).

## Ruling log

All rulings are the owner's, given on 2026-11-05, in this order.

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

## Where things stand

| Item | State on 2026-11-05 |
| --- | --- |
| Language defects | 167 recorded against the withdrawn stronger claim. Triage by layer: 48 language, 39 shared convention, 30 type contract, 50 runtime profile. Of the 87 in the first two layers, three are closed and 84 are ruled. Of the 50 in the runtime profile, 49 are answered by an entry of the implementation-defined list and one is ruled. The 30 type-contract defects wait for their contracts. None has a vector. |
| Readiness gate | None of G1 to G11 is met. The gate was revised for layer decision LD4. |
| Specification | [spec](../spec/README.md) exists as a draft with 355 promises across 21 files: 189 in the language layer, 137 in the shared conventions and 29 in the implementation interface, plus the implementation-defined list of 13 entries. Every area has been ruled. A first re-audit by three cold readers found about 170 problems, 11 of them unsafe ([record](proposals/reaudit/README.md)). Consolidation pass 1 applied the 37 accepted decisions and the fixes that needed none, and a follow-up pass applied 25 more rulings (33 to 35). The definitions are not yet brought into line, and the JSON Schemas and the core package are not written. No vectors yet. `conformance/` and `reference-model/` do not exist. |
| Vocabulary | Ruled ([record](proposals/vocabulary/README.md)). Not yet applied to the text. |
| Placement and instantiation | Ruled, and written as promises PLC-001 to PLC-014 and INS-001 to INS-018. |
| Build process and brief | Drafted: [independent builds](implementation/INDEPENDENT_BUILDS.md), [engine build brief](implementation/ENGINE_BUILD_BRIEF.md). Not issued. |
| Code | [Experiment 004](../experiments/004-stateright-evaluation/README.md), a Rust model-checking evaluation. No engine and no reference model. |

## Purpose

Ruled 2026-11-05 (ruling 15).

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
- **The grant is an object.** Each agent's access to each resource can be inspected, narrowed
  and revoked independently of both ends.
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
| Type contracts | How each type behaves: an Agent's lifecycle, a directory's edge cases, message read positions | Each controller's behavior | Owned by the type's author; versioned separately |
| Runtime profile | Engine choices: outage handling, retries, cursors, process layout | The reference control plane's behavior | Decided and tested for the reference engine; other engines may differ |

Behavioral decisions are still made and recorded, in the type contracts and the runtime profile
rather than in the language. The critical concepts are unchanged: Pools owning Resources and
shared logins, Conduits carrying grants, placement, instantiation, explicit actions rather than
a background reconciler, control and observation through the Interface, and plugins over MCP.

### Layer decisions

| ID | Decision | Ruling | Status |
| --- | --- | --- | --- |
| LD1 | The test that assigns a rule to a layer. | Language if it can be answered by reading the declarations alone. Shared convention if two independently written parts would break each other, or a safety promise, by disagreeing. Type contract if it matters only to instances of one type. Runtime profile otherwise. | done: accepted as recommended |
| LD2 | Where setup graphs sit. | Keep graphs. The language defines their static meaning (steps, bindings, dependencies, validity) and two invariants: a step never runs before its prerequisites succeed, and setup succeeds only if every required step does. Scheduling, retry, cancellation and partial outputs move to the runtime profile. | done: accepted as recommended |
| LD3 | The list of shared conventions, and whether one engine's internal services must work with another's. | Six conventions: operation identity and outcome set; revision-checked change; no managed access without a recorded grant, checked at the target, with restrictions only narrowing; declarations distinct from observed state, with no effect from registration; single Pool ownership and retire-before-unregister; an inspectable record of every managed operation. Cross-engine interoperation of internal services is not a v0 goal; the plugin interface is the shared wire. | done: accepted as recommended |
| LD4 | What the two independent builds must cover. | The language layer in full (declarations in, resolved objects and diagnostics out), the shared conventions through vectors, and the plugin interface. One reference engine; a second full engine is optional later. | done: accepted as recommended |
| LD5 | Where each layer is written down. | The language and shared conventions in the normative surface as promises with vectors. Each type contract with its type, in a standard library shipped beside the reference engine. The runtime profile with the reference engine. | done: accepted as recommended |

## Defect closure plan

The register's 167 defects were found against the withdrawn claim. After the layer decisions are
ruled, each defect is assigned to a layer:

| Layer | What happens to the defect |
| --- | --- |
| Language or shared convention | Fixed in the language, with a promise and a vector. Only these count for readiness condition G7. |
| Type contract | Recorded against that type and decided when its contract is written. |
| Runtime profile | Decided for the reference engine, recorded and tested there. |
| None | Closed with the reason, where the layering makes the question disappear. |

The triage is recorded in the [defect register](LANGUAGE_DEFECTS.md#triage-by-layer): 48
language, 39 shared convention, 30 type contract and 50 runtime profile. The estimate made before
triage, about a third in the first two layers, was low; the figure is about half.

## Priorities

| Priority | Theme | Why it comes here |
| --- | --- | --- |
| P0 | Make the language concrete, complete and free of known defects | The owner's goal, and the precondition for everything else. |
| P1 | Prove the language can be rebuilt and its promises kept | The reference model, vectors, model checking and the two engine builds; native probes run alongside. |
| P2 | Prune and re-baseline, under the scope guard | Uses the evidence to re-sequence what the first slice does not need. |
| P3 | Vocabulary | Moved up by ruling 16: decided now and applied with the rework. |

## Action items

### P0 — make the language concrete and complete

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A47 | Owner rulings on the [form of the specification](proposals/normative-surface/README.md): NS1 to NS4. | A32, A33 | — | done: ruling 17 |
| A32 | Define the normative surface: the exact set of documents from which an engine must be rebuildable. | Goal, Q2, Q10 | The manifest exists and everything outside it is marked non-normative (G1). | in progress: form ruled; `spec/` not yet created |
| A33 | Build the promise register: restate accepted behavior as numbered promises, each with a vector. | Goal, D8 | Every topic is converted, with word counts before and after (G2). | open: format ruled |
| A48 | Owner rulings on the [layer decisions](#layer-decisions) LD1 to LD5. | Ruling 13 | — | done: ruling 14 |
| A49 | State the language's purpose and agent-first differentiation, and turn it into tests applied to every rework decision. | Ruling 14 | The purpose and the tests are recorded and the owner has accepted them. | done: [purpose](#purpose) and the nine preservation tests accepted (rulings 15, 17) |
| A42 | Re-triage the defects by layer and close those in the language and shared conventions, per the [defect closure plan](#defect-closure-plan). | Rulings 12, 13 | Two consecutive sweeps of the language and shared conventions are clean (G7). | in progress: triage done; 86 of 167 are in the language or shared conventions |
| A51 | First batch of fixes: relationships and access. Eight decisions and sixteen rules are proposed in the [relationships record](proposals/relationships/README.md). | Rulings 15, 17 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled (rulings 19, 20) and written as 41 promises in [spec](../spec/README.md); vectors pending |
| A54 | Fifth and last batch of fixes: packages, first start and conformance. Five decisions and eight rules, in the [packages and start record](proposals/packages-and-start/README.md). | Rulings 14, 23 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled except PD1 (ruling 24) and written as 20 promises; vectors pending |
| A55 | Naming patterns: research how existing systems name packages and refer to their contents, then decide how Conduit does, across packages, exports, types and capabilities. | Ruling 24, L27 | The naming scheme is ruled and written as promises. | done: ruled (rulings 25, 27) and written as promises NAM-001 to NAM-017; vectors pending |
| A56 | Consolidation audit: find every place where one idea has several unrelated mechanisms, notations or keys, or one key has several meanings. | Ruling 25 | Each finding is consolidated, or recorded as a deliberate distinction. | done: all 26 findings ruled (rulings 27 to 29) and applied to the specification in one pass; the [consolidation record](proposals/consolidation/README.md#rulings) lists the rulings and the choices made while writing them, which the owner has not ruled on one by one |
| A57 | Write the four remaining shared conventions: ownership and lifecycle, declared against observed state, records, and nothing implicit. | Ruling 29 | Each is a file of promises under `spec/conventions/`. | done: ruling 30; written as LIF-014 to LIF-023, ACC-033, PLC-016, OBS-001 to OBS-006, REC-001 to REC-006 and IMP-001 to IMP-007 ([record](proposals/conventions/README.md)) |
| A58 | Decide the schema of each family's `spec` and of contracts and manifests, the implementation interface, and the implementation-defined list. | Ruling 30 | Each is written in `spec/`; the readiness conditions G2 and G9 can then be worked on. | done: ruling 31; written as TPL-001 to TPL-012, CTR-001 to CTR-006, IFC-001 to IFC-017, the implementation-defined list, and additions to the object, graph, Conduit, Pool and package promises ([record](proposals/schemas-and-interface/README.md)). The JSON Schemas follow the re-audit. |
| A59 | Consolidate the promises and audit the whole specification again with cold readers who see only `spec/`. | Ruling 30 | Two consecutive rounds with no finding in the language, conventions or interface. | round 1 done: about 170 findings, 11 of them unsafe; 37 decisions accepted (ruling 32) and applied by consolidation pass 1, which changed 212 promises, removed 21 and added 48 ([record](proposals/reaudit/README.md)). The follow-up decisions (rulings 33 to 35) are applied; 355 promises. In progress: the definitions, and the core package, schemas and manifest format for the owner's review. Then round 2. |
| A60 | Write the concrete surface that the first re-audit found missing (decision G1): the core package as documents, the JSON Schemas of every document, and the manifest's format. | Ruling 32 | Each is a normative part of `spec/`, checked by `tools/check_schemas.py`, and accepted by the owner. | drafted and proposed ([record](proposals/core-and-schemas/README.md)): `spec/core/`, `spec/schemas/` with 6 valid and 40 invalid examples, and `spec/promises/manifest.yaml` (MAN-001 to MAN-014). The checker validates 129 documents and rejects all 40 invalid ones. Awaiting the owner's review of the choices and open points. |
| A53 | Fourth batch of fixes: graphs and the schema profile. Four decisions and nine rules, in the [graphs and schemas record](proposals/graphs-and-schemas/README.md). | Ruling 14 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled (ruling 23) and written as 27 promises; vectors pending |
| A52 | Third batch of fixes: declarations. Five decisions and thirteen rules, in the [declarations record](proposals/declarations/README.md). | Ruling 14 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled (rulings 21, 22) and written as 45 promises; vectors pending |
| A13 | Second batch of fixes: operations, outcomes and revisions. Five decisions and nine rules are proposed in the [operations record](proposals/operations/README.md). | Q6, L1, L10 | It is ruled, model-checked and swept clean. | in progress: ruled (ruling 20) and written as 27 promises; model checking in Experiment 004 supports the at-most-once promises; vectors and a sweep pending |
| A41 | Fix the uncertain-write rule (L1) with A13. | L1 | The convention states when reissue is safe and a vector covers it. | open |
| A12 | Name the target MCP revision and specify the plugin interface and its profile. Whether the wire between an engine's own services is shared is layer decision LD3. | Q2, G9 | A plugin written against the interface works with any conforming engine. | ruled: ruling 31, IFC-001 to IFC-017; the baseline revision is 2026-07-28; vectors and test implementations pending |
| A11 | State what the language requires of caller authentication, then choose the mechanism from the native probes. | Q3 | Both are in the normative surface. | open |
| A16 | Conduit instance granularity, participation and grant lifetime. | Q5 | Ruled in the layer it is assigned to and swept clean. | open |
| A25 | Agent lifecycle, including what survives replacement. | Q8 | Ruled in the layer it is assigned to and swept clean. | open |
| A17 | State what runs with no frontend attached and what starts it. | Q7 | It is a promise. | open |
| A35 | Specify the owner-only declare-lost operation. | Ruling 7 | Its contract, fencing and effects are promises. | open |
| A43 | Write the schemas and a strict loader in Rust. | G3 | Every scenario file validates and the negative cases are rejected. | open |
| A21 | Repair status and consistency defects in the documents. | Q10 | Each item under [known inconsistencies](#known-inconsistencies) is fixed or dropped. | in progress: readiness audit applied 2026-11-05 |
| A1 | Permit exploratory code. | Q9 | — | done: ruling 3 |
| A2 | Name the acceptance scenario. | Q1 | — | done: S1 and S2 passing every method; real Agents and the four-account check are later gates |
| A9 | Decide how v0 honors multi-system support. | D4, Q11 | — | done: ruling 5 |
| A31 | Settle placement in the language. | Q11 | — | done: ruling 7; written as promises PLC-001 to PLC-013 |
| A36 | Settle instantiation arguments. | Q12 | — | done: ruling 8; written as promises INS-001 to INS-018 |
| A37 | Redesign the experiment set. | Ruling 9 | — | done: see [experiment program](#experiment-program) |
| A26 | Define the independent author. | Q4 | — | done: a fresh session that did not write the language (ruling 11) |
| A46 | Apply the repository readiness audit. | Ruling 12 | — | done 2026-11-05: entry documents, agent instruction files, status notes and ignore rules |
| A3 | Cold-author test. | E0 | — | superseded: by the cold-reader sweeps of A42 |

### P1 — prove the language can be rebuilt and its promises kept

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A38 | Write the executable reference model in Rust for the language layer and the shared conventions: a pure, deterministic library in which every case cites a promise. | Rulings 10, 14 | It passes every vector with no unhandled case (G4). | open |
| A39 | Publish the conformance vectors, with a held-out set. | Ruling 10 | Any engine can run them through an adapter; coverage by promise is reported. | open |
| A44 | Mutation-test the reference model against the vectors. | G6 | No unexplained surviving mutant. | open |
| A40 | Model-check the safety promises with Stateright in small configurations, in the ordinary test run. | [Experiment 004](../experiments/004-stateright-evaluation/README.md) | Each safety promise holds or has a recorded reason it cannot be checked this way (G5). | open: tool evaluated |
| A45 | Publish the build base: the orphan branch, the brief as `BUILD.md`, agent instruction files, and a pilot run of the brief on S1. | Ruling 12 | G10 passes and the owner signs off (G11). | open: brief and process drafted |
| A34 | Two independent builds in Rust, by Codex and by Claude, of the language layer, the shared conventions and the plugin interface (LD4). | Rulings 10, 14 | Both pass the public and held-out vectors; every divergence is traced to the language and fixed. | open: blocked by the readiness gate |
| A50 | Build one reference engine, with a standard library of core type contracts and a recorded runtime profile. | LD4, LD5 | It runs the nine scenarios. | open |
| A14 | Run the acceptance scenario on a built engine and use it for a week. | Ruling 4 | Each promise is scored exercised, contradicted or never needed. | open |
| A4 | Native launch and identity probe. | E1 | Results recorded for Claude Code and Codex. | open |
| A5 | Four-account coexistence probe, 48 hours. | E2 | Refresh and logout behavior recorded per account. | open |
| A6 | Mediation proxy probe, including a lost response. | E3 | A trace exists for discovery, invocation, refresh and a lost response. | open |
| A7 | Bypass probe, and the honest v0 security claim. | E4, D5 | The claim is a promise. | open |
| A10 | Messaging and next-turn notice probe. | E5 | A two-Agent exchange is recorded with the human prompts it needed. | open |
| A8 | Check subscription terms for the owned model loop. | E11 | The owned loop is confirmed, changed to API keys or dropped. | open |
| A30 | Breadth scenarios unlike the acceptance scenario. | Scope guard | — | done: S3 and S8 in the suite |
| A15 | Recovery and outage proofs under a simulated network. | E7 | — | superseded: by A40 for the model and A34 for the engines |

### P2 — prune and re-baseline, under the scope guard

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A18 | Present promises that no scenario exercised as candidates for later sequencing, each for an owner ruling. Nothing leaves the accepted scope by default. | D1 | Each candidate under [deferrable scope](#deferrable-scope) has a ruling. | open |
| A19 | Run the graph-need comparison and rule on authored graphs in v0. | E8, D6 | Ruling recorded with line and concept counts. | open |
| A20 | Rule on v0 authorization scope. | D5 | The cross-path, issuer-independence and provider-scope questions each have an answer. | open |
| A22 | Keychain probe. | E10 | Results recorded. | open |
| A23 | Map Experiment 002's cases onto promises and vectors, then remove it. | D3, ruling 9 | Every case is covered by a vector or dropped with a reason. | open |
| A24 | Adopt a review rule: a new ruling carries a vector that can fail. | D8 | The contributor guide states it. | open |
| A27 | Citation hygiene. | [Reference repositories](#reference-repositories) | The Pi pins are confirmed upstream and the Pi helper probes are retained or their claims removed. | open |

### P3 — vocabulary

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A28 | Decide the final vocabulary, with merging of concepts permitted, folding in the reviewer's simplification pass. | D7, ruling 16 | Decisions V1 to V8 of the [vocabulary proposal](proposals/vocabulary/README.md) are ruled. | done: rulings 17 and 18 |
| A29 | Apply the chosen names as each area is converted into the specification, recording old and new spellings in the vocabulary map. | A28 | No area is rewritten twice for naming. | open |

## Experiment program

Accepted 2026-11-05 (ruling 9).

**Claim to prove:** the [layered claim](#layered-claim). The language fully determines what a set of
declarations means; the shared conventions hold as invariants; everything else is a type
contract or the runtime profile.

| Method | What it shows | Action |
| --- | --- | --- |
| Promise register with must, should and may wording and an explicit implementation-defined list | Every behavior is pinned or openly left to implementations | A33 |
| Executable reference model in Rust | Completeness: an unhandled case is a hole in the language | A38 |
| Public conformance vectors | Any implementation can be checked | A39 |
| Mutation testing of the model against the vectors | Whether the vectors actually pin behavior | A44 |
| Model checking of safety promises under faults | The promises can all hold together | A40 |
| Cold-reader sweeps | Places where two builders would diverge, found cheaply before any build | A42 |
| Independent builds by Codex and by Claude of the language layer, shared conventions and plugin interface | Unambiguity, by the recognized public standard of independent implementations passing one suite | A34 |

| # | Scenario | Mainly exercises |
| --- | --- | --- |
| S1 | Two Agents share a project directory and a message history on one location | All five families, instantiation, communication, inspection |
| S2 | One service login shared by two Agents with different restrictions | Requirements, grants, shared authentication, dependent calls |
| S3 | S1 and S2 across two locations, including a partition and a lost location | Placement promises, outage rule |
| S4 | Setup that fails partway, is retried, and loses a response | Graphs, outcome model, recovery |
| S5 | A control change under contention | Revisions, partial confirmation, lost receipt |
| S6 | Access revoked and a Resource retired while requests are in flight | Current-access checks, retirement duties |
| S7 | An outside author packages an existing MCP server and a small code plugin | Packages, contracts, types, location-need declarations |
| S8 | Search over storage returning only what the caller may see | Restrictions on returned content, derived content |
| S9 | First initialization, later start, and explicit repair | Root Interface and foundational setup |

The scenarios exercise all four layers, so they run on the reference engine with its standard
library of types (A50). The acceptance scenario is S1 and S2 passing there, with their
language-layer and convention parts also passing in both independent builds. Running them with
real Claude and Codex Agents, and the two-Claude, two-Codex account check, are later gates.

Order:

1. Shared foundations, in series: the layer decisions (A48), the normative surface and promise
   format (A47), then the defect re-triage (A42).
2. The language and shared conventions, each ruled, checked and swept.
3. Tracer, in series: S1 alone through every method, to fix the file formats for scenarios,
   vectors and promises.
4. Fan-out, in parallel: S2 to S9, one scenario per session, with one integrator.
5. Whole-model checks: model checking and mutation testing.
6. The readiness gate, then the two engine builds.
7. Prune and vocabulary.

In parallel throughout: the native probes, which need only real accounts and elapsed time.

Independence rules for the builds, accepted (ruling 11) and detailed in
[independent builds](implementation/INDEPENDENT_BUILDS.md): each builder is a fresh session that
did not write the language; its only inputs are the normative surface and the vectors; it cannot
see the other engine, the reference model's source, or anything else in this repository; and the
owner or a neutral harness compares the results.

Removal of the old experiments: 001 when S1 and S2 exist; 002 after its cases are mapped onto
promises and vectors (A23); 003 when the Rust loader exists (A43).

## Findings

### Needs clarification

| ID | Question | Detail | Status |
| --- | --- | --- | --- |
| Q1 | What does v0 do that a plain MCP configuration cannot? | Sharing one service with two Agents takes about 260 lines across six documents in the Experiment 001 fixture, against about five lines today. | done: S1 and S2 name it, one login shared under per-Agent restriction, plus shared history |
| Q2 | What is the protocol, normatively? | No MCP revision is named in the protocol reference, the profile keys are unselected and the wire between services is open. | answered by ruling 31 for the wire to implementations and consumers; the wire between an engine's own services is not shared (LD3) |
| Q3 | How does a native Agent prove who it is? | Every access rule rests on an authenticated originating caller, but [owner and client recognition](protocol/MODEL.md#interfaces-and-the-user-workflow) is left open. | open: A11 |
| Q4 | Who is the independent author? | The project has one owner and the sessions that wrote the language. | done: A26 |
| Q5 | What is the granularity of a Conduit instance? | Per consumer and target, or shared; grant lifetime for a crashed Agent. | open: A16 |
| Q6 | What is the outcome model? | Prerequisites require successful completion, but no representation of success, failure or uncertainty exists. | open: A13 |
| Q7 | What runs when no frontend is attached? | Evidence must persist, yet no always-running process is required. | open: A17 |
| Q8 | What survives Agent replacement? | A replacement does not automatically acquire private context or grants. | open: A25 |
| Q9 | When may code be written? | — | done: ruling 3 |
| Q10 | Which text is normative? | The proposals folder holds about 120 "Accepted" paragraphs outside the protocol reference. | in progress: A47 |
| Q11 | How does the language say where an instance runs? | — | done: ruling 7 |
| Q12 | What may an instantiator supply, and how does a template say so? | — | done: ruling 8 |

### Disagreements

| ID | Disagreement | Outcome | Status |
| --- | --- | --- | --- |
| D1 | Risk was tested in the wrong order: the native-Agent and account assumptions were scheduled last. | The native probes run in parallel with the language work. | done |
| D2 | "The language is the primary product" was asserted, not argued. | Ruled, and made testable by the two independent builds. | done |
| D3 | Experiment 002 was v0, not a spike. | Its cases are mapped onto promises and vectors, then it is removed (A23). | done |
| D4 | The first proof was distributed before anything ran on one machine. | Placement in the language, then separated components under a simulated network. | done |
| D5 | The authorization machinery exceeds what is enforceable natively. | To be ruled under A20, informed by the bypass probe (A7). | open |
| D6 | The graph language is verbose and weak at once. | To be decided by the graph-need comparison (A19). | open |
| D7 | A vocabulary pass before code would churn again after code. | The pass comes last and may merge concepts. | done |
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

Kept as the record of ruling 5. "Multi-system support" has two parts, and the plans at the
time of the audit covered only the first.

**Runtime.** Can parts of Conduit run in separate processes or machines and survive each other's
failure? The accepted first proof answers by building one management service and two execution
services as separate processes on one machine, each with its own store.

**Language.** Can an author say that this Resource runs on one system and that Agent on another?
At the time of the audit, no: Pool ownership was explicitly not placement and nothing else selected
a host or execution service. The [placement record](proposals/placement/README.md) now answers it.

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
across two machines. Option (c) is the [placement record](proposals/placement/README.md).

## Supporting detail

### Research and design gaps

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

- `proposals/authoring-syntax/AUTHORING.md` lines 433–436 declare a singular runtime with output
  references, against the accepted named runtime map and explicit start operation.
- The two package entry-document examples (authoring proposal and package walkthrough) use
  different fields, and neither carries a format version.
- Control declarations differ between the package walkthrough and `instances.yaml`.
- Templates in Experiment 001 hard-code predicted instance names, so they are per-deployment
  documents rather than reusable templates.
- Operations that create instances, open Conduits and start runtimes are presented as ordinary
  plugin exports, with no plugin-to-host interface described.
- The conceptual review relies on rulings R1–R7, which are defined nowhere in the current tree.
- `research/ESTABLISHMENT.md` has `<session-workspace>`, an artifact of a Session-to-Agent
  replacement whose intended text is not known.
- Rulings R1, R5 and R6 are cited in the lifecycle topic and defined nowhere.
- Bare labels such as `[DESIGN]`, `[REVIEW]` and "specification" point at files that have since
  moved; the link targets resolve but the labels mislead.
- About 30 links point into Experiments 001 to 003 and will break when those are removed; the
  vocabulary map holds 17 of them.
- The local branch `implementation/rust-runtime-plugins` contains no code and its name could
  mislead.

Fixed on 2026-11-05: stale status and sequence statements in the entry documents; the overview's
date; the corrupted citation URL in `research/CONFIGURATION.md`; "a Agent-local"; the two shapes
used for a Pool entry; the placement record's proposal wording; and the note that the migration
checker cannot fail.

### Deferrable scope

Accepted for v0 but possibly not needed by the first slice, to be confirmed by the scenarios (A14): all 19 enforced
formats; re-exports and replaceable dependencies; multiple named runtimes and the runtime-target
source; the invocation-metadata source; the object and list binding forms; graph-backed
operations and public-output prerequisites; requirement composition and recursion; live
ownership-scoped bindings and bounded issuance; partial control confirmation; registration
batches as graphs; human-interaction wait retention; snapshot-to-feed handoff; edit-mode capture
and symlink rules; conditional-default rejection; persistence, custody and observation as plugin
contracts; type contracts for Pool, Conduit and Interface.

## Reference repositories

No clone held uncommitted work, stashes or local-only branches, and nothing in this repository
reads a sibling directory. All citations of these repositories are commit-pinned GitHub URLs.

Kept beside this repository, as `../agentwatch` and `../pi-reference`, through the first
implementation slice. Consult only; never edit them:

| Repository | Pin | Why |
| --- | --- | --- |
| `jxp-software/agentwatch` | `04025a6` | The only Rust code near the planned stack: MCP server, Claude Code hooks, room sequencing. Proprietary, so consult only. Its links will likely fail for readers outside that organization. |
| `earendil-works/pi` | `0441483` | Named frontend reference. This pin is a detached shallow HEAD; confirm it resolves upstream. A sixth cited commit, `642e30b`, was never in the clone. |

Removed on 2026-11-05; re-clone at the pin if needed:

| Repository | Pin |
| --- | --- |
| `xerj-org/xerj` | `63b6aef` |
| `tilt-dev/tilt` | `f3acf27` |
| `jxp-software/jxp-skills` | `b49a754` |
| `aaalexliu/pstack-pi` | `c87e65f` |
| `earendil-works/pi-chat` | `9adbd29` |
| `jayzeng/pi-memory` | `1e276a7` |
| `josephkern/pi-memory` | `312627f` |
| `nicobailon/pi-messenger` | `09937ed` |
| `desek/agent-observability` | `bab5337` (cited nowhere in this repository) |
