# Check of the Model Context Protocol claims in the implementation interface

## Summary

The promises IFC-001 to IFC-033 in `spec/interface/implementation.yaml` were checked against the
published Model Context Protocol specification on 2026-11-06. Revision 2026-07-28 exists, was
released on 2026-07-28 and is the revision the protocol's site marks **Current**; the draft does
not yet differ from it, so nothing in the file is true only in the draft. Of 31 claims, stated or
relied on, 26 are confirmed (5 of them with a caveat worth a small change of wording), 3 are wrong
and 2 cannot be confirmed. The three that are wrong: the file calls 128 characters a limit of the
protocol, where the protocol only recommends it; the engine's server answers a call to an unknown
tool with a tool result, where the revision prescribes the JSON-RPC error -32602 (the same error
IFC-010 expects of a plain server); and a tool that declares an `outputSchema` returns some results
that are not errors with no `structuredContent`, which the revision forbids. The two that cannot be
confirmed: whether a result whose `isError` is true must conform to `outputSchema` (the revision is
silent and an issue on it is open), and whether "reports refused" in IFC-032 can be kept where the
engine identifies consumers by the protocol's own authorization, which requires HTTP 401. The file
makes no claim about cancellation, progress, or `_meta` on a tool definition.

Date checked: 2026-11-06.

## What was opened

Every page below loaded. Pages of the site were read as Markdown (the same URL with `.md`). Nothing
here rests on a page that did not load. What was not opened is listed at the end of this section.

- Specification index and versioning:
  <https://modelcontextprotocol.io/specification/2026-07-28>,
  <https://modelcontextprotocol.io/specification/versioning>
- Base protocol, with `_meta`, error codes and statelessness:
  <https://modelcontextprotocol.io/specification/2026-07-28/basic>
- Versioning and compatibility:
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning>
- Tools: <https://modelcontextprotocol.io/specification/2026-07-28/server/tools>
- Changelog: <https://modelcontextprotocol.io/specification/2026-07-28/changelog>
- Draft changelog: <https://modelcontextprotocol.io/specification/draft/changelog>
- Cancellation, progress and multi round-trip requests:
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/cancellation>,
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/progress>,
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr>
- Authorization: <https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization>
- Transports: <https://modelcontextprotocol.io/specification/2026-07-28/basic/transports>,
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio>,
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http>
- Caching: <https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/caching>
- Extensions: <https://modelcontextprotocol.io/extensions/overview>,
  <https://modelcontextprotocol.io/extensions/tasks/overview>
- Tasks extension specification:
  <https://github.com/modelcontextprotocol/ext-tasks/blob/main/specification/2026-07-28/tasks.md>
- Open proposal and open issue on tool-call errors:
  <https://github.com/modelcontextprotocol/modelcontextprotocol/pull/2145>,
  <https://github.com/modelcontextprotocol/modelcontextprotocol/issues/3003>
- From the protocol's repository on GitHub, the files in this table:

| What | URL |
| - | - |
| Released schema, read at the tag `2026-07-28` and on `main`; the two files are identical | <https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/schema/2026-07-28/schema.ts> |
| Draft schema on `main`; it differs from the released one only in four links | <https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/draft/schema.ts> |
| Releases | <https://github.com/modelcontextprotocol/modelcontextprotocol/releases> |

Not opened: the text of the SEP documents themselves (only their file names under `seps/`, the
changelog's references to them, and the description of pull request 2145 were read); the pages for
`server/discover`, subscriptions and the authorization sub-pages; the rendered schema reference
page (the TypeScript source was read in its place). The schemas of revisions 2025-03-26, 2025-06-18
and 2025-11-25 were searched for two field names only, and the tools pages of 2025-06-18 and
2025-11-25 for the figure 128 only.

## Status of revision 2026-07-28

- The versioning page says: "The **current** protocol version is **2026-07-28**." It defines
  Current as "the current protocol version, which is ready for use and may continue to receive
  backwards compatible changes".
- The repository has a release `2026-07-28`, published 2026-07-28 and not marked as a pre-release,
  and an earlier `2026-07-28-RC`, published 2026-05-29 and marked as a pre-release.
- The draft changelog holds only "Changes since the most recent release will accumulate here."
  The draft schema is the released schema with its links changed.
- A Current revision can still gain backwards-compatible text. The quotations below are the text as
  it stood on 2026-11-06.

## Claims

In the table, S stands for `https://modelcontextprotocol.io/specification/2026-07-28` and G for
the released schema, whose URL is in the table of the section above. A claim marked "relied on" is
one the file does not state but needs to be true.

| No. | Promise | Claim | Verdict | Governing text and URL | Suggested correction |
| - | - | - | - | - | - |
| 1 | Header, IFC-002, `spec/definitions.md` (baseline revision) | A revision of the protocol dated 2026-07-28 exists and can be a baseline. | Confirmed. It is released and Current, not a candidate or a draft. | "The **current** protocol version is **2026-07-28**." <https://modelcontextprotocol.io/specification/versioning>. Schema: `LATEST_PROTOCOL_VERSION = "2026-07-28"` (G). | |
| 2 | Header, IFC-003, IFC-007 | At the baseline the revision is named for each request in `io.modelcontextprotocol/protocolVersion` in the request's `_meta`. | Confirmed | "Every request declares the protocol version it is using via the `io.modelcontextprotocol/protocolVersion` key in its `_meta` field, and the server accepts or rejects each request independently." <https://modelcontextprotocol.io/specification/versioning>. The key is required: S`/basic` (per-request protocol fields). | |
| 3 | Header, IFC-003 | Either side may support more than one revision, and the protocol's own negotiation picks one. | Confirmed | "Clients and servers **MAY** support multiple protocol versions simultaneously." <https://modelcontextprotocol.io/specification/versioning>. "There is no negotiation handshake. Every request carries its protocol version". S`/basic/versioning` | |
| 4 | IFC-003 | An unsupported revision is rejected with a JSON-RPC error of the protocol, and the request is sent again at a revision both support. | Confirmed. The error is `UnsupportedProtocolVersionError`, code -32022. A server of revision 2025-11-25 or earlier does not return it. | "it **MUST** respond with an `UnsupportedProtocolVersionError` listing the versions it does support"; "The client **SHOULD** select a mutually supported version from the `supported` list and retry the request". S`/basic/versioning` | |
| 5 | IFC-003, IFC-022 (relied on) | Requests are self-contained; there is no `initialize` handshake at the baseline. | Confirmed | "Make MCP stateless: remove the `initialize`/`notifications/initialized` handshake. Every request now carries its protocol version and client capabilities in `_meta`". S`/changelog`, major change 2 | |
| 6 | IFC-001, IFC-004 | A tool is called with `tools/call`, which carries a `name` and `arguments`. | Confirmed | `CallToolRequest`: `method: "tools/call"`; `CallToolRequestParams`: `name: string; arguments?: { [key: string]: unknown }` (G). S`/server/tools` | |
| 7 | IFC-001, IFC-004, IFC-024, `spec/definitions.md` (result) | A tool result has `structuredContent`, which may hold a map of outputs keyed by name. | Confirmed | "This can be any JSON value (object, array, string, number, boolean, or null) that conforms to the tool's `outputSchema` if one is defined." S`/server/tools` | |
| 8 | IFC-004, IFC-006, IFC-020, IFC-023 | A `tools/call` request has `_meta`, and it may hold keys that are not the protocol's own. | Confirmed | `RequestParams { _meta: RequestMetaObject }`, where `RequestMetaObject extends MetaObject` and `MetaObject = Record<string, unknown>` (G). "third-party extensions use their own vendor prefix". S`/basic` | |
| 9 | IFC-004, IFC-009, IFC-025, IFC-026 | A tool result has `_meta`, and it may hold such keys. | Confirmed | `Result { _meta?: ResultMetaObject; resultType: ResultType; ... }` (G) | |
| 10 | IFC-006, `spec/names.yaml` | The prefix `io.github.shrav-k.conduit/` is a valid prefix of a `_meta` key, in reverse-DNS form as the protocol's own keys are, and is not reserved. | Confirmed. The labels are `io`, `github`, `shrav-k` and `conduit`; the second is neither `modelcontextprotocol` nor `mcp`. | "Labels MUST start with a letter and end with a letter or digit; interior characters can be letters, digits, or hyphens"; "Implementations SHOULD use reverse DNS notation"; "Any prefix where the second label is `modelcontextprotocol` or `mcp` is **reserved** for MCP use." S`/basic` | |
| 11 | IFC-007 | The protocol itself requires keys in a request's `_meta`, such as `io.modelcontextprotocol/protocolVersion`. | Confirmed, with a caveat. Two keys are required (`protocolVersion` and `clientCapabilities`); `clientInfo` is recommended, not required; `progressToken` and the trace keys are permitted. Read strictly, "only what the protocol itself requires" would bar the recommended `clientInfo`. | "fields marked as required **MUST** be included on every request"; "Clients **SHOULD** include `io.modelcontextprotocol/clientInfo` on every request unless specifically configured not to do so." S`/basic` | Optional: in IFC-007 write "holds only keys that the protocol itself defines" in place of "holds only what the protocol itself requires". |
| 12 | IFC-004, IFC-008, IFC-010, IFC-025, IFC-026 | `isError` is the flag that marks a tool result as an error, as distinct from a JSON-RPC error. | Confirmed | "Any errors that originate from the tool SHOULD be reported inside the result object, with `isError` set to true, _not_ as an MCP protocol-level error response." (G, `CallToolResult`) | |
| 13 | IFC-010 | A result is complete when its `resultType` is `complete` or absent. | Confirmed | "clients **MUST** treat an absent `resultType` as `"complete"`." S`/basic` | |
| 14 | IFC-010 | A `tools/call` request can be answered by a result whose `resultType` is `input_required`. | Confirmed | "Servers **MAY** respond to `tools/call` with an `InputRequiredResult` to indicate that additional input is needed before the tool call can be completed." S`/server/tools` | |
| 15 | IFC-010 | The JSON-RPC error that says the tool is unknown or the request malformed is invalid-params, code -32602. | Confirmed, with caveats. The same code is also the one for invalid tool arguments and for a request that lacks a required `_meta` key. A message that is not valid JSON-RPC at all gets -32600 or -32700, which IFC-010 then reads as "any other error". | "**Protocol Errors** indicate issues with the request structure itself ...: Unknown tool; Malformed requests (requests that fail to satisfy CallToolRequest schema); Server errors", with the example `"code": -32602, "message": "Unknown tool: invalid_tool_name"`. S`/server/tools`. `InvalidParamsError`: "**Tools**: Unknown tool name or invalid tool arguments" (G) | Optional: in IFC-010 write "the request's parameters malformed" in place of "the request malformed", in both places. |
| 16 | IFC-012 | The characters the protocol allows in a tool name are letters, digits, `_`, `-` and `.`, so dots are allowed. | Confirmed, with a caveat: the letters are ASCII letters, and the rule is a recommendation. | "The following **SHOULD** be the only allowed characters: uppercase and lowercase ASCII letters (A-Z, a-z), digits (0-9), underscore (\_), hyphen (-), and dot (.)". Example valid name: `admin.tools.list`. S`/server/tools` | Optional: write "ASCII letters". |
| 17 | IFC-012, IFC-014 | "The protocol limits a tool name to 128 characters"; no name "is longer than the protocol allows". | Wrong as worded. The figure is right and the force is not: it is a recommendation, and the schema has no limit on `name`. | "Tool names **SHOULD** be between 1 and 128 characters in length (inclusive)." S`/server/tools`. "Tool and prompt names are only **SHOULD**-constrained to header-safe characters". S`/basic/transports/streamable-http` | See correction A below. |
| 18 | IFC-012, IFC-013 (relied on) | A server may present a different tool list to each consumer, and the list may change. | Confirmed, on the condition IFC-022 already sets: the list follows what the request presents, not the connection. | "This set **MAY** be empty and **MAY** change over time ..., but **MUST NOT** vary per-connection or as a side effect of other requests on the connection. The set **MAY** vary by the authorization presented on the request". S`/server/tools` | |
| 19 | IFC-012 with IFC-025 and IFC-032 (relied on) | A server may answer a call that names a tool it does not present with a tool result (`isError` true and a reason in `_meta`). | Wrong against the revision's text, which makes an unknown tool a protocol error. IFC-010 expects exactly that error of a plain server, so the file is not even with itself. A proposal to change the protocol's rule (SEP-2145) is open and not accepted. | "any errors in _finding_ the tool, an error indicating that the server does not support tool calls, or any other exceptional conditions, should be reported as an MCP error response." (G, `CallToolResult`). "Unknown tool" is listed under Protocol Errors, "returned as standard JSON-RPC errors". S`/server/tools`. Open proposal: <https://github.com/modelcontextprotocol/modelcontextprotocol/pull/2145> | See correction B below. |
| 20 | IFC-033 | A tool definition has `inputSchema`, which describes a map, and `outputSchema`. | Confirmed | `inputSchema: { $schema?: string; type: "object"; [key: string]: unknown }`; `outputSchema?: { $schema?: string; [key: string]: unknown }` (G). Replacing a `$ref` agrees with "Implementations **MUST NOT** automatically dereference `$ref` values that resolve to a network URI." S`/basic` | |
| 21 | IFC-033 with IFC-025 (relied on) | A tool that declares an `outputSchema` may return a result that is not an error and has no `structuredContent`: a result whose content is withheld, and a repeat of an invocation that has not ended. | Wrong against the revision's text | "If an output schema is provided: Servers **MUST** provide structured results that conform to this schema. Clients **SHOULD** validate structured results against this schema." S`/server/tools` | See correction C below. |
| 22 | IFC-033 with IFC-008, IFC-025, IFC-026 (relied on) | A tool that declares an `outputSchema` may return a result whose `isError` is true and that has no `structuredContent`, or has `diagnostics` there in place of the outputs. | Cannot be confirmed. The sentence quoted in row 21 makes no exception for a result that is an error, and nothing else in the revision says whether one applies. An issue asking exactly this is open. | As row 21. Open issue: "structuredContent with outputSchema ... is undefined for errors." <https://github.com/modelcontextprotocol/modelcontextprotocol/issues/3003> | See correction D below. |
| 23 | IFC-025 | A tool result has a `content` member, to which Conduit gives no meaning. | Confirmed, with a caveat: the member is required, so it is present even when empty, and the protocol recommends a serialized copy of the structured content there. | `CallToolResult { content: ContentBlock[]; structuredContent?: unknown; isError?: boolean }` (G). "For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block." S`/server/tools` | Optional: in IFC-025 say that `content` is always present and is an empty list where the engine puts nothing there. |
| 24 | IFC-016 | The protocol has a tasks extension named `io.modelcontextprotocol/tasks`. | Confirmed, with a caveat: it is an official extension outside the core revision, and its stage is "Beta, to Core", in which breaking changes are allowed. Its identifier does not change with its stage. | "Move experimental tasks out of the core protocol and into an official extension (`io.modelcontextprotocol/tasks`)." S`/changelog`, major change 6. "This extension is identified as: `io.modelcontextprotocol/tasks`." <https://github.com/modelcontextprotocol/ext-tasks/blob/main/specification/2026-07-28/tasks.md>. Stage: <https://modelcontextprotocol.io/extensions/overview> | Optional: in IFC-016 write "The protocol's tasks extension, `io.modelcontextprotocol/tasks`, which is not part of the baseline revision, may also be used". |
| 25 | Header, IFC-011 | The protocol gives guidance for stateful tools: a handle is an opaque string, it permits nothing by being held, the server checks it on every call, and an unknown handle gives a tool error. | Confirmed. The guidance is marked non-normative. | "a handle is a name, not a capability. The server should validate the caller's authorization against the handle on every call."; "opaque identifiers do not"; "A call against an expired or unknown handle should return a tool execution error that says so". S`/server/tools`, Stateful Tools | |
| 26 | IFC-017, IFC-001 (`command` or `url`) | The protocol has transports to choose among. | Confirmed: stdio and Streamable HTTP, and custom transports are permitted. On stdio it is the client that starts the server's process. | "the standard transports: 1. stdio ... 2. Streamable HTTP"; "Clients and servers **MAY** implement additional custom transport mechanisms". S`/basic/transports`. "the client launches the MCP server as a subprocess." S`/basic/transports/stdio` | |
| 27 | IFC-022 (relied on) | A server can and should establish who a request comes from for that request alone, not from the connection. | Confirmed. The protocol's own `clientInfo` cannot serve, because it is not verified, so DL13 has to name something else. | "Servers **MUST NOT** rely on prior requests over the same connection to establish context (e.g., capabilities, protocol version, client identity)." S`/basic`. "`clientInfo` and `serverInfo` are self-reported by the sender and are not verified by the protocol ... **SHOULD NOT** rely on them for security decisions." S`/basic` | |
| 28 | IFC-032 with DL13 (relied on) | A request from which no instance can be established can always be answered "refused", as a tool result. | Cannot be confirmed: it depends on DL13. Authorization is optional in the protocol, but where an engine uses the protocol's authorization on an HTTP transport, a request with a missing or invalid token must be answered with HTTP 401, and no tool result can be returned. | "Authorization is **OPTIONAL** for MCP implementations."; "Invalid or expired tokens **MUST** receive a HTTP 401 response." S`/basic/authorization` | See correction E below. |
| 29 | IFC-030 (relied on) | Not passing a consumer's credential on to an implementation agrees with the protocol. | Confirmed | "MCP servers **MUST** only accept tokens that are valid for use with their own resources. MCP servers **MUST NOT** accept or transit any other tokens." S`/basic/authorization` | |
| 30 | IFC-004 | The baseline provides `tools/call` with `name` and `arguments`, `_meta` on the request and the result, `structuredContent` and `isError`; some other revisions do not. | Confirmed. Of the earlier revisions, 2025-06-18 and 2025-11-25 have `structuredContent` in their schemas and 2025-03-26 does not. | Rows 6 to 9 and 12. Earlier schemas: <https://github.com/modelcontextprotocol/modelcontextprotocol/tree/main/schema> | |
| 31 | IFC-001 (relied on) | One server has one tool of a given name. | Confirmed | "Tool names **SHOULD** be unique within a server." S`/server/tools` | |

Count by verdict: confirmed 26 (of which rows 11, 15, 16, 23 and 24 carry a caveat); wrong 3 (rows
17, 19 and 21); cannot be confirmed 2 (rows 22 and 28); true only in the draft 0.

## Suggested corrections

Each is the least change of wording that makes the text true. Corrections B, C, D and E touch
behavior or state a departure from the protocol, so each is for the owner to rule on. None of them
has been applied.

### A. Row 17: the 128 characters are a recommendation (IFC-012, IFC-014)

- In IFC-012, replace "The protocol limits a tool name to 128 characters." with "The protocol says
  that a tool name should be at most 128 characters, and this specification makes that a limit."
- In IFC-012, replace "so only a Conduit made from a composite can pass the limit" with nothing
  new: it stays true once the sentence before it is corrected.
- In IFC-014, replace "none is longer than the protocol allows" with "none is longer than the 128
  characters that the protocol recommends".

### B. Row 19: an unknown tool on the engine's server (IFC-012)

The revision prescribes the JSON-RPC error -32602 for a call that names an unknown tool. IFC-012
with IFC-025 has the engine answer with a tool result that carries the reason refused. Two ways to
make the text true:

- Keep the behavior and say that it departs. Add to the observable of IFC-012: "The engine answers
  such a call with a tool result that carries the reason, as IFC-025 states, and not with the
  invalid-params error, code -32602, that the protocol's revision 2026-07-28 prescribes for an
  unknown tool."
- Or follow the protocol: a call that names a tool the engine has never presented to the instance
  is answered with -32602 and is still recorded as refused. This changes behavior, and the result
  then has no `_meta` in which to carry Conduit's keys.

The first is the smaller change. The protocol may move toward it: pull request 2145 (SEP-2145),
open and not merged on 2026-11-06, proposes that an unknown tool be reported as a tool result
whose `isError` is true, and says the official SDKs already behave so. That last point also bears
on IFC-010: a plain server built on such an SDK may answer an unknown tool with `isError` true
and not with -32602. IFC-010's table still gives a safe outcome for it.

### C. Row 21: a result that is not an error and has no `structuredContent` (IFC-025, IFC-033)

Add to the observable of IFC-025: "The protocol requires a result of a tool that declares an
`outputSchema` to carry `structuredContent` that conforms to it. A result whose content is
withheld, and a repeat of an invocation that has not ended, carry none. A consumer that validates
results against `outputSchema` does not treat such a result as invalid: it reads
`io.github.shrav-k.conduit/withheld` and `io.github.shrav-k.conduit/state` first."

That states the departure and keeps the behavior. To remove the departure instead, the owner
would have to change what such a result carries, or what `outputSchema` IFC-033 derives; either
is a change of behavior and is not proposed here.

### D. Row 22: a result that is an error (IFC-025)

Add to the observable of IFC-025: "The protocol's revision 2026-07-28 does not say whether a
result whose `isError` is true must conform to `outputSchema`. This specification takes it that
it need not: such a result carries no outputs, and may carry `diagnostics`."

### E. Row 28: refusing a client that is not an instance (IFC-032)

Add to the observable of IFC-032: "Where what a request presents is a credential of the protocol's
own authorization on an HTTP transport, a request whose credential is missing or invalid is
answered as the protocol requires, with HTTP 401, and with no tool result. It is recorded as
refused all the same."

The other way is to narrow DL13 so that an engine may not choose a means of identifying consumers
that prevents it from returning a tool result. That is a larger change.

### Optional changes for the rows that carry a caveat

- Row 11, IFC-007: "holds only keys that the protocol itself defines".
- Row 15, IFC-010: "the request's parameters malformed".
- Row 16, IFC-012: "ASCII letters".
- Row 23, IFC-025: say that `content` is always present, and empty where the engine puts nothing
  there.
- Row 24, IFC-016: say that the tasks extension is not part of the baseline revision.

## What the file does not claim

The cold reader's list expected claims on these subjects. The file makes none, so there was
nothing to classify. What the revision says is recorded for whoever writes them later.

- **Cancellation.** No promise of the file mentions it; DL14 of `spec/implementation-defined.yaml`
  leaves it to the engine. The revision: on stdio "The client **MUST** send a
  `notifications/cancelled` notification referencing the request ID"; on Streamable HTTP "Closing
  the SSE response stream is the cancellation signal." A server "**SHOULD**" stop and "Not send a
  response for the cancelled request". A task of the tasks extension is cancelled with
  `tasks/cancel`, never with the notification.
- **Progress.** No claim. The revision: a request opts in with `progressToken` in its `_meta`,
  and the server "**MAY**" then send `notifications/progress`.
- **`_meta` on a tool definition.** No claim; IFC-033 puts nothing there. The schema allows it:
  `Tool { ...; _meta?: MetaObject }`.
- **How a client identifies itself.** The file leaves it to DL13. The revision offers
  `io.modelcontextprotocol/clientInfo`, which is unverified (row 27), and OAuth 2.1 bearer tokens
  on HTTP transports, sent "in every HTTP request from client to server" (row 28). On stdio it
  says to "retrieve credentials from the environment".

## Requirements of the revision that the file is silent on

These are not claims of the file, so they are not in the count. Each is something an engine or an
implementation built to the baseline must do, and a reader of the file alone would not learn it.

- Every request must also carry `io.modelcontextprotocol/clientCapabilities` in its `_meta`. A
  request without a required key "is malformed; the server **MUST** reject it with JSON-RPC error
  code `-32602`".
- "Servers **MUST** implement `server/discover`." This binds the server the engine presents under
  IFC-012 and every implementation.
- A `tools/list` result must carry `ttlMs` and `cacheScope`. A list that differs for each consumer
  is `"private"`. A client may use a cached list while it is fresh, so IFC-013's "next tool list"
  is the next one the engine serves, not necessarily the next one a client uses.
- On Streamable HTTP the tool name is also sent in the `Mcp-Name` header, and a mismatch with the
  body is rejected with -32020.
- A server that negotiated the tasks extension may answer `tools/call` with a result whose
  `resultType` is `task`. IFC-010's table names `complete`, absent and `input_required` only.
  The revision says "A `resultType` of any value unrecognized by the client **MUST** be considered
  invalid."
- A server must not put in `inputRequests` a request for a capability the client did not declare,
  but it may still answer `input_required` with a `requestState` alone, so the case in IFC-010
  can arise even for an engine that declares no capability.
