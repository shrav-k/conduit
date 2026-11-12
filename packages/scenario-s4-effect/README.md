# S4 effect servers

This package serves scenario S4 and is not for general use. It targets Unix. Its external
ledger stands for effects; `start-model-work.v1` records entries standing for the model work
an Agent would have begun. No model runs. Task support belongs to S4-SRV1b.

The package offers three Resource Types, each using one item of the same binary:

| Type | Item / first argument | Operations |
| --- | --- | --- |
| `effect-plain.v1` | `plain` / `plain` | `apply-effect`, `start-model-work` |
| `effect-keyed.v1` | `keyed` / `keyed` | `apply-effect.v1` |
| `effect-ledger.v1` | `reader` / `read` | `read-effects.v1` |

The aware items declare Conduit's implementation extension at `v0`. Plain declares no
extension, and accepts no Conduit metadata. Every operation accepts `label`: a lowercase letter or digit, followed by up to 62 lowercase
letters, digits or hyphens; only the reader allows its omission. Effect success returns
`structuredContent: {record: STRING}` and text naming the record. Aware successes also state
outcome `succeeded`, with `isError: false`. An unsupported filter returns `unavailable`
with its restriction and no outcome. Invalid metadata is diagnosed by key name, before work.

## Configuration and modes

All items learn configuration exclusively from their environment, never from `_meta`.
These places must be present in configuration, even where their value is null:

| Place | Environment variable | Items |
| --- | --- | --- |
| `/ledger` | `S4_EFFECT_LEDGER` | all |
| `/control` | `S4_EFFECT_CONTROL` | plain, keyed |
| `/mode` | `S4_EFFECT_MODE` | plain |

The ledger path is absolute, with an existing parent directory chosen by the operator or
S4-H outside the engine store. The control path, when supplied, is absolute, names an existing
directory owned by the process user, and is writable by neither group nor others. A null
`control` leaves its variable out, disabling barriers. Startup rejects invalid settings with
a fixed diagnostic naming the variable, without printing its value.

Plain `complete` commits both parts of a new record on every call. Plain `partial-error`
commits part 1, then returns JSON-RPC -32021 with
`data.requiredCapabilities: {elicitation: {form: {}}}`. The same behavior applies to both
plain tools. A partial effect stays unknown; it establishes nothing about the whole effect.
There is no repair, confirmation or reset operation: inspect the independent evidence and
reconcile through the engine's `reconcile.v1` where the evidence permits it.

Keyed calls use `(instance, caller, idempotencyKey, invocation)` from IFC-006. The instance
is the target. IFC-006 provides no sending instance; OPS-008's one invocation per scoped key
lets the invocation ID distinguish that scope. Neither the Conduit nor time nor trace attempt
count participates. A complete record is reused and noted as deduplicated; a record with only
part 1 receives part 2 alone. Lookup and missing-part writes hold one exclusive file lock.
No key means new work every time, including identical arguments. The ledger survives restart;
there is no separate cache or database to recover.

## Barrier handshake

Each call holds at `before-apply`, after its received line and before any entry, and at
`after-commit`, after all entries for its mode and before answering, including deduplicated
attempts. A held call holds no ledger lock and delays no other call.

Names are `CALL.BARRIER.reached`, `CALL.BARRIER.release`, `CALL.BARRIER.acknowledged` in
the control directory. S4-H scans reached files; it identifies calls by the file's JSON:
`{version: 1, package: ADDRESS, barrier: NAME, call: CALL, details: CALL_DETAILS}`. Details
hold `version`, `call`, `mode`, `operation`, `label`, `instance`, `caller`, `key`,
`invocation`, and `attempt`; plain's four metadata fields are null. Reached means everything
before that barrier is committed. Reached and acknowledged files are new, mode 0600, written
under temporary names ending in none of the three handshake suffixes and renamed into place.

The controller writes the literal UTF-8 verdict `proceed` or `drop` (an optional final newline
is accepted) to a temporary file in the same directory whose name does not end in `.release`,
then renames it to `CALL.BARRIER.release`. Direct writes to a release file are unsupported.
The server polls only that final file and never times out. Polling is the only wait: no timer
orders events. The engine's timeout may leave a call unknown while it is held here.

`proceed` continues; `drop` commits a dropped reply receipt, acknowledges it, performs
nothing further, and never answers. Unwritable or unreadable barrier files and invalid verdicts
act as drop, with a `control-lost` line. Acknowledgement follows any release receipt's fsync,
including the reply at after-commit. If acknowledgement itself cannot be written, control is
lost, the receipt becomes dropped, and no answer is sent. Failed control does not bypass a hold.

Before answering the server checks `context.ct`: a cancelled request gets `cancelled` in
place of `released`. rmcp drops answers after peer cancellation. Cancellation can still arrive
between this check and transport send and drop a `released` answer; a released receipt alone
does not prove receipt by the engine. S4-H must combine receipts with engine evidence to call
something a lost success. There are no tasks or task cancellation calls in this version.

## Ledger v1

One JSON Lines file, owned and written only by this binary, created mode 0600. Every append
takes an exclusive advisory `flock`, appends its whole JSON line in one write, calls fsync,
and releases the lock. The creation also syncs the parent directory. A line is committed
when fsync returns. Both entry writes occur under the same lock but have separate fsyncs;
a stopped process can leave one committed part. Appending after a torn last line first adds
a newline, preserving that invalid line for readback. The engine never reads this ledger.

Every line has `version: 1`, `kind`, `call`, `mode`, `operation` (the local operation name
including `.v1`), `label`, `instance`, `caller`, `key`, `invocation`, and `attempt`. Calls
use lowercase letters, digits and hyphens: process ID, start epoch nanoseconds and an atomic
counter. Plain metadata is null. Attempt counts are computed under lock from received lines
with the same keyed tuple, or plain operation and label, and survive restart. They are trace
evidence, never an IFC-006 attempt field.

| Kind | Additional fields |
| --- | --- |
| `received` | none; written before the first barrier |
| `entry` | `record`, `part` (1 or 2), `of: 2` |
| `deduplicated` | `record` |
| `reply` | `reply`: released/dropped/cancelled; `answer`: full `{result: MCP_RESULT}`, `{error: JSON_RPC_ERROR}`, or null before application |
| `control-lost` | `barrier`, `cause` (fixed diagnostic) |

Extra kind-specific fields do not change the common fields. A later reply for the same call
supersedes an earlier one only if acknowledgement failed. No barrier wait holds this lock.

## Readback v1

`scenario-s4-effect readback LEDGER` prints the same JSON that the reader tool returns in
structuredContent. Both use one function taking only a shared lock on the external ledger.
They consult no engine store, engine record, process state or server answer. The reader tool
does not write ledger lines. The complete output schema is in [contracts.yaml](contracts.yaml).

The object holds `version: 1`, `label` (the filter, or null), `records`, `calls`, and
`invalidLines` (one-based line numbers, including a torn last line). Records hold `record`,
`operation`, `label`, sorted unique `parts`, and `state: whole|partial`. Calls, ordered by
their call ID, hold all received fields except `version` and `kind`, plus `parts` (the
`{record, part}` entries this call wrote), `deduplicated`, `reply`, and `answer`. No reply
line means `reply: none`, `answer: null`: still held or stopped, never treated as ended.
Filtering selects calls and records by label, but invalid line numbers cover the entire file.
A label with no record yields that label and empty records, without a failed/absent assertion.

A whole record with a released success shows an answer was released. Where the engine kept
that success and the client lacks it, the client lost a success. A dropped/cancelled reply or
an open call was never answered by the server, so the engine's outcome is unknown. Partial
records report only their parts, never whole-effect failure or absence (S4 Q5).

## Build and qualification

Run inside this directory, with Rust 1.97.1:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
cargo run --release --bin package
```

Cargo commands are serial. Tests drive the built binary through rmcp with full request
metadata. Controllers wait for reached and acknowledged files and atomically publish releases;
they do not cancel held calls except in the cancellation test. Concurrent attempts use two
processes sharing the ledger. Readback covers partial, whole, open, dropped and cancelled
receipts, and torn lines. Output schemas are validated. Declaration validation explicitly
skips when the repository's spec is absent. Packaging tests compare archive bytes twice.

`package` requires the already built server alongside it; it never invokes nested Cargo.
It writes deterministic USTAR/gzip `dist/scenario-s4-effect.tar.gz`, containing only
`bin/scenario-s4-effect`, `conduit.yaml`, `contracts.yaml`, `README.md`, and a SHA-256
sidecar holding bare lowercase hex plus newline. Build output and dist are ignored.

The installation fixture pins the address and version with `sha256:PLACEHOLDER`, replaced
in a temporary copy by the archive digest. Its three Resource templates register without
instances: submit opens and sets up nothing. Qualification of dispatch through Conduit is
S4-H's work. This package introduces no engine change and no vector.

## D15 and preparation counts

D15's fields follow [S10's measures](../../experiments/021-s10-swapped-providers/README.md#measures).
This is package and fault preparation counted apart from S4 M3 author work.

| Field | Evidence |
| --- | --- |
| (a) Own operations and custom code — observed | `src/lib.rs`: 386 counted lines for three operations, metadata checks, receive/apply and receipt/barrier orchestration; `src/ledger.rs`: 133 for locked writes and independent readback; `src/control.rs`: 60 for validation, atomic publication and polling; `src/main.rs`: 30 for environment/stdio lifecycle and CLI readback; `src/bin/package.rs`: 57 for deterministic packaging. Tests exercise these functions through the binary. |
| (b) Delegated work — observed locally | rmcp handles MCP stdio, discovery and cancellation routing; Unix flock/fsync and atomic rename supply file coordination and durability; tar/flate2/sha2 supply archive encoding and digest. No real service or model is contacted. |
| (c) Persistent state — observed | The external operator-selected ledger, owned by this binary, is retained across stop/restart. Control files are harness evidence; use a new private directory for each run. The engine store is separate. Preserve the ledger to preserve deduplication. |
| (d) Operator setup and recovery — locally qualified | Build and install the package, provide ledger/control paths and mode, register templates, and explicitly create instances for scenario runs. No credentials or remote gate are needed for these local command items. Reconcile only from independent evidence; production isolation and real-client recovery are untested. |

Count nonblank lines excluding standalone comments; YAML document separators count. Include
every package source once, excluding ignored target/dist. Keep declarations, implementation,
qualification, documentation and generated lockfile separate; there is no generated schema
copy: discovery embeds and parses the operation contracts themselves. No author task, model
request, person intervention or recovery time was measured here.

<!-- PACKAGE_COUNTS_START -->
| Category | Files | Nonblank, noncomment lines |
| --- | --- | --- |
| Rust implementation and packaging | 5 | 666 |
| Manifests, contracts, fixture and build configuration | 7 | 161 |
| Qualification tests and installation script | 4 | 959 |
| README | 1 | 161 |
| Generated Cargo.lock | 1 | 1518 |
| Total | 18 | 3465 |
<!-- PACKAGE_COUNTS_END -->

Configuration: 3 Types, 3 items, 6 environment bindings, 3 distinct configuration places
(all required where present, control nullable), 3 operations, 3 capabilities, 4 operation
mappings, no external package dependency, no supported requirement. The installation fixture
adds 1 package pin (address/version/digest) and 3 templates, with no instances or grants.
Provider concepts: Resource, Type, capability, operation, implementation item, manifest,
package, configuration, run, invocation, caller, idempotency key, revision, outcome, reason,
filter, restriction and reconciliation (18). Technologies: Rust/Cargo, MCP stdio, Unix file
locks and durability, JSON Lines, JSON Schema, atomic file barriers and USTAR/gzip/SHA-256 (7).
Client-specific concepts: none. The README's format declarations are v1 and frozen for S4-H.
