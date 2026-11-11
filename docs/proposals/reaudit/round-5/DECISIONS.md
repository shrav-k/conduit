# Re-audit, round 5: decisions

Status: decided 2026-11-07 by the coordinator under rulings 37 and 38. No ruling has accepted
these decisions, and the owner can strike any of them.
Accepted by [ruling 58 (point 1)](../../ruling-58/README.md).

Six cold readers read `spec/` at commit `d0c425f` on `docs/pre-codex`, as rulings 40 to 43
left it. Reader 5 read the whole repository, and reader 6 checked the interface against the
published Model Context Protocol, revision 2026-07-28. Together they reported 119 findings.
The coordinator merged them into 111 items: eight items each join two findings that describe
one defect. Each item has one severity, which may differ from a reader's.

Under ruling 37, as the owner confirmed it in [ruling 38](../../ruling-38/README.md), the
coordinator settles a finding without a ruling, and a finding goes to the owner only where
settling it would change what Conduit is for
([who decides what](../../../AUDIT.md#who-decides-what)). Of the 111 items, 110 are fixed and
1 is rejected. None goes to the owner. All ten unsafe items are fixed.

"Fix" means the text changes as the decision says. A fix pass applies the changes and logs
what it did beside this record. Where two items change one text, the section
[items that change one text](#items-that-change-one-text) says how they combine.

## The readers

| Reader | Unsafe | Contradiction | Undetermined | Wrong | Misleading | Minor | Total |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [Reader 1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-1.md), contradictions | 1 | 4 | 4 | 6 | 2 | 7 | 24 |
| [Reader 2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-2.md), terms and shapes | 0 | 1 | 1 | 3 | 4 | 12 | 21 |
| [Reader 3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-3.md), behavior on one engine | 1 | 1 | 10 | 1 | 1 | 2 | 16 |
| [Reader 4](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-4.md), behavior across machines | 7 | 0 | 11 | 1 | 1 | 3 | 23 |
| [Reader 5](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-5.md), the whole repository | 0 | 1 | 2 | 2 | 7 | 2 | 14 |
| [Reader 6](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-6.md), the protocol | 1 | 1 | 6 | 7 | 2 | 4 | 21 |
| All findings | 10 | 8 | 34 | 20 | 17 | 30 | 119 |
| The 111 items | 10 | 8 | 32 | 19 | 16 | 26 | 111 |

A source ID such as 5-4-03 is round 5, reader 4, finding 3 of that reader's report.

| Area | Items | Fix | Reject | Owner |
| --- | --- | --- | --- | --- |
| F1. Providers, secrets and placement | 22 | 22 | 0 | 0 |
| F2. Manifests, schemas, templates and submissions | 9 | 9 | 0 | 0 |
| F3. Invocations, retirement and reconciliation | 13 | 13 | 0 | 0 |
| F4. The implementation interface, the protocol and credentials | 34 | 33 | 1 | 0 |
| F5. The core package, the examples and the names | 8 | 8 | 0 | 0 |
| F6. The specification's README and definitions | 13 | 13 | 0 | 0 |
| F7. The tracker and the entry documents | 12 | 12 | 0 | 0 |
| All | 111 | 110 | 1 | 0 |

## F1. Providers, secrets and placement

| Item | Source IDs | Severity | Finding | Decision | Reason |
| --- | --- | --- | --- | --- | --- |
| M01 | 5-4-03 | unsafe | A secret Resource reached by a plain `remote` item stands behind no gate, and may stand at `http` on another machine | fix | Holds. SEC-024 and IFC-044 leave a plain store at an `http` URL unchecked, so its values cross the network in clear, and two sentences of `spec/README.md` are false for it. SEC-024 applies its URL rule to any item that a template selects for `get-secret.v1`, and SEC-006, `spec/README.md` and the schemas README state the limit. An `http` loopback store is still accepted on one machine. |
| M02 | 5-4-04 | unsafe | A plain `remote` item at an `http` loopback URL is accepted on an engine that moves or runs as several parts | fix | Holds. SEC-024's own reason applies to every request on such a connection, not only to a secret header. On an engine that can move or run as parts, SEC-024 rejects every `remote` item at a loopback host, and IFC-041 sends nothing to one registered earlier. IFC-044 and DL17 follow. An engine that cannot move is unaffected. The PLC-007 part is not taken, because M22 covers it. |
| M03 | 5-4-07 | unsafe | A run is asked for again where its provider reports `unknown`, and nothing keeps a provider from starting a second | fix | Holds. PRV-007 and TPL-018 ask for a run only where the provider reports `stopped`. Where it reports `unknown`, the part reports unavailable, and an actor settles it under PRV-009 or declares the node lost. `run.v1` and `inspect-run.v1` say what `stopped` means. DL17 and the definitions stop citing TPL-019 for one run per instance. The engine's own runner knows its processes, so one machine sees no change. |
| M04 | 5-1-04 | contradiction | The PRV-004 example rejects a Conduit template that PRV-004 lets be registered | fix | Holds. PRV-004 also rejects such a template when it is registered, as CON-028 does. The example and the tool's column stand. |
| M05 | 5-1-02 | contradiction | SEC-024 is a language promise whose result depends on the engine's configuration | fix | Holds. SEC-024 takes the layer `convention`, as PRV-002 and PRV-013 already reject documents as conventions. Widening what resolving reads would change the Language level. |
| M06 | 5-1-06, 5-3-09 | undetermined | Where the item that is run of an Interface or a Pool runs, and what machine `run.v1` is given | fix | Holds. An Interface or a Pool has no placement and no `runtime`, so TPL-019 rejects a template of those families that selects an item that is run. That is one rule, where the other option needs exceptions in five promises. No example and no core template selects such an item. |
| M07 | 5-1-07, 5-6-03 | undetermined | Which registry types and transports the engine's own runner runs is stated nowhere | fix | Holds. PRV-003 has the own runner run a `package` item only over stdio and of a registry type that DL16 states, and end any other failed under PRV-012. DL16 states the types, as a Nomad client states its task drivers. |
| M08 | 5-1-09 | undetermined | DL16 and PRV-026 each leave to the other whether a secret Resource's process on the own runner ends with the engine | fix | Holds. PRV-002 says that such a process ends when the engine stops, as a provider's server does, and PRV-026 runs it again. This matches the coordinator's decision after ruling 43, and keeps a process that holds a store's credential from outliving the engine. |
| M09 | 5-4-08 | undetermined | When the part that holds a provider's server stops, nothing runs the provider again | fix | Holds. A part that stops while the engine goes on counts as a restart for the provider runs it held. PRV-026, PRV-002, SEC-010, SEC-011, ACC-019, REC-002, DL17, `get-secret.v1`, the definitions and the README say so. The narrower trigger is taken: "ended without the engine asking for it" would also cover a crash, and widen ACC-019 beyond what ruling 43 accepted. |
| M10 | 5-4-12 | undetermined | Whether the engine keeps a connection's secret headers or reads them again is open | fix | Holds. SEC-013 holds the values for as long as the connection lasts, and loses them sooner only when the engine restarts or the part that holds the connection stops. SEC-023 follows. This is the reading that the text already assumes. |
| M11 | 5-4-13 | undetermined | "Gone silent" is not defined, so whether a gate cancels a long call depends on the engine | fix | Holds. PRV-025 defines silence as the gate's own liveness check going unanswered, an HTTP/2 PING or a TCP keepalive, which every engine's stack answers. No new duty on the engine and no new item is needed. |
| M12 | 5-4-16 | undetermined | Whether a machine provider's report from before a restart still counts after it | fix | Holds. PLC-023 counts every machine but the one `local` represents as omitted until the first report after the engine starts. It keeps no new state, extends PLC-023's rule for a late report, and is the conservative choice, as in M03. |
| M13 | 5-6-12 | wrong | Keeping only the host of the engine's address breaks RFC 9728's exact-URL check | fix | Holds. A `CONDUIT_URL` given as the run sees it keeps the engine's address whole: scheme, host, port and path. This narrows the coordinator's decision N4 after ruling 43. |
| M14 | 5-6-14 | wrong | SEC-024 misquotes the protocol's rule on `http` | fix | Holds, checked against the security considerations of revision 2026-07-28. Only the citation changes. |
| M15 | 5-6-15 | wrong | PRV-025 limits disconnect-as-cancellation to streams | fix | Holds. The protocol has a server treat any client disconnect as cancellation, whether it answers with a stream or one JSON object. PRV-025 says so, and keeps that a server may notice late. Outcomes do not change. |
| M16 | 5-1-17 | misleading | The README and PRV-006 say a runtime provider makes a restriction enforced, but ACC-027 always reports mediated | fix | Holds. PRV-006 and `spec/README.md` say that a provider could make a restriction enforced, and that this version reports every restriction as mediated. ACC-027 stands. |
| M17 | 5-2-10 | minor | Words that definitions.md reserves to one sense are used in others: answer, entry, chain, plain, object | fix | Holds for each word. PRV-005, PRV-022, PRV-028, SEC-010, IFC-026, IFC-044, MAN-017 and CTR-012 are reworded, so the definitions keep their claims of one sense. |
| M18 | 5-2-11 | minor | "Store" also names the server that reads a store | fix | Holds. SEC-006 and `secret-provider.v1` write "an implementation of `get-secret.v1`", as the row "secret store" says to. |
| M19 | 5-2-12 | minor | "Refuse" also names a store's or a provider's declining, and in one place a conflict | fix | Holds. SEC-006, SEC-012, PRV-012, PRV-025 and the row "install" say what happens: the store declines, the run ends failed, the request reports conflict. |
| M20 | 5-1-20 | minor | A hostname longer than 63 characters cannot be the `kubernetes.io/hostname` label that PRV-014 requires | fix | Holds. PRV-014 reports no hostname longer than 63 characters, the most a label's value holds, and a longer name is reported under one that fits. `list-machines.v1` and `place.v1` bound `hostname`. An engine whose own hostname is longer is configured with another, under PLC-026 and DL5. |
| M21 | 5-2-20 | minor | Head comments written in the present tense still use renamed words | fix | Holds. The present-tense sentences in the heads of placement.yaml and providers.yaml use the names in force. Lines that record earlier passes keep their words. |
| M22 | 5-4-21 | minor | An instance with no item that is run is placed on an arbitrary machine, and is made unavailable and retired with it | fix | Holds. PLC-002 and PRV-015 place on `local` an Agent or a Resource for which no item that is run is selected, as PRV-002 places a provider. Decision 8c of the principles review covers what runs, and decides nothing for an instance that runs nothing. A placement given in a document still counts. |

## F2. Manifests, schemas, templates and submissions

| Item | Source IDs | Severity | Finding | Decision | Reason |
| --- | --- | --- | --- | --- | --- |
| M23 | 5-6-02, 5-4-23 | contradiction | A Remote's URL variables are admitted by `server-remote.v1` but always rejected, and SEC-024 has a rule for a case that cannot arise | fix | Holds. `url` is a URI under RFC 3986, which has no braces, and no key binds a variable. MAN-010 says outright that such a `url` is rejected, and SEC-024's sentence, the schemas README's clause and a docstring of the tool go. `variables` stays in the schema, to follow server.json. Binding variables would be a new feature, for a later ruling. |
| M24 | 5-2-01 | contradiction | OBJ-019 rejects the author's keys under `patternProperties`, `dependentRequired` and `dependentSchemas`, which the profile admits | fix | Holds. Those keys are names and patterns that the author chooses, not keywords, so OBJ-019 exempts them. The head of schema.yaml and the schemas README follow. |
| M25 | 5-1-08 | undetermined | A provider whose implementation is reached by `remote` has no way to be run, and no promise rejects it | fix | Holds, and P6 of ruling 41 already decides where providers run. A new promise, PKG-030, rejects a `remote` item that implements an operation of the three provider capabilities. A provider for a cloud API is a local item that calls the API, as the kubelet reaches its runtime at a local socket. The tool gains a rule and an example. |
| M26 | 5-2-02 | undetermined | Whether a reference schema reached through a second `$ref` governs a place is not said | fix | Holds. OBJ-021 follows `$ref` chains, since SCH-001 gives `$ref` its JSON Schema meaning. SEC-001 and the definitions follow. Rejecting aliases would refuse an ordinary idiom, and an alias of `secret-reference.v1` falls under the same rules as the direct form. |
| M27 | 5-3-10 | undetermined | A program given as a relative path escapes the check of PRV-007 once SUB-010 has resolved it | fix | Holds. PRV-007's guard follows what the manifest wrote, and SUB-010 says that a resolved program is still one given as a relative path. This keeps the reason of D20 of the open questions after the rewrite. |
| M28 | 5-3-11 | undetermined | Two directory submissions have one digest and two sets of programs | fix | Holds. NAM-009 has a template keep the items it selects as its own registration resolved them, and SUB-008 states the consequence. MAN-017, MAN-018 and SEC-024 already cite the item as the template holds it. |
| M29 | 5-3-12 | undetermined | Whether a retirement asks for the run when the retire hook is served by a remote item | fix | Holds. TPL-018 asks for the run only where the hook's implementation is the item that is run, as PRV-007, TPL-009 and `retire.v1` already read it. |
| M30 | 5-5-02 | undetermined | A configuration value that is a map or a list can reach a variable, an argument or a header, and no rule converts it | fix | Holds. Row 22 of the open questions after the rewrite already chose to reject such a value, not encode it. MAN-017's test becomes positive: the schema of a bound place confines its value to a string, a number, a boolean or null. The tool and an example follow. This closes the check that rulings 42 and 43 left for round 5. |
| M31 | 5-1-19 | minor | SUB-008 provides for a missing manifest that MAN-015 makes impossible | fix | Holds. The clause is struck. |

## F3. Invocations, retirement and reconciliation

| Item | Source IDs | Severity | Finding | Decision | Reason |
| --- | --- | --- | --- | --- | --- |
| M32 | 5-3-01 | unsafe | A repeat answered from the stored outputs bypasses the filter of the Conduit it arrives through | fix | Holds. ACC-008 and ACC-011 already state the rule, so it is not the owner's. OPS-008 keeps the filters that the latest attempt was sent. OPS-009 answers from the outputs only where every filter that applies to the repeat was among them, or where the operation declares no outputs. Otherwise OPS-034 reports conflict, under its own ruleId, since a new ID would need edits in five other places. |
| M33 | 5-1-03 | contradiction | LIF-023's rule forbids the retirements that PLC-024 requires | fix | Holds. The exceptions move from LIF-023's observable into its `when`. Behavior does not change. |
| M34 | 5-3-02 | contradiction | An invocation reconciled once and attempted again can end unknown, with no rule on a second reconciliation | fix | Holds. A reconciliation settles the latest attempt, so a later attempt that ends unknown is reconciled in its turn. OPS-030 and LIF-013 follow. |
| M35 | 5-3-04 | undetermined | A control that ends after its instance was declared lost | fix | Holds. Its outcome is recorded and nothing is stored, whether it ends succeeded or is reconciled, since LIF-015 keeps a retired instance from any change. REV-007, REV-010, REV-012, OPS-031, LIF-015, `reconcile.v1` and the definitions follow. |
| M36 | 5-3-05 | undetermined | An earlier retirement is reconciled as having occurred while a later one has failed or is in progress | fix | Holds. REV-014 counts a reconciliation of a setup or a retirement as one, so it waits for the other to end. LIF-024 says that any retirement that succeeded, or that is reconciled as having occurred, confirms the shutdown. |
| M37 | 5-3-06 | undetermined | An open of unknown outcome is reconciled as having occurred after the Conduit's consumer was retired | fix | Holds. LIF-028 records the outcome and leaves the Conduit registered under CON-031, as it already leaves a retired instance retired. `reconcile.v1` follows. This is the safe reading. |
| M38 | 5-3-07 | undetermined | `retire-with-resources.v1` arrives while the Pool's own setup is in progress | fix | Holds. REV-014 counts the request as a retirement of the Pool, so it reports conflict at once while a setup, a retirement or a change of the Pool is in progress. The Pool's `retire.v1` that it dispatches is part of it. |
| M39 | 5-3-08 | undetermined | A repeat of an invocation that was reconciled as having succeeded | fix | Holds. The engine keeps no outputs of an invocation whose outcome a reconciliation set. A repeat of one falls under OPS-034 where the operation declares outputs, and under OPS-009 where it declares none. |
| M40 | 5-3-13 | undetermined | Whether the connection to a remote item is a part with an effect | fix | Holds. OPS-022 counts the connection as a part with no effect: it is the engine's own holding of header values, and changes nothing outside the engine. Counting it as an effect would leave a failed setup unknown, with no true reconciliation but "did not occur". |
| M41 | 5-3-03 | wrong | A retired instance whose retire hook needs a secret it can no longer read is never removed | fix | Holds, and any retire hook that always fails reaches the same dead end. `unregister.v1` may remove any retired instance, as removing a Kubernetes finalizer by hand does. The stop is asked for once more, the instance's Conduits close, and the request's authorization is unchanged. LIF-017 and the promises and rows that limited unregistering to a node declared lost follow. |
| M42 | 5-3-15 | minor | No hook can deduplicate, yet two promises describe a hook of unknown outcome dispatched as a new attempt | fix | Holds. OPS-021, OPS-028 and the row "attempt" say that a hook of unknown outcome is not dispatched again until it is reconciled. Behavior does not change. |
| M43 | 5-3-16 | minor | The order of LIF-021 is not the reverse of POOL-013's | fix | Holds. LIF-021's order extends the reverse of POOL-013's. |
| M44 | 5-1-18 | minor | LIF-029 repeats POOL-003's rule for an entry | fix | Holds. LIF-029 is limited to a declared Conduit, and LIF-009 and LIF-033 name POOL-003 for an entry. No promise is removed or moved. |

## F4. The implementation interface, the protocol and credentials

| Item | Source IDs | Severity | Finding | Decision | Reason |
| --- | --- | --- | --- | --- | --- |
| M45 | 5-1-01 | unsafe | A filter's optional input, added by a later release, can be ignored by an implementation built for an earlier release | fix | Holds. IFC-019 fails closed: an implementation that is sent a filter argument its version of the requirement does not declare cannot apply the filter, performs nothing and answers unavailable. CON-022 says so. Only the implementation knows its version, and the rule also covers filters carried onward. |
| M46 | 5-4-01 | unsafe | A move fences only the engine's own runner, so a run that another runtime provider made on `local` is left behind | fix | Holds. DL17 fences every run of an instance on `local`, whichever provider made it, as the Kubernetes precedent it cites fences the machine. Rejecting other providers on `local` would contradict the README's advice. |
| M47 | 5-4-02 | unsafe | A run's credential goes in clear to an `http` loopback address that anything may hold while the engine is not listening | fix | Holds. DL13 and PRV-022 give an `http` loopback `CONDUIT_URL` only to a run of the own runner that ends with the engine, on an engine that can neither move nor run as parts. Every other run gets `https`. One machine with everything on the own runner still needs no certificate. |
| M48 | 5-4-05 | unsafe | The engine may follow an HTTP redirect and present its certificate to a server that has proved nothing | fix | Holds. IFC-017 has the engine follow no redirect from a run or from a `remote` item that is not plain, and read one as no result. |
| M49 | 5-4-06 | unsafe | The engine takes a token for one `remote` item from whatever authorization server that item names | fix | Holds. IFC-041 takes a token only from the server that DL13 states, or from one that the user configured for the item's URL. The client credentials extension leaves that choice to the client. DL13 states how the user configures it. |
| M50 | 5-6-01 | unsafe | `tasks/update` and task notifications are decided under no Conduit | fix | Holds. IFC-016 decides every request of the tasks extension for the task, and a `subscriptions/listen` that names it, through the invocation's Conduit, for the instance whose request began it, and refuses it from any other. A task's ID establishes nothing. |
| M51 | 5-4-09 | undetermined | Which proof a run's gate is given is the engine's choice, and which proof it admits is the provider's | fix | Holds. DL13 has every engine present both forms to a run's gate: its certificate when the gate asks for one, and its token. Every engine already issues both, so a gate written once works under every engine, as PRV-022 intends. |
| M52 | 5-4-10 | undetermined | Nothing fixes how the engine's token names the instance it is meant for | fix | The first part holds: DL13 fixes the audience as the name that the run's own `token` carries in `sub`. The second part is not taken: Q1 of the multi-machine proposal, which ruling 43 accepted, makes SPIFFE IDs recommended and not required. DL13 says which gates that leaves portable. |
| M53 | 5-4-11 | undetermined | At an `http` loopback address no engine must accept the token, so a harness written once may be refused | fix | Holds. IFC-042 accepts the run's `token` at the loopback address that DL13 gives a run, and IFC-022 and DL13 follow. Under M47 that address goes only to runs that end with the engine. |
| M54 | 5-4-14 | undetermined | A `remote` item that takes Conduit's keys has no credential for the onward requests the text expects of it | fix | Holds. IFC-015 says that an implementation reached by `remote` makes no onward request in this version, which the text already implies. A fixed token-exchange grant would add a mechanism that no ruling asked for. |
| M55 | 5-4-15 | undetermined | Where the existence of the instance that a valid credential names cannot be read, engines differ | fix | Holds. ACC-037 takes the case: the request reports unavailable, with no 401, as where the state of Conduits cannot be established. IFC-032 follows. |
| M56 | 5-4-17 | undetermined | What a request to a ready instance whose process on the own runner has ended reports is not stated | fix | Holds. IFC-017: where the own runner has no process for the run, nothing is sent, and the request or the hook reports unavailable. A process that dies during a call stays the unknown of OPS-007. |
| M57 | 5-4-18 | undetermined | Which roots the engine trusts when TLS proves a `remote` item's host is not stated | fix | Holds. DL13 states them, and IFC-017 cites it. |
| M58 | 5-6-04 | undetermined | The wire form of a refused `tasks/get`, `notifications/tasks`, and declaring tasks to an implementation | fix | Holds. A request of the extension that does not proceed is answered with -32602, which the extension uses for a task a server does not hold. No task notification goes out once the Conduit is not open. Tasks are declared to an implementation only where the consumer's request declared them. |
| M59 | 5-6-05 | undetermined | A plain server's unstructured content reaches the consumer or not, depending on the engine | fix | Holds. IFC-025 passes a plain implementation's `content` on unchanged. Never passing it on would block most existing servers, against the principle never to block the protocol. It goes only to the consumer that asked, and gains no meaning as outputs. DL12 follows. |
| M60 | 5-6-06 | undetermined | Which client capabilities the engine declares, and what it does with input requests | fix | Holds. IFC-037 and IFC-007 declare no capability but Conduit's extension, and the tasks extension where IFC-016 lets it, and IFC-016 says that the engine answers no input request. OPS-007 tells IFC-008's -32021 apart by `data.requiredCapabilities`. |
| M61 | 5-6-07 | undetermined | `x-mcp-header` needs the server's tool list, and -32020 has no resend | fix | Holds. IFC-001 has the engine read `tools/list` when it reads discovery, and send a call rejected with -32020 once more as one invocation, as IFC-003 does for a version error. |
| M62 | 5-6-08 | undetermined | A tool's description and `readOnlyHint` are not fixed | fix | Holds. IFC-033 gives a tool the contract's `metadata.description`, and sets `readOnlyHint` exactly where the contract states no `effect`, which CTR-002 makes accurate. |
| M63 | 5-1-13 | wrong | DL17 says that replacing the template fixes a secret header sent to `http`, but existing instances keep their item | fix | Holds under INS-020. DL17 says that each instance made from the old template is replaced too. |
| M64 | 5-4-19 | wrong | A short-lived engine credential does not bound how late a paused part's request lands | fix | Holds. DL17 says what the credential does bound: a request on which it is checked. A connection checked earlier by mutual TLS, or a token from an authorization server, bounds the rest. Closing connections in time would rely on the paused part's timing, which DL17 rejects. |
| M65 | 5-6-09 | wrong | IFC-004 excludes every legacy revision for plain servers too | fix | Holds. IFC-004's list applies to a call that carries Conduit's keys, and a call to a plain implementation relies on less. Whether an engine speaks a legacy revision stays DL12's. |
| M66 | 5-6-10 | wrong | The client credentials extension does not allow mutual TLS | fix | Holds. IFC-041 drops the RFC 8705 branch, since the engine can always sign an RFC 7523 assertion. This strikes half of choice 8 of ruling 43's verification. |
| M67 | 5-6-11 | wrong | Tokens sent as the protocol's authorization need not come from its authorization server | fix | Holds. DL13 has the issuer of the mechanism's tokens listed in the engine's protected resource metadata, which RFC 9728 lets name several servers, and named by a `remote` item to which the engine sends such a token. |
| M68 | 5-6-13 | wrong | IFC-037 cites the era cache as a rule for discovery results | fix | Holds. Only the attribution changes, and IFC-037's rule stands. |
| M69 | 5-1-16 | misleading | DL11 leaves to each engine what ACC-001, ACC-003 and ACC-036 already decide | fix | Holds. The only choice that keeps every promise restates ACC-003, so DL11 is removed and its ID is not reused. LIF-027 points to ACC-003, and the list has 15 items. |
| M70 | 5-4-20 | misleading | DL17 names reads and a compare-and-set on one invocation as the mechanism, which do not keep rules that span invocations | fix | Holds. DL17 adds that a decision among competing changes is committed against the state it read, as an etcd transaction compares a revision. |
| M71 | 5-6-16 | misleading | A refusal may never reach an Agent's model | fix | Holds, on one of the tracker's two points to look at. IFC-025 puts first in an error's `content` one text block that states the reason or the outcome, and IFC-012's error `message` says that the tool is not presented. Nothing new is disclosed. |
| M72 | 5-1-22, 5-2-21 | minor | The interface file's head comment omits IFC-044 from its language-layer promises | fix | Holds. The head names IFC-044 too. |
| M73 | 5-1-23 | minor | DL4's subject names a case that cannot arise | fix | Holds. DL4 is about access decisions that cannot be made, under ACC-037. |
| M74 | 5-4-22 | minor | A `file:` fetch on an engine of several parts reads the file of whichever part takes the request | fix | Holds. DL17 says that the fetch reads one file whichever part makes it, and leaves how to the engine. |
| M75 | 5-6-18 | minor | SEP-2145 would put IFC-012 against IFC-001 | fix | Holds as a latent conflict, on the tracker's second point. IFC-012 answers as the protocol prescribes at the revision in use, which at the baseline is -32602. |
| M76 | 5-6-19 | minor | A private cache scope does not separate instances by itself, and `ttlMs` is unset | fix | Holds. IFC-038 states the protocol's rule on authorization contexts, and DL10 gains the tool list's `ttlMs`. |
| M77 | 5-6-20 | minor | IFC-036's list of protocol rejections omits the Origin 403 | fix | Holds. IFC-036 adds the case with its HTTP status, and DL12 states which origins the engine accepts. |
| M78 | 5-6-21 | minor | A stdio-only consumer has no way to reach the engine | reject | Nothing here differs between engines. Every engine accepts the token in the `Authorization` header (IFC-042, and at a loopback address under M53), so an existing stdio bridge that sends it works under all of them. A new implementation-defined item would add a difference where none exists. |

## F5. The core package, the examples and the names

| Item | Source IDs | Severity | Finding | Decision | Reason |
| --- | --- | --- | --- | --- | --- |
| M79 | 5-1-05 | contradiction | The valid project's existing server `git-mirror-server` is not marked plain | fix | Holds. The example marks it `plain`, with `tools`, which keeps its comment's meaning. The project then also shows a plain `command` item. |
| M80 | 5-1-10 | wrong | `register.v1`'s description of when a registration is rejected for want of a machine contradicts PRV-015 | fix | Holds. The description states PRV-015's rule, with "omitted". |
| M81 | 5-1-11, 5-2-05 | wrong | `get-secret.v1` and `secret-provider.v1` say that a secret Resource's server is reached only with the engine's proof | fix | Holds: PKG-020, PRV-001, SEC-006 and PRV-025 allow a plain `remote` store and a process of the own runner. Both descriptions state the condition: run by another runtime provider, or reached by `remote` and not plain. |
| M82 | 5-1-12 | wrong | `declare-lost.v1` says that the instances on a lost node can never confirm shutdown | fix | Holds under PLC-017 and PLC-027. The description begins as the row "declare lost" does. |
| M83 | 5-2-04 | wrong | The core README misstates what the engine's own runner passes from the engine's environment | fix | Holds. The README gives PRV-023's other set of variables and its exclusion of values that begin with `()`. |
| M84 | 5-2-09 | misleading | The schemas README misstates the shapes of `object.v1`'s `nested` and of `record.v1`'s `data` | fix | Holds. The row on `nested` says that the result gives an ID and the request takes a reference (LIF-031). The row on a record adds `serving`, `inputs` and `outputs`. |
| M85 | 5-1-24 | minor | The core README still leaves open which files of `identity` a run is given | fix | Holds. Only whether `tls.crt` and `tls.key` go to a run on the own runner is left open. |
| M86 | 5-2-19 | minor | names.yaml calls `ca.crt` alone the trust bundle file | fix | Holds as to the key, which becomes `x509BundleFile`, SPIFFE's name beside `jwtBundleFile`. No name that the specification assigns changes, and the two tools read the new key. |

## F6. The specification's README and definitions

| Item | Source IDs | Severity | Finding | Decision | Reason |
| --- | --- | --- | --- | --- | --- |
| M87 | 5-5-03 | undetermined | No conformance level covers what a runtime provider or a machine provider must itself do | fix | Holds. The Implementation level covers every promise whose obligation falls on an implementation, whatever its layer, a provider's among them. The Engine level covers the engine's own obligations, those its own runner and providers keep included. No fourth level is needed and no behavior changes. The overview follows. |
| M88 | 5-1-14, 5-3-14 | wrong | The README's rotation bullet ignores the reads after a restart of SEC-023 and PRV-026 | fix | Holds, and a retirement that sends the retire hook reads anew too. The bullet is restated from SEC-013, SEC-014, SEC-023, PRV-026 and TPL-018. |
| M89 | 5-1-15 | wrong | The README says that SEC-019 binds an implementation of `get-secret.v1`, but it binds every implementation given a secret | fix | Holds. The paragraph on layers says so. |
| M90 | 5-2-03 | wrong | The table of words the promises do not use is false for grant, declaration, policy, host, origin and the engine's own machine | fix | Holds. Each row says that the word is not used in the retired sense, and names the standard senses that remain. "The engine's own machine" becomes "the machine the engine runs on" wherever M47 and M53 do not rewrite the sentence. |
| M91 | 5-1-21, 5-2-08 | misleading | "Names a version assigns" means three names in one place and every name of names.yaml in another | fix | Holds. Q1 of the multi-machine proposal fixed the names of what `identity` carries with the other names, so OBJ-024 and the row count them, and only a new version may change them. |
| M92 | 5-2-06 | misleading | "Contract" also names a provider's or a type's published behavior, which no contract document can hold | fix | Holds. A new row defines a type's, a provider's and a package's contract as the documents with the prose published beside them, which no engine reads. Renaming the word in some twenty promises is not needed. |
| M93 | 5-2-07 | misleading | "The engine's address" is a URL and is undefined, while "address" is defined as a package's identity | fix | Holds. A new row defines the engine's address, and the row "address" is qualified. The new row says what M13 and M47 leave: the address kept whole, and `http` on loopback only under DL13's condition. |
| M94 | 5-2-13, 5-6-17 | misleading | The definition of connection names both a transport connection and the engine's held values, and calls them the protocol's | fix | Holds. The row names Conduit's own notion first and says when a promise means the transport's connection, with no new term. IFC-037's "at each connection" is the held sense. |
| M95 | 5-2-14 | minor | "Registration" is defined as the one operation that takes documents, but `replace.v1` takes one too | fix | Holds. The row says that `register.v1` takes a submission, and names the other senses (LIF-009, PLC-026, FST-001). |
| M96 | 5-2-15 | minor | "Label" is a key-and-value pair in its own row, but the key everywhere else | fix | Holds. The row makes the label the key. |
| M97 | 5-2-16 | minor | "ID" and "the engine's ID" name both an ID the engine gives and an ID the mechanism gives the engine | fix | Holds. The mechanism's ID becomes "the engine's credential ID" in the definitions, IFC-041, DL13 and the core README. The file `engine-id` keeps its name. |
| M98 | 5-2-17 | minor | Defined terms are used against their own rows: a hook as a request, a provider call as an invocation, "ever set up" | fix | Holds. The hook descriptions say "from its call", and the rows "configuration", "provider call" and "hook" follow IFC-006, the row "invocation" and TPL-018. |
| M99 | 5-2-18 | minor | The definitions row for status leaves out four things that promises put in a status | fix | Holds. The row adds them, with their promises. |

## F7. The tracker and the entry documents

| Item | Source IDs | Severity | Finding | Decision | Reason |
| --- | --- | --- | --- | --- | --- |
| M100 | 5-5-01 | contradiction | The tracker says five rounds have read `spec/`, but no fifth round exists | fix | Holds at `d0c425f`. The tracker says that four rounds read the text before the rewrite and that round 5 read it after, with a link to this record, and that `round-6` is the next directory. Round 4's record gains a line on ruling 41's order. |
| M101 | 5-5-04 | wrong | Three entry documents give 440 promises, but the specification holds 449 | fix | Holds. README.md and the tracker give no total and link the contents table. INDEPENDENT_BUILDS.md gives the count once this round's edits are in, which PKG-030 makes 450. |
| M102 | 5-5-05 | wrong | The tracker says that the implementation-defined list has fourteen items, but it has sixteen | fix | Holds. The row records DL16 and DL17, and, since M69 removes DL11, ends at fifteen. |
| M103 | 5-5-06 | misleading | The README, AGENTS.md and the tracker describe the specification as ruling 42 left it | fix | Holds. They are dated 2026-11-07 and name ruling 43, and the tracker's row on secrets lists every read (SEC-010, SEC-013, SEC-023, PRV-026). |
| M104 | 5-5-07 | misleading | Entry points send the reader to lists of open questions that later records settled | fix | Holds. The links say, in the past tense, where each list was settled, and a dated line under each heading of the logs says the same. |
| M105 | 5-5-08 | misleading | What was left for round 5, and what is still open, is named nowhere a reader is sent | fix | Holds. The tracker names the check against the protocol in step 3, lists what the records left for a reader, and says that Q5 of the multi-machine proposal settled X2. M30 is fixed, so MAN-017 is not listed as open. |
| M106 | 5-5-09 | misleading | Bare identifiers in the tracker name both its own D and Q items and the questions of later records | fix | Holds. The identifiers table gains rows for the two records, and the rows of the ruling log stay as written. |
| M107 | 5-5-10 | misleading | The tracker does not say which decisions made after ruling 39 stand unaccepted, or where they are | fix | Holds. "Who decides what" lists where they are logged, this record among them, and says that the 43 decisions of which ruling 42 struck none keep the standing of decisions the owner can strike. AGENTS.md and `spec/README.md` follow. This concerns provenance only. |
| M108 | 5-5-11 | misleading | Records and indexes still call ruled proposals unruled | fix | Holds. The vocabulary audit gains a banner, two lines "Status:" become "Was:", and the indexes and GAPS.md say what was ruled. |
| M109 | 5-5-12 | misleading | A verifier's choice withdrew a behavior that rulings 42 and 43 accepted, and no list records it | fix | Holds as a gap in the record. The substance follows from the proposal that ruling 43 accepted. The tracker's later amendments and the choices of ruling 43's verification record MX-21. No promise changes. |
| M110 | 5-5-13 | minor | Three action items are out of date | fix | Holds. A21 drops the branch, A22 points to Experiment 005, and A29 points to the definitions. |
| M111 | 5-5-14 | minor | AGENTS.md says that the names tool rewrites every occurrence | fix | Holds. AGENTS.md says where the tool rewrites, and that the overview's quotation is updated by hand. |

## Items that change one text

- SEC-024 has one new text that carries M01, M02, M05 and M14. M23's change applies to it as
  well: its sentence on a host written as a variable of the Remote ends after "as the Remote
  writes it". The row for SEC-024 in the schemas README takes M01's and M23's changes.
- PRV-025's observable has one text for M11 and M15.
- PRV-022 takes M13 and M17 in one sentence of its observable. M47 rewrites the sentence on an
  `http` loopback address, which replaces M90's edit of it.
- DL13's sentence on the address that the engine gives a run is M47's, which replaces M90's
  edit. IFC-042's sentence on loopback is M53's, which replaces M90's edit.
- TPL-018 takes M03 and M29 in one sentence.
- PRV-002 and PRV-026 take M08 and M09 in different clauses.
- OPS-009 and OPS-034 have one text for M32 and M39.
- REV-014's observable takes M36 and M38 in different places.
- `reconcile.v1`'s description takes M35 and M37.
- IFC-016 takes M50, M58 and M60. IFC-025 takes M59, M71 and M75, and IFC-012 takes M71 and
  M75.
- The new row "the engine's address" (M93) states PRV-022 as M13 and M47 leave it.
- Counts: PKG-030 (M25) makes 450 promises, 203 of them in the layer `language`, and 22 in
  packages.yaml. M69 leaves 15 implementation-defined items. M25 and M30 each add an example
  under `examples/invalid-by-rule`, which then holds 30. `spec/README.md` and every document
  that states these counts is recounted with a script after the fix pass.

## For the owner

No item goes to the owner: none of the decisions changes what Conduit is for. These come
closest to an earlier ruling or decision, and the owner may want to read them first:

- M13 narrows the coordinator's decision N4 after ruling 43: a `CONDUIT_URL` given as the run
  sees it keeps the whole address, not only the host.
- M22 places on `local` an instance that runs nothing. Decision 8c of the principles review,
  which ruling 41 accepted, has the machine provider place instances, and covers what runs.
- M41 lets `unregister.v1` remove any retired instance, not only one whose node was declared
  lost, as decision 17 of the principles review keeps a shutdown pending until it is settled.
- M47 gives `https` to every run that may outlive the engine. A run under a container provider
  on one machine then needs the certificate checked against `ca.crt`, which it mostly needs
  already, since it reaches the host at a name that is not loopback.
- M59 passes a plain server's `content` to the consumer unchanged.
- M66 strikes half of choice 8 of ruling 43's verification: the engine no longer authenticates
  by mutual TLS to an authorization server.
- M69 removes DL11 from the implementation-defined list.
- M107 reads ruling 42's "struck none" as leaving the coordinator's 43 decisions with the
  standing of decisions the owner can strike, not as accepting them.
