# Re-audit, round 5: log of the fix pass for area F3

Date: 2026-11-07. This pass applies, to the files of `spec/conventions/`, the changes of
[DECISIONS.md](DECISIONS.md) that fall in them: the items of area F3 (M32 to M44) and the parts
of M09 (area F1) and of M55, M60 and M69 (area F4) that change a convention. It edits no other
file under `spec/`. No promise is added, removed or moved, and each changed promise keeps its ID.
Each changed file gains one head-comment line for round 5: `access.yaml`, `lifecycle.yaml`,
`operations.yaml`, `records.yaml` and `revisions.yaml`. `implicit.yaml` and `observation.yaml`
are unchanged.

## Applied

| Item | Files | Promise IDs | What changed |
| --- | --- | --- | --- |
| M09 | `spec/conventions/access.yaml` | ACC-019 | `then`: the read for a provider's run that PRV-026 asks for again is excepted once the engine has restarted, "or the part of it that held that run has stopped". Observable: a provider's run that "ended with the engine, or with the part of it that held the run". As decided. |
| M09 | `spec/conventions/records.yaml` | REC-002 | Observable: "once the engine has restarted" struck from the sentence on the invocation that a read for PRV-026 serves, so it holds for either trigger. As decided. |
| M32, M39 | `spec/conventions/operations.yaml` | OPS-008, OPS-009, OPS-034 | OPS-008 keeps "the filters that its latest attempt was sent". OPS-009's `when` takes the decided whole-field text: it answers from the outputs only where the operation declares none, or the outputs are kept and every filter that applies to the request was sent with the latest attempt. Its observable says that no outputs are kept of an invocation whose outcome a reconciliation set (M39), and points the other case to OPS-034. OPS-034's `when` takes the decided whole-field text, and its observable the decided replacement: the outputs of a reconciled invocation are never kept, what "sent" means under IFC-018 and SCH-007, and why outputs made without a filter are not given through it. The texts of M32 already held M39's pieces, so each was applied once. |
| M33 | `spec/conventions/lifecycle.yaml` | LIF-023 | `when` and `then` take the decided whole-field texts, so the exceptions are in the `when`. The observable's first sentence becomes the pointer to PLC-024. Behavior is unchanged. |
| M34 | `spec/conventions/operations.yaml` | OPS-030 | `when`: "or it has been reconciled already" struck. Observable: a second reconciliation reports conflict unless a new attempt dispatched since, under OPS-024, has ended unknown; a reconciliation settles the latest attempt. As decided. |
| M34 | `spec/conventions/lifecycle.yaml` | LIF-013 | Observable: `unreconciled` lists "each invocation whose outcome is unknown"; "not yet reconciled" is struck. As decided. |
| M35 | `spec/conventions/revisions.yaml` | REV-007, REV-010, REV-012 | REV-007: the values are stored "unless its instance was retired while it ran", and its observable says that such a control stores nothing and leaves the revision unchanged. REV-010: a reconciliation that stores values needs an instance that "exists and is not retired"; one of a control whose instance is retired names no revision and records the outcome alone, whichever way it records the effect; the observable's sentence on a retired instance is replaced as decided. REV-012's observable adds that a control in progress runs to its end and stores nothing in the retired instance. As decided. |
| M35 | `spec/conventions/operations.yaml` | OPS-031 | `when`: `revision` does not belong where the control's instance "is retired or no longer exists". Observable: the storing reconciliation needs an instance that is not retired, and a control whose instance is retired is listed among the reconciliations that store nothing, under REV-010. As decided. |
| M35 | `spec/conventions/lifecycle.yaml` | LIF-015 | Observable: the sentence on a reconciliation that would store in a retired instance is replaced by the decided one: a reconciliation of a control dispatched to it stores nothing in it. With less text than decided, the list of requests that a retired instance takes also drops "where the reconciliation stores nothing in it" after "any other invocation that was dispatched to it", since that is now always so and the next sentence says it. |
| M36 | `spec/conventions/revisions.yaml` | REV-014 | Observable: a reconciliation of a setup or a retirement of the instance, an open or a close among them, counts as a setup or a retirement of it. As decided. |
| M36 | `spec/conventions/lifecycle.yaml` | LIF-024 | Observable: the latest invocation counts only for a shutdown that is not confirmed; any invocation of `retire.v1` that succeeded, or that a reconciliation records as having occurred, confirms it. As decided. |
| M37 | `spec/conventions/lifecycle.yaml` | LIF-028 | `then`: an open or a setup recorded as having occurred makes the Conduit open or the instance ready "unless the instance, or the Conduit's consumer, has been retired". Observable: such a Conduit stays registered under CON-031 until CON-008 closes it. As decided. |
| M38 | `spec/conventions/revisions.yaml` | REV-014 | Observable: a request for `retire-with-resources.v1` is a request to retire the Pool and holds back every other setup, retirement or change of it; the Pool's own `retire.v1` that it dispatches is part of it and is not held back. The decided "the Pool it is made of" is written "the Pool it is requested of", the words LIF-021 uses ("requested of the Pool"). Placed after the sentence on LIF-021, apart from M36's. |
| M40 | `spec/conventions/operations.yaml` | OPS-022 | Observable: the connection to a `remote` item counts as a part of an operation whose contract states no `effect`. As decided. |
| M41 | `spec/conventions/lifecycle.yaml` | LIF-007, LIF-015, LIF-017, LIF-035, LIF-036 | LIF-017's `then` and observable take the decided whole-field texts: any retired instance may be unregistered, the request is the finalizer-style assertion of PLC-027, the stop is asked once more, and the Conduits of which it was the consumer close on removal. LIF-007, LIF-015 and LIF-035 drop the words that limited unregistering to an instance whose node was declared lost. LIF-036's sentence on the node's labels begins "Where the node was declared lost". As decided. |
| M41 | `spec/conventions/access.yaml` | ACC-033 | `when`: "to unregister a template or a retired instance". The authorization is unchanged. As decided. |
| M42 | `spec/conventions/operations.yaml` | OPS-021, OPS-028 | Observables: neither `setup-hook.v1` nor `retire-hook.v1` declares `deduplicates: true`, so a hook whose outcome is unknown is not dispatched again until it is reconciled; OPS-028 says that the implementation recognizes the new attempt of a hook that failed. As decided. |
| M43 | `spec/conventions/lifecycle.yaml` | LIF-021 | Observable: the order "extends the reverse of" POOL-013's, which "comes from the references among the entries alone". As decided. |
| M44 | `spec/conventions/lifecycle.yaml` | LIF-029, LIF-009, LIF-033 | LIF-029's `when` and observable take the decided whole-field texts, limited to a declared Conduit, with a pointer to POOL-003 for an entry. LIF-009 and LIF-033 name POOL-003 for an entry. As decided. |
| M55 | `spec/conventions/access.yaml` | ACC-037 | `when`: also where whether the instance that an otherwise valid credential names still exists cannot be established, under IFC-032. As decided. |
| M60 | `spec/conventions/operations.yaml` | OPS-007 | `when`: the -32021 that IFC-008 has an implementation send is the one "whose `data.requiredCapabilities` names Conduit's extension". As decided. |
| M69 | `spec/conventions/lifecycle.yaml` | LIF-027 | Observable: DL11 is replaced by "Meanwhile the consumer may make the requests that its open Conduits permit, under ACC-003, and no other." As decided. |
| All | the five changed files | none | Each gains one head-comment line for the fifth round, naming every promise it changed and the item that changed it. |

Fields were rewrapped greedily at 96 characters from the first changed line until the new lines
rejoin the old ones at a full line; where they do not, the rest of the field is rewrapped. No
words outside the decided changes moved, except the LIF-015 and REV-014 wording noted above.

## Not applied here

| Item | File | Promise IDs | Why |
| --- | --- | --- | --- |
| M23 | `spec/promises/secrets.yaml` | SEC-024 | Outside this pass's files, which are `spec/conventions/*.yaml`. The decision's own routing guessed that secrets.yaml is F3's, but the files of `spec/promises/` belong to another pass. When this pass ran, SEC-024 in the working tree already ended the sentence at "as the Remote writes it.", from that pass. |
| M26 | `spec/promises/secrets.yaml` | SEC-001 | Outside this pass's files. Not yet applied in the working tree when this pass ran. |
| M27 | `spec/promises/providers.yaml` | PRV-007 | Outside this pass's files. Not yet applied in the working tree when this pass ran. |

## For the other passes

These changes that the decisions name fall outside `spec/conventions/` and were not made here:

- M41: CON-008 must say that the Conduits of which an instance is the consumer close when it is
  removed by a request to unregister it, which LIF-017 now cites. The definitions rows
  "retired", "remove" and "unregister" still limit unregistering to an instance whose node was
  declared lost.
- M35 and M37: the description of `reconcile.v1` and the definitions.
- M42: the definitions row "attempt, new attempt", which still says that a hook of unknown
  outcome is dispatched as a new attempt on the conditions of OPS-012.
- M69: DL11 in `spec/implementation-defined.yaml` and the count in `spec/README.md`.
- M09: PRV-026, PRV-002, SEC-010, SEC-011, DL17, `get-secret.v1`, the definitions and the
  README.
- M55 and M60: IFC-032, IFC-037, IFC-007 and IFC-016.
- M32, not named in the decision: the definitions row "repeat" says that a repeat of an
  invocation that succeeded is answered from its outputs "where the engine still keeps" them,
  and reports conflict where they are no longer kept. OPS-009 and OPS-034 now also send to
  conflict a repeat to which a filter applies that the latest attempt was not sent, and a
  repeat of an invocation whose outcome a reconciliation set.

## For the coordinator

- M41 opens one case that no promise settles. Before it, only an instance whose node was
  declared lost could be unregistered, and such a node represents no machine, so PLC-023 never
  held back LIF-036's stop. Now a retired instance on a node that is not lost, whose machine is
  reported unreachable or omitted, can be unregistered; PLC-023 then does not ask for the stop
  and reports unavailable. LIF-036 removes the instance "once the stop has ended, or has not
  been dispatched because the runtime provider is not ready or" lacks Conduit's extension,
  "whatever came of it", and does not name this case. One engine removes the instance at once,
  as for a runtime provider that is not ready; another reports unavailable and keeps it until
  the machine is reachable or reported gone. Not changed here: either answer is a decision.
- LIF-018 is left as it is, as M41's reason leaves statements that only describe the lost-node
  case. Its `then` names a reconciliation and a further retirement as what ends an unknown
  shutdown, and its observable names a request to unregister only for an instance whose node
  was declared lost; a request to unregister now removes any retired instance, under LIF-017.

## Checks

Run from the repository root after the edits, with the other passes' edits in the working tree:

- The validation of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): 450 promises ok.
- `python3 tools/check_docs.py`: 2 errors, both a missing path
  `../../docs/proposals/reaudit/round-5/log-fix-F5.md`, in `spec/core/README.md` and
  `spec/schemas/README.md`: other passes' files, linking a log that did not exist yet. Noted,
  not fixed. This log adds no error.
- `python3 tools/set_names.py --check`: 0 errors.
- `python3 tools/check_schemas.py`: 0 errors.
- `git diff --check`: no output.
- No line of `spec/conventions/*.yaml` is over 96 characters.
