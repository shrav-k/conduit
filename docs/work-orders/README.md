# Work orders

Not binding. The admin writes each order before dispatch, as
[work orders](../implementation/WORK_ORDERS.md) describes under
[ruling 48](../proposals/ruling-48/README.md). Merged orders 001 to 036k and 060 to 066
have left the tree; their results, decisions and observations are in
[the record](../implementation/WORK_ORDERS.md#the-record).

## Orders still in the directory

| Order | What |
| --- | --- |
| [037](037-spec-short-form.md) | The short form in the specification |
| [038a](038a-spec-pending-and-sign-in.md) | Refusals, pending actions and sign-in |
| [039](039-build-local-names-and-expected-id.md) | Local names and expectedId in the build |
| [040](040-build-fleet-and-first-start.md) | Fleet and first start in the build |
| [067](067-condense-work-orders.md) | Move merged orders into their record |

## Shared and staged files

- [The shared report schema](004-i2-01-vectors-and-first-start.schema.json), used by every
  remaining order; its original name is kept.
- [The standing verifier order](verify.md) and [its report schema](verify.schema.json).
- [036b-staged/](036b-staged/README.md): exact patches and handoffs for later orders, including
  their vector impact. This directory stays until its consumers apply the patches.

Any `*-declared.yaml` files stay with the orders that use them. None is present at this step.
While an order is in flight, its admin Record stays in its file; at merge, the admin writes
its row serially in the record before removing that order and its own schema.
