# S6: order-level plan

> **In short**
> - S6-ACCESS: propose in-flight/consumer-revocation qualification to the admin.
> - S6-DUTY: propose hook/stop/cascade/notice qualification to the admin.
> - S6-WIRE: propose end-to-end retirement/task/provider wire qualification to the admin.
> - Admin A: coordinate S4's task orders and DSP, or resplit; recommendation: reuse both once.
> - Admin B: keep cycles in the complete run and preserve PIN; recommendation: qualify existing DL1 after PIN.
> - Owner: no new question; Q4 includes unordered and cyclic cases.

## Status

Written on 2026-11-11 by work order 118, against `main` at `31acd73`.
A plan for the lane coordinators and the admin: it binds nothing, and changes nothing the owner
accepted in [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the
admin. No order named here has been dispatched.

## What `main` has now

041b is implemented (`1091a71`); its effects, rather than the earlier README prerequisite,
set the remaining work. See [CONFORMANCE.md](../../engine/CONFORMANCE.md).

| README need or opening | Present evidence; missing work |
| --- | --- |
| Pool retirement and LIF-021/DL1 | `engine/crates/core/src/cascade.rs:125` and CONFORMANCE.md:52 implement Pool/Fleet cascades. Independent members use stored order; cycle members ascending ID, with inter-cycle edges preserved. Member IDs are captured at dispatch and later registration blocks the container. T6's unordered/cyclic prerequisite is already met; S6-DUTY qualifies it, not invents an ordering. |
| Original response/current access, ACC-034 | `conformance/conventions/setup/close-in-flight/case.yaml` and CONFORMANCE.md:19 cover late results. An original response and a later task fetch are different. IFC-016's actual task relay remains absent (core completion boundary `requests.rs:1247`); S4-TASK1/2 and S6-ACCESS qualify this separation. |
| Onward checks and retired consumer, ACC-017–019/039 | Existing `engine/crates/core/src/access.rs:363` evaluates requests/serving; ordinary close and retire-hook slices exist. S6-ACCESS qualifies nested onward calls, denial after close and only the retirement-chain exception. No assumption that cached tools grant access. |
| Hook then stop, TPL-009/018, PRV-008 | `engine/crates/core/src/retirement.rs:252` selects hook then `:276` stop; CONFORMANCE.md:315 documents preparation parts and unknown run behavior. Built-in local execution exists; runtime provider execution is unserved (CONFORMANCE.md:40). DSP and S6-STOP/S6-WIRE qualify the uncertain-stop branch. |
| Failed hook/removal | `retirement.rs:289` settles parts under OPS-022; the retire-hook contract still disagrees. 081 is an unexecuted order here: MC6-001 fixes that conflict; MC6-002 narrows LIF-019 for Conduits removal closes. S6-DUTY and its provider/wire dependents wait for it. |
| Shared-Pool notices, PND-007 | Written promise, absent pending-action State (`core/src/lib.rs:63`). 047 builds the computed notice; S6-DUTY tests a non-root sender and another Fleet, with actual closed-Conduit IDs. Batch-2 act-for authority is not required. |
| Records/backing data and removal | `conformance/conventions/changes/retirement/case.yaml` covers retained records; no full independent data/neighbour check. S6-DUTY/H qualify custody only, never equate unknown with stopped or require rollback. |
| G3 safety field | Gap: IN_FORCE topic 13 still marks it spec pending. Manifest `safety_rows` does not name the required safety promise field. 071 → SV1 precedes writing these cases; SV4 applies. |
| DL1 opening / deadlines / late completions | DL1 is decided for this engine above; DL2 late results and DL10 deadlines stay implementation choices. No missing portable cancellation, rollback or shutdown-deadline rule is assumed. |

Tree status: order files 042, 049a1, 050a, 070–072, 075–082 and 038c are present but their
requested changes are absent: State still clones (`core/src/lib.rs:305`), pending/tombstone
maps and manifest pieces are missing, Agent Codex's manifest is 0.1.1, and OPS-014 is unfenced
(`spec/conventions/operations.yaml:286`). 081/082's commit subjects are not their work.
The actual tree agrees with the admin's implemented 041b, unfinished control/declare-lost and
file-only DL7 statements. It supplies no evidence of the unmerged side branches.

[G5](../012-model-checking/G5.md) assigns static safety decisions to vectors, not blanket model
coverage. MC2-001 needs 077; MC4-001/009 and MC6-001/002 need 081. MC4-002's old/new-instance
run overlap is an explicit limit of 081, not permission to retain new post-removal engine custody.
MC7b-001/002 need 082; records lost outside the engine must not be confused with purge/crash.
These are existing findings, not new rulings. No S6 core order may settle them differently.

## The orders

Sizes before pipeline fixes: XS <45 min, S 1–1.5 h, M 2–2.5 h, L 2.5–3.5 h.
Estimated lines include fixtures/tests. Dispatch waits for a fresh skeptic's review of the split.

“Tools checks” means the root Python document/ruling/name/schema/roadmap checks, Python
`tools/tests` with PyYAML, and `git diff --check`. Every changed Rust crate also runs
`cargo test --release`, `cargo fmt --check` and clippy with warnings denied; engine orders
use the pinned engine toolchain. Vector expectations are authored from promises, never regenerated.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S6-SRV | server | Ledger-backed onward/task/retire-hook fixture | ACC-034/039, TPL-009 | — | L, 600–800 lines | lane 8 | now |
| S6-STOP | provider | Controlled runtime stop/report fixture | PRV-007/008/009/021 | 081, A63-H | M, 400–600 lines | lane 8 | after dependencies |
| S6-ACCESS | core | Qualify close/retire/task/consumer boundaries | ACC-011/017–019/034/036/039, IFC-016 | PIN, 042, S4-TASK1, SV1 | M, 450–650 lines | admin | after dependencies |
| S6-DUTY | core | Qualify shutdown, removal, cascades and notice | TPL-009/018, PRV-008, LIF-016/019/020/021, PND-007 | S6-ACCESS, 081, 077, 047, 082 | L, 650–850 lines | admin | after dependencies |
| S6-WIRE | shell | Exercise actual task, retirement and runtime provider paths | IFC-016, ACC-034/039, PRV-008 | S6-DUTY, S4-TASK2, DSP, S6-SRV, S6-STOP | M, 400–600 lines | admin | after dependencies |
| S6-H | harness | Deterministic branch/cascade scheduling | README P2–P5, M0–M2 | SCR, S6-SRV | M, 400–550 lines | lane 10 | after dependencies |
| S6-RUN | run | Coordinator and fresh-author scripted runs | README P1–P5 | S6-H, S6-WIRE, SV4 | two runs, about 1 h each | lane 10 | after dependencies |

**S6-SRV.** Propose Rust `packages/scenario-s6-retirement/` with Cargo.toml, Cargo.lock,
pinned toolchain, conduit.yaml, contracts.yaml, README and tests, in the existing package form.
Provide independent durable effects/readback, ordinary and nested onward calls, a task result,
and retire-hook variants: success, proven no-effect failure and unknown. A barrier acknowledges
the original call in flight; another holds completion, and a third holds the retire hook.
Allow original completion before releasing shutdown; lose one completion in a reset branch.
No provider/engine policy here. Checks: package release/fmt/clippy, schema and install tests.

**S6-STOP.** Propose one Rust provider crate `packages/provider-s6-stop/`, the same package
files and provider-contract tests through A63-H. Start/report/stop one local fixture run per
instance ID; gates hold before stop confirmation and can report unknown without implying stopped.
Keep owed stops under PRV-021 while the instance exists; use 081's same-ID and failed-hook rules,
never promise old/new-ID exclusion after removal. No task or production remote runtime replacement.
Run release/fmt/clippy, provider vectors and package admission tests; DSP serves its engine calls.

**S6-ACCESS.** Core request/access/lifecycle tests with only scoped fixes; hot
`core/src/requests.rs`, `access.rs`, `lifecycle.rs`, runner `conformance/src/lib.rs` and own
manifest piece; registration.rs unchanged. Preserve actual original outcome/outputs; deny fresh,
repeat, nested onward, later task fetch and task notification after closure using current authority.
Owns V1 `s6-close-in-flight` (safety ACC-034), V2 `s6-retire-in-flight` (safety CON-008),
V3 `s6-retired-consumer-onward` (safety ACC-039). Keep retire-chain exception narrow.
Checks: engine release/fmt/clippy, tools checks, vector_delta and experiments/011 release tests.

**S6-DUTY.** Core retirement/cascade/pending tests, hot requests.rs/lifecycle.rs/access.rs,
runner lib.rs and own manifest piece; registration.rs unchanged. Qualify hook then stop whatever
hook outcome, stopped/unknown/no-setup-ever, explicit further retirement/reconciliation, removal,
retained records/data and unaffected neighbour. 081's failed hook plus successful stop is unknown.
Owns V4 `s6-retirement-duties` (safety LIF-016) and V5 `s6-cascade-order` (safety LIF-021),
including dependency failure blocking only successors, independent progress, unordered members,
cycles, container last and PND-007 notice. No new tie order or retention after removal.
Same checks as S6-ACCESS; false reconciliation waits for 077, notice for 047, removal for 081.

**S6-WIRE.** Shell tests and only scoped retirement transport fixes; hot
`shell/src/server.rs`, `runner.rs`, `lib.rs`; identity.rs and CLI command tables unchanged.
Use DSP's provider calls rather than a second dispatcher. Authenticate task operations through
S4-TASK2, deliver the original result while retire hook/stop are held, then test task-result and
notification refusal after close. Qualify failure/unknown stop and ACC-039 on actual MCP paths.
No additional named vector; all V1–V5 belong to their core orders. Checks: engine
release/fmt/clippy, tools checks and provider/task wire regressions with barrier acknowledgements.

**S6-H.** Rust driver under this experiment calls SCR for independently authenticated Agents;
root or an authorized non-root actor sends management requests through its own identity, never
through an Agent impersonating root. Build dependent pair, independent third member, two Fleets
and legal cycle; verify order by acknowledged events, not sleeps. Drive T1–T6, failures/unknowns,
repeat/fresh/onward/task branches, stop evidence, remaining data and notice IDs; reset only
between branches. No runner/manifest/engine edits. Test scheduler; local release/fmt/clippy.

**S6-RUN.** Qualify all vectors and real transports before two separately counted author runs.
Each author does every T1–T6 branch, including unordered and cyclic retirement. Keep the fresh
pack to spec/tasks/CLI format, pin versions, and retain M0–M2: commands, author changes, barriers,
dispatch/no-dispatch, outcome/reason/outputs, stopped evidence, external data, notice recipients,
manual recovery and time without access. Record missing evidence and dated failed reruns.
No native client replay, subscription, second machine or account sign-in is added.

## Shared with other plans

- SCR is owned once by S4's [plan](../015-s4-partial-setup/plan.md), including task requests,
  fixed keys, stale calls and per-identity evidence. S4-TASK1/2 (also owned by order 118 there)
  supply the single core/transport implementation used here; S6 adds revocation qualification.
- 070/S0b, 071, 072, 042, 046a/046b, 047 and 051a/051b are batch-1 seams, tombstone/lift,
  diagnostics, notices and author/panel commands. 047 alone builds PND-007, not S6-DUTY.
- 077 supplies OPS-014's fence; 081 supplies one-run/stop and failed-hook/removal corrections;
  082 supplies records/install corrections. Pending order files do not satisfy dependencies.
- SV1 after 071 supplies safety metadata before writing cases; SV2/SV3 tag old/S1 evidence,
  SV4 requires every new order to carry its safety promise. All five S6 vectors are safety.
- A63-H qualifies S6-STOP's provider contract; A63-RP can later supply the packaged ordinary
  local runtime but the built-in runner already handles ordinary local branches. A63-MP/PS are
  optional package choices; no secret store or PSD https fetch is required.
- DSP is shared provider dispatch, after its own prerequisites on the node side branch and PIN.
  ADDR/AGT/049a-1/SSH-R/SSH-M are not independently required by S6's one-machine run. Respect
  DSP's own dependencies without choosing answers to M2-1–M2-4. No remote machine is substituted.
- B2-S/C/H are not prerequisites for ordinary close/retire or the non-root notice fixture.
  Do not use act-for authority not yet written/built. 050 and sign-in are unnecessary here.

## The order of the work

After skeptic review, S6-SRV starts now, following SCR in the combined dispatch queue; S6-H
follows those two. S6-STOP waits for 081/A63-H. Scenario core/shell/vectors wait for 053 → PIN,
including batch 1's 052a-1/2 and 052b join, unless the admin chooses a side branch.
Critical path: max(PIN, 042, SV1, S4-TASK1) → S6-ACCESS →
max(S6-ACCESS, 081, 077, 047, 082) → S6-DUTY →
max(S6-DUTY, S4-TASK2, DSP, S6-STOP, S6-SRV) → S6-WIRE → S6-RUN.
S6-H joins the last run. The provider branch is 081 + A63-H → S6-STOP; no owner sign-in or
batch-2 prerequisite. Pending fixes/core hot files merge serially through the admin; servers
and harnesses can run beside them in two or three lanes. DL1 does not need another policy order.

## For the admin

- **A — shared transport:** options are reuse S4 tasks plus DSP, or duplicate a local subset.
  **Recommendation:** reuse once; review task and DSP merge dates before dispatch and sequence
  S6-WIRE after their hot server.rs/runner.rs/lib.rs changes. If provider qualification pushes
  S6-WIRE above L, split before dispatch, not by omitting uncertain-stop evidence.
- **B — placement/completion:** options are full S6 after PIN, or preliminary partial evidence
  on an isolated branch. **Recommendation:** full qualification after PIN, including existing
  DL1 cycles and 081's settled text. No partial run counts as the full gate. Admin owns runner
  and shared manifest integration even if a lane authors cases. Preserve starting vectors and
  root tools until PIN. D4 needs no additional replay in S6; sister account questions stay open.

## For the owner

None. Q4 already includes unordered and cyclic retirement once DL1 is documented, which it is.
No new shutdown deadline, cancellation guarantee or M2-1–M2-4 answer is proposed.

## Limits

This writes no result or language. Access closure is not cancellation or proof that a remote
effect stopped. Provider fixtures qualify their stated contract, not every runtime; model
checking's finite findings and old/new-ID overlap limit remain visible.
