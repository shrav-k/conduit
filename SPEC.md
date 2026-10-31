# Conduit specification

Updated: 2026-10-31. Current working model on `research/conduits-as-graphs`; implementation has
not started. The pre-inquiry design remains on `main` at `c013f0a`. [DESIGN.md](DESIGN.md) holds
detailed contracts; [REVIEW.md](REVIEW.md) groups remaining decisions. Research is supporting
evidence and proposed syntax, not an additional source of requirements.

## Working model

Conduit connects human users, autonomous actors and capabilities through five families:
**Interface, Agent, Resource, Pool and Conduit**. Each has a reusable **template** and concrete
**instance** form. One template can create many instances. Instance identity, native process,
transport connection and underlying data are different; not every instance runs a process.

| Family | Responsibility |
| --- | --- |
| Interface | User-facing authoring, observation and creation; establishes Conduits for interacting with running targets. |
| Agent | Autonomous participant with runtime/access configuration, private context and executions; may initiate or receive operations. |
| Resource | One responsibility with related operations, exposed as a particular capability, data source or service. |
| Pool | Owns and creates/attaches its Resource instances; composes reusable Resource templates. |
| Conduit | Carries permitted interactions/data between endpoints, applying filtering and maintaining connection state. |

Agent replaces the former Session name. Plugins implement typed operations and runtime behavior;
graph nodes compose setup. Plugins, executions and attachment records do not introduce another
object family. Resources may run services without being autonomous Agents. Interfaces convey
human authority; they are not themselves the human identity.

## The user workflow

The default Interface is the primary entry point, with the broadest local management scope and
visibility across configured templates and instances. CLI, terminal UI and automated test clients
use the same contracts.

1. **Author templates.** Create/edit Resource and Agent templates, including their establishment
   instructions, configuration and permitted changes.
2. **Compose Pools.** Select Resource templates and creation settings in a Pool configuration.
3. **Establish Pools and Resources.** Register/instantiate and explicitly run the configured
   establishment lifecycle. An Interface may combine these actions while reporting separate outcomes.
4. **Establish Agents.** Instantiate Agent templates with Conduits to the relevant Pools/capabilities.
5. **Observe and interact.** Observe state; establish a Conduit to a running target to request its
   supported configuration changes, lifecycle actions or other operations.

Visibility does not grant a direct instance-edit path. Agents use the same Conduit-mediated
interaction principle. Broad Interface access does not manufacture unsupported provider controls.
Bootstrap of the first Interface/management connection and precise frontend attachment remain open.

## Templates are complete construction and change contracts

**Accepted direction:** a template contains all instructions needed to establish its instance and
specifies which configuration can change afterward and within what parameters. Completeness can
use explicit references to versioned type/plugin contracts and graphs; it does not require
embedding code, credentials or backing data in YAML.

| Part | Required meaning |
| --- | --- |
| Establishment | Initial settings, create/attach intent, dependencies and an inline or referenced setup graph. |
| Ongoing behavior | The typed runtime behavior the instance manages after and, where needed, during setup. |
| Allowed changes | The exposed configurable surface and its valid parameters/conditions, within implementation support. |
| Observations | How the system reports actual configuration, execution and operation outcomes separately from intent. |

Plugins declare typed inputs/outputs. Graph bindings imply dependencies; explicit prerequisites
cover ordering without data. Graphs run at instance start, while plugins may verify already
satisfied work. A graph can request instance-managed service startup before later setup steps.
Instances manage ongoing processes and availability; setup nodes do not become permanent supervisors.

Instances retain their resolved graph, concrete configuration and applicable change contract
independently of source-template edits/deletion. There is no required template-history or override
archive. Current authority and Resource data remain live. Changes to an existing instance use its
permitted controls through a Conduit; the template does not silently propagate edits to it.

Template intent, instance configuration, requested changes and observed state must remain
distinguishable. Blanket instance immutability is replaced by explicit change contracts. Exact
mutable fields, omission defaults, change-handler syntax, restart/replacement categories, contract
migration and stored requested/applied representation remain for grouped review. This is neither
a promise that every setting is mutable nor a generic unrestricted spec-patch API.

## Conduits expose permitted interactions

The target's effective contract gives a Conduit an inspectable surface: supported operations,
inputs, constraints and observations. The Interface or Agent requests a change through the
connection; the target validates and applies it using its implementation and reports the outcome.
Provider-specific scheduling and recovery stay in those contracts.

Filters narrow authority and target capabilities. An allowed operation name alone does not
establish that every parameter is allowed. Discovery, dispatch and current-state validation must
agree. The proposed constraint composition is intersection of implementation support, template
limits, current authority and Conduit restrictions; exact schema and expression language are open.
A filter does not silently rewrite the requested value or grant access.

Conduits cover Agent-to-Agent and Resource-to-Resource interaction as well as user control and
Pool access. Reusing a template does not choose between opening a new flow and joining an existing
one. The enclosing consumer supplies its endpoint identity for declared connections. Explicit
preparation attempts configured connections, required by default; optional failures report and
continue. Partial failure retains successful attachments for retry; explicit cancellation detaches.

Logical consumers/peers keep a flow alive, including between polls or during a temporary socket
failure. The last departure ends the flow and closes remaining provider-side connections. This
does not delete backing data, undo delivered content or prove cancellation of external work.

Current access applies at execution and delivery, including retries and queued changes. Report
unavailable or unsupported operations. A response acknowledging a request is distinct from its
application; uncertain effects remain unknown until reconciliation or a supported safe retry.

## Resource ownership and creation

Every Resource instance is created/attached through exactly one Pool and remains owned by it.
Resource templates are reusable definitions with no exclusive Pool owner. Different Pools may
instantiate the same template, either creating distinct backing or explicitly attaching to shared
external data. Separate registry objects do not prove separate credentials, quotas or write targets.

Keep Resources unitary: filesystem, storage, search and integration are working groups; storage,
search and MCP access are separate Resources by default even when supplied by one backend.
Local-directory and git-worktree are distinct types. Groups are independent of plugin packaging;
content, identity and agent-communication groups remain candidates.

A folder Resource describes backing directory access; each Agent's symlink is preparation state.
MCP service identity may differ per attachment where supported, separately from model subscriptions.
Resource-to-Resource access uses Conduits. Activation instructions belong to the consumer; Pool
membership and availability alone do not load skills/documents into model context.

Conduits needing durable content use Pool-owned Resources. Routing state need not be a Resource
per cursor. Pool ownership does not grant data access or authorize deletion of backing files.
Retirement, transfer and orphan handling need explicit lifecycle semantics, never implicit purge.

## Configuration language

YAML is the authoring surface; typed objects and explicit semantics define the protocol.
The common envelope is `apiVersion`, family `kind`, top-level `form`, `metadata`, `spec`, plus
system-owned `status` for inspection. Exact API version and new control fields remain open.

Resource type references use separate `group`, `name`, `version`, with typed `config` and
consumer `conduits`. Type-contract versions differ from common API versions and plugin releases.
The allowed-change declaration extends the behavioral contract; its exact field layout is not
selected. A schema annotation by itself is not runtime enforcement.

Retain [registry/authoring contracts](DESIGN.md#configuration-language):

- Stable IDs, names unique per family across forms, generated template-name instance suffixes,
  readable kind/form/name references and explicit rebind of established identity.
- Explicit registration/export; file edits do not update active objects. Ordinary draft fields
  use replacement/explicit child edits rather than an implicit deep merge or parameter DSL.
- Materialized instance defaults, strict known schemas, and separate validation versus operational
  readiness. Missing runtime/service can be reported without inventing success.
- Checked template edits with separate ID/version preconditions and recoverable edit baselines.
  These are not a bypass for running-instance controls. Their runtime concurrency counterpart
  remains under review.
- Status is not authored success. Configuration exports omit system-owned state. Registration
  removal, execution shutdown and backing-data purge have distinct contracts.

## Implementation direction and retained behavior

Conduit owns its runtime in Rust, with a custom Pi-inspired terminal and replaceable frontends.
Pi/Durable are references; native-agent adapters separately report supported control, context,
authentication and telemetry. BYO skills use compatible native ecosystems and tooling.

MCP is the preferred external capability protocol; preserve native provider operations and schemas.
Linear owns its tracking workflow, and memory/search/graph backends remain plugins. Configuration
and logical mediation do not imply complete interception of native host access in v0.

Native execution remains the default, with optional or template-required isolation. Verify separate
account/private state for two Claude and two Codex/OpenAI subscriptions before claiming support.
Retain current-access checks, portable context, scoped memory/provenance, bounded search, direct
read-only sharing, safe workspace cleanup, retention and usage coverage in [DESIGN.md](DESIGN.md).

## Acceptance scenarios

These are intended outcomes, not completed tests. [REVIEW.md](REVIEW.md#initial-experiment) defines
remaining choices and staged evidence.

| Scenario | Observable outcome |
| --- | --- |
| Interface workflow | Author templates, compose/establish a Pool, instantiate Agents, connect and observe through one shared CLI/UI/test contract. |
| Repeated construction | One template creates distinct instances; source edits do not alter existing configuration/control contracts. Resource ownership is explicit. |
| Allowed change | A declared setting changes through a permitted Conduit, with valid parameters and observable application. |
| Rejected change | Type/template constraints, current authority or a narrower Conduit restriction reject an invalid request without silently transforming it. |
| Transition and uncertainty | Valid values can still have an unsupported transition; pending and unknown outcomes are not reported as applied. |
| Resource data versus configuration | Writing a note, changing a search setting and attaching a Conduit use their respective contracts. |
| Recovery | Retain valid setup progress, reconcile uncertain effects, avoid duplicate execution and do not replay unsafe writes. |
| Native coverage | Unsupported hot control or enforcement is reported; no fabricated provider compatibility. |

## Design boundary

The five families, template/instance forms, Pool ownership, complete construction templates and
template-declared change boundaries are settled direction. Concrete control schemas, permitted
contract evolution, Conduit filtering/composition, lifecycle transitions and communication topology
remain open. Resolve them with complete examples rather than provider-specific micro-rulings.
