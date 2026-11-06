# Round 4: log of the fix pass

Written 2026-11-06 by the editor of the fix pass. The work order was every row of
[the decisions](DECISIONS.md), which are the coordinator's under ruling 37 and which the owner
has not ruled on. Every row below is the editor's account of applying those decisions: where a
row words something that a decision left open, the wording is the editor's, and the table
[Choices](#choices) lists it. The findings are in the reports of [reader 1](reader-1.md),
[reader 2](reader-2.md), [reader 3](reader-3.md) and [reader 4](reader-4.md), and in the
[second check against the Model Context Protocol](mcp-recheck.md). The rules were those of
[CONSOLIDATION.md](../CONSOLIDATION.md).

Edited: files under `spec/`, `tools/check_schemas.py` and `docs/OVERVIEW.md`. Nothing else
under `docs/` was edited apart from this log. Nothing was committed or staged. No sub-reader
was started: the read-through at the end was the editor's own.

## Result

| | Before | After |
| --- | --- | --- |
| Promises | 437 | 442 |
| Of the layer `language` | 221 | 223 |
| Of the layer `convention` | 181 | 183 |
| Of the layer `interface` | 35 | 36 |
| Items of the implementation-defined list | 15 | 15 |
| Documents the core package exports | 99, and 1 implementation | 99, and 1 implementation |
| Operation contracts of the core package | 21 | 21 |
| Schema contracts under `spec/schemas/` | 45 in 9 files | 45 in 9 files |
| Files under `examples/valid` | 7 in 3 directories | 7 in 3 directories |
| Files under `examples/invalid` | 61 | 62 |
| Files under `examples/invalid-by-rule` | 20 | 24 |
| Rules the checker applies in its step 6 | 19, from 17 promises | 23, from 21 promises |

- Added, 5: LIF-033 and LIF-034, of the layer `convention`; TPL-016 and NAM-028, of the layer
  `language`; IFC-036, of the layer `interface`.
- Changed, 90: ACC-001, ACC-005, ACC-012, ACC-015, ACC-017, ACC-018, ACC-019, ACC-024,
  ACC-026, ACC-031, ACC-032, ACC-033, ACC-034, ACC-038, ACC-040, ACC-041, ACC-042; IMP-004,
  IMP-006, IMP-010, IMP-012; LIF-001, LIF-009, LIF-017, LIF-018, LIF-019, LIF-021, LIF-022,
  LIF-023, LIF-024, LIF-027, LIF-028, LIF-030; OPS-005, OPS-006, OPS-009, OPS-010, OPS-012,
  OPS-015, OPS-016, OPS-017, OPS-022, OPS-024, OPS-026, OPS-031; REC-003, REC-004; REV-010;
  IFC-010, IFC-012, IFC-018, IFC-024, IFC-025, IFC-026, IFC-027, IFC-029, IFC-032, IFC-033;
  CON-004, CON-008, CON-024; GRA-003, GRA-008, GRA-022, GRA-024; INS-006, INS-013, INS-019,
  INS-020, INS-025; MAN-004; NAM-009, NAM-019, NAM-021, NAM-022, NAM-023, NAM-026; OBJ-026;
  PKG-017, PKG-018, PKG-023, PKG-027; SCH-009; SUB-008, SUB-009, SUB-010; TPL-007, TPL-009,
  TPL-011, TPL-015.
- Changed level, 1: IFC-026, from `may` to `must` (decision 4A-9). No promise changed layer,
  none was removed, and no ID was reused.
- Per file, where the count changed: lifecycle 34, templates 16, naming 28, interface 36.
- One core operation gained an output: `rename.v1` returns `revision`. No export was added.
- One schema of a document changed shape: the `conduit` of a step in `step.v1` is a name or a
  dotted path of names, where it was a name.
- New example files: `invalid/GRA-024-step-conduit-with-an-empty-segment.yaml`, and under
  `invalid-by-rule/` `GRA-024-step-names-a-composite-with-two-components.yaml`,
  `TPL-016-step-invokes-the-stop-operation-of-a-runtime.yaml`,
  `NAM-028-selection-that-names-nothing.yaml` and
  `NAM-021-manifest-states-the-core-address.yaml`. The last holds a manifest: the checker now
  reads a file under `invalid-by-rule/` with `-manifest-` in its name as one.
- The checker gained the rules `conduit-names-one` (GRA-024), `runtime-operation-step`
  (TPL-016), `names-nothing` (NAM-028) and `core-address` (NAM-021), and in its step 5 the
  check that `replace.v1` and `rename.v1` declare the output `revision`.

## What was changed for each finding

"Definitions" is `spec/definitions.md`, "core README" is `spec/core/README.md`, "schemas
README" is `spec/schemas/README.md`, and a name ending in `.v1` is that contract's description
in `spec/core/operations.yaml` unless another file is named.

| Finding | Promises | Other documents |
| --- | --- | --- |
| 4A-1 | LIF-024 (`then`: the shutdown of an instance that holds a runtime is never confirmed; when it is unknown and when pending; the reconciliation that reports conflict), LIF-028 (`then`), LIF-017, LIF-018, LIF-030, TPL-009, TPL-015, CON-008 | Definitions (shutdown, hold, reconcile); `reconcile.v1`; core README ("Reconciling"); overview |
| 4C-1 | ACC-040 (`then`: no Conduit to which the work is tied has a retired consumer), ACC-019 (`then`), IMP-006 (`then`), ACC-001 | Definitions (later work); `start-later-work.v1`; core README; overview |
| 4A-5 | IMP-004 and IMP-010 (`when`: every selected graph, in the words of GRA-005), IMP-012 (observable) | Definitions (later work) |
| 4A-2 | TPL-015 (`then`: no handle where the instance no longer exists), OPS-031 (`when` rewritten) | Definitions (reconcile); `reconcile.v1`; core README; head of `spec/core/operations.yaml` |
| 4A-3, 4B-1 | None | `object-management.v1` in `spec/core/capabilities.yaml` |
| 4A-4 | IFC-018 (`then`) | Definitions (restriction) |
| 4A-6 | REV-010 (`when` widened to every reconciliation of a control; `then` says which one names a revision; the observable cites REV-002 and REV-011 for the reconciliation that stores settings alone), OPS-031 | `reconcile.v1`; core README (table of operations, "Reconciling") |
| 4A-7 | ACC-015 and ACC-026 (observables state the consequence), LIF-001 | Definitions (policy); `apply-policy.v1`; core README; overview |
| 4A-8 | OPS-015 (`when`: a target that no longer exists, however it is named), OPS-016 (`when`), IFC-024 | `retrieve-result.v1`; core README |
| 4A-9 | IFC-026 (`must`; `when` and `then` reworded as an obligation), IFC-027, OPS-006 | Definitions (how a request ends); `spec/README.md` (conformance) |
| 4A-10 | New LIF-033 (from the observable of LIF-009) and LIF-034 (from that of LIF-022); LIF-017, TPL-011, NAM-023 and IFC-029 (`then` states the reason); ACC-012 (`then` states the reason that stood in ACC-024); ACC-032 (`when` and `then` cover the request to open a Conduit that lists a reserved operation); PKG-023 (`when` covers a relative `fetch`) | `register-replacement.v1` |
| 4A-11 | SUB-008 (`then`), SUB-010 (`then`) | Definitions (unpublished package) |
| 4A-12 | PKG-027 (`then`: decided like any other), PKG-023 | Definitions (install); `install.v1`; core README ("Installing") |
| 4B-2 | NAM-021 (`when`, `then`) | `spec/schemas/manifest.yaml` (head, description of `manifest.v1`); schemas README (table); new example and checker rule |
| 4B-3 | CON-024, ACC-033 (observables) | Definitions (management verb); core README; head of `spec/core/operations.yaml` |
| 4B-4 | MAN-004, INS-025, NAM-009, NAM-022, PKG-017, SUB-008, ACC-038 | Definitions (project, published, `#name`, manifest, unpublished package, inspect); `spec/README.md`; schemas README; `inspect.v1`; `spec/schemas/instances.yaml`; head of `spec/schemas/manifest.yaml`; comments of the example project |
| 4B-5 | None | Schemas README (new row for SUB-009); head of `spec/schemas/manifest.yaml` |
| 4B-6 | None | `spec/README.md`; core README; `spec/core/conduit.yaml`; `spec/core/templates.yaml`; overview |
| 4B-7 | ACC-017, ACC-019, ACC-040, ACC-041, IMP-006 | Definitions (new row "chain"; onward request, later work) |
| 4B-8 | None | Definitions (hold, `supports`) |
| 4B-9 | SUB-009 (`then`), OBJ-026 (`then`) | `rename.v1` gains the output `revision`; `replace.v1`; definitions (replace, rename); core README; head of `spec/core/operations.yaml`; checker |
| 4B-10 | None | Core README (the keys of a request) |
| 4B-11, 4D-26 | Head of each of the 22 promise files | Heads of `spec/implementation-defined.yaml`, of the files of `spec/core/` and `spec/schemas/`; status line of `spec/schema-profile.md`; opening of the definitions; `spec/README.md`; core README; schemas README |
| 4C-2 | GRA-024 (`then`), ACC-018, GRA-022 | `step.v1` in `spec/schemas/graphs.yaml`; definitions (step, declared Conduit); schemas README; core README ("Every export"); two new examples and a checker rule |
| 4C-3 | CON-004 (`then`), ACC-005, LIF-027 (`then`: the exception), LIF-023, GRA-003, OPS-022, CON-008. The decision was replaced afterwards: see the [follow-up](#follow-up), which took the exceptions out again | Definitions (close, set up, declared Conduit); `set-up.v1`; DL1 |
| 4C-4 | OPS-024 (`then`), OPS-012, OPS-017, ACC-034, ACC-019 | Definitions (attempt); `retrieve-result.v1`; core README |
| 4C-5 | LIF-009 (`then`), INS-013, INS-020 | Definitions (nested instance); `register-replacement.v1` |
| 4C-6 | LIF-021 (`then`) | `retire-with-resources.v1`; core README ("Retiring") |
| 4C-7 | OPS-022 (`then`) | None |
| 4C-8 | REC-004 (`then`) | Definitions (purge); `purge.v1` |
| 4C-9 | ACC-031 (`then`), ACC-042 (`when`) | Definitions (policy); `apply-policy.v1`; core README ("Published contracts only") |
| 4C-10 | SCH-009 (`when` and `then`: every schema that governs a supplied value), OPS-026 (`when`, `then`), INS-006, INS-013, INS-019, OPS-009, OPS-010, NAM-019, ACC-031, ACC-042, IFC-018, GRA-008 | Definitions (input); `spec/schema-profile.md` |
| 4C-11 | New NAM-028; NAM-022, PKG-018 | Definitions (resolve, `#name`, selection); schemas README; new example and checker rule |
| 4C-12 | New TPL-016; TPL-007 | Definitions (runtime); head of `spec/schemas/graphs.yaml`; schemas README; new example and checker rule |
| 4C-13 | ACC-018 (`then`), ACC-017 | Definitions (onward request) |
| 4C-14 | IFC-033 (`then`), IFC-012 (`then`), NAM-026 | Definitions (tool); core README |
| 4C-15 | REC-003, LIF-019 | DL8; definitions (unregister) |
| Protocol, row 26 | New IFC-036; IFC-025, OPS-005 | Definitions (result, Conduit's keys); core README |
| Protocol, row 22 | IFC-025 (`then`), IFC-033, ACC-034 | Definitions (result); core README; head of `spec/core/operations.yaml` |
| Protocol, row 36 | IFC-012 (observable, the report's paragraph B) | None |
| Protocol, row 21 | None | None |
| Protocol, rows 18, 23, 29, 30, 47 | IFC-025 (18, 23), IFC-010 (29, 30), IFC-032 (47) | DL12 (23); head of `spec/interface/implementation.yaml` |
| 4D-17 to 4D-21 | None | Overview: see [below](#what-changed-in-the-overview) |

## Choices

Each of these is the editor's choice where a decision left wording or a detail open. None is
the coordinator's or the owner's, and each stands unless struck.

| Choice | Why |
| --- | --- |
| 4A-1: while an instance holds a runtime its shutdown reports unknown where the outcome of its latest retirement is unknown, and pending otherwise. | The decision fixes only that it is not confirmed. LIF-024 had two sentences that gave both values for one state, and this keeps the rule it already had for the latest retirement. |
| 4A-1: the conflict stands in the `then` of LIF-024, and LIF-028 repeats it with a citation. | The decision places the rule in LIF-024, and LIF-028 is the promise whose `when` is the reconciliation. |
| 4A-2, 4A-6: `handle` stated beside a record that a start did not occur reports invalid, as `revision` and `settings` do beside a record that a control did not take effect. | Decision 4A-6 gives the rule for a control: an input is stated only where the reconciliation stores it. The same rule is applied to `handle`, so that OPS-031 is one rule. |
| 4A-5: IMP-012 gained a sentence in its observable and no change to its rule. | A graph that a step of `setup` or `retire` invokes implements an operation without the mark, so IMP-010 as widened already rejects it. |
| 4A-7: the consequence is stated in the observables of ACC-015 and ACC-026, and in one sentence of LIF-001. | The decision says the behavior stands, so no `then` changes. |
| 4A-10: beside the five that reader 1 named, three more reasons were moved: that of LIF-022, into the new LIF-034; that of ACC-024, into the `then` of ACC-012; and that of a request to open a Conduit which lists a reserved operation, into ACC-032. A relative `fetch` of `install.v1` was added to the `when` of PKG-023. | Each stood only in an observable and is not the refusal of a request that no Conduit lists. Refusals that follow from "dispatched only if" were left where they were. |
| 4B-2: a manifest handed to `replace.v1` is not named in NAM-021. | SUB-009 already rejects `address` in such a manifest. |
| 4B-4: MAN-004 keeps its rule, both or neither, and says that the two keys change nothing in the manifest of a submitted project. A qualified reference that uses the stated address is still rejected under NAM-007. | The decision says the keys change nothing. NAM-022 already has a document reach its own project as `#name`. |
| 4B-7: the row for chain says that a step of a graph serves the invocation that its graph serves. | ACC-019 ties work that a step with no `conduit` arranged to the Conduit and the chain of that invocation, so the step has to be in the relation. |
| 4B-9: for a request that changes nothing, `revision` is the revision the object had. | The output is required, and reader 2 proposed this value. |
| 4B-11, 4D-26: each promise file gained one paragraph, "Standing", after its list of changes, in place of a clause after each mention of the docs audit and of round 2. | One sentence that covers every listed change cannot be missed for one of them. |
| 4C-2: a path may stop at a component that is itself a composite, and the rule of one Conduit that lists the operation applies below it. A step whose Conduits list the operation in none is rejected, as before. | IFC-012 writes every component on the way, and the decision gives the rule for the ambiguous case alone. |
| 4C-2: resolving fixes which component Conduit a step means, and a later replacement does not change it. | INS-013 has an instance hold its resolved definition. Without it, a replacement made under decision 4C-5 from a changed template could make a resolved step ambiguous. |
| 4C-2: no use was added to the valid example project. | It declares no Conduit from a composite template. The dotted path is shown in the example for the ambiguous case, whose documents are each valid against the schemas, and a new example under `invalid/` shows a path the schema rejects. |
| Struck by the coordinator, and undone in the [follow-up](#follow-up). 4C-3: "closed again" is applied as written. The component Conduits that the step opened are closed, which retires them for good, so a later setup needs a replacement for each. LIF-027 and LIF-023 gained the exception. The step's outcome is failed whatever the request that opened none reported. A component Conduit that is closed when the step is reached makes the step report conflict before it opens any. | The decision governs. This is the first point under [Still needs a decision](#still-needs-a-decision). |
| 4C-5: a `from` that names a template of a package means that template in the version the instance holds. | PKG-005 has the instance hold the version, and a published version does not change. |
| 4C-8: a purge removes records and nothing else. The Conduits to which later work is tied, what an instance holds and the content of a retained result are not records. | The decision adds three things to the list and not the chain. ACC-019 ties the work when it is arranged, and REC-004 removes records. |
| 4C-10: a value that is supplied to a control is filled as any supplied value is. REV-006 still applies no default to an input that is left out. | The decision is one rule for every schema that governs a supplied value. |
| 4C-12: the rule is for a step with no `conduit`, for a runtime of the template or of its type, and for every graph selected for the template's instances. | A step with `conduit` is a request to another instance. An instance has the runtimes of its type too. A graph that a manifest lists would otherwise escape, as in finding 4A-5. |
| 4C-14: a consumer's call to the tool that is not presented reports refused, and a step of a graph that names the Conduit and the operation reports unavailable under NAM-026, as before. | The decision speaks of the tool. A step calls no tool, and NAM-026 already gives its reason. |
| 4C-15: DL8 gained a sentence that names the case. | The decision says that DL8 is where it is settled, and DL8 did not mention it. |
| Protocol, row 26: a new promise, IFC-036, in place of a sentence in the observable of IFC-025. Where the engine could establish no instance, IFC-032 says what the request reports. | The decision gives a reason and a record, which belong in a `then`. |
| Protocol, row 22: the tool of an operation that declares no outputs has an `outputSchema` of a map with no properties. | IFC-033 derives `outputSchema` "in the same way", and the decision has one rule for every tool. |
| DL1 gained the order in which the component Conduits of a composite are opened, and DL12 the serialized copy of `structuredContent`. The follow-up took the clause out of DL1 again, since the components are now opened together. | CON-004 and IFC-025 cited those items for them. |

## Found in the read-through and fixed

The read-through covered the 95 promises changed or added, their neighbours, the definitions,
the core package, the schemas and the examples.

- LIF-028 still said that the shutdown is pending after a retirement is recorded as not having
  occurred, with no word on a start of unknown outcome. It now gives what LIF-024 gives.
- ACC-019 and ACC-040 did not say which Conduit work is arranged through where the arranging
  step's graph was itself invoked by a step with no `conduit`. ACC-019 now names the nearest
  invocation of the chain that was dispatched through a Conduit.
- LIF-009 said that the tool names of a declared Conduit do not change with a replacement.
  After decision 4C-5 a replacement may list other operations, so the sentence now speaks of
  the operations that the new Conduit still lists.
- Decisions 4C-2 and 4C-5 meet: a replacement made from a changed template could make a
  resolved step ambiguous. GRA-024 says that resolving fixes the component (see Choices).
- INS-020 did not say what a replacement is made from once a template has been replaced. It
  now points to LIF-009.
- ACC-001 said that the Conduit through which the work was arranged decides a start. It is
  every Conduit to which the work is tied.
- INS-019 said that controls apply no default, which read against decision 4C-10. It now
  speaks of a setting whose input is not supplied.
- OPS-005 named one request that is answered with no result. It names the requests of IFC-036
  too.
- The core README cited OPS-016 for the refusal of a retrieval whose target is unregistered.
  The rule is OPS-015.
- The head of `spec/interface/implementation.yaml` said that the promises had not been
  checked against the protocol since round 3. It now says that the second check read them.
- The conformance table of `spec/README.md` lists IFC-026 among the promises that bind an
  implementation, since it is now a `must`.
- The opening of the definitions credited every export name to ruling 37. It now excepts
  `start-later-work.v1` (4B-11).

## What changed in the overview

`docs/OVERVIEW.md` was read in full against the specification as this pass leaves it.

- The "last read" line names this pass (4D-21).
- "Conduits and access": a new paragraph says that the engine decides whether a Conduit lists
  the operation and checks a bound and the shared-host requirement, that every other
  restriction is sent to the target's implementation, which applies it and must state its
  requirement under `supports`, that a plain implementation supports none, and that a request
  whose restriction the implementation does not support reports unavailable (4D-17). It ends
  with the consequence of decision 4A-7 for `management`, `access` and a Pool.
- The same section says that no policy applies to a request of `root`, that no first-start
  object is retired, unregistered or renamed, that no request closes a Conduit of `root`, and
  that two operations are reserved to it (4D-18).
- It says that a declared Conduit whose template is a composite makes one Conduit for each
  component.
- "One lifecycle": the Conduits of a retired consumer are also closed at once where it is
  declared lost; shutdown and declaring a location lost are each explained in a sentence; the
  shutdown is never confirmed while the instance holds a runtime or a start is unknown (4D-20,
  4A-1). The stop that the engine dispatches for no actor is stated (4D-19).
- "Operations, outcomes and reasons": the implementation applies the restrictions after the
  engine's decision and answers refused or unavailable where one forbids the request or
  cannot be applied; a reconciliation goes to
  `management` where the Pool is retired or the instance unregistered (4D-20); the stop of
  OBS-007 is named beside later work as an invocation that no consumer requested (4D-19); a
  start of later work also needs that no Conduit it is tied to has a retired consumer (4C-1).
- "Packages": the core package exports the templates that first start instantiates (4B-6).

The page is about 2,400 characters longer than it was. The two quoted documents were compared
with the files of the example project by a script, and each still occurs in its file
character for character.

## Still needs a decision

The five points that this section listed after the fix pass were settled by the coordinator in
the section "The fix pass, and what it left" of [the decisions](DECISIONS.md), and applied in
the [follow-up](#follow-up) below. What remains:

- Decision 4A-7 is also listed for the owner.
- For the owner, as before: how a value is marked as secret.
- With the conformance vectors: the shape of a diagnostic beyond `message`, and the full
  shape of an inspection result.

The follow-up found nothing new that needs a decision. What it had to word is in its table of
choices, for the coordinator to confirm or strike.

## Not done, and not verified

- Nothing was checked against the Model Context Protocol in this pass. What the pass changed
  in what the interface says of the protocol is the wording of the second check for rows 26,
  36, 22, 18, 23, 29, 30 and 47, and IFC-036 adds two sentences of its own on the protocol:
  that a client sends a request again at a revision both sides support, and that an error's
  `data` may hold what the server defines. Both rest on rows 4 and 25 of that check, which
  confirmed them. The new text has not been read against the protocol by anyone.
- No conformance vector exists, so no rule in this log was tested. The four new rules of the
  checker are applied as far as the documents in front of it show, and each was seen to
  reject its example. They are not an engine's check. No rule was added to the checker for
  IMP-004 and IMP-010, for the composite step of CON-004 or for the defaults of SCH-009.
- The step-6 line of the checker counts the manifest of the new example among "documents".
- The read-through was the editor's own. No cold reader has read the result.

## Checks

Run from the root of the repository, after the last edit. The lines that begin "rejected" are
left out of the second, the path of the interpreter is elided, and the lines of its steps 5
and 6 are wrapped here: the tool prints each as one line.

```text
$ /usr/bin/python3 - <<'PY'   (the validation script of CONSOLIDATION.md)
442 promises ok

$ python3 tools/check_schemas.py
Python 3.9.6 at (path elided); PyYAML 6.0.1; jsonschema 4.20.0
Profile: 27 keywords, 5 formats. Format checkers available for all of them.
1. Read 110 files holding 240 documents as strict YAML.
2. Checked 58 Schema contracts against the profile, and every `$ref`.
3. Validated 138 documents under spec/schemas/, spec/core/ and examples/valid/.
4. 62 invalid documents in 62 files were each rejected by a schema, or rejected when read.
5. Core package: 99 documents exported, 0 documents deliberately private and 1
   implementation, which is also an export. Checked 439 references for the kind they name; 1
   name packages that are not in this repository and were not checked. Checked 3 controls,
   the 5 operations that take no `revision`, the 2 that return one, the tables of the two
   READMEs, and 881 citations of promises in 113 files. The example packages keep 0
   documents private.
6. Applied 23 rules of this tool, which come from 21 promises, to the core package and 3
   example directories. 24 sets of documents in 24 files under examples/invalid-by-rule/
   were each rejected by the rule they are named for; each of their 41 documents is valid
   against the schemas.
   SCH-019: validated 10 defaults of the core package and the valid examples against their
   schemas; 0 were not validated, because a `$ref` in the schema leads outside this
   repository.
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x437, apiVersion x243, metaPrefix x98. Errors: 0

$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 135 Markdown files and 1694 local links (offline).
Errors: 0

$ git diff --check
(no output, exit status 0)
```

Also checked by script: no line of a YAML file under `spec/` is longer than 96 characters; no
new file has trailing whitespace; every promise ID cited under `spec/`, in `docs/OVERVIEW.md`
and in `tools/check_schemas.py` exists, apart from removed IDs, which appear only in the
comments at the heads of the promise files; every item of the implementation-defined list
that is cited exists; and the counts of promises in the contents table of `spec/README.md`
are the counts in the files.

## Follow-up

Done on 2026-11-06 by the same editor, on the five decisions of the section "The fix pass, and
what it left" of [the decisions](DECISIONS.md). They are the coordinator's too. The rows are
the editor's account of applying them. No promise was added or removed, so the counts of the
table under [Result](#result) stand: 442 promises, 223 of the layer `language`, 183 of the
layer `convention` and 36 of the layer `interface`.

Changed in the follow-up, 21: ACC-005, ACC-015, ACC-017, ACC-019, ACC-031, ACC-034, ACC-039;
LIF-023, LIF-027, LIF-028; OPS-005, OPS-022; REC-001; IFC-018, IFC-032, IFC-036; CON-004,
CON-008, CON-022; MAN-008; NAM-028. LIF-023 and LIF-027 are again as they were before the fix
pass, and ACC-039, REC-001, CON-022 and MAN-008 had not been changed by it. So against the
specification as round 4 found it, 92 promises are changed and 5 are new. No export, no schema,
no example and no rule of the checker changed, and `tools/check_schemas.py` was not edited.

| Point | Promises | Other documents |
| --- | --- | --- |
| 1. A composite Conduit of which one component cannot be opened | CON-004 (`then` rewritten: the opening is decided for every component before any is opened; where one would not be dispatched none is opened, nothing is closed, and the step reports that component's reason; otherwise all are opened in one invocation with one outcome), ACC-005 (`then`), LIF-028 (`then`: the reconciliation of that opening records every component as open or every one as not open), LIF-027 and LIF-023 (the exceptions taken out), CON-008, OPS-022 | Definitions (declared Conduit, close, set up); `set-up.v1`; DL1 (the clause on the order of opening taken out) |
| 2. Onward requests that serve an invocation whose chain has a retired consumer | ACC-039 (`when` and `then` widened to an onward request, through any Conduit, in whose chain a Conduit has a retired consumer; the exception for that consumer's own retirement kept, and said to hold directly or through others), ACC-019 (`then`: the requests of work already started), ACC-034, ACC-017 | Definitions (onward request, later work); core README ("Retiring"); overview |
| 3. How a re-exported requirement is named on the wire and matched against `supports` | IFC-018 (`then`: `requirement` is the reference in the package that originally exports it), CON-022 (`then`: matched by that origin; an item of `supports` that names the origin or any re-export states it), ACC-015, ACC-031 | Definitions (restriction, `supports`) |
| 4. A re-export in `exports` that names nothing in an installed package | NAM-028 (`when` and `then` cover a manifest's re-export), MAN-008 | Definitions (resolve); head of `spec/schemas/manifest.yaml` |
| 5. A request that the protocol rejects, from a client that is not an instance | IFC-036 (`then`: answered so whoever sent it, and not recorded where no instance can be established), IFC-032 (`when` excepts the requests of IFC-036), REC-001, OPS-005 | Definitions (Conduit's keys) |

Choices made in the follow-up, each the editor's and standing unless struck:

| Choice | Why |
| --- | --- |
| Point 1: "a setup step is then failed" is written as the step reporting that component's reason and not having succeeded. GRA-003 then keeps the setup from succeeding where the declared Conduit is required, and has it reported as not opened where it is optional. | A step that is not dispatched has a reason and no outcome, as the step of a single declared Conduit has under ACC-005. The outcome failed belongs to something that was dispatched. |
| Point 1: where the opening of more than one component would not be dispatched, OPS-023 says which reason the step reports, and a diagnostic names each. | The decision speaks of one component. |
| Point 1: the one opening is called one invocation of `open.v1`, which the engine makes for the step and which stands for every component Conduit that the step opens. | The decision gives the opening one outcome and one reconciliation, and LIF-028 and ACC-033 reconcile an opening as an invocation of `open.v1` through `access`. The contract of `open.v1` names one Conduit in its input `conduit`, and was left as it is: no consumer makes this request, and what a record keeps of an operation's inputs is not settled. |
| Point 1: the rule is stated for the step of a setup. A request for `open.v1` or `close.v1` that an actor makes still names one Conduit, a component Conduit among them. | No instance stands for a composite (CON-003), so no request can name one. An actor can therefore still open or close one component alone. |
| Point 2: the rule stands in ACC-039, and ACC-034, ACC-017 and ACC-019 point to it. A step with no `conduit` is not an onward request and is not covered, as it is not under ACC-034. | ACC-039 is where the exception for the consumer's retirement is stated. |
| Point 3: no example address is written in IFC-018. | `tools/set_names.py` reads a qualified reference in the interface file as a `_meta` key. |
| Point 4: no rule was added to the checker for a re-export that names nothing. | The decision does not ask for one, and the pass was to be small. |
| Point 5: IFC-032 excepts "a request that the protocol has a server reject with an error of its own under IFC-036", in place of the words "a request that can be read". | IFC-029 also calls a request with keys under two prefixes one that cannot be read, and that request is answered with a result. The two promises now divide every request between them. |

Re-read for the follow-up: the 21 promises above and their neighbours, among them GRA-003,
GRA-022, GRA-024, LIF-008, LIF-015, LIF-026, ACC-006, ACC-016, ACC-029, ACC-040, IFC-012,
IFC-029, IFC-034, PKG-013, PKG-018 and REC-002. Nothing in them disagrees with the text as it
now stands. The second sentence of what the fix pass wrote under "Not done" on the protocol
still holds: nothing was checked against the Model Context Protocol, and IFC-036 now also says
that such a request is answered as the protocol states whoever sent it, which is the
protocol's own rule.

Checks, run from the root of the repository after the last edit of the follow-up:

```text
$ /usr/bin/python3 - <<'PY'   (the validation script of CONSOLIDATION.md)
442 promises ok

$ python3 tools/check_schemas.py
(steps 1 to 6 as above: 110 files, 240 documents, 58 Schema contracts, 138 documents
validated, 62 invalid documents rejected, 23 rules from 21 promises, 24 sets rejected)
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x437, apiVersion x243, metaPrefix x98. Errors: 0

$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 135 Markdown files and 1700 local links (offline).
Errors: 0

$ git diff --check
(no output, exit status 0)
```

Also checked by script, as before: no line of a YAML file under `spec/` is longer than 96
characters, and every promise ID and every item of the implementation-defined list that is
cited under `spec/` and in `docs/OVERVIEW.md` exists.
