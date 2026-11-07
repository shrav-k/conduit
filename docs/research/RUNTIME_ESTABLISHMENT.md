# Connecting setup graphs to instance runtimes

> Not binding. This research predates the specification in [`spec/`](../../spec/README.md); where it says accepted, current, SPEC.md or DESIGN.md, it means the documents as they stood before 2026-11-05.

Research date: 2026-11-03. This follows the [directory, MCP and database walkthroughs](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax/AUTHORING.md#guided-directory-walkthrough)
and the [graph authoring audit](language-protocol-foundations/GRAPH_AUTHORING_REVIEW.md).
The owner requested comparative database/cloud research before resolving the next structural question.
The owner subsequently accepted approach B: ordinary typed operations explicitly bound to declared
instance runtimes, publishing results with defined guarantees. The
[lifecycle contract](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/LIFECYCLE.md#lifecycle-protocol) records that ruling. This report
preserves its reasoning and alternatives; remaining representation choices are still proposals.
The follow-up ruling places the runtime target in a declared typed operation input using ordinary
graph bindings, with explicit forwarding through reusable compositions. Trusted caller/authority
context remains separate; the target input grants no access.
All Conduit supporting vocabulary and example names remain provisional.

## The question and existing boundary

How should a setup graph address the ongoing runtime declared by its template, supply prepared
inputs, and receive outputs that let subsequent setup continue?

The [accepted lifecycle contracts](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/LIFECYCLE.md) already establish that:

- Templates declare ongoing behavior; setup graphs supply prepared inputs.
- Setup can request instance-managed startup before later steps such as database migration.
- The instance owns ongoing activity from startup. A setup helper is not its permanent supervisor.
- Required setup failure normally preserves an already-started runtime for inspection or explicit
  retry. Cancellation, runtime shutdown and backing-data deletion are separate concerns.
- Explicit retry uses retained progress only while its results and prerequisites remain valid.
  Uncertain effects need the responsible operation's recovery contract.
- Finite dependency graphs compose typed operations; provider-specific branching and polling
  remain within implementations. Definitions do not change silently on existing instances.

The accepted connection is an explicit binding between the runtime declaration and an ordinary
startup operation; its precise representation remains open. A graph output named `database`
does not by itself explain which runtime was started, what readiness was checked or who now
manages it. Neither a PID nor a provider operation ID is
automatically a usable, authorized database connection.

## Evidence and limits

The sources below were inspected for this report. CloudNativePG source is pinned to commit
`b6617eee0152822f74e8a7242bbfe014304db3a1`. Other links are moving primary documentation unless
their API version appears explicitly. No database, container cluster or cloud deployment was run;
the comparisons are documentary evidence and design reasoning, not integration validation.

### Docker Compose: choose what a dependency must establish

Compose distinguishes a dependency being started, passing its configured health check and
completing successfully. Ordinary startup ordering alone does not wait for service readiness.
These are explicit dependency conditions in its authoring model.
[Source: Compose startup order](https://docs.docker.com/compose/how-tos/startup-order/).

**Conduit implication:** the database example needs both a usable database and a successful
migration prerequisite. A health observation cannot replace migration evidence. Compose offers
useful authoring distinctions, but its service lifecycle and reconciliation choices are not
automatically Conduit's lifecycle policy.

### Kubernetes: finite initialization and continuing services have different lifetimes

Ordinary init containers finish in sequence before application containers start. Restartable init
containers used as sidecars can remain active as later initialization proceeds; their startup
conditions govern that progression. Kubernetes also distinguishes startup, readiness and liveness
probes, with different consequences for traffic and restart behavior.
[Sources: init containers](https://kubernetes.io/docs/concepts/workloads/pods/init-containers/),
[sidecars](https://kubernetes.io/docs/concepts/workloads/pods/sidecar-containers/),
[probes](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/).

**Conduit implication:** putting every step in a single “before runtime” phase is insufficient.
A migration cannot connect to a regular application container in the same Pod if that container
cannot start until the migration finishes. An external database or a different lifecycle layout
can solve that Kubernetes case. Conduit's already-accepted startup during setup solves the
corresponding dependency directly. Kubernetes restart policies should not silently replace our
explicit restart, unknown-execution and uncertain-effect rules.

### CloudNativePG: database-specific setup coexists with a dedicated manager

CloudNativePG's instance manager is the parent of PostgreSQL and manages its process lifecycle,
including probe handling. Its bootstrap contract separately supports ordered initialization SQL;
errors prevent successful bootstrap completion. This is database-specific behavior inside an
operator, rather than a generic graph deciding what every database needs.
[Sources: instance manager](https://github.com/cloudnative-pg/cloudnative-pg/blob/b6617eee0152822f74e8a7242bbfe014304db3a1/docs/src/instance_manager.md),
[bootstrap](https://github.com/cloudnative-pg/cloudnative-pg/blob/b6617eee0152822f74e8a7242bbfe014304db3a1/docs/src/bootstrap.md).

**Conduit implication:** keep process supervision with the instance's responsible service and
database knowledge in the selected contributions. A reusable setup graph may compose those
contributions without becoming a PostgreSQL operator itself. Bootstrap SQL is not evidence of a
universal contract for every later application migration.

### AWS: operation progress, resource identity and usability are distinct

CloudFormation resource handlers return progress and a resource model containing identity. Its
contract requires desired-state stabilization before success, while runtime-state stabilization
has weaker requirements; those guarantees must not be conflated. Its callback context can carry
information needed when a handler is invoked again to continue work.
[Sources: handler contract](https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-test-contract.html),
[progress events](https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-test-progressevent.html).

The CDK custom-resource framework separately offers an initial handler and an optional completion
check, allowing asynchronous work to outlive an individual handler execution. RDS's availability
waiter polls the database instance status; it is not an application-schema migration check.
[Sources: CDK provider framework](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.custom_resources-readme.html),
[RDS availability waiter](https://docs.aws.amazon.com/cli/latest/reference/rds/wait/db-instance-available.html).

**Conduit implication:** retain operation progress and the actual target identity, then wait for
the contract's promised condition. Polling that operation is different from issuing creation
again. These APIs demonstrate useful separation; they do not justify adopting CloudFormation's
whole handler vocabulary, desired-state controller or deployment rollback policies.

### Google Cloud SQL: a completed operation still needs its outcome inspected

Cloud SQL represents asynchronous work with an operation identity distinct from its target
instance. The operation includes status, target references and a separate error field. `DONE`
means the operation completed; success also requires inspecting its outcome.
[Source: Cloud SQL operations](https://docs.cloud.google.com/sql/docs/postgres/admin-api/rest/v1/operations).

**Conduit implication:** “the request finished,” “the database exists” and “the application schema
is prepared” are separate facts. A setup operation can retain the provider operation reference
for recovery without exposing it as the database capability consumed by later steps.

### Azure: acceptance is followed through a provider-defined operation endpoint

Azure Resource Manager describes asynchronous responses and operation-status URLs, with
`Azure-AsyncOperation` or `Location` used according to the response and API contract. Azure
PostgreSQL's versioned create API includes an accepted response with a location for continued
tracking. An initial response is therefore not sufficient evidence of completed setup.
[Sources: ARM asynchronous operations](https://learn.microsoft.com/en-us/azure/azure-resource-manager/management/async-operations),
[PostgreSQL create, API 2024-08-01](https://learn.microsoft.com/en-us/rest/api/postgresql/servers/create?view=rest-postgresql-2024-08-01).

**Conduit implication:** the driver should translate its provider's actual progress and outcome
semantics into the operation contract. A frontend disconnect or polling timeout cannot establish
that the remote service stopped, that no resource was created or that another create is safe.

### Nomad and Flyway: useful boundaries, not a universal phase/recovery model

Nomad lifecycle hooks distinguish prestart, poststart and poststop tasks, with sidecar lifetimes
separate from ephemeral tasks. Poststart ordering is based on the main tasks running; it does not
by itself establish an application's particular readiness condition.
[Source: Nomad lifecycle](https://developer.hashicorp.com/nomad/docs/job-specification/lifecycle).

Flyway documents how migration transaction and failure behavior depends on the database and
statements involved. Nontransactional effects may require manual cleanup or repair; not every
migration failure can roll back cleanly.
[Source: Flyway transaction handling](https://documentation.red-gate.com/fd/migration-transaction-handling-273973399.html).

**Conduit implication:** lifecycle phases can be convenient for common recipes, while our graph
allows more explicit interleaving. Recovery must remain attached to the actual effect contract;
the graph cannot manufacture rollback, idempotency or safe replay.

## What the comparison supports

The sources converge on distinctions, not one canonical syntax or orchestration language.
For our database example, distinguish:

| Fact | What it establishes | What it does not establish |
| --- | --- | --- |
| Instance registered | Conduit has an identity and definition. | Provisioning or runtime startup occurred. |
| Startup/create request accepted | A responsible service has accepted work. | The database is usable or that work succeeded. |
| Database usable for setup | The startup contract's specific connection/check succeeded. | The application migration succeeded. |
| Required setup completed | Migration and verification prerequisites succeeded. | The database will remain available forever. |
| Current runtime observation | What the responsible service can presently confirm. | A past result is still a live handle, or current access is authorized. |

These are example guarantees to describe in contracts, not a proposal for a closed status enum
shared by every Resource. A directory attachment might require no persistent helper process.
An MCP runtime needs protocol/authentication checks. A managed cloud database has provider-side
supervision; Conduit manages its binding, operations and observations without claiming control
of the provider's internal operating-system processes.

### Avoid a hidden readiness cycle

The intended sequence is:

```text
prepare backing → start database → database usable for migration
                                      ↓
                                apply migration → verify schema → setup complete
```

If startup waits for “application ready,” and that condition includes the migration, the graph
can deadlock: migration waits for startup's output while startup waits for migration. This can
occur even when the visible graph is acyclic, because the driver introduces the other dependency.

The startup/output contract should therefore state the guarantee it provides. Later checks can
establish stronger guarantees. This does not require a universal list of readiness levels or
claim that a successful check remains true after a subsequent outage.

## Three authoring approaches

The following are Conduit design alternatives inferred from the comparison. No source platform
selects these exact options for us. The owner selected B after discussing this report; A and C
remain comparisons rather than additional accepted v0 authoring surfaces.
In every option, registration alone starts nothing; this concerns an explicitly requested setup.

| Approach | How it would express the example | Advantages | Costs |
| --- | --- | --- | --- |
| A. Special lifecycle nodes | A dedicated graph instruction starts a declared runtime; another can wait for its readiness. | Lifecycle intent is recognizable by the engine and tooling. | Adds a special node vocabulary and rules alongside ordinary plugin operations; extension boundaries need care. |
| B. Typed operations with an explicit runtime binding | An ordinary operation targets the instance's declared runtime, supplies prepared inputs and returns the contract's result. | Reuses plugin contracts, graph composition, validation and recovery; target and ownership stay visible. | Requires a public managed-lifecycle interaction contract and checks that the operation is compatible with the selected runtime. |
| C. Startup driven by available inputs | The declared runtime starts when its input dependencies resolve; later graph steps consume its outputs. | Concise declarative authoring; less explicit startup wiring. | Execution ordering spans runtime declarations and graph nodes; readiness cycles, cancellation and recovery need a combined dependency model. |

A could internally lower to B; C could later be convenient authoring over B. Those possibilities
do not require supporting all three authoring surfaces initially. Hiding the runtime target in
an operation's implicit “current instance” context is another convenience, but makes composition
and compatibility less inspectable. It should not be mistaken for an explicit typed binding.

**Accepted direction: B.** Keep the graph composed of typed operations and make the
runtime they operate on explicit. Let the selected implementation handle provider details while
the instance's responsible service retains ongoing ownership. This favors our protocol's primary
goal: small authored contributions that can be composed into larger behaviors.

This choice does not introduce a Runtime object family, a mandatory primary plugin,
one operating-system process per instance, a universal lifecycle enum or an arbitrary command
that becomes managed simply by returning a PID. A subsequent ruling permits multiple named runtime
declarations when the instance's type supports them. Declaration layout and reference field spelling
remain open; start/wait granularity must state each operation's guarantees under the existing contracts.

## How B would work in the examples

### A local database

1. The Resource template declares its database runtime, selected driver and permitted behavior.
2. A graph step prepares backing and returns a typed descriptor.
3. A startup operation is explicitly bound to that declared runtime. It supplies the descriptor
   and captured settings to the instance's responsible service, which manages startup from the
   beginning. Any helper used to carry out the request is not the lifetime owner.
4. Once the startup contract's condition is established, the operation publishes its database
   result. That result can support migration through the authorized managed access contract.
5. Migration and verification finish. The setup invocation can end while the runtime continues.

The graph-selected startup contribution and declared runtime driver must have compatible contracts;
they need not be the same package. Pool ownership and access to separately managed backing or
authentication Resources remain governed by their existing Conduits. Naming a runtime or passing
a descriptor does not create a grant or make internal setup operations public tools.

### A cloud database

The shape is similar, but startup requests creation or attachment through the cloud adapter. The
instance's responsible service retains the target and operation evidence; the provider runs the
database. The adapter tracks the provider operation and verifies the promised setup capability.
Migration remains a later operation if the template requires it.

If creation completes remotely but the response is lost, recovery attempts to establish the
existing outcome using available evidence. It cannot assume failure and issue another creation.
If the provider lacks sufficient reconciliation support, uncertainty remains visible. The same
public operation structure does not promise identical recovery capabilities across providers.

### Shared authenticated MCP access

The authentication Resource retains custody of credentials under its own contract. A service
Resource's graph obtains an authorized binding and supplies it to that Resource's declared MCP
runtime through a startup operation. The runtime establishes the required protocol session and
reports its promised capability; later checks can inspect the available catalog.

Once setup finishes, refresh belongs to the authentication Resource and supported connection
maintenance belongs to the service runtime. Consumer-specific Conduit participation and endpoint
activity remain separately managed. A transient reconnect is not a reason to replay the whole
setup graph or an uncertain provider tool write. This is an architectural example, not a claim
that a particular Linear transport or authentication mode has been validated.

### A folder attachment

The instance may only need an established directory descriptor plus supported access/control
behavior. B must not force a dummy daemon or a startup step when the type has no continuing process
to launch. Agent-local exposure remains Agent preparation, and attaching a separately managed
Resource remains a Conduit relationship. The common boundary is responsibility, not process count.

## Failure, recovery and inspection consequences

| Event | Consequence under the existing model and proposed binding |
| --- | --- |
| Startup request accepted, readiness not confirmed | Keep operation progress and any known runtime/provider identity; do not publish a stronger usable result. |
| Database usable, migration fails | Preserve the managed runtime and effects; setup is incomplete, and verification's success prerequisite is unmet. |
| Connection handle expires or runtime restarts | Retain historical result evidence, but revalidate/reacquire usable access under its contract; a recorded value is not proof of current validity. |
| Retry of the same incomplete graph | Reuse only valid recorded progress, and reconcile uncertain effects; no automatic creation or migration replay. |
| Captured configuration differs from a later confirmed setting | Preserve the execution's input snapshot without claiming it describes the runtime's current state. Apply existing revision and current-authority checks. |
| Frontend disconnects or polling times out | The responsible service/provider may still be working. Inspect retained evidence; do not infer cancellation. |
| Management authorization cannot be verified | Keep applicable supervision and already-dispatched outcomes; hold new protected work as required by the accepted no-offline-grants rule. |

Inspection should connect the instance, its declared runtime, the requesting operation, retained
target identity, setup progress and current observations. These relationships explain why a
database can be running while setup is incomplete. They need not become separately maintained
user-authored records or expose credentials. Exact observation and protected-handle fields remain
part of the existing contract review.

## Review disposition

Research supports the accepted separation of finite setup and instance-owned ongoing behavior.
No accepted decision needs to be reversed merely because cloud platforms use different lifecycle
vocabularies or declarative controllers. The strongest refinement is to make the runtime target
and promised startup guarantee visible in the authoring contract.

The owner accepted ordinary typed operations explicitly bound to declared runtimes as the
structural boundary. Next, work through the database and MCP examples to resolve target selection,
prepared inputs and output validity before choosing their field names. The owner also selected a
declared typed input for the runtime target, reusing ordinary graph binding and composition instead
of adding a separate runtime-context binding. Caller identity/authority remain trusted context.
The direct-field reference layout and whole-value pointer-omission behavior were also subsequently
accepted. Later rulings accept typed invocation metadata, the five-category v0 reference-source set
and successful completion as the default ordering prerequisite without data. Multiple named runtime
declarations are now accepted when the type supports them, with explicit targets and instance-owned
supervision. This does not add another primary Agent execution or a process-per-declaration rule.
Target/selector encoding, declaration layout, prerequisite notation and final vocabulary remain open. This report
does not close the [behavior/structure merge checkpoint](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/REVIEW.md#behavior-and-structure-merge-checkpoint)
or authorize runtime implementation.
