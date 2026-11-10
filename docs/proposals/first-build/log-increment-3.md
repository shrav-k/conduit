# Increment 3: the shell

Not binding. The [specification](../../../spec/README.md) alone binds. This log follows
[ruling 46](../ruling-46/README.md), point 4. The coordinator settles findings; the owner can
strike each settlement. A worker's implementation choices await the coordinator's review.
The increment's complete requirements remain in
[the first build](../../implementation/FIRST_BUILD.md#what-increment-3-needs).

## Findings

| ID | Class | Promises | Finding | What the build did | Settlement |
| --- | --- | --- | --- | --- | --- |
| 3-01 | contradiction | DL13, IFC-022, PRV-022 | Order 020's "no exp" conflicts with DL13's far-future expiration and [JWT-SVID section 3.3](https://github.com/spiffe/spiffe/blob/main/standards/JWT-SVID.md#33-expiration-time), which requires exp. The first round followed the order and failed to report the conflict; the verifier found it. | The correction issues exp at iat plus 100 calendar years and rejects expired tokens. | The admin settled it for far-future exp, with no change to spec/. The order's original instruction is superseded by this settlement. |
| 3-02 | contradiction | MAN-017, SCH-009 | The order asked for an optional field; MAN-017 requires a bound input to be a required scalar. | The Agent Types require a model string with an empty-string default; authors can still omit it. | The admin settled it for an empty default, no change to spec/. |

The unsupported public management paths already named in
[the handoff](increment-3-handoff.md#current-implementation-limits-to-resolve-before-exposure)
remain core implementation work. The trusted shell API propagates their embedding errors;
the consumer adapter bounds their exposure as described under I3-03 below.

## I3-01: store, loop and identities (work order 020)

[The order](../../work-orders/020-i3-01-store-loop-identity.md) supplies the decisions.
The new [shell crate](../../../reference-model/crates/shell/src/lib.rs) supplies an Executor
method for each current intent, durable transitions, wait handles for pending requests,
completion correlation, opening recovery, output expiry and identity materialization.
It starts no implementation and presents no server. Only tests supply scripted executors.

### Implementation choices

- A process-long exclusive advisory file lease prevents a second opener from recovering a
  live shell. A mutex serializes transitions within the owner. Executors and notification
  callbacks run outside the mutex and may read State or submit their completions.
- Version 1 stores State and output-completion timestamps in an atomic JSON file. Timestamps
  live outside core State so record purge cannot erase retention ages. Each expiry is a
  separate core transition and commit. Size counts the compact UTF-8 retained outputs/content
  JSON, including overhead; age is checked on open and at transition boundaries, without an
  idle timer. Equal completion times sort by invocation ID.
  Store decoding allows document depth plus State's wrappers, rather than imposing serde's
  default 128-level JSON limit on the serialized State. Authored input still goes through
  the strict loader's limit.
- Evidence starts with 256 UUIDs/revisions/suffix candidates and retries exhaustion once with
  16,384, from unchanged committed State. First start gets its required seven revisions and
  two suffixes. Time is UTC from the system clock. UUIDs and suffix entropy come from the OS.
- One private authority file atomically retains the engine trust domain, PKCS#8 signing key
  and same-key CA certificate. Per-run materialization uses the required-name arrays from
  run.v1, preserving their role order, and refuses descriptors that differ. Public JWKS uses
  key ID store. Token audience is an array containing the configured full engine address.
  Tokens have iat, a fresh UUID jti and exp at iat plus 100 calendar years, as settled
  for finding 3-01. No credential bytes
  go back into a core input. RunIdentity has no Debug or Serialize implementation.
- Existing store directories/files must already have the required private modes; broader
  permissions are refused. The store's parent must exist. A missing authority in an
  established store is an error, rather than a new identity. No distributed/network
  filesystem locking or durability claim is made.
- Unserved calls report missing, inspection reports unknown, and unserved run/connection
  work reports failed to the core. Cancel intents acknowledge the absent transport because
  the core has already ended them and defines no cancellation completion input. Unknown
  kinds and mismatched executor responses stop with an embedding error.
- The CLI registers ordinary directories with the S1 snapshots preinstalled, then explicitly
  requests opening root's submitted Conduits and setup in Pool/Resource/Agent/Interface order.
  It pauses on the first built-in runner intent and prints only its kind, operation and
  invocation. No pending value is printed as a successful result. Inspection uses core
  inspect.v1 calls as root, preserving the core's authority refusals. Record export is JSON
  lines in source order, with gaps reported on stderr.
  First start supplies the three mandatory machine labels under PRV-014, mapping Rust's
  platform names to GOOS/GOARCH, and no custom label unless given. The order's "no labels"
  is read as no custom labels: omitting the mandatory labels is rejected by the core.
- The core exposes Transition.request_replies, containing its own final replies correlated
  by invocation and attempt, including filter refusals that remove an invocation or restore
  its previous result. The shell delivers them unchanged after committing. This replaces the
  first round's requests::finished_reply accessor, which could reconstruct an earlier success
  or lose a refusal. No access, repeat or lifecycle decision changes.
- Output accounting traverses previous snapshots of pending repeats. A new successful core
  completion starts a retention age; moving or restoring an existing result preserves it.
  The core's expire-outputs now removes retained results wherever the invocation holds them,
  including previous snapshots, under OPS-034 and DL10; invocation state is preserved.

### Evidence and limits

[Shell tests](../../../reference-model/crates/shell/src/tests.rs) exercise the S1 vector through
the store and scripted completions, source-order record export, concurrent requests and
pending waits, missing executors, output cap/age, store format/permissions/ownership, credential
verification and containment, and killed child processes on each side of dispatch. The S1
test supplies the vector's deterministic evidence only through a test-only facility; the
production entry point always generates evidence at the boundary.
Additional tests exercise larger evidence on allocation exhaustion, an unchanged commit
after both attempts exhaust, and store round trips with document depth plus State wrappers.
The correction round adds ticket-level regressions for a first-attempt filter refusal and
refusal of a repeat after success, cap enforcement while a previous result is held by a pending
repeat, and original retention age after restoration and during a pending repeat. Identity
checks require the far-future exp and refuse signed tokens with expired, missing or malformed
expiration claims.
The command-line test runs init twice, registers S1 until the runner wait, inspects and
compares exported JSON lines to the retained records. Identity checks independently verify
the JWT through its JWK and the root certificate's self-signature with that same public key.

The CLI's paused registration is deliberately left in progress until recovery or a runner
completion; another command opens the store and first recovers it unknown. Orders 021 and
022 supply persistent runner/server ownership. No real Agent, process runner, remote gate,
HTTP authentication, OAuth server or multi-machine restart is demonstrated here.
The implemented DL2, DL10 and DL13 choices are in
[CONFORMANCE.md](../../../reference-model/CONFORMANCE.md).

## I3-02: the engine's own runner (work order 021)

[The order](../../work-orders/021-i3-02-runner.md) supplies the decisions. The
[runner](../../../reference-model/crates/shell/src/runner.rs) serves built-in inspection and
run, dispatch, refresh and both cancellation intents. Deferred tasks hold no shell transition
lock while preparing packages, initializing MCP or waiting for results. They reenter through
setup_report or complete, with the original invocation and attempt. No core decision or
fixture changed, and no new language finding was needed.

### Implementation choices

- Runner owns a two-thread Tokio runtime and one rmcp 3.9.0 client session per run. Workers
  hold a weak shell reference during I/O. Either owner's drop ends every run. A small internal
  mode of the same conduit executable leads a fresh process group, starts the program without
  a shell, and watches an engine-only lifetime socket. EOF kills that group, including after
  SIGKILL of the engine. It reaps its direct child and returns helper exit status through the
  socket in the opposite direction. The engine also kills groups on shutdown and setup
  cancellation. Runner shutdown shares the spawn lock, preventing a deferred start from
  registering a new group just after shutdown's scan. No automatic restart or autonomous
  inspection is added; an empty run table
  after reopening reports stopped. This is Unix process-group lifetime management, with no
  claim of confinement against a same-user process that deliberately escapes its group.
- Each run starts in a fresh empty store/work directory, mode 0700, apart from both the held
  shared root and its identity directory. Only permitted inherited names are read. Package
  environment declarations supply fixed values and defaults over inheritance; bound
  environment and secrets override them, and identity wins last. Package arguments and
  environment values substitute declared server.json variables once, preserving unresolved
  identifiers. Fixed positional values need no valueHint. Helpers receive only
  permitted inherited environment, with neither bound secrets nor run identity. The guard
  reconstructs the explicit environment at the second exec too. A direct /usr/bin/env test
  checks that boundary; Apple's Rust test harness separately adds __CF_USER_TEXT_ENCODING
  within a scripted test process, which is excluded from that harness's inherited-name test.
- npm pack verifies the fixture's artifact hash before any decompression, dependency install
  or execution. The store holds a hash-keyed unpacked cache and a separate npm download
  cache. An incomplete cache is rebuilt; only successful installation marks it ready. The
  runner invokes node on the package's sole bin. Other registry types and non-stdio Package
  transports fail. Installed relative commands are written under store/programs with their
  file modes and links, matching the held version and digest rather than the first package
  with that address. Program traversal preserves symlinks and parent components while requiring
  the final file to stay in those installed files; expanded old paths are never substituted
  for missing retained files.
  Absolute programs run as given; bare names use the permitted PATH, as MAN-010 specifies.
- npm gets a private empty user configuration, no global configuration, the public npm
  registry and disabled audit/fund requests. It never consults the owner's npm credentials.
  Installation keeps --omit=dev and --ignore-scripts and also uses --legacy-peer-deps:
  npm 10.2.4 failed while resolving the unchanged package's peers with "Cannot read properties
  of null (reading 'edgesOut')"; that extra flag installed its runtime dependencies and the
  unchanged server passed S1. This is the nearest sound reading of decision 2 on this machine;
  peer dependencies are not automatically installed. The artifact hash pins no dependency
  graph. No other npm package or Node distribution was fetched to repair the local tools.
- Initialization has a configurable 30-second deadline; dispatch and refresh-plus-call have a
  configurable 120-second deadline. npm helpers have a fixed 120 seconds and at most another
  5 seconds for exit status. Initialization failure kills its group before reporting failed;
  a missing process or call timeout returns missing through the core. Late results cannot
  settle unknown. Diagnostics retain fixed messages and registry names or exit codes, never
  child stderr, credentials, environment values or argument values.
- rmcp adds client and progress metadata automatically. A transport extension restores the
  exact supplied method and params at the byte boundary, retaining the core's metadata for
  an aware implementation and its absence for a plain implementation. It preserves raw
  result or protocol-error code/data for the completion instead of reconstructing content
  through SDK defaults. No Roots capability is declared. Refresh performs tools/list once
  and the supplied safe read once; only the core decides eligibility. Cancellation uses the
  actual request ID for that invocation and attempt, including cancellation racing send.
- CLI registration waits for real setup and prints only final core replies. It owns its runs
  for the command's lifetime, then stops them; persistent serving belongs to order 022.
  Config and CLI expose initialization/call deadlines and the initially empty environment
  pass list. Provider-call and connect remain unserved and use order 020's missing/failed
  paths. No complete public retirement/stop-run API, runtime provider, remote connection,
  consumer HTTP server, Conduit extension discovery or Agent client is claimed here.

### Evidence and limits

The [runner tests](../../../reference-model/crates/shell/tests/runner.rs) cover exact plain
pipe params, no Roots capability, environment names and identity precedence, synthetic engine
GITHUB_TOKEN exclusion, empty private working directories, initialization/call deadlines,
unasked exit and stopped inspection, one read repair and no write repair, cancellation by
request ID with a late result, setup cancellation, shell drop, runner drop during
initialization while the shell remains open, and SIGKILL of an engine with
a run and a descendant. Unit tests also check exact exec environment, Package positional and
named arguments without runtimeArguments, and installed program bytes/modes in place of an
old expanded path. These are shell tests, not additions to the conformance vector catalog.

On 2026-11-10, the opt-in real S1 test passed against the unchanged pinned
@modelcontextprotocol/server-filesystem 2.1.0 artifact. It initialized two sessions, read and
wrote project bytes as B through the core, wrote history bytes to the separate directory,
and observed the server refuse an outside-root path. For each request, its access-decision
record precedes invocation-dispatched and its one invocation-ended record. The independent
hash-mismatch test passed before extraction. The available tools were Node 21.5.0 and npm
10.2.4; the package declares Node >=24. The runner does not check a Package's declared
engines, so this evidence qualifies the exercised operations on this machine, not its whole
declared platform range. No other Node was installed. Default tests fetch nothing.
The CLI's opt-in test also passed: registration reached ready with no pending invocation.
The required complete checks are reported by the worker after running them; the implemented
DL10 and DL16 choices and remaining interface limits are in
[CONFORMANCE.md](../../../reference-model/CONFORMANCE.md).

### After the verifier's review (work order 021, second round)

The admin accepted all five verifier findings as runner defects. No specification change
was needed. The runner now distinguishes bare program names from package-relative paths,
preserves parent components in validated installed symlink targets, accepts fixed positional
arguments without valueHint, composes Package environment values/defaults with supplied
bindings, and substitutes declared argument variables as
[server.json specifies](https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json).
Secret bindings and identity retain their precedence; unknown or unfilled identifiers remain
in braces, and inserted values are not substituted again.

Five regression tests execute the resulting programs, arguments or environment at the real
exec boundary: an installed bare command with a same-named package file uses only its permitted
PATH; a relative command follows bin/worker -> ../lib/worker, including parent traversal after
another symlink; fixed positional values need no label; Package fixed/default environment
values reach exec with binding/secret/identity precedence; and fixed argument variables resolve
while unresolved braces survive. Installed programs still undergo the final canonical
containment check, and archive admission's link validation is unchanged.

The admin extended scope to tools/set_names.py. Its smallest exception is the exact build-test
switch CONDUIT_TEST_NPM; other unknown CONDUIT_ names, including identity misspellings, still
fail. Tests and the README now spell the switch plainly, without concatenation. The
child-engine fixture still uses WO021_ENGINE_TEST_PARENT. The declared Node >=24 versus
observed Node 21.5.0 qualification limit is also explicit in CONFORMANCE.md's DL16 package row.
The full order checks, including the opt-in real npm suite, are rerun for this correction.
Order 022's presented server remains outside this tree and this order's work.

## I3-03: the consumer server (work order 022)

[Order 022](../../work-orders/022-i3-03-presented-server.md) supplies the decisions.
The [server adapter](../../../reference-model/crates/shell/src/server.rs) serves MCP over
loopback with rmcp 3.9.0 and axum. This order changes no core decision or specification promise.
The existing H03 management limits are implementation work, not new language findings.

### Implementation choices

- A private `engine-url.json` fixes the full endpoint before tokens are issued. Every later
  open compares it with Config; client commands read it without taking the store lease.
  `/mcp`, HTTP loopback and a nonzero port are required. `localhost` binds IPv4 loopback.
- Every HTTP request verifies its Bearer token against current State. The adapter accepts
  only its own exact Origin, binds legacy sessions to consumers, and never uses clientInfo
  as identity. Readable unidentified calls are recorded as refused through identity none;
  authenticated transport refusals use the core's readable-false input for invalid records.
  Error bodies wait for that commit. The body limit is 4 MiB.
- The core's tool publication is preserved with private cache scope and no shell-set ttlMs.
  For 2026-07-28 rmcp supplies the protocol-required zero freshness default; legacy results
  omit it. This is the nearest sound reading of decision 3, rather than a chosen cache lifetime.
  The server declares listChanged and the implementation-interface extension. A decorating
  Executor routes each committed transition's tool_list_changes to the matching consumer's
  legacy sessions and modern subscriptions. Its broadcast queue holds 1,024 notices; a
  lagged receiver closes. Notification delivery says nothing about a client's model catalog.
- Blocking core/store work runs outside the async transport workers. A pending call checks
  its internal ticket every 10 ms, with no additional deadline, and returns only the core's
  final result/error after commit. A guard handles cancellation during begin or handler
  teardown; each response body's lifetime token also reaches its handler, so stream closure
  commits cancel for that invocation. Client cancellation does so too. Shutdown also commits
  recovery for interrupted work.
- Root receives a private owner helper token at initialization/serving. Materialized instances
  also receive helper tokens in the store's 0700 client-tokens directory, files 0600.
  `call` uses root and `tools --as` uses the named instance through ordinary MCP requests.
  Neither command opens the writer lease or performs a direct core invocation.
- Startup compares core publication with the handoff's dispatcher inventory. Unsupported
  nonmanagement exposure stops startup. Management-only unfinished operations stay visible
  and return -32603 naming the operation with no transition committed. The explicit inventory
  and public invocation limits are in
  [CONFORMANCE.md](../../../reference-model/CONFORMANCE.md#consumer-server-order-022).

### Evidence and limits

The [HTTP tests](../../../reference-model/crates/shell/src/server_tests.rs) connect rmcp
clients as A and B using materialized run-directory tokens. They compare standalone tool
schemas and annotations with core publication, exercise A's refused write and B's write/read,
read durable request/invocation endings before accepting replies, and exercise pending
completion and cancellation. T7 uses the management MCP client to close A's directory Conduit,
replace it explicitly and set up A again; B's list and notification stream stay independent.
Authentication cases include missing, forged, foreign-audience and absent-instance tokens,
Origin rejection, malformed requests and keyed repeats. Both legacy notifications and modern
subscriptions are covered. The CLI integration test starts the actual conduit serve process,
uses tools/call as root, rejects an address change and stops it with SIGTERM.

The checkout supplied to this worker has order 020's shell and paused registration; it has no
runner module or dependencies from order 021, and no record of that order's implementation.
This is an absent prerequisite, not a finding against the language. The CLI explicitly refuses
setup or runnable implementations needing it. Scripted S1 HTTP coverage runs the library
server, not a conduit serve subprocess with a real filesystem executor. No real filesystem,
npm server, Agent or runner shutdown is demonstrated. CONDUIT_TEST_NPM runs the available
suite only; the real-filesystem tests of order 021 are absent. Runner integration and real S1
process coverage remain for the coordinator; the complete order 022 done-when is not claimed.

Validation on 2026-11-10: both `cargo test --release` and the same command with
`CONDUIT_TEST_NPM=1` passed all 108 available Rust tests. After the final missing-runner guard
was corrected to recognize ready instances' package/command items, both CLI integration tests
were rerun and passed, including a ready-S1 snapshot that must refuse to serve without the
runner. Clippy with warnings denied, fmt, the four Python repository checks and whitespace
checks passed. These runs add no real npm/filesystem evidence for the absent order 021.

### After the verifier's review (work order 022, correction round)

The admin accepted all three implementation faults reported by the fresh verifier. They
require no specification change:

- The checkpoint's legacy session transport did not connect a closed HTTP response stream
  to handler cancellation. Its documentation claimed more than the explicit notification
  test showed. Each response now owns a lifetime token carried in the request extensions;
  dropping its body signals only that handler, which submits core cancel. Regressions close
  raw HTTP connections without sending notifications/cancelled for both supported legacy
  revisions and the modern revision. They require executor cancellation, durable unknown
  invocation/request endings and an independent second call still pending.
- The unfinished inventory skipped nonengine implementation sources and therefore custom
  controls. It now inspects every actually published operation's contract in its held
  environment, using the exact scope in the publisher's decision. A contract with changes
  is unfinished regardless of type/template source. Regressions use a held custom control
  with a conflicting other scope, reject nonmanagement exposure and check that management
  still lists it but receives -32603 naming the operation, with the store unchanged.
- A method string alone no longer makes an unidentified tools/call readable. The adapter
  requires rmcp's typed CallToolRequest variant. Malformed requests continue through rmcp's
  own rejection path without invoking the core or creating a record (IFC-036). Regressions
  exercise a missing name, nonobject params and nonobject arguments, with absent or forged
  proof, across all supported revisions. rmcp treats malformed object parameters as custom
  requests and answers method-not-found; nonobject params fail its transport decoder.

Order 021 remains absent and untouched. These corrections add no runner or real filesystem
evidence and do not remove the integration limit above.

An initial full release run passed all 111 available Rust tests. The first npm-gated run
failed the new stream-close regression's 10-second cancellation wait; the other 20 shell
tests passed. The focused stream regression and the full release run had passed that case.
The regression now uses the suite's existing 60-second wait limit for observing cancellation,
without adding an invocation deadline to the server. Final full-suite runs set
RUST_TEST_THREADS=4 to reduce test contention; no tests are excluded.

Final correction-round validation on 2026-11-10: both complete release suites, with and
without CONDUIT_TEST_NPM=1, passed all 111 available tests, including all three correction
regressions and both CLI integration tests. Clippy with warnings denied, formatting, the
four Python repository checks and whitespace checks passed. The npm-gated command still
supplies no real npm/filesystem evidence in this checkout without order 021.

### Joined to order 021 (work order 022, merge round)

The admin merged main, including order 021 and its correction round, into order 022. This
round resolves all six conflict files without staging or changing Git history. Both orders'
log sections and conformance choices remain; neither introduced a duplicate finding or
numbered limit, so no renumbering was needed. The sole finding remains 3-01. Cargo regenerated
the lockfile from the union of dependencies, including stdio and HTTP features on rmcp 3.9.0.
The earlier absent-runner limits above describe those rounds and are superseded by this join.

`conduit serve` now owns the real runner. Before listening it explicitly sets up registered
S1 instances, and inspects ready instances with executable items so setup restarts runs that
ended with the registration command. The shared registration/setup helper keeps the original
root and owning-Pool management routes; it fails startup on a failed setup reply. The
absent-runner refusal and its obsolete CLI regression are removed. Signals are handled during
startup too; interrupted tickets are recovered and every process group ends before exit.
The runner remains owned on the synchronous entry-point thread, since its Tokio runtime must
be dropped there. The serving executor holds only an adapter to its inner executor, and a
weak completion-executor reference makes deferred setup/dispatch completions deliver tool-list
changes through the same serving decorator, without an ownership cycle.

The runner writes private historical process-group diagnostics under store/run-groups, with
only engine PID and group ID. The runner never uses them for recovery, inspection or termination:
persisted PIDs may have been reused. Working directories remain empty at launch. The process
test reads these IDs and checks the OS groups directly before and after stopping serve;
this avoids the sandbox's refusal to execute ps. No credentials or child arguments are in
those diagnostic files.

The new [process test](../../../reference-model/crates/shell/tests/serve.rs), gated by
CONDUIT_TEST_NPM, begins with registered S1 and outstanding setup. The real serve process
initializes both unchanged filesystem servers. rmcp consumers use the run-directory tokens
for A and B, with exactly five and six tools respectively. Base S1 Agents have no executable;
the test materializes their identities with the same run-contract API the runner uses, and
starts no models. A's write is refused; B writes and reads actual disk bytes. Missing and
forged tokens receive 401 with WWW-Authenticate, leave the bytes intact and have durable
refused request records. Each successful filesystem reply already has one invocation-ended
and one request-ended record, with access decided before dispatch. T7 closes A's project
Conduit through conduit call; A loses its project tools and its cached read is refused.
B's tools stay identical, it writes while A is closed, and receives no notification. Explicit
register-replacement followed by setup, both through conduit call, restores A's list and read
access; B continues writing. SIGTERM ends serve successfully, leaves no in-progress invocation,
and both observed filesystem process groups disappear.

During development the focused process test first exposed startup trying to set up ready
foundational Resources through a nonexistent owning-Pool route; ready-instance inspection is
now limited to executable items. The next attempt hit the sandbox's ps restriction. A first
version of the diagnostics also included already-ended npm helper groups; diagnostics now
identify actual run groups only. These were shell/test fixes, not language findings.

The same process test also reopens the ready store, proves B can read through fresh runner
sessions, and checks both new process groups disappear after a second SIGTERM. The focused
run passed on 2026-11-10. Two offline regressions passed too: deferred setup and dispatch
transitions reach the configured completion decorator; and SIGTERM during an unanswered
initialization cancels setup, ends that process and starts no later Resource. The startup
worker observes cancellation while waiting for its ticket and is joined before runner drop,
so it cannot race shutdown by submitting a later part. The startup-signal regression initially
waited for shared when history was the first Resource; it now observes whichever starts first
and requires the other not to start. No Resource order was changed.

Merge-round validation on 2026-11-10: the complete offline release suite passed 131 tests,
with two ignored child-process fixtures exercised by their parent tests. The focused real
S1 process test passed its initial and ready-store starts, both clean stops, and every T7
assertion. The focused completion-decorator and startup-signal regressions passed. Clippy
with warnings denied, fmt, the four Python repository checks, git diff --check and an explicit
scan of all eleven changed files for trailing whitespace and conflict markers passed.

The complete CONDUIT_TEST_NPM=1 release suite also passed 131 tests, with the same two
child-process fixtures ignored as standalone tests and exercised by their parents. This run
activated the real npm registration, artifact-mismatch, runner S1 and serve S1 cases; the
serve case passed T7, ready-store restart and both no-surviving-group checks. All checks used
RUST_TEST_THREADS=4 for complete release suites. A test-only Conduit lookup was also corrected
to read consumer reference maps as well as bare IDs. The final report lists each check's final
result; no specification, core decision or conformance vector was changed in this round.

### Second fresh verification of the join (work order 022, correction round)

The admin accepted all four findings from the second fresh verifier of the joined orders
021 and 022. These corrections change only the shell and its documentation:

- A tools/call authenticated as B but carrying A's session ID now commits the core's
  readable-false rejection before returning HTTP 401 with WWW-Authenticate. Its regression
  reads the durable request-ended record as B and checks that nothing was dispatched.
- Rejection inputs carry each syntactically valid nonempty serving and idempotency metadata
  string independently, including when the other field is malformed. Attribution and record
  construction remain in the core. Regressions cover rejected Origin, missing extension
  capability and malformed metadata, and an onward rejection whose consumer differs from
  its original caller; both access-decision and request-ended records are read from disk.
- A legacy notification receiver that overflows now closes and removes its session through
  rmcp's session manager, ending its HTTP streams. The adapter shares the ID assigned after
  initialization with that session's forwarding task. A deterministic regression overflows
  the 1,024-notice queue for each legacy revision, observes the stream end and 404 for the
  old session, and receives a notification after reconnecting. Modern overflow already ends
  its subscription stream. CONFORMANCE.md specifies both forms of closure and reconnect.
- First start accepts only the HTTP layer's supported hosts, 127.0.0.1, localhost and ::1,
  before fixing the store's token audience. A regression refuses other loopback hosts,
  verifies no audience was persisted and accepts each supported host.

No new language finding or conformance vector is introduced. No Git index or history change
is part of this round.

Validation on 2026-11-10: all eight focused HTTP tests passed. Both complete release suites,
with and without CONDUIT_TEST_NPM=1, passed all 134 tests; two child-process fixtures were
ignored as standalone tests and exercised by their parents. Each full suite used
RUST_TEST_THREADS=4. The npm-enabled suite passed real registration, artifact-mismatch
rejection, runner S1 and the serve-process S1 test, including T7, ready-store restart and
shutdown. Clippy with warnings denied, formatting, all four Python repository checks,
git diff --check and an explicit whitespace/conflict-marker scan of the five changed files
passed. The index and history were left unchanged.

## I3-04: the Agent packages (work order 023)

The two packages are independent Rust crates under packages/agent-claude-code and
packages/agent-codex: no outside path dependencies, their own lockfiles and pinned toolchains,
and copied adapter/test/packaging code. Each exports one Agent Type, a conversation capability
and turn.v1. The plain implementation maps that operation to the wrapper's sole turn tool.
It binds the two reach inputs and model through environment JSON Pointers and records neither
inputs nor outputs. The wrapper starts the native client only when an authorized turn arrives.
Claude uses one streamed process; Codex resumes its exact thread per process/turn. It writes
Claude configuration into the run's private directory, passes only the six permitted ambient
variables and the two identity variables, removes both API-key variables, and reads the run
token only through its header-helper mode. Raw streams/stderr are not retained as logs.

The client's own startup tool names are reported in clientTools with source event positions.
An absent list is explicitly not_reported, including later Claude turns without an init
event and the qualified Codex baseline's lack of a startup list. Repeated session/thread init
lists on later turns are unavailable with session_init_unverified freshness; they do not
establish a current catalog. This does not replace the
engine list and does not claim notification delivery proves model catalog refresh. The
[closeout plan](increment-3/closeout.md) names each remaining real-client need and its evidence;
the worker did not run either real client or fetch anything except crates.

### Adjustments to the order, not new language decisions

- **Model binding (MAN-017, SCH-009).** The first attempted registration was rejected: an
  environment pointer to optional model is forbidden by MAN-017 even with a default. The
  nearest sound reading makes model required in the Type schema with an empty-string default.
  Authors can still omit it: the core fills the default. The live registration vector and
  scripted CLI registration exercise that omission. No specification/core change is proposed.
- **Host pins (MAN-010, PKG-004/021/026).** A runnable archive includes a host binary and
  cannot have one portable digest. Live S1 in the tree pins the metadata-only fixture archives
  used by its registration vector. The documented live preparation copies the project and
  replaces those two pins with the generated host hashes. The CLI never silently changes a
  submitted manifest. Its --package option explicitly installs via the existing semantic
  archive API, not the unfinished install.v1 public invocation.
- **Codex peers.** The qualified Codex launch exposes no separate peer-messaging switch.
  peerMessaging is accepted/bound for the common input contract and changes no Codex argument.
  Claude's false/true settings are tested as emitted configuration. True selects/allows only
  the three peer tools and removes the explicit inbound refusal, using the native default;
  no guessed inbound enum value is written. This is the nearest sound reading of --tools
  empty plus a working peer opt-in, while machine tools remain unselected. Real message
  refusal, the inbound default and effective opt-in remain closeout work.

These are limits and an order/spec constraint, not cases the core could not decide. The
admin accepted the model settlement as finding 3-02 above. An omitted native catalog cannot
be repaired by inventing a language rule or recording the engine's own list as the client's
observation.

### Packaging and evidence

Each package's cargo run --release --bin package builds its host wrapper and writes sorted
USTAR/gzip with zero owners/times, executable mode 0755 and YAML mode 0644, plus SHA-256.
The conformance/fixtures/agents copies are metadata pins only and say where they come from.
The single new registration-live-agents vector registers live S1 against those copies,
including authorizing turn Conduits; setup/model work is absent from that vector.

The shell accepts repeated --package ARCHIVE#SHA256 arguments before registration. A test
uses unmodified host archives through that CLI and rejects incorrect digests without changing
state. It then installs explicit scripted-client variants, starts S1's wrappers and a
scripted filesystem, and calls both Agents through conduit call on the real loopback server.
The filesystem/client substitutions are test evidence, not real npm or model evidence.
Each package tests all four reach combinations, exact arguments/environment, its helper's
single header output, no client startup at MCP initialization/listing, one conversation and
failure poisoning. Packaging tests check reproducible bytes and archive metadata/digests.

Mediated restrictions remain explicit (ACC-027): Codex's read-only shell can read beyond the
engine, the OS user can reach other run credentials, and both Agents share the owner's login.
The defaults request connectors/apps/peer reach off; true opts in per Agent in emitted
configuration, with the Codex peer limitation above. Effective native behavior and account
identity still need the admin's sentinel-backed closeout. This order claims no completed
increment-3 closeout or strong machine confinement.

Validation on 2026-11-10: the full reference-model release suite passed 135 tests, with three
standalone child-process fixtures ignored and exercised by their parent tests. The focused
scripted S1 integration passed again after the final assertions; both Agent calls reached the
engine under their own identities and neither turn payload was recorded. Each standalone
package passed four tests (one packaging test and three wrapper tests). All three clippy runs
with warnings denied and all three formatting checks passed. Both standalone cargo packaging
commands completed, and the single new registration vector passed. The four Python repository
checks, git diff --check and a scan of all changed text files for trailing whitespace and
conflict markers passed. The default suite did not activate npm-gated tests; no npm artifact
or real native client was fetched or run for this order. No files were staged or committed.

### Fresh verification corrections (work order 023, second round)

The admin accepted all six verifier findings, including both marked should. The corrections
and regressions are:

- CLI package admission recognizes conduit.yaml, conduit.yml and conduit.json, parsing each
  syntax correctly. Metadata discovery resolves hard links and symbolic links with the core's
  virtual unpack semantics. Since those helpers are private and the core is outside this
  order's scope, the shell copies them for discovery; install-archive still owns admission.
  CLI regressions install both alternate filenames and both linked manifest forms.
- Codex receives each prompt verbatim through stdin and the fixed dash prompt argument.
  The scripted client now implements that convention; regressions cover literal dash and
  option-like prompts on initial and resumed turns.
- Repeated session/thread init catalogs are observations of unverified freshness, reported
  as unavailable on later turns, even if their names differ. First-turn startup catalogs
  remain reported; absent lists remain not_reported. Observations retain raw reportedNames
  separately from engine names. The standalone Claude fixture copies the recorded events
  of experiment 009, attempt-3/claude-r2, including its stale second init, refused file call
  and later notes call. That regression does not claim a current second-turn list.
- Both MCP handlers observe cancellation while queued and again before launch. Active
  cancellation ends/reaps the client and poisons the session. Each package's MCP regression
  holds one turn, cancels a second queued turn, releases the first, and proves the queued
  client work never launched; it also cancels active work and checks implicit restart refusal.
- The S1 integration obtains scripted-client paths from cargo metadata's target_directory,
  respecting CARGO_TARGET_DIR. It is rerun below with a custom target directory.

The freshness output fields are declared in both package contracts; their pinned YAML
copies, metadata archives, registration fixture and live S1 hashes were refreshed together.
The first integration run exposed this missing output-schema update, which was corrected
before validation. The CLI regression initially reused the preinstalled filesystem address
with changed bytes and was correctly refused; it now uses a distinct test package address.

The model contradiction is now numbered 3-02, as the admin requested. No specification or
core decision changed. No Git index or history mutation was performed.

Correction-round validation on 2026-11-10: the complete reference-model release suite,
with RUST_TEST_THREADS=1, passed 136 tests, with 3 standalone child fixtures ignored and
exercised by their parents. Each independent package passed six tests, including its
packaging test and the new regressions. The S1 integration passed with both default package
target directories and an explicit CARGO_TARGET_DIR pointing to reference-model/target;
the latter checks each client's actual executable path as well as both distinct identities.
The registration-live-agents vector passed with the refreshed metadata pins. Both standalone
packaging commands, all three clippy runs with warnings denied and formatting checks, the
four Python checks, git diff --check, fixture provenance/copy comparisons and the explicit
scan of all changed text files passed. CONDUIT_TEST_NPM was absent; no npm-dependent test
or real native-client closeout was activated.

An earlier parallel full-suite attempt and a diagnostic duplicate were interrupted before
a verdict; neither is reported as passing. The live serial log shows the existing tests
progressing slowly, not a demonstrated deadlock. No source workaround was introduced for
that run. The staged diff stayed empty and HEAD stayed at the admin's checkpoint.
