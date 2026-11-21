# Local-process runtime provider

A63-RP1 is a Unix provider package exporting `local-process.v1`, a Resource Type offering
only the core's `runtime-provider.v1`. It implements `run.v1`, `stop.v1` and `inspect-run.v1`
over stdio through the [runtime-provider kit](../runtime-provider-kit/README.md). Its package
address is `example.org/conduit/provider-local-process`, version 0.1.0. The engine's own
runner remains S10's first implementation.

It runs `command` items whose programs are absolute paths on this machine, identified by
the operating system's hostname. Bare names, relative paths, other machines, all registry
types and all package transports fail without launching a program. The engine's configurable
hostname (DL5) must agree with the OS hostname for placement to match this provider.
Command arguments are literal; no shell evaluates them. Configuration bindings arrive in
the call's `environment`, with secret bindings separately. `packageArguments` require a
package and are unsupported along with packages. Registry packages await A63-RPN.

## Start and configuration

Build with Rust 1.97.1 using `cargo build --locked --release --bin provider-local-process`.
The engine supplies the provider's private `CONDUIT_STATE_DIR` (mode 0700) under PRV-029.
For a direct start of `target/release/provider-local-process`, supply that variable and
`LOCAL_PROCESS_DURABLE_DIR`. The state directory's parent must exist. Before setup, the
operator creates `durableDir` outside the engine store, owned by the provider's OS user
at mode 0700, and configures its absolute path.

The manifest binds these configuration fields to startup variables. Values are read once;
a configuration change takes effect at the provider's next start. Calls still carry their
current configuration and revision for the kit's durable recovery context. All seven fields
are required by the Type schema under MAN-017. The six fields with defaults still supply
omitted configuration values.

| Field | Startup variable | Default |
| --- | --- | --- |
| `bindAddress` | `LOCAL_PROCESS_BIND_ADDRESS` | `127.0.0.1` |
| `advertisedHost` | `LOCAL_PROCESS_ADVERTISED_HOST` | `127.0.0.1` |
| `keepaliveIdle` | `LOCAL_PROCESS_KEEPALIVE_IDLE` | 30 seconds |
| `keepaliveInterval` | `LOCAL_PROCESS_KEEPALIVE_INTERVAL` | 5 seconds |
| `keepaliveCount` | `LOCAL_PROCESS_KEEPALIVE_COUNT` | 3 probes |
| `maxLifetime` | `LOCAL_PROCESS_MAX_LIFETIME` | 3600 seconds |
| `durableDir` | `LOCAL_PROCESS_DURABLE_DIR` | None. Supply the absolute path to the operator-created directory. |

The bind IP must identify an interface; unspecified addresses are refused. The advertised host
is a bare IP or DNS name. Keepalive and lifetime values must be positive whole seconds, and
counts positive integers. The keepalive silent-cut bound must fit signed 32-bit milliseconds.
The kit's unchanged buffer/message defaults are 256 queued messages and 8 MiB per message.

Startup refuses a missing, empty, relative, non-directory, foreign-owned or non-0700
`durableDir`, including a symbolic link as its final component. The durable instance subdirectory
also has to be an owned directory at mode 0700. Startup refuses a durable directory
inside `CONDUIT_STATE_DIR` or its parent, including either directory itself. Placement uses
canonical paths, including macOS's `/var` alias. If `CONDUIT_STATE_DIR` still has a legacy
`ledger.json` or `markers/`, startup names every entry found and refuses. Stop the provider
and move those entries into its computed durable subdirectory, preserving the contents and
private modes.

Each run has its own TLS 1.3 gate listener and Streamable HTTP URL. The gate serves the run's
certificate, whose URI identifies the instance, and admits the engine's ES256 bearer token.
The silent-cut bound is idle time plus interval times count: 45 seconds by default. Linux
also sets `TCP_USER_TIMEOUT`. macOS lacks it; outstanding unacknowledged data can exceed
that bound there, while the keepalive bound applies when probes can run. Privileged silent
packet-drop testing has not been performed. See the kit's README for its session/bridge limits.

## Processes, credentials and retained state

Every run starts in a fresh 0700 private directory, `runs/<SHA-256(instance-ID)>`, under
the provider's state directory. IDs are hashed as their UTF-8 bytes to 64 lowercase hex digits;
an ID such as `../x` never becomes a path. The environment is cleared, then populated with
`HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM` and `USER` when present and not beginning with `()`,
then the call's ordinary and secret variables, then identity and state-directory variables.
No additional inheritance list is provided in RP1. Startup configuration variables do not
reach runs unless an item explicitly binds them.

Launch memory and the run's environment hold secret variables. Identity is also materialized
in `identity/` inside the private run directory: directory mode 0700, files mode 0600,
including `tls.key`. `CONDUIT_IDENTITY_DIR` points there, and `CONDUIT_URL` preserves the
engine URL unchanged. The gate holds identity in memory for its proof checks. The provider
keeps no launch configuration, secret or identity in its answers, log or ledger. Run stderr
goes to `stderr.log`, mode 0600, inside the private run directory; the program itself can
write credentials there. A confirmed stop removes the private directory and its contents.
Staging failure or cancellation also removes material that never launched. No network channel
carries launch inputs: the guard receives them through a private inherited Unix pipe.

Each run also receives `CONDUIT_STATE_DIR` pointing to
`instances/<SHA-256(instance-ID)>` under the provider's state directory, initially empty and
mode 0700. Every later run reuses it, including after provider restart. The provider never
reads its contents or removes it during a run's lifecycle. Engine removal of the provider
Resource deletes its `CONDUIT_STATE_DIR`, including `runs/` and `instances/`.

Runs use the provider's operating-system user and existing login session. This is mediated
access, with no OS sandbox or separate gate user in RP1. Same-user processes can read private
material and bypass the gate. Separate users are RP2 work.

The provider's binary leads each run's process group in an internal guard mode. The guard
holds a locked marker while a group may execute, observes the provider's lifetime pipe, and
enforces `maxLifetime` independently of the provider and gate. Provider loss kills the group;
the group never automatically restarts. An ordinary stop requests TERM, waits 200 ms, then
requests KILL for the whole group, including the guard. Confirmation waits up to three seconds
for the marker lock to become free and the group to be absent. A failed signal reports an
unknown stop. The backend's injectable signaller is a library interface for testing; the
shipped binary exposes no control that makes signalling fail.

Stop reaches the process group only. A descendant that calls `setsid` escapes it, as with
the engine's own runner. Inventory reads every committed marker under `markers/` with its
original call context. The guard fsyncs its process-group ID into the locked marker before
starting any program. State is running while locked; a free lock establishes stopped only
if no program was started or a signal-zero probe establishes that the recorded group is
absent. This saved ID is used only to prove absence, never as authority to signal a process.
If the guard alone dies while its program or descendants survive, inspection and stop report
unknown, retain identity files and an owed stop, and refuse new runs on this machine, including
after restart. An operator must terminate the survivors independently; the provider confirms
stop only after the group disappears. Unreaped group zombies can delay that confirmation.
Legacy markers without the durable witness and unreadable/unverifiable markers also remain
unknown. A restart retries inventoried stops before new work; live gates and sessions are
never recovered.

Under `durableDir`, each provider has its own mode 0700 subdirectory named by the lowercase
hex SHA-256 of the original `CONDUIT_STATE_DIR` bytes, without a trailing newline. The engine
never reuses that path for another instance. The provider takes `owner.lock` before checking
`state-dir`, which contains those original bytes. First startup writes the empty
`ledger.json` before writing `state-dir`. An existing valid ledger with no binding is retained.
A different binding, an unreadable or invalid ledger, or a missing ledger for an existing
binding refuses startup.

Keep `durableDir` fixed for an existing provider Resource. Before initializing an empty ledger,
startup refuses a non-empty `CONDUIT_STATE_DIR/runs/` because `durableDir` may have changed and
owed stops may be in the old directory. Restart with the original `durableDir`.

Startup refuses a `markers/` directory with a mode other than 0700. For an owed stop with a
missing, unreadable or mismatched `markers/<key>` entry, startup names the instance and the
entry and refuses. Recovery can finish a stop with no marker entry when the retained run
state proves the instance never launched. For a refused stop, end any survivors first, then
restore the matching marker or remove the owed-stop record from `ledger.json` before restarting.

| Place | Contents |
| --- | --- |
| Durable instance subdirectory | `owner.lock`, `state-dir`, `ledger.json` and `markers/` |
| `CONDUIT_STATE_DIR/runs/` | Run identity files and `stderr.log` in private run directories |
| `CONDUIT_STATE_DIR/instances/` | Each run's private state across starts |

The mode 0600 ledger holds the kit's run records and owed stops. Each upsert fsyncs a private
temporary file, renames it, then fsyncs its parent before acknowledging. Markers hold only
noncredential recovery context. Startup, provider calls and a one-second kit drain retry held
stops. Under the owner's 2026-11-12 rule MC4-002, an owed stop in this provider's ledger blocks
new runs through this provider until carried out. A refused start names the owed instance and
machine. A later provider Resource uses its own ledger.

PRV-021 keeps owed stops outside the engine's lifecycle. A stop still owed at provider
retirement stays in its durable subdirectory. RP1 performs no retry after retirement.
The operator independently identifies and ends any survivors, and removes the subdirectory
only once nothing is owed there. The [external drainer](../../docs/implementation/PROVIDERS.md)
is RP2 work.

## Package and verification

`cargo run --release --bin package` builds the host provider and writes
`dist/provider-local-process.tar.gz` and `dist/provider-local-process.tar.gz.sha256`.
The archive contains only the provider binary and its two YAML documents, in fixed order,
with fixed USTAR metadata and gzip headers. Identical binary/documents yield identical bytes.
It is specific to the host platform. `target/` and `dist/` are ignored.

Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and
`cargo test --release` inside this directory. Unix integration tests build a small real MCP
program, exercise the gate with freshly generated credentials, verify the certificate chain
and instance URI, kill a provider and its descendants, kill only a guard and retain the
stop obligation across restart, reopen durable state, check synthetic
environment inheritance, and block a new run behind an injected failed stop. Schema validation
explicitly skips when `spec/` is absent. A template using the exported Type also passes the
repository's semantic registration rules; removing any bound field from `required` must fail
MAN-017. This semantic test uses the repository checker's Python dependencies and interpreter
fallback. No keys or certificates are committed. Tests inspect
credential presence with boolean assertions so failures never print their values.

## D15 inventory

The four fields follow [S10's measures](../../experiments/021-s10-swapped-providers/README.md#measures).
Shared kit work is counted once in that library; this provider lists its use below.

| Field | Status and evidence |
| --- | --- |
| (a) Own operations and custom code | Observed: `src/backend.rs` implements staging, inventory, start, state and stop; `src/guard.rs` supervises groups and lifetimes; `src/storage.rs` commits the ledger; `src/lib.rs` reads startup configuration; `src/main.rs` wires the kit; `src/bin/package.rs` archives the package. File/line counts are recorded below. Tests exercise real processes through MCP. |
| (b) Delegated backing-tool operations | Observed: Unix process creation, pipes, flock, signals and file modes provide process/state mechanisms. The kit supplies MCP operations, admission and owed-stop coordination, recovery, TLS gate, token verification and stdio bridging. No external service, registry, supervisor script or container runtime is used. |
| (c) Persistent state and obligations | Observed: the owned durable subdirectory holds `owner.lock`, `state-dir`, `ledger.json` and `markers/`. Owed stops and markers survive removal of the provider's `CONDUIT_STATE_DIR`; the next Resource has a separate ledger. The engine removes `runs/` and `instances/` with the provider. Tests also reopen the ledger and run state after provider SIGKILL. |
| (d) Operator setup and recovery | Observed in tests: pinned Rust toolchain, Unix host, engine-managed state and an operator-created, owned 0700 `durableDir` outside the engine store. The operator retains the durable subdirectory until all stops are accounted for and independently ends survivors after retirement. The external drainer and separate-user deployment are RP2 work. Privileged packet-drop testing remains open. |

Own implementation counts (`wc -l`, physical lines including comments and blanks):

| File | Lines | Function |
| --- | --- | --- |
| `src/backend.rs` | 527 | Process operations, staging, inherited environment and locked inventory |
| `src/guard.rs` | 134 | Provider-loss/lifetime fence, durable group witness and signals |
| `src/storage.rs` | 368 | Durable layout, state binding and atomic ledger |
| `src/lib.rs` | 68 | Startup configuration and exported mechanisms |
| `src/main.rs` | 51 | Stdio server and internal guard entrypoint |
| `src/bin/package.rs` | 85 | Deterministic host archive |
| Total | 1233 | Kit implementation excluded |

Tests and test helpers occupy 1789 additional lines, including the 258-line TLS helper copied
from the kit's tests for certificate/URI verification and HTTP requests. The kit remains a shared
dependency, counted once as its own implementation, also intended for the later Docker provider.
