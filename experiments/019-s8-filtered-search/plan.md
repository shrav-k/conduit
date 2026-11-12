# S8: order-level plan

> **In short**
> - **Admin / FLT-D:** finish and qualify caller/filter delivery and support checks; include V4/V5.
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
> - **Admin question A2:** read D4 before scheduling real-client runs.
> - **Owner / S8-A:** accept the concrete Requirement and search/storage contracts before code.

## Status

Written on 2026-11-11 by work order 119, against `main` at
`31acd7378d9134a205208ec9491a01d2ea459da0`. A plan for the lane coordinators and the admin:
it binds nothing, and changes nothing the owner accepted in [README.md](README.md).
Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched.

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
does not imply the remaining work: 070–072, 050a/076, 075/077/038c and 081/082 are unimplemented
at this base; the index still has inline cases ([manifest.yaml](../../conformance/manifest.yaml):3).
There is no difference from the admin's qualified state description.
Safety's portable field remains a specification/format gap in topic 13: SV1, after 071, supplies it.
G2/G3 need package contracts, not an admin spec amendment; any newly discovered missing engine
behavior must be reported and given an admin spec order before code relies on it.

## The orders

Estimates precede pipeline fixes. All code here is Rust, with lockfile, pin and ignored output.
New scenario pieces are private; editing runner/shared pieces transfers merge authority to admin.
Repository checks mean `check_docs.py`, `check_rulings.py`, `set_names.py --check`,
`check_schemas.py`, `check_roadmap.py` under `tools/`, checker unittests and `git diff --check`.
Rust checks mean `cargo test --release`, `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check` in each changed crate; this paper order builds none.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S8-C | server | Draft complete package contracts and qualification matrix | CTR-002/004/005, IFC-019 | — | S, 180–300 lines | lane 8 | now |
| S8-H1 | harness | Independent corpus, oracle and fixed query schedule | IFC-019, ACC-017 | — | M, 350–550 lines | lane 10 | now |
| S8-A | owner | Accept concrete Requirement and Operations | CTR-005, README Q2/Q3 | S8-C | owner step | owner | after S8-C |
| S8-F | server | Filesystem storage/filter-aware adapter | CTR-005, IFC-019, ACC-014 | S8-A | M, 400–600 lines | lane 8 | after owner acceptance |
| S8-Q | server | SQLite FTS search/filter-aware adapter | CTR-005, IFC-019, ACC-017 | S8-A | L, 650–850 lines | lane 8 | after owner acceptance |
| S8-H2 | harness | Permission/cache/failure controls and wire/effect counts | OPS-009, ACC-014/017, IFC-020 | S8-H1, S8-F, S8-Q | M, 400–600 lines | lane 10 | after dependencies |
| S8-V | vectors | V1–V3, using existing core input/completion forms | IFC-019, OPS-009, ACC-001 | 071, SV1, S8-A, S8-H2, PIN | M, 400–650 lines | lane 10 | after dependencies |
| FLT-D | core | Shared integration, with S8's V4/V5 in its fixing increment | CON-022, ACC-014/017, IFC-019/020 | S8-H2, 071, SV1, PIN; order 117's prerequisites | M, 350–500 added S8 case lines; shared code sized by 117 | admin | after dependencies |
| S8-O | owner | Label second accounts; sign each fresh Agent home in | PND-005/010/011 | 050, 051b | owner step | owner | after sign-in chain |
| S8-R1 | run | Coordinator's T1–T6 over frozen fixtures | README P1–P6, M0–M2 | S8-V, FLT-D, S8-O, PIN, A2 | about 1 hour | lane 10 | after dependencies and admin reading |
| S8-R2 | run | Fresh Codex author's separate T1–T6 | same | S8-R1 | about 1 hour | lane 10 | after S8-R1 |

**S8-C.** Changes package contract documents/prose under new search/storage package directories;
states caller/scope, permission-version semantics, indirect references, denial/output and state rules.
Specifies full-source visibility for derivation, visible-only counts/ranking and cache qualification
per caller/version, per the accepted answers; two implementations must be qualified by D05's method.
No implementation, new core language or vectors. Checks: schema/package validation and owner review.

**S8-H1.** Changes only this experiment's Rust corpus/oracle crate and evidence templates.
Freezes A-only, B-only, shared and neither-visible sources, hidden sentinels, mixed-source derivations,
pagination, empty and hidden-only matches. Keeps expected IDs/text/source sets and metadata apart
from adapter outputs; provisional contract details are finalized only after S8-A, before timing.
No adapters, engine edits or vectors. Checks: oracle self-checks with hand-reviewed expected cases,
release tests, clippy/fmt and repository checks; absence of sentinels alone never establishes safety.

**S8-A.** Owner reviews S8-C's complete Requirement and both Operation contracts, not another
choice of backing tools or derived-content policy. Acceptance must be recorded before S8-F/Q.
No code or vectors. Check: dated accepted contracts/pins available to the author and oracle.

**S8-F.** Changes one Rust filesystem adapter package: contracts, aware MCP server and tests.
Applies every carried/local filter to reads and every return, advertises truthful supports and
returns unavailable without data work if authorization cannot be judged. No new storage system,
engine changes or portable vectors. Checks: accepted contract matrix, unavailable-work counters,
release tests, clippy/fmt, repository checks; count all adapter code and configuration.

**S8-Q.** Changes a separate Rust SQLite FTS adapter package with the same accepted filter contract.
Uses SQLite for indexing/search; checks onward caller/filters through its own storage Conduit,
and limits derived outputs, metadata and warmed-cache reuse to the caller's visible sources.
No distributed index, new access language or vectors. Checks: both adapters' common D05 matrix,
direct output/provenance/cross-caller-cache cases, release tests, clippy/fmt and repository checks.

**S8-H2.** Changes this experiment's Rust driver and evidence templates, not the conformance runner.
Injects permission changes, unavailable authorization, broad-to-narrow cache warming and missing
onward Conduit; captures exact keyed read repeats, dispatch counts, metadata and record chains.
Scripted exact-protocol checks supplement the accepted Agent tasks. No portable vector IDs here.
Checks: deterministic schedules, independent oracle comparison, release tests and repository checks.

**S8-V.** Changes only a new S8-private directory and index under `conformance/`.
Owns V1 `s8-visible-search-results`, V2 `s8-derived-content-visibility`, V3
`s8-authorization-change-and-cache`; all name safety IFC-019 using SV1's field.
Uses independently authored expected completions; real application of filters is also checked by
S8-H2 against both adapters, since a supplied completion is not evidence of server enforcement.
No core/runner changes. Checks: engine release vectors, vector delta and repository checks.

**FLT-D (S8 contribution; owned by 117).** V4 `s8-unsupported-or-unavailable-filter` names
safety ACC-014; V5 `s8-onward-filter-chain` names safety ACC-017. Put these cases with the core
increment that makes them pass, not a later lane order. Hot files potentially touched by the shared
order: core `requests.rs`, `access.rs`, `registration.rs`; shell `runner.rs`, `server.rs`, `lib.rs`;
runner `lib.rs` and shared manifest pieces only if needed. No S8 policy in core, no `lifecycle.rs`,
`identity.rs` or CLI tables expected. Checks: engine release tests/clippy/fmt, vector delta,
experiment 011 tests if core/loader changes, repository checks. Admin/117 must size/split the whole.

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
  checks; it asks that order for V4/V5's fixing increment. S8-V owns V1–V3 only. If either needs
  a core change, move that case to FLT-D before dispatch; no lane order waits on a core fix to pass.
  The table's dependencies gate the S8 contribution, not FLT-D's earlier S2 increment.
- S2's D05 is the qualification method, not S8's scope/deadline contract or implementation.
  `S2-F` and `S2-L` supply neither filesystem nor SQLite FTS adapters. No SEC-D dependency.
- `050` (with 050a/076), `047`, `048a-1/2`, `048b-relay/join`, `051b`: homes, account checks
  and sign-in. Their shared shell hot files are `server.rs`, `runner.rs`, `lib.rs`, with CLI
  command tables in 051b; the admin sequences them before FLT-D. S8 adds no parallel sign-in path.
- `071` → `SV1` and `SV4`: private index and safety field/tagging; SV2/SV3 do not own S8 cases.
  `PIN` follows batch 1 through 053. S8 does not need PSD, Docker, machine work or batch 2 for
  this no-secret local task; batch 2's driven-Agent secret restriction must not be bypassed later.
- Sister plans from 117/118/120 and `docs/implementation/PROVIDERS.md` are not present at this base.

## The order of the work

Dispatch S8-C, then S8-H1 now; these independent preparations change no S1 inputs.
S8-C → S8-A → S8-F and S8-Q in parallel → S8-H2; finalize S8-H1's oracle after acceptance.
The admin's batch 1 runs through 053 → PIN; 071 → SV1 is a second prerequisite for cases.
After PIN, FLT-D's fixing increment and S8-V join 050/051b → S8-O, then S8-R1 → S8-R2.
This join is the critical path. Owner contract review and A2 can each hold it; no A22/A73 or B2
wait is needed. All core/shell merging is admin's, in two or three post-pin lanes as scheduled.

## For the admin

- **A1 — placement:** put S8's dependent V4/V5 in FLT-D, or invent a second filter-delivery core
  order. **Recommendation:** keep one FLT-D, with separate case directories; order 117/admin
  splits any combined scope above L and owns runner/shared-index edits. Fresh skeptic verifies
  V1–V3 need no additional core behavior before assigning S8-V to lane 10.
- **A2 — D4:** apply it to the optional S4/S5/S6/S9 replays, retaining S8's explicit real-Agent
  tasks and second accounts, or read it as also limiting S8. **Recommendation:** confirm the
  former against D2 and the accepted README; hold counted runs for that reading. A change to
  accepted tasks returns to the owner, not to a worker choosing scripted substitutes.

## For the owner

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
