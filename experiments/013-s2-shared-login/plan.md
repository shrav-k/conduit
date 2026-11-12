# S2: order-level plan

> **In short**
> - Admin proposal SEC-D1: secret-read decisions, attribution and fail-closed records in the core.
> - Admin proposal SEC-D2: private secret reads and delivery to runs in the shell.
> - Admin proposal SEC-D3a: secret-bearing remote connections in the shell.
> - Admin proposal SEC-D3b: SEC-023 authorized reread after values are lost.
> - Admin proposal FLT-D1: qualify and complete the core's filters, repeats and onward checks.
> - Admin proposal FLT-D2: qualify their delivery through the shell to real implementations.
> - Admin proposal FLT-D0: run baseline filter/current-tool cases before sizing fixes.
> - Admin proposal FLT-D3: S8 V4/V5 after FLT-D1 and FLT-D2 only.
> - Admin A1: accept the secret/core/shell split and merge sequence?
> - Admin A2: keep engine integration after PIN, or authorize another side branch?
> - Ruling 77: `tailscale serve` after the Mac’s tailnet rename, with ADDR and DSP, before isolated provider runs; O1–O5 stand.

## Status

Written on 2026-11-11 by work order 117, against `main` at `31acd7378d9134a205208ec9491a01d2ea459da0`.
A plan for the lane coordinators and the admin: it binds nothing, and changes nothing the owner
accepted in [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched.

Revised on 2026-11-11 by work order 121, against `1b717c408291b632d285a6bf0a782596478cc770`.

## What `main` has now

Evidence below is inspection of this commit, not a new test result. The log's entries for orders
070–072, 076–082 and 038c add order files, not their implementations. 050a/076's work is absent:
[Codex's manifest](../../packages/agent-codex/conduit.yaml), lines 3–6, still exports 0.1.1 and `.v1`.
042 and 049a-1 are absent too; [State](../../engine/crates/core/src/lib.rs), lines 65–81, has no
tombstone, pending-action or machine maps. This agrees with the admin's warning about order files.

| README prerequisite or gap | Present evidence and remaining work |
| --- | --- |
| Secret reads/delivery and record failure | [lifecycle.rs](../../engine/crates/core/src/lifecycle.rs), lines 786–803, makes ordinary run bindings; [lib.rs](../../engine/crates/shell/src/lib.rs), lines 68–87, defaults provider/connection execution to Missing. SEC-010/012/015/016's read workflow is missing: SEC-D1/D2/D3a/D3b. Literal redaction already exists in requests.rs, lines 125–145; it is not secret delivery. |
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

Order 075 is merged (`55cf747`): an Agent Type states the reach of its client’s own tools.

## The orders

A scenario vector goes in the core or shell order that makes it pass if that order stays at most L;
otherwise an admin vectors order immediately follows it and depends only on the core and shell orders
its cases need, never on a lane package, owner step or run. A provider-contract case goes in lane 8’s
A63-VR, A63-VM, A63-VP or A63-VS suite in [the provider plan](../../docs/implementation/PROVIDERS.md).
Every package this plan places under `packages/` is a scenario server: its directory is named
`scenario-s2-...`, and its README says that it serves S2 and is not a provider for general use
(the admin's rule, 2026-11-11).

Sizes are estimates before pipeline fixes; line counts include tests, excluding generated archives.
Shared SEC-D means D1/D2/D3a/D3b; FLT-D means D0–D3; S2-L means both implementation orders.
G means the seven paper checks of order 117, including the Python unit suite. E adds engine fmt,
clippy with warnings denied, release tests and vector-delta review against the dispatch base;
core/loader edits also run experiment 011's release tests. P adds fmt/clippy/release tests in
each changed Rust crate, a pinned toolchain, Cargo.lock, ignored output and package admission.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S2-F | server | Ship O3's restriction contracts | CON-018, IFC-019 | — | S, 150–220 lines | lane 8 | now |
| S2-L1 | server | Local Linear stub and aware implementation | IFC-019/020, SEC-019 | S2-F | L, 550–800 lines | lane 8 | after S2-F |
| S2-L2 | server | Separate adapter crate, reading only; the plain row runs in S2's runs | CON-022, IFC-007/019 | S2-F, S2-L1 stub protocol freeze only | L, about 1,200–1,600 lines | lane 8 | order 139 |
| S2-PL | package | Package and admit the plain server chosen for the runs | PKG-020, MAN-010/017 | the plain server's choice, with the owner | S, 150–300 lines | lane 8 | after that choice |
| SEC-D1 | core | Secret decisions/records; V4 and S10 V1 safety | SEC-006–018, SEC-023/024 | PIN, 042, S0b, SV1 | L, 650–900 lines | admin | after dependencies |
| SEC-D2 | shell | Execute reads; deliver private values to runs | SEC-010–020, PRV-011/022/026 | SEC-D1, DSP | L, 650–900 lines | admin | after SEC-D1 and DSP on main |
| SEC-D3a | shell | Secret-bearing remote connection headers | SEC-013/014/018 | SEC-D2 | L, 550–800 lines | admin | after SEC-D2 |
| SEC-D3b | shell | Authorized reread after lost values; S2 V3 | SEC-018/023/024 | SEC-D2 | M, 350–550 lines | admin | after SEC-D2 |
| FLT-D0 | vectors | Baseline V1/V2/V5; V6 if already passing | IFC-019, ACC-006/034/001 | 071, SV1, PIN | XS or S, 100–250 lines | admin | after dependencies |
| FLT-D1 | core | Fix only gaps demonstrated by D0; onward-close owner | ACC-017/034, IFC-019/020, CON-022, OPS-024 | FLT-D0, S0b, 042, 077 | sized after D0; at most L, split if needed | admin | after baseline cases run |
| FLT-D2 | shell | Filter/dependent-call wire checks with internal fixtures | ACC-017, IFC-019/020 | FLT-D1, DSP | M, 350–500 lines | admin | after dependencies |
| FLT-D3 | vectors | S8 V4/V5 with scripted completions | ACC-014/017 | FLT-D1, FLT-D2 | M, 300–450 lines | admin | after these engine orders only |
| S2-H | harness | Clock, authorization mutations and restoration barriers | OPS-024, ACC-034, SEC-014 | S2-L, SCR | L, 600–850 lines | lane 10 | after S2-L and SCR |
| S2-O | owner | Test keychain/item, narrow service key, account labels and runtime readiness | SEC-019, PRV-029 | — | about 1 hour presence | owner | after owner's availability |
| S2-LOG-P | server | Linear's OAuth login as a provider package: declared login, token in its own state, account check, one read | ruling 75 C1, C6, C7; SEC-020 | the sign-in chain's Resource login (below) | L, about 900–1,300 lines with an OAuth stub | lane 8 | after the engine relays a package's declared login |
| S2-LOG-O | owner | Register a Linear OAuth application on the primary account's test workspace; give its client ID | ruling 75 C6 | — | about 15 minutes | owner | before S2-LOG |
| S2-LOG | probe | Sign Linear in through the Interface: return carried to the Interface, account checked, one read with the token | ruling 75 C1, C2, C7; ruling 72's later answer | S2-LOG-P, S2-LOG-O, the sign-in chain | S, 100–200 lines of harness plus about 30 minutes of owner presence | lane 10 | beside S2's runs; not in S2's run gate |
| S2-R1 | run | Coordinator, local, T1–T7 | README P1–P6 | run gate below | about 1 hour; 45 min active owner proposed | lane 10 | after run gate |
| S2-R2 | run | Fresh Codex author, local, T1–T7 | README P1–P6 | S2-R1 | about 1 hour; same owner allowance | lane 10 | after S2-R1 |
| S2-R3 | run | Coordinator, isolated, T1–T7 | README P1–P6 | S2-R2, A63-DR, ADDR and DSP, and `tailscale serve` set up after the Mac’s tailnet rename (ruling 77) | about 1 hour; same owner allowance | lane 10 | after isolated readiness and the ruling 77 TLS setup |
| S2-R4 | run | Fresh Codex author, isolated, T1–T7 | README P1–P6 | S2-R3, A63-DR, ADDR and DSP, and `tailscale serve` set up after the Mac’s tailnet rename (ruling 77) | about 1 hour; same owner allowance | lane 10 | after S2-R3 and the ruling 77 TLS setup |

**S2-F.** New contracts-only package under `packages/scenario-s2-linear-restrictions/`: manifest,
contracts, documentation and admission fixtures. State clock/deadline, issue IDs, changed
authorization data, indirect/returned content, onward calls and denial/state rules as package
behavior under O3. Change no core contract, service code or vectors. Checks G and admission.

**S2-L1.** New standalone Conduit-aware Rust crate under `packages/scenario-s2-linear/`, with the package files/tests
used by [agent-codex](../../packages/agent-codex/README.md). Build deterministic local stub
mode and aware mode, controllable authorization/clock and a dependent-call barrier; synthetic
secret markers only. Ship container-runnable binaries/items for Docker; S3-PATH adds PATH manifests. No new filter semantics or engine edits; no portable vectors here. Checks G/P.

**S2-L2.** Separate adapter crate under `packages/scenario-s2-linear-adapter/`, so D01 counts each
implementation apart. Wait only for S2-F and L1's stub protocol/fixture interface, not L1's
completed aware implementation; the local Linear stub remains with L1. Containers and machines
are listed, not built, as S2-L1's are. **The adapter, as this plan defines it** (the lane
coordinator's reading of the admin's decision below, logged for the owner to strike): the least
that an author writes to put Linear's own API behind Conduit for D01's task, reading, with S2's
restriction, calling the service's GraphQL itself; it offers `issue-reading.v1` only (read and
list, since TPL-004 needs both). S7's README reads an adapter as new code around unchanged
upstream code. **The two readings are one definition (the admin's decision, 2026-11-12, logged for
the owner to strike):** the rulings use "adapter" for the glue an author writes to put an existing
tool or service behind Conduit, as in [ruling 44](../../docs/proposals/ruling-44/README.md#what-follows)'s
"the adapters written" and [ruling 75](../../docs/proposals/ruling-75/README.md#the-decisions)'s
"SQLite search behind counted adapters" for S8, where the existing thing is SQLite; for S2 the
existing thing is Linear's API, so the adapter calls its GraphQL itself, and for S7 it is upstream
code, so the adapter wraps it. Each
row's own tests qualify its restriction cases against the stub; S2-H or the runs compare
identical reads across the rows. No fork of the plain server or provider implementation; no
portable vectors. Checks G/P, D01/D05 matrix.

**The plain row (the admin's decision, 2026-11-12, logged for the owner to strike).** It runs only
in the owner's S2 runs, against the test workspace; S2-L2 is the adapter alone. No open-source
Linear MCP server found on 2026-11-12 runs unchanged against the stub. Most fix Linear's address
in their code.
[`@universal-mcp-toolkit/server-linear` 0.1.2](https://github.com/Markgatcha/universal-mcp-toolkit/tree/66293855dd03390ccaa038026e81ad12649046c0/servers/linear)
(MIT) reads its address from `LINEAR_API_URL`, but sends the key as a bearer token, which the
stub refuses and which [Linear's documentation](https://linear.app/developers/graphql) gives only
for OAuth tokens, not personal keys, and asks for teams and issue filters that the stub does not
serve. PyPI's
[`adelaidasofia-linear-mcp` 0.3.2](https://github.com/adelaidasofia/linear-mcp/tree/37fa4b592ad2b802fe507068404b7d76be4277e4)
(MIT) also reads its address from the environment, but reads and writes files under the user's
`~/.claude`, and asks for issue fields the stub does not serve. A
stub changed to suit either would be less faithful to the real API. This changes none of S2's
pass conditions: P1–P6 do not name the plain server, D01 keeps its three rows and S2-T2 its three
implementations, in the runs. It does move the plain package from S2-L2 to S2-PL: the contracts
package's README, inside its pinned archive, still gives it to S2-L2, and that sentence is
superseded here rather than edited, which would change the pinned digest.

**SEC-D1.** Core read workflow modules, lifecycle/requests/records integration and private
read-result boundary, plus conformance fixtures/assertions where needed. Hot: core requests.rs,
access.rs, lifecycle.rs; runner lib.rs and assigned manifest pieces; registration.rs only if
binding admission requires it. Persist descriptors/outcomes, never values; no store-specific logic.
Carry O3's restriction contract from S2's README as an engine-owned fixture; neither S2-F nor
S2-L is a dependency. Own V4 `s2-secret-read-fails-closed` (safety SEC-012) and S10 V1's
SEC-007 engine refusal, using scripted engine servers/completions and record-unavailability
evidence. Test failed/unknown/locked reads, no fallback, attribution and no value in records. G/E.

**SEC-D2.** Shell executor, runner and private delivery helpers; hot shell lib.rs, runner.rs,
identity.rs; no server.rs, CLI tables, core files, runner lib.rs or manifest edits planned.
Deliver only to the selected run via its authenticated runtime channel after durable read
dispatch; discard partial values and keep original values fixed for an existing run. No raw
values in State, intents published as evidence, diagnostics or Agent results. No new S2 V1–V6; G/E.

**SEC-D3a / D3b.** Separate shell jobs, each at most L; hot runner.rs/lib.rs, no other hot files.
D3a implements secret-bearing remote connections left unserved by DL16, with URL-bound
confidential headers. D3b implements SEC-023's authorized reread after values are lost, retaining
caller attribution on provider restart; no cache or automatic rotation. S2 V3
`s2-rotate-replace-repair` (safety SEC-018), including partial restoration, goes with D3b if
it makes it pass; otherwise with the actual fixing SEC-D part. If cases push that order above L,
an admin vectors order immediately follows only the engine orders needed. Checks G/E.

**FLT-D0.** Admin conformance cases/own manifest piece only, after 071/SV1/PIN. Write and run
V1 `s2-filter-current-read` (IFC-019), V2 `s2-bound-allows-unfiltered` (ACC-006), V5
`s2-dependent-call-after-close` (ACC-034), and V6 `s2-cached-tool-refused` (ACC-001) if it
passes at base. Commit only passing cases. Report failures and move those cases to FLT-D1
(or the fixing order for V6); size D1 only after this evidence. Checks G, release vectors/delta.

**FLT-D1.** Core requests/access/registration and focused assertions; hot requests.rs,
access.rs, registration.rs, runner lib.rs and assigned manifest pieces, not lifecycle.rs.
Keep existing support/chain/repeat behavior; fix only demonstrated omissions against promises.
Take only D0’s failing V1/V2/V5 cases, retaining their names/promises; V6 goes with its fixing
order if it failed at base. Own onward calls after close, also cited by S6-ACCESS. Use an internal
O3 fixture written from S2’s README, not S2-F/L; conformance cases supply implementation
completions under conformance/README.md and engine tests use scripted servers. Include static bound/support decisions
as vectors: [G5](../012-model-checking/G5.md) does not count static decisions as model proof. G/E.

**FLT-D2.** Shell dispatch/discovery tests with scripted servers inside engine tests, carrying O3's
contract from S2's README, independent of S2-F/L. Hot runner.rs,
server.rs and lib.rs; no identity.rs, core files, CLI tables or coverage index changes planned.
Check carried metadata, stale serving IDs, closure during a dependent call, and plain calls'
omission of Conduit keys. Complete only wire omissions found; never evaluate a package filter
in the engine. No new V1–V6. Checks G/E and request/reply-to-record correlation.

**FLT-D3.** Admin vectors only: S8 V4 `s8-unsupported-or-unavailable-filter` (ACC-014)
and V5 `s8-onward-filter-chain` (ACC-017), after FLT-D1 and FLT-D2 only. Synthetic
completions, own case directory/manifest piece; no S8-H2 or lane package dependency. G/E vectors.

**S2-H.** New Rust harness crate under `experiments/013-s2-shared-login/harness/`, with
Cargo.lock/toolchain/tests and ignored output. Fix the deadline clock, mutate authorization,
pause dependent calls, fail restoration after A/before B and record actual barriers. Use SCR
as the Agent-token MCP driver: SCR2 supplies exact keyed deadline replay, forced cached tool
names/serving IDs, retained-output absence assertions and clock synchronization; S2-H keeps
the clock and faults; do not substitute
unkeyed calls or scripted evidence for P2. No engine/runner/index edits or new vectors. G/P.

**S2-O and S2-R1–4.** Owner uses the existing [accounts](../../docs/guides/accounts.md),
[service](../../docs/guides/1password-and-linear.md) and [Docker](../../docs/guides/docker.md)
guides subject to the accepted answers. Runs write dated evidence beside README, from empty
`<lab>` directories, pinning all inputs before timing. Run gate: PIN, B2-S/C/H, 050 and its
entire sign-in chain, 052b, SEC-D, FLT-D0/1/2, S2-H/O, A63-KC, S2-PL (below) and passing tagged
safety vectors;
the isolated profile also needs A63-DR, ADDR and DSP, and `tailscale serve` set up after the Mac’s tailnet rename (ruling 77). FLT-D3 is S8-only, not this run gate. Measure every README cell, outage and manual repair;
keep fresh-author/coordinator evidence apart. No production edits; checks are README P1–P6 and M0/M1.

**The plain server for the runs** is chosen before them, with the owner, since it runs under the
owner's account against the test workspace, unconfined on the Mac as the engine's own runner
starts it (the admin's requirements, 2026-11-12). It must have an OSI-approved license; be pinned
by exact version and archive digest; have its source read, and its dependencies' for telemetry or
update checks, for every network destination other than Linear's API and every file it writes,
and be disqualified by any; send the personal key in the bare `Authorization` form that Linear
documents; take the key by an environment variable or an argument; and be reached by a command
or a package over stdio. A server with Linear's address fixed in its code is acceptable here,
since the runs use the real API. **S2-PL** then packages and admits it: an item marked `plain`
with its `tools` map (PKG-020), its own Operation, Capability and Type contracts, and the key
bound through `environment` (MAN-010, MAN-017).

## Logins through the Interface (ruling 72's later answer)

The owner asked that every login be tested through Conduit's Interface login flow (rulings 73 and
75, C): "we should test the interface login flow with all of these logins". The accounts are named
by label only; their addresses stay outside the repository. Each is signed in through the
Interface at least once in testing, first where this table says:

| Account (label) | First signed in through the Interface | Also |
| --- | --- | --- |
| Primary Claude | S2-R1, Agent A: Claude's browser callback on the Mac (75 C3) | S2-R2 to R4 |
| Primary ChatGPT (Codex) | S2-R1, Agent B: Codex's browser login (75 C5) | S2-R2 to R4 |
| Second Claude | S7-O, each Agent home signed in through Conduit | S10-O |
| Second ChatGPT (Codex) | S7-O | S3-O2 and S3-A81b (carried across machines), S10-O |
| Linear (primary address) | S2-LOG, below | — |

**Linear's own login, a probe beside S2.** S2 keeps the narrow API key from the keychain (O4) for
its runs, and none of its accepted pass conditions changes. Linear's login, which is OAuth, is
tested apart: S2-LOG-P builds a provider package that declares Linear's login (75 C6: the
authorization origin and path, the client ID, the redirect), keeps the token in its own state
directory (75 C1; the engine holds none, SEC-020), and checks the account by Linear's `viewer`
query (75 C7); its tests use a local OAuth stub, never the real service. S2-LOG signs the primary
Linear account in through the Interface, the return carried to the Interface that received the
sign-in (75 C2), checks the account, makes one read with the token, and records a dated trace with
no code or token. It needs the engine to relay a package's declared login for a Resource through
the Interface: the admin's sign-in chain (047, 048a-1/2, 048b-relay/join, 050, 051b), or the order
that adds Resource logins to it. Its size: the package L, about 900 to 1,300 lines with tests; the
probe S, about 100 to 200 lines of harness and about 30 minutes of the owner's presence for
consent, after about 15 minutes to register the OAuth application. A register row for it is the
admin's to add.

## Shared with other plans

- Batch 1 supplies 070, S0b/S0c, 042, 046a/046b, 047, 048a-1/2, 048b-relay/join,
  051a/051b, 052a-1/2, 052b, 053 and PIN. 050 includes 050a/076 and AGT where needed;
  075 already states its own-tool declaration; 042/052a must implement its inspection/plan reach; 038c states its allowed logins. These are existing
  admin orders, not additional S2 orders. B2-S/C/H supply G1 only after the rerun's judgment.
- 077 supplies fenced negative reconciliation; 081 supplies same-ID run behavior and states
  the old/new-ID overlap limit; 082 supplies the storage-loss/install corrections. None is
  implemented at this base. Do not infer completion from a work-order commit.
- A63-KC depends on S2-O’s throwaway test keychain and its owner authorization step;
  it is the store for all runs in the logged-in session; A63-DR supplies D20 containers,
  separate persistent Agent homes, browser callbacks and no service/store credential mounts.
  DSP must be on main for its run calls; ADDR is its shared prerequisite. Order 120 owns both
  providers' splits. A63-H supplies provider qualification; A63-FS ships but is not S2's store.
  A63-RP/MP/PS and PSD are not prerequisites for a lab using built-in local facilities and file
  archive installs. Neither Git fetching nor A63-1P is added to this scenario.
- SV1 after 071 supplies the field, SV2/SV3 cover earlier cases, SV4 requires every later
  order to tag its own. Manifest/runner changes stay with the admin, including FLT-D0/D3.
- S4’s [plan](../015-s4-partial-setup/plan.md) owns SCR1/2; SCR2 supplies E02’s extensions. S2-H owns fault preparation. Order
  119's S8 uses FLT-D; S3 and S10 use SEC-D. See [S3](../014-s3-two-machines/plan.md), [S4](../015-s4-partial-setup/plan.md)
  and [PROVIDERS](../../docs/implementation/PROVIDERS.md); coordinate IDs, not duplicate builds.
- A6 discovery/invocation traces come from T1/T4, refresh from T5's key rotation; S4 supplies
  the lost response. These are recorded during engine runs, not a new proxy probe.

Planned on the admin's reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9 only. Provisional until the owner confirms.

## The order of the work

Today dispatch S2-F first; then S2-L1 and S2-L2 as their inputs become fixed, independently
of the engine lane. S2-H follows S2-L/SCR. A63-KC need not wait for A22. These packages do not
change S1's starting snapshot, root tools or author tasks. A fresh skeptic reviews this split first.

The engine path is the admin's batch-1 chain: 070 → S0b → 042/046a → 046b/047 → 048a-2
(joining 048a-1) → 048b-join (joining 048b-relay) → 050/051b → 053 → PIN.
The other join is S0c/051a plus 043a/043b and K3 → 052a-1/2 → 052b → 053.
Do not replace the admin's splits with the older plan's unsplit 046/048/052 orders.

After PIN, FLT-D0 runs before FLT-D1 is sized. SEC-D1 → SEC-D2 → SEC-D3a/D3b joins
DSP from `s3/m2`; FLT-D1 → FLT-D2 joins DSP too. Cases stay with passing/fixing orders as
above; FLT-D3 follows D1/D2 for S8 independently. Join 052b/050 and preparation for R1 → R4.
B2-S → B2-C/B2-H gates R1 after the S1 judgment. Isolated R3/R4 also wait A63-DR, ADDR and DSP, and `tailscale serve` set up after the Mac’s tailnet rename (ruling 77).
Use [S4’s proposed hot-file merge order](../015-s4-partial-setup/plan.md#for-the-admin): the admin interleaves SEC-D1 and FLT-D1
with S3-L1/L2/RET1 in core, SEC-D2/D3a/D3b and FLT-D2 with S3-RET2 in shell. D0 follows
071/SV1/PIN; D3 follows D1/D2. There is no parallel requests.rs merge lane.

## For the admin

- **A1, split:** combine secrets in one order, or use SEC-D1/D2/D3a/D3b plus FLT-D0–3.
  **Recommendation:** the proposed split, keeping each at most L; serialize core requests.rs
  edits and shell lib.rs/runner.rs edits, and assign private manifest pieces after 071.
- **A2, placement:** integrate before PIN on a side branch, or after PIN on main.
  **Recommendation:** after PIN; use the existing `s3/m2` DSP work, not another dispatch build.

## For the owner

**Ruling 77:** isolated S2-R3/R4 wait for ADDR and DSP, and `tailscale serve` set up after the Mac’s tailnet rename, to terminate TLS at the engine's HTTPS address.
Every runtime-provider run receives that address through DSP/079, including a local Docker run.
O1–O5 stand and no other S2 work waits on this setup.

**The plain server:** chosen with the owner before S2's runs, under the requirements above; the
admin's decision that the plain row runs only there is logged here for the owner to strike.

## Limits

No run, vector or provider result is claimed. This is no daemon claim, no confinement proof,
no new language and no automatic credential restart/rotation. Keep ACC-027's mediated report,
SEC-014's retained-value limit, G5's bounded evidence and all README counting rules. General
control dispatch and DL7 HTTPS/package-source fetching remain unserved at this base; neither
is silently required by choosing a service implementation that needs them.
