# Plan audit and refinement tracker

Opened: 2026-11-05. State: 2026-11-10.

This tracker holds the state of the work, open questions, priorities and action items. The
[roadmap](../ROADMAP.md) holds the order of the work and every experiment and probe.
The [specification](../spec/README.md) alone binds behavior; [IN_FORCE.md](../IN_FORCE.md)
holds every other decision in force, including accepted language changes marked spec pending.
The [ledger](proposals/ruling-log.md) records the rulings and their history.

The external audit report is not kept as a file. Ruling 1's audit is preserved through its
[findings](archive/plan-audit-history.md#findings) and the historical sections of
[the tracker's history](archive/plan-audit-history.md). Neither tracker nor roadmap is a specification.

## Identifiers

A bare identifier in this document is one of the tracker's own:

| Prefix | Meaning |
| --- | --- |
| **A** | Action item |
| **Q**, **D** | Question and disagreement, in the [findings](archive/plan-audit-history.md#findings) |
| **E** | Standing probe or comparison of the plan audit, E1 to E5, E8, E10 and E11, first worded in [its history](archive/plan-audit-history.md#standing-probes-and-comparisons). Each is an action item now (A4, A5, A6, A7, A10, A19, A22 and A8), whose row is in the roadmap's [experiments and probes](../ROADMAP.md#experiments-and-probes) and is written there as "probe E10 of the plan audit". The earlier ideas E0, E6, E7, E9 and E12 were superseded by A42, A14, A40, A43 and A34, whose rows are here |
| **S** | Scenario of the experiment program, in the [roadmap](../ROADMAP.md#scenarios) |
| **P** | Priority, P0 to P3 |
| **LD** | [Layer decision](#layer-decisions) |
| **G** | Condition of the [readiness gate](implementation/INDEPENDENT_BUILDS.md#readiness-gate), G1 to G11 |
| **DL** | Implementation-defined entry in [the specification](../spec/implementation-defined.yaml) |
| **L** | Language defect, in the [register](LANGUAGE_DEFECTS.md) |

Record-specific prefixes are in the ledger's [historical appendix](proposals/ruling-log.md#history-record-specific-id-prefixes). A promise ID, such as `CON-001`, names its file through the [specification contents](../spec/README.md#contents).

## How to use this document

- An action item has one row here, in the roadmap, or in the [history](archive/plan-audit-history.md). Its done-when is kept until met. Statuses are open, in progress, ruled, done, answered, dropped or superseded, with a note where needed; ruled means written as promises, with vectors still to come.
- Whoever starts an item records in progress, with the branch or pull request carrying it. Experiments keep fixtures and dated results under [experiments/](../experiments/README.md); consult the [roadmap's register](../ROADMAP.md#experiments-and-probes) before proposing one.
- A finding against `spec/` goes to the [re-audit](proposals/reaudit/README.md); a first-build finding goes to its increment's log. A behavior missing from `spec/` is a gap there.
- The ledger's status column changes when a later ruling changes a decision. Its historical appendices preserve the old wording. Promise counts in old records are as first written; current contents are in [spec/README.md](../spec/README.md#contents).

**How the readiness gate now reads (ruling 38, point 4).** The gate of two consecutive clean
rounds of cold reading is withdrawn: four rounds showed that reading alone does not reach it.
The scenario reader found fifteen gaps in each of rounds 3 and 4, where features that were
specified apart meet. In round 5 the readers who walked scenarios on one engine and across
machines reported 16 and 23 findings. Convergence is judged instead on two things: the latest
round of readers reports no unsafe finding, and the core and the conformance
vectors agree with the promises. As of 2026-11-10 they exist on S1, with coverage gaps recorded
in the [manifest](../conformance/manifest.yaml). Round 5 reported ten unsafe findings, and
all ten are fixed; the verifiers of its fixes classed none of theirs as unsafe. Round 6
reported two findings that its reader classed as wrong and safety-critical: specific
procedures that could not be followed without breaking a general safety promise, a conflict
that round 5 classed as unsafe (M32). Both are fixed, so no such finding is open, but round 6
was not clean. The first of the two things is judged next on the read that follows the first
build, and the second on the build itself. A reader is asked to report every finding, and to
mark as unsafe any by which an engine that keeps every promise could grant access it should
not, leak a secret, lose or repeat an effect, or report a wrong result, and also any such
conflict between a procedure and a safety promise.

## Where things stand

| Item | State on 2026-11-10 |
| --- | --- |
| Specification | Draft; [contents](../spec/README.md#contents), [coverage manifest](../conformance/manifest.yaml). Accepted changes not yet written are the spec pending lines of [IN_FORCE.md](../IN_FORCE.md). Builds follow the promises as written. |
| Checks and cold reads | Six rounds, none clean; all reported findings settled. [Round 5](proposals/reaudit/round-5/README.md) and [round 6](proposals/reaudit/round-6/README.md) preserve counts, decisions and limits. Text changed since has not been read cold in full. |
| First build | Increments 1 to 3 done. Both increment 4 authors completed T1 to T7 with real clients and failed the owner's authoring bar. [Increment 4](proposals/first-build/log-increment-4.md) records findings; simplification and Fleets are ruled before implementation and rerun. |
| Secrets and machines | Present in `spec/`; [secret reasoning](proposals/secrets/README.md), [multi-machine reasoning](proposals/multi-machine/README.md). Enrollment and pending actions await their specification batch. |
| Readiness gate | [G1 to G11](implementation/INDEPENDENT_BUILDS.md#readiness-gate). The confirmed assessment before rulings 40 to 43 met none, partly G1 and G3; it is historical, not a new assessment. Ruling 58 point 7 calls for the current assessment, coordinator's until G11. Round 6 was unsafe; the next cold read judges G7. |
| Language defects | [First-sweep register](LANGUAGE_DEFECTS.md): 48 language, 39 convention, 30 type contract, 50 conformance-document rows. The 30 type rows wait for their contracts. Counts and historical triage remain there; current coverage is in the manifest. |
| Independent builds | [Process](implementation/INDEPENDENT_BUILDS.md) and [brief](implementation/ENGINE_BUILD_BRIEF.md) drafted, not issued; `build-base` unpublished. The first build is separate (ruling 46). |
| Code and evidence | [Engine](../engine/README.md), [Agent packages](../packages/), [experiments](../experiments/README.md). Current choices and limits are in [CONFORMANCE.md](../engine/CONFORMANCE.md); no broader result is claimed. |
| Publication | v0 gate in [ruling 47](proposals/ruling-47/README.md); ConduitLang chosen in [ruling 65](proposals/ruling-65/README.md). The owner's account steps wait in [the transition guide](guides/move-to-conduitlang.md). |

### What happens next

The [roadmap's order](../ROADMAP.md#the-order-of-the-work) governs: write the ruled simplification
and Fleet structure once, build it and rerun S1 with both authors; then Fleet authority and the
remaining experiments. The first build feeds a cold read before independent builds begin.

### For the next reader

Start at [the cold readers' instructions](guides/cold-readers.md); use the convergence rule above
and the known-open list below. Findings go to the next [re-audit](proposals/reaudit/README.md)
round, with IDs, severity, promises, exact quotes, a concrete situation and the least fix.
A reader changes no promise. Read `main` after the accepted fixes, not round 6's old commit.

### Who decides what

The owner makes rulings. Ruling 37, confirmed by ruling 38 point 13, lets the coordinator settle
a cold reader's finding unless it changes what Conduit is for. Ruling 46 point 4 extends that
to build findings; a fix adding or removing a feature also goes to the owner. The specification
binds behavior, regardless of a record's standing.

Ruling 48 makes the coordinator the admin session: it plans and decides, writes orders, reviews
workers' reports and accepts or undoes each worker's own choice before merging. A worker lists
its choices separately and commits nothing. [Work orders](implementation/WORK_ORDERS.md) define
the process. Sessions writing the language commit and push at milestones to their pull-request
branch, as ruling 38 confirmed; work-order workers do not.

Ruling 71 adds one or two lane coordinators beside the admin session, which stays the merge
authority for the core, the conformance runner and manifest, `spec/`, IN_FORCE.md and the ledger.
A lane coordinator merges only pull requests confined to its lane's paths. Any coordinator may
settle a small finding that adds a feature, logged for the owner to strike, except, until the S1
rerun, one that could move S1's author counts.

Ruling 58 point 1 accepts every coordinator decision, editor choice and work-order decision
logged before ruling 57, except review-list items 15 to 19 and increment 4's open decisions.
The owner reviewed items 15 to 19 on 2026-11-12, and they stand; findings 4-01 and 4-11 stay
open until S1's bar is judged. Choices logged after the cutoff stay open to the owner to strike. Ruling
67 is the coordinator's under explicit delegation. Acceptance does not prevent a later reader
from challenging a decision (ruling 39 point 6).

The records distinguish owner rulings, coordinator decisions on findings, and editor choices:

- [The ledger](proposals/ruling-log.md) links every ruling record and states its status.
- [Re-audit records](proposals/reaudit/README.md) and the [documents audit](proposals/docs-audit/README.md) keep findings, coordinator decisions and reasons.
- The rewrite's choices are in [ruling 41's open decisions](proposals/ruling-41/open-decisions.md), the “After the ruling” sections of [42](proposals/ruling-42/README.md#after-the-ruling) and [43](proposals/ruling-43/README.md#after-the-ruling), and their adjacent editor logs. Ruling 41 accepted P1 to P20 with P2 changed.
- [Round 5 decisions](proposals/reaudit/round-5/DECISIONS.md) and [fix log](proposals/reaudit/round-5/log-fixes.md) hold the integration's PLC-023/LIF-036 choice and verification. [Round 6 decisions](proposals/reaudit/round-6/DECISIONS.md) and its editor logs hold the later details of ruling 44.
- [The roadmap log](proposals/ruling-45/log-roadmap.md#the-coordinators-decisions) keeps A72's choices. [First-build logs](proposals/first-build/README.md) and the [review list](proposals/first-build/to-review-with-the-owner.md) keep build decisions; work-order records keep coordinator decisions and workers' own choices.

| Word | Who it is |
| --- | --- |
| The coordinator | The Claude session that the owner asked to write the language, and that keeps this tracker. It decides a finding where ruling 37 lets it, starts the readers and editors, checks their work and commits it. Since ruling 48 it is the admin session, which writes the work orders that Codex workers carry out, and since ruling 71 the merge authority beside one or two lane coordinators. It is not the owner. |
| A worker | A Codex session that carries out one work order (ruling 48). It changes only what the order allows, and commits nothing. |
| A cold reader | A separate session that the coordinator starts with no context. It is given only `spec/`, or the whole repository, and writes a report, which is kept. It changes nothing. The counts of findings for a round are the counts of these reports. In round 2 the editors of the closing pass and of the last pass also started three readers of their own, whose findings are rows of [the closing log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2/log-closing.md) and of [the log of the last pass](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2/log-tail.md), outside the 125. |
| An editor | A separate session that the coordinator starts to apply decisions to named files. "Editor P" and "Editor S" in round 2 were the editors of the promises and of the core package and schemas. An editor logs each choice it makes, and lists what needs a new decision without deciding it. |

## Waiting for the owner

Only the owner settles these questions; later-stage questions below retain their stated trigger.

| What | Where it is recorded |
| --- | --- |
| A20: v0 authorization's cross-path, issuer-independence and provider-scope questions, after the bypass probe A7 | [Action items](#p2--prune-and-re-baseline-under-the-scope-guard), [findings](archive/plan-audit-history.md#findings), [A7](../ROADMAP.md#experiments-and-probes) |
| After the S1 rerun: S1's bar, with findings 4-01 and 4-11, and C6 to C12 | [Ruling 71](proposals/ruling-71/README.md), points 2 and 3; [increment 4](proposals/first-build/log-increment-4.md); [ruling 62](proposals/ruling-62/README.md) |
| Ruling 77's setup on the Windows computer, which needs the owner's sudo: `conduit-run`, the coordinator's key moved to it, the lockdown, and Codex signed in as `conduit-run`; before the end-to-end run, the Mac renamed in the tailnet and HTTPS certificates turned on | [Ruling 77](proposals/ruling-77/README.md#what-follows) |
| Readiness G8: close open questions or put them outside builders' scope; G11: owner's sign-off | [Readiness gate](implementation/INDEPENDENT_BUILDS.md#readiness-gate) |

## Known to be open

Do not report these again unless there is something new:

- Secrets. The specification has them since ruling 41
  ([secrets.yaml](../spec/promises/secrets.yaml)). What its guarantee does not cover is listed
  in [spec/README.md](../spec/README.md).
- The full shape of an inspection result beyond the implemented S1 slice
  ([what the core package leaves](../spec/core/README.md#what-this-package-does-not-settle)).
  As of 2026-11-10, increment 2 supplies S1 inspection and record vectors and
  [documents its choices](../engine/CONFORMANCE.md). Since ruling 41 (item 24) a
  diagnostic is SARIF's `result`, reduced, and a condition and a record each have a core schema.
- Coverage beyond the first build's implemented slice. As of 2026-11-10, 364 portable vectors,
  the pure core on S1 and the runnable engine shell exist. The
  [manifest](../conformance/manifest.yaml) states the coverage gaps; the
  [conformance document](../engine/CONFORMANCE.md) states the implementation limits.
- `tools/check_schemas.py` does not reject an item selected for `get-secret.v1` at an `http`
  URL whose host is not a loopback address, nor an item that is run for an Interface or a Pool.
  The [schemas README](../spec/schemas/README.md) says what it checks.
- A consumer that can speak only stdio. Round 5 rejected a finding on it (M78 of its
  [decisions](proposals/reaudit/round-5/DECISIONS.md)): every engine accepts the token in the
  `Authorization` header, so an existing stdio bridge that sends it works under all of them.
- Two limits of delegation that v0 keeps on purpose, and that it states plainly since ruling
  44 (option a of D03 and D04 of round 6): a bound does not limit the filters of the Conduits
  that its holder opens, and access to management cannot be narrowed to one template
  ([what access does not narrow](../spec/README.md#what-access-does-not-narrow), ACC-029,
  ACC-030). Narrow provisioning goes through a broker, a trusted operation of a Resource whose own contract
  allows only what it should. S1 and S2 measure whether that is good enough.
- What a client and a model see. The protocol lets a client keep a protocol error from its
  model, and round 6 found no further defect in IFC-012 or IFC-025 on it
  ([its report](proposals/reaudit/round-6/reader-1.md#the-two-interface-questions-singled-out-by-the-tracker)).
  As of 2026-11-10, [experiment 008](../experiments/008-real-client/README.md) has measured
  this early for E17 of round 6, with 68 runs and a 12-session later-turn follow-up, both
  reviewed by the admin in the dated results; S2 uses that evidence.
- A request repeated without its idempotency key is new work (OPS-011). Whether a client keeps
  the key across the retries it makes is a property of the client, measured in S4 (D02 of
  round 6), and not a gap in the language.

- MAN-010 cannot bind a repeated named argument, and no promise binds its several values through `isRepeated`; waits for S7 and A63. PLC-024 needs a portable record field for its cause (A39). OPS-028's late onward request rule stays in v0, is tested in E20, and is revisited with local enforcement (A65). [Round 6's four open points](proposals/reaudit/round-6/design-critique.md#the-four-open-points) records them.
- Round 6's integration left: directory-based replacement, now tied to S1's edit-cycle measurement; INS-010's destination comparison; and no checker rule for SEC-005. Its root-copy-key collision is settled by [2-02](proposals/first-build/README.md#2-02); escaped `$ref` copies are covered by e05-escaping. [Integration](proposals/reaudit/round-6/log-integration.md#left-for-a-decision) keeps the older list.
- The protocol watch list: issue 3003 asks whether an `isError` result must conform to `outputSchema` (IFC-025's reading); the tasks extension is “Beta, to Core”, where breaking changes are allowed (IFC-016). These are the recorded observations, not a fresh protocol check. [MCP recheck](guides/cold-readers.md#what-to-watch).
- Increment 4: 4-17, whether and how ended conditions appear; 4-18, core commits copying the whole state and their measured cost. [Finding log](proposals/first-build/log-increment-4.md).
- Round 5 changed 22 interface promises and added 2 without a subsequent protocol recheck; changed 101 promises and added 3 without a complete cold read. [Known after round 5](proposals/reaudit/round-5/README.md#known-after-the-round). Text changed by round 6 and later rulings has not been read cold in full.
- S1 safety tagging: the plan counted 74 manifest cases with no `safety_rows` and 38 with an empty list; these were historical counts, not a safety classification. Order 030 found 21 distinct tags and only four naming proposed traces. Safety vectors beyond S1's proposed rows remain undefined, and `spec/` has no safety field. [Order 030](implementation/WORK_ORDERS.md#order-030), [G5](implementation/INDEPENDENT_BUILDS.md#readiness-gate), [proposed traces](../conformance/s1-safety-cases.md).
- The adjacent S1 safety boundaries: the old -32602 no-effect assumption (since corrected by increment 2 decision 8a after work order 018), IFC-046 wire resends versus repeats, and filtered-replay variants of 6-1-02 needing Conduit-aware fixtures. [Plan](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md), [plain-errors check](proposals/first-build/increment-2/decisions.md#evidence-for-decision-8a), [decision 8a](proposals/first-build/increment-2/decisions.md).
- Increment 3 closeout: the required user-settings sentinels (`owner_sentinel` MCP server and output style) were neither run nor listed as not shown; Claude Code's user skills were not tried. [Closeout](proposals/first-build/increment-3/closeout-results-2026-11-10.md).
- Loader remainder: `saphyr-parser` accepts a `---` line inside a root block scalar as content; report upstream. The earlier `patternProperties`/`additionalProperties` remainder is settled for author schemas by the core's matching from increment 2; [1-04](proposals/first-build/README.md#1-04) and [1-17](proposals/first-build/README.md#1-17) keep the decision. The loader itself delegates those keywords for document schemas, which do not use them.
- The [Codex package](../packages/agent-codex/README.md) records the real-client follow-up: six `collaboration.*` tools remained listed under every switch tried. Finding 4-06 was settled by the admin on 2026-11-10 in [increment 4](proposals/first-build/log-increment-4.md) as a stated route to parallel model work within the parent Agent's access; what offers the tools is still not found.
- Fleet papers' unverified gaps: how bounds judge `reconcile.v1` through `access`; no prohibition on registering another core management/access Resource; which Conduit carries ACC-033 row 1. [Authority inventory](proposals/projects/brief.md#unverified-authority-gaps). PKG-006 and NAM-011 speak of removing a package, but no operation removes one: [ownership inventory](proposals/projects/brief.md#ownership-and-lifecycle-constraints).
- The [multi-machine proposal](proposals/multi-machine/README.md) says “No wider survey of spec/ was made”; A65's done-when still requires that wider design. The [defect register](LANGUAGE_DEFECTS.md) says runtime rows are recorded and tested, while [CONFORMANCE.md](../engine/CONFORMANCE.md) cites no L-ID. The 30 type-layer defects wait on their type contracts.
- 032-G1 and 032-G2: two passages in `spec/promises/secrets.yaml`'s head have no promise: a standard-library file store and no engine-variable store; what each store package's contract states. Preserved until a secrets change writes promises. [Order 032's record](implementation/WORK_ORDERS.md#order-032).

**To look at in particular (ruling 38, point 11).** The interface follows the Model Context
Protocol on errors: a call to a tool that the engine does not present is answered with the
protocol's error, code -32602. A result has `isError` true where the request ended with a
reason or its outcome is failed or unknown (IFC-025). Ruling 41 (item 6b) put that in place of
the earlier rule, under which every result that carried none of its operation's declared
outputs had it. Two things follow that the owner wants examined. The protocol lets
a client keep protocol errors from its model, so an Agent's model may never see that a request
was refused. And an open proposal to the protocol, SEP-2145, would turn an unknown tool into
an error result, which would reverse the first. Round 5 changed the text on both (M71 and M75
of its [decisions](proposals/reaudit/round-5/DECISIONS.md)). A result whose `isError` is true
now begins its `content` with one text block that states the reason or the outcome (IFC-025).
A call to a tool that is not presented is answered as the protocol prescribes at the revision
in use, -32602 at the baseline revision, and the error's `message` says that no tool of that
name is presented to the instance (IFC-012). Round 6 looked at both as they stood at
`c4d7d33` and found no further defect: the message helps a client that shows the error to its
model and cannot make one do so, which is the protocol's limit, and SEP-2145 was still open on
2026-11-07. As of 2026-11-10, [experiment 008](../experiments/008-real-client/README.md)
records what the real clients showed, including the later-turn follow-up, for E17 of round 6
in S2.
- Order 082's remaining questions, the coordinator's to settle as defects, logged for the owner
  to strike: what an engine owes when it cannot store invocation state, tombstones, provenance or
  installed packages (with existing outcomes, failing the request rather than acting unrecorded),
  and races across addresses, between two Types claiming one client ID or an install and a
  project's registration. Records restored from a copy older than a purge fall under the limit
  the owner kept on 2026-11-12: they are read as found, and root purges them again
  ([what the promises on records cover](../spec/README.md#what-the-promises-on-records-cover);
  [order 082's Record](work-orders/082-spec-records-and-installs.md#record)).

**To look at after round 6.** The text that round 6 and ruling 44 changed has not been read
cold. [Round 6's decisions](proposals/reaudit/round-6/DECISIONS.md) say what changed, and the
logs beside them list each promise. Among them are interface promises that rest on the published
protocol at the baseline revision: the reading of the error -32021 (IFC-007, IFC-010, IFC-045,
IFC-047 and OPS-007), the tool definition presented as a self-contained bundle, narrowed by the
target's template (IFC-033, IFC-012), and the root of an ordinary Operation's schemas (CTR-012).
The independent check read the published protocol on each of the three; the editors' logs say
what they read. Round 5's own decisions that the owner may want to read first are in
[its record](proposals/reaudit/round-5/README.md#known-after-the-round), where V2-15 is now
reversed by ruling 44.

## Purpose

Ruled 2026-11-05 (ruling 15).

This section is kept in the words of the ruling, except one sentence that ruling 39 amended.
Since rulings 27 to 29 there is no separate grant: a grant is the access an open Conduit carries, to issue one is to open a Conduit, and to
revoke one is to close it. "Provision" is setting up, and a plugin is an implementation. Read
the section with those words.

**The relationship is the product.** Infrastructure languages such as Kubernetes, Docker Compose
and Terraform declare workloads, which do what their specification says. An agent does what it
decides. An agent-first language therefore declares what surrounds the agent: what it may reach,
under which limits, and who answers for what it did. That is why the family that carries grants
gives the project its name.

- **First user:** a single developer running agents from several vendors on their own machine.
- **Teams:** not in v0, but the language must be able to support them later, most likely through
  Interfaces. No rule may assume there is exactly one owner.
- **Enforcement:** a future, optional part of the language. By default a restriction applies to
  what passes through Conduit. An author can opt in to a selected isolation framework through the
  language and plugins, as with Docker, to have it enforced.

What is distinctly Conduit's, and must survive the rework:

- **Establish once, share under restriction.** A login, checkout or memory is owned by a Pool and
  used by agents from any vendor, each under its own limits, with the credential never handed
  over.
- **The Conduit is an object.** Each agent's access to each resource can be inspected, narrowed
  and revoked independently of both ends. (Amended by ruling 39, point 7: the ruling said "the
  grant".)
- **Agents can operate the system.** An agent can provision a resource, start a peer and delegate
  a narrower slice of access, through the same contract a human uses.
- **Nothing implicit spends tokens or repeats an effect.** Actions are explicit, an unknown
  outcome stays unknown, and there is no background reconciler or automatic wake.
- **Private and shared stay separate.** An agent's subscription and context are its own; anything
  shared goes through a Resource.

Not differentiating, and so free to simplify: the graph engine, the package system, the schema
profile, persistence and export. For graphs and schemas the owner's standing instruction
(ruling 23) is to do nothing revolutionary and to rely heavily on existing frameworks and
paradigms; the same applies to packages (ruling 24).

### Preservation tests

Accepted 2026-11-05 (ruling 17). Each rework decision is checked against them.

Since rulings 27 to 29 a grant is the access an open Conduit carries, and "provision" is setting
up; read the tests with those words. The tests themselves are not reworded.

| # | Test |
| --- | --- |
| 1 | An agent is a participant, never a workload: the language does not specify its reasoning. |
| 2 | Every access is a relationship with a grant that can be inspected and revoked. |
| 3 | A shared thing is owned once and never copied to an agent. |
| 4 | Nothing assumes a particular vendor's agent. |
| 5 | Every management action is available to an agent under a grant, and delegation only narrows. |
| 6 | Nothing starts model work or repeats an effect implicitly. |
| 7 | Everything an agent does through Conduit is attributable. |
| 8 | No rule assumes exactly one owner: a human's access is an Interface holding grants. |
| 9 | A restriction always reports whether it is enforced or only mediated, and enforcement is opted into through the language. |

Two additions to the shared conventions were accepted with them. There is a seventh convention,
for test 6: nothing starts model work or repeats an effect implicitly. And the access convention
states, for test 5, that an agent may issue grants to others only within a bound it was given.

## Scope guard

The acceptance scenario is the first thing that must work. It does not define what Conduit is.

- Passing the scenario removes nothing from the accepted model. A capability is not dropped or
  narrowed because the scenario does not exercise it.
- "Never exercised by a scenario" is evidence about sequencing, not about scope. Moving anything
  to later scope, or dropping it, takes an explicit owner ruling (A18).
- The object model is checked against scenarios unlike the acceptance scenario before any
  pruning (S3 to S9), so that a design fitted to one scenario is caught early.
- Simplifications made to reach the scenario sooner are recorded here as shortcuts with the
  general behavior they stand in for, so they can be replaced rather than inherited.

| Shortcut taken for the first slice | General behavior it stands in for | Status |
| --- | --- | --- |
| — | — | — |

## Current direction

The [decisions in force](../IN_FORCE.md) state the direction by topic. Conduit innovates only on
connection and access (ruling 40); other infrastructure is a provider behind a contract. Work that
makes the language concise, concrete and provable comes first.

Later work is outside v0: no operation arranges through the engine work that outlasts its invocation
(ruling 38 point 6). Policies, composite Conduits, requirement composition, `obliges`, re-exports and
rename are outside v0 (ruling 41). Policies remain a candidate access feature (A66).

The first user is one developer with agents from different vendors, on one machine; this must not
narrow the accepted scope. Native macOS paths come first; cross-machine support is claimed only
with a test across two machines. The first build is the core plus shell of ruling 46. Its S1
simplification rerun precedes Fleet authority and the remaining scenarios; publication waits for
ruling 47's whole gate and the owner's decision. [ROADMAP.md](../ROADMAP.md) gives the sequence.

## Ruling log

The [ledger of rulings](proposals/ruling-log.md) holds every ruling, its status and historical appendices.
[IN_FORCE.md](../IN_FORCE.md) holds every other decision in force and accepted language changes marked spec pending.
The specification alone binds behavior; the owner's words govern a conflicting IN_FORCE line until fixed, and builds follow spec/ until pending changes are written (ruling 67).

### Later amendments

See the ledger's [status column](proposals/ruling-log.md) and [historical amendment tables](proposals/ruling-log.md#history-later-amendments).

#### Changes that no ruling has accepted

Historical heading retained for links. Ruling 58 point 1 accepted the [round-5 changes](proposals/ruling-log.md#history-round-5-changes), with its stated exceptions.

## Layered claim

Ruling 42 renamed the runtime profile the conformance document, and operation identity the
idempotency key (vocabulary audit, N18 and N15); this section and its table of layer decisions
keep the words of rulings 13 and 14.

Ruled 2026-11-05 (ruling 13). An earlier working claim, that the documents determine one
observable outcome for every situation, is the bar for a programming-language specification. It
is withdrawn. Conduit is a configuration language, and by that bar even Kubernetes would fail:
its control plane has one implementation, its conformance suite tests a core subset, and most of
its behavior lives in controllers rather than in the language.

**The claim:** the language fully determines what a set of declarations means: which objects
exist, how they relate and which invariants always hold. How a runtime gets there is the
runtime's contract.

| Layer | What it holds | Comparable part of Kubernetes | How precise |
| --- | --- | --- | --- |
| Language | The five families, template and instance, identity and references, instantiation, placement, Conduits and grants as declared data, schemas | Resource schemas and API conventions | Fully specified; small |
| Shared conventions | The few behaviors every part must agree on, such as operation identity, the outcome set, revisions and what a grant permits | Resource versions, optimistic concurrency, finalizers | Specified as invariants, not step by step |
| Type contracts | How each type behaves: an Agent's conditions (running, paused, idle), a directory's edge cases, message read positions | Each controller's behavior | Owned by the type's author; versioned separately |
| Runtime profile | Engine choices: outage handling, retries, cursors, process layout | The reference control plane's behavior | Decided and tested for the reference engine; other engines may differ |

Behavioral decisions are still made and recorded, in the type contracts and the runtime profile
rather than in the language. The critical concepts are unchanged: Pools owning Resources and
shared logins, Conduits carrying grants, placement, instantiation, explicit actions rather than
a background reconciler, control and observation through the Interface, and plugins over MCP.

The specification holds three layers: language, convention and interface. The third is the
implementation interface of layer decisions LD3 and LD4, which this section calls the plugin
interface. Type contracts and the runtime profile stay outside the specification. This section
and the table of layer decisions below are kept in the words of rulings 13 and 14: read
"declarations" as documents, "grant" as the access an open Conduit carries, "plugin" as
implementation, and "normative surface" as the specification. MCP is the Model Context
Protocol.

Ruling 40 adds one layer beside these: **provider contracts**. A provider is existing
infrastructure that the user chooses (a secret store, a runtime, a set of machines, a source of
packages), reached through a contract that Conduit states and any tool can implement, as
Kubernetes reaches runtimes, storage and networks through CRI, CSI and CNI. The contracts belong
to the specification; the providers do not. What a runtime profile held about process layout and
placement now belongs to the chosen provider.

### Layer decisions

| ID | Decision | Ruling | Status |
| --- | --- | --- | --- |
| LD1 | The test that assigns a rule to a layer. | Language if it can be answered by reading the declarations alone. Shared convention if two independently written parts would break each other, or a safety promise, by disagreeing. Type contract if it matters only to instances of one type. Runtime profile otherwise. | done: accepted as recommended |
| LD2 | Where setup graphs sit. | Keep graphs. The language defines their static meaning (steps, bindings, dependencies, validity) and two invariants: a step never runs before its prerequisites succeed, and setup succeeds only if every required step does. Scheduling, retry, cancellation and partial outputs move to the runtime profile. | done: accepted as recommended. Overtaken: ruling 40, point 3c, reduced graphs to declared dependencies, and ruling 41, item 1, removed them. An instance's own setup and retirement work are two hooks |
| LD3 | The list of shared conventions, and whether one engine's internal services must work with another's. | Six conventions: operation identity and outcome set; revision-checked change; no managed access without a recorded grant, checked at the target, with restrictions only narrowing; declarations distinct from observed state, with no effect from registration; single Pool ownership and retire-before-unregister; an inspectable record of every managed operation. Cross-engine interoperation of internal services is not a v0 goal; the plugin interface is the shared wire. | done: accepted as recommended. There are seven conventions since ruling 17 |
| LD4 | What the two independent builds must cover. | The language layer in full (declarations in, resolved objects and diagnostics out), the shared conventions through vectors, and the plugin interface. One reference engine; a second full engine is optional later. | done: accepted as recommended |
| LD5 | Where each layer is written down. | The language and shared conventions in the normative surface as promises with vectors. Each type contract with its type, in a standard library shipped beside the reference engine. The runtime profile with the reference engine. | done: accepted as recommended |

## Priorities

The priorities group the action items by theme. The order of the work is the
[roadmap](../ROADMAP.md#the-order-of-the-work).

| Priority | Theme | Why it comes here |
| --- | --- | --- |
| P0 | Make the language concrete, complete and free of known defects | The owner's goal, and the precondition for everything else. |
| P1 | Prove the language can be rebuilt and its promises kept | The engine, vectors, model checking and the two engine builds; native probes run alongside. |
| P2 | Prune and re-baseline, under the scope guard | Uses the evidence to re-sequence what the first slice does not need. |
| P3 | Vocabulary | Moved up by ruling 16: decided now and applied with the rework. |

## Action items

Open, in progress and ruled items are below, except the experiments and probes: they are
action items too, and their rows are in the roadmap's
[experiments and probes](../ROADMAP.md#experiments-and-probes). Completed and superseded items
are in [the history](archive/plan-audit-history.md#completed-and-superseded-action-items),
except the experiments and probes, which the roadmap lists among
[what is done](../ROADMAP.md#done-removed-dropped-or-not-adopted).

Promise ranges and counts in the status notes are as first written; the current files and
counts are in [spec/README.md](../spec/README.md#contents).

### P0 — make the language concrete and complete

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A32 | Define the specification: the exact set of documents from which an engine must be rebuildable. | Goal, Q2, Q10 | The contents list exists and everything outside it is marked as not binding (G1). | in progress: all areas written; secrets since ruling 41. The specification alone binds. |
| A33 | Build the promise register: restate accepted behavior as numbered promises, each with a vector. | Goal, D8 | Every topic is converted, with word counts before and after (G2). | in progress: all areas written; S1 vectors only, gaps in the [manifest](../conformance/manifest.yaml); word counts not done. |
| A42 | Re-triage the defects by layer and close those in the language and shared conventions, per the [defect closure plan](LANGUAGE_DEFECTS.md#triage-by-layer). | Rulings 12, 13 | Readiness condition G7, as ruling 38 restated it: the latest round of readers reports no unsafe finding, and the core and the vectors agree with the promises. It was: two consecutive sweeps of the language and shared conventions are clean. | in progress: triage complete; 87 language/convention rows. Gate judged after the first build. |
| A51 | First batch of fixes: relationships and access. Eight decisions and sixteen rules are proposed in the [relationships record](proposals/relationships/README.md). | Rulings 15, 17 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: written; remaining vector coverage in the manifest. Composite Conduits and policies removed by ruling 41. |
| A13 | Second batch of fixes: operations, outcomes and revisions. Five decisions and nine rules are proposed in the [operations record](proposals/operations/README.md). | Q6, L1, L10 | It is ruled, model-checked and swept clean. | in progress: written, S1 vectors; promises not model-checked. Experiment 004 evaluated the tool against older text only. |
| A52 | Third batch of fixes: declarations. Five decisions and thirteen rules, in the [declarations record](proposals/declarations/README.md). | Ruling 14 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: written, S1 vectors; rename removed, fixed definitions remain (OBJ-011). |
| A53 | Fourth batch of fixes: graphs and the schema profile. Four decisions and nine rules, in the [graphs and schemas record](proposals/graphs-and-schemas/README.md). | Ruling 14 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: schema profile remains, S1 vectors; graphs removed by ruling 41. |
| A54 | Fifth batch of fixes: packages, first start and conformance. Five decisions and eight rules, in the [packages and start record](proposals/packages-and-start/README.md). | Rulings 14, 23 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: written, S1 vectors; PD1 settled by 25 and 27; missing packages reject, no re-exports. |
| A59 | Consolidate the promises and audit the whole specification again with cold readers who see only `spec/`. | Ruling 30 | As restated by ruling 38, point 4: the latest round of readers reports no unsafe finding, and the core and the conformance vectors agree with the promises. It was: two consecutive rounds with no finding in the language, conventions or interface, which four rounds showed reading alone does not reach. | in progress: six rounds, none clean; all findings settled. The convergence rule above governs the next read. |
| A61 | Audit the documents outside `spec/` and make the repository ready for a cold read. | Ruling 37 | Every decision of the audit is applied, and a cold reader can tell from the entry points what binds. | in progress: DA1 to DA13 applied; later entry-point findings fixed. Done when a cold reader reports nothing misleading. |
| A41 | Fix the uncertain-write rule (L1) with A13. | L1 | The convention states when reissue is safe and a vector covers it. | ruled: OPS-012 to OPS-014; S1 vectors, broader coverage pending. |
| A12 | Name the target MCP revision and specify the implementation interface and its profile. Whether the wire between an engine's own services is shared is layer decision LD3. | Q2, G9 | An implementation written against the interface works with any conforming engine. | ruled: baseline 2026-07-28; S1 shell and vectors, full interface coverage pending. |
| A11 | State what the language requires of caller authentication, then choose the mechanism from the native probes. | Q3 | Both are in the specification. | in progress: IFC-022, IFC-030, IFC-042 and DL13 state proof; A4 tests how native Agents obtain and present the token. |
| A16 | Conduit instance granularity, participation and grant lifetime. | Q5 | Ruled in the layer it is assigned to and swept clean. | ruled: pairwise Conduits; closure coverage incomplete. Closed Conduits removed (LIF-035). |
| A25 | Agent lifecycle, including what survives replacement. | Q8 | Ruled in the layer it is assigned to and swept clean. | ruled: replacement has a new ID and none of the retired instance's Conduits; own declared Conduits open at setup; vector pending. |
| A17 | State what runs with no frontend attached and what starts it. | Q7 | It is a promise. | open |
| A35 | Specify the declare-lost operation, which is reserved to the root Interface. | Ruling 7 | Its contract, fencing and effects are promises. | ruled: PLC-017; provider-reported loss also acts through PLC-024; vector pending. |

### P1 — prove the language can be rebuilt and its promises kept

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A68 | The first build: a working demonstration, not another round of review of the prose. The strict loader in Rust first (A43, ruling 45), then the S1 scenario through a core in Rust (A38), conformance vectors for the safety cases first (A39), and the first providers (A63), with round 6's evidence cases placed in the [roadmap](../ROADMAP.md#round-6s-evidence-and-measurements). Each run measures what a new author must do: the files edited, the concepts met, the grants made by hand, the adapters written, and the steps of recovery that need a person (D22 of round 6). | Ruling 44 | S1 runs end to end on the engine and the first providers, its safety vectors pass, and the author measurements are recorded with dated results, beside what they show for the simplifications that round 6 proposed. Report the disposition and S1 cost effects: none of round 6’s applied D06, D07, D11, D12, D14, D18 and D21 saved an S1 author line. Ruling 41’s cuts 3a (policies), 10 (rename) and 17 (removal after retirement) moved cost into T4, T5/T6 and T7. Compare these costs again on the rerun ([map-round6 §1](proposals/first-build/increment-4/measurement-plan.md#the-s1-rerun)). | in progress: increments 1 to 3 done; both increment 4 authors ran T1 to T7 but failed the authoring bar. Rerun follows simplification. Record what [round-6 simplifications actually changed](proposals/first-build/increment-4/measurement-plan.md#the-s1-rerun): applied changes saved no S1 author lines; immutability, naming and root-Conduit costs were measured, not removed. |
| A38 | Write the executable core in Rust for the language layer and the shared conventions: a pure, deterministic library in which every case cites a promise. | Rulings 10, 14 | It passes every vector with no unhandled case (G4). | in progress: S1 core runs; S2 to S9 extend it. Measures keyed, unkeyed, failed and unknown invocation storage for A14 (D19). |
| A39 | Publish the conformance vectors, with a held-out set. | Ruling 10 | Any engine can run them through an adapter; coverage by promise is reported. | in progress: [manifest](../conformance/manifest.yaml) indexes coverage. PLC-024's loss cause needs its portable record field. Lean fixtures remain proposed: [measured costs and migration constraints](../conformance/README.md#lean-vectors); A79 is already the short-form spike in the roadmap. |
| A44 | Mutation-test the core against the vectors. | G6; [resolver probes](../conformance/README.md#evidence-of-vector-strength-so-far), [regression mutations](../conformance/README.md#evidence-of-vector-strength-so-far), [record-omission probes](../conformance/README.md#evidence-of-vector-strength-so-far) | No unexplained surviving mutant. | open; [mutation results and limits](../conformance/README.md#evidence-of-vector-strength-so-far). |
| A40 | Model-check the safety promises with Stateright in small configurations, in the ordinary test run, including two machines with message loss, reordering, a partition and a lost machine (ruling 40). | [Experiment 004](../experiments/004-stateright-evaluation/README.md) | Each safety promise holds or has a recorded reason it cannot be checked this way (G5). | in progress: the harness, [experiment 012](../experiments/012-model-checking/README.md) (work order 104); the tool was evaluated in experiment 004 |
| A45 | Publish the build base: the orphan branch, the brief as `BUILD.md`, agent instruction files, and a pilot run of the brief on S1. | Ruling 12 | G10 passes and the owner signs off (G11). | open: brief and process drafted |
| A34 | Two independent builds in Rust, by Codex and by Claude, of the language layer, the shared conventions and the implementation interface (LD4). | Rulings 10, 14 | Both pass the public and held-out vectors; every divergence is traced to the language and fixed. | open: blocked by the readiness gate |
| A50 | Build one reference engine, with a standard library of core type contracts and a recorded conformance document. | LD4, LD5; [Resource contract source material](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/RESOURCES.md#L148-L250) | It runs the scenarios S1 to S10. | in progress: S1 core and shell; remaining scenarios on this codebase. [Choices and limits](../engine/CONFORMANCE.md). |
| A62 | Write the provider contracts of ruling 40 in the specification: the runtime that runs an instance's implementation (run, stop, status); the machines, their labels and the loss of one; the secret store; the source of packages. | Ruling 40 | Each is a contract any existing tool can implement, and the specification names no backing system. | in progress: provider and secret contracts written under 41 to 43; broader vectors pending. |
| A63 | Build the first providers, as packages in Rust under a top-level `packages/` directory, one crate per provider (ruling 40, after the ruling): two for each provider contract, on different existing tools. Candidates: secret stores on a file (the file store of the standard library: under ruling 41, item 13a, no store is built into the engine, and none reads the engine's own variables), the macOS keychain and 1Password; runtimes on a local process and Docker; machines on the local machine and a second host reached over the network; package sources on Git and a local directory. | Ruling 40 | Each passes the vectors of its contract, and S10 passes with every provider swapped. | open: needs A62. Where its providers come in the order of the work is in steps 3 and 4 of the [roadmap](../ROADMAP.md#the-order-of-the-work). For the keychain store, what [Experiment 005](../experiments/005-macos-keychain/README.md) measured, in the user's logged-in session, and what it means for the package, is in [its record](../experiments/005-macos-keychain/README.md#what-it-means-for-a-macos-keychain-provider-package); a real background daemon is still untested (A22). The 1Password store has its own probe (A73). What round 6 measures of the providers under ruling 44 (D01, D15 and D16 of round 6, and a real package whose server repeats a named argument, MAN-010) is placed in the roadmap's [table of round 6's evidence](../ROADMAP.md#round-6s-evidence-and-measurements), under S2, S7 and S10 |
| A64 | Require proof of the sender on every request that crosses machines, in both directions, with the mechanism swappable (mutual TLS, SPIFFE, the authorization of the Model Context Protocol). | Ruling 40, point 3e; ruling 43 (D5, Q1) | Written as promises; one vector shows a request to the engine without proof refused, and one shows a direct call to a gated server, from anything but an enforcement point, refused. | in progress: IFC-022, IFC-032, IFC-042, PRV-025 and DL13 written; no cross-machine gate run shown. Plain remote servers stay outside the gate, taking no Conduit keys or sender proof (IFC-007, IFC-041); S2/S3 measure credential exposure. |
| A65 | Design local enforcement for v1: each machine enforces access from configuration pushed to it, with a stated bound on how long a revocation takes to arrive. | Ruling 40, point 3f; ruling 43 (Q4) | A proposal for the owner, before v1. Nothing in v0 may assume that every decision is made in one place. | in progress: [v1 design and changed promises](proposals/multi-machine/README.md#in-v1-for-a65), Q4's leased local decision state and monotonic clock. No wider spec survey was made; detailed proposal still to write. |
| A66 | Design policies again as an access feature of Conduit's own, for a later version: a limit placed on an open Conduit without closing it. | Ruling 41, on item 3a of the principles review; [authorization reasoning](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/AUTHORIZATION.md) §2/§4 and [cloud authority](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/research/CLOUD_AUTHORITY.md) §9: an Agent never inherits its launcher's full rights; a skipped required restriction is never satisfied; revocation needs admission and consistency; administering a grant differs from using it; Cedar, OPA and Biscuit are options behind contracts | A proposal for the owner, before the version that would carry it. | open |
| A67 | Consider three candidate features that no one has asked for yet, which the questions after the rewrite turned up: a forced removal of an instance whose shutdown can never be confirmed; a verb that restarts a ready instance's run and keeps its identity and Conduits, as `kubectl rollout restart` does; an address owned by the project that first submitted it. | [Open questions after the rewrite](proposals/ruling-41/open-decisions.md) | A proposal for the owner, if one is wanted. | open. Round 6's D09 measures a credential's rotation by explicit replacement first, in S2 and S10; if that is disproportionately disruptive, the restart verb comes to the owner with its exact semantics |
| A14 | Run the acceptance scenario on a built engine and use it for a week. | Ruling 4 | Each promise is scored exercised, contradicted or never needed. | open. With it, the state kept for every invocation, measured by the core (D19 of round 6), is projected for the week's workload; dropping any of it would change OPS-008 and is the owner's |

### P2 — prune and re-baseline, under the scope guard

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A18 | Present promises that no scenario exercised as candidates for later sequencing, each for an owner ruling. Nothing leaves the accepted scope by default. | D1 | Each candidate under [deferrable scope](archive/plan-audit-history.md#deferrable-scope) has a ruling. | open |
| A20 | Rule on v0 authorization scope. | D5 | The cross-path, issuer-independence and provider-scope questions each have an answer. | open |
| A24 | Adopt a review rule: a new ruling carries a vector that can fail. | D8 | The contributor guide states it. | open |
| A27 | Citation hygiene. | [Reference repositories](archive/plan-audit-history.md#reference-repositories) | The Pi pins are confirmed upstream and the Pi helper probes are retained or their claims removed. | open |
| A80 | Carry out the rest of [the condensation plan](proposals/condensation/plan.md): its removal orders, steps 4 to 11, and its close. | Ruling 58 | Every step is merged, each removed path is in [the archive's index](archive/README.md#removed-from-the-tree), and the plan's working papers have condensed into the records. | open |

### P3 — vocabulary

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A29 | Apply the chosen names as each area is converted into the specification, recording old and new spellings in the vocabulary map. | A28 | No area is rewritten twice for naming. | in progress: ruled names applied; [retired meanings](../spec/definitions.md#words-the-promises-do-not-use) kept; no mass renaming of history. |

### Organization and publication after v0

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A75 | Bring the v0 engine and the full experiment evidence to the owner for a publication decision. | Ruling 47 | The required experiments have passed, observational findings are recorded, and the owner has judged whether the system works and is worth publishing. | open |
| A76 | Carry out the organization transition and first public preview. | Ruling 47 | The project owns the main repo and public namespace, and a released runtime, an independently released official integration and an external package-author path meet the [preview criteria](proposals/ruling-47/README.md#actions-after-a-decision-to-publish). | open: after a positive A75 decision and the owner's [account steps](guides/move-to-conduitlang.md). |

### Done since ruling 39

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A72 | Show the roadmap where a reader finds it: the order of the work, what comes next, and each experiment and probe with its state and what it feeds. | Ruling 45 | A reader can tell from the entry points, without reading this tracker, what is next and whether an experiment they have in mind is already planned. | done 2026-11-09: roadmap at root, ruling 45. |
| A69 | Apply D06 and D12 of round 6 as the owner accepted them, revised, after ruling 44: a package's own Operation contract states no default implementation, and the engine implements the operations of the core package by rule; names stay the only address, and `set-up.v1`, `retire.v1`, `inspect.v1` and `unregister.v1` take an optional expected ID, which reports conflict where the object that the name resolves to has another. | Ruling 44, [after the ruling](proposals/ruling-44/README.md#after-the-ruling) | Written as promises, in the core package, the schemas and the examples, and logged beside round 6's decisions. | done 2026-11-07: D06/D12 revised and applied; CTR-002, TPL-003/004, LIF-037; LIF-031 unchanged. |
| A43 | Write the schemas and a strict loader in Rust. | G3 | Every scenario file validates and the negative cases are rejected. | done 2026-11-09: strict loader, increment 1. |
| A21 | Repair status and consistency defects in the documents. | Q10 | Each item under [known inconsistencies](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/plan-audit-history.md#known-inconsistencies) is fixed or dropped. | done: ruling 55 dropped the five remaining inconsistencies. |

## Experiment program

The experiment program, accepted by ruling 9 and amended by later rulings, among them 16, 40,
41, 44 and 45, is in the roadmap: its
[claim, methods and scenarios](../ROADMAP.md#the-experiment-program), the
[order of the work](../ROADMAP.md#the-order-of-the-work) and the row of every
[experiment and probe](../ROADMAP.md#experiments-and-probes). It moved there under A72, and
[the log](proposals/ruling-45/log-roadmap.md) says what moved from where.
### Round 6's evidence and measurements

Round 6's evidence cases and measurements, placed by scenario, are in the
[roadmap](../ROADMAP.md#round-6s-evidence-and-measurements).
