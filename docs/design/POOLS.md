# Pool contracts

Part of the accepted [design contracts](../DESIGN.md). Updated: 2026-11-02.
The [conceptual review](../REVIEW.md) records completed rulings and deferred work;
syntax and implementation examples do not amend these rules.

## Pools

Users compose a Pool through an Interface, referencing reusable Resource templates and their
establishment settings. These entries describe what the Pool can provision; listing or adding a
template does not create/attach a Resource instance, authenticate a service or connect an Agent.
Available templates and the Pool's owned Resource instances are distinct views, not new object
families. "Catalog" is only shorthand for that available-template view; it does not require a
separate object, package registry or service. A Pool template describes availability, provisioning
settings, policies and permitted management/configuration changes. A Pool instance creates/attaches selected
Resources through explicit provisioning or declared setup-graph steps and exclusively owns the
resulting Resource instances. Pools remain the resource-supply and shared service-access boundary
for Agents and connections, including skills,
tools, memory and connection support such as conversation storage. A Pool need not run a process.

**Dependency composition:** reusable Resource templates declare their required connections;
Pool composition supplies concrete targets during selected Resource instantiation. The consumer
instance supplies its own local endpoint and retains the resolved Conduit declaration/target binding. For example,
one reusable MCP template can connect to Euphony's authentication Resource in a work Pool and to a
personal authentication Resource in another Pool without embedding either concrete identity in the
reusable template. This also applies to other Resource dependencies, such as search using storage.

This separates reusable requirements from creation-time wiring. It is not automatic account
selection, a new template-parameter DSL or permission granted by Pool membership. Keep one resolved
connection assignment on the consumer instance, with derived reverse views. Later template/Pool
edits do not silently rebind established connections. Compatibility uses the capability contracts
above; exact declaration syntax, version matching and explicit rebinding controls remain open.

**Local composition:** Pool templates name their Resource entries locally, allowing a
dependency binding to refer to another entry in that composition. For example, `linear` binds its
authentication requirement to `auth`. Provisioning the selected composition resolves that reference
to the concrete authentication Resource for this Pool, and the consumer retains its resolved stable
target ID. Catalog entries alone have no deployed Resource identity. Other Pool instances own their
own deployed Resources; template reuse alone does not share instance identity. Local entry names
are not global registry aliases or an alternative instance identity; referencing one grants no access.

**Provisioning intent:** templates/graphs explicitly distinguish provisioning from a
named Pool entry from using a selected existing Resource instance. The former creates a Resource
through its owning Pool; the latter binds an already-registered instance without changing its owner.
Both resolve to a concrete Resource binding. An entry is a reusable provisioning choice, not a
singleton instance or an implicit reuse-or-create instruction. A matching template/name alone must
not decide between those actions. Pool composition or explicit creation inputs can supply an existing
target without embedding a particular deployment's identity in every reusable template.

For example, two tasks can provision separate worktrees from one entry, while several Agents share
the Pool's existing authenticated Linear Resource through their own Conduits. A new service can bind
the same authentication Resource rather than provision another login. Provisioning with attach
behavior is different: it creates a new registration around existing backing, such as a local folder.
Composition carries the choice without a separate operation manifest or a manual question on every
start. Authority, compatibility and explicit binding rules still apply; retry reuses valid recorded
progress rather than treating a provision instruction as permission to create duplicates.

**Composition mechanism:** a reusable graph step consumes the composition's explicit
provisioning/existing-instance choices and dependency bindings. It establishes newly requested
Resources through their own setup contracts and resolves existing targets without treating their
selection as a new provisioning request. It exposes concrete results for subsequent bindings.
A declared setup graph may establish a chosen composition; catalog membership alone
must not cause all entries to deploy. Custom graphs can reference the same entries for extra
ordering or checks; authors need not restate Resource configuration or hand-wire every component's
setup. Required dependencies govern preparation, not YAML mapping order. This preserves each
Resource's runtime ownership, authorization and recovery contract. Exact operation names, typed
results and graph syntax remain open.

**Partial availability:** ready Resources are normally usable through authorized Conduits
while other components of their Pool are waiting or have failed. Each Resource's own prerequisites
and any declared gates still apply. A ready checkout can be used while shared authentication waits
for consent and Linear remains unavailable. Report the Pool's incomplete setup and component
states; do not infer readiness merely from a running process. Successful partial work remains
retained under the existing recovery rules. Aggregate status/completion fields and explicit
whole-Pool gate contracts remain open; this adds no automatic retry or cleanup behavior. Templates
merely available in the catalog are not pending/failed setup components; only requested setup
contributes such progress and readiness obligations.

Creating a Pool registers its identity and configuration. Provisioning is explicitly requested
to create or attach Resources through their owning Pool. A frontend may offer both in one
explicit action with separate outcomes; successful registration does not imply provisioning.
No background reconciler is implied.

Use the accepted local composition declarations, resolved instance bindings and ordinary operation
evidence without requiring a second provisioning-plan ledger. Pools group unitary Resources and
derive useful views from their contracts. Exact composition fields and result wiring belong to the
configuration profile in [REVIEW.md](../REVIEW.md#review-outcomes). Stable identity, explicit
effects, safe retry and separate backing-data cleanup remain requirements.

**Label removal:** Pool inclusion uses explicit named Resource-template entries.
V0 has no generic registry-object labels or label-based selectors, including for discovery or
automatic Pool inclusion. Inspect/query defined fields such as kind, form, Resource type, owning
Pool and explicit relationships. No label inheritance or selector-refresh contract is needed.
Adding/removing an available-template entry is distinct from provisioning/retiring its deployed
Resources; availability changes do not silently delete instances or retarget existing bindings.
Knowledge-record classification and typed Conduit filters remain separate. Reusing templates
creates distinct owned instances, not overlapping ownership; Pool schemas stay open to new types.

Connection participants can access Resources in several Pools through authorized paths to
the owners. Sharing backing data between separate instances does not transfer ownership or
establish access. Pool ownership, current access, Agent activation and native backing-data
management have distinct meanings.

Creating a Resource for a connection must identify its owning Pool; the connection requests
provisioning through that Pool. Provisioning retries reuse valid recorded progress; any further dynamic support requirements
need an explicit type contract. A template registration alone does not create Resources.

**Connection selection:** omit Pool-provided default Agent connections in v0.
Agent templates declare starting Conduits, optionally referencing reusable composite Conduit
templates. Authorized Interface/Agent operations explicitly add or remove connections later.
Pools make provisioning templates and owned Resources available; they do not automatically suggest
or add a connection set when an Agent uses the Pool. This avoids a second source of launch choices
and its merge/precedence rules. Defaults inside Resource configuration remain supported and are
unrelated to Agent connection selection. Current authority and Agent limits still constrain each
connection; declaring one does not issue its own grant.

**Retirement:** keep a Pool instance registered until its owned Resource instances
have been explicitly retired through their supported lifecycle and checked unregistration.
One explicit bulk action can retire the Resources and then remove the Pool. If retirement only
partly succeeds, preserve the Pool and report remaining work; do not claim rollback of completed
steps. This also covers Resource registrations created during incomplete Pool setup. Ordinary
Pool deletion does not implicitly cascade retirement, and ownership transfer is deferred in v0.
Stopping a managed client does not itself delete a remote account or stop a shared external
service. Backing data and retained evidence follow their separate cleanup/retention contracts.
Missing ownership remains an inconsistency to resolve, never implicit orphan-purge authority.

### Shared access and authentication

Pools are the normal place to establish and manage shared service access. Their
owned capabilities retain the selected provider bindings so multiple authorized Agents can use
them without each completing provider login. This applies beyond MCP: a suitable integration can
manage database, repository, cloud or credential-service access. Actual provider support and any
interactive renewal requirements remain explicit.

A binding means a managed association to an authorized service identity, not a copied secret in
configuration. Plugins implement authentication; instance runtimes own continuing state/refresh.
The Pool owns and organizes the capability rather than introducing a second authentication engine.

**Representation:** reusable authentication is a Pool-owned Resource, accessed by service
Resources through Conduits. It owns the reusable authentication/refresh lifecycle; consumers own
their service behavior and clients without becoming separate owners of the login. Pool templates
can compose these Resources and connections so one integration does not require manual authoring
of every helper. This adds no family and does not make each token or login attempt a Resource.

**Credential-custody implementations:** for secrets Conduit itself retains, the default is an
OS credential store, initially macOS Keychain on the first validated platform. Supply custody
through the existing plugin contract/implementation-bundle mechanism: a minimal typed custody
contract with a bundled OS adapter, open to later password-manager or credential-service
implementations. This is an adapter choice, not a new family or a requirement to implement those
additional services in v0. Ordinary configuration, SQLite records and events contain secret
references rather than the secret values; locked, denied or unavailable custody is reported
without silent plaintext fallback.

The authentication Resource still owns the selected service identity and refresh behavior; the
custody implementation provides supported secret-storage/access behavior. Native Agent login
and externally managed secret Resources keep their declared ownership. A package may also supply
a standalone password-manager Resource type, but a package itself is not a Resource and installing
it grants no access. Implementations must declare supported behavior and limitations; selecting a
backend cannot imply unsupported write, renewal or recovery capabilities. Existing implementation
selection/replacement rules apply; a new backend does not silently migrate secrets or reinterpret
existing references. Exact contracts and package syntax remain open.

**Enrollment and unlocking:** authorized setup/control operations use the selected adapter's
supported flow, with the Interface presenting required interaction. Reuse existing authorized
access; when human action is needed, the affected operation reports that need and the user
completes the native/provider flow. Unattended execution cannot complete that step until access
is available. The instance retains progress, and continuation or re-establishment follows the
operation's recovery contract. Unrelated work can continue. Passwords and tokens stay outside
ordinary conversations and logs; the Interface does not take over credential custody.

One authentication Resource can provide supported bindings for several service consumers; tokens,
audiences and provider grants may differ even when the underlying account is the same. Sharing
the account never establishes credential compatibility. Its exact type group/name, operations,
connection binding and handling of non-reusable internal authentication state remain open.

| Concern | Contract |
| --- | --- |
| Service authority | The provider identity and grant bound to the capability; reuse is intentional and must be supported. |
| Caller authority | The Agent/Interface identity and its current permission to use that capability. Shared login does not mean identical rights. |
| Connection restrictions | Each Conduit narrows permitted Resources, operations and parameters. Pool membership alone does not grant access. |
| Credential custody | Keep upstream authentication with its managed owner. Ordinary capability use does not hand the provider token to the caller. Secret retrieval/injection is a separate explicit operation where supported. |
| Transport | Shared identity does not mandate one socket/client/process. Clients can be pooled or separate while refresh has one coordinated owner. |
| Attribution | Record the actual caller and Conduit locally. The provider may attribute all calls to the shared service identity; do not invent per-Agent upstream attribution. |
| Lifetime | Detachment releases the flow's own state, not shared credentials or another consumer's client. Login failure/revocation can affect all users of that binding. |

The selected binding's provider permissions, Resource policy, caller authority and Conduit filters
all constrain use. An additional authorized Agent attaches to the existing capability; it does
not rerun its service login as a mandatory attachment step. Revoking just that Agent's access leaves
the binding available to others. Templates declare supported shared setup and management controls;
binding replacement, logout, refresh failures and idle runtime policy need explicit type semantics.

Per-consumer service identities remain an explicit exception when selected or required by the
provider. Several identities can target the same service. Matching endpoints or using a common
template does not implicitly combine accounts, grants or OAuth state. Cross-Pool reuse also needs
an explicit authorized path to the owning capability, not duplicated ownership or copied tokens.
Shared authentication does not merge private Agent context or selected model subscriptions.

For example, an Euphony Pool owns an authentication Resource and a Linear Resource connected to
it through a Conduit. A review Agent reads issues while
an implementation Agent may update them through a different Conduit. Removing the review Agent
preserves the implementation Agent's access. The Resource's authentication state and each caller's
connection/authorization state remain independently observable. See the
[full walkthrough](../research/CONDUITS.md#5-access-linear-through-mcp).

