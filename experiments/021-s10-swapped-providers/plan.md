# S10: order-level plan

> **In short**
> - **Admin / DSP (080):** serve runtime calls, discovery, run certificates and the TLS bridge.
> - **Admin / ADDR (079):** supply the second engine address required by DSP's provider runs.
> - **Admin / PSD:** serve package-source/https fetching, archive checks and scheme admission; V3/V4.
> - **Admin / SEC-D:** read/deliver secrets and qualify second-store recovery; V1/V5.
> - **Admin / FLT-D:** qualify unchanged caller/filter chains with all providers swapped; V6.
> - **Admin / 050:** finish independent homes, Agent hooks and sign-in packages.
> - **Admin / 042:** supply explicit lift and inventory for replacement/repair.
> - **Admin / 047:** supply pending actions.
> - **Admin / 048a-1:** validate sign-in Types and detect input-required results.
> - **Admin / 048a-2:** implement sign-in actions and SignInUndone.
> - **Admin / 048b-relay:** relay input-required results.
> - **Admin / 048b-join:** join shell relay and sign-in.
> - **Admin / 051b:** supply pending/sign-in/answer commands.
> - **Admin / 071:** split conformance pieces before safety additions.
> - **Admin / SV1:** provide the safety field before writing new safety vectors.
> - **Admin / 077:** write fenced reconciliation before relying on repaired unknown effects.
> - **Admin / 081:** state single-run safety and replacement overlap before qualification.
> - **Admin / 082:** state serialized first installs and storage-loss handling before qualification.
> - **Admin / B2-S:** write the driven-Agent secret restriction that S2 replays need.
> - **Admin / B2-C:** implement that restriction.
> - **Admin / B2-H:** serve that behavior in the running shell.
> - **Admin decision (b), 2026-11-11:** P3's pairs are the packaged local providers and the second
>   providers; the built-ins stay S10's first configuration, with no adapter in A63-H.
> - **Admin question A1:** approve the six vectors' placements.
> - **Admin question A2:** settle D4's reading before real-client replay scheduling.
> - **Admin question A3:** sequence DSP/PSD/SEC-D/FLT-D merges and the selected second store.
> - **Owner:** no new choice; accepted first implementations and twelve replays stay fixed.

## Status

Written on 2026-11-11 by work order 119, against `main` at
`31acd7378d9134a205208ec9491a01d2ea459da0`. A plan for the lane coordinators and the admin:
it binds nothing, and changes nothing the owner accepted in [README.md](README.md).
Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched.

## What `main` has now

Source inspection supplies this inventory; no new provider, contract suite or scenario has run.

| README need or gap | Present evidence at the base; remaining work |
| --- | --- |
| Accepted S1/S2 tasks and real Agent packages | [FIRST_BUILD tasks](../../docs/implementation/FIRST_BUILD.md#the-tasks) and [S2 tasks](../013-s2-shared-login/README.md#the-tasks) exist. [agent-codex](../../packages/agent-codex/conduit.yaml):3–6 is 0.1.1 / `.v1`, not 050's homes/hooks. S2 also needs SEC-D, S2-F, S2-L, FLT-D and batch 2's driven-Agent restriction. |
| First runtime and package source | [CONFORMANCE.md](../../engine/CONFORMANCE.md):24–25, 36–40 and 235–245 documents built-in process execution and file admission; [shell lib.rs](../../engine/crates/shell/src/lib.rs):381–405 reads file bytes. These are the accepted first implementations, under ruling 46.5. Under the admin's decision (b), packaged A63-RP/PS are P3's runtime and package-source partners, not replacements in the replays. |
| Second stores and shared service | No provider crates yet in [packages](../../packages/). Need A63-KC plus S2's service for the first configuration; A63-1P after A73, else A63-FS, for the second. A22 stays open, but logged-in-session keychain reads suffice; no daemon gate. |
| Runtime dispatch and Docker | [Executor::provider_call](../../engine/crates/shell/src/lib.rs):81–83 still defaults to Missing; [CONFORMANCE.md](../../engine/CONFORMANCE.md):40 says runtime execution is unserved. DSP/ADDR and A63-DR are needed, including gated runs and container home/browser-callback preparation. Core provider intents alone are insufficient. |
| Package-source dispatch, archive check and P5 | DL7 at CONFORMANCE.md:55 leaves providers and https unfinished. [archives.rs](../../engine/crates/core/src/archives.rs):184 already checks PKG-026. PRV-013/016 in [providers.yaml](../../spec/promises/providers.yaml):376–400 and 481–502 require admission claims/immutable schemes; PSD must qualify actual admission, selection and no fetch on rejection, not just transport. |
| P3: identical contract vectors on two tools | Existing [runtime-explicit](../../conformance/language/registration/runtime-explicit/case.yaml), [review-relative-provider](../../conformance/conventions/setup/review-relative-provider/case.yaml), [digest-mismatch](../../conformance/language/packages/digest-mismatch/case.yaml) are partial core evidence. Under the admin's decision (b), A63-H drives the packaged local providers (A63-RP, A63-PS), Docker, Git and both stores through the same suites over MCP, retaining per-tool reports; it does not drive the built-ins. |
| Rotation/recovery and D09 | [S2 D09](../013-s2-shared-login/README.md#measures) is accepted and reused exactly. No implemented store/read-delivery path establishes it here; SEC-D plus S2's prepared rotation/partial-restoration controls are required before V5/runs. |
| E06 and D15/D16 | Definitions and allowed edits are accepted in [README](README.md#measures), not executed fixtures/ledgers. S10-H1 prepares rejected Type and duplicate claimant; S10-H2 freezes the comparison and resets. |
| Owner setup | Accounts, twelve-replay matrix and 1Password-after-A73-or-file are answered in [README](README.md#open-for-the-owner). Owner still supplies Docker inventory, throwaway service/store scope and independent sign-ins. A73's desktop case needs its separate approval; no values enter evidence. |
| G4: safety format | Topic 13 still marks the portable field spec pending. [manifest.yaml](../../conformance/manifest.yaml):3 is inline; safety_rows are group coverage, not safety promises. SV1 after 071 supplies the format; no S10 vector is written first. |

041b already serves both cascades and applies REV-010 ([CONFORMANCE.md](../../engine/CONFORMANCE.md):125–129);
do not rebuild these. Declare-lost and control dispatch remain unserved and are not needed for E06:
the Type with a scheme-changing control must be rejected at admission, without invoking that control.
Order files 042, 049a1, 050a, 070–072, 075–082 and 038c are present, but their work is absent.
In particular 081/082's apparent merge subjects added order text only (`1054889`, `728517b`);
OPS-014 lacks 077's fence and PRV-007/PKG-025 lack the specified concurrency repairs.
This matches the admin's warning, rather than establishing those orders as merged builds.
No new dispatch-language gap: PRV-005/007/017 and the provider Operations already state it.
S2's missing driven-Agent prohibition needs B2-S; G4 needs SV1. Provider locators, supported
registries and transports belong to their contracts, not new language or an S10 spec order.

## The orders

All estimates precede pipeline fixes. Shared orders keep their owners; incremental vector scope
below is proposed for their fixing increments, not a separate lane order dependent on a core fix.
Repository checks mean `check_docs.py`, `check_rulings.py`, `set_names.py --check`,
`check_schemas.py`, `check_roadmap.py` under `tools/`, checker unittests and `git diff --check`.
Rust checks mean `cargo test --release`, `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check` in each changed crate. The paper order builds none.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S10-H1 | harness | E06 Type/claimant fixtures and admission/dispatch capture | PRV-013/016 | — | S, 200–350 lines | lane 10 | now |
| S10-H2 | harness | D15/D16 ledgers, resets, frozen tasks and ID/edit comparison | README P4, REC-002 | — | M, 350–550 lines | lane 10 | now |
| S10-V | vectors | V2 runtime-equivalence case, using existing core intents | PRV-007/021/022/025/028 | 071, SV1, A63-H, DSP, A63-DR, PIN | M, 300–500 lines | lane 10 | after dependencies |
| SEC-D | core | Shared delivery/recovery work, including S10 V1/V5 | SEC-006/007/010/014/019/023 | PIN, SV1, synthetic store/service fixtures | M, 350–550 added case lines; whole sized by 117 | admin | after dependencies |
| PSD | shell | Shared fetch/admission work, including S10 V3/V4 | PRV-013/016/017, PKG-026 | PIN, SV1, S10-H1, A63-GS; order 120's prerequisites | M, 350–550 added case lines; whole sized by 120 | admin | after dependencies |
| FLT-D | core | Shared caller/filter integration, including S10 V6 | ACC-001/008/017, IFC-020 | PIN, SV1, S10-H2's relationship fixtures | M, 200–350 added case lines; whole sized by 117 | admin | after dependencies |
| S10-O | owner | Docker/store/service setup and second-account sign-ins | PND-005/010/011 | 050, 051b, selected providers | owner step | owner | after dependencies |
| S10-R1 | run | Coordinator: five configurations, S1 and S2 in each; E06 | README P1–P5, M0–M3 | all above, P3 reports, B2-S/C/H, 077/081/082, A2 | ten replays, about 1 hour each | lane 10 | after dependencies and admin reading |
| S10-R2 | run | Fresh Codex author: all-swapped S1 and S2; E06 | same | S10-R1 | two replays, about 1 hour each | lane 10 | after S10-R1 |

**S10-H1.** Changes only Rust harness/fixture files under this experiment, with lockfile, pin and
ignored output. Prepares a package-source Type whose control changes `/schemes` (plus nested/root
overlap cases), and a second live claimant, using synthetic archives and counting fetch attempts.
No engine/provider implementation or vectors. Checks: fixture schema validity, expected admission
locations, offline harness tests/clippy/fmt and repository checks. Actual E06 waits for PSD.

**S10-H2.** Changes this experiment's Rust comparison harness and evidence templates only.
Freezes S1 T1–T7 and S2 T1–T7, contracts, artifacts, data, prompts, results and intended relationships.
D15 inventories all six first/second implementations: own code, delegation, persistent state/owner,
setup, custody and restart/stop obligations. D16 lists every permitted edit, ID/replacement/Conduit
map, copied state, panel action, downtime and access-loss interval; forbidden edits fail comparison.
Resets per configuration, especially a fresh install cache. Reuses S2's exact D09 controls/counting.
No vectors/engine edits. Checks: deterministic reset/comparison tests and repository checks.

**S10-V.** Changes only V2 `s10-runtime-equivalence` and its new private S10-runtime index under
`conformance/`, with safety PRV-025 in SV1's field; existing core intents/completions suffice.
Checks same run/stop/inspect/identity behavior; A63-H supplies actual process/Docker observations,
including Docker gate refusal, before this case is counted as scenario evidence. A supplied gate
completion alone proves no real gate. No runner/shared index/core edits; if core work is needed,
admin absorbs this case in that fixing order. Checks: engine release vectors, delta, repository checks.

**SEC-D contribution.** Owns V1 `s10-secret-store-equivalence` (safety SEC-007) and V5
`s10-swapped-rotation-recovery` (safety SEC-019), including partial restoration and nondisclosure.
Change only its delivery/recovery scope and S10 case files/private pieces; 117 sizes/splits the whole.
Hot core files: `requests.rs`, `lifecycle.rs`, `access.rs`; shell `runner.rs`, `lib.rs`, potentially
`server.rs`; runner `lib.rs`/manifest pieces if extending inputs. No new store or copied Agent login.
Checks: engine release tests/clippy/fmt, delta, experiment 011 if core/loader changes, repository checks.

**PSD contribution.** Owns V3 `s10-package-source-equivalence` (safety PKG-026) and V4
`s10-scheme-claim-admission` (safety PRV-013), with PRV-016 control rejection before registration.
Reject duplicate live claims before fetch; preserve first claim and its later working fetch.
Hot files: shell `lib.rs`, `runner.rs`; core `registration.rs`, `requests.rs` if admission needs fixing;
runner `lib.rs`/manifest pieces if required; archives/packages modules for digest admission.
No mutable schemes or changed operation contracts. 120 sizes/splits transport and admission if above L.
Checks: engine release tests/clippy/fmt, digest/refusal/no-dispatch cases, delta, repository checks
and experiment 011's release tests if core/loader changes.

**FLT-D contribution.** Owns V6 `s10-all-providers-swapped` (safety ACC-001), with unchanged
caller/filter chain and forbidden calls; actual full task replays remain S10-R1/R2.
Hot core files: `access.rs`, `requests.rs`, `registration.rs`; shell `server.rs`, `runner.rs`, `lib.rs`;
runner `lib.rs` and shared manifest pieces only if necessary. No provider-specific access branch,
weakened filter, `identity.rs` or CLI table change. Checks as SEC-D. Keep with its fixing increment.

**S10-O.** Owner installs/inventories Docker, supplies throwaway service and selected store scope,
and signs every replacement home into its labelled second subscription through Conduit.
1Password waits for A73 and its required approval, else use the accepted file-store fallback.
No vectors. Checks: exact versions/nonsecret setup and account/panel observations recorded apart.

**S10-R1 / S10-R2.** Coordinator with owner performs the twelve accepted replays: R1 first,
secret-only, Docker-only, Git-only and all-swapped configurations × S1/S2; R2 all-swapped × S1/S2.
These are counted runs, not Codex build orders; book owner time per replay, not one hour for all.
E06 once per author; D09 on second-store-only and all-swapped S2 replays. Retain the no-secret S1
replay in secret-only too. Checks: six vectors, P3 tool reports, task oracle, access/attribution and
complete M0–M3/D15/D16/D09 ledgers. Preserve failures; no live fixes inside a counted attempt.

## Shared with other plans

- Order 120 and the admin's decision (b) of 2026-11-11: `A63-H` runs each contract's suite over
  MCP on actual tools. For P3, the runtime pair is `A63-RP` and `A63-DR`, the package-source pair
  `A63-PS` and `A63-GS`, and the secret pair `A63-KC` and the second store; each keeps its own
  report with tool and version. A63-H drives no built-in: the engine's runner and file fetching stay
  S10's first configuration in the replays, as the owner accepted, and their behavior is evidenced
  by the replays and the core's vectors, not by the provider suites. Never count a package's
  report as a built-in's. `A63-MP` and S3's second machine implementation supply machine-contract
  evidence outside S10's three swaps.
- `A63-KC`: first store in the logged-in session. `A63-1P` after A73, or `A63-FS`: second store.
  `A63-DR`: Docker package items, home/state, callback and identity/engine-URL translation prepared
  before D16 freeze. A relative package-contained command is not portable to Docker (PRV-007).
  `A63-GS`: same pinned archive bytes through Git and fresh cache; `PSD`: dispatch and scheme claims.
- `DSP`/`ADDR`: 080/079's discovery-first calls, certificates, TLS-before-send bridge and second
  engine address. Hot shell files `runner.rs`, `identity.rs`, `lib.rs`, `server.rs` as scoped there;
  no M2-1–M2-4 answer is selected. SSH-R/SSH-M/AGT and enrollment are not S10 prerequisites.
- Order 117: `SEC-D`, `FLT-D`, `S2-F`, `S2-L`, accepted D01/D05/D09 and S2's prepared faults.
  S10 replays S2's tasks/contracts; it does not redefine those measures. `SCR` belongs to 118;
  it may support exact protocol checks but cannot silently replace accepted real-Agent replays.
  Shared-table estimates/dependencies describe S10 contributions only. Portable cases use synthetic
  completions; real providers join at runs, so S2's base increments do not wait on S10 providers.
- `042`, `047`, `048a-1/2`, `048b-relay/join`, `050` with 050a/076, `051b`: repair and sign-in;
  shell hot files `server.rs`, `runner.rs`, `lib.rs`, CLI tables in 051b. Batch 1 completes through
  053 → `PIN` before scenario core merges. `B2-S/C/H` supplies S2 G1 after the owner's rerun judgment.
- `077`, `081`, `082` are still work to do, not satisfied prerequisites. `071` → `SV1`, then
  `SV4` governs all six case fields. SV2/SV3 cover existing/S1 safety, not these additions.
  Sister plans and `docs/implementation/PROVIDERS.md` are absent; shared scopes need reconciliation.

## The order of the work

Dispatch S10-H1, then S10-H2 now; both preserve the rerun's starting snapshot/tools/tasks.
Critical join: batch 1 → 053 → PIN → owner judges S1 bar → B2-S → B2-C → B2-H; alongside,
ADDR → DSP plus A63-DR; PSD plus A63-GS; A63-KC and selected second store → SEC-D; then FLT-D.
071 → SV1 and A63-H qualify contract reports, with 077/081/082 merged before dependent evidence.
S10-H2's final freeze joins S2 readiness, all six vectors, provider reports, S10-O and A2 → R1 → R2.
These joins, not fabricated parallel completion dates, determine the critical path; if A73 is late,
the accepted file fallback avoids that probe wait. No A22 daemon wait. Side-branch DSP/ADDR merges
wait for PIN; admin sequences hot files and any starting-vector changes. No owner-held M2 choice is assumed.

## For the admin

- **P3 — decided:** the admin chose option (b) of order 120 on 2026-11-11: P3 is met by the
  packaged local providers and the second providers passing the same suites, and the accepted
  matrix keeps the built-ins as its first configuration. No built-in adapter is planned.
- **A1 — vector placement:** approve the six unique vector owners above with 117/120; their fixing
  increments absorb dependent cases and split scopes above L. Fresh skeptic first.
- **A2 — D4:** retain S1/S2 real Agents in S10's explicit twelve-replay matrix, or read D4 as
  limiting these replays too. **Recommendation:** confirm the explicit replay/account reading;
  hold run dispatch until resolved. Any changed configuration or client requirement goes to owner.
- **A3 — merge/store order:** stack all engine work, or rebase separate post-pin increments.
  **Recommendation:** separate DSP, PSD, SEC-D, FLT-D increments with admin-sequenced hot files;
  freeze 1Password only with A73 evidence, else the already accepted file fallback. No new owner
  fallback approval is invented; record which accepted branch was used before comparison.

## For the owner

None newly raised. No change to the first implementations, five configurations, second accounts
or twelve replays is proposed. If A1/A2 cannot be resolved within them, the admin brings the
specific change back; this plan selects none of the held M2 choices.

## Limits

No result or suite pass is claimed. Static admission cases and bounded G5 models are not real-tool
equivalence evidence. S10 supplies no live migration, daemon, confinement or machine-fault result.
All paths in future evidence use `<lab>`; no secrets, machine/network/user names or sign-in codes.
