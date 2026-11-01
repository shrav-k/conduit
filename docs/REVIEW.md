# Review outcome and schema walkthrough

Updated: 2026-11-01. The current v0 direction review is complete: R1–R7 are settled within the
boundaries below. There is no remaining tracker ruling. Linear/Jira integrations, message history
and specialized tracking are Resource capabilities within the existing five-family model.
[SPEC.md](SPEC.md) and [DESIGN.md](DESIGN.md) define the current contract; research supplies evidence,
examples and future possibilities rather than a separate decision queue.

This closes the original direction review, not implementation or provider validation. The reviewed
baseline is on `main`; subsequent accepted extension rulings are recorded on the implementation
branch until merged. The earlier baseline remains available in Git history at `c013f0a`.

## Next discussion

The [implementation proposal](proposals/implementation-v0/README.md) now describes processes,
contracts, persistence, ownership, crate organization and delivery milestones. Its
[review queue](proposals/implementation-v0/REVIEW.md) groups seven pending implementation choices.
Start there for architecture review; these proposals do not reopen or supersede R1–R7.

**Current focus:** conceptual plugin and template boundaries, beginning with I3 and concrete author
journeys. The [implementation review](proposals/implementation-v0/REVIEW.md#review-priority-plugin-and-template-authors)
retains collaborative rulings; recommendations have not become defaults. Existing infrastructure
may support the first implementation, but the public authoring and behavioral contracts take
priority over its efficiency or specific runtime arrangement. The initial authoring direction
is now accepted: lightweight Conduit support, with real plugins validating the public protocol
and structure; a broad helper library is not a v0 prerequisite. Plugins publish inspectable
contracts with their implementations, and MCP is accepted as the public plugin protocol base
with a small Conduit profile. Code-optional contributions and independently accessible MCP
components are also accepted; the design-fit audit below states their boundaries. Detailed syntax
across contracts and templates stays at the end, after conceptual rulings.

**Accepted endpoint arrangement:** endpoint templates select connection plugins; instances retain
and manage those implementations and local bindings. Conduits match compatible capabilities and
coordinate establishment, policy and participation. They do not install new endpoint plugin code.
The [family-by-family walkthrough](proposals/implementation-v0/AUTHORING.md#5-where-the-example-implementations-execute)
records this boundary. **Accepted design direction:** Conduit templates compose reusable typed
requirements, with a small base vocabulary and author extensions, as defined in
[DESIGN](DESIGN.md#composable-relationship-requirements). Requirement definitions are reusable and
versioned independently of a particular implementation; multiple plugins may implement the same
definition, and both may ship together. Plugins publish supported versions/roles and known limits;
Conduit checks compatibility before establishment and verifies applicable runtime facts separately.
Requirement definitions may also compose existing requirements without new code when every part
is supported. The [authoring concept map](DESIGN.md#authoring-concepts-beneath-the-object-families)
records accepted temporary names and divisions: contracts/requirements describe behavior and
obligations, graphs compose callable operations, plugins publish contributions, and templates
assemble these into the five families. Graphs may themselves implement typed operations published
through plugins for reuse by callers or other graphs. Final names may change after conceptual
review. Composite public results are derived by default from declared public bindings and
constituent contracts, with explicit construction available and underlying effects/uncertainty
preserved. Graph-backed operations reuse graph recovery by default, resuming valid child progress
on explicit retry and delegating uncertain effects to child contracts. The
[implementation review](proposals/implementation-v0/REVIEW.md#i3--plugin-execution-and-distribution)
now records requirement fulfillment duties as accepted: contracts declare when guarantees must
hold and relevant endpoint roles; established Conduits identify responsible implementations for
Interface inspection. Composed requirements remain individually enforced, with optional known-conflict
or combined-effect checks and no mandatory general solver. Experiment 002's combined authoring
proof is accepted: graph-backed operations, derived/explicit results and recovery, composed/custom
requirements and Interface inspection within the existing synthetic fixture. Authored plugin code
normally runs in supervised instance-associated helpers; trusted built-in Rust code may remain
in-process. Bundled first-party plugins plus explicit local package installation are accepted; a
marketplace and automatic dependency resolution across languages are deferred. I3's conceptual
review is complete. Persistence, credential custody and observation capture are next; exact helper
grouping, evidence mechanisms, recovery details and authoring syntax remain open. No implementation
has started.

I4 is exploring an [active Interface role](proposals/implementation-v0/ARCHITECTURE.md#interface-as-system-director--proposed-refinement):
the root Interface would direct establishment and ongoing administration/repair as part of its
normal responsibilities. Frontends attach to it; management and instance services retain their
accepted state authority, and Pools retain Resource ownership. Responsibility and authority are
being refined before accepting that role: directives, implemented capabilities, permission to act
and confirmed target outcomes must remain distinct. The
[cloud IAM follow-up](research/CLOUD_AUTHORITY.md#9-interface-administration-and-independent-policy)
now recommends policy independent of Interface lifecycle, administered by multiple authorized
Interfaces with broad default root administration. The user leans toward this; it is not yet a
ruling and does not narrow broad v0 Interface access. The expanded role, foundational storage
and first storage engine remain under discussion.

I1 is settled for the initial implementation: separate execution services supervise managed Agents/Resources while
management coordinates configuration/control and the Interface presents them. External MCP
services retain their own supervision. Shared registration/template and access-policy authority
plus instance-owned state are also accepted: one designated service owns each instance's resolved
definition, confirmed configuration/revision and operation results; Interface views identify
freshness and gaps. The [cloud authority comparison](research/CLOUD_AUTHORITY.md) records the
rationale and alternatives. V0 uses no offline grants: retain supervision and dispatched
outcomes, but hold new managed requests without verifiable permission. Remote production support
still requires validation. The first proof uses management and two independent executors as
separate processes on one machine, with controlled connection failures; two-machine validation
follows before remote support is claimed. Logical ownership is not a process-per-object rule.

## Design-fit audit: code-optional plugins and independent MCP access

These extensions fit the existing five families without reopening R1–R7. The authoritative detail
is in [plugins and MCP components](DESIGN.md#plugins-and-independently-accessible-mcp-components).

| Existing boundary | Consequence of the accepted extension |
| --- | --- |
| Plugins versus templates/instances | Plugins publish reusable contracts and implementation bindings; templates configure them; instances realize them. A normal MCP endpoint configuration need not become a new plugin. |
| Pool ownership and unitary Resources | Code-optional Resources still have one owner. Search and MCP-access responsibilities can remain distinct, connected by Conduits. External infrastructure is not implicitly owned by its client Resource. |
| Fixed definitions and implementations | Retain declaration/mapping and adapter identities. Changes to those fixed definitions require replacement; hosted provider code is observed for compatibility, not falsely claimed as pinned. |
| Authentication, filters and supported behavior | Reuse Pool authentication and current access paths. A declared mapping cannot invent permissions, provider semantics or safe retry guarantees. |
| Interface and independent capabilities | Components may expose supported MCP surfaces without the aggregate UI in the call path. Managed evidence remains inspectable through the Interface; no process-per-object or public setup-export requirement follows. |
| Partial adoption | Standalone implementations supply their own dependencies/guarantees. Managed components retain required ownership, authority and evidence. A standard MCP client does not need a Conduit graph SDK to use a public capability. |

No additional product-level ruling is needed to adopt these consequences. I1's management/execution
separation, shared-policy/instance-state authority and no offline grants are accepted. Resolve
the remaining I4–I6 choices; I3's conceptual execution, extension and distribution boundaries are
settled. I2 is complete: native Claude/Codex adapters after the synthetic proof, then Conduit's
Rust model/tool loop; macOS-first native integration/account validation with a portable core and
OS adapters. Final contract/template syntax remains I7 work.

## Configuration walkthrough

Review the [concrete YAML packet](../experiments/001-shared-access/README.md) as one connected
example. It now includes templates for all five families, resolved instance examples, fixture
contracts, a control-change walkthrough and an acceptance plan. Experiments live at the repository
root with their own documentation; they are separate from the specification and source research.

Focus on whether Pool entries versus explicit setup, dependency binding, Conduit access,
instance-owned runtimes and supported controls read naturally together. The proposed identifiers,
keys and plugin contracts are not approved merely because the files parse. Bring material behavior
changes back with a counterexample; choose routine spelling and engineering defaults together
rather than reopening every settled ruling.

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

The [experiment directory](../experiments/README.md) owns test sequence and evidence. The first
packet is static documentation; synthetic runtime tests and live provider checks have not run.
Use the same Interface for control and inspection throughout. Progress from two synthetic Agents
to the required separate two-Claude/two-Codex compatibility checks, without an issue-tracker gate.

## Drafting and deferred work

| Work | How to resolve it |
| --- | --- |
| YAML field names, API/type/capability versions, exact ID/cursor/receipt formats and validation diagnostics | Review the proposed packet, then define schemas against the agreed contracts. |
| Rust crates, plugin ABI/processes, persistence, transport, client pooling and exporter implementations | The [implementation packet](proposals/implementation-v0/README.md) turns [research](research/RUST_RUNTIME.md) into a proposal and grouped review. [Experiment 002](../experiments/002-runtime-plugins/README.md) provides the first failure tests; implementation choices remain unratified. |
| Limits, timeouts, provider-specific queues/recovery and UI layout | Type/adapter defaults tested against failure examples, not universal micro-rulings. |
| Multi-user/remote administration, broader interception and sandbox tiers | Later extensions; retain current native/isolated boundaries and honest coverage. |
| Advanced sharing/publication, automatic extraction/expiry, inheritance and public registries | Keep deferred until a concrete need warrants the added contracts. |
| Additional Resource types, communication modes and integrations | Extend capability/type/plugin contracts; no closed catalog or new core tracker concept. |

Research still contains broader candidate behavior. Its descriptions of Signal, MCP, A2A, Pi,
Chord or agentwatch are evidence, not automatic Conduit requirements. Keep source facts/version
limits, and bring only material new protocol choices back to review.
