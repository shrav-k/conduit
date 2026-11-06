# Conduit specification

Status: draft, begun 2026-11-05. Incomplete, and not a release. Every area has been written as
promises; the parts still to come are listed under [Contents](#contents). Accepted behavior that
the promises do not yet cover still lives in [docs/protocol](../docs/protocol/README.md).

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

## Contents

| Path | Part | State |
| --- | --- | --- |
| [names.yaml](names.yaml) | The names this version assigns: the core package's address, `apiVersion` and the prefix of its `_meta` keys. The one place they are defined | Current |
| [definitions.md](definitions.md) | The terms the promises use | Draft; follows the promises as they stand |
| [promises/objects.yaml](promises/objects.yaml) | Names, IDs, references, kinds and the shape of a document | Draft; 24 promises; no vectors |
| [promises/schemas.yaml](promises/schemas.yaml) | Schemas and document syntax | Draft; 18 promises; no vectors |
| [schema-profile.md](schema-profile.md) | The standards relied on, the YAML features not accepted, the permitted schema keywords and the enforced formats | Draft |
| [promises/instantiation.yaml](promises/instantiation.yaml) | How a template and its arguments become an instance | Draft; 22 promises; no vectors |
| [promises/templates.yaml](promises/templates.yaml) | What the `spec` of a template holds: the type, implementations, runtimes, retirement and controls | Draft; 12 promises; no vectors |
| [promises/contracts.yaml](promises/contracts.yaml) | The five kinds of contract and what each holds | Draft; 8 promises; no vectors |
| [promises/naming.yaml](promises/naming.yaml) | How packages, their exports and contracts are named, where versions live, and which version of a contract governs | Draft; 26 promises; no vectors |
| [promises/packages.yaml](promises/packages.yaml) | Versions, digests, the archive, and what a package's manifest states | Draft; 16 promises; no vectors |
| [promises/submissions.yaml](promises/submissions.yaml) | Handing over several documents at once, and the contracts of a project that is not published | Draft; 8 promises; no vectors |
| [promises/pools.yaml](promises/pools.yaml) | Ownership and entries | Draft; 14 promises; no vectors |
| [promises/placement.yaml](promises/placement.yaml) | Where an Agent or Resource runs: locations, default placement and requirements on placement | Draft; 17 promises; no vectors |
| [promises/conduits.yaml](promises/conduits.yaml) | What a Conduit is, what its template holds, when it exists and when it is open | Draft; 23 promises; no vectors |
| [promises/graphs.yaml](promises/graphs.yaml) | The static meaning of a graph and its two invariants | Draft; 22 promises; no vectors |
| [promises/first-start.yaml](promises/first-start.yaml) | The objects that exist before anything is submitted | Draft; 4 promises; no vectors |
| [conventions/lifecycle.yaml](conventions/lifecycle.yaml) | The convention on ownership and lifecycle: the one lifecycle every instance has, what inspecting returns, retirement and unregistration | Draft; 26 promises; no vectors |
| [conventions/access.yaml](conventions/access.yaml) | The access convention: every permission is an open Conduit | Draft; 28 promises; no vectors |
| [conventions/operations.yaml](conventions/operations.yaml) | The convention on operation identity and outcomes | Draft; 24 promises; no vectors |
| [conventions/revisions.yaml](conventions/revisions.yaml) | The convention on revision-checked change | Draft; 13 promises; no vectors |
| [conventions/observation.yaml](conventions/observation.yaml) | The convention on declared against observed state | Draft; 6 promises; no vectors |
| [conventions/records.yaml](conventions/records.yaml) | The convention on records | Draft; 6 promises; no vectors |
| [conventions/implicit.yaml](conventions/implicit.yaml) | The convention that nothing starts model work or repeats an effect implicitly | Draft; 9 promises; no vectors |
| [interface/implementation.yaml](interface/implementation.yaml) | The implementation interface: the Model Context Protocol as the wire between an engine, its implementations and its consumers | Draft; 29 promises; no vectors |
| [implementation-defined.yaml](implementation-defined.yaml) | What is left to each engine, which it documents as its runtime profile | Draft; 13 items |

Every area has been written as promises, and the rulings of the
[consolidation audit](../docs/proposals/consolidation/README.md#rulings) and the decisions of
the [re-audit](../docs/proposals/reaudit/README.md) have been applied to them. Still to come:
the JSON Schemas of documents, contracts and manifests; the core package as documents; the
manifest's file format; and the conformance vectors.

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

An engine claims conformance to a named version of this specification at one of two levels. A
version is named by the `apiVersion` that [names.yaml](names.yaml) states for it.

| Level | What it covers |
| --- | --- |
| Language | Every promise whose layer is `language`: documents in, resolved objects or diagnostics out. |
| Engine | The Language level, plus every promise whose layer is `convention` or `interface`, plus a runtime profile that documents a choice for every item of the implementation-defined list. |

An engine that is given a document it cannot honor rejects the document and says why in a
diagnostic. It never accepts a document and ignores part of it.

## Existing standards first

Where an existing standard or widely used practice covers a need, the specification adopts it
rather than defining its own. This holds above all for graphs, schemas and packages, which are
not what distinguishes Conduit. [schema-profile.md](schema-profile.md) lists what is relied on.

## Reading order

Read [definitions.md](definitions.md) first, with [names.yaml](names.yaml) beside it. Then read
the files in the order of the Contents table, which is an order a reader of the Language level
can follow:

1. The shape of a document and of a schema: `promises/objects.yaml`, `promises/schemas.yaml`
   and [schema-profile.md](schema-profile.md).
2. Templates and what they are made of: `promises/instantiation.yaml`,
   `promises/templates.yaml` and `promises/contracts.yaml`.
3. Packages and submissions: `promises/naming.yaml`, `promises/packages.yaml` and
   `promises/submissions.yaml`.
4. The families and how they relate: `promises/pools.yaml`, `promises/placement.yaml`,
   `promises/conduits.yaml`, `promises/graphs.yaml` and `promises/first-start.yaml`.

A reader of the Engine level goes on to the conventions, beginning with
`conventions/lifecycle.yaml` and `conventions/access.yaml`, then to
`interface/implementation.yaml`, and last to
[implementation-defined.yaml](implementation-defined.yaml).
