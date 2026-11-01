# Plugin and template authoring walkthrough

Status: authoring examples, 2026-11-01. Lightweight Conduit support with the public protocol and
extension structure prioritized over a helper library is accepted. Publishing inspectable
contracts with implementations is also accepted. Detailed contract and template syntax is
deferred until conceptual review is complete. MCP is accepted as the public invocation base with
a small Conduit profile. Code-optional implementation bindings and independent component access
are accepted; profile details, SDK, package syntax and execution arrangement remain open.
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
| Plugin package | Published contracts with bindings to authored code, existing MCP implementations or both. A package can contribute operations or type implementations; it is not automatically a Resource instance. |
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
implementation is code, an existing compatible MCP service, or both need not change that role.
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
implementation choices within I1's accepted boundaries and the pending I3 packaging review.

## 5. Where the example implementations execute

**I3 remains open:** after reviewing these examples, the user asked to define the instance's
execution boundary before choosing plugin workers. The
[instance boundary proposal](ARCHITECTURE.md#instance-execution-boundary--proposed-for-i3-review)
places setup/change handlers and ongoing behavior in their owning instance's context, while
leaving physical hosting separate. An instance-specific runner may host several plugin handlers;
the protocol does not require one worker per plugin. The arrangements below are implementation
comparisons that preserve the same public MCP contract, templates and instance ownership.

```text
Separate worker: executor → MCP → plugin executable/runner → domain behavior
In-process:     executor → loaded implementation          → domain behavior
```

In the second arrangement, an adapter still supplies the accepted public MCP surface. An internal
function call is an implementation detail, not a replacement public plugin protocol. In either
arrangement the responsible instance service retains authoritative configuration and operation
results. A worker is an implementation process, not a sixth object family or a Resource created
for every call. Choosing separation would not yet choose worker counts, sharing or idle lifetime.

| Example | Separately supervised implementation | Implementation loaded into executor |
| --- | --- | --- |
| Pure checksum | Author publishes the contract and handler; a small executable or runner receives MCP calls and returns digests. It needs no managed Resource merely to run a setup node. | The same domain function runs in the executor through a supported loading/extension interface. This avoids the worker launch and interprocess exchange. |
| Directory establishment | A filesystem worker validates create/attach intent, performs the filesystem operation and returns its evidence. The Pool still owns the directory Resource; the executor records setup progress. | The implementation performs the same operation inside the executor. Resource identity, permissions and recovery requirements do not change. |
| Shared Linear access | Existing provider MCP needs no newly authored worker. A declaration can reuse a standard adapter; custom adapter code, when needed, follows the selected authored-code boundary. | Existing provider MCP remains external. A custom adapter instead runs inside the executor. Neither arrangement duplicates the provider service for each Agent. |
| Managed database | A worker implements setup/migration/driver behavior. The executor starts and supervises the declared database runtime on behalf of its Resource instance. | The same driver code runs inside the executor, which still manages the database runtime separately from a finite setup call. |

For the directory example, an Agent-local symlink remains attachment preparation, not a second
directory Resource. A worker exiting does not delete the directory or imply that its creation
failed. If it created the directory but lost its response, the operation needs reconciliation;
process separation supplies no exactly-once guarantee.

For the database example, the intended arrangement is:

```text
executor ── supervises ── database process (Resource runtime)
    └────── MCP ──────── setup/driver worker
```

The setup worker can finish while the database continues. If a migration attempt crashes, keep
the database under supervision and inspect/reconcile the operation before retrying, following
the already accepted partial-setup policy. Restarting a worker is not automatically permission
to replay a migration. The worker cannot be the only custodian of a database process handle or
of evidence needed to recover its effects.

For Linear, sharing concerns the Pool-owned service capability and authorized account binding.
Each Agent retains its own attributable Conduit participation and restrictions. Native tool names
and schemas can remain visible regardless of adapter placement. An existing service supplies its
own supervision; an adapter process is additional only when the chosen implementation needs one.

The tradeoff is **process plumbing versus a shared failure boundary**. Separate workers add
startup, serialization and supervision, but let authors implement the wire contract without a
host-language loading ABI and help contain an ordinary implementation crash. Loading code into
the executor can be simpler for tightly integrated trusted code, but an abort or unsafe failure
can affect other instances hosted by that executor. Neither arrangement inherently limits host
filesystem/network authority, CPU or memory; permission isolation is a separate policy and
implementation concern. MCP is compatible with either arrangement.

First settle the instance boundary, then choose how to host its authored code. Separate helpers
and in-process trusted core functionality remain candidates alongside an instance-specific runner.
Use only the minimum author support needed for real plugins; a broad SDK is not a prerequisite.
These are proposals for review, not accepted deployment rules or tested isolation guarantees.

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
bindings and independent component use now join those accepted boundaries. Management/execution
separation, shared policy with instance-owned state and no offline grants are also accepted;
the first proof uses separate management and two executor processes on one machine. I2 accepts
native Claude/Codex adapters before the owned Rust model/tool loop, with macOS-first native
integration/account validation and a portable core. I3 execution/packaging choices come next. Detailed
package/contract/template syntax is deferred to the end. These rulings do not choose the
host/worker arrangement or impose a new server framework.

The first experiment should prove both a new plugin contribution and a template-only composition
without modifying core dispatch. Record the author-written files, generated artifacts and actual
diagnostics. Performance work can follow that demonstration; a fast built-in-only demo would not
prove Conduit's extension protocol.
