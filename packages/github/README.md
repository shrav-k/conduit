# Read one GitHub repository and create and merge its pull requests

Package `example.org/conduit/github`, version 0.2.0, exports `read` and `write`
Conduit templates and separate repository and pull-request Resources.
The read Resource is `repository.v1`, with two required configuration keys.
`repository` is `owner/name`: the owner uses 1–39 ASCII letters, digits or hyphens, and the name uses
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

## Create and merge pull requests

The `pull-requests.v1` Resource requires exactly `repository`, `base` and `token`.
`repository` has the read Resource's owner/name form. `base` and a create's `head`
are branches of 1–255 ASCII letters, digits, dots, underscores, slashes or hyphens.
They cannot start with `-`, `/` or `.`, end with `/`, `.` or `.lock`, or contain
`..`, `//`, `/.` or `.lock/`. `token` references the separate write secret Resource.
The Pool owns both Resources. The writer Resource declares its own Conduit to its
secret Resource listing core `get-secret.v1`.

Only root's Conduit from `example.org/conduit/github#write` should name the writer,
no Agent's. This is the project's declaration: neither the engine nor this package
enforces a root-only writer. The engine admits operations listed by an open Conduit;
the package adds no caller check. Closing that write Conduit ends later writes
through it. Inputs and outputs of both writes are recorded.

At registration, the owner names the one repository in the `pull-requests.v1`
Resource's `repository` argument. Create a second fine-grained personal token for
that one repository alone, with Contents and Pull requests read and write and an
expiry. Pull requests write lets it create a pull request; Contents write lets it
merge. GitHub also adds Metadata read. Keep the token in the dedicated keychain
through the [keychain store](../provider-keychain/README.md), or in a regular 0600
file through the [file store](../provider-file/README.md). Leave the read token as
it is ([DEC-173](../../docs/decisions/log.md#dec-173-2026-11-23-merging-through-conduit-uses-a-second-fine-grained-github-token)).

The owner's ruling of 2026-11-24, DEC-212, is:
"Contents + PR write, this repo (Recommended)". It lands as "the second fine-grained
token holds Contents write and Pull requests write on this repository alone,
held as a secret Resource that only root's write Conduit reaches". The writer
Resource reaches the secret through its own declared Conduit, as described above;
root's write Conduit targets the writer. Contents write also lets whoever holds the token
push to any branch the repository's rules allow, not only merge.
An unconfined v0 Agent can reach the token's store with its own tools
([DEC-018](../../docs/decisions/log.md#dec-018-2026-11-20-what-an-agent-reaches-enforcement-in-v0-after-s2)).

The `writer` item runs `bin/conduit-github --write`, lists only the two write
operations and receives only its own secret as `GITHUB_PERSONAL_ACCESS_TOKEN`.
There is no credential fallback. Missing or empty credentials, invalid arguments or
configuration, identical head and base, and arguments or configured names containing
the credential fail before any server starts. A rotated token arrives on the next run.
The writer uses the same lifetime supervisor as the reader and gives its server only
`PATH`, `HOME` and that token. Its exact server arguments are:

```text
stdio --tools list_pull_requests,create_pull_request,pull_request_read,merge_pull_request
```

There is no `--toolsets` or `--read-only` for this server. A release without `--tools`
refuses the flag, so every write fails. No call can select other flags or tools.
The following table is checked once per server, before any tool call, including every
listed property. A mismatch fails every write naming the tool, property where missing,
and server version. This table and flag remain untested against a real server; the
owner checks the installed release at the first live merge.

| Tool | Arguments |
| --- | --- |
| `list_pull_requests` | `owner`, `repo`, `state: "open"`, `head: "OWNER:HEAD"`, `base` |
| `create_pull_request` | `owner`, `repo`, `title`, `head`, `base`, `body` |
| `pull_request_read` | `method: "get"`, `owner`, `repo`, `pullNumber` |
| `merge_pull_request` | `owner`, `repo`, `pullNumber`, `merge_method: "squash"`, `commit_title`, `commit_message`, `expectedHeadSha` |

`owner`, `repo` and `base` come from each call's configuration. No other argument is
sent, and no call is retried. Create takes `head`, `title` (1–256 characters without
line breaks) and `body` (at most 65,536 characters). It first lists open pull requests
from that head into the base. A failed or malformed list, or an existing open pull
request, fails with nothing created. A create reply succeeds only with a page URL of
that repository, compared without regard to ASCII case, and returns `{number, url}`.
After an `isError`, it lists again: none open means failed with the server's text;
one open, or a list it cannot read, means unknown, naming the number where known.

Merge takes `number`, `expectedHead` (40 lower-case hexadecimal digits), `title`
(1–1,024 characters without line breaks) and `message` (at most 65,536 characters).
It reads first and fails with nothing merged if the read lacks state, merged status,
head SHA or base ref, or if the pull request is already merged, closed, targets another
base or has another head. It requests squash with `expectedHeadSha`: GitHub checks
`expectedHead` again at the merge, so a head that moved after the read is refused.
Only `merged: true` and a 40-digit lower-case SHA confirm success, returning `{sha}`.
After `isError` it reads again: not merged means failed with the server's text and,
if moved, the new head and a reminder to gate it before merging again. A readable
re-read that shows merged reports that observation with an unknown outcome; a
failed or malformed re-read reports that it could not establish the effect.

Any other write reply, timeout, server exit, answer over 8 MiB, or stop after dispatch
means unknown. A stop before a write is called means failed; an ended session may
return nothing. Unknown diagnostics direct the caller to `pull-request.read.v1` or
`pull-request.list.v1` through the read Conduit. Failed writes also direct the
caller to read before trying again. Neither operation declares
`deduplicates`: an unchanged repeat of an unknown invocation is unknown and the
engine does not send it again (OPS-012). Failed and unknown results set `isError`;
only a filter refusal gives a reason. Server error text is redacted and bounded to
2 KiB, and server stderr is discarded. No response or diagnostic contains the token.

## D15 inventory

The fields follow [S10's measures](../../experiments/021-s10-swapped-providers/README.md#measures).

| Field | Status and evidence |
| --- | --- |
| (a) Own operations and custom code | [src/lib.rs](src/lib.rs) implements the reader; [src/writes.rs](src/writes.rs) implements the writer. Together they serve six operations, validate the configuration and calls, check the upstream tables, manage the child, and bound and redact replies. [tests/mcp.rs](tests/mcp.rs) exercises this surface over stdio. |
| (b) Delegated backing-tool operations | GitHub's server performs issues and pull-request reads and the writer's create and squash merge. rmcp handles stdio MCP and Tokio handles child I/O and signals. Tests replace the backing binary with a local stand-in. |
| (c) Persistent state, ownership and stop/restart | Each front, reader or writer, holds its own token only for its run and creates no persistent files. The Pool owns both secret Resources, the read token's and the write token's. Stand-in tests verify cleanup and the next-call restart. |
| (d) Operator setup, credentials, gate and recovery | The owner installs the external server and supplies separate scoped tokens through each reader or writer Resource's secret Conduit. The loopback engine test exercises delivery with the file store. Real GitHub access and owner authorization remain for live qualification. |

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
