# Conduit templates, connections and Pool-owned Resources

**Current-design reading note:** [SPEC.md](../SPEC.md) and [DESIGN.md](../DESIGN.md#template-defined-configuration-changes)
now define five families and templates that declare establishment plus permitted configuration
changes. Earlier architecture proposals below identify the
research context, not competing current contracts. Read the
[control-contract synthesis](CONFIGURATION.md#template-defined-changes-research-synthesis) for
how this evidence informs Interface and Conduit behavior.

Updated: 2026-10-31. The five-family template/instance model, exclusive Pool ownership,
Pool-only Resource creation and explicit open/join intents are accepted. Detailed interaction,
state and lifecycle proposals below remain for review; no schema or implementation is implied.
[DESIGN.md](../DESIGN.md#conduits) records contracts; [REVIEW.md](../REVIEW.md) groups questions.
No live login, deployment or provider mutation was performed for this research.

## Common model across the scenarios

| Family | Reusable template | Concrete instance |
| --- | --- | --- |
| Interface | User authoring, observation, creation and permitted connection capabilities. | The user-facing endpoint for instantiation and Conduit-mediated control. |
| Agent | Account/model/runtime choices, limits and activation instructions. | Durable participant with private context and executions. |
| Resource | How to create or attach a typed capability. | Deployed/attached capability belonging to exactly one Pool instance. |
| Pool | Resource provisioning choices, defaults and policies. | Creates and exclusively owns its Resource instances. |
| Conduit | Interaction, endpoint, filter and support requirements. | A live filtered flow with logical attachments and connection state, implemented by the engine/plugins. |

Template identity, instance identity, participant attachments and individual operations are
distinct. Opening a Conduit template establishes a new connection; joining selects an existing
one. Reusing a template does not implicitly choose either operation or share credentials.
Instances need not have resident processes. One authority per fact is useful without making
every fact belong to one Agent, connection or backend.

Agent and Resource templates can reference Conduit templates as declarations of connections
to establish; the enclosing instance supplies the consumer endpoint automatically. Explicit
preparation normally attempts each configured open/join under current authority and reports
the result; registration alone opens nothing. Required connection failure blocks its consumer's
completion of preparation; optional failure is reported and permits continuation. Unrelated
consumers are unaffected. Declarations default to required; optional must be explicit and saved
instance configuration materializes the default. Partial preparation retains successful
attachments for retry; explicit cancellation detaches those retained by the preparation.
Other consumers, backing Resources and outcomes survive; delivered data/external effects are
not rolled back. Retry/timeout and incomplete cleanup rules remain open.
A Conduit has instance-managed behavior in the common runtime/plugins, which perform transfers and
maintain clients or sync work. Logical consumer/peer attachments keep it live between polls and
during temporary transport failure. The last consumer/peer detaching ends the flow and closes
provider-side connections. Templates, backing Resources and retained evidence remain separate.

A connection that needs backing Resources must request them through an identified Pool.
Connection supervision and participant attachments can have different lifetimes from those
Resources. One Resource instance cannot appear in several Pools. Separate instances can
explicitly attach to the same external data or provision independent backing, according to
their contract. Cross-Pool access uses authorized paths to the owning Pools.

Conduits express interactions across the five families, including logical Resource-to-Resource
access such as search reading storage. That relationship uses the connection/filter model;
sharing a Pool alone does not establish it. Individual queries or sockets do not each require
a Conduit. The five scenarios below are examples, not a closed endpoint taxonomy.
Exact contract types, endpoint declarations, state placement and delivery semantics
remain open. Room-based behavior is one candidate, not the universal Conduit primitive.

## Walkthroughs

### 1. Expose Euphony repositories as local paths

The Pool creates or attaches repository/workspace Resource instances according to their template
establishment configuration. One-off/import syntax remains under review where an instance export
lacks creation-only settings; this does not require a separate authored operation recipe.
An access connection selects permitted Resources and operations. Agent activation
can prepare a participant's local path projection, subject to adapter and execution-mode
capabilities.

For the local-folder case, the Resource instance identifies the configured folder attachment.
Per-Agent symlinks are preparation state, not separate Resource instances. Removing one
symlink preserves that Resource, its backing folder and other Agents' views. This boundary
is accepted; exact placement, cleanup timing and other projection mechanisms remain open.

Two symlinks to the same checkout expose the same working files. Creating another symlink
does not create a separate editing environment, a read-only boundary or an isolated account.
Separate worktrees provide different working trees, although some repository administration
and references remain shared. Git documents that separation explicitly.
[Git worktree](https://git-scm.com/docs/git-worktree)

State separation (folder/symlink identity is settled; remaining details are for review):

| State | Natural responsibility to test |
| --- | --- |
| Repository and checkout identity | Resource/native workspace contract. |
| Selected paths and permitted operations | Effective Conduit access constrained by Agent limits and explicit authority dependencies. |
| Agent-local symlink/mount location | Participant preparation and attachment state. |
| Task-associated worktree | Pool owns the Resource instance; task association and native workspace management guide its use/cleanup. |
| Concurrent modification | Actual target plus operation; separate writers by default, shared editing explicitly selected. |

Removing a local projection must be distinguished from deleting its target. The existing
backing-data preservation and explicit workspace-cleanup rules remain applicable. Messages
between writers can coordinate intent, but do not themselves lock files or prevent conflicts.

### 2. Coordinate the Agents using those repositories

A dedicated agent-communication Resource group is now a candidate for direct messaging,
conversations and shared context. Agent-specific behavior can justify that distinction even
when ordinary storage backs it. Its types and division of responsibility with live Conduits
remain open; see [Resource typing](RESOURCES.md#agent-communication-as-a-domain-specific-group).

A communication connection may have several participants, shared retained exchanges, pending
requests and a routing service. Each participant can have its own identity, delivery state
and context. Shared conversation storage need not be editable shared working memory.

One Agent leaving can remove its attachment while the flow remains for other consumers/peers.
The last one leaving ends the live Conduit and closes remaining provider-side connections;
retained history belongs to a Pool-owned Resource and can outlive the flow. Later use of that
history does not by itself reopen the old instance. One-time transfer completion/detachment
and reopening/new-instance identity remain under review.

Agentwatch demonstrates the separation: its room code persists posts before delivery,
keeps live membership in memory, and uses per-room ordering independently of the lifetime
of an empty membership entry. Its explicit history-read tool pages stored messages without
consuming an agent's inbox. These mechanisms support the distinction; they do not settle
Conduit's retention, delivery, attention or topology policy. An older opening comment in
the room module says nothing is stored; the later persistence path and current README
describe the implemented history behavior.
[Room implementation](https://github.com/jxp-software/agentwatch/blob/04025a6f8e3887b8c1152ea44930e1c429b75948/crates/agentwatch-server/src/rooms.rs)
· [History read](https://github.com/jxp-software/agentwatch/blob/04025a6f8e3887b8c1152ea44930e1c429b75948/crates/agentwatch-mcp/src/tools/room_read.rs)

The engine/plugin implementing a flow must identify its authority to access Pool-owned storage.
A process serving two Agents must not silently use one participant's broader credentials
for the other's requests. Joining communication also must not confer repository access,
import private context or automatically start a model turn.

Still open: which exchanges are durable, how lost consumers are detected, who administers
the flow, how new participants join, and recovery after engine failure. "Room" remains
one possible interaction example, not the universal Conduit abstraction.

### 3. Access a 1Password account or vault

The Pool instantiates a Resource template to attach a credential service or selected vault
capabilities, and owns that attachment instance. The Conduit establishes authorized access. An Agent's attachment
resolves the chosen native account/grant and how authentication is obtained and refreshed.

Do not require every credential service to implement one login per Agent, and do not
assume reuse is safe. 1Password distinguishes desktop-app integration, manual sign-in,
service accounts and Connect. Its CLI also has authentication-source precedence. An adapter
must report the actual mode and principal, with an explicit choice of reuse versus separation.
[Sign-in modes](https://developer.1password.com/docs/cli/sign-in-manually/)
· [Service-account CLI configuration](https://developer.1password.com/docs/service-accounts/use-with-1password-cli/)

Several independently authenticated clients still access the same vault and items. Login
state, secret revisions, refresh/logout effects and provider limits are separate concerns.
The model subscription account is another identity again; a Claude account does not determine
the identity used at 1Password, AWS or Linear.

An integration could supply a secret reference or inject a credential into an authorized
subprocess without making its value ordinary model context. Record the operation and source
identity without placing credentials in reusable YAML, shared conversation or general logs.
How one connection supplies credentials to another is a dependency/authority contract to
settle, not permission implied by Pool membership.

### 4. Access AWS through its CLI

Distinguish the executable, authentication configuration and the AWS targets. Account,
IAM role, region, service and concrete deployment target affect what an operation means.
The adapter can prepare Agent-specific configuration and credential references, then
run native commands without requiring a resident process for every attachment.

AWS already distinguishes named CLI profiles from SSO-session configuration; multiple
profiles can reference one SSO session. Separate profile names alone therefore do not prove
independent login or cache ownership.
[AWS IAM Identity Center configuration](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sso.html)

"AWS access" is not inherently exclusive. Reads, deployments to different environments,
and conflicting updates to one target have different coordination needs. A managed deploy
operation might require serialization for a stack or environment while allowing unrelated
work. The concurrency boundary must refer to the underlying target across Conduits.

Accepted versus completed work also differs. CloudFormation's UpdateStack starts an update
that is subsequently inspected through provider state; returning from the initiating call
does not mean deployment completed. Persist operation identity and outcome evidence;
disconnecting an Agent must not be interpreted as cancellation of the provider operation.
[CloudFormation UpdateStack](https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStack.html)

A generic unrestricted shell adapter cannot promise to classify and serialize every AWS
effect. If a deployment wrapper is mediated, report that coverage; native paths outside it
remain outside that guarantee. This preserves the accepted native-mode and future-isolation
direction without making deployment orchestration a prerequisite for v0.

### 5. Access Linear through MCP

This expanded walkthrough tests the boundary between templates, plugins, graphs, ongoing runtime
and permitted changes. **The architecture and example controls below are proposals**, applying
the accepted five-family model. They do not select YAML fields, a plugin ABI, process topology or
new universal retry policy. No provider call, login or live mediation test was performed.

Provider facts: Linear documents a managed remote MCP endpoint at `https://mcp.linear.app/mcp`,
Streamable HTTP, interactive OAuth and token authentication. It also offers a read-only endpoint
at `https://mcp.linear.app/mcp/readonly`. These are service capabilities; they do not establish
that our proposed adapter can enforce arbitrary workspace or parameter restrictions.
[Linear MCP documentation](https://linear.app/docs/mcp)

#### The objects and their responsibilities

An Interface authors templates and asks an Euphony Pool to instantiate a Linear MCP Resource.
Two Agents open access Conduits to that Resource, selected through its owning Pool. The Interface
can separately open a management Conduit to the Resource. Opening access does not grant control
over the Resource's configuration.

| Element | Responsibility in this example |
| --- | --- |
| Resource template | Service target, constraints, establishment recipe, supported runtime and exposed operations/controls. |
| Resource instance | The concrete Pool-owned service capability and its applicable configuration/state. It need not run its own daemon. |
| Access Conduit instance | The permitted relationship, filters and per-attachment client/authentication lifetime; Agent A and B can use different permitted service identities. |
| Management Conduit instance | The Interface's permitted path for inspection and supported changes on the Resource. |
| Plugin implementation | Typed operations for authentication, MCP interaction or runtime control. One package may supply several operations; packaging is not Resource identity. |
| Graph | Ordering and value bindings for a procedure requiring several operations. Its nodes do not become permanent supervisors. |

The Pool owns the Resource; it need not relay every network request. A plugin implementation can
serve many instances without sharing their credentials or state. A common endpoint does not imply
a common OAuth grant, private cache or authenticated client. Per-attachment clients are managed
runtime state, not additional Resources merely because there are two of them.

#### Establish the service and its attachments

The Resource's establishment recipe validates its configured service target and prepares the
capability to accept supported attachments. This does not prove that every prospective consumer
is logged in. Service validation that requires authentication belongs after an identity is available.

For each Agent, the access template supplies an attachment procedure:

```text
select permitted service identity
  → authenticate or verify an existing login
  → prepare the instance-managed MCP client
  → discover capabilities in that authorization context
  → expose the permitted operations to the Agent
```

These are proposed operation boundaries, not required individual plugins. One integration may
perform several steps. Authentication may wait for a user; its result is a protected binding,
not a token embedded in exported YAML. The service identity is separate from the Agent's model
subscription. A workspace constraint must be verified by supported evidence, not inferred from
the local Resource name.

The Conduit instance manages its attachment/client from startup, including supported refresh and
reconnection. Setup finishes when its declared prerequisites are satisfied. An auth graph node
does not remain running forever to own the connection. Resource-wide status describes the shared
capability; attachment status describes the individual authorized path. One failed login need
not disable the other Agent, whereas a provider outage can affect both.

MCP defines named tools with input schemas, `tools/list` discovery and `tools/call` invocation.
Its tool list can depend on the authorization supplied with a request. Therefore discovery must
respect the authenticated context; one shared catalog is not proof that all consumers can use it.
[MCP tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools)
An ongoing logical relationship does not require an always-open socket: Streamable HTTP uses
requests and can stream responses. The adapter must negotiate what the deployed service actually
supports rather than assume it implements every feature of the cited specification.
[MCP transports](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports)

#### Invoke a provider operation

Suppose Agent A requests an issue read through the provider-native tool schema. “Read issue” here
is a description, not a verified Linear tool identifier. The proposed mediated route is:

```text
Agent A's native tool call
  → its access Conduit checks current authority, filters and inputs
  → Resource operation handler uses A's authorized attachment binding
  → MCP plugin invokes the provider operation
  → provider result and recorded outcome return through the same relationship
```

The Resource exposes the capability contract; the Conduit supplies the authorized access context.
This logical route does not require a separate network hop or process for every box. A compatible
native client configuration or adapter must actually route the call through mediation. Direct
native calls outside that route remain outside its inspection/enforcement coverage.

An ordinary pass-through call can use one typed plugin operation. It does not need a new graph
or Conduit instance per request. A procedure that checks, transforms and combines several results
could use a graph when its contract calls for that behavior. Preserve native names, schemas and
results where possible; do not invent a parallel Conduit issue workflow.

Discovery is not authorization at dispatch. A newly advertised tool must satisfy current grants
and filters; discovery alone adds no permission. A team/workspace restriction may need issue-ID
resolution or other provider-specific checks; a
tool-name filter alone cannot establish it. Unsupported restrictions must be reported. Separate
clients can still mutate the same issues and consume shared quotas; duplication provides neither
data isolation nor automatic conflict resolution. After an uncertain write, reconcile available
evidence rather than blindly replaying it.

#### Change running configuration through a management Conduit

Use a deliberately local example: the MCP Resource template exposes a **client request timeout**,
initially 30 seconds, permitted from 5 to 120 seconds. These values and the control are illustrative,
not a claim about a Linear setting. The template declares that applying a new value affects
subsequent requests; existing calls keep their original deadlines. This is one possible control
policy, not a protocol-wide rule.

The Interface requests 60 seconds through its management Conduit. The target checks the caller's
current authority, the control's bounds and transition conditions. Its change handler applies the
setting to the managed request path and reports whether 60 seconds is confirmed in use, pending,
failed or uncertain. A successful request receipt alone does not prove application. The precise
common outcome vocabulary and concurrency checks remain under review.

This change can be a single runtime/plugin operation; it does not require repeating login or
Resource establishment. A more involved permitted change could run a graph. In both cases the
instance owns the resulting behavior. The source template stays at its original defaults, so a
new Resource does not inherit a previous instance's edits. Restart persistence must be declared.

Placing this timeout on the Resource deliberately makes it shared across its applicable attachment
requests. If the intended effect were only Agent A's access, define an attachment/Conduit control
instead. Do not author the same setting in several places without an explicit contract. Agent B's
permission to read issues does not imply permission to change either control.

Contrast four actions using the same general interaction path:

| Action | What changes |
| --- | --- |
| Update a Linear issue | Provider-owned content. |
| Change the Resource's declared timeout | Supported instance configuration. |
| Detach Agent A | A connection's participation/lifecycle. |
| Edit the reusable Resource template | The recipe for future instances. |

They can share discovery, typed invocation and outcome reporting without becoming equivalent
operations. None implies permission to replace the running instance's entire definition.

#### Detach and stop without confusing ownership

Detaching A releases its relationship and the client state owned by that attachment. B remains
connected. The last logical consumer leaving an access Conduit ends that flow, while the Resource
remains registered and Pool-owned. Closing a client does not inherently revoke a reusable service
login or delete provider data.

Stopping the Resource, if its contract supports it, stops serving through that Resource and affects
all its consumers. Its runtime must report how in-flight calls drain or cancel and retain unknown
external outcomes; local cancellation does not prove a remote write was undone. It does not stop
Linear's hosted server. Deleting a registration, purging retained evidence and deleting external
issues are separate operations with separate authority.

#### The structural ruling this tests

**Recommendation for review:** templates bind named lifecycle, invocation and configuration-change
handlers to a typed plugin operation or a composed graph. Instances own ongoing state and services.
A handler identifies the procedure; it is not a sixth object family and need not be a process.
The template/type contract determines which handlers are externally callable; internal setup
helpers are not automatically exposed through an access Conduit.

This preserves small plugin contributions and reusable graphs without making every function a
Resource or every Resource a permanent process. Resolve this shared mechanism first; detailed
timeout values, queueing and transport implementation can remain type-specific. A subsequent
database walkthrough should test managed startup, migrations and recovery before finalizing the
handler schema.

## Replace "replicable versus finite" with independent dimensions

The reusable definition, client, underlying data and operation have different replication
and conflict properties. Useful contract dimensions include:

| Dimension | Questions the contract must answer |
| --- | --- |
| Endpoints and topology | Which participants/capabilities are connected? One-to-one, several participants, request/reply, publication, observation or another supported pattern? |
| Instance boundary | One shared relationship, per-Agent attachment, per-execution client, per-operation process, or a combination? What identifies a particular instance? |
| Authentication | Which principal acts at each hop? Are login, refresh, revocation and caches shared deliberately or independent? |
| Backing state | Another view of the same state, a snapshot, an independent copy, or a new managed object? How are native target identity and revisions tracked? |
| Effects and coordination | Which operations conflict, over what target, and what mechanism actually prevents conflicts? Are leases advisory or enforced? |
| Capacity | Which quotas apply per principal, target, service or runtime? How are bounded queues, rate limits and overload reported? |
| Persistence and lifetime | What survives detach, last participant leaving, execution exit, runtime restart, template deletion and explicit purge? |
| Failure and change | What can remain usable, what must be stopped, and which completed or uncertain effects require reconciliation? |

These are questions for typed contracts, not mandatory knobs on every YAML object. A
simple directory conduit should remain simple. Adapters/types can define defaults and
supported policies; users override only relevant choices. Prefer validated combinations
over an unconstrained cross-product of settings.

Resource contracts can describe native capabilities and constraints. Conduit contracts
choose supported interaction policy within those capabilities and current authority. An
integration reports the guarantees it can implement. For example, a requested single-writer
policy is not an enforced exclusion unless something controls all relevant write paths.
Kubernetes makes a similar distinction: many volume access modes participate in matching
and mounting but do not themselves enforce write protection after mounting.
[Kubernetes volume access modes](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes)

Two different Conduit names reaching the same production stack must not receive independent
"exclusive" leases and claim safety. Native target identity may be finer than a Resource
registration and shared across several registrations. Where that identity cannot be proven,
report limited coordination rather than inventing a guarantee.

## Supporting Resources and lifecycle ownership

Resource instances are created only through Pools and each has exactly one owner. If a
connection requires a conversation store, queue or other retained capability, it requests
provisioning through a selected Pool; the connection never becomes a second Resource owner.
Reusable Resource templates describe the support capability without deploying it.

Protocol bookkeeping such as participant attachments, cursors and in-flight routing state
can remain connection state. Not every cursor or message needs an individually registered
Resource. Retained content exposed as a Resource still belongs to a Pool, even if a shared
connection runtime writes it. The distinction must not become an unscoped storage path.

A collaboration Pool can own the backing conversation Resource for Agents using different
domain Pools. Participants retain their own access. Closing a connection, detaching its last
participant and retiring the backing Resource are separate lifecycle operations. Pool deletion,
possible ownership transfer and partial provisioning need explicit rules. Missing ownership
is not proof that purging the backing data is safe; existing retention and external-data
protection remain applicable.

Agent activation prepares a participant. Pool provisioning creates or attaches
Resource instances. Connection supervision operates a shared interaction service when needed.
Ordering, service identity, dependency failure and cleanup among these operations remain open.
A common template/instance pattern does not imply one universal startup procedure.

## Can there be one euphony-access Conduit?

Yes as an authoring goal, but aggregation must preserve the constituents' different meanings.
Two candidates are a composite Conduit template referencing others, or a named selection of
connection choices in Agent configuration. Both fit the five-family model; neither is yet
selected. Opening and joining constituents must remain explicit.

Prefer independent typed components and references over one giant provider-specific payload:

```text
euphony-access                    proposed grouped selection / composite
  repository access              shared checkout or selected task worktree
  coordination                   shared exchange + participant delivery state
  credential access              explicit native authentication boundary
  AWS capability                 target-specific operations; optional credential dependency
  Linear MCP                     native service + participant-specific clients/authentication
```

The grouping can make one user action convenient without requiring one shared token,
process, lifetime or failure state. We must settle dependencies, required/optional parts,
startup ordering, status, rollback limits and cleanup. For example, Linear login failure
need not remove repository access, while a deploy requiring unavailable credentials cannot
proceed. External writes already completed cannot be undone merely by failing an aggregate.

The [establishment investigation](ESTABLISHMENT.md) makes these lifecycle questions the next
review priority. It proposes typed prerequisite/output contracts, plugin-owned resumable stages
and engine coordination. Authentication waits and expiring access belong to the relevant
attachment/account; ordering or output references never substitute for a permitted Conduit.
This does not choose a composite-Conduit schema or a general workflow language.

Selecting the group must not make authority transitive: repository access does not grant
communication membership, posting a link does not grant access to its target, and possession
of an AWS credential does not imply every operation is allowed by Conduit's managed policy.
Existing narrowing-only filters and capability-tuple composition remain applicable.

## Review sequence and tests

With the five families, complete instance specs, Pool ownership and explicit open/join intents
settled, define materialization and state responsibilities/lifetimes together.
Follow with endpoint/authentication contracts,
operation coordination and composition before choosing concrete YAML fields. The existing
scenario fixture illustrates templates, explicit Pool provisioning and connection open/join
requests. Its fields are proposals, not an accepted authoring or API schema.

Use these change cases alongside the five happy paths:

1. Two Agents join one instance; then create separate instances from the same template.
   Show exactly which state is shared and private, including any shared backing data.
2. One detaches, crashes, changes account or loses permission; the other continues where valid.
3. The last consumer/peer leaves and the flow ends; later participation must follow the
   reopening/new-instance contract still to be chosen, with retained history considered separately.
4. Two different Conduits reach the same checkout, vault item, issue or deployment target.
5. A credential refresh or logout occurs while another client is active.
6. A write is accepted remotely and its local caller disappears before learning the outcome.
7. One optional component fails; a dependent required operation remains unavailable without
   falsely marking every other capability broken or claiming rollback of completed effects.
8. A template or backing Resource registration is deleted; inspect retained data,
   surviving participants and the exact cleanup action still required.

This changes the review order, not the accepted envelope, identity/update rules, private Agent context, Agent
activation, Pool ownership/access route or backing-data preservation.
Participant/connection state placement, topology, communication semantics and composition
remain proposals; exclusive Pool ownership of Resource instances is settled.
