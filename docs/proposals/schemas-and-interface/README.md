# Document schemas, the implementation interface and the implementation-defined list — accepted

Status: accepted by the owner on 2026-11-05 (plan-audit ruling 31, action A58). Every
recommendation was accepted, with decision I1 amended as [stated below](#the-ruling-on-i1).
Written in the specification: [templates](../../../spec/promises/templates.yaml),
[contracts](../../../spec/promises/contracts.yaml), additions to the object, graph, Conduit, Pool
and package promises, the [implementation interface](../../../spec/interface/implementation.yaml)
and the [implementation-defined list](../../../spec/implementation-defined.yaml). This was the last
batch of rulings before the promises are consolidated and the specification is audited again.

This is the record of a ruling. The specification in [`spec/`](../../../spec/README.md) governs
where they differ. Amended by:

- ruling 32: a control is an Operation contract that also states the settings it changes, so a
  template's `controls` map is keyed by the control's qualified reference, which changes D6;
  every request names its Conduit and carries the restrictions that apply, and an onward request
  names the invocation it serves, which adds keys to I2; a plain implementation never counts as
  deduplicating; a thirteenth entry joined the implementation-defined list;
- ruling 33: decision X1 is settled, and the names come from the repository, not `conduit.dev`;
  there is a fifth kind of contract, `Schema`, which changes D11; changes to one instance are
  applied one at a time, which changes entry DL9; a tool's name also holds the name of each
  component, which changes I6; an implementation that carries Conduit's keys may answer with a
  reason in place of an outcome;
- ruling 37: the core package, the JSON Schemas and the manifest's format are written; the
  core package's reference schemas carry a version segment, as in `resource-reference.v1`.

Promise IDs are as first written; the comment at the head of each spec file says where a removed
ID went.

The inventory behind it was taken from the specification, the accepted records, `docs/protocol`
and the fixtures of Experiments 001 to 003. Where the accepted text already decides something it
is restated without a decision. The decisions are the points with no ruled shape, or with two
conflicting shapes.

## A name the owner controls

> Later ruling: ruling 33 settled X1. Until there is a domain the names come from the
> repository: the core package is `github.com/shrav-k/conduit/core`, a document's `apiVersion`
> is `github.com/shrav-k/conduit/v0`, and Conduit's `_meta` keys begin
> `io.github.shrav-k.conduit/`. They are defined once, in
> [`spec/names.yaml`](../../../spec/names.yaml). `conduit.dev` is no longer used.

| ID | Decision | Options | Recommendation |
| --- | --- | --- | --- |
| X1 | Three things need a name under a domain the owner controls: the core package's address, the `apiVersion` of a document, and the prefix of the keys Conduit adds to a Model Context Protocol request. The specification currently writes `conduit.dev`, which is an assumption. | (1) The owner names a domain. (2) Keep `conduit.dev` as a placeholder and settle it before any release. | (2) unless the owner already holds a domain. It blocks nothing now, but it must be settled before the engine builds, because the builds will hard-code it. |

## Part 1: the shape of documents

### What is already decided

A template is `kind`, `metadata` and `spec`. An instance is `kind`, `metadata`, `from`, its
standard arguments and `arguments`. Inside `spec`: `inputs` with `schema`, `to`, `default` and
`optional`; `type`; `config`; `conduits`; a setup graph whose steps have `operation`, `arguments`
and `optional`; selections as qualified references; a Pool's `entries`; a Conduit's capability,
operations and filters. Fields the engine owns are never written.

### Decisions

> Later rulings changed two rows below.
>
> - D6: a control is an Operation contract that also states the settings it changes. A type
>   lists its controls by reference, and a template's `controls` map is keyed by the control's
>   qualified reference, with an `inputs` map that narrows the control's inputs (ruling 32,
>   decision G11 of the re-audit; CTR-006, TPL-010).
> - D11: there are five kinds of contract, with `Schema` (ruling 33, decision F21 of the
>   re-audit; CTR-001, CTR-008).

| ID | Decision | Options | Recommendation | Existing practice |
| --- | --- | --- | --- | --- |
| D1 | Where an implementation is selected. The sources disagree: an alias map, a key on each step, and a step that names only its operation. | (1) One `implementation` map in `spec`, keyed by what is implemented, an operation's qualified reference or a runtime's name. Its value is a qualified reference to an implementation, or an inline graph. Anything absent falls back to the default its type or its operation's contract states. Steps, runtimes and controls never carry their own selection. (2) A key on every step, runtime and control. | (1). A step then names only its operation, as the naming record's example already does, and overriding an implementation is one line. | Terraform picks a provider from the resource type and lets one line override it |
| D2 | Spelling conventions for keys. | (1) camelCase keys; a step's dependencies under `dependsOn`; every document carries `apiVersion`, and one with an unknown version is rejected; `metadata` holds `name`, an optional `description`, and optional `annotations`, a map of strings with no meaning to the language. (2) snake_case, as Compose and Terraform write. | (1). The envelope is already Kubernetes' and the fixtures already use camelCase. | Kubernetes |
| D3 | How a value reference is written, beyond `source`. | A closed list of five sources, each a small map: `{source: input, name}`, `{source: config, pointer}`, `{source: step, step, output, pointer}`, `{source: runtime, name}` for a handle, and `{source: instance, name}` for the instance's own `id` or `name`. No string interpolation. | As stated. The four binding forms are written `literal`, the reference map, `object` and `list`. | Structured references rather than `${...}` strings, which the instantiation ruling already excluded |
| D4 | What a runtime declaration holds. | (1) A named pair of operations: `runtimes: {watcher: {start: <operation>, stop: <operation>, optional: true}}`. The start operation returns one handle, which the instance keeps; the stop operation takes it. A setup step starts a runtime by naming it, `{runtime: watcher, arguments: {...}}`, in place of `operation`. (2) A fifth kind of contract with its own start, stop and inspect methods in the implementation interface. (3) No runtimes in the language; a long-lived process is the implementation's private matter. | (1). It adds no contract kind and nothing to the interface, and the engine still knows what an instance holds and must stop. | The Model Context Protocol's guidance for stateful tools: one tool creates and returns a handle, others take it |
| D5 | Whether a template can say how its instances are retired. | (1) An optional `retire` graph, the mirror of `setup`. Without one, retiring stops the runtimes the instance started and nothing else. (2) Retirement is always the default. | (1). Shutdown is "what its type contract defines", and a graph is how a template states an operation. | Terraform providers implement destroy beside create |
| D6 | Where controls are defined. | (1) Only by the type contract: a name, its inputs and the settings it changes. A template's `controls` map only chooses which are available and may narrow a control's input with a schema. Omitting the map makes all of the type's controls available. A template cannot invent a control. (2) A template may also define controls. | (1). A template narrows its type and never widens it, which is already a promise. | Kubernetes: the resource definition fixes which fields exist |
| D7 | The Conduit template. | `capability`: exactly one qualified reference, fixed in the template. `operations`: a list of qualified references, each of which the capability must include. `requirements`: a map of named applied requirements, each `{requirement: <reference>, arguments}`; a filter is a requirement whose contract narrows requests, and the shared-host requirement goes in the same map. A composite has `components`, a map of nested instantiations, and no capability of its own. A Conduit template never fixes a consumer or a target. | As stated. One key for every requirement on a Conduit removes the question of where shared-host goes. | Kubernetes Role rules list verbs per resource |
| D8 | Which families have a `type`. | (1) Interface, Agent, Resource and Pool always do; the core package exports a plain Interface type and a plain Pool type. A Conduit has none: its capability plays that part. (2) Every family. | (1). | — |
| D9 | What setting up a Pool does. | (1) Setting up a Pool sets up the Resources of its entries, as steps of the Pool's setup, in an order in which each comes after the entries it refers to. An entry may be marked `optional: true`. Setting up one Resource directly still sets up no other. References among entries may not form a cycle. (2) Setting up a Pool touches no entry. | (1). It is the same rule as a declared Conduit being opened by its consumer's setup, and it gives one request that brings a declared system up. It reverses one choice made while writing ruling 29, that entries may refer to each other in a cycle. | `docker compose up` |
| D10 | Family-specific fields for an Interface and an Agent, and policies written in templates. | None. Accounts, models, frontends and isolation are configuration of the type. A policy is applied only through `access`; the old "permission limits" on an Agent template and "policies" on a Pool template are dropped. | As stated. | — |
| D11 | How contracts and packages are written. | Contracts are documents with the same envelope and four kinds: `Type`, `Capability`, `Operation` and `Requirement`. The manifest holds the package's own `address` and `version`, the `packages` it uses, `exports`, the names of the documents it makes public, and `implementations`, each stating what it implements and how it is reached. A document a package does not export cannot be referred to from outside it. | As stated. One loader and one schema profile then cover everything an author writes. | Kubernetes resource definitions are documents; Go's `module` line states a module's own address |

### What each contract holds

Every field traces to a promise that already exists.

| Kind | Fields |
| --- | --- |
| `Operation` | `inputs` and `outputs`, each with a schema and `optional`; whether an output is a handle; the `effect` it declares, or none; `startsModelWork`; `deduplicates`; a default implementation. |
| `Capability` | The `operations` it includes. An operation reserved to the root Interface can be listed only by the core package. |
| `Type` | Its family; the schema of `config`; the capabilities it `offers`; the operations and runtimes it requires, with default implementations; its controls; its requirements on placement, stated even when there are none; the further conditions it reports, such as running or paused; whether its implementation may restore its own runtime; what content its records keep. |
| `Requirement` | `inputs`; where it may be applied, on a Conduit, as a policy or on placement; whether it narrows requests; and, for a composition, its constituents. |

### Example

> Later rulings changed three shapes in the example below.
>
> - `conduit.dev` names are now derived from the repository (ruling 33;
>   [`spec/names.yaml`](../../../spec/names.yaml)): `apiVersion` is
>   `github.com/shrav-k/conduit/v0`, and the core package is `github.com/shrav-k/conduit/core`.
>   A package outside the core, written here as `conduit.dev/files`, has its own address.
> - A template's `controls` map is keyed by the control's qualified reference, and narrows an
>   input under `inputs`, as in `inputs: {limit: {schema: {maximum: 50}}}` (ruling 32; TPL-010).
>   It is not keyed by a bare name such as `set-read-limit`.
> - The reference schema is `github.com/shrav-k/conduit/core#resource-reference.v1` (rulings 33
>   and 37).
>
> The current worked example is
> [`spec/schemas/examples/valid/project/`](../../../spec/schemas/examples/valid/project/).

```yaml
apiVersion: conduit.dev/v0
kind: Resource
metadata: {name: project-directory}
spec:
  type: conduit.dev/files#local-directory.v1
  inputs:
    path: {schema: {type: string, minLength: 1}, to: [/config/path]}
  config: {readLimit: 10}
  runtimes:
    watcher: {start: conduit.dev/files#watcher.start, stop: conduit.dev/files#watcher.stop, optional: true}
  setup:
    steps:
      establish:
        operation: conduit.dev/files#directory.establish
        arguments: {path: {source: config, pointer: /path}}
      watch:
        runtime: watcher
        arguments: {directory: {source: step, step: establish, output: directory}}
  controls:
    set-read-limit: {schema: {properties: {limit: {maximum: 50}}}}
---
apiVersion: conduit.dev/v0
kind: Conduit
metadata: {name: directory-read}
spec:
  capability: conduit.dev/files#directory-access.v1
  operations: [conduit.dev/files#directory.read, conduit.dev/files#directory.list]
  requirements:
    same-machine: {requirement: conduit.dev/core#shared-host.v1}
---
apiVersion: conduit.dev/v0
kind: Agent
metadata: {name: reviewer}
spec:
  type: github.com/acme/harness#coding-agent.v1
  inputs:
    checkout: {schema: {$ref: "conduit.dev/core#resource-reference"}, to: [/conduits/checkout/target]}
  conduits:
    checkout: {from: directory-read}
```

## Part 2: the implementation interface

### What is already decided

The Model Context Protocol is the base for invoking an implementation. Conduit's context travels
in the request's metadata, outside the operation's own arguments. Each request states the version
of that context, with no prior negotiation. An existing server that knows nothing of Conduit stays
usable. Work that finishes later is an ordinary tool that returns a reference. Internal parts of
an engine need not speak the protocol to each other. An implementation's result states its
outcome, and the engine may change it only to unknown.

### Decisions

> Later rulings changed two rows below.
>
> - I2: a request's `_meta` also holds the engine's ID for the invocation, the Conduit and the
>   restrictions that apply, and an onward request names the invocation it serves (ruling 32;
>   IFC-006, IFC-020). A result may carry a reason in place of an outcome (ruling 33; IFC-026).
> - I6: a tool is named by the Conduit's local name, then the name of each component for a
>   Conduit made from a composite template, then the operation's export name, joined by dots
>   (ruling 33; IFC-012).

| ID | Decision | Options | Recommendation | Existing practice |
| --- | --- | --- | --- | --- |
| I1 | The wire. | (1) An implementation is a Model Context Protocol server at revision 2026-07-28, the current one. Each operation it implements is one tool, named by the operation's export name. The tool's input schema is the operation's inputs and its structured content is the operation's outputs. (2) A wire of Conduit's own. | (1). That revision has no sessions and carries everything per request, which suits "an implementation's behavior for one instance does not depend on another". Export names were already restricted to what a tool name allows. | The protocol's own tools |
| I2 | What Conduit adds to a request and a result. | Keys in the request's `_meta` under Conduit's prefix: the interface version, the instance's ID, the operation's qualified reference, the operation identity and the caller's ID. One key in the result's `_meta`: the outcome. A result without it is recorded as unknown. | As stated. | The protocol's own `io.modelcontextprotocol/` keys in `_meta` |
| I3 | How a plain server's answer becomes an outcome, since it states none. | (1) A completed result that is not an error is succeeded. An error on an operation that declares no effect is failed. A protocol error saying the tool is unknown or the request malformed is failed. Any other error, and no answer, are unknown. (2) Every result from a plain server is unknown. | (1). Under (2) no existing server would be usable. | The protocol's `isError` and its two kinds of error |
| I4 | Where an implementation says what it supports: which operations it implements, which requirements it can apply, its requirements on placement, and for a plain server which tool serves which operation. | (1) In its package's manifest, readable without running anything. (2) Discovered from the running server. | (1). It keeps "can this Conduit be opened" answerable from documents. | — |
| I5 | Handles on the wire. | An opaque string that the implementation issues and the engine passes back unchanged. It grants nothing, and the implementation checks it on every call. | As stated. | The protocol's guidance for stateful tools, word for word |
| I6 | How a consumer reaches its targets. | (1) The engine presents to each Agent or Interface a Model Context Protocol server whose tools are exactly the operations its open Conduits list, each named by the Conduit's name, a dot and the operation's export name. Closing a Conduit removes its tools. (2) Left to each harness. | (1). It is the accepted "Conduit-managed proxy", it makes access visible to any existing agent harness, and it closes the known gap that nothing described how an implementation calls onward. | The protocol lets a tool list vary by the caller's authorization |
| I7 | Transport and process layout. | The protocol's own transports. Which one, who starts the process, and how many instances share a process belong to the implementation-defined list. | As stated. | — |

## Part 3: the implementation-defined list

An engine must document its choice for every entry, and its choice must keep every promise. That
document is the engine's runtime profile.

> Later rulings: the list now has thirteen entries. Ruling 32 added DL13, on how the engine
> establishes which instance a consumer is. Ruling 33 (decision F10 of the re-audit) took "how
> many run at once" out of DL9: changes to one instance are applied one at a time (REV-012).
> The list as it stands is
> [`spec/implementation-defined.yaml`](../../../spec/implementation-defined.yaml).

| ID | Entry | Defects it answers |
| --- | --- | --- |
| DL1 | Graph execution: the order among steps that do not depend on each other, concurrency, cancellation, the states a step passes through, and what happens to the remaining steps after one fails. | L3 to L8, L11, L12, L14, L15, L24, L25 |
| DL2 | Dispatch records: where the record is written before an implementation is called, how attempts are numbered, and what is done with a late result. | L54, L59, L62 |
| DL3 | Waiting for a person: expiry, and an answer given twice or during an outage. | L74 |
| DL4 | When `access` or `management` cannot be reached: whether a request fails at once or is held, and for how long. | L53, L55, L63, L65, L66, L124 to L127, L141 |
| DL5 | Locations: how unreachability is detected, and what happens when a location returns. | L67, L68 |
| DL6 | First start: what triggers it, what an interrupted or repeated one does, and how repair works. | L32, L34, L36, L40 to L42 |
| DL7 | Packages: where they are stored, how they are fetched and removed, and any editing mode. | L39, L43 |
| DL8 | Delivery of observations and records: feeds, positions and snapshots. | L70 to L73 |
| DL9 | Controls: how many run at once, what happens to a request for a busy instance, and the order in which several failed checks are reported. | L102 to L104, L106, L116 |
| DL10 | Limits: sizes, counts, time allowed for a call, and behavior when storage is exhausted. | L97, L114 |
| DL11 | Conduits during setup: what a consumer with some Conduits open may do, and how an optional Conduit that failed is opened later. | L155, L156 |
| DL12 | Transport and process layout for implementations. | — |

| ID | Decision | Recommendation |
| --- | --- | --- |
| L1 | Whether this is the list. | Accept it. Each of the 50 defects assigned to the runtime profile is then marked as answered by one entry, except L158, which ruling 30 already decided. |

## The ruling on I1

The owner asked that the wire be able to follow new revisions of the Model Context Protocol, and
ruled: one baseline revision that everyone supports, negotiation above it, and a new version of
this specification may raise the baseline. Written as IFC-002 to IFC-005:

- Every engine, and every implementation that carries Conduit's keys, supports the baseline,
  2026-07-28.
- Either side may support other revisions. The protocol's own per-request negotiation picks one
  that both support.
- Conduit's keys carry their own version, separate from the protocol's.
- The interface lists the four features of the protocol it relies on. A revision that keeps them
  needs no change here.
- Which revisions an engine supports beyond the baseline is entry DL12 of the
  implementation-defined list.

## Choices made while writing the rulings

The owner did not rule on these one by one. Each can be changed by a ruling.

| Choice | Why | Promise |
| --- | --- | --- |
| The `implementation` map is keyed only by operation, not also by a runtime's name. | Under D4 a runtime is two operations, so there is nothing else to key. | TPL-003 |
| A Conduit template has no `config`, `setup`, `runtimes` or `controls`. | Under D8 it has no type, and those are checked or defined by a type. What opening needs is done by the implementations that apply its requirements. | CON-016 |
| When an instance is retired, its `retire` graph runs first and then each runtime it still holds is stopped. | The graph's steps may need the runtimes' handles. | TPL-009 |
| In a setup graph, each declared Conduit and each Pool entry is a step under its own name, which a written step may depend on. | A step often needs a Conduit open first, and D9 needs entries ordered. | GRA-022 |
| A Conduit whose target's type does not offer its capability is rejected when it is instantiated, not when it is opened. | Both are known from the documents. | CON-021 |
| A tool presented to a consumer is named by the declared Conduit's local name, or by a standalone Conduit's own name; a standalone Conduit that would collide is not created. | A declared Conduit's instance name is generated and would make an unreadable tool name. | IFC-012, IFC-014 |
| An implementation that does not support the interface version performs nothing, and the outcome is failed. | It is confirmed that nothing happened. | IFC-008 |
| `apiVersion` is `conduit.dev/v0`. | Decision X1 kept `conduit.dev` as a placeholder. Superseded by plan-audit ruling 33: the names now come from `github.com/shrav-k/conduit`. | OBJ-017 |
| The JSON Schemas of documents, contracts and manifests are written after the specification has been audited again. | So that they are written once. | — |

## After this batch

The promises are consolidated: overlapping ones merged, each file checked against the
definitions, and the whole specification put back through the cold-reader, consolidation and
defect sweeps.
