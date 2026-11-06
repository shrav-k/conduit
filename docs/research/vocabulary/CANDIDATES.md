# Candidate vocabulary

Status: proposals, 2026-11-04. **Nothing in this table is an accepted rename.** The comparisons build
on [primary-source precedents](PRECEDENTS.md); definitions remain in the linked protocol topics.
Candidate words are alternatives to compare, not extra concepts to introduce simultaneously.

## Authoring and extension boundaries

| Topic | Candidates and tradeoffs | Initial recommendation for discussion |
| --- | --- | --- |
| [V01](../../protocol/MODEL.md#v01): template / instance / form | **Template/instance** is general and familiar. **Blueprint/instance** emphasizes construction but says less about runtime and controls. **Image/container** incorrectly suggests a filesystem artifact and isolation. **Definition/instance** works, but “definition” already includes contracts and graphs. | Retain template/instance; explain that an instance can be stopped and still exist. Keep `form` provisional until envelope vocabulary is reviewed. |
| [V02](../../protocol/MODEL.md#v02): contract / schema | **Contract** includes behavior, effects and recovery. **Schema** precisely describes data constraints. **Specification** is useful for the protocol as a whole, but broad for every reusable contribution. | Retain both with a strict distinction: a contract can contain schemas; validating a schema does not prove behavioral conformance. |
| [V03](../../protocol/MODEL.md#v03): type contract | **Type contract** clearly specializes a family. **Resource type** works only for one family. **Class** suggests language inheritance/constructors. **Kind** collides with the five family discriminator. | Use family-qualified type where helpful, such as Resource type or Agent type, retaining type contract for the shared concept. Group/name/version spelling remains later work. |
| [V04](../../protocol/MODEL.md#v04): capability | **Capability** expresses offered behavior across implementations. **Service contract** implies a service boundary. **Interface** collides with the fixed family. In security literature “capability” can also mean authority conveyed by a reference. | Retain capability or capability contract, explicitly distinguishing offered behavior from permission to invoke it. An advertised capability is not a grant. |
| [V05](../../protocol/MODEL.md#v05): plugin / implementation binding | **Plugin** is approachable but often implies loaded code. **Implementation** fits code, MCP bindings and graph compositions, but is broad. **Extension** can include templates/skills as well as code. **Adapter** implies translation and cannot cover all implementations. | Strongest open naming decision: use implementation for the precise selected behavior; consider plugin as the author-facing term only if its code-optional meaning stays clear. Do not make both separate primitives. |
| [V06](../../protocol/MODEL.md#v06): package / contribution / bundle | **Package** is a distribution unit. **Contribution** describes one provided definition without implying a process. **Bundle** can mean packaging or composition and is less precise. **Module** is overloaded by programming languages. | Prefer package and published contribution; qualify “implementation bundle” if retained. A package may contain only definitions/templates. |
| [V07](../../protocol/MODEL.md#v07): entry document / export / re-export | **Manifest** is familiar for package metadata but sometimes means an object declaration. **Entry document** is exact but verbose. **Export/re-export** explains the public namespace; **component** implies more structure than any exported definition needs. | Discuss package manifest as the entry document; retain export/re-export. Keep export identity separate from file paths and implementation identities. |
| [V08](../../protocol/MODEL.md#v08): dependency / import / alias / selection | **Dependency** can mean package resolution, graph ordering or live access. **Import** emphasizes a definition boundary. **Alias** is a local name, not a distinct copy. **Selection** describes resolution, not ongoing connectivity. | Keep conventional terms with qualifiers: package dependency, imported contract, local alias, implementation selection. Do not call package imports Conduits. |

The first discussion should compare two coherent descriptions:

- **Plugin-centered:** “A package publishes plugins and templates. A plugin satisfies contracts and
  can use code, an MCP binding or a graph.” Accessible, but “plugin” still needs a precise extent.
- **Contract-centered:** “A package exports contracts, implementations and templates. Operations can
  be implemented by code, MCP services or graphs.” More direct about the public extension boundary,
  but “implementation” is less distinctive in introductory material.

Both express the accepted model. Neither should turn an executable export, an entire package, and a
selected per-instance binding into one indistinguishable thing. The choice can be tested without
changing package scope or requiring a primary implementation.

## Execution and value flow

| Topic | Candidates and tradeoffs | Initial recommendation for discussion |
| --- | --- | --- |
| [V09](../../protocol/MODEL.md#v09): operation / invocation | **Operation** names callable behavior; **invocation** names a particular request. **Action** is shorter but vague about request/result boundaries. **Command** can suggest CLI input or fire-and-forget. **Tool** is a particular exposure to an Agent/MCP client. | Retain operation/invocation. An operation may be exposed as a tool; not every tool call is a new graph definition. |
| [V10](../../protocol/MODEL.md#v10): graph / node / step / prerequisite | **Graph** describes structure but not purpose. **Procedure** describes finite instructions without promising a workflow platform. **Workflow** is familiar but carries scheduling/recovery expectations. **Pipeline** often suggests a chain or ongoing data processing. **Recipe** is friendly but weak for controls/recovery. | Compare “graph” with “procedure” in complete authoring sentences. Prefer step over node in prose. Keep DAG structure explicit and avoid “flow” for setup while it also describes Conduit activity. |
| [V11](../../protocol/LIFECYCLE.md#v11): binding / reference / assembly | **Value binding** qualifies an overloaded term. **Expression** suggests calculations we defer to operations. **Wire** works visually but does not describe literal/object/list construction. **Value source** misses assembly. | Value binding is the clearest umbrella. Use literal, reference, object assembly and list assembly for the existing four forms; preserve JSON Pointer's external meaning. |
| [V12](../../protocol/LIFECYCLE.md#v12): input / output / port / public result | **Input/output** identifies direction. **Port** could mean a graph interface or socket number. **Result** may mean a successful output, complete outcome, or public composite response. | Prefer named input/output over introducing “port” in introductory prose. Use public result for deliberate exposure and operation outcome for what happened; do not imply successful data describes all effects. |
| [V13](../../protocol/LIFECYCLE.md#v13): runtime / driver / runtime target | **Instance runtime** qualifies managed ongoing behavior. **Managed component** avoids collision with the overall engine but is generic. **Service** assumes service-like lifetime. **Process** wrongly fixes physical placement. **Driver** describes the implementation role but collides with I/O/device drivers. | Compare instance runtime and managed runtime component. Keep the declaration, its selected driver, a typed target reference and a usable live result distinct. No new object family or process-per-component rule. |
| [V14](../../protocol/LIFECYCLE.md#v14): control / handler | **Control** captures an exposed supported change. **Setting** is data, not the action applying it. **Command** covers too much. **Mutation** suggests a direct data write and misses runtime effects. | Retain typed control backed by an operation/graph handler. Avoid “controller” unless the implementation actually reconciles continuously. |
| [V26](../../protocol/LIFECYCLE.md#v26): execution / attempt / retry / recovery | **Run** is approachable but can mean an entire Agent turn, graph or process. **Execution** still needs scope. **Attempt** distinguishes a producing effort from the logical invocation. **Replay** can imply deterministic recomputation or repeated effects. | Qualify graph execution and operation attempt; retain retry/recovery separately. Do not label uncertain-write reconciliation “rerun” or imply graph recovery always repeats completed work. |
| [V27](../../protocol/LIFECYCLE.md#v27): invocation context / metadata | **Context** is already overloaded by LLM inputs, memory and tracing. **Invocation metadata** is clear for declared values. **Request context** can include trusted state not exposed to the graph. | Qualify both: trusted invocation context versus declared invocation metadata. Caller-provided metadata must not sound like an authority credential. |
| [V28](../../protocol/LIFECYCLE.md#v28): managed reference / handle / descriptor | **Handle** suggests indirect access with lifetime; **descriptor** suggests inspectable data; **reference** can mean an authored graph expression. **Token** misleadingly implies bearer authority. | Use qualified managed reference or handle for runtime values and descriptor for descriptive data where its contract fits. Keep graph reference syntax separate from the value it selects. |

“A graph implements an operation” is a particularly useful sentence. It explains reusable composition
without saying every composition is a new process, plugin package or managed object. “A setup graph
starts a declared instance runtime” similarly keeps establishment separate from ongoing supervision.

## Relationships, restrictions and authority

| Topic | Candidates and tradeoffs | Initial recommendation for discussion |
| --- | --- | --- |
| [V17](../../protocol/CONDUITS.md#v17): requirement / constituent / obligation | **Requirement** covers positive duties and restrictions but can mean a dependency in TOSCA. **Constraint** suggests narrowing and may miss a duty to authenticate or produce evidence. **Guarantee** may overstate what is verified. **Rule** is broad. **Guard** sounds like a single gate. | Retain requirement provisionally; compare obligation for a duty within it. Always distinguish an authored required condition from evidence that an implementation fulfills it. Composition is not an execution graph. |
| [V18](../../protocol/CONDUITS.md#v18): filter | **Filter** fits narrowing flow. **Restriction** is clearer for access than data transformation. **Valve** suggests one on/off or rate-control device. **Transform** may change data and is not necessarily a restriction. | Filter is already a strong thematic and technical match when explicitly narrowing. Do not use it as an umbrella for all requirements or all plugin transformations. |
| [V19](../../protocol/MODEL.md#v19): policy / governing requirement binding | **Policy** communicates authoritative applicability but can imply another language or an object family. **Governing rule/binding** is more specific but cumbersome. **Requirement assignment** emphasizes where a reusable requirement applies. | Keep policy as explanatory language only if its representation is clearly an authoritative application of shared requirements. Review whether requirement binding is the precise authoring noun. Do not add another family. |
| [V20](../../protocol/MODEL.md#v20): grant / issuance / delegation / revocation | **Grant** states authority. **Permission** can mean one allowed action or the whole grant. **Lease** implies expiry/renewal. **Token** implies a credential. **Delegation** can imply a live parent-chain dependency that we rejected. | Retain grant and grant issuance/revocation; qualify delegation as an authorized issuance action under our rules. No implied dependency on the issuer staying active. |
| [V21](../../protocol/CONDUITS.md#v21): participation / attachment / open / join | **Participation** expresses logical membership and can survive socket loss. **Attachment** suggests a lifecycle relation but is also used for frontends/filesystems. **Connection** collides with both transport and the whole Conduit. **Subscription** implies a specific delivery model. | Prefer participation for the logical record; attach/detach or join/leave can be action verbs if used consistently. Preserve open-new versus join-existing intent. Review frontend attachment separately. |
| [V22](../../protocol/CONDUITS.md#v22): route / access path | **Route** is concise and thematic. **Access path** emphasizes authority constraints. **Channel** may imply a messaging stream or new independently identified thing. | Retain route/access path as one concept with a chosen primary term. A route is complete; pieces of permissions from alternative routes do not combine. |
| [V23](../../protocol/CONDUITS.md#v23): endpoint / consumer / provider / peer | **Endpoint** is neutral but often interpreted as an address. **Consumer/provider** describe roles but provider also names a model integration. **Source/sink** fit directed data only. **Peer** fits symmetric participation. | Use logical endpoint, consumer endpoint, provider endpoint and peer where appropriate. Qualify model provider. An endpoint need not be a process, machine or HTTP URL. |

This is the most important place to resist synonym compression. The answers to “what is offered?”,
“what access was granted?”, “which restrictions apply?” and “which implementation enforces them?”
must remain separately inspectable even if one package implements all four.

## Configuration, lifecycle and publication

| Topic | Candidates and tradeoffs | Initial recommendation for discussion |
| --- | --- | --- |
| [V15](../../protocol/CONFIGURATION.md#v15): spec / config / status | **Definition** emphasizes immutable structure; **configuration** covers supported settings; **status** reports observations. **Desired/actual** would imply a controller model. **State** is too broad alone. | Use fixed definition, stored configuration and observed status in explanation. Review envelope spellings later; pending/uncertain changes do not become confirmed configuration merely by being requested. |
| [V16](../../protocol/CONFIGURATION.md#v16): name / ID / revision / binding | **Name** is readable lookup; **ID** is stable identity; **revision** is a concurrency baseline. **Version** already selects contract interpretation. **Binding** means resolved identity here. | Retain those distinctions and qualify configuration revision / resolved object binding. No name implies automatic rebinding or aliases. |
| [V24](../../protocol/LIFECYCLE.md#v24): setup / establish / prepare / provision / attach / start | **Establish** is a broad umbrella. **Provision** fits creating backing resources. **Prepare** is readiness work. **Start** initiates declared ongoing behavior. **Attach** can adopt an existing target without creating it. **Deploy** blurs all of them. | Keep distinct effect verbs; make each operation's contract state its effect. Prefer setup for the overall procedure. Do not invent a universal deployment verb that silently combines registration and effects. |
| [V25](../../protocol/LIFECYCLE.md#v25): cancel / stop / retire / unregister / purge | **Cancel** requests interruption; **stop** addresses ongoing runtime; **retire** coordinates shutdown obligations; **unregister** removes a registration; **purge** explicitly removes retained data. **Delete** is ambiguous about scope. | Retain precise verbs, qualify any delete operation by its target. An ended Conduit and deleted history are different events. |
| [V32](../../protocol/CONFIGURATION.md#v32): contract version / package release / artifact digest / capture | **Version** can mean interpretation, publication or concurrency. **Release** should mean an explicitly preserved publication. **Digest** identifies content. **Snapshot** collides with observation but fits a qualified retained package capture. | Retain qualified terms. Edit-mode capture is not a release or a promise that edited code remains available for new instantiation. Exact hash and publication syntax remain separate. |

## Observation, hosting and domain language

| Topic | Candidates and tradeoffs | Initial recommendation for discussion |
| --- | --- | --- |
| [V29](../../protocol/MODEL.md#v29): observation / evidence / source / snapshot / feed / cursor | **Observation** is a report; **evidence** supports a conclusion; **telemetry** often suggests traces/metrics/logs; **event** may mean a durable fact or a transient notification. **Feed** suggests incremental delivery without necessarily promising a universal stream. | Retain qualified observation terms and standard telemetry names for corresponding signals. Keep authoritative outcomes, current views and exported telemetry distinguishable, including gaps and stale coverage. |
| [V30](../../protocol/MODEL.md#v30): management service / executor / host / helper | **Execution service** identifies responsibility; **executor** can also mean an async runtime. **Host** may be a machine, process or MCP coordinator. **Worker** suggests scheduling, not ownership. **Sidecar** fixes placement. | Use management service and execution service in architecture; instance-associated helper for supervised authored code. Reserve host/client/server for a qualified protocol or deployment context. |
| [V31](../../protocol/MODEL.md#v31): frontend / attachment / root / local owner | **Frontend** is presentation; **client** is a protocol role; **console** suggests an interactive UI; **root** can mean highest authority or graph entry. | Keep frontend distinct from Interface. Say root Interface and local owner explicitly. A CLI command is not automatically another Interface instance. |
| [V33](../../protocol/RESOURCES.md#v33): memory / context / history / record / collection | **History** describes retained events; **conversation** describes a communication domain; **memory** describes reusable knowledge; **context** describes what an operation/model receives. **Room** is one application metaphor. | Use capability-qualified domain names, not a universal Context object. Transcript entry, memory record and operation evidence need not share one lifecycle. |
| [V34](../../protocol/POOLS.md#v34): owner / backing data / Resource entry | **Owner** has a clear lifecycle meaning; **member** merely suggests grouping. **Resource-template entry** is more explicit than catalog or plan. **Inventory** can mean actual instances rather than provisioning options. | Prefer Pool owner and named Resource-template entry. Keep offered templates, deployed Resource instances and backing data distinct. No labels or automatic deployment are reintroduced. |
| [V35](../../protocol/POOLS.md#v35): authentication / account / credential / custody | **Authentication** verifies a service identity; **credential** is access material; **custody** names where it is held; **authorization** governs permitted action. **Identity** also names registry objects. | Qualify service identity, model account, caller identity and credential custody. Shared provider credentials do not make all callers the same Agent or grant them unrestricted access. |
| [V36](../../protocol/AGENT.md#v36): activation / preparation / exposure | **Activation** can mean loading skills/tools, starting an actor, or enabling a feature. **Exposure** presents a permitted capability. **Load** is concrete for instructions but not every resource. | Describe the specific effect: load instructions, expose tools, prepare filesystem access. Retain an umbrella only if it adds useful shared semantics. Authorization and availability remain separate. |
| [V37](../../protocol/AGENT.md#v37): model / provider domain / subscription / account | **Model** is a selectable model identity; **provider** can be a vendor, implementation or endpoint role; **account/subscription** governs access and billing in its native system. | Preserve native distinctions with qualifications rather than a themed noun. An allowed model switch is not an account replacement. |

## Theme: where it helps and where it costs

The following is deliberately broader than the shortlist. These are naming probes, not proposed
new abstractions or extra review obligations.

| Candidate | Natural use | Risk / recommendation |
| --- | --- | --- |
| Flow | Ongoing data or interaction through a Conduit. | Good explanatory word; avoid also naming the finite setup graph “Flow.” It does not promise an ordered stream. |
| Route | Selected path for access. | Strong fit, provided constraints and identity remain attached to the complete route. |
| Endpoint | Participant's role at a connection boundary. | Strong fit; say logical endpoint when an address/process interpretation would mislead. |
| Filter | Narrowing permitted operations/data. | Strong fit for restrictions, not every requirement or transformation. |
| Channel | A particular messaging capability. | Familiar but implies communication-specific behavior; not a replacement for all Conduits. |
| Stream | Ordered incremental data with a declared contract. | Useful only where ordering/lifetime are specified. Observation gaps and one-shot calls still need names. |
| Source / sink | Directed telemetry or data processing roles. | Useful locally; too directional for all bidirectional requests and peers. |
| Connector / adapter | Integration or protocol translation role. | Useful implementation roles, too narrow for every operation/plugin. |
| Tap | Inspecting a flow in a UI illustration. | Could imply passive, complete or privileged visibility. Avoid as the formal observability contract. |
| Valve / gate | A particular check or flow-control mechanism. | Too narrow for distributed requirements with continuing duties and result-delivery checks. |
| Junction / manifold | A visual composite connection. | Adds jargon and suggests topology behavior not specified by composition alone. Keep composite Conduit. |
| Pump | A producer or continuously driving process. | Implies autonomous work, pressure or automatic wake. Poor universal runtime name. |
| Blueprint / recipe | Reusable setup instructions. | Understandable, but duplicates template and graph and weakens control/runtime coverage. |
| Reservoir / basin | Retained domain data or ownership grouping. | Duplicates the settled Pool concept and makes precise ownership less obvious. |
| Circuit | A connected arrangement. | Suggests a closed loop or circuit breaker. No clear missing concept justifies it. |

My preference is a **technical core with relational imagery**: a package exports a template; its
setup graph invokes operations; its instance participates in Conduits; requests follow routes under
grants and requirements; the Interface observes outcomes. This already sounds like Conduit without
making authors memorize a second vocabulary for familiar computing concepts.

## Review safeguards

Before accepting a term, apply it to [all walkthroughs](WALKTHROUGHS.md), including failure and
shutdown. Choose one primary term for each concept and record useful explanatory synonyms without
turning them into new protocol primitives. Preserve source-system names in research and standard
keywords on external wire surfaces. Names alone should not change current behavior, field ownership,
capability compatibility or implementation pinning.

All 37 stable topics are covered above. Candidate discussions are grouped to reduce repeated rulings;
their outcomes should be recorded in [the existing map](../../VOCABULARY.md#review-and-migration-procedure),
not in a second acceptance ledger here.
