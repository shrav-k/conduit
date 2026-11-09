# Experiment 008 harness

A harness for the [pre-registration](README.md), not an engine or conformance evidence.
No real client has been run to produce these files. The worker's checks use a scripted HTTP
client only. The worker leaves the admin's amended pre-registration and the specification unchanged.

## Run

From this directory:

```sh
cargo run --release -- --dry-run
cargo run --release -- --output /tmp/conduit-008-evidence-2026-11-09
```

The actual command launches agents and spends model tokens; the admin runs it. Its output
must be an absolute path outside this repository. Existing run directories are refused, so
use a fresh output root for a new matrix. Existing output roots can hold disjoint subsets.
A normal cell selects all three repetitions; a 401 cell selects its single run:

```sh
cargo run --release -- --dry-run --only claude-C3-cache-off-ttl-0-auth-401
cargo run --release -- --output /tmp/conduit-008-subset --only codex-C5-cache-off-ttl-0-auth-valid
```

There are 24 cells and 68 sequential runs: 16 C1/C5 cells and 6 C2–C4 base cells,
three repetitions each, plus 2 C3 401-first cells, one run each. Only C1 and C5 vary
cache on/off and TTL 0/60,000. The base variant is cache off, TTL 0; the 401 runs use it too.
Budget about 68 minutes, an estimate rather than a measured duration. Each client retains a
180-second deadline, adjustable with `--timeout-secs`; 68 timeouts alone take 204 minutes.
The stub runs in the runner's process; the dry run prints
its equivalent standalone command, the generated configurations, the version command and the
client command. It runs no command, binds no socket and creates no output files. The port in
that plan is a placeholder; a real run binds port zero and uses the socket's assigned port.
Actual runs, including standalone `--serve`, require `--output`: there is no default output
directory. Without `--output`, a dry run uses the literal `<output>` placeholder only.

Needs: rustup with Rust 1.97.1, a C compiler for `ring`, crates available locally or network
access for Cargo, Claude Code 2.1.318 and Codex 0.167.0 on PATH, and each client's existing
owner subscription login. The runner records each installed version; compare it with those
versions before interpreting the records. The runner does not read or copy the owner's
login files, relocate either client's configuration home, or write owner configuration.
Both clients are launched to use their existing subscription login through the clients
themselves, with the owner's settings excluded by flags. No fresh Claude configuration directory or trust record
is created. `ANTHROPIC_API_KEY` and `OPENAI_API_KEY` are removed from every client command's
environment, including version checks. Authentication failures are retained as failures,
not results of the five MCP cases.

Every run writes `authentication.json`, which records client-reported mode/source fields,
their JSON pointers and line numbers in `events.jsonl`. Claude's system initialization event's
`apiKeySource` is copied verbatim. Codex startup/authentication metadata is inspected for
`authMode`, `auth_mode`, `authenticationMode` or `apiKeySource`. If the client does not expose
these, the report explicitly says authentication was not reported; it does not infer a login
from the requested setup, missing API-key variables or model text. The admin should inspect
the raw event cited before accepting subscription authentication as established.

## Scenario and fixed prompts

The stub serves dummy files from memory: `hello.txt`, `other.txt`, and, after opening `notes`,
its `hello.txt`. `fail.txt` produces failed. A write returns unknown without changing the dummy
files: uncertainty is injected at the wire, not measured against a real filesystem.
The test control is modeled as a third, always-open Conduit `control` listing `next`.

| Case | State 0: open Conduits and tools | First `control.next.v1` | State 1: open Conduits and tools | Requested sequence |
| --- | --- | --- | --- | --- |
| C1 | `files`: `files.read.v1`, `files.write.v1`; `control`: `control.next.v1`; `notes` closed | Close `files`; notify | `control` alone; a cached `files.read.v1` call reports refused | Read `hello.txt`, step, read `hello.txt` again |
| C2 | Same as C1 state 0 | Keep tools; no notification | Same as state 0 | Call `never.presented.v1` with `{}` |
| C3 | Same as C1 state 0 | Keep tools; no notification | Same as state 0 | Read `fail.txt`: failed |
| C4 | Same as C1 state 0 | Keep tools; no notification | Same as state 0 | Write `hello.txt` with text `replacement`: unknown |
| C5 | Same as C1 state 0 | Open `notes`; notify | `files`, `notes` and `control`: add `notes.read.v1` | Step, then read `hello.txt` with `notes.read.v1` |

The code in `Cell::prompt` fixes the full prompts, including the request to report what happened.
No follow-up prompt is generated from a result. Repeated steps leave state 1 unchanged, with no
additional list-change notification. A missing file also produces failed. Only calls to a
presented tool can start an invocation; unknown and closed tool names have no invocation.

The tool names follow IFC-012: the Conduit's local name and the operation's export name,
including its version segment, joined by a dot: `files.read.v1`, `files.write.v1`,
`notes.read.v1` and `control.next.v1`. The admin settled the previous unversioned-name
contradiction before any run; it is no longer an open finding.

## Protocol and admission

The official Rust SDK, `rmcp` 3.9.0, supplies Streamable HTTP, legacy initialization and
sessions, modern discovery and subscriptions, protocol validation, and SSE framing. Its
model types allow every needed field. Axum middleware supplies per-request JWT admission,
Origin checks, wire logging and Conduit's reason on authenticated protocol errors. The stub
uses the SDK's typed tool/result objects rather than implementing JSON-RPC transport itself.

| Revision | Lifecycle and change stream | Lists | Unknown or closed tool |
| --- | --- | --- | --- |
| 2025-06-18 | `initialize`, session ID, HTTP GET SSE | Private cache and TTL hints emitted as additive fields; that revision assigns them no cache semantics | JSON-RPC -32602, reason refused in error data, no tool result or `isError` |
| 2025-11-25 | Same legacy lifecycle | Same additive hints | Same error |
| 2026-07-28 | `server/discover` or inline request metadata; `subscriptions/listen`, acknowledgment and subscription IDs; no legacy session | `resultType`, `cacheScope: private`, `ttlMs` | Same error, as IFC-012's baseline requires |

The SDK lists exactly those three revisions. Logs distinguish offered and negotiated legacy
versions, and each modern HTTP request's protocol header. Discovery lists Conduit's extension
at revision `v0`. No unchanged client must declare that extension merely to use ordinary tools.
A modern request carrying Conduit's keys without declaring it is rejected with HTTP 400
and -32021. Protocol errors keep their code and HTTP status, with reason invalid added to
authenticated error data (IFC-036), except unsupported-version negotiation errors, which carry
no Conduit reason and are not recorded as invalid.
An invalid Origin is answered with 403. No HTTP Authorization value is logged.

Each stub generates an in-memory ES256 signing key and a JWT-SVID with `kid`, `sub`, `aud` and
`exp`. The consumer ID is `spiffe://experiment.test/conduit/instance/consumer-1`; the synthetic
canonical audience is `https://localhost:<port>/`. Verification checks the signature, algorithm,
consumer, audience and expiration on every request, including a request on an existing session.
The single consumer exists throughout the stub's lifetime. The deliberately forced first 401
happens even for a valid token, exactly once. It is not a token rotation or a validity result.
`token_valid` and the HTTP status record those facts separately.

The identity filenames, environment variable names, and every Conduit metadata prefix come
from `spec/names.yaml`, embedded at compile time. The token file is mode 0600. The directory
also receives the public JWK set, engine credential ID and a JSON environment description.
This loopback stub does not mint an X.509 chain or implement a runtime provider; it claims only
the token delivery part of PRV-022, not its whole `identity` or any HTTPS/OAuth deployment.
A helper invocation reads the experiment's token file afresh and prints the JSON header map
only to the client; `helper.jsonl` records its time without its value. The key is never written.

Successful calls carry outcome succeeded, an invocation ID, `isError: false` and outputs in
`structuredContent` and serialized text. Failed and unknown calls carry the outcome and
invocation ID, `isError: true`, a first text block stating the outcome, and no outputs.
Schema-invalid calls carry reason invalid and an error diagnostic citing OPS-033, with no
invocation or outcome. This validation precedes control dispatch too: invalid `control.next.v1`
arguments change no state and send no notification. Tool annotations include `readOnlyHint`
only for read tools, with value true; write and control definitions omit it (IFC-033).
Refused unknown-tool calls have
error data with reason refused and no outcome, invocation ID, tool result or `isError`.
This is the protocol-required exception to the work order's request to test refused `isError`.

## Client isolation and cache switches

Claude uses `--setting-sources project`, an explicit settings file, `--strict-mcp-config`,
one HTTP server from `--mcp-config`, `--tools ""`, `--allowedTools 'mcp__stub__*'`,
`--disable-slash-commands`, `--no-session-persistence`, and JSON events. The installed help
explicitly says that the empty `--tools` value disables built-in tools. The fresh lab has no
owner settings or trust records. `--allowedTools` permits the stub without relying on
workspace trust; the explicit lab settings additionally disable hooks and plugins.
`MCP_DISCOVERY_CACHE=0` or `1` selects the requested cache variant.

Codex uses `--ignore-user-config --strict-config --ignore-rules --disable apps`, a read-only
sandbox, `--ephemeral --skip-git-repo-check --json`, disabled web search and only one configured
MCP server. It retains its built-in tools under that sandbox, as the work order's specified
Codex isolation permits; the prompt restricts use to the stub. Any other tool call is evidence
of a deviation and must be classified from the events. `startup_readiness="catalog"` is the
cache-on variant and `"connection"` the cache-off variant. This setting is a readiness choice,
not an independently documented master cache-disable switch. With fresh labs and free ports,
neither client's on variant is guaranteed a previously warmed entry; no agent warm-up run is
inserted into the pre-registered matrix. The admin must retain that limitation in the results.

Claude receives a `headersHelper` shell command; Codex receives an
`mcp_servers.stub.http_headers_helper` shell command. Both invoke this binary's
`--headers-helper` mode with absolute, shell-quoted lab paths. Standard input is null for
version checks and agent commands. Model prompts and commands contain no token.

Local documentation consulted: installed CLI help; locally saved Claude Code MCP/environment
references; locally saved MCP July 2026 transport, tools, caching, discovery and schema docs;
and the downloaded official `rmcp` source. Installed Codex binary metadata confirms the helper
key and the `connection`/`catalog` enum, but no local full Codex reference established stronger
cache semantics. Remote documentation was not fetched: this work order limits network use to
crates. Useful source locations for admin verification are
[Claude MCP](https://code.claude.com/docs/en/mcp),
[Claude environment variables](https://code.claude.com/docs/en/env-vars),
[Codex configuration](https://developers.openai.com/codex/config-reference/), and
[MCP July 2026](https://modelcontextprotocol.io/specification/2026-07-28).
These links identify references, not a claim that the worker fetched their live contents.

## Evidence and interpretation

Every run gets its own directory:

| Artifact | What it establishes |
| --- | --- |
| `cell.json`, `prompt.txt`, `plan.json`, `mcp.json`, `settings.json` | Requested cell, repetition, prompt and exact generated command/configuration |
| `version.txt`, `version-stderr.txt` | Installed client version and version-check stderr |
| `stub.jsonl` | Admission validity, HTTP request bodies and the explicit protocol-header allowlist below, response headers/status, emitted wire chunks, notifications and whether a notification stream was open |
| `helper.jsonl` | Header-helper reads, including whether a client invokes it again after a 401; absent when no helper ran |
| `events.jsonl`, `stderr.txt` | Raw client event stream and stderr |
| `client-exposed.jsonl` | Selected client-exposed assistant/user or completed-item events, copied without interpretation |
| `final.txt` | Claude's final result text or Codex's output-last-message; empty when none was exposed |
| `authentication.json` | Client-reported authentication fields with event line numbers and JSON pointers; explicit unavailable status if absent |
| `status.json`, `visibility.txt` | Exit status, timeout or launch error; the boundary of visibility evidence |
| `identity/`, `tmp/`, `lab/`, `npm-cache/` | Throwaway per-run state; the identity directory contains only experiment credentials |

Request `headers` captures only `Mcp-Session-Id`, `Mcp-Protocol-Version`, `Mcp-Method`,
`Mcp-Name`, `Origin`, `Accept`, `Content-Type` and `Last-Event-ID`, using lowercase keys and
arrays of values. Missing headers are absent. Authorization and unlisted request headers are
excluded; the minted token is redacted from captured values. Response headers are recorded
separately in response events.

Wire chunks are JSON-escaped strings with request IDs and timestamps. Reassemble `wire` data
by ID before parsing SSE. Request/response IDs are local log correlation IDs, distinct from
JSON-RPC request IDs. SDK notification enqueue success is logged apart from actual bytes emitted
on a stream. Closing a stream is also logged. A change without an open stream is recorded;
subsequent lists still reflect the current state. No claim of delivery follows from enqueueing.

Neither client's public event stream is a guaranteed copy of every model input. The harness
keeps what the client exposes and never infers model visibility of `_meta` from a wire response.
Where exact tool-result/model-input visibility is absent, the classifier must mark it unavailable;
this harness cannot make that observation from unchanged headless clients alone. No model
behavior, visibility classification, performance measurement or experiment result is reported
by the worker. A future work order classifies the admin's runs.

## Checks

```sh
cargo test --release
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run --release -- --dry-run
```

The HTTP tests drive all five cases at all three revisions and both TTLs, checking IFC-012,
IFC-013, IFC-022, IFC-025, IFC-033, IFC-036, IFC-038, IFC-042, ACC-011, OPS-001 and OPS-005.
They also test admission, a forced first 401, the experiment token file/helper read, notification
subscriptions and private list hints. Runner tests check every matrix entry and repetition,
subset selection, isolation flags, shell quoting, and rejection of repository output paths,
including symlink aliases, the exact 68-run amended matrix, required explicit output,
API-key environment removal, authentication event extraction, invalid-control non-dispatch,
HTTP error statuses, unsupported-version negotiation and protocol header logging.
They launch only the harness helper and harmless local shells,
never an agent.
