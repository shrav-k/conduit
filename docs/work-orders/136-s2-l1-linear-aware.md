# Work order 136 (S2-L1b): S2's Conduit-aware Linear implementation

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and never reach linear.app or any
other real service. Run one cargo command at a time, and never print a whole large file or log.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never print a key or a secret value, even a synthetic one that a test made.

## What and why

Order 133 published S2's contracts as `packages/scenario-s2-linear-restrictions/` (archive
`sha256:6380ce0e6a7db56712f457fd3a56aa7a6802ce3e019c56a3433eda440feac43e`): the operations
`read-issue.v1`, `list-issues.v1` and `update-issue.v1`, the capabilities `issue-reading.v1` and
`issue-writing.v1`, and the filter `issue-scope.v1`. Read its README and `contracts.yaml` first,
above all its clauses F1 to F6: this order implements them, as S2-L1 of
[S2's plan](../../experiments/013-s2-shared-login/plan.md), the Conduit-aware row of D01. Order
135 built the local stub of Linear's API (`experiments/013-s2-shared-login/linear-stub/`) that
this order's tests run against. This order starts only once order 135 is merged.

Read first:

- the contracts package and the stub's README (its frozen interface);
- S2's README: P3 and P4, D01 and D05, and "Preparation, apart from author tasks";
- IFC-006, IFC-008, IFC-009, IFC-018, IFC-019, IFC-026, IFC-037 and IFC-047 in
  `spec/interface/implementation.yaml`; CON-022 in `spec/promises/conduits.yaml` and PKG-019 in
  `spec/promises/packages.yaml` (`supports`, on the manifest's implementation item, as
  `spec/schemas/examples/valid/files-package/conduit.yaml` shows, near line 60); OPS-007, OPS-009,
  OPS-024 and OPS-034 in `spec/conventions/operations.yaml`;
- how an item binds a secret: MAN-010's `environment` and MAN-017 in `spec/promises/manifest.yaml`,
  a SEC-001 place in the Type's schema (`spec/schemas/examples/valid/project/contracts.yaml`, near
  line 90, and its `conduit.yaml`, near lines 44 to 56), SEC-010 and PRV-007's `secrets`, SEC-014
  (a running implementation keeps its original value) and SEC-019;
- how the engine sends a request's filters and reads a `reason`: `engine/crates/core/src/requests.rs`
  and `engine/crates/shell/src/tests.rs`; and a package's form and builder in
  `packages/provider-local-machine/`.

## The decisions it applies

1. **Where it lives.** `packages/scenario-s2-linear/`, a crate with its own `Cargo.toml`, committed
   `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/` and `dist/`. Its
   only dependency on this repository is a path dev-dependency on the stub crate, which stays out of
   the archive. Its README says first that it serves scenario S2 and is not for general use.
2. **Its Type and item.** It exports a Type, `linear-aware.v1`: family Resource, offering
   `issue-reading.v1` and `issue-writing.v1` from the contracts package, which it pins by address,
   version and digest, its `implementation` mapping the three operations to its item, with a
   configuration schema whose places are required and scalar where the item binds them (MAN-017):
   the service's base URL, the API key as a secret place (SEC-001), and an optional test clock
   (decision 5). Its item is a relative command, `bin/scenario-s2-linear`, run by the engine's own
   runner (MAN-010, PRV-007), not plain, stating in `supports` that it applies
   `example.org/conduit/scenario-s2-linear-restrictions#issue-scope.v1` (CON-022, PKG-019). Its
   `environment` binds the base URL and the key. The engine's reading of secrets is not built yet
   (SEC-D): tests set the variable directly, and the README says so. D01 counts this Type and its
   configuration.
3. **The server.** An MCP server over stdio, Conduit-aware:
   - it lists Conduit's extension at revision `v0` (IFC-037), and answers a call without it, or at
     another revision, with -32021 whose `data.requiredCapabilities` names the extension, and with
     no service request;
   - it answers each outcome as OPS-007 reads it, with the `outcome` key and `isError` agreeing
     (IFC-009), and a diagnostic that names the cause (ruling 52);
   - it fails closed: where an IFC-006 key is absent or malformed, `filters` above all, it performs
     nothing (a missing `filters` never means no filter); it applies every item of `filters`,
     duplicates included (IFC-018); a requirement it does not support answers `unavailable`.
4. **The filter, as the contracts package's clauses say.** Scope and deadline are judged from the
   call's arguments and its filters, before any request to the service. A read or update outside
   scope, and any call at or after the deadline, sends nothing to the service and answers `refused`
   in IFC-026's form. A list is judged by F2: it returns only the in-scope issues that exist, scope
   before limit, as `succeeded`; only the deadline refuses a list. The deadline is judged once, at
   receipt (F4). Returned content is the contracts' projection, and a mention of another issue in a
   text is returned as it is.
5. **The clock.** The deadline is judged by the real clock, unless the configuration's optional
   test clock gives an instant; the README says the test clock is for tests and prepared runs only.
   A malformed test clock performs nothing and answers `failed`.
6. **The service.** It sends the key only over `https`, or `http` to a loopback address (the
   principle of SEC-024). A service it cannot reach, or that times out, is `unknown` for
   `update-issue.v1` and `failed` for the two reads, which change nothing (as IFC-010 splits a plain
   server's); a 401 or 403, as after the key is rotated, is `failed` for all three. The key appears
   in no answer, log line or error. It makes no onward request through Conduit: report that D05's
   onward-call case stays unexercised.
7. **Containers and machines, listed, not built.** The item is a host build run by the engine's own
   runner. Say in the README what A63-DR and S3-PATH would need: a Linux build, and a PATH-name or
   absolute item in an operator-supplied image or machine, which pins nothing.
8. **Its README** gives D15's four fields as
   [S10's README](../../experiments/021-s10-swapped-providers/README.md#measures) defines them, and
   D01's counts for this row (files, lines, concepts, configuration) as S2's M0 counts them. A
   `package` binary writes a deterministic archive and its digest.
9. **Tests**, driving the binary with an `rmcp` client and the full request `_meta`, against the
   stub started in process, with synthetic keys and markers in no real token's format:
   - discovery; -32021 without the extension and at another revision; answers validate against
     the output schemas; `isError` agrees with the outcome;
   - in scope: read, list and update succeed;
   - out of scope, a read and an update are refused, and the stub counts no request for them; a
     list returns only in-scope issues, scope before limit;
   - after the deadline (by the test clock), each operation is refused, a repeated read included;
     a missing in-scope issue fails;
   - "changed authorization data": two calls with different `filters` arguments, the second judged
     on its own; the stub's own visibility change, labelled as the service's authorization;
   - judge once (F4): a call allowed at receipt is held at the stub's barrier, the test clock passes
     the deadline, and the released call succeeds;
   - a missing `filters` key, a malformed one and an unsupported requirement each perform nothing;
   - output delivery: the projection, and a mention of an out-of-scope issue returned unchanged;
   - rotation: a new process with the new key succeeds, the old process's calls fail with 401's
     outcome (SEC-014);
   - the service unreachable: `unknown` for update, `failed` for reads;
   - no key or marker in any answer, log line or stderr;
   - `conduit.yaml` and `contracts.yaml` validate against `spec/schemas/` (skipped, saying so,
     where `spec/` is absent), and the archive is the same, byte for byte, when built twice.
10. **What this order does not do:** no adapter (S2-L2), no plain server, no real Linear request,
    no engine change and no vector.

## Scope

Change only `packages/scenario-s2-linear/` (new).

## Done when

In the crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --release`, and `cargo run --release --bin package` twice, with the same digest. From
the repository root: `python3 tools/check_docs.py` and `python3 tools/set_names.py --check`. Run
these with `bash`. Each prints nothing when it passes:

```bash
D=packages/scenario-s2-linear
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "lin_api_|linear\.app/" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
```

`target/` and `dist/` stay ignored and unstaged. Report each choice of configuration and each thing
the contracts package leaves open, as your own choices and findings. Do not stage anything.

## Record

Run on 2026-11-12 on the Mac, from base 9f55b1e: the worker and a fresh verifier, then a second
worker round (281 s) and a fresh verifier (489 s). The result is `packages/scenario-s2-linear/`:
the Type `linear-aware.v1` and its item, the Conduit-aware MCP server for S2's three operations,
the filter `issue-scope.v1` applied before any service request and failing closed, refusals in
IFC-026's form, the key sent only over `https` or loopback, and the package builder.

Round one's two musts were fixed in round two: an integral `limit` written as a float (`1.0`) was
read as unlimited, and an unknown tool answered a failed result instead of a protocol error.
Round two's verifier left two musts. **The lane coordinator fixed them, an admin-side fix at the
admin's word (small findings, ruling 71):** every line the server writes now passes a redactor that
replaces the key, raw and JSON-escaped, since the MCP library builds some protocol errors itself,
-32022 among them, echoing a field the caller sent; and the test clock is checked by the same
format checker as the deadline, since chrono alone accepted a space separator. A fresh reader then
found that the redactor could drop the last answer when input ended early, and three clock strings
chrono still accepted; both were fixed, with regressions (an early end of input, 100 times; the
three strings; the redacted -32022). The reader confirmed that nothing else writes to stdout. The
crate's 21 tests pass on the Mac.

The worker's findings: D05's onward-call case stays unexercised, since this implementation makes no
onward request; and a Linux build and a PATH or absolute item, which A63-DR and S3-PATH would need,
are listed in its README, not built.
