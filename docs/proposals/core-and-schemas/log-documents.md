# The ruling on the core package and the schemas — log of the document side

Written 2026-11-05 by Editor S, under [APPLY.md](APPLY.md), the [record](README.md) and the
rules of the [consolidation instructions](../reaudit/CONSOLIDATION.md). Editor S's files are
everything under `spec/core/` and `spec/schemas/`, `spec/promises/manifest.yaml`,
`tools/check_schemas.py` and the record. No other file was edited, and nothing was committed.
The promise side is logged in [log-promises.md](log-promises.md).

Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

"Item" names the row of APPLY.md, or the choice (C), point (P) or disagreement (D) of the
record, that the change applies.

## Every file: its status

Each YAML file under `spec/core/` and `spec/schemas/`, the examples included, said "PROPOSED,
not accepted" in its first lines. Each now says that it was accepted by the owner on
2026-11-05 (plan-audit ruling 37), or, for an example, begins "Example". The two READMEs and
the header of `spec/promises/manifest.yaml` say the same and link APPLY.md.

## spec/core

| File | What changed | Item |
| --- | --- | --- |
| `operations.yaml` | New: the control `set-facts.v1`, with `changes: [/facts]`, the inputs `facts` and `revision` and the output `settings` | X1 |
| `operations.yaml` | `inspect.v1`: the status may hold `policies`, each with an `id` | D12 |
| `operations.yaml` | `register.v1`: a rejected registration still carries `diagnostics` | D21 |
| `operations.yaml` | `register-replacement.v1`: authorized by the same Conduits as registering, each listing this operation | D8 |
| `operations.yaml` | `install.v1`: the three reasons a rejected installation reports | P5 |
| `operations.yaml` | `rename.v1`: a rename that makes a tool name equal to another, or longer than 128 characters, reports conflict | P8 |
| `operations.yaml` | `apply-policy.v1`, `remove-policy.v1`: say that neither names a revision | D12 |
| `operations.yaml` | `unregister.v1`: an instance is removed if its shutdown is confirmed or it was retired because its location was declared lost, as LIF-017 states. The description had only the first | Found in Editor P's log |
| `operations.yaml` | Header: the rules for a revision, a control, the reserved mark, how a request names things, and diagnostics | P14, D13, X4, X2, D21 |
| `capabilities.yaml` | New: the capability `location-settings.v1`, which includes `set-facts.v1` | X1 |
| `types.yaml` | `location.v1` offers `location-settings.v1`, requires `set-facts.v1` and lists it under `controls` | X1 |
| `conduit.yaml` | Exports `location-settings.v1` and `set-facts.v1`; the engine implements `set-facts.v1`; the comment on the version no longer calls it a proposal | X1, C41, D2 |
| `requirements.yaml` | Comment: where a bound may be applied, and that its lists hold references through `items` | X3, D14 |
| `references.yaml` | Comment: a reference schema governs a place directly, as `items` or as a branch; `{entry: name}` is rejected outside an entry | D14, D15 |
| `templates.yaml` | Comment on `location`: the argument sets the facts, and the control changes them afterwards | X1 |
| `results.yaml` | Comment: names choice C16 | — |
| `README.md` | Status; the counts (20 Operation and 5 Capability contracts, 98 exports); rows for the two new exports; the controls, the reserved mark, how a request names things, diagnostics, the `_meta` keys beside `retrieve-result.v1`, and the status of `access`; what is still not settled | X1, X2, X4, D12, D13, D21, P11 to P14 |

Confirmed and left as they were: `set-up.v1`, `retire.v1`, `retire-with-resources.v1`,
`open.v1` and `close.v1` take no `revision` (P14); `apply-policy.v1` returns an ID and
`remove-policy.v1` takes it (D12); `set-default-location.v1` returns `settings` (D13);
`retrieve-result.v1` takes `target` (D22); `purge.v1` takes `before` (D23); every document of
the core package writes the core package's address in full (D18).

No schema and no output of a core operation describes a result's `_meta`, so P11, P12 and P13
add no key to any contract. The core package's README names the three keys and the promises
that state them.

## spec/schemas

| File | What changed | Item |
| --- | --- | --- |
| `schema.yaml` | `schema.v1` accepts a third form of `$ref`: `#` and a name | P3 |
| `schema.yaml` | Comment: a `default` is judged where it is written, so nothing about it is left to the engine | D26 |
| `templates.yaml` | `template-spec.v1`, `pool-template-spec.v1` and `conduit-template-spec.v1` each carry, as one pattern under `$defs` as `reaches`, the closed list of places a destination may reach | P4 |
| `templates.yaml` | `conduit-template-spec.v1`: a template that is not composed requires `capability`, and `operations` or else `inputs` | D25 |
| `templates.yaml` | `implementation-map.v1`, `template-controls.v1`: a key is a selection, `#name` included | D9, D20 |
| `values.yaml` | `contract-name.v1` requires at least one segment before the version segment | D27 |
| `values.yaml` | `address.v1`, `qualified-reference.v1`: a label of the host is at most 63 characters | P6 |
| `values.yaml` | `destination.v1`: says that it checks the form only | P4 |
| `contracts.yaml` | `operation-spec.v1` documents the mark `reservedToRoot`, in its description and on the key | X4 |
| `instances.yaml` | Comment: what is left to the engine now names the bound's target, a location's placement and `{entry: name}` | X3, D31, D15 |
| `manifest.yaml` | Comment: the promises are no longer "proposed"; names NAM-027; `supports` and `placement` may be left out | X5, D24 |
| `README.md` | Status; the three forms of `$ref`; the table of destinations; the table of what a schema cannot express, with a column saying which rules the tool applies itself and new rows for CON-016, CON-028, CTR-009, NAM-027, PLC-008 and POOL-005; the open `default`; the examples; what the checker does | P3, P4, X3, X4, X5, D15, D25, D31 |

## spec/schemas/examples

| File | What changed | Item |
| --- | --- | --- |
| `valid/project/contracts.yaml` | New: a Schema contract of the project itself | P3 |
| `valid/project/templates.yaml` | The input `model` uses `$ref: '#model-name.v1'`; new Conduit template `team-access`, which applies `bound.v1` | P3, X3 |
| `valid/project/instances.yaml` | New Conduit `operator-access`, to `access` | X3 |
| `valid/files-package/contracts.yaml` | Two inputs use `$ref: '#read-limit.v1'` | P3 |
| `valid/first-start/first-start.yaml` | Comment: the facts are changed by `set-facts.v1` | X1 |
| `invalid/INS-012-destination-item-of-operations.yaml`, `-whole-nested-instantiation.yaml`, `-requirement-of-a-requirement.yaml`, `-under-inputs.yaml` | New | P4 |
| `invalid/CTR-001-contract-name-only-a-version.yaml` | New | D27 |
| `invalid/NAM-001-manifest-address-label-too-long.yaml`, `-numeric-host.yaml`, `-trailing-dot.yaml` | New | P6 |
| `invalid/CON-016-template-without-operations.yaml` | New | D25 |
| `invalid/SCH-015-ref-to-a-bare-name.yaml` | New | P3 |
| `invalid/TPL-002-conduit-template-with-type.yaml` | Gained `operations`, so that `type` is its only fault under the changed schema | D25 |
| `invalid-by-rule/CON-028-bound-applied-twice.yaml`, `CON-028-bound-on-a-target-that-is-not-access.yaml` | New | X3 |
| `invalid-by-rule/CTR-009-reserved-mark-outside-the-core-package.yaml` | New | X4 |
| `invalid-by-rule/NAM-027-two-documents-one-name.yaml` | New | X5 |
| `invalid-by-rule/PLC-008-location-with-placement.yaml` | New | D31 |
| `invalid-by-rule/POOL-005-entry-reference-outside-an-entry.yaml` | New | D15 |
| `invalid-by-rule/CON-016-no-input-fills-operations.yaml` | New | D25 |
| `invalid-by-rule/SCH-015-ref-to-a-name-that-is-not-a-schema.yaml` | New | P3 |

The directory `invalid-by-rule` is new. No schema can reject what it holds: every document in
it is valid against the schemas, and the checker verifies that before it applies the rule the
file is named for. It is kept apart from `invalid` so that the report never credits a schema
with a rejection a schema did not make.

## spec/promises/manifest.yaml

No promise was added or removed. The header says the file is accepted and lists what is cited
and not restated.

| ID | What changed | Item |
| --- | --- | --- |
| MAN-005 | Cites NAM-006 for what an item states and NAM-012 for `fetch`, and NAM-001 for the form of an address | Part (e) of the brief |
| MAN-007 | Cites PKG-018 for what an export is, and NAM-027 for a name being the name of one document | X5 |
| MAN-009 | Cites PKG-019 and PKG-020 for the keys, and PKG-019 for what leaving out `supports` or `placement` means, which it no longer says itself | D24 |
| MAN-010 | Cites NAM-018 for a selection and PKG-020 for `tools`; says that `plain: false` means the same as leaving `plain` out | D9, C8 |
| MAN-011 | Names `.yml` as an ending that is not read | D28, C53 |
| MAN-012 | Cites NAM-009 and PKG-005 for what a template or an instance holds, which it no longer says itself; says that the core package's manifest lists no `packages` and that the engine states `bound.v1` under `supports` | D2, D1 |
| MAN-013, MAN-014 | Cite PKG-023 for the reason a rejected installation reports | P5 |

## tools/check_schemas.py

| What changed | Item |
| --- | --- |
| Step 2 accepts and resolves `#name` in a `$ref` of a Schema contract; step 5 resolves it in every document of a project or package | P3 |
| Step 5: every export exists; every document is exported or listed in `PRIVATE`, which is empty; the five operations of REV-012 take no `revision`; every control has `changes` its type's schema describes, takes `revision`, has every other input optional and returns `settings`; the tables of the two READMEs list the files, the exports, their kinds and promises; every promise cited under `spec/core/`, `spec/schemas/` and in `spec/promises/manifest.yaml` exists | Part (h) of the brief, P14, D13 |
| Step 6, new: eight rules the tool applies itself, from seven promises, to the core package and the valid examples, and to each set under `examples/invalid-by-rule/` | X3, X4, X5, D15, D25, D31, P3 |
| A contract's name needs a segment before its version segment | D27 |
| Its report says of each rejection whether a schema or one of its own rules made it. Without jsonschema it runs steps 1, 2, 5 and 6 and says that no document was validated | — |

The rules of step 6 go only as far as the documents in front of the tool show, and each rule's
comment says how far. They are not the engine's checks.

## The record

Status and title; the link to APPLY.md; the new section "Outcome", with the five changes, the
open points and the disagreements, one line each, what the batch holds now, the choices made
while applying the ruling, what was found and not settled, and what was verified. Sections
(a) to (e) are as they were proposed.

## Choices made that neither the record nor APPLY.md states

They are listed, with the reason for each, in the record under
[Outcome](README.md#choices-made-while-applying-the-ruling) as A1 to A7. In short: what
`set-facts.v1` takes and declares beyond X1; that only `id` is fixed of a policy in a status;
how a name is written inside a destination; the form of `#name` in a `$ref`; how far
`conduit-template-spec.v1` can express D25; the separate directory for rules no schema
expresses; and two sentences added to MAN-010 and MAN-012.

## Left undone, and why

1. **No promise was added to `manifest.yaml`.** The brief allows one where the accepted batch
   needs it. The one candidate changes behavior and is not in APPLY.md, so it is reported and
   not written: NAM-022 says that `#name` "refers to the document of that name", while an
   implementation, which is not a document, is selected as `#name` under NAM-018; and neither
   NAM-027 nor PKG-012 makes an implementation's name differ from the name of a document that
   its package does not export.
2. **OBJ-019 and the value of a `default`.** `template-input.v1` and `requirement-spec.v1`
   leave the value of an input's `default` open. OBJ-019 exempts "a key inside `config`,
   `annotations` or a literal value" and does not name a `default`. The schemas were left as
   accepted, and `spec/schemas/README.md` now says that this one place is open beyond what
   OBJ-019 exempts. Editor P's log reports the same.
3. **D22, the second half.** `retrieve-result.v1` names the target by a reference. A target
   that has been unregistered can no longer be named, and nothing says how its result is
   retrieved.
4. **`{entry: name}` in a schema.** The reference schemas still accept it wherever a Resource
   is accepted (C14, accepted). The document schemas could reject it in the `placement`,
   `consumer` and `target` of an instance document, which are never inside an entry, only by
   no longer using the reference schemas there. That is a change to the accepted schemas that
   APPLY.md does not ask for, so the engine is left the rule and the tool applies it to those
   places.
5. **D31 in a schema.** A schema could reject `placement` beside
   `from: github.com/shrav-k/conduit/core#location` with a pattern that excludes that one
   string. It would miss a location made from any other template of the location type, and
   it would write the core package's address in a form `tools/set_names.py` does not rewrite.
   It is listed for the engine.
6. **P7 and P9 to P13** change no document: no schema or core document states them.
7. **A valid example that leaves out `config`** (D17) was not added. Every template that has
   a type still writes `config`.
8. **`pattern` is still evaluated by Python's `re`**, not by an ECMA-262 engine (SCH-016).

## Checks

The results, as run from the repository's root after the last edit, are in the record under
[Verified after the ruling was applied](README.md#verified-after-the-ruling-was-applied).
