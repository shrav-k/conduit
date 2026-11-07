# Ruling 42: log of the core package and the schemas

Date: 2026-11-06. This is the account of how the editor of the core package and the schemas
applied [ruling 42](README.md) to its files: parts (a) and (b) of the ruling, the coordinator's
decisions in the table
[Settled by the coordinator](../ruling-41/open-decisions.md#settled-by-the-coordinator) with the
notes "Settled together" under it, and the owner's answers D6, D8, D10, D11, D12, D17 and R5
under [For the owner](../ruling-41/open-decisions.md#for-the-owner), where they touch these
files. The rulings are the owner's. The wording is the editor's, and stands unless the
coordinator or the owner strikes it. The names are those in force before the renames of the
vocabulary audit (`secret-reveal.v1`, `reveal-secret.v1`, location, operation identity and so
on): another stage applies the names N1 to N20 and the editorial fixes E1 to E23. Row 5 of the
table, D5 and D20 are not applied: D6 decides row 5, and the owner left D5 and D20 for more
research. For D5 nothing here changed; for D20 the text keeps row 17's guard.

Files: everything under [`spec/core/`](../../../spec/core/README.md) and
[`spec/schemas/`](../../../spec/schemas/README.md), with the examples, and
[`tools/check_schemas.py`](../../../tools/check_schemas.py). No other file was touched but this
log.

## Counts

These files hold no promise, so no promise was changed or added here. What changed:

| | Before | Changed | Added | Removed | After |
| --- | --- | --- | --- | --- | --- |
| Exports of the core package | 102 | 21 | 0 | 0 | 102 |
| of which Operation contracts | 23 | 16 | 0 | 0 | 23 |
| of which Capability, Type and Requirement contracts | 14 | 3 | 0 | 0 | 14 |
| of which Schema contracts | 58 | 2 | 0 | 0 | 58 |
| Rules of `tools/check_schemas.py` | 20, from 19 promises | 2 | 2 | 0 | 22, from 21 promises |
| `examples/invalid-by-rule/` | 21 files | 0 | 5 | 0 | 26 files |
| `examples/valid/` | 7 files | 1 | 0 | 0 | 7 files |

The contracts changed, by file:

- `spec/core/operations.yaml` (16): `register.v1`, `unregister.v1`, `inspect.v1` (its schema
  too), `set-up.v1`, `retire.v1`, `register-replacement.v1`, `install.v1`,
  `retire-with-resources.v1`, `close.v1`, `setup-hook.v1`, `retire-hook.v1`, `run.v1` (its
  schema too), `stop.v1`, `list-machines.v1`, `place.v1`, `reveal-secret.v1`. The comments that
  state the rules of the file changed too.
- `spec/core/capabilities.yaml`: `secret-reveal.v1`. `spec/core/types.yaml`: `location.v1`.
  `spec/core/requirements.yaml`: `bound.v1`. All three in their descriptions alone.
- `spec/core/results.yaml`: `record.v1`, its schema and its description.
- `spec/schemas/manifest.yaml`: `implementation-item.v1`, its description alone.
- `spec/schemas/templates.yaml` and `spec/schemas/contracts.yaml`: their comments alone.
- `spec/core/README.md` and `spec/schemas/README.md`: the status, a paragraph on ruling 42, and
  the passages and table rows that the changes touch.

Three shapes changed: `run.v1`'s inputs `secrets` and `identity` and its required output
`endpoint`; `inspect.v1`'s `status.unreconciled`; and `record.v1`'s `data.serving`. Each file
changed has a line for ruling 42 in its head comment, and the new example files say what they
show in theirs.

## Decisions applied

Rows are those of the coordinator's table; answers are the owner's.

| Row or answer | Promises | What changed here |
| --- | --- | --- |
| 1 (D1) | PLC-007, PRV-015 | `place.v1`: for an instance that has an item that is run and names no `runtime`, `selector` also holds the hostname of the engine's own machine. `spec/core/README.md`, Providers: the same |
| 3 (D3) | TPL-009 | `retire.v1` and `retire-hook.v1`: the stop is asked for whatever came of the hook; a hook that did not succeed keeps the shutdown pending and is dispatched again, against a new run, by a further retirement |
| 6, 10, 36 (D7, D12 R27, R27) | ACC-029, SUB-008, PKG-029 | `bound.v1`: an item of `operations` covers the operation of that name in every submission at its address, and in a package installed there later. `install.v1`: the conflict with a submitted project's address holds while a template or an instance holds that submission. README, Bounds |
| 9 (D12 part) | SEC-013, TPL-018 | `retire.v1` and `retire-hook.v1`: a retirement whose hook goes to a `remote` item with no connection that lasts connects first |
| 11, 33 (D13, R25) | TPL-012, OPS-027, LIF-021 | `set-up.v1`: no part begins once the instance is retired or the setup is cancelled, and the cancellation is passed on. `retire.v1`: a cancelled retirement still asks for the stop. `retire-with-resources.v1`: a cancelled request dispatches no further retirement |
| 12, 18 (D14, R3) | REC-002 | `record.v1`: `data.serving`, an ID; the description says what `serving` holds, that the operation identity may be one the engine gave a hook, that a record with no ID of its own holds in `invocation` the invocation within which it happened, and that a record of `run.v1`, `stop.v1` or `inspect-run.v1` names the instance in `subject` |
| 13 (D15) | PRV-014, PLC-026, PLC-023 | `list-machines.v1`: canonical hostnames, each at most once, the contract states which name; `gone` only for a machine known to exist no longer; a report that names a hostname twice is not used; an omitted machine counts as unreachable. `location.v1`: a location stands for a hostname, whichever provider reports it |
| 15 (D18) | PKG-020, PRV-005, IFC-037 | New rule `plain-provider` (PKG-020) and its example; the rules comment of `operations.yaml`; README, Providers and Conduit's keys; `manifest.yaml`'s comment; the schemas README |
| 16, 17, 22, 23, 24 (D19, D20 part, R9, R10, R11) | PRV-007, PRV-003, MAN-017 | `run.v1`: `required: [endpoint]`, every runtime provider returns it; `secrets` holds `environment` and `arguments`, maps of names to strings, in place of keys that were pointers; the conversion of numbers and booleans, and null left out; a name never in both `environment` and `secrets.environment`; never a `command` item whose program is a relative path (row 17's guard, kept for D20). README: the open question on `secrets` keys is gone |
| 19 with R5 | LIF-021 | `retire-with-resources.v1`: a retirement that ended without the shutdown confirmed, or that reported a reason such as conflict in place of being dispatched, holds back only the Resources that the order puts after its Resource; every other Resource is retired; the Pool is not; a repeat finishes. README, Retiring and removing |
| 20 (R6) | TPL-004, TPL-017, TPL-018 | `setup-hook.v1` and `retire-hook.v1`: selected in the template's `implementation` map or by its type's default |
| 26 (R16) | PLC-025 | Rule `label-conflict` also compares every pair of selected items; example `PLC-025-two-items-name-one-label.yaml`; `templates.yaml`'s comment; the schemas README |
| 27 (R17) | PLC-005, PRV-006, LIF-009 | `register.v1` and `register-replacement.v1`: a `runtime` that names a retired runtime provider is rejected. `templates.yaml`'s comment: a template gives no `runtime`, as it gives no placement. Schemas README, the row for PRV-006 |
| 28 (R19) | PRV-015, LIF-009 | `register.v1` and `register-replacement.v1`: where the machines provider cannot place an instance now, the request reports unavailable; rejected for want of a machine only where no location matches |
| 22, 24 with MAN-017 | MAN-017 | Rule `item-place-required` also rejects a place at which the schema admits an object or an array, unless `secret-reference.v1` governs it, and one that admits null for an input that the Package or the Remote marks `isRequired: true`; two examples |
| 29 (R20 second half) | IFC-036, IFC-037 | README, Conduit's keys: the extension is declared on provider calls too, the engine's server lists it, and a request with Conduit's keys that does not declare it reports invalid |
| 31 (R22) | OPS-032, OPS-034 | The rules comment of `operations.yaml` and the README cite OPS-034 for a repeat of an invocation whose outputs are no longer kept |
| 32 (R23) | LIF-013 | `inspect.v1`: `status.unreconciled`, a list of IDs, and its description. README |
| 35 (R27) | MAN-010 | `implementation-item.v1`: `arguments` names a named argument by `name`, a positional one by `valueHint`, never one that states a fixed `value`. `manifest.yaml`'s comment |
| D6 | ACC-011 | `close.v1`: a request through a closed Conduit reports refused. README, Results: both cases, refused through it and invalid naming it |
| D8 | PRV-023 | README, Providers: what the engine's own runner passes |
| D10 | SEC-022 | `secret-reveal.v1`: a Type that offers it offers no other capability, and a store that lists or writes secrets does so through another Type. New rule `secret-only` (SEC-022) and its example. `contracts.yaml`'s comment; both READMEs |
| D11 | LIF-036, PRV-021 | `unregister.v1`: the stop asked once more, and the instance removed whatever comes of it. `stop.v1`: carried out once the provider can; asked once more at an unregister. READMEs' tables |
| D12 | SEC-023 | `reveal-secret.v1`: the read again, for the caller of the setup that made a connection whose values the engine lost, through the reading instance's own Conduit as it stands then. README, Providers |
| D17 | PRV-022, DL13 | `run.v1`: the new input `identity`, `{environment: {name: string}, files: {path: string}}`; delivered and kept as `secrets` are; no record holds it. README: what `identity` holds is DL13's |

No change here was needed for rows 2, 4, 7 (D8 supersedes it), 8, 14, 21, 25, 30, 34, 37 to 44:
their promises are in other groups' files, and no contract, schema, example or rule of these files
states what they change. Rule `provider-conduit` already matches the operations of row 37 by
their qualified references in the core package.

## New names and IDs

No promise ID was assigned here. These files cite the IDs assigned in advance: OPS-034,
LIF-036, SEC-022, SEC-023, PRV-021, PRV-022 and PRV-023, each of which exists in its file now.

New in these files:

- The input `identity` of `run.v1`, with its members `environment` and `files`, is a coined
  name, as the work order gave it, for the owner to strike. So are `serving` and `unreconciled`,
  which the coordinator's record already named working names.
- The rules `plain-provider` (PKG-020) and `secret-only` (SEC-022) of `tools/check_schemas.py`.
- The examples `PLC-025-two-items-name-one-label.yaml`,
  `PKG-020-manifest-plain-runtime-provider.yaml`,
  `SEC-022-secret-store-type-that-offers-another-capability.yaml`,
  `MAN-017-environment-place-admits-an-array.yaml` and
  `MAN-017-required-argument-place-admits-null.yaml` under `examples/invalid-by-rule/`.

## Choices beyond the record

1. `identity` is optional, since the engine gives it only where its mechanism gives a run
   something. The keys of `files` are non-empty strings, and their form, absolute or relative,
   is DL13's. A file's content is a string: a credential that is not text would need an
   encoding, which is not fixed.
2. `run.v1`'s `secrets` and `identity` state no `additionalProperties: false`, as the record
   wrote `secrets` and as `location` is written.
3. `unregister.v1` keeps `deduplicates: true`, although it now asks for a stop: the instance is
   removed whatever comes of the stop and the outcome is the removal's (LIF-036), and a new
   attempt only asks for the stop again, which succeeds where nothing runs. The rules comment
   says so. Its `effect` names no stop. The alternative is to drop the mark, so that a repeat
   after an unknown outcome reports unknown under OPS-012.
4. The descriptions cite no promise ID, as before, because a description becomes a tool's
   (IFC-033). The IDs are in the comments and the READMEs.
5. Some contracts changed that the record agent's list for these files did not name, because a
   row of the table changes what their descriptions state: `register.v1` and
   `register-replacement.v1` (rows 27 and 28), `place.v1` (row 1), `location.v1` (row 13),
   `bound.v1` and `install.v1` (rows 6, 10 and 36), `set-up.v1` and `retire.v1` (rows 3, 9, 11
   and 33), `retire-hook.v1` (rows 3 and 9), `close.v1` (D6), `unregister.v1` and `stop.v1`
   (D11), `reveal-secret.v1` (D12) and `secret-reveal.v1` (D10).
6. `retire-with-resources.v1` now names `runtime` among the relations of the order, as LIF-021
   does; its description had left it out.
7. The record named the PKG-020 example `PKG-020-plain-provider.yaml`. The rule is on
   manifests, and the checker reads a file under `examples/invalid-by-rule/` as a manifest only
   where its name holds `-manifest-`, so it is `PKG-020-manifest-plain-runtime-provider.yaml`.
8. The rules for PKG-020 and SEC-022 are the tool's and not the schemas'. The profile could
   state each with `if` and `contains`; `type-spec.v1` and `implementation-item.v1` keep out
   the names of the core package's capabilities and operations, as they do for PRV-001.
9. To show two selected items that name one label with different values, the valid example's
   `legacy-reader` in `files-package/conduit.yaml` was given `placement:
   {kubernetes.io/os: windows}`. No valid template selects it, so no valid example breaks.
10. The MAN-017 rule takes both rows 22 and 24, though the record marked row 22's check optional
    and named no check for row 24. It follows the branches of `anyOf` and `oneOf` at the place,
    not `allOf` and not a `$ref`, and reads `type`, `enum` and `const` alone, as MAN-017 does. It
    matches `isRequired` through the server.json input that the binding's key names: a variable
    or a header by `name`, an argument by `name` or `valueHint`. A `command` item declares no
    input, so it is not checked for null.
11. No tool rule was added for row 27's `default` of an input whose destination is an entry's
    `runtime` (PLC-005), nor for row 35's keys of `arguments` (MAN-010): only the comments say
    that the engine checks them.
12. `spec/core/README.md`'s list of what the package does not settle no longer has the keys of
    `secrets`, which rows 22 and 23 settle, and has instead what the engine puts in `identity`,
    which DL13 states.

## Open

1. How a file of `identity` that the runtime provider refreshes in place gets its new content.
   PRV-022 lets the provider replace it, and no call of the engine carries a new credential to a
   provider. DL13 may state it; local enforcement in v1 will need it.
2. A name in `identity.environment` that is also in `environment` or `secrets.environment`.
   Row 23 keeps `environment` and `secrets.environment` apart; nothing says whether a name of
   `identity` may repeat one of theirs, or which value wins.
3. `unregister.v1`'s `deduplicates: true` (choice 3), for the coordinator to confirm.
4. D20, not ruled: the valid example's Pool template `workspace` still routes a `runtime` input
   to an entry whose item is `bin/files-server`, a relative path, which row 17's guard ends
   failed under any runtime provider but the engine's own runner. D5, not ruled: nothing here
   states who may reach a run's `endpoint`.

## Checks

Run from the repository root at the end of this pass.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | `439 promises ok` |
| `python3 tools/check_schemas.py` | `Errors: 0`. 101 files holding 239 documents read as strict YAML; 58 Schema contracts checked against the profile; 146 documents validated; 52 invalid documents each rejected; 102 exports and 1 implementation; 414 references checked for their kind; 955 citations of promises in 104 files, every one of a promise that exists; 22 rules from 21 promises applied, and the 26 sets under `examples/invalid-by-rule/` each rejected by the rule they are named for; 10 defaults validated |
| `python3 tools/set_names.py --check` | `Errors: 0` |
| `python3 tools/check_docs.py` | `Errors: 0`. 145 Markdown files and 1874 local links |
| `git diff --check` | Clean |
