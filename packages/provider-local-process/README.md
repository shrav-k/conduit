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
Start `target/release/provider-local-process` with `CONDUIT_STATE_DIR` naming its own state
directory, private to its OS user (0700). The parent directory must exist. An absent or empty
variable refuses startup with an explicit diagnostic. Only one provider process may own that
directory at once. Keep it outside transient package installations and retain it after retirement.

The manifest binds these configuration fields to startup variables. Values are read once;
a configuration change takes effect at the provider's next start. Calls still carry their
current configuration and revision for the kit's durable recovery context. All six fields
are required by the Type schema as MAN-017 requires for environment bindings; their defaults
still supply omitted configuration values.

| Field | Startup variable | Default |
| --- | --- | --- |
| `bindAddress` | `LOCAL_PROCESS_BIND_ADDRESS` | `127.0.0.1` |
| `advertisedHost` | `LOCAL_PROCESS_ADVERTISED_HOST` | `127.0.0.1` |
| `keepaliveIdle` | `LOCAL_PROCESS_KEEPALIVE_IDLE` | 30 seconds |
| `keepaliveInterval` | `LOCAL_PROCESS_KEEPALIVE_INTERVAL` | 5 seconds |
| `keepaliveCount` | `LOCAL_PROCESS_KEEPALIVE_COUNT` | 3 probes |
| `maxLifetime` | `LOCAL_PROCESS_MAX_LIFETIME` | 3600 seconds |

The bind IP must identify an interface; unspecified addresses are refused. The advertised host
is a bare IP or DNS name. Keepalive and lifetime values must be positive whole seconds, and
counts positive integers. The keepalive silent-cut bound must fit signed 32-bit milliseconds.
The kit's unchanged buffer/message defaults are 256 queued messages and 8 MiB per message.

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
reads its contents and never removes this directory: it receives no unregistration notice.

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

`ledger.json`, mode 0600, holds the kit's run records and owed stops in one file. Each upsert
writes a private temporary file, fsyncs it, renames it, then fsyncs the parent directory before
acknowledging. Markers hold only noncredential recovery context, and `owner.lock` excludes
simultaneous provider owners. Startup, provider calls and a one-second kit drain retry held
stops. Under the owner's 2026-11-12 rule MC4-002, a held stop is carried out before any new
run on its machine starts; a refused start names the owed instance and machine. This provider
has one machine, so one owed stop blocks every new run until carried out.

RP1 has no external drainer after provider retirement, separate OS users or registry packages.
Owed stops remain on disk after retirement, but nothing outside a running provider retries
them yet. Retain the state directory and restart the provider to retry, or arrange the RP2
external drainer when available. Do not delete markers to claim a stop completed.

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
| (c) Persistent state and obligations | Observed: provider-user-owned state directory, 0700; `ledger.json`, `markers/`, `owner.lock`, transient `runs/` and retained `instances/`. Tests reopen the ledger and preserve instance state after provider SIGKILL. Guards fence groups on loss; confirmed stop removes private identity files. External retirement drainer and separate users are absent in RP1. |
| (d) Operator setup and recovery | Observed in tests: pinned Rust toolchain, Unix host, private provider state directory, startup variables above, engine-provided identity and secrets. Operator retains state across provider installations/retirement and makes the configured engine hostname match the OS hostname. Separate-user deployment, privileged packet-drop testing and external retirement recovery are untested/absent as stated above. |

Own implementation counts (`wc -l`, physical lines including comments and blanks):

| File | Lines | Function |
| --- | --- | --- |
| `src/backend.rs` | 449 | Process operations, staging, inherited environment and locked inventory |
| `src/guard.rs` | 134 | Provider-loss/lifetime fence, durable group witness and signals |
| `src/storage.rs` | 88 | Private directories and atomic durable ledger |
| `src/lib.rs` | 68 | Startup configuration and exported mechanisms |
| `src/main.rs` | 47 | Stdio server and internal guard entrypoint |
| `src/bin/package.rs` | 85 | Deterministic host archive |
| Total | 871 | Kit implementation excluded |

Tests and test helpers occupy 1254 additional lines, including the 258-line TLS helper copied
from the kit's tests for certificate/URI verification and HTTP requests. The kit remains a shared
dependency, counted once as its own implementation, also intended for the later Docker provider.
