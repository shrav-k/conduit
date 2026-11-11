# 021 — S10: every provider swapped

> **For the owner**
> - **Accepted under ruling 75 D on 2026-11-11.** Every item labelled proposed below is accepted,
>   with the answers under "Open for the owner" applied.
> - **Proposed:** P4 — D16 preserves tasks, intended access, results and attribution.
> - **Proposed:** P5 — E06 checks immutable scheme claims and refuses a second claimant.
> - **Proposed:** M0 — carry the common author measures and selected counting rules.
> - **Proposed:** M1 — D15 inventories each provider's own work, delegation, state and setup.
> - **Proposed:** M2 — D16 counts all provider-specific edits and replacement costs.
> - **Proposed:** M3 — repeat S2's D09 on the swapped store.
> - **Proposed:** V1 — `s10-secret-store-equivalence`.
> - **Proposed:** V2 — `s10-runtime-equivalence`.
> - **Proposed:** V3 — `s10-package-source-equivalence`.
> - **Proposed:** V4 — `s10-scheme-claim-admission`.
> - **Proposed:** V5 — `s10-swapped-rotation-recovery`.
> - **Proposed:** V6 — `s10-all-providers-swapped`.
> - **Accepted, changed (ruling 75 D1):** twelve replays. The coordinator runs all five configurations;
>   the fresh author runs only the one with every provider swapped.
> - **Open:** Q1 — Which subscription accounts should sign in for the runs?
> - **Open:** Q2 — 1Password or the file store as the second secret store?
> - **Open:** Q3 — Accept P4–P5, M0–M3, V1–V6 and the comparison/run plan, or revise them?

## Status

Pre-registered on 2026-11-11 by work order 103. Accepted by the owner on 2026-11-11 under
[ruling 75](../../docs/proposals/ruling-75/README.md), D: its pass conditions, measures,
counting rules and planned vectors, with the answers under "Open for the owner" applied. Ruling
45 fixes what an experiment must show before it runs. No result is recorded here.

## Question

Do S1 and S2 still work with another secret store, runtime and package source, as
[S10](../../ROADMAP.md#scenarios) asks? What edits, replacements and operator work preserve the relationships?

## What it must show

1. **P1 — ruled:** S10 runs end to end on the reference engine with its vectors passing;
   judge task evidence and the vector report ([ruling 47](../../docs/proposals/ruling-47/README.md)).
2. **P2 — ruled:** every safety vector added here passes and names its safety promise
   ([ruling 68, point 2](../../docs/proposals/ruling-68/README.md)).
3. **P3 — ruled:** each swapped provider contract has two implementations on different existing
   tools passing the same contract vectors ([ruling 40](../../docs/proposals/ruling-40/README.md),
   [IN_FORCE.md topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement), A63).
   Keep a per-implementation vector report and tool/version identity; simulations alone do not
   establish this. The machines contract's second implementation is S3's evidence.
4. **P4 — proposed, for the owner's acceptance:** every replay meets D16's invariants below.
   Judge the frozen task oracle, relationship graph, record attribution and complete edit/replacement
   ledger; any unlisted edit is a deviation, not silently allowed portability.
5. **P5 — proposed, for the owner's acceptance:** E06 rejects a package-source Type with a control
   changing `schemes`, then refuses a second live claimant of a scheme; the first claim remains
   intact and no rejected provider fetches. Judge admission/state and dispatch evidence against
   PRV-016 and PRV-013, not the critique's older mutable-claims design.

## What it covers

- [Round 6's allocation](../../ROADMAP.md#round-6s-evidence-and-measurements): E06, E14; D15,
  D16 and D09 on the swapped store. D01 belongs to S2/S7, not a new S10 definition.
- [providers.yaml](../../spec/promises/providers.yaml): PRV-001–PRV-013, PRV-016, PRV-017,
  PRV-021, PRV-022, PRV-025–PRV-028 — provider calls, execution, claims, identity and gates.
- [secrets.yaml](../../spec/promises/secrets.yaml): SEC-005–SEC-014, SEC-019, SEC-020, SEC-023 —
  locators, exclusive engine reads, delivery and lifetime; [placement.yaml](../../spec/promises/placement.yaml):
  PLC-007, PLC-009 — compatible placement, fixed for an instance's life.
- [packages.yaml](../../spec/promises/packages.yaml): PKG-004, PKG-019, PKG-020, PKG-026,
  PKG-030; [manifest.yaml](../../spec/promises/manifest.yaml): MAN-009, MAN-010, MAN-017 —
  pinned portable artifacts, bindings and local provider implementations.
- [contracts.yaml](../../spec/promises/contracts.yaml): CTR-004–CTR-006; [schemas.yaml](../../spec/promises/schemas.yaml): SCH-001, SCH-013 — provider Types and controls.
- [access.yaml](../../spec/conventions/access.yaml): ACC-001, ACC-008, ACC-016, ACC-017,
  ACC-027; [implementation.yaml](../../spec/interface/implementation.yaml): IFC-006, IFC-009,
  IFC-019, IFC-020, IFC-041, IFC-049 — unchanged access, attribution, gate and sign-in behavior.
  [core/requirements.yaml](../../spec/core/requirements.yaml) supplies the same bound/shared-host
  contracts; a provider change cannot widen them. Replay-specific promises remain S1/S2's.
- Feeds [A63](../../docs/AUDIT.md#p1--prove-the-language-can-be-rebuilt-and-its-promises-kept);
  overlaps A22's keychain daemon question, A73's 1Password probe and A67's rotation/restart question.
  S10's scenario row is its register row; no new row is added.

## The tasks

1. **S10-T1:** install the first-provider configuration, declare the frozen S1/S2 relationships,
   and record the first-provider task outputs, effective access and request attribution.
2. **S10-T2:** change only the secret-store choices allowed by D16, explicitly replace what is
   immutable, and replay S1 and S2, including D09 on the second store.
3. **S10-T3:** from a fresh first-provider configuration, select Docker instead of the local
   process runtime, perform explicit replacement, and replay S1 and S2.
4. **S10-T4:** from a fresh first-provider configuration, select Git instead of the local-directory
   package source, install the same archives, and replay S1 and S2.
5. **S10-T5:** select all three second providers together and replay S1 and S2, including D09;
   inspect equivalent access and attribution and explain every necessary edit.
6. **S10-T6:** submit the prepared illegal scheme-changing Type and second claimant; inspect
   rejections and verify the accepted claimant still fetches correctly.

Replay [S1 T1–T7](../../docs/implementation/FIRST_BUILD.md#the-tasks) and **“The tasks” in
[S2's pre-registration](../013-s2-shared-login/README.md)** as order 101 writes it; freeze that accepted list before running.

Preparation, apart from author tasks: reset stores/datasets per configuration; provide identical
package archives through a local directory and a disposable Git repository, fault controls for
D09 as S2 defines them, and E06's rejected Type/second claimant. Package-source replays must use
a fresh install cache so an installed package cannot conceal an unused fetch provider.
The provider suite's digest mismatch, gate and secret fixtures are prepared, not live attacks.

## Measures

- **M0 — proposed, for the owner's acceptance (order 103, decision 4):** carry by reference the
  [first build's measure table](../../docs/implementation/FIRST_BUILD.md#measures), plus only
  “Provider work apart”, “A client's concepts count” and “Requests and time without access” from
  [its counting rules](../../docs/implementation/FIRST_BUILD.md#counting-rules-added-after-the-baseline-ran).
  Count per replay task and provider-change task; maintain distinct ledgers to avoid counting an
  edit twice in a total. Use [increment 4's plan](../../docs/proposals/first-build/increment-4/measurement-plan.md)
  as the counted-run model.
- **Ruled counting:** carry [ruling 63's line count](../../docs/proposals/ruling-63/README.md),
  [ruling 68 R1's panel actions apart](../../docs/proposals/ruling-68/README.md) and
  [IN_FORCE.md topic 13's per-run rules](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement).
  Source rules hold; M0's extension is proposed. No A70 comparison/bar, S1 T3 reach-around
  counting rule, T4/T5 better-baseline rule or prescribed local name `project` is carried.
- **Ruled:** record time and mistakes before any numeric target (ruling 46, point 3). S10 has no numeric bar; compare its first-provider replay, not A70.
- **M1 / D15 — proposed, for the owner's acceptance:** for **each** first and second implementation,
  record (a) operations and custom code it implements itself, by files/lines and function,
  (b) backing-tool operations delegated, (c) persistent state, location/owner and restart/stop
  obligations, and (d) operator setup, credentials, gate configuration and manual recovery.
  Mark each field observed, absent or untested with evidence. Cover all six implementations. Count
  shared library work once and list which providers use it; disclose setup scripts and supervisors.
- **M2 / D16 — proposed, for the owner's acceptance:** use the complete comparison below.
- **M3 / D09 — proposed, for the owner's acceptance:** repeat **D09** exactly as defined in
  [S2's pre-registration](../013-s2-shared-login/README.md) on the second store in T2 and T5.
  Cite that measurement by name in results; do not define rotation counting again here.

D16's following comparison and every counting rule are **proposed, for the owner's acceptance**:

- Freeze the engine/CLI revision, client versions, package contracts, artifact digests, fixture
  data, prompts, expected results and intended relationship graph before the first run.
- Allowed provider-specific edits are provider package pins/Type selections, provider Resource
  configuration and store locators, credential references, instance `runtime` selections, package
  `fetch` URIs/schemes, and backing-tool endpoint/path/volume/network/gate settings. List identity
  and engine-URL translations a container needs. A changed package pin is allowed only for the
  provider package, not a replacement operation contract or silently changed service artifact.
- Prepare pinned `package` items that both runtimes support before freezing the comparison.
  A relative package-contained `command` is not portable to Docker (PRV-007); conversion needed
  before the freeze is provider preparation, recorded in D15, not hidden as a free swap.
- Allow explicit retirement, removal and recreation of provider Resources and affected service/
  Agent instances, plus re-created Conduits pointing at their new IDs. Runtime (PRV-006),
  placement (PLC-009), secret locator (SEC-005) and scheme claims (PRV-016) are not edited live.
  Retire an old scheme claimant before registering its replacement; fixed schemes never mutate.
- Keep the same user tasks, operation contracts, intended consumer/target/capability/operation
  relationships and restrictions, expected service results and accountable caller/target chain.
  Map old/new IDs explicitly; byte-identical IDs, run handles and record timestamps are not required.
  Newly required provider/secret delivery relationships are listed separately with their authority;
  they cannot broaden an Agent's user-task access. No provider-specific branch in relationship
  semantics, weaker filter, copied Agent credential or bypass may make a replay pass.
- Count each changed document once per transition and each authored line by the carried rules;
  list each edit by JSON Pointer or command/value, old/new nonsecret value and reason. Count
  replacements by old/new ID, Conduits recreated, state copied/migrated (items and bytes), manual
  setup/recovery actions and all downtime/access-loss intervals with start/end evidence. List zero
  and not-needed explicitly. Compare results and attribution per task and report every difference.
  Separate generated changes, provider work and author work; give primary and strict totals.
  Record a forbidden edit as a failed comparison, retain it as evidence, and seek a revised plan
  before another counted comparison. No live migration or preserved sessions are promised.

## Vectors expected

Every row is **proposed, for the owner's acceptance**. All are safety vectors under
[the tracker](../../docs/AUDIT.md#how-to-use-this-document), ruling 68; none is written here.

| ID / proposed name | Observation and cited promises | Safety promise |
| --- | --- | --- |
| V1 `s10-secret-store-equivalence` | Both stores pass identical read/delivery/refusal cases: SEC-006, SEC-007, SEC-010, SEC-019 | SEC-007 |
| V2 `s10-runtime-equivalence` | Same run/stop/inspect/identity/gate suite on process and Docker: PRV-007–PRV-012, PRV-021, PRV-022, PRV-025, PRV-028 | PRV-025 |
| V3 `s10-package-source-equivalence` | Both fetch identical archive bytes and reject corruption: PRV-017, PKG-026 | PKG-026 |
| V4 `s10-scheme-claim-admission` | E06 rejects scheme-changing control and second claimant: PRV-013, PRV-016 | PRV-013 |
| V5 `s10-swapped-rotation-recovery` | S2's D09/E13 case on second store, including partial restoration: SEC-014, SEC-019, SEC-023, ACC-001 | SEC-019 |
| V6 `s10-all-providers-swapped` | Full replay preserves narrower access and caller/filter chain: ACC-001, ACC-008, ACC-017, IFC-020 | ACC-001 |

Existing partial coverage: [runtime-explicit](../../conformance/language/registration/runtime-explicit/case.yaml),
[review-relative-provider](../../conformance/conventions/setup/review-relative-provider/case.yaml),
[digest-mismatch](../../conformance/language/packages/digest-mismatch/case.yaml) and
[collision-secret](../../conformance/conventions/access/collision-secret/case.yaml).
They do not establish two real tools passing the same provider suite.

## Runs

**Accepted under ruling 75 D1, as changed:** the coordinator runs all five configurations: first
providers; second secrets only; Docker only; Git only; all second providers. Each configuration
replays S1 and S2, ten replays. The fresh Codex author runs only the configuration with every
provider swapped, replaying S1 and S2: two replays, twelve in all, plus E06 once per author. S1's no-secret replay is retained even in the secrets-only comparison.
The fresh author receives only `spec/`, accepted tasks/contracts and CLI format reference. Keep failed attempts and authors' counts apart.

| Contract | First implementation / existing tool | Second implementation / existing tool |
| --- | --- | --- |
| Secrets | S2's macOS keychain store, ruling 46 point 5 | 1Password (recommended after A73) or shipped file store, Q2 |
| Runtime | Engine's local-process runner | Docker provider over the owner's selected Docker installation |
| Package source | Engine's local-directory fetching | Git provider fetching the same pinned archives from a disposable repository |

All candidates come from A63. The shipped file store is a package; secret reading is not built-in.
Provider packages are Rust crates, one per provider. S3 supplies the second machine provider.
Use one Mac with Node/npm, Git, Docker and both clients. Record versions, pins, provider configuration and machine state before timing.

**Ruled accounts:** both the S1 replay and the S2 replay need the owner's **second Claude
subscription account and second Codex subscription account**, each client as its corresponding
Agent, each signed in through Conduit's flow ([ruling 49](../../docs/proposals/ruling-49/README.md),
[ruling 72 point 2](../../docs/proposals/ruling-72/README.md)); no API key substitutes for either.
Each new Agent home signs in through that flow, on its own (ruling 75 D2). A Claude Agent signs
in with a browser on its own machine (C3). Codex uses its browser login, with the device code an
option where the account has it on (C5). S2 additionally uses its isolated service account/
workspace and shared service credential, held only by its provider/service, as S2 defines.
Owner presence is needed at initial/replacement-home sign-ins, lapses and pending actions; record actual panel actions/time apart.

Use [the 1Password/Linear guide](../../docs/guides/1password-and-linear.md) for throwaway scope, and [Docker's guide](../../docs/guides/docker.md) for installation inventory. Apply
[experiment 005's safety rules](../005-macos-keychain/README.md#safety): throwaway keychain only,
unchanged default/search list checks, no other keychain reads, bounded noninteractive children
and cleanup. A22's real-daemon case remains untested. 1Password needs A73's dated observations,
a throwaway vault/service account, and separate owner approval before A73's desktop-app case. Keep credentials out of prompts, diffs, logs and results.

Retain dated results beside this page: frozen plan, task prompts, pins/versions, nonsecret
configuration/diffs, per-task counts, D15 inventories, D16 edit/ID/state/downtime ledger,
D09 links, wire/client observations, records, vector reports, deviations and cleanup evidence.
Use `<lab>` paths. Compare declared relationships and observed outcomes.

## What it needs first

S1/S2's accepted tasks and prerequisites; real Agent/sign-in packages; both stores and shared service;
local-process/Docker and directory/Git implementations with
one common vector suite per contract. The engine must dispatch runtime provider calls and reach
their gated servers (PRV-005, PRV-007, PRV-025, PRV-028), and dispatch package-source calls
(PRV-017), then check archives (PKG-026). [DL16 and DL7](../../engine/CONFORMANCE.md)
explicitly leave those execution paths unfinished; current core/loader vectors do not finish them.
The owner supplies second subscriptions, signs in, installs Docker and prepares throwaway service/store scope.
A73 and its desktop-case approval precede 1Password; final S2 D09 precedes M3.

## What `spec/` does not yet decide

No additional dispatch gap is identified: PRV-005/PRV-007 specify runtime calls, PRV-017 specifies source selection,
and [core/operations.yaml](../../spec/core/operations.yaml) defines inputs/outputs, including base64 archive bytes.
DL7/DL16's unserved paths are **implementation work**. Supported registries/transports, Git schemes
and store locators belong to provider contracts (PRV-012, PRV-016, SEC-005), pinned before running.
No promise requires live migration. Log any new missing behavior as a gap before a task relies on it.

- **G4 — gap:** [IN_FORCE.md topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement) marks the
  portable safety-promise field/tagging **spec pending, with the vectors**; resolve it without using group tags as safety tags.

## Open for the owner

- **Q1: Which subscription accounts should sign in for the runs?** The ruled second Claude and
  Codex subscriptions are required; options are the owner's supplied second test accounts or
  other owner-supplied second subscriptions. **Recommendation:** designate the test pair once
  for both replays, using isolated homes and Conduit's sign-in flow; do not substitute API keys.
- **Q2:** second store 1Password after A73, or the shipped file store?
  **Recommendation:** 1Password after A73 and required approvals; if unavailable, explicitly
  accept the file-store configuration before freezing the run. It still differs from keychain.
- **Q3:** accept the proposed criteria, measures, vectors and comparison/run plan, or amend them?
  **Recommendation:** accept the five-configuration matrix after providers and S2's D09 are ready.

**Answers, under [ruling 75](../../docs/proposals/ruling-75/README.md), D (2026-11-11):**

- Q1: the second subscription accounts. Every account has its own label, and every Agent home
  signs in on its own (D2). A Claude Agent signs in with a browser on its own machine (C3); Codex
  uses its browser login, with the device code an option where the account has it on (C5).
- Q2: 1Password after A73, or else the file store.
- Q3: accepted, with twelve replays in place of twenty (D1). The coordinator runs all five
  configurations; the fresh Codex author runs only the one with every provider swapped.

## Limits

This demonstrates substitution for pinned tasks/tools, not automatic migration, zero adapter work,
OS confinement or all registry/transport kinds. Machines are proved in S3. A22's daemon question,
A73's unattended behavior, scaling and S1's A70 bar remain separate. Access is mediated (ACC-027).
