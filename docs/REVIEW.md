# Current review

**Superseded as a work queue by the [plan audit](AUDIT.md).** The plan audit is the one tracker:
it holds the state of the work, the owner's rulings and the open action items. Accepted behavior
is in the [specification](../spec/README.md), which alone binds. Nothing here binds.

The sections below are kept as they were written, for the links into them. These statements in
them no longer hold:

- that this is the only active review entry point, and that a new question returns here;
- that accepted rules live in the protocol reference, which is superseded;
- that the vocabulary pass comes last, and that only five names are settled. The vocabulary is
  ruled, and the terms in force are in [spec/definitions.md](../spec/definitions.md);
- that schemas and the representation of documents are a later pass. They are written, under
  [spec/schemas/](../spec/schemas/README.md).

The [completed authoring record](archive/reviews/AUTHORING-2026-11-04.md) preserves the earlier
rulings and their rationale. No Conduit engine exists.

## Plan audit, 2026-11-05

The owner set a new direction on 2026-11-05. The [plan audit](AUDIT.md) records the rulings, the
priorities, the action items and the experiment program, and it governs wherever the sections below
differ. In short: the language is the core; current work is the normative surface, a promise
register, schemas, a Rust reference model and conformance vectors; two independent engine builds
follow once the [readiness gate](implementation/INDEPENDENT_BUILDS.md#readiness-gate) passes; and
the vocabulary pass comes last. Known defects in the accepted text are in the
[defect register](LANGUAGE_DEFECTS.md).

**Owner ruling, 2026-11-05 — exploratory code:** experiment code may be written and run under
[experiments](../experiments/README.md) whenever it is useful, including solely to prove out an
idea, before the vocabulary and schema work below. It must be written in Rust, reflecting the
eventual runtime. This lifts the earlier prohibition for exploratory code. A Rust
reference model and two independent engine builds are planned in the plan audit. Results remain
evidence rather than accepted behavior.

## Future vocabulary PR

By the 2026-11-05 rulings this pass comes last, after the language is concrete and proven, and it
may merge concepts as well as rename them.

Only **Interface, Agent, Resource, Pool and Conduit** are settled names. All other Conduit-owned terms
and field spellings remain provisional, including spellings used while accepting a structure.

Use the [vocabulary map](VOCABULARY.md) to work term by term. It links stable documentation IDs to
accepted meanings, worked examples and affected surfaces; the IDs are not protocol identifiers.
Review distinctions before names. Preserve external-standard terminology, historical source language
and attributed dissent. No replacement name is selected by this audit.

The [naming research](research/vocabulary/README.md) now supplies cross-system precedents
and the reviewer's simplification pass. Its proposed discussion order begins with package,
contract and plugin/implementation boundaries, then execution and relationships. This is preparation
for owner rulings; it adopts no names, adds no object families and reopens no accepted behavior.

After a ruling, update canonical text and active examples together, retain stable anchors, record the
old/new spelling in the map, and run the documentation checks. Naming must not silently change behavior.

## Later authoring and conformance pass

The [authoring packet](proposals/authoring-syntax/README.md) contains proposals within accepted behavior.
It is not a released schema or runnable tutorial. Remaining representation work spans these areas:

| Area | Accepted source | Remaining deliverable |
| --- | --- | --- |
| Packages, exports and types | [Model](protocol/MODEL.md#authoring-concepts-beneath-the-object-families), [configuration](protocol/CONFIGURATION.md) | Manifest/schema organization, identifiers, concrete references and artifact layout. |
| Templates, graphs and runtimes | [Lifecycle](protocol/LIFECYCLE.md), [Pools](protocol/POOLS.md) | Complete typed declarations and operation/driver schemas preserving accepted ordering, defaults and recovery. |
| Managed invocation and access | [Conduits](protocol/CONDUITS.md), [controls](protocol/LIFECYCLE.md#template-defined-configuration-changes) | MCP profile fields, evidence and interaction/result representations. |
| Observation and conformance | [Observation](protocol/MODEL.md#interface-observability), [capability claims](protocol/CONFIGURATION.md#declared-capability-conformance) | Source/cursor/event schemas and independently consumable validation and behavior fixtures. |

Do not reopen accepted graph forms, source categories, type scope, defaults or authority rules merely
because the final fields are unnamed. Concrete schemas still need review against these guarantees;
libraries and parser behavior do not define the language by accident.

## Refinements to carry into concrete examples

- Partial control confirmation must leave the complete stored configuration valid, with other effects'
  uncertainty visible; a confirmed subset does not prove the whole request succeeded.
- Graph recovery/cancellation retain actual effects and required lifecycle duties. Definition
  replacement, explicit retry and transport reconnection remain different actions.
- Initial required preparation, later independent work and per-consumer participation have different
  readiness implications. Ready Pool Resources may be usable while the Pool is incomplete.
- Independent MCP access verifies current authority and records outcomes. Metadata, references and
  existing provider credentials do not supply grants.
- Root Interface repair and credential interaction preserve identities/progress, verify the local
  owner where required, and use supported adapter flows without exposing secrets.

These are acceptance obligations, not unresolved alternative models. The
[runtime experiment plan](../experiments/002-runtime-plugins/README.md) carries planned cases;
its expected outcomes are not executed evidence.

## Implementation details, not new conceptual rulings

Crate/module boundaries, helper grouping, initial transport, SQLite tables/durability settings,
retry budgets, parser/tool choices and website generation remain engineering proposals in the
[implementation packet](implementation/README.md). Elevate a choice when it changes a public guarantee,
support promise or scope. Schema and conformance work precede claims of protocol implementation.

## Owner decisions before public distribution

Apache-2.0, one main repository with explicit public boundaries, owner-led authority and no
backward-compatibility promise remain accepted. See [publication policy](../CONTRIBUTING.md#project-ownership-and-publication)
and [release compatibility](protocol/CONFIGURATION.md#release-compatibility).

Permanent organization/domain/package naming, public support/security contacts and any additional
contribution procedure remain deferred until needed. They do not block vocabulary or schema work.

## Explicitly later scope

Do not turn these into blockers for the first slice: a second complete runtime; multi-machine
support claims before validation; multi-user/differentiated Interface permissions; offline grants;
authored graph loops/dynamic fan-out; in-place definition upgrades; marketplace or automatic
cross-language dependency resolution; automatic wake; structured asynchronous work jobs; advanced
memory sharing/publication; general automatic catalog refresh; ownership transfer; or a Pool-owned
authority store. Native/isolated execution choices remain accepted; stronger isolation claims require
their own evidence.

## Behavior and structure merge checkpoint

Completed and merged in [PR #3](https://github.com/shrav-k/conduit/pull/3), commit `82e87cd`.
The [frozen record](archive/reviews/AUTHORING-2026-11-04.md#behavior-and-structure-merge-checkpoint)
contains the closure checklist. The paper fixtures were migrated to accepted bindings, named runtimes
and explicit startup operations. The merge did not approve every spelling, implement a runtime or
claim conformance. This heading remains a navigation target, not a pending approval gate.

<a id="migration-review-rulings"></a>

## Closed and preserved

- [Completed authoring and migration rulings](archive/reviews/AUTHORING-2026-11-04.md): full pre-audit
  record, including M1–M3, owner rulings, authoring decisions and source references.
- [Conceptual review](archive/reviews/CONCEPTUAL.md) and [implementation review](archive/reviews/IMPLEMENTATION.md):
  earlier accepted boundaries and intentionally deferred syntax.
- [Collaborator comparison](archive/collaborator-proposals/REVIEW.md): earlier PRs, adoption decisions
  and attributed dissent.
- [Initial migration](archive/migrations/2026-11-protocol-framing.md) and
  [vocabulary-preparation audit](archive/migrations/2026-11-vocabulary-audit.md): preservation and
  navigation accounting, not competing specifications.

After a future ruling, update its canonical topic and affected examples, then remove its pending
status here. Preserve rationale and intentional reversals in the completed record.
