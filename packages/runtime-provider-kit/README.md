# Runtime provider kit

A63-RK1's standalone library, `runtime_provider_kit`, implements the MCP server for
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

Implement `RuntimeBackend`, `StateStore` and `Gate`, then call `RuntimeServer::new` with shared
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
errors are the payload-free `Unavailable` type, so subprocess output or credential-bearing
error messages cannot become an answer. Backend and gate implementations must apply the same
rule to their own logging.

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
| `Gate` / `GateHandle` | Consume the byte channel and receive the run's identity; return its current URL, close the listener/channel on stop and signal termination. Opening must cooperate with cancellation; closing is idempotent and completes only after the channel closes. RK2 supplies the real gate. |

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

`Identity`, `Secrets` and `RunInput` have no `Debug` or `Serialize` implementation. Staged
material must clean up when dropped; staging that fails or is cancelled cleans its temporary
material before returning. A cancelled start must quiesce: it cannot launch anything after
returning. Start failure is treated as potentially having started a tree, so the kit stops it.
The gate must not put secrets or identity material into its URL. The kit also rejects a new URL
containing a supplied sensitive value. No actual file staging, runtime, TLS, certificate parsing
or production gate is implemented here. A runtime and gate remain trusted implementations of
these contracts; the kit cannot prove remote cleanup or backend file permissions.

## Recovery and lifecycle choices

These are the choices in work order 122 decision 5, isolated in the server's policy functions;
order 081 settles the specification text. The current specification remains the authority.

- `repeated_run_policy`: a running run returns its held gate's current URL without staging or
  starting another. Unknown starts nothing and answers unknown. Without a held gate, a running
  run answers unknown because the kit cannot supply the required URL.
- `successful_stop_policy`: cancel the instance's in-flight stage/start/gate opening, wait for
  it to quiesce, close its gate, then stop its whole tree. No restart follows a successful stop.
  A subsequent explicit `run.v1` may ask for a new run. Calls for one ID are serialized, including
  calls that name different machines; a queued start overtaken by a stop is cancelled too.
  Registration and cancellation happen before the first ledger read, so a stalled read cannot
  hide an earlier run from a later stop.
- `lifetime_policy`: EOF, channel errors/drop, gate termination or the configured lifetime bound
  ends the run and updates the ledger through the same durable stop path. There is no implicit
  restart (`implicit_restart_policy`). `max_lifetime` is required and nonzero. The runtime must
  enforce it remotely so that it works when this server is gone.
- `startup_policy`: sweep both the ledger and the backend inventory. Stop every reported run,
  since this server holds none of their old channels. Failed stops remain owed. An incomplete
  inventory or an inability to durably record recovery intent prevents serving.
- `drain`: drain owed stops at startup, before each well-formed call and at `drain_interval`,
  which the runtime sets to a nonzero duration. Both interval and tool calls use the instance
  lock. Invalid arguments and extension errors reach no backend. Each stop intent is durably
  written before the backend effect; clearing it follows confirmed whole-tree termination.
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
trace-level server logs. They make no claim to production gate, remote-machine or OS-isolation
qualification.

Regression tests also pause a run's initial ledger read until a later stop succeeds, lose the
provider transport with ledger reads failing, stop active and pending runs with unreadable
ledgers, drain obligations written during read failures, and change machine labels between calls.
