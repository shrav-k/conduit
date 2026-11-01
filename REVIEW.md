# Review outcome and schema walkthrough

Updated: 2026-10-31. The current v0 direction review is complete: R1–R7 are settled within the
boundaries below. There is no remaining tracker ruling. Linear/Jira integrations, message history
and specialized tracking are Resource capabilities within the existing five-family model.
[SPEC.md](SPEC.md) and [DESIGN.md](DESIGN.md) define the current contract; research supplies evidence,
examples and future possibilities rather than a separate decision queue.

This closes direction review, not implementation or provider validation. Work remains on
research/conduits-as-graphs; the earlier baseline is preserved on main at c013f0a.

## Next discussion

Draft a coherent YAML and acceptance packet using one concrete scenario. Review the objects
together so their relationships are visible, instead of voting on every field independently:

1. A reusable Interface supplies the unified control and observability surface, exposed through
   CLI/terminal/test frontends. Establish and inspect the system management connection.
2. A Pool offers explicit Resource-template entries. Select provisioning or existing-instance use
   for a directory/worktree, shared authentication, an MCP integration and retained message history.
   Listing an entry alone starts nothing.
3. Instantiate two Agents with separate account/private state and declared Conduits to permitted
   Pool Resources. Agent activation determines what becomes available in the selected runtime.
4. Walk one supported configuration change through its typed control, handler, revision check,
   confirmed setting and Interface-visible evidence. Include an uncertain result.
5. Post a message, disconnect one participant, read independently, narrow permission and reconnect.
   Follow the same activity through Interface inspection and an exporter test.
6. Show template YAML, resolved instance configuration and system-owned observations separately;
   include valid/invalid examples and explain every default and reference.

Draft exact fields, type/capability identifiers, event and cursor shapes within the agreed language.
If an example reveals a materially new behavior, identify that question with the concrete example.
Do not reopen settled rules merely to select names, constants or a Rust library.

## Review outcomes

| Bundle | Accepted v0 contract | Deferred boundary |
| --- | --- | --- |
| R1 — instance definitions | Fixed graph/control definitions require replacement instances, including for unrestricted Interface administration. Existing controls and connection/lifecycle operations remain available. Explicit backing reuse is supported where its type permits. | In-place definition upgrades and migration. |
| R2 — construction | Finite acyclic graphs of typed plugin steps; output bindings imply dependencies and explicit ordering covers other prerequisites. Plugins handle branches/loops; instances own ongoing services. Waits, valid-progress recovery and supported parallel steps remain. | Authored loops, branching and dynamic graph fan-out. |
| R3 — ownership/lifecycle | Each Resource has one Pool owner. Explicit Resource retirement precedes Pool deletion, with bulk action and partial-outcome reporting. The last logical peer/consumer ends a Conduit; later participation gets a new instance. Backing-data cleanup remains separate. | Ownership transfer and same-ID reopening of ended flows. |
| R4 — composition/access | Explicit named Pool entries and provision-or-use-existing intent. No generic labels/selectors or Pool default Agent connections. Capability matching, whole-path filters, unambiguous/explicit routing and recipient-bound delegation apply. | Implicit creation/reuse inference or silent route/account substitution. |
| R5 — activation | Declared compatible preparation, required/optional readiness, deliberate collision handling and explicit refresh. Known stale/incompatible calls are withheld; detection limits remain visible. Current access, credential refresh and reconnect are separate. | Automatic first-use/background activation refresh. |
| R6 — configuration/extensions | Strict typed YAML, materialized instance defaults, one stored configuration map, confirmed control changes and separate uncertain requests. Fixed definitions and exact Conduit-selected implementations remain bound to an instance; upgrade by replacement. | YAML convenience expansion, in-place implementation upgrades and implicit substitution. |
| R7 — communication/observation | Composable capability guarantees; retained messages and inspection first. Shared append-only history, explicit participants, independent read positions and next-turn notices without automatic wake. Durable local evidence and pluggable export are intrinsic to Interface observability. | Structured work submission, automatic-wake hooks and remote communication. |

These outcomes retain the earlier common envelope/forms, stable IDs, checked writes, explicit
rebinding, private Agent state, native/default and optional/required isolated execution, scoped
memory, safe retries, retention-until-purge and supported native-provider contracts. Their details
remain in DESIGN rather than duplicated as another decision ledger here.

## Communication and Resource boundaries

The first retained-message capability supports direct conversation with two explicit participants
and group discussion with more. Its shared append-only history belongs to a Pool-owned Resource.
Each Agent reads independently under current access, with bounded catch-up. Reading does not
consume another participant's unread content, grant access or prove model understanding.

Posting, retrying delivery or replaying content does not start an extra model turn. Supported
integrations provide a brief unread notice on the next normal turn; message content is explicitly
retrieved. Notice support and gaps are inspectable. An optional per-Agent automatic-wake hook
remains a future capability with its own execution/authorization behavior.

An Agent may ask another to review through a message and later receive findings as another
contribution. That is not a structured job-acceptance/progress/result protocol. Structured work
submission is deferred; existing tools, typed controls and their outcomes remain available.

Neither this retained-history capability nor a room is the required shape of every Conduit.
Conduits remain extensible connections. Resources can supply other communication or tracking
capabilities. Linear/Jira integrations are ordinary Resources; there is no mandatory issue link,
core tracker abstraction, ticket engine or unresolved future requirement to add one. A custom
Resource can implement domain-specific tracking without changing the protocol's object families.

Shared context remains explicit Resource access, not automatic merging of Agent histories.
Ending a Conduit preserves separately retained Resource content; later access uses a new flow
with current authority. Existing safe-retry and retention rules apply to message operations.

## Interface observability

Control and observability are inseparable parts of the Interface contract. The same Interface
that authors, starts, connects and changes instances exposes their state, activity and outcomes.
Multiple CLI/terminal/test frontend attachments inspect the same underlying evidence, including
Agent-initiated and Resource-to-Resource operations.

V0 requires a durable, inspectable local foundation with pluggable export. Managed components
produce evidence even when no frontend is attached. This does not require an always-running UI
or a second per-frontend event history. An external dashboard/backend is optional; it supplements
the Interface instead of defining whether core observation exists.

| Coverage | Acceptance evidence |
| --- | --- |
| Identity and causality | Follow an action through its actor, target, execution, operation, Conduit path and relevant plugin work without confusing shared service identity with caller identity. |
| Setup and controls | Inspect progress, waits, failures, valid retry, supported changes and confirmed versus uncertain outcomes. |
| Communication and access | Inspect posting, independent consumption progress, connection changes and relevant access decisions without claiming storage is execution. |
| Model/tool activity | Correlate Conduit-managed calls and available model/subagent activity and usage. Distinguish counts, estimates, unknown coverage and observed zero; avoid overlap/double counting. |
| Persistence and presentation | Reopen a frontend and inspect retained outcomes. Report stale snapshots/gaps. An exporter failure must not erase committed local evidence. |
| Content boundaries | Retained payloads, credentials, ordinary diagnostic traces and usage records follow their applicable access/retention rules; export does not imply copying every payload or secret. |

Exact event schemas, exporter formats and UI presentation are drafting/engineering work.
Native integrations must identify their actual coverage. Core observation does not promise
interception of every native action or visibility into unreported model-internal activity.
Custom tracking Resources can extend this foundation without replacing it.

## Initial experiment

Prepare the YAML/contract packet and a synthetic acceptance walkthrough before implementation.
The local-first direction and required four-account compatibility checks remain in force.

1. Use an Interface to establish a Pool and independent Agents. Inspect ownership, resolved
   configuration, active connections, readiness and outcomes through the same Interface contract.
2. Exercise shared service authentication and separately filtered access, a directory/worktree,
   retained message history and the selected memory capabilities. These are Resource examples;
   no Linear/Jira integration or particular backend is a prerequisite.
3. With two Agents and an Interface, exercise independent reading, offline catch-up, a lost response
   and safe retry, permission narrowing, a slow observer and reconnection. Verify no automatic wake.
4. Inspect the corresponding local evidence, supported usage attribution and explicit gaps.
   Exercise export and exporter failure. Distinguish simulated evidence from actual enforcement.
5. Separately verify two Claude and two Codex accounts, private-state and refresh/logout separation,
   native compatibility and telemetry before claiming support. This document records intended tests,
   not completed provider validation or live sign-ins.

## Drafting and deferred work

| Work | How to resolve it |
| --- | --- |
| YAML field names, API/type/capability versions, exact ID/cursor/receipt formats and validation diagnostics | Draft examples and schemas against the agreed contracts. |
| Rust crates, plugin ABI/processes, persistence, transport, client pooling and exporter implementations | Engineering choices; surface any material behavior or trust change. |
| Limits, timeouts, provider-specific queues/recovery and UI layout | Type/adapter defaults tested against failure examples, not universal micro-rulings. |
| Multi-user/remote administration, broader interception and sandbox tiers | Later extensions; retain current native/isolated boundaries and honest coverage. |
| Advanced sharing/publication, automatic extraction/expiry, inheritance and public registries | Keep deferred until a concrete need warrants the added contracts. |
| Additional Resource types, communication modes and integrations | Extend capability/type/plugin contracts; no closed catalog or new core tracker concept. |

Research still contains broader candidate behavior. Its descriptions of Signal, MCP, A2A, Pi,
Chord or agentwatch are evidence, not automatic Conduit requirements. Keep source facts/version
limits, and bring only material new protocol choices back to review.
