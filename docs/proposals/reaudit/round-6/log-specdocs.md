# Round 6: log of the docs editor

Date: 2026-11-07. This is the account of the docs editor's pass under
[ruling 44](../../ruling-44/README.md), which settled [round 6](README.md) of the re-audit as the
[decisions](DECISIONS.md) record. The ruling and the decisions are the owner's and the
coordinator's. The edits below are the docs editor's, and each stands unless the coordinator or
the owner strikes it. Nothing was committed.

The files of this pass: `spec/definitions.md`, `spec/README.md`, `spec/schema-profile.md` and
`docs/OVERVIEW.md`, and this log. The promise files were changed by the other groups at the same
time (logs `log-core.md`, `log-promises.md`, `log-conventions.md` and `log-interface.md` beside
this one); this pass read their text as it stood when it ended and wrote the four files to it.
D06 and D12 are not applied, as the coordinator directed: TPL-003, TPL-004 (but its sentence on
a hook's default), CTR-002's `implementation` and LIF-031 were left alone, and so were the rows
that state them (selection, ID, resolved form, reference).

## What changed

| File | Change | Promises | Finding or critique | Why |
| --- | --- | --- | --- | --- |
| `spec/README.md`, head | The status and the draft condition of ruling 38 first, then one paragraph that says the page describes the specification as it stands and that its history is at the end, naming ruling 44 as the latest change | — | D21 | The entry point put three paragraphs of history before the design |
| `spec/README.md`, new section Reading paths | Three paths: an author of documents, a writer of contracts and providers, an engine builder. The engine builder's path is the former Reading order, unchanged, with a pointer to the limits; the section Reading order is gone | — | D21 | Ruling 44 accepted reading paths for the three readers |
| `spec/README.md`, Contents | Interface 44 promises (IFC-047 added); `invalid-by-rule` 33 files (three examples added by the other groups); the paragraph after the table says 453 promises, 202, 207 and 44, and points to History for what was applied | IFC-047 | 6-1-01, 6-1-03, 6-1-05 | Counts taken from the working tree by the validation script and `ls` |
| `spec/README.md`, Conformance | An engine never accepts a document and ignores part of it unsaid: where a promise leaves part of an accepted document unused, as TPL-004 does a type's default for a hook it does not require, a diagnostic that does not reject names it. IFC-047 joins the examples of the Implementation level | TPL-004, IFC-047 | Open point on the hook default; 6-1-01 | The old sentence, "never accepts a document and ignores part of it", contradicted TPL-004 as now written; the verification of the status (verify:status) flagged it |
| `spec/README.md`, new section Limits of this version, with What access does not narrow | A bound never limits filters, with the example of a filtered read and a right to open that covers it; a bound that names a Pool covers its later Resources; closing a delegator's Conduit to `access` closes nothing it opened; access to `management` cannot be narrowed to one template nor below whole operations; the broker, its trusted operation, why the actor's Conduit to it applies no filter, and that S1 and S2 measure it | ACC-009, ACC-015 to ACC-018, ACC-029, ACC-030, ACC-033, CON-008, CON-022, CON-028, MAN-012 | D03, D04 | Ruling 44, point 2, option a: v0 says these limits plainly in `spec/README.md` |
| `spec/README.md`, What the guarantee for secrets covers | Made a subsection of Limits of this version; its text and anchor are unchanged | — | D21 | One section on limits, as the work order names it |
| `spec/README.md`, new section History, at the end | The three paragraphs of history from the head and the list of rulings applied, moved unchanged; round 6's decisions added to the records; ruling 44 added to the list; one paragraph on what ruling 44 changed, and that D06 and D12, as revised, are not yet applied (A69) | IFC-007, IFC-010, IFC-033, IFC-045, IFC-047, OPS-007, OPS-009, OPS-024, OPS-034, ACC-008, ACC-029, ACC-030, ACC-033, CTR-006, CTR-012, PRV-016, SUB-009, TPL-004 | All six findings; D03, D04, D07, D11, D14, D18, D21; the hook default | The head comment for ruling 44, and D21's "current design before history" |
| `spec/definitions.md`, head | A short introduction: what the file is, that the first table gives the core terms, that the history is at the end, and that ruling 44 is the latest pass | — | D21 | The head comment for ruling 44 |
| `spec/definitions.md`, new section Core terms | 32 rows, one line each, every one linking the section that holds the full row | — | D21 | Ruling 44 accepted a compact glossary that links the longer rows |
| `spec/definitions.md`, new section How this file changed, at the end | The history paragraphs of the head, moved unchanged, and a paragraph that names the rows ruling 44 added and changed | — | D21 | See the choices below |
| `spec/definitions.md`, row project | A project is also handed over, with the one template that replaces another, in a request to replace a template, which resolves it as a registration does | SUB-005, SUB-009 | D11 | SUB-009 now takes the project |
| `spec/definitions.md`, row register, registration | `replace.v1` takes a document with the contracts and the manifest of its project, resolves it as a registration does, and registers nothing | SUB-009 | D11 | As above |
| `spec/definitions.md`, row replace, replacement | The document is handed over with its project in `contracts` and `manifest`; `#name` reaches that project and never the one held before; SUB-008's digest, the address rules; when a replace changes nothing; one template, no apply of several | SUB-009, NAM-022, PKG-029, INS-020 | D11 | The row said that a manifest may hold `packages` alone, which SUB-009 no longer says |
| `spec/definitions.md`, row the contracts of a submitted project | The same holds of the project handed over with a replacing template, and a template replaced with them keeps the address, version and digest | SUB-009, NAM-009 | D11 | As above |
| `spec/definitions.md`, row `management` | ACC-033 gives its targets as one decision list; a Conduit to `management` carries no bound and can apply no filter, so its access cannot be narrowed to one template; a broker gives narrower access | ACC-030, ACC-033, CON-022, CON-028 | D04, D18 | — |
| `spec/definitions.md`, row bound | A right to open trusts its consumer with unfiltered access whatever filters apply to the Conduits it already has; link to the README's limits | ACC-009, ACC-030 | D03 | — |
| `spec/definitions.md`, new row broker | A Resource with a trusted operation that gives an actor narrower access to `management` than a Conduit carries; the onward request through the broker's own Conduit; no filter on the actor's Conduit to it; the narrowing is the broker's; a pattern, of no family or kind of its own | ACC-015 to ACC-017, ACC-030, ACC-033 | D03, D04 | ACC-030's observable now uses the word |
| `spec/definitions.md`, row filter | It judges a call, so a repeat that is not dispatched is judged by none and gives only an outcome already recorded, and no repeat to which one applies is given outputs it has not judged; it limits no other Conduit of its consumer and none the consumer may open | ACC-008, ACC-009, ACC-030, OPS-009, OPS-024, OPS-034 | 6-1-02, D03 | — |
| `spec/definitions.md`, row narrow | The tool presented for a control shows the template's narrowing | IFC-033 | D07 | — |
| `spec/definitions.md`, row operation | The root of each schema holds `type: object` and may use any keyword of the profile, but a control's | CTR-012 | D14 | — |
| `spec/definitions.md`, row setup hook, retire hook, hook | A type's default for a hook it does not require is not used, and the registration of a template that selects none for it reports the default in a diagnostic that does not reject | TPL-004 | Open point on the hook default | — |
| `spec/definitions.md`, row repeat | The four cases where the invocation succeeded: no filter (outputs, or conflict), and with a filter, an operation with no effect, not a control and starting no model work (new attempt), any other with no outputs (outcome alone), any other with outputs (conflict); a repeat not dispatched is judged by no filter | OPS-009, OPS-024, OPS-034, ACC-008, OPS-032 | 6-1-02 | The row described the earlier test of which filters were sent, which OPS-034 no longer makes |
| `spec/definitions.md`, row attempt, new attempt | A new attempt is also made after success where a filter applies and the operation has no effect, is not a control and starts no model work; an answer with a reason leaves the outcome and the outputs as they were | OPS-024, IMP-007 | 6-1-02 | The row said a new attempt follows only failed or unknown |
| `spec/definitions.md`, How a request ends | The conflict row and the paragraph after the table: conflict for a repeat of a succeeded invocation of an operation that declares outputs, where they are not kept, or a filter applies and OPS-024 does not dispatch it again | OPS-034, OPS-024 | 6-1-02 | As above |
| `spec/definitions.md`, row plain | Its errors are failed for an operation with no declared effect and for invalid parameters, and otherwise unknown; -32021 for an undeclared capability is read by that table like any other | IFC-010, OPS-007 | 6-1-01 | — |
| `spec/definitions.md`, row Conduit's extension | An implementation that carries Conduit's keys answers -32021 for an undeclared capability only where it has performed nothing, so the outcome is failed; a plain server's -32021 is any other error | IFC-047, OPS-007, IFC-010, IFC-045 | 6-1-01 | — |
| `spec/definitions.md`, row tool | Its schemas are bundles and, for a control, an input that the template narrows is the `allOf` of the two; no tool for a control the target does not have; a call that names such a tool reports invalid | IFC-033, IFC-012, TPL-013 | 6-1-04, D07 | — |
| `spec/definitions.md`, new row bundle (of a tool's schema) | One copy of each Schema contract reached under the root `$defs`, keyed by qualified reference with `@` and version; rewritten `$ref`s; sibling keywords kept; self-reference; optional inlining; not governed by the profile or CTR-012; a trust bundle is something else | IFC-033, SCH-001, SCH-015 | 6-1-04 | The work order named the row; IFC-033 does not use the word, so the row says what it means and that it is not a trust bundle |
| `spec/definitions.md`, row control | No two pointers of `changes` equal or nested; the tool shows the narrowing and is absent for a control the target lacks; a control's schema root; a secret Resource's type lists none, a package-source provider's none that reaches `schemes` | CTR-006, IFC-033, IFC-012, CTR-012, SEC-005, PRV-016 | 6-1-03, 6-1-05, D07, D14 | — |
| `spec/definitions.md`, row package-source provider | Its type lists no control whose `changes` holds the empty pointer, `/schemes` or a pointer within it, so its schemes are fixed for its life; another provider is registered once none claims them | PRV-016, PRV-013 | 6-1-05 | — |
| `spec/definitions.md`, row configuration | No control changes a locator, or the `schemes` of a package-source provider | SEC-005, PRV-016 | 6-1-05 | The row said that controls change the configuration afterwards |
| `spec/schema-profile.md`, head | Names ruling 44 and what it changed on this page | — | — | The head comment for ruling 44 |
| `spec/schema-profile.md`, row Pointers into values | No two pointers of a control's `changes` overlap, as no two destinations do; two pointers are compared by unescaped reference tokens, the empty pointer over all, `/a~1b` not `/a/b` | CTR-006, PRV-016, INS-010 | 6-1-03 | — |
| `spec/schema-profile.md`, row The implementation interface | What the protocol's tool definition allows at revision 2026-07-28 (an `inputSchema` holds `type: object` and any keyword beside it, an `outputSchema` any schema, and the advice on composition at the root); an Operation contract may use the whole profile at either root, keeping `type: object` because the outputs are the members of `structuredContent`; a control's root keeps four keys | CTR-012, IFC-001 | D14 | Checked against the 2026-07-28 schema reference: `inputSchema: {$schema?; type: "object"; [key: string]: unknown}`, `outputSchema?: {$schema?; [key: string]: unknown}`, "This can be any valid JSON Schema 2020-12" |
| `spec/schema-profile.md`, Schemas this profile does not govern | The schemas of a tool that the engine presents: neither the profile nor CTR-012 governs them; what the bundle holds, its keys, its pointers (`~0`, `~1`, `%23`), sibling keywords, self-reference, optional inlining, the relation to JSON Schema 2020-12's bundling, and a control's narrowed input | IFC-033, TPL-010 | 6-1-04, D07 | IFC-033 says the profile and CTR-012 govern the contract and not the definition |
| `docs/OVERVIEW.md`, head | Last read as ruling 44 left it | — | — | The record |
| `docs/OVERVIEW.md`, Documents, templates and instances | A template is changed only by `replace.v1`, which hands over the new document with its project's contracts and manifest, resolved as a registration; one template, nothing registered | SUB-009, INS-020 | D11 | — |
| `docs/OVERVIEW.md`, Conduits and access | The two limits, with the filtered-read example and the broker | ACC-009, ACC-030 | D03, D04 | — |
| `docs/OVERVIEW.md`, Operations, outcomes and reasons | Repeats where a filter applies; the -32021 of a plain server is no proof that nothing was done, and an implementation that carries Conduit's keys answers it only where it did nothing; a control's pointers, its narrowed tool and no tool for a control left out | OPS-009, OPS-024, OPS-034, IFC-010, IFC-047, OPS-007, CTR-006, IFC-033, IFC-012 | 6-1-01, 6-1-02, 6-1-03, D07 | The paragraph on repeats described the earlier test |
| `docs/OVERVIEW.md`, Packages, contracts and the core package | The root of an Operation's schemas may use the profile, a control's not; a tool's schema holds a copy of every schema it refers to | CTR-012, IFC-033 | D14, 6-1-04 | — |
| `docs/OVERVIEW.md`, Providers | A package-source provider's schemes are fixed for its life | PRV-016 | 6-1-05 | — |

## Choices beyond the record

1. **The history of `spec/definitions.md` moved to its end.** D21 and the work order ask for the
   current design before history in `spec/README.md` and a table of core terms at the top of
   `definitions.md`. Ninety lines of history stood at the top of `definitions.md`; the table
   could not be at the top with them there, so they moved, unchanged, to a new last section,
   How this file changed, and the head says where they are. No anchor changed.
2. **The core terms link sections, not rows.** A Markdown table row has no anchor, and no file in
   the repository uses HTML anchors, so each line links the section that holds its full row.
   Six lines name pairs (Interface and Agent; template and instance; outcome and reason;
   idempotency key and repeat; set up and retire; `management` and `access`) where one line
   says both.
3. **One section on limits in `spec/README.md`.** D03 and D04's statement is a new subsection,
   What access does not narrow, beside What the guarantee for secrets covers, which became a
   subsection of the same section, Limits of this version. The anchor of the secrets section is
   unchanged.
4. **The Conformance sentence was changed** to fit TPL-004's hook default, which the record did
   not name. Without it the README said that no part of an accepted document is ignored.
5. **The engine builder's path is the former Reading order, word for word,** so no promise or
   file moved between the paths of a reader of the Language and the Engine levels.
6. **The row is named "bundle (of a tool's schema)"**, because "trust bundle" has its own row and
   CTR-005 uses "a bundle of limits" in its ordinary sense.
7. **The schema profile states pointer comparison for CTR-006 and PRV-016, and cites INS-010
   only for the rule that destinations do not overlap.** INS-010 does not say how it compares
   two destinations; whether it compares by reference tokens too is not settled here (see
   below).
8. **The README's history says that D06 and D12, as revised, are not yet applied, naming A69.**
   The sentence must change when A69 applies them.
9. **IFC-047 was added to the examples of the Implementation level** under Conformance, beside
   IFC-008, since it binds an implementation and not the engine.

## For the coordinator

- **OPS-005 and IFC-012 disagree on one case.** IFC-012 now has a call to the tool that a Conduit
  would give a control its target does not have report invalid, under TPL-013. OPS-005's
  observable still says of every call that names a tool the engine does not present that "its
  reason, refused, is in the protocol's error". `conventions/operations.yaml` is another group's
  file; this pass did not change it.
- **INS-010 does not say how two destinations are compared.** CTR-006 and PRV-016 now compare
  pointers by unescaped reference tokens. Stating the same of INS-010 would make the three
  agree; it is a one-sentence change to `promises/instantiation.yaml`, which was not in this
  pass.
- **A69.** When D06 and D12 are applied, this pass's files need: the README's history sentence on
  them; the rows selection and hook (D06), ID, reference and resolved form (D12) of
  `definitions.md`; and, for D12, the sentence on the resolved form in the overview's section
  Documents, templates and instances.
- **The keys of a tool's bundle.** IFC-033 writes `#` as `%23` in a `$ref` to a copy, as a URI
  fragment needs. `schema.v1`'s pattern for `$ref` would accept that form too, but IFC-033 says
  the profile does not govern the definition, so nothing checks it; a vector should.

## Checks

At the end of this pass, on the working tree with every group's changes:

- The validation of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): 453 promises, every one
  with exactly the seven fields, and no ID twice.
- `python3 tools/check_docs.py`: 187 Markdown files and 2,614 local links, no error. An earlier
  run during the pass found links to `DECISIONS.md`, `log-core.md` and `log-tracker.md` that
  other groups had not yet written; they had all been written by the last run.
- `python3 tools/set_names.py --check`: no error.
- `python3 tools/check_schemas.py`: all six steps, no error; 33 sets under `invalid-by-rule/`
  each rejected by the rule it is named for.
- `git diff --check`: no error.

No line of the four files is over 96 characters but table rows, a link that cannot be broken and
lines that were already longer.
