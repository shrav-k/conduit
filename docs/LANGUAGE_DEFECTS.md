# Language defect register

Opened 2026-11-05. A language defect is a place where the accepted text would let two
independent engine builders produce different observable behavior, or where following it
literally breaks another promise. Defects are found by model checking, by cold readers and,
later, by the engine builders. Each is fixed in the language itself.

This register feeds condition G7 of the
[readiness gate](implementation/INDEPENDENT_BUILDS.md#readiness-gate). The
[plan audit](AUDIT.md) tracks the surrounding work.

**What this register covers.** It covers the first sweep, which read the protocol topics under
[docs/protocol](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/README.md) before the [specification](../spec/README.md) existed. The
table of defects stops there. Findings against `spec/` are not added here: they are in the
[re-audit record](proposals/reaudit/README.md). Defect text keeps the words of the text it was
found in, such as handler, plugin, grant, provision, host and join; the terms in force are in
[spec/definitions.md](../spec/definitions.md).

**These defects were found against a stronger claim than the one now in force.** The readers were
asked where two engines could differ in any observable behavior. On 2026-11-05 the owner adopted
the [layered claim](AUDIT.md#layered-claim): the language determines what declarations mean, and
behavior beyond a few shared conventions belongs to type contracts and a runtime profile. Each
defect below was then re-triaged by layer, as the
[triage by layer](#triage-by-layer) describes. Only those assigned to the language
or the shared conventions are language defects in the strict sense; the rest are decisions for
a type contract or the runtime profile.

## Triage by layer

Done 2026-11-05 under layer decisions LD1 to LD5, applying the LD1 test in order. A first pass
by one reader; an assignment can be challenged like any other entry.

| Layer | Defects | What happens to them |
| --- | --- | --- |
| Language | 48 | Fixed in the language, with a promise and a vector. Counts for readiness condition G7. |
| Convention | 39 | Fixed in the shared conventions, with a promise and a vector. Counts for G7. |
| Type | 30 | Recorded against the type and decided when its contract is written. |
| Runtime | 50 | Decided for the reference engine, recorded and tested there. |

So 87 of the 167 are defects in the language in the strict sense. Since ruling 41, 13 of those
87 are closed and 74 are ruled. Of the 50 in the runtime profile, 5 are closed, 16 are ruled and
29 are answered by an item of the implementation-defined list. The 30 in the type layer are
open. The old assertion “None has a vector” is no longer supported: the current
manifest lists 367 cases and cites 100 distinct promises named by these statuses, counting each range of IDs in full.
It indexes promises, not L-IDs, so overlap does not establish closure of a defect scenario.
The retained “vector pending” statuses are historical; no row is re-triaged here.

The triage applied these readings of the layer decisions:

- A graph's static meaning and validity are Language; its scheduling, retry, cancellation and
  partial outputs are Runtime (LD2).
- How an operation is identified and how its outcome is classified are Convention, because a
  plugin and an engine written by different people must agree. The mechanism that keeps the
  promise, such as where an intent record is stored, is Runtime.
- Anything that decides whether access is allowed, or how long a grant lasts, is Convention,
  because it is a safety promise. When and how often a check is refreshed is Runtime.
- Which objects exist after a declaration is applied, and when a reference binds, are Language.
- Observation cursors and feeds are Runtime: the convention is only that every managed operation
  leaves a record that can be inspected.

The first sweep capped each reader at its 25 most divergence-prone findings, and every reader
judged that many more remain. This list is therefore a lower bound.

## Categories

| Code | Meaning |
| --- | --- |
| U | Undetermined: the text covers the situation but does not determine the outcome and does not declare it implementation-defined. |
| C | Contradiction: two accepted statements require different outcomes for the same situation. |
| X | Unsafe as written: following the rule literally breaks another promise. |
| D | Dangling: the behavior depends on a mechanism the text leaves open, so no outcome can be computed without inventing semantics. |

## Status values

| Status | Meaning |
| --- | --- |
| `open` | Not decided. |
| `fix proposed` | A rule is drafted and awaits an owner ruling. |
| `ruled` | The owner has decided, and the rule is written as the promises the row names. Its recorded status awaits a vector; current promise overlap is not scenario coverage. |
| `closed` | The language states the rule and a vector covers it; or the question no longer arises, and the row gives the reason. |
| `implementation-defined` | Deliberately left to each engine. The row names the item, `DLn`, of the [implementation-defined list](../spec/implementation-defined.yaml) that covers it. |

A promise identifier in a status, such as `OPS-012`, names a promise in [spec](../spec/README.md)
as it stands now. Where a promise first cited was later folded into another, or its rule moved to
another, the row cites the one that holds the rule now. The comment at the head of each file
under `spec/` says where each removed identifier went, and the
[specification's README](../spec/README.md#promise-files-that-rulings-removed) lists the
identifiers of each file that a ruling removed whole. Where an item of the implementation-defined
list no longer covers a row, as ruling 41 narrowed DL1, DL5 and DL9 and round 5 of the re-audit
removed DL11, the row cites the promise that decides it now. Where ruling 41 removed the feature
a row is about, the row is closed and names the item of the
[principles review](proposals/principles-review/README.md) that removed it.

## Defects

### Open type contracts and ruled runtime rows

| ID | Cat. | Layer | Area | Defect | Proposed fix | Status |
| --- | --- | --- | --- | --- | --- | --- |
| L3 | U | Runtime | Graphs | No closed set of step states and no rule deriving the graph outcome. A required step that is uncertain, never run, rejected at input validation or denied at dispatch has no defined state. | Add a step-state list and a table from step states to graph outcome. | ruled: OPS-022; vector pending |
| L5 | U/D | Runtime | Graphs | Retry cannot be computed. After `A` succeeded with limit 10, `B` failed and a control confirmed 15, a retry may keep 10 and reuse `A`, recapture 15 and rerun `A`, or refuse. | Define retry identity, recapture, the validity test for reusing a step and which steps are redispatched. | ruled: OPS-021, OPS-022, OPS-028; vector pending |
| L6 | C | Runtime | Graphs | "A graph runs each instance start" against "revalidate recorded setup progress before reusing it": a second start either redispatches every step or skips earlier successes. | State whether a start is a new execution or a resume. | ruled: OPS-021, OPS-022, OPS-028; vector pending |
| L8 | U | Runtime | Graphs | Whether "uncertain" is final within one execution, or later reconciliation can unblock dependents. | State it, and whether the graph stays non-terminal while a step is uncertain. | ruled: TPL-012, OPS-022; vector pending |
| L11 | U | Runtime | Graphs | Cancellation outcome and races: cancel after required work is done, before dispatch, or together with the last completion. | Define the cancelled state, the point at which cancel takes effect and whether a cancelled graph can be retried. | ruled: OPS-027; vector pending |
| L24 | U | Runtime | Graphs | A started runtime is "normally" retained after required setup fails; the exceptions are unstated. | Make retention unconditional or list the exceptions. | ruled: TPL-009, TPL-012; vector pending |
| L25 | D | Runtime | Graphs | Undefined terms that gate behavior: "ordinary steps", "mandatory stop rules", "required lifecycle cleanup". Whether the engine invokes any cleanup on cancel or failure is unstated. | Define the cleanup the engine performs, or state that it performs none. | ruled: TPL-009, TPL-012; vector pending |
| L67 | X/D | Runtime | Placement | A location declared lost that later reconnects with live instances and credentials. | Declare-lost issues a fencing epoch; a fenced location is refused and must stop its instances. | ruled: PLC-017, PLC-026; vector pending |
| L68 | U | Runtime | Placement | How "unreachable" is detected, who reports unknown, and what happens on rejoin. | Define the liveness rule and the rejoin exchange. | ruled: PRV-014 (the machines provider's contract), PLC-023; vector pending |
| L69 | U | Type | Agents | An Agent whose execution status is unknown is a dead end: resume is forbidden, delete is rejected, stop cannot be confirmed. | An owner-only "declare execution ended" operation with evidence, or a statement that it stays stuck. | open |
| L75 | U | Type | Operations | A provider rejection mid-call as evidence of drift: confirmed failure or unknown, and whether it marks the definition stale. | A protocol-level rejection is a confirmed failure and marks the definition stale; any other error on a write is unknown. | open |
| L76 | U | Type | Agents | The effect of pause or close on a running execution, and whether input to a paused or closed Agent is rejected or queued. | Publish a state-by-request transition table and say whether pause and close stop, drain or ignore the execution. | open |
| L77 | D | Type | Agents | Input scheduling across submitters: queue, reject when busy, or steer the current turn; what a cancel reaches. | Owner decision between queue, reject and steer, plus cancellation scope. | open |
| L78 | D | Type | Agents | "Stop" and "reconcile" are referenced but never defined. | Define stop-execution and reconcile-status, with the evidence needed for "confirmed ended". | open |
| L79 | U | Type | Agents | One active primary execution under two simultaneous resumes; whether "active" includes idle but alive. | Active means any execution not confirmed ended; resume is an atomic claim and the loser gets a named outcome. | open |
| L80 | C | Type | Agents | A required activation that fails after the Agent is ready (a refresh): unready, or ready with the prior load. "Readiness" itself is undefined. | Define what readiness gates; a failed refresh keeps the prior load and readiness; restart is never implicit. | open |
| L81 | C | Type | Agents | Fork and Fresh do not connect to the instantiation or placement rulings: what the new instance is built from and what it inherits. | Define Fork and Fresh as the standard instantiation shape plus a checkpoint argument, inheriting nothing. | open |
| L82 | U | Type | Resources | Purging a record that derived content depends on: readable, or withheld permanently because access to the source cannot be verified. | State what the deletion marker retains and that a purged source counts as a missing payload. | open |
| L83 | U | Type | Resources | Search caps, ordering and duplicates across several Resources. | Specify over-cap handling, de-duplication by record identity and either a deterministic order or an explicit backend-defined declaration. | open |
| L85 | U | Type | Resources | A second writer on a workspace without opting in to shared editing: rejected, given a second worktree, or advisory only. | State whether the default is enforced and by what check. | open |
| L86 | U | Type | Resources | What counts as "unique local work" that blocks worktree removal, and whether an override exists. | Enumerate the blocking conditions and state whether an owner override exists. | open |
| L87 | U | Type | Agents | Where checkpoints are stored, which decides whether an Agent can be moved off a dead location. | Name the owning store for checkpoints and tie the move to declare-lost. | open |
| L88 | U | Type | Agents | Two connections exposing a tool with the same name and no selection declared. | An unresolved conflict fails that activation action under its required or optional marking; no automatic renaming. | open |
| L89 | U | Type | Resources | Archive semantics: operations on an archived record, and archive racing an update. | Define archive as a revision-checked state change and list the operations permitted on archived records. | open |
| L90 | U | Type | Resources | Visibility and transitivity of withheld derived records: placeholder or omitted; direct inputs only or the transitive closure. | Specify the response shape and make the access check transitive. | open |
| L91 | D | Type | Resources | A share has no defined object, unit or end. | Define the share operation, its unit and which lifetime rules bind in v0. | open |
| L92 | U | Type | Resources | Local-directory edge states: restart after the directory was deleted, a crash between creation and the recorded outcome, a target that is a file. | Restart never recreates; give an explicit outcome for each case. | open |
| L93 | U | Type | Agents | When account identity is verified and what an unverifiable or changed identity causes. | State the verification points and the mandatory result. | open |
| L94 | U | Type | Agents | Which checkpoint a resume loads, and whether a failed capture fails the pause. | Define "applicable"; the pause completes with a recorded gap. | open |
| L95 | C | Type | Agents | Execution preference fallback and "launch overrides", which have no mechanism under the instantiation rulings. | Define the fallback and remove or redefine overrides. | open |
| L96 | U | Type | Resources | Overlapping backing paths between Resources, and cleanup of a directory Conduit created. | Define use by backing path and the cleanup rule for created directories. | open |
| L102 | U | Runtime | Controls | When the one-at-a-time slot is released: after the handler call, when a deferred operation ends, or when uncertainty is reconciled, which may be never. | Define when application ends (owner decision) and whether unresolved uncertainty blocks later controls. | ruled: REV-012, REV-014; vector pending |
| L103 | U | Runtime | Controls | Busy-target default, queue order and the fate of queued requests at retirement are "plugin-defined". | Default to reject with a busy outcome; a declared queue is first-in first-out and retirement fails queued entries. | ruled: REV-014; vector pending |
| L104 | D | Runtime | Controls | Parallel-safe controls and the revision check: checked at start or at confirmation. | Check at application start under the target lock; each confirmation advances the revision. | ruled: REV-002, REV-012; vector pending |
| L106 | U | Runtime | Controls | A partial confirmation that cannot produce valid stored configuration leaves a known-false value stored; revision, admission and export in that state are unspecified. | Define an out-of-contract instance state and its effect on revision, admission and export. | ruled: REV-007, REV-015; vector pending |
| L128 | C | Type | Authority | A caller "does not need direct access to every internal backing service" against derived records requiring read access to all declared sources. | State which rule governs index-derived results. | open |
| L130 | D | Type | Authority | Classifying provider tools discovered later against a grant for "read operations". | Bundles expand to named operations at issuance; an unlisted tool is refused. | open |
| L139 | U | Type | Authority | Mixed-access content "withheld as a whole": the unit, what the caller sees, and whether limits apply before or after withholding. | The unit is the record; state the notice form and the order relative to limits. | open |
| L151 | U | Type | Communication | A participant's initial read position, the catch-up bound, what is returned beyond it, and whether the history has one total order under concurrent posts. | State the initial position, the bound, over-bound behavior and a single total order assigned at acceptance. | open |
| L152 | U | Type | Communication | What advances a read position: retrieval or explicit acknowledgement; whether the position skips or blocks at a message withheld by an access check. | Define the advance trigger and the withheld-message rule. | open |
| L153 | U | Type | Communication | Whether a read position is keyed by Agent and history or by participation, so that it survives a leave and rejoin. | Name the key and the reset rules. | open |
| L154 | U | Type | Communication | Notice semantics: one coalesced notice or one per post, repeated until read or once, whether own posts count. | Define the notice as a function of unread count at turn start, excluding own posts, with a repetition rule. | open |
| L155 | U | Runtime | Conduits | What an "unprepared but partly connected" consumer may do: run, use the attached connection, be used by others. | Define the operations permitted in the unprepared state. | ruled: ACC-003, ACC-036 (DL11 removed by round 5 of the re-audit, M69); vector pending |
| L156 | U | Runtime | Conduits | What attaches an optional connection that failed at preparation once its target becomes ready, given that no background reconciler exists. | State the trigger. | ruled: CON-004, ACC-006; vector pending |
| L158 | U | Runtime | Retirement | Bulk Pool retirement when one Resource's shutdown is uncertain: order, continue or stop, and what the Pool and Resources admit meanwhile. | Define the order (consumers first), continue-on-failure and admission states. | ruled: LIF-021; vector pending |
| L166 | U | Type | Communication | Purging a message in an append-only history: marker or nothing, and whether positions shift. | Positions are stable sequence numbers and a purge leaves a marker. | open |

### Crosswalk of the other decided rows

Statuses and references are copied from the historical rows. Each scenario is the first
sentence of its original defect; “vector pending” remains a recorded status, not a new
coverage assessment. Closed rows number 18 (13 language/convention and 5 runtime), not 23.

| ID | Decision and recorded status | Promise or DL item | Scenario |
| --- | --- | --- | --- |
| L1 | ruled: OPS-012 to OPS-014; vector pending | OPS-012 to OPS-014 | The rule "After an uncertain write, inspect local/provider state before reissuing" is unsafe against a provider that does not deduplicate by operation identity: the original request can arrive after the provider reports that it has not. |
| L2 | ruled: TPL-012, POOL-013; vector pending | TPL-012, POOL-013 | Whether a data dependency needs the producing step to succeed, or only its output to be published. |
| L4 | closed: removed by ruling 41 (principles review item 1) | ruling 41, principles review item 1 | When graph completion is reported while optional work is still running. |
| L7 | closed: removed by ruling 41 (principles review item 1) | ruling 41, principles review item 1 | Fate of outputs published before a later required step fails: kept or withdrawn, and whether a failed graph-backed operation's result includes them. |
| L9 | ruled: OPS-007; vector pending | OPS-007 | Step outcome when one declared output fails validation: failed, uncertain or succeeded; whether valid sibling outputs still publish. |
| L10 | ruled: OPS-001 to OPS-007; vector pending | OPS-001 to OPS-007 | How an operation result is classified as success, failure or uncertain is unspecified, yet prerequisites depend on it. |
| L12 | closed: removed by ruling 41 (principles review item 1) | ruling 41, principles review item 1 | A binding error on an optional public output: reported without preventing success, or fails the graph. |
| L13 | closed: removed by ruling 41 (principles review item 1) | ruling 41, principles review item 1 | Which wiring and type errors must be caught early, and at which phase: registration, execution start or the consuming step. |
| L14 | closed: removed by ruling 41 (principles review item 1) | ruling 41, principles review item 1 | Continue-after-failure is stated for setup and change graphs only; graph-backed operations are unstated. |
| L15 | closed: removed by ruling 41 (principles review item 1) | ruling 41, principles review item 1 | "Continued validity" of a published output at the moment a later step consumes it: who checks, when, and the consumer's resulting state. |
| L16 | ruled: SCH-005, SCH-006; vector pending | SCH-005, SCH-006 | Integer and fraction boundary: `15.0` or `1e1` for an integer field; a large integer under `type: number` or in an untyped literal. |
| L17 | ruled: SCH-002, SCH-007; vector pending | SCH-002, SCH-007 | Numeric comparison: `multipleOf: 0.1` with 0.3; `const: 1` against 1.0; republishing 1 then 1.0 under one attempt. |
| L18 | ruled: SCH-008 to SCH-010; vector pending | SCH-008 to SCH-010 | Which schema locations contribute defaults: identical defaults under `allOf`, defaults inside `items`, through `$ref`, `patternProperties`, or inside a materialized parent default. |
| L19 | ruled: SCH-002, SCH-008; vector pending | SCH-002, SCH-008 | Which keywords count as conditional for the rejected branch-dependent defaults (`anyOf`, `dependentSchemas`, `not`, single-branch `oneOf`). |
| L20 | ruled: SCH-002; vector pending | SCH-002 | The recognized schema-keyword set is not published (`$comment`, `deprecated`, `readOnly`, `unevaluatedProperties`, `$dynamicRef`, `contentEncoding`, another `$schema`). |
| L21 | ruled: SCH-003, SCH-004; vector pending | SCH-003, SCH-004 | Format strictness and unknown formats: validator libraries disagree on edge cases; an unknown format is rejected at schema acceptance or at validation. |
| L22 | closed: removed by ruling 41 (principles review item 1) | ruling 41, principles review item 1 | Whether a template-local, unpublished graph-backed operation may reference captured configuration. |
| L23 | closed: removed by ruling 41 (principles review items 1 and 2) | ruling 41 | Whether a pointer applies to runtime-target and metadata references, and what unavailable optional metadata yields. |
| L26 | closed: removed by ruling 41 (principles review item 1) | ruling 41, principles review item 1 | A prerequisite listed twice, or listed for a step that is also data-bound: rejected or accepted. |
| L27 | ruled: NAM-001 to NAM-017; vector pending | NAM-001 to NAM-017 | Whether installing a package registers its exported templates, and how a registry reference reaches a package export. |
| L28 | ruled: PKG-005; vector pending | PKG-005 | When an implementation binding is resolved and captured: at template registration, instance creation or first start. |
| L29 | ruled: FST-001 to FST-003, FST-005, OBJ-002; vector pending | FST-001 to FST-003, FST-005, OBJ-002 | Foundational inventory, order and placement. |
| L30 | ruled: PLC-007; vector pending | PLC-007 | Where a package must be installed (management or each location), and where a plugin declares its own needs of a location. |
| L31 | ruled: ACC-022, ACC-023; vector pending | ACC-022, ACC-023 | Authority of a second Interface: "Interfaces are unrestricted" against "future access policy is deferred" and the recorded-grant rule. |
| L32 | implementation-defined: DL6 | DL6 | What triggers first initialization and what a repeated or half-finished initialization does; where resume progress lives before persistence exists. |
| L33 | ruled: ACC-024; vector pending | ACC-024 | Revoking the root's management grant: refused, allowed and unrecoverable, or re-issued from owner verification, which is a standing bypass. |
| L34 | implementation-defined: DL6 | DL6 | "Verified local-owner context" is undefined, so any process under the owner's OS account, including an Agent, may count as an owner frontend. |
| L35 | ruled: NAM-006, PKG-003; vector pending | NAM-006, PKG-003 | Resolving a dependency alias when several releases and an edit-mode copy of one package are installed; the scope of a "configured default". |
| L36 | implementation-defined: DL6 | DL6 | "Missing or inconsistent" foundational components on a later start: what is checked and which operations remain available. |
| L37 | ruled: TPL-003, TPL-004, CTR-004; vector pending | TPL-003, TPL-004, CTR-004 | Type-default handlers against the single dependency map: a default handler from a package absent from the template's map, or two aliases contributing the same control. |
| L38 | ruled: PKG-006, PKG-007, NAM-011; vector pending | PKG-006, PKG-007, NAM-011 | Where an unresolved dependency is accepted or rejected, and whether an existing instance late-binds when the package is installed later. |
| L39 | implementation-defined: DL7 | DL7 | Edit mode against the rule that conflicting definitions of one contract identity and version cannot coexist. |
| L40 | implementation-defined: DL6 | DL6 | Where code for Pool, Conduit and Interface instances runs, given that they carry no placement but every type must state its location needs. |
| L41 | implementation-defined: DL6 | DL6 | Whether repair uses the root definition bundled with the current runtime or the one the root instance retained, after a runtime upgrade. |
| L42 | implementation-defined: DL6 | DL6 | Whether each frontend attachment is its own participation on the management Conduit, and whether the Conduit ends when the last frontend exits. |
| L43 | implementation-defined: DL7 | DL7 | Removing a package release that a stopped instance is bound to, and whether reinstalling the same digest restores it. |
| L44 | closed: removed by ruling 41 (principles review item 9) | ruling 41, principles review item 9 | Re-export chains and cycles, and which digest a binding through a re-export records. |
| L45 | ruled: PKG-001, PKG-003; vector pending | PKG-001, PKG-003 | What "exact version" equality means (`1.0` against `1.0.0`; an unversioned contract). |
| L46 | ruled: conformance section of the specification; vector pending | — | What a conformance claim must list and where unsupported semantics are reported. |
| L47 | ruled: PKG-016; vector pending | PKG-016 | Whether plugin state is isolated per instance or shared when two instances use one plugin. |
| L48 | ruled: PKG-004; vector pending | PKG-004 | The content digest is not deterministic across engines: algorithm, canonical form, file modes and symlinks are unspecified. |
| L49 | ruled: INS-001 to INS-014, INS-016, INS-017, OBJ-013; vector pending | INS-001 to INS-014, INS-016, INS-017, OBJ-013 | Protocol text still describes creation as editing ordinary fields of a draft, against the instantiation record's closed inputs; direct creation of a complete instance is unbounded. |
| L50 | ruled: ACC-033; vector pending | ACC-033 | Which target authorizes Resource creation: the owning Pool, the management Resource, or both. |
| L51 | ruled: OPS-008; vector pending | OPS-008 | Who mints an operation identity, its scope, and its relation to the "operation reference". |
| L52 | ruled: ACC-002; vector pending | ACC-002 | The enforcement point for a call crossing two locations is unstated. |
| L53 | implementation-defined: DL4 | DL4 | How "current permission cannot be established" is detected: a live query per dispatch, or a local copy treated as available authority. |
| L54 | implementation-defined: DL2 | DL2 | The boundary between "new" and "already dispatched" for a request that was accepted and checked but is queued when management drops. |
| L55 | implementation-defined: DL4 | DL4 | What "hold" means: fail fast or block and dispatch on reconnect. |
| L56 | ruled: OPS-009, OPS-018; vector pending | OPS-009, OPS-018 | No intent record is required before an effect. |
| L57 | ruled: OPS-009; vector pending | OPS-009 | A retry that arrives while the original is still in flight or unknown: block, report in progress, reject or dispatch. |
| L58 | ruled: OPS-010; vector pending | OPS-010 | The same operation identity reused with different arguments, caller or target. |
| L59 | implementation-defined: DL2 | DL2 | Attempt identity and late results: which result stands when an earlier attempt's success arrives after a later attempt was issued. |
| L60 | ruled: ACC-034, OPS-009, OPS-032; vector pending | ACC-034, OPS-009, OPS-032 | Whose access governs delivery of a retained result, and what a withheld result looks like. |
| L61 | ruled: OPS-011; vector pending | OPS-011 | A plain MCP client cannot reuse an operation identity: after a lost response it calls again under a new request identity. |
| L62 | implementation-defined: DL2 | DL2 | Which service holds the record of an operation between instances on different locations. |
| L63 | implementation-defined: DL4 | DL4 | How a withdrawal is ordered against a permission answer already given ("new rules immediately"). |
| L64 | ruled: OPS-014; vector pending | OPS-014 | How an operation leaves the unknown outcome: no operation records a reconciliation. |
| L65 | implementation-defined: DL4 | DL4 | A graph in progress when management drops: resumes on reconnect or ends and needs an explicit retry; whether timeouts run meanwhile. |
| L66 | implementation-defined: DL4 | DL4 | Whether an explicit stop is admitted on a system cut off from management, or held like any new managed request. |
| L70 | implementation-defined: DL8 | DL8 | Committing an outcome and appending it to the observation feed are not atomic; after a crash an observer misses the change with no gap report. |
| L71 | implementation-defined: DL8 | DL8 | Snapshot position semantics: inclusive or exclusive, how duplicates are detected, whether uncommitted changes appear. |
| L72 | implementation-defined: DL8 | DL8 | A cursor that precedes purged entries, narrowed access or a restored store. |
| L73 | implementation-defined: DL8 | DL8 | Whether a cursor is held by the client or the source; two frontends resuming from one cursor. |
| L74 | implementation-defined: DL3 | DL3 | An interaction wait whose challenge and deadline both expire, that two frontends answer, or that is answered during an outage. |
| L84 | ruled: INS-018; vector pending | INS-018 | Resolving a relative path against a base at creation conflicts with pure instantiation and with the rule that path joins belong in a plugin operation. |
| L97 | implementation-defined: DL10 | DL10 | Retention until explicit purge when storage is exhausted. |
| L98 | ruled: REV-007, REV-015; vector pending | REV-007, REV-015 | How a control's result becomes stored configuration: the requested value, the value the handler reports, or nothing, since no shared "confirmed settings" shape exists. |
| L99 | ruled: REV-006; vector pending | REV-006 | An omitted field in a control input: left unchanged, or reset because operation-input defaults are materialized. |
| L100 | ruled: REV-003 to REV-005, REV-009; vector pending | REV-003 to REV-005, REV-009 | Configuration revision: per instance or per field, and whether a failed or uncertain control advances it. |
| L101 | ruled: REV-001; vector pending | REV-001 | Whether the instance's "configuration revision" and the registry's "configuration version" are one value, and which authority checks delete and rename. |
| L105 | ruled: INS-008 to INS-011; vector pending | INS-008 to INS-011 | Destination semantics: an unsupplied optional input over a template value, a destination whose parent is absent, two inputs with overlapping destinations, a supplied null. |
| L107 | ruled: OBJ-008, OBJ-009; vector pending | OBJ-008, OBJ-009 | When a reference resolves (registration or instantiation) and when a wrong-form target is rejected. |
| L108 | ruled: OBJ-008 to OBJ-010; vector pending | OBJ-008 to OBJ-010 | Whether a full template replacement re-resolves references whose authored text is unchanged. |
| L109 | closed: removed by ruling 41 (principles review item 10) | ruling 41, principles review item 10 | Instance rename "uses the normal checked configuration update", which elsewhere does not apply to instances. |
| L110 | ruled: REV-005; vector pending | REV-005 | Whether a version can recur (A, B, A), letting a stale update pass. |
| L111 | ruled: SCH-007; vector pending | SCH-007 | Equality normalization decides whether a resubmission is a change (`30.0` over `30`, a reordered list). |
| L112 | ruled: SUB-003, SUB-004; vector pending | SUB-003, SUB-004 | Whether a duplicate target rejects the whole batch or only the duplicated entries. |
| L113 | ruled: SUB-004; vector pending | SUB-004 | Unordered batch entries that conflict by resulting name; the winner depends on scheduling. |
| L114 | implementation-defined: DL10 | DL10 | Retrying a batch whose earlier attempt completed some creates. |
| L115 | ruled: SUB-005; vector pending | SUB-005 | What a batch entry may be: a template, an instantiation request, a direct instance definition. |
| L116 | implementation-defined: DL9 | DL9 | The order in which checks are reported when several fail (stale revision, bad value, busy target). |
| L117 | ruled: OBJ-004; vector pending | OBJ-004 | System-owned metadata present, and correct, in submitted YAML: rejected or ignored. |
| L118 | ruled: OBJ-001, OBJ-003; vector pending | OBJ-001, OBJ-003 | Name comparison (case) and what happens when a generated name collides. |
| L119 | ruled: every instance comes from a template (ruling 18) | ruling 18 | Direct creation of a complete instance definition that carries its own graph and controls with no template. |
| L120 | ruled: ACC-022 to ACC-024; vector pending | ACC-022 to ACC-024 | Interfaces are "unrestricted", yet the root needs a recorded grant that any grant administrator may revoke; a second Interface has no stated authority. |
| L121 | ruled: LIF-008; vector pending | LIF-008 | A revoked grant is silently re-established when the Agent's declared connection is set up again on restart or import. |
| L122 | ruled: ACC-003; vector pending | ACC-003 | The extent of a grant established at connection: exactly the requested access, or the establisher's maximum narrowed by filters. |
| L123 | ruled: ACC-006; vector pending | ACC-006 | Telling an "explicit broader request" from a caller-added restriction without a subset check. |
| L124 | implementation-defined: DL4 | DL4 | Cannot-verify: hold or fail, for how long, and how it ranks against a known missing grant. |
| L125 | implementation-defined: DL4 | DL4 | A rule change between admission, dispatch and delivery: whether the caller still learns the outcome and identity of its own dispatched write. |
| L126 | implementation-defined: DL4 | DL4 | Routing: whether an automatic resolution is sticky when a second route appears, what a configured choice is keyed on, and what happens when the chosen route is revoked. |
| L127 | implementation-defined: DL4 | DL4 | "Eligible route" is undefined: eligible for this operation, or for the target at all. |
| L129 | ruled: ACC-009, ACC-017; vector pending | ACC-009, ACC-017 | Which of a caller's restrictions travel across a Resource-to-Resource hop when they exist only as a filter on a different path. |
| L131 | closed: removed by ruling 41 (principles review item 3a) | ruling 41, principles review item 3a | A Resource that newly matches a governing binding but whose plugin does not support the requirement: exempt, unavailable, or provisioning refused. |
| L132 | ruled: ACC-014; vector pending | ACC-014 | An operation a requirement cannot evaluate, within its declared applicability. |
| L133 | ruled: CON-022; vector pending | CON-022 | A filter the endpoint does not support, at establishment: refuse, establish with everything unavailable, or establish partly. |
| L134 | ruled: ACC-006; vector pending | ACC-006 | The authority needed to delegate: holding the access, holding an administrative grant, or both. |
| L135 | ruled: ACC-011, ACC-012; vector pending | ACC-011, ACC-012 | Revoking or changing a grant wider than the administrator's own extent; whether partial revocation exists. |
| L136 | closed: removed by ruling 41 (principles review item 3a) | ruling 41, principles review item 3a | Issuing a grant that exceeds the recipient Agent's limits: refused, or issued but ineffective until the limit lifts. |
| L137 | ruled: ACC-019; vector pending | ACC-019 | Whether deferred or scheduled work created by a caller stays subject to that caller's access when it later runs. |
| L138 | ruled: ACC-016 to ACC-018; vector pending | ACC-016 to ACC-018 | An accepted ruling that composed operations follow the dependent-call rule is recorded only in the proposals folder; whose grants each graph node uses is unstated in the protocol. |
| L140 | closed: removed by ruling 41 (principles review item 5) | ruling 41, principles review item 5 | Whether a composite Conduit carries one grant or one per component, and a composite filter naming an operation a component lacks. |
| L141 | implementation-defined: DL4 | DL4 | Replacing the service identity behind a recorded route: continues as the same path, or needs a new explicit choice. |
| L142 | ruled: REC-001, OPS-005; vector pending | REC-001, OPS-005 | Whether refusals and cannot-verify results are recorded, with what fields, and what reason the caller sees. |
| L143 | ruled: CON-007, CON-008; vector pending | CON-007, CON-008 | What ends logical participation: whether a crash, stop, pause, close or delete of the consumer does, or only an explicit detach. |
| L144 | ruled: CON-002, CON-004; vector pending | CON-002, CON-004 | Preparing a consumer again with an `open` declaration: reuse the existing Conduit or open a second with a new identity and no grants. |
| L145 | closed: a Conduit is pairwise, so there is no join (ruling 17) | ruling 17 | Joining an instance that has ended: "later participation opens a new instance" against "join requires a flow that has not ended". |
| L146 | closed: a Conduit is pairwise, so there is no join (ruling 17) | ruling 17 | Whether a Conduit with no consumer is live, and how two Agents from one template come to share one conversation rather than opening two. |
| L147 | ruled: LIF-007, LIF-008; vector pending | LIF-007, LIF-008 | The registry state of an ended Conduit: removed, or retained as ended so that its name stays taken. |
| L148 | ruled: ACC-005, LIF-008; vector pending | ACC-005, LIF-008 | Which grant, if any, preparation establishes on a re-open or a rejoin, and under whose authority. |
| L149 | closed: removed by ruling 41 (principles review item 5) | ruling 41, principles review item 5 | Composite Conduits: how many instances one open creates, whether components are shared between consumers, how required and optional component failures aggregate, and when components end. |
| L150 | ruled: POOL-003, POOL-004; vector pending | POOL-003, POOL-004 | Provisioning a composition twice under a new request: duplicates, no-op or conflict; and which of two `auth` Resources a later sibling binds to. |
| L157 | ruled: CON-008, CON-011; vector pending | CON-008, CON-011 | A provider Resource retired while a consumer holds a required connection to it: whether the Conduit ends and whether the consumer stays prepared. |
| L159 | ruled: CON-008, CON-011; vector pending | CON-008, CON-011 | Declare-lost conflicts with "no permission to bypass an unresolved shutdown duty"; its effects on the Pool, Conduits and grants are undefined. |
| L160 | ruled: POOL-002, POOL-005 to POOL-007; vector pending | POOL-002, POOL-005 to POOL-007 | Dependency composition still says it is "not a template-parameter DSL" and lets a binding name a sibling entry not yet created, against closed declared inputs. |
| L161 | ruled: POOL-003; vector pending | POOL-003 | Name is a standard instantiation argument, but a Pool entry is not a singleton: a named entry provisioned twice conflicts. |
| L162 | ruled: CON-002; vector pending | CON-002 | Whether a consumer pins the Conduit template named in its `open` block at creation, or uses the current template at each preparation. |
| L163 | closed: no join; a target binds like any other reference (see L107) | — | When a join declaration's target binds: at creation, or at each preparation following the name. |
| L164 | ruled: POOL-009; vector pending | POOL-009 | The single place where a Pool entry's placement is supplied: the provisioning request, the Pool instance's entry, or the Pool template's inputs. |
| L165 | ruled: PLC-006, CON-006; vector pending | PLC-006, CON-006 | Where participation and read-position state is authoritative when a location is unreachable, and the host identity of endpoints that have no placement. |
| L167 | ruled: CON-012 to CON-014; vector pending | CON-012 to CON-014 | Adding a connection to an already prepared consumer: established at once or at the next preparation; whether an Interface can be a conversation participant. |
