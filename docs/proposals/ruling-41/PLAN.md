# Ruling 41: the work order for applying rulings 40 and 41 to the specification

Status: **a work order, not a ruling.** Written by the planner on 2026-11-06 for the coordinator
of pull request 7. It applies [ruling 40](../ruling-40/README.md) and
[ruling 41](README.md), which accepted every decision of the
[principles review](../principles-review/README.md) as recommended: section 2 (items 1 to 31,
each lettered line as recommended), section 3 (C33, C51, C52, C58, C14, C20, C21) and section 4
(the secret type, reduced, which governs where it and the
[secrets proposal](../secrets/README.md) differ). Where this plan and the rulings differ, the
rulings govern; once the work is done, the specification governs.

The planner had to fill gaps that the review leaves open, so that no editor has to. Each such
gap is listed under [Planner's choices](#planners-choices) as P1 to P20. None is ruled. The
coordinator confirms or strikes each before the editors start, and the vocabulary audit that
ruling 41 orders reviews every new name in this plan before pull request 7 is finished.

## Contents

- [How to use this plan](#how-to-use-this-plan)
- [Planner's choices](#planners-choices)
- [1. Terms](#1-terms)
- [2. New files](#2-new-files)
- [3. The editing groups](#3-the-editing-groups)
  - [G1 lifecycle](#g1-lifecycle)
  - [G2 placement and providers](#g2-placement-and-providers)
  - [G3 access](#g3-access)
  - [G4 operations](#g4-operations)
  - [G5 interface](#g5-interface)
  - [G6 language](#g6-language)
  - [G7 secrets](#g7-secrets)
- [4. Core package and schemas](#4-core-package-and-schemas)
- [5. Docs](#5-docs)
- [6. Decisions checklist](#6-decisions-checklist)
- [Counts](#counts)

## How to use this plan

Order of work. G1 to G7 work in parallel, each on its own files. Every ID that one group cites
from another is fixed here, so no group waits for another. Then the core-and-schemas editor,
then the docs editor. Then the coordinator integrates: it fixes every citation of a removed ID
that a section below lists under "Cited by", and runs the checks of
[AGENTS.md](../../../AGENTS.md#checks).

Files and groups. Every file under `spec/` belongs to exactly one editor:

| Editor | Files |
| --- | --- |
| G1 | `spec/promises/graphs.yaml`, `spec/promises/templates.yaml`, `spec/conventions/lifecycle.yaml`, `spec/conventions/observation.yaml`, `spec/conventions/implicit.yaml` |
| G2 | `spec/promises/placement.yaml`, `spec/promises/pools.yaml`, `spec/promises/first-start.yaml`, new `spec/promises/providers.yaml` |
| G3 | `spec/conventions/access.yaml`, `spec/promises/conduits.yaml` |
| G4 | `spec/conventions/operations.yaml`, `spec/conventions/revisions.yaml`, `spec/conventions/records.yaml` |
| G5 | `spec/interface/implementation.yaml`, `spec/implementation-defined.yaml` |
| G6 | `spec/promises/objects.yaml`, `spec/promises/contracts.yaml`, `spec/promises/schemas.yaml`, `spec/promises/naming.yaml`, `spec/promises/packages.yaml`, `spec/promises/manifest.yaml`, `spec/promises/submissions.yaml`, `spec/promises/instantiation.yaml`, `spec/schema-profile.md` |
| G7 | new `spec/promises/secrets.yaml` |
| Core and schemas | everything under `spec/core/` and `spec/schemas/` with its examples, `spec/names.yaml` (no change), `tools/check_schemas.py` |
| Docs | `spec/definitions.md`, `spec/README.md`, `docs/OVERVIEW.md` |

Rules for every promise, from
[CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#rules-for-every-edit):

1. Exactly the fields `id`, `layer`, `level`, `when`, `then`, `observable`, `vectors: []`;
   folded `>-` text indented four spaces; YAML lines of at most 96 characters.
2. IDs are never reused. A new promise takes the ID this plan gives it, and no other. An editor
   who finds that a promise this plan does not list must change asks the coordinator.
3. A removed promise is deleted and named in its file's head comment with the ruling that
   removed it and where its content went, or "nothing replaces it".
4. No promise moves between files. Where content moves, the old ID is removed and the new ID
   named here carries it. A layer may be changed in place where this plan says so.
5. Every request that is not dispatched names exactly one of refused, invalid, conflict and
   unavailable, in the order of OPS-023.
6. Write in the terms of [spec/definitions.md](../../../spec/definitions.md) as
   [section 1](#1-terms) changes them. A term this plan introduces is used exactly as section 1
   defines it.
7. Head comment: each file gains one line of the form "Ruling 41
   (docs/proposals/ruling-41/README.md, 2026-11-06), applying ruling 40 and the principles
   review, removed X, changed Y and added Z", with the IDs. Repo-relative paths only.
8. A rewrite keeps everything in the promise that this plan does not name, apart from
   sentences that cite a removed ID or a term that goes, which are cut or pointed at the ID
   this plan names in its place.

In this plan, "(5)" cites decision 5 of the review, "(8e)" its lettered line, "(C20)" an item of
its section 3, "(S4.5)" point 5 of its section 4, and "(R40 3e)" a point of ruling 40.

## Planner's choices

The review decides the direction; these fill what it leaves open. Each is the smallest reading
the planner found. The coordinator confirms or strikes each; the vocabulary audit renames.

| # | Choice | Why it is needed | Alternative |
| --- | --- | --- | --- |
| P1 | New names: operations `setup-hook.v1`, `retire-hook.v1`, `run.v1`, `stop.v1`, `inspect-run.v1`, `list-machines.v1`, `place.v1`, `fetch-package.v1`; capabilities `runtime-provider.v1`, `machine-provider.v1`, `package-source.v1`; schemas `handle.v1`, `record.v1`, `condition.v1`, `label-selector.v1`, `server-package.v1`, `server-remote.v1`; the standard argument `runtime`; item keys `package`, `remote`, `digest`, `environment`, `arguments`, `headers`; `_meta` keys `config` and `revision`; the extension `io.github.shrav-k.conduit/interface`; the Operation `spec` keys `inputSchema`, `outputSchema`, `idempotentHint`, `destructiveHint`, `openWorldHint`; the input `nested` of `register-replacement.v1`; the words "part" (of a setup or retirement), "remove" (an instance), "provider call" | The review names `secret-reveal.v1`, `reveal-secret.v1` and `secret-reference.v1` only | Names the vocabulary audit proposes |
| P2 | `spec/promises/graphs.yaml` stays as a file that holds no promise (`[]`), with a head comment naming its 26 removed IDs | The rule says a removed ID is named in its file's head comment | Delete the file and name the IDs in `spec/README.md` |
| P3 | One run per instance (2a) means a template whose selected implementations include two different items that are run (`command` or `package`) is rejected (TPL-019) | 2a names a run by the instance's ID, so an instance can have at most one | Name a run by the instance's ID and the item's name |
| P4 | A setup asks for the run only where the provider does not report it running (1c); a retirement asks for one only where a retire hook is selected and the run is not running (4c) | 1c leaves a running runtime as it is; 4c names a retirement that runs it for its hook | Every setup asks for a new run |
| P5 | The engine's invocation of `run.v1`, `stop.v1`, `inspect-run.v1`, `list-machines.v1`, `place.v1` and `fetch-package.v1` is a "provider call", not a request: no Conduit, no access decision; it is recorded; and no Conduit may name those three capabilities (PRV-004, PRV-005). A secret read is a request (S4.4) | Otherwise any Conduit to a runtime provider could run or stop another instance's run, and a call with no caller would return, against 4c | Make them ordinary operations that Conduits may list |
| P6 | Providers of runs, machines and packages run on the engine's own runner at `local` (PRV-002). The machines provider and package-source providers are owned by `system`, at most one machines provider, one package-source provider per URL scheme, with the schemes in its configuration under `schemes` (PRV-013, PRV-016) | 31b: "the machines provider is the one the root Interface registers", and a package source "is picked by the scheme of `fetch`"; something must record which scheme | A key in the Type contract for schemes; registration reserved to the root Interface |
| P7 | A location for a reported machine is named by the first label of its hostname, lowercased, where that satisfies OBJ-001 and is free; otherwise by OBJ-003 (PLC-026). The hostname itself is its label `kubernetes.io/hostname` | 8d names a location "by its hostname", which may hold dots | Always a generated name |
| P8 | A repeat of an invocation that succeeded, and whose outputs the engine no longer keeps (DL10), reports conflict with the engine's ID, as a repeat of one in progress does (OPS-032) | 6b makes `isError` false only with outputs, and the protocol requires them where an `outputSchema` is declared | Keep outputs as long as the invocation's state |
| P9 | `register-replacement.v1` names the instance that holds the nested instantiation and, in `nested`, its name (LIF-009) | 17 removes a closed Conduit at once, so the retired instance can no longer be named | Keep removed Conduits until replaced |
| P10 | A request to install a package at an address that a submitted project's manifest states reports conflict (PKG-029) | 11 forbids the other order only | Reject neither, and let the package shadow the project |
| P11 | A submitted project's contracts keep SUB-008's digest as the identity of one submission, which a template keeps (NAM-009); outside the project's own documents they are named `address#name` (11) | Templates must keep what they resolved after the project is edited | Drop the digest and keep only the latest submission |
| P12 | A Requirement contract loses `appliesTo` (C26, with 3a and 8a), and a Type's `placement` becomes optional | `appliesTo` would admit only `conduit`; an empty selector means nothing | Keep `appliesTo: [conduit]` |
| P13 | Inspection reports a run's state as the condition `Running` (PRV-009) | 2a has an actor settle a run of unknown outcome by reading its status | A member `run` in the status |
| P14 | An entry's Resource takes `runtime` only from an input whose destination is `/entries/<name>/runtime`, as its placement (POOL-009, INS-012) | 31b gives every Resource the argument; entries write no standard argument | Entries always run on the engine's own runner |
| P15 | A `package` item states `digest` (the SHA-256 of the artifact, written as PKG-004) beside the server.json Package; `environment`, `arguments` and `headers` map the names that the Package or Remote declares (and, for `command`, any variable name) to JSON Pointers into the configuration | 25a requires a digest, 25c and 25d bind declared names to places | Put the pointers inside the Package |
| P16 | `diagnostic.v1` is SARIF 2.1.0's `result` reduced to `ruleId`, `level`, `message.text` and `locations[].logicalLocations[].fullyQualifiedName`, which holds the JSON Pointer | 24a names a JSON Pointer as a logical location | `physicalLocation` with a region |
| P17 | `fetch-package.v1` returns the archive in `archive`, base64 (RFC 4648) | A tool's outputs are JSON | A `file:` URL the provider writes to |
| P18 | `run.v1` returns `endpoint`, a Streamable HTTP URL; the engine's own runner gives none and reaches its process over stdio | 2 says the run returns the endpoint | A local socket path |
| P19 | `inspect.v1`'s output `records` is a list of `record.v1` (24c) | 24c gives `record.v1` no other place of use | Export only |
| P20 | A setup or retire hook is dispatched through no Conduit, for the caller of the setup or retirement, with an operation identity scoped to the instance alone (OPS-028), so a further setup by another actor does not repeat a hook that succeeded | 1c: the hook repeats "only as its operation identity allows" | Scope the identity to the caller too, as OPS-008 does |

## 1. Terms

The docs editor writes these into `spec/definitions.md`; every editor writes in them now.
"New" and "changed" rows give the meaning to use. "Goes" rows name what replaces the term.

### New terms

| Term | Meaning | Promises |
| --- | --- | --- |
| provider | A Resource whose type offers one of the core package's provider capabilities: a runtime provider, the machines provider, a package-source provider or a secret store. The engine's own runner, its own machine and its own `https:` and `file:` fetching are the built-in implementations of the first three (31a). | PRV-001, PRV-003 |
| provider capability | One of `runtime-provider.v1`, `machine-provider.v1`, `package-source.v1` and `secret-reveal.v1` (31a, 19). | PRV-001 |
| runtime provider | A provider whose type offers `runtime-provider.v1`: it runs an instance's implementation, stops it and reports its status (2). | PRV-006 to PRV-012 |
| run | The running of one instance's implementation item by a runtime provider, named by the instance's ID; one run serves one instance (2a). | PRV-007, TPL-019 |
| `runtime` (standard argument) | The reference by which an Agent or a Resource names the runtime provider that runs it, fixed for life; with none, the engine's own runner (31b). | PRV-006, OBJ-013 |
| machine | A computer that the machines provider reports, with its hostname, its labels and its state: reachable, unreachable or gone (8). | PRV-014 |
| machines provider | The provider whose type offers `machine-provider.v1`: it reports the machines and places instances on them; with none, the engine's own machine (8, 31b). | PRV-013 to PRV-015 |
| label | A key and a string value that the machines provider reports of a machine, as Kubernetes labels a node; reported status, never configuration (8d, C58). | PRV-014, PLC-026 |
| label selector | A map from label to value that a Type and an implementation item may state under `placement`; it matches a location whose labels hold each (8a). Replaces "requirement on placement". | PLC-007, PLC-025 |
| package-source provider | A provider whose type offers `package-source.v1`: it fetches an archive for the URL schemes it claims (21, 31b). | PRV-016, PRV-017 |
| provider call | The engine's invocation of an operation of a provider capability other than `reveal-secret.v1`. It is not a request: no Conduit permits it and no access decision is made for it (P5). | PRV-005 |
| secret store | A provider whose type offers `secret-reveal.v1`; each store is its own Type with a schema for its locator (19). | SEC-005, SEC-006 |
| secret Resource | A Resource whose type offers `secret-reveal.v1` (19). | SEC-002 |
| secret reference | A value in a place that `secret-reference.v1` governs: a reference to a secret Resource, never the secret itself (S4.1). | SEC-001 |
| read (of a secret) | The engine's request for `reveal-secret.v1`, through the instance's own Conduit, before a run or a connection; the only way a secret is read in this version (4a, 4b). | SEC-010, SEC-011 |
| setup hook, retire hook | The core operations `setup-hook.v1` and `retire-hook.v1`, which a template selects in its `implementation` map: the instance's own setup work, dispatched once its Conduits, entries and run are up, and its own work before its run is stopped (1a). | TPL-017, TPL-018 |
| part (of a setup or a retirement) | One of what a setup or a retirement does: opening one declared Conduit, setting up one entry's Resource, the run or the connection, a hook, the stop (1). Replaces "step". | TPL-012, OPS-022 |
| remove (an instance) | End an instance's existence: at once once its shutdown is confirmed, or by `unregister.v1` where its location was declared lost (17). | LIF-035, LIF-017 |
| Conduit's extension | The extension of the Model Context Protocol, `io.github.shrav-k.conduit/interface`, under which Conduit's keys travel, with the setting `revision: v0` (18). | IFC-008, IFC-037 |

### Changed terms

| Term | What changes | Promises |
| --- | --- | --- |
| location | A Resource of `location.v1`, owned by `system`, that the engine registers for each machine the machines provider reports; `local` is the engine's own machine. It has no facts and no control, and is never registered by an actor (8, 8d). | PLC-008, PLC-026, FST-006 |
| placement | Still one location, fixed for life. Where a document gives none, the machines provider places the instance (8c). | PLC-002, PRV-015 |
| shared-host requirement | `shared-host.v1`: the `kubernetes.io/hostname` labels of the two ends' locations are equal, decided once when the Conduit is instantiated (8b). | PLC-013 |
| requirement | A Requirement contract: `inputs` alone. It is applied only to a Conduit. A requirement of the core package is applied by the engine and never sent; any other is a filter (C20, 3a, 7, 14, P12). | CTR-005 |
| filter | A requirement other than the core package's, applied to a Conduit; the target's implementations apply it to each request (C20). | CON-018, CON-022 |
| restriction | A filter or a bound that applies to a request, including a filter carried by an onward request or a hook. No policy (3a). | ACC-008, ACC-017 |
| bound | `bound.v1`, applied by the engine itself (C20); it may name the operations of a submitted project as `address#name` (11). | ACC-029, ACC-030 |
| `supports` | Lists the filters an implementation reached by `command`, `package` or `remote` can apply; `engine` states none (C20, 25). | PKG-019, MAN-012 |
| implementation item | Reached by exactly one of `command`, `package` (one server.json Package) and `remote` (one server.json Remote, Streamable HTTP); no `url`, no `graph` (25, 1). | MAN-009, MAN-010 |
| implementation | No longer a graph (1). | PKG-019 |
| project | A project that holds contracts or lists implementations states `address`, and may state `version`; its contracts are `address#name` outside its own documents (11). | MAN-004, MAN-015, SUB-008 |
| unpublished package | Goes as a name: "the contracts of a submitted project", identified for one submission by SUB-008's digest (11, P11). The form `<digest>#<name>` goes. | SUB-008, NAM-009 |
| control | Applies exactly what it is sent or fails; declares the input `revision` and no output `settings`; the engine checks the result against the type's schema before dispatch (15). | CTR-006, REV-007, REV-015 |
| revision | An opaque non-empty string compared only for equality (C21). | REV-001 |
| handle | A top-level property of an Operation's schema whose schema is `handle.v1`. No instance holds one (2, 16a). | IFC-011, CTR-002 |
| input | An Operation contract's inputs are the `properties` of its `inputSchema`; a template's and a Requirement's keep the member form (16a). | CTR-002, OBJ-015 |
| optional | A template's input, a Requirement's input, a declared Conduit and an entry are marked `optional: true`; an Operation's input or output is optional where `required` does not list it (16a). | OBJ-015 |
| standard argument | Adds `runtime` for an Agent and a Resource (31b). | OBJ-013 |
| condition | Reported in `conditions` in the shape of Kubernetes `metav1.Condition`, with `lastHeartbeatTime` and `origin` added as Conduit's own (24b). | LIF-011, OBS-002 |
| diagnostic | SARIF 2.1.0's `result`, reduced (24a, P16); every result that reports invalid or conflict carries an `error` one naming the promise that decided it. | IFC-025 |
| record | Its form is `record.v1`, a CloudEvent with REC-002's fields in `data` (24c). | REC-002 |
| result | `isError` is true only where the request ended with a reason or its outcome is failed or unknown (6b). | IFC-025 |
| repeat | A repeat of an invocation that has not ended reports conflict, with the engine's ID (6b). | OPS-009, OPS-032 |
| operation identity | The engine gives a hook one scoped to the instance alone (1c, P20). | OPS-028 |
| caller | Every request has one; the stop "for no actor" goes (2, 4c). | IFC-006 |
| request | Three kinds: an actor's own, an onward request, and the engine's read of a secret for a run, which has the caller of the request that asked for the run (4c). | SEC-011 |
| onward request | Made by an instance, or by its hook, while serving an invocation (1). | ACC-016 |
| chain | No steps: an onward request and a hook serve the invocation they name or belong to. | ACC-017 |
| cancel | The protocol's cancellation of the request that dispatched an invocation (22). | OPS-027 |
| set up, setup | Its parts, in order: declared Conduits or entries, the run or the connection, the setup hook (1). | TPL-012 |
| retire, retirement | The retire hook, then the stop of the run (1, 2). | TPL-009 |
| shutdown | Never held up by a runtime; confirmed once `retire.v1` has succeeded (2). | LIF-024 |
| unregister | Remove a template; or remove an instance whose location was declared lost (17). | LIF-017 |
| exists | Until unregistered or removed (17). | LIF-007 |
| declare lost | Also the effect of a machine reported gone (8e). | PLC-017, PLC-024 |
| inspect | The status holds `conditions` (24b); `records` is a list of `record.v1` (P19). | LIF-013 |
| tool, tool name | No retrieval tool, no component segment; the engine presents no tool for `reveal-secret.v1` (6c, 5, 4b). | IFC-012 |
| Conduit's keys | Travel as Conduit's extension (18); no `interfaceVersion`, `state` or `withheld`; add `config` and `revision` (1b). | IFC-006, IFC-025 |
| management verb | Every operation of `object-management.v1`, `access-control.v1` and `pool-management.v1`. | CON-024 |
| reconcile | `reconcile.v1` loses `handle` and `settings` (2a, 15). | OPS-031 |
| purge | Removes records; what repeats and reconciliations need is engine state that no purge touches (C33). | REC-004, OPS-008 |
| schema | JSON Schema 2020-12 with the applicator and validation vocabularies, less the exclusions of decision 20. | SCH-008 |
| manifest | Named `conduit.yaml`, `conduit.yml` or `conduit.json` (C14); `exports` lists the package's own documents (9). | MAN-001, MAN-007 |
| `fetch` | A URI reference; `https:` and `file:` on every engine, other schemes by a package-source provider; it carries no credential (21). | NAM-012, MAN-006 |
| name | Fixed once the object is made (10). | OBJ-011 |
| mediated, enforced | Enforced only where the consumer's runtime provider confines it; in v0 every restriction is mediated (2d). | ACC-027 |
| depends on | Only an entry on the entries it refers to; parts of a setup run in order (1). | POOL-006, TPL-012 |
| configuration | Carried in `_meta` on every request to an implementation that carries Conduit's keys (1b). | IFC-006 |
| definition | No runtimes and no graphs. | — |
| type | No `runtimes`, no `restoresRuntimes`; `placement` is a label selector (2, 8a). | CTR-004 |
| export | No re-export (9). | MAN-007 |
| core package | Exports as section 4 lists. | — |

### Terms that go

| Term | Replaced by |
| --- | --- |
| runtime (a template's or a Type's declaration), `runtimes` | run, runtime provider (2) |
| hold (a handle), handle held by an instance | nothing: no instance holds a handle (2a) |
| restore (a runtime), `restoresRuntimes` | a runtime provider's restart (2b) |
| graph, setup graph, retire graph | setup hook, retire hook, part (1) |
| step | part (1) |
| value binding, value reference, `source` | nothing (1) |
| output (of a graph) | nothing; an operation's outputs stay |
| policy, `apply-policy.v1`, `remove-policy.v1` | closing a Conduit and opening a narrower one (3a) |
| composite, component, component Conduit | two declared Conduits (5) |
| constituent, composition, composed requirement | several requirements side by side (7) |
| re-export | nothing (9) |
| rename, `rename.v1` | a name is fixed (10) |
| location fact, facts, host, `set-facts.v1`, `location-settings.v1` | label (8) |
| default location, `set-default-location.v1`, `system-settings.v1`, `system-pool.v1` | placement by the machines provider (8c) |
| requirement on placement | label selector (8a) |
| obliges | nothing (14) |
| narrows | filter as redefined (C20) |
| retained result, retrieval tool, `retrieve-result.v1` | a repeat (6c) |
| withheld | nothing (6a) |
| `inProgress` as a key value | conflict on a repeat (6b) |
| isolation plugin | a runtime provider that confines its consumer (2d) |
| unregister (of an instance whose shutdown is confirmed) | remove (17) |
| the request for no actor | nothing (2, 4c) |
| `<digest>#<name>` | `address#name` (11) |

## 2. New files

| File | Layer of its promises | Prefix | First ID | Last ID | Written by | Decisions |
| --- | --- | --- | --- | --- | --- | --- |
| `spec/promises/providers.yaml` | language, convention | PRV | PRV-001 | PRV-018 | G2 | 2, 8, 21, 26 (provider side), 31, C58 |
| `spec/promises/secrets.yaml` | language, convention, interface | SEC | SEC-001 | SEC-020 | G7 | section 4, 4, 13, 19, 25c |

Each new file opens with a head comment in the style of the others: what it is, the rulings
(40 and 41, with the paths of their records), the standards it follows, and "Draft: no vector
exists yet." The secrets file's head comment also says that its IDs are new and that the
secrets proposal's drafts SEC-1 to SEC-22 are placeholders it does not reuse, and gives the
map: SEC-001 from SEC-1, SEC-002 from SEC-2, SEC-003 from SEC-3, SEC-004 from SEC-22, SEC-006
from SEC-12, SEC-008 from SEC-7, SEC-009 from SEC-6, SEC-010 to SEC-012 from SEC-16, SEC-015 from
SEC-8, SEC-016 from SEC-19, SEC-017 and SEC-018 from SEC-10, SEC-019 from SEC-11.

`spec/promises/graphs.yaml` keeps its file and holds no promise (P2): its content is `[]` under
a head comment that names GRA-001 to GRA-028 as removed by ruling 41, with where each went.

## 3. The editing groups

Each file lists: promises removed (ID, the decision, where it went), promises rewritten (ID,
decision, exactly what changes), promises added (ID, layer, level, one sentence), and, for the
integrator, what cites each removed ID outside that file. "Unchanged" lists the rest.

### G1 lifecycle

Decisions written here: 1 (1a, 1b's use, 1c's lifecycle side), 2 as it bears on templates and
lifecycle, 17, 24b (conditions), the lifecycle side of 8e. Cites: PRV-007 to PRV-009 (G2),
CON-004 and POOL-013 (G2, G3), OPS-022 and OPS-028 (G4), SEC-010 and SEC-011 (G7), IFC-041 (G5).

#### `spec/promises/graphs.yaml`

Removed, all by decision 1 (R40 3c); the file becomes `[]` (P2):

- GRA-002 → TPL-012 (parts begin in order). GRA-003 → OPS-022, CON-004, POOL-013.
- GRA-022 → CON-004 (declared Conduits) and POOL-013 (entries).
- GRA-001, GRA-004 to GRA-012, GRA-014, GRA-015, GRA-017 to GRA-021, GRA-023 to GRA-028:
  nothing replaces them.

Cited by, outside the file: GRA-003 by CON-004, OPS-022, OPS-028; GRA-007 by OPS-026; GRA-010
and GRA-020 by TPL-008; GRA-014 by MAN-010; GRA-017 by OBJ-015; GRA-021 by OBJ-015, OPS-022;
GRA-022 by CON-004, POOL-013; GRA-024 by ACC-018, TPL-004; GRA-027 by OPS-028. Also
`spec/definitions.md`, `spec/core/README.md`, `spec/schemas/README.md`,
`spec/schemas/graphs.yaml`, `spec/schemas/manifest.yaml`, `docs/OVERVIEW.md`,
`tools/check_schemas.py` and the examples named in section 4.

#### `spec/promises/templates.yaml`

Removed:

- TPL-006 (2): runtimes go; replaced by the run, PRV-007.
- TPL-007 (2): replaced by PRV-007 and 1c's "a running run is left as it is".
- TPL-015 (2a): `reconcile.v1` loses `handle`; a run of unknown outcome is settled by its
  status, PRV-009.
- TPL-016 (1, 2): nothing replaces it.

Rewritten:

- TPL-001 (1, 2): the `spec` may hold `type`, `inputs`, `config`, `implementation` and
  `controls`; for an Interface, Agent or Resource also `conduits`; for a Pool `entries`. Drop
  `runtimes`, `setup` and `retire`, and drop `runtimes` from the maps keyed by name. Observable:
  drop "an isolation setting"; add that a template that writes `runtimes`, `setup` or `retire`
  is rejected under OBJ-019.
- TPL-003 (1, R40 3c): each value of the `implementation` map is a string, the selection of an
  implementation; a graph written in place goes. Observable: drop the step and runtime
  sentence; replace the `setup`/`retire` sentence with: `setup-hook.v1` and `retire-hook.v1`
  are selected here like any operation, under TPL-017 and TPL-018.
- TPL-004 (1, 2, 12): `when` drops "that a runtime of the template or of its type, or one of
  the template's steps without `conduit`, names". Observable: drop the step-with-`conduit`
  sentence and the last sentence (registration while a package was not installed, INS-022).
- TPL-008 (1): `when` becomes "An instance is set up or retired"; `then`: it is set up by the
  one invocation of `set-up.v1` and retired by the one invocation of `retire.v1` that name it;
  the engine implements both; neither is selected in the `implementation` map. Observable:
  keep the target of ACC-033 and the Conduit sentence; drop every graph sentence; add that
  what a setup does is TPL-012 and what a retirement does is TPL-009.
- TPL-009 (1, 2, 17): `then`: within the invocation of `retire.v1` the engine dispatches the
  retire hook where one is selected and a setup of the instance was ever dispatched, under
  TPL-018, and then stops the instance's run, under PRV-008; nothing else is done, apart from
  closing its Conduits under CON-008. Observable: no setup ever dispatched → nothing is
  dispatched and the invocation succeeds at once (LIF-025); the invocation succeeds only if the
  hook and the stop did, as OPS-022 counts them; drop every runtime, handle and LIF-030
  sentence; keep the CON-008 and ACC-039 sentence.
- TPL-012 (1, GRA-002): layer `language` → `convention`. `when`: "An instance is set up by an
  invocation of `set-up.v1`". `then`: the setup is these parts, each begun only once every part
  before it that is not marked optional has succeeded: opening its declared Conduits, under
  CON-004, or for a Pool setting up the Resources of its entries, under POOL-013; then asking
  for its run, under PRV-007, or connecting to its `remote` item, under IFC-041; then
  dispatching its setup hook, under TPL-017. Nothing else is dispatched. Observable: a run is
  asked for only once the declared Conduits are open (S4.5); with no item that is run, no
  `remote` item that names headers and no hook, a setup dispatches nothing beyond the first
  part.

Added:

- TPL-017 (convention, must; 1a, P20): Where a template selects an implementation for
  `setup-hook.v1`, the engine dispatches it within each invocation of `set-up.v1` for its
  instances, as the last part of TPL-012, to that implementation, through no Conduit, for the
  caller of the setup and with the operation identity of OPS-028.
- TPL-018 (convention, must; 1a, 4c): Where a template selects an implementation for
  `retire-hook.v1`, the engine dispatches it within each invocation of `retire.v1` for an
  instance for which a setup was ever dispatched, before it stops the run, asking for the run
  first under PRV-007 where it is not running.
- TPL-019 (language, must; 2a, P3): A template whose selected implementations, by its
  `implementation` map or by default, include two different implementation items reached by
  `command` or `package` is rejected, because one run serves one instance.

Unchanged: TPL-002, TPL-005, TPL-010, TPL-011, TPL-013.

Cited by, outside the file: TPL-006 by `spec/schemas/*`, examples, `spec/core/README.md`,
definitions, `tools/check_schemas.py`; TPL-007 by LIF-024, OPS-021; TPL-015 by LIF-015, LIF-018,
LIF-024, LIF-028, OPS-014, OPS-031, REC-004, `spec/core/operations.yaml`; TPL-016 by the schemas
README and `tools/check_schemas.py`.

#### `spec/conventions/lifecycle.yaml`

Removed:

- LIF-030 (17): an instance whose shutdown is confirmed no longer exists, so a request that
  names it reports invalid under OPS-029.

Rewritten:

- LIF-001 (3a): observable: "to limit access for a time without ending it, a policy is applied
  under ACC-031 and later removed" → "to narrow access, its Conduit is closed and a narrower
  one opened, under ACC-011"; drop the two sentences on policies and ACC-015.
- LIF-007 (17): `then`: it still exists, in the state retired, until it is removed under
  LIF-035, or unregistered under LIF-017 where its location was declared lost. Observable:
  "until an unregister operation removes it" → "until it is removed"; a request that names it
  afterwards reports invalid under OPS-029.
- LIF-008 (1, 17): observable: "the step for it reports conflict under LIF-015" → "the part of a
  setup for it reports conflict, under CON-004 or POOL-013"; "the consumer of a closed declared
  Conduit" stays.
- LIF-009 (5, 17, P9): `then`: the request is `register-replacement.v1`, which names, in
  `instance`, the instance that holds the nested instantiation and, in `nested`, the name of
  the declared Conduit or the entry. Observable: drop every component and composite sentence;
  after "Instances that held the retired one's ID still hold it" add "and, once it is removed,
  report it missing under LIF-019"; "for which PLC-004 chooses no placement" → "that the
  machines provider cannot place, under PRV-015".
- LIF-010 (8e): `when`: "the machines provider reports its machine unreachable, under
  PLC-023". Observable: the last sentence → "What a request to such an instance reports is
  PLC-023."
- LIF-011 (24b): `then`: they are reported in `conditions` in the instance's status, each in
  the shape of Kubernetes `metav1.Condition` (`type`, `status`, `reason`, `message`,
  `lastTransitionTime`, `observedGeneration`) with `lastHeartbeatTime` and `origin`, which are
  Conduit's additions; they never change the lifecycle state. Observable: "whose process has
  ended" → "whose run is reported stopped, under PRV-009".
- LIF-013 (24b, P19): observable: the status also holds `conditions`, under LIF-011, and
  `records` is a list in the form `record.v1`, under REC-002.
- LIF-015 (2, 17): observable: the list of requests a retired instance takes becomes: inspect
  it; retire it again under LIF-026; reconcile its setup or retirement under LIF-028; reconcile
  any other invocation dispatched to it where the reconciliation stores nothing; unregister it
  where its location was declared lost; ask under LIF-009 for a new instance in its place;
  declare it lost where it is a location. Drop the runtime and LIF-030 clauses.
- LIF-017 (17): `when`: "An actor asks to unregister an instance". `then`: dispatched only where
  the instance is retired and its location was declared lost; it removes the instance;
  otherwise not dispatched, conflict. Observable: an instance whose shutdown is confirmed has
  been removed already under LIF-035, so a request that names it reports invalid under
  OPS-029; drop the runtime sentence; keep the revision sentence; `unregister.v1` on a template
  is LIF-012.
- LIF-018 (2, 17): drop the runtime sentences in `then` and observable; "can be unregistered" →
  "is removed, under LIF-035"; keep the lost-location sentence, pointing at LIF-017.
- LIF-019 (17): `when`: "An object is unregistered or removed".
- LIF-020 (17): `when` drops "or to unregister a Pool that owns a Resource that still exists".
- LIF-021 (1, 17, 23): observable: "a step of a Resource's `retire` graph that names a declared
  Conduit to another Resource of the Pool" → "an onward request of a Resource's retire hook
  through a Conduit to another Resource of the Pool"; drop the LIF-030 sentence; add that a
  Resource whose setup, retirement or change is in progress makes its retirement report
  conflict under REV-014.
- LIF-022 (10): drop "or to rename", "renamed" and the two Conduits of `root` from `when` and
  `then`; observable drops its rename sentence.
- LIF-023 (17, 8e): `then`: "An instance is retired" (drop "or unregistered"). Observable: the
  retirements without a request also include those of PLC-024; removal after a confirmed
  shutdown is LIF-035 and is not a retirement.
- LIF-024 (2, 17): `then`: a Conduit's shutdown is confirmed when it is closed; any other
  instance's is confirmed once an invocation of `retire.v1` for it has succeeded, and at once
  where no setup was ever dispatched (LIF-025); it is unknown where the outcome of the latest
  invocation of `retire.v1` is unknown, and where a setup was dispatched and its location was
  declared lost (PLC-017) or its machine reported gone (PLC-024); pending otherwise.
  Observable: drop every runtime, start, stop and handle sentence; a confirmed shutdown
  removes the instance under LIF-035.
- LIF-025 (1, 2): `then`: its retire hook is not dispatched, no stop is asked for, and its
  shutdown is confirmed at once.
- LIF-026 (1, 1c): observable: "OPS-021 and DL1 ... decide which of its steps are dispatched
  again" → "the retire hook is dispatched again only as OPS-021 allows, and the stop is asked
  for again, which succeeds where nothing runs (PRV-008)"; "a step of the further retirement
  that names one of them under `conduit`" → "an onward request of the retire hook through one
  of them"; drop the LIF-030 pointer at the end → "A request that names an instance whose
  shutdown is confirmed reports invalid under OPS-029."
- LIF-027 (1, 2): observable adds that a run it asked for is not stopped.
- LIF-028 (2, 5, 17): drop every runtime clause from `then` and observable; "can be
  unregistered" → "is removed under LIF-035"; drop the last sentence on composite declared
  Conduits.
- LIF-029 (5): observable drops "and for a component".
- LIF-031 (1, 3a, 6c): observable: "A step of a graph, and an onward request" → "An onward
  request"; drop "a value reference with the source `config` yields" (the configuration still
  holds the resolved form, and an implementation receives it in `_meta` under IFC-006); drop the
  `retrieve-result.v1` sentence; "An invocation and a policy are not objects: `reconcile.v1`
  and `remove-policy.v1` name each" → "An invocation is not an object: `reconcile.v1` names it".
- LIF-033 (5, P9): `when`: the request names, in `instance` and `nested`, a nested
  instantiation that the named instance does not hold, or names a first-start object.
  Observable drops the composite sentences.
- LIF-034 (10): drop "or to rename" and the two Conduits of `root`.

Added:

- LIF-035 (convention, must; 17): Once the shutdown of a retired instance is confirmed, the
  instance is removed and no longer exists, its records kept as LIF-019 states; a Conduit is
  so removed when it is closed.

Unchanged: LIF-002 to LIF-006, LIF-012, LIF-014, LIF-016, LIF-032.

Cited by, outside the file: LIF-030 by `spec/core/README.md`, definitions (inside the file:
LIF-015, LIF-021, LIF-024, LIF-026; TPL-009 in G1).

#### `spec/conventions/observation.yaml`

Removed:

- OBS-005 (2): restoring goes; a provider's restart is PRV-010.
- OBS-007 (2): the stop for no actor goes; nothing replaces it.

Rewritten:

- OBS-002 (24b): observable: an observation is reported as a condition of LIF-011, its time in
  `lastHeartbeatTime` and its origin in `origin`.
- OBS-003 (8e, 24b): `when`: "because its machine is reported unreachable, under PLC-023";
  `then`: reported as unknown (a condition whose `status` is `Unknown`) with
  `lastHeartbeatTime` of the last successful observation. Observable: "location" → "machine".
- OBS-004 (2b): `then` drops "other than an implementation restoring its own runtime, which
  OBS-005 and OBS-007 govern". Observable adds: a runtime provider's restart under its own
  policy is the provider's, under PRV-010, and the engine dispatches nothing because of it.

Unchanged: OBS-001, OBS-006.

Cited by, outside the file: OBS-005 by CTR-004, examples, definitions, `docs/OVERVIEW.md`;
OBS-007 by ACC-019, IFC-006, REC-002, `spec/core/README.md`, definitions, `docs/OVERVIEW.md`.

#### `spec/conventions/implicit.yaml`

Removed:

- IMP-004, IMP-005, IMP-009 (1): no graphs; IMP-002 and IMP-008 cover the hooks.

Rewritten:

- IMP-002 (1, 2, 3a, 8e): `when`: drop "a policy changes" and "an implementation restores a
  runtime"; "a location is reached again" → "a machine is reported reachable again"; add "a
  setup hook or a retire hook is dispatched; a runtime provider restarts a run".
- IMP-008 (1): observable: "or a step of a setup or retire graph" → "or a setup or retire
  hook, whose operations carry no mark".

Unchanged: IMP-001, IMP-003, IMP-007.

Cited by, outside the file: IMP-004, IMP-005, IMP-009 by `spec/schemas/README.md`,
`spec/schemas/graphs.yaml`, definitions.

### G2 placement and providers

Decisions written here: 2 and 2a to 2d (provider side), 8 and 8a to 8e, 21 (provider side),
26 (provider side), 31a, 31b, C58. Cites: TPL-012, TPL-017 to TPL-019, LIF-035 (G1); SEC-010
to SEC-014 (G7); OBJ-013, INS-012, MAN-010 (G6); IFC-022, IFC-041 (G5).

#### `spec/promises/placement.yaml`

Removed:

- PLC-003, PLC-004, PLC-016, PLC-022 (8): the default location goes; PLC-002 and PRV-015.
- PLC-021 (8): facts and `set-facts.v1` go; labels are PLC-026.

Rewritten:

- PLC-002 (8c): layer `language` → `convention`. `when`: an Agent or Resource instance document
  gives no `placement`. `then`: its placement is the location of the machine that the machines
  provider chooses under PRV-015, or `local` where there is no machines provider. Observable:
  the resolved instance holds that location's ID; drop "other than a location's".
- PLC-007 (8a): `when`: the instance's type, or the implementation item selected for it,
  states `placement`, a label selector, and the labels reported for the location the instance
  would be placed on do not hold each of its labels with that value. `then` unchanged.
  Observable: the selector is the union of the type's and the item's, which PLC-025 keeps from
  conflicting; it is tested when the instance is instantiated and not again; drop "stored
  facts".
- PLC-008 (8, 8d): `when`: "A location exists". `then`: it is a Resource of the core type
  `location.v1`, owned by `system`, its own placement, with no configuration; the engine makes
  it under PLC-026, and no instance document makes one (FST-006). Observable: drop `facts`, the
  argument `facts` and PLC-021; its labels are in its status.
- PLC-011 (17): `when`: "... that has other instances placed on it that exist". Observable:
  "unregistered" → "removed or unregistered".
- PLC-012 (8): "a location or a host" → "a location or a machine".
- PLC-013 (8b): layer `convention` → `language`. `when`: a Conduit is instantiated that applies
  `shared-host.v1`, and the `kubernetes.io/hostname` labels of its consumer's and its target's
  locations are not equal, or either is absent. `then`: the Conduit is not instantiated.
  Observable: decided once; nothing changes a location's hostname and a placement never
  changes (PLC-009), so no request is checked again.
- PLC-017 (8e, 17): observable: "Each of them can then be unregistered under LIF-017 without a
  reconciliation" → "One whose shutdown is unknown is removed by a request to unregister it,
  under LIF-017; one whose shutdown is confirmed is removed under LIF-035"; add that a machine
  reported gone has the same effect, under PLC-024.
- PLC-020 (8): "on the host of the Resource's location" → "on the machine of the Resource's
  location".

Added:

- PLC-023 (convention, must; 8e, replaces DL5): Where the machines provider reports a machine
  unreachable, a request that would be dispatched to the implementation of an instance placed
  on its location is not dispatched and reports unavailable, and nothing is retired.
- PLC-024 (convention, must; 8e): Where the machines provider reports a machine gone, its
  location and every instance placed on it are retired as PLC-017 retires them, and the record
  says that the provider reported the machine gone.
- PLC-025 (language, must; 8a): A template that selects an implementation item whose
  `placement` names a label that its type's `placement` names with another value is rejected.
- PLC-026 (convention, must; 8d, P7): For each machine that the machines provider reports and
  no location stands for, the engine registers and sets up a location from the core template
  `location`, owned by `system` and named as P7 gives; the machine the engine runs on is
  `local`, and a location's labels are what the provider last reported, in its status.

Unchanged: PLC-001, PLC-005, PLC-006, PLC-009, PLC-015, PLC-018, PLC-019.

Cited by, outside the file: PLC-003 by `spec/core/*`, definitions; PLC-004 by LIF-009,
OBJ-013, POOL-016, `spec/schemas/instances.yaml`; PLC-016 by `spec/core/*`; PLC-021 by
`spec/core/*`, the first-start example; PLC-022 by OPS-029.

#### `spec/promises/pools.yaml`

Removed: none.

Rewritten:

- POOL-003 (5): observable: "for a declared Conduit and a component" → "for a declared
  Conduit".
- POOL-009 (31b, P14): `when` and `then` also cover `runtime`: an input whose destination is an
  entry's `runtime` gives that entry's Resource its runtime provider; an entry writes neither
  key itself. Observable: an entry whose runtime no input supplies runs on the engine's own
  runner.
- POOL-013 (1, GRA-022): `then`: the setup of the Resource of each entry that is not retired is
  a part of the Pool's setup, under the entry's name, begun after the parts for the entries it
  refers to; for a Resource already ready nothing is dispatched and the part counts as
  succeeded; where the entry has no Resource that is not retired, the part reports conflict.
  Observable: an entry marked optional whose Resource is not set up does not keep the setup
  from succeeding, and is reported as not set up (from GRA-003).
- POOL-015 (1): "a Pool's setup reaches the step that sets up" → "a Pool's setup comes to the
  part that sets up"; "the step" → "the part" throughout.
- POOL-016 (8c): `then`: the Resource is placed under PLC-002, by the machines provider.
  Observable: drop the default-location sentences; where the provider places none, the Pool is
  not instantiated under POOL-002; a replacement is placed the same way when it is made.

Unchanged: POOL-001, POOL-002, POOL-004 to POOL-007, POOL-011, POOL-012, POOL-014, POOL-017.

#### `spec/promises/first-start.yaml`

Rewritten:

- FST-001 (8d, 10): observable: "Those names stay theirs, because LIF-022 refuses the rename" →
  "A name is fixed, under OBJ-011"; "The stored facts that `local` is given are DL6" → "`local`
  is the location of the machine the engine runs on, and its labels are those of PRV-014".
- FST-002 (10): observable drops the rename sentence.
- FST-006 (8d): `when` adds the template `location`. `then`: the engine alone makes locations,
  under PLC-026. Observable drops "The template `location` is not one of the four".

Unchanged: FST-003, FST-005.

#### `spec/promises/providers.yaml` (new)

Head comment: providers are how ruling 40's "swappable by the user" is written in the
language; standards followed: Kubernetes CRI and RuntimeClass, Nomad task drivers, Kubernetes
node lifecycle and well-known labels, Git's remote helpers by URL scheme.

- PRV-001 (language, must; 31a): A Type contract that offers a provider capability,
  `runtime-provider.v1`, `machine-provider.v1`, `package-source.v1` or `secret-reveal.v1`, is of
  the family Resource and offers no other of them; one that is not is rejected when its package
  is installed or its project submitted.
- PRV-002 (convention, must; 31a, P6): A Resource whose type offers `runtime-provider.v1`,
  `machine-provider.v1` or `package-source.v1` has `local` as its placement, whether or not its
  document gives it, and runs on the engine's own runner; a document that gives it another
  placement or a `runtime` is rejected.
- PRV-003 (convention, must; 31a, 8c, 21): Where no provider of a kind serves (for a run, where
  the instance names no `runtime`), the engine's own implementation of that capability serves,
  its runner, its machine as `local` and its `https:` and `file:` fetching, and keeps every
  promise this file makes of a provider of that kind.
- PRV-004 (language, must; P5): A Conduit whose capability is `runtime-provider.v1`,
  `machine-provider.v1` or `package-source.v1` is not instantiated.
- PRV-005 (convention, must; 31a, 4c, P5): A provider call is not a request: no Conduit permits
  it and no access decision is made for it; it is made within the invocation it serves, carries
  of Conduit's keys only `operation`, `invocation` and, where that invocation has one, `caller`,
  and is recorded under REC-001.
- PRV-006 (language, must; 31b): An Agent or Resource instance document may give `runtime`, a
  reference to a Resource whose type offers `runtime-provider.v1`, which the instance holds in
  resolved form and no operation changes; one that names anything else is rejected, and an
  instance that gives none runs on the engine's own runner.
- PRV-007 (convention, must; 2, 2a, 2c, 25d, 1c, P4, P18): Within a setup, once the
  instance's declared Conduits are open, where it has an item that is run and its runtime
  provider does not report the run running, the engine asks that provider for the run with
  `run.v1`, giving the instance's ID, which names the run, the item, the values its
  `arguments` and `environment` take from the configuration, the values of SEC-010 apart in
  `secrets`, and its location's hostname and labels; `endpoint` is where the engine then
  reaches the server. Observable: no variable of the engine's own environment is passed;
  where the runtime provider is not ready, the part reports unavailable.
- PRV-008 (convention, must; 2a): Within a retirement, the engine asks the instance's runtime
  provider to stop its run with `stop.v1`, naming the instance's ID, and stopping a run that
  is not running succeeds.
- PRV-009 (convention, must; 2a, 24b, P13): Inspecting an instance that has an item that is
  run reports, as its condition `Running`, the state that its runtime provider's
  `inspect-run.v1` last gave, running, stopped or unknown, by which an actor settles a setup
  or retirement of unknown outcome under OPS-014.
- PRV-010 (convention, may; 2b, 2c): A runtime provider restarts a run that stopped under its
  own restart policy, and the engine makes no read and dispatches nothing because of it.
- PRV-011 (convention, must; 2c): A runtime provider given values in `secrets` delivers each as
  the variable or argument that the item binds, by its own means, puts it nowhere the item does
  not bind, and states in its contract where it keeps the value.
- PRV-012 (convention, must; 25a): A runtime provider asked to run a `package` item whose
  `registryType` its contract does not list ends the run failed and runs nothing.
- PRV-013 (convention, must; 31b, P6): A Resource whose type offers `machine-provider.v1` or
  `package-source.v1` is not instantiated where its owner is not `system`, where another
  machines provider that is not retired exists, or where another package-source provider that
  is not retired claims one of its schemes.
- PRV-014 (convention, must; 8, 8e, C58): The machines provider's `list-machines.v1` gives each
  machine its `hostname`, its `labels`, among them `kubernetes.io/hostname`, `kubernetes.io/os`
  and `kubernetes.io/arch` with GOOS and GOARCH values, and its `state`: reachable, unreachable
  or gone.
- PRV-015 (convention, must; 8c): An Agent or Resource instance document that gives no
  `placement` is placed when it is registered by the machines provider's `place.v1`, given the
  label selector of PLC-007, which names one reachable machine whose labels match; where it
  names none, the document is rejected.
- PRV-016 (language, must; 21, P6): A Type that offers `package-source.v1` states a `schema`
  that requires `schemes`, a non-empty list of URI schemes; one that does not is rejected when
  its package is installed or its project submitted.
- PRV-017 (convention, must; 21, 31b, P17): `install.v1` whose `fetch` has a scheme that a
  package-source provider claims fetches the archive with that provider's `fetch-package.v1`,
  and one whose scheme is `https` or `file` and that no provider claims is fetched by the
  engine itself; the archive is checked against the digest under PKG-026.
- PRV-018 (convention, must not; 26, R40 3e): Which runtime provider ran an instance, or on
  which machine, is taken as establishing which instance sends a request; IFC-022 alone
  establishes it.

### G3 access

Decisions written here: 3a, 3b, 5, 6a, 14, 28; the access side of 4 (4b), 11 and C20; 17 and 1
as they bear on access. Cites: SEC-007, SEC-008, SEC-011 (G7); TPL-017, TPL-018, LIF-035 (G1);
PRV-004, PLC-013, PLC-023 (G2); OPS-009, OPS-032 (G4); IFC-016 (G5); SUB-008, MAN-015 (G6).

#### `spec/conventions/access.yaml`

Removed:

- ACC-026, ACC-031, ACC-035, ACC-042, ACC-043 (3a): policies go. Narrowing is closing a
  Conduit and opening a narrower one (ACC-011). Recorded by ruling 41 as a candidate access
  feature for a later version (for the coordinator's tracker, not the specification).

Rewritten:

- ACC-001 (6c, 4b): observable: drop the retrieval sentence; add that a request for
  `reveal-secret.v1` that an actor sends reports refused under SEC-007 whatever Conduit lists
  it.
- ACC-003 (4b): `then` adds: except that no actor's request for `reveal-secret.v1` is
  dispatched, under SEC-007.
- ACC-005 (1, 5, S4.4): `when`: "A setup opens a declared Conduit, under CON-004". `then`: drop
  the composite sentence. Observable: drop the composite sentence; add that a declared Conduit
  that lists `reveal-secret.v1` is opened only where the root Interface sent the setup, under
  SEC-008.
- ACC-006 (S4.4, 17): `then` adds: "Where the new Conduit lists `reveal-secret.v1`, SEC-008
  states a further condition." Observable: "The step of a setup is left as succeeded" → "The
  part of a setup counts as succeeded"; a request to open a closed Conduit names one that no
  longer exists, under LIF-035, and reports invalid under OPS-029 (replacing "conflict under
  LIF-015").
- ACC-009 (3a): `then`: limited by its own Conduit's operations and filters and, for an
  onward request, by the restrictions carried under ACC-017; drop "by every policy". Observable
  drops the policy sentence.
- ACC-011 (17): observable adds: the closed Conduit is removed under LIF-035, and its tools
  are no longer presented, under IFC-013.
- ACC-012 (17): `then`: drop "A request to close a Conduit that is already closed ... succeeds".
  Observable: a closed Conduit no longer exists, so a request to close it reports invalid
  under OPS-029, and a repeat of the request that closed it is answered under OPS-009.
- ACC-015 (3a, 7, 9, 1, C20): `when`: a request carries a filter under ACC-017 or ACC-018 and
  the implementation it would be dispatched to does not state the filter's requirement in
  `supports`. Observable keeps: the result names the restriction; a plain implementation
  states none; a request for a core operation that carries a filter reports unavailable,
  because `engine` states none (MAN-012). Drop every sentence on policies, compositions,
  re-exports and graphs.
- ACC-017 (3a, 1, 4c): `then`: drop "and the policies that apply ... in that invocation's
  chain"; keep the filters. Observable: drop the policy sentences; "A step with no `conduit`
  carries ... under ACC-018" → "A hook carries them in the same way, under ACC-018"; add that a
  secret read carries none of them, under SEC-011.
- ACC-018 (1): `when`: the engine dispatches a setup hook or a retire hook, under TPL-017 or
  TPL-018. `then`: it is dispatched to the instance's own implementation and is not a request
  through any Conduit; it carries the restrictions of the invocation of `set-up.v1` or
  `retire.v1` it belongs to, as ACC-017 has an onward request carry them, and its own onward
  requests are onward requests under ACC-016. Observable: rewrite in those terms; drop every
  step, policy, composite and graph sentence.
- ACC-019 (2, 6c): `then`: drop "other than a call to `retrieve-result.v1` under IFC-024".
  Observable: drop the retrieval sentences, the OBS-007 sentence and "or to retrieve a result
  under IFC-024"; "IFC-016 says how its result is read" stays (IFC-016 rewritten).
- ACC-027 (2d): `then`: a restriction is enforced only where the runtime provider that runs
  the consumer confines it so that it reaches targets only through the engine, as that
  provider's contract states; in this version every restriction is reported mediated.
  Observable: drop "isolation plugin".
- ACC-029 (3a, 7, 9, 11, C20): `then`: drop "and so is an actor that a policy names".
  Observable: the bound is `bound.v1`, which the engine applies itself and never sends (C20);
  `operations` names each operation by a qualified reference, which reaches a contract of an
  installed package or of a submitted project that states that address (SUB-008, MAN-015);
  drop the sentences on unpublished packages, re-exports and CTR-011.
- ACC-030 (28): observable adds: a right to open trusts the delegate with every operation it
  names on every target it covers, with no filter that the delegate's own templates do not
  apply; and a Conduit to `management` that lists `register.v1` or `replace.v1` trusts its
  consumer with every object of that kind.
- ACC-033 (3a, 10, 6c, 8, 17, P9): `when`: drop "rename", "to apply or remove a policy";
  "unregister an object" → "unregister a template, or an instance whose location was declared
  lost". `then`: drop "renaming" and the policy clause; `register-replacement.v1` is authorized
  as registering the instance it makes would be. Observable: drop the sentences on
  `retrieve-result.v1`, on a control of a location and of the `system` Pool (none exists);
  "has been unregistered" → "no longer exists".
- ACC-034 (6a): `then`: the invocation runs to its end, its outcome is recorded, and its result,
  with its outputs, is delivered as for any other; an onward request made after the close by
  that invocation, or by any invocation that serves it, is not dispatched and reports refused.
  Observable: drop every withheld sentence and OPS-017; a later repeat, or a `tasks/get`, is
  decided anew, and through a closed Conduit reports refused (ACC-036).
- ACC-036 (3a, R40 3f): `then`: "from the Conduits as they stand". Observable drops "or a
  policy that forbids it applies" and "or that policy is removed".
- ACC-037 (3a): `when`: "the state of the Conduits that the decision needs".
- ACC-038 (14, 11, 1, C20): `then`: for each requirement, the implementation that applies it:
  for a filter, each implementation the target has selected for the operations listed; for a
  requirement of the core package, the engine, `github.com/shrav-k/conduit/core#engine`; each by
  its qualified reference, which for a submitted project's implementation is `address#name`.
  Observable: drop the consumer, graph and `<digest>#<name>` sentences.
- ACC-039 (1, 2, 4c): `then`: permitted only if it belongs to the retirement of that consumer:
  its retire hook, an onward request that serves it directly or through others, or a read under
  SEC-010 for a run that the retirement asks for. Observable: rewrite the step and runtime
  sentences in those terms.

Unchanged: ACC-002, ACC-008, ACC-014, ACC-016, ACC-022, ACC-023, ACC-024, ACC-028, ACC-032.

Cited by, outside the file: ACC-026 by `spec/core/README.md`, definitions, `docs/OVERVIEW.md`;
ACC-031 by CTR-005, IFC-018, LIF-001, NAM-019, `spec/core/*`, `spec/schemas/README.md`,
definitions, `docs/OVERVIEW.md`; ACC-035 by `spec/core/README.md`, definitions,
`docs/OVERVIEW.md`; ACC-042 by SCH-009, `spec/core/README.md`, definitions; ACC-043 by
`spec/core/README.md`.

#### `spec/promises/conduits.yaml`

Removed:

- CON-003, CON-019, CON-029 (5): composite Conduit templates go.
- CON-026 (10): rename goes.
- CON-030 (C20): `narrows` goes; a core requirement is applied by the engine, any other by the
  target's implementations.

Rewritten:

- CON-001 (5): observable drops "or for a composite template one Conduit per component".
- CON-002 (5): `then` drops the component clause.
- CON-004 (1, 5, 17, GRA-022): `when`: "An instance that declares Conduits is set up". `then`:
  the opening of each declared Conduit is a part of the setup, under the declared Conduit's
  name: where its Conduit's state is registered, a request for `open.v1` that the instance that
  sent the setup makes, under ACC-005; where it is open, nothing is dispatched and the part
  counts as succeeded; where the nested instantiation has no Conduit that is not retired, the
  part reports conflict; setup instantiates no Conduit. Observable: a declared Conduit marked
  optional that is not opened does not keep the setup from succeeding and is reported as not
  opened (from GRA-003); drop every composite sentence.
- CON-007 (8e): "its location becoming unreachable" → "its machine reported unreachable".
- CON-008 (2, 17, 1): `then` adds: a closed Conduit is removed, under LIF-035. Observable: drop
  "It is not confirmed while the outcome of a start ... or while it holds a runtime"; "a step of
  the consumer's `retire` graph can make a request" → "the consumer's retire hook can make an
  onward request".
- CON-016 (5): `then`: may hold `inputs`, and holds `capability`, `operations` and
  `requirements`; drop "or `components`" and the non-composite qualifier. Observable drops the
  `components` sentence.
- CON-018 (7, C20): `then`: drop nothing of the item form; observable: drop the composition
  and CON-030 sentences; add that a requirement of the core package is applied by the engine
  and every other is a filter, sent under IFC-018.
- CON-020 (5): observable drops "and a component that names a target" and the component
  clause.
- CON-022 (14, 7, 9, 1, 25b, C20): `when`: a Conduit is instantiated that applies a filter, and
  an implementation that the target has selected for an operation the Conduit lists does not
  state the filter's requirement in `supports`. `then`: not instantiated; an implementation
  reached by `command`, `package` or `remote` supports a requirement where it states it, matched
  by its address and export name. Observable: drop every sentence on `obliges`, the consumer,
  compositions, re-exports and graphs; keep plain and NAM-024.
- CON-024 (6c, 8): observable: a management verb is every operation of `object-management.v1`,
  `access-control.v1` and `pool-management.v1`; drop "other than its two controls and the one
  tool ... `retrieve-result.v1`".
- CON-028 (7): observable drops the constituent and CTR-011 sentence.
- CON-031 (17): observable: a Conduit whose target is retired is closed and removed already,
  so a request to open it reports invalid under OPS-029; drop "a request to open a closed
  Conduit reports conflict under LIF-015"; "The step of a setup" → "The part of a setup".

Unchanged: CON-006, CON-011 to CON-014, CON-017, CON-021, CON-023, CON-025, CON-027.

Cited by, outside the file: CON-003 by INS-016, LIF-009, OBJ-003, the project example,
`spec/implementation-defined.yaml`, definitions; CON-019 by `spec/schemas/*`, the project
example, `spec/core/README.md`; CON-026 by IFC-014; CON-029 by `spec/schemas/README.md`; CON-030
by CTR-005, `spec/core/*`, `spec/schemas/README.md`, the CON-030 example,
`tools/check_schemas.py`.

### G4 operations

Decisions written here: 6b, 6c (operations side), 15, 22, 23, 24c, C21, C33, and 1c (hook
identity). Cites: TPL-012, TPL-017, TPL-018, LIF-035 (G1); PRV-009 (G2); IFC-016, IFC-025 (G5).

#### `spec/conventions/operations.yaml`

Removed:

- OPS-015, OPS-016 (6c): retrieval goes; a caller that lost a result repeats the request
  (OPS-009).
- OPS-017 (6a): nothing is withheld.

Rewritten:

- OPS-005 (6b, 24a): `when`: drop "and it is not a repeat answered under OPS-009 or OPS-012"
  → "and it is not a repeat that OPS-009 or OPS-012 answers with the invocation's outcome".
  Observable adds that a result that reports invalid or conflict carries an error diagnostic
  naming the promise that decided it, under IFC-025.
- OPS-007 (16a): observable: "A declared output that is not marked `optional: true`" → "An
  output that the contract's `outputSchema` lists under `required`".
- OPS-008 (C33): `then`: identity scope unchanged; at most one invocation for one caller,
  sending instance, target and identity; the engine keeps, as its own state apart from the
  records, what repeats and reconciliations need: each invocation's identity, the engine's ID,
  its state, a digest of its contents, its operation, caller, instance, Conduit, authorizing
  target and, for a control, the values it was sent; no purge touches that state. Observable:
  "is read from the records" goes; add the hook exception of OPS-028.
- OPS-009 (6a, 6b, 6c): `when`: the invocation succeeded (drop "is in progress or"). `then`:
  nothing is dispatched, and the result is that invocation's: the outcome succeeded and its
  outputs, which the engine keeps for as long as DL10 says. Observable: drop the in-progress,
  withheld and retrieval sentences; a repeat of an invocation that has not ended, or whose
  outputs are no longer kept, is OPS-032; keep the access sentence, ending "reports refused
  under ACC-011".
- OPS-014 (2a, 6c, 17): observable: the ID is learnt from the result's `_meta` (IFC-025) or the
  records; drop `retrieve-result.v1` and TPL-015; "has been unregistered" → "no longer exists";
  the run of a setup or retirement of unknown outcome is checked by PRV-009.
- OPS-019 (3a, 10): `when`: drop "rename" and "apply a policy". Observable: example verbs
  `register.v1`, `set-up.v1`, `open.v1`, `install.v1`.
- OPS-021 (1, 1c, P20): `when`: the setup or retirement of an instance runs again, and the
  invocation of its setup hook or retire hook from an earlier one has the outcome unknown.
  `then`: it is not dispatched again except as a new attempt under OPS-012, with the identity
  of OPS-028. Observable: drop the step, DL1 and runtime sentences; add that an open Conduit, a
  ready Resource and a running run are left as they are, under CON-004, POOL-013 and PRV-007.
- OPS-022 (1, GRA-003): `when`: an invocation of `set-up.v1` or `retire.v1` has ended (drop the
  graph case). `then`: succeeded only if every part not marked optional succeeded, failed only
  if no part had an effect, otherwise unknown; in a further setup or retirement a part that is
  not dispatched counts with the outcome it has from before or from its reconciliation.
  Observable: the parts are those of TPL-012 and TPL-009; drop every graph-output and composite
  sentence.
- OPS-023 (3a): observable: "the state of the Conduits or of the policies" → "the state of the
  Conduits".
- OPS-024 (6a, 6c): observable drops the retained-result, OPS-017 and withheld sentences.
- OPS-026 (16a, 1): `when`: "An operation is requested and ..." (drop "or is invoked by a step
  of a graph"); the default of an input is the `default` its property states in the contract's
  `inputSchema`. Observable: drop the graph and GRA-007 sentences.
- OPS-027 (22): `when`: an invocation is cancelled while it is in progress, by the protocol's
  cancellation of the request that dispatched it: `notifications/cancelled` on stdio, a closed
  stream on Streamable HTTP, `tasks/cancel` for a task. `then` adds: the engine passes the
  cancellation on to the implementation by the protocol's means. Observable: drop DL14; no
  confirmation format is defined, so a cancelled invocation is usually unknown until it is
  reconciled; a dropped HTTP connection cancels.
- OPS-028 (1c, P20): `when`: the engine dispatches a setup hook or a retire hook. `then`: it
  gives the invocation an operation identity derived from the instance's ID and the hook's
  operation, the same in every setup, or every retirement, of that instance and scoped to the
  instance alone, so a further one dispatches the hook only where OPS-009, OPS-012 and OPS-024
  would dispatch a repeat. Observable: drop the step, graph and composite sentences.
- OPS-029 (3a, 8, 10, 17, P9): observable: the list drops `rename.v1`, `apply-policy.v1`,
  `remove-policy.v1`, `set-default-location.v1` and PLC-022 and OPS-015; "an object, a policy or
  an invocation" → "an object or an invocation"; "has been unregistered" → "has been
  unregistered or removed"; `register-replacement.v1` names a holder and a nested name.
- OPS-031 (2a, 15): `when`: the request states `revision` where the invocation is not of a
  control, where its instance no longer exists, or where it records the control as not having
  taken effect. Observable: drop `handle` and `settings` throughout.

Added:

- OPS-032 (convention, must; 6b, P8): A request that matches, as OPS-009 describes a match,
  the request of an invocation that has not ended, or of one that succeeded and whose outputs
  the engine no longer keeps, is not dispatched and reports conflict, and its result's `_meta`
  holds the engine's ID for that invocation.

Unchanged: OPS-001 to OPS-004, OPS-006, OPS-010 to OPS-013, OPS-018, OPS-020, OPS-025, OPS-030.

Cited by, outside the file: OPS-015 by IFC-021, `spec/core/README.md`, definitions; OPS-016 by
ACC-001, IFC-016, IFC-021, `spec/core/README.md`, definitions,
`spec/implementation-defined.yaml`; OPS-017 by ACC-034, IFC-025, REC-004, `spec/core/README.md`,
definitions.

#### `spec/conventions/revisions.yaml`

Removed:

- REV-008 (15): a control returns no `settings`; REV-015 checks before dispatch.

Rewritten:

- REV-001 (C21): observable: the revision is an opaque non-empty string, compared only for
  equality.
- REV-003 (10): `when`: "definition or configuration changes" (drop "or name"); observable
  drops the rename sentence.
- REV-004 (10, 15): `when`: drop "and name"; observable: "a change that confirmed the values
  already stored" → "a control whose values equal those stored".
- REV-006 (16a): observable: "whether or not its contract marks it `optional: true`" →
  "whether or not the control's `inputSchema` lists it under `required`".
- REV-007 (15): `then`: a control that succeeds has applied exactly the values it was sent, and
  exactly those are stored at the pointers of its `changes`; it returns no `settings`, and an
  implementation never applies another value in their place. Observable: drop the `settings`
  examples and REV-008; keep REV-006's case.
- REV-010 (15): `then`: a reconciliation that records the control as having taken effect, on
  an instance that exists, names the instance's current revision, and the values the control
  was sent, which the engine keeps under OPS-008, are stored. Observable: drop the `settings`
  input and its shape; keep the revision, retired-instance and unregistered sentences, with
  "no longer exists".
- REV-011 (10): observable: the list drops `rename.v1`.
- REV-012 (10, 23): observable: drop "renames among them"; "What happens to a request ... is DL9"
  → "A request that arrives meanwhile is REV-014."
- REV-013 (15): `when`: a reconciliation records a control as having taken effect, and the
  values it was sent, applied to the current configuration, would make it fail the type's
  schema. `then`: not dispatched, invalid. Observable drops the `settings` key sentence.

Added:

- REV-014 (convention, must; 23): A request to set up, retire or change an instance, other
  than a repeat under OPS-009, that arrives while another setup, retirement or change of it has
  been dispatched and has not ended, the engine's own included, is not dispatched and reports
  conflict at once.
- REV-015 (convention, must; 15): A control whose supplied values, applied to the stored
  configuration at the pointers of its `changes`, would make it fail the type's schema is not
  dispatched and reports invalid.

Unchanged: REV-002, REV-005, REV-009.

Cited by, outside the file: REV-008 by `spec/core/types.yaml`, `spec/schemas/README.md`.

#### `spec/conventions/records.yaml`

Rewritten:

- REC-001 (P5, S4.6, 17): `when` adds: a provider call (PRV-005); an object is removed.
  Observable: a secret read is recorded as a request (SEC-015).
- REC-002 (24c, 2): observable: drop the OBS-007 sentence; add that a record's form is the core
  schema `record.v1`, a CloudEvent with these fields in `data`, and that an engine may also
  export records in that form (DL8); the record of a read keeps no value (SEC-015) and of a
  secret reference only the resolved form (SEC-004).
- REC-003 (17): observable: "unregistered" → "unregistered or removed".
- REC-004 (C33, 6c, 2): `then`: a purge removes the records made before a time the request
  names, and leaves a record of what it purged; drop the list of kept fields. Observable: what
  repeats and reconciliations need is engine state under OPS-008, which no purge touches, so a
  repeat is still answered under OPS-009 to OPS-012, OPS-024 and OPS-032 and an unknown outcome
  can still be reconciled; drop the retrieval and runtime sentences.

Unchanged: REC-005, REC-006.

### G5 interface

Decisions written here: 1b (wire), 6b, 6c (wire), 16b, 18, 22 (DL14), 23 (DL9), 24a, 25 (wire),
25c, 26a, 27, 30, C51, C52, and the implementation-defined items of 2a, 8e and 21. Cites:
OPS-009, OPS-032 (G4); TPL-017 (G1); PRV-007, PLC-023 (G2); SEC-007, SEC-013 (G7); CTR-002
(G6).

#### `spec/interface/implementation.yaml`

Removed:

- IFC-024 (6c): the retrieval tool goes.
- IFC-034 (14): `obliges` goes.

Rewritten:

- IFC-001 (25a, 25b): `when`: "reached by `command`, `package` or `remote`". Observable
  unchanged otherwise.
- IFC-004 (18): `then` adds the protocol's extension negotiation (`server/discover`,
  `clientCapabilities`, an extension's `revision` setting, the error -32021) and tool
  annotations.
- IFC-006 (1b, 2, 11, 18): `then`: the keys are `instance`, `operation`, `invocation`,
  `caller`, `restrictions`, `config`, the instance's configuration, and `revision`, its
  revision; `operationIdentity` where the caller supplied one or the request is a hook
  (OPS-028); `conduit` where it came through a Conduit. Drop `interfaceVersion` and the OBS-007
  sentence: every request has a caller. Observable: drop `v0` (now IFC-037), the step sentence
  and `<digest>#<name>`; a contract of a submitted project is named `address#name`; the
  configuration holds references in the resolved form and secrets only as references.
- IFC-008 (18): `when`: an implementation receives a request that declares Conduit's
  extension at a `revision` it does not support. `then`: it performs nothing and answers with
  the protocol's error -32021. Observable rewritten accordingly.
- IFC-011 (16a): `when`: "An operation's input or output is a handle: a top-level property whose
  schema is `github.com/shrav-k/conduit/core#handle.v1`".
- IFC-012 (5, 6c, 4b, C51): `then`: the tools are exactly the operations its open Conduits
  list and for which IFC-033 gives a definition, other than `reveal-secret.v1` (SEC-007); a tool
  is named by the Conduit's local name and the operation's export name, joined by a dot.
  Observable: drop the retrieval-tool, component and length sentences; a call to a tool not
  presented is answered as now; the list's notifications are IFC-038.
- IFC-014 (5, 6c): `when`: a tool name it would have equals another tool's name presented to
  the same consumer (drop the length case, which no name can now reach: 63 + 1 + 63 < 128).
  Observable drops the IFC-024 and CON-026 sentences.
- IFC-016 (6c): `when`: an invocation outlasts its call. `then`: the protocol's tasks
  extension, `io.modelcontextprotocol/tasks`, may be used where both sides declare it;
  `tasks/get` and `tasks/cancel` are decided anew under the Conduit the invocation was
  dispatched through; it never stands in for the operation identity. Observable drops the
  retained-result sentences.
- IFC-017 (2, 2a): level `may` → `must`. `when`: the engine reaches an implementation. `then`:
  an item that is run is reached at the `endpoint` its run returns, or over stdio by the
  engine's own runner (PRV-007); a `remote` item over Streamable HTTP at its URL.
- IFC-018 (3a, 7, 9, 11, 14, C20): `then`: the list holds every filter that applies: the
  filters of the request's own Conduit and those carried under ACC-017 and ACC-018; a
  requirement of the core package is never sent; each item holds `requirement`, its qualified
  reference, `address#name` for a submitted project's, and its stored `arguments`. Observable:
  drop policies, compositions, re-exports, `obliges` and the shared-host sentence's
  "constituent".
- IFC-019 (1, 7): observable: "a step that has no `conduit`" → "a hook"; drop the composed
  sentence.
- IFC-021 (6c): `when` drops "other than one for `retrieve-result.v1`"; observable drops the
  retrieval sentences.
- IFC-022 (26a, R40 3e): `then` adds: the mechanism establishes exactly one instance or none
  from what the request presents, by a cryptographic proof of the instance that sent it
  wherever the request crosses machines; what it presents says who is asking and nothing more,
  and the Conduits still decide. Observable: mutual TLS, SPIFFE and the protocol's authorization
  are each one such mechanism (DL13); PRV-018.
- IFC-025 (6a, 6b, 6c, 24a): `then`: `invocation` is held where the request has an outcome or
  is a repeat under OPS-032; drop `state` and `withheld`; `isError` is true where the request
  ended with a reason or its outcome is failed or unknown, and in no other case; a result whose
  `isError` is not true holds the outputs. Observable: a result that reports invalid or
  conflict carries, in `diagnostics`, an error diagnostic whose `ruleId` is the ID of the
  promise that decided it; a refused result carries none (OPS-023); drop the withheld,
  retrieval and composed sentences.
- IFC-028 (6c): drop "where the request is not a call to `retrieve-result.v1`" and the
  retrieval observable sentences.
- IFC-029 (18): `then`: a request carries the keys of one version only; a request to an
  implementation declares the extension's `revision` for its version; a request to the engine
  carries keys under one prefix, and one with keys under two cannot be read and reports
  invalid. Observable: drop `interfaceVersion`.
- IFC-032 (6c): observable drops "the tool of IFC-024 included".
- IFC-033 (16a, 16b): `then`: the definition is the contract's `inputSchema` and
  `outputSchema`, each `$ref` to a Schema contract replaced by that contract's schema; for a
  control, `required` lists `revision` alone (REV-006); `annotations.readOnlyHint` is true only
  where the contract states no `effect`, is not a control and does not start model work, and
  `idempotentHint`, `destructiveHint` and `openWorldHint` are passed on as the contract states
  them. Observable: drop the step and IFC-034 sentences.

Added:

- IFC-037 (interface, must; 18): The engine reads an implementation's discovery result before
  its first request and declares Conduit's extension on each request; where that result does
  not declare the extension at the revision `v0`, a request that would carry Conduit's keys is
  not dispatched and reports unavailable, and is never sent without them.
- IFC-038 (interface, must; C51): The server the engine presents declares
  `tools.listChanged`, notifies each stream that asked when the instance's tool list changes,
  and marks its tool lists `cacheScope: private`.
- IFC-039 (interface, must; C52): Trace context travels only in the `_meta` keys that the
  protocol reserves for it, as OpenTelemetry's conventions for the protocol give them.
- IFC-040 (interface, must not; C52): The engine takes a caller, a restriction or an access
  decision from trace context.
- IFC-041 (interface, must; 25c): When the engine connects to a `remote` item to serve an
  instance, it sends each header the item's `headers` names with the value at its place in the
  instance's configuration, a secret's value being read under SEC-013.

Unchanged: IFC-002, IFC-003, IFC-005, IFC-007, IFC-009, IFC-010 (27: the relay is not in v0),
IFC-013, IFC-015, IFC-020, IFC-023, IFC-026, IFC-027, IFC-030, IFC-031, IFC-036.

Cited by, outside the file: IFC-024 by ACC-019, LIF-031, OPS-014, REC-004, `spec/core/*`,
definitions, `docs/OVERVIEW.md`; IFC-034 by CON-022, CTR-005, `spec/core/README.md`,
definitions.

#### `spec/implementation-defined.yaml`

Removed: DL14 (22): cancellation is the protocol's (OPS-027).

Rewritten (the item's `subject` and `document`):

- DL1 (1): subject "Order of a Pool's Resources"; only the order in which
  `retire-with-resources.v1` retires Resources that depend on each other in a cycle, or not at
  all, under LIF-021.
- DL2 (C33): "which a purge keeps under REC-004" → "which the engine keeps under OPS-008".
- DL3 (6b): "a repeat of its request is told `inProgress` under OPS-009" → "a repeat of its
  request reports conflict under OPS-032".
- DL5 (8e): subject "The machines provider's report"; how often the engine asks for it and
  what it does while none arrives. What a request reports meanwhile is PLC-023.
- DL6 (8): drop "and the stored facts that the location `local` is given".
- DL7 (21): drop "how they are fetched" and "which schemes of `fetch` are accepted"; add
  "whether it keeps a cache, and how the user names a mirror, each of which gives the same bytes
  under PKG-026".
- DL8 (24c): add "and whether records are exported in the form of `record.v1`".
- DL9 (23): subject "Order of failed checks"; only the order in which several failed checks
  that give the same reason are reported.
- DL10 (6c): "how long the content of a retained result is kept for retrieval under OPS-016" →
  "how long the outputs of a succeeded invocation are kept for a repeat under OPS-009".
- DL12 (2a): subject "Protocol revisions and `content`"; drop the transport, who starts a
  process and how many instances share one.
- DL13 (26a): the mechanism an engine uses under IFC-022, which establishes exactly one
  instance or none from what a request presents, by a cryptographic proof wherever the request
  crosses machines; on one machine over stdio, what the engine's own runner gives the process.
- DL15 (30, 5): drop "and, for a component Conduit, CON-003"; add that the generator of the
  Kubernetes API server, five lowercase alphanumeric characters, is one choice that meets it.

Unchanged: DL4, DL11. Head comment as rule 7, naming DL14 as removed.

### G6 language

Decisions written here: 7, 9, 10, 11, 12, 16a, 19 (contract side), 20, 21 (language side),
25 (manifest), 25d, C14, C20 (contract side), C26 (P12); and the language side of 1, 2, 3a, 5,
8a, 14, 31b. Cites: PRV-001, PRV-006, PRV-016, PLC-025 (G2); TPL-012, TPL-019 (G1); SEC-001
(G7); IFC-033 (G5).

#### `spec/promises/objects.yaml`

Removed: OBJ-005, OBJ-026, OBJ-027, OBJ-028 (10): rename goes; OBJ-011 states that a name is
fixed.

Rewritten:

- OBJ-003 (5, 30): observable drops the component-Conduit sentence and "From outside its Pool"
  stays.
- OBJ-010 (10): `when`: "... the object it resolved to no longer exists while another object
  has taken its name" (drop "has another name, or").
- OBJ-011 (10): `when`: "An object exists". `then`: no document and no operation changes its
  name, which is fixed once it is made, and no document rewrites an instance's definition.
  Observable: drop the OBJ-026 pointer.
- OBJ-013 (31b, 8, 8d): `then`: the standard arguments add `runtime` for an Agent and a
  Resource, which may be left out (PRV-006); `placement` left out is chosen under PLC-002; drop
  "a location's document leaves it out, under PLC-008".
- OBJ-014 (5, 31b): `then` drops the component; observable: an entry that carries `owner`,
  `placement` or `runtime` is rejected.
- OBJ-015 (16a, 1, 2): `when`: "A template's input, a Requirement contract's input, a declared
  Conduit or a Pool's entry is to be optional". `then` adds: an Operation contract's input or
  output is optional where its `inputSchema` or `outputSchema` does not list it under
  `required`. Observable: drop steps, graph outputs and runtimes.
- OBJ-016 (1, 25d): `when`: an input's destination, a value of a control's `changes`, or a
  pointer of an item's `environment`, `arguments` or `headers` (drop the value reference). The
  root of the last three is the instance's configuration.
- OBJ-019 (1, 3a, 7, 25): exemptions: drop the literal of a value binding, `steps` and
  `outputs`, the `object` binding and the constituent; add the inside of an item's `package` or
  `remote`, which server.json's schema governs, and the keys of `environment`, `arguments` and
  `headers`. Observable: replace "a policy written in a template" by "`runtimes` in a template";
  drop the step and constituent sentence.
- OBJ-021 (1, 15, S4.1): observable: drop "the one a value reference with the source `config`
  yields" and "the one a control returns in `settings`"; add "the one an implementation
  receives in `config` under IFC-006"; add that `secret-reference.v1` is a reference schema too,
  with the narrower rule of SEC-001.
- OBJ-022 (1, 2, 5, 7, 8a, 20): `when`: the `inputs`, `conduits`, `entries` or `requirements`
  of a template, or the `inputs` of a Requirement contract (drop runtimes, components, Type
  `placement`, constituents, steps). Observable: the rule is stated in the schemas by
  `propertyNames` (20); the keys of a label selector are labels, not names.

Unchanged: OBJ-001, OBJ-002, OBJ-004, OBJ-006 to OBJ-009, OBJ-012, OBJ-017, OBJ-018, OBJ-020,
OBJ-023 to OBJ-025.

Cited by, outside the file: OBJ-005 by `spec/core/*`, `spec/schemas/values.yaml`, definitions;
OBJ-026 by REV-003, `spec/core/*`, definitions, `tools/check_schemas.py`; OBJ-027, OBJ-028 by
definitions.

#### `spec/promises/contracts.yaml`

Removed: CTR-010, CTR-011 (7): requirement composition goes.

Rewritten:

- CTR-002 (16a, 16b): `then`: the `spec` may hold `inputSchema` and `outputSchema`, as the
  protocol's Tool writes them (CTR-012 limits their root); a handle is a top-level property
  whose schema is `$ref: github.com/shrav-k/conduit/core#handle.v1`, which states no `default`;
  further `effect`, `startsModelWork`, `deduplicates`, `reservedToRoot`, `implementation`,
  `changes` for a control, and the booleans `idempotentHint`, `destructiveHint` and
  `openWorldHint`. Observable: drop the member form, `handle: true` and `optional: true`;
  property names may be any non-empty string; an `outputSchema` property that states a
  `default` is rejected; IFC-033 says what the hints become.
- CTR-004 (2, 8a, 20, P12): `then`: `spec` holds `family`; may hold `schema`, `offers`,
  `requires`, `implementation`, `controls`, `conditions`, `records` and `placement`, a label
  selector. Drop `runtimes` and `restoresRuntimes`. Observable: drop "or with no `placement`
  key" and the runtime sentences; the Pool rule is now also in `type-spec.v1` (20); a provider
  Type is PRV-001.
- CTR-005 (3a, 7, 8a, 14, C20, P12): `then`: its `spec` may hold `inputs`, a map from a name to
  a member that holds `schema` and may hold `default` and `optional: true`; a requirement is
  applied only to a Conduit; one of the core package is applied by the engine and never sent,
  any other is a filter applied by the target's implementations. Observable: drop `appliesTo`,
  `narrows`, `obliges`, `constituents`, placement and policies.
- CTR-006 (15, 16a): `then`: a control is an Operation contract with `changes`, whose
  `inputSchema` requires `revision`, and which declares no output `settings`. Observable: drop
  `settings`; a type that lists a control whose `inputSchema` does not require `revision` is
  rejected.

Added:

- CTR-012 (language, must; 16a): An Operation contract whose `inputSchema` or `outputSchema` has
  at its root anything but `type: object`, `properties`, `required` and `additionalProperties` is
  rejected when its package is installed or its project submitted.

Unchanged: CTR-001, CTR-003, CTR-007 to CTR-009.

Cited by, outside the file: CTR-010 by IFC-018, OBJ-019, `spec/core/*`, `spec/schemas/*`, the
two CTR-010 examples and the package example, `tools/check_schemas.py`; CTR-011 by ACC-029,
CON-028, PKG-023, `spec/core/*`, `spec/schemas/*`, `tools/check_schemas.py`.

#### `spec/promises/schemas.yaml`

Rewritten:

- SCH-008 (20): `then` and observable: a `default` is also rejected under `not`, `if`, `then`,
  `else` and `dependentSchemas`, beside `allOf`, `anyOf` and `oneOf`.
- SCH-009 (20, 3a, 1): observable: a `default` reached only through a `$ref` inside any of those
  eight is not applied; drop "or of a policy under NAM-019 and ACC-042" and "whether a request or
  a step of a graph supplies it".
- SCH-019 (16a): observable: an Operation contract's input default is a property's `default` in
  its `inputSchema`, checked as any default against the schema that states it.

Unchanged: SCH-001 to SCH-007, SCH-010 to SCH-018, SCH-020, SCH-021.

#### `spec/promises/naming.yaml`

Rewritten:

- NAM-009 (11, P11): `then`: for a contract or an implementation of its own submitted project,
  it keeps the project's address, its version where the manifest states one, and the digest of
  that submission under SUB-008. Observable: that item has `address`, `digest` and, where
  stated, `version`.
- NAM-012 (21, 12): `then` adds: every engine fetches `https:` and `file:` URLs, a relative
  `fetch` resolving to `file:`; other schemes are fetched by a package-source provider under
  PRV-017; an engine may take the same bytes from a cache or a mirror the user names; a URL
  never carries a credential. Observable: `install.v1` with no `fetch`, for a package not
  installed, reports invalid under PKG-023; a registration whose manifest lists a package that
  is not installed is rejected under PKG-006.
- NAM-018 (1, 2, 7): `then` drops "may instead be a graph written in place"; observable drops
  "of a constituent", "a step's `operation`" and "the `start` and `stop` of a runtime".
- NAM-019 (3a, 8a): `when`: "A document applies a requirement to a Conduit"; observable drops
  the policy sentence.
- NAM-020 (8d): observable: the example reference becomes
  `github.com/shrav-k/conduit/core#interface.v1`.
- NAM-021 (25): observable: "none of `command`, `url` and `graph`" → "none of `command`,
  `package` and `remote`".
- NAM-022 (11): observable: a contract of a submitted project is named `address#name` outside
  its own documents (SUB-008); "the unpublished package that the template being replaced holds"
  → "the project contracts that the template being replaced holds".
- NAM-026 (1): observable drops the step sentence.
- NAM-028 (9, 12, 1): `when` drops the re-export; observable drops the re-export example, "a
  step's `operation`" and the deferral sentence ("Only a package that is not installed defers"),
  since no registration is made with a package missing.

Unchanged: NAM-001 to NAM-008, NAM-010, NAM-011, NAM-013 to NAM-017, NAM-023 to NAM-025,
NAM-027.

#### `spec/promises/packages.yaml`

Removed: PKG-013, PKG-014 (9): re-exports go.

Rewritten:

- PKG-005 (11): observable: a submitted project's contracts are listed with its address, its
  version where stated, and the digest.
- PKG-006 (12): `when`: a registration's manifest lists a package that is not installed at
  that version and digest. `then`: the registration is rejected, with a diagnostic naming each
  such package. Observable: drop INS-022; the reference engine's command line may offer one
  command that installs them first, which is no promise.
- PKG-019 (25, 8a, 1, 7): `then`: each item states `implements` and exactly one of `command`,
  `package` and `remote`; it may state `placement`, a label selector, and `supports`; a
  `command` or `package` item may state `environment`, a `package` item `arguments`, and a
  `remote` item `headers`. Observable: drop graphs and compositions.
- PKG-020 (25, 1): `then`: "A plain implementation is reached by `command`, `package` or
  `remote`"; drop the graph clauses. Observable: a plain server learns its configuration only
  through `environment`, `arguments` or `headers` (25d).
- PKG-023 (21, 9): `when` adds: the request gives no `fetch` for a package that is not
  installed; its `fetch` has a scheme that neither the engine nor a package-source provider
  accepts; or its `fetch` carries a credential. Observable: drop PKG-014 and CTR-011 from the
  list of causes.
- PKG-024 (21): observable: "How an engine fetches an archive is DL7" → "An archive is fetched
  under PRV-017".
- PKG-028 (12): `then`: the installation is rejected, with a diagnostic naming each package its
  manifest lists that is not installed; the request reports invalid under PKG-023.

Added:

- PKG-029 (convention, must; 11, P10): A request to install a package at an address that the
  manifest of a submitted project states is not dispatched and reports conflict.

Unchanged: PKG-001, PKG-003, PKG-004, PKG-007, PKG-012, PKG-016 to PKG-018, PKG-021, PKG-022,
PKG-025 to PKG-027.

Cited by, outside the file: PKG-013 by ACC-029, CON-022, IFC-018, INS-025, `spec/core/README.md`,
definitions; PKG-014 by `spec/schemas/*`.

#### `spec/promises/manifest.yaml`

Removed: MAN-008 (9): re-exports go.

Rewritten:

- MAN-001 (C14): the one file named `conduit.yaml`, `conduit.yml` or `conduit.json`.
- MAN-002 (C14): a directory, or several files, holding more than one of those three names is
  rejected.
- MAN-004 (11): `then`: beside `apiVersion` it may hold `packages`, `exports`,
  `implementations`, `address` and `version`; `version` only beside `address`. Observable: a
  package's manifest states both (PKG-017); a submitted project's manifest states `address`
  where MAN-015 requires it and may state `version`; its contracts are then `address#name`
  outside its documents (SUB-008).
- MAN-006 (21, 20): `then`: a URI reference, checked by the format `uri-reference`; a relative
  one resolves against the manifest's directory to a `file:` URL, and is rejected without one;
  it carries no user information. Observable: drop DL7; schemes are NAM-012 and PRV-017.
- MAN-007 (9): `then`: each item is the name of a document of the package; observable drops the
  re-export form.
- MAN-009 (25, 1): `then`: `implements`; exactly one of `command`, `package` and `remote`; and,
  where given, `plain`, `tools`, `supports`, `placement`, `digest`, `environment`, `arguments`
  and `headers`.
- MAN-010 (25, 8a, P15): `then`: `command` a list of strings; `package` one Package as
  server.json's schema 2025-12-11 gives it, with `digest` beside it; `remote` one Remote of that
  schema whose `type` is `streamable-http`; `environment` a map from a variable name to a JSON
  Pointer; `arguments` and `headers` maps from a name the Package or Remote declares to a JSON
  Pointer; `placement` a label selector. Observable: drop `url`, `graph` and GRA-014; a name
  that the Package or Remote does not declare is rejected; "Who starts the program is DL12" →
  "The runtime provider runs it (PRV-007)".
- MAN-011 (C14): the documents are the files ending `.yaml`, `.yml` or `.json`; drop "`.yml`
  among them".
- MAN-012 (25, C20): `then`: each item states `implements` and none of `command`, `package` and
  `remote`, and states no `supports`. Observable: the engine applies the core package's
  requirements itself and is sent none; it implements no provider operation and not
  `reveal-secret.v1` (SEC-020).

Added:

- MAN-015 (language, must; 11): A submitted project that holds a contract document or lists an
  implementation, and whose manifest states no `address`, is rejected.
- MAN-016 (language, must; 11): A submitted project whose manifest states the address of a
  package that is installed is rejected.
- MAN-017 (language, must; 25d): A template that selects an item whose `environment`,
  `arguments` or `headers` gives a pointer to a place that its type's schema does not require,
  with every member on the way required, is rejected.

Unchanged: MAN-003, MAN-005, MAN-013, MAN-014.

Cited by, outside the file: MAN-008 by `spec/schemas/manifest.yaml`, the package example,
definitions.

#### `spec/promises/submissions.yaml`

Rewritten:

- SUB-005 (C14): `.yaml`, `.yml` or `.json`.
- SUB-008 (11, 12, P11): `then`: the contracts and implementations are available to the
  project's documents as `#name`, and elsewhere, on the wire and in a bound, as `address#name`
  with the address its manifest states (MAN-015); the engine identifies each submission's
  contracts by the digest, computed as now; drop "Where a contract of the project needs a
  package that ... is not installed, the submission is accepted". Observable: drop the
  `<digest>#<name>` and missing-package sentences; a contract that needs a package that is not
  installed makes the registration rejected under PKG-006.
- SUB-009 (11): "the unpublished package that the template being replaced holds" → "the project
  contracts that the template being replaced holds", in `then` and observable.
- SUB-010 (C14, 12): observable: "a package that the manifest lists and that is not installed
  is reported ... as PKG-006 reports it" → "is rejected under PKG-006"; the manifest names of
  MAN-001.

Unchanged: SUB-001 to SUB-004, SUB-006, SUB-007.

#### `spec/promises/instantiation.yaml`

Removed: INS-022 (12): no registration is made with a package missing, so no check is deferred.

Rewritten:

- INS-012 (5, 31b, P14): `when`: drop the composite clause and add an entry's `runtime`.
  Observable: drop `/components/read/arguments/root`, the composite sentences and `runtimes`,
  `setup`, `retire` from the rejected list.
- INS-016 (5): `then` drops the composite sentence.
- INS-025 (9): observable drops the PKG-013 sentence.

Unchanged: INS-001 to INS-011, INS-013, INS-014, INS-017 to INS-021, INS-023, INS-024.

Cited by, outside the file: INS-022 by NAM-028, PKG-006, PKG-028, SUB-008, TPL-004.

#### `spec/schema-profile.md`

- (20) "Schema keywords in the profile": the applicator vocabulary (`allOf`, `anyOf`, `oneOf`,
  `not`, `if`, `then`, `else`, `dependentSchemas`, `prefixItems`, `items`, `contains`,
  `properties`, `patternProperties`, `additionalProperties`, `propertyNames`); the validation
  vocabulary except `multipleOf` (`type`, `enum`, `const`, `maximum`, `exclusiveMaximum`,
  `minimum`, `exclusiveMinimum`, `maxLength`, `minLength`, `pattern`, `maxItems`, `minItems`,
  `uniqueItems`, `maxContains`, `minContains`, `maxProperties`, `minProperties`, `required`,
  `dependentRequired`); `format`, `default`, `title`, `description`, `examples`, `$ref` in its
  three forms and `$defs`. Outside it: `$id`, `$schema`, a remote `$ref`, `$anchor`,
  `$dynamicRef`, `$dynamicAnchor`, `$vocabulary`, `$comment`, `unevaluatedProperties`,
  `unevaluatedItems`, `multipleOf`, the content vocabulary, `deprecated`, `readOnly` and
  `writeOnly`.
- (20) Formats: add `uri-reference` (RFC 3986, section 4.1); the `fetch` row cites it.
- (20) The `default` paragraph names the conditional keywords of SCH-008.
- (21) The `fetch` row: `https:` and `file:` (RFC 8089) on every engine.
- (25) Rows for an implementation's `remote` URL and for the MCP Registry's server.json, schema
  2025-12-11 (MAN-010).
- (24) Rows for SARIF 2.1.0 (`diagnostic.v1`), CloudEvents 1.0 (`record.v1`) and Kubernetes
  `metav1.Condition` (`condition.v1`).
- (1) "Graphs and schemas are not what distinguishes Conduit" → "Schemas are not"; drop the
  sentence on a step's use of an output; keep "depends on" only for entries.
- Head paragraph: add ruling 41.

### G7 secrets

Decisions written here: section 4 points 1 to 8, 4a to 4c, 13a, 13b, 19 and the secret side of
25c and 25d. Cites: PRV-007, PRV-010, PRV-011 (G2); TPL-012, TPL-018 (G1); ACC-003, ACC-039
(G3); IFC-006, IFC-012, IFC-041 (G5); MAN-010, MAN-017 (G6); REC-002 (G4).

#### `spec/promises/secrets.yaml` (new)

Head comment: as section 2 says; add the limit of the guarantee as a note, pointing to
`spec/README.md`, and what moves into each store package's contract (its locator's form and
schema, versions or stages, one key of a structured secret, caching, its own credential and
what it needs from where it runs, where it must run, and for the file store its encoding,
final newline and largest value).

- SEC-001 (language, must; S4.1, 2b, 20): A place that the core schema `secret-reference.v1`
  governs, directly or as the `items` of a list, holds only a reference to a Resource, a literal
  there making the document rejected; and a schema that puts it under an applicator beside any
  branch other than `{type: null}` is rejected.
- SEC-002 (language, must; S4.1, 19): A reference in such a place that resolves to a Resource
  whose type does not offer `secret-reveal.v1` makes the document that holds it rejected.
- SEC-003 (language, must not; S4.1): A diagnostic holds a value written in such a place, or
  any part of it.
- SEC-004 (convention, must not; S4.1): A record holds a value at such a place in any form other
  than the resolved form of OBJ-009, whatever the type's `records` states.
- SEC-005 (language, must; S4.2, 19): A Type that offers `secret-reveal.v1` lists no control,
  and one that lists one is rejected, so only the root Interface's choice of Conduit decides
  what a reader receives.
- SEC-006 (interface, must; S4.3, 1b): An implementation of `reveal-secret.v1` takes the value
  only from the store that the locator in its instance's configuration names, never from that
  configuration and never from another store, and answers failed with no value where it cannot
  read that store.
- SEC-007 (convention, must; 4b): An actor's request for `reveal-secret.v1`, on its own behalf
  or as an onward request, is not dispatched and reports refused, whatever Conduit lists it.
- SEC-008 (convention, must; S4.4): A request to open a Conduit that lists `reveal-secret.v1`
  is dispatched only where the actor is the root Interface and the consumer is a Resource or an
  Agent, and otherwise reports refused.
- SEC-009 (language, must; S4.4): A Conduit that lists `reveal-secret.v1` and whose consumer is
  an Interface is not instantiated.
- SEC-010 (convention, must; 4a, 2c, 25d, 13b, S4.5): Before each run that the engine asks a
  runtime provider for, it reads each secret that the item binds under `environment` or
  `arguments`, through the instance's own open Conduit to that secret Resource that lists
  `reveal-secret.v1`, and passes the values in the run's `secrets`. Observable: a store's own
  credential is such a secret, and a chain of stores ends at one that binds none.
- SEC-011 (convention, must; 4c): A read belongs to the request that asked for the run or the
  connection, the setup of the instance or a retirement that serves its retire hook; its record
  names that invocation and its caller, and it is decided by the instance's own Conduit and the
  filters on it alone, carrying none of that caller's restrictions.
- SEC-012 (convention, must; S4.5): Where a read does not succeed, no run is asked for and no
  connection is made, and a request that was to be dispatched to that implementation is not
  dispatched and reports unavailable.
- SEC-013 (convention, must; 25c): When the engine connects to a `remote` item for an
  instance's setup, it reads each secret that the item's `headers` binds through the instance's
  own Conduit, sends the value only to that item's URL and holds it only for that connection.
- SEC-014 (convention, must; S4.5, 2c): A value read for a run is fixed for the life of that
  run: a restart its provider makes reuses it, and closing the Conduit it was read through
  recalls nothing.
- SEC-015 (convention, must not; S4.6): The record of a read holds the value, or any part of it.
- SEC-016 (convention, must; S4.6): A read whose record cannot be kept is not dispatched and
  reports unavailable.
- SEC-017 (convention, must not; S4.7): The engine keeps a value it read to stand in for a later
  read.
- SEC-018 (convention, must not; S4.7): The engine puts a value it read anywhere but the
  `secrets` of the run it was read for or the header of SEC-013: not in an inspection result,
  an observation, a diagnostic, a record, any result, or the `_meta` of any request.
- SEC-019 (interface, must not; S4.8): An implementation that carries Conduit's keys puts a
  value it was given into a result, a diagnostic, or the `arguments` or `_meta` of a request it
  sends to the engine.
- SEC-020 (convention, must not; 13a): The engine serves `reveal-secret.v1` itself, or gives a
  run any variable of its own environment.

## 4. Core package and schemas

The core-and-schemas editor works after G1 to G7, from this section and the promises as they
then stand. Every description in a core document cites promise IDs that exist after the work.

### Exports removed (16)

- Operations (6): `rename.v1` (10), `retrieve-result.v1` (6c), `apply-policy.v1`,
  `remove-policy.v1` (3a), `set-default-location.v1`, `set-facts.v1` (8).
- Capabilities (2): `system-settings.v1`, `location-settings.v1` (8).
- Types (1): `system-pool.v1` (8); the template `system` selects `pool.v1`.
- Schemas (7): `graph.v1`, `lifecycle-graph.v1`, `step.v1`, `value-binding.v1`,
  `value-reference.v1` (1); `runtime.v1` (2); `conduit-component.v1` (5). The file
  `spec/schemas/graphs.yaml` is deleted by editing it away (P2 applies to promise files only).

### Exports added (20)

- Operations (9), each in `inputSchema` form, none with a default implementation:
  - `setup-hook.v1`, `retire-hook.v1` (1a): no inputs, no outputs, an `effect`; in no
    capability.
  - `run.v1` (2, P18): inputs `instance` (`id.v1`), `item` (`implementation-item.v1`),
    `arguments`, `environment` and `secrets` (maps of strings), `location` (`hostname`,
    `labels`); output `endpoint`, optional, a `uri`. `stop.v1` (2a): input `instance`.
    `inspect-run.v1` (2a): input `instance`; output `state`, one of `running`, `stopped`,
    `unknown`; no `effect`.
  - `list-machines.v1` (8, C58): output `machines`, a list of `hostname`, `labels`
    (`label-selector.v1`) and `state` (`reachable`, `unreachable`, `gone`); no `effect`.
    `place.v1` (8c): inputs `instance`, `selector`; output `hostname`, optional; no `effect`.
  - `fetch-package.v1` (21, P17): inputs `address`, `version`, `digest`, `fetch`; output
    `archive`, a base64 string; no `effect`.
  - `reveal-secret.v1` (S4.2): no inputs; output `value`, a string; no `effect`.
- Capabilities (4): `runtime-provider.v1` [`run.v1`, `stop.v1`, `inspect-run.v1`];
  `machine-provider.v1` [`list-machines.v1`, `place.v1`]; `package-source.v1`
  [`fetch-package.v1`]; `secret-reveal.v1` [`reveal-secret.v1`].
- Schemas (7): `handle.v1` (16a, a string) and `condition.v1` (24b) and `record.v1` (24c, P19)
  in `spec/core/results.yaml`; `secret-reference.v1` in `spec/core/references.yaml` (a reference
  schema of the kind Resource, no bare name, accepting the resolved form); `label-selector.v1`
  (8a; keys in the Kubernetes label-key syntax, values strings) in `spec/schemas/values.yaml`;
  `server-package.v1` and `server-remote.v1` (25, transcriptions of server.json 2025-12-11's
  Package and Remote, the Remote limited to `streamable-http`) in
  `spec/schemas/manifest.yaml`.

### Exports changed

- Capabilities: `object-management.v1` loses `rename.v1`; `access-control.v1` loses
  `apply-policy.v1` and `remove-policy.v1`.
- Types: `location.v1` has `schema: {type: object, additionalProperties: false}`, no `offers`,
  no `requires`, no `controls` (8); every core type drops `placement: {}`, which is now
  optional and empty means nothing (P12); `requires` lists drop the removed operations.
- Requirements: `bound.v1` and `shared-host.v1` lose `appliesTo` and `narrows` (C20, P12).
- Operations: every contract moves to `inputSchema` and `outputSchema` (16a).
  `register-replacement.v1` takes `instance` and `nested` (P9). `reconcile.v1` loses `handle` and
  `settings` (2a, 15). `inspect.v1`'s `records` is a list of `record.v1` and its `status`
  describes `conditions` as a list of `condition.v1` (24b, P19). `set-up.v1`, `retire.v1` and
  `unregister.v1` get new `effect` text (1, 2, 17). `install.v1`'s `fetch` is a
  `uri-reference` (21).
- Results: `diagnostic.v1` becomes the SARIF subset of P16, `ruleId` being a promise ID.
- Templates: `system` selects `pool.v1`; `location` has no input and no `config` (8);
  `root-management` drops `rename.v1`; `root-access` drops the policy operations.
- Manifest `spec/core/conduit.yaml`: exports updated; `engine` implements every operation of
  `object-management.v1`, `access-control.v1` and `pool-management.v1`, and no other; it states
  no `supports` (C20).

Totals after: 102 exports, of which 23 Operation contracts, 7 Capability contracts, 5 Type
contracts, 2 Requirement contracts, 58 Schema contracts (12 reference schemas, 5 in
`results.yaml`, 41 under `spec/schemas/`) and 7 templates; one implementation.

### Schema changes

- `operation-spec.v1` (16a, 16b, 15): `inputSchema`, `outputSchema` (root limited as CTR-012),
  `effect`, `startsModelWork`, `deduplicates`, `reservedToRoot`, `implementation`, `changes`,
  `idempotentHint`, `destructiveHint`, `openWorldHint`; no `inputs` or `outputs`; an
  `outputSchema` property may not state `default`.
- `type-spec.v1` (2, 8a, 20): drop `runtimes`, `restoresRuntimes`; `placement` optional,
  `label-selector.v1`; a Pool type's `offers` contains `pool-management.v1`, written with `if`,
  `then` and `contains` (20).
- `requirement-spec.v1` (C20, 7, 14, P12): `inputs` only.
- `template-spec.v1`, `pool-template-spec.v1` (1, 2, P14): drop `runtimes`, `setup`, `retire`;
  the Pool destination pattern adds `/entries/<name>/runtime`.
- `conduit-template-spec.v1` (5): no `components` branch, no composite destinations.
- `implementation-map.v1` (1): values are selections only.
- `instance.v1` (31b): Agent and Resource accept `runtime`.
- `manifest.v1` (9, 11, 20): `exports` items are names; `version` requires `address`; `fetch`
  is `format: uri-reference`; keys of `packages` by `propertyNames`.
- `implementation-item.v1` (25, 8a): exactly one of `command`, `package`, `remote`; `digest`
  required with `package`; `environment`, `arguments`, `headers`, `placement`; no `url`,
  `graph`.
- `schema.v1` (20): the profile of G6's `schema-profile.md`, with `uri-reference`, and a
  `default` rejected under the conditional keywords.
- `values.yaml` (20, C21): `revision.v1` an opaque non-empty string; named maps elsewhere use
  `propertyNames` with `name.v1`.
- `spec/core/README.md` and `spec/schemas/README.md`: counts, tables of exports and files,
  "What a schema cannot express" (drop the rows that 20 moves into schemas and every graph,
  runtime, composite, constituent, re-export and `obliges` row; add the rules below), "Choices
  the schemas encode", and each head paragraph names ruling 41.

### Examples

Remove from `examples/invalid/` (19): `CON-016-capability-and-components`,
`CTR-002-input-with-schema-and-handle`, `CTR-004-type-without-placement`,
`CTR-005-obliges-an-unknown-end`, both `CTR-010-*`, the six `GRA-*`,
`INS-012-destination-operations-in-a-composite`, `MAN-004-manifest-address-without-version`,
`MAN-009-manifest-item-with-command-and-url`, `MAN-010-manifest-url-not-a-uri`,
`PKG-019-manifest-graph-with-supports`, `PKG-020-manifest-graph-marked-plain`,
`TPL-006-runtime-marked-optional`.

Add to `examples/invalid/` (8): `CTR-012-operation-input-schema-with-another-root-keyword`,
`MAN-004-manifest-version-without-address`, `MAN-009-manifest-item-with-command-and-remote`,
`MAN-010-manifest-remote-url-not-a-uri`, `MAN-010-manifest-remote-with-another-transport`,
`SCH-008-default-under-if`, `OBJ-013-runtime-on-a-pool`, and
`CTR-004-pool-type-without-pool-management` (moved from `invalid-by-rule`, now a schema's).

Rewrite in `examples/invalid/`: `CTR-002-operation-arranges-later-work`,
`CTR-002-output-with-default`, `CTR-006-changes-as-a-list` (to `inputSchema` form);
`OBJ-019-depends-on-in-snake-case` (onto an entry, not a step);
`PKG-020-manifest-plain-with-supports` (`url` → `remote`);
`SCH-002-keyword-outside-the-profile` (`patternProperties` → `multipleOf`).

Remove from `examples/invalid-by-rule/` (12): `CON-030-*`, the `CTR-004-*` (moved), both
`CTR-005-*`, `CTR-010-*`, `CTR-011-*`, the three `GRA-*`, `PLC-008-location-with-placement`,
`TPL-006-*`, `TPL-016-*`.

Add to `examples/invalid-by-rule/` (8), one for each new rule: `FST-006-instance-from-location`,
`TPL-019-two-items-that-are-run`, `PLC-025-type-and-item-name-one-label`,
`MAN-017-environment-place-not-required`, `MAN-015-manifest-project-contracts-without-address`,
`SEC-001-secret-reference-beside-a-string`, `PRV-001-provider-type-of-another-family`,
`PRV-004-conduit-to-a-runtime-provider`.

Rewrite in `examples/invalid-by-rule/`: `SCH-019-*` and `TPL-004-*` (to `inputSchema` form).

Valid examples:

- `valid/project`: the Resource template drops `runtimes` and `setup`, and selects a setup
  hook; the composite Conduit template goes; the project's manifest states `address` (MAN-015);
  add an Agent instance that names `runtime`, a store Type offering `secret-reveal.v1` with a
  locator schema, a secret Resource, a type whose `config` holds a `secret-reference.v1`, and a
  `command` item whose `environment` binds it.
- `valid/files-package`: no re-export; the graph item becomes a `package` item with `digest`;
  the plain `url` item becomes `remote`; Operation contracts in `inputSchema` form, the watcher
  handle as `handle.v1`; the Type drops `runtimes` and `restoresRuntimes` and states a label
  selector; the Requirement drops `appliesTo`, `narrows` and `obliges`; the composed requirement
  and the placement requirement go.
- `valid/first-start`: `local` has no `facts`; comments cite PLC-026 in place of PLC-021.

Counts after: `invalid/` 52 files, `invalid-by-rule/` 20 files.

### Checker rules (`tools/check_schemas.py`)

- Remove from `RULES` (12): `location-placement`, `constituent-input`, `pool-offers`,
  `runtime-once`, `applies-to`, `narrows-on-conduit`, `obliges-narrows`, `bound-constituent`,
  `runtime-step-output`, `graph-invokes-itself`, `runtime-operation-step`, `conduit-names-one`.
- Change: `implementation-selected` (TPL-004: no runtimes or steps; no default for provider and
  hook operations); `names-nothing` (NAM-028: no re-export, no step); `default-valid` (SCH-019:
  walk `inputSchema`); `core-address` (NAM-021: `package` and `remote`).
- Add (8): `location-from` (FST-006), `one-run` (TPL-019), `label-conflict` (PLC-025),
  `item-place-required` (MAN-017), `project-address` (MAN-015), `secret-branch` (SEC-001),
  `provider-family` (PRV-001), `provider-conduit` (PRV-004). After: 19 rules.
- Step 2: the profile's keyword list and formats as `schema-profile.md` gives them; a `default`
  rejected under the conditional keywords (SCH-008).
- Step 5: `RETURNS_REVISION = ("replace.v1",)`; a control's `inputSchema` requires `revision`
  and declares no output `settings` (CTR-006); every Operation contract uses `inputSchema`
  form; the README tables as above.
- The module docstring's list of steps follows.

`spec/names.yaml`: no change. `tools/set_names.py` reads the keys from
`spec/interface/implementation.yaml`, so the keys added there (`config`, `revision`, the
extension's name) and removed (`interfaceVersion`, `state`, `withheld`) need no change to it.

## 5. Docs

### `spec/definitions.md`

- Head paragraph: add rulings 40 and 41 and this plan.
- Rows: every row of [section 1](#1-terms): the new ones added, the changed ones rewritten, the
  ones that go removed; "Words the promises do not use" gains `graph`, `step`, `policy`,
  `composite`, `re-export`, `rename`, `facts`, `retained result` and `isolation plugin`, each
  with what replaced it.
- Sections: "Graphs" goes; a section "Providers" holds provider, provider capability, runtime
  provider, run, machine, machines provider, label, label selector, package-source provider,
  provider call; a section "Secrets" holds secret store, secret Resource, secret reference, read;
  "Placement" is rewritten around machines and labels.
- Every row's promise IDs as they stand after the work; no removed ID remains.

### `spec/README.md`

- Opening paragraphs: the secret type is ruled (ruling 41); rulings 40 and 41 applied.
- "What this is": add one paragraph: Conduit owns its documents, connection, access,
  accountability and the shape of its contracts; everything else is a provider behind a
  contract that the user picks (R40 1, 2, 31).
- Add a section "What the guarantee for secrets covers", the limit of section 4 of the review,
  as a statement and not a promise.
- Contents table, with these counts: objects 24, schemas 21, instantiation 23, templates 14,
  contracts 10, naming 28, packages 21, manifest 16, submissions 10, pools 15, placement 19,
  conduits 22, graphs 0 (removed by ruling 41, P2), first-start 5, providers 18 (new), secrets
  20 (new), lifecycle 34, access 29, operations 29, revisions 14, observation 5, records 6,
  implicit 5, implementation 38, implementation-defined 14 items; core 102 exports; schemas 41
  in 8 files (`graphs.yaml` gone); `invalid` 52 files, `invalid-by-rule` 20 files. Total 426
  promises.
- Descriptions in the table: templates ("type, implementations, hooks, retirement and
  controls"), placement ("locations, machines, labels and placement by the provider"), manifest
  (no re-export), core files' contents.
- "Existing standards first": drop graphs; name provider contracts.
- Reading order: `providers.yaml` and `secrets.yaml` after `first-start.yaml`; drop
  `graphs.yaml`.

### `docs/OVERVIEW.md`

Its note under the title stays. Rewrite: "Conduits and access" (no policies, no composite, no
retrieval tool, no isolation plugin; secrets read only by the engine); "Pools, ownership and
placement" (machines, labels, the machines provider); "One lifecycle" (hooks, runs, removal);
"Packages, contracts and the core package" (no graph implementation, no re-export,
`package` and `remote` items, a project's address); add a short section "Providers" (31) and
one "Secrets" with the five stores of section 4. Check the worked example for `narrows`,
policies and composite templates.

## 6. Decisions checklist

Every decision line, with the group that writes it and the promises that carry it. "Cites"
names promises in other groups that point to it.

| Line | Writes | Promises that carry it | Cites |
| --- | --- | --- | --- |
| R40 1 (the line) | G2, Docs | PRV-001, PRV-003; `spec/README.md` | — |
| R40 2 (swappable by the user) | G2 | PRV-001 to PRV-017 | SEC-005 |
| R40 3a (runtime provider contract) | G2 | PRV-006 to PRV-012 | TPL-009, TPL-012 |
| R40 3b revised (locations stay; provider reports) | G2 | PLC-002, PLC-008, PLC-023 to PLC-026, PRV-014, PRV-015 | LIF-010 |
| R40 3c (setup graphs to dependencies) | G1 | graphs.yaml emptied; TPL-012, TPL-017, TPL-018 | CON-004, POOL-013 |
| R40 3d (now, before round 5) | Coordinator | this plan | — |
| R40 3e (cryptographic proof across machines) | G5 | IFC-022 | PRV-018 |
| R40 3f (one control plane in v0) | G3 | ACC-036, ACC-037 (rule kept; no other promise assumes one place) | — |
| R41: policies a candidate feature | Coordinator | tracker entry, not spec | — |
| R41: names not final | Vocabulary audit | P1 | — |
| 1 (graphs go; dependency order kept) | G1, G2, G3 | GRA-* removed; TPL-012; CON-004; POOL-013 | OPS-022 |
| 1a (setup hook, retire hook) | G1 | TPL-017, TPL-018 | ACC-018, ACC-039, OPS-028 |
| 1b (configuration and revision in `_meta`) | G5 | IFC-006 | SEC-006, OBJ-021 |
| 1c (a further setup leaves what is done) | G4 | OPS-021, OPS-028 | CON-004, POOL-013, PRV-007 |
| 2 (runtime provider; runtimes go) | G1, G2 | TPL-006, TPL-007, OBS-005, OBS-007 removed; PRV-007 | IFC-006, REC-002 |
| 2a (run named by instance ID) | G2 | PRV-007 to PRV-009; TPL-019 (G1) | OPS-031, IFC-017, DL12 |
| 2b (restart policy is the provider's) | G2 | PRV-010 | OBS-004 |
| 2c (secrets in a run) | G2, G7 | PRV-011; SEC-010, SEC-014 | PRV-007 |
| 2d (enforcement later) | G3 | ACC-027 | — |
| 3a (remove policies) | G3 | ACC-026, ACC-031, ACC-035, ACC-042, ACC-043 removed; ACC-009, ACC-017 | LIF-001, IMP-002, OPS-019, OPS-023, OPS-029, IFC-018, NAM-019, SCH-009 |
| 3b (no outside decision point in v0) | G3 | nothing written | — |
| 4a (one path: engine reads before a run) | G7 | SEC-010 | PRV-007 |
| 4b (no actor's request for `reveal-secret.v1`) | G7 | SEC-007 | ACC-001, ACC-003, IFC-012 |
| 4c (whose read it is) | G7 | SEC-011 | ACC-017, ACC-039 |
| 5 (remove composite Conduit templates) | G3 | CON-003, CON-019, CON-029 removed; CON-001, CON-002, CON-004, CON-016, CON-020 | LIF-009, LIF-029, LIF-033, OBJ-003, OBJ-014, INS-012, INS-016, IFC-012, IFC-014, DL15 |
| 6a (permitted at dispatch, outputs delivered) | G3 | ACC-034 | OPS-017 removed (G4) |
| 6b (`isError`; repeat in progress is conflict) | G4, G5 | OPS-032, OPS-009; IFC-025 | DL3 |
| 6c (remove retrieval; tasks extension) | G4, G5 | OPS-015, OPS-016, IFC-024 removed; IFC-016 | ACC-001, ACC-019, IFC-021, IFC-028, LIF-031 |
| 7 (remove requirement composition) | G6 | CTR-010, CTR-011 removed; CTR-005 | CON-022, CON-028, IFC-018, IFC-019, OBJ-019, OBJ-022, NAM-018 |
| 8 (default location and facts go) | G2 | PLC-003, PLC-004, PLC-016, PLC-021, PLC-022 removed; PLC-008 | POOL-016, OPS-029, FST-001, DL6 |
| 8a (label selector inline) | G2, G6 | PLC-007, PLC-025; CTR-004, MAN-010 | CTR-005 |
| 8b (shared host by hostname label, once) | G2 | PLC-013 | CON-006 |
| 8c (placed by the provider) | G2 | PLC-002, PRV-015, POOL-016 | OBJ-013 |
| 8d (engine registers a location per machine) | G2 | PLC-026, FST-006, PLC-008 | FST-001 |
| 8e (unreachable and gone) | G2 | PLC-023, PLC-024, PRV-014 | LIF-010, LIF-023, OBS-003, DL5 |
| 9 (drop re-exports) | G6 | PKG-013, PKG-014, MAN-008 removed; MAN-007 | ACC-029, CON-022, IFC-018, INS-025, NAM-028 |
| 10 (drop rename) | G6 | OBJ-005, OBJ-026 to OBJ-028 removed; OBJ-011; CON-026 removed (G3) | LIF-022, LIF-034, REV-003, REV-011, ACC-033 |
| 11 (a project's contracts get an address) | G6 | MAN-004, MAN-015, MAN-016, SUB-008, NAM-009, PKG-029 | ACC-029, ACC-038, IFC-006, IFC-018 |
| 12 (install before register) | G6 | PKG-006, PKG-028; INS-022 removed | NAM-012, NAM-028, SUB-008, SUB-010, TPL-004 |
| 13a (no store in `engine`) | G7 | SEC-020 | MAN-012 |
| 13b (a store's own credential is a secret) | G7 | SEC-010 | — |
| 14 (remove `obliges`) | G3 | CON-022, ACC-038 | IFC-034 removed (G5), CTR-005 |
| 15 (controls without clamping) | G4 | REV-007, REV-008 removed, REV-010, REV-013, REV-015 | CTR-006, OPS-031 |
| 16a (Operation contracts as MCP tools) | G6 | CTR-002, CTR-012, OBJ-015 | IFC-011, IFC-033, OPS-007, OPS-026, REV-006 |
| 16b (tool hints) | G5 | IFC-033 | CTR-002 |
| 17 (removal after retirement) | G1 | LIF-035, LIF-007, LIF-017, LIF-024; LIF-030 removed | CON-008, CON-031, ACC-011, ACC-012, PLC-011, OPS-029, REC-003 |
| 18 (Conduit's keys as an MCP extension) | G5 | IFC-008, IFC-029, IFC-037, IFC-006 | — |
| 19 (one Type per store; core capability) | G7 | SEC-002, SEC-005 | PRV-001 |
| 20 (schema profile 2020-12 with exclusions) | G6 | `schema-profile.md`, SCH-008, SCH-009 | MAN-006, OBJ-022 |
| 21 (where packages come from) | G6, G2 | NAM-012, MAN-006, PKG-023; PRV-016, PRV-017 | PKG-024, DL7 |
| 22 (cancellation is the protocol's) | G4 | OPS-027; DL14 removed (G5) | — |
| 23 (a busy instance reports conflict) | G4 | REV-014, REV-012 | LIF-021, DL9 |
| 24a (diagnostics as SARIF) | G5 | IFC-025 | OPS-005 |
| 24b (conditions) | G1 | LIF-011, OBS-002, OBS-003 | PRV-009 |
| 24b, `shutdown` member | G1 | LIF-014, LIF-024 (kept as its own member) | — |
| 24c (records as CloudEvents) | G4 | REC-002 | DL8 |
| 25a (`package` item) | G6 | MAN-009, MAN-010, PKG-019 | PRV-012 |
| 25b (`remote` replaces `url`) | G6 | MAN-009, MAN-010, PKG-020 | IFC-001, IFC-017 |
| 25c (headers of a remote server) | G5, G7 | IFC-041; SEC-013 | MAN-010 |
| 25d (values from the configuration) | G6 | MAN-010, MAN-017 | PRV-007, SEC-010 |
| 26a (identity as a contract) | G5 | IFC-022, DL13 | PRV-018 |
| 26b (MCP Authorization on HTTP) | G5 | not now: when a client connects over HTTP (DL13 names it as one mechanism) | — |
| 26c (engine issues tokens) | G5 | not now, as 26b | — |
| 27 (requests for input: not in v0) | G5 | IFC-010 unchanged | — |
| 28 (bounds; what delegation trusts) | G3 | ACC-030 | — |
| 29 (keep the package digest) | G6 | PKG-004 unchanged | — |
| 30 (keep DL15) | G5 | DL15 | OBJ-003 |
| 31a (what a provider is) | G2 | PRV-001 to PRV-005 | SEC-005 |
| 31b (how the user picks one) | G2 | PRV-006, PRV-013, PRV-017, POOL-009 | OBJ-013, INS-012 |
| C33 (repeat and reconcile state is engine state) | G4 | OPS-008, REC-004 | DL2 |
| C51 (`tools.listChanged`, `cacheScope`) | G5 | IFC-038 | IFC-012 |
| C52 (trace context) | G5 | IFC-039, IFC-040 | — |
| C58 (well-known labels) | G2 | PRV-014 | PLC-026 |
| C14 (`.yml`, `conduit.yml`) | G6 | MAN-001, MAN-002, MAN-011, SUB-005 | SUB-010 |
| C20 (drop `narrows`) | G6, G3 | CTR-005; CON-030 removed, CON-018, CON-022 | IFC-018, MAN-012, ACC-029 |
| C21 (opaque revision) | G4, core | REV-001; `revision.v1` | — |
| C26 (`appliesTo` goes with 3a and 8a) | G6 | CTR-005 (P12) | — |
| S4.1 (marking a secret) | G7 | SEC-001 to SEC-004 | OBJ-021 |
| S4.2 (capability and its one operation) | G7, core | SEC-005; `secret-reveal.v1`, `reveal-secret.v1` | PRV-001 |
| S4.3 (what every store must do) | G7 | SEC-006 | — |
| S4.4 (who may read) | G7 | SEC-007, SEC-008, SEC-009 | ACC-003, ACC-005, ACC-006 |
| S4.5 (how a value arrives) | G7 | SEC-010 to SEC-014 | PRV-007, PRV-011, TPL-012 |
| S4.6 (records) | G7 | SEC-015, SEC-016 | REC-001, REC-002 |
| S4.7 (where the value goes) | G7 | SEC-017, SEC-018 | — |
| S4.8 (what an implementation does with it) | G7 | SEC-019 | IFC-031 |
| S4, the limit of the guarantee | Docs | `spec/README.md` section | SEC head comment |
| S4, what moves into each store's contract | G7, Docs | SEC head comment; `docs/OVERVIEW.md` | — |
| S4, five stores; the file store in the standard library | Coordinator | `packages/` (ruling 40, after the ruling); not spec | — |

No decision line was left without a carrier. Three lines are carried by writing nothing now, as
the review recommends: 3b, 26b and 26c, and 27 keeps IFC-010 as it is.

## Counts

| Group | Removed | Rewritten | Added |
| --- | --- | --- | --- |
| G1 lifecycle | 36 | 35 | 4 |
| G2 placement and providers | 5 | 16 | 22 |
| G3 access | 10 | 32 | 0 |
| G4 operations | 4 | 28 | 3 |
| G5 interface | 2 promises, 1 DL item | 18 promises, 12 DL items | 5 |
| G6 language | 10 | 49 | 5 |
| G7 secrets | 0 | 0 | 20 |
| All | 67 promises, 1 DL item | 178 promises, 12 DL items | 59 |

Promises before: 434. After: 434 − 67 + 59 = 426.
