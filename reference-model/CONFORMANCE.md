# First-build conformance choices

Not binding. The [specification](../spec/README.md) alone binds. This document states the
choices implemented through increment 2, under the admin's
[decisions](../docs/proposals/first-build/increment-2/decisions.md), and distinguishes them
from choices the shell must still make. It is not a declaration of whole-language conformance.
The executable [profile](../conformance/profiles/first-build.yaml) and
[coverage manifest](../conformance/manifest.yaml) name the evidence. A vector demonstrates
its stated slice of a promise; a coverage reason is not a passing vector.

## Implemented choices

| DL item | First build's choice | Where it is shown |
| --- | --- | --- |
| DL2: contents | SHA-256 of RFC 8785 JSON holding the qualified operation and default-filled arguments; caller/sender/target/key scope is separate. Invocation state, held contexts and retained results are independent of records. | `requests-key-scope`, `requests-defaults`, `records-purge`, [requests.rs](crates/core/src/requests.rs) |
| DL2: dispatch boundary | `step` returns a proposed state, appended records and intents atomically. The embedding must commit state and records before executing any intent. No dispatch occurs inside the core; actual durable persistence remains shell work. | `s1-whole`, `requests-restart-interrupted`, [core interface](crates/core/src/lib.rs) |
| DL2: attempts and late results | Attempts start at 1 and increase on an eligible caller repeat. Completion identifies both invocation and attempt. A stale attempt or ended invocation answers `ignored`; a late response cannot settle unknown. A parameter-header repair is once within the same attempt, only for IFC-046's read-only case, with no general effectful resend. | `requests-unknown-write`, `requests-repair-read-success`, `setup-close-in-flight`, [requests.rs](crates/core/src/requests.rs) |
| DL3: waiting, implemented portion | There is no autonomous clock or actor-reply timeout in the core. Pending lasts until supplied completion, cancellation or recovery. Duplicate/late completion is ignored. No actor-reply provider has been qualified. | `requests-key-scope`, `requests-unknown-write`, `requests-cancellation`, [requests.rs](crates/core/src/requests.rs) |
| DL4 | An unreadable identity/authority or state needed to decide access yields unavailable immediately; no waiting queue. | `access-identity`, `access-current-tools`, `records-authority` |
| DL5: reports, implemented portion | Time, canonical hostname, labels and run reports are supplied values. First start keeps the supplied local machine report. Setup asks for run inspection and interprets running/stopped/unknown; observation does not revise the definition. No polling cadence is chosen here. | `first-start`, `setup-run-unknown`, `setup-review-conditions-builtin`, `setup-review-conditions-provider` |
| DL6 | First start is one atomic empty-store transition. A repeated first start preserves identities and creates nothing. Allocation exhaustion/reuse is an embedding error and leaves the input state intact. Recovery ends interrupted work unknown without reissuing it; no repair is inferred. | `first-start`, `first-start-repeat`, `requests-restart-interrupted`, [boundary tests](crates/conformance/tests/core_boundary.rs) |
| DL7: archives | Gzip-compressed POSIX USTAR, without GNU/PAX extensions. UTF-8 paths; files, directories, symbolic links and hard links to earlier regular files. At most 40 link expansions. Last member at a path wins except replacing a nonempty directory or using a non-directory parent. All paths/targets are checked, including ignored documents. | `archives-link-expansions-39`, `archives-link-expansions-40`, `archives-link-expansions-41`, `archives-hardlink`, `archives-symlink`, [archive fixtures](../conformance/fixtures/install-archives/README.md), [archives.rs](crates/core/src/archives.rs) |
| DL7: storage and submission | Installed packages retain a virtual directory with bytes/modes in `State.installed_packages`; no compressed cache, removal or mirror API is implemented. Directory URL and fetched bytes enter at the boundary. The runner supplies directory documents for an established actor; registration installs nothing. A preinstalled snapshot explicitly says so. | `archives-program`, `archives-s1`, `packages-relative-fetch-directory`, `registration-s1`, [packages.rs](crates/core/src/packages.rs) |
| DL8: records | One ordered CloudEvents source, `urn:conduit:first-build`. Six event types: access-decision, request-ended, invocation-dispatched, invocation-ended, provider-call, object-change. Object-change actions and provider made/ended phases are in the profile. Export explicitly returns ordered records, including records for removed objects. No live record-feed cursor is implemented. | `first-start`, `records-inspection`, `records-unreadable`, `setup-review-provider-records`, [records.rs](crates/core/src/records.rs) |
| DL8: inspection and purge | Inspect keeps id/revision/packages/declared/status/records separate, omits template status, and reports unresolved invocations separately. Gap metadata is conservative across all known source gaps. Purge removes records strictly before its instant and retains permanent source/ID certificates. Export uses gaps/complete; inspection uses recordGaps/recordsComplete. Purge never alters the invocation index or retained results. | `records-inspection`, `records-purge`, `records-purge-succeeded`, `records-input-reference`, `records-unreadable` |
| DL9 | There is no extra portable ordering rule among diagnostics of the same reason. The implementation returns its deterministic traversal order. OPS-023's ordering between different reasons still binds. Vectors compare the required rule/location facts rather than validator prose. | `registration-s1-rejected-orders`, `access-routing`, `loader-schema-attribution`, [case assertions](../conformance/README.md#assertions) |
| DL10: structural limits | 128 levels of nesting (portable floor 64). No stated byte limit for files or archives, or count limit for documents/files in a submission. This does not promise unlimited physical memory. | `loader-depth-limit`, profile `DL10`, loader `MAX_DEPTH` |
| DL10: retained results | Retain outputs and succeeded plain content for the entire vector unless expiry is an explicit input. No key expires and no invocation index is compacted. Expiry drops retained results, not invocation state. Reconciliation retains no invented outputs; eligible repeats follow OPS-024/034. | `requests-plain-content`, `requests-plain-content`, `records-purge-succeeded`, [requests.rs](crates/core/src/requests.rs) |
| DL12: tool schemas | Inline no optional contract references. Copy reached contracts once, with escaped local roots and finite recursive definitions; target-held contexts choose releases. | `e05-local`, `e05-two-contexts`, `e05-recursive`, `e05-siblings`, `e05-escaping`, [schemas.rs](crates/core/src/schemas.rs) |
| DL12: result content | Preserve successful plain MCP content alongside structuredContent. For other successes use structuredContent without an optional serialized text copy. Errors carry a status text block; invalid/conflict also carry diagnostic structuredContent. | `s1-whole`, `requests-plain-content`, `requests-declared-outputs`, [result envelope](crates/core/src/requests.rs) |
| DL13: identity boundary only | Core identities are established instance, none, or unreadable. Consumer-supplied caller attribution is not trusted; onward attribution comes from the served invocation. Run intents name the materialization subject and required environment/file names read from the core contract, never credential bytes. This implements no cryptographic proof. | `access-identity`, `records-broker`, `s1-whole`, `setup-base` |
| DL15 | Generated suffixes are five lower-case letters/digits, supplied by the boundary and retried on collision. Authored and generated names share the ruled grammar; unused suffixes are not consumed identities. | `registration-generated-collision`, `registration-s1`, [registration.rs](crates/core/src/registration.rs) |

D19 is a measurement convention under DL2/DL8, not a DL promise: compact UTF-8 JSON bytes
for records, gap metadata and invocation arrays separately, with per-step signed growth and
per-invocation bytes/key status/outcome. Held contexts and retained results count. The runner
prints these measurements; they are not a persistence format or compaction policy.

## Choices still required

| DL item | What has not been chosen or implemented here |
| --- | --- |
| DL1 | Ordering of otherwise unordered/cyclic Resource retirement. Complete public retirement is outside the current core slice. |
| DL2 | Durable storage mechanism, transaction/serialization discipline between competing callers, and crash tests at the persistence/dispatch boundary. |
| DL3 | Actor-reply waiting policy and delivery while the server cannot receive; no such provider runs here. |
| DL5 | OS hostname discovery/override, machine report intervals/deadlines and voluntary runtime inspection intervals. |
| DL7 | Physical package store, removal/mirror controls, command/API directory entry point and complete install.v1 authorization/records/fetch orchestration. Archive admission alone is implemented. |
| DL8 | Live feed/cursor delivery and physical record storage. |
| DL10 | Tool-list ttlMs, actual call/retire-hook deadlines, production output retention and storage-exhaustion behavior. A vector lifetime is not a production retention policy. |
| DL12 | Supported MCP revisions beyond the baseline, transport envelopes and accepted Origin values. |
| DL13 | Trust root, signing/verification, credential IDs/issuer/expiry/audience, identity files, token helpers, refresh, gate proof, TLS and authorization-server integration. |
| DL16 | Built-in runner environment/platform rules, supported Package registry types, restart policy, child process lifetime and identity delivery. The fixture's npm Package item is no runner implementation. |
| DL17 | Multi-machine restart/partition/fencing. No distributed configuration or guarantee is demonstrated by this core. |

DL11 and DL14 are removed identifiers, not unanswered choices. The
[increment 3 handoff](../docs/proposals/first-build/increment-3-handoff.md) assigns the next
shell work. Any choices still pending must be documented and evidenced before the build
claims their behavior. The [increment log](../docs/proposals/first-build/log-increment-2.md)
records the accepted implementation choices and language findings separately.
