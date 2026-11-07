# Re-audit, round 5: log of the fix pass for area F5

Date: 2026-11-07. Area F5, the core package, the examples and the names, applies items M79 to
M86 of [DECISIONS.md](DECISIONS.md), and the changes of M03, M09, M18, M20, M35, M37 and M41
that fall in its files: `spec/core/operations.yaml`, `spec/core/capabilities.yaml`,
`spec/core/README.md`, `spec/schemas/README.md`,
`spec/schemas/examples/valid/project/conduit.yaml`, `spec/names.yaml`,
`tools/check_schemas.py` and `tools/set_names.py`. Nothing else was edited apart from this
log, and nothing was committed or staged.

No promise is in these files, so no promise ID was changed, added or moved; the column
"Promises" names those that each change states. No export was added or removed. Each changed
YAML file and each changed tool gained a head-comment line for round 5, and each changed
README a history sentence. Folded text was rewrapped only from each change until the old line
breaks resumed, at no more than 96 characters a line.

## Applied

| Item | Files | Promises | What changed |
| --- | --- | --- | --- |
| M03 | `operations.yaml` | PRV-007, PRV-014, TPL-018 | `run.v1` description: after "so one run serves one instance.", "A provider starts no run for an instance's ID while a run that it started for that ID may still be running." `inspect-run.v1` description: after "`running`, `stopped` or `unknown`.", "It reports `stopped` only where it knows that no run that it started for the ID is running, and `unknown` where it cannot tell, as where it cannot reach the run's machine; the engine asks for a run only where the state is `stopped`." As decided. |
| M09 | `operations.yaml` | PRV-026, ACC-019, DL17 | `get-secret.v1` description: "once the engine has restarted or the part of it that held the run has stopped, its reads before it runs again a provider whose run ended with it", in place of "once the engine has restarted, its reads before it runs again a provider whose run ended with the engine". As decided. |
| M09 | `spec/core/README.md` | PRV-026 | Not in the work order; added for the same finding, since the section on providers restates PRV-026: the providers' servers "end when the engine does, or the part of it that holds them; once it has restarted, or that part has stopped, the engine runs each again". |
| M18 | `capabilities.yaml` | SEC-006, SEC-020 | `secret-provider.v1` description: "An implementation of `get-secret.v1` reached by `remote` that cannot check that proof is marked `plain`", where it said "A store reached by `remote`". Written into M81's text of the same passage. |
| M20 | `operations.yaml` | PRV-014, PLC-026 | `list-machines.v1` description: "trimmed and lowercased, of at most 63 characters, and at most once". `outputSchema` of `list-machines.v1` and of `place.v1`: `hostname` has `maxLength: 63` beside `minLength: 1`. `run.v1`, `stop.v1` and `inspect-run.v1` take a hostname that the engine had reported to it, and were not bounded, as the decision names only the two. |
| M35 | `operations.yaml` | REV-010, LIF-015, OPS-031 | `reconcile.v1` description: "Where a control of an instance that exists and is not retired took effect, `revision` is required". Its `effect` also: "For a control that took effect on an instance that exists and is not retired, stores the values it was sent", where it said "For a control that took effect"; the effect would otherwise say that values are stored where the description says they are not. |
| M35 | `spec/core/README.md` | REV-010 | Not in the work order; the bullet on reconciling restated the old rule: "For a control that took effect, on an instance that exists and is not retired, the request names `revision`". |
| M37 | `operations.yaml` | LIF-028, CON-031 | `reconcile.v1` description: "makes the instance ready or the Conduit open, unless the instance or the Conduit's consumer has been retired". Its `effect` also: "sets the state of the instance or the Conduit to ready, unless the instance or the Conduit's consumer is retired", for the same reason as under M35. |
| M37 | `spec/core/README.md` | LIF-028 | Not in the work order; the bullet on reconciling restated the old rule, and now ends the sentence as the description does. |
| M41 | `operations.yaml` | LIF-017, LIF-036, CON-008 | `unregister.v1` description: "Remove a template, or a retired instance, from the objects that exist", and "A request that names an instance that is not retired reports conflict". Its `effect`: "Removes the template, or the retired instance, from the objects that exist, and closes the Conduits of which that instance is the consumer"; the closing is added because CON-008, as area F2 changed it, now closes them when an unregistering removes the consumer, which the effect did not need to say while only an instance on a node declared lost, whose Conduits were already closed, could be unregistered. `stop.v1` description: "once more when an actor unregisters a retired instance". |
| M41 | `spec/core/README.md` | LIF-017, LIF-036, CON-008 | Bullet "Retiring and removing": "or a retired instance (LIF-017)", and the Conduits of which a retired instance is the consumer "are closed once its shutdown is confirmed or it is unregistered"; the second is not in the work order and follows CON-008. Section on providers: "once more when an actor unregisters a retired instance (LIF-036)". Both paragraphs reflowed. |
| M79 | `spec/schemas/examples/valid/project/conduit.yaml` | PKG-020, IFC-037 | The comment above `git-mirror-server` as decided; the item gains `plain: true` and `tools` with the one entry `'#mirror.sync.v1': sync`; head-comment line as decided. |
| M79 | `spec/schemas/README.md` | PKG-020 | Examples table, row `examples/valid/project`: "with a plain `command` item whose `environment` binds it". |
| M80 | `operations.yaml` | PRV-015, LIF-009 | `register.v1` description as decided: "a machine reported unreachable or omitted", and "for want of a machine it is rejected where no node matches, and where the provider names none, unless a node other than `local` that is not retired and matches represents a machine reported unreachable or omitted". `register-replacement.v1` description also gains "or omitted" in its clause on unavailable, which left it out in the same way; that is not in the work order. |
| M81 | `operations.yaml` | PKG-020, PRV-025, SEC-006, PRV-003 | `get-secret.v1` description: the decided two sentences replace the last one. |
| M81 | `capabilities.yaml` | PKG-020, PRV-001, PRV-025, SEC-006 | `secret-provider.v1` description: the decided two sentences, with M18's words in the second. |
| M82 | `operations.yaml` | PLC-017, PLC-027 | `declare-lost.v1` description begins "Retire a node and every instance placed on it, without their shutdown being confirmed."; the two sentences it replaces are gone, the rest and the `effect` are as they were. |
| M83 | `spec/core/README.md` | PRV-023, DL16 | Section on providers: the decided sentence, with the other set of DL16 and the exclusion of a value that begins with `()`. The next sentence, on the gate, had a line of 140 characters, which the reflow split. |
| M84 | `spec/schemas/README.md` | REC-002, IFC-020, CTR-004, LIF-009, LIF-031 | "Choices the schemas encode": the rows on `record.v1` and `object.v1` as decided. |
| M85 | `spec/core/README.md` | PRV-022, DL13 | "What this package does not settle", bullet on `identity`: "whether it gives `tls.crt` and `tls.key` to a run on its own runner, under the forms of proof the engine offers". |
| M86 | `spec/names.yaml` | none | The key `trustBundleFile` becomes `x509BundleFile`, edited by hand; its value stays `ca.crt`. Head-comment line as decided. |
| M86 | `tools/set_names.py` | none | `FILE_KEYS` reads `x509BundleFile`, with the decided comment above it. |
| M86 | `tools/check_schemas.py` | none | `IDENTITY_FILE_KEYS` reads `x509BundleFile`; the history block before `RULES = {` gains the decided line. |

## Head-comment lines and history sentences

| File | Items |
| --- | --- |
| `spec/core/operations.yaml` | M03, M09, M20, M35, M37, M41, M80, M81, M82 |
| `spec/core/capabilities.yaml` | M18, M81 |
| `spec/schemas/examples/valid/project/conduit.yaml` | M79 |
| `spec/names.yaml` | M86 |
| `tools/set_names.py` | M86 |
| `tools/check_schemas.py` | M86 |
| `spec/core/README.md`, a paragraph before "The promises are the source of truth" | M03, M09, M18, M20, M35, M37, M41, M80 to M83, M85 |
| `spec/schemas/README.md`, at the end of the history before "The promises are the source of truth" | M79, M84, M86 |

## In these files, and not in this work order

The decisions change these files for other items too, and this pass did not touch them:
SEC-024's row and the limit of a plain store in `spec/schemas/README.md` under M01; the
clause on a Remote's variables in that README and the docstring of `tools/check_schemas.py`
under M23; the head of `spec/schemas/schema.yaml` and that README under M24; the rule and the
example for PKG-030 under M25, and the tool and the example under M30, after which
`examples/invalid-by-rule` holds 30 files, as the README must then count; the engine's
credential ID in `spec/core/README.md` under M97; and the hook descriptions of
`operations.yaml` under M98. The log of area F7 assigns M97 and M98 to area F6.

## Checks

Run from the repository root after the edits:

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../CONSOLIDATION.md#validation) | 450 promises ok |
| Lines over 96 characters in the YAML files of F5 | none |
| `python3 tools/check_schemas.py` | Errors: 0; 28 sets under `examples/invalid-by-rule/`, since the examples of M25 and M30 are not yet written |
| `python3 tools/set_names.py --check` | Errors: 0; `ca.crt` x11, as before |
| `python3 tools/check_docs.py` | Errors: 0 |
| `git diff --check` | clean |
