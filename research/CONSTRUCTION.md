# Construction, interfaces and runtime interaction

Updated: 2026-10-31. **Active working direction; detailed protocol schemas remain open.**
The pre-inquiry design is preserved on `main` at
[`c013f0a`](https://github.com/shrav-k/conduit/tree/c013f0a20c0c579f511aea84fdc6804b66417b96).
This branch uses five working families. The boundary between fixed instance definitions and
Conduit-mediated configuration changes is being reviewed. Earlier explorations remain in Git. No implementation or live provider operation has been performed.

## Base concepts

Templates are reusable, image-like definitions supporting repeated instantiation. One template
can create many independent instances without acquiring their private state. Each family has
both forms; an instance need not have a dedicated process or be continuously running.

| Family | Definition and responsibility |
| --- | --- |
| Interface | The user-facing observation and creation layer. It establishes Conduits for interaction with existing instances and can be expressed through different frontends. |
| Agent | An autonomous participant with its own execution/access boundary and private context. It initiates actions and can expose operations for other participants to invoke. |
| Resource | A unitary capability, data source or service, created or attached through exactly one owning Pool. |
| Pool | The ownership and creation boundary for its Resource instances; grouping can reflect their types and capabilities. |
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

## The observation and creation Interface

**User direction:** the basic Interface observes the configured system, can choose any available
template to instantiate an object, and can establish Conduits. Interacting with or changing a
running target requires a Conduit to that target. Broad visibility is not a direct-edit exception.

| Path | Expected behavior of the initial Interface |
| --- | --- |
| Observation | Discover available templates and instantiated objects across families; inspect their reported state and relationships. |
| Creation | Select a template, customize ordinary creation fields and instantiate using the declared construction contract. Resource creation/attachment still belongs to an owning Pool. |
| Connection | Create or join a permitted Conduit to a target or targets. |
| Interaction | Through that Conduit, inspect supported controls and request configuration/lifecycle changes. The target validates and applies them under its contract. |
| Feedback | Present observed state, progress and operation outcomes, distinguishing requested changes from changes actually applied. |

The Interface provides observability and entry points, not a parallel implementation of target
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
authority and recovery path are not yet selected. Whether the proposed additional object stack
means a separate management layer or another object family remains to be clarified. Do not hide this circularity
by requiring a pre-existing application Pool or Conduit to create the very first one.

## A common interaction contract

**Working direction:** Interfaces and Agents invoke supported operations through Conduits.
The Conduit connects endpoints and applies permitted routing/filtering; the target instance's
runtime owns the operation's meaning and application. The protocol should generalize interaction,
not standardize each provider's settings or scheduling choices.

```text
User → frontend → Interface → observation / template instantiation
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

## Instance configuration and supported changes

**Reopened:** do not settle blanket instance-spec immutability before defining interaction.
The earlier fixed-definition rule was too broad to answer the user's current question. A running
instance may expose configuration changes through a Conduit; which fields can change, and what
application requires, belongs to its type/runtime contract. No arbitrary direct registry edit of
a running target is implied by the Interface's visibility.

Three things need distinct meanings, without prematurely fixing their serialized field layout:

| Layer | Meaning |
| --- | --- |
| Template | A reusable construction recipe and defaults. Editing it does not silently update existing instances. |
| Instance configuration | The concrete configuration established for one instance, including whatever settings its contract permits changing through a Conduit. |
| Observed state | What the instance reports as actually applied, running or available. A requested configuration change cannot assert its own success. |

**Proposed protocol boundary:** an endpoint advertises supported configuration/control operations;
an Interface or Agent invokes them over a permitted Conduit; the target validates the change,
applies it as supported and reports its outcome. The protocol carries typed intent and evidence.
The target contract determines whether a particular change is live, requires restart, requires a
replacement instance, or is unsupported. These categories are review candidates, not a mandatory
universal mutation API or a final field-by-field policy.

This keeps two questions separate: *through what path may a change be requested?* and *which
changes can that target support?* Requiring a Conduit answers the first; it does not by itself
make every spec/graph field editable or immutable. Whole-object replacement versus typed settings
updates, version checks, concurrent changes and the stored requested/applied representation remain
open. Do not import provider-specific queue behavior into that core discussion.

Instances still retain their resolved setup graph and configuration independently of later source
template edits/deletion. This is self-contained execution configuration, not mandatory template
history. If changes to that retained configuration are supported, their effect on future starts
and recorded setup progress needs an explicit contract. The source template does not acquire the
instance's live state. Current authority and Resource data remain live throughout.

Conduit attachments can be added/removed. Current membership is derived from actual relationships,
not duplicate editable endpoint lists. Supported attachment operations may regenerate native MCP
configuration or Agent-local symlinks; native hot attachment is not universally guaranteed. Closing
one Conduit does not delete backing data or other consumers. The last logical consumer/peer ends
its flow; temporary transport failure does not itself detach that consumer.

Shared context is a Pool-owned Resource exposed to participating instances through Conduits.
A replacement Agent can access the same permitted context without automatically inheriting private
native state. New instances have new IDs; existing bindings are not silently retargeted. Resource
ownership, current access and explicit cleanup remain independent of the configuration-edit policy.

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

A Pool owns an MCP-access Resource. Two Agents can connect through separate Conduits using
different permitted service identities. Those identities are distinct from their model subscriptions.
The Resource describes the service capability; each Conduit manages its access relationship.
Native clients may host the actual MCP connection, with observable/control coverage reported.

Partial Conduit template, again illustrative rather than a validated schema:

```yaml
apiVersion: example/v0alpha1
kind: Conduit
form: template
metadata:
  name: service-access
spec:
  config:
    target: {kind: Resource, form: instance, name: euphony-mcp}
    serviceIdentity: euphony-work
  graph:
    nodes:
      authenticate:
        plugin: {group: auth, name: service-login, version: v1alpha1}
        inputs:
          target: {from: config, field: target}
          identity: {from: config, field: serviceIdentity}
  runtime:
    implementation: {group: integration, name: mcp-client, version: v1alpha1}
    inputs:
      target: {from: config, field: target}
      authentication: {from: node, name: authenticate, output: binding}
```

The enclosing consumer supplies its identity. Authentication produces a protected binding, not a
token embedded in YAML. The instance manages prepared runtime state after the setup node finishes.
Exact output/start bindings and serialized state remain open. Linear/provider compatibility has
not been live-tested; neither a proxy-only topology nor universal native-client hotplug is assumed.

MCP's [transport](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports) and
[versioning](https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning) contracts
were checked on 2026-10-31: client-launched stdio differs from Streamable HTTP, and the current
per-request metadata differs from older initialization-based sessions. An ongoing Conduit is
not necessarily an always-open socket. Keep version handling in integrations, not one mandatory
handshake in every graph.

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

Review the common interaction contract with the Interface as the observation/creation entry point:
discovery, connection establishment, typed configuration/control changes and observable outcomes. Work through
Interface → Pool → Resource creation, then Interface → Agent control and Agent → Agent control.
Use the same rules with different target contracts; do not return to one model's queue policy.

Then settle the bootstrap/identity boundary, type/version discovery and lifecycle schema in
[REVIEW.md](../REVIEW.md). Preserve Resource ownership, dynamic Conduits, account/private-state
separation, current access, memory/evidence retention and safe recovery. The five-family framing
does not promise support for every native agent's control surface or full sandboxing in v0.
