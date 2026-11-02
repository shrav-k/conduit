# Conduit specification

Conduit is an extensible protocol for connecting independent Agents to shared Resources and to one
another. Its durable product is the contract: authors can publish small plugins, compose them into
repeatable templates, establish instances and inspect/control them through compatible Interfaces.
A Rust reference implementation and replaceable frontends will demonstrate that contract.

Status: conceptual design, 2026-11-02. No runnable implementation exists. The
[design topics](DESIGN.md#contract-map) hold accepted detailed behavior. The
[review](REVIEW.md) records completed conceptual closure; specific authoring syntax is deferred.

## Five families

Every family has a reusable template form and a concrete instance form. Instances have identity,
configuration and state; they do not necessarily require their own process.

| Family | Meaning |
| --- | --- |
| Interface | Active management and observability endpoint. Users attach CLI, terminal or test clients; the root Interface directs foundational setup and explicit repair. |
| Agent | Autonomous participant with a selected runtime/account boundary, private context, supported controls and Conduit attachments. |
| Resource | One unitary capability with related operations, established and owned by exactly one Pool. |
| Pool | Organizes available Resource templates and exclusively owns its provisioned Resource instances, including shared authentication. |
| Conduit | An ongoing relationship with concrete participation and composable requirements, implemented through compatible endpoint behavior. |

Pool ownership is logical, not a process or placement rule. A Pool can eventually own Resources on
several hosts. A Conduit can connect Agents, Resources and other supported endpoints without creating
a new object for every request or socket. Supporting durable content belongs to Pool-owned Resources.

## What powers the families

| Concept | Responsibility |
| --- | --- |
| Contract / capability | Public typed meaning and behavior; a capability describes what an endpoint offers. |
| Operation / graph | A callable action and a finite dependency composition of actions. Graphs can implement reusable operations. |
| Plugin | Published contracts plus implementation bindings to code, existing MCP services, composed operations, or combinations. |
| Requirement | A reusable, versioned obligation with parameters, applicability and enforcement duties; requirements can compose. |
| Policy / grant | Authoritative application of requirements, and recorded permission on a particular Conduit participation. |

These are extension concepts beneath the families, not additional template/instance object stacks.
Temporary names may evolve without changing their responsibilities. Published language-neutral
contracts are authoritative; implementations conform. Schemas alone cannot describe every effect,
recovery or lifecycle guarantee.

Conformance describes explicitly supported capabilities. A limited client/plugin need not implement
an entire runtime; a runtime claiming portable template execution must implement its public graph,
control, lifecycle and recovery semantics. Physical runtime mechanisms remain replaceable. Exact
profiles remain authoring work. The first conformance milestone uses an independently authored
client and plugin against the reference runtime and published contracts/tests. A second complete
runtime is deferred; that initial proof cannot establish runtime-to-runtime portability.

A template supplies typed configuration, its setup graph, declared ongoing behavior and permitted
controls. It can bind compatible contributions from several plugins without a mandatory primary.
Standard templates and declared defaults simplify authoring, while instances retain explicit resolved
bindings. Plugins are not required to contain new code or run a separate server each.

## One user journey

1. The local owner starts the root Interface from a locally available definition. Its graph establishes
   the system Pool, management/access-management Resources and ordinary management Conduits/grants.
   Later startup reuses them; missing components require explicit repair.
2. Through the Interface, the user composes an application Pool with named Resource-template entries.
   Listing an entry does not deploy it, authenticate an account or grant access.
3. Explicit setup provisions selected Resources or selects existing ones. A shared authentication
   Resource can support a Linear integration through a Conduit, while a directory Resource exposes
   the project files. Ready Resources can be used while unrelated setup is incomplete.
4. Agents start from templates with their own model subscriptions/private state and declared Conduits
   to permitted capabilities. They reuse shared service authentication under their individual grants.
5. Users and authorized Agents inspect objects, add/remove connections and invoke supported controls.
   The Interface relates each request to its effects, progress, failures and evidence.

Native provider tools preserve their familiar contracts where supported. A managed MCP endpoint may
proxy them without forcing a generic replacement tool or one combined gateway. Shared provider login
is distinct from the calling Agent's authorization and from its model subscription.

## Construction, controls and recovery

Registration performs no provisioning or model work. Explicit establishment executes a finite graph
of typed operations; output bindings establish dependencies. Provider branching/polling remains in
plugins initially. The instance owns ongoing processes, including a runtime started during setup.
Graph completion, readiness and execution status are separate observations.

Instances retain resolved definitions and exact Conduit-selected implementations. Later template edits
do not alter them; definition or implementation upgrades require replacement in v0. Named typed controls
can change their permitted settings. The single stored configuration advances only after application
is confirmed; pending/uncertain requests are recorded separately. Controls normally serialize per target
and check its inspected revision, with contract-declared safe exceptions.

Failed setup preserves successful work by default. Explicit retry reuses only valid progress and
reconciles uncertain effects. A timeout does not prove failure; reconnection is not permission to replay
a write or start model work. Stop, detachment, unregistration and backing-data purge have distinct effects.
Retirement stops new work and permits unregistration only after required shutdown duties are
confirmed. Preserve shared dependencies and operation evidence; local shutdown does not prove an
external effect was cancelled. Missing confirmation leaves retirement incomplete.

## Identity and access

The accepted authoring envelope uses `apiVersion`, `kind`, `form`, `metadata` and `spec`. Inspection
adds system-owned `status`; ordinary authoring rejects it. Strict typed YAML excludes ambiguous implicit
conversions, duplicate keys, anchors/aliases, merge keys, custom tags and undeclared fields. Exact further
package/schema formats remain proposals.

Stable IDs survive renaming; names are unique per family across forms. Authored kind/form/name references
resolve to IDs that remain bound until explicitly rebound. Creation/update intent is explicit; template
edits are checked complete replacements, and editing a file alone does not change active configuration.
Instances materialize defaults and export current stored settings, not live credentials, grants or progress.

Ordinary managed access requires a recorded participation grant, including root management access.
Governing requirements apply independently of administering Interfaces. Every applicable restriction on
the selected path must pass; separate paths cannot mix operations and targets into broader permission.
Requirements and Pool membership do not grant access. Grant administration checks current authority;
issued grants do not depend on issuer ancestry or continued activity. Recipient detachment, withdrawal
and current restrictions govern use. V0 has no offline grants.

Resources serving callers use their own dependency grants while preserving authenticated caller
context and enforcing applicable restrictions on effects and returned content. Autonomous maintenance
uses the Resource's own authority. A caller need not receive direct access to internal backing services;
a broader internal grant cannot widen what the caller can cause or receive.

Endpoint instances select/manage their implementations; Conduits coordinate compatible capabilities,
requirements and participation. Required setup failures block preparation, optional failures are reported,
and later dependency failures normally allow unrelated work. Ending the last logical participation ends
a Conduit; later access uses a new instance. A transport outage alone does not end participation.

## Shared content, communication and observation

Memory, search, workspaces, integrations and authentication are Resource capabilities. The accepted
memory contracts include revisioned records, provenance, bounded scoped search, direct read-only sharing
and declared source dependencies for derived content. Native service data keeps its provider contracts.
Resources stay unitary; storage, search and integration remain distinct responsibilities by default.

The initial communication capability is a Pool-owned append-only shared history with independent
read positions and current-access checks. It supports explicit participation, bounded catch-up and
state inspection. Next-turn unread notices do not automatically wake Agents. Reading is not evidence
of understanding or task acceptance. Other communication modes remain extensible. Linear/Jira and
specialized message tracking are ordinary Resources, with no mandatory tracker abstraction.

Control and observability are inseparable in the Interface. Local authoritative operation/recovery
evidence survives frontend closure and optional exporter failure. A lightweight observation bundle
adds correlated traces/diagnostics, small health metrics and optional OpenTelemetry export; plugins can
add CPU/token or other domain observations. Report freshness, uncertainty and coverage. Real payload
capture needs explicit policy; credentials stay out of ordinary logs. Retained controlled content/evidence
remains until explicit purge, with external retention limits reported.

## First implementation and limits

Management owns registration/templates and shared policy; one designated service owns each instance's
confirmed configuration and outcomes. Executors supervise managed activity; backends own domain data.
The Interface aggregates sources rather than becoming another writable authority. Initial persistence
uses a minimal public plugin contract with bundled SQLite as a service dependency. Secret custody uses
replaceable OS adapters, initially macOS Keychain, with Interface-guided enrollment/unlock.

The first synthetic proof uses management and two executor processes on one machine, instance-associated
helpers for authored code, composed operations/requirements, failure injection and Interface inspection.
Then validate native Claude/Codex adapters on macOS, followed by Conduit's own Rust model/tool loop.
Two-Claude/two-Codex accounts and later two-machine support require their own evidence. Pi supplies
research/frontend inspiration; Xerj is an optional Resource backend.

Native execution remains the default; isolated execution is optional or required by an Agent's fixed
contract and cannot be claimed until demonstrated. Managed access enforcement is not universal native
host interception. BYO compatible skills remain native, explicitly activated from permitted Resources.

The [review](REVIEW.md) settles conformance scope, dependent-call authority and cross-object lifecycle.
A later PR will review concrete authoring syntax and conformance fixtures. Remote/multi-user operation, automatic wake, general authored loops, in-place
definition upgrades and a marketplace remain deferred. The [collaborator proposals](proposals/COLLABORATOR_REVIEW.md)
are evaluated against this model; they do not silently add deployment requirements.
