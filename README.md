# Conduit

Conduit connects models and agents through shared memory, scoped resource access,
and communication between sessions. Pools organize extensible collections of resources
for a domain. Conduit preserves native tools and skills while adding continuity and,
progressively, observation and enforcement.

Conduit owns its runtime in Rust. The first frontend is a custom Conduit terminal
inspired by Pi, with selective component reuse where useful. Pi's agent loop and
Pi Durable are implementation references, not runtime dependencies.
The communication model is being reconsidered in [REVIEW.md](REVIEW.md#deferred-communication-redesign).

**Status:** current specification for collaborator review; no implementation yet.
The core will be native Rust, initially for personal use with private local
configuration and shareable code.

Read these documents in order:

1. [SPEC.md](SPEC.md) — purpose, terminology, expected behavior, and acceptance scenarios.
2. [DESIGN.md](DESIGN.md) — ownership, architecture, and behavioral contracts.
3. [REVIEW.md](REVIEW.md) — the remaining choices before implementation.

[RESEARCH_PI.md](RESEARCH_PI.md) investigates frontend-only Pi reuse, Pi Durable's
execution/recovery mechanisms, the proposed Rust runtime, ecosystem lessons and experiments.
It is supporting evidence; proposals there do not become requirements until reviewed.

These files describe the current design. Update the relevant section when a choice
changes; use Git history for past versions. Keep recommendations visibly separate
from requirements. Implementation planning follows agreement on the first workflow.
