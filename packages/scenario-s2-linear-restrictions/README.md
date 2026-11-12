# S2 Linear restriction contracts

This contracts-only package serves [scenario S2](../../experiments/013-s2-shared-login/README.md)
and is not a package for general use. It has no implementation, service or stub. S2-L1's
Conduit-aware implementation and S2-L2's adapter both pin this package by digest. The unchanged
plain server qualified for D01 keeps its own operation and input names: S2-L2 declares those
when it pins that server (PKG-020). These operations describe the adapter and aware server.

The address is `example.org/conduit/scenario-s2-linear-restrictions`, version `0.1.0`.
[conduit.yaml](conduit.yaml) exports the six contracts in [contracts.yaml](contracts.yaml).
The two capabilities keep read-only implementations eligible: `issue-reading.v1` contains
`read-issue.v1` and `list-issues.v1`; `issue-writing.v1` contains `update-issue.v1`. A Conduit
has one capability (CON-001), and an offered capability needs every operation implemented
(TPL-004). No Type, default implementation or core contract is added.

## Inputs and answers

Read takes `issue`; update takes `issue` and at least one of `title` and `description`.
An omitted edit field stays unchanged; a null description clears it. Title is a string,
including the empty string. List takes a required integer `limit` of at least 1, with no cursor,
returns distinct issues in ascending ASCII identifier order, and applies scope before limit.
It returns neither a count nor a cursor; missing issues are omitted and an empty list is legal.

Read and update return `{issue: ...}`; list returns `{issues: [...]}`. Every issue has exactly
`identifier`, `title`, `description`, `state` and `updatedAt`. Description is text or null for
none, state is an unconstrained service state name, and updatedAt is a date-time with an offset.
Update returns the issue after the change and changes only title and description, with the
service's consequent updated time; it never moves the issue or changes its team or parent.
All declared fields in an answer are required; extra fields are rejected. Relations, parent,
sub-issues and comments are excluded. Issue mentions in text remain unchanged as content.

Every identifier, including filter inputs and returned identifiers, uses the same case-sensitive
pattern `^[A-Z]+-[0-9]+$`: one or more ASCII uppercase letters, a hyphen and one or more ASCII
digits, such as `SYN-123`. A companion `not` pattern `[^A-Z0-9-]` rejects all other characters,
including a final newline that ECMA-262's `$` alone could accept (SCH-016). No normalization,
case folding or extra constraint on digit count is applied. `issue-scope.v1` requires a nonempty
array `issues` with unique identifiers and a `deadline` in RFC 3339 date-time form, with its
offset, as the core's `expires` is. Neither input has a default or is optional (CTR-005).

Each operation sets `openWorldHint: true` because it reaches an external service. Read and list
set `idempotentHint: true`: they have no effect. Update sets it false: assigning the same text
may advance the service's updated time again. This is a tool hint, not deduplication (CTR-002,
IFC-033). No operation starts model work. Read and list declare no `effect`; update declares an
`effect`, outputs and no `deduplicates`.

## Filter clauses and their basis

`issue-scope.v1`'s description is the package's behavioral contract. Its F1-F6 labels identify
clauses for qualification; the issue/deadline semantics are package behavior, not core rules.

| Clause | Package behavior | Basis in Conduit |
| --- | --- | --- |
| F1 | Judge the call's arguments and current filter inputs on every call; different Conduits are judged apart, never from an earlier answer. | CTR-005 and CON-018 place this non-core filter on a Conduit; IFC-018 sends that call's filters; IFC-019 requires applying each. |
| F2 | Refuse an out-of-scope read or update before lookup, independent of existence. An absent in-scope issue fails. List only existing in-scope issues before limiting. | IFC-019 applies filters to actions and returns; IFC-026 permits refusal only before any part is performed. Scope and absence semantics are this package's. |
| F3 | Deliver only the five issue fields above; preserve text mentions without traversing them. | CTR-002 declares outputs; IFC-019 filters returns. The projection and content distinction are this package's. |
| F4 | Refuse at or after deadline by the implementation's clock at receipt, comparing instants across offsets. Judge once: an allowed call completes past the deadline. S2-H may set this clock. | IFC-019 leaves this predicate to the filter. OPS-024 redispatches filtered reads, so each receipt gets a fresh decision. |
| F5 | Carry the filter on any onward request; the onward recipient applies its own receipt-time and scope checks. | IFC-020 names the served invocation; the engine carries its filters under ACC-017. CON-022 and ACC-015 check support. |
| F6 | Perform nothing on a forbidden call; answer refused with the received filter item and isError true, with no outcome or issue output. An undeclared input in a supported requirement version answers unavailable the same way. | IFC-019 and IFC-026 give the two grounds and wire form. No additional cannot-apply ground is invented. |

F6 uses the assigned `io.github.shrav-k.conduit/reason` and
`io.github.shrav-k.conduit/restriction` keys in result `_meta`. The restriction is the received
filter item with `requirement` and `arguments`; `isError: true` is on the result. The result
has no `io.github.shrav-k.conduit/outcome`. A forbidden request performs no part of its action;
an in-scope missing issue instead has the ordinary `failed` outcome. No extra package-specific
denial ground is defined. Implementations must list this requirement in `supports` for every
filtered operation (CON-022); the plain server supports no filters (PKG-020).

## D05 restriction cases

| Case | What decides it |
| --- | --- |
| Changed authorization data | F1/F2 recheck the current filter inputs and the call's own arguments at once. An earlier answer grants nothing. |
| Keyed repeat | Read/list have no effect and start no model work: OPS-024 dispatches a successful filtered repeat, and F1-F4 decide afresh, including expiry. Update declares an effect and outputs: OPS-034 returns conflict for a successful filtered repeat, with no dispatch and no retained output delivery. This branch is Conduit's, not a filter evaluation. OPS-009 governs other successful repeats. |
| Output delivery | F2/F3 limit lists and project each issue; F6 returns no issue output on refusal. OPS-024 and OPS-034 prevent cached outputs bypassing the current filter. |
| Onward calls | F5 and IFC-020/ACC-017 carry the filter. This package makes no calls: D05's onward-call case remains unexercised unless S2-L1 or S2-L2 actually makes one and qualifies it. |
| Revocation | Closing the Conduit is Conduit's decision, not a filter predicate (ACC-011). ACC-034/039 govern an already dispatched invocation and later dependent calls after close, as S2 V5 requires. Deadline expiry is F4 and is separate from close. |

This filter is not a bound (ACC-029). It limits no right to open other Conduits: a separate
bound may permit an unfiltered Conduit within its Pools, targets and operations (ACC-030, S2 V2).
It confines nothing outside Conduit; restrictions remain mediated (ACC-027). Holding the shared
login once and keeping its credential from the Agents belong to S2's implementations and secret
delivery evidence, not to these schemas. Admission proves no D01/D05 runtime result.

The engine's FLT-D and SEC-D fixtures are written independently from S2's README, not copied
from this package. Where they differ, S2's README wins and this package must be fixed.

## Admission and packaging

With the pinned Rust 1.97.1 toolchain, run these from this directory:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

The only binary is the packaging tool. It writes
`dist/scenario-s2-linear-restrictions.tar.gz` and its `.tar.gz.sha256` file. The archive contains
only conduit.yaml, contracts.yaml and README.md, in that order, at its root. Fixed USTAR metadata,
gzip headers and entry order make identical source bytes produce identical archive bytes. There
is no platform binary inside. The printed digest is `sha256:` followed by lowercase hex; the
digest file contains the hex alone and a newline. Install using `ARCHIVE#SHA256` (the hex after
`#`) and pin the address, version and `sha256:` digest in the manifest (MAN-005, PKG-004).

Tests check exports, capability membership, schema compilation, positive/negative arguments,
answer projection and deterministic archives. Following the local-machine package's YAML test,
they validate the manifest and every document against `spec/schemas/`, including the embedded
schema profile. That comparison explicitly skips when `spec/` is absent; compilation and sample
checks still run. `tools/check_schemas.py` does not read packages. No portable vectors are added.
