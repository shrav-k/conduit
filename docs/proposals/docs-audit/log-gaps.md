# Converting the gaps: log of the edits to the specification

Editor's log of 2026-11-05 and 2026-11-06. It records what was changed under `spec/` and in
`tools/check_schemas.py` to carry out decision DA3 of the [audit record](README.md): every row
of [GAPS.md](GAPS.md) except GP3, which that page leaves to the owner. Nothing outside `spec/`
was edited except `tools/check_schemas.py` and this file, and nothing was committed. The rules
followed are those of the
[consolidation instructions](../reaudit/CONSOLIDATION.md#rules-for-every-edit).

Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

"Row" names the row of GAPS.md that a change applies. "(c)" marks a change made only so that
one part of the specification agrees with another that a row changed.

## Counts

| File | Before | Changed | Added | After |
| --- | --- | --- | --- | --- |
| `promises/objects.yaml` | 24 | 1 | 0 | 24 |
| `promises/schemas.yaml` | 18 | 1 | 3 | 21 |
| `promises/instantiation.yaml` | 22 | 3 | 1 | 23 |
| `promises/templates.yaml` | 12 | 0 | 0 | 12 |
| `promises/contracts.yaml` | 9 | 4 | 1 | 10 |
| `promises/naming.yaml` | 27 | 3 | 0 | 27 |
| `promises/packages.yaml` | 19 | 1 | 0 | 19 |
| `promises/manifest.yaml` | 14 | 2 | 0 | 14 |
| `promises/submissions.yaml` | 8 | 1 | 0 | 8 |
| `promises/pools.yaml` | 14 | 0 | 0 | 14 |
| `promises/placement.yaml` | 17 | 0 | 1 | 18 |
| `promises/conduits.yaml` | 24 | 1 | 1 | 25 |
| `promises/graphs.yaml` | 22 | 2 | 3 | 25 |
| `promises/first-start.yaml` | 4 | 0 | 0 | 4 |
| `conventions/lifecycle.yaml` | 26 | 0 | 1 | 27 |
| `conventions/access.yaml` | 28 | 4 | 3 | 31 |
| `conventions/operations.yaml` | 25 | 1 | 2 | 27 |
| `conventions/revisions.yaml` | 13 | 1 | 0 | 13 |
| `conventions/records.yaml` | 6 | 1 | 0 | 6 |
| `conventions/observation.yaml` | 6 | 1 | 0 | 6 |
| `conventions/implicit.yaml` | 9 | 0 | 0 | 9 |
| `interface/implementation.yaml` | 29 | 5 | 3 | 32 |
| Total | 376 | 32 | 19 | 395 |

No promise was removed, reused or moved between files. The implementation-defined list went
from 13 items to 14.

New IDs, each the next number never used in its prefix: CTR-010, SCH-019, SCH-020, SCH-021,
INS-024, CON-029, PLC-020, GRA-025, GRA-026, GRA-027, OPS-026, OPS-027, ACC-036, ACC-037,
ACC-038, LIF-027, IFC-030, IFC-031, IFC-032, and the item DL14.

## Promises

| ID | What changed | Row |
| --- | --- | --- |
| CTR-002 | An input that holds a `schema` may hold `default`; an output, and an input that is a handle, may not | GP7 |
| CTR-004 | A Type may hold `restoresRuntimes`, and `records` with `inputs` and `outputs`; each is false when left out | GP4 |
| CTR-005 | A Requirement may hold `obliges`, drawn from `consumer` and `target`; `constituents` is a map of the constituents CTR-010 gives, no longer "a map of applied requirements" | GP15, GP1 |
| CTR-006 | A control declares the input `revision` and the output `settings`; a type that lists one without either is rejected | GQ10 |
| CTR-010 | **New.** A constituent holds `requirement` and may hold `arguments`, value bindings whose only source is `input`, naming an input of the composition | GP1 |
| SCH-009 | Observable: the `config` of a template that writes none is present, so the type's defaults apply; a `default` reached only through a `$ref` inside a combiner is not applied | GQ5, GQ6 |
| SCH-019 | **New.** A `default` that fails the schema that states it, or an input's `default` that fails the input's `schema`, rejects the document | GP5 |
| SCH-020 | **New.** A number that is not an integer is read as an IEEE 754 binary64 value | GP6 |
| SCH-021 | **New.** Such a number outside the range of binary64 rejects the document | GP6 |
| OBJ-019 | A literal value is the value of `literal` or of an input's `default`; a constituent's `arguments` hold value bindings | GQ1, GP1 (c) |
| INS-011 | Observable: a destination inside the `config` of a template that writes none has its parent | GQ5 |
| INS-014 | Observable: cites INS-024 | GQ5 |
| INS-018 | Observable: cites PLC-020 | GP12 (c) |
| INS-024 | **New.** A template that writes no `config` means the same as one that writes `config: {}` | GQ5 |
| CON-003 | Observable: where no shortening makes the name fit, CON-029 applies | GQ4 |
| CON-029 | **New.** A component Conduit whose name cannot be made to fit in 63 characters is not instantiated | GQ4 |
| SUB-008 | `kind` and `metadata.name` are compared by the bytes of their UTF-8 strings | GQ3 |
| NAM-018 | Observable: the `requirement` of a constituent is a selection | GP1 (c) |
| NAM-022 | `#name` refers to the document, or the implementation, of that name | GQ11 |
| NAM-027 | No implementation has the name of a document of its project or package | GQ11 |
| PKG-012 | Observable: an implementation with the name of a document is rejected under NAM-027, exported or not | GQ11 |
| PKG-023 to PKG-025 | Not changed. The header of `packages.yaml` says that they are kept as three | GQ13 |
| MAN-003 | Cites SCH-020 and SCH-021 beside the other rules on numbers | GP6 (c) |
| MAN-007 | A name an implementation also has is rejected under NAM-027, no longer PKG-012 | GQ11 |
| PLC-020 | **New.** A value bound to a location is interpreted only by the instance's implementation at its placement | GP12 |
| GRA-007 | Observable: an input that states a `default` need not be given a value | GP7 (c) |
| GRA-020 | Observable: in a constituent, `input` is the only source | GP1 (c) |
| GRA-025 | **New.** A step whose resolved arguments do not satisfy its operation's inputs is not dispatched and reports invalid | GP8 |
| GRA-026 | **New.** A step with a value reference whose `pointer` selects nothing is not dispatched and reports invalid | GP9 |
| GRA-027 | **New.** A graph reads the configuration as it stood when its invocation was dispatched | GP10 |
| OPS-016 | Observable: once the target is unregistered the result can no longer be retrieved through the tool; the records remain | GQ7 |
| OPS-026 | **New.** The engine fills in the `default` of an Operation's input before dispatch; a control applies none | GP7 |
| OPS-027 | **New.** A cancelled invocation's outcome is unknown unless its implementation confirms that nothing was done | GP19 |
| ACC-008, ACC-014, ACC-015 | Observable: the response names the restriction in `io.github.shrav-k.conduit/restriction` | GQ8 |
| ACC-027 | In this version every restriction is reported as mediated | GQ9 |
| ACC-036 | **New.** An access decision is made anew for each request, and no earlier one is reused | GP2 |
| ACC-037 | **New.** Where the state of the Conduits or the policies cannot be established, the request reports unavailable | GP2 |
| ACC-038 | **New.** Inspecting an open Conduit names, for each requirement, the implementation that applies it | GP15 |
| LIF-027 | **New.** A setup that does not succeed closes no Conduit it opened and retires nothing it set up | GP18 |
| REV-007 | `settings` is an object keyed by the pointers of the control's `changes`, holding the value now stored at each place confirmed | GQ10 |
| REC-002 | Observable: what content is kept is stated under `records` in the type's contract | GP4 |
| OBS-005 | Observable: a type allows restoring with `restoresRuntimes: true` | GP4 |
| IFC-006 | The new key `io.github.shrav-k.conduit/package`; a contract of an unpublished package is named `#name` | GQ2 |
| IFC-018 | `requirement` is a qualified reference or, for an unpublished package, `#name` | GQ2 |
| IFC-022 | Observable: cites IFC-030 and IFC-032 | GP13, GP17 (c) |
| IFC-024 | Observable: a result whose target is unregistered can no longer be retrieved | GQ7 |
| IFC-025 | The engine's own answer names the restriction under the key an implementation uses | GQ8 |
| IFC-030 | **New.** What a consumer presents to identify itself is never sent to an implementation | GP13 |
| IFC-031 | **New.** An implementation that calls a service outside never forwards Conduit's keys to it | GP14 |
| IFC-032 | **New.** A request that cannot be established as coming from an instance is not dispatched and reports refused | GP17 |

## The other files of the specification

| File | What changed | Row |
| --- | --- | --- |
| `implementation-defined.yaml` | DL1 gains "when a graph's outputs become available". New item DL14, cancelling an invocation | GP11, GP19 |
| `schema-profile.md` | A row for IEEE 754 binary64; the order of an unpublished package's documents; a `default` reached through a `$ref` in a combiner; a `default` satisfies its schema; a number outside binary64 among what is not accepted | GP6, GQ3, GQ6, GP5 |
| `definitions.md` | New rows: value bound to a location, constituent, isolation plugin, cancel. Changed rows: document, type, template, input, argument, `#name`, access decision, requirement, applied requirement, mediated, retained result, invalid, unavailable, Conduit's keys, restore, configuration, control, unpublished package, step, value binding, `source`, set up, inspect, record | Every row that adds or changes a term |
| `README.md` | The status says that the specification alone binds, that `docs/protocol/` is superseded source material for type contracts, and that a behavior missing from `spec/` is a gap in `spec/`. A third conformance level, Implementation. Every count recounted: 395 promises, 14 items, 55 and 11 example files | GQ12, GP16 |

## Schemas, core package and examples

| File | What changed | Row |
| --- | --- | --- |
| `schemas/contracts.yaml`, `type-spec.v1` | `restoresRuntimes`, a boolean; `records`, with the booleans `inputs` and `outputs` | GP4 |
| `schemas/contracts.yaml`, `operation-spec.v1` | An input that holds `schema` may hold `default`. Inputs and outputs now have separate definitions, so an output cannot | GP7 |
| `schemas/contracts.yaml`, `requirement-spec.v1` | `obliges`; a constituent is no longer `applied-requirement.v1` and has its own form, with value bindings whose one source is `input` | GP15, GP1 |
| `schemas/contracts.yaml`, `applied-requirement.v1` | Description only: it is no longer the form of a constituent | GP1 |
| `schemas/templates.yaml` | Comment: `config` may be left out; a `default` is a literal value | GQ5, GQ1 |
| `schemas/README.md` | The row on an input's `default` no longer reports a difference from OBJ-019. New rows for SCH-019, CTR-010 and the implementation half of NAM-027. A table of the four additions. The counts of examples | GQ1, GP1, GP4, GP5, GP7, GP15, GQ11 |
| `core/operations.yaml` | Header and descriptions: the shape of `settings`; no core input states a `default`; a result whose target is unregistered. No contract changed shape: both controls already declare `revision` and `settings` | GQ10, GP7, GQ7 |
| `core/types.yaml`, `core/requirements.yaml` | Comments: no core contract states `restoresRuntimes`, `records` or `obliges`, and none is a composition | GP4, GP15, GP1 |
| `core/README.md` | The controls; the keys of a request's `_meta`, with `package`; the engine's own answer carries `restriction`; retrieval after unregistration is settled and no longer listed as open | GQ10, GQ2, GQ8, GQ7 |
| `examples/valid/files-package/contracts.yaml` | `directory.read.v1` gives `maxEntries` a `default`; `local-directory.v1` states `restoresRuntimes` and `records`; `rate-limit.v1` states `obliges`; new composed Requirement `review-limit.v1` | GP7, GP4, GP15, GP1 |
| `examples/valid/files-package/conduit.yaml` | Exports `review-limit.v1` | GP1 |
| `examples/valid/project/templates.yaml` | The Interface template `console` writes no `config` | GQ5 |
| `examples/invalid/CTR-002-output-with-default.yaml` | New | GP7 |
| `examples/invalid/CTR-004-records-as-a-list.yaml` | New | GP4 |
| `examples/invalid/CTR-005-obliges-an-unknown-end.yaml` | New | GP15 |
| `examples/invalid/CTR-010-constituent-with-source-config.yaml`, `-with-bare-value.yaml` | New | GP1 |
| `examples/invalid-by-rule/CTR-010-constituent-names-an-undeclared-input.yaml` | New | GP1 |
| `examples/invalid-by-rule/SCH-019-default-fails-its-schema.yaml`, `SCH-019-input-default-fails-its-schema.yaml` | New | GP5 |

## tools/check_schemas.py

| What changed | Row |
| --- | --- |
| Reading a number: a whole number is told from another by its written value, so a number with a fractional part above the I-JSON range is no longer refused as a whole number; a number that is not whole and is beyond binary64 is refused for SCH-021; `1e400` no longer stops the tool with an error and is refused for SCH-006 | GP6 |
| New rule of step 6, for SCH-019: every `default` of the core package and the valid examples is validated against the schema that states it, and every input's `default` against the input's `schema`. It follows a `$ref` inside the repository and counts, without validating, a default whose schema leads outside it. It needs jsonschema; without it the rule is not applied and the report says so | GP5 |
| New rule of step 6, for CTR-010: a value reference in a constituent names an input the composition declares | GP1 |
| A control that lacks the input `revision` or the output `settings` is reported for CTR-006, no longer as a check against the record | GQ10 |
| An implementation that has the name of a document is reported for NAM-027 | GQ11 |

## Choices made that GAPS.md does not state

| Choice | Why |
| --- | --- |
| GP8, GP9 and GP10 are in `graphs.yaml`, with the layer `convention`. GP7 and GP19 are in `operations.yaml` | Their subject is a step and a graph. The two invariants of a graph are already there with that layer |
| GP2 is two promises, ACC-036 and ACC-037 | One obligation to a promise, and the second names a reason |
| GP6 is two promises, SCH-020 and SCH-021 | The same |
| GP15's convention half is in `access.yaml`, beside ACC-027, which is the other thing an inspected Conduit reports | — |
| SCH-019 covers an input's `default` against the input's `schema`, as well as the keyword `default` inside a schema | In this specification an input states its default beside its schema, not inside it. The row's "its template registered" fits only that reading |
| An input of an Operation that states a `default` may be left out whether or not it is marked `optional: true` (OPS-026, GRA-007) | Otherwise GRA-007 would reject the step that the default is for. INS-004 already reads a template's input so |
| A constituent's value reference may add `pointer` | GRA-020 allows it for the source `input`, and the row says value bindings |
| A constituent has its own form inside `requirement-spec.v1`, and no new Schema contract | A new contract would be a new export of the core package |
| No core contract states `obliges`, `restoresRuntimes` or `records`, and no core input states a `default` | No promise says what obliging an end requires of it, so no value for `bound.v1` or `shared-host.v1` could be checked. No core type declares a runtime |
| ACC-037 cites DL4 for whether the engine waits | DL4 already leaves that to the engine, and the two then agree |
| The Implementation level names four promises as examples and does not list them all | Which promises bind an implementation follows from each promise's own wording |

## Left undone, or found in disagreement

Nothing was chosen where a row did not decide. Each point below needs a decision.

1. **GQ2, two unpublished packages in one request.** IFC-006 now sends one key, `package`. An
   onward request can carry a restriction from the Conduit the caller came in through (ACC-017),
   whose requirement may be a contract of another unpublished package than the operation's.
   One digest cannot name both. The row was written as it stands.
2. **GQ7, the reason.** OPS-016 says the result can no longer be retrieved through the tool. It
   does not say which of the four reasons a call then reports, and no promise says what a
   request that names an object that does not exist reports in general.
3. **GP5 and SCH-010.** The older text checked a default "after applicable default
   preparation". The row does not, so SCH-019 does not say whether a default that is an object
   is checked before or after the defaults of its own properties apply under SCH-010.
4. **GP9, a graph's outputs.** GRA-026 covers a value reference in a step's arguments, as the
   row does. A `pointer` that selects nothing in a value binding of a graph's `outputs` is not
   covered.
5. **GP15.** `obliges` says which end a requirement obliges. No promise says what follows for
   that end, how ACC-038 names an implementation, or where in the result of `inspect.v1` the
   name appears. `inspect.v1` fixes none of it, as it fixes none of ACC-027.
6. **GQ10, `reconcile.v1`.** REV-007 gives the shape of a control's output `settings`.
   REV-010 has a reconciliation state "the settings that took effect" in the input `settings`
   of `reconcile.v1`, and nothing says that it has the same shape. The outputs of the two core
   controls are still described as `{type: object}`.
7. **GP7 and OPS-009.** A repeat has "the same contents" when its arguments are equal. Whether
   that is judged before or after a default is filled in is not said.
8. **GP4.** No promise says what follows when an implementation restores a runtime and its
   type does not state `restoresRuntimes: true`.
9. **GP17 and REC-002.** A record holds the caller. A request that reports refused under
   IFC-032 has no instance established for it, and nothing says what its record holds.
10. **GP1 against the earlier wording.** CTR-005 said that `constituents` is "a map of applied
    requirements", and the definition of an applied requirement gives it plain `arguments`
    (NAM-019). The row replaces that for a constituent, so CTR-005, the definition and the
    schema were changed. Nothing else collided with a row.
11. **No example for SCH-021 or for the implementation half of NAM-027.** A number that has a
    fractional part and is beyond binary64 is longer than a line of an example. A set under
    `examples/invalid-by-rule` holds no manifest, so it cannot hold an implementation. The tool
    applies both rules, and the first was tried by hand on a number of 320 digits.
12. **GP7 against OBJ-015.** OBJ-015 says of an input that "without that mark it is required".
    OPS-026 says that an input of an Operation that states a `default` "may be left out
    whether or not it is marked `optional: true`". INS-004 already reads a template's input in
    that way, so both were left: an input with a default is required in that it always has a
    value. OBJ-015 was not reworded.
13. **GP3** was not converted, as GAPS.md says.

## Checks

Run from the repository's root after the last edit.

- The validation command of the consolidation instructions, which reads every file under
  `spec/promises/`, `spec/conventions/` and `spec/interface/`, `promises/manifest.yaml`
  included: `395 promises ok`.
- Every promise ID and every item of the implementation-defined list cited in any `.yaml` or
  `.md` file under `spec/`, file names included: 2,478 citations of promises and 30 of items
  in 119 files, 0 missing. 43 of them name a removed promise, each in the header comment of a
  promise file.
- No line of a YAML file under `spec/` is longer than 96 characters, except one line of
  `spec/core/references.yaml`, which is 97 and was so before this pass. `git diff --check`
  reports nothing for `spec/` or `tools/`.
- `python3 tools/check_schemas.py`, which ran itself again under `/usr/bin/python3`:
  `Errors: 0`. It read 90 files holding 205 documents, checked 58 Schema contracts, validated
  135 documents, saw 55 invalid documents rejected by a schema or refused when read, found 98
  exports and 1 implementation in the core package, applied 10 rules of its own from 9
  promises, saw 11 sets under `invalid-by-rule` rejected, and validated 8 defaults against
  their schemas with none left unvalidated.
- The same tool with jsonschema made unimportable: steps 3 and 4 and the rule for SCH-019
  reported as not run, `Errors: 0 in the steps that ran`, exit status 3.
- `python3 tools/set_names.py --check`: `Errors: 0`.
- `python3 tools/check_docs.py`: `Errors: 0`.
