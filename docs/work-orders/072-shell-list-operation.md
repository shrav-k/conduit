# Work order 072 (LST): list.v1 and the stubbed operations through the shell

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

The shell refuses four operations that the core answers.

- Order 040 built `list.v1` in the core. `engine/crates/core/src/records.rs` handles it (lines
  8-18, and the branch at line 280), and its vectors are under `conformance/conventions/list/`.
- The core answers `lift.v1`, `answer.v1` and `sign-in.v1` with C11's temporary stub
  ([spec batch 1's plan](../proposals/spec-batch-1/README.md), C11, lines 310-313). The stub is at
  `engine/crates/core/src/requests.rs:901-915`. It ends the request as unavailable, with the text
  "not built yet, order NNN": 042, 047 and 048a. Through `reject` and `end` (lines 487-538 and
  459-485), it leaves a `request-ended` record with that reason (OPS-005).

The shell's list of the engine operations it executes, `EXECUTABLE_ENGINE_OPERATIONS` in
`engine/crates/shell/src/server.rs` (lines 39-52), names none of the four. Order 040 did not
change `server.rs`. So:

- `unfinished()` (lines 92-132) counts all four as unfinished;
- root's tools for them are presented, such as `management.list.v1`, `system.list.v1` and
  `access.lift.v1`, because root's Conduits list them (`spec/core/templates.yaml:102-124`,
  `:126-139`, `:141-159`). But every call answers JSON-RPC -32603, "operation ...#NAME is
  unfinished" (line 401). It leaves no record, and no caller sees the listing or the stub's answer;
- `preflight()` (lines 133-153), which `conduit serve` runs first, refuses a store in which a
  consumer other than root holds an open Conduit that lists any of the four. The templates
  `pool-manage` and `fleet-manage` (`templates.yaml:164`, `:193`), for example, list `list.v1` and
  `answer.v1`.

The vectors run the core alone, so none of them showed this. The promises it breaks:

- OPS-036: through an open Conduit that lists `list.v1`, the engine lists;
- OPS-019: listing and lifting are management verbs, so each ends in one of the defined ways and
  leaves a record;
- OPS-005: a request answered unavailable carries that reason in its result and in its record;
- MAN-012: the core package's implementations are the engine itself. Its item `engine` implements
  all four (`spec/core/conduit.yaml:172-193`).

This order makes the shell pass the four operations to the core. The core then lists, or answers
with its stub. Only `declare-lost.v1` stays unfinished in the shell, because the core does not
handle it: it would fall through to an external intent (`requests.rs:1158-1160`). The order also
adds a test that fails when the shell's lists stop covering the engine's operations, so that the
next operation added to the core package cannot be missed the same way. The coordinator's plan
lands it before the S1 rerun.

You start from `main` after order 041b merged. The line numbers here are from `main` at cfd7b23.
041b moves some of them, so find each place by its name.

- While it was in flight, 041b's tree added `retire-with-resources.v1` and
  `retire-with-agents.v1` to the executable list. It also rewrote the paragraph of
  `engine/CONFORMANCE.md` that step 5 edits.
- Order 070 (E1) also edits `server.rs`: it changes how `unfinished()` publishes. It also edits
  `CONFORMANCE.md`'s DL2 storage row, which this order does not touch. If 070 has merged, keep
  its form of `unfinished()`. If 070 changed how the executable list is held, so that it is no
  longer a constant array of operation names, stop and report.

This order changes no vector and no part of the core. It is small: four entries in a list, a
one-entry table, two new tests, two sentences and a log row. Do not build a registry of
operations: a later order does.

Read:

- OPS-036, OPS-019 and OPS-005 in `spec/conventions/operations.yaml`;
- MAN-012 in `spec/promises/manifest.yaml`, and IFC-012 in `spec/interface/implementation.yaml`;
- `implementations.engine.implements` in `spec/core/conduit.yaml`, and `list.v1`'s contract in
  `spec/core/operations.yaml` (line 1099);
- in `engine/crates/shell/src/`: `server.rs`, `tests.rs` and `server_tests.rs`, which `tests.rs`
  includes as its module `http` (lines 1258-1259);
- the core's stubs, `engine/crates/core/src/requests.rs:901-915`, and how `list.v1` ends:
  `requests::finish` (`records.rs:540`, `requests.rs:1178-1239`).

## What to do

1. **Check the starting point.** Change nothing yet.
   - The core package's item `engine` implements 19 operations.
   - On your tree, expect the shell's executable list to name 14 of them:
     - `install.v1`, `retire.v1`, `unregister.v1`, `set-up.v1`, `open.v1`, `close.v1`;
     - `register-replacement.v1`, `register.v1`, `replace.v1`, `inspect.v1`, `purge.v1`,
       `reconcile.v1`;
     - `retire-with-resources.v1` and `retire-with-agents.v1`, from 041b.
   - Expect the other five to be absent: `list.v1`, `declare-lost.v1`, `lift.v1`, `answer.v1`
     and `sign-in.v1`.

   If the executable list or the 19 operations differ from this, stop before changing anything,
   and report the difference: the admin corrects this order. Moved line numbers are not a
   difference.

2. **The failing test first.** In `server_tests.rs`, add the test
   `root_reaches_list_and_the_lift_stub_over_mcp`. It serves the S1 store and calls root's tools
   over MCP. Set up the server as `s1_http_consumers_authority_records_and_private_t7_changes`
   (line 169) does:
   - `fixed_shell`, then `server::preflight`;
   - root's client token, from `shell.issue_client_token`;
   - `server::serve` with a `ServingExecutor` over `crate::MissingExecutor`, built as
     `server_tests.rs:994` builds it, because none of the calls dispatches anything;
   - `ready`.

   Let `root` be `actor(&shell, "root")`. Then make three calls with
   `server::client(temp.path(), None, Some(TOOL), ARGUMENTS)`, in this order:
   - **`management.list.v1` with `{}`.**
     - The result is not an error: `isError` is false.
     - `structuredContent` holds the arrays `objects`, `packages` and `tombstones`. It holds no
       key that `list.v1`'s output schema does not name; the only other key the schema names is
       `pending`. (OPS-036: no count, total or cursor.)
     - `objects` holds root's instance: an item whose `id` equals `root`, with `kind`
       `Interface`, `name` `root`, `template` false and a `state`. Root's template has the same
       kind and name (`templates.yaml:78-82`), so kind and name alone prove nothing.
     - The ending is durable (OPS-019). `_meta` holds `{prefix}outcome` "succeeded" and a string
       `{prefix}invocation`, where `prefix` is `shell.context.meta_prefix`. Deserialize the reply
       into `CallToolResult`, as `call_tool` in `server.rs` does, and call `durable_ending`
       (line 141): it checks the `invocation-ended` and `request-ended` records.
   - **`management.list.v1` with `{"kind":"Interface"}`.** The same ending checks hold. Every
     item of `objects` is an Interface, and root's instance is among them, as above.
   - **`access.lift.v1` with `{"id":"00000000-0000-4000-8000-000000009999"}`**, the arguments
     that `tests.rs`'s `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders`
     uses (OPS-005).
     - The call returns a result, not a JSON-RPC error.
     - `isError` is true. `_meta` holds `{prefix}reason` "unavailable".
     - The result's text contains "not built yet, order 042".
     - Across the call, the store's records gain exactly one `request-ended` record whose
       `data.reason` is "unavailable".

   Cite the promises in comments, as `records.rs:300` does: OPS-036 and OPS-019 at the list
   calls, OPS-005 at the lift call.

   Shut the server down as that test does (lines 504-509). Run the new test on the unchanged
   tree. It must fail at the first call with the shell's refusal: an error that names `list.v1`
   and says "unfinished". Report the test's name and that message. If it fails in any other way,
   stop and report.

3. **The fix.** In `server.rs`:
   - Add `"list.v1"`, `"lift.v1"`, `"answer.v1"` and `"sign-in.v1"` to
     `EXECUTABLE_ENGINE_OPERATIONS`; its length becomes 18. Make the constant `pub(crate)`, so
     that the tests can read it.
   - Write one comment above it. It says that the list names the core package's operations that
     the shell passes to the core, and that it includes the operations the core answers with a
     temporary stub, unavailable and naming the order that builds each (OPS-005).
   - Beside it, add
     `#[cfg(test)] pub(crate) const UNFINISHED_ENGINE_OPERATIONS: [(&str, &str); 1]`. It lists
     each operation of the core package that the shell does not pass to the core yet, with the
     order that builds it. Its one entry is
     `("declare-lost.v1", "none yet: ROADMAP.md S3, AUDIT.md A35")`. No numbered order builds
     `declare-lost.v1`: `ROADMAP.md`'s scenario S3 needs it, and `docs/AUDIT.md`'s action item
     A35 says its vector is pending.
   - Write one comment above the table. It says that an order that builds one of these
     operations moves it to `EXECUTABLE_ENGINE_OPERATIONS`, and that a test in `tests.rs` checks
     both lists against the core package (MAN-012).
   - Change nothing else in `server.rs`: not `unfinished()`, not `preflight()`, and no message.

   Run the test of step 2 again. It passes. So does
   `s1_http_consumers_authority_records_and_private_t7_changes`: root's unfinished tools, at its
   lines 486-497, still include `management.declare-lost.v1`. If it fails, stop and report.

4. **The coverage test.** In `tests.rs`, add the test
   `every_engine_operation_is_executable_or_unfinished_with_its_order`. Cite MAN-012 in a
   comment.
   - It reads the engine's operations from the core package that `context()` (line 20) loads.
     The core package's manifest is `Context::builtin` (`engine/crates/core/src/lib.rs:51`), a
     `Project` whose `manifest` (`engine/crates/core/src/packages.rs:17`) `boundary::context`
     fills (`engine/crates/shell/src/boundary.rs:38` and `:95`).
   - Take the manifest's `implementations.engine.implements`. Remove the core package's address
     and `#` from each entry; use `core_package` from the context. Do not write the 19 names
     into the test.

   It asserts that:
   - no operation appears twice, within either list or across the two;
   - the two lists together name exactly the engine's operations. An operation the core package
     adds that neither list names fails the test, and so does an entry the core package does not
     name;
   - every unfinished entry names an order: either an order number, which is three ASCII digits
     and at most one lowercase ASCII letter, such as `042` or `048a`, or exactly the placeholder
     `none yet: ROADMAP.md S3, AUDIT.md A35`.

   Its failure message names each operation at fault, and says which list it is missing from or
   stale in.

   Then show that the test can fail:
   - remove `"list.v1"` from the executable list, and change its length from 18 to 17;
   - run the test, and see it fail, naming `list.v1`;
   - put both back.

   Report the message.

5. **CONFORMANCE.md.** In `engine/CONFORMANCE.md`, the paragraph that begins "H03 is bounded
   explicitly" lists the operations that public engine execution supports.
   - Add `list.v1` to that list.
   - Add one sentence after it. It says that the shell also passes `lift.v1`, `answer.v1` and
     `sign-in.v1` to the core, which answers each unavailable, naming the order that builds it
     (042, 047 and 048a; OPS-005).
   - Leave the sentence that names `declare-lost.v1` as unfinished as it is.

   Change nothing else in the file.

6. **The finding.** Increment 4 is the open increment (`docs/proposals/first-build/README.md`,
   line 47). Add one row to [its log](../proposals/first-build/log-increment-4.md). The log is a
   Markdown table: write the row as one line, and each cell as prose, as rows 4-01 to 4-18 are.
   Its ID is the next free one: 4-19 on `main` at cfd7b23, or the next if your tree already has
   a 4-19. The cells:
   - **Class:** defect of the build
   - **Promises:** OPS-036, OPS-019, OPS-005, MAN-012
   - **Finding:** Order 040 built `list.v1` in the core, and the core answers `lift.v1`,
     `answer.v1` and `sign-in.v1` with a stub that names the order that builds each. The shell's
     list of the engine operations it executes gained none of the four. Root's tools for them
     were presented, and every call answered JSON-RPC -32603, naming the operation as
     unfinished, with no record, so no caller saw a listing or the stub's answer. `conduit serve`
     refused a store in which a consumer other than root held an open Conduit that lists any of
     them. The vectors run the core alone, so none showed it.
   - **What the build did:** Found by the admin on 2026-11-11, while planning. Nothing until the
     fix.
   - **Settlement:** Fixed by order 072. The shell passes the four operations to the core. An MCP
     test, which failed before the fix, lists through root's management tool and gets the lift
     stub's answer. A shell test checks that every operation the core package's item `engine`
     implements is either executed or listed as unfinished, with the order that builds it.
     `declare-lost.v1` alone remains unfinished.

Work in this order. Run one cargo command at a time. While you work, run the tests you are
changing by name: `cargo test --release -p conduit-shell --lib NAME`. Run the full suite once, at
the end. The tests read the helper tokens of a test store, which are credentials: never print one.

## Scope

Change:

- `engine/crates/shell/src/server.rs`: the executable list's four new entries, its length, its
  visibility and its comment, and the new test-only table with its comment. Nothing else.
- `engine/crates/shell/src/server_tests.rs`: the test of step 2.
- `engine/crates/shell/src/tests.rs`: the test of step 4.
- `engine/CONFORMANCE.md`: the paragraph of step 5.
- `docs/proposals/first-build/log-increment-4.md`: one row.

Do not change:

- the core (`engine/crates/core/`), the loader, or the conformance runner
  (`engine/crates/conformance/`);
- any case, vector, fixture or the manifest under `conformance/`. This order declares no vector
  change and has no declaration file;
- `spec/`, `packages/`, `tools/`, or any other record;
- any other shell code: `unfinished()`, `preflight()`, any message, the CLI;
- any existing test, including
  `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders` (`tests.rs:1323`),
  which still checks the core's own stub replies.

Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/check_private_paths.py`, if your starting tree has it (order 068 adds it)
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py HEAD`, whose last line is
  `Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.`
- `git diff --check`
- `git status --short`, which lists exactly the five files of the scope, each modified, and
  nothing else

In `engine/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- these four tests, by name, all green:
  - `root_reaches_list_and_the_lift_stub_over_mcp`;
  - `every_engine_operation_is_executable_or_unfinished_with_its_order`;
  - `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders`;
  - `s1_http_consumers_authority_records_and_private_t7_changes`;
- the full suite, once: `cargo test --release`. If your starting tree's AGENTS.md names
  `cargo nextest run --release` as the gate instead (order 068), run what it names.

Report:

- the failure message of step 2 before the fix;
- the failure message of step 4's trial;
- the full suite's wall time, as a number;
- the totals passed and failed.

Write no local path in the report. If a message holds one, such as the store directory that
`client()`'s initialization error prints (`server.rs:836-841`), replace it with `<store>`.

Tests that bind loopback ports may fail in your sandbox. If they do, say so, and the admin reruns
them. Do not stage anything.
