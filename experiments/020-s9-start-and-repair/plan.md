# S9: order-level plan

> **In short**
> - S9-CORE1: propose foundation/start V1/V2/V5 and baseline V3 to the admin.
> - S9-CORE2: propose setup repair V4 to the admin.
> - S9-CUT1: propose initialization cuts and the pinned lab binary.
> - S9-CUT2: propose repair/protected-refusal lab qualification.
> - S4-LATE (shared): propose late ordinary/hook completion forwarding after timeout to the admin.
> - Admin A: accept core/shell split and test-only cuts, or resplit; recommendation: accept after skeptic review.
> - Admin B: place vector/runner changes after PIN and SV1; recommendation: preserve the pin and DL6 scope.
> - Owner: no new question; Q4 restricts repair to documented cuts and reachable authored setup.

## Status

Written on 2026-11-11 by work order 118, against `main` at `31acd73`.
A plan for the lane coordinators and the admin: it binds nothing, and changes nothing the owner
accepted in [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the
admin. No order named here has been dispatched.

Revised on 2026-11-11 by work order 121, against `1b717c408291b632d285a6bf0a782596478cc770`.

## What `main` has now

The [engine conformance account](../../engine/CONFORMANCE.md) states DL6's implemented policy;
[engine/README.md](../../engine/README.md) describes the boundary and startup. Neither is a new
portable promise. Qualification must use the actual shell, not just synthesized snapshots.

| README need or opening | Present evidence; missing work |
| --- | --- |
| Atomic first start and domain | `engine/crates/core/src/lib.rs:648` first-start and `conformance/conventions/first-start/initial/case.yaml`, `repeat/case.yaml`, `trust-domain/case.yaml` cover foundation/domain/repetition slices. CONFORMANCE.md:23 states atomic empty-store initialization; shell store `store.rs:60` writes/fsyncs/renames. S9-CUT must demonstrate actual before/after commit faults. |
| Later start and retained state, OPS-008 | CONFORMANCE.md DL2/DL6 and `engine/crates/core/src/requests.rs:1496` recover interrupted work unknown without replay. Shell tests `tests.rs:1250` kill before send and after send. S9-CORE/CUT qualify later/repeated starts, retained identities/keys and independent external effects. |
| Startup repair | CONFORMANCE.md:324 keeps serve available after setup failure; `conformance/conventions/setup/restore/case.yaml` is an existing core slice. Unknown hooks are not called again (OPS-021); a failed hook gets one startup attempt, still fails before explicit contract repair. S9-CORE qualifies both reachable T4 branches. |
| Executable fence after timeout, OPS-013/014 | `engine/crates/shell/src/runner.rs:1031–1047` drops late replies; `docs/work-orders/077-spec-reconcile-fenced.md:377–380` excludes shell work. Shared S4-LATE must deliver the late hook completion to 077's core before T4's explicit no-effect reconciliation can succeed. |
| PND-006 evidence | Promise exists, pending-action implementation absent in `core/src/lib.rs:63`. 047 supplies IDs, computed setup parts/evidence and answers; 051b supplies author panel access. S9-CORE must qualify evidence before claiming P4. |
| Protected foundation | `conformance/conventions/first-start/lifecycle/case.yaml` and `root-system/case.yaml` cover FST-006/LIF-034/ACC-024 slices. S9-CORE qualifies public rejections for foundational submissions, retirement, unregister, protected root routes and local declare-lost. |
| Explicit replacement/lift | 041b is implemented, but 042's tombstone/lift/ACC-055 changes are absent. 052b supplies author replacement commands. S9-CORE tests explicit close, restart without recreation, lift and new-ID replacement; startup is never replacement. |
| T4 repair/readback contract | No supplied repair fixture yet. S9-SRV defines a separate ready Resource whose explicit operation establishes the hook's backing precondition; independent readback confirms both absence of effect and ended work. This is package behavior, not a generic engine repair verb. |
| DL6 opening | Deliberate implementation-defined policy, not a language gap requiring a new repair operation. Q4 excludes arbitrary damaged-foundation reconstruction; first-start cuts must obey the existing engine policy. |
| G3 safety field | Gap: IN_FORCE topic 13 still says spec pending; `safety_rows` does not implement the promise field. 071 → SV1 writes the agreed form before any S9 safety vector is written. |
| Missing root-to-Pool route example | FLEET-005 makes protected defaults now; the old startup-repair example is not proof of current public reachability. Q4 uses reachable authored precondition failure/lost reply, not damaged/legacy seeds. No new spec order is needed for that accepted scope. |

The base is `31acd73`, with 041b at `1091a71`. Order files 042, 049a1, 050a, 070–072,
076–082 and 038c are present but their requested work is absent: State still clones at
`core/src/lib.rs:305`, pending/tombstone maps and manifest pieces are absent, Agent Codex is
0.1.1. OPS-014 at `spec/conventions/operations.yaml:286` remains unfenced; 081's contract
corrections and 082's storage/install rules are also unwritten. Those commit subjects do not
prove implementation. As the admin stated, declare-lost/control dispatch and DL7 https fetching
are unfinished. Only the protected local declare-lost rejection is needed here, not a general
lost-node implementation; S9-CUT must not score an unsupported-operation error as that rejection.

[G5](../012-model-checking/G5.md) lists static safety choices for vectors, not passing coverage.
MC2-001 requires 077 for T4's false reconciliation; MC4-001/002/009 and MC6-001/002 are taken
by 081, including the failed-hook shutdown conflict and the limit of old/new-ID overlap.
MC7b-001/002 are taken by 082: detected storage loss is distinct from engine crash continuity,
and concurrent first package installs are not concurrent first initialization. S9 adds no
claim of arbitrary lost-store reconstruction or new serialization policy.

Order 075 is merged (`55cf747`): an Agent Type states the reach of its client’s own tools.

## The orders

A scenario vector goes in the core or shell order that makes it pass if that order stays at most L;
otherwise an admin vectors order immediately follows it and depends only on the core and shell orders
its cases need, never on a lane package, owner step or run. A provider-contract case goes in lane 8’s
A63-VR, A63-VM, A63-VP or A63-VS suite in [the provider plan](../../docs/implementation/PROVIDERS.md).
Every package this plan places under `packages/` is a scenario server: its directory is named
`scenario-s9-...`, and its README says that it serves S9 and is not a provider for general use
(the admin's rule, 2026-11-11).

Classes before pipeline fixes: XS <45 min, S 1–1.5 h, M 2–2.5 h, L 2.5–3.5 h.
Lines include fixtures/tests. A fresh skeptic must review this split before any dispatch.

“Tools checks” means the root Python document/ruling/name/schema/roadmap checks, Python
`tools/tests` with PyYAML, and `git diff --check`. Every changed Rust crate also runs
`cargo test --release`, `cargo fmt --check` and clippy with warnings denied; engine orders
use the pinned engine toolchain. Vector expectations are authored from promises, never regenerated.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S9-SRV | server | Repair/readback specialization reusing S4’s lost-reply hook | TPL-012/017, OPS-021/022 | S4-SRV2 | M, 300–450 lines | lane 8 | after hook interface |
| S9-CORE1 | core | V1/V2/V5 first/later start and foundation; baseline V3 | FST-001–003/005–007, OPS-008/018, LIF-034/041, ACC-024/033 | PIN, 042, SV1 | L, 550–800 lines | admin | after dependencies |
| S9-CORE2 | core | V4 explicit authored setup repair and pending evidence | OPS-014/021, LIF-027, PND-006 | PIN, 047, 077, 081, SV1, S4-SET | M, 400–600 lines | admin | after dependencies |
| S9-CUT1 | shell | Initialization cuts and feature-gated pinned lab binary | FST-003/007, OPS-008/018, DL6 | S9-CORE1 | M, 350–550 lines | admin | after dependencies |
| S9-CUT2 | shell | Restart/repair and local declare-lost refusal | OPS-008/021, LIF-034 | S9-CUT1, S9-CORE2, S4-LATE | M, 400–600 lines | admin | after dependencies |
| S9-H | harness | Scripted task branches and process/cut orchestration | README P2–P5, M0–M2 | SCR, S9-SRV, S4-SRV1, S9-CUT1 | M, 400–550 lines | lane 10 | after dependencies |
| S9-RUN | run | Two separately measured author runs | README P1–P5 | S9-H, S9-CUT, 051b, 052b, SV4 | two runs, about 1 h each | lane 10 | after dependencies |

**S9-SRV.** Propose Rust `packages/scenario-s9-repair/` with Cargo.toml, Cargo.lock, pinned toolchain,
conduit.yaml/contracts.yaml/README/tests. A hook fails without effect until a separate already-ready
repair Resource explicitly establishes its backing precondition; readback reports precondition/effect
count/work ended. Reuse S4-SRV2’s implementation/barrier for the no-effect hook that loses
its reply, rather than build it twice. Specialize only backing precondition/repair/readback.
Before reconciling failed, release a deliberately late terminal response after unknown through S4-LATE:
077's core records the fence without settling unknown. Independent readback verifies no effect/work remaining; it alone cannot
bypass the fence. No new confirmation API or provider edit during tasks. Checks: release/fmt/clippy,
schemas and install/use tests.

**S9-CORE1 / CORE2.** Internal core fixtures/completions, no lane server dependency.
CORE1 owns V1 `s9-first-initialize` (safety FST-003), V2 `s9-later-start` (safety OPS-018),
V5 `s9-protected-foundation` (safety LIF-034): foundation/provenance, three protected root
Conduits, IDs/domain/keys, closed-state continuity, protection/lift/new ID. Needs PIN/042/SV1.
V3 `s9-interrupted-initialize` (safety FST-003, DL6) already passes today's empty/committed
DL6 core cases: include it here, or an admin vectors order immediately after PIN/SV1 if those
make it pass, never behind shell cuts. Kill/reopen qualification is CUT1's separate work.
CORE2 owns V4 `s9-explicit-repair` (safety LIF-027 and OPS-021), after 047/077/081 and S4-SET:
pending IDs/evidence, failed/unknown hooks, fenced explicit reconcile, repair/one new attempt,
successful parts retained. S4-SET fixes hook retry/LIF-027 first; add only S9's qualification.
Hot core requests.rs/lifecycle.rs/access.rs/registration.rs, runner lib.rs/own manifest pieces;
registration.rs belongs chiefly to CORE1 protection/admission, CORE2 uses setup/pending seams.
Checks: engine release/fmt/clippy, tools, vector delta and experiment 011 release tests.

**S9-CUT1 / CUT2.** Shell tests/internal scripted hook/effect servers, no S9-SRV dependency.
Hot shell lib.rs/server.rs/runner.rs; identity.rs/CLI tables only for a scoped lab init seam;
runner lib.rs/own piece only if needed. CUT1 makes acknowledged cuts before durable commit
(empty reopen) and after file fsync/rename/directory fsync but before reply (complete reopen).
Pin a lab binary with these cuts behind a Cargo feature no release enables; test-only barriers
also have to be accessible in the binary authors run. CUT1 owns the feature/build/pin, CUT2
extends its repair cuts; H/RUN use that exact lab binary, not release-only unreachable barriers.
CUT2 cuts after effect dispatch/before completion commit, checks unknown/no resend, and repair
with protected local declare-lost refusal (not -32603). Update CONFORMANCE.md's H03 paragraph
(starting line 324) for that refusal. Drop the hook late-reply regression S4-LATE already owns;
use its tested delivery path. No production crash API, full declare-lost or new repair policy.
Checks: engine release/fmt/clippy, tools, vector delta and actual kill/reopen/barrier tests.

**S9-H.** Rust harness under this experiment uses SCR with separate Agent identities and existing
root CLI requests for initialization/management. Processes and fault channels acknowledge cuts;
never time sleeps to guess a commit. Fresh stores under `<lab>` separate configured/generated/
malformed domains, before/after cuts and failed/unknown hook branches. T2 uses S4-SRV1’s keyed effect and interrupted-call barriers; T2/T4 keep their store
through restart/repair. Run the pinned CUT1/CUT2 feature-enabled lab binary for acknowledged cuts. Drive T1–T5 and capture readback/effect ledgers, retained invocation IDs,
foundation/domain snapshots, pending evidence, diagnostics and manual actions. No engine/runner/
manifest edits. Stub-test scheduler and snapshot comparisons; local release/fmt/clippy.

**S9-RUN.** First qualify all vectors, actual commit cuts and repair packages. The coordinator
and fresh Codex author each do T1–T5, both cuts, both T4 branches and all T5 protected/replacement
branches; their counts stay separate. Give the fresh author spec/tasks/CLI format only. Pin
the CUT1/CUT2 feature-enabled lab binary and versions; retain M0–M2 evidence, requests/replies, fault events, attempts before/after restart,
manual repair/verification, changed author lines and time without access; panel actions apart.
Unknowns/missing branches remain visible, failed reruns remain dated. No native client/account
or subscription run is added and no tokens/private keys are captured.

## Shared with other plans

- SCR is owned by S4's [plan](../015-s4-partial-setup/plan.md). Use its MCP discovery/calls,
  keyed replay, barriers and redacted per-identity transcripts; no task transport is needed for S9.
- 070 → S0b supplies core seams; 071 manifest pieces; 072 public list; 042 tombstones/lift,
  inventory and bounded reconciliation; 046a/046b diagnostics; 047 computed setup actions;
  051a/051b author/panel access; 052b explicit replacement commands. Do not duplicate batch 1.
- 077 supplies failed reconciliation's fence in text/core. 081 supplies final run/stop/removal
  contract corrections. 082 supplies engine-versus-storage-loss limits and install serialization.
  Their order files exist but do not satisfy these dependencies; initialization cuts are DL6,
  not an experiment proving the disputed external storage behavior.
- S4-LATE, owned by S4's plan (M, 400–600 lines), forwards late ordinary/hook completions after
  timeout and qualifies the actual wire path; S9-CUT depends on it rather than duplicating integration.
- SV1 after 071 supplies safety fields before writing cases; SV2/SV3 cover existing/S1 vectors,
  SV4 requires later orders to tag their own. All five S9 vectors are safety, as named above.
- A63-RP/MP/PS may package the local choices but are not needed to use existing built-ins and
  file archive install. A63-H is provider qualification, not this repair Resource. PSD, secret
  stores, 050/sign-in and the remote-node orders are not S9 prerequisites.
- B2-S/C/H need the owner's rerun judgment before merging; no batch-2 forwarding, appointment,
  indirect secrets or withhold behavior is used here. No M2-1–M2-4 answer is selected.

## The order of the work

After review SCR1/2 and S4-SRV1/2 prepare shared client/effect/hook interfaces; S9-SRV
specializes S4-SRV2, then H uses them and the acknowledged lab build. Engine/vector work waits
batch 1 through 053 → PIN (including 052a-1/2 → 052b), unless admin assigns a side branch.
PIN/042/SV1 → CORE1 → CUT1; PIN/047/077/081/SV1/S4-SET → CORE2;
CUT1/CORE2/S4-LATE → CUT2 → RUN, joining H, 051b/052b and the pinned lab binary.
V3's passing core cases need no shell cut or lane server. S4-LATE has no S4 server dependencies.
Use [S4’s proposed hot-file merge order](../015-s4-partial-setup/plan.md#for-the-admin): CORE1/2 follow S6-DUTY1/2;
C3a's registration.rs changes follow CORE's. CUT1/2 follow S5-WIRE and precede S6-WIRE.
No task transport, owner presence or batch-2 gate for S9; independent packages stay outside
engine dependencies and are qualified by H.

## For the admin

- **A — split/cuts:** accept CORE1/2 and CUT1/2, or revise after actual seams land.
  **Recommendation:** these bounded parts, V3 passing core cases separate from kill/reopen,
  and a pinned lab feature no release enables so H/RUN can acknowledge real cuts.
  Protected local declare-lost needs its actual refusal; update H03, never count -32603.
  S4-LATE already owns the hook regression; CUT2 does not repeat it.
- **B — placement/index:** prepare server/harness interfaces now, merge engine/manifest work
  after PIN/SV1 through S4's common hot-file order. **Recommendation:** retain DL6 scope and
  exclude arbitrary damaged/legacy foundation reconstruction.

Planned on the admin's reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9 only. Provisional until the owner confirms.

## For the owner

None. Q3 is engine clients only; Q4 already selects documented initialization cuts and reachable
authored-setup repair. No arbitrary foundation reconstruction, cancellation guarantee or new
portable repair mechanism is proposed.

## Limits

This is a plan, not crash/repair evidence. Atomic initialization is this engine's stated policy;
it proves neither backup quality nor recovery from arbitrary storage loss. Scripted clients do
not qualify real-client presentation, distributed failover or every runtime provider.
