# Ruling 55: the short form, and the old inconsistencies

Status: in force; done (B: ruling 56) (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 3. Vocabulary and the project's names](../../../IN_FORCE.md#3-vocabulary-and-the-projects-names), [IN_FORCE.md: 4. Documents, templates, instantiation and submission](../../../IN_FORCE.md#4-documents-templates-instantiation-and-submission), [IN_FORCE.md: 5. Packages, the manifest, contracts, the core package and schemas](../../../IN_FORCE.md#5-packages-the-manifest-contracts-the-core-package-and-schemas), [IN_FORCE.md: 6. Conduits and access](../../../IN_FORCE.md#6-conduits-and-access) (spec pending except where a promise is cited). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

The coordinator put the decisions of [the shape of the language](../language-shape/README.md#decisions-for-the-owner)
that do not depend on Projects (group A), enrollment's fifth decision (group B) and action item A21
(group C). The owner's answer: "Agree with A and C rulings but we need to discuss B further".

## A: the short form

| # | Decision | Answer |
| --- | --- | --- |
| A1 | Whether the short form is part of the language, or a layer of the CLI | **Part of the language**: a sectioned document, defined in `spec/` by numbered translation rules into the unchanged canonical documents, which stay the kernel, the wire and the stored form; canonical documents stay legal beside it |
| A2 | How grants avoid long lists of qualified names | **Grants refer to Conduit templates for reuse** (`from: read`); operation names stay fully qualified wherever they are written (ruling 25) |
| A3 | Names across a replacement | **Stable**: an upgrade or a new directory retires the old object before making the new one under the same name, accepting a longer window without access, to be measured in the rerun |
| A4 | Approvals in v0 (ruling 53, decision 2) | **The command is the approval**: the plan is printed and recorded, every dangerous act needs a named flag (`--reopen`, `--recreate`, `--prune`, `--replace`, `--adopt`), counted when used, and narrowing never waits. Approval by people other than root is later work |
| A5 | Defaults added to a published Type | **None**: a property added to a published Type's schema states no default; the Agent Types' reach flags, false by default (ruling 53), ship as `claude-code.v2` and `codex.v2` |
| A6 | The rulings the form amends | **Amended**: rulings 8 and 18 (an inline definition makes a template and its instance together), and the wording of 31, 35 and 37 (a document's frame becomes optional, the version taken from the manifest). **Ruling 19 is kept**: a reference names its kind |
| A7 | Who may lift a tombstone (ruling 53's engine-enforced tombstones) | **An actor holding a separate operation of `access`**, which a bound can withhold, so that agents keep every management action under a grant (preservation test 5) |

## C: the old inconsistencies (A21)

Five of the eleven known inconsistencies recorded at the plan audit stand as written in files that
are marked as not binding or are not edited. **They are dropped**: they stay as written, and A21 is
closed.

## B: enrollment's fifth decision

Not decided; to be discussed further.

## What follows

The short form is written into `spec/` with ruling 53's five additions, after the Project rulings,
so that each is written once; a Rust spike under `experiments/` checks its translation first. The
derived defaults (ruling 53, decision 6) are answered with the Projects rulings, which the owner's
direction already shapes (root holds a Conduit to every Project and Pool by default).
