# Round 6 of the re-audit: log of the conventions pass

Date: 2026-11-07. This pass applies [ruling 44](../../ruling-44/README.md) to the files of
`spec/conventions/`: the parts of findings 6-1-01 and 6-1-02 of the [findings](reader-1.md) that
fall in them, and the critiques D03, D04 and D18 of the [design critique](design-critique.md). It
edits no other file under `spec/`. No promise is added, removed or moved, and each changed
promise keeps its ID. Three files changed, and each gains one head-comment entry for ruling 44
and round 6: `access.yaml`, `implicit.yaml` and `operations.yaml`. `lifecycle.yaml`,
`observation.yaml`, `records.yaml` and `revisions.yaml` are unchanged.

The independent check of the findings (the journal's `verify:01`, `verify:02`,
`verify:simplify`) was read first. Its recommended least changes were followed except where the
decided details below differ, and the differences are listed under the choices.

## Applied

| What | Promise IDs | From | Why |
| --- | --- | --- | --- |
| `when`: the exception for -32021 is limited to an implementation that carries Conduit's keys, for both of its cases: naming Conduit's extension (IFC-008) and naming a capability the call did not declare, which IFC-047 has such an implementation answer only where it has performed nothing. Observable: says so, and says that a plain implementation's -32021 is not excepted: IFC-010 reads it as any other error, failed for an operation that declares no effect and otherwise unknown, and a repeat of an invocation so left unknown is not dispatched to it, under OPS-012. | OPS-007 | 6-1-01 | The protocol says a server answers -32021 where processing needs an undeclared capability, not that it has not acted. Only Conduit's own contract can bind a server to have done nothing. Reverses V2-15 of round 5, as decided. |
| `then`: the engine no longer keeps "the filters that its latest attempt was sent". | OPS-008 | 6-1-02 | The only reader of that state was OPS-009's and OPS-034's comparison of the filters sent, which the decided rule replaces. `verify:02` recommended it. |
| `when`: a repeat of an invocation that succeeded is answered from it where no filter applies and the operation declares no outputs or its outputs are kept, or where a filter applies and the operation declares no outputs and states an `effect`, is a control or starts model work. Observable: the answer for an operation with no outputs is given unless OPS-024 dispatches the repeat; where a filter applies, it gives only the outcome already recorded and no filter judges it (ACC-008); the cases of OPS-032, OPS-034 and OPS-024 are listed. | OPS-009 | 6-1-02 | Decided detail 02: an operation with an effect and no outputs keeps OPS-009's answer from its outcome. |
| `when`: also a repeat of an invocation that succeeded, where a filter applies and the operation states no `effect`, is not a control and is not marked `startsModelWork: true`, as IFC-033 tests for `readOnlyHint`. `then` unchanged: it is dispatched as a new attempt. Observable: why (the filters decide afresh what the attempt may do and return now, as with a deadline that has passed), that no effect is applied twice, that a successful new attempt's outputs are the ones kept, and that a new attempt answered with a reason under IFC-026 reports that reason and leaves the invocation's outcome and kept outputs as they were. | OPS-024 | 6-1-02 | Decided detail 02: an operation with no declared effect is dispatched again so its filters decide afresh. |
| `when`: conflict where the operation declares outputs and either no filter applies and the outputs are not kept, or a filter applies and the operation states an `effect`, is a control or starts model work. Observable: the paragraph on which filters were sent is replaced by why outputs are not given through a filter even where it is the same filter with the same arguments, and why the operation is not dispatched instead; later repeats report conflict unless OPS-024 dispatches them. | OPS-034 | 6-1-02 | Decided detail 02: an operation with an effect and declared outputs reports conflict instead of delivering stored outputs. |
| Observable: the sentence on OPS-034 now names the case as one that OPS-009 does not answer and OPS-024 does not dispatch. | OPS-005 | 6-1-02 | It quoted the removed "latest attempt was not sent" test. |
| Observable: a later repeat is answered under OPS-009, OPS-012 or OPS-024, or reports conflict under OPS-034 where neither OPS-009 nor OPS-024 answers it. | OPS-032 | 6-1-02 | It said conflict only where the outputs are no longer kept. |
| `when`: every invocation that has ended, whatever its outcome, or that was in progress at a restart. Observable: a new attempt is made only for a repeat that OPS-012 or OPS-024 dispatches, which now includes a repeat, to which a filter applies, of an invocation that succeeded, of an operation with no effect by IFC-033's test. | IMP-007 | 6-1-02 | A succeeded invocation can now be dispatched again, so the "must not" has to cover it: never without a repeat. |
| Observable: a filter judges only a call that an implementation receives, so no filter judges a repeat that is not dispatched under OPS-009, OPS-012, OPS-032 or OPS-034; where a filter applies to one, it returns only an outcome already recorded, or conflict, and no outputs. A repeat of an operation with no effect to which a filter applies is dispatched again under OPS-024. | ACC-008 | 6-1-02 | Decided detail 02: "add to ACC-008 or OPS-009 that a repeat not dispatched is judged by no filter and returns only an outcome already recorded". Put in ACC-008, the general rule that conflicted with the replay, with a pointer from OPS-009. |
| Observable: a bound does not limit the filters of the Conduits its consumer opens; a consumer whose own Conduit to a target applies a filter may open, for itself or another, one to that target that applies none. | ACC-029 | D03 | Ruling 44, item 2, option a: say it plainly. |
| Observable: "never by filters" is restated as "does not limit the filters of the Conduits that its consumer opens". A Conduit to `management` carries no bound (CON-028) and no filter, since `engine` supports none (CON-022, MAN-012), so access to `management` cannot be narrowed to one template. A narrower provisioning goes through a broker: a Resource with a trusted operation whose own contract and configuration allow only what they should, whose implementation makes the onward request through the broker's own Conduit to `management` (ACC-016); the actor's Conduit to the broker applies no filter, since the onward request would carry it (ACC-017) and would report unavailable (ACC-015). | ACC-030 | D03, D04 | Ruling 44, item 2, option a, naming the broker pattern. |
| `then`: the prose is replaced by one decision list of fourteen rows, each the request, the family and state of the object, and the authorizing target, the first row that fits deciding, and the closing rule that no request is authorized against a retired Pool. The `when` and the observable are unchanged. | ACC-033 | D18 | Derived from the current text, row by row; no answer changes (checked below). |
| One head-comment entry each, naming every promise changed and the finding or critique. | none | all | The rules of the pass. |

Fields were rewrapped greedily at 96 characters. The rows of ACC-033's `then` each begin a new
line in the source, so that the list can be read there; the value is folded, so the text is the
same as one paragraph.

## How ACC-033's answers were checked

Each sentence of the old `then` maps to rows: registering into an existing Pool that is not
retired (1); registering anything else, a Resource whose owner is retired or registered in the
same submission among them, replacing and unregistering (2); `register-replacement.v1` (3); set
up and retire of a Resource, refused where its Pool is retired (4), as the observable already
said; of an Agent, an Interface or a Pool (5), a retired Pool's further retirement under LIF-026
among them; inspecting a Resource, with `management` in place of a retired Pool (6), and
anything else (7); `retire-with-resources.v1` (8); `install.v1` and the operations of ACC-032
(9); open and close (10); a control (11); reconciling where the instance no longer exists (12),
which comes first because the old text sent it to `management` whatever target would otherwise
authorize it, an `open.v1` whose Conduit is gone among them; a management verb (13) and any other
invocation (14), each with `management` in place of a retired Pool. The traces of the observable
(a registration reconciled through `management`, a hook reconciled as any operation of its
instance, `retire-with-resources.v1` reconciled through `management` once the Pool is retired)
give the same targets under the rows.

## Choices beyond the record

| Choice | Why |
| --- | --- |
| OPS-007 cites IFC-047, not IFC-045, for the duty of an implementation that carries Conduit's keys to answer -32021 only before acting. | The decided detail placed the duty "in IFC-045". The interface pass, working beside this one, wrote it as a new promise, IFC-047, since a promise holds one obligation; IFC-045's observable points to it. If that number changes, OPS-007 changes with it. |
| The exception in OPS-007 for IFC-008's -32021 also names "an implementation that carries Conduit's keys". | No change of behavior: only such an implementation is declared Conduit's extension (IFC-037, IFC-007). It lets one phrase cover both cases. |
| The dispatch of a repeat of a succeeded no-effect invocation is put in OPS-024's `when`, not in a new promise. | OPS-024's obligation, "dispatched as a new attempt", is the same; IMP-007's "only for a repeat that OPS-012 or OPS-024 dispatches" then stays true. The six cases of a repeat of a succeeded invocation (filter or not, effect or not, outputs or not, kept or not) are split without overlap between OPS-009, OPS-024 and OPS-034. |
| The no-effect test is written out each time as IFC-033's: the contract states no `effect`, the operation is not a control, and its contract is not marked `startsModelWork: true`. | No defined term names it, and a control or an operation that starts model work need not state an `effect` (`verify:02`). The definitions row "repeat" says "does not start model work", which means the same (IMP-001). |
| A new attempt that the implementation answers with a reason under IFC-026 leaves the invocation's outcome, and the outputs kept for it, as they were, and the repeat reports that reason. Stated in OPS-024 for every new attempt. | Needed for the decided rule: the deadline case of 6-1-02 ends in refused, and nothing said what the invocation then is. It was also unstated for a new attempt of a failed invocation; the implementation performed nothing (IFC-026), so nothing changed. The definitions row "attempt, new attempt", as edited in the same round, says the same. |
| Where a new attempt of a succeeded invocation succeeds, the outputs kept are its own. | Follows from the attempt being the invocation's latest; stated so that a later repeat with no filter does not give the older outputs. |
| Where no filter applies, a repeat of a no-effect operation whose outputs are no longer kept still reports conflict under OPS-034. | The decided detail limits the new dispatch to "where a filter applies". Dispatching such a repeat too would be safe and simpler for a caller, but is not in the record. |
| IMP-007's `when` covers every invocation that has ended. | Before this change a succeeded invocation was never dispatched again, so the `when` did not need it. |
| ACC-033: "No other request is authorized against a retired Pool" becomes "No request is ..."; the sentence "In both cases the Conduit lists `reconcile.v1`" is dropped; row 4 says "none" where the old text said, in the observable, that such a request reports refused. | The rows now route the two former exceptions (inspecting a Resource, reconciling) to `management` themselves, so "other" referred to nothing; the dropped sentence is the first sentence's "lists the operation". No answer changes. |
| ACC-030's broker text follows the definitions row "broker" and `spec/README.md` as found in the tree (a Resource with a trusted operation; "the narrowing is the broker's and not the engine's"). | So that the three say the same. The ruling's words were "a trusted operation of a Resource, a broker, whose own contract allows only what it should". |
| No change for 6-1-03 (REV-006, REV-007, REV-015), 6-1-04, 6-1-05, D07, D11, D14, the hook default or 6-1-06. | Read for each: REV-006 and REV-007 stop contradicting each other once CTR-006 rejects overlapping pointers (`verify:03`); OPS-026 still holds with D14, since SCH-008 keeps a `default` on root `properties` alone; REV-006's "lists `revision` alone as required" still holds with D07; LIF-009's reference to `replace.v1` and SUB-009 still holds with D11. |

## Needs a decision, or is a cost to note

- **A repeat of a read can now end unknown.** A new attempt of a succeeded no-effect invocation
  can end failed or unknown, which replaces the outcome succeeded (`verify:02`). One left unknown
  is listed under `unreconciled` (LIF-013) and, under OPS-012, gets no further attempt unless
  its contract declares `deduplicates: true`. This follows from the decided rule; it is noted
  for the owner, not changed.
- **Lost results through a filtered Conduit.** An operation with an effect and outputs, reached
  through a Conduit that applies a filter, can no longer recover a lost result by repeating: the
  repeat reports conflict, and the outcome is in the records. This is the cost the reader named
  and the ruling accepted. A contract under which a filter's earlier approval suffices would
  win it back, and is a later decision.
- **Errors left as they are.** IFC-010's reading of -32602 as failed whatever the effect, and
  IFC-046's "before acting" for -32020, rest on the same kind of inference as V2-15
  (`verify:01`, risk 5). Neither is in the ruling, and neither is in this pass's files.

## For the other passes

Found while reading, in files outside `spec/conventions/`, in the working tree as this pass ended:

- `spec/interface/implementation.yaml`: IFC-016's observable still says a repeat is answered
  "while the engine keeps them and every filter that applies to the repeat was sent with the
  latest attempt", and IFC-025's observable "a filter applies to the repeat that the latest
  attempt was not sent". Both quote the test that OPS-009 and OPS-034 no longer use.
- `spec/definitions.md`: the rows "repeat" and "attempt, new attempt" already agree with OPS-009,
  OPS-024 and OPS-034 as changed here.

## Validation

Run at the end of this pass, with the other passes' edits in the working tree:

- The command of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): 453 promises ok. The count
  includes IFC-047, added by the interface pass; this pass added none.
- No line of `spec/conventions/*.yaml` is over 96 characters.
- `python3 tools/check_schemas.py`: 0 errors.
- `git diff --check`: passes.
- `python3 tools/check_docs.py`: 1 error, the link from `round-6/README.md` to
  `log-tracker.md`, which another pass has not yet written. Not in this pass's files. An earlier
  run gave 13, each a link to a record of another pass not yet written then.
- `python3 tools/set_names.py --check`: 0 errors. An earlier run gave 2, in
  `spec/interface/implementation.yaml`, since fixed by its pass.
