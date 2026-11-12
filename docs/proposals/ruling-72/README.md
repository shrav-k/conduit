# Ruling 72: metrics after batch 2 and outside v0's gate, second accounts signed in through Conduit, the Windows machine as the second machine

Status: in force (ruled by the owner on 2026-11-11). Holds now: [IN_FORCE.md: 7. Operations, outcomes, revisions, records and observation](../../../IN_FORCE.md#7-operations-outcomes-revisions-records-and-observation), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement). This record says why.

After [ruling 71](../ruling-71/README.md), the coordinator put three smaller questions from the
acceleration plan, each with a recommendation:

1. **Do metrics gate v0?** The coordinator recommended no gate, with metrics built right after
   batch 2, so that token usage by account and model can be seen while Conduit builds Conduit.
2. **Which Agents for the S3 and S10 replays?** The gate requires the owner's subscription clients
   only for S1 and S2. The coordinator recommended API-key or second-account Agents for the
   replays.
3. **A fallback second machine.** If the owner's Windows computer were late, the coordinator
   recommended a rented machine joined by a VPN as the fallback.

The owner: "1. Agreed 2. Can give you second accounts here but should test conduit authentication
with this flow 3. We can use the windows machine whenever needed, I can connect now if absolutely
necessary".

## The decisions

1. **Metrics are not part of v0's gate.** They are built right after batch 2, as ruling 68 already
   placed them, in the form ruling 69 decided.
2. **The S3 and S10 replays use second subscription accounts**, which the owner provides. Each is
   signed in through Conduit's own sign-in flow (ruling 49), so those runs also test Conduit's
   authentication. API keys are not used for them.
3. **The owner's Windows computer is the second machine** for A74 and S3, available whenever the
   work needs it. No rented machine stands in.

## What it amends

Nothing that was ruled. It answers the acceleration plan's questions D3, D6 and D8.

## Later answers

2026-11-11: the owner needed the Windows computer and asked: "can we finish up our current tasks
there and pause the environment until later when I ask you to resume?", then "You can close the
distro, I don't have anything else running". The coordinator stopped and checkpointed the work
orders running there and stopped the distro. The computer stays paused until the owner says to
resume.

2026-11-12: the coordinator put ten numbered questions to the owner. Among them were the Windows
computer's decisions as item 2 (M2-1 to M2-5 and M2-7; M2-6 is settled by PLC-023), the labels of
point 2's second subscription accounts as item 9, and whether Conduit should move into a WSL
distro of its own as item 10. The owner's whole answer: "Agree with all decisions except need more
info on 2, 3 just pause and let me know, 9 I will give you the new accounts in the morning, 10 you
can just use that distro I use tor nothing else". Item 3 is recorded in
[ruling 71](../ruling-71/README.md#later-answers). So Ubuntu-24.04, the computer's default distro,
is Conduit's distro, and the other Windows decisions wait for the owner, who asked for more
information.
