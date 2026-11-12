# S2: order-level plan

> **In short**
> - Admin proposal SEC-D1: secret-read decisions, attribution and fail-closed records in the core.
> - Admin proposal SEC-D2: private secret reads and delivery to runs in the shell.
> - Admin proposal SEC-D3: secret-bearing remote connections and recovery in the shell.
> - Admin proposal FLT-D1: qualify and complete the core's filters, repeats and onward checks.
> - Admin proposal FLT-D2: qualify their delivery through the shell to real implementations.
> - Admin proposal S2-V: register rotation/repair and cached-tool vectors in the coverage index.
> - Admin A1: accept the secret/core/shell split and merge sequence?
> - Admin A2: keep engine integration after PIN, or authorize another side branch?
> - Admin A3: include E02's extra controls in SCR, or keep a separate driver?
> - Owner: no new question; O1–O5's accepted answers stand.

## Status

Written on 2026-11-11 by work order 117, against `main` at `31acd7378d9134a205208ec9491a01d2ea459da0`.
A plan for the lane coordinators and the admin: it binds nothing, and changes nothing the owner
accepted in [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched.

## What `main` has now

Evidence below is inspection of this commit, not a new test result. The log's entries for orders
070–072, 075–082 and 038c add order files, not their implementations. 050a/076's work is absent:
[Codex's manifest](../../packages/agent-codex/conduit.yaml), lines 3–6, still exports 0.1.1 and `.v1`.
042 and 049a-1 are absent too; [State](../../engine/crates/core/src/lib.rs), lines 65–81, has no
tombstone, pending-action or machine maps. This agrees with the admin's warning about order files.

| README prerequisite or gap | Present evidence and remaining work |
| --- | --- |
| Secret reads/delivery and record failure | [lifecycle.rs](../../engine/crates/core/src/lifecycle.rs), lines 786–803, makes ordinary run bindings; [lib.rs](../../engine/crates/shell/src/lib.rs), lines 68–87, defaults provider/connection execution to Missing. SEC-010/012/015/016's read workflow is missing: SEC-D1–3. Literal redaction already exists in requests.rs, lines 125–145; it is not secret delivery. |
| Filtered repeat dispatch, OPS-024 | [requests.rs](../../engine/crates/core/src/requests.rs), lines 792–838, redispatches a successful filtered read. Keep this; FLT-D1 adds deadline/current-authorization evidence rather than replacing the repeat rules. 077 remains pending, including its filtered plain-content clarification. |
| CON-022 support check | [registration.rs](../../engine/crates/core/src/registration.rs), lines 2335–2371 and 2426–2451, checks every operation's selected item's supports and rejects plain/engine filter targets. Already met statically; qualify the S2 contract in FLT-D1. |
| Identity, carried filters, dependent calls | requests.rs, lines 569–608, emits caller/invocation/filter metadata; [access.rs](../../engine/crates/core/src/access.rs), lines 382–425, derives the serving chain and rechecks closure/retirement. Existing slices include [broker](../../conformance/conventions/records/broker/case.yaml). FLT-D adds end-to-end evidence and only demonstrated missing paths. |
| Replacement and explicit repair | [CONFORMANCE.md](../../engine/CONFORMANCE.md), lines 123–130, serves register-replacement, replace, retire and both cascades. 041b's REV-010 reconciliation and DL1 ordering are present. Author-facing repair still needs 042, 051a, 052a-1/2 and 052b; do not schedule those again. |
| Real clients' tool changes | CONFORMANCE.md, lines 91–109, has private notifications, current per-call checks and explicit visibility limits; [current-tools](../../conformance/conventions/access/current-tools/case.yaml) exists. The `.v1` wrapper reports later catalogs as unverified: [Codex README](../../packages/agent-codex/README.md), lines 49–58. V6 and T4 must measure wire, model evidence and next action separately. |
| Separate homes and sign-in | PRV-029 state directories exist (CONFORMANCE.md, line 38); account homes/hooks do not. requests.rs, lines 902–917, still stubs sign-in/answer. All tasks need 050; T1 specifically needs 048a-1/2, 048b-relay/join and 051b. Each later fresh run repeats T1; T5/T6 use the same independently signed-in homes, never copied logins. |
| Complete packages and isolated runtime | Only the two Agent crates exist under packages/. S2-F, S2-L, A63-KC and A63-DR remain. DL16 executor coverage (CONFORMANCE.md, line 40) leaves provider calls/remote connections unserved: DSP must reach main before Docker runs; SEC-D3 serves secret-bearing remote items beyond DSP's provider-run bridge. |
| Owner/store prerequisites | O2 uses primary accounts; O3 explicit issue IDs plus deadline/refusal; O4 keychain alone; O5 logged-in session only. A22 stays open and does not block A63-KC. No A73/1Password dependency. |
| G1: driven-Agent secrets | Still a language gap: SEC-008 allows Agent consumers. B2-S writes ruling 61 B13, B2-C enforces both grant directions, B2-H shows the one-hop reach in inspection/plans. Gate the run on all three; neither Agent gets the service-secret Conduit. |
| G2: package contracts | Deliberate contract gap, not new language: S2-F fixes the full issue/deadline contract; S2-L implements it; 050 states own-tool behavior; A63-KC states locator/session/cache/upgrade behavior. Freeze these before qualification. |
| G3: safety field | Still absent from [case.schema.json](../../conformance/case.schema.json); SV1 follows 071. Every safety case below waits for SV1 and follows SV4. No invented field or premature tagging. |

## The orders

Sizes are estimates before pipeline fixes; line counts include tests, excluding generated archives.
Shared order SEC-D means all three parts; FLT-D means both; S2-L means both implementations orders.
G means the seven paper checks of order 117, including the Python unit suite. E adds engine fmt,
clippy with warnings denied, release tests and vector-delta review against the dispatch base;
core/loader edits also run experiment 011's release tests. P adds fmt/clippy/release tests in
each changed Rust crate, a pinned toolchain, Cargo.lock, ignored output and package admission.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S2-F | server | Ship O3's restriction contracts | CON-018, IFC-019 | — | S, 150–220 lines | lane 8 | now |
| S2-L1 | server | Local Linear stub and aware implementation | IFC-019/020, SEC-019 | S2-F | L, 550–800 lines | lane 8 | after S2-F |
| S2-L2 | server | Unchanged plain server qualification and small adapter | CON-022, IFC-007/019 | S2-L1 | L, 500–750 lines | lane 8 | after S2-L1 |
| SEC-D1 | core | Decide secret reads and record before dispatch; V4 | SEC-006–018, SEC-023/024 | PIN, 042, S0b, S2-L1, SV1 | L, 650–900 lines | admin | after dependencies |
| SEC-D2 | shell | Execute reads; deliver private values to runs | SEC-010–020, PRV-011/022/026 | SEC-D1, DSP | L, 650–900 lines | admin | after SEC-D1 and DSP on main |
| SEC-D3 | shell | Connect with headers; recover values after loss | SEC-013/014/018/023/024 | SEC-D2 | L, 550–800 lines | admin | after SEC-D2 |
| FLT-D1 | core | Complete/qualify filters and onward paths; V1/V2/V5 | ACC-017/034, IFC-019/020, CON-022, OPS-024 | PIN, S0b, 042, 077, S2-L1, SV1 | L, 500–750 lines | admin | after dependencies |
| FLT-D2 | shell | Verify exact filtered dispatch and dependent-call wire | ACC-017, IFC-019/020 | FLT-D1, DSP, S2-L | M, 350–500 lines | admin | after dependencies |
| S2-V | vectors | V3 and V6, including their index entries | SEC-018, ACC-001 | SEC-D, FLT-D, 050, 052b, SV1 | M, 300–450 lines | admin | after dependencies |
| S2-H | harness | Clock, authorization mutations and restoration barriers | OPS-024, ACC-034, SEC-014 | S2-L, SCR | L, 600–850 lines | lane 10 | after S2-L and SCR |
| S2-O | owner | Test keychain/item, narrow service key, account labels and runtime readiness | SEC-019, PRV-029 | — | about 1 hour presence | owner | after owner's availability |
| S2-R1 | run | Coordinator, local, T1–T7 | README P1–P6 | run gate below | about 1 hour; 45 min active owner proposed | lane 10 | after run gate |
| S2-R2 | run | Fresh Codex author, local, T1–T7 | README P1–P6 | S2-R1 | about 1 hour; same owner allowance | lane 10 | after S2-R1 |
| S2-R3 | run | Coordinator, isolated, T1–T7 | README P1–P6 | S2-R2, A63-DR | about 1 hour; same owner allowance | lane 10 | after S2-R2 and isolated readiness |
| S2-R4 | run | Fresh Codex author, isolated, T1–T7 | README P1–P6 | S2-R3 | about 1 hour; same owner allowance | lane 10 | after S2-R3 |

**S2-F.** New contracts-only package under `packages/linear-restrictions/`: manifest,
contracts, documentation and admission fixtures. State clock/deadline, issue IDs, changed
authorization data, indirect/returned content, onward calls and denial/state rules as package
behavior under O3. Change no core contract, service code or vectors. Checks G and admission.

**S2-L1.** New standalone Rust crate under `packages/linear/`, with the package files/tests
used by [agent-codex](../../packages/agent-codex/README.md). Build deterministic local stub
mode and aware mode, controllable authorization/clock and a dependent-call barrier; synthetic
secret markers only. No new filter semantics or engine edits; no portable vectors here. Checks G/P.

**S2-L2.** Same service crate's adapter and qualification fixtures; pin the unchanged plain
server separately, recording its supported operations and honest unsupported-filter verdict.
Qualify identical reads across all three, restriction cases across adapter/aware only. No
fork of the plain server or provider implementation; no portable vectors. Checks G/P, D01/D05 matrix.

**SEC-D1.** Core read workflow modules, lifecycle/requests/records integration and private
read-result boundary, plus conformance fixtures/assertions where needed. Hot: core requests.rs,
access.rs, lifecycle.rs; runner lib.rs and assigned manifest pieces; registration.rs only if
binding admission requires it. Persist descriptors/outcomes, never values; no store-specific logic.
Own V4 `s2-secret-read-fails-closed` (safety SEC-012), using S2-L's stub and record-unavailability
evidence. Test failed/unknown/locked reads, no fallback, attribution and no value in records. G/E.

**SEC-D2.** Shell executor, runner and private delivery helpers; hot shell lib.rs, runner.rs,
identity.rs; no server.rs, CLI tables, core files, runner lib.rs or manifest edits planned.
Deliver only to the selected run via its authenticated runtime channel after durable read
dispatch; discard partial values and keep original values fixed for an existing run. No raw
values in State, intents published as evidence, diagnostics or Agent results. No new V1–V6; G/E.

**SEC-D3.** Shell connection/recovery helpers and tests; hot runner.rs and lib.rs, no other
listed hot files. Implement currently unserved remote connections with URL-bound confidential
headers, and SEC-023's authorized reread after value loss; preserve caller attribution on
provider restart. No generic secret cache, automatic rotation or new V1–V6. Checks G/E.

**FLT-D1.** Core requests/access/registration and focused assertions; hot requests.rs,
access.rs, registration.rs, runner lib.rs and assigned manifest pieces, not lifecycle.rs.
Keep existing support/chain/repeat behavior; fix only demonstrated omissions against promises.
Own V1 `s2-filter-current-read` (IFC-019), V2 `s2-bound-allows-unfiltered` (ACC-006), and V5
`s2-dependent-call-after-close` (ACC-034); all use S2-L. Include static bound/support decisions
as vectors: [G5](../012-model-checking/G5.md) does not count static decisions as model proof. G/E.

**FLT-D2.** Shell dispatch/discovery tests with S2-L's adapter/aware server. Hot runner.rs,
server.rs and lib.rs; no identity.rs, core files, CLI tables or coverage index changes planned.
Check carried metadata, stale serving IDs, closure during a dependent call, and plain calls'
omission of Conduit keys. Complete only wire omissions found; never evaluate a package filter
in the engine. No new V1–V6. Checks G/E and request/reply-to-record correlation.

**S2-V.** Only conformance cases/fixtures and their manifest registration; admin ownership
because the index is shared. Own V3 `s2-rotate-replace-repair` (SEC-018) and V6
`s2-cached-tool-refused` (ACC-001). Both depend on S2-L; V3 includes partial restoration, V6
separates protocol refusal/tool failure. No runner or production code changes. Checks G, release vectors/delta.

**S2-H.** New Rust harness crate under `experiments/013-s2-shared-login/harness/`, with
Cargo.lock/toolchain/tests and ignored output. Fix the deadline clock, mutate authorization,
pause dependent calls, fail restoration after A/before B and record actual barriers. Use SCR
as the Agent-token MCP driver, adding exact keyed payload replay after a controlled deadline,
forced cached tool names/serving IDs and retained-output absence assertions; do not substitute
unkeyed calls or scripted evidence for P2. No engine/runner/index edits or new vectors. G/P.

**S2-O and S2-R1–4.** Owner uses the existing [accounts](../../docs/guides/accounts.md),
[service](../../docs/guides/1password-and-linear.md) and [Docker](../../docs/guides/docker.md)
guides subject to the accepted answers. Runs write dated evidence beside README, from empty
`<lab>` directories, pinning all inputs before timing. Run gate: PIN, B2-S/C/H, 050 and its
entire sign-in chain, 052b, SEC-D, FLT-D, S2-V/H/O, A63-KC and passing tagged safety vectors;
the isolated profile also needs A63-DR/DSP. Measure every README cell, outage and manual repair;
keep fresh-author/coordinator evidence apart. No production edits; checks are README P1–P6 and M0/M1.

## Shared with other plans

- Batch 1 supplies 070, S0b/S0c, 042, 046a/046b, 047, 048a-1/2, 048b-relay/join,
  051a/051b, 052a-1/2, 052b, 053 and PIN. 050 includes 050a/076 and AGT where needed;
  075 must permit its own-tool declaration; 042/052a must implement its inspection/plan reach; 038c states its allowed logins. These are existing
  admin orders, not additional S2 orders. B2-S/C/H supply G1 only after the rerun's judgment.
- 077 supplies fenced negative reconciliation; 081 supplies same-ID run behavior and states
  the old/new-ID overlap limit; 082 supplies the storage-loss/install corrections. None is
  implemented at this base. Do not infer completion from a work-order commit.
- A63-KC is the store for all runs, in the logged-in session; A63-DR supplies D20 containers,
  separate persistent Agent homes, browser callbacks and no service/store credential mounts.
  DSP must be on main for its run calls; ADDR is its shared prerequisite. Order 120 owns both
  providers' splits. A63-H supplies provider qualification; A63-FS ships but is not S2's store.
  A63-RP/MP/PS and PSD are not prerequisites for a lab using built-in local facilities and file
  archive installs. Neither Git fetching nor A63-1P is added to this scenario.
- SV1 after 071 supplies the field, SV2/SV3 cover earlier cases, SV4 requires every later
  order to tag its own. Manifest/runner changes stay with the admin, including S2-V.
- Order 118 owns SCR; request the E02 extensions above. S2-H owns fault preparation. Order
  119's S8 uses FLT-D; S3 and S10 use SEC-D. Sister plans are `experiments/014-s3-two-machines/plan.md`,
  `experiments/015-s4-partial-setup/plan.md` and order 120's A63 plan; coordinate IDs, not duplicate builds.
- A6 discovery/invocation traces come from T1/T4, refresh from T5's key rotation; S4 supplies
  the lost response. These are recorded during engine runs, not a new proxy probe.

## The order of the work

Today dispatch S2-F first; then S2-L1 and S2-L2 as their inputs become fixed, independently
of the engine lane. S2-H follows S2-L/SCR. A63-KC need not wait for A22. These packages do not
change S1's starting snapshot, root tools or author tasks. A fresh skeptic reviews this split first.

The engine path is the admin's batch-1 chain: 070 → S0b → 042/046a → 046b/047 → 048a-2
(joining 048a-1) → 048b-join (joining 048b-relay) → 050/051b → 053 → PIN.
The other join is S0c/051a plus 043a/043b and K3 → 052a-1/2 → 052b → 053.
Do not replace the admin's splits with the older plan's unsplit 046/048/052 orders.

After PIN, sequence SEC-D1 → SEC-D2 → SEC-D3, joining DSP from `s3/m2`; in a second lane,
077 → FLT-D1 → FLT-D2. Join 052b/050 for S2-V and all preparation for S2-R1 → R2 → R3 → R4.
B2-S → B2-C/B2-H merges only after the owner judges S1's rerun, and gates R1. Whichever of
that judgment/batch-2 path, SEC-D/DSP or package/runtime preparation finishes last determines
the critical path to R4. Runtime isolation and owner availability remain explicit joins.

## For the admin

- **A1, split:** combine secrets in one order, or use SEC-D1–3 plus FLT-D1–2.
  **Recommendation:** the proposed split, keeping each at most L; serialize core requests.rs
  edits and shell lib.rs/runner.rs edits, and assign private manifest pieces after 071.
- **A2, placement:** integrate before PIN on a side branch, or after PIN on main.
  **Recommendation:** after PIN; use the existing `s3/m2` DSP work, not another dispatch build.
- **A3, driver:** extend SCR, or build a second protocol client. **Recommendation:** extend
  SCR with controlled-time synchronization, raw keyed replay/stale-call overrides and assertions;
  keep the clock and faults in S2-H. Confirm this interface with order 118 before dispatch.

## For the owner

No new question. Use the accepted O1–O5 answers; neither A22 nor optional 1Password is reopened.

## Limits

No run, vector or provider result is claimed. This is no daemon claim, no confinement proof,
no new language and no automatic credential restart/rotation. Keep ACC-027's mediated report,
SEC-014's retained-value limit, G5's bounded evidence and all README counting rules. General
control dispatch and DL7 HTTPS/package-source fetching remain unserved at this base; neither
is silently required by choosing a service implementation that needs them.
