# Check of plain-server errors -32602 and -32020

Not binding. Checked 2026-11-09 for [work order 018](../../work-orders/018-plain-errors-check.md),
under [increment 2, decision 8](increment-2/decisions.md#the-decisions). This report proposes
corrections; it accepts none. The [specification](../../../spec/README.md) still governs.

## Answers

Neither error code alone proves that an effect has not happened. IFC-010's unconditional
reading of -32602 as failed is unsupported. IFC-046's statement that MCP guarantees rejection
before acting is also unsupported by the text checked. The normal SDK validation paths reject
before calling a tool, but SDKs also let a tool handler return these codes after it has acted.
Those are separate paths, as the [protocol text](#protocol-text) and
[tagged SDK source](#what-the-sdks-send) below show. An SDK permitting such an answer does not
make every use of the code conform to MCP's intended error categories.

For the filesystem server, neither requested version has a source path that answers either
code after a write by that call. That narrower result does not establish a guarantee for other
plain servers; see [the versioned trace](#the-filesystem-server).

## Basis and limits

The relevant Conduit text is IFC-001 to IFC-004, IFC-010 and IFC-046 in
[implementation.yaml](../../../spec/interface/implementation.yaml), and DL12 in
[implementation-defined.yaml](../../../spec/implementation-defined.yaml). The baseline is
2026-07-28. Additional revisions are an engine's documented choice, subject to the features
IFC-004 requires; it explicitly mentions 2025-11-25 and earlier revisions for plain calls.
The table below checks 2025-06-18 and 2025-11-25 as well as the baseline. For completeness it
also checks 2024-11-05 and 2025-03-26, without claiming that their smaller feature sets suffice
for every Conduit call.

The SDK evidence is a static trace of released source, not an executed fault-injection test.
All external sources are MCP's specification or source in its official SDK and reference-server
repositories. Files were read from the following tags, not from moving `main` branches:

| Repository | Tag | Commit |
| --- | --- | --- |
| [TypeScript SDK][ts-tag] | `v2.4.0` | `951fe5603a7ec0b2dcb6a0b95d92cd86f93ad9e5` |
| [Python SDK][py-tag] | `v2.3.0` | `2118f14f8a19bc158d8a1cf90af58d85d187f849` |
| [Rust SDK, rmcp][rs-tag] | `rmcp-v3.9.0` | `5596a8a0bba2a1ec5519cc855cdadf23e4dcecdc` |
| [Reference servers][fs21-package] | `v2.2.0` | `194cc86e3ae6400958555bd4116849d822d69263` |
| [Reference servers][fs22-package] | `v2.3.0` | `36380287ed3b15d115b55f546617d723072b1296` |

## Protocol text

### -32602: invalid parameters

Each revision's Tools, **Error Handling** section gives this exact excerpt from its protocol
error example:

```json
"code": -32602,
"message": "Unknown tool: invalid_tool_name"
```

This is an excerpt, not a complete JSON document. The revision-specific citations and the
surrounding categories are:

| Revision and section | When the text uses a protocol error | Does it state that no effect has happened? |
| --- | --- | --- |
| [2024-11-05, Tools / Error Handling][tools24] | Lists “Unknown tools”, “Invalid arguments” and “Server errors”; gives the quoted -32602 unknown-tool example. Tool execution failures use a result with `isError: true`. | No statement about effects, rollback or the time at which -32602 is produced. |
| [2025-03-26, Tools / Error Handling][tools03] | Lists “Unknown tools”, “Invalid arguments” and “Server errors”; gives the quoted example. | No such statement. |
| [2025-06-18, Tools / Error Handling][tools06] | Lists “Unknown tools”, “Invalid arguments” and “Server errors”; gives the quoted example. | No such statement. |
| [2025-11-25, Tools / Error Handling][tools11] | Lists “Unknown tools”, “Malformed requests” (with a schema qualification) and “Server errors”; gives the quoted example. Input validation failures within tool execution belong to `isError: true` results. | No such statement. |
| [2026-07-28, Tools / Error Handling][tools26] | Lists “Unknown tool”, “Malformed requests” (with a schema qualification) and “Server errors”; gives the quoted example. Distinguishes tool execution failures, including input validation, returned with `isError: true`. | No such statement. |

The baseline's schema, **InvalidParamsError**, describes it exactly as:

> A JSON-RPC error indicating that the method parameters are invalid or malformed.

Its tool-specific entry is:

> **Tools**: Unknown tool name or invalid tool arguments

The same schema defines `INVALID_PARAMS = -32602`; neither this description nor its list of
contexts supplies an execution-order or no-effect condition.
([2026-07-28 schema.ts, lines 311–316 and 361–391][params26])

There are required rejection cases too. Basic / `_meta` says exactly:

> A request missing any required field is malformed; the server **MUST** reject it with
> JSON-RPC error code `-32602` (Invalid params).

The next sentence requires HTTP status 400. That describes how to reject malformed input; it does not
state the converse that every answer carrying -32602 certifies that nothing was performed.
([2026-07-28, Basic / `_meta`][meta26])

**Reading of IFC-010:** the text supports an unknown-tool rejection and input validation before
execution as ordinary reasons for -32602. It does not support treating the code as a universal
certificate that an effect did not happen. That distinction matters because Conduit's failed
outcome means the effect happened neither wholly nor partly, under OPS-003, whereas IFC-010
currently makes every -32602 failed.
([Conduit operations](../../../spec/conventions/operations.yaml), IFC-010;
[SDK counterpaths below](#what-the-sdks-send))

### -32020: header mismatch

| Revision | What it defines for -32020 |
| --- | --- |
| 2024-11-05 | No MCP definition in [Tools / Error Handling][tools24] or [Transports][transport24]; there is no statement to quote about -32020 or its execution order. |
| 2025-03-26 | No MCP definition in [Tools / Error Handling][tools03] or [Transports][transport03]. |
| 2025-06-18 | No `HeaderMismatch` or -32020 definition in its [schema][schema06], [Tools][tools06] or [Transports][transport06]. |
| 2025-11-25 | No `HeaderMismatch` or -32020 definition in its [schema][schema11], [Tools][tools11] or [Transports][transport11]. |
| 2026-07-28 | Defined in Streamable HTTP, **Server Validation**, and in the schema as `HEADER_MISMATCH = -32020`. ([Transport][headers26], [schema.ts, lines 427–434 and 452–470][mismatch26]) |

The baseline transport's **Server Validation** table gives this exact description:

> The HTTP headers do not match the corresponding values in the request body, or required headers are missing/malformed.

That section requires rejection with HTTP 400 and a JSON-RPC -32020 error. It covers missing
standard headers, mismatched values (after decoding where required), and invalid characters.
**Custom Headers from Tool Parameters / Server Behavior for Custom Headers** also requires rejection when a recognized
`Mcp-Param-*` header fails validation. **Client Behavior** says that when missing or mismatched
`Mcp-Param-*` headers cause this error, the client SHOULD read `tools/list` for schema changes
and retry with appropriate headers. It does not prescribe a retry for every possible -32020,
nor does it set Conduit's one-retry bound.
([2026-07-28, Streamable HTTP, these sections][headers26])

Neither those sections nor the schema's **HeaderMismatchError** definition explicitly says
that the server has performed nothing, that validation precedes all effects, or that effects
are rolled back. Rejecting during validation before dispatch is a sound implementation of
the rule, and the inspected SDKs do that on their built-in paths. Reading *every received code*
as proof of that path is an additional inference. A response with HTTP 400 does not settle its
origin either: Python and rmcp can give a handler-produced -32020 that status.
([Transport][headers26], [schema][mismatch26], [Python trace](#python-v230),
[rmcp trace](#rust-rmcp-v390))

**Reading of IFC-046:** the recommendation to refresh and retry is present, as a SHOULD, for the
specific `Mcp-Param-*` case. The claimed protocol guarantee of rejection before acting is not
present in the text checked. The SDK traces show why code recognition, even with an HTTP-status
check, cannot establish that guarantee. This report checks the -32020 arm; it does not settle
IFC-046's separate unsupported-revision arm.
([Transport][headers26], [IFC-046](../../../spec/interface/implementation.yaml))

## What the SDKs send

The following are possible source paths, not reports of observed misbehaving tools. In each
counterpath, assume that a handler first performs a write and then returns the indicated error.
The cited dispatch code has no knowledge of that write and performs no rollback.

### TypeScript v2.4.0

- **Built-in -32602:** `packages/server/src/server/mcp.ts`, lines 296–303, throws
  `ProtocolError(ProtocolErrorCode.InvalidParams, ...)` for an absent or disabled tool before
  invoking a handler. Input and output validators also construct InvalidParams errors, but
  the surrounding catch converts those into a tool error result; they must not be counted
  as unconditional wire -32602 sources. ([mcp.ts, lines 296–418][ts-mcp])
- **Built-in -32020:** `packages/server/src/server/createMcpHandler.ts`, lines 817–840,
  checks annotated arguments against `Mcp-Param-*` headers and returns the header rejection
  before dispatch. This particular schema-based check runs for `McpServer`, whose registry
  supplies the tool schema; the source explicitly skips it for a low-level `Server` factory.
  Standard header validation is also done at entry, at lines 756–763.
  ([createMcpHandler.ts][ts-http])
- **Handler after an effect:** the low-level `Server.setRequestHandler` path lets a handler
  throw a `ProtocolError` with either code. `packages/core-internal/src/shared/protocol.ts`,
  lines 1147–1167, serializes a handler's integer code, message and data into a JSON-RPC error.
  The baseline codec preserves both codes. Thus a low-level `tools/call` handler can act and
  then produce either error. ([protocol.ts][ts-dispatch], [codec.ts, line 275][ts-codec])
- **High-level exception and HTTP distinctions:** `McpServer` catches handler exceptions and
  returns `isError: true`; only `UrlElicitationRequired` is rethrown there. Throwing either
  studied code from an ordinary registered tool consequently does **not** produce that
  JSON-RPC code through this wrapper. The HTTP per-request transport also distinguishes
  validation-origin errors from handler-origin errors: handler-produced -32020 stays in an
  HTTP 200 response (or an already-open SSE stream), whereas validation rejection uses 400.
  Neither distinction is a universal property of MCP servers.
  ([mcp.ts, lines 305–342][ts-mcp], [perRequestTransport.ts, lines 274–320][ts-status])

### Python v2.3.0

- **Built-in codes:** `src/mcp/shared/inbound.py`, lines 429–484, constructs -32602 for a
  missing/malformed request envelope and -32020 for header/body mismatch. Its
  `validate_mcp_param_headers`, lines 530–595, constructs -32020 for missing, unexpected,
  duplicated or mismatched annotated headers. The modern HTTP entry returns those
  rejections before `serve_one` dispatches the request.
  ([inbound.py][py-inbound], [_streamable_http_modern.py, lines 443–472][py-http])
- **Handler after an effect:** `src/mcp/shared/jsonrpc_dispatcher.py`, lines 92–106, preserves
  an `MCPError`'s `ErrorData`; its exception boundary sends that error at lines 772–783.
  `MCPError(code=-32602, message=...)` and `MCPError(code=-32020, message=...)` raised after
  a write therefore survive the low-level handler path. A handler-raised Pydantic
  `ValidationError` can also become -32602 at this boundary, so the explicit protocol
  exception is not the only possible late source. ([jsonrpc_dispatcher.py][py-dispatch])
- **The higher-level wrapper preserves protocol exceptions too:**
  `src/mcp/server/mcpserver/tools/base.py`, lines 179–201, invokes the function and rethrows
  `MCPError`. `mcpserver/server.py`, lines 437–445, rethrows it again rather than making an
  `isError` tool result. This differs from TypeScript's wrapper.
  ([tools/base.py][py-tool], [mcpserver/server.py][py-mcp])
- **HTTP does not prove early rejection:** `runner.py`'s `modern_error_data`, lines 534–548,
  uses the same exception mapping. `_streamable_http_modern.py`, lines 139–143 and 184–199,
  turns the exception into a JSON-RPC reply and maps its code to an HTTP status.
  `shared/inbound.py`, lines 309–317, maps both studied codes to 400; a handler-produced
  error can therefore have the same status as validation rejection.
  ([runner.py][py-runner], [modern HTTP entry][py-http], [inbound.py][py-inbound])

### Rust rmcp v3.9.0

- **Built-in -32602:** `crates/rmcp/src/handler/server/router/tool.rs`, lines 560–575,
  returns `ErrorData::invalid_params` for a missing or disabled tool before calling it.
  Argument-deserialization errors with the recognized prefix are instead converted to an
  `is_error` tool result by lines 144–155. ([router/tool.rs][rs-router])
- **Built-in -32020:** `crates/rmcp/src/transport/streamable_http_server/tower.rs`,
  lines 1043–1058, validates headers against the body and tool schema and returns a header
  mismatch rejection. `header_mismatch_jsonrpc_response`, lines 968–983, constructs
  `ErrorData::header_mismatch` and HTTP 400. ([tower.rs][rs-http])
- **Handler after an effect:** `ErrorData::invalid_params` and `ErrorData::header_mismatch`
  construct the studied codes. Returning either after a write is possible through
  `Result<CallToolResult, ErrorData>`: `IntoCallToolResult` preserves `Err(ErrorData)`, the
  router preserves errors other than its special argument-deserialization case, and
  `service.rs`, lines 1615–1627, serializes the error as a JSON-RPC error. No special
  message prefix is needed for this counterpath.
  ([model.rs, lines 683–690 and 724–725][rs-errors],
  [handler/server/tool.rs, lines 119–143][rs-convert], [router/tool.rs][rs-router],
  [service.rs][rs-dispatch])
- **HTTP 400 is insufficient here too:** `tower.rs`, lines 796–803, maps -32020 to 400 by
  code. Its regression test at lines 818–826 explicitly covers a handler returning
  `ErrorData::header_mismatch`. ([tower.rs][rs-http])

These counterpaths refute a claim about what all SDK-built servers can answer. They do not
demonstrate that a particular server actually wrote and then sent either error, or that such
use always meets MCP's error-category rules. The findings require neither claim.

## The filesystem server

The work order's two versions are those A70 ran in experiment 007: 2.1.0 at T1, upgraded to
2.2.0 at T5. Experiment 008 uses a Rust stub, with simulated writes, rather than these
filesystem versions. This report interprets the order as asking for the actual reference
filesystem packages used by A70 as well as the protocol errors that 008 simulated.
([A70 results](../../../experiments/007-s1-baseline/results-2026-11-09.md),
[008's stub](../../../experiments/008-real-client/README.md#the-stub),
[008's limits](../../../experiments/008-real-client/results-2026-11-09.md))

The repository tags and package versions are **different**:

| Package | Repository tag and identifying source | SDK in that tag's lockfile |
| --- | --- | --- |
| `@modelcontextprotocol/server-filesystem` 2.1.0 | `v2.2.0`, [src/filesystem/package.json, lines 1–3][fs21-package] | `@modelcontextprotocol/server` 2.4.0, [package-lock.json, lines 852–859][fs21-lock] |
| `@modelcontextprotocol/server-filesystem` 2.2.0 | `v2.3.0`, [src/filesystem/package.json, lines 1–3][fs22-package] | `@modelcontextprotocol/server` 2.4.0, [package-lock.json, lines 858–865][fs22-lock] |

Both use `McpServer.registerTool`, with the TypeScript wrapper traced above. In 2.1.0,
`src/filesystem/server.ts`, lines 451–458, validates a path, writes, then returns content;
lines 479–485 edit, and lines 731–746 move. Its outer registration wrapper waits for roots
and invokes the handler at lines 228–252. In 2.2.0, the corresponding registration wrapper
is at lines 280–301, `write_file` at 545–552, and the new `append_file` and `copy_file`
handlers at 574–581 and 945–959. None of these paths throws either protocol code after
writing. The directory-creation handlers follow the same register/validate/mutate/return
pattern. ([2.1.0 server.ts][fs21-server], [2.2.0 server.ts][fs22-server])

**Answer for both versions:** no such post-write answer path was found. An unknown or removed
tool can get -32602 before its handler runs. An error thrown during or after a mutation,
including a partially completed mutation, goes through the SDK wrapper's `isError: true`
result path, not either studied JSON-RPC code. Even its output validator's internal
InvalidParams exception is caught there. The S1 entry point serves stdio, so the HTTP-only
header validation that normally generates -32020 is absent. Version 2.2.0 also has explicit
-32602 errors in `file-resources.ts`, lines 182–193 and 218–223, but these are for reading
resources, not a write-tool reply.
([TypeScript wrapper][ts-mcp], [2.1.0 entry point][fs21-entry],
[2.2.0 entry point][fs22-entry], [2.2.0 file-resources.ts][fs22-resources])

This is a result about the tagged source and its locked SDK, not a byte-for-byte attestation of
A70's installed npm cache. Both package manifests declare `@modelcontextprotocol/server` as
`^2.3.1`, so the package version alone does not fix every future dependency resolution.
It also does not prove that a filesystem error means no write occurred; an `isError` result
can follow a partial effect. Neither limitation restores a general no-effect interpretation
of the two codes. ([Package manifests][fs21-package], [2.2.0 manifest][fs22-package],
[SDK wrapper][ts-mcp])

## Options for the admin

These are recommendations for a later decision and specification/vector work. Nothing below
changes the current table or authorizes the shell to depart from it.

### IFC-010: what does a plain -32602 establish?

| Option | Consequence |
| --- | --- |
| A. Keep the unconditional failed classification | Relies on an unproved convention that every plain server rejects before an effect. The Python, rmcp and low-level TypeScript counterpaths remain possible. |
| B. Treat -32602 as any other plain-server error | Failed only for IFC-033's existing safe category: no declared `effect`, not a control, and not `startsModelWork: true`; unknown otherwise. A plain effect with an unknown outcome is not dispatched again under OPS-012. |
| C. Permit a narrower no-effect classification only with an explicit implementation guarantee | Requires a defined, verifiable contract or adaptation that distinguishes rejection before any effect; neither the number nor the human-readable error message supplies this. Adds a new qualification surface. |

**Recommend B.** It applies the same conservative treatment already chosen for -32021 and
needs no new server classification. A rejection that the engine itself makes before it sends
a call is not a plain result at all: it keeps the reason that applies and has no outcome, under
OPS-005; do not equate that local fact with a code returned by a plain server.
The specification's existing meaning of failed and its rule for unknown plain effects make
this the smallest correction supported by the evidence.
([IFC-010 and IFC-045](../../../spec/interface/implementation.yaml),
[OPS-003, OPS-004 and OPS-012](../../../spec/conventions/operations.yaml),
[SDK traces](#what-the-sdks-send))

### IFC-046: when may -32020 cause a second send?

| Option | Consequence |
| --- | --- |
| A. Keep unconditional refresh and resend based on -32020 | Follows MCP's recovery recommendation for its intended validation path, but can repeat an effect on the handler counterpaths. Adding an HTTP 400 test alone does not repair it. |
| B. Restrict resending to calls known to have no effect, and to implementations with an explicit no-effect rejection guarantee | For plain operations outside the existing safe category, do not resend merely because of -32020; read the error as unknown. For implementations carrying Conduit's keys, add a guarantee, analogous to IFC-047, that this rejection is issued only after performing nothing. Retain one refresh/resend within the guaranteed case. |
| C. Remove automatic -32020 recovery entirely | Treat it as an ordinary error even when a trusted implementation explicitly guarantees rejection before acting. Simplest mechanics, but loses safe automatic recovery. |

**Recommend B.** Follow the protocol's SHOULD recovery where it is safe, and make Conduit's
stronger requirement rest on a stated implementation obligation. The no-effect category here
must be exactly IFC-033's test, not `readOnlyHint` from an unqualified server, `idempotentHint`,
HTTP status, or wording in `message`. An effectful plain implementation with only this error
code available must leave an unknown outcome and receive no implicit second call for that
invocation. Re-reading `tools/list` may repair headers for a future permitted invocation; it
cannot undo an effect of the original call.
([MCP Client Behavior][headers26], [SDK traces](#what-the-sdks-send),
[IFC-033 and IFC-047](../../../spec/interface/implementation.yaml),
[OPS-012 and OPS-018](../../../spec/conventions/operations.yaml))

If adopted, a follow-up order should change IFC-010 and the -32020 arm of IFC-046, reconcile
IFC-001's cross-reference and OPS-007's treatment of implementation errors, and add vectors
before increment 3 implements resending. The vectors should cover each code after a partial
effect from a plain implementation (unknown, no second dispatch), both controls and model work
without a declared `effect`, ordinary reads, and a guaranteed rejection followed by one resend
for an implementation carrying Conduit's keys. Include a second rejection to test the bound.
Decision 8 leaves the current core table in force pending that decision.
([Increment 2 decisions](increment-2/decisions.md#the-decisions),
[interface](../../../spec/interface/implementation.yaml),
[operations](../../../spec/conventions/operations.yaml))

[tools24]: https://modelcontextprotocol.io/specification/2024-11-05/server/tools#error-handling
[tools03]: https://modelcontextprotocol.io/specification/2025-03-26/server/tools#error-handling
[tools06]: https://modelcontextprotocol.io/specification/2025-06-18/server/tools#error-handling
[tools11]: https://modelcontextprotocol.io/specification/2025-11-25/server/tools#error-handling
[tools26]: https://modelcontextprotocol.io/specification/2026-07-28/server/tools#error-handling
[transport24]: https://modelcontextprotocol.io/specification/2024-11-05/basic/transports
[transport03]: https://modelcontextprotocol.io/specification/2025-03-26/basic/transports
[transport06]: https://modelcontextprotocol.io/specification/2025-06-18/basic/transports
[transport11]: https://modelcontextprotocol.io/specification/2025-11-25/basic/transports
[schema06]: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2025-06-18/schema/2025-06-18/schema.ts
[schema11]: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2025-11-25/schema/2025-11-25/schema.ts
[params26]: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/schema/2026-07-28/schema.ts#L311-L391
[meta26]: https://modelcontextprotocol.io/specification/2026-07-28/basic#meta
[headers26]: https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http#server-validation
[mismatch26]: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/schema/2026-07-28/schema.ts#L427-L470
[ts-tag]: https://github.com/modelcontextprotocol/typescript-sdk/tree/v2.4.0
[py-tag]: https://github.com/modelcontextprotocol/python-sdk/tree/v2.3.0
[rs-tag]: https://github.com/modelcontextprotocol/rust-sdk/tree/rmcp-v3.9.0
[ts-mcp]: https://github.com/modelcontextprotocol/typescript-sdk/blob/v2.4.0/packages/server/src/server/mcp.ts#L296-L418
[ts-http]: https://github.com/modelcontextprotocol/typescript-sdk/blob/v2.4.0/packages/server/src/server/createMcpHandler.ts#L756-L840
[ts-dispatch]: https://github.com/modelcontextprotocol/typescript-sdk/blob/v2.4.0/packages/core-internal/src/shared/protocol.ts#L1147-L1167
[ts-codec]: https://github.com/modelcontextprotocol/typescript-sdk/blob/v2.4.0/packages/core-internal/src/wire/rev2026-07-28/codec.ts#L275
[ts-status]: https://github.com/modelcontextprotocol/typescript-sdk/blob/v2.4.0/packages/server/src/server/perRequestTransport.ts#L274-L320
[py-inbound]: https://github.com/modelcontextprotocol/python-sdk/blob/v2.3.0/src/mcp/shared/inbound.py#L309-L595
[py-dispatch]: https://github.com/modelcontextprotocol/python-sdk/blob/v2.3.0/src/mcp/shared/jsonrpc_dispatcher.py#L92-L106
[py-tool]: https://github.com/modelcontextprotocol/python-sdk/blob/v2.3.0/src/mcp/server/mcpserver/tools/base.py#L179-L201
[py-mcp]: https://github.com/modelcontextprotocol/python-sdk/blob/v2.3.0/src/mcp/server/mcpserver/server.py#L437-L445
[py-runner]: https://github.com/modelcontextprotocol/python-sdk/blob/v2.3.0/src/mcp/server/runner.py#L534-L548
[py-http]: https://github.com/modelcontextprotocol/python-sdk/blob/v2.3.0/src/mcp/server/_streamable_http_modern.py#L132-L515
[rs-router]: https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.9.0/crates/rmcp/src/handler/server/router/tool.rs#L144-L575
[rs-http]: https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.9.0/crates/rmcp/src/transport/streamable_http_server/tower.rs#L790-L1058
[rs-errors]: https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.9.0/crates/rmcp/src/model.rs#L683-L725
[rs-convert]: https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.9.0/crates/rmcp/src/handler/server/tool.rs#L119-L143
[rs-dispatch]: https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.9.0/crates/rmcp/src/service.rs#L1615-L1627
[fs21-package]: https://github.com/modelcontextprotocol/servers/blob/v2.2.0/src/filesystem/package.json#L1-L3
[fs22-package]: https://github.com/modelcontextprotocol/servers/blob/v2.3.0/src/filesystem/package.json#L1-L3
[fs21-lock]: https://github.com/modelcontextprotocol/servers/blob/v2.2.0/package-lock.json#L852-L859
[fs22-lock]: https://github.com/modelcontextprotocol/servers/blob/v2.3.0/package-lock.json#L858-L865
[fs21-server]: https://github.com/modelcontextprotocol/servers/blob/v2.2.0/src/filesystem/server.ts#L228-L748
[fs22-server]: https://github.com/modelcontextprotocol/servers/blob/v2.3.0/src/filesystem/server.ts#L280-L961
[fs21-entry]: https://github.com/modelcontextprotocol/servers/blob/v2.2.0/src/filesystem/index.ts#L1-L158
[fs22-entry]: https://github.com/modelcontextprotocol/servers/blob/v2.3.0/src/filesystem/index.ts#L1-L207
[fs22-resources]: https://github.com/modelcontextprotocol/servers/blob/v2.3.0/src/filesystem/file-resources.ts#L173-L223
