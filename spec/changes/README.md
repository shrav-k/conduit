# Changelog fragments

Each change to `spec/` writes its lines here, in a file of its own, in the form
[spec/CHANGES.md](../CHANGES.md) states; a release moves them into it (DEC-177). This file is
no fragment: it keeps the directory when a release empties it.

A fragment is named as its order's file in `docs/work-orders/`, or, for a change with no
order, as its branch with `-` for `/`: a lowercase letter or digit, then lowercase letters,
digits, `.` and `-`, then `.md`. Put it directly under `spec/changes/`. It holds only list
items, at least one, with no heading; a wrapped item indents its continuation by two spaces.
For example, a fragment for this order could hold this item on two lines:

```markdown
- Changed: `spec/README.md#contents` states no count;
  the checkers give the totals (DEC-177).
```
