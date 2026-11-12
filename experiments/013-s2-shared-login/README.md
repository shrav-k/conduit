# 013 — S2: one service login shared by two Agents

> **For the owner**
> - **Accepted under ruling 75 D on 2026-11-11.** Every item labelled proposed below is accepted,
>   with the answers under "Open for the owner" applied.
> - **Proposed, for the owner's acceptance — P4:** the E02/E07 filter and bound observations.
> - **Proposed, for the owner's acceptance — P5:** the E13 rotation, close and repair observations.
> - **Proposed, for the owner's acceptance — P6:** E17 visibility and D20 credential observations.
> - **Proposed, for the owner's acceptance — M0:** apply the carried counting rules below to S2.
> - **Proposed, for the owner's acceptance — D01:** the three-implementation qualification count.
> - **Proposed, for the owner's acceptance — D05:** the two-implementation restriction count.
> - **Proposed, for the owner's acceptance — D09:** the explicit-replacement rotation count.
> - **Proposed, for the owner's acceptance — D20:** the local/isolated credential comparison.
> - **Proposed, for the owner's acceptance — M1:** the three separate E17 observations.
> - **Proposed, for the owner's acceptance — V1:** `s2-filter-current-read`.
> - **Proposed, for the owner's acceptance — V2:** `s2-bound-allows-unfiltered`.
> - **Proposed, for the owner's acceptance — V3:** `s2-rotate-replace-repair`.
> - **Proposed, for the owner's acceptance — V4:** `s2-secret-read-fails-closed`.
> - **Proposed, for the owner's acceptance — V5:** `s2-dependent-call-after-close`.
> - **Proposed, for the owner's acceptance — V6:** `s2-cached-tool-refused`.
> - **Proposed, for the owner's acceptance — R1:** the workload, tasks and four-run matrix below.
> - **Open — O1:** Accept the proposed pass conditions, counting rules, vectors and run matrix below?
> - **Open — O2:** Which account labels should the runs declare for Claude Code and Codex?
> - **Open — O3:** Which complete issue-scope/deadline restriction contract should D05 qualify?
> - **Open — O4:** Use the keychain alone, or add the guide's 1Password route after A73?
> - **Open — O5:** Which keychain execution session should S2 claim to support?

## Status

Pre-registered on 2026-11-11 by work order 101. Accepted by the owner on 2026-11-11 under
[ruling 75](../../docs/proposals/ruling-75/README.md), D: its pass conditions, measures,
counting rules and planned vectors, with the answers under "Open for the owner" applied. Ruling
45 fixes what an experiment must show before it runs. No result is recorded here.

Its order-level plan is [plan.md](plan.md), written by work order 117; the plan changes nothing accepted here.

## Question

Can one service login be shared by two Agents with different restrictions, through
Requirements, Conduits, shared authentication and dependent calls, as
[S2's roadmap row](../../ROADMAP.md#scenarios) asks?

## What it must show

1. **P1 — ruled:** S2 runs end to end on the reference engine, its vectors passing, including
   every safety vector it adds ([ruling 47](../../docs/proposals/ruling-47/README.md#when-to-publish),
   [ruling 68, point 2](../../docs/proposals/ruling-68/README.md#the-decisions)).
2. **P2 — ruled:** Claude Code and Codex, run by the engine as Agents, carry out the tasks
   through it; match every request to its record. Simulated clients alone do not pass, and
   two Claude Code instances cannot substitute (rulings 47 and 68, point 4).
3. **P3 — accepted by the owner:** restrictions differ as intended without copying the shared
   service credential to the Agents ([IN_FORCE topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement),
   [ruling 46, point 3](../../docs/proposals/ruling-46/README.md#the-points-and-the-answers)).
   Judge by allowed/denied calls, service effects and the credential delivery trace.
4. **P4 — proposed, for the owner's acceptance:** E02's keyed successful read is dispatched
   again after its filter deadline, refused without retained output; E07's separate bound
   permits opening an unfiltered Conduit when it covers the target and operation. Inspect
   both relationships and record the author's intended grant; filters are not bounds.
5. **P5 — proposed, for the owner's acceptance:** rotate with both Agents active, retire and
   replace the service, interrupt access restoration, then repair explicitly. Old runs keep
   their original value; replacement reads the new one. Closing A's access leaves B working.
   Logs identify each outage and repair, with no service secret in Agent inputs or results.
6. **P6 — proposed, for the owner's acceptance:** record E17 at wire, client-to-model and next
   model action separately; D20 names readable credentials in each runtime and reports every
   restriction as mediated. No visibility or confinement is inferred from a successful call.

## What it covers

The [round-6 allocation](../../ROADMAP.md#round-6s-evidence-and-measurements) gives S2 E02, E07,
E13 and E17, and D01, D05, D09 and D20. Their definitions are in the
[critique](../../docs/proposals/reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage).
E15 belongs to S3; its D20 reuses this definition under partition.

Promise IDs exercised, by file, as written today:

- [secrets.yaml](../../spec/promises/secrets.yaml): SEC-001, SEC-002, SEC-006 to SEC-020,
  SEC-023 and SEC-024: references, reads for runs/connections, fixed values and nondisclosure.
- [conduits.yaml](../../spec/promises/conduits.yaml): CON-004, CON-008, CON-011 to CON-014,
  CON-018 and CON-022: independent access and supported filters.
- [providers.yaml](../../spec/promises/providers.yaml): PRV-003, PRV-006, PRV-007, PRV-011,
  PRV-022 to PRV-025 and PRV-029: runtimes, delivery, gates and private state.
- [placement.yaml](../../spec/promises/placement.yaml): PLC-001, PLC-007, PLC-012, PLC-020;
  [access.yaml](../../spec/conventions/access.yaml): ACC-001, ACC-006, ACC-011, ACC-027,
  ACC-029, ACC-030, ACC-034, ACC-036 to ACC-039, ACC-056.
- [operations.yaml](../../spec/conventions/operations.yaml): OPS-007, OPS-009, OPS-012,
  OPS-014, OPS-018, OPS-024, OPS-034; [records.yaml](../../spec/conventions/records.yaml): REC-001.
- [implementation.yaml](../../spec/interface/implementation.yaml): IFC-007, IFC-012, IFC-013,
  IFC-019, IFC-020, IFC-022, IFC-025, IFC-026, IFC-038, IFC-049;
  [pending.yaml](../../spec/promises/pending.yaml): PND-004, PND-005, PND-010, PND-011.
- [requirements.yaml](../../spec/core/requirements.yaml): `bound.v1` (ACC-029/030), distinct
  from the package filters. The core supplies no issue-scope or deadline filter contract.

Feeds A50/A39 and [A63](../../docs/AUDIT.md#p1--prove-the-language-can-be-rebuilt-and-its-promises-kept).
A6's traces come from S2's runs, by the admin's decision of 2026-11-11, logged for the owner to
strike. Discovery and invocation are taken from S2-T1 and S2-T4. A6's refresh is read as S2-T5's
rotation of the shared key. Each is recorded at the wire and in the engine's records. A6's lost
response comes from S4. A71/008's client evidence overlaps S2-T4. A22 supplies keychain evidence; A73 precedes any 1Password option. S7/A63 reuse
D01; S10 repeats D09 on its swapped store. No new register row is needed.

## The tasks

**R1 — proposed, for the owner's acceptance.** A is Claude Code, B is Codex. Use only the
[guide's synthetic Linear workspace](../../docs/guides/1password-and-linear.md#linear), one
service key held in a dedicated test macOS keychain. Proposed workload: A reads a small issue
set before a deadline; B reads a different, overlapping set and may update its test issues.
O3 must settle the package contract before preparation; these are workload choices, not promises.

| Task | What the author does |
| --- | --- |
| S2-T1 | Declare the two Agents, accounts, service, secret reference and Conduits; set up and sign in; have both read a shared test issue, B update one and A attempt the forbidden update. |
| S2-T2 | Qualify the same issue-read task through an unchanged plain server, a small adapter and a Conduit-aware implementation (D01); attempt a filtered Conduit to each. Run the common restriction cases on the latter two (D05). |
| S2-T3 | Demonstrate a permitted filtered read, repeat its keyed request after the deadline (E02), then inspect and exercise a separate bound permitting an unfiltered read (E07); explain the authority actually given. |
| S2-T4 | Inspect each Agent's tools and restrictions; close A's service Conduit, ask both clients to read, observe cached-tool refusal, reopen it and check both again; request a protocol refusal and a distinct tool failure (E17). |
| S2-T5 | Have both Agents work while the owner rotates the service key in the store; inspect the old run, explicitly retire and replace the service, then restore the intended access (E13, D09). |
| S2-T6 | Diagnose the prepared partial restoration, list what is unavailable and repair it explicitly; check both Agents against intended access, including forbidden requests, without guessing an unknown effect failed. |
| S2-T7 | Describe and check reach through Conduit and by the processes' own means in local and isolated deployments (D20), using test credentials only. |

**Preparation, apart from author tasks:** a Rust harness fixes the deadline clock, changes test
issue authorization data, pauses a dependent call, and fails restoration after A but before B
is restored. Pin the failure boundary before timing. E02's exact keyed replay and forced stale
calls may use a scripted protocol driver alongside the real clients; record that qualification
apart, never equate identical unkeyed calls with a repeat. Neither driver nor stub replaces P2.
Use synthetic secret markers for leak checks; never expose the live key to a checking Agent.

## Measures

**M0 — proposed, for the owner's acceptance (lane coordinator's decision, logged to strike):**
carry by reference the first build's [table of measures](../../docs/implementation/FIRST_BUILD.md#measures)
and only its [counting rules](../../docs/implementation/FIRST_BUILD.md#counting-rules-added-after-the-baseline-ran)
“Provider work apart”, “A client's concepts count”, and “Requests and time without access”.
Carry [ruling 63's line count](../../docs/proposals/first-build/increment-4/measurement-plan.md#the-s1-rerun),
with panel actions apart (**ruled**, ruling 68 R1), and
[topic 13's per-run rules](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement)
(**ruled**, their cited rulings). The original table/rules retain their recorded standing;
their extension to S2 is M0. Use the [counted-run model](../../docs/proposals/first-build/increment-4/measurement-plan.md).
Do not carry A70's comparison/bar, T3's reach rule, T4/T5's better-baseline rules, or `project`.
No numeric bar applies; observed time and mistakes precede any target (**ruled**, ruling 46.3).

Each following measure and its counting rules is **proposed, for the owner's acceptance**:

| ID | How counted, per author, task and runtime; evidence cited for each cell |
| --- | --- |
| D01 | Three rows: unchanged plain server, adapter, aware implementation. Same issue-read input and intended output; list supported operations/requirements, plain/key metadata behavior, onward-call and sign-in needs, actual success/refusal/unsupported result. Count adapter/package files, lines, concepts and configuration by M0 apart from author work. An unsupported filter on plain is an honest qualification, not a pass as filtered sharing. S7 and A63 reuse this definition. |
| D05 | Matrix of each proposed restriction case × adapter/aware implementation. Count cases passed, failed and unexercised, listing assertions: changed authorization data, keyed repeat, output delivery, onward calls and revocation. Count contract/vector/implementation work apart by M0. Freeze whose calls, clock/deadline, scope, indirect references, denial/output and state rules in complete contracts before running; `supports` alone earns no coverage. |
| D09 | On the keychain store, timestamp rotation, last old-value use, retirement, replacement, each restored Conduit and first successful authorized call per Agent. Count changed files/lines, requests, changed IDs, recreated relationships, person interventions and every interval without intended access, including partial failure. Record in-flight outcomes and external effects before repair. No live restart is assumed; S10 repeats this definition on its swapped store. |
| D20 | Same relationships locally and isolated. For each Agent process list attempted/readable/not-readable/not-tested test credential locations: own identity, other Agent's identity, service key/store, runtime channel and engine state. Record OS users, mounts, network routes, gate exceptions and own-client tools. Count successful reads over attempts by location and every request/outage interval; absence of an attempt proves nothing. Isolation is a runtime property, while ACC-027 still reports mediated. S3 repeats under partition. |
| M1 | For each client and E17 case, record wire reason/outcome, exact client-to-model evidence or “not observable”, and next action (retry/report refusal/report failure/report unknown/other, quoted). Count observations per category, with missing evidence separate; record notifications, session boundaries and cache settings, as in 008. |

## Vectors expected

Each name and assertion below is **proposed, for the owner's acceptance**. Use a local Linear
stub and synthetic secrets, with no account or network. Safety means failure is
[unsafe](../../docs/AUDIT.md#how-to-use-this-document); each safety vector names its safety promise
([ruling 68.2](../../docs/proposals/ruling-68/README.md#the-decisions)). No vector is written here.

| ID / proposed name | Promises and intended assertion | Safety / safety promise |
| --- | --- | --- |
| V1 `s2-filter-current-read` | OPS-009, OPS-024, OPS-034, ACC-036, IFC-019: E02 and changed authorization data rechecked, no cached disclosure. | Yes: IFC-019 |
| V2 `s2-bound-allows-unfiltered` | ACC-006, ACC-029, ACC-030, CON-022: E07 permitted within bound; exceeding it refused; unsupported plain filter rejected. | Yes: ACC-006 |
| V3 `s2-rotate-replace-repair` | SEC-010, SEC-014, SEC-018, SEC-019, CON-008: new run reads new value; partial restoration and repair never send it to an Agent. | Yes: SEC-018 |
| V4 `s2-secret-read-fails-closed` | SEC-006, SEC-012, SEC-015, SEC-016: locked/unreadable store and unavailable dispatch record prevent launch; no fallback or value in records. | Yes: SEC-012 |
| V5 `s2-dependent-call-after-close` | ACC-034, ACC-039, IFC-020: invocation completes but later onward calls after closing its chain are refused. | Yes: ACC-034 |
| V6 `s2-cached-tool-refused` | ACC-001, ACC-011, IFC-012, IFC-013, IFC-025, IFC-038: stale tool grants no access; separate protocol refusal and tool failure. | Yes: ACC-001 |

Existing slices: [current-tools](../../conformance/conventions/access/current-tools/case.yaml),
[bounds](../../conformance/conventions/access/bounds/case.yaml),
[secret-omitted](../../conformance/interface/schemas/e05-secret-omitted/case.yaml), and
[failed-read-repeat](../../conformance/conventions/requests/failed-read-repeat/case.yaml).
The [plain-filter case](../../conformance/conventions/access/s1-plain-filter/case.yaml) covers a carried filter reaching a plain target.
These do not establish deadline filtering, rotation or real client/model visibility.

## Runs

**R1 — proposed, for the owner's acceptance:** four author runs: coordinator and fresh Codex
author, each on local and isolated profiles, sequentially, from empty labs, doing T1–T7.
The fresh author gets spec, tasks and shipped CLI format reference; author evidence stays apart.
Each run uses one Claude Code Agent and one Codex Agent, primary subscription accounts used by
A4/increment 3, one independently signed-in home per Agent. No API key authenticates an Agent.
[A8](../010-subscription-terms/results-2026-11-10.md#what-this-establishes) found that a Codex login
cannot be shared between Agents: never copy its login file or run concurrent copies.

Each run needs the Mac, test Linear workspace/key, packaged keychain store, service and both
Agent packages; local runner versus a Docker runtime configured by the
[Docker guide](../../docs/guides/docker.md), with no service credential mounted in an Agent.
Pin engine, contracts, packages, client/model versions, OS, runtime and all settings before timing.
Sign-in exercises account declaration, missing-login pending action, root panel URL consent,
Codex's browser login, with the device code an option (ruling 75 C5), and Claude's browser
callback on the Mac, account check and explicit new turn after
completion (PND-005/010/011, IFC-049). Owner supplies labels through the
[accounts guide](../../docs/guides/accounts.md), signs in and rotates the key; reserve a proposed
45 minutes of active owner time per run, plus availability for repair, recording actual time.

Keep dated results beside this file: pinned inputs/prompts, redacted task logs/diffs, author and
provider counts, records, external issue effects, fault timestamps, wire/client/model evidence,
vector verdicts, deviations, failures and missing cells. Preserve the counted method; raw runs
may be linked separately. No credential value or personal machine path goes in the evidence.

## What it needs first

The S1 engine must grow secret read/delivery and record failures (SEC-010/012/015/016), filtered
repeat dispatch (OPS-024), filter support validation (CON-022), dependent-call checks (IFC-020),
replacement/repair and real client tool changes. Packages must supply complete service/filter
contracts, plain/adapter/aware implementations, a keychain provider and Docker runtime for D20;
the file store ships with the engine but is not S2's store (ruling 46.5, IN_FORCE topic 12).
Agent packages must implement separate homes and Conduit's sign-in, not reuse historical shared
login setup. Await the pending batch-2 restriction in G1 below. Owner prepares the throwaway
keychain/item and narrow Linear key; follow [005's safety rules](../005-macos-keychain/README.md#safety).
A22 must test any claimed daemon session before that session is used: 005 measured only the
owner's logged-in session. No daemon support is inferred. A73 runs before optional 1Password.

## What `spec/` does not yet decide

- **G1 — gap:** driven Agents and secret-reading Conduits: SEC-008 currently permits Agent
  consumers; the prohibition when others can take their turns is *spec pending, batch 2* in
  [IN_FORCE topics 9/12](../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach).
  Await it; do not invent a turn-holder check. S2 gives neither Agent the service-secret Conduit.
- **G2 — contract gap:** no promise supplies issue-scope/deadline semantics, client own-tool controls or keychain locator,
  session, cache and upgrade behavior. CON-018, IFC-019 and secrets.yaml's head leave these to
  package contracts. Complete those contracts under O3/O5; no new language is proposed.
- **G3 — gap:** `spec/` supplies no safety-vector field/tagging contract yet; topic 13 marks
  it *spec pending, with the vectors*. V1–V6 name safety promises here and wait for that format.

Sign-in itself is already promised (PND-005/010/011, PRV-029, IFC-049); missing Agent-specific
homes/login choices and own-tool controls are package/engine prerequisites, including topic 9's
*spec pending, order 050*. Do not treat an older guide's “pending” as a missing sign-in promise.

## Open for the owner

- **O1:** Accept the proposed pass conditions, counting rules, vectors and run matrix below?
  Options: accept, or amend named items before any run. **Recommendation:** review them together.
- **O2:** Which account labels should the runs declare for Claude Code and Codex?
  Options: primary accounts for S2 and second accounts for S3, or other owner-supplied labels
  within those ruled roles. **Recommendation:** distinct labels and independent sign-ins per home.
- **O3:** Which complete issue-scope/deadline restriction contract should D05 qualify?
  Options: explicit issue IDs with deadline and refusal on an out-of-scope request, or another
  complete contract chosen before implementation. **Recommendation:** the first, with returned
  content and onward-call semantics specified by the package, tested on adapter and aware server.
- **O4:** Use the keychain alone, or add the guide's 1Password route after A73?
  **Recommendation:** keychain alone; store the Linear key there through the owner's tools.
  The guide's 1Password service-account route adds a second store and its own credential.
- **O5:** Which keychain execution session should S2 claim to support?
  Options: logged-in session only, or daemon as well after A22 evidence. **Recommendation:**
  logged-in only initially; do not claim a daemon result until A22 tests it.

**Answers, under [ruling 75](../../docs/proposals/ruling-75/README.md), D (2026-11-11):**

- O1: accepted.
- O2: S2 uses the primary Claude Code and Codex accounts, as ruled. Every account has its own
  label, and every Agent home signs in on its own (D2).
- O3: explicit issue IDs with a deadline, refused outside that scope.
- O4: the keychain alone.
- O5: the owner's logged-in session only, with no claim about a daemon.
- Sign-in: Codex uses its browser login, with the device code an option where the account has it
  on (C5).

## Limits

No result, numeric authoring bar or S1 baseline comparison is claimed. Linear's key is a service
credential, separate from Agent subscription authentication. The engine's OS user is trusted;
local credential readability limits P3 to delivery through Conduit. A gate is not consumer
confinement, and isolation does not change ACC-027. This is neither A5's 48-hour account test,
A73, S8's general search/derived-content case, nor proof over every client version or store.
