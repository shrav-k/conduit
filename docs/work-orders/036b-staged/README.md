# Core contracts staged by 036b

C25 of the [batch plan](../../proposals/spec-batch-1/README.md#the-order-of-the-edits)
keeps edits that require a later build out of the active core. The patches contain exact
target paths and final text. Orders 039 and 040 applied their staged edits; apply [041a](041a.md) next.
Do not copy a whole old file over a later order's changes: apply the relevant hunks after
rebasing. Order 038a adds answer and sign-in to the live capabilities and scope templates and the
staged root lists before 040 lands them. [048a](048a.md) adds the state-directory
reservation to the checker when the sign-in loader is built.

[049a](049a.md) and [049b](049b.md) record the enrollment handoff to 038b; 036b has no
enrollment patch. None of these staged edits binds until it is applied under spec/.

Each numbered Markdown file links its exact `.patch` file. The local `.gitattributes`
sets `*.patch -whitespace` so unified-diff context lines remain exact, including the
single space that represents an empty context line. The Markdown files have no embedded
patches or trailing whitespace.

The scratch checks apply the three patches verbatim to a copy outside the repository,
including the staged change to tools/check_schemas.py. No live checker or engine file changes.

[Vector impact](vector-impact.md) lists affected cases and zero-based steps.
[Tombstone validation](tombstone-validation.md) records the live and fully staged
ownership checks from round 2.
