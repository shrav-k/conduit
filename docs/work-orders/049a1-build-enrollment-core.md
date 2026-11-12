# Work order 049a1 (E8a-1): enrollment in the core

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count. Run one cargo command at a time. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never read or print a
credential. No test may reach another machine or run a real machine provider: a case or a
scripted executor supplies the ending of every provider call.

## What and why

**When this order starts.** Your tree is the side branch `s3/m2`, not `main`. This order starts
only once order 038b ([its order](038b-spec-enrollment.md)) is merged to `main` and `s3/m2` has
been updated from `main`, and once the admin has re-made 038b's two staged patches (step 1). If
your tree has no `spec/promises/enrollment.yaml`, or no `docs/work-orders/038b-staged/049a.patch`
and `049b.patch`, stop and report, and change nothing.

**What it builds.** Order 038b wrote machine enrollment into the specification. A machine that
joins the machine provider is pending. It becomes a node only when root admits it, on evidence
that the provider reports, after the person types the fingerprint read on the machine and the
engine compares it ([ruling 68](../proposals/ruling-68/README.md), point 3, R2). The promises are
ENR-001 to ENR-012 in `spec/promises/enrollment.yaml` and PND-008 in `spec/promises/pending.yaml`;
038b also amended PLC-023, PLC-026 and PRV-013 to PRV-015. It staged, as exact patches, the
contract edits that the engine would reject until enrollment is built.

This order builds that enrollment in the core. It is the first half of batch 1's order 049a
([the plan](../proposals/spec-batch-1/README.md), lines 583-590), which the coordinator split:

- **049a1, this order:** the core's enrollment state, the three root verbs and their provider
  calls, machine reports, and admitted nodes;
- **049a2, after orders 047 (the pending-action core) and 048a:** PND-008's pending actions of
  kinds `enrollment`, `machine` and `machine-removal`, ENR-012's removal question, and their cases;
- **049b, after this order:** the fixture provider package and its contract.

**Why now, and where.** This is item 7 of the coordinator's plan for running Agents on the Windows
computer through Conduit once it is a node. The owner let the coordinator adopt that plan's
recommendations ([ruling 76, later answers](../proposals/ruling-76/README.md#later-answers)). Four
of the plan's choices are held for the owner: which machine provider enrolls the Windows computer;
which operating-system user the runs on it execute as; how TLS is terminated at the engine's
address; and whether that machine's Windows interop and drives are turned off. Nothing in this
order, and nothing you write, may name or assume any of them. The core assumes no machine
provider: its cases drive a fixture provider whose every call the case itself answers.

This order lands root's three enrollment verbs. They change first start's snapshot and the tools
root sees, both of which the S1 rerun measures, so the order is built on `s3/m2` and merges to
`main` with order 049b only after the rerun's pin. Orders [079](079-shell-https-address.md)
(ADDR, the engine's `https` address) and [080](080-shell-runtime-provider-calls.md) (DSP, the
shell's calls to runtime providers) also build on `s3/m2` and edit `engine/crates/shell/`. Order
080 and the coordinator's plan call this order 049a-1. Keep your shell changes small and local.
Size: L, about 1.0 to 1.3k lines and 17 new cases.

**Read first:**

1. Order 038b, and what it wrote: ENR-001 to ENR-012; PND-004, PND-008 and PND-012; IFC-049;
   PLC-007, PLC-008, PLC-023, PLC-024 and PLC-026; PRV-003, PRV-005, PRV-013, PRV-014, PRV-015 and
   PRV-019; REC-001; ACC-032; DL5 in `spec/implementation-defined.yaml`; and the contracts
   `enroll-machine.v1`, `admit-machine.v1`, `refuse-machine.v1`, `enroll.v1`, `accept-machine.v1`
   and `remove-machine.v1` in `spec/core/operations.yaml`. Also, unchanged by 038b: OPS-007 and
   OPS-013 in `spec/conventions/operations.yaml`, IMP-007 in `spec/conventions/implicit.yaml`,
   and `record.v1` in `spec/core/results.yaml`.
2. `docs/work-orders/038b-staged/`: `README.md`, `049a.md`, `049b.md`, both patches and
   `vector-impact.md`. This order's cases come from the table in `049a.md`.
3. In the plan: its orders 047 (lines 556-563), 049b (lines 592-595) and 051c (lines 642-645), and
   its conflict C11 (lines 310-313). In the
   [area draft](../proposals/spec-batch-1/areas/machines-and-trust.md): its Vectors and Build
   sections (lines 175-265).
4. Under `engine/crates/`:
   - `core/src/lib.rs`: `State` (lines 61-76) and first start (lines 646-809);
   - `core/src/requests.rs`: where engine verbs are routed (lines 1047-1053 and 1154-1160), the
     digest (line 780), the stored invocation (line 1056), the records policy for inputs and
     outputs (lines 366-371) and `recover` (lines 1486-1510);
   - `core/src/lifecycle.rs`: `Report` (lines 8-26), `emit` (lines 703-748), `report`, which
     takes a setup report (lines 786-848), and `observe_run` (lines 1122-1171);
   - `core/src/retirement.rs`: the retire hook (lines 246-262), the stop of a run (lines 270-282)
     and its report (from line 339);
   - `core/src/registration.rs`: placement (lines 1204-1272);
   - `core/src/access.rs`: ACC-032 (lines 477-481 and 577-587);
   - `shell/src/server.rs`: `EXECUTABLE_ENGINE_OPERATIONS` (lines 39-52);
   - `shell/src/lib.rs`: `Shell::setup_report` (lines 267-276), the inputs that get evidence
     (lines 338-350), `drive` (lines 559-611) and `missing` (lines 629-655);
   - `shell/src/runner.rs`: how `builtin_runner` reports a run it started later, through
     `Shell::setup_report` (lines 283-310).
5. If order 080 is in your tree: its step 3, the function that sends one provider call for any
   operation and returns what it read, and its `provider_call`.

Line numbers under `engine/` are those of `s3/m2` at 342c5ba, whose engine was `main`'s then. They
will have moved. This order starts only after `s3/m2` has been updated from `main`, and orders
still in flight to `main` edit these files: order 041b, for one, edits the core's `access.rs`,
`lib.rs`, `lifecycle.rs`, `registration.rs`, `requests.rs` and `retirement.rs`, and the shell's
`main.rs` and `server.rs`. Orders 079 and 080 edit the shell's `lib.rs`, `identity.rs`, `main.rs`
and `runner.rs`. So find every line under `engine/` by its words, and use the number only as a
hint. Order 038b changes nothing under `engine/`. Do not rely on line numbers in 038b's files;
find their text by promise ID.

`038b-staged/README.md` names three entries out of line with ENR-010 and IFC-049: PRV-027, LIF-036
and DL18. If your tree still has them so, follow ENR-010 and IFC-049, and report it.

Every decision you build cites its promise. Every reason a verb reports has a diagnostic that
names its deciding rule and its cause ([ruling 52](../proposals/ruling-52/README.md); ACC-056;
IFC-050).

## What to do

1. **Land the staged contract edits.** Before dispatch, the admin re-makes
   `docs/work-orders/038b-staged/049a.patch` and `049b.patch` against `main` without
   `docs/work-orders/036b-staged/048a.patch` (038b made them on top of it, and order 048a has not
   landed). From the repository root, run `git apply --check` and then `git apply` for
   `049a.patch`, then the same for `049b.patch`. If either check fails, stop and report. Do not
   edit a patch, apply part of one, or make one again. The two land together:
   - `049a.patch` adds `enroll-machine.v1`, `admit-machine.v1` and `refuse-machine.v1` to
     `object-management.v1`, to `management.v1`'s `requires`, to `root-management` and to the
     engine's `implements`, which TPL-004 requires to move together;
   - `049b.patch` adds `enroll.v1`, `accept-machine.v1` and `remove-machine.v1` to
     `machine-provider.v1` and to `CALLED_OPERATIONS` in `tools/check_schemas.py`; gives
     `list-machines.v1` its `id`, the state `pending`, `enrollment` and `evidence`, and `place.v1`
     its output `id`; and records reading 18's exception in `spec/core/README.md`.

   Keep both patch files: `IN_FORCE.md` links to them, and the admin updates that line and
   removes them at merge. Add one line to `038b-staged/README.md` saying that order 049a1 applied
   both patches. These are the only changes this order makes to `spec/` and `tools/`.

2. **The machine provider in force, and what the core keeps.** In a new
   `engine/crates/core/src/machines.rs`:
   - The machine provider in force is the Resource that is not retired, is owned by `system`, and
     whose type offers `machine-provider.v1` (PRV-013). The engine does not yet build PRV-013's
     check at registration: no file under `engine/` names PRV-013. Build its machine-provider
     half: a Resource whose type offers `machine-provider.v1` is not instantiated where its owner
     is not `system`, or where another machine provider is not retired; registration reports
     invalid under OBJ-023, with a diagnostic naming PRV-013, the Resource and any provider in the
     way. Leave the package-source half as it is, and say so. No existing vector registers a
     machine provider, so no vector changes.
   - New `State` fields, serialized only when not empty, as `record_gaps` and the others are:
     - **the enrollments.** Each holds what ENR-001 lists: its ID, the provider Resource's ID,
       `labels`, its expiry, `require`, `headless` and `acceptWeakerCheck`. It also holds the ID
       allocated for its pending action, its state (open, or closed admitted, refused or expired),
       the evidence kinds that `enroll.v1` declared, and the machines that joined by it. Only until
       a machine joins by it, it closes or it expires, it holds the join action, `message` and
       `carriesSecret` (ENR-003, PND-004). Once admitted, it holds the machine's id and whether the
       fingerprint was compared or the weaker check accepted (ENR-007).
     - **the latest report used** from the provider in force (step 7).
   - The core's own checks read these fields: admission, joins, refusal and expiry. Nothing
     discloses them but the panel view of step 9 and the tests: no tool result, inspection, list,
     export, record, diagnostic, decision or `_meta` (ENR-003, ENR-004, PND-008).

3. **`enroll-machine.v1` (ENR-001, ENR-002, ENR-003, IFC-049).** Route the three verbs to
   `machines.rs`, beside `records::handles` and `lifecycle::handles` in `requests.rs`. Root's
   request is decided in OPS-023's order:
   - any other actor is refused under ACC-032, by the existing check;
   - it reports invalid, naming ENR-001 and the input, for a reserved label key (`os`, `arch`,
     `kubernetes.io/hostname`, and every key under `kubernetes.io/` or `k8s.io/`); for an
     `expires` that is not later than the request's time, or is later than DL5's maximum after it;
     and for `acceptWeakerCheck` without `headless`. DL5's maximum is 24 hours, the area draft's
     suggestion (line 135); state it in `engine/CONFORMANCE.md` (step 15);
   - it reports unavailable, naming ENR-001, PRV-003 and the missing or unready provider, where no
     machine provider is in force or the one in force is not ready.

   Once dispatched:
   - It allocates the enrollment's ID and its pending action's ID. It emits one provider-call
     intent for `enroll.v1` to the provider in force, within the invocation, with `enrollment`,
     `labels`, `expires`, `require` and `headless`; an omitted `labels` or `require` is sent
     empty, and an omitted `headless` as false (ENR-001). Shape the intent as `emit` shapes a
     runtime provider's (`provider`, `config`, `revision`, `invocation`, `caller`), with the
     provider Resource as `subject`, and record it made (PRV-005, REC-001). The invocation stays
     pending.
   - The call ends succeeded with an output that validates against `enroll.v1`'s `outputSchema`,
     and its declared `evidence` kinds meet ENR-001's minimum: `fingerprint` for an enrollment
     that is not `headless`; `fingerprint` and `attestation` for `headless` without
     `acceptWeakerCheck`; one of the two for `headless` with it; and `attestation` too wherever
     `require` names it. Then the enrollment opens, with the earlier of the requested `expires` and
     the provider's as its expiry, and the invocation ends succeeded with exactly `enrollment` and
     `pendingAction`, the two IDs (ENR-001).
   - Where the declared kinds fall short, the invocation ends failed with a diagnostic naming
     ENR-001 and the missing kinds, both alternatives where neither `fingerprint` nor
     `attestation` is declared. No enrollment opens, and nothing of the action is kept.
   - A call that ends failed or unknown ends the invocation failed or unknown, and opens nothing.
     A call that ends succeeded with an output that does not validate against `enroll.v1`'s
     `outputSchema` opens nothing, and the invocation ends unknown: PRV-005 reads a provider
     call's result as OPS-007 reads one, and OPS-007 gives unknown where "a declared output fails
     validation in a result that would otherwise have the outcome succeeded". Its diagnostic
     names PRV-005 and OPS-007.
   - **No URL relay.** Every request, whatever its client declares, gets the two IDs at once, with
     no -32021 and no InputRequiredResult, and every action, whether `url`, `command` or `byOs`
     (an action for each operating system, ENR-002), is kept only for the panel. ENR-003 and
     IFC-049 allow the engine to relay a secret-free `url` and do not require it, so this order
     builds no relay.

4. **`admit-machine.v1` (ENR-006, ENR-007, ENR-010, ENR-011).**
   - Apply ENR-006's eight checks, against the latest report used and the enrollment, as ENR-006
     words them. Each failed check gives one diagnostic naming ENR-006, its check's number and its
     cause. A failed check 8, a missing typed fingerprint included, reports invalid, and every
     other failed check reports conflict; where several fail, OPS-023 picks one reason, invalid
     before conflict. Where every check holds but the provider is not ready, the request reports
     unavailable, naming that cause under ENR-006.
   - Check 8 compares character for character: no case folding, no trimming, and no assumption
     about the fingerprint's alphabet, which is the provider's (ENR-002). If check 7 fails,
     nothing is compared. On a `headless` enrollment with `acceptWeakerCheck`, checks 7 and 8 do
     not apply, and a supplied `fingerprint` is checked by the schema and otherwise ignored.
   - **The typed fingerprint** appears in no record, reply, diagnostic, decision, intent, retained
     output or `_meta`, and in no field of `State`. Remove it from the arguments that the stored
     invocation keeps (`requests.rs`, line 1056), so that the digest OPS-008 keeps, made over the
     whole request (line 780), is the only thing derived from it that the engine keeps (ENR-007).
     Repeats then follow OPS-009 and OPS-010 by that digest.
   - Once dispatched, it emits one provider-call intent for `accept-machine.v1`, with `id` and
     `enrollment`, as step 3 shapes it.
   - **Succeeded:** the engine registers a node from the core template `node`, as first start
     makes `local` (`lib.rs`, lines 713-798), with the same two `object-change` records:
     - owned by `system`, its own placement, in the lifecycle state `ready` at once, with a setup
       that dispatches nothing (PLC-008, PLC-026); its condition `Ready` is step 7's, `Unknown`
       until a report gives its machine a state;
     - named by the part of the hostname before its first dot, where that is valid under OBJ-001
       and unused by any existing Resource under OBJ-002, and otherwise under OBJ-003 (PLC-026);
     - its `status` holding `machine`, as `{provider, id, hostname}`; `enrollment`; and `labels`,
       fixed under ENR-010: the three descriptive labels as last reported, and the enrollment's
       labels (PLC-026, ENR-007, ENR-010).

     The enrollment closes admitted, with its record (step 6). The output `node` is the node as
     `object.v1`. No Conduit and no credential is made (ENR-011).
   - **Failed:** no node. The invocation ends failed, with a diagnostic naming the cause, and the
     enrollment stays open for another admission. **Unknown:** no node; the invocation ends
     unknown, and nothing is retried without a request (ENR-007, OPS-012).

5. **`refuse-machine.v1` (ENR-008), and the provider's retirement.**
   - Decided in OPS-023's order, each reason with its diagnostic: a named enrollment or machine
     that does not exist reports invalid under ENR-008 (a machine exists where the latest report
     used gives its id, or where it joined by the named enrollment); a machine that a node
     represents reports conflict naming the node, before any removal; a provider that is not in
     force, is retired or is not ready reports unavailable.
   - Once dispatched, a named open enrollment closes refused at once, with its record, whatever
     the removals then do. Then one `remove-machine.v1` intent, with its `id`, goes out for each
     named machine, and for each machine that joined by the named enrollment, that no node
     represents. No provider is asked to remove another provider's id.
   - The admin's reading, to apply and report: the invocation succeeds where every removal
     succeeded or none was needed. Otherwise it ends failed where any removal failed, and unknown
     where none failed, with one diagnostic for each removal that did not succeed, naming its
     machine and its cause. A failed or unknown removal is never reported as a removal. Nothing is
     retried, and nothing is removed, without a request (ENR-008, ENR-009).
   - When the machine provider in force is retired, each of its open enrollments closes refused,
     with its record, in the transition that marks the provider retired (`retirement.rs`, where an
     object's state becomes `retired`, line 194), and no provider is called (PRV-013, ENR-009).
     Every node but `local` then counts as omitted (step 7). A later provider's report never
     matches it, since a node is bound to its provider (PLC-026).

6. **Records and recovery.**
   - Add a seventh record type, `enrollment-change`. Its `subject` is the provider Resource; its
     `data` holds the enrollment's ID and its transition: `opened`, `admitted`, `refused` or
     `expired` (REC-001). An `opened` record also holds the enrollment's expiry, `require`,
     `headless`, `acceptWeakerCheck`, and the `carriesSecret` and evidence kinds that `enroll.v1`
     declared. An `admitted` record also holds the machine's id, the weaker-check flag, and
     whether the fingerprint was compared or the weaker check accepted (ENR-007). No
     `enrollment-change` record holds a join action, `message`, evidence or typed fingerprint
     (ENR-003, ENR-007). The evidence shown at admission is recorded with the pending action's
     answer under PND-012, which is order 049a2's, together with its withholding from export and
     inspection (PND-008).
   - The three provider calls' records follow `emit`'s: a `provider-call` record made, with the
     call's inputs where the provider's type keeps inputs under `records`, and one ended, with its
     outcome. The ended record of `enroll.v1` holds `outputs` only where that type keeps outputs
     under `records` (`record.v1` in `spec/core/results.yaml`; the policy in `requests::record`),
     and even then never `action` (ENR-003, PRV-005: "whatever the provider's records policy")
     or `message`. `message` comes with the action and may restate it, so the engine keeps it out
     whatever the policy; state that departure from `record.v1` under DL8 (step 15), and report
     it as a finding.
   - On `recover`, each invocation of the three verbs that is still in progress ends unknown, as
     `recover` ends all interrupted work (IMP-007, OPS-013; the core's `recover` cites these, with
     OPS-018 and DL2): no enrollment opens, no node is made and nothing is removed. An ending
     that arrives later for such an invocation is ignored, as `lifecycle::report` ignores a report
     for an invocation that has ended.

7. **Machine reports and nodes (ENR-004, ENR-010, PLC-023, PLC-026, PRV-014).** Add a core input
   `machine-report`, which carries evidence; `provider`, the provider Resource's ID; `outcome`,
   the call's outcome, `succeeded`, `failed` or `unknown`; and, where it succeeded, `outputs`,
   the call's result of `list-machines.v1`. No intent asks for it: order 051c adds the shell's
   loop that calls `list-machines.v1` and commits each ending in this shape.
   - **Which reports count.** The core uses a report only from the provider in force, only where
     it succeeded and its outputs validate against `list-machines.v1`'s `outputSchema` as
     `049b.patch` makes it, and only where it names no hostname and no id twice. A report that
     succeeded with outputs that do not validate is unknown (PRV-005, OPS-007, as in step 3).
     Any other report counts as one not received (PRV-014, DL5): it changes nothing but its own
     records.
   - **Its records.** REC-001 records a provider call when it "is made, or ends", and the
     profile's `provider_calls` says "made and ended". So each report adds two `provider-call`
     records, in the transition that commits it: one made, naming the provider as `target` and
     `subject` and the operation, with no `invocation` and no `caller`, since the engine makes
     this call of its own accord (PRV-005); and one ended, with its outcome and no outputs,
     whatever the provider's type states under `records`, so that no record holds a machine's
     evidence (PND-008). State that departure under DL8 (step 15). Records are kept in the
     store's one state file (`engine/CONFORMANCE.md`, DL8 under "Choices still required"). At a
     report every 10 seconds, the cadence the coordinator proposes for order 051c, that is about
     17,000 records a day: measure the bytes one report adds to the state, and report them.
   - **Pending machines.** A machine reported `pending` gets no node (ENR-004, PLC-026). Where its
     `enrollment` names an open, unexpired enrollment of this provider, it joins that enrollment,
     which drops its join action (ENR-003). Two machines attributed to one enrollment both join,
     and ENR-006's check 2 then fails for each. Any other pending machine is kept, unattributed,
     for the panel. Raising the actions of kinds `enrollment` and `machine` is order 049a2's
     (PND-008).
   - **Nodes.** A node other than `local` matches a reported machine only by its whole triple:
     the provider in force, the id and the hostname (PLC-026). A matching report sets the node's
     condition `Ready`, as `observe_run` keeps `Running`: `True` for `reachable`, `False` for
     `unreachable`. A node that the report omits, and one whose machine the report gives under
     another id or another hostname, counts as omitted: `Ready` is `Unknown`, with a reason and a
     message that name the cause, as Kubernetes does for a Node that stops reporting (PLC-023). A
     mismatch makes no node and rebinds none. A reported machine that no node represents and that
     is not pending makes no node.
   - **`Ready` from admission.** A report gives a node's machine a state only with `reachable`,
     `unreachable` or, until it is built, `gone`. `pending` is the state of a machine that no node
     represents (ENR-004, `list-machines.v1` as `049b.patch` makes it). So, as the admin reads
     PLC-023: a new node's `Ready` is `Unknown` from its admission, with a reason and a message
     saying that no report has given its machine a state since admission, until a report used
     after the admission gives the triple `reachable` or `unreachable`; and a later report that
     gives the node's whole triple with the state `pending` counts as omitted, with a reason and
     a message naming that state. PLC-023 does not say what a represented machine reported
     `pending` counts as: report that as a finding.
   - **Labels.** A report whose labels differ from a node's fixed labels changes none of them. A
     condition of its own type, which you name and list in `engine/CONFORMANCE.md`, names each key
     that differs, until a report matches again (ENR-010).
   - **`gone`** (PLC-024) is left to the work for scenario S3 in `ROADMAP.md` (E15: "a machine
     reported unreachable, then gone"), which has no order yet. Until it is built, a node whose
     machine is reported `gone` counts as unreachable, and nothing is retired;
     `engine/CONFORMANCE.md` says so.
   - **After a restart.** After `recover`, every node but `local` counts as omitted until the first
     report used after it (PLC-023). No report changes `local` (PLC-026).
   - **PLC-023.** A request, a setup hook or a retire hook that would be dispatched to the
     implementation of an instance placed on a node other than `local` whose `Ready` is not
     `True`, and a run or a stop that the engine would ask of that instance's runtime provider, is
     not dispatched or asked for, and reports unavailable naming PLC-023 and the node, with
     PLC-023's own exceptions: the read of a secret that PRV-026 makes, and the stop that a
     request to unregister asks for under LIF-036. Nothing is retired. Requests are held back in
     `requests.rs`, the setup's run in `lifecycle.rs`, and the retire hook and the stop in
     `retirement.rs`. No existing vector places an instance on a node other than `local`, so none
     changes.
   - **The machine's id reaches the runtime provider (PRV-019).** `emit` gives `run.v1`,
     `stop.v1` and `inspect-run.v1` the node's whole `status` as `machine` (`lifecycle.rs`, lines
     715-718). Build `machine` instead: `hostname` and `labels`, as today, and, for a node other
     than `local`, `id`, the machine's id from `status.machine`. The three contracts' `machine`
     schemas require `hostname` and `labels` and allow other members. For `local`, every intent
     must stay byte for byte as it is: if any existing vector's intent changes, stop and report.
     Run.v1's description says that `machine` is "the hostname and the labels" of the machine,
     while PRV-019 has the provider run the item on that machine and no other, and ENR-002 makes
     the id the machine's stable name within its provider. Report that gap as a finding, and do
     not change `spec/`.
   - **A node without `status.machine`.** A node other than `local` that has no `status.machine`
     can exist only in a state made by hand, as a test or a snapshot makes one. `emit` must not
     fail on it: it gives `hostname` and `labels` alone. It matches no report, so it counts as
     omitted. No vector and no shell test on `s3/m2` at 342c5ba holds such a node, and order 079
     tests on `local` alone. If a test of order 079 or 080 fails under your change, do not change
     that test: report its name and its failure.

8. **Expiry (ENR-009, ENR-003, PND-004).**
   - An enrollment whose expiry is at or before an input's time counts as expired for every check
     and every join, and in the panel view.
   - **Every input with a time closes it.** Before the core decides any input that carries
     evidence, it closes expired each open enrollment whose expiry is at or before the input's
     time, with its record, without a provider call and removing nothing, and drops its join
     action, `message` and `carriesSecret`. ENR-003 and PND-004 hold an action "only until" its
     join, answer or expiry, and this order's shell commits no `clock`: without this, an expired
     action, which may carry a secret, would stay in the store until a shell does. A machine that
     joins by a closed enrollment afterwards is kept, unattributed, for the panel (ENR-004,
     ENR-009).
   - **`clock`.** Add a core input `clock`, which carries evidence and does only that. Order 047
     builds a `clock` input for the expiry of every kind of pending action
     ([the plan](../proposals/spec-batch-1/README.md), line 561), and is on the rerun's path, so
     it may have reached `s3/m2` before this order starts. If your tree has a `clock` input,
     extend it with the enrollment's expiry and add no second one; otherwise build it in a form
     that 047's can extend.
   - Until a shell commits `clock` (order 051c adds it), an expired action is dropped only at the
     next input that has a time; `engine/CONFORMANCE.md` says so (step 15).

9. **The panel reads core state (ENR-003, ENR-004, PND-008).** Add a public, pure function in
   `machines.rs` that returns, from `State` and a time alone, what root's panel shows:
   - each open, unexpired enrollment: its ID, provider, labels, expiry and flags, and, while no
     machine has joined by it, its join action and `message`;
   - each pending machine of the latest report used: its hostname, id, labels and evidence, the
     enrollment it joined if any, and each of ENR-006's checks 1 to 7 that does not hold, by
     number and cause (ENR-004).

   Order 051c renders this view over a root-only channel that no MCP tool exposes. This order
   adds no command, page or tool.

10. **The shell.**
    - Add the three verbs to `EXECUTABLE_ENGINE_OPERATIONS` in `server.rs`. If order 072's test
      `every_engine_operation_is_executable_or_unfinished_with_its_order` is in your tree, it must
      pass (038b's `049a.md`).
    - **The endings.** Provider-call intents for `enroll.v1`, `accept-machine.v1` and
      `remove-machine.v1` go to the executor's `provider_call`, as a runtime provider's do. Each
      ending comes back as the existing core input `setup-report`, naming the intent's
      invocation, with one of three new report kinds in `lifecycle::Report`:
      - `{kind: enroll, outcome}`, with `outputs`, the call's result, only where `outcome` is
        `succeeded`;
      - `{kind: accept-machine, outcome}`;
      - `{kind: remove-machine, id, outcome}`, where `id` is the machine's id that the intent's
        inputs named, so that each removal of one refusal ends on its own.

      `outcome` is `succeeded`, `failed` or `unknown`. `lifecycle::report` hands a report for an
      invocation of one of the three verbs to `machines.rs` before its setup and retirement
      branches. `machines.rs` takes a report only for a call that the invocation still awaits;
      any other is ignored, as a report for an ended invocation is today. This reuses what
      exists: `drive` already turns a provider call's `Execution::SetupReport` into this input,
      `Shell::setup_report` already commits one that comes later, as order 080's calls do, the
      input already gets evidence, and the case schema already takes any `report` object. So add
      no input for these endings.
    - **If order 080 is in your tree,** its `provider_call` answers `Missing` for every operation
      but `run.v1`, `stop.v1` and `inspect-run.v1`, and it has a function that sends one provider
      call for any operation through the provider's own run, after `server/discover`, and returns
      what it read (its step 3). In `runner.rs`, make one narrow edit to its `provider_call`, in
      both `impl Executor for Runner` and `impl Executor for Arc<Inner>`: send the three machine
      operations through that function, deferred, and commit each ending through
      `Shell::setup_report`, as order 080 reports a run. Its reading becomes the outcome thus:
      succeeded, with its outputs, gives `succeeded`; failed gives `failed`; a call that the
      provider performed nothing for (the error -32021 of IFC-008, or a reason in place of an
      outcome), or that was not sent, gives `failed`; unknown, or no result, gives `unknown`
      (PRV-005, OPS-007). Change nothing else in `runner.rs`. If order 080's function is not
      there, change nothing in `runner.rs`.
    - **With no executor for them,** `missing` today returns an embedding error for these
      operations. Map each to the outcome `failed`, since nothing was sent, in the shape above, so
      that root's request ends failed with a diagnostic and the shell keeps serving.
    - Add `machine-report` and `clock` to the kinds that get evidence. The shell commits neither
      on its own: order 051c adds the report loop and the clock.

11. **The runner.** In `conformance/case.schema.json`, `engine/crates/conformance/` and
    `conformance/README.md`, or in the pieces and modules that order 071 made, if it has landed:
    - the inputs `machine-report` and `clock` (or, if your tree has 047's `clock`, nothing new for
      it); and, in `conformance/README.md`, the three report kinds of `setup-report` from step 10;
    - an expectation `panel`: a required-field projection of step 9's view, at the step's time;
    - an expectation that a given string occurs nowhere in the step's reply, decisions, intents
      and records, nor in the state's records, invocations and objects, nor in the exported
      records. By default it does not look in the fields of step 2, which the panel reads; a case
      may ask it to look in the whole state.
    - **A fixture machine provider**, the smallest the cases need: a fixture Type that offers
      `machine-provider.v1`, and a Resource of it owned by `system`, registered by the cases or
      held in a snapshot. Put it under `conformance/fixtures/machine-provider/`, with a README
      saying that order 049b completes it. No provider program runs: each case supplies every
      provider call's ending. Order 049b makes this fixture the fixture provider package, with the
      contract that ENR-002 asks for; do not write that contract here.

12. **The 17 cases**, under `conformance/conventions/` and with case IDs made from their paths, as
    the other cases have (`enrollment-enroll-opens`, `nodes-id-mismatch`). They come from the
    table in `038b-staged/049a.md`. Each is written without its PND-008 assertions: it asserts no
    pending action, no action's answer and no action listing, which order 049a2 adds. Each cites
    its promises and is listed in `conformance/manifest.yaml`'s `cases`. If order 071 has landed,
    `manifest.yaml` has no `cases`: list each case instead in the piece of its directory,
    `conformance/conventions/enrollment/index.yaml` or `conformance/conventions/nodes/index.yaml`,
    made as 071 makes pieces.

    | Case | What it shows | Promises |
    | --- | --- | --- |
    | `enrollment/enroll-own-provider` | With no machine provider in force, and with one that is not ready: unavailable, naming the missing or unready provider; no intent and no `enrollment-change` record | ENR-001, PRV-003 |
    | `enrollment/enroll-reserved` | Each verb, asked by an Interface other than root and by an Agent, is refused before any other check; root opening, for another consumer, a Conduit that lists `admit-machine.v1` is refused | ACC-032, ACC-056 |
    | `enrollment/enroll-opens` | One `enroll.v1` intent with the enrollment's ID and the filled inputs, and its made record; the completion's `command` holds a marker string; the invocation ends succeeded with exactly two distinct IDs; an `opened` record; the expiry is the earlier of the two; the marker is nowhere step 11's expectation looks; the panel shows the action | ENR-001, ENR-002, ENR-003, PRV-005, REC-001 |
    | `enrollment/enroll-invalid-inputs` | One step each: each kind of reserved label key, a past `expires`, one beyond DL5's maximum, and `acceptWeakerCheck` without `headless`; each invalid, naming ENR-001 and the input, with no intent | ENR-001 |
    | `enrollment/enroll-evidence-unmet` | Completions that declare only `attestation` for an enrollment that is not `headless`, only `fact`, and no `attestation` where `require` names it: each ends failed, naming ENR-001 and the missing kinds; nothing opens, and the panel shows nothing | ENR-001, ENR-005 |
    | `enrollment/join-attaches` | A report of a pending machine attributed to the open enrollment: no node; the panel shows its hostname, id, labels and evidence, and no longer the join action; a marker in its evidence is nowhere else; the same report again changes nothing | ENR-003, ENR-004, PLC-026 |
    | `enrollment/admit` | The matching typed fingerprint: one `accept-machine.v1` intent; on success, one ready node owned by `system`, its own placement, named from the hostname, with `status.machine`, `status.enrollment` and the fixed labels, and `Ready` `Unknown` with step 7's cause; the `admitted` record; the output `node`; no Conduit added; a following `reachable` report of its triple makes `Ready` `True`, and one that still gives it `pending` leaves it `Unknown`, naming that state | ENR-006, ENR-007, ENR-010, ENR-011, PLC-008, PLC-023, PLC-026 |
    | `enrollment/admit-fingerprint` | The reported fingerprint has 7 characters, so that a typed value with a trailing space stays within `admit-machine.v1`'s 6 to 8 and is decided by check 8, not by SCH-001. A missing typed fingerprint, and typed values that differ from the reported one only in case, by a trailing space, or in a `+` or a `/`: each invalid, naming check 8; none of those values is anywhere, the whole state included; the exact value, with mixed case, `+` and `/`, admits; on a `headless` enrollment with `acceptWeakerCheck`, a differing typed value is ignored and the machine is admitted | ENR-006, ENR-007, OPS-023 |
    | `enrollment/admit-not-admissible` | One step for each of checks 1 to 8, each with its diagnostic; zero and two reported `fingerprint` items, with a typed value supplied: conflict under check 7, and nothing compared; a missing typed value together with another failed check: invalid; no `accept-machine.v1` intent in any step | ENR-006, OPS-023, IFC-050 |
    | `enrollment/admit-accept-failed` | `accept-machine.v1` ends failed: no node; the invocation ends failed with its diagnostic; the enrollment stays open, and a second admission then succeeds | ENR-007 |
    | `enrollment/admit-accept-unknown` | It ends unknown: no node; the invocation ends unknown; no further intent | ENR-007, OPS-012 |
    | `enrollment/refuse-pending` | Refusing an enrollment with a joined machine: a `refused` record at once, and one `remove-machine.v1` intent; a removal that succeeds, one that fails and one that ends unknown, in separate enrollments, each reported as it ended; naming an unattributed pending machine's id gives its own intent; nothing is retried | ENR-008 |
    | `enrollment/expiry-before-join` | A `clock` past the expiry: an `expired` record, no intent, and the action gone from the panel and from the state; a later report attributed to it does not join; an admission reports conflict under check 2; for a second enrollment, a request from root past its expiry, with no `clock`, closes it the same way before it is decided (step 8) | ENR-003, ENR-004, ENR-009, PND-004 |
    | `enrollment/expiry-after-join` | A machine joins, then a `clock` past the expiry: an `expired` record, no intent and no removal; an admission reports conflict | ENR-009 |
    | `nodes/no-node-without-admission` | A report of a reachable machine and of a pending one, neither admitted: no node | ENR-004, PLC-026 |
    | `nodes/id-mismatch` | After an admission and a `reachable` report of the node's triple (`Ready` `True`), reports of the node's hostname under another id, and of its id under another hostname: `Ready` `Unknown` with the cause, and no new node; a request to, or the setup of, an instance placed on the node reports unavailable naming PLC-023; a matching report makes `Ready` `True` again | PLC-023, PLC-026 |
    | `nodes/labels-drift` | After an admission, a `reachable` report of the node's triple with a changed label and an extra one: the node's labels unchanged, a condition naming both keys, and the instance placed on it unchanged; a matching report clears the condition | ENR-010, PLC-007 |

13. **What the verbs change in existing vectors (the plan's C10 and C11).**
    - The ten expectations at `expect/state/objects/5/operations` that
      `038b-staged/vector-impact.md` lists, in eight cases, gain the three verbs where `049a.patch`
      puts them in `root-management`. `conformance/fixtures/root-system-state.json` gets the same
      list, and no other change.
    - If any other expectation changes, it can only be a list of `root-management`'s operations or
      of root's tools: change that list alone, and report it.
    - In `conformance/manifest.yaml`, add the new cases to the `s1_coverage` rows of PLC-026 and
      PRV-014, which `vector-impact.md` asks for, and to any other `s1_coverage` row whose promise
      a new case shows. `s1_coverage` stays in `manifest.yaml` whether or not order 071 has
      landed.
    - Write a declaration of these shapes under `docs/work-orders/049a1-declared.yaml`, as order
      045's is written, and run `python3 tools/vector_delta.py --declared
      docs/work-orders/049a1-declared.yaml HEAD`: no undeclared change. Never regenerate an
      expectation from the engine's output.

14. **Unit tests.** In the core, for what the 17 cases do not show and order 049a2's cases will:
    - `headless` with and without `acceptWeakerCheck`, at opening and at admission (ENR-001,
      ENR-006);
    - a repeat with the same typed fingerprint, and one with a changed one (OPS-009, OPS-010);
    - refusing an enrollment before any join, and a join by it afterwards (ENR-008);
    - refusing a machine that a node represents (ENR-008);
    - a second admission after the node's retirement, which makes a new node that may take the
      same hostname and revives no old one (PLC-026, LIF-040);
    - retiring the provider: its open enrollments close refused, every node but `local` is omitted,
      and nothing calls the retired provider (PRV-013);
    - PRV-013's check at registration;
    - `recover`: nodes omitted, the three verbs' invocations in progress ended unknown, and a
      later ending for one of them ignored;
    - a new node's `Ready` from admission, and a report that gives its triple `pending` (step 7);
    - a node other than `local` with no `status.machine`: `emit` gives `hostname` and `labels`
      alone, and the node counts as omitted;
    - an input other than `clock` whose time is past an open enrollment's expiry: the enrollment
      closes expired, with its record, and its join action, `message` and `carriesSecret` are gone
      from the state (step 8);
    - a `machine-report` that failed, one that succeeded with outputs that do not validate, and one
      that names a hostname twice: each changes nothing but its two records, the second recorded
      unknown;
    - the panel view of step 9.

    In the shell:
    - with no executor for the machine provider's calls, root's `enroll-machine.v1` ends failed
      with a diagnostic, not an embedding error;
    - if order 080 is in your tree, through the runner against its scripted provider: `enroll.v1`,
      `accept-machine.v1` and `remove-machine.v1` reach the provider after `server/discover`, with
      PRV-005's keys, and each ending of step 10's mapping, a provider that performed nothing
      included, ends root's request as step 3, 4 or 5 says;
    - with a scripted executor whose `enroll.v1` returns a `command` holding a marker: the call
      returns the two IDs; the marker is in no record, tool result or client output; the store
      holds it only while the enrollment is open and nothing has joined by it;
    - root's tool list holds the three verbs, and no other consumer gets them.

15. **`engine/CONFORMANCE.md` and the profile.**
    - **DL5:** the enrollment's maximum, 24 hours; reports supplied by `machine-report` and used as
      step 7 says; a new node's `Ready` from admission, and a represented machine reported
      `pending`; after `recover`, every node but `local` omitted until a report; expiry closed by
      `clock` or by any input with a time, and, until order 051c commits `clock`, an expired
      action kept until the next such input. Leave the report cadence under "Choices still
      required", for order 051c.
    - **DL8:** the seventh record type, `enrollment-change`, and what its `opened` and `admitted`
      records hold; a machine provider's call names the provider Resource as its `subject`; each
      report adds a made and an ended record, and the ended one holds no outputs, whatever the
      provider's `records`; the ended record of `enroll.v1` never holds `action` (PRV-005) or
      `message`. Say that the report's ended record and the `message` left out are the two places
      where a record holds less than `record.v1`'s records policy keeps. Update the vocabulary in `conformance/README.md`, and `record_types` and
      `provider_calls` in `conformance/profiles/first-build.yaml`.
    - **DL16, executor coverage:** the machine provider's three calls end through `setup-report`
      with step 10's report kinds; with order 080 in your tree, they are sent through its
      provider-call function and mapped as step 10 says; without it, each ends failed. Edit only
      what the row says of these three calls.
    - **A section for enrollment**, saying what this order built and naming as unfinished, each
      with its order:
      - order 049a2: PND-008's actions of kinds `enrollment`, `machine` and `machine-removal`,
        their raising, answers, listing and expiry under DL19; the answer records of PND-012 with
        the evidence shown, withheld from export and inspection; ENR-012's removal question; the
        action assertions of `join-attaches`, `refuse-pending` and the two expiry cases, and those
        that the rows of `038b-staged/049a.md` give `enroll-own-provider`, `enroll-opens`,
        `enroll-evidence-unmet` and `admit-accept-failed`; and the cases `enroll-headless`,
        `admit-attestation`, `admit-repeat`, `refuse-cancel`, `refuse-represented`,
        `nodes/readmit`, `nodes/provider-replaced`, `enrollment/action-visibility` and
        `nodes/retire-removal`, whose behavior this order builds with unit tests only;
      - `enrollment/url-retry`, which waits for the URL relay and for K1, the coordinator's MCP
        citation check ([the plan](../proposals/spec-batch-1/README.md), line 408); and
        `nodes/declare-lost-removal` and `nodes/gone`, with PLC-024's `gone`, which wait for the
        work of scenario S3 in `ROADMAP.md`, which has no order yet;
      - the URL relay, which IFC-049 makes optional and this order does not build;
      - order 051c: the panel's rendering, the machine commands and the report loop;
      - order 080, if it is not in your tree: the shell sending the provider calls;
      - order 049b: the fixture provider package and its contract.

16. **Before you finish, check:**
    - every request that is not dispatched names exactly one of refused, invalid, conflict and
      unavailable, in OPS-023's order, with a diagnostic naming its rule and its cause;
    - a marker in a join action, or in reported evidence, appears nowhere but in the fields of
      step 2;
    - a typed fingerprint that differs from the reported one appears nowhere at all, the whole
      state included;
    - no join action, `message` or `carriesSecret` stays in the state after the first input whose
      time is at or past its enrollment's expiry;
    - for `local`, every intent, status and existing vector is unchanged, apart from the lists of
      step 13;
    - no code, test, name or text assumes a particular machine provider.

## Scope

Change:

- `engine/`: the core (`machines.rs`, and the sites in `lib.rs`, `requests.rs`, `lifecycle.rs`,
  `retirement.rs`, `registration.rs` and `records.rs` that the steps name; report any other core
  file you change, and why), the conformance runner, the shell's `server.rs` and `lib.rs` and
  their tests, and `CONFORMANCE.md`;
- the shell's `runner.rs`, only for step 10's narrow edit of `provider_call`, and only if order
  080 is in your tree;
- `conformance/`: `case.schema.json`, `README.md`, the new cases under `conventions/enrollment/`
  and `conventions/nodes/`, the fixture under `fixtures/machine-provider/`, the expectations and
  the snapshot of step 13, `manifest.yaml` and `profiles/first-build.yaml`; if order 071 has
  landed, the pieces under `case-schema/` and the two `index.yaml` pieces of step 12 in place of
  the parts they hold;
- `spec/core/` and `tools/check_schemas.py`, only by applying the two patches of step 1;
- one line in `docs/work-orders/038b-staged/README.md`;
- the declaration `docs/work-orders/049a1-declared.yaml`;
- `experiments/011-short-form-spike/`, only if its tests need it: it depends on the core.

Do not change:

- `spec/` beyond the two patches: report any defect as a finding;
- the patch files themselves, or `IN_FORCE.md`: the admin updates its line on the patches at
  merge;
- `packages/`: no machine provider package is written here;
- the shell's `runner.rs` beyond step 10's edit, `identity.rs` and `main.rs`, or its listeners
  and addresses, which orders 079, 080 and 051c change;
- the tests of orders 079 and 080 (step 7);
- any CLI command or panel page (order 051c);
- pending actions, `answer.v1` or its stub (orders 047 and 049a2);
- a record: the rulings' records, the ledger, `docs/AUDIT.md`, `ROADMAP.md`,
  `docs/implementation/` and `docs/proposals/`.

Report anything else that needs changing, among it `IN_FORCE.md`'s line on the two patches.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py --declared docs/work-orders/049a1-declared.yaml HEAD`, with that
  interpreter: no undeclared change and no flip
- `python3 tools/check_private_paths.py`, if your tree has it (order 068 adds it)
- `git diff --check`

In `engine/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release -p conduit-core -p conduit-conformance` as often as you need, then the full
  `cargo test --release` once.

In `experiments/011-short-form-spike/`: `cargo test --release`.

Report the totals passed, failed and ignored of each full run. If a shell test fails, rerun it
alone; its timing-sensitive tests can fail under load (finding 4-18): rerun those with
`-- --test-threads=2`. Tests that bind loopback ports may fail in your sandbox: say so, name each,
and the admin reruns them.

Report:

- the cases added, and every existing expectation changed, by path and step;
- each reading you applied, among them refusal's outcome (step 5), `Ready` from admission
  (step 7), the name of the labels condition and the reasons of `Ready`;
- whether order 080, order 047's `clock` and order 071's pieces were in your tree, and what you
  did for each;
- the bytes that one machine report adds to the state (step 7);
- every finding against the specification, among them `machine`'s description in `run.v1`,
  `stop.v1` and `inspect-run.v1`; a represented machine reported `pending` under PLC-023; and
  `message` kept out of `enroll.v1`'s ended record;
- each check's last line.

Do not stage anything.

## Record

2026-11-12, verifier repair round. The admin supplied the following coordinator decision,
logged for the owner to strike, to settle overlapping acceptances (ENR-006, ENR-007,
PLC-026):

> One machine has at most one live node, as Kubernetes registers one Node per name.
> Acceptances of the same machine (the same ENR-002 machine id) are decided one at a time,
> in the order the engine begins them, as PKG-032 decides installs. An acceptance that finds
> a live node for that machine is refused with conflict, and its message names the existing
> node and the machine id (ruling 52). A repeat of the same accepted request answers as its
> first did (OPS-009) and publishes nothing new. A machine whose node was removed, or
> declared lost, can be accepted again.

The core applies this decision by provider Resource and machine id. It records provider
endings as they arrive, holds later successful acceptance endings until earlier acceptances settle,
and checks for a live node atomically before publication. Regression tests cover both
completion orders, one successful admission and one conflict, one live node, and the
successful request's repeat. The ENR promises need this serialization and conflict rule
written by the admin's next specification order; this repair changes no specification.
PLC-023's retirement gate now follows the work required: no prior setup needs no dispatch
(LIF-025, TPL-009); after setup, only a new hook or stop needs node availability.
