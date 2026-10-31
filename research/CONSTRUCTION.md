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

The open point is **when the graph's steps execute**:

- While preparing a reusable template, analogous to image construction.
- At each instantiation, using that instance's actual bindings.
- In two explicitly separated phases, if both are intended.

Do not choose among these without clarification. Installing runtime dependencies, authenticating
a particular account and creating Agent-local links are useful probes because their inputs and
lifetimes differ. The earlier construction-ownership proposal is not a prerequisite or ruling.
Shared templates must not accidentally retain an instance's private authentication or live handles.

## YAML and graphs are compatible

Recommend retaining YAML as the initial serialized form for both templates and instances:

| Form | YAML can express | What it does not establish by itself |
| --- | --- | --- |
| Template | Typed settings, references, construction nodes, dependencies and result bindings. | That creation effects have run or that the selected account/service is ready. |
| Instance | Identity, resolved ongoing spec, references to other instances and reported status. | A living process, credential bytes, an open socket or proof of current external readiness. |

The graph can be embedded in template YAML or referenced as a reusable definition. A future
editor or code-based authoring frontend can produce the same typed construction representation.
That is independent of whether the resulting instance can be inspected/exported as YAML.
Do not select another language before examples show what the typed graph cannot express.

Instance inspection may include system-owned status; normal configuration writes still cannot
claim success by submitting status. Registry updates, external establishment and agent execution
remain distinct effects. Template completeness does not imply that loading a file starts work.

A complete ongoing instance spec is not necessarily a reproducible construction recipe. Instance
export/recreation must still account for creation-only choices and required secrets/references.
Changing or deleting a completed instance's source template does not silently change the instance.

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
family or mandatory construction-state ownership scheme is selected here. Settle the reusable
template/graph/instance relationship and execution timing before returning to those details.

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

The immediate question is timing: do plugin graph steps prepare the reusable template, execute
for each instantiation, or occupy explicitly separated preparation/instantiation phases?
Use runtime installation, account authentication and instance-local linking as the examples.

Once clarified, show one complete reusable template and two instances created from it. Only
then settle its authoring syntax, node contracts and minimal execution/recovery evidence.
Agent remains the proposed replacement for Session; preserve the established account, private
context, Resource ownership and access requirements while reviewing the vocabulary.
