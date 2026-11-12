# S2 small Linear adapter

This package serves [scenario S2](../../experiments/013-s2-shared-login/README.md) and is not
for general use. It calls the service's GraphQL API directly for D01's reading task. It was
written independently of the aware implementation's source. Offline qualification uses only
[stub interface v1](../../experiments/013-s2-shared-login/linear-stub/README.md); compatibility
with a real workspace remains untested. No real service was contacted.

[conduit.yaml](conduit.yaml) pins `example.org/conduit/scenario-s2-linear-restrictions`,
version `0.1.0`, digest `sha256:6380ce0e6a7db56712f457fd3a56aa7a6802ce3e019c56a3433eda440feac43e`.
[contracts.yaml](contracts.yaml) exports `linear-adapter.v1`, family Resource, offering only
`issue-reading.v1`. Its two operations, `read-issue.v1` and `list-issues.v1`, map to the
`adapter` item, with `issue-scope.v1` in `supports`. The relative command
`bin/scenario-s2-linear-adapter` runs under the engine's own runner, with no plain flag.
There is no update tool, onward request, cache, shared library or generated GraphQL client.

## Configuration and local choices

| Place | Delivery and meaning |
| --- | --- |
| `baseUrl` (required) | Full GraphQL endpoint including path, captured once from `S2_LINEAR_ADAPTER_BASE_URL`; the call's configuration must match it. |
| `apiKey` (required) | Core `secret-reference.v1`; call metadata holds only the resolved `{kind: Resource, id}` reference. The value is captured once from `S2_LINEAR_ADAPTER_API_KEY`, sent bare in Authorization. |
| `testClock` (optional) | RFC 3339 instant in each call's configuration, bound to no variable. Tests/prepared runs only. Omission samples the real clock once at handler receipt. Fractions retain all digits for deadline comparisons across offsets; malformed values fail before service access. |

The engine's secret-reading path (SEC-D) is not built yet: tests inject the process variable
directly and do not establish engine delivery or S2 P3. The key stays fixed for the process's
life (SEC-014). Rotation requires explicit replacement with a newly supplied key. No sign-in,
secret store, OAuth flow, supervisor, gate or automatic recovery is provided.

The following are adapter choices where the contract leaves mechanism open. URLs must use HTTPS
or HTTP with a literal loopback IPv4/IPv6 address; HTTP hostnames, user information, query strings
and fragments are rejected before transport. Proxies and redirects are disabled. Connect timeout
is two seconds and total request timeout five seconds, longer than the receipt-time barrier hold.
Responses are bounded to 2 MiB; there are no retries. Service authorization (401/403), transport,
timeout, GraphQL lookup/error and invalid-response failures answer `failed` with fixed diagnostics.
Service error bodies/headers are never included. Successful output is validated against embedded
copies of the pinned operation schemas; credential-bearing output is suppressed.

There is one fixed query document per operation. Read projects `state.name` into the five-field
issue output. List pages through `issues` in batches of 100, intersects all scopes, deduplicates
by identifier, sorts in ASCII order, then applies the limit. It reads all pages to honor the
contract's sorting independent of service order. Opaque cursors may not repeat. Missing issues
are omitted. An explicit empty filters array permits an unfiltered list. Integral limits written
as `1.0` or `1e0` are integers. No state is kept between calls.

The server advertises Conduit's extension at `v0` via discovery. Missing or other revisions
receive -32021 with `data.requiredCapabilities`, without service access. Unknown tools (including
update) receive protocol error -32602. All required IFC-006 metadata is checked; missing/malformed
filters never mean unfiltered. Required IDs/revision are nonempty strings, operation matches the
tool, configuration has the declared shape, and optional Conduit/idempotency IDs are checked when
present. Malformed operation/filter inputs fail closed. Unknown requirements and undeclared
requirement inputs answer `unavailable` with the received filter.

F1-F4 are evaluated on every receipt, including repeated idempotency metadata. Deadlines compare
instants across offsets and are judged once before service access. Forbidden reads and expired
lists perform nothing and answer IFC-026's `refused`, received restriction, isError true, no outcome
and no issue output. Lists otherwise succeed, including empty scope intersections. Mentions in
issue text stay unchanged. Duplicate filters are applied independently. Outcomes and isError
agree. Every stdout line, including library-generated errors, passes through a raw/JSON-escaped
key redactor. The writer flushes directly through the framing sink; there is no output pump to
race input EOF. There is no logging subscriber; fatal stderr diagnostics are fixed.

## D01 and D05 qualification

`tests/mcp.rs` uses an rmcp client, complete request metadata and an in-process stub. Successful
answers validate against output schemas. Tests count service requests on denied/malformed calls,
exercise discovery and protocol errors, pages, content projection, integral limits, deadline
replays, strict clocks, changed filters, service visibility and key rotation. A real-clock read
waits past its deadline at the stub barrier, succeeds when released, and is refused on repeat
without another service request. All captured stdout and stderr are checked for synthetic API
and control credentials; -32022 redaction and immediate input EOF are repeated 100 times.

| D05 case | Local evidence and limit |
| --- | --- |
| Changed authorization data | Changed call filters are rejudged; the service's separate visibility change produces failed lookup. |
| Keyed repeat | Identical invocation/idempotency metadata is rejudged after expiry; engine OPS-024 dispatch remains outside these tests. |
| Output delivery | Five-field projection, unchanged mentions, multiple scopes, pages, ordering, missing issues and scope before limit. |
| Onward calls | Unexercised: this row makes no onward request. |
| Revocation | Deadline and service-key expiry are exercised; Conduit close belongs to the engine and remains unexercised. |

D01's supported operations are exactly read/list; its only requirement is issue-scope. Full
Conduit keys are mandatory, with no plain fallback. No interactive sign-in is needed by this
process. The unchanged plain-server row and its choice belong to S2-PL and the owner's runs.
These tests qualify this adapter locally; no author task, model/client concept observation,
panel action, elapsed author time, outage interval or real-Agent run was measured.

## D15 and provider-work counts

Following [S10's four fields](../../experiments/021-s10-swapped-providers/README.md#measures):

| Field | Record |
| --- | --- |
| (a) Own work — observed | `src/lib.rs` (379 counted lines) implements read/list, metadata checks, scope/deadline enforcement, projection and outcomes. `src/redactor.rs` (78) protects every protocol line; `src/main.rs` (23) captures startup configuration and serves stdio; `src/bin/package.rs` (51) builds the archive. Declarations and qualification are counted apart below. |
| (b) Delegation — observed locally | rmcp handles MCP framing and lifecycle; reqwest/rustls handle HTTP/TLS; service GraphQL handles lookup, pagination and authorization. Only loopback HTTP is exercised; real HTTPS behavior is untested. |
| (c) Persistent state — absent | None. Endpoint, key and HTTP connection pool live only in the process. Service owns issues/visibility. Stop discards local state; replacement supplies configuration/key again. |
| (d) Setup and recovery — partly observed | Build/install host package, configure endpoint and secret Resource, arrange runner delivery and Conduits. Tests substitute variable injection. Rotation starts a new process. Engine delivery, gates, sign-in, isolation and manual outage recovery are untested. |

Counts follow S2 M0's provider work apart from author work: physical nonblank lines excluding
standalone comments, with declaration separators counted. Generated copies and dependency lock
are disclosed separately. Tests, documentation and lockfile do not enter implementation counts.
No aware implementation code was read or copied; stub and contracts work are counted elsewhere.

<!-- COUNTS_START -->
| Category | Files | Nonblank, noncomment lines |
| --- | --- | --- |
| Rust implementation and packaging | 4 | 531 |
| Manifest, Type and build configuration | 5 | 66 |
| Generated embedded operation contracts | 1 | 132 |
| Qualification tests | 2 | 1011 |
| Generated dependency lock | 1 | 2685 |
| README | 1 | 122 |
| Total | 14 | 4547 |
<!-- COUNTS_END -->

Configuration count: 2 required places, 1 optional place, 2 environment bindings, 1 Type,
1 item, 1 external pin with 3 fields (address/version/digest), 1 capability, 2 operation mappings,
1 supported requirement. Provider concepts (17 distinct specification terms): Resource, Type,
capability, operation, requirement, filter, implementation item, manifest, package, configuration,
secret reference, run, invocation, caller, revision, outcome, reason. Technologies counted apart
(6): Rust/Cargo, MCP stdio, GraphQL, HTTP/TLS, JSON Schema, RFC 3339 clocks. Client concepts: none
observed. Block-style and strict-command side readings belong to the later author run; these
are counts of the preparation artifacts, with no formatting conversion credited.

## Build and archive

From this directory under Rust 1.97.1, run one Cargo command at a time:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

The package binary consumes the adjacent release server binary already built by the test command;
it never runs Cargo. An initial build without tests can use `cargo build --locked --release --bins`.
It writes `dist/scenario-s2-linear-adapter.tar.gz` and `.tar.gz.sha256` (hex plus newline), printing
its `sha256:` digest. Fixed USTAR metadata, entry order/modes and gzip header produce identical
bytes from identical inputs on the host. This makes no cross-platform reproducible-build claim.
Archive entries: executable `bin/scenario-s2-linear-adapter` (0755), `conduit.yaml`, `contracts.yaml`,
`README.md` (0644). Source, tests, lockfile and stub are excluded; operation schemas are embedded
in the executable. Tests compare bytes twice, validate package YAML against `spec/schemas/`,
and check embedded operation copies against the pinned contracts; absence of those sources
prints an explicit skip. `target/` and `dist/` are ignored.

A63-DR and S3-PATH are listed, not built: they need a Linux build and a PATH-name or absolute item
in an operator-supplied image or machine. Such an item pins nothing; the relative package command
is for the engine's own runner. No image, machine provider or isolation was built or qualified.
