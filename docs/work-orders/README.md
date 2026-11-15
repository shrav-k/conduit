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
| [151](151-private-names-and-secret-scan.md) | Private names and secrets checked before anything private is committed (the full review's order 2) |
| [152](152-ci-engine-shards-stand-alone.md) | The engine's CI stands alone, and CI builds every crate (the full review's order 3) |
| [153](153-model-account-checked.md) | the model-checking account, checked by a tool |
| [154](154-plan-apply-reference-corrected.md) | the plan-and-apply reference corrected against spec/ |
| [155](155-increment-4-log.md) | increment 4's log: the dogfood frictions and the Records' findings |
| [156](156-runner-stopped-after-end.md) | the runner reports a run stopped only once it has seen it end |
| [158](158-fresh-author-wording.md) | wording the fresh author reads at the pin: flags recorded with the plan, and CTR-013's citation corrected |
| [159](159-a5-stage-0-client-facts.md) | the two client facts order 050 needs (A5's stage 0) |
| [157](157-own-tools-reported.md) | an Agent's own tools reported as its Type declares them (the full review's order 15) |
| [160](160-mc2-rerun-077.md) | MC2 rerun on order 077's text, as ruling 78 leaves it (the full review's order 9) |
| [161](161-a78-stage-1-evidence.md) | A78's stage 1: its evidence and the engine's records of order 042 enter the tree (the full review's order 10) |
| [162](162-dispatch-deadline-cancels.md) | a dispatch deadline cancels its call before the call is reported missing (the full review's order 14) |
| [163](163-a24-at-entry.md) | A24 at entry: a new promise lands with a case that can fail (the full review's order 18) |
| [164](164-planner-difference-table.md) | the planner's difference table, and package contents by digest (the full review's order 23) |
| [165](165-by-rule-examples-public-routes.md) | the by-rule examples sent through the engine's public routes (the full review's order 25) |
| [166](166-conformance-pieces-messages.md) | the conformance pieces' last messages (follow-up to 071; after 163) |
| [167](167-a5-probe-signals-tested.md) | A5's probe: signals and the shim tested, and a signal never cuts a logout (follow-up to 159) |
| [168](168-tidy-after-153-155.md) | the points orders 153 to 155 left: G5.md, the tracker, and one difference recorded |
| [169](169-model-account-reads-index-pieces.md) | the model-checking account's tool reads the coverage index's pieces |

## Shared and staged files

- [The shared report schema](004-i2-01-vectors-and-first-start.schema.json), used by every
  remaining order; its original name is kept.
- [The standing verifier order](verify.md) and [its report schema](verify.schema.json).
- [036b-staged/](036b-staged/README.md): exact patches and handoffs for later orders, including
  their vector impact. This directory stays until its consumers apply the patches.

Any `*-declared.yaml` files stay with the orders that use them. None is present at this step.
While an order is in flight, its admin Record stays in its file; at merge, the admin writes
its row serially in the record before removing that order and its own schema.
