# Collaborator review

Updated: 2026-10-30. [SPEC.md](SPEC.md) and [DESIGN.md](DESIGN.md) contain the current
non-communication contracts. Intersession communication is reopened and consolidated
here; prior room choices below are material to reconsider, not implementation requirements.

## Review order

1. Refine the concepts and relationships below, using the Pi investigation as evidence.
   Review illustrative YAML alongside those concepts before selecting implementation details.
2. Design what it means to establish a “conduit,” without assuming the former room model.
3. Resolve runtime depth/account integration, finalize initial YAML contracts, then design
   the experiment before implementation.

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
separates the former scope object's responsibilities: an authorized attachment supplies
access, filters constrain it, and role limits remain the ceiling. “Attachment” remains
a working term; its final name, representation and composition are not yet selected.

Review these relationships next:

- Specify reference/selector resolution and update behavior; decide overlapping/nested
  membership separately. Membership, ownership, access and activation
  remain distinct. Resolve how domain resources such as skills compose with role selections
  through general resource contracts, rather than special fields for each use case.
- Define where filters attach and how multiple filters/attachments compose. Keep
  pool membership selectors distinct from access constraints; filters cannot add grants.
  Preserve visible effective access and current-rule checks as the schema evolves.
- A conduit could represent a typed relationship with explicit endpoints and supported
  operations/exchanges. Discuss whether resource access and intersession communication use
  this same concept or distinct relationships with common metadata. Do not assume one
  transport, topology, delivery guarantee or automatic context injection.

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

## Deferred communication redesign

**Status: reopened, after review of the Pi findings.** This is the sole review home for room-specific
and intersession communication choices. The user wants a more extensible way to open
lines of communication between sessions and explore a “conduit” as the abstraction.
The term is provisional: this is a semantic redesign, not a rename of the room API.
No topology, communication record kind, participant binding, tracker requirement, or
communication acceptance criterion below is currently fixed.

### Material carried forward for reconsideration

| Area | Previous approach and reasoning to revisit |
| --- | --- |
| Core abstraction | A shared room/message board modeled after group chat, with durable append-only history. Messages were Conduit records with common identity/scope/provenance metadata, immutable after acceptance, with corrections as later posts to avoid shared-document merge conflicts. |
| Memory boundary | No editable room-owned working-memory document. Sessions kept their own working context; scoped knowledge/checkpoints could be linked or quoted. This was motivated by concurrent editing and ownership concerns. |
| Participants and attribution | The Conduit session identified the participant across resumed executions; fresh/fork created new participants. Each post identified its producing execution. Restart/account/frontend changes preserved participant identity. |
| History and catch-up | Shared history, optional participant-local copies, and one independent read position per session per room. One reader did not consume another's unread history. Late joiners/restarted sessions could recover retained messages, with gaps reported. |
| Replies and addressing | One timeline with reply-to links and optional recipient-session hints. No separate direct-message inboxes or thread cursors; the same-room approach was intended to cover small/private conversations too. Hints did not alter visibility or permissions. |
| Access and membership | Effective scopes governed discovery, joining, reading, and posting. Joining was explicit, without a second invitation gate, and enabled participation/notices without granting extra access. Merely acquiring scope access did not join a room or merge participants' memory permissions. |
| Attention and execution | A brief unread notice on the next normal turn, content retrieved on demand, and no automatic wake/model turn. Paused/closed sessions needed explicit resume; role-specific wake hooks were a possible later capability. |
| Delivery and context | Queued content, retries, and replays were to recheck current recipient access. Whole-record checks and scoped provenance could omit inaccessible content, without erasing context already received. Message acknowledgement, retry identity, ordering, and cursor advancement remained unresolved. |
| Tracker relationship | Independent room identity; tracker-free initial testing, followed eventually by required tracker linkage. The transition milestone/cardinality were unresolved. Provider-native tracking was preferred, and a post did not automatically become an external issue comment. |
| Primary tracker proposal | One primary tracker item plus related references was proposed immediately before this deferral. It was not accepted. Do not treat it as a constraint on the new model. |
| Transport and MCP | Local communication first, with room for remote transport later. Candidate MCP tools covered posts/reads, resources exposed history, and notifications signaled unread content. Protocol primitives alone did not establish application-level durability or delivery guarantees. These mappings and sequencing are reopened. |
| Original experiment | Four accounts discussing work through one room, independent catch-up/read positions, visible message provenance and delivery gaps, plus scope-aware context exchange. These communication checks await redesign; the four-account and memory-access checks remain active. |

### Questions for the future deep review

Start with what a line of communication is for, how it is opened/closed, and what
sessions need to exchange. Compare addressing and topology, directionality, message
or event/request semantics, lifecycle, extensibility, and compatibility with native
agent interfaces. Then choose identity/authority, persistence, attention, delivery
assurances, and relationships to work, pools, trackers, and shared knowledge.
Do not constrain that analysis to group chat, a single log, or the previous membership model.

Use the local `agentwatch` reference for concrete communication behavior and the
[Pi communication research](RESEARCH_PI.md#communication-research-for-the-upcoming-redesign)
for native input timing, pi-messenger delivery, and pi-chat supervision. Separate
addressing, authorization, retained exchange, attention, and execution when comparing
options; this is a discussion frame, not a selected object model.
`jxp-skills` and provider-native tracking remain
useful coordination references. Bring the conclusions back into SPEC.md and DESIGN.md
only after review, replacing this provisional material rather than creating a decision ledger.

## After the review: first YAML design

Discuss concrete examples together before freezing a schema or implementing:

- Choose the configuration schema-version convention, object shapes, names/references,
  file layout, validation, path/endpoint matching, and merge/override rules. Keep versions
  distinct from MCP protocol and plugin versions.
- Define search-limit settings and text-size units; tune default values and caps in the
  experiment.
- Specify managed-worktree creation settings: base revision, branch selection, directory
  placement, and treatment of any existing local changes.
- Express roles, account references, shared connection definitions, attachments/filters, pools,
  and execution policy. Keep runtime, frontend, model and account selections distinct.
  Show resolved launch settings and adapter output; classify supported skill/plugin
  resources without copying every Pi setting. Keep credentials out of shareable
  configuration. Add communication objects only after redesign.
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
layout remain staged. Communication delivery order is reopened above. A custom terminal
is first; other frontends and external native runtime integrations remain goals.
General durable workflows, recoverable script execution and comprehensive Pi plugin
compatibility are not prerequisites for the first Conduit-owned runtime.

Keep reasoning beside accepted contracts in DESIGN.md. Remove answered questions here
and use Git history for earlier versions. The communication material above is a focused
redesign brief, not an additional historical decision register.
