# 002 — Rust supervision, independent execution and plugins

Status: proposed, not implemented or executed. Written 2026-11-01 from the
[Rust runtime research](../../docs/research/RUST_RUNTIME.md). This experiment tests engineering
choices within the [specification](../../docs/SPEC.md); it does not replace the
[shared-access YAML walkthrough](../001-shared-access/README.md) or approve its draft syntax.

The [implementation proposal](../../docs/proposals/implementation-v0/README.md) expands the host,
worker, schema and persistence boundaries this spike would exercise. Its review choices remain
pending; this experiment still contains no executable implementation or measured results.
The [distributed-runtime follow-up](../../docs/research/DISTRIBUTED_RUNTIME.md) revises the proposed
layout below. **The user has not accepted I1.** The two-executor comparison is proposed evidence
for that ruling, not an approved deployment or crash-survival guarantee.

## Question

Can Rust services coordinate independently owned activity, finite plugin graphs and
recoverable effects while preserving exact plugin implementations and making every managed
outcome inspectable through the Interface?

Test Tokio, a local transactional store and a versioned executable worker first. Use MCP for
the worker's compatible capability surface and identify lifecycle semantics that need an
explicit Conduit profile. Do not add a workflow server, package manager or Wasm engine merely
to run this experiment.

## Proposed placement comparison

Compare independent execution with the earlier shared-host baseline before claiming either is
sufficient. Run one management service, two independently supervised execution services and an
Interface frontend on one machine. One fixture Pool owns Resources across both executors; Pool
membership must not imply process co-location. Place one synthetic Agent on each executor and
include a Resource-to-Resource Conduit across them.

Use serialized transport and separately addressable endpoints rather than shared mutable Rust
objects. Fault management, executor, worker, capability and observation links independently. The
comparison does not require a scheduler, replicated registry, federation or public network exposure.
Production remote peer enrollment, secret custody and trust need separate validation. Fixed
placement is sufficient; unknown ownership never triggers automatic reassignment.

First demonstrate retaining an already-running synthetic process while management is unavailable,
with new effects held whenever current authority cannot be verified. This is a proposed baseline,
not an implicit offline-grant policy. Record actual reconnect, evidence and output-draining behavior.
A later two-machine run should retain the same identities/ownership contracts; I1 determines when
that run becomes mandatory.

## Smallest useful fixture

- A test Interface that issues commands and reads retained evidence through the same management
  path. A production TUI is unnecessary.
- One system Pool and one fixture Pool. The latter owns a fake authentication Resource and a
  fake service Resource; fixture identities are not real credentials.
- Two synthetic Agent instances with independent consumer identities. They do not call models.
- A Conduit attachment per consumer with distinct allowed operations over shared service access.
- A setup graph with authentication preparation, instance-managed service startup and an
  initialization step that can fail. The service has one revision-checked configuration control.
- A versioned worker with two distinguishable implementations, A and B, and a temporary local
  effect ledger that permits inspection after a deliberately lost response.
- A minimal registry/management store plus executor-local recovery evidence, with each fact's
  authority explicitly assigned. Do not assume a transaction can span both stores. An optional
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
formats remain under review. Record author effort, repeated plumbing and hidden/manual
prerequisites as evidence alongside correctness, so later helpers address demonstrated needs.
Built-in-only demonstrations or faster execution cannot substitute for proving this extension path.

### Runtime and contract acceptance

| Case | Fault or action | Required observation |
| --- | --- | --- |
| Contract checking | Missing schema, wrong input type, unsupported restriction or missing implementation | Reject invalid configuration or report unready execution as appropriate. Never drop a filter or substitute code. |
| Exact implementation | Start with A, make B available, then restart the original instance | Original remains bound to A or reports A unavailable. A replacement explicitly uses B. |
| Partial setup | Fail initialization after managed service startup | Service remains owned and inspectable; setup is incomplete. Retry uses valid recorded progress. |
| Interactive wait | Fake authentication asks for a fixture response; restart host before supplying it | Wait and operation identity remain inspectable; recovery follows the plugin contract, without a fabricated login result. |
| Uncertain effect | Worker performs a write and exits before returning its result | No blind replay. Inspect the effect ledger, reconcile and preserve one logical result. |
| Late result | Delay a result from an obsolete attempt until a newer attempt has been reconciled | Old response cannot overwrite current confirmed state. Its evidence remains attributable. |
| Cancellation | Block a worker and request cancellation | Interface distinguishes requested cancellation from confirmed end. Process escalation, if used, is recorded and reaped. Remote-effect uncertainty remains explicit. |
| Host responsiveness | Busy or uncooperative worker while another consumer inspects state | Bounded queues and worker isolation prevent unbounded memory growth or loss of basic inspection. |
| Shared access | Both Agents use one fake service identity with different operation permissions | Caller and service identity remain distinct. Each complete route enforces its own restrictions. |
| Revocation/reconnect | Remove access while a request waits, then restore the transport | Dispatch/retry rechecks current authority. No alternate account/path is selected silently. |
| Concurrent control | Two callers submit a change from the same configuration revision | Apply one at a time; stale request conflicts. Store confirmed settings separately from unresolved requests. |
| Interface reconnect | Close all frontend attachments, then reopen | Instance-owned work follows lifecycle policy; committed outcomes remain inspectable. Closing a frontend does not itself terminate it. |
| Export failure | Disconnect or stall diagnostics export | Local outcomes remain committed and inspectable; export coverage/gaps are visible. |
| Management loss | Stop management while both executors own live activity | Under the proposed independent arrangement, processes remain supervised; block effects needing unavailable authority, retain evidence and reconnect without duplicate work. |
| One executor fails | Stop executor A while B remains available | B's independent work and the Pool's other usable Resources remain available; A's status is ended or unknown according to evidence. |
| Cross-executor route | Interrupt the Resource-to-Resource Conduit transport | Report affected availability without detaching logical participation, changing account/path or inventing a replacement Resource. |
| Stale coordinator | Deliver a delayed control from an obsolete management generation | The target verifies current authority/ownership and rejects obsolete admission; generation labels alone are not a fencing guarantee. |
| Evidence backpressure | Disconnect observation aggregation and fill bounded diagnostic queues | Required local outcomes remain recoverable; optional drops/gaps are visible; undrained output must not silently defeat process-survival claims. |
| Shutdown and restart | Separately stop management, executor or worker around effect/commit boundaries | Distinguish each source's known state; reconcile receipts and registry updates without assuming a cross-service transaction. |

Use deterministic fault hooks for crash windows rather than relying only on timing races. Keep
the fake effect ledger separate from the host's operation store so a lost response genuinely
tests the external-effect boundary. A request ID or received protocol reply alone is not proof
of durable storage or exactly-once execution.

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
The separate two-Claude/two-Codex exercise still needs explicit native authentication and
account/private-state verification; this fixture cannot establish those properties.

## Evidence log

| Date | Work | Evidence |
| --- | --- | --- |
| 2026-11-01 | Research and experiment design | Source-backed reports and documentation checks; I1 remains open. Added proposed independent-executor comparison. No fixture code, runtime tests, benchmarks or provider connections yet. |

Record actual results here when executed, including failures and unresolved behavior. Expected
outcomes above are acceptance criteria, not completed tests.
