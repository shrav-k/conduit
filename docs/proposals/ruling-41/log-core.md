# Ruling 41: log of the core package and the schemas

Written by the core-and-schemas editor on 2026-11-06, applying section 4, "Core package and
schemas", of the [work order](PLAN.md) for [ruling 40](../ruling-40/README.md) and
[ruling 41](README.md), after groups G1 to G7 and the [integration](log-integration.md) of
the promise files. Scope: everything under `spec/core/` and `spec/schemas/`, with the examples,
and `tools/check_schemas.py`; `spec/names.yaml` needed no change. One row of `spec/README.md`,
the docs editor's file, was removed (see [Outside the scope](#outside-the-scope)). The names
are those of the review and the plan, such as `secret-reveal.v1` and `reveal-secret.v1`: the
vocabulary audit that ruling 41 orders proposes others before pull request 7 is finished.

## Checks

Run from the repository root at the end of this pass.

| Check | Result |
| --- | --- |
| `python3 tools/check_schemas.py` | `Errors: 0`. 95 files holding 228 documents read as strict YAML; 58 Schema contracts checked against the profile (41 keywords, 6 formats); 146 documents validated; 52 invalid documents in 52 files each rejected; 102 exports and 1 implementation; 413 references checked for their kind, 2 naming packages not in this repository; 1 control; 919 citations of promises in 98 files, every one of a promise that exists; 19 rules from 18 promises applied, and the 20 sets under `examples/invalid-by-rule/` each rejected by the rule they are named for; 10 defaults validated |
| `python3 tools/set_names.py --check` | `Errors: 1`, in `spec/definitions.md` (the removed key `withheld`), the docs editor's file. The two such errors that were in `spec/core/README.md` are gone |
| `python3 tools/check_docs.py` | `Errors: 0` |
| `git diff --check` | Clean |

## Counts

| | Before | Removed | Added | After |
| --- | --- | --- | --- | --- |
| Exports of the core package | 98 | 16 | 20 | 102 |
| Operation contracts | 20 | 6 | 9 | 23 |
| Capability contracts | 5 | 2 | 4 | 7 |
| Type contracts | 6 | 1 | 0 | 5 |
| Requirement contracts | 2 | 0 | 0 | 2 |
| Schema contracts | 58 | 7 | 7 | 58 |
| of which reference schemas (`references.yaml`) | 11 | 0 | 1 | 12 |
| of which in `results.yaml` | 2 | 0 | 3 | 5 |
| of which under `spec/schemas/` | 45 in 9 files | 7, and `graphs.yaml` | 3 | 41 in 8 files |
| Templates | 7 | 0 | 0 | 7 |
| Implementations (`engine`) | 1 | 0 | 0 | 1 |
| `examples/invalid/` | 63 files | 19 | 8 | 52 files |
| `examples/invalid-by-rule/` | 24 files | 12 | 8 | 20 files |
| `examples/valid/` | 7 files in 3 directories | 0 | 0 | 7 files in 3 directories |
| Rules of `tools/check_schemas.py` | 23 | 12 | 8 | 19, from 18 promises |

The counts after are those the plan gives `spec/README.md`.

## The core package

Exports removed (16), as the plan lists: `rename.v1`, `retrieve-result.v1`, `apply-policy.v1`,
`remove-policy.v1`, `set-default-location.v1`, `set-facts.v1`; `system-settings.v1`,
`location-settings.v1`; `system-pool.v1`; `graph.v1`, `lifecycle-graph.v1`, `step.v1`,
`value-binding.v1`, `value-reference.v1`, `runtime.v1`, `conduit-component.v1`.
`spec/schemas/graphs.yaml` was deleted from the tree (not with Git).

Exports added (20): `setup-hook.v1`, `retire-hook.v1`, `run.v1`, `stop.v1`, `inspect-run.v1`,
`list-machines.v1`, `place.v1`, `fetch-package.v1`, `reveal-secret.v1`; `runtime-provider.v1`,
`machine-provider.v1`, `package-source.v1`, `secret-reveal.v1`; `handle.v1`, `condition.v1`,
`record.v1` (`results.yaml`), `secret-reference.v1` (`references.yaml`), `label-selector.v1`
(`values.yaml`), `server-package.v1` and `server-remote.v1` (`manifest.yaml`).

Changed, by file:

| File | What changed |
| --- | --- |
| `operations.yaml` | Every contract in `inputSchema` and `outputSchema` form; an input that was `optional: true` is left out of `required`, and an output that was not optional is listed there. `register-replacement.v1` takes `instance` and `nested` (P9). `reconcile.v1` loses `handle` and `settings`. `inspect.v1`: `packages` has two forms, the core package's and any other's, whose `version` may be left out for a submitted project (NAM-009); `status` holds `conditions`, a list of `condition.v1`, and no `policies`; `records` is a list of `record.v1`. `install.v1`'s `fetch` is `format: uri-reference`. New descriptions for `set-up.v1` (TPL-012), `retire.v1` (TPL-009, LIF-035), `unregister.v1` (LIF-017), `register.v1` (PKG-006, MAN-015, MAN-016), `install.v1` (PKG-023, PKG-028, PKG-029, PRV-017), `purge.v1` (OPS-008, REC-004), `open.v1` and `close.v1` (ACC-006, ACC-012, SEC-008, LIF-035), `declare-lost.v1` (PLC-024) and `retire-with-resources.v1` (REV-014, LIF-035). The nine new operations as the plan gives them |
| `capabilities.yaml` | `object-management.v1` without `rename.v1`; `access-control.v1` without the policy operations; the four provider capabilities |
| `types.yaml` | `system-pool.v1` gone; `location.v1` with an empty, closed schema and no `offers`, `requires` or `controls`; no type writes `placement: {}`; `requires` lists without the removed operations |
| `requirements.yaml` | No `appliesTo` and no `narrows`; `bound.v1` applied by the engine, its `operations` reaching a submitted project's `address#name`; `shared-host.v1` by the `kubernetes.io/hostname` labels, decided once; its `spec` is `{}` |
| `results.yaml` | `diagnostic.v1` as SARIF's `result` reduced (P16); `object.v1`'s `nested`; the three new schemas |
| `references.yaml` | `secret-reference.v1`: the kind Resource, no bare name, a Pool's entry and an entry reference, and the resolved form (SEC-001) |
| `templates.yaml` | `system` of `pool.v1`; `location` with no input and no `config`; `root-management` and `root-access` without the removed operations |
| `conduit.yaml` | The exports; `engine` implements the 14 management verbs, the operations of `object-management.v1`, `access-control.v1` and `pool-management.v1`, and states no `supports` |
| `README.md` | Rewritten: head paragraph for rulings 40 and 41, the file table, two tables of operations, the reading notes without retrieval, policies, runtimes and graphs, results and Conduit's keys as an extension, a section on providers, first start, the table of every export and what is not settled |

## The schemas

| Schema | What changed |
| --- | --- |
| `schema.v1` | The 41 keywords of the profile and the format `uri-reference`; a `default` is accepted on a member of `properties` that is not inside `allOf`, `anyOf`, `oneOf`, `not`, `if`, `then`, `else` or `dependentSchemas` (SCH-008) |
| `type-spec.v1` | No `runtimes` or `restoresRuntimes`; `placement` optional, `label-selector.v1`; a Pool type's `offers` contains `pool-management.v1`, with `if`, `then` and `contains` |
| `operation-spec.v1` | `inputSchema`, `outputSchema` (root of `type: object`, `properties`, `required`, `additionalProperties`, CTR-012), `idempotentHint`, `destructiveHint`, `openWorldHint`; a property name is a non-empty string; a handle states no `default`; no property of `outputSchema` states one |
| `requirement-spec.v1` | `inputs` alone, keyed by `name.v1` |
| `applied-requirement.v1` | Description: applied to a Conduit alone |
| `template-spec.v1`, `pool-template-spec.v1` | No `runtimes`, `setup` or `retire`; named maps keyed with `propertyNames`; a Pool input may reach `/entries/<name>/runtime` |
| `conduit-template-spec.v1` | One shape: `capability` required, and `operations` or `inputs`; no `components` |
| `implementation-map.v1`, `template-controls.v1` | Values are selections only; keys are selections, stated with `propertyNames` |
| `instance.v1` | `runtime` on an Agent and a Resource |
| `manifest.v1` | One form for a project or a package, with `version` only beside `address` (`dependentRequired`), and the core package's form; `exports` items are export names; `fetch` a `uri-reference` with no user information; keys of `packages` and `implementations` by `propertyNames` |
| `implementation-item.v1` | Exactly one of `command`, `package`, `remote` (`oneOf`); `digest` with `package` and only with it, `arguments` with `package`, `headers` with `remote`, `environment` not with `remote`; `plain: true` requires `tools` and forbids `supports`, and an item that is not plain has no `tools` |
| `values.yaml` | `revision.v1` an opaque non-empty string (C21); `label-selector.v1` |
| `envelope.yaml`, `document.yaml` | No schema changed; the head comment says so |
| `README.md` | Rewritten as the plan asks: head paragraph, file table, closed and open places, "What a schema cannot express" without the rows that decision 20 moved into the schemas and every graph, runtime, composite, constituent, re-export and `obliges` row, and with the new rules; "Choices the schemas encode"; the examples |

## Examples

As the plan lists, with these differences:

- `examples/invalid-by-rule/MAN-015-manifest-implementation-without-address.yaml` in place of
  the plan's `MAN-015-manifest-project-contracts-without-address`: a file with `-manifest-` in
  its name holds one manifest and no documents, so it can show the case of a manifest that
  lists an implementation, and not that of a project that holds contracts; the second case is
  checked on the valid examples.
- `CTR-009-reserved-mark-outside-the-core-package` and
  `SCH-015-ref-to-a-name-that-is-not-a-schema` in `invalid-by-rule/` were rewritten to
  `inputSchema` form too, and
  `NAM-021-manifest-states-the-core-address`, `NAM-027-two-documents-one-name`,
  `NAM-028-selection-that-names-nothing`, `CON-016-no-input-fills-operations` and
  `invalid/CON-016-template-without-operations` lost words or keys that are gone (`url`,
  `graph`, `placement: {}`, a composite, a deferred check).
- `valid/files-package`: the plan's "the graph item becomes a `package` item" makes `listing` a
  server.json Package of `npm` that implements `directory.list.v1`. So that an instance of
  `local-directory.v1` has one item that is run (TPL-019), the type's `implementation` map now
  selects `files-server` for `directory.list.v1`, ahead of the contract's own default
  `#listing`; the example `TPL-019-two-items-that-are-run` selects `listing` and is rejected.
  The watcher operations keep their handles as ordinary operations, outside any capability.
  `example.org/common` stays listed, used by a `$ref` in `directory.establish.v1`, so that the
  manifest still shows `fetch`. Two hints show decision 16b.
- `valid/project`: the manifest states `address: example.org/review-desk`. The runtime
  provider's template is `docker-context` and its instance `docker`, since NAM-027 forbids a
  template and an instance of one project to share a name. The Pool `workspace` gains an input
  whose destination is `/entries/checkout/runtime` (P14). The bound of `team-access` names an
  operation of the project as `example.org/review-desk#mirror.sync.v1` (ACC-029). The optional
  declared Conduit `notes` of `reviewer` takes the place of the composite's use of `#name` in
  `from`.

## Checker rules

| Change | Rules |
| --- | --- |
| Removed (12) | `location-placement`, `constituent-input`, `pool-offers`, `runtime-once`, `applies-to`, `narrows-on-conduit`, `obliges-narrows`, `bound-constituent`, `runtime-step-output`, `graph-invokes-itself`, `runtime-operation-step`, `conduit-names-one` |
| Changed (4) | `implementation-selected` (TPL-004): no runtimes or steps, and the selection it finds for each operation is kept for the rules below; `names-nothing` (NAM-028): no re-export; `default-valid` (SCH-019): reads an Operation's defaults in `inputSchema` and `outputSchema`, and walks every applicator of the profile; `core-address` (NAM-021): unchanged in what it checks, beside the new rule on manifests |
| Added (8) | `first-start-from` (FST-006), `one-run` (TPL-019), `label-conflict` (PLC-025), `item-place-required` (MAN-017), `project-address` (MAN-015), `secret-branch` (SEC-001), `provider-family` (PRV-001), `provider-conduit` (PRV-004) |

Other changes to the tool:

- Step 2 walks every applicator of the profile, and treats `not`, `if`, `then`, `else` and
  `dependentSchemas` as `allOf`, `anyOf` and `oneOf` are treated (SCH-008).
- Step 5: `RETURNS_REVISION = ("replace.v1",)`; the operations of REV-012 and the controls are
  read in `inputSchema` and `outputSchema` form; a control's `inputSchema` requires `revision`
  and nothing else (CTR-006, REV-006, IFC-033), and declares no output `settings`.
- Step 5 reads the citations of `spec/promises/manifest.yaml` from its first promise on: rule
  3 of the work order has a promise file's head comment name the IDs that rulings removed from
  it, as that one names MAN-008.
- Step 5 also looks for the prefix `GRA`, since no promise of that prefix exists any longer and
  a citation of one would otherwise go unseen.
- A directory under `examples/valid/` is a package, whose exports the others reach, where its
  manifest states both `address` and `version` (PKG-017); the documents of any directory whose
  manifest states `address` are reached by that address. A directory that holds a contract and
  has no manifest breaks MAN-015.
- The rules that follow a template's selections into an item read the items from the
  manifests of the core package and of the valid examples, and from a set's own manifest.
- The docstring follows.

## Choices beyond the plan, for the coordinator

Each is consistent with the promises as they stand; the coordinator confirms or strikes it.

1. The plan's rule `location-from` is named `first-start-from` and checks all five templates
   that FST-006 names, not `location` alone: the check is the same, and FST-006 rejects all
   five. Because of it, `examples/valid/first-start/` is exempt from that one rule: it writes
   out the documents that first start makes, which no submission may hold, and the tool names
   the directory in `FIRST_START_DIRECTORY`.
2. `object.v1`'s `nested` is `{instance, name}`, both required, where it was
   `{instance, path}` with `path` a list for components: with no component, the list had one
   name, and LIF-009 names a nested instantiation by those two. Not in the plan's list.
3. `run.v1`'s `secrets` is keyed by the JSON Pointer of the place each value was read for,
   which is how PRV-011 words the delivery ("the variable or the argument that the item binds
   to the place it was read for"). G7's question D1 is still open; the key is the one line to
   change.
4. An operation with no inputs states no `inputSchema`, and one with no outputs no
   `outputSchema`. IFC-033 gives the tool of an operation with no outputs an `outputSchema`
   that describes a map with no properties; read the same way, the tool of one with no inputs
   has the `inputSchema` `{type: object}`, which IFC-033 does not state.
5. The core operations' `inputSchema` states no `additionalProperties: false`: IFC-001 already
   makes a request's `arguments` exactly the operation's inputs.
6. `server-package.v1` and `server-remote.v1` leave out server.json's descriptions, examples and
   defaults, and are as open as server.json is. Its defaults are annotations there, and one
   stands inside `allOf`, where SCH-008 rejects a `default`. `server-remote.v1`'s `url` also
   carries `format: uri`, beside server.json's pattern, since MAN-010 requires a URI.
7. `diagnostic.v1`'s `ruleId` must have the form of a promise ID, and the schema stays open, as
   SARIF's `result` is, so that an engine may add what else SARIF holds. A diagnostic names the
   document it is about by no fixed member.
8. `record.v1` requires `specversion`, `id`, `source`, `type`, `time` and `data`; `data` holds
   REC-002's fields under the names IFC-006 gives Conduit's keys (`caller`, `instance`,
   `conduit`, `operation`, `invocation`, `operationIdentity`) with `target`, `outcome`,
   `reason`, `inputs` and `outputs`, none required, since each is present only where it
   applies. The values of `source` and `type` are left to the engine, so that no new name is
   coined before the vocabulary audit.
9. `condition.v1` keeps the patterns, lengths and `required` of Kubernetes `metav1.Condition`
   (`type`, `status`, `reason`, `message`, `lastTransitionTime`). `origin` is a non-empty
   string.
10. `label-selector.v1` gives a value the syntax of a Kubernetes label value, beside the plan's
    "values strings", since labels are Kubernetes's (C58).
11. MAN-006's "no user information" is stated in `manifest.v1` with `not` and a pattern, as
    decision 20 allows; `install.v1`'s `fetch` has the format only.
12. The keys of an item's `environment`, `arguments` and `headers`, of a control's `changes` and
    of a template's control `inputs` are non-empty strings by `propertyNames`; that a name is
    one the Package or Remote declares (MAN-010) is left to the engine.
13. The core manifest's `engine` states no `placement`, where it stated `placement: {}`; MAN-012
    still allows one.
14. `type-spec.v1` does not state PRV-001, PRV-016 or SEC-005, which `if` and `contains` could
    express: the plan gives PRV-001 to the tool, with an example that the schemas must accept,
    and gives the other two to no one. The schemas' README lists all three.
15. `secret-branch` does not follow a `$ref` to a member of `$defs` that holds
    `secret-reference.v1`; it treats `items`, a member of `properties`, `additionalProperties`
    and a member of `$defs` as the places SEC-001 allows.
16. `item-place-required` follows `properties` and `required` and stops, judging nothing, at a
    `$ref`; it takes the empty pointer, the whole configuration, as always present.

## Open questions

None of these is decided in the text; each is listed where it bears on these files.

| # | Question | Where it bears |
| --- | --- | --- |
| Q1 | The keys of `run.v1`'s `secrets` (G7's D1; integration R10): this pass keys them by pointer (choice 3) | `run.v1`, PRV-007, PRV-011, SEC-010 |
| Q2 | What a run receives for a place that holds a value that is not a string (integration R9): `run.v1` takes maps of strings | `run.v1`, MAN-010, MAN-017 |
| Q3 | Whether a runtime provider other than the engine's own runner may return no `endpoint` (integration R18); `run.v1` leaves `endpoint` optional | `run.v1`, PRV-007 |
| Q4 | Whether a Type may require a hook, or state a default implementation for one (integration R6): no core contract does, and the tool allows either | `setup-hook.v1`, `retire-hook.v1`, TPL-004 |
| Q5 | Which key of a location's status holds its labels (PLC-026), and how a diagnostic names the document it is about: both left to the conformance vectors | `inspect.v1`, `diagnostic.v1` |
| Q6 | The values of a record's `source` and `type`, which would be new names (choice 8) | `record.v1`, REC-002, DL8 |

For the vocabulary audit: every name the plan coined (P1) appears in these files, among them
`setup-hook.v1`, `retire-hook.v1`, `run.v1`, `stop.v1`, `inspect-run.v1`, `list-machines.v1`,
`place.v1`, `fetch-package.v1`, `runtime-provider.v1`, `machine-provider.v1`,
`package-source.v1`, `handle.v1`, `record.v1`, `condition.v1`, `label-selector.v1`,
`server-package.v1`, `server-remote.v1`, the input `nested`, and `secret-reveal.v1`,
`reveal-secret.v1` and `secret-reference.v1`. This pass coined no other export name, and the
rule names of the tool are its own.

## Outside the scope

- `spec/README.md`: the row of its contents table that linked `schemas/graphs.yaml` was
  removed, because the file is gone and `tools/check_docs.py` reported the link. The docs
  editor rewrites the table with the counts above.
- For the docs editor: `spec/definitions.md` still names the key `withheld` (the one error of
  `tools/set_names.py --check`) and the removed IDs that the integration log lists. Terms these
  files use: provider, provider capability, provider call, run, item that is run, runtime
  provider, machines provider, package-source provider, label, label selector, secret
  Resource, secret reference, setup hook, retire hook, handle, condition, record and Conduit's
  extension, as section 1 of the plan and the integration log give them. Counts for
  `spec/README.md`: core 102 exports (23 Operation, 7 Capability, 5 Type, 2 Requirement, 58
  Schema contracts, 7 templates); `operations.yaml` 23, `capabilities.yaml` 7, `types.yaml` 5,
  `references.yaml` 12, `results.yaml` 5; schemas 41 in 8 files (`templates.yaml` 7,
  `instances.yaml` 4, `manifest.yaml` 4, `values.yaml` 13, the others as before); examples
  `valid` 7 files in 3 directories, `invalid` 52 files, `invalid-by-rule` 20 files.
