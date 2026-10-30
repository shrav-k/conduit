# Conduit specification

Updated: 2026-10-30. Accepted working model; implementation has not started.

## Working model

Conduit connects models, agents and capabilities through four families: **Session, Resource,
Pool and Conduit**. Each has a reusable **template** and a concrete **instance** form.
This distinction applies throughout the system; it does not require every instance to run a
process or every family to have identical lifecycle operations.

| Family | Template defines | Instance represents |
| --- | --- | --- |
| **Session** | Runtime/model/account defaults, activation instructions, required limits and default connection choices. | A durable participant with resolved settings, private context/history and executions. |
| **Resource** | A typed recipe for creating or attaching data/capabilities, with supported configuration and behavior. | A particular provisioned or attached capability with identity, backing-source information and exactly one owning Pool instance. |
| **Pool** | Resource-template selections, provisioning defaults and policies. | An ownership boundary that creates and contains its own Resource instances. |
| **Conduit** | A typed interaction contract, endpoint requirements, filters and supporting-resource requirements. | An opened **connection** with resolved endpoints, participant attachments and any shared operational state. |

Profile is the former name for a Session template, not a fifth family. Connection is the
instance term for Conduit, not a fifth family. A plugin's type schema describes valid
contracts; a template supplies reusable configuration for such a contract. Neither template
form nor instance form implies an additional general-purpose programming language.

```mermaid
flowchart LR
    ST[Session template] -->|instantiate| S[Session instance]
    PT[Pool template] -->|instantiate| P[Pool instance]
    RT[Resource template] -->|provision or attach through Pool| R[Resource instance]
    P -->|creates and exclusively owns| R
    CT[Conduit template] -->|open| C[Connection: Conduit instance]
    S <-->|join and interact| C
    C -->|authorized access / provisioning request| P
```

This diagram shows common interactions, not a closed topology or process layout. Conduits
are the extensible interaction layer across the families; each contract specifies meaningful
endpoint roles and operations. Templates describe potential behavior; instances carry identity
and realized state. Executions, transports and participant attachments are supporting state.

## Resource ownership and creation

Every Resource instance is created through a Pool instance and belongs to exactly that Pool.
This includes attaching an existing directory/service and provisioning backing Resources for
a connection. A bare Resource-creation path must not bypass Pool ownership. Authoring or
registering a Resource template does not create a deployed Resource.

Different Pools can instantiate the same template. The resulting Resource instances are
distinct even when they intentionally attach to the same repository, vault or remote service.
Contracts must distinguish creating new backing data from attaching to existing data. Separate
registrations do not prove separate files, credentials, quotas or mutation targets.

Conduits access Resources through their owning Pools. A connection that needs conversation
storage requests it from a selected Pool, then uses that Pool-owned Resource. Connections and
Sessions may use several Pools; this does not make them owners of the Resources. Cross-Pool
access follows authorized connections to the owner, not duplicate membership of one instance.

Pool ownership is not automatic access, activation or authority to destroy external data.
Selectors may help choose templates or the Pool's owned instances; their exact targets and
live behavior remain open. They cannot implicitly adopt another Pool's Resource instance.
Pool retirement, ownership transfer and orphan handling require explicit lifecycle design.
No automatic purge follows from a missing Pool reference.

## How instances work together

1. **Instantiate a Session from its template.** Resolve one template and explicit overrides.
   Existing Sessions retain behavioral defaults until reconfigured; required access limits
   remain live. V0 defers Session-template inheritance.
2. **Establish Pools and their Resources.** Pool templates describe intended provisioning;
   explicit operations create Pool instances and create/attach their Resource instances.
   The operation sequence, partial-failure behavior and effects of later template edits remain
   to be specified. No background reconciler is implied.
3. **Open or join a connection explicitly.** Opening uses a Conduit template to establish a
   new instance; joining targets an existing one. Reusing a template name does not choose
   between these operations or imply shared clients, credentials or private context.
4. **Apply access and activation.** Connection filters narrow authorized resources and
   operations. The Session applies its template's activation instructions to the permitted
   inventory. Availability alone does not load instructions or content into the model.
5. **Keep lifetimes distinct.** A participant can detach while shared conversation storage
   remains. A remote operation can continue after its initiating client exits. The connection,
   its Pool-owned Resources, participant state and underlying processes need separate contracts.

Session activation, Pool provisioning and connection supervision are different responsibilities.
The first prepares a participant; the second creates/attaches owned Resources; the third
implements an interaction. Their ordering and dependency rules remain for review.

## Configuration language

**YAML is the authoring format; typed objects and explicit semantics define the language.**
The common envelope remains `apiVersion`, `kind`, `metadata`, `spec`, with system-owned
`status` in inspection. How family and template/instance form are encoded in `kind` or other
fields is unresolved. Four families do not predetermine the number of concrete API schemas.

Retain the accepted registry and editing contracts in [DESIGN.md](DESIGN.md#configuration-language):

- Stable IDs survive rename; replacement objects receive new IDs. Readable references record
  resolved identity, and established explicit bindings require explicit rebind to another ID.
- Use one local registry. Family/form qualification and the resulting name-uniqueness boundary
  need resolution; readable names must never silently substitute a template for an instance.
- YAML registration/export is explicit. Editing a file does not update active configuration;
  changing a template is distinct from instantiation, provisioning, execution or reconciliation.
- Create/update intent is explicit. Updates replace complete authored configuration and use
  separate expected-ID/configuration-version checks. Status-only changes do not invalidate edits.
- Unchanged authored values preserve the configuration version after checks. Omission remains
  different from explicitly setting a default; presentation-only differences do not count.
- CLI/editor edits retain recoverable baselines; absent baselines need explicit preparation.
  Status is rejected on normal writes and omitted with system IDs/versions from reusable exports.
- Deletion is checked; registration removal, native shutdown and backing-data purge are distinct.
  Pool ownership introduces lifecycle obligations that ordinary reference behavior does not settle.

References, defaults, template inputs, selectors, filters, activation and provisioning require
field-specific semantics. There is no universal deep merge or implicit textual substitution.
Authored intent, resolved configuration, observations and operation outcomes stay distinguishable.

## Implementation direction and retained behavior

Conduit owns its runtime in Rust, including execution, context construction, provider/tool
dispatch and recovery. The first frontend is a custom terminal inspired by Pi. Pi's runtime
and Durable are research references; external-agent adapters separately support native runtimes.
Provider adapters and native-agent adapters have different authentication and lifecycle contracts.

MCP is the preferred external capability protocol. Preserve provider-native names, schemas and
workflows. Resource-service authentication is separate from model subscriptions. Native skills
and plugins are brought through compatible ecosystems, without a new Conduit package manager.
Linear owns its tracking workflow; memory/search/graph providers remain replaceable plugins.

Native execution is the v0 default, with selectable or template-required isolation. Four
independent accounts remain an early acceptance target: two Claude and two Codex/OpenAI
subscription identities. Actual identity, subscription compatibility and refresh/logout isolation
must be verified. No silent account or billing fallback is permitted.

Session continuity, portable checkpoints, current-access checks, memory revisions/provenance,
bounded search with coverage reporting, read-only sharing, workspace retention, safe retry and
usage attribution remain as specified in [DESIGN.md](DESIGN.md). A four-family vocabulary does
not relax these rules or make native host access fully mediated.

## Acceptance scenarios

These are intended outcomes, not completed tests. [REVIEW.md](REVIEW.md#initial-experiment)
defines the remaining selections and staged evidence.

| Interaction | Observable outcome |
| --- | --- |
| Template versus instance | References identify the intended form; registering a template does not start work. Instance identity and resolved inputs are inspectable. |
| One template, several Pools | Pools create distinct Resource instances; each has exactly one owner. A foreign Resource cannot become a local member through a label or reference. |
| Shared versus independent backing | Both explicit attachment to shared backing and provisioning independent backing are expressible; target identity and actual enforcement are reported. |
| Connection support | Opening a connection requiring storage provisions it through an identified Pool. Joining reuses the intended connection without creating a duplicate conversation. |
| Concurrent accounts | Four Sessions retain independent private state and verified account selection while accessing authorized common Resources. |
| Session activation | Instructions from the Session template prepare only permitted Resources; connection filters do not become loading instructions. |
| Context and communication | Sessions exchange permitted context through the selected contract while retaining private histories; memory retrieval reports omissions and coverage. |
| Change and failure | Stale edits conflict, unknown effects remain unknown, and retries do not mistake a repeated create request for a deliberate second instance. |
| Lifecycle | Detach, close, stop, unregister and purge have distinct outcomes; removing configuration does not silently destroy files or retained evidence. |

## Design boundary

The four families, template/instance distinction, exclusive Pool ownership and Pool-only
Resource creation are settled. Exact schema encoding, addressing, lifecycle, provisioning and
communication contracts remain open. Review those connected choices before implementation.
Research proposals and the synthetic scenario are discussion aids, not additional requirements.
