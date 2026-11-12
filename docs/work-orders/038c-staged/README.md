# Sign-in logins and return handoffs

Order 038c writes the logins a Type states and the browser login's return in the specification.
The live schema case is
[contracts-sign-in-logins](../../../conformance/language/contracts/sign-in-logins/case.yaml).
The core remains unreleased at 0.1.0.

[048a](048a.md) builds the stated logins, account checks and sign-in core before the S1 rerun.
[Return](return.md) hands off PND-013 after 048b, before S3, and the package and CLI parts.
[Vector impact](vector-impact.md) records the search of existing expectations.

No 038c patch is staged. Apply the [036b patch](../036b-staged/048a.patch), then the
enrollment patches for [049a](../038b-staged/049a.patch) and
[049b](../038b-staged/049b.patch), in that order.
No engine, package, checker, ruling record or enrollment patch is changed by this order.
