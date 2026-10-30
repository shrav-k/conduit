# Collaborator review

Updated: 2026-10-30. [SPEC.md](SPEC.md) and [DESIGN.md](DESIGN.md) contain accepted
contracts. A conduit is now the common relationship for both resource access and
session communication, and the project's central concept. Its specific interaction
contracts are the next deep research priority; previous room choices remain unselected.

## Review order

1. Research communication and resource-access strategies against the conduit model,
   using the brief below. Produce a comparison and proposed contract boundaries.
2. Review the recommended interaction kinds and YAML examples together, including how
   conduits connect pools/resources and sessions through filters and role limits.
3. Resolve runtime depth/account integration, finalize initial YAML contracts, then design
   the experiment before implementation. Do not freeze a transport or library first.

## Concepts and configuration review

**Accepted frontend clarification:** a custom Conduit terminal inspired by Pi is sufficient.
Full stock-UI preservation is unnecessary; selective component reuse remains an implementation
choice. Conduit owns its runtime.

**Accepted:** a **pool** is a named, extensible collection of resources relevant to
interacting with a domain, replacing the former Conduit project grouping. It includes
both data and capabilities. Examples such as tools, skills and documents do not define
a closed list; use-specific names such as “onboarding” should not become dedicated
core kinds or pool fields. The generic resource/membership contract should accommodate
new kinds. Pools support both explicit resource references and label selectors. The
prior project access-default behavior now applies to pools.

**Accepted:** filters only narrow selections/grants; they never grant access. This
separates grant and constraint responsibilities: an authorized conduit supplies access,
filters constrain it, and role limits remain the ceiling.

**Accepted:** “conduit” covers resource access and session communication. This interaction
strategy is the project's core. It replaces the provisional access “attachment” term;
its exact kinds, endpoints, representation and guarantees require deep research.

Review these relationships next:

- Specify reference/selector resolution and update behavior; decide overlapping/nested
  membership separately. Membership, ownership, access and activation
  remain distinct. Resolve how domain resources such as skills compose with role selections
  through general resource contracts, rather than special fields for each use case.
- Define where filters attach and how multiple filters/conduits compose. Keep
  pool membership selectors distinct from access constraints; filters cannot add grants.
  Preserve visible effective access and current-rule checks as the schema evolves.
- Determine the common conduit contract and the interaction-specific extensions.
  Establish endpoint/addressing, lifecycle, authority and observation semantics without
  assuming one transport, topology, delivery guarantee or automatic context injection.

Use pi-ai for provider/capability contracts, pi-mcp for protocol/transport separation,
pi-telemetry for explicit operation attribution, and Chord for typed references, caller-aware
interfaces and observable state. Keep retained exchanges and execution outcomes distinct
from transient state subscriptions. These are design inputs, not selected dependencies.

Use Kubernetes and networking APIs heavily as design references, including Kubernetes
versioning. Candidate patterns are common identity/metadata, kind-specific schemas,
references/selectors, intended configuration versus observed status, and explicit
relationship contracts. Exact field names and the version scheme remain undecided.
API/schema versions, record revisions and application/plugin releases describe different
changes and must not be conflated. Source references:
[objects](https://kubernetes.io/docs/concepts/overview/working-with-objects/),
[labels/selectors](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/),
[API groups/versioning](https://kubernetes.io/docs/reference/using-api/),
[custom-resource versions/conversion](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definition-versioning/),
[Gateway relationships](https://gateway-api.sigs.k8s.io/docs/concepts/api-overview/), and
[cross-namespace reference grants](https://gateway-api.sigs.k8s.io/reference/api-types/referencegrant/).
These inform Conduit; they do not select its deployment machinery, namespace semantics,
full conversion infrastructure or automatic agent-restart behavior.

Use concrete examples to test references, selection, composition, ownership, lifecycle
and effective access. YAML authoring should describe intended configuration; discovered
capabilities, live sessions, credentials and observed state need their own treatment.
Keep proposals here until ruled, then incorporate them into SPEC.md/DESIGN.md.

The four-account objective remains: two Codex/OpenAI and two Claude subscription
identities running concurrently with separate private state and scoped shared knowledge.
The own-runtime authentication path needs validation; native Claude Code/Codex integration
is a separate compatibility claim. Communication will be specified after its redesign.
Native execution remains the default with optional/required isolation; “native” here
means running on the host, not selecting Pi's or another external agent's runtime.

## Runtime and frontend review

**Settled by the user's clarification:** Conduit implements its runtime in Rust. A Pi-inspired
custom terminal is the first of multiple possible frontends. Pi's agent loop and Pi Durable
are research references, not execution dependencies. The earlier stock-Pi/thin-Conduit recommendation is removed.
This does not abandon external native agent adapters or imply a replacement skills
framework, tracker, or search engine.

The [research report](RESEARCH_PI.md) contains the detailed source evidence and
[proposed runtime boundaries](RESEARCH_PI.md#proposed-rust-runtime). It distinguishes
frontend rendering, provider adapters, execution recovery and shared memory. Resolve
these two implementation-scope choices after the concept/configuration discussion:

| Choice | Recommendation and implication |
| --- | --- |
| **First runtime depth** | Own the turn/tool loop, private history, bounded context, durable input/tool intents and outcomes, explicit recovery and usage from the start. Defer general workflows, background scheduling/compaction, deterministic Code Mode replay and hot reload. Confirm the minimum recovery and long-session behavior before defining concrete state/storage schemas. |
| **Subscription/account path** | Keep supported native login delegation for external Claude Code/Codex adapters; establish supported provider/account adapters for Conduit-owned execution. Verify distinct identities, refresh ownership and eligibility. Pi OAuth source does not prove those requirements or authorize token reuse. Unsupported subscription access must be surfaced, not replaced silently by API billing or an external runtime. |

The frontend control/state contract should expose Conduit's existing one-controller,
detach, resume/fresh/fork and coverage semantics. Evaluate MCP where it fits and a thin
versioned control/event API where necessary; choose exact transport and schemas during
implementation planning. UI reuse must not accidentally introduce another agent loop.

Memory lifecycle, scoped retrieval, BYOS, execution modes, portable checkpoints and
usage contracts remain applicable. Runtime-specific plugins need compatibility checks
or ports. The research's implementation details are proposals, not a new decision ledger
or accepted requirement to reproduce every Pi feature.

## Conduit interaction research

**Direction established; detailed research is next.** This brief defines what the
investigation must answer. It is not a completed research report or a selected set of
conduit types. Access and communication are one design problem with different interaction
contracts, rather than independent features to bolt together after implementation.

### Questions and comparison criteria

| Area | What the research must resolve |
| --- | --- |
| Interaction patterns | Compare capability/resource invocation, query/read, request/reply, asynchronous messages, publish/subscribe, retained streams, state observation and handoff. Identify a small common contract and where distinct semantics are necessary. This is a candidate list, not a closed taxonomy. |
| Endpoints and topology | What may be addressed: session, resource, pool selection, service or other endpoint? Compare direct and routed links, one-to-one and one-to-many exchange, discovery and binding. Keep role presets separate from running participants. |
| Contract and transport | Which semantics belong to Conduit, which map to existing protocols, and which require adapters? Preserve provider-native operations and distinguish configured relationships from sockets/processes. Compare local-first operation with remote extension and Rust integration. |
| Authority and filtering | Who may establish or accept a relationship and grant operations? How do endpoint authority, role ceilings and narrowing filters compose? Cover current-rule checks and revocation without presupposing comprehensive v0 sandboxing. |
| Delivery and failures | Ordering scope, correlation, durable acceptance, acknowledgement meaning, replay/cursors, duplicates, idempotency and unknown outcomes. Compare offline endpoints, storage failures, retries, reconnect and cancellation; separate delivery from execution and completion. |
| State and context | Retained exchange versus current state snapshots, payloads versus references/artifacts, query versus unsolicited delivery, and optional context inclusion. Preserve provenance and private working context; observing state does not automatically require shared editable memory. |
| Lifecycle and attention | Authored declaration versus active instance, opening/closing, pause/resume, timeouts, expiry, backpressure and participant turnover. Distinguish a message notification from permission to start a model turn. |
| Extensibility and observability | Common identity/schema/version rules, interaction-specific extensions, capability negotiation and unsupported behavior. Carry operation/execution/account attribution across boundaries without conflating traces with retained outcomes. |
| Configuration and usability | Demonstrate pool references/selectors, conduit endpoints, grants/filters and observed status through YAML. Compare direct configuration with reusable presets; make resolution and omissions inspectable. |

### Reference families to investigate

Use primary documentation, specifications, source and focused tests. Pin inspected
versions and distinguish source evidence, executed checks and proposed adaptations.
These are starting points to compare, not dependencies or an exhaustive source list.

| Family | Starting references | Investigative purpose |
| --- | --- | --- |
| Typed object/capability interaction | Pi Chord and its local source; capability-oriented RPC designs | Caller-aware interfaces, typed references, observable state, lifecycle and local/remote consistency. |
| Existing model/agent protocols | [MCP](https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture), [A2A specification](https://a2a-protocol.org/v1.0.1/specification/) | Resource/tool interaction, agent discovery, work exchanges, status and artifacts; establish where protocol guarantees end. |
| Messaging and retained exchange | [NATS/JetStream](https://docs.nats.io/concepts/jetstream), [Zenoh specification](https://spec.zenoh.io/spec/1.0.0/concepts/entities.html); compare topic and queue designs as needed | Routing, query versus publication, independent consumers, persistence, replay, acknowledgements and backpressure. |
| RPC and streams | [gRPC concepts](https://grpc.io/docs/what-is-grpc/core-concepts/), relevant HTTP/JSON-RPC streaming contracts | Request boundaries, cancellation, deadlines, streaming and transport limitations. |
| Declarative connectivity | Kubernetes objects/selectors/versioning and Gateway API references above | References, attachment/route concepts, grants, policy placement, extension points and observable resolution. |
| Execution ownership and recovery | Pi Durable, actor/mailbox/supervision designs, durable-execution systems | Work ownership, scheduling boundaries, cancellation propagation, recovery and why message receipt differs from completed work. |
| Practical agent coordination | Local agentwatch, pstack-pi, jxp-skills; previously inspected pi-messenger/pi-chat | Actual multi-session/account workflows, retained history, attention, failures and provider-native tracking. |

### Scenarios to use throughout

Compare approaches against the same concrete cases:

1. Four sessions across two Claude and two Codex/OpenAI accounts use one domain pool
   with different allowed operations and narrowing filters, retaining private state.
2. One session requests a review from another and can distinguish acceptance, progress,
   result, cancellation and an unknown outcome after interruption.
3. A session exposes a finding or artifact to several permitted recipients, including
   a recipient that reconnects later. Determine delivery and context inclusion separately.
4. A frontend or session observes progress without receiving every retained payload or
   gaining permission to execute work; a slow observer cannot create unbounded buffering.
5. Access changes while work is queued or reconnecting; retrying a provider write must
   respect the existing uncertain-outcome contract.
6. Work continues after a model/account/frontend change with permitted context and
   provenance, without confusing a role preset, session, execution and provider identity.
7. A tracker integration associates exchanges with provider-native work records without
   requiring every interaction to be a room or adopting an internal ticket engine.

### Required research output

Produce one focused report with a comparison matrix, concrete flow/failure examples,
and recommendations. Include candidate conduit contracts and illustrative YAML, a small
v0 set versus later extensions, and what to reuse, adapt or implement in Rust. Explain
which designs were rejected and why only where it clarifies a live tradeoff. Return a
short list of consequential rulings rather than a new exhaustive decision ledger.

No backend, broker, transport, topology, room model or automatic-wake policy is selected
by this direction. Existing account/access, explicit session lifecycle, retention and
safe-retry contracts remain constraints. Configuration and contract design precede code.

### Prior room ideas as comparison material

Earlier room discussions are hypotheses for comparison, not current requirements:
append-only group history with per-session cursors; session identity across executions;
reply links and recipient hints; scope-authorized participation; passive unread notices
with possible later wake hooks; tracker linkage; and local-first transport. The former
model avoided a shared editable room scratchpad. Research should explain which ideas
fit particular conduit contracts and which should be replaced. None establishes the
new model's topology, identity, membership, tracker cardinality or delivery guarantees.

The [Pi communication investigation](RESEARCH_PI.md#communication-research-for-the-upcoming-redesign)
provides initial evidence, including input timing and mailbox/supervision behavior.
Use Git history for the full earlier discussion; keep accepted results in SPEC.md and
DESIGN.md as the review progresses.

## After the review: first YAML design

Discuss concrete examples together before freezing a schema or implementing:

- Choose the configuration schema-version convention, object shapes, names/references,
  file layout, validation, path/endpoint matching, and merge/override rules. Keep versions
  distinct from MCP protocol and plugin versions.
- Define search-limit settings and text-size units; tune default values and caps in the
  experiment.
- Specify managed-worktree creation settings: base revision, branch selection, directory
  placement, and treatment of any existing local changes.
- Express roles, account references, shared connection definitions, conduits/filters, pools,
  and execution policy. Keep runtime, frontend, model and account selections distinct.
  Show resolved launch settings and adapter output; classify supported skill/plugin
  resources without copying every Pi setting. Keep credentials out of shareable
  configuration. Derive interaction-specific objects from the research review.
- Consider explicitly writing execution mode in generated role files; decide whether
  handwritten files may omit it and use the native default.
- Check ordinary use and conflicting/incomplete configurations. Preserve native tooling;
  do not require Kubernetes machinery, a replacement skills language, or an internal ticket engine.

## Then: initial experiment design

Use the [proposed trial sequence](RESEARCH_PI.md#proposed-experiment-sequence) and its
fault cases as input. First demonstrate a Pi-derived frontend against a fake Conduit
control/state service, then a minimal Rust-owned model/tool loop. Neither proves live
subscription access; account feasibility should be investigated before extensive UI work.
No experiment should use Pi's runtime to make the own-runtime requirement appear met.

Then select host/provider versions, account enrollment, a minimal memory backend and an
isolation backend to evaluate. Define setup, expected observations, failure criteria,
restart/cleanup and trial order before running anything. Cover four identities,
permitted/shared and private knowledge, independent account lifecycle, session continuity,
frontend replacement, execution modes and honest usage. Add communication after redesign.

Use fixture failures around tool intent/effect/result boundaries, storage acknowledgements,
permission changes, stale workers and frontend reconnect. Evaluate actual subscription
identity/refresh separately from fake-account labels. Native external-agent interoperability
and OS isolation require their own checks; a Conduit provider call does not prove either.
No account registration, implementation start or live experiment is implied by this review.

## Deferred implementation scope

Comprehensive MCP operation inspection/enforcement, advanced memory-sharing delivery,
automatic classification/extraction, automatic expiry, exact cost accounting, plugin ABI, and crate
layout remain staged. Conduit interaction kinds and their delivery order follow research. A custom terminal
is first; other frontends and external native runtime integrations remain goals.
General durable workflows, recoverable script execution and comprehensive Pi plugin
compatibility are not prerequisites for the first Conduit-owned runtime.

Keep reasoning beside accepted contracts in DESIGN.md. Remove answered questions here
and use Git history for earlier versions. The communication material above is a focused
redesign brief, not an additional historical decision register.
