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

The unsupported public management paths already named in
[the handoff](increment-3-handoff.md#current-implementation-limits-to-resolve-before-exposure)
remain core implementation work. The shell propagates their embedding errors.

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
