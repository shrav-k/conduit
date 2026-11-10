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
| [005](005-real-client-results.md) | The results of experiment 008 (A71): every run classified, wire, client and model apart |
| [006](006-i2-02-schemas-and-e05.md) | I2-02: authored schemas, the tool definitions, and E05 |
| [007](007-real-client-later-turn.md) | Experiment 008's follow-up: the harness extended to a later turn of the same session |
| [008](008-later-turn-results.md) | The results of experiment 008's follow-up, a later turn |
| [009](009-i2-03-packages-and-projects.md) | I2-03, first part: packages and projects, resolved from snapshots |
| [010](010-native-launch-harness.md) | The harness of experiment 009 (A4): wrappers that run Claude Code and Codex as Agents |
| [011](011-native-launch-results.md) | The results of experiment 009 (A4), native launch and identity |
| [012](012-i2-04-registration.md) | I2-04: registration and the resolving of instances |
| [013](013-i2-03-archives.md) | I2-03, second part: installing a package from its archive |
| [014](014-i2-05-access.md) | I2-05: the access decision and the tools an Agent is shown |
| [015](015-i2-06-requests-end.md) | I2-06: how a request ends, its repeats, and plain results |
| [016](016-i2-07-setup-and-conduits.md) | I2-07: setup, and the opening and closing of Conduits |
| [017](017-i2-08-records.md) | I2-08: records, inspection, and the bounds of repair |
| [018](018-plain-errors-check.md) | The check of two protocol errors, -32602 and -32020, before increment 3 sends any call again (decision 8) |
| [019](019-i2-09-s1-on-the-core.md) | I2-09: S1 whole on the core, the coverage of its promises, the first build's conformance document, and what increment 3 receives |

[The standing order of a verifier](verify.md), with its schema, is given to the fresh Codex
session that checks each order's result before the admin session reviews it.
