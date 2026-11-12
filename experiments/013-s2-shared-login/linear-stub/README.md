# S2 Linear API stub

Test infrastructure for [S2](../README.md), excluded from D01's implementation lines.
This independent Rust crate depends on no repository crate. It is neither a Conduit
implementation nor an engine vector fixture. It makes no outbound connections.

**Frozen interface: S2 Linear stub v1**, crate version 0.1.0. S2-L1, S2-L2 and S2-H may
rely on the library lifecycle, HTTP routes, schema and semantics below. Incompatible changes
need a new interface version. Fidelity to the real API cannot be checked offline: the
coordinator's S2 runs against the owner's test workspace check it. No live-service check
was made to build this stub. The differences below are intentional fixture choices, not
claims that the real service behaves the same way.

## Start and stop

The toolchain is pinned to Rust 1.97.1. Use this crate as a path dev-dependency; its crate name
in Rust is `s2_linear_stub`. From a Tokio runtime, start it in process:

```rust
use s2_linear_stub::{IssueFixture, Stub, StubConfig};

# async fn example() -> std::io::Result<()> {
let config = StubConfig::new(vec![IssueFixture::new("SYN-1", "Synthetic issue")]);
let api_key = config.api_key.clone();
let control_token = config.control_token.clone();
let stub = Stub::start(config).await?;
let graphql_url = stub.graphql_url();
let control_url = stub.control_url();
// Use api_key.expose() or control_token.expose() only to set an HTTP header.
// Never print tokens or request headers.
stub.shutdown().await?;
# Ok(())
# }
```

`StubConfig::new` generates distinct synthetic tokens and binds `127.0.0.1:0`.
`config.bind` may select a fixed port or IPv6 loopback; non-loopback addresses are rejected.
`Stub::address`, `graphql_url` and `control_url` report the selected listener. `shutdown`
drains requests (an unreleased barrier is bounded by its deadline); dropping `Stub` aborts
the listener task. `SyntheticToken::generate` makes a fresh test token. Tokens have redacted
`Debug` output and no `Display` implementation. Do not log `expose()`.

The binary takes two positional paths:

```bash
cargo run --release -- fixture.json target/connection.json
```

The first file is a JSON array of `IssueFixture` values, as in [fixture.json](fixture.json).
Identifiers must be unique and match `SYN-[0-9]+`. All text must be synthetic; use no real
workspace, team or user names. Fixture fields are `identifier`, `title`, `description`
(text or null; omission also means null), `state` (a state name) and `updatedAt` (RFC 3339).
Unknown fields are rejected. `IssueFixture::new` supplies null description, state `Todo`
and the fixed updated time `2026-01-01T00:00:00Z`.

The binary creates the second file exclusively, with mode 0600 on Unix. It contains
`interface_version` (1), `graphql_url`, `control_url`, `api_key` and `control_token`.
It prints only the interface version and listener address; Ctrl-C drains the server.
The file remains after shutdown for the harness to remove. Keep it private and outside
tracked files; never display its contents. An existing connection file causes startup to
fail. No environment variable, real credential store or external service is read.

## GraphQL subset

POST JSON to `/graphql`, with `query` and optional `variables` and `operationName`.
Set the `Authorization` header to the raw synthetic API key, without a bearer prefix,
matching the personal API key header convention. Missing, wrong or rotated-out keys receive
HTTP **401** and `errors` with code `AUTHENTICATION_ERROR`. Authentication is checked at
HTTP receipt; rotating a key does not cancel a request already admitted.

The schema is executed by **async-graphql** (7.2.1 in Cargo.lock), served by Axum. GraphQL
selection sets, aliases, variables, named operations, fragments and schema introspection
work normally. Unsupported fields, arguments or input fields and malformed GraphQL produce
HTTP 200 with GraphQL `errors`; malformed JSON produces HTTP 400. The default Axum JSON
body limit is 2 MiB. GraphQL depth is limited to 16 and complexity to 1000.

These are the subset's application types (GraphQL's normal introspection types also exist):

```graphql
scalar DateTime
type Query {
  issue(id: String!): Issue!
  issues(first: Int! = 50, after: String): IssueConnection!
}
type Mutation {
  issueUpdate(id: String!, input: IssueUpdateInput!): IssuePayload!
}
type Issue {
  id: String!
  identifier: String!
  title: String!
  description: String
  state: WorkflowState!
  updatedAt: DateTime!
}
type WorkflowState { name: String! }
input IssueUpdateInput { title: String, description: String }
type IssuePayload { success: Boolean!, issue: Issue! }
type IssueConnection {
  nodes: [Issue!]!
  edges: [IssueEdge!]!
  pageInfo: PageInfo!
}
type IssueEdge { node: Issue!, cursor: String! }
type PageInfo {
  hasNextPage: Boolean!
  hasPreviousPage: Boolean!
  startCursor: String
  endCursor: String
}
```

`issue(id:)` and `issueUpdate(id:)` accept an exact, case-sensitive identifier or the returned
synthetic canonical `id`, `stub:SYN-123`. Missing and service-inaccessible issues both
produce the same GraphQL error, `ENTITY_NOT_FOUND`. No partial issue is returned.

`issues` selects only service-visible existing issues, sorts identifiers in ascending ASCII
order, applies `after` exclusively, then takes `first` (1 to 250; default 50).
The cursor is an identifier, not an encoded offset. A syntactically valid cursor need not
exist. `hasPreviousPage` means a currently visible issue sorts at or before `after`;
`hasNextPage` means one sorts after the returned page. Empty pages have null start/end
cursors. Edges and nodes carry the same issues. There is no total count.

`issueUpdate` changes supplied title and description only. Omitted fields stay unchanged;
null description clears it; null title leaves it unchanged; an empty title is allowed.
An empty edit is a successful no-op. Any supplied edit advances `updatedAt` to the later
of the current UTC time and the prior value plus 1 ms. No other issue or field changes.
Success returns `success: true` and the current issue. DateTime output is RFC 3339 UTC.

For example, callers may project S2's string state from `state.name`:

```graphql
query Read($id: String!) {
  issue(id: $id) { identifier title description state { name } updatedAt }
}
```

## Control endpoint v1

These routes share the same loopback listener and require the **separate raw control token**
in `Authorization`. The API token does not authorize control calls, nor does the control
token authorize GraphQL calls. Wrong control authentication is HTTP 401. No response echoes
a token. All POST bodies are JSON objects; unknown properties are rejected.

| Method and path | Body | HTTP 200 response |
| --- | --- | --- |
| GET `/control/v1/count` | none | `{"api_requests": 0}` |
| POST `/control/v1/service-authorization` | `{"key": "<current synthetic API key>", "issues": ["SYN-1"]}` | `{"ok": true}` |
| POST `/control/v1/rotate-key` | `{"key": "<new synthetic API key>"}` | `{"ok": true}` |
| POST `/control/v1/barrier/arm` | `{"match": {"field": "issue", "identifier": "SYN-1"}, "timeout_ms": 5000}` | `{"id": 1, "state": "armed"}` |
| POST `/control/v1/barrier/wait` | `{"id": 1, "timeout_ms": 2000}` | `{"id": 1, "state": "reached"}` (or `released`) |
| POST `/control/v1/barrier/release` | `{"id": 1, "timeout_ms": 2000}` | `{"id": 1, "state": "released"}` |

`service-authorization` sets the **service's own authorization for the current accepted key**.
It is not the Conduit filter's authorization data. The stub receives no Conduit metadata,
scope filter or deadline input and cannot qualify those restrictions itself. Initially all
fixture issues are visible. An empty set hides all; nonexistent synthetic identifiers are
allowed but yield no issue. Wrong keys or malformed identifiers receive HTTP 400. Permissions
are read when each resolver executes, after its barrier; a change while blocked is observed.

`rotate-key` replaces the one accepted API key immediately, preserving the service-visible
set. The old key becomes unauthorized for new requests. Use `SyntheticToken::generate` in
the harness; only the stub's synthetic token format is accepted (HTTP 400 otherwise).
The control token is fixed and cannot become the API key. Nothing restarts implicitly.

The count is cumulative HTTP POST requests routed to `/graphql`, incremented before
authentication, JSON extraction or GraphQL validation. It includes failures, oversized bodies,
and requests held at a barrier. One document with multiple root fields counts once.
Control requests, including failed control requests, and other routes/methods do not count.
There is no reset: start another stub for an independent count.

Barrier rules:

- `field` is `issue`, `issues`, `issueUpdate` or `any`. Optional/null `identifier` matches any
  identifier; otherwise it matches an exact `SYN-...` identifier, normalized from either
  accepted form of `id`. `issues` cannot have an identifier matcher. Invalid matchers are 400.
- Matching uses a validated root resolver, not a query string, alias or operation name.
  A wrong API key, schema validation failure or invalid pagination argument does not reach
  a barrier. The next matching request claims it before reading/changing service data.
  Only that request waits; later matching requests proceed. At most one barrier can be
  claimed per HTTP request, even if its document has several matching root fields.
- Arming sets an absolute deadline `timeout_ms` from arm time. The barrier expires even if
  no request reaches it. Arming over an armed/reached barrier returns 409; rearming after
  release/expiry replaces it and returns a monotonically increasing ID.
- Every timeout must be 1 through 60000 milliseconds (400 otherwise). Wait and release have
  their own bounded wait for arrival; a wait timeout is 408 and leaves the barrier armed.
  Release may be requested before arrival, waiting up to its timeout. Releasing an already
  released barrier succeeds. Unknown/replaced IDs are 404; expired barriers are 409.
- An unreleased request unblocks at the arm deadline with HTTP 200, GraphQL error code
  `STUB_BARRIER_TIMEOUT`, and no work by the held resolver. Timeout is fixture behavior,
  not a simulation of a real transport failure or an unknown external effect. A timeout
  does not roll back other root fields that already completed in the same document.

## Differences from the real API

These are the interface's own fixture choices and the fidelity limits to check in S2:

1. Loopback HTTP `/graphql`, raw generated test tokens and a private connection file replace
   the real deployment and login. One accepted API key, one issue visibility set and the
   private `/control/v1` routes are test machinery; no OAuth, teams, users or memberships exist.
2. The schema contains only the listed application types, fields and arguments. There are no
   issue relations, comments, filters, ordering arguments, reverse pagination, other mutations,
   subscriptions, rate limits, webhooks or service-side retries. Introspection describes this
   small schema. Local body/depth/complexity limits and JSON error bodies are fixture choices.
3. Canonical IDs are `stub:SYN-...`, not UUIDs. Identifiers are restricted to synthetic `SYN`
   data. State is only an object with a name; it is required here even though richer/nullable
   real-service data may need handling. There is no state ID or state editing.
4. Forward pagination uses identifier cursors and ASCII identifier order, a default page of
   50 and a 250 maximum, with the explicit page flags above. These ordering, limits and cursor
   semantics are deterministic fixture choices, not a guarantee about real-service pagination.
   S2 callers must project/limit/filter according to their own contracts.
5. The public `issue`, `issues` and `issueUpdate` shapes are modeled on Linear's GraphQL API;
   exact nullability, scalar choices and supported inputs are limited to the displayed schema.
   Missing/inaccessible error codes, messages and envelopes beyond HTTP 401 are fixture choices.
   GraphQL execution errors use HTTP 200. Live error envelopes require coordinator verification.
6. Mutation edits and no-op/null-title behavior are local rules. Timestamps advance using the
   host UTC clock with a minimum increment; no remote scheduling, history or side effects exist.
   The stub does not implement Conduit's idempotency or repeat semantics.
7. State is in memory per process. Service permission changes take effect at resolver execution;
   key rotation takes effect at admission and preserves the visible set. No multi-key access,
   revocation propagation delays or persistence are modeled. Barriers and API counts are fixture
   controls and have no corresponding real-service behavior.

## Checks

Run from this directory: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`
and `cargo test --release`. Tests use actual loopback HTTP with proxies disabled and bounded
client timeouts; they do not contact a real service or print generated tokens. The repository's
documentation checker runs from its root with `python3 tools/check_docs.py`.
