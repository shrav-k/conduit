# Tombstone ownership validation

Round 2, 2026-11-11. LIF-040 retains ownership; FLEET-002 fixes an Agent's owner
as a Fleet. The live `tombstone.v1` in [results.yaml](../../../spec/core/results.yaml)
requires a Resource's owner of kind Pool. [040.patch](040.patch) adds the Fleet kind
and requires an Agent's owner of kind Fleet. The Resource condition stays in force
when the owner property's allowed kinds widen to include Fleet.

The checks used `/usr/bin/python3` with PyYAML 6.0.1 and jsonschema 4.20.0, loading
every Schema contract from `spec/core/` and `spec/schemas/` through the repository's
`tools/check_schemas.py` functions `load`, `bundle`, `names` and `validator`.
Validation targeted `tombstone.v1` through the same bundled validator and format
checker used by the repository checker. Each negative case asserted rejection
and a `required` error for missing fields or a `const` error for mismatched kinds.

The base value was:

```yaml
id: tombstone-1
object: {kind: Resource, name: retired, id: object-1}
cause: {invocation: invocation-1}
time: "2026-11-11T12:00:00Z"
owner: {kind: Pool, name: owning, id: owner-1}
```

Each row changed only the named fields; the Agent rows changed `object.kind`
to Agent. The scratch tree applied 039, 040 and 041a in that order, with
`git apply --check` before each `git apply`, without fuzz or whitespace repair.

| Object kind | Owner | Live result | All staged edits applied |
| --- | --- | --- | --- |
| Resource | Pool | accepted | accepted |
| Resource | missing | rejected | rejected |
| Resource | Interface | rejected | rejected |
| Resource | Fleet | not tested | rejected |
| Agent | Fleet | not tested | accepted |
| Agent | missing | not tested | rejected |
| Agent | Pool | not tested | rejected |
| Resource | Pool, missing id | rejected | rejected |
| Resource | Pool, missing name | rejected | rejected |
| Agent | Fleet, missing id | not tested | rejected |
| Agent | Fleet, missing name | not tested | rejected |

Results: `Passed 5 tombstone ownership checks` in the live tree and
`Passed 11 tombstone ownership checks` in the scratch tree; both exited 0.
Agent and Fleet cases are checked after staging because C25 keeps the Fleet
kind and Agent ownership condition together in 040.

Both `python3 tools/check_schemas.py` and `python3 tools/check_docs.py` in the
scratch tree exited 0 with last line `Errors: 0`.

These corrections add no vector expectation changes to the existing
[impact list](vector-impact.md). No conformance case or engine file changed.
