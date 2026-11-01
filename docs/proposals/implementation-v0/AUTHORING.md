# Plugin and template authoring walkthrough

Status: authoring examples, 2026-11-01. Lightweight Conduit support with the public protocol and
extension structure prioritized over a helper library is accepted. Publishing inspectable
contracts with implementations is also accepted. Detailed contract and template syntax is
deferred until conceptual review is complete. MCP is accepted as the public invocation base with
a small Conduit profile. Code-optional implementation bindings and independent component access
are accepted. Authored code normally runs in supervised instance-associated helpers; trusted built-in
Rust code may remain in-process. Profile details, SDK, package syntax and exact helper grouping remain open.
Code is illustrative and has not been executed.
The [review](REVIEW.md#review-priority-plugin-and-template-authors) records the ruling and remaining
choices; [DESIGN](../../DESIGN.md) supplies the settled semantics.

The objective is an extension an ordinary author can understand and test. A template author
should be able to compose existing behavior without becoming a plugin author. A plugin author
should implement the behavior their extension adds, without rebuilding the entire Conduit host.
Real plugins should test these contracts early. Conduit can add reusable helpers as recurring
needs become clear; the initial implementation does not depend on building that library first.

## The pieces, before choosing an authoring API

| Piece | Concrete meaning in these examples |
| --- | --- |
| Plugin package | Published contracts with bindings to authored code, existing MCP implementations, graphs of operations or a combination. A package can contribute operations or type implementations; it is not automatically a Resource instance. |
| Operation export | A callable contribution, such as computing a digest or establishing a directory. Being available to a setup graph does not automatically expose it to Agents. |
| Resource type | The contract for a configured responsibility, such as a local directory: configuration, capabilities and applicable lifecycle/controls. Its implementation can reuse operation exports. |
| Template | Selects implementations, supplies configuration and composes setup behavior. It can reuse existing plugins without adding code. |
| Instance | Holds the resolved definition, current confirmed settings and managed runtime state. A Resource instance has a Pool owner. |

The plugin authoring question is how code reaches the callable boundary. It does not change
these object distinctions or require one process per package, operation or instance.

## 1. The same small plugin, exposed two ways

Start with a pure operation: `sha256` accepts a UTF-8 string and returns its digest. It needs no
filesystem access, login, persistent process or recovery lookup. Calling it again with identical
input produces the same result and has no external side effects. That makes the plumbing visible
without conflating it with directory ownership or uncertain writes.

Both paths need the same domain contract. This is an **illustrative contract fragment**, not
approved manifest syntax; the effect/repetition behavior described above is also part of the
contract, however its eventual representation is chosen:

```yaml
operations:
  sha256:
    inputSchema:
      type: object
      properties:
        text: {type: string}
      required: [text]
      additionalProperties: false
    outputSchema:
      type: object
      properties:
        hex: {type: string, pattern: "^[0-9a-f]{64}$"}
      required: [hex]
      additionalProperties: false
```

Rust is used to make the example tangible, not to settle supported authoring languages. The
domain function could look like this; schema serialization, dependency declarations and tests
are omitted, and this snippet has not been compiled:

```rust
use sha2::{Digest, Sha256};

pub struct Input { pub text: String }
pub struct Output { pub hex: String }

pub fn digest(input: Input) -> Output {
    Output { hex: format!("{:x}", Sha256::digest(input.text.as_bytes())) }
}
```

### A. Conduit supplies the standard protocol adapter

The author's package might contain:

```text
digest-plugin/
  contract.yaml       operation contract; exact format still pending
  Cargo.toml          implementation and helper dependencies
  src/lib.rs         digest function
  src/main.rs        registers the function with the helper runner
  tests/             domain and contract tests
```

The entry point means **register `sha256` with this contract and handler; serve using the standard
adapter**. The helper supplies protocol dispatch, conversion/validation, request correlation and
standard reporting. The author implements the digest and its declared semantics. This need not
mean loading arbitrary Rust functions into the host: one possible implementation compiles the
handler and helper into an executable plugin. That implementation choice remains open.

### B. The author supplies the protocol adapter

The package still has the contract and digest function. Its entry point instead creates an
MCP/protocol service, registers discovery and invocation handling, adapts the request to `Input`,
calls `digest`, returns the typed result and integrates the required Conduit execution context
and reporting. The author can use a standard protocol library for much of this work.

This is not necessarily much more code. The official Rust MCP SDK already supplies tool macros
and service helpers; a Conduit helper could build on that same SDK. The distinction is who
supplies and maintains the **Conduit-specific adapter**, not whether one approach uses functions
and the other hand-writes a protocol. [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk#tools)

An existing server with its own request routing or service lifecycle may fit B naturally. A new
small extension may benefit from A's consistent defaults. Under either approach the public
contract and conformance checks must be satisfied; an ordinary MCP tool server alone does not
prove support for Conduit's execution, identity or recovery semantics.

### What a template sees

The template need not change between A and B. For example, a setup graph could fingerprint text
returned by an earlier, authorized corpus-read step:

```yaml
fingerprint:
  operation: {plugin: text, name: sha256}
  inputs:
    text: {from: output, node: read-corpus, port: text}
```

This is a node fragment. The enclosing template must declare the `text` implementation alias,
the `read-corpus` operation and the capability access that operation needs. The digest operation
does not acquire corpus access merely by receiving an input. Conduit resolves the exact
implementation and contract, orders the nodes, invokes the adapter and records the outcome.
No extra Resource is necessary solely to host this pure setup function. If digesting is to be
offered as a callable Resource capability, that exposure must be declared separately.

The shared authoring direction is therefore: implement the behavior, publish its contract, and
let templates bind its inputs. Initially, use only lightweight Conduit support and existing
libraries as needed. The amount of reusable adapter code can grow with experience; the public
contract must remain usable without depending on one comprehensive helper framework.

## 2. A directory Resource

### A concrete package to review

**Publication is accepted; this layout remains illustrative.** This is not a created package
or an executable implementation. A minimal filesystem contribution could
ship these files, whether assembled by hand or by a later build helper:

```text
filesystem-plugin/
  plugin.yaml                           package identity and export index
  contracts/
    local-directory.type.yaml            type/capability/runtime declarations
    directory-config.schema.json         configuration data rules
    directory-establish.operation.yaml   inputs, outputs and behavioral contract
  bin/
    filesystem-plugin                    implementation for the selected platform
```

Its source project would also contain implementation code and tests. Shipping source, building
locally or distributing platform artifacts are separate packaging choices. This diagram shows
the boundary between inspectable contracts and runnable code, not a required directory layout.
The package/export index is supporting metadata, not a sixth registered object family.

An illustrative `plugin.yaml` could index the contents like this:

```yaml
package:
  name: example.filesystem
  release: 0.1.0
implementation:
  entry: bin/filesystem-plugin
exports:
  types:
    - group: filesystem
      name: local-directory
      version: v1alpha1
      definition: contracts/local-directory.type.yaml
  operations:
    directory.establish:
      definition: contracts/directory-establish.operation.yaml
```

These are explanatory fields, not approved syntax. Exact artifact identity/digests, compatible
protocol versions and platform selection still need representation; a release label alone is
not the accepted implementation pin. One package can illustrate multiple exports without making
the package name the Resource type's logical group.

The configuration schema could contain this ordinary JSON Schema fragment:

```json
{
  "type": "object",
  "properties": {
    "path": {"type": "string", "minLength": 1},
    "establish": {"type": "string", "enum": ["create", "attach"]}
  },
  "required": ["path", "establish"],
  "additionalProperties": false
}
```

This checks shape, not the complete type contract. Instance paths must also satisfy the agreed
absolute-path rules, and access/ownership require registry and execution-context checks. The
operation contract references the configuration input, declares its directory-descriptor output
and describes create/attach, effects and recovery behavior. The type declaration connects those
contributions to its directory capability and applicable runtime/lifecycle behavior. Those files
are intentionally not expanded into a speculative full contract language here.

**Before launching plugin code**, Conduit could inspect this bundle and reject `establish: erase`,
an unknown setup export or a binding to an undeclared output port. It could validate and register
configuration even if the implementation cannot yet run on this machine. Required schemas must
be available, and registration still performs no filesystem setup.

**At execution**, Conduit still needs the exact compatible implementation, current authority and
the resolved target environment. The implementation checks the real path and performs the
declared operation. A valid schema cannot prove filesystem readiness, honest implementation,
safe recovery or future service availability. Runtime responses are checked against contracts;
conformance tests are needed for behavioral guarantees. A disagreement is reported, not treated
as permission to silently replace the bound contract.

Publishing the contracts makes validation independent of executable discovery, including before
the implementation can run on a target platform. A third-party MCP provider's changing tool
catalog is a different boundary: the MCP adapter's own contract can be published while its
upstream catalog remains dynamic.

The accepted boundary requires contract loading, validation and execution integration, not a
general SDK or generator. Contract/code drift must be tested regardless of whether authors write
or generate the descriptors. Exact file formats, field syntax, package layout and source of
truth remain for the final syntax review.

### What the template author supplies

The author wants a reusable recipe for creating a project directory. The existing filesystem
plugin supplies the directory contract and implementation. A template selects it and configures
establishment; a Pool subsequently provisions instances. A second template can attach an existing
directory using the same plugin. Neither change needs new Rust or Python code.

This uses the implementation packet's **proposed spelling**, not a published schema:

```yaml
apiVersion: conduit/v0alpha1
kind: Resource
form: template
metadata:
  name: project-directory
spec:
  type:
    group: filesystem
    name: local-directory
    version: v1alpha1
  implementation:
    package: example.filesystem
    release: 0.1.0
  config:
    path: /example/euphony
    establish: create
  setup:
    nodes:
      establish:
        operation: {plugin: owner, name: directory.establish}
        inputs:
          configuration: {from: config}
```

The example implementation reference is illustrative; artifact selection still resolves and
retains the exact implementation before execution. The `owner` alias refers to that code package,
not the Pool owner or a permission grant. This fragment shows setup; the selected type supplies
its directory capability/runtime contract, which need not require a dedicated long-running process.

The resulting Resource instance has one Pool owner and a concrete path in its execution
environment. An Agent-local view or symlink is preparation for authorized access, not a
second directory Resource. If another machine needs the files, it needs an appropriate remote
access/copy/mount capability; it cannot treat this path as its own local directory.

### What the plugin author supplies

Suppose directory support does not exist. The new implementation needs a typed configuration,
the establishment operation, the directory capability and any applicable lifecycle/control
behavior. The operation's domain procedure could be as small as:

```python
# Illustrative domain procedure; not a proposed Conduit SDK.
from pathlib import Path

def establish_directory(config):
    path = Path(config["path"])
    if config["establish"] == "create":
        path.mkdir(exist_ok=False)
    elif not path.is_dir():
        raise ValueError("Attachment requires an existing directory")
    return {"path": str(path)}
```

The typed contract permits only supported establishment values; the instance has already resolved
its path. This small function illustrates the domain procedure, **not a complete plugin**. It does
not itself establish path enforcement, recovery, cancellation or Pool ownership. A complete package
must identify which behavior is supplied by Conduit, the type's reusable implementation or its own
handlers. Native filesystem calls alone cannot establish a sandbox guarantee.

For example, if creation succeeded and the response was lost, finding the directory on retry
does not prove this operation created it. A recovery handler needs sufficient evidence; otherwise
the outcome remains uncertain. A helper runner cannot infer an exactly-once guarantee from `mkdir`.
These requirements should have reusable implementations where possible rather than being copied
into every author project.

### What this adds to the pure operation

| Situation | Expected behavior in this example |
| --- | --- |
| `create`, path absent | Create the directory and report the result. |
| `create`, path already present | Report a conflict; do not reinterpret creation as attachment. |
| `attach`, existing directory | Verify the target and return its descriptor without creating it. |
| `attach`, missing path | Report unavailable/missing; do not create it. |
| Directory created, reply lost | Reconcile using adequate evidence or report an uncertain outcome; do not infer ownership from existence. |

A plugin author can start with conservative recovery behavior that reports uncertainty where
evidence is insufficient; the helper must preserve that distinction. Providing a provider-specific
recovery handler is a stronger contribution, not something a wrapper can generate from the
function body. The same responsibility applies to both authoring paths.

In both cases Conduit still binds the exact implementation, establishes the caller and target,
checks the selected access path, tracks operations and retains inspectable outcomes. Plugin claims
must conform to their declared contracts; a convenient wrapper does not supply missing semantics.

## 3. A shared Linear MCP Resource

Here the underlying service already exposes a protocol. A user configuring it should normally
need templates and existing adapters, not a custom plugin implementation.

The template author composes a Pool with an authentication Resource and an MCP-access Resource.
The MCP template declares its authentication dependency; Pool composition binds it to the concrete
authentication Resource. The author specifies the service target and supported configuration,
while Agent templates select Conduits with narrower access. Concrete provider configuration is
left out here because no live connection or provider compatibility has been validated.

The conceptual invocation path is:

```text
Agent sees a permitted provider-native tool
  → selected Conduit validates the caller's complete access path
  → Pool-owned MCP Resource uses its authorized service binding
  → provider operation runs and returns its native result
  → Conduit records attributable evidence and delivers within current access
```

It does not require the Agent to call a generic `run_plugin` wrapper or expose authentication
tokens to every Agent. The existing provider MCP server does not have to implement Conduit's
instance lifecycle protocol: a Resource adapter supplies that boundary.

If a user wants to add an unsupported provider, the adapter author implements only the missing
integration behavior. Standard discovery, connection handling and common observation plumbing
should be reusable where compatible. If a native server cannot enforce a requested restriction,
the adapter must enforce it or leave that binding unavailable; it cannot quietly omit the rule.

Existing protocol servers and authored operations are compatible extension paths. A compatible
MCP implementation can fulfill a published contribution without new plugin-specific code.

### Code-optional MCP integrations

An existing MCP implementation can back a plugin, so a contributor can publish contracts and
bindings without writing new integration code. Independent MCP capability exposure is also accepted
with the managed/standalone boundaries below. No new YAML is proposed here; syntax is deferred.

| Contribution | What the author supplies | Where behavior runs |
| --- | --- | --- |
| Authored implementation | Contracts and implementation code. | The selected plugin execution environment. |
| Existing MCP backing | Contracts, an implementation/service reference and bindings compatible with a standard adapter. | An existing local or remote MCP service, with Conduit integration supplied by the adapter. |
| Graph composition | A typed operation contract with a graph binding its inputs/outputs to selected operations, including other graph-backed operations. | The existing graph execution machinery invokes implementations under the applicable instance and access boundaries. No separate executable merely to compose them. |
| Hybrid | Contracts, MCP bindings and code for behavior the standard adapter cannot supply. | The existing service plus the selected additional implementation. |

These are ways to fulfill contracts, not new object families or a closed taxonomy.
The published unit is **contracts plus an implementation binding**, rather than
necessarily a bundle containing a new executable. A service endpoint is not a credential, and
the package need not embed a concrete deployment or account. Templates/Pool composition can
supply those instance-specific selections as they already do for reusable dependencies.

For shared Linear access, a declaration could reuse the MCP-access adapter and reference the
provider service. Pool composition supplies the account binding; a Pool-owned Resource realizes
access; Agents attach through individually restricted Conduits. The adapter discovers and exposes
permitted native operations and correlates calls with Conduit actors and operations. It need not
require one authored Conduit operation definition for every provider tool. Where no additional
translation or behavior is needed, the contribution contains no new code. Both a locally launched
existing MCP server and an independently hosted server fit the idea; lifecycle obligations differ.

The declarations still have to describe support honestly. Standard discovery/invocation can be
reused; a contract cannot manufacture safe replay of a provider write or restricted access that
neither the adapter nor the provider can enforce. Additional behavior can be implemented when
needed, or the integration can expose its supported subset. Published contract/adapter versions
can be retained exactly; an externally operated service's code cannot be pinned by Conduit just
because its endpoint is recorded. Preserve observed compatibility and the accepted explicit
refresh/drift rules. This is a limitation of what can be guaranteed, not a demand for extra
per-call approval or a new security tier.

Independent use has two distinct meanings:

- An authored plugin or existing provider can expose ordinary MCP capabilities to clients
  outside Conduit, with only the dependencies that service actually requires. Those calls do
  not acquire Conduit-managed ownership, access, recovery or observability guarantees by name.
- A Conduit-managed Resource or other component can expose its supported capabilities/controls
  over MCP while retaining its governing authority and evidence. The Interface can aggregate
  inspection and management across those components; it need not become a single mandatory
  capability gateway. A minimal deployment still supplies whatever authority/runtime dependencies
  that component requires. Ordinary MCP connectivity itself grants no additional access.

Neither meaning requires a dedicated process per object, exposure of internal setup operations
to every client, or use of MCP inside every Rust module. The contract surface can be independently
addressable while an implementation shares an execution service. Pool-owned managed Resources
still have one owner. The first proof uses separate management and two executor processes on one machine; production deployment remains open. Shared policy, instance-owned state and no offline grants are accepted.

This direction follows MCP's independently operating local/remote servers and composable
capabilities. MCP supplies discovery/invocation; the additional Conduit semantics remain ours
to define and test. [MCP architecture](https://modelcontextprotocol.io/specification/2026-07-28/architecture)
and [tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools).

### Worked comparison: configuring search versus publishing a search capability

Consider a hypothetical existing MCP server that exposes document search. Keep its data storage outside this example;
we are discussing integration and search responsibilities, not combining all memory operations.

**Just use the service:** configure a template for the existing MCP-access type with the selected
service and authentication requirements. A Pool provisions its MCP-access Resource; an Agent
connects through a Conduit and sees the permitted provider-native search tools. No new plugin is
needed if this is the whole requirement. Endpoint/account choices are instance configuration,
not automatically a new extension contract.

**Publish a reusable search capability:** an author wants to supply a versioned search contract
that templates can depend on independently of one provider's native tool names. The
plugin publishes that contract, declares its MCP-access dependency and binds the required search
operation to a compatible provider tool. Pool composition supplies the concrete MCP Resource
through a Conduit. If ordinary bindings and the shared adapter express all needed behavior, this
plugin can contain declarations without a new executable. Its search Resource remains separate
from the MCP-access Resource, preserving their unitary responsibilities.

```text
Agent → permitted Conduit → search Resource
                              → permitted Conduit → MCP-access Resource → existing service
```

This optional search abstraction does not replace the provider-native tools for Agents that
prefer to use them directly. The names above describe responsibilities, not final syntax.
Installing the extension publishes a capability; a template configures an instance of it; Pool
instantiation creates the Resource and resolves its dependencies. Neither the package nor a
binding declaration owns credentials or bypasses the selected Conduits.

**When code is needed:** suppose the server returns several incompatible result formats, or
fulfilling the promised search contract requires provider-specific behavior beyond supported
bindings. The author supplies a small adapter implementation for that behavior. The contract
and consuming templates can stay the same where semantics are preserved. An ordinary MCP server
already implementing the required contract may be used directly through the shared adapter;
MCP compatibility alone is not proof of that semantic match.

The declared binding can supply delegation to a compatible operation while retaining typed
contracts, current access and honest outcome handling. A forwarding handler is not mandatory
when it adds no behavior. This does not rename every existing MCP Resource template as a plugin.

The conceptual distinction is: templates configure known contributions; plugins
publish reusable contributions and explain how their contracts are fulfilled. Whether the
implementation is code, an existing compatible MCP service, a graph of operations, or a combination
need not change that role.
Do not settle package/template distribution syntax or add a second object registry from this
example. Separate execution services own managed activity; concrete deployment, persistence and
production deployment details remain under review; the first one-machine process proof and v0 no-offline-grants policy are accepted.

## 4. A Resource with a persistent process

A local database needs more than a finite establishment function. Its type declares ongoing
runtime behavior; a setup graph can prepare storage, request instance-managed startup and run
initialization against it. The instance supervises the process after setup finishes. Supported
controls use their declared handlers, and observations report the actual execution and readiness.

| Concern | Template author | Plugin/driver author | Conduit runtime |
| --- | --- | --- | --- |
| Establishment | Choose configuration, dependencies and graph composition. | Implement provider-specific setup and recovery. | Validate bindings, order steps and retain valid progress. |
| Ongoing execution | Select declared runtime behavior and limits. | Supply start/inspect/stop/reconnect behavior as supported. | Assign execution custody, supervise and correlate evidence. |
| Configuration change | Select/narrow supported controls. | Establish what applies a change and proves its outcome. | Check permissions/revisions and retain confirmed versus uncertain state. |
| Connections | Declare compatible inputs and intended Conduits. | Implement supported capability/protocol behavior. | Preserve target identity, delegation and whole-path restrictions. |

An SDK can help implement these contracts; it cannot turn a one-shot script into a durable database
supervisor automatically. Conversely, the checksum or directory operation should not be forced
to implement database lifecycle hooks. Exactly which contributions are optional, generated or
provided by reusable types is part of the authoring review.

For example, the author's contributions might be `prepare_data`, a database runtime driver and
`initialize_schema`. The setup graph connects them in that order, requesting instance-managed
startup before initialization. The driver reports database-specific readiness and supports its
declared stop/recovery behavior; execution infrastructure supplies the agreed process-management
facilities. When setup finishes, the database remains under the instance's management. A helper
server ending does not itself establish that the database ended, nor does using a protocol
service make it the authority for the Resource's entire lifecycle. Those hosting choices remain
implementation choices within I1's authority boundaries and I3's helper execution default.
Distribution supports bundled plugins and explicit local installation; package syntax remains I7 work.

## 5. Where the example implementations execute

**Accepted:** endpoint templates select their plugins; endpoint instances manage implementations
and local bindings; Conduits match compatible capabilities and coordinate the relationship.
Conduit templates define relationship contracts by composing reusable typed requirements, with
a small base vocabulary and author extensions. Definitions may themselves compose existing
requirements while retaining every constituent's meaning and limits. Detailed publication,
composition and satisfaction mechanics remain under review. Supervised instance-associated helpers
are the accepted default for authored code; exact grouping, SDK API and YAML syntax remain open.
The [concept map](../../DESIGN.md#authoring-concepts-beneath-the-object-families)
separates working vocabulary from the five object families; final naming stays open.

### Plugin, template, graph and instance

A plugin publishes typed behavior with an implementation binding: authored code, existing MCP
services, a graph composing operations, or a combination. It can contribute setup operations,
capability implementations, runtime behavior or supported control handlers. These are roles a contribution can fulfill, not a required export
list for every plugin or a new set of object families. The same package can contribute to several
families without combining their instances or authority.

A template selects and configures these contributions, binds dependencies, orders finite setup
steps, declares ongoing runtime behavior and exposes supported controls. An instance realizes that
recipe with its own identity, resolved definition, concrete bindings, confirmed settings and
runtime/operation evidence. A graph orders plugin operations for setup or a supported change;
it is not the ongoing process supervisor. Registration alone executes nothing.

An instance's logical boundary does not dictate its process count. Its responsible service can
host trusted handlers, run an instance-specific helper containing several plugins, supervise child
processes or attach external implementations. Code reuse does not implicitly share private state,
authentication or grants. Supported external-service association does not imply owning that service.

### Publishing a graph as a reusable operation

**Accepted:** the temporary terms and divisions in the concept map remain our working vocabulary;
final names may change. A graph can implement a typed operation contract, and a plugin can publish
that composite operation just as it publishes an operation backed by code or an MCP service. The
caller uses the declared contract; the implementation composes selected operations from other
plugins. This is a composite plugin from the author's perspective, using the existing contribution
model; exact declaration syntax remains open.

For example, an illustrative `prepare-linear-access` operation accepts an authorized authentication
binding and workspace selection, then uses a graph to validate the binding, prepare the MCP client
and inspect compatible provider capabilities. Its declared result describes the prepared access;
it need not expose every internal output or credential. A Resource setup graph can invoke this
operation as one step, while another template reuses it with different permitted inputs. Required
instance-managed services remain supervised after finite preparation completes.

The author publishes the external operation contract and graph binding instead of writing glue
code for the same sequence. Individual operations can be reused separately where exposed and
permitted. A plugin may export multiple operations, and a private setup graph need not be packaged
as a plugin. Graph composition is not necessarily one package or process per node.

The composed operation must describe its actual effects and recovery behavior. A successful login
followed by a failed connection is partial work, not an automatically rolled-back transaction.
Graph-backed operations reuse retained child progress through graph recovery by default, leaving
uncertain effects to child recovery contracts. Exact parent/child coordination remains open.
Nested composition retains v0's finite graph limit and selected implementation/access boundaries.

### Derived and authored composite results

Use an illustrative database graph: establish the database and its managed runtime, apply a
migration, then prepare consumer access. Each operation publishes its own typed outputs and
failure/effect semantics. The same graph can support a derived public result or an explicitly
constructed domain result; the following names are explanatory, not approved schema identifiers.

| Observed execution | Constituent information available for derivation | Possible authored public result |
| --- | --- | --- |
| All required steps complete | Database reference, migration evidence and usable access output. | `database-ready`, with selected references and schema information. |
| Migration reports failure after startup | Database/runtime still exists; migration reports its known effects; later access preparation has not run. | `setup-incomplete`, identifying the retained database and migration failure without presenting it as ready. |
| Migration response is lost | Startup is confirmed; migration's effects are unconfirmed; ready access has not been established. | `setup-unconfirmed`, with the known database reference and unresolved migration outcome. |
| Start fails after backing creation | Backing data is retained; no successful runtime start or later steps are claimed. | `startup-failed`, with permitted retained-resource and failure references. |

Derivation can compose declared types and failure sources. It cannot infer that a failed migration
left the old schema intact or that a timed-out operation had no effect. The Interface retains
inspectable child progress under normal evidence/access rules, even when the public result groups
several child failures into one domain result. The caller need not receive all intermediate payloads.
The existing default keeps a started runtime managed after setup failure; this table does not add
automatic retries or rollback.

**Accepted authoring default:** derive the public result contract from declared public output
bindings and constituent contracts, with explicit construction available. Both preserve relevant
effects, uncertainty and scoped evidence without publishing all intermediate values. Exact field
names and mapping syntax remain open.

**Accepted recovery default:** a published graph-backed operation reuses graph recovery without
requiring a bespoke handler merely for composition. Explicit retry of this unchanged invocation
reuses still-valid startup progress and delegates the uncertain migration effects to its recovery
contract before proceeding as that contract permits. If it cannot establish a safe continuation,
the graph cannot infer one. This does not change automatic retry policy or promise rollback.
[Detailed coordination and custom recovery hooks](CONTRACTS.md#composite-outputs-and-outcomes--proposal)
remain open.

### One Linear connection through the authoring concepts

This is a conceptual walkthrough, not final package names or graph syntax:

| Concept | In this example |
| --- | --- |
| Plugin contributions | Authentication and MCP integration contracts with code or service bindings that implement them. They may be packaged together or separately. |
| Offered capability | The MCP-access Resource supports discovery and invocation of provider-native tools through its declared integration. |
| Relationship requirements | A reusable `review-access` definition combines permitted read operations, the selected workspace and a response-size bound. Every part must have endpoint support. |
| Policy | Authoritative application of requirements, such as a workspace restriction governing the Resource across Conduit paths. Uses the same requirement definitions and supporting plugins. |
| Grant | Records the particular access actually issued to this Agent/participation; declaring `review-access` or sharing provider credentials does not issue it. |
| Callable operations | Resolve authentication, prepare a connection and inspect its catalog. Grant administration is also an ordinary protected capability operation; requirements constrain the proposed grant. Each operation need not be a separate plugin. |
| Setup graph | Orders those operations using their typed outputs and dependencies. Runtime startup, where needed during setup, remains instance-managed. |
| Templates | A Pool template assembles the auth and MCP Resource entries; Resource/Agent templates select their implementations; a Conduit template declares the relationship and restrictions. |
| Instances | The Pool owns the concrete Resources. Endpoints manage shared authentication/client state and local bindings. The Conduit retains the concrete relationship and participation. |

After setup, an Agent's native Linear tool invocation uses its selected Conduit path. Endpoint
implementations enforce applicable requirements and invoke the shared Resource's provider access.
The setup graph does not run anew for every call, and its successful completion is not permanent
authorization. Per-call work, ongoing services and connection participation retain their existing
operation and instance lifetimes. The Interface exposes their correlated state and outcomes.

The [Policy and Grant contracts](AUTHORIZATION.md) define issuance, current authority and
delegation through the same protected operations. Policy applies existing requirement contracts;
there is no separate policy rule language or mandatory policy-plugin class. A small plugin author does not need a separate IAM
server: its contract declares operations/requirement duties, while common runtime integration
supplies trusted invocation context and the selected authority checks. Endpoint enforcement and
attribution still need to be implemented wherever that plugin accepts responsibility.

This is why a plugin is not a Resource: the same implementation can serve separate Resources with
different Pool owners, workspaces and authentication. A requirement is not a setup node: its condition
may need to hold on every invocation. A Conduit is not a graph: the live relationship can outlast
its establishment procedure. Final naming should preserve these differences.

### The five families

| Family | Template selects and declares | Plugin behavior serving an instance | Boundary to preserve |
| --- | --- | --- | --- |
| Interface | Frontend integration, inspection/interaction behavior and management connections. | Present observations, collect input and invoke supported operations through Conduits. | A view of an Agent or Resource is not a second authority over its state. Management effects execute at the relevant management/target capability. |
| Agent | Native or owned runtime, account/provider boundary, activation, initial settings, supported endpoint roles, connection declarations and controls. | Prepare private state, establish the declared runtime, manage local connection bindings, handle interaction and supported setting changes. | A remote tool call remains attributed to this Agent but uses the target Resource's capability through a Conduit. |
| Resource | One capability responsibility, creation/attachment intent, dependencies, runtime and controls. | Establish a directory, maintain service access, authenticate, search, store or expose other declared behavior as appropriate to its type. | Every Resource instance has one Pool owner. A plugin package is not itself a Resource, and these examples need not be bundled into one Resource. |
| Pool | Explicit named Resource-template entries, composition bindings and supported management actions. | Coordinate explicit provisioning/reuse and child dependency binding; inspect owned Resources. | Child setup and ongoing capability state belong to the child Resource. Pool ownership does not require co-location. |
| Conduit | Endpoint roles/contracts, connection establishment, filters, transport/interaction behavior, supported changes and cleanup. | Coordinate endpoint preparation, compatible bindings and restrictions; correlate connection progress, reconnect and release participation. | Endpoint templates select the plugins; endpoint instances manage local execution/bindings. Conduits coordinate compatible capabilities and participation. |

An instance context must exist before its setup effects are dispatched, so incomplete setup has
an identity and owner for inspection/recovery. The detailed creation sequence remains open. Starting
from a template retains the resolved recipe; supported controls can update confirmed settings.
Adding a compatible Conduit uses a declared attachment mechanism, rather than silently rewriting
the endpoint's fixed graph or controls. Under endpoint-managed pairing, the endpoints retain their
plugin implementations and the Conduit retains any orchestration implementation it uses. A new
binding is not an implicit implementation upgrade. Genuine new endpoint code requires replacement
under the fixed-definition rule; new parameters or native tool catalogs can use existing contracts.

### Endpoint-selected plugins and Conduit matching

Endpoint templates select the implementations providing their connection capabilities. Instances
retain those implementations and manage local bindings. Conduits select compatible capabilities,
connection requirements and restrictions; they do not select or install new endpoint plugin code.
This is the accepted authoring arrangement, not a requirement for separate worker processes.

For example, an Agent template selects its MCP-consumer implementation and a Resource template
selects its provider implementation. Their instances can establish multiple separately attributable
bindings using those supported capabilities. Detachment releases the affected binding without
removing the endpoint's shared plugin/client or other participation. Fixed-definition changes still
require replacement; compatible new settings and native provider catalogs follow existing rules.

The rationale is clear local supervision, implementation retention and reuse of shared services.
The tradeoff is that endpoints must already support the behavior a connection requires. A genuinely
new implementation requires a revised endpoint template/new instance; a new service target need
not require new code when existing capabilities support it. Matching is about compatible behavior,
not identical packages or a closed list of named plugin pairs. Exact satisfaction checks remain open.

The Conduit remains active: its graph or orchestration implementation requests preparation,
coordinates binding, applies declared connection policy through supporting endpoints, aggregates
readiness and requests release. It owns the relationship and its observations, not a second copy
of endpoint configuration or credentials. This does not imply every data call passes through a
separate coordinator process.

Filesystem exposure is a useful minimal pair. A directory Resource's plugin validates/provides
the authorized directory capability; the Agent's filesystem-integration plugin prepares the local
view/symlink. The Conduit binds them and their constraints. On detach, the Agent-side plugin removes
that participation's view and the Resource-side plugin releases any associated binding. The folder
remains. Either side can finish setup without a continuously running process; a remote filesystem
would need an appropriate different capability, not reinterpret a remote path as local.

### A Conduit with behavior at both ends

This example illustrates the accepted endpoint-managed arrangement. Consider Agent A using the
Pool-owned Linear MCP Resource L. The Pool also owns authentication
Resource H; L already has authorized access to H through a separate Conduit. Opening C between
A and L could establish:

```text
Agent A's supported attachment surface
    ↕ client-facing MCP binding serving C's participation
    ↕ transport / filters defined by C
    ↕ resource-facing binding serving C's participation
Resource L's shared MCP capability
    ├── authorized authentication Conduit → Resource H
    └── provider calls → existing provider MCP service
```

The layers are logical responsibilities, not a required process per line or two implementations
of every filter. Existing compatible endpoint behavior can satisfy one side without deploying code.
The target side must enforce restrictions it is responsible for; client-side tool hiding alone
cannot establish server-side authorization. Exact enforcement placement is contract-specific.

The endpoint-managed ownership split is:

| Activity/state | Governing instance or contract |
| --- | --- |
| Agent-facing exposure and local binding implementation/state | Agent A's endpoint plugin; its binding records the C participation it serves. |
| Resource-facing exposure and local enforcement/binding implementation/state | Resource L's endpoint plugin; each binding retains caller and participation attribution. |
| Selected endpoints, connection policy and logical participation | C; local bindings must satisfy this relationship's current restrictions and endpoint limits. |
| Shared provider client and MCP capability | Resource L, usable by other authorized Conduits. |
| Provider identity/credential lifecycle | Resource H under its own capability and access rules. |
| Agent private context and runtime settings | Agent A; supported changes use its controls and revision rules. |
| A provider operation | L executes the capability with caller A and path C recorded; current permission and effect/recovery rules apply. |

Thus a request can have an initiating Agent, a Conduit carrying it and a target Resource executing
it. The Conduit is active implementation, not a passive link, but it does not become the authority
for every effect it transports. Each endpoint service reports its own effects; C records correlated
connection progress. No second writable copy of endpoint configuration or atomic distributed setup
is implied. The Interface assembles the observations with their known freshness and gaps.

A Conduit targeting a Pool can select or compose access to its permitted Resource capabilities.
That grouping does not require a Pool process to relay every call or move Resource execution into
the Pool. Explicit provisioning remains distinct from selecting an already deployed Resource.

On detachment, C requests that the endpoints release the participation's local bindings and any
helpers exclusive to them; endpoints retain responsibility for execution and reporting. Other
participants and independently shared Resource clients remain. The last logical consumer/peer
ends C under the accepted lifecycle rule. A lost transport is not detachment; an uncertain cleanup
or in-flight provider write must be reconciled rather than declared undone. Exact helper custody
and cleanup acknowledgement remain review choices.

### Other connections test the same distinction

| Example | Connection-specific behavior | Independently retained behavior/data |
| --- | --- | --- |
| Directory exposure to an Agent | Prepare the permitted local view/symlink and release that attachment on detach. | The directory Resource and backing files. A symlink itself does not enforce filesystem permissions in native mode. |
| Agent-to-Agent retained messages | Establish each participant's post/read binding and supported next-turn notice integration. | Pool-owned message history and independent reader state; posting does not wake a model automatically. |
| Interface changes an Agent's model | Carry the authorized typed control request and its response/progress. | The Agent's handler applies the change within its fixed provider/account boundary and records confirmed configuration. |
| Search Resource reads storage | Establish the search-to-storage capability path and restrictions. | Separate Resource responsibilities, backing data and supported search/index lifecycle. |

### Composable Conduit contracts

**Accepted direction:** compose reusable typed requirements, with a small base set and author-defined
extensions. Each versioned definition owns its meaning, parameters and applicability independently
of any implementation; multiple endpoint plugins can support it. Plugins publish supported versions,
endpoint roles and known limitations for pre-establishment matching, followed by applicable runtime
verification. Requirement contracts declare when guarantees must hold and relevant endpoint-role
duties; established Conduits record responsible selected implementations for Interface inspection.
Detailed declaration syntax, matching and evaluation remain proposed in
[CONTRACTS](CONTRACTS.md#composable-relationship-requirements--proposed-contract). A Conduit template
states relationship requirements; endpoint-selected plugins supply capabilities that satisfy them;
a Conduit instance binds concrete endpoints and retains current state/evidence. Requirements are
reusable contract material, not another family or an executable instance per rule.

For example, a read-only MCP relationship can combine compatible consumer/provider behavior,
permitted native operations, a provider-aware target restriction and bounded returned content.
The same endpoint plugins can support another Conduit with different allowed targets or limits.
They implement the reusable requirement meanings, not a list of user-authored template names.
Native MCP discovery remains dynamic; this does not require copying every provider tool schema
into the Conduit definition or replacing familiar tool names with a generic wrapper.

| Author task | What is authored | What need not change |
| --- | --- | --- |
| Restrict an existing relationship | Compose known requirements with values and selected endpoints in a Conduit template. | Plugin code and core dispatch, where selected endpoints already support the constraints. |
| Package a reusable specialization | Accepted: a requirement definition composes existing requirements with explicit references and chosen parameters. Every constituent keeps its meaning and limits. | No new executable when endpoints support every part. Exact packaging/composition syntax remains open. |
| Introduce new behavior | Publish its requirement meaning, parameter schema and implementation support/checking/enforcement where needed. Select supporting plugins in endpoint templates. | The five families and generic core dispatch; publication alone does not add code to existing instances. |

**Concrete extension example:** a fake deployment service accepts a typed region argument. An
external author adds an allowed-region requirement, including the region-list schema, its precise
meaning and endpoint implementation support. The Conduit permits a chosen region; the endpoint
checks the actual dispatch parameters before the fake provider effect. Unsupported endpoints
cannot claim fulfillment, and another request cannot bypass the rule by reusing setup-time proof.
This is a proposed synthetic example, not a claim about AWS CLI interception or real cloud support.

The example separates authorship into three jobs: the requirement author defines what must hold;
the endpoint implementation author supplies support; the Conduit author composes that requirement
with others. One person/package can do all three, or a second endpoint author can implement the
same allowed-region contract without redefining it. Each implementation publishes its supported
versions/roles and known limits; for example, direct deployments may be supported while a separate
operation that indirectly chooses a region is not. This declaration is inspectable before setup;
current login and enforcement still need runtime verification. Definitions and implementations can
ship together while static validation remains possible without executing code. Requirements do not
instruct the runtime to download code, provision a missing capability or widen authority.

Filesystem access illustrates a different composition: a directory capability, consumer-local-view
support and applicable placement/cleanup requirements. Authentication is a runtime fact; a path or
symlink is not proof of enforced read-only access. Requirement declarations must describe the
actual guarantee rather than infer it from a label. Retained messaging can add delivery/history
obligations backed by a Pool-owned Resource; these are not implied by a transport connection.

The proposal complements existing graphs: requirements describe obligations and restrictions;
graphs call declared endpoint operations to establish prerequisites; instances manage ongoing
behavior. A satisfied parameter schema does not prove behavioral correctness, authorization or
recovery. Plugin tests and runtime evidence remain necessary where the requirement calls for them.

Requirement composition keeps individual enforcement, with optional known-conflict or combined-effect
checks. No general solver or merge algorithm is required from custom authors. Default graph recovery
and declared fulfillment duties are also accepted. The combined proof of composed operations and
requirements in the existing synthetic fixture is accepted; it does not select a permanent core
catalog or authorize implementation start. I3's conceptual review is complete; persistence,
credential custody and evidence capture are next. Detailed evaluation interfaces, custom recovery
hooks and syntax remain open.

The accepted execution default runs locally authored code in supervised helper processes
associated with the served instance, retaining supervision/state in its responsible service.
Helpers can contain ordinary plugin crashes and support language-neutral implementations; an
instance-specific runner can host multiple compatible handlers. Trusted built-in code can remain
in-process. Helper startup and protocol plumbing are the tradeoff for separating authored code
from the executor memory space.
Existing MCP services keep their own supervision. None of these choices automatically
supplies an OS sandbox, process-per-instance rule or exactly-once effects. A worker crash after
creating a directory can leave an uncertain result; a database setup helper ending must not orphan
the database's instance-managed supervision. Exact process grouping, helper packaging and transport
remain open, with lightweight author support and MCP as the public contract.

## 6. Accepted direction and remaining boundaries

Our existing design already distinguishes code, typed contracts, templates, graphs and instances.
The accepted implementation priority is the public protocol and extension structure, with
extremely lightweight support from Conduit. Most integration behavior should be authored as
plugins early so it tests that structure and speeds development. A broad helper library is not
the prerequisite; additions can follow demonstrated repetition and difficulty.

Both examples may already look like small handlers when built with good libraries. A minimal
adapter is useful where it removes incidental work, but effects, recovery and lifecycle behavior
remain explicit plugin obligations when applicable. Publication of inspectable contracts with
implementation bindings and MCP as the public invocation base are accepted. Existing-MCP
bindings, graph-backed operation composition and independent component use join those boundaries.
Management/execution separation, shared policy with instance-owned state and no offline grants are also accepted;
the first proof uses separate management and two executor processes on one machine. I2 accepts
native Claude/Codex adapters before the owned Rust model/tool loop, with macOS-first native
integration/account validation and a portable core. I3 now accepts endpoint-selected plugins;
supervised instance-associated helpers are the default for locally authored code. Detailed
fulfillment evidence, connection cleanup and helper grouping remain open. Distribution starts with
bundled first-party plugins and explicitly installed local packages; declarations binding an
existing MCP service need no new executable. A marketplace and automatic dependency resolution
across languages are deferred. Installation does not instantiate, authenticate or grant access.
Package, contract and template syntax is deferred to I7; these boundaries impose no new server framework.

The first experiment should prove both a new plugin contribution and a template-only composition
without modifying core dispatch. Record the author-written files, generated artifacts and actual
diagnostics. Performance work can follow that demonstration; a fast built-in-only demo would not
prove Conduit's extension protocol.
