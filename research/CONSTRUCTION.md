# Construction templates and runtime instances

Updated: 2026-10-31. **Active conceptual proposal; detailed contracts remain open.**
This supersedes the branch proposal that Conduits themselves are the universal construction
graphs. Earlier work remains in Git. The pre-inquiry design is preserved on `main` at
[`c013f0a`](https://github.com/shrav-k/conduit/tree/c013f0a20c0c579f511aea84fdc6804b66417b96).
No implementation or live provider operation has been performed.

## Base concepts

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

## Templates combine configuration and construction

A template needs enough information to establish the intended instance, using explicit inputs
and references where necessary. Recommend three parts within its contract:

1. The resulting object's kind/type and configurable settings.
2. Construction steps and their dependencies, implemented through typed plugin operations and
   reusable construction procedures.
3. The result contract: how verified outputs become instance configuration and observed state.

These are responsibilities, not selected field names. An ordinary sequence is a simple graph.
Independent work can be parallel; dependencies determine ordering. This preserves step-by-step
construction without requiring every recipe to be one serial list.

A shared construction language can describe creation of all four families. Different families
and Resource types still have different valid operations, inputs, readiness and output contracts.
Universal syntax must not erase those differences.

Templates can call other templates to establish additional instances, and reuse library
procedures without copying authentication or filesystem logic. A procedure that only checks a
path or obtains a credential need not become a Conduit. Whether reusable procedure fragments
are inline, plugin exports or separately addressable definitions remains open; avoid adding a
fifth object family simply to name every helper.

Plugins implement the effects and expose meaningful waits, outputs, failures and recovery
support. The engine orders work, checks current managed authority, records progress and
coordinates supported continuation. The template can express the orchestration while code
implements browser exchanges, native tool integration, filesystem calls and protocol details.

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

## One concrete walkthrough

For an Euphony Agent, preparation can be expressed as the following construction recipe. This
is semantic pseudocode; names and nesting do not define a YAML schema.

```text
Agent template: euphony-engineer
  settings:
    selected runtime and account
    private-state and activation settings
    permitted Pool and connection selections

  construct:
    prepare private state
    authenticate and verify the selected account
    instantiate a Conduit template for workspace access
    instantiate a Conduit template for permitted MCP access
    wait for required connections to be ready
    finish Agent preparation
```

Each Conduit template has its own construction graph. An MCP-access template can authenticate
a permitted service identity, open the client, install supported restrictions and return an
ongoing Conduit instance. A workspace-access template can resolve Pool-owned directories and
prepare Agent-local projections. Account authentication is an operation in construction; the
continuing MCP or workspace relationship is the Conduit.

A directory Resource template can be much smaller:

```text
Resource template: euphony-repository
  type: {group: filesystem, name: local-directory, version: v1alpha1}
  setting: path = /work/euphony
  construct: verify existing directory → attach through selected owning Pool
  result: Resource instance with verified backing identity and absolute path
```

Resource types retain separate group/name/version fields. Attachment does not create a missing
directory.
A creation recipe uses the distinct fresh-create behavior. Access to the resulting directory
uses an ongoing Conduit; creation itself is not a Conduit instance.

After successful preparation:

| Instance | What remains |
| --- | --- |
| Agent `euphony-engineer-a7f2` | Its identity, resolved configuration, private context and execution state. Preparation need not start a model turn. |
| Pool `euphony` | Ownership of its Resource instances. |
| Resource `euphony-repository-r4d8` | The registered capability and backing reference; it need not run a process. |
| Conduit `workspace-access-c12e` | The Agent-to-Pool/Resource relationship, current selection and projection state. |
| Conduit `linear-access-b91c` | Its participants, permitted service identity, client state and supported filtering. |

The construction recipes have completed. The Agent, Pool, Resources and Conduits remain according
to their lifecycles. The mechanism performing construction is not repurposed as the ongoing
filter. Transport or authentication can later fail without implying that construction must be
blindly replayed or the model automatically restarted.

## All running things are instances

Use instance identity for every independently managed Agent, Resource, Pool and Conduit.
Instances can also be stopped, idle, failed or persistent without a process. Therefore the rule
is not that every instance must be running, nor that YAML status starts or maintains a process.

One boundary needs a ruling: does a construction execution itself become a separate first-class
instance, or is it tracked under the instance being established?

**Recommendation:** each template targets one primary instance. The engine reserves/tracks that
identity during construction and records pending work, waits, outcomes and partial results under
it. A template can invoke other templates to establish additional instances with their own
identity and ownership. It does not need a fifth top-level workflow instance merely to establish
its main result. A failed setup can remain an inspectable failed/unready instance; exact registry
and rollback semantics need definition before adoption.

Nodes, workers and helper processes must still have tracked execution state and cleanup rules.
Making them supporting execution state avoids requiring users to manage a Resource object for
every authentication poll, socket or symlink. A separately addressable capability with its own
lifecycle should be an appropriate instance, with Resources created through their owning Pool.
The alternative is a first-class construction-run instance distinct from the objects it creates;
it may suit arbitrary multi-output workflows but adds identity and lifecycle choices.

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

Resolve the foundation before adding more lifecycle fields:

1. Confirm the four families and whether Agent is a naming change to the durable Session concept.
2. Decide whether a template constructs one primary instance with tracked subordinate construction,
   or produces objects from a separate first-class construction-run instance.
3. Work through one template's settings, construction graph and resulting instance representation.
   Define reusable procedures and template calls without forcing duplicated configuration.
4. Then define construction completion, continuing readiness, reauthentication/reconfiguration and
   cleanup. These should follow the object and connection contracts, not make every object a workflow.
