# Conduit specification

Status: draft, begun 2026-11-05. Incomplete, and not a release. Every area has been written as
promises, and the core package, the JSON Schemas of documents and the manifest's format have
been written as documents; the parts still to come are listed under [Contents](#contents).

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
[fix log](../docs/proposals/reaudit/round-5/log-fixes.md) records, and since ruling 39 also in the
[open questions after the rewrite](../docs/proposals/ruling-41/open-decisions.md) and in the
sections "After the ruling" of the records of
[ruling 42](../docs/proposals/ruling-42/README.md#after-the-ruling) and
[ruling 43](../docs/proposals/ruling-43/README.md#after-the-ruling). The owner can strike any
decision or choice made since ruling 39 ([who decides what](../docs/AUDIT.md#who-decides-what)
lists where they are). The comment at the head of each file says which pass changed which
promise. Later work is left out of this version
([ruling 38](../docs/proposals/ruling-38/README.md)). The specification is still a draft,
because it has no conformance vectors and no reference model; it stays one until it has them,
the reference model agrees with it, and a round of readers finds nothing unsafe in it
(ruling 38, point 4).

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

Two things are deliberately outside it. How one type behaves, such as the conditions it defines
(running or paused, for example) or a directory's edge cases, belongs to that type's contract;
the one lifecycle that every instance has is not among them, and is a shared convention. How a
system gets from documents to running work, beyond what the promises fix, belongs to the
provider that does it, such as how a runtime provider starts a process or where a machine
provider places an instance, which that provider's contract states; and what stays the engine's
own, such as retries and the storage of its records, belongs to that engine's conformance
document, which is its documented choice for every item of the [implementation-defined
list](implementation-defined.yaml). Neither binds other engines.

One thing is left out of this version on purpose: work that outlasts its invocation. No
operation arranges, through the engine, work that starts or runs after its own invocation has
ended; work that is to happen later is asked for later, by an actor (ACC-019, ruling 38). The
requests that the engine makes on behalf of an invocation that has ended are two reads of a
secret: the read again for a connection that a setup made, once the engine has lost its values
(SEC-023), and, once the engine has restarted, the reads before it runs again a provider whose
run ended with it, for the setup that last asked for that run (PRV-026).

## Contents

| Path | Part | State |
| --- | --- | --- |
| [names.yaml](names.yaml) | The names this version assigns: the core package's address, `apiVersion`, the prefix of its `_meta` keys, and the names of what `identity` carries. The one place they are defined | Current |
| [definitions.md](definitions.md) | The terms the promises use | Draft; follows the promises as they stand |
| [promises/objects.yaml](promises/objects.yaml) | Names, IDs, references, kinds and the shape of a document | Draft; 24 promises; no vectors |
| [promises/schemas.yaml](promises/schemas.yaml) | Schemas, document syntax and numbers | Draft; 21 promises; no vectors |
| [schema-profile.md](schema-profile.md) | The standards relied on, the YAML features not accepted, the schema keywords in the profile and the formats that are checked | Draft |
| [promises/instantiation.yaml](promises/instantiation.yaml) | How a template and its arguments become an instance, and what `from` may be | Draft; 23 promises; no vectors |
| [promises/templates.yaml](promises/templates.yaml) | What the `spec` of a template holds: the type, implementations, hooks, retirement and controls | Draft; 14 promises; no vectors |
| [promises/contracts.yaml](promises/contracts.yaml) | The five kinds of contract and what each holds | Draft; 10 promises; no vectors |
| [promises/naming.yaml](promises/naming.yaml) | How packages, their exports and contracts are named, where versions live, and which version of a contract governs | Draft; 28 promises; no vectors |
| [promises/packages.yaml](promises/packages.yaml) | Versions, digests, the archive, what a package's manifest states, and what a request to install reports | Draft; 22 promises; no vectors |
| [promises/manifest.yaml](promises/manifest.yaml) | The manifest's format: the file `conduit.yaml`, `conduit.yml` or `conduit.json`, its keys, implementation items, which variables of a run they may not bind, which files of a project are its documents, and the layout of a package's archive | Draft; 17 promises; no vectors |
| [promises/submissions.yaml](promises/submissions.yaml) | Handing over several documents at once, the contracts of a submitted project and the address that names them, how a template is replaced, and that every engine takes a directory | Draft; 10 promises; no vectors |
| [promises/pools.yaml](promises/pools.yaml) | Ownership and entries | Draft; 15 promises; no vectors |
| [promises/placement.yaml](promises/placement.yaml) | Where an Agent or Resource runs: nodes, machines, labels and placement by the provider, and values tied to a node | Draft; 21 promises; no vectors |
| [promises/conduits.yaml](promises/conduits.yaml) | What a Conduit is, what its template holds, when it exists and when it is open | Draft; 22 promises; no vectors |
| [promises/first-start.yaml](promises/first-start.yaml) | The objects that exist before anything is submitted | Draft; 5 promises; no vectors |
| [promises/providers.yaml](promises/providers.yaml) | Providers: what one is and how the user picks one; the runtime provider and its runs, what a run is given to reach the engine, the gate before every server that the engine does not run itself, other than a `plain` server reached by `remote`, and how what answers at a run's URL proves itself, a stop that cannot be carried out now, and what the engine's own runner passes to a process; the machine provider, package sources, the provider call, and a provider's run made again after the engine restarts | Draft; 28 promises; no vectors |
| [promises/secrets.yaml](promises/secrets.yaml) | Secrets: how a document marks one, which Resources hold one and what else their type may offer, who may have one read, how the engine reads one before a run or a connection, and again once it has lost a connection's values, and where the value may go, a header that carries one among it | Draft; 24 promises; no vectors |
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
| [schemas/examples/valid](schemas/examples/valid) | Documents that the schemas accept: a submitted project, a package, and the instance documents of the first-start objects | Draft; 7 files in 3 directories |
| [schemas/examples/invalid](schemas/examples/invalid) | Documents that a schema rejects, or that are rejected when the file is read, one to a file, each file named for the promise it breaks | Draft; 56 files |
| [schemas/examples/invalid-by-rule](schemas/examples/invalid-by-rule) | Sets of documents that are valid against the schemas and are rejected by a rule no schema can express, each file named for the promise the set breaks | Draft; 30 files |
| [conventions/lifecycle.yaml](conventions/lifecycle.yaml) | The convention on ownership and lifecycle: the one lifecycle every instance has, what inspecting returns, retirement, removal and unregistration, and what reconciling a setup or a retirement changes | Draft; 35 promises; no vectors |
| [conventions/access.yaml](conventions/access.yaml) | The access convention: every permission is an open Conduit, decided anew for each request | Draft; 29 promises; no vectors |
| [conventions/operations.yaml](conventions/operations.yaml) | The convention on idempotency keys and outcomes | Draft; 31 promises; no vectors |
| [conventions/revisions.yaml](conventions/revisions.yaml) | The convention on revision-checked change | Draft; 14 promises; no vectors |
| [conventions/observation.yaml](conventions/observation.yaml) | The convention on declared against observed state | Draft; 5 promises; no vectors |
| [conventions/records.yaml](conventions/records.yaml) | The convention on records | Draft; 6 promises; no vectors |
| [conventions/implicit.yaml](conventions/implicit.yaml) | The convention that nothing starts model work or repeats an effect implicitly | Draft; 5 promises; no vectors |
| [interface/implementation.yaml](interface/implementation.yaml) | The implementation interface: the Model Context Protocol as the wire between an engine, its implementations and its consumers, with Conduit's keys as an extension of the protocol, and the proof of who is who in both directions | Draft; 43 promises; no vectors |
| [implementation-defined.yaml](implementation-defined.yaml) | What is left to each engine, which it documents as its conformance document | Draft; 15 items |

Every area has been written as promises, 452 in all: 202 of the layer `language`, 207 of the
layer `convention` and 43 of the layer `interface`. The rulings of the
[consolidation audit](../docs/proposals/consolidation/README.md#rulings), the decisions of the
[re-audit](../docs/proposals/reaudit/README.md), the
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
the vocabulary audit, [ruling 43](../docs/proposals/ruling-43/README.md) and the
[decisions of the fifth round of the re-audit](../docs/proposals/reaudit/round-5/DECISIONS.md),
with the [verification of its fixes](../docs/proposals/reaudit/round-5/log-fixes.md#verification),
have been applied to them. The JSON Schemas of
documents, contracts and manifests, the core package as documents and the manifest's file
format now exist: they are `schemas/`, `core/` and `promises/manifest.yaml`. Still to come: the
conformance vectors, and a reference model.

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
implementation, a provider's or a store's among them, claims the Implementation level, whatever
the layer of the promises that bind it.

| Level | Who claims it | What it covers |
| --- | --- | --- |
| Language | An engine | Every promise whose layer is `language`: documents in, resolved objects or diagnostics out. |
| Engine | An engine | The Language level, plus every promise whose layer is `convention` or `interface` and whose obligation falls on the engine, plus a conformance document that documents a choice for every item of the implementation-defined list. The promises on providers count among these where the engine's own runner, machine provider and fetching keep them because no provider serves (PRV-003). |
| Implementation | An implementation, a provider's among them | Every promise, whatever its layer, whose obligation falls on the implementation and not on the engine. Examples are IFC-008, IFC-009, IFC-019, IFC-026, IFC-031, SEC-006 and SEC-019. For a provider's implementation this includes the promises that bind the provider, which have the layer `convention`, such as PRV-010 to PRV-012, PRV-014, PRV-019, PRV-021, PRV-022, PRV-025 and PRV-028. For a server reached by `remote` that is not plain, it includes PRV-025. |

An engine that is given a document it cannot honor rejects the document and says why in a
diagnostic. It never accepts a document and ignores part of it.

## What the guarantee for secrets covers

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
rather than defining its own. This holds for everything that is not connection and access: the
schemas, the packages and the wire, and the infrastructure that the provider contracts reach,
whose shapes follow the Container Runtime Interface and the node lifecycle and well-known
labels of Kubernetes, Git's remote helpers, and the typed stores of External Secrets Operator
and Dapr. The proof of who is who takes the forms of SPIFFE's X509-SVID and JWT-SVID, and the
protocol's own authorization, and a gate is made of existing tools, such as a mutual TLS proxy,
a service mesh or the protocol's authorization, as each provider's contract states (DL13,
PRV-025). [schema-profile.md](schema-profile.md) lists what is relied on for documents, and the
comment at the head of each promise file names the standards that its promises follow.

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
   `promises/conduits.yaml` and `promises/first-start.yaml`.
5. What the engine reaches behind contracts: `promises/providers.yaml`, then
   `promises/secrets.yaml`.
6. The core package: [core/README.md](core/README.md), which lists every export with the
   promise that needs it, and then the files of `core/`. The first-start objects are instances
   of its templates, and the management verbs of the conventions are its operations.

A reader of the Engine level goes on to the conventions, beginning with
`conventions/lifecycle.yaml` and `conventions/access.yaml`, then to
`interface/implementation.yaml`, and last to
[implementation-defined.yaml](implementation-defined.yaml).
