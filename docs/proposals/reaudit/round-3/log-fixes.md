# Round 3: log of the fix pass

Written 2026-11-06 by the editor of the fix pass. The work order was every row of
[the decisions](DECISIONS.md), which are the coordinator's under ruling 37 and which the owner
has not ruled on. The findings are in the reports of [reader 1](reader-1.md),
[reader 2](reader-2.md), [reader 3](reader-3.md) and [reader 4](reader-4.md). The rules were
those of [CONSOLIDATION.md](../CONSOLIDATION.md).

Edited: files under `spec/`, `tools/check_schemas.py` and `docs/OVERVIEW.md`. Nothing else
under `docs/` was edited apart from this log. Nothing was committed or staged. No sub-reader
was started: the read-through at the end was the editor's own.

## Result

| | Before | After |
| --- | --- | --- |
| Promises | 424 | 436 |
| Of the layer `language` | 221 | 221 |
| Of the layer `convention` | 168 | 180 |
| Of the layer `interface` | 35 | 35 |
| Items of the implementation-defined list | 15 | 15 |
| Documents the core package exports | 99, and 1 implementation | 99, and 1 implementation |
| Operation contracts of the core package | 21 | 21 |
| Schema contracts under `spec/schemas/` | 45 in 9 files | 45 in 9 files |
| Files under `examples/valid` | 7 in 3 directories | 7 in 3 directories |
| Files under `examples/invalid` | 59 | 60 |
| Files under `examples/invalid-by-rule` | 17 | 20 |
| Rules the checker applies in its step 6 | 16, from 14 promises | 19, from 17 promises |

- Added, 12: ACC-041, ACC-042, ACC-043, OPS-030, LIF-031, LIF-032, PLC-022, OBJ-027, OBJ-028
  and TPL-015, of the layer `convention`; GRA-028 and CTR-011, of the layer `language`.
- Changed, 75: ACC-006, ACC-012, ACC-015, ACC-017, ACC-018, ACC-019, ACC-027, ACC-029,
  ACC-031, ACC-033, ACC-034, ACC-035, ACC-037, ACC-038, ACC-040; OPS-014, OPS-016, OPS-021,
  OPS-023, OPS-028, OPS-029; LIF-007, LIF-009, LIF-012, LIF-015, LIF-021, LIF-028; REV-010;
  REC-002, REC-004; OBS-003, OBS-007; IFC-006, IFC-021, IFC-024, IFC-025, IFC-028; CON-008,
  CON-022, CON-028, CON-030, CON-031; POOL-004, POOL-017; PLC-011, PLC-016, PLC-017; OBJ-005,
  OBJ-009, OBJ-010, OBJ-026; INS-020, INS-025; NAM-022; PKG-020, PKG-023 to PKG-027; MAN-007,
  MAN-009, MAN-010, MAN-011; SUB-008, SUB-009; TPL-003, TPL-007; GRA-005, GRA-020, GRA-026;
  CTR-002, CTR-003, CTR-005, CTR-010.
- Changed layer, 2: OBJ-005 and INS-020, from `language` to `convention` (finding 3A-15). The
  two new `language` promises leave that layer's count at 221. No promise changed level, none
  was removed, and no ID was reused.
- Per file, where the count changed: access 36, operations 30, lifecycle 32, placement 20,
  objects 28, templates 15, graphs 26, contracts 11.
- Two core operations gained an optional input: `reconcile.v1` the handle `handle`, and
  `apply-policy.v1` `version`. No export was added and no schema of a document changed shape.
- The example package gained one Operation contract, `directory.sweep-now.v1`, so the valid
  examples hold 38 documents where they held 37.
- New example files: `invalid/PKG-020-manifest-graph-marked-plain.yaml`, and under
  `invalid-by-rule/` `CTR-011-bound-as-a-constituent.yaml`,
  `GRA-028-output-of-a-runtime-step.yaml` and `GRA-005-graph-invokes-its-own-operation.yaml`.
  The checker gained the rules `bound-constituent`, `runtime-step-output` and
  `graph-invokes-itself`.

## What was changed for each finding

"Definitions" is `spec/definitions.md`, "core README" is `spec/core/README.md`, "schemas
README" is `spec/schemas/README.md`, and a name ending in `.v1` is that contract's description
in `spec/core/operations.yaml` unless another file is named.

| Finding | Promises | Other documents |
| --- | --- | --- |
| 3C-1 | ACC-019 (`then`: work that a start arranges is tied to every Conduit of that start's work), new ACC-041 (what a start carries), ACC-017, ACC-040 | Definitions (later work); `start-later-work.v1`; core README |
| 3C-2 | New TPL-015; TPL-007, OPS-014, LIF-028 point to it | `reconcile.v1` gains the input `handle`, with its description and effect; definitions (reconcile, hold); core README (table of operations, "Reconciling", "Every export"); schemas README |
| 3A-1 | OPS-023, ACC-037 | Definitions (how a request ends, access decision); core README ("Diagnostics") |
| 3B-1 | New CTR-011; CON-028, CON-030, CTR-005, ACC-029, PKG-023 cite it | Definitions (bound); `spec/core/requirements.yaml`; core README; schemas README; head of `spec/schemas/contracts.yaml`; new example and checker rule |
| 3A-2 | CON-031 (`when`: a Conduit that is not open), ACC-006 | `open.v1`; definitions (open) |
| 3A-3 | POOL-004 (rejects the document), POOL-017 (the request under LIF-009 alone, now invalid), ACC-033 | `register.v1`, `register-replacement.v1`; head of `spec/core/capabilities.yaml`; definitions (owner) |
| 3A-4 | INS-025 and NAM-022 (`then`) | Schemas README (row for `from`); `template-reference.v1` in `spec/schemas/instances.yaml`; a comment in the example project |
| 3A-5 | ACC-034, IFC-025 | Definitions (withheld); head of `spec/core/operations.yaml`; core README |
| 3A-6 | ACC-015 (`when` widened to every carried restriction), ACC-017, ACC-018 | Definitions (`supports`) |
| 3A-7 | LIF-009 (`then`), POOL-017 | `register-replacement.v1` |
| 3A-8 | ACC-033 | Definitions (`management`); `set-up.v1`, `retire.v1`; heads of `spec/core/operations.yaml` and `spec/core/capabilities.yaml`; core README |
| 3A-9 | OBJ-026, SUB-009 | `rename.v1` and `replace.v1`, each with its `effect`; definitions (rename, replace) |
| 3A-10 | CON-022 (`then`), ACC-015, ACC-018, ACC-038 | Definitions (`supports`) |
| 3A-11 | OPS-028 (`then`), OPS-021 | Definitions (attempt) |
| 3A-12, 3C-4 | IFC-024 (`then`), IFC-021 and IFC-028 (`when`), OPS-016 | `retrieve-result.v1`; definitions (caller, retained result, retrieval tool, Conduit's keys); core README |
| 3A-13 | OBS-007 (`then`), IFC-006 (`then`), REC-002 | Definitions (caller, Conduit's keys) |
| 3A-14 | See the table below | Citations in OPS-029, OBJ-009, the definitions, the core README, the schemas README and the heads of `spec/core/references.yaml`, `spec/core/operations.yaml` and `spec/schemas/instances.yaml` |
| 3A-15 | OBJ-005 and INS-020 become `convention`; OBJ-010 stays `language` and is reworded | Head comments of the two files |
| 3A-16 | — | Core README: the rows of `diagnostic.v1`, `id.v1` and `revision.v1` name promises; head of `spec/core/results.yaml`; the comment of `invalid/INS-012-destination-operations-in-a-composite.yaml` |
| 3B-2 | New LIF-031; OPS-029, OBJ-009, LIF-007 cite it | Definitions (ID, reference, resolved form, invalid); core README; schemas README; heads of `spec/core/references.yaml`, `spec/core/operations.yaml`, `spec/schemas/instances.yaml` |
| 3B-3 | — | `examples/valid/files-package/`: new contract `directory.sweep-now.v1`, listed in the capability, in the type's `requires`, in `exports` and in the `implements` of `files-server`; schemas README |
| 3B-4 | PKG-020 (`then`), MAN-009 | Schemas README (the choice is named); head and description in `spec/schemas/manifest.yaml`; definitions (plain); new invalid example |
| 3B-5 | — | Definitions (request, caller, the instance that sent a request) |
| 3B-6 | CTR-002 (`then`) | Definitions (new row `deduplicates`) |
| 3B-7 | — | Definitions (bound, definition); `spec/core/requirements.yaml`: "the package that originally exports it", "names both" |
| 3B-8 | — | Core README ("Diagnostics") |
| 3B-9 | IFC-021, ACC-027, OBS-003 | `retrieve-result.v1`; core README; the head comment of `spec/conventions/operations.yaml`; the invalid example of 3A-16; a comment in `tools/check_schemas.py` |
| 3B-10 | TPL-003 | — |
| 3B-11 | — | Schemas README (GRA-020, GRA-028, IMP-012 in the row on graphs; CON-029 beside IFC-014); head of `spec/schemas/graphs.yaml` (IMP-010, IMP-012) |
| 3B-12 | MAN-011 (`then`), MAN-007, PKG-023 | Definitions (package); schemas README; head of `spec/schemas/manifest.yaml` |
| 3B-13 | MAN-010 (`then`) | Schemas README; head of `spec/schemas/manifest.yaml` |
| 3C-3 | ACC-017 (`then`), ACC-041, ACC-031, ACC-040 ("A start never carries less" is now about filters, in ACC-041) | Definitions (onward request, later work); `start-later-work.v1`; core README |
| 3C-5 | PKG-025 and PKG-027 (`then`), PKG-024, PKG-026 | `install.v1`; definitions (install); core README |
| 3C-6 | LIF-021 (`then`) | `retire-with-resources.v1`; core README |
| 3C-7 | ACC-031 (`then`), new ACC-042 | `apply-policy.v1` gains the input `version`; `inspect.v1`; definitions (policy); core README; schemas README |
| 3C-8 | ACC-033 (`then`), REV-010 (`then`), OPS-014 | `reconcile.v1`; definitions (`management`, reconcile); core README |
| 3C-9 | New GRA-028; GRA-026 (`when` and `then`), GRA-020 | Definitions (`source`); schemas README; head of `spec/schemas/graphs.yaml`; new example and checker rule |
| 3C-10 | CTR-010 (`then`) | Definitions (constituent) |
| 3C-11 | SUB-009 (`then`), NAM-022 | `replace.v1` |
| 3C-12 | PLC-011 (`when`), PLC-017 (`then`), LIF-015 | `declare-lost.v1`; definitions (location, declare lost) |
| 3C-13 | SUB-008 (`then`) | — |
| 3C-14 | REC-004 (`then`) | `purge.v1`; definitions (purge) |
| 3C-15 | GRA-005 (`when`) | Schemas README; head of `spec/schemas/graphs.yaml`; new example and checker rule |
| 3D-2 | The head comment of every promise file this pass changed says that the pass rests on the coordinator's decisions | `spec/README.md`; the status lines of the core README and the schemas README; the first lines of the 8 YAML files under `spec/core/` and the 9 under `spec/schemas/`; the sentence above the rules in `spec/core/operations.yaml`; "Every export" in the core README; the introduction of the definitions |
| 3D-11 to 3D-17 | — | `docs/OVERVIEW.md`, listed below |
| 3D-16 | — | Definitions (how a request ends) |
| 3D-5 | Not part of this pass | — |

### The eleven rules of 3A-14

| Rule that stood in an observable | Now in the `then` of |
| --- | --- |
| LIF-007: a request names an object by a reference | New LIF-031, with the reason and the exceptions of 3B-2 |
| LIF-012: `set-up.v1` or `retire.v1` that names a template or a Conduit reports invalid | New LIF-032 |
| OBJ-026: a rename onto a name that is taken reports conflict | New OBJ-027 |
| OBJ-026: a rename to a name that breaks OBJ-001 reports invalid | New OBJ-028 |
| ACC-006: opening an open Conduit succeeds | ACC-006 |
| ACC-012: closing a closed Conduit succeeds | ACC-012 |
| OPS-014: reconciling an invocation whose outcome is not unknown reports conflict | New OPS-030 |
| CON-008: a request to a retired target reports refused | CON-008 |
| ACC-031: the causes for which `apply-policy.v1` reports invalid | New ACC-042, with the causes of 3C-7 |
| ACC-035: a policy whose `actors` names `root` reports refused | New ACC-043 |
| PLC-016: a value that names no location that exists reports invalid | New PLC-022 |
| CTR-003: a capability outside the core package that lists a reserved operation is rejected | CTR-003 |
| IFC-025: the result of an operation that declares no outputs is never withheld | IFC-025 |

The reader's table has eleven rows, and two of them hold two rules, so thirteen rules moved. A
new promise was made where the old `when` did not cover the case, or where the promise would
otherwise name two reasons. Each observable keeps a pointer.

## Choices made where a decision left the wording open

| Choice | Why |
| --- | --- |
| What a start carries is a new promise, ACC-041, and not a sentence of ACC-040. | It is an obligation of its own, and it stood only in the observable of ACC-040. |
| The policies that a step with no `conduit` carries are judged as ACC-017 judges them. | ACC-018 has such a step carry the restrictions of the invocation its graph serves, and decision 3C-3 fixes when those are judged. |
| The rule on `bound.v1` as a constituent is a new promise in `contracts.yaml`, CTR-011, and not a sentence of CON-028. | It rejects a Requirement contract, when a package is installed or a project submitted. CON-028 rejects a template and a Conduit. |
| 3A-15: OBJ-005 and INS-020 become `convention`. OBJ-010 stays `language`. | OBJ-005 says what a rename does, and INS-020 what a replace and an unregistration do: both are about requests. OBJ-010 says that a stored resolved form is not resolved again, which resolving shows. |
| 3A-8: a request for a retired Pool other than the two that move reports refused, because no target authorizes it. | The decision says such a request "is not dispatched under LIF-015". LIF-015 gives conflict to a permitted request, and none is permitted: every Conduit to the Pool is closed, and `management` does not authorize it. OPS-023 puts refused first. |
| 3A-3: registering a Resource into a retired Pool, and the request under LIF-009 for one of its entries, need the Conduit to `management` alone. | Round 2 said so in POOL-017, and without it no actor could be permitted, so the reason invalid that the decision gives could never be reported. ACC-033 now writes "a Pool that already exists and is not retired". |
| 3C-8: the rule for an unregistered instance covers an invocation of a management verb that named the instance, as well as an invocation dispatched to it. | The decision says "an invocation whose instance is unregistered". A setup of a Resource names it and is dispatched to no instance. |
| 3A-10: the rule is for a graph written in place. A `graph` item of a manifest still states `supports`, or supports nothing, under PKG-019. | The finding is about a graph that has nowhere to state `supports`, and the schema lets an item state it. |
| 3A-10: a graph none of whose steps lacks `conduit` supports every requirement. | It follows from the rule as decided, and CON-022 says so to keep two engines from reading it two ways. |
| 3A-10: inspecting an open Conduit names, for a graph written in place, the implementations selected for its steps that have no `conduit` (ACC-038). | A graph written in place has no name, and those implementations are the ones that apply the filter. |
| 3A-6: ACC-015 also covers a restriction that a start of later work carries under ACC-041. | The same check, for the third way a restriction is carried. |
| 3C-7: inspecting `access` shows the version and the digest that each policy keeps (ACC-031, `inspect.v1`). | The decision has the policy keep them, and an observable needs somewhere to see them. Where the status holds them is left with the other unsettled keys in the core README. |
| 3C-6: the order is stated as "the retirement of a Resource has ended before that of any Resource it refers to, or to which it is the consumer of a Conduit, is dispatched". | "Retired before" could mean dispatched before or ended before. A `retire` graph needs its Conduit open while it runs. |
| 3C-9: the list of what makes a source absent moved from the observable of GRA-026 to its `when`, and "a step that is not dispatched in a run returned no outputs in that run" is in its `then`. | The second rule of the decision extends that list. |
| 3C-11: the sentence that a rejected or mismatched replacing document reports invalid moved from the observable of SUB-009 to its `then`, with the two new rejections. | The promise then names its one reason in its `then`. |
| 3B-3: the new operation is named `directory.sweep-now.v1`, and returns the number of entries removed. | The reader's suggested name. |
| 3B-4: an invalid example was added for a `graph` item marked `plain`. | The schema rejects it, and the example shows that it does. |
| 3B-12: no example was added. | A set under `invalid-by-rule` holds documents only, and the rule needs a manifest that states an address beside an instance document. |
| 3D-2: the wording names the documents audit as well as rounds 2 and 3. | Its changes are the coordinator's too, as its record says. |
| 3D-2: "Every export" in the core README says that every export but `start-later-work.v1` was in the batch the owner accepted. | The README's own account of each pass says that no other export was added. It was not checked against the record of the batch. |

## Found in the read-through and fixed

Each of these follows from the text or from a decision.

- `set-up.v1`, `retire.v1`, the head of `spec/core/operations.yaml` and the head of
  `spec/core/capabilities.yaml` still said that `management` authorizes every request for a
  Resource of a retired Pool. They now name the two requests of decision 3A-8.
- IFC-006 said that a start of later work is sent "the Conduit to which the work is tied".
  Work is now tied to several, so it says "the Conduit through which it is dispatched". ACC-040
  was reworded in the same way.
- TPL-015 first said that after a stop is reconciled as not having occurred the instance
  still holds the runtime. That is not so for the stop of OBS-007, after which the engine
  already treats the handle as no longer held. The observable now gives both cases.
- PLC-017 first said that declaring lost a location that a request retired frees the
  instances still placed on it. PLC-011 as changed leaves none there, so it now says what
  remains: the location itself, where its own shutdown is not confirmed.
- OPS-023 and ACC-037 said "whatever else is wrong" of a request whose access decision cannot
  be made. A request that cannot be read still reports invalid, and both now say so.
- The definitions said both that every request ends in one of seven ways and that the result
  of a repeat in progress "is not one of the seven". The repeat is not a new request, and the
  request ends when its invocation does (3D-16).
- The comment on `'#directory-settings'` in the example project, and the description of
  `template-reference.v1`, said that `#name` as `from` names a template of the same project.
  Both now follow INS-025.
- The comment in `tools/check_schemas.py` on the rule `operations-filled` said "not composed",
  and the docstring of `bounds` said that a bound applied through constituents is not
  followed. Both were corrected.
- PKG-023 lists examples of promises that reject a package when it is installed. MAN-011 and
  CTR-011 were added to it.
- The head comment of `spec/conventions/operations.yaml` said "the sender".

## What changed in the overview

- The opening says when the page was last read against the specification, and after which
  pass (3D-11).
- "Documents, templates and instances": a request that an actor makes on its own behalf
  names an object by a reference and never writes `{kind, id}` (LIF-031).
- "Pools, ownership and placement": a Resource is made on its own for a Pool that is not
  retired (POOL-004).
- "Conduits and access": the two tools of the core package that need no Conduit (3D-12).
- "One lifecycle": retiring an instance closes its Conduits, at once where it is the target,
  and once its shutdown is confirmed where it is the consumer; until then they permit only
  the requests of its retirement (3D-13).
- "Operations, outcomes and reasons": "exactly one of seven ways" became "one of seven ways",
  with a sentence on a repeat of an invocation that has not ended (3D-16); and a paragraph on
  later work, which is how an invocation begins without a consumer's request (3D-15).
- "Packages, contracts and the core package": listing a package does not install it (3D-14),
  and the engine also presents the two tools that need no Conduit (3D-12).
- "Three layers": a claim of conformance names a version (3D-17).

The two quoted documents were compared with the files of the example project by a script, and
each still occurs in its file character for character.

## Still needs a decision

The ten points that this section listed after the fix pass were settled by the coordinator in
the section "The fix pass, and what it left" of [the decisions](DECISIONS.md), and applied in
the [follow-up](#follow-up) below. What remains:

- For the owner, as before: how a value is marked as secret.
- With the conformance vectors: the shape of a diagnostic beyond `message`, and the full
  shape of an inspection result.

The follow-up found nothing new that needs a decision. Two things it had to word are in its
table of choices, for the coordinator to confirm or strike.

## Not done, and not verified

- Nothing was checked against the Model Context Protocol. IFC-006, IFC-021, IFC-024, IFC-025
  and IFC-028 changed only in what they say of Conduit's own keys and tools, and what the
  interface says of the protocol itself was left as it was (3D-5).
- No conformance vector exists, so no rule in this log was tested. The three new rules of the
  checker are applied as far as the documents in front of it show, and each was seen to
  reject its example. They are not an engine's check.
- No example was added for 3B-12, for the reason in the table of choices.
- `spec/schema-profile.md` and `spec/implementation-defined.yaml` were read and not changed:
  no decision bears on them. DL1 still leaves to each engine what a later run does with steps
  that already ran, within OPS-021 and OPS-028.
- The read-through was the editor's own reading of the 87 promises changed or added and of
  their neighbours. No cold reader has read the result.

## Checks

Run from the root of the repository, after the last edit. The lines that begin "rejected" are
left out of the second, the path of the interpreter is elided, and the lines of its steps 5
and 6 are wrapped here: the tool prints each as one line.

```text
$ /usr/bin/python3 - <<'PY'   (the validation script of CONSOLIDATION.md)
436 promises ok

$ python3 tools/check_schemas.py
Python 3.9.6 at <path of the system interpreter>; PyYAML 6.0.1; jsonschema 4.20.0
Profile: 27 keywords, 5 formats. Format checkers available for all of them.
1. Read 104 files holding 230 documents as strict YAML.
2. Checked 58 Schema contracts against the profile, and every `$ref`.
3. Validated 138 documents under spec/schemas/, spec/core/ and examples/valid/.
4. 60 invalid documents in 60 files were each rejected by a schema, or rejected when read.
5. Core package: 99 documents exported, 0 documents deliberately private and 1 implementation,
   which is also an export. Checked 439 references for the kind they name; 1 name packages
   that are not in this repository and were not checked. Checked 3 controls, the 5 operations
   that take no `revision`, the tables of the two READMEs, and 846 citations of promises in
   107 files. The example packages keep 0 documents private.
6. Applied 19 rules of this tool, which come from 17 promises, to the core package and 3
   example directories. 20 sets of documents in 20 files under examples/invalid-by-rule/ were
   each rejected by the rule they are named for; each of their 33 documents is valid against
   the schemas.
   SCH-019: validated 10 defaults of the core package and the valid examples against their
   schemas; 0 were not validated, because a `$ref` in the schema leads outside this repository.
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x436, apiVersion x233, metaPrefix x96. Errors: 0

$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 127 Markdown files and 1612 local links (offline).
Errors: 0

$ git diff --check
(no output)
```

Also checked, by scripts that are not kept in the repository:

- No line of a YAML file under `spec/` is longer than 96 characters.
- Every promise ID and every DL item cited under `spec/` and in `docs/OVERVIEW.md` exists:
  4102 citations in all. The IDs of removed promises occur only in the comments at the heads of
  files.
- The counts in the contents table of `spec/README.md`, in the core README and in the schemas
  README were recounted against the files: the promises of each of the 22 files, 15
  implementation-defined items, 99 exports with 21, 5, 6, 2, 11 and 2 contracts and 7
  templates under `spec/core/`, 45 Schema contracts in 9 files, and 7, 60 and 20 example
  files.
- No trailing whitespace in the new files.

## Follow-up

Done on 2026-11-06 by the same editor, on the ten decisions of the section "The fix pass, and
what it left" of [the decisions](DECISIONS.md). They are the coordinator's, and the owner has
not ruled on them. Same rules, same files. Nothing was committed or staged, and no sub-reader
was started.

Result: 436 promises became 437, with 221 of the layer `language`, 181 of the layer
`convention` and 35 of the layer `interface`. Added: OPS-031, of the layer `convention`.
Changed, 24: ACC-015, ACC-018, ACC-031, ACC-033, ACC-038; LIF-015, LIF-018, LIF-021, LIF-024,
LIF-028, LIF-031; OPS-014; REV-010; IFC-024; CON-008, CON-022; GRA-005; MAN-009; PKG-019,
PKG-020; SUB-009; TPL-007, TPL-009, TPL-015. No promise changed layer or level, and none was
removed. `operations.yaml` now holds 31 promises. The implementation-defined list still has 15
items, and DL1 gained a point. One schema changed shape: `implementation-item.v1` no longer
lets an item that states `graph` state `supports`. Examples: 61 files under `examples/invalid`,
one more, `PKG-019-manifest-graph-with-supports.yaml`; 20 under `examples/invalid-by-rule` and
7 under `examples/valid`, as before. The checker still applies 19 rules, from 17 promises. The
core package still exports 99 documents, and no contract of it gained or lost an input.

| Point | Promises | Other documents |
| --- | --- | --- |
| 1. A start of unknown outcome, and its reconciliation on a retired instance | LIF-024 (`then`: the shutdown is unknown while such a start is unknown, and pending once it is reconciled as having occurred on a retired instance), LIF-015 (admits the reconciliation of a start or a stop), LIF-018, LIF-028, TPL-015, TPL-007, TPL-009, CON-008 | `reconcile.v1`; definitions (shutdown, reconcile); core README |
| 2. Inputs of `reconcile.v1` that do not belong | New OPS-031; OPS-014, REV-010 and TPL-015 point to it | `reconcile.v1`; definitions (reconcile); core README |
| 3. A request to a retired Pool | ACC-033 (the observable now gives the reason through ACC-011) | — |
| 4. No graph states `supports` | PKG-019 (`then`), CON-022 (`then`), PKG-020, MAN-009, ACC-015, ACC-018, ACC-038 | `implementation-item.v1` and the head of `spec/schemas/manifest.yaml`; a comment on `listing` in the example package, which stated none already; new invalid example; schemas README; definitions (`supports`) |
| 5. The order of retirements, and stopping | LIF-021 (`then`) | DL1; `retire-with-resources.v1`; core README |
| 6. Every selection of a graph | GRA-005 (`when`) | A comment in `tools/check_schemas.py` |
| 7. How a requirement is matched | CON-022 (`then`: by its address and its export name), ACC-015, ACC-031 | Definitions (`supports`) |
| 8. The manifest handed to `replace.v1` | SUB-009 (`then`) | `replace.v1`; definitions (replace) |
| 9. The resolved form in a retrieval | LIF-031, IFC-024 | `retrieve-result.v1`; definitions (resolved form) |
| 10. Until when an instance holds a runtime | TPL-007 (`then`), TPL-015 | Definitions (hold) |

Choices made in the follow-up:

| Choice | Why |
| --- | --- |
| Point 1: LIF-015 admits the reconciliation of a stop of a runtime on a retired instance, as well as that of a start. | A stop is dispatched within a retirement, so its reconciliation is always on a retired instance, and point 10 has that reconciliation end the holding. |
| Point 1: TPL-009 says that an invocation of `retire.v1` that succeeds does not confirm the shutdown while such a start is unknown. | It follows from LIF-024 as changed, and TPL-009 said that such an invocation "succeeds at once". |
| Point 2: the rule is a new promise, OPS-031, beside OPS-030. | It names one reason, and covers inputs of two kinds of reconciliation. |
| Point 2: a reconciliation that records a start as **not** having occurred and states `handle` is not made invalid. | The decision speaks of an invocation that is not the start of a runtime. TPL-015 already gives this request its end: the outcome becomes failed and the instance is as it was. For the coordinator to confirm or strike. |
| Point 4: for every graph, inspecting an open Conduit names the implementations selected for its steps that have no `conduit` (ACC-038), also where a manifest lists the graph under a name. | Those implementations apply the filter, and the promises now say one thing for every graph. |
| Point 4: CON-022 says that where another promise speaks of an implementation that "states a requirement in `supports`", it means for a graph that the graph supports it. | CTR-005, IFC-018 and IFC-034 use those words, and were not reworded. |
| Point 5: DL1 also takes the order among Resources that neither relation joins, not only the order within a cycle. | The decision names the cycle. Once a request stops at the first retirement that is not confirmed, the order among unrelated Resources shows as well, and DL1 already leaves the like order among steps to the engine. For the coordinator to confirm or strike. |
| Point 7: the rule is in the `then` of CON-022, where "supports" is defined, and the sentence on a policy's arguments in the observables of ACC-015 and ACC-031. | One place defines the match. |
| Point 8: "may hold `packages` alone" is written "beside its `apiVersion`". | MAN-003 requires `apiVersion` of every manifest. |

Read again after the edits: the promises touched and their neighbours LIF-014, LIF-017,
LIF-020, LIF-025, LIF-026, LIF-030, PLC-017, OPS-013, OPS-021, OPS-030, MAN-010, MAN-012,
IFC-019, CTR-005, NAM-009 and NAM-024. One thing was found and fixed: the observable of LIF-024
said that an instance with such a start is not unregistered until the start is reconciled,
and a location declared lost also frees it, under LIF-017.

Checks after the follow-up, run from the root of the repository:

```text
$ /usr/bin/python3 - <<'PY'   (the validation script of CONSOLIDATION.md)
437 promises ok

$ python3 tools/check_schemas.py
Python 3.9.6 at <path of the system interpreter>; PyYAML 6.0.1; jsonschema 4.20.0
Profile: 27 keywords, 5 formats. Format checkers available for all of them.
1. Read 105 files holding 231 documents as strict YAML.
2. Checked 58 Schema contracts against the profile, and every `$ref`.
3. Validated 138 documents under spec/schemas/, spec/core/ and examples/valid/.
4. 61 invalid documents in 61 files were each rejected by a schema, or rejected when read.
5. Core package: 99 documents exported, 0 documents deliberately private and 1 implementation,
   which is also an export. Checked 439 references for the kind they name; 1 name packages
   that are not in this repository and were not checked. Checked 3 controls, the 5 operations
   that take no `revision`, the tables of the two READMEs, and 852 citations of promises in
   108 files. The example packages keep 0 documents private.
6. Applied 19 rules of this tool, which come from 17 promises, to the core package and 3
   example directories. 20 sets of documents in 20 files under examples/invalid-by-rule/ were
   each rejected by the rule they are named for; each of their 33 documents is valid against
   the schemas.
   SCH-019: validated 10 defaults of the core package and the valid examples against their
   schemas; 0 were not validated, because a `$ref` in the schema leads outside this repository.
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x436, apiVersion x234, metaPrefix x96. Errors: 0

$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 127 Markdown files and 1613 local links (offline).
Errors: 0

$ git diff --check
(no output)
```

No line of a YAML file under `spec/` is longer than 96 characters, every promise ID and DL
item cited under `spec/` and in `docs/OVERVIEW.md` exists (4181 citations), and the counts in
`spec/README.md` and the schemas README were recounted. `docs/OVERVIEW.md` was read against
the ten decisions and not changed: none bears on what it says. Nothing was checked against the
Model Context Protocol, and no cold reader has read the result.
