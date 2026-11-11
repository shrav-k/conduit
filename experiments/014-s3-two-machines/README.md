# 014 — S3: S1 and S2 across two machines

> **For the owner**
> - **Proposed, for the owner's acceptance — P3:** the E15 partition and current-access observations.
> - **Proposed, for the owner's acceptance — P4:** loss, return and old-identity observations.
> - **Proposed, for the owner's acceptance — P5:** second-account sign-in and undo observations.
> - **Proposed, for the owner's acceptance — P6:** D20 under partition with honest credential limits.
> - **Proposed, for the owner's acceptance — M0:** apply the carried counting rules below to S3.
> - **Proposed, for the owner's acceptance — D20:** reuse S2's definition under each fault cell.
> - **Proposed, for the owner's acceptance — M1:** state, dispatch and external-effect accounting.
> - **Proposed, for the owner's acceptance — M2:** enrollment and account-flow accounting.
> - **Proposed, for the owner's acceptance — V1:** `s3-partition-current-access`.
> - **Proposed, for the owner's acceptance — V2:** `s3-lost-response-effect`.
> - **Proposed, for the owner's acceptance — V3:** `s3-unreachable-return`.
> - **Proposed, for the owner's acceptance — V4:** `s3-declared-lost-return`.
> - **Proposed, for the owner's acceptance — V5:** `s3-gate-identity`.
> - **Proposed, for the owner's acceptance — V6:** `s3-second-account-sign-in`.
> - **Proposed, for the owner's acceptance — V7:** `s3-managed-enrollment` (blocked on promises).
> - **Proposed, for the owner's acceptance — R1:** tasks, fault cells and four-run matrix below.
> - **Open — O1:** Accept the proposed pass conditions, counting rules, vectors and run matrix below?
> - **Open — O2:** Which account labels should the runs declare for Claude Code and Codex?
> - **Open — O3:** Accept the fixed control plane and managed-node topology below?
> - **Open — O4:** Accept root declare-lost after WSL shutdown, with provider-gone a separate case?

## Status

Pre-registered on 2026-11-11 by work order 101. Awaits the owner's acceptance of what it proposes; nothing here runs
before that. Ruling 45 fixes what an experiment must show before it runs, and IN_FORCE.md topic 13 lists the criteria the
owner has accepted. No result is recorded here.

## Question

Do S1 and S2 work across two machines, including a partition and a lost machine, as
[S3's roadmap row](../../ROADMAP.md#scenarios) asks? Exercise placement by a provider, retirement and Conduit closure on
loss, unknown outcomes across the network and proof of identity.

## What it must show

1. **P1 — ruled:** S3 runs end to end on the reference engine with its vectors passing, including every safety vector it
   adds ([ruling 47](../../docs/proposals/ruling-47/README.md#when-to-publish),
   [ruling 68, point 2](../../docs/proposals/ruling-68/README.md#the-decisions)).
2. **P2 — ruled:** use real machines
   ([IN_FORCE topic 10](../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), rulings 47 and 68.4): the
   Mac and owner's Windows computer with WSL2; no rented stand-in
   ([ruling 72.3](../../docs/proposals/ruling-72/README.md#the-decisions)). Record actual placement.
3. **P3 — proposed, for the owner's acceptance:** in E15's timing cells, no new dispatch occurs without current
   authorization. An unreachable/omitted machine produces unavailable for new requests and no retirement. A dispatched
   request whose reply is lost remains unknown until checked; reconcile with evidence rather than guessing no effect.
   Restore connectivity and test both a still-open relationship and one revoked during the cut against current state.
4. **P4 — proposed, for the owner's acceptance:** explicit loss retires the node and its placed instances and closes their
   Conduits; shutdown stays unknown until confirmed. On return, old runs gain no access from location or name reuse;
   removed IDs' credentials are refused. Evidence separates retirement, owed stop, confirmed stop, removal and new
   registration. Before restoring connectivity, unregister a retired test instance and witness a nonempty pending-stop
   obligation at its runtime provider; after return, verify that provider completes the stop despite removal.
5. **P5 — proposed, for the owner's acceptance:** trace second-account authentication through Conduit's flow, including
   missing-login pending action, person-only URL/evidence, correct account completion, wrong-account undo and explicit
   retry. No copied login or API-key Agent substitutes for the
   [ruled second subscriptions](../../docs/proposals/ruling-72/README.md#the-decisions).
6. **P6 — proposed, for the owner's acceptance:** D20 explains what local and isolated runtimes protect during a
   partition, which test credentials each Agent can read, and every interval without intended access. A gate or network
   membership never counts as consumer confinement; ACC-027 still reports mediated.

## What it covers

S3 owns E15 and D20 under partition in the [round-6 allocation](../../ROADMAP.md#round-6s-evidence-and-measurements),
defined in the [critique](../../docs/proposals/reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage).
It replays S1's tasks and S2's E02/E07/E13/E17 workload; it does not redefine S2's D01/D05/D09.

Promise IDs exercised, by file, as written today:

- [placement.yaml](../../spec/promises/placement.yaml): PLC-001, PLC-002, PLC-007 to PLC-009, PLC-011 to PLC-013, PLC-015,
  PLC-017 to PLC-020, PLC-023 to PLC-027: placement and loss.
- [providers.yaml](../../spec/promises/providers.yaml): PRV-002, PRV-003, PRV-006 to PRV-009, PRV-011, PRV-013 to PRV-015,
  PRV-018, PRV-019, PRV-021 to PRV-029: machine reports, placement, authenticated runs, pending stops, gate and private
  state.
- [conduits.yaml](../../spec/promises/conduits.yaml): CON-007, CON-008, CON-011 to CON-014, CON-018, CON-022;
  [access.yaml](../../spec/conventions/access.yaml): ACC-001, ACC-011, ACC-027, ACC-034, ACC-036, ACC-037, ACC-039:
  current authority and in-flight work.
- [operations.yaml](../../spec/conventions/operations.yaml): OPS-001, OPS-007, OPS-008, OPS-009, OPS-012, OPS-014,
  OPS-018, OPS-024, OPS-034: repeats and uncertainty; [lifecycle.yaml](../../spec/conventions/lifecycle.yaml): LIF-010,
  LIF-025, LIF-036; [records.yaml](../../spec/conventions/records.yaml): REC-001 to REC-003.
- [secrets.yaml](../../spec/promises/secrets.yaml): SEC-010 to SEC-019, SEC-023, SEC-024: secret delivery, outages and
  values retained by runs.
- [implementation.yaml](../../spec/interface/implementation.yaml): IFC-012, IFC-013, IFC-017, IFC-019, IFC-022, IFC-025,
  IFC-032, IFC-038, IFC-041, IFC-042, IFC-049; [pending.yaml](../../spec/promises/pending.yaml): PND-001 to PND-005,
  PND-010, PND-011.
- [requirements.yaml](../../spec/core/requirements.yaml): `shared-host.v1` (PLC-013, CON-006) rejects a cross-host
  relationship; `bound.v1` remains S2's distinct access bound.

Feeds A50/A39, A64 and A40's later model checking. Exercises
[A63's machines contract](../../docs/AUDIT.md#p1--prove-the-language-can-be-rebuilt-and-its-promises-kept) on the local
machine and a second host here, not in S10. Overlaps A6's lost-response trace, A22's store limits and A5/A8's account
evidence; A73 only if the S2 owner selects 1Password. [A74](../../ROADMAP.md#experiments-and-probes) runs first; its
mechanism evidence prepares S3, never substitutes for an engine run. No new register row is needed.

## The tasks

**R1 — proposed, for the owner's acceptance.** One fixed control plane on M1 (the Mac); M2 is the owner's Windows computer
running Linux in WSL2. Managed enrollment, not peer mode. Claude Code A runs on M1 so its browser callback is on the
engine's machine; Codex B and a shared service run on M2. Place the keychain provider on M1; deliver the service secret to
M2 only through the authenticated runtime channel. Place S1's shared directory/history on M2, with node-local paths
interpreted there. Repeat with consumer isolation using a Docker runtime.

| Task | What the author does | A74 hypotheses depended on |
| --- | --- | --- |
| S3-T1 | Enroll M2 through Conduit, compare its fingerprint in the engine panel, declare/select placement and inspect both nodes. Sign the two Agents in to their second accounts through Conduit, including the prepared wrong-account attempt. | H1/H3 for channel proof; no H1–H10 tests enrollment or account checks themselves. |
| S3-T2 | Replay S1's T1–T7 across M1/M2: directory, history, inspect reach, live access change, upgrade, replacement, revoke and restore; confirm both real clients and records. | H1/H2/H3/H7: gates, endpoint identity and remote channel; H4 when the replay removes an instance. |
| S3-T3 | Replay S2's T1–T7 across M1/M2, including differing restrictions, dependent calls, key rotation, partial restoration and D20; keep its counts by task. | H1/H2/H3/H7 and H4 on replacement; SEC-014's retained-value limit still applies. |
| S3-T4 | For each workload, diagnose the four prepared partition cells, request work during the cut, inspect state, check an unknown effect, and reconcile only with evidence. | H1/H2/H3/H6/H7; H6 supplies stream/timeout observations, not proof that an effect stopped. |
| S3-T5 | After healing each cut, test retained versus revoked access and restore intended access explicitly; enumerate downtime. T6 exercises owed-stop recovery separately. | H1/H2/H3/H6/H7; H4 if repair removes/recreates an instance. |
| S3-T6 | After the prepared WSL shutdown, inspect unreachable state, declare M2 lost as root and inspect retirement/closure. While WSL is still down, unregister a retired test instance with a selected run implementation. Verify that its runtime provider received `stop.v1`, returned unknown and recorded an outstanding obligation before removal (LIF-036, PRV-021). Only then restart WSL, verify the provider completes that owed stop despite removal, and explicitly remove/recreate remaining instances as needed; test old and new identities. | H1/H2/H3/H4/H7; H6 explains unconfirmed work before loss. |

No task depends on H5 (renewal, not relied on in v0), H8 (OCI repackaging), H9 (two engine stand-ins sharing dispatch), or
H10 (third host behind NAT). Pin a remote-capable implementation item without OCI conversion. A later choice using OCI, a
multi-part engine or NAT must amend this plan before running and name its extra dependencies. This does not waive A40 or
ACC-037.

**Fault preparation, apart from author tasks:** follow the
[guide's network-cut method](../../docs/guides/second-machine.md#how-s3-cuts-the-network): drop traffic to/from M1 using a
Linux-side firewall rule; loss is `wsl --shutdown`. A coordinator script restores both. Nothing outside these machines is
touched. For each workload, prepare four cells: cut before/after an externally witnessed effect × cut before/after a
revocation committed at M1. If the cut precedes revocation, revoke at M1 during the cut. Use barriers and a withheld reply
to distinguish effect from response. Record the actual ordering, not its intended label. Each cell reports unreachable,
heals, then T5 checks current access. Use a separate fresh phase for T6; silence or WSL shutdown alone never warrants
provider `gone` (PRV-014). Scripted variants may report `gone` only with the stipulated knowledge; test that path
separately from explicit root declare-lost. Test current-state-store unreadability with a controlled fault at M1 as well:
packets to M2 alone do not make M1's state unavailable. For T6, keep the runtime provider reachable on M1 while WSL is
down; its contract must retain an unperformable stop and complete it on return. Prepare a running test instance on M2
before shutdown, with provider evidence keyed by instance/run ID for stop receipt, pending obligation and completion.

## Measures

**M0 — proposed, for the owner's acceptance (lane coordinator's decision, logged to strike):** carry by reference the
first build's [table of measures](../../docs/implementation/FIRST_BUILD.md#measures), only its
[three counting rules](../../docs/implementation/FIRST_BUILD.md#counting-rules-added-after-the-baseline-ran) “Provider
work apart”, “A client's concepts count”, and “Requests and time without access”,
[ruling 63's line count](../../docs/proposals/first-build/increment-4/measurement-plan.md#the-s1-rerun), with panel
actions apart (**ruled**, ruling 68 R1), and
[topic 13's per-run rules](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement) (**ruled**, their cited
rulings). The source measures retain their recorded standing; extending them to S3 is M0. Follow the
[counted-run model](../../docs/proposals/first-build/increment-4/measurement-plan.md). Do not carry A70's comparison/bar,
T3's reach rule, T4/T5's better-baseline rules or `project`. No numeric bar applies; observed time and mistakes precede
any target (**ruled**, ruling 46.3).

Each following measure and its counting rules is **proposed, for the owner's acceptance**:

| ID | How counted, separately by author, runtime, workload and fault cell |
| --- | --- |
| D20 | Reuse [S2's D20 definition](../013-s2-shared-login/README.md#measures). Repeat the credential-location attempts and requests before/during/after each partition and after loss/return. List allowed reads, refused calls and unavailable calls separately. Timestamp each interval without intended access; record gate keepalive, configured wait bounds and observed close/cancellation times. An unobserved close is not zero time and no cancellation proves absence of an effect. |
| M1 | Join request/attempt/invocation IDs with provider reports, node/instance states, close/retire/remove/stop events and service effect receipts. Count dispatches and external effects per keyed invocation; separate unkeyed work. Count unavailable reasons, unknown outcomes, retired instances, owed stops and confirmed stops separately at each barrier. For each T6 loss/return phase, require at least one provider-received stop and pending obligation while WSL is down, then match its completion after return to the removed instance/run ID; zero pending stops cannot pass P4. Compare the planned and actual ordering; missing witnesses leave a cell unmeasured. Count each reconciliation and its evidence, each manual repair and all outage intervals by M0. |
| M2 | Count authored account/enrollment lines under M0 and panel actions apart: enrollment start, evidence comparison, admission, sign-in consent, wrong-account undo, correct completion and explicit retry. Record account labels, per-instance homes, pending IDs and checks, never login files or codes. Count failed/unknown hooks separately; neither completes sign-in. List A74 H1–H10 as used/not used, with dated evidence and unresolved observations. |

## Vectors expected

Each name and assertion below is **proposed, for the owner's acceptance**; use a deterministic network/machine-provider
fixture and S2's local service stub, with no accounts or network. Safety follows
[the tracker](../../docs/AUDIT.md#how-to-use-this-document) and
[ruling 68.2](../../docs/proposals/ruling-68/README.md#the-decisions); name the safety promise.

| ID / proposed name | Promises and intended assertion | Safety / safety promise |
| --- | --- | --- |
| V1 `s3-partition-current-access` | ACC-001, ACC-011, ACC-036, ACC-037: timing cells and unreadable decision state; no new dispatch without current authority. | Yes: ACC-037 |
| V2 `s3-lost-response-effect` | OPS-007, OPS-012, OPS-014, OPS-018: before/after-effect lost replies stay unknown, checked reconciliation, no duplicate keyed effect. | Yes: OPS-018 |
| V3 `s3-unreachable-return` | PLC-023, CON-007, PLC-017, LIF-036, PRV-021: unreachable/omitted alone is unavailable, not retired. In a separate loss phase, explicitly declare lost and unregister a retired test instance while cut; require a received stop and nonempty pending obligation, then completion on return despite removal. | Yes: PLC-023 |
| V4 `s3-declared-lost-return` | PLC-017, PLC-024, PLC-027, CON-008, LIF-036, PRV-021, IFC-022, IFC-032: root loss and valid provider-gone reports retire/close and skip impossible hooks. Unregister while cut; witness a nonempty provider stop obligation and its completion after return despite removal. Removal/name reuse grants nothing to old identity. | Yes: IFC-022 |
| V5 `s3-gate-identity` | PRV-011, PRV-018, PRV-022, PRV-025, PRV-028, IFC-017, IFC-042: reject missing/wrong-instance proof before sending content; secret transport confidential. | Yes: PRV-025 |
| V6 `s3-second-account-sign-in` | PND-004, PND-005, PND-010, PND-011, IFC-049, PRV-029: person-only evidence, wrong-account undo, no model work until the expected account. | Yes: PND-011 |
| V7 `s3-managed-enrollment` | PLC-026, PND-003, PND-004 are adjacent today, not sufficient enrollment rules. Wait for G4's promises, then cite them and reject unconfirmed/wrong-machine admission. | Safety intended; safety promise missing until G4 is specified. |

Existing slices: [identity](../../conformance/conventions/access/identity/case.yaml),
[current-tools](../../conformance/conventions/access/current-tools/case.yaml),
[reconcile-true](../../conformance/conventions/requests/reconcile-true/case.yaml),
[reconcile-false](../../conformance/conventions/requests/reconcile-false/case.yaml) and
[S1 whole](../../conformance/conventions/s1/whole/case.yaml). None proves real network behavior, remote gating or managed
admission. V7 cannot be authored against guessed promise IDs.

## Runs

**R1 — proposed, for the owner's acceptance:** four sequential author runs: two authors × two runtime profiles,
from fresh labs, doing T1–T6. Each has two workload replays, eight partition cells and two loss/return phases.
Keep authors' evidence apart; the fresh Codex author gets spec, tasks and CLI format reference. Vectors supplement each run.

Every run needs M1/M2, a reachable fixed engine HTTPS address, local and second-host machines implementations qualified
against the same machine contract vectors, a remote runtime package and its gate even in the local-process profile, and
Docker for isolated consumers. Follow the [second-machine](../../docs/guides/second-machine.md) and
[Docker](../../docs/guides/docker.md) guides. Use S1's directory/history packages and S2's service/keychain/filter/Agent
packages, Linear test workspace/key on M1's test store, and the two real native clients. Pin versions, contracts,
transport/gate settings, topology and fault barriers before timing.

Each run uses one second Claude subscription account for A and one second ChatGPT subscription account for B, provided by
the owner; no API-key Agent ([ruling 72.2](../../docs/proposals/ruling-72/README.md#the-decisions)). Independently sign in
each new home through Conduit; never copy a Codex login between Agents or machines
([A8](../010-subscription-terms/results-2026-11-10.md#what-this-establishes)). T1 exercises declared expected account,
missing login, root's URL consent/panel evidence, Claude's browser callback on M1 and Codex's device flow on M2,
wrong-account undo, correct-account check and new explicit work request, as
[ruling 49](../../docs/proposals/ruling-49/README.md) and PND-011 state. The owner enables device sign-in, supplies labels
and completes vendor pages through the [accounts guide](../../docs/guides/accounts.md). Reserve a proposed hour of active
owner time per run for enrollment, sign-ins and recovery, plus availability to restore WSL; count actual time.

Dated results go beside this file: pinned documents/prompts, author/provider counts, machine and client versions, A74
dependencies, redacted wire/client/model evidence and records, external effects, fault ordering, downtime, reconciliation
evidence, vector verdicts and missing/failed cells. Keep deviations and counted artifacts; link raw runs separately,
without credentials, personal machine paths, or a model's transcription standing in for engine-panel evidence.

## What it needs first

A74's dated H1–H10 observations must exist first; use its results for the dependencies above, reporting contradictions or
missing evidence before the dependent task proceeds. A74 is an observational probe, not a new numerical pass gate. Its H10
third host is not required by S3. The engine needs enrollment promises/contracts and implementation before T1; the guide's
manual setup serves only until enrollment exists and cannot pass T1 or replace it. Needed work: machine-provider
reports/placement (PRV-014/015, PLC-026), gates and remote identity, confidential secret delivery, durable owed stops,
lifecycle loss, records, uncertainty and reconciliation. A63's second machines implementation is exercised here; its
vectors must cover both reports and placement. S2's prerequisites and complete package contracts must exist, including a
Mac-side keychain provider whose actual session is supported. Agent packages must supply separate homes and sign-in hooks;
current promises already specify sign-in, even where older guides say it is pending. Owner sets up WSL2/mirrored
networking, SSH and the private network, Docker where needed, test service/keychain and second subscription accounts; no
rented substitute.

## What `spec/` does not yet decide

- **G4 — gap:** enrollment is *spec pending, needed before S3* in
  [IN_FORCE topic 10](../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment). PLC-026 currently registers
  nodes from reports; no promise yet binds admission to stable machine ID, enrollment and key/attestation evidence, root
  confirmation and typed fingerprint comparison. Await the enrollment batch; neither the guide nor this plan supplies
  behavior.
- **G5 — gap:** PLC-024 states the cause in status/retirement records but provides no portable cause field.
  [IN_FORCE topic 7](../../IN_FORCE.md#7-operations-outcomes-revisions-records-and-observation) marks it *spec pending,
  with the vectors*. Record available evidence without inventing a field; portable assertions await that work.
- Reuse **G1/G2/G3** of [S2's gaps](../013-s2-shared-login/README.md#what-spec-does-not-yet-decide): driven-Agent secrets,
  missing package-specific contracts/controls and the safety-vector field. The accepted own-tool defaults remain *spec
  pending, order 050* in topic 9. V7's missing safety promise is part of G4. H5 renewal and peer authority are not
  prerequisites of this topology.

## Open for the owner

- **O1:** Accept the proposed pass conditions, counting rules, vectors and run matrix below? Options: accept, or amend
  named items before any run. **Recommendation:** review them together.
- **O2:** Which account labels should the runs declare for Claude Code and Codex? Options: primary accounts for S2 and
  second accounts for S3, or other owner-supplied labels within those ruled roles. **Recommendation:** distinct labels and
  independent sign-ins per home.
- **O3:** Accept the fixed control plane and managed-node topology below? Options: this topology, or a separately
  pre-registered peer/failover extension after its prerequisites. **Recommendation:** fixed M1 and managed M2; it tests S3
  without H9 or H10.
- **O4:** Accept root declare-lost after WSL shutdown, with provider-gone a separate case? Options: explicit root loss
  plus the fixture's knowledge-based gone report, or a real provider with independently demonstrated knowledge of
  destruction. **Recommendation:** the first; WSL shutdown is a reversible fault and silence must never be promoted to
  gone.

## Limits

No result or exhaustive proof is claimed; A40 remains. Partition availability, authorization caches, engine failover,
peer mode and checkpoint migration are outside this run. Unknown work may finish; retirement proves no stop and recalls
no secret. Gates do not confine Agents; OS-user trust and readable credentials remain explicit. The proposal's section-2
labels are not program scenarios; H1–H10 name only its section-11 hypotheses.
