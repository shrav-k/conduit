# Work order 025 (I4-01): the operations S1's tasks of change need

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates, to fetch the one
npm package named below, and for tests that bind a loopback port; use it for nothing else. You
cannot run `claude` or `codex` as clients here.

## What and why

The first order of increment 4 of [the first build](../implementation/FIRST_BUILD.md). In
increment 4 the admin, then a fresh author, does S1's tasks T1 to T7 on the engine and measures
the work against the baseline (A70). Increment 3 ran T1, T2 and T7. T4 to T6 change what exists:

- **T4**: agent A may now also write the shared directory, and agent B may now only read it.
- **T5**: the filesystem server behind the directory goes from
  `@modelcontextprotocol/server-filesystem` 2.1.0 to 2.2.0, which adds `append_file` and
  `copy_file`.
- **T6**: the directory is replaced with another, and both Agents keep the access intended.

Under the specification an object, once made, is never rewritten (OBJ-002, OBJ-011): a template
changes by `replace.v1` (SUB-009), a nested Conduit is made again from its template by
`register-replacement.v1` (LIF-009), an instance holds its package versions (PKG-005), a new
package version is installed by `install.v1`, and an instance ends by `retire.v1` and is removed
by `unregister.v1`. The owner chose, on 2026-11-10, to run T4 to T6 by the routes the
specification allows today and measure them, before deciding whether the language needs another
way to change what exists ([log of increment 4](../proposals/first-build/log-increment-4.md),
4-01). The engine supports `replace.v1`, `close.v1`, `register-replacement.v1`, `open.v1` and
`register.v1`, but `install.v1`, `retire.v1` and `unregister.v1` answer JSON-RPC -32603 as
unfinished, and an author can reach `replace.v1` only by writing its whole input as JSON. This
order finishes those three operations, gives the author commands for them, and adds the
filesystem package for 2.2.0.

Read: `reference-model/CONFORMANCE.md`, the shell as increment 3 left it
(`reference-model/crates/shell/`, its tests above all `tests/serve.rs`, which scripts T7), the
core's management paths (`reference-model/crates/core/src/registration.rs`, `records.rs`),
`conformance/fixtures/s1/` and its README, `reference-model/examples/s1/`, the baseline's T4 to T6
(`experiments/007-s1-baseline/results-2026-11-09.md`), and these promises: OBJ-002, OBJ-009,
OBJ-010, OBJ-011, SUB-009, INS-020, PKG-005, PKG-006, PKG-007, PKG-020, PKG-025, NAM-011, NAM-012,
LIF-008, LIF-009, LIF-019, LIF-029, LIF-036, CON-004, CON-025, IFC-012, IFC-038, REV-012, and the
operations `install.v1`, `retire.v1`, `unregister.v1` and `replace.v1` in
`spec/core/operations.yaml`.

## The decisions to apply

1. **`install.v1`, `retire.v1` and `unregister.v1` become public operations** as
   `spec/core/operations.yaml` and the promises describe them, requested of the serving engine
   like any other management operation, with their records. For `install.v1`, the engine fetches
   `file` URLs itself; an `https` fetch may be left unfinished, reported invalid with a
   diagnostic that says so, and listed in `CONFORMANCE.md`. A package installed while the
   engine serves is usable by the next request without a restart. `retire.v1` stops the run of an
   item that is run, through the runner, and closes the Conduits of which the instance is the
   target at once and those of which it is the consumer when its shutdown is confirmed; a
   retired Agent's run ends and its consumer stream is ended. Where the core already decides a
   case and the shell only lacked the path, keep the core's decision; where the core cannot
   decide a case from the promises, it is a finding: report it, do not settle it.
2. **Commands for the author**, each a request of the serving engine as the root Interface, as
   `conduit call` is, printing the result as `call` does:
   - `conduit install --store S ARCHIVE#SHA256`: `install.v1` with the address and version read
     from the archive's manifest, the digest given and an absolute `file:` URL for `fetch`.
   - `conduit submit --store S DIR`: `register.v1` of the documents in a directory, read as
     MAN-001 and SUB-010 read a project directory, with that project's manifest and contracts.
     It then sets up the new instances and opens the new Conduits whose consumer is the root
     Interface, as `conduit register` does at first registration, each as a request with its
     record.
   - `conduit replace --store S DIR KIND/NAME`: `replace.v1` of the template of that kind and name,
     its new document read from the project directory `DIR` with that project's manifest and
     contracts, at the template's current revision, which the command reads by `inspect.v1`.
   - `conduit retire --store S KIND/NAME` and `conduit unregister --store S KIND/NAME`, the
     latter at the current revision.

   The commands work only while `conduit serve` holds the store; without it each says so and
   exits non-zero. `conduit register` stays the command of first registration.
3. **The filesystem package 0.2.0.** Beside `example.org/s1/filesystem` 0.1.0, add 0.2.0: the same
   contracts and the same `tools` map, pinning `@modelcontextprotocol/server-filesystem` 2.2.0 by
   its `fileSha256`, which you compute from the npm archive you fetch. Its archive is not a
   preinstalled snapshot: it is installed by `conduit install`. Lay out its files beside 0.1.0's
   in `conformance/fixtures/s1/`, with a way to rebuild its archive and digest that the README
   states. The history package stays on filesystem 0.1.0.
4. **Vectors and tests**, each failing before this order and passing after:
   - T4's route: `replace.v1` of a Conduit template that a nested Conduit was made from, then
     `close.v1`, `register-replacement.v1` and `open.v1` of that nested Conduit. The consumer's
     list then has exactly the operations the template now lists, under the same tool names
     (IFC-012, LIF-009), the `list_changed` notice reaches that consumer alone (IFC-038), and
     another consumer whose nested Conduit was made from the same template before the replace
     keeps its list until its own replacement (INS-020).
   - T5's route: `install.v1` of 0.2.0; a new Resource whose template resolves filesystem 0.2.0;
     a Conduit to it that lists the three operations. Its consumer is offered exactly three
     tools, and `append_file` and `copy_file` never appear in any consumer's list (PKG-020,
     IFC-012); an instance made on 0.1.0 keeps 0.1.0 (PKG-005). With `CONDUIT_TEST_NPM=1`, the
     2.2.0 server runs and a read through it succeeds.
   - Retirement: retiring a Resource closes the Conduits that target it at once; a
     `register-replacement.v1` whose target is retired reports invalid (CON-025); `unregister.v1`
     of an instance that is not retired reports conflict, and of a retired one removes it while
     its records stay.
   - Each command of decision 2, in the shell's tests, against a serving store.
5. **`CONFORMANCE.md`** lists the operations now supported, what remains unfinished, and every
   choice this order leaves to you, and the reference model's README shows the commands.

## Scope

Create and change: `reference-model/`, `conformance/`. Report anything else that needs changing,
including any change to `spec/` you find necessary: do not make one.

## Done when

`install.v1`, `retire.v1` and `unregister.v1` are served with their records; the five commands
work against a serving store; filesystem 0.2.0 installs and runs 2.2.0 with only the Conduit's
operations offered; the vectors of decision 4 pass. From the repository root:
`python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass;
in `reference-model/`, `cargo test --release`, `cargo clippy --all-targets -- -D warnings`,
`cargo fmt --check` and `CONDUIT_TEST_NPM=1 cargo test --release` pass. Running the npm tests is
part of this order's network grant. Check the files you create for trailing white space
yourself; do not stage anything.
