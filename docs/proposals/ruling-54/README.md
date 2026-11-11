# Ruling 54: agents stay in their project, and revocation is individual

Status: in force; amended by 60 (Fleet); amended by 61 (B9, B10) (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach) (spec pending except where a promise is cited). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

Under [the owner's direction for Projects](../owner-direction/2026-11-10-projects.md), a Project is
to its Agents what a Pool is to its Resources, and a Project can hold Conduits to Pools and to
`access`, through which its managers grant its Agents use of Resources. The coordinator asked two
questions: whether an Agent can move between Projects, and what happens to the Conduits granted to
a Project's Agents when the Project loses the authority under which they were granted. The owner
answered: "No, a new agent needs to be declared in the new project. Agents can be given their own
conduits to pools and resources, these agents need their conduits to be revoked individually when a
project loses their conduit I think". The coordinator proposed how individual revocation is kept
from leaving forgotten access, and asked whether root may keep such a grant on purpose. The owner:
"Yes the root can deliberately keep it, when the agent is retired the conduits automatically close.
This is the right model".

## The points and the answers

| # | Point | Answer |
| --- | --- | --- |
| 1 | Can an Agent move to another Project? | **No.** An Agent belongs to one Project for its whole life, as a Resource does to its Pool; an Agent wanted in another Project is declared there as a new Agent |
| 2 | When a Project loses a Conduit under which its Agents were granted Conduits, are those grants closed with it? | **No: each is revoked individually.** Taking away the authority to grant does not undo the grants made under it, as closing a delegation today closes none of the Conduits opened under it (ACC-030) |
| 3 | How forgotten access is prevented | Every Conduit records the authority it was granted under: who opened it and through which Conduit. Closing an authority's Conduit lists, in its result, every Conduit still open that was granted under it. Those grants are raised as a pending action at the Interface, to be reviewed, and each is closed individually, with its own record. Inspection marks them as granted under a closed authority until each is closed or kept |
| 4 | May root keep such a grant on purpose? | **Yes.** Keeping it is recorded, and the mark is removed |
| 5 | An Agent that is retired | Its Conduits close automatically, as the Conduits of which a retired instance is the consumer do today |

## What follows

These are written into the investigation of the Project model, and into the specification with it.
