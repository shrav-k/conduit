# Ruling 50: adding machines, and the Interface as a control panel

Status: in force; done (point 1 design: rulings 52, 56); amended by 66 (point 3); amended by 77 (point 1, on the direction that Tailscale be a resource in the pool: the Windows computer joins through an SSH provider) (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment) (spec pending except where a promise is cited). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

The coordinator's [setup guide for a second machine](../../guides/second-machine.md) had the
owner install WSL2, Tailscale and SSH by hand. The owner answered: "tailscale should be a
resource in the pool, and we shouldn't have to set this all up manually, we have to figure out
the best way to set this up within our system and how to add machines in a standardized way. I
can do a manual set up for testing but adding machines should be much simpler with conduit."
The coordinator proposed Tailscale as the machine provider the specification already has
(PRV-013 to PRV-015: `list-machines.v1` and `place.v1`), noted that no operation adds a machine,
sketched an enrollment (the provider mints a single-use, tagged join key; the person runs one
command on the new machine; the machine appears and becomes a node), and put three points. The
owner's answer: "1. Yes enrollment should become a part of the language, but we should likely
do our research and go over this in detail 2. Confirm for v0 3. Yes, this is the purpose of the
interface to provide a control panel and bubble up all pending actions".

## The points and the answers

| # | Point | Answer |
| --- | --- | --- |
| 1 | Whether adding a machine, enrollment, becomes part of the language, as an operation of the machine provider's capability that returns an action for a person | **Yes, in direction.** Its design waits for research and a detailed review with the owner before anything is written into the specification |
| 2 | Whether a machine that joins becomes a node at once, or once a person confirms | **Once a person confirms, in v0** |
| 3 | Whether actions waiting on a person become one mechanism, sign-ins of ruling 49 and enrollment alike | **Yes.** The Interface is a control panel: every action waiting on a person is raised there |

## What follows

- A proposal on enrollment, from research into how existing systems add a machine, comes back
  to the owner before any promise is written.
- The specification gains one notion of an action pending for a person, raised at the root
  Interface in v0, of which ruling 49's sign-in is the first kind.
- The manual setup of the [second machine](../../guides/second-machine.md) is for testing only:
  A74, the probe that needs no engine, and S3 until enrollment exists.
- Tailscale is a candidate machine provider, beside the second providers the owner named in
  [the owner's direction](../owner-direction/2026-11-10.md).
