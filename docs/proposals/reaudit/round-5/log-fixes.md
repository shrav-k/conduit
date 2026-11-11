# Re-audit, round 5: log of the fix pass

Date: 2026-11-07. The fix pass applied the coordinator's [decisions](DECISIONS.md) on round 5,
which no ruling has accepted and which the owner can strike. Seven passes applied them by
area, each in its own files, and an integration then made the files agree. This log sums up
the seven passes, with a link to each log, and records what the integration changed.

Nothing was committed or staged.

### The choice of this pass

The log of area F3 found a case that M41 opens and no promise settles. Before M41 only an
instance whose node was declared lost could be unregistered, and such a node represents no
machine, so PLC-023 never held back the stop that LIF-036 asks for. Now a retired instance on
a node that is not lost can be unregistered while the machine provider reports its machine
unreachable or omits it. PLC-023 then held back the stop and reported unavailable, and LIF-036
did not say whether the instance is removed.

This pass settled it under rulings 37 and 38, as following from M41, and the owner can strike
it. PLC-023 no longer holds back the stop that a request to unregister asks for, and LIF-036
says why: the request is the last time the stop is asked for, so it is asked for even while
the machine is reported unreachable or omitted, and what a runtime provider does with a stop
that it cannot carry out now is PRV-021, as for a node declared lost. That keeps what LIF-036
is for, that a run left on a machine that was cut off is asked to stop, and treats both cases
alike.

The options were:

1. As done here: the stop is asked for all the same, and the instance is removed.
2. The stop is not asked for, and the instance is removed all the same, as where the runtime
   provider is not ready. A run left on that machine is then never asked to stop.
3. The request reports unavailable and the instance stays until the machine is reachable, or
   an actor declares its node lost. That keeps a retired instance that the actor asked to
   remove, and declaring the node lost retires every other instance on it.

## Verification

Date: 2026-11-07. Two verifiers checked the fixes of this round and the integration, and
reported 29 findings: V1 to V13 from the first, V2-01 to V2-16 from the second. The coordinator
checked each against the text and settled it under rulings 37 and 38, as the decisions of this
round are settled: each fix below follows from a ruling, a record or the wording it cites, and
the owner can strike it. Every finding holds. 28 are fixed; 1, V6, is
[for the owner](#for-the-owner), with options and a recommendation. Nothing was committed or
staged.

Note of 2026-11-07: V6 was settled afterwards by the coordinator, under rulings 37 and 38, as
its option 1, and applied to PLC-007 and PLC-002, as the end of
[for the owner](#for-the-owner) records. All 29 findings of this verification are resolved;
the row for V6 below, and the heading of that section, keep their words as written.

| Finding | Severity | Decision | What changed |
| --- | --- | --- | --- |
| V1 | contradiction | fix | IFC-022's `then`: where only whether the named instance still exists cannot be read, the request establishes neither, and is decided under ACC-037, as IFC-032 says. DL13's "exactly one instance or none" says the same. |
| V2 | contradiction | fix | IFC-046, added: a call that the implementation rejects with -32020, or with the error for a revision it does not support, is sent once more, and the two are one call. IFC-010's `then` and OPS-007's `when` except an error after which the call is sent once more, and IMP-007's observable says that such a call is part of the same dispatch. IFC-001 and IFC-003 cite IFC-046 in place of stating the resend. |
| V3 | misleading | fix | The description of `register.v1` and the core README's section on secrets restate SEC-024 as it now reads: a secret in a header or as the value of `get-secret.v1`, and a loopback host on an engine that can move or run as several parts. |
| V4 | misleading | fix | IFC-016 answers a repeat from the outputs only while every filter that applies to it was sent with the latest attempt, and reports conflict under OPS-034 otherwise. OPS-005, IFC-025, the row "conflict" and the text under "How a request ends" in the definitions name the filter case beside the outputs no longer kept. |
| V5 | misleading | fix | With V2-03: PRV-007 says that no operation settles a run's state, that the part that asks for the run and a retire hook that needs it report unavailable while the provider reports `unknown`, and what an actor can do: read `Running` under PRV-009 and wait, retire the instance and unregister it, or declare its node lost where that node is not `local`. |
| V6 | minor | owner | Holds: PLC-002 places an instance that runs nothing on `local` and checks it under PLC-007, so a type whose `nodeSelector` `local` does not match makes such a template rejected. Neither way follows from the records alone; see [for the owner](#for-the-owner). |
| V7 | minor | fix | PRV-025 drops the condition that a token-checking proxy not pass the token on, and says why a token passed on gives the server nothing: its audience is the instance's ID (DL13), which no other gate, no `remote` item and not the engine's address admits. M51's two proofs to every gate stand. |
| V8 | minor | fix | IFC-036: "where the response carries one". |
| V9 | minor | fix | IFC-045, added, states in its `then` what a call to an implementation declares in `clientCapabilities`, for every call and provider call; IFC-037's `then` no longer states it, and IFC-007, IFC-016 and IFC-037 cite IFC-045. The resend after -32020 is IFC-046's `then` (V2). Reading `tools/list` stays in IFC-001's observable: it is how the engine keeps the protocol's requirement on `x-mcp-header`, which IFC-001's `then` already makes binding. |
| V10 | minor | fix | `spec/schema-profile.md`, `spec/core/README.md` and `spec/schemas/README.md` say that ruling 42 struck none of the coordinator's decisions, as `spec/README.md` does. The head comments of `placement.yaml`, `providers.yaml`, `pools.yaml` and `first-start.yaml` record ruling 42 as it was read then, and keep their words as history. |
| V11 | minor | fix | Items 8 and 10 of the [choices of ruling 43's verification](../../ruling-43/log-integration.md#choices-of-the-verification) have dated lines for M66 and M53. |
| V12 | minor | fix | "Who decides what" in the tracker and the list in `spec/README.md` name this log beside the round's decisions, for the integration's choice on PLC-023 and LIF-036 and the decisions of this verification. |
| V13 | minor | fix | The [re-audit record](../README.md) names round 5, with its counts and links to its decisions and this log. |
| V2-01 | contradiction | fix | CON-006 rejects a shared-host requirement where an end is one for which no item that is run is selected, as it rejects one where an end has no placement: `shared-host.v1` says that the two ends run on one machine, and such an end runs on none. PLC-013, `shared-host.v1` and the row "shared-host requirement" follow. |
| V2-02 | wrong | fix | NAM-024: an input that a later release adds to a Requirement contract states no `default`, so it is sent only where a document writes it, and an implementation built for the earlier release is sent no argument it does not know unless an author wrote one. CON-022, IFC-019 and the row "contract" follow. M45's fail-closed rule stands. |
| V2-03 | misleading | fix | As V5. |
| V2-04 | wrong | fix | PLC-023 does not hold back a read of a secret that PRV-026 makes to run the machine provider again: that provider's report is what gives a machine a state, so holding the read would keep every machine but `local`'s omitted after every restart. A read has no effect, and where it cannot reach its machine it does not succeed and the report counts as not received. PRV-026 and the row "unreachable, gone" follow. The verifier's other option, rejecting a provider whose secret Resource is not on `local`, would add a rule; PRV-026 keeps that as advice. |
| V2-05 | contradiction | fix | `inspect.v1`, the rows "inspect" and "the engine's ID for an invocation", two places in the core README and the overview say "each invocation whose outcome is unknown", as LIF-013 does. The core README's sentence on ruling 42 and the head of `lifecycle.yaml` record that time and keep their words. |
| V2-06 | contradiction | fix | The rows "result", "tool" and `Conduit's keys`, and the core README's section on results, say that -32602 with no tool result is the answer at the baseline revision, and that at a revision at which a tool result answers such a call, it carries the reason (IFC-012, IFC-025). |
| V2-07 | minor | fix | The row "part" defines a part of the engine as a second sense, written "a part of the engine" or "the part of it", and says that a part that reports a reason or ends with an outcome is always a part of a setup or a retirement. |
| V2-08 | misleading | fix | IFC-017 names the stop of a part of the engine, and excepts the run of a secret Resource that PRV-026 makes again and the run that TPL-018 asks for in a retirement. The rows "caller", `serving`, "runtime provider" and "the engine's read of a secret" add the stop of a part. |
| V2-09 | minor | fix | ACC-039, CON-031, LIF-015 and LIF-024 add a request to unregister the consumer, under LIF-017. |
| V2-10 | minor | fix | As V3. |
| V2-11 | minor | fix | IFC-022: the token is accepted over TLS at an `https` URL and without TLS at the `http` loopback URL that DL13 gives a run. DL13 says the same. |
| V2-12 | minor | fix | DL13 leaves to the engine only the key and the client identifier of its JWT assertion under RFC 7523. |
| V2-13 | undetermined | fix | REC-002: the record of a request that ACC-037 decides because whether the instance that its credential names still exists cannot be read holds no caller, and says that this could not be read. IFC-032 cites it. |
| V2-14 | undetermined | fix | IFC-001: where the tool list cannot be read, or does not hold the tool that a call names, nothing is sent, and the request, the hook or the part reports unavailable with a record that says why, as IFC-037 has it for discovery. |
| V2-15 | minor | fix | IFC-010 reads the protocol's -32021 for a client capability that the call did not declare as failed, as it reads -32602, since the protocol has a server answer so where acting would need that capability; OPS-007 excepts it for an implementation that carries Conduit's keys, and IFC-045 says why, as IFC-008 reads the -32021 that names Conduit's extension. |
| V2-16 | minor | fix | IFC-015 says that the engine gives a `remote` item no means to make an onward request, as a consequence of IFC-041, and TPL-017 no longer implies that a hook reached by `remote` makes onward requests. No rule is added on what DL13's authorization server issues, as M54 added no mechanism. |

Note of 2026-11-07: ruling 44 reversed V2-15, on finding 6-1-01 of
[round 6](../round-6/DECISIONS.md#the-six-findings), which an independent check confirmed. The
protocol has a server answer -32021 where "processing" a request needs a capability that it did
not declare, and says nothing of whether the server acted first, so the reason given for V2-15
does not follow from it. The error is now read as any other error, failed for an operation that
states no effect, is not a control and starts no model work, as IFC-033 tests, and unknown for
any other, except where Conduit's own contract binds the server to have done nothing: the -32021
that names Conduit's extension (IFC-008), and an implementation that carries Conduit's keys,
which answers so only where it has performed nothing for the call (IFC-047). IFC-007, IFC-010,
IFC-045, IFC-047, added, and OPS-007 changed together. The row above keeps its words as written.

### Counts

Counted with a script on the working tree after the verification: 452 promises, 202 of the
layer `language`, 207 `convention` and 43 `interface`; by level, 404 `must`, 44 `must not`, 3
`may` and 1 `should`. The two added, IFC-045 and IFC-046, are of the layer `interface`, so
`interface/implementation.yaml` holds 43. None was removed or moved between files, and no ID
was reused. [spec/README.md](../../../../spec/README.md#contents) and
[INDEPENDENT_BUILDS.md](../../../implementation/INDEPENDENT_BUILDS.md) give 452 and 43. The
implementation-defined list still holds 15 items, and the examples and the checker's rules are
as after the integration.

Changed: ACC-039; CON-006, CON-022, CON-031; IFC-001, IFC-003, IFC-007, IFC-010, IFC-015,
IFC-016, IFC-017, IFC-019, IFC-022, IFC-025, IFC-032, IFC-036, IFC-037; IMP-007; LIF-015,
LIF-024; NAM-024; OPS-005, OPS-007; PLC-013, PLC-023; PRV-007, PRV-025, PRV-026; REC-002;
TPL-017: 30 promises, and the item DL13. Each keeps its ID. Each changed YAML file has a line
in its head comment for this verification, and each changed Markdown file under `spec/` a
sentence in its history. In the core package the descriptions of `register.v1`, `inspect.v1`
and `shared-host.v1` changed. Outside `spec/`, the verification changed the tracker (V6's row
under "Waiting for the owner", V12), the re-audit record (V13), the choices of ruling 43's
verification (V11), `INDEPENDENT_BUILDS.md` (the counts) and the overview (V2-05).

Note of 2026-11-07: V6's row under "Waiting for the owner" was removed from the tracker when
the coordinator settled V6.

### For the owner

**V6. An instance that runs nothing, and the `nodeSelector` of its type and items.** M22 places
on `local` an Agent or a Resource for which no item that is run is selected, and PLC-002 checks
that placement under PLC-007 as any other. A template whose type states a label that `local`
lacks, such as `kubernetes.io/os: linux` on an engine on macOS, is then rejected even where it
selects only a `remote` item and runs nothing. Before M22 the machine provider placed it on a
machine that matched. Options:

1. PLC-007 leaves the type's and the items' `nodeSelector` out of the selector of an instance
   for which no item that is run is selected, since a `nodeSelector`, as decision 8a takes it
   from Kubernetes, says where a workload runs, and nothing of it runs. Such a template is
   accepted wherever `local` stands.
2. As now, with PLC-002 saying why: a type's `nodeSelector` is a placement need of every
   instance of the type, and an instance that runs nothing is placed on `local`, so `local` must
   match. A user whose type needs another machine gives a placement in the document, which
   brings back the unavailability with that machine that M22 removed.
3. PLC-002 places such an instance on `local` where `local` matches its selector, and otherwise
   asks the machine provider, as before M22.

Settled by the coordinator on 2026-11-07, under rulings 37 and 38, as option 1, and applied to
PLC-007 and PLC-002. Recommendation: 1. It removes a rejection that has no reason where nothing runs, which is
friction on one machine that the owner's principles rule out; it adds no exception to M22; and
it is consistent with V2-01, under which an end that runs nothing is taken to run nowhere.

The decisions of this verification that the owner may want to read first, since each settles
a behavior and not only a wording: V2-01, which rejects a shared-host requirement on an end that
runs nothing; V2-02, which forbids a `default` on an input that a later release adds to a
requirement; V2-04, which exempts the machine provider's reads from PLC-023; V2-15, which reads
the protocol's -32021 for a capability that the call did not declare as failed; and V7, which
lets a gate pass the engine's token on to the server.

Note of 2026-11-07: in V6 above, the sentence that records its settlement stands before the
recommendation; the settlement took the recommended option, 1. Ruling 44 reversed V2-15, as
the note under the table of this verification says.
