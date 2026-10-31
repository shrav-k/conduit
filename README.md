# Conduit

Conduit has four concept families: **Session, Resource, Pool and Conduit**. Each has a
**template** form describing reusable configuration and an **instance** form representing
something established. An instance can persist without a running process.

| Family | Template | Instance |
| --- | --- | --- |
| Session | Model/agent/account defaults, activation instructions, limits and connection choices. | A durable participant with private context and executions. |
| Resource | How to create or attach data or a capability. | A concrete capability owned by exactly one Pool instance. |
| Pool | Resource provisioning selections, defaults and policies. | The ownership and creation boundary for its Resource instances. |
| Conduit | Endpoints, interaction contract, filters and supporting-resource requirements. | An opened **connection**, with participants and operational state. |

Resource instances are created only through a Pool. Templates can be reused across Pools;
the same Resource instance cannot belong to several Pools. Separate instances may attach
to the same external data or create independent backing data, explicitly according to their
contract. Connections use Resources through their owning Pools, including any storage they
need for communication. They can request provisioning through a Pool but do not own Resources.

Opening a connection from a Conduit template and joining an existing connection are explicit,
distinct intents. Sharing a connection does not implicitly share credentials or private context.
Session templates contain the activation behavior previously called a Profile; Profile is no
longer a separate family. **Connection** names a Conduit instance, not an additional family.

**Retain YAML; define the language through typed objects and explicit semantics.** Each object
uses its family as `kind` and a top-level `form` to identify a template or instance. Names are
unique per family across both forms: templates name reusable concepts, while instances have
names generated from the template name plus a unique suffix unless an explicit unique name
is supplied. Readable object references include `kind`, `form` and `name`. Instances retain
resolved behavioral/provisioning settings until explicitly reconfigured; access limits stay
live. Creation and reconfiguration retain the source template ID/version, explicit inputs and
overrides, and resolved settings. Source-template content retention and family-specific
lifecycle contracts are the next design work.

Conduit owns its runtime in Rust, with a custom Pi-inspired terminal first and replaceable
frontends and integrations. Provider-native tools, MCP, native skills, trackers and memory
backends retain their useful contracts. Implementation has not started.

Read in order:

1. [SPEC.md](SPEC.md) — vocabulary, interactions and intended outcomes.
2. [DESIGN.md](DESIGN.md) — accepted contracts, including retained account/memory/access rules.
3. [REVIEW.md](REVIEW.md) — grouped questions, walkthroughs and the initial experiment.

| Research | Purpose |
| --- | --- |
| [Conduit connections and state](research/CONDUITS.md) | Repository, communication, credential, AWS and Linear scenarios; shared state and participant lifetimes. |
| [Configuration languages](research/CONFIGURATION.md) | Kubernetes/Docker evidence, configuration-language papers and candidate expression of the four families. |
| [Template/instance scenario](research/templates-instances.scenario.yaml) | Synthetic review data: templates, explicit operations and expected ownership. Not an accepted configuration/API schema. |
| [Pi connectors and context](research/PI_CONNECTORS.md) | Provider/authentication boundaries, context portability, activation, MCP and Chord. |
| [Pi runtime and Durable](research/PI_RUNTIME.md) | Detailed execution, recovery, frontend, storage and ecosystem evidence. |

Update these documents in place; Git history preserves earlier designs. Research proposals
are not requirements. Settle the configuration contracts and experiment before implementation.
