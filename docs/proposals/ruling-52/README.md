# Ruling 52: explicit refusals, submissions, enrollment's decisions and the measurement plan

Status: in force; done (point 5: ruling 56) (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 4. Documents, templates, instantiation and submission](../../../IN_FORCE.md#4-documents-templates-instantiation-and-submission), [IN_FORCE.md: 7. Operations, outcomes, revisions, records and observation](../../../IN_FORCE.md#7-operations-outcomes-revisions-records-and-observation), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement); [SUB-010](../../../spec/promises/submissions.yaml) (spec pending except where a promise is cited). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

After the coordinator's run of S1's tasks, the coordinator put to the owner what was open:
how a refusal is explained (finding 4-08), relative values in a submission through the serving
engine (4-03), the seven decisions of the [enrollment proposal](../enrollment/README.md), and
the choices of the [measurement plan](../first-build/increment-4/measurement-plan.md). The
owner's answer: "2. I agree with this, and since we're working with agents, our rule of thumb
across conduit should be to be as explicit and descriptive as possible over all connection 3.
Agreed. Enrollment: Agreed on everything except 5 is the only thing I'm skeptical on deciding
now, it feels like this might be something we should also make customizable or research
canonical method. Measurement: Agreed".

## The points and the answers

| # | Point | Answer |
| --- | --- | --- |
| 1 | How a refusal is explained (4-08). OPS-023 has a refused request say nothing else about the objects it names | **Two tiers.** Every refusal, to any caller and to an Agent's model among them, names the operation and the target as the caller named them and says that no open Conduit from the caller to that target lists the operation; a target that does not exist gets the same answer where the caller has no authority over it. A caller that may inspect the target, and the root Interface always, is also told which Conduits from it to that target exist, open or closed, and what each lists, or that the target does not exist |
| 2 | The rule behind it | **Be as explicit and descriptive as possible over every connection**, since the callers are often agents: a result, a refusal, a diagnostic and a notice say what happened and why, in full, unless saying it would disclose what the caller may not learn |
| 3 | Relative values in a submission through the serving engine (4-03) | **Settled as the specification has it.** SUB-010 resolves relative values only where the engine is handed a directory; a request that carries values rejects them, so `conduit submit` takes absolute values. Uploading a directory is left until remote clients exist; whether relative values go altogether goes with the simplification of findings 4-01 and 4-11 |
| 4 | Enrollment's decisions 1 to 4, 6 and 7 | **Accepted as recommended**: option C, with a provider's secret-free link where it has one; a node bound to the provider's machine `id` as well as its hostname; labels named by the enrollment and bound by the provider; the join secret only in the action shown at the root Interface; refusing removes the machine at the provider, while declaring a node lost asks first; a Windows computer running WSL2 reports `linux`, to be tried in A74 |
| 5 | Enrollment's decision 5, what the person checks before admitting a machine | **Not decided.** Research the canonical methods of verifying a device, and whether the method should be customizable, first |
| 6 | The measurement plan's choices | **Accepted**: copied digests counted as lines and listed apart; shipped packages counted as provider work; the root Interface's Conduits listed apart from grants to Agents; every author command a request, and every interval without access listed |

## What follows

- OPS-023 and the refusal's form change for point 1, with vectors, and the rule of point 2 is
  written where the specification states what a result and a diagnostic carry.
- Finding 4-03 is settled; 4-08 is settled in direction, its promises still to be written.
- Enrollment's promises wait for decision 5.

## Later answers

2026-11-12: the coordinator recommended closing point 3's open question, whether relative values
go. They stay as SUB-010 has them: resolved where an engine is handed a directory, and refused
where documents are carried, so a project that uses them cannot be applied as written. The owner:
"Agree with all decisions except need more info on 2", and the whole answer is in [ruling 72's later answer](../ruling-72/README.md#later-answers). Point 3 is
closed, and relative values stay.
