# Declarations: references, instantiation, submissions and Pool entries — accepted

Status: accepted by the owner on 2026-11-05 (plan-audit ruling 21, action A52). Written as promises
in the specification: [objects](../../../spec/promises/objects.yaml),
[instantiation](../../../spec/promises/instantiation.yaml) and
[Pools](../../../spec/promises/pools.yaml). The owner asked what a "document" is concretely before the submission promises were
written; after that explanation they were accepted (ruling 22) and are in
[submissions](../../../spec/promises/submissions.yaml). Spellings follow the [ruled vocabulary](../vocabulary/README.md).

This is the third batch of fixes, and it is entirely in the language layer: everything here can
be answered by reading declarations. It is the part the two independent builds must agree on
exactly, since for them the language is a function from declarations to resolved objects or
diagnostics.

Defects covered, from the [register](../../LANGUAGE_DEFECTS.md): L49, L84, L105, L107, L108,
L109, L112, L113, L115, L117, L118, L150, L160, L161 and L164.

## Decisions

The owner accepted every recommendation.

| ID | Decision | Options | Accepted | Defects |
| --- | --- | --- | --- | --- |
| DD1 | When a reference becomes an ID. | (a) Templates hold names; instances hold IDs. A reference resolves once, when the instance that holds it is created, and never again. A name in a template that does not resolve yet is reported but accepted. (b) A template's references resolve when the template is registered. | (a). A template stays a reusable definition that fixes no particular object, registration order between templates stops mattering, and replacing a template never has to re-resolve anything. | L107, L108 |
| DD2 | What a submission of several documents is. | (a) A set. References among its documents are resolved together, the order of work is derived from those references, and if any document is invalid nothing is registered. (b) An ordered batch whose author writes the prerequisites between entries, as the text says today. | (a). This is how Docker Compose and Terraform treat a configuration. It removes authored ordering between documents and the rules that came with it. | L112, L113, L115 |
| DD3 | How many Resources a Pool entry can produce. | (a) At most one live Resource per entry. Any further Resource in the Pool is made by ordinary instantiation with that Pool as owner. (b) An entry is a reusable recipe that can be provisioned many times, as the text says today. | (a). It reverses a sentence of the accepted text, whose example was two worktrees from one entry; that case is served by ordinary instantiation. In return a reference from one entry to another is unambiguous, and an entry can carry a name. | L150, L161 |
| DD4 | Provisioning an entry that refers to another entry not yet provisioned. | (a) Refuse, naming the missing entry. One request may name several entries, and they are provisioned in the order their references require. (b) Provision the missing entry first, automatically, as Docker Compose starts a dependency. | (a). Provisioning has effects, such as a login, and preservation test 6 says no effect starts implicitly. | L160 |
| DD5 | A relative path in a declaration. | (a) Not allowed in an instance. A path argument is absolute, and a frontend may resolve a relative path before submitting. (b) A standard base argument with a defined join. | (a). A path is interpreted at the instance's placement, where the submitter's working directory means nothing. | L84 |

## Rules

These involved no real choice once the decisions were fixed. The owner accepted them as a group.

| ID | Rule | Defects |
| --- | --- | --- |
| DR1 | Two inputs of one template may not name the same destination, or one inside the other. Such a template is rejected at registration. | L105 |
| DR2 | A destination must exist in the template: its parent is present in the template's definition. Otherwise the template is rejected at registration. | L105 |
| DR3 | An optional input that is not supplied and has no default assigns nothing. Its destination keeps the value the template gives it, if any. | L105 |
| DR4 | A supplied null is assigned only if the input's schema permits null. It is never treated as "not supplied". | L105 |
| DR5 | An instance's definition is never rewritten through the registry. Renaming an instance is its own operation, checked against the revision, and changes only the name. | L109 |
| DR6 | A submitted document that carries a system-owned field, such as an ID, a revision or status, is rejected, even if the values are correct. | L117 |
| DR7 | A name is made of lowercase letters, digits and hyphens, starts with a letter, and is at most 63 characters. Names are compared exactly. | L118 |
| DR8 | An instance created without a name gets its template's name followed by a generated suffix. If that name is taken, another is generated. | L118 |
| DR9 | A submission in which two documents would produce the same kind and name, or change the same object, is rejected whole. | L112, L113 |
| DR10 | A submission may contain templates and instances. Nothing else is a document. | L115 |
| DR11 | A reference from one Pool entry to another is the other entry's name. It resolves to that entry's Resource when the referring entry is provisioned. | L160 |
| DR12 | An entry's placement comes from the Pool template's inputs, whose destinations are the entries. A Pool template never names a location. | L164 |
| DR13 | The text that describes creating an instance by editing a template's fields is replaced by the instantiation rulings. | L49 |

## Where each is written

| Decision or rule | Promises |
| --- | --- |
| DD1 | OBJ-008, OBJ-009, OBJ-010 |
| DD2 | SUB-001, SUB-002, SUB-003, SUB-006, SUB-007 |
| DR9 | SUB-004 |
| DR10 | SUB-005 |
| DD3 | POOL-003, POOL-004 |
| DD4 | POOL-007, POOL-008 |
| DD5 | INS-018 |
| DR1 | INS-010 |
| DR2 | INS-011 |
| DR3 | INS-008 |
| DR4 | INS-009 |
| DR5 | OBJ-011 |
| DR6 | OBJ-004 |
| DR7 | OBJ-001 |
| DR8 | OBJ-003 |
| DR11 | POOL-005, POOL-006 |
| DR12 | POOL-009 |
| DR13 | INS-001 to INS-017 state the instantiation rulings; the protocol text is replaced when its topic is converted |

The instantiation rulings themselves (IN1 to IN10) are INS-001 to INS-017, and the accepted
ownership rules are POOL-001 and POOL-002.

## Not in this batch

- Graphs and the schema profile: the next batch.
- Packages, first start and the conformance profile: the batch after that.
- Applying a submission to a running system, and what happens if that is interrupted: the
  runtime profile.
