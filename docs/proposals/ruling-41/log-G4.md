# Ruling 41: log of group G4 (operations, revisions, records)

Date: 2026-11-06. This is the account of how the editor of group G4 applied
[the work order](PLAN.md#g4-operations) for [ruling 41](README.md), which applies
[ruling 40](../ruling-40/README.md) and the [principles review](../principles-review/README.md).
The rulings are the owner's. The wording that carries them out is the editor's, and each choice
below stands unless the coordinator or the owner strikes it. The vocabulary audit that ruling
41 orders has not run: every name here is the plan's or the review's.

Files: [operations.yaml](../../../spec/conventions/operations.yaml),
[revisions.yaml](../../../spec/conventions/revisions.yaml) and
[records.yaml](../../../spec/conventions/records.yaml). No other file was touched.

| File | Before | Removed | Rewritten | Added | After |
| --- | --- | --- | --- | --- | --- |
| `spec/conventions/operations.yaml` | 31 | 3 | 15 | 1 | 29 |
| `spec/conventions/revisions.yaml` | 13 | 1 | 9 | 2 | 14 |
| `spec/conventions/records.yaml` | 6 | 0 | 4 | 0 | 6 |
| All | 50 | 4 | 28 | 3 | 49 |

These are the plan's counts. The validation script of
[CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) passes on the tree as it stood when
this group finished, with other groups still at work. Every line of the three files is at most
96 characters, and `git diff --check` reports nothing for them. Every promise ID that the three
files cite exists once the plan's removals and additions are made, and none of them cites a
removed ID.

## Promises

| ID | Action | Decisions | What changed |
| --- | --- | --- | --- |
| OPS-005 | Rewritten | 6b, 24a | `when`: not a repeat that OPS-009 or OPS-012 answers with the invocation's outcome. Observable: a result that reports invalid or conflict carries an error diagnostic whose `ruleId` is the promise that decided it (IFC-025); a repeat under OPS-032 reports conflict. |
| OPS-007 | Rewritten | 16a | Observable: an output that the `outputSchema` lists under `required`, in place of one not marked `optional: true`. |
| OPS-008 | Rewritten | C33, 1c | `then`: the engine keeps, as its own state apart from the records, what repeats and reconciliations need, for every invocation; no purge touches it. Observable: "read from the records" and the retrieval-shaped "asking" go; the scope's one exception is a hook's identity (OPS-028). |
| OPS-009 | Rewritten | 6a, 6b, 6c | `when`: the invocation succeeded and its outputs are still kept. `then`: the result is the outcome succeeded and the outputs. Observable: the in-progress, withheld and retrieval sentences go; OPS-032, OPS-012 and OPS-024 named for the other repeats; the access sentence kept. |
| OPS-014 | Rewritten | 2a, 6c, 17 | Observable: the ID is learnt from the result's `_meta` or the records; `retrieve-result.v1` and TPL-015 go; "unregistered" → "no longer exists"; a run of a setup or retirement of unknown outcome is checked under PRV-009. |
| OPS-015 | Removed | 6c | Retrieval goes. Nothing replaces it: a caller that lost a result repeats the request (OPS-009, OPS-032). |
| OPS-016 | Removed | 6c | As OPS-015. |
| OPS-017 | Removed | 6a | Nothing is withheld; nothing replaces it. |
| OPS-019 | Rewritten | 3a, 10 | `when` drops rename and apply a policy; the example verbs are `register.v1`, `set-up.v1`, `open.v1` and `install.v1`. |
| OPS-021 | Rewritten | 1, 1c, P20 | About a setup hook or a retire hook of unknown outcome, in place of a step: it is dispatched again only as a new attempt under OPS-012, with OPS-028's identity. Observable: an open Conduit, a ready Resource and a running run are left as they are (CON-004, POOL-013, PRV-007); the step, DL1 and runtime sentences go. |
| OPS-022 | Rewritten | 1 | `when`: an invocation of `set-up.v1` or `retire.v1` (the graph case goes). Steps → parts, those of TPL-012, TPL-009 and TPL-018; the graph-output and composite sentences go. |
| OPS-023 | Rewritten | 3a | Observable: "the state of the Conduits", without policies. |
| OPS-024 | Rewritten | 6a, 6c | Observable: the retained-result and withheld sentences go; the close of the latest attempt's Conduit is ACC-034's. |
| OPS-026 | Rewritten | 16a, 1 | The default is the one the input's property states in the `inputSchema`; validated against the `inputSchema`; the graph and GRA-007 sentences go. |
| OPS-027 | Rewritten | 22 | Cancellation is the protocol's: `notifications/cancelled`, the close of the response stream on Streamable HTTP, `tasks/cancel`; the engine passes it on; DL14 goes; a dropped HTTP connection cancels. |
| OPS-028 | Rewritten | 1c, P20 | About a setup hook or a retire hook, in place of a step: an identity derived from the instance's ID and the hook's operation, the same in every setup or every retirement, scoped to the instance alone. |
| OPS-029 | Rewritten | 3a, 8, 10, 17, P9 | `when`: an object or an invocation (no policy). Observable: the list drops `rename.v1`, the policy operations, `set-default-location.v1`, PLC-022 and OPS-015; "unregistered or removed"; `register-replacement.v1` names the holder in `instance`. |
| OPS-031 | Rewritten | 2a, 15 | Only `revision` can be out of place now: `handle` and `settings` go throughout. |
| OPS-032 | Added | 6b, P8 | A repeat of an invocation that has not ended, or of one that succeeded and whose outputs are no longer kept, is not dispatched, reports conflict, and holds the engine's ID in its result's `_meta`. |
| REV-001 | Rewritten | C21 | Observable: a revision is an opaque non-empty string compared only for equality. |
| REV-003 | Rewritten | 10 | A name no longer changes, so only the definition or the configuration advances the revision. |
| REV-004 | Rewritten | 10, 15 | As REV-003; "a change that confirmed the values already stored" → "a control that succeeded with values equal to those already stored". |
| REV-006 | Rewritten | 16a | Observable: "whether or not the control's `inputSchema` lists it under `required`". |
| REV-007 | Rewritten | 15 | A control applies exactly the values it was sent, and those are stored; it returns no `settings`; no clamping. |
| REV-008 | Removed | 15 | No confirmed settings remain to check; REV-015 checks the values before dispatch. |
| REV-010 | Rewritten | 15 | A reconciliation that records a control as having taken effect stores the values the control was sent, which the engine keeps under OPS-008; the `settings` input and its shape go; "no longer exists". |
| REV-011 | Rewritten | 10 | Observable: the list drops `rename.v1`. |
| REV-012 | Rewritten | 10, 23 | Observable: "renames among them" goes; a request that arrives meanwhile is REV-014, in place of DL9. |
| REV-013 | Rewritten | 15 | About the values the control was sent, which would fail the type's schema once applied to the current configuration; the `settings` key sentence goes. |
| REV-014 | Added | 23 | A request to set up, retire or change an instance while another of them, the engine's own included, has not ended reports conflict at once. |
| REV-015 | Added | 15 | A control whose values would make the configuration fail the type's schema is not dispatched and reports invalid. |
| REC-001 | Rewritten | P5, S4.6, 17 | `when` adds a provider call and the removal of an object. Observable: a provider call is recorded; a read of a secret is recorded as a request, without its value. |
| REC-002 | Rewritten | 24c, 2 | Observable: the OBS-007 sentence goes; a record's form is `record.v1`, a CloudEvent with these fields in `data`, which `inspect.v1` returns and an engine may export (DL8); the records of a read and of a secret reference (SEC-015, SEC-004). |
| REC-003 | Rewritten | 17 | "unregistered" → "unregistered or removed". |
| REC-004 | Rewritten | C33, 6c, 2 | `then`: a purge removes the records before a time and leaves a record of itself; the list of kept fields goes. Observable: what repeats and reconciliations need is the engine's state under OPS-008; the retrieval and runtime sentences go. |

Unchanged, as the plan lists them: OPS-001 to OPS-004, OPS-006, OPS-010 to OPS-013, OPS-018,
OPS-020, OPS-025, OPS-030; REV-002, REV-005, REV-009; REC-005, REC-006. OPS-013 still says
"asking for the invocation's state"; with retrieval gone that is a repeat under OPS-012 or a
reading of the records, and the sentence needs no change.

Head comments: each file gains the line of rule 7 of the plan, naming what was removed, with
the ruling and where its content went, what was changed and what was added. In
`operations.yaml` the descriptive sentence "[the invocation's state] is read from the records
about the invocation" was also changed, since it no longer holds (C33); the historical lines,
such as "No promise of this file has been removed", were left as the record of their time, and
the new line says that these are the first removals.

## Choices

Where the plan left wording open, the editor chose as follows.

| # | Where | Choice | Why |
| --- | --- | --- | --- |
| E1 | OPS-009, OPS-032 | OPS-009's `when` requires that the engine still keeps the outputs, so that OPS-009 and OPS-032 never both apply. | The plan puts DL10's limit in OPS-009's `then` and the case of outputs no longer kept in OPS-032; stated once in `when`, the two promises do not overlap. |
| E2 | OPS-009 | An operation that declares no outputs has none to lose: a repeat of a succeeded invocation of it is always answered with the outcome and an empty `structuredContent`. | Carried from the current IFC-025 ("content to keep only where its operation declares outputs"); without it, OPS-032 could report conflict for an operation with nothing to keep. |
| E3 | OPS-008 | The engine's state is kept "for every invocation", the operation identity "where it carries one" and the Conduit "where it went through one". | `reconcile.v1` names any invocation by the engine's ID, with or without an identity, as REC-004 had it; a hook goes through no Conduit. |
| E4 | OPS-008 | "Asking by that caller, target and identity returns the state" became a repeat being answered from the one invocation (OPS-009, OPS-012, OPS-024, OPS-032). | Retrieval was the only other way to ask, and it goes. |
| E5 | OPS-021, OPS-028 | A later dispatch of a hook is matched against the earlier invocation as OPS-009 matches a repeat, whoever is the caller of the setup or retirement; a hook that succeeded is not run again, one of unknown outcome only as OPS-012 allows, one that failed as a new attempt under OPS-024. | P20: the hook's identity is scoped to the instance alone. The caller of such a new attempt is left open (D1). |
| E6 | OPS-022 | The parts of a retirement are those of TPL-009 and TPL-018, so the run that a retire hook needs (P4) counts; only a declared Conduit or an entry can be marked optional, and the stop of a run is not. | The old text made each stop of a runtime a step that is not optional; the plan's term "optional" names only those four things, two of which are parts. |
| E7 | OPS-022 | "A step that GRA-003 leaves as succeeded" became "a part that CON-004 or POOL-013 counts as succeeded because what it does is done already". | GRA-003's rule moved into CON-004 and POOL-013 (plan, G1 and G3). |
| E8 | OPS-027 | The observable says that the protocol has a receiver that stops send no response, so a cancelled invocation is usually unknown until reconciled; a `tasks/cancel` is decided under IFC-016; how long the engine waits is DL3 and DL10. | The baseline's words, as [the round-2 citations](../reaudit/round-2/mcp-citations.md#what-the-file-does-not-claim) quote them ("Closing the SSE response stream is the cancellation signal"; a server should "Not send a response for the cancelled request"); the review's decision 22 names DL3 and DL10. |
| E9 | OPS-014 | The ID is learnt from the `_meta` of the request's result or of a repeat's (IFC-025 holds it for a request with an outcome and for OPS-032), or from the records (REC-002). To reconcile a setup or retirement that asked for a run or its stop, the actor checks the run's state under PRV-009. | The plan's "the result's `_meta` or the records", and its "checked by PRV-009". OPS-014 does not say whether a provider call can be reconciled by itself (see I3). |
| E10 | OPS-005 | Added that a repeat under OPS-032 is not dispatched and reports conflict. | Its `when` now admits that repeat, and saying so keeps the reader from looking for its outcome. |
| E11 | OPS-029 | The LIF-032 sentence, which hung on the removed PLC-022 sentence ("So is ..."), now stands alone; a `nested` that names nothing the holder holds is LIF-033's. | The removed sentence was its antecedent. |
| E12 | REV-001 | "There is no second counter" → "There is no second revision"; the comparison follows an HTTP entity tag, as C21 says: nothing is read from a revision's form. | An opaque string is not a counter. |
| E13 | REV-007 | The example of a clamp (20 asked, 15 confirmed) became the rule against one: 20 is stored, or the control does not succeed. | Decision 15 forbids clamping. |
| E14 | REV-010 | "The values are not stated in the request: the engine stores those it sent." | Follows from removing the `settings` input; the engine keeps the values under OPS-008. |
| E15 | REV-013 | Observable says when it arises: another change was stored after the control was dispatched, since REV-015 checked the same values against the configuration then. | It explains why the promise survives decision 15. |
| E16 | REV-014 | The exception is "a repeat that OPS-009 or OPS-012 answers without a dispatch", where the review and the plan say "a repeat under OPS-009". A repeat that OPS-012 or OPS-024 dispatches as a new attempt is subject to REV-014. | After 6b, OPS-009 no longer covers the repeat answered with the outcome unknown; the exception's purpose, not reporting a busy conflict for what dispatches nothing, covers it too. Strike to keep the literal text. |
| E17 | REV-014 | "A control is a change, and so is a reconciliation that stores the values of a control (REV-010)." The engine neither queues nor waits; the caller asks again later. | Names what "change" covers, from REV-010; restates "at once" as observable behavior. |
| E18 | REV-015 | The check is made after the defaults of supplied values are filled (OPS-026), with an unsupplied input's setting checked as stored (REV-006); the example uses `maximum`. | The plan's text with what it implies from OPS-026 and REV-006. |
| E19 | REC-001 | A provider call is recorded when it is made and when it ends, "with how it ended". | PRV-005 says it is recorded under REC-001, and REC-001 lists events. |
| E20 | REC-002 | The removed OBS-007 sentence is replaced by "every other request has a caller": a read of a secret has the caller of the request that asked for the run (SEC-011), and a provider call's record holds the caller of the invocation it serves where it has one (PRV-005). `inspect.v1` returns records as `record.v1` (P19). | Decision 2 and 4c remove the request for no actor; the plan's terms say every request has a caller. |
| E21 | REC-004 | The sentence "What a purge keeps of an invocation is read as records of that instance" was cut. | Under C33 a purge keeps nothing as records. See D3. |

## For the integrator

What cites a promise that this group removed, as the tree stood at commit `7cd6e2c`, outside
the three files (inside them, nothing does any longer):

| Removed | Cited by |
| --- | --- |
| OPS-015 | IFC-021, IFC-024 (removed by G5), `spec/core/README.md`, `spec/definitions.md` (retained result) |
| OPS-016 | ACC-001, IFC-016, IFC-021, IFC-024 (removed by G5), `spec/core/README.md`, `spec/definitions.md` (retained result, retrieval tool), `spec/implementation-defined.yaml` (DL10) |
| OPS-017 | ACC-034, IFC-024 (removed by G5), IFC-025, `spec/core/README.md`, `spec/definitions.md` (retained result, withheld) |
| REV-008 | `spec/core/types.yaml`, `spec/schemas/README.md` |

Promises of other groups that these files now cite, as the plan writes them: TPL-009, TPL-012,
TPL-017, TPL-018, LIF-033 (rewritten for P9) and LIF-013 (P19) of G1; CON-004 and POOL-013
(parts that count as succeeded) of G3 and G2; PRV-005, PRV-007 and PRV-009 of G2; SEC-004,
SEC-010, SEC-011 and SEC-015 of G7; IFC-016 and IFC-025 (the `invocation` key on an OPS-032
result, the `ruleId` diagnostic) of G5; OBJ-015 (optional by `required`) and CTR-006 (no output
`settings`) of G6; and the implementation-defined items DL2 (the engine keeps the digest under
OPS-008), DL3 (a repeat reports conflict under OPS-032), DL8 (export as `record.v1`) and DL10
(how long outputs are kept for a repeat under OPS-009).

Further things the integrator may check:

- I1. `spec/definitions.md`: the rows to rewrite are listed under [Terms](#terms).
- I2. The core package: `reconcile.v1` loses `handle` and `settings` (OPS-031, REV-010), no
  control declares an output `settings` (REV-007), and `purge.v1`'s description no longer says
  what a purge keeps (REC-004).
- I3. OPS-014 and OPS-022 assume that a provider call is not itself reconciled with
  `reconcile.v1`: the setup or retirement it was made within is. That rests on the reading of
  PRV-005 (G2) under which the `invocation` a provider call carries is that of the invocation it
  serves; if G2 gives a provider call an ID of its own, OPS-014 needs a sentence on it.

## Terms

G4 introduces no new term. It uses "part", "setup hook", "retire hook", "run", "provider call",
"remove" and "further setup" or "further retirement" (as LIF-026 already does) as the plan's
section 1 defines them. For the docs editor, the rows of `spec/definitions.md` that these
changes reach:

- the invocation's state: kept by the engine as its own state, apart from the records, and not
  read from them (OPS-008); no key `state` holds it on a result.
- in progress: no longer written `inProgress` on the wire; a repeat of such an invocation
  reports conflict (OPS-032).
- repeat: its result is the invocation's outcome and outputs (OPS-009), or conflict while the
  invocation has not ended or its outputs are no longer kept (OPS-032).
- the engine's ID for an invocation: returned in the result's `_meta`, an OPS-032 result
  included; no retrieval tool.
- attempt, new attempt: a hook in place of a step (OPS-021, OPS-028).
- operation identity: the engine gives each hook one, scoped to the instance alone (OPS-028).
- reconcile: no `handle` and no `settings`; the values of a control are the ones the engine
  sent (REV-010); TPL-015 goes.
- cancel: the protocol's cancellation (OPS-027); DL14 goes.
- control: no output `settings`; it applies exactly what it is sent (REV-007); its values are
  checked against the type's schema before dispatch (REV-015).
- revision: an opaque non-empty string compared only for equality (REV-001); a name no longer
  advances it (REV-003).
- record: its form is `record.v1`, a CloudEvent (REC-002); a provider call and a removal are
  recorded (REC-001).
- purge: removes records and keeps none of an invocation's fields; repeats and reconciliations
  use the engine's state (REC-004, OPS-008).
- retained result, retrieval tool, withheld: go (OPS-015 to OPS-017 removed).

## Still needs a decision

None of these was decided in the text; each is left as the rulings and the plan leave it.

| # | Where | Question |
| --- | --- | --- |
| D1 | OPS-028, TPL-017, OPS-008 | A hook is dispatched "for the caller of the setup" (TPL-017), an invocation is for one caller (OPS-008), and the hook's identity is scoped to the instance alone (P20). When a further setup or retirement sent by another actor finds an earlier hook invocation that failed, or that is of unknown outcome on an operation that deduplicates, and dispatches a new attempt of it, for which caller is that attempt made, and whose restrictions does it carry (ACC-018)? |
| D2 | OPS-032 | Its result does not say which of its two cases it is. A caller cannot tell an invocation still in progress, which a later repeat will answer, from one whose outputs are gone, which no repeat will. Options: a diagnostic whose message says which, a key of its own, or leave it to the records. |
| D3 | REC-004, OPS-008, LIF-013 | Before C33 a purge kept an invocation's fields as records of its instance, so inspecting the instance still showed an invocation of unknown outcome and its ID. Now inspection lists only records. After a purge, an invocation of unknown outcome whose request carried no operation identity, and whose result was lost, cannot be found to be reconciled. Options: inspection also lists, from the engine's state, the invocations of unknown outcome dispatched to the instance; or accept it. |
| D4 | REV-014 | REV-014 tells a request to report conflict while the engine's own setup or retirement is in progress. What the engine does with its own setup or retirement that would begin while another of the instance's has not ended is not decided: a retirement because a machine is reported gone (PLC-024), a location's setup (PLC-026) or another retirement without a request (LIF-023). It is not a request, so it cannot report conflict. |
| D5 | OPS-027 | A cancelled `set-up.v1` or `retire.v1` is implemented by the engine itself. What the engine does with the parts of that setup or retirement that are in progress (an `open.v1`, a Resource's setup, a run, a hook) is not decided: whether it cancels them by the protocol's means, lets them run to their end, or stops at the next part. |
