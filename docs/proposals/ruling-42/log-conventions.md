# Ruling 42: log of the conventions group

Date: 2026-11-06. This is the account of how the editor of the conventions group applied
[ruling 42](README.md) to the files under `spec/conventions/`: the decisions of the coordinator
in the table [Settled by the coordinator](../ruling-41/open-decisions.md#settled-by-the-coordinator),
with the notes "Settled together" below it, and the owner's answers D6, D8, D10, D11, D12, D17
and R5, as the [record](README.md#what-follows) states them and as the subsections of
[For the owner](../ruling-41/open-decisions.md#for-the-owner) describe their options. The
decisions are the coordinator's and the answers the owner's. The wording that carries them out
is the editor's, and each choice below stands unless the coordinator or the owner strikes it.

Not applied here: the names N1 to N20 and the editorial fixes E1 to E23 of the
[vocabulary audit](../vocabulary-audit/README.md), which a later stage applies, so every name
below is the one in force today (location, operation identity, origin, `reveal-secret.v1`,
`serving`, `unreconciled`); and D5 and D20, which the owner did not rule. Neither D5 nor D20
touches a file of this group, so nothing here keeps or changes what the text says for them.

Files: [access.yaml](../../../spec/conventions/access.yaml),
[lifecycle.yaml](../../../spec/conventions/lifecycle.yaml),
[operations.yaml](../../../spec/conventions/operations.yaml),
[records.yaml](../../../spec/conventions/records.yaml),
[revisions.yaml](../../../spec/conventions/revisions.yaml) and
[implicit.yaml](../../../spec/conventions/implicit.yaml).
[observation.yaml](../../../spec/conventions/observation.yaml) needed no change. No other file
was touched. Each changed file has a line in its head comment naming ruling 42 and what changed.

| File | Before | Changed | Added | Removed | After |
| --- | --- | --- | --- | --- | --- |
| `access.yaml` | 29 | 6 | 0 | 0 | 29 |
| `lifecycle.yaml` | 34 | 4 | 1 | 0 | 35 |
| `operations.yaml` | 30 | 8 | 1 | 0 | 31 |
| `records.yaml` | 6 | 2 | 0 | 0 | 6 |
| `revisions.yaml` | 14 | 2 | 0 | 0 | 14 |
| `implicit.yaml` | 5 | 1 | 0 | 0 | 5 |
| `observation.yaml` | 5 | 0 | 0 | 0 | 5 |
| All | 123 | 23 | 2 | 0 | 125 |

## New IDs

| ID | File | Assigned | Subject |
| --- | --- | --- | --- |
| OPS-034 | `operations.yaml` | In advance (row 31, R22) | A repeat of an invocation that succeeded and whose outputs the engine no longer keeps: nothing is dispatched, it reports conflict, its result holds the engine's ID, and its diagnostic's `ruleId` is OPS-034. |
| LIF-036 | `lifecycle.yaml` | In advance (D11) | When a request to unregister an instance on a location declared lost is dispatched, the engine asks its runtime provider once more for the stop, before it removes the instance. |

No other ID was added. No promise was removed or moved between files.

## Decisions applied

| Decision | Promises | What changed |
| --- | --- | --- |
| D6 (row 5) | ACC-011 | Observable: a request through a closed Conduit is decided under ACC-001 from the open Conduits (ACC-036), OPS-023 puts refused before invalid, so it reports refused although the engine still holds the Conduit's ID (OPS-008, LIF-019); a request that names the closed Conduit itself, to open, close or inspect it, reports invalid (OPS-029). The rule in `then` was already refused and is unchanged. |
| Rows 6, 10 and 36 | ACC-029 | Observable: an item of a bound is compared when a request is decided, so it covers an operation of a package installed after the bound was written, and an operation of a submitted project in every submission at that address (SUB-008). That NAM-007 no longer applies to a bound's addresses, a check made before ruling 41, was already stated, and stays dropped. |
| Row 9 | ACC-039 | `then` and observable: the engine's read of a secret for a connection that the retirement makes to serve the hook belongs to the retirement, as one for a run does. |
| Row 11 | REV-012 | Observable: the retirements of PLC-017 and PLC-024, and the closing of a Conduit at once under CON-008, dispatch nothing and are not held back; TPL-012 says what a setup in progress then does. |
| Rows 11, 31 and 39 | REV-014 | `when`: every repeat answered without a dispatch is excepted, under OPS-009, OPS-012, OPS-032 or OPS-034. Observable: the setups and retirements that the engine dispatches within an actor's invocation (POOL-013, LIF-021) count as an actor's; a retirement under PLC-017 or PLC-024, and a location's setup under PLC-026, dispatch nothing and hold nothing back; a repeat that dispatches nothing is its own promise's, and one that would be a new attempt reports conflict here while another has not ended. |
| Row 12 and row 18 | REC-002 | `then`: a record holds the engine's ID for the invocation that it serves, where it serves one, and the operation identity that the engine gave under OPS-028. Observable: what `invocation` and `serving` hold for a request, a hook, a read, a part and a provider call; the parts of an invocation are the records whose invocation or invocation served names it; a record of `run.v1`, `stop.v1` or `inspect-run.v1` names its instance in `subject`; a change that no request asked for holds the invocation within which it happened and its caller, or states the cause where there is none. |
| D12 | REC-002, ACC-019 | REC-002: a read made again under SEC-023 has the caller of the setup that made the connection, and names that setup as the invocation it serves. ACC-019: the read that SEC-023 makes again is the one request that the engine dispatches on behalf of an invocation that has ended (`then`), decided by the instance's own Conduit and its filters as they stand, so closing that Conduit stops it (observable). |
| D17 | REC-002 | Observable: the record of a call of `run.v1` holds no part of its `identity`, under PRV-022. |
| D11 | LIF-036 (new), LIF-017 | LIF-036: the stop asked for once more at unregister, and what comes of it. LIF-017's observable points to it. |
| Row 19, row 33 and R5 | LIF-021 | `then`: a Resource's retirement is dispatched only once the shutdown of every Resource that the order puts before it is confirmed; a retirement that ends without the shutdown confirmed, or that reports a reason in place of being dispatched, holds back only the Resources that the order puts after it, and every other Resource is retired. The sentence under which the request dispatched no further retirement after the first that failed, decision 5 of round 3, is gone. Observable: a Resource that cannot be retired keeps back only what it may still be using; a repeat finishes; a cancellation is passed on, dispatches no further retirement, and each retirement in progress still asks for its stop. |
| Rows 27 and 28 | LIF-009 | Observable: a replacement for which no location matches, or whose stored `runtime` names a retired runtime provider, reports invalid; one that the machines provider cannot place now reports unavailable under PRV-015 and can be made again. |
| Row 32 | LIF-013, OPS-014, REC-004 | LIF-013: the status lists in `unreconciled` the engine's ID for each invocation of unknown outcome, not yet reconciled, that was dispatched to the instance or that names it in an input or as its target, read from OPS-008's state, which no purge shortens. OPS-014: the ID is also found there; reconciling a retirement confirms the shutdown although its hook is still unknown, and otherwise every further retirement, or setup, ends unknown until the hook is reconciled. REC-004: after a purge such an invocation is still found in the status of its instance while that instance exists. |
| Row 31 | OPS-032, OPS-034 (new), OPS-005, OPS-008, OPS-009, OPS-024, ACC-012, REC-004, REV-014 | OPS-032 keeps a repeat of an invocation that has not ended, and its diagnostic's `ruleId` tells the caller that a later repeat is answered once it ends; OPS-034 takes a repeat of one that succeeded and whose outputs are no longer kept. Each promise that named both cases names the one that applies. |
| Rows 21 and 31 | OPS-008 | Observable: the caller kept for a hook's invocation is that of its latest attempt; a repeat once the outputs are gone reports conflict under OPS-034. |
| Row 21 | OPS-028, IMP-007 | OPS-028 `then`: a new attempt of a hook is made for the caller of the setup or the retirement within which it is dispatched, who is the caller of the hook's invocation from then on. Observable: each attempt's record names its own caller; an onward request of an earlier attempt still running on a machine cut off is decided for the latest caller, a limit of this version. IMP-007: the further setup or retirement that dispatches the new attempt is the repeat. |
| Row 33 | OPS-027 | Observable: where the engine implements the invocation in parts (`set-up.v1`, `retire.v1`, `retire-with-resources.v1`), it passes the cancellation on to each part in progress, by the protocol's means or by cancelling a part that `engine` implements; no later part begins; every retirement still asks for its stop; parts that ended are not undone; a cancelled part's invocation ends when the cancellation is sent, so its onward requests are refused (ACC-019). |
| Row 34 | ACC-018 | Observable: in this version no hook carries a filter, so the restrictions that a hook is sent are an empty list (IFC-018). |

Rows that list no promise of this group, or whose promises here needed nothing: rows 1 to 4,
7, 8, 13 to 17, 20, 22 to 26, 29, 30, 35, 37, 38 and 40 to 44. The owner's answers D8 and D10 touch no
promise of this group: the variables that the engine's own runner passes are PRV-023, and the
rule on a store's Type is SEC-022.

## Choices beyond the record

1. **D6, ACC-011.** The analyst's entry for row 5 was not applied as such, as the work order
   asks. The text applied is D6's option "Refused, as the text says", in the words of
   [D6](../ruling-41/open-decisions.md#d6-a-request-through-a-closed-conduit). It comes to the
   same rule as the entry.
2. **Row 31, ACC-012.** `close.v1` declares no outputs, so OPS-034 never answers a repeat of it.
   The analyst's entry would have cited OPS-034 beside OPS-009 and OPS-032; the promise says
   instead that OPS-034 does not apply to it. The table's row 31 lists ACC-012, and this is how
   it is changed.
3. **Row 34, ACC-018.** `retire-with-resources.v1` is named beside `set-up.v1` and `retire.v1`
   among the operations that `engine` implements, since the retirements it dispatches carry
   the hook too; and the request that carries a filter is said to report unavailable, as
   ACC-015 has it.
4. **D12, ACC-019.** The exception is written into `then`, which forbids every request on
   behalf of an invocation that has ended, and not only into the observable: without it,
   ACC-019 would forbid the read that SEC-023 makes.
5. **D12, REC-002.** A read made again under SEC-023 names, as the invocation it serves, the
   setup that made the connection. SEC-023 and SEC-011, as the secrets group wrote them, say
   the same.
6. **D17, REC-002.** REC-002 says that the record of a call of `run.v1` holds no part of its
   `identity`, citing PRV-022, which states the rule, so that the promise that lists what a
   record holds names it. The sentence on `secrets` cites the description of `run.v1`.
7. **R5, LIF-021.** The rule is written as the order's: a Resource's retirement is dispatched
   only once every Resource that the order puts before it is confirmed shut down. Ruling 42's
   table says "every Resource that does not depend on it"; the record and R5's option say "only
   the Resources that the order puts after the one that could not be retired wait". The two
   are read as one: what waits is what the stuck Resource refers to, names in `runtime` or is
   the consumer of a Conduit to, and so on in turn, which it may still be using, and within a
   cycle what the engine's order puts after it. A Resource that neither relation joins to
   another holds back none, whatever sequence DL1 gives.
8. **D11, LIF-036.** Beyond the record: the instance is removed once the stop has ended, or was
   not dispatched because the runtime provider is not ready, whatever came of it, and the
   outcome of the request is that of the removal, as the request to unregister asserts that the
   shutdown is settled in another way (row 4). The core group's description of `unregister.v1`
   says the same. PLC-023 does not hold the stop back, since a location declared lost stands
   for no machine (PLC-026). The promise covers only an instance for which an item that is run
   is selected, as PRV-008 does.
9. **Row 33, OPS-027.** "Every retirement still asks for its stop" is read to mean also that a
   stop already in progress is not cancelled: the cancellation is passed on to a provider call
   other than a stop. The instance is retired from the moment its retirement was dispatched
   (LIF-005), so a cancelled stop would gain nothing.
10. **Row 32.** The list `unreconciled` is stated in LIF-013's observable, beside the rest of
    what the status holds, as the analyst's entry put it, and not in its `then`.

## Open

- **For the owner or the coordinator.** Choice 8: whether an unregister whose stop failed or
  ended unknown should still remove the instance. The other reading, that the removal waits
  for a stop that succeeds, would leave an instance on a machine that never returns with no
  way to be removed, which is what LIF-017 exists for.
- **For the owner, with D5 and D20.** OPS-028 states a limit of this version: an onward request
  from an earlier hook attempt still running on a machine that was cut off is decided for the
  latest caller. Work across machines, which the owner asked to be researched further, may
  change it, with local enforcement.
- **For other groups.** `spec/definitions.md` still says, under "repeat", that a repeat of an
  invocation that succeeded and whose outputs are no longer kept reports conflict under
  OPS-032; it is OPS-034 now. Lines 211 and 216 cite OPS-032 alone, which still holds for an
  invocation that has not ended.
- **For the vocabulary stage.** `serving`, `unreconciled` and the subject of OPS-034 are
  working names, as the open questions record says.

## Checks

Run from the repository root when this group finished, with other groups still at work:

- The validation script of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation): passes.
- Every line of the files of this group is at most 96 characters.
- Every promise ID that the files of this group cite in a promise exists, OPS-034, LIF-036 and
  the IDs assigned to other groups (PRV-021, PRV-022, SEC-023) among them.
- `python3 tools/set_names.py --check`, `python3 tools/check_schemas.py` and
  `git diff --check`: no errors.
- `python3 tools/check_docs.py`: one error, in a file of another group: `spec/core/README.md`
  links to `docs/proposals/ruling-42/log-core.md`, which the core group had not yet written.
  It is noted here and not fixed.
