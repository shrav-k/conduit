# Round 5, reader 6, the protocol

Read at commit d0c425f on the branch `docs/pre-codex`, on 2026-11-07: everything under `spec/`
that speaks of the Model Context Protocol, which is the whole of
`spec/interface/implementation.yaml` (IFC-001 to IFC-044), `spec/implementation-defined.yaml`,
the promises of `providers.yaml`, `secrets.yaml`, `manifest.yaml`, `contracts.yaml`,
`packages.yaml` and `schemas.yaml` that touch the wire, OPS-005, OPS-007, OPS-027, ACC-034,
ACC-036 and IMP-007, `spec/core/` (README and `operations.yaml`), `spec/schemas/manifest.yaml`
and `contracts.yaml`, `schema-profile.md`, `names.yaml` and the rows of `definitions.md` on the
interface. Each was checked against the published protocol, fetched the same day.

**The latest published revision is 2026-07-28.** The versioning page says "The **current**
protocol version is **2026-07-28**", the repository's release `2026-07-28` is marked Latest and
is not a pre-release, and the draft changelog holds one change since then (restoring `_meta` on
five client-side types), which touches nothing Conduit uses. The released schema was read at
the tag `2026-07-28`; the copy on `main` differs from it only in doc comments. The tasks
extension was read at `specification/2026-07-28/tasks.md` on `main` (last changed 2026-10-09,
commit 899c1bb), the client credentials extension at its stable `2026-07-28` text, and
server.json at schema 2025-12-11, which is still the latest released (the registry's draft
changes only the URL pattern).

**Counts.** 21 findings: 1 unsafe, 1 contradiction, 6 undetermined, 7 wrong, 2 misleading,
4 minor.

**Overall.** The core of the wire is right: the `_meta` keys and the extension's identifier
follow the naming rules, the error codes are used with their meanings, the tool names, results,
caching scope, list notifications and the cancellation of each transport agree with the
revision, and the earlier checks' corrections hold. The defects are where Conduit meets parts
of the protocol that it touches lightly: the tasks extension beyond `tasks/get` and
`tasks/cancel` (the one unsafe finding), the protocol's authorization (who issues a token, what
the client credentials extension allows, RFC 9728's exact-URL check), the edges of server.json,
and the paths that an existing plain server actually takes, which are a pre-2026-07-28
revision, unstructured `content`, input requests, `x-mcp-header` and an HTTP package on the
engine's own runner. Several of those block standard servers outright.

**The two points of `docs/AUDIT.md` ("To look at in particular").** Both hold, and each is a
finding: 5-6-16 (a refusal may never reach an Agent's model, and not only on the path of the
protocol error) and 5-6-18 (SEP-2145 is still open and stalled; if a later revision adopts it,
IFC-012 as written contradicts IFC-001 there).

Conventions below: S stands for `https://modelcontextprotocol.io/specification/2026-07-28`;
every quotation from S is of revision 2026-07-28. A promise named without a file is in
`spec/interface/implementation.yaml`.

---

## 5-6-01

- **Severity:** unsafe
- **Promises:** IFC-016, ACC-034, ACC-036, IFC-010
- **Quotes:** IFC-016: "A `tasks/get` or a `tasks/cancel` for the invocation's task is decided
  anew, as a request through the Conduit through which the invocation was dispatched."
  ACC-034: "a `tasks/get` for the invocation under IFC-016, is decided anew under ACC-036: made
  through the closed Conduit, it reports refused". `definitions.md`, request: "The asking for one
  operation on one target."
- **Protocol:** tasks extension, revision 2026-07-28
  (<https://github.com/modelcontextprotocol/ext-tasks/blob/main/specification/2026-07-28/tasks.md>):
  "The client provides responses via the `inputResponses` field in one or more subsequent
  `tasks/update` requests." "Servers **MUST** perform authentication and authorization checks on
  each task-related request to ensure that the client has permission to access a task." "A
  server **MAY** use task IDs as bearer tokens for a server's stored state." Task status
  notifications: "clients send a `subscriptions/listen` request ... including a list of task
  IDs".
- **Situation:** The extension has three client methods and a subscription; IFC-016 decides two
  of them. A `tasks/update` is not "the asking for one operation", so no promise decides it, and
  nothing stops an engine from relaying a task's `inputRequests` to the consumer. Agent A calls
  `board.issues.delete.v1`, declaring the extension; the implementation answers with a task and
  later puts it in `input_required` with an elicitation, "type the issue number to confirm". The
  owner closes `board`. A's `tasks/get` now reports refused, but A's `tasks/update` with the
  confirmation is forwarded, and the deletion, which without that answer would not have
  happened, happens after the right was withdrawn. An engine that takes the protocol's leave to
  treat a task ID as a bearer token forwards the same `tasks/update` from any instance that has
  learned the ID, such as one to which A's model passed it in an argument. Each such engine keeps
  every promise.
- **Least change:** In IFC-016 write "A `tasks/get`, `tasks/update` or `tasks/cancel` for the
  invocation's task, and a `subscriptions/listen` that names the task in `taskIds`, is decided
  anew, as a request of the instance that sent the request that began the invocation, through
  the Conduit through which the invocation was dispatched." The other way is one sentence: "The
  engine relays no input request of an implementation to a consumer."

## 5-6-02

- **Severity:** contradiction
- **Promises:** MAN-010, SEC-024, IFC-041; schema `server-remote.v1`
- **Quotes:** `server-remote.v1`: "the headers it takes and the variables of its URL. `url` is
  also a URI as RFC 3986 gives one", with `variables` in its properties and `format: uri` on
  `url`. MAN-010: "a `remote` whose `url` is not a URI ... [is] rejected". SEC-024: "The URL is
  compared as the Remote writes it, so a host written as a variable of the Remote is none of
  these."
- **Protocol:** server.json schema 2025-12-11
  (<https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json>),
  `StreamableHttpTransport.url`: "URL template ... In Remote context, they reference variables
  from the transport's 'variables' object. After variable substitution, this should produce a
  valid URI." Its changelog for 2025-12-11: "Remote servers can now define URL template
  variables using `{curly_braces}` notation"
  (<https://github.com/modelcontextprotocol/registry/blob/main/docs/reference/server-json/CHANGELOG.md>).
- **Situation:** A Remote copied from the registry, `url: https://{tenant_id}.api.example.com/mcp`
  with `variables: {tenant_id: ...}`. RFC 3986 admits no `{` or `}`, and SCH-001 asserts the
  format, so `server-remote.v1` and MAN-010 reject the item; yet the same schema admits
  `variables`, and SEC-024 has a rule for exactly such a URL. Were it accepted, nothing would
  supply the variable: no key of an implementation item binds one, and IFC-041 connects "at the
  URL of its Remote". So the feature that schema 2025-12-11 added is described as accepted and is
  always rejected, and every multi-tenant Remote in the registry is blocked.
- **Least change:** Say it: in MAN-010 add "a `remote` whose `url` holds a variable" to the
  rejected items, drop `variables` from `server-remote.v1`, and drop the SEC-024 sentence.
  Supporting the variables instead needs a binding key, a check of the URL after substitution,
  and SEC-024 made at setup rather than at registration.

## 5-6-03

- **Severity:** undetermined
- **Promises:** IFC-017, PRV-003, PRV-007, PRV-012, MAN-010, PRV-025
- **Quotes:** IFC-017: "only the engine's own runner returns none, and the engine reaches the
  process it starts over stdio." PRV-007: "The engine then reaches the server at the `url` that
  the run returns or, on its own runner, over the process's stdio." MAN-010: "`package` is one
  Package ... whose `transport` is of the `type` `stdio` or `streamable-http`". PRV-003: its own
  runner "keeps every promise that this file makes of a provider of that kind". `manifest.yaml`
  header: "a Package's transport is `stdio` or `streamable-http`, since IFC-017 reaches a run
  over no other."
- **Protocol:** S/basic/transports/stdio: "In the **stdio** transport, the client launches the
  MCP server as a subprocess." S/basic/transports/streamable-http: "In the **Streamable HTTP**
  transport, the server operates as an independent process that can handle multiple client
  connections"; "When running locally, servers **SHOULD** bind only to localhost".
- **Situation:** An instance names no `runtime` and selects a registry Package whose transport
  is `streamable-http` (`url: http://localhost:{port}/mcp`). MAN-010 accepts it; the own runner
  must reach the process over stdio, which such a server does not speak; PRV-012 refuses a
  transport that a provider's contract does not list, but no text lists the own runner's
  transports. One engine ends the run failed; another starts the process and gets no answer on
  stdio, so the part ends unknown with a server listening on loopback; a third connects over
  loopback HTTP, which breaks IFC-017 and leaves a server with no gate that every local process
  can reach (PRV-025 asks no gate of the own runner because stdio "reaches no other process").
- **Least change:** In PRV-003 add: "Its own runner runs a `command` item and a `package` item
  whose `transport` is of the `type` `stdio`, and ends the run of any other `package` failed, as
  PRV-012 has a runtime provider do."

## 5-6-04

- **Severity:** undetermined
- **Promises:** IFC-016, ACC-034, IFC-025, OPS-027
- **Quotes:** ACC-034: "So where the tasks extension answered the call with a task, a result that
  only a later `tasks/get` would read is not delivered once the Conduit has closed". IFC-025 is
  about a tool result: its `_meta`, `isError` and `structuredContent`.
- **Protocol:** tasks extension (URL as 5-6-01): "Invalid or nonexistent `taskId`: `-32602`
  (Invalid params) - Servers **MUST** return this error for `tasks/get`." "`tasks/get` ...
  `resultType` field **MUST** be set to `"complete"`", and its result is a `Task` with a
  `status`, not a tool result. "Each notification carries a complete `DetailedTask` for the
  current status, identical to what `tasks/get` would have returned at that moment." "A server
  **MUST NOT** return `CreateTaskResult` to a client that did not include the extension
  capability on its request". "If a server is unable to service a request to a client that does
  not declare this extension capability without returning `CreateTaskResult`, the server
  **MUST** return an error with the code `-32021`".
- **Situation:** A `tasks/get` through a closed Conduit "reports refused", but nothing says in
  what form. A tool result's shape does not fit a `Task`, and no task status means refused. One
  engine answers -32602 with the reason in `data`, as IFC-012 does; another a `complete` result
  with the reason in `_meta` and some status. A consumer that opened `subscriptions/listen` with
  the task's ID before the close receives the final result in `notifications/tasks` from an
  engine that takes the stream as decided when it opened, and not from one that reads ACC-034
  literally. Nor is it said whether the engine declares the extension to an implementation when
  the consumer did not, which decides whether a plain server that only works with tasks answers
  -32021 (unknown under IFC-010, on every call) or a task the engine must then poll for itself.
- **Least change:** In IFC-016 add: "A request of the extension that does not proceed is
  answered with the JSON-RPC error -32602, whose `data` holds the reason, as IFC-012 answers a
  call to a tool not presented. The engine sends no `notifications/tasks` for a task whose
  Conduit is not open, and declares the extension to an implementation only on a call for a
  request that declared it."

## 5-6-05

- **Severity:** undetermined
- **Promises:** IFC-001, IFC-025, IFC-010, OPS-007, CTR-012, DL12
- **Quotes:** IFC-001: "the result's `structuredContent` is the operation's outputs, keyed by
  name". IFC-025: "This specification gives it no meaning: an operation's outputs are in
  `structuredContent`." DL12: "what the engine puts in the `content` member of a result it
  returns to a consumer". OPS-007: "a plain server that returns a complete result that is not an
  error, and no `structuredContent`, for an operation with such an output has the outcome
  unknown."
- **Protocol:** S/server/tools: "Tool results may contain **structured** or **unstructured**
  content. **Unstructured** content is returned in the `content` field of a result". The page's
  own `tools/call` example returns only `content`, and `outputSchema` is optional.
- **Situation:** The protocol's ordinary tool, and most existing servers, return their result as
  text in `content` and declare no `outputSchema`. A Conduit contract cannot describe that
  (CTR-012 makes every output a member of `structuredContent`). If the contract declares a
  required output, every call is unknown (OPS-007). If it declares none, the call succeeds with
  `structuredContent: {}`, and whether the consumer sees the server's text is DL12: one engine
  passes the implementation's `content` on, another sends `[]`. The same request against the
  same server gives an Agent the issue's text on one engine and nothing on another. This is the
  main path by which a plain server is useful, and the promises carry none of it.
- **Least change:** In IFC-025 add: "Where the implementation is plain, the result's `content`
  is the implementation's `content`, unchanged." (The alternative, never passing it on, is also
  determinate but blocks every server that answers in text; this is for the owner.)

## 5-6-06

- **Severity:** undetermined
- **Promises:** IFC-007, IFC-037, IFC-010, OPS-007
- **Quotes:** IFC-007: "The call's `_meta` holds only keys that the protocol itself defines,
  such as `io.modelcontextprotocol/protocolVersion`." IFC-037: "It declares Conduit's extension
  ... on each call, in `io.modelcontextprotocol/clientCapabilities`." IFC-010: "A result whose
  `resultType` is `input_required` ... any other error, and no result at all, is unknown."
  OPS-007: "No result arrives, other than the protocol's error -32021 with which IFC-008 has an
  implementation say that it performed nothing".
- **Protocol:** S/basic: `io.modelcontextprotocol/clientCapabilities` is required on every
  request; "A server **MUST NOT** rely on capabilities the client has not declared. If processing
  a request requires a capability the client did not include ..., the server **MUST** return a
  `MissingRequiredClientCapabilityError` (`-32021`)". S/server/tools: "Servers **MAY** respond
  to `tools/call` with an `InputRequiredResult`".
- **Situation:** Nothing says what else the engine declares in `clientCapabilities` (elicitation,
  the deprecated sampling and roots, tasks), whether it copies the consumer's declaration, or
  whether it relays input requests. A plain server that confirms destructive calls by
  elicitation: an engine that declares `elicitation` gets `input_required` and records unknown
  on every call; one that declares nothing gets -32021, also unknown, or, from a server that
  then proceeds without asking, a performed deletion that succeeded. The same applies to a
  server that needs the tasks extension (5-6-04). And OPS-007 treats one -32021 as failed without
  saying how the engine tells it from a -32021 that names another capability (by
  `data.requiredCapabilities`, presumably).
- **Least change:** In IFC-007 and IFC-037 add: "In `io.modelcontextprotocol/clientCapabilities`
  the engine declares no capability but Conduit's extension, where this promise has it, and the
  tasks extension where IFC-016 lets it, and it fulfils no input request." In OPS-007 write "the
  error -32021 whose `data.requiredCapabilities` names Conduit's extension".

## 5-6-07

- **Severity:** undetermined
- **Promises:** IFC-001, IFC-003, IFC-010, IMP-007
- **Quotes:** IFC-001: "The engine and the implementation each keep every requirement of the
  protocol at the revision in use". IFC-003: a request rejected for its revision "is sent again
  at a revision both support, and is one invocation, not two." IMP-007 (`must not`): "It is
  dispatched again without a repeat of the request from its caller."
- **Protocol:** S/basic/transports/streamable-http: "While the use of `x-mcp-header` is optional
  for servers, clients **MUST** support this feature. When a server's tool definition includes
  `x-mcp-header` annotations, conforming clients **MUST** mirror the designated parameter values
  into HTTP headers." "Client omits header but value is in body: Non-conforming client: Server
  MUST reject the request". "If the server rejects a request with a `HeaderMismatch` error
  because required `Mcp-Param-*` headers are missing or do not match the body, the client
  **SHOULD** call `tools/list` to check for changes to the tool's `inputSchema`, then retry the
  original request with the appropriate headers."
- **Situation:** The engine builds a call from the contract (IFC-001), but the headers it must
  add come from the server's own tool definition, which no promise has it read. A plain remote
  server whose `region` input carries `x-mcp-header: Region`: an engine that never calls
  `tools/list` omits `Mcp-Param-Region`, the server rejects with 400 and -32020, and IFC-010
  records unknown, every time. After the server changes its schema, the protocol says re-list and
  resend; IMP-007 forbids a second dispatch and IFC-003 excepts only -32022. One engine resends,
  by analogy with IFC-003, and succeeds; another records unknown for a call that the protocol
  had the server reject unread.
- **Least change:** In IFC-001 add: "Before its first call over Streamable HTTP to an
  implementation, the engine reads its `tools/list`." In IFC-003 extend the observable to "A
  request that the implementation rejects ... with the protocol's error for an unsupported
  version, or with -32020 after which the engine reads the tool list again, is sent again, and is
  one invocation, not two."

## 5-6-08

- **Severity:** undetermined
- **Promises:** IFC-033, IFC-012
- **Quotes:** IFC-033: "Its `inputSchema` and `outputSchema` are the contract's own ... Its
  `annotations` hold `readOnlyHint` true only where the contract states no `effect`, is not a
  control and does not start model work". `contract-metadata.v1` has `description`.
- **Protocol:** S/server/tools: "A tool definition includes: `name` ...; `title`: Optional
  human-readable name ...; `description`: Human-readable description of functionality".
- **Situation:** A standard client gives its model each tool's `description` to choose among
  tools. IFC-033 says nothing of `description` or `title`, so one engine copies the contract's
  `metadata.description` and another sends none, and an Agent's model sees a bare
  `checkout.directory.read.v1`. "`readOnlyHint` true only where" is a necessary condition, not
  a sufficient one, so one engine marks every operation without effect read-only and another
  marks none. Both are visible in the tool list.
- **Least change:** In IFC-033 write "Its `description` is the contract's `metadata.description`
  where it states one, and its `annotations` hold `readOnlyHint` true exactly where ...".

## 5-6-09

- **Severity:** wrong
- **Promises:** IFC-004, IFC-003, IFC-007, IFC-010, PKG-020, DL12
- **Quotes:** IFC-004: "It provides what this interface relies on: ... the negotiation of
  extensions, by which a server lists the extensions it supports in its result of
  `server/discover`, a client declares an extension on each request in
  `io.modelcontextprotocol/clientCapabilities` with `revision` in the extension's settings
  object, and a server answers with the error -32021 ...". Observable: "A revision that lacks any
  of these is not used, whatever both sides support." `definitions.md`, Conduit's extension: "A
  plain implementation is declared none."
- **Protocol:** S/basic/versioning: "**Legacy**: protocol versions that establish a session with
  an `initialize` handshake (`2025-11-25` and earlier)"; extensions overview
  (<https://modelcontextprotocol.io/extensions/overview>): "Extensions became an official part
  of the protocol in version `2026-07-28`", and "Each extension specifies the schema of its
  settings object" (S/basic/versioning); the `revision` string is the overview's advice to an
  extension that breaks, not something a revision provides.
- **Situation:** IFC-004 says the interface relies on extension negotiation at every revision
  used with any implementation (IFC-003's `when` covers plain ones). A call to a plain
  implementation declares no extension (IFC-007), so the claim is false of it, and its effect is
  to exclude 2025-11-25 and every earlier revision for plain servers, which need none of it. A
  registry server built on an SDK that speaks only 2025-11-25, three months after the new
  revision, cannot be used at all, whatever DL12 lists; IFC-010's "or absent" `resultType`,
  which only such a server omits, then never applies.
- **Least change:** Limit the extension clause of IFC-004 to "a revision used for a call that
  carries Conduit's keys", and say that for a plain implementation a revision is used where it
  provides `tools/call` with `name` and `arguments`, `isError`, and `structuredContent` where the
  operation declares outputs. Whether the engine speaks a legacy revision stays DL12.

## 5-6-10

- **Severity:** wrong
- **Promises:** IFC-041, DL13
- **Quotes:** IFC-041: "the engine authenticates to that one with its own credential, as a JWT
  assertion under RFC 7523 or by mutual TLS under RFC 8705, so it holds no client secret outside
  every Conduit", in tokens "issued to the engine, under the protocol's extension for client
  credentials".
- **Protocol:** OAuth Client Credentials extension, stable, revision 2026-07-28
  (<https://github.com/modelcontextprotocol/ext-auth/blob/main/specification/stable/oauth-client-credentials/2026-07-28/oauth-client-credentials.mdx>):
  "Clients **MUST** authenticate using one of these methods: JWT Authentication (RECOMMENDED)
  ... as defined in RFC 7523 Section 2.2; Client Secret ... as defined in OAuth 2.1 Section
  2.4.1". Its metadata rule: "`token_endpoint_auth_methods_supported`: **MUST** include at least
  one of: `"private_key_jwt"`, `"client_secret_basic"`". RFC 8705 is not mentioned.
- **Situation:** An engine whose own credential is the X.509 certificate of DL13 authenticates
  to the user's authorization server by `tls_client_auth`, as IFC-041 allows. That is not one of
  the extension's methods, so the flow is not the extension, and an authorization server that
  offers only mutual TLS does not meet the extension's metadata rule. An engine built on the
  SDKs' providers for the extension (secret or private-key JWT) cannot follow that branch.
- **Least change:** In IFC-041 drop "or by mutual TLS under RFC 8705", or write "or by mutual
  TLS under RFC 8705, which is outside the extension".

## 5-6-11

- **Severity:** wrong
- **Promises:** IFC-042, IFC-022, PRV-022, IFC-041, DL13
- **Quotes:** IFC-042: "a signed token in the form of a JWT-SVID, sent as the protocol's own
  authorization in the `Authorization` header". PRV-022: "`token` is what the run sends as the
  protocol's authorization at the engine's address". IFC-041: "a token whose audience is the
  item's canonical URI, either in the form of a JWT-SVID that the mechanism of DL13 signs, or
  issued to the engine ... by the authorization server that the item's protected resource
  metadata names". DL13: "the authorization server that the protected resource metadata at the
  engine's address names".
- **Protocol:** S/basic/authorization: "MCP clients **MUST NOT** send tokens to the MCP server
  other than ones issued by the MCP server's authorization server." "MCP servers **MUST**
  implement OAuth 2.0 Protected Resource Metadata ([RFC9728]) ... MCP clients **MUST** use OAuth
  2.0 Protected Resource Metadata for authorization server discovery."
- **Situation:** The protocol's authorization is a token issued by the authorization server that
  the server's metadata names. The run's `token` is written into a file by DL13's mechanism, and
  the engine's token to a `remote` item may be signed by that mechanism too; nothing makes the
  mechanism that authorization server. DL13 may name an OAuth server for consumers on other
  machines while a separate signer issues the runs' tokens: every run then sends, "as the
  protocol's own authorization", a token its client is forbidden to send, and toward a `remote`
  item whose metadata names that OAuth server the engine itself breaks a MUST that IFC-001 binds
  it to.
- **Least change:** In DL13 add: "The authorization server that the protected resource metadata
  at the engine's address names is the issuer of every token that the mechanism signs, and the
  token's `iss` names it."

## 5-6-12

- **Severity:** wrong
- **Promises:** PRV-022, IFC-042
- **Quotes:** PRV-022: "an address so given keeps the host of the engine's address, as a host
  entry or a tunnel that keeps the server name does, so that TLS, the token's audience and the
  protocol's authorization hold as they are." IFC-042: "whose audience is the engine's address
  as its canonical `https` URI".
- **Protocol:** RFC 9728, section 3.3 (<https://www.rfc-editor.org/rfc/rfc9728>): when the
  metadata URL came from `WWW-Authenticate`, "the `resource` value returned MUST be identical to
  the URL that the client used to make the request", and "If these values are not identical, the
  data contained in the response MUST NOT be used." S/basic/authorization lists
  `https://mcp.example.com:8443` as a canonical URI distinct from `https://mcp.example.com`.
- **Situation:** A runtime provider gives a run `CONDUIT_URL=https://engine.example.org:18443/mcp`
  through a tunnel that keeps the host; the engine's address is `https://engine.example.org/mcp`.
  A client that follows the protocol's authorization gets a 401, reads metadata whose `resource`
  is the engine's address, finds it not identical to the URL it used, and must discard it; and a
  token it obtained for its own URL would have the wrong audience. Keeping the host is not
  enough; only a client that sends the file's token blindly gets through.
- **Least change:** In PRV-022 write "keeps the engine's address whole, its scheme, host, port
  and path, as a host entry or a tunnel on the same port does".

## 5-6-13

- **Severity:** wrong
- **Promises:** IFC-037
- **Quotes:** IFC-037: "The protocol has a client keep what it learned of a server for the
  server's process on stdio and for its origin on HTTP, so a discovery result read for one run or
  one connection is not taken for another."
- **Protocol:** S/basic/versioning: "The era determination is a property of the server, not of
  an individual request. Clients **SHOULD** cache the result for the lifetime of the server
  process (stdio) or origin (HTTP)". S/server/utilities/caching: "Servers MUST include caching
  hints on results ... returned by ... `server/discover`"; "If `ttlMs` is positive, the client
  **SHOULD** consider the result fresh for that many milliseconds".
- **Situation:** The protocol's rule about process and origin is about whether a server is
  modern or legacy, not about the discovery result, which it has a client keep for its `ttlMs`.
  The conclusion does not follow either: two runs behind one ingress at
  `https://runs.example.org/a/mcp` and `/b/mcp` share an origin, so the rule quoted would let a
  client reuse one's determination for the other. IFC-037's own rule (read before the first
  call to each run, and at each connection) is sound; only its attribution is wrong.
- **Least change:** Replace the sentence with: "The engine reads it for each run and each
  connection whatever caching hints it carries, so a discovery result read for one is not taken
  for another."

## 5-6-14

- **Severity:** wrong
- **Promises:** SEC-024
- **Quotes:** SEC-024: "the Model Context Protocol accepts `http://` for its authorization URLs
  only on loopback addresses during local development."
- **Protocol:** S/basic/authorization/security-considerations: "1. All authorization server
  endpoints **MUST** be served over HTTPS. 2. All redirect URIs **MUST** be either `localhost`
  or use HTTPS."
- **Situation:** The revision has no exception for local development, none for authorization
  server endpoints, and names `localhost`, not loopback addresses, for redirect URIs only. The
  rule SEC-024 states does not depend on the sentence, but a reader checking the citation does
  not find it.
- **Least change:** Write "the Model Context Protocol has every endpoint of an authorization
  server served over `https`, and allows `http` only for a redirect URI on `localhost`."

## 5-6-15

- **Severity:** wrong
- **Promises:** PRV-025, OPS-027
- **Quotes:** PRV-025: "The bound on a silent connection counts only where the server answers
  with a stream, which the gate then closes and the protocol has the server treat as a
  cancellation: work whose result is sent as one JSON object may run to its end unseen".
- **Protocol:** S/basic/patterns/cancellation: "**Streamable HTTP**: Closing the SSE response
  stream is the cancellation signal. The server **MUST** treat a client disconnect as
  cancellation of that request."
- **Situation:** The protocol makes a disconnect a cancellation whether the server would answer
  with a stream or with one JSON object. A gate that closes the server's HTTP request on a
  silent connection cancels in both cases; what is true is that a server may notice it late.
  Outcomes do not change (OPS-027 makes them unknown), but a provider's contract written to
  PRV-025 would state a bound that it thinks does not apply to JSON answers.
- **Least change:** Write "which the gate then closes and the protocol has the server treat as a
  cancellation, whether it answers with a stream or with one JSON object, though a server may
  notice it late".

## 5-6-16

- **Severity:** misleading
- **Promises:** IFC-012, IFC-025, IFC-027, OPS-005, DL12
- **Quotes:** IFC-012: "The engine answers such a call as the protocol prescribes for an unknown
  tool: with the JSON-RPC error for invalid parameters, code -32602, and with no tool result. The
  error's `data` holds `io.github.shrav-k.conduit/reason` with the value refused". IFC-027: "The
  caller is told the reason, which a consumer receives as IFC-025 states". IFC-025: "The
  `content` member is always present in a tool result, as the protocol requires, and may be an
  empty list"; "A result that reports refused carries none" (no diagnostic).
- **Protocol:** S/server/tools: "**Protocol Errors** indicate issues with the request structure
  itself that models are less likely to be able to fix: Unknown tool ..."; "Clients **MAY**
  provide protocol errors to language models, though these are less likely to result in
  successful recovery. Clients **SHOULD** provide tool execution errors to language models to
  enable self-correction." `_meta` is "to allow clients and servers to attach additional
  metadata" (S/basic); nothing has a client show it to a model. Issue 3003, open, asks whether
  `content` is the error signal a model is meant to see
  (<https://github.com/modelcontextprotocol/modelcontextprotocol/issues/3003>).
- **Situation:** This is the first point that `docs/AUDIT.md` asks about, and it holds, more
  widely than stated. (a) A call to a tool that is not presented, including one whose Conduit has
  just closed, is answered with -32602; a client may keep that from its model, and the reason is
  in `data`, which a host rarely shows. (b) Every other refusal is a tool result with `isError`
  true, which the client should give the model; but the reason is in `_meta`, `structuredContent`
  holds nothing for refused, and `content` may be empty under DL12. So on either path an Agent's
  model may see nothing, or an error with no words, and retry, or tell its user that the tool is
  broken. Nothing is performed and the record is right, so this is not unsafe; but "the caller is
  told" holds only at the client, and the relationship the language declares is invisible to the
  party that decides.
- **Least change:** Narrow DL12 in IFC-025: "A result whose `isError` is true carries in
  `content` one text block that states its reason, or its outcome, and nothing that its result
  may not say under OPS-023", and in IFC-012 "and its `message` says that no tool of that name is
  presented to the instance". The protocol's caching page already has a client re-fetch its list
  after such an error, which then shows the tool gone.

## 5-6-17

- **Severity:** misleading
- **Promises:** `definitions.md` (connection), SEC-013, IFC-041
- **Quotes:** `definitions.md`: "connection | The Model Context Protocol's connection of a client
  to a server: the engine connects to each implementation item ... over stdio or Streamable
  HTTP". SEC-013: "The baseline revision of the protocol keeps no session over Streamable HTTP
  and carries authorization in every HTTP request, so a connection is the engine's holding of
  these values".
- **Protocol:** S/basic/transports/streamable-http: "Revision 2026-07-28 changed the behavior of
  Streamable HTTP ... Removal of protocol-level sessions." S/basic: "an open connection, such as a
  STDIO process, is not a conversation or session".
- **Situation:** The definition attributes the term to the protocol, which at the baseline has
  no connection object on Streamable HTTP, each message being its own POST. SEC-013 says it
  correctly: on HTTP a connection is Conduit's own notion. A reader who takes the definition at
  its word looks for a protocol session that the revision removed.
- **Least change:** Begin the row "The engine's connection to an implementation item: on stdio,
  the process's standard streams; on Streamable HTTP, which in the baseline revision has no
  session, the engine's holding of ...".

## 5-6-18

- **Severity:** minor
- **Promises:** IFC-012, IFC-001, IFC-003, IFC-010, DL12
- **Quotes:** IFC-012 as in 5-6-16. IFC-001: "keep every requirement of the protocol at the
  revision in use". IFC-003: "The meaning of Conduit's keys and of an outcome is the same at every
  revision."
- **Protocol:** SEP-2145, "Standardize `tools/call` failure reporting", pull request 2145
  (<https://github.com/modelcontextprotocol/modelcontextprotocol/pull/2145>): open, labelled
  `in-review`, document status Draft; it would require that servers "**MUST** report tool
  resolution failures as Tool Execution Errors, including: **Unknown tool** ... **Tool not
  callable** ... policy-restricted". The last comment from a person is of 2026-07-31; since then
  only a bot's reminders that its sponsor has been inactive, the latest on 2026-11-02. The draft
  changelog holds nothing of it. At the baseline the -32602 is given by the tools page's example
  and the schema's note "any errors in _finding_ the tool ... should be reported as an MCP error
  response", with no RFC 2119 keyword; the SEP itself records that the TypeScript SDK went back to
  the protocol error in March 2026 while the Python SDK returns an error result.
- **Situation:** This is the second point of `docs/AUDIT.md`. Nothing is wrong today: no revision
  after 2026-07-28 exists. If a later revision adopts SEP-2145 and an engine's DL12 lists it,
  IFC-001 requires an error result for an unknown tool there and IFC-012 requires -32602, so the
  engine cannot keep both. The SEP would also settle 5-6-16 (a): the refusal would arrive as a
  tool execution error, which a client should give its model. For a plain server at such a
  revision, IFC-010 would read an unknown tool as an error result, failed or unknown, safely.
- **Least change:** In IFC-012 write "as the protocol prescribes for an unknown tool at the
  revision in use, which at the baseline revision is the JSON-RPC error for invalid parameters,
  code -32602, ... The reason is in the error's `data`, or in the `_meta` of a result where the
  revision has one answer the call." Keep watching the pull request.

## 5-6-19

- **Severity:** minor
- **Promises:** IFC-038, IFC-013, DL13
- **Quotes:** IFC-038: "The list depends on the instance for which it is served, so it is private
  in the protocol's sense, and no cache serves one instance's list to another." IFC-013: "A
  consumer may hold a tool list for as long as the protocol lets it."
- **Protocol:** S/server/utilities/caching: "`"private"`: ... Cached responses **MAY** be reused
  for the same authorization context. Caches **MUST NOT** be shared across authorization contexts
  (e.g. a different access token requires a different cache)." "Servers **MUST** provide a
  `ttlMs` value that is `>= 0`."
- **Situation:** "Authorization context" is defined only by the example of an access token. Where
  the instance is established by mutual TLS, or by DL13's other means on loopback, a gateway that
  serves two Agents and keys its cache by token sees no token and one context, and may serve one
  instance's list to the other; the calls are still refused, but the names of another
  instance's Conduits are shown. And no promise or DL item sets `ttlMs`, which decides how long a
  consumer may keep a closed Conduit's tools.
- **Least change:** Replace the clause with "and the protocol forbids a cache to share it across
  authorization contexts", and add the tool list's `ttlMs` to DL10.

## 5-6-20

- **Severity:** minor
- **Promises:** IFC-036, IFC-025, IFC-032
- **Quotes:** IFC-036, `when`: "a request that the protocol has a server reject with an error of
  its own: one that lacks a key ..., one whose parameters do not have the form of a `tools/call`
  request, one whose headers on an HTTP transport are missing or disagree with its body, or one
  that carries any of Conduit's keys and does not declare Conduit's extension". Observable:
  "IFC-025 says what the result of every other request holds."
- **Protocol:** S/basic/transports/streamable-http: "If the `Origin` header is present and
  invalid, servers **MUST** respond with HTTP 403 Forbidden."
- **Situation:** A browser-hosted Agent sends a `tools/call` with an `Origin` the engine does not
  accept. The protocol requires 403, which is not in IFC-036's list, so IFC-025 claims the
  request and gives it a tool result that it cannot have; whether it is recorded refused, invalid
  or not at all is open.
- **Least change:** In IFC-036 write "a request that the protocol has a server reject with an
  error or an HTTP status of its own, such as one that lacks ...", adding "one whose `Origin` the
  engine does not accept".

## 5-6-21

- **Severity:** minor
- **Promises:** IFC-042, IFC-022, PRV-022, DL13
- **Quotes:** IFC-042: "A consumer sends a request to the engine's address at an `https` URL."
  PRV-022: "`CONDUIT_URL`, the engine's address". DL13: "the engine's address that it gives a
  run, which on the engine's own machine may be an `http` URL on loopback".
- **Protocol:** S/basic/transports: the standard transports are "stdio: newline-delimited
  messages over the standard streams of a client-launched subprocess" and Streamable HTTP;
  S/basic/transports/stdio: "In the **stdio** transport, the client launches the MCP server as a
  subprocess."
- **Situation:** The engine is reached only at a URL. An Agent whose host connects only to
  servers it launches, which the protocol allows, cannot reach it without a bridge program that
  presents the run's credential, and no promise or DL item names one, so whether such an Agent
  can be used depends on what each engine ships.
- **Least change:** In DL13 add "and how a consumer whose client reaches servers only over stdio
  reaches the engine, such as through a bridge that the engine provides and that presents the
  run's `token`".

---

## Checked without a finding

- `_meta` names: the prefix `io.github.shrav-k.conduit/` (labels begin with a letter and end with
  a letter or digit; the second label is not `modelcontextprotocol` or `mcp`), every key name, and
  the extension identifier `io.github.shrav-k.conduit/implementation-interface` (S/basic, and the
  identifier rule in S/basic/versioning). Trace context in `traceparent`, `tracestate` and
  `baggage` only (IFC-039).
- Error codes: -32602 for an unknown tool and for a missing required `_meta` key with HTTP 400;
  -32020 with 400; -32021 with 400 and `data.requiredCapabilities`, whose schema does not close
  `data`, so Conduit's `reason` beside it is allowed; -32022 and the resend (IFC-003). Conduit
  emits no code of the reserved range that the revision does not define.
- Extension negotiation per request, the settings object, the `revision` string as the
  extensions overview advises, "disabled by default" (IFC-020), a server acting only on what the
  request declares (IFC-036), and the fallback rule "revert to core protocol behavior or reject
  the request" (S/basic/versioning), which IFC-036 and IFC-037 follow.
- Tools: names of 1 to 128 recommended characters, `inputSchema` `{type: object}` for no inputs,
  `outputSchema` of an empty map, `structuredContent` conforming, the stated reading on
  `isError` and `outputSchema` (issue 3003 still open), annotation defaults and the
  `idempotentHint` wording (CTR-002), `$ref` replaced before presenting (IFC-033).
- Lists: `tools` with `listChanged`, notifications on `subscriptions/listen`, `cacheScope:
  private`, and IFC-012's reading of "side effect of other requests on the connection".
- Cancellation by transport and by `tasks/cancel`, and no response after it (OPS-027); the
  tasks extension's stage ("Beta, to Core") and per-request declaration (IFC-016, IFC-036).
- Authorization: optional and for HTTP only, stdio credentials from the environment (secrets
  header), 401 for a missing or invalid token (IFC-032, IFC-042), audience binding, no token
  passed on (IFC-030), no session over Streamable HTTP (SEC-013).
- PRV-023's six variables and the `()` rule, against the stdio clients of the TypeScript SDK
  (`packages/client/src/client/stdio.ts`) and the Python SDK (`src/mcp/client/stdio.py`) on
  `main` of 2026-11-07.
- server.json 2025-12-11: Package fields, `fileSha256`, `transport` limited to `stdio` and
  `streamable-http`, `isSecret`, `valueHint`, `packageArguments`, and the Remote apart from
  5-6-02.

## What was opened

Fetched on 2026-11-07. Pages of the site were read as Markdown; the released specification was
also read from the repository at the tag `2026-07-28`, and every quotation above was matched
against that copy.

- Versioning and revisions: <https://modelcontextprotocol.io/specification/versioning>; S/changelog;
  <https://modelcontextprotocol.io/specification/draft/changelog>; releases,
  <https://github.com/modelcontextprotocol/modelcontextprotocol/releases>.
- S/basic, S/basic/versioning, S/basic/transports, S/basic/transports/stdio,
  S/basic/transports/streamable-http, S/basic/patterns/cancellation,
  S/basic/patterns/subscriptions, S/basic/authorization,
  S/basic/authorization/security-considerations,
  S/basic/authorization/authorization-server-discovery, S/server/tools,
  S/server/utilities/caching, S/deprecated.
- Schema: <https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/schema/2026-07-28/schema.ts>,
  the generated `schema.json` on `main`, and the examples under `InvalidParamsError`; the
  2025-11-25 schema for what a legacy revision provides.
- Extensions overview: <https://modelcontextprotocol.io/extensions/overview>; OAuth Client
  Credentials page and its specification (URL in 5-6-10); tasks extension (URL in 5-6-01), with
  its draft, which differs only in two links.
- server.json: the schema 2025-12-11 and the registry's changelog (URLs in 5-6-02).
- RFC 9728 (<https://www.rfc-editor.org/rfc/rfc9728>), sections 2, 3.1, 3.3 and 5.
- Pull request 2145 and the SEP document on its branch; issue 3003.
