# Round 6: log of the integration

Date: 2026-11-07. Six editors applied [ruling 44](../../ruling-44/README.md) to disjoint files,
each with its own log beside this one: [interface](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-interface.md),
[conventions](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-conventions.md), [promises](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-promises.md), [core and schemas](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-core.md),
[definitions and spec docs](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-specdocs.md) and [tracker and entry points](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-tracker.md).
This pass read their files together and made them agree: every citation exists and says what
it is cited for, no group's text contradicts another's, the counts in the entry points agree
with the files, and every decided item of the ruling but D06 and D12 has landed. It edits in
place, adds no promise and removes none, and keeps every ID. Each changed promise file gained a
head-comment sentence for this pass. Nothing was committed or staged.

The independent check of the findings and the simplifications (the journal's `verify:01` to
`verify:05`, `verify:status` and `verify:simplify`) was read first, with the six logs.

## Left for a decision

None of these is decided by the record, and this pass decided none.

1. **A replace read from a directory.** SUB-009 takes the manifest as a value, so a relative
   `fetch` or program is rejected (MAN-006, MAN-010), which D11's own case of editing a local
   project meets. SUB-010 makes every engine take a directory for a registration and not for a
   replace. The promises pass recommends deciding it with the S1 measurement of the edit cycle
   ([its log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-promises.md#for-a-decision)).
2. **A key of an operation's own root `$defs` equal to a copy's key.** Since D14, an ordinary
   operation's root may state `$defs`, whose keys are the author's (OBJ-019), and IFC-033 keeps
   them beside the copies. A key such as `example.org/acme/files@0.3.0#path.v1` written there
   would collide with a copy. Keys of copies cannot collide across packages, as decided; this
   collision is within one contract. A rule in CTR-012 that rejects a root `$defs` key holding
   `#` would close it ([the interface log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-interface.md#for-the-coordinator)).
3. **How INS-010 compares two destinations.** CTR-006 and PRV-016 now compare pointers by
   unescaped reference tokens, with the empty pointer over all, and the schema profile says so
   for them. INS-010 does not say how it compares; one sentence would make the three agree
   ([the docs log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-specdocs.md#for-the-coordinator)).
4. **A rule of the checker for SEC-005**, beside the new one for PRV-016: a few lines and an
   example ([the core log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-core.md#for-a-decision)).
5. **A vector for the escaped `$ref` of a copy.** IFC-033 writes `/` as `~1`, `~` as `~0` and
   `#` as `%23` in a `$ref` to a copy; nothing checks that form until the vectors (A39).

Also for the owner, as the logs and DECISIONS.md say: IFC-047 as a promise of its own; the
reason invalid, not refused, for a call to the tool of a control that the target does not
have (IFC-012, OPS-005, TPL-013); and the costs of 02 (a repeated read can end unknown; a lost
result of an operation with an effect and outputs, through a filtered Conduit, is not
recovered by repeating).

Status at this condensation: item 1 waits for the S1 edit-cycle measurement; items 3 and 4
remain open. Item 2 was settled by [2-02](../../first-build/README.md#2-02);
item 5 is covered by the `e05-escaping` vectors. The current list is in the
[tracker](../../../AUDIT.md#known-to-be-open). The older list and recommendations keep their words.

## Verification

Date: 2026-11-07. Two verifiers read the result of this pass and of the six editors' passes,
and reported twenty findings, C-01 to C-07 and R-01 to R-13. Each was checked against the
working tree and against the record it rests on: ruling 44, the two reports of round 6,
DECISIONS.md and the independent check (the journal's `verify:01` to `verify:05`,
`verify:status` and `verify:simplify`). A finding that held was fixed where the fix follows
from ruling 44, the round-6 reports or the wording in force, and is otherwise listed below for
the owner. Several findings name one defect twice: C-01 and R-12, C-02 with R-01 and R-02, and
C-03 with R-09. Nothing was committed. No promise was added, removed or moved, and every
changed file has a head-comment sentence for this verification.

| Finding | Severity | Verdict | Outcome | What changed |
| --- | --- | --- | --- | --- |
| C-01 | wrong | Holds: IFC-010's table read an error as failed for any operation that states no `effect`, and nothing keeps a plain implementation from serving a control or an operation marked `startsModelWork: true`, where an error may follow part of the work. | Fixed | IFC-010's `then` uses IFC-033's test: an error is failed for an operation that states no `effect`, is not a control and starts no model work, invalid parameters apart, and unknown for any other; its observable says why. IFC-007, IFC-045 and OPS-007, the row plain of `spec/definitions.md`, the overview, DECISIONS.md (01), the tracker and round 5's note on V2-15 follow. E01 of the experiment program (S4) adds the case of an operation marked `startsModelWork: true` that states no `effect`, and the task's failed error. Decided item 02 already glosses "no declared effect" as IFC-033's test, and the independent check found the literal test wrong; 01 uses the same words. The table has one test, so the change reaches every error of a plain server on such an operation, not only -32021: it was failed, it is now unknown. |
| C-02 | undetermined | Holds: read in order, row 5 caught `retire-with-resources.v1` before row 8, and rows 1 and 2 caught `register-replacement.v1`, which LIF-009 calls a registration, before row 3. | Fixed | ACC-033: rows 1 and 2 name `register.v1`, row 2 names `replace.v1` and `unregister.v1`, row 5 names `set-up.v1` and `retire.v1`, rows 6 and 7 `inspect.v1`, and row 8 `retire-with-resources.v1`, as the old prose tied them. Each of the fourteen management verbs was traced through the list: each reaches exactly the row that gives the answer of the text at `c4d7d33`, rows 12 to 14 keep their order for reconciliations, and no row is unreachable (E18). |
| C-03 | undetermined | Holds: since D14 a root `$ref` may reach a property whose `default` SCH-008 accepts where it is written, and no promise said whether it is filled. | Fixed, by the option that rejects nothing | SCH-009's observable: such a `default`, reached from the root of an Operation's `inputSchema` only through a keyword beside its `properties`, is not applied, as one reached only through a conditional keyword already was not; the arguments as a whole are no input's schema, and OPS-026 fills the defaults of inputs alone. CTR-012's observable points to it. This states what the promises already gave, since none filled such a default, and keeps SCH-008's rule that a `default` is judged where it is written, so a Schema contract with defaults can still be referred to from a root. Rejecting the contract instead, the verifiers' other option, would judge a `default` by where it is reached, against SCH-008; it is listed for the owner below. E16 (S5) adds the case. |
| C-04 | misleading | Holds. | Fixed | The tracker (the section on round 5's reversed decision and "To look at after round 6") and round 5's note on V2-15 cite IFC-047 for the duty, use its words "only where it has performed nothing for the call", and list IFC-047 among the promises that changed. |
| C-05 | minor | Holds: TPL-004's `when` does not reach a hook that the type does not require. | Fixed | CTR-004's observable, whose `when` is that a Type contract is read: a default that `implementation` states for a hook that `requires` does not list is used by no template, the type is not rejected, and installation or resolving returns a diagnostic, which does not reject, naming the type and the hook. TPL-004's sentence now points to it. The decided detail allowed TPL-004 or CTR-004. The hook row of `spec/definitions.md`, `spec/README.md`, the tracker and DECISIONS.md (the open point) follow. |
| C-06 | minor | Holds. | Fixed | ACC-008's `then`: every restriction that applies must permit the request, except that a filter judges only a call, so a repeat to which a filter applies and that is not dispatched is judged by no filter, and gives only an outcome already recorded, or reports conflict. The observable explains the cases. |
| C-07 | minor | Holds. | Fixed | The rows repeat and filter of `spec/definitions.md` and the overview say that no outputs the filter has not judged are given; `spec/README.md`'s history names IFC-033's three-part test. |
| R-01 | contradiction | Holds; the same defect as C-02, for `register-replacement.v1`. | Fixed | As C-02. |
| R-02 | contradiction | Holds; the same defect as C-02, for `retire-with-resources.v1`. | Fixed | As C-02. |
| R-03 | contradiction | Holds: a protocol error is no result (the row result), so a plain server's error met OPS-007's first case and IFC-010 at once. | Fixed | OPS-007's first case is a missing result from an implementation that carries Conduit's keys. A plain server's errors, and its missing result, are IFC-010's alone, which gives unknown for no result at all, as before. The observable says so. This also ends the older overlap for -32602 that the verifier named. |
| R-04 | undetermined | Holds in part: the protocol has a server that comes to need an undeclared capability answer -32021, so IFC-047 and the protocol could not both be kept by one that reached that point after acting. | Fixed in part | IFC-047's `then` adds that such an implementation does not begin a call whose processing may come to need a capability that the call does not declare, which the protocol's own rule that a server rely on no undeclared capability already implies; the observable says why it can, inside a task or not, and that one that reached the point after acting would break the promise. The verifier's second step, a result that states unknown in place of -32021, was not taken: the protocol requires -32021 there, and IFC-001 adopts the protocol whole. |
| R-05 | contradiction | Holds. | Fixed | CTR-005's `then`: a filter is applied to each call that the implementations receive for a request through the Conduit, under IFC-019; its observable adds that a repeat that is not dispatched reaches no implementation. |
| R-06 | misleading | Holds. | Fixed | The rows repeat and filter of `spec/definitions.md` say the case of a repeat to which a filter applies and add "or reports conflict"; OPS-008's observable and DL10 add "unless OPS-024 dispatches it". |
| R-07 | misleading | Holds. | Fixed | OPS-005's observable, the row tool of `spec/definitions.md` and the paragraph on results in `spec/core/README.md` add "where the target holds the control's contract", as IFC-012 has it. |
| R-08 | undetermined | Holds, in two parts. | First part fixed; second open | The row bundle of `spec/definitions.md`, `spec/schema-profile.md` and `spec/core/README.md` put the version in a copy's key only where the package or project states one, as IFC-033 does. The second part, a key of an operation's own root `$defs` equal to a copy's key, is item 2 of [left for a decision](#left-for-a-decision), and stays open: it needs a new rule, which no decision gave. |
| R-09 | undetermined | Holds; the same defect as C-03. | Fixed | As C-03. |
| R-10 | misleading | Holds: the diagnostic was made only at the registration of a template that selects no hook, and the claim in `spec/README.md` was wider than any promise. | Fixed | The diagnostic is CTR-004's, made when the type is read (C-05), so it is reported whatever the type's templates select, at installation and in every resolving, a replace's among them. The sentence in `spec/README.md` now says that a promise that leaves part of an accepted document unused says so, as SCH-009 does, and that the hook's default is also named in a diagnostic. |
| R-11 | minor | Holds: since D11 a request for `replace.v1` hands over contracts and implementation items, so it chooses the programs and URLs of later instances, as `register.v1` does. | Fixed | ACC-030's observable, the third point of [what access does not narrow](../../../../spec/README.md#what-access-does-not-narrow) and the row `management` of `spec/definitions.md` say so; DECISIONS.md (D04) notes it. |
| R-12 | minor | Holds; the same defect as C-01. | Fixed | As C-01. |
| R-13 | minor | Holds: a request that matches a repeat that ended with a reason also matches the request that began the invocation. | Fixed | OPS-025's `when` excludes an earlier request that began or repeated an invocation that OPS-008 keeps, and its observable names the two cases, a conflict under OPS-034 and a new attempt under OPS-024 that a filter forbade; the text under "How a request ends" and the row repeat of `spec/definitions.md` follow. |

Fixed: 19, one of them in part (R-04, whose second step was not taken). Open: 1, the second
part of R-08. Of the 20, C-02, R-01 and R-02 are one defect, as are C-01 and R-12, and C-03
and R-09.

### Changed

| File | Promise IDs | Findings |
| --- | --- | --- |
| `spec/interface/implementation.yaml` | IFC-007, IFC-010, IFC-045, IFC-047 | C-01, R-12, R-04 |
| `spec/conventions/operations.yaml` | OPS-005, OPS-007, OPS-008, OPS-025 | R-07, C-01, R-03, R-06, R-13 |
| `spec/conventions/access.yaml` | ACC-008, ACC-030, ACC-033 | C-06, R-11, C-02, R-01, R-02 |
| `spec/promises/contracts.yaml` | CTR-004, CTR-005, CTR-012 | C-05, R-10, R-05, C-03, R-09 |
| `spec/promises/schemas.yaml` | SCH-009 | C-03, R-09 |
| `spec/promises/templates.yaml` | TPL-004, the sentence on a hook's default only | C-05, R-10 |
| `spec/implementation-defined.yaml` | DL10 | R-06 |
| `spec/definitions.md` | rows filter, repeat, plain, tool, bundle, hook and `management`; "How a request ends"; its history | C-01, C-05, C-07, R-06, R-07, R-08, R-10, R-11, R-13 |
| `spec/README.md` | the paragraph after the Conformance table; what access does not narrow; the history of ruling 44 | R-10, R-11, C-07 |
| `spec/core/README.md`, `spec/schema-profile.md` | none | R-07, R-08 |
| `docs/OVERVIEW.md` | none | C-01, C-07 |
| `docs/AUDIT.md` | none: the open point on a hook's default, "Still open", "To look at after round 6", round 5's reversed decision, E01 and E16 | C-01, C-03, C-04, C-05 |
| `docs/proposals/reaudit/round-5/log-fixes.md` | none: the dated note on V2-15 | C-01, C-04 |
| `docs/proposals/reaudit/round-6/DECISIONS.md`, `README.md` | none: 01, D04, D14, the open point on a hook's default, the list of promises changed, "For the owner" | C-01, C-05, R-11, C-03 |

TPL-003, CTR-002's `implementation`, LIF-031 and every other sentence of TPL-004 are unchanged,
and nothing of D06 or D12 is applied.

### For the owner

1. **A key of an operation's own root `$defs` equal to a copy's key (R-08, second part).**
   Since D14 an ordinary operation's root may state `$defs`, whose keys are the author's
   (OBJ-019), and IFC-033 keeps them beside the copies of a tool's bundle. An author's key such
   as `example.org/acme/files@0.3.0#path.v1` would collide with a copy's, and no promise says
   which schema the tool then holds under it. The options:
   - a. CTR-012 rejects a member of an Operation contract's root `$defs` whose key holds `#`,
     which every copy's key holds. It touches only what D14 newly allows, and an author can
     always choose another key.
   - b. The engine puts the copies under one member of the root `$defs` that it reserves, and
     CTR-012 rejects that member's name in an author's root `$defs`. Another reserved name, and
     every pointer to a copy one level deeper.
   - c. The engine renames a colliding key of the author's when it bundles, and rewrites the
     pointers to it. No rule for authors, but a wire form that depends on what else the tool
     refers to, which vectors comparing definitions would have to allow for.

   Recommendation: a.
2. **Behaviors that the verification settled from the decided wording**, which the owner may
   want to read first: the table of IFC-010 now reads every error of a plain server on a
   control, or on an operation marked `startsModelWork: true`, as unknown even where its
   contract states no `effect` (C-01); IFC-047 has an implementation that carries Conduit's
   keys begin no call whose processing may come to need a capability that the call lacks
   (R-04); the diagnostic on a type's default for a hook it does not require is made when the
   type is read (CTR-004; C-05, R-10); and a `default` that the root of an operation's
   `inputSchema` reaches only through a `$ref` or another keyword beside its `properties` is
   not applied (SCH-009; C-03, R-09). For the last, the alternative is to reject such a
   contract under CTR-012; the recommendation is to keep the rule as written, since it rejects
   nothing that D14 allows and judges a `default` where it is written, as SCH-008 does.
