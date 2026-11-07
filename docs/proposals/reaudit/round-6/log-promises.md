# Round 6: log of the promises editor

Date: 2026-11-07. This pass applies [ruling 44](../../ruling-44/README.md) to the files under
`spec/promises/`, for the parts given to it: finding 03 (CTR-006) and finding 05 (PRV-016) of
the [findings](reader-1.md), the simplifications D11 (SUB-009) and D14 (CTR-012) of the
[design critique](design-critique.md), and the first of its four open points, a Type's default
for a hook that it does not require. It works from the independent check that the coordinator
ran before applying the ruling, whose verdicts on findings 03 and 05 and on the simplifications
D11 and D14 hold, and from the choices the coordinator fixed for this pass.

Nothing outside `spec/promises/` was edited apart from this log, and nothing was committed or
staged. Every changed promise keeps its ID, none was moved, none was added and none was
removed. Each changed file gained a head-comment line for ruling 44 and round 6 that names its
changed promises. Every changed field is folded with `>-`, and no line is over 96 characters.
Text was rewrapped only inside the fields that changed; TPL-004 keeps its lines as they were
and gains one passage at the end of its observable.

D06 and D12 were not applied, as the coordinator directed, and TPL-003, TPL-004 (but for the
sentence on the hook's default), CTR-002 and LIF-031 were not touched. The record of ruling 44
now carries the owner's answer on the revised D06 and D12 ("Agree on both"). Applying them is
for a later pass; the sentence added to TPL-004 speaks only of a type's default, and stands
under the revised D06 as it does now.

## Applied

| What | Promise IDs | Finding or critique | Why |
| --- | --- | --- | --- |
| `then` adds "No two pointers of its `changes` are equal, and none lies inside another." The observable rejects such a control when its package is installed or its project is submitted, with a diagnostic naming both inputs and both pointers, as INS-010 names two destinations that overlap, and gives `{a: /limit, b: /limit}` and `{o: /options, a: /options/limit}` as rejected. It says how two pointers are compared: by their reference tokens, as RFC 6901 splits a pointer, each with `~1` and `~0` unescaped, so `/limit` and `/limits` do not overlap, `/a~1b` is the one member `a/b` and does not lie inside `/a`, and the empty pointer, the whole configuration, lies over every other. | CTR-006 | 6-1-03 | Two inputs that set one place, or a place and a place inside it, give REV-006 and REV-007 two answers for one setting, and `changes` is a map with no order (SCH-007), so no order of writing can settle them. The language already rejects the same overlap for a template's destinations (INS-010). The check found the empty pointer, which the finding left out; the coordinator fixed the comparison by tokens. REV-006, REV-007 and REV-015 need no change once no two pointers overlap. |
| `then`: a package-source provider's Type lists no control whose `changes` holds the pointer `/schemes`, a pointer inside it or the empty pointer, and a Type that lists one is rejected when its package is installed or its project is submitted, as one whose `schema` does not require `schemes` is. Observable: the diagnostic also names the control; pointers are compared by their reference tokens, as CTR-006 compares them (`/schemes/0` lies inside `/schemes`, `/schemesExtra` does not, the empty pointer lies over it); with no such control nothing changes the schemes that a provider claims, for its whole life, as Kubernetes makes the `handler` of a RuntimeClass immutable, so PRV-013 decides the claim once and no control that has not ended, or whose outcome is unknown, can later make two providers claim one scheme; a scheme is added by registering another provider and moved by retiring the one that claims it and registering one that does. | PRV-016 | 6-1-05 | The coordinator chose to fix the claim for the provider's life, as SEC-005 fixes a secret Resource's locator, in place of a procedure that would reserve the schemes of controls not yet ended or of unknown outcome. The empty pointer and pointers inside `/schemes` are covered, as the check asked, so a control on the whole configuration cannot get round the rule. |
| Observable: the schemes that a package-source provider claims never change, under PRV-016, so the check made when it is instantiated keeps one provider to a scheme at any time. | PRV-013 | 6-1-05 | The finding's point was that PRV-013's "at any time" was checked only at instantiation. One sentence ties the two together. PRV-017 needed no change. |
| `when`: the root of either schema of any Operation contract still needs `type: object`; only a control's contract, whose `spec` holds `changes`, is still held to `type`, `properties`, `required` and `additionalProperties`. Observable: the root of any other Operation contract may hold any keyword of the profile beside `type: object`, as the protocol's tool definition lets `inputSchema` hold any keyword of JSON Schema 2020-12 beside `type` and lets `outputSchema` be any schema, with the example `oneOf: [{required: [path]}, {required: [uri]}]`; the inputs and the outputs are still the members of the root's `properties`, and a keyword beside them adds none; a `default` never stands inside a keyword that SCH-008 names, at the root or below; a control keeps the four keys because every input but `revision` may be left out (REV-006), and `oneOf` at its root would make one required all the same. `type: object` stays at the root of `outputSchema` for IFC-001's reason, which the observable already gave. | CTR-012 | D14 | Ruling 44 opens the root to the whole profile for an ordinary Operation and keeps it closed for a control. The published revision 2026-07-28 was read for both schemas: `inputSchema` is `{$schema?, type: "object", [key]: unknown}`, and "any JSON Schema 2020-12 keyword may appear alongside `type`"; `outputSchema` "can be any valid JSON Schema 2020-12". Conduit keeps `type: object` on `outputSchema`, which the protocol allows, because the outputs are the members of `structuredContent` by name. The `when` uses the test that the definitions give for a control, a `spec` that holds `changes`, which is also the test that the core group's schema and examples use. |
| Observable, at its end: `setup-hook.v1` and `retire-hook.v1` are in no capability, so a type's default for either is used only where the type lists that hook under `requires`; a default that a type states for a hook it does not require is not used; a template of that type that selects no implementation for the hook has none, so the hook is not dispatched (TPL-017, TPL-018), and the template is registered with a diagnostic that reports that default and did not prevent registration. | TPL-004 | The open point on a Type's default for a hook that is not required | The open point, as ruling 44 accepts the critique's recommendation: ignored, with a diagnostic. The current rules already ignore such a default (TPL-004's `when` does not reach it, and TPL-017 and TPL-018 take a type's default only where the type requires the hook); the new part is the diagnostic. |
| Rewritten. `replace.v1` gives, with the replacing template, the project it belongs to: its contract documents, as the input `contracts`, and its manifest, as the input `manifest`. The engine resolves the template against that project as a registration resolves one against its own: the project's contracts and implementations are made available as SUB-008 states, with SUB-008's digest; the rules on the manifest of a submitted project apply (MAN-015, MAN-016, PKG-006, and MAN-006 and MAN-010 for a manifest with no directory); and the template as replaced keeps what it resolved through as NAM-009 states. `#name` reaches the project handed over and never the one the template held before. A replace that changes nothing is one whose document is equal and through which every package, and the project where the document refers to it, has the address, the version and the digest the template keeps, and each item selected is the one it keeps. Instances made before keep what they resolved (INS-020, PKG-005). The request replaces one template and registers nothing, and never replaces several together: `contracts` holds contract documents alone. The rule that a manifest handed to `replace.v1` holds `packages` alone is gone. | SUB-009 | D11 | Ruling 44: replacing a template resolves it against the project submitted with it, as registration does, under the existing check of revisions, and instances made before keep what they resolved. The coordinator fixed that SUB-008's digest and the address rules of PKG-029 and MAN-016 are reused exactly, and that this is not an apply of several templates. The input is named `contracts`, items `contract.v1`, as the core group gave it in `spec/core/operations.yaml`. |
| Observable: in the document that replaces a template, `#name` reaches a contract or an implementation of the project handed over with that document, as SUB-009 states, and never one of the project that the template held before. | NAM-022 | D11 | The old sentence said the opposite, that `#name` reaches the project the template held, and would have contradicted SUB-009. NAM-022 is not among the promises the coordinator named for D11, but it is in this pass's files and could not be left contradicting SUB-009. |
| `when`: the address may be that of a submitted project or of the project that a request to replace a template hands over under SUB-009, while a template or an instance holds "that project's" contracts or implementations, where it said "that submission's". | PKG-029 | D11 | A project handed over with a replace is not "handed over for registration", which is how the definitions name a submitted project. Without this, a package could be installed at the address of a project that a replaced template holds, which is the clash PKG-029 and MAN-016 exist to prevent. MAN-015 and MAN-016 need no change, because SUB-009 applies the rules on a submitted project's manifest to the manifest of a replace. SUB-008 and NAM-009 needed no change: SUB-009 names each as the rule it follows. |

## Choices beyond the record

1. **The hook's diagnostic is reported where a template is registered (TPL-004), not where the
   Type is installed (CTR-004).** Both were open to this pass. The definitions give a
   diagnostic that does not reject only at registration ("is reported and does not prevent
   registration", OBJ-008, NAM-008), and a diagnostic at a package's installation that does
   not reject is not defined. At registration it also reaches the author for whom it matters,
   the one whose template runs no hook. It is reported only where the template selects no
   implementation for the hook: where the template selects one, the type's default is beside
   the point. The alternative, a diagnostic at installation or submission under CTR-004, would
   tell the author of the Type once, and needs the row "diagnostic" of the definitions to say
   that such a diagnostic does not prevent an installation either.
2. **The inputs and outputs of an Operation are still the members of the root's `properties`
   alone (CTR-012).** With the whole profile at the root, a `$ref` or a branch of `oneOf` at the
   root could declare `properties` of its own. CTR-002 says that the properties of
   `inputSchema` are the inputs, and the rules that name an input by its name, such as a handle
   under CTR-002 and the default of an input left out under OPS-026, take the inputs as CTR-002
   gives them. So a keyword beside the root's `properties` constrains the arguments as a whole
   and adds no input. The protocol advises the same: "When using composition keywords,
   declare `properties` at the schema root and keep the branches to presence assertions such
   as `required`." The rule is stated, not enforced: a branch that declares `properties` is not
   rejected, in keeping with "the whole profile". Rejecting it would narrow the ruling.
3. **A replace hands its manifest over as a value, so a relative `fetch` or program is rejected
   (SUB-009).** This follows from MAN-006 and MAN-010 as they stand, and matches `register.v1`
   and the core group's description of `replace.v1`. No engine is obliged to take a directory
   for a replace, as SUB-010 obliges it to for a registration. See the first decision below.
4. **The equality that makes a replace change nothing is stated through NAM-009 (SUB-009).** A
   replace changes nothing where its document is equal and the template would keep, under
   NAM-009, exactly what it keeps: the address, version and digest of each package and of the
   project where the document refers to it, and each implementation item it selects. This
   extends the old test, which named packages alone, to the project that a replace now hands
   over. A project whose digest differs is a change only where the document refers to that
   project.
5. **The precedent for fixed schemes (PRV-016) is Kubernetes's RuntimeClass, whose `handler`
   "is immutable".** The check also cited CSIDriver. RuntimeClass was chosen because its
   immutable field is the routing key itself; the check noted that RuntimeClass does not make
   that key unique, which PRV-013 does.
6. **CTR-006 cites RFC 6901 for "reference token".** The term is the RFC's, and RFC 6901 is
   already the standard named for pointers in [the schema profile](../../../../spec/schema-profile.md).
   The definitions have no row for it.

## For a decision

1. **A replace from a directory.** In D11's own scenario a developer edits a contract, an
   implementation item and a template of a local project together, and that project's items
   often name a program by a relative path. A replace takes values, so such an item is rejected
   and the developer must write the path whole. Making every engine take a directory for a
   replace, as SUB-010 makes it take one for a registration, would remove that, at the cost of
   a new obligation and a line of DL7. This pass did not add it. Recommended: decide it with the
   S1 measurement of the edit cycle that ruling 44 asks for.
2. **A diagnostic that does not reject, at installation.** Only if the coordinator prefers the
   hook's diagnostic under CTR-004 (choice 1): the row "diagnostic" of the definitions would
   need to allow one at installation.

## For the other groups

- **Definitions** (`spec/definitions.md`), rows to bring in line: "project" and "submitted
  project", which name only a project handed over for registration, where SUB-009 now hands one
  over with a replace; "replace, replacement", which still says that a manifest handed over
  with the document "may hold `packages` alone"; "package-source provider", which may say that
  its claim is fixed for its life (PRV-016); and "control", which may say that no two pointers
  of its `changes` overlap (CTR-006).
- **Schemas and checker** (`spec/schemas/README.md`): the row for SUB-009's manifest that holds
  `packages` alone (the rule is gone); the rows for CTR-006 and PRV-016, now that the checker
  applies changes-overlap and fixed-schemes; and the comment on `replace.v1`'s manifest in
  `spec/schemas/manifest.yaml`. The checker's comparison by tokens, with the empty pointer
  over all, matches CTR-006 and PRV-016 as written.
- **Core** (`spec/core/operations.yaml`): the description of `replace.v1` agrees with SUB-009.
  It names the input `contracts`, which SUB-009 uses.

## Validation

Run on 2026-11-07, with the files of the other groups as they stood then.

- The command of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): 453 promises ok. The count
  was 452 at `c4d7d33`; the promise added is in another group's file, since this pass added
  none.
- No line of `spec/promises/*.yaml` is over 96 characters, and `git diff --check` reports
  nothing.
- `python3 tools/check_schemas.py`: Errors: 0. Its examples of CTR-006 and PRV-016, which the
  core group added, are each rejected by the rule they are named for.
- `python3 tools/check_docs.py`: 13 errors, none in a file of this pass. Each is a link to
  `round-6/DECISIONS.md` or `round-6/log-core.md`, which other groups have not yet written.
- `python3 tools/set_names.py --check`: 2 errors, in `spec/interface/implementation.yaml` at
  lines 1066 and 1067, where `example.org/files` is read as a `_meta` key. That file is another
  group's.
