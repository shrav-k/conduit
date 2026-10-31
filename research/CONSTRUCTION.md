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
composition and ordering. Plugin interfaces still need inputs, outputs and failure behavior;
plugins do not need to become independently running Agent/Resource/Pool/Conduit objects.
Their implementation may launch a process while executing a node without changing this distinction.

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

The following are design recommendations for the node contract, not a cache specification:

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
reusable defaults. Exact graph fields, source-reference syntax and plugin contracts remain open;
the sketches below do not settle them. Code remains in the plugin implementations.

### What to preserve and what to change

| Part | Proposed treatment |
| --- | --- |
| Common envelope | Keep apiVersion, kind, form, metadata and spec. Agent is the proposed replacement for Session; exact API identifiers remain open. |
| Typed configuration and defaults | Keep ordinary domain fields for model/runtime, account references, paths and connection selections. Avoid duplicating their values in graph nodes. |
| Resource types and ownership | Keep separate group/name/version and typed config. Resource instantiation still goes through its one owning Pool. |
| Execution recipe | Express the steps as a graph of focused plugin nodes. Nodes reference configuration fields and preceding outputs where needed. |
| Reuse | One template creates many instances. Permit graph-source reuse without inventing a setup Conduit or another runtime family. |
| Runtime observations | Keep generated identity, readiness and execution outcomes in managed instance state; authored YAML cannot assert successful login or creation. |

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

To restart consistently, an instance needs an identifiable executable recipe or binding that
cannot silently change when the source template is edited. Whether it retains a resolved graph
or a versioned graph reference needs a separate contract. This is distinct from retaining every
original YAML file, comment or historical template revision. No blanket template-history archive
or cache mechanism is being selected here. Export must report any dependencies needed for reuse.

Compose also suggests a future convenience view: select several Agent, Pool, Resource and Conduit
templates, supply settings and express their relationships together. This can be an authoring
bundle over the existing model, not a new running object family or a mandatory second setup
recipe. A Pool continues to own only its Resource instances; it does not become an application
owner for Agents and Conduits merely because their declarations share a file.

Defer that multi-object syntax until one reusable template and its instance representation are
clear. For now, the template YAML remains sufficient with its explicit dependencies. CLI/editor
choices can supply per-instance values through the same typed fields. Registration, starting an
instance and observing its results remain different operations.

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
family or mandatory construction-state ownership scheme is selected here. The graph-on-start
rule is settled; execution bookkeeping must support it without replacing reusable templates
with per-instance construction definitions.

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

The execution baseline is settled: run the plugin graph on each instance start, with plugins
able to verify and reuse already-satisfied state. Implementation optimizations remain deferred.

The [authoring boundary](#template-yaml-and-compose-inspired-composition) is also settled:
retain the typed YAML envelope and configuration, with an inline or explicitly referenced graph.
Exact graph/binding syntax remains open; multi-object composition can follow later.

Then work through the same reusable Agent template started twice, using an illustrative sequence:

```text
prepare instance-private state
  → authenticate the selected account
  → establish configured Conduit instances
  → report the Agent ready
```

| Case | Authentication behavior | Resulting instances |
| --- | --- | --- |
| First start requires sign-in | The plugin performs the supported sign-in flow and verifies the selected identity. | An Agent and its configured connections, with their own runtime state. |
| Another start has a valid existing login | The same plugin verifies current identity/usability and can return sooner. | A separate Agent and its configured connections; template reuse does not merge private context or connection identities. |

The example is an intended contract scenario, not verified native-provider behavior or a new
schema. A real adapter must support the selected account/state separation. Opening a fresh
connection versus explicitly joining a shared one remains a declared choice.

Use those two executions to define the minimum plugin-node contract together: inputs, outputs,
prerequisites, interaction/failure reporting and what a successful result promises. Then express
the reusable graph and instance views in YAML. Defer cache layouts and build-system internals.
Agent remains the proposed replacement for Session; preserve account, private-context,
Resource-ownership and access requirements while reviewing the vocabulary.
