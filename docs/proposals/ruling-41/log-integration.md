# Ruling 41: log of the integration of the promise files

Date: 2026-11-06. This is the account of the integrator's pass over the promise files after
groups G1 to G7 applied the [work order](PLAN.md) for [ruling 40](../ruling-40/README.md) and
[ruling 41](README.md), each to its own files (logs [G1](log-G1.md), [G2](log-G2.md),
[G3](log-G3.md), [G4](log-G4.md), [G5](log-G5.md), [G6](log-G6.md), [G7](log-G7.md)). The
rulings are the owner's. The edits below are the integrator's, and each stands unless the
coordinator or the owner strikes it. Names are those of the review and the plan, such as
`secret-reveal.v1` and `reveal-secret.v1`: the vocabulary audit that ruling 41 orders proposes
others before pull request 7 is finished.

Scope: every file under `spec/promises/`, `spec/conventions/` and `spec/interface/`, and
`spec/implementation-defined.yaml`. The core package, the schemas and their examples,
`spec/definitions.md`, `spec/README.md` and `tools/check_schemas.py` belong to the
core-and-schemas editor and the docs editor, who work next; what they must fix is listed under
[For the next editors](#for-the-next-editors).

## Checks

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | 426 promises ok: every promise has exactly the seven fields, a valid layer and level, and no ID is used twice |
| Lines over 96 characters, trailing whitespace, a field not folded with `>-` | None, in every file of the scope |
| `git diff --check` on `spec/` | Clean |
| An ID cited in promise text (`when`, `then`, `observable`) that does not exist | None. The seven editors had already pointed every citation of a removed ID at its successor or cut it, as the plan's "Cited by" lists asked |
| An implementation-defined item cited that does not exist | None in scope (DL14 is cited only in `spec/definitions.md`) |
| A removed ID not named in its file's head comment | None: all 67 are named, with ruling 41 and where their content went |
| A new ID that an earlier head comment names as removed | None |
| Every file in scope has a head-comment line for ruling 41 | Yes; each file this pass changed also names the change and this log |
| A request that is not dispatched and names no reason, or two | None |
| `python3 tools/check_docs.py` | 0 errors |
| `python3 tools/set_names.py --check` | 3 errors, none in scope: the removed keys `state` and `withheld` in `spec/core/README.md` and `spec/definitions.md` |
| `python3 tools/check_schemas.py` | 12 errors, none in scope: rules named for removed promises, and `schema.v1` not yet listing the profile's new keywords and `uri-reference` |

Head comments still name removed IDs in their history ("the second round changed GRA-017").
Those lines are the record of their time, and each such ID is named as removed later in the
same comment, or in the comment of the file that held it, so they were left as they are.

## Counts

Before the rewrite (commit `0a9a323`) 434 promises; after it 426: 67 removed and 59 added, as
the plan counts them. This pass removed and added none.

| Layer | Before | After |
| --- | --- | --- |
| `language` | 220 | 196 |
| `convention` | 179 | 190 |
| `interface` | 35 | 40 |
| All | 434 | 426 |

By level after: `must` 379, `must not` 43, `may` 3, `should` 1.

| File | Promises | `language` | `convention` | `interface` |
| --- | --- | --- | --- | --- |
| `spec/promises/objects.yaml` | 24 | 22 | 2 | 0 |
| `spec/promises/schemas.yaml` | 21 | 21 | 0 | 0 |
| `spec/promises/instantiation.yaml` | 23 | 22 | 1 | 0 |
| `spec/promises/templates.yaml` | 14 | 8 | 6 | 0 |
| `spec/promises/contracts.yaml` | 10 | 10 | 0 | 0 |
| `spec/promises/naming.yaml` | 28 | 24 | 4 | 0 |
| `spec/promises/packages.yaml` | 21 | 15 | 5 | 1 |
| `spec/promises/manifest.yaml` | 16 | 16 | 0 | 0 |
| `spec/promises/submissions.yaml` | 10 | 9 | 1 | 0 |
| `spec/promises/pools.yaml` | 15 | 10 | 5 | 0 |
| `spec/promises/placement.yaml` | 19 | 10 | 9 | 0 |
| `spec/promises/conduits.yaml` | 22 | 13 | 9 | 0 |
| `spec/promises/graphs.yaml` | 0 | 0 | 0 | 0 |
| `spec/promises/first-start.yaml` | 5 | 3 | 2 | 0 |
| `spec/promises/providers.yaml` (new) | 18 | 4 | 14 | 0 |
| `spec/promises/secrets.yaml` (new) | 20 | 5 | 13 | 2 |
| `spec/conventions/lifecycle.yaml` | 34 | 2 | 32 | 0 |
| `spec/conventions/access.yaml` | 29 | 0 | 29 | 0 |
| `spec/conventions/operations.yaml` | 29 | 0 | 29 | 0 |
| `spec/conventions/revisions.yaml` | 14 | 0 | 14 | 0 |
| `spec/conventions/observation.yaml` | 5 | 0 | 5 | 0 |
| `spec/conventions/records.yaml` | 6 | 0 | 6 | 0 |
| `spec/conventions/implicit.yaml` | 5 | 1 | 4 | 0 |
| `spec/interface/implementation.yaml` | 38 | 1 | 0 | 37 |

`spec/implementation-defined.yaml` holds 14 items (DL14 removed).

## Terms

A search of every promise's `when`, `then` and `observable` for the terms that go found none in
a rule: graph, step, a runtime as a template or a Type declares it (`runtimes` appears only as a
key that OBJ-019 and TPL-001 reject), a handle held, policy, composite, component, constituent
(only as a key CTR-005 rejects), re-export, rename, facts, default location, `obliges` and
`narrows` (only as keys CTR-005 rejects), retained result, retrieval, withheld, isolation
plugin, `interfaceVersion`, `inProgress`, value reference, unpublished package and
`<digest>#<name>`. "Unregister" is used of an instance only where its location was declared
lost (LIF-017) and of a first-start object, which LIF-034 refuses; a confirmed shutdown removes
an instance (LIF-035). Wording that read as an old term was changed in OBS-004 ("restart
policy"), ACC-030 and SEC-001 ("narrows") and PRV-001 and PRV-002 ("secret store"); see the
fixes. "Narrows" stays in TPL-010, which the plan keeps
unchanged and where a template's schema narrows a control's input; "narrow access" (LIF-001,
ACC-011) is the plan's own phrase.

## What this pass fixed

| ID | File | What changed | Why |
| --- | --- | --- | --- |
| OPS-007 | `spec/conventions/operations.yaml` | "No result arrives" excepts the error -32021 of IFC-008, and the observable says that error gives failed | IFC-008 (G5) answers a revision of Conduit's extension that the implementation does not support with the protocol's error and no tool result, and says the outcome is failed, as it was before ruling 41; OPS-007 (G4) made every case with no result unknown. The two promises contradicted each other. Failed keeps the outcome IFC-008 always had: the implementation performed nothing. The coordinator may rule unknown instead; then IFC-008 changes, not OPS-007 (G5's log, item 5) |
| REC-002 | `spec/conventions/records.yaml` | A provider call's record holds a caller only where the call is made for a request, that request's caller; a read of a secret has the caller of the setup or the retirement it belongs to | REC-002 (G4) gave a provider call "the caller of the invocation it serves", but PRV-005 (G2) gives `place.v1` and `fetch-package.v1`, made before a request is dispatched, the request's caller and no invocation, and gives a call the engine makes of its own accord neither. SEC-011 names the setup or retirement as what a read belongs to |
| ACC-005 | `spec/conventions/access.yaml` | A declared Conduit that lists `reveal-secret.v1` is opened at setup only where the root Interface sent the setup on its own behalf | SEC-008 (G7) dispatches such an opening only where the root Interface sent it on its own behalf; ACC-005 (G3) said only "the root Interface sent the setup", which admitted an onward request of the root Interface (G7's log, C6) |
| ACC-006 | `spec/conventions/access.yaml` | The observable's last clause names a retired consumer only, and says that a Conduit whose target is retired is closed already | The clause said that a request to open a Conduit "either of whose ends is retired" reports conflict, beside a sentence saying that a closed Conduit no longer exists and reports invalid; a Conduit whose target is retired is closed at once (CON-008) and so removed (LIF-035) |
| ACC-030 | `spec/conventions/access.yaml` | "A bound narrows by Pools" → "A bound limits by Pools" | `narrows` is a term that goes (C20) |
| SEC-001 | `spec/promises/secrets.yaml` | "This narrows OBJ-021" → "This rule is stricter than OBJ-021" | As ACC-030 |
| OBS-004 | `spec/conventions/observation.yaml` | "under its own restart policy" → "under its own setting for restarts" | "Policy" goes (3a); PRV-010 (G2) already says "its own setting for restarts" |
| PRV-001, PRV-002 | `spec/promises/providers.yaml` | "a secret store's type" → "the type of a secret Resource"; "A secret store is not among them" → "A secret Resource is not among them"; the head comment likewise | `secrets.yaml` (G7) calls the object a secret Resource and keeps "store" for the system outside the engine that a locator names (G7's log, C16). The plan's rows define "secret store" and "secret Resource" as the same object; the promises now use one name for it |
| PRV-002 | `spec/promises/providers.yaml` | "The engine's runner" → "The engine's own runner" | The term every other promise uses |
| PLC-002 | `spec/promises/placement.yaml` | "Where no machines provider exists" → "exists that is not retired" | PRV-003 and PRV-013 (G2) count only a machines provider that is not retired |
| PLC-005 | `spec/promises/placement.yaml` | The example "as the `target` of a declared Conduit" goes; the observable says that a location offers no capability, so no Conduit to one is instantiated (PLC-008, CON-021) | A location offers no capability after decision 8 (PLC-008), so that example no longer held (G2's log, "Found outside the plan") |
| POOL-012 | `spec/promises/pools.yaml` | `runtime` joins `owner` and `placement` among the keys an entry may not carry | OBJ-014 (G6) rejects all three; POOL-012 named two (G2's log) |
| TPL-009 | `spec/promises/templates.yaml` | "Nothing else is done" excepts the run that TPL-018 asks for to serve the hook and its reads of secrets; the parts of a retirement include that run | TPL-018 (G1) asks for the run before the hook where it is not running (P4), SEC-010 reads before every run, and OPS-022 (G4) counts the parts of a retirement as those of TPL-009 and TPL-018; TPL-009 said that only the hook and the stop are done |

## Settled without an edit

Questions that the editors' logs raise across groups, found already consistent:

| Question | Where it was raised | Why no edit is needed |
| --- | --- | --- |
| A further setup after a hook's outputs have lapsed would report conflict under OPS-032 | G1, item 2 | OPS-009 (G4, E2) answers a repeat of an invocation of an operation that declares no outputs from its outcome alone, and `setup-hook.v1` and `retire-hook.v1` declare none (plan section 4), so a hook that succeeded is never a repeat that OPS-032 answers |
| `register-replacement.v1` that names `root` in `instance` | G1, item 5 | LIF-033 gives invalid and LIF-029 conflict; OPS-023 reports invalid first. No case found where it matters |
| ACC-001 and ACC-003 against SEC-007 | G7 | Both now say "an actor on its own behalf, or any instance as an onward request", as SEC-007 does |
| Whether a provider call is reconciled by itself | G4, I3 | PRV-005 gives a provider call made within an invocation that invocation's ID, so the setup or retirement it serves is what OPS-014 reconciles |
| DL1, DL5, DL9 and DL10 changed meaning | G5 | LIF-026, OPS-021 and OPS-028 no longer cite DL1; LIF-021 cites it for the order of `retire-with-resources.v1`, which it still holds. LIF-010 cites PLC-023 for what DL5 held; PLC-023 and PRV-014 cite DL5 for the report's frequency. REV-012 cites REV-014 for what DL9 held. DL10 still holds the behavior when storage is exhausted, which SEC-016 cites |
| CON-008's clause for a machine reported gone | G3, choice 4 | Kept: PLC-024 makes the location count as declared lost, and the clause states the same effect for a Conduit's ends |
| Citations of SEC IDs and of other groups' new IDs | G2, G3, G7 | Every one exists and says what the citing sentence relies on |

## Editors' choices beyond the plan, kept for the coordinator

Each is consistent with the other files; the coordinator confirms or strikes it.

- PRV-004 also refuses a Conduit that lists an operation of `runtime-provider.v1`,
  `machine-provider.v1` or `package-source.v1` under another capability (G2, choice 16).
- PRV-005 has a provider call carry the provider's own `instance`, `config` and `revision`,
  so that a provider learns its configuration as any implementation does under IFC-006 (G2,
  choice 17).
- REV-014 excepts a repeat that OPS-009 or OPS-012 answers without a dispatch, where the plan
  says "a repeat under OPS-009" (G4, E16).
- TPL-012 connects to each `remote` item among the selected implementations, where the plan says
  "or connecting to its `remote` item" (G1).
- IFC-009's clause "as IFC-008 states for one such result" was cut (G5); NAM-008, NAM-021 and
  PKG-017 were changed beyond the plan's list to agree with PKG-006, MAN-016 and MAN-004 (G6).
- OBJ-019 exempts the labels of a label selector, and CTR-012 requires `type: object` at the root
  of `inputSchema` and `outputSchema` (G6, choices 5 and 8).

## What remains

### Needs a decision

Gathered from the seven logs, merged where two logs raise one question, with the integrator's
own findings marked. None is decided in the text.

| # | Question | Raised by | Promises |
| --- | --- | --- | --- |
| R1 | The owner's answer on P2 deletes `spec/promises/graphs.yaml` and has `spec/README.md` list the promise files that rulings removed, with their IDs. The file is still `[]`, and its head comment says it is kept so that its IDs stay named. This pass did not delete it: that needs the README's list first (docs editor), and this pass was not asked to remove a file | Integrator | `graphs.yaml`, `spec/README.md` |
| R2 | Refused or invalid through a closed Conduit. Decision 17's cost line says a request through a closed Conduit reports invalid, because the Conduit is removed; the plan and G3 keep refused for a request through one (its tools are no longer presented) and give invalid only to a request that names it. If the cost line is read as ruled, ACC-011, ACC-034, OPS-009, IFC-012 and IFC-013 change together | G3, D2 | ACC-011, ACC-034, IFC-012, IFC-013 |
| R3 | What closed a Conduit, once it is removed: REC-002 does not say that the record of a closing that no request made names its cause | G3, D1 | CON-008, REC-001, REC-002 |
| R4 | Whether the stop of a run waits for the retire hook to succeed, as the parts of a setup wait for each other; the text asks for it after the hook in every case | G1, item 1 | TPL-009, TPL-018, PRV-008 |
| R5 | Whether `retire-with-resources.v1` goes on to retire the other Resources after one reports conflict under REV-014 | G1, item 3 | LIF-021, REV-014 |
| R6 | Whether a Type may require a hook, or state a default implementation for one; plan section 4 has the checker allow no default for hook operations | G1, item 4 | TPL-004, TPL-017, TPL-018 |
| R7 | The caller, and the restrictions, of a new attempt of a hook that a further setup by another actor dispatches, given that the hook's identity is scoped to the instance alone | G4, D1 | OPS-028, TPL-017, ACC-018 |
| R8 | A retirement that needs a connection to a `remote` item for its retire hook, and a connection that drops or an engine that restarts outside a setup: the read of the headers' secrets belongs to no invocation and has no caller (4c) | G3, D3; G5, item 1; G7, D2 | SEC-011, SEC-013, IFC-041, ACC-039 |
| R9 | Values that are not strings at a place that `environment`, `arguments` or `headers` points to | G2, item 6; G5, item 6; G6, item 3 | MAN-010, MAN-017, PRV-007, IFC-041 |
| R10 | The keys of `run.v1`'s `secrets`, since a variable and an argument may share a name | G7, D1 | SEC-010, PRV-007, PRV-011 |
| R11 | A place that holds null under `{type: null}`: no variable, or an empty one | G7, D4 | SEC-010, MAN-017 |
| R12 | What the engine's own runner gives a process beyond what the item binds (`PATH`, `HOME`, the login session a keychain needs) | G7, D3 | SEC-020, PRV-003 |
| R13 | SEC-004 keeps a reference that is not yet resolved in no form | G7, D5 | SEC-004 |
| R14 | Layers: PLC-007, PLC-013 and POOL-016 are `language` but read labels or a placement that the machines provider reported, as PLC-002, moved to `convention`, does | G2, item 1 | PLC-007, PLC-013, POOL-016 |
| R15 | A machine that a later report omits without reporting it gone; a machines provider that is retired or replaced; `local` reported gone, which would retire the first-start objects; an actor that retires the location of a machine still reported | G2, items 2 to 5 | PLC-024, PLC-026, PRV-013, LIF-022 |
| R16 | Two selected items whose `placement` name one label with different values: accepted at registration, and every instance then fails PLC-007 | G2, item 8 | PLC-025 |
| R17 | A `default` for an input whose destination is an entry's `runtime`; a `runtime` that names a retired runtime provider | G2, items 9 and 10 | POOL-009, PLC-005, PRV-006 |
| R18 | How often the engine asks `inspect-run.v1`; whether the engine asks for a run or its stop while the machine is unreachable; a run of another provider that returns no `endpoint` | G2, items 7 and 11; G5, item 2 | PRV-007, PRV-009, PLC-023, IFC-017 |
| R19 | A machines provider that is not ready when an instance is to be placed: PRV-015 does not say whether the registration reports unavailable, as PRV-017 has an installation do for a package-source provider, or the document is rejected | Integrator | PRV-015 |
| R20 | Whether a provider call declares Conduit's extension, and what happens where a provider's discovery result lacks it; whether the engine's own server lists the extension and a consumer must declare it | G5, items 3 and 4 | PRV-005, IFC-029, IFC-037 |
| R21 | IFC-008's outcome: this pass made OPS-007 agree with IFC-008's failed; unknown is the other reading | G5, item 5 | IFC-008, OPS-007 |
| R22 | An OPS-032 result does not say whether the invocation is still in progress or its outputs are gone | G4, D2 | OPS-032 |
| R23 | After a purge, an invocation of unknown outcome whose request carried no operation identity, and whose result was lost, cannot be found to be reconciled | G4, D3 | REC-004, OPS-008, LIF-013 |
| R24 | What the engine does with its own setup or retirement (a location's setup, a retirement for a machine reported gone) that would begin while another of the instance's has not ended | G4, D4 | REV-014, PLC-024, PLC-026 |
| R25 | What the engine does with the parts in progress of a cancelled `set-up.v1` or `retire.v1` | G4, D5 | OPS-027 |
| R26 | Whether a hook can carry a filter in practice, since `set-up.v1` and `retire.v1` are dispatched to `engine`, which supports none | G3, D4 | ACC-015, ACC-018 |
| R27 | Replacing a template while a package its manifest lists is missing; how long a submitted project's address is held, and whether two projects may state one address; which key names a positional argument of a server.json Package | G6, items 1, 2 and 4 | PKG-006, SUB-009, PKG-029, MAN-016, MAN-010 |
| R28 | To check against the protocol in round 5: the extension negotiation, `listChanged` and `cacheScope`, and the reserved trace keys | G5, item 8 | IFC-004, IFC-008, IFC-037 to IFC-039 |

### For the coordinator's tracker

- Ruling 41 records removing policies (3a) as a candidate access feature to design for a later
  version, as Conduit's own. It is in no promise, by design.
- The vocabulary audit reviews every name the plan coined (P1) with `secret-reveal.v1` and
  `reveal-secret.v1`, and G5's question on the key `revision` beside the extension's `revision`
  setting (G5, item 7).

## For the next editors

Citations of removed IDs outside the scope of this pass, by file, as the tree stands now. Each
is the core-and-schemas editor's or the docs editor's under the plan; this pass changed none of
these files.

| File | Removed IDs it cites |
| --- | --- |
| `spec/definitions.md` | 53, among them every GRA ID it named, DL14, OBS-005, OBS-007, TPL-006, TPL-007, TPL-015, TPL-016, LIF-030, PLC-003, PLC-004, PLC-016, PLC-021, the policy, composite, rename and retrieval IDs |
| `spec/core/README.md` | 30 |
| `spec/schemas/README.md` | 27 |
| `spec/schemas/graphs.yaml` | 22 (the file is to be deleted, plan section 4) |
| `spec/core/operations.yaml`, `types.yaml`, `capabilities.yaml`, `requirements.yaml`, `templates.yaml`, `results.yaml` | 6, 5, 4, 4, 1, 1 |
| `spec/schemas/contracts.yaml`, `manifest.yaml`, `instances.yaml`, `templates.yaml`, `values.yaml` | 3, 3, 2, 2, 1 |
| Examples under `spec/schemas/examples/` | 10 files, most of them to be removed or rewritten under plan section 4 |
| `tools/check_schemas.py` | 9, in the rules that plan section 4 removes or changes |

Terms that the promises now use, for `spec/definitions.md`, beside the rows of the plan's
section 1:

- **secret Resource** for the object and **store** for the system outside the engine that its
  **locator** names (SEC-005, SEC-006). The plan's row "secret store" names the same object as
  "secret Resource"; the promises do not use it.
- **item that is run**: an implementation item reached by `command` or `package` (TPL-019,
  PRV-007); one reached by `remote` is **connected to** (IFC-041), and the **connection** is a
  part of the setup (SEC-013).
- **hook**, alone, for a setup hook or a retire hook (ACC-015 to ACC-018).
- **stand for**: a location stands for the machine it was registered for, until it is retired
  (PLC-026); **counts as declared lost**: a location whose machine was reported gone (PLC-024).
- **claim** (a scheme): a package-source provider claims the schemes its configuration lists
  under `schemes` (PRV-016).
- **the engine's own runner, machine and fetching**: the built-in implementations of PRV-003.
- **restriction**: a filter or a bound; a **bound** is a restriction that the engine applies
  itself, and no longer a filter (ACC-029).
- **management verb**: an operation of `object-management.v1`, `access-control.v1` or
  `pool-management.v1` (CON-024).
- **the engine's read of a secret** for the plan's "read" (ACC-001, SEC-010).
- **qualified reference** also covers `address#name` of a submitted project's contract or
  implementation (ACC-029, ACC-038, SUB-008).
- **user information**, RFC 3986's term, for what `fetch` may not hold (MAN-006).
- **removed**: LIF-014 still lists confirmed among a retired instance's shutdowns, but a
  confirmed shutdown removes the instance (LIF-035), so confirmed is seen only in the result
  and the record of what confirmed it (G1).

## Coverage check

Date: 2026-11-06. A coverage check read the rewrite against every decision of rulings 40 and
41 and against scenarios across two machines, and returned 45 findings. This pass applied
them to `spec/`, `tools/check_schemas.py` and `docs/OVERVIEW.md`, fixing what follows from the
rulings and listing what needs a new decision. The names are still the review's, such as
`secret-reveal.v1` and `reveal-secret.v1`, for the vocabulary audit. Every file this pass
changed under `spec/` names the change in its head comment, or for a Markdown file in its
head paragraph.
To delete `spec/promises/graphs.yaml`, three links in records outside that scope were pointed
at the file's last version, at commit `0a9a323`: one in
`docs/proposals/graphs-and-schemas/README.md` and two in `docs/proposals/secrets/README.md`.

### Checks

Run from the repository root at the end of this pass.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | `432 promises ok` |
| `python3 tools/check_schemas.py` | `Errors: 0`. 96 files holding 229 documents; 58 Schema contracts; 146 documents validated; 52 invalid files rejected; 102 exports; 926 citations of promises in 99 files; 20 rules from 19 promises, and the 21 sets under `examples/invalid-by-rule/` each rejected by the rule they are named for |
| `python3 tools/set_names.py --check` | `Errors: 0` |
| `python3 tools/check_docs.py` | `Errors: 0` |
| `git diff --check` | Clean |
| Lines over 96 characters in a YAML file under `spec/` | None |
| A field of a promise not folded with `>-` | None |
| A promise ID or a DL item cited that does not exist | None. The GRA IDs that `spec/README.md` lists as removed, and head comments that name removed IDs as history, are the only citations of IDs that no promise holds |

### Counts

432 promises: 198 `language`, 194 `convention` and 40 `interface`; by level `must` 385,
`must not` 43, `may` 3 and `should` 1. This pass added six and removed none:

| ID | File | Layer |
| --- | --- | --- |
| PRV-019 | `spec/promises/providers.yaml` | `convention` |
| PRV-020 | `spec/promises/providers.yaml` | `convention` |
| PLC-027 | `spec/promises/placement.yaml` | `convention` |
| PLC-028 | `spec/promises/placement.yaml` | `language` |
| SEC-021 | `spec/promises/secrets.yaml` | `language` |
| OPS-033 | `spec/conventions/operations.yaml` | `convention` |

Files whose counts changed: `providers.yaml` 20 (4 `language`, 16 `convention`),
`placement.yaml` 21 (11, 10), `secrets.yaml` 21 (6 `language`, 13 `convention`, 2
`interface`), `operations.yaml` 30 (30 `convention`). `spec/promises/graphs.yaml`, which held
none, is gone. `examples/invalid-by-rule/` holds 21 files, and the tool applies 20 rules.

### One row per finding

"Fixed" means the text now says what the rulings decide. "Listed" means the finding needs a
decision that the rulings do not make; its number in the next list says which.

| # | Finding | Outcome | What changed, or why not |
| --- | --- | --- | --- |
| 1 | No promise runs an instance on the machine of its location, and the engine's own runner runs every instance on the engine's machine (decisions 2, 8c, 31b; ruling 40, point 3b revised) | Fixed; the alternative is D1 | New PRV-019: a runtime provider runs the item on the machine that `location` names, and on no other, or the run ends failed and runs nothing. PRV-003: the engine's own runner runs only on the engine's machine, so under PRV-019 it ends failed the run of an instance placed elsewhere. That is the fix's option (b), which follows from PRV-003's rule that the engine's own runner keeps every promise of a runtime provider. PRV-006, PRV-007, POOL-009, IFC-017, `run.v1`, the definitions rows location, the engine's own runner, runtime provider, `runtime` and run, the core README and `docs/OVERVIEW.md` (placement, Providers) say the same |
| 2 | A runtime provider can be retired while an instance names it, which can then never be removed (2, 31b, 17) | Fixed; editor's choice, D2 | New PRV-020: the retirement of a runtime provider that an instance which exists names, retired or not, is not dispatched and reports conflict, with a diagnostic naming the instances, as PLC-011 has it for a location. Decision 31b has an instance name its runtime "as it names its placement". PRV-006 and PRV-008 cite it. LIF-021 retires a Resource before the runtime provider it names and notes the conflict. The definitions row and the OVERVIEW say so |
| 3 | SEC-012 has no later part begun after a failed read in a retirement, against TPL-009's stop after the hook (1a, 2c, 17) | Fixed; R4 is D3 | TPL-009 asks for the stop whatever came of the hook, as its text and G1's reading had it. SEC-012's observable now separates a setup, where no later part is begun, from a retirement, where the hook is not dispatched, its part reports unavailable, and the stop is still asked for. TPL-018, PRV-008 and the OVERVIEW say the same. Kubernetes likewise stops a container after a PreStop hook that failed |
| 4 | A further retirement asks for a run, with reads of secrets, for a hook that it will not dispatch (1a, 1c, 2c) | Fixed | TPL-018 asks for the run only where the hook is to be dispatched: no earlier invocation of it, or one that OPS-012 or OPS-024 would dispatch as a new attempt. Otherwise the retirement goes on to the stop. PRV-007's and SEC-010's observables and the definitions row run follow |
| 5 | A further retirement of an instance on a location declared lost, or whose machine is gone, asks for a run that cannot be described and dispatches its hook (8e, 1a, 2) | Fixed; the alternative is D4 | New PLC-027: for such an instance no run is asked for, no secret is read and the hook is not dispatched, its part reporting unavailable, as PLC-023 has it for a machine reported unreachable; the stop is still asked for. PRV-007 says that `location` therefore always names a machine for which a location that is not retired stands. LIF-026, TPL-018 and PLC-024 point to it |
| 6 | No promise restricts who may send requests to a run's endpoint (2, 31b, section 4 point 4) | Listed, D5 | `spec/README.md` now names the exposure among what the guarantee for secrets does not cover |
| 7 | LIF-016 forbids the retirement from stopping the run (2, 17) | Fixed | LIF-016 allows "its own run, which its retirement stops under PRV-008"; the definitions row shutdown and the OVERVIEW say so |
| 8 | PLC-017, LIF-024 and LIF-014 describe the status of instances and Conduits already removed (17) | Fixed | PLC-017 no longer has a confirmed instance "stay confirmed", and says that one with no setup is removed. LIF-024 says that a closed Conduit's shutdown is confirmed when it closes and the Conduit is then removed. LIF-014 says that inspection shows pending or unknown, and confirmed only in the result and the record of what confirmed it |
| 9 | Refused or invalid through a closed Conduit, against decision 17's cost line (R2) | Listed, D6 | No change |
| 10 | `spec/promises/graphs.yaml` is still in the tree (owner's answer on P2; R1) | Fixed | The file is deleted, not with Git. Its last version is at commit `0a9a323`, to which the three record links and the row of `spec/README.md` under "Promise files that rulings removed" now point. R1 is settled |
| 11 | ACC-039 counts a hook as a request through a Conduit (1a, 4c) | Fixed | ACC-039's `then` permits "an onward request that serves the consumer's retire hook", and no longer the hook itself. The wider sense of "request" is left to the vocabulary audit |
| 12 | The engine's read of a secret on another machine: no proof of the engine and no promise that the endpoint admits only the engine (section 4, points 4 and 5; ruling 40, point 3e) | Listed, D5 | `spec/README.md` names the exposure, and that a value read crosses the path between a store's machine and the engine's |
| 13 | A project cannot name its own operations in a bound under NAM-022 and MAN-004, which the valid example does (11) | Fixed; editor's choice, D7 | ACC-029: each item of a bound's `operations` is data under OBJ-021, compared as a name and never resolved, so NAM-007 does not apply and a project's own documents may write its address there, as SUB-008 ("in a bound") and the example have it. NAM-007, NAM-022 and MAN-004 note the exception; `bound.v1`'s description and comment, the definitions row bound and the core README follow. Two projects that state one address (R27) is D12 |
| 14 | SEC-018 forbids the engine's own runner from giving a process the value it read (section 4, points 5 and 7) | Fixed | SEC-010 has the engine's own runner deliver each value itself. SEC-018 adds the variable or argument of the process that runner starts, and says that the process is the one place where it keeps the value, which answers PRV-011 for the built-in runner with no item of the implementation-defined list |
| 15 | ACC-034 delivers outputs after the close, while a `tasks/get` for them is refused (6a, 6c) | Fixed | ACC-034 delivers the result in the response to the request that dispatched it. A result that only a later `tasks/get` would read is not delivered once the Conduit has closed, and the caller learns the outcome from the records, which is decision 6's cost line. IFC-016 says the same |
| 16 | INS-006 accepts a bare name where `secret-reference.v1` governs (section 4, point 1) | Fixed | INS-006 excepts `secret-reference.v1`, under SEC-001 |
| 17 | SEC-020's observable claims more than its `then`, relying on R12 (13a) | Fixed in part; R12 is D8 | The observable now says that the engine passes no exported token to any run, which is what the `then` binds, and that this promise does not state what the engine's own runner gives a process beyond what the item binds |
| 18 | A rotated value reaches a running instance only through a retirement (section 4, point 5) | Stated; the choice is D9 | SEC-014's observable and `spec/README.md` state the consequence of the text as it stands. Whether an actor may end a ready instance's run is not decided |
| 19 | A secret Resource's type may offer another capability whose operation returns the value (19, section 4, point 4) | Listed, D10 | `spec/README.md` names the case among what the guarantee does not cover |
| 20 | A variable that server.json marks `isSecret` may be bound to a place for a literal (section 4, point 1; 25d) | Fixed | New SEC-021 (`language`): a template whose selected item binds such a name to a place that `secret-reference.v1` does not govern is rejected. No checker rule or example was added |
| 21 | A machine reported gone that comes back keeps a run, and its secrets, that nothing stops (8e; section 4, point 5) | Listed, D11 | `spec/README.md` names the case. Under TPL-009 and PLC-027 a further retirement of each instance asks for the stop before a request to unregister removes the instance |
| 22 | ACC-022 says the root Interface can open any Conduit (section 4, point 4; 31) | Fixed | ACC-022 names what ACC-032, PRV-004 and SEC-009 rule out |
| 23 | `docs/AUDIT.md` lacks the tracker entry for policies as a candidate access feature (3a) | Not done here | `docs/AUDIT.md` is outside the files of this pass. For the coordinator, as "For the coordinator's tracker" above already says |
| 24 | R8 and R27 still break decisions 4c and 11 | Listed, D12 | No change |
| 25 | The checker and both READMEs require a control's `inputSchema` to list `revision` alone, which no promise states and REV-006 contradicts (16a) | Fixed | Decision 16a keeps REV-006, which lets a control list other inputs under `required`. The tool no longer fails such a control and cites CTR-006 alone; `spec/core/README.md` and `spec/schemas/README.md` say so. The tool presented to a consumer still lists `revision` alone, under IFC-033 |
| 26 | OPS-008's `when` covers only a request with an operation identity, but its state is promised for every invocation (C33) | Fixed | OPS-008's `when` covers every invocation that is dispatched, and its scope rule speaks of an identity that a request carries |
| 27 | REV-012 and REV-014 against PLC-024's retirement while a setup is in progress (R24) | Listed, D13 | No change |
| 28 | No promise decides a request whose arguments fail the `inputSchema`, so IFC-025 has no `ruleId` for it (24a) | Fixed | New OPS-033: such a request is not dispatched and reports invalid, a control's inputs other than `revision` apart, under REV-006. NAM-023 points to it |
| 29 | `record.v1` has one invocation member, but the record of a read needs two IDs (24c) | Listed, D14 | No change: the member's name and which ID a provider call's record puts where need a decision |
| 30 | Duplicate of 1: the store of a secret placed on another machine runs on the engine's machine (25) | Fixed with 1 | As row 1 |
| 31 | The tool of an operation with no `inputSchema` has none, which the protocol requires (16a) | Fixed | IFC-033: its tool has the `inputSchema` `{type: object}`, as the core-and-schemas log's choice 4 read it, with no `additionalProperties`, as its choice 5 has it |
| 32 | `replace.v1` with a manifest that lists a package that is not installed is checked by nothing (12) | Fixed | PKG-006 also rejects such a replacement, and SUB-009 names it among the manifests rejected. NAM-028's clause "and that version of the package is installed" is left as it is |
| 33 | `server-package.v1` accepts the transport `sse`, over which no promise reaches a run (25a) | Fixed | MAN-010 and `server-package.v1` accept `stdio` and `streamable-http` only, as 25b does for `remote`. No invalid example was added |
| 34 | Duplicate of 1, naming POOL-009 | Fixed with 1 | As row 1; POOL-009 says that an entry given no runtime runs only on the machine for which `local` stands |
| 35 | A report that the engine's own machine is unreachable or gone retires, or locks out, the first-start objects (8d, 8e; R15) | Fixed in part; the rest is D15 | PLC-023 and PLC-024 leave out the machine for which `local` stands: the engine runs on it, and `management`, `access` and the providers are placed there. The definitions row first-start object and the OVERVIEW now hold. How the engine tells its own machine in a report is not stated |
| 36 | A project's template of the type `location.v1` registers a location by hand (ruling 40, point 3b revised; 8, 8d) | Fixed | New PLC-028 (`language`): a template other than the core package's `location` whose `type` is `location.v1` is rejected. FST-006 and PLC-008 point to it. New checker rule `location-type` and example `PLC-028-template-of-the-location-type.yaml`; `spec/schemas/README.md`, the definitions row location |
| 37 | POOL-016 is `language` but places by a provider call (R14) | Listed, D16 | No change |
| 38 | A run on another machine has no stated way to reach the engine or to get what it must present (26a; 3e) | Listed, D17 | No change |
| 39 | Duplicate of 12: requests from the engine to an implementation on another machine are not authenticated (3e) | Listed with 12, D5 | As row 12 |
| 40 | How decision 18's extension applies to provider calls (R20) | Listed, D18 | No change |
| 41 | PLC-023 does not cover the run, the stop or the reads for an instance on an unreachable machine (8e; R18) | Fixed; the rest of R18 is D19 | PLC-023 now covers asking for the run or the stop of such an instance: the part reports unavailable, and no secret is read for a run that is not asked for. PRV-007 and PRV-008 point to it. A `remote` item is not on the instance's machine and is not covered |
| 42 | A runtime provider other than the engine's own runner cannot find an in-package program (25a, 2) | Listed, D20 | No change |
| 43 | Duplicate of 10 | Fixed with 10 | As row 10 |
| 44 | ACC-011, ACC-034 and CON-008 state revocation at once, beside ACC-037 (ruling 40, point 3f) | No change for v0 | For the tracker, with ruling 40's item for v1: these three are the promises, beside ACC-037, that a bound on revocation delay would change |
| 45 | The head comment of `secrets.yaml` says "the engine's host" (8) | Fixed | It says "the machine that the engine runs on" |

### Still needs a decision

| # | Rows | Question |
| --- | --- | --- |
| D1 | 1, 30, 34 | Whether an instance that names no `runtime` is placed only on `local` (PRV-015 does not ask the machines provider for it), or is rejected at instantiation where it is placed elsewhere, so that the failure shows when the document is resolved and not at setup. The text now ends such a run failed (PRV-019) |
| D2 | 2 | Confirm PRV-020, or have the stop count as succeeded once the instance's runtime provider is gone |
| D3 | 3 (R4) | Whether the stop waits for the retire hook to succeed. If so, TPL-009, PRV-008 and SEC-012 change together, and an instance whose hook's run can never be had needs a way to a confirmed shutdown |
| D4 | 5 | Confirm PLC-027, or have a retirement of an instance on a location declared lost skip the hook and ask only for the stop, so that it can succeed and remove the instance with no request to unregister |
| D5 | 6, 12, 39 | Whether ruling 40's point 3e covers requests from the engine to an implementation on another machine: a promise that a run's endpoint admits only the engine and that the engine proves it sent a request, by a mechanism of DL13, with the trust domain naming the path to a store; or a record that 3e is read as consumer to engine only, with the endpoint's exposure left to the runtime provider's contract |
| D6 | 9 (R2) | Refused or invalid for a request through a closed Conduit |
| D7 | 13 | Confirm that a bound's `operations` are compared as names and never resolved |
| D8 | 17 (R12) | What the engine's own runner gives a process beyond what the item binds: a promise naming a set, or an item of the implementation-defined list |
| D9 | 18 | Whether an actor may end the run of a ready instance, so that a rotated value reaches it at the next setup |
| D10 | 19 | Whether a type that offers `secret-reveal.v1` may offer any other capability, or one with an operation that has outputs |
| D11 | 21 | Whether the engine asks for the stop of the runs on a location declared lost once it can, or a stop is required before a request to unregister |
| D12 | 13, 24 (R8, R27) | A read for a reconnection outside a setup (R8); two submitted projects that state one address (R27) |
| D13 | 27 (R24) | The engine's own retirement under PLC-024 while a setup of the instance is in progress |
| D14 | 29 | A member of `record.v1`'s `data` for the invocation that a read or a provider call belongs to, its name (the wire key `serving` is one candidate) and which ID each record puts where |
| D15 | 35 (R15) | How the engine tells its own machine in a machines provider's report, for example by its `kubernetes.io/hostname` label; and the other parts of R15 |
| D16 | 37 (R14) | The layers of POOL-016, PLC-007 and PLC-013 |
| D17 | 38 | How a run on another machine learns where the engine's server is and gets what DL13's mechanism needs: inputs of `run.v1`, the runtime provider's own platform, or no placement off `local` until decisions 26b and 26c |
| D18 | 40 (R20) | Whether a provider call declares Conduit's extension, what a provider whose discovery result lacks it reports, whether a provider type may have a `plain` implementation, and how a provider call's result is read |
| D19 | 41 (R18) | How often the engine asks `inspect-run.v1`, and a run of another provider that returns no `endpoint` |
| D20 | 42 | How a runtime provider other than the engine's own runner gets an in-package program: the package's address, version and digest in `run.v1`, or a rule that an item with a relative `command` runs only on the engine's own runner |
