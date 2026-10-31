# Conduit

**Active proposal:** [Construction templates and runtime instances](research/CONSTRUCTION.md).
Reserve **Conduit** for ongoing connections and filters. Use a shared construction language
across templates to establish all kinds of instances. **Agent** is the proposed replacement
for Session. Templates are reusable, image-like definitions supporting repeated instantiation;
focused plugin nodes compose their Dockerfile-like graphs. The graph runs on each instance start;
a plugin may verify that its work is already satisfied and finish sooner. Caching and build
optimizations remain implementation concerns. Implementation has not started.

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

YAML can represent both template configuration/graphs and instance spec/status. The format
is separate from execution: an instance can be running, idle or persistent without a process.
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
