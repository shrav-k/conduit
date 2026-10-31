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

**The normal service-access model is establish once in a Pool, then share through restricted
Conduits.** Pools organize service authentication with their owned Resources. Each authorized
Agent uses a separately attributable, filtered relationship; it need not repeat the provider login.
Shared service identity does not merge private Agent context or model-subscription accounts.

| Family | Responsibility |
| --- | --- |
| Interface | User-facing authoring, observation and creation; establishes Conduits for interacting with running targets. |
| Agent | Autonomous participant with runtime/access configuration, private context and executions; may initiate or receive operations. |
| Resource | One responsibility with related operations, exposed as a particular capability, data source or service. |
| Pool | Owns and establishes Resource instances, composes reusable templates and organizes shared service authentication/access. |
| Conduit | Carries permitted interactions/data between endpoints, applying filtering and maintaining connection state. |

Agent replaces the former Session name. Plugins implement typed operations and runtime behavior;
graphs compose procedures. Handlers can use a typed plugin operation or a graph; instances own
ongoing behavior. Plugins, handlers, executions and attachment records do not introduce another
object family. Resources may run services without being autonomous Agents. Interfaces convey
human authority; they are not themselves the human identity.

## The user workflow

The default Interface is the primary entry point, with the broadest local management scope and
visibility across configured templates and instances. CLI, terminal UI and automated test clients
use the same contracts.

Initial local startup seeds a system Pool owning a management Resource, the default Interface,
and a management Conduit connecting them. A small runtime bootstrap establishes this initial set;
subsequent template/registry management and instance creation use that managed path. No application
Pool or external login is required first. Resource creation still goes through its owning Pool.

V0 establishes one local registry owner from the initializing OS-user context, without a separate
Conduit login. The default Interface acts for that owner. Agents receive explicit delegated access
through their own Conduits; sharing the owner's OS account does not itself constitute that grant.
Credential handling and native-versus-isolated enforcement remain separate from this authority model.

The default Interface is a reusable instance with identifiable CLI, terminal and test-client
attachments, including multiple clients at once. Separate Interface instances remain available
for different configurations. Attachment grants no additional authority and does not override
target-specific exclusive-control rules or require an always-running process.

1. **Author templates.** Create/edit Resource and Agent templates, including their establishment
   instructions, configuration and permitted changes.
2. **Compose Pools.** Select Resource templates, establishment settings and intended shared service
   identities; supply concrete targets for the templates' declared connection dependencies.
3. **Establish Pools and Resources.** Register/instantiate and explicitly run the configured
   establishment lifecycle, completing needed service authentication. An Interface may combine these
   actions while reporting separate outcomes. Existing valid authentication can be reused.
4. **Establish Agents.** Instantiate Agent templates and authorize their Conduits to the relevant
   Pools/capabilities. Attaching a new Agent does not inherently trigger another provider sign-in.
5. **Observe and interact.** Observe state; establish a Conduit to a running target to request its
   supported configuration changes, lifecycle actions or other operations.

Visibility does not grant a direct instance-edit path. Agents use the same Conduit-mediated
interaction principle. Broad Interface access does not manufacture unsupported provider controls.
Owner/client binding, bootstrap recovery/protection, frontend transport and target-control mapping
remain open.

## Templates are complete construction and change contracts

**Accepted direction:** a template contains all instructions needed to establish its instance and
specifies which configuration can change afterward and within what parameters. Completeness can
use explicit references to versioned type/plugin contracts and graphs; it does not require
embedding code, credentials or backing data in YAML.

| Part | Required meaning |
| --- | --- |
| Establishment | Initial settings, create/attach intent, dependencies and an inline or referenced setup graph. |
| Ongoing behavior | The typed runtime behavior the instance manages after and, where needed, during setup. |
| Allowed changes | Named typed controls, their handlers and valid parameters/conditions, within implementation support. |
| Observations | How the system reports actual configuration, execution and operation outcomes separately from intent. |

Permitted configuration changes are exposed as named typed controls backed by handlers. Templates
normally reuse their selected type's declared default controls, with optional narrowing or explicit
selection of other supported controls. Reuse includes schemas and implementations; the instance
retains its resolved control surface, so plugin updates do not silently add controls. This is distinct
from the accepted discovery of dynamic provider-native tool catalogs.
A frontend can present ordinary field editing while invoking the relevant control through a
Conduit. This requires no separate user-authored operation-request document.

MCP is the standard external interaction for named controls: discoverable tools carry typed inputs,
explanatory content and plugin-defined structured results. Reuse supported MCP interaction and
observation mechanisms where applicable. Application behavior remains extensible; MCP message
types do not impose an exhaustive Conduit-wide domain lifecycle enum. Plugins describe their own
requirements and follow-up behavior. This neither requires MCP between internal Rust components
nor changes provider-native tool preservation. Exact mappings and client support remain to be verified.

Plugins declare typed inputs/outputs. Graph bindings imply dependencies; explicit prerequisites
cover ordering without data. Graphs run at instance start, while plugins may verify already
satisfied work. A graph can request instance-managed service startup before later setup steps.
Instances manage ongoing processes and availability; setup nodes do not become permanent supervisors.

Instances retain their resolved graph, concrete configuration and applicable change contract
independently of source-template edits/deletion. There is no required template-history or override
archive. Current authority and Resource data remain live. Changes to an existing instance use its
permitted controls through a Conduit; the template does not silently propagate edits to it.
In v0, the retained setup graph and control definitions are fixed. Changing their steps, schemas,
constraints or handler bindings requires a new instance from a revised definition. Declared
settings, dynamic attachments, live access restrictions and provider catalogs retain their agreed
behavior. Plugin binary-version and compatible implementation-update policies remain separate.

Template intent, instance configuration, requested changes and observed state must remain
distinguishable. Configuration changes advance the instance's stored settings only after the
handler confirms application. Pending or uncertain requests remain separate; a last confirmed
value is not presented as currently active when the outcome is unknown. Plugins define evidence
of application. Initial creation settings likewise do not prove setup has run.

Configuration changes apply one at a time per target instance by default, across all its Conduits.
A type/plugin contract may allow safe parallel application. Ordinary Resource use follows its own
rules; queueing versus rejection remains plugin-defined. Ordering alone does not detect stale intent.
Controls normally require the configuration revision the caller inspected, checked at application.
A mismatch reports a conflict; callers can reread and reconcile. A control contract may explicitly
allow safe unconditional operations. Inspection and clients can carry the revision without human
confirmation; exact revision scope and MCP representation remain open.

Blanket instance immutability is replaced by explicit change contracts. Exact mutable fields,
change-handler syntax, plugin application behavior, revision mechanics and storage
layout remain for grouped review. This is neither a promise that every setting is mutable nor a
generic unrestricted spec-patch API.

## Conduits expose permitted interactions

The target's effective contract gives a Conduit an inspectable surface: supported operations,
inputs, constraints and observations. The Interface or Agent requests a change through the
connection; the target validates and applies it using its implementation and reports the outcome.
Provider-specific scheduling and recovery stay in those contracts.

Integration capability contracts can define discovery and invocation while providers supply their
concrete operation catalogs and schemas dynamically. For MCP, expose permitted native tools;
do not require a separate Conduit contract for every provider operation or a generic model-facing
wrapper. Static operation contracts remain valid for other capabilities. Discovery supplies
available behavior, not additional permission or a changed instance-control contract.

Filters narrow authority and target capabilities. An allowed operation name alone does not
establish that every parameter is allowed. Discovery, dispatch and current-state validation must
agree. Every applicable restriction along the path used for a request must pass, including target
support/limits, current authority and composite/component restrictions. Independent authorized paths
remain complete alternatives: target, operation and parameter constraints stay associated within
each path. Reading Euphony through one path and updating Demo through another does not permit
updating Euphony. Actor-wide and target-wide limits still apply wherever relevant. Exact filter
syntax and routing remain open. A filter does not silently rewrite a request or grant access.

Filters normally use typed declarations with defined semantics, backed by supporting plugins for
validation and enforcement. Custom code can implement new filter contracts, including through
scripts or graphs; restrictions remain inspectable in configuration. A declaration alone does not
prove enforcement. Exact filter schemas, operators and expression-level composition remain open.

An authorized user or Agent can delegate an explicit subset of service access while establishing
another consumer's Conduit attachment. Record that delegation with the attachment; a separate
Agent-permission setup is not required first. The creator must be permitted to delegate it, and
target/recipient limits still apply. Merely declaring requested access, naming a Resource or
editing a filter grants nothing. Delegation does not inherently authorize onward delegation.

Conduits cover Agent-to-Agent and Resource-to-Resource interaction as well as user control and
Pool access. Reusing a template does not choose between opening a new flow and joining an existing
one. The enclosing consumer supplies its endpoint identity for declared connections. Explicit
preparation attempts configured connections, required by default; optional failures report and
continue. Partial failure retains successful attachments for retry; explicit cancellation detaches.

Reusable Conduit templates can compose other Conduits. An Agent can declare an access bundle such
as `euphony-access`, retaining each component's targets, authority, explicit open/join intent and
lifecycle. The bundle has a Conduit instance; it introduces no object family or mandatory process.
Each consumer's composite attachment tracks its own component participation for access, readiness
and cleanup. Detaching a consumer releases only its participation; component Conduits, clients and
authentication can still be shared. Supporting Resources remain Pool-owned. Exact attachment
representation, aggregate readiness and component reuse rules remain open.

Logical consumers/peers keep a flow alive, including between polls or during a temporary socket
failure. The last departure ends the flow and releases its owned connections/leases. This
does not delete backing data, undo delivered content or prove cancellation of external work.

Current access applies at execution and delivery, including retries and queued changes. Report
unavailable or unsupported operations. A response acknowledging a request is distinct from its
application; uncertain effects remain unknown until reconciliation or a supported safe retry.

## Shared Pool access and authentication

Reusable authentication is a Pool-owned Resource. Service Resources access it through Conduits
rather than independently owning copies of the same login/refresh state. It manages supported
provider bindings: associations to authorized identities, not raw credentials in YAML. Plugins
implement authentication; the owning instance manages its ongoing state. A Pool template can
compose the authentication and service Resources automatically. This adds no object family or
requirement for a Resource per token, and makes the Pool neither a mandatory network hop nor process.

Reusable Resource templates declare required connections; Pool composition supplies their concrete
targets during instantiation. Each resulting instance retains its explicit resolved Conduit
bindings. The same service template can therefore use work authentication in one Pool and personal
authentication in another. This does not imply automatic account matching, additional permission,
late rebinding or a second editable connection inventory. Exact binding syntax remains open.

Pool templates use named local Resource entries and can bind dependencies to those entries.
Each Pool instantiation resolves them to its own Resource instances and their stable IDs; local
names are composition references, not registry aliases. Reusing a Pool template does not imply
shared Resource identity or credentials. Normal Pool setup uses a reusable graph step that consumes
the entries and bindings and establishes Resources through their own setup contracts. Custom graphs
can reference the same entries for additional ordering or checks. This runs only during explicitly
requested setup. Exact field names, plugin contracts and graph syntax remain open.

During partial Pool setup, ready Resources remain usable through authorized Conduits while other
components wait or fail. Each Resource's prerequisites and declared gates still apply; report the
Pool's incomplete setup without withholding unrelated ready capabilities. This preserves partial
work under the existing recovery rules and does not equate a running process with readiness.

Separate these contracts:

- **Provider identity:** what the service authorizes, centrally managed for the capability.
- **Caller authority:** which Agent/Interface may use it, for which operations and targets.
- **Conduit restrictions:** further narrowing for that particular relationship.
- **Transport state:** client/socket reuse, independent of whether the service identity is shared.

Use through a Conduit does not distribute the upstream credential. Pool membership, knowing an
endpoint or discovering a tool does not itself grant access. Provider, Resource, caller and
Conduit limits must all hold. Provider-specific identities per attachment remain an explicit
exception where required or selected; one account or client is not mandated for every Resource.
One authentication Resource can support several service Resources with provider-appropriate
bindings; a shared account does not imply that the same token works for every service. Exact type
names, operations, connection binding syntax and the treatment of non-reusable internal state
remain open.

Revoking one Agent's access leaves other permitted consumers and the shared login intact. Loss
of the shared login can affect all its users. Ending a Conduit cleans up what that flow owns,
not Resource-owned credentials or clients used by other flows. Provider logs may attribute work
to the shared service identity; Conduit retains the actual caller/connection attribution.

Preserve provider-native MCP tools through supported mediation: external clients can connect to
a Conduit-managed provider endpoint, while owned runtimes can mediate internally. Neither requires
a generic replacement tool or one combined gateway. Direct provider paths have only their verified
native controls; the design does not claim universal interception in v0.

## Resource ownership and creation

Every Resource instance is created/attached through exactly one Pool and remains owned by it.
Resource templates are reusable definitions with no exclusive Pool owner. Different Pools may
instantiate the same template, either creating distinct backing or explicitly attaching to shared
external data. Separate registry objects do not prove separate credentials, quotas or write targets.

Keep Resources unitary: filesystem, storage, search and integration are working groups; storage,
search and MCP access are separate Resources by default even when supplied by one backend.
Local-directory and git-worktree are distinct types. Groups are independent of plugin packaging;
content, identity and agent-communication groups remain candidates.

Required connections match named, versioned **capability contracts** describing operations and
behavior a consumer can rely on. Different Resource types can fulfill the same contract; the Pool
still selects the concrete target explicitly. For example, local-directory and git-worktree could
both provide directory access while retaining distinct setup/lifecycle types. A capability contract
is a definition, not another managed family or the user-facing Interface. Similar names or schemas
alone do not prove compatibility, authority or readiness. Exact identifiers and version matching
remain open.

A folder Resource describes backing directory access; each Agent's symlink is preparation state.
Service access follows the [shared Pool authentication contract](#shared-pool-access-and-authentication).
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
  These are not a bypass for running-instance controls. Controls coordinate application per target;
  they check expected configuration revisions by default, with explicit contract exceptions.
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
| Shared authentication | Establish one Pool-owned service login; several authorized Agents use it through different Conduit restrictions without repeating provider sign-in. |
| Independent revocation | Remove one Agent's access while another continues; retain shared Resources and credentials. Report wider impact when the shared login itself fails or is revoked. |
| Native tool compatibility | Preserve provider tool contracts on a supported mediated route; distinguish local caller authority from upstream service identity. |
| Allowed change | A declared setting changes through a permitted Conduit, with valid parameters and observable application. |
| Rejected change | Type/template constraints, current authority or a narrower Conduit restriction reject an invalid request without silently transforming it. |
| Transition and uncertainty | Valid values can still have an unsupported transition; pending and unknown outcomes are not reported as applied. |
| Resource data versus configuration | Writing a note, changing a search setting and attaching a Conduit use their respective contracts. |
| Recovery | Retain valid setup progress, reconcile uncertain effects, avoid duplicate execution and do not replay unsafe writes. |
| Native coverage | Unsupported hot control or enforcement is reported; no fabricated provider compatibility. |

## Design boundary

The five families, template/instance forms, Pool ownership and shared service authentication,
complete construction templates, typed handlers and template-declared change boundaries are
settled direction, including Pool-owned reusable authentication Resources. Concrete binding and
authentication-operation schemas, control schemas, Conduit filtering/composition,
lifecycle transitions and communication topology
remain open. Resolve them with complete examples rather than provider-specific micro-rulings.
