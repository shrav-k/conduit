# Ruling 41: log of group G1 (lifecycle)

Date: 2026-11-06. This is the account of how the editor of group G1 applied the
[work order](PLAN.md#g1-lifecycle) for [ruling 40](../ruling-40/README.md) and
[ruling 41](README.md) to its five files. The rulings are the owner's. The wording is the
editor's, and stands unless the coordinator or the owner strikes it. Names are the plan's and the
review's (P1); the vocabulary audit that ruling 41 orders renames them.

Files: `spec/promises/graphs.yaml`, `spec/promises/templates.yaml`,
`spec/conventions/lifecycle.yaml`, `spec/conventions/observation.yaml`,
`spec/conventions/implicit.yaml`. No other file was touched.

| File | Before | Removed | Rewritten | Added | After |
| --- | --- | --- | --- | --- | --- |
| `graphs.yaml` | 26 | 26 | 0 | 0 | 0 (`[]`, P2) |
| `templates.yaml` | 15 | 4 | 6 | 3 | 14 |
| `lifecycle.yaml` | 34 | 1 | 24 | 1 | 34 |
| `observation.yaml` | 7 | 2 | 3 | 0 | 5 |
| `implicit.yaml` | 8 | 3 | 2 | 0 | 5 |
| G1 | 90 | 36 | 35 | 4 | 58 |

These are the plan's counts for G1 (36, 35, 4). Each file's head comment has a line for ruling
41 naming what it removed, changed and added, and pointing to this log.

## Promises

### Removed

| ID | Decision | Where its content went |
| --- | --- | --- |
| GRA-001 | 1 (R40 3c) | Nothing replaces it |
| GRA-002 | 1 | TPL-012: its parts begin in order |
| GRA-003 | 1 | OPS-022 (the outcome of a setup), CON-004 and POOL-013 (optional parts) |
| GRA-004 | 1 | Nothing replaces it |
| GRA-005 | 1 | Nothing replaces it |
| GRA-006 | 1 | Nothing replaces it |
| GRA-007 | 1 | Nothing replaces it |
| GRA-008 | 1 | Nothing replaces it |
| GRA-009 | 1 | Nothing replaces it |
| GRA-010 | 1 | Nothing replaces it |
| GRA-011 | 1 | Nothing replaces it |
| GRA-012 | 1 | Nothing replaces it |
| GRA-014 | 1 | Nothing replaces it |
| GRA-015 | 1 | Nothing replaces it |
| GRA-017 | 1 | Nothing replaces it |
| GRA-018 | 1 | Nothing replaces it |
| GRA-019 | 1 | Nothing replaces it |
| GRA-020 | 1 | Nothing replaces it |
| GRA-021 | 1 | Nothing replaces it |
| GRA-022 | 1 | CON-004 (declared Conduits) and POOL-013 (entries) |
| GRA-023 | 1 | Nothing replaces it |
| GRA-024 | 1 | Nothing replaces it |
| GRA-025 | 1 | Nothing replaces it |
| GRA-026 | 1 | Nothing replaces it |
| GRA-027 | 1 | Nothing replaces it |
| GRA-028 | 1 | Nothing replaces it |
| TPL-006 | 2 | Runtimes go; the run of PRV-007 replaces them |
| TPL-007 | 2 | PRV-007, and 1c's "a running run is left as it is" (TPL-012, LIF-027) |
| TPL-015 | 2a | `reconcile.v1` loses `handle`; a run of unknown outcome is settled by its status, PRV-009 |
| TPL-016 | 1, 2 | Nothing replaces it |
| LIF-030 | 17 | An instance whose shutdown is confirmed is removed (LIF-035), so a request that names it reports invalid under OPS-029 |
| OBS-005 | 2 | Restoring goes; a runtime provider's own restart is PRV-010 |
| OBS-007 | 2 | The stop for no actor goes; nothing replaces it |
| IMP-004 | 1 | No graphs; IMP-002 and IMP-008 cover the hooks |
| IMP-005 | 1 | As IMP-004 |
| IMP-009 | 1 | As IMP-004 |

GRA-013 and GRA-016 were removed earlier, by consolidation pass 1, and are named again in the head
comment of `graphs.yaml`.

### Rewritten

| ID | Decision | What changed |
| --- | --- | --- |
| TPL-001 | 1, 2 | `spec` keys lose `runtimes`, `setup` and `retire`, and so do the maps keyed by name; observable drops "an isolation setting" and says a template that writes those keys is rejected under OBJ-019 |
| TPL-003 | 1, R40 3c | Each value of the `implementation` map is a string; the graph in place goes. Observable: the step and runtime go from the sentence on `implementation` keys; hooks are selected here like any operation (TPL-017, TPL-018) |
| TPL-004 | 1, 2, 12 | `when` drops runtimes and steps; observable drops the step-with-`conduit` sentence and the INS-022 deferral |
| TPL-008 | 1 | `when`: an instance is set up or retired; `then`: each setup is one invocation of `set-up.v1`, each retirement one of `retire.v1`, implemented by the engine and never selected; observable drops every graph sentence and points at TPL-012 and TPL-009 |
| TPL-009 | 1, 2, 17 | `then`: where a setup was ever dispatched, the retire hook (TPL-018), then the stop of the run (PRV-008); observable drops the runtime, handle and LIF-030 sentences |
| TPL-012 | 1, GRA-002, S4.5 | Layer `language` → `convention`. `then`: the parts of a setup and their order: declared Conduits or entries, then the run and the connections, then the setup hook; each begun once every earlier part not marked optional has succeeded |
| LIF-001 | 3a | Observable: narrowing is closing a Conduit and opening a narrower one (ACC-011); the policy sentences go |
| LIF-007 | 17 | `then`: exists until removed, under LIF-035 or by `unregister.v1` where its location was declared lost (LIF-017); a request that names it afterwards reports invalid |
| LIF-008 | 1, 17 | Observable: the part of a setup for it reports conflict under CON-004 or POOL-013 |
| LIF-009 | 5, 17, P9 | `register-replacement.v1` names the holder in `instance` and the nested name in `nested`; component and composite sentences go; a removed instance is reported missing (LIF-019); PLC-004 → PRV-015 |
| LIF-010 | 8e | `when`: the machines provider reports the machine unreachable (PLC-023); DL5 → PLC-023 |
| LIF-011 | 24b | `then`: conditions in `conditions`, in the shape of `metav1.Condition` with `lastHeartbeatTime` and `origin` added; observable: "whose run is reported stopped" (PRV-009) |
| LIF-013 | 24b, P19 | Observable: the status holds `conditions`; `records` is a list of `record.v1` (REC-002) |
| LIF-015 | 2, 17 | Observable: the list of requests a retired instance takes drops LIF-030, TPL-015 and a plain "unregister it"; unregistering is only where its location was declared lost |
| LIF-017 | 17 | `then`: dispatched only where the instance is retired and its location was declared lost, and it removes the instance; otherwise conflict. Observable: a confirmed shutdown has removed the instance already (LIF-035) |
| LIF-018 | 2, 17 | Runtime sentences go; a reconciled retirement removes the instance (LIF-035); the lost-location case points at LIF-017 |
| LIF-019 | 17 | `when`: "unregistered or removed" |
| LIF-020 | 17 | `when` drops the unregistering of a Pool |
| LIF-021 | 1, 17, 23 | Observable: an onward request of a retire hook replaces the `retire` graph step; the LIF-030 sentence goes; a busy Resource reports conflict under REV-014 |
| LIF-022 | 10 | Renaming goes, and so do the two Conduits of `root` |
| LIF-023 | 17, 8e | `then`: retired only (unregistering goes); observable adds the machine reported gone (PLC-024) and says removal is LIF-035 and not a retirement |
| LIF-024 | 2, 17 | `then`: the runtime conditions go; unknown also where the machine was reported gone (PLC-024). Observable: runtime, start, stop and handle sentences go; a confirmed shutdown removes the instance |
| LIF-025 | 1, 2 | `then`: no retire hook and no stop |
| LIF-026 | 1, 1c | Observable: the hook is dispatched again only as OPS-021 and OPS-028 allow, the stop is asked for again (PRV-008); an onward request of the hook replaces the step; LIF-030 → invalid under OPS-029 |
| LIF-027 | 1, 2 | A run that the setup asked for is not stopped; a later setup leaves a running run as it is (PRV-007) |
| LIF-028 | 2, 5, 17 | Runtime clauses go; a reconciled retirement removes the instance; the composite sentence goes |
| LIF-029 | 5 | Observable drops the component |
| LIF-031 | 1, 3a, 6c | Observable: an onward request alone; the configuration received in `_meta` (IFC-006) replaces the value reference; the retrieval and policy sentences go |
| LIF-033 | 5, P9 | `when`: names, in `instance` and `nested`, a nested instantiation that the instance does not hold, or a first-start object; the composite sentences go |
| LIF-034 | 10 | Renaming and the two Conduits of `root` go |
| OBS-002 | 24b | Observable: an observation is a condition, its time in `lastHeartbeatTime` and its origin in `origin` |
| OBS-003 | 8e, 24b | `when`: the machine reported unreachable (PLC-023); `then`: a condition whose `status` is `Unknown`, with `lastHeartbeatTime` |
| OBS-004 | 2b | `then` drops the restoring exception; observable: a runtime provider's restart (PRV-010) is the provider's, and the engine dispatches nothing because of it |
| IMP-002 | 1, 2, 3a, 8e | `when` drops policies, restoring and "a location is reached again"; adds the hooks, a provider's restart and "a machine is reported reachable again" |
| IMP-008 | 1 | Observable: a setup or retire hook replaces the step of a graph |

### Added

| ID | Layer, level | Decision | Rule |
| --- | --- | --- | --- |
| TPL-017 | convention, must | 1a, P20 | Where a template selects an implementation for `setup-hook.v1`, the engine dispatches it as the last part of each setup, through no Conduit, for the caller of the setup, with OPS-028's identity |
| TPL-018 | convention, must | 1a, 4c | Where a template selects an implementation for `retire-hook.v1`, the engine dispatches it within a retirement of an instance that was ever set up, before the stop, asking for the run first where it is not running |
| TPL-019 | language, must | 2a, P3 | A template whose selected implementations include two different items reached by `command` or `package` is rejected |
| LIF-035 | convention, must | 17 | Once a retired instance's shutdown is confirmed, it is removed and no longer exists, its records kept; a Conduit is so removed when it is closed |

## Choices

| Where | Choice |
| --- | --- |
| `graphs.yaml` | The first two lines of the head comment now say that the file holds no promise and why it is kept; the history below them is kept as it was. |
| `templates.yaml` head | The sentence on runtimes as pairs of operations that pass a handle is replaced by one on the hooks, citing the standards decision 1 names (Kubernetes PostStart and PreStop, Compose `post_start` and `pre_stop`). |
| `lifecycle.yaml` head | "retired and unregistered" → "retired and removed" in its first line. The section comment "Retirement and unregistration" is kept as a record. |
| TPL-003 | Of the observable sentence "A step, a runtime or a member of a template's `controls` map that carries its own `implementation` key is rejected", only the step and the runtime are cut; the `controls` case stays. Beside the plan's hook sentence, "`set-up.v1` and `retire.v1` are not [selected here]: the engine implements them, as TPL-008 states" is kept from the old sentence. |
| TPL-008 | The plan's "the one invocation of `set-up.v1`" is written "each setup of it is one invocation", since an instance can be set up, and retired, more than once. The Conduit sentence points at LIF-032. |
| TPL-009, TPL-018 | The stop is asked for "where an item that is run is selected for it", as PRV-007 to PRV-009 say it; with no hook and no such item the retirement dispatches nothing and succeeds at once, as the old text said for no graph and no runtime. The observable says the stop is asked for whether or not the run is reported running, which is why the old rules on a start of unknown outcome have no successor. |
| TPL-012 | "asking for its run, under PRV-007, or connecting to its `remote` item" is written "asking for its run ... and connecting to each `remote` item among its selected implementations": TPL-019 limits the items that are run to one, but not the `remote` items, so an instance can have both. The order: a part is begun only once every part before it in the order, not marked optional, has succeeded; among entries, POOL-013's order. "Nothing else is dispatched" excepts the reads of SEC-010 and SEC-013, since a read is a request (P5). |
| TPL-017, TPL-018 | "Selects an implementation" covers a selection by default under TPL-004 as well as one in the `implementation` map. Model work: the hook starts none under IMP-001 and its onward requests none under IMP-008. |
| TPL-018 | Where the run cannot be had, as where a read fails, the hook is not dispatched and reports unavailable, as SEC-012 says for any request that was to go to that implementation. |
| TPL-019 | `then` states the rule ("an instance has at most one item that is run"); the observable says what an item that is run is, and that one item implementing several operations is one run. |
| LIF-008 | "has only the retired instance" → "has no instance that is not retired", since the retired one may have been removed (17); this is CON-004's and POOL-013's wording. |
| LIF-009 | `when` adds "or has been removed", the reason P9 gives. "Two instances" → a later one and the earlier one until it is removed; "has been unregistered, under OBJ-009" → "no longer exists"; the CON-003 citation goes with CON-003, and naming is OBJ-003's alone. |
| LIF-011 | Cites the core schema `condition.v1` (plan section 4). |
| LIF-015 | "would store settings" → "would store ... the values the control was sent", following REV-010 under decision 15. |
| LIF-017 | The observable says that a location whose machine the machines provider reported gone is declared lost in this sense, under PLC-024, which is the plan's term "declare lost" (8e). It points at LIF-012 for a template. |
| LIF-018 | The observable says that the actor tells whether the run was stopped from the condition `Running` (PRV-009), which replaces the route through TPL-015. |
| LIF-021 | The LIF-030 sentence becomes "A Resource whose shutdown is confirmed has been removed, under LIF-035, and gets no invocation of `retire.v1`". The REV-014 sentence says only that the retirement of a busy Resource reports conflict, so the Pool is not retired by the request; see "Still needs a decision". |
| LIF-024 | "unless an earlier one succeeded" goes: an instance whose retirement succeeded has been removed. |
| LIF-026 | "only as OPS-021 allows" → "only as OPS-021 and OPS-028 allow": OPS-028's identity is what makes the repeat. |
| LIF-027 | "A run that it asked for is stopped" is added to `then` as well as to the observable, so that the promise states its rule in `then`. |
| LIF-028 | "which is pending unless a start of unknown outcome keeps it unknown" goes; it points at LIF-024 alone. |
| LIF-033 | The plan's "or names a first-start object" is read as naming one in `instance`. |
| LIF-035 | The observable lists what follows from removal: recorded under REC-001, the name free again under OBJ-002, holders report it missing under LIF-019, and pending or unknown shutdowns not removed. |
| OBS-002 | The `then` is kept; the condition fields are in the observable, as the plan says. |
| OBS-004 | The observable calls a provider's restart "the provider's own setting", so the `must not` binds the engine and not the provider. |
| IMP-002 | Exactly the plan's changes; "removed" is not added to the list of events. |

Terms used as section 1 of the plan defines them, for the definitions: part, setup hook, retire
hook, run, runtime provider, machines provider, remove, declare lost (with a machine reported
gone), condition, provider call. One phrase is used throughout and should get a row or a mention
under "run": **an item that is run**, an implementation item reached by `command` or `package`
(TPL-019, PRV-007).

## For the integrator

What cites the IDs removed here, outside their own files, as found when G1 finished. Other groups
were editing at the same time, so some of these may already be gone; the plan's lists
([graphs](PLAN.md#specpromisesgraphsyaml), [templates](PLAN.md#specpromisestemplatesyaml),
[lifecycle](PLAN.md#specconventionslifecycleyaml),
[observation](PLAN.md#specconventionsobservationyaml),
[implicit](PLAN.md#specconventionsimplicityaml)) name the promises that their groups rewrite.

| Removed | Still cited by |
| --- | --- |
| GRA-001, GRA-002, GRA-012, GRA-025, GRA-027 | `spec/definitions.md` |
| GRA-003 | `spec/definitions.md`, `spec/promises/conduits.yaml` |
| GRA-004, GRA-006, GRA-008, GRA-010, GRA-011 | `spec/schemas/graphs.yaml`, `spec/schemas/README.md` |
| GRA-005, GRA-028 | `spec/schemas/graphs.yaml`, `spec/schemas/README.md`, `tools/check_schemas.py` (GRA-028 also `spec/definitions.md`) |
| GRA-007 | `spec/schemas/graphs.yaml` |
| GRA-014 | `spec/definitions.md`, `spec/promises/manifest.yaml`, `spec/schemas/graphs.yaml`, `spec/schemas/manifest.yaml`, `spec/schemas/README.md` |
| GRA-015, GRA-018 | `spec/definitions.md`, `spec/schemas/graphs.yaml` |
| GRA-017 | `spec/core/README.md`, `spec/definitions.md`, `spec/schemas/graphs.yaml`, `spec/schemas/README.md`, `spec/schemas/examples/valid/project/templates.yaml` |
| GRA-019 | `spec/core/README.md`, `spec/definitions.md`, `spec/schemas/graphs.yaml`, `spec/schemas/examples/invalid/CTR-010-constituent-with-bare-value.yaml` |
| GRA-020, GRA-021 | `spec/core/README.md`, `spec/definitions.md`, `spec/schemas/graphs.yaml`, `spec/schemas/README.md` |
| GRA-022 | `docs/OVERVIEW.md`, `spec/definitions.md`, `spec/promises/conduits.yaml`, `spec/schemas/graphs.yaml`, `spec/schemas/README.md` |
| GRA-023 | `spec/core/README.md`, `spec/definitions.md`, `spec/schemas/graphs.yaml` |
| GRA-024 | `spec/core/README.md`, `spec/definitions.md`, `spec/schemas/graphs.yaml`, `spec/schemas/README.md`, `tools/check_schemas.py`, `spec/schemas/examples/invalid-by-rule/GRA-005-graph-invokes-its-own-operation.yaml` |
| GRA-026 | `spec/definitions.md`, `spec/schemas/examples/invalid-by-rule/GRA-028-output-of-a-runtime-step.yaml` |
| TPL-006 | `spec/core/README.md`, `spec/definitions.md`, `spec/schemas/contracts.yaml`, `spec/schemas/templates.yaml`, `spec/schemas/README.md`, `spec/schemas/examples/valid/files-package/contracts.yaml`, `spec/schemas/examples/valid/project/templates.yaml`, `tools/check_schemas.py` |
| TPL-007 | `spec/definitions.md`, `spec/schemas/examples/invalid-by-rule/GRA-028-output-of-a-runtime-step.yaml` |
| TPL-015 | `spec/core/operations.yaml`, `spec/core/README.md`, `spec/definitions.md`, `spec/schemas/README.md` |
| TPL-016 | `spec/definitions.md`, `spec/schemas/graphs.yaml`, `spec/schemas/README.md`, `tools/check_schemas.py` |
| LIF-030 | `spec/core/README.md`, `spec/definitions.md` |
| OBS-005 | `docs/OVERVIEW.md`, `spec/definitions.md`, `spec/schemas/examples/valid/files-package/contracts.yaml` |
| OBS-007 | `docs/OVERVIEW.md`, `spec/core/README.md`, `spec/definitions.md`, `spec/interface/implementation.yaml` (IFC-006) |
| IMP-004 | `spec/schemas/graphs.yaml`, `spec/schemas/README.md` |
| IMP-005, IMP-009 | `spec/definitions.md`, `spec/schemas/graphs.yaml`, `spec/schemas/README.md` |

Also for the integrator:

- `spec/promises/graphs.yaml` is `[]`. The validation script of
  [CONSOLIDATION.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#validation) reads it without error. Any tool that
  expects a promise file to hold at least one promise, or that reads GRA IDs from it, needs a look.
- TPL-012 changed layer, from `language` to `convention`. Counts by layer in `spec/README.md`
  move with it.
- LIF-014, unchanged as the plan says, still lists confirmed among the values of a retired
  instance's shutdown. Under LIF-035 an instance is removed once its shutdown is confirmed, so
  that value is seen in the result and the record of the request that confirmed it, and no
  later inspection shows it. The definitions' rows for shutdown, exists, retired and unregister
  should say so.
- Definitions rows that cite what G1 removed or changed: exists, retired, set up, retire,
  shutdown, unregister, setup operation, dispatch ("or of a step of a graph"), caller (OBS-007's
  request with no caller), condition and observation (24b), type (`restoresRuntimes`), nested
  instantiation and nested instance (components), and the section "Graphs".

## Still needs a decision

1. **The stop after a failed retire hook.** TPL-009 asks for the stop after the hook in every
   case, as the plan's "and then" and G2's PRV-008 ("the stop comes after the retire hook") read,
   and as Kubernetes kills a container whose PreStop hook failed. The setup's parts, by contrast,
   wait for each other (TPL-012). Whether the stop also waits for the hook to succeed is not
   decided by the review; the text leaves the stop unconditional.
2. **A further setup after the hook's outputs have lapsed.** OPS-028 gives a hook the same
   operation identity in every setup, so that a further setup's hook is a repeat that OPS-009
   answers. P8 (OPS-032) makes a repeat of an invocation that succeeded, and whose outputs the
   engine no longer keeps under DL10, report conflict. A hook has no outputs. Unless OPS-032
   excepts an invocation that has none, or OPS-022 counts such a hook as succeeded from before,
   a further setup made after DL10's time would report conflict for its hook part and not
   succeed. This is G4's promise and the coordinator's call.
3. **A busy Resource in `retire-with-resources.v1`.** LIF-021 says that where a Resource's setup,
   retirement or change is in progress, the retirement of it that the request would dispatch
   reports conflict under REV-014, so the Pool is not retired by that request. Whether the
   request goes on to retire the other Resources, or stops as it does after a retirement that
   ended without its shutdown confirmed, is not decided.
4. **A hook required by a Type, or given a default.** TPL-017 and TPL-018 apply whenever an
   implementation is selected for a hook, which under TPL-004 could be a default where a Type
   requires the hook. Plan section 4 has the checker allow no default for hook operations.
   Whether a Type may require a hook, or state a default for one, is for G6 and the core editor.
5. **Replacing one of the two Conduits of `root`.** Under the plan's LIF-033, a
   `register-replacement.v1` that names `root` in `instance` reports invalid; before, a request
   for one of its two Conduits reported conflict under LIF-029, since neither can be closed
   (ACC-024). No case where the difference matters was found; it is listed so the coordinator
   can confirm the reading.

## Validation

- The validation script of [CONSOLIDATION.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#validation), run over
  the whole of `spec/` while other groups were editing: "424 promises ok".
- On G1's files alone: every promise has exactly the seven fields, a valid layer and level and
  `vectors: []`; no line is over 96 characters; no line has trailing whitespace; `git diff
  --check` is clean; and no promise text cites an ID removed by the plan.
- `python3 tools/set_names.py --check` reports three errors, none in G1's files: the key
  `withheld`, which G5 removed, is still named in `spec/core/README.md` and
  `spec/definitions.md`.
