# Conduits: relationships, participants and shared state

Updated: 2026-10-30. **Architecture proposal for discussion, not an accepted schema.**
Explicit support for joining an existing instance and creating a new one from a reusable
declaration is now accepted; the detailed representation and lifecycle proposals remain open.
This review uses the Euphony examples to resolve the meaning of Conduits before deciding
more individual fields. [DESIGN.md](../DESIGN.md#conduits) retains accepted boundaries;
[REVIEW.md](../REVIEW.md#architecture-pause--walk-the-five-conduits) carries the questions.
No implementation, login, deployment or provider mutation was performed for this review.

## Why the ownership question was insufficient

Choosing whether a Session or a Conduit owns "the binding" compresses several different
things into one record. A Session can own its participation in a relationship while the
relationship owns shared history, a coordinating runtime or other supporting state. A
backing Resource can outlive both. Session-owned attachments are a good candidate for
ordinary Pool access, but cannot determine every Conduit's lifecycle.

The five concepts still work:

| Concept | Responsibility in this review |
| --- | --- |
| Profile | Templates a participant's runtime/model/account defaults, limits and activation instructions; selects default Conduits. |
| Session | Identifies the participant, preserving private context, resolved configuration and its own participation state. |
| Resource | Identifies available data or a capability, its native source and its contract. Multiple routes can reach the same backing state. |
| Pool | Organizes available Resources. Membership neither duplicates data nor grants access. |
| Conduit | Defines an authorized interaction; its contract may require shared state/runtime as well as participant attachments. |

**Proposed distinction within these concepts:** declaration, live relationship, participant
attachment and individual operation. These are representations and lifecycle units, not
four additional top-level YAML kinds. They need not each require a process or database table.

- A **declaration** describes a connection's intended contract and configuration.
- A **live relationship** realizes it for particular endpoints and may have shared state.
- A **participant attachment** records one Session's resolved access, authentication
  references, local preparation and participation in that relationship.
- An **operation** reads, writes, posts, invokes, subscribes or otherwise acts through it.
  Its outcome can outlive the attachment or process that initiated it.

**Accepted direction:** explicitly support both joining an existing relationship instance
and creating a new instance from a reusable declaration. For example, four Sessions can
join the same Euphony discussion; another experiment can create a separate discussion from
the same declaration. Relationship reuse does not imply credential-cache sharing, and a
new relationship does not imply new backing data. Exact instance addressing, repeated-create
behavior, configuration binding and YAML syntax remain open.

Conduits express an extensible range of interactions across the five concepts. Pool access
and Session communication are examples rather than a closed endpoint taxonomy. Each typed
contract still needs meaningful endpoint roles, operations and supported guarantees.

## Walkthroughs

### 1. Expose Euphony repositories as local paths

The Pool references repository/workspace Resources. An access Conduit selects permitted
Resources and operations. Profile activation can prepare a Session's local path projection,
subject to adapter and execution-mode capabilities.

Two symlinks to the same checkout expose the same working files. Creating another symlink
does not create a separate editing environment, a read-only boundary or an isolated account.
Separate worktrees provide different working trees, although some repository administration
and references remain shared. Git documents that separation explicitly.
[Git worktree](https://git-scm.com/docs/git-worktree)

Proposed state separation:

| State | Natural responsibility to test |
| --- | --- |
| Repository and checkout identity | Resource/native workspace contract. |
| Selected paths and permitted operations | Effective Conduit access constrained by Profile limits. |
| Session-local symlink/mount location | Participant preparation and attachment state. |
| Task-owned worktree | Managed workspace lifecycle, already separate from account or Session ownership. |
| Concurrent modification | Actual target plus operation; separate writers by default, shared editing explicitly selected. |

Removing a local projection must be distinguished from deleting its target. The existing
backing-data preservation and explicit workspace-cleanup rules remain applicable. Messages
between writers can coordinate intent, but do not themselves lock files or prevent conflicts.

### 2. Coordinate the Sessions using those repositories

A communication Conduit may have several participants, shared retained exchanges, pending
requests and a routing service. Each participant can have its own identity, delivery state
and context. Shared conversation storage need not be editable shared working memory.

For a discussion contract, one Session leaving might remove only its live attachment;
the relationship and retained history could remain for others and later participants.
For an ephemeral call, completion might release all transient runtime state while retaining
only an operation result. These are different contracts, not one universal cleanup rule.

Agentwatch demonstrates the separation: its room code persists posts before delivery,
keeps live membership in memory, and uses per-room ordering independently of the lifetime
of an empty membership entry. Its explicit history-read tool pages stored messages without
consuming an agent's inbox. These mechanisms support the distinction; they do not settle
Conduit's retention, delivery, attention or topology policy. An older opening comment in
the room module says nothing is stored; the later persistence path and current README
describe the implemented history behavior.
[Room implementation](https://github.com/jxp-software/agentwatch/blob/04025a6f8e3887b8c1152ea44930e1c429b75948/crates/agentwatch-server/src/rooms.rs)
· [History read](https://github.com/jxp-software/agentwatch/blob/04025a6f8e3887b8c1152ea44930e1c429b75948/crates/agentwatch-mcp/src/tools/room_read.rs)

A shared runtime must identify what authority it uses to read/write backing storage.
A process serving two Sessions must not silently use one participant's broader credentials
for the other's requests. Joining communication also must not confer repository access,
import private context or automatically start a model turn.

Still open: which exchanges are durable, what keeps a relationship alive, who administers
it, how new participants join, and which state survives loss of the runtime. "Room" remains
one possible interaction example, not the universal Conduit abstraction.

### 3. Access a 1Password account or vault

The Resource exposes a credential service or selected vault capabilities; the Pool makes
that Resource selectable. The Conduit establishes authorized access. A Session's attachment
resolves the chosen native account/grant and how authentication is obtained and refreshed.

Do not require every credential service to implement one login per Session, and do not
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
The adapter can prepare Session-specific configuration and credential references, then
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
disconnecting a Session must not be interpreted as cancellation of the provider operation.
[CloudFormation UpdateStack](https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStack.html)

A generic unrestricted shell adapter cannot promise to classify and serialize every AWS
effect. If a deployment wrapper is mediated, report that coverage; native paths outside it
remain outside that guarantee. This preserves the accepted native-mode and future-isolation
direction without making deployment orchestration a prerequisite for v0.

### 5. Access Linear through MCP

One Resource can describe the native Linear MCP service while different Sessions establish
their own clients and authorized attachments. A common endpoint does not imply a common
OAuth grant, private cache or transport session. Linear hosts its remote server and supports
OAuth setup as well as token-based authentication; it also documents read-only access.
[Linear MCP](https://linear.app/docs/mcp)

All those clients can still reach the same issues, projects and comments. Duplicating a
client does not duplicate the data or remove mutation conflicts and rate limits. Preserve
native schemas, source identity and provider outcomes rather than inventing a parallel
Conduit issue workflow. Local mediation and tool filtering must report what is actually
enforced, as already required by the provider-native integration direction.

If one Session's authentication fails, another attachment may remain usable. Conversely,
a provider outage can affect every client. These need different status and recovery paths;
a single `ready` flag on an aggregate cannot explain both.

## Replace "replicable versus finite" with independent dimensions

The reusable definition, client, underlying data and operation have different replication
and conflict properties. Useful contract dimensions include:

| Dimension | Questions the contract must answer |
| --- | --- |
| Endpoints and topology | Which participants/capabilities are connected? One-to-one, several participants, request/reply, publication, observation or another supported pattern? |
| Instance boundary | One shared relationship, per-Session attachment, per-execution client, per-operation process, or a combination? What identifies a particular instance? |
| Authentication | Which principal acts at each hop? Are login, refresh, revocation and caches shared deliberately or independent? |
| Backing state | Another view of the same state, a snapshot, an independent copy, or a new managed object? How are native target identity and revisions tracked? |
| Effects and coordination | Which operations conflict, over what target, and what mechanism actually prevents conflicts? Are leases advisory or enforced? |
| Capacity | Which quotas apply per principal, target, service or runtime? How are bounded queues, rate limits and overload reported? |
| Persistence and lifetime | What survives detach, last participant leaving, execution exit, runtime restart, declaration deletion and explicit purge? |
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

A Conduit should be allowed to depend on, and where authorized provision, backing Resources.
That does not mean every cursor or temporary buffer needs a Resource registration.
Proposed boundary:

- Protocol bookkeeping such as membership, current cursors and in-flight routing state
  can remain system-managed relationship/attachment state.
- Independently retained or reusable content, such as searchable exchange history, can use
  a Resource contract. Its storage backend need not be the same as the runtime process.
- A backing Resource may already exist or be created for a relationship. Record which
  lifecycle is managed; using it does not imply permission to destroy it.
- If Sessions access that Resource as data, preserve the Pool→Conduit access route. Merely
  backing a relationship must not make it publicly discoverable or expose raw storage.

The unresolved boundary is how internal service authority and backing-Resource registrations
are expressed: explicit support Pools, generated memberships, or another representation
within the five concepts. Ordinary protocol replies should not require inventing a new
Pool for each message. Equally, "internal state" cannot become a hidden route around data
access rules. Define this once with communication history and credential-service examples.

Profile activation still prepares a participant to use available capabilities. Starting a
shared routing service or provisioning its history store is infrastructure lifecycle for
the relationship. A Profile may request the connection; which component supervises that
service, with what authority and lifetime, is a separate contract still to be decided.

## Can there be one euphony-access Conduit?

Yes as an authoring goal, but aggregation must preserve the constituents' different meanings.
Two candidates are a composite Conduit referencing others, or a named selection of separate
declarations in an existing Profile/Session field. Neither requires a sixth core kind.

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

Selecting the group must not make authority transitive: repository access does not grant
communication membership, posting a link does not grant access to its target, and possession
of an AWS credential does not imply every operation is allowed by Conduit's managed policy.
Existing narrowing-only filters and capability-tuple composition remain applicable.

## Review sequence and tests

With explicit join-existing and create-new intents settled, next define instance identity
and state responsibilities/lifetimes together. Follow with endpoint/authentication contracts,
operation coordination and composition before choosing concrete YAML fields. The existing
fixture illustrates access and a candidate review request; it does not yet express the
accepted join/create distinction or shared runtime lifecycles.

Use these change cases alongside the five happy paths:

1. Two Sessions join one instance; then create separate instances from the same declaration.
   Show exactly which state is shared and private, including any shared backing data.
2. One detaches, crashes, changes account or loses permission; the other continues where valid.
3. The last participant leaves; then a new participant arrives and the runtime restarts.
4. Two different Conduits reach the same checkout, vault item, issue or deployment target.
5. A credential refresh or logout occurs while another client is active.
6. A write is accepted remotely and its local caller disappears before learning the outcome.
7. One optional component fails; a dependent required operation remains unavailable without
   falsely marking every other capability broken or claiming rollback of completed effects.
8. A declaration or backing Resource registration is deleted; inspect retained data,
   surviving participants and the exact cleanup action still required.

This changes the review order, not the accepted five-object envelope, identity/update rules,
private Session context, Profile activation, Pool access route or backing-data preservation.
Specific state ownership, topology, communication semantics and composition remain proposals.
