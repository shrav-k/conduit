# S8: order-level plan

> **In short**
> - **Admin / FLT-D:** caller/filter delivery; FLT-D3 holds S8 V4/V5 after D1/D2 only.
> - **Admin / 050:** finish separate homes and real Agent sign-in packages.
> - **Admin / 047:** supply sign-in's pending-action core.
> - **Admin / 048a-1:** check sign-in Types and detect input-required results.
> - **Admin / 048a-2:** finish sign-in pending actions and SignInUndone.
> - **Admin / 048b-relay:** relay input-required results.
> - **Admin / 048b-join:** join shell relay and sign-in.
> - **Admin / 051b:** expose pending/sign-in/answer commands.
> - **Admin / 071:** provide separate conformance pieces.
> - **Admin / SV1:** define the safety field before new safety cases.
> - **Admin question A1:** accept the vector/core boundary and FLT-D's merge placement.
> - **Owner:** confirm D4’s reading for counted real-client runs.
> - **Owner / S8-A:** accept the concrete Requirement and search/storage contracts before code.

## Status

Written on 2026-11-11 by work order 119, against `main` at
`31acd7378d9134a205208ec9491a01d2ea459da0`. A plan for the lane coordinators and the admin:
it binds nothing, and changes nothing the owner accepted in [README.md](README.md).
Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched.

Revised on 2026-11-11 by work order 121, against `8b5c8093c1ca2a7a73e71fd50cce90a8f522f4f8`.

## What `main` has now

These are inspected source/vector slices, not results from running S8.

| README need or gap | Present evidence at the base; remaining work |
| --- | --- |
| Concrete Requirement and Operations; G2 | [core requirements](../../spec/core/requirements.yaml) supply bound/shared-host only. CTR-005 delegates filter semantics to contracts; S8's visibility/authorization-version and search/storage contracts are absent. S8-C drafts them and S8-A accepts them before implementation. This is package-contract work, not a new core Requirement. |
| Derived answers, provenance/cache; G3 | [ACC-017](../../spec/conventions/access.yaml) and [IFC-019](../../spec/interface/implementation.yaml) bind carried filters and returns but define no search policy. Q3 is now answered: derive only from sources visible in full. S8-C must state that rule across metadata, ranking/counts, links, errors and cache reuse; owner acceptance remains prerequisite. |
| Two qualified filter-aware implementations | No storage/search package exists at this base: [packages](../../packages/) contains the two Agent crates. Q2 selects filesystem and SQLite FTS behind counted Rust adapters; S8-F/S8-Q build them. S2's D05 supplies a method only, not these implementations. |
| Truthful supports, CTR-005/CON-022 | [registration.rs](../../engine/crates/core/src/registration.rs):2344–2363 checks support; [plain-registration](../../conformance/conventions/access/plain-registration/case.yaml) covers rejection. S8 adapters must advertise only the accepted filters they apply. FLT-D qualifies any missing integration without duplicating this implemented check. |
| Onward identity/filter delivery, ACC-017/IFC-020 | [requests.rs](../../engine/crates/core/src/requests.rs):581–599 constructs aware-call metadata; [access.rs](../../engine/crates/core/src/access.rs):725–747 combines carried and local filters. [s1-plain-filter](../../conformance/conventions/access/s1-plain-filter/case.yaml) covers an onward plain refusal. Need FLT-D's end-to-end qualification with own storage Conduit and actual S8 returns. |
| IFC-019 on returns and ACC-014 no-work failure | General result processing is in requests.rs:1334–1363; no filesystem/FTS filter implementation or unavailable-authorization fixture exists. Implement in S8-F/S8-Q, inject in S8-H2, verify counts and content together; the engine does not implement package visibility policy. |
| OPS-009 read repeats | requests.rs:817–827 redispatches filtered successful reads; [failed-read-repeat](../../conformance/conventions/requests/failed-read-repeat/case.yaml) is partial evidence. V3 needs changed permissions, identical read key/arguments and independently expected results, not a new repeat language. |
| Accounts and tools | Second subscriptions, filesystem/SQLite FTS and visible-only derivation are answered in [README](README.md#open-for-the-owner). [agent-codex](../../packages/agent-codex/conduit.yaml):3–6 is still 0.1.1 / `.v1`. 050 and sign-in commands/relay remain prerequisites. |
| Independent corpus/oracle and controls | Only pre-registration exists. S8-H1 fixes corpus/query/provenance expectations independently; S8-H2 injects permission-version changes, cache warming, unavailable authorization and missing onward access. |

041b's cascades and REV-010 are already present ([CONFORMANCE.md](../../engine/CONFORMANCE.md):125–129).
No S8 need calls declare-lost or a control through the unfinished public path. Order-file presence
does not imply the remaining work: 070–072, 050a/076, 077/038c and 081/082 are unimplemented
at this base; the index still has inline cases ([manifest.yaml](../../conformance/manifest.yaml):3).
There is no difference from the admin's qualified state description.
Safety's portable field remains a specification/format gap in topic 13: SV1, after 071, supplies it.
G2/G3 need package contracts, not an admin spec amendment; any newly discovered missing engine
behavior must be reported and given an admin spec order before code relies on it.

Order 075 is merged (`55cf747`): an Agent Type states the reach of its client’s own tools.

## The orders

A scenario vector goes in the core or shell order that makes it pass if that order stays at most L;
otherwise an admin vectors order immediately follows it and depends only on the core and shell orders
its cases need, never on a lane package, owner step or run. A provider-contract case goes in lane 8’s
A63-VR, A63-VM, A63-VP or A63-VS suite in [the provider plan](../../docs/implementation/PROVIDERS.md).
Every package this plan places under `packages/` is a scenario server: its directory is named
`scenario-s8-...`, and its README says that it serves S8 and is not a provider for general use
(the admin's rule, 2026-11-11).

Estimates precede pipeline fixes. All code here is Rust, with lockfile, pin and ignored output.
New scenario pieces are private; editing runner/shared pieces transfers merge authority to admin.
Repository checks mean `check_docs.py`, `check_rulings.py`, `set_names.py --check`,
`check_schemas.py`, `check_roadmap.py` under `tools/`, checker unittests and `git diff --check`.
Rust checks mean `cargo test --release`, `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check` in each changed crate; this paper order builds none.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S8-C | server | One contracts-only package pinned by both adapters | CTR-002/004/005, IFC-019 | — | M, 350–550 lines | lane 8 | now |
| S8-H1 | harness | Independent corpus, oracle and fixed query schedule | IFC-019, ACC-017 | — | M, 350–550 lines | lane 10 | now |
| S8-A | owner | Accept concrete Requirement and Operations against oracle | CTR-005, README Q2/Q3 | S8-C, S8-H1 | owner step | owner | after contracts and independent oracle |
| S8-F | server | Filesystem storage/filter-aware adapter | CTR-005, IFC-019, ACC-014 | S8-A | M, 400–600 lines | lane 8 | after owner acceptance |
| S8-Q | server | SQLite FTS search/filter-aware adapter | CTR-005, IFC-019, ACC-017 | S8-A | L, 650–850 lines | lane 8 | after owner acceptance |
| S8-H2 | harness | Finalize oracle after acceptance; permission/cache/failure controls | OPS-009, ACC-014/017, IFC-020 | S8-H1, S8-A, S8-F, S8-Q, SCR | M, 400–600 lines | lane 10 | after dependencies |
| S8-V | vectors | V1–V3 with existing input/completion forms | IFC-019, OPS-009, ACC-001 | 071, SV1, PIN | M, 400–600 lines | lane 10 | after engine/format prerequisites only |
| FLT-D3 | vectors | S8 V4/V5; admin order owned by S2 plan | ACC-014/017 | FLT-D1, FLT-D2 only | M, 300–450 lines | admin | after engine orders only, not S8-H2 |
| S8-O | owner | Label second accounts; sign each fresh Agent home in | PND-005/010/011 | 050, 051b | owner step | owner | after sign-in chain |
| S8-R1 | run | Coordinator T1–T6 | README P1–P6, M0–M2 | S8-V, FLT-D3, S8-H2, S8-O, PIN | about 1 hour | lane 10 | after dependencies; the owner confirms D4's reading |
| S8-R2 | run | Fresh author T1–T6 | same | S8-R1 | about 1 hour | lane 10 | after S8-R1; the owner confirms D4's reading |

**S8-C.** Ships one contracts-only package under `packages/scenario-s8-search-restrictions/`, as S2-F does;
both filesystem and SQLite adapters pin it, rather than copying contracts into each package. It
states caller/scope, permission-version semantics, indirect references, denial/output and state rules.
Specifies full-source visibility for derivation, visible-only counts/ranking and cache qualification
per caller/version, per the accepted answers; two implementations must be qualified by D05's method.
No implementation, new core language or vectors. Checks: schema/package validation and owner review.

**S8-H1.** Changes only this experiment's Rust corpus/oracle crate and evidence templates.
Freezes A-only, B-only, shared and neither-visible sources, hidden sentinels, mixed-source derivations,
pagination, empty and hidden-only matches. Keeps expected IDs/text/source sets and metadata apart
from adapter outputs. It is available before S8-A’s review; S8-H2 finalizes provisional
contract-dependent oracle details after acceptance and before timing, without copying adapter outputs.
No adapters, engine edits or vectors. Checks: oracle self-checks with hand-reviewed expected cases,
release tests, clippy/fmt and repository checks; absence of sentinels alone never establishes safety.

**S8-A.** Owner reviews S8-C's complete Requirement and both Operation contracts, not another
choice of backing tools or derived-content policy. Acceptance must be recorded before S8-F/Q.
No code or vectors. Check: dated accepted contracts/pins available to the author and oracle.

**S8-F.** Changes one Rust filesystem adapter package, `packages/scenario-s8-files/`: contracts, aware MCP server and tests.
Applies every carried/local filter to reads and every return, advertises truthful supports and
returns unavailable without data work if authorization cannot be judged. No new storage system,
engine changes or portable vectors. Checks: accepted contract matrix, unavailable-work counters,
release tests, clippy/fmt, repository checks; count all adapter code and configuration.

**S8-Q.** Changes a separate Rust SQLite FTS adapter package, `packages/scenario-s8-sqlite/`, with the same accepted filter contract.
Uses SQLite for indexing/search; checks onward caller/filters through its own storage Conduit,
and limits derived outputs, metadata and warmed-cache reuse to the caller's visible sources.
No distributed index, new access language or vectors. Checks: both adapters' common D05 matrix,
direct output/provenance/cross-caller-cache cases, release tests, clippy/fmt and repository checks.

**S8-H2.** Changes this experiment's Rust driver and evidence templates, not the conformance runner.
Injects permission changes, unavailable authorization, broad-to-narrow cache warming and missing
onward Conduit; captures exact keyed read repeats, dispatch counts, metadata and record chains.
Finalize the oracle against accepted contracts here (no separate H1b order); use SCR1/2
for scripted exact-protocol checks supplementing the accepted Agent tasks. No portable vector IDs here.
Checks: deterministic schedules, independent oracle comparison, release tests and repository checks.

**S8-V.** Changes only a new S8-private directory and index under `conformance/`.
Owns V1 `s8-visible-search-results`, V2 `s8-derived-content-visibility`, V3
`s8-authorization-change-and-cache`; all name safety IFC-019 using SV1's field.
Uses independently authored fixtures and expected completions, with no S8-C/A/H1/H2 or adapter
dependency. Owner acceptance still gates adapter implementation and qualification in S8-F/Q/H2;
S8-H2 checks real filter application, since a supplied completion is not evidence of server enforcement.
No core/runner changes. Checks: engine release vectors, vector delta and repository checks.

**FLT-D3 (S8 contribution; owned by S2's plan).** Admin vectors order for V4
`s8-unsupported-or-unavailable-filter` (safety ACC-014) and V5 `s8-onward-filter-chain`
(safety ACC-017), after FLT-D1 and FLT-D2 only. No S8-H2, adapter or owner dependency.
Own case directory/manifest piece with scripted completions; no core/shell code or policy.
FLT-D1/2 already carry internal O3/scripted engine fixtures and never wait on S8-H2.
Checks: engine release vectors, vector delta and repository checks.

**S8-O.** Owner supplies the second subscription labels, signs each home in independently and
handles lapses through the panel. No shared service credential or personal index is needed.
No vectors. Check: expected accounts and panel actions/time recorded apart.

**S8-R1 / S8-R2.** Each separately retains T1–T6 artifacts, faults, requests, outputs, vector report
and M0–M2 counts here; fresh author gets spec/tasks/contracts/CLI reference and no finished project.
Both consuming Agents query the same frozen corpus; root is the authorized control caller.
No code fixes inside a counted attempt. Checks: every promised output surface, permission/cache
case, absent onward call, revoked A/still-authorized B, and all five vectors. Never average runs.

## Shared with other plans

- `FLT-D` is owned by order 117. S8 needs ACC-017, IFC-020, IFC-019 delivery and CON-022 support
  checks; V4/V5 are its admin vectors part FLT-D3 after D1/D2 only. S8-V owns V1–V3 only. If either needs
  a core change, move that case to FLT-D before dispatch; no lane order waits on a core fix to pass.
  Neither FLT-D1 nor FLT-D2 waits S8 harnesses or adapters; D2 depends on DSP.
- S2's D05 is the qualification method, not S8's scope/deadline contract or implementation.
  `S2-F` and `S2-L` supply neither filesystem nor SQLite FTS adapters. No SEC-D dependency.
- `050` (with 050a/076), `047`, `048a-1/2`, `048b-relay/join`, `051b`: homes, account checks
  and sign-in. Their shared shell hot files are `server.rs`, `runner.rs`, `lib.rs`, with CLI
  command tables in 051b; the admin sequences them before FLT-D. S8 adds no parallel sign-in path.
- `071` → `SV1` and `SV4`: private index and safety field/tagging; SV2/SV3 do not own S8 cases.
  `PIN` follows batch 1 through 053. S8 does not need PSD, Docker, machine work or batch 2 for
  this no-secret local task; batch 2's driven-Agent secret restriction must not be bypassed later.
- See [S2](../013-s2-shared-login/plan.md), [S4](../015-s4-partial-setup/plan.md)
  and [PROVIDERS](../../docs/implementation/PROVIDERS.md) for their shared parts.

## The order of the work

Dispatch S8-C and H1 now. Both → A; accepted C is pinned by F/Q → H2, which finalizes the
oracle and uses SCR1/2 for scripted checks. H1 never waits A to supply the independent draft.
Batch 1 through 053 → PIN; 071/SV1/PIN → V, independently of owner acceptance and lane preparation.
FLT-D1 plus DSP → FLT-D2; D1/D2 → FLT-D3, with no S8 harness/package/owner dependency.
V/FLT-D3/H2 and 050/051b → O join R1 → R2; only the counted runs wait:
the owner confirms D4's reading. Contract acceptance may hold implementation separately.
Use [S4’s proposed hot-file merge order](../015-s4-partial-setup/plan.md#for-the-admin) for FLT-D1/2; D3 follows D2 in manifest
pieces. S8 adds no separate engine order. No PSD/Docker/A22/A73/batch-2 prerequisite.

## For the admin

- **A1 — placement:** accept FLT-D3 after FLT-D1/D2 and V1–V3 separately, or absorb a newly
  failing case into its actual engine fix. **Recommendation:** the split above, with no harness
  or package dependency for engine/vector fixes and at most L per fixing order. S4 holds the
  common merge sequence; runner/shared-manifest changes remain admin's.
- **A2 — D4:** Planned on the admin's reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9 only. Provisional until the owner confirms.

## For the owner

Counted R1/R2 wait: the owner confirms D4's reading; preparation does not wait.

S8-A asks for acceptance of the concrete Requirement and search/storage Operation contracts once
drafted. Options: accept the complete contracts applying the settled Q2/Q3 answers, or return
specific contract clauses for revision. **Recommendation:** accept only after the oracle and
two-implementation qualification matrix can be read against every clause. No new backing-tool
or derived-content-policy question is raised, and no implementation starts on assumed acceptance.

## Limits

No new core visibility language, run result or numeric bar. G5 lists ACC-014's filter evaluation
outside its model: neither bounded carriage checks nor static vectors prove these adapters apply
filters. All surfaces and counters need actual harness observations. Data stays synthetic under
`<lab>`; access remains mediated, without confinement, timing-side-channel or scale claims.
