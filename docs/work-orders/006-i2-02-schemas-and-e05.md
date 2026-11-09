# Work order 006 (I2-02): authored schemas, the tool definitions, and E05

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema every build order of increment 2 shares. This order has network access, to fetch crates;
use it for nothing else.

## What and why

The second of increment 2's build orders, I2-02 of the
[study](../proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders). The
core now has its first transition (order 004). This order gives it the evaluation of the
schemas that authors write: defaults, validation against a schema in the context of its
contract, and the publication of a tool's standalone definition, which round 6's E05 is about.
A tool definition that the engine publishes must mean what the authored schema means, with no
resolver behind it.

Read: [the study](../proposals/first-build/increment-2/plan.md), section 3's "E05 first" table
and the two publication checks after it; [the admin's decisions](../proposals/first-build/increment-2/decisions.md),
decision 6 above all; IFC-011, IFC-033, NAM-014, NAM-023, SCH-001, SCH-009, SCH-015, SCH-016,
SCH-019, SUB-005, TPL-011 and TPL-013, and what they cite; round 6's
[decision record](../proposals/reaudit/round-6/DECISIONS.md#the-four-open-points) on the root
`$defs` key; [the conformance README](../../conformance/README.md); and the core and the runner
as order 004 left them.

## The decisions to apply

1. **Reuse the loader's validators.** The core evaluates an authored schema with the profile's
   rules (formats asserted, patterns as SCH-016 says, defaults as SCH-009 and SCH-019 say),
   reusing the loader's validator and its pattern engine. No new dialect.
2. **Publication**: the core gives the tool definition of an operation of an open Conduit as
   IFC-033 says: a standalone schema, each contract it reaches copied once under its key, every
   local reference rewritten so it resolves within the definition, siblings of a `$ref` kept,
   finite on recursion, with no network and no package resolver.
3. **E05's vectors**, every row of the study's table and the two publication checks after it:
   each validates its samples twice, against the authored schema in its contract's context and
   against the published definition with no resolver, and compares the verdicts, not the
   wording. Cover `inputSchema` and `outputSchema`.
4. **The root `$defs` key** (decision 6, settling the study's F2): an authored schema whose root
   `$defs` holds a key of the form IFC-033 gives a contract's copy is rejected under IFC-033
   when its document is installed or submitted. Amend IFC-033 to say so, with the diagnostic
   it gives, add a vector, and log the fix in `docs/proposals/first-build/log-increment-2.md` as
   "Settled by the admin, decision 6", with "The owner can strike it". If the change belongs in
   `check_schemas.py` too, so that an example shows it, make it there with an example.
5. **A finding**: as order 004's decision 6 says. Stop a case the text does not decide; log it
   "For the admin".

## Scope

Create and change: `conformance/`, `reference-model/crates/core/`,
`reference-model/crates/conformance/`, the loader's public interface where the core needs its
validators, `reference-model/Cargo.toml` and `Cargo.lock`, `reference-model/README.md`, IFC-033
in `spec/interface/implementation.yaml` for decision 4, `tools/check_schemas.py` and
`spec/schemas/examples/` if decision 4 needs them, and
`docs/proposals/first-build/log-increment-2.md`. Report anything else.

## Done when

Every E05 vector and both publication checks pass, with the two verdicts equal on every sample
and no resolver used for the published definition; the root-key vector passes; every decision
cites a promise; and the checks of order 004's done-when pass, with the vectors run by
`conduit-conformance`.

## Record

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 18 minutes, then a second round. The core's evaluation of authored schemas and its
  standalone tool definitions; 28 vectors for E05 and the publication checks; IFC-033 amended
  by decision 6; one point left to the admin (2-03).
- **Verifier**: 4 minutes. Four `must`: a narrowed control's references left dangling; the
  pattern adapter changed what a reference to a declared property means; defaults not filled
  under `additionalProperties` and `patternProperties`; and the runner's check that state is
  unchanged could not fail, so it hid every other fault of that kind.
- **Admin**: settled 2-03 (different unconditional defaults for one place are rejected under
  SCH-009) and sent the worker back once with it and the four findings. Each fix has a vector
  that failed before it; 46 vectors pass, and none of the first 41 failed once the check could.
