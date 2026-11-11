# Vector impact of the 036b contracts

Step numbers below are zero based, as in the case arrays. This is a source audit, not a
prediction from an updated engine. No expectation was rewritten. The active core changes
passed `all_portable_cases_pass` on the existing engine; the staged changes are intentionally
not active yet. The behavior promises were already written by 036a.

## Required Agent owner: 041a

The following steps hand over at least one Agent instance with no owner. Their input must be
amended when 041a lands the required field; an accepted result cannot hold for the unamended
input. For currently rejected inputs, this is a diagnostic-order review, not a claimed
acceptance flip. Later steps in the same case must be rechecked after the fixture is amended.
The source scan follows each step's fixture/project file mappings and indexed request fixtures, rather than inferring an
outcome from the engine.

| Case | Step | Agents without owner | Current accepted assertion |
| --- | --- | --- | --- |
| `conformance/conventions/access/bounds/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/access/current-tools/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/access/identity/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/access/plain-registration/case.yaml` | 0 | `agent-a, agent-b` | false |
| `conformance/conventions/access/routing/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/access/s1-exact-access/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/access/s1-plain-filter/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/access/s1-pool-authority/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/access/tool-collision/case.yaml` | 0 | `agent-a, agent-b` | false |
| `conformance/conventions/changes/retirement/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/changes/t4/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/changes/t4/case.yaml` | 2 | `agent-c` | iserror: false |
| `conformance/conventions/changes/t5/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/changes/t5/case.yaml` | 5 | `new-consumer` | iserror: false |
| `conformance/conventions/requests/aware-capability-extension/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/aware-capability-undeclared/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/aware-failed/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/aware-headers-no-repair/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/aware-malformed/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/aware-unknown/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/cancel-confirmed/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/cancellation/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/declared-outputs/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/defaults/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/effectful-plain-headers/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/failed-read-repeat/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/identity-scope/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/invalid-outputs/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/key-scope/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-failed-reason-array/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-failed-reason-null/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-failed-reason-number/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-failed-reason-object/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-outcome-null/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-outcome-number/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-succeeded-reason-array/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-succeeded-reason-null/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-succeeded-reason-number/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/metadata-succeeded-reason-object/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/plain-content/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/plain-model-work/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/plain-read-text-only/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/read-capability-error/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/read-input-required/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/read-invalid-parameters/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/read-is-error/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/read-missing/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/read-parameter-headers/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/read-success/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/read-unknown-result-type/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/reason-order/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/reason-reconsidered/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/reconcile-false/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/reconcile-invalid/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/reconcile-revision-in-progress/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/reconcile-revision-succeeded/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/reconcile-true/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/records-disabled/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/records-inputs/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/records-inputs-invalid/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/records-key/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/repair-read-success/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/restart-interrupted/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/unknown-write/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/unsafe-write-headers/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/write-capability-error/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/write-input-required/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/write-invalid-parameters/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/write-is-error/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/write-missing/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/write-parameter-headers/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/write-success/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/requests/write-unknown-result-type/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/s1/whole/case.yaml` | 1 | `agent-a, agent-b` | true |
| `conformance/conventions/s1/whole/case.yaml` | 2 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/base/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/cancel/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/close/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/close-in-flight/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/hook-failed/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/hook-succeeded/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/hook-unknown/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/optional/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/pool-entry/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/restore/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/restore-current-template/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/restore-missing-template/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/restore-retired-target/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-conditions-builtin/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-conditions-provider/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-dependencies-failed/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-dependencies-succeeded/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-hook-refused/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-hook-refused-no-effect/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-hook-unavailable/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-hook-unavailable-no-effect/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-pool-cancel/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-provider-records/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-relative-provider/case.yaml` | 0 | `a` | true |
| `conformance/conventions/setup/review-retire-before-run/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-retire-connection-succeeded/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-retire-connection-unknown/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-retire-run-succeeded/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/review-retire-run-unknown/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/run-connections/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/run-failed/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/run-partial/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/run-unknown/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/run-unknown-run/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/sender/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/sender-no-access/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/sender-onward/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/conventions/setup/target-registered/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/language/archives/registered-project-holder/case.yaml` | 0 | `a` | true |
| `conformance/language/archives/s1/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/language/archives/s1/case.yaml` | 4 | `agent-a, agent-b` | true |
| `conformance/language/packages/s1/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/language/packages/s1-digest-mismatch/case.yaml` | 0 | `agent-a, agent-b` | false |
| `conformance/language/packages/s1-missing/case.yaml` | 0 | `agent-a, agent-b` | false |
| `conformance/language/registration/argument-map-default/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/argument-overrides/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/arguments-default/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/arguments-invalid/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/arguments-null/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/arguments-optional/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/arguments-relative/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/arguments-required/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/arguments-unknown/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/config-invalid/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/config-standard-defaults/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/entry-active-replacement/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/entry-retired-only/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/frozen-template/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/frozen-template/case.yaml` | 1 | `b` | true |
| `conformance/language/registration/live-agents/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/language/registration/nested-reference-input/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/nested-reference-missing/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/reference-authored-id/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/reference-chains/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/reference-config/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/reference-missing/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/reference-null/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/reference-patterns/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/reference-template/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/retained-package-build-metadata/case.yaml` | 1 | `a` | true |
| `conformance/language/registration/retained-package-missing/case.yaml` | 1 | `a` | false |
| `conformance/language/registration/runtime-explicit/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/runtime-non-provider/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/runtime-retired/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/runtime-wrong-family/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/s1/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/language/registration/s1-atomic-rejection/case.yaml` | 0 | `agent-a, agent-b` | false |
| `conformance/language/registration/s1-orders/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/language/registration/s1-rejected-orders/case.yaml` | 0 | `agent-a, agent-b` | false |
| `conformance/language/registration/s1-reverse/case.yaml` | 0 | `agent-a, agent-b` | true |
| `conformance/language/registration/standard-argument-consumer/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/standard-argument-owner/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/standard-argument-placement/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/standard-argument-runtime/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/standard-argument-target/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/template-family/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/template-kind-name/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/template-missing/case.yaml` | 0 | `a` | false |
| `conformance/language/registration/template-project-local/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/type-default/case.yaml` | 0 | `a` | true |
| `conformance/language/registration/type-no-schema/case.yaml` | 0 | `a` | true |

## First start: 040

The root template and operation lists contradict the old first-start lists and counts in:

| Case | Steps |
| --- | --- |
| `conformance/conventions/first-start/initial/case.yaml` | 0 |
| `conformance/conventions/first-start/repeat/case.yaml` | 0, 1 |
| `conformance/conventions/first-start/lifecycle/case.yaml` | 0 |
| `conformance/language/first-start/protected/case.yaml` | 0; 2, 3, 4 change refused to invalid under FST-006 once root has system authority |
| `conformance/conventions/s1/whole/case.yaml` | 0 |

C10 assigns the root operation-list edits in `conformance/fixtures/root-system-state.json`
to 040. The snapshot is input, not an asserted reachable first start. C8 lists here omit
answer and sign-in as ordered; 038a adds them before 040 lands these patches. 049a later
adds enrollment verbs.

## Existing 036a promise obligations

The area plans and C8/C9 already assign these effects to later build orders. They are not
additional expectation changes introduced by an optional 036b input or output:

| Case | Steps or fixture indices | Existing obligation |
| --- | --- | --- |
| `conformance/conventions/access/routing/case.yaml` | 7 (access fixture index 6) | An Agent setup through management must use its Fleet authority |
| `conformance/conventions/setup/restore/case.yaml` | 14 | lift before register-replacement |
| `conformance/conventions/setup/restore-current-template/case.yaml` | 12 | lift before register-replacement |
| `conformance/conventions/changes/t4/case.yaml` | 10, 15 | lift before register-replacement |
| `conformance/conventions/s1/whole/case.yaml` | 50 | lift before register-replacement |
| `conformance/interface/schemas/e05-local/case.yaml` | 0 | IFC-033 target and Type description |

C25 leaves the schemas supporting those input changes staged. Optional expectedId on open
and close supplies no changed expectation in an existing case. Inventory and provenance are
added outputs; existing assertions project the fields they name. The cold engine run is
not evidence that the staged schema or promise obligations above are already implemented.
