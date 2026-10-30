# Conduit

Conduit is built around **conduits**: typed, authorized relationships for both resource
access and communication between sessions. This interaction model is the project's core.
Pools organize extensible collections of resources; filters narrow what a conduit permits;
roles configure the sessions that use them. Shared memory, native tools and skills,
continuity and observation operate through this common model.

Conduit owns its runtime in Rust. The first frontend is a custom Conduit terminal
inspired by Pi, with selective component reuse where useful. Pi's agent loop and
Pi Durable are implementation references, not runtime dependencies.
The common concept is established. Specific interaction contracts and YAML relationships
are the next [research and review priority](REVIEW.md#conduit-interaction-research).

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
