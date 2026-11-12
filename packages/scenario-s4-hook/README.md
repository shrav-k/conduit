This package serves scenario S4 and is not for general use. It targets Unix. The library also
supplies S9-SRV's reusable hook; local qualification does not qualify the engine's late replies.

## Configuration and repair

[conduit.yaml](conduit.yaml) exports [contracts.yaml](contracts.yaml)'s six contracts at
`example.org/conduit/scenario-s4-hook`, version `0.1.0`: two operations, two capabilities and
two Resource Types. The `hook` item runs `bin/scenario-s4-hook hook` (setup and setting tools);
`reader` runs `bin/scenario-s4-hook read`. Both are Conduit-aware stdio servers, declare the
implementation-interface extension at `v0`, and bind no environment or supports. Missing/other
extension revisions answer -32021 naming the requirement. Any filter answers unavailable with
the exact restriction and no outcome (IFC-026); a setup through a filtered Conduit cannot run it.

| Place | Required | Meaning |
| --- | --- | --- |
| `ledger` | yes, both Types | Absolute JSON Lines path with an existing parent directory; outside engine store. |
| `blockedBy` | no, hook only | Null or absolute path. A regular file, including a link to one, blocks setup. Missing paths/directories do not. Metadata errors fail without effect. Contents are never opened or copied. |
| `control` | no, hook only | Absolute existing directory owned by the server's effective user, with no group/other write permission. Enables both barriers. |

Every call reads its configuration from `io.github.shrav-k.conduit/config`. Malformed metadata,
configuration or arguments fail before any receipt/effect/change, naming the offending place.
IDs and revisions are opaque nonempty strings. Hook calls require an idempotency key.
`set-blocked-by.v1` requires `revision` and `blockedBy`, maps the latter to `/blockedBy`, and
answers succeeded, isError false, with no outputs or ledger write. The engine stores exactly
the value sent (REV-007); later calls read it with no new run. T1's M2 counts an author's repair
through a Conduit to the hook instance, calling this control with null or a corrected path.
Removing the file outside Conduit is an environment step, not an author repair.
The hook belongs to Resources such as Pool entries; TPL-019 gives a Pool no run.
Every attempt that proceeds produces its own effect; setup-hook declares no deduplication.
A second effect for one key is M3 evidence about the engine, never hidden here.

## Barriers v1

No control means no barriers. Otherwise the hook waits at `before-apply` after committed
received, then `before-reply` after committed ended. Each call ID consists of lowercase
letters/digits/hyphens, generated from PID, process start nanoseconds and a counter.
The server chooses `NAME = CALL.BARRIER` and never times out or gives up waiting.

1. Server atomically creates `NAME.reached`, new mode 0600. It contains one JSON object with
   `version: 1`, `package` (the supplied package name), `barrier`, `id` (call ID), and `call`
   (`id`, `instance`, `invocation`, `caller`, `key`, `revision`, `attempt`). Configuration is
   omitted. Reached acknowledges everything preceding it has committed.
2. Controller writes a JSON string `"proceed"`, `"drop"` or, at before-apply alone, `"fail"`
   under a temporary name in the same directory, ending in none of `.reached`, `.release`,
   `.acknowledged`, then renames it to `NAME.release`. Never write a release in place. The
   server reads only the final name. S4-H scans `*.reached` and matches the JSON call details.
3. Server commits a `released` line with barrier/verdict, then atomically creates
   `NAME.acknowledged`, 0600, `{"version":1}`, and only then continues. Its own temporary files
   also end in none of the handshake suffixes. Each handshake name is used once.

Polling every 5 ms is the only wait; no timer orders events. A held call holds no ledger lock
and blocks no other call. The engine's own timeout can turn a held call unknown.
Fail ends without effect and answers failed after before-reply. Drop performs nothing further,
records reply dropped and never answers; a before-apply drop first records ended/no effect.
Handshake I/O errors or invalid verdicts record control-lost and act as drop. Failed releases
have no acknowledgement. After a before-reply drop any already committed effect remains.

For a lost reply, release before-apply with fail and hold the ended no-effect call before-reply
until S4-H observes unknown, then release proceed. S4-LATE forwards that late terminal failed
answer to the core to establish the fence before explicit no-effect reconciliation. The server
knows no engine state. Onward requests are S4-SRV2b's; no engine changes/vectors are here.

Before answering the hook checks context.ct. Peer cancellation of a held attempt records reply
cancelled instead of released, and no answer leaves. rmcp 3.9.0 removes cancelled requests'
response slots and discards their answers. Thus engine cancellation after it stops waiting
prevents the late reply; S4-LATE must account for that path. Cancellation can also arrive
between the check and the transport send and discard an answer whose ledger says released.
Released means permission to send, not proof of delivery.

## Ledger and readback v1

Only this package writes the ledger; the engine never reads it. Creation uses mode 0600 and
syncs the parent directory. Each append takes an exclusive advisory flock, writes complete
lines in one write, fsyncs and releases the lock. Receipt counting/appending is one locked
transaction across processes. Effect and ended commit together. Ledger errors/short writes
stop with a protocol error, never claim no effect. Appending after a torn line first separates
it with a newline, preserving its invalid line number.

Every line has `version: 1`, `package`, `kind`, `id` (call ID), `instance`, `invocation`, `caller`,
`key` (string or null), `revision`, `attempt` (calls received for that instance/key, trace only:
IFC-006 sends no attempt number). Several instances/processes may share a ledger.

| Kind | Extra fields and meaning |
| --- | --- |
| `received` | Durable receipt before any work. |
| `effect` | `effect`, its unique ID; this complete ledger entry is the hook's setup work. |
| `ended` | `effect`, ID or null; `cause`, null or blockedBy/controller/gate; `diagnostic`. Own work has finished. |
| `released` | `barrier`, `outcome`: proceed/fail/drop; committed before acknowledgement. |
| `reply` | `outcome`: released/dropped/cancelled. |
| `control-lost` | `barrier`, `outcome: drop`, fixed `diagnostic`. |

One function reads only the ledger under a shared flock; missing files are errors, never
proof of no effect. `scenario-s4-hook readback LEDGER` prints the same JSON as MCP operation
`read-hook-ledger.v1` returns in structuredContent, optionally filtered by its input instance.
The full output schema is in contracts.yaml. The v1 readback object contains:

- `version: 1`, `attempts` in receipt order, and `invalidLines`, one-based malformed/incomplete
  line numbers (also counted outside an instance filter).
- Each attempt's common fields except kind, plus `work`: open/ended, `effect`:
  `{state: applied, id: ID}`, `{state: none, cause: CAUSE}` or `{state: not-known}`, and
  `reply`: released/dropped/cancelled/none.
- Only explicit ended-with-null-effect evidence establishes none. An open attempt without an
  effect line stays not-known. A committed effect line remains evidence even without ended.
  Invalid lines are reported, never silently dropped.

It states no outcome for a hook invocation or setup as a whole. A part with no effect does not
make a partially effective setup failed (OPS-022).

## Library interface v1 for S9-SRV

`scenario_s4_hook` has no repository dependency. These public Rust names are frozen as v1:

| Interface | Use |
| --- | --- |
| `ledger::Call` | Cloneable public id/instance/invocation/caller/key/revision/attempt/configuration; serde omits configuration. |
| `Call::line(package, kind, fields)` | Construct a common v1 line. |
| `ledger::append(path, &[Value])` | Exclusive-lock append/sync batch. |
| `ledger::receive(path, package, &mut Call)` | Assign receipt count and commit received under one lock. |
| `ledger::readback(path, Option<&str>) -> io::Result<Value>` | Shared-lock readback; S9 can extend the returned object with its own fields. |
| `ledger::new_id()` | Evidence ID generator. |
| `barrier::validate_directory(path)`, `barrier::publish(path, &Value)` | Check directory; atomically publish a new 0600 handshake file. |
| `barrier::wait(directory, package, &Call, ledger, barrier)` | Async acknowledged handshake; io::Result of Verdict::Proceed/Fail/Drop. |
| `serve_hook(package, Option<Gate>)` | Stdio hook service; package name appears in discovery, barrier and ledger. |
| `Gate`, `GateVerdict` | Arc of synchronous Send+Sync function &Call -> Proceed or Fail(String), called after blockedBy, before work; failure cause gate. |
| `HookServer::new(package, reader, Option<Gate>)`, `serve_reader(package)` | Handler and reader stdio service. |

S9 provides its own gate, repair Resource and readback fields via a path dependency, changing
nothing here. Gate functions must finish and avoid untracked effects. The binary has no gate.

## Build and qualification

Run in this directory under pinned Rust 1.97.1, one Cargo command at a time:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
cargo run --release --bin package
```

Tests drive the built binary with rmcp, full metadata and atomic controller releases. No held
call future is timed out or dropped: only the cancellation test sends notifications/cancelled.
Poll deadlines only fail broken tests. Schema tests explicitly report skipping when spec/ is
absent. The template-only installation fixture has no instances and dispatches nothing.
Release tests build the server executable; packaging requires it and starts no nested Cargo.
Ignored dist/scenario-s4-hook.tar.gz contains bin/scenario-s4-hook, conduit.yaml, contracts.yaml,
README.md; USTAR/gzip headers have fixed timestamp/UID/GID/modes. The companion .tar.gz.sha256
holds bare lowercase hex and newline. This is a host Unix binary archive.

D15 follows [S10's fields](../../experiments/021-s10-swapped-providers/README.md#measures):

| Field | Evidence |
| --- | --- |
| (a) Own operations/custom code — observed locally | Setup, blockedBy setting and ledger read; metadata checks, barriers, append/readback, deterministic packaging. Files/lines below. |
| (b) Delegation — observed locally | rmcp: MCP transport/discovery/cancellation; Unix: flock/metadata/rename/fsync; Rust crates: serialization/compression/hashing. No backing service. |
| (c) Persistent state — observed locally | Operator-chosen server-user-owned ledger outside engine store, plus controller files. Stop preserves committed evidence. Restart counts earlier receipts; open work stays uncertain. No automatic recovery/cleanup. |
| (d) Operator setup/recovery — locally qualified | Build/install, ledger parent directory, optional private controller directory/blocked file, reader/settings Conduits, explicit release/repair. No credentials/real service. Engine late forwarding and author recovery remain scenario work. |

Counts are nonblank lines excluding standalone comments; YAML separators count. Package/fault
preparation is counted apart from author work for M3. Shared library work counts once here;
S9 records its reuse. Generated embedded schemas and lockfile count separately.

<!-- PACKAGE_COUNTS_START -->
| Category | Files | Nonblank, noncomment lines |
| --- | --- | --- |
| Rust implementation and packaging | 5 | 692 |
| Manifests, contracts, fixture and build configuration | 7 | 288 |
| Generated embedded schemas | 1 | 192 |
| Qualification tests | 3 | 824 |
| README | 1 | 143 |
| Generated Cargo.lock | 1 | 1518 |
| Total | 18 | 3657 |
<!-- PACKAGE_COUNTS_END -->
