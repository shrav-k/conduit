# X08-one-run: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Reviewed at 2eb8f258 (2026-11-12).

**Does spec/ hold one run per instance ID wherever it can?** Largely yes. PRV-019 and PRV-030 close MC4-009 on the provider's side and PRV-007 on the engine's; with one provider for an instance's life (PRV-006), a provider that keeps both starts no second run for an ID. Three things undercut it: the owed-stop sentence says "fails" where PRV-019 says unknown (F1); a run has no stated extent, so a stop can succeed over live descendants (F5); and neither promise can be tested outside the kit's fake-backend tests (F9).

**Is MC4-002's overlap stated to whoever is affected?** It is stated honestly in spec/README.md:597-606 and in LIF-009, LIF-036 and PRV-021, with the StatefulSet precedent. No caller is told when it happens (F6). The owner's mitigation (IN_FORCE.md:244) works for kit-built providers mainly because the kit ties a run to its channel and owes a stop when the channel drops, not through PRV-021. PLC-023 keeps a retirement's stop from the provider while the machine is reported unreachable (spec/promises/placement.yaml:305-307), so PRV-021's `when` is reached only through LIF-036 or an unreported cut. The rule is also per provider instance: a later instance naming another runtime provider on the same machine is not held. No record says either.

**PRV-021 against PRV-029 and LIF-035.** Not a contradiction. Removal, not retirement, deletes the state directory, and a retire hook that does not succeed keeps a retired provider and its directory (F2). SSH-R's operator-set directory and external drainer follow from a misleading observable and an over-read `then` (F3); they are not the right answer (F4).

**Existing tools.** systemd already gives SSH-R a lifetime bound enforced with no provider alive (`RuntimeMaxSec`), a stop that cancels queued starts and restarts, and units listed by name for a sweep at reconnection. The kubelet keeps no list of owed stops; it compares what runs with what the API server holds. launchd has no lifetime bound, and a local provider never loses its machine. The drainer adds one window only (F4).

**Kubernetes and Nomad in MC4-002's situation** (from memory, not fetched). Both let the replacement run beside the original by default. Kubernetes marks an unreachable Node's Pods for deletion after the toleration (300 s by default); a Deployment's replacement starts at once and the old Pod runs until its kubelet reconnects and kills it. A StatefulSet waits until the old Pod is confirmed gone, unless an admin force-deletes it or taints the Node out-of-service. Nomad replaces a disconnected client's allocations at once; `stop_on_client_after` has the client stop them itself, and `disconnect` can withhold the replacement or pick one on reconnection. Conduit took the Deployment's default for everything, by the owner's ruling; I report nothing against that beyond F6.

**Language, or a provider's business?** Split. The engine's half (PRV-007 to PRV-009, LIF-025, LIF-036, TPL-018, PLC-023) and the contract that makes an ID name one run (PRV-019, PRV-030) are language: the CRI of `runtime-provider.v1`. PRV-021's durable-place clause, its 27-line observable and the owed-stop cases are a provider's business written into the language: a `must` on prose no engine reads. They should shrink to the outcomes the engine reads and a pointer to the provider's contract; the three cases belong in the kit's README, where order 141 put them.

**What the reader read.** Read in full: docs/guides/full-review.md; AGENTS.md "What holds"; PRV-001 to PRV-010, PRV-013, PRV-018 to PRV-021 and PRV-025 to PRV-030; LIF-009, LIF-017, LIF-024 to LIF-026, LIF-035 and LIF-036; TPL-009, TPL-012 and TPL-018; PLC-023; OPS-014, OPS-021, OPS-022 and OPS-024; REC-001 and REC-002; IFC-017, IFC-025 and IFC-050; DL5, DL16 and DL17; the contracts `run.v1`, `stop.v1`, `inspect-run.v1`, `unregister.v1`, `register-replacement.v1`, `set-up.v1` and `retire-hook.v1`; the left-behind-run limit in spec/README.md and the form of a promise; ruling 42's D11, its log items and later answers; ruling 72's later answers; order 081 (to step 4b) with its Record; orders 129, 130 and 141; both package READMEs; the Records of orders 137, 140, 145, 146, 149 and 150 and order 145's rationale; experiment 022's README, case form and runtime index; the known-open lists in docs/AUDIT.md, docs/guides/cold-readers.md and IN_FORCE.md, the review list and HANDOVER's strike list.

Read in part: docs/implementation/PROVIDERS.md (kit, decisions A to D, order table); G5.md (MC4 rows only); docs/proposals/multi-machine/README.md (D11 section); order 080 (its handling of a running run with no url); packages/runtime-provider-kit/src/server.rs lines 372-470.

Not checked: nothing was built or run. The engine's own runner was not read; the 081 Record's statement that it implements neither new guarantee is taken as given. The MC4 model and its traces were not read. SSH-R's source was measured by line count only, so F4's judgement rests on its README and Records. Facts about Kubernetes, Nomad, systemd, launchd and CRI are from memory. PRV-025 was read and gave no finding in this area. The "labels last reported" wording was left as queued.

Recorded elsewhere and not reported: IN_FORCE.md has no line citing PRV-019 or PRV-030, and order 081's settlements are still to be added to the review list (docs/implementation/HANDOVER.md:253-259).


## FR-227

**The owed-stop sentence says "fails" where PRV-019 says unknown, in three places**

X08/F1 · contradiction · unsafe · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** PRV-021, PRV-019, PRV-007

**Where.** spec/promises/providers.yaml:662

**Quote.** "Under such a contract, a start under `run.v1` that finds a stop still owed fails, naming that stop, and may be retried once the stop is carried out (PRV-019)."

**Situation.** Confirmed as queued, with two places the queue does not name: the same unsplit sentence is in `run.v1`'s description (spec/core/operations.yaml:885-887) and in spec/core/README.md:417-420. PRV-019's `then` (providers.yaml:579-580) says "Where it cannot tell whether that run is running, it starts none and the call ends unknown". Steps to the harm: instance I's stop is held because its machine is cut off; `inspect-run.v1` answers `stopped` just before the cut returns, or the provider cannot read its own state; the engine asks `run.v1` for I. A builder following PRV-021's sentence answers failed. The engine reads failed as "nothing is run", the setup or retirement can end failed, OPS-024 lets a repeat go as a new attempt, and nothing lists the invocation as unreconciled, while I's earlier run may still be running with its secrets. A builder following PRV-019 answers unknown. The kit implements three cases (packages/runtime-provider-kit/README.md:205-211; src/server.rs:385-452). SSH-R's contract prose still describes the answer before order 141, which names no stop (packages/provider-ssh-runtime/README.md:181-184).

**Counterargument.** An observable does not override a `then`, and PRV-019's `then` already gives unknown. PRV-007 asks for a run only after `inspect-run.v1` reports `stopped`, and a provider that owes a stop for the ID reports `unknown` there, so the case is reached only by a race or an unreadable ledger. The kit already answers as PRV-019 says.

**Least change.** Replace the sentence at providers.yaml:662-664 with: "Under such a contract, a start under `run.v1` on a machine on which the provider still owes a stop starts no run, and ends as PRV-019 has it end. Where the stop is owed for the call's own ID and has not been carried out, that run may still be running, and the call ends unknown. Where the provider cannot tell whether a run that it started for the call's own ID is running, as where it cannot read what it keeps of its runs, the call ends unknown. Where it can tell that no run of the call's own ID remains, and the stop is owed for another ID, it cannot run the item on that machine yet, and the call ends failed. The text of each such result names every stop that holds the start, by its instance's ID and its machine's hostname, or every stop that the provider still owes on the machine where it cannot tell which hold it, and says that the start may be asked for again once those stops are carried out. A stop owed for the call's own ID that the provider carries out before it starts the run does not hold the start." In `run.v1`'s description and spec/core/README.md: "a start that finds a stop still owed there starts none: it ends unknown where the stop is owed for the call's own ID, or where the provider cannot tell whether a run of that ID is running, and failed where the stop is owed for another ID alone; its text names each such stop (PRV-019, PRV-021)." Then correct SSH-R's README:181-184.

**Precedent.** PRV-019's own `then`; the kit's order 141, decisions 2 and 3 (docs/work-orders/141-a63-rk-own-owed-stop.md:49-61); OPS-014's rule that unknown is for an effect that may stand.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction whose fix adds no behavior, since PRV-019's `then` already decides each outcome. Queued for the next specification order (docs/guides/full-review.md:132).

**Verifier.** weakened, medium. Quote verbatim at spec/promises/providers.yaml:662-664. The same unsplit sentence is at spec/core/operations.yaml:884-887 and spec/core/README.md:417-420, as stated. PRV-019's `then` at spec/promises/providers.yaml:579-580: "Where it cannot tell whether that run is running, it starts none and the call ends unknown". Already recorded: docs/guides/full-review.md:132-134 (queued) and docs/work-orders/141-a63-rk-own-owed-stop.md:22-24 ("the admin corrects PRV-021's sentence in a spec order") and :37-38. packages/provider-ssh-runtime/README.md:181-184 is stale after order 141, as stated; the kit's three answers are at packages/runtime-provider-kit/src/server.rs:410 and :448. Against the unsafe mark: `stop.v1` is asked only by PRV-008 and LIF-036 (spec/promises/providers.yaml:282, spec/conventions/lifecycle.yaml:747), so a stop is owed for an ID only once its instance is retired or removed; LIF-015 refuses a setup of a retired instance (spec/conventions/lifecycle.yaml:312-314); the only later `run.v1` for that ID serves a retire hook, and only where `inspect-run.v1` reports `stopped` (spec/promises/providers.yaml:215-216, 264-265), which its contract allows "only where it knows that no run that it started for the ID is running" (spec/core/operations.yaml:1029-1031); TPL-009 then asks the stop "whatever came of the hook" (spec/promises/templates.yaml:148-150), and that stop ends unknown while the machine is cut off.

**Verifier's corrections.** Class stands: a contradiction between PRV-019's `then` and an observable sentence repeated in `run.v1`'s description and the core README. Unsafe mark: remove. A provider that keeps `inspect-run.v1`'s contract is never asked to start an ID whose stop it still owes, and no run starts under either reading; the step "the setup or retirement can end failed" does not hold (no setup of a retired instance; the retirement's stop still ends unknown). What remains is a failed answer where unknown is right, on one provider call, in a branch reached only by a provider that reports `stopped` wrongly. Known mark is right: queued; new are only the two further places and SSH-R's README:181-184. Wording: the six-sentence draft lengthens an observable the reviewer's own assessment says should shrink, and its first sentence ("on a machine on which the provider still owes a stop starts no run") conflicts with its last (an own stop carried out first "does not hold the start"). Use the short form the reviewer drafted for `run.v1` in all three places and leave the three cases in the kit's README. Coordinator decides, as stated.


## FR-228

**PRV-021's observable says a retired provider's server never runs again; a retire hook runs it again and keeps its state directory**

X08/F2 · defect · verifier: confirmed, medium · decides: coordinator · new

**Promises.** PRV-021, PRV-020, TPL-018, TPL-009, OPS-021, LIF-035, PRV-029

**Where.** spec/promises/providers.yaml:652

**Quote.** "A stop that only that server carries out so waits while the server does not run, and is never carried out once the provider is retired; a contract under which the provider does what it states through both keeps the stop where something that runs without that server carries it out"

**Situation.** The text (lines 648-658, repeated at PRV-020:617-619 and spec/README.md:592-596) holds only for a provider whose template selects no retire hook. A provider is "a Resource like any other" (PRV-001, providers.yaml:30). If its Type requires `retire-hook.v1` and the hook carries out owed stops and fails while one remains, then: the hook runs before the provider's own stop (TPL-009); a failed hook "keeps the shutdown from being confirmed" (templates.yaml:165-166); the instance "is not removed: it stays, retired" (lifecycle.yaml:731); PRV-029 deletes its state directory only at removal; and each further retirement asks for its run and dispatches the hook again (TPL-018, OPS-021: "one that failed is dispatched as a new attempt"). So stops kept in the engine-supplied state directory outlive the provider's retirement, and the engine's restart on its machine or another (DL16, DL17), with nothing outside the lifecycle. Order 145 reasoned from the observable ("retiring the provider removes its instance ... so no stop still owed can be carried out after retirement", docs/work-orders/145-ssh-r-durable-dir.md:16-18) and moved the state to an operator-set directory. So PRV-021 and PRV-029 with LIF-035 do not contradict: removal, not retirement, deletes the directory.

**Counterargument.** No provider built so far selects a retire hook, so the sentence is true of each. A hook-held provider is carried forward only when an actor retires it again, where PRV-021's example carries the stop out once the machine returns. A retired provider that cannot be removed is a cost of its own.

**Least change.** Replace providers.yaml:651-654 "and does not run again once the provider is retired ... is never carried out once the provider is retired;" with: "and, once the provider is retired, runs again only to serve its retire hook, in its retirement and in each further one while its shutdown is not confirmed, under TPL-018 and LIF-026. A provider whose retire hook carries out the stops it still owes, and does not succeed while one remains, naming each, is not removed until none remains or an actor unregisters it, under LIF-035, and keeps its state directory until then, under PRV-029, as a Kubernetes finalizer keeps an object until its controller has cleaned up: stops kept there outlive its retirement. A provider with no such hook is removed when its retirement succeeds, and a stop that only its server carries out is then lost;" and keep the rest. Make PRV-020:617-619 and spec/README.md:592-596 agree.

**Precedent.** Kubernetes finalizers, which LIF-017 already cites (lifecycle.yaml:354-355); Terraform's destroy-time provisioner, which TPL-009 already cites (templates.yaml:167-168).

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect whose fix adds no behavior; TPL-009, TPL-018, OPS-021 and LIF-035 already give it.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/providers.yaml:652-655; line 651 says the server "does not run again once the provider is retired". Against it, TPL-018 at spec/promises/templates.yaml:304-306: "Where the hook is to be dispatched, its implementation is an item that is run and its runtime provider reports the run `stopped`, the engine first asks for the run, under PRV-007"; TPL-009 at :164-168 (a hook that did not succeed keeps the shutdown from being confirmed, and a further retirement dispatches it again); OPS-021 at spec/conventions/operations.yaml:454-455 ("one that failed is dispatched as a new attempt"); LIF-035 at spec/conventions/lifecycle.yaml:730-732 ("is not removed: it stays, retired"); PRV-029 at spec/promises/providers.yaml:974 (the directory goes at removal or unregistration). PRV-001 makes a provider "a Resource like any other" (:29-30), and nothing in spec/ keeps a provider's Type from requiring `retire-hook.v1`. Order 145's reasoning is at docs/work-orders/145-ssh-r-durable-dir.md:15-18. New: no mention of a retire hook for a provider in docs/AUDIT.md, docs/guides/cold-readers.md, the review list, increment 4's log, G5.md, PROVIDERS.md decision D (:345-356) or orders 137, 145 and 146.

**Verifier's corrections.** Two precisions. (1) A retirement whose hook failed and whose own stop succeeded ends unknown, not failed, under OPS-022 (spec/conventions/operations.yaml:463-465); the instance stays retired either way. (2) The stop is then carried out only when an actor retires the provider again under LIF-026, so the observable's "waits while the server does not run" stays true of the unattended case; the replacement text should say so and be shorter than drafted. The fix must also reach spec/conventions/lifecycle.yaml:781-783 ("a stop that the provider owes outlives ... the provider's own retirement") and `stop.v1`'s description (spec/core/operations.yaml:1003-1007), which the finding does not list. Class and who decides stand.


## FR-229

**PRV-021's `then`: must every provider keep an uncarried stop durably, or only state what it does?**

X08/F3 · ambiguity · verifier: weakened, low · decides: coordinator · new

**Promises.** PRV-021, PRV-030, PRV-010

**Where.** spec/promises/providers.yaml:635

**Quote.** "and where it keeps the stop meanwhile, in a place that neither a restart of the engine, on its machine or on another, nor the provider's own retirement loses; and it does what it states."

**Situation.** Reading A: every runtime provider keeps each stop it cannot carry out, in a place that outlives the engine's restart and its own retirement. SSH-R and the plan took it ("Each backend names an external mechanism that drains them after the provider server never runs again", docs/implementation/PROVIDERS.md:173-174; external drainers also planned for RP2 and DR2a, lines 131 and 139). Reading B: the contract states what becomes of the run, and the place clause binds only a provider that keeps the stop. The same promise supports B: "and then keeps nothing on the engine's side" (line 657-658), "this obliges no provider to carry it out beyond what its contract states" (line 660), and PRV-030 (line 1003-1004). So does the text ruling 43 accepted: "A provider whose part on the machine stops the run itself ... needs to keep nothing on the engine's side" (docs/proposals/multi-machine/README.md:857-859). Under B, SSH-R could state that systemd ends each run at `RuntimeMaxSec` and keep nothing. The `must` also binds prose "which no engine reads" (spec/definitions.md:404), so nothing can check it, against rule 2 of the form of a promise (spec/README.md:298).

**Counterargument.** The `then` is one sentence whose plain reading makes the place mandatory, and ruling 43's text says "Its contract states where it keeps them". The provider's own part on the run's machine is also a place, so the Nomad example fits reading A.

**Least change.** Rewrite the `then`: "Its contract states what becomes of that run, and it does what it states: either that the provider keeps the stop and carries it out once it can reach the machine, whether or not the instance still exists by then, and where it keeps it meanwhile, in a place that neither a restart of the engine, on its machine or on another, nor the provider's own retirement loses; or what on the run's machine ends the run without the provider, and within what time."

**Precedent.** Nomad's `stop_on_client_after`, which PRV-021 already cites; systemd's `RuntimeMaxSec=`, which SSH-R already sets on every unit.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity settled in the words ruling 43 accepted; no behavior added.

**Verifier.** weakened, low. Quote verbatim at spec/promises/providers.yaml:635-637. The observable already points to reading B: :657-658 ("and then keeps nothing on the engine's side"), :660-661 ("this obliges no provider to carry it out beyond what its contract states"), :1003-1004. The record agrees: docs/proposals/ruling-42/README.md:178 ("the contract states what the provider does with it, carrying it out being the example. The stronger form is the other option") and docs/proposals/multi-machine/README.md:855-859. SSH-R keeps stops because the owner's answer has shipped providers carry out owed stops (IN_FORCE.md:244), so the place clause binds it under either reading; order 145 rests on the observable's retirement claim (docs/work-orders/145-ssh-r-durable-dir.md:15-18), which is F2, not on reading A. docs/implementation/PROVIDERS.md:173-174 speaks of owed stops, which fits both readings. On "nothing can check it": spec/README.md:322 lists PRV-021 among the promises of the Implementation level, and "it does what it states" is checked against the provider's behavior.

**Verifier's corrections.** The grammar of the `then` is loose, but the observable and both records settle reading B, and no builder is shown to have taken reading A as "every provider must keep the stop". Smaller than stated: a clarity fix. The drafted `then` ("either ... or ...") is not a no-behavior fix: it removes a contract that states anything else, such as that the provider does nothing, which is the stronger form that ruling 42's integration log set aside. As drafted it amends D11 and is the owner's. A coordinator's fix keeps "such as" and makes the place clause conditional: "...; where it keeps the stop, its contract states where, in a place that neither ... loses; and it does what it states", repeated in `stop.v1`'s description (spec/core/operations.yaml:1003-1007), spec/README.md:588-590 and spec/conventions/lifecycle.yaml:781-783. Drop the rule-2 argument.


## FR-230

**SSH-R's operator-set durable directory and external drainer duplicate what the retire hook and systemd already give**

X08/F4 · needless-complexity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** PRV-021, PRV-029, PRV-030, TPL-018

**Where.** packages/provider-ssh-runtime/README.md:189

**Quote.** "The required `durableDir` / `SSH_RUNTIME_DURABLE_DIR` is outside that lifecycle: under the admin's decision of 2026-11-12 for PRV-021, it retains owed stops and the identity of the remote part after provider retirement."

**Situation.** Orders 145, 146, 149 and 150 added a required operator directory, a second binary, `owner.lock`, `drainer.lock`, `yield`, `drainer-config` and a key-removal watch: `src/drainer.rs` 291 lines, `src/durable.rs` 296, `tests/drainer.rs` 924. Order 149's Record says "each round of orders 146 and 149 found another race between drainers handing over". What it buys is one window: a machine that heals before `RuntimeMaxSec` while the provider's server is not running. Every run already ends on its machine at `maxLifetime` (README:66-67, 91-92); the kit drains at each call and interval while the server runs; the provider sweeps on each reconnection before admitting runs (README:92-95); and the drainer makes its last attempt at `maxLifetime + stopTimeout` (README:153-155). It "leaves the engine runner's process group by design" and still dies when the engine's service cgroup stops or the machine reboots (README:128-130), so it does not survive the engine restart PRV-021 names. It is a child that outlives its run's stop, which the kit forbids of the runs it starts ("Stop the entire tree", kit README:47). The directory sits outside the engine's store, so DL17's move to another machine carries it only if the operator arranges it. PROVIDERS.md plans the same helper for two more providers (lines 131, 139, 253).

**Counterargument.** The drainer is what PRV-021's observable describes, and the owner asked that shipped providers carry out owed stops. It stops a run without an actor when the machine heals early, where `RuntimeMaxSec` may leave it up to an hour. A retire hook that fails leaves a retired provider until an actor retires it again or unregisters it.

**Least change.** SSH-R's Type lists `retire-hook.v1` under `requires` with a default that carries out owed stops and fails, naming each, while one remains. The ledger, markers, part and `known_hosts` return to `CONDUIT_STATE_DIR`. Remove `durableDir`, the drain binary, the two extra locks, `yield`, `drainer-config` and the key watch; drop order 147's sweep and the external helpers planned for RP2 and DR2a. The contract states that systemd ends a run on its machine at `RuntimeMaxSec` whatever the provider does. Depends on F2 and F3.

**Precedent.** Kubernetes finalizers; systemd transient units with `RuntimeMaxSec=`; the kubelet, which keeps no list of owed stops and compares what runs with what the API server holds.

**Who decides.** coordinator. The directory (order 145) and the drainer (PROVIDERS.md decision D) were the admin's own decisions under ruling 71, and PROVIDERS.md binds nothing; ruling 40 prefers an existing paradigm. The owner's answer on owed stops (IN_FORCE.md:244) is kept. Log it for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at packages/provider-ssh-runtime/README.md:189-191. Sizes recounted: src/drainer.rs 291, src/durable.rs 296, tests/drainer.rs 924. Record quote at docs/work-orders/149-ssh-r-drainer-finish.md:82-83. Every run ends on its machine without the provider: README:66-67 (RuntimeMaxSec), :85-88 (sshd closes the run's stdin within 120 s of a cut; "RuntimeMaxSec is the backstop even when SSH cannot deliver a stop"), :88-91 (sweep at reconnection). The drainer's own bound is maxLifetime + stopTimeout (:153-156) and it ends with the engine's service cgroup or a reboot (:127-130). The owner asked only that shipped providers carry out an owed stop before any start (IN_FORCE.md:244; docs/proposals/ruling-42/README.md:279-281); the directory and the drainer are the admin's (docs/work-orders/145-ssh-r-durable-dir.md:20-22; docs/implementation/PROVIDERS.md:345, 350-354). The durable directory is a local path outside the store and the README says nothing of an engine that restarts on another machine, which PRV-021's `then` names (spec/promises/providers.yaml:636). New: the brief asks the question (docs/guides/full-review.md:163-164) and no record answers it; no order considered a retire hook.

**Verifier's corrections.** Does not depend on F3. A retire hook that fails while a stop remains keeps the provider retired and its state directory through retirement (F2), so PRV-021's `then` is met under either reading; only unregistering the provider loses the stops, as deleting `durableDir` does now. State the costs with the change: an owed stop is then carried out only when an actor retires the provider again (LIF-026); a hook attempt that ends unknown is not dispatched again until reconciled (OPS-021); the kit must serve `retire-hook.v1`. Coordinator decides, as stated: these were the admin's own build decisions in a package, logged for the owner to strike.


## FR-231

**A run has no stated extent: a stop can succeed while processes the run started go on**

X08/F5 · ambiguity · unsafe · verifier: confirmed, high · decides: coordinator · new

Same fault, or overlapping: FR-219.

**Promises.** PRV-030, PRV-008, LIF-024, PRV-003

**Where.** spec/promises/providers.yaml:992

**Quote.** "It reports succeeded while any run that it started for that ID is running, or is still being started under an earlier `run.v1`"

**Situation.** spec/definitions.md:314 defines a run as "The running of an instance's item that is run", and no text in spec/ mentions a descendant, process group or control group (grep finds only DL16's "the processes it started"). Reading A: the run is the process the provider started; the stop succeeds when it exits. Reading B: the run is that process and everything started from it (the kit: "Stop the entire tree", packages/runtime-provider-kit/README.md:47). Steps to the harm under A: an Agent's client starts a shell child; `stop.v1` ends the client and reports succeeded; the retirement succeeds; LIF-024 says it "leaves no run of the instance running, under PRV-030" (lifecycle.yaml:515-516); the shutdown is confirmed and the instance removed; the child keeps the secrets and identity files it was given and acts outside Conduit beside the replacement. That is MC4-002's overlap after a confirmed shutdown, which the stated limit does not cover. It has happened on the engine's own runner: "Codex's terminal children outlived the run's process-group stop in the smoke test" (docs/guides/full-review.md:171-172). SSH-R's README:27-28 records that a run can start `systemd-run --user` units "which escape the provider's stop and RuntimeMaxSec".

**Counterargument.** The kit and SSH-R already stop the tree or the cgroup. A process that leaves the cgroup cannot be found by any provider, so no wording makes the promise hold completely. After removal the credential reaches nothing (IFC-022). The brief already lists the Codex case as a friction to log.

**Least change.** In the `run` row of spec/definitions.md add: "A run is every process that its runtime provider started for the ID and every process started from one of them that the provider's stop reaches, such as those of its control group." In PRV-030's observable add: "A stop succeeds only once none of those processes is running. The provider's contract states which processes a run can start that its stop does not reach." DL16 states the same for the engine's own runner.

**Precedent.** systemd's `KillMode=control-group`; CRI's `StopPodSandbox`, which ends every process in the sandbox; Nomad's executor, which kills the task's process tree.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity settled in the reading the kit and SSH-R already build; the sentence asked of a provider's contract is a small addition under ruling 71 point 4.

**Verifier.** confirmed, high. Quote verbatim at spec/promises/providers.yaml:992-993. spec/definitions.md:314 defines a run only as "The running of an instance's item that is run". A search of spec/ for descendant, process group, control group, cgroup and process tree finds nothing; the only near text is DL16's "what becomes of the processes it started when the engine stops" (spec/implementation-defined.yaml:183), which is about the engine stopping. LIF-024 relies on the stop: "one that succeeds leaves no run of the instance running, under PRV-030" (spec/conventions/lifecycle.yaml:514-516). The stated limit covers only a run whose stop did not succeed (spec/README.md:582-584, 597-601). The two builds chose the wide reading (packages/runtime-provider-kit/README.md:47; packages/provider-ssh-runtime/README.md:66-68, with the escape named at :27-28). The occurrence on the engine's own runner is listed as a friction "not yet findings" (docs/guides/full-review.md:169-172); it is in no known-open list, log, review list or G5 row. Harm worked through under the narrow reading: the stop ends the client, reports succeeded, the retirement succeeds, LIF-035 removes the instance, and a child holding the run's environment, secrets included, goes on with no record that anything runs.

**Verifier's corrections.** Unsafe mark stands as a wrong result (a confirmed shutdown while the run's processes go on), not as new access: after removal the credential reaches nothing (IFC-022). The drafted definition is circular: "every process started from one of them that the provider's stop reaches" makes any stop succeed by definition and settles neither reading. Define the run as every process the provider started for the ID and every process started from one of them, and have the provider's contract state which of them its stop cannot reach. The fix binds the engine's own runner through PRV-003 (spec/promises/providers.yaml:74-75), which the smoke test showed does not meet it: log a first-build finding and state the own runner's reach in DL16. Coordinator may settle the ambiguity; since it changes what the language says of every runtime provider, add it to the review list for the owner to strike.


## FR-232

**MC4-002's overlap is stated in the limits and told to no caller when it happens**

X08/F6 · gap · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-014, FR-198.

**Promises.** LIF-036, LIF-009, LIF-017, PRV-021, IFC-050

**Where.** spec/conventions/lifecycle.yaml:765

**Quote.** "the request to unregister asserts that the instance's shutdown is settled in another way, and the outcome of the request is that of the removal."

**Situation.** Three results stay silent. (1) `unregister.v1` declares no outputs (spec/core/operations.yaml:245-270). An actor unregisters I; the last stop ends unknown or is not dispatched; the result is succeeded with nothing about the stop. The actor cannot tell "the machine was back and the run is stopped" from "a run may still hold its secrets"; only the provider call's record under I's subject shows how it ended (REC-002). (2) `register-replacement.v1` and the replacement's setup say nothing where the instance replaced still exists with its shutdown pending or unknown, though the engine holds that state. (3) PRV-021 has the provider name the stop that holds a start, and no promise carries a provider call's stated cause into the result of the setup that asked for the run, or says whether another instance's ID may be shown to a caller with no authority over it (ACC-056). IFC-050 requires a diagnostic only for a reason or a failed or unknown outcome (spec/interface/implementation.yaml:1343-1345), so two builders differ on all three.

**Counterargument.** The actor who unregisters has just asserted that the shutdown is settled another way and could see it pending or unknown beforehand. The overlap is a stated limit (spec/README.md:597-606). The stop's outcome is in the records.

**Least change.** Add to LIF-036's observable: "Where that stop did not succeed or was not dispatched, the result of the request carries a diagnostic, which does not change its outcome, that names the instance, its runtime provider and the machine, says how the stop ended, and says that the run may still be running with the values it was given." Give `unregister.v1` the output `diagnostics`, as `register.v1` has. Add the same kind of diagnostic to LIF-009 where the instance replaced exists with its shutdown not confirmed. Add to PRV-007: the engine's diagnostic for a run part that failed or ended unknown carries the provider's text.

**Precedent.** `kubectl delete --force` prints that immediate deletion does not wait for confirmation and the resource may continue to run on the cluster indefinitely.

**Who decides.** coordinator. Ruling 71 point 4: a small finding that adds a diagnostic, under ruling 52 point 2, which already rules that every result says what happened and why. Log it for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/lifecycle.yaml:765-766. `unregister.v1` declares no outputs (spec/core/operations.yaml:259-269). IFC-025: "Where it declares no outputs, the result carries no `structuredContent`, except for error diagnostics" (spec/interface/implementation.yaml:669-670) and `content` "may be an empty list where the result is not an error" (:719-721). So a succeeded unregistration whose last stop ended unknown, failed or was not dispatched may return nothing at all, and the instance can no longer be inspected. IFC-050's observable requires a diagnostic only for a reason or a failed or unknown outcome (:1342-1344). LIF-009 states the overlap for a replacement and asks for no diagnostic (spec/conventions/lifecycle.yaml:154-158), though `register-replacement.v1` has the output (spec/core/operations.yaml:594-600). No record found in docs/AUDIT.md, docs/guides/cold-readers.md, the review list, increment 4's log or G5.md; G5 holds only the fence question, which the owner answered.

**Verifier's corrections.** Point 3 is partly decided: IFC-050's `then` has the engine give "the cause in words" and leave out "only what ACC-056, LIF-013 and PND-004 withhold" (spec/interface/implementation.yaml:1338-1340), so another instance's ID is not withheld by any promise; what is open is only whether the provider's own text is carried into the engine's diagnostic. Giving `unregister.v1` a `diagnostics` output gives its successful result a `structuredContent` under IFC-025: make the output required, as `register.v1` has it (spec/core/operations.yaml:172-178). Class, unsafe mark and who decides stand.


## FR-233

**A run reported `running` for which the engine holds no `url` can never be reached**

X08/F7 · gap · verifier: confirmed, medium · decides: coordinator · known-new-situation

**Promises.** PRV-007, PRV-009, PRV-019, IFC-017

**Where.** spec/promises/providers.yaml:320

**Quote.** "Its result may also give `url`, the URL at which the run's server is reached now, where it has moved since the run began, and the engine then reaches the server there, under IFC-017."

**Situation.** The engine stops between a provider starting a run and the engine keeping `run.v1`'s result, or the result is lost; the setup ends unknown. The provider then reports `running`. PRV-007 says "nothing is asked for and the part counts as succeeded" (line 250-251), and the run "is asked for only where the provider reports it `stopped`" (line 265). IFC-017 reaches the server "at the `url` that its runtime provider last reported"; none was reported, and `inspect-run.v1` gives `url` only as a `may`, "where it has moved since `run.v1` returned one" (spec/core/operations.yaml:1033-1034; the schema requires only `state`). Order 081's repair, PRV-019's "the call succeeds and its result gives the `url`", is reached only by a race, because PRV-007 never sends `run.v1` for a running run. Order 080 met this and chose to report unknown (docs/proposals/first-build/to-review-with-the-owner.md:70). The situation the list lacks: the instance then stays unavailable until it is retired and replaced, since no later call can return the url.

**Counterargument.** An engine may keep each url in its own state across restarts. PRV-007 already says a run part that ended unknown is ended by a retirement's stop. Order 080's reading is safe and stands as review item 27.

**Least change.** In `inspect-run.v1`'s output schema require `url` where `state` is `running`, and describe it as "the Streamable HTTP URL at which the run's server is reached now". Say the same in PRV-009, keeping PRV-003's exception for the engine's own runner.

**Precedent.** CRI's `PodSandboxStatus` returns the sandbox's network status on every call, and a Pod's status always carries `podIP`.

**Who decides.** coordinator. Ruling 71 point 4: a small tightening of one output of a provider's contract, logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/providers.yaml:320-322. PRV-007: "nothing is asked for and the part counts as succeeded" (:250-251) and the run "is asked for only where the provider reports it `stopped`" (:264-265). `inspect-run.v1` gives `url` as something "a provider may give ... where it has moved since `run.v1` returned one" (spec/core/operations.yaml:1033-1035) and its schema requires only `state` (:1050-1054). IFC-017 reaches a run "at the `url` that its runtime provider last reported" (spec/interface/implementation.yaml:447-448). The known part is review item 27 (docs/proposals/first-build/to-review-with-the-owner.md:70) and order 080's handling "because there is no `url` or the bridge fails" (docs/work-orders/080-shell-runtime-provider-calls.md:215-217); neither records that the provider's contract lets the `url` stay absent for good, nor the fix.

**Verifier's corrections.** Known-new-situation mark is right. It is a dead end more than an undecided situation: each step is decided (the part counts as succeeded, then every request reports unavailable under IFC-017), and nothing but retire and replace ends it. It is wider than the lost result: no promise says the engine keeps a run's `url` across its own restart, so any engine that does not is in the same place. The kit already returns the `url` on inspection (packages/runtime-provider-kit/README.md:225-226), so shipped providers are not affected. Coordinator decides under ruling 71 point 4, as stated.


## FR-234

**No call tells a runtime provider that an instance was removed, so a state directory it supplies is never removed**

X08/F8 · gap · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-054.

**Promises.** PRV-029, LIF-035, LIF-036, PRV-008

**Where.** spec/conventions/lifecycle.yaml:733

**Quote.** "Removing or unregistering an instance removes its state directory and contents under PRV-029."

**Situation.** LIF-035's sentence reads as general, while PRV-029's `then` binds only the engine's own runner and its observable says "Another runtime provider supplies such a directory only where its contract states it does" (providers.yaml:979-980). A runtime provider is given `run.v1`, `stop.v1` and `inspect-run.v1` only. A confirmed shutdown removes the instance with no call, and the stop at unregistration is the same call as a retirement's stop, so a provider cannot tell removal from retirement. SSH-R therefore states its per-instance remote state directory is "kept across starts and never removed (PRV-029)" (packages/provider-ssh-runtime/README.md:48-49). Agent packages use that directory for the Agent's home and its login (PRV-029). On the second machine a removed Agent's login stays on disk for good, under the one run user of ruling 77 that every later run shares. An actor reading LIF-035 believes it is gone.

**Counterargument.** PRV-029 is scoped to the engine's own runner, and SSH-R's contract says plainly that it never removes the directory, so no promise is broken. Keeping a home across a replacement can be wanted. The shared user's unmet isolation is already stated in SSH-R's contract.

**Least change.** Qualify LIF-035's sentence: "... removes the state directory that the engine's own runner gave it, with its contents, under PRV-029." Add to PRV-029's observable: "No call tells another runtime provider that an instance was removed: a directory that such a provider supplies lasts until the means its contract states remove it." Add the same line to the limits in spec/README.md. For the owner's batch: whether `runtime-provider.v1` gains a call at removal.

**Precedent.** CRI, whose shape ruling 40 gave the runtime provider (IN_FORCE.md:240), has `RemovePodSandbox` after `StopPodSandbox`; the kubelet removes a Pod's volumes when the Pod is deleted.

**Who decides.** coordinator. Rulings 37 and 46 point 4 for the statement, which adds no behavior. A removal call is a new operation, which ruling 40 and order 081's stop list make the owner's.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/lifecycle.yaml:733-734. PRV-029's `when` is "The engine's own runner starts a process for an instance's run" (spec/promises/providers.yaml:968-969) and its observable says "Another runtime provider supplies such a directory only where its contract states it does" (:979-980). `runtime-provider.v1` lists three operations only (spec/core/capabilities.yaml:152-155); a confirmed shutdown removes the instance with no call (LIF-035), and the stop at unregistration is the same `stop.v1` as a retirement's (LIF-036, spec/conventions/lifecycle.yaml:746-749). SSH-R states the consequence: its per-instance remote state directory is "kept across starts and never removed (PRV-029)" (packages/provider-ssh-runtime/README.md:47-48). No record found in the known-open lists, the review list, increment 4's log or PROVIDERS.md.

**Verifier's corrections.** One overstatement: a removed Agent's login stays there only once Agent packages keep their homes in that directory on a remote runner; PRV-029 says they use it, and the homes are order 050's, unwritten (IN_FORCE.md:187). The split of who decides is right: the statement is the coordinator's, a removal call is the owner's.


## FR-235

**Nothing outside the kit's fake-backend tests can test PRV-019, PRV-030 or the owed-stop rule**

X08/F9 · gap · verifier: weakened, low · decides: coordinator · new

**Promises.** PRV-019, PRV-021, PRV-030

**Where.** experiments/022-provider-contracts/README.md:64

**Quote.** "H2 adds trust fixtures, run certificates, a gate and fault controls. SV1 will supply the safety-case field; H1 invents none."

**Situation.** The three promises have `vectors: []`, and order 081 wrote none because the conformance runner tests the engine. The provider-contract harness has no runtime case (`cases: []`, suites/runtime/index.yaml; "No real provider has been tested and there are no contract cases yet", README:107). As built it could express one slice of PRV-030: run, stop, pause, then `inspect-run.v1` still `stopped`, and a repeated `run.v1` returning the same url. It cannot express the rest. Steps are `call`, `pause` and `restart`, each call awaited, so a stop arriving while a start is in flight cannot be sent. It has no fault control to cut a machine off, so no stop can become owed against a real provider. Assertions read only the provider's own answers (docs/provider-yaml.md:102-110) and the harness "reads none of the provider's private state directory", so "no run is running" is judged by the provider under test. `run.v1` requires `identity` and "H1 creates no identity fixtures" (docs/provider-yaml.md:191). The 081 Record adds that the shell's own runner implements neither PRV-019's nor PRV-030's new guarantee, and that MC4 must be registered again.

**Counterargument.** H1 is the first tier by design and H2 is planned with fault controls and identity. The kit's tests cover the three cases and the in-flight cancel with a fake backend, and model checking is planned to rerun MC4.

**Least change.** Before A63-VR's runtime suite, H2 adds: a call step that does not wait for its answer, with a later step that awaits it; a fault control that the descriptor maps to the backend, for a machine that cannot be reached and a stop that fails; an assertion on a lab file that the run's fixture item writes, so liveness is seen apart from the provider's answer; and identity fixtures. Run it first against the kit's fake backend, then SSH-R over loopback sshd. Register the cases against F1's three outcomes before they run.

**Precedent.** `critest` of cri-tools, the validation suite every CRI runtime passes.

**Who decides.** coordinator. Experiments bind nothing; the harness is the coordinator's and lane 8's (docs/implementation/PROVIDERS.md, decision A), and ruling 45 has measures fixed before a run.

**Verifier.** weakened, low. Quote verbatim at experiments/022-provider-contracts/README.md:64-65. `cases: []` at experiments/022-provider-contracts/suites/runtime/index.yaml:2; "No real provider has been tested and there are no contract cases yet" at README:107; steps are call, pause and restart (docs/provider-yaml.md:61-65); assertions read the provider's answers only (docs/provider-yaml.md:104-110); "H1 creates no identity fixtures; H2 owns them" (docs/provider-yaml.md:191-192). Most of it is already recorded: the harness states its own limits in those lines; order 081's Record says the shell's own runner implements neither guarantee and that MC4 must be registered again (docs/work-orders/081-spec-one-run-per-instance.md:622-626); the plan has fault controls in A63-H2b and the runtime suite's "repeated-run cases after 081" (docs/implementation/PROVIDERS.md:125-126); the brief asks for MC4's re-registration against the kit's owed-stop rule (docs/guides/full-review.md:182-183).

**Verifier's corrections.** Not a gap in spec/: experiments bind nothing, and this is a change to an experiment for item 4 of the brief's "What to produce". New are three requirements for H2 that no plan holds: a call step that does not wait for its answer, a liveness check that does not rest on the provider's own answer, and PRV-030, which A63-VR's list of promises omits (docs/implementation/PROVIDERS.md:126). Keep those three and drop the rest as recorded.


## Further points the verifier noticed

Not verified by a second reader.

- **SSH-R's drainer is a process of the provider's run that outlives the stop of that run, which fits only the narrow reading of F5** (packages/provider-ssh-runtime/README.md:126-129). "It leaves the engine runner's process group by design (DL16), and survives that group's kill." The engine's own runner runs the provider (PRV-002) and "keeps every promise that this file makes of a provider of that kind" (spec/promises/providers.yaml:74-75), PRV-030 among them. If F5 is settled as the kit and SSH-R build it for their own runs (a stop succeeds only once every process started from the run has ended), the stop of the provider at its retirement reports succeeded while the drainer goes on. Settle F5 before F4: under the wide reading the drainer needs a stated exception in DL16 or has to go.
- **LIF-036 states as fact that an owed stop outlives the provider's retirement** (spec/conventions/lifecycle.yaml:781-783). "a stop that the provider owes outlives the engine's restart, on its machine or on another, and the provider's own retirement, kept where its contract states, under PRV-021." It is a fourth copy of the clause F2 and F3 correct, beside PRV-020:617-619, spec/README.md:588-596 and `stop.v1`'s description (spec/core/operations.yaml:1003-1007). Neither finding lists it or the `stop.v1` copy, and it is unconditional where PRV-021 leaves the matter to the contract.

## The verifier's notes

Checked at 2eb8f258 in the review worktree, read only; nothing was built or run. Every quote and line was opened: all nine quotes are verbatim, and the line citations are right to within one line.

Verdicts: five confirmed (F2, F4, F5, F6, F7), three weakened (F1, F3, F9), none refuted, none wholly known.

- F1 is real and already queued; its unsafe mark does not survive, because `stop.v1` is asked only for a retired or removed instance and `inspect-run.v1`'s contract keeps the engine from asking for that ID's run while the stop is owed.
- F3's ambiguity is mostly settled by the promise's own observable and by rulings 42 and 43's records, and its drafted `then` would amend D11, so that draft is the owner's.
- F4 does not depend on F3: a failed retire hook keeps the state directory through retirement, so PRV-021's `then` is met as it stands. That answers the brief's question: PRV-021 with PRV-029 and LIF-035 is neither a contradiction nor asking too much.
- F5 is the only finding I rate high; its drafted definition is circular and needs the rewording given.
- F9 answers a question the assignment posed and is not a spec finding.

Not verified: the reviewer's account of Kubernetes and Nomad (stated as from memory; I fetched nothing), and whether the first build can dispatch a hook to a provider Resource. I did not read the kit's or SSH-R's source beyond the lines cited, or the Records of orders 129, 130, 137 and 140 beyond the passages quoted.
