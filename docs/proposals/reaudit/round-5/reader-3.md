# Round 5, reader 3, behavior on one engine

I read everything under `spec/` at commit d0c425f, on the branch `docs/pre-codex`: the README,
`definitions.md`, `names.yaml`, the implementation-defined list, the schema profile, the 15
promise files, the 7 convention files, the interface file, the core package (its operations,
capabilities, types, templates, requirements and results, part of its references, its README
and its manifest), the schemas README and the valid examples. I read the schema contracts and
the invalid examples only where a scenario needed them, and I opened nothing outside `spec/`. I
walked the ten scenarios end to end, citing the promise that decides each step.

**Counts.** 16 findings: 1 unsafe, 1 contradiction, 1 wrong, 10 undetermined, 1 misleading
and 2 minor.

**Judgment.** Most steps of the ten walks have a promise that decides them: first start, a
registration before and after an install, the access decision with a filter, a bound and onward
requests, a close in mid-flight, repeats before reconciliation, purge, replacing a template, a
restart during a setup, and rotating a secret. The defects are where two mechanisms meet:

- A repeat answered from the stored outputs never meets the filter of the Conduit it arrives
  through, which defeats the way that ACC-011 gives to narrow access (01).
- Reconciliation does not mesh with new attempts, later retirements, controls in progress and
  opens (02, 04 to 06 and 08).
- A retired instance whose retire hook needs a secret that it can no longer read is never
  removed (03).
- The programs that a directory submission resolves are outside the digest that is to identify
  them (10, 11).

Only 01 is unsafe. Its case is narrow, but it is the very case that ACC-011 recommends.

---

## 5-3-01

**Severity:** unsafe. **Title:** A repeat answered from the stored outputs bypasses the filter of
the Conduit it arrives through.

**Promises:** OPS-008, OPS-009, OPS-024, ACC-002, ACC-008, ACC-011, ACC-014, ACC-036 and IFC-019.

**Quotes:**

- OPS-024: "OPS-008 does not put the Conduit in the scope of an idempotency key, so the request is
  a repeat all the same."
- OPS-009: "Nothing is dispatched, and the result is that invocation's: the outcome succeeded and
  its outputs."
- ACC-008: "It is permitted only if the Conduit lists the operation and every restriction that
  applies permits it."
- ACC-011: "To narrow access, the Conduit is closed and a narrower one, which lists fewer
  operations or applies further filters, is opened."

**Situation (scenario 3).**

1. Agent A is the consumer of Conduit C2 to Resource T. C2 lists `t.read.v1` and has no filter.
2. A requests `t.read.v1` with the idempotency key k. The invocation succeeds, but A loses the
   result.
3. The operator narrows A's access as ACC-011 says: it closes C2 and opens C, which lists the same
   operation and applies a redaction filter F.
4. A repeats with k through C. The engine's own decision (ACC-002, ACC-036) finds an open Conduit
   that lists the operation, so the request is permitted. OPS-009 then answers from the stored
   outputs, and dispatches nothing.

Only an implementation applies F (IFC-019), and none is called. So A receives, through the
Conduit that F governs, the content that F forbids. An engine that keeps every promise does
this. An engine that reads ACC-008 or ACC-014 strictly takes F as a restriction that cannot be
evaluated without a call, and reports unavailable instead.

**Least change:** In OPS-009, and in the same way in OPS-034, answer from the stored outputs only
where the repeat comes through a Conduit that applies the same filters as the Conduit of the
invocation's latest attempt, which OPS-008 already keeps. Otherwise the repeat reports conflict,
with a `ruleId` of its own.

## 5-3-02

**Severity:** contradiction. **Title:** An invocation that is reconciled once and then attempted
again can end unknown, and no promise says whether it can be reconciled a second time.

**Promises:** OPS-013, OPS-014, OPS-021, OPS-022, OPS-024, OPS-030 and LIF-013.

**Quotes:**

- OPS-030, when: "whose outcome is not unknown: it is in progress, it has ended with the outcome
  succeeded or failed, or it has been reconciled already."
- OPS-030, observable: "A second reconciliation of one invocation reports conflict in this way,
  whichever way the first recorded the effect."
- OPS-024: "The invocation's state is then in progress, and afterwards the outcome of the new
  attempt."
- OPS-013: "The outcome stays unknown until it is reconciled."

**Situation (scenario 4).**

1. An invocation I of `mirror.sync.v1`, which does not deduplicate, ends unknown.
2. An actor reconciles I as not having occurred, so its outcome is failed.
3. The caller repeats with the same key. OPS-024 dispatches a new attempt, which also ends
   unknown, after a cancellation or a restart. I's outcome is unknown again.

OPS-014 lets an actor reconcile I. The `when` of OPS-030 does not hold, because the outcome is
unknown, but its observable has a second reconciliation report conflict. Engine A accepts the
reconciliation. Engine B reports conflict, so I stays unknown for good: OPS-012 answers every
later repeat with unknown.

Engine B also drops I from `unreconciled`, which lists only invocations "not yet reconciled"
(LIF-013). For a setup hook under engine B, every further setup ends unknown for good (OPS-021,
OPS-022).

**Least change:** In OPS-030, drop "or it has been reconciled already" and the sentence about a
second reconciliation. In their place, say that a reconciliation settles the outcome of the
invocation's latest attempt, so a new attempt dispatched after a reconciliation that ends
unknown may be reconciled again.

## 5-3-03

**Severity:** wrong. **Title:** A retired instance whose retire hook needs a secret that it can no
longer read is never removed.

**Promises:** LIF-017, LIF-020, LIF-021, LIF-022, LIF-024, LIF-026, TPL-018, SEC-008, SEC-012,
SEC-013, CON-008, CON-025, CON-031, OPS-022 and OPS-030.

**Quotes:**

- LIF-026: "It is the way from a shutdown that is pending, after a retirement that failed, to one
  that is confirmed."
- `retire-with-resources.v1`: "so a request made again finishes what this one could not."
- TPL-018: "Where the hook is to be dispatched, its implementation is a `remote` item and no
  connection to it lasts whose values the engine holds, the engine first connects to it". Also:
  "Where the run or the connection cannot be had, as where a read fails under SEC-012, the hook is
  not dispatched and reports unavailable".
- CON-031: "A Conduit in the state registered whose consumer is retired stays registered until
  CON-008 closes it, once the consumer's shutdown is confirmed, and is never opened".
- LIF-017: "The request is dispatched only where the instance is retired and its node was declared
  lost".

**Situation (scenarios 2 and 10).** An Agent placed on `local` has no item that is run. It
selects a `remote` item that is not plain as its retire hook, whose `headers` bind a secret
reference, and it declares the Conduit `vault` to that secret Resource. Any of these makes the
read impossible for good:

- The setup was sent by an actor other than the root Interface, so the part for `vault` was
  refused and `vault` stays registered (ACC-005, SEC-008).
- The root Interface closed `vault`, or retired the secret Resource, which closes `vault` under
  CON-008, and the engine then restarted. The connection's values are then lost, so TPL-018
  connects anew.

Now the Agent is retired:

1. No invocation of the hook exists, so the hook is to be dispatched.
2. The read is refused, so the hook's part reports unavailable.
3. The Agent has no stop to ask for, so under OPS-022 the retirement fails, and the shutdown is
   pending.
4. Every further retirement does the same, because the hook has still never been dispatched.

No way out exists:

- No Conduit can be opened, or made, for a retired consumer (CON-031, CON-025).
- A failed retirement cannot be reconciled (OPS-030).
- Unregistering needs a node declared lost (LIF-017), and `local` cannot be declared lost
  (LIF-022). On another node, the only way out retires every instance placed there.

The Agent stays retired, with its shutdown pending and its name held, for good. A Resource in
the same state keeps its Pool from ever being retired (LIF-020, LIF-021). The quoted statements
are false of both.

**Least change:** In LIF-017, also let the root Interface unregister a retired instance whose
shutdown is not confirmed, with the stop of LIF-036 asked for once more. PLC-027 already calls
unregistering the removal of a finalizer by hand.

## 5-3-04

**Severity:** undetermined. **Title:** A control that ends after its instance was declared lost.

**Promises:** REV-003, REV-007, REV-009, REV-010, REV-012, LIF-015, PLC-017, PLC-024, ACC-034 and
OPS-031.

**Quotes:**

- REV-012: "each takes effect at once, whatever setup, retirement or change of the instance is in
  progress, and TPL-012 says what a setup in progress then does."
- REV-007: "It has applied exactly the values it was sent, and exactly those values are stored".
- REV-010: "A retired instance takes the reconciliation that stores nothing, and one that would
  store values in it reports conflict, under LIF-015."

**Situation (scenario 5).** A control that names the revision r is dispatched to a Resource.
While it runs, the root Interface declares the Resource's node lost, or the machine provider
reports its machine gone. The Resource is retired at once, and ACC-034 lets the invocation run
to its end. It ends succeeded:

- Engine A stores the values and advances the retired instance's revision (REV-007, REV-003).
- Engine B stores nothing, as REV-010 has a reconciliation store nothing in a retired instance.

What an inspection shows differs. Where the control ends unknown instead, the only
reconciliation that the retired instance takes is one that records that the effect did not
occur (REV-010, OPS-031), even where the actor checked that it did.

**Least change:** In REV-012, say what a change in progress does when its instance is retired
under PLC-017 or PLC-024: for example, its outcome is recorded and nothing is stored. Let a
reconciliation of it record that it took effect while storing nothing.

## 5-3-05

**Severity:** undetermined. **Title:** An earlier retirement is reconciled as having occurred
while a later one has failed or is in progress.

**Promises:** LIF-024, LIF-026, LIF-028, LIF-035, REV-014, OPS-030, CON-008 and PRV-027.

**Quotes:**

- LIF-024: "An instance may be retired more than once, under LIF-026, and it is the latest
  invocation that counts."
- LIF-028: "Recording that a retirement or a close occurred confirms the shutdown."
- REV-014: "A control is a change, and so is a reconciliation that stores the values of a control".

**Situation (scenarios 2 and 4).** A retirement R1 ends unknown, and a further retirement R2 is
dispatched under LIF-026. An actor then reconciles R1 as having occurred. OPS-030 looks only at
R1, which is unknown, and REV-014 holds back no reconciliation but one that stores a control's
values.

- Engine A dispatches the reconciliation. The shutdown is confirmed, and the instance is removed
  (LIF-035) and its Conduits closed while R2 is still dispatching its hook or asking for the
  stop. That stop names an instance that no longer exists, whose node PRV-027 no longer promises.
- Engine B reports conflict until R2 has ended.

The same disagreement arises with no overlap in time. Suppose R2 failed, which leaves the
shutdown pending, and R1 is then reconciled as having occurred. LIF-028 confirms the shutdown,
while LIF-024 says that the latest retirement, R2, counts, which leaves it pending.

**Least change:** In REV-014, count a reconciliation of a setup or of a retirement of an instance
as a setup or a retirement of that instance. In LIF-024, say that a reconciliation that records
any retirement as having occurred confirms the shutdown, whatever a later retirement ended with.

## 5-3-06

**Severity:** undetermined. **Title:** An open of unknown outcome is reconciled as having occurred
after the Conduit's consumer was retired.

**Promises:** LIF-004, LIF-028, CON-031, ACC-039 and IMP-007.

**Quotes:**

- LIF-028: "Recording that a setup or an open occurred makes the instance ready or the Conduit
  open."
- LIF-028, observable: "an instance that was retired in the meantime stays retired".
- CON-031: "A Conduit in the state registered whose consumer is retired stays registered until
  CON-008 closes it, once the consumer's shutdown is confirmed, and is never opened".

**Situation (scenarios 2 and 9).**

1. An `open.v1` of an Agent's declared Conduit `vault`, to a secret Resource, is in progress when
   the engine restarts. It ends unknown (IMP-007), and `vault` stays registered (LIF-004).
2. The Agent is retired, and its shutdown is pending.
3. An actor with `reconcile.v1` on `access` records the open as having occurred.

Engine A opens `vault` under LIF-028, and ACC-039 then lets the reads of the retirement go
through it. A retired consumer so gains access that it never had while it was ready. Engine B
leaves `vault` registered under CON-031. LIF-028 says what becomes of an instance retired in the
meantime, and nothing of a Conduit whose consumer was.

**Least change:** In LIF-028, add that a reconciliation that records an open as having occurred,
where the Conduit's consumer is retired, records the outcome and leaves the Conduit as it is,
under CON-031.

## 5-3-07

**Severity:** undetermined. **Title:** `retire-with-resources.v1` arrives while the Pool's own
setup is in progress.

**Promises:** REV-014, LIF-021, LIF-025, LIF-035, POOL-013, CON-024 and OPS-005.

**Quotes:**

- REV-014: "A request to set up, retire or change an instance arrives while another setup,
  retirement or change of it, the engine's own included, has been dispatched and has not ended".
- LIF-021: "the retirement of it that the request would dispatch is not dispatched and reports
  conflict, under REV-014".
- LIF-021: "Where the shutdown of every Resource is confirmed, the Pool's retirement is dispatched
  as an invocation of `retire.v1` of its own, to which LIF-024 applies, and the outcome of the
  request is the outcome of that retirement."

**Situation (scenario 6).** A Pool's setup is setting up its entry `a`, which is busy. Its entries
`b` and `c` refer to `a`, so their parts have not begun. An actor asks the Pool for
`retire-with-resources.v1`, which CON-024 dispatches while the Pool is registered.

- Engine A takes it as a request to retire the Pool, and reports conflict at once under REV-014.
  Nothing changes.
- Engine B dispatches it. `b` and `c` are retired first, since they refer to `a`. They were
  never set up, so they are removed at once (LIF-025, LIF-035). The retirement of `a` reports
  conflict, so the Pool is not retired and the outcome is failed. The Pool's setup then reaches
  `b` and `c`, reports conflict for each under POOL-013, and needs replacements.

A further gap opens where every Resource's shutdown is confirmed but the Pool's setup is still
running, such as its setup hook. The Pool's own `retire.v1` then reports conflict, so "the
outcome of that retirement" does not exist. The request has dispatched retirements, so it
cannot end with a reason either (OPS-005).

**Least change:** In LIF-021, or in REV-014, say that `retire-with-resources.v1` is a request to
retire the Pool. It then reports conflict at once while a setup, a retirement or a change of the
Pool is in progress.

## 5-3-08

**Severity:** undetermined. **Title:** A repeat of an invocation that was reconciled as having
succeeded.

**Promises:** OPS-009, OPS-014, OPS-034 and IFC-025, and the contract of `reconcile.v1`.

**Quotes:**

- OPS-009, when: "the invocation that request began succeeded; and the engine still keeps its
  outputs."
- OPS-034, when: "the request of an invocation that succeeded and whose outputs the engine no
  longer keeps."
- IFC-025: "A result whose `isError` is not true has the outcome succeeded, and its
  `structuredContent` conforms to the `outputSchema` of IFC-033".

**Situation (scenario 4).**

1. An invocation of `mirror.sync.v1`, whose output `commit` is required, is unknown, because it
   was cancelled or was in progress at a restart.
2. An actor reconciles it as having occurred. `reconcile.v1` takes no outputs, so the engine never
   held any.
3. The caller repeats with the same key, to learn the result.

Neither promise covers the repeat: OPS-009 needs outputs that the engine "still keeps", and
OPS-034 needs outputs that it "no longer keeps". An engine may answer succeeded with an empty
`structuredContent`, which does not conform where an output is required, or report conflict with
the `ruleId` OPS-034. For an operation whose outputs are all optional, both answers conform, and
an engine may give either.

**Least change:** In OPS-009 and OPS-034, say that an invocation whose outcome a reconciliation
set has no outputs kept. A repeat of one reconciled as having succeeded is answered under
OPS-009 where the operation declares no outputs, and reports conflict under OPS-034 otherwise.

## 5-3-09

**Severity:** undetermined. **Title:** An Interface or a Pool whose template selects an item that
is run.

**Promises:** TPL-001, TPL-012, TPL-019, PRV-003, PRV-006, PRV-007, PRV-019, PLC-006, PLC-007 and
IFC-017.

**Quotes:**

- PRV-007: "in `machine`, the hostname and the labels of the machine that the instance's node
  represents."
- PRV-006: "an Interface, a Pool or a Conduit gives no `runtime`".
- PRV-003: "Asked to run an instance whose placement is a node other than `local`, it ends the run
  failed and runs nothing".

**Situation (scenarios 2 and 6).** TPL-001 lets an Interface or a Pool template hold an
`implementation` map, and the schemas `template-spec.v1` and `pool-template-spec.v1` accept one.
Its type may offer a capability, or require `setup-hook.v1`, served by a `command` item. Its
setup reaches the part that asks for the run (TPL-012), and a request to it is dispatched to that
item (IFC-017). Yet it has no node (PLC-006) and no `runtime`.

- Engine A runs the item on its own runner, on the machine the engine runs on. No promise keeps it
  from doing so: PLC-007 binds an Agent or a Resource, and the runner's refusal is for "a node
  other than `local`".
- Engine B ends the run failed, since there is no node and so no `machine`.
- Engine C rejects the template.

The outcome of the setup, and of every request to the instance, differs.

**Least change:** Reject at registration a template of an Interface or a Pool whose selected
implementations include an item that is run, as TPL-019 rejects a template that selects two.
Otherwise, say that the engine's own runner runs such an item on `local`.

## 5-3-10

**Severity:** undetermined. **Title:** A program given as a relative path escapes the check of
PRV-007 once SUB-010 has resolved it.

**Promises:** MAN-010, PRV-007 and SUB-010, and the contract of `run.v1`.

**Quotes:**

- MAN-010: "A program given as a relative path is taken from the directory the manifest was read
  from, which for a package is the root of its archive, and only the engine's own runner runs it,
  under PRV-007".
- SUB-010: "with the same result as a request for `register.v1` that is given the values read,
  once each relative `fetch` and each program given as a relative path is resolved against the
  directory."
- PRV-007: "It asks no runtime provider other than its own runner for the run of a `command` item
  whose program is a relative path".

**Situation (scenario 1).** A project directory lists `server: {command: [bin/server]}`. An Agent
selects it and names a runtime provider that runs items on another machine. SUB-010 registers
the values with the program resolved, such as `/home/u/proj/bin/server`. The item that the
template holds, and that `run.v1` sends, now names an absolute program, which the check of
PRV-007 does not catch.

- Engine A keeps only the resolved value, and asks the provider to run `/home/u/proj/bin/server`
  on the other machine. Where no such file exists there the run fails, and where one does,
  whatever lies there runs.
- Engine B remembers that the program was given as a relative path, and ends the part failed
  without asking.

The statement in MAN-010 holds of engine B alone.

**Least change:** In SUB-010 and PRV-007, say that a program given as a relative path keeps that
mark once it is resolved, so the check of PRV-007 still applies to it.

## 5-3-11

**Severity:** undetermined. **Title:** Two directory submissions have one digest and two sets of
programs.

**Promises:** SUB-008, SUB-010, NAM-009, PKG-005 and MAN-016.

**Quotes:**

- SUB-008: "Two submissions whose contract documents and manifest are equal under SCH-007 give the
  same digest, however their files are written and from whatever directory they are read".
- SUB-008: "The engine identifies the contracts and implementations of each submission by a
  digest."
- SUB-008: "each is told apart by its digest, which every template that resolved against it keeps
  under NAM-009, so a template and its instances reach only their own submission."

**Situation (scenarios 1 and 7).** The directories `/a` and `/b` hold the same manifest, with the
address A and `command: [bin/server]`, and the same contracts. Their programs at `bin/server`
differ, and so do the names of their templates. Both are registered: MAN-016 does not stop this,
and SUB-008 allows two submissions at one address. Both get the digest D, while SUB-010 resolves
their programs to `/a/bin/server` and `/b/bin/server`. A template from `/b` keeps (A, D).

- Engine A keys implementations by the digest, so the instances of `/b` run `/a/bin/server`, or
  the reverse.
- Engine B keeps the resolved items of each registration, so they run `/b/bin/server`.

The identifier that SUB-008 names cannot keep each template to its own submission.

**Least change:** In SUB-008, say that a template keeps, beside the digest, the implementations as
its own registration resolved them, so that two submissions with one digest stay apart.

## 5-3-12

**Severity:** undetermined. **Title:** Whether a retirement asks for the run when the retire hook
is served by a `remote` item.

**Promises:** TPL-009, TPL-018, PRV-007 and SEC-010, and the contract of `retire.v1`.

**Quotes:**

- TPL-018: "Where the hook is to be dispatched, an item that is run is selected for the instance
  and its runtime provider does not report the run running, the engine first asks for the run,
  under PRV-007."
- `retire.v1`: "asking first for its run where the run is not reported running, or connecting
  first to the `remote` item that serves the hook".

**Situation (scenario 2).** An Agent's operations are served by a `command` item, and its retire
hook by a `remote` item. Its run has stopped, as after a crash, or after a restart under DL16.

- TPL-018, read as written, has the retirement ask for the run, with reads under SEC-010, though
  the hook does not go to the run. Where a read fails, the run "cannot be had", and the hook is
  not dispatched.
- `retire.v1` asks for the run only for a hook that the run serves.

A call of `run.v1` and the reads in the records, and whether the hook is dispatched at all,
differ.

**Least change:** In TPL-018, ask for the run only where the item that is run implements
`retire-hook.v1`.

## 5-3-13

**Severity:** undetermined. **Title:** Whether the connection to a `remote` item is a part with an
effect.

**Promises:** OPS-022, TPL-012 and IFC-041.

**Quotes:**

- OPS-022: "failed if every part that was dispatched either failed, was answered with a reason in
  place of an outcome under IFC-026, or is of an operation whose contract states no `effect`, and
  is unknown if any part succeeded with an effect or ended unknown on an operation that has one."

**Situation (scenario 2).** An Agent's setup finds its declared Conduits open already, connects to
its `remote` item, whose `headers` carry a secret, and then its setup hook fails. The connection
is a part under TPL-012 and in the definitions, but it is not an operation and has no contract.

- Engine A counts it as a part without an effect, and the setup fails.
- Engine B counts the holding of the secret's value as an effect, so the setup ends unknown and is
  listed in `unreconciled`.

**Least change:** In OPS-022, say whether the connection to a `remote` item counts as a part with
an effect.

## 5-3-14

**Severity:** misleading. **Title:** The README says when a rotated secret reaches a `remote` item,
and leaves out the restart.

**Promises:** SEC-013, SEC-014, SEC-023 and PRV-026, and the section of `spec/README.md` on what
the guarantee for secrets covers.

**Quotes:**

- README: "A value rotated in its store reaches an instance's implementation only in a later run,
  which a further setup asks for once the run has ended, and a `remote` item only at the next
  setup (SEC-014, SEC-013)."

**Situation (scenario 10).** After the engine restarts, SEC-023 reads every secret of a connection
that lasts again before the connection's next HTTP request, with no setup. A value rotated
meanwhile reaches the `remote` item then. In the same way, PRV-026 runs a provider again after a
restart, with fresh reads: a later run that no setup asks for. A reader of the README would take
a rotation to wait for a setup.

**Least change:** Replace "only at the next setup" with "at the next setup, or after the engine
restarts, under SEC-023". Beside "which a further setup asks for", add the run that PRV-026 asks
for again.

## 5-3-15

**Severity:** minor. **Title:** No hook can deduplicate, yet two promises describe a hook of
unknown outcome dispatched as a new attempt.

**Promises:** OPS-021, OPS-028 and IMP-007, the contracts of `setup-hook.v1` and
`retire-hook.v1`, and the row "attempt" of the definitions.

**Quotes:**

- OPS-021: "One whose operation declares it, and whose implementation is not plain, is dispatched
  as a new attempt on the conditions of OPS-012".
- OPS-028: "An implementation whose operation declares `deduplicates: true` therefore recognizes
  the new attempt that OPS-021 dispatches".
- `core/operations.yaml`: "It is not declared where the engine dispatches other work, as it does
  for `set-up.v1`, `retire.v1` and `retire-with-resources.v1`, nor for an operation that the
  engine does not implement."

**Situation (scenario 9).** A hook's operation is always the core package's `setup-hook.v1` or
`retire-hook.v1`. Neither declares `deduplicates`, and no package can change that. So a hook of
unknown outcome, such as one in progress at a restart (IMP-007), is never dispatched again until
it is reconciled. The conditional text describes a case that cannot arise, and suggests that the
implementation of a hook can opt in to new attempts.

**Least change:** In OPS-021 and OPS-028, say that a hook of unknown outcome is not dispatched again
until it is reconciled, because neither hook's contract declares `deduplicates`.

## 5-3-16

**Severity:** minor. **Title:** The order of LIF-021 is not the reverse of POOL-013's.

**Promises:** LIF-021, POOL-013 and DL1.

**Quotes:**

- LIF-021: "The order is the reverse of the one POOL-013 gives a Pool's setup".
- POOL-013: "The order comes from the references among the entries alone".

**Situation (scenario 6).** The order of LIF-021 also follows `runtime` and the Conduits of which a
Resource is the consumer, and covers Resources that no entry made. POOL-013 orders entries alone,
and only by their references. Two entries that only a Conduit registered on its own joins are in
no order at setup, and in an order at retirement.

**Least change:** In LIF-021, say "an order that extends the reverse of the one POOL-013 gives".
