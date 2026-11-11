# Work order 018: the check of two protocol errors, -32602 and -32020

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `003-increment-2-study.schema.json`, reading its
`decisions_needed` as the questions it settles or leaves, each with options and a
recommendation. This order has network access, to read the Model Context Protocol's
specification and the source of its official SDKs; use it for nothing else.

## What and why

[Decision 8](../proposals/first-build/increment-2/decisions.md) of increment 2 asks for this
check before the shell of increment 3 builds any path that sends a call again. Two readings in
the specification rest on an inference that round 6 struck in a like case (finding 6-1-01, for
-32021), and the tracker lists them as still open (`docs/AUDIT.md`, "Still open"):

1. IFC-010 reads a JSON-RPC error -32602 from a `plain` server as the call having failed,
   whatever the operation's effect: that is, that the server did nothing.
2. IFC-046 says the protocol has a server reject a call that gets -32020 before it acts, so
   that the engine may read the tool list again and send the call once more.

If either is wrong, an engine could report an effect as not having happened, or send an effect
twice. This order checks both against the protocol's text and what its servers do.

## What to do

1. **The protocol's text**: in each revision this specification names (read IFC-001, IFC-003 and
   DL12 for which), quote exactly what it says of -32602 and of -32020: when a server sends
   each, and whether it says the server has not acted. Cite each revision and section.
2. **What servers do**: in the official SDKs a server is built on (TypeScript, Python and
   Rust's `rmcp`, at least), find where each code is sent, and whether a tool's handler, after
   it has acted, can make the server answer with it (a handler that throws the protocol's own
   error with that code, for instance). Cite files and lines of a tagged release.
3. **The reference servers**: whether the filesystem server that S1 uses, at the versions
   experiment 008 and A70 ran (2.1.0 and 2.2.0), ever answers either code after writing.
4. **Write** `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/check-plain-errors.md`, not binding: for each reading,
   what the protocol says, what its servers do, whether the reading holds, and if it does not,
   the options for IFC-010 or IFC-046 with a recommendation for the admin. Decide nothing.

## Scope

Only the new file. Change nothing under `spec/`.

## Done when

Both readings are answered with quotations and citations, every claim cites its source, and
`python3 tools/check_docs.py` and `git diff --check` pass.

## Record

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 8 minutes. The protocol's text and the official SDKs' source, cited: neither
  `-32602` nor `-32020` proves that a server has not acted, and the SDKs let a handler answer
  either after acting. No such path in the filesystem server at 2.1.0 or 2.2.0.
- **Verifier**: 3 minutes. One `must`: the report gave `failed` to the engine's own rejection
  before dispatch, which has a reason and no outcome (OPS-005).
- **Admin**: fixed that sentence, and decided the two readings as decision 8a of
  [increment 2's decisions](../proposals/first-build/increment-2/decisions.md), which order
  015 applies.
