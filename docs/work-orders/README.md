# Work orders

Not binding. Each file here is one order that the admin session wrote for a Codex worker, before
it ran, as [work orders](../implementation/WORK_ORDERS.md) describes under
[ruling 48](../proposals/ruling-48/README.md). Beside each order is the JSON Schema of its
report. An order's results are its pull request and the record in
[work orders](../implementation/WORK_ORDERS.md#the-record).

| Order | What |
| --- | --- |
| [001](001-review-fixes.md) | The coordinator's fixes from the review of increment 1 and A70, and ruling 48, point 3 |
| [002](002-real-client-harness.md) | The harness of experiment 008 (A71): a stub of the presented server and a runner, in Rust |
| [003](003-increment-2-study.md) | A study for increment 2, the core on S1: S1 as documents, the promises and safety cases it touches, a format for vectors, and a sequence of orders |
| [004](004-i2-01-vectors-and-first-start.md) | I2-01: the vectors' format, the core's crates, and first start |

[The standing order of a verifier](verify.md), with its schema, is given to the fresh Codex
session that checks each order's result before the admin session reviews it.
