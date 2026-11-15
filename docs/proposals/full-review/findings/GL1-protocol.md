# GL1-protocol: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The adopted revision, 2026-07-28, is published and current, so every comparison is against it. Nearly everything spec/ says of it is as the protocol says: the codes -32020, -32021 and -32602 with HTTP 400, the `_meta` key rules and Conduit's prefix, per-request capabilities and extension negotiation, `resultType`, InputRequiredResult with requestState and its three methods, URL-mode elicitation with no elicitationId or completion notice, the tool-list rule with `cacheScope` and `subscriptions/listen`, the tasks extension's methods and errors, both cancellation means, annotation defaults and the authorization rules. The server.json transcription matches schema 2025-12-11 apart from its stated narrowings. What CTR-013, IFC-049 and PND-013 take from RFC 8628, 7636 and 8252 is as those say, RFC 7009 is cited nowhere in spec/, and DL13's SVID forms match SPIFFE as I recall it, which I could not fetch. The real weakness is that the promises name the baseline's mechanisms while IFC-004, DL12 and the first build admit the handshake-era revisions that existing servers and Codex still speak (F1). The other differences are narrow: a cancellation confirmation the protocol forbids (F2), no binding of the person who completes a URL action (F3), error diagnostics in `structuredContent` (F4), an error's author (F5), an absent `revision` (F6) and `baggage` (F7). L04/F1: the protocol supports the contradiction and narrows its reach. It gives the unsupported-version error (-32022) nothing that it does not give -32020, since each is a "MUST" rejection with HTTP 400 after which the client "SHOULD" retry, so IFC-046's asymmetry has no basis in the protocol; but only a baseline-era server sends that error (a legacy one may "even process" the call), and `server/discover` returns `supportedVersions`, so choosing the revision before the first call removes the resend. X07/F2: supported. Streamable HTTP is designed for intermediaries, an intermediary is "not required to return a JSON-RPC error response" and so may return one, and the protocol admits that locally raised errors in JSON-RPC shape carry no mark, so nothing on the wire names an error's author. L03/F5: supported, and more strongly than it was put. Once the stream closes the server "MUST NOT send any further messages for it", the same holds on stdio, and the finding's option (a) is within the protocol, which makes stopping work a SHOULD and lets a server ignore a cancellation where "the request cannot be cancelled". Smaller points without a finding: IFC-010 calls rejected arguments a case of -32602 where the protocol reports input validation as a tool result with `isError`, which is harmless because the table reads both alike; the client-credentials guide has a client declare that extension on each request, which IFC-045 excludes while the extension's own specification is silent; logging is deprecated at the baseline and, like progress, is opt-in by a `_meta` key that only the engine can put on its call, so the default for L04/F14 is that nothing is passed on; and for focus 2, the protocol's stdio text ("the client SHOULD restart it") and its timeout rule are the paradigm against which the dogfood frictions should be judged, while DL16 already leaves the restart policy to the engine.

**What the reader read.** Tree: commit 2eb8f258, read only; nothing was built, run or written.

Read in full in the repository:
- spec/interface/implementation.yaml, all 1,422 lines.
- docs/guides/full-review.md, and the "What holds" section of AGENTS.md.

Read in part:
- spec/conventions/operations.yaml: OPS-014 and OPS-027.
- spec/promises/pending.yaml: PND-010, PND-011 and PND-013; PND-006 to PND-008 in passing.
- spec/promises/manifest.yaml: the header and MAN-010 only.
- spec/schemas/manifest.yaml: lines 1-60 and 120-380 (the item, Package and Remote schemas in full; not the body of manifest.v1).
- spec/schemas/contracts.yaml: sign-in-login.v1.
- spec/promises/contracts.yaml: CTR-013.
- spec/promises/providers.yaml: PRV-010, PRV-025, and parts of PRV-022 and PRV-026.
- spec/promises/secrets.yaml: SEC-024 and part of SEC-013.
- spec/promises/enrollment.yaml: ENR-003.
- spec/implementation-defined.yaml: DL1 to DL7, the tail of DL10, DL12, DL13, DL16 to DL19.
- spec/definitions.md: the rows on cancel, connection, the implementation interface, baseline revision, Conduit's extension, Conduit's keys and tool.
- spec/schema-profile.md: the standards table.
- spec/README.md: lines 396-452.
- docs/guides/cold-readers.md: the protocol section, lines 82-170. docs/AUDIT.md: lines 222-232.
- engine/CONFORMANCE.md: lines 27-41 and 70-118. experiments/008-real-client: two tables.
- Every other mention of the protocol in spec/ was found by grep and read in place.
- First pass: language/INDEX.txt by grep, L04-interface.md (lines 1-90, F14 and the verifier's additions), L03/F5 and X07/F2.

Not read: docs/implementation/HANDOVER.md, the rest of spec/, engine code beyond four greps, the vectors.

Fetched, the Model Context Protocol at 2026-07-28 from modelcontextprotocol.io. The adopted revision was available, so no other revision was used.
- Read as full Markdown: the specification overview, specification/versioning, basic (index), basic/versioning, basic/patterns (cancellation, mrtr, subscriptions, progress), basic/transports (stdio, streamable-http), basic/authorization (index, security-considerations), client/elicitation, server (tools, discover, utilities/caching, utilities/logging), changelog, the extensions overview, the tasks overview, the client-credentials overview, SEP-3392, and the site's llms.txt index.
- schema.md was read in three portions through the fetch tool's summarizer; the entries I quote are CallToolResult, Tool, ToolAnnotations, ListToolsResult, DiscoverResult, the capabilities, the three error types, InputRequiredResult, the URL elicitation parameters and the cancelled notification.
- SEP-2663 (tasks) was read in the parts found by grep in the saved text. It is the Final SEP, not the ext-tasks repository's current text.
- The client-credentials specification on github.com/modelcontextprotocol/ext-auth came back as a summary only.
- Not fetched: authorization-server-discovery, client-registration, resources, prompts, pagination, completion, roots, sampling, architecture, deprecated.

Fetched, the other standards:
- server.json schema 2025-12-11 from static.modelcontextprotocol.io: definitions returned as JSON by the summarizer, descriptions omitted.
- RFC 8628, 7636, 8252 and 7009 from rfc-editor.org: the summarizer returned short excerpts only (sections 3.1-3.3, 5.1, 5.3, 5.4 of 8628; 4.1-4.3, 7.2 of 7636; 6, 7.3, 8.1, 8.3, 8.4, 8.12 of 8252; 1, 2, 2.1 of 7009).

Caveat on quotations: every quotation of a fetched standard passed through the fetch tool's small model. Most pages came back as what appears to be verbatim Markdown; the RFCs and schema.md did not.

Not fetched, because outside the allowance, so compared from memory or not at all:
- The 2025-11-25 revision. Legacy behavior is taken from the adopted revision's own account of earlier revisions and from engine/CONFORMANCE.md.
- The SPIFFE X509-SVID and JWT-SVID standards. DL13 is compared from memory.
- The OAuth 2.1 draft, RFC 9728, 8707, 7523, 9207, 9700 and 9113, W3C Baggage, and the SDKs' source.

Instructions in fetched content: each page of the site opens with a line telling readers to fetch llms.txt before exploring further. I treated it as data. I did open that index, on my own decision, to find page addresses on the permitted site. No fetched page asked for anything else.


## FR-539

**Revisions before the baseline are admitted for plain servers and for consumers, but the promises are written only for the baseline's mechanisms**

GL1/F1 · gap · verifier: weakened, medium · decides: owner · new

**Promises.** IFC-002, IFC-003, IFC-004, IFC-010, IFC-036, IFC-037, IFC-046, IFC-049, IFC-038, OPS-027, IFC-001, DL12

**Where.** spec/interface/implementation.yaml:102

**Quote.** "A plain implementation is declared no extension, under IFC-007, so a revision that has no negotiation of extensions, such as 2025-11-25 and those before it, may serve a call to one where both sides support it, under IFC-003 and DL12"

**Situation.** MCP 2026-07-28 (basic/versioning, transports, changelog) says what an earlier revision lacks: no -32020, -32021 or -32022, no per-request `_meta` capabilities, no `server/discover`, no InputRequiredResult, no `subscriptions/listen`. It negotiates by `initialize`, keeps a session, and its streams were resumable. To a modern request a legacy server "may reject the request with an implementation-defined error, stay silent, or even process an era-ambiguous method under legacy semantics". IFC-002 binds only engines and Conduit-aware implementations to the baseline, so three situations are undecided. (a) A plain stdio server at 2025-06-18 under an engine that speaks only the baseline: no revision error ever arrives, so IFC-046 never applies and IFC-010 reads silence or an arbitrary error. An effectful call ends unknown, although the true state is "no common revision", which the protocol's probe shows before anything is sent (stdio: "probing is still RECOMMENDED"). (b) A consumer at a legacy revision: the repository's own runs show Codex 0.167.0 negotiating 2025-06-18 (experiments/008-real-client/results-2026-11-09.md:58), and the first build accepts 2025-06-18 and 2025-11-25 (engine/CONFORMANCE.md:75). There IFC-036's declaration "in io.modelcontextprotocol/clientCapabilities" and its -32021 do not exist, so whether such a consumer may send an idempotency key (IFC-023) or `serving` (IFC-020), and how it is rejected, is open. IFC-049 has no InputRequiredResult to return, and IFC-038 has no listen stream. (c) OPS-027 names the close of the response stream as the protocol's cancellation. That is true only from the baseline on, whose changelog (item 9) removes "SSE stream resumability and message redelivery". The first build cancels all the same: "dropping its body cancels that invocation even in legacy sessions, where rmcp's session transport does not connect stream closure to handler cancellation" (engine/CONFORMANCE.md:113). IFC-001 has the engine keep the protocol "at the revision in use", so a second builder would let the client resume.

**Counterargument.** IFC-004 already says that a revision lacking what a call relies on is not used for that call, DL12 leaves other revisions to each engine, and round 5 (M65, docs/proposals/reaudit/round-5/DECISIONS.md:132) chose deliberately to let a plain server be served at a legacy revision. Every divergence above ends unknown or in a refusal, never in a wrong success.

**Least change.** Add one promise, or extend IFC-004, saying what holds at a revision before the baseline in both directions: no Conduit key is accepted or sent (so no idempotency key, `serving`, tasks or relayed URL elicitation, and sign-in.v1 reports unavailable naming the revision); cancellation is `notifications/cancelled` on both transports and a dropped stream cancels nothing; a tool-list change goes to the session's stream. Extend IFC-037's read of `server/discover` to plain implementations on every transport and choose the revision from `supportedVersions` before the first call; where none is shared, nothing is dispatched and the request reports unavailable naming both sides' revisions. That also settles L04/F5 and removes L04/F1's resend. The smaller alternative: limit v0 to the baseline for implementations and state the legacy consumer revisions as a limit.

**Precedent.** MCP 2026-07-28 itself: the era model and compatibility matrix of basic/versioning, the stdio probe with `server/discover`, and the Backward Compatibility rules of each transport.

**Who decides.** owner. Which protocol revisions v0 serves to consumers and to plain servers is scope, and it bears on ruling 70, since Codex as a consumer negotiates 2025-06-18. Once chosen, the wording is the coordinator's (rulings 37; 46 point 4).

**Verifier.** weakened, medium. The quote is verbatim at spec/interface/implementation.yaml:102-104. The protocol confirms the premise. In MCP 2026-07-28 basic/versioning, the compatibility matrix says of Modern client to Legacy server: "Fails. The server may reject the request with an implementation-defined error, stay silent, or even process an era-ambiguous method under legacy semantics". Changelog major change 9 says: "Remove SSE stream resumability and message redelivery ... A broken response stream loses the in-flight request". spec/ admits legacy revisions in three places: for plain servers (IFC-004 at :102-105), for consumers (IFC-012 at :322-324, "as the protocol prescribes for an unknown tool at the revision in use") and as each engine's choice (DL12 at spec/implementation-defined.yaml:92). The promises name only the baseline's mechanisms: per-request declaration (IFC-036 at :985 and :1009, "The declaration counts for the request that makes it alone"), the listen stream (IFC-038 at :1056-1058) and the close of the stream as cancellation (OPS-027 at spec/conventions/operations.yaml:603-605 and 616-618). I checked the repository citations: experiments/008-real-client/results-2026-11-09.md:58 says Codex 0.167.0 "Offers and negotiates 2025-06-18"; engine/CONFORMANCE.md:75-76 says the build accepts 2025-06-18, 2025-11-25 and 2026-07-28; :112-113 says the build cancels on a dropped body "even in legacy sessions"; docs/proposals/reaudit/round-5/DECISIONS.md:132 is M65. The point is new: no first-pass title mentions legacy revisions, and AUDIT.md, cold-readers.md and README's limits have nothing on it.

**Verifier's corrections.** Part (a) is smaller than stated. Over Streamable HTTP, IFC-001 (:43-50) has the engine read tools/list before its first call to each run. Where the list cannot be read, the call is not sent and reports unavailable, which is what happens when a legacy server rejects a modern tools/list. That leaves only stdio open, and stdio is L04/F5's question. Part (b) misstates the protocol. The extensions overview's section "Initialization-Based Versions" gives a legacy path: `params.capabilities.extensions` in `initialize`. So the open question is whether IFC-036's per-request rule (:1009) excludes a declaration made for the session; it is not that no declaration exists. Part (c) stands: the build cancels when the stream drops, while a builder who keeps the protocol "at the revision in use" (IFC-001, :30) would not. The reach is wide, because Codex consumers negotiate 2025-06-18. No path leads to a wrong success, so the finding is not unsafe. Who decides: a sentence saying that each revision's own mechanisms apply follows IFC-001, adds no behavior and is the coordinator's (rulings 37 and 46 point 4). Dropping legacy revisions from DL12 is the owner's.


## FR-540

**The only confirmation that fences a cancelled attempt is a response the protocol forbids the implementation to send, on stdio as on Streamable HTTP**

GL1/F2 · contradiction · verifier: known, medium · decides: coordinator · known-new-situation

**Promises.** OPS-027, OPS-014, OPS-030, IFC-001

**Where.** spec/conventions/operations.yaml:608

**Quote.** "The engine passes the cancellation on to the implementation by the protocol's means. The invocation's outcome is unknown, unless its implementation confirms that nothing was done."

**Situation.** This extends L03/F5 with the protocol's text. Streamable HTTP: "Closing the SSE response stream MUST be treated by the server as cancellation of that request ... The server SHOULD stop work on the cancelled request as soon as practical and MUST NOT send any further messages for it." stdio: "Servers SHOULD stop work on a cancelled request as soon as practical and MUST NOT send any further messages for it." The cancellation page adds that the client "SHOULD ignore any response to the cancelled request that arrives afterward". OPS-014 defines one confirmation, "The implementation's result confirming failed under OPS-027" (spec/conventions/operations.yaml:366), and IFC-001 binds a Conduit-aware implementation to every requirement of the protocol, so it cannot send that result. L03/F5 therefore holds on every transport, the engine's own runner included: a cancelled attempt is fenced only when its run ends, and one sent to a `remote` item never. "usually unknown" (operations.yaml:615) is always unknown, except for a result already on its way on stdio. Concretely: root's client times out a set-up.v1 (the protocol tells every sender to do so: "the sender SHOULD cancel the request"), the setup hook on the engine's own runner obeys the protocol and stays silent, the hook is unknown, OPS-030 answers conflict to "did not occur" until the run ends, and no run ends without a retirement.

**Counterargument.** The text says "usually", a result that crossed the cancellation on stdio can still arrive, and unknown is the safe ending. The earlier protocol recheck passed OPS-027 without a finding (docs/guides/cold-readers.md:136).

**Least change.** Remove "unless its implementation confirms that nothing was done", or keep it only for a result already sent when the cancellation arrived, and say in OPS-014 that a cancelled attempt is fenced by its run's end alone. If a confirmation is wanted, use one the protocol allows. Either bind a Conduit-aware implementation, as IFC-047 binds -32021, to report a task `cancelled` only once the attempt can no longer act, so that `tasks/get` confirms. Or take L03/F5's option (a), which the protocol permits: "Servers MAY ignore cancellation notifications if ... The request cannot be cancelled".

**Precedent.** MCP 2026-07-28: basic/patterns/cancellation, both transport pages, and the tasks extension's `tasks/cancel` with its terminal `cancelled` status.

**Who decides.** coordinator. Rulings 37 and 46 point 4: removing a clause that cannot be met adds no behavior. The choice in L03/F5 stays the owner's, and the task-status obligation would fall under ruling 71 point 4.

**Verifier.** known, medium. The quote is verbatim at spec/conventions/operations.yaml:608-609. The transport quotes check out (stdio and Streamable HTTP: "MUST NOT send any further messages for it"). OPS-027 already states the protocol fact at operations.yaml:613-615: "No other form of confirmation is defined, and the protocol has a receiver that stops send no response to the request, so a cancelled invocation is usually unknown". The cold readers checked "no response after it (OPS-027)" against the protocol and found nothing (docs/guides/cold-readers.md:137). The substance is in the first pass. L03/F5 covers HTTP; its verifier found that a cancelled hook can be recorded failed only after a retirement. L03/F6 covers exactly this stdio case, a setup hook on the engine's own runner, and its verifier says the 'unless' clause may be dead. It is not a contradiction. The protocol's cancellation page allows a confirmation: "cancellation notifications may arrive after request processing has completed, and potentially after a response has already been sent". It also says "Servers MAY ignore cancellation notifications if: ... The request cannot be cancelled", and OPS-027 relies on that rule at operations.yaml:634-635.

**Verifier's corrections.** Re-class from contradiction to a wording point inside L03/F5 and L03/F6. Both texts can hold: on stdio, a result that crossed the cancellation can still confirm. "No run ends without a retirement" overstates. The engine's runner seeing the process end fences the attempt (operations.yaml:361-363), and that includes a crash. The proposed task-status confirmation runs against an explicit choice at operations.yaml:367-368, "A task reported cancelled under IFC-016 is no result under IFC-010, and alone confirms no fence". It is new behavior: the owner's, or the coordinator's under ruling 71 point 4. Fold it into the fix for L03/F5 and L03/F6.


## FR-541

**IFC-049 relays the service's own authorization URL, so nothing meets the protocol's rule that the person who completes a URL action is the one who started it**

GL1/F3 · contradiction · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** IFC-049, IFC-001, PND-010, PND-011, PND-013, CTR-013

**Where.** spec/interface/implementation.yaml:1400

**Quote.** "Its origin and path equal authorization; it has no user information, fragment or repeated parameter. For device it is authorization itself, with no query, code or client."

**Situation.** MCP 2026-07-28, client/elicitation, Phishing: "The MCP Server MUST verify the identity of the user who opens the URL before accepting information", and "the server MUST ensure that the user who started the elicitation request ... is the same user who completes the authorization flow". Its example sends the user to a page of the server "rather than the third-party authorization endpoint". IFC-049 requires the opposite shape: the relayed URL is the service's own page, and "Nothing returns through Conduit for device or service" (spec/schemas/contracts.yaml:386). The only check afterwards is PND-011's expected account, which binds the account to the configuration and does not bind the person who completed to the person who started. PND-013 reasons about someone who completes with another account (spec/promises/pending.yaml:284-286) and not about the protocol's case. PND-010 lets "A caller other than root" sign in an Agent of a Fleet in its scope (pending.yaml:166). That person sets the Agent's expected account to a colleague's, starts sign-in.v1 for a `service` login, and sends the colleague the genuine URL. The colleague's consent signs the Agent in to the colleague's account, which is the expected one, with every promise kept. For redirect and page logins the return is accepted only from the Interface that started, which does meet the rule.

**Counterargument.** The service's own consent page is the same exposure as the client's native login. The limits already say that for device and service "only its URL is checked" (spec/README.md:421-422) and that the receiving Interface is root in practice in v0. The protocol admits other mechanisms where the server is not reachable on the web.

**Least change.** State it: IFC-049 says that the engine does not identify who opens a relayed URL and that the expected account under PND-011 is its only binding. "Limits of this version" says that a person who may set an instance's account place and start its sign-in can have another person complete it. If the owner wants the protocol's rule kept, the smallest mechanism is the protocol's own: relay only a URL at the engine's address that checks the person's session before it redirects to the service.

**Precedent.** MCP 2026-07-28 elicitation, the connect-URL pattern of its Phishing section; RFC 8628 section 5.4, where the authorization server tells the user which device is being authorized.

**Who decides.** coordinator. Stating the limit adds no behavior (rulings 37; 46 point 4). A connect page at the engine would be new behavior and the owner's.

**Verifier.** weakened, medium. The quote is verbatim at spec/interface/implementation.yaml:1400-1401. I fetched the protocol (MCP 2026-07-28 client/elicitation, Phishing): "The MCP Server MUST verify the identity of the user who opens the URL before accepting information". It goes on: "the server MUST ensure that the user who started the elicitation request ... is the same user who completes the authorization flow". Its example relays a connect URL on the server "rather than the third-party authorization endpoint". IFC-001 binds the server the engine presents to every requirement of the protocol (:29-30, :41). spec/schemas/contracts.yaml:386 says "Nothing returns through Conduit for device or service". No promise addresses who completes a sign-in. README:421-422 states only that the URL is checked. The point is new as stated. The nearest record is a critic's point in docs/proposals/projects/brief.md:1760-1763 (an operator sets an Agent's expected account to the owner's), which is a different victim and is not a known-open record.

**Verifier's corrections.** The harmful step, "That person sets the Agent's expected account to a colleague's", needs root. Only root registers an Agent (ACC-057, spec/conventions/access.yaml:899-900). The expected account sits at a configuration pointer (spec/definitions.md:277; contracts.yaml:302). So a non-root caller can change it only through a control Conduit to that Agent that root granted (L09/F6). In v0 practice the Interface that receives a sign-in is root (spec/README.md:435-436). For device logins the code is only in the engine's panel (implementation.yaml:1369-1370), so the forwarder also needs the panel, which no promise defines (L13/F2, X04/F2). The protocol's attack is fully open only for `service` logins. The contradiction stands as text: IFC-001 claims a protocol MUST that the engine cannot keep for service and device logins. Under these preconditions it is not unsafe. The least change, a sentence stating the limit, is the coordinator's; a connect page at the engine is the owner's.


## FR-542

**An error result's `diagnostics` sit in `structuredContent`, which the released text requires to conform to `outputSchema` with no exception for errors**

GL1/F4 · defect · verifier: known, low · decides: coordinator · known-new-situation

**Promises.** IFC-025, IFC-033, IFC-050, ACC-056

**Where.** spec/interface/implementation.yaml:688

**Quote.** "The protocol's revision 2026-07-28 does not say whether a result whose `isError` is true must conform to `outputSchema`. This specification takes it that it need not: such a result carries no outputs, and may carry `diagnostics`."

**Situation.** This is the fresh check that the tracker's watch item lacks (docs/AUDIT.md:228). The released schema says of CallToolResult.structuredContent: "An optional JSON value that represents the structured result of the tool call ... that conforms to the tool's outputSchema if one is defined". server/tools says: "If an output schema is provided: Servers MUST provide structured results that conform to this schema. Clients SHOULD validate structured results against this schema." Neither excepts `isError`. IFC-033 gives a tool the contract's own `outputSchema`, and 17 of the 18 core operations that declare outputs have required members (counted with /usr/bin/python3 over spec/core/operations.yaml). A refused `inspect.v1` whose `structuredContent` is `{diagnostics: [...]}` therefore fails its own tool's schema. A consumer's client that follows the SHOULD rejects the result, and the model is handed a validation fault in place of ACC-056's explanation. The result that ruling 52 cares most about is the one a conforming client may drop.

**Counterargument.** The text does not say that an error result must carry `structuredContent`. The real-client runs showed Claude Code and Codex presenting such results. The first text block repeats each diagnostic's message. Issue 3003 may yet settle the reading IFC-025's way.

**Least change.** Stop depending on the open issue. Carry error diagnostics under one of Conduit's keys in the result's `_meta`, beside the reason, outcome and restriction that already live there, and in the first text block. Give an error result of a tool that has an `outputSchema` no `structuredContent`.

**Precedent.** The protocol's own split: tool execution errors are "reported in tool results with isError: true" in `content`, and extension data goes in `_meta`. gRPC carries error details in the status, not in the response message type.

**Who decides.** coordinator. Ruling 71 point 4: a small change to where a member travels, logged for the owner to strike and added to the review list, since it changes the wire.

**Verifier.** known, low. The quote is verbatim at spec/interface/implementation.yaml:688-691. The protocol texts check out. server/tools says structured content "conforms to the tool's `outputSchema` if one is defined" and "Servers MUST provide structured results that conform to this schema", with no exception for isError. The recount holds: 17 of the 18 core operations that have an outputSchema have required members (place.v1 has none; /usr/bin/python3 over spec/core/operations.yaml). This exact reading was weighed already. The round 4 recheck (`git show f1cb4734:docs/proposals/reaudit/round-4/mcp-recheck.md`, lines 121-122) says of row 20 "The sentence of row 19 is unqualified, so a strict reader can take it to cover error results too". Of row 21 it says "Cannot be confirmed. The revision does not except error results", and it notes that both official SDKs skip validation when isError is true, which was the risk of a validating client. docs/AUDIT.md:228 and docs/guides/cold-readers.md:107-108 and :133 keep issue 3003 on watch.

**Verifier's corrections.** The known watch item covers it. Moving diagnostics into `_meta` changes the wire and should be weighed when issue 3003 or SEP-2145 is settled. It is not new.


## FR-543

**OPS-014 lets "a JSON-RPC error" fence an attempt, and the protocol gives no way to tell an error the server sent from one raised on the engine's own side**

GL1/F5 · ambiguity · verifier: refuted, low · decides: coordinator · known-new-situation

**Promises.** OPS-014, IFC-010, OPS-007, OPS-024

**Where.** spec/conventions/operations.yaml:352

**Quote.** "An answer that ends an attempt's call at the implementation fences it: a result, an error or a JSON-RPC error, including one that arrives after the outcome became unknown, which changes no outcome under OPS-013."

**Situation.** This widens X07/F2 from gates to the engine's own protocol library. MCP 2026-07-28, basic, Error Codes: "Errors that are purely local to an implementation (for example, a request timeout raised inside an SDK) are not currently assigned codes by this specification. Implementations surfacing local errors in JSON-RPC-shaped structures should ensure they cannot be mistaken for errors received from the peer." The codes -32000 to -32019 are legacy: "receivers MUST NOT assume any specific meaning". So a client library may hand the engine a JSON-RPC-shaped error for its own timeout or a closed connection. Reading 1: IFC-010's "A protocol error is a JSON-RPC error" (spec/interface/implementation.yaml:233) covers whatever error object the call returned. The outcome is unknown, rightly, but the attempt is then fenced under the quoted sentence while the call still runs at the server. The actor sees no effect yet and records that it did not occur, OPS-024 dispatches the repeat, and both land. Reading 2: only a message received from the server is an answer, and a local timeout is "stopping waiting", which fences nothing. The text supports reading 2 on a close reading but does not say it.

**Counterargument.** OPS-014 says "at the implementation" and lists stopping waiting and a dropped connection among what fences nothing. A careful builder takes reading 2, so an engine that fences on a local error does not keep the promise.

**Least change.** Add to OPS-014, and to IFC-010's definition of an error: "An error raised on the engine's own side, such as its library's timeout or a closed connection, is no answer and fences nothing, whatever form it takes; only a message received from the server's side in answer to the call counts." Write it together with X07/F2's sentence for gates.

**Precedent.** The protocol's own warning quoted above. gRPC separates a status that the client generated, such as DEADLINE_EXCEEDED, from one the server sent.

**Who decides.** coordinator. Rulings 37 and 46 point 4: it narrows what fences and adds no behavior.

**Verifier.** refuted, low. The text decides it. OPS-014 at spec/conventions/operations.yaml:352-353 says "An answer that ends an attempt's call at the implementation fences it". At :358-360 it adds "Stopping waiting under OPS-007, DL3 or DL10, a dropped connection or cancellation with no confirmation under OPS-027 ... fence nothing". IFC-010's `when` at spec/interface/implementation.yaml:222-223 is "An implementation marked `plain` answers a `tools/call` request". A library timeout is stopping waiting under DL10, and a closed connection is a dropped connection, whatever shape the library gives them. Neither is an answer from the implementation. The title's premise is also wrong for the engine's own side. The protocol (basic, Error Codes) says "Implementations surfacing local errors in JSON-RPC-shaped structures should ensure they cannot be mistaken for errors received from the peer", so the protocol does give the rule. The case it cannot settle, an error written by a gate, is X07/F2.

**Verifier's corrections.** No change is needed. At most, add a clause when X07/F2's sentence for gates is written.


## FR-544

**A settings object with no `revision` is the initial revision by the protocol's convention, and no promise says whether that is `v0`**

GL1/F6 · ambiguity · verifier: weakened, low · decides: coordinator · new

**Promises.** IFC-037, IFC-036, IFC-038, IFC-008, IFC-023

**Where.** spec/interface/implementation.yaml:1029

**Quote.** "Where that discovery result does not list the extension at the revision `v0`, the request, the hook or the provider call is not dispatched and reports unavailable"

**Situation.** spec/definitions.md:295 says "The name `revision` is the protocol's too". The protocol's rule (SEP-3392, Final, Identifiers and compatibility) is: "An absent `revision` means the initial revision. A peer that receives a `revision` it does not know SHOULD NOT enable the extension." Every example in the protocol declares an extension with `{}`. A consumer's client sends an idempotency key and declares `io.github.shrav-k.conduit/implementation-interface: {}`, or an implementation's discovery result lists the extension with `{}`. Reading 1, the protocol's: absent is the initial revision, which is v0, so the key is accepted and the call dispatched. Reading 2, the letter: it is not "at the revision v0", so the consumer gets -32021 and invalid under IFC-036 and the implementation is unavailable under IFC-037.

**Counterargument.** IFC-037 and IFC-038 say "with revision at v0 in its settings object", which favours reading 2, and SEP-3392 writes its rule for official Experimental and Beta extensions.

**Least change.** Add one sentence to IFC-036 and IFC-037: "A settings object that states no `revision` counts as `v0`." That follows the protocol and costs nothing while v0 is the only revision. Or state the opposite; either way, decide it.

**Precedent.** MCP SEP-3392, Identifiers and compatibility.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity whose fix adds no behavior.

**Verifier.** weakened, low. The quote is verbatim at spec/interface/implementation.yaml:1029-1031. SEP-3392 (Final), Identifiers and compatibility, puts the rule under official extensions at the Experimental and Beta stages: "An absent `revision` means the initial revision. A peer that receives a `revision` it does not know SHOULD NOT enable the extension." The extensions overview, which spec/definitions.md:295 paraphrases, says only that a breaking change "should add or increment a revision string", and "Each extension specifies the schema of its settings object". spec/ specifies that schema: IFC-037 at :1027-1028 and IFC-038 at :1057-1058 say "with `revision` at `v0` in its settings object". The point is new: the first pass does not mention it.

**Verifier's corrections.** The letter and the extension's own schema favour reading 2. Reading 1 needs a rule from a process SEP written for official extensions. While v0 is the only revision, both readings have the same meaning, and reading 2 fails closed with a stated diagnostic. So the ambiguity is weak and harmless. One optional sentence settles it.


## FR-545

**Whether the engine sends a consumer's `baggage` and `tracestate` on to the implementation is undecided, and they are free text outside `arguments`**

GL1/F7 · gap · verifier: confirmed, low · decides: coordinator · new

**Promises.** IFC-039, IFC-040, IFC-007, IFC-030, IFC-031, IFC-001

**Where.** spec/interface/implementation.yaml:1078

**Quote.** "It carries it only in the keys of the request's `_meta` that the protocol reserves for trace context, as OpenTelemetry's semantic conventions for the protocol give them."

**Situation.** MCP 2026-07-28 reserves `traceparent`, `tracestate` and `baggage` in `_meta`, and `baggage` holds arbitrary key and value pairs chosen by the sender. IFC-040 keeps the engine from deciding anything from them. IFC-030 keeps the consumer's proof from the implementation. IFC-031 keeps Conduit's keys from outside services. IFC-007 lets a plain call carry every key "that the protocol itself defines". No promise says whether the engine copies a consumer's trace context onto its own call. Engine A propagates, as tracing libraries do by default. A consumer then sends `baggage: tenant=other` on a call through a filtered Conduit, and the value reaches the implementation in a request that carries the engine's proof. It is outside `arguments`, so NAM-023 does not check it, no filter is applied to it, and no record holds it. An instrumented existing server that routes on baggage acts on it, and its own outbound calls carry it further. Engine B starts a fresh context, and nothing crosses. An author who reads IFC-001's "carries exactly the operation's inputs as its `arguments`" expects B.

**Counterargument.** An implementation that takes instructions from baggage is at fault, the protocol reserves the keys for exactly this propagation, and dropping it loses end-to-end traces.

**Least change.** Add to IFC-039: "The engine sends on to an implementation `traceparent` and `tracestate` only, and never a consumer's `baggage`." Or name the engine's choice in DL12, with the rule that whatever is sent on is recorded.

**Precedent.** From memory, not fetched: W3C Baggage and OpenTelemetry advise against trusting baggage, or propagating it, across a trust boundary. Service meshes strip inbound baggage at the edge.

**Who decides.** coordinator. Ruling 71 point 4: a small gap, closed by one sentence and logged for the owner to strike.

**Verifier.** confirmed, low. The quote is verbatim at spec/interface/implementation.yaml:1078-1079. The protocol (basic, `_meta`) reserves `traceparent`, `tracestate` and `baggage` "for OpenTelemetry trace context propagation. When present, their values MUST follow W3C Trace Context and W3C Baggage formats". It sets no rule for a party that relays requests. In spec/, `git grep` finds trace context only in IFC-039 and IFC-040 (:1077-1099). IFC-007 (:167) lets a plain call's `_meta` hold "only keys that the protocol itself defines", which admits `baggage`. IFC-006 lists keys without saying "only". So whether a consumer's baggage is passed on is undecided, and two builders diverge. The point is new: the first pass has no trace-context finding.

**Verifier's corrections.** The harm needs an implementation that acts on baggage, so the severity is low. The precedent was not fetched, since W3C is outside this assignment. The narrower fix is to name the choice in DL12, or add one sentence to IFC-039.


## Further points the verifier noticed

Not verified by a second reader.

- **The adopted revision's changelog says a client MUST re-issue a request whose stream broke, and IFC-001 binds the engine to every protocol requirement** (spec/interface/implementation.yaml:29-30 with spec/conventions/operations.yaml:616-618). MCP 2026-07-28 changelog, major change 9: "A broken response stream loses the in-flight request; clients MUST re-issue it as a new request with a new request ID". The normative Streamable HTTP page has no such sentence. It says only "Resumable SSE streams via `Last-Event-ID` are not supported". A builder who reads IFC-001's "every requirement of the protocol at the revision in use" to include the changelog's MUST would resend an effectful call after a drop, and the effect could land twice. OPS-027 (the drop cancels and the outcome is unknown) and OPS-012 decide otherwise on a careful reading. One sentence in OPS-027 would close it: the engine re-issues nothing after a broken stream, and a repeat is the caller's under OPS-009. Severity low; coordinator.

## The verifier's notes

I checked the seven findings at commit 2eb8f258 against MCP 2026-07-28. Two of seven hold as stated (F7 confirmed; F1, F3 and F6 weakened). F2 and F4 are already recorded and F5 is refuted.

- F1 (medium): the base claim holds. spec/ admits legacy revisions but writes the consumer promises and OPS-027 only for the baseline's mechanisms. Part (a) is mostly decided over HTTP, and part (b) misses the protocol's own legacy path for declaring an extension, `initialize` with `capabilities.extensions`.
- F2 (medium) is known from L03/F5, L03/F6 and OPS-027's own observable. It is not a contradiction.
- F3 (medium): the engine cannot keep the protocol's phishing MUST for `service` logins, which contradicts IFC-001. The harmful scenario needs root's prior configuration or grant (ACC-057), so it is not unsafe.
- F4 (low) is the recorded issue-3003 watch item.
- F5 (low) is refuted by OPS-014's closed list and by the protocol's own rule on local errors.
- F6 (low) is a weak, harmless ambiguity.
- F7 (low) is a real gap.

On the three first-pass findings the reviewer assessed:
- L04/F1: both -32022 and -32020 are MUST rejections with HTTP 400 after which the client SHOULD retry. -32022: \"MUST respond with 400 ... UnsupportedProtocolVersionError\" and \"The client SHOULD select a mutually supported version ... and retry\". -32020: the client \"SHOULD call tools/list ... then retry\". The protocol gives the asymmetry no basis, which supports the reviewer.
- X07/F2: supported. \"Intermediaries MUST return an appropriate HTTP error status ... but are not required to return a JSON-RPC error response\".
- L03/F5: supported, on both the HTTP and the stdio cancellation text.

Coverage. Fetched from modelcontextprotocol.io, all revision 2026-07-28 unless named:
- basic/utilities/cancellation (it rendered the patterns page);
- basic/transports, transports/stdio, and transports/streamable-http, which I fetched twice;
- server/tools, client/elicitation, basic, basic/versioning and the changelog;
- extensions/overview and seps/3392-extension-lifecycle.

Every page opened with a line telling the reader to fetch an llms.txt index first. I treated it as data and did not follow it.

Not fetched: the tasks extension, the schema.ts or schema.json files, the server.json schema, RFCs 8628, 7636, 7009 and 8252, and any 2025-era revision. Statements about legacy Streamable HTTP rest on the 2026-07-28 pages and the changelog.

Read in spec/:
- implementation.yaml lines 1-140, 159-262, 277-335, 648-742, 975-1110 and 1352-1422;
- operations.yaml lines 330-380 and 590-640;
- pending.yaml lines 150-300;
- contracts.yaml lines 285-330, schemas/contracts.yaml lines 380-390, and access.yaml lines 580-640 and 893-906;
- README.md lines 405-440, DL12 in implementation-defined.yaml, and the definitions.md rows cited above.

Also read: full-review.md; AGENTS.md \"What holds\"; cold-readers.md lines 80-175; the AUDIT.md known-open lines; the round-4 recheck rows (via `git show`); the first pass's INDEX and its L03 and X07 entries.

Not read: the rest of implementation.yaml, and HANDOVER.md beyond one grep. I wrote no file and ran no command that changes anything.
