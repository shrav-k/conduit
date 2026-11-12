# Work order 139 (S2-L2): S2's small Linear adapter

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no real service. Run one
cargo command at a time, and never print a whole large file or log. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never print a key or a
token, even a synthetic one that a test made.

## What and why

S2's D01 qualifies one user task, reading an issue, through three rows: an unchanged plain server,
a small adapter and a Conduit-aware implementation, and counts each row's authoring work apart
([S2's README](../../experiments/013-s2-shared-login/README.md), D01, D05 and S2-T2). Order 136
built the aware row, `packages/scenario-s2-linear/`. This order builds the adapter, which
[S2's plan](../../experiments/013-s2-shared-login/plan.md) now defines (the lane coordinator's
reading of the admin's decision below, logged there for the owner to strike): the least that an
author writes to put Linear's own API behind Conduit for D01's task, reading, with S2's
restriction, calling the service's GraphQL itself. A wrapper around the plain server, as S7 reads
an adapter, would wait for the plain server's choice.

**The plain row is not in this order** (the admin's decision of 2026-11-12, logged in
[S2's plan](../../experiments/013-s2-shared-login/plan.md) for the owner to strike): no
open-source Linear MCP server found then runs unchanged against the local stub, so the plain row
runs only in the owner's S2 runs, against the test workspace, with a server chosen with the owner
before them and packaged by a later order, S2-PL.

Read first:

- the contracts package, `packages/scenario-s2-linear-restrictions/` (order 133): its README and
  `contracts.yaml`, above all `read-issue.v1` and the clauses F1 to F6 of `issue-scope.v1`;
- the stub's README, `experiments/013-s2-shared-login/linear-stub/README.md`: its frozen
  interface v1, which is all this order relies on;
- [order 136](136-s2-l1-linear-aware.md), whose decisions 2 to 7 this order applies to the
  adapter as written there, except where this order says otherwise, and the promises and files it
  lists to read, and its Record, whose four fixed defects the tests below cover;
- S2's README: D01 and D05, and "Preparation, apart from author tasks".

Do not read or copy the aware implementation's code under `packages/scenario-s2-linear/src/`: D01
counts this row's own work. Its package files and README may be read for form.

## The decisions it applies

1. **Where it lives.** `packages/scenario-s2-linear-adapter/`, a crate with its own `Cargo.toml`,
   committed `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/` and
   `dist/`. Its only dependency on this repository is a path dev-dependency on the stub crate,
   which stays out of the archive. Its README says first that it serves scenario S2 and is not for
   general use.
2. **Reading only.** It exports a Type, `linear-adapter.v1`: family Resource, offering
   `issue-reading.v1` from the contracts package, pinned by address, version and digest, as order
   136's Type does. That capability lists `read-issue.v1`, D01's task, and `list-issues.v1`, and
   TPL-004 requires an implementation of every operation of a capability a Type offers, so it
   implements both, and not `update-issue.v1`. Its configuration and item follow order 136's
   decision 2: the service's base URL, the API key as a secret place, the optional test clock, a
   relative command `bin/scenario-s2-linear-adapter` run by the engine's own runner, not plain,
   stating in `supports` that it applies `issue-scope.v1`.
3. **The server, the filter, the clock and the service** are order 136's decisions 3 to 6, for these
   two operations, the test clock reaching it in each call's configuration and bound to no variable,
   as there: Conduit's extension at `v0` and -32021 without it; OPS-007's outcomes with `isError`
   agreeing; fail closed on a missing or malformed IFC-006 key; scope and deadline judged from the
   call and its filters before any request to the service (F1, F2, F4), judged once at receipt; the
   five-field projection (F3); refusal in IFC-026's form with no issue output (F6); a list of only
   the in-scope issues that exist, scope before limit, refused only by the deadline (F2); the key
   sent only over `https` or to loopback `http`, in the bare `Authorization` form the stub and
   Linear's personal keys use; reads `failed` when the service is unreachable, times out (a bounded
   request timeout, longer than the barrier test's hold) or answers 401 or 403; and every line the
   server writes passes a redactor of the key, raw and JSON-escaped, as order 136's Record says. It
   makes no onward request: D05's onward-call case stays unexercised for this row.
4. **Small, and counted.** Write it as small as these decisions allow: one GraphQL query document
   per operation, no generated client, no state between calls, no cache. The stub's `issues` takes
   no filter, so a list sends the read's document once per in-scope identifier, leaving out each
   that comes back not found, or pages through `issues`: say which. Its README gives D01's counts
   for this row (files, lines, concepts, configuration) as S2's M0 counts them, apart from the
   tests, and D15's four fields as
   [S10's README](../../experiments/021-s10-swapped-providers/README.md#measures) defines them.
   Containers and machines are listed, not built, as order 136's decision 7 says. A `package`
   binary writes a deterministic archive and its digest.
5. **Tests**, driving the binary with an `rmcp` client and the full request `_meta`, against the
   stub started in process, with synthetic keys and markers in no real token's format, compared
   with `assert!(a == b)`, never `assert_eq!`, where a marker could print:
   - discovery, with `tools/list` naming exactly the read and the list; -32021 without the
     extension and at another revision; a call to `update-issue.v1` or an unknown tool is a
     protocol error, with no stub request; answers validate against the output schemas; `isError`
     agrees with the outcome;
   - an in-scope read succeeds with the projection, and a mention of an out-of-scope issue in its
     text is returned unchanged;
   - an out-of-scope read is refused, and the stub counts no request for it; a missing in-scope
     issue fails; a list returns only the in-scope issues that exist, scope before limit, with a
     `limit` written `1.0` read as 1;
   - after the deadline, by the test clock, a read and a list are refused, a repeated read
     included;
   - changed authorization data: two calls with different `filters`, the second judged on its own;
     the stub's own visibility change, labelled as the service's authorization;
   - judge once (F4), by the real clock: a read with a deadline about a second ahead, allowed at
     receipt, is held at the stub's barrier past the deadline, released and succeeds, and the same
     read repeated is refused with no further stub request;
   - the test clock checked as order 136's is: a malformed one, and the strings chrono alone would
     accept, each perform nothing and answer `failed`;
   - a base URL over `http` to a non-loopback address performs nothing;
   - a missing `filters` key, a malformed one and an unsupported requirement each perform nothing;
   - rotation: a new process with the new key succeeds, and the old process's reads fail;
   - the service unreachable: `failed`;
   - no key or marker in any answer, log line or stderr, a -32022 that echoes a sent field
     included; and an early end of input still delivers the last answer, many times over;
   - `conduit.yaml` and `contracts.yaml` validate against `spec/schemas/` (skipped, saying so,
     where `spec/` is absent), and the archive is the same, byte for byte, when built twice.
6. **What this order does not do:** no plain server and no choice of one, no update, no
   onward call, no real Linear request, no engine change and no vector.

## Scope

Change only `packages/scenario-s2-linear-adapter/` (new).

## Done when

In the crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --release`, and `cargo run --release --bin package` twice, with the same digest. From
the repository root: `python3 tools/check_docs.py` and `python3 tools/set_names.py --check`. Run
these with `bash`. Each prints nothing when it passes:

```bash
D=packages/scenario-s2-linear-adapter
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "lin_api_|linear\.app/" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
```

`target/` and `dist/` stay ignored and unstaged. Report each choice of configuration and each thing
the contracts package leaves open, as your own choices and findings. Do not stage anything.
