# Runtime provider kit

A63-RK1/RK2's standalone library, `runtime_provider_kit`, implements the MCP server for
`runtime-provider.v1`. A runtime package depends on this crate by path and supplies the
mechanisms below. The kit has no Type, package manifest or archive and depends on no other
repository crate. Its lockfile and Rust 1.97.1 toolchain travel with it.

The server offers exactly `run.v1`, `stop.v1` and `inspect-run.v1`, and advertises Conduit's
implementation-interface extension at revision `v0` through `server/discover`. Every call must
itself declare that revision in `io.modelcontextprotocol/clientCapabilities`. A missing or
unsupported revision gives -32021 with `data.requiredCapabilities`, before any backend effect.
Other rejected calls return failed tool results with fixed diagnostics. All tool results carry
an outcome and the corresponding `isError`; diagnostics are text content, never Conduit's
mutually exclusive reason key. Inspection succeeds with the state, including unknown.

## Embedding

Implement `RuntimeBackend` and `StateStore`, use `gate::TlsGate` (or supply another `Gate`), then call `RuntimeServer::new` with shared
instances and `Options`. Construction performs recovery before serving. Use `serve_stdio` for
the engine's process runner, or `serve_transport` for an owned duplex byte transport; both wait
for MCP transport termination and then close gates and stop held trees. Call `shutdown` for an
explicit orderly stop. The serving future must be allowed to finish cleanup, rather than being
aborted. A provider process crash is handled by the backend's channel loss/lifetime mechanisms
and the next startup sweep.

The MCP handler is private: the public serving entrypoints always apply transport lifetime
cleanup and sensitive-input protection.

The byte transport uses MCP's newline-delimited JSON wire form, also used for stdio. It moves
call parameters and Conduit metadata into opaque, private rmcp extensions before rmcp sees a
request: rmcp 3.9.0 otherwise logs full requests at debug and trace levels. The handler recovers
the original parameters without serializing that extension. Parsing errors close the byte
transport without logging its raw input. No logger is installed or globally disabled. Backend
errors use `Unavailable` without a reason by default. A runtime can attach its own explanation
with `Unavailable::because(text)` under the rules below. Passed-through subprocess output or
credential-bearing error text must never become a reason. Backend and gate implementations
must apply the same rule to their own logging.

## Runtime interfaces

| Interface | Runtime responsibility |
| --- | --- |
| `RuntimeBackend::declaration` / `Declaration` / `MachineSelector` | List registry types, package transports, whether commands run, and hostname/label selectors for machines. Outside the declaration, staging and start are never called. Remote items and relative command paths cannot be run. |
| `RuntimeBackend::inventory` | Enumerate every potentially live run, including those absent from the kit ledger, with the original provider-call context. This is administrative recovery before any current call exists; an incomplete inventory fails construction. It does not read configuration from the environment. |
| `RuntimeBackend::stage` | Prepare the item, package arguments, ordinary environment, secrets and identity for the specified instance and machine. `Staged` is an opaque runtime-owned value and may represent staging on another machine. |
| `RuntimeBackend::start` | Consume staged material and return `Channel`, a duplex stream to the run's stdio. Apply the lifetime bound on the run's machine as well as observing cancellation. |
| `RuntimeBackend::stop` | Stop the entire tree on the context's machine. Return success only when no run for that ID there can remain or restart, including queued launches and backend restart jobs. An unavailable machine returns `Unavailable`. |
| `RuntimeBackend::state` | Return running, stopped or unknown for all runs this provider started for that ID on the machine. Stopped proves none may remain. Errors become unknown. |
| `StateStore` | Place and durably retain the run ledger and owed stops. Atomically upsert a `RunRecord` by instance and machine hostname; success acknowledges durable commit. The store remains reachable after provider retirement and instance removal. |
| `Gate` / `GateHandle` | Consume the byte channel and receive the run's identity; return its current URL, close the listener/channel on stop and signal termination. Opening must cooperate with cancellation; closing is idempotent and completes only after the channel closes. `gate::TlsGate` supplies the shared per-run TLS gate. |

`CallContext` carries `config`, `revision` and `machine` to staging, start, stop and state. For a
live provider call, config and revision come only from that call's metadata, including when it
checks a previously recorded machine. Administrative recovery and the public drainer use the
original context saved in the record. Declaration and inventory are administrative interfaces,
not provider calls. Configuration must contain references rather than secret values, as PRV-005
and IFC-006 specify. Machine selectors use exact label equality; an empty list supports no
machines, while a selector with no hostname and no labels supports every machine.

Staging must deliver every identity environment variable and every supplied identity file at
its schema-named path in the private directory, and give each secret only as its named variable
or package argument (PRV-011/022). Identity replaces package defaults. Carry these values across
machines only through an authenticated confidential channel, and keep them only where the
runtime contract states. The engine URL must preserve scheme, host, port and path. Ordinary
and secret environment names cannot overlap or bind identity variables, case-insensitively for
the latter. No ambient environment or state-directory variable is added by the kit. A runtime
supplies a state directory only if its contract declares one (PRV-029).

A reason is text the runtime writes itself naming what failed: a setting, a variable's name,
a program or a machine's hostname, never a value. Never pass through an operating-system,
library or remote tool's error text. `Unavailable` remains usable as a value and a type;
compare it as a value, never use it as a pattern. Its `Debug` always prints only `Unavailable`,
even with a reason.

Only failed staging and a backend `start` error can put a reason in an answer: `staging failed:`
or `start did not complete:` followed by a space and the reason, with the existing tree-stop
suffix for start. Outcomes and text without a reason stay unchanged. The kit drops reasons from gate
opening, the state store, inventory, stop and state. Reasons are text content, never Conduit's
`_meta` reason key. The kit holds them only until building the answer and never writes them
to its ledger, state store, records or its own logs. Like a gate URL, the answer carrying a
reason can reach rmcp's debug log, so the kit checks it before answering.

The reason and its one-line form are checked against the same non-empty sensitive values
used to check a gate URL: identity variables, identity files, secret variables and secret
package arguments. A whole value found in the original reason, or its one-line form found in
the one-line reason, withholds the entire reason and substitutes
`runtime's reason was withheld because it contained a value supplied to the run`.
This check precedes cutting, so a value beyond the length limit still withholds the whole
reason. A fragment of a value is not caught.
Ordinary environment values and item/package arguments are not secrets by their schemas and
do not count; short ordinary values would otherwise withhold most reasons.

The kit makes reasons one line by replacing each control character and U+2028 and U+2029 with
a space, and cuts them to at most 512 Unicode scalar values on a character boundary. The same
replacement makes sensitive values one line for comparison. An empty or blank reason after
cleaning and cutting counts as no reason.

`Identity`, `Secrets` and `RunInput` have no `Debug` or `Serialize` implementation. Staged
material must clean up when dropped; staging that fails or is cancelled cleans its temporary
material before returning. A cancelled start must quiesce: it cannot launch anything after
returning. Start failure is treated as potentially having started a tree, so the kit stops it.
The gate must not put secrets or identity material into its URL. The kit also rejects a new URL
containing a supplied sensitive value. The kit implements TLS, certificate identity parsing and the shared gate; actual file staging
and the runtime remain caller responsibilities. A runtime remains a trusted implementation of
these contracts; the kit cannot prove remote cleanup or backend file permissions.

## Shared TLS gate

`gate::TlsGate::new(gate::GateConfig::default())` implements the existing `Gate` interface.
Each `open` consumes one stdio `Channel` and binds a new port for that run. It serves the run's
PEM certificate and key from `identity` in memory, with TLS 1.3 and ALPN `http/1.1` only. The
certificate must have exactly one SAN, the URI named by the run token's `sub`. No client
certificate is requested. Missing or mismatched identity fails opening and drops the channel.
Nothing is logged or persisted by the gate.

The default bind IP and advertised host are both `127.0.0.1`. Set them separately for a routed
provider: the bind IP must be a specific interface, never `0.0.0.0` or `::`; the advertised host
is a bare DNS name or IP, with IPv6 bracketed in the resulting `https://<host>:<port>/mcp` URL.
A provider must still put its runtime and runs under separate OS principals (PRV-025).

Every POST, GET and DELETE requires the engine's ES256 bearer JWT: its `kid` selects a P-256
key in the run's JWKS, its `sub` equals the trimmed engine ID, its sole audience equals the
run token's `sub`, and its integer expiry is in the future. Critical JWT headers are refused.
A failed proof returns an empty 401 with `WWW-Authenticate: Bearer` before the channel is used.
Host, Origin, MCP protocol version and GET Accept are not checked.

`GateConfig` defaults to a 30-second keepalive idle time, 5-second interval and 3 probes;
`silent_cut_bound()` reports 45 seconds. Every accepted TCP socket gets these settings. Linux
also gets `TCP_USER_TIMEOUT` at that bound. macOS has no equivalent, so outstanding
unacknowledged data can exceed this bound there; the TCP keepalive bound applies when probes
can run. A silent packet-drop test needs privileged network controls and is not part of these
tests. The configuration requires positive whole seconds and positive counts.

One session is live at a time. An initialize assigns a fresh random 256-bit session ID; another
initialize cancels open work and closes the old GET stream before forwarding the new initialize.
Thus a persistent stdio server sees a second initialize when the engine bridge restarts; the
runtime must use a server that tolerates this. Session IDs are required on later messages.
DELETE ends the session and cancels its open requests, leaving the channel alive.

POST bytes are forwarded as one stdio line. Notifications and client responses return empty
202s. Requests receive the matching response bytes in one SSE event if Accept allows
`text/event-stream`, otherwise as JSON. Only messages with a `method` enter GET, including
server requests whose IDs equal engine request IDs. Unmatched responses are discarded. A lost
response connection, DELETE, session replacement or close writes `notifications/cancelled` for
pending requests. `close` waits for listener, connections and channel to close, and is idempotent;
`ended` also resolves on channel EOF/error. A stalled channel gets at most one second to drain
queued cancellations at shutdown, then its writer is dropped.

The following are kit choices and limits of this bridge, rather than additions to the language:

- The GET buffer defaults to 256 messages; an active GET has the same queue capacity. Overflow
  ends the session and cancels pending requests, leaving the channel alive. Before initialization
  run-originated messages are discarded. A new GET replaces the old one; no replay is promised.
- The message limit defaults to 8 MiB per POST or stdio line. Oversize POSTs return empty 413s;
  malformed, oversized or unfinished stdio messages close the gate. CRLF stdio delimiters are
  accepted. POSTs must be single-line JSON-RPC objects; malformed objects, batches, raw CR/LF
  and null request IDs return empty 400s, rather than being rewritten for stdio.
- Active duplicate request IDs and IDs cancelled earlier on this channel return empty 409s.
  The latter remain reserved for the channel's lifetime: stdio carries no session discriminator
  that could distinguish an old cancelled reply from a new reply with that ID.
- GET carries no SSE event IDs, resumption, replay or `Last-Event-ID` handling. The bridge does
  not implement `Mcp-Param`, protocol-version negotiation, task-specific behavior or heartbeat
  events. MCP content and capabilities, including initialization's negotiated version, pass
  through to the server. It inspects only JSON envelope fields needed for framing/correlation.
- Content-Type and JSON Accept negotiation are not enforced; POST Accept only selects SSE or
  JSON. Other methods return empty 405s, and other paths or query strings return empty 404s.
  A refusal carries no JSON-RPC body, since the engine bridge would forward one.

## Recovery and lifecycle choices

These are the choices in work order 122 decision 5, isolated in the server's policy functions;
PRV-019, PRV-021 and PRV-030 in [the provider promises](../../spec/promises/providers.yaml),
as order 081 wrote them, state the specification's promises.
The current specification remains the authority.

- `repeated_run_policy`: a running run returns its held gate's current URL without staging or
  starting another. Unknown starts nothing and answers unknown. Without a held gate, a running
  run answers unknown because the kit cannot supply the required URL.
- `successful_stop_policy`: cancel the instance's in-flight stage/start/gate opening, wait for
  it to quiesce, close its gate, then stop its whole tree. No restart follows a successful stop.
  A subsequent explicit `run.v1` may ask for a new run. Calls for one ID are serialized, including
  calls that name different machines; a queued start overtaken by `stop.v1` is cancelled too.
  A `stop.v1` arriving while a start is in flight cancels it. A drain carrying out a stop owed
  before the start began does not cancel that start, whichever drain carries it out. When
  the start's own drain carries out its owed stop, the start proceeds.
  Registration and cancellation happen before the first ledger read, so a stalled read cannot
  hide an earlier run from a later stop.
- `lifetime_policy`: EOF, channel errors/drop, gate termination or the configured lifetime bound
  ends the run and updates the ledger through the same durable stop path. There is no implicit
  restart (`implicit_restart_policy`). `max_lifetime` is required and nonzero. The runtime must
  enforce it remotely so that it works when this server is gone.
- `startup_policy`: sweep both the ledger and the backend inventory. Stop every reported run,
  since this server holds none of their old channels. Failed stops remain owed. An incomplete
  inventory or an inability to durably record recovery intent prevents serving. A stop that the
  startup sweep could not carry out is owed and blocks new runs on that machine until a drain
  carries it out.
- `drain`: drain owed stops at startup, before each well-formed call and at `drain_interval`,
  which the runtime sets to a nonzero duration. Both interval and tool calls use the instance
  lock. Invalid arguments and extension errors reach no backend. Each stop intent is durably
  written before the backend effect; clearing it follows confirmed whole-tree termination.
  A held stop is carried out before any new run on that machine starts; a start held by
  this names the owed stop, and starts nothing. A runtime's contract must state this rule.
  On a provider of one machine, a single owed stop blocks every new run there. Stops held only
  in memory also block the machine; another instance's stops owed on other machines do not.
  If the start's own owed stop cannot be carried out, the answer is unknown, beginning with
  "run may still be running" and naming each of its owed stops with its instance ID and that
  stop's machine hostname, including historical machines. When the start's own state is
  unknown, the answer also names every other instance's owed stop on the call's machine.
  Another instance's owed stop on that machine gives failed where the start's own state can
  be read, else unknown. Both answers name the stop and machine and say that the start may
  be tried again once those stops are carried out. A repeated run that is already running still
  returns its held gate's URL. Stop intents are published in memory before storage or backend work,
  including intents that become durable, so admission sees stops published during its state
  lookup. A per-machine read-write lock coordinates this publication with admission checks
  before staging and immediately before start. A run is admitted only if, at the moment its
  start begins, no other instance owes a stop on its machine. Starts share the read side;
  a stop publishes its obligation under the write side. A stop never waits on staging, only
  on the backend's start calls already in progress on its machine. A runtime's contract must
  state these admission and stop-waiting rules. Admission never takes another instance's
  operation lock.
- `inspect`: inspect every recorded machine for an ID and the call's machine. Report stopped
  only if all are known stopped; any uncertainty or owed stop gives unknown. Return the held
  gate's current URL wherever known. Backend state is authoritative rather than cached ledger
  state, and store/read failures produce a succeeded inspection of unknown.

`drain_owed_stops` is public and needs no MCP server or instance registry. A runtime's external
mechanism calls it after the provider is retired. It returns the number of obligations cleared;
individual unavailable stops remain owed. The runtime must ensure exclusive ownership: do not
run a second drainer or server over the same backend/store simultaneously. The server's internal
drainer coordinates its own concurrent calls. Multiple provider processes sharing a store need
an exclusive lease supplied by the runtime; the kit does not invent a distributed lock protocol.

Beyond decision 5, `invalid_call_policy` isolates the choice to fail unknown operations and
missing/malformed config or revision without effects. A start that fails after it may have
launched is failed only after confirmed cleanup; otherwise it is unknown with an owed stop.
Successful inspection still reports unknown if the ledger cannot be read. A storage failure
still permits gate closure and an attempt to stop every tree whose context the server holds,
plus the stop call's machine. Stop intents are written wherever writes remain available. A stop
with an unreadable ledger stays unknown because other durable runs cannot be ruled out.
Shutdown uses the held contexts too, independently of ledger reads, and attempts all known
trees before reporting a storage failure. Durability depends on the store honoring its contract.
For a live call, current config and revision replace historical values; current machine labels
replace labels for the same hostname, while distinct historical machines retain theirs. Runtime-specific
staging, storage placement, confidential channels, process/container mechanisms, restart
configuration and the external retirement drainer belong to each runtime's contract.

The only durable fields are instance, machine, original provider configuration/revision, state
and owed-stop flag. The ledger never holds an item, ordinary/secret environment, arguments,
identity, URL or staged material. The kit writes no log lines. A gate's in-memory URL belongs
to that gate and is not persisted.

## Schema copies and checks

[src/schemas.json](src/schemas.json) holds both schemas where present for each operation;
`stop.v1` has no output schema in the specification. The eight transitively reached Schema
contracts are copied alongside them. Each core-qualified reference becomes `#/$defs/name`;
a definition's local `#/$defs/member` becomes `#/$defs/name/$defs/member`. Tool schemas attach
the complete shared `$defs` map, so both input and output schemas stand alone and resolution
never fetches anything from the network. Identity filenames are read from this copy, rather
than repeated as Rust literals. Assigned protocol/environment names appear in one Rust module.

The drift test independently reads the operation contracts, walks all reached definitions and
performs that same reference rebasing, then compares the complete copies. It also checks the
assigned names and identity filenames against `spec/names.yaml`. In a standalone copy with no
sibling specification it skips that repository-only comparison; compilation and runtime schema
validation never read the specification. Inside this repository, any schema change fails the
test until its copy is deliberately updated.

Run from this directory:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
```

The tests use an rmcp client, a fake backend/gate and a file-backed fake store with durable
writes. They cover lifecycle races, provider transport loss, startup orphans, restart recovery,
external draining, schema drift and synthetic sensitive markers in answers, records and
trace-level server logs. The gate tests additionally use test-time authorities, signed ES256 tokens and real loopback
TLS clients that check the certificate chain and SPIFFE URI. They cover authentication, sessions,
byte preservation, GET buffering, connection resets, cancellation, keepalive socket readback and
shutdown. They make no claim to remote-machine or OS-isolation qualification.

Regression tests also pause a run's initial ledger read until a later stop succeeds, lose the
provider transport with ledger reads failing, stop active and pending runs with unreadable
ledgers, drain obligations written during read failures, and change machine labels between calls.
