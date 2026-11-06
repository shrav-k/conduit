# Ruling 41 — log of group G3 (access)

Written 2026-11-06 by the editor of group G3, under the [work order](PLAN.md#g3-access) and the
rules of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#rules-for-every-edit). G3's files are
[spec/conventions/access.yaml](../../../spec/conventions/access.yaml) and
[spec/promises/conduits.yaml](../../../spec/promises/conduits.yaml). No other file under `spec/`
was edited. Names are those of the review and the work order (`reveal-secret.v1`,
`setup-hook.v1`, `retire-hook.v1`, "part", "provider call" and the rest); the vocabulary audit
that [ruling 41](README.md) orders renames them.

## Counts

| File | Before | Removed | Rewritten | Added | After |
| --- | --- | --- | --- | --- | --- |
| `spec/conventions/access.yaml` | 34 | 5 | 20 | 0 | 29 |
| `spec/promises/conduits.yaml` | 27 | 5 | 12 | 0 | 22 |
| Both | 61 | 10 | 32 | 0 | 51 |

These match the work order's counts for G3 (10 removed, 32 rewritten, none added) and its
"Unchanged" lists exactly: ACC-002, ACC-008, ACC-014, ACC-016, ACC-022, ACC-023, ACC-024,
ACC-028, ACC-032; CON-006, CON-011 to CON-014, CON-017, CON-021, CON-023, CON-025, CON-027 are
byte for byte as they were. No layer or level changed.

The validation command of CONSOLIDATION.md passes (424 promises at the last run, the rest in
files other editors were changing at the same time). Every line of both files is at most 96
characters, every promise has exactly the seven fields with folded `>-` text, `git diff --check`
reports nothing for them, and `python3 tools/set_names.py --check` reports no error in them.
Every ID that a promise of these files cites exists now or is an ID the work order fixes
(LIF-035, MAN-015, PLC-023, PLC-024, PRV-004 to PRV-006, PRV-008, PRV-010, SEC-007, SEC-008,
SEC-010, SEC-011, TPL-017, TPL-018); none cites a removed ID. Each head comment gained the line
of rule 7, naming the removed IDs, where their content went and the IDs changed.

## Promises removed

| ID | Decision | Where it went |
| --- | --- | --- |
| ACC-026 | 3a | Policies go. Nothing replaces it; to narrow access a Conduit is closed and a narrower one opened (ACC-011). |
| ACC-031 | 3a | As ACC-026. `apply-policy.v1` and `remove-policy.v1` go with it. |
| ACC-035 | 3a | As ACC-026. |
| ACC-042 | 3a | As ACC-026. |
| ACC-043 | 3a | As ACC-026. |
| CON-003 | 5 | Composite Conduit templates go; a template declares two Conduits. Nothing replaces it. |
| CON-019 | 5 | As CON-003. |
| CON-029 | 5 | As CON-003. |
| CON-026 | 10 | Rename goes; a name is fixed (OBJ-011). Nothing replaces it. |
| CON-030 | C20 | `narrows` goes: a requirement of the core package is applied by the engine, any other is a filter (CON-018). |

The section comments that headed only removed promises were removed with them: the one over
ACC-042 and ACC-043, and those over CON-029 and over CON-030.

## Promises rewritten

| ID | Decisions | What changed |
| --- | --- | --- |
| ACC-001 | 6c, 4b | Observable: the retrieval sentence goes; a request for `reveal-secret.v1` that an actor makes, or any instance as an onward request, reports refused even where a Conduit lists it (SEC-007); only the engine's read is dispatched (SEC-010). |
| ACC-003 | 4b | `then`: such a request for `reveal-secret.v1` is never dispatched through a Conduit (SEC-007). Observable: the engine presents no tool for it (IFC-012). |
| ACC-005 | 1, 5, S4.4 | `when`: a setup opens a declared Conduit, under CON-004. "Step" becomes "part"; every composite sentence goes. Observable adds: a declared Conduit that lists `reveal-secret.v1` is opened only where the root Interface sent the setup (SEC-008). |
| ACC-006 | S4.4, 17 | `then` adds SEC-008 as a further condition. Observable: "the part of a setup counts as succeeded"; a request to open a closed Conduit names one that no longer exists (LIF-035) and reports invalid (OPS-029), replacing conflict under LIF-015. |
| ACC-009 | 3a | `then`: limited by its own Conduit's operations and restrictions and, for an onward request, those carried under ACC-017; "every policy" goes. Observable drops the policy sentences. |
| ACC-011 | 17, 3a | Observable adds: the closed Conduit is removed (LIF-035) and its tools are no longer presented (IFC-013); narrowing names "fewer operations or further filters". |
| ACC-012 | 17 | `then` drops the sentence on closing a closed Conduit. Observable: it no longer exists, so a request to close it reports invalid (OPS-029); a repeat of the request that closed it is answered as OPS-009 and OPS-032 state. |
| ACC-015 | 3a, 7, 9, 1, C20 | `when`: a request or a hook carries a filter under ACC-017 or ACC-018 that its implementation does not state in `supports`. `then` adds what a hook's part reports. Observable keeps the restriction named, `plain`, and the core-operation case (`engine` states no `supports`, MAN-012); every policy, composition, re-export and graph sentence goes, and "of a location" goes (a location offers no operation now). |
| ACC-017 | 3a, 1, 4c | `then`: the policies go; filters are carried from the served invocation or, for a hook, from its `set-up.v1` or `retire.v1`; the chain is defined with hooks. Observable: policy sentences go; "a hook carries them in the same way, under ACC-018"; a secret read carries none of them (SEC-011). |
| ACC-018 | 1 | Rewritten for hooks: `when` is the dispatch of a setup or retire hook (TPL-017, TPL-018); `then`: dispatched to the instance's own implementation through no Conduit, carrying the restrictions of its `set-up.v1` or `retire.v1`, its own requests onward requests (ACC-016). Every step, policy, composite and graph sentence goes. |
| ACC-019 | 2, 6c | `then` drops the `retrieve-result.v1` exception. Observable drops the retrieval and OBS-007 sentences, and adds that a read of a secret is made only within the setup or retirement that asks for a run or a connection (SEC-011) and none on a provider's own restart (PRV-010). |
| ACC-027 | 2d | `then`: enforced only where the consumer's runtime provider confines it, as its contract states; mediated for every restriction in this version. Observable: "isolation plugin" goes; confinement is opted into by `runtime` (PRV-006). |
| ACC-029 | 3a, 7, 9, 11, C20 | `then`: the bound is a restriction the engine applies itself; "an actor that a policy names" goes. Observable: never sent (CON-018); `operations` reaches a contract of an installed package or of a submitted project that states the address (SUB-008, MAN-015); the unpublished-package, re-export and CTR-011 sentences go. |
| ACC-030 | 28 | Observable adds what delegation trusts: unfiltered access within the bound, and for `register.v1` or `replace.v1` on `management` any object authorized against `management` alone. |
| ACC-033 | 3a, 10, 6c, 8, 17, P9 | `when`: rename and policies go; unregister names a template or an instance whose location was declared lost. `then`: renaming and the policy clause go; `register-replacement.v1` is authorized as registering what it makes. Observable: a management verb is an operation of the three capabilities (CON-024); retrieval, location and `system` controls go; a hook is reconciled as an operation of its instance; "unregistered" becomes "no longer exists". |
| ACC-034 | 6a | `then`: the invocation runs to its end and its result, with its outputs, is delivered; onward requests after the close are refused. Observable: every withheld and OPS-017 sentence goes; a later repeat or `tasks/get` is decided anew, and through the closed Conduit reports refused (ACC-036, IFC-016). |
| ACC-036 | 3a, R40 3f | `then`: "from the Conduits as they stand"; policy clauses go from the observable. The rule itself is kept (one control plane in v0). |
| ACC-037 | 3a | `when`: "the state of the Conduits that the decision needs". |
| ACC-038 | 14, 11, 1, C20 | `then`: for a filter, the implementations the target selected; for a core requirement, the engine; a submitted project's implementation is `address#name`. Observable: the consumer, graph and `<digest>#<name>` sentences go; the engine is named for the shared-host requirement (PLC-013) and the bound (ACC-029). |
| ACC-039 | 1, 2, 4c | `then`: permitted only for the consumer's retire hook, an onward request that serves it, or the read of a secret for the run the retirement asks for (TPL-018, SEC-011). Observable: step and runtime sentences rewritten in those terms; the stop of a run is a provider call (PRV-004, PRV-005, PRV-008). |
| CON-001 | 5 | Observable drops the composite clause. |
| CON-002 | 5 | `then` drops the component clause. |
| CON-004 | 1, 5, 17, GRA-022, GRA-003 | `when`: an instance that declares Conduits is set up. `then`: each declared Conduit's opening is a part of the setup, under its name; registered → `open.v1` under ACC-005; open → nothing dispatched, counts as succeeded; no Conduit that is not retired → conflict; setup instantiates none. Observable: from GRA-003, a required part keeps the setup from succeeding and an optional one is reported not opened; the parts come before the run and the hook (TPL-012); every composite sentence goes. |
| CON-007 | 8e | `when`: "the machine of its location being reported unreachable, under PLC-023". |
| CON-008 | 2, 17, 1, 8e | `then` adds: a Conduit an end of which is placed on a machine reported gone is closed at once (PLC-024); a closed Conduit is removed (LIF-035). Observable: the runtime sentence goes; the retire hook makes onward requests; inspection of a closed Conduit gives way to its removal. |
| CON-016 | 5 | `then`: `inputs`, `capability`, `operations` and `requirements` and no other key; `components` and the composite qualifier go. Observable drops the `components` sentence. |
| CON-018 | 7, C20 | `then` adds the bound and the C20 rule. Observable: a requirement is applied only to a Conduit (CTR-005); the shared-host requirement and the bound are the engine's (PLC-013, ACC-029); a filter is sent under IFC-018. Composition and CON-030 sentences go. |
| CON-020 | 5 | Observable drops the component clauses. |
| CON-022 | 14, 7, 9, 1, 25b, C20 | `when`: an implementation the target selected for a listed operation does not state the filter's requirement. `then`: `command`, `package` or `remote` support by stating it, matched by address and export name. Observable: `obliges`, consumer, composition, re-export and graph sentences go; `plain` and NAM-024 kept; `engine` states none (MAN-012). |
| CON-024 | 6c, 8 | Observable: a management verb is an operation of `object-management.v1`, `access-control.v1` or `pool-management.v1`; the controls and retrieval-tool clause goes. |
| CON-028 | 7 | Observable drops the constituent and CTR-011 sentence. |
| CON-031 | 17 | Observable: a Conduit whose target is retired is closed and removed, so a request to open it reports invalid (OPS-029); "the step of a setup" becomes "the part of a setup". |

## Promises added

None, as the work order says.

## Choices made

Wording the work order left open:

1. **A bound is a restriction, not a filter.** Section 1 of the work order now defines a filter
   as a requirement other than the core package's, and a restriction as "a filter or a bound".
   ACC-029 said "The bound is a filter"; it now says "The bound is a restriction, which the
   engine applies itself". ACC-009 said "another Conduit that carries a filter", which under the
   old terms covered a bound; it now says "a restriction: a filter or a bound", so that its rule
   is unchanged.
2. **"Part" is kept for its new meaning.** Where a rewritten promise used "part" in its ordinary
   sense, it was reworded: ACC-006 "not opened for the part that was within the bound" → "not
   opened, not even with only the operations that the bound names"; ACC-011 "No part of it
   remains in force" → "None of it remains in force"; CON-022 "plays no part" → "does not
   count". "Policy" was kept out of ACC-019 too: a provider "restarts a run of its own accord"
   (PRV-010 itself says "restart policy").
3. **The rule in `then`.** Where the work order put a rule only in an observable, it was also
   stated in `then`: CON-018 states the C20 rule there; ACC-003 states the `reveal-secret.v1`
   exception there; ACC-015 states what a hook's part reports.
4. **A machine reported gone (CON-008).** Decision 8e gives a gone machine PLC-017's effect, and
   PLC-024 now says the location then counts as declared lost. Without a clause, a Conduit whose
   consumer is on a gone machine would never close, since that consumer's shutdown stays unknown
   (LIF-024). CON-008's `then` names "placed on a machine that is reported gone, under PLC-024"
   beside "declared lost". The integrator may drop the clause if PLC-024's wording is taken to
   cover it.
5. **Inspecting a closed Conduit (CON-008).** Its observable said that inspecting a closed
   Conduit reports which event closed it. A closed Conduit is now removed at once (LIF-035), so
   that sentence became: a request that names it reports invalid (OPS-029), and its records are
   kept (LIF-019). See "Still needs a decision", item 1.
6. **Hooks in ACC-015 and ACC-018.** A hook is not a request (section 1: three kinds of request),
   so ACC-015's `when` says "a request or a hook", and a hook that cannot be dispatched has "its
   part" report the reason. ACC-018's `then` puts the hook's own onward requests under ACC-016.
7. **Reconciling a hook (ACC-033).** "A step that invokes the implementation of its own instance
   was dispatched to that instance" became "a setup hook or a retire hook is dispatched to its own
   instance, under ACC-018, and is reconciled in the same way", that is, against the Pool that
   owns a Resource and against `management` otherwise.
8. **Management verb (ACC-033, CON-024).** Defined by the three capabilities, since the core
   package now exports provider and hook operations that are not management verbs.
9. **ACC-015's core-operation sentence** names `management`, `access` and a Pool, not a location:
   `location.v1` offers no capability now. `engine` is described as implementing the management
   verbs, as the core manifest of section 4 has it.
10. **ACC-019** gained one sentence on the engine's read of a secret (only within the setup or
    retirement that asks for a run or a connection, SEC-011; none on a provider's own restart,
    PRV-010), so that the promise that no request follows an ended invocation covers the third
    kind of request.
11. **ACC-030's trust statement** (decision 28) names the exceptions that still hold for a
    delegate: the reserved operations of ACC-032 and `reveal-secret.v1` under SEC-008.
12. **ACC-034** says the delivery of a result is not decided again, and that a later repeat or a
    `tasks/get` is decided anew; through the closed Conduit it reports refused, as the work order
    says. See "Still needs a decision", item 2.
13. **ACC-039** permits the read of a secret only "for a run that the retirement asks for to serve
    that hook", as SEC-011 limits a retirement's reads to the run; it also cites PRV-004 for the
    stop of a run, which no Conduit permits.
14. **ACC-012**: a repeat of the request that closed a Conduit is answered "as OPS-009 and OPS-032
    state", since OPS-009 now answers only where the engine still keeps the outputs.
15. **CON-022** adds that no Conduit is instantiated that applies a filter and lists an operation
    for which its target selected `engine`, since `engine` states no `supports` (MAN-012). This
    was already so when `engine` supported `bound.v1` alone; the sentence makes it visible.

## Terms, for the definitions

- **restriction**: "a filter or a bound" (ACC-009, ACC-029). The row for **bound** must no
  longer say "It is a filter"; it is a restriction that the engine applies itself (ACC-029,
  CON-018), and it no longer limits which policies an actor may apply.
- **filter**: a requirement other than the core package's, applied to a Conduit (CON-018).
- **chain**: "an onward request serves the invocation it names, and a hook serves the
  invocation it belongs to" (ACC-017).
- **hook**: used alone for "a setup hook or a retire hook" (ACC-015, ACC-017, ACC-018,
  ACC-034, ACC-039, CON-022). Not a request; it is dispatched, and its part reports a reason
  where it is not.
- **part**: a declared Conduit's opening is a part of a setup, under the declared Conduit's
  name (CON-004); a hook's part (ACC-015, ACC-018).
- **management verb**: an operation of `object-management.v1`, `access-control.v1` or
  `pool-management.v1` (CON-024, ACC-033).
- **qualified reference**: ACC-029 and ACC-038 use it, as the work order does, for
  `address#name` of a submitted project's contract or implementation, the address being the
  one its manifest states (decision 11). The definitions' row says "a package's address"; it
  should also cover a project's.
- **narrow (access)**: closing a Conduit and opening a narrower one, which lists fewer
  operations or applies further filters (ACC-011).
- **the engine's read of a secret**: the phrase used for the work order's "read (of a secret)"
  (ACC-001, ACC-003, ACC-017, ACC-019, ACC-039, CON-008).

## For the integrator

What cites a removed ID outside G3's files, as the tree stood at the last commit (other editors
may already have fixed some):

| Removed | Promises that cite it | Other files that cite it |
| --- | --- | --- |
| ACC-026 | — | `spec/core/README.md`, `spec/definitions.md`, `docs/OVERVIEW.md` |
| ACC-031 | LIF-001, IFC-018, CTR-005, NAM-019 | `spec/core/README.md`, `spec/core/capabilities.yaml`, `spec/core/operations.yaml`, `spec/core/requirements.yaml`, `spec/core/types.yaml`, `spec/schemas/README.md`, `spec/definitions.md`, `docs/OVERVIEW.md` |
| ACC-035 | — | `spec/core/README.md`, `spec/definitions.md`, `docs/OVERVIEW.md` |
| ACC-042 | SCH-009 | `spec/core/README.md`, `spec/definitions.md` |
| ACC-043 | — | `spec/core/README.md` |
| CON-003 | LIF-009, INS-016, OBJ-003 | `spec/implementation-defined.yaml` (DL15), `spec/schemas/examples/valid/project/instances.yaml`, `spec/definitions.md` |
| CON-019 | — | `spec/core/README.md`, `spec/schemas/README.md`, `spec/schemas/templates.yaml`, `spec/schemas/instances.yaml`, `spec/schemas/examples/valid/project/templates.yaml`, `spec/definitions.md` |
| CON-026 | IFC-014, OBJ-026 (removed by G6) | `spec/definitions.md` |
| CON-029 | — | `spec/schemas/README.md` |
| CON-030 | CTR-005, CTR-011 (removed by G6) | `spec/core/README.md`, `spec/core/requirements.yaml`, `spec/schemas/README.md`, `spec/definitions.md`, the example `spec/schemas/examples/invalid-by-rule/CON-030-requirement-that-does-not-narrow-on-a-conduit.yaml`, `tools/check_schemas.py` (rule `narrows-on-conduit`) |

Promises kept whose subject changed, so that what cites them should be read again: ACC-018
(steps → hooks), ACC-034 (a result is delivered, nothing is withheld), ACC-029 (a bound is a
restriction applied by the engine, never sent), CON-004 (now holds GRA-022's and GRA-003's rule
for declared Conduits, with no composite), CON-018 (holds the C20 rule), ACC-039 (no runtime
stop; the read of a secret).

Also for the coordinator, not the specification: ruling 41 records removing policies (3a) as a
candidate access feature to design for a later version, for the tracker.

## Still needs a decision

1. **What closed a Conduit, once it is removed.** Before, inspecting a closed Conduit reported
   which event closed it (an actor's close, its target retired, an end declared lost). A closed
   Conduit is now removed at once (LIF-035), so only its records remain. REC-001 records the
   change of lifecycle state, but REC-002's fields do not say which event caused a closing that
   no request made (CON-008). Whether the record of that change must name its cause is for the
   records promises (G4) or the coordinator; CON-008 now promises only that its records are
   kept.
2. **Refused or invalid through a removed Conduit.** Decision 17's cost line says "a closed
   Conduit is removed when it closes, so a request through it reports invalid rather than
   refused". The work order keeps refused for a request *through* a closed Conduit (ACC-011,
   ACC-034, OPS-009, IFC-013: its tools are no longer presented and a call to one reports
   refused, IFC-012) and gives invalid only to a request that *names* it (ACC-006, ACC-012,
   CON-031, OPS-029). G3 followed the work order. If the review's cost line is to be read as
   ruled, ACC-011, ACC-034, IFC-012 and IFC-013 change together.
3. **A retirement that connects to a `remote` item.** ACC-039 permits a read of a secret during
   a retirement only for the run that TPL-018 asks for, as SEC-011 has it. A retire hook whose
   item is a `remote` that binds a header to a secret would need a connection, and SEC-013
   reads header secrets only for a setup. Whether a retirement may make that read is for G5 and
   G7; ACC-039 follows SEC-011 as it stands.
4. **Can a hook carry a filter?** ACC-015 and ACC-018 keep the general rule. In practice
   `set-up.v1` and `retire.v1` are dispatched to `engine`, which states no `supports`, so a
   setup or retirement that carries a filter is itself unavailable under ACC-015 and its hook is
   never reached. No decision is needed unless the coordinator wants the rule narrowed.
