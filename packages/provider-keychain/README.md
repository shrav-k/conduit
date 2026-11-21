# macOS keychain secret store

Package `example.org/conduit/provider-keychain`, version 0.1.0, supplies the Resource Type
`keychain-secret.v1`. It offers only the core `secret-provider.v1` capability, implements
`get-secret.v1` through `bin/provider-keychain` over stdio, and records neither inputs nor
outputs.

The locator is `{keychain, service, account}`. The keychain path is absolute and names a
regular file. The other two fields are the non-empty service and account attributes of
one generic password item in that keychain. All three fields are required. Extra fields
are refused.

The owner creates a dedicated file-based keychain for Conduit and configures its absolute
path. The dedicated keychain must sit in a directory only its owner can write to.
The provider resolves the locator through links and always refuses the login keychain
and the user's default keychain before opening a keychain. It also refuses files with
multiple hard links, whose other names cannot be determined from the locator. The
default-keychain path is queried read-only, and the provider compares paths and file
identities in memory without logging.

Each request disables user interaction, checks that it is off, and reopens the selected
keychain. A read never waits for a person. The value comes from that item alone and is
returned as UTF-8, up to 65,536 bytes, with every byte preserved. The provider discards
the value if it cannot verify the file's identity again after the read. The provider
caches no values and logs none. A failed read returns a fixed explanation and no value.
This store serves the owner's logged-in macOS session. Daemon operation is open under A22. A binary
built for another operating system serves discovery and reports failed for a well-formed
read.

The owner adds the item to the dedicated keychain with Apple's tools and authorizes the
provider binary by naming it with `-T <the binary>`. For a dedicated keychain in the
partitioned format, the owner also adds the binary's `cdhash:` to the item's partition list.
A rebuild that changes the code-directory hash
loses access, so each such upgrade repeats the owner's authorization step A63-OKC.
Experiment 005 tested the rebuild and the two keychain formats. The package tests use
throwaway keychains and authorize the development binary at its test path. Authorizing
the copy the engine installs at its own path is outside the package tests.
[Experiment 005](../../experiments/005-macos-keychain/README.md#what-it-means-for-a-macos-keychain-provider-package)

Conduit dispatches the operation as the reading instance through its own Conduit. Discovery
advertises the implementation-interface extension at revision `v0`. The provider checks
that extension and the engine's request metadata. A nonempty filter list returns
`unavailable` with the first restriction before opening a file. The engine supplies
authorization, delivery, and recording.

## D15 inventory

| Field | Status and evidence |
| --- | --- |
| (a) Own operations and custom code | Locator checks and the MCP surface are in [src/lib.rs](src/lib.rs). Selected-keychain reads and fixed failure outcomes are in [src/keychain.rs](src/keychain.rs). [MCP tests](tests/mcp.rs) and [keychain tests](tests/keychain.rs) exercise the binary. |
| (b) Delegated backing-tool operations | Apple's Security framework performs explicit-file generic password reads. rmcp handles MCP over stdio. The deterministic archive builder uses tar, flate2, and sha2. |
| (c) Persistent state, ownership and stop/restart | The owner retains the keychain. The provider reopens it each time and creates no persistent state. The keychain tests cover item updates and the lock and unlock of a throwaway. |
| (d) Operator setup, credentials, gate and recovery | The owner configures the locator and authorizes the binary in the logged-in session. A locked keychain must be unlocked by its owner. Re-run A63-OKC when the binary's hash changes. The package tests exercise local authorization and leave engine integration for S2-3. |

The crate adapts [provider-file](../provider-file/README.md)'s discovery, extension and
metadata checks, archive builder, and schema drift tests. It has no repository crate
dependency.

## Build and check

Run one Cargo command at a time with the pinned Rust 1.97.1 toolchain:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

The builder writes `dist/provider-keychain.tar.gz` and its SHA-256 digest. Archive tests
compare two builds. The schema drift tests validate the package documents and compare the
copied output schema with the core contract when the specification is present.

The macOS tests use only throwaway keychains created inside their own temporary
directories. They add and update items through the Security framework with interaction
disabled in the process making the changes. They check that the user's search list and
default setting match before and after, compare generated markers by digest, and require
empty provider stderr. The locator tests cover the login basename and an alias to that
basename. A unit test uses temporary stand-in files to check default-path matching,
including symlinks, case variants and hard links.
