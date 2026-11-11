# Work orders

Not binding. Each file here is one order that the admin session wrote for a Codex worker, before
it ran, as [work orders](../implementation/WORK_ORDERS.md) describes under
[ruling 48](../proposals/ruling-48/README.md). Beside each order is the JSON Schema of its
report. An order's results are its pull request and the record in
[work orders](../implementation/WORK_ORDERS.md#the-record).

As of 2026-11-10, orders 001 to 028 have admin records; order 029's record is pending.

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
| [020](020-i3-01-store-loop-identity.md) | I3-01: the shell's store, its loop and its identities |
| [021](021-i3-02-runner.md) | I3-02: the engine's own runner and the unchanged filesystem server |
| [022](022-i3-03-presented-server.md) | I3-03: the authenticated server that presents each consumer's tools |
| [023](023-i3-04-agent-types.md) | I3-04: the Agent types for Claude Code and Codex, as packages |
| [024](024-i3-05-closeout-fixes.md) | I3-05: the fixes from the first run with the real clients |
| [025](025-i4-01-change-operations.md) | I4-01: the operations and author commands S1's tasks of change need |
| [026](026-docs-entry-refresh.md) | The entry documents brought up to date, and instructions for cold readers |
| [027](027-i4-02-engine-fixes.md) | I4-02: retirement hooks, startup repair, inspection and Codex's collaboration tools |
| [028](028-i4-03-commands.md) | I4-03: reliable author commands, help, ended conditions and explicit package installation |
| [029](029-docs-after-rulings.md) | The documents after rulings 49 to 54, the settled Codex route and the two S1 author runs |

[The standing order of a verifier](verify.md), with its schema, is given to the fresh Codex
session that checks each order's result before the admin session reviews it.
