# 002 — Rust supervisor and plugin boundary

Status: proposed, not implemented or executed. Written 2026-11-01 from the
[Rust runtime research](../../docs/research/RUST_RUNTIME.md). This experiment tests engineering
choices within the [specification](../../docs/SPEC.md); it does not replace the
[shared-access YAML walkthrough](../001-shared-access/README.md) or approve its draft syntax.

## Question

Can a small Rust supervisor coordinate instance-owned activity, finite plugin graphs and
recoverable effects while preserving exact plugin implementations and making every managed
outcome inspectable through the Interface?

Test Tokio, a local transactional store and a versioned executable worker first. Use MCP for
the worker's compatible capability surface and identify lifecycle semantics that need an
explicit Conduit profile. Do not add a workflow server, package manager or Wasm engine merely
to run this experiment.

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
- A minimal committed operation store and optional diagnostic exporter that can be disconnected.

Resources, actors and connections are logical instances; the fixture must not assume one process
per object. Pin the tested dependencies and worker artifact identities when the spike is written.
Keep generated databases, logs and binaries in an explicitly selected temporary directory.

## Acceptance cases

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
| Shutdown and restart | Stop the host around intent, dispatch, result and publication boundaries | Recover distinguishable states. Do not infer “not executed” from a missing local response. |

Use deterministic fault hooks for crash windows rather than relying only on timing races. Keep
the fake effect ledger separate from the host's operation store so a lost response genuinely
tests the external-effect boundary. A request ID or received protocol reply alone is not proof
of durable storage or exactly-once execution.

## Measurements and decision gates

Record worker startup time, idle memory, invocation latency, queue bounds, cancellation/end
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
| 2026-11-01 | Research and experiment design | Source-backed report and documentation checks. No fixture code, runtime tests, benchmarks or provider connections yet. |

Record actual results here when executed, including failures and unresolved behavior. Expected
outcomes above are acceptance criteria, not completed tests.
