# Local machine provider

`example.org/conduit/provider-local-machine`, version 0.1.0, exports the Resource Type
`local-machine.v1`. It offers only core `machine-provider.v1`, served over MCP stdio by
`bin/provider-local-machine`. The engine's own runner starts it on `local` (PRV-002).
It has no configuration: labels are reported status, never settings.

It implements `list-machines.v1` and `place.v1`. Enrollment is not served until MP2;
there is no enrollment evidence, admission, removal or expiry handling in MP1.

The provider reports exactly one machine, the one its process runs on, always `reachable`,
never `pending`, `unreachable` or `gone`. Its name is the operating system's hostname,
trimmed and lowercased, without DNS lookup or truncation. An empty name, a name longer
than 63 characters, a non-UTF-8 name or a name that cannot be a Kubernetes label value
produces a `failed` answer with a diagnostic. No actual machine name is included here.
The only labels are `kubernetes.io/hostname`, `kubernetes.io/os` and `kubernetes.io/arch`.
OS/architecture spellings follow Go, including `darwin`, `linux`, `amd64`, `arm64` and
`loong64`. The engine's first-start report still needs the corresponding LoongArch, little-endian MIPS and big-endian ARM mapping fixes.

`place.v1` returns that machine's `hostname` if every `nodeSelector` label equals its
reported value. An empty selector matches. A missing or unequal label yields an empty
object, without `hostname`. Invalid inputs return `failed` before reading or making a
key. This follows the operation's current output schema; PRV-015's promise instead
names placement by machine ID. An engine configured with its own hostname (DL5) can
therefore disagree with this provider's OS hostname even on the same physical machine.
The engine must handle its fixed node labels and admission rules; MP1 does not admit nodes.

This provider requires Unix; the atomic key installation currently supports Linux and
macOS (other Unix targets fail explicitly if initialization needs an unsupported rename).
It requires its own state directory through `CONDUIT_STATE_DIR` (PRV-029). Missing or
empty `CONDUIT_STATE_DIR` refuses startup with a diagnostic. The engine supplies and
retains the directory across runs, then removes it on unregistration. A standalone
operator must supply it; its parent must already exist. The provider creates the
specified directory if absent, requires ownership by its OS user, refuses a symlink as
the directory itself, sets mode 0700 and fsyncs both the directory and its parent. Do not share one directory between instances.
Isolation from other processes of that OS user remains mediated.

On the first valid provider call it generates a random 32-byte Ed25519 seed with the
OS CSPRNG through `ring`. The seed stays in `machine.key`, mode 0600, in that directory;
it is never returned on the wire, logged or included in the package. The opaque,
provider-scoped `id` is `sha256:` followed by the lowercase hexadecimal SHA-256 of the
Ed25519 public key. It survives restart and changes only with a different key.
Creation writes a private temporary file, fsyncs it, installs it with an atomic
non-replacing rename, then fsyncs the directory before serving the ID. Concurrent
initializers use the installed winner. An unreadable, malformed, symlinked or insecure
existing key fails without replacement. Orphan temporary files after a crash are never
used as keys. Preserve the installed key during recovery; deleting it loses continuity.

Discovery lists Conduit's implementation-interface extension at revision `v0` and
exactly the two operation tools. A call missing that extension or naming an unsupported
revision receives MCP error -32021 before any identity work. Discovery and tool listing
do not create a key. Tool results carry Conduit's `outcome` key and consistent `isError`;
successful results carry `structuredContent`. `list-machines.v1` has no input schema in
the operation: discovery advertises `{type: object}`, accepts arbitrary object keys and
ignores them. `place.v1` uses the operation's input schema. Schema copies ship with the
crate and require no repository files at runtime.

D15's four fields, following [S10's measures](../../experiments/021-s10-swapped-providers/README.md#measures):

| Field | Inventory and evidence |
| --- | --- |
| (a) Own operations and custom code | `src/lib.rs` (196 lines): MCP discovery, extension checks, outcomes, hostname normalization, machine report and selector matching. `src/main.rs` (21 lines): required state directory and stdio lifecycle. `src/state.rs` (143 lines): seed persistence and ID derivation. `src/schemas.rs` (21 lines) and `src/schemas.json` (88 lines): standalone operation schemas. `src/bin/package.rs` (79 lines): deterministic host packaging. These 548 source/schema lines include comments and blank lines. File/line counts are reproducible with `wc -l src/*.rs src/*.json src/bin/*.rs`; these are implementation inventory, not measured S10 replay costs. Tests in `tests/mcp.rs`, `tests/schema_drift.rs` and `tests/package.rs` exercise these paths. |
| (b) Delegated backing-tool operations | Observed through the tests: `hostname` reads the OS hostname; `ring` obtains OS randomness and derives the public key; `sha2` hashes it; Rust/rustix and tempfile perform private file creation, fsync and non-replacing rename; rmcp carries MCP stdio; jsonschema checks inputs/outputs; tar/flate2 encode archives. No external daemon, coordination service or network provider is delegated work. |
| (c) Persistent state and lifecycle | Observed in restart, mode and concurrency tests: only the installed `machine.key` is needed, under the instance-owned state directory. No machine registry or placement decisions persist. Stop/restart retains the key and has no remote cleanup; runner removal/unregistration removes the directory. Crash may leave an unused private temporary file. Actual power-loss durability is untested; the implementation orders file fsync, non-replacing rename and directory fsync. |
| (d) Operator setup and recovery | Direct stdio startup with a throwaway private state directory is tested. Credentials and gate configuration are absent: stdio provider calls use the engine's own runner. Preserve the directory/key for recovery; an operator may remove unused temporary files only while stopped. Actual engine registration, runner removal and operator recovery after power loss are untested here. |

Build and check from this directory, with Rust 1.97.1:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

The packager writes `dist/provider-local-machine.tar.gz` and its `.sha256`. It builds the
host binary with the lockfile, uses fixed USTAR metadata and gzip headers, and includes
the manifest, Type contract and this contract prose. Packaging twice from unchanged
inputs produces identical bytes on the same host/toolchain. `target/` and `dist/` are ignored.
Schema tests announce a skip when `spec/` is absent in a standalone crate copy. Repository
runs compare the operation schema copies and validate the package's manifest and Type
against the current manifest/document schemas.
