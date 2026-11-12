# S7: order-level plan

> **In short**
> - **Admin / 050:** finish the Agent homes, account checks and sign-in packages.
> - **Admin / 042:** finish tombstones and explicit lift for repeated clean installations.
> - **Admin / 047:** finish the pending-action core used by sign-in.
> - **Admin / 048a-1:** validate sign-in Types and detect input-required results.
> - **Admin / 048a-2:** finish sign-in pending actions and SignInUndone.
> - **Admin / 048b-relay:** relay input-required results in the shell.
> - **Admin / 048b-join:** join the relay to sign-in.
> - **Admin / 051b:** expose pending, sign-in and answer commands.
> - **Admin / 071:** split the conformance index before adding scenario pieces.
> - **Admin / SV1:** provide the safety-promise field before writing safety cases.
> - **Admin / FLT-D:** conditional only if the author's plugin declares filter support.
> - **Admin question A1:** approve the split and placement of the isolated vector piece.
> - **Owner:** confirm D4’s reading for counted runs; provide author’s repository/npm scope.

## Status

Written on 2026-11-11 by work order 119, against `main` at
`31acd7378d9134a205208ec9491a01d2ea459da0`. A plan for the lane coordinators and the admin:
it binds nothing, and changes nothing the owner accepted in [README.md](README.md).
Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched.

Revised on 2026-11-11 by work order 121, against `1b717c408291b632d285a6bf0a782596478cc770`.

## What `main` has now

Evidence below is source inspection, not a new execution result. Order files alone are not builds.

| README need or gap | Present evidence at the base; remaining work |
| --- | --- |
| Install/archive admission, PKG-026 | [archives.rs](../../engine/crates/core/src/archives.rs), line 184, checks bytes; [digest-mismatch](../../conformance/language/packages/digest-mismatch/case.yaml) and [CONFORMANCE.md](../../engine/CONFORMANCE.md), lines 24–25 and 235–245, cover archives and public file installation. Already met for local archives; no new installer order. |
| Bindings, MAN-010 | [packages.rs](../../engine/crates/core/src/packages.rs), lines 934–958, requires exactly one configurable argument; [package-binding](../../conformance/language/packages/package-binding/case.yaml) is partial evidence. Repeated literal argv still needs real upstream qualification and V2. |
| Selectors, PLC-007/025 | [registration.rs](../../engine/crates/core/src/registration.rs), lines 1302–1326, rejects conflicting selectors and a wrong node; [placement-label-darwin](../../conformance/language/registration/placement-label-darwin/case.yaml) is partial coverage. V3 adds the package's paired cases. No enrollment or second node is needed. |
| Plain tool mapping, PKG-020/IFC-007 | [packages.rs](../../engine/crates/core/src/packages.rs), line 920, checks mappings; [requests.rs](../../engine/crates/core/src/requests.rs), lines 581–599, omits Conduit metadata for plain calls; [plain-registration](../../conformance/conventions/access/plain-registration/case.yaml) rejects unsupported filters. Already implemented; real package qualification remains. |
| Real Agents and sign-in | [agent-codex manifest](../../packages/agent-codex/conduit.yaml), lines 1–17, still exports 0.1.1 / `.v1`; its [README](../../packages/agent-codex/README.md), lines 35–43, describes shared historical login. 050a/076 are order files, not merged package work. Need 050 and batch 1's sign-in chain. |
| Plugin/adapter contracts, including IFC-019 | General wire/filter slices exist in requests.rs:581–599 and [access.rs](../../engine/crates/core/src/access.rs):725–747. These do not qualify S7's plugin; the outside author supplies plugin/adapter work in T3. FLT-D is conditional on declared filter support. |
| D01, fixture and upstream | [S2's D01](../013-s2-shared-login/README.md#measures) is accepted; S7's named synthetic fixture task and first candidate `mcp-remote` are accepted too. No qualified pinned upstream or S7 harness exists. S7-H1 must verify eligibility before the author starts. |
| Brief, accounts, publication location | Fresh Codex, second subscription accounts and TypeScript/npm in the author's own repository are settled in [the answers](README.md#open-for-the-owner). Need isolated brief, argv capture and owner sign-ins, not another language choice. |
| G1: binding one occurrence | Still Open in [topic 11](../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items); MAN-010 supplies rejection, not occurrence selection. No planned task needs that behavior. If evidence later requires it, a new owner ruling and an admin spec order would precede implementation; none is authorized here. |
| Non-Rust location | A repository decision already answered, not missing language. No TypeScript is written by an order in this repository. |

The admin's state agrees with the tree: 041b is implemented, including cascades and REV-010
([CONFORMANCE.md](../../engine/CONFORMANCE.md):125–129); declare-lost and controls remain unfinished.
S7 needs neither unfinished path. 042, 049a1, 050a, 070–072, 076–082 and 038c have order files;
their specified work is absent here. For example, the index is still inline in
[manifest.yaml](../../conformance/manifest.yaml):3, and OPS-014 still lacks 077's fence.
The commits titled 081 and 082 add orders, not their specification changes.

Order 075 is merged (`55cf747`): an Agent Type states the reach of its client’s own tools.

## The orders

A scenario vector goes in the core or shell order that makes it pass if that order stays at most L;
otherwise an admin vectors order immediately follows it and depends only on the core and shell orders
its cases need, never on a lane package, owner step or run. A provider-contract case goes in lane 8’s
A63-VR, A63-VM, A63-VP or A63-VS suite in [the provider plan](../../docs/implementation/PROVIDERS.md).

Sizes are estimates before pipeline fixes, not measured results. Every split needs a fresh skeptic
before dispatch. `now` means no dependency, pin or owner gate; qualification still needs real bytes.
Repository checks mean `check_docs.py`, `check_rulings.py`, `set_names.py --check`,
`check_schemas.py`, `check_roadmap.py` under `tools/`, checker unittests and `git diff --check`.
Rust checks mean `cargo test --release`, `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check` in each changed crate; no Rust is built by this paper order.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S7-H1 | harness | Synthetic argv capture and upstream qualification kit | MAN-010, IFC-007 | — | M, 350–500 lines | lane 10 | now |
| S7-H2 | harness | Isolated author brief, consumer handoff and count ledger | PKG-018, REC-002 | S7-H1 | S, 150–250 lines | lane 10 | after H1, not S2-L |
| S7-V | vectors | V1–V4 in private S7 case directory/index | PKG-026, MAN-010, PLC-007, CON-022 | 071, SV1, PIN | M, 400–600 lines | lane 10 | after dependencies; no S7-P |
| S7-O | owner | Supply labelled second accounts and sign each Agent home in | PND-005/010/011 | 050, 051b | owner step | owner | after sign-in chain |
| S7-R | run | Fresh Codex author T1–T6, including T3 plugin/adapter | README P1–P5, M0–M2 | S7-H2, S7-V, S7-O, PIN; FLT-D only if plugin supports filters | about 1 hour | lane 10 | after dependencies; the owner confirms D4's reading |
| S7-NO | owner | Set up author-owned repository and npm scope | README Q4 | outside author identified | owner step | owner | before S7-N |
| S7-N | run | External TypeScript/npm publication check, T7 | README P6 | S7-R, S7-NO | about 1 hour | lane 10 | after dependencies; the owner confirms D4's reading |

**S7-H1.** Changes only a Rust harness crate under this experiment, with lockfile, pinned
toolchain and ignored output. Captures synthetic argv in order and qualifies `mcp-remote` first
against pinned npm bytes and a disposable upstream stub H1 itself builds under
`experiments/018-s7-outside-package/`; checks actual repeated named arguments.
No finished author package, secrets, engine edits or vectors. Checks: Rust release tests,
clippy/fmt and repository checks; coordinator retains dated eligibility/hash/argv observations.
If the candidate fails, report the blocker and return a verified alternative before a run.

**S7 preparation boundary.** Drop S7-P: README preparation requires no finished reference
plugin or adapter; the outside author builds and counts those in T3. H1 supplies only its
qualification stub and capture kit, H2 the isolated brief/handoff. No solution implementation
is supplied before the counted run.

**S7-H2.** Changes this experiment's Rust harness, brief and evidence templates only; freezes
fixture text, expected answers, upstream pins and compatible/conflicting/nonmatching examples.
Copies only spec, tasks, format/contracts and CLI reference into a history-free author checkout;
supplies no authored solution. Records interventions, diagnostics, package and argv costs per task.
No vectors or engine edits. Checks: offline isolation/clean-consumer rehearsal and repository checks.

**S7-V.** Changes only `conformance/` in its own new S7 directory, including its private index;
uses existing input/assertion forms. No runner, shared index, manifest root or core changes.
Owns V1 `s7-outside-package-install-use` (safety PKG-026), V2 `s7-repeated-argument-binding`
(safety MAN-010), V3 `s7-label-selector-admission` (safety PLC-007), and V4
`s7-plain-filter-refused` (safety CON-022). Each gets SV1's field only after SV1 merges.
Checks: engine release vectors, vector delta and repository checks. If a case needs a core fix,
move that case into the admin's fixing order; do not dispatch a dependent lane vector order.

**S7-O.** Owner uses Conduit's flow per home, browser login for Codex (device code optional)
and Claude's browser on its machine. No credential values in evidence and no vectors.
Checks: expected-account result and actual panel actions/time, recorded separately.

**S7-R.** Coordinator schedules the outside author and owner, freezes engine, harness and prepared
dependency implementations and pins, including clients, and retains dated T1–T6 prompts,
packages, records, vector verdicts and M0–M2/D01 tables here.
Allows and counts the author's required plugin, adapter and package work, corrections and
workarounds during the counted run, apart without hiding totals. Blocked work stays blocked.
Checks: P1–P5 and all four vectors, both consuming clients' observations, clean consumer
installation and handoff.

**S7-NO / N.** Before N, the owner sets up the author-owned repository and npm scope (Q4).
A local stand-in for “on npm” requires an owner answer; this plan does not authorize one. The outside author writes TypeScript and publishes npm artifacts in a repository they
own; no work order here writes that implementation. Retain pinned install/use evidence and costs
here, separately from S7-R. Checks: T7/P6 on a clean consumer and implementation language recorded.
This publication check adds no v0 condition.

## Shared with other plans

- Order 117's `S2-L`: reuse D01's qualification format, capture method and local stub techniques;
  its Linear server is not S7's named-fixture server. S7 adds npm qualification and the author’s counted plugin/adapter, fixture and argv evidence. `S2-F`'s issue/deadline contract is not S7's contract.
- `FLT-D`: depend on it only if the author’s plugin declares filter support. D01 has no
  restriction cases; otherwise drop the dependency. For declared filters use caller/filter delivery; IFC-019 remains the
  implementation's duty. Any required core/shell fixes belong to that admin order, not S7-V.
  Shared hot files are core `requests.rs`, `access.rs`, `registration.rs`, shell `runner.rs`,
  `server.rs`, `lib.rs`, and runner `lib.rs`/manifest pieces only if the fixing scope needs them.
- `050` with 050a/076, `047`, `048a-1`, `048a-2`, `048b-relay`, `048b-join`, `051b`:
  independent homes, account checks, sign-in and panel. `042` supplies lift if a retry needs it.
- `071`, `SV1`, `SV4`: private pieces and safety field, then every new case tags its promise.
  SV2/SV3 cover existing/S1 cases, not S7's four additions; safety-format changes are admin's.
- `PIN`: batch 1 through 053 precedes the rerun pin. `PSD` is not required: S7 installs pinned
  local archives; T7 still requires actual npm publication in the author’s scope. An actual `https` Conduit
  archive install would add PSD as a dependency, rather than invent another fetcher here.
- No dependency on B2-S/C/H for S7’s synthetic no-secret task; no Windows enrollment work.
  See [S2](../013-s2-shared-login/plan.md), [S4](../015-s4-partial-setup/plan.md) and
  [PROVIDERS](../../docs/implementation/PROVIDERS.md) for the shared work.

## The order of the work

Dispatch H1 now: build the disposable upstream and qualify mcp-remote against it; H2 follows.
No reference plugin/adapter precedes the author's T3. Batch 1 completes through 053 → PIN,
including 050's sign-in chain; 071/SV1/PIN → V, 050/051b → O. H2/V/O → R → NO/N,
joining FLT-D only if the author's plugin declares filter support. Counted R/N also wait:
the owner confirms D4's reading. Only those runs wait on it; no A22/A73/batch-2 wait.
Use [S4’s proposed hot-file merge order](../015-s4-partial-setup/plan.md#for-the-admin) for any conditional FLT-D engine
work; S7 adds no core/shell order of its own. H1/H2/V have no S2-L or S7-P dependency.

## For the admin

- **A1 — split/placement:** private scenario index after 071, or central index edits owned by
  admin. **Recommendation:** private S7 piece, lane 10, after PIN/SV1; admin takes any runner,
  shared-piece or core fix with its affected case. Approve the proposed scopes with a fresh skeptic.
- **A2 — D4:** Planned on the admin's reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9 only. Provisional until the owner confirms.

## For the owner

- **D4:** counted S7-R/N wait: the owner confirms D4's reading. Build/qualification preparation
  proceeds independently; no scripted substitute for real consuming Agents is selected.
- **S7-NO:** supply the author's own repository and npm scope before N. If actual npm cannot
  be used, options are defer T7 or explicitly accept a local stand-in. **Recommendation:**
  retain actual npm; a local stand-in changes Q4 and is an owner's question, not a plan choice.
- G1's repeated-argument occurrence selection stays Open; bring measured need back only if
  literal packaging cannot complete the task. Q1–Q5's other answers stand.

## Limits

No run result, eligibility result or passing vector is claimed. Static admission vectors and
G5's bounded models do not establish an outside author's success or implementation filtering.
All evidence uses `<lab>`, synthetic headers/data and no credential values. No broad MCP,
Docker, daemon, confinement or remote-node claim is added; access remains mediated.
