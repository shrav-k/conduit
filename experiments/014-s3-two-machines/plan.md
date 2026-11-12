# S3: order-level plan

> **In short**
> - Admin proposal S3-CS: specify PLC-024's portable loss cause before asserting it.
> - Admin proposal S3-L1: root declare-lost and provider-gone retirement/closure, with V4.
> - Admin proposal S3-L2: unreachable/return and removal's stop boundary, with V3.
> - Admin proposal S3-RET1: core browser-return handling after 038c states it.
> - Admin proposal S3-RET2: person's Interface return relay and remote sign-in integration.
> - Admin proposal S3-V1: partition, uncertainty and gate vectors V1/V2/V5 and index entries.
> - Admin proposal S3-V2: account/enrollment vectors V6/V7 and index entries.
> - Admin A1: accept the loss/return/relay split and hot-file sequencing?
> - Admin A2: accept two A74 orders with explicit unmeasured hypotheses?
> - Admin A3: who resolves D4's real-client wording before counted S3 runs?
> - Admin A4: confirm A63-DR's second-machine qualification and shared provider vectors?
> - Owner hold M2-1: machine provider; SSH-M is only the admin's candidate.
> - Owner hold M2-2: operating-system user for runs on the second machine.
> - Owner hold M2-3: TLS termination at the engine's HTTPS address.
> - Owner hold M2-4: whether the second machine's Linux is locked down.
> - Owner hold MC4-002: any extra fence on old/new-ID run overlap remains outside this plan.

## Status

Written on 2026-11-11 by work order 117, against `main` at `31acd7378d9134a205208ec9491a01d2ea459da0`.
A plan for the lane coordinators and the admin: it binds nothing, and changes nothing the owner
accepted in [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched.

## What `main` has now

The log records 041b at `1091a71`; commits headed 070–072, 075–082, 049a1 and 038c add order files. Their work is absent here; side-branch contents and
real-machine setup cannot be verified from this checkout. This agrees with the admin's state.

| README prerequisite or gap | Present evidence and remaining work |
| --- | --- |
| A74 first, H1–H10 observations | [ROADMAP](../../ROADMAP.md), line 70, remains open; `experiments/006-across-machines/` does not exist. Stage 0's observations are not in this tree. S3-A741/2 record used, unmeasured and contradictory observations, never infer results. |
| Enrollment G4 | Already partly met since the README: [enrollment.yaml](../../spec/promises/enrollment.yaml), lines 5–219, has ENR-001–008, including stable ID/evidence and typed fingerprint comparison; ENR-009–012 follow. [pending.yaml](../../spec/promises/pending.yaml), lines 243–270, has PND-008. V7 can now name ENR-006. Still missing: staged capability/engine and machine contracts (049a-1/049b), core, panel, reports and PND-008 implementation (049a-2/051c). Manual setup cannot pass T1. |
| Machine reports/placement | [registration.rs](../../engine/crates/core/src/registration.rs), lines 1261–1328, validates fixed placement/selectors. [State](../../engine/crates/core/src/lib.rs), lines 65–81, has no machine map/report input. 049a-1/049b, DSP, 051c and the M2-1-selected provider supply reports/admission/placement; S3-M qualifies both machines implementations. |
| Gates and remote identity | [CONFORMANCE.md](../../engine/CONFORMANCE.md), lines 33–40 and 58–61, claims loopback proof only; runtime calls/connections remain unserved. ADDR/DSP/SSH-R, AGT and the selected machine provider supply the node path; S3 adds real fault evidence and V5. Root's token must never be accepted at the remote-run address. |
| Confidential secret delivery and S2 packages | S2's [plan](../013-s2-shared-login/plan.md) accounts for SEC-D/FLT-D, S2-F/L, A63-KC, 050 and B2-S/C/H. None is replaced here. Keychain stays on the engine's machine in its logged-in session. SEC-D delivers to the remote service only over the authenticated runtime channel. |
| Owed stops and lifecycle loss | CONFORMANCE.md, lines 123–130, includes both cascades and REV-010 but explicitly leaves declare-lost/control dispatch unfinished. [PRV-021](../../spec/promises/providers.yaml) requires retained provider custody; SSH-R supplies it. S3-L1/2 and S3-H prove receipt, nonempty pending obligation and completion after instance removal; stage 0 proves none of this. |
| Records, uncertainty, reconciliation | [requests.rs](../../engine/crates/core/src/requests.rs), lines 919–962, checks unknown reconciliation; [reconcile-false](../../conformance/conventions/requests/reconcile-false/case.yaml) exists. 077's negative fence is not implemented; wait for it before V2. 082 is likewise only an order file; keep state unreadability distinct from saved-record loss. |
| G5: portable loss cause | Still a language gap: [placement.yaml](../../spec/promises/placement.yaml), lines 337–359, requires provider-gone cause in status/retirement records, but [record.v1](../../spec/core/results.yaml), lines 154–205, specifies no portable cause field. S3-CS must specify it; no fixture invents a wire field. |
| Agent homes/sign-in and G1/G2/G3 | [Codex manifest](../../packages/agent-codex/conduit.yaml), lines 3–6, is 0.1.1/.v1. 050, the full 048/051b chain, 075 and 038c are pending; 038c must state the carried-return behavior before S3-RET builds it. Reuse S2's batch-2 secret prohibition, package contracts and SV1 field prerequisites. |
| Docker, second accounts, owner setup | No Docker provider/package is present. A63-DR supplies both machines' isolated profile; S3-P qualifies its node reach, home and callback behavior. Owner supplies second subscriptions, labels and independent sign-ins. A81 is open (ROADMAP, line 74): S3-A81 checks carried Codex redirects first. M2-1–4 stay unanswered. |

## The orders

G/E/P mean repository paper checks; engine fmt/clippy/release/vector delta (plus experiment 011
on core/loader edits); and crate fmt/clippy/release/admission, lockfile, toolchain and ignored output. Sizes include tests, before pipeline fixes. No order exceeds L.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S3-A741 | probe | A74's Rust stand-ins, gate and fault witnesses | IFC-017/022, PRV-025/028 | — | L, 600–850 lines | lane 9 | now |
| S3-O | owner | Prepare real machines, runtime and second accounts | README P2/P5 | M2-1–4, owner availability | about 1 hour presence, setup time recorded | owner | after held choices |
| S3-A742 | probe | A74 dated real-machine observations, all H1–H10 accounted for | A74, not engine proof | S3-A741, S3-O | M, 200–350 lines plus observation session | lane 9 | after dependencies |
| S3-CS | spec | Portable PLC-024 loss-cause contract | PLC-024, REC-001 | PIN, 081, 082, SV1 | S, 120–200 lines | admin | after dependencies |
| S3-L1 | core | Declare lost and provider-gone path; V4 | PLC-017/024/027, CON-008 | PIN, 042, 049a-1, 049b, 081, S3-CS, SV1 | L, 650–900 lines | admin | after dependencies |
| S3-L2 | core | Unreachable/return and removed-instance stop boundary; V3 | PLC-023, LIF-036, PRV-021 | S3-L1 | M, 400–600 lines | admin | after S3-L1 |
| S3-RET1 | core | Carried-return decisions and one-login constraints | 038c's promises, PND-010/011 | PIN, 038c, 048b-join | M, 400–600 lines | admin | after dependencies |
| S3-RET2 | shell | Interface return relay and remote sign-in integration | 038c's promises, IFC-049 | S3-RET1, 051b, 051c, DSP | L, 600–850 lines | admin | after dependencies |
| S3-A81 | probe | Codex carried-login observations by both routes | A81, ruling 75 C5 | 050, S3-RET2, S3-O | S, 120–200 lines plus owner sign-ins | lane 9 | after dependencies |
| S3-M | harness | Qualify local/selected machine implementations alike | PRV-013/014/015, ENR-002/006 | 049b, A63-H, A63-MP, M2-1 provider | M, 300–450 lines | lane 9 | after dependencies |
| S3-P | harness | Two-machine package/profile preparation | PRV-006/019/029, PLC-020 | S2-L/F, 050, AGT, SSH-R, A63-DR, DSP, S3-O | M, 350–500 lines | lane 9 | after dependencies |
| S3-H | harness | Partition, loss, state-unreadability and owed-stop witnesses | ACC-037, OPS-018, PRV-021 | S3-A742, S3-L2, 077, 082, S2-H, S3-P | L, 600–850 lines | lane 10 | after dependencies |
| S3-V1 | vectors | V1/V2/V5 with deterministic network/gate fixture | ACC-037, OPS-018, PRV-025 | SV1, 077, S3-L2, SEC-D, FLT-D, DSP, S2-L | L, 400–650 lines | admin | after dependencies |
| S3-V2 | vectors | V6/V7: sign-in and managed admission | PND-011, ENR-006 | SV1, 049a-2, 049b, 051c, S3-RET2 | M, 300–450 lines | admin | after dependencies |
| S3-R1 | run | Coordinator, local profile, T1–T6 | README P1–P6 | run gate below | about 1 hour owner presence | lane 10 | after run gate and A3 resolution |
| S3-R2 | run | Fresh author, local profile, T1–T6 | README P1–P6 | S3-R1 | about 1 hour owner presence | lane 10 | after S3-R1 |
| S3-R3 | run | Coordinator, isolated profile, T1–T6 | README P1–P6 | S3-R2 | about 1 hour owner presence | lane 10 | after S3-R2 |
| S3-R4 | run | Fresh author, isolated profile, T1–T6 | README P1–P6 | S3-R3 | about 1 hour owner presence | lane 10 | after S3-R3 |

**S3-A741/2 (A74, two orders only).** New probe crate, tests and dated records under
`experiments/006-across-machines/`. A741 builds engine-free stand-ins, gate proof/wrong-instance
checks, removed-ID replay, effect/stream witnesses and configurable silent cuts. A742 runs
H1/H2/H3/H4/H6/H7 on the real pair, importing stage-0 observations only with dated evidence;
accounts for H5/H8/H9/H10 as measured or explicitly unmeasured/not used, never as passes.
No engine, enrollment, provider qualification or counted scenario claim. Checks G/P, reversible
fault restoration and witness completeness; contrary/missing evidence blocks only dependent tasks.

**S3-O.** Owner prepares the actual pair per [second-machine](../../docs/guides/second-machine.md)
and [Docker](../../docs/guides/docker.md) guides, under the eventual M2-1–4 answers. No rented
substitute, chosen user/network name or credential is written here. Includes service/keychain
and second account readiness, not copying login files. Check readiness against README; no vectors.

**S3-CS.** Spec placement/records promises and core result schema, with examples as needed;
no engine/runner edits and no guessed field in advance. Settle G5 through the admin's finding
process, after 081/082; no new loss rule. Add `s3-loss-cause-schema` (safety PLC-024) with
validate-values and index registration after SV1; S3-L1 owns behavioral V4. Checks G and engine release tests.

**S3-L1.** Core loss module/report integration, shell operation exposure and conformance
fixtures/assertions. Hot core requests.rs, access.rs, lifecycle.rs, registration.rs; shell
server.rs; runner lib.rs and assigned manifest pieces; no shell runner.rs/lib.rs/identity.rs
or CLI tables planned. Expose declare-lost only when implemented, preserve root/local protection,
and apply provider-gone without dispatch, including unregister's stop request. Own V4 `s3-declared-lost-return` (IFC-022), including
typed cause, closure, old-ID/name reuse and a received stop obligation after unregister. G/E.

**S3-L2.** Core reports/retirement and deterministic provider fixture, reusing L1's stop boundary; hot lifecycle.rs and
requests.rs, runner lib.rs and assigned manifest pieces, no other listed hot files planned.
Own V3 `s3-unreachable-return` (PLC-023): omission/unreachable retire nothing; separate loss
phase unregisters while cut, requires a received stop/nonempty obligation and its later completion.
Do not implement provider persistence twice: SSH-R owns custody; 081's old/new-ID overlap is
observed, not fenced by a new rule. Checks G/E and explicit pending-stop witness assertions.

**S3-RET1/2.** RET1 uses core pending/sign-in modules, hot requests.rs and lifecycle.rs,
runner lib.rs/manifest pieces only if new return assertions are needed. RET2 uses shell
panel/return helpers, hot server.rs, runner.rs, lib.rs and CLI command tables, not identity.rs.
No registration.rs/access.rs changes planned. Build the return only after 038c, using its actual
promise IDs, allowlist, consent, single redirect and account check; no Claude code is carried.
No guessed callback protocol or second authentication system. New V1–V7 belong only to the
orders assigned here, not RET; module tests cover return/security paths. Checks G/E.

**S3-A81.** Probe fixtures and redacted dated evidence under `experiments/014-s3-two-machines/a81/`;
One Codex carried login per route (person's Interface and SSH forward), account checked, no code retained. Manual forwarding can be observed earlier, but both routes are needed for
the register's done-when. No model workload or portable vectors. Checks G/P where code is added.

**S3-M.** Harness under `experiments/014-s3-two-machines/machine-qualification/`, consuming
A63-H's same contract vectors for A63-MP and the M2-1-selected implementation. Required cases:
list-machines states/omission versus gone, provider ID/machine ID/hostname mismatch, unique
identity and fixed labels, contract-defined enrollment key/evidence, admission/refusal/expiry/removal,
place.v1 selectors and no-match, valid machine-or-none, shared-host refusal and node-local paths.
Cover PRV-005/013–015/018, ENR-002/005–010, PLC-007/012/013/020/023/026 and schema rejection.
No provider/index edits; request missing shared cases from order 120/049b. G/P.

**S3-P.** Rust preparation crate/fixtures under `experiments/014-s3-two-machines/profiles/`;
pin node-capable S1 directory/history and S2 service/filter/Agent items for each architecture.
AGT's PATH item is an explicit unpinned-program shortcut, never an archive digest guarantee.
Qualify A63-DR on the second machine: process-tree stop, authenticated gate/channel, per-Agent
home, Codex callback reach and no service/store credential mounts; Claude stays beside its browser.
No second Docker provider, OCI conversion or chosen M2 user/lockdown policy. No new vectors; G/P.

**S3-H.** New Rust harness under `experiments/014-s3-two-machines/harness/`. Prepare four
actual timing cells per workload, effect/response barriers, revocation during cuts, reversible
Linux-side drops and WSL shutdown; separately fault current state at the engine. Keep SSH-R
reachable on the engine's machine while the node is down. Before healing T6 require a received
stop and nonempty owed-stop evidence keyed by removed instance/run ID, then match completion.
Include 081's old/later-run overlap observation. No production/index changes or new vectors. G/P.

**S3-V1.** Conformance only, including admin-owned fixture/index registration: V1
`s3-partition-current-access` (ACC-037), V2 `s3-lost-response-effect` (OPS-018), V5
`s3-gate-identity` (PRV-025). Use S2-L's local stub, deterministic network/provider completions
and proof facts; actual cryptography/TLS has DSP tests and A74/run evidence. No runner or engine
changes in this vectors order. Check G, release vectors and delta; never infer no effect from a timeout.

**S3-V2.** Conformance only plus admin-owned index registration: V6 `s3-second-account-sign-in`
(PND-011) and V7 `s3-managed-enrollment` (ENR-006, now present). Wrong-account undo and
person-only evidence, wrong-machine/unconfirmed admission, expiry and weaker-check conditions;
no live accounts or network. No runner/engine edits; G, release vectors and delta.

**S3-R1–4.** Dated evidence beside README; each fresh `<lab>` performs T1–T6, two workload
replays, eight partition cells and two loss/return phases. Gate: PIN; S2's prerequisites and
passing runs; B2-S/C/H; 049a-1/2, 049b, 051c; AGT/ADDR/DSP/SSH-R and selected machine provider;
S3-A742/A81/M/P/H, SEC-D/FLT-D, 077/081/082, both vector orders and all tagged safety cases.
Pin inputs before timing; real clients/accounts remain contingent on A3 below. No code edits;
checks are README P1–P6 and M0/M1/M2, including actual outage/stop/effect evidence.

## Shared with other plans

- Build on `s3/m2`, not another enrollment/transport implementation: AGT (078), ADDR (079),
  DSP (080), 049a-1, SSH-R, the M2-1-selected machine provider (SSH-M is the candidate),
  049b, 049a-2 and revised 051c. DSP forwards enrollment's provider operations; 051c supplies
  DL5 reports. SSH-R depends only on main for construction; real-host testing needs owner setup.
- S2 owns SEC-D, FLT-D, S2-F/L and S2-H. Order 120 owns A63-H/MP/KC/DR; machine providers
  here must pass the same reports/placement vectors. A63-DR must support the second-machine
  profile; no new Docker crate here. PSD/Git/OCI fetching is not needed for pinned file installs.
- Batch 1 supplies 042, 046a/b, 047, 048a-1/2, 048b-relay/join, 050 (050a/076),
  051a/b, 052a-1/2, 052b, 053 and PIN, with 070/S0b/S0c and 043a/043b joining as S2 lists.
  075/038c must land before their packages/return builds; B2-S/C/H supplies driven-Agent
  secret exclusion and one-hop inspection/plans after the owner's rerun judgment.
- 077's fence, 081's run/loss limits and 082's records/install fixes are not merged work here.
  SV1 after 071 precedes every new safety vector; SV2/SV3 cover prior cases; apply SV4 throughout.
  Sister plans of orders 118/119/120 remain their coordinators' work, not dependencies on unwritten links.

## The order of the work

Dispatch S3-A741 now; owner preparation and M2-1–4 answers join before S3-A742. A74 runs
before any dependent engine task. T1 waits for A742's H1/H3; T2/T3 for H1/H2/H3/H7 and H4
on removal/replacement; T4/T5 for H1/H2/H3/H6/H7 and H4 on repair; T6 for H1/H2/H3/H4/H7
with H6's unconfirmed-work evidence. A741 alone meets no task's observation dependency.
H5/H8/H9/H10 are not S3 task dependencies; unmeasured cells remain visible.

The main path to PIN is batch 1 through 053 as S2 lists. ADDR → DSP and 049a-1 → 049b
stay on `s3/m2` until PIN, then merge with the admin's rebases. 047/048a-2 → 049a-2 and
DSP/049a-1 → 051c join. After PIN, 081/082 → S3-CS → S3-L1 → S3-L2 can run beside
038c/048b → S3-RET1 → S3-RET2 → S3-A81. The chosen machine provider, A63-DR and S2's
completion join S3-M/P/H and V1/V2. B2-S/C/H waits for the S1 judgment, not merely PIN.
The last of these joins, A74 evidence, owner holds and A3 resolution is the critical path to
S3-R1 → R2 → R3 → R4. Stage 0 is preparation only, never partitions/loss/Docker/account proof.

## For the admin

- **A1, split/merge:** one machine increment or the proposed separate loss/return/relay orders.
  **Recommendation:** separate them; serialize requests.rs/lifecycle.rs merges, then shell
  lib.rs/runner.rs/CLI-table merges against DSP/051c. Reserve manifest pieces via 071.
- **A2, A74:** run all ten hypotheses before S3, or record all ten and run the six this topology
  uses first. **Recommendation:** the latter in A741/2, with explicit missing cells and no waiver
  of the register's wider probe. Missing/contrary used evidence stops the dependent task.
- **A3, D4:** read ruling 75 D4 as limiting optional S4/5/6/9 replays, or as limiting every
  scenario beyond S2. Ruling 72.2, 75 D2 and S3's accepted README require second-account Agents.
  **Recommendation:** admin reconcile the reading with the owner before scheduling runs;
  preserve the accepted four-run matrix and do not substitute SCR or erase account tests here.
- **A4, providers:** extend A63-DR qualification to the second machine, or defer its profile.
  **Recommendation:** coordinate that qualification with order 120; S3's isolated runs wait.
  Agree the shared machine-vector inventory with 049b/A63-H before dispatching S3-M.

## For the owner

No new split question beyond the admin's D4 referral. Existing holds M2-1–4 remain: machine
provider, run user, TLS termination and Linux lockdown. This plan selects none. S3-O and
affected live preparation wait; independent probe construction need not. No new overlap fence
is proposed: the existing MC4-002 question stays with the owner.

## Limits

No observed A74/A81 or scenario result is claimed; manual enrollment cannot pass T1.
No new language beyond the proposed G5 finding process, peer/failover/NAT/renewal/OCI work,
second Docker provider or confinement guarantee. Static placement/admission decisions need
vectors even where G5 names a model; bounded model evidence does not prove real TLS or
durable provider stops. Retired/removed is not stopped, unknown is not failed, and retained
secrets are not recalled. Keep ACC-027 and every accepted measurement/credential limit.
