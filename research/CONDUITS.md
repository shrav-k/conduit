# Conduit templates, connections and Pool-owned Resources

**Current-design reading note:** [SPEC.md](../SPEC.md) and [DESIGN.md](../DESIGN.md#shared-access-and-authentication)
define five template/instance families, Pool-managed shared service authentication and
Conduit-mediated access. Templates supply establishment, runtime and permitted-change contracts;
plugin operations and graphs implement handlers, while instances own ongoing state. Source-system
terminology is retained where it describes those systems; proposed syntax below is not a schema.

Updated: 2026-10-31. The five-family template/instance model, exclusive Pool ownership,
Pool-managed shared service authentication, Pool-only Resource creation, typed handlers and explicit open/join intents are accepted. Detailed interaction,
state and lifecycle proposals below remain for review; no schema or implementation is implied.
[DESIGN.md](../DESIGN.md#conduits) records contracts; [REVIEW.md](../REVIEW.md) groups questions.
No live login, deployment or provider mutation was performed for this research.

## Common model across the scenarios

| Family | Reusable template | Concrete instance |
| --- | --- | --- |
| Interface | User authoring, observation, creation and permitted connection capabilities. | The user-facing endpoint for instantiation and Conduit-mediated control. |
| Agent | Account/model/runtime choices, limits and activation instructions. | Durable participant with private context and executions. |
| Resource | How to create or attach a typed capability. | Deployed/attached capability belonging to exactly one Pool instance. |
| Pool | Resource composition, shared authentication setup, defaults and policies. | Owns its Resources and organizes their shared service-access lifecycle. |
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
during temporary transport failure. The last consumer/peer detaching ends the flow and releases
its owned connections/leases. Shared Resource-owned credentials and clients survive according to
their own lifecycle; templates, backing Resources and retained evidence remain separate.

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
The last one leaving ends the live Conduit and releases its own connection state; shared
Resource-owned services and retained history can outlive the flow. Later use of that
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
capabilities, and owns that instance. It normally establishes a supported shared service binding;
each Agent's Conduit authorizes and filters use of the capability without another provider login.
The managed capability owns authentication and refresh; the Agent owns its private context.

Do not require every credential service to implement one login per Agent, and do not
assume reuse is safe. 1Password distinguishes desktop-app integration, manual sign-in,
service accounts and Connect. Its CLI also has authentication-source precedence. An adapter
must report the actual mode and principal, with an explicit choice of reuse versus separation.
[Sign-in modes](https://developer.1password.com/docs/cli/sign-in-manually/)
· [Service-account CLI configuration](https://developer.1password.com/docs/service-accounts/use-with-1password-cli/)

Shared or separately authenticated clients may still access the same vault and items. Login
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
The Pool-owned capability can manage shared service authentication and invoke native commands
through an adapter; Conduits narrow each caller's allowed operations/targets. Agent-specific
projections are optional integration behavior, not mandatory independent sign-in. If a native
CLI path needs credential injection, expose that as a supported operation and report the coverage;
do not equate injection into unrestricted shell access with enforced per-command filtering.

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
and permitted changes. **Shared Pool authentication and separately restricted access are accepted
direction; the concrete controls and wiring remain illustrative.** They do not select YAML fields, a plugin ABI, process topology or
new universal retry policy. No provider call, login or live mediation test was performed.

Provider facts: Linear documents a managed remote MCP endpoint at `https://mcp.linear.app/mcp`,
Streamable HTTP, interactive OAuth and token authentication. It also offers a read-only endpoint
at `https://mcp.linear.app/mcp/readonly`. These are service capabilities; they do not establish
that our proposed adapter can enforce arbitrary workspace or parameter restrictions.
[Linear MCP documentation](https://linear.app/docs/mcp)

#### The objects and their responsibilities

An Interface authors templates and asks an Euphony Pool to instantiate an authentication Resource
and a Linear MCP Resource connected to it through a Conduit. The authentication Resource manages
the reusable provider login. Two Agents open individually
restricted access Conduits without separately logging into Linear. The Interface can also open a
management Conduit to the Resource. Opening access does not grant configuration control or give
the Agent the upstream credential. Per-attachment identities remain an explicit supported exception.

When authorized to delegate, the user can establish each Agent's attachment with its explicit
permitted subset in the same action. For example, the review Agent requests issue-read access;
the attachment records the actual authorized delegation. No separate Agent permission record is
required first, but a requested bundle or filter in YAML is not self-issued authority.

| Element | Responsibility in this example |
| --- | --- |
| Resource template | Service target, constraints, establishment recipe, supported runtime and exposed operations/controls. |
| Authentication Resource instance | The Pool-owned reusable identity/binding capability and coordinated login/refresh lifecycle. Its exact type and operations remain open. |
| Service Resource instance | The Pool-owned Linear MCP capability, upstream clients and service configuration. It uses the authentication Resource through a Conduit. Neither Resource needs its own daemon. |
| Access Conduit instance | Each Agent's authorized relationship, filters and local attachment state. It uses the explicitly shared binding here; a different mode can select a permitted per-attachment service identity. |
| Management Conduit instance | The Interface's permitted path for inspection and supported changes on the Resource. |
| Plugin implementation | Typed operations for authentication, MCP interaction or runtime control. One package may supply several operations; packaging is not Resource identity. |
| Graph | Ordering and value bindings for a procedure requiring several operations. Its nodes do not become permanent supervisors. |

The Pool owns both Resources; it need not relay every network request. This is Pool-managed access
implemented by owned Resources, not a reason to add a second authentication implementation to
the Pool itself. A plugin can serve many instances; endpoint equality never implicitly shares
credentials. Sharing the chosen service identity also does not require sharing a single socket or
protocol session. The integration may reuse clients or keep separate upstream clients under the
same managed identity. Those clients are runtime state, not automatically additional Resources.

#### Establish the service and its attachments

The authentication Resource establishes or verifies its supported shared provider binding. The
service Resource's recipe establishes its authorized Conduit to that capability, validates its
service target, prepares its upstream client and discovers supported operations. Authentication
may wait for a user once, or again if provider consent expires; it is
not repeated merely because another Agent attaches. A protected binding stays with the managed
Resource, not in exported YAML or the Agent's tool arguments. A workspace constraint requires
supported verification rather than inference from the local Resource name.

For each Agent, the access template supplies a different procedure:

```text
identify and authorize the Conduit caller
  → verify permission to use the Resource's shared service binding
  → resolve that attachment's permitted operations and filters
  → expose the supported provider tool surface to the Agent
```

These are proposed operation boundaries, not required individual plugins. Local caller identity
and authority are distinct from the provider identity and the Agent's model subscription. Provider
permission, Resource policy and the caller's Conduit restrictions all limit effective access.
Joining a Pool alone does not grant use of the shared login. Native-client registration/approval
may still be needed, but it need not entail another provider sign-in.

The authentication Resource manages the reusable binding/refresh; the service Resource manages
its upstream clients using that capability. Each Conduit manages its attachment and any
consumer-facing transport. Setup nodes finish after preparing that
instance-managed state. Expiration of the shared login affects its users; revoking only A's access
does not disable B or revoke the shared login. In the per-attachment identity variant, authentication
can instead be prepared for each attachment, and one failed login need not affect another.

MCP defines named tools with input schemas, `tools/list` discovery and `tools/call` invocation.
Its tool list can depend on the authorization supplied with a request. Upstream discovery uses the
selected service identity; downstream exposure is additionally narrowed for each Conduit caller.
One shared upstream catalog is not proof that all consumers can use every tool.
[MCP tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools)

The accepted Conduit boundary is a versioned discovery/invocation capability contract with the
provider's concrete tools and schemas discovered dynamically. Each permitted tool remains native
in the Agent's view; it does not need its own Conduit contract or generic model-facing wrapper.
Catalog refresh and schema evolution remain detailed integration questions, not new authority.

An ongoing logical relationship does not require an always-open socket: Streamable HTTP uses
requests and can stream responses. The adapter must negotiate what the deployed service actually
supports rather than assume it implements every feature of the cited specification.
[MCP transports](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports)

#### Invoke a provider operation

Suppose Agent A requests an issue read through the provider-native tool schema. “Read issue” here
is a description, not a verified Linear tool identifier. For an external native Agent using MCP,
the proposed route is:

```text
Agent A's native tool call
  → Conduit-managed MCP endpoint preserves the permitted Linear tool contract
  → A's access Conduit checks current authority, filters and inputs
  → Resource handler uses the explicitly shared upstream identity
  → MCP client plugin invokes Linear's real MCP server
  → provider result and recorded outcome return through the same relationship
```

In this mode the native Agent connects to a Conduit-managed MCP server, not directly to Linear's
hosted URL. Preserve Linear's operations and schemas; do not require the model to wrap them in a
generic `conduit.execute` tool. A provider-specific registration can still appear as Linear in
the client's tool surface, subject to its naming rules, while inspection makes the mediated route
explicit. No combined server for all providers is required.

A Conduit-owned Agent runtime can apply the same handler/filter boundary internally, without an
extra MCP server. A literal direct provider connection remains a separate mode: only supported,
verified native interception can give it Conduit controls. Provider-native contracts and direct
network routing are different promises. Each box above is logical, not a mandatory process/hop.

Keep the downstream caller's authentication separate from the upstream Linear grant. Conduit must
not accept a token intended only for Linear as authorization to its own HTTP MCP endpoint. Its
managed upstream client uses the properly acquired service credential; sharing authorized use of
that client is different from passing incoming bearer tokens through unchecked.
[MCP token handling](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization#token-handling)

An ordinary pass-through call can use one typed plugin operation. It does not need a new graph
or Conduit instance per request. A procedure that checks, transforms and combines several results
could use a graph when its contract calls for that behavior. Preserve native names, schemas and
results where possible; do not invent a parallel Conduit issue workflow.

Discovery is not authorization at dispatch. A newly advertised tool must satisfy current grants
and filters; discovery alone adds no permission. A team/workspace restriction may need issue-ID
resolution or other provider-specific checks; a
tool-name filter alone cannot establish it. Unsupported restrictions must be reported. Separate
clients can still mutate the same issues and consume shared quotas; duplication provides neither
data isolation nor automatic conflict resolution. Linear sees the configured shared service
principal unless its own supported delegation mechanism supplies finer attribution. Conduit
records the actual Agent/Conduit responsible; it must not claim those identities appear in the
provider audit trail. After an uncertain write, reconcile evidence rather than blindly replaying it.

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

Detaching A releases its relationship and consumer-local state. B continues using the shared
service capability. The last logical consumer leaving an access Conduit ends that flow, while
the Resource remains registered and Pool-owned. It may retain its authorized service binding for
future consumers; upstream idle-client policy is separate. Detachment does not inherently revoke
the shared login or delete provider data.

Stopping the Resource, if its contract supports it, stops serving through that Resource and affects
all its consumers. Its runtime must report how in-flight calls drain or cancel and retain unknown
external outcomes; local cancellation does not prove a remote write was undone. It does not stop
Linear's hosted server. Deleting a registration, purging retained evidence and deleting external
issues are separate operations with separate authority.

#### The common mechanism illustrated

**Working mechanism:** templates bind named lifecycle, invocation and configuration-change
handlers to a typed plugin operation or a composed graph. Instances own ongoing state and services.
A handler identifies the procedure; it is not a sixth object family and need not be a process.
The template/type contract determines which handlers are externally callable; internal setup
helpers are not automatically exposed through an access Conduit.

This preserves small plugin contributions and reusable graphs without making every function a
Resource or every Resource a permanent process. Detailed handler schemas remain open;
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

**Accepted:** reusable Conduit templates can compose other Conduits. An Agent can declare
`euphony-access` once; its instance composes repository and Linear Conduits while retaining their
identities, permissions and provider state. Setup graphs can establish the components; ongoing
attachment/lifecycle state belongs to the instances. Opening and joining remain explicit. This
adds no family or required daemon and does not transfer supporting Resources out of their Pools.

Prefer independent typed components and references over one giant provider-specific payload:

```text
euphony-access                    composite Conduit; component names illustrative
  repository access              shared checkout or selected task worktree
  coordination                   shared exchange + participant delivery state
  credential access              Pool-owned managed service binding
  AWS capability                 target-specific operations; optional credential dependency
  Linear MCP                     shared service authentication + per-caller restrictions
```

**Accepted per-consumer component participation.** Each consumer's composite attachment
tracks its participation in the components, so access, readiness and cleanup remain attributable.
These are logical attachments, not a requirement for another upstream client, provider login or
Resource per consumer. Component Conduit instances may be explicitly shared. Detaching A releases
only A's participation through the composite; B and independent attachments remain unaffected.

For the lifecycle example, let A and B use one composite with required repository access and optional
Linear access. The optional marking is an example choice; declarations otherwise default to required.
The following illustrates existing lifecycle rules with the accepted per-consumer relationships;
exact aggregate status and required/optional composition fields remain open:

| Event | Lifecycle illustration |
| --- | --- |
| A prepares while Linear awaits login | A can finish preparation if its required repository connection is ready; report its unavailable optional Linear connection. Marking Linear required would instead keep A's preparation incomplete while retaining successful attachments. |
| B joins the composite | Establish B's permitted component participation, preserving B's identity and restrictions even when component flows/clients are shared. A's existing access does not authorize B. |
| A detaches | Release A's participation through this composite. Preserve B and any independent attachments to the components; release a component flow only when its last logical consumer/peer leaves. |
| The composite loses its last consumer | End the composite and release remaining participation/leases it owns. Components still used elsewhere and Pool-owned Resources/login remain intact. Unknown cleanup is reported. |

This avoids treating a ready repository Resource as proof that an Agent's required connections
are ready. Explicit cancellation and retry would use the retained-progress rules for the affected
consumer; completed external effects are not rolled back by detachment. Exact attachment records,
component reuse selection, aggregate status and required/optional composition fields remain open.

The [establishment investigation](ESTABLISHMENT.md) supplies dependency/output and recovery cases
for typed plugin graphs. Shared authentication waits and refresh belong to the managed capability;
caller authorization/attachment state belongs to the relationship. An explicitly selected
per-consumer service identity uses its own declared boundary. Ordering or output references never
substitute for a permitted Conduit.
This does not choose a composite-Conduit schema or a general workflow language.

Selecting the group must not make authority transitive: repository access does not grant
communication membership, posting a link does not grant access to its target, and possession
of an AWS credential does not imply every operation is allowed by Conduit's managed policy.
Existing narrowing-only filters and capability-tuple composition remain applicable.

### Filter declaration and implementation

**Accepted:** use typed filter declarations with meanings defined by capability or
constraint contracts; supporting plugins implement validation and enforcement. Templates and
attachments can expose inspectable restrictions without embedding the checking procedure at each
use site. A user can implement a new contract with a small plugin, including a script or graph
where needed. Contract identity is separate from the implementation package.

| Example restriction | Declaration describes | Implementation must establish |
| --- | --- | --- |
| Repository subtree | Permitted paths and operations | Whether the actual access stays within the selected paths, using the integration's supported filesystem controls. |
| Linear issue access | Permitted native operations and workspace/team constraints | Whether the request and relevant targets satisfy those constraints; tool-name filtering alone may be insufficient. |
| Resource configuration control | Permitted values or bounds for a supported control | Whether the requested value and current transition satisfy the target contract and caller restrictions. |

These are candidate constraint meanings, not accepted schemas or verified provider support.
Filters narrow existing grants; they neither issue permission nor silently rewrite requests.
Unsupported restrictions retain the existing unavailable/reporting behavior. This authoring choice adds
no blanket interception or sandbox guarantee to native execution.

Declarative contracts are the normal authoring surface; checking procedures live in their
implementations. Exact operators, names, schemas, composition and conformance tests remain open.

### Filter composition across access paths

**Proposal for review:** require all applicable restrictions along the path used for a request
to pass. Intersect target support/limits, current caller authority and the composite/component
filters that apply to that operation. A component's wider allowance cannot override its parent's
narrower restriction. This is a rule for logical authorization, not a required sequence of network
hops, and does not treat an entire Pool's internal dependency graph as the caller's access path.

Independent authorized paths remain distinct complete choices. Preserve the target, operation,
input and other constraints together, including applicable actor-wide or target-wide exclusions.
Do not combine field-wise allowlists into permissions that no complete path authorizes:

| Available path | Complete permitted combination in this example |
| --- | --- |
| A | Read issues in Euphony. |
| B | Update issues in Demo. |

Reading Euphony can use A; updating Demo can use B. Updating Euphony cannot borrow the operation
from B and the target from A. An unrelated read-only path also does not make every other path
read-only. The actual path used must authorize the complete request; whether/how a runtime chooses
another path is a separate routing decision, not settled here.

Within one path, an Euphony-only composite and a Demo-only component have no common permitted
workspace. Report the unusable access rather than treating either declaration as an override.
Exact conflict diagnostics, filter expression syntax, Boolean alternatives within a layer and
evaluation implementation remain open. This recommendation does not prescribe a new policy engine.

## Review sequence and tests

With the five families, complete instance specs, Pool-managed shared authentication and explicit
open/join intents settled, review authentication reuse and state ownership first. Follow with
endpoint/filter contracts, lifecycle handlers and YAML representation. [REVIEW.md](../REVIEW.md)
is the single agenda; supporting examples do not establish an executable schema.

Use these change cases alongside the five happy paths:

1. Two Agents join one instance; then create separate instances from the same template.
   Show exactly which state is shared and private, including any shared backing data.
2. One detaches, crashes, changes account or loses permission; the other continues where valid.
3. The last consumer/peer leaves and the flow ends; later participation must follow the
   reopening/new-instance contract still to be chosen, with retained history considered separately.
4. Two different Conduits reach the same checkout, vault item, issue or deployment target.
5. A shared credential refresh/logout occurs while several clients are active; distinguish its
   impact from revoking just one caller's Conduit permission.
6. A write is accepted remotely and its local caller disappears before learning the outcome.
7. One optional component fails; a dependent required operation remains unavailable without
   falsely marking every other capability broken or claiming rollback of completed effects.
8. A template or backing Resource registration is deleted; inspect retained data,
   surviving participants and the exact cleanup action still required.

This changes the review order, not the accepted envelope, identity/update rules, private Agent context, Agent
activation, Pool ownership/access route or backing-data preservation.
Participant/connection state placement, topology, communication semantics and composition
remain proposals; exclusive Pool ownership of Resource instances is settled.
