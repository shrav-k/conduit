# Ruling 41: the review of the whole design, and the secret type

Status: in force; amended by 42 (D6, D8, D12, names); amended by 43 (item 4c, Q3); amended by 44 (D14); amended by 58 (M22, M41) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 5. Packages, the manifest, contracts, the core package and schemas](../../../IN_FORCE.md#5-packages-the-manifest-contracts-the-core-package-and-schemas), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets); [PRV-001 to PRV-026](../../../spec/promises/providers.yaml), [SEC-001 to SEC-023](../../../spec/promises/secrets.yaml). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

After ruling 40 the coordinator had the whole design reviewed against the owner's principles
(Conduit innovates only on connection and access; infrastructure is swappable behind contracts;
nothing is overcomplicated). The [review](../principles-review/README.md) put 31 decisions to the
owner, listed the alignments the coordinator would make and three changes to choices that rulings
had accepted, and reduced the secret type to a contract that any store implements (its section
4). The coordinator recommended accepting all of it, and noted that items 3a (removing policies),
5 (removing composite Conduits) and 6a (a result delivered after its Conduit closed) remove or
weaken parts of access, which is Conduit's own ground.

The owner's ruling: "Accept all, though I don't necessarily like the name secret reveal for the
contract, we should do another vocab audit as well before we finish up pr 7".

## What follows

- Every decision of section 2 of the review is accepted as recommended, with each lettered line
  as recommended: items 1 to 31.
- The alignments of section 3 are accepted: C33, C51, C52 and C58, and the three changes to
  choices that rulings accepted, C14, C20 and C21.
- The secret type is ruled as section 4 of the review reduces it. The
  [secrets proposal](../secrets/README.md) is the reasoning and the record of the options; where
  it and section 4 differ, section 4 governs. This settles point 1 of ruling 39.
- Removing policies (item 3a) is recorded as a candidate access feature to design for a later
  version, as Conduit's own, rather than only as an outside decision point.
- The name of the shared contract of secret stores, `secret-reveal.v1`, and of its operation,
  `reveal-secret.v1`, are not final. A vocabulary audit of the whole specification, as it stands
  after this ruling is applied, proposes names before pull request 7 is finished, and the owner
  rules on them.
- The specification is rewritten around provider contracts as rulings 40 and 41 say, then read
  by the vocabulary audit and by a fifth round of readers, before pull request 7 is merged.

## After the ruling

The planner of the rewrite ([PLAN.md](PLAN.md), "Planner's choices") had to fill twenty gaps
that the review left open, P1 to P20, and the coordinator put them to the owner while the
editors worked. The owner's answer, on 2026-11-06: "Accept all with p2 changed".

- P1 to P20 are accepted as the plan states them, except P2.
- P2 is changed: `spec/promises/graphs.yaml` is not kept as a file with no promise. It is
  deleted, and `spec/README.md` lists the promise files that rulings removed, with the IDs each
  held, so that no removed ID can be reused.
- The names that P1 coins are not final: the vocabulary audit reviews them with
  `secret-reveal.v1` and `reveal-secret.v1`.
