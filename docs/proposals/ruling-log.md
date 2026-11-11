# Ledger of rulings

History. What holds now is [IN_FORCE.md](../../IN_FORCE.md); the specification alone binds behavior.

The specification alone binds behavior. IN_FORCE.md holds every other decision in force and marks accepted language decisions not yet written as spec pending. Where IN_FORCE.md differs from the owner's words, the owner's words govern until it is fixed; builds and vectors follow spec/ until pending changes are written. The ledger records what was ruled and its status; each record says why (ruling 67, point 1).

| # | Date | Ruling | The owner's words | Status |
| --- | --- | --- | --- | --- |
| 1 | 2026-11-05 | The external audit guides action items and priorities; unneeded reference clones are removed. No record. | (the coordinator's summary) | in force; open (reference clones after increments 1 to 3) |
| 2 | 2026-11-05 | Vocabulary stays provisional until the language is shown to be expressive enough. No record. | (the coordinator's summary) | replaced by 16 |
| 3 | 2026-11-05 | Exploratory code may be written and run whenever useful, in Rust only. No record. | (the coordinator's summary) | in force |
| 4 | 2026-11-05 | Claude and Codex on one project on one machine is the starting acceptance scenario; passing it must not narrow the design. No record. | (the coordinator's summary) | in force |
| 5 | 2026-11-05 | Work across machines: settle placement in the language first (c), then separated components under a simulated network (b). No record. | (the coordinator's summary) | done (c: placement, ruling 7); open (b) |
| 6 | 2026-11-05 | The language is the core and the runtime fulfils its promise; whatever makes it more concise, concrete and useful is done. No record. | (the coordinator's summary) | in force |
| 7 | 2026-11-05 | [Placement](placement/README.md) PL1 and PL3 to PL8: no new managed work when cut off, types state location needs, declare-lost. | (the coordinator's summary) | in force; amended by 40 (point 3b); amended by 41 (items 8, 8a to 8c); amended by 42 (node); open (PL8: fork-from-checkpoint) |
| 8 | 2026-11-05 | [Instantiation](instantiation/README.md): a closed list of declared inputs, structure never an input, explicit destinations, only the result stored; standard arguments. | (the coordinator's summary) | in force; amended by 40 (point 3b); amended by 41 (items 8, 31b); amended by 55 (A6) |
| 9 | 2026-11-05 | The nine-scenario experiment program, order and parallel work; old experiments rewritten or removed. No record; see [ROADMAP.md](../../ROADMAP.md). | (the coordinator's summary) | in force; amended by 16 (vocabulary order); amended by 40 (providers); amended by 41 (experiment scope); amended by 44 (first build next); amended by 45 (roadmap) |
| 10 | 2026-11-05 | Sufficiency must be provable to a user of the protocol; the Rust reference model is accepted; full rebuilds if proof needs them. No record. | (the coordinator's summary) | in force; amended by 14 (LD4) |
| 11 | 2026-11-05 | Independent builds follow independence rules; the session that wrote the language builds no engine. No record. | (the coordinator's summary) | in force |
| 12 | 2026-11-05 | An explicit build brief, builders on separate branches; language defects closed first; builds wait until success is very likely. No record. | (the coordinator's summary) | in force |
| 13 | 2026-11-05 | [The layered claim](../AUDIT.md#layered-claim): documents determine meaning; behavior beyond shared conventions belongs to type contracts and a conformance document. | (the coordinator's summary) | in force; amended by 40 (providers); amended by 41 (graphs, process layout); amended by 42 (N18) |
| 14 | 2026-11-05 | [Layer decisions](../AUDIT.md#layer-decisions) LD1 to LD5; static graphs; three independently built layers; the language's purpose preserved through rework. | (the coordinator's summary) | in force; amended by 17 (V7); amended by 40 (LD2); amended by 41 (graphs); amended by 42 (N18) |
| 15 | 2026-11-05 | [Purpose](../AUDIT.md#purpose): the relationship is the product; one developer first; teams later; enforcement opted into. | (the coordinator's summary) | in force; amended by 17 (V7); amended by 27 (grants); amended by 28 (grants); amended by 29 (grants); amended by 39 (point 7); amended by 41 (items 2d, 3a) |
| 16 | 2026-11-05 | The vocabulary is decided now, with the reviewer's pass on pull request 5, applied with the rework; replaces ruling 2. No record. | (the coordinator's summary) | in force |
| 17 | 2026-11-05 | Preservation tests, the seventh convention, the [specification's form](normative-surface/README.md), [vocabulary](vocabulary/README.md) V1 and V3 to V8; Conduits pairwise. | (the coordinator's summary) | in force; amended by 27 (grants); amended by 28 (grants); amended by 29 (grants); amended by 42 (names) |
| 18 | 2026-11-05 | [V2](vocabulary/README.md): one kind per family; `from` marks an instance, its absence a template; references give kind and name. | (the coordinator's summary) | in force; amended by 42 (names); amended by 55 (A6) |
| 19 | 2026-11-05 | [Relationships and access](relationships/README.md), RD1 to RD8 and RR1 to RR16; bare names where one kind fits; Conduits to live instances. | (the coordinator's summary) | in force; amended by 27 (grants); amended by 28 (grants); amended by 29 (grants); amended by 38 (point 6); amended by 41 (items 3a, 5) |
| 20 | 2026-11-05 | Conduits to live instances; the promise's `layer` field; [operations, outcomes and revisions](operations/README.md), OD1 to OD5 and OR1 to OR9. | (the coordinator's summary) | in force; amended by 38 (point 6); amended by 41 (items 6a, 6c, 15) |
| 21 | 2026-11-05 | [Declarations](declarations/README.md), DD1 to DD5 and DR1 to DR13; DD2 held until a document is explained concretely. | (the coordinator's summary) | in force; amended by 41 (item 10) |
| 22 | 2026-11-05 | [DD2](declarations/README.md) stands: a submission is a set of documents, checked together, and only registers. | (the coordinator's summary) | in force |
| 23 | 2026-11-05 | [Graphs and the schema profile](graphs-and-schemas/README.md); standing instruction: rely on existing frameworks, nothing revolutionary. | (the coordinator's summary) | in force; amended by 40 (graphs); amended by 41 (items 1, 8a, 20) |
| 24 | 2026-11-05 | [Packages, first start and conformance](packages-and-start/README.md): PD2 to PD5, PR1 to PR8 accepted; PD1 export names held. | (the coordinator's summary) | in force; amended by 25 (PD1); amended by 27 (PD1); amended by 38 (point 12, PD5); amended by 41 (items 9, 12) |
| 25 | 2026-11-05 | [Naming](naming/README.md): address-like package identity, fully qualified references with no aliases, one pin per project; a consolidation audit asked. | (the coordinator's summary) | in force |
| 26 | 2026-11-05 | Connections must work across package versions; whether packages are recorded in templates and instances is to be discussed. No record. | (the coordinator's summary) | in force; done (question on packages: ruling 27) |
| 27 | 2026-11-05 | [Naming](naming/README.md) NA to NF: `address#name`, one manifest, contract names carry versions; [grants merge into Conduits](consolidation/README.md). | (the coordinator's summary) | in force; amended by 41 (item 21, NE) |
| 28 | 2026-11-05 | [Consolidation](consolidation/README.md): all but two recommendations accepted; opening requires a Conduit to `access`; ownership and lifecycle left open. | (the coordinator's summary) | in force; done (open points: ruling 29); amended by 40 (graphs); amended by 41 (CC9, CC11, CC12, CS6) |
| 29 | 2026-11-05 | [Ownership](consolidation/README.md) is fixed and permits nothing; lifecycle moves forward through registered, ready, retired; a closed Conduit is retired. | (the coordinator's summary) | in force; amended by 41 (items 3a, 17); amended by 58 (M41) |
| 30 | 2026-11-05 | The [four remaining conventions](conventions/README.md), fourteen decisions; then one more batch, consolidation, and a re-audit. | (the coordinator's summary) | in force; amended by 41 (item 17); amended by 58 (M41); amended by 59 (A1, OL2); amended by 60 (OL2) |
| 31 | 2026-11-05 | [Documents and interface](schemas-and-interface/README.md): MCP wire, baseline 2026-07-28; twelve implementation-defined entries. | (the coordinator's summary) | in force; amended by 32 (DL13); amended by 33 (F3, F21); amended by 38 (D4; point 5 withdrew the domain); amended by 40 (runtime model); amended by 41 (items 1, 2, 18, 22, 23); amended by 42 (DL16, names); amended by 43 (DL17); amended by 55 (A6); amended by 58 (M69) |
| 32 | 2026-11-05 | [The first re-audit's](reaudit/README.md) 37 decisions as recommended; the core package, schemas and manifest to be written as normative. | (the coordinator's summary) | in force; amended by 38 (point 12); amended by 41 (items 1, 1c, 3a, 5, 6a, 8, 8b, 18, P20) |
| 33 | 2026-11-05 | [Follow-up decisions](reaudit/README.md#follow-up-decisions--accepted) and five readings accepted, except F3: names come from this repository until a domain exists. | (the coordinator's summary) | in force; amended by 41 (items 1, 5, 8, 10, 18); amended by 42 (names) |
| 34 | 2026-11-05 | A way to change the names later: `spec/names.yaml` and `tools/set_names.py`. No record. | (the coordinator's summary) | done (names tool; proposal accepted in ruling 35) |
| 35 | 2026-11-05 | Names belong to the specification version; only a new version changes them. No record. | (the coordinator's summary) | in force; amended by 55 (A6, wording) |
| 36 | 2026-11-05 | Track the work on pull request 6; merge pull request 5, keeping the reviewer's contribution; rebase. No record. | (the coordinator's summary) | done; amended by 58 (retained vocabulary sources; SIMPLIFICATION stays) |
| 37 | 2026-11-05 | [Core package, schemas and manifest](core-and-schemas/README.md), with five changes; definitions; cold-reader findings settled without individual rulings. | (the coordinator's summary) | in force; amended by 38 (points 10, 12); amended by 41 (items 6c, 8, 9, 24, C14, C21); amended by 44 (D06); amended by 55 (A6) |
| 38 | 2026-11-06 | [Fourteen points after round 4](ruling-38/README.md): secrets never literal, later work dropped, the convergence rule, no domain needed before building. | In the record. | in force; done (points 2, 14); amended by 41 (points 1, 3, 7, 8, 11, 12); amended by 42 (N1) |
| 39 | 2026-11-06 | [Fifteen points before Codex](ruling-39/README.md): research secrets; the Conduit is an object; logged decisions accepted; tree cleaned. | In the record. | in force; amended by 41 (points 1 to 5; logged decisions on composition, `obliges`, `narrows`, re-exports); amended by 42 (R5); amended by 44 (C42, accepted again under point 6); amended by 57 (point 11); amended by 58 (point 6 record notes; point 8 assessment); done (points 8 to 14) |
| 40 | 2026-11-06 | [Innovate only on connection and access](ruling-40/README.md); providers for everything else; robust across machines; one control plane in v0. | In the record. | in force; amended by 43 (point 3e, D5's gate); amended by 47 (the layout of `packages/`) |
| 41 | 2026-11-06 | [Review against ruling 40](ruling-41/README.md): graph hooks, CRI-shaped runtime, labels, policies removed, secret contract; planner's choices accepted with P2 changed. | "Accept all, though I don't necessarily like the name secret reveal for the contract, we should do another vocab audit as well before we finish up pr 7"; then "Accept all with p2 changed" | in force; amended by 42 (D6, D8, D12, names); amended by 43 (item 4c, Q3); amended by 44 (D14); amended by 58 (M22, M41) |
| 42 | 2026-11-06 | [The rewrite's open questions](ruling-42/README.md) and the vocabulary audit's twenty names; D5 and D20 held for research. | "I accept every suggestion except for 1 and 8, I'd like some more research and clarification on how to solve the multi machine issue and be forward looking even if some functionality doesn't exist in v0" | in force; amended by 43 (D17); done (D5 and D20: ruling 43) |
| 43 | 2026-11-06 | [Work across machines](ruling-43/README.md): gates, secret headers, packaged programs on the engine's runner, two forms of proof, failover. | "Agree all, prepare for the codex audit" | in force; amended by 58 (M13, M47, M66: later logged choices) |
| 44 | 2026-11-07 | [Codex cold read](ruling-44/README.md): build next, delegation limits stated; revised D06 and D12 accepted again. | In the record. | in force; amended by 61 (B8, point 2: Conduits only) |
| 45 | 2026-11-09 | [A collaborator's two proposals](ruling-45/README.md): the definition kept, A70, A71, A43 and A63 taken, pre-registration, the roadmap shown. | "Let's do as you recommend, but it seems from the overlap our roadmap for the experiments was not easily discoverable. After you merge these two do a pass over the docs and decide the cleanest way to display the roadmap" | in force |
| 46 | 2026-11-09 | [The first build](ruling-46/README.md): one Rust codebase with a pure core and a thin shell; S1 with real Agents; increment logs. | In the record. | in force; amended by 58 (point 4: increment logs become settlements register); open (point 2 fallback against ruling 47's gate) |
| 47 | 2026-11-10 | [The v0 gate and organization](ruling-47/README.md): v0 is the reference engine, required experiments gate publication, repositories and preview planned. | "We need to prove to ourselves with a v0 of the engine that passes all of our experiments that the system works and is worth publishing" | in force; amended by 65 (point 1: public name); directive (A75, A76: namespace after organization) |
| 48 | 2026-11-09 | [The first review, and who works](ruling-48/README.md): reach flags off, interoperable patterns, fresh Codex reads, the coordinator as admin session. | In the record. | in force; done (point 2 b: ruling 49); amended by 58 (work-order records); amended by 61 (B12, A78's trigger) |
| 49 | 2026-11-10 | [Signing in through the engine](ruling-49/README.md): URL elicitations to a person's Interface, never to an Agent; the account checked. | In the record. | in force; amended by 66 (point 3, on point 2) |
| 50 | 2026-11-10 | [Adding machines](ruling-50/README.md): enrollment in the language after research; admission once a person confirms; one pending-action mechanism. | In the record. | in force; done (point 1 design: rulings 52, 56); amended by 66 (point 3) |
| 51 | 2026-11-10 | [Managed machines and peer engines](ruling-51/README.md): ten networking decisions; two modes, managed and peer, for the owner's machines. | In the record. | in force; amended by 56 (point 8, answer); amended by 62 (C4) |
| 52 | 2026-11-10 | [Refusals in two tiers](ruling-52/README.md); the explicitness rule; relative values as SUB-010; enrollment's decisions 1 to 4, 6, 7; measurement. | In the record. | in force; done (point 5: ruling 56); open (point 3: whether relative values go) |
| 53 | 2026-11-10 | [The simplification's first decisions](ruling-53/README.md): five core additions, plan and apply, pins written by install, reach flags, qualified names. | In the record. | in force; done (points 1, 2: ruling 55 A1, A4; point 6: ruling 59 A3) |
| 54 | 2026-11-10 | [An Agent stays in its Fleet](ruling-54/README.md); grants under a lost authority are revoked individually; root may keep one. | In the record. | in force; amended by 60 (Fleet); amended by 61 (B9, B10) |
| 55 | 2026-11-10 | [The short form is language](ruling-55/README.md); stable names; the command approves; no new defaults; tombstone lift; A21 dropped. | "Agree with A and C rulings but we need to discuss B further" | in force; done (B: ruling 56) |
| 56 | 2026-11-10 | [Checking a joining machine](ruling-56/README.md): the provider's method, the engine's minimum, evidence shown only by the engine, root admits. | "Regarding enrollment, I find the path you suggest acceptable for v0, I accept all your recommendations" | in force |
| 57 | 2026-11-10 | [Historical documents may be edited](ruling-57/README.md) and condensed; the standard is relevance, not a perfect record. | In the record. | in force; done (open points: ruling 58) |
| 58 | 2026-11-10 | [How the repository is condensed](ruling-58/README.md): earlier logged decisions accepted with exceptions; record forms; condense at close. | "I like everything and all your recommendations!" | in force; amended by 67 (point 2); directive (A80: the condensation plan's remaining steps) |
| 59 | 2026-11-10 | [Agent-owning family](ruling-59/README.md): ownership, retirement, root's standing Conduits, `list.v1`, short-form key; bounds keep resolved IDs. | In the record. | in force; done (A5 and family name: ruling 60) |
| 60 | 2026-11-10 | [The family is a Fleet](ruling-60/README.md); "project" keeps its meaning; what a Fleet is not. | "Fleet" | in force |
| 61 | 2026-11-10 | [A Fleet's authority](ruling-61/README.md): Pool administration, a bounded right to grant, forwarding, appointment, `make.v1`, review, keep, withhold. | "All as recommended, though what is in C?" | in force; amended by 66 (point 3: B9 pending action also at scoped Interfaces) |
| 62 | 2026-11-10 | [Five smaller Fleet points](ruling-62/README.md): root alone opens root's Conduits, bounds limit reconcile, consent in v1, peers, shared Pools. | "Agree with your recs for c1-c5, let's revisit the rest after your tests" | in force; open (C6 to C12) |
| 63 | 2026-11-10 | [The S1 rerun's counting rules](ruling-63/README.md), flow style documented, and packages that ship standard Conduit templates. | "All as recommended" | in force |
| 64 | 2026-11-10 | [Organization and repository](ruling-64/README.md): merge PR 14; research name; free CI; rename to `engine/`; organization items wait. | In the record. | in force (point 3: free CI); done (points 1, 2 and 4); amended by 65 (point 5: organization steps) |
| 65 | 2026-11-10 | [The public name is ConduitLang](ruling-65/README.md), `conduitlang` everywhere; the language stays Conduit; the steps wait for the owner. | "Sounds good on conduitlang, but I'm unable to take those actions quite yet, write up a doc on action items to switch to conduitlang and how to set each step up" | in force; directive (A76: owner's account steps) |
| 66 | 2026-11-10 | [Scoped Interfaces](ruling-66/README.md): `manages:` expands into ordinary Conduits; pending actions at scoped Interfaces; no Scope object. | "I agree with all your recommendations"; then "Understood, and I'm not sure we even need a grouping shorthand, declaring a new interface should be sufficient" | in force |
| 67 | 2026-11-10 | [Decisions in force, ledger and reasons](ruling-67/README.md), delegated by the owner and decided by the coordinator. | In the record. | in force (coordinator under delegation, strikeable) |

## History: ruling-log preamble

All rulings are the owner's, in this order: rulings 1 to 37 on 2026-11-05, rulings 38 to 43
on 2026-11-06, ruling 44 on 2026-11-07, rulings 45, 46 and 48 on 2026-11-09, and rulings 49 to 59
on 2026-11-10. As of 2026-11-10, number 47 is still reserved for the owner's open pull request
14. Each row is the coordinator's summary of the owner's messages, with the owner's own words quoted where they are
marked as quoted; the owner confirmed that labelling in ruling 39, point 15. The record linked
from a row holds the owner's words in full where the ruling was given point by point.

## History: Rulings 1 to 37 as given

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
| 30 | Accepted: all fourteen decisions on the four remaining shared conventions ([record](conventions/README.md)). An instance is retired from the moment the request is dispatched, and is unregistered only once its shutdown is confirmed; one authority covers an object's whole life; first-start objects are permanent; a difference between declared and observed state is reported and never repaired without a request; a closed list of events is always recorded, and only `root` may purge; model work starts only through an operation whose contract marks it, never from setup. The owner also ruled the order of what remains: one more batch of rulings, then the promises are consolidated and the whole specification is audited again. |
| 31 | Accepted: the last batch ([record](schemas-and-interface/README.md)), every recommendation, with one amendment. Documents: one `implementation` map; camelCase keys and `apiVersion`; structured value references from five sources; a runtime is a pair of start and stop operations; an optional `retire` graph; controls are defined only by a type; the Conduit template holds one capability, its operations and one `requirements` map; setting up a Pool sets up its entries; contracts are documents of four kinds. Interface: the Model Context Protocol is the wire, an operation is a tool, Conduit adds keys in `_meta`, a plain server's answers map to outcomes by a fixed table, and the engine presents each consumer its permitted operations as tools. Amendment, at the owner's request: one baseline revision, 2026-07-28, that everyone supports, negotiation above it, and a new version of the specification may raise the baseline. The implementation-defined list of twelve entries is accepted. `conduit.dev` stays a placeholder until the owner names a domain, which must happen before the engine builds. |
| 32 | Accepted: all 37 decisions of the first re-audit ([record](reaudit/README.md)), as recommended. Among them: every request carries the Conduit and the restrictions that apply, and an onward request names the invocation it serves; a plain server never counts as deduplicating; a delegate's bound limits the policies it may apply; a repeat of an identity is the new attempt; one order among the four reasons; a location's facts are stored configuration; a control is an Operation contract; and the core package, the JSON Schemas and the manifest format are to be written as normative parts of the specification and reviewed as one batch. |
| 33 | Accepted: the 26 follow-up decisions from consolidation pass 1 and the editors' five readings ([record](reaudit/README.md#follow-up-decisions--accepted)), except F3. The owner ruled that until there is a domain the names come from this repository: the core package is `github.com/shrav-k/conduit/core`, a document's `apiVersion` is `github.com/shrav-k/conduit/v0`, and Conduit's `_meta` keys have the prefix `io.github.shrav-k.conduit/`. This replaces the `conduit.dev` placeholder and settles decision X1 of ruling 31. |
| 34 | The owner asked for a way to change the names later, since they come from the repository only until there is a domain. Done in the repository: `spec/names.yaml` is the one place the names are defined, `python3 tools/set_names.py --check` verifies that every file under `spec/` agrees with it and refuses a retired name, and the same tool rewrites every occurrence for a new domain or repository. Proposed and not yet ruled: a rule in the language that the names belong to a version of the specification, so that a later version can change them. |
| 35 | Accepted: the names belong to a version of the specification. Each version states its names in `spec/names.yaml`, and only a new version may change them, as with the protocol baseline. A document's `apiVersion` says which version it is written for, so an engine that supports two versions loads documents of both. To be written as promises once the editors applying ruling 33 have finished with the files it touches. |
| 36 | The owner asked for the work to be tracked on a pull request ([#6](https://github.com/shrav-k/conduit/pull/6)), and for the vocabulary research pull request ([#5](https://github.com/shrav-k/conduit/pull/5)) to be merged or closed and this branch rebased, keeping a record of the reviewer's contribution and discarding what is no longer needed. Done in the tree: the reviewer's simplification pass is kept as written with a note of its outcome; the precedents and sources are kept as evidence; the candidate tables and walkthroughs, which the rulings replaced, are removed, with links pointing at the last commit that held them. The owner then confirmed the merge: #5 was merged into `main` with a merge commit, which keeps the reviewer's commit under their name, and this branch was rebased onto it. |
| 37 | Accepted: the core package, the document schemas and the manifest format as drafted ([record](core-and-schemas/README.md)), with five changes: a control that sets a location's facts; requests name objects by reference only; a bound is rejected on a Conduit whose target is not `access` or when applied twice; a package outside the core may not mark an operation as reserved to `root`; and names are unique across kinds within a project. Also accepted: the definitions of the four families, model work, enforced and mediated, project and published. The owner further ruled that findings of the cold readers are to be resolved without waiting for a ruling on each, that the documents outside `spec/` are to be audited once more, and that the repository is to be made ready for a cold read by Codex after these rounds. How each item is applied is in [the instructions](core-and-schemas/README.md#outcome). |

## History: Multi-system options under ruling 5

## Multi-system options

> **Historical.** Recorded at the audit, before rulings 13 to 37. Not maintained.

Kept as the record of ruling 5. "Multi-system support" has two parts, and the plans at the
time of the audit covered only the first.

**Runtime.** Can parts of Conduit run in separate processes or machines and survive each other's
failure? The accepted first proof answers by building one management service and two execution
services as separate processes on one machine, each with its own store.

**Language.** Can an author say that this Resource runs on one system and that Agent on another?
At the time of the audit, no: Pool ownership was explicitly not placement and nothing else selected
a host or execution service. The [placement record](placement/README.md) now answers it.

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
across two machines. Option (c) is the [placement record](placement/README.md).

## History: Rulings before the plan audit

These extracts preserve the decisions as recorded; the ledger and IN_FORCE.md give their later standing.

The following notes identify later homes without changing the extracts:

| Earlier decision | Later standing or superseding text |
| --- | --- |
| M1 | Early delivery and working-set pinning are recorded here; contract compatibility follows [naming promises](../../spec/promises/naming.yaml), and publication follows ruling 47. |
| M2 | Superseded where the layers of rulings 13 and 14 and the first build of ruling 46 change the older implementation scope. |
| M3 | Resource, Pool and Conduit meaning is now in [definitions](../../spec/definitions.md) and their promises; no separate Context family was added. |
| License and attribution | [LICENSE](../../LICENSE) and [contribution terms](../../CONTRIBUTING.md#license-and-attribution) hold the current terms. |
| Public identities | Superseded by ruling 65's ConduitLang choice; organization steps still wait for the owner. |
| Change and release authority | [IN_FORCE.md, governance](../../IN_FORCE.md#15-how-the-work-is-governed) and ruling 47 hold the current authority and publication gate. |
| Explicitly later scope | Superseded wherever rulings 38 to 43 and 49 to 66 decide a listed feature; the specification determines implemented meaning. |
| I1 | Superseded by rulings 13 and 14's layers and ruling 40's provider contracts and one control plane in v0; the three-process layout is historical. |
| I2 | Native paths and macOS-first remain the recorded direction; real Agents in the first build and the publication gate follow rulings 46 and 47. |
| I3 | Superseded by rulings 40 and 41's providers and removal of graphs and requirement composition. |
| I4 | Superseded by [access promises](../../spec/conventions/access.yaml) and ruling 41's removal of policies; the old SQLite service layout is historical. |
| I5 | Superseded by ruling 41's secret-provider contract and ruling 49's sign-in direction; [secret promises](../../spec/promises/secrets.yaml) bind current behavior. |
| I6 | Capture defaults are preserved here as the recorded direction; [record promises](../../spec/conventions/records.yaml) govern required evidence and [CONFORMANCE.md](../../engine/CONFORMANCE.md) states the first build's limits. |
| I7 | Superseded by [contracts](../../spec/promises/contracts.yaml), [templates](../../spec/promises/templates.yaml) and ruling 41's removal of graphs and requirement composition. |
| C1 | Superseded by rulings 13 and 14's layers and [specification conformance](../../spec/README.md#conformance). |
| C2 | Superseded by the current [access](../../spec/conventions/access.yaml) and [implementation-interface](../../spec/interface/implementation.yaml) promises for onward requests. |
| C3 | Superseded by ruling 29's common lifecycle and current [lifecycle](../../spec/conventions/lifecycle.yaml) and [Conduit](../../spec/promises/conduits.yaml) promises. |
| Review outcomes | Superseded wherever the specification and later rulings change the older summary. |

### History: migration-review rulings and owner decisions before public distribution

Later standing: publication and names follow rulings 47 and 65; the specification supersedes the older behavioral wording.

## Migration-review rulings

- **M1 accepted:** no backward-compatibility promises or obligation to maintain legacy compatibility
  code. Users can pin a working set. Early delivery prioritizes a working protocol; strict release
  coordination, exact packaging and installers are not prerequisites. Claims about particular
  contracts and retained implementation identity still apply. See [compatibility](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/CONFIGURATION.md#release-compatibility).
- **M2 accepted direction/scope:** language/contracts, a minimal reference implementation, and small
  authoring examples with conformance checks using the accepted Experiment 002 scope. Bundled
  capabilities are replaceable and do not become a universal standard library. A documented working
  source checkout can suffice initially. Individual package contents and artifacts will be refined
  with the authoring slice; no detailed catalog or release format was selected. See
  [initial delivery](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/implementation/README.md#delivery-and-evidence).
- **M3 resolved through the existing model:** contexts can be ordinary Resources exposing specific
  plugin capabilities. A Pool owns the Resource, and Conduits supply its permitted access and
  dependencies. No universal Context layer or owning-context hierarchy is added. A filtered Conduit
  suffices when no separate Resource behavior/lifecycle is needed. Record identity, revisions,
  provenance and sharing remain capability contracts; exposed source records retain their ownership.
  See [memory and evidence](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/RESOURCES.md#retained-memory-and-evidence).

## Owner decisions before public distribution

- **License — accepted:** Apache-2.0 for original Conduit documentation, contracts and reference
  code. The official [LICENSE](../../LICENSE) is included; reproduced third-party material retains its
  own terms and attribution. No separate CLA or DCO requirement was selected. Any additional
  contribution procedure remains open if needed; see [license scope](../../CONTRIBUTING.md#license-and-attribution).
- **Public identities — deferred:** keep the current `shrav-k/conduit` repository location while
  protocol work continues. Organization selection/transfer, domains and permanent package naming
  remain for the owner to choose later. The accepted one-main-repository direction is unchanged;
  provisional examples and directory names do not establish permanent protocol identities.
- **Change and release authority — accepted:** the repository owner retains final authority over
  protocol direction. Collaborators propose/review changes; merges and releases use existing repository
  permissions and explicitly appointed maintainers/delegated responsibilities. Existing task
  authorizations remain valid. No fixed quorum or formal governance process is required initially.
  See [project ownership](../../CONTRIBUTING.md#project-ownership-and-publication). Public support/security
  contact details and additional contribution procedures are deferred until needed for distribution.

These are project-owner decisions, not reasons to reopen the five families or invent another runtime
authority model. The [repository proposal](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/reviews/REPOSITORY-2026-11-02.md) retains options and rationale.

Apache-2.0 was selected for permissive reuse with an explicit, scoped contributor patent grant.
The [official license text](https://www.apache.org/licenses/LICENSE-2.0.html) was checked on
2026-11-02. Licensing does not change the no-backward-compatibility rule or select a release process.

### History: explicitly later scope

Later standing: rulings 38 to 43 and 49 to 66 decide v0 and its later work; the specification binds.

## Explicitly later scope

Do not convert these into blockers for the first slice: a second complete runtime; multi-machine
support claims before validation; multi-user/differentiated Interface permissions; offline grants;
graph loops/dynamic fan-out; in-place instance-definition upgrades; marketplace or automatic
cross-language dependency resolution; automatic wake; structured asynchronous work jobs; advanced
memory sharing/publication; general automatic catalog refresh; ownership transfer; or a Pool-owned
authority store. Existing native/isolated execution choices and testing obligations remain accepted;
stronger isolation claims require their own evidence.

### History: implementation rulings I1 to I7

Superseded where changed by rulings 13 and 14 (layers), 40 and 41 (providers, graphs and policies), and 43 (machines and proof); the specification binds. I2 and I6 remain historical directions, with current proof and capture requirements in ruling 47.

## I1 — Independent execution and management

**Accepted:** separate management and execution services; shared registration/policy authority;
one designated authority for each instance's definition, confirmed settings and outcomes; backend
ownership of domain data; source-aware Interface views. Services may host many instances.
No offline grants: supervision and dispatched results survive where supported, while new managed
requests wait if current permission cannot be verified.

The first proof uses management and two executor processes on one machine with controlled failures.
Two-machine validation precedes remote-support claims. See the
[authority contract](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#shared-policy-and-instance-owned-state) and
[outage contract](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#v0-outage-admission).

## I2 — First real Agent path and supported platform

**Accepted:** after the synthetic proof, native Claude/Codex adapters precede Conduit's own Rust
model/tool loop. Native subscription-account validation is macOS-first, with a portable core and
OS adapters. The two-Claude/two-Codex account goal remains; synthetic identities do not satisfy it.
Pi is research/frontend inspiration, not the runtime dependency. See
[Agent runtime](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/AGENT.md#runtime-and-presentation).

## I3 — Plugin execution and distribution

**Accepted:** published contracts and implementation bindings may use code, existing MCP services,
graphs or combinations. MCP plus a Conduit profile is the public invocation boundary. Independent
managed surfaces retain authority/evidence obligations. Endpoint instances select/manage their
plugins; Conduits compose requirements and coordinate compatible behavior.

Requirements compose obligations; graphs compose typed operations. Composite public results derive
from declared public bindings by default, with explicit domain construction available. Reuse graph
recovery and valid child progress; preserve effects and uncertainty. Requirement contracts expose
fulfillment duties; optional conflict checks do not replace individual enforcement.

Authored code normally uses supervised instance-associated helpers; trusted built-ins may be
in-process. There is no process-per-plugin rule. Bundle first-party plugins and allow explicit local
installation; defer marketplace and automatic cross-language dependency resolution. Experiment 002
is scoped to prove executable and requirement composition together. See
[extension model](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#authoring-concepts-beneath-the-object-families),
[plugin boundaries](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#plugins-and-independently-accessible-mcp-components) and
[requirements](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/CONDUITS.md#composable-relationship-requirements).

## I4 — Local transactional authority

**Accepted:** policy is independent of Interfaces; recorded participation grants govern managed
access; issued grants do not depend on issuer ancestry. Policy applies composable requirements
with explicit targets and live ownership-based matching. Bounded issuance can supply mandatory
use restrictions. The system Pool owns separate management and access-management Resources.

The locally loadable root Interface directs first setup and explicit recovery, with verified
local-owner authority for those foundational operations. Ordinary access uses managed grants;
failed management does not reopen initialization or grant offline access.

Publish a minimal persistence plugin contract, loaded as an authority-service runtime dependency
in v0. Bundle SQLite with separate local service databases. A future Pool-owned store remains
possible; there is no distributed transaction or atomicity with external effects. See
[foundational system](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#interfaces-and-the-user-workflow),
[access](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/CONDUITS.md#authority-and-current-access), and
[persistence](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#plugins-and-independently-accessible-mcp-components).
[AUTHORIZATION](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax/AUTHORIZATION.md) retains detailed examples/proposed representations.

## I5 — Credential custody and account compatibility

**Accepted:** default Conduit-retained secrets to a replaceable OS-store implementation bundle,
initially macOS Keychain. Config/evidence hold references, not secrets; no silent plaintext fallback.
Interface-guided adapter enrollment/unlock reuses existing access and surfaces required human
interaction. Native Agent login and external secret ownership remain distinct. See
[shared authentication and custody](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/POOLS.md#shared-access-and-authentication).

## I6 — What observability captures by default

**Accepted:** extensible observation plugins/Conduits, lightweight correlated traces/diagnostics,
small health metrics and optional OpenTelemetry export through Interface inspection. Synthetic tests
record all instrumented traces at source with bounded fixture capture and visible gaps. Real full
content needs explicit recording policy; credentials stay out of ordinary logs. Authoritative
operation/recovery evidence is unsampled and independent of optional exporters. See
[observability](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#interface-observability).

## I7 — Authoring conventions and multi-plugin graphs

**Accepted:** published language-neutral contracts govern public APIs and behavior. Contracts may
be handwritten or generated, then reviewed/versioned; implementations conform. Complete templates
can bind compatible contributions from multiple plugins without a mandatory primary. Standard
templates and declared defaults resolve to explicit retained bindings, without silently merging
installed packages or falling back from an invalid explicit choice. See
[composition](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#authoring-concepts-beneath-the-object-families).

**Deferred:** concrete package, template and contract syntax. The named plugin map, uniform export
references, package-local/default scope, schema dialect and API identifiers have not been ruled on.
The [authoring packet](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax/README.md) preserves the examples for a later PR.

### History: conceptual rulings C1 to C3 and review outcomes

Superseded where changed by rulings 13 and 14 (conformance), 27 to 29 (Conduits, lifecycle), and 40 and 41 (providers, composition and results); the specification binds.

### C1 — What does conformance promise?

**Already settled:** contracts are language-neutral and authoritative; templates, finite graphs,
operations, requirements and instance behavior have public semantics. MCP is the plugin invocation
base, with a Conduit profile. Components can expose supported capabilities independently. A concrete
frontend, Rust crate layout or storage engine does not define those public semantics.

**Accepted:** conformance describes explicitly supported capabilities. Clients/plugins can support
limited surfaces; a runtime claiming portable template execution must honor public graph, control,
lifecycle and recovery semantics. Required executable behavior cannot silently become opaque.
The [canonical contract](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/CONFIGURATION.md#declared-capability-conformance) records this ruling.
Exact names and syntax remain deferred.

**Accepted evidence threshold:** the first conformance milestone requires an independently authored
client and plugin working with the reference runtime through published contracts and conformance
tests. A second complete runtime is deferred. This tests the supported extension surfaces without
claiming runtime-to-runtime portability. C1 is complete; exact profiles, syntax and fixtures remain
for the future authoring PR.

**Why now:** collaborator PR #2 makes this choice visible. Its opaque setup/runtime proposal would
lose the portability we have prioritized. [Comparison and sources](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/collaborator-proposals/REVIEW.md).

### C2 — Whose authority governs a dependent call?

**Already settled:** every managed caller needs recorded participation authority; requirements
narrow access; shared credentials are not caller grants. Resource-to-Resource calls use Conduits.
Requirements have declared applicability and enforcement duties. Reads and delivery recheck current
access, and derived knowledge retains declared source-access dependencies.

**Accepted:** a Resource serving an Agent uses its own authorized dependency connections while
preserving the authenticated originating caller and enforcing applicable restrictions on requested
effects and returned content. Autonomous work uses the Resource's own authority. Capability contracts
define the enforcement; callers do not need direct grants to every internal backing service.

For example, search may index Euphony and Demo under its own authority, while an Euphony-only Agent
cannot retrieve Demo through a query. A shared service credential or dependency grant cannot widen
that caller's permitted effects/results. This does not introduce issuer ancestry, token forwarding
or a universal serialized identity chain. The [dependent-call contract](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/CONDUITS.md#authority-across-dependent-calls)
records the boundary. C2 is complete; concrete propagation and checking mechanisms remain authoring work.

### C3 — What does retirement guarantee across connected objects?

**Already settled:** setup completion, runtime readiness and operation outcomes differ. Retry reuses
valid progress; uncertain effects are not blindly replayed. Agent deletion requires confirmed
execution end. Pool deletion requires explicit Resource retirement. A Conduit ends after its last
logical consumer/peer detaches; external effects may outlive that detach. Backing data is preserved
unless explicitly cleaned up. Incoming references alone do not cause cascading deletion.

**Accepted:** retirement stops new work, releases the instance's own activity/attachments, and permits
unregistration only when its contract's required shutdown duties are confirmed. Missing confirmation
leaves retirement incomplete. Preserve shared dependencies, other instance identities and operation
evidence. Already-dispatched external effects may outlive local shutdown; no cancellation or rollback
is inferred. Contracts define the duties without one universal lifecycle enum.

For example, retiring Linear access preserves the Agents and shared authentication Resource, while
an in-flight write keeps its recorded or uncertain outcome. Frontend detachment is distinct from
Interface retirement. The [canonical lifecycle rule](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/LIFECYCLE.md#retirement-and-unregistration)
applies alongside existing Agent/Pool checks and separate cleanup/purge contracts. C3 is complete.

## Review outcomes

Accepted details now live in the [design topic files](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/README.md#contract-map). Review documents
summarize decisions and link to their authority; they do not repeat a second specification.

| Area | Settled direction |
| --- | --- |
| Object model | Interface, Agent, Resource, Pool and Conduit; reusable templates and concrete instances. |
| Extensibility | Published contracts, code/MCP/graph-backed plugins, composable requirements, finite graphs; no mandatory primary plugin. |
| Configuration | Strict typed YAML, stable IDs, explicit creation/updates, fixed instance definitions and exact implementations, named controls for supported changes. |
| Relationships | Endpoint-managed implementations, explicit open/join, per-consumer participation, narrowing complete access paths, new identity after a flow ends. |
| Access | Independent governing requirements, recorded participation grants, current checks, no offline grants or issuer-dependent cascades. |
| Resources and Pools | One Pool owner per Resource; explicit available-template entries and provision/use-existing intent; shared service authentication. |
| Authority and recovery | Shared registration/policy; one authority per instance; root Interface setup/repair; explicit resume and honest uncertain outcomes. |
| Reference implementation | Separate management/executors, supervised instance-associated helpers, SQLite service dependencies, replaceable OS credential custody. |
| First proof | One machine, management plus two executors; composition/requirements, controlled failures and Interface evidence; native macOS Agent adapters next. |

I1–I7 remain summarized in the [implementation review](#history-implementation-rulings-i1-to-i7).
The earlier history remains in Git; removing repeated ruling prose does not withdraw accepted rules.

## History: Later amendments

The ledger's status column says what is current; these tables are kept as written.

The rows above are kept as the rulings were given. Later rulings amended some of them. In a
row that cites ruling 41, an item is a decision of the
[principles review](principles-review/README.md), C14, C20 and C21 are the changes
of its section 3, and a planner's choice is one of P1 to P20 of the
[work order](ruling-41/README.md#planners-choices), which ruling 41 also accepted:

| Ruling | What it says | Amended by |
| --- | --- | --- |
| 2 | Vocabulary stays provisional. | Replaced by ruling 16; the vocabulary was then ruled (17, 18). |
| 10 | Codex and Claude each build the whole engine. | Narrowed by ruling 14 and layer decision LD4: the two builds cover the language layer, the shared conventions and the implementation interface, and one reference engine is built separately. |
| 14, 15 | "Plugin interface", "plugins". | Vocabulary decision V7 (ruling 17): the implementation interface, and implementations. |
| 15, 17, 19 | Grants, and issuing them. | Rulings 27 to 29: grants are merged into Conduits. The access is what an open Conduit carries, and to issue is to open. |
| 24 | Decision PD1, how a package's exports are named, is held back. | Settled by rulings 25 and 27. |
| 31 | Contracts are documents of four kinds. | Five, with `Schema`, since follow-up decision F21 of the re-audit (ruling 33). |
| 31 | The implementation-defined list has twelve entries. | Fifteen. Ruling 32 added DL13. Decision DA3 of the documents audit, taken under ruling 37, added DL14 when the behaviors that the older text accepted were converted. Round 2 of the re-audit, under ruling 37, added DL15. Ruling 41 removed DL14, since cancellation is the protocol's own (item 22), which leaves fourteen. Ruling 42 (D8) added DL16, what the engine's own runner gives a process, and ruling 43 (Q2) added DL17, an engine that restarts on another machine or runs as several parts, which makes sixteen. Round 5 of the re-audit removed DL11, since the access convention already decides what it left to each engine (M69). That leaves fifteen. |
| 31 | `conduit.dev` stays a placeholder. | Replaced by ruling 33: the names come from this repository. |
| 7, 8, 9 | Placement as Conduit's own model: locations registered as Resources with facts, a default location, Conduit's own rules for choosing one. | Ruling 40, point 3b: a provider reports the machines, places instances by label and reports a lost one; what placement and loss mean for access stays Conduit's. Ruling 41, item 8, wrote it so. The engine registers a node for each machine that the machine provider reports (PLC-026), and that provider places an instance that is given no placement (PRV-015). Since round 5 of the re-audit it places only one for which an item that is run is selected, and one that runs nothing is placed on `local` (PLC-002, M22, [below](../AUDIT.md#changes-that-no-ruling-has-accepted)). A machine reported unreachable makes requests to its instances report unavailable (PLC-023), and the engine declares lost the node of one reported gone (PLC-024). The "default location", facts and `set-facts.v1` are removed. |
| 13, 14 (LD2), 23, 28 (CC11) | Setup graphs, with the static meaning of steps, bindings and dependencies, in the language. | Ruling 40, point 3c: reduced to declared dependencies; an operation built from other operations is an ordinary implementation. Ruling 41, item 1, then removed graphs and steps, with GD1 and GR1 to GR4 of ruling 23 and CC11 of ruling 28. An instance's own setup and retirement work are a setup hook and a retire hook, which its template selects in its `implementation` map (TPL-017, TPL-018). Only declared Conduits and entries keep a dependency order (CON-004, POOL-013). `graphs.yaml` is deleted, as the owner changed planner's choice P2, and [spec/README.md](../../spec/README.md#promise-files-that-rulings-removed) lists its IDs. |
| 13 | Process layout is part of each engine's runtime profile. | Ruling 40: processes are run by a runtime provider that the user chooses. Ruling 41, items 2 and 31: a runtime provider is a Resource whose type offers the core capability `runtime-provider.v1` (PRV-001), and an Agent or a Resource names the one that runs it by its standard argument `runtime`. With none, the engine's own runner runs it (PRV-006). |
| 31 | Runtimes declared in templates, with start and stop operations and handles the engine holds. | Ruling 40, point 3a: a runtime provider contract (run, stop, status). Ruling 41, item 2, wrote it in the shape of CRI. `runtimes`, held handles and `restoresRuntimes` are removed, with the runtime decisions of rounds 2 to 4 that ruling 39 accepted. The engine names each run by the instance's ID, so one run serves one instance (item 2a, PRV-007, TPL-019), and a restart is the provider's own restart policy (item 2b, PRV-010). So decision I7 and DL12 no longer leave to each engine who starts a process, over which transport it is reached, or how many instances share one. |
| 15 | "The grant is an object", in the purpose. | "The Conduit is an object", by ruling 39, point 7. |
| 31 | A domain must be named before the engine builds. | Withdrawn by ruling 38, point 5. |
| 19, 20 | Rule RR8 of the relationships record: work that a caller started stays subject to that caller's access whenever it later runs. | Later work is dropped from v0 by ruling 38, point 6: no operation arranges, through the engine, work that outlasts its invocation. ACC-019 states that instead, and IMP-006 is removed. |
| 34 | Proposed: the names belong to a version of the specification. | Accepted as ruling 35. |
| 7, 23, 28 | Location needs: types declare them (ruling 7), and every type states them, including none (ruling 23). They are requirements on an instance's placement (CC9, ruling 28). | Ruling 41, item 8a, with planner's choice P12: a placement need is a label selector, a map of label to value written inline as `nodeSelector` (`placement` until ruling 42) in the Type and in the implementation item, and matched against the labels that the machine provider reports (PLC-007, PLC-025). Requirement contracts no longer apply to placement, and a Type's `nodeSelector` is optional. |
| 8 | Name, owner and placement are standard arguments. | Ruling 41, item 31b: `runtime` is one more, for an Agent and a Resource. It names the runtime provider that runs the instance, and is fixed for life (OBJ-013, PRV-006). |
| 15 | Enforcement is opted into through the language and "plugins that select an isolation framework". | Ruling 41, item 2d, reads such plugins as a runtime provider whose contract confines its consumer, which the user picks by the standard argument `runtime` (item 31b). In v0 every restriction is mediated (ACC-027). |
| 15 | Each agent's access "can be inspected, narrowed and revoked independently of both ends", in the purpose. | Ruling 41, item 3a, removed policies, so an open Conduit is not narrowed in place. To narrow an access, an actor closes the Conduit and opens a narrower one (ACC-011). A limit placed on an open Conduit is a candidate for a later version (A66). |
| 19, 28, 29, 32, 38 | Policies. A policy on an Agent is the tool for restricting it across its Conduits (RD8, ruling 19). A policy is applied through an operation on `access` and recorded there (CC12, ruling 28). A temporary limit is a policy (CC3, ruling 29). A delegate's bound limits the policies it may apply (U7, ruling 32). `apply-policy.v1` warns when a policy makes an actor's requests to a target that the engine implements unavailable (ruling 38, point 7). | Ruling 41, item 3a: policies, `apply-policy.v1` and `remove-policy.v1` are removed from v0. Policies are a candidate access feature for a later version (A66). |
| 19, 38, 39 | Composite Conduit templates: a template that expands into several pairwise Conduits (RD4, ruling 19), which opens with a whole decision at setup (ruling 38, point 3) and an outcome for each component (ruling 39, point 5). | Ruling 41, item 5: removed. A template that needs two capabilities of one target declares two Conduits. |
| 20 | Rule OR8: a control's result states the settings it confirmed, and the engine stores exactly those. | Ruling 41, item 15: a control applies exactly what it was sent or fails, and on success the engine stores what was sent (REV-007). An implementation never applies another value in the place of one it was sent. |
| 20, 32, 37 | Retained results. Only the original caller retrieves one, checked against its access at retrieval (OD3, ruling 20). A call whose Conduit closes while it runs delivers its content only if the caller's access then permits (U6, ruling 32). A tool for retrieving one's own results needs no Conduit (Q7, ruling 32). The core export `retrieve-result.v1` (ruling 37). | Ruling 41, items 6a and 6c: an invocation permitted when it was dispatched delivers its outputs even if its Conduit closes while it runs (ACC-034). Retained results and `retrieve-result.v1` are removed: a caller that lost a result repeats the request with the same idempotency key (OPS-009). |
| 21, 38 | Rule DR5: renaming an instance is its own operation, checked against the revision (ruling 21). A rename advances the revision (ruling 38, point 12). | Ruling 41, item 10: there is no rename. A name is fixed once the object is made (OBJ-011). |
| 23 | Decisions SD1 and SD2: a short closed list of schema keywords, and five string formats. | Ruling 41, item 20, partly reversed them: JSON Schema 2020-12 with the applicator and validation vocabularies, less stated exclusions, and with `uri-reference` ([schema profile](../../spec/schema-profile.md), SCH-008). |
| 24, 38 | Rule PR2: a template whose dependency is not installed is registered, and the missing dependency is reported (ruling 24). Ruling 38, point 8, kept that report for a directory handed to an engine. | Ruling 41, item 12: such a registration, or such an installation, is rejected, with a diagnostic naming each package that is not installed (PKG-006, PKG-028). The rest of point 8 stands: an engine handed a directory installs nothing (SUB-010). |
| 24, 37, 39 | Re-exports: rule PR5 (ruling 24), choice C49 of the core-and-schemas record (ruling 37), and the decisions of rounds 2 and 4 on them that ruling 39 accepted. | Ruling 41, item 9: removed from v0. `exports` lists only the package's own documents (MAN-007). |
| 27 | Decision NE: a package's address is by default also where it is fetched from. | Ruling 41, item 21: every engine fetches `https:` and `file:` URLs, and another scheme only through a package-source provider. The address is not a source (NAM-012). |
| 28 | Decision CS6: one marker, `optional: true`, for inputs, steps, declared Conduits and outputs. | Ruling 41, item 16a, narrowed it: an Operation contract's input or output is optional where its schema's `required` does not list it, as the protocol's tool definitions write it (CTR-002). Template inputs, a Requirement's inputs, declared Conduits and entries keep `optional: true`. There are no steps. |
| 29, 30 | An object exists until it is unregistered (ruling 29), and an instance is unregistered only once its shutdown is confirmed (OL3, ruling 30). | Ruling 41, item 17: a retired instance is removed, with no request, once its shutdown is confirmed, and a Conduit when it is closed (LIF-035). `unregister.v1` stays for templates, and for an instance whose node was declared lost (LIF-017). Since round 5 of the re-audit it removes any retired instance, as an assertion that its shutdown is settled another way (LIF-017, LIF-036, M41, [below](../AUDIT.md#changes-that-no-ruling-has-accepted)). |
| 31 | Decisions D3 and D5: value references from five sources, and an optional `retire` graph. | Ruling 41, item 1: removed with graphs. An instance's own retirement work is its retire hook, a core operation that its template selects in its `implementation` map (TPL-018). |
| 31 | DL9: whether a request to set up, retire or change a busy instance is queued or rejected is each engine's. | Ruling 41, item 23: it reports conflict at once (REV-014). DL9 keeps only the order of failed checks. |
| 31, 32, 33 | Decision I2: Conduit's keys in a request's `_meta`, the interface version among them. | Ruling 41, item 18: the keys travel as one extension of the Model Context Protocol, with the setting `revision: v0`, and the key for the interface version goes. A request to an implementation that does not declare the extension reports unavailable (IFC-037). |
| 32, 37 | A location's facts are stored configuration, changed only by a control, and a requirement on placement is a schema over them (G5, ruling 32). The owner's change of ruling 37: a control that sets a location's facts. The shared-host requirement is checked when the Conduit is opened and on each request (G13, ruling 32). | Ruling 41, items 8 and 8b: facts, `set-facts.v1` and `location-settings.v1` are removed, and a node's labels are what the machine provider reports. The shared-host requirement compares the hostname labels of the two ends, and is decided once, when the Conduit is instantiated (PLC-013). |
| 32 | Decision U1: each request's `_meta` names the Conduit and carries every restriction that applies. | Planner's choice P20 of ruling 41: a setup or retire hook is dispatched through no Conduit, for the caller of the setup or the retirement. It carries that invocation's filters and no Conduit (TPL-017, TPL-018). |
| 32 | Decision U5: when setup or retirement runs again, a step whose outcome is unknown is not dispatched again unless a new attempt is allowed, a runtime the instance holds is not started again, and the rest is DL1. | Ruling 41, item 1c, with planner's choices P4 and P20: a further setup leaves what is already done as it is, asks for a run only where the provider does not report one running, and dispatches a hook again only as its idempotency key allows (OPS-021, OPS-028). DL1 keeps only the order of a Pool's Resources. |
| 32, 33 | Re-audit decisions that rest on what ruling 41 removed: Q3, G4 and the `retire` graph of G12 (ruling 32); F5's component segment and its check on a rename, F8, F15's default placement, F19 and F25 (ruling 33). | Ruling 41, items 1, 5, 8 and 10: graphs and steps, composite Conduit templates, the "default location" and rename are removed, so these no longer apply as written. Q3's rule for an outcome holds of a setup and a retirement, counted by their parts (OPS-022). What F8 and G12 said of the `retire` graph holds of the retire hook: it carries no mark of model work, and it is not dispatched for an instance for which no setup was ever dispatched (TPL-018). An entry that no input places is placed by the machine provider (PLC-002), and since round 5 of the re-audit only where an item that is run is selected for it; one that runs nothing is placed on `local` (PLC-002, M22, [below](../AUDIT.md#changes-that-no-ruling-has-accepted)). |
| 37 | Choices C43 and C53 of the core-and-schemas record: the manifest is `conduit.yaml` or `conduit.json`, and `.yml` files are not read. | Ruling 41, change C14: `.yml` files are read and `conduit.yml` is accepted, and a directory with two manifest names is rejected (MAN-001, MAN-002). |
| 37 | Choice C15 of the core-and-schemas record: a revision is a whole number from 0. | Ruling 41, change C21: a revision is an opaque non-empty string, compared only for equality, as an HTTP entity tag is (REV-001). |
| 37 | Choices C16 and C38 of the core-and-schemas record: only the `message` of a diagnostic and the top-level names of an inspection result are fixed, and the rest is settled with the conformance vectors. | Ruling 41, item 24: a diagnostic is SARIF 2.1.0's `result`, reduced, whose `ruleId` is a promise's ID (24a). A status reports `conditions` in the shape of Kubernetes `metav1.Condition` (24b). A record has the core schema `record.v1`, a CloudEvent (24c). The rest of an inspection result is still settled with the vectors. |
| 38 | Point 1: a secret is referred to and never written as a literal, which needs a secret type in the core package. | Ruling 41, item 19: no core type. A core capability, `secret-provider.v1`, is offered by each store's own Type, which has a schema for its locator (SEC-002, SEC-005). Ruling 42 gave the capability that name in place of `secret-reveal.v1` (vocabulary audit, N1). |
| 38 | Point 11: a result that carries none of its operation's declared outputs is an error result, to be looked at again. | Ruling 41, item 6b: `isError` is true only where the request ended with a reason or its outcome is failed or unknown (IFC-025). A repeat of an invocation still running reports conflict (OPS-032). |
| 39 | Points 1 to 4: the secret type is not decided. Only a Resource reads a secret, and only the root Interface may open a Conduit that reads one. The engine reads a secret through the instance's own Conduit and sets it in a process's environment at start, for a server and for an Agent's own credential. | Ruling 41 settled point 1 with section 4 of the principles review ([secrets.yaml](../../spec/promises/secrets.yaml)). Points 3 and 4 stand: before each run that it asks a runtime provider for, the engine reads each secret that the item needs (items 2c and 4a, SEC-010). Point 2 is restated: no actor's request for `get-secret.v1` is dispatched (item 4b, SEC-007), and the root Interface may open a Conduit that lists it for a Resource or an Agent (SEC-008). |
| 39 | Requirement composition: GP1 of the documents audit (decision DA3), and round-2 decisions R2-G3 and 10, accepted with every logged decision. | Ruling 41, item 7: removed. A Requirement contract has no `constituents`, and several requirements are applied side by side in a Conduit template's `requirements` map. CTR-010 and CTR-011 are removed. |
| 39 | `obliges`: round-2 decisions 8 and 10, part of decision 7, and the `obliges` half of GP15 (decision DA3). | Ruling 41, item 14: removed. A filter binds only the target, whose implementations apply it (CON-022). IFC-034 is removed. |
| 39 | `narrows` on a Requirement contract, and CON-030, accepted with the specification. | Ruling 41, change C20: `narrows` is dropped. A requirement of the core package is applied by the engine and never sent, and any other is a filter that the target's implementations apply (CTR-005, CON-018). CON-030 is removed. |
| 39 | Coordinator decisions 4B-4, R2-S6 and R1-2.6/R2-C4: a project's contracts are kept as an unpublished package, named by a digest of its documents and written `<digest>#<name>`. | Ruling 41, item 11: a project that holds contracts states `address`, and outside its own documents they are `address#name` (MAN-015, SUB-008). The digest still identifies one submission (planner's choice P11). |
| 41 (item 17) | A request through a closed Conduit reports invalid, as the cost line of item 17 says. | Refused, by ruling 42 (D6). |
| 41 (item 4c) | Every read of a secret belongs to a request in progress. | Also a read again for the caller of the setup, where a remote connection's values were lost, by ruling 42 (D12). |
| 41 (section 4 of the review, line 18) | What else a run receives is the provider's. | For the engine's own runner, six variables and a list set on the runner, by ruling 42 (D8). |
| 39 (decision 5 of round 3) | `retire-with-resources.v1` stops at the first Resource whose retirement does not complete. | It goes on: only the Resources that the order puts after the one it could not retire wait, by ruling 42 (R5). |
| 17, 18 (V4), 33, 41 | Location, origin, operation identity and the names that ruling 41's work order coined. | The names of the [vocabulary audit](vocabulary-audit/README.md), by ruling 42. |
| 42 (D17) | `run.v1` carries a credential as a file that the runtime provider may refresh in place. | In this version a credential lasts as long as its instance and nothing renews it (DL13), as the [multi-machine proposal](multi-machine/README.md) that ruling 43 accepted has it; renewal on the run's machine is v1's (A65). Ruling 43's verification therefore moved the leave to replace a file in place out of PRV-022's `then` into its observable (MX-21), a [choice](ruling-43/README.md#choices-of-the-verification) that the owner can strike. |
| 41 (item 16a) | An Operation contract's `inputSchema` and `outputSchema` have a root limited to `type: object`, `properties`, `required` and `additionalProperties` (CTR-012). | Ruling 44 (D14 of round 6): the schemas of an Operation contract that is not a control may hold any keyword of the schema profile at their root beside `type: object`, as the published protocol's tool definition allows at the baseline revision; the inputs and outputs are still the members of the root's `properties`. A control keeps the limited root, which its partial updates need (REV-006), and SCH-008's ban on a `default` under a conditional keyword stands (CTR-012). The rest of item 16a stands. |
| 37 (choice C42 of the core-and-schemas record) | The default implementation of an operation is stated by each Operation contract. | Ruling 44 (D06 of round 6, as revised after the ruling): a package's own Operation contract states no default implementation, and the engine implements the operations of the core package by rule, with no mapping. Applied in CTR-002, TPL-003 and TPL-004 ([log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-d06-d12.md)). |

Ruling 44 does not change X2, the owner's own change in ruling 37 that an actor's request names
objects by reference only, nor LIF-031: as revised after the ruling, D12 of round 6 keeps names
as the only address and gives the lifecycle actions an optional expected ID instead (LIF-037). The record's table of
[earlier rulings this changes](ruling-44/README.md#earlier-rulings-this-changes) cites
ruling 39 for C42 and X2; ruling 39 accepted them again with every logged decision, and ruling
37 accepted them first.

Decisions of the coordinator, taken under ruling 37, also changed some of what a ruling had
accepted. The owner accepted each of the six below in ruling 38 (points 10 and 12):

| Ruling | What it says | Changed by the coordinator, and accepted by ruling 38 |
| --- | --- | --- |
| 24 | Decision PD5: two conformance levels. | Three, with Implementation, since row GP16 of the [documents audit](docs-audit/GAPS.md). |
| 31 | Decision D4: a runtime may be marked `optional: true`. | A runtime carries no such mark; a step that starts it can be optional. [Round 2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2/DECISIONS.md). No longer applies after ruling 41: runtimes and steps are removed (items 1 and 2). |
| 32 | An operation identity is scoped to one caller and one target. | And to the instance that sent the request (OPS-008). Round 2, finding R3-3. Since ruling 41 the idempotency key of a setup or retire hook is scoped to the instance alone (OPS-028, planner's choice P20). |
| 32 | Decision Q5: the order of reasons is refused, invalid, conflict, unavailable. | Where the decision on access cannot be made, unavailable stands in the place of refused (OPS-023). [Round 3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-3/DECISIONS.md), finding 3A-1. |
| 37 | The owner's change X1: a control's `changes` is a list of pointers. | A map from an input's name to a pointer. Round 2, finding R2-G2. |
| 37 | Requests name objects by reference only. | A step and an onward request may hold the resolved form. Round 3, finding 3B-2. After ruling 41 there are no steps (item 1); an onward request still may (LIF-031). |

### History: later rulings 45 to 67 and their recorded effects

Rows below add the later records' stated changes or settlements; a confirmation is not inferred to amend a ruling. The two explicit additions for ruling 43 are also preserved here.

| Later ruling | Earlier ruling or open point | Recorded effect |
| --- | --- | --- |
| [43](ruling-43/README.md) | 40 point 3e; 41 item 4c; 42 D5 and D20 | Gates also bind a server the engine does not run; secrets read again before restarted runs; machine questions ruled. |
| [45](ruling-45/README.md) | 9, 44; A43, A63, A70, A71, A72 | Experiments placed in the existing program, the strict loader first, pre-registration, and the roadmap shown; no change to DA11. |
| [46](ruling-46/README.md) | 44 A68; 45 open S2 to S9 substrate; 37 and 38 delegation | First build is one core and shell; S2 to S9 run on the same codebase; delegation extends to build findings. |
| [47](ruling-47/README.md) | 40 packages layout | Rust packages remain in the main repository through validation; official integrations move to separate repositories after publication is approved. |
| [48](ruling-48/README.md) | 46 review; roles and experiment program | Reach flags, patterns and a fresh Codex author; coordinator as admin, Codex workers, A77 and A78. |
| [49](ruling-49/README.md) | 48 point 2 b; IFC-007, IFC-016 | A person signs in at root in v0; account checked; URL elicitation becomes language. |
| [50](ruling-50/README.md) | 49 sign-in pending actions; PRV-013 to PRV-015 | Enrollment to be designed; generic pending actions; person confirms admission. |
| [51](ruling-51/README.md) | 50 enrollment and pending actions | Managed and peer modes, ten networking decisions, generic pending actions and trust domain. |
| [52](ruling-52/README.md) | 50 enrollment design; 4-03, 4-08; measurement plan | Two-tier refusal and explicit descriptions; SUB-010 retained; enrollment decision 5 remains open; measurement choices accepted. |
| [53](ruling-53/README.md) | simplification decisions 1, 2 and 6 | Five additions, explicit plan/apply, install writes pins, reach flags and qualified names accepted; three points need discussion. |
| [54](ruling-54/README.md) | Fleet investigation; ACC-030 | Agents stay in their owning family; grants revoked individually with review and deliberate keep. |
| [55](ruling-55/README.md) | 8, 18; wording of 31, 35, 37; 53 points 1 and 2; A21 | A6 amends inline template/instance and optional frame; ruling 19 kept. Short form is language, command approves; old inconsistencies dropped. |
| [56](ruling-56/README.md) | 50, 51 pending-action result; 52 point 5; 55 B | Provider chooses verification under engine minimum; evidence at root; answer replaces outcome for pending actions. |
| [57](ruling-57/README.md) | 39 point 11; archive and research editing rule | Prohibition on editing historical documents revoked; relevant Conduit information preserved. |
| [58](ruling-58/README.md) | 36 retained vocabulary sources; 39 point 6 record notes and assessment; 46 point 4 logs; 48 work-order records | The accepted [plan’s amendment list](condensation/plan.md#rulings) changes the retained sources and record forms; SIMPLIFICATION.md stays. Point 1 accepts pre-57 choices with the two exceptions; point 7 replaces the old build assessment. |
| [59](ruling-59/README.md) | 30 OL2; 53 point 6; ACC-024 | A1 changes one authority for Agents, A3 supplies root standing Conduits and the closing exception; bounds keep resolved IDs. |
| [60](ruling-60/README.md) | 30 OL2; 54, 59 and brief naming; 59 A5 | The family is Fleet; project keeps its meaning; Fleet permits and runs nothing. |
| [61](ruling-61/README.md) | 44 point 2 for Conduits; 54 review; 48 A78 | B8 adds make.v1; B9 and B10 broaden review and constrain keeping; B12 changes A78 trigger and dispatch. B3 amends no ruling. |
| [62](ruling-62/README.md) | 51 peer root; C6 to C12 | C1 to C5 accepted; peer root template names admitted Interface; remaining C points deferred until after rerun. |
| [63](ruling-63/README.md) | 45 counting before run; 53 point 5; short form and packages | Pins and bare commands excluded from author lines, other counts beside; flow documented; packages ship standard Conduit templates. |
| [64](ruling-64/README.md) | 47 organization; repository directory | PR 14 merge, name research, free CI, rename reference-model to engine, organization items wait. |
| [65](ruling-65/README.md) | 47 point 1; 64 point 5 | ConduitLang public name, conduitlang identifiers, language stays Conduit; account steps wait for owner. |
| [66](ruling-66/README.md) | 49 point 2; 50; 61 B9 pending action; ACC-033 | Point 3 raises pending actions at scoped Interfaces too; reconcile limited by sender; Fleet holdings retained; no Scope object or grouping shorthand. |
| [67](ruling-67/README.md) | 58 point 2 | Ledger leaves AUDIT, carries owner words and statuses; IN_FORCE and records' status lines added; other points stand. |

## History: round-5 changes

The following passage is kept as written at C0. Ruling 58, point 1, later accepted these changes, including the integration choice and verification, except its stated exceptions; V2-15 had already been reversed by ruling 44.

#### Changes that no ruling has accepted

[Round 5](reaudit/round-5/README.md) of the re-audit, under ruling 37, also changed
what rulings had accepted. No ruling has accepted these changes, and the owner can strike each
([who decides what](../AUDIT.md#who-decides-what)). The three below change what the tables above give.
Others that come close to a ruling or to a decision of the coordinator are among the
[decisions the owner may want to read first](reaudit/round-5/DECISIONS.md#for-the-owner).

| Ruling | What it says | Changed by round 5 |
| --- | --- | --- |
| 41 (item 8c) | The machine provider places an instance that is given no placement (PRV-015). | M22: an Agent or a Resource for which no item that is run is selected is placed on `local`, and no machine provider is asked (PLC-002). Such an instance is not checked against its type's and its items' `nodeSelector`, since nothing of it runs (PLC-007), as the coordinator settled V6 of the verification. |
| 41 (item 17) | `unregister.v1` removes an instance only where its node was declared lost (LIF-017). | M41: it removes any retired instance (LIF-017). Following from it, the integration's [choice](reaudit/round-5/log-fixes.md#the-choice-of-this-pass) has PLC-023 not hold back the stop that LIF-036 asks for before the instance is removed, so the stop is asked for even while the machine is reported unreachable or omitted. |
| 31 | The implementation-defined list, with DL11, on Conduits during setup. | M69: DL11 is removed, since ACC-003 and ACC-036 decide what it left to each engine; its ID is not reused. The row for ruling 31 in the first table gives the count. |

Ruling 44 reversed one of round 5's decisions. V2-15 of the
[verification of its fixes](reaudit/round-5/log-fixes.md#verification) read the
protocol's error -32021, for a client capability that the call did not declare, as failed,
since it took the protocol to have a server answer so before acting. The protocol says
"processing ... requires", and nothing of the order of effects. Round 6's finding 6-1-01 reads
that error as any other error, failed for an operation that states no effect, is not a control
and starts no model work, as IFC-033 tests, and unknown for any other, except where Conduit's
own contract binds the server to have done nothing: the error that names Conduit's extension
(IFC-008), and an implementation that carries Conduit's keys, which answers so only where it
has performed nothing for the call (IFC-047). IFC-007, IFC-010, IFC-045, IFC-047, added, and
OPS-007 changed together.

### History: other round-5 decisions near earlier rulings

Accepted by ruling 58, point 1. The following list is verbatim from round 5's “For the owner”; M22, M41 and M69 are also in the table above. M107 describes the standing before that later acceptance.

- M13 narrows the coordinator's decision N4 after ruling 43: a `CONDUIT_URL` given as the run
  sees it keeps the whole address, not only the host.
- M22 places on `local` an instance that runs nothing. Decision 8c of the principles review,
  which ruling 41 accepted, has the machine provider place instances, and covers what runs.
- M41 lets `unregister.v1` remove any retired instance, not only one whose node was declared
  lost, as decision 17 of the principles review keeps a shutdown pending until it is settled.
- M47 gives `https` to every run that may outlive the engine. A run under a container provider
  on one machine then needs the certificate checked against `ca.crt`, which it mostly needs
  already, since it reaches the host at a name that is not loopback.
- M59 passes a plain server's `content` to the consumer unchanged.
- M66 strikes half of choice 8 of ruling 43's verification: the engine no longer authenticates
  by mutual TLS to an authorization server.
- M69 removes DL11 from the implementation-defined list.
- M107 reads ruling 42's "struck none" as leaving the coordinator's 43 decisions with the
  standing of decisions the owner can strike, not as accepting them.

## History: record-specific ID prefixes

The records of the rulings number their own decisions, and some reuse a letter. An identifier
from a record is therefore written with the record's name, as in "re-audit decision G1".
The rows of the ruling log are not rewritten, so in the rows of rulings 42 and 43, and wherever
an identifier follows the number of one of them, as in "ruling 42 (D6)" or "ruling 43 (Q4)", a
D, R or Q is one of the open questions after the rewrite or of the multi-machine proposal, in
the table below. An M is round 5's, as is a V or a V2- written with round 5 or its
verification, as in "V6 of the verification of round 5's fixes". In the row of ruling 44, and
wherever a D or an E is written with round 6, as in "D07 of round 6", it is one of round 6's
design critiques or evidence cases, in the last row.

| Prefix | Meaning | Record |
| --- | --- | --- |
| **PL**, **IN** | Placement and instantiation decisions, written with no hyphen: PL1 to PL8 and IN1 to IN10. With a hyphen, [**PL-1** to **PL-10**](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/placement/README.md#promises-this-adds) are something else: the earlier wording of the placement rules, which are now the promises `PLC-001` onward | [placement](placement/README.md), [instantiation](instantiation/README.md) |
| **NS** | Decisions on the form of the specification, NS1 to NS4 | [normative-surface](normative-surface/README.md) |
| **V** | Vocabulary decisions, V1 to V8. The removed [vocabulary map](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md) also numbered its topics V01 to V37. In a sentence on round 5 of the re-audit, a V is the verification's, in the last row | [vocabulary](vocabulary/README.md) |
| **RD**, **RR** | Decisions and rules on relationships and access | [relationships](relationships/README.md) |
| **OD**, **OR** | Decisions and rules on operations, outcomes and revisions | [operations](operations/README.md) |
| **DD**, **DR** | Decisions and rules of the declarations batch | [declarations](declarations/README.md) |
| **GD**, **SD**, **GR**, **SR** | Decisions and rules on graphs and on the schema profile | [graphs-and-schemas](graphs-and-schemas/README.md) |
| **PD**, **PR** | Decisions and rules on packages, first start and conformance | [packages-and-start](packages-and-start/README.md) |
| **NA** to **NF** | Naming decisions | [naming](naming/README.md) |
| **CS**, **CC** | Consolidation findings, on the shape of documents and on concepts | [consolidation](consolidation/README.md) |
| **OL**, **DO**, **RC**, **NI** | Decisions on the four remaining shared conventions | [conventions](conventions/README.md) |
| **D**, **I**, **X** | Decisions on the shape of documents, on the interface, and on the name the owner controls | [schemas-and-interface](schemas-and-interface/README.md) |
| **DL** | An item of the [implementation-defined list](../../spec/implementation-defined.yaml), DL1 onward | The specification |
| **U**, **Q**, **A**, **G**, **W** | Re-audit decisions, by group. **F** is a follow-up decision | [reaudit](reaudit/README.md) |
| **C**, **N**, **P**, **D**, **X** | Choices, names, open points, disagreements and accepted changes of the core package and schema batch | [core-and-schemas](core-and-schemas/README.md) |
| **DA** | Decisions of the audit of the documents outside `spec/` | [docs-audit](docs-audit/README.md) |
| Items 1 to 31, **C** | The decisions of the principles review, which ruling 41 accepted: items 1 to 31, with lettered lines such as 3a, and its candidates C1 to C58. C14, C20 and C21 are its changes to choices that rulings had accepted | [principles-review](principles-review/README.md) |
| **P** | The planner's choices P1 to P20 for the rewrite under ruling 41, which the owner accepted with P2 changed | [ruling-41 work order](ruling-41/README.md#planners-choices) |
| **D**, **R** | The questions that the rewrite under ruling 41 left open, D1 to D20 and R1 to R28, numbered as in [its integration log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-integration.md#what-remains) | [open questions after the rewrite](ruling-41/open-decisions.md) |
| **Q** | The five smaller questions on work across machines, Q1 to Q5 | [multi-machine](multi-machine/README.md) |
| **M**; **V**, **V2-** | Round 5 of the re-audit: its items M01 to M111, into which the coordinator merged the readers' findings, and the findings of the verification of its fixes, V1 to V13 and V2-01 to V2-16. A V here is never a vocabulary decision, and M is not the M ruling set of the archived reviews | [round 5's decisions](reaudit/round-5/DECISIONS.md), [its fix log](reaudit/round-5/log-fixes.md#verification) |
| **6-1-**; **D**, **E** | Round 6 of the re-audit, the owner's cold read by Codex: its findings 6-1-01 to 6-1-06, its design critiques D01 to D22 and its evidence cases E01 to E20, always written with two digits and with round 6. A D here is never a disagreement of the plan audit, and an E never one of its probes | [round 6's decisions](reaudit/round-6/DECISIONS.md), [its report](reaudit/round-6/design-critique.md) |
| **X** | The five experiments that pull request 10 proposed, X1 to X5, always written with "of pull request 10", as in "X3 of pull request 10". They are not the X of the two records above | [ruling 45](ruling-45/README.md) |
| **H** | The hypotheses H1 to H10 of the experiment across two machines (A74) | [multi-machine](multi-machine/README.md#11-an-experiment-across-two-machines) |

A promise has an identifier of letters, a hyphen and three digits, such as `CON-001`. The
letters name its file; [spec/README.md](../../spec/README.md#contents) lists the files. None of
these sets reuses the earlier R, I, C or M ruling sets of the archived reviews.

Additional historical prefix destinations (C0 pins preserve the pre-condensation records):

| Prefix | Record |
| --- | --- |
| R1-, R2-, R3- (round 2) | [Last version](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2) |
| 3A to 3D | [Last version](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-3) |
| 4A to 4D | [Last version](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4) |
| D, R (ruling 41 integration) | [Last version](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-integration.md) |
| P (planner) | [Last version](ruling-41/README.md#planners-choices) |

## Appendix: MX-21 of ruling 43

Source: [ruling 43, verification](ruling-43/README.md#choices-of-the-verification).

Added in round 5 (M109), which found it missing here: **PRV-022 no longer gives leave to replace a file of `identity` in place** (MX-21). In this version a credential lasts as long as its instance (DL13), as the proposal has it, so the leave that ruling 42's D17 gave is left, in PRV-022's observable, for renewal on the run's machine in v1 (A65).

| Finding | Held | Change |
| --- | --- | --- |
| MX-21: as C10; PRV-022 still lets a file be replaced in place | Yes | As C10. PRV-022's leave to replace a file's content in place moved from its `then` to its observable, as what this version does not do; the row `identity` no longer grants it |
