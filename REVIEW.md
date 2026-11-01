# Protocol review closeout

Updated: 2026-10-31. Audited all 11 repository Markdown documents for current contracts,
unresolved decisions, conflicting recommendations and obsolete questions. Source research remains
dated evidence; this audit did not rerun provider investigations or validate a runtime.

**The review is not complete yet.** R1–R3 are accepted; four connected decision bundles remain below.
Their recommendations are not newly accepted requirements. Field names, implementation choices
and future catalog ideas are separated from those rulings. [SPEC.md](SPEC.md) and [DESIGN.md](DESIGN.md)
remain the accepted design; research supplies evidence rather than a second decision queue.
Implementation has not started. Work remains on research/conduits-as-graphs; the earlier baseline
is preserved on main at c013f0a.

## Next discussion

Finish **R4** by defining the purpose of labels and Pool selection before selecting refresh
behavior. The user has accepted unambiguous/explicit routing and recipient-bound delegation;
live label-driven Pool inclusion was not accepted. The earlier reference-plus-selector direction
is reopened because the Pool model now distinguishes available templates from owned instances.

"Catalog" has been shorthand for a Pool's available-template view, not an additional object,
registry service or package manager. Prefer concrete terms: available Resource templates, owned
Resource instances and named local entries. The local registry, installed type-contract registry
and provider-native tool listings have different responsibilities; calling all of them catalogs
does not make them one mechanism.

| Concept | Purpose | Example |
| --- | --- | --- |
| Object identity/reference | Identify the definition or instance actually selected. | A named Resource template reference resolves to a stable object ID. |
| Pool-local entry | Make a template available with configuration and dependency bindings for this Pool. | Euphony's `issues` entry references a reusable Linear template and binds Euphony authentication. |
| Type/capability contract | Define validated behavior and compatibility. | MCP discovery/invocation support comes from the contract, not a `tool=mcp` label. |
| Optional label | Describe an object for organization or discovery. | `team=platform` can help list relevant templates/instances; its target and meaning must be explicit. |
| Label selector | Evaluate metadata to choose a set. Its consequences depend on where it is used. | A search filter lists matches; a Pool-inclusion selector would change what that Pool offers. |

One reusable Linear template can be used by Euphony and personal Pools with different settings.
Putting `domain=euphony` on that template conflates the reusable definition with one use of it.
Pool-local entries already express the relationship. Labels on templates, local entries and
instances must not be silently treated as the same metadata or inherited between them.

**Recommendation, not a ruling:** use explicit named Resource-template entries for Pool inclusion
in v0. Optional labels can assist discovery without adding matching templates to a Pool. Defer
label-driven Pool inclusion until a concrete case needs it. This would revise the earlier
reference-plus-selector direction for Pool composition; established Conduit access-filter
contracts and record classification remain separate and are not silently removed.

The alternative is to retain label-driven Pool inclusion and define it now: which objects are
queried, where labels live, who maintains them, matching/empty-set behavior, refresh and dependency
binding, and effects when labels change. Labels alone still confer no identity, capability,
ownership, permission, provisioning instruction or activation. If metadata drives a behavioral
selector, edits can change its matches; they are more than cosmetic annotations in that use.

## Settled foundation

| Area | Accepted direction; not reopened by this audit |
| --- | --- |
| Model | Interface, Agent, Resource, Pool and Conduit; template and instance forms. Rust-owned runtime, replaceable frontends, Pi as reference. |
| Administration | Unrestricted v0 Interfaces, reusable frontend attachments and shared Interface control. Delegated Agent commands coexist without taking over or gaining Interface authority. Interface scoping is deferred. |
| Construction | R2 selects finite, acyclic typed dependency graphs with branching/loops inside plugins; authored conditions, iteration and dynamic fan-out are deferred. Waits, valid-progress recovery and supported independent parallel steps remain. Instances own ongoing behavior. Direct complete-instance creation and repeated template use are both supported. |
| Configuration | Resolved instance configuration/graph, materialized defaults, no required template-history ledger. R1 requires replacement for fixed-definition changes, including Interface administration. Named controls, confirmed settings separate from outstanding requests, target coordination and revision checks. |
| Ownership/authentication | Exactly one Pool owns each Resource instance; creation/attachment goes through it. Reusable authentication is a Pool-owned Resource. Shared service login does not merge Agent accounts/private state. |
| Retirement | R3 requires explicit Resource retirement before Pool deletion, with bulk actions allowed and partial failures reported. Ownership transfer is deferred. A fully ended Conduit is replaced by a new instance for later participation; retained Resources/data remain separate. |
| Composition/access | Pool template availability does not provision Resources. Agent templates declare starting Conduits; no Pool-provided default Agent connections in v0. Local entries, reusable composition, capability matching and partial readiness remain. Filters preserve complete paths; routing is unambiguous or explicitly selected. Grants follow recipient participation under current restrictions. |
| Protocol | MCP preferred externally; native provider catalogs preserved. Extensible plugin behavior, no exhaustive domain-state enum or mandatory generic invocation wrapper. |
| Continuity/evidence | Stable identity, explicit rebind, current-access delivery, safe retries, explicit restart of stopped Agents, portable checkpoints, retained evidence and honest telemetry gaps. |
| Startup | Seed management objects once, reuse them and explicitly repair inconsistencies. No silent reseeding/purge. Repair mechanics remain implementation work. |

## Remaining decision bundles

R1–R3 are settled; agree or explicitly defer R4–R7. Recommendations become normative only after a
ruling. Draft related schemas/examples together instead of voting on every field separately.

| ID | Contract needing a ruling | Recommended v0 direction | Alternative adds |
| --- | --- | --- | --- |
| R1 — accepted | Can administration edit fixed definitions in place? | Replacement required for graph/control-definition changes, including Interface administration; explicit reuse of backing data is supported where applicable. | In-place definition upgrades deferred. |
| R2 — accepted | What execution semantics do authored graphs guarantee? | Finite, acyclic typed dependency graphs, waits, valid-progress recovery and supported independent parallel steps; branching/loops inside plugins initially. | Authored conditions, iteration and dynamic graph fan-out deferred. |
| R3 — accepted | How do Resources, Pools and ended Conduits retire or get replaced? | Explicit Resource retirement before Pool deletion; bulk action allowed, partial failure retains the Pool. Later participation after flow end uses a new Conduit ID and fresh attachments. | Ownership transfer deferred; no implicit cascade or same-ID reopening of ended Conduits. |
| R4 — partly settled | How are connections selected, delegated and routed? | Accepted: availability does not deploy; no Pool connection defaults; unambiguous/explicit routing with no silent fallback; grants follow recipient participation. Labels and Pool inclusion are reopened; recommend explicit named entries in v0. | Defining label-driven Pool inclusion and its metadata/matching/refresh semantics now. |
| R5 | When do accessible skills, content and catalogs become active? | Explicit compatible preparation; resolve collisions deliberately; source changes do not silently replace loaded instructions/extensions. Refresh at a supported boundary. | Hot reload, implicit precedence or automatic context injection semantics. |
| R6 | What portable configuration/extension profile is shared? | One concrete typed YAML profile; explicit contract/implementation resolution and retained versions; no implicit environment expansion or plugin substitution. | Broader parser features, version-range selection or implicit migration rules. |
| R7 | What communication contract and usable slice does v0 deliver? | Distinguish calls, retained exchanges and observations; minimal local retained exchange, passive attention and explicit context reads. Trackers optional for the experiment. | Automatic wake, remote guarantees, mandatory tracker coupling or shared editable conversation state. |

### R2 — graph execution and recovery

**Accepted:** v0 setup/change graphs have finite, acyclic execution dependencies between typed
plugin operations. Output bindings imply dependencies; explicit ordering covers prerequisites
without data. Branching/loops stay inside plugins initially; authored conditions, iteration and
dynamic graph fan-out are deferred. Waits, valid-progress recovery and independent parallel work
remain supported. This avoids adding branch/iteration recovery semantics to the authoring language
before demonstrated need, while preserving reusable composition.

Authenticate → connect → initialize can be authored as separate steps; the authentication plugin
handles existing login, challenges and polling. A Pool composition step establishes declared
entries through their setup contracts. Finite graph structure does not impose a short wait or
require the network of live Conduits to be acyclic. Plugins still follow current authority and
safe recovery; instances own ongoing services. Output timing, validity and runtime readiness
remain distinct under the earlier accepted contracts.

Schema/example work: render human wait → early output → partial failure → explicit retry/cancel,
including enough attempt/input/output identity to reject stale results and reuse only valid
progress. Cancellation reports remaining effects rather than promising rollback; plugin results
remain extensible. Exact representation is drafting work within the accepted boundary.
[Construction](research/CONSTRUCTION.md#plugin-contracts-and-graph-nodes) and
[establishment](research/ESTABLISHMENT.md#ordering-and-dependency-semantics) supply the cases.

### R3 — ownership and retirement

**Accepted:** keep a Pool instance registered until its Resource instances are explicitly retired:
perform supported stop/disconnect steps and checked unregistration. An explicit bulk action may
retire the Resources and then remove the Pool; partial failure retains the Pool and reports
unfinished work. Ordinary Pool deletion does not implicitly retire its Resources. Ownership
transfer is deferred. This preserves a clear owner during partial cleanup and incomplete setup.

After the last logical consumer/peer detaches and a Conduit ends, later participation opens a new
instance with a new ID and fresh attachments. It may explicitly reuse permitted Pool-owned
Resources/history; old attachments and grants do not silently revive. Transport failure while
participation remains intact recovers the existing instance. Unknown state does not prove end.

Ending a Linear flow leaves the Pool's authentication/service Resources intact. Retiring those
Resources removes their managed participation according to their contracts, not the external
account or stored service data. Backing files, service data and retained evidence follow separate
cleanup/retention rules. Templates remain reusable. These are lifecycle semantics, not an extra
approval step or a requirement to issue a separate command for every component.

Schema/example work: survivor/deletion table for detach, unknown state, stop, replacement and
partial bulk retirement. [Ownership scenarios](research/CONDUITS.md#supporting-resources-and-lifecycle-ownership).

### R4 — selection, delegation and routing

**Accepted:** Pool template availability, selected provisioning, owned instances and consumer
attachments are distinct. Catalog additions do not deploy. Agent templates declare starting
Conduits, including reusable bundles; explicit operations add/remove connections later. V0 omits
Pool-provided default Agent connection sets, avoiding an extra merge/precedence layer. Resource
configuration defaults are unaffected. Selection confers no authority.

**Accepted routing:** without an already-selected route, automatically resolve only a single
eligible route. Several alternatives require explicit choice at attachment/invocation. Record the
path/service identity; do not silently substitute another route/account after failure. Configured
choices can be reused without per-call human confirmation. Same-path reconnection is supported;
deliberate alternate-path use still follows safe outcome/retry rules.

**Accepted delegation lifetime:** access follows recipient logical participation, surviving the
issuing frontend closing or coordinator execution stopping. Withdrawal and current governing
restrictions still apply; recipient detachment ends that attachment's access. Temporary socket
loss is not detachment. Issuer shutdown is not itself grant withdrawal.

**Remaining discussion:** labels, the meaning of available-template views and whether Pool inclusion
needs selectors in v0. No live/snapshot label-selection behavior has been chosen. Existing live
access matching and stable-ID rebinding rules remain; unsupported restrictions leave affected
access unavailable, with native bypass limitations honestly reported.

Closure example: two Pools, overlapping selectors/routes and one revoked consumer; identify the
chosen target, identity, rights and survivors. [Conduit contracts](DESIGN.md#conduits).

### R5 — activation and changing sources

Activation belongs to Agent configuration; access does not imply loading. Recommend compatible
preparation actions, deduplication by Resource/revision/action, and explicit collision resolution.
Required activation blocks readiness; optional failure is visible. Changed sources become available,
but replacing loaded instructions/extensions needs a supported explicit refresh. Dynamic native
tool catalogs remain supported; incompatible schemas cannot be dispatched as the old contract.

A new model within the same account/runtime boundary is not the same as installing a native
extension. Runtime compatibility, model context and native host APIs remain distinct.
Closure example: activate a skill/tool bundle, change its source, introduce a duplicate and refresh.
[Pi activation findings](research/PI_CONNECTORS.md#agent-activation-and-pool-resources).

### R6 — configuration and extension profile

The envelope, forms, IDs, Resource fields, strict schemas and checked authoring writes are settled.
Prepare one coherent proposal covering the remaining semantics:

- Fixed setup/control definitions, establishment inputs, applied settings and status. Define fresh
  creation/export after settings change without exporting credentials or importing observed success.
- YAML values, string/duplicate keys, null/empty/omitted values, units, explicit path/environment
  inputs and optional parser features. No implicit shell evaluation.
- Contract identity/version versus implementation package/release. Resolve collisions explicitly;
  groups are not publisher identities. Recommend a curated local catalog and explicit implementation
  selection with retained versions in v0; public naming governance can wait.
- Unknown versions, unavailable implementations and compatible recovery. An installed-package update
  must not silently reinterpret retained work; dynamic provider catalogs remain a separate contract.

Field spelling, ID encoding and version labels can be drafted autonomously in that profile;
new behavior exposed by examples returns for review. This audit does not select a plugin ABI,
database or schema library. [Configuration evidence](research/CONFIGURATION.md).

### R7 — communication and the usable slice

Communication is central and still needs a selected contract. Research separates invocation/results,
retained exchange and observation; these are behavior classes, not additional object families or
a requirement for separate transports.

Recommend Pool-owned retained content, Conduit-mediated exchange, independent participant progress,
acceptance distinct from processing/completion, bounded catch-up and current-access delivery.
Start with passive attention; automatic model work is a later explicit hook. Two peers and groups
should exercise an extensible mechanism without making rooms the universal primitive or assuming
an editable scratchpad. Addressing, ordering, receipts and offline behavior need one worked scenario.
Tracker coupling is still an open product choice; the latest model establishes no mandatory policy.

Closure example: A asks B for a review; B disconnects before reading; access changes; B returns;
a third observer falls behind. Identify stored, delivered, read and executed work, and what survives
the final connection. [Communication comparison](research/PI_CONNECTORS.md#three-proposed-interaction-families).

## Implementation decisions and deferred work

These do not need individual product rulings unless they change an approved contract:

| Work | Resolution |
| --- | --- |
| Exact fields/names, command spelling, source diagnostics, parser/schema tooling | Draft the R6 profile with valid/invalid examples. |
| Rust crates, plugin process/ABI, storage/journal, transports, caching/client pooling | Engineering design against the contract; surface material deployment/trust changes. |
| Timing constants, budgets, helper paths, UI layout, provider-specific queue/restart options | Type/adapter defaults and targeted experiments, not universal provider micro-rulings. |
| Subscription authentication, private-state separation, provider filters, native MCP support | Empirical gates; accepting the design does not prove compatibility. |
| Broader Resource groups/types/providers | Extend the chosen contracts as use cases arise; no exhaustive catalog vote. |
| Narrower Interfaces, multi-user/remote administration, full agent interception and richer sandbox tiers | Later work; retain the agreed native/default and explicit isolation requirements. |
| Advanced sharing/publication, extraction/expiry, inheritance and public registries | Preserve existing deferred/extension status; research examples are not v0 commitments. |

## Audit coverage and corrections

| Documents | Outcome |
| --- | --- |
| README, SPEC, DESIGN | Current model retained. Direct complete-instance creation was already accepted; removed wording that still questioned it. Remaining semantics map to R1–R7. |
| REVIEW | Replaced the sprawling agenda with this closure list, recommendations and implementation backlog. |
| CONSTRUCTION, CONFIGURATION | Retained evidence; removed redundant questions about graph-backed controls and a second provisioning ledger. R2's graph boundary is accepted; settings/export/version representation maps to R6. |
| ESTABLISHMENT, RESOURCES | R2/R3 execution and ownership boundaries are accepted. Kept lifecycle/type cases; remaining extension choices map to R6, with recovery examples drafted within settled contracts. |
| CONDUITS | Corrected stale claims that control outcome/concurrency policy was unchosen. R3 settles retirement/ended-flow identity; routing/communication map to R4/R7. |
| PI_CONNECTORS, PI_RUNTIME | Retained pinned evidence; removed the obsolete single-controller recommendation. Conduit-facing examples use Agent/Interface terminology. Activation/communication map to R5/R7. |

No research result proves a Conduit implementation. Source-system terms remain where they describe
those systems; candidate types and illustrative fields are not accepted schemas.

## Initial experiment

After these bundles are resolved, prepare a coherent YAML/contract packet and synthetic acceptance
scenario before implementation. Proposed sequence, subject to R7 scope approval:

1. An unrestricted Interface authors/inspects templates and creates a Pool plus independent Agents.
2. The Pool establishes a directory, shared authentication/service access and minimal memory/search.
   Two consumers use differently filtered connections without duplicating service login.
3. Exercise supported/stale changes, missing dependencies, lost responses, explicit retry, detach
   and owner retirement. Distinguish simulated results from actual enforcement.
4. Exercise the chosen communication contract with offline delivery and independent context.
5. Separately verify two Claude and two Codex/OpenAI subscriptions, private-state/refresh separation,
   native compatibility and telemetry before claiming support. This audit authorizes no live login.

Completion means R1–R7 are accepted or explicitly deferred with consequences recorded, the docs
contain no competing normative choices, and the schema/acceptance packet can be drafted without
inventing those semantics. It does not mean every adapter or future extension is designed.
