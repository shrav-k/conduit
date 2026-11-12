# S4: order-level plan

> **In short**
> - S4-TASK1: propose task state and current-authority checks to the admin.
> - S4-TASK2: propose task transport and failure mapping to the admin.
> - S4-LATE: propose late ordinary/hook completion forwarding after timeout to the admin; S9 shares it.
> - S4-SET: propose setup/hook qualification and its vectors to the admin.
> - S4-REC: propose keyed recovery/retention qualification and its vectors to the admin.
> - Admin A: accept the split and hot-file sequence, or revise it; recommendation: accept after a fresh skeptic's review.
> - Admin B: place pending shared fixes against PIN; recommendation: preserve batch 1's pin, use a side branch if needed.
> - Admin C: confirm the D4 run boundary with sister plans; recommendation: retain exactly S4's one E09 replay here.
> - Owner: no new question; Q1–Q5 are answered.

## Status

Written on 2026-11-11 by work order 118, against `main` at `31acd73`.
A plan for the lane coordinators and the admin: it binds nothing, and changes nothing the owner
accepted in [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the
admin. No order named here has been dispatched.

## What `main` has now

Evidence is from this commit, not from an order's presence or its commit subject.
The [engine conformance account](../../engine/CONFORMANCE.md) is the implementation baseline.

| README need or opening | Present evidence; missing work |
| --- | --- |
| Setup parts/hooks, TPL-012/017, OPS-021/022/028 | `engine/crates/core/src/requests.rs:1524` and `conformance/conventions/setup/hook-unknown/case.yaml` already retain hooks; `setup/restore/case.yaml` exercises restoration. S4-SET qualifies dependencies, eligible unfinished parts and latest-caller attribution. No new setup engine is planned. |
| Task failures, IFC-010/016 | `requests.rs:1247` accepts final completions; `engine/README.md:52` lists no task input. No qualified task relay exists. S4-TASK1/2 supply it; 048a-1's sign-in detection alone is insufficient. |
| Attempts/deduplication, OPS-012 | `requests.rs:820` and `conformance/conventions/requests/unknown-write/case.yaml` cover unknown repeats. S4-REC adds the complete twelve-cell matrix, with independent server deduplication. |
| Retention/reconciliation/purge, OPS-008/014/034, REC-004 | CONFORMANCE.md DL2/DL8/DL10 and `conformance/conventions/records/purge-succeeded/case.yaml` already retain invocation truth separately from purgable records; expiry exists. Qualification remains, and failed reconciliation needs 077. |
| Executable fence after timeout, OPS-013/014 | `engine/crates/shell/src/runner.rs:1031–1047` removes the response slot on timeout; `docs/work-orders/077-spec-reconcile-fenced.md:377–380` explicitly excludes shell work. S4-LATE must forward late ordinary/hook terminal replies to 077's core completion input; 077 alone cannot make this path executable. |
| Current authority, ACC-033/055 | 041b (`1091a71`) is implemented: `engine/crates/core/src/access.rs:278` routes by current owner. 042 still must implement bounded reconciliation limits and tombstones/lift; S4-REC tests wrong routes before/after removal. |
| G1: partial effect | Gap: OPS-014 supplies no partial-effect reconciliation representation. Q5 leaves genuine partial work unknown. A future spec order would need an owner-approved contract/behavior if this scope expanded; none is proposed or required for these tasks. |
| G2: retired owner with extant Resource | ACC-033 specifies the boundary, not a demonstrated public preparation. Q4 accepts a labelled snapshot plus the reachable retirement/removal path; S4-REC must show both apart, never claim the seed is reachable. |
| G3: safety field | Gap: IN_FORCE topic 13 still says spec pending; the manifest's `safety_rows` are not the required promise field. SV1 after 071 supplies the agreed format before any scenario safety case is written. |

Tree audit: 042, 049a1, 050a, 070–072, 075–082 and 038c have order files, but their requested
changes are absent here. `State` at `engine/crates/core/src/lib.rs:63` has no tombstone/pending
maps and `:305` still clones state; the manifest is monolithic. Agent Codex is still 0.1.1 in
[its manifest](../../packages/agent-codex/conduit.yaml). OPS-014 at
`spec/conventions/operations.yaml:286` remains unfenced; 081's failed-hook and run obligations
and 082's records/install changes are also unwritten. The supplied state is consistent with
those being pending; commit subjects for 081/082 alone would suggest the wrong conclusion.
Declare-lost/control dispatch and DL7 https fetching remain unfinished, as the admin stated.
No S4 task requires declare-lost, remote nodes or https package fetching.

[G5](../012-model-checking/G5.md) assigns static decisions to vectors, not passing evidence.
MC2-001 needs 077; MC4-001/002/009 and MC6-001/002 need 081; MC7b-001/002 need 082.
These are not fresh rulings by this plan. In particular, a stopped implementation does not prove
that an independent downstream service has no request in flight; preserve 077's stated limit.

## The orders

Sizes are before pipeline fixes: XS <45 min, S 1–1.5 h, M 2–2.5 h, L 2.5–3.5 h.
Line estimates include code, fixtures and tests; run time is estimated separately.
Every dispatch first needs a fresh skeptic's review of this split.

“Tools checks” means the root Python document/ruling/name/schema/roadmap checks, Python
`tools/tests` with PyYAML, and `git diff --check`. Every changed Rust crate also runs
`cargo test --release`, `cargo fmt --check` and clippy with warnings denied; engine orders
use the pinned engine toolchain. Vector expectations are authored from promises, never regenerated.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SCR | harness | Shared scripted MCP Agent client | IFC-016/022/025, OPS-009/010/032 | — | L, 700–900 lines | lane 10 | now |
| S4-SRV1 | server | Plain partial/task and deduplicating effect fixture | IFC-010, OPS-012/018 | — | L, 650–850 lines | lane 8 | now |
| S4-SRV2 | server | Setup hook and independent ledger reader | TPL-012, OPS-028 | — | M, 350–500 lines | lane 8 | now |
| S4-TASK1 | core | Task identity, lifecycle and authority boundary | IFC-010/016, ACC-034 | PIN, 077, 048a-1, SV1 | L, 650–850 lines | admin | after dependencies |
| S4-TASK2 | shell | Task MCP relay and final-result mapping | IFC-010/016/025 | S4-TASK1, S4-SRV1 | L, 650–850 lines | admin | after dependencies |
| S4-LATE | shell | Forward late ordinary/hook completions after timeout | OPS-013/014/018/021 | PIN, 077, S4-SRV1, S4-SRV2 | M, 400–600 lines | admin | after dependencies |
| S4-SET | core | Qualify partial setup and hook attribution | TPL-012/017, OPS-021/022/028 | PIN, 077, 081, 042, SV1 | M, 400–600 lines | admin | after dependencies |
| S4-REC | core | Qualify matrix, purge/expiry and owner routes | OPS-008/012/014/034, REC-004, ACC-033/055 | S4-SET, S4-TASK1, 082 | L, 650–850 lines | admin | after dependencies |
| S4-H | harness | Six tasks, matrix and fault scheduling | README P2–P5, M0–M3 | SCR, S4-SRV1, S4-SRV2 | M, 400–550 lines | lane 10 | after dependencies |
| S4-RUN | run | Coordinator and fresh-author scripted runs | README P1–P5 | S4-H, S4-TASK2, S4-LATE, S4-REC, 047, 051b, SV4 | two runs, about 1 h each | lane 10 | after dependencies |
| S4-ACCOUNT | owner | Provide second subscription and sign in its Agent home | PND-005/010/011 | 050, 048a-1, 048a-2, 048b-relay, 048b-join, 051b | about 1 h reserved | owner | after dependencies and availability |
| S4-E09 | run | One lost-response real-client replay; A6 trace | OPS-008/012/018 | S4-RUN, S4-ACCOUNT | one run, about 1 h | lane 10 | after dependencies and owner |

**SCR.** Propose `experiments/015-s4-partial-setup/scripted-client/`, a standalone Rust crate
with Cargo.lock, pinned toolchain and ignored output. It is shared scenario apparatus under S4,
not a new numbered experiment or a production Agent package. Reuse the wire approach in
[the package fixture](../../packages/agent-codex/tests/fixtures/client.rs), not its native-client emulation.
Initialize MCP, discover tools, authenticate from only the Agent's own supplied identity, issue keyed
repeats/fresh/unkeyed calls, hold/release named barriers, and record each request/reply with credentials
excluded. Support tasks, forced stale tool calls and explicit before/after-deadline E02 replay;
S2's plan owns its extra requirements. Test with a local stub, including no hidden retry; no engine edits.

**S4-SRV1.** New package crate `packages/scenario-s4-effect/`: Cargo.toml, lock/toolchain,
conduit.yaml, contracts.yaml, README and tests, following the existing Agent package form.
Plain variants write ledger entry one then return -32021, directly or as a failed task, for both
an effect operation and startsModelWork without effect. Non-plain mode deduplicates across all attempts
of the scoped idempotency key (CTR-002, OPS-012); attempt numbers are trace evidence only, never part
of the deduplication key. A fixture test sends two attempts with that same key and proves one effect.
Barriers hold before application, after ledger commit/before reply, and before task terminal error;
independent readback and receipts distinguish lost success from unknown. No engine or generic repair API.

**S4-SRV2.** New package crate `packages/scenario-s4-hook/` with the same package files/checks.
A no-effect hook can end before its reply is lost, expose independent evidence that work ended,
and delay A's onward call until B's attempt. For E20, hold the ended call's terminal
response until the engine has marked it unknown, then release it through S4-LATE: 077's core records
the fence without settling unknown, before explicit no-effect reconciliation. No partial ledger
is called a whole-effect failure.
Use explicit barrier acknowledgements, never sleep races. Both server orders run package release tests,
fmt/clippy, package installation checks and repository document/schema checks; no model or subscription.

**S4-TASK1.** Core task module and boundary inputs; hot files `core/src/requests.rs`, `access.rs`,
`lifecycle.rs`, runner `conformance/src/lib.rs` and S4's manifest piece; registration.rs unchanged.
Keep one invocation through a task, authenticate every task request/subscription, suppress disallowed
notifications, map failed task errors like direct errors, preserve ended outcomes. No sign-in redesign.
Owns V2 `s4-plain-partial-error-task` (safety OPS-018), including both effect/model-work variants.
Checks: engine release tests, fmt/clippy, tools checks, vector_delta and experiments/011
release tests; expand a seam only as needed.

**S4-TASK2.** Shell task transport and tests, synchronized with 048b's MCP work; hot files
`shell/src/server.rs`, `runner.rs`, `lib.rs`; identity.rs and CLI command tables unchanged.
Negotiate tasks only when both sides declare them; relay get/update/cancel/listen with current
checks, deliver terminal failures, and drop only the selected reply. No autonomous retry or model work.
No separately named scenario vector; V2 belongs to S4-TASK1. Run engine release/fmt/clippy,
wire/barrier regressions and tools checks. S6 reuses these two orders, both owned here.

**S4-LATE.** Shell runner response routing, completion delivery and tests; hot shell runner.rs/lib.rs;
server.rs/identity.rs, CLI tables, core, conformance runner and manifest pieces unchanged.
Retain invocation/attempt correlation after timeout and forward late ordinary and setup/retire-hook
terminal replies through 077's core completion input. No automatic outcome change, retry or reconciliation.
Real timeout/barrier regressions for ordinary calls and hooks first observe unknown and failed-reconcile
conflict, then release the terminal reply: the fence is established, unknown and its record remain,
and only explicit reconciliation with independent no-effect evidence records failed. Nonterminal replies
fence nothing. No new scenario vector; checks: engine release/fmt/clippy, tools and wire/barrier tests.

**S4-SET.** Qualify existing setup/retirement parts rather than replacing them; core setup and
requests modules, hot requests.rs/lifecycle.rs; access.rs/registration.rs unchanged unless a
scoped current-authority regression requires it. Runner lib.rs and own manifest piece may change.
Owns V1 `s4-partial-setup-retry` (safety LIF-027) and V6 `s4-hook-late-onward` (safety ACC-019).
Test no rollback, no unknown/successful hook redispatch and latest caller while active/refusal after end.
No new partial-effect state; checks are engine release/fmt/clippy, tools checks, vector_delta
and experiments/011 release tests.

**S4-REC.** Core requests/records and authority tests; hot requests.rs/access.rs, runner lib.rs
and own manifest piece; registration.rs/lifecycle.rs and shell hot files unchanged.
Owns V3 `s4-key-matrix` (safety OPS-018 for keyed cells; unkeyed cells are new-work evidence),
V4 `s4-purge-expiry` (safety OPS-008), V5 `s4-owner-state-reconcile` (safety ACC-033).
Assert twelve cells, expired outputs, retained keys/unknowns and ACC-055 original-sender limits;
separate E18 seed from legal public removal. No storage-loss detector or new reconciliation verb.
Checks: engine release/fmt/clippy, tools checks, vector_delta and experiments/011 release tests,
including static authority branches.

**S4-H.** Rust harness under this experiment, using SCR and server barrier channels. Drop the
client-side reply only after durable success; separately drop server-to-engine completion to make
unknown. Drive T1–T6, all twelve cells, purge/expiry, E18's two labelled preparations and E20.
No edits to engine, runner or manifest. Test scheduling and independent ledgers; release/fmt/clippy.

**S4-RUN / S4-ACCOUNT / S4-E09.** Pin engine/packages/client/protocol and qualify all vector and
wire tests before timing. Two separate author packs/runs preserve M0–M3, all faults/requests,
unknown explanations before ledger reveal, manual actions and access gaps. A failed rerun stays dated.
The owner signs in via Conduit, never via SCR or a copied login; record account by role and panel
work apart. E09 is exactly one replay, not twelve real-client matrix cells, with receipt, client
exposure and next action recorded separately for A6. Never record credential contents.

## Shared with other plans

- 070 then S0b, and 071/072: batch 1's seams and manifest pieces; these are not in the base.
  042 supplies tombstones/lift/ACC-055; 046a diagnostics, 046b closed-Conduit diagnostics;
  047 setup evidence; 051a/051b inspection and panel commands. Do not duplicate these in S4.
- 077 supplies fenced reconciliation in spec **and core**; every OPS-014 outcome here waits.
  S4-LATE supplies the missing shell delivery once, shared with S9; run-end/restart fences are outside it.
  081 supplies one-run/stop and failed-hook/removal corrections; 082 supplies records/install fixes.
  Neither file's existence is completion. Their dispatch/merge placement belongs to the admin.
- SV1 after 071 defines the safety field; SV2 tags old cases, SV3 covers S1's missing cases,
  SV4 requires each later order to tag its own. Our vectors carry their listed promises after SV1.
- 050 (including 050a/076), 075, 038c and 048a-1/2, 048b-relay/join, 051b supply the real
  Agent home, account hooks and sign-in. The [accounts guide](../../docs/guides/accounts.md)
  predates ruling 75's browser choice: use that ruling, not its old device-only instructions.
- A63-RP/MP/PS can package the local implementations but are not prerequisites to using the
  existing built-in local runner, local machine and file archive install in these runs.
  A63-H is provider qualification, not SCR. No SEC-D, FLT-D, PSD or remote-node dependency.

## The order of the work

Today dispatch, after skeptic review: SCR, S4-SRV1, S4-SRV2, in that order as slots free;
then S4-H can be prepared independently of the engine lane. No scenario engine order starts now.
Batch 1 still runs through 053 → PIN, including 070 → S0b, 042 → 046a → 047 → 048a-2 →
048b-join → 050, with 048a-1 and 048b-relay joining. Its other CLI/plan-and-apply work also
joins 053; this plan does not bypass 052a-1/2 or 052b.
Critical path: max(PIN, 077, 081, 042, 071 → SV1) → S4-SET;
max(PIN, 077, 048a-1, SV1) → S4-TASK1 → S4-TASK2;
max(PIN, 077, S4-SRV1, S4-SRV2) → S4-LATE;
max(S4-SET, S4-TASK1, 082) → S4-REC → S4-RUN → S4-E09.
S4-LATE, S4-TASK2, S4-H and 047/051b join before S4-RUN; sign-in/050 and owner availability join before E09.
No batch-2 authority is used: ordinary current routes suffice. B2-S/C/H wait for the owner's
rerun judgment and are not dependencies here. Run engine lanes two or three at a time only
with disjoint hot files; merge requests.rs changes serially and rebase against PIN.

## For the admin

- **A — split/merge sequence:** keep task state and transport separate, or combine/redivide.
  **Recommendation:** the L-sized split above; assign S4's task work once for S6, and review
  transport scope against actual MCP support before dispatch. If >L, split before dispatch.
  Keep late ordinary/hook forwarding in S4-LATE or fold it into task transport; **recommendation:**
  keep its M-sized scope separate so S9 need not wait for tasks. Serialize runner.rs/lib.rs with
  S4-TASK2 and 048b/DSP; require the actual timeout regressions before S4 or S9's repair runs.
- **B — shared fixes and PIN:** merge in batch 1 only with admin-approved rerun impact, or use
  a side branch and merge after PIN. **Recommendation:** preserve PIN; do not change starting
  vectors/root tools/S1 author's work on main beforehand. Serialize own manifest pieces/runner
  changes through the admin even when all cases pass already. 077/081/082 need real completion.
- **C — D4 across scenarios:** literal “only” could conflict with 72 point 2 and 75 D2's
  S3/S7/S8/S10 accounts. **Recommendation:** keep those sister plans' interpretation pending
  admin clarification; this plan requires only S4's explicitly accepted single replay.

## For the owner

None. Q4's boundary/path distinction and Q5's unknown partial effect remain as accepted;
no answer to M2-1–M2-4 or a new repair feature is selected.

## Limits

This is a proposed dispatch plan, not execution evidence. No new language, vector, package,
client trace or safety verdict is supplied here. Local barrier cases do not prove arbitrary
external-effect detection or a remote partition, and SCR never qualifies real-client behavior.
