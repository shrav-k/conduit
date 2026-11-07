# Round 2: log of the closing pass

Written 2026-11-06 by the editor of the closing pass. The instructions were the last section of
[the decisions](DECISIONS.md), "The fix pass, and what it left"; the two logs of the fix pass,
[promises](log-promises.md) and [documents](log-documents.md), with every finding they mark as
not applied or applied in part and every point they leave open; the reports of
[reader 1](reader-1.md), [reader 2](reader-2.md) and [reader 3](reader-3.md); findings A2 and
B1 of [reader 4](reader-4.md); and the rules of [CONSOLIDATION.md](../CONSOLIDATION.md).

Accepted by the owner in [ruling 39](../../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

Edited: files under `spec/`, `tools/check_schemas.py`, `docs/OVERVIEW.md`, and, for counts and
the status of round 2 only, `docs/AUDIT.md` and `docs/README.md`. Nothing was committed.

After the decisions and the findings were applied, the specification was read through once
more for disagreements. Two further readers, who were given the text and not this log, read
it cold at the same time: one read the promises and the definitions, and the other the core
package, the schemas, the examples, the READMEs and the overview. What they and the editor
found is under [Found in the read-through](#found-in-the-read-through-and-fixed) and
[Still needs a decision](#still-needs-a-decision).

Finding IDs are those of the two earlier logs: `R1-n.m`, `R2-Tn`, `R2-Sn`, `R2-Cn`, `R2-Gn`,
`R2-Stn` and `R3-n`.

## Result

- 410 promises, 4 more than after the fix pass. None was removed and no ID was reused.
- Added: TPL-013, PKG-026, PKG-027 and PKG-028. TPL-013 and PKG-027 are of the layer
  `convention`; PKG-026 and PKG-028 are of the layer `language`.
- 76 promises changed. No promise changed layer or level.
- The implementation-defined list still has 15 items. DL7, DL10 and DL12 each gained a point.
- Per file, where the count changed: templates 13, packages 22.
- The core package still exports 98 documents and has 1 implementation. `retrieve-result.v1`
  gained the output `invocation`; `location-settings.v1` and the type `location.v1` lost
  `reconcile.v1`.
- Schemas: `operation-spec.v1` gained `arrangesLaterWork`; `runtime.v1` lost `optional`.
- Examples: 59 files under `examples/invalid` (one added, one renamed), 14 under
  `examples/invalid-by-rule`. The example package gained the Operation `directory.sweep.v1`.

## The decisions on what the fix pass left

One row for each row of the last table of DECISIONS.md.

| Decision | Promises and files changed | What changed |
| --- | --- | --- |
| Which Conduit permits reconciling an operation that is not a management operation | ACC-033, OPS-014, LIF-015, LIF-018, LIF-028, REV-010; definitions (`reconcile`, `management`); `core/operations.yaml` (`reconcile.v1` and the header), `core/capabilities.yaml`, `core/types.yaml`, `core/README.md`; `examples/valid/files-package/contracts.yaml` | An invocation of a management verb is reconciled through the target that authorized it. Any other is reconciled through the Pool that owns the instance it was dispatched to, or `management`. `reconcile.v1` is out of `location-settings.v1`, of the `requires` of `location.v1`, and of the example capability and its type. No capability has to include it beside the three that hold management verbs. The checker needed no rule for it |
| A filter that obliges only the consumer | CTR-005, CON-022, ACC-038, IFC-018; definitions (`requirement`); `schemas/contracts.yaml` (`obliges`), `schemas/README.md`, `core/requirements.yaml`, `core/conduit.yaml`, `core/README.md`, the comment on `rate-limit.v1` in the example package | The target's implementations always state a filter in `supports` and are always sent it. `obliges` that names `consumer` adds the consumer's |
| Naming the implementation that applies a requirement when it has no address | ACC-038; definitions (`inspect`, `unpublished package`); `inspect.v1`; `core/README.md` | By qualified reference, or `<digest>#<name>` for one that the manifest of a project that is not published lists |
| How an operation says that it arranges later work | CTR-002, ACC-019, IMP-006; definitions (`later work`); `operation-spec.v1`; `core/operations.yaml` (header), `core/README.md`; `schemas/README.md`; the example Operation `directory.sweep.v1` | The mark `arrangesLaterWork: true`. Without it an operation arranges none. A later start of model work needs `startsModelWork: true` as well |
| Renaming one of `root`'s two Conduits | LIF-022, FST-002, OBJ-026; definitions (`rename`, `first-start object`); `rename.v1`; `core/templates.yaml` (comment), `core/README.md` | Refused, as renaming a first-start object is |
| The example that writes out the first-start objects | FST-006; the header of `examples/valid/first-start/first-start.yaml`; `schemas/README.md` (the Examples table and a new row of "What a schema cannot express"); `core/README.md`, `core/templates.yaml` | The example stays valid against the schemas. Its header and the README say that it shows what first start makes and that submitting it is rejected, under FST-006 and OBJ-002 |
| A consumer that lost a result cannot learn the engine's ID | IFC-024, IFC-025, OPS-014, OPS-016, REC-004; definitions (`retrieval tool`, `the engine's ID for an invocation`); `retrieve-result.v1`, `reconcile.v1`, `purge.v1`; `core/README.md` | `retrieve-result.v1` returns `invocation`. A purge keeps that ID, so the tool can still return it |
| What `optional` means on a runtime and on a graph's output | OBJ-015, TPL-006, GRA-021, GRA-026, OPS-022; definitions (`optional`, `runtime`, `output`); `runtime.v1`, `graph.v1`; the example package's type, the example project's template, `invalid-by-rule/TPL-006-…`; new `invalid/TPL-006-runtime-marked-optional.yaml` | A runtime carries no `optional`, and the schema rejects one. An optional output is one a graph may leave without a value: the result then holds no such output |
| What `actors` and `targets` of a policy do | ACC-031, ACC-009, ACC-015, ACC-026, ACC-035, LIF-001; definitions (`policy`, `root Interface`); `apply-policy.v1`, `remove-policy.v1`; `core/README.md` | A policy applies to a request whose caller is one of `actors`, where given, and whose target is one of `targets`, where given. A request that gives neither reports invalid |
| Retiring a Pool with its Resources | LIF-021, ACC-033; `retire-with-resources.v1` | The Conduit need list only `retire-with-resources.v1` |
| A re-exported operation in a bound | ACC-029; definitions (`bound`); `bound.v1` and the comment in `core/requirements.yaml` | Naming the re-export or its origin names both |
| A relative `fetch` or `command` without a directory | MAN-006, MAN-010; definitions (`fetch`); `schema-profile.md`; `register.v1`, `install.v1`; `schemas/manifest.yaml` (comment), `schemas/README.md` | Resolved against the directory the manifest was read from, and rejected where there is none |
| "Plain" for the built-in Interface and Pool types | none in `spec/`: every use of "plain" was already of an implementation, and the two types were already "basic". `tools/check_schemas.py` | The checker's internal name for a place in a schema is now `other` |
| Every other finding an editor did not apply | the next table | |

## Findings finished

One row for each finding that a log of the fix pass marks as not applied or applied in part.

| Finding | Promises and files changed | What was done |
| --- | --- | --- |
| R1-4.5 "The owner" as a person | none | The reader says no change is needed |
| R2-T13 Reserved words in an ordinary sense | `schema-profile.md`, `spec/README.md`, `tools/check_schemas.py`, `core/README.md`, `core/operations.yaml` | The two headings are now "Schema keywords in the profile" and "Formats that are checked", and the checker reads them by those names. "A retained result belongs to the Conduit"; `settings` is "a map". The key `state` keeps its two uses, which the definitions state |
| R2-G5 The inspection result | none | Not applied. The reader gives no fix, and the full shape needs a decision. `core/README.md` still lists what is not fixed |
| R2-G6 `optional` on a runtime and an output | as the decision above | |
| R2-G7 Two operations of one export name in one implementation | IFC-001 | Such an implementation has one tool of that name and tells the two apart by the key `operation` that IFC-006 sends. A plain one has its `tools` map |
| R2-G8 `actors` and `targets` | as the decision above | |
| R2-St1 Sections of the Model Context Protocol | none | Not applied. The sections and the error code of revision 2026-07-28 cannot be checked from this repository |
| R2-St2 Which tar format | PKG-022; DL7; `schema-profile.md` | Added to DL7 as a point each engine documents |
| R2-St5 Smaller points on standards | none | The reader gives no fix and says no result changes |
| R3-9 A policy on `access` or `management` | ACC-035, ACC-031; definitions; `apply-policy.v1` | See the choices below |
| R3-11 The rules on files have no place on the wire | SUB-005, MAN-006, MAN-010; `register.v1` | The rules on a file apply where the engine is handed files. A registration that hands over values has no file to check. Whether an engine must offer an entry point that takes files is still open |
| R3-13 What the result carries beyond `_meta` and `structuredContent` | OPS-009, OPS-016, IFC-024, IFC-025; DL10, DL12 | A repeat sent through a closed Conduit reports refused. The result of a repeat of a succeeded invocation is its retained result. How long the content of a retained result is kept is DL10. `content` has no meaning here, a withheld result holds none of the implementation's, and the rest is DL12 |
| R3-15 `install.v1`: three cases and dependencies | PKG-017, PKG-023, PKG-026, PKG-027, PKG-028 (three new); definitions (`install`); `install.v1` | See the choices below |
| R3-28 A control its template leaves out | TPL-013 (new), TPL-010 | The request reports invalid |
| R3-29 Endings with no stated reason | IFC-012, ACC-006; definitions (`tool`); `open.v1` | A call to a tool the engine does not present reports refused. `open.v1` on an open Conduit succeeds and changes nothing; on a closed one it reports conflict |
| R3-30 A plain server that returns no required output | OPS-007, PKG-020 | The outcome is unknown. The `tools` map gives a tool's name only |
| R3-31 "Required" for a runtime | as the decision on `optional` | |
| R3-32 `retire-with-resources.v1` and `retire.v1` | as the decision above | |
| R3-33 A policy with neither list | as the decision above | |
| R3-34 A key of `settings` outside `changes` | REV-007, REV-013 | In a control's result it makes the outcome unknown and nothing is stored. In a reconciliation it reports invalid |
| R3-36 A re-exported operation in a bound | as the decision above | |
| R3-38 An entry's placement from the default rules | POOL-016, PLC-021 | A placement the default rules chose is not written into the entry, so a replacement is placed by the rules as they then stand. A requirement on placement is tested when an instance is instantiated and not again |
| R3-40 Values that had to be guessed | IFC-025, ACC-006, IFC-020, IFC-018 | `isError` is not true on the result `inProgress`. A refusal for the bound names it in `restriction`. `serving` is not sent on. Two equal restrictions are two equal items |

## What the two logs left in disagreement or open

| Point | Where it is settled |
| --- | --- |
| CON-022 against IFC-018 and IFC-019 | The decision on `obliges` |
| ACC-038 and an implementation with no qualified reference | The decision on naming it |
| IFC-025 and `retrieve-result.v1` | The decision on the engine's ID |
| FST-006 and the first-start example | The decision on the example |
| R3-7, reconciling a control or an ordinary operation | The decision on reconciling |
| R1-3.1, how a contract says that it arranges later work | The decision on `arrangesLaterWork` |
| R3-8, renaming one of `root`'s Conduits | The decision on it |
| The words of R2-T8 | Already "basic" and "literal values" everywhere |
| The counts in `spec/README.md` | Recounted: see the checks |
| A promise for `nested` in `object.v1` | OBJ-003 now says that the result of `register.v1` says of a nested instance which nested instantiation made it. The shape of the member is still the schema's |

## Choices made where a reader gave options or no wording

Each can be struck by the owner.

| Where | Choice | Why |
| --- | --- | --- |
| R3-9, ACC-035 | No policy applies to a request whose caller is the root Interface. A request whose `actors` names it is refused. A policy that gives only `targets` is applied and does not limit the root Interface | The definitions already said that no policy is applied to it. The other readings either refuse every policy that names only targets or let a policy on `access` lock every actor out for good |
| R3-33, ACC-031 | A list that is given and is empty matches no request | It is what the decision says when read to the letter, and adds no rule to the schema |
| R3-15 (i), PKG-026 | An archive whose digest is not the one the request names is rejected, so the request reports invalid | PKG-021 already refuses such an archive for a manifest item |
| R3-15 (ii), PKG-017 | A manifest that states another version than the one asked for is rejected, as one that states another address is | It extends the sentence the reader cites |
| R3-15 (iii), PKG-027 | Installing what is already installed with the same digest succeeds and changes nothing | The effect `install.v1` describes has occurred |
| R3-15 (iv), PKG-028 | A package that lists a package that is not installed is installed, and the missing one is reported | It is what PKG-006 does for a template. Nothing is fetched because another package lists it |
| R3-28, TPL-013 | invalid | The instance does not have the control. LIF-012 gives invalid for a request that fits no object of that sort |
| R3-29, a tool that is not presented | refused | It is a request that no open Conduit permits, which is ACC-001 |
| R3-29, `open.v1` on an open Conduit | succeeded, with no change | The step of a setup is left as succeeded in the same case (CON-004) |
| R3-13 (ii), OPS-009 | The result of a repeat of a succeeded invocation carries the outputs where the engine still keeps them | It is the retained result, which the definitions already describe |
| R3-13 (iii), R2-St2, R3-13 (i) | Retention of a result's content, the tar formats read, and the `content` of a result went into DL10, DL7 and DL12 | Each adds no behavior, as DL5 and DL15 did in the fix pass |
| R3-40, ACC-006 | "Names what is outside the bound" became "names the bound in `restriction`" | No key could carry the first, and ACC-029 already states the second |
| The decision on the engine's ID, REC-004 | A purge keeps the engine's ID for an invocation | Otherwise `retrieve-result.v1` could not return it after a purge, and an unknown outcome could not be reconciled |
| The decision on `optional`, GRA-021 | A graph that marks as optional an output the contract does not make optional is rejected | GRA-021 allows the mark only on such an output, and an engine never accepts a document and ignores part of it |
| The decision on `arrangesLaterWork` | The example package gained one Operation that carries the mark | Every other key of an Operation contract is shown in a valid example |
| The decision on `optional` | One new invalid example, of a runtime that carries `optional` | The schema now rejects it, and the checker shows that it does |

## Found in the read-through and fixed

Each fix follows from the text or from DECISIONS.md. "Editor" is the editor's own reading;
"reader A" read the promises and the definitions, and "reader B" the documents.

### Between promises

| Found by | What disagreed | Fix |
| --- | --- | --- |
| Reader A | REV-006 lets every input of a control but `revision` be left out; OBJ-015 and IFC-033 made an unmarked one required | REV-006, OBJ-015 and IFC-033 say that a control's inputs other than `revision` may be left out, marked or not |
| Reader A | LIF-017's `then` said "retired because its location was declared lost", and its observable, PLC-017 and the definitions said "its location was declared lost" | LIF-017, LIF-024, LIF-018, PLC-017 and `unregister.v1` now say the second. An instance whose shutdown was confirmed stays confirmed |
| Reader A | LIF-024 and LIF-018 judged the shutdown by "the invocation of `retire.v1` that retired it", and LIF-026 allows a further one | LIF-024: confirmed once an invocation has succeeded, otherwise by the latest. LIF-018: or a further retirement succeeds |
| Reader A | LIF-015's closed list left out reconciling an invocation dispatched to a retired instance, which OPS-013 needs | LIF-015 lists it, where the reconciliation stores nothing in the instance |
| Reader A | OPS-017 withheld content that OPS-016 and IFC-024 say is no longer kept | OPS-017 holds where the engine still keeps the content |
| Reader A | OPS-012 and OPS-024 cited OPS-009 for the definition of a repeat, and OPS-009 covers only an invocation in progress or succeeded | Both state the match themselves |
| Reader A | IFC-018 sent one item for each constituent, and also said a requirement that does not narrow never appears | One item for each constituent that narrows requests |
| Reader A | IFC-029 gave a consumer's request with keys under two prefixes an outcome, though it is not dispatched | Invalid for a request the engine receives; the rule of IFC-008 for one an implementation receives |
| Reader A | ACC-038 named an implementation of an unpublished project by a digest that SUB-008 gave only to a project with contracts | SUB-008 and NAM-009 cover a project whose manifest lists implementations |
| Reader A | CTR-005's `then` had the target's implementations state every requirement; CON-022 limits that to a filter | CTR-005 says "for a filter" |
| Reader A | ACC-019 said "the caller's access", IMP-006 "that actor's access", of the same Conduit | Both say the access that the Conduit carries |
| Editor | IFC-024 returns `invocation` for any invocation, and REC-004 let a purge drop that ID | REC-004 and `purge.v1` keep it |
| Editor | OPS-009 gave a repeat the outputs even where it came through another Conduit than the invocation's | The outputs are given where OPS-017 does not withhold them, and IFC-025 cites OPS-017 |

### Reasons that no promise named

| Found by | Request | Now |
| --- | --- | --- |
| Reader A | `reconcile.v1` for an invocation whose outcome is not unknown | conflict (OPS-014) |
| Reader A | `close.v1` on a closed Conduit | Succeeds and changes nothing, as `open.v1` on an open one (ACC-012) |
| Reader A | `rename.v1` to a name another object has, or to a name that breaks OBJ-001 | conflict; invalid (OBJ-026) |
| Reader A | `replace.v1` with a document that is rejected, or of another kind or name | invalid (SUB-009) |
| Reader A | `apply-policy.v1` with a requirement that does not apply as a policy, or with arguments that do not fit | invalid (ACC-031) |
| Reader A | A control that the instance's type does not list | invalid: TPL-013 covers it as it covers one the template leaves out |
| Reader A | A registration whose manifest, or whose submission as a whole, is rejected | invalid: OBJ-023 names all three |
| Reader A | A package one of whose documents is rejected; `install.v1` with a relative `fetch` | The package is rejected; invalid (PKG-023) |

### Documents against promises

| Found by | Where | Fix |
| --- | --- | --- |
| Reader B | `open.v1` and `close.v1` gave the general condition for a Conduit to `access` too | Both say that ACC-030 states the condition in its place |
| Reader B | "No type has to offer `reconcile.v1`", though every Pool type offers a capability that includes it | "No capability has to include it beside the three", in ACC-033, the definitions and `core/README.md` |
| Reader B | The definitions' row for `management` left out reconciling a management verb | Added |
| Reader A, reader B | `replace.v1` declares the input `manifest`, which no promise mentioned | SUB-009 states it, and that NAM-007 and NAM-009 apply |
| Reader B | LIF-020 has the result name the Resources in the way; `retire.v1` and `unregister.v1` declare no output | Listed in `core/README.md` among what the package does not settle |
| Reader B | `bound.v1` did not say that it also limits policies | Added |
| Reader B | `core/templates.yaml`: "the one export a promise names" | Removed |
| Reader B | `core/results.yaml` and the export table said no promise bears on `object.v1` | Both cite OBJ-003 |
| Reader B | `core/README.md`: "the two values every status carries" | The lifecycle state, and the shutdown of a retired instance |
| Reader B | The descriptions of `object-management.v1` and `pool-management.v1` left out `register-replacement.v1` | Added |
| Reader B | `schemas/README.md`: `plain` listed as a choice of the schemas, though MAN-010 states it; a table row broken over two lines; "beside its manifest" of a directory that has none; "rejected by a schema" of files that are rejected when read | Each corrected |
| Reader B | `invalid/GRA-017-…` also broke TPL-006; the header of `invalid-by-rule/SCH-019-input-…` said that each of three documents is rejected; `invalid/PKG-001-manifest-version-range.yaml` was named for a promise about installing | The runtime is renamed; the header names the two; the file is now `NAM-006-manifest-version-range.yaml` |
| Editor | `core/operations.yaml` and the first-start example cited PLC-008 for how a location's facts are changed | PLC-021 |
| Editor | The definitions cited GRA-020 for "a value binding yields a value" | GRA-025 |
| Reader B | The overview: "registers them all" of documents that include contracts; a policy listed as something a Conduit carries | Corrected |

### Terms

| Found by | Term | Fix |
| --- | --- | --- |
| Reader A | "composition" and "composed", for a composite Conduit template and for a composed requirement | "composite" for the template, in CON-003, CON-016, CON-019, CON-020, INS-012, INS-016, the definitions, the schemas and two examples. "Composition" is kept for a requirement |
| Editor, reader B | "answer" as a noun, which the definitions rule out | CON-022, PLC-007, SCH-008 and DL3 reworded |
| Editor, reader B | "object" for a JSON map | INS-024, `schemas/README.md`, `schema.yaml`, one example and one message of the checker say "map" |
| Editor | "bound to" for something other than the bound; "combiner" in a message of the checker | Reworded |
| Reader B | "consumer" and "caller" for the instance that the retrieval tool is presented to and that calls it | "instance", in `core/README.md`, `capabilities.yaml` and `operations.yaml` |
| Reader B | "plain-language" in the overview; "inspecting a Conduit" where ACC-038 says an open one; "resolved path" in an example | Reworded |
| Reader A | ACC-026, "a policy that applies to its consumer's requests"; ACC-031, "applies only to" beside a policy that an onward request carries; MAN-010, "a relative path" | Reworded. MAN-010 says a program is a relative path where it holds a `/` and does not begin with one |
| Reader A | The definitions' rows for filter and for the core package lagged CTR-005 and FST-005 | Amended. A paragraph of "How a request ends" now excepts a step that is left as succeeded with nothing dispatched |

## Still needs a decision

Nothing below was changed, or only the part that the text settles. Where two promises still
collide, both IDs are given.

1. **A Resource of a retired Pool.** LIF-007 and LIF-015 say a retired instance "can still be
   inspected". ACC-033 authorizes inspecting a Resource, and reconciling its invocations, only
   "against the Pool that owns the instance", and CON-008 closes every Conduit to a retired
   Pool "at once". After LIF-021 nothing can inspect such a Resource or reconcile it.
2. **A failed retirement of a consumer.** LIF-026 calls a further retirement "the way from a
   shutdown that is pending ... to one that is confirmed". CON-008 closes the consumer's
   Conduits when its `retire` graph "have ended", whatever the outcome, so a step with
   `conduit` in the further run reports refused under ACC-011.
3. **A further `retire.v1` of an instance whose shutdown is confirmed.** LIF-015 lets it be
   dispatched and LIF-026 covers only a shutdown that is not confirmed. Whether its `retire`
   graph runs again rests on DL1.
4. **What a permitted request reports when it names what does not exist**: an object in
   `inspect.v1`, `set-up.v1`, `open.v1`, `unregister.v1`, `rename.v1` or `apply-policy.v1`; an
   ID that names no invocation in `reconcile.v1`; an ID that names no policy in
   `remove-policy.v1`. Only OPS-015 (refused) and PLC-016 (invalid) name a reason for such a
   case, and they differ. Also `set-up.v1` or `retire.v1` that names a Conduit.
5. **`#name` as the `from` of a nested instantiation of an unpublished project**, resolved when
   the template that holds it is instantiated later. NAM-022 says it "is never looked up among
   the objects that exist", and SUB-008 keeps only contracts and the manifest.
6. **An operation with no implementation selected.** TPL-004 requires one only for an operation
   the type requires or a runtime or step names. A Conduit may list any operation of an offered
   capability, and a control.
7. **A requirement applied to a Conduit that does not narrow and is not `shared-host.v1`.**
   Nothing says who checks it or what a request reports. With it: `obliges` on such a
   requirement has no stated effect.
8. **What `obliges: [consumer]` has the consumer's implementation do.** CON-022 has it state
   the filter in `supports`. IFC-018 sends the filter to the target's implementation only.
9. **Later work.** ACC-019 and IMP-006 say that "each start is an invocation that the engine
   itself dispatches". They do not say of which operation or with which arguments, and IMP-008
   has no case for it. Also: a graph that implements an operation without the mark
   `arrangesLaterWork` and holds a step whose operation carries it. IMP-004 rejects the like
   for model work, and no promise does here.
10. **Whether a registration installs the packages its manifest lists.** NAM-012 says "The
    package is fetched from that place". PKG-006 registers a template whose package "is not
    installed", and PKG-028 says "No package is fetched or installed because another package
    lists it". Until this is settled, the rule of MAN-006 on a relative `fetch` has no stated
    moment.
11. **The outcome of `retire-with-resources.v1` when Resources remain** (LIF-021): failed, or
    unknown as an effect that occurred in part under OPS-004.
12. **Three items of the implementation-defined list.** DL7 leaves to the engine "whether a
    package can be used from files that are still being edited, with no archive", which
    PKG-022, PKG-004 and NAM-006 do not allow. DL3 is about "a request that waits for an actor
    to reply", which no promise introduces. DL11 leaves open "how an optional Conduit that
    failed to open is opened later", which CON-004 and ACC-006 determine.
13. **The entry point that takes files** (R3-11). SUB-005 now says where the rules on a file
    apply. Whether every engine must take files, and not only values through `register.v1`, is
    not said.
14. **The shape of an inspection result and of a diagnostic** (R2-G5), with what LIF-020 needs
    of the result of `retire.v1` and `unregister.v1`.
15. **The sections of the Model Context Protocol, revision 2026-07-28**, that IFC-003, IFC-010,
    IFC-012 and IFC-016 rely on, and the error code in IFC-003 (R2-St1). They cannot be checked
    from this repository.
16. **How a value is marked as secret.** Left for the owner, as before.
17. **The choices above**, each of which the owner can strike: above all ACC-035 (no policy
    applies to the root Interface), PKG-026 to PKG-028, TPL-013, and the outcome of `open.v1`
    and `close.v1` on a Conduit that is already open or closed.

## Outside this pass

These are now out of date and were not in the files this pass could edit:

- `docs/proposals/reaudit/round-2/README.md` still says "Status: in progress", and does not
  link the decisions, the two logs of the fix pass or this log.
- The front page, `README.md`, in its row on the specification.
- Action A61 in `docs/AUDIT.md` still ends "The action waits for round 2 to finish". Only the
  status of A59, the two sentences that said round 2 was in progress, and the count of
  implementation-defined items were changed there.

## Checks

Run from the repository root on 2026-11-06, after the last edit to `spec/`, the checker and
the documents. Each exited with status 0.

```text
$ /usr/bin/python3 - (the validation command of CONSOLIDATION.md)
410 promises ok

$ (every promise ID and DL item cited in a .yaml or .md file under spec/, file names
  included, exists)
126 files; 3248 citations of promises, 47 of DL items; 410 promises and 15 DL items exist
promise IDs cited and missing: {}
DL items cited and missing: {}
named only in header comments of promise files, as removed: ACC-004 ACC-007 ACC-010 ACC-013 ACC-020 ACC-021 ACC-025 CON-005 CON-009 CON-010 CON-015 FST-004 GRA-013 GRA-016 INS-015 PKG-002 PKG-008 PKG-009 PKG-010 PKG-011 PKG-015 PLC-010 PLC-014 POOL-008 POOL-010

$ python3 tools/check_schemas.py        (first line, the line of each step, last line)
Python 3.9.6 at /Applications/Xcode.app/Contents/Developer/usr/bin/python3; PyYAML 6.0.1; jsonschema 4.20.0
1. Read 97 files holding 216 documents as strict YAML.
2. Checked 58 Schema contracts against the profile, and every `$ref`.
3. Validated 136 documents under spec/schemas/, spec/core/ and examples/valid/.
4. 59 invalid documents in 59 files were each rejected by a schema, or rejected when read.
5. Core package: 98 documents exported, 0 documents deliberately private and 1 implementation, which is also an export. Checked 432 references for the kind they name; 1 name packages that are not in this repository and were not checked. Checked 3 controls, the 5 operations that take no `revision`, the tables of the two READMEs, and 772 citations of promises in 100 files. The example packages keep 0 documents private.
6. Applied 13 rules of this tool, which come from 12 promises, to the core package and 3 example directories. 14 sets of documents in 14 files under examples/invalid-by-rule/ were each rejected by the rule they are named for; each of their 22 documents is valid against the schemas.
   SCH-019: validated 10 defaults of the core package and the valid examples against their schemas; 0 were not validated, because a `$ref` in the schema leads outside this repository.
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x429, apiVersion x219, metaPrefix x69. Errors: 0

$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 118 Markdown files and 1521 local links (offline).
Errors: 0

$ git diff --check
(no output; exit status 0)
```

`git diff --check` does not read a file that Git does not track yet. The three new files, this
log and the two examples `invalid/TPL-006-runtime-marked-optional.yaml` and
`invalid/NAM-006-manifest-version-range.yaml`, were searched for trailing spaces apart, and
have none. No line of a YAML file under `spec/` is longer than 96 characters.

Every number in the Contents table of `spec/README.md` was counted again by a script after the
last edit: the promises of each of the 22 files and their total of 410, the 15
implementation-defined items, the contracts and templates of each file of `core/` and of
`schemas/`, the 98 exports and 1 implementation, and the 7, 59 and 14 example files. All
agree. The quotation in `docs/OVERVIEW.md` was compared with
`examples/valid/project/templates.yaml` and `instances.yaml` and is exact; neither quoted
document changed.
