# S10: order-level plan

> **In short**
> - **Admin / DSP (080):** serve runtime calls, discovery, run certificates and the TLS bridge.
> - **Admin / ADDR (079):** supply the second engine address required by DSP's provider runs.
> - **Admin / PSD1a:** core Type/scheme admission with V4.
> - **Admin / PSD1b:** core source/serialized-install/archive decisions with V3 safety.
> - **Admin / PSD2:** package-source MCP dispatch through DSP.
> - **Admin / PSD3:** unclaimed HTTPS fetches after PSD1/DSP TLS.
> - **Admin / SEC-D:** keep V1’s SEC-007 safety refusal in the engine.
> - **Admin / S10-EV:** V5/V6 engine composite after SEC-D, FLT-D and PSD1.
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
> - **Owner:** confirm D4’s reading for counted replays; M2-3 holds Docker T3/T5 and R2.
> - **Admin question A3:** sequence DSP/PSD/SEC-D/FLT-D merges and the selected second store.
> - **Admin A4:** provisional reading of A73’s “or else”; confirm before choosing fallback.

## Status

Written on 2026-11-11 by work order 119, against `main` at
`31acd7378d9134a205208ec9491a01d2ea459da0`. A plan for the lane coordinators and the admin:
it binds nothing, and changes nothing the owner accepted in [README.md](README.md).
Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched.

Revised on 2026-11-11 by work order 121, against `1b717c408291b632d285a6bf0a782596478cc770`.

## What `main` has now

Source inspection supplies this inventory; no new provider, contract suite or scenario has run.

| README need or gap | Present evidence at the base; remaining work |
| --- | --- |
| Accepted S1/S2 tasks and real Agent packages | [FIRST_BUILD tasks](../../docs/implementation/FIRST_BUILD.md#the-tasks) and [S2 tasks](../013-s2-shared-login/README.md#the-tasks) exist. [agent-codex](../../packages/agent-codex/conduit.yaml):3–6 is 0.1.1 / `.v1`, not 050's homes/hooks. S2 also needs SEC-D, S2-F, S2-L, FLT-D and batch 2's driven-Agent restriction. |
| First runtime and package source | [CONFORMANCE.md](../../engine/CONFORMANCE.md):24–25, 36–40 and 235–245 documents built-in process execution and file admission; [shell lib.rs](../../engine/crates/shell/src/lib.rs):381–405 reads file bytes. These are the accepted first implementations, under ruling 46.5. Under the admin's decision (b), packaged A63-RP/PS are P3's runtime and package-source partners, not replacements in the replays. |
| Second stores and shared service | No provider crates yet in [packages](../../packages/). Need A63-KC plus S2's service for the first configuration; A63-1P after A73, else A63-FS, for the second. A22 stays open, but logged-in-session keychain reads suffice; no daemon gate. |
| Runtime dispatch and Docker | [Executor::provider_call](../../engine/crates/shell/src/lib.rs):81–83 still defaults to Missing; [CONFORMANCE.md](../../engine/CONFORMANCE.md):40 says runtime execution is unserved. DSP/ADDR and A63-DR are needed, including gated runs and container home/browser-callback preparation. Core provider intents alone are insufficient. |
| Package-source dispatch, archive check and P5 | DL7 at CONFORMANCE.md:55 leaves providers and https unfinished. [archives.rs](../../engine/crates/core/src/archives.rs):184 already checks PKG-026. PRV-013/016 in [providers.yaml](../../spec/promises/providers.yaml):376–400 and 487–502 require admission claims/immutable schemes; PSD must qualify actual admission, selection and no fetch on rejection, not just transport. |
| P3: identical contract vectors on two tools | Existing [runtime-explicit](../../conformance/language/registration/runtime-explicit/case.yaml), [review-relative-provider](../../conformance/conventions/setup/review-relative-provider/case.yaml), [digest-mismatch](../../conformance/language/packages/digest-mismatch/case.yaml) are partial core evidence. Under the admin's decision (b), A63-H drives the packaged local providers (A63-RP, A63-PS), Docker, Git and both stores through the same suites over MCP, retaining per-tool reports; it does not drive the built-ins. |
| Rotation/recovery and D09 | [S2 D09](../013-s2-shared-login/README.md#measures) is accepted and reused exactly. No implemented store/read-delivery path establishes it here; SEC-D plus S2’s preparation and S10-H2b’s second-store rotation controls are needed for runs; V5 uses scripted completions. |
| E06 and D15/D16 | Definitions and allowed edits are accepted in [README](README.md#measures), not executed fixtures/ledgers. S10-H1 prepares rejected Type and duplicate claimant; S10-H2 freezes the comparison and resets. |
| Owner setup | Accounts, twelve-replay matrix and 1Password-after-A73-or-file are answered in [README](README.md#open-for-the-owner). Owner still supplies Docker inventory, throwaway service/store scope and independent sign-ins. A73's desktop case needs its separate approval; no values enter evidence. |
| G4: safety format | Topic 13 still marks the portable field spec pending. [manifest.yaml](../../conformance/manifest.yaml):3 is inline; safety_rows are group coverage, not safety promises. SV1 after 071 supplies the format; no S10 vector is written first. |

041b already serves both cascades and applies REV-010 ([CONFORMANCE.md](../../engine/CONFORMANCE.md):125–129);
do not rebuild these. Declare-lost and control dispatch remain unserved and are not needed for E06:
the Type with a scheme-changing control must be rejected at admission, without invoking that control.
Order files 042, 049a1, 050a, 070–072, 076–082 and 038c are present, but their work is absent.
In particular 081/082's apparent merge subjects added order text only (`1054889`, `728517b`);
OPS-014 lacks 077's fence and PRV-007/PKG-025 lack the specified concurrency repairs.
This matches the admin's warning, rather than establishing those orders as merged builds.
No new dispatch-language gap: PRV-005/007/017 and the provider Operations already state it.
S2's missing driven-Agent prohibition needs B2-S; G4 needs SV1. Provider locators, supported
registries and transports belong to their contracts, not new language or an S10 spec order.

Order 075 is merged (`55cf747`): an Agent Type states the reach of its client’s own tools.

## The orders

A scenario vector goes in the core or shell order that makes it pass if that order stays at most L;
otherwise an admin vectors order immediately follows it and depends only on the core and shell orders
its cases need, never on a lane package, owner step or run. A provider-contract case goes in lane 8’s
A63-VR, A63-VM, A63-VP or A63-VS suite in [the provider plan](../../docs/implementation/PROVIDERS.md).

All estimates precede pipeline fixes. Shared orders keep their owners; incremental vector scope
below is proposed for their fixing increments, not a separate lane order dependent on a core fix.
Repository checks mean `check_docs.py`, `check_rulings.py`, `set_names.py --check`,
`check_schemas.py`, `check_roadmap.py` under `tools/`, checker unittests and `git diff --check`.
Rust checks mean `cargo test --release`, `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check` in each changed crate. The paper order builds none.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S10-H1 | harness | E06 Type/claimant fixtures and admission/dispatch capture | PRV-013/016 | — | S, 200–350 lines | lane 10 | now |
| S10-H2a | harness | D15/D16 ledgers and comparison tool | README P4, REC-002 | — | M, 300–450 lines | lane 10 | now |
| S10-H2b | harness | Final freeze, per-configuration resets and second-store D09 controls | README P4, SEC-019 | S10-H2a, S2-H, S2-R1, provider packages | M, 350–550 lines | lane 10 | after S2 evidence and providers |
| S10-EV | vectors | V5/V6 engine composite, no provider-package dependency | SEC-019, ACC-001 | SEC-D1/D2/D3a/D3b, FLT-D1/D2, PSD1a/PSD1b only | M, 400–600 lines | admin | after required engine work |
| SEC-D1 | core | V1 safety: engine refuses secret access under SEC-007 | SEC-007 | S2 plan’s engine prerequisites | included in D1 at most L; overflow immediately after | admin | after engine prerequisites |
| PSD1a | core | V4 source Type/scheme admission | PRV-013/016 | provider plan’s engine prerequisites | M, 400–600 lines including cases | admin | after engine prerequisites |
| PSD1b | core | V3 archive safety and serialized source install | PRV-017, PKG-026 | PSD1a, 082 | M, 400–600 lines including cases | admin | after engine prerequisites |
| FLT-D | core | Caller/filter engine work; no H2 dependency | ACC-001/008/017, IFC-020 | S2 plan’s engine prerequisites | D1 sized after D0, at most L; D2 M | admin | after engine prerequisites |
| S10-A73 | probe | 1Password service-account and desktop-session observations | A73 | first A63 providers started; owner approval before desktop case | M, 300–500 lines plus observations | lane 11 | beside first providers; desktop held |
| S10-O | owner | Docker, HTTPS Git repository/credential, stores/service and sign-ins | PND-005/010/011 | 050, 051b, selected providers | owner step | owner | after dependencies |
| S10-R1 | run | Coordinator: five configurations × S1/S2; E06 | README P1–P5, M0–M3 | run gate/P3 orders below, B2-S/C/H, 077/081/082; M2-3 on T3/T5 only | ten replays, about 1 hour each | lane 10 | after dependencies; the owner confirms D4's reading |
| S10-R2 | run | Fresh author all-swapped S1/S2; E06 | same | S10-R1, M2-3 | two replays, about 1 hour each | lane 10 | after S10-R1; the owner confirms D4's reading |

**S10-H1.** Changes only Rust harness/fixture files under this experiment, with lockfile, pin and
ignored output. Prepares a package-source Type whose control changes `/schemes` (plus nested/root
overlap cases), and a second live claimant, using synthetic archives and counting fetch attempts.
No engine/provider implementation or vectors. Checks: fixture schema validity, expected admission
locations, offline harness tests/clippy/fmt and repository checks. Actual E06 waits for PSD.

**S10-H2a / H2b.** H2a starts now under this experiment with D15's own code/delegation/
persistent state/owner setup fields and D16's permitted edits, IDs/replacements/Conduits,
copied state/actions/downtime ledger and comparison tool. No final freeze or provider-dependent reset.
H2b follows S2-H, S2-R1 and provider packages: freeze S1/S2 T1–T7, contracts/artifacts/data/
prompts/results/relationships; implement clean per-configuration caches/resets and actual second-store
rotation/partial-restoration controls. D09 is defined on keychain in S2; second-store controls are
H2b work, not assumed from S2. No vectors/engine edits; reset/comparison tests and repository checks.

**S10 vector placement.** Drop S10-V. V2 `s10-runtime-equivalence` (safety PRV-025) is a
provider-contract case in A63-VR, run on the runtime pair, not a synthetic core gate completion.
V1 `s10-secret-store-equivalence` keeps safety SEC-007 in SEC-D1: engine refusal of access,
with deterministic store completions/scripted engine servers; A63-VS shows pair equivalence only.
V3 `s10-package-source-equivalence` keeps safety PKG-026 in PSD1b's engine archive check;
A63-VP shows pair equivalence only. V4 `s10-scheme-claim-admission` stays in PSD1a, safety
PRV-013, including PRV-016 Type rejection/duplicate claims before fetching. No safety promise changes;
a proposed change goes to the owner. PSD1 uses its own fixtures, never S10-H1 or A63-GS.
SEC-D/PSD1 carry their safety cases while at most L, otherwise an immediately following admin
vectors order depends only on needed engine work. All use SV1's field, unchanged promises.

**S10-EV.** Admin conformance/index cases only after SEC-D1/D2/D3a/D3b, FLT-D1/D2 and PSD1a/PSD1b: V5
`s10-swapped-rotation-recovery` (safety SEC-019) and V6 `s10-all-providers-swapped`
(safety ACC-001). Synthetic completions test the engine's rotation/refusal/chain, not real provider
behavior. No H2/package/owner/run dependency; no engine/runner fix or safety reassignment here.
If a case needs a missing engine behavior, place it with that fixing order under the head rule.
Checks engine release vectors/delta and repository checks. FLT-D never waits on S10-H2.

**Shared hot files.** SEC-D core requests.rs/access.rs/lifecycle.rs; shell runner.rs/lib.rs/identity.rs.
PSD1 is core/loader admission, including registration.rs/requests.rs, loader/packages Type checks;
PSD2/3 are shell lib.rs/runner.rs. FLT-D core requests.rs/access.rs/registration.rs, shell
server.rs/runner.rs/lib.rs; runner lib.rs/manifest pieces only where needed. Internal scripted
engine servers test all engine orders; harnesses qualify actual packages. See S4's merge sequence.

**S10-A73.** Lane 11's probe in `experiments/025-1password/`, next unused after A81's planned
024 and excluding reserved 022/used 023. The creating order adds its experiment-index row and updates A73’s register row,
pre-registers measures and writes dated results, with Rust lockfile/toolchain and ignored output.
Follow experiment 005's safety rules on throwaway account/vault; bound children and keep values
out of evidence. Observe service-account and desktop-app session separately; the desktop case
waits explicit owner approval. No provider implementation, engine change or passing claim here.

**S10-O.** Owner installs/inventories Docker, supplies throwaway service and selected store scope,
prepares a disposable HTTPS Git repository and its credential (A63-GS claims git+https only),
and signs every replacement home into its labelled second subscription through Conduit.
1Password waits for A73 and its approvals. The timing interpretation of “or else” is
provisional under admin A4, not automatic permission to choose file when the probe is late.
No vectors. Checks: exact versions/nonsecret setup and account/panel observations recorded apart.

**S10-R1 / S10-R2.** Coordinator with owner performs the twelve accepted replays: R1 first,
secret-only, Docker-only, Git-only and all-swapped configurations × S1/S2; R2 all-swapped × S1/S2.
These are counted runs, not Codex build orders; book owner time per replay, not one hour for all.
Docker-only T3 and all-swapped T5, plus R2, wait **M2-3**: every provider run receives
the engine’s HTTPS address under DSP/079. Local Docker needs its TLS termination too.
E06 once per author; D09 on second-store-only and all-swapped S2 replays. Retain the no-secret S1
replay in secret-only too. Checks: six vectors, P3 tool reports, task oracle, access/attribution and
complete M0–M3/D15/D16/D09 ledgers. Preserve failures; no live fixes inside a counted attempt.

R1's run gate names its P3 inputs: A63-RK1/RK2, A63-H1, A63-H2a/H2b,
A63-RP1/RP2, A63-DR1/DR2a/DR2b, A63-PS, A63-KC, A63-1P or A63-FS, A63-GS,
A63-VR (only repeated-run cases wait 081), A63-VP and A63-VS. Also require S2-H and
SCR1/SCR2, S10-H1/H2a/H2b/O/EV, all six placed cases, DSP/ADDR/PSD/SEC-D/FLT-D1/D2,
050/sign-in and S2's other prerequisites; M2-3 joins only the provider-run configurations.

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
  before D16 freeze. 050’s owner supplies container-runnable Agent items, S2-L supplies its
  service items, and [S3-S1ITEM](../014-s3-two-machines/plan.md) is proposed to admin for
  S1 example/fixture server items. A relative package-contained command is not portable to Docker (PRV-007).
  `A63-GS`: same pinned archive bytes through Git and fresh cache; `PSD`: dispatch and scheme claims.
- `DSP`/`ADDR`: 080/079's discovery-first calls, certificates, TLS-before-send bridge and second
  engine address. Hot shell files `runner.rs`, `identity.rs`, `lib.rs`, `server.rs` as scoped there;
  M2-3 remains a held wait on T3/T5/R2; no answer is selected. SSH-R/SSH-M/AGT and enrollment are not S10 prerequisites.
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
  See [S2](../013-s2-shared-login/plan.md), [S4](../015-s4-partial-setup/plan.md)
  and [PROVIDERS](../../docs/implementation/PROVIDERS.md) for shared scopes/splits.

## The order of the work

Dispatch H1 and H2a now; S10-A73 starts beside the first providers on lane 11, with owner
approval before its desktop case. H2b waits S2-H/S2-R1 and packages before freezing/resetting.
Batch 1 → 053 → PIN → owner judges S1 → B2-S/C/H joins ADDR/DSP, PSD1a/b/2/3, SEC-D/FLT-D,
provider suites/packages, S3-S1ITEM/S2-L/050 container items, HTTPS Git setup and owner sign-ins.
071/SV1 gates safety cases; EV follows SEC-D, FLT-D1/D2 and PSD1 only (no S8-only FLT-D3); no engine order waits H1/H2/Git.
R1 joins the named P3 orders, S2-H/SCR and final H2b; R2 follows R1. Only counted runs wait:
the owner confirms D4's reading. M2-3 separately holds T3/T5 and R2's provider-run engine access.
The A73 “or else” timing interpretation remains provisional under admin A4; no automatic late-probe fallback.
Use [S4’s proposed hot-file merge order](../015-s4-partial-setup/plan.md#for-the-admin): PSD1a/b and SEC-D1/FLT-D1 are
admin-interleaved core work, DSP/PSD2/PSD3/SEC-D2/D3a/D3b/FLT-D2 are interleaved shell work.
No S10 core/shell order is added; EV uses manifest pieces after its engine prerequisites.

## For the admin

- **P3 — decided:** the admin chose option (b) of order 120 on 2026-11-11: P3 is met by the
  packaged local providers and the second providers passing the same suites, and the accepted
  matrix keeps the built-ins as its first configuration. No built-in adapter is planned.
- **A1 — vector placement:** approve the six unique vector owners above with 117/120; their fixing
  increments absorb dependent cases and split scopes above L. Fresh skeptic first.
- **A2 — D4:** Planned on the admin's reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9 only. Provisional until the owner confirms.
- **A3 — merge/store order:** use S4's single proposed hot-file sequence and name the frozen
  second store before H2b comparison. **Recommendation:** keep all engine dependencies free
  of lane packages; use the named P3 reports rather than a generic “P3 reports” gate.
- **A4 — A73 “or else”, provisional:** does the owner's fallback permit file whenever A73 is
  late, or only after A73 reports 1Password unsuitable/unavailable? **Recommendation:** confirm
  the former for scheduling, with the choice recorded before freeze. This is the plan's reading
  for admin confirmation. Provisional until the owner confirms. No changed ruling
  or automatic late-probe fallback is recorded here.

## For the owner

Counted R1/R2 wait: the owner confirms D4's reading. Only those runs wait; all independent
preparation proceeds. **M2-3** is a held choice for Docker-only T3, all-swapped T5 and R2 to
reach the engine's HTTPS address through runtime providers; no TLS termination answer here.
S10-A73's desktop-app case waits the owner's explicit approval under A73's register row.
Accepted configurations, second accounts and twelve replays stand. Any changed safety promise,
fallback meaning or client requirement returns to the owner, not a worker's assumption.

## Limits

No result or suite pass is claimed. Static admission cases and bounded G5 models are not real-tool
equivalence evidence. S10 supplies no live migration, daemon, confinement or machine-fault result.
All paths in future evidence use `<lab>`; no secrets, machine/network/user names or sign-in codes.
