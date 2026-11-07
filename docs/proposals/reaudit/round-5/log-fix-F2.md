# Re-audit, round 5: log of the fix pass for area F2

Date: 2026-11-07. Area F2, manifests, schemas, templates and submissions, applies items M23 to
M31 of [DECISIONS.md](DECISIONS.md), and the changes of M03, M06, M17, M41 and M45 that fall
in its files: `spec/promises/templates.yaml`, `conduits.yaml`, `objects.yaml`,
`naming.yaml`, `packages.yaml`, `manifest.yaml`, `submissions.yaml` and `contracts.yaml`.
`instantiation.yaml` and `schemas.yaml` are in the area too, and no change fell in them, so
they were not edited. Nothing else was edited apart from this log, and nothing was committed
or staged.

Every changed promise keeps its ID, and none was moved. One promise was added, PKG-030, which
continues the prefix of `packages.yaml`. Each changed file gained a head-comment line for round
5 that names its changed or added promises. Text was rewrapped only around each change, at
no more than 96 characters a line.

## Applied

| Item | Files | Promise IDs | What changed |
| --- | --- | --- | --- |
| M03, M29 | `templates.yaml` | TPL-018 | `then`: the sentence on the run, in one text for both items, as [items that change one text](DECISIONS.md#items-that-change-one-text) says. It now reads "Where the hook is to be dispatched, its implementation is an item that is run and its runtime provider reports the run `stopped`, the engine first asks for the run, under PRV-007; where it reports the run's state `unknown`, the hook is not dispatched and its part reports unavailable." The observable is unchanged: its sentence under which the stop is asked for all the same covers the case. |
| M06 | `templates.yaml` | TPL-019 | `when` adds "or, where the template is of an Interface or a Pool, include one". `then` reads "an Agent or a Resource has at most one item that is run, and an Interface or a Pool none". In the observable, the diagnostic names "the two items, or the one item of an Interface or a Pool", and a last sentence says that an Interface or a Pool has no placement and no `runtime` (PLC-006, OBJ-013), so there is no machine on which to run an item for it (PRV-019) or to give in `machine` (PRV-007), and that its implementations are `engine` or reached by `remote`. |
| M17 | `contracts.yaml` | CTR-012 | Observable: "a result that is not a map", where it said "an object". |
| M17 | `manifest.yaml` | MAN-017 | M30's new text replaces the whole clause that said "admits an object or an array", so the word goes without a separate edit. See M30. |
| M23 | `manifest.yaml` | MAN-010 | Observable: after the sentence that ends "or `headers` beside anything but `remote`.", the decided sentence: a `url` that holds a variable in braces, such as `https://{tenant}.example.com/mcp`, is not a URI and so is rejected; no key binds a variable of a Remote, so the item writes its URL whole, and a Remote's `variables` are never used. |
| M24 | `objects.yaml` | OBJ-019 | `when`: the author's names inside a schema are those of a member of `properties`, `dependentRequired`, `dependentSchemas` or `$defs`, and a pattern that keys a member of `patternProperties`. Observable: "only the names and the patterns are the author's". |
| M25 | `packages.yaml` | PKG-030 (new) | Added after PKG-029, under a `# ---` line that names the item, as the file marks each batch of added promises. Layer `language`, level `must`. `when` and `then` are as decided. The observable's first sentence reads "Installation or resolving returns a diagnostic naming the item and the operation, as for a plain item under PKG-020", in place of "The diagnostic names the item and the operation, as PKG-020's does for a plain item": PKG-020 states no diagnostic of its own, and this form is CTR-012's. The rest of the observable is as decided. |
| M26 | `objects.yaml` | OBJ-021 | `then`: "a `$ref` in each of these standing for the schema it leads to, through any number of `$ref`s". Observable: before the sentence on a map with the keys `kind` and `name`, the decided example of the Schema contract `token.v1` whose `spec` is a `$ref` to `secret-reference.v1`, and of a `$ref` to a member of `$defs` that holds that `$ref`. |
| M27 | `submissions.yaml` | SUB-010 | `then`: "save that a program so resolved is still, under PRV-007, one given as a relative path". Observable: "Two things do not follow the resolved values", the digest and PRV-007's guard, in place of "The one thing". |
| M28 | `naming.yaml` | NAM-009 | `then`: the template also keeps each implementation item of its own submitted project that it selects, as its own registration gave it, with a program given as a relative path resolved under SUB-010. Observable: two submissions with one digest, read from two directories, give their templates the items as each directory resolved them. |
| M28 | `submissions.yaml` | SUB-008 | Observable: where two submissions share a digest, read from two directories, each template also keeps the items it selects as its own registration resolved them, under NAM-009. |
| M30 | `manifest.yaml` | MAN-017 | `when`: a pointer is rejected where the place's schema does not confine its value, by `type`, `enum` or `const`, to a string, a number, a boolean or null, in itself or in each branch of its `anyOf` or `oneOf`, a `$ref` standing for the schema it leads to; the exception for `secret-reference.v1` stays. Observable: a place whose schema is `{}`, or that `required` names and neither `properties` nor `additionalProperties` describes, is rejected too, since either admits a map or a list. |
| M31 | `submissions.yaml` | SUB-008 | `then`: "which is left out when the project has none" is struck. |
| M41 | `conduits.yaml` | CON-008 | `then`: a Conduit whose consumer is retired is closed once its shutdown is confirmed, "or once a request to unregister the consumer removes it, under LIF-017". Observable: the record of the closing may name the invocation of `unregister.v1` that removed the consumer, and a Conduit whose consumer's shutdown stays pending or unknown stays so until the consumer is also "declared lost or unregistered". |
| M45 | `conduits.yaml` | CON-022 | Observable: an implementation that is sent an argument for an input that its version of the requirement does not declare cannot apply the filter, under IFC-019, so the request reports unavailable and nothing is performed. |

## Head-comment lines

| File | Names |
| --- | --- |
| `templates.yaml` | TPL-018 (M03, M29), TPL-019 (M06) |
| `conduits.yaml` | CON-008 (M41), CON-022 (M45) |
| `objects.yaml` | OBJ-019 (M24), OBJ-021 (M26) |
| `naming.yaml` | NAM-009 (M28) |
| `packages.yaml` | PKG-030 added (M25) |
| `manifest.yaml` | MAN-010 (M23), MAN-017 (M17, M30) |
| `submissions.yaml` | SUB-008 (M28, M31), SUB-010 (M27) |
| `contracts.yaml` | CTR-012 (M17) |

## Left to other areas

The decisions also change these, outside the files of F2, and this pass did not touch them:
PRV-007, `run.v1`, `inspect-run.v1`, DL17 and the definitions under M03; SEC-024, the schemas
README and the tool's docstring under M23; the head of `spec/schemas/schema.yaml` and the
schemas README under M24; the tool's rule and the example for PKG-030 under M25; SEC-001 and
the definitions under M26; the tool and the example under M30; LIF-017 and the promises and
rows that limit unregistering under M41; and IFC-019 under M45.

## Checks

Run from the repository root after the edits:

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../CONSOLIDATION.md#validation) | 450 promises ok |
| Lines over 96 characters in the files of F2 | none |
| `python3 tools/check_docs.py` | 2 errors, neither in F2's files: `spec/core/README.md` line 118 and `spec/schemas/README.md` line 93 link to `log-fix-F5.md`, which area F5 had not yet written. This log's links resolve. |
| `python3 tools/set_names.py --check` | 0 errors |
| `python3 tools/check_schemas.py` | 0 errors; 28 examples under `examples/invalid-by-rule/`, since the examples of M25 and M30 belong to another area |
| `git diff --check` | clean |

With PKG-030, `packages.yaml` holds 22 promises. The layer `language` counted 202 when these
checks ran, not the 203 that the decisions give: it counted 202 at `HEAD`, PKG-030 adds one,
and M05, applied in another area's pass, moves SEC-024 to the layer `convention`. The
decisions' 203 leaves out M05, and the recount after the fix pass should give 202.
