# Ruling 59: the structure of the agent-owning family

Status: **ruled by the owner on 2026-11-10** (plan-audit ruling 59). Its open items were answered by [ruling 60](../ruling-60/README.md): the family is a Fleet, and "project" keeps its meaning. This is the record of a
ruling. Where it and the specification differ, the specification governs.

The investigation of [the owner's direction for Projects](../owner-direction/2026-11-10-projects.md)
ended in [a brief](../projects/brief.md) with three groups of rulings: group A, the structure, to be
written before the S1 rerun; group B, project authority, after it; group C, v1. The coordinator put
group A. The owner: "I like everything and all your recommendations! But due to the
project/submission class, and since a project is a collection of agents and ownership perhaps we
should choose a different name for project that highlights the agentic theme? What do you think?
Should we research some ideas". This record calls the family "Project" until its name is chosen.

## The decisions

| # | Decision | Answer |
| --- | --- | --- |
| A1 | How Agents are owned | **Exactly as Resources are owned by Pools**: `owner` required and fixed for life; setting up, retiring, inspecting and reconciling an Agent go through its Project; registering an Agent needs `management` and its Project, since whoever registers an Agent chooses the program that runs as the engine's user. Ruling 30's one authority becomes "its owning Pool for a Resource, its owning Project for an Agent, `management` for everything else". In v0 only root registers Agents |
| A2 | Retiring a Project | **As a Pool**: refused while it owns an Agent whose shutdown is not confirmed; `retire-with-agents` retires its Agents, then the Project |
| A3 | Root's standing Conduits | To every Pool, `system` included, and every Project, made by the engine with each. They list every operation **except the cascades and acting for a Project**, so that whatever drives root, a model-driven session included, cannot retire a whole Project or Pool in one call; a cascade takes an explicit flag that opens, uses and closes a one-time Conduit, printed in the plan. ACC-024 is amended so that root may close its own Conduits other than its first-start and default ones. Their names are generated, with `as` equal to the target's name; a Pool and a Project may not share a name; `management`, `access` and `system` are reserved |
| A4 | Discovery | A new operation `list.v1` on `management`, every Pool and every Project, listing only what the caller may inspect, with no counts; an object's inventory shows its Conduits in full to whoever may inspect the object |
| A5 | The existing word "project" (the documents and manifest handed over together) | **Open**: depends on the family's name. If the family takes another name, "project" may keep its meaning |
| A6 | The family in the short form | A top-level key in a file makes the family's instance and owns every Agent in the file; an Agent elsewhere states `owner:`. The key's word follows the family's name |
| A7 | Bounds when a Resource is replaced under its name | Bound items keep the ID they resolved to; the plan of a replacement prints every bound that stops covering the Resource; guidance bounds on Pools, which cover a replacement |

## Open

- **The family's name**: the owner asked for a name that highlights the agentic theme, to be
  researched.
- Group B, put after group A, and group C, v1.
