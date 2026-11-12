# File secret store

Unix-only package `example.org/conduit/provider-file`, version 0.1.0. Its Resource Type
`file-secret.v1` offers only the core `secret-provider.v1`, implements `get-secret.v1`,
lists no controls, and records neither inputs nor outputs. The engine's own runner starts
`bin/provider-file` on the machine holding the file and connects over stdio.

The locator is the instance configuration `{path: <absolute path to one file>}`.
No extra configuration keys are accepted. Each request opens that file only; the store
never takes a value from configuration, environment variables or another file, and has
no fallback. There is no version, stage or structured-key selection.

The file must be readable by the process, regular, and have `mode & 0o077 == 0`:
no group or other read, write or execute permissions. Mode 0600 or 0400 is suitable
when the process can read the file. The store opens with `O_NOFOLLOW`, then checks the
opened descriptor with `fstat` before reading. A symbolic link as the final component
is refused. Parent-directory links are allowed; paths are never compared against a
canonical spelling. Nonblocking open allows FIFOs to be refused without waiting.
Docker secret mounts at 0444 and Kubernetes secret mounts using links or 0644 are
refused. Copying a mounted secret to a private regular file, outside this provider,
requires the operator to manage that copy's permissions, replacement and removal.

The largest accepted file is **65,536 bytes (64 KiB)** before UTF-8 decoding or newline
removal. A read consumes at most 65,537 bytes, including one byte to detect overflow
if the file grows after inspection. The response's `value` is UTF-8 with exactly one
final `\n` removed. A second newline remains, as does a `\r` preceding the final
newline. Empty files give an empty value. Missing, unreadable, nonregular, non-UTF-8,
oversized and improperly permissioned files give `failed`, an explicit safe message
and no value. Read buffers and decoding errors are never logged or formatted.

The store retains no copy, cache or persistent state and logs no value. Each read
reopens the locator, so a replacement is visible on the next call, and removal fails
the next call. This makes no claim about clearing process memory or crash dumps.
The original secret file remains under the operator's control.

`get-secret.v1` arrives as a request through the reading instance's own Conduit, rather
than a provider call. Its `_meta` carries `instance`, `operation`, `invocation`,
`caller`, `filters`, `config`, `revision` and `conduit` under Conduit's prefix.
Discovery advertises the implementation-interface extension at revision `v0`;
missing or unsupported declarations answer -32021 before any read. This store
supports no filters: a nonempty list returns `reason: unavailable`, the first
restriction unchanged, `isError: true` and no outcome, before reading anything.
The MCP tool is used for engine dispatch; it does not authorize actors to read secrets.
The engine supplies authorization, delivery and its recording obligations.

## D15 inventory

The fields follow [S10's measures](../../experiments/021-s10-swapped-providers/README.md#measures).
Evidence here is package tests; engine integration and the shared store suite are untested.

| Field | Status and evidence |
| --- | --- |
| (a) Own operations and custom code | Observed: [src/lib.rs](src/lib.rs), `read_secret` (line 23) implements the locator, descriptor checks, bounded read and newline rule; `answer` (line 81) forms safe outcomes; `FileServer::{get_info, discover, list_tools, call_tool}` (lines 97 onward) implements the MCP surface, request metadata checks and filter refusal. [src/main.rs](src/main.rs), `main` (line 6), starts stdio. [src/bin/package.rs](src/bin/package.rs), `run` (line 10), builds the archive and digest. [tests/mcp.rs](tests/mcp.rs) exercises these paths through the binary. |
| (b) Delegated backing-tool operations | Observed: Unix `open`/`fstat` through rustix, descriptor reads and UTF-8 conversion through Rust std; rmcp handles MCP/stdio, Tokio schedules it, serde_json encodes it; tar, flate2 and sha2 build and digest archives. These are external crates, with no repository crate dependency or backing CLI. The copied MCP/archive/schema-test scaffolding from `provider-local-machine` is disclosed below. |
| (c) Persistent state, ownership and stop/restart | Provider state absent: the source file is operator-owned at the configured absolute path; the provider creates no state directory or second file. No stop/restart obligations for provider state. Reopening on every call, replacement and removal are observed in `utf8_newline_boundaries_and_no_cache`; `parent_directory_symlinks_are_allowed` covers linked parents. Packaging alone writes ignored `dist/` artifacts. |
| (d) Operator setup, credentials, gate and recovery | Observed in tests: create a private regular UTF-8 file, set its permissions, and supply its absolute locator. No store credential, login session, network endpoint, setup script, supervisor or gate configuration is needed with the engine's own runner over stdio. On failure, repair the file or permissions, or replace it at the same path; the next request reopens it. A new locator requires a new Resource. Engine-run setup and end-to-end recovery remain untested here. |

The independent crate copies and adapts the small MCP/discovery/result construction,
deterministic archive builder, package test and YAML schema-reference converter from
[provider-local-machine](../provider-local-machine/README.md). It adds request metadata
handling for secret reads and has no machine-provider state or calls. This duplication
is deliberate under work order 132's prohibition on repository crate dependencies.

## Build and check

Rust is pinned to 1.97.1. Run one Cargo command at a time from this directory:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

The package builder writes `dist/provider-file.tar.gz` and its `.sha256` digest with
fixed USTAR metadata, gzip timestamp and archive entry order. Repeated builds on the
same host with the same binary must produce identical bytes; no cross-host binary
reproducibility is claimed. `target/` and `dist/` are ignored.

Tests use synthetic markers created at run time and assertions that never display
them. They capture the child's stderr and require it to be empty, validate successful
answers against the copied [output schema](src/output-schema.json), and compare the
copy to `get-secret.v1` when `spec/` is present. They validate both package YAML files
against `spec/schemas/` and announce skips if `spec/` is absent. The unreadable-file
test announces its skip when run as root. Refusal tests check the file's access time
as additional evidence of no read; access-time tracking depends on the filesystem.
The archive test builds twice and compares both bytes and digest, and checks metadata
and contents. These tests establish package behavior, not the engine's secret delivery.
