# Increment 4: S1 complete

Not binding. The [specification](../../../spec/README.md) alone binds. This log follows
[ruling 46](../ruling-46/README.md), point 4. The coordinator settles findings; the owner can
strike each settlement. A finding whose fix adds or removes a feature goes to the owner. The
increment's requirements are in [the first build](../../implementation/FIRST_BUILD.md#increments).

## Findings

| ID | Class | Promises | Finding | What the build did | Settlement |
| --- | --- | --- | --- | --- | --- |
| 4-01 | gap | OBJ-002, OBJ-010, OBJ-011, LIF-009, INS-020, PKG-005 | Planning T4 to T6, the coordinator found no way to change what exists other than through templates and new objects. A submission only adds objects. One Agent's access changes only by replacing the Conduit template its nested Conduit came from, which every holder of that template takes at its next replacement, and leaves the template's name describing what it no longer grants. No operation points an existing Agent's nested Conduit at another template or Resource, and an existing Resource keeps its package version, so T5 and T6 need a new Resource and either new Agents, losing their sessions, or standalone Conduits, whose tool names differ from the declared ones. In the baseline each of T4 to T6 was one or two lines per agent. | Work order 025 finishes `install.v1`, `retire.v1` and `unregister.v1` and gives the author commands, so the tasks can run by the routes the specification allows. | The owner chose, on 2026-11-10, to run T4 to T6 by those routes and measure them against the baseline before deciding whether the language changes how what exists is changed. Under consideration: resubmitting a changed document updates the object, as `kubectl apply` does, with the engine moving the affected Agents over; or operations that point one Agent's nested Conduit at another template or Resource and move a Resource to another package version. |
