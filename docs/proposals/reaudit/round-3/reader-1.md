# Round 3, reader 1: duplicates and contradictions

I read all of `spec/` and nothing else: 129 files, which are `README.md`, `definitions.md`,
`names.yaml`, `schema-profile.md`, `implementation-defined.yaml`, the 22 promise files with
their 424 promises, the 9 files of `spec/core/`, the 10 files of `spec/schemas/` and the 83
example files. Two small read-only scripts counted the promises, checked that every cited
promise and DL item exists, and compared the core package's tables with its documents. I have
16 findings: 1 `unsafe`, 3 `contradiction`, 2 `drift`, 1 `citation` and 9 `minor`. The one
`unsafe` finding is a disclosure in a narrow window, not a grant of access. Most of the text is
consistent, and the last section lists what I checked without a finding. Nothing here repeats
the four points that were given to me as open.

## Findings

### 3A-1. A request whose permission cannot be decided reports conflict or invalid first

Severity: `unsafe` (disclosure to an actor that was never found to be permitted).

Involved: OPS-023, ACC-037, OBJ-025, REV-002, and in a weaker form ACC-014 and ACC-015.

Colliding text:

> OPS-023: "It reports one reason, chosen in this order: invalid if the request cannot be read;
> otherwise refused, then invalid, then conflict, then unavailable."

> OPS-023, observable: "A request that is not permitted reports refused whatever else is wrong
> with it, and its result says nothing else about the objects it names."

> ACC-037: "the state of the Conduits or of the policies that the decision needs cannot be
> established" ... "The request is not dispatched, and reports unavailable."

> OBJ-025, observable: "The result carries the diagnostic that names both objects".

Situation. An actor that is the consumer of no Conduit to `management` sends `register.v1`
with a Pool document named `team`, at a moment when the engine cannot establish the state of
the Conduits. A Pool `team` exists. Refused does not apply, because the access decision was
not made. Two reasons apply: conflict under OBJ-025, and unavailable under ACC-037. OPS-023
puts conflict before unavailable, so the result reports conflict and carries a diagnostic that
names the existing object. The same holds for `rename.v1` with a stale revision (REV-002).
Had the state been established, the actor would have been refused and told nothing. The
order protects the objects only when permission can be decided, and gives them away exactly
when it cannot. In the weaker form, a caller whose request a policy may forbid, and whose
policy cannot be applied (ACC-015), is told conflict in place of unavailable.

Least change. One sentence in the `then` of OPS-023: where the access decision cannot be made
under ACC-037, or a restriction cannot be evaluated or applied under ACC-014 or ACC-015, the
request reports unavailable and no reason that ranks after refused is reported. ACC-037's
observable can then cite it.

### 3A-2. Opening an open Conduit whose consumer is retired: succeeded or conflict

Severity: `contradiction`.

Involved: ACC-006, CON-031, and the description of `open.v1` in `spec/core/operations.yaml`.

Colliding text:

> ACC-006, observable: "A request to open a Conduit that is already open is decided like any
> other, changes nothing and has the outcome succeeded"

> CON-031: "An actor asks to open a Conduit whose consumer or whose target is retired." ...
> "The request is not dispatched, and reports conflict."

The description of `open.v1` states both in two neighboring sentences, with no order between
them.

Situation. Agent `a` is the consumer of the open Conduit `c`. An invocation of `retire.v1` for
`a` fails, so `a` is retired with its shutdown pending, and CON-008 leaves `c` open. An actor
with an unlimited right to open asks for `open.v1` naming `c`. ACC-006 gives the outcome
succeeded. CON-031 gives the reason conflict, and its observable says only that such a Conduit
"is as ACC-039 states", which is about requests through it. OPS-023 orders reasons against
each other, and not a reason against an outcome.

Least change. Add to CON-031's observable that the rule holds for a Conduit that is still
open under ACC-039 too, and add "unless one of its ends is retired" to the sentence of ACC-006
and to the description of `open.v1`.

### 3A-3. A Resource for a retired Pool: instantiated, or conflict, or invalid like its siblings

Severity: `contradiction`, with a drift among three sibling rules.

Involved: POOL-004, POOL-017, LIF-009, PLC-018, CON-025, OBJ-023.

Colliding text:

> POOL-004 (`language`): "The Resource is instantiated like any other instance and is owned by
> that Pool."

> POOL-017 (`convention`): "The request is not dispatched, and reports conflict."

> POOL-017, observable: "a retired Pool gains no Resource, as a retired location is given no
> instance under PLC-018 and a retired end no Conduit under CON-025."

> OBJ-023: "for any cause other than the clash with an existing name that OBJ-002 states" ...
> "reports invalid."

Situation, first part. A Resource document writes `owner: team`, and the Pool `team` is
retired. No promise of the layer `language` rejects the document: POOL-001 asks only that the
owner be a Pool instance, and the `then` of POOL-004 says the Resource is instantiated. Its
observable points at POOL-017, whose `then` says nothing is registered. An engine of the
Engine level keeps both layers and cannot keep both `then`s. An engine of the Language level
returns a Resource for documents that an engine of the Engine level refuses. The `then` of
LIF-009 collides with POOL-017 in the same way for an entry of a retired Pool.

Situation, second part. POOL-017 presents itself as the same rule as PLC-018 and CON-025.
Those two are of the layer `language`, reject the document, and so report invalid under
OBJ-023. POOL-017 reports conflict. Registering an Agent placed on a retired location reports
invalid, registering a Conduit with a retired end reports invalid, and registering a Resource
owned by a retired Pool reports conflict.

Least change. Add "that is not retired" to the `when` of POOL-004, and say in POOL-017 that
it differs from PLC-018 and CON-025 in layer and in reason. Or make all three one kind of
rule. The owner picks the reason the family reports.

### 3A-4. `#name` written as `from`: the `then` and the observable name different templates

Severity: `contradiction` (a `then` against the observable of the same promise).

Involved: INS-025, NAM-022, and the row `#name` of `spec/definitions.md`.

Colliding text:

> INS-025, `then`: "or `#` followed by a name, for a template of the same project or package as
> the document that holds it"

> NAM-022, `then`: "It refers to the document, or the implementation, of that name in the same
> project, or the same package, as the document that holds it."

> NAM-022, observable: "it is never looked up among the objects that exist or in another
> package. There is one exception. ... `#name` written as `from` resolves as the bare name
> does under INS-025: among the templates that exist"

Situation. An Agent template `reviewer` exists, registered earlier from another project. A new
project submits an Agent instance document with `from: '#reviewer'` and holds no template of
that name. By both `then`s the reference is to a template of the same project, there is none,
and INS-002 rejects the document. By both observables it resolves to the existing template and
the instance is made. The sentence of INS-025 that "The same documents then read alike as a
project and as a package" does not hold in this case either: as a package the documents are
rejected.

Least change. Move the exception into the `then` of NAM-022, and word the fourth form in the
`then` of INS-025 as "for a template of the same package or, in a project that is not
published, for a template that exists".

### 3A-5. A result with no outputs whose Conduit closed: withheld and an error, or neither

Severity: `drift`.

Involved: ACC-034, IFC-025, and through them LIF-021 and CON-008.

Colliding text:

> ACC-034, observable: "the result carries its outcome and no `structuredContent`, its `_meta`
> holds `io.github.shrav-k.conduit/withheld` with the value true, and its `isError` is true"

> IFC-025, observable: "the result of an operation that declares none is never marked as
> withheld, and where its outcome is succeeded its `isError` is not true, even where its
> Conduit was closed before it ended."

Situation. `retire-with-resources.v1` declares no outputs. An actor requests it through its
Conduit to the Pool `team`. Every Resource confirms its shutdown, so under LIF-021 "the Pool's
retirement is dispatched as an invocation of `retire.v1` of its own". The Pool is retired from
that moment (LIF-005), and CON-008 closes every Conduit to it "at once", the actor's among
them, while the invocation of `retire-with-resources.v1` has not ended. ACC-034 then applies.
Its observable makes the result an error marked as withheld. IFC-025 says it is neither. So
every successful request for this operation meets the difference, and a consumer that reads
`isError` reads a success as an error under one of the two texts. The exception of IFC-025 is
itself stated only in an observable.

Least change. Add to the observable of ACC-034: "where the operation declares outputs; what
the result of an operation that declares none carries is IFC-025". Move the exception into
the `then` of IFC-025.

### 3A-6. The check that an implementation states a requirement misses a carried restriction

Severity: `drift`. This is the one drift with a bearing on access.

Involved: CON-022, ACC-015, ACC-018, ACC-017, ACC-014, IFC-007.

Colliding text:

> ACC-015, `when`: "A policy applies to a request under ACC-031, and the implementation that
> the request's target has selected for the operation does not state the policy's requirement
> in `supports`."

> ACC-018, observable: "A step whose implementation is marked `plain`, or cannot apply one of
> those restrictions, reports unavailable, as ACC-014 states."

> ACC-031, observable: "an onward request that serves such a request carries the policy under
> ACC-017, whatever its own target."

Situation. An Agent requests an operation of the Resource `r1` through a Conduit that carries
a filter. To serve it, `r1` makes an onward request through its own Conduit, which carries no
filter, to the Resource `r2`, whose selected implementation is marked `plain`. Under ACC-017
the onward request carries the filter. CON-022 looked only at the filters of the Conduit to
`r2`, of which there are none. ACC-015 speaks of a policy that applies under ACC-031, and
neither of a carried filter nor of a policy that is carried without applying to `r2`. The
sentence of ACC-018 is about a step with no `conduit`. IFC-007 sends `r2` no restrictions.
What still forbids the call is general: ACC-014 ("cannot be evaluated") and the `then` of
ACC-017. Neither says that the engine checks `supports` or `plain` here. An engine that takes
ACC-015 for the whole of the engine's check dispatches the request with the filter unapplied.

Least change. Widen the `when` of ACC-015 to every restriction that IFC-018 has the engine
send, carried ones included, and to an implementation marked `plain`, and let ACC-018 cite it.

### 3A-7. `register-replacement.v1` that cannot make the instance has no reason

Severity: `minor`.

Involved: LIF-009, CON-025, PLC-018, PLC-004, OBJ-009, OBJ-023, POOL-016, POOL-017, and the
description of `register-replacement.v1`.

Colliding text:

> LIF-009: "A new instance is instantiated from the nested instantiation as the instance it
> belongs to holds it in its resolved definition" ... "The request is a registration"

> CON-025: "A Conduit is instantiated whose consumer or target is retired." ... "The Conduit is
> not instantiated."

> OBJ-023, `when`: "A document, the manifest or the submission that is handed over for
> registration is rejected"

Situation. The declared Conduit `checkout` of an Agent was closed because its target was
retired, which is the usual way a declared Conduit comes to be closed. An actor asks for a
replacement. The nested instantiation, as the Agent holds it, names the retired target by its
ID, so CON-025 does not instantiate the Conduit. No promise gives the reason. OBJ-023 speaks of
a document that was handed over, and none was. POOL-017 gives conflict for the like case of a
retired Pool, and LIF-009 gives invalid for a component Conduit. The same gap holds for an
entry whose stored placement is a retired location (PLC-018, with POOL-016: "a replacement
takes it"), for an entry that PLC-004 leaves without a placement, and for a target that has
since been unregistered (OBJ-009).

Least change. One sentence in LIF-009 that names the reason for a replacement that a rule on
instantiation does not make.

### 3A-8. A retired Pool cannot be asked for `retire-with-resources.v1` through `management`

Severity: `minor`.

Involved: ACC-033, `spec/core/capabilities.yaml`, `retire-with-resources.v1`, LIF-026, and the
row `management` of `spec/definitions.md`.

Colliding text:

> ACC-033: "Retiring a Pool together with the Resources it owns, in the one operation of
> LIF-021, is authorized against that Pool." ... "Where a Pool is retired, every request that
> this promise authorizes against that Pool is authorized against `management` in its place."

`object-management.v1` does not list `retire-with-resources.v1`, and the operation has no
input: its description says "Retire the Pool that is the target of the request".

Situation. The Pool's own `retire.v1`, dispatched within `retire-with-resources.v1`, fails.
The Pool is retired with its shutdown pending, and every Conduit to it is closed. By the
sentence of ACC-033 a further `retire-with-resources.v1` is authorized against `management`.
No Conduit to `management` can list it under CON-017, and the request could not name the Pool.
The way that works is `retire.v1` naming the Pool, under LIF-026.

Least change. Except this operation in the sentence of ACC-033 and in the definition, and say
that a retired Pool is retired again with `retire.v1`.

### 3A-9. A rename or a replace that changes nothing: the revision advances and must not

Severity: `minor`.

Involved: REV-004, OBJ-026, SUB-009, and the `effect` of `replace.v1` and of `rename.v1`.

Colliding text:

> REV-004 (`must not`): "An object's stored definition, configuration and name do not change."
> ... "Its revision advances."

> OBJ-026, `then`: "The revision advances, as it does for any stored change."

> SUB-009, observable: "After a replace the template's revision has advanced, under REV-003."

Situation. `rename.v1` gives an object the name it already has, which no other object has, so
OBJ-026 reports no conflict. Or `replace.v1` gives a document equal to the stored template.
Nothing that is stored changes. REV-004 forbids an advance, and says so of "a change that
confirmed the values already stored". OBJ-026 and SUB-009 say the revision advances.

Least change. Write "advances where the name changed" and "where the definition changed" in
the two promises and the two `effect`s.

### 3A-10. A graph written in place has nowhere to state `supports`

Severity: `minor`.

Involved: CON-022, ACC-015, ACC-018, ACC-038, TPL-003, `graph.v1` in
`spec/schemas/graphs.yaml`, `implementation-item.v1` in `spec/schemas/manifest.yaml`.

Colliding text:

> CON-022, observable: "The implementations that the target has selected for the operations
> the Conduit lists must always state the requirement"

> TPL-003: "Each value is either a string, which is the selection of an implementation, or a
> map, which is a graph written in place."

`graph.v1` holds `steps` and `outputs` and is closed. `supports` is a key of an implementation
item only.

Situation. A Resource template implements an operation by a graph written in place. A Conduit
template lists that operation and carries a filter. By CON-022 the implementation "does not
state it", so no such Conduit is ever instantiated, and by ACC-015 every request under a
policy is unavailable. Yet ACC-018 describes how a filter and a policy "limit every step of a
graph that implements the operation", which supposes that such a request is dispatched. No
text says what a graph written in place supports, or what ACC-038 names for it.

Least change. One sentence in CON-022: a graph written in place supports nothing, as PKG-019
says of an item that leaves `supports` out, or it supports what the implementations of its
steps support.

### 3A-11. The identity of a step outlives its run, and only the unknown case says what follows

Severity: `minor`.

Involved: OPS-028, OPS-021, OPS-009, OPS-010, OPS-024, OPS-008, GRA-027, DL1.

Colliding text:

> OPS-028: "derived from the ID of the invocation in which the step is first dispatched and
> from the step's name"

> OPS-010: "with different contents" ... "It is not dispatched, and reports conflict."

> GRA-027, observable: "a later run reads the configuration as changed."

> DL1: "what a re-run of setup or retire does with steps that already ran"

Situation. A step of a `setup` graph takes a path from `config` and fails. An actor changes
the path with a control, and the same actor asks for `set-up.v1` again. The step keeps the
identity of the first run and now has other arguments. OPS-021 applies OPS-012 to a step of a
later run, so the rules on repeats reach steps. If OPS-010 reaches this one, it reports
conflict in every later run, against what GRA-027 expects. With equal arguments OPS-024 would
dispatch a new attempt and OPS-009 would dispatch nothing for a step that succeeded, which
leaves DL1 nothing to choose. With another actor as the caller, OPS-008 scopes the identity
apart and the step is a new invocation. Two engines can read this in two ways.

Least change. Say in OPS-028 that the identity of the first dispatch is kept only by a step
whose outcome is unknown, and that any other step of a later run gets a new one. Or say that
OPS-009, OPS-010 and OPS-024 apply to steps, and take the item out of DL1.

### 3A-12. A Resource cannot say for which caller it retrieves a result

Severity: `minor`. On the strictest reading this is a gap and not a contradiction.

Involved: IFC-021, IFC-024, IFC-028, IFC-035, OPS-015, OPS-016, OPS-008.

Colliding text:

> IFC-021: "makes a request, other than one for `start-later-work.v1`, that names no invocation
> in `io.github.shrav-k.conduit/serving`" ... "The request is not dispatched, and reports
> refused."

> IFC-024, observable: "A call to it is decided under OPS-015 to OPS-017 and not under
> ACC-001."

> OPS-016, `when`: "for the result of an invocation that began with a request it sent itself,
> for the same caller."

Situation. A Resource that serves an invocation for an Agent sends an onward request with an
operation identity, as OPS-008 and IFC-020 provide. It then calls `retrieve-result.v1`. A
retrieval is not an onward request, and no promise says it names a served invocation, as
IFC-035 says of the other tool that it names none. Without the key IFC-021 refuses the call,
though OPS-016 says the instance "is told the invocation's state". With the key, IFC-028
refuses it once the served invocation has ended. An Agent that omits the key is its own
caller, which is not "the same caller", and is refused under OPS-015. So the result of an
onward request cannot be retrieved after the invocation it served has ended, and a Resource
may not be able to retrieve one at all.

Least change. Say in IFC-024 how the caller of a retrieval is established, and whether
IFC-021 and IFC-028 apply to it.

### 3A-13. The stop that OBS-007 has the engine send has no caller

Severity: `minor`.

Involved: OBS-007, IFC-006, REC-002, and the row `caller` of `spec/definitions.md`.

Colliding text:

> OBS-007: "The engine invokes the runtime's `stop` operation with the handle."

> IFC-006: "The request's `_meta` holds" ... "`io.github.shrav-k.conduit/caller`"

> Definition of caller: "The actor on whose behalf a request is made."

Situation. An implementation restores a runtime and its type does not state
`restoresRuntimes: true`. The engine dispatches `stop` to that implementation. No actor
asked. It is the only dispatch to an implementation outside the core package that no request
causes, and no text says which caller IFC-006 sends or what the record of REC-002 holds.

Least change. One sentence in OBS-007 on what the request carries and what its record says.

### 3A-14. Rules with a reason of their own that stand only in an observable

Severity: `minor`. No behavior is in doubt here, except where another finding says so.

Involved, each with the rule that its `then` does not state:

| Promise | The rule in its observable |
| --- | --- |
| LIF-007 | "A request names an object by a reference and never by an ID" |
| LIF-012 | `set-up.v1` or `retire.v1` that names a template or a Conduit reports invalid |
| OBJ-026 | A rename onto a name that is taken reports conflict; a bad name reports invalid |
| ACC-006, ACC-012 | Opening an open Conduit, and closing a closed one, succeeds |
| OPS-014 | Reconciling an invocation whose outcome is not unknown reports conflict |
| CON-008 | "A request to a retired target reports refused" |
| ACC-031 | The four causes for which `apply-policy.v1` reports invalid |
| ACC-035 | A request to apply a policy whose `actors` names `root` reports refused |
| PLC-016 | A value that names no location that exists reports invalid |
| CTR-003 | A capability outside the core package that lists a reserved operation is rejected |
| IFC-025 | The result of an operation that declares no outputs is never withheld |

Situation. Other texts cite these as promises. LIF-007 is cited for its sentence on requests
by `spec/definitions.md`, OBJ-009, `spec/core/references.yaml`, `spec/schemas/README.md` and
`spec/schemas/instances.yaml`, while its `then` is about a retired instance that still exists
and its layer is `language`. OPS-029 cites LIF-012 for a reason that LIF-012 gives only in its
observable. Rule 1 of `spec/README.md` is "One obligation per promise", and a vector written
from a `then` would not test these.

Least change. Give each rule a `then`, in its own promise or in the one it stands under. The
first two are cited most.

### 3A-15. Three `language` promises that only an operation can show

Severity: `minor` (layer).

Involved: OBJ-005, OBJ-010, INS-020.

Colliding text:

> `spec/README.md`: "Everything here is answered by resolving, which reads the documents
> against what already exists and runs nothing."

> OBJ-005, `when`: "An object is renamed."

> INS-020, observable: "A request to unregister a template that has instances does not report
> conflict on their account."

Situation. An engine that claims the Language level has no `rename.v1`, `replace.v1`,
`unregister.v1` or `inspect.v1`: OBJ-026, SUB-009, LIF-017 and LIF-013 are of the layer
`convention`. It cannot show that it keeps OBJ-005, OBJ-010 or INS-020, and the observable of
INS-020 states what a request reports. The second round moved the rename and the replace
themselves to `convention` and left these three.

Least change. Give the three the layer `convention`, or word each as a rule on what resolving
reads, such as "a stored resolved form is not resolved again".

### 3A-16. Three rows of the core package's table say "none" where the files cite promises

Severity: `citation`.

Involved: the table "Every export" of `spec/core/README.md`, the head of
`spec/core/results.yaml`, the head of `spec/schemas/values.yaml`, and one example comment.

Colliding text:

> `spec/core/README.md`: "| `diagnostic.v1` | Schema | none |", and the same for `id.v1` and
> `revision.v1`. "Where it says 'none', the export is a choice of the batch".

> `spec/core/results.yaml`: "diagnostic.v1  OBJ-008, NAM-008, PKG-006"

> `spec/schemas/values.yaml`: "id.v1, revision.v1      OBJ-005, REV-001"

Situation. A reader who asks which promise needs `diagnostic.v1` is told none by one file and
three by the other, and IFC-025 now gives `diagnostics` as "a list of diagnostics" in every
result that is not dispatched. One wording leftover belongs here too: the comment of
`spec/schemas/examples/invalid/INS-012-destination-operations-in-a-composite.yaml` says "a
template that is not composed", where INS-012 and the definitions say "composite".

Least change. Write the promises into the three rows, and "is not a composite" into the
comment.

I found no other citation that points at something missing or says something else. Every
promise ID and DL item cited anywhere under `spec/` exists. The 25 removed IDs appear only in
the comments at the heads of files, which name where each went.

## What I checked and found consistent

- Counts. `spec/README.md` against the files: 424 promises, the count of each file, 221
  `language`, 168 `convention` and 35 `interface`, and the files that hold a promise of
  another layer than their directory's. Every promise has all seven fields and an empty
  `vectors`.
- The core package against its own page: 99 exported documents, 58 of them Schema contracts,
  one implementation; the table of operations (capabilities, `revision`, `deduplicates`,
  `reservedToRoot`, `effect`); every core operation states `engine` and no `default`; each type
  requires exactly the operations of the capabilities it offers; `engine` implements all 21
  operations and supports `bound.v1`.
- ACC-033 against `spec/core/capabilities.yaml`, apart from 3A-8: every operation that is
  authorized against `management`, a Pool or `access` is in that target's capability, and
  `reconcile.v1` is in the three that hold management verbs.
- The shapes in the promises against the schemas: OBJ-012 to OBJ-014, OBJ-018, INS-012,
  INS-021, TPL-001, TPL-003, TPL-006, TPL-010, CTR-002 to CTR-006, CTR-010, CON-016, GRA-017,
  GRA-019 to GRA-021, GRA-023, MAN-003 to MAN-010, MAN-012, NAM-001, NAM-004, PKG-004 and
  PKG-020. The destination patterns carry exactly the closed list of INS-012.
- `spec/schema-profile.md` against `schema.v1` and SCH-001 to SCH-021: keywords, formats, the
  three forms of `$ref`, and where a `default` is accepted.
- The valid examples against the promises that govern them, read by hand: the project, the
  package and the first-start documents, with their tool names under IFC-012. The file name of
  each invalid example against the promise that rejects it.
- The definitions against the promises they cite, row by row. Apart from what the findings
  name, each row says what its promises say.
- The list of cases in which `isError` is true: the same five in `spec/definitions.md`,
  IFC-025, `spec/core/README.md` and the head of `spec/core/operations.yaml`, and OPS-009
  agrees for the two that are repeats.
- When a Conduit is closed: CON-008, ACC-011, ACC-039, LIF-016, LIF-024, LIF-028, the
  definition of close, and the `effect` of `retire.v1` and of `declare-lost.v1` agree.
- Shutdown: LIF-014, LIF-017, LIF-018, LIF-024 to LIF-026, LIF-030, PLC-011 and PLC-017 give
  one value for every sequence of retirement, declaring lost and reconciliation that I tried.
- Reconciliation: OPS-014, ACC-033, LIF-018, LIF-028, REV-010, REV-013 and the description of
  `reconcile.v1` name the same target and the same reasons.
- The retrieval tool: IFC-024, OPS-015 to OPS-017, REC-004 and the contract of
  `retrieve-result.v1`, apart from 3A-12.
- Later work: ACC-019, ACC-040, IMP-006, IMP-008, IMP-010 to IMP-012, TPL-014, IFC-016,
  IFC-035 and the contract of `start-later-work.v1` state the same conditions and the same
  reason.
- The bound and policies: ACC-029 to ACC-031, ACC-035, CON-028, `bound.v1` and the contract of
  `apply-policy.v1`.
- The order of reasons where a promise names it: OBJ-023, OBJ-025, TPL-013, POOL-017, CON-031,
  PKG-023 and LIF-022 each agree with OPS-023. The requests that name something that does not
  exist report invalid in OPS-029, ACC-031, ACC-033, OPS-014 and the contracts alike, and the
  two tools that need no Conduit report refused in OPS-015, ACC-040 and OPS-029 alike.
- Installing: NAM-012, PKG-006, PKG-023 to PKG-028, MAN-006, SUB-010, DL7 and the contract of
  `install.v1`.
- The implementation-defined list: no item gives an engine a choice that a promise takes
  away, apart from the doubt about DL1 in 3A-11.
- Promises that say "not dispatched": each names exactly one reason, in its `then` or in its
  observable, apart from 3A-7.
