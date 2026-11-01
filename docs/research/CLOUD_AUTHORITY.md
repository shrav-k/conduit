# Cloud management patterns and Conduit's authority boundary

Research date: 2026-11-01. **I1 remains open on authority and outage behavior.** Separate
management and execution services are accepted. This report compares the remaining alternatives
using AWS, Google Cloud and Azure, with concrete control and recovery examples. It extends
[DISTRIBUTED_RUNTIME](DISTRIBUTED_RUNTIME.md), rather than replacing its process-supervision and
distributed-systems evidence. The [implementation review](../proposals/implementation-v0/REVIEW.md#i1--independent-execution-and-management)
is the place to make the ruling.

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

**Recommendation for discussion: C.** It combines the independence the user wants with a common
access model. This is a refinement of the alternatives, not an accepted implementation change.
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

We therefore still need a separate outage ruling: require fresh authoritative admission; permit
a defined bounded validity period; or use another explicit policy with testable withdrawal
semantics. A v0 implementation can avoid offline grants without requiring every workload to stop.
It must show which effects are waiting and why. No option is selected by this research.

Revocation also cannot undo an already completed call or erase context already delivered to a
native Agent. This preserves the existing managed-access versus actual isolation distinction.

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

## 7. Consequences to review before choosing C

The recommendation has costs, which should be explicit in the ruling:

1. **One logical registry, distributed facts.** The registry remains the naming/registration surface.
   An inventory row can be a cached observation. A current instance read/export may need its owner,
   and unavailable owners produce a stated gap rather than an invented current export.
2. **One configuration authority per instance.** Keep one authoritative `spec.config` and revision;
   management's copy is a view. There are no independently writable desired/applied maps.
3. **Whole-object consistency still matters.** Registration, renames, retirement and ownership
   changes can involve both the shared directory and the instance owner. Their ID/version checks
   and reservations need coordination. Do not quietly introduce separate, weaker revision checks
   for metadata or promise atomic multi-object updates. Exact mechanics follow the authority ruling.
4. **Execution and authority are distinct roles.** An assigned executor may host the instance
   authority in v0. Restarting a worker does not transfer that authority. Unknown owner status must
   not trigger a second writer; automatic migration/failover remains out of scope.
5. **Observation is independently recoverable.** The Interface combines sources, identifies
   freshness and missing coverage, and can reconnect to retained results. A diagnostic export
   cannot become the only record of successful application.
6. **Provider truth has limits.** For existing external MCP services, Conduit owns its client
   Resource configuration and records what the provider confirms. It does not own the provider's
   internal database or gain stronger recovery guarantees by wrapping it.

If these costs outweigh the benefit for the first implementation, A is a reasonable starting
arrangement, provided its operation protocol preserves the ability to place state ownership
elsewhere later. B is also coherent, but decentralizing policy is an additional decision beyond
making execution and configuration independent. No cloud reference makes one arrangement mandatory.

## 8. Changes proposed for experiment 002

Keep the accepted separate management/execution layout and synthetic plugin/template authoring
proof. Add fault boundaries that discriminate between authority choices:

- Apply the search setting, retain the responsible service's result, and sever its response to
  the caller/aggregator. Inspect the canonical revision separately from the cached view.
- Crash between an external effect and its local receipt; demonstrate recovery without blind replay.
- Submit two changes from R7 through different Interface/Agent paths; one authority coordinates
  them, and a pending or uncertain first change cannot be bypassed through another endpoint.
- Keep the aggregate inventory stale, then reconnect; older observations cannot overwrite a newer
  authoritative revision or be passed off as a globally current snapshot.
- Keep execution alive while breaking policy reachability. Record separately what is supervised,
  what was already admitted and what new work is held under the chosen outage policy.
- Ensure the recovered result remains available through an authorized standard MCP client without
  requiring an aggregate frontend or a new plugin-specific recovery server.

These are proposed tests, not observed results. The first experiment need not implement all three
authority models or deploy on AWS/GCP/Azure. Select the conceptual owner first, then make that
choice falsifiable with local service boundaries. Storage technology remains I4; schema spellings
and MCP profile fields remain I7.
