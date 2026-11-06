# Conduit specification

Status: draft, begun 2026-11-05. Incomplete, and not a release. Every area has been written as
promises, and the core package, the JSON Schemas of documents and the manifest's format have
been written as documents; the parts still to come are listed under [Contents](#contents).

Some parts say that the owner accepted them, as [the core package](core/README.md) and
[the schemas](schemas/README.md) do. The two words do not conflict, and "accepted" covers less
than the part as it now stands. The owner accepted each part as it stood at the ruling that
its status line or the comment at the head of its file names, by the number of the ruling in
the [ruling log](../docs/AUDIT.md#ruling-log). What the documents audit and rounds 2 to 4 of
the re-audit changed since then rests on the decisions of the session that coordinates the
re-audit, which ruling 37 lets settle a reader's finding without a ruling on each
([who decides what](../docs/AUDIT.md#who-decides-what-since-ruling-37)). Those decisions are
logged, in [GAPS.md](../docs/proposals/docs-audit/GAPS.md) and in the decisions of
[round 2](../docs/proposals/reaudit/round-2/DECISIONS.md), of
[round 3](../docs/proposals/reaudit/round-3/DECISIONS.md) and of
[round 4](../docs/proposals/reaudit/round-4/DECISIONS.md). The owner has not ruled on them and
can strike any of them, so no text that they changed or added is a choice the owner accepted,
with the exceptions that [ruling 38](../docs/proposals/ruling-38/README.md) of 2026-11-06
makes. That ruling accepted some of those decisions: a retired consumer's Conduits staying
open for its own retirement, the resolved form in a step and in an onward request, the
interface following the Model Context Protocol on errors, and the eight changes of its point
12. It also made four changes of its own: later work is not part of this version, the
result of `apply-policy.v1` warns of targets that cannot apply the policy, a directory handed
to an engine installs no package, and the opening of a composite Conduit at setup is
worded anew.
The comment at the head of each file says which pass changed which promise. The specification
as a whole is a draft until it has conformance vectors, a round of readers that finds nothing
unsafe in it, and a reference model that agrees with it (ruling 38, point 4).

The specification alone binds. Nothing under [docs/protocol](../docs/protocol/README.md) binds:
it is the design as it stood before the specification, superseded, and kept as source material
for the type contracts that are still to be written. A behavior that is missing from `spec/` is
a gap in `spec/`, and no older document supplies it.

## What this is

Conduit is a configuration language for the relationships between independent agents and the
things they share. This directory is its specification: the documents that bind every engine.
It has three layers, and every promise says in its `layer` field which one it belongs to.

| Layer | What it determines |
| --- | --- |
| `language` | What a set of documents means: which objects they make, how those relate, and what is valid. Everything here is answered by resolving, which reads the documents against what already exists and runs nothing. |
| `convention` | The few behaviors every part must agree on, stated as invariants. There are seven shared conventions: operation identity and outcomes; revision-checked change; access; declared against observed state; ownership and lifecycle; records; and that nothing starts model work or repeats an effect implicitly. |
| `interface` | The implementation interface: the Model Context Protocol as the shared wire between an engine and an implementation, and between an engine and a consumer, with the keys Conduit adds to it. |

Two things are deliberately outside it. How one type behaves, such as the conditions it defines
(running or paused, for example) or a directory's edge cases, belongs to that type's contract;
the one lifecycle that every instance has is not among them, and is a shared convention. How an
engine gets from documents to a running system, such as scheduling, retries and process layout,
belongs to that engine's runtime profile, which is its documented choice for every item of the
[implementation-defined list](implementation-defined.yaml). Neither binds other engines.

One thing is left out of this version on purpose: work that outlasts its invocation. No
operation arranges, through the engine, work that starts or runs after its own invocation has
ended; work that is to happen later is asked for later, by an actor (ACC-019, ruling 38).

## Contents

| Path | Part | State |
| --- | --- | --- |
| [names.yaml](names.yaml) | The names this version assigns: the core package's address, `apiVersion` and the prefix of its `_meta` keys. The one place they are defined | Current |
| [definitions.md](definitions.md) | The terms the promises use | Draft; follows the promises as they stand |
| [promises/objects.yaml](promises/objects.yaml) | Names, IDs, references, kinds and the shape of a document | Draft; 28 promises; no vectors |
| [promises/schemas.yaml](promises/schemas.yaml) | Schemas, document syntax and numbers | Draft; 21 promises; no vectors |
| [schema-profile.md](schema-profile.md) | The standards relied on, the YAML features not accepted, the schema keywords in the profile and the formats that are checked | Draft |
| [promises/instantiation.yaml](promises/instantiation.yaml) | How a template and its arguments become an instance, and what `from` may be | Draft; 24 promises; no vectors |
| [promises/templates.yaml](promises/templates.yaml) | What the `spec` of a template holds: the type, implementations, runtimes, retirement and controls | Draft; 15 promises; no vectors |
| [promises/contracts.yaml](promises/contracts.yaml) | The five kinds of contract and what each holds | Draft; 11 promises; no vectors |
| [promises/naming.yaml](promises/naming.yaml) | How packages, their exports and contracts are named, where versions live, and which version of a contract governs | Draft; 28 promises; no vectors |
| [promises/packages.yaml](promises/packages.yaml) | Versions, digests, the archive, what a package's manifest states, and what a request to install reports | Draft; 22 promises; no vectors |
| [promises/manifest.yaml](promises/manifest.yaml) | The manifest's format: the file `conduit.yaml`, its keys, how a re-export is written, which files of a project are its documents, and the layout of a package's archive | Draft; 14 promises; no vectors |
| [promises/submissions.yaml](promises/submissions.yaml) | Handing over several documents at once, the contracts of a submitted project, how a template is replaced, and that every engine takes a directory | Draft; 10 promises; no vectors |
| [promises/pools.yaml](promises/pools.yaml) | Ownership and entries | Draft; 15 promises; no vectors |
| [promises/placement.yaml](promises/placement.yaml) | Where an Agent or Resource runs: locations, default placement, requirements on placement and values tied to a location | Draft; 20 promises; no vectors |
| [promises/conduits.yaml](promises/conduits.yaml) | What a Conduit is, what its template holds, when it exists and when it is open | Draft; 27 promises; no vectors |
| [promises/graphs.yaml](promises/graphs.yaml) | The static meaning of a graph, its two invariants, and what holds when a step is dispatched | Draft; 26 promises; no vectors |
| [promises/first-start.yaml](promises/first-start.yaml) | The objects that exist before anything is submitted | Draft; 5 promises; no vectors |
| [core/](core/README.md) | The core package, written out as documents: what it exports, and the promise that needs each export | Draft; 98 documents exported and 1 implementation, which is also an export |
| [core/conduit.yaml](core/conduit.yaml) | The core package's manifest: its address, its version, its exports and its one implementation, the engine itself | Draft |
| [core/operations.yaml](core/operations.yaml) | The Operation contracts: every management verb, the retrieval tool `retrieve-result.v1`, and the controls `set-default-location.v1` and `set-facts.v1` | Draft; 20 contracts |
| [core/capabilities.yaml](core/capabilities.yaml) | The Capability contracts: what `management`, `access`, a Pool, the `system` Pool and a location offer | Draft; 5 contracts |
| [core/types.yaml](core/types.yaml) | The Type contracts: the basic Interface and Pool types, and the types of `system`, of a location, of `management` and of `access` | Draft; 6 contracts |
| [core/requirements.yaml](core/requirements.yaml) | The Requirement contracts `bound.v1` and `shared-host.v1` | Draft; 2 contracts |
| [core/references.yaml](core/references.yaml) | The reference schemas: the Schema contracts that make a value a reference | Draft; 11 contracts |
| [core/results.yaml](core/results.yaml) | The Schema contracts `object.v1` and `diagnostic.v1`, which the outputs of the operations share | Draft; 2 contracts |
| [core/templates.yaml](core/templates.yaml) | The templates that first start instantiates: those of the five first-start objects and of the two Conduits of `root` | Draft; 7 templates |
| [schemas/](schemas/README.md) | The JSON Schemas of every document and of the manifest, each a Schema contract of the core package; which schema to use; and what a schema cannot express | Draft; 45 contracts in 9 files |
| [schemas/document.yaml](schemas/document.yaml) | `document.v1`: any template, instance or contract document | Draft; 1 contract |
| [schemas/envelope.yaml](schemas/envelope.yaml) | `apiVersion`, what every document carries, and `metadata` | Draft; 5 contracts |
| [schemas/templates.yaml](schemas/templates.yaml) | A template document and the parts of its `spec` | Draft; 8 contracts |
| [schemas/instances.yaml](schemas/instances.yaml) | An instance document of each family, the value of `from`, and the three nested instantiations | Draft; 5 contracts |
| [schemas/graphs.yaml](schemas/graphs.yaml) | A graph, a step, a value binding and a value reference | Draft; 5 contracts |
| [schemas/contracts.yaml](schemas/contracts.yaml) | A contract document of each of the five kinds, and an applied requirement | Draft; 6 contracts |
| [schemas/schema.yaml](schemas/schema.yaml) | `schema.v1`: the schema profile written as a schema | Draft; 1 contract |
| [schemas/manifest.yaml](schemas/manifest.yaml) | `manifest.v1` and `implementation-item.v1`: the manifest and one implementation item | Draft; 2 contracts |
| [schemas/values.yaml](schemas/values.yaml) | The small values the others share: names, addresses, the forms of a reference to an export, pointers, versions and digests | Draft; 12 contracts |
| [schemas/examples/valid](schemas/examples/valid) | Documents that the schemas accept: a submitted project, a package, and the instance documents of the first-start objects | Draft; 7 files in 3 directories |
| [schemas/examples/invalid](schemas/examples/invalid) | Documents that a schema rejects, or that are rejected when the file is read, one to a file, each file named for the promise it breaks | Draft; 63 files |
| [schemas/examples/invalid-by-rule](schemas/examples/invalid-by-rule) | Sets of documents that are valid against the schemas and are rejected by a rule no schema can express, each file named for the promise the set breaks | Draft; 24 files |
| [conventions/lifecycle.yaml](conventions/lifecycle.yaml) | The convention on ownership and lifecycle: the one lifecycle every instance has, what inspecting returns, retirement, unregistration and what reconciling a setup or a retirement changes | Draft; 34 promises; no vectors |
| [conventions/access.yaml](conventions/access.yaml) | The access convention: every permission is an open Conduit, decided anew for each request | Draft; 34 promises; no vectors |
| [conventions/operations.yaml](conventions/operations.yaml) | The convention on operation identity and outcomes | Draft; 31 promises; no vectors |
| [conventions/revisions.yaml](conventions/revisions.yaml) | The convention on revision-checked change | Draft; 13 promises; no vectors |
| [conventions/observation.yaml](conventions/observation.yaml) | The convention on declared against observed state | Draft; 7 promises; no vectors |
| [conventions/records.yaml](conventions/records.yaml) | The convention on records | Draft; 6 promises; no vectors |
| [conventions/implicit.yaml](conventions/implicit.yaml) | The convention that nothing starts model work or repeats an effect implicitly | Draft; 8 promises; no vectors |
| [interface/implementation.yaml](interface/implementation.yaml) | The implementation interface: the Model Context Protocol as the wire between an engine, its implementations and its consumers | Draft; 35 promises; no vectors |
| [implementation-defined.yaml](implementation-defined.yaml) | What is left to each engine, which it documents as its runtime profile | Draft; 15 items |

Every area has been written as promises, 434 in all, and the rulings of the
[consolidation audit](../docs/proposals/consolidation/README.md#rulings), the decisions of the
[re-audit](../docs/proposals/reaudit/README.md), the
[ruling on the core package and the schemas](../docs/proposals/core-and-schemas/APPLY.md), the
[decisions of the docs audit](../docs/proposals/docs-audit/GAPS.md), which converted the
behaviors that the older text accepted and the specification did not state, the
[decisions of the second round of the re-audit](../docs/proposals/reaudit/round-2/DECISIONS.md),
the [decisions of its third round](../docs/proposals/reaudit/round-3/DECISIONS.md), the
[decisions of its fourth round](../docs/proposals/reaudit/round-4/DECISIONS.md) and
[ruling 38](../docs/proposals/ruling-38/README.md) have been applied to them. The four sets of decisions before ruling 38 are the coordinator's, as the
paragraph on "accepted" above says. The JSON Schemas of documents, contracts and manifests,
the core package
as documents and the manifest's file format now exist: they are `schemas/`, `core/` and
`promises/manifest.yaml`. Still to come: the conformance vectors, and a reference model.

The core package is one package kept in two directories. `core/` holds its manifest and
everything a promise says it exports. `schemas/` holds the schemas of documents, which are
Schema contracts of the same package, because one schema can use another only as a Schema
contract (SCH-015).

Files under `promises/` are organized by topic, files under `conventions/` by convention, and
`interface/` holds the implementation interface. A file's place does not fix the layer of what
it holds: several topic files hold promises of the layer `convention`, one holds a promise of
the layer `interface`, and a few rules on documents in the other two directories have the layer
`language`. A promise's own `layer` field decides.

## The form of a promise

A promise is a data record with these fields:

| Field | Meaning |
| --- | --- |
| `id` | A stable identifier. It is never reused, and a changed promise keeps it. A removed promise is named, with where it went, in the comment at the head of its file. |
| `layer` | `language`, `convention` or `interface`. |
| `level` | `must`, `must not`, `should` or `may`. |
| `when` | The situation. |
| `then` | The single obligation. For `must not`, the thing that must not happen. |
| `observable` | What can be checked to tell whether the promise was kept. |
| `vectors` | The conformance vectors that test it. None exists yet. |

Rules for writing one:

1. One obligation per promise.
2. Stated as something that can be checked. If nothing could tell, it is not a promise.
3. No sentence of the form "this does not imply". State the correct behavior as its own promise.
4. Every term is defined in [definitions.md](definitions.md).
5. A promise with no vector is not yet testable and does not count as covered.

The `layer` field lets coverage be counted by layer.

## Conformance

A claim of conformance names a version of this specification and one of three levels. A
version is named by the `apiVersion` that [names.yaml](names.yaml) states for it. An engine
claims the Language level or the Engine level. Something that is not an engine, which is an
implementation, claims the Implementation level.

| Level | Who claims it | What it covers |
| --- | --- | --- |
| Language | An engine | Every promise whose layer is `language`: documents in, resolved objects or diagnostics out. |
| Engine | An engine | The Language level, plus every promise whose layer is `convention` or `interface`, plus a runtime profile that documents a choice for every item of the implementation-defined list. |
| Implementation | An implementation | Every promise whose layer is `interface` and that binds an implementation: one whose obligation falls on the implementation and not on the engine, such as IFC-008, IFC-009, IFC-019, IFC-026 and IFC-031. |

An engine that is given a document it cannot honor rejects the document and says why in a
diagnostic. It never accepts a document and ignores part of it.

## Checking

Run these from the root of the repository:

```sh
python3 tools/set_names.py --check   # the names under spec/ agree with names.yaml
python3 tools/check_schemas.py       # schemas, core package and examples
git diff --check                     # whitespace errors
python3 tools/check_docs.py          # links, in every Markdown file of the repository
```

The first three read `spec/`. The first checks that every file under `spec/` writes the names
of [names.yaml](names.yaml) as that file states them. The second reads every file under `core/`
and `schemas/` as strict YAML, checks every schema against
[schema-profile.md](schema-profile.md), validates every document of the core package and
every valid example against the schemas, checks that every invalid example is rejected, checks
the references among those files, and applies a few of the rules that no schema can express,
among them that a `default` satisfies its schema (SCH-019). It needs PyYAML and jsonschema,
and runs itself again under `/usr/bin/python3` where the first interpreter lacks them. The
third finds whitespace errors in what was changed, and needs a Git working tree.

The fourth, `check_docs.py`, covers the whole repository: it checks the links between all its
Markdown files, those under `spec/` among them. It is not for a copy that holds `spec/` alone,
because files under `spec/` link to records under `docs/`. These are the four checks that
[AGENTS.md](../AGENTS.md#checks) lists, and that page says what each needs. None of them is a
conformance vector: no promise is tested yet.

## Existing standards first

Where an existing standard or widely used practice covers a need, the specification adopts it
rather than defining its own. This holds above all for graphs, schemas and packages, which are
not what distinguishes Conduit. [schema-profile.md](schema-profile.md) lists what is relied on.

## Reading order

Read [definitions.md](definitions.md) first, with [names.yaml](names.yaml) beside it. Then read
the files in the order of the Contents table, which is an order a reader of the Language level
can follow:

1. The shape of a document and of a schema: `promises/objects.yaml`, `promises/schemas.yaml`
   and [schema-profile.md](schema-profile.md). The JSON Schema of each shape is in
   [schemas/](schemas/README.md), and can be read beside the promises it encodes.
2. Templates and what they are made of: `promises/instantiation.yaml`,
   `promises/templates.yaml` and `promises/contracts.yaml`.
3. Packages and submissions: `promises/naming.yaml`, `promises/packages.yaml`,
   `promises/manifest.yaml` and `promises/submissions.yaml`.
4. The families and how they relate: `promises/pools.yaml`, `promises/placement.yaml`,
   `promises/conduits.yaml`, `promises/graphs.yaml` and `promises/first-start.yaml`.
5. The core package: [core/README.md](core/README.md), which lists every export with the
   promise that needs it, and then the files of `core/`. The first-start objects are instances
   of its templates, and the management verbs of the conventions are its operations.

A reader of the Engine level goes on to the conventions, beginning with
`conventions/lifecycle.yaml` and `conventions/access.yaml`, then to
`interface/implementation.yaml`, and last to
[implementation-defined.yaml](implementation-defined.yaml).
