# Round 2: log of the fix pass on the promises

Written by Editor P, 2026-11-06. It covers every YAML file under `spec/promises/` except
`manifest.yaml`, `spec/conventions/`, `spec/interface/`, `spec/implementation-defined.yaml`,
`spec/schema-profile.md`, `spec/definitions.md` and `spec/README.md`. The instructions were
[the decisions](DECISIONS.md) and the findings of [reader 1](reader-1.md),
[reader 2](reader-2.md) and [reader 3](reader-3.md). The pass on `spec/core/`, `spec/schemas/`,
`spec/promises/manifest.yaml` and the checker is Editor S's.

Accepted by the owner in [ruling 39](../../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

## Result

- 406 promises, 11 more than before. No promise was removed and no ID was reused.
- Added: OBJ-025, OBJ-026, INS-025, SUB-009, PLC-021, FST-006, LIF-028, LIF-029, OPS-028,
  OBS-007 and IFC-033.
- LIF-026 changed level from `may` to `must`. No promise changed layer.
- The implementation-defined list has 15 items: DL15 was added, and the key that says what an
  item is about is now `subject`.
- Per file: objects 26, schemas 21, instantiation 24, templates 12, contracts 10, naming 27,
  packages 19, submissions 9, pools 14, placement 19, conduits 25, graphs 25, first-start 5,
  lifecycle 29, access 31, operations 28, revisions 13, observation 7, records 6, implicit 9,
  implementation interface 33. `manifest.yaml`, which this pass did not touch, has 14.

## Reader 1

| Finding | Promises changed or added | What changed, or why it was not applied |
| --- | --- | --- |
| R1-1.1 | ACC-029, ACC-030, ACC-022 | A Conduit to `access` with no bound is unlimited; it is within another only if that other has none. |
| R1-1.2 | OBJ-026 (new), OBJ-011, REV-003, REV-004, REV-012 | A rename advances the revision. `rename.v1` renames a template or an instance. |
| R1-1.3 | LIF-024, LIF-018, CON-008 | A Conduit's shutdown is confirmed when it is closed, however it was closed. |
| R1-1.4 | ACC-033, OPS-014, LIF-018 | Reconciling is authorized against the target that authorized the request being reconciled. |
| R1-1.5 | OPS-015, OPS-016, IFC-024 | OPS-015 is now the rule for an identity under which the sender has no invocation: refused. |
| R1-1.6 | NAM-007 | Excepts the core package's address. |
| R1-1.7 | PLC-005, POOL-009 | PLC-005 covers placement only: an entry's `placement`, or the default of an input whose destination is one. |
| R1-1.8 | CON-004, POOL-013, GRA-003 | A step for something already done is left as succeeded. |
| R1-1.9 | LIF-023 | Adds the Resources retired with their Pool under LIF-021. |
| R1-1.10 | OBJ-013 | `owner`, `consumer` and `target` are required; `placement` may be left out. |
| R1-1.11 | OBJ-015, CTR-002, NAM-019, OPS-026 | An input that states a `default` may be left out, for all three kinds of input. |
| R1-2.1 | GRA-014, GRA-003, GRA-020, GRA-022, GRA-027, TPL-003, TPL-008, TPL-009, TPL-012, IMP-005, IMP-009, OPS-022, LIF-003, LIF-005, LIF-018, LIF-024, ACC-033, OPS-014 | One invocation of `set-up.v1` or `retire.v1`, implemented by the engine, runs the steps of the template's `setup` or `retire` graph. The graphs implement no operation. |
| R1-2.2 | LIF-009, LIF-029 (new), POOL-003 | A component of a composite instantiated by its own document cannot be replaced. A replacement asked for while the instance is not retired reports conflict. |
| R1-2.3 | CON-028 | A template that applies `bound.v1` twice is rejected when registered. |
| R1-2.4 | ACC-029 | The bound is a filter, and so a restriction. |
| R1-2.5 | CTR-004, OPS-019 | A Type of the family Pool that does not offer `pool-management.v1` is rejected. |
| R1-2.6 | ACC-029, ACC-031, IFC-018 | A policy and a bound name published contracts only. |
| R1-2.7 | none | The wording to change is in `spec/core/`. OPS-023 is unchanged. |
| R1-2.8 | definitions | The row for location fact names `localPaths`. |
| R1-2.9 | README | "98 documents exported and 1 implementation, which is also an export". |
| R1-3.1 | ACC-019, IMP-006 | ACC-019 keeps the access half and IMP-006 the model-work half. |
| R1-3.2 | TPL-009, LIF-024 | "For which no setup was ever dispatched" in all three; LIF-025 had it. |
| R1-3.3 | OBJ-014, POOL-012, POOL-009, CON-020 | POOL-012 leaves the shape of an entry to OBJ-014 and keeps its ID. OBJ-014 no longer restates placement. CON-020 covers the Conduit template only. |
| R1-3.4 | LIF-006, LIF-008 | The sentence is removed from LIF-006, and LIF-008 cites LIF-015. |
| R1-4.1 | NAM-019 | "Applies a requirement to a Conduit or to placement". |
| R1-4.2 | CTR-002, CTR-003, CTR-004, CTR-005, CTR-006, CTR-007 | "When its package is installed or its project is submitted". |
| R1-4.3 | implementation-defined.yaml, definitions | The key is `subject`; "every item"; "item DL2". |
| R1-4.4 | none | Both leftovers are in Editor S's files. |
| R1-4.5 | none | The reader says no change is needed. |
| R1-5.1 | OBJ-011, OBJ-026 (new), SUB-007, SUB-009 (new) | The operations moved to two new promises of the layer `convention`. |
| R1-5.2 | OBJ-002, OBJ-025 (new), OBJ-023 | OBJ-002 keeps the rejection; OBJ-025 gives the reason conflict. |
| R1-5.3 | PLC-008, PLC-021 (new) | How a location's facts are changed is PLC-021, of the layer `convention`. |
| R1-6.1 | OBJ-001 | Cites CTR-001. |
| R1-6.2 | TPL-004 | Cites NAM-022 beside NAM-014. |
| R1-6.3 | CON-028 | "As ACC-029 states" is dropped. |
| R1-6.4 | definitions | "Set up" cites TPL-008, GRA-022, CON-004 and POOL-013, since GRA-014 no longer speaks of setup; "right to open" cites ACC-006 for what it is. |

## Reader 2

| Finding | Promises changed or added | What changed, or why it was not applied |
| --- | --- | --- |
| R2-T1 | as R1-2.1; definitions | New row "setup operation, retire operation"; the rows for set up, retire, setup graph, retire graph and graph follow it. |
| R2-T2 | SUB-005, SUB-008; definitions | A project is the documents and the one manifest handed over in one registration. |
| R2-T3 | GRA-025, GRA-026, TPL-004, PLC-020; definitions | A value binding yields a value; "resolve" is kept for documents and references. |
| R2-T4 | definitions | A step ends in the same seven ways and is dispatched in the same sense. |
| R2-T5 | PKG-003, PKG-020, PKG-021, NAM-012, NAM-018, NAM-020; definitions | "Item of `packages`" and "implementation item". |
| R2-T6 | NAM-003 | "Local reference"; "an object that exists". |
| R2-T7 | LIF-013; definitions; implementation-defined.yaml | "Records"; "item DL2"; the key `subject`. |
| R2-T8 | definitions, README | Arguments are "the values themselves"; `interface.v1` and `pool.v1` are the basic types. |
| R2-T9 | CON-024, ACC-015, ACC-019, IMP-006, SCH-009, IFC-016; definitions; implementation-defined.yaml | Rows for replace and replacement, rename, management verb, later work and `supports`. "Combiner", "editing mode" and the "reference" of IFC-016 are reworded. |
| R2-T10 | OBJ-023, PKG-023, ACC-008, ACC-014, ACC-015, LIF-020, OPS-005, OPS-006, OPS-009, OPS-012, OPS-023, IFC-010, IFC-019, IFC-025; definitions | "Result" throughout, with a row; "answer" as a verb only. |
| R2-T11 | TPL-004, NAM-023, SCH-014 | "The package that holds the contract"; "a schema of an existing server". |
| R2-T12 | definitions | The rows for optional, record, `source` and hold. |
| R2-T13 | ACC-019, IMP-006, PLC-020, INS-018, SCH-008, SCH-009, SCH-010, SUB-008, REV-007, CTR-006; definitions, README, schema profile | Applied in part. Not applied: the headings "Permitted schema keywords" and "Enforced formats", which `tools/check_schemas.py` reads by name, and MAN-001, which is Editor S's. The key `state` keeps both uses, and the definitions now say so. |
| R2-T14 | none | The sentence is in `spec/core/capabilities.yaml`. |
| R2-T15 | NAM-019 | As R1-4.1. |
| R2-T16 | definitions | The rows for manifest, core package, input and nested instance. |
| R2-S1 | INS-025 (new), INS-002, OBJ-013, OBJ-014, PKG-013, NAM-018, NAM-022 | The four forms of `from`. `from` is not a selection. |
| R2-S2 | CTR-004, CTR-005 | "May hold", the required keys, what each left-out key means, and the shape of a Requirement's input. |
| R2-S3 | TPL-006, GRA-017 | An instance has its type's runtimes as well as its template's; a name declared by both is rejected. |
| R2-S4 | INS-011, OBJ-014, NAM-019 | A left-out `arguments` map is present and empty. |
| R2-S5 | INS-012 | `operations` only in a template that is not composed; a component's `arguments` only in a composite. |
| R2-S6 | OBJ-004, NAM-009, PKG-005, LIF-013 | The key `packages` and the shape of its items. |
| R2-S7 | IFC-029, IFC-023 | A result and a consumer's request carry no interface version; an operation identity is a non-empty string. |
| R2-S8 | PKG-020 | An item that is not plain states no `tools`. The rest is in `spec/schemas/README.md`. |
| R2-S9 | NAM-001 | A path segment starts with a letter or a digit. |
| R2-S10 | NAM-005 | Names the form it rejects and says why no part of a reference selects a release. |
| R2-S11 | OBJ-004 | The rule is for top-level keys. The second point is in `spec/schemas/README.md`. |
| R2-C1 | CTR-004, OPS-019 | As R1-2.5. |
| R2-C2 | LIF-009, LIF-029 (new) | As R1-2.2. |
| R2-C3 | definitions | As R1-2.8. |
| R2-C4 | ACC-029, ACC-031 | As R1-2.6. |
| R2-C5 | none | The output is in `spec/core/operations.yaml`. |
| R2-C6 | none | The table is in `spec/core/README.md`. |
| R2-G1 | OBJ-009, OBJ-010, OBJ-021, INS-014, POOL-006, POOL-014, PLC-001, PLC-016, LIF-013, REV-007, IFC-018 | The resolved form `{kind, id}`, which only the engine writes. |
| R2-G2 | CTR-006, OBJ-016, OBJ-022, REV-006, REV-007, REV-009 | `changes` is a map from an input's name to the pointer it sets. |
| R2-G3 | CON-022, CTR-005, CTR-010, PKG-019, IFC-018, IFC-019, IFC-025 | What `supports` must state, one item for each constituent, and where a composition narrows and may be applied. |
| R2-G4 | NAM-022 | See the choices below. |
| R2-G5 | none | Not applied. The shape of an inspection result needs decisions, and `spec/core/README.md` lists it as not settled. |
| R2-G6 | none | Not applied. What `optional: true` means on a runtime and on a graph's output needs a decision. |
| R2-G7 | IFC-033 (new) | Applied in part: the definition of a presented tool. Two operations of one export name in one implementation needs a decision. |
| R2-G8 | none | Not applied. What `actors` and `targets` each do needs a decision. |
| R2-G9 | PLC-008 | A location's document writes `owner: system`. |
| R2-G10 | OBJ-003; implementation-defined.yaml | DL15, generated names. |
| R2-G11 | IFC-016 | "A value that names the work, as the operation's contract declares it". |
| R2-St1 | none | Not applied. The sections and the error code of the Model Context Protocol revision could not be checked from this repository. |
| R2-St2 | none | Not applied. Which tar format an engine must read needs a decision. |
| R2-St3 | SCH-016; schema profile | The edition JSON Schema 2020-12 cites, matched with the `u` flag. |
| R2-St4 | SCH-001; schema profile | The format-assertion vocabulary; the `uuid` clause. |
| R2-St5 | none | Not applied. The reader gives no fix and says no result changes. |

## Reader 3

| Finding | Promises changed or added | What changed, or why it was not applied |
| --- | --- | --- |
| R3-1 | ACC-018, IFC-006, IFC-018, IFC-019 | A step without `conduit` carries the restrictions of the invocation its graph is serving. |
| R3-2 | OPS-028 (new), OPS-021, IFC-006 | The engine gives every step an operation identity and sends it; a new attempt carries the same one. |
| R3-3 | OPS-008, OPS-009, OPS-010, OPS-015, OPS-016, OPS-018, OPS-025, IFC-020, IFC-024, ACC-001 | An identity and a retrieval are scoped to the caller, the target and the instance that sent the request. |
| R3-4 | OPS-017, ACC-034, IFC-024 | A retained result belongs to the Conduit its invocation was dispatched through. |
| R3-5 | ACC-005, ACC-006, ACC-033, POOL-015 | The steps of a setup are authorized by the Conduits of the instance that sent the request. |
| R3-6 | ACC-034 | The rule holds for every invocation that serves the closed one, directly or through others. |
| R3-7 | ACC-033, OPS-014, LIF-018, LIF-003, LIF-028 (new), REV-010 | The authorizing target, and what reconciling a setup, an open, a retirement and a close changes. |
| R3-8 | FST-001, FST-006 (new), LIF-022 | Identified by ID; a rename is refused; no second instance of the four templates. |
| R3-9 | none | Not applied. Whether ACC-035 covers a policy on `access` or `management`, and what is exempt, needs a decision. |
| R3-10 | as R2-G3 | A composition is expanded into its constituents. |
| R3-11 | none | Not applied. Where files become documents needs a decision and touches `register.v1`. |
| R3-12 | IFC-033 (new), IFC-012 | The tool definition is derived from the Operation contract as the target holds it. |
| R3-13 | none | Not applied. Each of the four points needs a decision. |
| R3-14 | ACC-019, IMP-006 | See the choices below. |
| R3-15 | none | Not applied. Each of the four endings of `install.v1` needs a decision. |
| R3-16 | NAM-022 | See the choices below. |
| R3-17 | as R1-1.3 | |
| R3-18 | as R1-1.2 | |
| R3-19 | REV-010 | "The current configuration". |
| R3-20 | GRA-026 | See the choices below. |
| R3-21 | CON-024 | See the choices below. |
| R3-22 | LIF-009 | The name of a replacement, and that only a component of a declared Conduit can be replaced. |
| R3-23 | IFC-025 | A result's `_meta` holds `invocation`. |
| R3-24 | LIF-010; implementation-defined.yaml | Added to DL5. |
| R3-25 | FST-001; implementation-defined.yaml | Added to DL6. |
| R3-26 | OBJ-003; implementation-defined.yaml | DL15. The member of `objects` is in `spec/core/`. |
| R3-27 | as R1-1.1 | |
| R3-28 | none | Not applied. The reason for a control its template leaves out needs a decision. |
| R3-29 | OPS-015 | Applied in part: an identity that names no invocation reports refused. A tool name never presented, and `open.v1` on an open Conduit, need decisions. |
| R3-30 | none | Not applied. Whether a missing required output is unknown needs a decision. |
| R3-31 | TPL-006, GRA-017 | Applied in part, as R2-S3. What "required" means for a runtime needs a decision. |
| R3-32 | none | Not applied. Needs a decision. |
| R3-33 | none | Not applied. Needs a decision. |
| R3-34 | CTR-006, REV-009 | Applied in part, as R2-G2. A key of `settings` outside `changes` needs a decision. |
| R3-35 | as R2-G1 | |
| R3-36 | ACC-029 | Applied in part, as R1-2.6. How a re-exported operation matches needs a decision. |
| R3-37 | PLC-016 | The control sets the setting to a location that exists or to nothing; afterwards LIF-019 governs. |
| R3-38 | none | Not applied. Needs a decision. |
| R3-39 | LIF-026 | `must`. |
| R3-40 | IFC-023 | Applied in part: the type of `operationIdentity`. The other values need decisions. |

## The points known before the round

| Point | Promises | What changed |
| --- | --- | --- |
| One `package` key | IFC-006, IFC-018 | `<digest>#<name>`; the key is removed. |
| Retrieval once the target is unregistered | OPS-015, OPS-016, IFC-024 | Refused. |
| An object's default | SCH-010, SCH-019 | Checked after its own properties' defaults are filled in. |
| A pointer that selects nothing in `outputs` | OPS-022, GRA-026 | Unknown if a step had an effect, failed otherwise. |
| What `obliges` requires | CTR-005, CON-022, ACC-038 | The ends whose implementations must state the requirement in `supports`. |
| The `settings` input of a reconciled control | REV-010 | The shape of the control's output `settings`. |
| A repeat's "same contents" | OPS-009 | Judged after defaults are filled in. |
| A runtime restored where the type does not allow it | OBS-007 (new), OBS-004, OBS-005 | The handle is no longer held, and the runtime is stopped. |
| The record of a request from no known instance | REC-002, IFC-032 | It holds no caller and says so. |
| How a value is marked as secret | none | Left for the owner. |

## Choices made where a reader gave options or no wording

| Where | Choice | Why |
| --- | --- | --- |
| R1-3.3, POOL-012 | Kept the ID and reduced the promise to what is its own. | `spec/core/` and `spec/schemas/` cite it, and the checker fails on a citation of a removed promise. |
| R1-4.3, R2-T7 | The key is `subject`. | Reader 1 proposes `item` and reader 2 `subject`; `item: Graph execution` would read as the item itself. |
| R1-2.2, LIF-009 | A request that names a component which cannot be replaced reports invalid. | Every request that is not dispatched names one reason, and LIF-012 gives invalid for a request that fits no object of that sort. |
| R2-S2, CTR-004 | A type that states no `schema` puts no rule on `config`. | INS-014 then has nothing to fail. The other reading would reject documents the examples accept. |
| R2-S7, IFC-029 | The version of a result is known by the prefix alone. | It adds no key. |
| R2-S10, NAM-005 | Kept the rejection and named the form. | The example keeps its name. |
| R2-T8 | "Basic" for `interface.v1` and `pool.v1`. | The reader's first word. |
| R2-T13 | "A value tied to a location" for the term that was "a value bound to a location". | "Bound" is the limit on a Conduit to `access`. |
| R2-G4, R3-16, NAM-022 | In the document that replaces a template, `#name` reaches a contract of the unpublished package the replaced template holds. | It changes no operation. |
| R2-St3, SCH-016 | "The edition that JSON Schema 2020-12 cites". | It names one edition without this file asserting its number. |
| R3-12, IFC-033 | `required` lists the inputs that are not optional and state no default; a handle is a string; a `$ref` to a Schema contract is replaced by that schema. | Each follows a promise: OBJ-015, IFC-011 and SCH-015. A description is not mentioned. |
| R3-14, ACC-019, IMP-006 | Each start is an invocation that the engine dispatches, and the work's requests name it. | It adds no key. |
| R3-20, GRA-026 | An absent source is treated as a pointer that selects nothing: invalid. | GRA-026 already lets no default stand in. |
| R3-21, R2-T9, CON-024 | The exempt operations are the management verbs, so a control of a target that is not ready reports unavailable. | It uses the term the definitions already had. |
| R3-24 | What a request to an unreachable location reports went into DL5. | It adds no behavior. |
| CON-022 | Where `obliges` names the consumer, the implementations are all those selected for the consumer. | The decision does not say which, and there is no narrower set. |
| ACC-018 | A step whose implementation is plain, or cannot apply a carried restriction, reports unavailable. | It is what ACC-014 already states. |
| OPS-028 | The identity derives from the invocation in which the step is first dispatched. | Otherwise the new attempt that OPS-021 dispatches in a later run could not carry the same one. |

## Left in disagreement or open

- CON-022 against IFC-018 and IFC-019. A filter whose `obliges` names only the consumer need
  not be in the `supports` of the target's implementation, yet "every filter on the Conduit" is
  sent to that implementation, which must apply it or answer unavailable.
- ACC-038 has an implementation named "by its qualified reference". An implementation of a
  project that is not published has none.
- IFC-025 returns the engine's ID of an invocation in the result of the request that began it.
  The result of `retrieve-result.v1` does not carry it, so a consumer that lost the first result
  cannot learn the ID.
- FST-006 rejects an instance document made from `system`, `management`, `access` or `root`.
  The valid example `first-start.yaml` writes those documents to show what first start makes.
- R3-7. ACC-033 and OPS-014 apply the decision's sentence to every invocation, so an unknown
  outcome of a control or of an ordinary operation is reconciled through a Conduit to the
  instance itself, whose type must offer a capability that includes `reconcile.v1`. Before the
  round such an invocation was reconciled through the Pool or `management`. The row of
  DECISIONS.md is titled for management operations. Editor S's documents follow ACC-033 as
  written, and no promise says what holds for a type that leaves `reconcile.v1` out.
- R1-3.1. ACC-019 says "an operation whose contract says that it arranges later work". No key
  of an Operation contract says so, and the decision names none.
- R3-8. LIF-022 refuses the rename of a first-start object. The two Conduits of `root`, which
  FST-002 lists, are not first-start objects, and nothing says what a rename of one reports.
- The findings marked "not applied" above each need a decision that is not in the record.

## Checks

Run from the repository root on 2026-11-06, after the last edit to these files.

```text
$ /usr/bin/python3 - (the validation command of CONSOLIDATION.md)
406 promises ok

$ (every promise ID and DL item cited in the files of this pass, and in this log outside
  this block, exists)
26 files; 2250 citations of promises, 44 of DL items; 406 promises and 15 DL items exist
cited in a promise, a definition, the README or this log and missing: []
DL items cited and missing: []
named only in header comments as removed: ACC-004 ACC-007 ACC-010 ACC-013 ACC-020 ACC-021 ACC-025 CON-005 CON-009 CON-010 CON-015 FST-004 GRA-013 GRA-016 INS-015 PKG-002 PKG-008 PKG-009 PKG-010 PKG-011 PKG-015 PLC-010 PLC-014 POOL-008 POOL-010
YAML lines over 96 characters: []

$ python3 tools/check_schemas.py        (last line; exit status 0)
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x433, apiVersion x217, metaPrefix x66. Errors: 0

$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 117 Markdown files and 1500 local links (offline).
Errors: 0

$ git diff --check -- (the files of this pass)
(no output; exit status 0)
```

The first run of `tools/check_docs.py` reported two missing links, both in Editor S's files and
both to that editor's log, which did not exist yet. The run above was made after it did.
