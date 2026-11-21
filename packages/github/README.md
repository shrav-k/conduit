# Read one GitHub repository

Package `example.org/conduit/github`, version 0.1.0, exports a `repository.v1` Resource
and a `read` Conduit template. The configuration has two required keys. `repository`
is `owner/name`: the owner uses 1–39 ASCII letters, digits or hyphens, and the name uses
1–100 ASCII letters, digits, dots, underscores or hyphens. A name of `.` or `..` is
refused. `token` is a reference to a secret Resource.

The Pool owns the token Resource and every repository Resource. Each repository
declares a Conduit to that token Resource listing core `get-secret.v1`. During setup
the engine reads the token through the repository's Conduit and delivers it to the
front as `GITHUB_PERSONAL_ACCESS_TOKEN`. A reader can use
`from: example.org/conduit/github#read` or list fewer operations on its own Conduit.

For live use, the owner installs `github-mcp-server` from GitHub's releases on the
`PATH` used by `conduit serve`. The package digest covers the front, manifest,
contracts and README. The external binary is resolved on that path at run time
(MAN-010). The owner makes a fine-grained personal token for the chosen repositories
with read-only Issues, Pull requests and Metadata permissions and an expiry. Store
it in the owner's dedicated keychain or a regular file of mode 0600 through the
corresponding [keychain store](../provider-keychain/README.md) or
[file store](../provider-file/README.md).

The front starts one child on the first eligible call with
`stdio --read-only --toolsets issues,pull_requests` and supplies only its `PATH`,
`HOME` and `GITHUB_PERSONAL_ACCESS_TOKEN`. Configuration fixes `owner` and `repo`
on each call. Input and configuration validation reject extra keys and reject an
owner or repository name containing the active credential. Discovery declares
Conduit's implementation-interface extension at `v0`. The front refuses filters
it cannot apply and omits restriction details that contain the credential.

| Operation | Upstream tool | Upstream arguments |
| --- | --- | --- |
| `issue.read.v1` | `issue_read` | `method: get`, `owner`, `repo`, `issue_number` |
| `issue.list.v1` | `list_issues` | `owner`, `repo`, `perPage` |
| `pull-request.read.v1` | `pull_request_read` | `method: get`, `owner`, `repo`, `pullNumber` |
| `pull-request.list.v1` | `list_pull_requests` | `owner`, `repo`, `perPage` |

The table has been exercised against a stand-in. Checking it against the owner's
installed GitHub release is part of live qualification. The front checks every
listed tool and argument before forwarding a call, and diagnoses mismatches
with the reported version. Reads take a positive `number`. Lists take a `limit`
of 1–100 and return the first page in the server's default state and order.
The server's JSON object or array is returned in `data`. Tokens in returned strings
are replaced with `[token]`. The front bounds answer text at 8 MiB, redacts and bounds
forwarded error text, and discards child stderr.

The front shuts down on stdin EOF, SIGTERM, SIGINT, SIGHUP or SIGQUIT. A supervisor
reaps the server when the front's private lifetime socket closes, including
after an abrupt front exit.
An exited child is replaced on a subsequent call. A token change takes effect when the
engine starts a fresh front. No Conduit gives an Agent the token. An unconfined v0 Agent
can reach its store using its own tools, as described in
[DEC-018](../../docs/decisions/log.md#dec-018-2026-11-20-what-an-agent-reaches-enforcement-in-v0-after-s2).

## D15 inventory

The fields follow [S10's measures](../../experiments/021-s10-swapped-providers/README.md#measures).

| Field | Status and evidence |
| --- | --- |
| (a) Own operations and custom code | [src/lib.rs](src/lib.rs) implements the four tools, validates the repository configuration and calls, checks the upstream table, manages the child, and bounds and redacts replies. [tests/mcp.rs](tests/mcp.rs) exercises this surface over stdio. |
| (b) Delegated backing-tool operations | GitHub's server performs issues and pull-request reads. rmcp handles stdio MCP and Tokio handles child I/O and signals. Tests replace the backing binary with a local stand-in. |
| (c) Persistent state, ownership and stop/restart | The front holds the token only for its run and creates no persistent files. The Pool owns the secret Resource. Stand-in tests verify cleanup and the next-call restart. |
| (d) Operator setup, credentials, gate and recovery | The owner installs the external server and supplies the scoped token through the repository's secret Conduit. The loopback engine test exercises delivery with the file store. Real GitHub access and owner authorization remain for live qualification. |

## Build and check

Rust is pinned to 1.97.1. Run one Cargo command at a time from this directory.

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --offline --locked --release --bin package
```

Packaging writes `dist/github.tar.gz` and its SHA-256 digest. The archive contains the
front and the package documents. The stand-in is available to the test harness as
a separate build target. It opens no socket.
