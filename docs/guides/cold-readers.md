# Instructions for cold readers

For the owner's collaborators reading Conduit as the first build leaves it. Not binding: the
[specification](../../spec/README.md) alone binds. These instructions were written on
2026-11-10 for the **Read** row of the [roadmap](../../ROADMAP.md#the-order-of-the-work),
after step 3. As of that date, increments 1 to 3 are done and increment 4 is in progress:
both author runs completed T1 to T7 with real clients but failed the owner's bar for
authoring work. Simplification is being settled before it is built and S1 rerun. A read before step 3
is finished can help, but the build may still change the text. Record which text you read.

## What to read

Begin with [for the next reader](../AUDIT.md#for-the-next-reader). It sets the scope, the
reading order and known open points. Its [convergence rule and definition of unsafe](../AUDIT.md)
remain in the audit; use them when judging safety and readiness. Follow this order:

1. The top [README](../../README.md), for what Conduit is and what exists.
2. [spec/README.md](../../spec/README.md), then
   [spec/definitions.md](../../spec/definitions.md), for the layers and the terms in force.
3. The promises, in the specification's [contents order](../../spec/README.md#contents),
   checking the definitions, core contracts, schemas and examples they depend on.

Read the head of `main` as the build leaves it, and record its commit ID and your date. Do not
use `c4d7d33`, which round 6 read. If the owner assigns a narrower reading scope, state it in
the report. For a whole-repository read, use the [documentation map](../README.md) to follow
the entry points and supporting documents too. The [overview](../OVERVIEW.md) can help with
the idea, but it cannot supply a rule missing from the promises. Consult earlier decisions
when checking whether a finding is already known; they do not override `spec/`.

Walk concrete situations where promises meet: access changing while work is in flight, a
repeat after a lost result, retirement, and a secret crossing a provider boundary. Apply the
next-reader section's safety test, including a procedure that conflicts with a safety promise.
A passing vector tests its stated slice; it does not prove that the whole promise holds.

## What is known to be open

Use the [audit's known open points](../AUDIT.md) as the index, rather than reporting an acknowledged limit again
without a new situation. Its status statements have not all caught up with the build. As of
2026-11-10:

- The engine, vectors and a runnable shell exist. Use the
  [roadmap's step 3](../../ROADMAP.md#the-order-of-the-work), the
  [coverage manifest](../../conformance/manifest.yaml) and the build's
  [conformance document](../../engine/CONFORMANCE.md) for current coverage and
  implementation limits. A missing implementation is distinct from an undecided language rule.
- The [build logs](../proposals/first-build/README.md) record decisions since round 6. For
  example, increment 2 settled the reserved root `$defs` copy-key question and the two plain
  protocol-error cases. Check the settlement before raising the old question again.
- The [real-client closeout](../proposals/first-build/increment-3/closeout-results-2026-11-10.md)
  records T1, T2 and T7 and carries unshown checks forward. Access is mediated; Codex's home
  instructions still leak, and separate homes and the sign-in flow of
  [ruling 49](../proposals/ruling-49/README.md) are pending work. A ruling is not evidence that
  its specification or implementation changes have been made.
- [IN_FORCE.md](../../IN_FORCE.md) lists accepted language decisions as *spec pending* until
  they are written into the specification. The [language-shape investigation](../proposals/language-shape/README.md)
  and the owner's [Project direction](../proposals/owner-direction/2026-11-10-projects.md)
  precede that writing. Do not read a proposed short form or Project model as an existing rule.
- The [increment 4 log](../proposals/first-build/log-increment-4.md) distinguishes the fixes
  from orders 025 to 028 from open findings. In particular, 4-06 is settled as parallel model
  work within the parent Agent's access, 4-03 retains SUB-010's submission rule, and 4-08's
  explicit refusals are ruled but await promises. Turn deadlines (4-12) and the form of ended
  conditions (4-17) remain open; both measured author runs failed the bar for authoring work.

The deliberate limits of v0, including the two delegation limits, remain in the
[specification's limits](../../spec/README.md#limits-of-this-version). No round so far was
clean; the next-reader section explains how the readiness gate is judged. This guide does
not reassess that gate.

## The protocol

These are dated checks against MCP **2026-07-28**, not fresh claims about the protocol. Use the
claims and their sources when IFC-005's baseline moves. The watch list comes from the
[round 4 recheck](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/mcp-recheck.md#what-to-watch-in-the-protocol);
the claims and sources come from
[round 5's protocol reader](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-6.md#checked-without-a-finding).
Below, `S` stands for `https://modelcontextprotocol.io/specification/2026-07-28`; references
to `5-6-01`, `5-6-02` and `5-6-10` name sections of the linked round 5 report.

### What to watch

Copied from the [round 4 protocol recheck](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/mcp-recheck.md),
whose verdicts and numbered rows ("row 21", "row 19 of the earlier check") it refers to. These
are not claims of the specification; each could change one of that report's verdicts without any
change to `spec/`.

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

### Checked without a finding

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

### What was opened

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

## How to report a finding

Prepare one report per reader for `docs/proposals/reaudit/round-7/`, the next directory after
[round 6](../proposals/reaudit/round-6/README.md). Hand the local report to the coordinator;
do not publish it. Use [round 6's reader report](../proposals/reaudit/round-6/reader-1.md) as
the example of the form. At the top, give your date, commit ID, scope and what you did and did
not check. For each finding give:

- A unique ID, severity and a short title. Distinguish wrong, undetermined and misleading;
  mark unsafe cases and safety-critical contradictions explicitly, as the tracker asks.
- The file and line, promise IDs where applicable, and short exact quotes of the text at issue.
- A concrete situation showing the problem and the strongest counterargument you found.
- The least change that fixes it, and any cost or choice it leaves for the owner.

Report unclear entry text as well as defects in promises. Separate observed results from
reasoning and proposed tests. Keep recommendations pending: the owner rules, or the
coordinator decides under ruling 37, and the decision is logged beside the report. Round 6's
[decisions](../proposals/reaudit/round-6/DECISIONS.md) show that separate step.

## What not to do

Do not change `spec/` directly, read the sibling reference clones, or publish anything: no
commit, push, pull request or release. The task is to report what the text means to a new
reader, not to implement an engine or silently repair a rule. Follow [AGENTS.md](../../AGENTS.md)
for repository rules; do not read credentials or change the owner's configuration.

## How long it takes

The next-reader instructions and round 6's reports give no elapsed reading time or fixed
duration. There is no recorded estimate to promise here. State your coverage and any part
left unread when handing over the report.
