# Re-audit, round 5: log of the fix pass for area F6

Applied 2026-11-07 under the [decisions of round 5](DECISIONS.md). The files of this pass are
`spec/definitions.md`, `spec/README.md`, `spec/schema-profile.md` and `docs/OVERVIEW.md`. No
promise file is among them, so no promise changed here and no head comment was added; the
history paragraph at the top of `spec/definitions.md` gains a sentence for round 5.

## What was applied

| Item | Files | Promise IDs | What changed |
| --- | --- | --- | --- |
| M69 | `spec/README.md` | DL11 (removed elsewhere) | The contents row for `implementation-defined.yaml` says 15 items, which the tree holds. Recounted with M102 and every count of the contents table against the tree: `packages.yaml` holds 22 promises, and the total is 450, of which 202 are `language`, 207 `convention` and 41 `interface`. The list of what has been applied names the decisions of round 5. `examples/invalid-by-rule` still holds 28 files in the tree, so its count stands until area F2 adds its two examples. |
| M87 | `spec/README.md` | PRV-003, PRV-010 to PRV-012, PRV-014, PRV-019, PRV-021, PRV-022, PRV-025, PRV-028 | Conformance: the first paragraph, the Engine row and the Implementation row as decided. |
| M87 | `docs/OVERVIEW.md` | PRV-025 | The paragraph on claims of conformance, as decided, rewrapped. |
| M88 | `spec/README.md` | SEC-013, SEC-014, SEC-023, PRV-026, TPL-018 | The rotation bullet's first sentence, as decided; the rest of the bullet kept. |
| M89 | `spec/README.md` | SEC-006, SEC-019 | The paragraph on layers, as decided. |
| M90 | `spec/definitions.md` | IFC-042, DL13, PLC-017, PLC-024, IFC-036, IFC-037, PRV-018, PRV-025, SEC-024, IFC-017, PRV-022, DL12 | Rows grant, declaration, policy, facts and machines provider as decided, with one change to grant: "a grant by which a client obtains a token from an authorization server, as a consumer does from the one that DL13 states", so that the row still holds if IFC-041 comes to name a grant of its own. Row origin: area F4 has already removed IFC-037's sentence "for its origin on HTTP" (M68), so the row names instead the sense that IFC-036 and DL12 now use, the `Origin` header. |
| M91 | `spec/definitions.md` | OBJ-024, PRV-022 | Row "names a version assigns", as decided. |
| M92 | `spec/definitions.md` | CTR-004, LIF-016, PRV-011, PRV-014, PRV-021, PRV-025, SEC-006, SEC-017, OBJ-018 | The sentence added to the row contract, and the new row after it. The new row's term also names "type contract", the words of LIF-016 and of the row shutdown, in place of the parenthesis "LIF-016, which writes type contract"; its last sentence, "The provider does what its contract states where a promise has it do so (PRV-021)", is left out, since a row says what a word means and the promises state the rule. |
| M93 | `spec/definitions.md` | NAM-001, PRV-018, PRV-022, IFC-042, DL13 | Row address qualified as decided. New row "the engine's address" after "implementation interface". It states PRV-022 and DL13 as M13 and M47 leave them, as the decisions record's section on items that change one text asks, and not as the batch's text had it: a value given as the run sees it keeps the engine's address whole, its scheme, host, port and path, and DL13 gives an `http` URL on loopback only to a run on the engine's own runner whose process ends when the engine stops, on an engine that can neither restart on another machine nor run as several parts. Both match PRV-022 and DL13 as areas F1 and F4 have now written them. |
| M94 | `spec/definitions.md` | IFC-012, IFC-017, IFC-022, OPS-027, PRV-018, PRV-025, SEC-013 | Row connection: the new opening and the sentence at its end, as decided. "To the item's URL" became "to the URL at which it reaches the item", since a run's server is reached at the run's `url`, not at a URL of the item. |
| M95 | `spec/definitions.md` | SUB-006, SUB-009, LIF-009, PLC-026, FST-001 | Row register, registration, as decided; "runs resolving on a submission" became "on the submission". |
| M96 | `spec/definitions.md` | PRV-014, PLC-026 | Row label, as decided. |
| M97 | `spec/definitions.md` | DL13, PRV-022, PRV-025 | Row ID ends as decided; new row "the engine's credential ID" after it; rows gate, `identity` and credential renamed. The new row says "which the mechanism gives, and which only a credential that it issues to the engine names", since the mechanism issues a credential and not an ID. |
| M98 | `spec/definitions.md` | IFC-006, PRV-005, TPL-018 | Rows configuration, provider call and hook, as decided. |
| M99 | `spec/definitions.md` | LIF-004, LIF-005, CON-011, PLC-017, PLC-024 | Row status, as decided. |
| M103 | `docs/OVERVIEW.md` | see the next column | The note gives 2026-11-07 and names round 5. As the item asks, the overview follows this round's decisions where they change what it describes: placement on `local` of an instance with no program to run (M22); unregistering a retired instance whose shutdown is not confirmed (M41); a repeat answered from the outputs only where its filters were sent (M32, M39); a provider's server that stops with the part of the engine that runs it (M09); no provider reached at a URL (M25, PKG-030); a store at a URL and a loopback host on an engine that can move (M01, M02, SEC-024); values lost when the part that holds a connection stops (M10); and the gate, which no plain server reached at a URL stands behind (M01). |
| M104 | `spec/README.md` | none | The three sentences of the opening, as decided, rewrapped with each link kept on one line. |
| M107 | `spec/README.md` | none | Both changes, as decided, rewrapped. |

## What was not applied here

The batch also carried these changes, to files outside this pass. They were not made, since
this pass edits only its four files; the area that holds each file is to make them.

| Item | File | Change not made |
| --- | --- | --- |
| M01, M02 | `spec/schemas/README.md` | The row for SEC-024 in "What a schema cannot express". |
| M06 | `spec/schemas/README.md` | The row for TPL-019. |
| M23 | `spec/schemas/manifest.yaml`, `spec/schemas/README.md`, `tools/check_schemas.py` | The description of `server-remote.v1`, the clause of the SEC-024 row, and the docstring of `secret_may_go`. |
| M24 | `spec/schemas/schema.yaml`, `spec/schemas/README.md` | The OBJ-019 entry of the head comment, and the row on a schema in "Closed and open". |
| M25 | `spec/schemas/manifest.yaml`, `spec/schemas/README.md`, `tools/check_schemas.py`, `spec/schemas/examples/invalid-by-rule/PKG-030-manifest-remote-runtime-provider.yaml` | The head comment, the README row and counts, the rule `remote-provider`, and the new example. |
| M30 | `tools/check_schemas.py`, `spec/schemas/examples/invalid-by-rule/MAN-017-environment-place-admits-any-value.yaml`, `spec/schemas/README.md` | The rule `confined`, the new example, and the MAN-017 row. |

The decisions record also has changes to this pass's files that no item of the batch
carries, so they were not made: the limit of a plain store in `spec/README.md`, whose
sentences "Every server that the engine does not run itself ... stands behind a gate" and "Only
an enforcement point reaches the server of a secret Resource that the engine does not run
itself" are still false for a plain `remote` store (M01); a part that stops counting as a
restart, in the definitions and the README (M09); a provider that could make a restriction
enforced, in the README (M16); the rows that cite TPL-019 for one run per instance (M03); the
row reference schema and `$ref` chains (M26); the rows on a control that ends after its
instance was declared lost (M35); the rows retired, remove, unregister and run, which still
limit unregistering to a node declared lost (M41); the row attempt (M42); and the row
implementation item, whose test at a bound place and whose list of SEC-024's rejections M30,
M01 and M02 change. M91's change to OBJ-024 and M97's renames in IFC-041, DL13 and the core
README are in other areas' files; until they are made, the rows of M91 and M97 are ahead of
those texts.

## Checks

Run on the working tree with the other areas' changes in it, at the end of this pass:

- The validation of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): 450 promises ok.
- `python3 tools/check_docs.py`: 0 errors.
- `python3 tools/set_names.py --check`: 0 errors.
- `python3 tools/check_schemas.py`: 0 errors.
- `git diff --check`: no output.

The counts of `spec/README.md` were taken from the tree as it stood; the recount by script
after the whole fix pass, which the decisions record calls for, still applies.
