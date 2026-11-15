# Plan and apply: the CLI reference

Not binding. The specification in spec/ alone binds.

This is the proposed CLI reference for spec batch 1, K3, with hand-written golden cases.
It describes tooling, not additional language promises or engine behavior. Its five approval
and command readings follow [ruling 76, points 1–5](../../proposals/ruling-76/README.md).
A builder follows the specification where this reference and it differ. No program has produced
these expected plans. The remaining cases are listed under
[Safety cases for 052a's convergence check](#safety-cases-for-052as-convergence-check).
What ships to the rerun's fresh author is the rerun preparation's decision, not this reference
([ruling 53, point 9](../../proposals/ruling-53/README.md)).

## Commands

The three columns follow the shell's [help table](../../../engine/crates/shell/src/help.rs).
Approval flags in this table are repeatable, each with one generated object name as operand.
`--store DIR` overrides the current engine context. There are no additional options.

| Usage | Example | Description |
| --- | --- | --- |
| `conduit plan [PROJECT_DIR] [--store DIR] [--reopen OBJECT] [--recreate OBJECT] [--prune OBJECT] [--replace OBJECT] [--adopt OBJECT] [--with-resources OBJECT] [--with-agents OBJECT]` | `conduit plan --replace shared` | Read the project and engine as root, and print the requests apply would send. |
| `conduit apply [PROJECT_DIR] [--store DIR] [--reopen OBJECT] [--recreate OBJECT] [--prune OBJECT] [--replace OBJECT] [--adopt OBJECT] [--with-resources OBJECT] [--with-agents OBJECT]` | `conduit apply --replace shared` | Compute, print and record the plan, then send its requests as root without a prompt. |

PROJECT_DIR defaults to the current directory. A bare `conduit apply` carries no author-chosen
value ([ruling 63, point 1](../../proposals/ruling-63/README.md)). `conduit serve` records the
engine it serves as the person's current engine context; commands use that context unless
`--store DIR` overrides it ([ruling 76, point 3](../../proposals/ruling-76/README.md)).
This is proposed help, not today's parser. No separate context-selection command is defined.
If no context or override is available, stop with status 2; context persistence and discovery
are implementation work for order 052a (O8 below).
`conduit plan` is available and never required: apply computes its own plan, with no prior plan
invocation or saved-plan prerequisite ([ruling 76, point 5](../../proposals/ruling-76/README.md)).
`--withhold` belongs to batch 2 ([ruling 61, B11](../../proposals/ruling-61/README.md)).
This reference does not define install, render, init, sign-in, answer, pending, list or fleet
commands, apart from the copy `conduit install` keeps in the package cache (O7). Installation
happens beforehand; plan reads installed release identities with `list.v1`, and each release's
contents from the cache.

Both commands are requests of a serving engine, `conduit serve`, as root, like
[today's author commands](../../../engine/README.md#change-a-serving-project). Neither starts
an engine or installs a package. Their wire documents are framed (SCT-001, SCT-003); they use
only [core operations](../../../spec/core/operations.yaml), adding no operation or promise
([ruling 53, point 4](../../proposals/ruling-53/README.md), SCT-015 and the head comment of
[sections.yaml](../../../spec/promises/sections.yaml)).

## Read, compare, print, keep, send

1. Read the manifest and its documents, translate sectioned documents (SCT-003, SCT-004), and
   resolve against the contents of each pinned release read from the package cache by digest
   (O7); list and inspect do not supply them.
   Compute each Agent's `ownTools` member from its Type's contract and resolved configuration;
   this needs the package's contents, as any resolution does (O7). Print it where the text grammar
   below requires it. Inspection results and their verification are unchanged.
   Check every place governed by `secret-reference.v1` locally during translation and resolution,
   before printing any step (SEC-001). A literal or bare name stops the plan with status 2:
   print `plan`, then `stop KIND/NAME place=POINTER SEC-001` for each invalid place, then
   `flags []`, then `stopped`. Each stop names the document and the place by its JSON Pointer;
   no line holds any part of the value (SEC-003). This plan has no step, `detail` or `difference`
   line; nothing is kept or sent. A valid value is a reference to a Resource, never the secret itself. Print,
   keep in `.conduit/state.yaml` and compare it like any other reference: as written, `{kind, name}`,
   with the ID it resolves to as inspection shows it (OBJ-009, SEC-004). Where the schema allows
   null, as for S1's `apiKey`, null is not a reference.
   Read the working copy's kept state described below.
   Reject malformed local state rather than treating it as a new working copy.
2. Send `management.list.v1 {}` as root. Inspect each listed object through the tool ACC-033
   requires: a live Resource through its Pool, a live Agent through its Fleet, everything else
   through management, including members of retired owners. Each inspection uses `expectedId`
   from the list. Read resolved definitions, held packages, revisions, lifecycle, inventories,
   tombstones and unreconciled invocations (OPS-036, LIF-013, LIF-040). No private snapshot is
   an acceptable substitute. Read all visible objects to find bounds and outside Conduits.
3. Compare desired translation, last applied definitions/IDs and observed definitions/IDs.
   Print every difference, including drift, control changes, operation contracts, implementation
   maps, nested entries and declared Conduits, including package-provided ones. Comparisons that
   require package contents read them from the cache (O7); do not infer equality from identities
   alone. Compare actual resolved operations even when standard templates should keep their lists
   (PKG-031). A package not installed, or an archive not in the cache, stops the plan with status 2,
   as rows 1 and 2 say. The [difference table](#the-difference-table) decides each difference.
4. Build the deterministic plan below, or a stopped plan with reasons. `plan` prints it and exits:
   it sends only `list.v1` and `inspect.v1`, writes no local files, and sends no mutation.
5. `apply` prints the identical plan before any change. If stopped, print `not sent 1..N` when
   the plan has N steps, then exit without recording or sending anything: no step was sent,
   narrowing included. A stopped plan with no step prints no such line. Otherwise durably keep
   its exact text and digest before sending step 1.
   Persist each request's arguments and key before sending, and its result immediately afterwards.
   Failure to keep this state stops sending. No request is sent after a local persistence error.
6. Send the numbered steps in order, waiting for each result. Print `result N OUTCOME JSON` after each, with JSON the safe projection of its returned
   result in sorted-key compact JSON; these lines never enter the plan digest. An unsuccessful
   result stops execution. Print `not sent N..M` for the remaining steps, if any. No rollback.
   When a successful set-up raises sign-ins, continue apply and print each returned pending
   action ID as `panel action ID: sign-in required`, apart from authored lines. Answer none
   ([ruling 76, point 4](../../proposals/ruling-76/README.md); TPL-012, PND-005, PND-011).

The command is the approval; there is no prompt and nothing runs automatically or later
([ruling 55, A4](../../proposals/ruling-55/README.md),
[specification limits](../../../spec/README.md#what-the-short-forms-meaning-covers)).
Initial reads precede printing and are not step lines. Reads after printing, including inspection
of newly registered objects before open or setup, are steps, printed and keyed like other requests.
Their results must match the expected resolved definition, held packages and returned ID before
execution continues (synthesis F6). A failed verification stops without opening the object.
Every request whose schema accepts `expectedId` carries it (LIF-037), including every open and
close. `replace.v1` instead carries the inspected revision; register and lift accept no expectedId.
A reference in arguments is `{kind, name}`, never an actor-supplied `{kind, id}` (LIF-037).

Inspection and dispatch are separate requests, not an atomic snapshot. IDs defeat name swaps;
revision inputs defeat template changes. Open and setup accept no expected revision or definition
digest. The residual same-ID race is open point O2 below, not an invented engine precondition.

## The difference table

Step 3 compares three values at each place of each object: the file's, from its translation
resolved against package contents read from the cache; the ledger's, from the kept state with
the journal of this working copy's interrupted apply, if any; and the engine's, from list and
inspect. A place is a JSON Pointer into the object's document as the file translates it or,
where only its resolution differs, into its resolved definition. A setting is a JSON Pointer
into an instance's configuration, as a control's `changes` writes it.

Read the rows in order: the first whose condition holds decides. Check rows 1 to 6 first;
row 5 alone looks ahead, and decides only where a step of this plan would reach the object,
so that it never hides a stop of the rows below. Rows 7 to 16 compare an object's presence
and lifecycle; rows 17 to 26 compare each place of an object that the ledger and the engine
hold under one ID and the file holds by name. Each row gives one action: nothing, create,
update in place, replace, remove, or a stop that names its cause. The last row stops on any
difference that no other row names, so planning is total by construction. Where the places of
one object fall under several rows, the plan prints every stop they give; with no stop, the
object is replaced where any of those rows replaces it, and otherwise each row's action is
taken. An inline template `N--t`, while N is in the engine and row 7, 14 or 15 decides N, and a
member that row 14's cascade retires, take no row of their own after row 6. The row that acts on
them decides them, and rows 1 to 6 still apply to them first, as row 5 says of a cascade. Any other
object that another object's row acts on, such as an incident Conduit that a replacement makes
again, keeps its own row. The plan prints every stop that either row gives, so replacing an endpoint
never takes over a Conduit another actor changed (row 10; safety case 5). Where neither row stops,
the row that acts on it decides it.

An object that neither the file nor the ledger holds, and that the engine did not make, is
outside the project: no row applies to it, and the plan reads it only for bounds and outside
Conduits (row 26). An object that only the ledger holds is gone already: nothing is sent for
it, and the next kept state leaves it out. Lines that report what stays, such as the
retained-pin `difference` line and case 07's `remains ready` line, are printed as now.

| # | Difference | What the plan does | Decided by |
| --- | --- | --- | --- |
| 1 | A package the manifest pins is not installed at that version with that digest, as `list.v1` reports | Stop: `stop Package/ADDRESS version=VERSION digest=sha256:HEX installed=no requires=conduit-install PKG-006`. Where that version is installed with another digest: `installed=sha256:OTHER`, no `requires=`, and PKG-025, since installing it would conflict | PKG-006, PKG-021, PKG-025 |
| 2 | The archive of a package the manifest pins, of a package that an archive's manifest lists, through every dependency, other than the core package (NAM-009), or of a release an object holds that row 19 compares, is not in the package cache, or its cached bytes do not hash to its digest | Stop: `stop Package/ADDRESS version=VERSION digest=sha256:HEX archive=absent requires=conduit-install PKG-004`, or `archive=mismatch`. Installing that archive again fills the cache and changes nothing in the engine (PKG-027) | O7; PKG-004, PKG-027 |
| 3 | A relative `fetch`, or a program given as a relative path | Stop, as O1 says: `stop KIND/NAME place=POINTER SUB-010`, with `Manifest/-` as the subject in the project's manifest | O1; SUB-010 |
| 4 | A literal or a bare name at a place that `secret-reference.v1` governs | Stop, as O5 says: `stop KIND/NAME place=POINTER SEC-001`, with no step, `detail` or `difference` line | O5; SEC-001, SEC-003 |
| 5 | An object with an unreconciled invocation, or an instance with a setting reported unknown, that a step of this plan would reach: a step that the rows below give it, read as though this row did not hold, or would give under the flag that one of their stops requires, or a step that another object's row sends to it, as a cascade does | Stop while the invocation is unreconciled or the setting is reported unknown, as "Stops, unknown outcomes and repeats" says: `stop KIND/NAME unreconciled=ID OPS-013`, or `stop KIND/NAME setting=POINTER state=unknown REV-009`, and print beside it every stop that the rows below give the object. Apply sends no reconciliation and no control. Where no step would reach the object, this row does not decide it: the first row below whose condition holds does, as for any other object, whether that is row 17's nothing, row 19's retained-pin line or a stop such as row 27's. Beside that row's decision the plan prints the same cause as a `difference` line: `difference KIND/NAME unreconciled=ID OPS-013`, or `difference KIND/NAME setting=POINTER state=unknown REV-009` | OPS-012 to OPS-014, REV-009, LIF-013; ruling 78 point 5 |
| 6 | An object the engine made: root's first-start objects, or a FLEET-005 default | Nothing is sent to it, and no later row closes, replaces or adopts it. A default closes only when its Pool or Fleet retires (CON-008), which the plan prints under `affects`. Where the file holds a document of its kind and name: `stop KIND/NAME made-by=engine ACC-024` for a Conduit, or with OBJ-002 for another object; no flag permits it | ACC-024, FLEET-005, CON-008, OBJ-002; safety case 13 |
| 7 | A retired instance whose shutdown is unconfirmed: one this working copy retired, or one the file holds that this working copy did not retire | A further retirement under the flag that permitted the retirement, which this command names again (LIF-026); never `unregister.v1` (LIF-017). Without the flag: `stop KIND/NAME state=retired requires=FLAG NAME LIF-026`. For an instance that `--prune` named, once it is removed, `unregister.v1` of its inline template `N--t` follows under the same flag, as row 15 says. Where the file holds an instance that this working copy did not retire, because another actor retired it or its node was declared lost (PLC-017, PLC-024): `stop KIND/NAME state=retired LIF-024`, with no `requires=`. No later row decides it; once its shutdown is confirmed, row 11 does (safety case 3) | LIF-024, LIF-026, LIF-017 |
| 8 | An instance this working copy registered that is still registered: an earlier apply stopped before its setup or open | Where no place of it differs from the file's: inspect and verify it, then set it up or open it under the flag its registration carried, as the journal shows. Where that was a flag this command does not name: `stop KIND/NAME state=registered requires=FLAG NAME LIF-013`. Where a place differs, rows 17 to 26 decide it | LIF-013, LIF-037, TPL-012 |
| 9 | In the file; absent from the ledger and the engine; no tombstone matches it | Create: register; then, for an instance, inspect, then set up or open, with no flag (cases 01 and 02) | SCT-003, LIF-037, TPL-012 |
| 10 | In the file and in the engine under its kind and name, where the ledger has no entry for it or holds another ID, or, for a template, another revision | Stop: `stop KIND/NAME id=ID requires=--adopt NAME OBJ-002`, with `revision=REVISION` after `id=` for a template that another actor replaced. With `--adopt`, take it into the ledger as "Approval and command readings under ruling 76" says; the rows below then decide it | OBJ-002, LIF-037; safety case 5 |
| 11 | In the file and absent from the engine, and a tombstone matches its registration or its open | Lift it with no further flag where "Which tombstones apply lifts with no further flag" allows. Otherwise stop: `stop KIND/NAME tombstone=ID requires=--reopen NAME LIF-041` for a Conduit, `--recreate NAME` for another instance, and `tombstone=(KIND/OLD)` for a tombstone this apply would make, as for a renamed Conduit. With the flag: lift it, register, inspect, then open or set up (case 08) | LIF-040, LIF-041, ACC-053; ruling 76 point 1; cases 08 and 09; safety cases 1 to 4 |
| 12 | In the file and the ledger; absent from the engine; no tombstone matches it | Stop: `stop KIND/NAME absent=engine requires=--recreate NAME LIF-040`, or `--reopen NAME` for a Conduit. With the flag: register, inspect, then set up or open | LIF-040 |
| 13 | In the ledger and the engine, absent from the file: a Conduit | Close it, with no flag; its tombstone stays (case 07). A renamed Conduit is this row and row 11 | ruling 76 point 1; CON-008 |
| 14 | In the ledger and the engine, absent from the file: a Pool or Fleet that owns members | Its cascade under `--with-resources NAME` or `--with-agents NAME`, printed as "Replacement, removal and cascades" shows. Without the flag: `stop Pool/NAME absent=file requires=--with-resources NAME LIF-020`, or the Fleet form. Members that the file also leaves out take no row of their own. Once each is removed, `unregister.v1` of its inline template follows under the same flag | LIF-020; ruling 59 A3 |
| 15 | In the ledger and the engine, absent from the file: another instance | `--prune NAME`, then its retirement, then, once it is removed, `unregister.v1` of its inline template `N--t` under the same flag. Without the flag: `stop KIND/NAME absent=file requires=--prune NAME LIF-035`. A renamed instance is this row and row 9 | ruling 76 point 1; SCT-006, LIF-035 |
| 16 | In the ledger and the engine, absent from the file: a template that is not inline, or an inline template `N--t` whose instance N the engine no longer holds | `--prune NAME`, then `unregister.v1` at its revision, carrying `expectedId`. Without the flag: `stop KIND/NAME absent=file requires=--prune NAME INS-020` | INS-020 |
| 17 | A place where the file's value is the engine's | Nothing, whatever the ledger holds | this table |
| 18 | A project template whose document differs from the engine's, or that refers to a package whose pin moved, or to a contract document or an implementation item of the project that changed | Update in place: `replace.v1` at its inspected revision, with the project's contracts and manifest, class `replaces`. It changes no instance, so it takes no flag of its own; where the plan replaces its inline instance, the step carries that `--replace` (cases 05 and 06). Rows 19 to 26 decide each instance | SUB-008, SUB-009, INS-020, SCT-006 |
| 19 | An object whose held pin is not the one the file's resolution gives, where row 18 does not replace it and its resolved meaning and execution would not differ under the file's release | Nothing: it keeps its pins, and the plan prints its retained-pin `difference` line, as cases 05 to 09 do | NAM-009 to NAM-011 |
| 20 | A nested instance, a declared Conduit or a Pool entry's Resource, whose template, named by its `from`, changed, while the instance that holds it is not replaced | Close it, or retire it through its Pool; list and lift the tombstone that leaves where the new one would match it; then `register-replacement.v1`, which gives it a new name and ID; then inspect, and open or set up. The close or retirement, the lift, the registration and the open or setup carry `--replace NAME`, naming the nested instance as inspection shows it, or no flag for a declared Conduit whose access narrows as row 21 says. Without the flag: `stop KIND/NAME place=/from requires=--replace NAME LIF-009` | LIF-009, LIF-041; ruling 76 points 1 and 2 |
| 21 | A Conduit whose access after the change is within its access before: the same consumer and target, its resolved operations a subset, and each requirement kept with the same arguments, except that requirements may be added and each list of a `bound.v1` (`pools`, `targets`, `operations`) may shrink | Close, list, lift its tombstone, register, inspect, open, with no flag (case 04, Agent B's Conduit). A change of `as` alone is one | ruling 76 point 1; ACC-029, LIF-041 |
| 22 | Any other change to a Conduit: its consumer, target, operations, requirements or their arguments, its `from` or its template | Replace it under `--replace NAME` by the same steps (case 04, Agent A's Conduit). A change the planner cannot show to narrow is widening. Without the flag: `stop Conduit/NAME place=POINTER requires=--replace NAME OBJ-011` | ruling 76 point 1; OBJ-011 |
| 23 | A setting whose value in the engine is not the ledger's: a control changed it | Where the file's value is the ledger's, send nothing for it and print `difference KIND/NAME setting=POINTER changed-by=control REV-007`. Stop instead, with `stop KIND/NAME setting=POINTER changed-by=control REV-007`, where the plan would replace the instance for another difference, which would revert the control's value, or where the file holds a third value. No flag permits that stop; writing the engine's value into the file settles it | REV-007 |
| 24 | A Pool or Fleet that owns members, whose definition the file changed | Stop: `stop KIND/NAME place=POINTER LIF-020`. No flag permits it, since replacing it would retire its members | LIF-020 |
| 25 | A setting whose value the file changed, where the engine's value is the ledger's | Replace the instance as row 26 says; apply never sends a control. Without `--replace NAME`: `stop KIND/NAME setting=POINTER control=OPERATION requires=--replace NAME OBJ-011`, one line for each setting. `control=` names a control the instance has, one its Type lists that its template's `controls` map, where it has one, also lists (TPL-010, TPL-013), and whose `changes` holds that pointer. It is given only while the instance is ready, since a control is not dispatched otherwise (CON-024). Calling that control is the author's other remedy. Where the instance has no control whose `changes` holds that pointer, or it is not ready, the line has no `control=` item, and `--replace NAME` is the only remedy | OBJ-011, OPS-020, REV-007, TPL-013, CON-024; decision D67 |
| 26 | Another instance whose definition would differ: its `from`, `owner`, `description` or `annotations`; its resolved definition other than its configuration, `placement` and `runtime`; or a pin of a package it uses, where row 19 does not hold | Replace it under `--replace NAME`, as "Replacement, removal and cascades" says: its incident Conduits are made again under the same flag; the plan prints the bounds that cease to cover it and the loss of an Agent's sign-in; and a Conduit outside the file whose consumer or target it is stops the plan, as "Approval and command readings under ruling 76" says. An owner change is a replacement, never a move. Without the flag: `stop KIND/NAME place=POINTER requires=--replace NAME OBJ-011`, with `pin=ADDRESS@VERSION` in place of `place=` for a moved pin, one line for each | OBJ-011, INS-020, FLEET-002, NAM-010, ACC-029 |
| 27 | Any other difference | Stop, naming the field: `stop SUBJECT place=POINTER rule=none -`. No flag permits it; a later order adds a row for it. In v0 it takes, among others, a changed `placement` or `runtime` (safety case 12), `apiVersion`, and the manifest's `address`. `state=STATE` takes the place of `place=` for a lifecycle state that no row above decides | this table |

## Approval and command readings under ruling 76

Each actual approval flag use is printed once under `flags`, recorded with the plan and counted when used
([ruling 55, A4](../../proposals/ruling-55/README.md),
[ruling 59, A3](../../proposals/ruling-59/README.md)). Unused, ambiguous and contradictory
operands stop planning; one permission never means all objects.

| Point | Ruled reading | Source |
| --- | --- | --- |
| A: acts | Narrowing never needs a flag. Apply closes a Conduit the file dropped, and closes/recreates a retained Conduit whose resolved operations narrow, printing and recording each closing. Widening, replacing, reopening, recreating, adopting and cascades keep their named flags. `--prune` still permits removal of another object absent from the translation; it is not needed for a dropped Conduit. `--replace` is not needed for a Conduit operation change whose resolved access is a subset. | Ruling 76 point 1 amends ruling 53 point 4 for removal only. |
| B: operands | Every approval flag names one object by its generated name, including `--reopen agent-a--project`. Resolve unambiguously by kind and ID before printing. No operand-free permission is defined. | Ruling 76 point 2. |
| C: engine | Commands use the person's current engine context recorded by `conduit serve`; `--store DIR` overrides it. No engine value is required on each command. | Ruling 76 point 3. |
| D: sign-in | A successful set-up returns pending action IDs; apply goes on, prints each `panel action ID: sign-in required`, and answers none. An unsuccessful result still stops under the ordinary rule. Panel actions are listed apart from authored lines. | Ruling 76 point 4; ruling 68 R1; TPL-012, PND-005, PND-011. |
| E: plan | Plan is available and never required. Apply computes, prints and records its own plan. | Ruling 76 point 5. |

The other named permissions retain these precise meanings:
`--reopen` lifts a Conduit tombstone that the rule "Which tombstones apply lifts with no further
flag", under [Replacement, removal and cascades](#replacement-removal-and-cascades), leaves to a
flag, before registering/opening its desired grant;
`--recreate` does that for another instance; `--adopt` takes over a foreign existing object into
this working copy's ledger only after its ID, resolved definition and held packages are printed
and verified. Adoption does not overwrite, lift, open or set up by itself: any such act needs its
own permission. Nothing besides adopt takes over a foreign object (synthesis F6).
`--replace` includes incident Conduits that must be closed and recreated to replace its named
endpoint, and the necessary inline-template update. It does not authorize unrelated replacements,
outside tombstones, or retargeting outside-file Conduits. An outside-file Conduit needs its own
named replacement permission and explicit desired definition; otherwise the plan stops.
Closing a dropped Conduit without a flag, or removing an instance with `--prune`, leaves the
resulting tombstones to the named reopening/recreation flags under the rule below.
`--with-resources N` and `--with-agents N`
permit only the named container's printed cascade. None of the flags bypasses engine access.

## Ordering and access

The limits say: "Narrowing never waits. A plan prints and records every widening, including one
caused by moving a package pin or changing a reusable template's operations (PKG-031)" and
"the plan still checks the operations that actually resolve"
([specification](../../../spec/README.md#what-the-short-forms-meaning-covers)).

Reading adopted here: every narrowing step precedes every widening step. Access after each step,
as a whole, is a subset of access before apply or a subset of the access requested by the files.
For T4 close both project Conduits, restore B's read, then open A's write: no intermediate state
lets both write. Register inert documents between these phases where dependencies need them.
Where a plan closes or retires anything, it first sends every template step, before the first
close or retirement: `replace.v1` for each template it replaces, then, where it adds templates,
one `register.v1` holding them. A template step changes no access and no existing instance
(INS-020), so a template the engine rejects stops the plan before anything is closed or retired.
Cases 05 and 06 replace `shared--t` at step 1 and retire `shared` at step 2.
Root defaults added during registration count as added access and must follow all narrowings.
The one-time Conduit that a cascade flag permits is the one stated exception to both narrowing
before widening and the subset rule: the plan prints its opening, use and closure
([ruling 59 A3](../../proposals/ruling-59/README.md)).
This is our ordering choice, from "It narrows before it widens" in the
[accepted simplification, point 4](../../proposals/simplification/README.md).
That proposal's make-before-retire replacement was reversed by ruling 55 A3.

Our choice on an incomplete approval: print the whole stopped plan and send no part, including
narrowings, when any required flag is missing. This does not defer an independently requested
narrowing behind a prompt: no execution starts for an invalid command. A plan containing only
flag-free narrowings is valid without flags, under ruling 76 point 1. Do not run a narrowing subset whose
removal could invalidate the wider plan that the command purported to approve.
This holds for every stop the difference table gives, and a narrowing so waits only until the
command is run again, with the missing flag or without the change that needs it.
Order ties by phase, then dependency, then kind/name in byte order. In the golden cases a batched
registration preserves the written documents' order; independent setup orders Pool, Resource,
Fleet, Agent, Interface and then name. Reads immediately precede the activation they verify.

## Root's authority and protected Conduits

Every request is root's own request, without an onward caller, in SEC-008's own-behalf form;
SEC-008 itself restricts reads of secrets, not these management verbs. Only root registers an
Agent (ACC-057) or registers/opens a Conduit consumed by root (ACC-054). Each request uses the
Conduit ACC-033 requires; its printed tool is root's actual local tool name (IFC-012).
Registering into an existing Pool or Fleet needs management and that owner even though the
called tool is `management.register.v1`. Pool/Fleet default tools use their target's local name.

Never request closure of root's first-start `management`, `access`, `system`, or any FLEET-005
default (ACC-024). A default goes only when its Pool or Fleet retires (CON-008, FLEET-005).
Root may close its other Conduits on its own behalf: `N--operator` (SCT-009), or a standalone
`conduits` member consumed by root. Reopening follows the ordinary tombstone rules.
Synthesis F6's old restriction on all root-consumed Conduits predates ruling 59 A3 and is not used.

## Replacement, removal and cascades

A standalone replacement keeps kind and name (SCT-004; ruling 55 A3). Print and send:

1. Close the old Conduit, or retire the old instance, before making the replacement.
2. Continue only after removal. Closing removes immediately. A succeeded retirement confirms
   shutdown and removes without another request (LIF-024, LIF-035 and `retire.v1`). Until then
   the name is occupied; OBJ-002 rejects another document of that kind and name.
3. Lift every matching tombstone this same apply caused, without another flag (LIF-041,
   ACC-053; [the interplay note](../../proposals/spec-batch-1/areas/additions.md)). Read fresh
   tombstones with list after removal, and verify their cause before lifting; do not guess IDs.
4. Register under the same name, taking each new ID from the result.
5. Inspect, verify, then explicitly open or set up, using those returned IDs.

An inline template `N--t` (SCT-006) already exists: do not register it again (OBJ-002).
Change it with `management.replace.v1` at its current revision, remove it with
`management.unregister.v1` (ACC-033 row 2). Existing instances keep their resolved definitions
(INS-020). In T5 replace `shared--t` with the same definition but the filesystem 0.2.1 manifest;
retire/remove `shared`, lift, register a new `shared` from the updated template, and recreate
its two project Conduits. T6 changes `shared--t`'s root to `/lab/replacement`, then does the same
instance sequence at the same package pin. The history and Agent instances are unchanged.
Merely moving a manifest pin does not rewrite unaffected held pins (NAM-009, NAM-010): compare
resolved meaning and execution, not manifest text alone. Print this retained-pin difference.

After `conduit install` moves a package pin:

**Templates.** Replace each project template that refers to the moved package, and no other,
using `replace.v1` at its revision with the new manifest (SUB-009). Case 05 replaces `shared--t`
alone; for the other templates SUB-009 would store no change.

**Instances.** Replace each existing instance whose resolved meaning or execution would differ
under the new release, under `--replace` naming it (INS-020).

**Everything else.** Every other object keeps the pins it holds (NAM-009 to NAM-011). While it
holds an old pin, every plan prints its retained-pin `difference` line.

**Contents needed.** The comparison reads both releases' archives from the cache (O7).

A nested instance is different (LIF-009). For a declared Conduit or a Pool entry's Resource,
print `management.register-replacement.v1 ParentKind/parent expected=ID neither flag=--replace NAME LIF-009`,
or `flag=-` for a declared Conduit whose access narrows as row 21 says, with `nested=member`
in the detail. It generates a new name and ID; show its later references as
`Kind/(step N).object expected=(step N)`. Before it, the plan lifts the tombstone that its own
close or retirement of the nested instance left, where the new one would match it (LIF-041),
under the same flag (row 20). Old retired nested names remain occupied until removed. The
local name of a declared Conduit remains unchanged.

A tombstone made outside this apply needs `--reopen` (Conduit) or `--recreate` (other instance)
under the rule below, even if a line was renamed or records purged (LIF-040, LIF-041).
An interrupted apply's own journal may establish the same apply and cause; otherwise require
the named flag.

- **Which tombstones apply lifts with no further flag.** Apply lifts a tombstone this command
  caused, with no further flag, when the same command makes the same object again under
  the same kind and name: the object a `--replace` names and its incident Conduits (cases 04,
  05 and 06), or a retained Conduit closed and made again because its access narrows as row 21
  says (ruling 76 point 1; case 04, steps 2 and 5). Apply also lifts with no further flag a
  tombstone that this command left by closing or retiring a nested instance that it makes
  again by `register-replacement.v1` under a new name (row 20). Every other tombstone a
  registration or open would match needs `--reopen` (Conduit) or `--recreate` (another instance),
  naming the object to be made. This includes an outside tombstone and one this apply leaves by closing a dropped
  Conduit or by `--prune`, including Conduits a pruned instance's retirement closes under
  CON-008. Reopening still takes a flag
  ([ruling 76, What it amends](../../proposals/ruling-76/README.md#what-it-amends)).

A Conduit renamed in the file that keeps its consumer, target and capability is a flag-free
close of OLD and a new Conduit whose registration matches OLD's tombstone (LIF-041). The plan
needs `--reopen NEW`, naming the new Conduit by its generated name. With that flag, close OLD,
list, lift that tombstone, register NEW, inspect NEW, then open NEW. Print the flag on the lift,
register and open, as case 08 does. Without it, stop before sending anything:
`stop Conduit/NEW tombstone=(Conduit/OLD) requires=--reopen NEW LIF-041`.
A tombstone this apply would make is named by its object in brackets; case 09 instead gives
an existing tombstone's ID.

Any other instance renamed in the file is removal of OLD under `--prune OLD` and addition of
NEW. NEW matches no tombstone, since another instance matches by kind and name. OLD's retirement
closes its Conduits (CON-008) and leaves their tombstones. A Conduit the file names that matches
one needs `--reopen` naming that Conduit, as above; for example, a Conduit with the same consumer
and local name that now targets NEW.

Removal depends on kind:

- A Conduit the file drops is closed with no flag (ruling 76 point 1). Its tombstone stays;
  adding it back takes `--reopen` (cases 07, 09 and 08).
- Another instance takes `--prune N`, then its retirement; adding it back takes `--recreate N`.
  The same `--prune N` permits removing its inline template `N--t` (SCT-006), as `--replace N`
  covers that template's update. Once N's retirement has removed N, remove the template with
  `unregister.v1` at its revision, carrying `expectedId`.
- A dropped template that is not inline takes `--prune` naming it, then `unregister.v1` at its
  revision, carrying `expectedId`.
- A Pool or Fleet that owns members takes its cascade flag, as described below (LIF-020).
- A retirement that does not succeed is recovered by a further explicit retirement under the
  original permission (LIF-026), subject to the observed identity and lifecycle checks below.
  Apply never sends `unregister.v1` to recover it: that request asserts shutdown settled
  elsewhere (LIF-017), which this reference leaves to the author.

An unsuccessful retirement stops this apply. The next plan freshly lists and inspects the current
identity and lifecycle state (OPS-036, LIF-013); it does not infer retirement from a rejected
request. Retirement starts at dispatch, and an expectedId conflict retires nothing (`retire.v1`;
OPS-005). A rejected request can leave the instance ready, or name an instance already removed.
If the observed instance is retired with shutdown unconfirmed, it still holds its name (LIF-024).
Only in that case is our recovery choice a further explicit retirement (LIF-026), under the
original replacement/prune permission. Otherwise re-plan from the observed identity and state:
do not send a further-retirement recovery step for an absent object, or reuse the old expectedId
for a replacement object. A new command is required, with ordinary drift/tombstone checks and
no automatic resend. Do not offer unregister as normal recovery: it asserts shutdown settled
elsewhere (LIF-017). No new flag is invented for this assertion; this planner leaves it to the
existing explicit unregister command after the author settles shutdown. Unknown outcomes must
first be reconciled.

Print every bound that ceases to cover a replaced Resource: individual target items retain the
old ID; a bound naming its unchanged owning Pool covers the new Resource (ACC-029, ruling 59 A7).
Print every Conduit retirement closes (CON-008), including outside the file. An owner change is
a replacement, never a move (FLEET-002). Print the loss of an Agent's sign-in on replacement,
not a promise to preserve it. A changed control value (REV-007) is a shown difference, never
reverted by this planner, under any flag. No control call is inferred from a file edit here.
An author's edit to a setting is a replacement under `--replace`, and while the instance is
ready its stop names a control that covers the setting (rows 23 and 25). Whether apply may
send the control instead is the owner's, asked with finding 4-01 after the S1 rerun
([decision D67](../../proposals/full-review/decisions.md#the-experiments-documents-and-the-register)).

Dropping a Pool/Fleet that still owns members stops unless the appropriate cascade flag names it
(LIF-020). Each cascade is one command, counted under that one flag (ruling 59 A3, plan C8/C13).
The following is the sole printed pattern; substitute `Fleet`, `retire-fleet`, `with-agents` and
`retire-with-agents.v1` for the Pool variant. IDs/names created by a step remain symbolic.

```text
1 management.register.v1 Conduit/cascade-team expected=(step 1) adds flag=--with-resources team ACC-054
2 management.inspect.v1 Conduit/cascade-team expected=(step 1) neither flag=- LIF-013
3 access.open.v1 Conduit/cascade-team expected=(step 1) widens flag=--with-resources team LIF-037
4 cascade-team.retire-with-resources.v1 Pool/team expected=POOL-ID narrows flag=--with-resources team LIF-021
5 management.list.v1 Tombstone/cascade-team expected=- neither flag=- LIF-040
6 access.lift.v1 Tombstone/cascade-team expected=(step 5) neither flag=--with-resources team ACC-053
affects step=4 closes=[Conduit/cascade-team] CON-008
flags [--with-resources team]
ready
```

This necessary scoped permission is the ordering and subset-rule exception of ruling 59 A3:
its temporary management access must open before the cascade narrows; list it explicitly,
never hide it as provider work. It grants
only retirement, not data/model operations. The cascade flag permits this one-time Conduit's
opening, use and closure, all printed in the plan.
The one-time name is a deterministic unused `cascade-TARGET` name (truncate and add a digest
suffix on collision); it is not an engine-made default. Register it from core `retire-pool` or
`retire-fleet`, consumer root, target the named container, `as` the one-time name. The schema of
the cascade takes no expectedId; the expected field printed at step 4 is a checked observation,
not a new wire argument. After successful retirement CON-008 has closed/removed this Conduit
as part of step 4.
That is the close printed under affects: sending another close would be invalid. Step 5 reads
its tombstone and step 6 lifts it. This is the reading under CON-008 (O4), in place of C13's
literal five requests. If the cascade does not succeed, the ordinary stop rule applies. The
container's retirement can fail after its dispatch has already closed/removed the temporary
Conduit (LIF-021, TPL-009, CON-008); failure alone does not say whether it survives.
After any unknown outcome is reconciled, the next explicit apply first offers cleanup under
the original named cascade permission, before any new cascade. Freshly list objects and
tombstones, and inspect a surviving temporary Conduit with the ID returned by its journaled
registration (OPS-036, LIF-013, LIF-037). Close it only if that observation verifies the same
object, carrying that ID in `expectedId`, then list again. If already removed, skip closure.
Lift only the tombstone whose retained object ID matches that journaled Conduit, using the
tombstone's freshly observed ID (LIF-040, ACC-053). If neither object nor tombstone remains,
cleanup has nothing to send. If the name now identifies a replacement object, stop cleanup
and report the identity conflict; never close it or lift its tombstone. The S1 golden cases
use no cascade.

## Stops, unknown outcomes and repeats

Refused, invalid, conflict, unavailable, failed and unknown all stop sending. Print completed
results and unattempted steps. Never send a request again on this command's own initiative
([seventh convention](../../../spec/README.md#what-this-is)). A new plan reads actual inventory
and the partial journal; successful steps are not undone or sent again merely to reach the next
one. Failed attempts require a new explicit command and a newly journaled attempt number.
The difference table makes planning total, not applying atomic. A step can still fail after
earlier steps succeeded; apply then stops, prints `not sent N..M`, exits 1 and rolls nothing
back, and the next plan starts from what the engine shows.

For unknown, show the invocation ID from the result or LIF-013's `unreconciled`, the affected
objects and the steps withheld. Refuse a plan that would mutate, replace or reactivate an affected
object until an actor explicitly reconciles it (OPS-012 to OPS-014); this planner sends no
reconcile. Even where OPS-012 permits a deduplicating retry, it does not do one automatically.
Unrelated work can be planned separately, with the unknown difference printed.

Repeating an identical plan is not permission to repeat its effects. Each request key is
`plan:<hex digest>:<attempt>:<step>`, carried in
`io.github.shrav-k.conduit/idempotencyKey` (IFC-023, OPS-008). Attempt is a working-copy counter,
initially 1, increased only by a new explicit command after a known unsuccessful attempt; keep
it across interruptions. An uncertain delivery keeps the old key and stops for reconciliation.
For an explicit resumed identical request, OPS-009 returns kept success without dispatch;
OPS-034 conflicts if required outputs expired. Never choose a new key to recover expired outputs.
If all intended changes already succeeded, re-planning prints no changes, not the old plan.
Restoration of the same grant after another revocation has a different observed tombstone/ID,
so a different plan digest. A valid no-op apply sends nothing and writes nothing.

## Kept state and privacy

All paths here are relative to PROJECT_DIR. Keep exact plan text as UTF-8, LF, one final newline,
in `.conduit/plans/HEX.txt`; SHA-256 hashes precisely those bytes, not result lines or its filename.
The digest is `sha256:HEX`. Keep `.conduit/state.yaml` with format `1`, a relative project-file
map, applied object ledger (kind/name, ID, revision, declared definition and held pins), tombstones
and plan references. Keep `.conduit/results/HEX.yaml` as the ordered send journal, arguments,
keys and returned outcomes/IDs, including partial progress. Use atomic replacement for state,
and durable journal-before-send. No timestamps enter plan text. Plan and no-op apply write none.

Do not commit `.conduit/`; recreate/adopt deliberately for another working copy rather than
copying its ownership ledger. Anyone who edits this state and runs apply as root can widen
access: this ledger is not an authorization boundary. Engine permissions remain the boundary.
Keep no absolute path, store path or credential.
Use relative file origins and typed redacted markers for those values in saved snapshots, with
in-memory comparisons against freshly read files. For a non-secret absolute path, retain only
`{pathSha256: HEX}`, the SHA-256 of its UTF-8 bytes, and print `root-sha256=HEX` in the plan.
This is a CLI comparison fingerprint, not a new wire input; it makes T6 distinct without storing
an absolute path. Never fingerprint a credential. SEC-003 and SEC-004 bind diagnostics and
records only; applying their protection to this local state, and keeping a valid reference as
written beside its resolved ID, is our choice by analogy. Check secret-reference places before
printing any step (SEC-001); diagnostics hold no part of an invalid value (SEC-003). Keep a valid
secret reference as written, `{kind, name}`, with its resolved ID as inspection shows it, and
compare it like any other reference (OBJ-009, SEC-004). It refers to a Resource and holds no secret.
No raw wire envelope or returned CloudEvent is persisted if it violates that policy. Keep safe
projections of results; reread a required omitted value next time. O1 records the relative-path
limitation; never pretend a redacted snapshot proves equality.

The exact safe record in each case is `recorded: {path, digest, text}`. A null path means
no record is written: the case still states the computed digest and exact text for review. `kept` is a compact
serialization of the safe ledger; request entries are retained by plan digest and step under
`journals`, using `safe:case:ID/requests` to reference the safe projection of that case's entries.
The projection replaces every non-secret absolute path with `{pathSha256: HEX}`; raw authoring
arguments in the case remain available for checking the wire input, not for saving locally.
That reference is a fixture notation for the saved safe results, not a runtime dependency on a
case file. The actual journal contains the entries themselves. Generated IDs returned by the
engine are concrete in results/kept, symbolic in the plan printed before they exist.

## Text grammar and statuses

The first line is `plan`; no timestamp, endpoint, path, random ID or command spelling occurs.
One numbered line per post-print request. A batched registration needing several named
permissions joins their exact spellings with commas in its flag field:

```text
N TOOL KIND/NAME expected=ID CLASS flag=PERMISSION PROMISE
```

An object registered by this apply is `expected=(step N)` on its registration and all later
references, with the step that registered it. The wire uses its returned ID on later requests.
List reads with no single object use `Tombstone/* expected=-`; list-result tombstone IDs are
`(step N)` until discovered, constrained by their printed cause. A register with several documents
has one line per request, its primary object followed by `objects=[Kind/name,...]` detail.
Additional deterministic lines start `detail`, `affects`, `difference`, `flags`, or `stop`.
Detail names the from/template change, new root as its `root-sha256` fingerprint, nested member,
resolved operation set and package identity where relevant. Beside each Agent registered or
replaced, print `detail ownTools Agent/NAME JSON CTR-014` after that step's other `detail` lines,
one per Agent in its `objects=` list's order. Beside each Conduit opened whose target is an Agent
and whose operations include one marked `startsModelWork: true`, print
`detail targetOwnTools Agent/NAME JSON CTR-014` after its `operations=` line (CTR-014, LIF-013).
JSON is the member as `inventory.v1` gives it, in sorted-key compact JSON, except that an
absolute-path `directory` is printed as `{"pathSha256":"HEX"}`, as document lines print `root`:
plan text holds no path (see [Kept state and privacy](#kept-state-and-privacy)).
This distinguishes otherwise similar plans without disclosing prohibited values.
Full arguments are journaled only in safe form.
Affects lines follow all steps, then differences, then flags sorted by flag and operand.
The last line is `ready`, `no changes`, or `stopped`.

```text
stop SUBJECT CAUSE... [requires=REMEDY] PROMISE
```

SUBJECT is `KIND/NAME` for an object, `Package/ADDRESS` for a package, or `Manifest/-` for the
project's manifest. CAUSE is one or more `key=value` items, as the row of the difference table
gives them. The keys are `place`, `setting`, `pin`, `control`, `tombstone`, `id`, `revision`,
`absent`, `state`, `unreconciled`, `made-by`, `changed-by`, `installed`, `archive`, `version`,
`digest` and `rule`. `requires=` gives the flag and its operand that would permit the step, or
`conduit-install`; it is absent where nothing would. PROMISE is the promise that decides, or
`-` where none does.

A plan prints one stop line for each cause. Stop lines follow the steps and their `detail`
lines and come before `affects`, sorted by subject and then by the whole line, in byte order.
The `difference` lines of rows 5 and 23 take the same form, with `difference` in place of `stop`
and no `requires=`; the retained-pin line of row 19 and case 07's `remains ready` line keep
their present form.

A stopped plan has one form, case 09's: `plan`, its steps and their `detail` lines, its stop
lines, any `affects` and `difference` lines, its `flags` line, then `stopped`. A stop of row 4
still prints no step, `detail` or `difference` line, but prints `flags []` before `stopped`.

| Class | Meaning |
| --- | --- |
| adds | Registers a previously absent object, including any root default made within registration; prints all such defaults. |
| narrows | Removes access, by close or retirement, or changes it to a strict subset. |
| widens | Activates access beyond the current set, by open or setup that can open declared Conduits. |
| replaces | Changes the stored template for a replacement; its instances still keep their old resolution (INS-020). |
| removes | Unregisters a template or settled retired instance; note any access that disappears. |
| neither | Read, lift or access-neutral setup; lifting permits a later registration but itself grants nothing. |

Setup is `widens` if any declared Conduit can open, including nested Pool setups that open any;
otherwise `neither` (TPL-012). A lifecycle operation's closed Conduits still print under affects.
The S1 fixture Agents have only standalone consumes, so their setups are neither; the standalone
opens are explicit widening steps. Count and print all widening steps even under one flag.
A recreated Conduit's open is `widens` relative to its just-closed state. Permission follows
the resolved access change across the apply: restoring B with fewer operations in T4 is
flag-free narrowing under ruling 76 point 1, even though its open step has that class.
A plan with no step and no stop prints `plan`, then each `difference` line, then
`no changes`; it has no `affects` or `flags` line. With no difference, as in case 03, it is exactly
`plan\nno changes\n`. Every plan from case 05 on prints the retained-pin `difference` line,
so a no-op there has at least three lines. A no-op sends nothing and writes nothing;
a no-op leaves kept state unchanged.

| Status | Both commands |
| --- | --- |
| 0 | Valid plan (even with proposed steps), successful apply, or no changes. Pending sign-in alone is 0 under ruling 76 point 4. |
| 1 | A sent request did not succeed, verification failed, or transport failed, including unknown delivery; remaining steps not sent. |
| 2 | Usage, local input/state/persistence error, missing permission, foreign object, drift conflict, blocked tombstone, unresolved unknown or unsupported input; no mutations if detected before sending. |

An I/O failure after partial sending exits 2 and retains the completed journal; it never implies
rollback. These statuses are CLI choices, not additional core outcomes.

## Case format and sequence

The [nine YAML cases](cases/) are author expectations, not vectors. `printed` is full stdout,
including apply result lines after `ready`; `recorded.text` is only the preceding plan, which
is the text hashed. A null record path means no write, not an absent expected digest. Sources use promise IDs,
rulings and the batch plan's line numbers (052a 643–647; 052b 649–652; K3 412). Their areas are
plan (T1–T3), grants (T4, T7 revoke/restore), replacement (T5–T6), stops (unflagged restoration).
The remaining cases are listed under
[Safety cases for 052a's convergence check](#safety-cases-for-052as-convergence-check).

`files` contains literal flow-style documents. Requests carry the complete framed documents,
with no short form on the wire. `arguments` contains expectedId exactly where the operation
accepts it; its absence on register, replace, list and lift is deliberate. Results show outcome
and all values needed by subsequent steps. NAM-008 reports each installed pin unused by the
submitted documents, even when other objects in the engine use it. These cases retain the
project's full manifest and print each unused address as a nonblocking `note` diagnostic with
`ruleId: NAM-008` and message `Unused package ADDRESS; submission accepted.` This exact wording
is a golden-case choice. It appears in the request result and printed result line, outside the
hashed plan text; saved journals retain it through their safe result references.
Inspection `view` and list `tombstones` results use
the compact notation below
for declared/package/inventory projections; omitted records are irrelevant safe projections,
not invented empty record lists. All nine cases assume the person's package cache holds the
archive of every listed address, version and digest, put there by the install commands under
"S1 commands and counts under ruling 76" (T1, T2 and T5); the core package is built in. Schemas,
defaults, operation contracts and implementation maps come from those archives. The compact
`engine` notation lists identities only, and the cache is the planner's other input (O7).

`engine` has `base: first-start`, installed `packages` and `objects`, `tombstones`, `unknown: []`.
Base means the complete first-start objects/defaults from the core, with no project objects.
Its fixture choices are system=UUID suffix 001, local=002, management=003, access=004, root=005,
and root management/access Conduits=006/007; these are not imported from the old state snapshot.
Local placement is the only node; no provider beyond the engine defaults is selected.
Rows are `[kind, name, id, revision, state, definition, pins]`; template state is `template`.
Definition is the framed template spec or instance body plus a `resolved` spec and, for open
Conduits, a `provenance` projection; package references in a row resolve
using its pins, local references using listed IDs. Instances additionally keep the resolved spec
of their `from` at creation, not the current template. `config.root: {pathSha256: HEX}` in kept rows is
a safe comparison fingerprint; actual authored/registered value is in that case's files/requests.
Inventory is the join of all standalone Conduits by consumer and target IDs, plus their tombstones;
these cases have no nested Conduits/entries or outside bounds. Default rows carry `engine-made`
and `as`; their operations are exactly FLEET-005. This is one compact read-state notation, not a
new engine format. Initial reads expand it as OPS-036/LIF-013 require. Returned generated names
and IDs are fixed fixture choices, in the conformance UUID form; a real engine need not pick them.

The sequence is cases 01–07, then 09 (unflagged restoration), then 08 (successful restoration).
Each case's `applied` equals the preceding case's `kept`. Case 09 keeps its input unchanged,
so case 08 can then restore with the named flag. The original filenames stay unchanged so
experiment 023's case mapping still names the same inputs.
Between cases only explicit install commands add packages:
T1 installs dependencies before apply; T2 adds history and its filesystem dependency; T5 adds
filesystem 0.2.1. Thus "only first-start objects" does not mean an empty installed catalog.
History 0.1.0 holds filesystem 0.1.0 while the project initially pins 0.1.1. This is legal:
NAM-006 chooses one version per manifest, NAM-009/NAM-010 retain each object's pins, and NAM-011
installs releases side by side. No illustrative pin is needed. Fixtures provide all releases.
Exact archive digests come from [install archive digests](../../../conformance/fixtures/install-archives/digests.json)
and [S1 computed digests](../../../conformance/fixtures/s1/README.md#computed-digests).

Names quoted as they stand in [names.yaml](../../../spec/names.yaml): core package
`github.com/shrav-k/conduit/core`, apiVersion `github.com/shrav-k/conduit/v0`, metadata prefix
`io.github.shrav-k.conduit/`, `CONDUIT_URL`, `CONDUIT_IDENTITY_DIR`, `CONDUIT_STATE_DIR`, `ca.crt`,
`jwks.json`, `engine-id`, `token`, `tls.crt`, `tls.key`. `tools/set_names.py` does not rewrite
these quotes or the cases under docs; maintain them explicitly when the assigned names change.

## Safety cases for 052a's convergence check

Order 069b was K3b and wrote none of these cases. The 18 safety cases below, a cascade case
and cases for `--adopt`, `--recreate` and `--prune` have no case file and no order. They are to
be fixed inputs of the convergence check that is order 052a's done-when
([decision D26](../../proposals/full-review/decisions.md#the-s1-rerun-and-its-report)).

Adapted from the [synthesis cases 1–18](../../proposals/language-shape/short-form/synthesis.md#golden-plan-safety-cases).
These are specified outcomes, not results from an implementation. No interactive confirmation
or catch-all approval or control-takeover flag is part of batch 1.

| # | Trigger | Rule and outcome |
| --- | --- | --- |
| 1 | Outside closure, then an edit | LIF-041 and ruling 76 point 1: stop without reopen; editing cannot lift. |
| 2 | Grant removed/re-added under another key | LIF-041 matches consumer/local name or target/capability; stop without reopen, even when renamed in the same apply or a later one. |
| 3 | Agent retired by another actor | Three-way comparison, LIF-040: stop without recreate; no unrelated apply resurrects it. |
| 4 | Records purged | LIF-040 keeps tombstones apart from records; still stop. |
| 5 | Another copy narrowed a grant | Drift comparison stops a stale widening; replace alone does not take over drift. |
| 6 | Decoy between register and open | Verify returned ID; open expectedId conflicts on a different ID (LIF-037); no decoy opened. |
| 7 | Narrowing after printing, before dispatch | Reinspection and revisions catch observable changes; O2 records the unprotected same-ID race. There is no prompt interval. |
| 8 | Interrupted T4 | Close both before reopening; B read before A write; no state lets both write. |
| 9 | Optional input or tools map changes on upgrade | Resolved contract/implementation diff is widening; stop without replace; PKG-031's list alone is insufficient. |
| 10 | Outside Conduit targets replaced Resource | Print it and its lost bound coverage; stop without separate permission/desired replacement. |
| 11 | Unknown write on shared before T5 | LIF-013 and OPS-012 to OPS-014: stop until explicit reconciliation. |
| 12 | Placement with two nodes | Left out: second-node enrollment follows the rerun. Do not invent replacement placement behavior. |
| 13 | Change a first-start/default root Conduit | ACC-024: refuse before sending closure. Ordinary root-consumed Conduits follow replacement rules. |
| 14 | Generated-name collision | Translator's SCT-013, not a planner golden case here. |
| 15 | Value at a secret-reference place | Check locally during translation/resolution before any step (SCT-014, SEC-001). A literal or bare name stops with status 2: `plan`, one `stop KIND/NAME place=POINTER SEC-001` per invalid place, then `flags []`, then `stopped`; no value or part of it, step, detail or difference, and nothing kept or sent (SEC-003). A valid Resource reference is printed, kept in `.conduit/state.yaml` and compared as written, `{kind, name}`, with its resolved ID as inspection shows it (OBJ-009, SEC-004); permitted null, as for S1's `apiKey`, is not a reference. |
| 16 | Caller has only Pool authority | Engine inspection LIF-013/ACC-033, not this root CLI: no Agent inspection, no hidden counts; full inventory of inspectable objects. |
| 17 | Setup raises sign-in | Setup returns pendingAction; ruling 76 point 4 continues apply, prints action IDs for the panel and answers nothing (TPL-012, PND-005, PND-011). |
| 18 | Pin downgrade | Same resolved diff and replace permission as upgrade; no special downgrade bypass. |

## Open points

O1: Decided for v0. A project with a relative `fetch` or a program given as a relative path
cannot be applied and must be packaged. The plan stops with status 2, naming the document and
the place. No submitted directory is on this wire. A request that hands over values rejects a
relative `fetch` or program (SUB-010; `replace.v1`). Making these values absolute would change
the digest SUB-008 takes of the manifest as handed over, and lose PRV-007's guard that a relative
program runs only on the engine's own runner (SUB-010). Do not silently absolutize or invent an
input; S1 uses neither.

O2: Open. LIF-037 pins identity, not definition/lifecycle revision for open/setup. F6 inspection is
possible but cannot promise atomic validation against a concurrent same-ID change. The reference
stops on detected drift, and makes no stronger engine promise.

O3: Answered by [ruling 59 A3](../../proposals/ruling-59/README.md). A cascade takes a flag that
opens, uses and closes a one-time Conduit, printed in the plan. That Conduit is the one stated
exception to narrowing before widening and to the subset rule.

O4: Decided. CON-008 closes and removes the one-time cascade Conduit as soon as its target retires.
A later close.v1 would name a removed object and be invalid (LIF-035). Printing the engine's closure within the
cascade under `affects` instead of sending another close is the reading under CON-008.
`spec/README.md` still lists "close the Conduit" among the cascade's requests, contradicting
CON-008; the next specification order corrects it
([decision D86](../../proposals/full-review/decisions.md#the-state-of-the-documents)).
The specification is not amended here.

O5: Decided. Check every place governed by `secret-reference.v1` locally during translation and
resolution before printing any step (SEC-001). A literal or bare name stops with status 2:
`plan`, one `stop KIND/NAME place=POINTER SEC-001` for each invalid place, then `flags []`,
then `stopped`.
No line holds any part of the value (SEC-003); there is no step, detail or difference, and
nothing is kept or sent. A valid value is a reference to a Resource, never the secret itself:
print, keep in `.conduit/state.yaml` and compare it like any other reference, as written,
`{kind, name}`, with its resolved ID as inspection shows it (OBJ-009, SEC-004). Where permitted,
null, as for S1's `apiKey`, is not a reference. Public absolute paths use the fingerprint defined
above, so their edits do not collide in plan keys.

O7: Decided. The planner reads package contents from the person's package cache, keyed by the
archive's digest, as Go's module cache, Terraform's provider cache and npm's cache are. This
needs no engine operation and discloses nothing new: the pin already names the bytes (PKG-004).

`conduit install ARCHIVE#HEX` keeps a copy of the archive in the cache under HEX once the bytes
hash to HEX, whatever `install.v1` then answers: a copy named by its digest is never wrong.
Nothing else writes the cache; plan and apply only read it. The cache belongs to the person
on that machine and lies outside every project and its `.conduit/`, so every working copy
shares it. Order 052a fixes its directory with O8's context data. It holds archives only: no
credential, and nothing from the engine's store.

The planner reads the archive of each package the manifest pins, of each package that an
archive's manifest lists, through every dependency, and of each release an object holds that
the comparison of row 19 needs. It hashes the bytes it read and uses them only where the hash
is the digest. Rows 1 and 2 of the difference table give the stops.

The planner never fetches: not a pin's `fetch`, not through a package-source provider, and not
from the engine's store. A package that the person installed with `conduit install` on this
machine after the cache existed, from any of their working copies, is in the cache, since
every working copy shares it. A miss is only a package that no `conduit install` of theirs on
this machine has kept: one installed from another machine, one a provider fetched, or one
installed before the cache existed. Running `conduit install` with its archive fills the
cache and changes nothing in the engine (PKG-027).

The core package has no archive (NAM-009): the planner uses the core contents of its own build,
as the engine does. So a CLI of another version than the serving engine stops with status 2
before it sends any request, naming both versions; order 052a fixes how the CLI learns the
serving engine's version, with O8's context data.

O8: Open. Ruling 76 point 3 fixes how commands choose the engine: `conduit serve` records the engine
it serves as the person's current context, and `--store` overrides it. Order 052a still needs
to specify the local context's persistence/discovery and unavailable-context diagnostics. No
additional context command is proposed here. Context data stays outside the project's kept state.

## S1 commands and counts under ruling 76

This schedule uses the current engine context and the current project directory throughout.
`conduit serve` is harness work and records that context before the tasks; it is excluded, as in
[experiment 023](../../../experiments/023-paper-count/results-2026-11-11.md#the-counts).
Init and install are listed only for accounting; their behavior is outside this reference,
apart from the copy `conduit install` keeps in the package cache (O7).
Plan is optional and absent from the task schedule. T3 reads declarations without a command;
its golden plan demonstrates a no-op, rather than a required inspection step.

```text
T1: conduit init
T1: conduit install conformance/fixtures/install-archives/filesystem-0.1.1.tar.gz#eaa13cfee0f2499c1f8dfaf929cdcf7632f3cd3f034d405860411c49dab80614
T1: conduit install conformance/fixtures/install-archives/agents-0.2.0.tar.gz#479ef47083c8b151eb9cabf54ee500e9ed1972e4293393dc17bbad2da6e07c71
T1: conduit apply
T2: conduit install conformance/fixtures/s1/archives/filesystem.tar.gz#200de7c103aa560201dab8821e974088ce470e76c859c86aa01605cdf72d2160
T2: conduit install conformance/fixtures/s1/archives/history.tar.gz#e946746509490be49cbcc07d41ff5fed24a6e6c8b377bf423c77f79c02502f69
T2: conduit apply
T3: (none)
T4: conduit apply --replace agent-a--project
T5: conduit install conformance/fixtures/install-archives/filesystem-0.2.1.tar.gz#f01c1a0a4bc864cb3593f8fd9f29e76ba25987be30e367f1e9033491cacd5614
T5: conduit apply --replace shared
T6: conduit apply --replace shared
T7: conduit apply
T7: conduit apply --reopen agent-a--project
```

History's filesystem 0.1.0 dependency is installed explicitly at T2, before history; nothing
moves that command into setup. This resolves the old O6 accounting question for this schedule
using experiment 023's retained dependency install. Generated-name operands change the restoration
text, but not its line count. Narrowing B at T4 removes one named flag use, while A's widening
still makes that command count. Bare T7 revocation carries no author-chosen value.

Primary counts only commands carrying an author-chosen value; strict command counts include
every scheduled command ([ruling 63, point 1](../../proposals/ruling-63/README.md)).
The file hunk costs below come from experiment 023's flow states, not the compact serialization
inside these cases. Install-written pins and copied digests are listed apart in that experiment.
The task's strict total also includes panel actions, following its counting method.

| Task | Command lines, primary | Command lines, strict | Authored file lines | Panel actions apart | Total primary | Total strict |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| T1 | 2 | 4 | 19 | 2 | 21 | 25 |
| T2 | 2 | 3 | 3 | 0 | 5 | 6 |
| T3 | 0 | 0 | 0 | 0 | 0 | 0 |
| T4 | 1 | 1 | 2 | 0 | 3 | 3 |
| T5 | 2 | 2 | 0 | 0 | 2 | 2 |
| T6 | 1 | 1 | 1 | 0 | 2 | 2 |
| T7 | 1 | 2 | 1 | 0 | 2 | 3 |
| Total | 9 | 13 | 26 | 2 | 35 | 41 |

The total primary sequence `21, 5, 0, 3, 2, 2, 2` agrees with experiment 023's row
"No store + K3 A's omitted narrowing flags" and ruling 76's forecast. Strict remains
`25, 6, 0, 3, 2, 2, 3`: omitting values or flags removes no scheduled command or panel action.
The two T1 panel actions are sign-in for A and B, listed apart from authored lines
([ruling 68 R1](../../proposals/ruling-68/README.md)). Apply continues through both pending
sign-ins, without a follow-up apply or an answer. A returned pendingAction is a list of IDs,
never evidence or a prompt, under `io.github.shrav-k.conduit/pendingAction` (IFC-025).
These actions are prospective for real Agent packages; fixture agents 0.2.0 raise none.
All counts remain a paper forecast, not a fresh-author result or a live sign-in run.

### Case command and output counts

These counts describe the literal cases, separately from the full task schedule above.
Every command uses the current engine context. The optional no-op plan and the stopped
restoration are demonstrations, excluded from the task schedule. Flag uses count each named
permission once, even when several request steps carry it. Plan lines are the recorded text's
lines, including the first and last; stdout also includes result lines. A null record path in
cases 03 and 09 means no file is written, although the expected plan text and digest are stated.

| Case | Command primary/strict | Named flag uses | Requests after printing | Plan lines | Stdout lines |
| --- | ---: | ---: | ---: | ---: | ---: |
| [01-t1](cases/01-t1.yaml) | 0/1 | 0 | 15 | 37 | 52 |
| [02-t2](cases/02-t2.yaml) | 0/1 | 0 | 7 | 20 | 27 |
| [03-t3-noop](cases/03-t3-noop.yaml) | 0/1 | 0 | 0 | 2 | 2 |
| [04-t4](cases/04-t4.yaml) | 1/1 | 1 | 10 | 20 | 30 |
| [05-t5](cases/05-t5.yaml) | 1/1 | 1 | 11 | 26 | 37 |
| [06-t6](cases/06-t6.yaml) | 1/1 | 1 | 11 | 26 | 37 |
| [07-t7-revoke](cases/07-t7-revoke.yaml) | 0/1 | 0 | 2 | 8 | 10 |
| [09-t7-restore-unflagged](cases/09-t7-restore-unflagged.yaml) | 0/1 | 0 | 0 | 5 | 5 |
| [08-t7-restore](cases/08-t7-restore.yaml) | 1/1 | 1 | 4 | 12 | 16 |

Ruling 76 changed the hashed plan text of cases 04, 07, 08 (successful restoration) and 09
(stopped restoration). The `ownTools` lines changed case 01's plan text. Each changed digest
was re-derived with Python `hashlib.sha256` from the exact UTF-8 plan bytes; their paths, request
keys and later ledger/journal references changed with them.
Sending templates first changed the plan text of cases 05 and 06, and their digests were
re-derived the same way. Cases 02 and 03 keep their plan digests: command spelling, source
citations and case order are outside the hash. No package, file or path fingerprint digest
changed. Case 09 now precedes case 08 in the documented sequence so all nine cases form one
applied/kept chain.
