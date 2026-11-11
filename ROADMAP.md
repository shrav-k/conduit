# Roadmap

Not binding. The specification in [spec/](spec/README.md) alone binds, and this page states no
rule of the language and defines no behavior: it cites promises and action items by their IDs.

This is the experiment program that ruling 9 accepted and later rulings amended, among them 16,
40, 41, 44 and 45. It moved here from the [plan audit](docs/AUDIT.md) under A72 (ruling 45).
The owner rules the order of the work, and the coordinator updates the states in the pull
request that changes the work. Action items keep the plan audit's numbers, and an experiment's
or a probe's row on this page is its only row; every other action item has its row in the plan
audit's [action items](docs/AUDIT.md#action-items) or, if ruling 39 moved it there as done or
superseded, in its [history](docs/archive/plan-audit-history.md#completed-and-superseded-action-items).

## The order of the work

Steps 1 to 8 are the order that ruling 9 accepted, as later rulings amended it: step 4 and S10
came with the provider layer of ruling 40, and what rulings 44 and 45 added is written inside
step 3. The steps keep their present numbers. Ruling 9's work in parallel and the rows added
since carry no number, so no step number shifts. A step's state is done, now (it may run
today), next or later. Each item keeps its own row and state, in the
[register](#experiments-and-probes) below or in the plan audit.

| Step | Work | Items | Needs | State |
| --- | --- | --- | --- | --- |
| 1 | Shared foundations, in series: the layer decisions (A48), the form of the specification and of a promise (A47), then the defect re-triage (A42). | A48, A47, A42 | — | done: A42's triage is done, and A42 stays in progress until its done-when, G7, is met |
| 2 | The language and shared conventions, each ruled, checked and swept. | Ruled batches and questions: A51, A13, A41, A52, A53, A54, A12, A16, A25, A35. Writing the specification: A32, A33, A62, A64, A11, A29. Questions and repairs: A17, A20, A21, A24, A27 | The owner, for A20 and A21 ([waiting for the owner](docs/AUDIT.md#waiting-for-the-owner)) | done as a step, as the coordinator's note: every area is written as promises (A33), and ruling 44 put the build next. Its items are not done: some questions and repairs are still to settle, the vectors begin in step 3 (A39), model checks come in step 6, and the next sweep is the read after step 3 |
| In parallel | Throughout, what needs no engine, or only real accounts and elapsed time: the native probes; since ruling 45, the baseline for S1 and the probe of a real client; the keychain's daemon case; the experiment across two machines, which prepares S3; and, since ruling 48, the work orders that Codex workers carry out. Since ruling 58, the condensation of the repository's records (A80). | A70, A71, A4, A5, A6, A7, A8, A10, A22, A74, A77, A80 | No engine; real accounts and elapsed time for the native probes; real machines for A74 | now: each row of the [register](#experiments-and-probes) gives its own state |
| 3 | Tracer, in series: S1 alone through every method, to fix the file formats for scenarios, vectors and promises, on the first provider of each contract. Ruling 44 makes this the first build (A68), with the vectors for the safety cases first and the author measurements of D22 of round 6 on every run. Ruling 45 puts first the strict loader in Rust (A43), which validates the documents under `spec/schemas/examples/` against the schemas (X4 of pull request 10); then come the core on S1 (A38), the vectors (A39) and the first provider of each contract (A63). The probe of 1Password with no person present (A73) runs beside it once the first providers start. Ruling 46 settles its nature: one codebase, a pure core and a thin shell that makes it runnable and is the first slice of the reference engine (A50), built in four increments that [the first build](docs/implementation/FIRST_BUILD.md#increments) pre-registers, with its success criteria and measures. Ruling 48 has Codex workers carry out the increments as [work orders](docs/implementation/WORK_ORDERS.md), runs A4 before increment 3, and sends the work orders through Conduit itself after Fleet authority (batch 2), as ruling 61, B12, settles (A78). | A68, A43, A38, A39, A63, A73, A50, A78, A79 | The engine's built-in providers (ruling 46); Codex installed and signed in, for increment 3 | now: increments 1 to 3 are done; [the first-build register](docs/proposals/first-build/README.md) links their findings and dated evidence.<br>Increment 4 remains in progress: both author runs failed the bar; simplification, then the S1 rerun under [its measurement plan](docs/proposals/first-build/increment-4/measurement-plan.md), precede Fleet authority. |
| Read | The next read by cold readers, on the text as the build leaves it, with reports under `docs/proposals/reaudit/round-7/` ([where findings go](docs/AUDIT.md#for-the-next-reader)). | A59, A61 | Step 3 | later |
| 4 | Second providers: a second implementation of each provider contract, on a different existing tool, and S10 (A63). | A63 | Step 3 | later |
| 5 | Fan-out, in parallel: S2 to S9, one scenario per session, with one integrator. S3 runs on at least two real machines. | A50; A74 prepares S3 | Step 3; the same codebase as S1 (ruling 46) | later |
| 6 | Whole-model checks: model checking, including the safety promises across machines under loss, reordering and partition, and mutation testing. | A40, A44 | A38, A39 | now: A40's harness is [experiment 012](experiments/012-model-checking/README.md), begun under ruling 71; its models follow; mutation testing (A44) later |
| 7 | The readiness gate, then the two engine builds. | [G1 to G11](docs/implementation/INDEPENDENT_BUILDS.md#readiness-gate), A45, A34 | The owner's sign-off (G11) | later |
| 8 | Prune. | A18 | The owner rules on each candidate | later |
| After | The acceptance scenario on a built engine, used for a week. | A14 | A built engine | later |
| Publication | The owner reviews v0 against the full adopted experiment program and judges whether Conduit is worth publishing. A positive decision starts the organization transfer, official package releases and the [public preview plan](docs/proposals/ruling-47/README.md). | A75, A76 | A v0 engine and complete experiment evidence for A75, then the owner's decision to proceed for A76 | later |
| Later versions | Proposals for the owner, each before the version that would carry it: local enforcement for v1, policies again, and three candidate features. | A65, A66, A67 | The owner | later |

### Open in the order

Nothing is open in the order. Ruling 46 settled the one question that was: S2 to S9 run on the
same codebase as S1, whose shell is the first slice of the reference engine (A50).

## Experiments and probes

The register. Every experiment and probe, planned, running or done, has one row here, and that
row is the item's only row. A probe of the plan audit is written with its earlier name, as in
"probe E10 of the plan audit"; its first wording is in the
[history of the plan audit](docs/archive/plan-audit-history.md#standing-probes-and-comparisons),
whose statuses are as ruling 39 left them. Round 6's evidence cases and measurements are cases
of the scenarios, [placed below](#round-6s-evidence-and-measurements); E17 of round 6 runs
early, as A71. Of the five experiments that pull request 10 proposed, X1, X3 and X5 are A73, A70
and A71 below, X2 is among what was dropped, and X4, a validator in Rust of the example
documents, is the strict loader A43, the first piece of the first build in
[step 3](#the-order-of-the-work). Results go in the item's own directory under
[experiments/](experiments/README.md), which this table links and does not summarize.

| Item | Question it settles | Needs | Feeds | Done when | State | Source |
| --- | --- | --- | --- | --- | --- | --- |
| A70 | The baseline for S1, before the tracer: what do S1's tasks take, set up and then changed as the chapter on change and repair of D08 of round 6 does, with the tools people use today, a coding agent's own permission settings and its configuration of MCP servers, and a Compose file? It counts what the S1 row of [round 6's table](#round-6s-evidence-and-measurements) counts: duplicated credentials, per-agent changes of access, inspection effort, provider-specific code and steps of recovery | No engine | The bar that the first build's author measurements (A68; D22 of round 6) are compared with | Dated results in [007-s1-baseline](experiments/007-s1-baseline/README.md), on the tasks and measures of [the first build](docs/implementation/FIRST_BUILD.md#the-tasks), each counted as defined there before it runs, which the first build's author measurements are then compared with | done on 2026-11-09: [results](experiments/007-s1-baseline/results-2026-11-09.md) | Ruling 45 (X3 of pull request 10, from `allocmemory`) |
| A71 | What a real client shows its model, before the engine exists: a real coding agent reaches, over loopback Streamable HTTP with a bearer token, a stub in Rust of the engine's presented server that answers as IFC-012, IFC-013, IFC-025 and IFC-038 prescribe. The cases: a tool of a Conduit closed after a `list_changed` notification, called from a cached list; a call to a tool never presented; a tool failure; an unknown outcome; and, added on 2026-11-09 from the review of A70, a Conduit opened mid-session. Each case is run headless and, where it can be, interactive; the protocol revision each client negotiates is recorded; Claude Code's discovery cache and Codex's cached catalog (`startup_readiness`) are each tried on and off; the stub states a `ttlMs`; and each client reads its token from a file by its header helper, including after a 401. What the client receives, what it passes to its model and what the model does next, the refusal of a call included, are recorded apart. A work order builds the stub; the admin session runs the clients | No engine | E17 of round 6, in S2, run early; it overlaps A4 and A6 | Dated results under `experiments/`, for E17 in S2 | done on 2026-11-09: [dated results](experiments/008-real-client/results-2026-11-09.md), 68 runs, classified by a work order and reviewed by the admin; the [later-turn follow-up](experiments/008-real-client/results-2026-11-09-later-turn.md) ran 12 two-turn sessions that day, its worker classifications reviewed by the admin before merge | Ruling 45 (X5 of pull request 10, from `allocmemory`); E17 of round 6; A6 |
| A4 | Native launch and identity: can native Agents be launched with private configuration and a Conduit endpoint? Claude Code and Codex are started with separate configuration homes and a per-Agent Conduit token, the identity of IFC-042 and not a login to a model. Since ruling 48 it runs before increment 3, isolating each client by the flags of A70, and records for each: the wrapper that lets the runner reach it (PRV-007), which flags keep the owner's configuration out, its working directory, how its header helper reads the token file, and which login it used | Real accounts | If not, no real Agent exists until the owned loop; how a native Agent obtains and presents its token (A11, IFC-042); see A71 | Results recorded for Claude Code and Codex | done on 2026-11-09: [results](experiments/009-native-launch/results-2026-11-09.md), including two failed Codex attempts and the observations still unshown | Probe E1 of the plan audit |
| A5 | Four accounts for 48 hours: do two Claude and two Codex subscriptions coexist under one macOS user? | Four accounts, 48 hours | If they collide, Agents need separate OS users or containers; the later two-Claude, two-Codex account check | Refresh and logout behavior recorded per account | open | Probe E2 of the plan audit |
| A6 | Mediation, including a lost response: does a small proxy holding one provider login serve two Agents with different allowlists? | The engine: S2's and S4's runs | Whether the idempotency key, then called operation identity, survives clients that send none: D02 and E09 of round 6, in S4; see A71 | A trace exists for discovery, invocation, refresh and a lost response | open: its traces come from the engine's runs, by the admin's decision of 2026-11-11, logged for the owner to strike: discovery, invocation and refresh (as the shared key's rotation) from [S2](experiments/013-s2-shared-login/README.md), and the lost response from [S4](experiments/015-s4-partial-setup/README.md) | Probe E3 of the plan audit |
| A7 | Bypass, and the honest v0 security claim: what can a native Agent bypass? An Agent is told to reach data it is denied, and every path that works is recorded. A70's T3 is partial evidence: Codex's read-only shell reads any file the user can, its login brings apps that no file declares, and Claude Code's messaging and scheduling tools need no permission; on the engine's own runner, an Agent can also read another run's token file (IFC-022) | Real accounts | The honest security claim; A20, the owner's ruling on the scope of authorization | The claim is a promise | open | Probe E4 and disagreement D5 of the plan audit |
| A10 | Messaging and a next-turn notice: does messaging progress without wake, through history tools and a next-turn notice given through an Agent's hooks? | Real accounts | Whether automatic wake moves into v0 | A two-Agent exchange is recorded with the human prompts it needed | open | Probe E5 of the plan audit |
| A8 | Subscription terms for the owned model loop, and since ruling 48 for native clients that the engine runs headless: may they use subscriptions? | A desk check of the current terms | Whether the owned loop is viable as planned | The owned loop is confirmed, changed to API keys or dropped | done on 2026-11-10: [Experiment 010](experiments/010-subscription-terms/results-2026-11-10.md) confirms headless use of each unmodified client on its user's own subscription, names API keys as the alternative, and finds a Codex login cannot be shared between Agents; to be read again before v0 is published | Probe E11 of the plan audit |
| A22 | The keychain with no person present: does the macOS keychain work unattended, from a daemon, a rebuilt unsigned binary and a locked session? | No engine; a Mac, and a real background daemon for the case still open | The keychain store of A63, and the custody of credentials | Results recorded | in progress: [Experiment 005](experiments/005-macos-keychain/README.md) recorded results on 2026-11-06, in the user's logged-in session; a real background daemon is untested (A63) | Probe E10 of the plan audit |
| A73 | 1Password with no person present: can an item be read with no prompt answered, by a service account and by the desktop app's session, on a throwaway account and vault? | The first providers started (A63); [Experiment 005's safety rules](experiments/005-macos-keychain/README.md#safety); the owner's approval before the case that needs the desktop app | The 1Password store of A63 | Dated results under `experiments/`, for the service account and for the desktop app's session, with each measure defined before it runs | open: runs later, beside step 3, once the first providers start | Ruling 45 (X1 of pull request 10, from `allocmemory`), which put it under A63 |
| A74 | Across two machines, with no engine: do the mechanisms under S3 hold on real machines? Among hypotheses H1 to H10 of [section 11 of the multi-machine proposal](docs/proposals/multi-machine/README.md#11-an-experiment-across-two-machines): a call without the engine's proof refused at a gate, a removed instance's credential refused, a stream after a silent cut, an OCI digest kept, dispatch at most once between two stand-ins and a gate behind NAT | Two real machines on one network and, for H10, a third behind NAT | S3 (step 5), which needs an engine; A64; A40 | Dated results in `experiments/006-across-machines/`, stating for each hypothesis what was measured | open: no one has started it | Ruling 43, which accepted the multi-machine proposal |
| A77 | Work orders: when Codex workers carry out the work under the admin session's orders, is the work as good, and does the admin session spend less? | Codex signed in | How the rest of the program is run (ruling 48) | Each order's row in [work orders](docs/implementation/WORK_ORDERS.md#the-record): the order, verifier findings, admin corrections and recorded times; reviewed after five orders | in progress as of 2026-11-11: each merged order is a row of [the record](docs/implementation/WORK_ORDERS.md#the-record), covering 001 to 036k and 060 to 066; [lessons](docs/implementation/WORK_ORDERS.md#what-this-round-taught) are recorded, but admin time was not recorded, so no complete spend comparison is available | Ruling 48 |
| A78 | Work orders through Conduit after Fleet authority (batch 2): each worker an Agent of the `conduit-core` Fleet, fixed worker and tree slots set up once by root; orders 1 to 3 dispatched by root, orders 4 and 5 by a coordinator Agent of that Fleet appointed by root, granting each tree through the Fleet's Conduits and driving workers through turns root opened; one queue per Fleet; no slot retires | Agents that run code (ruling 70, order 050a) and authority through Fleets (041b) for root's orders 1 to 3; appointment (batch 2) for orders 4 and 5 | S1 in daily use; findings of the first build | Five orders run through Conduit, each friction logged as a finding of the first build | open: brought forward by ruling 70 on 2026-11-11; awaits 041b and 050a | Ruling 48, amended by [ruling 61, B12](docs/proposals/ruling-61/README.md); [ruling 70](docs/proposals/ruling-70/README.md) |
| A79 | Does S1 in the short form translate into canonical documents today's loader and core accept, and how many lines does its author write in flow and block YAML? | Today's loader, core and shipped S1 fixtures | Ruling 55's translation rules and increment 4's simplification | All task states checked in both styles; dated counts with limits in [experiment 011](experiments/011-short-form-spike/README.md) | done on 2026-11-10: [results](experiments/011-short-form-spike/results-2026-11-10.md), 0 exact states passed / 14 failed (`as` unsupported); 14 diagnostic probes without `as` passed; paper totals 55 flow / 97 block, not an author run | Ruling 55; work order 031 |
| [Experiment 012](experiments/012-model-checking/README.md) | The harness for A40: explicit loss, reordering, duplication, partition, crash and a lost machine, and the checks every model of the safety promises uses, tested on one model of its own | No engine | A40 and G5: the models of the safety promises, beginning with operation identity | The harness and its own model pass in the ordinary test run, with each variant's verdict and state counts in dated results | in progress: work order 104, in the proof lane of ruling 71 | Ruling 71; A40 |
| [Experiment 023](experiments/023-paper-count/README.md) | PB: a paper count of S1's tasks in flow style against the baseline A70, under ruling 63 and ruling 68's R1, with ruling 71's bar applied only as a forecast | Order 069's CLI reference; experiment 011's ledger | The design of the flow style and the CLI; never the bar | Dated counts per task, primary and strict, with block style beside | open: work order 073, after 069 merges | Ruling 71; A79 |

### Done, removed, dropped or not adopted

| Item | What it asked | State | Record |
| --- | --- | --- | --- |
| Experiment 004 | Is Stateright, a model checker embedded in Rust, a way to check the safety promises under loss, reordering and crashes? | done on 2026-11-05: a tool evaluation, not evidence about a Conduit engine; it feeds A40 | [004](experiments/004-stateright-evaluation/README.md) |
| A19 | Run the graph-need comparison, probe E8 of the plan audit, and rule on authored graphs in v0 (disagreement D6 of the plan audit); done when the ruling is recorded with line and concept counts | superseded: closed by ruling 41, item 1, which removed graphs from v0 and with them the comparison E8 and the disagreement D6 of the plan audit. The comparison was not run | [Principles review](docs/proposals/principles-review/README.md), which ruling 41 accepted |
| Experiments 001 to 003 | Shared access on paper, with static fixtures (001); a plan for runtimes and plugins (002); probes of JSON Schema and YAML libraries (003) | removed: 001 on 2026-11-05, once [spec/schemas/examples](spec/schemas/examples/) had replaced it as the worked example (decision DA4 of the documents audit); 002 and 003 by ruling 39 | [Archive index](docs/archive/README.md#removed-from-the-tree) |
| X2 of pull request 10 | An author's effort, counted on paper | dropped by ruling 45: the first build takes the author measurements of D22 of round 6 on every run (A68), and with no engine there is nothing to run the documents on | [Ruling 45](docs/proposals/ruling-45/README.md#the-points-and-the-answers) |
| Sealmesh; a local root Agent; a VM per environment | Candidates in the collaborator's pull request 1: a messaging backend, an optional Agent that administers the host, and a VM for each environment as an isolated way to run | not adopted: the review of pull request 1 left each as a candidate, to be evaluated before any use | [Review of the collaborator's proposals](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/collaborator-proposals/REVIEW.md#additional-roadmap-candidates) |

## Proposing an experiment

- Check this page first: the [order of the work](#the-order-of-the-work), the
  [register](#experiments-and-probes), the [scenarios](#scenarios) and
  [round 6's cases](#round-6s-evidence-and-measurements). One idea may already be filed under an
  action item, a scenario or a case of round 6. A proposal names the row it serves, or says that
  it is new and what it feeds.
- Each experiment states, before it runs, what it must show and how each of its measures is
  counted (ruling 45).
- A new item gets the next A-number of the plan audit when it is accepted, and its row here.
- A directory under `experiments/` takes the next unused number, counting the numbers this page
  holds (006 for A74). No number is reused.
- Whoever starts an item writes `in progress` in its State, with the branch or the pull request
  that carries the work.
- A probe of a real secret store follows
  [Experiment 005's safety rules](experiments/005-macos-keychain/README.md#safety), and a case
  that needs a person's session comes to the owner before it runs.
- The [experiments index](experiments/README.md) gives the rules every experiment follows.

## The experiment program

Accepted 2026-11-05 (ruling 9), and amended by later rulings, among them 16, 40, 41, 44 and 45.
Its order is
[the order of the work](#the-order-of-the-work) above.

**Claim to prove:** the [layered claim](docs/AUDIT.md#layered-claim), with the provider layer of
ruling 40. The language fully determines what a set of documents means; the shared conventions
hold as invariants, on one machine and across several; everything else is a type contract, a
provider or the conformance document. And every provider can be swapped: each provider contract
has at least two implementations, built on different existing tools, that pass the same vectors.

**What v0 must pass.** Ruling 47 lists the experiments that v0, the reference engine, must pass
before the owner judges whether to publish it (A75): [the required experiments](docs/proposals/ruling-47/README.md#when-to-publish).
The other probes are observational and do not block publication.

| Method | What it shows | Action |
| --- | --- | --- |
| Promise register with must, should and may wording and an explicit implementation-defined list | Every behavior is pinned or openly left to implementations | A33 |
| Executable core in Rust | Completeness: an unhandled case is a hole in the language | A38 |
| Public conformance vectors | Any implementation can be checked | A39 |
| Mutation testing of the model against the vectors | Whether the vectors actually pin behavior | A44 |
| Model checking of safety promises under faults | The promises can all hold together | A40 |
| Cold-reader sweeps | Places where two builders would diverge, found cheaply before any build | A59 |
| Two providers for each provider contract, built on different existing tools | The infrastructure really is swappable: the same documents behave the same on either | A63 |
| Multi-machine runs, with a partition and a lost machine | The access promises hold across machines | S3, A74, A40 |
| Independent builds by Codex and by Claude of the language layer, shared conventions and implementation interface | Unambiguity, by the recognized public standard of independent implementations passing one suite | A34 |

### Scenarios

| # | Scenario | Mainly exercises | Step |
| --- | --- | --- | --- |
| S1 | Two Agents share a project directory and a message history on one node | All five families, instantiation, communication, inspection | 3 |
| S2 | One service login shared by two Agents with different restrictions ([pre-registration](experiments/013-s2-shared-login/README.md)) | Requirements, Conduits, shared authentication, dependent calls | 5 |
| S3 | S1 and S2 across two machines, including a partition and a lost machine. Central to the claim since ruling 40 ([pre-registration](experiments/014-s3-two-machines/README.md)) | Placement by a provider, what a lost machine retires and closes, unknown outcomes across the network, proof of identity across machines | 5 |
| S4 | Setup that fails partway, is retried, and loses a response ([pre-registration](experiments/015-s4-partial-setup/README.md)) | The parts of a setup and its setup hook, which replaced graphs (ruling 41, item 1), outcome model, recovery | 5 |
| S5 | A control change under contention ([pre-registration](experiments/016-s5-control-contention/README.md)) | Revisions, a control that applies exactly what it was sent or fails, which replaced partial confirmation (ruling 41, item 15), lost receipt | 5 |
| S6 | Access revoked and a Resource retired while requests are in flight ([pre-registration](experiments/017-s6-revoke-in-flight/README.md)) | Current-access checks, retirement duties | 5 |
| S7 | An outside author packages an existing MCP server and a small code plugin ([pre-registration](experiments/018-s7-outside-package/README.md)) | Packages, contracts, types, label selectors for placement, which replaced "location needs" (ruling 41, item 8a) | 5 |
| S8 | Search over storage returning only what the caller may see ([pre-registration](experiments/019-s8-filtered-search/README.md)) | Restrictions on returned content, derived content | 5 |
| S9 | First initialization, later start, and explicit repair ([pre-registration](experiments/020-s9-start-and-repair/README.md)) | Root Interface and foundational setup | 5 |
| S10 | S1 and S2 again with every provider swapped: another secret store, another runtime, another package source ([pre-registration](experiments/021-s10-swapped-providers/README.md)) | Provider contracts (ruling 40) | 4 |

The scenarios exercise every layer, so they run on the reference engine with its standard
library of types (A50) and the first providers (A63). The acceptance scenario is S1 and S2
passing there. The independent builds inform the owner's judgment without gating v0
([ruling 47](docs/proposals/ruling-47/README.md#when-to-publish)). Running them with real Claude and Codex Agents, and the two-Claude, two-Codex account
check, are later gates. Step 3 runs S1 through the core and its shell, which grows
into the reference engine on which S2 to S9 run (ruling 46).

The vocabulary is no longer a last step: ruling 16 moved it up, and it was decided and applied
with the rework.

Independence rules for the builds, accepted (ruling 11) and detailed in
[independent builds](docs/implementation/INDEPENDENT_BUILDS.md): each builder is a fresh session
that did not write the language; its only inputs are the specification and the vectors; it
cannot see the other engine, the reference engine's source, or anything else in this repository;
and the owner or a neutral harness compares the results.

### Round 6's evidence and measurements

Every E and D in this section is round 6's.

Ruling 44 put round 6's evidence cases, E01 to E20, and the qualification tables of its D01 and
D02 into this program, with D05, D08 to D10, D15, D16, D19 and D20 as measurements. Each case is
described in
[the critique](docs/proposals/reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage);
the assertion given here is what the specification requires since round 6's decisions, or the
question the case settles. None has been run. Every run also records the author measurements of
D22: the files edited, the concepts met, the grants made by hand, the adapters written and the
steps of recovery that need a person.

| Scenario | Evidence cases | Measurements and qualification |
| --- | --- | --- |
| S1 | E05: a tool whose schema refers to a contract with local `$defs`, two contracts with one local name, a recursive contract and a `$ref` with keywords beside it; the definition presented is a self-contained bundle that a value passes exactly when it passes the contract's schemas (IFC-033). E08: an agent may start only an approved reviewer, through a broker, with its trusted code and setup counted (D04). E11: a local contract and its implementation edited, then the template replaced while an old instance exists; the replacement resolves the project submitted with it, and the old instance keeps what it resolved (D11). E12: an object inspected, removed and its name reused before a follow-up request; once D12 is applied as revised (A69), a request that carries the expected ID reports conflict, and an ID grants nothing | D08: a chapter on change and repair, which changes a live setting, upgrades a template, replaces a nested Resource and restores the intended access, with the exact calls, identities and permissions. D19: the bytes kept for each invocation (A38). A comparison with a manually configured baseline on the same tasks, the same agents and the same service, counting duplicated credentials, per-agent changes of access, inspection effort, provider-specific code and recovery steps (A70) |
| S2 | E02: a read succeeds before a filter's deadline and is repeated after it, with the same Conduit and arguments; the read is dispatched again and refused, and no stored output is delivered (OPS-009). E07: an agent with a filtered read and a bound over the same operation opens an unfiltered Conduit, which the bound allows (D03). E13: the shared service's secret rotated with both agents working, the service replaced, and the restoring of access failing partway; no secret reaches an Agent's run. E17: what a real client and its model see after a revoke with cached tools, a protocol refusal and a tool failure; A71 runs it early | D01: a plain server, a small adapter and a Conduit-aware implementation qualified on one user task, with the adapter work counted (also S7, A63). D05: the first restrictions as complete contracts with vectors shared by two implementations, covering changed authorization data, a repeat, onward calls and revocation. D09: rotation by explicit replacement (also S10). D20: what a local deployment and an isolated one of the same relationships each protect, and which credentials an Agent process can read (also S3) |
| S3 | E15: a machine reported unreachable, then gone, with partitions before and after an effect and before and after a revocation; unavailable, unknown, retired and stopped stay distinct, and nothing new is dispatched without current authorization | D20, under a partition |
| S4 | E01: a plain server makes part of an effect, then answers -32021, once directly and once as a task's failed error; the outcome of an operation that states an effect is unknown, and so is that of one marked `startsModelWork: true` that states none, and a repeat with the same key is not dispatched (IFC-010, OPS-012). E09: a client that keeps its key and one that sends none, each repeating after a lost response. E18: the owning Pool retired, the Resource inspected and reconciled, the instance removed and reconciled again; each step authorized as ACC-033's table says. E19: records purged and outputs expired, then an old keyed request repeated and an unknown invocation inspected; no effect is repeated. E20: a hook's reply lost, its outcome reconciled failed, a new attempt for another caller, then the earlier attempt's late onward request, decided for the latest caller (OPS-028, A65) | D02: clients that keep the key and clients that send none, against plain and deduplicating implementations, after a success whose response was lost, after an unknown outcome, and for an intended second action. D10: what the actor can conclude, verify and do next after an unknown outcome, a conflict and a partial setup |
| S5 | E03 and E04: a control whose `changes` gives two inputs one setting, or one a setting inside another's; the control is rejected (CTR-006). E10: a control narrowed by its template to 50 under a contract that allows 100, asked for 75, then left out of the template's map of controls; the tool presented shows the narrowed schema, and no tool is presented for the excluded control (IFC-033, IFC-012). E16: an omitted input, null, an empty map, a nested default, an array element and an omitted control input; keyed repeats compare one normalized value (D13); and a `default` that only a `$ref` at the root of an operation's `inputSchema` reaches, which fills nothing (SCH-009, CTR-012) | D10: what `ruleId`, the invocation's identity and its status let a frontend tell apart in a conflict |
| S7 | None of E01 to E20. Round 6's decisions add a real package whose server repeats a named argument, such as `-e`, before any way to bind one occurrence is added (MAN-010, A63) | D01, as in S2 (A63) |
| S10 | E06: two package-source providers that would claim one scheme; a Type that lists a control changing `schemes` is rejected, so a claim is fixed for the provider's life (PRV-016), and a second claimant is refused (PRV-013). E14: S1 and S2 with each provider swapped, with every provider-specific edit listed | D15 (A63): for each provider, a record of what it implements itself, what it delegates, what persistent state it needs and what the operator configures. D16 (A63): a comparison for S10, defined before it runs, with the provider-specific edits and the replacements it allows, and what a swap keeps. D09, on the swapped store |
