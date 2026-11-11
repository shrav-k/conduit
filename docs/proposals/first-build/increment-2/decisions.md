# Increment 2: the admin's decisions on the study

Not binding. The specification in [spec/](../../../../spec/README.md) alone binds. These are the
coordinator's decisions, as the admin session under
[ruling 48](../../ruling-48/README.md), on the proposal of work order 003 in
[plan.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md), made on 2026-11-09 under ruling 46, point 4. The owner can strike any of
them. None adds or removes a feature or changes what Conduit is for, so none goes to the owner.
Accepted by [ruling 58 (point 1)](../../ruling-58/README.md).

## The decisions

| # | Question | Decision |
| --- | --- | --- |
| 1 | The message history of S1 | A70's convention: one new file per post, through a second directory Resource served by the same filesystem server, with its limits stated (a post can be overwritten, and order is by name). Ruling 40: no service is invented for it |
| 2 | The packages the first vectors use | Preinstalled snapshots first, marked as such. Installation from an archive gets vectors of its own, with real bytes and hashes, before the build claims installation (PKG-004, PKG-022, PKG-026). The drafts' illustrative digests are replaced in anything that runs |
| 3 | The format of the vectors (A39), and the core's crates | The portable format of section 4: YAML cases under `conformance/`, each naming its layer, its promises and the first build's profile, whose documents stay ordinary Conduit files. Two crates beside `conduit-loader`: `conduit-core`, pure, and `conduit-conformance`, which runs the vectors and owns their file input. Every input that is not deterministic enters at the boundary, as section 5's table says |
| 4 | The first build's choices on the implementation-defined list | As section 5 recommends: DL2's contents digest is SHA-256 of the RFC 8785 form of the operation and its filled arguments; DL15's generated suffix is five lower-case letters or digits, retried on a collision; DL4 is unavailable at once; DL12 inlines nothing optional; DL8 has one source of records with an explicit export; DL10 is 128 levels of nesting, with outputs kept for the whole of a vector unless the vector tests their expiry; DL6 first start is one transaction from an empty state, and a restart never makes new identities; under DL2, state and records are kept before a dispatch is carried out. Each goes in the first build's conformance document |
| 5 | F1: what a successful repeat of a `plain` operation answers | Its content is kept and given again, as the outputs of an operation that declares them are, for as long as DL10 keeps outputs; after that a repeat reports conflict under OPS-034, as one of an operation with outputs does. A succeeded call is never dispatched again to recover its content. The fix goes in OPS-009, IFC-025 and DL10's wording, with a vector, in order I2-06 |
| 6 | F2: an Operation's root `$defs` key that has the form of a copy key | Rejected. A key of the form IFC-033 gives a contract's copy, a qualified reference, is reserved: a schema an author writes that uses one at its root `$defs` is rejected under IFC-033 when its document is installed or submitted. This keeps IFC-033's "members keep their keys" and needs no rewriting of an author's schema, which the study's option (b) would. The fix goes in IFC-033, with an E05 vector, in order I2-02 |
| 7 | F3: the vocabulary of records and the shape of inspection | Portable required-field projections, and a small vocabulary of event types documented for the first build: access-decision, request-ended, invocation-dispatched, invocation-ended, provider-call and object-change. Exact sets of objects and required fields are compared; a projection never hides an omission |
| 8 | The open safety questions on `plain` errors (IFC-010, IFC-046, OPS-003, OPS-018) | A focused check of how servers report `-32602` and `-32020`, by a work order, before the shell builds the paths that send a call again; the core follows the current table meanwhile |
| 8a | The outcome of that check, [work order 018](decisions.md#evidence-for-decision-8a), 2026-11-09 | Neither reading holds: the protocol gives no guarantee that a server which answers `-32602` or `-32020` has not acted, and its official SDKs let a handler answer either after acting. So IFC-010 treats `-32602` from a `plain` server as any other error of such a server: `failed` for an operation that has no effect, is not a control and starts no model work, and `unknown` otherwise; and IFC-046 sends a call again after `-32020` only for an operation that has no effect or that its contract declares safe to repeat, and otherwise ends it `unknown`. A rejection by the engine itself before it sends a call keeps its reason and has no outcome (OPS-005). Order 015 applies both, with vectors |
| 9 | How much of the Agent types increment 2 needs | Registration only. Their wrappers and launch, with every flag and helper that [the first build](../../../implementation/FIRST_BUILD.md#what-increment-3-needs) lists, are increment 3's |
| 10 | The sequence of orders | Section 7's nine orders, I2-01 to I2-09, in its order, each a work order of its own; the admin may split or join them as their results come in |

Decision 8a was narrowed by finding 2-07: after `-32020` the engine sends a call again only
for an operation with no declared `effect`, that is not a control and starts no model work;
any other ends `unknown`. No contract declares an effectful `plain` operation safe to repeat,
and none is added.

## What each order carries from here

- I2-01 starts from decisions 3 and 4.
- I2-02 settles F2 by decision 6.
- I2-06 settles F1 by decision 5.
- I2-08 uses decision 7.
- The check of decision 8 is written as its own order before increment 3's retry paths.

## Evidence for decision 8a

The focused check found neither error code alone proves that no effect happened. Normal SDK
validation rejects before the handler; low-level/handler paths can answer after acting.
SDK permission for such an answer does not mean every use conforms to MCP’s intended category.
The check covered protocol revisions 2024-11-05 to 2026-07-28 and these tagged sources:

| Repository | Tag | Commit |
| --- | --- | --- |
| TypeScript SDK | v2.4.0 | 951fe5603a7ec0b2dcb6a0b95d92cd86f93ad9e5 |
| Python SDK | v2.3.0 | 2118f14f8a19bc158d8a1cf90af58d85d187f849 |
| rmcp | rmcp-v3.9.0 | 5596a8a0bba2a1ec5519cc855cdadf23e4dcecdc |
| Reference servers | v2.3.0 | 36380287ed3b15d115b55f546617d723072b1296 |
| Reference servers (S1’s filesystem 2.1.0) | v2.2.0 | 194cc86e3ae6400958555bd4116849d822d69263 |

Filesystem 2.1.0 and 2.2.0 have no found path returning either code after a write. Their
registered tool wrappers return isError for mutation exceptions; stdio has no HTTP header
validation. This narrow result supplies no guarantee for other plain servers.

IFC-010’s rejected alternatives, verbatim from the check:

| Option | Consequence |
| --- | --- |
| A. Keep the unconditional failed classification | Relies on an unproved convention that every plain server rejects before an effect. The Python, rmcp and low-level TypeScript counterpaths remain possible. |
| C. Permit a narrower no-effect classification only with an explicit implementation guarantee | Requires a defined, verifiable contract or adaptation that distinguishes rejection before any effect; neither the number nor the human-readable error message supplies this. Adds a new qualification surface. |

IFC-046 option B also proposed a no-effect rejection guarantee for Conduit-aware
implementations. It was not adopted: finding [2-07](../README.md#2-07) dropped the declaration
exception because adding that contract feature is the owner’s decision. No such guarantee
is supplied by an error number, HTTP status, readOnlyHint, idempotentHint or message wording.

## Verification of the study’s sections 4 to 7

- Section 4’s portable case format, exact mandatory sets, diagnostics/keyword-target matching,
  causal records and failing unknown inputs are carried by conformance/README.md, its case
  schema and runner. The earlier example syntax is superseded by the executable format;
  the runner remains separate so fixture I/O stays outside the core.
- Section 5’s pure inputs, nondeterministic evidence, retained invocation state and opaque
  identity descriptor are carried by the core and the engine README’s current boundary.
  Decisions 3/4 and the profile carry the DL choices. The study recommended a monotonic revision
  counter; it was a proposal, not a requirement adopted by decision 4. Today the shell supplies
  UUID revisions. No behavioral change is made in this condensation.
- Section 6’s F1/F2/F3 are decisions 5/6/7 and findings 2-04/2-02; the error-code assumptions
  are corrected by 8a and 2-05 to 2-07. The scope/provider questions are decisions 1/2/9 and
  the code’s explicit boundaries, not undecided language behavior.
- Section 7’s sequence is decision 10 and the implemented I2-01 to I2-09 slices. Their remaining
  coverage work is in conformance/README.md, “Planned work”; shell/management limits are in
  engine/CONFORMANCE.md. Loader ports, boundary pairs, nested Resource replacement and real
  client evidence remain explicitly incomplete. No additional unresolved study requirement
  was found beyond those existing coverage and implementation entries in the tracker.
