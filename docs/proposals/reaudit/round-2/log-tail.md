# Round 2: log of the last pass

Written 2026-11-06 by the editor of the last pass. The work order was the section "The closing
pass, and what it left" of [the decisions](DECISIONS.md), which settles the fifteen points that
the [closing pass](log-closing.md) listed, and then the section "What the check against the
Model Context Protocol found", which settles what the [check](mcp-citations.md) reported. The
rules were those of [CONSOLIDATION.md](../CONSOLIDATION.md). The choices of the closing pass
stand.

Edited: files under `spec/`, and `tools/check_schemas.py`. Nothing under `docs/` was edited
apart from this log. Nothing was committed.

After the decisions were applied, every promise that was changed or added was read again with
its neighbours, and a further reader, who was given the text and the list of changed promises
and not this log, read the same promises cold. What the two found is under
[Found in the read-through](#found-in-the-read-through-and-fixed) and
[Still needs a decision](#still-needs-a-decision).

## Result

- 421 promises, 11 more than after the closing pass: 220 of the layer `language`, 166 of the
  layer `convention` and 35 of the layer `interface`. None was removed and no ID was reused.
- Added: ACC-039, ACC-040, LIF-030, OPS-029, IMP-010, IMP-011, IFC-034, IFC-035, CON-030,
  TPL-014 and SUB-010. IMP-010, CON-030, TPL-014 and SUB-010 are of the layer `language`;
  IFC-034 and IFC-035 of the layer `interface`; the others of the layer `convention`.
- 52 promises changed. No promise changed layer or level.
- Per file, where the count changed: access 33, lifecycle 30, operations 29, implicit 11,
  interface 35, conduits 26, templates 14, submissions 10.
- The implementation-defined list still has 15 items. DL3 was reworded, DL7 and DL11 each lost
  a point, and DL7 gained one.
- The core package exports 99 documents, one more, and has 1 implementation. The new export is
  the Operation contract `start-later-work.v1`, so `core/operations.yaml` holds 21 contracts.
  `retire-with-resources.v1` lost its output `remaining`.
- No schema changed shape.
- Examples: 59 files under `examples/invalid`, as before, and 17 under
  `examples/invalid-by-rule`, three more. The checker applies 16 rules, three more, which come
  from 14 promises.

## The decisions on what the closing pass left

One row for each row of the table of DECISIONS.md, by its number there.

| Decision | Promises and files changed | What changed |
| --- | --- | --- |
| 1. A Resource of a retired Pool | ACC-033, OPS-014, REV-010; definitions (`management`, `reconcile`); `core/operations.yaml` (the header, `inspect.v1`, `set-up.v1`, `retire.v1`, `reconcile.v1`), `core/capabilities.yaml` (comment), `core/README.md` | Where a Pool is retired, every request that ACC-033 authorizes against that Pool is authorized against `management` |
| 2. The Conduits of a retired consumer | CON-008, ACC-039 (new), ACC-003, ACC-011, LIF-015, LIF-016, LIF-024, LIF-026, LIF-028, TPL-009; definitions (`close`, `Conduit`, `permitted`); `core/operations.yaml` (`retire.v1`, its `effect`, and `close.v1`), `core/README.md` | A Conduit whose consumer is retired is closed once the consumer's shutdown is confirmed, and at once where the consumer is declared lost. Until then its state is as it was, and it permits only the requests of the consumer's retirement |
| 3. `retire.v1` of an instance whose shutdown is confirmed | LIF-030 (new), LIF-015, LIF-026, TPL-009; definitions (`retire`); `retire.v1`; `core/README.md` | It succeeds, dispatches no step and changes nothing |
| 4. A permitted request that names what does not exist | OPS-029 (new), LIF-012, OPS-014, ACC-031, ACC-033; definitions (the row for invalid); `core/operations.yaml` (the header, `rename.v1`, `unregister.v1`, `inspect.v1`, `set-up.v1`, `retire.v1`, `reconcile.v1`, `open.v1`, `close.v1`, `apply-policy.v1`, `remove-policy.v1`), `core/README.md` | It reports invalid, whatever the bound. `set-up.v1` or `retire.v1` that names a Conduit reports invalid, as for a template. OPS-015 is unchanged |
| 5. `#name` as `from` in a submitted project | NAM-022, INS-025; definitions (`#name`) | It resolves as the bare name does, among the templates that exist, those of the same submission among them |
| 6. An operation that a Conduit may list and for which no implementation is selected | TPL-004; `tools/check_schemas.py` (rule `implementation-selected`); new `invalid-by-rule/TPL-004-offered-operation-without-an-implementation.yaml`; `schemas/README.md` | TPL-004 covers every operation of every capability the type offers and every control the type lists. The core package and the valid examples satisfy it as they stood |
| 7. A requirement on a Conduit that does not narrow | CON-030 (new), CTR-005, CON-018, ACC-031; definitions (`requirement`, `policy`); `schemas/contracts.yaml` (descriptions), `schemas/README.md`, `core/requirements.yaml` (comment), `core/README.md`, `apply-policy.v1`; `tools/check_schemas.py` (rules `narrows-on-conduit` and `obliges-narrows`); new `invalid-by-rule/CON-030-requirement-that-does-not-narrow-on-a-conduit.yaml` and `invalid-by-rule/CTR-005-obliges-on-a-requirement-that-does-not-narrow.yaml` | A Conduit template that applies such a requirement, itself or as a constituent, is rejected unless the requirement is the core package's. A requirement that does not narrow requests and states `obliges` is rejected. Applying one as a policy reports invalid |
| 8. What `obliges: [consumer]` has the consumer's implementation do | IFC-034 (new), IFC-033, CTR-005, CON-022; definitions (`Conduit's keys`); `core/README.md`; the header of the interface file | The definition of a tool carries `restrictions` in its `_meta`: the Conduit's filters whose contract obliges the consumer. `python3 tools/set_names.py --check` reads the keys from the interface file and passes |
| 9. Later work | ACC-040 (new), IFC-035 (new), IMP-010 (new), IMP-011 (new), TPL-014 (new), ACC-019, ACC-001, IMP-006, IMP-008, IFC-006, IFC-012, IFC-014, IFC-021, IFC-025, IFC-032, CTR-002, PKG-020; definitions (`later work`, `start-later-work.v1`, `tool`, `tool name`, `operation identity`, `plain`, `core package`); `core/operations.yaml` (the new contract and the header), `core/conduit.yaml`, `core/capabilities.yaml` (comment), `core/README.md`; `spec/README.md`; `schemas/README.md` | The implementation that holds the work asks for each start with `start-later-work.v1`, a tool the engine always presents. ACC-040 says when the start is dispatched, and IFC-035 what the tool takes and returns |
| 10. Whether a registration installs packages | NAM-012, PKG-006, PKG-028, MAN-006; definitions (`install`); `register.v1`, `install.v1`; `core/README.md` | It does not. `fetch` in an item is the place an installation of that item fetches from |
| 11. The outcome of `retire-with-resources.v1` when Resources remain | LIF-021; `retire-with-resources.v1`, its `effect` and its outputs; `core/README.md` | Failed, with the diagnostics of the result naming what remains. See the choices and the open points |
| 12. Three implementation-defined items | DL3, DL7, DL11 | DL3 is in the terms of an invocation that does not end until an actor replies. DL7 and DL11 each lost the point that promises decide |
| 13. Whether every engine takes files | SUB-010 (new), SUB-005, DL7; definitions (`project`); `spec/README.md` | Every engine takes a directory, with the same result as `register.v1` given the values read |
| 14. The shape of an inspection result and of a diagnostic | LIF-020, LIF-008, IFC-025, OPS-005; definitions (`diagnostic`); `core/operations.yaml` (the header, `unregister.v1`, `retire.v1`), `core/README.md` | What a request that is not dispatched says beyond its reason, it says in diagnostics. The shapes are deferred to the conformance vectors, and `core/README.md` says so |
| 15. The Model Context Protocol | the next section | |

## What the check against the Model Context Protocol found

One row for each row of the last table of DECISIONS.md. Where the report and the decision
differ, the decision was applied: the specification conforms to the protocol.

| Finding | Promises and files changed | What changed |
| --- | --- | --- |
| Row 17. 128 characters are a recommendation | IFC-012, IFC-014; definitions (`tool name`) | The protocol says that a tool name should be at most 128 characters, and this specification makes that a limit |
| Row 19. A call to a tool the engine does not present | IFC-012, IFC-013, IFC-025, IFC-032, OPS-005; definitions (`result`, `tool`, `Conduit's keys`); `core/README.md` | The engine answers with the protocol's error for invalid parameters, code -32602, and no tool result. The error's `data` holds `reason` with the value refused, and the request is recorded as refused |
| Rows 21 and 22. `outputSchema` and a result with no outputs | IFC-025, IFC-009, IFC-033, IFC-035, OPS-007, OPS-009, ACC-034; definitions (`result`, `withheld`, the paragraph under "How a request ends"); `core/operations.yaml` (the header), `core/README.md` | `isError` is true on a result that the engine gives a consumer where it carries none of the operation's outputs: it ended with a reason, its outcome is failed or unknown, its content is withheld, or it is a repeat of an invocation still in progress. A result that is not an error conforms to `outputSchema`. IFC-025 says that the revision does not say whether an error result must conform, and that this specification takes it that it need not |
| Row 28. HTTP 401 | IFC-032 | Such a request is answered as the protocol requires, with no tool result, and is recorded as refused |
| Rows 11, 15, 16, 23 and 24 | IFC-007, IFC-010, IFC-012, IFC-025, IFC-016 | The report's wording for each |
| What the revision requires and the interface does not mention | IFC-001, IFC-010, IFC-013; `schema-profile.md` | Each side keeps every requirement of the protocol at the revision in use, and the specification repeats none. Where the tasks extension is used, the table of IFC-010 applies to the task's final result. A consumer may hold a tool list for as long as the protocol lets it |

IFC-003 and IFC-024 needed no change: the report confirms what IFC-003 says of an unsupported
revision, and the result of the retrieval tool is not an error.

## Choices made where a decision left the wording open

Each can be struck by the owner.

| Where | Choice | Why |
| --- | --- | --- |
| Decision 2, ACC-039 | What the Conduit permits is its own promise, in the access convention. A request "serves" the retirement where it is a step of the invocation of `retire.v1` that names the Conduit, or an onward request that names, in `serving`, the invocation of a step of the `retire` graph or of the stop of a runtime | CON-008 keeps one obligation, when the Conduit is closed. A step with no `conduit` and a stop invoke the consumer's own implementation, which then calls onward |
| Decision 2, CON-008 | The Conduit's state is unchanged until it is closed, and its tools are still presented | LIF-024 confirms a Conduit's shutdown when it is closed, so a Conduit that is not closed is not retired |
| Decision 3 | A new promise, LIF-030, beside LIF-026 | One obligation to a promise |
| Decision 4, OPS-029 | Placed in the operations convention. It lists `replace.v1` too, for the template its document names. A request for `start-later-work.v1` that names no invocation is not such a request, and reports refused | SUB-009 already gave invalid. No Conduit lists `start-later-work.v1`, and decision 9 gives refused |
| Decision 5 | A nested instantiation that writes its `from` as `#name`, in a template of the project or in a document that replaces one, resolves in the same way whenever the template that holds it is instantiated | The decision says that a later instantiation or a replacement resolves like any bare name |
| Decision 6, TPL-004 | It also names the runtimes of the type, beside those of the template | TPL-006 gives an instance the runtimes of its type, and a step may start one |
| Decision 7, CTR-005 | A requirement that does not narrow requests and states `obliges` is rejected whatever the list holds, an empty list included | The decision says "`obliges` on a requirement that does not narrow", and an engine never accepts a document and ignores part of it |
| Decision 7, CON-030 | A composition none of whose constituents narrows is itself a requirement that does not narrow, so one made only of `shared-host.v1` is rejected. `shared-host.v1` may be a constituent of a composition that narrows | It is the decision read to the letter: such a composition is not a contract of the core package |
| Decision 8, IFC-034 | Where the Conduit carries no filter that obliges the consumer, the definition holds no such key. A composed filter is written as IFC-018 sends it. `obliges` is read from the contract of the requirement the Conduit applies | A tool of an ordinary Conduit is unchanged. CON-022 reads `obliges` from the same contract |
| Decision 9, IFC-035 | A call begins one invocation, the one it starts, and its result is that invocation's: its outcome and the engine's ID for it in `_meta`, and the started operation's outputs under the one declared output `outputs` | An Operation contract declares fixed outputs, and `retrieve-result.v1` holds another operation's outputs in the same way |
| Decision 9, `start-later-work.v1` | `operation` is a string, a qualified reference or `<digest>#<name>`. `arguments` is optional. The contract states an `effect` and no `deduplicates` | IFC-006 writes an operation so on the wire. The engine invokes another implementation, as it does for `set-up.v1` |
| Decision 9, ACC-040 | The restrictions sent with a start are the filters of the Conduit and the policies that apply to a request of that caller to that instance, as they stand when it is dispatched. A request that names an ID of no invocation, of an invocation of another instance, or of one that came through no Conduit reports refused, and says nothing of whether the invocation exists | "With the restrictions as they then stand", and "otherwise the request reports refused". OPS-015 gives the same reason and the same silence |
| Decision 9, IMP-010 | It covers a graph that implements an operation, as IMP-004 does. A `setup` or `retire` graph is not rejected | The decision names the analog of IMP-004 only. See the open points |
| Decision 11, LIF-021 | The result lists what remains in diagnostics, and `retire-with-resources.v1` no longer declares the output `remaining` | Decision 11 has the result list what remains, and the decision on rows 21 and 22 has a result whose outcome is failed hold no outputs. Diagnostics are what such a result may carry |
| Decision 11, LIF-021 | The Pool's retirement is an invocation of `retire.v1` of its own, and where it is dispatched the outcome of the request is the outcome of that retirement | LIF-021 says that the request to retire the Pool is dispatched, and the decision makes the Pool's retirement the effect of the operation |
| Decision 1, ACC-033 | The sentence reads "where a Pool is retired", so it also covers the reconciliation of an invocation of `retire-with-resources.v1` and of the Pool's own retirement, which the Pool authorized | Otherwise such an invocation with an unknown outcome could never be reconciled: every Conduit to a retired Pool is closed |
| Decision 3, LIF-030 | Where another promise has a request to retire the instance not dispatched, that promise governs: refused under LIF-022, conflict under LIF-020 or PLC-011 | LIF-030 says that the request is decided like any other |
| Decision 13, SUB-010 | Of the layer `language`. The promise and DL7 say "takes a directory" and "how a directory is handed to an engine", and not "entry point". DL7 also has the engine name the actor for which it then registers | The rules on files are language promises. The definitions keep "entry" for a Pool's entry. A registration is authorized under ACC-033, so it has a caller |
| Decision 14, IFC-025 | A request that is not dispatched carries `diagnostics` in its `structuredContent` whether or not the operation's contract declares an output of that name | `retire.v1` and `unregister.v1` need no output |
| Rows 21 and 22, IFC-025 | `isError` is also true on a repeat of an invocation that succeeded and whose outputs the engine no longer keeps. Such a result has the outcome succeeded and neither `withheld` nor `state` | Such a result holds no outputs, and one that is not an error must conform to `outputSchema` |
| Rows 21 and 22, IFC-025 | The result of an operation that declares no outputs has no content to withhold: it is never marked as withheld, and where its outcome is succeeded its `isError` is not true | The definitions say withheld of "the content of a result that exists". A successful `retire-with-resources.v1` always ends after its Conduit is closed |
| Rows 21 and 22, IFC-009 | An implementation sets `isError` to true on a result that states failed or unknown. The engine takes the outcome from the key `outcome` and not from `isError`, and sets `isError` on its own result to a consumer | IFC-008 and IFC-026 already had two such results carry `isError`. One rule for which of the two governs |
| Row 19, IFC-032 | The engine presents no tool to a client from which it establishes no instance, so each call of such a client is answered as a call to a tool that is not presented | IFC-012 presents tools to an instance |

## Found in the read-through and fixed

Each fix follows from the text or from DECISIONS.md.

| Found by | What disagreed | Fix |
| --- | --- | --- |
| Editor | OPS-007 gave the outcome unknown to any result that lacks a declared output, and IFC-008 has an implementation answer failed with no outputs | OPS-007 checks the declared outputs of a result that would otherwise have the outcome succeeded |
| Editor | REV-010 authorized the reconciliation of a control against "`management` otherwise", which left out a Resource whose Pool is retired | It adds the case, as ACC-033 states it |
| Editor | LIF-008 had a step "name the retired instance", with no place to name it | A diagnostic names it |
| Editor | The `effect` of `retire.v1` said that it closes the Conduits of which the instance is an end | The Conduits of which it is the target at once, and those of which it is the consumer once its shutdown is confirmed |
| Editor | The `effect` of `retire-with-resources.v1` was the retirement of each Resource and then of the Pool, which made a failure in part an unknown outcome under OPS-004 | The effect is the Pool's retirement, and each Resource's retirement is an invocation of its own |
| Editor | IFC-006 said that `operationIdentity` is sent "when the caller supplied" one, and that `conduit` is absent for a step of the instance's own implementation; neither covered a start of later work | IFC-006 says what a start is sent |
| Editor | `requirement-spec.v1` said that a composition is sent "as one item for each constituent", and IFC-018 says for each constituent that narrows requests | The description follows IFC-018 |
| Editor | PKG-020 and CON-022 did not point to the new rules on a plain implementation and on how a consumer learns of a filter | Each cites TPL-014 or IFC-034 |
| Reader | LIF-021 had a failed result list what remains in the output `remaining`, and IFC-025 has a result whose outcome is failed hold no outputs | The result lists them in diagnostics, and the operation declares no output. See the choices |
| Reader | The observable of IFC-025 said that the keys beside `isError` say why, and a repeat whose outputs are no longer kept has none of them; the header of `core/operations.yaml` listed four cases | IFC-025 says how that result is told apart, and the header lists five |
| Reader | An error result "carries none of the operation's outputs" and "may carry `diagnostics`", which `register.v1` and three other operations declare as an output | IFC-025 and the definitions say "`diagnostics` apart" |
| Reader | SUB-010 gave a directory the same result as `register.v1` given the values read, which rejects a relative `fetch` | The same result once each relative `fetch` and program is resolved against the directory |
| Reader | ACC-040, IFC-035 and two core documents said that a start which is not dispatched reports refused, and the same promise has other checks give invalid or unavailable | Refused where one of the listed conditions does not hold; another check gives its own reason |
| Reader | LIF-024, LIF-026, ACC-011 and the definitions said that the Conduit of a retired consumer is closed "only once" the shutdown is confirmed, and CON-008 also closes it where the consumer is declared lost or an actor asks | Each names the other ways, and LIF-026 says what a step reports where the Conduit is closed |
| Reader | LIF-030 and LIF-020, or PLC-011, gave succeeded and conflict for one request | LIF-030 says that the other promise governs |
| Reader | After `retire-with-resources.v1` no promise said which invocation LIF-024 counts for the Pool, or how one that the retired Pool authorized is reconciled | LIF-021: the Pool's retirement is an invocation of `retire.v1`. ACC-033: `management` authorizes in the place of any retired Pool |
| Reader | The `then` of LIF-021 dispatches a request to retire each Resource, and a clause the editor had added to its observable spoke of one that was not dispatched | The clause is gone |
| Reader | ACC-003, the definitions' rows for Conduit and permitted, and LIF-016 still stated the rule that ACC-039 narrows; the description of `object-management.v1` left out a Resource whose Pool is retired | Each names the exception |
| Reader | IFC-009 cited IFC-026 for a result that states failed or unknown, and that result gives a reason | The citation says so |
| Reader | "Entry point" in SUB-010, DL7 and the definitions, where the definitions keep "entry" for a Pool's entry | "Takes a directory", and "how a directory is handed to an engine" |

The reader found nothing to report on the vocabulary, on the citations beyond the one above,
on the counts of `spec/README.md`, or on decisions 4 to 8, 10 and 12 to 14 beyond what is
listed here and below.

## Still needs a decision

Where two promises collide, or a choice above settles a collision between two decisions, both
are named. Nothing below was decided beyond what its row above says.

1. **`remaining` against the rule on `isError`.** Decision 11 has the result of
   `retire-with-resources.v1` list what remains where its outcome is failed. The decision on
   rows 21 and 22 has a result whose outcome is failed hold no outputs. The editor put the
   list in diagnostics and removed the output `remaining` (LIF-021, IFC-025). The other ways
   are to except this output from IFC-025, or to change the outcome.
2. **A Resource registered into a retired Pool.** No promise rejects a Resource whose owner is
   retired: PLC-018 rejects a retired location as a placement and CON-025 a retired end, and
   POOL-004 accepts any Pool. ACC-033 has such a registration "also" need a Conduit to the
   Pool. Read with decision 1, a Conduit to `management` then suffices, and a retired Pool can
   gain Resources and replacements for its entries under LIF-009. Read the other way, the
   request is refused for good, because every Conduit to a retired Pool is closed.
3. **Later work arranged by an onward request.** ACC-017 has an onward request carry "the
   filters on the Conduit the caller came in through", and ACC-040 sends a start "the filters
   of that Conduit and the policies that apply", as decision 9 words it. A start then runs
   without the filters that the arranging request carried, and goes on after the Conduit that
   the caller came in through is closed, because ACC-034 stops only the onward requests of an
   invocation in progress.
4. **Later work arranged by a step that has no `conduit`.** IFC-006 sends such a step no
   Conduit, so ACC-040 refuses every start of the work. ACC-018 ties the step's restrictions
   to the invocation that its graph is serving; whether the work is tied to that invocation's
   Conduit in the same way is not said.
5. **A `setup` or `retire` graph that holds a step whose operation is marked
   `arrangesLaterWork`.** IMP-005 and IMP-009 reject the like for model work. Decision 9 names
   only the analog of IMP-004, so no promise rejects the template. The step is refused when it
   runs, under IMP-011, where it names a Conduit, and otherwise arranges work of which no start
   is dispatched (IMP-010).
6. **A start that names an arranging invocation whose outcome is failed**, or that was
   reconciled as not having occurred. The conditions of ACC-040 are met, and OPS-003 says that
   nothing was arranged.
7. **How a consumer learns the result of later work.** IFC-016 has the result of work that
   finishes after its call returns "retrieved later under the operations convention", by the
   operation identity and the target of the request. Under ACC-040 each start is an invocation
   whose identity is scoped to the instance that asked for it, so the consumer that arranged
   the work has no invocation to retrieve (OPS-015). IFC-016 names neither `arrangesLaterWork`
   nor ACC-040, and the comment on `directory.sweep.v1` in the example package cites it.
8. **A Conduit in the state registered whose consumer is retired with its shutdown not
   confirmed.** CON-008 now leaves it as it was. ACC-006 and CON-023 forbid opening only to a
   retired target, so `open.v1` opens it and ACC-039 limits it. CON-025 refuses to instantiate
   a Conduit for a retired consumer, which points the other way.
9. **A composition made only of requirements of the core package that do not narrow**, such as
   one whose single constituent is `shared-host.v1`. CON-030 rejects it, by the letter of
   decision 7. The engine could check it.
10. **A constituent whose contract names `consumer` in `obliges`, in a composition that states
    no `obliges`.** CON-022 and IFC-034 read `obliges` from the contract of the requirement the
    Conduit applies, so the consumer is not obliged.
11. **Whether an implementation's result that states succeeded with `isError` true, or failed
    with `isError` not true, is a contradiction.** IFC-009 now has the engine take the outcome
    from the key. IFC-009 gives the outcome unknown to a result that holds both an outcome and
    a reason, which is the nearest case.
12. **The shape of an inspection result and of a diagnostic**, deferred by decision 14 to the
    conformance vectors and known for round 3. LIF-020 and LIF-021 now rely on a diagnostic
    naming a Resource, and no promise says how.
13. **How a value is marked as secret.** Left for the owner, as before.
14. **The choices above**, each of which the owner can strike.

## Outside this pass

- `docs/AUDIT.md` and `docs/README.md` state counts that this pass changed: 421 promises, 99
  exports, 21 Operation contracts, 17 files under `examples/invalid-by-rule`. The coordinator
  updates them.
- The comment on the component `settings` in `examples/valid/project/templates.yaml` says that
  `#directory-settings` "reaches a template of the same project". It is true of that project.
  NAM-022 now says how: as the bare name does.
- The check of the protocol found nothing wrong in IFC-004. This pass added to it the `_meta`
  of a tool's definition, on which IFC-034 relies; the report says that the released schema
  has it (`Tool { ...; _meta?: MetaObject }`).

## Checks

Run from the repository root on 2026-11-06, after the last edit. Each exited with status 0.

```text
$ /usr/bin/python3 - (the validation command of CONSOLIDATION.md)
421 promises ok

$ (every promise ID and DL item cited in a .yaml or .md file under spec/, file names
  included, exists)
129 files; 3578 citations of promises, 53 of DL items; 421 promises and 15 DL items exist
promise IDs cited and missing: {}
DL items cited and missing: {}
named only in header comments of promise files, as removed: ACC-004 ACC-007 ACC-010 ACC-013 ACC-020 ACC-021 ACC-025 CON-005 CON-009 CON-010 CON-015 FST-004 GRA-013 GRA-016 INS-015 PKG-002 PKG-008 PKG-009 PKG-010 PKG-011 PKG-015 PLC-010 PLC-014 POOL-008 POOL-010

$ python3 tools/check_schemas.py        (first line, the line of each step, last line)
Python 3.9.6 at /Applications/Xcode.app/Contents/Developer/usr/bin/python3; PyYAML 6.0.1; jsonschema 4.20.0
1. Read 100 files holding 224 documents as strict YAML.
2. Checked 58 Schema contracts against the profile, and every `$ref`.
3. Validated 137 documents under spec/schemas/, spec/core/ and examples/valid/.
4. 59 invalid documents in 59 files were each rejected by a schema, or rejected when read.
5. Core package: 99 documents exported, 0 documents deliberately private and 1 implementation, which is also an export. Checked 434 references for the kind they name; 1 name packages that are not in this repository and were not checked. Checked 3 controls, the 5 operations that take no `revision`, the tables of the two READMEs, and 799 citations of promises in 103 files. The example packages keep 0 documents private.
6. Applied 16 rules of this tool, which come from 14 promises, to the core package and 3 example directories. 17 sets of documents in 17 files under examples/invalid-by-rule/ were each rejected by the rule they are named for; each of their 29 documents is valid against the schemas.
   SCH-019: validated 10 defaults of the core package and the valid examples against their schemas; 0 were not validated, because a `$ref` in the schema leads outside this repository.
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x431, apiVersion x227, metaPrefix x89. Errors: 0

$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 121 Markdown files and 1542 local links (offline).
Errors: 0

$ git diff --check
(no output; exit status 0)
```

`git diff --check` does not read a file that Git does not track yet. The four new files, this
log and the three examples under `invalid-by-rule`, were searched for trailing spaces apart,
and have none. No line of a YAML file under `spec/` is longer than 96 characters.

Every number in the Contents table of `spec/README.md` was counted again by a script after the
last edit: the promises of each of the 22 files and their total of 421, the 15
implementation-defined items, the contracts and templates of each file of `core/` and of
`schemas/`, the 99 exports and 1 implementation, and the 7, 59 and 17 example files. All
agree.

The new rule of the checker for TPL-004 was also tried on the valid documents with one default
implementation taken away, in memory: it then rejects the template `project-directory` of the
example project and the template `location` of the core package, so it is applied to them and
they satisfy it as they stand.

Not verified: nothing was checked against the Model Context Protocol in this pass. What the
interface now says of the protocol rests on the report of the check and on the decisions on
it.
