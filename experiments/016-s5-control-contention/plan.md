# S5: order-level plan

> **In short**
> - S5-C1: propose control admission, serialization and exact completion to the admin.
> - S5-C2: propose retained unknown-control qualification to the admin; reuse 041b's REV-010.
> - S5-C3a: propose E03/E04 admission with V1 to the admin.
> - S5-C3b: propose discovery/default/diagnostic V3/V5 to the admin.
> - S5-WIRE: propose public control dispatch and wire qualification to the admin.
> - Admin A: accept the four core parts after 041b; recommendation: keep each within L.
> - Admin B: choose vector placement; recommendation: keep dependent cases in their core orders after SV1.
> - Owner: no new question; packaged fixture and scripted runs are already accepted.

## Status

Written on 2026-11-11 by work order 118, against `main` at `31acd73`.
A plan for the lane coordinators and the admin: it binds nothing, and changes nothing the owner
accepted in [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the
admin. No order named here has been dispatched.

Revised on 2026-11-11 by work order 121, against `1b717c408291b632d285a6bf0a782596478cc770`.

## What `main` has now

The [conformance account](../../engine/CONFORMANCE.md) at lines 123–133 supersedes the README's
old claim that control reconciliation is absent. 041b (`1091a71`) implements it; public control
dispatch still fails the shell's supported-operation preflight. File presence is not merge evidence.

| README need or opening | Present evidence; missing work |
| --- | --- |
| OPS-020, REV-006/007 | `engine/crates/core/src/revisions.rs:18` applies held arguments for reconciliation, not ordinary control completion. `access.rs:654` already distinguishes control normalization. S5-C1 supplies full request-time configuration validation and exact completion, preserving omitted settings. |
| REV-009/010/013, original values | `revisions.rs:50` preflights current revision/schema, `:94` applies held original values and skips retired/removed targets. `conformance/conventions/fleets/e3b/reconcile-whole-config/case.yaml` and `reconcile-array-element/case.yaml` show these slices. S5-C2 qualifies reachable unknown controls, invalid merges and markers, without rebuilding REV-010. |
| REV-014/015 contention | `engine/crates/core/src/lifecycle.rs:139` detects busy setup/retirement; reconciliation checks it. Ordinary controls still need the same atomic busy/current-revision preflight and completion path. S5-C1 qualifies no queue, no-op and changing back as well as setup/retirement contention. |
| ACC-033/055 | Current owner routing is in `access.rs:278`; 042 supplies bounded original-sender reconciliation limits. S5-C2 covers live, retired-owner and removed-instance routes with wrong-route probes. |
| Tool narrowing/defaults | `engine/crates/core/src/schemas.rs:974` omits excluded controls and `:1048` constructs control tool schemas. Existing `conformance/interface/schemas/e05-control/case.yaml` and `schema-root-ref-default/case.yaml` are slices. S5-C3 qualifies E10/E16 against live dispatch and normalized keys. |
| E03/E04 static overlap | `engine/crates/core/src/registration.rs:212` checks input destinations, not proof of control-contract pointer-overlap rejection. No whole E03/E04 coverage is established. S5-C3a owns control-contract admission and its static cases together. |
| M1 diagnostic/status facts | Existing request envelopes and unknown status support some conflicts; 046a supplies reason diagnostics/runner assertions. S5-C3 qualifies all five promised distinctions, not a new generic frontend. |
| G3 safety field | Gap: still spec pending in IN_FORCE topic 13. `safety_rows` is not a promise tag. 071 → SV1 must land before these safety cases are written; SV4 governs each addition. |
| Other README openings | REV-010 already decides reconciliation; readback is contract-specific under Q4, not a missing generic verifier. DL2 late results are chosen in CONFORMANCE.md:19. No additional control-language gap is established. |

At this base, 070 still has whole-State copying to remove (`core/src/lib.rs:305`), 071's
manifest pieces are absent, 072's public list path is unfinished. 042, 047, 049a1, 050a,
076–082 and 038c likewise have order files but not their changes. In particular OPS-014 remains
unfenced at `spec/conventions/operations.yaml:286`; 077 must land before false reconciliation.
081/082 are also pending despite their commit subjects. Retire cascades/DL1 are already present;
declare-lost and DL7 https fetching remain unfinished, consistent with the admin's state.
None of the node/provider-fetch/sign-in work is needed for this scripted local control scenario.

[G5](../012-model-checking/G5.md) marks TPL-011/013 and other static choices for vectors;
it gives no existing-vector pass. Its MC2-001 needs 077, MC4-001/002/009 and MC6-001/002 need
081, MC7b-001/002 need 082. Unknown markers cannot stand in for physical readback, and storage
loss is not the same fault as an engine crash or a purge.

Order 075 is merged (`55cf747`): an Agent Type states the reach of its client’s own tools.

## The orders

A scenario vector goes in the core or shell order that makes it pass if that order stays at most L;
otherwise an admin vectors order immediately follows it and depends only on the core and shell orders
its cases need, never on a lane package, owner step or run. A provider-contract case goes in lane 8’s
A63-VR, A63-VM, A63-VP or A63-VS suite in [the provider plan](../../docs/implementation/PROVIDERS.md).
Every package this plan places under `packages/` is a scenario server: its directory is named
`scenario-s5-...`, and its README says that it serves S5 and is not a provider for general use
(the admin's rule, 2026-11-11).

Classes are before pipeline fixes: XS <45 min, S 1–1.5 h, M 2–2.5 h, L 2.5–3.5 h.
Line estimates include fixtures/tests. A fresh skeptic reviews this split before dispatch.

“Tools checks” means the root Python document/ruling/name/schema/roadmap checks, Python
`tools/tests` with PyYAML, and `git diff --check`. Every changed Rust crate also runs
`cargo test --release`, `cargo fmt --check` and clippy with warnings denied; engine orders
use the pinned engine toolchain. Vector expectations are authored from promises, never regenerated.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S5-SRV | server | Packaged two-setting control/readback fixture | OPS-020, REV-007/009 | — | M, 450–600 lines | lane 8 | now |
| S5-C1 | core | Control preflight, exclusion and exact completion | OPS-020, REV-001–007/012/014/015 | PIN, 042, 046a, SV1 | L, 650–850 lines | admin | after dependencies |
| S5-C2 | core | Retained originals, unknown markers and reconciliation | REV-009/010/013, ACC-033/055 | S5-C1, 077, 081 | M, 400–600 lines | admin | after dependencies; not 082 |
| S5-C3a | core | E03/E04 contract admission and V1 | CTR-006, TPL-011/013 | PIN, SV1 | M, 350–550 lines | admin | after PIN/SV1 only |
| S5-C3b | core | Publication/defaults/diagnostic facts; V3/V5 | TPL-010, SCH-007–011, IFC-025/033/050, REV-006 | S5-C2, 046a | M, 400–600 lines | admin | after dependencies |
| S5-WIRE | shell | Public control dispatch with internal scripted server | OPS-020, REV-006/007/009 | S5-C3a, S5-C3b, S4-LATE | M, 400–550 lines | admin | after dependencies |
| S5-H | harness | Two-actor barriers and measured task branches | README P2–P5, M0–M2 | SCR, S5-SRV | M, 350–500 lines | lane 10 | after dependencies |
| S5-RUN | run | Two separately counted author runs | README P1–P5 | S5-H, S5-WIRE, SV4 | two runs, about 1 h each | lane 10 | after dependencies |

**S5-SRV.** Propose standalone Rust `packages/scenario-s5-control/`, with Cargo.toml,
Cargo.lock, pinned toolchain, conduit.yaml, contracts.yaml, README and tests, patterned after
[agent-codex](../../packages/agent-codex/README.md). Two disjoint settings, a cross-setting
constraint, a declared receipt and independent readback. Barrier after receipt/before applying;
then exact success, independently proven no-effect failure or partial-effect/unknown branches.
Hold after physical commit/before reply; the harness separately loses the committed engine receipt.
No generic repair API or engine edits. Run package release/fmt/clippy and schema/install checks.

**S5-C1.** Core revisions module plus requests/lifecycle; hot `core/src/requests.rs`,
`lifecycle.rs`, `access.rs`, runner `conformance/src/lib.rs` and own manifest piece;
registration.rs and shell files unchanged. Validate changes against resulting configuration,
serialize against setup/retirement/control without waiting, and apply exactly supplied values only
on success. Changed success gets a new revision; no-op preserves it, changing back does not revive
an old revision. Tests use internal scripted control completions/servers, not S5-SRV. Owns V2 `s5-control-contention` (safety REV-012), including unknown/no-op branches.
Checks: engine release/fmt/clippy, tools checks, vector_delta and experiments/011 release tests.

**S5-C2.** Core revisions/request state and inspection projections; hot requests.rs/access.rs,
runner lib.rs and own manifest piece; lifecycle.rs/registration.rs unchanged unless required by a
scoped regression. Retain original normalized inputs separately from stored configuration; unknown
controls label affected settings unknown while retaining original configuration/revision. Internal
scripted control/readback fixtures qualify engine behavior independently of S5-SRV. Reuse
041b's current-revision merge, including stale/missing revision, occurred:false with/without revision,
schema-invalid, retired-owner and removed-instance branches. Owns V4 `s5-control-lost-reconcile`
(safety REV-010). No automatic verification/reconciliation. Same checks as S5-C1; depend on 077.

**S5-C3a / C3b.** C3a fixes E03/E04 equal/ancestor control pointers in both orders in
core/loader admission; hot registration.rs, runner lib.rs/own manifest piece, no requests.rs or
lifecycle.rs needed. Own V1 `s5-control-overlap` (safety CTR-006), with PIN/SV1 only.
C3b qualifies E10 max-50/max-100, discovery/direct-call exclusion, seven E16 inputs and root-$ref
exception; hot access.rs/requests.rs, runner lib.rs/own piece, registration.rs only if needed.
Own V3 `s5-control-discovery-defaults` (safety REV-006) and V5 `s5-conflict-explanation`
(safety IFC-025): busy/stale/changed-key/in-progress/output-expired with permitted facts.
Both use internal fixtures/completions, no lane package dependency; same checks as C1.

**S5-WIRE.** Shell supported-operation inventory and executor paths; hot `shell/src/server.rs`,
`runner.rs`, `lib.rs`; identity.rs and CLI command tables unchanged (existing call suffices).
Remove the control preflight refusal only when the whole path works; exercise narrowed publication,
retained arguments, physical readback and lost receipts with scripted control servers in
engine tests. S5-H qualifies the lane package; S5-WIRE never waits for S5-SRV. No speculative
control repair or retry. No separately named vector. A lost control reply reaches the core through
S4-LATE's late-completion delivery, shared with S4 and S9; S5-WIRE adds no second path. Its
regression uses a no-effect lost-reply fixture with a barrier: reconciliation conflicts before the
late terminal reply; once that reply is delivered and establishes the fence, the outcome stays
unknown; explicit `occurred: false` reconciliation then records failed without changing the stored
configuration. Run engine release/fmt/clippy and tools checks, plus actual MCP race/failure/receipt
tests, rebasing against 048b and DSP where hot files overlap.

**S5-H.** Rust harness under this experiment, driving SCR as two independently authenticated
Agents. Hold an admitted control at the server barrier, send a contender, release the first,
then make a stale call. Drive T1–T6, all seven E16 cases and T5 reset branches, with ledger and
revision snapshots; expire the receipt through configured retention. No engine/manifest/runner
changes. Test deterministic scheduling and dropped-reply location; release/fmt/clippy locally.

**S5-RUN.** Coordinator and fresh Codex author each author T1–T6 with scripted clients, all five
M1 conflict diagnoses and every T5/E16 branch. Pin versions and independently verify vector/wire
qualification before timing. Preserve M0–M2, submitted files, requests/replies, barrier events,
physical/stored values/unknown markers, changed lines, panel actions and time without access.
Record author explanations before comparing ledgers. No real-client replay or account step.
A failure stays a dated result; any rerun is separate, never an overwritten pass.

## Shared with other plans

- SCR is owned by S4's [plan](../015-s4-partial-setup/plan.md): MCP discovery/calls, stable keys,
  forced stale calls, barriers and request/reply evidence with each Agent's own identity.
  S5 requires no task relay, subscription, filter delivery or secret store.
- 070 → S0b and 071 supply core/manifest seams; 042 supplies current authority/tombstones,
  046a diagnostic facts/assertions and 046b closed-route reasons. Do not reimplement batch 1.
- 077 fences every attempt before an unknown control may be reconciled failed. 081/082 remain
  shared fixes, not extra S5 features: qualify retire/removal branches against their final text.
- SV1 follows 071 and supplies the field before writing vectors; SV2/SV3 cover existing/S1
  safety work, SV4 requires every later order to carry its own promise tags as listed above.
- A63-RP/MP/PS package local provider choices; built-in local runner/machine and file install
  already suffice. A63-H tests providers, not this server's control contract. PSD is unnecessary.
- B2-S, B2-C and B2-H are not prerequisites: no act-for/appointment/withhold or indirect-secret
  authority is assumed. 050/sign-in and AGT/ADDR/DSP/SSH-R/SSH-M are not local S5 requirements.

## The order of the work

After split review S5-SRV starts now; SCR1/2 in S4's plan join S5-H. Admin engine/vector work
waits batch 1 through 053 → PIN, including 052a-1/2 and 052b, absent an assigned side branch.
PIN/042/046a/SV1 → S5-C1 → 077/081/S5-C2 → S5-C3b; C3a needs only PIN/SV1.
C3a/C3b/S4-LATE → S5-WIRE → S5-RUN; S5-H joins the run. C2 does not wait 082.
Use [S4’s proposed hot-file merge order](../015-s4-partial-setup/plan.md#for-the-admin): C1 after S4-SET, C2 after
S4-REC1/2, C3a just after S9-CORE's registration.rs changes, C3b last in that core sequence;
WIRE follows S4-TASK2 in shell. That placement adds merge ordering, not C3a dependencies.
No batch-2/owner gate. Test against final applicable 081 text; retain storage-loss limits without
making unrelated 082 work a C2 dependency.

## For the admin

- **A — split:** accept C1/C2/C3a/C3b, or revise after the batch-1 seams land.
  **Recommendation:** these four bounded parts; C3a has only PIN/SV1 prerequisites and is
  merged by its registration.rs overlap against S9-CORE, as S4's canonical sequence proposes.
- **B — vectors:** V1 stays with C3a, V2 with C1, V4 with C2, V3/V5 with C3b.
  **Recommendation:** keep each fixing order within L; only vector overflow moves to an
  immediately following admin vectors order depending on needed engine work alone.

## For the owner

None. Q4 selected the two-setting local fixture with explicit readback, counted as provider work;
its order is kind server, not a new runtime provider. Q3 is scripted only.

Planned on the admin's reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9 only. Provisional until the owner confirms.

## Limits

This plan supplies no control result or new rule. Physical verification is specific to the
package contract. Deterministic contention is not a load benchmark or distributed lock proof;
scripted-client diagnostics do not prove what any model client displays.
