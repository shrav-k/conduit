# Vector impact of 038a

This is a source audit of existing expectations, not a run of a new engine. Steps are zero
based. No vector or expectation was edited. The 036b vector-impact report separately records
036a's Fleet ownership, root defaults and routing changes; those are not new effects of 038a.

IFC-050 requires a deciding diagnostic for every reason and engine-decided failed or unknown
outcome; ACC-056 requires the refusal explanation. The following assertions explicitly deny
that diagnostic or assert a bare first text block. Arrays are compared in order; adding fields
to an existing map projection alone does not contradict it. A bare implementation text block
is passed unchanged after the engine's explanatory first block under IFC-025.

## Explicit absence of diagnostics

| Case | Steps |
| --- | --- |
| `conformance/conventions/access/bounds/case.yaml` | 2, 3, 7, 8, 12, 13, 14, 15, 17 |
| `conformance/conventions/access/current-tools/case.yaml` | 3, 4 |
| `conformance/conventions/access/identity/case.yaml` | 1, 2, 3, 4 |
| `conformance/conventions/access/s1-exact-access/case.yaml` | 13, 15, 17 |
| `conformance/conventions/access/s1-plain-filter/case.yaml` | 1, 2, 3, 4, 5 |
| `conformance/conventions/access/s1-pool-authority/case.yaml` | 1, 4 |
| `conformance/conventions/records/expected-id/case.yaml` | 2, 5, 7 |
| `conformance/language/first-start/protected/case.yaml` | 2, 3, 4 |

## Root-management exact operation lists (040)

| Case | Steps |
| --- | --- |
| `conformance/conventions/first-start/initial/case.yaml` | 0 |
| `conformance/conventions/first-start/lifecycle/case.yaml` | 0 |
| `conformance/conventions/first-start/repeat/case.yaml` | 0, 1 |
| `conformance/conventions/s1/whole/case.yaml` | 0 |
| `conformance/language/first-start/protected/case.yaml` | 0 |

## Bare error text contradicts the required explanation

| Case | Steps |
| --- | --- |
| `conformance/conventions/records/expected-id/case.yaml` | 5, 7 |
| `conformance/conventions/requests/aware-failed/case.yaml` | 2 |
| `conformance/conventions/requests/aware-headers-no-repair/case.yaml` | 2 |
| `conformance/conventions/requests/aware-malformed/case.yaml` | 2 |
| `conformance/conventions/requests/aware-unknown/case.yaml` | 2 |
| `conformance/conventions/requests/cancel-confirmed/case.yaml` | 2 |
| `conformance/conventions/requests/cancellation/case.yaml` | 2, 4 |
| `conformance/conventions/requests/effectful-plain-headers/case.yaml` | 2, 3 |
| `conformance/conventions/requests/failed-read-repeat/case.yaml` | 2 |
| `conformance/conventions/requests/invalid-outputs/case.yaml` | 2 |
| `conformance/conventions/requests/key-scope/case.yaml` | 2, 3, 4, 5, 9 |
| `conformance/conventions/requests/plain-content/case.yaml` | 5 |
| `conformance/conventions/requests/plain-model-work/case.yaml` | 2 |
| `conformance/conventions/requests/read-capability-error/case.yaml` | 2 |
| `conformance/conventions/requests/read-input-required/case.yaml` | 2 |
| `conformance/conventions/requests/read-invalid-parameters/case.yaml` | 2 |
| `conformance/conventions/requests/read-is-error/case.yaml` | 2 |
| `conformance/conventions/requests/read-missing/case.yaml` | 2 |
| `conformance/conventions/requests/read-parameter-headers/case.yaml` | 3 |
| `conformance/conventions/requests/read-unknown-result-type/case.yaml` | 2 |
| `conformance/conventions/requests/reason-order/case.yaml` | 2, 3, 4, 5, 7 |
| `conformance/conventions/requests/reason-reconsidered/case.yaml` | 1 |
| `conformance/conventions/requests/reconcile-false/case.yaml` | 2, 3, 6 |
| `conformance/conventions/requests/reconcile-invalid/case.yaml` | 1, 3, 4 |
| `conformance/conventions/requests/reconcile-revision-in-progress/case.yaml` | 2 |
| `conformance/conventions/requests/reconcile-revision-succeeded/case.yaml` | 3 |
| `conformance/conventions/requests/reconcile-true/case.yaml` | 2, 3, 6, 7 |
| `conformance/conventions/requests/records-inputs-invalid/case.yaml` | 1 |
| `conformance/conventions/requests/unknown-write/case.yaml` | 2, 3, 5 |
| `conformance/conventions/requests/unsafe-write-headers/case.yaml` | 2, 3 |
| `conformance/conventions/requests/write-capability-error/case.yaml` | 2 |
| `conformance/conventions/requests/write-input-required/case.yaml` | 2 |
| `conformance/conventions/requests/write-invalid-parameters/case.yaml` | 2 |
| `conformance/conventions/requests/write-is-error/case.yaml` | 2 |
| `conformance/conventions/requests/write-missing/case.yaml` | 2 |
| `conformance/conventions/requests/write-parameter-headers/case.yaml` | 2 |
| `conformance/conventions/requests/write-unknown-result-type/case.yaml` | 2 |

## Added outputs and unchanged inputs

setup actions and notices add pending outputs and unanswered metadata; existing projections
that do not deny those keys are not contradictions. The current registration-only Agent
fixtures have no signIn, so the URL capability and account checks do not apply to them.
The read-input-required and write-input-required cases contain no URL inputRequests: their
unknown outcome stays unknown, but IFC-050 changes the first explanatory text as listed above.
No existing case declares signIn or asserts the state directory variable, and none asserts a
sign-in, notice or pending-action answer. The root-system-state snapshot is input, not an
assertion of reachable first start; 040 updates its root-management list under C10.

MAN-018's new reserved variable and CTR-013 need new negative loader coverage in 048a. K1's
polling and missing-capability behavior need new vectors; this order adds none.
