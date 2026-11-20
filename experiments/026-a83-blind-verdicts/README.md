# 026 — A83: blind verdicts from spec/ alone

A83 asks whether a reader given only `spec/` and the first-build profile gives the engine's
verdicts, names the deciding promise and places each promise in the layer that LD1's test gives.
Its four parts are V, verdicts; R, resolving by hand; L, layers; and E, the specification's
examples, scored apart, as [the review's A83 section](../../docs/proposals/full-review/experiments.md#a83-the-blind-verdict-test)
and D81 of [its decisions](../../docs/proposals/full-review/decisions.md) state. Parts V, R and L,
and Part E's readers, run after SO-A and SO-B merge and the S1 rerun's pin, under
[ruling 78, point 1](../../docs/proposals/ruling-78/README.md#the-decisions). Each is
pre-registered here before it runs; this first registration covers only Part E's engine run.

## Part E's engine run

[Work order 165](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/165-by-rule-examples-public-routes.md) pre-registers
this run. The [test](../../engine/crates/shell/tests/by_rule.rs) sends the examples and compares
its observations with the [table](../../engine/crates/shell/tests/expected/by-rule-outcomes.txt).
The dated result is [results-2026-11-15.md](results-2026-11-15.md).

1. The route is root's Interface, through `Shell::submit` of the `invoke` input that a client's
   `tools/call` becomes, without `serving` or a key, using `MissingExecutor`. Registrations call
   `management.register.v1` with `documents` and `manifest`. Installations call
   `management.install.v1` with the arguments of `boundary::install_arguments` and a `file` URL;
   the shell reads the archive. Each by-rule file is read by `conduit_loader::read_file` and its
   documents become JSON values in their original order.
2. Each set gets a fresh store under a temporary directory and `Shell::open`, with
   `boundary::context` of the repository root and `Config::default()`. First start receives the
   fixed reachable machine `by-rule`, with hostname label `by-rule`, OS `linux` and architecture
   `amd64`. This removes host-dependent placement; `files-server` asks for Linux. No port is bound.
3. Seven sets have their own manifest, the document without `kind`; it and the other documents
   are sent unchanged, with nothing added. The other 34 receive a manifest with the specification's
   `apiVersion` and address `example.org/by-rule`. Only the 12 sets whose documents contain
   `example.org/files#` get that package pinned at 0.3.0 to the archive's actual digest. Before
   those registrations, root installs a stub `example.org/common` 2.0.1 exporting the string
   schema `timestamp.v1` with format `date-time`, then `example.org/files` 0.3.0. Both archives
   are gzip tar files of regular root files, built once under the test's temporary directory.
   The common archive holds `conduit.json` and `contracts.json`; the files archive holds its
   original `contracts.yaml` bytes and its loaded manifest as `conduit.json`, changing only the
   common dependency's digest. Nothing else is supplied: some sets name objects or packages
   that the set does not write out, as [the schema examples explain](../../spec/schemas/README.md#examples).
4. Every by-rule file is one registration, including NAM-028 and SCH-019's operation/template
   set. `valid/equal-defaults` is read with `read_project` and registered with its own manifest;
   `valid/files-package` is installed after the common stub in its own engine. The four excluded
   directories are `sectioned-project` and `sectioned-package` (translation awaits the translator),
   `first-start` (not a project) and `project`. The last names the unwritten harness package and
   relative program paths that a registration without a directory rejects. The test requires
   every valid directory to be sent or excluded and every by-rule file to have a table line.
5. Classes read only client-visible `isError`, reason (or outcome) under the shell's metadata
   prefix, and all error diagnostics' `ruleId` values. `named` requires a by-rule rejection with
   reason `invalid` and its filename's rule among those diagnostics. Other rejections are `other`;
   a failed prerequisite installation is `scaffold`; `isError: false` is `accepted`; a submit
   error or protocol error is `error`. A by-rule set passes only as `named`, and a valid set only
   as `accepted`. The table retains every observed class, reason and distinct sorted rule ID;
   the regression assertion checks the table, even where the example does not pass.

Two changes to the review's Part E text are fixed before the run: `valid/project` is excluded
because its missing harness and relative commands prevent this public registration, and `named`
requires reason `invalid`, as [FR-357's verifier](../../docs/proposals/full-review/findings/P07-engine-build.md#fr-357)
requires. This checks the rule's refusal reason as well as a diagnostic's rule ID.

The review's expectation, before the run: the files of CTR-009, PLC-028, PRV-001, PRV-004,
PRV-016, SEC-001, SEC-022 and SEC-024 are not `named`, since no engine code named those eight
rules at the review's commit. SEC-002 has no example. These are expectations, not results;
PKG-030 and PLC-025 are also reported apart because FR-357's verifier found no case for them.
