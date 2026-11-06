# Round 4, reader 3: undetermined behavior

Cold read of `spec/` alone, looking for behavior on which two engines that keep every promise
could still differ in a way a user or a conformance test could observe. I found 15 points:
1 `unsafe`, 10 `undetermined` and 4 `minor`. I read all of `spec/`: `README.md`,
`definitions.md`, `names.yaml`, `schema-profile.md`, `implementation-defined.yaml` (15 items),
every promise file under `promises/`, `conventions/` and `interface/` (437 promises by the
README's count), `core/` (its README and seven files), `schemas/` (its README and nine files),
and every example: 7 valid files, 61 invalid and 20 invalid-by-rule. Nothing was sampled. I
opened nothing outside `spec/`, and ran no checker.

Most of the scenarios I was given came out determined, usually by one promise and sometimes by
a chain of three or four. The gaps cluster where two features meet that were each specified on
their own: a composite Conduit used as a declared Conduit, later work and a retired caller, a
repeat that arrives through another Conduit, a replacement made from a template that has since
changed, and a purge followed by something that needs what it removed.

## Scenarios walked

The twelve that the brief names:

1. First start, a project directory with one package not installed, then installed.
2. An Agent's setup with a failed optional step, a start of unknown outcome and a declared
   Conduit that cannot open; every way on to ready, and to retired with shutdown confirmed.
3. A request through a Conduit with a filter while a policy applies, with onward requests to a
   graph and to a plain implementation, and a close or a policy change in mid-flight.
4. An invocation of unknown outcome: retrieval, reconciliation each way, repeats before and
   after, for a consumer, for a Resource's onward request and for a retired or unregistered
   instance.
5. Later work arranged directly, by an onward request, by a step and by a start; a start after
   each kind of change; a repeat of the start; a start of model work.
6. Retirement: a retire graph that fails halfway, a Pool with its Resources, a lost location,
   unregistering, and a new instance for each kind of nested instantiation.
7. Replacing a template, renaming, a stale `revision`, reconciling a control.
8. Delegation with a bound, policies with and without `version`, losing the right.
9. Packages: two versions, a re-export, a digest mismatch, installing twice, `#name`.
10. Plain implementations, a self-contradicting result, a client that is no instance, a tool
    that is no longer presented.
11. Graphs: outputs that are missing for each cause, defaults, self-invocation.
12. Records, purge and inspection after each of the above.

My own:

13. A repeat of a failed request that arrives through the replacement of a closed declared
    Conduit, under the same tool name.
14. `retire-with-resources.v1` asked a second time after the first failed.
15. Later work arranged through a chain, and the Agent at the head of the chain retired with
    its shutdown pending.
16. `register-replacement.v1` after the nested instantiation's own template was replaced, and
    after it was unregistered.
17. A purge, then a start of later work, a retrieval that names a served invocation, and the
    reconciliation of a runtime's start.
18. A declared Conduit made from a composite template, as a step of a setup and as the
    `conduit` of a step.
19. A policy named through a re-export while two versions of the originating package are
    installed.
20. An argument that is a map, where the input's schema gives a property a `default`.
21. A template whose `type` names nothing in a package that is installed.
22. A `retire` graph that invokes the stop operation of a runtime as an ordinary step.
23. A policy applied while a graph is half-way through its steps.
24. A Conduit that lists an operation which the target's version of the package does not hold.
25. A second control on a setting that an earlier control of unknown outcome addressed.
26. A second setup, asked by another actor, while a step of the first is unknown.
27. A Conduit instantiated on its own under the local name of a closed declared Conduit, and
    then a replacement asked for the declared one.
28. A policy that gives only `actors`, and that actor's own requests of `management`.
29. Reading records once every object a record names has been unregistered.

## Findings

### 4C-1. A start of later work while a Conduit of its chain has a retired consumer

- Severity: `unsafe`
- Scenario: 5 and 15. Agent `a` calls Resource `r` through Conduit K1 (`a` to `r`). To serve
  it, `r` makes an onward request through K2 (`r` to `t`) for an operation of `t` marked
  `arrangesLaterWork: true`. The work is tied to K2 and to K1. `a` is retired, its retirement
  fails, and its shutdown stays pending, so K1 stays open under CON-008. The implementation of
  `t` then asks for a start.
- Promises: ACC-019, ACC-039, ACC-040, ACC-003, IMP-006, CON-008.
- Quotes. ACC-040 asks only that the chain be open, and applies ACC-039 to the Conduit of
  dispatch alone:

  > that Conduit and every other Conduit to which the work is tied under ACC-019 are open

  > one through a Conduit whose consumer is retired reports refused under ACC-039

  ACC-019 and ACC-003 say something wider:

  > While all of those Conduits are open, the work is subject to the access they carry as it
  > stands when the work runs.

  > From the moment its consumer is retired, an open Conduit permits less: only the requests
  > of the consumer's retirement, under ACC-039.

  IMP-006 adds "the access they carry permits the start at that moment", which no chain Conduit
  can do literally, because K1 lists operations of `r` and not of `t`.
- Two behaviors. Engine A dispatches the start: every listed condition of ACC-040 holds, and
  the consumer of K2 is `r`, which is not retired. Engine B refuses it: K1 now carries only the
  requests of the retirement of `a`, so the access the chain carries "as it stands" permits no
  work for `a`. Under A the work, model work included, goes on starting for a retired caller
  for as long as its shutdown is not confirmed, which CON-008 lets be for ever. Where `a`
  arranged the work directly, the text is clear and the start is refused.
- Fix: add to the conditions of ACC-040 that no Conduit to which the work is tied has a retired
  consumer, or say in ACC-019 that a chain Conduit need only be open.

### 4C-2. A step whose `conduit` names a composite declared Conduit

- Severity: `undetermined`
- Scenario: 18. An Agent template declares the Conduit `checkout` from a composite template
  whose components `read` and `audit` both list `directory.read.v1`, with different filters. A
  step of its `setup` graph, or of a graph that implements one of its operations, writes
  `conduit: checkout` and `operation: ...#directory.read.v1`.
- Promises: GRA-024, CON-002, ACC-018, IFC-006, IFC-012, IFC-014.
- Quotes. GRA-024:

  > With `conduit`, whose value is the name of one of the template's declared Conduits, the
  > step is a request through that Conduit to its target

  > A step whose `conduit` names no declared Conduit of the template, or whose operation is
  > not one that Conduit lists, is rejected

  CON-002 makes "that Conduit" several: "one for each component where the declared Conduit is
  made from a composite template". The value of `conduit` is a name, so it cannot name a
  component, and IFC-014 lets two components list one operation because the component's name is
  part of the tool name.
- Two behaviors. Engine A sends the request through the component `read`, with its filter and
  its ID in `io.github.shrav-k.conduit/conduit`; engine B sends it through `audit`; a third
  rejects the template. The restrictions the target is sent differ. Where exactly one component
  lists the operation the answer can be inferred, and is still not stated.
- Fix: in GRA-024, reject a step whose `conduit` names a composite declared Conduit unless
  exactly one of its component Conduits lists the operation, and say that the request goes
  through that one.

### 4C-3. How the step of a composite declared Conduit ends when it opens in part

- Severity: `undetermined`
- Scenario: 1, 2 and 18. Setup reaches the step of the declared Conduit `checkout`, made from
  a composite with the components `read` and `settings`. The bound of the actor's Conduit to
  `access` names the operation of `read` and not the control that `settings` lists.
- Promises: ACC-005, CON-004, GRA-003, GRA-022, OPS-022, LIF-008.
- Quotes. ACC-005 and CON-004 speak of one Conduit:

  > The step is a request for `open.v1` that names that Conduit

  > opens the Conduit made for it if its state is registered. Where the Conduit is already
  > open nothing is dispatched, and the step is left as succeeded.

  OPS-022 has two wordings that part here: "failed only if no step had an effect, and
  otherwise unknown", and "unknown if any step succeeded with an effect or ended unknown".
  Neither `conventions/access.yaml` nor `promises/graphs.yaml` uses the word component.
- Two behaviors. One Conduit is opened and the other request is refused. Engine A ends the
  step with the reason refused and the setup with the outcome failed, since no step succeeded.
  Engine B gives the setup the outcome unknown, since an effect occurred. They also differ on
  the mixed states: whether the step is "left as succeeded" when one component Conduit is open
  and the other registered, what it reports when one is closed and the other open, whether a
  step that depends on `checkout` runs, and what "not opened" means for an optional one.
- Fix: say in GRA-022 that a declared Conduit made from a composite gives one step for each
  component Conduit, or define the one step: succeeded only when every component Conduit is
  open, and counted by OPS-022 as having had an effect when any was opened.

### 4C-4. A new attempt that goes through another Conduit

- Severity: `undetermined`
- Scenario: 4, 6 and 13. A consumer requests an operation through its declared Conduit
  `checkout` with the operation identity X, and the outcome is failed. That Conduit is closed,
  a replacement is registered under LIF-009 and opened, and the consumer sends the request
  again with X. LIF-009 keeps the tool name, and the sender, caller, target, identity and
  contents are the same, so OPS-024 dispatches a new attempt, which succeeds.
- Promises: OPS-008, OPS-009, OPS-012, OPS-024, OPS-017, ACC-034, ACC-019, LIF-009, REC-002.
- Quotes. OPS-017:

  > A retained result belongs to the Conduit through which its invocation was dispatched: its
  > content is delivered only while that Conduit is open, and never under another one

  OPS-024 says only "Like any dispatch, the new attempt needs the request to be permitted under
  ACC-001", and the definitions call a new attempt "a further dispatch of the same invocation".
  OPS-008 does not put the Conduit in the scope of an identity.
- Two behaviors. Engine A holds that the invocation was dispatched through the first Conduit,
  which is closed: the result of the attempt that just succeeded is withheld, and stays so.
  Engine B takes the Conduit of the latest attempt, and delivers the content. The same choice
  decides which close withholds a result under ACC-034, to which Conduit later work that the
  invocation arranged is tied under ACC-019, and which filters a start then carries.
- Fix: one sentence in OPS-024 and OPS-012: an invocation counts as dispatched through the
  Conduit of its latest attempt. Or add the Conduit to the scope in OPS-008, so that the
  request through the replacement is a new invocation.

### 4C-5. A replacement made after the nested instantiation's template changed

- Severity: `undetermined`
- Scenario: 6, 7 and 16. The Agent `reviewer-1` holds the declared Conduit `checkout`, whose
  `from` is the registered template `directory-read`. The Conduit is closed. `directory-read`
  is replaced with `replace.v1`, so that it lists one more operation, or is unregistered. An
  actor then asks for `register-replacement.v1`.
- Promises: LIF-009, INS-013, INS-020, OBJ-010, OBJ-014, INS-002.
- Quotes. LIF-009:

  > A new instance is instantiated from the nested instantiation as the instance it belongs to
  > holds it in its resolved definition

  > That the template was replaced or unregistered in the meantime changes none of this. The
  > rules on instantiation are applied to the nested instantiation as the instance holds it,
  > with its references already resolved, and as the objects stand when the request is made.

  INS-013 says the instance holds "the nested instantiations its template declared", and
  OBJ-014 that a nested instantiation "holds `from`". "The template" in the quoted sentence can
  be the consumer's template or the one that `from` names.
- Two behaviors. Engine A holds `from` as a resolved pointer and makes the Conduit from
  `directory-read` as it now stands: the replacement lists the added operation, and after the
  template is unregistered the request reports invalid. Engine B holds what `from` resolved to
  when `reviewer-1` was made: the replacement lists the old operations, and is made even after
  the template is unregistered. The operations that the consumer can reach differ, and so do
  the packages the new instance holds.
- Fix: say in INS-013 whether an instance holds, for each nested instantiation, the resolved
  definition of the template that `from` named or only the pointer, and make the sentence of
  LIF-009 name which template it means.

### 4C-6. `retire-with-resources.v1` and a Resource that is already retired

- Severity: `undetermined`
- Scenario: 6 and 14. A first request fails because the retirement of the Resource `x` failed,
  so its shutdown is pending. The cause is removed and the request is made again.
- Promises: LIF-021, LIF-026, LIF-030, and the contract of `retire-with-resources.v1`.
- Quotes. LIF-021:

  > Once the retirement of one Resource has ended without its shutdown confirmed, the request
  > dispatches no further retirement.

  The contract says "Each Resource is retired", and LIF-026 that a request to retire such an
  instance "can be dispatched, as a further invocation of `retire.v1`".
- Two behaviors. Engine A dispatches a further `retire.v1` for `x`, which now succeeds, and
  goes on to retire the Pool. Engine B finds that the retirement of `x` has ended without its
  shutdown confirmed, dispatches nothing, and reports failed until `x` is retired by a request
  of its own. They differ in the records too: whether a Resource whose shutdown is confirmed
  gets an invocation of `retire.v1` that changes nothing under LIF-030, or none.
- Fix: say in LIF-021 that the request dispatches a `retire.v1` for each Resource the Pool owns
  whose shutdown is not confirmed, one that is already retired included, and none for a
  Resource whose shutdown is confirmed; and that "has ended" means a retirement this request
  dispatched.

### 4C-7. The outcome of a further run when a step is left as it was

- Severity: `undetermined`
- Scenario: 2 and 6. In the first retirement of an instance the step `flush`, of an operation
  that has an effect, ends unknown. A further retirement under LIF-026 does not dispatch it,
  under OPS-021, and the engine's choice under DL1 is to dispatch no step that already
  succeeded. Nothing is dispatched in the second run.
- Promises: OPS-021, OPS-022, OPS-028, LIF-024, GRA-026, DL1.
- Quotes. OPS-022:

  > the outcome is failed if every step that was dispatched either failed, was answered with a
  > reason in place of an outcome under IFC-026, or is of an operation whose contract states
  > no `effect`, and is unknown if any step succeeded with an effect or ended unknown on an
  > operation that has one

  LIF-024 then reads the latest invocation: "unknown when the outcome of the latest invocation
  of `retire.v1` for it is unknown", and "pending otherwise".
- Two behaviors. Engine A counts the steps of this run: none was dispatched, none had an
  effect, the outcome is failed and the shutdown reports pending. Engine B counts the step as
  it stands: `flush` ended unknown, the outcome is unknown and the shutdown reports unknown.
  DL1 leaves which steps are dispatched to the engine, and not how an undispatched step counts.
  The same question decides whether a later run can succeed where a step that succeeded
  before, or was reconciled, is not dispatched again: OPS-022 must count it as succeeded for
  that, which is the standing reading, and no promise says so.
- Fix: one sentence in OPS-022: in a further run a step that is not dispatched counts with the
  outcome it has from the earlier run or from its reconciliation.

### 4C-8. A purge removes what later requests are decided from

- Severity: `undetermined`
- Scenario: 5, 12 and 17. The root Interface purges the records made before a time. After it,
  an implementation asks for a start of later work that an invocation before that time
  arranged; a Resource retrieves the result of an onward request and names, in `serving`, an
  invocation before that time; an actor reconciles the start of a runtime as having occurred.
- Promises: REC-004, ACC-040, ACC-019, ACC-041, IFC-024, OPS-017, TPL-015.
- Quotes. REC-004 gives a closed list, and says it was built from what a reconciliation needs:

  > It keeps, for every invocation, its operation identity, as OPS-008 scopes it, the engine's
  > ID for it, its outcome, a digest of its contents, its operation, the instance it was
  > dispatched to and the target that authorized it

  ACC-040 needs more: the start is dispatched "for the caller of the arranging invocation and
  through the Conduit through which the work was arranged", and a request that names an
  invocation "to which no Conduit is tied, reports refused". IFC-024 needs the caller of the
  served invocation, OPS-017 the Conduit of the retained result, and TPL-015 the runtime that
  the start was for.
- Two behaviors. The list holds no Conduit, holds the caller only as part of the scope of an
  identity, which an invocation need not carry, and holds no name of a runtime. Engine A keeps
  exactly the list: the start reports refused, the retrieval finds no caller and reports
  refused, and the handle of a reconciled start cannot be given to one runtime where two share
  a start operation, as `watcher` and `path-watcher` do in `schemas/examples/valid/project`.
  Engine B keeps the ties, the callers and the step as state that is not a record, and
  dispatches, answers and stores as before the purge.
- Fix: add to the list of REC-004 the caller, the Conduit and the chain of each invocation and,
  for a step, its name; or say that the ties of ACC-019, the Conduit of a retained result and
  what an instance holds are stored state that a purge does not touch.

### 4C-9. The `version` of a policy whose requirement is named through a re-export

- Severity: `undetermined`
- Scenario: 8, 9 and 19. `example.org/files` 0.3.0 re-exports the requirement `rate.v1` of
  `example.org/common`, and lists `common` at 2.0.1. Installed are `files` 0.3.0, `common`
  2.0.1 and `common` 2.1.0. An actor asks for `apply-policy.v1` with the requirement
  `example.org/files#rate.v1`.
- Promises: ACC-031, ACC-042, PKG-013, MAN-008, NAM-014.
- Quotes. ACC-031:

  > the request may name one in `version`, the version of the requirement's package; where it
  > names none, the version is the one installed version of that package that exports the
  > requirement

  PKG-013 says a selection of a re-export "resolves to the package that originally exports it",
  and ACC-029 settles the same point for a bound, which "names both". Nothing settles it for a
  policy.
- Two behaviors. With no `version`: engine A takes the package the reference writes, finds one
  installed version of `files`, applies the policy, and keeps `files` 0.3.0 and its digest;
  engine B takes the originating package, finds two installed versions of `common`, and
  reports invalid under ACC-042. With `version: 2.1.0` the two swap: A reports invalid and B
  applies the policy under the contract of `common` 2.1.0. What inspecting `access` lists for
  the policy differs as well.
- Fix: say in ACC-031 that `version` is a version of the package whose address the reference
  writes, and that a re-export is then followed through that package's own manifest, as
  NAM-014 has it for a document.

### 4C-10. Property defaults in a schema that is not a type's

- Severity: `undetermined`
- Scenario: 3, 4, 11 and 20. An Operation's input `options` has the schema
  `{type: object, properties: {depth: {type: integer, default: 1}}, required: [depth]}`. A
  request supplies `options: {}`. The same shape can stand in the schema of a Requirement's
  input, for the arguments of a filter or a policy, and of a template's input.
- Promises: SCH-008, SCH-009, SCH-010, SCH-001, INS-006, INS-019, OPS-026, OPS-009, NAM-019.
- Quotes. SCH-009 has a narrow situation and a wide rule:

  > A template is instantiated and a property that has a `default` is absent from a map that
  > is present.

  INS-019 speaks only of "the type schema's default, applied under SCH-009", INS-006 says each
  destination "holds exactly the supplied value", and OPS-026 fills only the `default` of an
  input that was left out. SCH-010 does apply nested defaults to a map that a default supplies.
- Two behaviors. Engine A applies no property default to a supplied argument: the request
  fails `required` and reports invalid. Engine B fills `depth` first: the request is
  dispatched with `{depth: 1}`. They also differ on whether a request that supplies `{}` and
  one that leaves `options` out, where the input's `default` is `{}`, have the same contents
  under OPS-009 or conflict under OPS-010, on what the `arguments` of a restriction hold, and,
  for a template's input, on whether `declared` shows `{}` or `{depth: 1}`.
- Fix: say in SCH-009 which schemas it applies: the type's schema alone, with every argument
  checked and passed on as supplied; or every schema that governs an argument.

### 4C-11. A selection that names nothing where the package is at hand

- Severity: `undetermined`
- Scenario: 1, 9 and 21. A template writes `type: example.org/files#local-folder.v1`. The
  package is installed at the version the manifest lists, and exports no such name. Or it
  writes `type: '#notebook.v1'`, and its project holds no document of that name. The same
  holds for a step's `operation`, a value of the `implementation` map, a `capability`, and a
  qualified `from` in a nested instantiation.
- Promises: NAM-022, PKG-018, OBJ-008, PKG-006, TPL-002, CTR-004, INS-022.
- Quotes. NAM-022 and PKG-018 stop at the word:

  > Where no document and no implementation of that project or package has the name, the
  > reference is unresolved

  > is unresolved, with a diagnostic naming it

  The definitions use "unresolved" for the two cases that do not reject, an object that does
  not exist and a package that is not installed. TPL-002 rejects a template "with no `type`,
  with a type of another family", and CTR-004 rejects a Type "whose default implementation
  cannot be resolved". No promise says what a template with an unresolved selection is.
- Two behaviors. Engine A rejects the template: the registration reports invalid and nothing
  in the submission is registered. Engine B registers it and reports the name in a diagnostic
  that did not prevent it, as OBJ-008 and PKG-006 do, and rejects the instance document later
  under INS-022.
- Fix: one promise: a selection, or a qualified `from`, that names nothing in a package that
  is installed, and a `#name` that names nothing in its own project or package, makes the
  document that holds it rejected.

### 4C-12. A step that invokes the stop operation of a runtime

- Severity: `minor`
- Scenario: 2, 6 and 22. A `retire` graph holds a step whose `operation` is the stop operation
  of the runtime `watcher` and whose argument is `{source: runtime, name: watcher}`, so that
  another step can depend on it. The step succeeds.
- Promises: TPL-007, TPL-009, TPL-015, IFC-011.
- Quotes. TPL-007: the instance holds the handle "until a stop of the runtime has succeeded".
  TPL-009: the engine "then invokes the stop operation of each runtime the instance still
  holds". TPL-015 counts as stops only "a stop under TPL-009 or OBS-007".
- Two behaviors. Engine A takes the step as a stop of the runtime: the holding ends and the
  retirement succeeds. Engine B does not: the engine stops the runtime again with a handle
  that the implementation no longer recognizes, the stop fails under IFC-011, and the shutdown
  is pending after every retirement. TPL-015 points to B, by an inference.
- Fix: say in TPL-007 that only the stop that the engine itself invokes under TPL-009 or
  OBS-007 ends the holding, or that any successful invocation of the stop operation with the
  held handle does.

### 4C-13. When a policy is judged for a step that names no Conduit

- Severity: `minor`
- Scenario: 3 and 23. A graph implements the requested operation and has three steps with no
  `conduit`. A policy that names the caller is applied after the first step, or one that
  applied is removed.
- Promises: ACC-017, ACC-018, ACC-036, IFC-018.
- Quotes. ACC-018: such a step "carries the restrictions of the invocation that its graph is
  serving". ACC-017, after saying that a policy is judged anew for an onward request: "A step
  with no `conduit` carries the restrictions of the invocation its graph is serving in the
  same way". ACC-036 is about "a request", and ACC-018 says the step "is not a request through
  any Conduit".
- Two behaviors. Engine A sends the later steps the policies as they stand when each step is
  dispatched. Engine B sends every step the list that the graph's invocation was dispatched
  with. The words "in the same way" point to A.
- Fix: say in ACC-018 that the policies a step carries are judged when the step is dispatched.

### 4C-14. The definition of a tool whose operation the target's version does not hold

- Severity: `minor`
- Scenario: 9 and 24. A Conduit template whose project holds `example.org/files` 0.4.0 lists
  an operation that 0.4.0 added to a capability, as NAM-024 allows. The target was made from a
  template that holds 0.3.0, which has no contract of that name. The Conduit is instantiated
  and opened, under NAM-017.
- Promises: IFC-012, IFC-033, NAM-023, NAM-026.
- Quotes. IFC-012: the tools are "exactly the operations its open Conduits list". IFC-033:
  the definition "is derived from the Operation contract as the target holds it". NAM-026
  gives the request: "not dispatched and reports unavailable".
- Two behaviors. The target holds no such contract. Engine A presents the tool with a
  definition taken from the version the Conduit holds. Engine B cannot derive one and leaves
  the tool out, so a call is answered with the protocol's error and the reason refused, where
  NAM-026 has it report unavailable in a result.
- Fix: say in IFC-033 that where the target's version holds no contract of the operation, the
  definition is derived from the contract as the Conduit holds it.

### 4C-15. No stated way to read the records of an object that no longer exists

- Severity: `minor`
- Scenario: 6, 12 and 29. Every object that a record names has been unregistered: a refused
  request of an Interface that was later retired and unregistered, for example, or what a
  purge kept of an invocation "as records of that instance" once that instance is gone.
- Promises: REC-001, REC-003, REC-004, LIF-019, LIF-013, OPS-029, DL8.
- Quotes. REC-001: "Each such event can be found by inspecting the objects it names." REC-003:
  "The record can still be read after the engine has restarted and after the objects it names
  are unregistered." OPS-029: "An object that has been unregistered no longer exists", so
  `inspect.v1` that names it reports invalid.
- Two behaviors. An engine whose only way to read records is `inspect.v1` cannot show these
  records, and one that offers a feed under DL8 can. No vector can test REC-003 or LIF-019 for
  this case through an operation that the specification names.
- Fix: say that records are also read in the way DL8 leaves to the engine, and that an engine
  must offer one that reaches the records of unregistered objects; or let `inspect.v1` of
  `management` return them.

## Walked and found determined

Each line is a step where the text gave one result, with the promise that gave it.

- A submission whose instance document needs a package that is not installed registers
  nothing and reports invalid; the templates alone would register with a diagnostic: PKG-007,
  SUB-003, OBJ-023, PKG-006. After `install.v1` the same submission is decided afresh:
  OPS-025.
- A directory handed over gives what `register.v1` gives for the same values; which actor
  asks, and whether packages are installed first, are the engine's: SUB-010, DL7.
- The root Interface cannot set up a new Pool's entries until it has an open Conduit to that
  Pool that lists `set-up.v1`; that Conduit can be opened while the Pool is registered:
  POOL-015, CON-023, CON-024.
- Entries that refer to each other in a cycle reject the Pool template: POOL-011. A reference
  to an entry from outside resolves in the same submission: POOL-014, SUB-002.
- A setup with a failed optional step, a start of unknown outcome and a required declared
  Conduit that is not opened has the outcome unknown where the start operation states an
  effect, and the Agent stays registered: OPS-022, GRA-003, LIF-004. What it opened stays
  open: LIF-027.
- From there to ready: reconcile the `set-up.v1` invocation as having occurred, which makes
  the Agent ready whatever its steps show: LIF-028. Or reconcile the start, with the handle or
  as not having occurred, cure the Conduit, and set up again: TPL-015, TPL-007, CON-004,
  OPS-021, OPS-028.
- A declared Conduit whose Conduit is closed makes the step report conflict until a
  replacement is registered, and a replacement whose target is retired reports invalid:
  LIF-008, LIF-009, CON-025.
- A retirement that succeeds while a start is still unknown reports the shutdown unknown;
  reconciled as not having occurred it is confirmed, and reconciled with a handle it is
  pending until a further retirement stops the runtime: LIF-024, TPL-015, LIF-026.
- A second setup asked by another actor while a step is unknown dispatches no new attempt of
  that step, because OPS-012 needs the same sender and caller: OPS-021, OPS-012. This takes an
  inference from "where OPS-012 dispatches one".
- A filter and a policy reach the target's implementation as two items; an onward request
  carries both to the third instance: IFC-018, ACC-017. To a plain implementation the onward
  request reports unavailable and names the carried restriction: ACC-015. To a graph it is
  checked against the implementations of its steps that have no `conduit`: CON-022, ACC-015.
- Closing the first Conduit in mid-flight lets every invocation in progress run to its end,
  withholds the first result, and refuses every later onward request in the chain, a step with
  `conduit` among them: ACC-034, ACC-018. A policy applied in mid-flight limits the next
  onward request and not one already dispatched: ACC-017, ACC-036.
- A lost result is found by identity and target with no open Conduit; after the close its
  content is withheld, and the engine's ID is still told: OPS-016, OPS-017, IFC-024.
- A repeat before reconciliation is a new attempt only with `deduplicates: true` and an
  implementation that is not plain: OPS-012. After a reconciliation as having occurred it is
  told succeeded, with `isError` true and no outputs: OPS-009, IFC-025, GRA-026. After one as
  not having occurred it is dispatched as a new attempt: OPS-024. A second reconciliation
  reports conflict: OPS-030.
- A Resource retrieves the result of its onward request by naming the served invocation, in
  progress or not, and may give `target` in the resolved form: IFC-024, IFC-028, LIF-031. A
  repeat of the onward request itself needs the served invocation to be in progress: IFC-028.
- For a retired target the retrieval still tells the state, and a reconciliation is taken
  where it stores nothing: LIF-007, LIF-015. For an unregistered one the retrieval reports
  refused and the reconciliation goes through `management`: OPS-015, ACC-033, OPS-031.
- A start of later work is refused after any Conduit of the chain is closed, while the
  arranging invocation is in progress, failed or unknown, and for a start of model work that
  an unmarked operation arranged: ACC-040, ACC-019, IMP-008. It is dispatched once the
  arranging invocation is reconciled as having occurred: ACC-040. A policy applied since
  limits it: ACC-041. Replacing the target's template changes nothing: INS-020. A repeat with
  one identity is the same start: ACC-040, OPS-009. Where the consumer of the Conduit of
  dispatch is retired the start is refused: ACC-039.
- A retire graph that fails halfway leaves the instance retired with its shutdown pending and
  its Conduits as consumer open for the retirement alone: LIF-005, LIF-024, CON-008, ACC-039.
- `retire-with-resources.v1` stops at the first Resource whose shutdown is not confirmed,
  reports failed with diagnostics, and leaves the Pool as it was; the order within a cycle is
  the engine's: LIF-021, DL1.
- Declaring a location lost retires what is on it, keeps a confirmed shutdown confirmed,
  reports the rest unknown, closes their Conduits at once and lets each be unregistered;
  `local` cannot be declared lost, so one location that is not retired always exists: PLC-017,
  CON-008, LIF-017, LIF-022.
- A new instance is refused with conflict while the old one is not retired, and with invalid
  for a component Conduit that an instance document made: LIF-029, POOL-003, LIF-009.
- A standalone Conduit may take the local name of a closed declared Conduit, and the
  replacement for the declared one then reports invalid for the equal tool name: IFC-014,
  LIF-009.
- Replacing a template changes no instance, and none that is being set up, because an
  instance runs its own resolved definition: INS-020, INS-013. An equal document with equal
  packages changes nothing and succeeds; an equal document with another digest advances the
  revision; a document that needs a package and comes with no manifest is rejected: SUB-009.
- A rename keeps the ID and every stored reference; to the same name it changes nothing; to a
  taken name it reports conflict: OBJ-005, OBJ-010, OBJ-026, OBJ-027. A stale `revision`
  reports conflict and a missing one invalid: REV-002, REV-011.
- A setting that a control of unknown outcome addressed is reported unknown until that control
  is reconciled, whatever a later control stored; the reconciliation names the current
  revision: REV-009, REV-010, REV-013.
- A bounded actor opens a Conduit only where its bound covers the target and names every
  operation, and never one that lists an operation of an unpublished package: ACC-006,
  ACC-029. Closing its own Conduit to `access` afterwards closes nothing it opened, and its
  policies stay until `remove-policy.v1` names them: ACC-030, ACC-031.
- With two installed versions that both export the requirement directly, a policy with no
  `version` reports invalid, and one that names either is checked against it: ACC-042. A
  policy that names the root Interface reports refused: ACC-043. A composition that reaches
  `bound.v1` is rejected: CTR-011.
- A policy that gives only `actors` applies to that actor's requests of `management` too, and
  they report unavailable, since `engine` states only `bound.v1` in `supports`: ACC-031,
  ACC-015, MAN-012. Determined, and worth a sentence of warning.
- Two versions of a package sit side by side; the target's version of a contract governs:
  NAM-011, NAM-023. A digest mismatch reports invalid; an installed package asked for again
  succeeds, or conflicts with another digest, with nothing fetched: PKG-026, PKG-027, PKG-025.
  A package that holds an instance document is rejected: MAN-011.
- `#name` reaches the project's own contracts and, as `from`, the templates that exist:
  NAM-022, INS-025, SUB-008.
- A plain implementation is sent no key, gets its outcome from the table, is never sent a new
  attempt after unknown, and cannot be selected for an operation that arranges later work:
  IFC-007, IFC-010, OPS-012, TPL-014. Serving a Resource, it cannot make an onward request:
  IFC-021.
- A result that states both an outcome and a reason, or whose `isError` disagrees, is unknown:
  IFC-009, OPS-007. A client that is no instance is refused on every tool: IFC-032. A call to
  a tool no longer presented gets the protocol's error with the reason refused: IFC-012,
  IFC-013.
- A step that reads an output of a step that did not run, was left as succeeded or was
  reconciled reports invalid; a reference to a step that names a runtime is rejected; an
  output binding that selects nothing gives failed or unknown unless the output is marked
  optional: GRA-002, GRA-026, GRA-028, OPS-022, GRA-021. A graph that reaches its own
  operation by any selection is rejected: GRA-005.
- A purge keeps the identity with its scope, so a repeat is still answered and other contents
  still conflict: REC-004, OPS-010.
