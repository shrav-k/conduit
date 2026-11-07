# Round 6: log of the core-and-schemas editor

Date: 2026-11-07. This pass applies [ruling 44](../../ruling-44/README.md) to the core package,
the schemas, their examples and `tools/check_schemas.py`: findings 6-1-03 and 6-1-05 of
[reader 1](reader-1.md), with a rule of the tool and examples each; the critique's D11, on the
inputs of `replace.v1`; D14, in `operation-spec.v1` and a valid example; D07 and finding 6-1-04
where the core package and the schemas describe the tool that the engine presents; and finding
6-1-02 where `spec/core/operations.yaml` and `spec/core/README.md` describe repeats. The
verdicts, evidence and risks of the independent check of the six findings and of the
simplifications were read first.

Files: `spec/core/README.md`, `spec/core/operations.yaml`, `spec/core/capabilities.yaml`,
`spec/schemas/README.md`, `spec/schemas/contracts.yaml`, `spec/schemas/templates.yaml`,
`spec/schemas/examples/valid/files-package/contracts.yaml` and `conduit.yaml`,
`spec/schemas/examples/valid/project/templates.yaml`, one file under
`spec/schemas/examples/invalid/` (renamed), three new files under
`spec/schemas/examples/invalid-by-rule/`, and `tools/check_schemas.py`. Nothing else was
edited apart from this log. Nothing was committed or staged; the rename was made with `mv`, so
Git shows it as a deleted file and an untracked one.

No promise is in these files, so no promise ID was changed, added or moved. The column
"Promises" names those that each change states, as the promise editors wrote them. No export
of the core package was added or removed. Each changed YAML file and the tool gained a
head-comment line for ruling 44 and round 6, and each changed README a history paragraph.
Folded text and changed paragraphs were rewrapped at no more than 96 characters a line.

## Applied

| What | Files | Promises | From | Why |
| --- | --- | --- | --- | --- |
| A function `tokens` splits a JSON Pointer into its reference tokens, unescaping `~1` and then `~0` as RFC 6901 says, the empty pointer having none; `overlap` says whether one pointer is the other or lies inside it, as a prefix of tokens; `is_place` and `shown` serve the messages. `check_controls` reads a pointer's tokens with `tokens`, as it did inline | `tools/check_schemas.py` | CTR-006, PRV-016 | 6-1-03, 6-1-05 | The check warned that comparing strings would take `/limits` for a place inside `/limit`, take `/a~1b` for `/a/b`, and miss the empty pointer. Tried in a scratch run on eleven pairs, among them those three, `/schemes/0`, `/schemesX`, `/a~0b` against `/a~b` and `/x/` against `/x/y`: each gave the expected answer |
| Rule `changes-overlap`, for CTR-006, in step 6: no two pointers of an Operation contract's `changes` are equal, and none lies inside another; the message names both inputs and both pointers, as CTR-006's observable asks | `tools/check_schemas.py` | CTR-006 | 6-1-03 | The task asked for the check by reference tokens with an example under `examples/invalid-by-rule`. That directory holds sets only for the rules of step 6 (`RULES`), and `check_controls` is step 5, so the overlap is a rule of step 6, applied to every Operation contract of a set that holds `changes`, whether or not a Type of the set lists it, since CTR-006 binds a control as it is defined. `check_controls` keeps its other checks in step 5 |
| Two examples: `CTR-006-two-inputs-one-setting.yaml` (`limit` and `maxEntries` both `/readLimit`) and `CTR-006-setting-inside-another.yaml` (`/listing` and `/listing/depth`) | `examples/invalid-by-rule/` | CTR-006, REV-006, REV-007, INS-010 | 6-1-03 | The reader's two cases, equal pointers and a parent with its child. Each comment says which of REV-006 and REV-007 could not both hold |
| Rule `fixed-schemes`, for PRV-016, in step 6: a Type that offers the core package's `package-source.v1` lists no control whose `changes` holds a pointer that overlaps `/schemes`: `/schemes`, one inside it such as `/schemes/0`, or the empty pointer. Applied where the tool has the control's contract, in the set or in a package of this repository | `tools/check_schemas.py` | PRV-016 | 6-1-05 | "A check beside SEC-005's if it fits". The tool has no rule for SEC-005 (the README marks its row "—"; a comment in `contracts.yaml` read as if it did, and is corrected below), so the rule sits beside the rules on provider Types, PRV-001 and SEC-022, in the code, and in SEC-005's row of the README's table. Comparing tokens keeps `/schemesExtra` out, as PRV-016's observable says |
| Example `PRV-016-package-source-type-with-a-control-on-schemes.yaml`: the Type `git-source.v1` and its control `set-schemes.v1`, which sets `/schemes` | `examples/invalid-by-rule/` | PRV-016, PRV-013 | 6-1-05 | The comment names the other two forms the rule rejects |
| `operation-spec.v1`: the root of `inputSchema` and `outputSchema` requires `type: object` and admits every keyword of the profile, through `schema.v1`; where the `spec` holds `changes`, an `if`/`then` holds both roots to `type`, `properties`, `required` and `additionalProperties` (`$defs/control-root`). The rules that no property of `outputSchema`, and no handle, states a `default` are kept on the root's `properties`. The description and the comment "Promises encoded" say so | `spec/schemas/contracts.yaml` | CTR-012, CTR-002, CTR-006, SCH-008 | D14 | CTR-012, as its editor wrote it, keeps `type: object` at both roots and holds a control's two roots to the four keys. A control is told by `changes`, which CTR-006 makes the mark of one. The schema of an Operation is "the checker's CTR-012 rule": the tool has no code of its own for CTR-012. Tried in a scratch run: a root `oneOf`, a root `$ref` with `$defs` and an `outputSchema` with a root `anyOf` are accepted; a `default` under a root `allOf`, a root with no `type`, `type: array`, a boolean schema, a default on an output, a default on a handle, and `oneOf` at either root of a control are rejected |
| The Model Context Protocol's 2026-07-28 schema was read for `outputSchema`, as the task asked: `inputSchema` is `{ $schema?: string; type: "object"; [key: string]: unknown }`, whose comment lets "any JSON Schema 2020-12 keyword" stand beside `type` and advises declaring `properties` at the root with branches that are presence assertions; `outputSchema` is `{ $schema?: string; [key: string]: unknown }`, "any valid JSON Schema 2020-12", with no `type` required | `spec/schemas/contracts.yaml` | CTR-012, IFC-001 | D14 | Followed as far as ruling 44 allows: both roots take the whole profile, and both keep `type: object`, which the ruling names and CTR-012's observable grounds in IFC-001 |
| The example `CTR-012-operation-input-schema-with-another-root-keyword.yaml`, whose `oneOf` at the root of an ordinary operation is now valid, became `CTR-012-control-input-schema-with-another-root-keyword.yaml`, a control, `set-limits.v1`, with the same `oneOf` | `examples/invalid/` | CTR-012, REV-006 | D14 | Otherwise step 4 fails. The count of 56 files under `examples/invalid` is unchanged. The old name is quoted in docs/proposals/ruling-41/PLAN.md, a record of its time, which was left alone |
| `directory.find.v1`, an Operation that is not a control, whose `inputSchema` holds at its root `oneOf: [{required: [name]}, {required: [pattern]}]` beside `properties`, as the protocol advises; exported, and implemented by `files-server` | `examples/valid/files-package/contracts.yaml`, `conduit.yaml` | CTR-012, CTR-002 | D14 | The valid example the task asked for. It is the operation that the old invalid example held, now accepted |
| `replace.v1`: new input `contracts`, a list of `contract.v1`; `manifest` is a whole manifest. The description says that the template, its contracts and its manifest are one project, resolved as `register.v1` resolves one; that nothing of the project the template held before is reached; that it replaces one template and registers nothing; the rules on the manifest of a submitted project (MAN-015, MAN-016, PKG-006, a relative `fetch` or program); and when an equal request changes nothing | `spec/core/operations.yaml` | SUB-009, SUB-008, NAM-009, PKG-029, MAN-015, MAN-016, PKG-006, NAM-007 | D11 | As decided, under the existing check of revisions. `contracts` is typed as contract documents, so the schema itself refuses another template or an instance: it is not an apply of several templates. SUB-009, as its editor wrote it, uses the same two input names, `contracts` and `manifest` |
| The management verbs table: `replace.v1` names SUB-009, SUB-008, PKG-006, MAN-015 and MAN-016 | `spec/core/README.md` | as named | D11 | It now follows a registration's rules |
| The row "That a manifest handed to `replace.v1` holds `packages` alone" of the table of what a schema cannot express became: that the template, contracts and manifest handed to `replace.v1` are resolved as one submitted project, and that the template has the kind and name of the one it replaces; the schemas check that `contracts` holds contract documents alone | `spec/schemas/README.md` | SUB-009, SUB-008 | D11 | The old restriction is gone |
| The notes of `operations.yaml` on a contract's root: the root of each core contract holds `type`, `properties` and `required` and no other keyword; CTR-012 lets that of an operation that is not a control hold any keyword of the profile, and holds a control's to the four keys | `spec/core/operations.yaml` | CTR-012 | D14 | The note said that CTR-012 allows nothing else |
| The paragraph on operations in the core package's README: the same, and that the root of every core operation holds no keyword but `type`, `properties` and `required` (checked: `place.v1`'s `outputSchema` has no `required`) | `spec/core/README.md` | CTR-012, REV-006 | D14 | As above |
| The notes of `operations.yaml` on results: a repeat of an invocation that succeeded is answered from what the engine keeps, unless its outputs are not kept (OPS-034) or a filter applies; then an operation that states no `effect`, is not a control and starts no model work is dispatched again as a new attempt, any other reports conflict where it declares outputs and is answered from its outcome where it declares none; a repeat that is not dispatched is judged by no filter, so where one applies it gives only an outcome already recorded. Also: a Conduit applies no filter where its target selected `engine`, and a request for a core operation that carries one reports unavailable | `spec/core/operations.yaml` | OPS-009, OPS-034, OPS-024, IFC-033, CON-022, MAN-012, ACC-015 | 6-1-02 | As decided. The last sentence says why the change touches no core verb, in the words of CON-022 and MAN-012 rather than a new claim |
| The section on results of the core package's README: the same, in place of "or a filter applies to the repeat that the latest attempt was not sent" | `spec/core/README.md` | as above | 6-1-02 | Not named in the task, which names the comments of `operations.yaml`; the README restated the old rule of M32 in the same words, and would otherwise contradict OPS-009 and OPS-034 as their editor wrote them |
| The tool presented: the definition holds a copy of each Schema contract it refers to under the `$defs` of its root, keyed by the contract's qualified reference and its package's version, so that no two packages' contracts collide, each `$ref` rewritten and every keyword beside it kept, each contract copied once; a contract whose schema holds no `$ref`, referred to with no keyword beside the `$ref`, may stand in place of the reference; so the tools of `register.v1` and `replace.v1` hold a root `$defs` | `spec/core/README.md`, `spec/schemas/README.md` | IFC-033 | 6-1-04 | Where the two READMEs describe the presented tool. The schemas README says that `tools/check_schemas.py` bundles the same way, keyed by bare names because it holds the core package alone |
| The tool presented for a control gives an input that the target's template narrows `allOf` of the contract's schema and the template's, and a control that the target does not have, as where its template's `controls` map leaves it out, has no tool: in the README's paragraph on operations and its bullet on controls, in the description of `template-controls.v1`, and in a comment on `project-directory`, whose `limit` is shown with the contract's maximum of 100 and the template's of 50 | `spec/core/README.md`, `spec/schemas/README.md`, `spec/schemas/templates.yaml`, `examples/valid/project/templates.yaml` | IFC-033, IFC-012, TPL-010, TPL-013 | D07 | Where the core package and the schemas describe the presented tool. The example is the critique's case E10 |
| The bullet on controls of the core package's README says that no two pointers of `changes` overlap, compared as reference tokens | `spec/core/README.md` | CTR-006 | 6-1-03 | It restates CTR-006 |
| The row for CTR-006 of the table of what a schema cannot express adds the overlap, and that the tool checks it in step 6 with two examples; the row for SEC-005 and PRV-016 adds PRV-016's rule on controls and marks the tool's rule; the comment "Not expressed here" of `contracts.yaml` adds both, and no longer reads as if the tool checked SEC-005 | `spec/schemas/README.md`, `spec/schemas/contracts.yaml` | CTR-006, PRV-016, SEC-005 | 6-1-03, 6-1-05 | The old comment ended "which tools/check_schemas.py also checks" after a list that included SEC-005 and PRV-016, which the tool did not check; it now points to the README's table |
| The description of `package-source.v1` says that its Type lists no control whose `changes` reaches `schemes`, so a provider claims the schemes it was made with for its whole life, and that one that is to claim others is a new provider; the README's section on providers says the same | `spec/core/capabilities.yaml`, `spec/core/README.md` | PRV-016, PRV-013 | 6-1-05 | Not named in the task; the capability's description restates PRV-016 beside `secret-provider.v1`'s "which lists no control", and the README restates PRV-013 there |
| The descriptions of `setup-hook.v1` and `retire-hook.v1`: an implementation selected "by the default of a type that lists it under `requires`", and, for the setup hook, "A type's default for a hook that the type does not require is not used" | `spec/core/operations.yaml` | TPL-004, TPL-017, TPL-018 | Open point 1 | Not named in the task; the descriptions said "by its type's default" without the condition, which TPL-004 now states with its diagnostic. One sentence in the setup hook, which is the one that a reader meets first, rather than two |
| Counts and examples of the schemas README: 33 files under `examples/invalid-by-rule`, and the row of `examples/valid/files-package` names the operation with `oneOf` at its root | `spec/schemas/README.md` | — | — | Kept true |

## Choices beyond the record

1. **The overlap of pointers is a rule of step 6, not a check of `check_controls` in step 5.**
   The task names `check_controls` and an example under `examples/invalid-by-rule`; only a
   rule of step 6 can be shown by such an example. `check_controls` uses the same function for
   tokens, and its docstring names the rule.
2. **`changes-overlap` reads every Operation contract that holds `changes`**, and not only
   those that a Type of the same set lists: CTR-006's `when` is "A control is defined", and a
   control is an Operation contract whose `spec` holds `changes`.
3. **No rule for SEC-005 was added**, though it would sit beside the new one, because the task
   asked for PRV-016's rule only. It would be a few lines and an example, if wanted.
4. **A control is told in `operation-spec.v1` by `changes`**, and both of its roots keep the four
   keys, `outputSchema` too, as CTR-012's `when` now says ("either root").
5. **Two examples for CTR-006**, where the task asked for one: the reader's two cases differ in
   kind (equal pointers, and one inside another), and the README's table counts both.
6. **The old invalid example for CTR-012 was turned into a control and renamed**, rather than
   removed: it still shows CTR-012's rule, now for the case that remains.
7. **`directory.find.v1` is exported and implemented by `files-server`** in the example package,
   so that it stands as a real operation of the package; no capability includes it and no type
   requires it, so no template must select an implementation for it.
8. **The description of `replace.v1` says that nothing of the project the template held before
   is reached.** D11 says that the project submitted with the replacement is resolved as
   registration does; SUB-009, as its editor wrote it, says the same ("never those of the
   project that the template held before"). A replace that hands over no contracts and whose
   template still writes `#name` for a contract of the old project is now rejected (NAM-028),
   where it was resolved before.
9. **Sentences outside the named parts were changed where they restated a changed rule**: the
   section on results and on providers of `spec/core/README.md`, the description of
   `package-source.v1`, and the two hooks' descriptions. Each is in the table with its reason.

## Checks

Run from the repository root at the end of this pass, while the other groups were still
editing their files:

- The validation command of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): "453 promises
  ok".
- `python3 tools/check_schemas.py`: all six steps ran with PyYAML 6.0.1 and jsonschema 4.20.0;
  **Errors: 0**. Step 6 applied 27 rules, from 26 promises; 33 sets under
  `examples/invalid-by-rule` were each rejected by the rule they are named for, and the 56
  files under `examples/invalid` by a schema.
- `python3 tools/check_docs.py`: one error, in another group's file:
  `docs/proposals/reaudit/round-6/README.md` links `log-tracker.md`, which does not exist yet.
  Noted, not fixed. Earlier in the pass the links to `round-6/DECISIONS.md` were missing too;
  they now resolve, as do the two links to this log.
- `python3 tools/set_names.py --check`: no error. Earlier in the pass it reported two, in
  `spec/interface/implementation.yaml`, another group's file, which that group has since fixed.
- `git diff --check`: clean. No line of a changed YAML file is longer than 96 characters.

## For a decision

None of the changes above needs one: each applies a decided detail of the task. For the
coordinator:

- **The wording of IFC-033 and IFC-012.** The interface editor changed both during this pass.
  The two READMEs, `template-controls.v1` and the example project's comment were then aligned
  with them: copies keyed by the qualified reference and the version, `allOf` of the
  contract's schema and the template's, no tool for a control that the target does not have.
  A later change to either promise should be followed there.
- **Whether to add a rule for SEC-005** to the tool, beside the new one for PRV-016 (choice 3).
