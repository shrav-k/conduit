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
