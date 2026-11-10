# Increment 2: the admin's decisions on the study

Not binding. The specification in [spec/](../../../../spec/README.md) alone binds. These are the
coordinator's decisions, as the admin session under
[ruling 48](../../ruling-48/README.md), on the proposal of work order 003 in
[plan.md](plan.md), made on 2026-11-09 under ruling 46, point 4. The owner can strike any of
them. None adds or removes a feature or changes what Conduit is for, so none goes to the owner.

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
| 8a | The outcome of that check, [work order 018](../check-plain-errors.md), 2026-11-09 | Neither reading holds: the protocol gives no guarantee that a server which answers `-32602` or `-32020` has not acted, and its official SDKs let a handler answer either after acting. So IFC-010 treats `-32602` from a `plain` server as any other error of such a server: `failed` for an operation that has no effect, is not a control and starts no model work, and `unknown` otherwise; and IFC-046 sends a call again after `-32020` only for an operation that has no effect or that its contract declares safe to repeat, and otherwise ends it `unknown`. A rejection by the engine itself before it sends a call keeps its reason and has no outcome (OPS-005). Order 015 applies both, with vectors |
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
