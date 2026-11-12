# S2 Conduit-aware Linear implementation

This package serves [scenario S2](../../experiments/013-s2-shared-login/README.md) and is not
for general use. Its offline qualification uses only the frozen v1
[Linear stub](../../experiments/013-s2-shared-login/linear-stub/README.md). No real service was
contacted. Stub compatibility does not establish compatibility with a real workspace.

[conduit.yaml](conduit.yaml) pins the
[restriction contracts](../scenario-s2-linear-restrictions/README.md) at address
`example.org/conduit/scenario-s2-linear-restrictions`, version `0.1.0`, digest
`sha256:6380ce0e6a7db56712f457fd3a56aa7a6802ce3e019c56a3433eda440feac43e`.
[contracts.yaml](contracts.yaml) exports `linear-aware.v1`, family Resource. It offers
`issue-reading.v1` and `issue-writing.v1`, mapping all three operations to the `linear` item.
The item states `issue-scope.v1` in `supports`; it is Conduit-aware, with no `plain` flag.

## Configuration and operation

| Place | Required | Meaning and delivery |
| --- | --- | --- |
| `baseUrl` | yes | Complete GraphQL endpoint URL, including its path. Bound to `S2_LINEAR_BASE_URL` at process start. |
| `apiKey` | yes | Core `secret-reference.v1` place. The runner delivers its value as `S2_LINEAR_API_KEY` at process start. The configuration in call metadata holds the resolved Resource reference, never the value. |
| `testClock` | no | RFC 3339 receipt instant, read from that call's configuration metadata. For tests and prepared runs only. Omission uses the real UTC clock captured at receipt. Malformed values fail before service access. |

The engine's secret-reading path (SEC-D) is not built yet. These tests set the variable directly;
they do not demonstrate engine delivery or S2 P3 end to end. No secret store, sign-in flow,
OAuth flow, supervisor or gate is provided. The key and endpoint are captured once at startup,
and the call's `baseUrl` must match that startup value. Rotating the service key requires an
explicit new run with the new value; an existing run retains its old value (SEC-014).
Both environment-bound places are required (MAN-017); the optional clock is not bound.

The MCP server uses stdio and lists Conduit's implementation-interface extension at revision
`v0` on discovery. Missing or different extension declarations receive -32021 naming the required
capability, before any service request. Unknown tools receive protocol error -32602 after that
extension check, with no service request. All IFC-006 required keys are checked, including a
mandatory array `filters`; only an explicit empty array means unfiltered. IDs and revision must
be nonempty strings; operation must match the tool; configuration must have the declared shape.
Malformed metadata, filter shapes, required filter inputs and operation arguments fail closed
with a fixed diagnostic. Unknown requirements or undeclared requirement inputs answer
`unavailable` with the received filter. Duplicate items are evaluated independently.

F1-F6 govern scope, projection and receipt time. Read/update scope is checked before lookup;
list intersects every scope before its limit, including integer limits written as `1.0` or `1e0`.
Each list traverses the service's forward pages
of 100, projects the five contract fields, deduplicates by identifier and sorts in ASCII order.
Missing issues are omitted from lists; missing/inaccessible reads or updates fail. Mentions
inside title/description remain unchanged. Null descriptions clear on update; omitted edits
remain unchanged. The test clock is sampled once per call; the real clock is also sampled once.
No timer cancels an admitted call after its deadline. Calls carry no authorization cache.

The endpoint must be HTTPS, or HTTP with a literal loopback IPv4/IPv6 address. HTTP hostname
resolution is deliberately excluded. URL user information, queries and fragments are rejected.
Proxies and redirects are disabled; the raw key is sent only in the Authorization header.
HTTP requests have a two-second connect and response timeout and a 2 MiB response-body bound.
No service retry is made. Unreachable, timed-out or interrupted reads fail; updates are unknown.
HTTP 401/403 fail all operations. Missing/inaccessible GraphQL entities fail; other GraphQL,
HTTP or malformed-response failures are unknown for updates and failed for reads. Service error
bodies, headers and values are never rendered in diagnostics. Output is schema-validated before
success; a response containing the held credential is suppressed. The server has no logging
subscriber and emits only protocol messages on stdout; fatal startup/transport stderr is fixed.

## D01, D05 and D15 evidence

This is D01's aware implementation row: `read-issue.v1`, `list-issues.v1`, `update-issue.v1`;
one supported filter, `issue-scope.v1`; full request metadata; no plain fallback; no onward
request; no interactive sign-in. `tests/mcp.rs` drives the actual binary using an rmcp client
and the in-process stub. Successful outputs validate against the pinned operation schemas;
refusals/unavailable carry the exact received filter, isError true and no outcome or output.
The tests count stub requests to prove zero service access on denied or malformed calls.

D05 evidence is local qualification, apart from the coordinator's engine and real-Agent run:

| Case | Evidence and limit |
| --- | --- |
| Changed authorization data | Success followed by changed call filters refuses without service access. A separate change to the service's own visibility produces a failed lookup, not a filter refusal. |
| Repeat | A dispatched read with the same invocation/idempotency metadata is rejudged after expiry, with no cached output. Engine OPS-024 redispatch and OPS-034 update-repeat conflict are not tested here. |
| Output delivery | Projection, unchanged text mentions, null/omitted edits, pagination, scope before limit, intersecting and duplicate filters are tested. |
| Onward calls | Unexercised: this implementation makes no onward request through Conduit. |
| Revocation | Deadline expiry and service key rotation are tested. Closing a Conduit belongs to the engine and remains unexercised here. |

The barrier test holds an admitted real-clock call until after its deadline, releases it to
success, then repeats the same call and observes refusal without another service request.
A separate deterministic-clock test compares offsets and refuses read/list/update at or after
the deadline. Rotation starts a new process with the new synthetic key; old calls fail 401.
All observed MCP answers, capability errors, discovery/tool responses and process stderr are
checked for both synthetic API credentials and the control marker. The stub is not counted as
provider implementation work. No portable vector or engine code is added.

D15's four fields, following [S10's measures](../../experiments/021-s10-swapped-providers/README.md#measures):

| Field | Record |
| --- | --- |
| (a) Own operations and custom code — observed | `src/lib.rs`: 419 counted lines for three operations, IFC metadata validation, scope/deadline enforcement, projection and outcomes. `src/main.rs`: 22 for environment capture and stdio lifecycle. `src/bin/package.rs`: 57 for deterministic packaging. Counts below separate declarations and qualification. |
| (b) Delegated work — observed locally | rmcp handles MCP framing/discovery transport; reqwest and rustls handle HTTP/TLS; the service performs reads, listing, authorization and edits. Only loopback HTTP delegation is exercised; real HTTPS service behavior is untested. |
| (c) Persistent state — absent | None. Endpoint, credential and HTTP client live in the process; service owns issue state. Stop discards local state. Restart requires operator-provided configuration and the chosen credential again. |
| (d) Operator setup and recovery — partly observed | Build host binary, install package, configure endpoint/secret Resource, arrange its Conduit and runner delivery. Tests substitute environment injection. Rotate by explicit replacement. Engine secret reads, gates, real sign-in, isolation and manual outage recovery remain untested. |

D01/M0 provider-work counts are listed apart from author work. Count nonblank lines excluding
standalone comments; document separators count. The counts cover this package's files once,
exclude ignored build/archive output, and separate implementation/declarations, tests, README
and lockfile. Generated schema copies and the dependency lock are disclosed, not hidden.
The stub and contracts-only package are separate work. No author task, client-model concept,
person intervention, recovery time or real-Agent request measurement was performed here.

<!-- PACKAGE_COUNTS_START -->
| Category | Files | Nonblank, noncomment lines |
| --- | --- | --- |
| Rust implementation and packaging | 3 | 498 |
| Manifest, Type and build configuration | 5 | 68 |
| Generated embedded contract copies | 1 | 243 |
| Qualification tests | 3 | 1021 |
| README | 1 | 126 |
| Generated Cargo.lock | 1 | 2702 |
| Total | 14 | 4658 |
<!-- PACKAGE_COUNTS_END -->

Configuration: 2 required places, 1 optional place, 2 environment bindings, 1 Type, 1 item,
1 external package pin with 3 pin fields (address/version/digest), 2 offered capabilities,
3 operation mappings and 1 supported requirement. Provider concepts (17 distinct specification
terms): Resource, Type, capability, operation, requirement, filter, implementation item,
manifest, package, configuration, secret reference, run, invocation, caller, revision,
outcome and reason. Implementation technologies (6, counted apart): Rust/Cargo, MCP stdio,
GraphQL, HTTP/TLS, JSON Schema and RFC 3339 clocks. Client-specific concepts: none.

## Build and package

Use the pinned Rust 1.97.1 toolchain, from this directory, one Cargo command at a time:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo build --locked --release --bins
cargo run --release --bin package
```

The package tool consumes the adjacent built release server binary; it never invokes Cargo.
`cargo test --release` also builds that binary. It writes `dist/scenario-s2-linear.tar.gz` and
its `.tar.gz.sha256` file. Fixed USTAR metadata, gzip header, entry order and modes make the
same host binary and documents produce identical archive bytes. Entries are the executable
`bin/scenario-s2-linear` (0755), `conduit.yaml`, `contracts.yaml`, `README.md` (0644). Source,
Cargo files, tests, stub and schema-copy source are absent from the archive; schemas are embedded
in the executable. This is reproducibility of packaging, not cross-platform reproducible builds.
The digest file is lowercase hex plus newline, and the tool prints a `sha256:` digest.
Tests compare archive bytes twice and validate both YAML documents against `spec/schemas/`,
explicitly reporting a skip when `spec/` is absent. Bundled operation schemas are checked against
the pinned contracts source when available, with a reported skip when it is absent.

A63-DR and S3-PATH are listed, not built: they would require a Linux build and a PATH-name or
absolute command item in an operator-supplied image or machine. A relative package-contained
command is for the engine's own runner only (PRV-007); a PATH-name/absolute command pins nothing.
No image, remote runtime, machine placement or isolation is qualified by this host build.
