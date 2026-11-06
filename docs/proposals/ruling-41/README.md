# Ruling 41: the review of the whole design, and the secret type

Status: **ruled by the owner on 2026-11-06** (plan-audit ruling 41). This is the record of a
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
