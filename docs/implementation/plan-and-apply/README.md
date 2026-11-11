# Plan and apply: the CLI reference

Not binding. The specification in spec/ alone binds.

This is the proposed CLI reference for spec batch 1, K3, with hand-written golden cases.
It describes tooling, not additional language promises or engine behavior. Its five approval
and command readings follow [ruling 76, points 1–5](../../proposals/ruling-76/README.md).
A builder follows the specification where this reference and it differ. No program has produced
these expected plans. A later order supplies the remaining safety cases and a cascade case.
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
commands. Installation happens beforehand; plan reads installed release identities. Obtaining
their contents for resolution remains open point O7; installation alone does not supply that input.

Both commands are requests of a serving engine, `conduit serve`, as root, like
[today's author commands](../../../engine/README.md#change-a-serving-project). Neither starts
an engine or installs a package. Their wire documents are framed (SCT-001, SCT-003); they use
only [core operations](../../../spec/core/operations.yaml), adding no operation or promise
([ruling 53, point 4](../../proposals/ruling-53/README.md), SCT-015 and the head comment of
[sections.yaml](../../../spec/promises/sections.yaml)).

## Read, compare, print, keep, send

1. Read the manifest and its documents, translate sectioned documents (SCT-003, SCT-004), and
   resolve against package contents matching the installed release identities, where those contents
   are available. Their CLI-side source is unspecified (O7); list and inspect do not supply them.
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
   require package contents are conditional on that input (O7); do not infer equality from identities
   alone. Compare actual resolved operations even when standard templates should keep their lists
   (PKG-031). If required contents are unavailable, print the unresolved input and stop before
   sending changes, with status 2.
4. Build the deterministic plan below, or a stopped plan with reasons. `plan` prints it and exits:
   it sends only `list.v1` and `inspect.v1`, writes no local files, and sends no mutation.
5. `apply` prints the identical plan before any change. If stopped, exit without recording or
   sending changes. Otherwise durably keep its exact text and digest before sending step 1.
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
`--reopen` lifts an outside-apply Conduit tombstone before registering/opening its desired grant;
`--recreate` does that for another instance; `--adopt` takes over a foreign existing object into
this working copy's ledger only after its ID, resolved definition and held packages are printed
and verified. Adoption does not overwrite, lift, open or set up by itself: any such act needs its
own permission. Nothing besides adopt takes over a foreign object (synthesis F6).
`--replace` includes incident Conduits that must be closed and recreated to replace its named
endpoint, and the necessary inline-template update. It does not authorize unrelated replacements,
outside tombstones, or retargeting outside-file Conduits. An outside-file Conduit needs its own
named replacement permission and explicit desired definition; otherwise the plan stops.
Neither flag-free closure nor `--prune` lifts its resulting tombstone.
`--with-resources N` and `--with-agents N`
permit only the named container's printed cascade. None of the flags bypasses engine access.

## Ordering and access

The limits say: "Narrowing never waits. A plan prints and counts every widening, including one
caused by moving a package pin or changing a reusable template's operations (PKG-031)" and
"the plan still checks the operations that actually resolve"
([specification](../../../spec/README.md#what-the-short-forms-meaning-covers)).

Reading adopted here: every narrowing step precedes every widening step. Access after each step,
as a whole, is a subset of access before apply or a subset of the access requested by the files.
For T4 close both project Conduits, restore B's read, then open A's write: no intermediate state
lets both write. Register inert documents between these phases where dependencies need them.
Root defaults added during registration count as added access and must follow all narrowings.
This is our ordering choice, from "It narrows before it widens" in the
[accepted simplification, point 4](../../proposals/simplification/README.md).
That proposal's make-before-retire replacement was reversed by ruling 55 A3.

Our choice on an incomplete approval: print the whole stopped plan and send no part, including
narrowings, when any required flag is missing. This does not defer an independently requested
narrowing behind a prompt: no execution starts for an invalid command. A plan containing only
flag-free narrowings is valid without flags, under ruling 76 point 1. Do not run a narrowing subset whose
removal could invalidate the wider plan that the command purported to approve.
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

A nested instance is different (LIF-009). For a declared Conduit or a Pool entry's Resource,
print `management.register-replacement.v1 ParentKind/parent expected=ID neither flag=... LIF-009`,
with `nested=member` in the detail. It generates a new name and ID; show its later references as
`Kind/(step N).object expected=(step N)`. It lifts no tombstone, and any matching tombstone must
already have been lifted by an independently permitted step. Old retired nested names remain
occupied until removed. The local name of a declared Conduit remains unchanged.

A tombstone made outside this apply needs reopen (Conduit) or recreate (other instance), even
if a line was renamed or records purged (LIF-040, LIF-041). An interrupted apply's own journal
may establish the same apply and cause; otherwise require the outside permission.
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

This necessary scoped permission is an ordering exception: its temporary management access must
open before the cascade narrows; list it explicitly, never hide it as provider work. It grants
only retirement, not data/model operations. The global access-subset rule above cannot literally
include this temporary permission; O3 records that limitation rather than claiming it passes.
The one-time name is a deterministic unused `cascade-TARGET` name (truncate and add a digest
suffix on collision); it is not an engine-made default. Register it from core `retire-pool` or
`retire-fleet`, consumer root, target the named container, `as` the one-time name. The schema of
the cascade takes no expectedId; the expected field printed at step 4 is a checked observation,
not a new wire argument. After successful retirement CON-008 has closed/removed this Conduit
as part of step 4.
That is the close printed under affects: sending another close would be invalid. Step 5 reads
its tombstone and step 6 lifts it. This is the nearest sound reading, not the literal five
requests of C13 (O4). If the cascade does not succeed, the ordinary stop rule applies. The
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
Keep no absolute path, store path, credential, or value at a place governed by a secret reference.
Use relative file origins and typed redacted markers for those values in saved snapshots, with
in-memory comparisons against freshly read files. For a non-secret absolute path, retain only
`{pathSha256: HEX}`, the SHA-256 of its UTF-8 bytes, and print `root-sha256=HEX` in the plan.
This is a CLI comparison fingerprint, not a new wire input; it makes T6 distinct without storing
an absolute path. Never fingerprint a credential or a secret-reference value. SEC-003 and
SEC-004 bind diagnostics and records only; applying the same protection to this local state is our privacy choice by analogy.
For a secret reference, omit the authored value entirely rather than hashing it. No raw wire
envelope or returned CloudEvent is persisted if it violates that policy. Keep safe
projections of results; reread a required omitted value next time. O1/O5 record the relative-path
and secret comparison limitations; never pretend a redacted snapshot proves equality.

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
resolved operation set and package identity where relevant. This distinguishes otherwise similar
plans without disclosing prohibited values. Full arguments are journaled only in safe form.
Affects lines follow all steps, then differences, then flags sorted by flag and operand.
The last line is `ready`, `no changes`, or `stopped`.

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
No-op text is exactly `plan\nno changes\n`; a no-op leaves kept state unchanged.

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
A later order adds cascade and the other flags' cases.

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
not invented empty record lists. All nine cases assume the planner is separately supplied with
the repository's fixture package contents matching every listed address/version/digest, including
transitive dependencies. Schemas, defaults, operation contracts and implementation maps come from
those fixture contents. This is an explicit paper-case input assumption, not a claim that list or
inspect returns package contents or that the CLI can obtain them today (O7). The compact `engine`
notation lists identities only; it does not encode this additional content input.

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

## Safety cases deferred to K3b

Adapted from the [synthesis cases 1–18](../../proposals/language-shape/short-form/synthesis.md#golden-plan-safety-cases).
These are specified outcomes, not results from an implementation. No interactive confirmation
or catch-all approval or control-takeover flag is part of batch 1.

| # | Trigger | Rule and outcome |
| --- | --- | --- |
| 1 | Outside closure, then an edit | LIF-041 and ruling 76 point 1: stop without reopen; editing cannot lift. |
| 2 | Grant removed/re-added under another key | LIF-041 matches consumer/local name or target/capability; stop without reopen, even when renamed. |
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
| 15 | Value at a secret-reference place | Translator's SCT-014/SEC-003; planner/local-state privacy also omits it. |
| 16 | Caller has only Pool authority | Engine inspection LIF-013/ACC-033, not this root CLI: no Agent inspection, no hidden counts; full inventory of inspectable objects. |
| 17 | Setup raises sign-in | Setup returns pendingAction; ruling 76 point 4 continues apply, prints action IDs for the panel and answers nothing (TPL-012, PND-005, PND-011). |
| 18 | Pin downgrade | Same resolved diff and replace permission as upgrade; no special downgrade bypass. |

## Open points

O1: A document-carrying request rejects relative fetch and relative program values (SUB-010;
`replace.v1`; today's shell reference). No submitted directory is on this wire. Such a project
cannot be applied as written. Do not silently absolutize or invent an input; S1 uses neither.

O2: LIF-037 pins identity, not definition/lifecycle revision for open/setup. F6 inspection is
possible but cannot promise atomic validation against a concurrent same-ID change. The reference
stops on detected drift, and makes no stronger engine promise.

O3: The literal global narrowing/subset condition conflicts with opening a temporary cascade
Conduit. The ordering chapter gives the data-access interpretation; the owner/admin must decide
whether that exception is intended before a cascade golden case claims compliance.

O4: CON-008 closes the one-time cascade Conduit as soon as its target retires. A later close.v1
would name a removed object and be invalid (LIF-035). Printing the engine's closure within the
cascade instead of a second request is the nearest sound reading of plan C13; the specification is not amended here.

O5: Privacy removes secret-reference values from the local baseline. A redacted snapshot cannot
prove their equality. This planner neither adopts changed secret bindings nor reverts them: print
a redacted unresolved difference and stop when equality would be needed. A safe, durable comparison
method for those values is open; S1 uses null apiKey and no secret reference. Public absolute
paths use the fingerprint defined above, so their edits do not collide in plan keys.

O7: Package contents are a missing planner input. `list.v1` exposes installed package identities
as address/version/digest (OPS-036); `inspect.v1` reads a template or instance and lists the packages
it holds (LIF-013). Neither operation supplies installed package archives or manifests. Translation
and resolution (SCT-003), and contract/implementation comparisons on pin changes (PKG-031), need
contents beyond those identities. No verified CLI-side source for those contents is defined here;
do not assume access to the engine's private store or add a read operation. Resolution and those
comparisons remain conditional on separately supplied matching contents; without them the plan
prints the missing input and stops with status 2. The nine paper cases assume fixture contents
are supplied as described above. The admin must settle the input/disclosure before a builder can
implement these rules for arbitrary installed releases.

O8: Ruling 76 point 3 fixes how commands choose the engine: `conduit serve` records the engine
it serves as the person's current context, and `--store` overrides it. Order 052a still needs
to specify the local context's persistence/discovery and unavailable-context diagnostics. No
additional context command is proposed here. Context data stays outside the project's kept state.

## S1 commands and counts under ruling 76

This schedule uses the current engine context and the current project directory throughout.
`conduit serve` is harness work and records that context before the tasks; it is excluded, as in
[experiment 023](../../../experiments/023-paper-count/results-2026-11-11.md#the-counts).
Init and install are listed only for accounting; their behavior is outside this reference.
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
| [01-t1](cases/01-t1.yaml) | 0/1 | 0 | 15 | 35 | 50 |
| [02-t2](cases/02-t2.yaml) | 0/1 | 0 | 7 | 20 | 27 |
| [03-t3-noop](cases/03-t3-noop.yaml) | 0/1 | 0 | 0 | 2 | 2 |
| [04-t4](cases/04-t4.yaml) | 1/1 | 1 | 10 | 20 | 30 |
| [05-t5](cases/05-t5.yaml) | 1/1 | 1 | 11 | 26 | 37 |
| [06-t6](cases/06-t6.yaml) | 1/1 | 1 | 11 | 26 | 37 |
| [07-t7-revoke](cases/07-t7-revoke.yaml) | 0/1 | 0 | 2 | 8 | 10 |
| [09-t7-restore-unflagged](cases/09-t7-restore-unflagged.yaml) | 0/1 | 0 | 0 | 5 | 5 |
| [08-t7-restore](cases/08-t7-restore.yaml) | 1/1 | 1 | 4 | 12 | 16 |

Ruling 76 changes the hashed plan text only in cases 04, 07, 08 (successful restoration) and 09
(stopped restoration). Python `hashlib.sha256` re-derived those four digests from the exact
UTF-8 plan bytes; their paths, request keys and later ledger/journal references changed with them.
Cases 01, 02, 03, 05 and 06 keep their plan digests: command spelling, source citations and case
order are outside the hash. No package, file or path fingerprint digest changed. Case 09 now
precedes case 08 in the documented sequence so all nine cases form one applied/kept chain.
