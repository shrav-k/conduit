# Ruling 41 — log of group G2 (placement and providers)

Written 2026-11-06 by the editor of group G2, under the
[work order](PLAN.md#g2-placement-and-providers) and the rules of
[CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#rules-for-every-edit). G2's files are
[spec/promises/placement.yaml](../../../spec/promises/placement.yaml),
[spec/promises/pools.yaml](../../../spec/promises/pools.yaml),
[spec/promises/first-start.yaml](../../../spec/promises/first-start.yaml) and the new
[spec/promises/providers.yaml](../../../spec/promises/providers.yaml). No other file was edited.
The names of the provider capabilities and operations (`runtime-provider.v1`, `run.v1` and the
rest) are the planner's (P1), kept as the work order gives them; the vocabulary audit that
[ruling 41](README.md) orders proposes names.

## Counts

| File | Before | Removed | Rewritten | Added | After |
| --- | --- | --- | --- | --- | --- |
| `spec/promises/placement.yaml` | 20 | 5 | 8 | 4 | 19 |
| `spec/promises/pools.yaml` | 15 | 0 | 5 | 0 | 15 |
| `spec/promises/first-start.yaml` | 5 | 0 | 3 | 0 | 5 |
| `spec/promises/providers.yaml` (new) | 0 | 0 | 0 | 18 | 18 |
| G2 | 40 | 5 | 16 | 22 | 57 |

These are the plan's counts for G2 (5, 16 and 22). Two layers changed in place, as the plan
says: PLC-002 `language` → `convention`, PLC-013 `convention` → `language`.

Checks. The validation command of CONSOLIDATION.md passes: 425 promises at the last run, the
rest in files that other editors were changing at the same time. In G2's four files every
promise has exactly the seven fields, folded `>-` text indented four spaces, and no line is
longer than 96 characters. No promise in them cites an ID that the plan removes, and every ID
they cite exists now or is one the plan adds (TPL-017 to TPL-019, LIF-035, IFC-041, the SEC
IDs). `python3 tools/check_docs.py` reports no error; `python3 tools/set_names.py --check`
reports three errors, none in G2's files (`spec/core/README.md`, `spec/definitions.md`).

Each request that a G2 promise leaves undispatched names one reason: PLC-011 conflict, PLC-019
and PLC-023 unavailable, POOL-013 conflict, POOL-015 refused, POOL-017 invalid, PRV-007 and
PRV-008 unavailable (runtime provider not ready), PRV-017 unavailable under PKG-024. A document
that a G2 promise rejects reports invalid under OBJ-023.

## Promises removed

| ID | Decision | Where its content went |
| --- | --- | --- |
| PLC-003 | 8 | Nothing: the default location goes; placement by the machines provider (PLC-002, PRV-015) |
| PLC-004 | 8 | Nothing: the default location goes; an instance the provider cannot place is PRV-015 |
| PLC-016 | 8 | Nothing: `set-default-location.v1` and `system-settings.v1` go |
| PLC-021 | 8 | Nothing: facts and `set-facts.v1` go; a location's labels are PLC-026 |
| PLC-022 | 8 | Nothing: the control it governed goes |

Each is named in the head comment of `placement.yaml`, with ruling 41.

## Promises rewritten

| ID | Decision | What changed |
| --- | --- | --- |
| PLC-002 | 8c | Layer → `convention`. With no placement given (document, or an entry's input), the placement is the location that stands for the machine the machines provider names under PRV-015; `local` with no machines provider. Providers of PRV-002 excepted. |
| PLC-007 | 8a | A type's or a selected item's `placement` is a label selector; the location's reported labels must hold each label with its value. Union of the type's and every selected item's selector; decided once, from the labels in the location's status. |
| PLC-008 | 8, 8d | `when` "A location exists". A Resource of `location.v1`, owned by `system`, its own placement, empty configuration; the engine makes it (`local` at first start, others under PLC-026), no document does (FST-006). Labels in its status; no facts, no control. |
| PLC-011 | 17, 8e | Instances "that exist" count; "unregistered" → "removed or unregistered"; `then` also names a machine reported gone (PLC-024) as a way a location with instances is retired. |
| PLC-012 | 8 | "a location or a host" → "a location or a machine". |
| PLC-013 | 8b | Layer → `language`. `shared-host.v1`: the Conduit is not instantiated where the `kubernetes.io/hostname` labels of the two ends' locations differ or either is absent; decided once. |
| PLC-017 | 8e, 17 | Observable: an instance whose shutdown is unknown is removed by unregistering it (LIF-017), one confirmed is removed under LIF-035; a machine reported gone has the same effect (PLC-024); the ACC-012 analogy dropped. |
| PLC-020 | 8 | "on the host of the Resource's location" → "on the machine of the Resource's location". |
| POOL-003 | 5 | "a declared Conduit and a component" → "a declared Conduit". |
| POOL-009 | 31b, P14 | An input may also have an entry's `runtime` as a destination; an entry writes neither `placement` nor `runtime`; with no runtime supplied, the engine's own runner. |
| POOL-013 | 1 | Each entry's setup is a part of the Pool's setup, under the entry's name, begun once the parts for the entries it refers to have succeeded; a ready Resource counts as succeeded with nothing dispatched; an entry with no Resource that is not retired reports conflict; optional entries as GRA-003 had them. |
| POOL-015 | 1 | "step" → "part" throughout. |
| POOL-016 | 8c | The entry's Resource is placed under PLC-002 by the machines provider (PRV-015); default-location sentences dropped; a replacement is placed by the provider when it is made. |
| FST-001 | 8d, 10 | A name is fixed (OBJ-011); `local` is the location of the machine the engine runs on (PLC-026), with the labels of PRV-014. DL6 sentence dropped. |
| FST-002 | 10 | The rename sentence dropped. |
| FST-006 | 8d | `when` adds the template `location`; the engine alone makes locations (PLC-026). |

Unchanged, as the plan says: PLC-001, PLC-005, PLC-006, PLC-009, PLC-015, PLC-018, PLC-019;
POOL-001, POOL-002, POOL-004 to POOL-007, POOL-011, POOL-012, POOL-014, POOL-017; FST-003,
FST-005.

## Promises added

| ID | Layer, level | The rule its `then` states | Decisions |
| --- | --- | --- | --- |
| PLC-023 | convention, must | Where a machine is reported unreachable, a request, or a hook, that would be dispatched to the implementation of an instance placed there is not dispatched and reports unavailable; nothing is retired. | 8e (replaces DL5's rule) |
| PLC-024 | convention, must | Where a machine is reported gone, its location and every instance placed on it are retired as PLC-017 retires them, the location counts as declared lost, and status and record say the provider reported it gone. | 8e |
| PLC-025 | language, must | A template whose selected item's `placement` names a label that its type's `placement` names with another value is rejected. | 8a |
| PLC-026 | convention, must | For each machine reported reachable or unreachable for which no location stands, the engine registers and sets up a location from `location`, owned by `system`, named as P7 gives; `local` stands for the engine's own machine; labels are the last reported, in its status. | 8d, P7, C58 |
| PRV-001 | language, must | A Type that offers a provider capability is of the family Resource and offers no other; otherwise rejected at installation or submission. | 31a |
| PRV-002 | convention, must | A runtime, machines or package-source provider is placed on `local` and run by the engine's own runner; a document or input giving it another placement or a `runtime` is rejected. | 31a, P6 |
| PRV-003 | convention, must | Where no provider of a kind serves, the engine's own runner, machine (`local`) or `https:`/`file:` fetching serves and keeps this file's promises. | 31a, 8c, 21 |
| PRV-004 | language, must | A Conduit whose capability is one of the three, or that lists one of their operations, is not instantiated. | P5 |
| PRV-005 | convention, must | A provider call is not a request: no Conduit, no access decision, no restriction; it carries the provider's `instance`, `config` and `revision`, `operation`, and `invocation` and `caller` where it has them; it is recorded. | 31a, 4c, P5, 1b |
| PRV-006 | language, must | `runtime` is a reference to a Resource whose type offers `runtime-provider.v1`, held in resolved form for life; anything else is rejected; none means the engine's own runner. | 31b |
| PRV-007 | convention, must | Where an item that is run is selected and the provider does not report the run running, the engine asks for it with `run.v1`: instance ID, item, `arguments`, `environment`, `secrets` apart, and the machine's hostname and labels; it reaches the server at `endpoint`, or over stdio on its own runner. | 2, 2a, 2c, 25d, 1c, P4, P18 |
| PRV-008 | convention, must | In a retirement the engine asks the runtime provider to stop the run with `stop.v1`, naming the instance's ID; stopping what does not run succeeds. | 2a |
| PRV-009 | convention, must | Inspection reports the condition `Running`, `True`, `False` or `Unknown` as `inspect-run.v1` last gave `running`, `stopped` or `unknown`, with its time and origin. | 2a, 24b, P13 |
| PRV-010 | convention, may | A runtime provider may restart a run that stopped, under its own setting, with the values it was given. | 2b, 2c |
| PRV-011 | convention, must | A runtime provider delivers each value of `secrets` as the variable or argument the item binds, puts it nowhere else, and states in its contract where it keeps it. | 2c |
| PRV-012 | convention, must | A `package` item whose `registryType` the provider's contract does not say it runs ends the run failed, and nothing is run. | 25a |
| PRV-013 | convention, must | A machines or package-source provider is not instantiated unless owned by `system`, and while another machines provider, or another claiming one of its schemes, is not retired. | 31b, P6 |
| PRV-014 | convention, must | `list-machines.v1` gives each machine's `hostname`, `labels` (with `kubernetes.io/hostname`, `kubernetes.io/os`, `kubernetes.io/arch`) and `state`. | 8, 8e, C58 |
| PRV-015 | convention, must | With no placement given, the engine asks `place.v1` with the selector of PLC-007; the placement is the location that stands for the machine named; with none named, or a machine with no location, not instantiated. | 8c |
| PRV-016 | language, must | A Type that offers `package-source.v1` has a `schema` that requires `schemes`, a list of at least one URI scheme; otherwise rejected. | 21, P6 |
| PRV-017 | convention, must | A `fetch` whose scheme a package-source provider claims is fetched with its `fetch-package.v1`; `https` or `file` unclaimed by the engine; either way checked under PKG-026. | 21, 31b, P17 |
| PRV-018 | convention, must not | Which runtime provider ran an instance, or on which machine, is taken as establishing which instance sent a request. | 26, R40 3e |

`providers.yaml` by layer: `language` 4 (PRV-001, PRV-004, PRV-006, PRV-016) and `convention`
14, the layers the plan gives; by level: `must` 16, `may` 1 (PRV-010), `must not` 1 (PRV-018).

## Choices made where the plan leaves wording open

1. **Entries and documents alike.** PLC-002, PRV-006 and PRV-015 speak of an Agent or a
   Resource "given" no placement or a `runtime`, by its document or, for an entry's Resource,
   by an input of the Pool's template, so that POOL-009 and POOL-016 can point at them. The
   plan's wording named instance documents only.
2. **"Stands for".** A location stands for the machine it was registered for, which its status
   names by its `kubernetes.io/hostname` label, until it is retired; a retired location stands
   for none (PLC-026). So a machine reported gone that comes back, or one whose location an
   actor retired, is given a new location at the next report (PLC-024, PLC-026): with a
   forward-only lifecycle (LIF-006) the only other reading would be that such a machine never
   gets a location again.
3. **The name of a reported machine's location (P7).** "The first label of its hostname" is
   written "the part of the machine's hostname before the first dot", so that "label" keeps
   the one meaning of section 1. "Free" is written as "a valid name under OBJ-001 that no
   Resource that exists has under OBJ-002".
4. **Who registers a location.** The engine itself, as at first start: no actor asks, no
   request is made, and the registration and its lifecycle states are recorded under REC-001
   (PLC-026). A machine reported gone gets no location.
5. **Gone counts as declared lost.** PLC-024 says the location "counts from then on as
   declared lost", so LIF-017, CON-008 and LIF-024 apply without a second clause each, as the
   term table's row for "declare lost" says. G1's LIF-017 and G3's CON-008 already read so.
6. **PLC-011's `then`** now names the report of a gone machine beside `declare-lost.v1` as a
   way a location with instances is retired; otherwise "only the declare-lost operation"
   would contradict PLC-024.
7. **PLC-017** loses "as a request to close a closed Conduit does under ACC-012": G3 rewrites
   ACC-012 so that such a request reports invalid.
8. **PLC-007** with several selected items: the selector to match is the union of the type's
   and every selected item's (the plan said "the item's").
9. **PLC-013** points an end with no placement at CON-006, which already rejects it, and says
   that the engine applies `shared-host.v1` itself and never sends it (C20).
10. **PLC-023** covers a setup or retire hook as well as a request, since a hook is dispatched
    to the instance's implementation through no Conduit.
11. **PLC-025** reads "selects" as TPL-019 does: by the `implementation` map or by default.
12. **PLC-008** says that `location.v1` offers no capability and lists no control, as section 4
    of the plan makes it, and that `local` is made at first start and the others under PLC-026.
13. **POOL-013** carries GRA-002's rule (a part begins only once those for the entries it
    refers to have succeeded) and GRA-012's (order from the references alone, not from the
    order of writing); a part with no Resource that is not retired "is not dispatched and
    reports conflict".
14. **PRV-002** also rejects an input of a Pool's template that supplies a placement or a
    `runtime` for a provider's entry.
15. **PRV-003**: the built-in implementations are part of the engine; no package provides
    them, no Resource stands for them, no Conduit reaches them.
16. **PRV-004 also covers a Conduit that lists an operation of the three capabilities**, not
    only one whose capability is one of them. Without it, a capability of one's own that lists
    `run.v1`, offered by a runtime provider's type, would open the hole that P5 closes. The
    coordinator may strike this and keep the plan's narrower rule.
17. **PRV-005 carries the provider's own `instance`, `config` and `revision`**, beside the plan's
    `operation`, `invocation` and `caller`. Under 31b a provider is configured as it needs (a
    Docker context), and under 1b configuration travels in `_meta`; with the plan's list a
    provider could not learn its own configuration. The coordinator may strike this.
18. **PRV-005, calls outside an invocation.** `place.v1` and `fetch-package.v1` are made while
    the engine decides a request, before it is dispatched (PRV-015 rejects the document and
    PKG-024 reports unavailable, both reasons and not outcomes), so they carry the request's
    `caller` and no `invocation`. `list-machines.v1`, and `inspect-run.v1` when the engine asks
    of its own accord for PRV-009, carry neither. The plan's "made within the invocation it
    serves" holds for `run.v1`, `stop.v1` and `inspect-run.v1` in a setup or retirement.
19. **PRV-007** learns whether the run is running by asking `inspect-run.v1` at that part, and
    gives in `location` the hostname and labels of the machine that the location stands for.
    It covers the run asked for to serve a retire hook (TPL-018) as well as a setup's.
20. **PRV-008** says a stop is not dispatched and reports unavailable where the runtime provider
    is not ready, as PRV-007 does for the run.
21. **PRV-009** maps `running`, `stopped` and `unknown` to the condition's `status` `True`,
    `False` and `Unknown`; the report's time is `lastHeartbeatTime` and its `origin` the
    runtime provider.
22. **PRV-010** keeps the level `may`; what the engine must not do because of a restart is in
    its observable, citing SEC-014.
23. **PRV-012**: "its contract says it runs" is the prose of the provider's contract; no key of
    the Type contract lists registry types.
24. **PRV-013** reports invalid (a rejected document under OBJ-023), not conflict.
25. **PRV-015**: the provider names a machine for which a location already stands; one it names
    with no location makes the instance not instantiated, so that deciding a request registers
    no location. Alternative: register the location first.
26. **PRV-016** compares a scheme without regard to case, as RFC 3986 says of schemes.
27. **PRV-017**: a package-source provider that is not ready makes the request report
    unavailable under PKG-024, because fetching comes before dispatch.
28. **Words that go** are avoided in new text: "host" (PRV-011 says "the machine that the engine
    runs on"; PRV-017 "server"), "policy" (PRV-010 says "its own setting for restarts"),
    "a handle is held" and "declaration" (a word the promises do not use).

### Terms to note for the definitions

- **stand for** (a location for a machine): PLC-026; used in PLC-002, PLC-007, PLC-008,
  PLC-013, PLC-023, PLC-024, PRV-003, PRV-007, PRV-015.
- **item that is run**: an implementation item reached by `command` or `package` (PRV-007,
  TPL-019); one reached by `remote` is connected to.
- **the engine's own runner, the engine's own machine**: the built-in implementations of
  PRV-003.
- **claim** (a scheme): a package-source provider claims the schemes its configuration lists
  under `schemes` (PRV-016).
- **counts as declared lost**: a location whose machine was reported gone (PLC-024).
- `user` appears in PRV-001, PRV-003 and PRV-012 for the person who picks providers, as in
  ruling 40; the vocabulary audit may prefer another word.

## What outside G2's files cites a promise G2 removed (for the integrator)

At the time of writing; the promise files of the other groups already cite none of them
(LIF-009, OBJ-013 and OPS-029 were fixed by G1, G6 and G4).

| Removed | Still cited by |
| --- | --- |
| PLC-003 | `spec/core/templates.yaml` (head comment, `system`), `spec/core/types.yaml` (head comment), `spec/core/README.md`, `spec/definitions.md` (default location) |
| PLC-004 | `spec/schemas/instances.yaml` (head comment), `spec/definitions.md` (placement) |
| PLC-016 | `spec/core/capabilities.yaml`, `spec/core/operations.yaml`, `spec/core/types.yaml`, `spec/core/README.md` (four places), `spec/definitions.md` (default location) |
| PLC-021 | `spec/core/capabilities.yaml`, `spec/core/operations.yaml`, `spec/core/types.yaml`, `spec/core/README.md` (three places), `spec/definitions.md` (location fact, `set-facts.v1`), `spec/schemas/examples/valid/first-start/first-start.yaml` |
| PLC-022 | none |

Also for the core-and-schemas and docs editors: `spec/core/requirements.yaml` describes
`shared-host.v1` by "stored facts" and "host" and cites CON-030 and CTR-010;
`spec/core/templates.yaml` and `spec/core/types.yaml` still describe `facts`, `localPaths` and
`location-settings.v1`; `spec/schemas/instances.yaml` says a location's document states no
`placement` (PLC-008 now says no document makes a location). In `spec/definitions.md`, the
rows location, location type, placement, host, location fact, `set-facts.v1`, default location,
requirement on placement, shared-host requirement, `system` (its default-location setting),
first-start object ("renamed") and the reason unavailable ("the shared-host requirement does
not hold", in "How a request ends") follow the old PLC-002 to PLC-022; PLC-013 is now decided
when a Conduit is instantiated, and reports invalid through the rejected document, never
unavailable. `docs/OVERVIEW.md` (around its lines on placement and the worked example's
shared-host requirement) speaks of hosts.

## Found outside the plan, left unchanged

- **PLC-005**'s observable says a template may refer to a location "as the `target` of a
  declared Conduit". With `location.v1` offering no capability (section 4 of the plan), no
  Conduit to a location can be instantiated (CON-021), so that example no longer holds, and
  "among the `targets` of a bound" covers nothing a Conduit could reach. The plan lists
  PLC-005 as unchanged; the coordinator decides whether to drop the two examples.
- **POOL-012**'s observable names `owner` and `placement` as keys an entry may not carry, and
  not `runtime`, which OBJ-014 now also rejects. It is still true as written.

## Still needs a decision

1. **Layers that now read what a provider reports.** PLC-007 and PLC-013 are `language`
   (PLC-013 by the plan's change), and POOL-016 stays `language`, but each now depends on the
   labels that the machines provider reported, held in a location's status, or on its
   `place.v1`; "resolving ... runs nothing". The plan moved PLC-002 to `convention` for this
   reason. Should PLC-007, PLC-013 and POOL-016 be `convention` too?
2. **A machine that a later report omits**, without reporting it gone: unreachable, gone, or
   nothing? No promise says.
3. **A machines provider that is retired or replaced.** What becomes of the locations, and the
   instances on them, of machines that it reported and that the next provider (or the
   engine's own machine) does not report?
4. **`local` reported gone.** PLC-024 retires any location whose machine is reported gone; for
   `local` that would retire `management`, `access` and `root`, which LIF-022 protects from
   requests. Exempt `local`, or accept it?
5. **An actor retiring a location of a machine still reported.** PLC-011 allows it once no
   instance is placed there; PLC-026 then registers a new location for that machine at the
   next report. Should the engine's own locations be retired only by a report or by
   `declare-lost.v1`?
6. **Non-string values for `run.v1`.** `arguments`, `environment` and `secrets` are maps of
   strings (plan section 4), but a configuration place that an item points to may hold a
   number, a boolean or a map. How it becomes a string (its JSON text, or a rejection under
   MAN-017) is not said.
7. **How often the engine asks `inspect-run.v1`** for PRV-009's "last gave": no item of the
   implementation-defined list names it (DL5 is the machines provider's report).
8. **Two selected items whose `placement` name one label with different values.** PLC-025
   rejects only a clash between the type and an item; such a template is accepted, and every
   instance of it then fails PLC-007. Reject it at registration too?
9. **A `default` for an entry's `runtime`.** PLC-005 forbids a template to give a placement, as
   an input's `default` among other ways; nothing says whether an input whose destination is
   an entry's `runtime` may state a `default`.
10. **A `runtime` that names a retired runtime provider.** PLC-018 rejects a retired location as
    a placement; nothing rejects a retired runtime provider, whose instances' runs would then
    report unavailable at every setup.
11. **The run of an instance on an unreachable machine.** PLC-023 stops requests and hooks to
    the instance; whether the engine asks the runtime provider for its run or its stop
    meanwhile is not said.
