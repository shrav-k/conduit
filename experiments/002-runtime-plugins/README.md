# 002 — Rust supervision, independent execution and plugins

Status: first-proof layout, core authority/outage rules and combined composition-authoring proof
scope and supervised instance-associated helper default accepted; remaining implementation details
are proposed. Not implemented or executed. Written 2026-11-01 from the
[Rust runtime research](../../docs/research/RUST_RUNTIME.md). This experiment tests engineering
choices within the [specification](../../docs/SPEC.md); it does not replace the
[shared-access YAML walkthrough](../001-shared-access/README.md) or approve its draft syntax.

The [implementation proposal](../../docs/proposals/implementation-v0/README.md) expands the host,
worker, schema and persistence boundaries this spike would exercise. Its review choices remain
pending; this experiment still contains no executable implementation or measured results.
The [distributed-runtime follow-up](../../docs/research/DISTRIBUTED_RUNTIME.md) revises the proposed
layout below. **I1 accepts separate management/execution services and shared policy with
instance-owned state, plus no offline grants in v0.** The first proof uses management and two
independent executors as separate processes on one machine, with controlled connection failures.
That layout is accepted; no runtime or crash-survival guarantee has been demonstrated.
The [cloud authority comparison](../../docs/research/CLOUD_AUTHORITY.md) adds proposed failure
cases below. Canonical instance configuration and results belong to the designated instance
service; management owns templates, registration and shared policy. The experiment tests that
accepted split rather than implementing every authority alternative.

The [Policy/Grant proposal](../../docs/proposals/implementation-v0/AUTHORIZATION.md#8-review-sequence-and-conformance-evidence)
adds candidate common Agent/Interface authorization cases for review. These clarify the intended
protocol proof. Policy as authoritative requirement application and grant administration through
ordinary protected capabilities are now accepted. Governing bindings share the requirement machinery;
there is no separate policy-rule language to implement. A dedicated access-management Resource
in the system Pool is accepted for grants and governing bindings, alongside system management.
Both may share the existing management process/store. Additional cases, exact schemas and startup
wiring remain for review; this does not authorize extra test cases or begin implementation.

The accepted first-launch boundary starts the root Interface from a locally available definition
before registry services exist. Its declared graph directs foundational setup, supported by runtime
owner recognition and minimum loading/persistence. Explicit initialization uses verified local-owner
authority, then establishes ordinary management grants once access management is ready; this initial
authority cannot silently reopen after an outage. Partial-setup recovery and the repair route remain
under review. These are design rulings, not implemented fixture behavior or additional test scope.

## Question

Can Rust services coordinate independently owned activity, finite plugin graphs and
recoverable effects while preserving exact plugin implementations and making every managed
outcome inspectable through the Interface?

Test Tokio, a local transactional store, an authored worker and a code-optional binding to an
existing fake MCP service. MCP is the public invocation base; identify the lifecycle semantics
that need an explicit Conduit profile. Do not add a workflow server, package manager, broad helper
SDK or Wasm engine merely to run this experiment.

## Accepted first-proof layout

The accepted implementation separates management from execution. The accepted first proof runs
one management service, two independently supervised execution services and an Interface frontend
on one machine, as separate processes with controlled connection failures. A combined-host
benchmark is optional, not a prerequisite. One fixture Pool owns Resources across both executors; Pool
membership must not imply process co-location. Place one synthetic Agent on each executor and
include a Resource-to-Resource Conduit across them.

Locally authored plugin code normally runs in supervised helpers associated with its served
instance. The executor retains authoritative state and supervision. Test an ordinary helper crash
separately from executor loss; inspection and uncertain-effect recovery must remain available when
the executor survives. Trusted built-ins may run in-process; graph composition alone needs no extra
helper, and the fake external MCP service retains its own supervision. This chooses neither a
process per instance/plugin/invocation nor an OS sandbox; exact helper grouping remains open.

Use serialized transport and separately addressable endpoints rather than shared mutable Rust
objects. Fault management, executor, worker, capability and observation links independently. The
comparison does not require a scheduler, replicated registry, federation or public network exposure.
Production remote peer enrollment, secret custody and trust need separate validation. Fixed
placement is sufficient; unknown ownership never triggers automatic reassignment.

Test the accepted outage rule: retain supervision and already-dispatched outcomes while
management is unavailable, holding new managed requests whenever current permission cannot be
verified. There are no offline grants. Record actual reconnect, evidence and output-draining
behavior; the policy is accepted, but these guarantees have not yet been demonstrated.
Validate across two machines before claiming remote support, retaining the same identity/ownership
contracts. The local proof alone does not establish machine-loss tolerance or production remote
enrollment and credential placement.

## Smallest useful fixture

- A test Interface that issues commands and reads retained evidence through the same management
  path. A production TUI is unnecessary.
- One system Pool owning system management and dedicated access-management Resources, plus one
  fixture Pool. The latter owns a fake authentication Resource and a
  fake service Resource; fixture identities are not real credentials.
- Two synthetic Agent instances with independent consumer identities. They do not call models.
- A Conduit attachment per consumer with distinct allowed operations over shared service access.
- A setup graph with authentication preparation, instance-managed service startup and an
  initialization step that can fail. The service has one revision-checked configuration control.
- A versioned worker with two distinguishable implementations, A and B, and a temporary local
  effect ledger that permits inspection after a deliberately lost response.
- A code-optional contribution bound to the fake MCP service through a common adapter, with
  no new executable for that contribution; the fake service need not implement the Conduit profile.
- A standard MCP test client using an independently exposed managed capability through a
  pre-established authorized attachment, without the aggregate frontend in the request path.
- A management store for templates, registration and shared policy, plus instance-authority
  stores for resolved definitions, confirmed settings/revisions and operation results. Aggregate
  views are projections, not a second writable configuration. Do not assume cross-store transactions. An optional
  diagnostic exporter can be disconnected independently.

Resources, actors and connections are logical instances; the fixture must not assume one process
per object. Pin the tested dependencies and worker artifact identities when the spike is written.
Keep generated databases, logs and binaries in an explicitly selected temporary directory.

## Acceptance cases

### Authoring acceptance

Before treating the fixture as a successful implementation pattern, demonstrate:

- A small plugin authored outside the host's source modules, with its own declared contract,
  loaded without changing core dispatch or adding its type name to a hard-coded switch.
- A template using that plugin alongside existing operations, then producing two independent
  instances from the same recipe. Show typed binding errors and useful author-facing diagnostics.
- A template-only variation that reuses existing capabilities and connections without new code.
- A managed Resource extension exposing its supported lifecycle/control/observation contract
  through the same system. A tiny finite-operation plugin need not implement those unused hooks.
- Inspection through the Interface of resolved contracts, selected code, setup progress, allowed
  controls and attributable outcomes. Existing permission, identity and recovery tests still apply.

The accepted authoring direction is lightweight Conduit support with real plugins testing the
public protocol; a helper library is not an acceptance prerequisite. Concrete package/wire
formats remain under review, with detailed syntax deferred until conceptual rulings finish.
Plugins publish contracts with their implementations: demonstrate template validation against
those contracts without starting plugin code, then test runtime conformance separately.
Exercise explicit local installation of the authored package and the code-optional package under
the accepted distribution scope. Verify that installation alone does not instantiate objects,
authenticate services or grant access; existing implementation-retention tests cover later package
versions. No marketplace or automatic cross-language dependency resolver is required.
Record author effort, repeated plumbing and hidden/manual prerequisites as evidence alongside
correctness, so later helpers address demonstrated needs.
Built-in-only demonstrations or faster execution cannot substitute for proving this extension path.

Code-optional contributions and independently accessible components are accepted. Compare the
declaration-only integration's validation, access and observations with the authored implementation;
both paths must be demonstrated. Exercise the standard MCP client with no Conduit graph SDK while
retaining the managed component's authority/evidence dependencies. Also show that using a standalone
implementation outside Conduit supplies only that implementation's own guarantees. Production placement,
production enrollment and remote deployment details remain under review; neither public accessibility nor a fake endpoint
proves a production standalone deployment.

### Runtime and contract acceptance

| Case | Fault or action | Required observation |
| --- | --- | --- |
| Contract checking | Missing schema, wrong input type, unsupported restriction or missing implementation | Reject invalid configuration or report unready execution as appropriate. Never drop a filter or substitute code. |
| Exact implementation | Start with A, make B available, then restart the original instance | Original remains bound to A or reports A unavailable. A replacement explicitly uses B. |
| Code-optional binding | Validate published contracts with the provider stopped, then connect the fake service through the retained adapter | Registration/validation does not execute the provider. Ready calls use the declared capability; missing/incompatible behavior is reported, not supplied by an invented guarantee. |
| External service drift | Change the fake provider's supported operation/schema after preparation | Apply existing known-drift/explicit-refresh rules; retain declaration/adapter identity and report provider compatibility separately from code pinning. |
| Independent MCP access | Invoke through the managed endpoint using the ordinary MCP client with no aggregate frontend attached | Current caller/path restrictions and operation evidence still apply; later Interface inspection finds the attributable result. Raw caller arguments cannot grant management authority. |
| External lifecycle | Disconnect/retire the client Resource attached to an independently running fake server | Release owned connections; preserve provider process/data unless an explicitly authorized management operation owns that effect. |
| Partial setup | Fail initialization after managed service startup | Service remains owned and inspectable; setup is incomplete. Retry uses valid recorded progress. |
| Interactive wait | Fake authentication asks for a fixture response; restart host before supplying it | Wait and operation identity remain inspectable; recovery follows the plugin contract, without a fabricated login result. |
| Uncertain effect | Worker performs a write and exits before returning its result | No blind replay. Inspect the effect ledger, reconcile and preserve one logical result. |
| Late result | Delay a result from an obsolete attempt until a newer attempt has been reconciled | Old response cannot overwrite current confirmed state. Its evidence remains attributable. |
| Cancellation | Block a worker and request cancellation | Interface distinguishes requested cancellation from confirmed end. Process escalation, if used, is recorded and reaped. Remote-effect uncertainty remains explicit. |
| Host responsiveness | Busy or uncooperative worker while another consumer inspects state | Bounded queues and worker isolation prevent unbounded memory growth or loss of basic inspection. |
| Shared access | Both Agents use one fake service identity with different operation permissions | Caller and service identity remain distinct. Each complete route enforces its own restrictions. |
| Revocation/reconnect | Remove access while a request waits, then restore the transport | Dispatch/retry rechecks current authority. No alternate account/path is selected silently. |
| Concurrent control | Two callers submit a change from the same configuration revision | Apply one at a time; stale request conflicts. Store confirmed settings separately from unresolved requests. |
| Confirmed change, lost receipt | Apply a control and commit its responsible service's result, then cut the reply to management/caller | Recover the same operation and the instance authority's committed revision. Management updates its projection without a second canonical configuration commit. Never repeat the effect solely to refresh a view. |
| Stale aggregate inventory | Delay observations while the authoritative instance state advances | Interface identifies source/freshness; older observations cannot overwrite current state. A checked mutation reaches the authority rather than trusting a stale inventory snapshot. |
| Policy reachability versus process survival | Break authorization reachability while an executor still supervises activity | Distinguish retained supervision, already-admitted effects and new effects requiring admission. Hold new managed requests without verifiable permission; retain supervision and dispatched outcomes within admitted contracts. No offline grants or process-derived permission. |
| Retention versus delivery during policy loss | Dispatch a permitted effect, lose policy contact, retain its confirmed outcome, then withdraw the caller's access before reconnect | Preserve the outcome at its instance authority; later retrieval/delivery checks current access. The next distinct protected request waits or reports unavailable if current permission cannot be established. A retained result is not a new read grant. |
| Interface reconnect | Close all frontend attachments, then reopen | Instance-owned work follows lifecycle policy; committed outcomes remain inspectable. Closing a frontend does not itself terminate it. |
| Export failure | Disconnect or stall diagnostics export | Local outcomes remain committed and inspectable; export coverage/gaps are visible. |
| Management loss | Stop management while both executors own live activity | Verify retained supervision and dispatched outcomes; hold new managed requests without verifiable permission. Reconnect without duplicate work and preserve existing stop/revocation rules. |
| One executor fails | Stop executor A while B remains available | B's independent work and the Pool's other usable Resources remain available; A's status is ended or unknown according to evidence. |
| Cross-executor route | Interrupt the Resource-to-Resource Conduit transport | Report affected availability without detaching logical participation, changing account/path or inventing a replacement Resource. |
| Stale coordinator | Deliver a delayed control from an obsolete management generation | The target verifies current authority/ownership and rejects obsolete admission; generation labels alone are not a fencing guarantee. |
| Evidence backpressure | Disconnect observation aggregation and fill bounded diagnostic queues | Required local outcomes remain recoverable; optional drops/gaps are visible; undrained output must not silently defeat process-survival claims. |
| Shutdown and restart | Separately stop management, executor or worker around effect/commit boundaries | Distinguish each source's known state; recover instance-owned results and management projections without assuming a cross-service transaction. |

Use deterministic fault hooks for crash windows rather than relying only on timing races. Keep
the fake effect ledger separate from the host's operation store so a lost response genuinely
tests the external-effect boundary. A request ID or received protocol reply alone is not proof
of durable storage or exactly-once execution.

## Combined composition-authoring proof

Accepted experiment scope; not implemented or executed. Demonstrate requirement and executable
composition together using the existing fake service, two synthetic consumers and separate
authorities. The accepted process layout includes the authored-code helper default above. Exact
syntax, permanent core vocabulary and helper grouping remain separate choices:

- Reuse the fixture's externally authored operation plugin in an exported graph-backed operation,
  then invoke that operation from a template. Composition should need no new glue executable or
  hard-coded core dispatch. Record the actual author-written artifacts, not only host-side tests.
- Exercise default result derivation and an explicitly constructed domain result over the same
  underlying behavior. Preserve effects/uncertainty and scoped child evidence. Fault a setup step,
  then explicitly retry the same invocation to demonstrate still-valid progress reuse and child
  recovery instead of blindly replaying effects.
- Compose two reusable starting requirements in a named definition and use it in Conduit templates.
  Operation allowlisting and a result-count bound are fixture candidates, not an approved permanent
  core catalog. Demonstrate constituent enforcement without requiring a generic merge/solver.
- Publish one custom requirement outside core modules, with typed parameters, declared fulfillment
  duties and implementation support selected in the endpoint template. Allowed regions on a fake
  deployment operation is a candidate; this makes no claim about real cloud or CLI interception.
- Show valid/invalid parameter diagnostics without running the service; show that known schemas
  with missing runtime support leave the affected connection unready rather than dropping the rule.
- Demonstrate allowed and rejected invocations and inspect actual fake-service dispatches. Interface
  evidence identifies selected contracts/implementations, fulfillment responsibilities and outcomes;
  recorded support alone is not evidence of enforcement.
- Change a relevant authorized fact or governing restriction after establishment and show that a
  cached setup result does not bypass required runtime checking. Preserve unrelated usable work.

The purpose is to prove the public extension/composition boundaries without a hard-coded requirement
or provider switch in Rust, a large vocabulary, expression engine or general solver. This accepted
scope adds no live cloud dependencies and does not authorize implementation start. Package syntax,
helper grouping and detailed evidence capture remain separate review items.

## Measurements and decision gates

Record arrangement, per-service/worker startup time and idle memory, local/cross-executor invocation
latency, queue bounds, cancellation/end
latency and recovery time on the actual test machine. Include dependency versions, repetitions
and fixture payload sizes. No performance targets have been approved; compare measured costs
with the workload rather than inventing thresholds here.

The spike should produce a short contract trace showing typed discovery, invocation, progress,
inspection and recovery. Map each exchange to standard MCP facilities or a clearly identified
extension. If most behavior requires a second ad hoc protocol, revisit the worker contract
before implementing more providers.

After the process path works, optionally run one pure transform through Wasmtime or Extism.
Compare packaging, typed conversion, cancellation and host-call limits. That comparison should
decide whether a Wasm lane buys enough to justify another runtime; it is not a prerequisite for
the first Agent collaboration experiment.

Advance to live Xerj and model-account tests only after the synthetic evidence is recorded.
I2's accepted Agent delivery order is native Claude/Codex adapters first, then Conduit's owned
Rust model/tool loop. Native integration/account validation is macOS-first, with a portable core
and OS-specific adapters; other platform support needs separate evidence. Native success does not establish
the owned loop's provider behavior or its subscription compatibility.
The separate two-Claude/two-Codex exercise still needs explicit native authentication and
account/private-state verification; this fixture cannot establish those properties.

## Evidence log

| Date | Work | Evidence |
| --- | --- | --- |
| 2026-11-01 | Research and experiment design | Source-backed reports and documentation checks. Separate execution, shared-policy/instance-state authority and no offline grants accepted; I1 first-proof scope accepted: separate management and two executors on one machine. Planned proof plus accepted code-optional integration and independent MCP client cases. No fixture code, runtime tests, benchmarks or provider connections yet. |

Record actual results here when executed, including failures and unresolved behavior. Expected
outcomes above are acceptance criteria, not completed tests.
