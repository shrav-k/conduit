# Proposed repository and delivery organization

Status: implementation proposal, 2026-11-01. Directory/crate names below describe the intended
layout, not files already created. [Architecture](ARCHITECTURE.md) defines responsibilities;
[closed review](../archive/reviews/IMPLEMENTATION.md) records accepted implementation boundaries.
The [current review](../REVIEW.md) distinguishes remaining decisions from deferred engineering choices.

The newer [repository/distribution review](../archive/reviews/REPOSITORY-2026-11-02.md) accepts one main repository under a
Conduit organization, with independently consumable public protocol/conformance, runtime and extension
surfaces. Publication details remain under review. Use this document's crate/dependency/milestone
details as earlier candidates. The documentation migration is complete; neither proposed code tree
is adopted. Resolve source/package boundaries against the first authoring slice before scaffolding crates.

I1 accepts separate management and execution services; crate boundaries must preserve that
arrangement and the accepted shared-policy/instance-state authority split. The
[research](../research/DISTRIBUTED_RUNTIME.md) informs independent execution tests. Binary names,
concrete deployment and detailed failure mechanisms remain implementation work. I4 selects the
minimal public persistence plugin contract, service runtime dependency and bundled SQLite engine
with separate local management/executor databases in the first experiment. A single workspace
and lockfile do not combine execution and management custody. V0 has no offline grants: retain
supervision/results while holding new managed requests without verifiable permission; test this
behavior rather than treating separate processes as proof of it. The first proof runs management
and two independent executors as separate processes on one machine, with controlled connection
failures; two-machine validation follows before remote support is claimed.

## Review and PR sequence

The [conceptual review](../archive/reviews/CONCEPTUAL.md) is complete, with the
[collaborator proposals](../archive/collaborator-proposals/REVIEW.md) reconciled and their dissent preserved.
This does not settle package-reference syntax.
A later authoring PR will review contracts, template/package examples, MCP/profile mappings and
conformance fixtures together. Implementation milestones follow that approved boundary; a detailed
crate plan is not authorization to start them now.

The actual documentation layout is the [overview](../OVERVIEW.md), [protocol reference](../protocol/README.md),
[authoring path](../authoring/README.md), one active [REVIEW](../REVIEW.md), reference implementation
design, deferred syntax proposals and research. Experiments remain outside docs with their own
documentation. The source-tree layout below is a historical planning example, not a list of directories
already created or a requirement to create every proposed crate.

## 1. Repository layout

Use one Cargo workspace and one lockfile for the application, workers and first-party plugins.
Keep reusable public schemas independent of Rust source paths. Experiments stay outside docs,
and private registry data never belongs in the repository.
This organizes the authored Rust code; a code-optional integration package needs neither a Cargo
crate nor a private worker. It can publish contracts/bindings and reuse an existing MCP adapter.

```text
conduit/
  Cargo.toml
  Cargo.lock
  rust-toolchain.toml
  README.md
  crates/
    conduit-core/
    conduit-config/
    conduit-runtime/
    conduit-store/
    conduit-transport/
    conduit-plugin-sdk/
    conduit-builtin/
    conduit-app/
  schemas/
    conduit/v0alpha1/
    worker/v0alpha1/
    fixtures/
  packages/
    builtin/
      manifest.json
      contracts/
      templates/
  tests/
    conformance/
    integration/
    recovery/
    compatibility/
  tools/
    xtask/
  docs/
    SPEC.md
    DESIGN.md
    REVIEW.md
    research/
    proposals/implementation-v0/
  experiments/
    001-shared-access/
    002-runtime-plugins/
```

Create each crate/directory when it has real responsibility and tests. Do not begin by generating
eight empty abstraction layers. Early code can establish core/config/store boundaries first;
the first real worker determines the final transport/SDK split. Avoid a `utils` crate, per-family
microservices or a crate per graph operation.

Cargo workspaces support shared dependency declarations, package metadata, lint settings and a
lockfile. Choose an explicit edition/resolver/toolchain and verify dependency compatibility; do
not infer a minimum Rust version from whichever compiler happens to be installed locally.
[Cargo workspace reference][cargo-workspaces]

## 2. Crate contracts and dependency direction

| Crate | Owns | May depend on | Must not own |
| --- | --- | --- | --- |
| `conduit-core` | Family/form types, identity/revision types, contract descriptors, pure access/lifecycle rules and narrow host ports | Small serialization/error/type dependencies | Tokio execution, SQL, provider SDKs, filesystem effects, UI |
| `conduit-config` | Marked strict parsing, schema registry, defaults, binding/DAG compilation and diagnostics | Core and parsing/validation libraries | Plugin execution, network schema retrieval, automatic file watching |
| `conduit-runtime` | Supervisors, graph scheduling, invocation coordination, recovery, route/participation and Interface services | Core, config, Tokio; ports for store/plugins/environment | SQLite implementation, provider-specific HTTP, terminal rendering |
| `conduit-store` | SQLite implementation, migrations, bounded store thread, transactions, backup and evidence queries | Core and storage libraries | Provider effects or policy bypass through direct SQL |
| `conduit-transport` | MCP client/server adapters, local bridge, worker wire mappings and negotiated feature handling | Core, Tokio, rmcp; dispatch ports | Independent business rules, a second registry or hidden retry policy |
| `conduit-plugin-sdk` (optional extraction) | Minimal recurring descriptor/operation/driver helpers and conformance support, only as actual authoring needs justify them | Core and transport helpers | Application runtime, a prerequisite framework or unrestricted access to its database |
| `conduit-builtin` | First-party filesystem/history/auth/MCP/native-agent implementations, shared adapters and fixture exports | Transport/integration libraries; minimal helpers where useful | Reimplementing host authorization or defining global caller identity |
| `conduit-app` | Dependency wiring, `conduit`, management/executor entry points, workers and terminal attachment (binary/command layout proposed) | Implementations above | A second source of domain semantics in CLI handlers |

Keep core ports narrow and use them where there are real implementations: SQLite versus a test
store, process worker versus a fake, clock/process control versus deterministic fault fixtures.
Do not abstract every standard library call. Unvalidated YAML/JSON cannot construct a trusted
dispatch context; validation and authentication produce distinct internal types.

For composable requirements, the proposed allocation is definition/parameter interpretation in
config, generic contract/support descriptors in core, and binding/runtime coordination in runtime.
Endpoint implementations provide requirement-specific support. These are responsibilities within
the existing candidate crates, not a requirement for a new crate or hard-coded core variant for
every extension. Definition reuse across implementations is accepted; publication mechanics,
support/satisfaction syntax and the first vocabulary remain authoring work; I3's reusable definitions,
support declarations and composition boundaries are accepted. No new expression-language
dependency follows from the accepted design direction.

The terminal initially lives as a client module in the app crate, not a second authoritative
runtime. Extract a reusable client crate if multiple real frontends need it. Native provider
adapters begin as modules in the built-in package; separate distributable packages when independent
release/security/dependency requirements justify it. Package boundaries and Resource type groups
remain independent.

## 3. Dependency recommendations

These are selections to test, not a claimed compatible lockfile. Pin exact tested resolution when
implementation begins and record enabled features. Avoid `latest` in build instructions.

| Need | Candidate | Selection condition |
| --- | --- | --- |
| Async execution | Tokio + tokio-util cancellation/task tracking | Cooperative cancellation, bounded queues and explicit child ownership pass the recovery tests. |
| Serialization | Serde + serde_json | No silent coercion; public numeric limits and canonicalization are tested. |
| YAML syntax | yaml-rust2 marked event parser | All strict-profile fixtures reject/accept correctly, with source locations and resource limits. |
| Data schemas | jsonschema, offline registry | Explicit supported 2020-12 subset, reference/format behavior and bounded validation cost. |
| Rust schema assistance | schemars where useful | Generated drafts reviewed against public artifacts; dependency updates cannot silently change a contract. |
| Durable local storage | rusqlite with a pinned SQLite build | Durability, migration, backup and contention behavior tested on target platforms. |
| MCP | Official rmcp SDK | Exact negotiated versions/features tested with fixtures and later native peers. |
| Diagnostics | tracing / tracing-subscriber, optional OpenTelemetry bridge | Separate committed evidence and bounded diagnostics; exporter loss cannot block core outcomes. |
| Terminal | Ratatui candidate, terminal I/O backend selected with it | A thin Interface client supporting reconnect, unknown states and keyboard-safe interruption. |
| Credentials | Explicit platform-store adapters; keyring-core/store ecosystem is a candidate | Actual macOS interaction, headless behavior and error handling verified. No silent plaintext fallback. |
| WASM | None required initially | Wasmtime/Extism only after a measured use case and host-call/lifetime tests. |

The JSON Schema crate supports offline reference handling, but its documented coverage differs
across drafts/features. Test the subset we publish. Schemars can derive schemas; that convenience
does not establish protocol compatibility. Ratatui is a Rust terminal library, not an agent runtime.
[jsonschema][jsonschema] · [Schemars][schemars] · [Ratatui][ratatui]

Use maintained libraries for transport, crypto primitives, parsing and storage. Write Conduit's
identity, ownership, graph and recovery semantics ourselves. Do not import a general actor or
durable-workflow framework until a concrete boundary benefits enough to justify its lifecycle
and deployment assumptions.

## 4. Schema and package organization

**Accepted source of truth:** published, language-neutral contracts, including typed schemas and
behavioral definitions. Authors may handwrite or generate them, then explicitly review and version
the released contract. Rust types and other implementations conform to it; internal refactors do
not implicitly change the public API. Schema checks and behavioral conformance fixtures should
test that agreement. Plugin packages supply schemas under exact contract identities; exact dialect,
layout and generation tooling remain I7 work. Host compilation need not enumerate every future
Resource type.

The accepted authored-schema profile rejects unknown keywords while providing one dedicated map
for namespaced custom descriptive annotations. Reuse standard descriptive annotations where they
fit; custom tooling hints do not implement validation, authority or runtime behavior. The map keyword,
namespace syntax and value schemas remain open. Provider MCP schemas retain their own dialect rules.

The core uses explicit family/form enums, while type/capability/group names remain extensible.
Plugin configuration can be represented internally as validated JSON with a known schema identity;
it is not an unvalidated catch-all. A changed description does not authorize changing semantics
under an existing contract version. There is no backward-compatibility or migration-availability
promise between releases; any migration supplied must be explicit and tested. See the
[accepted compatibility rule](../protocol/CONFIGURATION.md#release-compatibility).

**Accepted distribution scope:** bundle first-party plugins and support explicit installation of
locally supplied plugin packages. Defer a marketplace and automatic dependency resolution across
programming languages. The layouts below are proposed mechanics within that scope.

Each package has one explicit entry document declaring exports and dependencies, with definitions
inline or referenced in package-local files. Load that declared surface without executing plugin
code or inferring exports from directory conventions; merely packaging a file does not export it.
The document can be handwritten or generated, and its concrete filename/fields remain open.
Validate public export-name uniqueness across contribution kinds and inline/referenced definitions
before execution. Do not resolve a collision by kind or load order; references still check the
expected kind and contract after selecting one export. Internal names and registry naming are separate.
Resolve explicit re-exports through declared dependencies while preserving the originating definition,
contract and internal reference scope, along with resolved dependency identities. A local public alias
does not require a wrapper, change behavior or grant access; the same export validation still applies.
Resolve template implementation selections through one named dependency map and alias-plus-export
references for setup, runtime and controls. Check each use's expected kind/contract; there is no primary
alias. Keep package-internal scope and resolved identities intact. Concrete field names remain open.
Package manifests describe contracts and how contributions are fulfilled: authored exports,
bindings to existing MCP implementations, graphs of operations, or a combination. Retain exact
declarations and selected adapter/code artifacts; report external service compatibility separately. A manifest is not another Conduit object.
The accepted common package envelope supports plugin contributions, template-only collections and
combinations, with each contribution retaining its typed contract. Installing a package does not
instantiate its templates, grant access or authenticate an account. The first-party package may supply several Resource
groups. No mandatory public marketplace, namespace service or build DSL is needed in v0.

The proposed coordinated application release bundles a schema set, first-party manifests and
platform worker artifacts; that release arrangement remains a candidate, not an early delivery gate.
The first implementation prioritizes a working protocol and authoring proof; exact release tooling
and component packaging can adapt to that work without requiring legacy compatibility code.
Some packages contain only declarations/templates; they must not be forced through a native build
or executable-install step. Installation alone does not contact or authenticate their providers.
Installing or capturing revised source must not overwrite code used by a running instance. An
instance's recorded binding still identifies its selected implementation; this does not require
an archive of edit-mode revisions for future recreation. Deliberate preservation uses an explicit
versioned publication of fixed contents through a public or private distribution source. Conduit
resolves and verifies the selected package; it need not own publishing, hosting or release storage.
Released contents remain fixed independently of instance retirement; publishing changes requires
another version. Publication/acquisition and removal mechanics remain open, and a separate local-only
release facility is not selected.
Host protocol/storage changes need
compatibility checks even where plugin code is unchanged. A newer host can drop older contract/store
support and report the incompatibility. Keeping the prior working host/package set is distinct from
adopting the update; repair, migration or replacement may be needed and need not be automated.

## 5. Local state and development artifacts

Use explicit service-local state directories, with a platform application-data default and an override
for tests/parallel registries. Management and each executor have separate stores; a common installation
root does not make their database files shared. Resolve paths once and record registry identity. A project checkout
is not implicitly a registry, and changing the shell directory does not select another account.

```text
<service-state>/       # one authority service, not a shared management/executor database
  state.sqlite
  runtime/             # socket, OS-held lock, current host metadata
  artifacts/           # instance-required contents; digest-indexed storage is a candidate
  payloads/            # committed content references with ownership metadata
  agent-state/<id>/    # private adapter/runtime state, not shared history
  edits/               # recoverable checked-edit baselines
  staging/             # incomplete artifacts/payloads; explicit cleanup evidence
  backups/             # explicitly requested consistent snapshots
```

This is a proposed local installation layout, not a shared filesystem required across machines.
Credentials normally live in their selected protected store; native runtime-owned credential files follow the adapter contract and are not blindly copied here.
Treat the entire registry as private user data. Exclude runtime state, accounts and experiment
payloads from Git. Backups preserve access implications and reference completeness; ordinary
configuration export is deliberately a different operation.

Development preserves code/contracts for running work. After editing a plugin, the ordinary workflow
can retire the affected test instance and create a replacement from the current source, normally in
a disposable registry. Backing-data reuse follows the type's attachment contract. There is no need
to keep both versions running or make old private snapshots available as selectable packages. Git can
retain authored history or support release publication; it is not a required execution backend. A temporary
edit-mode snapshot or cache is an execution mechanism, not a release. Version and release a selected
implementation through a public or private distribution source to preserve it independently of its
instances; no Conduit-operated marketplace is required.
Do not hot-reload code into an existing instance
and claim the immutable-definition contract still holds. Record only synthetic fixtures in the
repository; keep real provider test evidence scrubbed and outside public fixtures.

## 6. Delivery milestones

Keep initial authoring support deliberately lightweight. Implement real plugins against the
public protocol before extracting a broad SDK/helper library; reuse existing libraries or small
adapters where they remove incidental work. Contract conformance remains required regardless
of helper maturity. Extend tooling from repeated authoring needs rather than delaying the first
working extension path to design a general framework.

Plugin/template authoring is a first-class acceptance path throughout these milestones. The
first-party integrations must exercise the public extension boundary, and a small separately
authored plugin must load without a core-code change. Record the files and code the author actually
needs, including any generated manifest/schema artifacts; do not hide essential setup in the
developer's environment. SDK/runner details remain implementation work; syntax is deferred to the authoring pass.

Reuse existing infrastructure where its behavior can be adapted and tested against Conduit's
contracts. Extra process hops or conservative serialization are acceptable initially. Contract
validation, ownership, access and honest effect outcomes are not optional efficiency tradeoffs.
Measure costs after the authoring-to-instance path works; optimization is not its entry gate.

C1's accepted first conformance threshold requires an independently authored client and plugin to
work with the reference runtime using published contracts/tests. A second full runtime is deferred;
client/plugin success does not establish portability between runtimes. Fold that authoring evidence
into the early milestones without treating schema validation alone as behavioral conformance.

Each milestone produces an inspectable vertical slice, not just more abstractions. I2 accepts
native Claude/Codex adapters after the synthetic proof, followed by the owned Rust model/tool loop.
Native integration/account validation is macOS-first, with a portable Rust core and OS-specific
adapters. Other platform support needs separate evidence. The finer milestone breakdown below
is still a proposal;
no calendar estimates are implied.

| Milestone | Deliverable | Exit evidence |
| --- | --- | --- |
| M0 — executable language contract | Core types, strict parser/default pass, versioned schemas, fixture validation and template-to-instance draft expansion | Existing YAML packet differences explained; duplicate/alias/unknown/null/default cases covered; no effects during validation. |
| M1 — durable management | Shared registration/policy bootstrap, checked template writes, owner Interface CLI, source-aware observation/catch-up and recoverable edits | Restart retains IDs/revisions; stale writes and unavailable repair paths are visible; store/export failure tests. |
| M2 — plugin and lifecycle slice | Typed finite DAG, authored plugin plus code-optional MCP binding, graph-backed operation, composed/custom requirements, instance-owned runtime/configuration/results, control coordination and recovery | Experiment 002's accepted combined authoring proof and one-machine/two-executor layout; derived/explicit results, valid child progress, fulfillment evidence and enforcement without core-specific dispatch. Preserve exact controlled artifacts, external compatibility, uncertain-effect recovery and duplicate-execution prevention. Authored code uses supervised instance-associated helpers; exact grouping remains open. Passing tests must supply the evidence. |
| M3 — shared Resource slice | Local directory, retained history, synthetic auth/MCP service, independently usable managed MCP endpoint, composite participation and terminal inspection | Two synthetic Agents and a standard MCP test client exercise permitted access; Interface evidence survives frontend detachment; independent readers, reconnect and no automatic wake. |
| M4 — first real Agent path | Native Claude/Codex adapters first, with CLI/native interaction and a minimal Pi-inspired Interface terminal | Separate account and private-state verification, actual managed MCP exposure and explicit coverage gaps. |
| M5 — owned model loop | Rust context builder, one supported provider adapter, streaming and authorized tool execution | Deterministic fake-provider recovery first; real identity/access verified; transcript/usage distinctions preserved. |
| M6 — expanded integration | Required two-Claude/two-Codex experiment; optional Xerj backend and additional provider adapters | Recorded account/refresh/isolation evidence; backend outage/coverage tests; no secret or private-context crossover. |

M4 precedes M5 under the accepted delivery order. The four-account goal is not considered
satisfied by four fake identities or four API keys. Likewise native support does not complete
the Conduit-owned-loop objective.

The first environment driver can report native-only support. A requested isolated mode must fail
clearly until an actual isolation driver passes filesystem/process/network tests. Shipping a full
v0 claim that advertises isolated execution requires that evidence; do not quietly advertise an
unimplemented security tier. Production remote deployment, automatic wake, public plugin
distribution and advanced sharing remain later work. Serialized executor boundaries and controlled
connection failures are accepted for the initial spike; testing them locally is not proof of
production remote support.

## 7. Test strategy

| Layer | Meaningful tests |
| --- | --- |
| Pure rules | Stable references, whole-path permission composition, typed constraints, revision/equality behavior and ownership. |
| Parser/schema | Marked errors, duplicate keys, scalar edge cases, anchors/tags, defaults, graph cycles and schema references. |
| Store | Transactions, uniqueness, stale CAS, retained uncertainty, interrupted migration/backup and payload-reference consistency. |
| Worker protocol | Malformed/truncated frames, oversized outputs, missing features, wrong artifact, worker exit and cancellation/end distinction. |
| Code-optional integration | Offline contract validation, compatible provider binding, unsupported behavior, catalog drift and unavailable upstream; no new package-specific executable required. |
| Graph recovery | Every boundary before/after intent, external effect, result commit and observer publication; valid reuse versus invalidation. |
| Managed access | Separate routes cannot combine rights; revocation at dispatch/read/delivery; unsupported provider filters do not run. |
| Interface | Reconnect from cursor, snapshot/subscription race, slow observers, pending/uncertain controls and frontend detach. |
| Independent MCP use | Ordinary client invokes a managed component through its authorized path without the aggregate frontend; same identity, access checks and retained evidence; external client detachment preserves the provider. |
| Provider compatibility | Exact versions, actual identity, independent account refresh/logout, catalog changes and missing telemetry. |

Use injectable clocks and deterministic fault points. Kill real worker processes in integration
tests; an in-memory mock cannot establish process cleanup. For the external-effect window, the
fake provider's ledger must be independent from the host store. Resource-sensitive tests use
temporary roots and explicitly verify preserved backing after detach/deletion.

Default CI is offline and synthetic: formatting, linting, focused unit/integration tests, schema
fixtures and source/doc checks. Live account/model tests are opt-in and individually documented;
no CI credential or cost dependency is introduced into ordinary builds. Avoid broad repeated
tests after the relevant checks pass unless new failures or changes justify them.

Use benchmarks to decide worker multiplexing, Wasm or store batching. Record machine, versions,
payload sizes and repetitions. Measure startup/idle cost, invocation latency, queue bounds and
recovery/cancellation latency. Do not set marketing performance claims from a source README.

## 8. Change and review workflow

Implement one vertical behavior at a time on focused branches from the approved implementation
baseline. Each change includes the relevant contract, an observable result and failure evidence.
Keep refactors separate from semantic changes where that helps review. Native-provider-specific
logic belongs in adapters; a repeated cross-provider need is evidence for promoting a core port.

PR descriptions should state the concrete before/after behavior, test evidence and coverage gaps.
Require explicit review for protocol/version changes, permission or recovery semantics, storage
migrations and dependency changes affecting artifact retention. Ordinary naming/refactoring does
not need another product ruling. No mandatory ADR for every helper or field.

Keep documentation responsibilities clear:

- [Overview](../OVERVIEW.md) summarizes the model; [protocol topics](../protocol/README.md) own detailed accepted behavior.
- Implementation documents identify accepted boundaries and proposed mechanisms separately; a file move does not approve a mechanism.
- Experiment folders own runnable fixtures, test plans and dated actual results.
- Research owns source evidence and alternatives, not a competing requirement set.
- Package docs explain their contracts, capabilities, recovery and limitations using the common format.

Record the configuration actually tested: host/storage versions, worker-profile support,
package artifacts and tested native/provider versions. A concise inventory is sufficient initially;
this is not a requirement to test or maintain combinations of old releases. Before a supplied storage migration, create
a consistent backup and refuse a newer unsupported store. Failed migration does not start partial
runtime work. Downgrade is explicit restore/reconciliation, not an assumption that old code can
read new data. Cleanup must preserve code required by live activity or unfinished recovery. Historical
instance/evidence records do not alone require retaining edit-mode executable artifacts. Separately
preserved releases follow their own removal rules, not instance retirement. Later recreation from
unreleased source is not guaranteed; unavailable code must be reported without substituting a newer implementation.
Exact cleanup mechanics remain open under the [accepted development boundary](../protocol/CONFIGURATION.md#implementation-selection).

## 9. Research and validation limits

The proposal combines the source-pinned [Rust runtime report](../research/RUST_RUNTIME.md)
with primary documentation accessed on 2026-11-01. No Cargo workspace, dependency resolution,
MCP conformance test, keychain interaction, native login or provider request has run as part of
writing it. The dependency table is a testable plan, not a fabricated build result.

[cargo-workspaces]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[jsonschema]: https://docs.rs/jsonschema/latest/jsonschema/
[schemars]: https://docs.rs/schemars/latest/schemars/
[ratatui]: https://ratatui.rs/
