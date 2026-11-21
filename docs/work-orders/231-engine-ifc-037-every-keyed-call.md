# Work order 231: the engine declares Conduit's extension on every keyed call (IFC-037)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` each case you add, remove or change. This order
has no network access.

## What and why

Order 229 (the git package's first slice) stopped before any change. IFC-037
(`spec/interface/implementation.yaml`) says that, before its first call that carries Conduit's
keys to each run of an implementation, the engine reads the implementation's `server/discover`
result. It then declares `io.github.shrav-k.conduit/implementation-interface` with `revision`
`v0` on each such call. Where discovery does not list the extension, the call is not dispatched,
reports unavailable, and names the missing extension in its record.

The engine does this only for an implementation marked `plain` (its keyed hooks) and for secret
reads: `engine/crates/core/src/requests.rs`, near line 830, `if item["plain"] == true ||
v["secret_read"].is_object()`. So a call to a non-plain implementation, which always carries
Conduit's keys, goes out undeclared. A conforming package, like 229's, refuses it. The existing
test `keyed_hooks_read_plain_per_call` asserts, near line 2762, that the declaration is absent
for a non-plain command setup hook. That assertion encodes the defect, since `spec/` binds.

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
4, nothing assumes a particular vendor's agent (an implementation that checks the declaration, as
the git package does, works like any other); and 7, everything is attributable (a call withheld
for a missing extension is recorded with the extension named).

**Should this exist:** yes. It brings the engine to a promise it already must keep, found by the
dogfood. It adds nothing new.

## Decisions

1. Every call that carries Conduit's keys to an implementation follows IFC-037:
   - discovery is read once per run before the first such call;
   - the extension is declared on each such call;
   - a run whose discovery lacks it gets no such call, which reports unavailable, with the
     missing extension in its record.

   This covers requests and hooks to a non-plain implementation, keyed hooks of a plain one, and
   provider calls. Calls that IFC-007 governs, to a plain implementation for a hook not keyed,
   stay as they are: no keys, no declaration.
2. `keyed_hooks_read_plain_per_call`'s assertion of absence changes to IFC-037's presence, with
   a comment naming the promise. A new test shows a non-plain implementation whose discovery
   lacks the extension: nothing is dispatched, and the call reports unavailable with the
   extension named.
3. Every package in the tree whose implementation is not plain still passes its own tests and
   the engine's. Where one does not list the extension in its discovery answer, give it the
   answer it needs, in that package's existing style; `runtime-provider-kit` already has one.
   List each package you change. A package for which that is not a small change: stop and
   report.
4. A case of the conformance suite whose recorded calls change follows, declared in
   `docs/work-orders/231-declared.yaml`.

**Stop rule.** Change nothing under `spec/`. A choice `spec/` does not make: stop and report.

## Scope

`engine/crates/core/src/`, `engine/crates/shell/src/` where its transport needs it,
`engine/crates/shell/tests/`, `conformance/` where recorded calls change, `packages/*/src/` only
as decision 3 says, `engine/STANDING.md` by the tool, `engine/CONFORMANCE.md`, and
`docs/work-orders/231-declared.yaml`. Nothing else.

## Before you change anything

```sh
git merge-base --is-ancestor b45ffa13 HEAD; echo "main $?"
grep -c 'item\["plain"\] == true || v\["secret_read"\].is_object()' engine/crates/core/src/requests.rs
grep -c 'fn keyed_hooks_read_plain_per_call' engine/crates/core/src/requests.rs
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `1` and `0`.

## Done when

```sh
(cd engine && cargo fmt --check && cargo clippy --all-targets -- -D warnings)
(cd engine && RUST_TEST_THREADS=4 cargo test --release)   # once, at the end
grep -c 'item\["plain"\] == true || v\["secret_read"\].is_object()' engine/crates/core/src/requests.rs  # 0; fails today (1)
python3 tools/promise_standing.py --check; python3 tools/check_docs.py   # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/231-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors
git diff --check
```

Each changed package's own tests also pass.

## Record

Written by root on 2026-11-20 after order 229 stopped on this defect, and run through Conduit
itself, by the dogfood's own Agents, in Pacific time. The text merges with the build, and it had
no separate read.

Worker-2's first turn (16:51 to 17:03) built it for stdio calls and stopped for one checker
outside its scope. Root widened the scope to `engine/crates/conformance/src/records.rs`, so that
a permitted access record stored before discovery is recognized. The second turn (17:03 to
17:28) moved the 50 cases whose recorded calls change. The suite passed 424, conformance 557, and
the vector delta showed 61,050 declared changes and 24 flips.

`turn-verifier-2.turn.v1` (17:28 to 17:34) confirmed that every flip is one of 12 unchanged
assertions moved to a new step, and that nothing was weakened. It found three musts:
- provider calls bypassed discovery;
- a run replaced between discovery and dispatch lost the missing-extension ending;
- remote connections were not covered.

Root rebased the work onto order 221, and the third turn (17:35 to 17:59) fixed the first two
and the rebase's compile break. The suite passed 466. The engine makes no remote connection yet,
so IFC-037's remote half is recorded in `engine/CONFORMANCE.md` as waiting for one.

`turn-reader-1.turn.v1` (18:00 to 18:04), the Claude reader, accepted it with one should. Root
applied it: `engine/CONFORMANCE.md` and `engine/README.md` no longer say that a missing
extension reports unknown or failed. The gate's run on the squash is the coordinator's, at the
merge.
