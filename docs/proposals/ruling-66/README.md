# Ruling 66: scoped Interfaces

Status: in force (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach) (spec pending except where a promise is cited). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

The owner asked whether, with Fleets, an Interface can simply be full authority over certain Pools
and Fleets, with only root creating Pools and Fleets and discovering all of them, and added that such
an Interface must also resolve the prompts that bubble up from its scope. A workflow tested a scope
written as ordinary Conduits against a scope stored natively on the Interface. It found the model
already ruled (the owner's direction for Fleets; ruling 59's A1, A3 and A4; ruling 61's B4 and B5),
so that what changes is how a scope is written and shown, and that "full" cannot mean a copy of root.
The owner: "I agree with all your recommendations".

## The decisions

1. **A `manages:` shorthand.** In the short form an Interface (or a Fleet's own Agent, for its own
   Fleet; or a Fleet, for Pools) writes `manages: {fleets: [...], pools: [...]}` with a role of manage
   or view. It expands at apply into ordinary Conduits, made from standard templates and opened by
   root, and inspection shows them back as a scope. Nothing about a scope is stored but its Conduits,
   so rulings 27 to 29 hold. No wildcard; `system`, `management` and `access` are never members.
2. **What a scope covers, and what it never does.** Manage covers listing and inspecting the scope's
   Agents and Resources, setting them up and reconciling them, administering the Pools its Fleets
   hold, and acting for its Fleets, within ruling 61. It never covers creating anything, retiring
   members, the cascades, `system`, use or grants of its own, turns, appointing, keeping or
   withholding, or secrets. Root may write explicit turns for the owner's own consoles.
3. **Prompts bubble up to scoped Interfaces.** A pending action is raised at every Interface whose
   scope contains its subject, as well as at root; what an Interface may answer is limited by what it
   holds: closing a grant always, keeping one only with the keep operation, completing sign-ins for its
   own Fleets' Agents; enrollment of machines and anything touching a secret stay root's. This amends
   rulings 49 and 50, which raised pending actions at the root Interface alone in v0.
4. **Reconcile by its sender.** A holder other than root reconciles, through a Conduit to a Pool or a
   Fleet, only the invocations it sent or, through a Fleet, that the Fleet or its Agents sent; the
   sibling of ruling 62's C2. It amends ACC-033's reconcile rows for holders other than root.
5. **Scopes do not replace Fleet holdings.** The larger redesign, in which scopes would replace what
   a Fleet holds and acting for it by forwarding, is declined: it would amend ruling 61's B2, B3, B7,
   B8, B11 and B12 and reverse the owner's "give conduits to projects".

6. **No Scope object and no grouping shorthand.** The coordinator advised against a Scope object (a
   named group whose new members silently widen every holder) and suggested a grouping shorthand
   might come with v1's other managers. The owner: "Understood, and I'm not sure we even need a
   grouping shorthand, declaring a new interface should be sufficient". An Interface is the named
   set: a group of Pools and Fleets is an Interface that manages them.
