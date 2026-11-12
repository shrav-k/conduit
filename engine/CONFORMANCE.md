# First-build conformance choices

Not binding. The [specification](../spec/README.md) alone binds. This document states the
choices implemented through increment 2 and shell orders 020/021/022, under the admin's
[decisions](../docs/proposals/first-build/increment-2/decisions.md) and
[order 020](../docs/implementation/WORK_ORDERS.md#order-020), and distinguishes them
from choices the shell must still make. It is not a declaration of whole-language conformance.
The executable [profile](../conformance/profiles/first-build.yaml) and
[coverage manifest](../conformance/manifest.yaml) name the evidence. A vector demonstrates
its stated slice of a promise; a coverage reason is not a passing vector.

## Implemented choices

| DL item | First build's choice | Where it is shown |
| --- | --- | --- |
| DL2: contents | SHA-256 of RFC 8785 JSON holding the qualified operation and default-filled arguments; caller/sender/target/key scope is separate. Invocation state, held contexts and retained results are independent of records. | `requests-key-scope`, `requests-defaults`, `records-purge`, [requests.rs](crates/core/src/requests.rs) |
| DL2: dispatch boundary | `step` returns a proposed state, appended records and intents atomically. The shell commits before executing any intent or delivering a final reply. Executors run outside the transition mutex; their completions reenter it. Pending replies wait inside the shell. | `s1-whole`, `requests-restart-interrupted`, [shell tests](crates/shell/src/tests.rs) |
| DL2: storage | Format version 1: the whole core State, including records, in one JSON file. Each transition reads the latest commit under a mutex, writes a new file in the store directory, fsyncs it, renames it over the old file and fsyncs the directory. A process lease excludes a second shell, including recovery while another shell is alive. Store/run directories are 0700, files 0600; existing broader modes and unknown formats are refused. The store's parent must exist. | [store.rs](crates/shell/src/store.rs), killed-child tests before and after send |
| DL2: attempts and late results | Attempts start at 1 and increase on an eligible caller repeat. Completion identifies both invocation and attempt. A stale attempt or ended invocation answers `ignored`; a late response cannot settle unknown. A parameter-header repair is once within the same attempt, only for IFC-046's read-only case, with no general effectful resend. | `requests-unknown-write`, `requests-repair-read-success`, `setup-close-in-flight`, [requests.rs](crates/core/src/requests.rs) |
| DL3: waiting, implemented portion | There is no autonomous clock or actor-reply timeout in the core. Pending lasts until supplied completion, cancellation or recovery. Duplicate/late completion is ignored. No actor-reply provider has been qualified. | `requests-key-scope`, `requests-unknown-write`, `requests-cancellation`, [requests.rs](crates/core/src/requests.rs) |
| DL4 | An unreadable identity/authority or state needed to decide access yields unavailable immediately; no waiting queue. | `access-identity`, `access-current-tools`, `records-authority` |
| DL5: reports, implemented portion | Time, canonical hostname, labels and run reports are supplied values. First start keeps the supplied local machine report. Setup asks for run inspection and interprets running/stopped/unknown; observation does not revise the definition. No polling cadence is chosen here. | `first-start`, `setup-run-unknown`, `setup-review-conditions-builtin`, `setup-review-conditions-provider` |
| DL6 | First start is one atomic empty-store transition. A repeated first start preserves identities and creates nothing. Allocation exhaustion/reuse is an embedding error and leaves the input state intact. Recovery ends interrupted work unknown without reissuing it; no repair is inferred. | `first-start`, `first-start-repeat`, `requests-restart-interrupted`, [boundary tests](crates/conformance/tests/core_boundary.rs) |
| DL7: archives | Gzip-compressed POSIX USTAR, without GNU/PAX extensions. UTF-8 paths; files, directories, symbolic links and hard links to earlier regular files. At most 40 link expansions. Last member at a path wins except replacing a nonempty directory or using a non-directory parent. All paths/targets are checked, including ignored documents. | `archives-link-expansions-39`, `archives-link-expansions-40`, `archives-link-expansions-41`, `archives-hardlink`, `archives-symlink`, [archive fixtures](../conformance/fixtures/install-archives/README.md), [archives.rs](crates/core/src/archives.rs) |
| DL7: storage and submission | Installed packages retain a virtual directory with bytes/modes in `State.installed_packages`; no compressed cache, removal or mirror API is implemented. Directory URL and fetched bytes enter at the boundary. The runner supplies directory documents for an established actor; registration installs nothing. A preinstalled snapshot explicitly says so. | `archives-program`, `archives-s1`, `packages-relative-fetch-directory`, `registration-s1`, [packages.rs](crates/core/src/packages.rs) |
| DL8: records | One ordered CloudEvents source, `urn:conduit:first-build`. Six event types: access-decision, request-ended, invocation-dispatched, invocation-ended, provider-call, object-change. Object-change actions and provider made/ended phases are in the profile. Export explicitly returns ordered records, including records for removed objects. No live record-feed cursor is implemented. | `first-start`, `records-inspection`, `records-unreadable`, `setup-review-provider-records`, [records.rs](crates/core/src/records.rs) |
| DL8: inspection and purge | Inspect keeps id/revision/packages/declared/status/records separate, omits template status, and reports unresolved invocations separately. Gap metadata is conservative across all known source gaps. Purge removes records strictly before its instant and retains permanent source/ID certificates. Export uses gaps/complete; inspection uses recordGaps/recordsComplete. Purge never alters the invocation index or retained results. | `records-inspection`, `records-purge`, `records-purge-succeeded`, `records-input-reference`, `records-unreadable` |
| DL9 | There is no extra portable ordering rule among diagnostics of the same reason. The implementation returns its deterministic traversal order. OPS-023's ordering between different reasons still binds. Vectors compare the required rule/location facts rather than validator prose. | `registration-s1-rejected-orders`, `access-routing`, `loader-schema-attribution`, [case assertions](../conformance/README.md#assertions) |
| DL10: structural limits | 128 levels of nesting (portable floor 64). No stated byte limit for files or archives, or count limit for documents/files in a submission. This does not promise unlimited physical memory. | `loader-depth-limit`, profile `DL10`, loader `MAX_DEPTH` |
| DL10: retained results | Pure vectors keep results until explicit expiry. The shell caps retained outputs and successful plain content at 64 MiB total and 7 days by default, configurable by Config or CLI flags. Size is compact UTF-8 JSON bytes of each retained outputs/content map, including its JSON overhead. Results in a pending repeat's previous snapshots count toward both size and age; restoration after refusal preserves their original completion time. The core's expire-outputs removes retained results throughout those snapshots under OPS-034/DL10, preserving all invocation and snapshot state. Before requests and after transitions, expire oldest completion first through expire-outputs until within both limits. Completion times are shell metadata committed beside State, independent of purgable records. Idle stores are checked on opening; no timer runs here. No key expires and invocation state is never compacted. Reconciliation invents no outputs; repeats follow OPS-024/034. | retention cap/age/restart tests in [tests.rs](crates/shell/src/tests.rs), `requests-plain-content`, `records-purge-succeeded` |
| DL12: tool schemas | Inline no optional contract references. Copy reached contracts once, with escaped local roots and finite recursive definitions; target-held contexts choose releases. | `e05-local`, `e05-two-contexts`, `e05-recursive`, `e05-siblings`, `e05-escaping`, [schemas.rs](crates/core/src/schemas.rs) |
| DL12: result content | Preserve successful plain MCP content alongside structuredContent. For other successes use structuredContent without an optional serialized text copy. Errors carry a status text block; invalid/conflict also carry diagnostic structuredContent. | `s1-whole`, `requests-plain-content`, `requests-declared-outputs`, [result envelope](crates/core/src/requests.rs) |
| DL13: instance proof | One store-private ECDSA P-256 key and self-signed CA certificate. The FST-007 trust domain is configured with `init --trust-domain` or generated from two UUID v4 values (244 random bits), validated before use and fixed for the store. `system.status.trustDomain` records it; `engine-id` carries `spiffe://<domain>/conduit/engine`, and token subjects use `spiffe://<domain>/conduit/instance/<id>`. ES256 tokens carry the instance's SPIFFE ID, engine-address audience, iat, random jti and exp set to iat plus 100 calendar years (UTC, clamping February 29 to February 28 if necessary). Verification requires an integer exp strictly later than the current time, and checks signature, ES256, audience, SPIFFE form/trust domain and current instance existence on every request. Missing proof maps to none; invalid proof maps to unreadable. Every run gets a fresh private directory with the token, engine ID, public JWKS and CA PEM, under the names the run contract supplies. Credentials never enter core State, records or shell output. | [identity.rs](crates/shell/src/identity.rs), identity and S1 shell tests |
| DL13: loopback boundary | This shell accepts an http engine address on loopback only. ca.crt checks nothing at this address; its root uses the same P-256 key as the JWT authority. Built-in runs now end with the engine on this one machine: a group leader watches an engine-only lifetime socket and kills its process group on EOF, including SIGKILL of the engine. Dropping either shell or runner also kills all groups. The order 022 adapter serves authenticated loopback HTTP with the runner as its executor. | [shell](crates/shell/src/lib.rs), [handoff](README.md#the-cores-boundary) |
| DL10: process and call deadlines | MCP initialization: 30 seconds from opening the stdio session. Dispatch and a refresh-plus-read: 120 seconds per intent. Config and CLI can set both. Initialization timeout kills the group and reports failed; dispatch timeout reports missing with the original invocation and attempt and never resends a write. npm preparation helpers have a fixed 120-second deadline, plus at most 5 seconds for their exit status. | [runner.rs](crates/shell/src/runner.rs), deadline/exit/repair/cancellation tests in [runner tests](crates/shell/tests/runner.rs) |
| DL16: packages and commands | npm is the only supported registryType; any other fails with its name. Package transport must be stdio. npm pack fetches the exact version with scripts disabled. SHA-256 is checked before decompression, manifest reading, dependency installation or execution. Verified artifacts are unpacked into a hash-keyed store cache, dependencies installed with npm install --omit=dev --ignore-scripts --legacy-peer-deps (no automatic peer installation; needed for the observed npm 10.2.4 resolver failure), and the sole bin run with node. The artifact hash does not pin the dependency graph (MAN-010). The runner does not check a Package's declared engines. The unchanged filesystem server 2.1.0 declares Node >=24, but S1 was exercised on Node 21.5.0: qualification covers those exercised operations on this machine only, not the declared Node platform range. npm uses a store-local download cache and empty user configuration, no global configuration, the public npm registry, and no audit/fund requests. Relative command paths are materialized from retained installed-package files under the store with their modes; absolute programs run as given, and bare names resolve through the run's PATH. | [runner.rs](crates/shell/src/runner.rs), installed-program unit test, opt-in hash mismatch and real S1 tests |
| DL16: environment and working directory | Unix only, as the engine's operating-system user and existing login session. Clear the environment, then pass HOME, LOGNAME, PATH, SHELL, TERM and USER when present and not beginning with (). Config.pass_environment (CLI: repeated --pass-environment NAME), empty by default, names further inherited variables, also skipping function values. Package environment declarations supply fixed values and defaults over inheritance; supplied environment bindings and secrets override them, and the materialized identity's two variables and instance state directory win. Package argument and environment values substitute declared server.json variables once, preserving unresolved identifiers. Fixed positional values need no valueHint. The runner reads only the permitted inherited names. Each run starts in a fresh empty store/work directory, mode 0700, separate from shared data and identity files. This is no OS sandbox. | exact exec-environment and binding unit tests; synthetic GITHUB_TOKEN exclusion, working-directory and identity tests |
| DL16: instance state directories | PRV-029: each built-in run receives the variable named by stateDirVariable in spec/names.yaml, overriding inheritance, the pass list and package defaults. Its initially empty directory is `<store>/instances/<instance-ID>`, mode 0700; every later run and engine restart reuses it. The runner reads none of its contents. MAN-018 rejects selected items binding reserved variables, including secret-reference bindings, without regard to case. LIF-035: after a durable removal or unregister, recursive deletion runs before publishing the reply. A failed deletion leaves the directory as a durable retry marker; every later core commit and store reopen retries it. No idle timer runs cleanup. Invalid permissions or a symlink prevent reuse as a state directory. Same-user access remains mediated. Assigned variable names are embedded from spec/names.yaml at build time; a rename requires rebuilding. | state-dir-binding vector; private state, runner restart, retirement and unregister/retry shell tests |
| DL16: run state and restart | No automatic restart. One process group and initialized MCP client session per run, with no Roots capability. Unasked exit closes outstanding dispatches as missing, and the next inspect-run reports stopped. Reopening has no sessions from before and reports them stopped; recovery never replays attempts. Explicit subsequent setup may start a stopped run. The lifetime guard ends every member still in the process group on engine loss; deliberately escaping that group is outside this runner's confinement, like other same-user routes. | shell-drop, killed-engine/descendant, process-exit, initialization and restart tests; real outside-root filesystem refusal |
| DL16: executor coverage | builtin-runner, dispatch, refresh-tools-and-call, cancel and cancel-setup-parts are served. Runtime provider calls and remote connections are unserved: inspection reports unknown, run/connection parts fail, and other calls report missing through the existing loop. Cancellation uses the attempt's actual JSON-RPC ID; setup cancellation kills every group it started and starts none. The transport preserves the core's exact tools/call params, including its omission of metadata for plain servers, rather than adding rmcp client/progress metadata. Refresh reads tools/list once then sends the eligible core-supplied read once within its attempt; no effectful retry is added. | wire, repair, write-error, late-result and setup-cancellation tests |
| DL15 | Generated names retain at most the first 57 characters of the template name, then append a hyphen and a five-character suffix of lower-case letters/digits, supplied by the boundary and retried on collision. Authored and generated names share the ruled grammar; unused suffixes are not consumed identities. | `registration-generated-collision`, `registration-s1`, [registration.rs](crates/core/src/registration.rs) |

D19 is a measurement convention under DL2/DL8, not a DL promise: compact UTF-8 JSON bytes
for records, gap metadata and invocation arrays separately, with per-step signed growth and
per-invocation bytes/key status/outcome. Held contexts and retained results count. The runner
prints these measurements; they are not a persistence format or compaction policy.

## Choices still required

| DL item | What has not been chosen or implemented here |
| --- | --- |
| DL1 | Container retirement dispatches independent members in stored order. Within dependency cycles, members retire in ascending ID order; dependency edges between cycles are preserved (LIF-021). Pool and Fleet cascades are implemented. The member IDs are captured at dispatch; a later registration blocks container retirement with diagnostics and needs another explicit cascade. |
| DL3 | Actor-reply waiting policy and delivery while the server cannot receive; no such provider runs here. |
| DL5 | Machine report intervals/deadlines, hostname override and voluntary runtime inspection intervals. The shell now discovers the OS hostname and OS/architecture labels at first start, plus explicitly supplied labels. |
| DL7 | Package removal/mirror controls, https fetching and package-source providers remain unfinished. Public install.v1 supports local file URLs, authorization, admission and records; installed bytes and virtual files are persisted in State. |
| DL8 | Live feed/cursor delivery. Physical records are now in the atomic State file. |
| DL10 | Retirement orchestration deadlines. A store write/fsync error stops that shell operation as an embedding error; no intent or successful reply is issued for that operation. General disk-space management is pending. |
| DL12 | Additional MCP revisions and transports beyond the loopback adapter below. |
| DL13 | Gate proof, TLS and authorization-server integration. No public OAuth issuer/grant or remote-runtime credential delivery is claimed by the loopback-only authority. |
| DL16 | Platforms beyond Unix process groups and lifetime sockets; qualification beyond the npm filesystem server and local scripted servers. |
| DL17 | Multi-machine restart/partition/fencing. No distributed configuration or guarantee is demonstrated by this core. |

DL11 and DL14 are removed identifiers, not unanswered choices. The
[increment 3 handoff](README.md#the-cores-boundary) assigns the next
shell work. Any choices still pending must be documented and evidenced before the build
claims their behavior. The [increment 2 log](../docs/proposals/first-build/README.md#settlements-register)
records its accepted implementation choices and language findings separately; the
[increment 3 log](../docs/proposals/first-build/README.md#settlements-register) records this worker's
choices for coordinator review.

## Consumer server (order 022)

The shell adapter uses rmcp 3.9.0 and axum to serve Streamable HTTP at `/mcp`.
It accepts revisions 2025-06-18, 2025-11-25 and 2026-07-28. The endpoint is fixed in
`engine-url.json` before any instance token is issued; opening the store with another
address fails. Only HTTP addresses with hosts `127.0.0.1`, `localhost` or `::1` and a nonzero
port are accepted, matching the HTTP layer's Host allowlist. Other loopback hosts, including
`127.0.0.2`, are refused at first start before the store fixes its audience. `localhost`
binds IPv4 loopback. This is the same full address used as token audience.
An absent or invalid Bearer credential receives HTTP 401 with WWW-Authenticate.
Readable unauthenticated tool calls enter the core with identity none and are recorded as
refused. Invalid signatures, audiences and removed subjects establish no instance.
Present Origin values must equal the endpoint's own origin exactly; others receive 403.
Authenticated malformed requests enter invoke with readable false, for the core's invalid
ending and records, before their protocol error or HTTP status leaves. Session IDs are bound
to the authenticated consumer; they do not establish identity themselves. An authenticated
tools/call with another consumer's session ID commits a core rejection before HTTP 401.
Rejection inputs retain each syntactically valid, nonempty serving and idempotency metadata
string independently; the core owns caller attribution and records. A malformed
tools/call with no established identity receives rmcp's own protocol response and creates no
record, including missing names and nonobject parameters or arguments (IFC-036).

For DL10/DL12, **no ttlMs is set by the shell** on discovery or tool lists. For
2026-07-28 rmcp supplies the protocol-required zero default (no freshness); legacy results
omit it. This is the nearest sound reading of order 022: no positive cache lifetime is chosen.
Lists carry private cache scope. The server declares tools.listChanged and Conduit's implementation-interface extension
at v0. Legacy sessions receive list-change notifications through their session stream;
2026-07-28 clients opt into a tools-list subscription. Delivery is only to the changed
consumer's sessions/subscriptions. A broadcast queue holds 1,024 pending change notices;
on receiver overflow the adapter closes and removes that legacy session, ending its open
HTTP streams. Requests using its old session ID receive 404; the client must initialize a new
session and refresh its list, after which notification delivery resumes. For 2026-07-28,
which has no legacy session, overflow ends the affected subscription stream; the client must
open a new subscription and refresh its list.
The engine does not infer a client's model catalog from what it sends. Model refresh and
turn boundaries still need the real-client work of order 023 and the admin's closeout.

Calls carry their serving and idempotency metadata into core invoke; no cached access permit
is used. Core results are preserved, with rmcp providing the revision's transport envelope.
Pending replies stay inside the shell until completion. Cancellation notifications reach the
core's cancel. Each HTTP response owns a separate cancellation token, carried to its handler;
dropping its body cancels that invocation even in legacy sessions, where rmcp's session
transport does not connect stream closure to handler cancellation. Raw connection-close tests
cover both legacy revisions and 2026-07-28, checking executor cancellation and durable unknown
completion while a second invocation remains pending. The waiting adapter checks its ticket
every 10 ms; there is no additional invocation deadline here. Shutdown cancels the transport
and commits recovery for any work that remains interrupted. The HTTP body limit is 4 MiB.
`conduit call` and `conduit tools` are MCP clients, without opening or recovering the live
store. Root and materialized instances have owner-only helper tokens in `client-tokens`,
a 0700 directory with 0600 files. Per-run identity directories retain the run-contract names.
These helpers are for the store owner, not additional consumer authority.

H03 is bounded explicitly. Public engine execution currently supports set-up.v1, open.v1,
close.v1, register-replacement.v1, register.v1, replace.v1, inspect.v1, purge.v1,
reconcile.v1, install.v1, retire.v1, unregister.v1, retire-with-resources.v1 and
retire-with-agents.v1, subject to the core's preflights.
Declare-lost.v1 and control dispatch remain unfinished public invocation paths.
Control reconciliation applies the held input values to the current revision (REV-010);
Agent reconciliation uses the owning Fleet, with management as the retired-owner fallback
(ACC-033).
Startup inspects every published operation's held contract, using the publisher's resolved
scope, and treats a contract with changes as an unfinished control regardless of its type or
template implementation source. It compares publication with that inventory and refuses an
unsupported nonmanagement consumer. Unfinished management tools remain in the core's list; calls answer
JSON-RPC -32603 naming the operation, without committing a transition.

`conduit serve` owns order 021's runner, explicitly sets up S1's registered instances and
inspects ready instances to restart stopped runs before listening. Setup and dispatch
completions use the serving executor so changes from deferred transitions reach the same
consumer notification path. On signal the transport stops, interrupted work is recovered,
and the runner ends every process group before the command exits. The opt-in process test
uses real filesystem sessions and run-directory tokens (base S1 Agent identities are
materialized by the test through the runner's API, since those Agents have no executable); it covers A/B access, T7 replacement
through `conduit call`, private notifications, durable records and no surviving run groups.
The runner keeps private historical engine/group PID diagnostics under store/run-groups,
used by the test to observe actual group disappearance without process-table access. These
files are never used to recover or terminate a process after restart. No real Agent client
is demonstrated by these tests.

## I3-04: standalone Agent adapters

The first build uses the independently buildable packages under packages/agent-claude-code
and packages/agent-codex. Their Types use plain tool mappings (PKG-020): turn.v1 maps to turn,
marked startsModelWork. The single wrapper per Agent starts no native client at setup and
keeps one conversation per wrapper lifetime. Claude streams one process; Codex resumes the
exact thread with a new process at each turn. Each wrapper has a 180-second per-turn deadline;
a failure or deadline poisons that session rather than starting it again. Runner shutdown
ends the whole process group. Conversation persistence across wrapper restarts is not claimed.

MAN-017 requires each environment-bound scalar to be required by the Type schema. Therefore
model is required with an empty-string default, although authors may omit it in configuration.
This is the nearest sound reading of order 023's optional model plus three environment bindings;
no promise or core decision changes. An empty model defers to the native client's own default.
Both reach booleans remain required and are false in live S1.

The only credential-reading wrapper path is its header helper. It emits one Authorization
header from the run's token file. Codex gets the identity directory as a helper argument;
Claude uses the identity variable. Keys/tokens are absent from client arguments, environment
and wrapper logs. Client stdout is parsed in memory; raw events and stderr are not logged.
Structured turn results contain reply text and clientTools: engine tool names from that turn's
startup events, with event positions, raw reportedNames and freshness. Only the first turn's
startup catalog establishes current tools (reported). Later session/thread init catalogs are
unavailable with session_init_unverified freshness, even if names differ; experiment 009
recorded a stale repeated Claude init. Missing lists are not_reported. Neither unavailable
nor not_reported establishes a current catalog; no engine list or model prose substitutes.
Codex 0.167.0 did not report a native catalog in experiment 009. Notifications start no
model work and tool changes are qualified only at explicit turn boundaries (IFC-038).

The native launch baseline is Claude Code 2.1.318 / Codex 0.167.0, as experiment 009 qualified
it; this order ran scripted binaries only. Claude's peer input selects and allows only the three peer tools when true, and removes
the inbound refusal so the client's own inbound default applies. This is the nearest sound
reading of --tools empty alongside an effective peer opt-in: machine tools stay unselected.
Real incoming-message refusal, inbound default and opt-in effectiveness remain closeout checks.
Codex has no qualified peer-messaging switch: its bound peerMessaging input changes no native
argument. Apps false/true uses explicit disable/enable apps, requiring real-client evidence.
No claim of account-source exclusion, peer refusal or intended subscription use follows from
the scripted tests. The owner's subscription is shared, as ruling 48 requires for increment 3.

The launch disables the client's shell, browser, hooks and plugins, alongside account services
and messaging (order 024, findings 3-04 and 3-05). Codex's collaboration tools remain available,
as finding 4-06 below states. Claude uses --setting-sources "" and --tools ""; only
explicit --mcp-config/--settings supply the engine, header helper and allow rule. Codex uses
project_doc_max_bytes=0 and --disable for shell_tool, unified_exec, browser_use,
browser_use_external, in_app_browser, computer_use, multi_agent, hooks, plugins, view_image,
goals and image_generation, and leaves code_mode_host on, through which Codex calls MCP tools
and whose code execution imports nothing that reaches the machine (finding 3-06).
accountServices true enables apps alone; the other features stay
off. Claude's peer opt-in remains limited to its three named tools.

All restrictions are mediated (ACC-027). Neither wrapper provides OS-user isolation, and
the shared login and same-user access to other run credentials remain limits. Codex's former
read-only shell bypass is disabled; the admin must retry the outside-file read in closeout.
Codex reads the instruction file of its home, `$CODEX_HOME/AGENTS.md`, whatever its flags
say, and the wrapper uses the owner's home, so the owner's own Codex instructions reach every
Codex Agent (finding 3-07): a stated leak until each Codex Agent has a home of its own.
The direct trial still listed apply_patch in its read-only sandbox and a request for user
input. Closeout must report those and any other native tools still on, plus any account/peer
channel enabled by an input, separately from successful MCP access enforcement.

For finding 3-03, core publication omits outputSchema for an operation that declares no
outputs, and its successful result omits structuredContent while preserving plain content.
Declared outputs keep their schema and structured result; invalid/conflict error diagnostics
remain as IFC-025 requires. The plain-read-text-only vector checks publication and the read's
sole text content, and the repeat vectors check omission on successful repeated requests.

CLI --package is an explicit local archive-admission step via install-archive, **not** the
public install.v1 operation. It accepts repeated ARCHIVE#SHA256 arguments and
checks the compressed bytes before reading manifest metadata. Core admission still checks
the digest, archive paths and contracts. Admission is per archive; earlier accepted packages
remain installed if a later archive or registration fails. The submitted project manifest is
not rewritten. Host archive digests vary with the executable/target, so live S1's working copy
needs freshly generated pins; its checked-in pins identify metadata-only vector snapshots.
The new registration-live-agents vector registers the portable contract copies, not executable
files. The shell's scripted S1 test separately installs/runs actual host wrapper bytes and uses
conduit call for both turn requests, checking distinct Agent consumer/caller records.

Wrapper request cancellation is observed while waiting for the session lock and before
starting a client. A queued cancelled turn starts no work; active cancellation kills/reaps
the client and poisons the conversation, so later turns cannot implicitly restart it.
CLI package metadata discovery accepts conduit.yaml, conduit.yml and conduit.json, and
resolves archive links in memory with the core's virtual unpack semantics before admission.

## I4-01: operations for changing S1

Public `install.v1` admits the same virtual archives as `install-archive`, with the request's
access decision, invocation and ending records. The shell first evaluates a pure trial
transition. Only its `archive-unavailable` decision permits a file read; the trial is not
committed. It reads the absolute local `file:` URL under the store's writer lock, supplies the
completion, and commits the core's rechecked transition. A new admission also emits an
`object-change` event with action `package-installed`, correlated with its management
invocation (DL8). Invalid archives and unavailable
files have no dispatched invocation. Already installed versions cause no fetch, including
when the given fetch no longer exists; another digest conflicts. Installed packages are
available to the next request. `https` and package-source providers remain unfinished and
report invalid with an explicit diagnostic. No network fetching is added here.
The archive catalog's virtual directory is store/archive-catalog/DIGEST; retained files stay
in State and the runner materializes selected relative programs as before. There is no cache,
mirror or physical extraction policy beyond that existing runner behavior.

Public retirement marks the instance retired and closes/removes target Conduits in the
initial committed transition, before shutdown work runs. A dispatched setup is remembered
through its invocation, independently of output retention. A never-set-up instance is removed
without a hook or stop (LIF-025, LIF-035). Otherwise a selected retire hook uses the existing
hook invocation/repeat machinery. Before inspecting, starting or connecting to its
implementation, retirement decides whether a new hook attempt is permitted. A retained success
or unknown outcome goes straight to the stop. For a permitted attempt, a stopped run is started
for it, or a remote connection is requested, before the hook. A retired placement node makes
that hook part unavailable without dispatch or run; a successful stop still leaves shutdown
unknown. A previously successful hook on that node allows removal once the stop succeeds.
The stop is requested whatever the hook returns, including on
cancellation. Part outcomes determine shutdown: success removes the instance and its consumer
Conduits, failure leaves shutdown pending (unknown on a lost node), and uncertainty leaves it
unknown. A further
retirement is explicit. Reconciliation of a retirement as having occurred confirms shutdown
and removes the instance. Recovery marks interrupted retirements unknown without replay.
Runtime providers, remote connections and secrets still have the unserved boundaries listed
above; this order qualifies the built-in runner on S1, not those providers.

`unregister.v1` checks the current revision and retirement state, asks once more for the stop
of a selected run, then removes the instance whatever that stop returned. A template is removed
directly after authorization and revision checks, without a stop. Records and frozen references
remain. Successful retirement already removes an
instance, so the unregister vector deliberately uses an unknown retirement, not a successfully
retired instance that no longer exists. This is the nearest sound reading of order 025's
retirement vector under LIF-017 and LIF-035, not a change to those promises.
The runner stops the run's Unix process group immediately with its existing SIGKILL mechanism;
no graceful-stop interval is chosen. Retired consumers' legacy sessions are closed/removed,
and modern subscription streams end, once the consumer is removed; other consumers continue.
These stream endings also apply when the consumer had an empty tool list.

The author commands `install`, `submit`, `replace`, `retire` and `unregister` are root MCP
clients. They open no writer and perform no recovery. A two-second TCP reachability preflight
and a ten-second MCP initialization limit make a missing server an error; operation execution
retains the server's existing deadlines. Commands print each requested result as one JSON line,
including revision inspections, and stop with nonzero status on a failed management result.
`submit` opens newly registered root-consumer Conduits, then sets up newly registered Pools,
Resources, Agents and Interfaces in that order, through public requests. It keeps successful
earlier steps if a later one fails. It neither substitutes packages nor changes documents.
`replace` submits only the selected template, its project's contracts and its manifest,
reading the revision through `inspect.v1`; an intervening change remains a core conflict.
`register.v1` carries no project directory, so `submit` cannot preserve SUB-010's directory
semantics for relative values. Before sending registration, it checks the manifest and documents
for relative `fetch` values and relative programs (a first `command` entry containing `/` that
is not absolute). It refuses them with a nonzero exit and explains that a submission through
the serving engine takes absolute values only. Bare program names still use PATH, under
MAN-010. [Ruling 52, point 3](../docs/proposals/ruling-52/README.md#the-points-and-the-answers)
settled finding 4-03 under SUB-010's existing rule: relative values are resolved only when the
engine is handed a directory; a request carrying values rejects them. Uploading a directory
waits until remote clients exist; the specification is unchanged.
Resource inspection/setup/retirement choose the root's open route to its owning Pool from a
read-only store snapshot; every server request rechecks authority. There is no automatic
repointing of frozen references or migration of existing Agent sessions.

Filesystem 0.2.0 is an installable fixture, not a preinstalled snapshot. Its Type, contracts
and tools map are unchanged, with the npm artifact 2.2.0 pinned by its measured SHA-256.
History keeps filesystem 0.1.0. The portable cases `change-t4`, `change-t5` and
`change-retirement`, shell change tests, author-command process test, consumer-stream test and
opt-in npm serve test cover these paths. The npm test installs while serving, sets up a new
Resource at another directory, reads through server 2.2.0, checks the three-tool exposure and
held 0.1.0 instance, then retires the new run. No native Claude or Codex client runs in this
order; the Agent retirement test uses the existing scripted clients.

## I4-02: startup repair, retirement parts and inspection

Retirement selects the retire hook's implementation before inspecting a run. A remote hook
connects independently of any other selected runnable item; stopping the instance's run still
follows the hook. A run started for the hook and a connection made for it are retained among
retirement's parts, including successes. The run has an effect; the connection has none.
An unknown run state dispatches no hook and contributes an unavailable hook with no effect.
A failed or unknown preparation also leaves the hook unavailable. The shell's three retire-hook
regressions register real contracts and drive explicit boundary reports; they do not qualify
remote transport or runtime providers, which remain unfinished as above.

`serve` continues after an instance's setup request fails, printing the failure on stderr and
retaining the core's state and records. If root has no route to a Resource's Pool, startup
requests setup through management; the core refuses it and records that refusal. Startup
creates no missing Conduit. The serving engine then accepts a repair submission and a further
explicit setup through the newly opened route. Store, preflight and embedding errors still
stop startup. First-registration setup failures still make `register` exit nonzero.

`inspect` uses root MCP requests while the store's writer is held, and keeps the existing local
workflow when it can open the store. It uses the same read-only state snapshot as the author
commands for object enumeration and route selection; every request rechecks authority.
Each successful CLI inspection row for an instance additionally carries `consumedConduits`, including declared and
standalone Conduits, with each Conduit's actual name, target kind/name/ID and open boolean.
This supplement uses that snapshot and is outside `result`, not an addition to `inspect.v1`.
There is no defined inventory for standalone consumer Conduits or their open state in that
operation's output: `declared` describes the object's declaration, while `status` defines no
such inventory. This is reported as a gap for the coordinator; the specification is unchanged.
Refused inspections receive no supplement; finding 4-08 remains open.
CLI inspection is not an atomic inventory across concurrent management requests.

OPS-029 diagnostics identify the kind and reference supplied by the caller and say it does
not exist. They explain that a closed Conduit and a removed instance no longer exist, without
claiming that every missing name previously existed. Refused requests retain their existing
non-disclosing response. The runner tests now wait for parseable PID contents; the runner's
implementation is unchanged.

The Codex package remains 0.1.1. It additionally disables multi_agent_v2 and
multi_agent_v2_dynamic_tools on first launch and exact-thread resume, for every combination of
the reach inputs. The local bundled binary rejects --ignore-user-config, so its feature list
was instead read with an empty temporary CODEX_HOME. It lists both features, and lists all
three multi_agent switches false when explicitly disabled. Its strings also contain the
collaboration tools. This metadata and the argument tests do not establish their absence in
a real client; no native Agent client or model work ran in order 027's tests.

Finding 4-06 was settled by the admin on 2026-11-10 in the
[increment 4 log](../docs/proposals/first-build/log-increment-4.md). Codex 0.167.0 keeps all six
tools, `collaboration.spawn_agent`, `collaboration.send_message`, `collaboration.list_agents`,
`collaboration.wait_agent`, `collaboration.interrupt_agent` and `collaboration.followup_task`,
under every switch tried: `multi_agent`, `multi_agent_v2`, `multi_agent_v2_dynamic_tools`,
`collaboration_modes`, `in_app_local_automation` and `agents.max_depth=0`. In the admin's trial
with the wrapper's flags, a spawned sub-agent had no shell, could not read outside its sandbox
and had an `apply_patch` write refused by the read-only sandbox. Its requests of the engine
carry the parent Agent's credential, so it reaches only what that Agent may. This is a route
to more parallel model work within the Agent's own access, not around it. What offers the
tools is still not found. Finding 3-07's statement above remains in force.

As of 2026-11-10, ruling 52 settles 4-03 as SUB-010 already states and settles 4-08 in direction,
with its promises still to be written. Findings 4-01 and 4-11 are being settled with the owner
through the language-shape and Project investigations.


## Author commands (work order 028)

Every printed failed result makes the command exit nonzero, including `isError`, an outcome
other than `succeeded`, a rejected local registration, and incomplete record export.
`submit` prints each attempted registration, open and setup step's outcome after its results;
it stops on failure and preserves completed steps. Help describes every command and example.
`tools --as` accepts both names and kind/name references and explains missing identities.

S1 packages have no special treatment in engine code. Tests explicitly install their archives
(or scripted variants) before registration. Portable vectors retain snapshots as fixtures.
Authentication's blocking store read and verification run on the blocking pool so commits do
not occupy async workers. Client initialization gets three attempts of up to ten seconds;
errors identify the step and elapsed time. Requests already sent are never retried by the CLI,
and automatic transport reinitialization/replay is disabled.

A successful run start supersedes the earlier stopped observation. The core retains ended
observations internally; authorized CLI inspection shows them in `pastConditions`, each with
`condition` and `endedAt`, separately from the current status. LIF-011 and `condition.v1`
define no ended-condition history form, so this is a CLI supplement, not a new status promise.
The regression tests cover current running/unknown observations, ended times, parallel CLI
processes, failure exits and partial submission summaries.
