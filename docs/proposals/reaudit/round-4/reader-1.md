# Round 4, reader 1: duplicates and contradictions

Cold read of `spec/` alone: all 134 files, among them the 437 promises of the 22 promise
files, `spec/definitions.md`, `spec/implementation-defined.yaml`, `spec/schema-profile.md`,
`spec/names.yaml`, the nine files of `spec/core/`, the ten of `spec/schemas/` and all 88
example files. A script checked that every promise ID and every DL item cited anywhere under
`spec/` exists, and that the counts in `spec/README.md` are the counts in the files. Twelve
findings: 1 `unsafe`, 1 `contradiction`, 3 `drift`, 1 `citation`, 6 `minor`. Most of the text
is consistent, and the last section lists what was checked without a finding. The four points
listed as already open are not reported.

## Findings

### 4A-1. A reconciled retirement confirms the shutdown of an instance that holds a runtime

Severity: `unsafe`. Promises: LIF-028, LIF-024, TPL-015, LIF-030, LIF-017, CON-008.

LIF-028, `then`:

> Recording that a retirement or a close occurred confirms the shutdown, unless the outcome of
> a start of one of the instance's runtimes is still unknown, under LIF-024.

LIF-024, `then`:

> Once such a start is reconciled as having occurred on a retired instance, the instance holds
> the runtime, and its shutdown is pending until a further invocation of `retire.v1` for it
> succeeds.

Situation. An Agent has a start of a runtime whose outcome is unknown. It is retired, and that
invocation of `retire.v1` ends unknown too; it stopped no runtime, because the instance held
none (TPL-009). An actor reconciles the start as having occurred, with the handle: the
instance now holds the runtime (TPL-015) and LIF-024 says pending "until a further invocation
of `retire.v1`" succeeds. The actor then reconciles the first retirement as having occurred.
No start is "still unknown", so LIF-028 confirms the shutdown, and LIF-024 says it is pending:
the reconciled invocation is not a further one. Under LIF-028 the instance can be unregistered
(LIF-017), its Conduits are closed (CON-008), and LIF-030 makes every later retirement a
request in which "no runtime is stopped", so the engine reports a confirmed shutdown for an
instance that holds a running runtime and has no way left to stop it. In the other order, with
the retirement reconciled first, the two promises agree.

The same two sentences of LIF-024 also collide between themselves in the middle of this
situation: after the start is reconciled and before the retirement is, the shutdown is "unknown
when the outcome of the latest invocation of `retire.v1` for it is unknown" and also "pending
until a further invocation". And the observable of LIF-024 says "it is the latest invocation
that counts, unless an earlier one succeeded", which the pending rule overrides where an
earlier retirement succeeded before the start was reconciled.

Least-change fix. In LIF-028 widen the exception: "unless the outcome of a start of one of the
instance's runtimes is still unknown, or the instance holds a runtime, under LIF-024". In
LIF-024 say that the rule for a held runtime takes precedence over the two rules that precede
it, and reword the observable's "unless an earlier one succeeded" to match.

### 4A-2. The start of a runtime of an unregistered instance needs a handle nothing can hold

Severity: `contradiction`. Promises: TPL-015, ACC-033, OPS-014, REV-010, OPS-031.

TPL-015, `then`:

> A reconciliation that records the start as having occurred states the handle, in the input
> `handle` of `reconcile.v1`, and the instance then holds the runtime with that handle. [...]
> One that records the start as having occurred and states no handle is not dispatched, and
> reports invalid.

ACC-033, `then`:

> Where the instance that an invocation was dispatched to, or that an invocation of a
> management verb named, no longer exists, reconciling the invocation is authorized against
> `management`: the reconciliation records the outcome and stores nothing.

Situation. LIF-024 lets an instance with a start of unknown outcome be unregistered once "its
location is declared lost". The start is then reconciled as having occurred. TPL-015 has no
exception: the request without `handle` reports invalid, and with it "the instance then holds
the runtime", which no instance can. ACC-033 and OPS-014 say nothing is stored. For a control
the same case was settled the other way: REV-010 has the reconciliation name "no revision"
and state "no settings", and OPS-031 makes stating them invalid. So one engine may demand a
handle and discard it, and another may report invalid for stating one, by analogy with
OPS-031.

Related gap, same promise: OPS-031 gives invalid for `handle` on "an invocation that is not
the start of a runtime", but for `revision` and `settings` only "for a control whose instance
no longer exists". What a reconciliation of a setup, or of any invocation that is not a
control, reports where it states `revision` or `settings` is not said, although the observable
says the two "belong to the reconciliation of a control of an instance that exists".

Least-change fix. Add to TPL-015 the sentence REV-010 has: where the instance no longer
exists, a reconciliation of the start states no handle, records the outcome and stores
nothing; and add that case to the `when` of OPS-031.

### 4A-3. `object-management.v1` is described as retiring a Resource of a retired Pool

Severity: `drift`. Files: `spec/core/capabilities.yaml`, ACC-033.

`spec/core/capabilities.yaml`, description of `object-management.v1`:

> inspecting, setting up and retiring what is not a Resource, or is a Resource whose Pool is
> retired;

ACC-033, `then` and observable:

> Where a Pool is retired, two of the requests that this promise authorizes against that Pool
> are authorized against `management` in its place: inspecting a Resource that the Pool owns,
> and reconciling an invocation. No other request is authorized against a retired Pool.

> [...] and one made through a Conduit to `management` reports refused too: setting up or
> retiring a Resource that the Pool owns [...]

Situation. A request for `retire.v1` that names a Resource of a retired Pool, made through a
Conduit to `management`: refused by ACC-033, offered by the capability's description. The
comment at the head of the same file and `spec/core/README.md` agree with ACC-033; only this
description differs. The same sentence also lists "reconciling an invocation that it
authorized, or an invocation of an operation of an Agent, an Interface or a Pool" and leaves
out the two cases ACC-033 adds (a Resource of a retired Pool, an unregistered instance).

Least-change fix. Write "inspecting what is not a Resource, or is a Resource whose Pool is
retired; setting up and retiring what is not a Resource;" and add the two reconciliations.

### 4A-4. IFC-018 and the definition of a restriction leave out what a start carries

Severity: `drift`. Promises: IFC-018, ACC-041, ACC-019; `spec/definitions.md` (restriction).

IFC-018, `then`:

> The list holds every restriction that applies to the request: the requirements of the
> request's own Conduit that narrow requests, every policy that applies, and the restrictions
> carried from the served invocation under ACC-017. For a step that has no `conduit`, it holds
> the restrictions of the invocation that the step's graph is serving, under ACC-018.

ACC-041, `then`:

> [...] the filters that the arranging invocation carried, under ACC-017 from an invocation it
> served or, where it was itself a start of later work, under this promise; and the policies
> that apply, when the start is decided, to a request of that caller to the target of each
> invocation in the chain.

Situation. A start of later work whose arranging request was an onward request through a
filtered Conduit. ACC-019 says a start "serves no invocation", so none of the classes that
IFC-018 enumerates holds the filters of the chain or the policies on the targets of the chain.
An engine that builds `io.github.shrav-k.conduit/restrictions` from IFC-018's list sends fewer
restrictions than ACC-041 requires, although ACC-041 says they are sent "in the form IFC-018
gives". ACC-015 was updated to name ACC-041; IFC-018 was not, and the definition of
"restriction" still reads "including one that an onward request carries from the invocation it
serves, and one that a step with no `conduit` carries".

Least-change fix. Add to IFC-018: "For a start of later work it holds the restrictions that
ACC-041 has the start carry", and add the same clause to the definition of restriction.

### 4A-5. IMP-004 and IMP-010 cover only a graph written in the template

Severity: `drift` (unsafe if IMP-004 is applied as written). Promises: IMP-004, IMP-010,
GRA-005, IMP-008, IMP-011, IMP-001; `spec/definitions.md` (later work).

IMP-004, `when` (IMP-010 has the same form for `arrangesLaterWork`):

> A template is registered in which a graph contains a step whose operation is marked as
> starting model work, and the operation the graph implements is not so marked.

GRA-005, as changed by the follow-up of round 3:

> [...] among the graphs selected for the template's instances, however each selection was
> made: in the template's `implementation` map, as a default of its type, or as the default of
> an operation's own contract.

> Every selection under TPL-003 and TPL-004 counts, whether it writes a graph in place or
> names an implementation item whose manifest gives a `graph`.

The definition of later work states the rule without the limit: "A graph that implements an
operation without the mark holds no step whose operation carries it".

Situation. A package's manifest lists an implementation whose `graph` implements an unmarked
operation and holds a step, with no `conduit`, for an operation marked `startsModelWork: true`
(or `arrangesLaterWork: true`). A template selects that implementation by name, or takes it as
a default. No graph is written "in" the template, so IMP-004 and IMP-010 as worded reject
nothing; a graph in a Type contract's `implementation` map is in the same position. At
dispatch nothing refuses the step either: IMP-008 and IMP-011 apply to a request that "names,
in `io.github.shrav-k.conduit/serving`, an invocation", and a step with no `conduit` names
none. Model work then starts, or later work is arranged, within an operation whose contract
carries no mark, which is what IMP-001 and IMP-003 promise cannot happen.

Least-change fix. Give IMP-004 and IMP-010 the wording of GRA-005: "a graph selected for the
template's instances, however the selection was made", with the same sentence on INS-022 for
a package that was not installed.

### 4A-6. REV-010 cites REV-002 and REV-011 for a reconciliation that changes nothing

Severity: `citation`. Promises: REV-010, REV-002, REV-011, OPS-031.

REV-010, `when` and observable:

> A control whose outcome is unknown is reconciled under OPS-014 as having taken effect.

> Every reconciliation of a control of an instance that exists names the instance's current
> revision, whichever way it records the effect: one that names a revision that is not current
> is not dispatched and reports conflict under REV-002, and one that names none reports invalid
> under REV-011.

REV-002 and REV-011 both begin "A request to change an object". A reconciliation that records
that the effect did not occur "stores nothing", by the same observable, so it is not a request
to change an object, and it is outside the `when` of REV-010 itself. The rule that such a
reconciliation must name the current revision, with its two reasons, therefore stands only in
an observable, and the two promises it cites do not state it. OPS-031 repeats the citation
("where it names no revision is REV-011"). It also matters for a retired instance: LIF-015
admits a reconciliation "where the reconciliation stores nothing in it", and whether that one
reports invalid without a revision rests on this sentence alone.

Least-change fix. Widen the `when` of REV-010 to "is reconciled under OPS-014" and move the
sentence into its `then`, or state it as its own promise beside REV-013.

### 4A-7. A policy or a carried filter makes every request to a core target unavailable

Severity: `minor` (a consequence to confirm, not a logical contradiction). Promises: ACC-015,
ACC-031, ACC-026, LIF-001, MAN-012; file `spec/core/conduit.yaml`.

ACC-031: "A policy that gives only `actors` applies to every request of which one of them is
the caller, whatever its target". ACC-015: where "the implementation that the request would be
dispatched to does not support the requirement of that policy", the request "is not
dispatched, and reports unavailable". `spec/core/conduit.yaml` gives `engine`, which
implements every core operation, one entry under `supports`: `bound.v1`.

Situation. A policy with `rate-limit.v1` and `actors: [operator]`. Every request of `operator`
to `management`, to `access`, to any Pool for an operation of `pool-management.v1`, and for
`set-default-location.v1` or `set-facts.v1`, reports unavailable, `remove-policy.v1` and
`close.v1` among them. The same holds for an onward request to one of those targets that
serves an invocation which came through any filtered Conduit (ACC-017, ACC-015). ACC-026 reads
as if a policy only narrowed: "the policy limits what the consumer can do through it", and
"Requests the policy forbids report refused". LIF-001 offers a policy as the way "to limit
access for a time without ending it". Neither says that for the core targets the effect is a
blanket unavailable.

Suggested fix. Decide it and say it: either one sentence in ACC-015 and ACC-031 that states
this consequence for a target whose implementation is `engine`, or a rule for which
requirements the engine applies to its own operations.

### 4A-8. A retrieval for an unregistered target: "can no longer be named" is no longer true

Severity: `minor`. Promises: OPS-015, OPS-016, IFC-024, LIF-031.

OPS-015, `when` and observable:

> [...] by an operation identity and a target under which it has no invocation [...]

> [...] and for a target that has been unregistered and so can no longer be named.

IFC-024, observable, as changed by the follow-up of round 3:

> A call that names a served invocation may give `target` in the resolved form of a reference,
> under LIF-031.

> Once the target is unregistered it can no longer be named, and the call reports refused in
> the same way.

Situation. A Resource retrieves the result of an onward request after its target was
unregistered, and gives `target` as `{kind, id}`, the only form it holds. The target can be
named, by its ID, and LIF-031 says "the engine takes the object by its ID". The Resource does
have an invocation under that identity and that target, so the `when` of OPS-015 does not
cover the case, and the refusal rests on an observable whose stated cause is no longer true.
The outcome is still stated (refused); its ground is not.

Least-change fix. Add "or whose target no longer exists" to the `when` of OPS-015, and drop
"and so can no longer be named" from the three observables.

### 4A-9. IFC-026 has the level `may` for what IFC-019 makes a `must`

Severity: `minor` (level). Promises: IFC-026, IFC-019, ACC-008, ACC-014, OPS-006.

IFC-019 (`must`): "If one of them forbids the request, or it cannot apply one of them, it
performs nothing and answers with a reason in place of an outcome, as IFC-026 states."
IFC-026 (`may`) has the same situation as its `when`, and its `then` also fixes content that
is not optional ("The result's `_meta` also holds `io.github.shrav-k.conduit/restriction`
[...] and the result's `isError` is true"). Its observable says "An implementation gives a
reason only in those two cases", which are exactly the cases in which IFC-019 requires one.

Situation. A restriction forbids a request, the implementation performs nothing and answers
`failed`. It has kept IFC-026 and broken IFC-019 and ACC-008. Nothing is left for `may` to
permit.

Least-change fix. Make IFC-026 a `must` that states the form of the answer, or keep in it only
the form and leave the obligation to IFC-019.

### 4A-10. Reasons that stand only in an observable

Severity: `minor`. Promises: LIF-009, LIF-017, TPL-011, NAM-023, IFC-029.

Round 3 moved several such rules into a `then` (OBJ-027, OPS-030, PLC-022 and others). These
remain, each with a reason that is not the definitional "refused" of a missing Conduit:

- LIF-009. The `then` gives invalid where "a rule on instantiation does not make the new
  instance". The observable adds another cause: a component Conduit that "belongs to no
  instance [...] a request that names it is not dispatched and reports invalid". No `then`
  says what `register-replacement.v1` reports where it names any instance that no nested
  instantiation made, such as an ordinary retired Agent.
- LIF-017. The `then` is "dispatched only if"; conflict is in the observable alone.
- TPL-011 and NAM-023. Invalid is in the observable alone.
- IFC-029. "it is not dispatched and reports invalid" for keys under two prefixes is in the
  observable alone; the `then` only says that a request "carries the keys of one version only".

Least-change fix. Add the reason to each `then`; for LIF-009 add the case to the `when`, or
give it a promise as LIF-032 was given.

### 4A-11. The digest of an unpublished package: the manifest as written or as resolved

Severity: `minor`. Promises: SUB-008, SUB-010, MAN-006, MAN-010, IFC-006, ACC-038.

SUB-008 hashes "`manifest`, the project's manifest", and says "Two submissions whose contract
documents and manifest are equal under SCH-007 give the same digest, however their files are
written". SUB-010 says a directory is registered "with the same result as a request for
`register.v1` that is given the values read, once each relative `fetch` and each program given
as a relative path is resolved against the directory".

Situation. One project directory, with `command: [bin/server]` in its manifest, submitted from
two places. Hashed as written, the two digests are equal. Hashed as SUB-010's equivalent
`register.v1` request would hash it, the manifests hold two absolute paths and the digests
differ. The digest is observable: `inspect.v1` lists it under `packages` (NAM-009), and it is
the wire name `<digest>#<name>` (IFC-006, ACC-038). A conformance vector "made of files"
cannot state an expected digest until this is fixed.

Least-change fix. Say in SUB-008 that the manifest is hashed as it was written, before any
relative reference is resolved, and except the digest from "the same result" in SUB-010.

### 4A-12. Installing what is already installed, with a relative `fetch`

Severity: `minor`. Promises: PKG-027, PKG-023, MAN-006.

PKG-027: "The request changes nothing, and its outcome is succeeded. It is decided from what
is installed". PKG-023: "A request whose `fetch` is a relative reference reports invalid too,
under MAN-006". For a request that names an installed address, version and digest and a
relative `fetch`, one promise gives an outcome and the other a reason. OPS-023 orders reasons
only. ACC-006, ACC-012, LIF-030 and OBJ-026 avoid this with "is decided like any other";
PKG-027 does not say it.

Least-change fix. Add "is decided like any other" to PKG-027, as LIF-030 has it.

## What I checked and found consistent

- Citations. Every promise ID cited under `spec/` exists; the 25 IDs that do not exist are
  removed promises and are named only in the comments at the heads of files. Every DL item
  cited exists, and each says what the citing text claims. Export names, template names and
  the three assigned names agree with `spec/core/conduit.yaml` and `spec/names.yaml`.
- Counts. The per-file promise counts, the total of 437, the 15 DL items, the 99 exports, the
  45 and 58 Schema contracts and the 61, 20 and 7 example files are as `spec/README.md`,
  `spec/core/README.md` and `spec/schemas/README.md` state them.
- Layers. The `layer` of each promise fits its file comment and the note in `spec/README.md`
  (one `interface` promise in a topic file, PKG-016; `language` rules in the other two
  directories). No finding beyond the level of IFC-026.
- Later work. ACC-019, ACC-040, ACC-041, IMP-006, IMP-008, IMP-011, IFC-035, IFC-006, IFC-016,
  TPL-014, the definitions and the contract `start-later-work.v1` state the same conditions,
  the same Conduit, the same caller and the same single reason (refused); what a start
  carries agrees between ACC-017 and ACC-041. Only 4A-4 and 4A-5.
- Reconciliation. Which target authorizes it (ACC-033, OPS-014, LIF-018, LIF-028, REV-010,
  TPL-015, REC-004, the three capabilities, `reconcile.v1`); OPS-030 against OPS-029; a
  retired Pool; a retired instance under LIF-015. Only 4A-1, 4A-2 and 4A-6.
- What closes a Conduit, and a retired consumer's Conduits. CON-008, CON-031, CON-023,
  ACC-003, ACC-006, ACC-011, ACC-012, ACC-024, ACC-039, LIF-015, LIF-016, LIF-023, LIF-024,
  LIF-026, PLC-017 and the definitions agree.
- A retired Pool and its Resources. ACC-033, POOL-004, POOL-017, LIF-009, LIF-020, LIF-021,
  LIF-026 and the comments of `spec/core/` agree. Only 4A-3.
- Policies. ACC-031, ACC-035, ACC-042, ACC-043, ACC-009, ACC-017, ACC-029, `apply-policy.v1`
  and the definition agree on `actors` and `targets`, on `version`, on what an onward request
  carries and on each reason. Only the consequence in 4A-7.
- `supports` and graphs. CON-022, PKG-019, PKG-020, MAN-009, ACC-015, ACC-018, ACC-038,
  `implementation-item.v1`, the two examples and the definition agree.
- `replace.v1` with a manifest. SUB-009, NAM-007, NAM-009, NAM-022, OPS-029, the contract and
  the definition agree.
- `isError`. IFC-009, IFC-025, IFC-026, IFC-035, OPS-007, OPS-009, ACC-034, LIF-021 and the
  definitions of result and withheld agree, an operation with no outputs included.
- The order of reasons. OPS-023 and ACC-037 against every promise that names two reasons
  (LIF-022, LIF-030, OBJ-023, OBJ-025, PKG-023 to PKG-025, ACC-042, ACC-043, PLC-019,
  CON-024, TPL-013, FST-006): no request with two reasons outside that order.
- Installing. NAM-012, NAM-013, PKG-006, PKG-017, PKG-021 to PKG-028, MAN-006, SUB-010, DL7
  and `install.v1` agree. Only 4A-12.
- Schemas against promises. Each schema under `spec/schemas/` and each reference schema
  against the promise it names: the closed lists of destinations (INS-012), the forms of
  `from` (INS-025), a step and a value reference (GRA-017, GRA-020), the manifest (MAN-003 to
  MAN-012), names, addresses and versions. No difference found. The valid examples keep the
  promises they cite, and each invalid example breaks the promise its file is named for.
- The core package. Contracts against capabilities, types, templates and the manifest; the
  tables of `spec/core/README.md` against the contracts. Only 4A-3.
- The implementation-defined list against the promises that cite it: no item is also fixed
  by a promise.
