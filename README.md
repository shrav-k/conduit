# Conduit

Conduit connects models and agents through shared memory, rooms, and scoped resource
access. It preserves native tools and skills while adding a common layer for
continuity, communication, and eventually observation and enforcement.

**Status:** current specification for collaborator review; no implementation yet.
The core will be native Rust, initially for personal use with private local
configuration and shareable code.

Read these documents in order:

1. [SPEC.md](SPEC.md) — purpose, terminology, expected behavior, and acceptance scenarios.
2. [DESIGN.md](DESIGN.md) — ownership, architecture, and behavioral contracts.
3. [REVIEW.md](REVIEW.md) — the remaining choices before implementation.

These files describe the current design. Update the relevant section when a choice
changes; use Git history for past versions. Keep recommendations visibly separate
from requirements. Implementation planning follows agreement on the first workflow.
