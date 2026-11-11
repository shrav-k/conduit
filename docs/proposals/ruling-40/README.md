# Ruling 40: Conduit innovates on connection and access; everything else is swappable

Status: in force; amended by 43 (point 3e, D5's gate); amended by 47 (the layout of `packages/`) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 1. Purpose, scope and principles](../../../IN_FORCE.md#1-purpose-scope-and-principles), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items); [PRV-001 to PRV-026](../../../spec/promises/providers.yaml). This record says why.

The secret type, which
this direction reshapes, is ruled separately, with the decisions of the
[principles review](../principles-review/README.md). This is the record of a ruling. Where it and the
specification differ, the specification governs.

The owner's direction, in the owner's words: "The underlying infrastructure should be able to
swapped out to [Bitwarden] or Mac keychain etc, we don't care as long as it follows a shared
contract." "We should not innovate on the architecture ourselves just the contracts." "We
shouldn't overcomplicate this." "The point of conduit was always to leverage existing
infrastructure where possible and only innovate on connection and access."

| # | What was put | Recommended | The owner's ruling |
| --- | --- | --- | --- |
| 1 | The line: Conduit owns its documents, connection (Conduits), access (who may reach what, under which limits, and delegation), accountability (what must be recorded of who did what) and the shape of its contracts. Everything else is a provider behind a contract. The rules on model work stay Conduit's, as access; records stay Conduit's only as what must be recorded, not how it is stored. | Yes | "Yes" |
| 2 | Swappable by the user, against left to each engine: provider contracts in the first version only where one developer swaps (secrets, the runtime that runs implementations, the source of packages); identity, isolation, the storage of records and the engine's own state stay each engine's choice for now, in standard formats. | Yes | "Agreed for now, but we may push in this direction for future versions, so we should go as far as we can in the 'swappable by user' direction, leaving decisions to the user on architecture and existing tools as long as they meet requirements" |
| 3 | How far to cut the machinery that Conduit invented: its runtime model, its location model and its language of setup graphs. | Discuss with the evidence of the [principles review](../principles-review/README.md) | "lets discuss further" |
| 3a | Replace the runtime model (runtimes declared in templates, handles, holding, restoring) with a runtime provider contract: run an instance's implementation, stop it, report its status. | Yes | "Yes" |
| 3b | Replace the location model with placement by the provider and labels, with one implicit location in the first version. | Yes | Not as put: "multi machine workflows and coordination across resources and agents across multiple machines is a core part of the model and the type of access we seek to offer. We can absolutely offer swappable architecture around this but we MUST ensure the system works robustly in a distributed manner." Revised below. |
| 3b, revised | Locations stay in the language, and a provider reports them: which machines exist, their labels, and when one is lost (the root Interface may still declare one lost). The provider places an instance from its labels. What placement and loss mean for access stays Conduit's promises: where each instance runs, a Conduit that needs both ends on one machine, what a lost machine retires and closes. Removed: locations registered by hand as Resources, their facts, the setting of a default location, and Conduit's own rules for choosing a placement. | Yes | "Agree" |
| 3e | A request that crosses machines carries cryptographic proof of the instance that sent it. The mechanism is swappable: mutual TLS, SPIFFE, or the authorization of the Model Context Protocol. | Yes | "Agree" |
| 3f | Who decides access across machines. A: one control plane decides every request against the current state, so revocation is immediate; the engine survives the loss of a machine by keeping its state in a store that does (etcd, Postgres). B: each machine enforces access from configuration pushed to it, as a service mesh does, and keeps working when cut off from the control plane, at the cost of a delay before a revocation arrives. | A for the first version | "Agree, with A for v0, though a robust version of B will likely be a requirement for v1" |
| 3c | Reduce the language of setup graphs to declarations of dependencies; an operation built from other operations becomes an ordinary implementation, which any workflow tool may provide. | Yes | "Yes" |
| 3d | Make these changes now, before round 5 of the re-audit and the cold read by Codex. | Yes | "Yes" |

## What follows from points 1 and 2

- Every part of the design is tested against point 1. What is not connection, access,
  accountability or the shape of a contract becomes a provider behind a contract, or is left to
  the engine.
- Where the first version leaves a choice to each engine, it is written so that it can become a
  provider contract later without breaking anything: in a standard format, with no assumption of
  one backing system. Where the owner's direction allows it, the choice goes to the user, not the
  engine.
- Work across several machines is part of what Conduit is for (point 3b). The design keeps the
  meaning of distribution as Conduit's own: where each instance runs, what a request reports when
  a machine or the network fails, and how access stays correct across machines. The mechanics
  (scheduling, the network, the liveness of machines, the engine's own state, how an identity is
  proved) are providers or the engine's choice.
- The first version decides every request in one control plane (3f, A), as the specification
  already promises (ACC-036, ACC-037). A robust form of local enforcement (3f, B) is likely a
  requirement of the next version. So the first version must not rule it out: nothing outside the
  promises on how a decision is made may assume that decisions are made in one place, and the
  promise that no earlier decision stands in for one that cannot be made is the one place to
  revisit, with a stated bound on how long a revocation may take to arrive.

## After the ruling

Two further answers of the owner, on 2026-11-06:

| What was put | The owner's answer |
| --- | --- |
| Where the first providers' code lives: a top-level `packages/` directory of Rust crates, one per provider, beside the planned `conformance/` and `reference-model/`. | "Yes" |
| Start the probe of the macOS keychain now (Experiment 005), on a throwaway keychain, never the login keychain, knowing that macOS might show a permission dialog. | "Yes" |
