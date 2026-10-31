# Conduit

Conduit connects human users, autonomous agents and capabilities through five working families:
**Interface, Agent, Resource, Pool and Conduit**. Each has reusable template and concrete instance
forms. Implementation has not started.

| Family | Responsibility |
| --- | --- |
| Interface | The user-facing authoring, observation and creation layer; opens Conduits to interact with instances, through different frontends. |
| Agent | An autonomous participant that acts through Conduits and can expose operations to other participants. |
| Resource | A unitary capability, data source or service owned by exactly one Pool instance. |
| Pool | The ownership and creation boundary for Resource instances. |
| Conduit | A connection carrying permitted interactions and data between endpoints, with filtering. |

The first **management Interface** has system-wide visibility and management scope in the local
installation: author and inspect templates, compose Pools using Resource templates, instantiate
Pools and Agents with their configured Resources/connections,
and establish Conduits through which it requests supported configuration/lifecycle changes.
Visibility does not permit bypassing that interaction path. Resource creation still goes through
an owning Pool. CLI, terminal UI and test clients should exercise the same interaction contracts; choosing
a frontend must not select a different management implementation.

Templates contain complete establishment instructions and declare what instance configuration
can change and within what parameters. Typed YAML, inline/referenced setup graphs and explicit
plugin dependencies make this inspectable. Instances manage ongoing behavior; Interfaces and
Agents request permitted changes through Conduits. Type/plugin implementations supply the actual
operations, while templates expose and constrain them.

Conduit owns its runtime in Rust, with a Pi-inspired terminal and replaceable frontends and
integrations. Native tools, MCP, skills, trackers and memory backends retain their useful contracts.

## Read the design

These documents now describe the current research-branch model. The pre-inquiry design remains
saved on `main` at `c013f0a`; this branch has not been merged into it.

| Document | Responsibility |
| --- | --- |
| [SPEC.md](SPEC.md) | Five families, user workflow, construction/change contract and intended outcomes. |
| [DESIGN.md](DESIGN.md) | Detailed configuration, controls, ownership, account, memory, access and recovery contracts. |
| [REVIEW.md](REVIEW.md) | Grouped unresolved choices and the next end-to-end walkthrough. |
| [Construction research](research/CONSTRUCTION.md) | Graph/runtime evidence and worked examples supporting the current design. |

## Research

| Research | Purpose |
| --- | --- |
| [Establishment and recovery](research/ESTABLISHMENT.md) | Ordering, outputs, authentication waits and failure cases. |
| [Resource typing and groups](research/RESOURCES.md) | Core groups, candidate types, capabilities and plugin boundaries. |
| [Conduit connections and state](research/CONDUITS.md) | Repository, communication, credential, AWS and Linear scenarios. |
| [Configuration languages](research/CONFIGURATION.md) | Kubernetes/Docker evidence, language papers and schema proposals. |
| [Template/instance scenario](research/templates-instances.scenario.yaml) | Earlier ownership fixture; not validation of the current control contract. |
| [Pi connectors and context](research/PI_CONNECTORS.md) | Provider/authentication boundaries, context portability, activation, MCP and Chord. |
| [Pi runtime and Durable](research/PI_RUNTIME.md) | Execution, recovery, frontend, storage and ecosystem evidence. |

Research recommendations remain proposals unless explicitly accepted. Update current documents
in place; Git preserves superseded explorations. Settle contracts and a bounded experiment
before implementation or live account testing.
