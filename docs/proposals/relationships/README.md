# Relationships and access — accepted

Status: accepted by the owner on 2026-11-05 (plan-audit rulings 19 and 20, action A51). The decisions and rules
are written as promises in the specification: [Conduits](../../../spec/promises/conduits.yaml)
and [access](../../../spec/conventions/access.yaml). Spellings follow the
[ruled vocabulary](../vocabulary/README.md).

Amended on 2026-11-05 by the [consolidation rulings](../consolidation/README.md#rulings) (plan-audit
rulings 28 and 29). Where this record differs, the [specification](../../../spec/README.md) governs. The grant is no longer a separate object, the right to issue is an open Conduit to `access`, a
Conduit's states are registered, ready and retired, and a closed Conduit is replaced only on an
explicit request.

This is the first batch of fixes because [the relationship is the product](../../AUDIT.md#purpose).
It covers the defects in the [register](../../LANGUAGE_DEFECTS.md) that concern Conduits, grants
and access in the language and shared-convention layers.

## Ruled

| Ruling | Source |
| --- | --- |
| A Conduit is pairwise: one consumer, one target, one capability. It is opened and closed. | Vocabulary V3, ruling 17 |
| There is no "join". Two Agents sharing a conversation each hold a Conduit to the same history Resource. | Follows from V3 |
| A Conduit template is instantiated like any other: by `from`, with its consumer and target as arguments. An Agent or Resource template can declare Conduits as nested instantiations. | Instantiation IN3, ruling 18 |
| No managed access without a recorded grant; restrictions only narrow; the check happens at the target before dispatch. | Shared convention 3 |
| An agent may issue grants to others only within a bound it was given. | Ruling 17 |
| A restriction reports whether it is enforced or only mediated. | Preservation test 9 |

Closed by these rulings: joining an ended Conduit (L145), a Conduit with no consumer (L146), and
when a join target binds (L163).

## Decisions

Each was a genuine choice. The owner accepted every recommendation.

| ID | Decision | Options | Accepted | Defects |
| --- | --- | --- | --- | --- |
| RD1 | What closes a Conduit. | (a) Only explicit actions: a close, a revocation, or the retirement or declared loss of either end. (b) Also a crash, stop or pause of the consumer. | (a). A crash, stop, pause or unreachable location never closes one. Preservation test 6. | L143 |
| RD2 | Whether anything re-issues a revoked grant. | (a) Nothing does: not a restart, a repeated setup or an import. (b) Setup re-issues it under the authority of whoever runs setup. | (a). Today the text allows (b), which quietly undoes a revocation. | L121, L148 |
| RD3 | When a declared Conduit comes into existence. | (a) With its consumer's instantiation, in a "not open" state, and opened by setup. (b) Only when setup opens it. | (a). The requested access is then an object that can be inspected before anything runs, and the Conduit template is fixed at that moment. | L144, L162 |
| RD4 | What a composite Conduit is. | (a) A template that expands into several pairwise Conduits at instantiation, with no composite object afterwards. (b) A composite object that owns its components. | (a). Each Conduit can then be revoked on its own, and no aggregation rules are needed. | L140, L149 |
| RD5 | Whether the root Interface's grant can be revoked. | (a) Not through ordinary administration in v0. (b) Revocable, with an owner-verified re-issue for recovery. | (a). Option (b) makes the re-issue a standing bypass. Ownership transfer for teams is designed later. | L33, L120 |
| RD6 | A restriction the target's implementation cannot support, at open. | (a) Refuse to open the Conduit. (b) Open it with the affected operations unavailable. | (a). It fails closed and is simple to explain. | L133 |
| RD7 | What lets an actor issue a grant. | (a) An explicit, bounded right to issue grants on that target. Holding the access is neither needed nor enough. (b) Holding the access itself. | (a). The accepted text says both in different places. | L134 |
| RD8 | Whether a restriction on one Conduit applies to the same agent's other Conduits. | (a) No: each Conduit is its own relationship, and a policy on the Agent is the tool for restricting it everywhere. (b) Yes: restrictions on one target follow the agent across all its Conduits to that target. | (a). It is what the accepted text implies and should be said outright. The consequence is that an agent can read through one Conduit and write through another unless a policy on the Agent forbids it. | L129 |

## Rules

These involved no real choice once the decisions were fixed, but each had to be stated because
the text was silent or inconsistent. The owner accepted them as a group.

| ID | Rule | Defects |
| --- | --- | --- |
| RR1 | A grant made when a Conduit is opened equals exactly the access its declaration requests. A declaration that requests nothing grants nothing. | L122 |
| RR2 | Whether a request is broader than a bound is decided by comparing the listed targets and operations. Restrictions are only ever added; they are never compared. | L123 |
| RR3 | Opening a Conduit needs an actor with the right to issue its grant. This is the actor that runs the consumer's setup. Without it, a required Conduit fails setup and an optional one is reported. | L148 |
| RR4 | A consumer's declaration has at most one live Conduit. Running setup again uses it. | L144 |
| RR5 | A closed Conduit's record is kept, marked closed, until it is unregistered. Opening again creates a new Conduit with a new ID. | L147 |
| RR6 | An operation that a restriction cannot evaluate is unavailable. | L132 |
| RR7 | A target that newly matches a policy its implementation cannot support has the governed capability unavailable. | L131 |
| RR8 | Work a caller started stays subject to that caller's access whenever it later runs. | L137 |
| RR9 | When an instance calls onward through its own Conduit while serving a caller, three things travel with the request: the caller's identity, the restrictions on the Conduit the caller came in through, and any policy on the caller. The onward call is made under the instance's own grant and is limited by what travelled. This holds for a step in a graph as for any other call. | L129, L138 |
| RR10 | Every access decision, whether allowed, refused or unverifiable, leaves a record with a reason class the caller can see. | L142 |
| RR11 | Only the root Interface receives grants at first start. Any other Interface starts with none. | L31, L120 |
| RR12 | Creating a Resource is authorized on its owning Pool. | L50 |
| RR13 | A grant that exceeds a policy on its recipient is still issued. The policy limits it at use, so lifting the policy later needs no new grant. | L136 |
| RR14 | A grant is revoked whole. Narrowing one means revoking it and issuing another. An actor may revoke any grant on a target for which it holds the right to issue. | L135 |
| RR15 | When a target is retired or declared lost, its Conduits close. Their consumers stay set up and report that capability as unavailable. | L157, L159 |
| RR16 | A Conduit's record is held by management. A shared-host requirement is valid only between an Agent and a Resource, since only they have a placement; anywhere else it is rejected at validation. | L165 |

## Where each is written

| Decision or rule | Promises |
| --- | --- |
| Pairwise Conduit | CON-001 |
| RD1 | CON-007, CON-008 |
| RD2 | ACC-010 |
| RD3, RR4 | CON-002, CON-004 |
| RD4 | CON-003 |
| RD5 | ACC-024 |
| RD6 | ACC-013 |
| RD7 | ACC-006, ACC-007 |
| RD8 | ACC-009 |
| RR1 | ACC-003, ACC-004 |
| RR2 | ACC-007 |
| RR3 | ACC-005 |
| RR5 | CON-009, CON-010 |
| RR6 | ACC-014 |
| RR7 | ACC-015 |
| RR8 | ACC-019 |
| RR9 | ACC-016, ACC-017, ACC-018 |
| RR10 | ACC-020, ACC-021 |
| RR11 | ACC-022, ACC-023 |
| RR12 | ACC-025 |
| RR13 | ACC-026 |
| RR14 | ACC-011, ACC-012 |
| RR15 | CON-008, CON-011 |
| RR16 | PLC-006, CON-006 |
| Grant required; checked at the target; restrictions only narrow | ACC-001, ACC-002, ACC-008 |
| Enforced or mediated is reported | ACC-027 |

## Conduits to live instances

The owner asked whether a Conduit can be issued to an instance that is already running, and
stated that this must be a core part of the language. It can, and the earlier text allowed it
("authorized operations explicitly add or remove connections later"), but none of the decisions
above said so and the related defect (L167) had been left to the runtime profile. It is now
part of the language. The owner accepted these promises (ruling 20):

| Promise | Rule |
| --- | --- |
| CON-012 | A Conduit template can be instantiated on its own, naming an existing instance as consumer and another as target. The consumer need not have declared it, and the consumer's definition does not change. |
| CON-013 | Opening a Conduit whose consumer is already set up or running needs no stop, restart or repeated setup, and its grant is in force for the consumer's next request. |
| CON-014 | Closing a Conduit whose consumer is running leaves the consumer running, and its next request through that Conduit is refused. |

```yaml
kind: Conduit
metadata: {name: reviewer-issues}
from: capability-route
inputs:
  consumer: {kind: Agent, name: collaborator-0002}
  target: {kind: Resource, name: issue-service-0001}
  operations: [read]
```

Who may do this is already settled by the access promises: any actor, human or agent, that holds
a right to issue on the target. How a running consumer comes to see the new capability, for
example an Agent refreshing its tool list, belongs to the consumer's type contract.

Accepted with it: consumer and target are standard arguments of the Conduit family,
in the way name, owner and placement are for the others. A Conduit declared inside a template
takes the instance being made as its consumer.

## Not in this batch

- Result delivery after access changes, and what a withheld result looks like: with operations
  and outcomes.
- Classifying provider tools discovered later, and what a caller sees when content is withheld:
  with the type contracts that own them.
- How often a permission check is refreshed, and route selection among several Conduits: the
  runtime profile.
- The mechanism by which a caller proves who it is: audit question Q3, informed by the native
  probes.
