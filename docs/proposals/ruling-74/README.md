# Ruling 74: `main` protected before Conduit runs its own orders; S1's own-tools switch, counted and temporary

Status: in force (ruled by the owner on 2026-11-11). Holds now: [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach), [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement), [IN_FORCE.md: 14. Publication and organization](../../../IN_FORCE.md#14-publication-and-organization). This record says why.

The coordinator's plan for A78 under [ruling 70](../ruling-70/README.md) found two things for the
owner.

First, in v0 a worker Agent runs as the owner's user, with nothing confining it. So it can push
with the owner's credentials, and `main` had no branch protection.

Second, the `.v2` Agent Types turn the Agent's own tools on by default. The S1 rerun tests the
communication framework, so its Agent templates need `ownTools: false`, the switch ruling 70
point 3 provides. That is one line per Agent that the author writes.

The coordinator recommended:

1. protecting `main` before the first order that Conduit runs: changes only through pull requests,
   for administrators too, and no force-push or deletion;
2. counting the switch as an authored line under ruling 63, since honest counting counts it.

The owner: "1 and 2 I agree, though the flag for 2 should be temporary and we should discuss a more
durable solution than this flag".

## The decisions

1. **`main` is protected before Conduit runs its first work order.** Changes arrive only through
   pull requests, for administrators too, and `main` cannot be force-pushed or deleted. This guards
   against destructive mistakes. It does not stop an Agent that uses the owner's `gh` login from
   opening and merging a pull request. The other guards stay: each worker works in a clone that
   cannot push, and the harness checks after each turn that nothing was pushed.
2. **The S1 rerun's Agent templates write `ownTools: false`, and each such line is counted** as an
   authored line under ruling 63.
3. **The switch is temporary.** A more durable way to state and bound what an Agent reaches through
   its own tools is to be designed and put to the owner. The coordinator started that research the
   same day.

## What it amends

Nothing that was ruled. Point 3 marks ruling 70 point 3's switch as temporary.
