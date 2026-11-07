# Re-audit round 2 — log of the documents

Written 2026-11-06 by Editor S, under the [decisions of the round](DECISIONS.md) and the
rules of [CONSOLIDATION.md](../CONSOLIDATION.md). Editor S owns `spec/core/`,
`spec/schemas/` with its examples, `spec/promises/manifest.yaml` and
`tools/check_schemas.py`. No other file was edited, except this one, which is new.

Accepted by the owner in [ruling 39](../../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

Finding IDs: `R1-n.m` is section and item of [reader 1](reader-1.md); `R2-Tn`, `R2-Sn`,
`R2-Cn`, `R2-Gn` and `R2-Stn` are the term, shape, core package, guess and standards findings
of [reader 2](reader-2.md); `R3-n` is finding n of [reader 3](reader-3.md). "Known point" is a
row of the third table of DECISIONS.md. Only the findings that concern a file Editor S owns
have a row. A finding that concerns promises alone is Editor P's and has none.

Editor P reworded the promises at the same time. Where a row cites a promise that Editor P
added in this round (OBJ-025, OBJ-026, INS-025, FST-006, LIF-028, LIF-029, PLC-021, SUB-009,
OPS-028, IFC-033), the document was written to DECISIONS.md first and the citation was added
once the promise was on disk.

## Files

| Changed | Added |
| --- | --- |
| `spec/core/README.md`, `capabilities.yaml`, `conduit.yaml`, `operations.yaml`, `references.yaml`, `requirements.yaml`, `results.yaml`, `templates.yaml`, `types.yaml` | — |
| `spec/schemas/README.md`, `contracts.yaml`, `envelope.yaml`, `graphs.yaml`, `instances.yaml`, `manifest.yaml`, `templates.yaml`, `values.yaml` | — |
| `spec/schemas/examples/valid/files-package/conduit.yaml` and `contracts.yaml`, `valid/project/templates.yaml` | — |
| `spec/schemas/examples/invalid/NAM-005-reference-with-a-version.yaml`, `OBJ-015-input-marked-required.yaml`, `SCH-015-ref-to-an-https-address.yaml` (comments only) | `invalid/CTR-006-changes-as-a-list.yaml`, `invalid/INS-012-destination-operations-in-a-composite.yaml`, `invalid/NAM-001-manifest-address-dot-segment.yaml` |
| `spec/schemas/examples/invalid-by-rule/CON-028-bound-applied-twice.yaml` (comment), `SCH-019-input-default-fails-its-schema.yaml` | `invalid-by-rule/CTR-004-pool-type-without-pool-management.yaml`, `invalid-by-rule/CTR-005-composition-applied-where-a-constituent-may-not-be.yaml`, `invalid-by-rule/TPL-006-template-redeclares-a-runtime-of-its-type.yaml` |
| `spec/promises/manifest.yaml` (MAN-001, MAN-006 and the header) | — |
| `tools/check_schemas.py` | — |

## Counts that changed

`spec/README.md` is Editor P's or the coordinator's, and its Contents table still states the
old counts for two rows.

| What | Before | After |
| --- | --- | --- |
| Files under `examples/invalid` | 55 | 58 |
| Files under `examples/invalid-by-rule` | 11 | 14 |
| Exports of the core package | 98 documents and 1 implementation | The same. No export was added or removed |
| Promises in `manifest.yaml` | 14 | 14: 2 changed, none added or removed |
| Rules of step 6 of the tool | 10, from 9 promises | 13, from 12 promises |

## Reader 1

| Finding | Files | What changed, or why not |
| --- | --- | --- |
| R1-1.1 An unbounded right passes the "within" test | `core/operations.yaml`, `core/requirements.yaml`, `core/templates.yaml` | `open.v1`, `close.v1`, the comment on `bound.v1` and the template `root-access` say that a Conduit to `access` with no bound is unlimited, and is opened or closed only by an actor whose own has none |
| R1-1.2 A rename and the revision | `core/operations.yaml`, `core/README.md` | `rename.v1`: the revision advances; its `effect` says so. It gained no output |
| R1-1.3 A Conduit closed without a request | `core/operations.yaml` | `close.v1`: the shutdown is confirmed when the Conduit is closed, however it was closed |
| R1-1.4 Which target authorizes a reconciliation | `core/operations.yaml`, `core/capabilities.yaml`, `core/types.yaml`, `core/templates.yaml`, `core/README.md` | See R3-7 |
| R1-1.5 Retrieving under an unknown identity | `core/operations.yaml`, `core/README.md` | `retrieve-result.v1`: an identity under which the sender has no invocation reports refused |
| R1-1.7 "Names a location" | `schemas/templates.yaml` | The comment now says what PLC-005 rejects: a template that gives a placement, which leaves the `default` of an input whose destination is an entry's `placement`. No rule was added to the tool |
| R1-1.10 `placement` is not required | `schemas/instances.yaml` | Comment only. `instance.v1` already agreed |
| R1-1.11 An input with a default | `schemas/contracts.yaml`, `schemas/templates.yaml`, `schemas/README.md`, `core/requirements.yaml`, `core/README.md`, `examples/invalid/OBJ-015-…`, `examples/valid/files-package/contracts.yaml` | Descriptions and comments say that an input that states a `default` may be left out, marked optional or not. No schema required `optional` beside `default`, and none does |
| R1-2.1 The setup and retire operations | `core/operations.yaml`, `core/README.md`, `schemas/graphs.yaml` | `set-up.v1` and `retire.v1` say that one invocation is the setup or the retirement, that the engine implements it and runs the steps of the template's `setup` or `retire` graph, and that the graph is no implementation and has no inputs. `lifecycle-graph.v1` says the same |
| R1-2.2 Replacing a component | `core/operations.yaml` | `register-replacement.v1` covers a declared Conduit, a component Conduit made for a declared Conduit whose template is a composite, and the Resource of an entry. A component of a composite instantiated by its own document is not covered, and a request that names it reports invalid, as LIF-009 now states. A request while the instance is not retired reports conflict |
| R1-2.3 A bound applied twice | `examples/invalid-by-rule/CON-028-bound-applied-twice.yaml`, `core/requirements.yaml`, `schemas/templates.yaml`, `schemas/README.md` | The example stays: it is one template, and CON-028 now rejects the template when it is registered. Its comment, the comment on `bound.v1` and the README row say so |
| R1-2.4 That the bound is a filter | `core/requirements.yaml` | `narrows: true` stays. The description says the bound is a filter, and the comment cites ACC-029 for it in place of the definitions |
| R1-2.5 Every Pool offers its capability | `tools/check_schemas.py`, `examples/invalid-by-rule/CTR-004-…`, `schemas/contracts.yaml`, `core/capabilities.yaml`, `schemas/README.md` | The profile cannot say that a list holds a value, so the rule `pool-offers` of the tool applies CTR-004, with an example. `type-spec.v1` and `pool-management.v1` describe it |
| R1-2.6 Policies and bounds of unpublished contracts | `core/operations.yaml`, `core/requirements.yaml`, `core/README.md`, `examples/valid/project/templates.yaml` | `apply-policy.v1` and `bound.v1` say that a qualified reference names a contract of a published package only. The schemas did not change |
| R1-2.7 Diagnostics on any reason | `core/README.md`, `core/operations.yaml` | "that ends with the reason invalid or conflict", and that a refusal says nothing else (OPS-023) |
| R1-2.8 `localPaths` cited from the definitions | `core/README.md`, `core/types.yaml` | "the two facts the core package's schemas name" |
| R1-2.9 The count of exports | `core/README.md`, `tools/check_schemas.py` | "98 documents, … and one implementation, which is also an export". The tool's line for step 5 says the same |
| R1-3.2 "Never set up" | `core/operations.yaml` | `retire.v1` says "a setup of the instance was ever dispatched" |
| R1-3.3 The shape of a nested instantiation | `schemas/instances.yaml`, `schemas/templates.yaml`, `schemas/README.md`, `core/README.md` | Every citation of POOL-012 for an entry's shape, and of CON-020 for a declared Conduit or a component, now cites OBJ-014, which holds whether or not the other two are merged |
| R1-4.4 Two small leftovers | `examples/invalid/SCH-015-ref-to-an-https-address.yaml`, `promises/manifest.yaml` | The comment names the third form of a `$ref`. MAN-001 says "one YAML document" |
| R1-4.5 "The owner" as a person | — | Not applied. The reader says no fix is needed |

## Reader 2

| Finding | Files | What changed, or why not |
| --- | --- | --- |
| R2-T1 "Setup operation" and "retire operation" | as R1-2.1 | As R1-2.1 |
| R2-T2 What a project is | `core/operations.yaml`, `promises/manifest.yaml` | `register.v1`: the `documents` and the `manifest` of one request are one project. MAN-001: a project is the documents and the one manifest handed over together, and the input `manifest` of `register.v1` is the manifest |
| R2-T5 "Manifest item" | `promises/manifest.yaml`, `core/README.md` | MAN-006: "An item of `packages` in a manifest gives `fetch`". The README says "an item of `packages`" |
| R2-T8 "Plain" | `core/types.yaml`, `core/README.md`, `schemas/contracts.yaml` | "the basic Interface type", "the basic Pool type"; an applied requirement's `arguments` are "literal values". See the choices below |
| R2-T10 Three words for what comes back | `core/README.md` | "result" in place of "answer" as a noun, in two sentences |
| R2-T13 Reserved words in an ordinary sense | `schemas/README.md`, `tools/check_schemas.py`, `promises/manifest.yaml`, `core/operations.yaml`, `core/README.md`, `schemas/contracts.yaml` | A file is "rejected when it is read", not refused, in the README and in the tool's output; the header of `manifest.yaml` says "rejecting members"; MAN-001 says "YAML document"; `settings` is "a map keyed by" pointers |
| R2-T14 `capabilities.yaml` widens "right to open" | `core/capabilities.yaml` | "An open Conduit to `access` that lists `open.v1` is a right to open" |
| R2-S1 The value of `from` | `schemas/instances.yaml`, `schemas/values.yaml`, `schemas/README.md`, `core/README.md`, `examples/valid/project/templates.yaml`, `tools/check_schemas.py` | `template-reference.v1` states the four forms and no longer refers to `selection.v1`. One component of the example composite writes `from: '#directory-settings'`. The tool checks a `from` written as a qualified reference or as `#name` for the kind of template it names |
| R2-S2 Keys the schemas make optional | `schemas/contracts.yaml`, `schemas/README.md` | `type-spec.v1` says which keys may be left out and what that means, and that a type with no `schema` puts no rule on the configuration, as CTR-004 now states. `narrows` and `obliges` say what leaving them out means. The README moves the shape of a Requirement's input to the choices a promise states (CTR-005) |
| R2-S3 Runtimes of a type | `schemas/templates.yaml`, `schemas/contracts.yaml`, `schemas/graphs.yaml`, `examples/valid/project/templates.yaml`, `examples/invalid-by-rule/TPL-006-…`, `tools/check_schemas.py`, `schemas/README.md` | The example template no longer redeclares `watcher`: it declares `path-watcher`, and its steps name both. The rule `runtime-once` of the tool applies TPL-006, with an example |
| R2-S4 A left-out `arguments` map | `schemas/instances.yaml`, `schemas/contracts.yaml`, `schemas/templates.yaml`, `schemas/README.md`, `examples/invalid-by-rule/SCH-019-input-default-fails-its-schema.yaml` | Descriptions say the map is present and empty. The third document of the example gained `arguments`, as the reader asked |
| R2-S5 `/operations` in a composite | `schemas/templates.yaml`, `examples/invalid/INS-012-destination-operations-in-a-composite.yaml`, `schemas/README.md` | `conduit-template-spec.v1` carries two patterns: `/operations` and a requirement's `arguments` for a template that is not composed, a component's `arguments` for a composite |
| R2-S6 The recorded packages | `core/operations.yaml`, `core/README.md`, `schemas/envelope.yaml` | `inspect.v1` returns `packages` as a list. An item is one of three shapes: `address`, `version` and `digest`; the core address and `version`; `digest` alone |
| R2-S7 The keys of `_meta` | `core/README.md` | The list of a request's keys no longer has `package`, and says how a contract of an unpublished package is written. A result's `invocation` is named. The two gaps the reader lists are in IFC-029 and IFC-023, which are Editor P's |
| R2-S8 What the README says of schema and engine | `schemas/README.md`, `schemas/manifest.yaml` | Rows added for MAN-006, CTR-003, PKG-017 and OBJ-024; the row on uniqueness says that `uniqueItems` rejects an export written twice; the first row names the members OBJ-022 also covers; the comment in `manifest.yaml` says that an item that is not plain gives no `tools` |
| R2-S9 `.` and `..` in an address | `schemas/values.yaml`, `examples/invalid/NAM-001-manifest-address-dot-segment.yaml` | `address.v1` and `qualified-reference.v1`: a path segment starts with a letter or a digit |
| R2-S10 NAM-005 and its example | `examples/invalid/NAM-005-reference-with-a-version.yaml` | The file keeps its name, because NAM-005 now names the form it rejects, `@`. Its comment says that `@` is not a character of an address |
| R2-S11 Smaller points | `schemas/README.md` | The README says that the helper schemas that carry a destination's pattern, and `object.v1` and `diagnostic.v1`, are open by design |
| R2-C1 Every Pool offers `pool-management.v1` | as R1-2.5 | As R1-2.5 |
| R2-C2 `register-replacement.v1` and a component | as R1-2.2 | As R1-2.2 |
| R2-C3 A fact the definitions do not name | as R1-2.8 | As R1-2.8 |
| R2-C4 A bound and a policy of an unpublished package | as R1-2.6 | As R1-2.6 |
| R2-C5 `register-replacement.v1` declares no `diagnostics` | `core/operations.yaml`, `core/README.md` | The output is declared and the README lists it |
| R2-C6 The naming table | `core/README.md` | "and the two Conduit templates, which are named for their consumer and their target" |
| R2-G1 The form of a resolved reference | `core/references.yaml`, `schemas/README.md`, `core/README.md`, `core/operations.yaml`, `core/types.yaml`, `schemas/instances.yaml`, `schemas/values.yaml` | Each of the 11 reference schemas accepts `{kind, id}` as its last branch, with the kinds it accepts. No schema describes a resolved instance, so the README of the schemas describes one, with a fragment |
| R2-G2 Which input sets which setting | `schemas/contracts.yaml`, `core/operations.yaml`, `examples/valid/files-package/contracts.yaml`, `examples/invalid/CTR-006-changes-as-a-list.yaml`, `tools/check_schemas.py`, both READMEs | `changes` is a map from the name of an input to a pointer. `set-default-location.v1`: `{location: /defaultLocation}`. `set-facts.v1`: `{facts: /facts}`. `set-read-limit.v1`: `{limit: /readLimit}`. The tool checks that each key is an input the control declares and each pointer a place the type's schema describes |
| R2-G3 A composed requirement | `schemas/contracts.yaml`, `schemas/manifest.yaml`, `examples/valid/files-package/conduit.yaml` and `contracts.yaml`, `examples/invalid-by-rule/CTR-005-…`, `tools/check_schemas.py`, `schemas/README.md` | Descriptions say when an implementation supports a composition, how it is sent, that it narrows if a constituent does and where it may be applied. The rule `applies-to` of the tool applies CTR-005, with an example |
| R2-G4 Replacing a template that uses the project's contracts | `core/operations.yaml` | `replace.v1` says what `#name` reaches in the replacing document, as NAM-022 now states. Its inputs did not change |
| R2-G5 The inspection result | `core/operations.yaml`, `core/README.md` | Applied in part. `packages`, the resolved form under `declared`, and the implementation named under each requirement of a Conduit are stated. The README lists, with the promises, everything about the result that is still not fixed |
| R2-G6 `optional: true` on a runtime and on a graph's output | — | Not applied. No decision, and no promise gives either mark an effect. The schemas still accept both |
| R2-G8 What `actors` and `targets` of `apply-policy.v1` each do | — | Not applied. No decision. `apply-policy.v1` is unchanged on this point |
| R2-G9 The owner of a location's document | `core/README.md` | The README says that a location's document writes `owner: system` (PLC-008). The examples already did |

## Reader 3

| Finding | Files | What changed, or why not |
| --- | --- | --- |
| R3-1 Filters skip graph steps | `schemas/graphs.yaml` | `step.v1` says that a step without `conduit` carries the restrictions of the invocation its graph is serving |
| R3-3 Retained results scoped to the caller alone | `core/operations.yaml`, `core/README.md` | `retrieve-result.v1`: scoped to the caller, the target and the instance that sent the request |
| R3-4 No test for "access at that moment" | `core/operations.yaml`, `core/README.md` | `retrieve-result.v1`: content is delivered only while the Conduit the invocation was dispatched through is open, and never under another one |
| R3-5 Nested setup steps use the caller's rights | `core/operations.yaml` | `set-up.v1`: a step that opens a declared Conduit or sets up an entry's Resource is authorized exactly as the request was, by the Conduits of the instance that sent it |
| R3-7 Reconciling a management operation | `core/capabilities.yaml`, `core/types.yaml`, `core/templates.yaml`, `core/operations.yaml`, `core/README.md`, `examples/valid/files-package/contracts.yaml` | `access-control.v1`, the type `access.v1` and the template `root-access` include `reconcile.v1`. So do `location-settings.v1` and `location.v1`: see "Left in disagreement or open", point 1. `reconcile.v1` says what recording that the effect occurred, or did not, does for a setup, an open, a retirement, a close and a control |
| R3-8 The first-start objects are named, not identified | `core/operations.yaml`, `core/templates.yaml`, `core/README.md` | `rename.v1`: a first-start object is not renamed, and the request reports refused (LIF-022). The comment on the templates and the README say that no second instance is made from `system`, `management`, `access` and `root` (FST-006) |
| R3-10 A composed requirement | as R2-G3 | As R2-G3 |
| R3-11 The rules on files have no place on the wire | `core/operations.yaml`, `promises/manifest.yaml` | Applied in part: one request is one project, as R2-T2. The base of a relative `fetch` and of a relative `command` for a registration that hands over values has no decision, and MAN-006 and MAN-010 are unchanged |
| R3-15 `install.v1`: three cases and dependencies | — | Not applied. The reader asks for one promise each, and none exists yet. `install.v1` is unchanged |
| R3-16 `replace.v1` and the project's own contracts | as R2-G4 | As R2-G4 |
| R3-17 A Conduit closed because an end was retired | as R1-1.3 | As R1-1.3 |
| R3-18 Does a rename advance the revision | as R1-1.2 | As R1-1.2 |
| R3-19 Reconciling a control after another change | `core/operations.yaml`, `core/README.md` | `reconcile.v1`: the stated settings are stored in the instance's current configuration |
| R3-22 A replacement for a component Conduit | `core/operations.yaml` | `register-replacement.v1`: the new instance has a newly generated name, and the retired one keeps its own, as LIF-009 now states |
| R3-23 A consumer is never told the engine's ID | `core/operations.yaml`, `core/README.md` | `reconcile.v1` and the README say that a consumer learns the ID from `io.github.shrav-k.conduit/invocation` in its result (IFC-025) |
| R3-26 Generated names in the result of `register.v1` | `core/results.yaml`, `core/operations.yaml` | `object.v1` gained the optional member `nested`. `register.v1` says that `objects` lists every template and instance added, nested instances among them. See the choices below |
| R3-27 "Within" and a Conduit with no bound | as R1-1.1 | As R1-1.1 |
| R3-31 A runtime declared by a Type and a template | as R2-S3 | As R2-S3. What "required" means for a runtime is R2-G6, not applied |
| R3-32 `retire-with-resources.v1` and `retire.v1` | — | Not applied. No decision, and LIF-021 is unchanged. The contract is unchanged |
| R3-33 `apply-policy.v1` with neither `actors` nor `targets` | — | Not applied. No decision |
| R3-34 The settings a control addressed | as R2-G2 | As R2-G2. What a `settings` holds for a key outside `changes` is not stated anywhere, and was left |
| R3-35 An ID against a reference schema | as R2-G1 | As R2-G1: the stored form satisfies the reference schema, so REV-008 and REV-013 test what was checked before |
| R3-36 A bound's `operations` and a re-export | `core/requirements.yaml` | Applied in part. `bound.v1` says that it names operations of published packages only. Whether a re-exported operation matches by the string written or by the export it resolves to has no promise and was left |
| R3-37 The default location once it is unregistered | `core/types.yaml` | `system-pool.v1` says the control "sets" the setting to a location that exists or to nothing, which is what PLC-016 now says |
| R3-40 Values that had to be guessed | — | No change needed in the documents: `operationIdentity` is already a string that is not empty in `retrieve-result.v1`. The rest is IFC-023 and IFC-025 |

## Known points

| Point | Files | What changed |
| --- | --- | --- |
| One `package` key cannot name two unpublished packages | `core/README.md` | The list of a request's keys has no `package`. A contract of an unpublished package is written `<digest>#<name>` |
| The reason for retrieving a result once the target is unregistered | `core/operations.yaml`, `core/README.md` | Refused |
| An object's default against its properties' defaults | `tools/check_schemas.py`, `examples/valid/files-package/contracts.yaml`, `schemas/README.md` | The rule for SCH-019 fills in the defaults of an object's own properties before it validates the object's default. The type `local-directory.v1` has a property that shows it |
| What `obliges` requires | `schemas/contracts.yaml`, `core/requirements.yaml`, `core/conduit.yaml`, `core/README.md`, `core/operations.yaml` | `obliges` names the ends whose implementations must state the requirement in `supports`, and with none the target's must. `bound.v1` states none, and `engine`, the implementation behind `access`, supports it |
| The `settings` input of a reconciled control | `core/operations.yaml`, `core/README.md` | The same shape as the control's output `settings` |

## Choices made where a reader gave options

| Finding | Options | Choice | Why |
| --- | --- | --- | --- |
| R1-2.3 | Reject the template, or refuse only the instantiation and give the example an instance document | The example stays one template | DECISIONS.md chose the first |
| R1-2.4 | A sentence in ACC-029, or `narrows: false` and corrected comments | `narrows: true` stays | Editor P took the first, and the documents then needed no change of behavior |
| R1-2.8, R2-C3 | Add `localPaths` to the definitions, or reword the two sentences | Reworded the two sentences | It is right whichever the definitions do |
| R2-T8 | "literal values" or "values as written"; "basic" or "untyped-configuration" | "literal values" and "basic" | The first option of each. `spec/definitions.md` and `spec/README.md` still said "plain" when this pass ended, and whoever edits them should use the same two words or change these |
| R2-S1 | The pattern of a selection's `#name`, which allows dots, or a name with none | `#` and then a name as `name.v1` gives one, with no dot | A template's name holds no dot (OBJ-001, OBJ-003), so a `#name` with a dot could never resolve to a template |
| R2-S3 | Drop the template's runtime from the valid example, or give it another name | Another name, `path-watcher`, with the type's own pair of operations | A valid example of a runtime that a template declares is kept, beside the one of a type |
| R2-S10 | Rename the example for NAM-001, or keep it for NAM-005 | Kept for NAM-005 | NAM-005 as reworded rejects exactly this reference |
| R2-G1 | A valid example of a resolved instance, or a description | A description, with a fragment, in the README of the schemas | No schema for a resolved instance exists: `inspect.v1` leaves `declared` open |
| R2-G2 | Whether every input other than `revision` must be a key of `changes` | Not required | An input may set no setting. The tool checks only that each key is an input the control declares |
| R2-G3 | A check that a composition states the `narrows` its constituents give | No such check | CTR-005 now says a composition narrows "whatever its own `narrows` states". The tool applies only what CTR-005 rejects: applying a requirement, or a composition, where an `appliesTo` does not list |
| R2-S6 | One open item shape, or one shape for each of the three cases | `anyOf` of three closed shapes | The profile has no `if`, and three shapes say which members go together |
| R3-26 | The form of the member that names the nested instantiation | `nested: {instance, path}` in `object.v1`. `instance` is the ID of the instance whose template holds the nested instantiation and is absent for a component of a composite instantiated by its own document. `path` is the name of the declared Conduit or of the entry, then the name of each component, outermost first | The same order as a tool name under IFC-012. No promise names the member yet |
| R3-11 | An entry point that takes files, or `register.v1` given the files' bytes | Neither | Both change behavior, and there is no decision |

## Left in disagreement or open

1. **Reconciling a control or an ordinary operation.** ACC-033 as Editor P wrote it says that
   reconciling is authorized against the target that authorized the request being reconciled,
   and that "a control, like any other operation requested of an instance, is reconciled
   through a Conduit to that instance". A Conduit carries one capability, so the instance's
   type has to offer a capability that includes `reconcile.v1`. The documents follow that:
   `location-settings.v1` and the type `location.v1` now include it, a control of `system` is
   reconciled through a Conduit that carries `pool-management.v1`, and the example capability
   `directory-access.v1` and its type list
   `github.com/shrav-k/conduit/core#reconcile.v1`. No promise says that a type whose
   capabilities leave `reconcile.v1` out has instances whose unknown outcomes can never be
   reconciled. Before this round such an invocation was reconciled through the Pool or
   `management`. DECISIONS.md, row R3-7, is titled for management operations only.
2. **R1-3.1, "any operation whose contract says so may arrange later work".** No key of
   `operation-spec.v1` says so, and none was added: the decision names none. If a mark is
   meant, as `startsModelWork` is one, it needs a name.
3. **R3-8.** The documents state that a first-start object is not renamed and that no second
   instance is made from four of the templates. LIF-022 and FST-006 state both. Nothing says
   what a request reports that renames one of `root`'s two Conduits, which FST-002 lists and
   LIF-022 does not.
4. **The words of R2-T8.** See the choices above.
5. **`spec/README.md`.** Its Contents table states 55 and 11 example files; there are 58 and
   14. It also says "the plain Interface and Pool types".
6. **A promise for `nested`.** `object.v1` is a choice of the schemas, as it was before. No
   promise names the member, which is described only in `core/results.yaml`.
7. **R2-G6, R2-G8, R3-15, R3-32, R3-33, R3-36.** Not applied, each for the reason in its row.

## Checks

Run from the repository root on 2026-11-06, after the last edit.

```text
$ python3 tools/check_schemas.py        (last lines)
4. 58 invalid documents in 58 files were each rejected by a schema, or rejected when read.
5. Core package: 98 documents exported, 0 documents deliberately private and 1 implementation, which is also an export. Checked 431 references for the kind they name; 1 name packages that are not in this repository and were not checked. Checked 3 controls, the 5 operations that take no `revision`, the tables of the two READMEs, and 752 citations of promises in 99 files. The example packages keep 0 documents private.
6. Applied 13 rules of this tool, which come from 12 promises, to the core package and 3 example directories. 14 sets of documents in 14 files under examples/invalid-by-rule/ were each rejected by the rule they are named for; each of their 22 documents is valid against the schemas.
   SCH-019: validated 10 defaults of the core package and the valid examples against their schemas; 0 were not validated, because a `$ref` in the schema leads outside this repository.
Errors: 0
```

The count of citations depends on the promise files, which Editor P was still editing, so it
may differ by the time this is read. Each rule of the tool that was added or changed was also
tried against a copy of the repository in which one document was broken on purpose: a key of
`changes` that is no input, a pointer the type's schema does not describe, `changes` written
as a list, a runtime declared twice, a `from` that names nothing or a template of another
kind, a Pool type that offers nothing, a reference that writes both `id` and `name`, and an
object default whose member fails. The tool reported each one and reported none once the
document was restored.
