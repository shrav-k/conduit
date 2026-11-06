# Round 3, reader 3: undetermined behavior

I read all 129 files under `spec/` and nothing else: `README.md`, `definitions.md`, the 424
promises of `promises/`, `conventions/` and `interface/`, `implementation-defined.yaml`,
`schema-profile.md`, `names.yaml`, every file of `core/` and `schemas/`, and all 83 example
files. I walked the twelve scenarios I was given and eleven of my own. I report 15 findings:
2 `unsafe`, 9 `undetermined` and 4 `minor`. Most of what I walked came out determined, and the
last section lists it. The open points I was told of (secret values, the shape of a diagnostic
and of an inspection result, the absence of vectors, the check against the Model Context
Protocol) are not repeated here.

Two notes on method. The harness put the text of the repository's `AGENTS.md` in front of me
unasked; I used nothing from it. To place this file I listed the names in
`docs/proposals/reaudit/` and opened none of them.

## Scenarios walked

Given:

1. First start, then a project directory with a manifest, contracts, templates and instances,
   one package not installed; then `install.v1`, then registration again.
2. An Agent's setup with a failed optional step and a declared Conduit that cannot open; a
   later `open.v1`.
3. A request through a Conduit with a filter and a policy, an onward request to a third
   instance, and the first Conduit closed meanwhile.
4. An invocation that ends unknown, the lost result, retrieval, reconciliation, and a repeat
   before and after each.
5. Later work: arranged directly and by an onward request; a start after a close, after a
   policy, after the target or its template was replaced, after reconciliation; a repeat.
6. Retirement: a retire graph that fails halfway, `retire-with-resources.v1` with one unknown
   shutdown, a location declared lost, unregistering everything, a replacement for a nested
   instance.
7. Replacing a template with instances and during a setup, renaming, a stale `revision`,
   reconciling a control.
8. Delegation with a bound, a policy, the loss of the delegating right; the same by `root`.
9. Two versions of a package, a re-export, a digest mismatch, `#name` in an unpublished
   project, replacing a template of one.
10. A plain implementation as target and as consumer, a malformed or self-contradictory
    result, a client that is not an instance.
11. A step that uses the output of a failed optional step, an output binding that selects
    nothing, defaults, cycles.
12. Records, a purge, and inspection afterwards.

My own:

13. Later work arranged by an invocation that is itself a start of later work.
14. A policy applied or removed while an invocation it would cover is in progress.
15. A Resource that retrieves the result of an onward request after the invocation it served
    has ended.
16. `install.v1` for a package that is already installed, with the archive unreachable or
    changed.
17. A runtime's start that ends unknown and is reconciled as having occurred.
18. A second setup of an instance that holds a runtime, with a step that reads the start
    step's output.
19. A policy applied while two versions of the requirement's package are installed.
20. `declare-lost.v1` for a location that is already retired.
21. Reconciling an invocation whose instance has since been unregistered.
22. A graph that implements an operation and invokes that operation on its own instance.
23. A composed requirement whose optional input is not supplied.

## Findings

### 3C-1. Work arranged by a start of later work is not tied to the chain

- Severity: `unsafe`
- Scenario: 13 (and 5).
- Promises: ACC-019, ACC-040, IMP-011, IFC-021.

Caller X calls instance A through Conduit C1. A makes an onward request to B through C2, of
an operation marked `arrangesLaterWork`. The work W1 is tied to C2 and C1. B asks for a start
S1 with `start-later-work.v1`; S1 is dispatched through C2. IMP-011 lets S1 itself be of a
marked operation ("or when it is a start of later work under ACC-040, whose arranging
operation carries the mark"), so S1 arranges further work W2. C1 is then closed.

ACC-019 defines the chain only through onward requests:

> where the arranging request was an onward request, each Conduit that the invocations it
> served, directly or through others, came through

The request that began S1 is the call to `start-later-work.v1`, which under IFC-021 "names no
invocation in that key". It is not an onward request and served nothing. So the chain of W2 is
C2 alone. ACC-040 then tests, for a start of W2, "that Conduit and every other Conduit of its
chain are open", which holds. The same sentence gives the restrictions of that start as "the
restrictions that the arranging request carried under ACC-017 from the invocations it served",
which for S1 is none, so the filters of C1 are dropped too.

- Engine 1 follows the text: starts of W2 are dispatched for X after C1 is closed, without the
  filters of C1. Closing C1 ended W1 and not W2.
- Engine 2 reads "a start never carries less than the request that arranged the work" as
  covering the chain, and refuses.

Least-change fix: add to ACC-019 that work arranged by a start of later work, or by a request
that serves one, is tied to every Conduit to which the work of that start is tied; and to
ACC-040 that such a start carries what its arranging start was sent.

### 3C-2. A runtime whose start is reconciled as having occurred is never held

- Severity: `unsafe`
- Scenario: 17 (and 2, 4, 6).
- Promises: TPL-007, OPS-014, LIF-028, OPS-021, OPS-028, OPS-009, TPL-009, LIF-024.

A setup step names a runtime. Its start operation ends unknown: the process may be running and
no handle came back. An actor checks, and reconciles the step's invocation as having occurred.
`reconcile.v1` takes `invocation`, `occurred`, `checked`, `revision` and `settings`. It has no
input for an output or a handle, and OPS-014 says of anything beyond the outcome only this:

> What the reconciliation of a setup, an open, a retirement or a close changes beside the
> outcome is LIF-028, and what that of a control changes is REV-010.

TPL-007 makes the instance hold a runtime only "once that succeeds", with "the handle it
returned". Nothing was returned. On the next setup two promises pull apart. TPL-007: "If the
instance does not hold that runtime, the step invokes its start operation". OPS-028 gives the
step the identity it had when first dispatched, and under OPS-009 a request that matches an
invocation that succeeded has "Nothing ... dispatched".

- Engine 1 starts the runtime again. Two processes run, the record says one start occurred,
  and the first is never stopped, because retirement stops "each runtime the instance still
  holds" (TPL-009).
- Engine 2 dispatches nothing. The instance holds no runtime, a value reference with the
  source `runtime` yields no value (GRA-026), and a later `retire.v1` "succeeds at once", so
  the status reports the shutdown confirmed (LIF-024) while the process runs.

Either way an effect is duplicated or a shutdown is reported wrongly. Least-change fix: say in
TPL-007 or LIF-028 what the reconciliation of a runtime's start does. The smallest rule is
that it is reconciled as having occurred only with the handle, as a new optional input of
`reconcile.v1` in the way `settings` serves a control, and that without one the request
reports invalid. Say also that a stop reconciled as having occurred ends the holding.

### 3C-3. Which policies an onward request, or a start, carries after the policies changed

- Severity: `undetermined`
- Scenario: 14 (and 3, 5).
- Promises: ACC-017, ACC-031, ACC-036, ACC-040, ACC-026, IFC-020.

X calls A, and the invocation runs for a long time. While it runs, an actor applies a policy
P with `actors: [X]` and `targets: [A]`, or removes one that applied when it was dispatched.
A then makes an onward request to B. P does not match that request directly, because its
target is B. Whether it is carried is the question. ACC-017 speaks of the past:

> every restriction that applied to that invocation

ACC-036 speaks of the present: the decision is made "from the Conduits and the policies as
they stand when it is made". ACC-031 says "an onward request that serves such a request
carries the policy under ACC-017", which does not say whether "such a request" is judged when
it was dispatched or now.

- Engine 1 fixes the list when the served invocation is dispatched: a policy applied later is
  not carried, and a policy removed later still is.
- Engine 2 judges, at each onward request, which policies now apply to the served request: the
  new policy limits the onward request, and the removed one does not.

For a start of later work the text has both in one place. ACC-040 lists "the restrictions
that the arranging request carried under ACC-017 from the invocations it served, each as it
stands when the start is dispatched", and then: "A start never carries less than the request
that arranged the work." Where a carried policy was removed in the meantime, the first
sentence drops it and the second keeps it.

Least-change fix: one sentence in ACC-017, cited by ACC-040: the policies carried are those
that apply, when the onward request or the start is decided, to a request of that caller to
the target of each invocation in the chain. Then delete "A start never carries less ...", or
restrict it to filters.

### 3C-4. For which caller a retrieval is made, and whether a Resource can retrieve at all

- Severity: `undetermined`
- Scenario: 15 (and 4).
- Promises: OPS-015, OPS-016, IFC-021, IFC-024, IFC-028, OPS-008.

OPS-016 serves "an invocation that began with a request it sent itself, for the same caller".
`retrieve-result.v1` takes `operationIdentity` and `target` and nothing that names a caller.
For an actor's own request the caller is the actor. For an onward request it is the caller of
the served invocation, which the engine learns only from
`io.github.shrav-k.conduit/serving`. No promise says that a retrieval carries that key.
IFC-021 refuses a request from an instance that is not an actor and "names no invocation" in
it, and excepts one tool only: "other than one for `start-later-work.v1`". IFC-028 refuses a
`serving` that names an invocation "that is no longer in progress". IFC-024 says a retrieval
"is decided under OPS-015 to OPS-017 and not under ACC-001", and is silent on IFC-021.

Resource R served invocation I for caller X, sent an onward request under identity `k` to T,
and lost the result. I has ended: for example it was in progress across a restart, so its
outcome is unknown under IMP-007. R calls `retrieve-result.v1` with `k` and T.

- Engine 1 applies IFC-021: R is not an actor and names no served invocation, so the call
  reports refused. R can never learn the state or the ID that OPS-016 says it "can still"
  reconcile with. Naming I does not help, under IFC-028.
- Engine 2 treats the retrieval as outside IFC-021, as it is outside ACC-001, and answers by
  sender, identity and target. Where R used `k` with T for two callers, OPS-008 has two
  invocations, and nothing says which is returned.

An Agent that sent the onward request has the same gap from the other side: without `serving`
it is its own caller, and has no invocation under OPS-015.

Least-change fix: add to IFC-024 that a retrieval names in `serving` the invocation that the
original request served, that for this tool the invocation need not be in progress, and that
the caller is that invocation's; a retrieval that names none is made for the instance as its
own caller.

### 3C-5. Installing a package that is already installed, when the archive cannot be fetched

- Severity: `undetermined`
- Scenario: 16 (and 1, 9).
- Promises: PKG-024, PKG-025, PKG-026, PKG-027, OPS-023.

Four promises have conditions that can hold together, and none says whether an engine
fetches an archive for an address and version that is already installed.

Case A: the package is installed with the digest asked for, and its archive cannot be fetched
now. PKG-027: "The request changes nothing, and its outcome is succeeded." PKG-024: "The
request is not dispatched, and reports unavailable." OPS-023 orders reasons against each other
and says nothing of a reason against an outcome.

Case B: the package is installed with digest D1, the request names D2, and the archive at the
place of `fetch` hashes to D1. PKG-025 gives conflict. PKG-026 says the package "is rejected"
and "reports invalid under PKG-023, whether or not another version of the package is
installed". If both apply, OPS-023 puts invalid first.

- Engine 1 looks at what is installed before it fetches: succeeded in A, conflict in B.
- Engine 2 fetches first: unavailable in A, invalid in B.

Least-change fix: add to PKG-025 and PKG-027 that they are decided from what is installed,
and that nothing is fetched for an address and version that is already installed.

### 3C-6. The order in which `retire-with-resources.v1` retires the Resources

- Severity: `undetermined`
- Scenario: 6.
- Promises: LIF-021, CON-008, ACC-039, POOL-013, LIF-024, DL1.

LIF-021 says only that "A request to retire each Resource is dispatched". POOL-013 orders a
Pool's setup by the references among entries; nothing orders the retirements. The order is
observable. Entry `b` refers to entry `a`, its Resource declares a Conduit to the Resource of
`a`, and a required step of its `retire` graph names that Conduit.

- Engine 1 retires `a` first. CON-008 closes the Conduit "at once", the step of `b` reports
  refused, and the retirement of `b` fails where no other step had an effect. Its shutdown is
  pending, and the outcome of the request is failed. Each further retirement of `b` meets the
  same closed Conduit. A failed outcome cannot be reconciled, so the Pool is never retired.
- Engine 2 retires `b` first, and everything succeeds.

DL1 is the nearest item of the implementation-defined list. It is about "Graph execution" and
"The order among steps that do not depend on each other", and these retirements are neither
steps of a graph nor independent.

Least-change fix: add to LIF-021 that a Resource is retired before any Resource that it
refers to, or of which it is the consumer of a Conduit, among those the Pool owns; or name the
order in DL1.

### 3C-7. Which version of a requirement's contract a policy names

- Severity: `undetermined`
- Scenario: 19 (and 8, 9).
- Promises: ACC-031, NAM-005, NAM-006, NAM-011, NAM-019, NAM-024, ACC-015.

Everywhere else a qualified reference gets its version from a manifest: "the address names the
package, and the manifest names its version" (NAM-005). A policy is the one contract named by
a request. `apply-policy.v1` takes `requirement`, `arguments`, `actors` and `targets`, and no
manifest. NAM-011 lets two versions of a package be installed side by side, and NAM-024 lets
the later one add an optional input to a requirement of the same name. ACC-031 checks the
arguments "as NAM-019 checks those of an applied requirement", against the inputs of some
version.

With `example.org/files` installed at 0.3.0 and at 0.4.0, where 0.4.0 adds an optional input
`burst` to `rate-limit.v1`, a request to apply `example.org/files#rate-limit.v1` with
`burst: 5`:

- Engine 1 resolves against the newest installed version: dispatched.
- Engine 2 resolves against the first installed, or requires one version only: invalid.

The same choice decides which contract the arguments are sent under to an implementation that
holds the other version. Least-change fix: say in ACC-031 which version governs. The smallest
change in keeping with NAM-005 is an optional input `manifest` on `apply-policy.v1`, as
`replace.v1` has. Failing that, state that the request is invalid unless exactly one installed
version exports the requirement.

### 3C-8. Reconciling an invocation whose instance has been unregistered

- Severity: `undetermined`
- Scenario: 21 (and 4, 6, 7).
- Promises: ACC-033, OPS-013, OPS-014, OPS-029, REV-010, LIF-015, LIF-019.

An operation of Resource R ends unknown. R is later retired, its shutdown is confirmed, and it
is unregistered. The invocation "stays unknown until it is reconciled" (OPS-013), and its
records remain (LIF-019). ACC-033 authorizes the reconciliation against "the Pool that owns
that instance where it is a Resource", and closes with:

> Which target that is can be told only for an object that exists, and a request that names
> one that does not exist reports invalid under OPS-029.

The request names an invocation, which exists. The instance does not. LIF-015 says which
reconciliations a retired instance takes, and no promise speaks of an unregistered one.

- Engine 1 remembers the owner and authorizes against the Pool.
- Engine 2 authorizes against `management`, as for a retired Pool.
- Engine 3 cannot tell the target and reports refused, or invalid.

For a control, REV-010 adds that the request "names the instance's current revision", which no
longer exists. Least-change fix: add to ACC-033 that where the instance no longer exists the
reconciliation is authorized against `management`, records the outcome and stores nothing, and
that a control is then reconciled only as not having occurred.

### 3C-9. What a value reference yields from a step that was left as succeeded

- Severity: `undetermined`
- Scenario: 18 (and 2, 11).
- Promises: TPL-007, GRA-003, GRA-002, GRA-026, OPS-014, DL1.

A setup graph has a step `watch` that names a runtime, and a step `announce` whose argument is
`{source: step, step: watch, output: watcher}`, or another output of the start operation,
which TPL-006 lets it declare beside the handle. The first setup starts the runtime. On the
second, TPL-007 has the start "not invoked again and the step is left as succeeded". GRA-002
lets `announce` run. GRA-026 lists what makes a source absent:

> an optional input that the request left out and that states no default, an optional output
> that a step did not return, or a runtime that the instance does not hold

A required output of a step that returned nothing in this run is not on the list.

- Engine 1 yields the handle it holds, and for other outputs the values of the earlier run.
- Engine 2 yields no value: `announce` reports invalid, and every setup after the first fails
  where `announce` is required.

The same question arises for a step that was reconciled as having occurred, which has no
outputs at all. DL1 leaves "what a re-run of setup or retire does with steps that already
ran" to the engine, but TPL-007 takes this step out of that choice. Least-change fix: add to
GRA-026 that a step left as succeeded, or reconciled, has returned no outputs in that run, so
a value reference to one of them yields no value, and that the handle of a runtime is read
with the source `runtime`. Or state the opposite. Either is one sentence.

### 3C-10. A constituent's binding that yields no value

- Severity: `undetermined`
- Scenario: 23.
- Promises: CTR-010, NAM-019, CTR-005, GRA-026.

A composition declares an input `per-minute` marked `optional: true` with no `default`. Its
constituent binds `limit: {source: input, name: per-minute}`, and the constituent's own input
`limit` states `default: 20`. A Conduit template applies the composition with no arguments.
CTR-010 says only:

> each binding takes its value from the arguments given to the composition, and the values are
> checked against the constituent's inputs as NAM-019 checks the arguments of an applied
> requirement

GRA-026, which rejects a missing value "even where the input is optional or states a default",
is written for a step that "is about to be dispatched", and a constituent is not a step.

- Engine 1 treats the constituent's input as not supplied: `limit` takes 20, and the template
  is accepted.
- Engine 2 follows GRA-026 by analogy: the template is rejected.

The same holds for a `pointer` that selects nothing. It is a difference at the Language level.
Least-change fix: one sentence in CTR-010 choosing one of the two.

### 3C-11. The manifest handed over with `replace.v1`

- Severity: `undetermined`
- Scenario: 9 (and 7).
- Promises: SUB-009, NAM-022, NAM-007, NAM-009, SUB-008, MAN-010.

Two points, both when a template of an unpublished project is replaced.

First, `#name` for an implementation. NAM-022 says of the replacing document:

> it reaches a contract of the unpublished package that the template being replaced holds
> under NAM-009

It names a contract, not an implementation. The request "may hand over a manifest", and that
manifest may list `implementations`. SUB-008 makes an unpublished package from "a submission",
and a replacement is not one. Where the replacing document selects `#worker`, and the manifest
handed over lists `worker` with another `command` than the original project did:

- Engine 1 resolves `#worker` in the package the template holds: instances made afterwards run
  the old program, and the new item is ignored.
- Engine 2 takes the manifest handed over as the document's project: they run the new one, and
  the template holds another digest.

Second, no manifest. SUB-009 says the references to packages "resolve through it", where one is
handed over. Where none is, NAM-007 rejects a document that "is submitted with no manifest",
though a replacing document is not submitted, and NAM-022 shows the other pattern, in which a
replacement falls back on what the template holds.

- Engine 1 reports invalid for any reference to a package.
- Engine 2 resolves through the packages the template being replaced holds.

Least-change fix: add to SUB-009 that `#name` in the replacing document reaches the contracts
and the implementations of the unpublished package the template holds, that the
`implementations` and `exports` of a manifest handed to `replace.v1` are rejected, and that a
replacing document that refers to a package with no manifest is rejected under NAM-007.

### 3C-12. `declare-lost.v1` for a location that is already retired

- Severity: `minor`
- Scenario: 20 (and 6).
- Promises: LIF-015, PLC-017, PLC-011, LIF-030, ACC-012.

A second request to declare the same location lost, or one for a location that was retired by
`retire.v1`, has no promise of its own. The condition of LIF-015 covers "a request to set it up
or to change it". Its observable then gives a closed list: "The only requests that address a
retired instance and can be dispatched", and `declare-lost.v1` is not on it. So the request is
not dispatched, and a reader has to supply the reason, conflict. The other verbs that find
their work done succeed (ACC-006, ACC-012, LIF-030, PKG-027), and an engine could follow them.

PLC-011 needs the same care. It forbids retiring a location "that has other instances placed on
it". A retired instance that still exists keeps its placement, so read to the letter it counts.
An engine that counts only instances that are not retired lets the location be retired by
request while instances with an unknown shutdown remain on it. Those can then be freed only by
declaring the location lost, which is the request above.

Least-change fix: add to LIF-015 that a request to declare a retired location lost reports
conflict, or to PLC-017 that it changes nothing and succeeds; and add "retired or not" to
PLC-011.

### 3C-13. A project's own contract that needs a package that is not installed

- Severity: `minor`
- Scenario: 1.
- Promises: PKG-006, PKG-028, SUB-008, CTR-004, CTR-006, SCH-015, INS-022.

PKG-006 says what holds for a template that "refers to a package its manifest lists but that
is not installed", and PKG-028 for a package whose manifest lists one. For a contract of a
submitted project there is neither. A Type contract of the project that offers
`example.org/files#directory-access.v1`, or whose schema has a `$ref` into that package,
cannot be checked under CTR-006 or SCH-015 while the package is missing. Each of those says
the contract is rejected "when ... its project is submitted", if it breaks the rule.

By analogy with PKG-028 the submission is accepted with a diagnostic, and the check is made
when a template that uses the contract is instantiated. An engine that reads an unresolved
selection in a contract as a broken rule rejects the whole submission under SUB-003. One
sentence in SUB-008, in the words of PKG-028, settles it.

### 3C-14. What a purge keeps is less than a reconciliation needs

- Severity: `minor`
- Scenario: 12 (and 4).
- Promises: REC-004, ACC-033, OPS-014, LIF-028, REV-010, LIF-013.

REC-004 keeps, for every invocation, "its operation identity, as OPS-008 scopes it, the
engine's ID for it, its outcome, and a digest of its contents", and promises that "an
invocation whose outcome is unknown can still be reconciled". To reconcile it the engine must
know more. ACC-033 needs to know whether it was of a management verb, which target authorized
it and which instance it was dispatched to. LIF-028 needs the instance whose shutdown it
confirms, and REV-010 the control and its `changes`. An invocation that carried no operation
identity keeps no scope at all. An engine must therefore keep more than the list, and the
reader has to infer that.

It also leaves open whether `inspect.v1` still lists what was kept among "the records ... that
name the instance" (LIF-013): the kept items name no instance. Least-change fix: add to the
list of REC-004 the operation, the instance the invocation was dispatched to and the target
that authorized it, and say that they are read as records of that instance.

### 3C-15. A graph that invokes the operation it implements

- Severity: `minor`
- Scenario: 22 (and 11).
- Promises: GRA-005, GRA-024, GRA-014, TPL-004, DL10.

The specification rejects every other static cycle: steps (GRA-005), entries (POOL-011),
components (CON-019), compositions (CTR-005) and re-exports (PKG-014). GRA-005 counts "stated
dependencies and uses of outputs" within one graph. A graph that implements `directory.list.v1`
and holds a step with that `operation` and no `conduit` "invokes the implementation selected
for its own instance" (GRA-024), which is the graph. Two graphs of one template that name each
other's operations do the same. No promise rejects the template, so to the letter it is
registered and each request recurses until a limit of DL10 ends it.

A reader who expects the pattern of the other five may reject it at registration, and that
changes what `register.v1` reports. Least-change fix: one sentence in GRA-005 extending the
cycle to the graphs selected in one template's `implementation` map and in its type's, or one
stating that such a template is accepted.

## Walked and found determined

Each line is a step where the text gave one result, with the promise that gave it.

Scenario 1:

- After first start exactly five objects and the two Conduits of `root` exist, all ready:
  FST-001 to FST-003.
- A directory is read as its `.yaml` and `.json` files, the manifest apart, and registered as
  `register.v1` would: MAN-011, SUB-010. The actor is DL7.
- A template that refers to a listed package that is not installed is registered, with a
  diagnostic: PKG-006. An instance document in the same submission whose template needs that
  package is rejected, so nothing in the submission is registered and the request reports
  invalid: PKG-007, SUB-003, OBJ-023.
- A registration installs nothing: NAM-012, PKG-006. After `install.v1` the same submission is
  decided afresh, and the template's checks are made at instantiation: OPS-025, INS-022.
- Registering a Resource into a Pool of the same submission needs only the Conduit to
  `management`: ACC-033. Nothing is set up or opened: INS-016.
- `root` has no Conduit to the new Pool, so the entry steps of the Pool's setup report refused
  until one is registered and opened: POOL-015.

Scenario 2:

- A step that depends on a failed optional step does not run, and if it is required the setup
  does not succeed: GRA-001, GRA-002, GRA-003.
- A required declared Conduit that cannot open, for example under PLC-013, leaves the instance
  registered; what the setup opened stays open: LIF-004, LIF-027.
- The outcome of `set-up.v1` is unknown where another step succeeded with an effect, and failed
  otherwise: OPS-022. A repeat under the same identity is told unknown, because `set-up.v1`
  does not deduplicate: OPS-012.
- An optional Conduit that did not open is opened later by `open.v1`, or by the next setup:
  ACC-006, CON-004.

Scenario 3:

- The engine decides before any call; the filter and the policy go to the implementation, and
  a policy that the implementation does not state reports unavailable: ACC-002, IFC-018,
  ACC-015.
- The onward request is permitted only by the sender's own Conduit, and carries the caller and
  the restrictions: ACC-016, ACC-017.
- When the first Conduit is closed, the invocation and the onward invocation in progress run to
  their ends, the caller's content is withheld, and every later onward request of the chain
  reports refused: ACC-034, OPS-017.

Scenario 4:

- A repeat of an unknown invocation is a new attempt only where the contract deduplicates and
  the implementation is not plain: OPS-012.
- An actor retrieves the state and the engine's ID with no Conduit: OPS-016, IFC-024.
- Reconciliation is through the Pool that owns a Resource, or `management`: ACC-033, OPS-014.
  A second reconciliation reports conflict: OPS-014.
- After a reconciliation as failed a repeat is a new attempt; after one as succeeded it
  returns succeeded with no outputs and `isError` true: OPS-024, OPS-009, IFC-025. The last
  rests on reading "no longer keeps" to cover outputs that never existed.

Scenario 5:

- A start is refused unless the arranging invocation went to the asking instance, is marked,
  has succeeded and every Conduit of the chain is open: ACC-040.
- A start after a close of any Conduit of the chain, or once the target is retired, reports
  refused: ACC-019, CON-008.
- A start after a policy was applied to the caller and the instance carries it: ACC-040.
- Replacing the target's template changes nothing for the instance: INS-020. A replacement
  instance cannot name the old arranging invocation: ACC-040.
- An arranging invocation that is unknown gives refused until it is reconciled as succeeded:
  ACC-040.
- A repeat of a start under one identity is the same start: ACC-040, OPS-009.
- A plain implementation cannot be selected for a marked operation: TPL-014.

Scenario 6:

- A retirement that fails leaves the state retired and the shutdown pending, and the instance
  may be retired again; one that is confirmed is retired again with no effect: LIF-005,
  LIF-024, LIF-026, LIF-030.
- That a runtime stays held after a stop that failed or is unknown follows from "until the
  runtime is stopped": TPL-007. I read "stopped" as "a stop succeeded".
- The consumer's Conduits stay open for the retirement only, and close when the shutdown is
  confirmed: CON-008, ACC-039.
- With one Resource's shutdown unknown, `retire-with-resources.v1` ends failed, the Pool is
  not retired, and the Resource's `retire.v1` is reconciled on its own: LIF-021.
- Declaring a location lost retires everything on it, closes their Conduits at once and lets
  each be unregistered: PLC-017, CON-008, LIF-017.
- Unregistering needs a confirmed shutdown or a lost location, a Pool with no Resource left,
  and a location with no instance left: LIF-017, LIF-020, PLC-011.
- A replacement for a retired nested instance is made only by `register-replacement.v1`, from
  the nested instantiation as the instance holds it: LIF-008, LIF-009. A request while the
  instance is not retired reports conflict: LIF-029, POOL-003.
- A first-start object is never retired, unregistered or renamed: LIF-022.

Scenario 7:

- Replacing a template leaves its instances, and one being set up, as they were: INS-020,
  INS-013.
- A rename keeps the ID, and references held by instances follow the ID: OBJ-005, OBJ-010.
  A rename of a Conduit that would make two tool names equal reports conflict: CON-026.
- A control with a stale revision reports conflict, with none invalid: REV-002, REV-011.
- A control reconciled as having taken effect names the current revision and stores exactly
  the settings stated: REV-010, REV-013. On a retired instance that reports conflict: LIF-015.
- A control on a target that is not ready reports unavailable: CON-024.

Scenario 8:

- A bounded actor opens a Conduit for anyone, to a target and for operations its bound covers:
  ACC-006, ACC-029. It passes a right to open on only as it is or narrower: ACC-030.
- It applies a policy only where its bound names every actor and target: ACC-031.
- When its own Conduit to `access` is closed, what it opened and the policies it applied stay:
  ACC-030, LIF-023.
- No policy applies to `root`, and no request closes a Conduit of `root`: ACC-035, ACC-024.

Scenario 9:

- Two versions are installed side by side, and each template and instance keeps its own:
  NAM-011, NAM-010. The target's version of a contract governs a request: NAM-023, NAM-026.
- A re-export resolves to the package that originally exports it: PKG-013.
- A digest that does not match rejects the installation, and leaves a manifest item
  unresolved: PKG-026, PKG-021.
- `#name` reaches a contract of the same unpublished project, and as `from` a template that
  exists: NAM-022, INS-025. The template keeps the digest of SUB-008: NAM-009.

Scenario 10:

- A plain target is sent none of Conduit's keys, and its results become outcomes by one table:
  IFC-007, IFC-010. It supports no requirement, so no filtered Conduit is made to it and a
  policy makes its requests unavailable: CON-022, ACC-015.
- A Resource served by a plain implementation cannot name the invocation it serves, so its
  onward requests report refused: IFC-021.
- A result with both keys, with neither, or whose `isError` disagrees, is unknown: IFC-009,
  OPS-007.
- A client that is not an instance is presented no tool, and every call reports refused:
  IFC-032, IFC-012.

Scenario 11:

- A required output whose binding yields no value gives unknown if a step had an effect and
  failed otherwise; an optional one is left out: OPS-022, GRA-021.
- A default of an Operation's input is filled before dispatch; a control applies none:
  OPS-026, REV-006. A value reference to an optional input that was left out and has no default
  makes the step invalid: GRA-026.
- Cycles among steps, entries, components, compositions and re-exports are rejected: GRA-005,
  POOL-011, CON-019, CTR-005, PKG-014.

Scenario 12:

- Every decision, dispatch, end, state change, registration and revision change is recorded,
  with the caller, the Conduit, the target and how it ended: REC-001, REC-002.
- A purge removes records before a time, leaves a record of itself, and repeats are still
  recognized: REC-004. Unregistering erases no record: LIF-019, REC-003.
- A call to a tool that is not presented is recorded as refused: IFC-012.
