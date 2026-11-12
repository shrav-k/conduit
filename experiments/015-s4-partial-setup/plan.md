# S4: order-level plan

> **In short**
> - S4-TASK1: propose task state and current-authority checks to the admin.
> - S4-TASK2: propose task transport and failure mapping to the admin.
> - S4-LATE: propose late ordinary/hook completion forwarding after timeout to the admin; S9 shares it.
> - S4-SET: propose setup/hook qualification and its vectors to the admin.
> - S4-REC1: propose keyed matrix/retention V3/V4 qualification.
> - S4-REC2: propose current-owner reconciliation V5 qualification.
> - Admin A: accept the split and hot-file sequence, or revise it; recommendation: accept after a fresh skeptic's review.
> - Admin B: place pending shared fixes against PIN; recommendation: preserve batch 1's pin, use a side branch if needed.
> - Admin C: provisional D4 reading recorded below.
> - Admin D: confirm E09’s loopback proxy URL through 050’s Agent package.
> - Owner: no new question; Q1–Q5 are answered.

## Status

Written on 2026-11-11 by work order 118, against `main` at `31acd73`.
A plan for the lane coordinators and the admin: it binds nothing, and changes nothing the owner
accepted in [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the
admin. No order named here has been dispatched.

Revised on 2026-11-11 by work order 121, against `1b717c408291b632d285a6bf0a782596478cc770`.

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

Tree audit: 042, 049a1, 050a, 070–072, 076–082 and 038c have order files, but their requested
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

Order 075 is merged (`55cf747`): an Agent Type states the reach of its client’s own tools.

## The orders

A scenario vector goes in the core or shell order that makes it pass if that order stays at most L;
otherwise an admin vectors order immediately follows it and depends only on the core and shell orders
its cases need, never on a lane package, owner step or run. A provider-contract case goes in lane 8’s
A63-VR, A63-VM, A63-VP or A63-VS suite in [the provider plan](../../docs/implementation/PROVIDERS.md).
Every package this plan places under `packages/` is a scenario server: its directory is named
`scenario-s4-...`, and its README says that it serves S4 and is not a provider for general use
(the admin's rule, 2026-11-11).

Sizes are before pipeline fixes: XS <45 min, S 1–1.5 h, M 2–2.5 h, L 2.5–3.5 h.
Line estimates include code, fixtures and tests; run time is estimated separately.
Every dispatch first needs a fresh skeptic's review of this split.

“Tools checks” means the root Python document/ruling/name/schema/roadmap checks, Python
`tools/tests` with PyYAML, and `git diff --check`. Every changed Rust crate also runs
`cargo test --release`, `cargo fmt --check` and clippy with warnings denied; engine orders
use the pinned engine toolchain. Vector expectations are authored from promises, never regenerated.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SCR1 | harness | Core scripted MCP Agent client | IFC-022/025, OPS-009/010/032 | — | M, 350–550 lines | lane 10 | now |
| SCR2 | harness | Task drivers, S2 deadline/stale extras and clock sync | IFC-016, OPS-024, ACC-034 | SCR1 | M, 400–600 lines | lane 10 | after SCR1 |
| S4-SRV1 | server | Plain partial/task and deduplicating effect fixture | IFC-010, OPS-012/018 | — | L, 650–850 lines | lane 8 | now |
| S4-SRV2 | server | Setup hook and independent ledger reader | TPL-012, OPS-028 | — | M, 350–500 lines | lane 8 | now |
| S4-TASK1 | core | Task identity, lifecycle and authority boundary | IFC-010/016, ACC-034 | PIN, 077, 048a-1, SV1 | L, 650–850 lines | admin | after dependencies |
| S4-TASK2 | shell | Task MCP relay and final-result mapping | IFC-010/016/025 | S4-TASK1, DSP | L, 650–850 lines | admin | after dependencies |
| S4-LATE | shell | Forward late ordinary/hook completions after timeout | OPS-013/014/018/021 | PIN, 077 | M, 400–600 lines | admin | after dependencies |
| S4-SET | core | Qualify partial setup and hook attribution | TPL-012/017, OPS-021/022/028 | PIN, 077, 081, 042, SV1 | M, 400–600 lines | admin | after dependencies |
| S4-REC1 | core | Keyed matrix and purge/expiry; V3/V4 | OPS-008/012/014/034, REC-004 | PIN, 077, 082, SV1 | M, 450–600 lines | admin | after dependencies, not TASK1 |
| S4-REC2 | core | Current-owner reconciliation routes; V5 | ACC-033/055 | S4-REC1, 042 | M, 350–550 lines | admin | after dependencies, not TASK1 |
| S4-H | harness | Six tasks, matrix and fault scheduling | README P2–P5, M0–M3 | SCR, S4-SRV1, S4-SRV2 | L, 600–850 lines including E09 proxy | lane 10 | after dependencies |
| S4-RUN | run | Coordinator and fresh-author scripted runs | README P1–P5 | S4-H, S4-TASK2, S4-LATE, S4-REC1/2, 047, 051b, SV4 | two runs, about 1 h each | lane 10 | after dependencies |
| S4-ACCOUNT | owner | Provide second subscription and sign in its Agent home | PND-005/010/011 | 050, 048a-1, 048a-2, 048b-relay, 048b-join, 051b | about 1 h reserved | owner | after dependencies and availability |
| S4-E09 | run | One lost-response real-client replay; A6 trace | OPS-008/012/018 | S4-RUN, S4-ACCOUNT | one run, about 1 h | lane 10 | after dependencies and owner |

**SCR1 / SCR2.** `experiments/015-s4-partial-setup/scripted-client/`, a standalone Rust crate
with lockfile, toolchain and ignored output, shared apparatus rather than a numbered experiment.
SCR1 initializes MCP, discovers tools, authenticates only as its supplied Agent identity, sends
keyed/fresh/unkeyed calls, holds acknowledged barriers and records redacted request/reply traces.
SCR2 adds task drivers and S2's exact keyed payload replay after a controlled filter deadline,
forced cached tool names/serving IDs, retained-output absence assertions and clock synchronization.
S2-H owns the clock/faults. Stub tests prove no hidden retries; no engine or conformance edits.
Run SCR as an Agent through the Codex package's `scripted-client` binary: SCR1 supplies its
compatible protocol mode and launch fixture, not a native-client emulator. When 050 changes
Agent packages/homes/sign-in, update the harness launch manifest/fixture to that binary's new
item and identity/home bindings; 050's owner supplies any required package wiring. No real login
is needed for scripted runs, and SCR never substitutes for a required real client. Checks G/P.

**S4-SRV1.** New package crate `packages/scenario-s4-effect/`: Cargo.toml, lock/toolchain,
conduit.yaml, contracts.yaml, README and tests, following the existing Agent package form.
Plain variants write ledger entry one then return -32021, directly or as a failed task, for both
an effect operation and startsModelWork without effect. Non-plain mode deduplicates across all attempts
of the scoped idempotency key (CTR-002, OPS-012); attempt numbers are trace evidence only, never part
of the deduplication key. A fixture test sends two attempts with that same key and proves one effect.
Barriers hold before application, after ledger commit/before reply, and before task terminal error;
independent readback and receipts distinguish lost success from unknown. No engine or generic repair API.

**S4-SRV2.** New package crate `packages/scenario-s4-hook/` with the same package files/checks.
S9-SRV reuses this hook implementation and its lost-reply control; build it once.
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
Use internal scripted task servers in engine tests, independent of S4-SRV1; its actual
package is qualified by S4-H. No separately named scenario vector; V2 belongs to S4-TASK1. Run engine release/fmt/clippy,
wire/barrier regressions and tools checks. S6 reuses these two orders, both owned here.

**S4-LATE.** Shell runner response routing, completion delivery and tests; hot shell runner.rs/lib.rs;
server.rs/identity.rs, CLI tables, core, conformance runner and manifest pieces unchanged.
In CONFORMANCE.md’s DL2/DL10 rows, state retention precisely: keep the reply slot after
timeout until its terminal reply or observed run/channel end (no elapsed-time eviction); drop it
on engine shutdown and let S6-END establish restart fences. Use internal scripted ordinary/hook
servers, not S4-SRV1/2 dependencies. Retain invocation/attempt correlation and forward late ordinary and setup/retire-hook
terminal replies through 077's core completion input. No automatic outcome change, retry or reconciliation.
Real timeout/barrier regressions for ordinary calls and hooks first observe unknown and failed-reconcile
conflict, then release the terminal reply: the fence is established, unknown and its record remain,
and only explicit reconciliation with independent no-effect evidence records failed. Nonterminal replies
fence nothing. No new scenario vector; checks: engine release/fmt/clippy, tools and wire/barrier tests.

**S4-SET.** Qualify existing setup/retirement parts rather than replacing them; core setup and
requests modules, hot requests.rs/lifecycle.rs; access.rs/registration.rs unchanged unless a
scoped current-authority regression requires it. Runner lib.rs and own manifest piece may change.
Engine-owned hook fixtures supply deterministic completions, with no lane server dependency.
Owns V1 `s4-partial-setup-retry` (safety LIF-027) and V6 `s4-hook-late-onward` (safety ACC-019).
Test no rollback, no unknown/successful hook redispatch and latest caller while active/refusal after end.
No new partial-effect state; checks are engine release/fmt/clippy, tools checks, vector_delta
and experiments/011 release tests.

**S4-REC1 / REC2.** Core requests/records/authority tests, hot requests.rs/access.rs,
conformance runner lib.rs and own manifest pieces; no registration.rs/lifecycle.rs/shell edits.
REC1 owns V3 `s4-key-matrix` (safety OPS-018 for keyed cells; unkeyed is new-work evidence)
and V4 `s4-purge-expiry` (safety OPS-008); REC2 owns V5 `s4-owner-state-reconcile`
(safety ACC-033). Assert twelve cells, expired outputs, retained keys/unknowns and bounded
original-sender limits, with E18 seed apart from legal removal. No task is involved, so neither
part waits TASK1 or SET. Internal scripted fixtures/completions, no server package dependency.
Each stays M including vectors; no detector/new reconcile verb. Engine release/fmt/clippy,
tools checks, vector delta and experiment 011 release tests.

**S4-H.** Rust harness under this experiment, using SCR and server barrier channels. Drop the
client-side reply only after durable success; separately drop server-to-engine completion to make
unknown. Drive T1–T6, all twelve cells, purge/expiry, E18's two labelled preparations and E20.
For real E09/A6, add a loopback proxy between the real Agent and engine that drops the
engine-to-client success response and records a redacted wire trace. SCR’s dropped reply alone
cannot cause a real client’s loss. Wait on admin D’s answer about the Agent’s engine URL for
this preparation/E09 only. No edits to engine, runner or manifest. Test scheduling and independent ledgers; release/fmt/clippy.

**S4-RUN / S4-ACCOUNT / S4-E09.** Pin engine/packages/client/protocol and qualify all vector and
wire tests before timing. Two separate author packs/runs preserve M0–M3, all faults/requests,
unknown explanations before ledger reveal, manual actions and access gaps. A failed rerun stays dated.
The owner signs in via Conduit, never via SCR or a copied login; record account by role and panel
work apart. E09 uses S4-H’s real Agent/engine proxy and confirmed 050 URL seam. It is exactly one replay, not twelve real-client matrix cells, with receipt, client
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

After split review, dispatch SCR1, S4-SRV1 and S4-SRV2 now; SCR2 follows SCR1, S4-H their
interfaces. Batch 1 still completes its plan/apply/replacement/sign-in joins through 053 → PIN.
Core joins: PIN/077/048a-1/SV1 → TASK1; PIN/077/081/042/SV1 → SET;
PIN/077/082/SV1 → REC1 → REC2 (042). REC1/2 need neither tasks nor SET.
Shell joins: PIN/077 → LATE; TASK1/DSP → TASK2. LATE/TASK2/REC1/2/H and 047/051b → RUN;
050's sign-in chain, ACCOUNT, real-client proxy/URL answer and owner availability → E09.
No batch-2 gate. All hot-file merges follow the single proposed sequence below; independent
package/harness preparation can proceed alongside it without concurrent requests.rs merges.

## For the admin

- **A — one proposed merge order for every plan:** accept this sequence or assign revised
  slots from actual hot files. **Recommendation:** use it as the common post-PIN sequence:
  - Core: S4-TASK1 → S4-SET → S5-C1 → S6-ACCESS → S4-REC1 → S4-REC2 → S5-C2 →
    S6-DUTY1 → S6-DUTY2 → S9-CORE1 → S9-CORE2 → S5-C3a → S5-C3b.
    Place C3a just after S9-CORE's registration.rs work (both edit it); it depends only on
    PIN/SV1, so this is merge sequencing, not a semantic prerequisite. Admin interleaves
    SEC-D1, FLT-D1, S3-L1, S3-L2, S3-RET1 and PSD1a/PSD1b by their listed hot files;
    FLT-D1 precedes S6-ACCESS's onward-close qualification. S4-SET precedes S9-CORE2's hook retry.
  - Shell: DSP → S4-LATE → S4-TASK2 → S5-WIRE → S9-CUT1 → S9-CUT2 → S6-WIRE.
    Admin interleaves SEC-D2, SEC-D3a, SEC-D3b, FLT-D2, PSD2, PSD3 and S3-RET2.
  - S6-END spans core and shell: after 077, before S6-WIRE. FLT-D0/D3 edit manifest pieces:
    D0 after 071/SV1/PIN; D3 after FLT-D1 and FLT-D2. Admin places A63-MDIS explicitly too.
  Each order uses engine-owned scripted fixtures, and carries its cases only while at most L;
  any overflow is an immediately following admin vectors order with only engine dependencies.
- **B — shared fixes and PIN:** preserve batch 1's pin; changes to starting vectors/root tools/
  S1 work on main wait PIN, unless the admin assigns a side branch. No order file counts as
  completed 077/081/082 work. Admin owns runner/shared-manifest integration.
- **C — D4:** Planned on the admin's reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9 only. Provisional until the owner confirms.
- **D — E09 URL, for admin as 050's owner:** can the Agent package point its engine URL at
  S4-H's loopback response-dropping proxy, or does 050 need an explicit lab-only endpoint
  override while preserving token audience/Host/proof checks? **Recommendation:** expose the
  smallest harness-only URL seam through 050, with a redacted proxy trace; confirm before
  preparing E09. No worker invents an authentication bypass or drops replies only in SCR.

## For the owner

None. Q4's boundary/path distinction and Q5's unknown partial effect remain as accepted;
ruling 77 answered M2-1 to M2-4;
no new repair feature is selected.

## Limits

This is a proposed dispatch plan, not execution evidence. No new language, vector, package,
client trace or safety verdict is supplied here. Local barrier cases do not prove arbitrary
external-effect detection or a remote partition, and SCR never qualifies real-client behavior.
