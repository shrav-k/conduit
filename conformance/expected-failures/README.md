# Expected failures

Each order that lists a case the build does not pass yet writes its entries here, in a fragment
of its own named for its order file with `.yaml` for `.md`, in the form
[conformance/README.md](../README.md#expected-failures) states; the runner reads every fragment
as one list (DEC-190). This file is no fragment: it keeps the directory when no case is listed.

A fragment is named as its order's file under `docs/work-orders/` with `.yaml` for `.md`,
or, for a change with no order, as its branch with `-` for `/`: a lowercase letter or digit,
then lowercase letters, digits, `.` and `-`, then `.yaml`. The name is neither `index.yaml`
nor `case.yaml`. Put the regular file directly under this directory. It holds JSON with a
nonempty `cases` list, written with `serde_json`'s pretty printer and a final newline.

For example, `415-expected-failures-as-fragments.yaml` could hold this one-entry fragment:

```json
{
  "cases": [
    {
      "id": "access-ask-offered",
      "reason": "ACC-058: the first build does not offer request-access.v1; DEC-064's engine orders make it pass"
    }
  ]
}
```
