# Second check of the Model Context Protocol claims in the implementation interface

## Summary

The promises IFC-001 to IFC-035 in `spec/interface/implementation.yaml`, and the places in
`spec/definitions.md`, `spec/implementation-defined.yaml`, `spec/core/` and the conventions that
repeat them, were read as they stand and checked against the published Model Context Protocol
specification at revision 2026-07-28, which is still the revision the protocol's site marks
Current. Of 48 claims, stated or relied on, 44 are confirmed (5 of them with a caveat worth a
small change of wording), 1 is wrong as worded and 3 cannot be confirmed. The wrong one is new:
IFC-025 says that only two kinds of request are answered with no tool result, where the protocol
has a server answer several others with an error of its own (an unsupported revision, a missing
required `_meta` key, malformed parameters, a header that disagrees with the body). The three that
cannot be confirmed: that a result whose `isError` is true need not conform to `outputSchema`
(the revision still does not say, and the text now states this as its own reading); that every
result that is not an error carries conforming `structuredContent` where the operation declares
no outputs (the file does not say what such a tool declares or returns); and that a consumer's
tool list may change because of a request the same consumer made, such as `open.v1` (the protocol
forbids a list that varies "as a side effect of other requests on the connection", and whether
that covers this case is a matter of reading). Of the five problems of the
[earlier check](../round-2/mcp-citations.md), three are resolved, one is resolved for the cases
that check named and leaves the no-outputs case, and one cannot be resolved from the protocol and
is now stated openly as the specification's reading.

Date checked: 2026-11-06. Commit read: `f583469` on `docs/plan-audit`, with a clean working tree.

## What was opened

Every page below loaded on 2026-11-06. Pages of the site were fetched as Markdown (the same URL
with `.md`) and read in full unless a note says otherwise. Nothing in this report rests on a page
that did not load.

- Specification index and versioning:
  <https://modelcontextprotocol.io/specification/2026-07-28>,
  <https://modelcontextprotocol.io/specification/versioning>
- Base protocol (messages, error codes, statelessness, `_meta`, JSON Schema usage):
  <https://modelcontextprotocol.io/specification/2026-07-28/basic>
- Versioning and compatibility:
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning>
- Tools: <https://modelcontextprotocol.io/specification/2026-07-28/server/tools>
- Discovery: <https://modelcontextprotocol.io/specification/2026-07-28/server/discover>
- Caching: <https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/caching>
- Changelog, and the draft changelog, which still holds only "Changes since the most recent
  release will accumulate here.":
  <https://modelcontextprotocol.io/specification/2026-07-28/changelog>,
  <https://modelcontextprotocol.io/specification/draft/changelog>
- Transports: <https://modelcontextprotocol.io/specification/2026-07-28/basic/transports>,
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http>;
  of the stdio page only the sentences on who starts the process
- Multi round-trip requests and the message patterns overview (the normative sentences only):
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr>,
  <https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns>
- Authorization (the sections on protocol requirements, token requirements, token handling and
  error handling): <https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization>
- Extensions overview (the stages and the table of official extensions):
  <https://modelcontextprotocol.io/extensions/overview>
- Tasks extension specification, last changed 2026-10-09:
  <https://github.com/modelcontextprotocol/ext-tasks/blob/main/specification/2026-07-28/tasks.md>
- JSON-RPC 2.0, the error object: <https://www.jsonrpc.org/specification>
- From the protocol's repository on GitHub, the files in this table:

| What | URL |
| - | - |
| Released schema, read at the tag `2026-07-28`; the copy on `main` is identical | <https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/schema/2026-07-28/schema.ts> |
| Draft schema on `main`; it differs from the released one only in four links | <https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/draft/schema.ts> |
| Schema examples for an unknown tool, invalid arguments and a tool error | <https://github.com/modelcontextprotocol/modelcontextprotocol/tree/2026-07-28/schema/2026-07-28/examples> |
| Schemas of revisions 2025-03-26, 2025-06-18 and 2025-11-25, searched for `_meta` on `Tool`, `outputSchema` and `structuredContent` only | <https://github.com/modelcontextprotocol/modelcontextprotocol/tree/main/schema> |
| Releases: `2026-07-28` (2026-07-28, not a pre-release) is the latest | <https://github.com/modelcontextprotocol/modelcontextprotocol/releases> |
| SEP-2567, status Final, the section on list endpoints | <https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/seps/2567-sessionless-mcp.md> |
| Pull request 2145 (SEP-2145), open, labelled `in-review`, last updated 2026-11-02; its description only | <https://github.com/modelcontextprotocol/modelcontextprotocol/pull/2145> |
| Issue 3003, open, last updated 2026-10-31 | <https://github.com/modelcontextprotocol/modelcontextprotocol/issues/3003> |
| TypeScript SDK client and server, `main` of 2026-10-31, the code that validates a result against `outputSchema` | <https://github.com/modelcontextprotocol/typescript-sdk/blob/main/packages/client/src/client/client.ts>, <https://github.com/modelcontextprotocol/typescript-sdk/blob/main/packages/server/src/server/mcp.ts> |
| Python SDK client, `main` of 2026-11-05, the same code | <https://github.com/modelcontextprotocol/python-sdk/blob/main/src/mcp/client/session.py> |

Not opened or not read: the document of SEP-2145 itself; the rendered schema reference page (the
TypeScript source was read in its place); the pages for cancellation, progress, subscriptions,
pagination, resources and prompts; the authorization sub-pages; the rest of the stdio page. The
file makes no claim that rests on them. The SDKs are not the specification: they are cited only
as evidence of how the protocol's own implementations read a point on which the text is silent.

## The five earlier problems

The row numbers are those of the earlier check.

| Problem | What the text says now | Resolved or not |
| - | - | - |
| Row 17. The file called 128 characters a limit of the protocol, which only recommends it. | IFC-012: "The protocol says that a tool name should be at most 128 characters, and this specification makes that a limit." IFC-014: "none is longer than the 128 characters that the protocol recommends". The row for tool name in `spec/definitions.md` says the same. No sentence anywhere under `spec/` still calls it the protocol's limit. | Resolved. The protocol's sentence is unchanged: "Tool names **SHOULD** be between 1 and 128 characters in length (inclusive)." |
| Row 19. The engine answered a call to a tool it does not present with a tool result, where the revision prescribes the JSON-RPC error -32602. | IFC-012: the engine answers "with the JSON-RPC error for invalid parameters, code -32602, and with no tool result. The error's `data` holds `io.github.shrav-k.conduit/reason` with the value refused". IFC-013, IFC-025, IFC-032, OPS-005, `spec/definitions.md` and `spec/core/README.md` agree. | Resolved. The revision lists "Unknown tool" under protocol errors with the example code -32602, and lets an error's `data` hold anything (rows 24 and 25 below). Pull request 2145, which would turn an unknown tool into a result with `isError` true, is still open. |
| Row 21. A tool that declares `outputSchema` returned results that were not errors and had no `structuredContent`: a withheld result and a repeat of an invocation in progress. | IFC-025: `isError` is true where the content is withheld, where the request is a repeat of an invocation that has not ended, and where it is a repeat of one that succeeded and whose outputs are no longer kept. "A result whose `isError` is not true has the outcome succeeded and holds the operation's outputs in its `structuredContent`, which conforms to the `outputSchema` of IFC-033". | Resolved for the cases the earlier check named, on the reading of row 22. One case is left unstated: the result of an operation that declares no outputs, which is not an error and for which the file says neither whether the tool declares an `outputSchema` nor whether the result carries `structuredContent` (row 22 below). |
| Row 22. Whether a result whose `isError` is true must conform to `outputSchema` could not be confirmed. | IFC-025: "The protocol's revision 2026-07-28 does not say whether a result whose `isError` is true must conform to `outputSchema`. This specification takes it that it need not: such a result carries no outputs, and may carry `diagnostics`." | Not resolved by the protocol, and no longer hidden. The revision is unchanged and issue 3003 is still open, so the reading still cannot be confirmed (row 21 below). The text now says truthfully that it is the specification's own reading. The official TypeScript and Python SDKs both skip the check for a result whose `isError` is true, which supports the reading without making it the protocol's. |
| Row 28. "Reports refused" could not always be a tool result: where the protocol's own authorization is used over HTTP, an invalid token must receive HTTP 401. | IFC-032: "Where what a request presents is a credential of the protocol's own authorization on an HTTP transport, a request whose credential is missing or invalid is answered as the protocol requires, with HTTP 401, and with no tool result. It is recorded as refused all the same." | Resolved. The sentence agrees with the revision (row 47 below, with a caveat about HTTP 403). |

## Claims

In the table, S stands for `https://modelcontextprotocol.io/specification/2026-07-28` and G for the
released schema, whose URL is in the table of the section above. A claim marked "relied on" is
one the file does not state but needs to be true. Where a promise is named without a file it is
in `spec/interface/implementation.yaml`.

| No. | Promise | Claim | Verdict | Governing text and URL | Suggested correction |
| - | - | - | - | - | - |
| 1 | Header, IFC-002, `spec/definitions.md` (baseline revision), `spec/schema-profile.md` | A revision of the protocol dated 2026-07-28 exists and can be a baseline. | Confirmed. It is the latest release and is marked Current. | "The **current** protocol version is **2026-07-28**." <https://modelcontextprotocol.io/specification/versioning>. `LATEST_PROTOCOL_VERSION = "2026-07-28"` (G). | |
| 2 | Header, IFC-003, IFC-007 | At the baseline the revision is named for each request in `io.modelcontextprotocol/protocolVersion` in the request's `_meta`; the negotiation is per request. | Confirmed. On Streamable HTTP the same value is also sent in the `MCP-Protocol-Version` header. | "There is no negotiation handshake. Every request carries its protocol version, and the server accepts or rejects each request independently". S`/basic/versioning`. | |
| 3 | Header, IFC-003, DL12 of `spec/implementation-defined.yaml` | Either side may support more than one revision. | Confirmed | "Clients and servers **MAY** support multiple protocol versions simultaneously." <https://modelcontextprotocol.io/specification/versioning> | |
| 4 | IFC-003 | A request at an unsupported revision is rejected with a JSON-RPC error of the protocol and is sent again at a revision both support. | Confirmed. The error is `UnsupportedProtocolVersionError`, code -32022, with HTTP 400 on Streamable HTTP. A server of revision 2025-11-25 or earlier does not return it. | "it **MUST** respond with an `UnsupportedProtocolVersionError` listing the versions it does support"; "The client **SHOULD** select a mutually supported version from the `supported` list and retry the request". S`/basic/versioning` | |
| 5 | IFC-001 | The protocol requires keys in the `_meta` of every request. | Confirmed. Two are required: `io.modelcontextprotocol/protocolVersion` and `io.modelcontextprotocol/clientCapabilities`. | "fields marked as required **MUST** be included on every request"; "A request missing any required field is malformed; the server **MUST** reject it with JSON-RPC error code `-32602` (Invalid params). On HTTP, the response status **MUST** be `400 Bad Request`." S`/basic` | |
| 6 | IFC-001 | The protocol requires a server to implement something beside its tools. | Confirmed: `server/discover`. A server that has tools must also declare the `tools` capability there. | "Servers **MUST** implement `server/discover`." S`/basic/versioning` and S`/server/discover` | |
| 7 | IFC-001 | The protocol requires a tool list to carry certain members. | Confirmed: `ttlMs` and `cacheScope`. A list that differs for each consumer is `private`. | "Servers MUST include caching hints on results with `resultType: "complete"` returned by the following operations: ... `tools/list`"; private "is appropriate for ... filtered list results that vary per user". S`/server/utilities/caching` | |
| 8 | IFC-001 (relied on; the decisions name it) | On Streamable HTTP the tool's name is also sent in the `Mcp-Name` header, which the server checks against the body. | Confirmed. A name made as IFC-012 makes it is plain ASCII and needs no encoding. | "`Mcp-Name` ... `params.name` or `params.uri` ... `tools/call`, `resources/read`, `prompts/get` requests. These headers are **REQUIRED** for compliance."; "Servers **MUST** reject requests with a `400 Bad Request` HTTP status and JSON-RPC error code `-32020` (`HeaderMismatch`) if any validation fails." S`/basic/transports/streamable-http` | |
| 9 | IFC-001, IFC-004 | A tool is called with `tools/call`, which carries a `name` and `arguments`. | Confirmed | `CallToolRequestParams`: `name: string; arguments?: { [key: string]: unknown }` (G). S`/server/tools` | |
| 10 | IFC-001, IFC-004, IFC-024, IFC-035, `spec/definitions.md` (result) | A tool result has `structuredContent`, which may hold a map of outputs keyed by name. | Confirmed | "This can be any JSON value (object, array, string, number, boolean, or null) that conforms to the tool's `outputSchema` if one is defined." S`/server/tools` | |
| 11 | IFC-004, IFC-006, IFC-020, IFC-023, IFC-024 | A request has `_meta`, and it may hold keys that are not the protocol's own. | Confirmed | `RequestParams { _meta: RequestMetaObject }`; `RequestMetaObject extends MetaObject`; `MetaObject = Record<string, unknown>` (G). "third-party extensions use their own vendor prefix". S`/basic` | |
| 12 | IFC-004, IFC-009, IFC-025, IFC-026, IFC-035 | A tool result has `_meta`, and it may hold such keys. | Confirmed | `Result { _meta?: ResultMetaObject; resultType: ResultType; ... }` (G) | |
| 13 | IFC-006, `spec/names.yaml` | The prefix `io.github.shrav-k.conduit/` is a valid prefix of a `_meta` key, in reverse-DNS form, and is not reserved. | Confirmed. The names after it (`reason`, `restrictions`, `operationIdentity` and the rest) begin and end with a letter. | "Labels MUST start with a letter and end with a letter or digit; interior characters can be letters, digits, or hyphens"; "Any prefix where the second label is `modelcontextprotocol` or `mcp` is **reserved** for MCP use." S`/basic` | |
| 14 | IFC-007 | A request to a plain implementation holds in `_meta` only keys that the protocol itself defines, such as `io.modelcontextprotocol/protocolVersion`. | Confirmed. The earlier wording ("requires") is gone. | The table of reserved keys and the table of per-request fields. S`/basic` | |
| 15 | IFC-004, IFC-034, `spec/definitions.md` (Conduit's keys) | A tool's definition has `_meta`, and a server may put its own prefixed keys there. | Confirmed. The prose list "A tool definition includes" on the tools page does not mention `_meta`; the schema, which the base page calls "the source of truth", has it. The key rules are those of every `_meta` (row 13). | `Tool { ...; annotations?: ToolAnnotations; _meta?: MetaObject }` (G) | |
| 16 | IFC-004 | Some revisions do not provide what the interface relies on, `_meta` on a tool's definition included. | Confirmed. The schemas of 2025-06-18 and 2025-11-25 have `_meta` on `Tool`, `outputSchema` and `structuredContent`; that of 2025-03-26 has none of the three. | <https://github.com/modelcontextprotocol/modelcontextprotocol/tree/main/schema> | |
| 17 | IFC-004, IFC-008, IFC-009, IFC-010, IFC-026 | `isError` is the flag that marks a tool result as an error, and a tool reports its own errors with it. | Confirmed | "Any errors that originate from the tool SHOULD be reported inside the result object, with `isError` set to true, _not_ as an MCP protocol-level error response." (G, `CallToolResult`) | |
| 18 | IFC-025, IFC-035, ACC-034, OPS-009, `spec/definitions.md` (result, withheld), `spec/core/README.md` (relied on) | A server may set `isError` to true on a result that carries no outputs although the invocation succeeded or has not ended: withheld content, a repeat of an invocation in progress, a repeat whose outputs are no longer kept. | Confirmed, with a caveat. Nothing in the revision limits when a server sets the flag. The protocol's own meaning is narrower than Conduit's, and a client is told to give such a result to its model as an error to correct, so a model sees a succeeded and withheld result as a failed call unless it reads Conduit's keys. | `isError`: "Whether the tool call ended in an error." (G). "Clients **SHOULD** provide tool execution errors to language models to enable self-correction." S`/server/tools` | Optional: in IFC-025, after "The result's `isError` is true in these cases and in no other", add "The protocol calls `isError` the mark of a call that ended in an error; in a result that the engine gives a consumer it means only that the result carries no outputs, and the keys beside it say whether the invocation failed." |
| 19 | IFC-025, IFC-033 | The protocol requires a tool that declares an `outputSchema` to return `structuredContent` that conforms to it. | Confirmed | "If an output schema is provided: Servers **MUST** provide structured results that conform to this schema. Clients **SHOULD** validate structured results against this schema." S`/server/tools` | |
| 20 | IFC-025 | "The protocol's revision 2026-07-28 does not say whether a result whose `isError` is true must conform to `outputSchema`." | Confirmed as a statement of silence. No sentence of the tools page or of the schema speaks of an error result and `outputSchema` together; the examples of an error result carry no `structuredContent` and belong to no tool that declares an `outputSchema`. The sentence of row 19 is unqualified, so a strict reader can take it to cover error results too. | As row 19. Issue 3003, open: "structuredContent with outputSchema ... is undefined for errors." <https://github.com/modelcontextprotocol/modelcontextprotocol/issues/3003> | |
| 21 | IFC-025, IFC-033, IFC-009, IFC-035 (the stated reading) | A result whose `isError` is true need not conform to `outputSchema`: it may have no `structuredContent`, or `diagnostics` there in place of the outputs. | Cannot be confirmed. The revision does not except error results from the sentence of row 19, and nothing else in it settles the point. Both official SDKs behave as the reading has it: the TypeScript client checks "`result.structuredContent === undefined && !result.isError`" and validates "not when there's an error", its server returns early on `result.isError`, and the Python client validates only where `not result.is_error`. Pull request 2145, still open, would make a missing or non-conforming `structuredContent` a tool execution error. | As rows 19 and 20. SDK code: the URLs in the table of the section above. <https://github.com/modelcontextprotocol/modelcontextprotocol/pull/2145> | None needed: IFC-025 already says that this is the specification's reading and not the protocol's. Look again when issue 3003 or pull request 2145 is settled. |
| 22 | IFC-025, IFC-033, ACC-034 (relied on) | Every result that is not an error carries `structuredContent` that conforms to the tool's `outputSchema`, also where the operation declares no outputs, as `retire-with-resources.v1`, `retire.v1` and `unregister.v1` do. | Cannot be confirmed, because the file is silent on its own side. IFC-033 derives `outputSchema` "from the contract's outputs in the same way", and does not say whether an operation with no outputs gives a schema of an empty map or no `outputSchema`. IFC-025 does not say whether the result then carries `structuredContent`. If the tool declares a schema and the result carries none, the sentence of row 19 is broken, and the official clients reject the result ("has an output schema but did not return structured content"). | As row 19 | In IFC-025, write "holds the operation's outputs in its `structuredContent`, which is present on every such result and is an empty map where the operation declares no outputs". The other way is one sentence in IFC-033: "The tool of an operation that declares no outputs has no `outputSchema`." Either makes the claim true; the first keeps one rule for every tool. |
| 23 | IFC-025, DL12 | The `content` member is always present in a tool result and may be an empty list. | Confirmed, with a caveat. The member is required and has no minimum length. The revision also recommends a serialized copy of the structured content there, which an engine that leaves the list empty beside `structuredContent` does not follow. IFC-001 binds the engine to the protocol's requirements, and a reader cannot tell whether a recommendation is one. | `CallToolResult { content: ContentBlock[]; structuredContent?: unknown; isError?: boolean }` (G). "For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block." S`/server/tools` | Optional: in IFC-025, after "and may be an empty list", add "The protocol recommends that a result with `structuredContent` also carry it serialized in `content`; whether the engine does so is DL12." |
| 24 | IFC-012, IFC-013, IFC-025, IFC-032, OPS-005, `spec/definitions.md` (tool, result), `spec/core/README.md` | The protocol prescribes, for a call that names an unknown tool, the JSON-RPC error for invalid parameters, code -32602, and no tool result. | Confirmed. The code is given by the example and by the schema's note on the error, not by a sentence with MUST. Pull request 2145 (SEP-2145), open and in review on 2026-11-06, proposes the opposite: an unknown tool as a result with `isError` true. | "**Protocol Errors** ...: Unknown tool; Malformed requests ...; Server errors. They are returned as standard JSON-RPC errors", with `"code": -32602, "message": "Unknown tool: invalid_tool_name"`. S`/server/tools`. `InvalidParamsError`: "**Tools**: Unknown tool name or invalid tool arguments" (G). "any errors in _finding_ the tool ... should be reported as an MCP error response." (G) | |
| 25 | IFC-012, `spec/definitions.md` (Conduit's keys), `spec/core/README.md` | A JSON-RPC error's `data` member may hold content that the server defines, so it may hold `io.github.shrav-k.conduit/reason`. | Confirmed. The revision defines the shape of `data` only for the errors -32021 and -32022, not for -32602. The key rules of `_meta` do not apply to `data`, and nothing forbids a key written in that form. | "Error responses **MAY** include a `data` member with additional information of any type, such as nested errors." S`/basic`. "The value of this member is defined by the sender" (G, `Error`). JSON-RPC 2.0: "The value of this member is defined by the Server". <https://www.jsonrpc.org/specification> | |
| 26 | IFC-025 | Only two kinds of request are answered with no tool result: a call that names a tool the engine does not present, and the request of IFC-032 ("IFC-032 names the other request that has none"). | Wrong as worded. The protocol, which IFC-001 says binds the server the engine presents, has a server answer other requests with an error of its own and so with no tool result: one at a revision the server does not support (-32022), one that lacks a required `_meta` key (-32602, and HTTP 400), one whose parameters do not have the form of a `tools/call` request (a protocol error), one that needs a capability the client did not declare (-32021), and on Streamable HTTP one whose headers are missing or disagree with the body (-32020, HTTP 400). | Rows 4 and 5. "Malformed requests (requests that fail to satisfy CallToolRequest schema)" are protocol errors, "returned as standard JSON-RPC errors". S`/server/tools`. "servers **MUST** return HTTP status `400 Bad Request` and **MUST** include a JSON-RPC error response" for a header that fails validation. S`/basic/transports/streamable-http` | See correction A below. |
| 27 | IFC-010 | A result is complete when its `resultType` is `complete` or absent. | Confirmed | "clients **MUST** treat an absent `resultType` as `"complete"`." S`/basic` | |
| 28 | IFC-010 | A `tools/call` request can be answered with a result whose `resultType` is `input_required`, and the engine may leave it there and record unknown. | Confirmed. A client is not obliged to answer or to retry. | "Servers **MAY** respond to `tools/call` with an `InputRequiredResult`". S`/server/tools`. "Servers **MUST NOT** assume that clients will fulfill the `inputRequests` or retry the original request." S`/basic/patterns/mrtr` | |
| 29 | IFC-010 | The protocol's code for invalid parameters is -32602; an unknown tool is one case of it, and arguments the tool does not accept and malformed parameters are others. | Confirmed, with two caveats. The tools page puts "Input validation errors" under tool execution errors, so a plain server may report unaccepted arguments either way; the table of IFC-010 gives an outcome for both. And the table names no outcome for a result whose `resultType` is a value the engine does not recognize, which the revision has a client treat as invalid. | `InvalidParamsError`: "the method parameters are invalid or malformed ... **Tools**: Unknown tool name or invalid tool arguments" (G). "Input validation errors (e.g., date in wrong format, value out of range)" under tool execution errors. S`/server/tools`. "A `resultType` of any value unrecognized by the client **MUST** be considered invalid." S`/basic` | Optional: in IFC-010, write "A result whose `resultType` is `input_required` or is a value the engine does not recognize, any other error, and no result at all, is unknown." |
| 30 | IFC-010, IFC-016 | Where the tasks extension is used, a task has a final result to which the table of IFC-010 applies. | Confirmed, with a caveat. Only a task that ends `completed` has a result, and it is the tool result the call would have returned. A task that ends `failed` has a JSON-RPC error in its place, one that ends `cancelled` has neither, and a `tasks/get` for a task the server has discarded is itself answered with -32602, which the table would read as failed although the work may have had its effect. | "`completed`: ... results are available in the `result` field. This includes tool calls that returned results with `isError: true`. `failed`: The request failed due to a JSON-RPC error during execution." "The `tasks/get` endpoint returns exactly what the underlying request would have returned". "Invalid or nonexistent `taskId`: `-32602`". <https://github.com/modelcontextprotocol/ext-tasks/blob/main/specification/2026-07-28/tasks.md> | Optional: in IFC-010, write "the table applies to what the task ended with: the result of a task that completed, or the error of one that failed. A task that was cancelled, and one that the server no longer holds, is no result at all." |
| 31 | IFC-016, `spec/schema-profile.md` | The protocol has a tasks extension, `io.modelcontextprotocol/tasks`, which is an official extension outside the baseline revision. | Confirmed. Its stage is "Beta, to Core", in which breaking changes are allowed; its identifier does not change with its stage. A server may answer with a task only to a client that declared the extension on that request. | "Move experimental tasks out of the core protocol and into an official extension (`io.modelcontextprotocol/tasks`)." S`/changelog`. "Official extensions use the `io.modelcontextprotocol` vendor prefix." <https://modelcontextprotocol.io/extensions/overview>. "A server **MUST NOT** return `CreateTaskResult` to a client that did not include the extension capability on its request". Tasks specification, URL as row 30. | |
| 32 | IFC-012 | The characters the protocol recommends for a tool name are ASCII letters, digits, `_`, `-` and `.`, and a name made as IFC-012 makes it uses no other. | Confirmed. OBJ-001 and NAM-004 allow lowercase letters, digits, hyphens and, between segments of an export name, dots. | "The following **SHOULD** be the only allowed characters: uppercase and lowercase ASCII letters (A-Z, a-z), digits (0-9), underscore (\_), hyphen (-), and dot (.)". S`/server/tools` | |
| 33 | IFC-012, IFC-014, CON-026, `spec/definitions.md` (tool name) | The protocol recommends, and does not require, that a tool name be at most 128 characters. | Confirmed | "Tool names **SHOULD** be between 1 and 128 characters in length (inclusive)." S`/server/tools` | |
| 34 | IFC-001, IFC-014 (relied on) | One server has one tool of a given name. | Confirmed | "Tool names **SHOULD** be unique within a server." S`/server/tools` | |
| 35 | IFC-012, IFC-013, IFC-022, IFC-032 (relied on) | A server may present a different tool list to each consumer, established from what each request presents; the list may be empty and may change over time. | Confirmed, on the condition IFC-022 sets: the list follows what the request presents and not the connection. | "This set **MAY** be empty and **MAY** change over time ..., but **MUST NOT** vary per-connection or as a side effect of other requests on the connection. The set **MAY** vary by the authorization presented on the request ... since credentials are per-request input, not connection state." S`/server/tools` | |
| 36 | IFC-012, IFC-013 (relied on) | A consumer's tool list may change because of a request that the same consumer made: such as its own `open.v1` or `close.v1` for a Conduit of which it is the consumer. | Cannot be confirmed. The sentence of row 35 forbids a list that varies "as a side effect of other requests on the connection". The proposal that wrote the sentence, SEP-2567 (status Final), explains that "servers can no longer mutate list results as a side effect of other requests" and gives a tool whose call makes other tools appear as the pattern that "is no longer permitted". It also says the constraint is only that there is no "per-session or per-connection scope" for a list to depend on, and that a list may change when "a user's plan or granted scopes change". The engine's list depends on stored state that is the same on every connection, and an open Conduit is a grant, so the second reading fits it; the first, taken literally, does not. The text of the revision does not choose. The earlier check did not weigh this clause. | As row 35. <https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/seps/2567-sessionless-mcp.md>, section "Session-independent list endpoints" | See correction B below. |
| 37 | IFC-013 | A consumer may hold a tool list for as long as the protocol lets it. | Confirmed. The engine sets the time with `ttlMs`. The revision also tells a client that an invalid-parameters error on a call is a reason to fetch the list again, which fits the answer of IFC-012. | "If `ttlMs` is positive, the client **SHOULD** consider the result fresh for that many milliseconds"; "Clients **MAY** re-fetch before the TTL expires if they have reason to believe the data has changed (e.g., receiving an unexpected error on a tool call indicating the method was not found or the parameters were invalid)." S`/server/utilities/caching` | |
| 38 | IFC-024, IFC-035, IFC-012 (relied on) | A server may always present two tools of its own, named `retrieve-result.v1` and `start-later-work.v1`, to every client it knows as an instance, and none to a client it does not. | Confirmed. The names use only recommended characters, and row 35 lets the list differ by what the request presents and be empty. | As rows 32 and 35 | |
| 39 | IFC-024, IFC-035 (relied on) | A tool result may carry the state or the outcome of an invocation other than the call itself, and a call may name that invocation by an identifier. | Confirmed. The protocol gives no meaning to what `structuredContent` or a vendor key of `_meta` holds. Both tools also keep the one rule the revision has on state that spans requests: each call names the invocation outright (`operationIdentity` and `target`, or `invocation`). The tasks extension is the protocol's own way to fetch a result later; it is optional and IFC-016 allows it beside this one. | "State that needs to span multiple requests (e.g., long-running tasks, application-level handles) **MUST** be referenced by an explicit identifier the client passes on each request." S`/basic` | |
| 40 | IFC-015, IFC-035 (relied on) | An implementation, which is a server to the engine, reaches the engine by being a client of a server the engine presents; it has no other way to send the engine a request. | Confirmed. A server cannot send a request, so a start of later work has to be a call by a client. | "servers do not initiate JSON-RPC requests and clients do not send JSON-RPC responses." S`/basic/transports` | |
| 41 | IFC-033 | A tool definition has `inputSchema`, which describes a map, and `outputSchema`; replacing a `$ref` to a Schema contract by the schema agrees with the protocol. | Confirmed | `inputSchema: { $schema?: string; type: "object"; [key: string]: unknown }`; `outputSchema?: { $schema?: string; [key: string]: unknown }` (G). "Implementations **MUST NOT** automatically dereference `$ref` values that resolve to a network URI." S`/basic` | |
| 42 | SCH-014, `spec/schema-profile.md` | The schema of an existing server follows the rules of its own dialect. | Confirmed | "When a schema does not include a `$schema` field, it defaults to JSON Schema 2020-12"; "Schemas MAY include a `$schema` field to specify a different dialect". S`/basic` | |
| 43 | Header of `spec/conventions/implicit.yaml` | The protocol's tool annotations declare that a tool is read-only or destructive. | Confirmed. They are hints, and a client is told not to trust them from an untrusted server. | `ToolAnnotations { readOnlyHint?: boolean; destructiveHint?: boolean; ... }`, "all properties in `ToolAnnotations` are **hints**." (G) | |
| 44 | Header, IFC-011, header of `spec/promises/templates.yaml` | The protocol gives guidance for stateful tools: a handle is an opaque string, holding it permits nothing, the server checks it on every call, and an unknown handle gives a tool error. | Confirmed. The guidance is marked non-normative. | "a handle is a name, not a capability. The server should validate the caller's authorization against the handle on every call."; "A call against an expired or unknown handle should return a tool execution error that says so". S`/server/tools`, Stateful Tools | |
| 45 | IFC-017, IFC-001 (`command` or `url`), DL12 | The protocol has transports to choose among. | Confirmed: stdio and Streamable HTTP, and custom transports are permitted. On stdio the client starts the server's process, which for the server the engine presents to an implementation means the implementation's side starts it. | The standard transports are stdio, "newline-delimited messages over the standard streams of a client-launched subprocess", and Streamable HTTP; "Clients and servers **MAY** implement additional custom transport mechanisms". S`/basic/transports` | |
| 46 | IFC-022 | A server establishes who a request comes from for that request alone, and not from the connection or an earlier request on it. | Confirmed. The protocol's own `clientInfo` cannot serve, because it is not verified. | "Servers **MUST NOT** rely on prior requests over the same connection to establish context (e.g., capabilities, protocol version, client identity)."; `clientInfo` is "self-reported by the sender and ... not verified by the protocol". S`/basic` | |
| 47 | IFC-032 | Where the protocol's own authorization is used on an HTTP transport, a request whose credential is missing or invalid is answered with HTTP 401 and no tool result. | Confirmed, with a caveat. The protocol also names HTTP 403 for a valid token with "insufficient permissions". IFC-032 answers a valid credential from which no instance is established with -32602 and does not say that 403 never applies. Authorization is optional in the protocol and is defined for HTTP transports only. | "Invalid or expired tokens **MUST** receive a HTTP 401 response."; "Servers **MUST** return appropriate HTTP status codes for authorization errors", in a table that gives 401 for "Authorization required or token invalid" and 403 for "Invalid scopes or insufficient permissions". "Authorization is **OPTIONAL** for MCP implementations." S`/basic/authorization` | Optional: in IFC-032, write "is answered as the protocol requires, with the HTTP status it prescribes, which is 401 for a credential that is missing or invalid, and with no tool result". |
| 48 | IFC-030 (relied on) | Not passing a consumer's credential on to an implementation agrees with the protocol. | Confirmed | "MCP servers **MUST NOT** accept or transit any other tokens." S`/basic/authorization` | |

Count by verdict: confirmed 44 (of which rows 18, 23, 29, 30 and 47 carry a caveat); wrong 1
(row 26); cannot be confirmed 3 (rows 21, 22 and 36). Nothing in the file is true only in the
draft: the draft schema and the draft tools page do not differ from the release in substance.

## What was asked in particular

The request for this check named points to look at. Each is a row above; this list says where.

- The answer to a call to a tool the engine does not present: rows 24 and 25, confirmed.
- `isError` true on results that carry no outputs: row 18, confirmed with a caveat. The reading
  that such a result need not conform to `outputSchema`: rows 20 and 21, the silence confirmed
  and the reading not.
- HTTP 401 for an invalid token: row 47, confirmed with a caveat.
- The 128 characters and the allowed characters: rows 32 and 33, confirmed.
- `_meta` on a tool's definition and what a server may put there: rows 13 and 15, confirmed.
- `content` always present: row 23, confirmed with a caveat.
- The tasks extension: rows 30 and 31, confirmed, the first with a caveat.
- Both sides keep every requirement of the revision: rows 5 to 8 confirm that the four
  requirements named are requirements. Rows 21, 22, 26 and 36 are the places where the file's own
  text does not yet sit cleanly with that rule.
- How the revision is negotiated for each request: rows 2 to 4, confirmed.
- Whether `start-later-work.v1` and `retrieve-result.v1` conflict with the protocol: rows 38 to
  40. They do not.
- Whether an error's `data` may carry content the server defines: row 25, confirmed.

## Suggested corrections

Each is the least change of wording that makes the text true. Corrections A and B and the one
for row 22 touch what an engine does or state a reading of the protocol, so each is for the
coordinator or the owner to rule on. None of them has been applied.

### A. Row 26: the requests that have no tool result (IFC-025)

In the observable of IFC-025, replace "IFC-032 names the other request that has none." with:

> IFC-032 names another. A request that the protocol has a server reject with an error of its
> own has none either, and is answered as the protocol states: one at a revision the engine does
> not support, one that lacks a key the protocol requires in its `_meta`, one whose parameters do
> not have the form of a `tools/call` request, and one whose headers on an HTTP transport are
> missing or disagree with its body.

This states what the protocol already requires and adds no behavior. It leaves one thing open
that the file does not say and that this check cannot decide: whether such a request is a request
in the sense of the conventions, with the reason invalid under OPS-023 and a record under
REC-001, and whether the error's `data` then holds `io.github.shrav-k.conduit/reason` with the
value invalid, as IFC-012 has it hold refused. Carrying the reason in `data` would agree with the
protocol (row 25). It is a choice of behavior, so it is not proposed here.

### B. Row 36: a tool list that changes after the consumer's own request (IFC-012)

Add to the observable of IFC-012, after "and nothing else.":

> The list that the engine serves for a request depends only on the instance established for
> that request under IFC-022 and on which of that instance's Conduits are open when it is served.
> It depends neither on the connection nor on what was sent on it before: once a Conduit is
> opened or closed, every request of that instance is served the same list, on whatever
> connection it arrives. This specification takes the opening or the closing of a Conduit to be
> a change over time in what the instance is permitted, which the protocol allows of a tool
> list, and not a side effect of a request on a connection, which it forbids. That holds also
> where the consumer itself sent the request that opened or closed the Conduit.

This does for the tool list what IFC-025 already does for `isError` and `outputSchema`: it says
which reading the specification takes where the protocol's text allows two. The other way is the
one SEP-2567 describes for a server that wants no doubt: present every tool at all times and
refuse the call. That would undo IFC-012 and IFC-013 and is not proposed.

### Row 22: the result of an operation that declares no outputs (IFC-025 or IFC-033)

The correction is in the table: say in IFC-025 that `structuredContent` is present on every
result that is not an error and is an empty map where the operation declares no outputs, or say
in IFC-033 that the tool of such an operation has no `outputSchema`.

### Row 21: no change

The text is already true as a statement of the specification's reading. Two things would settle
it in the protocol: issue 3003 and pull request 2145. Both were open on 2026-11-06.

### Optional changes for the rows that carry a caveat

- Row 18, IFC-025: say that `isError` on a result the engine gives a consumer means only that the
  result carries no outputs.
- Row 23, IFC-025: say that the protocol recommends a serialized copy of `structuredContent` in
  `content`, and that whether the engine gives one is DL12.
- Row 29, IFC-010: name a `resultType` that the engine does not recognize among the cases that
  are unknown.
- Row 30, IFC-010: say what the table applies to for a task that failed, was cancelled or is no
  longer held.
- Row 47, IFC-032: write "the HTTP status it prescribes, which is 401 for a credential that is
  missing or invalid".

## What to watch in the protocol

These are not claims of the file. Each could change a verdict above without any change to
`spec/`.

- **Pull request 2145 (SEP-2145), "Standardize `tools/call` failure reporting".** Open, labelled
  `in-review`, last updated 2026-11-02. It would report an unknown tool, and a missing or
  non-conforming `structuredContent`, as a tool result with `isError` true. If it is accepted
  into a later revision, the answer IFC-012 gives is the old rule: correct at the baseline
  2026-07-28, and to be looked at again when the baseline moves under IFC-005. Its description
  says the Python and TypeScript SDKs already behave as it proposes, which bears on IFC-010: a
  plain server built on one may answer an unknown tool with `isError` true. The table of IFC-010
  gives a safe outcome for that.
- **Issue 3003.** Open, last updated 2026-10-31. It asks what `structuredContent` and
  `outputSchema` mean for an error result, which is row 21.
- **The tasks extension.** Its stage is "Beta, to Core", in which breaking changes are allowed
  and are recorded in a changelog. Its specification was last changed on 2026-10-09.
- **A protocol error may not reach a model.** "Clients **MAY** provide protocol errors to language
  models, though these are less likely to result in successful recovery." An Agent whose client
  does not pass the error on does not learn that its call was refused. This follows from the
  decision on row 19 of the earlier check and is not a conflict with the protocol.
