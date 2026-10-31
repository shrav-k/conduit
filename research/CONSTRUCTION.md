# Construction, interfaces and runtime interaction

Updated: 2026-10-31. **Construction/runtime research and illustrative examples.** Current working
contracts live in [SPEC.md](../SPEC.md) and [DESIGN.md](../DESIGN.md). Templates now declare complete
establishment and the permitted instance-configuration changes and parameters. This report supplies
reasoning and graph examples; exact fields remain proposals. The earlier design remains on `main`
at [`c013f0a`](https://github.com/shrav-k/conduit/tree/c013f0a20c0c579f511aea84fdc6804b66417b96).
No implementation or live provider operation has been performed.

## Base concepts

Templates are reusable, image-like definitions supporting repeated instantiation. One template
can create many independent instances without acquiring their private state. Each family has
both forms; an instance need not have a dedicated process or be continuously running.

| Family | Definition and responsibility |
| --- | --- |
| Interface | The user-facing authoring, observation and creation layer. It establishes Conduits for interaction with running instances and can be expressed through different frontends. |
| Agent | An autonomous participant with its own execution/access boundary and private context. It initiates actions and can expose operations for other participants to invoke. |
| Resource | A unitary capability, data source or service, created or attached through exactly one owning Pool. |
| Pool | Owns and establishes Resources and organizes their shared service authentication; grouping reflects capability contracts. |
| Conduit | A connection between endpoints through which permitted interactions and data flow, with filtering and connection state. |

**Interface is the fifth working family.** Agent replaces Session in this proposal. An Agent
retains durable identity separately from its executions; no additional Session family is implied.
An Interface is not the human identity itself. A CLI, terminal UI or browser is a frontend that
presents it, not another domain-object family. Exact frontend attachment semantics remain open.

Agents and users are actors. Resources expose capabilities; a running indexer or database does
not become an Agent merely because it has background processes. An Agent can also be a target:
one Agent may ask another to act through an authorized Conduit. Being an actor grants no blanket
access. Supporting runtime processes and plugin implementations do not add domain families.

Conduits describe ongoing connections, not the universal construction procedure. Focused plugin
nodes compose a setup graph shared by all template families. A login or linking step is not
thereby a Conduit. Family, template/instance form, setup graph and YAML representation are
separate concepts.

## The authoring, observation and creation Interface

**User-confirmed workflow:** there are exactly five families. Interfaces are the primary user
entry point for creating/editing Resource and Agent templates, adding Resource-template references
to Pool configurations, instantiating Pools and Agents, observing them, and establishing Conduits
for access and changes to running instances. CLI, terminal and other frontends express this same
Interface contract. Broad visibility is not a direct-edit exception for running targets.

| Path | Expected behavior of the initial Interface |
| --- | --- |
| Authoring | Create/edit reusable templates against their type contracts, including Resource/Agent templates and Pool composition. |
| Observation | Discover available templates and instantiated objects across families; inspect their reported state and relationships. |
| Creation | Select a template, customize ordinary creation fields and instantiate using the declared construction contract. Resource creation/attachment still belongs to an owning Pool. |
| Connection | Create or join a permitted Conduit to a target or targets. |
| Interaction | Through that Conduit, inspect supported controls and request configuration/lifecycle changes. The target validates and applies them under its contract. |
| Feedback | Present observed state, progress and operation outcomes, distinguishing requested changes from changes actually applied. |

The intended user journey is:

1. Use an Interface to author Resource templates and an Agent template.
2. Add selected Resource-template references and creation settings to a Pool configuration,
   supplying concrete targets for their declared connection requirements.
3. Instantiate the Pool and establish its Resources and shared service authentication.
4. Instantiate Agents whose declared Conduits authorize access to those capabilities without
   repeating the provider login merely because a new consumer attaches.
5. Observe the resulting instances; use Conduits from the Interface to request supported changes.

Resource templates remain reusable definitions; Pool ownership applies to the concrete Resource
instances established through that Pool. Referencing a template does not give it an exclusive
owner. The exact Pool composition fields and registration/start boundary remain under review;
this workflow does not introduce a second setup recipe outside the templates and graphs.

The Interface provides authoring, observability and entry points, not a parallel implementation of target
behavior. An existing Pool's Resource-creation operation must still respect its access/ownership
contract; listing or selecting a template does not bypass that contract. The exact platform
creation endpoint and its relationship to Pool operations remain part of the bootstrap review.

This broad scope is the default management Interface's configuration, not a reason to hard-code
unrestricted authority into every future Interface. Narrower Interfaces can be supported later.
Its visibility concerns registered/configured objects; it does not imply automatic discovery or
credential possession for every service on the machine or internet. Discovery also does not
activate every Resource or load all its data into a conversation.

CLI, terminal UI and automated test clients should use the same management capabilities and
outcome semantics. A frontend renders controls and state; it should not privately implement
Resource creation, provider switching or recovery. A test client can therefore exercise the real
contract without driving terminal pixels. This is a design requirement, not completed test coverage.

An Interface conveys the acting user's identity and authority. Its instance identity identifies
the interaction endpoint, not a replacement for the user. Agent-originated requests identify the
Agent and its effective authority. Both use Conduits and the target's declared contracts. Exact
identity fields and delegation rules remain for review; the existing account separation remains.

**Bootstrap question:** the first Interface must be able to reach registry/template management
and establish Conduits before application instances exist. A proposed solution is a local runtime
bootstrap that establishes the management Interface and its initial management connection.
Registry operations need a defined platform surface. Their endpoint representation, root
authority and recovery path are not yet selected; they do not introduce a sixth family. Do not
require a pre-existing application Pool or Conduit to create the very first one.

## A common interaction contract

**Working direction:** Interfaces and Agents invoke supported operations through Conduits.
The Conduit connects endpoints and applies permitted routing/filtering; the target instance's
runtime owns the operation's meaning and application. The template declares the exposed controls and parameter constraints within that runtime support.
The protocol generalizes their discovery, validation and outcomes, while provider-specific
application and scheduling belong to implementations.

```text
User → frontend → Interface → template authoring / observation / instantiation
                  Interface ── Conduit ──→ existing instance
                  Agent     ── Conduit ──→ existing instance
                             ← results, state and supported events
```

The shared contract below is a proposal for the next grouped review, not a finalized API:

| Protocol responsibility | Type/integration responsibility |
| --- | --- |
| Discover the operations and observations a target supports, with versioned typed inputs/results. | Define provider/domain operations, valid values, constraints and their actual effects. |
| Identify caller, target, operation and request; evaluate current authority through the connection. | Validate whether this instance can perform the requested operation in its current state. |
| Distinguish receipt, pending work, application, failure and unknown outcome where applicable. | Decide whether to apply now, wait for a supported boundary or reject; define ordering/coalescing where needed. |
| Make results and observed state inspectable; expose supported progress, waits and events. | Establish what can actually be observed, including provider/native-client limitations. |
| Preserve request/outcome identity for reconciliation and safe retries. | Declare safe repetition, cancellation, continuation and recovery capabilities; never infer them from a generic verb. |

A command can complete immediately or remain pending. No universal queue, latest-wins rule,
stream transport, event store or extra Workflow family is implied. Clients must not equate
acceptance with application, timeout with failure, or historical success with current availability.
Current permissions still apply when an operation is executed or content delivered. Uncertain
external effects require reconciliation or a supported safe retry, not blind repetition.

These are invocation semantics, not a second setup recipe assembled outside the template.
Starting an instance invokes its declared construction contract; a caller does not resupply its
authentication/linking steps as an ad hoc operation request. Built-in management operations
address templates, registry objects and lifecycle; type-specific operations address capabilities.
Whether some settings share a typed update operation or use named commands remains open.

The same discovery and outcome principles should cover Agent control, memory reads/updates,
Pool-mediated Resource creation and Conduit connection management. This does not require every
family to support every verb or hide provider-native tool names behind generic replacements.
Graph plugins and runtime implementations may share typed operations and code, but exact ABI,
packaging and operation discovery are still undecided.

The [Linear MCP walkthrough](CONDUITS.md#5-access-linear-through-mcp) follows one shared Resource
through shared service authentication, individually authorized attachments, invocation, a proposed
configuration control and shutdown. Handlers use a typed plugin operation or graph while ongoing
state remains instance-owned; the exact handler schema and exposure rules remain under review.

## Instance configuration and supported changes

**Accepted direction:** a template specifies both how to establish an instance and which of its
configuration can change within declared parameters. Type/plugin contracts supply implementation
support; the template exposes/constrains it; a Conduit provides the permitted path to use it.
The complete contract is defined in [DESIGN.md](../DESIGN.md#template-defined-configuration-changes).

A template is a reusable recipe, an instance has its own concrete configuration/change contract,
and observed state reports what is actually applied. Editing a source template does not alter
existing instances. Instance configuration is not universally immutable, nor is every field
implicitly editable. A declared setting change does not silently widen its governing constraints.

[The configuration research](CONFIGURATION.md#template-defined-changes-research-synthesis) compares
Kubernetes transitions, Docker runtime updates, schema constraints and Pi commands. Its bounded
search-control example shows how a template limit and narrower Conduit restriction compose.
Defaults for omitted controls, change-handler representation, transition rules, requested/applied
storage and evolution of a running instance's own control contract remain for grouped review.

Instances retain their resolved setup graph and runtime configuration independently of source
changes; dynamic Conduit attachment and Resource data are separate from definition edits. If a
supported change affects setup inputs, future start and recorded-progress validity must be defined.
No new field-level policy is implied by the construction examples below.

Shared context remains a Pool-owned Resource exposed through Conduits. New instances have new IDs;
replacement does not silently retarget existing bindings or copy private native state. Closing an
attachment preserves backing data/other consumers; last logical consumer departure ends the flow.

### Agent access and model selection

Model selection is no longer a separate core-protocol review track. Previously selected Agent
behavior is retained here as an integration example to test against the general change contract;
it does not settle immutability for all instance types:

- Runtime/account/access and provider domain remain fixed; credential refresh for the same
  identity is runtime state. Different subscription identities or GPT-to-Claude transitions
  require a new Agent, with continuity available through shared-context Resources.
- Compatible models within that fixed boundary are selectable by default, optionally narrowed
  by a list. Initial model is definition data; current model is runtime state. A common gateway
  or credential alone does not erase the declared provider/domain boundary.
- A supported selection waits for an appropriate execution boundary without cancelling work;
  pending and applied selections remain distinguishable and eligibility is checked at application.
- Explicit restart restores the last successfully applied selection after current checks;
  a new instance uses its initial selection. No silent substitute is chosen if use is uncertain.
- After failure, work can continue on a verified eligible previous model. If the active selection
  is uncertain, reconcile before further model requests; independent work need not stop.

The provider integration implements these details and reports unsupported behavior. Exact
model-switch queueing, supersession and cancellation are not universal protocol semantics.
The unanswered latest-pending-request proposal is removed from the core review. No specific
provider support or model identifier has been validated by this example.

## Reusable templates and plugin graphs

**Accepted construction boundary:** template YAML is the complete typed authoring entry point,
with an inline graph by default or an explicit graph-source reference. Plugin graphs execute
on each instance start. There is no mandatory separate template-build phase or selected OCI
format, cache, image repository or compiler. Implementations may optimize later without changing
operation meaning, authority or result validity.

One template may be instantiated repeatedly with ordinary typed creation-field edits. Assigning
a field replaces its value; nested child edits preserve unrelated siblings. No separate template
parameter DSL or implicit deep merge is required. Instances materialize declared defaults and
retain resolved definitions, not mandatory template snapshots or input/override archives.
Referenced code, credentials, Resource contents and live handles are not copied into that spec.
Configuration-only export excludes system-owned status and must identify required dependencies.

### Plugin contracts and graph nodes

| Concern | Accepted contract |
| --- | --- |
| Inputs and outputs | Plugins declare typed contracts; graph nodes select plugins and bind values rather than independently redefining their interfaces. |
| Ordering | Output-to-input bindings imply dependencies. Explicit prerequisites express ordering without data transfer. |
| Publication | Contracts may publish usable outputs during setup or upon completion. Early publication can unblock later setup; it does not turn a node into a permanent supervisor. |
| Validity/lifetime | Declared per output, independently of publication timing. Persistent descriptions and expiring handles may have different rules. |
| Existing state | A plugin may verify that work is already satisfied, such as an existing login, and complete sooner. Old success alone does not establish current validity. |
| Effects and recovery | Preserve explicit create/attach/verify intent, current authority and safe outcome handling. Starting/retrying never authorizes blind duplication of effects. |

Named/versioned plugin operations, explicit bindings and meaningful results support future
optimization. Exact identifiers, schemas, effect descriptors and graph scheduling remain open.
A local script/process can implement a plugin. Its setup result is evidence about that operation;
ongoing availability belongs to instance state.

### Setup ends; instances continue

Templates declare typed ongoing runtime behavior, with type-provided defaults. The graph supplies
prepared inputs. An instance is the managed object, not necessarily a separate supervisor process.
Every ongoing helper belongs to an appropriate instance, without registering every helper as a
Resource. Setup and runtime code may share a plugin package; their responsibilities remain distinct.

A graph may request an instance-managed start of its declared runtime before setup finishes:

```text
prepare database files → instance-managed service start → readiness → migrations → setup complete
```

The instance owns the service from startup. Graph completion records establishment, not permanent
readiness. A symlink needs no resident linking plugin; an external-service attachment does not
make Conduit the owner of the remote server. Later lifecycle operations may invoke finite plugins
without retaining the original graph as an endless monitoring loop.

| Situation | Accepted behavior or selected example policy |
| --- | --- |
| Required setup fails after runtime startup | Normally retain the managed running runtime for inspection or explicit retry/stop; report incomplete setup. No automatic rollback or unsafe repetition. |
| Explicit retry of incomplete setup | Continue from valid recorded progress under unchanged definition, current authority and plugin recovery contracts. Revalidate prerequisites; unresolved effects remain explicit. This is not cross-start caching. |
| Some required connections fail during preparation | Required failures block completion; optional failures report and continue. Required is the default. Successful attachments remain for retry; explicit cancellation detaches. |
| Temporary MCP transport failure after successful setup | Selected integration default: bounded reconnect, manual option, same endpoint and authorized identity. Exhaustion or interactive sign-in needs explicit action. No uncertain write replay, setup rerun or extra model work. |
| Required connection later unavailable | Normally continue independent consumer work and report the missing dependency; affected operations cannot use unavailable access. Initial preparation still requires it. Mandatory access-revocation stop rules remain. |
| Runtime/effect state unknown | Reconcile supported observations; do not invent success or launch duplicates. Report native integration coverage gaps. |

Exact start/wait binding syntax, retained progress, cancellation and recovery compatibility remain
open. Reuse of recorded progress assumes the same effective setup. If future contracts allow changes
to setup inputs or graphs, they must define invalidation or replacement before reusing that progress.
Recovery is instance/type behavior, not a universal operation scheduling algorithm.

## Template YAML and Compose-inspired composition

Keep the common envelope: `apiVersion`, `kind`, top-level `form`, `metadata`, `spec`, and a
system-owned inspection `status`. Resource contracts retain separate type group/name/version
and typed settings. Names, stable IDs, strict schemas and reference bindings remain applicable;
registry update rules for templates remain applicable. Their relationship to Conduit-mediated
instance configuration changes must be resolved rather than assumed. Exact Interface schema and
API versions remain open.

Partial illustrative Agent template; fields, plugin names and reference notation are proposals,
not installed contracts. It omits full runtime, authority and private-state details:

```yaml
apiVersion: example/v0alpha1
kind: Agent
form: template
metadata:
  name: native-worker
spec:
  config:
    runtime: claude-code
    account: claude-work
  graph:
    nodes:
      authenticate:
        plugin: {group: auth, name: native-login, version: v1alpha1}
        inputs:
          runtime: {from: config, field: runtime}
          account: {from: config, field: account}
      prepare:
        plugin: {group: agent, name: prepare, version: v1alpha1}
        inputs:
          runtime: {from: config, field: runtime}
          identity: {from: node, name: authenticate, output: identity}
```

The account is declared once; binding the verified identity supplies the dependency. Creating two
Agents with different account-field choices produces distinct private state and identity. A YAML
label is neither a credential nor proof of authorization. Graph-source references resolve into
the retained instance graph; editing a source file does not silently change registered behavior.

### MCP setup and instance runtime walkthrough

A Pool owns a reusable authentication Resource and an MCP-access Resource connected to it through
a Conduit. Two Agents use the service through separate access Conduits, normally sharing the
managed provider binding. Explicitly different permitted service identities remain possible where
supported; neither mode changes their model subscriptions. The service Resource owns its clients;
the authentication Resource owns reusable login/refresh, supplying access without distributing
the credential to Agents. See the [Linear walkthrough](CONDUITS.md#5-access-linear-through-mcp).
External native clients can use a Conduit-managed MCP endpoint preserving provider-native tools;
direct provider connections require separately verified native interception/control support.

The primary shared-access sequence is:

```text
Pool establishment
  → establish reusable authentication Resource and provider binding
  → connect service Resource to it through an authorized Conduit
  → prepare service Resource's runtime/client state

Agent attachment
  → identify caller and check authority
  → resolve permitted capability and filters
  → expose native operations through the supported mediated route
```

The first sequence may wait for provider consent; the second does not repeat that login for
another Agent. The authentication Resource owns login/refresh while each Conduit owns its caller
relationship. If a provider requires separate identities, an explicitly selected variant prepares
that authentication at the corresponding attachment boundary. Neither mode changes the Agent's
model subscription or makes credentials ordinary exported values.

Reusable authentication is a Pool-owned Resource. Resource templates declare connection needs;
Pool composition supplies concrete targets at instantiation, retained as resolved instance
bindings. Compatibility uses named, versioned capability contracts; exact version matching,
operations and YAML fields remain open. No separate
Resource per token is implied. Native interception/hotplug and provider compatibility remain unverified.

MCP's [transport](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports) and
[versioning](https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning) contracts
were checked on 2026-10-31: client-launched stdio differs from Streamable HTTP, and the current
per-request metadata differs from older initialization-based sessions. An ongoing Conduit is
not necessarily an always-open socket. Keep version handling in integrations, not one mandatory
handshake in every graph.

### Connected Pool and Agent YAML draft

**Review sketch, not accepted schema or runnable configuration.** This combines the accepted
ownership, shared authentication, local Resource entries, dependency binding, capability matching,
dynamic provider catalogs and delegation rules. Field layout beyond the settled envelope and
Resource type/config/conduits fields is proposed. All type, capability, plugin, graph and bundle
identifiers below are illustrative; no referenced implementation is installed or validated by this example. The source paths are
placeholders, not files to execute. Full graph/runtime definitions are omitted to focus on composition.

The management Interface acts for the authorized user. Assume a reusable template catalog:

| Template | Contract in the example |
| --- | --- |
| Resource `linear-authentication` | Selects the intended Linear account; establishes a reusable provider binding and manages its refresh. Exposes a service-authentication capability. |
| Resource `linear-service` | Exposes native MCP discovery/invocation using a required authentication connection. Its declaration is sketched below. |
| Conduit `capability-access` | Establishes an authorized logical attachment, carries supported interactions and applies its restrictions. No login is duplicated merely by using this template. |
| Agent `native-worker` | Supplies the worker runtime, its separate model-account selection, private state and activation behavior; the earlier Agent fragment illustrates setup. |

The catalog is a dependency of the example, not more object families. Complete templates must
resolve their actual plugin/graph/runtime contracts before use. Sharing a provider account across
MCP and a later CLI integration would not imply that the same credential works for both.

**The service template declares its requirement, without choosing an account.**

```yaml
apiVersion: example/v0alpha1
kind: Resource
form: template
metadata:
  name: linear-service
spec:
  type: {group: integration, name: remote-mcp, version: v1alpha1}
  config:
    endpoint: https://mcp.linear.app/mcp
  graph:
    source: example-graphs/establish-mcp.yaml
  runtime:
    implementation: {group: integration, name: mcp-client, version: v1alpha1}
  conduits:
    authentication:
      templateRef: {kind: Conduit, form: template, name: capability-access}
      requires: {group: identity, name: service-authentication, version: v1alpha1}
      required: true
      requestedAccess:
        bundle: example-use-service-authentication
```

The type's capability declaration would describe supported MCP discovery/invocation. The provider
supplies its native tool catalog; the template does not enumerate every Linear operation. The
authentication requirement names a contract, not a secret or a specific account. Proposed `requires`
syntax is separate from the already-selected kind/form/name syntax for registered object references.
The requested bundle names the service's desired use of authentication; its compatibility requirement
does not itself grant that use. Pool establishment must be authorized to delegate the actual access.

**The Pool composes instances and supplies the dependency target.**

```yaml
apiVersion: example/v0alpha1
kind: Pool
form: template
metadata:
  name: euphony-pool
spec:
  graph:
    source: example-graphs/compose-pool.yaml
  resources:
    auth:
      templateRef: {kind: Resource, form: template, name: linear-authentication}
      config:
        account: euphony-linear
    linear:
      templateRef: {kind: Resource, form: template, name: linear-service}
      bindings:
        authentication: {localResource: auth}
```

`auth` and `linear` illustrate the accepted local-entry mechanism. They are neither registry aliases
nor preassigned instance names. Proposed `localResource: auth` syntax means the Resource created
from that entry in this particular Pool instantiation. Another Pool created from the same template
gets different owned instances. Proposed `bindings` supplies the target for the service's declared `authentication`
connection; it does not create a second connection or an independent runtime binding ledger.

The `config` block supplies ordinary creation configuration. Assignment replaces that field rather
than introducing an implicit deep merge; this example assumes the authentication template's
config is entirely its account selection. Runtime/type settings remain separate. More complicated
child edits and creation overrides need their own explicit syntax rather than guessed merge rules.

The accepted default is a reusable composition step in the Pool's setup graph. It reads the
entries and bindings, invokes each Resource's setup contract and exposes their results. Custom
graphs can refer to the same entries for extra ordering or checks. Mapping order alone is not execution order.
Registration can establish identities before required preparation succeeds; the service cannot
complete required preparation until its authentication dependency is usable. An illustrative
resolution could be:

| Local composition entry | Resulting registered Resource | Owner |
| --- | --- | --- |
| `auth` | `linear-authentication-4d1b` | `euphony-pool-3a7c` |
| `linear` | `linear-service-92ab` | `euphony-pool-3a7c` |

The service instance retains its resolved Conduit target ID under the existing identity-binding
rules. Exact output/inspection representation and retry mechanics are not chosen by this table.
No editable copy of the child's live binding is implied on the parent Pool.

**The Interface creates the Agent with a requested attachment.**

The following is an excerpt from a creation draft expanded from `native-worker`. Its runtime,
graph and other instance configuration are omitted here; a complete saved instance retains them.
This is not a whole-spec patch to a running Agent or a separate setup-operation document.

```yaml
apiVersion: example/v0alpha1
kind: Agent
form: instance
metadata:
  name: native-worker-58fa
spec:
  conduits:
    issues:
      intent: open
      templateRef: {kind: Conduit, form: template, name: capability-access}
      requires: {group: integration, name: mcp-access, version: v1alpha1}
      target: {kind: Resource, form: instance, name: linear-service-92ab}
      requestedAccess:
        bundle: example-linear-issue-read
```

The Interface chooses the actual target from the instantiated Pool. Its authorized creation/
connection action can delegate the allowed subset to the Agent; the attachment records the issued
delegation separately from this request. Neither `requestedAccess` nor authoring the YAML grants
permission. `example-linear-issue-read` is an illustrative convenience bundle requiring a verified
expansion into native operations/constraints, not an assertion that MCP metadata can prove arbitrary
read-only behavior. Additional Conduit filters may narrow the delegation; a duplicate allowlist is
not required merely to restate the same requested access.

At runtime the Agent sees the permitted native Linear tools on a supported mediated path. The MCP
Resource uses its authorized connection to the authentication Resource, which manages the reusable
login. The Agent's model subscription stays independent. Detaching the Agent preserves both
Pool-owned Resources and the login for other permitted consumers.

**Accepted partial availability:** ready Resources remain usable while their Pool setup is
incomplete. Extend this example with a local checkout that has no dependency on the Linear login:

| Resource | Setup observation | Availability |
| --- | --- | --- |
| Checkout | Ready | Available through authorized Conduits. |
| Authentication | Waiting for user consent | Required authenticated capability is not ready. |
| Linear service | Waiting on authentication | Its dependent service access is unavailable. |

Allow the ready checkout to be used while reporting the Pool's incomplete setup.
This honors each consumer's required dependencies and any declared gates; it does not treat a
running process as a fully prepared Resource. Preserve partial work under the existing recovery rules.
Exact aggregate status/completion fields and explicit whole-Pool gates remain open.

**Review next:** whether [one reusable access Conduit](CONDUITS.md#can-there-be-one-euphony-access-conduit)
should compose these distinct connections for a consumer. Ready Pool Resources do not themselves
settle a composite Conduit's required components, status or detach behavior.

Local Resource entries are accepted; the names of `resources`, `bindings`, `localResource`,
`requires` and `requestedAccess`, exact capability/bundle identifiers and the graph-source syntax
are still proposals. This example does not choose an API version, provider implementation,
complete permission language or plugin ABI.

### What the research contributes

The following primary sources were checked in the earlier investigation. Conduit implications
are design inferences, not claims that these systems implement this proposal.

| Evidence | Implication |
| --- | --- |
| [Compose services](https://docs.docker.com/reference/compose-file/services/) and [build configuration](https://docs.docker.com/reference/compose-file/build/) | Keep reusable configuration and actual instance state distinct; inline/referenced recipes can share semantics. Conduit still runs setup at instance start. |
| [compose config](https://docs.docker.com/reference/cli/docker/compose/config/) and [include](https://docs.docker.com/reference/compose-file/include/) | Support resolved inspection and reusable sources with explicit path bases; resolution is not proof of successful effects. |
| [Compose merge rules](https://docs.docker.com/reference/compose-file/merge/) | Explicit replacement/child edits avoid importing surprising field-specific merges into graphs and authority. |
| [Compose startup ordering](https://docs.docker.com/compose/how-tos/startup-order/) | Distinguish readiness from completion, including managed startup before migrations. |
| [BuildKit](https://docs.docker.com/build/buildkit/) | Authoring and execution representations can differ; caching is not an authority or freshness guarantee. |
| [Temporal activities](https://docs.temporal.io/activity-definition) | Lost results and retries need explicit effect/recovery semantics; do not infer exactly-once external effects. |

Multi-object composition may become a convenience authoring bundle across the five families.
It need not be another runtime family or make a Pool own Agents, Interfaces or Conduits. Defer
its syntax until the individual contracts work. Detailed supporting research remains in
[configuration](CONFIGURATION.md), [establishment](ESTABLISHMENT.md), [Resources](RESOURCES.md),
[connections](CONDUITS.md) and the Pi reports.

## Next discussion

Use the [grouped review](../REVIEW.md#next-discussion) to formalize template-declared controls:
exposure/defaults, typed value and transition constraints, invocation/application, and contract
evolution. Test authoring through an Interface, Pool-mediated creation and then a configuration
change over a Conduit. Include a narrower parameter filter, rejected change and uncertain outcome.

The same template mechanism must express an Agent, a search Resource, a filesystem Resource and
a Conduit's own supported settings without turning every control into an object or imposing one
provider's queueing behavior. Keep setup graphs, runtime handlers and persistent observations
separate; decide whether change handlers may compose graphs without prematurely requiring one.
