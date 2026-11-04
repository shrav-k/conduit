# 002 — Rust supervision, independent execution and plugins

Status: first-proof layout, core authority/outage rules and combined composition-authoring proof
scope and supervised instance-associated helper default accepted; remaining implementation details
are proposed. Not implemented or executed. Written 2026-11-01 from the
[Rust runtime research](../../docs/research/RUST_RUNTIME.md). This experiment tests engineering
choices within the [specification](../../docs/OVERVIEW.md); it does not replace the
[shared-access YAML walkthrough](../001-shared-access/README.md) or approve its draft syntax.

The [implementation proposal](../../docs/implementation/README.md) expands the host,
worker, schema and persistence boundaries this spike would exercise. Accepted I1–I6 boundaries remain in force; C1–C3 conceptual closure is complete and concrete authoring/syntax returns in a future PR. This experiment still contains no executable implementation or measured results.
The [distributed-runtime follow-up](../../docs/research/DISTRIBUTED_RUNTIME.md) revises the proposed
layout below. **I1 accepts separate management/execution services and shared policy with
instance-owned state, plus no offline grants in v0.** The first proof uses management and two
independent executors as separate processes on one machine, with controlled connection failures.
That layout is accepted; no runtime or crash-survival guarantee has been demonstrated.
The [cloud authority comparison](../../docs/research/CLOUD_AUTHORITY.md) adds proposed failure
cases below. Canonical instance configuration and results belong to the designated instance
service; management owns templates, registration and shared policy. The experiment tests that
accepted split rather than implementing every authority alternative.

The [Policy/Grant proposal](../../docs/proposals/authoring-syntax/AUTHORIZATION.md#8-review-sequence-and-conformance-evidence)
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
authority cannot silently reopen after an outage. Explicit setup resume and foundational repair
use the locally loadable root Interface's declared recovery operations, with runtime owner
verification even when ordinary management access is broken. Valid progress and existing state are
preserved; uncertain effects require reconciliation. V0 now includes a minimal published persistence
plugin contract with one bundled implementation. This proof will use that boundary for its retained
management/executor state as a service runtime dependency, with Interface inspection/recovery.
It does not require several engines or a separately registered storage Resource. Preserve the public
contract for a future Pool-owned representation; that is not added experiment scope. The bundled
implementation uses SQLite, with separate local databases for management and each executor in this
experiment. Exact methods, Rust bindings, durability settings and fixtures remain implementation/
authoring work; no storage or recovery behavior has been validated yet.
These are design rulings, not implemented fixture behavior or additional test scope.

## Conformance milestone

C1 accepts an independently authored client and plugin working with the reference runtime through
published contracts and conformance tests. This complements the existing composition-authoring proof:
record which public contracts the client/plugin uses, exercise supported and rejected behavior, and
avoid private runtime APIs or special-case core dispatch. Exact fixtures follow the authoring PR.
A second complete runtime is deferred; this experiment cannot claim runtime-to-runtime portability.
No additional runtime, remote deployment or provider support is inferred from that milestone.

C2 supplies the dependent-call acceptance boundary: a Resource's authorized internal connection must
not widen the originating caller's permitted effects or returned content. Contrast that with autonomous
Resource work using its own authority. The contract decides the enforcement without requiring an
Agent to hold raw access to every backing service. Concrete fixture cases remain to be drafted.

C3 supplies the retirement boundary: stop new work and confirm required instance shutdown duties
before unregistration. Preserve shared dependencies and operation evidence; an external effect can
outlive local shutdown. Missing required confirmation keeps retirement incomplete. Exercise these
cases through the same supported lifecycle contracts, without inferring cancellation or adding a
universal lifecycle enum. No retirement test has run yet.

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

The accepted observation bundle uses plugin contracts and Conduits for correlated traces of operations,
graphs and plugins, structured diagnostics and a small service-health metric set. The Interface must
connect existing scenario failures to their instances, access paths, outcomes and recovery evidence.
For this low-volume fixture, record all instrumented operation traces at source and explicitly enable
bounded fixture-input/output capture using known synthetic data and fake credential references.
Report truncated capture, buffering/export loss and coverage gaps. Optional OpenTelemetry export
exercises the export boundary; required local operation/recovery evidence remains independent and
unsampled. Token/CPU collectors, comprehensive monitoring and an external dashboard are not
prerequisites. Real-workload full content still requires declared recording policy. These expectations
apply to the existing scenarios; no runtime test has been executed by this documentation update.

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
Composition requires no primary plugin. The existing composed-operation fixture must resolve its
required setup/runtime/control contributions explicitly, including declared defaults, and expose
the retained selections through the Interface. Known incompatible bindings must be diagnosed;
unavailable code/services remain readiness gaps, and explicit invalid choices must not silently
fall back to another implementation. This applies the accepted v0 authoring model to the existing
proof; concrete alias/export syntax remains I7 work.
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
| Batch prevalidation | Submit a valid history Resource template with a malformed or schema-invalid Agent template; separately use a missing required schema, graph cycle, invalid prerequisite or output-generated definition | No entries are written. Diagnostics identify the parsing/schema/graph problem. All definitions must be supplied before execution. Validation does not start plugin code or runtime setup. |
| Batch duplicate targets | Submit two updates to one stable object ID or two creates for one kind/name; repeat with an explicit prerequisite between them | Reject the batch before any writes. Ordering does not permit repeated targets. Combine edits into one checked replacement or use separate submissions. |
| Repeated template instantiation | Register one synthetic reviewer Agent template, then explicitly create and start two instances from it | Both instances have distinct identities, private state and independently managed lifecycles. Template reuse does not trigger duplicate-target rejection or consume/rewrite the template definition. Registration alone launches neither instance. |
| Batch dependencies and independent work | Submit complete History, Reviewer and Pool templates with an explicit History → Pool prerequisite; vary file order and cause History to fail | Pool waits for its prerequisite and remains blocked on failure. Reviewer remains independently eligible under current checks and execution limits. File order and ordinary object references do not infer edges. Shared graph cancellation and valid-progress recovery semantics apply. |
| Batch registration outcomes | Pass whole-batch validation for a history Resource template and an Agent template; arrange a name conflict for the Agent after history registration succeeds | Report per-object outcomes and preserve the successful history registration. Every write retains current authority and applicable precondition checks; prevalidation reserves nothing. No rollback, provisioning or Agent launch is inferred. |
| Portable integers | Exercise both limits of ±9,007,199,254,740,991, outside values and an explicitly decimal-string counter through YAML/JSON configuration, inputs and outputs | In-range integers remain exact across clients, subject to schema bounds. Outside integers are rejected before precision is lost; there is no rounding into range or automatic string conversion. An invalid plugin output preserves effect evidence under the existing output-validation rule. |
| Fractional values | Exchange `0.7` and `1.5` through equivalent YAML/JSON configuration, inputs and outputs; compare explicitly decimal-string and integer-millisecond contracts | Ordinary fractional values use finite binary64 semantics with normal representational rounding. Exact representations retain their declared meaning without implicit conversion. Integer checks remain strict; detailed comparison, `multipleOf`, range and round-trip cases must demonstrate the public profile rather than inherit untested library behavior. |
| Standard-format boundary | Exercise all 19 selected JSON Schema 2020-12 formats, then attempt a plugin-defined `workspace-id` format | Positive/rejection fixtures establish their standard syntax semantics before claiming support. URI checks make no endpoint requests. Report the custom format as unsupported without loading a plugin validator; a reusable schema using supported string constraints remains possible. Provider-native schemas retain their own dialect semantics. |
| Descriptive schema annotations | Add a namespaced slider hint to a limit schema; inspect with tooling that supports it and tooling that does not | Both retain the same 1–100 validation and control/authority behavior; unsupported presentation hints need not affect a numeric editor. A misspelled schema keyword outside the annotation map still fails acceptance. Exact map notation/value schemas remain open. |
| Active implementation preservation | Start with A, edit local plugin code/contracts to B and invoke the running instance again | Running work still uses A. Editing source does not replace its implementation or require publishing A as a reusable package. |
| Exact implementation on restart | Start with A, make B available, then restart the original instance | Original remains bound to A or reports A unavailable. It never silently adopts B. |
| Development replacement | Retire the A instance and create a replacement from revised source B, reusing fixture backing data explicitly | B is selected through normal contracts; both versions need not run together. No implicit grant transfer or backing-data deletion. Historical outcome records do not require a selectable build archive. |
| Unavailable edit-mode implementation | After safe retirement/cleanup of unreleased A's private contents, request fresh creation requiring A with only B available | Report unavailable A without substitution. A digest/export or draft version field alone does not preserve a release. |
| Published release | Publish fixed, versioned A through a distribution-source fixture, retire its instances, edit source to B, then retrieve/select released A for fresh creation | Verify A's digest independently of its old instances. B stays in edit mode until separately published under a new version. The fixture does not select production hosting, a local-only release facility or a backward-compatibility promise. |
| Code-optional binding | Validate published contracts with the provider stopped, then connect the fake service through the retained adapter | Registration/validation does not execute the provider. Ready calls use the declared capability; missing/incompatible behavior is reported, not supplied by an invented guarantee. |
| External service drift | Change the fake provider's supported operation/schema after preparation | Apply existing known-drift/explicit-refresh rules; retain declaration/adapter identity and report provider compatibility separately from code pinning. |
| Independent MCP access | Invoke through the managed endpoint using the ordinary MCP client with no aggregate frontend attached | Current caller/path restrictions and operation evidence still apply; later Interface inspection finds the attributable result. Raw caller arguments cannot grant management authority. |
| External lifecycle | Disconnect/retire the client Resource attached to an independently running fake server | Release owned connections; preserve provider process/data unless an explicitly authorized management operation owns that effect. |
| Partial setup | Fail initialization after managed service startup | Service remains owned and inspectable; setup is incomplete. Retry uses valid recorded progress. |
| Optional setup step | Fail an explicitly optional diagnostics step while required setup succeeds; in a second fixture make required verification depend on its missing output | The first fixture reports diagnostics failure without that failure alone blocking setup success. The second remains incomplete because required verification lacks its prerequisite. Preserve effects, validation, mandatory type/access checks and lifecycle duties in both cases. |
| Shared graph-step classification | Run required application plus optional diagnostics as setup, a reusable graph-backed operation and a change handler | All three use required-by-default steps and explicit optionality. Required failure prevents successful graph completion; optional failure alone does not. Dependencies and mandatory obligations still apply. Keep public-output availability and supported partial control confirmation separate from overall success; track outstanding optional activity rather than abandoning it. |
| V0 ordering-trigger boundary | Submit an authored failure/completion trigger; separately fail a migration whose plugin declares required lock cleanup | Reject the unsupported trigger before graph execution. The valid graph keeps success-dependent verification blocked, while the plugin/instance contract performs its required cleanup or records inability/uncertainty. Cleanup is not contingent on migration success; no rollback or unconfirmed effect cancellation is inferred. |
| Interactive wait | Fake authentication asks for a fixture response; restart host before supplying it | Wait and operation identity remain inspectable; recovery follows the plugin contract, without a fabricated login result. |
| Uncertain effect | Worker performs a write and exits before returning its result | No blind replay. Inspect the effect ledger, reconcile and preserve one logical result. |
| Late result | Delay a result from an obsolete attempt until a newer attempt has been reconciled | Old response cannot overwrite current confirmed state. Its evidence remains attributable. |
| Cancellation | Block a worker and request cancellation | Interface distinguishes requested cancellation from confirmed end. Process escalation, if used, is recorded and reaped. Remote-effect uncertainty remains explicit. |
| Host responsiveness | Busy or uncooperative worker while another consumer inspects state | Bounded queues and worker isolation prevent unbounded memory growth or loss of basic inspection. |
| Shared access | Both Agents use one fake service identity with different operation permissions | Caller and service identity remain distinct. Each complete route enforces its own restrictions. |
| Revocation/reconnect | Remove access while a request waits, then restore the transport | Dispatch/retry rechecks current authority. No alternate account/path is selected silently. |
| Concurrent control | Two callers submit a change from the same configuration revision | Apply one at a time; stale request conflicts. Store confirmed settings separately from unresolved requests. |
| Partial control confirmation | A fixture control that explicitly permits partial results confirms limit 10 → 15 while timeout 30 → 60 remains uncertain; separately return a subset that violates a cross-field constraint | Record 15 only when the complete resulting stored configuration is valid. Retain timeout 30 as last confirmed with visible uncertainty, without declaring whole-request success or releasing applicable coordination. Reject invalid stored results while retaining actual effect evidence. Do not infer rollback or replay the uncertain effect. |
| Unchanged control | With limit 15 already stored, invoke a control that verifies/refreshes the fixture provider and confirms 15 using the current revision; repeat with a stale revision | Preserve the configuration revision after the valid confirmed invocation while recording its work/outcome. Do not skip declared behavior or manufacture a cached success from equal values. Reject the stale checked request; other current state and authority checks remain effective. |
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

- Author a package with one explicit entry document declaring exports/dependencies. Exercise inline
  definitions and an equivalent version referencing package-local files; inspection resolves the
  same declared contracts without starting plugin code. Include a definition file with no export
  declaration and verify that it is not exposed merely because it is packaged or conventionally named.
  These variants can have different artifact digests; this checks their declared public surface.
- Declare an operation and a template with the same public export name, including a variant split
  across referenced files. Reject the collision before plugin execution rather than choosing by
  kind or load order. With distinct names, verify that selecting a template where an operation is
  required still reports a kind mismatch. Internal definition names remain a separate concern.
- Add a starter package that explicitly re-exports a dependency's public template under a different
  name. Verify that it resolves to the original definition without copied template content or a
  wrapper, retains the contract and resolved dependency identities, and uses the original internal
  reference scope even when the starter declares a same-named private helper. Check the re-export
  against public-name uniqueness and expected kind/contract rules; it grants no additional access.
- Select the history implementation once under a template-local alias, then reference its distinct
  setup operation, runtime driver and control handler exports. Include a second implementation for
  an independent check, without a primary alias. Verify each reference's kind/contract and retained
  selection; a missing alias or wrong-kind export must not trigger fallback or implicit lookup.
- Reuse the fixture's externally authored operation plugin in an exported graph-backed operation,
  then invoke that operation from a template. Composition should need no new glue executable or
  hard-coded core dispatch. Record the actual author-written artifacts, not only host-side tests.
- Declare steps in a map keyed by locally unique names. Reorder entries and verify unchanged
  dependency/ordering constraints without requiring one fixed schedule for independent work.
  Reject duplicate step keys before execution. Reuse the same local names in two distinct graphs
  and verify that they do not alias or expose each other's internal steps.
- Place a migration-success prerequisite on a verification step alongside its operation and inputs.
  Verification also consumes startup's database output. Require both dependencies, without a separate
  authored edge collection or duplicate prerequisites for data bindings. Reorder the step map and
  preserve those constraints; an unrelated step may still run independently. Reject missing predecessor
  names and cycles formed by combining explicit prerequisites with output-implied dependencies.
  Failed or uncertain migration must not release verification. Field spelling remains provisional.
- Invoke the same reusable graph-backed operation from two templates with differently named
  configuration fields, explicitly binding each to its declared `limit` input. Verify that the graph
  receives the supplied value without inheriting either caller's configuration namespace. Include an
  undeclared caller-configuration reference and require a diagnostic rather than implicit lookup.
  Keep the operation's supported scoped context and authorized Resource access available as declared.
- Supply a graph with `{limit: 15, options: {timeout: 30}}` under its input contract. Verify that
  the graph-input source with `/options/timeout` selects 30, and omitted/empty pointers select
  the complete payload, with no separate input-name selector. Test both step-input and public-output
  bindings. Missing members still error and receiving schemas still apply; no caller settings or
  invocation metadata are implicitly added to the input payload.
- Invoke code and graph implementations of a Conduit-authored operation with an omitted `limit`
  whose declared default is 10. Require both to receive the same validated payload containing 10,
  with `/limit` available to the graph. Repeat through a child invocation with its own input contract.
  Check explicit valid overrides, invalid values, permitted and forbidden nulls, conflicting defaults
  and invalid materialized values. A reference selecting a missing upstream member must still block
  invocation even if the receiving field has a default. Retain earlier effects and verify that the
  adapter does not automatically impose this default preparation on provider-native MCP inputs.
- Omit an optional `retry` object that has no default of its own but whose `delaySeconds` property
  defaults to 5. Verify absence is preserved in instance configuration and in prepared code/graph
  inputs, including child invocations. A graph reference to the absent `/retry` still fails rather
  than receiving a synthesized object. Repeat with a declared parent default of `{delaySeconds: 5}`
  and verify materialization subject to normal validation/conflict checks. Preserve authored template
  omissions; a required parent still absent after preparation fails validation. Explicit nulls and
  provider-native MCP schemas retain their separate rules.
- Supply `retry: {}` with a child default of 5 for `delaySeconds`, and verify prepared configuration
  and code/graph inputs contain that child. Preserve explicit child values and permitted nulls;
  reject invalid supplied values and conflicting applicable defaults. Give the parent a different
  whole-object default and verify it is not substituted for the supplied object. Do not merge old
  settings or fill empty lists. When the supplied object comes from a producer output, prepare the
  receiving payload without altering that output's recorded value. Template omissions and the
  absent-parent rule remain unchanged.
- Validate local/remote variants with explicit timeout values, then add a branch-dependent timeout
  default and require an unsupported-schema diagnostic before acceptance or invocation. Supplying
  that timeout explicitly must not bypass the schema rejection. Remove the branch-local default and
  verify conditional/alternative validation still operates; use a suitable common default outside
  the alternatives in a separate case. Retain ordinary/composed-default conflict behavior and the
  provider-native boundary, without silent omission or trial insertion to select a branch.
- Declare an integer range of 1–100 with a default of 0 and require schema rejection identifying the
  default and constraint, including when a caller supplies 15. Perform supported preparation before
  judging a default's validity. Keep conflicting otherwise-valid defaults as a separate case that
  accepts an explicit valid resolution. Validate actual prepared payloads even after schema checks
  pass; early checks do not claim validity in every future composition or alter provider-native rules.
- Declare named database and diagnostics runtimes under a fixture instance whose type supports both.
  Use a map keyed by the local names, reject duplicate keys, and verify that reordering entries has
  no execution meaning. Give a second instance the same local names without aliasing runtime identities.
  Select the database runtime through the runtime-target source and forward its typed target reference
  through a reusable graph input to startup. Verify that the responsible service resolves the intended
  instance-owned definition, with prepared settings passed explicitly. Check unknown targets and
  incompatible or unauthorized references before effects. Selection alone must not start either
  runtime; a supplied replacement definition cannot substitute for the target. Distinguish the target
  reference from the later usable service result, whose publication follows the startup contract.
  Reference encoding remains provisional; this is planned coverage, not a claim of implementation.
- Exercise literal inputs, configuration references and prior-output references. Include a small
  typed transformation implemented by the authored plugin, then consume its output in a later step
  with the dependency implied by that binding. Graph bindings must not evaluate inline expressions
  or interpolate expression-looking strings; ordinary input schemas still validate the values.
- Have two graph steps reference a configurable limit initially set to 10. Between them, use a
  supported control to confirm 15 where the fixture permits that concurrency. Both steps receive 10;
  a later execution receives 15. Verify separately that captured inputs do not bypass current access,
  dependency validity or a control's revision conflict, and that recovery revalidates retained results.
  The Interface must distinguish captured execution inputs from current confirmed instance settings.
- Pass a literal document containing reference-shaped data to a fixture operation, then pass the
  same content via an explicit reference to another operation's output. Preserve the data in both
  cases: only actual graph bindings establish dependencies, and reference resolution does not
  recursively interpret the retrieved payload. Use the accepted `literal`/`reference` form keys;
  reference payload fields remain open.
- Exercise JSON Pointer selection within explicitly chosen input, captured configuration and output
  values: nested members, array indices, escaped member names, the empty pointer and an empty-name
  member. Distinguish a numeric object key from an array index. Keep pointer-looking literal content
  unchanged. Use separate structured source and pointer fields, checking that output source selection
  establishes the correct dependency and the pointer selects within that source. Source selectors
  sit directly beside the optional pointer inside `reference`, as accepted; exact field spellings
  remain provisional. Omitting the pointer selects the whole explicitly chosen source.
- Return a schema-valid output that omits an optional endpoint, then explicitly reference that
  missing field for a consumer whose input has a default. Verify a reference-resolution error and
  no consumer dispatch, rather than a substituted null or default. Repeat with a missing array item
  and an optional receiving field. Separately select an actual null value and test both accepting and
  rejecting receiving schemas. Preserve earlier effects/outcomes and allow independent eligible
  branches to continue. Genuinely omitted inputs retain their declared default/validation rules.
- Assemble an object and a list explicitly from literals and prior outputs, including a nested
  assembly and a reference-shaped literal subtree. Verify the resulting values and dependencies
  without an assembly plugin, automatic merging or recursive interpretation of literal data.
  Require an explicit form at each input binding and each assembly child. Reject bare-value bindings;
  separately accept whole objects/lists as literal payloads without requiring forms for their data
  members. Verify that plugins receive only resolved values and ordinary configuration stays ordinary
  data. Require exactly one of `literal`, `reference`, `object` or `list` as the binding key, with its
  payload as the value. Reject empty binding objects, multiple form keys, unknown keys, and incorrect
  object/list payload shapes. Include empty assemblies and a literal payload containing all four
  form names to verify that the restriction applies to binding objects, not arbitrary data.
- Check undeclared output ports and provable type errors before any fixture steps run. Also wire an integer producer
  allowing 1–100 to an input accepting 1–10: a returned 5 permits dispatch, while 50 blocks the
  receiving step without coercion or clamping. Verify actual input validation even where early
  compatibility checks pass; preserve prior effects/outcomes when a later step's input is rejected.
  This demonstrates runtime value checking without requiring full schema-inclusion proofs.
- Return an output that violates its producer contract even though a downstream input would accept
  it. Verify that producer validation withholds it before downstream dispatch. Also create a fixture
  store and return a malformed descriptor: report the contract error, preserve the actual effect
  and any uncertainty, and reconcile through supported recovery without blind creation replay.
  Check that a valid declared failure result is distinguishable from an invalid output representation.
- Fail one branch of an explicitly requested graph while an independent branch has another eligible
  step waiting. Verify that the independent step still runs, a step with an unmet prerequisite stays
  blocked, and required unfinished setup is not reported complete. Preserve effects/outcomes without
  retrying the failed branch or rolling anything back. Current authority, coordination and stop rules
  remain prerequisites for dispatch.
- Cancel a graph with a pending ordinary step, a cancellable active child and an active child whose
  contract does not support cancellation. Verify that no new ordinary step starts, supported child
  cancellation is requested, and actual child outcomes remain tracked. Lose a cancellation response
  to prove the Interface does not claim confirmed stopping. Perform required preparation attachment
  cleanup while preserving other consumers and an already-started instance-owned fixture runtime;
  report incomplete cleanup rather than inferring rollback or automatically retiring that runtime.
- Exercise default result derivation and an explicitly constructed domain result over the same
  underlying behavior. Preserve effects/uncertainty and scoped child evidence. Fault a setup step,
  then explicitly retry the same invocation to demonstrate still-valid progress reuse and child
  recovery instead of blindly replaying effects.
- Bind declared public graph outputs using each accepted form: literal, reference, object assembly
  and list assembly. Combine a literal label with a managed reference in one fixture and verify
  resolved values reach consumers without extra assembly code or authoring wrappers. Preserve
  source scope, publication/validity and permitted exposure; do not expose unrelated internal values
  or declare whole-graph success from output availability alone. Reject malformed bindings and
  invalid public values while retaining earlier effects and operation evidence.
- Group public output declarations in a map: the database entry holds its binding and verification
  prerequisite, while the diagnostic-report entry holds its binding and explicit optionality. Reject
  duplicate public-output keys before graph execution. Reorder entries and verify that dependency
  and publication behavior is unchanged. Retain derived contracts and inherited conditions without
  requiring duplicated declarations; validate each entry under the existing output rules.
- Declare a required database output and an explicitly optional diagnostic-report output. Fail optional
  diagnostics and report the absent report's reason without that absence alone preventing success.
  Repeat with the report required by omission of optionality: the optional producer step must not
  waive that output obligation. Missing a required database output also prevents success. Verify
  that optional values still undergo validation and access checks, absent outputs become neither null
  nor fabricated data, and a consumer needing the absent result remains subject to normal binding
  rules. Preserve outstanding work and effect evidence independently of output presence.
- Bind a public database output to startup's service reference and declare verification as an
  explicit success prerequisite on that output. Withhold it while verification is unfinished,
  failed or uncertain, even though authorized setup steps can use the startup result. After confirmed
  success, enforce the output's remaining publication, validity and access conditions. Test an invalid
  or unavailable source value and an unknown prerequisite name; the gate must not bypass those checks.
  Require no extra forwarding plugin and preserve graph outcomes separately from output publication.
- Keep independent diagnostics running after verification succeeds. With the database output's value,
  declared conditions and access checks satisfied, require it to become eligible without waiting for
  diagnostics or whole-graph success. Verify that the Interface still reports unfinished graph work
  and the instance's actual preparation state. Add a stronger declared publication condition and
  require it to hold; invalidated values or revoked exposure access must still prevent publication.
- Publish a valid service reference from a step and a public graph output, then redeliver each
  recorded value under its original producing attempt. Permit redelivery without claiming exactly-once
  transport. Try publishing a different value for the same output/attempt: require a contract error,
  preserve the original value and retain any effects/uncertainty. Change the service's availability
  separately and verify that an unchanged reference does not bypass current validity or access checks.
  Where recovery supports a later producing attempt, record its result separately without rewriting
  earlier evidence or automatically replaying effects. Retained-progress reuse must still validate
  prerequisites; progress/observation updates must not mutate the published graph value.
- Compose two reusable starting requirements in a named definition and use it in Conduit templates.
  Operation allowlisting and a result-count bound are fixture candidates, not an approved permanent
  core catalog. Demonstrate constituent enforcement without requiring a generic merge/solver.
- Declare constituent requirements in a map with local keys for workspace, operations and result
  limit. Reject duplicate keys. Add another entry selecting the same result-limit contract with
  different parameters and retain both occurrences and obligations. Reorder entries and verify that
  enforcement does not depend on map order. Inspect diagnostics by local name without creating a
  runtime object per entry or conflating entries in separate compositions.
- Wire constituent requirement parameters with all four binding forms. Forward declared workspace
  and result-limit parameters, assemble child parameter objects/lists and retain literal payloads
  as data. Verify the resolved child values and schema validation without a forwarding plugin.
  Reject references to graph-only sources or ambient caller configuration. Parameter construction
  must neither issue grants nor prove fulfillment; inspect actual constituent enforcement with
  current facts, including denied requests after a relevant restriction changes.
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
scope adds no live cloud dependencies and does not authorize implementation start. Package syntax
and wire details remain I7 work; helper grouping remains implementation work. I6's accepted
capture policy applies to this fixture.

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
