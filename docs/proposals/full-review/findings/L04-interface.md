# L04-interface: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The idea of this layer is right and small: one tool of the Model Context Protocol per operation, Conduit's keys as one extension, one table that turns a plain server's answers into outcomes, and no transport of its own. The text is neither clear nor minimal: 46 promises carry about 13,600 words, 69% of them in `observable` fields that hold further obligations, wire keys and one document rule, so a builder cannot tell what binds, and none of the 46 has a vector (F12). Layering leaks both ways: a rule that rejects documents sits in IFC-033 (F10), access conventions sit in IFC-016, IFC-021, IFC-028 and IFC-050, and IFC-049 mixes the wire form of a URL action with sign-in and enrollment behavior and a parameter-by-parameter check of an OAuth URL that belongs more to the login's package than to the wire. I would cut the handle concept (F13), IFC-005 and IFC-029 (promises about a future version that no engine of this version can keep or break), and the hook-filter clauses of IFC-006, IFC-018, IFC-019 and IFC-026 that ACC-018 says are empty in this version (spec/conventions/access.yaml:342-347); and I would ask the owner whether the non-plain `remote` path (the proof clauses of IFC-041, with IFC-043 and IFC-044) earns its machinery in v0, since no package or scenario in the tree uses a `remote` item at all. The weakest point is how failures are read: one class of fault ends refused, invalid, unavailable or unknown depending on the transport, on whether the server is plain and on which promise is read (F2, F4, F5), and unknown is given where nothing could have happened (F11), which turns ordinary outages and lapsed logins into reconciliation work for a person. Results still fall short of ruling 52 in one plain way: a Conduit-aware implementation's own account of why it failed need not reach the caller (F3). The one finding I mark unsafe is IFC-046's unconditional resend after a revision error, which breaks the standard the same promise sets for -32020 (F1). The identity promises (IFC-022, IFC-042) are coherent, but most of their mechanism is in DL13, an implementation-defined item that I did not review.

**What the reader read.** Commit 2eb8f258 at the repository root, read only; nothing was built or run except read-only greps and one word count with /usr/bin/python3. Read in full, line by line: spec/interface/implementation.yaml (all 1,422 lines, 46 promises: IFC-001 to IFC-023, IFC-025 to IFC-033, IFC-036 to IFC-047, IFC-049, IFC-050). Read in spec/definitions.md: lines 1 to 303 (core terms through "Implementations and the implementation interface", including "How a request ends") and lines 408 to 467 ("What is left to each engine" and "Words the promises do not use"); not read: lines 304 to 407 (Providers, Secrets, Lifecycle, Inspection, Configuration and change, Packages). Read in spec/README.md: lines 24 to 97 (what this is, the three layers), 281 to 337 (form of a promise, conformance) and 331 to 678 (limits of this version to the end); not read: lines 98 to 280 (reading paths, contents). Read: docs/guides/full-review.md in full; AGENTS.md "What holds"; docs/AUDIT.md lines 174 to 246 (known to be open) and 361 to 411 (layered claim, layer decisions); docs/guides/cold-readers.md lines 39 to 173; IN_FORCE.md only by grep for lines that cite IFC promises or are marked Open or Spec pending; docs/proposals/first-build/increment-2/decisions.md (decision 8a and its evidence), findings 2-02 and 2-05 to 2-07, and finding 4-12. Cited promises opened and read whole: OPS-001 to OPS-005, OPS-007 to OPS-009, OPS-011 to OPS-013, OPS-018, OPS-023, OPS-024, OPS-027, OPS-029, OPS-032 to OPS-034; ACC-001, ACC-003, ACC-011, ACC-017 to ACC-019, ACC-034, ACC-037, ACC-056; NAM-004, NAM-023, NAM-024, NAM-026; TPL-013; OBJ-022; CON-017; CTR-002; PKG-016, PKG-020; PND-002, PND-005, PND-010, PND-011, PND-013; ENR-003, ENR-004; PRV-005, PRV-025, PRV-028, the first 60 lines of PRV-007; SEC-013, SEC-024; REC-001; DL3, DL4, DL10, DL12, DL13, DL16 to DL19; the core schema handle.v1. Cited but not opened: PRV-022, PRV-026, SEC-006, SEC-010, SEC-011, SEC-023, REC-002, CTR-012, CTR-013, TPL-010 to TPL-012, TPL-017, TPL-018, LIF-013, OPS-014, OPS-028, IMP-007, ACC-016, ACC-039, the rest of spec/core/. Not checked: the Model Context Protocol at revision 2026-07-28 itself (no network; what the promises say of the protocol was taken as stated), the engine's code, and the conformance vectors.


## FR-033

**IFC-046 resends an effectful call after a revision error, without the safety condition it applies to -32020**

L04/F1 · contradiction · unsafe · verifier: confirmed, high · decides: coordinator · known-new-situation

**Promises.** IFC-046, IFC-003, IFC-010, IFC-007, OPS-018, IMP-007

**Where.** spec/interface/implementation.yaml:1284

**Quote.** "After an error for the revision it sends the call once more at a revision that both support, where there is one. The two are one call, and what it ends with is read from the response to the second."

**Situation.** A plain server implements an effectful operation (the shipped Agent wrappers are plain and serve turn.v1, packages/agent-claude-code/conduit.yaml:16). It answers a call with the protocol's unsupported-version error. IFC-046 has the engine send the same effectful call again, with no idempotency key (IFC-007), and count both as one invocation. The same promise says of -32020 that the code "gives no guarantee that a plain server has not acted, so an effectful call is never repeated after that code" (implementation.yaml:1288-1289), and finding 2-06 records why: "Unconditional recovery can repeat an effect" (docs/proposals/first-build/README.md:80). Nothing in spec/ gives the revision error a stronger guarantee, so by the specification's own standard a promise-keeping engine can apply an effect twice. The text is also inconsistent the other way: where no common revision exists, the same error "is read as IFC-010 and OPS-007 read any other" (implementation.yaml:1296-1298), which is unknown for an effectful operation. One error is treated as proof of no effect for the resend and as no proof for the outcome.

**Counterargument.** The protocol has a client resend after a version error, and a server checks the revision before it routes a request to a handler, so in practice no server acts first. Decision 8a examined only -32602 and -32020.

**Least change.** In IFC-046's then: "After an error for the revision it sends the call once more, at a revision that both support, only for an operation whose contract states no `effect`, that is not a control and starts no model work; otherwise the invocation ends unknown." Or, if the owner accepts the revision error as proof that nothing was performed, say so once and have an unresolved revision error report unavailable (not dispatched) in place of unknown.

**Precedent.** The specification's own settlement for -32020 (findings 2-06 and 2-07). HTTP lets a client retry automatically only safe or idempotent methods (RFC 9110, section 9.2.2).

**Who decides.** coordinator. Rulings 37 and 46 point 4: a procedure against a safety promise (OPS-018); narrowing the resend adds no behavior, as 2-06 did. Accepting the revision error as a guarantee instead would be the owner's (finding 2-07).

**Verifier.** confirmed, high. Quote verbatim at spec/interface/implementation.yaml:1284-1286. The resend after a revision error is unconditional in every place that speaks of it: IFC-003 obs (implementation.yaml:79-81 "is sent again at a revision both support, under IFC-046, and is one invocation, not two"), IFC-010 then (229-230), OPS-007 when (spec/conventions/operations.yaml:115-116). The same promise reads the same error as proof of nothing when it is not resolved: "that error is read as IFC-010 and OPS-007 read any other" (1297-1298), which is unknown for an effectful operation, and it says of -32020 "gives no guarantee that a plain server has not acted, so an effectful call is never repeated after that code" (1288-1289). No text in spec/ gives the revision error a stronger guarantee: OPS-007 says "nothing in this specification binds a plain server" (operations.yaml:132-133). Not recorded: decision 8a and findings 2-06 and 2-07 cover only -32602 and -32020 (docs/proposals/first-build/increment-2/decisions.md:22,26; docs/proposals/first-build/README.md:80-81; to-review-with-the-owner.md:15). docs/AUDIT.md:232 lists only "IFC-046 wire resends versus repeats" as an S1 boundary to test, and conformance/s1-safety-cases.md:72-74 restates the present rule. So known-new-situation is right.

**Verifier's corrections.** Class and unsafe mark stand, but the mark rests on the specification's own standard from decision 8a, not on an observed server: no check like work order 018's was made for the version error. That check would settle how likely the harm is. Broader than stated: IFC-046's when covers Conduit-aware implementations too, and no promise binds one to answer the revision error only before acting, as IFC-008 and IFC-047 do for -32021; IMP-007's observable simply asserts it (spec/conventions/implicit.yaml:71-72 "A call that the implementation rejected before acting on it"). The least change must also edit IFC-003's observable (79-81) and IMP-007's sentence. Its cost is small: the engine meets the revision on harmless reads before the first call (tools/list under IFC-001, server/discover under IFC-037). Coordinator decides, by the precedent of 2-06, logged by extending review-list item 2.


## FR-034

**An operation that the Conduit lists and the target cannot serve reports refused, unavailable or invalid, depending on which promise is read**

L04/F2 · contradiction · verifier: weakened, medium · decides: coordinator · new

**Promises.** IFC-033, IFC-012, NAM-026, TPL-013, ACC-001, ACC-003, ACC-056

**Where.** spec/interface/implementation.yaml:959

**Quote.** "No tool is presented for that operation, and a call that names it is answered as IFC-012 answers a call to a tool that is not presented: it reports refused."

**Situation.** A later release adds an operation to a capability (NAM-024). A Conduit lists it; its target holds the earlier release. The consumer calls the tool by name. (1) IFC-033 and IFC-012 say refused under ACC-001, as "a request that no open Conduit permits" (implementation.yaml:317). But an open Conduit does list the operation, and ACC-003 says it "permits exactly the operations it lists" (spec/conventions/access.yaml:63), so ACC-001's when is not met and ACC-056's tier-1 cause, "No open Conduit from the caller to the target as named lists the operation" (access.yaml:941), is false. (2) NAM-026's then covers the same request, an operation "that the Conduit lists and that the capability's contract, in the version the target holds, does not include", and says unavailable (spec/promises/naming.yaml:457-461); only its observable sends the no-contract case to refused. (3) The twin case of a control the target does not have reports invalid under TPL-013, which IFC-012 calls "the one exception" (implementation.yaml:320). Three builders can ship three reasons for one class of fault, and the refused one tells the caller, and the record, a cause that is untrue.

**Counterargument.** The engine presents no tool for the operation, so a well-behaved client never sends the call; refused is the safest reason to report, and OPS-023 puts it first.

**Least change.** One reason for the class. In IFC-033 and in NAM-026's observable replace "reports refused" by "is a request through that Conduit and reports unavailable under NAM-026, with a diagnostic naming the operation and the version the target holds, unless OPS-023 gives another reason first"; in IFC-012 name it as a second exception beside TPL-013's. (Or invalid for all three; either way one rule.)

**Precedent.** HTTP separates 403 Forbidden from 404 and 501; the Kubernetes API answers a kind that the server's version does not serve with 404, not 403. The specification's own TPL-013 and NAM-023.

**Who decides.** coordinator. Rulings 37 and 46 point 4: two promises that cannot both hold; no behavior is added. It changes a reported reason, so it goes on the review list for the owner to strike.

**Verifier.** weakened, medium. Quote verbatim at implementation.yaml:959-961. Real: NAM-026's when covers the reviewer's situation and its then says unavailable (spec/promises/naming.yaml:457-461), while its own observable (464-467), IFC-033's observable (957-961) and IFC-012's observable (291-292, 316-319) say refused. The refusal is given "under ACC-001", whose when is "no open Conduit from that consumer to that target lists the operation" (spec/conventions/access.yaml:29-30), which is false here; ACC-003 says the Conduit "permits exactly the operations it lists" (access.yaml:63). ACC-056's causes would tell the caller an untrue or empty reason: "No open Conduit from the caller to the target as named lists the operation" (941) or "Presentation gives no extra permission (ACC-001, IFC-012)" (980). Not recorded: NAM-026 appears only as a static row in experiments/012-model-checking/G5.md:104.

**Verifier's corrections.** Smaller than stated: builders diverge two ways, not three. Invalid is not a reading open to a builder for an operation that is not a control: TPL-013's when is a control only (spec/promises/templates.yaml:252-254) and IFC-012 calls it "the one exception" (implementation.yaml:319-322). Three observables agree on refused, so the conflict is NAM-026's then against its own observable, plus a cause that is untrue (ruling 52). Class contradiction stands; not unsafe. Either fix works: the reviewer's (unavailable for the class), or narrowing NAM-026's when to a target that holds the operation's contract and giving IFC-033's row in ACC-056 a true cause. Both change a stated reason or cause, so coordinator decides and logs it on the review list. See also extra 1: the neighbouring case presents a tool that can never succeed.


## FR-035

**The cause that a Conduit-aware implementation gives for a failed or unknown outcome has no required path to the caller**

L04/F3 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** IFC-050, IFC-025, IFC-009, PND-010, PND-013

**Where.** spec/interface/implementation.yaml:1344

**Quote.** "An engine-decided failed or unknown outcome carries a diagnostic naming its cause. An implementation's own content is passed under IFC-025."

**Situation.** IFC-050's then requires every result to say "the cause in words". For an outcome the implementation decided, IFC-050 points to IFC-025. IFC-025 passes content on only "Where its outcome came from a result of an implementation marked `plain`" (implementation.yaml:673-675); for any other implementation what the engine puts in content is DL12, each engine's choice (spec/implementation-defined.yaml:89-93). IFC-009 gives an implementation no member in which to state a cause: a failed result "need not hold the operation's outputs" (implementation.yaml:210). So S2's Conduit-aware Linear server (packages/scenario-s2-linear/conduit.yaml) answers failed with "issue not found" in its content, and an engine that keeps every promise may hand the Agent only the text "failed". A plain server's explanation is passed and a Conduit-aware one's may be dropped, which is backwards. PND-010 and PND-013 already assume the path exists: a hook "fails with a diagnostic naming PND-010 and why it could not" (spec/promises/pending.yaml:191) and the engine reports "the hook's cause" (pending.yaml:263).

**Counterargument.** DL12 lets an engine pass the content, and a contract may declare an output named diagnostics, which IFC-025 lets an error result carry. But neither is required, and nothing says how the engine merges an implementation's diagnostics with its own.

**Least change.** In IFC-025's then, replace the plain-only sentence by: "Where its outcome came from an implementation's result whose `isError` is true, or from a result of an implementation marked `plain`, its `content` holds that result's `content`, unchanged, after that block where there is one."

**Precedent.** The Model Context Protocol reports a tool's own errors inside the result's content with isError true, so that the model sees them; gRPC carries a status message beside the code.

**Who decides.** coordinator. Ruling 71 point 4 (a small finding that adds a feature) and ruling 52 (every result says what happened and why). It cannot move S1's counts.

**Verifier.** confirmed, medium. Quote verbatim at implementation.yaml:1344-1346. IFC-025 passes content only "Where its outcome came from a result of an implementation marked `plain`" (673-675); anything more is each engine's choice under DL12 (spec/implementation-defined.yaml:89-93 "what the engine puts in the `content` member of a result it returns to a consumer beyond what IFC-025 fixes"). IFC-009 gives an implementation no member for a cause (210 "That result need not hold the operation's outputs"). IFC-050's then still demands "the cause in words" (1339-1340). PND-010 and PND-013 assume a path exists (spec/promises/pending.yaml:191, 263). No other text in spec/ names an implementation's own cause (grep over spec/ finds only implementation.yaml:1345 and pending.yaml:263). Not in docs/AUDIT.md, the review list, increment 4's log or ruling 52's record. packages/scenario-s2-linear/conduit.yaml:9-20 is a Conduit-aware item, as the reviewer says.

**Verifier's corrections.** Class: strictly DL12 decides it as each engine's choice, so this is less a gap than a result that ruling 52 calls terse; keep it as a finding under ruling 52. The least change is too broad as written: it must not pass a hook's text where IFC-049 withholds it ("No text from the hook is relayed", implementation.yaml:1371) or what PND-004 withholds, and it should say what a failed part of set-up.v1 or sign-in.v1 carries. Limit it to the result of the operation's own implementation. Coordinator under ruling 71 point 4 is right; it cannot move S1's counts (docs/proposals/ruling-71/README.md:42-44).


## FR-036

**No promise decides what a call reports when no JSON-RPC answer comes back: a refused connection, a failed certificate, an HTTP status or a redirect**

L04/F4 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** IFC-010, IFC-017, IFC-041, OPS-007, PRV-025, SEC-013

**Where.** spec/interface/implementation.yaml:473

**Quote.** "A plain item reached by `remote` is sent no proof and none of Conduit's keys, under IFC-007, and is reached as IFC-041 says."

**Situation.** (a) A plain `remote` server is down, its DNS name fails or its certificate has expired. For a run and for a non-plain `remote` item IFC-017 decides: "nothing is dispatched" and the request reports unavailable (implementation.yaml:468-470). For a plain item nothing does. One builder reports unavailable, since no byte was sent; another reads IFC-010's "no result at all, is unknown" (implementation.yaml:230), and every call during an outage, reads included, becomes an invocation to reconcile by hand. (b) The server, or a proxy before it, answers HTTP 401, 429 or 503 with no JSON-RPC body. IFC-010 defines an error as isError true or "A protocol error is a JSON-RPC error" (implementation.yaml:233), so a status is either "an error" (failed for a read) or "no result at all" (unknown). (c) A run's gate rejects the engine's token with 401. PRV-025 says such a request "reaches nothing" (spec/promises/providers.yaml:829-831), yet OPS-007 reads "No result arrives" as unknown. (d) IFC-017 forbids following a redirect only "from either" a run or a non-plain item (implementation.yaml:453); for a plain item SEC-013 keeps the secret header back (spec/promises/secrets.yaml:350) but nothing says whether the engine follows, with the caller's arguments, or what it reports.

**Counterargument.** A careful builder reads every such case as no result, so unknown, which is safe. Standard HTTPS already implies the certificate check.

**Least change.** Add to IFC-017's then: "Where no connection to an item can be made, or its certificate cannot be verified, plain or not, nothing is dispatched, and the request or the part reports unavailable. The engine follows no HTTP redirect from any item. An HTTP response that carries no JSON-RPC message is no result, except a 401 or 403 that a gate of PRV-025 gives, which reports unavailable."

**Precedent.** IFC-017's own rule for runs and non-plain items. gRPC reports UNAVAILABLE where no connection was made and lets a client retry, since the request was never sent.

**Who decides.** coordinator. Ruling 71 point 4: a small gap closed in line with IFC-017; the reading of HTTP statuses is logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at implementation.yaml:473-474. IFC-017's then binds only "a run's `url`, and ... an item reached by `remote` that is not `plain`" (450-453) and forbids redirects only "from either" (453). The definition of a connection makes a `remote` connection the engine's holding of header values, so a setup can succeed with the server down (spec/definitions.md:289). IFC-041 gives unavailable only for the loopback condition (1115-1119). SEC-013 keeps the secret header from a redirect (spec/promises/secrets.yaml:350-351) and says nothing of following one. PRV-025 says a request without proof "reaches nothing" and is answered 401 (spec/promises/providers.yaml:829-831) but gives no reason or outcome for the engine's own rejected request. Not in docs/AUDIT.md, cold-readers.md, IN_FORCE.md, the logs, G5.md or spec/README.md's limits (grep for redirect, HTTP status, 503, 429).

**Verifier's corrections.** Part (b) is mostly decided: "A protocol error is a JSON-RPC error" (233), so a bare HTTP status is "no result at all" and unknown (230; OPS-007, spec/conventions/operations.yaml:111). A failed certificate can be read from IFC-017's general sentence "Where the engine cannot verify who answers, nothing is dispatched" (468-470). What stays undecided: a plain `remote` item to which no connection can be made, a redirect from a plain item, and a gate's 401 to the engine's own request. In the least change, reading a gate's 401 or 403 as unavailable and forbidding every redirect are small behavior, so ruling 71 point 4 with a log entry is the right basis, as the reviewer says.


## FR-037

**Whether the engine reads an implementation's tools/list over stdio, and whether it reads it again, is open, so a missing tool is unavailable under one runtime provider and unknown under another**

L04/F5 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** IFC-001, IFC-037, IFC-046, IFC-010, IFC-017, PKG-020

**Where.** spec/interface/implementation.yaml:46

**Quote.** "Where that list cannot be read, or does not hold the tool that a call names, the engine cannot keep that requirement, so nothing is sent: the request, the hook or the part that needed the call is not dispatched and reports unavailable, and its record says why, as under IFC-037."

**Situation.** A plain package's `tools` map misspells a tool name; PKG-020 cannot check that when the package is installed. Reading 1: the sentence before ties the read to the header rule, "Over Streamable HTTP the engine mirrors ... so it reads" (implementation.yaml:42-45), so on the engine's own runner (stdio) the list is not read, the call is sent, the server answers unknown tool, and IFC-010 gives failed for a read and unknown for an effectful operation or a turn, each to be reconciled. Reading 2: the quoted sentence is general, the list is always read, and the request reports unavailable with a record that names the missing tool. Under a container runtime provider (Streamable HTTP) both readings give unavailable. So the same documents report differently when the runtime provider is swapped, against "The promises of this file hold whatever runtime provider runs the item" (implementation.yaml:489-490) and scenario S10's claim. Second divergence: the list is read "before its first call to each run", and a `remote` connection can last months. One builder keeps the first list, so a tool the server adds later stays unavailable; another reads again.

**Counterargument.** Over stdio there are no headers to mirror, so the read is not needed there, and the protocol leaves a client free to list tools when it likes.

**Least change.** Make it one obligation of its own: "Before its first call to each run, on either transport, and to an item reached by `remote` at each connection, the engine reads the implementation's `tools/list`; it reads it again when the implementation notifies that its list changed, and before it reports that the list does not hold a tool."

**Precedent.** Clients of the Model Context Protocol list tools before calling and list again on notifications/tools/list_changed; the kubelet checks a container runtime before it registers a Node, IFC-037's own analogy.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity; the fix adds no behavior beyond what one reading already has.

**Verifier.** confirmed, medium. Quote verbatim at implementation.yaml:46-49. The sentence before it scopes the read to one transport and one purpose: "Over Streamable HTTP the engine mirrors into `Mcp-Param` headers ... so it reads the implementation's `tools/list` before its first call to each run" (42-45), and "that requirement" (47) is the header mirroring, which stdio lacks; yet "each run" includes the engine's own runner, which is stdio (448-449). IFC-010 presupposes that a call can be sent for a tool the server lacks: "an unknown tool is one case of it" (233-235), read as failed or unknown. PKG-020 checks only the map's keys (spec/promises/packages.yaml:206-209). So the two readings give unavailable against failed or unknown, against "The promises of this file hold whatever runtime provider runs the item" (489-490). Round 5's M61 settled the read, not its transport (docs/proposals/reaudit/round-5/DECISIONS.md:128). Not in the known-open records.

**Verifier's corrections.** Class and who decides stand. One part of the least change adds behavior: reading the list again "when the implementation notifies that its list changed" has the engine act on an implementation's notification, which no reading has today; that part is ruling 71 point 4, or is dropped. The second divergence (keeping the first list over a long `remote` connection) is minor: IFC-046 already reads again after -32020 (1282-1283), and only discovery has "may read it again at any time" (1026).


## FR-038

**IFC-012 and IFC-013 name different deciding promises for a call to a closed Conduit's tool, and the ruleId would then disclose what IFC-012 says tier 1 hides**

L04/F6 · contradiction · verifier: confirmed, medium · decides: coordinator · new

**Promises.** IFC-012, IFC-013, IFC-050, ACC-001, ACC-011, ACC-056, REC-001

**Where.** spec/interface/implementation.yaml:344

**Quote.** "A call to a tool that is no longer presented reports refused under ACC-011, and is answered as IFC-012 states."

**Situation.** An Agent calls a tool whose Conduit was closed a minute ago. IFC-012 says it "reports refused under ACC-001, whether the name was never presented or its Conduit has been closed" and that "Tier 1 discloses no difference between a name never presented and a tool whose Conduit closed" (implementation.yaml:318-319, 331-332). IFC-013 says ACC-011. ACC-056 makes the ruleId "the refusing promise" and gives ACC-011 the cause "A request needs an open Conduit, regardless of earlier permission" (spec/conventions/access.yaml:915, 946); its table also has rows for IFC-012, IFC-013 and IFC-033 with causes of their own (access.yaml:969-970, 980). IFC-050 makes the ruleId observable (implementation.yaml:1349) and REC-001 records it. A builder may emit ACC-001, ACC-011, IFC-012 or IFC-013; with ACC-011 or IFC-013 the result tells a never-presented name from a closed one, which IFC-012 forbids. The same doubt holds for an implementation's refusal, where ACC-008, IFC-019, IFC-026 and IFC-027 each have a row (access.yaml:945, 972, 976-977). Two engines fail each other's vectors on a field the specification calls observable.

**Counterargument.** ACC-056 says "the ruleId remains the deciding rule", and ACC-011's observable says a request through a closed Conduit "is decided under ACC-001", so a careful reader lands on ACC-001. The consumer knows its own Conduit was closed, so the disclosure is slight.

**Least change.** IFC-013: "reports refused under ACC-001, as IFC-012 states." In ACC-056's table mark the rows of IFC-012, IFC-013, IFC-033, IFC-019, IFC-026 and IFC-027 as restating rows, each naming the one access promise whose ID is the ruleId (ACC-001 for the first three, ACC-008 for the rest).

**Precedent.** SARIF gives each result one ruleId that names one rule. GitHub's API answers 404 alike for a resource that is hidden and one that is absent.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction; no behavior is added.

**Verifier.** confirmed, medium. Quote verbatim at implementation.yaml:344-345. IFC-012 says the same call "reports refused under ACC-001, whether the name was never presented or its Conduit has been closed" (317-319) and "Tier 1 discloses no difference between a name never presented and a tool whose Conduit closed" (331-332). ACC-056's then makes the ruleId "the refusing promise" with "the cause in the refusing rule's row below" (spec/conventions/access.yaml:915-918), and its table has rows with causes of their own for ACC-011, IFC-012, IFC-013 and IFC-033 (946, 969-970, 980). The ruleId is observable (implementation.yaml:1349) and recorded (spec/conventions/records.yaml:33-34). No vector pins a ruleId of ACC-001 or ACC-011 (grep over conformance/), and G5.md:228 has no finding on ACC-011.

**Verifier's corrections.** The text leans to ACC-001, which lowers the practical divergence but leaves the literal conflict: ACC-011's observable says such a request "is decided under ACC-001 from the Conduits that are open" (access.yaml:191-192) and ACC-056's observable says "Promises that restate a refusal reuse the deciding access rule's cause; the ruleId remains the deciding rule" (935-936). That sentence is itself at odds with the table, where restating rows carry different causes. The reviewer's fix is right and adds no behavior; coordinator. Not unsafe: the consumer was shown the tool before the close.


## FR-039

**IFC-049 binds requestState to a "session", a term no definition gives and that the baseline transport is said not to have**

L04/F7 · defect · verifier: confirmed, medium · decides: coordinator · new

**Promises.** IFC-049, PND-010, PND-013, IFC-022, SEC-013

**Where.** spec/interface/implementation.yaml:1378

**Quote.** "The engine correlates retries through its own integrity-protected requestState, an opaque handle bound to the authenticated Interface, session and originating invocation, expiring with DL18."

**Situation.** A person starts sign-in.v1 from a laptop and spends two minutes in the browser; the client reconnects and retries with the requestState. "Session" is in no row of spec/definitions.md; the connection row says that in the baseline revision "that transport keeps no session" (definitions.md:289), SEC-013 says the same (spec/promises/secrets.yaml:347-348), and IFC-022 establishes the instance "for that request alone and not from an earlier request" (implementation.yaml:609). Reading 1: session means the transport connection, so the retry on a new connection is rejected as "another principal" or tampering, and the sign-in ends unknown at DL18. Reading 2: nothing checkable is meant and the binding is to the Interface and the invocation alone. PND-010 and PND-013 depend on the same word ("in the session and invocation bound by requestState", pending.yaml:195). The task path in the same promise carries "no requestState" (implementation.yaml:1393) and so has no session binding at all, which shows it is not needed. The sentence also calls requestState "an opaque handle", though handle has one meaning under IFC-011.

**Counterargument.** A reader may take session as the protocol's own word for whatever the token or the client holds; the single use of each state and the binding to the Interface already stop replay.

**Least change.** Strike the word in IFC-049 (twice) and in PND-010: "an opaque value bound to the Interface that the request establishes under IFC-022 and to the originating invocation".

**Precedent.** The protocol's own requestState at revision 2026-07-28: opaque to the client, integrity-protected and bound to the authenticated principal. OAuth's `state`.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a term the text cannot be followed with; removing it adds no behavior.

**Verifier.** confirmed, medium. Quote verbatim at implementation.yaml:1377-1379; second use at 1420 ("in its bound session and invocation"). No row of spec/definitions.md defines session; its only use there says the baseline transport "keeps no session" (definitions.md:289), as SEC-013 does (spec/promises/secrets.yaml:347-348). IFC-022 establishes the instance "for that request alone and not from an earlier request" (608-609). The task path carries "no requestState" (1393). PND-010 depends on the word (spec/promises/pending.yaml:195). spec/README.md's rule 4 requires every term to be defined. Not in the known-open records; rulings 73 and 75 do not use the word for this binding (grep).

**Verifier's corrections.** Add a fourth place: spec/core/operations.yaml:1382 ("in its bound session and invocation"). Reading 1 is live in the first build, which binds transport session IDs to the consumer (engine/CONFORMANCE.md:86-88, not binding), so the divergence is not hypothetical. Striking the word removes a stated binding and not only a term, so the coordinator's fix should be logged for the owner to strike. "Opaque handle" beside IFC-011's meaning of handle is a wording point only.


## FR-040

**A sign-in.v1 request without the URL capability is answered -32021, and no promise says which reason it reports or whether it is recorded**

L04/F8 · ambiguity · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-075.

**Promises.** IFC-049, IFC-036, PND-010, OPS-005, REC-001

**Where.** spec/interface/implementation.yaml:1359

**Quote.** "For sign-in.v1 the request must declare {"elicitation": {"url": {}}} in _meta.io.modelcontextprotocol/clientCapabilities; without it the engine answers -32021 with data.requiredCapabilities naming URL elicitation, sending no elicitation."

**Situation.** Root's console calls sign-in.v1 and does not declare URL elicitation. Every request ends in one of seven ways (spec/definitions.md:229), and REC-001 has every request found "with how it ended" (spec/conventions/records.yaml:25-26). IFC-036 lists the requests that the engine rejects with a protocol error; it has the -32021 for Conduit's extension, which "reports invalid: it is recorded so" (implementation.yaml:991-992), and not this one. PND-010 says "Missing URL capability uses -32021 with requiredCapabilities, not invalid" (spec/promises/pending.yaml:189). Reading 1: it is negotiation, like the revision error that IFC-036 calls "not a request that ended" (implementation.yaml:1006): no reason, no record. Reading 2: it is one more case of IFC-036: reason invalid in the error's data, recorded. A third builder reports unavailable. The record, the ruleId and the error's data differ. It is also unsaid whether the check comes before or after the access decision of OPS-023, so a caller with no Conduit may get -32021 or refused.

**Counterargument.** The wire answer is fixed, -32021 with requiredCapabilities, and a client that follows the protocol declares the capability and sends again, so the difference shows only in records.

**Least change.** Add the case to IFC-036's when: "or a request for sign-in.v1 that does not declare URL-mode elicitation, under IFC-049"; in PND-010 replace ", not invalid" by ", and reports invalid under IFC-036, once the access decision has permitted it".

**Precedent.** The specification's own IFC-036, which reads the -32021 for Conduit's extension so.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity between two promises; no behavior is added.

**Verifier.** confirmed, low. Quote verbatim at implementation.yaml:1359-1361. PND-010 says "Missing URL capability uses -32021 with requiredCapabilities, not invalid" (spec/promises/pending.yaml:189); the core contract says only "Missing URL capability returns -32021" (spec/core/operations.yaml:1375-1376). OPS-005 requires a request that is not dispatched to report "exactly one of: refused, invalid, conflict or unavailable" (spec/conventions/operations.yaml:70-72), and REC-001 has every request found "with how it ended" (spec/conventions/records.yaml:25-26). IFC-036's when lists its cases and not this one (979-987); its -32021 case reports invalid (990-993), and the revision error is "not a request that ended" (1005-1007). PND-013 states its order explicitly, "after access and before dispatch" (pending.yaml:272); this check states none. Not in the known-open records.

**Verifier's corrections.** The reviewer's least change reverses explicit text: "not invalid" was a deliberate correction (docs/work-orders/038a-spec-pending-and-sign-in.md:40-44, K1). The alternative that keeps it is the negotiation reading, as IFC-036 treats the revision error: no request ended and nothing is recorded. Either way the coordinator settles an ambiguity and logs it. Low because the wire answer is fixed; only the record, the reason in the error's data and the order against the access decision differ.


## FR-041

**A consumer's request with a Conduit key that is not defined for it, or with a malformed value such as an empty idempotency key, is decided by no promise**

L04/F9 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** IFC-023, IFC-020, IFC-036, IFC-049, PND-013, OPS-011

**Where.** spec/interface/implementation.yaml:639

**Quote.** "It supplies an idempotency key, which is a non-empty string, as `io.github.shrav-k.conduit/idempotencyKey` in the request's `_meta`."

**Situation.** (a) A client bug sends `idempotencyKey: ""` (an unset variable) with an effectful request, and sends it again after a timeout. Engine A takes an empty key as no key, so each send is a new invocation under OPS-011 and the effect is applied twice while the client believes it sent a key; engine B reports invalid. Neither breaks a promise. (b) A client echoes a result's `_meta` into its next request, or sends `.../caller` or `.../filters`. IFC-020 says the engine sends its own values "whatever else the onward request states" (implementation.yaml:564), which reads as silent ignoring for an onward request and says nothing for a request made on the caller's own behalf. (c) A `serving` that is not a string. PND-013 decides exactly one such misuse: signInReturn anywhere else "reports invalid ... with a diagnostic naming PND-013 and the key" (spec/promises/pending.yaml:271-273). The general case is open, and there is no one closed list to check against: the row for Conduit's keys names only idempotencyKey and serving on a request to the engine (spec/definitions.md:296), while IFC-049 adds signInLogin and signInReturn (implementation.yaml:1410, 1417) and IFC-025 adds pendingAction and unanswered on a result (implementation.yaml:732-734).

**Counterargument.** The engine never takes a caller, a filter or a configuration from a consumer (IFC-006, IFC-020, IFC-040), so ignoring an extra key is safe, and a malformed key is the client's own fault.

**Least change.** One sentence in IFC-036's when: "or one that carries, under the prefix of Conduit's keys, a key that this specification does not define for a request to the engine, or a value that is not of the form its promise gives", so it reports invalid with a diagnostic naming the key and holding no value. Bring the definitions row for Conduit's keys up to the full list.

**Precedent.** The specification's own PND-013. The IETF draft of Idempotency-Key has a server answer 400 to a malformed key; the Kubernetes API's strict field validation rejects an unknown field instead of dropping it.

**Who decides.** coordinator. Ruling 71 point 4: a small gap closed by an existing rule of the specification (PND-013), in line with ruling 52.

**Verifier.** confirmed, medium. Quote verbatim at implementation.yaml:639-640. No promise says what a request with an empty or non-string idempotency key reports: OPS-011's when is "carrying no idempotency key" (spec/conventions/operations.yaml:264), IFC-036's when lists no malformed Conduit key (979-987), and OPS-033 checks only arguments (operations.yaml:805-807). PND-013 decides one such misuse explicitly (spec/promises/pending.yaml:271-273). The definitions row for Conduit's keys names only `idempotencyKey` and `serving` on a request to the engine and omits signInLogin, signInReturn, pendingAction and unanswered (spec/definitions.md:296; implementation.yaml:1410, 1417, 732-735). Not in the known-open records.

**Verifier's corrections.** Two texts lean to invalid for a malformed value without deciding it: the definition of invalid, "does not satisfy a schema or a rule for what it asks" (definitions.md:237), and record.v1's `idempotencyKey: {type: string, minLength: 1}` (spec/core/results.yaml:194), under which an engine could not record an empty key. Part (b), an undefined key, is the weaker half: ignoring it is safe, since the caller is fixed by IFC-020 and IFC-021 (559-560, 585-586). Rejecting unknown keys is new behavior, so ruling 71 point 4 is the right basis. The lagging definitions row is a plain defect the coordinator fixes at once. Not unsafe: the repeat follows a client's own malformed key.


## FR-042

**A rule that rejects documents sits in the observable of an interface promise, so the Language level does not include it**

L04/F10 · defect · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-056, FR-098, FR-142.

**Promises.** IFC-033, CTR-012, SUB-005, PKG-023

**Where.** spec/interface/implementation.yaml:940

**Quote.** "A document that uses one there is rejected under IFC-033 when its package is installed or when it is submitted, with a diagnostic naming the reserved key and its position in the document."

**Situation.** A contract's schema states, in its root `$defs`, a key of the form `example.org/acme/files@0.3.0#path.v1`. The Language level covers "Every promise whose layer is `language`: documents in, resolved objects or diagnostics out" (spec/README.md:320). IFC-033 has the layer interface, its when is "The engine presents a tool for an operation to a consumer", and the rejection is a sentence of its observable. The file's head lists the document rules that are language, "IFC-014, IFC-043 and IFC-044" (implementation.yaml:6-7), and not this one. An engine that claims the Language level, or an independent loader built from the language promises, accepts the document; an engine at the Engine level rejects it. Two conforming engines disagree on whether one document is valid, which is what the language layer exists to prevent. Finding 2-02 settled the rule; its layer and its place were not.

**Counterargument.** Every engine that presents tools is at the Engine level and applies IFC-033 whole, so in practice the document is rejected wherever it could matter.

**Least change.** Move the sentence into a promise of its own in spec/promises/contracts.yaml, layer language: when "A schema that an author writes states, in the `$defs` of its root, a key in the form IFC-033 gives a contract's copy"; then "The document is rejected." IFC-033 keeps one clause that cites it.

**Precedent.** The file's own IFC-014, IFC-043 and IFC-044; JSON Schema reserves its `$`-prefixed keywords in the schema language, not in a consumer of it.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect of layer and form; the rule itself is unchanged.

**Verifier.** confirmed, low. Quote verbatim at implementation.yaml:940-942. IFC-033 has `layer: interface` (892) and the when "The engine presents a tool for an operation to a consumer" (895). The file's head names only "IFC-014, IFC-043 and IFC-044, rules on documents" as language (6-7). spec/README.md:270 says "A promise's own `layer` field decides" and :320 defines the Language level as "Every promise whose layer is `language`". No language promise restates the rule (grep for reserved over spec/promises and spec/schema-profile.md). The first build already treats it as a document rule: spec/schemas/examples/invalid-by-rule/IFC-033-reserved-root-defs.yaml. Finding 2-02 settled the rule itself (docs/proposals/first-build/README.md:76; docs/AUDIT.md:227), not its layer.

**Verifier's corrections.** None of substance. Low because the document that triggers it is rare (a root `$defs` key shaped like a qualified reference with a version). Class defect and coordinator stand.


## FR-043

**An operation that has no effect is given the outcome unknown, which puts reads on the reconciliation list and blocks their repeats**

L04/F11 · needless-complexity · verifier: confirmed, medium · decides: owner · new

Same fault, or overlapping: FR-021.

**Promises.** IFC-010, IFC-016, IFC-045, OPS-007, OPS-012, OPS-013, PND-005

**Where.** spec/interface/implementation.yaml:228

**Quote.** "A result whose `resultType` is `input_required` or is a value the engine does not recognize, any other error but one after which the engine sends the call once more under IFC-046, and no result at all, is unknown."

**Situation.** An Agent lists issues through a plain server and the call times out, or the server asks for a URL sign-in. The operation states no effect, is not a control and starts no model work. IFC-010 itself reasons that such an operation "does nothing that the error could leave half done" (implementation.yaml:238-239) and gives failed for any error, yet gives unknown for no result and for input_required. Unknown means that it "cannot be confirmed that the effect occurred or that it did not" (spec/definitions.md:235); here there is no effect to confirm. The cost is real: the invocation stays unknown "until it is reconciled" (OPS-013), it is listed in `unreconciled`, which "no purge shortens" (definitions.md:223), an actor with authority over the owning Pool must record whether a read "occurred", and a repeat with the same key is never dispatched, because a plain server never counts as deduplicating (OPS-012, PKG-020). A Conduit-aware implementation's JSON-RPC error on a read is unknown too (OPS-007) where a plain server's is failed. IFC-045 adds to it: the engine declares URL elicitation on every call to an instance whose Type states signIn, a capability it honors only inside sign-in.v1, so a lapsed login turns each read into an unknown.

**Counterargument.** Unknown is always safe, and one rule for every operation is simpler to state. IN_FORCE.md:196 records the ruled choice that a mid-call input_required ends unknown (ruling 41 item 27, ruling 49), and decision 8a fixed the table.

**Least change.** In IFC-010 and OPS-007: "For an operation whose contract states no `effect`, that is not a control and starts no model work, an error, an input request that is not answered and no result at all are failed." A repeat is then dispatched under OPS-024, and PND-005 still raises its action.

**Precedent.** HTTP's safe methods (RFC 9110, section 9.2.1): a GET that fails or times out is simply sent again, and nobody reconciles it. gRPC's retry policy for idempotent calls.

**Who decides.** owner. It changes the outcome table that decision 8a and ruling 41 (item 27) fixed, so it is a change of behavior for the owner; AGENTS.md, The owner makes the rulings.

**Verifier.** confirmed, medium. Quote verbatim at implementation.yaml:228-230. IFC-010's own reason for failed is that the operation "does nothing that the error could leave half done" (238-239); unknown means "It cannot be confirmed that the effect occurred or that it did not" (spec/definitions.md:235). OPS-013 keeps it unknown "until it is reconciled" (spec/conventions/operations.yaml:299); OPS-012 dispatches a repeat only for `deduplicates: true` and not plain (operations.yaml:281-283). The asymmetry holds: OPS-007's first case, "No result arrives from an implementation that carries Conduit's keys" with only -32021 and IFC-046 excepted (operations.yaml:111-116), makes a Conduit-aware read's JSON-RPC error unknown where a plain one's is failed (130-136). IFC-045 declares URL elicitation on every call to an instance whose Type states signIn (1253-1254), and IFC-016 ends such an elicitation unknown (429-431). Not in the known-open records.

**Verifier's corrections.** Citation: "no purge shortens" is at spec/definitions.md:370 (the `unreconciled` row), not :223. Half of it is ruled: IN_FORCE.md:196 holds "mid-call input_required ends unknown under IFC-016 (41 item 27, 49)", so that part goes to the owner as a question on a ruling, as the reviewer's "decides: owner" allows. "Blocks their repeats" holds only for a repeat with the same idempotency key; a client that sends no key makes a new invocation under OPS-011 (operations.yaml:264-267) and leaves an unreconciled entry behind. Class and owner stand.


## FR-044

**Most of the file's obligations, several wire keys and whole rules live in `observable` fields, so what binds, at which level and for whom cannot be told**

L04/F12 · defect · verifier: weakened, medium · decides: coordinator · new

**Promises.** IFC-001, IFC-003, IFC-011, IFC-016, IFC-023, IFC-025, IFC-033, IFC-041, IFC-049

**Where.** spec/interface/implementation.yaml:732

**Quote.** "io.github.shrav-k.conduit/pendingAction names the action IDs raised or holding the request (PND-005, PND-007). For an Interface's own result io.github.shrav-k.conduit/unanswered is the nonzero count under PND-002; it is absent for Agents."

**Situation.** The form says `then` is "The single obligation" and `observable` is "What can be checked" (spec/README.md:291-292). In this file the observables hold 9,412 of 13,646 words (my count over the 46 promises, with /usr/bin/python3). Examples a builder must treat as binding though they are not in any then: two result keys defined only in IFC-025's observable (quoted); the tools/list read and its unavailable rule in IFC-001's observable (implementation.yaml:42-49); the rule for every URL elicitation outside sign-in in the observable of IFC-016, whose when is "One invocation outlasts its call" (implementation.yaml:428-433); requestState, a dozen URL checks and two request keys in IFC-049's observable (implementation.yaml:1377-1421). Level stops meaning anything: IFC-003 and IFC-023 are `may`, yet their observables bind the engine to resend and to honor the key; IFC-016 is `must` and its then says the tasks extension "may be used". IFC-049's then alone holds at least eight obligations, IFC-041's three. IFC-011 mixes an engine's duty with an implementation's, so it fits neither the Engine nor the Implementation level whole. Two builders will differ on which observable sentences they implement, and no vector can cover one such promise: all 46 have `vectors: []`.

**Counterargument.** The observables grew from audit rounds that closed real gaps; keeping each explanation beside its promise helps a reader, and the content is mostly right.

**Least change.** Split only where an observable adds an obligation: the URL-elicitation rule out of IFC-016 (PND-005 already states the convention); pendingAction and unanswered into their own promise and the definitions row; the tools/list read out of IFC-001; IFC-049 into the wire form of a URL action, with behavior left to PND-010, PND-013 and ENR-003. Then state in spec/README.md that an observable adds no obligation.

**Precedent.** RFC 2119 practice of one requirement per normative sentence; a Kubernetes conformance test names the one behavior it tests.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect of form; moving text between fields and promises adds no behavior.

**Verifier.** weakened, medium. Quote verbatim at implementation.yaml:732-735. Recount with /usr/bin/python3 at this commit: 46 promises; when 974, then 3,260, observable 9,412 of 13,646 words (69%). Obligations in observables hold as cited: the tools/list read (42-49), the URL-elicitation rule under a when about tasks (428-433), requestState and the URL checks (1377-1409). IFC-003 and IFC-023 are `may` (70, 635) with observables that bind the engine (79-81, 642-643); IFC-016 is `must` with "may be used" (392-393). spec/README.md:291-292 gives then as "The single obligation" and rule 1 as "One obligation per promise". The brief asks for this check (docs/guides/full-review.md:116); no record lists it as open.

**Verifier's corrections.** Two supporting facts are wrong. (1) "No vector can cover one such promise" and the assessment's "none of the 46 has a vector": the `vectors` field is empty in 517 of 519 promises across spec/, and coverage is kept in conformance/manifest.yaml, whose cases name 23 of these 46 promises (IFC-001, 007 to 014, 020 to 022, 025 to 028, 032, 033, 036, 038, 041, 046, 047); spec/README.md:250 says "46 promises; vectors test slices of 23 promises". The stale field is its own, known point (README.md:193-195; IN_FORCE topic 13). (2) The two keys are not "defined only in IFC-025's observable": pendingAction is in PND-005's then (spec/promises/pending.yaml:148-150), and unanswered in PND-002's observable (pending.yaml:36-38). What remains true: unanswered sits only in observables, and pendingAction's list form only in IFC-025's (736-737). Class defect and coordinator stand; splitting makes new IDs, so it is an order of its own.


## FR-045

**The handle concept asks nothing of the engine that a string does not, and no shipped contract uses it**

L04/F13 · needless-complexity · verifier: confirmed, low · decides: owner · new

**Promises.** IFC-011, CTR-002, IFC-033, IFC-010

**Where.** spec/interface/implementation.yaml:269

**Quote.** "On the wire it is an opaque string that the implementation issued. The engine passes it back unchanged and never interprets it. A handle permits nothing, and the implementation checks it on every call."

**Situation.** For a property whose schema is handle.v1 the engine does what it does for any string: it validates `type: string, minLength: 1` (spec/core/results.yaml:115-117) and passes it on. The concept costs a promise (IFC-011), a definitions row, clauses of CTR-002 (top-level only; a handle with a default is rejected, spec/promises/contracts.yaml:40-41, 68-69), a core schema, a mention in IFC-033 and a row of words not to use. No Operation contract in spec/core/ or packages/ refers to handle.v1; the only uses are a schema example and one conformance fixture (grep over the tree). What IFC-011 adds is unobservable by the engine ("the implementation checks it on every call") or untrue in one case: its observable says an unrecognized handle "has the outcome failed" (implementation.yaml:273), but for a plain implementation's effectful operation IFC-010 gives unknown for every error and says the outcome "follows that table and nothing else" (implementation.yaml:249-250).

**Counterargument.** The name tells a contract's reader that a string is a live reference and not data, and the protocol's guidance for stateful tools recommends such identifiers. Ruling 31 (I4) accepted it, per IN_FORCE.md:239.

**Least change.** Remove IFC-011, the handle clauses of CTR-002 and the definitions row; keep handle.v1 only as an ordinary core Schema, or drop it. A contract that issues an opaque identifier writes a string and states its own checks.

**Precedent.** The protocol's guidance for stateful tools is itself only an opaque identifier passed as an ordinary string argument; Kubernetes passes its `continue` token as a plain string with no type of its own.

**Who decides.** owner. It removes a concept that a ruling holds (ruling 31, I4), which is the owner's: a fix that removes a feature.

**Verifier.** confirmed, low. Quote verbatim at implementation.yaml:269-271. handle.v1 is `type: string, minLength: 1` (spec/core/results.yaml:116-117). git grep at this commit finds handle.v1 used as a property's schema only in spec/schemas/examples/valid/files-package/contracts.yaml and two fixtures (conformance/interface/schemas/e05-handle, e05-secret-omitted); spec/core/conduit.yaml:103 only exports it; no Operation contract in spec/core/ or packages/ refers to it. Its machinery: IFC-011, CTR-002's clauses (spec/promises/contracts.yaml:40-41, 68-69), spec/schemas/contracts.yaml:334, definitions.md:301 and :429. The embedded contradiction holds: "has the outcome failed" (273-274) against IFC-010's unknown for any error of a plain effectful operation and "follows that table and nothing else" (229-230, 249-250).

**Verifier's corrections.** Owner is right: IN_FORCE.md:239 holds handles (24 PR7, 31 I4 and I5), and ruling 41 P1 names `handle.v1` (docs/proposals/ruling-41/README.md:58). Split the embedded contradiction out: if handles stay, IFC-011's observable against IFC-010 is a contradiction the coordinator may fix with no new behavior (say that a Conduit-aware implementation states failed, and that a plain server's error is read by IFC-010's table).


## FR-046

**Whether the engine passes an implementation's progress and logging notifications on to the consumer that asked is undecided**

L04/F14 · gap · verifier: confirmed, low · decides: coordinator · new

**Promises.** IFC-001, IFC-025, IFC-038, OPS-027

**Where.** spec/interface/implementation.yaml:38

**Quote.** "The interface is the protocol with Conduit's keys added. What the protocol requires of a client and of a server, such as the keys it requires in the `_meta` of every request, what a server must implement beside its tools, and what a tool list must carry, binds the engine and an implementation as the protocol states it."

**Situation.** An Agent's turn runs for hours; order 050a lets a turn deadline reach 604800 seconds (docs/proposals/first-build/log-increment-4.md:23). The consumer's client asks for progress and applies its own timeout to a tool call that reports none. The engine stands between two protocol connections, and the protocol makes progress optional on each, so nothing says whether the engine relays what the implementation sends. Engine A relays and the call survives; engine B does not, the client gives up and closes the stream, which OPS-027 reads as a cancellation with the outcome unknown (spec/conventions/operations.yaml:616-618). The only notifications the file speaks of are the tool-list change (IFC-038) and those of the tasks extension (IFC-016). A relayed message is also content that reaches the consumer outside `structuredContent` and its schema, which IFC-025 otherwise accounts for item by item. DL12 lists what else the engine puts in `content` but not this.

**Counterargument.** The protocol leaves progress to each side, the tasks extension is the intended route for long work, and the build already met its long-call problem with deadlines and a session idle limit (findings 4-12 and 4-20).

**Least change.** Add to DL12: "whether the engine passes on to a consumer that asked for them the progress and logging notifications that an implementation sends for its call". If the owner wants every engine to relay, that is one promise instead.

**Precedent.** The protocol's progress notifications with a progress token; a reverse proxy passes server-sent events through unchanged.

**Who decides.** coordinator. Ruling 71 point 4: naming an engine's choice in the implementation-defined list is a small fix that adds no behavior; requiring the relay would be the owner's.

**Verifier.** confirmed, low. Quote verbatim at implementation.yaml:37-41. grep over spec/ finds no mention of progress or logging notifications, progressToken or notifications/message. The only notifications the file decides are the tool-list change (1059-1061) and notifications/tasks (425-428). DL12 lists what else goes into `content`, and not this (spec/implementation-defined.yaml:89-93). OPS-027 reads a closed stream as a cancellation whose outcome is unknown (spec/conventions/operations.yaml:616-618). Long turns are real: findings 4-12 and 4-20 (docs/proposals/first-build/log-increment-4.md:23, 31), which settled deadlines and a session idle limit, not relay.

**Verifier's corrections.** None on class or who decides. One point strengthens it: a relayed logging message is text from an implementation that reaches a consumer outside IFC-025's accounting, and for a sign-in hook it would run against "No text from the hook is relayed" (implementation.yaml:1371) and PND-004. So the DL12 entry should say that whatever an engine relays stays under those two rules.


## Further points the verifier noticed

Not verified by a second reader.

- **A tool is presented for an operation that the target's version of the capability does not include, and every call to it reports unavailable** (spec/interface/implementation.yaml:283). IFC-012's then presents "exactly the operations that its open Conduits list and for which IFC-033 gives a definition" (283-284). IFC-033 gives one whenever the target holds the operation's contract, even where the capability at the target's version leaves the operation out, the case in which NAM-026 says unavailable (spec/promises/naming.yaml:457-461). The vector conformance/conventions/access/capability-skew/case.yaml shows it: the list holds `skew.read.v1`, and the call to it reports unavailable. In the neighbouring no-contract case no tool is shown at all. An Agent's model is so shown a tool that cannot succeed, with nothing in its description saying so. Class: needless inconsistency, low; fold it into the fix for F2 (present no tool in both cases, or one reason for both). Coordinator, logged.
- **No promise binds a Conduit-aware implementation to answer the protocol's revision error only before it acts, yet IMP-007 states that it did** (spec/conventions/implicit.yaml:71). IMP-007's observable says "A call that the implementation rejected before acting on it, and that the engine sends once more under IFC-046, is part of the same dispatch". IFC-008 and IFC-047 create that guarantee for -32021 (implementation.yaml:185, 1312-1314); nothing creates it for the revision error, and IFC-046's when covers every implementation (1276-1279). This widens F1 beyond plain servers: an effectful call to a Conduit-aware implementation is resent too, with an idempotency key only where the caller gave one. Either narrow the resend as F1 proposes for both kinds, or add the obligation on implementations, which is a small feature (ruling 71 point 4) or the owner's.

## The verifier's notes

Checked at commit 2eb8f258 (2026-11-12), read-only. I read spec/interface/implementation.yaml in full (1,422 lines) and opened each cited promise.

Result: 12 confirmed, 2 weakened (F2, F12), none known, none refuted. All 14 quotes are verbatim at the cited lines.

- **F1** is the only unsafe finding and the mark stands, by the specification's own standard for -32020. No evidence was gathered on whether real servers act before a version error; the check work order 018 made for -32602 and -32020 would settle the likelihood.
- **F2** is smaller than stated: builders diverge between refused and unavailable only, since TPL-013's invalid is a separately decided case.
- **F12**'s main claim holds and my recount matches (9,412 of 13,646 words in observables). Two of its facts are wrong:
  - The reviewer's statement that none of the 46 promises has a vector is false. The `vectors` field is empty almost everywhere in spec/, but conformance/manifest.yaml tests slices of 23 of these promises, as spec/README.md:250 says. Do not carry that sentence of the assessment into the report.
  - pendingAction is stated in PND-005's then, not only in IFC-025's observable.
- **Citation slip in F11**: "no purge shortens" is at spec/definitions.md:370, not :223.
- **Who decides**: I agree with every mark. Three coordinator fixes change or remove stated text and should be logged for the owner to strike: F2 (a reported reason), F7 (striking "session") and F8 (reversing PND-010's "not invalid").

Not checked: the text of the Model Context Protocol itself, so claims about what the protocol requires rest on what spec/ and the repository's records say of it; and DL13's mechanism beyond the lines cited.
