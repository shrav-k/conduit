# Work order 135 (S2-L1a): a local stub of the Linear API, for S2

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and never reach linear.app or any
other real service. Run one cargo command at a time, and never print a whole large file or log.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never print a key, even a synthetic one that a test made.

## What and why

S2 shares one Linear service login between two Agents with different restrictions
([its pre-registration](../../experiments/013-s2-shared-login/README.md)). Its implementations of
S2's contracts (`packages/scenario-s2-linear-restrictions/`, order 133) talk to Linear's API. Its
harness and its implementations' tests need that API with no account and no network. This order
builds a local stub of the part of it that S2 uses. It is test infrastructure, not an
implementation, so it lives under the experiment and its lines are not counted in D01. The
Conduit-aware implementation (order 136) and the adapter (S2-L2) test against it, and the
harness (S2-H) drives it. The engine's vectors do not use it: they write their own fixtures.

## The decisions it applies

1. **Where it lives.** `experiments/013-s2-shared-login/linear-stub/`, a crate with its own
   `Cargo.toml`, committed `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for
   `target/`, depending on no crate of this repository. It is a library and a binary, so that
   another crate's tests can start it in process through a path dev-dependency.
2. **The API subset.** A real GraphQL schema, served with a GraphQL library (async-graphql or the
   like), not by matching exact query strings: S2-L2's plain server is not chosen yet and will send
   its own queries. It covers reading one issue by its identifier, listing issues with a page size,
   and updating an issue's title and description, with the fields S2's contracts return
   (identifier, title, description, state, updated time), and with an API key in the
   `Authorization` header. Model the field names and shapes on Linear's public GraphQL API, and
   write in the README each way the stub differs from it.
3. **Data.** Synthetic issues only, with identifiers in a form such as `SYN-123`, made by the test
   or a fixture file; no real workspace, team or user name.
4. **The control endpoint**, on the same loopback listener, with its own synthetic token, its
   requests left out of the counts. It lets a test or the harness:
   - set which issues a key may see: the **service's own authorization**, labelled so, not the
     filter's (S2's "changed authorization data" is the filter's inputs, which the stub never sees);
   - arm a barrier on the next request that matches, wait until a request reaches it, and release
     it, each with a timeout;
   - read the count of API requests, so that a test can show a refused call reached no service;
   - rotate the key the stub accepts (S2-T5).
5. **Its interface is frozen for S2-L2**: the README documents the GraphQL subset and the control
   endpoint as the interface the other crates rely on, with a version, and says that its fidelity
   to the real API cannot be checked offline: the coordinator's S2 runs, against the owner's test
   workspace, check it.
6. **Tests:** each query and the mutation answer as documented; a wrong or rotated key is refused
   with Linear's status for it; a barrier holds a request until released, and times out when not
   released; counts exclude control requests; a query outside the subset answers a GraphQL error.

## Scope

Change only `experiments/013-s2-shared-login/linear-stub/` (new).

## Done when

In the crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and
`cargo test --release`. From the repository root: `python3 tools/check_docs.py`. Run these with
`bash`. Each prints nothing when it passes:

```bash
D=experiments/013-s2-shared-login/linear-stub
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "lin_api_|linear\.app/" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
```

`target/` stays ignored and unstaged. Report the GraphQL library, the field names and each
difference from the real API as your own choices. Do not stage anything.

## Record

Run on 2026-11-12 on the Mac, from base eed053a: the worker and a fresh verifier, then a second
worker round (135 s) and a fresh verifier (206 s), which found nothing to change. The result is
`experiments/013-s2-shared-login/linear-stub/`: a GraphQL schema subset of Linear's API for one
issue, a list and an update, with synthetic `SYN-` issues and an API key; a control endpoint with
its own token for the service's own authorization, barriers with timeouts, request counts and key
rotation; and its interface documented and frozen for S2-L2. Round one's one should (a negative
timeout answered 422, not the documented 400) was fixed in round two. Neither verifier's sandbox
could bind loopback; the lane coordinator ran the crate's tests on the result: 14 passed.
