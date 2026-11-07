# Re-audit, round 5: log of the fix pass for area F1

Date: 2026-11-07. Area F1, providers, secrets and placement, applies items M01 to M22 of
[DECISIONS.md](DECISIONS.md), with M47's change to PRV-022 and M23's part of SEC-024, in its
files: `spec/promises/providers.yaml`, `placement.yaml`, `pools.yaml`, `first-start.yaml` and
`secrets.yaml`. No change fell in `pools.yaml` or `first-start.yaml`, so they were not edited.
Nothing else was edited apart from this log, and nothing was committed or staged.

Every changed promise keeps its ID, and none was moved, added or removed. SEC-024's layer
changed from `language` to `convention`. Each changed file gained a head-comment paragraph for
round 5 that names its changed promises and the items. Text was rewrapped only from each change
on, at no more than 96 characters a line, and a code span or "RFC" and its number is not split.

## Applied

| Item | Files | Promise IDs | What changed |
| --- | --- | --- | --- |
| M01, M02, M05, M14 | `secrets.yaml` | SEC-024 | Replaced whole with the combined text, applied once: `layer: convention`; `when` rejects an item at a loopback host on an engine that can restart on another machine or run as several parts, and an item that binds a secret header, or that the template selects for `get-secret.v1`, at a URL that is neither `https` nor `http` on loopback; the observable as decided, with the protocol's rule on `http` cited as the 2026-07-28 revision states it. The old sentence "A header that carries no secret is not covered" goes with the whole replacement. |
| M23 (part) | `secrets.yaml` | SEC-024 | The sentence on the URL ends after "The URL is compared as the Remote writes it.", dropping "so a host written as a variable of the Remote is none of these". This was not in F1's list, but [items that change one text](DECISIONS.md#items-that-change-one-text) puts it in SEC-024's one text, and SEC-024 is in F1's file. F2 applied M23's rule to MAN-010 ([log-fix-F2.md](log-fix-F2.md)). |
| M01, M18 | `secrets.yaml` | SEC-006 | Observable: "an implementation of `get-secret.v1` reached by `remote` that cannot check an enforcement point's proof is used as plain, stands behind no gate, and can be sent a locator of its own choosing by whatever reaches its URL, which SEC-024 keeps to an `https` URL, or to loopback on an engine that can neither restart on another machine nor run as several parts", in place of "a store reached by `remote` ... is used as plain". |
| M19 | `secrets.yaml` | SEC-006, SEC-012 | SEC-006: "a store that declines the read" in place of "a refusal by the store". SEC-012: "where the store declines the read" in place of "where the store refuses". |
| M09 | `secrets.yaml` | SEC-010, SEC-011 | SEC-010 observable: "or again for a provider under PRV-026", dropping "once the engine has restarted". SEC-011 observable: "once the engine has restarted, or a part of it has stopped". |
| M17 | `secrets.yaml` | SEC-010 | Observable: "A sequence of such credentials" in place of "A chain". |
| M10 | `secrets.yaml` | SEC-013, SEC-023 | SEC-013 `then`: the engine holds the value "for as long as the connection lasts and no longer", and loses it sooner only where the engine restarts or the part that holds the connection stops, under DL17. SEC-023 `when` adds "or the part of it that held the connection stops, under SEC-013". |
| M03 | `providers.yaml` | PRV-007 | `then` begins "Where the runtime provider reports the run `stopped`, the engine asks it for the run", and ends with the case of `unknown`: no run is asked for, and the part is not dispatched and reports unavailable. Observable: the decided sentence on `unknown`, the Kubernetes warning, PRV-009 and PLC-017; and the run "is asked for only where the provider reports it `stopped`, so one run serves one instance; TPL-019 keeps a template to one item that is run". Two words differ from the decided text, each no longer: the last sentence of `then` names "the runtime provider" where the decided text says "it", which after the sentence before would read as the engine, and the observable says "reports it `stopped`" where the decided text says "reports none running", so that it matches `then`. TPL-018's part of M03 is F2's. |
| M04 | `providers.yaml` | PRV-004 | `when`, `then` and the observable's first sentence as decided: a Conduit template whose capability is a provider capability, or that lists one of their operations, is rejected when it is registered, and a template whose `operations` an input fills is checked for its operations at instantiation, as CON-028 checks at both times. The example and the tool's column stand. |
| M07 | `providers.yaml` | PRV-003 | `then`: the engine's own runner runs a `package` item only where its `transport` is of the `type` `stdio` and its `registryType` is one that DL16 states, ending the run of any other failed, under PRV-012. DL16's list of registry types is outside F1's files. |
| M08 | `providers.yaml` | PRV-002, PRV-026 | PRV-002 observable: where the engine's own runner runs a secret Resource, "its process too is reached over stdio and ends as that server does, and it is run again under PRV-026". The decided text says "ends when the engine stops"; "ends as that server does" is as short and also covers the part of the engine that M09 adds to the same sentence's server. PRV-026 observable: "under PRV-002 and PRV-003; DL16 leaves each to this promise", dropping "where DL16 has it end with the engine". |
| M09 | `providers.yaml` | PRV-002, PRV-026 | PRV-002 observable: the provider's server ends "when the engine stops, or the part of it that holds the server under DL17". PRV-026 `when`: "After the engine restarts, or a part of it that held a provider's run stops while the engine goes on, under DL17", and "ended when the engine or that part stopped". PRV-026 observable: "or when the part of the engine that holds them stops, under DL17". ACC-019, REC-002, DL17 and the other texts of M09 are outside F1's files. |
| M11, M15 | `providers.yaml` | PRV-025 | `then`: a connection has gone silent "once the gate's own check of that connection's liveness, such as an HTTP/2 PING or a TCP keepalive that it sends, goes unanswered". Observable: the combined text for both items, applied once, with one word changed: "with one JSON response" in place of "with one JSON object", since `definitions.md` calls a JSON object a map and keeps `object` for the JSON Schema type, which is M17's point. |
| M13, M17, M47 | `providers.yaml` | PRV-022 | `then`: a `CONDUIT_URL` given as the run sees it keeps "the engine's address whole: its scheme, host, port and path". Observable: the combined M13 and M17 text ("a line in a hosts file or a tunnel on the same port", RFC 9728's exact match of `resource`, and the canonical URI's port and path); and M47's sentences, under which an `http` URL on loopback goes only to a run on the engine's own runner whose process ends when the engine stops, on an engine that can neither restart on another machine nor run as several parts, and every other run is given an `https` URL. DL13's sentence of M47 is outside F1's files. |
| M16 | `providers.yaml` | PRV-006 | Observable: "only it could make a restriction enforced, although this version reports every restriction as mediated, under ACC-027". |
| M17 | `providers.yaml` | PRV-005, PRV-028 | PRV-005 `then`: "the error that the provider gives under IFC-008" in place of "an answer under IFC-008". PRV-028 observable: "unencrypted `http`" in place of "plain `http`". |
| M19 | `providers.yaml` | PRV-012, PRV-025 | PRV-012 observable: "and otherwise ends its run failed here". PRV-025 observable: "bridges it behind its gate or ends its run failed, under PRV-012". |
| M20 | `providers.yaml` | PRV-014 | `then`: no hostname "longer than 63 characters, the most that a label's value holds". Observable: the decided two sentences on a longer canonical name and on the engine's own hostname (PLC-026, DL5). The bounds in `list-machines.v1` and `place.v1` are outside F1's files. |
| M21 | `providers.yaml`, `placement.yaml` | none (head comments) | `providers.yaml`: `secret-provider.v1` for `secret-reveal.v1`, "its own machine provider" for "its own machine" and "the machine provider" for "the machines provider", in the sentences that describe the file as it stands. `placement.yaml`: "a node represents a machine that the machine provider reports" and "needs of a node". The lines that record earlier passes keep their words. |
| M22 | `placement.yaml`, `providers.yaml` | PLC-002, PRV-015 | PLC-002 `then`: where no item that is run is selected, the placement is `local` and no machine provider is asked; otherwise as before. PLC-002 observable: the decided last sentence. PRV-015 `when`: "An Agent or a Resource for which an item that is run is selected is instantiated with no placement given". |
| M12 | `placement.yaml` | PLC-023 | `when`: "or, while a node represents it, is omitted from the provider's latest report or no report has been received since the engine last started". Observable: a report from before the engine last started counts for nothing, so until the first report after it every machine but `local`'s counts as omitted. |

## Not applied

| Item | Files | Promise IDs | Why |
| --- | --- | --- | --- |
| M25 | `spec/README.md` | none | Outside F1's files. The row for `packages.yaml` in the contents table is F6's or the recount's: DECISIONS.md has every count recounted with a script after the fix pass. PKG-030 itself is in F2's log. |
| M26 | `spec/definitions.md` | none | Outside F1's files: the row "reference schema". |
| M32 | `spec/definitions.md` | none | Outside F1's files: the row "repeat". |
| M35 | `spec/definitions.md` | none | Outside F1's files: the rows "control" and "reconcile". |
| M41 | `spec/definitions.md` | none | Outside F1's files: the rows "run", "retired", "remove" and "unregister". |
| M42 | `spec/definitions.md` | none | Outside F1's files: the row "attempt, new attempt". |
| M91 | `spec/promises/objects.yaml` | OBJ-024 | Outside F1's files (F2's list names `objects.yaml`). |

## What the changes rely on elsewhere

- SEC-024 cites IFC-041 for an engine, made able to move after the template was registered,
  that connects to no item at a loopback host (M02's part of IFC-041, area F4).
- PRV-003 cites DL16 for the registry types that the engine's own runner runs (M07's part of
  DL16), and PRV-022 cites DL13 for the `http` loopback address (M47's part of DL13).
- `tools/check_schemas.py`'s rule `secret-header-https` checks only SEC-024's branch for a
  header that carries a secret, as its comment says. It does not check an item selected for
  `get-secret.v1`, and the branch on a loopback host depends on how an engine is configured,
  which its comment says is not in the documents. No check failed for it. The tool is outside
  F1's files.
- SEC-024 now has the layer `convention`, so the count of promises of the layer `language` falls
  by one, which DECISIONS.md's count of 203 already takes in.

## Checks

Run on the working tree with every area's edits in it, as they stood when this pass ended.

- The validation of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): 450 promises ok.
- `python3 tools/check_docs.py`: 0 errors on the last run. An earlier run reported 1 error in
  another area's file, `spec/core/README.md`, line 118, whose link to `log-fix-F5.md` led
  nowhere until that log was written. It was not F1's to fix.
- `python3 tools/set_names.py --check`: 0 errors.
- `python3 tools/check_schemas.py`: 0 errors.
- `git diff --check`: no output.
- No line of the three changed files is longer than 96 characters, and no added line splits a
  code span.
