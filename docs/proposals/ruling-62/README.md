# Ruling 62: five smaller points of the Fleet brief

Status: in force; open (C6 to C12); amended by 68 (point 3: C4 in batch 2) (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 6. Conduits and access](../../../IN_FORCE.md#6-conduits-and-access), [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment) (spec pending except where a promise is cited). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

Group C of the [brief](../projects/brief.md) holds points that are later work or independent of
Fleets. The coordinator explained all twelve and recommended ruling C1 to C5 now and the rest after
the S1 rerun. The owner: "Agree with your recs for c1-c5, let's revisit the rest after your tests".

## The decisions

| # | Decision | Answer |
| --- | --- | --- |
| C1 | Who may open a Conduit whose consumer is root | **Root alone**, `open.v1` included, once the existing vectors it would change are checked; today a delegate can open one, which then cannot be closed by anyone but root (ACC-024) |
| C2 | Does a bound limit `reconcile` for every delegate? | **Yes**, with C1 |
| C3 | May outsiders push Conduits into a Fleet's Agents (consent on the receiving side)? | **In v1**, as a new version of `bound.v1` with a consumers item; applied now it would change today's delegation vectors |
| C4 | Peer engines (ruling 51): does the owner's Interface admitted with full control get the peer root's defaults, appointment, keep, withhold and pending actions? | **Yes**: the peer's root template names the admitted Interface for these |
| C5 | Pools used by several Fleets | **Guidance**, every administrator shown in inspection, and a pending action when a sender other than root retires a Resource that another Fleet's Agents hold Conduits to; no rule of one administrator |

## Open, after the S1 rerun

C6 to C12: other people acting for Fleets and its prerequisites, registration through a Fleet, names
per Fleet, a Fleet setting what an Interface may use, records filtered by Fleet, root's tool count,
and reach beyond one hop.
