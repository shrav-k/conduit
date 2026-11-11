# Ruling 53: the simplification's first decisions

Status: in force; done (points 1, 2: ruling 55 A1, A4; point 6: ruling 59 A3) (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 4. Documents, templates, instantiation and submission](../../../IN_FORCE.md#4-documents-templates-instantiation-and-submission), [IN_FORCE.md: 5. Packages, the manifest, contracts, the core package and schemas](../../../IN_FORCE.md#5-packages-the-manifest-contracts-the-core-package-and-schemas), [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach) (spec pending except where a promise is cited). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

The coordinator put nine decisions from the [simplification investigation](../simplification/README.md)
to the owner. The owner's answer: "1. Can we do an investigation on the short form of the language
first? This seems like a critical moment to do this 2. Let's discuss further 3. Yes to all 4. Yes
for now 5. Yes 6. Need more detail 7. Yes 8. Yes 9. yes".

## The decisions

| # | Decision | Answer |
| --- | --- | --- |
| 1 | Whether an authoring layer counts for the bar, or the short form must be the language | **Open.** First, an investigation of making the short form the language itself, since this is a critical moment for the language |
| 2 | How a change is approved without a terminal, and how it counts | **Open**, to be discussed |
| 3 | The five core additions | **Accepted, each as recommended**: `as`, a local name for a standalone Conduit; `expectedId` on `open.v1` and `close.v1`; an inventory in `inspect.v1` of the Conduits an object consumes and is the target of, listing only what the caller may inspect and giving no count of the rest; tombstones of what was closed or retired, enforced by the engine at open and register; tool descriptions that name their target and its Type's description |
| 4 | An explicit plan and apply, composed of today's requests, never automatic and never pruning or reopening without a flag | **Accepted for now** |
| 5 | Package pins written by `conduit install` into the author's file | **Accepted** |
| 6 | The defaults the CLI derives: a Pool per project, the operator's Conduit to it, a turn Conduit per Agent | **Open**, more detail asked |
| 7 | The Agent Types' reach flags default to false in their contracts, as new package versions | **Accepted** |
| 8 | Operation names stay fully qualified (ruling 25) | **Accepted** |
| 9 | The rerun's fresh author receives the specification, the tasks and the authoring format's reference shipped with the CLI, stated before the run | **Accepted** |

## What follows

The investigation of decision 1 runs before any of the accepted additions is written into the
specification, so that they are written once, in whichever form the language takes.
