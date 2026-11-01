# Protocol review closeout

Updated: 2026-10-31. Audited all 11 repository Markdown documents for current contracts,
unresolved decisions, conflicting recommendations and obsolete questions. Source research remains
dated evidence; this audit did not rerun provider investigations or validate a runtime.

**The review is not complete yet.** R1–R6 are accepted. R7 now accepts composable communication
guarantees and a messages-plus-inspection first slice; its remaining recommendations are not
accepted requirements. Field names, implementation choices
and future catalog ideas are separated from those rulings. [SPEC.md](SPEC.md) and [DESIGN.md](DESIGN.md)
remain the accepted design; research supplies evidence rather than a second decision queue.
Implementation has not started. Work remains on research/conduits-as-graphs; the earlier baseline
is preserved on main at c013f0a.

## Next discussion

**R6 is settled:** one stored configuration map/export, strict YAML and exact retained
Conduit-selected implementations, with replacement instances for implementation upgrades in v0.
Concrete schema fields and tooling remain drafting work within those accepted rules.

**Accepted R7 direction:** shared mechanics with explicit, composable capability guarantees.
Invocation, retention and observation overlap; they are not exclusive message types, new families
or separate wire protocols. Contracts distinguish what receipt, retention, delivery and execution
evidence establish without an exhaustive domain-state enum or extra manual bookkeeping. Durable
content remains Pool-owned. MCP is the preferred supported surface, not a guarantee of retention
or Agent scheduling. The [protocol investigation](research/CONDUITS.md#messaging-and-networking-investigation)
supplies the examples and evidence.

**Accepted first slice:** retained Agent messages and state inspection. Structured work submission
with job acceptance, progress, results and cancellation is deferred. An Agent can ask another to
review through a message; this does not itself create a structured job. Ordinary tools and existing
supported instance controls remain available. No previous room policy is automatically restored.

Three closeout choices remain, grouped by user-visible behavior:

| Choice | Proposed v0 direction, not yet accepted | Example that resolves it |
| --- | --- | --- |
| Message addressing, history and recovery | Explicit Agent recipients, individually or as an explicit group; append-only retained exchanges with independent consumption progress and bounded catch-up. Apply existing safe-retry/current-access rules. | A posts for B and C; B is offline, C reads, a response is lost and B's access changes. Define what each participant can later retrieve and what an acknowledgment proves. |
| Attention | Unread availability on the next normal turn, with explicit reads; no automatic model turn from posting or replay. Preserve an optional future wake hook. | B is idle when A posts. Separate receipt from whether B runs, and avoid feedback loops from acknowledgments/status. |
| Experiment boundary | Local two-Agent exchange plus an Interface observer; tracker connection optional for testing, remote transport/federation later. Keep the separate two-Claude/two-Codex compatibility gate. | Exercise offline catch-up, a duplicate retry, permission narrowing and observer reconnection before live provider tests. State any longer-term tracker requirement explicitly. |

The choices should close with a small end-to-end example, not a vote on every message field.
Exact IDs, cursors, wire framing, status field names, numeric limits, broker/storage choices and
plugin implementation remain drafting/engineering work unless they expose new protocol behavior.
Current Pool ownership, retention-until-purge, access checks, safe retries and new-Conduit-after-end
rules already apply; no extra ruling is needed to restate them.

## Settled foundation

| Area | Accepted direction; not reopened by this audit |
| --- | --- |
| Model | Interface, Agent, Resource, Pool and Conduit; template and instance forms. Rust-owned runtime, replaceable frontends, Pi as reference. |
| Administration | Unrestricted v0 Interfaces, reusable frontend attachments and shared Interface control. Delegated Agent commands coexist without taking over or gaining Interface authority. Interface scoping is deferred. |
| Construction | R2 selects finite, acyclic typed dependency graphs with branching/loops inside plugins; authored conditions, iteration and dynamic fan-out are deferred. Waits, valid-progress recovery and supported independent parallel steps remain. Instances own ongoing behavior. Direct complete-instance creation and repeated template use are both supported. |
| Configuration | One `spec.config` map: initial values then confirmed settings; export stored values for fresh creation. Pending/uncertain state stays separate. Resolved graph/materialized defaults, no required template history. Fixed-definition changes require replacement; named controls, target coordination and revision checks govern supported changes. |
| YAML interpretation | YAML 1.2 Core scalar resolution, string keys, finite supported numbers and strict typed validation. Explicit omission/null/empty semantics. No anchors/aliases, merge keys, custom tags or implicit shell/environment expansion in v0. |
| Implementation selection | Explicit selection or configured default, resolved to exact release/artifact before execution and retained per instance. No silent substitution; upgrades require replacement instances in v0. External providers/native binaries retain separate boundaries. |
| Organization | No generic registry-object labels or label-based selectors in v0. Pools use explicit named entries; discovery uses defined fields and relationships. Knowledge-record classification and typed access filters remain separate. |
| Ownership/authentication | Exactly one Pool owns each Resource instance; creation/attachment goes through it. Reusable authentication is a Pool-owned Resource. Shared service login does not merge Agent accounts/private state. |
| Retirement | R3 requires explicit Resource retirement before Pool deletion, with bulk actions allowed and partial failures reported. Ownership transfer is deferred. A fully ended Conduit is replaced by a new instance for later participation; retained Resources/data remain separate. |
| Composition/access | Explicit provision-from-entry or use-existing-instance intent; availability alone does not deploy. Agent templates declare starting Conduits; no Pool connection defaults. Local entries, reusable composition, capability matching and partial readiness remain. Filters preserve complete paths; routing is unambiguous/explicit. Grants follow recipient participation. |
| Activation | Declared compatible preparation; required/optional failures and explicit collision resolution. Refresh of loaded content/extensions and exposed tool definitions is explicit in v0; automatic policies are deferred. Known stale/incompatible calls are withheld, with detection limits reported. |
| Protocol | MCP preferred externally; native provider catalogs preserved. Extensible plugin behavior, no exhaustive domain-state enum or mandatory generic invocation wrapper. |
| Communication foundation | Composable capability guarantees; retained messages and state inspection first. Structured work submission deferred. No required room abstraction or automatic context merging. |
| Continuity/evidence | Stable identity, explicit rebind, current-access delivery, safe retries, explicit restart of stopped Agents, portable checkpoints, retained evidence and honest telemetry gaps. |
| Startup | Seed management objects once, reuse them and explicitly repair inconsistencies. No silent reseeding/purge. Repair mechanics remain implementation work. |

## Remaining decision bundles

R1–R6 are settled; agree or explicitly defer R7. Recommendations become normative only after a
ruling. Draft related schemas/examples together instead of voting on every field separately.

| ID | Contract needing a ruling | Recommended v0 direction | Alternative adds |
| --- | --- | --- | --- |
| R1 — accepted | Can administration edit fixed definitions in place? | Replacement required for graph/control-definition changes, including Interface administration; explicit reuse of backing data is supported where applicable. | In-place definition upgrades deferred. |
| R2 — accepted | What execution semantics do authored graphs guarantee? | Finite, acyclic typed dependency graphs, waits, valid-progress recovery and supported independent parallel steps; branching/loops inside plugins initially. | Authored conditions, iteration and dynamic graph fan-out deferred. |
| R3 — accepted | How do Resources, Pools and ended Conduits retire or get replaced? | Explicit Resource retirement before Pool deletion; bulk action allowed, partial failure retains the Pool. Later participation after flow end uses a new Conduit ID and fresh attachments. | Ownership transfer deferred; no implicit cascade or same-ID reopening of ended Conduits. |
| R4 — accepted | How are connections selected, delegated and routed? | Explicit Pool entries and provision-or-use-existing intent; no generic labels/selectors, automatic deployment or Pool connection defaults. Unambiguous/explicit routing and recipient-bound grants. | No implicit reuse/provision selection; richer reuse policies deferred. |
| R5 — accepted | When do accessible skills, content and catalogs become active? | Declared compatible preparation, deliberate collision resolution and explicit refresh. Withhold affected known stale/incompatible calls; report detection limits. | Automatic reload/first-use refresh deferred; no implicit precedence or context replacement. |
| R6 — accepted | What portable configuration/extension profile is shared? | One `spec.config` map/export, strict YAML, exact retained implementation selection and replacement for upgrades. | In-place code upgrades, implicit substitution and YAML convenience features deferred. |
| R7 — partial | How do retained messages address participants, recover and attract attention; what bounds the experiment? | Composable guarantees and messages-plus-inspection are accepted. Recommend independent recipient progress, passive attention and local testing without a required tracker. | Structured work submission is deferred; wake policy, exchange details and tracker/remote boundaries still need closeout. |

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

**Accepted organization:** generic object labels and label-based selectors are excluded from v0.
Explicit Pool entries identify available templates; discovery uses defined fields/relationships.
Knowledge-record classification remains; existing typed access matching and stable-ID rebinding
rules still apply. Unsupported restrictions leave affected access unavailable, with native bypass
limitations honestly reported.

**Accepted provisioning intent:** templates/graphs explicitly distinguish provisioning from a named
Pool entry from using a selected existing Resource. Both resolve to a concrete instance binding;
matching a template/name alone does not choose reuse or creation. Two tasks can provision separate
worktrees while several Agents use the existing Linear/authentication Resources. Provisioning with
attach behavior creates a new registration around existing backing; using an existing Resource
preserves its identity and owner. No separate operation manifest or per-start question is required.
Capability checks, authority and safe recovery still apply. Exact fields/result wiring remain R6 work.

Closure example: two Pools, shared authentication, two worktrees and one revoked consumer;
identify the creation intent, bound instance, route, rights and survivors.
[Conduit contracts](DESIGN.md#conduits).

### R5 — activation and changing sources

**Accepted:** activation belongs to Agent configuration; access does not imply loading. Apply declared
compatible preparation, avoid duplicate equivalent loading and resolve conflicting exposed names
explicitly. Do not erase differing parameters, routes or rights. Required activation failure blocks
readiness; optional failure is reported. A template can establish these rules without repeated human
confirmation. Model compatibility alone does not supply a native extension host.

Updated source content remains available for reads, but replacing loaded instructions/extensions
uses a supported explicit refresh at a safe runtime boundary. Authorized Agents can request it.
Provider-native catalogs are still discovered dynamically; initial discovery and explicit refresh
are sufficient for v0. Automatic first-use/background reload policies are deferred. Current access
restrictions, credential refresh and transport recovery retain their separate behavior.

Distinguish a new Linear issue from a changed tool schema: the first changes data returned by an
existing operation, while the second may invalidate the exposed contract. At dispatch, use current
authority and supported change/compatibility evidence. Known affected stale/incompatible definitions,
or detected changes with unresolved compatibility, withhold the call pending explicit refresh or
reconciliation. Refresh supplies definitions for the caller to reconsider its request; it does not
silently adapt arguments or replay writes. Unaffected usable operations can continue.

This is not a universal per-call network fetch or guarantee of pre-call change detection. A provider
error may be the first evidence; freshness/coverage limits must be visible. Future automatic refresh
procedures must define invalidation, supported activation boundaries and safe retry. Exact mechanism
remains adapter work, with no additional v0 ruling required.

Closure example: load a guide/tool bundle, change the guide and a tool schema, detect an affected
call, explicitly refresh, reconsider the request, and separately handle a provider rejection without
advance notice. Use the current grants throughout.
[Pi activation findings](research/PI_CONNECTORS.md#agent-activation-and-pool-resources).

### R6 — configuration and extension profile

The envelope, forms, IDs, Resource fields, strict schemas and checked authoring writes are settled.
**Accepted representation/export:** the [worked R6 configuration example](research/CONFIGURATION.md#r6-worked-example-configuration-through-creation-change-and-export)
uses one `spec.config` map for initial and then confirmed settings. Pending/uncertain requests stay
separate. Fixed definitions remain fixed; later starts read current settings under the type's contract.
Export carries stored settings for fresh creation, reports uncertainty separately and carries no
credentials, grants, readiness or progress. Resource creation still requires a Pool and explicit
backing intent. The template remains unchanged; no parallel creation/applied maps are required.

**Accepted YAML profile:** YAML 1.2 Core scalar interpretation, string keys and finite supported
numbers with typed validation, duplicate/unknown-key rejection and schema-declared open maps.
Omission uses declared defaults/optionality; null is allowed only where declared and is not reset;
empty collections stay explicit. Comments and multiline strings remain. Anchors/aliases, merge-key
behavior, custom tags and implicit shell/environment expansion are excluded from v0. Use typed
templates/references/graphs and explicit inputs. Native Resource contents keep their own formats.

**Accepted implementation rule:** select explicitly or through a configured default, resolve the
exact implementation release/artifact before execution and retain it for the instance. Report
ambiguity or unavailable code without substitution; known schemas still permit valid registration
with a readiness gap. Conflicting contract definitions cannot silently overwrite each other.
Implementation upgrades require replacement instances in v0, with explicit backing reuse where
supported. This covers Conduit-selected code, not remote providers/catalogs or external native-agent
binaries. Public naming governance and package/ABI mechanics remain future/engineering work.

Field spelling, ID encoding and version labels can be drafted autonomously in that profile;
new behavior exposed by examples returns for review. This audit does not select a plugin ABI,
database or schema library. [Configuration evidence](research/CONFIGURATION.md).

### R7 — communication and the usable slice

The composable-capability direction and retained-messages-plus-inspection first slice are accepted.
The three choices under [Next discussion](#next-discussion) are the remaining queue. Structured work
submission is a later capability, so this experiment need not implement a job scheduler or task
lifecycle protocol. Existing named controls and operation outcomes remain in scope.

Closure example: A posts a review request in prose; B disconnects before reading; access changes;
B returns; an Interface observer falls behind. Identify stored/delivered content, per-recipient
progress, evidence gaps and what survives the final connection. An Agent can independently choose
to review and post findings; a message receipt is not job acceptance. The
[protocol investigation](research/CONDUITS.md#messaging-and-networking-investigation) and
[Pi/Chord comparison](research/PI_CONNECTORS.md#three-proposed-interaction-families) supply evidence,
not separate ruling queues. Broker/wire selection follows the required semantics.

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
| README, SPEC, DESIGN | Current model retained. R1–R6 are settled; remaining product choices are under R7. Concrete schemas and implementation details remain drafting work. |
| REVIEW | Replaced the sprawling agenda with this closure list, recommendations and implementation backlog. |
| CONSTRUCTION, CONFIGURATION | Retained evidence; removed redundant questions about graph-backed controls and a second provisioning ledger. R2 graph execution and R6 configuration/YAML/implementation rules are accepted; draft concrete schemas within them. |
| ESTABLISHMENT, RESOURCES | R2/R3 execution/ownership and R6 implementation boundaries are accepted. Kept lifecycle/type cases for schema drafting and recovery verification. |
| CONDUITS | R3 settles retirement/ended-flow identity and R4 routing. Communication remains R7; detailed fields are drafting work, not another decision queue. |
| PI_CONNECTORS, PI_RUNTIME | Retained pinned evidence; removed the obsolete single-controller recommendation. Conduit-facing examples use Agent/Interface terminology. R5 settles explicit activation/refresh; communication remains R7. |

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
