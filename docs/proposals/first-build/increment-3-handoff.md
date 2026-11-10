# What increment 3 receives

Not binding. The [specification](../../../spec/README.md) alone binds. Work order 019,
2026-11-09. This is a handoff of the implemented increment 2 boundary, not a new shell
contract or a claim that a real Agent ran. The [first build](../../implementation/FIRST_BUILD.md#what-increment-3-needs)
and the [admin's decisions](increment-2/decisions.md) supply the requirements. Implementation
choices and pending DL items are in [CONFORMANCE.md](../../../reference-model/CONFORMANCE.md).

## Calling the core

The API is [conduit_core::step](../../../reference-model/crates/core/src/lib.rs):

```rust
pub fn step(state: &State, input: Input, context: &Context) -> Result<Transition, String>
```

`State` is serializable: started, objects, records, allocated_ids, record_gaps, invocations,
installed_packages and preinstalled_snapshots. It includes the invocation index, held contexts,
setup progress and retained results; records alone cannot reconstruct these. `Context` holds
an optional built-in Project, core_package, meta_prefix, templates, record_source and
core_environment. Construct it from the specification/names and core documents as the
[runner](../../../reference-model/crates/conformance/src/lib.rs) does, not from copied literals.

`Evidence` is `{time, ids, revisions?, suffixes?}`: supplied RFC 3339 time, unique UUIDs in
consumption order, revision values and five-character generated suffix candidates. Exhaustion,
reused IDs, malformed trusted inputs or unsupported operations are embedding errors, not
invented Conduit reasons. The caller's state is immutable if a transition fails.

`Transition` is `{state, decisions, records, external_intents, reply}`. Each decision holds
`action, promises, implementation_choices, facts, affected`; DL items are separate from promise
IDs. `records` is the new batch; `state.records` also contains the retained batch. Persist the
new state and records as one transaction **before** executing an intent or returning a final
reply (DL2, REC-001). Serialize competing changes against the state they read. A pending reply
is internal scheduling information, not a successful MCP result.

The public `Input` enum uses kebab-case `kind` and rejects unknown fields. Every variant is
listed here so a shell order can distinguish semantic APIs from wire requests:

| kind | Fields besides kind | Use |
| --- | --- | --- |
| first-start | evidence, machine: `{hostname, labels, state}` | Empty-store bootstrap; repeat preserves identities (FST-001/002/005, DL6). |
| resolve-project | snapshots, builtin, project | Pure catalog/project resolution; no registration/fetch (PKG-006, SUB-008/010). |
| register-project | snapshots, builtin, project, evidence, request | Directory/project submission, with its original manifest and established actor/management route (SUB-001/003/008/010, ACC-033). |
| install-archive | archive, snapshots, builtin | Pure archive admission/catalog publication from supplied fetching completion (PKG-004/022/026); not a complete install.v1 invocation. |
| invoke | evidence, request, key? | Normal current-authority request and invocation entry; engine operations supported below execute internally (ACC-001/036, OPS-008/023/026). |
| complete | evidence, invocation, attempt, completion | Complete the named dispatch attempt (OPS-001/007/013). |
| cancel | evidence, invocation, confirmation? | Trusted cancellation notification; forwards cancellation and normally ends unknown (OPS-027). |
| setup-report | evidence, invocation, report | Report an outstanding runner/provider/connection part (TPL-012, PRV-007/009). |
| recover | evidence | End interrupted invocations unknown without reissuing them (IMP-007). |
| expire-outputs | invocation | Explicit retention boundary, never deletion of invocation state (OPS-034, DL10). |
| list-tools | identity | Consumer-private exact current list (IFC-012/038). |
| tool-definition | consumer, conduit, operation | Publish one held tool definition; used by schema vectors (IFC-033). |
| export-records | unreadable? (record IDs) | Export retained records and gaps; unreadable is read-availability evidence (REC-003/005/006). |
| access | evidence, request | Access-only verdict; no invocation. Never reuse a permit after state changes (ACC-036). |
| check-authored | environment, scope, documents | Pure authored schema admission (SCH-001/015/019). |
| evaluate-schema | environment, scope, schema, value, mode | Pure schema/default evaluation in held context (SCH-009/010/015). |
| submit | evidence, request, documents | Legacy protected-template submission test boundary, not the general project path. |
| request | evidence, request, object | Legacy resolved lifecycle refusal test boundary, not a consumer wire API. |

There are two distinct request structures. `Invoke`/`Access` uses
`{identity, tool, arguments, serving?, readable?, authority_readable?}`, where identity is
`{kind: instance, id}`, `{kind: none}` or `{kind: unreadable}`. The last two booleans default
true and are trusted availability facts. The shell verifies proof, establishes existence,
extracts serving/key metadata and supplies them; a consumer cannot assert those facts or an
original caller. The lower-level `Request` used by project registration and legacy inputs is
`{sender, caller, conduit, target, operation}` with resolved IDs and a qualified operation.
Here target is the actual Conduit target, not a Resource named in arguments (REC-002).

`Project` is `{manifest: value-or-null, documents: [ordinary loaded values], directory:
absolute-URL-or-null}`. Preserve the original manifest for SUB-008 hashing; expanded paths
belong only to resolution. A snapshot is `{source, address, version, digest, project}`, explicitly
preinstalled-snapshot where applicable. An archive input is
`{name, address, version, digest, directory, completion}`, with completion
`{status: complete, bytes: [u8...]}` or `{status: unavailable}`. Installed packages retain a
snapshot plus a virtual files map of file bytes/mode, directory or link target. Runner file
aliases, captures, fixture patches and observation recipes are **not** core or wire inputs.

## Intents and their completions

The complete current intent inventory comes from
[requests.rs](../../../reference-model/crates/core/src/requests.rs) and
[lifecycle.rs](../../../reference-model/crates/core/src/lifecycle.rs). Nothing executes inside
the core. The shell must correlate replies by both invocation and attempt, and setup reports
by outstanding phase/part. No intent here requests an autonomous repeat or model turn.

| kind | Exact outer fields besides kind | Shell action and returning input |
| --- | --- | --- |
| dispatch | invocation, attempt, target, implementation, call | Execute the supplied tools/call against the selected implementation. Return complete with the same invocation/attempt. A setup hook uses this shape too, with its own invocation and no Conduit metadata (TPL-017, ACC-018, IFC-001/006/007). |
| refresh-tools-and-call | invocation, attempt, target, implementation, call | Refresh parameter headers/tool information and send the safe read once within that attempt. Return complete. Do not generalize it to effectful writes or SDK automatic retries (IFC-046). |
| builtin-runner | operation, invocation, caller, subject, inputs | Inspect or start the subject's run. Return setup-report, not complete for the outer setup (PRV-007/009). |
| provider-call | operation, invocation, caller, subject, inputs, provider, config, revision | Call the selected runtime provider without creating a request invocation. Translate its response to setup-report; run success requires its URL (PRV-005/007/009). |
| connect | part, invocation, subject, implementation | Establish the selected remote connection; return setup-report with connected/part/outcome. No real remote/gate connection has been tested (IFC-017, PRV-025). |
| cancel | invocation, attempt | Forward the transport's cancellation for that attempt. The cancellation transition has already ended it; a later result cannot settle unknown (OPS-027, OPS-013). |
| cancel-setup-parts | invocation, subject | Cancel every outstanding run/connection part for that setup; begin no later part. The core has ended the cancelled setup. Late reports are ignored (OPS-027, TPL-012). |

`implementation` retains its selected item and held context/source; use its target-held
selection, not a new name lookup. `call` is `{method: "tools/call", params: {name, arguments,
_meta?}}`. For a plain filesystem server the tool is read_text_file, list_directory or
write_file; only the operation's arguments cross that boundary. No Conduit keys, config,
revision or identity are sent to it (IFC-007). For a Conduit-aware implementation, params._meta
holds the names-file-prefixed instance, operation, invocation, caller, conduit, filters,
config, revision and optional idempotencyKey. A setup hook omits conduit. Do not send the
consumer's serving key onward; it is interpreted by the engine (IFC-020).

Runner/provider `inputs` always include instance and machine status. An inspect-run.v1 intent
has just those. A run.v1 intent additionally has item, environment, packageArguments and
identity. The identity here is a **materialization request**:

```json
{"materializeFor":"instance UUID","environmentNames":["CONDUIT_URL","CONDUIT_IDENTITY_DIR"],"fileNames":["ca.crt","jwks.json","engine-id","token"]}
```

Those illustrative names are read from the run contract; the shell must use that source.
The shell constructs the actual run.v1 identity values/files, keeps them out of records,
and supplies the runner's permitted environment (PRV-022/023/024, DL13/16). The core has not
created credentials or a private directory. The filesystem Package argument binds the held
root string; the shell must install/run the item and verify its artifact, not treat a config
value as a process already running (MAN-010, PRV-007).

Completion inputs have these shapes:

```json
{"kind":"complete","evidence":{"time":"supplied instant","ids":[]},"invocation":"UUID","attempt":1,"completion":{"kind":"result","result":{}}}
```

Instead of result, completion may be `{kind: protocol-error, code: integer, data?: value}`
or `{kind: missing}`. Plain content is interpreted by IFC-010 and preserved on success;
Conduit-aware results must carry a valid outcome/reason envelope and valid outputs. Missing,
malformed and unconfirmed effects end unknown. In particular, a plain -32602 is unknown,
not proof of failure; -32020 on an effectful operation is unknown and never a resend.
Work order 018's [check](check-plain-errors.md) and admin decision 8a supersede the study's
original safety assumption (OPS-003/018, IFC-010/046).

A setup report is one of these values, wrapped with kind: setup-report, evidence and invocation:

```json
{"kind":"inspect-run","state":"stopped"}
{"kind":"run","outcome":"succeeded","url":"provider URL, when required"}
{"kind":"connected","part":"selected implementation name","outcome":"succeeded"}
```

Run state is running/stopped/unknown; outcomes are succeeded/failed/unknown. A report not
matching an outstanding phase is an embedding error; a report for an ended setup is ignored.
A cancellation's optional confirmation is a trusted implementation result: only valid
nonplain confirmation of failed establishes no effect; otherwise unknown remains.

## The tool contract presented to consumers

The shell presents MCP at the engine's address with authenticated consumer identity
(IFC-012/022/042). Call list-tools for that identity; publish exactly those tools using
standalone schemas, target-held contracts and annotations (IFC-033). A's list is project
read/list plus history post/read/list; B also has project write. Required operation arguments
are path for read/list and path/content for write. A history post is a new file, not an
enforced append: overwriting, file authorship and lexical ordering retain the
[fixture's limits](../../../conformance/fixtures/s1/README.md).

For tools/call, pass the established identity, tool/arguments and optional serving/key into
invoke. Current authority is checked on every request and repeat; guessed/cached names grant
nothing (ACC-036). A normal reply is the core's MCP result with content, isError,
structuredContent as applicable, and names-file-prefixed outcome **or** reason and invocation
where applicable (IFC-025). A pending semantic reply stays internal while the shell waits.
An unpresented tool is a baseline protocol error with refused in error.data, no invented
successful tool result. The shell owns JSON-RPC IDs, version-dependent envelopes, capability
negotiation, cancellation ownership, HTTP/authentication/Origin checks and transport timeouts.

After a committed state change, call `access::tool_list_changes(before, after, context)`.
It returns `{consumer, method: "notifications/tools/list_changed"}` for changed private
lists. Send only to that consumer's subscribed connections; this helper is not an intent
emitted by step. List replies declare private cache scope and listChanged. The core chooses
no ttlMs; the shell must document it and test clients rather than infer their model catalog
from server output (IFC-038, DL10/12).

## Work the shell's orders can take

These are work items, not accepted new implementation choices. Each requires its own evidence.
The numbered needs come from the first build; A4 ran as experiment 009, whose limits remain.

| Item | Work and evidence required |
| --- | --- |
| H01: transactions/restart | Implement durable state/records, serialize concurrent calls, execute only committed intents and test crash before/after dispatch. Recover unknown without blindly replaying. Record retention/limits and deadlines as DL2/6/8/10 choices (REC-001, OPS-008/018, IMP-007). |
| H02: runner/filesystem | Start the unchanged plain filesystem MCP server for each directory, bind held roots, inspect runs and return real setup reports. Qualify MCP Roots so it cannot widen configured reach; state registry support, environment and process lifetime/restart rules (needs 1/8, PRV-003/007/023, DL16). |
| H03: missing core transitions | Before offering full management tools, complete or explicitly bound the unsupported core operations described below. Keep access/records/outcomes in the core; never implement a second lifecycle decision in the shell. |
| H04: identity/HTTP | Implement token proof, identity files/helpers, loopback server, private lists/notifications, permitted capabilities, baseline errors and cancellation; show actual calls from both assigned identities, not debug connection claims (needs 6/9, IFC-022/036/038/042/045, PRV-022/024, DL13). |
| H05: wrappers | Give each runnable Agent Type an MCP wrapper with an explicit operation marked startsModelWork and an authorizing Conduit. Setup starts no model work. Count wrapper/provider work separately from the author project (need 1, IMP-001/002, PRV-007, TPL-019). Base Agent fixtures are registration-only. |
| H06: session boundaries | Keep one conversation per Agent. Claude uses streamed turns; Codex resumes the same thread in a new process per turn unless new evidence changes that choice. Apply access changes at a turn boundary; record both the engine list and each client's own list (need 2, experiment 008, IFC-012/013/038). |
| H07: configuration isolation | Claude: setting-sources, mcp-config and strict-mcp-config; Codex: ignore-user-config and strict-config. Use private empty working directories away from shared data and other identities. Prove settings/hooks/instructions exclusion with sentinels in authorized test fixtures (needs 3/8/10, experiment 009). |
| H08: own reach | Default off: Claude account connectors, SendMessage, ListAgents, RemoteTrigger and incoming cross-session messages; Codex apps disabled. Template inputs explicitly opt in per Agent. Independently prove Codex apps-off and Claude incoming-message refusal; startup events alone did not show either (needs 4/10, ruling 48). |
| H09: login/helper/approval | Owner's shared login remains the ruled interim choice until A4/A5 isolation. Establish which login each client actually uses without credential disclosure. Read token from identity file with headersHelper/http_headers_helper; Codex helper receives the directory as a command argument because it lacks the run environment and HTTP env_vars is rejected. Set one engine-server approval in each client's own terms (needs 5/6/7/10, experiment 009). |
| H10: end-to-end evidence | With real Agents, show exact per-client tools, A refusal, B read/write, independent close and explicit restoration, and one recorded end per request. Report mediated restrictions and every route around the engine. Keep server-wire, client-event and model reports separate (increment 3 done-when, ACC-027, REC-001). |
| H11: later S1 work | Increment 4 adds real history tasks, T4-T6 change/repair, author/provider measurements against A70 and the fresh author. Do not treat this core's scripted text or experiment stubs as those results. |
| H12: coverage and fixtures | Port the manifest's explicitly unvectorized boundaries, qualify release compatibility before T5, and carry out the lean-vector proposal in the increment log. These are distinct from undecided language findings. |

[Experiment 008's initial results](../../../experiments/008-real-client/results-2026-11-09.md)
showed a newly opened tool did not reach either model within that turn; its
[later-turn results](../../../experiments/008-real-client/results-2026-11-09-later-turn.md)
showed use on the next turn, with Codex reinitializing MCP on resume. Claude still attempted
a closed tool and was locally rejected. Notification delivery is not proof of model catalog
refresh. [Experiment 009](../../../experiments/009-native-launch/results-2026-11-09.md)
showed successful assigned-identity calls after two failed Codex helper configurations; its
final successful matrix did not prove every isolation, messaging or login property. H06-H09
carry those specific limits; this order ran no client or experiment.

## Current implementation limits to resolve before exposure

The general dispatcher executes set-up.v1, open.v1, close.v1, register-replacement.v1,
register.v1, replace.v1, inspect.v1, purge.v1 and reconciliation. Consult the code for exact
preflight limits. Full retire.v1, unregister.v1, retire-with-resources.v1, declare-lost.v1,
controls and install.v1 are not complete public invocation paths: an unsupported engine
operation returns an embedding error. Some safety cases supply retirement/name-reuse facts;
they do not execute retirement or stopping a run. Archive installation is a semantic API
with no access/dispatch/record workflow, and fetch produces no intent. The shell must not
advertise these as working merely because core contracts list their names. This is unfinished
implementation against existing promises, not permission to change tool exposure promises;
H03 needs a core work order before claiming the complete management interface.

There is no stop-run intent/completion yet, despite PRV-008/TPL-009 being in the study's
inventory. Remote/provider variants are scripted boundaries, not qualified providers. E08
supplies an already-approved trusted broker request; the trusted adapter's approval logic is
still provider work. Secrets, TLS/gates, distributed fencing and complete controls/retirement
recovery remain beyond this S1 core slice. The manifest gives explicit reasons for these
limits. No shell order should fill a missing language answer without logging a finding under
ruling 46; unsupported code alone does not establish a language gap.
