# Round 6: log of the integration

Date: 2026-11-07. Six editors applied [ruling 44](../../ruling-44/README.md) to disjoint files,
each with its own log beside this one: [interface](log-interface.md),
[conventions](log-conventions.md), [promises](log-promises.md), [core and schemas](log-core.md),
[definitions and spec docs](log-specdocs.md) and [tracker and entry points](log-tracker.md).
This pass read their files together and made them agree: every citation exists and says what
it is cited for, no group's text contradicts another's, the counts in the entry points agree
with the files, and every decided item of the ruling but D06 and D12 has landed. It edits in
place, adds no promise and removes none, and keeps every ID. Each changed promise file gained a
head-comment sentence for this pass. Nothing was committed or staged.

The independent check of the findings and the simplifications (the journal's `verify:01` to
`verify:05`, `verify:status` and `verify:simplify`) was read first, with the six logs.

## Changed

| File | Promise IDs | What changed | Why |
| --- | --- | --- | --- |
| `spec/interface/implementation.yaml` | IFC-016 | Observable: a repeat of an invocation that succeeded is answered with its outputs under OPS-009 where no filter applies and they are kept, and reports conflict under OPS-034 where they are not; where a filter applies, it is dispatched as a new attempt under OPS-024 for an operation with no `effect`, not a control and starting no model work, and otherwise reports conflict under OPS-034 or, with no outputs declared, is answered with its outcome alone under OPS-009. | It still gave the test that 6-1-02 removed, "every filter that applies to the repeat was sent with the latest attempt", which the conventions pass flagged. |
| `spec/interface/implementation.yaml` | IFC-025 | Observable: conflict under OPS-034 for an operation that declares outputs, where no filter applies and the outputs are not kept, or where a filter applies and OPS-024 does not dispatch the repeat. | The same old test, "a filter applies to the repeat that the latest attempt was not sent". |
| `spec/interface/implementation.yaml` | IFC-012 | Observable: the exception under which a call to the tool of a control that the target does not have reports invalid holds "where the target holds the control's contract". | Without it, a control that a later release added to a capability, whose contract the target's version does not hold, fell both under this exception (TPL-013: its type does not list it) and under NAM-026 and IFC-033, which say refused. With it, NAM-026 and IFC-033 stay true. |
| `spec/interface/implementation.yaml` | IFC-033, IFC-036 | IFC-033: "as IFC-012 answers a call to any tool that is not presented" becomes "a tool". IFC-036: "as it holds the reason refused in the error of IFC-012" becomes "the reason". | IFC-012 now has one call to a tool that is not presented report invalid. |
| `spec/promises/naming.yaml` | NAM-026 | Observable: "any tool" becomes "a tool", as in IFC-033. | As above. |
| `spec/conventions/operations.yaml` | OPS-005 | Observable: the reason of a call to a tool that is not presented is refused, or invalid under TPL-013 where the call names the tool that an open Conduit would give a control that its target does not have. Rewrapped. | It said refused for every such call, against IFC-012 as the interface pass wrote it; the docs pass flagged it. |
| `spec/core/README.md` | none | The paragraph on results says the same of the reason in the protocol's error, and the history paragraph of ruling 44 names this pass. | It said refused for every such call. |
| `spec/schemas/manifest.yaml` | none | The comment "Not expressed here": the rules on a submitted project's manifest hold of the manifest that `replace.v1` hands over, where it said that manifest holds `packages` alone (SUB-009). A head-comment sentence; the comment rewrapped at 96 characters. No schema changed. | D11 removed that rule; the promises pass flagged the comment. |
| `docs/AUDIT.md` | none | The open point on a hook's default cites TPL-004 alone, and says that the diagnostic is reported at the registration of a template that selects no implementation for the hook. The paragraph "Still open" points to the points below. | It cited CTR-004 too, which says nothing of it: the promises pass put the diagnostic in TPL-004. |
| `docs/proposals/reaudit/round-6/DECISIONS.md` | none | D07's row: the fixed form `allOf`; no tool for a control in either case of TPL-013; a call to such a tool reports invalid in the protocol's form for an unknown tool; the cost restated; the editors' choices named. The list of promises that the findings changed adds IFC-016 and IFC-025 and links this log. "For the owner" adds the choice of invalid over refused. | The row said "for example as `allOf`", named only the template's case, and gave as the cost that such a call is answered as one to any tool that is not presented. |
| `docs/proposals/reaudit/round-6/README.md` | none | The table of records links this log. | So that it is found. |
| `spec/README.md` | none | The History paragraph of ruling 44 says where round 6's decisions and logs are, this log among them. | So that it is found. |

## What was checked

**Citations.** Every promise ID written in a changed or new file exists, by a script over the
working tree: the only IDs that do not are in head comments and in the tracker's table of
later amendments, which name promises that earlier rulings removed (ACC-031, CTR-010, IFC-034,
IMP-006, CON-030 and others), as they did before. `tools/check_schemas.py` also checks 1037
citations in 115 files. Every citation that ruling 44 added or moved was read against the
promise it cites; those found wrong are in the table above. Among those read and found right:
ACC-030's CON-022, CON-028, MAN-012 and ACC-015 to ACC-017 for the broker; SUB-009's MAN-006,
MAN-010, MAN-015, MAN-016, PKG-006, PKG-029, NAM-007, NAM-028, INS-020 and PKG-005; IFC-047's
OPS-007 and OPS-024; OPS-024's IFC-033 and IFC-026; CTR-006's INS-010, REV-006 and REV-007;
PRV-016's CTR-006 and PRV-013; TPL-004's TPL-017 and TPL-018; and the rows of
`spec/definitions.md` that the docs pass changed, each against the promises as they now read.

**Contradictions.** Besides those fixed above, the cases of a repeat of an invocation that
succeeded were read across OPS-009, OPS-024, OPS-034, ACC-008, IMP-007, OPS-028, REV-014,
OPS-030, REC-004, IFC-016, IFC-025, the definitions of repeat, attempt and filter, the core
package's notes and README, and the overview. With and without a filter, with and without an
`effect`, with and without declared outputs, and with outputs kept or not, each case falls to
exactly one of OPS-009, OPS-024 and OPS-034. Both hooks state an `effect` and declare no
outputs, so OPS-028's "a hook that succeeded is not dispatched again" still holds. The -32021
reading was read across IFC-007, IFC-008, IFC-010, IFC-045, IFC-047, OPS-007, the definitions
of plain and of Conduit's extension, and the overview; they agree.

**IFC-047.** The decided detail placed the duty of an implementation that carries Conduit's
keys in IFC-045. The interface pass wrote it as IFC-047, since IFC-045 binds the engine and a
promise holds one obligation, and OPS-007, the definitions, `spec/README.md`, the overview,
DECISIONS.md and the tracker all cite IFC-047. This pass kept it: folding it into IFC-045 would
give one promise two bearers.

**Counts**, by a script over the working tree:

| What | Counted | Where it is stated |
| --- | --- | --- |
| Promises | 453: 202 `language`, 207 `convention`, 44 `interface` | `spec/README.md` (contents and the paragraph after it), `docs/implementation/INDEPENDENT_BUILDS.md` (453, 44 in the interface file) |
| Promises in each file | as each row of the contents table states, all 23 files | `spec/README.md` |
| Contracts in each file of `core/` and `schemas/` | unchanged, as the contents table states | `spec/README.md` |
| Items of the implementation-defined list | 15 | `spec/README.md` |
| Files under `examples/invalid-by-rule`, `examples/invalid` and `examples/valid` | 33, 56 and 7 | `spec/README.md`, `spec/schemas/README.md` |
| Rules of `tools/check_schemas.py` | 27, from 26 promises, by its own output | `docs/AUDIT.md` |
| Round 6's findings and points | 6 findings (3 wrong, 2 of them safety-critical, 2 undetermined, 1 misleading); 22 points: 8 applied, 10 for the experiment program, 2 revised and not yet applied, 2 no change | `docs/proposals/reaudit/round-6/README.md`, DECISIONS.md |

The root `README.md` gives no count and links the contents table.

**Decided items.** Each landed: 01 (IFC-007, IFC-010, IFC-045, IFC-047, OPS-007; V2-15
reversed); 02 (OPS-005, OPS-008, OPS-009, OPS-024, OPS-032, OPS-034, IMP-007, and ACC-008's
sentence that a repeat not dispatched is judged by no filter and returns only an outcome
already recorded; IFC-016 and IFC-025 here); 03 (CTR-006 by unescaped reference tokens with the
empty pointer over all, and the checker's rule `changes-overlap` with two examples); 04
(IFC-033: copies under the root `$defs`, rewritten pointers, sibling keywords kept, recursion,
keys with the address and version, optional inlining, DL12); 05 (PRV-016, rejected at install or
submission, the claim fixed for the instance's life; PRV-013; the rule `fixed-schemes`); 06 (the
entry points and records); D07 (IFC-033's `allOf`, IFC-012); D11 (SUB-009, NAM-022, PKG-029,
`replace.v1`; not an apply of several templates); D14 (CTR-012 and `operation-spec.v1`, with
`outputSchema` read in the protocol's 2026-07-28 schema, `type: object` kept at both roots, and
SCH-008's ban kept; a control's root strict); D03 and D04 (ACC-029, ACC-030 and
`spec/README.md`, with the broker); D18 (ACC-033's list); D21 (reading paths); the hook default
(TPL-004, with a diagnostic) and the other three open points (tracker entries); E01 to E20,
D01, D02 and the measurements (the experiment program). D06 and D12 are not applied, and every
file that names them says so, with A69. TPL-003, CTR-002's `implementation` and LIF-031 are
unchanged, and TPL-004 changed only by the sentences on a hook's default.

## Left for a decision

None of these is decided by the record, and this pass decided none.

1. **A replace read from a directory.** SUB-009 takes the manifest as a value, so a relative
   `fetch` or program is rejected (MAN-006, MAN-010), which D11's own case of editing a local
   project meets. SUB-010 makes every engine take a directory for a registration and not for a
   replace. The promises pass recommends deciding it with the S1 measurement of the edit cycle
   ([its log](log-promises.md#for-a-decision)).
2. **A key of an operation's own root `$defs` equal to a copy's key.** Since D14, an ordinary
   operation's root may state `$defs`, whose keys are the author's (OBJ-019), and IFC-033 keeps
   them beside the copies. A key such as `example.org/acme/files@0.3.0#path.v1` written there
   would collide with a copy. Keys of copies cannot collide across packages, as decided; this
   collision is within one contract. A rule in CTR-012 that rejects a root `$defs` key holding
   `#` would close it ([the interface log](log-interface.md#for-the-coordinator)).
3. **How INS-010 compares two destinations.** CTR-006 and PRV-016 now compare pointers by
   unescaped reference tokens, with the empty pointer over all, and the schema profile says so
   for them. INS-010 does not say how it compares; one sentence would make the three agree
   ([the docs log](log-specdocs.md#for-the-coordinator)).
4. **A rule of the checker for SEC-005**, beside the new one for PRV-016: a few lines and an
   example ([the core log](log-core.md#for-a-decision)).
5. **A vector for the escaped `$ref` of a copy.** IFC-033 writes `/` as `~1`, `~` as `~0` and
   `#` as `%23` in a `$ref` to a copy; nothing checks that form until the vectors (A39).

Also for the owner, as the logs and DECISIONS.md say: IFC-047 as a promise of its own; the
reason invalid, not refused, for a call to the tool of a control that the target does not
have (IFC-012, OPS-005, TPL-013); and the costs of 02 (a repeated read can end unknown; a lost
result of an operation with an effect and outputs, through a filtered Conduit, is not
recovered by repeating).

## Validation

Run from the repository root on the working tree with every pass's changes, after this pass:

```text
$ /usr/bin/python3 - <<'PY'   # the validation of CONSOLIDATION.md
453 promises ok
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 188 Markdown files and 2631 local links (offline).
Errors: 0
$ python3 tools/set_names.py --check
Names: corePackage x443, apiVersion x262, metaPrefix x103; of `identity`, CONDUIT_URL x19,
CONDUIT_IDENTITY_DIR x10, ca.crt x11, jwks.json x12, engine-id x14, token x18, tls.crt x16,
tls.key x14. Errors: 0
$ python3 tools/check_schemas.py
6. Applied 27 rules of this tool, which come from 26 promises, to the core package and 3
example directories. 33 sets of documents in 33 files under examples/invalid-by-rule/ were
each rejected by the rule they are named for; each of their 57 documents is valid against
the schemas.
Errors: 0
$ git diff --check
(no output)
```

No line of a changed or new YAML file is over 96 characters. `git diff --check` does not read
untracked files; this log was checked by hand for trailing whitespace.

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

### Validation of the verification

Run from the repository root on the working tree after these fixes. The promises number 453, as before: 202 of the layer `language`, 207
`convention` and 44 `interface`, so no count in the entry points changed.

```text
$ /usr/bin/python3 - <<'PY'   # the validation of CONSOLIDATION.md
453 promises ok
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 188 Markdown files and 2644 local links (offline).
Errors: 0
$ python3 tools/set_names.py --check
Names: corePackage x443, apiVersion x262, metaPrefix x103; of `identity`, CONDUIT_URL x19,
CONDUIT_IDENTITY_DIR x10, ca.crt x11, jwks.json x12, engine-id x14, token x18, tls.crt x16,
tls.key x14. Errors: 0
$ python3 tools/check_schemas.py
6. Applied 27 rules of this tool, which come from 26 promises, to the core package and 3
example directories. 33 sets of documents in 33 files under examples/invalid-by-rule/ were
each rejected by the rule they are named for; each of their 57 documents is valid against
the schemas.
Errors: 0
$ git diff --check
(no output)
```

No line of a changed YAML file is over 96 characters. `git diff --check` reads tracked files,
and every file changed here is tracked.
