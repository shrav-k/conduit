# Conduit

Conduit has four concept families: **Session, Resource, Pool and Conduit**. Each has a
**template** form describing reusable configuration and an **instance** form representing
something established. An instance can persist without a running process.

| Family | Template | Instance |
| --- | --- | --- |
| Session | Model/agent/account defaults, activation instructions, limits and connection choices. | A durable participant with private context and executions. |
| Resource | How to create or attach data or a capability. | A concrete capability owned by exactly one Pool instance. |
| Pool | Resource provisioning selections, defaults and policies. | The ownership and creation boundary for its Resource instances. |
| Conduit | Interaction, endpoint and filtering configuration for a flow. | A live filtered flow with logical consumer/peer attachments and connection state. |

Resource instances are created only through a Pool. Templates can be reused across Pools;
the same Resource instance cannot belong to several Pools. Separate instances may attach
to the same external data or create independent backing data, explicitly according to their
contract. Connections use Resources through their owning Pools, including any storage they
need for communication. They can request provisioning through a Pool but do not own Resources.

Each Resource has one responsibility and its related operations; Pools group Resources into
useful sets. Storage, search and MCP access are separate Resources by default, even when they
use one backend. Resource-to-Resource access also uses Conduits: for example, a search Resource
reads a storage Resource through a logical connection. Minimize duplicated configuration and
state within those explicit boundaries. Define Resource templates
and instances first, then how Pools group and coordinate them. Pool creation and Resource
provisioning remain explicit, distinct effects. Persistent Pool declarations and their binding
mechanics are back under review rather than prerequisites for defining Resources.

Session and Resource templates can reference Conduit templates to describe connections to
establish for their instances; the enclosing instance automatically supplies the consumer
endpoint. The engine and plugins implement the flow; a Conduit does not
have its own independent runtime. It can transfer data once, maintain access or synchronize
over time. Explicit consumer preparation normally attempts declared open/join actions and
reports their outcomes; registration alone starts no flow. Opening a new Conduit and joining
an existing one remain explicit, distinct intents; sharing does not imply shared credentials/private context.
Logical consumers/peers keep a live Conduit attached, including between sync polls or during
temporary transport failure. When the last one detaches, the flow ends and remaining
provider-side connections close. Templates, backing Resources and retained evidence survive.
Session templates supply the activation behavior previously called a Profile; instances retain
it in their own configuration. Profile is no longer a separate family. An **attachment** is a
consumer or peer's logical participation in a Conduit, not another object family.

**Retain YAML; define the language through typed objects and explicit semantics.** Each object
uses its family as `kind` and a top-level `form` to identify a template or instance. Names are
unique per family across both forms: templates name reusable concepts, while instances have
names generated from the template name plus a unique suffix unless an explicit unique name
is supplied. Readable object references include `kind`, `form` and `name`. Create instances
either by expanding a template and editing ordinary instance fields or directly from complete
instance YAML. Direct creation requires an explicit unique name; Resource creation always
goes through a Pool.
The complete instance `spec` is then edited directly. Draft assignments replace whole field
values; editing a nested child preserves its siblings. Creation and replacement updates store
declared schema defaults explicitly; templates preserve authored omissions. Template history
is optional; continuing Resource and authority dependencies are explicit on the instance.
Creation and replacement require the applicable schemas. Unknown configuration fields are
rejected except within schema-declared open maps. Otherwise valid configuration can be
registered before its runtime/services are available, with readiness reported separately.
Current-access checks and actual execution evidence remain required.

Conduit owns its runtime in Rust, with a custom Pi-inspired terminal first and replaceable
frontends and integrations. Provider-native tools, MCP, native skills, trackers and memory
backends retain their useful contracts. Implementation has not started.

Read in order:

1. [SPEC.md](SPEC.md) — vocabulary, interactions and intended outcomes.
2. [DESIGN.md](DESIGN.md) — accepted contracts, including retained account/memory/access rules.
3. [REVIEW.md](REVIEW.md) — grouped questions, walkthroughs and the initial experiment.

| Research | Purpose |
| --- | --- |
| [Resource typing and groups](research/RESOURCES.md) | Extensible type contracts, candidate core groups, capabilities, plugin boundaries and scenario-based review. |
| [Conduit connections and state](research/CONDUITS.md) | Repository, communication, credential, AWS and Linear scenarios; shared state and participant lifetimes. |
| [Configuration languages](research/CONFIGURATION.md) | Kubernetes/Docker evidence, configuration-language papers and candidate expression of the four families. |
| [Template/instance scenario](research/templates-instances.scenario.yaml) | Synthetic review data: templates, explicit operations and expected ownership. Not an accepted configuration/API schema. |
| [Pi connectors and context](research/PI_CONNECTORS.md) | Provider/authentication boundaries, context portability, activation, MCP and Chord. |
| [Pi runtime and Durable](research/PI_RUNTIME.md) | Detailed execution, recovery, frontend, storage and ecosystem evidence. |

Update these documents in place; Git history preserves earlier designs. Research proposals
are not requirements. Settle the configuration contracts and experiment before implementation.
