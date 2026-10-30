# Configuration language and contract review

Updated: 2026-10-30. **Retain YAML; define the language through typed objects and explicit
semantics.** [SPEC.md](SPEC.md) sets the intended outcomes; [DESIGN.md](DESIGN.md) records
accepted contracts. This agenda contains the remaining language and system choices for
Profiles, Sessions, Resources, Pools and Conduits.

The direction, common `apiVersion`/`kind`/`metadata`/`spec` envelope, stable object IDs and
default kind/name references with recorded resolved IDs are accepted. V0 uses one local
registry with names unique per kind. Established bindings from explicit references retain
resolved identity until explicitly rebound. Detailed fields, identifier encoding, schemas,
reference extensions, composition rules and operations remain open. The
[connected YAML](research/five-concepts.proposal.yaml) is a design fixture, not a specification
or implemented format.

## Review method

Work through one coherent part of the language at a time, asking individual questions
within it as needed. For each ruling, show **authored input → effective meaning → allowed
effects → observable result**, including one failure or change. Incorporate settled rules
into DESIGN.md and remove the corresponding open questions here. Do not accumulate another
decision ledger or duplicate the research reports.

Every field contract should answer these questions before its syntax is considered settled:

| Aspect | What we need to specify |
| --- | --- |
| Meaning and type | Which object owns it? Is it a value, reference, selector, constraint or action? What units/domain does it use? |
| Presence and defaults | What do omission, an explicit value, `null`, an empty collection and reset mean? Where does a default come from? |
| Composition | Does it replace, extend, intersect or sequence? Does order matter? What identifies duplicates or conflicts? |
| Change over time | Is it resolved once, pinned to a version, refreshed explicitly or evaluated live? What happens during active work? |
| Validation and effects | What can be checked offline? What requires an adapter or current authority? What does changing it actually do? |
| Evidence and evolution | How is its origin/result explained? What happens if a dependency is missing or its interpretation changes? |

Operation contracts additionally need request/result types, authority, acceptance versus
completion, partial failure, cancellation, retry and unknown-outcome behavior. These are
contract questions even when the eventual operation is exposed through MCP or a native tool.

## Next discussion

Use six connected review rounds. These are discussion groupings, not new domain concepts
or a requirement to decide every row in one answer. Later rounds refine earlier rulings
rather than reopening them.

| Round | Central question | Deliverable |
| --- | --- | --- |
| 1. Objects and references | What is an object, how is it addressed, and which representation may an author change? | Common envelope, identity/reference and registry contract. |
| 2. Resolution and composition | How do authored values become one explainable effective configuration? | Defaults, selectors, collections, input resolution and change rules. |
| 3. Activation and Session changes | What does preparing or changing a Session do, and when? | Profile activation and Session transition contract. |
| 4. Resource and adapter contracts | How does the language describe extensible capabilities without hiding native behavior? | Plugin boundary, compatibility and initial Resource/provider contracts. |
| 5. Conduit contracts | How do authorized capabilities and Session interactions flow through a binding? | Filter/authority semantics and initial interaction contracts. |
| 6. Validation and evolution | How do we explain, check, migrate and test the language before relying on it? | Diagnostics, conformance cases and staged experiment criteria. |

The common envelope, stable identity, default references, v0 naming boundary and binding
identity behavior are settled. Continue round 1 with the authoring/observation boundary and
remaining reference details before choosing merge syntax or activation steps.

### Shared example

Keep one example throughout: two Profiles create four Sessions across two Claude and two
Codex/OpenAI subscription identities. A `frontend-toolkit` Pool and an `application` Pool
overlap on a skill Resource and shared knowledge; the application also exposes a checkout
and tracker MCP service. Each Session binds selected Conduit declarations independently
and applies its Profile's activation instructions to the permitted inventory.

Use the [existing YAML](research/five-concepts.proposal.yaml) as a starting point, changing
it as we rule. Its paths, account aliases, model IDs and API group are synthetic. For
communication examples, add a review request/result and a finding sent to several permitted
Sessions, including one that reconnects later. Their interaction semantics remain open.

### Commitments this review must preserve

- Exactly five core concepts; Resources are independent, Pools collect them, Conduits
  govern access/interaction, and Profile instructions own activation.
- One Profile per Session; explicit behavioral overrides/reconfiguration; live access
  limits; independent private Session state and no implicit account or billing fallback.
- Rust-owned execution with replaceable frontends; native external-agent integrations
  remain different from own-runtime model-provider adapters.
- Bring compatible native skills/plugins; preserve provider-native MCP contracts and
  reuse connection definitions without assuming shared live clients or credentials.
- Accepted memory lifecycle, search/coverage, provenance, sharing, workspace, usage,
  recovery and isolation contracts remain in DESIGN.md. We are defining their expression
  and integration, not reopening them because the YAML changes.
- Communication specifics were reopened. Old room, tracker-link and wake rulings do not
  automatically become the contracts of a Conduit.

## Round 1 — Objects and references

The common envelope and [identity/reference rules](DESIGN.md#object-identity) are settled:
rename preserves the ID; delete/recreate produces a new one; authored references normally
use kind/name and resolution records the selected ID. V0 uses one local registry with names
unique per kind; namespaces and cross-registry references are deferred. Established bindings
from explicit references follow the resolved ID through renames; deletion makes the target
unavailable and replacement requires explicit rebinding. This does not pin content or
permissions, or settle live selectors. **Recommended next direction:** a clear system-owned
state representation and explicit configuration operations. Those details remain proposals.

| Question to resolve | Concrete case and proposed direction |
| --- | --- |
| Which fields are authored and which are system-owned? | Within the accepted envelope, define editable metadata and kind-specific spec fields. Consider `status` for returned observations, but decide whether it is a separate representation or included read-only view. Specify what import/create/update may set. |
| Which identity/reference details remain? | Decide optional direct-ID forms, ID encoding and metadata placement, handling of unresolved references, and which explicit operations create or rebind a relationship. Specify effects on authored references after a rename without changing the accepted binding-identity rule. |
| Which references are live or pinned? | Configuration generation, external source revision, native schema version and retained-record revision are different. What may an author pin, and what must resolution record even when the author uses a readable name? |
| Where is configuration authoritative? | If a user edits YAML and another client edits the registry, which operation reconciles them? Recommend explicit import/update/export with concurrency checks over silent bidirectional file synchronization; decide who owns deletion and rename intent. |
| Who owns actual Conduit bindings? | Recommend Session-owned binding instances with reverse views on declarations, rather than independently editable binding lists on both objects. How does a reusable declaration target `self` without letting a caller assert another Session's identity? |
| How are discovered and external objects represented? | Recommend independently identified Resources with attachment/management metadata; a service Resource can expose native items without one YAML object per file, ticket or tool. Decide whether inline declarations add enough value to justify identity rules. |

**Required walkthrough:** create two Resources, reference one from two Pools, rename it,
then delete/recreate its old name. Show the authored references, resolved identities,
existing binding behavior and diagnostics. Export/import a Session without claiming its
old execution is still running or cloning its private state.

Evidence: [object proposal](research/CONFIGURATION.md#proposed-envelope-and-versions),
[Kubernetes semantics](research/CONFIGURATION.md#kubernetes-an-api-model-with-configuration-semantics).

## Round 2 — Resolution and composition

**Recommended starting point:** explicit configuration roots and a constrained YAML data
surface; independent objects provide reuse. Use different composition rules for defaults,
Pool membership, access limits and activation. Keep a generic template/include/patch language
out of the initial authoring path unless an example demonstrates the need.

| Question to resolve | Concrete case and proposed direction |
| --- | --- |
| Which YAML features and scalar domains are accepted? | Decide string keys, duplicate-key rejection, boolean/number rules, aliases, custom tags and unknown fields. Define durations and byte/text/token budgets with units; do not let `0`, `null` or an empty filter acquire implicit special meaning. |
| Which input locations and ambient values affect resolution? | How are roots selected, directories traversed and relative paths based? Recommend declaring-file/native-package-relative paths, explicit external inputs and no implicit shell/environment interpolation. Decide whether duplicate object definitions fail the batch. |
| How do Profile defaults, Pool defaults and explicit Session choices compose? | A Profile names toolkit access, a selected application Pool has access defaults, and launch supplies its own list. Define all combinations. Explicit choices already replace Pool defaults unless add is requested; Profile-versus-Pool precedence remains unresolved. |
| What do absent, empty, null, add and reset mean? | Recommend omission uses resolved defaults; an explicit empty replacement selects none; add extends deliberately; unsupported null fails. Decide per field rather than treating this example as a universal rule. Reset, if supported, needs an explicit temporal meaning. |
| What does a Pool selector mean? | Recommend refs plus selector matches form a deduplicated union, AND within one selector and OR across selectors. Decide empty-selector behavior, missing-label matching, unresolved members and whether nesting is needed. Labels select; they do not confer authority. |
| What changes live and what requires refresh? | Access rules are already live; behavioral defaults are retained until reconfiguration. Decide Pool membership updates, referenced Resource content/version changes and discovery refresh. Recommend newly matched Resources become available without automatic activation or prompt loading. |
| How is each effective value explained? | Show source object/version and default/override steps. A Resource with two membership paths needs an explanation of both paths, without implying duplicate content or combined permission fields. |

**Required walkthrough:** resolve the shared example with no override, an explicit empty
selection and an explicit add. Change a Resource's label and its source revision separately.
Show membership, available capabilities, prepared resources and model context as different
results. Repeat file discovery in a different order and explain any permitted difference.

Evidence: [composition operators](research/CONFIGURATION.md#composition-needs-several-operators),
[Docker probes](research/CONFIGURATION.md#offline-observations), and
[language comparisons](research/CONFIGURATION.md#alternative-languages-and-contract-systems).

## Round 3 — Activation and Session changes

**Recommended starting point:** structured Profile preparation actions plus agent guidance,
applied only to the permitted inventory. Keep the preparation plan inspectable. State what
configuration apply, preparation, execution and explicit resume each do; exact command
names and whether one user action can deliberately combine stages remain open.

| Question to resolve | Concrete case and proposed direction |
| --- | --- |
| What is the minimum activation vocabulary? | Which actions advertise skills, import native configuration, register tools, load instructions or prepare a workspace? How can reusable loading conventions be supplied through Pools without Profile inheritance or Resource-access bypass? |
| How do defaults and explicit actions interact? | Can an adapter provide conventional import behavior, and how is it disabled/replaced? Recommend recording the resolved actions and source/version so adapter defaults are not invisible behavior. |
| What establishes ordering, duplication and conflicts? | Are actions an ordered list or named steps with dependencies? What if two Pools expose the same skill, or two different packages declare the same command? Recommend deduplication only when identity/version/action are compatible, with explicit conflict resolution. |
| What do required and optional preparation mean? | If a required document is unavailable but an optional MCP service is offline, which parts can prepare and can execution start? Define each outcome, retry scope and evidence rather than reducing all preparation to one ready flag. |
| Which edits affect a running Session, and when? | Specify model/account/runtime switches, activation changes, Resource refresh and Profile deletion. Behavioral reconfiguration must be explicit; access limits remain live. Decide safe transition boundaries and whether an active turn must finish or stop. |
| What is committed before effects occur? | Recommend version-checked local configuration changes and separately recorded external preparation outcomes. Decide batch versus per-object commits, resumable partial preparation, and handling of an installation that succeeded before a later step failed. |
| What belongs in the persisted Session state? | Define Profile origin, overrides, resolved behavior, binding references, activation results, execution boundaries and controller state. Specify create/import/export versus resume/fresh/fork without serializing credentials or duplicating an execution. |

**Required walkthrough:** prepare all four Sessions; fail one optional service and one
required source; retry without repeating completed unsafe actions. Edit the Profile and
then narrow live access. Show which values stay fixed, what becomes unavailable, what
requires explicit reconfiguration and what must stop in isolated mode.

Retain the agreed one-primary-agent, controller transfer, detachment and explicit-resume
contracts. General workflow scheduling and arbitrary durable scripts are not required to
express Profile preparation.

Evidence: [Pi loaders](research/PI_CONNECTORS.md#profile-activation-and-pool-resources),
[activation proposal](research/CONFIGURATION.md#profile-activation-proposal), and
[Durable failure cases](research/PI_RUNTIME.md#fault-cases-worth-deriving-from-durable).

## Round 4 — Resource and adapter contracts

**Recommended starting point:** a small common Resource envelope plus a named, versioned
contract and implementation-specific configuration. Preserve native schemas and operations;
require adapters to state compatibility and observation/enforcement coverage. Contract
identity, adapter version and account binding are separate choices.

| Question to resolve | Concrete case and proposed direction |
| --- | --- |
| What must a plugin declare? | Define configuration schema, operations, runtime requirements, discovery information and errors. Which metadata is authored versus discovered, and what is its source? Decide unknown-plugin handling: inspectable storage may be useful without implying valid activation. |
| What is portable and what stays native? | Distinguish instruction bundles, executable tools/helpers, native runtime hooks and presentation extensions. How does a Profile select a compatible preparation strategy without treating model family as runtime compatibility? |
| How are model/provider/account choices typed? | Define runtime adapter, provider/protocol, model identifier and credential alias. Validate the actual identity for two Claude and two OpenAI subscriptions, refresh ownership and concurrency. Keep own-runtime provider support separate from Claude Code/Codex runtime adapters. |
| What crosses an execution boundary? | Define context projections for a model/account change: selected history, opaque provider state, checkpoints, compaction, source revisions and losses. How are preservation, transformation and unavailable evidence recorded? Account identity must constrain reuse of private/provider-specific state. |
| How are MCP services represented? | A Resource can expose native tools, resource URIs and prompts beneath it. Specify native-operation identity/schema changes, supported protocol versions, nested dispatch and per-binding client/auth state. A shared definition is not proof that one live connection is shareable. |
| What are the first memory and workspace payloads? | Choose a backend and search-limit units/defaults/caps; specify coverage/freshness and revision-checked lifecycle operations. Define workspace base/branch/path and attached/managed status, preserving ownership and explicit safe cleanup. |
| How do external trackers and execution environments fit? | Specify Linear/work references without duplicating its workflow. Select isolation support and destination-matching rules; expose actual enforcement alongside requested mode. Native mode remains default and required isolation cannot silently downgrade. |

**Required walkthrough:** the same Resource is requested from an own-runtime Session and
an external-agent Session. Show the native schema preserved, preparation differences,
unsupported hooks, credential separation and operation attribution. Then switch one
Session's model/account and show exactly which context can be reused.

Important retained findings: Pi's inspected MCP implementation predates the selected July
2026 protocol, and its inspected history conversion does not fence opaque state by account.
These are adapter/conformance obligations; an inventory of supported provider factories
does not establish subscription eligibility or account isolation.

Evidence: [provider/authentication research](research/PI_CONNECTORS.md#provider-coverage-without-importing-pis-runtime),
[context pipeline](research/PI_CONNECTORS.md#session-history-and-model-context),
[MCP boundary](research/PI_CONNECTORS.md#mcp-as-a-resource-and-conduit-protocol), and
[retained memory contracts](DESIGN.md#retained-memory-and-evidence).

## Round 5 — Conduit contracts

**Recommended starting point:** a reusable typed declaration becomes an authenticated
Session binding. A contract describes endpoints, permitted operations, interaction behavior
and outcomes. Filters narrow capabilities; Profile activation remains separate. Transport,
retention and starting a model turn are not implicit consequences of a connection.

| Question to resolve | Concrete case and proposed direction |
| --- | --- |
| Where does binding authority come from? | How is an authorized local setup distinguished from a Session's access request? Define who may establish/change a binding and how effective grants are recorded. YAML text, `session: self` and discovered tool lists do not establish identity or permission. |
| How are filters represented and combined? | Choose inline/reusable forms within the five concepts; specify Resource, operation, native sub-capability, owning-context and supported path constraints. What happens to unsupported predicates? Preserve constrained capability tuples across independent paths. |
| Where do exclusions apply? | Distinguish narrowing one binding from a Profile-wide denial. With read access to `src/**` and write access to `docs/**`, no composition may create write access to `src/**`. Define current-rule evaluation and scoped explanations. |
| What is the binding lifecycle? | Specify opening, unavailable/pending states, closing and revocation separately from declaration deletion, live transport and Session lifecycle. Which declaration edits change existing bindings, and which require explicit rebind? |
| How does the contract describe invocation and exchange? | Define typed input/output, operation IDs, correlation, receipt versus completion, cancellation and retry behavior. Distinguish a Resource invocation, a request for agent work and an observation stream. |

### Interaction research

The previous research suggests capability requests, retained exchanges and observations
as candidate families. They remain proposals. Determine which the shared example actually
needs, with the following questions kept in the same contract discussion:

| Area | Questions to resolve |
| --- | --- |
| Addressing and topology | One-to-one versus fan-out; explicit Session identities versus resolved selections; routing/discovery; membership changes during an exchange. |
| Delivery and retention | Ordering boundary, durable acceptance, acknowledgement, replay/deduplication, offline recipients, retention responsibility and independent recipient position. |
| Attention and control | Passive availability, notifications, queued input and starting a model turn; paused/closed Sessions, optional future wake hooks, cancellation and user control. |
| Context and references | Retained content versus references, live/pinned sources, handoff checkpoints, current recipient access, scoped omissions and independent private context. |
| Backpressure and recovery | Slow observers, bounded buffers, lost updates, snapshot recovery, timeouts and unknown effects. Observation streams need not be complete audit histories. |
| Native protocols and tracking | MCP/A2A/other mappings, local operation and future remote transport, provider-native tracker associations and honest per-adapter guarantees. |

**Required walkthrough:** a recipient stops between receiving a review request and completing
it; decide which stages are durable. A finding fans out while one recipient is offline;
an observer falls behind; access is removed before queued content is delivered. Show
accepted, pending, completed, withheld and unknown states without inventing a new turn
or widening access.

Evidence: [Chord and communication comparison](research/PI_CONNECTORS.md#communication-lessons-for-typed-conduits),
[agentwatch and ecosystem lessons](research/PI_RUNTIME.md#communication-between-sessions-through-conduits).
Further protocol comparison can draw on [Gateway API](https://gateway-api.sigs.k8s.io/docs/concepts/api-overview/),
[MCP](https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture),
[A2A](https://a2a-protocol.org/v1.0.1/specification/), [NATS/JetStream](https://docs.nats.io/concepts/jetstream),
[Zenoh](https://spec.zenoh.io/spec/1.0.0/concepts/entities.html) and
[gRPC](https://grpc.io/docs/what-is-grpc/core-concepts/). None is selected by this agenda.

## Round 6 — Validation and evolution

**Recommended starting point:** define validation stages and their effects, publish schemas
and semantic examples, and make resolution explainable. Share one interpretation across
Rust, API/MCP, CLI and frontend tooling. The exact schema/code generation path is open.

| Question to resolve | Concrete case and proposed direction |
| --- | --- |
| What does an offline check promise? | Recommend parsing, declared-schema checks and static resolution against selected local/cached inputs, without network, package execution, login or model calls. Distinguish missing schema, unresolved reference and unknown live fact from proven invalid data. |
| What can an explicit probe do? | An MCP probe may start a process; authentication may refresh credentials. Require declared effects and current authority. Define what preview observes and what must be rechecked before applying changes or dispatching work. |
| What are the error and status contracts? | Specify stable categories, source locations/field paths, explanation, suggested repair, redaction and evaluated versions/timestamps. Define independent validity, resolution, compatibility, preparation and runtime conditions rather than one universal ready flag. |
| What is the supported schema/version policy? | Choose API group/version and progression, plugin schema dialect, conversion and migration boundaries. Decide treatment of unknown fields/versions and which changes to defaults/composition require migration. Start with one supported authoring version if sufficient. |
| How do edits stay consistent across clients? | Define concurrency tokens, batch validation/commit, stale-update errors and import/export semantics. Decide source preservation, canonicalization and source/resolved digests without mistaking a digest for identity or permission. |
| What evidence is durable and what is a view? | Define operation/activation outcomes, context-source records, transcripts, usage and subagent attribution versus transient frontend updates/traces. Preserve existing unknown/zero and non-overlapping-total rules; specify gaps and recovery. |
| What makes an implementation conform? | Require the same fixture to resolve identically through CLI/API/frontends, and adapters to report unsupported behavior. Identify where live compatibility tests are required beyond schema and fake-provider fixtures. |

The Compose include documentation/implementation mismatch and the Kustomize list tests
show why valid syntax is insufficient. Version and test semantics, especially defaults,
list topology, null/empty values, reference replacement, transitions and side effects.

Evidence: [validation/effect boundaries](research/CONFIGURATION.md#validation-and-effects-are-separate-contracts),
[versioned meaning](research/CONFIGURATION.md#version-meaning-not-just-field-names),
[configuration papers](research/CONFIGURATION.md#research-papers-and-their-limits), and
[Pi observation/recovery](research/PI_CONNECTORS.md#observation-and-durable-execution).

### Contract completion criteria

Before implementing the first schema, the shared example should have agreed object shapes,
reference resolution, precedence, access/activation boundaries and change behavior. The
minimum set of semantic fixtures should cover:

| Case | Question the expected result must answer |
| --- | --- |
| Duplicate definitions or missing plugin schema | Reject, retain unresolved or proceed partially—and with which effects forbidden? |
| Resource in two Pools; rename/delete/recreate | Which identity and versions do existing bindings use? |
| Omitted, empty and additive Conduit selection | What is selected, and why? |
| Disjoint path/operation grants plus exclusions | What exact capability combinations remain usable? |
| New Pool match; changed Resource version | What becomes available, prepared or loaded, and when? |
| Profile edit or account/model switch | Which Session behavior/context stays fixed; which change is explicitly recorded? |
| Partial preparation or uncertain tool effect | What can be retried without duplicating effects or claiming false completion? |
| Offline recipient, slow observer or revoked access | What is retained, delivered, withheld or reported unknown? |
| Old configuration under a new semantic version | What changes through migration, and how is that shown? |

These are proposed fixtures to settle and later implement, not a claim that Conduit passes
them today. The research YAML was checked structurally, not against an accepted Conduit
schema. Research ran 17 Pi history/authentication checks and 13 local Compose/Kustomize
checks; neither establishes live subscription or Conduit runtime compatibility.

## Experiment across the five concepts

After the language rulings, turn the same example into a staged experiment. Resolve the
offline graph first, then test account/provider feasibility early so frontend work does not
conceal a blocking assumption. The [Pi experiment proposal](research/PI_CONNECTORS.md#validation-performed-and-next-experiment)
and [Durable fault cases](research/PI_RUNTIME.md#fault-cases-worth-deriving-from-durable)
supply evidence and proposed tests; implementation and account enrollment have not started.

| Stage | Evidence required |
| --- | --- |
| Language and object graph | Agreed schemas, resolved plans, diagnostics and change fixtures across the five concepts. No implicit grant, activation or model turn from merely parsing configuration. |
| Profiles → Sessions | Four verified independent identities across two Claude and two OpenAI subscriptions; separate private state, refresh/logout/restart isolation and no API-billing fallback. |
| Session execution and frontend | Minimal Rust-owned loop and custom terminal with persisted operation evidence, controller transfer, detachment/reconnect and the accepted lifecycle. A fake provider tests mechanics, not subscription eligibility. |
| Resources → Pools → Conduits | One memory backend, workspace and native skill/MCP sources, with overlapping Pool membership, current filters and observable Profile preparation. Choose actual search budgets and initial source versions. |
| Session ↔ Conduit ↔ Session | The selected interaction contract, including failure, attention, reconnect and current recipient access. No room or transport is preselected. |
| Failure and integration coverage | Stale record/configuration writes, uncertain effects, access removal, partial backends, usage overlap and native/isolated execution. External-agent adapters receive separate compatibility checks. |

Select success/failure criteria, cleanup and release scope before coding. Advanced sharing,
publication, background classification/extraction, automatic expiry, exact cost accounting,
general durable workflows and comprehensive upstream interception remain later capabilities.
Their existing constraints are retained in DESIGN.md. Storage engines, crates, ABI and host
process layout follow the language/runtime contracts rather than defining them.
