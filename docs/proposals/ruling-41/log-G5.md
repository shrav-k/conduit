# Ruling 41: log of group G5 (the interface)

Date: 2026-11-06. This log is the account of how the editor of group G5 applied
[the plan's section for G5](PLAN.md#g5-interface) to `spec/interface/implementation.yaml` and
`spec/implementation-defined.yaml`. The rulings are the owner's ([ruling 40](../ruling-40/README.md)
and [ruling 41](README.md)); the wording that carries them out is the editor's, and stands
unless the coordinator or the owner strikes it. Decision numbers are those of the
[principles review](../principles-review/README.md), as in the plan.

| | Before | After |
| --- | --- | --- |
| Promises in `spec/interface/implementation.yaml` | 35 | 38 |
| Items of `spec/implementation-defined.yaml` | 15 | 14 |

Counts: 2 promises and 1 item removed; 19 promises rewritten (the plan's 18, and IFC-009,
outside the plan's list, see [Outside the plan](#outside-the-plan)) and 12 items rewritten;
5 promises added. Unchanged: IFC-002, IFC-003, IFC-005, IFC-007, IFC-010, IFC-013, IFC-015,
IFC-020, IFC-023, IFC-026, IFC-027, IFC-030, IFC-031, IFC-036; DL4, DL11.

## Promises

| ID | Action | Decision | What changed |
| --- | --- | --- | --- |
| IFC-024 | Removed | 6c | The tool `retrieve-result.v1` goes. A caller that lost a result repeats its request under OPS-009; nothing replaces the tool. Named in the file's head comment. |
| IFC-034 | Removed | 14 | Filters that oblige a consumer, in a tool's `_meta`, go with `obliges`. Nothing replaces it. Named in the file's head comment. |
| IFC-001 | Rewritten | 25a, 25b | `when`: an implementation reached by `command`, `package` or `remote`. |
| IFC-004 | Rewritten | 18, 16b | `then` adds a tool's `annotations` and the negotiation of extensions: `server/discover`, a declaration on each request in `io.modelcontextprotocol/clientCapabilities` with the extension's `revision` setting, and the error -32021. |
| IFC-006 | Rewritten | 1b, 2, 11, 18 | The request declares Conduit's extension (IFC-037). Keys: `instance`, `operation`, `invocation`, `caller`, `restrictions`, `config` (the instance's configuration), `revision` (its revision); `operationIdentity` where the caller supplied one or for a setup or retire hook (OPS-028); `conduit` where it came through a Conduit. `interfaceVersion` and the OBS-007 sentence go: every request the engine sends has a caller. Observable: `v0` moves to IFC-037; the configuration is as stored, references resolved, a secret only as its reference; the step sentence becomes a hook sentence (TPL-017, TPL-018, ACC-018); a secret read's caller is SEC-011's; a submitted project's contract is `address#name` (SUB-008); a provider call's keys are PRV-005's. |
| IFC-008 | Rewritten | 18 | `when`: a request declares Conduit's extension at a `revision` the implementation does not support. `then`: it performs nothing and answers with the protocol's error -32021, with no tool result. Observable: the outcome is failed, as before. |
| IFC-009 | Rewritten (cut only) | 18 | Outside the plan: the clause "as IFC-008 states for one such result" is cut, since IFC-008 no longer answers with a result. Nothing else changed; the observable was rewrapped. |
| IFC-011 | Rewritten | 16a, 2 | `when`: a handle is a top-level property of `inputSchema` or `outputSchema` whose schema is a `$ref` to `github.com/shrav-k/conduit/core#handle.v1`. "Holding one permits nothing" became "A handle permits nothing", since "hold (a handle)" goes. |
| IFC-012 | Rewritten | 5, 6c, 4b, C51 | `then`: the tools are the operations the open Conduits list and IFC-033 defines, other than `reveal-secret.v1` (SEC-007); a tool name is the Conduit's local name and the export name joined by a dot. Observable: the retrieval-tool, component and length sentences go; a Conduit that lists `reveal-secret.v1` gives no tool for it, and the engine's read goes through it (SEC-010); notifications are IFC-038. |
| IFC-014 | Rewritten | 5, 6c, 10 | `when` drops the length case. Observable drops the IFC-024 and CON-026 sentences and says why no name passes 128 characters. |
| IFC-016 | Rewritten | 6c | `then`: the tasks extension may be used where both sides of a call declare it; `tasks/get` and `tasks/cancel` are decided anew through the invocation's Conduit; it never stands in for the operation identity. Observable: the retained-result sentences go; a caller with no task repeats its request (OPS-032 while it runs, OPS-009 once it succeeded). |
| IFC-017 | Rewritten | 2, 2a | Level `may` became `must`. An item reached by `command` or `package` is run under PRV-007 and reached at its run's `endpoint` over Streamable HTTP, or over stdio by the engine's own runner; a `remote` item over Streamable HTTP at its URL (IFC-041). |
| IFC-018 | Rewritten | 3a, 7, 9, 11, 14, C20 | The list holds every filter that applies: the request's own Conduit's and those carried under ACC-017 or, for a hook, ACC-018. A requirement of the core package is never sent. Each item is `requirement` (a qualified reference, `address#name` for a submitted project's) and its stored `arguments`. Policies, compositions, re-exports, `obliges` and "constituent" go. Observable adds that a secret read is sent only its own Conduit's filters (SEC-011). |
| IFC-019 | Rewritten | 1, 7 | Observable: "a step that has no `conduit`" became a setup or retire hook; the composed sentence goes. |
| IFC-021 | Rewritten | 6c | `when` drops the exception for `retrieve-result.v1`; observable drops the retrieval sentences. |
| IFC-022 | Rewritten | 26a, R40 3e | `then` adds: the mechanism establishes exactly one instance or none, by a cryptographic proof of the sender wherever the request crosses machines; what a request presents says who is asking and nothing more. Observable adds mutual TLS, SPIFFE and the protocol's authorization as mechanisms (DL13) and PRV-018. |
| IFC-025 | Rewritten | 6a, 6b, 6c, 24a | `invocation` is held where the request has an outcome or is a repeat under OPS-032; `state` and `withheld` go; `isError` is true where the request ended with a reason or its outcome is failed or unknown, and in no other case. Observable: an invalid or conflict result carries an error diagnostic whose `ruleId` is the deciding promise; a refused one carries none (OPS-023); the withheld, retrieval and composed sentences go. |
| IFC-028 | Rewritten | 6c | `when` and observable drop the retrieval exception. |
| IFC-029 | Rewritten | 18 | A request to an implementation declares the extension's `revision` for its version; requests to the engine and results are read by prefix. `interfaceVersion` goes. |
| IFC-032 | Rewritten | 6c | Observable drops "the tool of IFC-024 included". |
| IFC-033 | Rewritten | 16a, 16b | The definition is the contract's `inputSchema` and `outputSchema`, each `$ref` to a Schema contract replaced; a control's `required` lists `revision` alone; `readOnlyHint` true only where no `effect`, not a control and no model work; the three other hints passed on as stated. Observable drops the step and IFC-034 sentences. |
| IFC-037 | Added | 18 | Interface, must. The engine reads `server/discover` before its first request, declares the extension `io.github.shrav-k.conduit/interface` at `revision: v0` on each request; where the result lacks it, the request is not dispatched and reports unavailable, and is never sent without the keys. |
| IFC-038 | Added | C51 | Interface, must. The engine's server declares `tools` with `listChanged`, notifies the streams that asked when an instance's list changes, and marks lists `cacheScope: private`. |
| IFC-039 | Added | C52 | Interface, must. Trace context travels only in the `_meta` keys the protocol reserves, as OpenTelemetry's conventions for the protocol give them. |
| IFC-040 | Added | C52 | Interface, must not. The engine takes no caller, restriction or part of an access decision from trace context. |
| IFC-041 | Added | 25c | Interface, must. On connecting to a `remote` item for an instance's setup, the engine sends each header `headers` names, with the value at its place in the configuration; a secret's value read under SEC-013. |

## Items of the implementation-defined list

| ID | Action | Decision | What changed |
| --- | --- | --- | --- |
| DL14 | Removed | 22 | Cancellation is the protocol's (OPS-027). Named in the file's head comment. |
| DL1 | Rewritten | 1 | Subject "Order of a Pool's Resources": only the order of `retire-with-resources.v1` under LIF-021. |
| DL2 | Rewritten | C33 | "which a purge keeps under REC-004" became "which the engine keeps under OPS-008". |
| DL3 | Rewritten | 6b | A repeat reports conflict under OPS-032. |
| DL5 | Rewritten | 8e | Subject "The machines provider's report": how often the engine asks (PRV-014) and what it does while none arrives; what a request reports meanwhile is PLC-023. |
| DL6 | Rewritten | 8 | `local`'s stored facts go. |
| DL7 | Rewritten | 21 | Fetching and schemes go; adds a cache and a mirror, each giving the same bytes under PKG-026. |
| DL8 | Rewritten | 24c, 17 | Adds whether records are exported as `record.v1`; "unregistered" became "unregistered or removed". |
| DL9 | Rewritten | 23 | Subject "Order of failed checks": only the order of failed checks that give one reason. |
| DL10 | Rewritten | 6c | How long the outputs of a succeeded invocation are kept for a repeat under OPS-009. |
| DL12 | Rewritten | 2a | Subject "Protocol revisions and `content`"; the transport, who starts a process and how many instances share one go. |
| DL13 | Rewritten | 26a | The mechanism under IFC-022, exactly one instance or none, by a cryptographic proof across machines; mutual TLS, SPIFFE and the protocol's authorization each one; on one machine over stdio, what the engine's own runner gives the process. |
| DL15 | Rewritten | 30, 5 | CON-003 goes; the Kubernetes API server's generator is cited as one choice that meets it. |

## Choices

| Where | Choice |
| --- | --- |
| IFC-006 | The step sentence is pointed at hooks rather than only cut (plan rule 8): `conduit` is absent for a hook, which carries the restrictions of its setup or retirement under ACC-018. Added pointers to SEC-011 (whose read it is) and PRV-005 (what a provider call carries). "Every request has a caller" is limited to requests the engine sends, so it does not contradict IFC-032's record with no caller. |
| IFC-008 | The outcome stays failed, as before: IFC-008 obliges the implementation to perform nothing, so its -32021 error says nothing was done. The record names the revision declared. |
| IFC-004, IFC-008, IFC-037 | -32021 is described as the error a server gives a request that does not declare a capability it requires, as the second check against the protocol (`https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/mcp-recheck.md`, row 26) reads it. The declaration goes in `io.modelcontextprotocol/clientCapabilities`. |
| IFC-012, IFC-014 | The length sentences go from IFC-012, as the plan says; IFC-014's observable keeps the 128-character statement with its reason (63 + 1 + 63). |
| IFC-016 | `when` kept as it was. The observable says how a caller with no task learns the result, which replaces retrieval (6c). |
| IFC-017 | `when` names the items reached by `command`, `package` or `remote`. Where the engine's own runner runs an item, how it starts the process is the engine's own. |
| IFC-025 | `invocation` names the invocation the request began or repeats. `isError` is said to keep the protocol's meaning. The ACC-034 sentence is kept in its new sense (a result whose Conduit closed holds its outputs). Diagnostics are in the form of `diagnostic.v1`; an error diagnostic is one whose `level` is `error`. |
| IFC-028 | Adds that a hook is an invocation dispatched to the instance, and its onward requests name it. |
| IFC-029 | Requests to the engine and results "are read by the prefix of their keys", rather than "declare no revision", so that a consumer is not forbidden to declare the extension. The extension's name is the prefix followed by `interface`. |
| IFC-033 | A hint the contract does not state, and `readOnlyHint` where it is not true, is left out of `annotations`, so the protocol's default holds. |
| IFC-037 | No extension of Conduit's is declared to a plain implementation. |
| IFC-038 | The notification is named generically ("the protocol's notification that the tool list changed"); the streams notified are those on which a request of the instance asked. |
| IFC-039 | Binds the engine, a consumer and an implementation. The reserved trace keys are not named: no check against the protocol in this repository quotes them. |
| IFC-040 | The observable extends to the instance that IFC-022 establishes, since it is part of every access decision. |
| IFC-041 | The connection is the part of the setup that TPL-012 names; the observable says it serves one instance, that a `remote` with no `headers` is sent none, and points at IFC-030. |
| DL8 | "unregistered" became "unregistered or removed", to match decision 17 (REC-003 is rewritten the same way). |
| Head comments | Each file gains one paragraph naming the removed IDs, where their content went, the changed and added IDs, and this log. The interface file names the removed keys without the prefix, so that `tools/set_names.py`, which reads the keys from that file, does not count them as defined. |

## Outside the plan

IFC-009 is listed as unchanged in the plan, but its observable said that an implementation sets
`isError` "as IFC-008 states for one such result". After IFC-008's rewrite, IFC-008 gives no
result. The clause was cut and nothing else changed. The coordinator confirms or reverts this.

## For the integrator

What cites a removed ID, a removed key or an item whose meaning changed. The promise citations
are as of the commit before the editors started (`7cd6e2c`); other groups are rewriting several
of them under the plan, so grep again.

| Removed or changed | Cited by |
| --- | --- |
| IFC-024 | ACC-019, LIF-031, OPS-014, REC-004 (OPS-015 to OPS-017 are removed); `spec/core/operations.yaml`, `spec/core/capabilities.yaml`, `spec/core/README.md`, `spec/definitions.md` (rows for caller, the engine's ID, in progress, retained result, retrieval tool, withheld, Conduit's keys and others), `docs/OVERVIEW.md` |
| IFC-034 | CON-022, CTR-005; `spec/core/README.md`, `spec/definitions.md` (Conduit's keys) |
| DL14 | OPS-027; `spec/definitions.md` (cancel) |
| Key `interfaceVersion` | `spec/core/README.md`, `spec/definitions.md` |
| Keys `state` and `withheld` under the prefix | `spec/core/README.md`, `spec/definitions.md`. `python3 tools/set_names.py --check` reports these now. |
| DL1 (re-runs gone) | LIF-026, OPS-021, OPS-028 cited it for what a re-run does; LIF-021 still cites it rightly |
| DL5 (now the machines provider's report) | LIF-010 |
| DL9 (busy instances gone; REV-014) | REV-012 |
| DL10 (retained results gone) | OPS-016 (removed); `spec/definitions.md` (retained result) |
| DL2 (purge gone from it) | REC-004; `spec/definitions.md` |
| IFC-008 outcome | OPS-007 lists "no result arrives" as unknown; IFC-008's -32021 error gives no tool result and is read as failed. OPS-007 should except it, or the coordinator rules the other way (below). |

New IDs other groups cite: IFC-037 (definitions, Conduit's extension), IFC-038, IFC-041 (TPL-012,
PRV-007, SEC-013). All exist.

## Still needs a decision

1. **Reconnecting to a `remote` item.** IFC-041 and SEC-013 tie the connection, and the read of
   its secrets, to the instance's setup. Nothing says what happens when that connection drops or
   the engine restarts: whether a later request may reconnect, and whose read a reconnection
   makes (decision 4c settles this for runs only).
2. **A run with no `endpoint` from another provider.** P18 lets `run.v1` return no `endpoint`, and
   says only the engine's own runner gives none. IFC-017 cannot reach a run of another provider
   that returns none; what the part reports is G2's (PRV-007) and is not stated.
3. **Provider calls and Conduit's extension.** PRV-005 has a provider call carry Conduit's keys,
   but a provider call is not a request, so IFC-037 does not cover it. Whether a provider call
   declares the extension, and what happens where a provider's discovery result lacks it (one
   reading: the provider is not ready, and the part reports unavailable), is not decided.
4. **The engine's own server and the extension.** Decision 18 speaks of requests to
   implementations. Whether the server the engine presents lists Conduit's extension in its own
   `server/discover`, and whether a consumer that sends Conduit's keys must declare it, is not
   decided; IFC-029 reads such requests by prefix.
5. **IFC-008's outcome.** Failed (kept, as the implementation performed nothing) or unknown (as
   OPS-007 treats a missing result). If failed, OPS-007 needs an exception (G4).
6. **A header whose configured value is not a string.** IFC-041 sends the value at a place in
   the configuration; whether MAN-017 must require a string there, or how another value is
   written, is not decided.
7. **Names for the vocabulary audit.** `config` and `revision` as keys (the key `revision`, the
   instance's revision, shares its word with the extension's `revision` setting, `v0`), and the
   extension's name `io.github.shrav-k.conduit/interface` (P1).
8. **To check against the protocol in round 5.** The extension negotiation (IFC-004, IFC-008,
   IFC-037), `listChanged` and `cacheScope` (IFC-038) and the reserved trace keys (IFC-039) are
   written from the review and the earlier checks, not from a fresh reading of revision
   2026-07-28.

## Validation

The validation command of [CONSOLIDATION.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#validation) passed over
the whole tree as it stood (411 promises, other groups mid-edit). Both files parse; every
promise of the interface file has exactly the seven fields; no line is longer than 96
characters. `python3 tools/set_names.py --check` reports three errors, all outside these files
(the removed keys `state` and `withheld` in `spec/core/README.md` and `spec/definitions.md`),
for the core-and-schemas and docs editors.
