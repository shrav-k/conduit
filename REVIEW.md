# Protocol review closeout

Updated: 2026-10-31. Audited all 11 repository Markdown documents for current contracts,
unresolved decisions, conflicting recommendations and obsolete questions. Source research remains
dated evidence; this audit did not rerun provider investigations or validate a runtime.

**The review is not complete yet.** R1 and R2 are accepted; five connected decision bundles remain below.
Their recommendations are not newly accepted requirements. Field names, implementation choices
and future catalog ideas are separated from those rulings. [SPEC.md](SPEC.md) and [DESIGN.md](DESIGN.md)
remain the accepted design; research supplies evidence rather than a second decision queue.
Implementation has not started. Work remains on research/conduits-as-graphs; the earlier baseline
is preserved on main at c013f0a.

## Next discussion

Resolve **R3: ownership and retirement** next. It contains two related choices: removal of an
owning Pool, and identity after a Conduit's last participant leaves. Neither changes the existing
separation between registration, managed runtime, backing data and retained evidence.

| Situation | Already accepted | Proposed remaining rule |
| --- | --- | --- |
| Remove a Pool that owns checkout and shared-authentication Resources. | Every Resource needs one owner; deleting registration does not authorize deleting backing files/service data. | Keep the Pool registered until its Resource instances have been explicitly retired/unregistered. Defer ownership transfer and implicit cascading deletion. |
| Retire several Resources together. | Their supported lifecycle and checked deletion contracts still apply. | The Interface may offer one explicitly requested bulk retirement workflow; preserve the owner and report unfinished components if it only partly succeeds. This need not require a separate command for each Resource. |
| The last consumer leaves a Conduit. | End the live flow and release its own connections/leases; preserve Pool-owned Resources and retained evidence. | Later participation opens a new Conduit instance, which may explicitly reuse the same Resources/history. |
| A transport drops while a logical participant remains attached. | Reconnect under the existing connection/recovery contract; do not confuse socket loss with detachment. | This is recovery of the existing instance, outside the ended-Conduit identity choice. |

**Recommendation:** require explicit Resource retirement before deleting the owning Pool, and
use a new Conduit instance after a flow has ended. Resource retirement means its supported
stop/disconnect steps and checked unregistration; backing-data cleanup remains separate. A
managed client shutdown does not imply deleting a remote account or stopping a shared external
service. Missing or uncertain cleanup is reported, not silently treated as completed.

The alternatives are automatic component retirement as part of Pool deletion, or reopening an
ended Conduit under its old ID with rules for retained participation, grants and progress. These
two choices can be decided independently. Ordinary incoming references still do not block
otherwise permitted checked deletion; owning a Resource is a stronger relationship.

## Settled foundation

| Area | Accepted direction; not reopened by this audit |
| --- | --- |
| Model | Interface, Agent, Resource, Pool and Conduit; template and instance forms. Rust-owned runtime, replaceable frontends, Pi as reference. |
| Administration | Unrestricted v0 Interfaces, reusable frontend attachments and shared Interface control. Delegated Agent commands coexist without taking over or gaining Interface authority. Interface scoping is deferred. |
| Construction | R2 selects finite, acyclic typed dependency graphs with branching/loops inside plugins; authored conditions, iteration and dynamic fan-out are deferred. Waits, valid-progress recovery and supported independent parallel steps remain. Instances own ongoing behavior. Direct complete-instance creation and repeated template use are both supported. |
| Configuration | Resolved instance configuration/graph, materialized defaults, no required template-history ledger. R1 requires replacement for fixed-definition changes, including Interface administration. Named controls, confirmed settings separate from outstanding requests, target coordination and revision checks. |
| Ownership/authentication | Exactly one Pool owns each Resource instance; creation/attachment goes through it. Reusable authentication is a Pool-owned Resource. Shared service login does not merge Agent accounts/private state. |
| Composition/access | Pool-local Resource entries, reusable composition step, capability matching, ready Resources usable during partial setup. Composite Conduits retain per-consumer participation. Filters narrow complete authorized paths. |
| Protocol | MCP preferred externally; native provider catalogs preserved. Extensible plugin behavior, no exhaustive domain-state enum or mandatory generic invocation wrapper. |
| Continuity/evidence | Stable identity, explicit rebind, current-access delivery, safe retries, explicit restart of stopped Agents, portable checkpoints, retained evidence and honest telemetry gaps. |
| Startup | Seed management objects once, reuse them and explicitly repair inconsistencies. No silent reseeding/purge. Repair mechanics remain implementation work. |

## Remaining decision bundles

R1 and R2 are settled; agree or explicitly defer R3–R7. Recommendations become normative only after a
ruling. Draft related schemas/examples together instead of voting on every field separately.

| ID | Contract needing a ruling | Recommended v0 direction | Alternative adds |
| --- | --- | --- | --- |
| R1 — accepted | Can administration edit fixed definitions in place? | Replacement required for graph/control-definition changes, including Interface administration; explicit reuse of backing data is supported where applicable. | In-place definition upgrades deferred. |
| R2 — accepted | What execution semantics do authored graphs guarantee? | Finite, acyclic typed dependency graphs, waits, valid-progress recovery and supported independent parallel steps; branching/loops inside plugins initially. | Authored conditions, iteration and dynamic graph fan-out deferred. |
| R3 | How do Resources, Pools and ended Conduits retire or get replaced? | Explicit component retirement before deleting an owning Pool; no implicit cascade or ownership transfer. New participation after flow end opens a new Conduit instance. | Cascade/transfer semantics or a same-ID reopening contract. |
| R4 | How are connections selected, delegated and routed? | Resolve creation selections once; live access matches within current rules. Explicit target/identity, no silent fallback. Delegation follows logical participation until withdrawal/current limits remove it. | Automatic rerouting, grant expiry or dynamic provisioning policies. |
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

Ownership, explicit cleanup and last-consumer flow end are settled; combined deletion behavior is
not. Recommend blocking Pool deletion while it owns Resource registrations. Retire components
explicitly; preserve backing data/evidence. An explicitly requested bulk workflow can coordinate
this without requiring individual commands; partial retirement keeps the Pool and reports what
remains. Active managed instances follow their lifecycle before unregistration. Defer ownership
transfer rather than silently orphaning or adopting Resources.

For example, the final Linear attachment leaving ends its Conduit, not the Pool's shared login.
Deleting the Pool must account for its authentication/service Resources and other consumers.
A later connection can use the same service or retained conversation content with a new Conduit ID.
That new-instance preference remains a proposal, not a settled reopening rule.

Closure example: survivor/deletion table for detach, crash/unknown state, stop, replacement and owner
retirement. [Ownership scenarios](research/CONDUITS.md#supporting-resources-and-lifecycle-ownership).

### R4 — selection, delegation and routing

Recommend resolving template/Pool provisioning selectors into an explicit creation selection;
later matching templates do not automatically provision Resources. Matches within already
authorized Resource access remain live. Stable-ID bindings still require explicit rebinding.

Settle precedence between Agent-declared connections, Pool launch defaults and explicit selections.
Recommend preserving declared dependencies, applying Pool defaults only to an otherwise unspecified
additional selection, and replacing only the explicitly named selection. Avoid a global merge
that silently drops required dependencies.

Retain the selected capability/service identity per attachment. Resolve multiple routes explicitly;
do not choose by discovery order or silently fall back to a different account. Recommend delegation
belonging to logical participation rather than the issuing frontend's lifetime. Revocation/limits
still apply at dispatch and delivery. Unsupported restrictions leave affected access unavailable;
native bypass paths retain their reported enforcement limitations.

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
| ESTABLISHMENT, RESOURCES | R2's execution boundary is accepted. Kept lifecycle/type cases; remaining ownership and extension choices map to R3/R6, with recovery examples drafted within settled contracts. |
| CONDUITS | Corrected stale claims that control outcome/concurrency policy was unchosen. Lifecycle/routing/communication map to R3/R4/R7. |
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
