# Construction templates and runtime instances

Updated: 2026-10-31. **Active conceptual proposal; detailed contracts remain open.**
This supersedes the branch proposal that Conduits themselves are the universal construction
graphs. Earlier work remains in Git. The pre-inquiry design is preserved on `main` at
[`c013f0a`](https://github.com/shrav-k/conduit/tree/c013f0a20c0c579f511aea84fdc6804b66417b96).
No implementation or live provider operation has been performed.

## Base concepts

**User-confirmed foundation:** templates are reusable, image-like definitions that support
repeated instantiation. A template is not consumed by an invocation or bound to one instance.
Plugin nodes provide focused functions such as authentication, linking or a particular script.
Graphs compose those nodes as a Dockerfile-like recipe. Plugins are a supporting concept, not
another independently running domain-object family.

Conduit should name an ongoing connection with filtering between platform objects. Construction
is a common mechanism for every family, expressed by templates. It should not require turning
a directory constructor or account-login procedure into a Conduit.

| Family | Template describes how to establish | Instance represents |
| --- | --- | --- |
| Agent — proposed replacement for Session | A participant with selected runtime/model/account, private context, activation and connection configuration. | A particular durable participant and its executions. |
| Resource | A typed capability, through attachment or creation under an owning Pool. | A particular folder, store, search capability, integration or other unitary capability. |
| Pool | An ownership boundary and any configured Resource establishment. | The Pool and its exclusively owned Resource instances. |
| Conduit | A connection: resolve endpoints, authenticate where needed, prepare transport/access and install filtering behavior. | An ongoing relationship with concrete participants, access configuration and connection state. |

Recommend treating Agent as a rename of the existing durable participant, preserving continuity
and separate execution state. An Agent instance can be idle, paused or disconnected. Do not
silently add a second Session identity underneath it or equate it with an OS process; any such
semantic change needs its own ruling. Resource ownership, account separation and retained
memory contracts remain relevant under the new name.

**Two independent axes:** family says what the object represents; form says whether it is a
reusable template or a particular instance. A graph is the structure of a procedure, not another
family. YAML is a representation format, not a lifecycle or an execution engine.

## Reusable templates and plugin graphs

One template can produce many independently identified instances, sequentially or concurrently.
Instance-specific bindings and runtime state must not be written back into the shared template.
Repeatability does not require copying all external data: a Resource contract still distinguishes
creating fresh backing data from attaching to existing backing data through its owning Pool.

The Docker analogy establishes reusable templates and repeatable instances. It does not yet
select an OCI image format, immutable layers, a build cache, mandatory template compilation or
a template repository. Avoid adding these merely because the analogy is useful.

A plugin supplies a focused operation, often backed by a script or process: authenticate, link
a directory, prepare a configuration file or perform another specific action. A graph supplies
composition and ordering. Plugins declare their typed input/output contracts; detailed schemas
and failure behavior remain to be defined. Plugins do not need to become independently running
Agent/Resource/Pool/Conduit objects.
Their implementation may launch a process while executing a node without changing this distinction.
Any process that continues after setup is managed as part of the applicable instance's runtime,
not by keeping the construction node active indefinitely.

**Accepted execution baseline:** the plugin graph runs each time an instance is started.
There is no required separate template-build execution phase in the protocol. The template stays
reusable; each start supplies the applicable instance bindings and evaluates the graph's operations.

A plugin can finish quickly when it establishes that its intended work is already satisfied.
For example, an authentication plugin checks the selected account's current login and either
returns a verified usable result or performs the required sign-in. It does not need to force a
new login merely because another instance starts. Existing login is a current-state observation,
not proof that an old cached authentication result is still valid.

This does not require repeating external creation effects blindly. Each operation retains its
declared intent: fresh creation, attachment, verification or another explicit action. Safe retry
and uncertain-outcome handling remain applicable. Exact restart/continuation behavior for an
existing instance still needs a lifecycle contract; it does not introduce a second template phase.

Caching, shared preparation and other build/runtime optimizations belong to implementations and
later work. The protocol should leave room for them without defining image layers, cache stores,
cache keys, scheduling algorithms or a particular artifact format now. A conforming implementation
can execute without an optimization layer. Shared templates must not acquire an instance's private
authentication state or live handles.

## Protocol properties that leave room for optimization

### Plugin contracts and graph nodes

The following plugin/graph rules are accepted:

| Concern | Contract |
| --- | --- |
| Input/output ownership | Each plugin declares its typed inputs and outputs. A graph node selects the plugin and binds values; it does not independently redefine the interface around a generic command. |
| Dependencies | Binding an output to an input creates an execution dependency. Explicit prerequisites express ordering without data transfer; output bindings need no duplicate ordering declaration. |
| Publication during setup | A plugin contract may make an output usable before its setup operation finishes or publish it upon successful completion. This can unblock later setup steps; it does not make the node an ongoing service supervisor. Consumers still require their other prerequisites and current authority. |
| Validity and lifetime | The plugin declares validity/lifetime rules for each output independently of publication timing. Its outputs need not share one lifetime policy. |

Plugins can still be local scripts or processes. No process protocol, schema language or
installation mechanism is selected by these rules. Exact input/output types, binding syntax,
readiness signals and concurrency policy remain open.

For example, an authentication plugin can accept an account reference and return a verified
identity; its graph node selects the account from template configuration. A directory-linking
plugin can accept an authorized directory reference and destination and return a projection
result. Reusing either plugin does not require templates to repeat its schemas.

Publication timing and lifetime are independent: a completed operation can produce an expiring
handle, and an early output can describe a persisted result. A plugin could return both a
persistent connection description and a live client handle, with different validity rules.
Neither persistence nor prior success guarantees perpetual usability or permission. These are
output contracts, not a requirement for the setup plugin to continue reporting health after
its operation ends. No fixed lifetime taxonomy or cache scheme is selected.

### Setup ends; instances continue

**Accepted responsibility boundary:** the graph is the execution layer for setup. Instances
manage ongoing processes, connections and current availability. A construction plugin reports
the result of its operation; it does not remain the authority for the live capability it helped
establish. This replaces the proposed plugin-owned live-output reporting model.

The instance is the managed object, not necessarily a dedicated process. Conduit's runtime
implements its ongoing behavior, using the applicable type/runtime implementation. How that
code is packaged remains open. Every ongoing local helper or process belongs to an appropriate
instance; this does not make every helper a separately registered Resource. Pool ownership of
Resource instances remains unchanged.

| Example | Setup graph establishes | Instance manages afterward |
| --- | --- | --- |
| Existing directory exposed to an Agent | Resolve permitted access and prepare the Agent-local symlink. | The prepared view and its lifecycle; the Pool-owned directory Resource and backing files remain distinct. No resident linking plugin is needed. |
| MCP access | Resolve the selected service identity, authenticate and prepare the client/transport. | The live connection, its observed availability and applicable reconnect/stop behavior. The setup node need not run for the connection's lifetime. |
| Agent account | Perform any required sign-in and verify the selected identity. | The execution's account binding and authentication state through its native/runtime integration. Refresh or renewed human sign-in is an ongoing lifecycle concern; the exact recovery contract remains open. |
| Local database | Establish configured storage and prepare or launch the service. | The managed service and its current state. Attaching to an external database instead manages that attachment, not ownership of the remote server. |

A recorded result such as “authentication succeeded” describes setup at that time. Current
usability belongs to instance state. A later disconnect does not rewrite that historical result
or require the completed graph to become a monitoring loop. Setup completion therefore differs
from continuing instance readiness. Early output publication remains useful within setup;
long-lived processes and handles must be managed by the instance beyond it.

**Accepted authoring responsibility:** templates declare typed ongoing runtime behavior, using
defaults supplied by their type; setup graphs supply prepared inputs. The
[MCP walkthrough](#mcp-setup-and-instance-runtime-walkthrough) illustrates this boundary.
Graphs may request startup of that declared runtime before setup finishes, with the instance
managing it from startup. Exact runtime fields and start/wait bindings remain open. Recovery
contracts are defined per case below; instance ownership alone does not authorize rerunning the
entire graph. Plugins may be invoked for a later lifecycle operation without remaining live
between operations.

### Optimization-compatible properties

Typed input/output ownership, dependency wiring, declared publication timing and per-output
validity/lifetime are accepted above. Remaining details below are recommendations, not a cache specification:

| Property | Why it helps independent implementations |
| --- | --- |
| Identifiable, versioned plugin operations and named graph nodes | Gives an operation and its meaning an inspectable identity. Exact identifier syntax remains open. |
| Explicit inputs, relevant bindings and typed outputs | Makes dependencies visible rather than hiding them in shell state or ambient account selection. |
| Explicit ordering and readiness requirements | Allows independent work to be scheduled without changing required behavior. |
| Declared effects and observable results | Distinguishes a reusable calculation, a verified existing condition, an external write and an unresolved outcome. Exact classifications remain open. |

Optimization must preserve the current operation contract, identity, authority and result semantics.
Matching YAML alone cannot establish that credentials remain valid or external data is unchanged.
These boundaries allow future caching or reuse where supported without making those mechanisms
part of the basic template-to-instance model.

## Template YAML and Compose-inspired composition

**Accepted authoring boundary:** template YAML remains the complete reusable, typed entry point.
It contains configuration and an inline plugin graph by default, with an optional explicit
graph-source reference for reuse. Both forms have the same execution semantics: the graph runs
on each instance start. A graph does not replace object identity, configuration schemas or
reusable defaults. Exact graph fields, source-reference syntax and detailed plugin schemas remain open;
the sketches below do not settle them. Executable behavior remains in implementation code.

### What to preserve and what to change

| Part | Proposed treatment |
| --- | --- |
| Common envelope | Keep apiVersion, kind, form, metadata and spec. Agent is the proposed replacement for Session; exact API identifiers remain open. |
| Typed configuration and defaults | Keep ordinary domain fields for model/runtime, account references, paths and connection selections. Avoid duplicating their values in graph nodes. |
| Resource types and ownership | Keep separate group/name/version and typed config. Resource instantiation still goes through its one owning Pool. |
| Execution recipe | Express the steps as a graph of focused plugin nodes. Nodes reference configuration fields and preceding outputs where needed. |
| Reuse | One template creates many instances. Permit graph-source reuse without inventing a setup Conduit or another runtime family. |
| Ongoing runtime and observations | The instance manages ongoing processes/connections and current readiness. Keep generated identity and execution outcomes in managed state; authored YAML cannot assert successful login or creation. Exact runtime configuration and setup handoff remain open. |

A complete template consists of its authored contract plus explicit, resolvable dependencies.
It need not be one enormous file. Small templates can keep their graph inline; shared/larger
graphs can use the accepted source-reference option. The two forms describe the same graph
contract. Exact packaging, source resolution and validation fields still need definition.
Referenced source needs an explicit resolution base and the existing checked-update semantics;
editing a referenced file must not silently change registered configuration or a running instance.

### What Compose actually contributes

Primary documentation rechecked on 2026-10-31. The Conduit column is our inference, not a claim
that Compose implements Conduit's graph-on-start model.

| Docker evidence | Conduit implication |
| --- | --- |
| [Compose services](https://docs.docker.com/reference/compose-file/services/) combine an image with runtime arguments; service definitions configure their containers. | Separate the reusable template from the actual settings selected for each instance. Keep those settings inspectable. |
| [Compose build configuration](https://docs.docker.com/reference/compose-file/build/) can refer to a Dockerfile or use dockerfile_inline. Build support is an optional part of the specification. | Inline and referenced recipes can share a contract. Conduit still executes its graph on each instance start; no mandatory image-build phase is being added. |
| [compose config](https://docs.docker.com/reference/cli/docker/compose/config/) resolves input files and variables and renders a canonical application model. | Provide a resolved configuration/graph view before effects. Distinguish resolved configuration from observed runtime success. Exact CLI commands remain open. |
| [Compose include](https://docs.docker.com/reference/compose-file/include/) supports reusable sub-configurations with their own project-directory path bases. | Support modular sources with explicit path resolution and predictable identities. Prior local Compose-version caveats remain in CONFIGURATION.md. |
| [Compose merging](https://docs.docker.com/reference/compose-file/merge/) has mapping, sequence and field-specific exception rules. | Retain Conduit's previously selected explicit field replacement/child edits; do not import a generic append/deep-merge rule for graphs or permission lists. |

The useful analogy is structural: recipe, reusable definition, chosen runtime configuration and
actual instance are distinct. Conduit is not adopting Docker's build/run timing or treating a
Compose service declaration as a running container. Secret values and live handles remain outside
reusable YAML; account/service references identify what an integration must resolve.

### Minimal template sketch

This is a partial Agent-template example showing configuration-to-node bindings. Plugin names,
versions, graph fields and reference notation are illustrative; they are not installed contracts
or an accepted schema. It omits the full account/private-state, connection and readiness contract.

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

The account value is declared once. The authenticate node consumes it; the prepare node consumes
the verified identity. The output reference supplies the dependency, so the graph does not also
need an independent ordering list for those nodes. Actual operations receive the authorized
instance context; a YAML account label is not permission or evidence that sign-in succeeded.

Instantiate the same template with account `claude-work-1` and again with `claude-work-2` by
selecting different values for the ordinary account field. Each gets its own resolved settings
and runtime state. The graph is reusable; plugins still establish the selected identity for
each start. The proposed graph input/output references extend the language, while instance
customization continues through ordinary typed fields rather than a second generic parameter DSL.

### Instance YAML and an optional composition view

An instance's YAML view describes the concrete object: its resolved settings, Resource/Pool/
Conduit bindings and system-owned observations. Configuration-only export omits status, as
already agreed. It is not another hand-authored copy of the graph with success fields filled in.

**Accepted:** each instance retains its resolved setup graph and runtime configuration in its
own spec. Inline and referenced template graphs both produce that current instance definition;
future starts and retries do not depend on the source template remaining registered or unchanged.
The instance definition is immutable after instantiation. Changing its graph or configured
function means editing a template/creation draft and creating a new instance, not replacing
the existing spec. Runtime operations and connections can still evolve as described below.

For example, creating Agent A from a template with an authentication-then-preparation graph
gives A that graph as its active configuration. Adding another setup step to the template later
affects newly created instances, not A's next start. Deleting the source template does not remove
A's recipe.

Retaining the current graph is not a source-template snapshot archive or input/override history.
It does not copy plugin implementations, Resource contents, credentials or live handles. Dynamic
input/output bindings remain typed references; observed results and execution progress remain
managed state. Plugins and other referenced dependencies must still be available and compatible,
and current access rules still apply. The exact resolved representation and schema remain open.
Configuration export must report its dependencies. Instances do not silently follow editable
source files; this introduces no cache mechanism. It chooses the instance's logical content and
inspection contract, not a requirement for physically duplicated storage of identical graphs.

Compose also suggests a future convenience view: select several Agent, Pool, Resource and Conduit
templates, supply settings and express their relationships together. This can be an authoring
bundle over the existing model, not a new running object family or a mandatory second setup
recipe. A Pool continues to own only its Resource instances; it does not become an application
owner for Agents and Conduits merely because their declarations share a file.

Defer that multi-object syntax until one reusable template and its instance representation are
clear. For now, the template YAML remains sufficient with its explicit dependencies. CLI/editor
choices can supply per-instance values through the same typed fields. Registration, starting an
instance and observing its results remain different operations.

### Immutable definitions and changing connections

**Accepted direction:** templates are editable; an instantiated definition is fixed. There is
no generic outside edit of the instance spec or its graph, pending-spec application mechanism,
or automatic rewrite of an instance after a template changes. Creation choices can still be
supplied through ordinary typed fields before instantiation. Restart/retry uses that instance's
retained definition; correcting the recipe means creating another instance.

**Connections remain dynamic:** users or authorized agents can create/open and remove Conduits
that connect an existing instance to additional Resources or other instances. This changes its
relationships through the declared runtime contract, not its underlying function or setup graph.
Initial connection declarations can remain in the frozen spec; current connection membership
and outcomes are managed state, with inspection derived from the actual Conduit relationships.
There is no requirement to edit the Agent's spec to add a connection or keep a second authored
list synchronized with it.

| Fixed definition | State and operations that may evolve within its contract |
| --- | --- |
| Resolved setup graph and runtime behavior | Start, stop, retry, progress, availability and supported recovery. |
| Declared connection/activation behavior and constraints | Open/join/detach/end Conduits; prepare currently permitted capabilities. |
| Resource configuration and ownership contract | Read/write its contents and create or attach Resources through the owning Pool's supported operations. |
| Explicit references to authority and data | Current access decisions and current Resource data; immutability does not freeze either. |

Native-client configuration files or symlinks may be updated as generated preparation state by
an attachment operation. This does not allow arbitrary edits to the instance's authored contract.
If a native integration cannot attach a capability live, report that limit and use only its
supported lifecycle operations; no universal hot-attachment guarantee is assumed. Closing a
Conduit ends access through that connection without implicitly deleting the backing Resource,
files or retained data.
The existing last-consumer rule and explicit cleanup requirements continue to apply.

**Shared context is a Resource:** create or attach it through an owning Pool and expose it to
each participating instance through a Conduit. A new instance can access the same permitted
memory Resource as an older instance without changing either definition or inheriting private
native state automatically.

```text
Pool owns memory Resource M
  Agent A -- Conduit A --> M
  Agent B -- Conduit B --> M
```

Changing the template can produce Agent B with a different fixed definition. Connecting B to M
provides shared context under current access. Removing Conduit A does not remove M or Conduit B.
New instances receive new identities; existing ID-bound relationships are not silently retargeted.
No automatic replacement/deletion of the old instance or copying of its private state is implied.

This supersedes the earlier editable-instance proposal. It removes the need to reconcile a
running setup graph against in-place definition edits. Metadata naming/label rules remain a
separate question. Model choice need not be the same kind of setting as runtime or subscription
identity: the distinction below refines how immutability applies to Agents.

### Agent access and model selection

**User clarification:** do not require a new Agent for every model change. A change such as
Astra to Sol may be supported within the same subscription/access, whereas GPT to Claude should
require a new Agent. Changing the subscription account remains distinct from selecting another
model available through that account. The named models are the user's illustrative labels, not
verified provider identifiers or a promise that a particular runtime supports such switching.

**Accepted selection boundary:** separate the immutable execution/access boundary and model-selection
rule from the current selected model. This expresses model choice independently in configuration,
while validating it against the access and runtime that will actually execute it.

| Concern | Meaning; exact field layout remains open |
| --- | --- |
| Runtime implementation | The native agent integration or Conduit-owned execution runtime. Fixed by the instance definition. |
| Model access | The selected provider route/endpoint and account/subscription or API principal. Fixed; credential refresh/rotation for the same identity is managed state. A new principal/access route requires a new Agent. |
| Model-selection rule | A fixed provider/model domain and any narrower list or capability requirements. Part of the immutable definition. A shared credential or common API protocol does not widen that domain. |
| Initial model | The template's initial choice within the selection rule; retained as creation configuration. Exact field layout remains open. |
| Current model | Runtime-managed selection, changeable through a supported typed operation within the rule. Actual selections are attributable to the operations/turns that used them. |

A future gateway may offer models from more than one provider behind one credential or endpoint.
Therefore same account, same URL or protocol compatibility alone cannot satisfy the requested
GPT/Claude boundary. The integration needs to identify the eligible model domain; do not infer
it solely from display-name prefixes. This is a model contract within the existing Agent family,
not another registry-object family or a mandated new access-profile object.

The effective selectable models must satisfy all of: the instance's model-selection rule, the
runtime's supported operations/capabilities, availability through its fixed access configuration,
and current authority. A same-domain model with unsupported context/tool requirements is not
automatically a usable replacement. Adding a Conduit does not rewrite or broaden this rule.

Partial illustrative instance inspection follows. Field names and domain/list syntax are proposals;
`status` is observed state and is not submitted in configuration writes. Account/route labels are
references, not credentials, and their concrete binding through integrations remains type-defined.

```yaml
spec:
  config:
    runtime: example-agent-runtime
    access:
      route: example-openai-access
      account: work
    modelSelection:
      domain: openai
      initial: astra
      allowed: [astra, sol]
status:
  currentModel: sol
```

Here selecting Sol changes managed state through the declared operation; it does not edit
`initial`, `allowed`, account or graph. Updating the list or execution/access boundary requires
a new instance. Crossing from GPT to Claude, or switching subscription identities, creates a
new Agent that can recover permitted shared context through a Pool-owned Resource and Conduit.
Switching within the permitted domain can retain the Agent's identity and context access when
the runtime supports it; this is not a guarantee of identical context limits or native state formats.

**Accepted default:** allow currently compatible models within the fixed provider/access boundary
by default, with an optional narrower list. The rule stays immutable while matching availability
can change, so a newly available compatible model need not force an instance replacement. The
explicit list in the sketch illustrates optional narrowing. Newly matching models become
selectable; availability changes do not themselves request a switch. Unknown compatibility or
authorization is not proof of eligibility. Catalog discovery, exact policy syntax, handling a
failed switch and retention of the selection across restarts remain open.

### Applying a model selection

**Next proposal:** an explicit selection request waits for a supported execution boundary when
the Agent is busy, leaving in-flight model/tool work uninterrupted. The runtime integration
defines where it can apply the switch, such as between completed model/tool exchanges or only
once the whole Agent turn has finished. A pending request and the model actually in use remain
distinguishable; submission alone is not evidence that a switch occurred.

Before application, recheck the selected model's compatibility, availability and current access.
The operation must report application or failure according to the integration's evidence. An
integration that cannot perform the switch must report that limitation, not silently substitute
another model, account or runtime. Exact error/recovery handling remains open.

For example, an Agent working with Astra receives a request for Sol. Its current model/tool
work completes under the existing selection. At the runtime's supported boundary, the request
can take effect after validation; subsequent model work uses the confirmed selection. This
changes runtime state under the immutable rule and neither starts another model turn nor
implements automatic per-task model routing.

The alternative rejects a request while busy, requiring the caller to resubmit when switching
is supported. Immediate interruption, pending-request replacement/cancellation and exact switch
boundary/status syntax remain separate choices. This timing policy is still under review.

## MCP setup and instance runtime walkthrough

Templates declaring typed runtime behavior and graphs supplying prepared inputs are accepted,
as is graph-requested runtime startup during setup under immediate instance management.
Graphs perform setup; instances manage ongoing behavior. The YAML, detailed startup bindings
and recovery mechanics remain proposals, not an accepted runtime schema or a second operation-request recipe.

### Objects in the example

A Pool named Euphony owns an MCP-access Resource describing the configured service. Agent A
opens a Conduit to that Resource; Agent B can open another, selecting its own permitted service
identity. Shared Resource configuration does not imply shared authentication state or clients.
The service identity is separate from the Agent's Claude/Codex subscription identity. Linear is
an illustrative service, not a tested authentication or transport implementation here.

The Conduit instance manages its participant-specific access relationship. A Resource instance
manages any service it actually operates. Connecting to a remote MCP offering does not make
Conduit the owner of that provider's server. If an existing native agent hosts the MCP client,
its integration must report what it can observe/control; missing support remains explicit. This
example does not require routing all calls through a Conduit proxy or abandoning native tools.

MCP itself is a protocol, not this object model. Its [transport specification](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports)
distinguishes client-launched stdio subprocesses from Streamable HTTP requests. An ongoing
Conduit relationship need not mean an always-open socket. Its [versioning contract](https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning)
also distinguishes current per-request metadata from older initialization-based sessions.
Keep protocol/version handling in the runtime implementation, not a universal hard-coded
handshake graph. Sources checked 2026-10-31; deployed provider compatibility remains unverified.

### Where ongoing behavior is declared

**Accepted:** the template declares typed ongoing runtime behavior; the setup graph supplies
prepared inputs, such as an authentication binding. This keeps intended behavior inspectable
before effects while allowing setup to resolve values only known at execution time. Endpoint
and account settings should not be duplicated across configuration, graph and runtime declarations.

Defaults supplied by the selected type keep ordinary templates short. Authors need not name
another implementation or repeat a standard policy when the type already supplies it. Resolved
inspection shows effective runtime intent. A runtime contract describes behavior the instance's
code supports; it is not another registry family or separately authored runtime instance.
Setup plugins and runtime implementations may share code or a distribution package. Their
different responsibilities do not require two incompatible plugin ecosystems; packaging/ABI
remain open.

This draws on [Compose service configuration](https://docs.docker.com/reference/compose-file/services/),
which describes runtime commands, health checks and restart policies alongside reusable service
definitions. Our inference is to make ongoing behavior explicit while keeping its mechanism in
code. Conduit's graph still runs at instance start, rather than Docker's image-build phase.

Partial illustrative YAML for this accepted boundary follows. Field names, runtime identifiers and
plugin contracts are proposed, not implemented. Endpoint/account choices occur once; the graph
and runtime refer to them. The consumer is supplied by the enclosing Agent's declaration as
already agreed. Filters, full authorization and error contracts are omitted from this sketch.

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

Here `binding` would be a protected authentication reference usable by the runtime, not a token
embedded in exported YAML. The runtime depends on that setup result. The instance retains its
selected configuration and resolved identities; prepared credentials/handles and current
observations remain managed state. Exact serialized bindings and restart representation still
need definition. Runtime policy fields are intentionally absent from the sketch: temporary MCP
transport failure now defaults to bounded automatic reconnection with a manual option, but its
field syntax/limits and other refresh/restart policies remain open.

### Avoiding a fragile process handoff

**Accepted:** a setup graph may request a managed start of its declared runtime before the
remaining setup steps finish. The instance manages it from startup, including while later setup
steps run. Registration alone still starts nothing; the graph runs on explicit instance start.
There is no required final ownership transfer from a setup plugin to the instance.

This is logical ownership, not a requirement to reparent OS processes, start one supervisor per
instance or create another Resource for every helper. A graph may prepare runtime inputs only;
where setup needs a live service for later validation/migration, an explicit managed-start step
can start it before the remaining graph completes. Exact start/wait bindings remain open.

For a database Resource, the illustrative sequence is:

```text
prepare database files → request instance-managed service start
  → wait until usable → apply migrations → finish setup
```

The instance manages the service while migrations run. A failed migration does not erase that
ownership or imply that setup succeeded. By default the managed service remains running for
inspection, explicit retry or explicit shutdown, as defined below. Creating or attaching the Resource still goes
through its Pool; this adds no new runtime object family. Allowing managed startup during setup
does not authorize implicit restarts or repeated creation effects.

The MCP scenario then proceeds as follows:

1. Start the existing or newly registered Conduit instance with its selected endpoint, identity
   and current authority. Resolve configuration and required contracts before their effects.
2. Run the authentication node. It verifies existing access or waits for supported sign-in.
3. Once prerequisites are satisfied, establish the instance-managed client/access binding and
   report what is usable. Native-client management may be delegated through its integration.
4. Finish setup once its required work succeeds. Ongoing operations and their status belong
   to the instance, even though all setup nodes have finished.

For stdio this may include managing a client-launched server process; for a remote endpoint it
may involve a client configuration/authentication binding without any persistent local daemon.
Successful graph completion records establishment, not a guarantee of future availability.

### Failure after runtime startup

**Accepted default:** if a required setup step fails after the declared runtime starts, normally
keep that runtime under instance management and running for inspection or an explicit setup
retry. Report setup as incomplete/failed independently of whether its process is running.
Explicit stop/cancellation can request runtime shutdown without deleting backing data. Exact
status labels and cancellation mechanics remain open.

For the database example, a failed migration leaves an inspectable managed service; it does not
trigger another migration attempt or create another database automatically. The existing rule
retaining successful Conduit attachments during partial preparation remains applicable.

This preserves current authority and does not restart a runtime that independently stops or
override a required shutdown. Failure does not imply rollback of external effects. Reconcile
uncertain outcomes before repetition; even a reported failure can leave partial effects. This
default selects neither automatic retry nor a universal response to failure after successful setup.

### Explicit setup retry

**Accepted:** an explicit retry of the same incomplete setup, with unchanged graph and
configuration, continues from recorded progress. Reuse completed steps only when their results
and prerequisites remain valid under the plugin/output contracts and current authority. A failed
or uncertain step needs its supported recovery or safe-repetition path; a failure label alone
does not make rerunning it safe.

For the failed database migration, retain the database instance and managed service, verify
the required access/readiness, and retry or recover the migration only as its contract permits.
Expired authentication or an unavailable service prevents treating dependent work as usable.
If valid continuation cannot be established, report the unresolved condition rather than
silently replaying earlier creation effects. Exact invalidation/repair mechanics remain open.

This concerns continuation of retained incomplete setup, not a new instance start, cross-start
output caching or automatic retry. The graph-on-each-start rule remains. Instance definitions
cannot be edited between retries; a changed recipe creates a different instance. Plugin/runtime
compatibility and the exact retained progress representation still need review. No full
template-history archive or additional runtime family is introduced.

### Connection recovery after successful setup

**Accepted default:** after successful setup, a temporary MCP transport failure triggers bounded
automatic reconnection where the integration supports it. Templates can select manual recovery
instead. Reconnection uses the same endpoint and still-authorized identity; current access rules
continue to apply. Exact retry limits, delays and YAML fields remain open.

Exhausted attempts or a requirement for interactive sign-in are reported for explicit action.
This policy does not rerun the setup graph, replay uncertain tool writes, change accounts or
start new model work. It does not reactivate a deliberately stopped/closed connection. Repairing
transport inside the existing instance is distinct from explicitly starting an instance again.

An integration must state the recovery it can perform or delegate to its native client; unknown
execution state does not justify duplicate launches. Restored transport does not resolve the
outcome of an earlier request whose response was lost. Automatic credential refresh, replacement
service processes and other failure categories still need their own supported contracts.

### Consumer behavior during an outage

**Accepted default:** a connection that was required during setup becoming unavailable afterward
does not automatically pause the whole consumer. Report the missing dependency, prevent affected
Conduit-managed operations from proceeding through unavailable access, and allow independent
capabilities/work to continue. A running Agent could keep editing local files while its Linear
connection recovers; it could not claim its Linear update succeeded. This does not start an extra
model turn or promise operation-level controls beyond an integration's actual coverage.

Required connections still block initial preparation. Existing mandatory stop rules, such as
isolated execution whose access cannot be revoked live, remain applicable. Exact readiness
labels, how runtime dependencies are declared and stricter type-specific behavior remain open.

### Changes and failures to test

| Event | Proposed interpretation and retained constraints |
| --- | --- |
| Sign-in needs a person | Setup remains waiting with explicit progress; no assertion that authentication succeeded. |
| A later required setup step fails | The instance remains not fully prepared. Its already-started runtime normally remains managed and running; inspection, retry or shutdown is explicit. Successful Conduit attachments remain retained. |
| Conduit crashes after starting a helper but before recording success | Reconcile using supported runtime/native evidence. Unknown state does not justify blindly starting a duplicate; no arbitrary spawn-and-report convention can guarantee recovery. The implementation contract is still required. |
| A prepared MCP connection suffers a temporary transport failure | The instance reports the gap and attempts bounded reconnection by default where supported; templates may select manual recovery. Exhaustion or interactive sign-in needs explicit action. Consumers normally continue independent work, subject to existing mandatory stop rules. |
| A tool write loses its response | Restoring transport is separate from replaying the write. Preserve the existing uncertain-outcome/reconciliation rules. |
| Runtime status cannot be observed | Report uncertainty or the integration's coverage gap; neither historical setup success nor a stored reference proves current health. |
| Consumer closes access | End its logical connection under the established last-consumer rule. Preserve other consumers, the Pool-owned Resource and external service data. |

The runtime implementation can invoke a finite auth/repair operation later where supported;
this does not turn the original setup graph into an always-running recovery loop. Starting a
replacement model execution remains distinct from repairing a service connection.

The same distinction fits simpler types: a symlink needs no resident linking node, a Git
worktree needs no permanently running Git process, and an owned local database may need a
supervised service. Each type defines its actual ongoing behavior; no mandatory health-polling
loop or universal restart policy follows from the word instance.

## Repeated instantiation is fundamental

```text
Reusable Agent template: euphony-engineer
  ├─ instantiate → Agent euphony-engineer-a7f2
  ├─ instantiate → Agent euphony-engineer-b91c
  └─ instantiate → Agent euphony-engineer-c12e

Reusable Conduit template: workspace-access
  ├─ instantiate → a connection for Agent a7f2
  └─ instantiate → a connection for Agent b91c
```

The same relationship applies to Resource and Pool templates. Instances retain their own
identity, configuration and state; the template remains reusable. Sharing an underlying Resource
or explicitly joining a shared Conduit is a separate choice from reusing a template.

Conduits remain ongoing connections and filters. Graph nodes are reusable operational building
blocks; a login step or directory-linking step is not thereby a Conduit. Agent, Resource, Pool
and Conduit instances are the managed objects. Some may be idle or have no dedicated process.

Execution tracking and recovery details remain for later design. No first-class construction-run
family is selected. The graph establishes an instance, and the instance manages ongoing behavior;
exact setup tracking and runtime handoff remain open. Execution bookkeeping must support the
graph-on-start rule without replacing reusable templates with per-instance construction definitions.

## What survives the research

- Typed dependencies, meaningful readiness/completion conditions, named outputs, composition,
  human-interaction waits and supported recovery are useful regardless of the procedure's name.
- The [BuildKit frontend/graph separation](https://docs.docker.com/build/buildkit/) supports
  keeping authoring and execution representations distinct. Its build-cache semantics do not
  automatically apply to authentication or mutable external effects.
- [Compose dependency conditions](https://docs.docker.com/compose/how-tos/startup-order/) distinguish
  readiness from completion. Construction completion and ongoing Conduit readiness need similarly
  precise meanings, without implying a Compose runtime dependency.
- [Temporal's activity discussion](https://docs.temporal.io/activity-definition) illustrates how
  lost results and retries can repeat external effects. Preserve Conduit's explicit outcomes,
  reconciliation and safe-retry constraints.
- Filters remain ongoing Conduit behavior and narrow authority. Their configuration can be
  established by a template; current managed operations still enforce access as it changes.
- Construction progress must remain recoverable across ordinary interruption. Minimal effective
  inputs/step versions and outcomes are execution evidence, not mandatory full template history
  for every completed object. Exactly what is retained remains a design choice.

The detailed [configuration](CONFIGURATION.md), [establishment](ESTABLISHMENT.md),
[Resource](RESOURCES.md) and [connection](CONDUITS.md) research remains useful evidence.
Conduit-specific setup graphs from the earlier branch proposal are preserved in Git rather than
presented as the current model.

## Next discussion

Templates declare typed ongoing runtime behavior with type-provided defaults; graphs supply
prepared inputs. Graphs may request startup of that runtime before setup finishes; instances
manage it from startup. Exact fields and start/wait bindings remain open.

Required setup failure normally retains the managed running runtime while setup remains incomplete.
Explicit retries of unchanged setup continue from valid recorded progress. Temporary MCP transport
failure after successful setup defaults to bounded automatic reconnection where supported, with
a manual option. Consumers continue independent work while missing dependencies are reported;
initial required-dependency checks and existing mandatory stop rules remain applicable.

Instances retain their resolved graph and runtime configuration in their own spec, independently
of the source template. Those definitions are immutable; change a template/creation draft and
instantiate again for a different definition. Dynamic Conduit connections and shared context
Resources are accepted. Current authority and Resource data remain live.

The [Agent selection boundary](#agent-access-and-model-selection) is settled: compatible models
within the fixed runtime/access and provider domain are selectable by default, optionally
narrowed by a list. Supported selection changes retain the Agent; subscription changes or
GPT-to-Claude transitions create a new Agent with shared context accessed through Conduits.
Next review [model-switch timing](#applying-a-model-selection), followed by failed-switch behavior
and restart persistence. Shared context across replacement Agents remains Pool-owned Resource
data; no new Conversation family is needed.

Apply the same contract to two Agent instances using different accounts and to a local directory.
Preserve repeated template use, instance-private state, Pool ownership and current authority.
Agent remains the proposed replacement for Session. Runtime packaging, status syntax and
recovery policies remain open.
