# Ruling 60: the agent-owning family is a Fleet

Status: **ruled by the owner on 2026-11-10** (plan-audit ruling 60). This is the record of a
ruling. Where it and the specification differ, the specification governs.

Ruling 59 accepted the structure of the family that owns Agents and left its name open, after the
owner asked for "a different name for project that highlights the agentic theme". The
[naming research](../projects/naming.md) recommended Fleet, with Flotilla as a fallback if "fleet"
were kept for groups of machines, and Project as the baseline. The owner: "Fleet".

## The decisions

- **The family is named Fleet.** `kind: Fleet`; the core template `core#fleet` and Type
  `core#fleet.v1`; the capability `core#fleet-management.v1`, beside `pool-management.v1`; promise
  IDs `FLEET-`, mirroring `POOL-`. Ruling 30's one authority reads "its owning Pool for a Resource,
  its owning Fleet for an Agent, `management` for everything else". In the short form, `fleet: <name>`
  at the top of a file; in the CLI, `conduit fleet ...`. Where ruling 59, the owner's direction and
  the [brief](../projects/brief.md) say "Project", read "Fleet".
- **"project" keeps its meaning** (ruling 59, A5): the documents and the one manifest handed over
  together in one registration. Nothing is reworded.
- **The definition states what a Fleet is not**, since the word's meanings elsewhere pull against
  the family's rules: owning a Fleet permits nothing, and running an Agent's turns takes a Conduit to
  the Agent; a Fleet applies nothing to its Agents together and runs nothing; it is engine-local, has
  no placement, contains no Pool and is not a group of machines; an Agent never changes Fleet. Prose
  avoids "fleet operator", since the short form's `operator:` key is root's turn Conduit.
- **"fleet" is taken for Agents.** A later grouping of machines, if v1 needs one, takes another word;
  the enrollment proposal's use of "fleet" for fleetdm's host groups is reworded when it is next
  condensed.
- S1's Conduit keeps its local name `project`, an author's choice, for comparability with the
  baseline.

## What follows

The Fleet family is written in the first spec batch with ruling 55's short form and ruling 53's
additions; group B of the brief, the authority of a Fleet, is put to the owner next.
