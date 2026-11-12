# Local directory package source

`example.org/conduit/provider-local-directory`, version 0.1.0, exports the Resource
Type `local-directory.v1`, offering only core `package-source.v1`. This provider requires
Unix directory handles with `O_PATH` or `O_SEARCH` (including Linux and macOS) and
serves `fetch-package.v1` over MCP stdio through `bin/provider-local-directory`.
It claims only `file`, compared without regard to case, for its whole life. There
are no controls. Its configuration requires `schemes: [file]` and `root`, an absolute
directory; extra properties are rejected. Duplicate `file` entries are harmless
and permitted by the Type's schema. Each call reads configuration from its own
`io.github.shrav-k.conduit/config` metadata (PRV-005).

The provider is owned by `system`, placed `local`, and run by the engine's own runner
(PRV-002, PRV-013). Once registered, it is the engine's only fetcher of `file` URLs
(PRV-017). **Every file install, resolved relative fetches included, must lie under
`root`: one root per engine while this provider stands.** To change the scheme claim,
retire the provider and register another (PRV-016). The engine resolves relative
manifest fetches to `file:` URLs before dispatch; the provider rejects a relative
reference received directly.

URL mapping uses `url::Url::to_file_path`, as the engine does, including percent
decoding. Only empty hosts and `localhost` are accepted. Both `root` and the target
are canonicalized with every symbolic link followed; containment is compared by path
components, so a sibling named `root2` is outside `root`. Links into the root work;
links out fail. After canonicalization, descriptor-relative opens refuse replacement
symlinks. Nonblocking opens prevent a FIFO from hanging the call, and the opened file
must be regular. Source files should remain stable during reads; the provider does
not make a snapshot of concurrent writes.

Directory handles use `O_PATH` where available, or `O_SEARCH`, so ancestors, root and
intermediate directories need traversal permission without directory-listing permission.
The final archive still requires read permission. Unix targets without either kind of
traversal-only directory handle are rejected at build time.

The largest archive accepted is **8,388,608 bytes (8 MiB)**, held in memory. The read
probes at most one further byte to detect growth past the bound. RFC 4648 standard padded base64 grows it to
11,184,812 encoded bytes at that limit; MCP serialization and buffers add memory
overhead. The returned `archive` decodes byte for byte to the file. Bytes are unchecked:
corrupt archives and digest mismatches are returned unchanged. The engine checks
the digest and archive validity (PKG-026). The provider keeps nothing: no archive
cache, state directory, copied credentials or persistent state.

The 8 MiB bound leaves room for base64 and result framing within rmcp's default
16 MiB incoming stdio message limit.

Outside-root paths, unsupported schemes, relative references, other hosts, user
information (credentials), missing or unreadable files, non-regular files, oversize
files and invalid configuration return `failed`, with `isError: true` and an explicit
diagnostic naming the reason. Diagnostics never include file bytes or echo fetch
URLs or credentials. Discovery lists Conduit's implementation-interface extension at
revision `v0` and exactly one operation tool. A call missing the extension or naming
another revision gets MCP error -32021 before any file work. Success carries
`structuredContent`, `isError: false`, and Conduit's `outcome: succeeded` metadata.

## Build and test

From this directory:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

The package builder writes `dist/provider-local-directory.tar.gz` and
`dist/provider-local-directory.tar.gz.sha256`. It builds the host executable with
the locked dependencies, uses deterministic USTAR metadata and a zero gzip timestamp,
and includes only the executable, manifest, contracts and this README. `dist/` and
`target/` are ignored. The archive test builds twice and compares every byte and digest.
The package is for the Unix OS/architecture on which it is built.

Tests launch the actual binary with an `rmcp` client and disposable directories.
They cover discovery, refusal before reads, exact and corrupt bytes, percent decoding,
case-insensitive scheme comparison, localhost, internal and escaping symlinks, root
canonicalization, component containment, all documented refusals, the exact size
boundary, fresh reads and per-call configuration. A non-root test reads an archive
through an ancestor, root and nested directory with execute-only permissions and verifies
that directory listing fails. This and the unreadable-file case explicitly print skips
when run as root. Copied operation schemas validate successful answers;
a drift test compares copies with `spec/`. Package documents validate against
`spec/schemas/`; those repository comparisons explicitly print skips when `spec/` is
absent. An independent package-shape test always checks required schemes, their exact
`file` constraint, required root, the implementation mapping, records and no controls.
Package tests exercise this provider directly; engine dispatch and S10 integration
remain outside this package's evidence.

## D15 inventory

The four fields follow [S10's measures](../../experiments/021-s10-swapped-providers/README.md#measures).

| Field | Status and evidence |
| --- | --- |
| (a) Own operations and custom code, by file/lines and function | Observed source and direct MCP tests: `src/lib.rs` implements URL refusal, configuration, canonical containment, bounded reads, base64 mapping, discovery and outcomes; `src/main.rs` serves stdio. `src/schemas.rs` and `src/schemas.json` carry copied operation validation. `src/bin/package.rs` builds the distributable. Exact line counts are recorded below. |
| (b) Backing-tool operations delegated | Observed in direct tests: Unix filesystem canonicalization, permission enforcement, descriptor-relative opens, metadata and reads through std/rustix; URL parsing/decoding through url; encoding through base64; MCP transport through rmcp; schema validation through jsonschema; packaging through Cargo, tar, flate2 and sha2. No separate daemon, supervisor or setup script. |
| (c) Persistent state, location/owner and stop/restart obligations | Absent provider state: no archive cache or state directory. Root and source archives are operator-owned input, retained by the operator across stop/restart. Each call rereads its source; tests replace and delete files between calls. Nothing is created or deleted in root by the provider. Engine restart integration is untested here. |
| (d) Operator setup, credentials, gate configuration and manual recovery | Disposable-directory setup observed in tests: provide an absolute root and pinned archive files, with read and directory traverse permissions for the provider OS user. Credentials and gate configuration absent: the engine's own runner reaches the provider over stdio. Manual recovery consists of restoring missing files/permissions; production recovery and engine registration are untested. |

Line counts count physical lines including comments and blanks; schema JSON is listed
apart from Rust:

| File | Lines |
| --- | ---: |
| `src/lib.rs` | 259 |
| `src/main.rs` | 13 |
| `src/schemas.rs` | 21 |
| `src/bin/package.rs` | 79 |
| `src/schemas.json` | 66 |

The MCP shell, packaging and schema-copy/drift scaffolding were copied
and adapted from `packages/provider-local-machine/` to keep this crate independent of
repository crates. These are duplicated source files, not a shared library; the D15
inventory discloses the duplication rather than treating it as new provider logic.
