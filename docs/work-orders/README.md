# Work orders

Not binding. The admin writes each order before dispatch, as
[work orders](../implementation/WORK_ORDERS.md) describes under
[ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md). Merged orders 001 to 036k and 060 to 066
have left the tree; their results, decisions and observations are in
[the record](../implementation/WORK_ORDERS.md#the-record).

## Orders still in the directory

Work order 222 removed the other finished orders; git keeps them at [`b49e8629`](https://github.com/shrav-k/conduit/tree/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders).

Each order file's first line is its index entry, which
`python3 tools/check_docs.py --work-orders` prints as a table. Orders 048a7, 048a8 and 050o,
which the coordinator's cuts of 048a-6 and 050 plan (order 117), are not yet written; they have
the rows below, without links, each saying what it comes after. On 2026-11-19 the
owner joined the last four parts of 048a-5 in pairs, as orders 048a5ef and 048a5gh, and moved
048a7, 048a8 and 051b3, the carried return, off 0.2.0's path and the dogfood path, to after the
dogfood, beside the keyed release (ruling 88).

| Order | What |
| --- | --- |
| 048a7 | an accepted return passed on: on a retry, PND-013's first three checks, a page login's code passed to `sign-in-hook.v1` in its retry, and `signInLogin`'s `returned`; after 048a-5gh and 048a-6, deferred after the dogfood, beside the keyed release (ruling 88) (planned; the coordinator's split, open to the owner's strike) |
| 048a8 | a redirect return: PND-013's fourth check, the value rebuilt and passed to `sign-in-hook.v1` in 048a-7's retry, and `error` in `signInLogin`; after 048a-7, deferred after the dogfood, beside the keyed release (ruling 88) (planned; the coordinator's split, open to the owner's strike) |
| 050o | the managed connection for a Claude sign-in, built in the Interface; after 051b-3 merged, and so after the dogfood (ruling 88), and 050b merged (planned; the coordinator's split, open to the owner's strike) |

## Shared and staged files

- [The shared report schema](004-i2-01-vectors-and-first-start.schema.json), used by every
  remaining order; its original name is kept.
- [The standing verifier order](verify.md) and [its report schema](verify.schema.json).
- [036b-staged/](036b-staged/README.md): exact patches and handoffs for later orders, including
  their vector impact. This directory stays until its consumers apply the patches.

An order that changes a case or a case's fixture writes its declaration here as
`NNN-declared.yaml`, and its pull request commits it: CI checks every vector change against
the declarations that a pull request adds or changes
([reviewing vector changes](../../conformance/README.md#reviewing-vector-changes)).
A merged order's file at its last commit is its Record (ruling 90, point 4).
041a-declared.yaml and 209-declared.yaml stay for the tool and the orders that read them.
