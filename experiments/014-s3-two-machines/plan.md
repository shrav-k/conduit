# S3: order-level plan

> **In short**
> - Admin proposal S3-CS: specify PLC-024's portable loss cause before asserting it.
> - Admin proposal S3-L1: root declare-lost and provider-gone retirement/closure, with V4.
> - Admin proposal S3-L2: unreachable/return, removal’s owed stop, and V1/V2/V3.
> - Admin proposal S3-RET1: core browser-return handling after 038c states it.
> - Admin proposal S3-RET2: person's Interface return relay and remote sign-in integration.
> - Admin proposal S3-EV: composite managed-enrollment V7 after 049b.
> - Admin proposal S3-S1ITEM: second-machine PATH and Docker items for S1 servers.
> - Admin A1: accept the loss/return/relay split and hot-file sequencing?
> - Owner: confirm D4’s reading for counted real-client runs; confirm A74 unmeasured-cell treatment.
> - Admin A4: confirm A63-DR's second-machine qualification and shared provider vectors?
> - Ruling 77, M2-1: SSH-M and SSH-R; Tailscale is the network.
> - Ruling 77, M2-2: the run user `conduit-run`, with no sudo.
> - Ruling 77, M2-3: ADDR and DSP, and `tailscale serve` set up after the Mac’s tailnet rename, before provider runs.
> - Ruling 77, M2-4: the distro locked down; Windows interop, `appendWindowsPath` and automount off.
> - Owner step S3-OF: choose the cut/heal/loss method before S3-H’s fault implementation and the runs.
> - Owner hold MC4-002: any extra fence on old/new-ID run overlap remains outside this plan.

## Status

Written on 2026-11-11 by work order 117, against `main` at `31acd7378d9134a205208ec9491a01d2ea459da0`.
A plan for the lane coordinators and the admin: it binds nothing, and changes nothing the owner
accepted in [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched.

Revised on 2026-11-11 by work order 121, against `8b5c8093c1ca2a7a73e71fd50cce90a8f522f4f8`.

## What `main` has now

The log records 041b at `1091a71`; commits headed 070–072, 076–082, 049a1 and 038c add order files. Their work is absent here; side-branch contents and
real-machine setup cannot be verified from this checkout. This agrees with the admin's state.

| README prerequisite or gap | Present evidence and remaining work |
| --- | --- |
| A74 first, H1–H10 observations | [ROADMAP](../../ROADMAP.md), line 70, remains open; `experiments/006-across-machines/` does not exist. Stage 0's observations are not in this tree. S3-A741/2 record used, unmeasured and contradictory observations, never infer results. |
| Enrollment G4 | Already partly met since the README: [enrollment.yaml](../../spec/promises/enrollment.yaml), lines 5–219, has ENR-001–008, including stable ID/evidence and typed fingerprint comparison; ENR-009–012 follow. [pending.yaml](../../spec/promises/pending.yaml), lines 243–270, has PND-008. V7 can now name ENR-006. Still missing: staged capability/engine and machine contracts (049a-1/049b), core, panel, reports and PND-008 implementation (049a-2/051c). Manual setup cannot pass T1. |
| Machine reports/placement | [registration.rs](../../engine/crates/core/src/registration.rs), lines 1261–1328, validates fixed placement/selectors. [State](../../engine/crates/core/src/lib.rs), lines 65–81, has no machine map/report input. 049a-1/049b, DSP, 051c and SSH-M (ruling 77) supply reports/admission/placement; S3-M qualifies both machines implementations. |
| Gates and remote identity | [CONFORMANCE.md](../../engine/CONFORMANCE.md), lines 33–40 and 58–61, claims loopback proof only; runtime calls/connections remain unserved. ADDR/DSP/SSH-R, AGT and SSH-M (ruling 77) supply the node path; S3 adds real fault evidence and V5. Root's token must never be accepted at the remote-run address. |
| Confidential secret delivery and S2 packages | S2's [plan](../013-s2-shared-login/plan.md) accounts for SEC-D/FLT-D, S2-F/L, A63-KC, 050 and B2-S/C/H. None is replaced here. Keychain stays on the engine's machine in its logged-in session. SEC-D delivers to the remote service only over the authenticated runtime channel. |
| Owed stops and lifecycle loss | CONFORMANCE.md, lines 123–130, includes both cascades and REV-010 but explicitly leaves declare-lost/control dispatch unfinished. [PRV-021](../../spec/promises/providers.yaml) requires retained provider custody; SSH-R supplies it. S3-L2 and S3-H prove receipt, nonempty pending obligation and completion after instance removal; stage 0 proves none of this. |
| Records, uncertainty, reconciliation | [requests.rs](../../engine/crates/core/src/requests.rs), lines 937–962, checks unknown reconciliation; [reconcile-false](../../conformance/conventions/requests/reconcile-false/case.yaml) exists. 077's negative fence is not implemented; wait for it before V2. 082 is likewise only an order file; keep state unreadability distinct from saved-record loss. |
| G5: portable loss cause | Still a language gap: [placement.yaml](../../spec/promises/placement.yaml), lines 337–359, requires provider-gone cause in status/retirement records, but [record.v1](../../spec/core/results.yaml), lines 154–205, specifies no portable cause field. S3-CS must specify it; no fixture invents a wire field. |
| Agent homes/sign-in and G1/G2/G3 | [Codex manifest](../../packages/agent-codex/conduit.yaml), lines 3–6, is 0.1.1/.v1. 050, the full 048/051b chain and 038c are pending; 075 is merged; 038c must state the carried-return behavior before S3-RET builds it. Reuse S2's batch-2 secret prohibition, package contracts and SV1 field prerequisites. |
| Docker, second accounts, owner setup | No Docker provider/package is present. A63-DR supplies both machines' isolated profile; S3-P qualifies its node reach, home and callback behavior. Owner supplies second subscriptions, labels and independent sign-ins. A81 is open (ROADMAP, line 74): S3-A81 checks carried Codex redirects first. Ruling 77 answered M2-1 to M2-4. |

Order 075 is merged (`55cf747`): an Agent Type states the reach of its client’s own tools.

## The orders

A scenario vector goes in the core or shell order that makes it pass if that order stays at most L;
otherwise an admin vectors order immediately follows it and depends only on the core and shell orders
its cases need, never on a lane package, owner step or run. A provider-contract case goes in lane 8’s
A63-VR, A63-VM, A63-VP or A63-VS suite in [the provider plan](../../docs/implementation/PROVIDERS.md).

G/E/P mean repository paper checks; engine fmt/clippy/release/vector delta (plus experiment 011
on core/loader edits); and crate fmt/clippy/release/admission, lockfile, toolchain and ignored output. Sizes include tests, before pipeline fixes. No order exceeds L.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S3-A741 | probe | A74's Rust stand-ins, gate and fault witnesses | IFC-017/022, PRV-025/028 | — | L, 600–850 lines | lane 9 | now |
| S3-O1 | owner | Stage-0 machine pair for A74 | A74 | owner availability | owner step | owner | before A742, no ruling 77 setup wait |
| S3-O2 | owner | Second accounts, Docker daemon/context and service on pair | README P2/P5, PRV-011 | S3-O1, ruling 77 setup | about 1 hour presence plus setup | owner | after ruling 77 setup |
| S3-OF | owner | Choose S3’s cut, heal and loss method (A/B/C below) | README T4/T5/T6, ruling 77 | — | A: estimated 9–18 extra active min/run; B/C: setup below | owner | before H’s fault implementation and R1–R4 |
| S3-A742 | probe | A74 dated real-machine observations, all H1–H10 accounted for | A74, not engine proof | S3-A741, S3-O1 | M, 200–350 lines plus observation session | lane 9 | after dependencies |
| S3-CS | spec | Portable PLC-024 loss-cause contract | PLC-024, REC-001 | PIN, 081, 082, SV1 | S, 120–200 lines | admin | after dependencies |
| S3-L1 | core | Declare lost and provider-gone path; V4 | PLC-017/024/027, CON-008 | PIN, 042, 049a-1, 049b, 081, S3-CS, SV1 | L, 650–900 lines | admin | after dependencies |
| S3-L2 | core | Unreachable/return and sole removal owed-stop owner; V1/V2/V3 | PLC-023, LIF-036, PRV-021, ACC-037, OPS-018 | S3-L1, 077 | L, 550–850 lines including cases; split vectors if above L | admin | after dependencies |
| S3-RET1 | core | Carried-return decisions; V6 | 038c promises, PND-010/011 | PIN, 038c, 048b-join, S3-A81a | L, 550–800 lines with V6 | admin | after manual-forward observation |
| S3-RET2 | shell | Interface return relay and remote sign-in integration | 038c's promises, IFC-049 | S3-RET1, 051b, 051c, DSP | L, 600–850 lines | admin | after dependencies |
| S3-A81a | probe | A81 manual SSH-forward carried-login route | A81, ruling 75 C5 | 050, S3-O2 | S, 120–200 lines plus sign-in | lane 9 | before RET1 dispatch |
| S3-A81b | probe | A81 person’s Interface carried-login route | A81, ruling 75 C5 | S3-A81a, S3-RET2 | S, 120–200 lines plus sign-in | lane 9 | after RET2 |
| S3-M | harness | Run common machine suite on SSH-M (ruling 77); record report | PRV-013/014/015, ENR-002/006 | A63-VM, A63-MP2, admin C, SSH-M (ruling 77) | S, 120–200 lines plus qualification | lane 9 | after dependencies |
| S3-P | harness | Two-machine package/profile preparation | PRV-006/011/019/029, PLC-020 | S2-L/F, S3-PATH, S3-S1ITEM, 050, AGT, A63-RK, SSH-R, A63-VR, A63-DR, DSP, S3-O2 | M, 350–500 lines | lane 9 | after dependencies |
| S3-H | harness | Partition, loss, state-unreadability and owed-stop witnesses | ACC-037, OPS-018, PRV-021 | S3-A742, S3-L2, 077, 082, S2-H, S3-P, A63-RK, A63-VR, S3-OF for fault implementation | L, 600–850 lines | lane 10 | after dependencies |
| S3-PATH | server | PATH manifests for S2 binaries on the second machine | MAN-010, PRV-007 | S2-L1/L2 binary interfaces, S2-F | M, 300–450 lines | lane 8 | after binary interfaces |
| S3-S1ITEM | server | Proposed S1 server PATH/Docker items in S1 example and fixtures | MAN-010, PRV-007 | S1 server binary interfaces | M, 300–450 lines | admin | after interface freeze |
| S3-EV | vectors | V7 enrollment composite beyond 049a1 cases | ENR-006 | 049b, SV1, PIN | S, 150–250 lines | admin | after dependencies; not RET2 |
| S3-R1 | run | Coordinator, local profile, T1–T6 | README P1–P6 | run gate below | about 1 hour owner presence | lane 10 | after run gate; the owner confirms D4's reading |
| S3-R2 | run | Fresh author, local profile, T1–T6 | README P1–P6 | S3-R1 | about 1 hour owner presence | lane 10 | after S3-R1; the owner confirms D4's reading |
| S3-R3 | run | Coordinator, isolated profile, T1–T6 | README P1–P6 | S3-R2 | about 1 hour owner presence | lane 10 | after S3-R2; the owner confirms D4's reading |
| S3-R4 | run | Fresh author, isolated profile, T1–T6 | README P1–P6 | S3-R3 | about 1 hour owner presence | lane 10 | after S3-R3; the owner confirms D4's reading |

**S3-A741/2 (A74, two orders only).** New probe crate, tests and dated records under
`experiments/006-across-machines/`. A741 builds engine-free stand-ins, gate proof/wrong-instance
checks, removed-ID replay, effect/stream witnesses and configurable silent cuts. Start A741
first and expose that silent-cut component for reuse by A63-H2b; build it only once. A742 runs
H1/H2/H3/H4/H6/H7 on the real pair, importing stage-0 observations only with dated evidence;
accounts for H5/H8/H9/H10 as measured or explicitly unmeasured/not used, never as passes.
No engine, enrollment, provider qualification or counted scenario claim. Checks G/P, reversible
fault restoration and witness completeness; contrary/missing evidence blocks only dependent tasks.

**S3-O1 / O2.** O1 prepares the stage-0 pair under the
[second-machine guide](../../docs/guides/second-machine.md), without waiting for ruling 77’s setup;
A742 needs O1, not O2. O2 then prepares second subscriptions, service/keychain and Docker on
the pair after ruling 77’s setup: the run user `conduit-run`, with no sudo, the coordinator’s key
moved to it, Codex signed in as it, and the distro locked down (ruling 77). Windows interop,
`appendWindowsPath` and automount are off: nothing in the distro starts Windows programs,
has the Windows path or mounts Windows drives. The owner supplies sudo for that setup.
Include the second daemon/context and PRV-011 confidential channel with A63-DR2b; never copy logins.
Before engine-facing runs, wait for ADDR and DSP, and `tailscale serve` set up after the Mac’s
tailnet rename (ruling 77); the owner renames the Mac before turning on HTTPS certificates. Each account signs in through Conduit's Interface login flow, as ruling 72's later answer asks; [S2's plan](../013-s2-shared-login/plan.md#logins-through-the-interface-ruling-72s-later-answer) lists where each is first tested.

**S3-CS.** Spec placement/records promises and core result schema, with examples as needed;
no engine/runner edits and no guessed field in advance. Settle G5 through the admin's finding
process, after 081/082; no new loss rule. Add `s3-loss-cause-schema` as non-safety:
PLC-024’s safety part is Conduit closure, not the cause field. Use validate-values and index registration after SV1; S3-L1 owns behavioral V4. Checks G and engine release tests.

**S3-L1.** Core loss module/report integration, shell operation exposure and conformance
fixtures/assertions. Hot core requests.rs, access.rs, lifecycle.rs, registration.rs; shell
server.rs; runner lib.rs and assigned manifest pieces; no shell runner.rs/lib.rs/identity.rs
or CLI tables planned. Expose declare-lost only when implemented, preserve root/local protection,
and apply provider-gone without dispatch. Own the deferred 049a1 cases
`nodes/declare-lost-removal` and `nodes/gone`. L2 alone owns the owed stop after removal. Own V4 `s3-declared-lost-return` (IFC-022), including
typed cause, closure and old-ID/name reuse; no duplicate owed-stop implementation. G/E.

**S3-L2.** Core reports/retirement and deterministic provider fixture; sole owner of the owed stop
after removal, reusable by S6-STOP; hot lifecycle.rs and
requests.rs, runner lib.rs and assigned manifest pieces, no other listed hot files planned.
Own V3 `s3-unreachable-return` (PLC-023): omission/unreachable retire nothing; separate loss
phase unregisters while cut, requires a received stop/nonempty obligation and its later completion.
Do not implement provider persistence twice: SSH-R owns custody; 081's old/new-ID overlap is
observed, not fenced by a new rule. Checks G/E and explicit pending-stop witness assertions.

**S3-RET1/2.** RET1 uses core pending/sign-in modules, hot requests.rs and lifecycle.rs,
runner lib.rs/manifest pieces only if new return assertions are needed. RET2 uses shell
panel/return helpers, hot server.rs, runner.rs, lib.rs and CLI command tables, not identity.rs.
No registration.rs/access.rs changes planned. RET1 owns V6 `s3-second-account-sign-in`
(PND-011), including wrong-account undo/person-only evidence, using deterministic completions.
No lane package dependency; use scripted engine servers. Build the return only after 038c, using its actual
promise IDs, allowlist, consent, single redirect and account check; no Claude code is carried.
No guessed callback protocol or second authentication system. RET2 adds wire/return security tests with internal scripted servers, no duplicate vector. Checks G/E.

**S3-A81a / A81b.** Own probe directory `experiments/024-codex-carried-login/`, the next
unused number other than reserved 022 (023 is already paper-count). Future creation adds its
experiment-index row and updates A81’s existing register row under roadmap rules. A81a observes the route by a hand-made SSH
forward before RET1 dispatch; A81b observes the person's Interface after RET2. One Codex carried
login per route, checked account and redacted dated evidence, no retained code or model workload.
Both routes are needed for A81's done-when. G/P where Rust is added.

**S3-M.** Harness under this experiment's machine-qualification directory only runs A63-VM on
SSH-M (ruling 77) and records tool/version/verdicts. Depends on A63-VM,
A63-MP2 and admin C's settled local qualification. Do not rebuild machine cases here; request
missing reports/placement/enrollment cases in A63-VM. Checks G/P and common-suite report.

**S3-PATH / S3-S1ITEM.** An archive item runs only on the engine's own runner (078).
PATH supplies S2-L binaries' own manifests under their separate packages (lane 8), with platform
qualification and explicit unpinned-program limits. S1ITEM is proposed to admin for S1 example/
fixtures' server PATH items and container-runnable items, shared with S10; no second server or
provider implementation. Both are M, 300–450 lines; check manifests/admission and real target
execution. Engine changes, if needed, get internal scripted fixtures rather than a lane dependency.

**S3-P.** Rust preparation crate/fixtures under `experiments/014-s3-two-machines/profiles/`;
pin node-capable S1 directory/history and S2 service/filter/Agent items for each architecture.
AGT's PATH item is an explicit unpinned-program shortcut, never an archive digest guarantee.
Qualify A63-DR on the second machine: process-tree stop, authenticated gate/channel, per-Agent
home, Codex callback reach and no service/store credential mounts; Claude stays beside its browser.
No second Docker provider or OCI conversion; use `conduit-run`, with no sudo, and the distro locked down (ruling 77). No new vectors; G/P.

**S3-H.** New Rust harness under `experiments/014-s3-two-machines/harness/`. Prepare four
actual timing cells per workload, effect/response barriers and revocation during cuts. S3-OF
settles the cut/heal/loss mechanism below; S3-H changes with that answer. Under (A), each cut,
heal and loss barrier prints the one Windows command for the owner and waits for the owner’s
confirmation before continuing; it never infers that the command ran from elapsed time.
Do not implement (A) as accepted before the answer. Separately fault current state at the engine. Keep SSH-R
reachable on the engine's machine while the node is down. Before healing T6 require a received
stop and nonempty owed-stop evidence keyed by removed instance/run ID, then match completion.
Include 081's old/later-run overlap observation. No production/index changes or new vectors. G/P.

**S3 vector placement.** V1 `s3-partition-current-access` (ACC-037) and V2
`s3-lost-response-effect` (OPS-018) go with S3-L2, or an admin vectors order immediately after
077's build if that makes them pass; retain deterministic completions, no S2-L dependency.
V3/V4 remain L2/L1's. V5 `s3-gate-identity` (PRV-025) is A63-VR's provider safety case.
V6 belongs to RET1. S3-EV owns V7 `s3-managed-enrollment` (ENR-006) after 049b: cite 049a1's
`enrollment/admit-fingerprint`, `enrollment/admit-not-admissible` and
`nodes/no-node-without-admission`; add only uncovered composite behavior, without RET2.
Cases/index only, G/release vectors/delta; no separate S3-V1/V2 orders remain.

**S3-R1–4.** Dated evidence beside README; each fresh `<lab>` performs T1–T6, two workload
replays, eight partition cells and two loss/return phases. Gate: PIN; S2's prerequisites (passing S2 runs are not required); B2-S/C/H; 049a-1/2, 049b, 051c, A63-MDIS; AGT/ADDR/DSP/A63-RK/SSH-R, A63-VR and SSH-M (ruling 77);
S3-A742/A81a/A81b/M/P/H, SEC-D/FLT-D0/1/2, 077/081/082, S3-PATH/S3-S1ITEM, placed vectors and all tagged safety cases.
Provider runs also wait for ADDR and DSP, and `tailscale serve` set up after the Mac’s tailnet
rename (ruling 77). S3-OF’s answer gates all four runs. Pin inputs before timing; counted runs wait: the owner confirms D4's reading. No code edits;
checks are README P1–P6 and M0/M1/M2, including actual outage/stop/effect evidence.

## Shared with other plans

- Build on `s3/m2`, not another enrollment/transport implementation: AGT (078), ADDR (079),
  DSP (080), 049a-1, SSH-R, SSH-M (ruling 77),
  049b, 049a-2 and revised 051c. DSP forwards enrollment's provider operations; 051c supplies
  DL5 reports. [A63-MDIS](../../docs/implementation/PROVIDERS.md#the-orders) supplies engine
  place/list-machines dispatch beyond that report loop and gates runs, not direct provider-suite
  qualification. SSH-R depends on A63-RK, not another provider or A63-H; its qualification
  needs A63-VR. SSH-M is a separate crate under the provider plan’s admin decision D.
  Both are unblocked by ruling 77; runs use `conduit-run`, with no sudo. Real-host tests need owner setup.
- S2 owns SEC-D, FLT-D, S2-F/L and S2-H. Order 120 owns A63-H/MP/KC/DR; machine providers
  here must pass the same reports/placement vectors. A63-DR must support the second-machine
  profile; no new Docker crate here. PSD/Git/OCI fetching is not needed for pinned file installs.
- Batch 1 supplies 042, 046a/b, 047, 048a-1/2, 048b-relay/join, 050 (050a/076),
  051a/b, 052a-1/2, 052b, 053 and PIN, with 070/S0b/S0c and 043a/043b joining as S2 lists.
  038c must land (075 is merged) before their packages/return builds; B2-S/C/H supplies driven-Agent
  secret exclusion and one-hop inspection/plans after the owner's rerun judgment.
- 077's fence, 081's run/loss limits and 082's records/install fixes are not merged work here.
  SV1 after 071 precedes every new safety vector; SV2/SV3 cover prior cases; apply SV4 throughout.
  See [S4](../015-s4-partial-setup/plan.md), [S10](../021-s10-swapped-providers/plan.md)
  and [PROVIDERS](../../docs/implementation/PROVIDERS.md) for shared splits.

## The order of the work

Dispatch S3-A741 now; S3-O1’s stage-0 pair joins before S3-A742, without ruling 77’s setup or O2. A74 runs
before any dependent engine task. T1 waits for A742's H1/H3; T2/T3 for H1/H2/H3/H7 and H4
on removal/replacement; T4/T5 for H1/H2/H3/H6/H7 and H4 on repair; T6 for H1/H2/H3/H4/H7
with H6's unconfirmed-work evidence. A741 alone meets no task's observation dependency.
Whether H5/H8/H9/H10 may remain unmeasured is held for the owner below; no waiver is inferred.

The main path to PIN is batch 1 through 053 as S2 lists. ADDR → DSP and 049a-1 → 049b
stay on `s3/m2` until PIN, then merge with the admin's rebases. 047/048a-2 → 049a-2 and
DSP/049a-1 → 051c join; PIN/049a-1/DSP/SV1 → A63-MDIS joins the engine-use run gate.
After PIN, 081/082 → S3-CS → S3-L1 → S3-L2;
050/S3-O2 → S3-A81a (manual forward) → S3-RET1 → S3-RET2 → S3-A81b (Interface).
A63-RK → SSH-R (ruling 77) → A63-VR joins P/H and the run gate. S2-L → S3-PATH and
S1 interfaces → S3-S1ITEM join P; Docker's second daemon/profile joins S3-O2/A63-DR2b.
A63-VM/MP2/admin C join S3-M. B2-S/C/H waits for the S1 judgment. The last of these joins,
A74 observations, ruling 77’s TLS setup, S3-OF’s answer, remaining owner holds and “the owner confirms D4's reading” on counted runs determines
R1 → R2 → R3 → R4. No other order waits on D4. S2 prerequisites suffice; S2 passing runs do not gate S3.
Use [S4’s proposed hot-file merge order](../015-s4-partial-setup/plan.md#for-the-admin): admin interleaves S3-L1/L2/RET1
against SEC-D1/FLT-D1 in core, and RET2 against SEC-D2/D3 and FLT-D2 in shell.

## For the admin

- **A1, split/merge:** one machine increment or the proposed separate loss/return/relay orders.
  **Recommendation:** separate them; serialize requests.rs/lifecycle.rs merges, then shell
  lib.rs/runner.rs/CLI-table merges against DSP/051c. Reserve manifest pieces via 071.
- **A3, D4:** Planned on the admin's reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9 only. Provisional until the owner confirms.

- **A4, providers:** extend A63-DR qualification to the second machine, or defer its profile.
  **Recommendation:** coordinate that qualification with order 120; S3's isolated runs wait.
  Agree the shared machine-vector inventory with 049b/A63-H before dispatching S3-M.

## For the owner

- **D4:** counted S3-R1–R4 wait: the owner confirms D4's reading. No build, probe or
  preparation waits on it; the provisional admin reading is recorded above.
- **A74 unmeasured cells:** does “A74's dated H1–H10 observations must exist first” permit
  recording H5/H8/H9/H10 unmeasured while running S3's used hypotheses, or require all ten
  measured first? **Recommendation:** permit explicitly unmeasured unused cells, with no pass
  or waiver of A74's wider register obligation; missing/contrary used evidence blocks its task.
  This narrows the accepted prerequisite only if the owner agrees; it is no longer admin A2.
- **Ruling 77:** SSH-M/SSH-R, the run user `conduit-run` with no sudo, `tailscale serve`
  after the Mac’s tailnet rename, and the distro locked down answer M2-1 to M2-4. O2 and
  affected live work wait for setup; O1/A741/A742’s stage-0 work does not. MC4-002’s overlap question stays held.
- **S3-OF — how should S3 cut/heal the network and lose the machine?** With Windows
  interop off, nothing inside Ubuntu-24.04 can run `wsl --shutdown` or set the Windows
  firewall; `conduit-run` has no sudo to set a Linux-side rule either. The README’s
  coordinator-run Linux firewall/WSL method therefore needs the owner’s answer before runs.
  Options (time estimates are planning allowances, not measured results):

  - **(A) The owner’s hand at a barrier.** At each cut, heal and loss, stop and print one
    command for the owner to run on Windows: a PowerShell firewall command that blocks
    traffic to/from the Mac’s tailnet address, its removal, or `wsl --shutdown`. Continue
    only after the owner confirms completion. Each workload has four partition cells
    (four cuts and four heals) and one loss phase (one shutdown); two workloads make
    **18 command-and-confirm actions per run**, **72 for four runs**. Allow 30–60 seconds
    per action: **9–18 extra active owner minutes per run**, **36–72 minutes for four**.
    Added to the README’s proposed hour for enrollment, sign-ins and recovery, reserve
    **69–78 active minutes per run**, **4 hours 36 minutes–5 hours 12 minutes for four**,
    plus availability to restore WSL. The two WSL restarts per run remain recovery work
    in that existing allowance; they are not included in the 18 fault commands. Only the
    temporary Windows firewall rule and WSL shutdown change the machine; nothing new
    is installed or granted.
  - **(B) A Windows-side mechanism set up once.** For example, a Windows OpenSSH account
    with a key restricted to the cut, heal and shutdown commands and reachable only from
    the Mac’s tailnet address. Allow **30–60 minutes of owner setup once** to install or
    enable the server if needed, configure the account/key restrictions and address rule,
    and verify them. It adds a remote entry to the Windows host and privileged firewall/
    shutdown authority limited to those commands. The coordinator can then drive the
    barriers; allow the README’s hour per run (four hours total), plus WSL recovery
    availability and any troubleshooting. The estimate is unmeasured.
  - **(C) A helper inside the distro.** A dedicated fault user, separate from `conduit-run`,
    with a sudo rule for one root-owned script that sets/removes a Linux-side rule and
    powers the distro off. Allow **20–40 minutes of owner setup once** for the account,
    script, sudo rule and checks, plus the README’s hour per run (four hours total) and
    WSL recovery availability. This adds a privilege that ruling 77’s “no sudo” kept out,
    even though `conduit-run` keeps no sudo. Power-off from inside may differ from
    `wsl --shutdown`; equivalence to S3-T6’s loss must be checked before counting a run.

  **Coordinator’s recommendation: (A) for the four counted runs.** The owner is present
  anyway for enrollment, sign-ins and recovery; it installs and grants nothing, and each
  action is one line. (B) is the fallback if (A)’s time proves too much. This recommends
  one option and accepts none: S3-OF remains a new owner step, S3-H changes with the
  answer, and S3-R1–R4 wait for it.

## Limits

No observed A74/A81 or scenario result is claimed; manual enrollment cannot pass T1.
No new language beyond the proposed G5 finding process, peer/failover/NAT/renewal/OCI work,
second Docker provider or confinement guarantee. Static placement/admission decisions need
vectors even where G5 names a model; bounded model evidence does not prove real TLS or
durable provider stops. Retired/removed is not stopped, unknown is not failed, and retained
secrets are not recalled. Keep ACC-027 and every accepted measurement/credential limit.
