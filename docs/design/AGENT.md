# Agent contracts

Part of the accepted [design contracts](../DESIGN.md). Updated: 2026-11-02.
The [conceptual review](../REVIEW.md) records completed rulings and deferred work;
syntax and implementation examples do not amend these rules.

## Agents

### Templates

An Agent template is reusable configuration for any number of independent Agents. It specifies
runtime/model/account defaults, execution preferences and requirements, optional permission
limits, default connection choices, activation instructions and the exposed configuration controls
with their constraints. Establishment uses one Agent template's configuration or a directly
authored complete instance definition. Export/creation representation belongs to the configuration
profile; direct creation itself is accepted. V0 defers inheritance and composition between Agent templates.

Establishment produces the Agent's complete spec. The Agent operates from that spec
and explicit live dependencies; retain actual execution settings.
Permission limits remain current. Resource data stays available through live authorized reads;
refresh of loaded content follows the activation contract below.

### Activation instructions

Activation instructions are supplied directly in the Agent spec or materialized from its
creation template. They tell it how to prepare and use Resources made available through its Conduits.
Resource-access Conduits select
and filter; they do not own import or context-loading instructions. The Agent performs
activation through compatible integrations, within current permissions.

Activation behavior is distinct from the Resource content it consumes. Skills, documents,
prompts, tools and configuration bundles are supplied through Pools; the Agent spec describes
how to use the permitted selection. Discovering a skill, importing it, exposing a tool
and placing instructions in model context can require different behavior.

**Activation policy:** apply the Agent's declared compatible preparation actions.
Required activation failure prevents readiness; optional failure is visible without blocking
unrelated usable capabilities. Resolve conflicting exposed names through explicit selection or
supported distinct names, not discovery order. Avoid duplicate equivalent loading without merging
different configurations, access paths or authority. Compatibility depends on the actual runtime;
model preference alone does not establish native plugin or hook compatibility.

For example, preparation can advertise selected skills for on-demand loading, inject a required
engineering guide and expose optional Linear tools. These are actions in the existing setup/control
model. Template/type defaults can supply conventional behavior without a human prompt for each step.

Source content changes become available for ordinary authorized reads, but do not silently replace
already-loaded instructions or executable extensions. A supported explicit refresh applies at a safe
runtime boundary and reports what was loaded; it does not rewrite earlier conversation history.
An authorized Interface or Agent can request refresh without mandatory human confirmation. Where
refresh is unsupported, report the limitation and use supported restart/replacement behavior; changing
fixed activation/control definitions still requires a new instance. Late dependency failures retain
the agreed independent-work behavior, not a new whole-Agent pause requirement.

Automatic reload/refresh policies are deferred beyond v0, including automatic catalog replacement
on first use. Initial discovery and explicit refresh remain supported. Current access restrictions,
credential refresh and bounded transport reconnection keep their own contracts. The MCP section
below distinguishes stale tool definitions from changed service data. Exact action fields, evidence,
compatibility diagnostics and future automatic-refresh triggers remain schema/adapter work.

### Account selection

Agent account references select identities without embedding credentials. Model-provider
and resource-service credentials remain distinct. Conduit verifies the account actually
selected; no silent account or API-billing fallback is permitted. Concurrent subscription
accounts must not overwrite one another's login, refresh, logout or private state.

External-agent integrations delegate authentication to their native runtimes. Conduit's
own runtime needs supported provider/account adapters, with validated subscription
eligibility, identity evidence, credential namespaces and refresh ownership. Successful
login alone does not prove concurrent-account correctness.

Shared [Pool service authentication](POOLS.md#shared-access-and-authentication) supplies capabilities such
as Linear, GitHub, AWS or a password manager without another service login for each Agent. It does
not replace an Agent's explicitly selected model subscription or merge private state. Reuse of
model-account credentials is governed by that runtime's account contract; do not automatically
redirect model requests to a Pool's service identity.

Coordinate credential reuse separately from Agent history. Do not switch accounts by
swapping one global credential directory or blindly cloning rotating credentials. An
Agent template is distinct from a native agent's login profile. The credential/state
layout is an adapter validation task, not a prescribed filesystem schema.

### Execution policy

Native execution is the v0 default: the runtime runs on the host without Conduit's full
OS sandbox. Isolated execution is selectable; a template requirement is materialized in the
Agent spec, alongside explicit authority dependencies. Required isolation cannot be weakened
by launch overrides; unavailable required isolation causes
a visible launch failure. Show the configured mode and record actual enforcement coverage.

Native mode preserves host integrations and suits users granting broad local access.
Account/Agent state separation addresses configuration collisions without the setup
cost of mandatory isolation. It is not an OS security boundary. Isolated mode covers
the runtime and its child processes; host tooling, keychains, IDE sockets and local
services may require explicit bridges or remain unavailable. This is why both modes exist.

Isolated execution defaults to declared network destinations: required model/login
services, selected connections, and extra destinations authorized through Conduits.
Agent limits and exclusions still apply; local/private services require explicit
access. Network reachability does not authorize every operation at a provider. Exact
sandbox technology and matching rules remain open. Native host access, network policy,
provider authorization and Conduit-managed access have different coverage.

### Instances

An Agent has a durable identity, resolved configuration, private context/history and
independent connection attachments. Sharing an Agent template or account does not share private state
or establish communication. One Agent may span multiple executions; record their actual settings.
The selected Agent example keeps its runtime/account/provider boundary fixed while allowing
supported model selection within it. That policy illustrates a template-declared control contract,
not a universal mutability rule for every family or integration.

There is at most one active primary execution per Agent instance. Independently coordinated parallel
workers use separate Agents. Runtime-managed subagents may remain within their parent's
execution, with ownership and usage attributed where available.

### Runtime and presentation

Conduit implements its own runtime in Rust: Agent supervision, model/tool execution,
context construction, provider dispatch, recovery and observations. Provider adapters
handle request/response conversion and supported authentication. External-agent adapters
connect separate runtimes and report their actual lifecycle/access/telemetry capabilities.

**Accepted delivery order:** after the synthetic runtime experiment, validate real Agents through
native Claude/Codex adapters first, then build Conduit's own Rust model/tool loop. The native path
tests separate subscription accounts, private state and shared Resource access through Conduits;
it does not establish the owned loop's behavior or unobserved native activity. Both paths remain
goals.

**Accepted initial platform:** validate native Agent integration and subscription accounts on
macOS first. Keep the Rust core portable and isolate OS-specific process,
credential and filesystem behavior behind adapters. Other platforms require their own integration
evidence before support is claimed; this is initial test coverage, not a permanent platform limit.

The first frontend is a custom terminal inspired by Pi. Presentation is replaceable;
it does not own credentials, model calls or authoritative execution state. Pi terminal
components may be reused, but Pi's agent loop and Pi Durable are research references.
A memory backend or frontend change must not replace Conduit's execution loop.

**Shared Interface control:** an Agent's direct user-control belongs to its controlling
Interface. All authorized frontend attachments of that Interface can issue input, cancellation and
settings commands; a terminal and CLI need no controller handoff between them. Each request keeps
its caller/attachment identity and passes through its permitted Conduit path and target contract.
Owner-operated frontends use the Interface's unrestricted v0 authority; configuration revision
checks and per-target coordination still provide meaningful outcomes. Simultaneous submission
does not promise simultaneous execution.

**Delegated Agent control:** an authorized Agent may invoke another Agent's permitted
controls through its own Conduit and identity while user-control through the Interface remains
available. No exclusive controller handoff is required. Each request remains subject to current
authority, relevant state checks and target coordination. Neither actor has automatic priority;
coexistence does not imply simultaneous execution. Ordinary communication delivery and wake
behavior remain separate from permission to invoke a control.

Differentiated control/authority across distinct Interfaces is deferred. Precise input scheduling
and loss/recovery mechanics remain open.
Disconnecting a frontend detaches and preserves supported ongoing Agent execution. Keep persisted
history, current model context, frontend state and shared Resource content distinct.

### Lifecycle and continuity

| Operation/state | Contract |
| --- | --- |
| Open, paused, closed | Agent lifecycle is separate from running/idle execution and frontend attachment. Paused Agents require explicit resume; closed Agents can be explicitly reopened. Closing does not complete work or delete resources. |
| Resume | Keep the logical Agent and reuse compatible native state and the applicable permitted checkpoint. Record any transformations or gaps. |
| Fresh | Create a new Agent without automatically loading prior conversation/checkpoint content. Existing records and files remain retrievable through permitted access. |
| Fork | Create an independent Agent from a selected portable checkpoint with an origin reference. The source Agent is unchanged. |
| Unexpected interruption | Reconnect only when the integration verifies the execution is still running and supports reconnection. A stopped execution requires explicit resume to start a replacement. Unknown status never launches a duplicate. |
| Delete Agent | Require its agent executions to be confirmed ended, including native agents that are idle but still alive. Reject deletion while an execution is live or its status is unknown, preserving the Agent so the caller can explicitly stop or reconcile and retry. Deletion does not initiate shutdown; closing a conversation and purging data are separate operations. |

Fresh/fork do not clone or delete workspace files. A new Agent for a different account or a
frontend change does not by itself create a checkout. Reusing an account must not expose another Agent's
private history, memory or provider-specific state.

Portable checkpoints contain optional objective/constraints, decisions, progress/evidence,
remaining work/questions, and artifact/source references. The active agent writes them
at planned handoff, explicit pause, task completion or on demand; Conduit requests and
saves them with known provenance. Failed capture continues with available permitted
context and explicit gaps, including absence of any checkpoint. That is distinct from
choosing fresh. Transcripts are retained when available; complete normalized history is
not guaranteed. Runtime recovery state and compaction summaries are not automatically
portable task checkpoints.

### Observation

Agent observation participates in the shared [Interface observability](MODEL.md#interface-observability)
contract; it is not a separate per-frontend logging system.

Selected observation plugins expose the available token usage, tool activity, transcripts and
subagent observations they support; broader instrumentation can be added over time. Attribute these
to Agent/execution identity. Usage records identify account and reporting source, and
distinguish reported counts, estimates, unknown data and observed zero. Sum only figures
known not to overlap; unresolved parent/subagent totals remain separate. Missing telemetry
is a visible integration gap. Exact cost accounting and comprehensive external coverage
remain staged work.

Frontend progress and optional trace exports are not the only evidence of completed work.
Conduit observes the operations it dispatches; external integrations declare coverage.
Retention follows the Resource contracts below. User hooks and broader interception
remain extension goals rather than assumptions about every runtime.

