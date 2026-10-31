# Conduit

Conduit connects human users, autonomous agents and capabilities through five working families:
**Interface, Agent, Resource, Pool and Conduit**. Each has reusable template and concrete instance
forms. Implementation has not started.

**Pools establish shared capabilities and normally manage their service authentication once.
Agents use those capabilities through Conduits that narrow access for each caller. Interfaces
provide unrestricted local administration in v0.**
This keeps service setup reusable across models without distributing provider credentials or
repeating the same provider login for every Agent. Private Agent state and model-account selection
remain separate.

| Family | Responsibility |
| --- | --- |
| Interface | The user-facing authoring, observation and creation layer; opens Conduits to interact with instances, through different frontends. |
| Agent | An autonomous participant that acts through Conduits and can expose operations to other participants. |
| Resource | A unitary capability, data source or service owned by exactly one Pool instance. |
| Pool | Owns and establishes Resource instances and organizes their shared service access/authentication. |
| Conduit | A connection carrying permitted interactions and data between endpoints, with filtering. |

The v0 **management Interface** is the user's unrestricted administration surface across all
templates and instances: inspect, author, instantiate, connect, configure, stop, remove and repair.
Interface permissions and narrower/multiple-Interface policies are deferred while the model is
developed. Normal interaction uses Conduits and runtime-supported operations; Resource creation
still goes through an owning Pool. CLI, terminal UI and test clients use the same contracts.
Changing an instance's fixed setup graph or control definitions requires a replacement in v0,
including through the Interface. Supported settings, connections and lifecycle remain manageable.
R1 is settled; [six review bundles remain](REVIEW.md#remaining-decision-bundles), beginning with
[graph execution semantics](REVIEW.md#next-discussion). Implementation details and future
extensions are tracked separately.

Templates contain complete establishment instructions and declare what instance configuration
can change and within what parameters. Typed YAML, inline/referenced setup graphs and explicit
plugin dependencies make this inspectable. Instances manage ongoing behavior; Interfaces and
Agents request permitted changes through Conduits. Type/plugin implementations supply the actual
operations, while templates expose and constrain them.

For example, an Euphony Pool owns a reusable authentication Resource and a Linear MCP Resource
that accesses it through a Conduit. Several Agents attach to Linear through separately filtered
Conduits, reusing that managed login. An external Agent can see Linear's native
tools through a Conduit-managed MCP endpoint; our own runtime can mediate internally. Preserving
the provider contract does not require a direct provider connection. Detaching an Agent preserves
the shared Resource and login for the others.

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
| [Conduit connections and state](research/CONDUITS.md) | Shared Pool access through repository, communication, credential, AWS and Linear scenarios. |
| [Configuration languages](research/CONFIGURATION.md) | Kubernetes/Docker evidence, language papers and schema proposals. |
| [Pi connectors and context](research/PI_CONNECTORS.md) | Provider/authentication boundaries, context portability, activation, MCP and Chord. |
| [Pi runtime and Durable](research/PI_RUNTIME.md) | Execution, recovery, frontend, storage and ecosystem evidence. |

Research recommendations remain proposals unless explicitly accepted. Update current documents
in place; Git preserves superseded explorations. Settle contracts and a bounded experiment
before implementation or live account testing.
