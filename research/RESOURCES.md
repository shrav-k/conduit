# Resource types, groups and extensibility

Research: 2026-10-31. **Evidence, accepted working groups and proposals; no implementation.**
Logical type groups independent of plugin packages, separate `spec.type.group`, `spec.type.name`
and `spec.type.version` fields, and the four core group names are accepted. Filesystem's
`local-directory` and `git-worktree` are distinct accepted type boundaries. Their detailed
contracts still need review; local-directory supports explicit attachment and creation through
its Pool in v0. Other types, consumer interfaces and extension governance remain open. An
agent-communication group is a candidate for further design, alongside content and identity.

Read alongside [DESIGN.md](../DESIGN.md#resources), [the review agenda](../REVIEW.md),
[connection scenarios](CONDUITS.md) and [configuration research](CONFIGURATION.md).
This investigation examined primary specifications/documentation, two research foundations and
the existing pinned Pi checkout. It performed design walkthroughs, not provider integration
tests, account sign-ins, runtime implementation or a benchmark.

## Recommendation

The accepted working core is **filesystem, storage, search and integration**. Evaluate
**content**, **identity** and an **agent-communication group** as additional candidates. These
are catalog boundaries, not a commitment to implement every example in v0. New groups and
types must remain possible without changing the four object families or requiring an
implementation package for each group.

The harder design issue is what a type promises. A useful catalog distinguishes:

| Layer | Meaning | Example |
| --- | --- | --- |
| Group | Stable organization of related Resource contracts. | `storage` |
| Type and contract version | Configuration and behavioral contract for one responsibility. | A versioned record-store type. |
| Consumer interface, proposed | Operations/semantics another component can depend on across compatible types. | Read a record by stable ID and revision. |
| Implementation package | Code and schemas delivering one or more contracts. | One package could implement storage, search and MCP access. |
| Resource template | Reusable values for a selected type. | Euphony's knowledge-store defaults. |
| Resource instance | Concrete configured capability, exclusively owned by a Pool. | The attached Euphony knowledge store. |
| Consumer attachment | Current authorized use through a Conduit, including private preparation. | One Session's service identity, cursor or filesystem projection. |

Only the existing four families are registered object families. Interfaces and package metadata
would describe contracts, not introduce another user-managed fleet of objects. Ordinary users
should configure instances; they should not repeatedly author capabilities already defined by
a type's implementation and schema.

**Main proposed rule:** group by responsibility; select and operate through explicit contracts.
Do not infer read/write behavior, isolation, compatibility or lifetime from the group name.

## Evidence from other systems

These sources inform different boundaries. They do not collectively prescribe Conduit's schema.
Documentation was accessed on the research date; versioned sources are identified below.

| Source | Observed mechanism | Conduit implication and limit |
| --- | --- | --- |
| [Kubernetes API groups](https://kubernetes.io/docs/reference/using-api/#api-groups) and [CRD categories](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#categories) | API groups participate in API identity; categories separately support grouped discovery. A CRD supplies a schema rather than just a descriptive tag. | Keep contract identity separate from changeable browsing labels. Adopt neither Kubernetes' combined string syntax nor an automatic controller loop. |
| [Terraform provider schemas](https://developer.hashicorp.com/terraform/plugin/framework/handling-data/schemas) and [providers](https://developer.hashicorp.com/terraform/plugin/framework/providers) | Providers register resource/data-source types with their own schemas and can configure clients used by them. | One implementation can supply several types. A shared client must still respect Conduit's per-attachment identity boundaries; package membership is not authority. |
| [Terraform data sources](https://developer.hashicorp.com/terraform/language/data-sources) | Data sources fetch external data without creating or modifying the external resource. | Attachment/observation and management are different responsibilities. Conduit need not copy Terraform's separate top-level constructs to express that difference. |
| [Pulumi package schemas](https://www.pulumi.com/docs/iac/guides/building-extending/packages/schema/) and [type identity](https://www.pulumi.com/docs/iac/concepts/resources/names/) | Schemas describe exported resources/functions; resource type tokens identify package, module and type. Schemas also drive generated documentation/SDKs. | A useful counterexample to our chosen separation: do not embed installation packaging in Conduit type identity. Borrow discoverable typed contracts and generated inspection/documentation. |
| [Crossplane managed resources, v2.4](https://docs.crossplane.io/v2.4/managed-resources/managed-resources/) | A managed registration and external object have separate identities. Management policies distinguish observation, creation, updates and deletion; support varies by provider. | Resource ownership does not prove ownership of the native target. Type support and the requested management mode must be visible; Conduit retains explicit effects instead of adopting continuous reconciliation. |
| [Docker plugin configuration](https://docs.docker.com/engine/extend/config/) | A plugin declares implemented interfaces separately from entrypoint, socket, mounts and execution settings. | Contract identity, communication mechanism and deployment package are separate concerns. A Rust implementation need not mean an in-process Rust plugin ABI. |
| [CSI specification](https://raw.githubusercontent.com/container-storage-interface/spec/master/spec.md) | Provisioning, attachment and publication are distinct operations. Capability checks and access modes describe supported use; the specification leaves plugin packaging outside its scope and does not promise POSIX compliance for every volume. | Distinguish native backing, consumer preparation and concurrent access. A label such as filesystem or local is insufficient to promise locking or safe concurrent writes. Do not import the entire storage-orchestration protocol. |
| [WIT interfaces/worlds](https://component-model.bytecodealliance.org/design/wit.html#worlds) | Worlds describe imports and exports; interface contracts are separate from component implementation. WIT defines signatures/types, not behavioral semantics. | Resource types can declare required/provided interfaces, with Conduits supplying actual relationships. Also document effects, consistency and failure behavior. This is inspiration, not a selection of Wasm, WIT or a plugin execution engine. |
| [Backstage descriptors](https://backstage.io/docs/features/software-catalog/descriptor-format/#kind-resource) | Resource catalog types can use organization-chosen strings; owner metadata describes responsibility and is explicitly not runtime authorization. | Keep an open catalog and useful discovery metadata. Conduit goes further: an executable type requires a known schema/contract, not an arbitrary label. |
| [W3C DCAT 3](https://www.w3.org/TR/vocab-dcat-3/#class-dataset) | Datasets, distributions and data services represent different aspects of accessible data. One conceptual dataset can have multiple representations/services. | The underlying collection, search service and MCP access need not be one Resource. Separate configurations do not imply copying the data. Do not introduce an RDF requirement. |
| [W3C WoT Thing Description 1.1](https://www.w3.org/TR/wot-thing-description11/) | Descriptions distinguish properties, actions and events, with data schemas and protocol forms. | A future device Resource can offer several related interaction modes. One operation need not mean one Resource, and protocol choice alone does not determine purpose. |
| [OpenTelemetry semantic conventions](https://opentelemetry.io/docs/concepts/semantic-conventions/) and [stability](https://opentelemetry.io/docs/specs/otel/versioning-and-stability/) | Shared names carry meaning for producers and consumers; changing conventions can break downstream tools. | Type/operation identity must remain stable enough for logs, search and configuration. Do not promise automatic schema migration; the current stability text cautions against reliance on schema transformations. |

MCP is a particularly important boundary. Its [2026-07-28 resources specification](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)
describes contextual data identified by URIs, while [tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools)
describe callable operations. The [version's changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog)
also identifies prompt discovery, removes protocol sessions/initialization, and uses per-request
metadata and discovery. Therefore an MCP-access Resource's responsibility is access to the
server's supported surface, not just tool invocation. Neither an MCP protocol resource nor an
HTTP connection is automatically a Conduit Resource instance. Durable Conduit attachments and
Session identities must not depend on legacy MCP transport-session identity.

Pi's pinned [package documentation](https://github.com/earendil-works/pi/blob/0441483926f283e42ca44eb7b4a46552e9469d6f/packages/coding-agent/docs/packages.md)
bundles skills, prompts, themes and extensions as a distribution unit. Its declared selections
can narrow those contents. This supports retaining native bundles while recognizing different
activation requirements. A bundle's installation format does not guarantee its executable
extensions work in Conduit's runtime. The [existing Pi investigation](PI_CONNECTORS.md#session-activation-and-pool-resources)
documents that compatibility boundary. No Pi runtime dependency follows from this finding.

### Research methods and their limits

[Kundisch et al., An Update for Taxonomy Designers (2022)](https://link.springer.com/article/10.1007/s12599-021-00723-x)
extends taxonomy-development guidance and emphasizes explicit purpose, iterative construction
and evaluation of usefulness beyond internal consistency. Its review covers 164 taxonomy
articles. Our adaptation is to classify actual Conduit scenarios, identify counterexamples,
and revise the groups rather than invent an exhaustive tree in advance. This is a design
method, not empirical proof that the working four groups are optimal. Collaborator evaluation
and working integration examples remain necessary.

[Liskov and Wing, A Behavioral Notion of Subtyping (1994)](https://www.cs.cmu.edu/Groups/venari/subtype-toplas.html)
relates substitutability to preserving properties promised by a supertype. The authors'
abstract and indexed paper excerpt were available; the full PDF fetch timed out, so this
report does not claim a detailed review of its proofs. The relevant design warning is that
matching method names or JSON shapes does not establish interchangeable behavior. Two stores
with a `read` operation may differ in revision guarantees or authorization. No inheritance
hierarchy or formal-verification requirement is proposed for Conduit.

## How to classify a Resource

Use the **responsibility managed by this registration** as the organizing question. Then
evaluate the independent dimensions below. This is our proposed analysis checklist, not a
mandatory block every author must fill in or a new metadata bookkeeping system.

| Dimension | Examples | Why it cannot be inferred from the group |
| --- | --- | --- |
| Responsibility | Filesystem view, record persistence, retrieval, native service access. | A provider can supply several responsibilities. |
| Consumer contract | Paths, record revisions, query results, native MCP operations. | Two storage types may not offer the same consistency guarantees. |
| Backing and placement | Local, remote, embedded, externally managed. | A local MCP process can call a remote service. |
| Creation/management | Attach existing, create managed, observe only. | A database can support attachment without permitting creation. |
| Preparation | Symlink, client configuration, index setup, credential resolution. | Not every instance needs a resident process. |
| Identity | External target, service principal, attachment identity. | Several logins may reach the same target and still need private state. |
| Effects and concurrency | Reads, revisions, append, exclusive operation, shared limits. | More clients do not create more independent deployment targets. |
| Persistence | Durable backing, rebuildable index, ephemeral client state. | A Conduit may end while its backing log remains. |
| Compatibility | Runtime hooks, file formats, protocol versions, optional operations. | A skill written for an agent can include nonportable executable behavior. |
| Current availability | Configured support, observed support, authorization, health. | A supported operation may be unavailable to this attachment. |

Groups need not encode these dimensions. Labels can help discovery, but labels are not
verified capabilities or permission grants. Contract-supplied facts should be derived once;
attachment-specific observations belong with the attachment and its outcome.

### When to create a new type

Prefer a new type when callers or lifecycle management must rely on a different contract:
different configuration meaning, required dependencies, observable results, preparation,
consistency or effects. Prefer a configuration value when the same contract can honestly
cover the variation. Add an operation within the type when it belongs to the same responsibility.

| Variation | Initial recommendation | Reason / qualification |
| --- | --- | --- |
| Another directory path | Same type, different instance/configuration. | Target changes; path-access contract does not. |
| Another Linear workspace or permitted account | Usually same MCP type. | Target/authentication choices do not create new protocol semantics; restrictions must actually be supportable. |
| Local-process vs HTTP MCP transport | Same MCP type with typed transport alternatives, provisionally. | Preserve native MCP surface; process preparation differs and must be explicit. Split if one schema cannot describe lifecycle clearly. |
| Git worktree management vs plain folder access | Accepted separate types: local-directory and git-worktree. | Worktree creation/cleanup and repository relationships add obligations beyond folder exposure. Local-directory supports explicit attach/create; detailed schemas and Git-specific modes remain open. |
| Keyword, vector and hybrid retrieval | Capabilities/configuration of an index/query type when the result contract remains valid. | An algorithm name alone is insufficient reason for three types. Different guarantees may justify a separate type. |
| Native SQL vs revisioned Conduit records | Different contracts/types or an explicit adapter. | A generic database API does not inherently satisfy Conduit's record lifecycle. |
| Records vs append-only history | Separate candidate storage types. | Update/conflict/ordering operations differ; common persistence is not enough. |
| Two implementations of exactly the same contract | Same type only with conformance evidence and explicit implementation selection. | Packaging is independent; installation order must not decide behavior. |
| Another method on a native MCP server | Discovered operation, normally no new Resource. | Avoid a registration for every native tool. |
| Executable runtime extension vs instruction bundle | Separate activation contract; potentially separate Resource type. | Reading text is not executing extension code. Preserve native packaging where practical. |

These recommendations do not relax the accepted separation of storage, search and MCP access.
They make each boundary operationally useful without turning every method, file or login into
another registered instance.

## Candidate catalog

The four core group names and the distinction between `local-directory` and `git-worktree` are
accepted; other type names and additional groups below remain proposals. The two separate
columns deliberately avoid a compound identifier. “Core” identifies the initial working
catalog, not all required v0 features.

### Alternatives considered

| Primary grouping strategy | Attraction | Problem for Conduit | Recommended use |
| --- | --- | --- | --- |
| Provider: AWS, Linear, GitHub | Familiar service names and native operations. | Each provider spans credentials, data, search and tools; switching provider changes the apparent category of the same responsibility. | Provider identity and native configuration; a domain-specific type when justified. |
| Transport: MCP, HTTP, local process | Easy adapter dispatch. | One protocol carries many capabilities; changing transport need not change the data contract. | Typed integration contracts and protocol settings, not the only grouping system. |
| Location: local, remote, cloud | Useful operational distinction. | Local processes can access remote services; one Resource may combine both. | Backing/preparation metadata and constraints. |
| Lifetime: ephemeral, persistent | Helps cleanup planning. | One registration may have durable backing and ephemeral clients. | Type-specific lifecycle semantics and observed state. |
| Very broad buckets: io, tools, memory | Few categories to learn. | IO includes almost everything, tools omits MCP data/prompts, and memory hides storage/search separation. | Search labels or user-facing views where helpful. |
| Deep hierarchy: data → storage → database → vector | Detailed browsing. | Multiple plausible parents, many unstable paths, and category moves become identity migrations. | Optional catalog navigation, not mandatory type-reference nesting. |
| Responsibility plus separate interfaces | Explains what each registration configures while supporting compatible implementations. | Requires explicit boundary examples; group names alone are intentionally insufficient for execution. | Recommended canonical grouping, tested against the scenarios below. |

This comparison explains the working core names, rather than proving that four is the final number
of groups. Keep one canonical group for a type's identity, with multiple optional discovery
labels. If a new type does not fit, add a coherent group instead of stretching an existing
definition or adding a catch-all executable type with unvalidated configuration.

### Accepted working core groups

| Group | Responsibility | Type names and status | Confidence and boundary |
| --- | --- | --- | --- |
| `filesystem` | A filesystem tree and its supported access/lifecycle. | Accepted: `local-directory`, `git-worktree`. Candidate: `remote-tree`. | Local-directory supports explicit attach/create. Schemas, Git-specific modes and any shared interface remain open. A remote tree must state its semantics. |
| `storage` | Persist and retrieve a defined data model. | `records`, `blobs`, `append-log`, `graph`. | High for separating persistence from retrieval ranking. Revisioned records and append-only logs should not pretend to have identical operations. |
| `search` | Retrieve relevant references/results from configured sources or an index. | `index`, possibly `federated-query`. | High for independence from storage; lower for exact type names and whether index maintenance belongs here or a separate processor. |
| `integration` | Expose a configured native service/tool surface without requiring a new domain model. | `mcp`, `cli`, `http-api`. | High for MCP access; CLI and arbitrary HTTP need narrower contracts before inclusion. Broader and more accurate than `tools` for data/prompt access. |

`filesystem` and `storage` deliberately differ by the abstraction promised: path/tree semantics
versus a record/blob/log/graph model. A store may persist its bytes in a directory without
registering an additional folder Resource merely for its private implementation files. If another
Resource or Session must use that directory independently, make that separate access explicit.

An integration Resource promises native access, not an implementation of every domain capability
reachable through it. Linear MCP does not automatically satisfy a portable ticket-store contract.
A search provider exposed by MCP may require a search Resource and an MCP-access Resource when
Conduit needs both independently configured responsibilities. The Conduit between them supplies
the logical access; ordinary socket mechanics do not create an additional Resource.

### Likely adjacent groups requiring a boundary ruling

| Group | Candidate types | What would justify the group | Main unresolved issue |
| --- | --- | --- | --- |
| `content` | `document`, `skill-bundle`, `prompt-bundle`, `native-config`. | Interpreted material with format, compatibility and activation requirements. | A document can simply be a record/file within an existing Resource. Make it a Resource only when independently configured access/lifecycle warrants one. |
| `identity` | `credential-source`, `secret-store`, `token-broker`. | Resolving/providing credentials or authentication as a managed capability. | Distinguish a service capability from an individual attachment's login/cache. Do not require a new Resource per login or move secret values into YAML. |

For skills, the unit can remain an ecosystem-native bundle. A bundle can expose a manifest of
contents, and activation can select compatible entries without creating a Resource for every
skill. Separate native executable extensions only when their runtime contract needs it. This
preserves bring-your-own skills rather than reviving the earlier Conduit skill framework.

### Agent communication as a domain-specific group

Keep a distinct group for agent communication under consideration. `communication` is a
possible name, not an accepted identifier. Its justification is the contract exposed to agents:
Session addressing, participant context, exchanges and coordination. Familiar backing storage
does not eliminate those semantics or require placing every such capability in `storage`.

| Mode to explore | Agent-specific responsibilities to examine | Questions still open |
| --- | --- | --- |
| Direct messaging | Address a Session, distinguish a request from a reply, track message availability. | Dedicated mailbox type, a two-participant conversation, or a Conduit mode? Delivery, offline handling and acknowledgements remain undecided. |
| Multi-participant conversation | Participant-visible history, topic/thread relationships and context for agents joining later. | Which state belongs to a Pool-owned Resource versus the live flow? No room topology, invitation or wake policy is selected. |
| Shared context | Make selected knowledge, checkpoints or collaboratively maintained material available across Sessions. | Snapshot exchange, shared references, live collection or editable state? Define updates, provenance and conflicts before selecting a contract. |

These are examples to test, not a closed list or three approved types. A group can cover several
agent-focused contracts without becoming one broad Resource that promises all communication.
Equally, direct messaging and a group conversation may turn out to share one well-defined type
with different configuration. Decide using their behavior and lifetime, not participant count alone.

The working distinction to test is: a Resource supplies a configured communication capability
and any state it manages; a Conduit establishes the live, filtered access or exchange among
consumers/peers. Resource instances remain Pool-owned. Conduits need no independent runtime,
and their last-consumer lifetime rule remains accepted. A new group alone decides neither
history retention nor when an agent wakes or loads context.

Do not require a separate generic storage registration beneath every communication Resource
solely because it stores bytes. A separately managed/shared backing capability can be another
Pool-owned Resource connected through a Conduit; private implementation state need not create
another public object. Which state needs that independent contract belongs in the walkthrough.
Shared context remains an open design probe, not acceptance of the previously rejected editable
room working-memory model.

### Extension space: examples, not reservations or a closed list

| Potential group | Candidate types/use cases | Boundary to protect |
| --- | --- | --- |
| `execution` | A container environment, VM, job runner or browser environment. | Session remains the durable participant; a provisioned execution environment need not become another Session. Do not require this indirection for every native launch. |
| `messaging` | Durable queue, broker topic or stream service. | Infrastructure-level capability; distinguish it from the agent-communication candidate above. Whether these need separate groups remains open. No default room topology follows. |
| `observability` | Trace store, metric-query endpoint, audit sink. | Reuse storage/integration types when sufficient; use a new type only for signal-specific semantics. |
| `coordination` | Lease service, lock service, rate/quota allocator. | Lock state has a target and authority; a category name does not make arbitrary native operations safe. |
| `transformation` | Embedding service, parser, media conversion or indexing processor. | Model invocation/processing can have independent operation and usage contracts; avoid one type per function by default. |
| `device` | Camera, microphone, attached instrument, remote sensor. | Readings, actions and event subscriptions can share one device responsibility; effects and access differ by operation. |
| Domain-specific groups | A deployment target, issue workflow adapter, research dataset service. | Preserve provider-native operations unless a portable domain contract adds demonstrated value. |

These examples test extensibility. There is no need to register empty groups, build all of
their infrastructure, or force every provider into this list before the first experiment.

## Worked mappings and difficult cases

### Euphony code and concurrent writers

The directory/worktree type distinction and local-directory's explicit attach/create operations
are accepted. Attachment requires an existing directory; fresh creation conflicts with an
existing target. Neither operation happens on registration, and cleanup remains separate.
Relative template paths resolve against an explicit creation base; the complete instance saves
an absolute host backing path. For example, `./repo` with base `/work/euphony` becomes
`/work/euphony/repo`, independently of later launch directories. This does not yet select
symlink-resolution behavior or the Session's prepared path.
Remaining detailed configuration and lifecycle choices in this walkthrough are proposals.
Plain directory access does not imply Git management, even when the folder contains a checkout.

Attach an existing checkout as a `filesystem` / `local-directory` Resource through its Pool.
Two Sessions can receive separate symlinks from their Conduit attachments; those symlinks are
preparation, not extra Resources. Their edits still affect the same target. For independent
writers, provision a `filesystem` / `git-worktree` Resource for each workspace through the Pool.
The repository's shared administration and remote remain distinct from each working tree.

Selecting a filesystem group cannot determine copy-versus-share behavior. The type contract and
establishment configuration must state it. Removing a projection or registration preserves backing data under
the accepted cleanup rules. No file-locking guarantee arises from ownership or naming.

### Linear MCP and AWS CLI

Use an `integration` / `mcp` template for configured Linear access. The same Resource can serve
multiple permitted service principals per attachment where supported. Workspace restriction is
a provider/adapter capability to verify; a field named workspace cannot establish it by itself.
Keep provider tool names/schemas visible. A Session's model account remains independent.

An `integration` / `cli` candidate represents prepared native command access. The AWS executable,
credential profile, target account and region are not all the same thing. Attaching four clients
does not create four deployment targets. A generic command adapter can preserve native use;
it cannot claim to identify or serialize every deployment embedded in arbitrary shell commands.
A dedicated deployment-target type is justified later only if it promises those semantics.
The [existing AWS walkthrough](CONDUITS.md#4-access-aws-through-its-cli) contains the provider evidence.

### 1Password and credential resolution

Ordinary 1Password CLI access could initially use a native integration. If Conduit manages
credential resolution/injection as a distinct capability, an `identity` type becomes useful.
The Pool owns that configured service Resource. A Session's login, refresh state and private
cache remain per-attachment/runtime state where needed, not a second ownership hierarchy.
Each downstream connection still requires authority; same-Pool membership does not deliver secrets.
Provider authentication modes remain integration-specific, as explored in the
[credential walkthrough](CONDUITS.md#3-access-a-1password-account-or-vault).

### Memory storage, search and a backend plugin

An Euphony Pool can own a `storage` / `records` Resource and a `search` / `index` Resource.
A declared Conduit gives search the required storage access. Sessions access each through
their permitted Conduits; authority for index maintenance does not authorize returning every
indexed record to every querying Session. Enforce the receiving Session's current access
before returning results and retain the agreed incomplete-search reporting.

One backend package may implement both responsibilities, and an MCP-access Resource can expose
the provider's native surface separately. This does not require duplicate data or a process for
each registration. Conversely, a generic MCP endpoint does not prove revision-checked writes,
hybrid retrieval, filtering, provenance or reliable freshness. Evaluate a candidate such as
Xerj against each contract independently; this report does not re-audit or certify its features.

Whether an index exposes both query and maintenance operations depends on one coherent
responsibility and lifecycle. A separately operated ingestion pipeline may warrant another
type. Avoid enforcing either decomposition until a concrete backend demonstrates the need.

### Skills, documents and Session activation

A native skill bundle can be a candidate `content` Resource when it has independently selected
source/version/compatibility. Its bytes might come from a filesystem or storage Resource through
a Conduit. Do not register every backing file again. For a plain document already in permitted
storage, a reference can suffice; a specialized Resource is useful when its management differs.

Session activation decides whether to advertise skill metadata, load selected instructions or
prepare a compatible native plugin. The Conduit filters availability. A `content` group cannot
cause automatic loading, and a model family alone cannot establish runtime-hook compatibility.

### Communication and retained history

A Pool-owned `storage` / `append-log` could retain conversation history. A live Conduit connects
participants and uses that Resource. When the last logical consumer/peer leaves, the flow ends;
the retained history can survive. A dedicated broker/queue Resource may supply delivery when
needed, but is not mandatory just because a Conduit has temporary routing state.

This supports the current Conduit model without reinstating old room decisions. A transcript,
queue, participant read position and active transport have different ownership/lifetime needs.
Which are durable shared Resources versus attachment state remains an interaction-contract choice.
An agent-communication Resource may expose richer semantics over this backing, as described in
the [candidate group](#agent-communication-as-a-domain-specific-group); generic storage is not
the only possible Resource contract for this scenario.

## What should actually drive compatibility

**Proposal:** a small set of versioned interfaces, introduced only where real consumers need
interchangeability. Type-group membership should help discovery, not imply interface inheritance.

For example, a search implementation can require record enumeration and revision-aware reads.
Two different storage types may satisfy that exact requirement. A third may only export bytes,
so it needs an explicit adapter or is incompatible. Its placement in `storage` is insufficient.
The Resource contract declares the requirement; its Conduit declaration supplies the actual
target. Do not maintain duplicate endpoint assignments inside a second configuration structure.

| Fact | Proposed authority | Example |
| --- | --- | --- |
| Valid configuration and behavioral promise | Selected type contract/version. | A records type defines revision-conflict behavior. |
| Implemented interfaces/options | Implementation declaration plus conformance evidence. | A backend supports conditional writes. |
| Selected mode/target | Resource instance spec. | Attach to an existing collection. |
| Observed current service support | Preparation/discovery state, with provenance/freshness. | A remote endpoint currently offers a search operation. |
| Permitted use | Current authority, filters and attachment identity. | This Session can read but not update. |
| Successfully established access | Actual preparation/operation outcome. | Authentication completed for the selected service principal. |

These facts can overlap but are not interchangeable. If a feature is mandatory in a contract,
its absence is a compatibility/readiness failure; it is not silently downgraded to optional.
Groups, advertised capabilities and schema validity do not prove enforcement by a remote server.
An instance author may request narrower use, but cannot declare unsupported behavior into existence.

For v0, prefer concrete adapters and only the shared interfaces needed by the experiment.
The open design question is how far to formalize this before implementation. A universal trait
registry, multiple inheritance, generated method wrappers for every service, or a policy field
per provider call would add bookkeeping before demonstrating value.

### Consequences for Pools and Conduits

Pools can offer useful views and provisioning choices from type metadata: show filesystem
Resources together, identify searchable capabilities, and surface which dependencies remain
unavailable. This is a candidate meaning of intelligent grouping, without duplicating each
Resource's configuration or hard-coding every future group into Pool behavior.

Conduits supply actual relationships and narrow access; the selected contract establishes what
can flow and which operations make sense. The Session's activation rules select how permitted
capabilities are presented. Neither a matching group nor an implementation package activates a
Resource or provides authority. A discovery label can help choose a Resource without becoming
an executable compatibility promise. Pool mechanics should follow these tested contracts.

## YAML, identity and extension rules

### Template establishment and instance state

**Accepted direction:** Resource templates contain establishment choices in their spec, including
local-directory create versus attach. Across the four families, users should not need a separate
operation-request recipe to supply missing setup behavior. An explicit lifecycle action triggers
what the configuration declares. **Proposed details below:** exact config/status fields and what
survives in the instance are not settled. The type contract defines semantics; the adapter
implements them. Template YAML need not contain shell scripts or reproduce adapter code.

Consider this abbreviated template, using an absolute path to keep contextual resolution out
of the example. The common apiVersion is omitted because its identifier remains undecided.

```yaml
kind: Resource
form: template
metadata:
  name: euphony-folder
spec:
  type: {group: filesystem, name: local-directory, version: v1alpha1}
  config:
    path: /work/euphony/repo
    establish: create
```

Selecting this template for establishment through Pool `euphony-development` is sufficient to
choose creation; the caller does not also supply an action flag or request manifest. Selecting
the owning Pool is still necessary. A different template, or an ordinary edit to the typed
configuration before use, can declare `establish: attach`. No implicit create-or-adopt behavior
is introduced. A complete reusable declaration can reference external inputs; completeness does
not require embedding credentials, every dependency's contents, or a literal owning Pool.
How Pool/context supplies declared values such as a relative-path base remains for review.

The type contract must explain the transition, rather than treating it as a blind config copy:

| Stage | Example responsibility |
| --- | --- |
| Validate and resolve | Verify schema/authority/owner, apply declared defaults and resolve any relative path against its explicit base. Registration alone performs no native action. |
| Establish | The directory adapter follows the declared create/attach intent. Fresh create conflicts with an existing target; attach fails when the directory is absent. |
| Record the instance | Assign identity/Pool ownership, retain ongoing settings, and report the actual establishment outcome and readiness. Failed/pending outcomes remain visible. |
| Use and prepare consumers | Sessions receive their own prepared paths through Conduits. This does not repeat backing creation or register another Resource per symlink. |

One candidate inspection result after success is:

```yaml
# Abbreviated inspection, not authorable status or an approved schema.
kind: Resource
form: instance
metadata:
  name: euphony-folder-7f3a
spec:
  type: {group: filesystem, name: local-directory, version: v1alpha1}
  config:
    path: /work/euphony/repo
status:
  establishment: {outcome: created}
  readiness: ready
```

The owning Pool and stable ID also exist, but their field layout is omitted. Here the action is
consumed during establishment and its result is reported. An alternative retains the selected
mode as immutable establishment configuration on the instance. Both keep intent in the template;
review which information is necessary for recovery/export without a mandatory full template
snapshot or override history. Instance spec remains usable configuration, not an editable claim
that the external effect succeeded. Status fields are system-owned and excluded from config writes.

Reattempting incomplete establishment is different from instantiating another Resource. Internal
operation identities/outcomes prevent unsafe blind retries without making users author a request
language. After a directory has been established, losing it does not by itself authorize recreating
it. Updating instance settings also does not silently replay creation. Exact failure-state retention,
recovery and permitted instance transitions still require decisions.

Two useful design tests follow. First, an exported instance lacking create/attach intent cannot
promise to be a complete setup recipe: define an explicit conversion to establishment configuration,
or retain enough declared intent, rather than guessing. Second, a template can be comprehensive
without being a universal workflow engine: plugin-defined fields, references and lifecycle behavior
may cover the examples. [Configuration research](CONFIGURATION.md#establishment-in-configuration)
compares Compose, Docker's lifecycle boundaries and Kubernetes before proposing extra syntax.

### Type reference and extension identity

The separate type fields are accepted; their example values and all plugin configuration below
are illustrative. These are partial `spec` fragments, not complete validated object documents.

```yaml
spec:
  type:
    group: filesystem
    name: local-directory
    version: v1alpha1
  config:
    path: /example/euphony
```

```yaml
spec:
  type:
    group: integration
    name: mcp
    version: v1alpha1
  config:
    transport: streamable-http
    url: https://service.example/mcp
```

No instance name, Pool, service account, implementation package or provider protocol version
is encoded in that type name. The Resource-type version governs the contract, while the native
protocol and common object `apiVersion` keep their separate meanings. The spelling `v1alpha1`
is still illustrative. This change does not decide the common `apiVersion` string grammar.

If group participates in contract identity, changing it is a contract-reference change, not
merely rearranging a UI. Keep canonical groups stable; use labels/catalog views for alternate
discovery categories. One type need not be duplicated to appear in several useful searches.
Do not force every third-party type into the built-in groups.

Proposed admission rules for future type providers:

1. Supply the type's identity/version, closed configuration schema with declared open maps,
   defaults, behavioral documentation and supported management modes.
2. Describe required/provided consumer interfaces and preparation/cleanup obligations only as
   applicable. Do not require every type to implement generic create/update/delete methods.
3. Make implementation availability distinct from schema availability, preserving the accepted
   rule: missing schema rejects registry writes; missing runtime may leave valid config unready.
4. Detect conflicting definitions rather than choosing by installation order. Two implementations
   of an agreed contract require explicit resolution; incompatible meanings need distinct identity.
5. Preserve backing data and report unknown external outcomes under existing lifecycle rules.
6. Provide examples and conformance evidence for the operations actually promised, including
   concurrency, identity separation and error behavior where relevant.

**Still unresolved:** a logical category is not a publishing authority. We can qualify group
identities, add a separate contract-authority field, or begin with a locally curated catalog
that rejects collisions. Each has portability costs. Keep this separate from picking group
responsibilities; do not hide publisher/package identity inside a new compound string. No
additional mandatory authority field or public plugin marketplace is selected by this report.

For Rust, a fixed enum listing every Resource type would conflict with the intended extension
model. A dynamic catalog resolving type identity/version to schemas and handlers is a plausible
implementation direction. Built-ins can use the same contract boundary as external plugins.
Whether handlers are built into the binary, processes, MCP adapters or sandboxed modules remains
open; this research does not choose an ABI or distribution system.

## Evaluation and the next review

The desk review classified the user's folder, communication, credential, AWS and Linear examples,
then tested memory/search, native bundles and future devices/queues against the same boundaries.
It found three recurring ambiguities: filesystem versus storage, protocol access versus domain
semantics, and interpreted content versus its backing files. The worked examples above resolve
them by the responsibility being configured, not the provider or transport name. This is a
reasoned proposal; it still needs user evaluation and integration evidence.

| Counterexample/change | Expected property of a useful design |
| --- | --- |
| One package is split into two packages. | Public type references and instance ownership remain stable. |
| A new storage vendor adds a plugin. | No new top-level family or forced global type enum change. Compatibility is explicit. |
| A server adds another native tool. | Discovery/filtering changes; no automatic Resource registration per operation. |
| Another account uses the same MCP endpoint. | Separate identity/private state can coexist without duplicating target configuration. |
| Two Resources reach the same deployment target. | They do not falsely appear to have independent concurrency limits. |
| A new queue or camera type is introduced. | It can have operation/lifecycle semantics not assumed for ordinary records. |
| A type moves in a documentation navigation tree. | Browsing metadata can change without renaming its canonical contract. |
| A plugin claims revisioned reads but returns only latest content. | Contract mismatch remains visible; shared group/schema shape does not excuse it. |
| A dependency disappears after preparation. | Existing current-access/readiness rules apply; type classification does not invent recovery guarantees. |

Recommended discussion sequence:

1. Use the accepted four groups and directory/worktree split for concrete type review; do not
   repeat those rulings. Local-directory create/attach intent and absolute instance backing paths
   are also settled, with establishment intent in the template spec. Review the typed transition
   to instance settings/status, failure/retry and export together, then Session preparation/symlinks.
2. Walk through directory, records, search and MCP contracts: configuration, operations,
   dependencies, preparation and one failure each. Evaluate content and identity, retaining
   the agent-communication candidate for the detailed communication review.
3. Decide the minimum shared consumer interfaces and type-collision/implementation selection rules.
4. Define remaining version/name grammar and connection declaration fields from those examples.
5. Return to Pool grouping with enough information to derive useful behavior and minimize state.

The initial implementation slice can be much smaller than this catalog: local directory access,
one record-store contract, the search behavior needed for it, and MCP access, with native content
activation as required by the experiment. A group can start with one supported type. Final scope
and exact adapters remain a review choice, not a new implementation authorization.
