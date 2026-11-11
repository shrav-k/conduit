# Ruling 76: plan and apply's five readings, and work orders on the Windows computer

Status: in force (ruled by the owner on 2026-11-11). Holds now: [IN_FORCE.md: 4. Documents, templates, instantiation and submission](../../../IN_FORCE.md#4-documents-templates-instantiation-and-submission), [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed) (spec pending, with a revision of order 069's reference and order 052a). This record says why.

Order 069 (K3) wrote the reference for `conduit plan` and `conduit apply`. Every reading that could
move S1's author counts it reserved for the owner (ruling 71, point 4). Order 073 (PB, experiment
023) counted S1 on paper under each reading.

With order 069's defaults, S1's tasks count 23, 6, 0, 3, 2, 2 and 3 lines, against the baseline's
33, 16, 0, 8, 3, 2 and 2. As a forecast, that is fewer on four tasks and much more on none, but T7
sits exactly at one and a half times its baseline.

The coordinator put the five readings with a recommendation each. It also asked whether ordinary
work-order workers may run on the Windows computer over SSH until it is enrolled as a node.
Codex Cloud's CLI cannot find current environments (openai/codex issue #50182), so the Windows
computer is the extra capacity. The owner: "agree with all recommendations, let's try this out".

## The decisions

1. **Narrowing never needs a flag.** A Conduit the file no longer names is closed on apply, and so
   is a narrowing of a Conduit's operations, as Terraform removes what its configuration drops.
   - The plan prints and records each closing.
   - Widening, replacing, reopening, recreating, adopting and cascades keep their named flags.
   - This amends ruling 53 point 4 for removal only: `--prune` is no longer needed to close a
     Conduit the file dropped.
2. **Every flag names its object, by its generated name,** such as `--replace agent-a--project`, as
   in experiment 011's ledger.
3. **Commands find the engine through a current engine context,** as kubectl's and Docker's do:
   `conduit serve` records itself as the person's current engine, and `--store` remains as an
   override.
4. **When a set-up raises a sign-in, apply goes on.** It prints each pending sign-in as the person's
   action at the panel, listed apart from authored lines (ruling 68, R1), and answers none.
5. **`conduit plan` is available and never required.**
6. **Work orders on the Windows computer.** Until the Windows computer is enrolled as a node through
   Conduit, the coordinator may run ordinary work-order workers on it over SSH, as on the Mac. That
   is the manual setup ruling 50 allows for testing. Conduit's own runs of work orders (A78) stay on
   the Mac until the Windows computer is a node.

With 1, 3 and 4, the paper forecast becomes 21, 5, 0, 3, 2, 2 and 2: fewer on four tasks, equal on
three, and none over the limit. It remains a forecast; the bar is judged on the real runs.

## What it amends

Ruling 53, point 4: pruning a Conduit the file dropped no longer takes a flag; reopening still does.

## Later answers

2026-11-11, evening: before being away until the next morning, the owner let the coordinator adopt
the recommendations of the plan for enrolling the Windows computer through Conduit during that
night, logged for the owner to strike, except a choice that changes behavior already ruled or that
exposes anything beyond the private Tailscale network: "agreed". The owner also kept the Windows
computer awake and logged in for the night.
