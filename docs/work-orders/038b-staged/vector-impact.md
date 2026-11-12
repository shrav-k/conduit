# Existing vector impact of enrollment

No vector changes in 038b. Checked against today's tree, beyond the area's original five
cases. Every row below asserts an exact root-management operations array at
`expect.state.objects[5].operations`, currently 13 entries. Landing 049a adds the three
root enrollment verbs, making 16. All ten expectations contradict the complete staged
root-management list (FST-002, ACC-032, ENR-001, ENR-006, ENR-008).

| Case path from repository root | Zero-based step |
| --- | --- |
| conformance/conventions/first-start/initial/case.yaml | 0 |
| conformance/conventions/first-start/repeat/case.yaml | 0 |
| conformance/conventions/first-start/repeat/case.yaml | 1 |
| conformance/conventions/first-start/lifecycle/case.yaml | 0 |
| conformance/language/first-start/protected/case.yaml | 0 |
| conformance/conventions/s1/whole/case.yaml | 0 |
| conformance/conventions/first-start/root-system/case.yaml | 0 |
| conformance/conventions/first-start/trust-domain/case.yaml | 0 |
| conformance/conventions/first-start/trust-domain/case.yaml | 1 |
| conformance/conventions/list/from-first-start/case.yaml | 0 |

Order 040 already supplied trustDomain in first-start evidence, expected system status and
the case schema. Those are no longer contradictory. The SPIFFE engine-id, token-subject and
trust-domain shell work is also present. The area's requested generated name on re-admission
is corrected: LIF-040 makes no node tombstone, so PLC-026 may reuse a free hostname name.

conformance/fixtures/root-system-state.json holds a snapshot, not a first-start claim.
It may be updated by 049a's coordinated pass, as C11 requests, but is no expectation or step
and is not counted above. Fixtures contain no node but local; the PLC-007 label cases use
local's first-start configuration, which ENR-010 preserves. A scan of the existing case
inputs and expectations found no machine-report, enrollment, fingerprint-item or place-output
expectation contradicted by the new provider shapes. The coverage descriptions in
conformance/manifest.yaml for PLC-026 and PRV-014 need updating with the new enrollment cases;
038b cannot change the manifest. No declaration is written for vector_delta.py.
