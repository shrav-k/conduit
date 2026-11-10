# Conduit specification

Status on 2026-11-10: draft, begun 2026-11-05. Incomplete, and not a release. Every area has
been written as promises, and the core package, the JSON Schemas of documents and the
manifest's format have been written as documents. The first build has a strict loader, a pure
reference model on S1 and a runnable engine shell in [reference-model/](../reference-model/README.md).
There are 364 portable vectors under [conformance/](../conformance/README.md), testing slices
of promises; the [manifest](../conformance/manifest.yaml) records the coverage and its gaps.
This remains a draft: the model and vectors do not yet cover the whole specification, and the
next cold read follows the first build. Ruling 38, point 4, asks for agreement between the
reference model, vectors and promises, and no unsafe finding in the latest round of readers.

This page describes the specification as it stands now. How it came to stand so, ruling by
ruling, and where the record of each decision is, are under [History](#history) at the end.
As of 2026-11-10, the logs of [increment 2](../docs/proposals/first-build/log-increment-2.md)
and [increment 3](../docs/proposals/first-build/log-increment-3.md) record the subsequent
settlements applied to the promises, including schema defaults, plain errors and successful
content without declared outputs. The shell's [closeout](../docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md)
ran real Claude Code and Codex as Agents through S1's T1, T2 and T7; increment 4, S1 complete
with author measurements, is next. The sign-in changes accepted in
[ruling 49](../docs/proposals/ruling-49/README.md) on 2026-11-10 are pending, not promises
already supplied by this specification.

The review of increment 1 of the first build on 2026-11-09 applied these changes:
schema rejections cite SUB-005 and the failing keyword's location, either syntax accepts one
leading byte order mark, SUB-011 has a floor and covers files, submissions and archives, and
MAN-010 states what a pin holds. Ruling 48, point 3, restricts SCH-016 to interoperable pattern
tokens. The [log](../docs/proposals/first-build/log-increment-1.md#after-the-review-of-2026-11-09)
records those fixes and their vectors. Before them, the coordinator settled the findings under
[ruling 46](../docs/proposals/ruling-46/README.md), point 4, of 2026-11-09, which
the [log of increment 1](../docs/proposals/first-build/log-increment-1.md) lists and the owner
can strike: it changed SCH-006, SCH-016, SCH-018, SCH-020, SUB-005, SUB-010, MAN-011, DL10 and
the [schema profile](schema-profile.md), and added SUB-011. Before it,
[ruling 45](../docs/proposals/ruling-45/README.md), of the same day, added six
[core terms](definitions.md#core-terms) to the definitions, and
[ruling 44](../docs/proposals/ruling-44/README.md), of 2026-11-07, settled
[round 6](../docs/proposals/reaudit/round-6/README.md) of the re-audit.

The specification alone binds. Nothing under [docs/protocol](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/README.md) binds:
it is the design as it stood before the specification, superseded, and kept as source material
for the type contracts that are still to be written. A behavior that is missing from `spec/` is
a gap in `spec/`, and no older document supplies it.

## What this is

Conduit is a configuration language for the relationships between independent agents and the
things they share. This directory is its specification: the documents that bind every engine.

Conduit innovates only on connection and access
([ruling 40](../docs/proposals/ruling-40/README.md)). What is its own is its documents;
connection, which is the Conduits; access, which is who may reach what, under which limits,
and how access is delegated; accountability, which is what must be recorded of who did what;
and the shape of its contracts. Everything else is a provider behind a contract that any
existing tool can implement and that the user picks, by registering a Resource of the
provider's type: the runtime that runs an instance's implementation, the machines and where an
instance is placed on them, the fetching of packages, and the stores of secrets
([promises/providers.yaml](promises/providers.yaml),
[promises/secrets.yaml](promises/secrets.yaml)). The engine's own runner, its own machine
provider and its own fetching of `https:` and `file:` URLs serve where the user picks none
(PRV-003), and no store of secrets is built into the engine (SEC-020). The storage of records
and the engine's own state stay each engine's choice for now, in standard formats (DL2, DL8),
and so does the mechanism that proves who is who, by a cryptographic proof wherever a request
crosses machines, within two standard forms: mutual TLS with certificates in the form of an
X509-SVID, and signed tokens in the form of a JWT-SVID, which the engine's address at an
`https` URL always accepts as the protocol's own authorization (IFC-022, IFC-042, DL13).
A run is given what it presents, and the engine's address, in the input `identity` of `run.v1`,
under names fixed for every engine (PRV-022). Every server that the engine does not run itself,
other than a `plain` server reached by `remote`, stands behind a gate: the server of a run that
a runtime provider makes, and a `remote` server that takes Conduit's keys. That gate admits only
the proof of an enforcement point, which in this version the engine alone makes, and the engine
sends it nothing until it has proved who it is (PRV-025, PRV-028, IFC-017, IFC-041). A `plain`
server reached by `remote` takes none of Conduit's keys, and whatever can reach its URL reaches
it (IFC-007, IFC-041). Whether a restriction can be enforced depends on the runtime provider;
this version reports every restriction as mediated (ACC-027).

The specification has three layers, and every promise says in its `layer` field which one it
belongs to.

| Layer | What it determines |
| --- | --- |
| `language` | What a set of documents means: which objects they make, how those relate, and what is valid. Everything here is answered by resolving, which reads the documents against what already exists, the lifecycle state and the status of the objects that exist among it, such as the labels reported for a node, and runs nothing. |
| `convention` | The few behaviors every part must agree on, stated as invariants. There are seven shared conventions: idempotency keys and outcomes; revision-checked change; access; declared against observed state; ownership and lifecycle; records; and that nothing starts model work or repeats an effect implicitly. |
| `interface` | The implementation interface: the Model Context Protocol as the shared wire between an engine and an implementation, and between an engine and a consumer, with the keys Conduit adds to it as an extension of the protocol. |

The provider contracts are not a fourth layer. Each is a capability of the core package whose
operations are written as any operation is, and its promises fall in the three layers: what a
provider's type must state, how an instance names its runtime provider and which Conduits may
reach a provider are `language`, as in PRV-001, PRV-004, PRV-006, SEC-005 and SEC-022; what
the engine asks of a provider, by a provider call or by a read of a secret, and what a provider
must do are `convention`, as in PRV-005, PRV-007, PRV-011, PRV-022 and SEC-010; and what an
implementation of `get-secret.v1` must do with a secret, and what any implementation that is
given one must not do with it, are `interface`, as in SEC-006 and SEC-019.

The following are deliberately outside it. How one type behaves, such as the conditions it defines
(running or paused, for example) or a directory's edge cases, belongs to that type's contract;
the one lifecycle that every instance has is not among them, and is a shared convention. How a
system gets from documents to running work, beyond what the promises fix, belongs to the
provider that does it, such as how a runtime provider starts a process or where a machine
provider places an instance, which that provider's contract states; and what stays the engine's
own, such as retries and the storage of its records, belongs to that engine's conformance
document, which is its documented choice for every item of the [implementation-defined
list](implementation-defined.yaml). Neither binds other engines. An Agent's own means, its
built-in tools, its shell and sandbox, its network, and the services of the account it signs in
with are outside the documents; inspecting an instance shows its reach through Conduit only.
Confining an Agent belongs to its runtime provider, under PRV-006, and every restriction in
this version is reported as mediated, under ACC-027.

One thing is left out of this version on purpose: work that outlasts its invocation. No
operation arranges, through the engine, work that starts or runs after its own invocation has
ended; work that is to happen later is asked for later, by an actor (ACC-019, ruling 38). The
requests that the engine makes on behalf of an invocation that has ended are two reads of a
secret: the read again for a connection that a setup made, once the engine has lost its values
(SEC-023), and, once the engine has restarted, the reads before it runs again a provider whose
run ended with it, for the setup that last asked for that run (PRV-026).

## Reading paths

Begin with [definitions.md](definitions.md): its first table gives the core terms in one line
each and links the section that holds each one's full row. [names.yaml](names.yaml) holds the
names this version assigns. Then follow the path for what you do. Each path reads the rules as
they stand, and none needs the records under `docs/proposals/`. Every promise is in the
[Contents](#contents) table's files; a path only orders them.

### An author of documents

Someone who writes templates, instances and a project's manifest, registers them and acts
through Conduits.

1. The shape of a document, its names and references, and schemas:
   `promises/objects.yaml`, `promises/schemas.yaml` and [schema-profile.md](schema-profile.md).
   The JSON Schema of each document is in [schemas/](schemas/README.md), and the
   [example project](schemas/examples/valid/project/) shows each shape.
2. Templates and instances: `promises/instantiation.yaml` and `promises/templates.yaml`.
3. Ownership, placement and the Conduits: `promises/pools.yaml`, `promises/placement.yaml` and
   `promises/conduits.yaml`.
4. Who may reach what: `conventions/access.yaml`, whose ACC-033 lists the target that
   authorizes each management request; then
   [what access does not narrow](#what-access-does-not-narrow).
5. Handing over a project: `promises/submissions.yaml`, `promises/manifest.yaml` and
   `promises/naming.yaml`, and `promises/packages.yaml` for installing a package.
6. What a request does and how it ends: `conventions/lifecycle.yaml` for setup, retirement and
   removal; `conventions/operations.yaml` for outcomes, idempotency keys, repeats and
   reconciling an unknown outcome; and `conventions/revisions.yaml` for controls.
7. Secrets: `promises/secrets.yaml`, then
   [what the guarantee for secrets covers](#what-the-guarantee-for-secrets-covers).

What an Agent sees of its access is its tools, IFC-012 and IFC-033; the rest of `interface/`
is for the other two paths.

### A writer of contracts and providers

Someone who writes Type, Capability, Operation, Requirement or Schema contracts, an
implementation of them, or a provider's package.

1. The author's path to its step 2.
2. The contracts: `promises/contracts.yaml`, controls among them (CTR-006), then
   `promises/naming.yaml` for names and what a later release may add, `promises/packages.yaml`
   and `promises/manifest.yaml` for a package, its implementation items, `plain` and
   `supports`.
3. The wire: `interface/implementation.yaml`, for what a call carries (IFC-006), how a filter is
   applied (IFC-019), what a result states (IFC-009, IFC-025, IFC-026), what the answers of a
   plain server mean (IFC-010), what a tool's definition holds (IFC-033), and the gate and the
   proof of who is who (IFC-017, IFC-041).
4. For a provider: `promises/providers.yaml`, and `promises/secrets.yaml` for a store's package.
   The provider capabilities and their operations are in
   [core/capabilities.yaml](core/capabilities.yaml) and
   [core/operations.yaml](core/operations.yaml).
5. [Conformance](#conformance) says, at the Implementation level, which promises bind what you
   write.

### An engine builder

Someone who builds an engine and claims a level of [conformance](#conformance). Read the files
in the order of the Contents table, which is an order a reader of the Language level can follow:

1. The shape of a document and of a schema: `promises/objects.yaml`, `promises/schemas.yaml`
   and [schema-profile.md](schema-profile.md). The JSON Schema of each shape is in
   [schemas/](schemas/README.md), and can be read beside the promises it encodes.
2. Templates and what they are made of: `promises/instantiation.yaml`,
   `promises/templates.yaml` and `promises/contracts.yaml`.
3. Packages and submissions: `promises/naming.yaml`, `promises/packages.yaml`,
   `promises/manifest.yaml` and `promises/submissions.yaml`.
4. The families and how they relate: `promises/pools.yaml`, `promises/placement.yaml`,
   `promises/conduits.yaml` and `promises/first-start.yaml`.
5. What the engine reaches behind contracts: `promises/providers.yaml`, then
   `promises/secrets.yaml`.
6. The core package: [core/README.md](core/README.md), which lists every export with the
   promise that needs it, and then the files of `core/`. The first-start objects are instances
   of its templates, and the management verbs of the conventions are its operations.

A reader of the Engine level goes on to the conventions, beginning with
`conventions/lifecycle.yaml` and `conventions/access.yaml`, then to
`interface/implementation.yaml`, and last to
[implementation-defined.yaml](implementation-defined.yaml), each of whose items its
conformance document states a choice for. [Limits of this version](#limits-of-this-version)
says what the promises leave open.

## Contents

Counts and vector status below are as of 2026-11-10. A vector tag tests a slice of a promise,
not necessarily its whole obligation; the [coverage manifest](../conformance/manifest.yaml)
lists the cases and the reasons for missing S1 coverage.

| Path | Part | State |
| --- | --- | --- |
| [names.yaml](names.yaml) | The names this version assigns: the core package's address, `apiVersion`, the prefix of its `_meta` keys, and the names of what `identity` carries. The one place they are defined | Current |
| [definitions.md](definitions.md) | The terms the promises use | Draft; follows the promises as they stand |
| [promises/objects.yaml](promises/objects.yaml) | Names, IDs, references, kinds and the shape of a document | Draft; 24 promises; vectors test slices of 10 promises |
| [promises/schemas.yaml](promises/schemas.yaml) | Schemas, document syntax and numbers | Draft; 21 promises; vectors test slices of 9 promises |
| [schema-profile.md](schema-profile.md) | The standards relied on, the YAML features not accepted, the schema keywords in the profile and the formats that are checked | Draft |
| [promises/instantiation.yaml](promises/instantiation.yaml) | How a template and its arguments become an instance, and what `from` may be | Draft; 23 promises; vectors test slices of 20 promises |
| [promises/templates.yaml](promises/templates.yaml) | What the `spec` of a template holds: the type, implementations, hooks, retirement and controls | Draft; 14 promises; vectors test slices of 9 promises |
| [promises/contracts.yaml](promises/contracts.yaml) | The five kinds of contract and what each holds | Draft; 10 promises; vectors test slices of 1 promise |
| [promises/naming.yaml](promises/naming.yaml) | How packages, their exports and contracts are named, where versions live, and which version of a contract governs | Draft; 28 promises; vectors test slices of 15 promises |
| [promises/packages.yaml](promises/packages.yaml) | Versions, digests, the archive, what a package's manifest states, and what a request to install reports | Draft; 22 promises; vectors test slices of 19 promises |
| [promises/manifest.yaml](promises/manifest.yaml) | The manifest's format: the file `conduit.yaml`, `conduit.yml` or `conduit.json`, its keys, implementation items, which variables of a run they may not bind, which files of a project are its documents, and the layout of a package's archive | Draft; 17 promises; vectors test slices of 9 promises |
| [promises/submissions.yaml](promises/submissions.yaml) | Handing over several documents at once, the contracts of a submitted project and the address that names them, how a template is replaced, that every engine takes a directory, and documents, files, submissions or archives beyond a limit that the engine states | Draft; 11 promises; vectors test slices of 10 promises |
| [promises/pools.yaml](promises/pools.yaml) | Ownership and entries | Draft; 15 promises; vectors test slices of 10 promises |
| [promises/placement.yaml](promises/placement.yaml) | Where an Agent or Resource runs: nodes, machines, labels and placement by the provider, and values tied to a node | Draft; 21 promises; vectors test slices of 7 promises |
| [promises/conduits.yaml](promises/conduits.yaml) | What a Conduit is, what its template holds, when it exists and when it is open | Draft; 22 promises; vectors test slices of 12 promises |
| [promises/first-start.yaml](promises/first-start.yaml) | The objects that exist before anything is submitted | Draft; 5 promises; vectors test slices of 5 promises |
| [promises/providers.yaml](promises/providers.yaml) | Providers: what one is and how the user picks one; the runtime provider and its runs, what a run is given to reach the engine, the gate before every server that the engine does not run itself, other than a `plain` server reached by `remote`, and how what answers at a run's URL proves itself, a stop that cannot be carried out now, and what the engine's own runner passes to a process; the machine provider, package sources, the provider call, and a provider's run made again after the engine restarts | Draft; 28 promises; vectors test slices of 7 promises |
| [promises/secrets.yaml](promises/secrets.yaml) | Secrets: how a document marks one, which Resources hold one and what else their type may offer, who may have one read, how the engine reads one before a run or a connection, and again once it has lost a connection's values, and where the value may go, a header that carries one among it | Draft; 24 promises; vectors test slices of 1 promise |
| [core/](core/README.md) | The core package, written out as documents: what it exports, and the promise that needs each export | Draft; 102 documents exported and 1 implementation, which is also an export |
| [core/conduit.yaml](core/conduit.yaml) | The core package's manifest: its address, its version, its exports and its one implementation, the engine itself | Draft |
| [core/operations.yaml](core/operations.yaml) | The Operation contracts: every management verb, the setup hook and the retire hook, the operations of the three provider capabilities that the engine invokes by provider calls, and `get-secret.v1` | Draft; 23 contracts |
| [core/capabilities.yaml](core/capabilities.yaml) | The Capability contracts: what `management`, `access` and a Pool offer, and the four provider capabilities | Draft; 7 contracts |
| [core/types.yaml](core/types.yaml) | The Type contracts: the basic Interface and Pool types, and the types of a node, of `management` and of `access` | Draft; 5 contracts |
| [core/requirements.yaml](core/requirements.yaml) | The Requirement contracts `bound.v1` and `shared-host.v1`, which the engine applies itself | Draft; 2 contracts |
| [core/references.yaml](core/references.yaml) | The reference schemas: the Schema contracts that make a value a reference, `secret-reference.v1` among them | Draft; 12 contracts |
| [core/results.yaml](core/results.yaml) | The Schema contracts `object.v1`, `diagnostic.v1`, `handle.v1`, `condition.v1` and `record.v1`: an object as a result names it, a diagnostic, a handle, a condition and a record | Draft; 5 contracts |
| [core/templates.yaml](core/templates.yaml) | The templates that first start instantiates: those of the five first-start objects and of the two Conduits of `root` | Draft; 7 templates |
| [schemas/](schemas/README.md) | The JSON Schemas of every document and of the manifest, each a Schema contract of the core package; which schema to use; and what a schema cannot express | Draft; 41 contracts in 8 files |
| [schemas/document.yaml](schemas/document.yaml) | `document.v1`: any template, instance or contract document | Draft; 1 contract |
| [schemas/envelope.yaml](schemas/envelope.yaml) | `apiVersion`, what every document carries, and `metadata` | Draft; 5 contracts |
| [schemas/templates.yaml](schemas/templates.yaml) | A template document and the parts of its `spec` | Draft; 7 contracts |
| [schemas/instances.yaml](schemas/instances.yaml) | An instance document of each family, the value of `from`, and the two nested instantiations | Draft; 4 contracts |
| [schemas/contracts.yaml](schemas/contracts.yaml) | A contract document of each of the five kinds, and an applied requirement | Draft; 6 contracts |
| [schemas/schema.yaml](schemas/schema.yaml) | `schema.v1`: the schema profile written as a schema | Draft; 1 contract |
| [schemas/manifest.yaml](schemas/manifest.yaml) | `manifest.v1` and `implementation-item.v1`: the manifest and one implementation item; and `server-package.v1` and `server-remote.v1`: a Package and a Remote of server.json | Draft; 4 contracts |
| [schemas/values.yaml](schemas/values.yaml) | The small values the others share: names, addresses, the forms of a reference to an export, pointers, versions, digests, IDs, revisions and label selectors | Draft; 13 contracts |
| [schemas/examples/valid](schemas/examples/valid) | Documents that the schemas accept: submitted projects, a package, and the instance documents of the first-start objects | Draft; 10 files in 4 directories, and one file whose name begins with a dot, which is not read |
| [schemas/examples/invalid](schemas/examples/invalid) | Documents that a schema rejects, or that are rejected when the file is read, one to a file, each file named for the promise it breaks | Draft; 62 files |
| [schemas/examples/invalid-by-rule](schemas/examples/invalid-by-rule) | Sets of documents that are valid against the schemas and are rejected by a rule no schema can express, each file named for the promise the set breaks | Draft; 41 files |
| [conventions/lifecycle.yaml](conventions/lifecycle.yaml) | The convention on ownership and lifecycle: the one lifecycle every instance has, what inspecting returns, retirement, removal and unregistration, and what reconciling a setup or a retirement changes, and the expected ID that a lifecycle action may carry | Draft; 36 promises; vectors test slices of 17 promises |
| [conventions/access.yaml](conventions/access.yaml) | The access convention: every permission is an open Conduit, decided anew for each request | Draft; 29 promises; vectors test slices of 20 promises |
| [conventions/operations.yaml](conventions/operations.yaml) | The convention on idempotency keys and outcomes | Draft; 31 promises; vectors test slices of 29 promises |
| [conventions/revisions.yaml](conventions/revisions.yaml) | The convention on revision-checked change | Draft; 14 promises; vectors test slices of 3 promises |
| [conventions/observation.yaml](conventions/observation.yaml) | The convention on declared against observed state | Draft; 5 promises; vectors test slices of 2 promises |
| [conventions/records.yaml](conventions/records.yaml) | The convention on records | Draft; 6 promises; vectors test slices of 6 promises |
| [conventions/implicit.yaml](conventions/implicit.yaml) | The convention that nothing starts model work or repeats an effect implicitly | Draft; 5 promises; vectors test slices of 2 promises |
| [interface/implementation.yaml](interface/implementation.yaml) | The implementation interface: the Model Context Protocol as the wire between an engine, its implementations and its consumers, with Conduit's keys as an extension of the protocol, and the proof of who is who in both directions | Draft; 44 promises; vectors test slices of 23 promises |
| [implementation-defined.yaml](implementation-defined.yaml) | What is left to each engine, which it documents as its conformance document | Draft; 15 items |

Every area has been written as promises, 455 in all: 203 of the layer `language`, 208 of the
layer `convention` and 44 of the layer `interface`. The JSON Schemas of documents, contracts and
manifests, the core package as documents and the manifest's file format exist: they are
`schemas/`, `core/` and `promises/manifest.yaml`. The vectors and the reference model exist on
S1, with the scope and remaining work stated above and in the
[conformance document](../reference-model/CONFORMANCE.md).
[History](#history) lists the rulings and decisions applied to the promises.

The core package is one package kept in two directories. `core/` holds its manifest and
everything a promise says it exports. `schemas/` holds the schemas of documents, which are
Schema contracts of the same package, because one schema can use another only as a Schema
contract (SCH-015).

Files under `promises/` are organized by topic, files under `conventions/` by convention, and
`interface/` holds the implementation interface. A file's place does not fix the layer of what
it holds: several topic files hold promises of the layer `convention`, two hold promises of the
layer `interface`, and a few rules on documents in the other two directories have the layer
`language`. A promise's own `layer` field decides.

### Promise files that rulings removed

A promise file whose every promise a ruling removed is taken out of the tree and listed here,
with the IDs it held, so that none of them is ever reused (the owner's answer on the planner's
choice P2, recorded in [ruling 41](../docs/proposals/ruling-41/README.md#after-the-ruling)).

| File | IDs it held | Removed by |
| --- | --- | --- |
| `promises/graphs.yaml`, the static meaning of a graph and what held when a step was dispatched; its [last version](https://github.com/shrav-k/conduit/blob/0a9a32326830452d9761f4963077db4fb618918a/spec/promises/graphs.yaml) | GRA-001 to GRA-028. Consolidation pass 1 folded GRA-013 into GRA-019 and GRA-016 into GRA-020. Ruling 41 removed the rest by decision 1 of the principles review (ruling 40, point 3c): an instance's own setup and retirement work is a setup hook and a retire hook, and an operation built from others is an ordinary implementation. GRA-002's rule went to TPL-012; GRA-003's to OPS-022, CON-004 and POOL-013; GRA-022's to CON-004 and POOL-013; nothing replaces the others. | [Ruling 41](../docs/proposals/ruling-41/README.md) |

## The form of a promise

A promise is a data record with these fields:

| Field | Meaning |
| --- | --- |
| `id` | A stable identifier. It is never reused, and a changed promise keeps it. A removed promise is named, with where it went, in the comment at the head of its file, or, where its whole file was removed, [above](#promise-files-that-rulings-removed). |
| `layer` | `language`, `convention` or `interface`. |
| `level` | `must`, `must not`, `should` or `may`. |
| `when` | The situation. |
| `then` | The single obligation. For `must not`, the thing that must not happen. |
| `observable` | What can be checked to tell whether the promise was kept. |
| `vectors` | The conformance vectors that test it. See the dated coverage status above. |

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
implementation, a provider's or a store's among them, claims the Implementation level, whatever
the layer of the promises that bind it.

| Level | Who claims it | What it covers |
| --- | --- | --- |
| Language | An engine | Every promise whose layer is `language`: documents in, resolved objects or diagnostics out. |
| Engine | An engine | The Language level, plus every promise whose layer is `convention` or `interface` and whose obligation falls on the engine, plus a conformance document that documents a choice for every item of the implementation-defined list. The promises on providers count among these where the engine's own runner, machine provider and fetching keep them because no provider serves (PRV-003). |
| Implementation | An implementation, a provider's among them | Every promise, whatever its layer, whose obligation falls on the implementation and not on the engine. Examples are IFC-008, IFC-009, IFC-019, IFC-026, IFC-031, IFC-047, SEC-006 and SEC-019. For a provider's implementation this includes the promises that bind the provider, which have the layer `convention`, such as PRV-010 to PRV-012, PRV-014, PRV-019, PRV-021, PRV-022, PRV-025 and PRV-028. For a server reached by `remote` that is not plain, it includes PRV-025. |

An engine that is given a document it cannot honor rejects the document and says why in a
diagnostic. It never accepts a document and ignores part of it unsaid: where a promise leaves
a part of an accepted document unused, the promise says so, as SCH-009 does a `default` reached
only through a conditional keyword, or through a `$ref` at the root of an operation's schema.
A type's default for a hook that the type does not require is also named, when the type is
read, in a diagnostic that does not reject (CTR-004, TPL-004).

## Limits of this version

The two sections below state what the promises leave open. They are statements, not promises,
and nothing tests them; each point cites the promises that state its parts. The first is what
[ruling 44](../docs/proposals/ruling-44/README.md) accepted on delegation and on access to
`management` (D03 and D04 of round 6). The second is section 4 of the principles review. Work
that outlasts its invocation is left out, as [What this is](#what-this-is) says.

### What access does not narrow

- A bound limits a right to open by Pools, targets and operations, and never by filters
  (ACC-029, ACC-030). Its consumer may open, for any consumer, a Conduit that lists any
  operation that the bound names, to any target that the bound covers, with no filter but those
  that the Conduit's own template applies. A filter limits only the requests through its own
  Conduit, and the onward requests and hooks that carry it (ACC-009, ACC-017, ACC-018). So an
  actor that reads a directory through a Conduit that applies a filter, and that is also the
  consumer of a Conduit to `access` whose bound covers the directory and names its read
  operation, can open for itself a Conduit that reads the directory with no filter.
- A bound that names a Pool covers every Resource that the Pool owns, those instantiated after
  the Conduit that carries the bound was opened among them (ACC-029). Closing an actor's own
  Conduit to `access` closes none of the Conduits that the actor opened (ACC-030): each is
  closed by its own request, or as its ends are retired (CON-008).
- Access to `management` cannot be narrowed to one template, nor to less than whole operations.
  A Conduit to `management` carries no bound (CON-028), and the engine, which implements the
  operations of `management`, supports no filter, so no Conduit that lists one of them and
  applies a filter is instantiated (CON-022, MAN-012). A Conduit to `management` that lists
  `register.v1` or `replace.v1` trusts its consumer to register, or to replace, any object that
  ACC-033 authorizes against `management` alone (ACC-030). Since `replace.v1` hands over the
  project of its document (SUB-009), a Conduit that lists it trusts its consumer, as one that
  lists `register.v1` does, to choose the contracts and the implementation items, programs and
  `remote` URLs among them, of the instances made from the template afterwards (ACC-030).
  Registering a Resource into a Pool that exists and is not retired also needs a Conduit to that
  Pool (ACC-033), which narrows by owner and never by template.
- Narrower access to `management` goes through a broker: a Resource with a trusted operation
  whose own contract allows only what it should, such as registering instances of one approved
  template. The actor is the consumer of a Conduit to the broker that
  lists that operation, and of none to `management`; the broker's implementation makes the
  onward request through the broker's own Conduit to `management`, which authorizes it (ACC-016,
  ACC-033). The narrowing is then the broker's, kept by its implementation and not by the
  engine, and the broker's own Conduit to `management` is as wide as any. The actor's Conduit to
  the broker applies no filter: the onward request would carry it, `management` cannot apply it,
  and the request would report unavailable (ACC-017, ACC-015). The scenarios S1 and S2 measure
  whether this is good enough; where it is clumsy, a focused constraint, such as a list of the
  templates that a Conduit to `management` allows, comes back to the owner (ruling 44).

### What the guarantee for secrets covers

This is a statement of the limit of the promises on secrets, not a promise, and nothing tests
it. It is section 4 of the [principles review](../docs/proposals/principles-review/README.md),
which ruling 41 accepted, with the limits that follow from the decisions and answers that
ruling 42 accepted and from ruling 43.

- The machine that the engine runs on and its operating-system user, and the place where the
  chosen runtime provider keeps a run's values (PRV-011), are inside the domain of trust of
  every secret the engine reads. So are the run's machine and its gate, for what that run
  serves (PRV-011, PRV-025), and the place where the signing keys of DL13 are held, with its
  backups and operators (DL13).
- Programs that run as the engine's operating-system user are inside the engine's domain of
  trust, and can get past every gate: they can read the engine's keys and make its proof
  (IFC-022, PRV-025). The processes of the engine's own runner are such programs wherever it
  runs them as that user, which DL16 states and which is usual (PRV-003, DL16), and a user who
  names no runtime gets that runner. An Agent that is not to be trusted that far is given a
  runtime provider that runs it apart, such as a container provider on `local` (PRV-006).
- No consumer can present as another, except a program that can read the credential of
  another instance's run: root, and every program of the operating-system user that the run
  runs as (IFC-022). Runs that share one operating-system user, as those of the engine's own
  runner usually do, can so present as one another.
- A value in a process's environment is readable by that process, by what it starts, by root,
  and by the user's other processes that the kernel admits, and it serves every caller of that
  process alike.
- An Agent's model that can run commands can read its own credential, and only the root
  Interface's choice of which Conduits to a secret Resource it opens limits which secret
  reaches an Agent's process through the engine (SEC-008).
- Only an enforcement point reaches the server of a secret Resource that the engine does not run
  itself, other than a `plain` implementation of `get-secret.v1` reached by `remote`, and the
  request and the value it returns cross between them confidential and unaltered (PRV-025,
  PRV-028, IFC-017, SEC-006). For a server reached by `remote` that is not `plain`, which is its
  own gate, this holds as far as its operator has configured it to admit only the engine's proof
  (IFC-041). A `plain` one stands behind no gate: whatever can reach its URL can send it a
  locator of its own choosing and read every secret that its store's credential reaches, and
  SEC-024 keeps it at an `https` URL, or at an `http` URL on loopback on an engine that can
  neither restart on another machine nor run as several parts (SEC-006, SEC-024). What the gate
  still admits on the server's machine, and who can make the engine's proof, are the points on
  the gate and on the engine's operating-system user.
- A copy that a store's package keeps is outside the engine (SEC-006, SEC-017).
- A value that an implementation returns or sends is data from then on (SEC-019).
- Opening a Conduit that lists `get-secret.v1` also trusts the secret Resource's locator,
  which any actor that may register that Resource writes (SEC-005).
- A secret bound to an argument is in the arguments of the process, which process listings
  show (PRV-007, SEC-010).
- A value rotated in its store reaches an instance's implementation only in a later run, one
  that a further setup, or a retirement that is to send the retire hook, asks for once the run
  has ended, or one that PRV-026 makes again for a provider after the engine restarts, or the
  part of it that held the run stops. It reaches a `remote` item only at a later connection,
  which a further setup or such a retirement makes, or at the read again of SEC-023 once the
  engine has lost a connection's values, as after it restarts (SEC-014, SEC-013, SEC-023,
  PRV-026, TPL-018). No operation ends the run of an instance that is ready, so through Conduit
  alone a running instance gets the new value only in another instance made in its place once it
  is retired (LIF-009), and retiring it closes the Conduits of which it is an end (CON-008).
- The engine's own runner passes a process it starts, of the engine's environment, only `HOME`,
  `LOGNAME`, `PATH`, `SHELL`, `TERM` and `USER`, or on an operating system such as Windows the
  variables that DL16 names in their place, each only where its value does not begin with `()`,
  and the variables that a list set on the runner names (PRV-023, DL16). A variable that the
  list names reaches every process that the runner starts, outside every Conduit; one that it
  does not name, such as a proxy setting or `KUBECONFIG`, is passed to none unless the item
  binds it, the servers of the providers that the runner runs among them (PRV-002). A process
  that runs as the engine's operating-system user may still read the engine's own environment,
  as the point on a process's environment says, so a token belongs in a store and not in the
  engine's environment.
  Under any other runtime provider, what else a run's process receives from where it runs is
  that provider's (SEC-020).
- A run is given, in `identity`, a credential that presents as its instance and lasts as long
  as the instance, and the place where its runtime provider keeps it, and the run's machine,
  hold that credential while the instance exists (PRV-022, DL13). The time at which it lapses
  is set so far ahead that it does not lapse meanwhile, so whoever holds it can present as the
  instance from anywhere until the instance is removed. Once the instance is removed it
  establishes no instance, since the engine checks on every request that the instance it names
  still exists, which is the only revocation (IFC-022, IFC-032, DL13).
- Once the engine has lost the values of a connection to a `remote` item, as after it restarts,
  it reads again the secret Resources that the setup which made the connection read for its
  headers, after that setup has ended, for that setup's caller, as the instance's own Conduit
  and its filters decide then; closing that Conduit stops it (SEC-023). Once it has restarted,
  or the part of it that held a provider's run has stopped, it reads in the same way the secrets
  of a provider whose run ended with it, a secret Resource that its own runner ran among them,
  before it runs that provider again (PRV-026).
- A header that carries a secret, and the value that an implementation of `get-secret.v1`
  reached by `remote` returns, cross only an `https` URL, or an `http` URL whose host is a
  loopback address on an engine that, as configured, can neither restart on another machine nor
  run as several parts. An engine that can do either rejects every `remote` item at a loopback
  host, and once an engine is configured so, it sends nothing to such an item, and no such
  header to an `http` URL, whenever the template was registered (SEC-024, SEC-013, IFC-041,
  DL17). There each crosses the loopback of the machine that the engine runs on in clear, on
  every HTTP request of the connection, to whatever listens at that address, a process of
  another operating-system user among them. Only a plain server stands at an `http` URL
  (IFC-044): a local server that takes Conduit's keys runs over stdio on the engine's own
  runner.
- A gate keeps out everything on a run's machine but root and the operating-system user or
  group that the runtime provider uses there for its own means, which its contract names and
  as which, or in which, no run runs (PRV-025).
- A run left on a machine whose node was declared lost, or that was reported gone, keeps
  the values it was given until something stops it. The declaration of loss asks for no stop
  (PLC-017). The engine asks its runtime provider for the stop in each later retirement of the
  instance and once more when the instance is unregistered (PRV-008, LIF-036), and what the
  provider does with a stop that it cannot carry out then, such as carry it out once the
  machine comes back, is what its contract states, with where it keeps that stop, in a place
  that neither a restart of the engine nor the provider's own retirement loses (PRV-021,
  PRV-020): once the instance is removed, the engine asks for nothing more. So a stop that the
  provider never received, as where its server was not running, is lost. A stop that only the
  provider's own server carries out waits while that server does not run, as after the engine
  restarts while no call needs that provider, and is never carried out once the provider is
  retired; a contract that keeps its stops through both has something that runs without that
  server carry them out, such as the provider's own part on the run's machine (PRV-021).

## Checking

Run these from the root of the repository:

```sh
python3 tools/set_names.py --check   # the names under spec/ agree with names.yaml
python3 tools/check_schemas.py       # schemas, core package and examples
git diff --check                     # whitespace errors
python3 tools/check_docs.py          # links, in every Markdown file of the repository
```

The first three read `spec/`. The first checks that every file under `spec/` writes the names
of [names.yaml](names.yaml) as that file states them. The second reads every file of
documents under `core/` and `schemas/` as UTF-8 and as strict YAML or JSON, checks every schema
against [schema-profile.md](schema-profile.md), validates every document of the core package and
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
rather than defining its own. This holds for everything that is not connection and access: the
schemas, the packages and the wire, and the infrastructure that the provider contracts reach,
whose shapes follow the Container Runtime Interface and the node lifecycle and well-known
labels of Kubernetes, Git's remote helpers, and the typed stores of External Secrets Operator
and Dapr. The proof of who is who takes the forms of SPIFFE's X509-SVID and JWT-SVID, and the
protocol's own authorization, and a gate is made of existing tools, such as a mutual TLS proxy,
a service mesh or the protocol's authorization, as each provider's contract states (DL13,
PRV-025). [schema-profile.md](schema-profile.md) lists what is relied on for documents, and the
comment at the head of each promise file names the standards that its promises follow.

## History

This section is the record of how the specification came to stand as the sections above
describe it. Nothing in it binds that the promises do not state.

The owner accepted the specification as it stood at
[ruling 39](../docs/proposals/ruling-39/README.md), of 2026-11-06, which accepted every
decision of the coordinator and every choice of an editor that the records log, before and
since ruling 37. [Ruling 40](../docs/proposals/ruling-40/README.md) and
[ruling 41](../docs/proposals/ruling-41/README.md), of the same day, then changed it: Conduit
innovates only on connection and access, and everything else is a provider behind a contract
that the user picks. At ruling 41 the owner accepted, as recommended, every decision of the
[principles review](../docs/proposals/principles-review/README.md), and the secret type as its
section 4 reduces it, and then the planner's choices of the
[work order](../docs/proposals/ruling-41/PLAN.md) that applies them, with one changed. The
specification as it stands is that rewrite. The logs of its editors, beside the work order
in `docs/proposals/ruling-41/`, list every change, the choices that the editors made beyond
the work order, which the coordinator confirms or strikes, and the questions they left open.
A [coverage check](../docs/proposals/ruling-41/log-integration.md#coverage-check) then read
the rewrite against every decision; its log lists what it fixed, and the questions it left were
settled in the [open questions after the rewrite](../docs/proposals/ruling-41/open-decisions.md)
and by rulings 42 and 43. [Ruling 42](../docs/proposals/ruling-42/README.md), of the same day,
struck none of the decisions that the coordinator settled on the
[questions the rewrite left open](../docs/proposals/ruling-41/open-decisions.md), and accepted
the recommended answers to seven of the nine put to the owner; the logs of its editors are
beside its record. The other two, D5 and D20, are about work across machines, which the owner
asked to have researched further; ruling 43 has since ruled them, as the end of this paragraph
says. The secret type is ruled: its promises are [promises/secrets.yaml](promises/secrets.yaml),
and the [secrets proposal](../docs/proposals/secrets/README.md) is the reasoning behind it. The
names that the review and the work order coined are settled: ruling 42 also accepted the names
and the editorial fixes that the
[vocabulary audit](../docs/proposals/vocabulary-audit/README.md) proposes. The names are
applied, and so are the editorial fixes, as the
[log of the editorial fixes](../docs/proposals/ruling-42/log-editorial.md) lists. The audit's
names for machines and placement, N2, N3 and N8 to N11, are as the
[log of their renames](../docs/proposals/ruling-42/log-rename-a.md) lists: a node, of the type
`node.v1`, represents a machine that the machine provider reports; a Type or an implementation
item states its label selector as `nodeSelector`, of the schema `labels.v1`; `run.v1` gives the
machine in its input `machine`; and the engine declares a node lost when its machine is
reported gone. The others, N1, N5 to N7 and N12 to N20, are as the
[log of the other renames](../docs/proposals/ruling-42/log-rename-b.md) lists: a secret
Resource's type offers `secret-provider.v1`, whose operation is `get-secret.v1`; `run.v1`
returns `url`; a `package` item binds `packageArguments` and is pinned by its Package's
`fileSha256` or, for an `oci` image, by a digest in its `identifier`; a caller attaches an idempotency key; a record and a condition name their source;
the filters a request carries are sent in `io.github.shrav-k.conduit/filters`, under Conduit's
extension `io.github.shrav-k.conduit/implementation-interface`; and an engine documents its
choices in its conformance document. A
[verification](../docs/proposals/ruling-42/log-integration.md#verification) of the result then
fixed what three readers found that follows from ruling 42; what it left was settled in the
[record of ruling 42](../docs/proposals/ruling-42/README.md#after-the-ruling) and in
[the items that waited on ruling 43](../docs/proposals/ruling-43/log-integration.md#the-items-that-waited-on-this-ruling),
apart from the few points that the tracker lists
[for the next reader](../docs/AUDIT.md#for-the-next-reader).
[Ruling 43](../docs/proposals/ruling-43/README.md), of the same day, then accepted the
[proposal on work across machines](../docs/proposals/multi-machine/README.md), with two
revisions, which rules D5 and D20 and five smaller questions and settles what that verification
left for it. The logs of its editors, and the
[log of its integration](../docs/proposals/ruling-43/log-integration.md), are beside its record;
the [verification](../docs/proposals/ruling-43/log-integration.md#verification) at the end of
that log fixed what three readers found that follows from the ruling; what it left was settled
in the record's section
[After the ruling](../docs/proposals/ruling-43/README.md#after-the-ruling).

The records under `docs/proposals/` say who decided what and when: the owner's rulings, listed
in the [ruling log](../docs/AUDIT.md#ruling-log), and the decisions of the session that
coordinates the re-audit ([who decides what](../docs/AUDIT.md#who-decides-what)), logged in
[GAPS.md](../docs/proposals/docs-audit/GAPS.md) and in the decisions of
[round 2](../docs/proposals/reaudit/round-2/DECISIONS.md), of
[round 3](../docs/proposals/reaudit/round-3/DECISIONS.md), of
[round 4](../docs/proposals/reaudit/round-4/DECISIONS.md) and of
[round 5](../docs/proposals/reaudit/round-5/DECISIONS.md), with the choices that its
[fix log](../docs/proposals/reaudit/round-5/log-fixes.md) records, and of
[round 6](../docs/proposals/reaudit/round-6/DECISIONS.md), with the choices that the logs beside
it record, and since ruling 39 also in the
[open questions after the rewrite](../docs/proposals/ruling-41/open-decisions.md) and in the
sections "After the ruling" of the records of
[ruling 42](../docs/proposals/ruling-42/README.md#after-the-ruling) and
[ruling 43](../docs/proposals/ruling-43/README.md#after-the-ruling). The owner can strike any
decision or choice made since ruling 39 ([who decides what](../docs/AUDIT.md#who-decides-what)
lists where they are). The comment at the head of each file says which pass changed which
promise. Later work is left out of this version
([ruling 38](../docs/proposals/ruling-38/README.md)).

The rulings of the [consolidation audit](../docs/proposals/consolidation/README.md#rulings), the
decisions of the [re-audit](../docs/proposals/reaudit/README.md), the
[ruling on the core package and the schemas](../docs/proposals/core-and-schemas/APPLY.md), the
[decisions of the docs audit](../docs/proposals/docs-audit/GAPS.md), which converted the
behaviors that the older text accepted and the specification did not state, the
[decisions of the second round of the re-audit](../docs/proposals/reaudit/round-2/DECISIONS.md),
the [decisions of its third round](../docs/proposals/reaudit/round-3/DECISIONS.md), the
[decisions of its fourth round](../docs/proposals/reaudit/round-4/DECISIONS.md),
[ruling 38](../docs/proposals/ruling-38/README.md),
[ruling 39](../docs/proposals/ruling-39/README.md),
[ruling 40](../docs/proposals/ruling-40/README.md),
[ruling 41](../docs/proposals/ruling-41/README.md),
[ruling 42](../docs/proposals/ruling-42/README.md), with the names and the editorial fixes of
the vocabulary audit, [ruling 43](../docs/proposals/ruling-43/README.md), the
[decisions of the fifth round of the re-audit](../docs/proposals/reaudit/round-5/DECISIONS.md),
with the [verification of its fixes](../docs/proposals/reaudit/round-5/log-fixes.md#verification),
and [ruling 44](../docs/proposals/ruling-44/README.md), on its sixth round, have been applied to
the promises.

[Ruling 44](../docs/proposals/ruling-44/README.md), of 2026-11-07, then settled
[round 6](../docs/proposals/reaudit/round-6/README.md) of the re-audit, the owner's cold read by
Codex of `main` at `c4d7d33`, whose two reports are six findings and a critique of whether
Conduit is simple and useful. As the ruling accepted them: an error -32021 for a client
capability that a call did not declare is read as any other error, unless Conduit's own contract
binds the server to have done nothing, as the new IFC-047 binds an implementation that carries
Conduit's keys (IFC-010, IFC-045, IFC-047, OPS-007); a repeat to which a filter applies is never
answered with outputs that the filter has not judged, and one of an operation that states no
effect, is not a control and starts no model work is dispatched again so that the filter decides
afresh (OPS-009, OPS-024, OPS-034, ACC-008); no two pointers of a control's `changes` overlap
(CTR-006); the schema of a tool that the engine presents is a self-contained bundle (IFC-033);
the schemes that a package-source provider claims are fixed for its life (PRV-016); and the
entry points no longer say that a point of round 5 waits for the owner. Of the four points that
were left open, a type's default for a hook that it does not require is not used, and a
diagnostic reports it (TPL-004, CTR-004); the tracker keeps the other three. Of the critique's
simplifications, a tool shows the narrowing and the controls of the target's template (D07;
IFC-033, IFC-012), replacing a template resolves the project handed over with it as registration
does (D11; SUB-009), and an ordinary Operation's schemas may use the whole profile at their root
(D14; CTR-012). The access convention states, as limits of this version, that a bound does not
limit filters and that access to `management` cannot be narrowed to one template, with the
broker that narrows it (ACC-029, ACC-030;
[What access does not narrow](#what-access-does-not-narrow); D03, D04); ACC-033 lists its
routing as one decision list, with no change of behavior (D18); and this page puts the current
design first, with [reading paths](#reading-paths) (D21). Two further simplifications, as the
owner revised them after the ruling
([After the ruling](../docs/proposals/ruling-44/README.md#after-the-ruling)), are applied, as
[their log](../docs/proposals/reaudit/round-6/log-d06-d12.md) lists: no Operation contract
states a default implementation, a template's selection or its type's default chooses what
runs, and the engine implements the operations that the core package's implementation `engine`
lists, with no selection (D06; CTR-002, TPL-003, TPL-004); and names stay the only address,
LIF-031 unchanged, while `set-up.v1`, `retire.v1`, `inspect.v1` and `unregister.v1` take an
optional `expectedId`, which reports conflict where the object named has another ID (D12;
LIF-037). Round 6's [decisions](../docs/proposals/reaudit/round-6/DECISIONS.md) list
what changed for each item, and the logs of its editors, with
[the integration](../docs/proposals/reaudit/round-6/log-integration.md) that made their files
agree, are beside them; the integration's
[verification](../docs/proposals/reaudit/round-6/log-integration.md#verification) lists what two
further readers of the result found and what was fixed.
