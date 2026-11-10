# Work order 015 (I2-06): how a request ends, its repeats, and plain results

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema every build order of increment 2 shares. This order has network access, to fetch crates
and for tests that bind a loopback port; use it for nothing else.

## What and why

I2-06 of the [study](../proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders):
once access allows a request (order 014), the core decides how it ends: its outcome or its
reason, the answer that a repeat gets, what a result from a `plain` server means, and what
happens when an outcome is unknown. Nothing here sends a call: the core decides, and the shell
of increment 3 carries out.

Read: the "How a request ends" table of the study's section 2, for the OPS, IMP and IFC promises
it lists; the rows S1-key-scope, S1-unknown-write, S1-reconcile and S1-plain-ends of section 3;
[decision 5](../proposals/first-build/increment-2/decisions.md), which settles the study's F1, and
decision 8; and the core as order 014 left it.

## The decisions to apply

1. **The ending of a request** under OPS-001 and OPS-005, with the result envelope of IFC-025,
   reasons in the order OPS-023 gives, and inputs normalized with defaults filled before a
   repeat is compared (OPS-009, OPS-012).
2. **Repeats**: the invocation index keyed as OPS-009 and OPS-012 say, by caller, sender,
   target, operation and filled contents; a repeat of a succeeded invocation is answered from
   what was kept and never dispatched again; one with other contents reports conflict
   (OPS-032, OPS-034); one still in progress is answered as the promises say.
3. **F1, by decision 5**: the content of a succeeded `plain` invocation is kept and given again
   on a repeat, as declared outputs are, for as long as DL10 keeps outputs; after that a repeat
   reports conflict under OPS-034. Amend OPS-009, IFC-025 and DL10 to say so, add the vector, and
   log the fix in `docs/proposals/first-build/log-increment-2.md` as "Settled by the admin,
   decision 5", with "The owner can strike it".
4. **Plain results**, with the table of IFC-010 and IFC-046 corrected by
   [decision 8a](../proposals/first-build/increment-2/decisions.md), which settles the check of
   [order 018](../proposals/first-build/check-plain-errors.md): `-32602` from a `plain` server is
   treated as any other error of such a server, `failed` for an operation that has no effect,
   is not a control and starts no model work, and `unknown` otherwise; and the engine sends a
   call again after `-32020` only for an operation that has no effect or that its contract
   declares safe to repeat, and otherwise ends it `unknown`. A rejection the engine makes
   before it sends a call keeps its reason and has no outcome (OPS-005). Amend IFC-010 and
   IFC-046, and OPS-003 or OPS-018 if they repeat the old reading, with vectors, and log each
   as "Settled by the admin, decision 8a", with "The owner can strike it". Then cover every
   result S1-plain-ends lists (success, `isError`, no result, `input_required`, `-32602`,
   `-32020` and `-32021`), reads and writes apart.
5. **Unknown outcomes and reconciliation**: an unknown write is never dispatched again by the
   core, not after a repeat and not after a restart from stored state (OPS-012, OPS-013,
   OPS-018, IMP-007); reconciliation by an authorized actor with the evidence it checked sets
   the outcome (OPS-014, OPS-030), never a timeout. Cancellation as its promises say.
6. **Records cannot erase invocations**: the invocation index is state of its own, not records,
   so that purging records leaves it whole.
7. **Vectors**: the four rows of section 3 named above, every variant they list, and F1's;
   every decision cites its promise.
8. **A finding**: as order 004's decision 6 says.

## Scope

As order 009's scope, and OPS-009 in `spec/conventions/operations.yaml`, IFC-025 in
`spec/interface/implementation.yaml` and DL10 in `spec/implementation-defined.yaml` for
decision 3, and IFC-010, IFC-046, OPS-003 and OPS-018 for decision 4.

## Done when

The vectors pass, with no unknown write dispatched again and F1 shown; every decision cites a
promise; and the checks of order 004's done-when pass.
