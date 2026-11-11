# Ruling 51: managed machines and peer engines

Status: in force; amended by 56 (point 8, answer); amended by 62 (C4) (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment) (spec pending except where a promise is cited). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

The owner asked whether the enrollment of ruling 50 generalizes beyond one owner's local
network: "We may be on a local net for now but eventually we may allow for multiple machines to
communicate without direct control." The coordinator had other networking paradigms researched
([networking paradigms](../enrollment/networking-paradigms.md), beside the
[enrollment proposal](../enrollment/README.md)) and put ten decisions. The owner's answer:
"Agreed on all of these decisions with one caveat, I should be able to connect to agents on
other machines as the root interface correct? I mean there should be able to still add a fully
managed machine to my purview, this capability should exist alongside communication. For
example if I wanted to be able to fully control the wsl on my windows computer or simply
communicate with conduit engine on it both modes should be supported".

## The decisions

All ten of [section 7](../enrollment/networking-paradigms.md#7-decisions-and-open-questions-for-the-owner)
are accepted as the coordinator recommended:

1. Network membership grants nothing; identity is per instance; the target's owner authorizes
   each request. Stated as a principle, which PRV-018 and IFC-022 already carry.
2. The engine names its trust domain at first start, in SPIFFE form, unique, recorded in status.
3. Another party's machine is never a node (as amended below).
4. A relationship across parties is two halves, each opened and closed by its own engine.
5. Ruling 50's pending action is generic: a kind, a subject, evidence, an expiry and an outcome.
6. No capability tokens in v0; across parties in v1, OAuth tokens bound to the receiving engine.
7. Between parties, `https` or an outbound connector behind a provider, not a shared overlay.
8. Enrollment's machine `id` is required, opaque, scoped to the provider and derived from the
   machine's key where the provider offers one.
9. A2A, ANP and NANDA are watched and none is adopted in v0.
10. A partner's caller chain is recorded and not used in decisions, for now.

## The caveat: two modes, both supported

A machine the owner controls may be connected in either of two modes, and both are supported:

- **Managed.** The machine is enrolled (ruling 50) and becomes a node of the owner's engine. The
  engine places and runs instances there, and the root Interface reaches its Agents as it
  reaches any other, through Conduits. Example: the engine runs Agents inside WSL on the owner's
  Windows computer.
- **Peer.** The machine runs an engine of its own, and the two engines are introduced to each
  other. Each decides requests into its own objects. Where both engines are the owner's, the
  peer may admit the owner's root Interface as an Interface of its own with Conduits to its
  `management` and `access`, so that the owner controls it fully from the first engine, while it
  keeps its own records, decisions and trust domain.

Decision 3 is read with it: what makes a machine eligible to be a node is that its engine's
owner controls it, not where it is. A machine of another party is only ever a peer.
