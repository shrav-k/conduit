# Consolidation pass 1 — log of group A

Written 2026-11-05 by the editor of group A, under the rules of
[CONSOLIDATION.md](CONSOLIDATION.md). Group A owns `conventions/access.yaml`,
`conventions/lifecycle.yaml`, `promises/conduits.yaml`, `promises/pools.yaml`,
`promises/placement.yaml` and `promises/first-start.yaml`. No other file was edited.

Finding IDs: `R1-n.m` is reader 1's section and item, `R2-Tn` and `R2-Sn` are reader 2's term
and shape findings, `R3-n` is reader 3's finding. Decision IDs are those of the
[re-audit record](README.md).

## Counts

| File | Before | Changed | Removed | Added | After |
| --- | --- | --- | --- | --- | --- |
| `conventions/access.yaml` | 32 | 17 | 6 | 2 | 28 |
| `conventions/lifecycle.yaml` | 23 | 16 | 0 | 3 | 26 |
| `promises/conduits.yaml` | 19 | 12 | 2 | 4 | 21 |
| `promises/pools.yaml` | 12 | 9 | 1 | 2 | 13 |
| `promises/placement.yaml` | 16 | 11 | 2 | 2 | 16 |
| `promises/first-start.yaml` | 5 | 3 | 1 | 0 | 4 |
| Total | 107 | 68 | 12 | 13 | 108 |

The validation command of CONSOLIDATION.md passes, and `python3 tools/check_docs.py` reports no
error. Every line of the six files is under 97 characters. No promise in any file of `spec/`
cites a removed ID of group A outside a header comment.

## access.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| ACC-001 | Now the negative case only: no listing Conduit, so refused, a record, no call. Retrieving one's own result under OPS-016 is excepted. | R1-1.6, R1-2.24, Q7 |
| ACC-003 | Took in "a Conduit that lists no operations permits nothing". "Created with" removed. | R1-1.6, W3 |
| ACC-004 | Removed, folded into ACC-003. | R1-1.6 |
| ACC-005 | Keeps only that the step is a request to open made by the actor that asked for the setup, decided under ACC-006. The sentence on required and optional steps is deleted. "Holds", and "remains registered", reworded. Names refused. | R1-1.7, R1-1.8, R1-3.8, R2-T6 |
| ACC-006 | Took in ACC-007. "Holding access" reworded. The Conduit to `access` must list the operation. Points at ACC-030 for a Conduit to `access` for another consumer. | R1-1.7, R1-3.8, R2-T6, U8, R3-7 |
| ACC-007 | Removed, folded into ACC-006. | R1-1.7 |
| ACC-009 | "Holds another Conduit" reworded. Adds the restrictions carried under ACC-017. "Succeeds" became "is permitted". | R1-2.20, R2-T6 |
| ACC-012 | "Dispatched only if". The bound covers the target and the operations. Excepts the root Interface's Conduits. Points at ACC-030 for a Conduit to `access`. | R1-2.19, U8, R2-T6 |
| ACC-013 | Removed. Its case is rejected at instantiation by CON-022. | G9, R1-2.17 |
| ACC-015 | "The capability the policy governs" dropped: each request the policy would apply to reports unavailable. | R3-36 |
| ACC-016 | "While serving an invocation". Cites `conduit.dev/serving` and `conduit.dev/conduit`. An instance that is not an actor and names no invocation reports refused. "Holds" reworded. | U1, U2, R3-2, R2-T6 |
| ACC-017 | The restrictions of the served invocation, named in `conduit.dev/serving`, are carried, and appear in `conduit.dev/restrictions`. "Allow" became "permit". | U1, U2, R3-1, R3-2 |
| ACC-018 | Applies to a step that names a declared Conduit under `conduit`. A step with none invokes the instance's own implementation. | G4, R3-17 |
| ACC-020, ACC-021 | Removed, folded into REC-001 and OPS-005. | R1-1.17 |
| ACC-022 | Keeps only "no bound". "The system" became "the engine". | R1-1.19, R2-T20 |
| ACC-025 | Removed, folded into ACC-033. | R1-1.18, A1 |
| ACC-026 | "Allows" and "succeed" became "permits" and "are permitted". | R2-T28 |
| ACC-029 | "Holder" and "created" reworded. | R2-T7, W3 |
| ACC-030 | Rewritten: opening or closing a Conduit to `access` for another consumer needs its bound to be within the actor's own. The observable is true at the time of opening. Closing the delegator's Conduit closes nothing it passed on. | U8, R3-7, R3-32 |
| ACC-031 | Every actor and target the policy applies to must be within the bound. "Recorded there" became "stored by `access`". | U7, R3-6, W2 |
| ACC-032 | The observable names refused for the reserved operation itself. | rule 6 |
| ACC-033 | Rewritten as the one statement of which target authorizes each management operation, with reconcile and controls. A Conduit is inspected and unregistered through `management`, opened and closed through `access`. | A1, A2, G11, R1-2.5, R2-T27, R3-19, R3-24, R3-30 |
| ACC-034 | New. A Conduit closed while an invocation through it is in flight. | U6, R3-5 |
| ACC-035 | New. No policy is applied to the root Interface. | U7, R3-6 |

## lifecycle.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| LIF-001 | Took in "no suspended state; apply a policy to limit access for a time" from CON-015. | R1-1.5 |
| LIF-002 | "Created" became "instantiated". | W3 |
| LIF-006 | Names the request to open a closed Conduit, from CON-015. | R1-1.5 |
| LIF-007 | "Remains registered" became "still exists". | W1, R1-3.8, R2-T9 |
| LIF-008 | "Declaration" became "nested instantiation". Took in CON-010. The step reports conflict under LIF-006. The sentence on required and optional steps is deleted. Holds until a replacement is registered. | R1-1.4, R1-1.8, R2-T5 |
| LIF-009 | The replacement is made from the nested instantiation as its parent instance holds it. The request is a registration, authorized under ACC-033, and opens nothing. A later setup may open a replacement Conduit. Took in POOL-010. | A3, G7, A1, R1-1.4, R3-26, R3-29 |
| LIF-010 | Keeps only "the lifecycle state is unchanged". Took in the placement and other-locations clauses of PLC-010. "Last recorded" removed. | R1-1.3, W2 |
| LIF-012 | A template exists until unregistered and has no lifecycle state. Layer became `convention`. | W1, R1-4.8 |
| LIF-014 | Split. Keeps the three-valued status. | R1-5.2 |
| LIF-015 | Keeps only the conflict half. Points at ACC-011 for requests through closed Conduits. | R1-1.1 |
| LIF-016 | Cites CON-008 for the closing of Conduits. | G12 |
| LIF-017 | An instance retired because its location was declared lost can be unregistered without confirmed shutdown. "Remains" became "still exists". | A2, R3-30, W1 |
| LIF-018 | Reconciling is authorized against the same target as retiring. An instance on a lost location has no retire operation to reconcile. | A2, R3-30 |
| LIF-020 | The guard on retiring a Pool is confirmed shutdown of every Resource it owns. "Still registered" became "still exists". | G12, R1-2.11, W1 |
| LIF-021 | The same guard: the Pool is retired only if the shutdown of every Resource is confirmed. | G12, R1-2.11 |
| LIF-022 | Keeps refused, and says it is reported where conflict would otherwise be. The clause on `root`'s Conduits is dropped in favor of ACC-024. Declaring `local` lost is refused. | Q5, R1-2.6, R1-1.19, R3-33 |
| LIF-024 | New, from LIF-014: when shutdown is confirmed, unknown or pending. | R1-5.2, R1-2.9, A2 |
| LIF-025 | New, from LIF-014: an instance never set up does not run its `retire` graph and is confirmed at once. | R1-5.2, G12, R1-2.9 |
| LIF-026 | New, from LIF-014: an instance whose shutdown is not confirmed may be retired again. | R1-5.2 |

## conduits.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| CON-001 | "Each resulting Conduit". "Declaration" became "document". | R1-2.16, R2-T5 |
| CON-002 | One Conduit per declared Conduit, or per component of a composite. The state clause is deleted. | R1-2.16, R1-1.2 |
| CON-004 | Layer became `convention`. Keeps "already open is left; no Conduit is instantiated", citing GRA-022. A registered replacement is opened by the next setup. | R1-4.4, R1-1.9, A3 |
| CON-006 | "The declaration is rejected" became "the document is rejected". | R2-T5, W4 |
| CON-008 | A Conduit whose target is retired closes at once. One whose consumer is retired closes after the consumer's `retire` graph and stops have ended. | G12, R3-42 |
| CON-010 | Removed, folded into LIF-008. | R1-1.4 |
| CON-011 | The consumer's status reports the Conduit as closed. "Unavailable" is no longer used for it. | R1-2.18 |
| CON-012 | The state clause is deleted. "Created" became "instantiated". | R1-1.2, W3 |
| CON-013 | "Is covered" became "is permitted". | R2-T24 |
| CON-014 | Keeps only "the consumer is not retired or interrupted". | R1-1.1 |
| CON-015 | Removed, folded into LIF-001, LIF-003, LIF-005 and LIF-006. | R1-1.5 |
| CON-018 | "Entry" became "item". The `appliesTo` rejection is left to CTR-005. | R2-T15, R1-1.24 |
| CON-020 | "Being made" became "being instantiated". | W3 |
| CON-021 | "Not created" became "not instantiated". | W3 |
| CON-022 | New, `language`. A Conduit carrying a filter that the target's selected implementation does not state in `supports` is not instantiated. | G9, U1, R1-2.17 |
| CON-023 | New, `convention`. A Conduit may be opened to a target that is registered. | G6, R3-20 |
| CON-024 | New, `convention`. A request through an open Conduit to a target that is not ready reports unavailable. | G6, R3-20 |
| CON-025 | New, `language`. A Conduit is not instantiated with a retired consumer or target. | R3-47 |

## pools.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| POOL-001 | "Created" became "instantiated". | W3 |
| POOL-002 | The state clause and "none has been set up" are deleted. "Created" became "instantiated". | R1-1.2, W3 |
| POOL-003 | Layer became `convention`. Points at LIF-009 for the retired case. | R1-4.3, R1-1.4 |
| POOL-004 | "Created by ordinary instantiation" reworded. | W3 |
| POOL-005 | "A registered object" became "an object that exists". | W1 |
| POOL-006 | "Made from" became "instantiated from". | R2-T25 |
| POOL-007 | "The requester" removed. The setup proceeds when a Resource it refers to is not ready, and requests to that one report unavailable. | R2-T7, R3-43, G6 |
| POOL-009 | Keeps only the positive rule on the input destination. The rejection is left to PLC-005. | R1-1.14 |
| POOL-010 | Removed, folded into LIF-009. | R1-1.4 |
| POOL-013 | Keeps the ordering and "already ready is left", citing GRA-022. The sentence on required and optional entries is deleted. | R1-1.9, R1-1.8 |
| POOL-014 | New, `language`. A reference from outside the Pool is `{kind: Pool, name, entry}` and resolves to the entry's Resource that is not retired. | G3, R3-15 |
| POOL-015 | New, `convention`. Setting up an entry's Resource during the Pool's setup is authorized against the Pool, for the actor that asked for the Pool's setup. | A1, R3-19 |

## placement.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| PLC-001 | "Created" became "instantiated". | W3 |
| PLC-002, PLC-003, PLC-004 | "Other than a location's" added. Only locations that are not retired count. PLC-004 applies when the default-location setting names none of them. | R1-2.14, R3-33, W1 |
| PLC-007 | The type or a selected implementation states a requirement on placement whose schema the location's stored facts do not satisfy. An absent fact fails. Stays `language`. | G5, R1-1.13, R1-4.6, R3-18 |
| PLC-008 | A location's facts are stored configuration, changed only by a control. | G5 |
| PLC-009 | "Creating" became "instantiating". | W3 |
| PLC-010 | Removed, folded into OBS-003 and LIF-010. | R1-1.3 |
| PLC-011 | Split. Keeps the conflict on unregistering. "Ends" became "retires". "Remain registered" became "still exist". | R1-5.3, R1-3.11, W1 |
| PLC-013 | Generalized to any opening and to each request, against the stored facts, reporting unavailable. The sentence on required and optional steps is deleted. | G13, R3-34, R1-3.4, R1-1.8 |
| PLC-014 | Removed, folded into CTR-004. | R1-2.15, R2-S1 |
| PLC-015 | "Created" became "instantiated". | W3 |
| PLC-016 | Layer became `convention`. "A registered location" became "a location that exists". | R1-4.6, W1 |
| PLC-017 | New, from PLC-011: what declaring a location lost does, and that those instances are unregistered without reconciliation. | R1-5.3, A2, R3-30 |
| PLC-018 | New, `language`. An instance is not placed on a retired location. | R3-33 |

## first-start.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| FST-001 | "The system" became "the engine". | R2-T20 |
| FST-002 | Layer became `convention`. "The system" became "the engine". | R1-4.7, R2-T20 |
| FST-003 | Layer became `convention`. "Service login" removed. "Is registered" became "exists". | R1-4.7, R1-3.7, W1 |
| FST-004 | Removed, folded into OBJ-002. | R1-1.10 |

## Left undone, and readings that should be confirmed

Needing a decision that is not in the record:

1. R3-7, second question. Decision U8 limits the bound of a Conduit to `access` that is passed
   on. It does not say whether that Conduit may list an operation of `access`, such as applying
   a policy, that the delegator's own Conduit does not list. ACC-030 keeps the sentence "passed
   on only as it is or narrower" and states no rule for the operation list.
2. R3-7, third question. ACC-030 uses U8's words: every target or Pool the new bound names is
   one the actor's own bound names. Whether a Resource that the actor's bound covers only
   through its Pool counts is not stated. Nor is whether a bound that names a Pool covers the
   Pool itself as a target, which an actor needs in order to open the Conduit to a Pool that
   decision A1 asks for; ACC-029 still says "exactly the Resources".
3. R3-31. Whether work arranged through a Conduit that was closed runs again once the actor
   has a new Conduit. ACC-019 is unchanged.
4. R1-1.14. Whether an entry's placement must be the destination of an input, or the default
   of PLC-002 and PLC-003 may apply. POOL-009 was reduced to the positive rule and does not
   decide it.
5. R1-3.11. PLC-011 now says only the declare-lost operation retires a location that still has
   instances. The reason an ordinary request to retire such a location reports is not stated.
6. R3-33, in part. Whether an instance may be placed on a location that is registered and not
   yet ready. PLC-018 covers only a retired location.
7. ACC-033 names no target for retiring a Pool together with its Resources (LIF-021), for the
   operations reserved to the root Interface, or for installing a package (R3-48).

Readings made while writing, to be confirmed:

1. CON-022 says "filter", which is the word of decision G9 and of the removed ACC-013, where
   the brief for this pass said "requirement". With "requirement", a Conduit that carries the
   shared-host requirement would need its target's implementation to list that requirement in
   `supports`, although the engine checks it under PLC-013.
2. PLC-013 reports unavailable for the request to open as well as for a request through the
   Conduit. Decision G13 names the reason only for "a request made while it does not hold".
3. ACC-006 and ACC-012 treat ACC-030 as the whole condition on the bound for a Conduit to
   `access` whose consumer is another instance, so the actor's bound need not also cover
   `access` as a target (R3-7, first question). A Conduit to `access` that an actor opens for
   itself stays under ACC-006.
4. An instance that was never set up and is on a location declared lost reports its shutdown
   as confirmed (LIF-025), not unknown (PLC-017). The two earlier sentences disagreed for this
   case and no finding raised it.
5. LIF-012 now says a template has no lifecycle state, where it said "no other state" than
   registered. Its layer became `convention` because its observable is how a request ends.
6. Under decision A2 the lost location itself is unregistered like the instances on it, once
   none is placed on it, because a location is its own placement.

Not applied because the rules of the pass exclude them: the moves of R1-4.1, R1-4.8 and
R1-4.10, and the splits R1-5.4, R1-5.9 and R1-5.10.

Depends on another group:

1. The sentence "setup fails if a required declared Conduit or entry does not succeed, and
   reports an optional one" was deleted from ACC-005, LIF-008, POOL-013 and PLC-013 under
   R1-1.8. It now rests on GRA-003 and GRA-022 alone. The step results "not opened" and "not
   set up" are no longer named in group A's files.
2. CON-022 cites PKG-020 for "an implementation marked `plain` states no `supports`", which
   group C adds under decision U1.
3. ACC-016, ACC-017 and ACC-034 cite the keys `conduit.dev/serving`, `conduit.dev/conduit` and
   `conduit.dev/restrictions` and do not define them.

For the coordinator's edit of `definitions.md` and `README.md`:

1. The promise counts in the README table are now 28, 26, 21, 13, 16 and 4 for the six files.
2. Rows that no longer match: `management` (ACC-033); location fact and host, which are stored
   and not reported (PLC-008); right to open and bound (ACC-030, ACC-031); unregister, which
   has the exception for a lost location (LIF-017); entry, which says "created, registered";
   refused, whose meaning is "not permitted".
3. Terms group A's promises use that still have no row: shared-host requirement, location
   type, default-location setting, access decision.
