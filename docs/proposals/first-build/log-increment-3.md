# Increment 3: the shell

Not binding. The [specification](../../../spec/README.md) alone binds. This log follows
[ruling 46](../ruling-46/README.md), point 4. The coordinator settles findings; the owner can
strike each settlement. A worker's implementation choices await the coordinator's review.
The increment's complete requirements remain in
[the first build](../../implementation/FIRST_BUILD.md#what-increment-3-needs).

## Findings

| ID | Class | Promises | Finding | What the build did | Settlement |
| --- | --- | --- | --- | --- | --- |
| 3-01 | contradiction | DL13, IFC-022, PRV-022 | Order 020's "no exp" conflicts with DL13's far-future expiration and [JWT-SVID section 3.3](https://github.com/spiffe/spiffe/blob/main/standards/JWT-SVID.md#33-expiration-time), which requires exp. The first round followed the order and failed to report the conflict; the verifier found it. | The correction issues exp at iat plus 100 calendar years and rejects expired tokens. | The admin settled it for far-future exp, with no change to spec/. The order's original instruction is superseded by this settlement. |

The unsupported public management paths already named in
[the handoff](increment-3-handoff.md#current-implementation-limits-to-resolve-before-exposure)
remain core implementation work. The shell propagates their embedding errors.

## I3-01: store, loop and identities (work order 020)

[The order](../../work-orders/020-i3-01-store-loop-identity.md) supplies the decisions.
The new [shell crate](../../../reference-model/crates/shell/src/lib.rs) supplies an Executor
method for each current intent, durable transitions, wait handles for pending requests,
completion correlation, opening recovery, output expiry and identity materialization.
It starts no implementation and presents no server. Only tests supply scripted executors.

### Implementation choices

- A process-long exclusive advisory file lease prevents a second opener from recovering a
  live shell. A mutex serializes transitions within the owner. Executors and notification
  callbacks run outside the mutex and may read State or submit their completions.
- Version 1 stores State and output-completion timestamps in an atomic JSON file. Timestamps
  live outside core State so record purge cannot erase retention ages. Each expiry is a
  separate core transition and commit. Size counts the compact UTF-8 retained outputs/content
  JSON, including overhead; age is checked on open and at transition boundaries, without an
  idle timer. Equal completion times sort by invocation ID.
  Store decoding allows document depth plus State's wrappers, rather than imposing serde's
  default 128-level JSON limit on the serialized State. Authored input still goes through
  the strict loader's limit.
- Evidence starts with 256 UUIDs/revisions/suffix candidates and retries exhaustion once with
  16,384, from unchanged committed State. First start gets its required seven revisions and
  two suffixes. Time is UTC from the system clock. UUIDs and suffix entropy come from the OS.
- One private authority file atomically retains the engine trust domain, PKCS#8 signing key
  and same-key CA certificate. Per-run materialization uses the required-name arrays from
  run.v1, preserving their role order, and refuses descriptors that differ. Public JWKS uses
  key ID store. Token audience is an array containing the configured full engine address.
  Tokens have iat, a fresh UUID jti and exp at iat plus 100 calendar years, as settled
  for finding 3-01. No credential bytes
  go back into a core input. RunIdentity has no Debug or Serialize implementation.
- Existing store directories/files must already have the required private modes; broader
  permissions are refused. The store's parent must exist. A missing authority in an
  established store is an error, rather than a new identity. No distributed/network
  filesystem locking or durability claim is made.
- Unserved calls report missing, inspection reports unknown, and unserved run/connection
  work reports failed to the core. Cancel intents acknowledge the absent transport because
  the core has already ended them and defines no cancellation completion input. Unknown
  kinds and mismatched executor responses stop with an embedding error.
- The CLI registers ordinary directories with the S1 snapshots preinstalled, then explicitly
  requests opening root's submitted Conduits and setup in Pool/Resource/Agent/Interface order.
  It pauses on the first built-in runner intent and prints only its kind, operation and
  invocation. No pending value is printed as a successful result. Inspection uses core
  inspect.v1 calls as root, preserving the core's authority refusals. Record export is JSON
  lines in source order, with gaps reported on stderr.
  First start supplies the three mandatory machine labels under PRV-014, mapping Rust's
  platform names to GOOS/GOARCH, and no custom label unless given. The order's "no labels"
  is read as no custom labels: omitting the mandatory labels is rejected by the core.
- The core exposes Transition.request_replies, containing its own final replies correlated
  by invocation and attempt, including filter refusals that remove an invocation or restore
  its previous result. The shell delivers them unchanged after committing. This replaces the
  first round's requests::finished_reply accessor, which could reconstruct an earlier success
  or lose a refusal. No access, repeat or lifecycle decision changes.
- Output accounting traverses previous snapshots of pending repeats. A new successful core
  completion starts a retention age; moving or restoring an existing result preserves it.
  The core's expire-outputs now removes retained results wherever the invocation holds them,
  including previous snapshots, under OPS-034 and DL10; invocation state is preserved.

### Evidence and limits

[Shell tests](../../../reference-model/crates/shell/src/tests.rs) exercise the S1 vector through
the store and scripted completions, source-order record export, concurrent requests and
pending waits, missing executors, output cap/age, store format/permissions/ownership, credential
verification and containment, and killed child processes on each side of dispatch. The S1
test supplies the vector's deterministic evidence only through a test-only facility; the
production entry point always generates evidence at the boundary.
Additional tests exercise larger evidence on allocation exhaustion, an unchanged commit
after both attempts exhaust, and store round trips with document depth plus State wrappers.
The correction round adds ticket-level regressions for a first-attempt filter refusal and
refusal of a repeat after success, cap enforcement while a previous result is held by a pending
repeat, and original retention age after restoration and during a pending repeat. Identity
checks require the far-future exp and refuse signed tokens with expired, missing or malformed
expiration claims.
The command-line test runs init twice, registers S1 until the runner wait, inspects and
compares exported JSON lines to the retained records. Identity checks independently verify
the JWT through its JWK and the root certificate's self-signature with that same public key.

The CLI's paused registration is deliberately left in progress until recovery or a runner
completion; another command opens the store and first recovers it unknown. Orders 021 and
022 supply persistent runner/server ownership. No real Agent, process runner, remote gate,
HTTP authentication, OAuth server or multi-machine restart is demonstrated here.
The implemented DL2, DL10 and DL13 choices are in
[CONFORMANCE.md](../../../reference-model/CONFORMANCE.md).
