# Cloud management patterns and Conduit's authority boundary

Research date: 2026-11-01. **Subsequent I1 ruling: shared policy with instance-owned state is
accepted, alongside separate management and execution services.** V0 subsequently adopted no
offline grants: retain supervision/dispatched outcomes, but block new managed requests without
verifiable permission. The first proof now uses management and two independent executor processes
on one machine, with controlled connection failures; two-machine validation precedes remote-support claims.
This report preserves the authority comparison and its rationale
using AWS, Google Cloud and Azure, with concrete control and recovery examples. It extends
[DISTRIBUTED_RUNTIME](DISTRIBUTED_RUNTIME.md), rather than replacing its process-supervision and
distributed-systems evidence. The [implementation review](../proposals/implementation-v0/REVIEW.md#i1--independent-execution-and-management)
records the accepted authority, outage and first-proof rulings.

The [Interface/policy follow-up](#9-interface-administration-and-independent-policy) examines the
separate I4 question of whether the root Interface owns policy or administers independent policy.
The user subsequently **accepted independent policy**, administered by authorized Interfaces with
broad default root administration. Policy representation remains open; the follow-up's Resource
and storage suggestions are not additional rulings. I1's service/state authority boundary remains.

Method: read primary public architecture, extension and API documentation. No cloud accounts,
credentials or deployments were used, and no failure behavior was tested. Public API contracts
do not reveal every internal storage or consensus mechanism. Provider facts below are sourced;
the proposed Conduit arrangements and examples are our analysis. Exact syntax stays deferred.

## 1. What the comparison changes

The original question coupled two choices: where instance configuration lives and where access
policy lives. They can be separated. A common management surface can route to independently
authoritative instance services while maintaining common registration and access rules.

Three candidates are worth comparing:

| Candidate | Registered templates, naming and identity | Instance configuration and controls | Access rules | Main cost |
| --- | --- | --- | --- | --- |
| A — Management owns configuration | Shared management authority | Management owns configuration/revisions; executors retain execution receipts and progress | Shared policy authority | Confirmed effects and canonical configuration must be reconciled across services. |
| B — Object services own configuration and policy | Common discovery/naming can remain | The designated object service owns configuration/revisions and execution records | Object services own their policies within an agreed governance model | Composing and withdrawing authority across services needs additional rules. |
| C — Shared policy, instance-owned state | Shared management authority | One designated service owns an instance's resolved definition, confirmed settings/revision and managed operation records | Shared policy authority; enforcement happens on the selected path | Authoritative object reads and an aggregate inventory must be distinguished; lifecycle/naming changes still need coordination. |

**Accepted direction: C.** It combines the independence the user wants with a common access
model. The user selected this after reviewing the comparison; alternatives remain here as rationale.
Under C, the designated service could be an executor for active instances and management for
passive objects. One service can own many instances. A plugin handler does not become an authority
merely because it implements an operation.

All three can preserve the five families, one logical registry, stable IDs, one Pool owner per
Resource, fixed instance definitions, named controls and independently accessible MCP capabilities.
None requires a daemon per object, permission to operate during a partition, or automatic failover.

## 2. What the cloud platforms actually establish

### AWS: keep existing activity independent; make recovery a contract

AWS describes *static stability*: existing provisioned activity should survive impairment of the
control APIs used to create or change it. Its EC2 example separates the running instance from
those APIs. This is a design principle with service-specific boundaries, not proof that every
operation or authorization check can run disconnected. [AWS static stability](https://docs.aws.amazon.com/whitepapers/latest/aws-fault-isolation-boundaries/static-stability.html)

Cloud Control API exposes asynchronous resource requests through progress records and request
tokens. Cancellation stops further handler calls but can leave partial changes; it is not rollback.
Its request records expire after seven days. These are useful examples of separating admission,
progress and completion, and of making retention explicit. [Cloud Control requests](https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html)

CloudFormation extension schemas describe properties, identifiers, handlers and property behavior.
Its progress contract can carry continuation context between handler invocations. This supports
small lifecycle implementations with inspectable contracts; it does not require a plugin to own
the complete orchestrator. [Resource type schema](https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html),
[ProgressEvent contract](https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-test-progressevent.html)

EC2 documents operation-specific idempotency: supported requests can reuse a client token with
matching parameters; incompatible reuse fails. A token's existence is therefore less important
than the provider's actual deduplication contract. [EC2 idempotency](https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html)

**Conduit implication:** preserve supervised processes and local recovery evidence independently
of the management frontend. Describe retry, inspection and partial effects in plugin contracts.
Do not copy CloudFormation's fixed CRUD surface, automatic reconciliation assumptions or retention
limits into Conduit. Our named controls and explicit retention rules remain intact.

### Google Cloud: management, request admission and traffic are different responsibilities

Google Service Infrastructure distinguishes management of service configuration, request control
such as admission/rate limiting, and data traffic. The service integrates admission checks and
telemetry reporting with independently running service code. Configuration management and request
checking are distinct APIs. This is a documented integration architecture, not a claim that every
Google Cloud product has identical internals. [Service Infrastructure architecture](https://docs.cloud.google.com/service-infrastructure/docs/overview),
[Service Control overview](https://docs.cloud.google.com/service-infrastructure/docs/service-control/reference)

Google's API guidance separates long-running operation handles from final results and allows typed
progress metadata. It separately describes freshness checks with ETags and duplicate-request
handling with request IDs. A response that an operation exists is not the operation's successful
result. [AIP-151](https://google.aip.dev/151), [AIP-154](https://google.aip.dev/154),
[AIP-155](https://google.aip.dev/155)

**Conduit implication:** distinguish policy authorship, enforcement, execution and observation even
if the first deployment combines some of their code. Preserve an operation's identity across MCP
connection loss. MCP request correlation alone does not establish durable completion, permission
freshness or safe repetition. Use the accepted Conduit profile and adapters for those semantics;
do not introduce another public invocation protocol.

### Azure: a common entry point can lead to service-owned state

Azure Resource Manager provides a common management API and forwards authorized requests to the
appropriate provider. Data-plane operations use service endpoints and can have different
credentials and enforcement mechanisms. A management restriction does not necessarily constrain
all ways of using the backing data. [Resource Manager overview](https://learn.microsoft.com/en-us/azure/azure-resource-manager/management/overview),
[Control and data planes](https://learn.microsoft.com/en-us/azure/azure-resource-manager/management/control-plane-and-data-plane)

Azure explicitly identifies resource providers as the source of resource state and Resource Graph
as an asynchronously updated index. Its guidance recommends provider verification when stale
inventory could make an action unsafe. This is particularly relevant to the Interface: aggregated
visibility and authoritative state need not have the same freshness. [Resource Graph query strategy](https://learn.microsoft.com/en-us/azure/governance/resource-graph/choose-query-strategy)

Azure asynchronous operations expose a status URL and polling guidance. Operation-specific
progress can accompany shared terminal status conventions, and status inspection itself has
permission requirements. [Tracking asynchronous operations](https://learn.microsoft.com/en-us/azure/azure-resource-manager/management/async-operations)

**Conduit implication:** one Interface and logical registry can present independently owned
instances. Views should identify their source and freshness; changes should reach the authority
that checks the instance's current revision. A shared management entry point does not imply one
payload proxy, database or executor. We should not adopt Azure's full resource hierarchy,
subscription model or broad management/data-plane permission split as new Conduit concepts.

### All three: policy distribution has observable limits

AWS IAM documents propagation and caching delays. Google IAM explicitly describes eventual
consistency for access changes. Azure RBAC documents delays that vary with assignment and identity
conditions. Their guarantees are not interchangeable and none should be imported as Conduit's
default revocation policy. [AWS IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot.html#troubleshoot_general_eventual-consistency),
[Google IAM](https://docs.cloud.google.com/iam/docs/access-change-propagation),
[Azure RBAC](https://learn.microsoft.com/en-us/azure/role-based-access-control/troubleshooting#symptom---role-assignment-changes-are-not-being-detected)

The relevant lesson is to state the consistency contract. Moving configuration ownership to an
executor does not tell that executor about a revocation it cannot receive. Conduit-managed
restrictions and upstream provider permissions must both hold; broad shared service credentials
cannot substitute for the requesting Agent's managed grant.

## 3. Walkthrough: a setting changes, then the reply is lost

Suppose a Pool-owned search Resource has confirmed `limit = 10` at configuration revision R7.
An Interface or authorized Agent invokes its existing control to set 15, using R7. These labels
illustrate semantics; they are not proposed YAML or new public status enums.

### A — Management is the configuration authority

1. Management checks current authority and R7, records operation K and reserves conflicting work.
2. The executor records K, invokes the handler and confirms that 15 was applied.
3. The executor records that receipt. Its reply to management is lost.
4. Management still holds 10/R7 as its last confirmed configuration, with K unresolved. It must
   not present 10 as verified current reality or allow another conflicting update as if K failed.
5. On reconnection, management reconciles K with the executor, commits 15/R8 and publishes the
   result. It does not call the handler again merely to obtain a fresh reply.

This is viable and keeps authoritative configuration inspection simple when management is healthy.
It needs an explicit receipt-to-configuration recovery path and can leave confirmed executor work
waiting for a management commit. The executor receipt and registry update are separate durable
facts, not a cross-service transaction.

### B — The object's service owns configuration and policy

1. The request reaches the responsible service, which checks its authoritative policy and R7.
2. It records K, applies the control and commits 15/R8 with its result.
3. Losing the reply leaves the caller uncertain, but querying that service can recover K and R8.
4. The aggregate registry later updates its view. It cannot overwrite R8 from an older snapshot.

This keeps configuration changes close to their effects. However, an Agent-wide withdrawal, a
Pool restriction and a Resource-specific rule may now have different authorities. We must define
how they compose and propagate. Object autonomy alone does not make those decisions safe offline.

### C — The instance owns state; access policy remains shared

1. The request reaches the designated instance service through an authorized Conduit. The service
   checks current governing access under the agreed policy contract and checks R7 locally.
2. It records K, invokes the handler and, once application is confirmed, commits 15/R8 and the
   outcome in its authority store.
3. Management or the observation link can disappear without preventing that local outcome commit.
4. The Interface recovers the result from the instance service when reachable. Its aggregate view
   may still show 10/R7, explicitly marked as an older observation, until synchronized.

C places confirmed settings and effect evidence together while retaining shared access policy.
It does **not** promise that a new request can be admitted with management/policy unavailable.
It improves outcome recovery even if new controls require online authorization.

For all candidates, if the handler changed an external service but crashed **before recording a
confirming result**, application is uncertain. Query the provider or use its supported deduplication
contract. Local transactions cannot encompass arbitrary remote writes. An external change made
outside Conduit can also diverge from its last confirmed settings; report or reconcile that
through supported controls rather than silently declaring the stored value current.

## 4. Walkthrough: startup, migration and management loss

A database Resource's setup graph starts its declared instance-managed runtime, then applies a
migration. Management becomes unavailable after the database starts.

| Event | What is knowable | Required distinction |
| --- | --- | --- |
| Management connection drops | Executor may still supervise the database | Loss of observation does not establish process exit. |
| Migration was dispatched; response is lost | Provider may have committed it | An unresolved effect is not a failed migration safe to repeat. |
| Executor retained a confirmed result | That executor can provide its evidence | Aggregation failure must not erase the authoritative receipt. |
| Next setup node has not started | It is new work needing admission | Whether it may start depends on outage/authority policy, not graph ordering alone. |
| Executor also fails | Process may be ended, surviving or unverifiable | Reconnect only with verified identity; do not launch a duplicate from silence. |

In A, the executor reports progress and management reconciles configuration when available. In B
or C, the owning service can additionally retain canonical instance settings. All preserve the
accepted rule that setup failure does not automatically destroy the database or its data.
“Keep a process supervised,” “finish already admitted work,” and “start a new effect” require
separate analysis. A bounded external database operation also differs from an Agent turn capable
of issuing further tool calls; process survival cannot imply unlimited authorization for the turn.

## 5. Walkthrough: shared Linear access and policy withdrawal

A Pool owns authentication and Linear MCP Resources. Agents A and B have different filtered
Conduits to the shared service. A's write access is withdrawn while the executor cannot reach
the policy authority.

- Retaining the upstream login is a custody/lifecycle choice; it does not preserve A's grant.
- The provider may still accept the shared credential, but that says nothing about A's Conduit.
- A disconnected enforcer cannot discover a withdrawal by consulting its old cached permission.
- B's independent access and unrelated Resources are separate from A's path.

The user selected no offline grants for v0 after the comparison below. Preserve supervision and
already-dispatched outcomes, but require verifiable current permission for new managed requests.
Show affected requests as pending or unavailable under their contracts. Bounded offline delegation
is deferred rather than inferred from live connections or shared credentials.

Revocation also cannot undo an already completed call or erase context already delivered to a
native Agent. This preserves the existing managed-access versus actual isolation distinction.

### Concrete outage comparison before ruling

These examples were used to review outage policy; the user subsequently chose no offline grants
for v0. The walkthrough explains that ruling and the deferred alternative, not tested behavior.

Assume an Euphony Pool owns a worktree, shared authentication/Linear access and message-history
Resources. Two Agents use independently filtered Conduits. Management supplies registration and
shared policy; execution services supervise the instances and retain their authoritative state.
The Interface presents their observations. These are logical responsibilities: registration and
authorization might share a process initially, so some failures below could occur together.

| Failure | Concrete situation | Consequence and remaining policy choice |
| --- | --- | --- |
| Frontend disconnect | The user closes the terminal while an Agent works. | Accepted detach semantics preserve instance-owned work where supported. No loss of authorization is implied. A reopened frontend retrieves retained observations; it does not start replacement work. |
| Registration/aggregate view unavailable; authorization still reachable | Template editing and the global inventory fail, but an existing Agent can reach its Resource and current policy. | Existing permitted capability use need not wait for the aggregate view. Creation/naming operations needing unavailable registration wait. This conditional example does not promise independently deployed policy in v0. |
| Authorization unreachable; instances and services alive | Linear would accept its shared credential, but Conduit cannot establish this Agent's current permission. | In v0, hold or reject the new managed request as temporarily unavailable. A future explicit offline grant could admit only its expressly covered effects. A live Conduit or credential alone is insufficient. |
| Linear unavailable; Conduit authorization healthy | A provider connection breaks while worktree and history remain available. | Independent permitted work can continue. Supported reconnection retains the same route/account; an uncertain Linear write requires reconciliation, not replay. This is dependency recovery rather than permission caching. |
| One executor lost | Executor A becomes unreachable while B still owns other usable Resources. | Preserve B's independent work. A's processes may be ended or unverified; do not substitute new execution from silence. Recovery follows verified reattachment/explicit resume, not an assumed permission to fail over. |

For the authorization-outage case, use one specific timeline:

1. An Agent requests a permitted Linear update and it is dispatched.
2. The policy service becomes unreachable while Linear processes that update.
3. Linear returns success. Under the accepted baseline, the instance service retains the outcome;
   a policy outage does not turn success into failure or trigger another write.
4. The Agent requests a **second** update. Without verifiable current permission, this request
   cannot be dispatched in v0. The same reasoning applies to protected
   reads and message-history access; “read-only” does not mean permission-free.
5. On reconnection, current policy governs subsequent dispatch and result delivery. If access was
   withdrawn, the queued or retried request is denied. A previously stored result can remain
   retained even when the caller is no longer permitted to retrieve it.

“Hold” describes a dispatch restriction, not a requirement to queue indefinitely. A capability
may report temporary unavailability for caller retry or support pending work under its contract.
This discussion does not select automatic queue replay, indefinite retention of pending requests
or a new mandatory status enumeration. Previously uncertain effects retain their original
operation/recovery identity rather than being retried as new work.

The deferred alternative can be illustrated by a grant with a hypothetical five-minute validity period.
It permits a specified Agent/path/operation set while policy contact is lost. If permission is
withdrawn after one minute but that withdrawal cannot reach the enforcer, permitted calls may
continue until expiry or until withdrawal is received. The duration is illustrative, not a
proposed default. A grant also needs a defined target and restrictions; it cannot allow arbitrary
new connections or widening privileges. This trades a known revocation delay for availability.

The admission unit matters too. An already-dispatched migration may contain several implementation
steps within its permitted operation. It need not stop between every SQL statement solely because
management is unavailable. Starting an Agent, however, does not by itself authorize every future
tool call or graph node. A broader permission to continue a run would need its own explicit bounds.
Retaining a native Agent process is also not proof that Conduit mediates or can block every action
that process can take. Enforcement coverage remains visible under either policy.

The accepted v0 baseline therefore preserves supervision and outcome recording while admitting
new managed effects only with verifiable permission. Its availability cost is real: if shared
authorization runs in management and that process fails, affected new capability calls wait.
The accepted state-ownership split removes management from canonical instance outcome commits;
it does not automatically remove management from the authorization path. Bounded offline grants
are deferred, not a hidden consequence of keeping an instance alive.

## 6. What authors should have to supply

Cloud extension patterns reinforce our accepted direction: publish typed contracts alongside an
implementation binding and keep orchestration machinery outside ordinary plugin code.

| Contribution | Author supplies | Conduit or a reusable adapter supplies |
| --- | --- | --- |
| Pure checksum operation | Input/output schema and a handler | MCP transport/correlation and attributable invocation evidence |
| Directory establishment | Create/attach semantics, result verification and recovery limits | Stable instance/operation association and valid-progress tracking |
| Database Resource | Runtime contract, controls, readiness and supported effect inspection | Instance supervision, target coordination and graph scheduling |
| Existing Linear MCP service | Compatible contracts/binding, supported restrictions and provider limitations | Native catalog exposure, managed path attribution and shared authentication integration |

Authors should not have to implement a registry, permission-replication system, distributed lock
manager or generic recovery server for each plugin. Existing code can fulfill compatible behavior
through an adapter; a declaration cannot invent an absent provider capability. The protocol should
allow explicit uncertainty and unsupported recovery, rather than requiring fictional exactly-once
semantics to register a useful integration.

Before finalizing syntax, the conceptual contract needs to identify who accepts work, who can
confirm the result, how a caller retrieves it after transport loss, what state a retry may reuse,
and what access is checked. These requirements can be implemented by lightweight shared tooling;
they do not imply a large SDK or an operation manifest authored by the user.

## 7. Consequences of the accepted authority split

The accepted arrangement has these implementation consequences; it does not settle their exact mechanics:

1. **One logical registry, distributed facts.** The registry remains the naming/registration surface.
   An inventory row can be a cached observation. A current instance read/export may need its owner,
   and unavailable owners produce a stated gap rather than an invented current export.
2. **One configuration authority per instance.** Keep one authoritative `spec.config` and revision;
   management's copy is a view. There are no independently writable desired/applied maps.
3. **Whole-object consistency still matters.** Registration, renames, retirement and ownership
   changes can involve both the shared directory and the instance owner. Their ID/version checks
   and reservations need coordination. Do not quietly introduce separate, weaker revision checks
   for metadata or promise atomic multi-object updates. Exact mechanics remain implementation review work.
4. **Execution and authority are distinct roles.** An assigned executor may host the instance
   authority in v0. Restarting a worker does not transfer that authority. Unknown owner status must
   not trigger a second writer; automatic migration/failover remains out of scope.
5. **Observation is independently recoverable.** The Interface combines sources, identifies
   freshness and missing coverage, and can reconnect to retained results. A diagnostic export
   cannot become the only record of successful application.
6. **Provider truth has limits.** For existing external MCP services, Conduit owns its client
   Resource configuration and records what the provider confirms. It does not own the provider's
   internal database or gain stronger recovery guarantees by wrapping it.

A remains a useful comparison for the cost of central configuration commits. B illustrates the
additional complexity of decentralizing policy. The user selected C for Conduit; that selection
is a design ruling, not a claim that a cloud reference makes one arrangement mandatory.

## 8. Evidence for experiment 002

Keep the accepted separate management/execution layout and synthetic plugin/template authoring
proof. Test the selected instance-state authority at these fault boundaries:

- Apply the search setting, retain the responsible service's result, and sever its response to
  the caller/aggregator. Inspect the canonical revision separately from the cached view.
- Crash between an external effect and its local receipt; demonstrate recovery without blind replay.
- Submit two changes from R7 through different Interface/Agent paths; one authority coordinates
  them, and a pending or uncertain first change cannot be bypassed through another endpoint.
- Keep the aggregate inventory stale, then reconnect; older observations cannot overwrite a newer
  authoritative revision or be passed off as a globally current snapshot.
- Keep execution alive while breaking policy reachability. Record separately what is supervised,
  what was already admitted and what new work is held by the accepted no-offline-grants policy.
- Ensure the recovered result remains available through an authorized standard MCP client without
  requiring an aggregate frontend or a new plugin-specific recovery server.

These are planned tests, not observed results. The accepted first experiment runs management
and two independent executors as separate processes on one machine, with controlled connection
failures. It need not implement all three authority models or deploy on AWS/GCP/Azure. Two-machine
validation follows before remote support is claimed. Storage technology remains I4; schema spellings
and MCP profile fields remain I7.

## 9. Interface administration and independent policy

Follow-up researched 2026-11-01. The question is logical policy responsibility, not the storage
engine: can several Interfaces administer the same durable policy under their own authority,
while the root has broad default administration? The subsequently accepted direction is **yes,
with policy independent of any particular Interface**. The comparison below explains the research
behind that ruling, rather than requiring Conduit to reproduce cloud IAM. Representation and
initial persistence remain under review.

### Where cloud policy lives

| Platform | Documented policy relationship | Administrative access | Conduit lesson |
| --- | --- | --- | --- |
| AWS | Customer-managed IAM policies are reusable policy objects; inline policies belong to one identity and are deleted with it. Resource-based policies attach to the governed resource. | Principals act under policy-defined permissions. Policy attachment is distinct from the console or other client used to author it. | Independence from an administrative Interface does not require every policy to be a standalone object or independent of its governed target. |
| Google Cloud | An allow policy attaches to a resource; bindings associate principals, roles and optional conditions. Applicable ancestor policies can affect descendants. | Changing access requires the relevant resource's policy-management permissions; console, CLI and API operate on those policies. | Multiple administrators can share one authoritative policy without giving each administrator a private policy copy. |
| Azure | A role assignment records a principal, role definition and scope, with its own assignment identity. | Assigning roles requires the corresponding authorization permission at the relevant scope. | Make administrative reach explicit; a client's ability to inspect an object is distinct from its ability to grant access to it. |

Sources: [AWS managed/inline policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_managed-vs-inline.html),
[AWS identity/resource policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_identity-vs-resource.html),
[Google allow policies](https://docs.cloud.google.com/iam/docs/allow-policies),
[Google access administration](https://docs.cloud.google.com/iam/docs/granting-changing-revoking-access),
[Azure role assignments](https://learn.microsoft.com/en-us/azure/role-based-access-control/role-assignments),
[Azure assignment permissions](https://learn.microsoft.com/en-us/azure/role-based-access-control/role-assignments-portal).

The shared pattern is policy associated with governed identities/resources/scopes and administered
by authorized principals. It is not policy owned by the UI instance that last changed it. A Conduit
Interface is more active than a cloud console—it may direct setup and repair—but those duties do
not by themselves require policy ownership. Shared authority also need not mean one process or a
network call through the root Interface for every operation.

### Root authority is not one universal cloud concept

AWS's root is an account sign-in identity. Organizations service control policies can constrain a
member account's root, and do not affect the management account. SCPs constrain permissions rather
than granting them. This supports retaining Conduit's distinction between grants and narrowing
requirements, without importing AWS's complete evaluation rules.
[AWS root identity](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_root-user.html),
[SCP scope and effects](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scps.html)

Google distinguishes Workspace/Cloud Identity super administrators from Google Cloud Organization
Administrators. Its organization setup guide separates assigning IAM roles and inspecting the
hierarchy from other actions such as creating projects/folders, which require additional roles.
Administrative ability to obtain or grant access is not identical to currently holding every
operational permission. [Google organization administration](https://docs.cloud.google.com/resource-manager/docs/creating-managing-organization)

Azure separates Microsoft Entra administration from Azure resource access. An Entra Global
Administrator can elevate to User Access Administrator at root scope, enabling visibility and
access assignment across the tenant's subscriptions and management groups. The role is not a
universal grant of all service operations.
[Azure elevation and authority boundaries](https://learn.microsoft.com/en-us/azure/role-based-access-control/elevate-access-global-admin)

**Conduit recommendation:** retain the simple owner-operated root with broad managed-system
administration by default. Express this as verified authority, not a magical property of the name
`root` or a directive in a template. It can administer all supported Conduit policy and lifecycle
operations without thereby acquiring an external provider's missing credentials or permissions.
Existing fixed-definition, revision and outcome contracts still apply. No cloud-style organization
tree, enterprise enrollment flow or hierarchy of administrator roles is required for v0.

### Lifecycle example: two Interfaces, one policy authority

Use the existing system Pool's management Resource to expose policy management. Its responsible
service retains the policy facts. Individual grants are records, not necessarily new Resources;
this proposal adds no object family, dedicated policy Resource type or chosen storage engine.

1. The local owner establishes root Interface R1. R1 can direct system establishment and administer
   shared policy through its management Conduit. Initial verification/persistence still need the
   I4 startup contract; calling an object root does not solve that first-load problem.
2. An Euphony Interface E can administer the policy it is authorized to manage. A future narrower
   assignment might permit Euphony delegation but exclude a personal Pool. That is an illustration
   of later differentiated Interface authority; today's broad owner-operated v0 Interface access
   is not silently narrowed. Both Interfaces use the same policy facts through their own paths.
3. E delegates read-only Linear access to Agent A's participation. The resulting grant records its
   attribution and recipient; closing E's frontend or stopping E does not end A's participation.
   Its current restrictions, withdrawal and detachment rules continue to govern access.
4. R1 is replaced by R2 after its fixed setup definition changes. R2 must establish valid owner
   administration and its own management connection. Existing policy/grants do not transfer between
   Interface owners or get recreated from template defaults. Pending R1 workflow effects still
   require ordinary inspection/recovery; policy independence does not migrate them automatically.
5. R2 and E concurrently edit the same policy. The policy's responsible service validates authority
   and checked changes; neither Interface's cached view becomes a second writable authority.
6. If R1/R2 is unavailable but policy verification is available, other authorized paths can work.
   If permission cannot be verified, hold new managed requests under the accepted no-offline-grants
   rule. Retain supervision and already-dispatched outcomes as previously agreed.

Replacing a client or Interface is also different from replacing the principal to which a grant
is bound. AWS provides a concrete identity example: a specific role ARN in a trust policy's
`Principal` is stored using the role's unique principal ID; deleting/recreating that role breaks
that binding despite reusing its name. This is consistent with Conduit's existing stable-ID rule,
not a requirement to adopt every AWS ARN or policy behavior.
[AWS principal identity and replacement](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html)

Consequently, independent policy must not be described as every grant surviving every deletion.
Deleting its target, recipient or governing policy, explicitly withdrawing it, or ending the
recipient participation has its own contract. Merely replacing an administrative Interface does
not imply any of those events. R2 receives valid access; it cannot inherit authority just by
reusing R1's readable name. Historical attribution remains unchanged.

### What to adopt, and what remains open

Accepted ruling: keep policy independent of Interface lifecycle; permit multiple authorized
Interfaces to administer it; give the owner-operated root broad default administration. Target
services retain their authoritative outcomes. The root does not need exclusive policy stewardship
or a policy-succession operation merely because its own template changes. This replaces the earlier
stewardship synthesis. It is compatible with the proposed active Interface direction role, whose
startup/repair details remain open. How policy is represented and managed is the next discussion.

Policy administration must distinguish authority to manage a grant from authority to use its
target. Future limited administrators need a defined delegation boundary; simple record-update
permission must not accidentally become unlimited delegation. Existing path/recipient restrictions
continue to apply. Detailed multi-Interface permissions can stay deferred, as agreed for v0.

After this ruling, I4 still needs initial Interface persistence and repair, authority-store
representation/extension and engine selection. The cloud comparison does not decide SQLite versus
another engine, a public storage plugin contract, credential custody, automatic failover or
multi-machine support. For the eventual experiment, verify two administrative clients against the
same policy, replacement without policy reset, checked concurrent edits and separate Interface
versus policy-service failures. These are candidate additions, not newly accepted experiment scope
or test results.
