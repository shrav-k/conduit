# Work order 016 (I2-07): setup, and the opening and closing of Conduits

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema every build order of increment 2 shares. This order has network access, to fetch crates
and for tests that bind a loopback port; use it for nothing else.

## What and why

I2-07 of the [study](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders):
an instance is set up in its parts, in order, and a Conduit is opened, closed, removed and
replaced as the promises say. S1's T1 and T7 rest on this: two Conduits to one directory,
opened and closed independently. Nothing here starts a process: the core emits the intents to
run and reads their completions, which the vectors script.

Read: the "Setup and opening" table of the study's section 2, for the CON, ACC, TPL, LIF, PRV,
REV and IMP promises it lists; the rows S1-setup-sender, S1-setup-again, S1-target-registered,
S1-close, S1-close-in-flight and S1-restore of section 3; the
[decisions](../proposals/first-build/increment-2/decisions.md); and the core as order 015 left
it.

## The decisions to apply

1. **Setup** in its parts, in the order its promises give, each part an invocation where the
   promises say so, with the sender's authority for each open checked as it is made; a part that
   fails leaves the parts before it, with no rollback, as the promises say; a second setup makes
   no new identity and opens nothing already open.
2. **Intents and completions**: running an implementation is an intent to the shell (PRV-003,
   PRV-007, PRV-022), and its report is an input; the core never starts a process.
3. **Opening and closing** under CON-004 and the CON and LIF promises: a closed Conduit's tools
   leave its consumer's list (IFC-013, through order 014); a result that arrives after its
   Conduit closed ends as ACC-034 and IFC-025 say; a closed Conduit is removed (LIF-035), and
   restoring one is an explicit registration of its replacement (LIF-009), never implicit.
4. **Vectors**: S1's base setup, and the six rows of section 3 named above, every variant they
   list; every part's invocation and caller links checked; every decision cites its promise.
5. **A finding**: as order 004's decision 6 says.

## Scope

As order 009's scope.

## Done when

S1's base setup succeeds by vectors, and the six rows pass, with T7's case shown: closing A's
Conduit leaves B's working; every decision cites a promise; and the checks of order 004's
done-when pass.

## Record

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 23 minutes, then a second round. Setup in its parts, intents to run and their
  completions, and the opening, closing, removal and explicit replacement of Conduits, with T7's
  case: closing A's Conduit leaves B's working; 22 vectors, then 345 in all.
- **Verifier**: 7 minutes. Eight `must`, among them Pool entries set up by name rather than by
  their dependencies, a refused hook leaving its setup in progress, cancellation or retirement
  losing an effect's unknown outcome, and a relative program sent to a runtime provider.
- **Admin**: sent the worker back once, insisting that an effect whose outcome is unknown is
  never reported failed and that nothing the promises forbid is dispatched. Each finding was
  decided by its promises (POOL-013, OPS-022, OPS-027, TPL-012, LIF-009, PRV-007, PRV-005 and
  others) and fixed with a vector that failed before it.
