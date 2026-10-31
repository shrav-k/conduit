# Conduit

**Active proposal:** [Construction templates and runtime instances](research/CONSTRUCTION.md).
Reserve **Conduit** for ongoing connections and filters. Use a shared construction language
across templates to establish all kinds of instances. **Agent** is the proposed replacement
for Session. Templates are reusable, image-like definitions supporting repeated instantiation;
focused plugin nodes compose their Dockerfile-like graphs. The graph runs on each instance start;
a plugin may verify that its work is already satisfied and finish sooner. Instances retain their
resolved setup graph and runtime configuration in an immutable spec, independent of later source
template changes. Change the template or creation draft to launch a differently configured
instance. Instances can gain or lose Conduit connections through supported operations while
their underlying function stays fixed. Shared context lives in Pool-owned Resources exposed
through Conduits. Caching and build optimizations remain implementation concerns. Implementation
has not started.

The Agent's runtime/account/access boundary and model-selection rule stay fixed. Compatible
models within its declared provider/model domain are selectable by default, with an optional
narrower list. A supported operation changes the current model as runtime state. GPT-to-Claude
or subscription-account changes require a new Agent. Selection requests queue until the runtime
can apply them without interrupting in-flight work; pending and applied choices remain distinct.
Restart persistence and detailed switch-state handling remain under review.

Plugins declare their typed inputs and outputs; graph nodes select plugins and bind values to
those contracts. Output bindings establish dependencies; explicit prerequisites cover ordering
without data transfer. Contracts can declare outputs usable during setup or upon successful
completion, with validity/lifetime rules declared per output. **Graphs establish instances;
instances manage ongoing processes, connections and availability.** Setup plugins can finish
while the established instance continues. Templates declare typed ongoing runtime behavior,
using type-provided defaults; graphs supply prepared inputs. A graph may request startup of
the declared runtime before setup finishes, with the instance managing it from startup.
Required setup failure normally leaves that runtime managed and running, with setup incomplete;
inspection, retry or shutdown is explicit. Unchanged setup retries continue from still-valid
recorded progress. After successful setup, temporary MCP transport failures use bounded automatic
reconnection by default where supported, with a manual option. This does not replay tool writes
or start model work. Consumers normally continue independent work while unavailable connections
are reported. Exact fields, start/wait bindings and recovery limits remain under review. Plugin
implementations can still be local scripts or processes.

| Family | Instance represents |
| --- | --- |
| Agent | A durable model/agent participant with account configuration, private context and executions. |
| Resource | A unitary capability owned by exactly one Pool instance. |
| Pool | The ownership and creation boundary for its Resource instances. |
| Conduit | An ongoing connection with concrete endpoints, filtering and connection state. |

Every family retains template and instance forms. One template can create many independent
instances without being consumed or absorbing their private state; instances retain their own
configuration and observed state. Construction graphs are part of the
common template mechanism. A Conduit template uses that mechanism to establish a connection;
a directory constructor or login procedure is not thereby a Conduit.

The [accepted YAML authoring boundary](research/CONSTRUCTION.md#template-yaml-and-compose-inspired-composition)
keeps a typed template with configuration and an inline graph by default or an explicit graph-source
reference. Exact fields and instance-view details remain open. Compose inspires modularity and
inspection; multi-object composition remains optional
future work. The format is separate from execution: an instance can be running, idle or persistent
without a process.
Resource ownership, account separation, current-access checks and recovery requirements remain
constraints on the new proposal.

Conduit owns its runtime in Rust, with a Pi-inspired terminal and replaceable frontends and
integrations. Provider-native tools, MCP, native skills, trackers and memory backends retain
their useful contracts.

## Read the design

The pre-inquiry design remains saved on `main` at `c013f0a`. This branch is investigating the
construction model; it has not silently rewritten the baseline contracts.

| Document | Responsibility |
| --- | --- |
| [Active construction proposal](research/CONSTRUCTION.md) | Base concepts, template/instance distinction, worked example and open rulings. |
| [SPEC.md](SPEC.md) | Baseline vocabulary, interactions and intended outcomes for comparison. |
| [DESIGN.md](DESIGN.md) | Baseline configuration, account, memory, access and recovery contracts. |
| [REVIEW.md](REVIEW.md) | Active discussion and preserved review checkpoint. |

## Research

| Research | Purpose |
| --- | --- |
| [Establishment and recovery](research/ESTABLISHMENT.md) | Ordering, typed outputs, plugin operations, authentication waits and failure cases. |
| [Resource typing and groups](research/RESOURCES.md) | Core groups, candidate types, capabilities and plugin boundaries. |
| [Conduit connections and state](research/CONDUITS.md) | Repository, communication, credential, AWS and Linear scenarios; shared state and participant lifetimes. |
| [Configuration languages](research/CONFIGURATION.md) | Kubernetes/Docker evidence, configuration-language papers and earlier schema proposals. |
| [Template/instance scenario](research/templates-instances.scenario.yaml) | Earlier synthetic ownership fixture; not validation of the active construction proposal. |
| [Pi connectors and context](research/PI_CONNECTORS.md) | Provider/authentication boundaries, context portability, activation, MCP and Chord. |
| [Pi runtime and Durable](research/PI_RUNTIME.md) | Detailed execution, recovery, frontend, storage and ecosystem evidence. |

Research recommendations remain proposals unless explicitly accepted. Update current documents
in place; Git preserves superseded explorations. Settle contracts and a bounded experiment
before implementation or live account testing.
