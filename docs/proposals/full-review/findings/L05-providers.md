# L05-providers: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The runtime-provider half of this slice is the strongest: run, stop and inspection follow the Container Runtime Interface, and PRV-007, PRV-019 and PRV-030 never turn an unknown into a second run. The faults sit at the seam with the newer enrollment text. A node is now a triple of provider, id and hostname, but run.v1, stop.v1 and place.v1 still speak in hostnames and selectors (F1, F3, F4), and the pending state and a machine without a node are left undecided (F5, F6). The slice is not minimal: 63 promises and about 16,400 words, of which `observable` holds 11,046 against 3,962 in `then`, with binding rules in the former, promises that bind engine and provider at once, and no vector listed on any of them. Layering is right except ENR-005 and the document rules in PRV-002 and PRV-013. I would have the engine filter placements and not ask the provider at all where one node or none can match, drop the label echo through the provider, cut ENR-011, which restates PRV-018, PLC-012 and ACC-001, and move PRV-025's and PRV-021's essays out of the promises; to the engine, `headless` alone means the same as `require: [attestation]`. The weakest point is that what the user picks is stated in prose that no engine reads: where a run's secrets are kept, the gate, what becomes of an owed stop, whether a state directory exists and on which machines a provider runs. A swap of provider therefore changes safety with nothing in inspection to show it, and promises such as PRV-021 and PRV-011 cannot be tested from the engine's side. Situations a real deployment meets that are decided only harshly or not at all: a machine renamed after admission is omitted for good, a machine provider cannot be swapped without declaring every other machine lost (F7), and an Agent that is replaced loses its state directory and must be signed in again, since PRV-029 keys the directory by instance ID.

**What the reader read.** Commit 2eb8f258, read only; nothing was built or run. Read in full, line by line: spec/promises/providers.yaml (1005 lines), spec/promises/placement.yaml (459), spec/promises/enrollment.yaml (288), spec/definitions.md (467, in parts), spec/README.md (678), docs/guides/full-review.md, and AGENTS.md "What holds". Read in part: spec/core/operations.yaml lines 855 to 1150 and 1455 to 1640 (the provider and enrollment operations), spec/core/capabilities.yaml (all), spec/core/README.md "What this package does not settle", and spec/implementation-defined.yaml items DL5, DL6, DL16, DL17 and DL19. Cited promises opened in full: LIF-009, 010, 011, 017, 024, 025, 026, 028, 035, 036, 040, 041; OPS-002, 003, 004, 007, 012, 014, 022, 023; TPL-009, 012, 018, 019; PND-001, 002, 003, 004, 008, 012; OBJ-002, 003, 023; SUB-001, 003, 006; ACC-002, 032; IFC-017; OBS-002, 003, 004; SEC-011, 012; CON-008, 024; REC-001; MAN-018; POOL-009; PKG-024. ACC-033 was searched, not read. Known-open records checked: docs/AUDIT.md lines 160 to 245 and 361 to 410, docs/guides/cold-readers.md lines 39 to 172, IN_FORCE.md lines 200 to 256, the brief's queued list, the index of findings in experiments/012-model-checking/G5.md (by search), the changed lines of docs/work-orders/038b-staged/049b.patch, a few lines of docs/work-orders/049a1-build-enrollment-core.md, ruling 50's record head, the ledger rows of rulings 50, 56, 68 and 77, and points O1 and O2 of docs/proposals/spec-batch-1/areas/machines-and-trust.md. Two package READMEs (provider-local-machine, provider-ssh-runtime) were read in excerpts as evidence only. Not read: the other promise files beyond the promises listed; IFC-022, IFC-041, IFC-042, IFC-049, IFC-050, DL13, SEC-010, SEC-020, SEC-023, PKG-020, PKG-023, PKG-030, CTR-004, ACC-027, ACC-056, FST-006, FST-007, REC-002 and CON-006, which the slice cites; core types, templates and results; conformance vectors (only the manifest's PRV, PLC and ENR tags were counted: 10, 11 and 0 promises); engine/, experiments/ and the rulings' records other than those named. Word counts and line numbers were recounted at this commit.


## FR-047

**A stop or inspection for an instance on a retired node names its machine by hostname alone, and another machine may by then hold that hostname**

L05/F1 · contradiction · unsafe · verifier: confirmed, high · decides: coordinator · new

**Promises.** PRV-027, PRV-008, PRV-030, PLC-026, PLC-027, ENR-006, LIF-024, OPS-014

**Where.** spec/promises/providers.yaml:920

**Quote.** "It gives in `instance` the instance's ID and in `machine` the machine of the instance's node as the node's status holds it, whether or not the node is retired: in `labels`, the labels that the status holds, and in `hostname`, the value of their `kubernetes.io/hostname` label."

**Situation.** 1. Resource X (no retire hook, run by runtime provider R) is on node N1, hostname `worker`. The machine is cut off; root declares N1 lost (PLC-017). X is retired, shutdown unknown; its run may still be alive with its secrets. 2. A replacement computer named `worker` is enrolled and admitted: ENR-006 check 3 excludes only "a non-retired node's" hostname (enrollment.yaml:135) and PLC-026 says a new admission "may take the hostname again" (placement.yaml:401). 3. An actor retires X again (LIF-026). PLC-023 does not hold the stop back, since N1 represents no machine; PLC-027 says "The stop is asked for all the same". stop.v1 carries {hostname: worker, N1's labels} and no machine id. 4. R keeps no state of its runs, the case PRV-027 says it serves; it reaches the machine now called `worker`, finds no run of X, and answers succeeded ("Stopping a run that is not running succeeds", providers.yaml:284). 5. PLC-027: the retirement "succeeds once the stop has succeeded"; LIF-024 confirms the shutdown, LIF-035 removes X. The old run was never stopped, and PRV-030 is broken by a provider that could not know. inspect-run.v1 gives `stopped` in the same way, which OPS-014 takes as a fence, so an actor may record failed for an invocation whose effect can still land.

**Counterargument.** PRV-030 already forbids the provider to report succeeded while a run it started is running, and PRV-011's channel must authenticate the machine, so a provider that pins a host key would end unknown. But what it pins is its own configuration, outside enrollment, and PRV-027's stated purpose is a provider that keeps no state.

**Least change.** ENR-006 check 3: "The hostname is neither local's, that of a node that exists, retired or not, nor another pending machine's." PLC-026: "a new admission may take the hostname again once the retired node has been unregistered". Then no stop or inspection is ever sent under a hostname that a second node holds. Giving PLC-026's provider and id in `machine` as well would let a provider tell the machines apart.

**Precedent.** Kubernetes: a Node's name is free again only once the old Node object is deleted, and the cloud node controller identifies a machine by spec.providerID, not by its name.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction between PRV-027 with PLC-026 and PRV-030 with LIF-024; the fix narrows one admission check and adds no behavior. Logged for the owner to strike.

**Verifier.** confirmed, high. Quote verbatim at spec/promises/providers.yaml:920-922. stop.v1 and inspect-run.v1 take only `instance` and `machine {hostname, labels}` (spec/core/operations.yaml:1011-1021, 1044-1054); no node, provider or machine id. Hostname reuse while the retired node still exists is allowed four times: enrollment.yaml:135 "a non-retired node's"; placement.yaml:400-402 "which may take the hostname again"; enrollment.yaml:286-287; placement.yaml:359-360. The stop on a lost node is still sent (placement.yaml:431-432 "The stop is asked for all the same") and its success confirms the shutdown (placement.yaml:439-441; conventions/lifecycle.yaml:500-501). PRV-027 states its own purpose as the stateless provider (providers.yaml:925-928 "a runtime provider that keeps no state of its own runs finds the run's machine in each call"); LIF-036 repeats it (lifecycle.yaml:758-760). OPS-014 takes a `stopped` report as a fence (conventions/operations.yaml:361-363). Precondition checked: PRV-014 "gives no hostname twice" (providers.yaml:431), so the old machine must have left the report first, which ENR-012/ENR-008 removal does; reachable. No deciding text found in spec/. Not in the known lists: docs/guides/full-review.md:136 covers only the "labels last reported" wording; docs/work-orders/049a1-build-enrollment-core.md:334-342 flags only that `machine` lacks the id.

**Verifier's corrections.** Class and unsafe mark stand (wrong result: shutdown confirmed and attempt fenced while the old run may live). It is narrow: it needs a provider that resolves by hostname and a further retirement, reconciliation or inspection after the hostname is reused. The least change must also amend ENR-012 (enrollment.yaml:286-287), PLC-024 (placement.yaml:359-360) and ENR-004 (enrollment.yaml:100-101), which repeat the reuse. Basis: the fix withdraws a behavior the text states (choice C27 of order 038b, docs/work-orders/038b-spec-enrollment.md:175-177), so it is ruling 71 point 4 plus the review list, not 'adds no behavior'. Adding only the id to `machine` (as 049a1 builds) does not help: the id is scoped to the machine provider and a stateless runtime provider cannot resolve it. Decide together with F7, which the tighter check 3 makes harsher.


## FR-048

**Nothing bounds the tries at the typed fingerprint, whose alphabet the provider chooses**

L05/F2 · gap · unsafe · verifier: weakened, medium · decides: coordinator · new

**Promises.** ENR-005, ENR-006, ENR-007

**Where.** spec/promises/enrollment.yaml:148

**Quote.** "A failed check 8, including a missing fingerprint input, reports invalid; every other failed check reports conflict"

**Situation.** The typed fingerprint exists so that a model-driven session holding root's tools cannot admit a machine blind (spec/README.md:406 allows for such a console). 1. Enrollment E is open and machine M is pending under it; it joined by a secret-free URL, the action ENR-002 prefers (enrollment.yaml:51), which whoever learned the URL could use. 2. The session sends admit-machine.v1 {E, M, fingerprint: g} with successive guesses. A wrong g fails only check 8: invalid, accept-machine.v1 is not called, the enrollment stays open and the action answerable. 3. ENR-005 fixes the length at 6 to 8 characters and leaves the alphabet to the provider's contract: digits give at most 10^6 requests, hex 1.7 x 10^7. 4. On the matching guess the engine accepts M and makes its node, and the answer's record says the fingerprint was compared. PRV-015 may now place instances there, and PRV-011 delivers their secrets to a machine whose fingerprint no person read.

**Counterargument.** Every failed request is recorded, the enrollment's expiry bounds the window, and a 32-symbol alphabet gives 2^30 values. A process under the engine's operating-system user can read root's token and the panel anyway, a stated limit (spec/README.md:424).

**Least change.** ENR-006: "A request that carries a fingerprint and fails check 8 closes the enrollment refused; its diagnostic says so, and root opens another." ENR-005: "the fingerprint's alphabet has at least 32 symbols".

**Precedent.** Bluetooth passkey entry ends the pairing on one mismatch; RFC 8628 section 5.1 has the server limit attempts on a user code.

**Who decides.** coordinator. Ruling 71 point 4: a small finding that adds a feature and cannot move S1's counts; ruling 68 point 3 set the comparison. Logged for the owner to strike.

**Verifier.** weakened, medium. Quote verbatim at spec/promises/enrollment.yaml:148-149. No promise bounds attempts or sets a floor on the alphabet: ENR-005 fixes only "a string value of 6 to 8 characters" (enrollment.yaml:111-112) and ENR-002 leaves "the fingerprint's alphabet" to the provider's contract (enrollment.yaml:56). A failed check 8 closes nothing (enrollment.yaml:159-160 "A failed check dispatches no accept-machine.v1"). The purpose of the typed value is exactly the model-driven session: docs/proposals/spec-batch-1/README.md:132 "a session driven by a model could still admit a machine without checking it, unless the command carries what the person read"; ruling 68 point 3 accepted it (docs/proposals/ruling-68/README.md:44-45). Searched spec/, IN_FORCE.md, AUDIT, the first-build logs, rulings 56 and 68, the enrollment proposal and 049a1 for attempt, guess, brute, entropy: nothing. New.

**Verifier's corrections.** Gap stands. The unsafe mark holds only by the letter. In v0 the only party able to send 10^5 to 10^6 root requests inside one enrollment is a process holding root's token, which the stated limit already places inside the trust boundary (spec/README.md:424-425) and which can do worse directly; a model limited to tool calls gets about 1e-6 per try on digits. Fix: an attempt bound alone is enough and is the existing paradigm (RFC 8628 section 5.1; PIN retry counters). Drop the 32-symbol floor: it would exclude hex fingerprints of a node key (ruling 56 points 6 and 10, docs/proposals/ruling-56/README.md:25,29). One strike against a small count is a behavior choice: ruling 71 point 4, logged to strike, on the review list. What would settle the unsafe mark: whether the panel is readable by a holder of root's token (not stated in spec/).


## FR-049

**An instance that can only be placed on local is still put to the registered machine provider, which need not know the engine's machine**

L05/F3 · defect · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-254.

**Promises.** PRV-015, PLC-002, PLC-007, PLC-023, PRV-003, ENR-010

**Where.** spec/promises/providers.yaml:488

**Quote.** "For an instance that has an item that is run and names no `runtime`, the selector holds the hostname of the machine that `local` represents, so the provider names that machine or none"

**Situation.** A user registers a machine provider to add a second machine. Its inventory lists the remote machines only, or names the engine's machine otherwise than `local`'s label does; PLC-023 allows this (a report that omits local's machine "makes nothing unavailable", placement.yaml:332), and packages/provider-local-machine/README.md:24 notes that the two names can disagree. PRV-003's own machine provider no longer serves. An Agent with a `command` item, no `runtime` and no `placement` is registered: PRV-015 has the engine ask place.v1 with local's hostname in the selector; the provider has no such machine and names none; "the instance is not instantiated". The same document registered the day before. `local`'s hostname label was fixed at first start (ENR-010), so no later configuration reconciles the names: every such document now needs `placement: local`. The engine already knows the answer, since PLC-007 admits one node only.

**Counterargument.** A machine provider is expected to report every machine, the engine's among them, under local's hostname, and a user picks one that does. PRV-015 already skips the call where a type or an item names another hostname.

**Least change.** PLC-002 and PRV-015's `when`: "Where an item that is run is selected for it and it names no `runtime`, its placement is `local`, checked under PLC-007, and no machine provider is asked."

**Precedent.** Kubernetes: a Pod whose spec.nodeName is set is not passed to the scheduler; a RuntimeClass's scheduling.nodeSelector confines its Pods without asking the node's provider.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect whose fix gives the result PLC-007 already forces and removes a provider call.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/providers.yaml:488-490. PLC-002 sends every run-item instance with no placement to the provider (placement.yaml:39-42); the only skip is where no node can match (providers.yaml:490-492). No promise obliges a registered provider to report the engine's machine: PRV-014 lists what a report gives per machine (providers.yaml:425-433) and PLC-023 allows the omission (placement.yaml:331-332 "or that omits it, makes nothing unavailable"). Result is decided and harsh: providers.yaml:473 "Where the provider names none, the instance is not instantiated". Supporting fault: `local` is bound to no id (placement.yaml:129-131, 396-398) while PRV-015 has the provider name a machine "by id" (providers.yaml:469), and the staged vector says "never select by hostname alone" (docs/work-orders/038b-staged/049b.md:33). packages/provider-local-machine/README.md:23-25 records that the two hostnames can disagree. Not in the known lists; docs/guides/full-review.md:135 is only the id-against-hostname output.

**Verifier's corrections.** Class: the text can be followed and decides the case (rejection), so this is needless complexity with a harsh outcome more than a defect; keep either. Not unsafe: correct. Who decides: coordinator, with precedent in round 5's M22, which already narrowed where the provider is asked (docs/proposals/ruling-log.md:571). The fix also removes the provider's ability to answer none for `local`, which ruling 41 item 8c gave it: log to strike and add to the review list. F4's fix subsumes this one.


## FR-050

**place.v1 obliges the provider to choose by facts that only the engine holds, and its 'none' carries no reason**

L05/F4 · defect · verifier: confirmed, medium · decides: coordinator · new

**Promises.** PRV-015, ENR-010, ENR-012, PLC-026, PLC-007

**Where.** spec/promises/providers.yaml:468

**Quote.** "The engine asks the machine provider with `place.v1`, giving the instance's ID and the label selector of PLC-007, and the provider names by id one machine already represented by a node that it reports reachable and whose node's fixed labels match the selector."

**Situation.** place.v1's inputs are `instance` and `nodeSelector` (spec/core/operations.yaml:1101); the staged 049b patch changes only its output. The provider is told neither which machines a node that is not retired represents nor those nodes' fixed labels, which ENR-010 lets differ from what it now reports (enrollment.yaml:248). (a) Root answers a retired node's removal question `kept` (enrollment.yaml:280): the machine stays reported reachable with no node. A provider that names it gives unavailable ("names a machine for which no node represents yet", providers.yaml:476); the request "made later is decided as any other", a deterministic provider names it again, and placement stays unavailable although a matching node exists. (b) A provider that matches its current labels names a machine whose node's fixed labels differ: PLC-007 rejects the placement as invalid, though another node matches. (c) Where it names none, the output is an absent id: the rejection cannot say whether no node matched or none was reachable, which the engine could have computed itself.

**Counterargument.** A provider that served accept-machine.v1 can remember which ids it accepted, and ENR-002 has it bind the labels, so drift is rare; ruling 41 put the choice at the provider on purpose.

**Least change.** place.v1 gains the input `candidates`: the ids of the machines that nodes not retired represent, reported reachable, whose fixed labels match. The provider names one of them, or none with a `message`. With no candidate the engine rejects with a diagnostic naming the selector and the nodes considered; with one it does not ask.

**Precedent.** The Kubernetes scheduler extender: kube-scheduler filters, then passes the candidate nodes to the extender's filter and prioritize verbs.

**Who decides.** coordinator. Ruling 71 point 4: a small added input to a contract that the staged 049b patch already edits in place; the provider still chooses, as ruling 41 has it. Logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/providers.yaml:468-470. place.v1's inputs are `instance` and `nodeSelector` only (spec/core/operations.yaml:1101-1107); the staged patch changes the description and the output alone (docs/work-orders/038b-staged/049b.patch:78-99). The provider is never told of a node's retirement (ENR-012 asks root, and `kept` leaves the machine in place: enrollment.yaml:279-281) nor of the fixed labels, which may differ from its report (enrollment.yaml:248-249). (a) providers.yaml:475-478 gives unavailable for "a machine for which no node represents yet", and providers.yaml:485-486 "the same request made later is decided as any other". (b) placement.yaml:113-115 checks the provider's choice under PLC-007. (c) The output is an absent id; providers.yaml:481-483 states no diagnostic for 'names none'. No deciding text; not in the known lists.

**Verifier's corrections.** Class, unsafe mark (false) and (a) to (c) stand. The clause 'with one candidate it does not ask' removes the provider's 'none' (ruling 41 item 8c): either drop that clause or put it on the review list. A new input on a core contract is ruling 71 point 4, as stated. Merge with F3.


## FR-051

**No promise decides a machine reported pending that a node already represents**

L05/F5 · gap · verifier: weakened, medium · decides: coordinator · new

**Promises.** PLC-023, ENR-004, PRV-014, PRV-015, ENR-008

**Where.** spec/promises/placement.yaml:300

**Quote.** "A machine other than the one that `local` represents is reported unreachable by the machine provider, or, while a node represents it, is omitted from the provider's latest report or no report has been received since the engine last started"

**Situation.** (a) Just after admission the latest report, made before accept-machine.v1, still gives the new node's machine as pending. (b) Later the provider's network withdraws approval, as on a key's expiry, and reports the node's machine pending. PLC-023's `when` lists unreachable and omitted only. Reading A: pending counts as omitted, and requests report unavailable. Reading B: PLC-023 does not apply, and run.v1, stop.v1 and requests go to a machine that "cannot send or receive traffic" (ENR-002). ENR-004 read to the letter says "No node represents it" and raises an action of kind machine "saying why it cannot be admitted" (enrollment.yaml:94) for a machine that is a node, and refuse-machine.v1 on it reports conflict.

**Counterargument.** Order 049a1 reads it as omitted and tells its worker to report the point, so the build has one reading; but 049a1 is checkpointed, not merged, and no promise holds the reading.

**Least change.** PLC-023's `when`: "is reported unreachable or pending by the machine provider". ENR-004's `when`: "reports pending a machine that no node represents". The node's condition names the reported state.

**Precedent.** Kubernetes sets a Node's Ready condition to Unknown or False and sends it nothing new when its kubelet's credential lapses or it stops reporting.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a gap closed by the existing rule for an unreachable machine; no behavior is added.

**Verifier.** weakened, medium. Quote verbatim at spec/promises/placement.yaml:300-302. spec/ does not decide a represented machine reported `pending`: PLC-023's `when` lists unreachable, omitted and no report; placement.yaml:335-338 covers only a mismatched triple. But it is recorded. Situation (a) is item 28 of the review list, which the owner let stand on 2026-11-12: docs/proposals/first-build/to-review-with-the-owner.md:71 "A newly admitted node starts `Ready: Unknown` (order 049a1), the cautious reading of PLC-023". Situation (b) is in docs/work-orders/049a1-build-enrollment-core.md:311-314: "counts as omitted ... PLC-023 does not say what a represented machine reported `pending` counts as: report that as a finding".

**Verifier's corrections.** Known mark: known-new-situation, not new. What the records lack is ENR-004: its `then` says "No node represents it" (enrollment.yaml:90) and raises an action of kind machine (enrollment.yaml:94-95), so one reading lets refuse-machine.v1 remove a node's machine and the other reports conflict under ENR-008 (enrollment.yaml:206-207). The fix is to write item 28's reading into PLC-023 and narrow ENR-004's `when`: coordinator, as stated.


## FR-052

**An admission whose accept call ends unknown cannot be finished, and a machine on the provider's network without a node is shown nowhere**

L05/F6 · gap · verifier: weakened, medium · decides: coordinator · new

**Promises.** ENR-007, ENR-004, ENR-006, ENR-008, LIF-028, OPS-014, OPS-012

**Where.** spec/promises/enrollment.yaml:182

**Quote.** "An unknown accept call likewise makes no node and ends unknown; repeats follow OPS-012, not an implicit retry."

**Situation.** accept-machine.v1 is cut mid-call, as in S3's partition: the provider accepted M and the engine got no result. No node is made. The next report gives M reachable; ENR-004 speaks only of a machine reported pending, so M leaves the panel, and the enrollment stays open showing no machine. A new admit fails check 1 (M is not pending): conflict. A repeat reports unknown, since admit-machine.v1 declares no `deduplicates`. OPS-014 lets an actor record that the effect occurred, but LIF-028 covers only set-up, open, retire and close, so nothing then makes the node, though the effect is "Accepts the machine, makes its node and answers the enrollment admitted" (spec/core/operations.yaml:1517). An honest actor can record neither that it occurred nor that it did not, and the invocation stays unknown for good. M is accepted on the network with no node, and root can remove it only by an id that no tool or panel now shows. The same holds for any machine reported reachable or unreachable that no node represents: one accepted in the provider's own console, one kept under ENR-012, a device that was on the network before Conduit.

**Counterargument.** ENR-008 can still remove M and a new enrollment starts over; PLC-026 says a report alone never makes a node, so ignoring such machines is deliberate, and order 049a1 builds it so.

**Least change.** ENR-007: "Recording under OPS-014 that an admission of unknown outcome occurred makes the node as this promise does, where the provider in force reports the machine accepted; recording that it did not leaves the action answerable." ENR-004: "A machine reported reachable or unreachable that no node represents is shown at root's panel, under the open enrollment it is attributed to or else under one action of kind machine, with its id."

**Precedent.** LIF-028 for a reconciled set-up; Kubernetes keeps an approved CertificateSigningRequest listed when its Node never registered.

**Who decides.** coordinator. Rulings 37 and 46 point 4 for the reconciliation, which follows LIF-028's rule for a set-up; ruling 71 point 4 for showing the machine under the existing kind machine.

**Verifier.** weakened, medium. Quote verbatim at spec/promises/enrollment.yaml:182-184. Real part: after an unknown accept that took effect, a new admission fails check 1 (enrollment.yaml:132), a repeat is not dispatched (conventions/operations.yaml:281-283), and LIF-028 covers only four verbs (conventions/lifecycle.yaml:594-595), so recording 'occurred' gives succeeded with no node (operations.yaml:313) and no honest record exists. Overstated part: the machine is not unreachable to root. refuse-machine.v1 by enrollment removes "each machine that joined by a named enrollment, that no node represents" (enrollment.yaml:198-199); expiry raises it as kind machine (enrollment.yaml:229-230); root typed its id in the admission. That other unrepresented machines get no node is decided: placement.yaml:400 "A report alone never makes a node".

**Verifier's corrections.** Split it. (1) Gap, medium: reconciliation of admit-machine.v1 (and by the same text enroll-machine.v1 and refuse-machine.v1) has no rule like LIF-028, and an accepted machine without a node cannot be admitted, only refused and enrolled again. (2) Showing every machine that no node represents is a feature, not a gap: the owner's, or ruling 71 point 4. The sentence 'root can remove it only by an id that no tool or panel now shows' is wrong for the accept-unknown case. The ENR-007 fix adds behavior on reconciliation: log to strike.


## FR-053

**Replacing the machine provider strands every other node for good; the text says 'until its machine is enrolled again'**

L05/F7 · contradiction · verifier: confirmed, medium · decides: owner · known-new-situation

**Promises.** PRV-013, PLC-026, PLC-023, ENR-006, PLC-011, PLC-017, PRV-018

**Where.** spec/promises/providers.yaml:409

**Quote.** "A later provider does not inherit the earlier provider's ids or nodes: each non-local node stays omitted until its machine is enrolled again, and a new admission makes a new node, not a rebinding of the old one (PLC-026)."

**Situation.** The user swaps machine provider P1 for P2. Nothing guards P1's retirement as PRV-020 guards a runtime provider's. Every other node is then omitted, and stays so: P1 is never brought back, and "Only a matching triple can report that node reachable again" (placement.yaml:338), so the 'until' never comes true for the old node. Enrolling the machine through P2 under its hostname fails ENR-006 check 3 while the old node is not retired. The old node is retired only once its instances are removed (PLC-011), and their retirements cannot succeed, because PLC-023 holds back the hook and the stop for an omitted machine; or by declare-lost, which retires them all with shutdown unknown and closes their Conduits. Each is then unregistered, its tombstone lifted (LIF-041), the machine enrolled again with a fingerprint, and every instance and Conduit registered anew. PRV-018 says a machine provider "can so be swapped without changing who may reach what" (providers.yaml:563).

**Counterargument.** Binding a node to the provider Resource and id is batch 1's reading O2, logged for the owner to strike, and follows ruling 52 point 4; a swap of machine provider is rare.

**Least change.** Text: PRV-013 and PLC-026 say that the old node never becomes reachable again and must be retired or declared lost before its hostname is admitted through another provider, and PRV-018's sentence names the runtime provider only. Behavior, for the owner: (A) a machine provider's retirement reports conflict while a node that is not retired is bound to it, as PRV-020 and PLC-011 do; or (B) an admission through the new provider of a machine with an existing node's hostname and matching fingerprint rebinds that node.

**Precedent.** Kubernetes in-use protection: a finalizer such as kubernetes.io/pvc-protection postpones removal while something still depends on the object.

**Who decides.** owner. O2 is a reading the owner may strike, and the rule decides whether a provider can be swapped, which is purpose (ruling 40 point 2, swappability). The text fix alone is the coordinator's under ruling 37.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/providers.yaml:408-411; repeated at placement.yaml:404-405. The order of events it describes cannot happen: enrollment under the same hostname fails check 3 while the old node is not retired (enrollment.yaml:135), and placement.yaml:337-338 "Only a matching triple can report that node reachable again". No guard on the provider's retirement: providers.yaml:402-404 "an actor retires the one in force and registers another. Retiring it retires no node". The stranding itself is the logged reading O2: docs/proposals/spec-batch-1/areas/machines-and-trust.md:308 and docs/work-orders/038b-spec-enrollment.md:163-166; IN_FORCE.md:223 states it as in spec. The reviewer's mark known-new-situation is right.

**Verifier's corrections.** Two small corrections. (1) A request path exists besides declare-lost: each retirement does not succeed (PLC-023 holds the stop back), but unregistering the retired instance is not held back (conventions/lifecycle.yaml:752-756) and removes it; the cost is the same teardown. (2) PRV-018's sentence (providers.yaml:563-564) is not contradicted to the letter, since Conduits stay open while a node is omitted; it becomes false once the swap is completed. Owner decides the behavior, coordinator the wording, as stated. F1's fix would add 'and unregistered' to what the swap needs.


## FR-054

**A runtime provider is never told that an instance is gone, and nothing bounds how long it keeps the run's secrets, identity and state**

L05/F8 · gap · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-234.

**Promises.** PRV-011, PRV-022, PRV-029, PRV-021, PRV-008, LIF-035, LIF-036

**Where.** spec/promises/providers.yaml:642

**Quote.** "The engine keeps no memory of a run once its instance is removed: the last stop it asks for is the one that a request to unregister the instance asks for, under LIF-036"

**Situation.** An Agent is run by a runtime provider on another machine. The provider keeps its secret values and identity files "where its contract says" (PRV-011, PRV-022) and supplies a state directory holding the Agent's login (PRV-029). The Agent is retired: stop.v1 succeeds and LIF-035 removes it. That stop is the same call as any other stop: a retired instance may be run again for its retire hook (TPL-018), so the provider cannot treat a stop as the end. PRV-029 has only the engine's own runner remove the directory ("Removal or unregistration of the instance removes it with its contents", providers.yaml:975). So the stored secret values and the login stay on that machine with no term, while Conduit shows the instance removed. The shipped SSH runtime says so: its remote state directory is "kept across starts and never removed (PRV-029)" (packages/provider-ssh-runtime/README.md:49).

**Counterargument.** The place where a provider keeps a run's values is inside the domain of trust, a stated limit (spec/README.md:495), and a provider's contract may say that it deletes them at a stop.

**Least change.** PRV-011 and PRV-022: "It keeps the values no longer than until a stop of the run succeeds; `run.v1` brings them again." stop.v1 gains the input `removed: true`, given on the stop of a retirement that then confirms the shutdown and on LIF-036's stop; a provider that supplies a state directory removes it then.

**Precedent.** The Container Runtime Interface separates StopContainer from RemoveContainer and RemovePodSandbox; the kubelet unmounts a Pod's secret volumes when the Pod is deleted.

**Who decides.** coordinator. Ruling 71 point 4: a small added duty and input in the runtime provider's contract; if it is done as a new operation it adds a feature and is the owner's.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/providers.yaml:642-644. No promise sets a term on what a runtime provider keeps: PRV-011 says only "its contract states where it keeps the value" (providers.yaml:354-355) and PRV-022 the same for identity (providers.yaml:682-683). PRV-029 binds only the engine's own runner (providers.yaml:969, 975) and says of others "supplies such a directory only where its contract states it does" (providers.yaml:979-980). A stop is not final, since a later retirement may ask for the run again (providers.yaml:211-212). The stated limits cover the place, not the term (spec/README.md:495-498, 566-572 "while the instance exists"). packages/provider-ssh-runtime/README.md:48 "kept across starts and never removed (PRV-029)". Not in the known lists.

**Verifier's corrections.** Class and unsafe mark (false: the residue stays inside the stated domain of trust) stand. The proposed `removed: true` has two holes: an instance removed by a reconciled retirement (conventions/lifecycle.yaml:598-599) and an unregister whose stop was never dispatched or received (lifecycle.yaml:763-769) never deliver it. Say so, or state the term as the provider's duty at the first stop that succeeds. A new duty on every runtime provider is ruling 71 point 4 at least; the CRI precedent (StopContainer against RemoveContainer) is apt.


## FR-055

**A Type's need of a state directory has no key, and the text does not say who fails the setup**

L05/F9 · ambiguity · verifier: confirmed, low · decides: coordinator · new

**Promises.** PRV-029, CTR-004, PRV-007, PRV-019

**Where.** spec/promises/providers.yaml:980

**Quote.** "A Type whose implementation needs one states that requirement; setup under a runner that gives none fails with an explicit diagnostic."

**Situation.** An Agent package keeps its home and login in CONDUIT_STATE_DIR. An instance of it names a runtime provider whose contract supplies no such directory. No key of a Type contract states the need, run.v1 has no input for it, and the provider's offer is prose that no engine reads (spec/definitions.md:404). Reading A: the engine fails the setup; it has nothing to read, so the sentence cannot be followed. Reading B: the implementation notices the missing variable; then it is undecided whether the process refuses to start, so that run.v1 ends failed, as packages/provider-local-machine/README.md:31 does, or the setup hook fails, and a plain server does neither. Either way registration accepts an instance that can never be set up, and the fault shows only at setup.

**Counterargument.** The sentence is in `observable` and may be meant as advice to package authors; an implementation that needs the variable will fail in some way without it.

**Least change.** PRV-029: "An implementation that needs the directory and is given no CONDUIT_STATE_DIR ends its run failed with a diagnostic naming the variable; the part reports failed under PRV-019." The owner's alternative, a feature: a Type key and a provider Type key, so that resolving rejects the instance.

**Precedent.** Kubernetes: a Pod declares its volumes and a CSIDriver object declares what the driver supports, so a mismatch is reported before the container starts.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity whose fix names the party and adds no behavior; a declared key would add a feature and is the owner's.

**Verifier.** confirmed, low. Quote verbatim at spec/promises/providers.yaml:980-981. No key holds the need: the only mentions of the state directory in spec/ are PRV-029, PRV-023, MAN-018, LIF-035, DL16, names.yaml and definitions.md:279; no schema or Type key. definitions.md:404 says such a statement is prose "which no engine reads". So the engine cannot be the party that fails the setup, and no promise binds the implementation or the provider to do it. packages/provider-local-machine/README.md:30-31 shows one implementation's choice. Not in the known lists.

**Verifier's corrections.** None of substance. Severity low: one sentence of `observable`; a well-written implementation fails closed. The risk worth stating is an implementation that falls back to a shared home when the variable is absent. Coordinator, as stated; a declared key is the owner's.


## FR-056

**Layers: ENR-005 is marked language, and PRV-002 and PRV-013 decide a document's validity in the convention layer**

L05/F10 · defect · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-042, FR-098, FR-142, FR-159.

**Promises.** ENR-005, PRV-002, PRV-013, PLC-018

**Where.** spec/promises/enrollment.yaml:105

**Quote.** "The panel shows the value exactly as the provider reported it. attestation is a platform or hardware statement that the provider verified."

**Situation.** ENR-005's `layer` is `language` (enrollment.yaml:105). It states what a provider reports at run time and what the engine's panel renders; no resolving of documents answers it (LD1). The Language level covers "Every promise whose layer is `language`: documents in, resolved objects or diagnostics out" (spec/README.md:320), so an engine that claims that level is bound by a promise it cannot keep or test. The other way round, PRV-002 (providers.yaml:44) and PRV-013 (providers.yaml:390) are `convention`, yet reject a document from the documents and what exists alone: a provider given a `runtime` or another placement, a second machine provider, one not owned by `system`. A Language-level engine, or an independent build of the language layer (LD4), accepts those documents, and an Engine-level one rejects them. PLC-018 and PRV-006, rules of the same kind, are `language`.

**Counterargument.** ENR-005 fixes the form of evidence-item.v1, which is a schema matter; PRV-002's second half (the engine's own runner runs it) is behavior, so the promise had to sit in one layer.

**Least change.** ENR-005: `layer: convention`. Move PRV-002's rejection, and PRV-013's rule for an owner other than `system` and for a second provider, into `language` promises, leaving "the engine's own runner runs it" in convention.

**Precedent.** LD1 of the tracker's layer decisions; PLC-018 and PRV-006 in the same files.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect of classification; no behavior changes at the Engine level.

**Verifier.** confirmed, low. Quote verbatim at spec/promises/enrollment.yaml:112-114; `layer: language` at enrollment.yaml:105. It is the only language promise of the twelve, and its sibling on the form of evidence, PND-004, is `interface` (spec/promises/pending.yaml:67-68). LD1: "Language if it can be answered by reading the declarations alone" (docs/AUDIT.md:406). PRV-002 is convention (providers.yaml:44) and rejects a document (providers.yaml:51-53); PRV-013 is convention (providers.yaml:390) and rejects from documents and what exists (providers.yaml:393-399); the like rules PLC-018 (placement.yaml:249) and PRV-006 (providers.yaml:170) are language. No language promise restates them. Not in the known lists.

**Verifier's corrections.** `where` should read enrollment.yaml:105 for the layer and 112-114 for the quote. For ENR-005, `interface` (as PND-004) is as defensible as `convention`. CONTRIBUTING.md:131-132 requires new IDs where content moves, so splitting PRV-002 and PRV-013 costs two new promises; changing a layer in place is allowed.


## FR-057

**Promises in this slice state several obligations for two parties, and rules that bind stand in `observable`**

L05/F11 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** PRV-007, PRV-015, PRV-014, ENR-001, ENR-002, ENR-006, PRV-021

**Where.** spec/promises/providers.yaml:249

**Quote.** "Where the runtime provider reports the run running, nothing is asked for and the part counts as succeeded, so a further setup leaves a running run as it is."

**Situation.** The form says `then` is "The single obligation" and `observable` is "What can be checked" (spec/README.md:291). In these three files `then` holds 3,962 words and `observable` 11,046. Rules found only in `observable`: the one quoted; that a report with a duplicate is not used (PRV-014); the least evidence kinds an enrollment needs (ENR-001); the reason for each failed admission check (ENR-006). A builder who takes the form at its word leaves them out. PRV-015's `then` binds the engine and the provider in one sentence, and ENR-002's holds six duties of the provider, so an Implementation-level claim, which covers a promise "whose obligation falls on the implementation and not on the engine" (spec/README.md:322), cannot say which half it keeps, and a vector's tag cannot say which half it tests. PRV-021's obligation is that a contract's prose states something and the provider does it, which no engine-side test observes. All 63 promises here have `vectors: []`.

**Counterargument.** The files are read as wholes and every builder so far has treated `observable` as binding; splitting would add promises to a catalogue of 519.

**Least change.** State in the form of a promise that a sentence of `observable` binds only where it restates a `then`; move each rule above into the `then` of its own promise; split PRV-015 and PRV-014 into the engine's half and the provider's; add a field `binds: engine | provider | implementation`.

**Precedent.** The Container Runtime Interface states the runtime's duties apart from the kubelet's; RFC 2119 documents keep their requirements out of notes.

**Who decides.** coordinator. Rulings 37 and 46 point 4: the text breaks the specification's own rules 1 and 2 for a promise; no behavior changes.

**Verifier.** weakened, low. Quote verbatim at spec/promises/providers.yaml:249-251. Recounted with PyYAML: 63 promises, `then` 3,962 words, `observable` 11,046: correct. The rule quoted does sit outside its promise's `when` (providers.yaml:213-214 excludes the running case). But: `vectors: []` is the prescribed form (CONTRIBUTING.md:122-123) and holds for 517 of 519 promises; coverage is counted from the manifest, and spec/README.md:214 and 217 give vectors for slices of 7 placement and 9 provider promises. Two of the four 'only in observable' examples are also stated elsewhere: the duplicate rule in list-machines.v1's description (spec/core/operations.yaml:1070-1071), and ENR-001's `then` points at "the checks below" (enrollment.yaml:13).

**Verifier's corrections.** Real but smaller, and specification-wide rather than of this slice. Drop the `vectors` sentence. Who decides: moving sentences into `then` and splitting PRV-015 is the coordinator's; a new field `binds` changes the ruled form of a promise (IN_FORCE.md:38, rulings 17 NS2, 20 and 41 P2) and is the owner's. PRV-021 is an Implementation-level promise (spec/README.md:322), testable by a provider harness; it is not a fault that no engine-side test sees it.


## FR-058

**refuse-machine.v1 has no stated outcome when the enrollment closes and a removal does not succeed**

L05/F12 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** ENR-008, OPS-003, OPS-004, OPS-022

**Where.** spec/promises/enrollment.yaml:202

**Quote.** "A named enrollment's action is answered refused when the enrollment closes, regardless of the removal's outcome."

**Situation.** Root sends refuse-machine.v1 {enrollment: E}; one machine joined by E. E closes refused and its action is answered; remove-machine.v1 then fails or ends unknown. ENR-008 says "A failed or unknown removal is reported as such" (enrollment.yaml:211). Reading A: the outcome is failed; but OPS-003 has failed mean that the effect has not occurred "in whole or in part", and part of it has. Reading B: the outcome is unknown under OPS-004; the invocation is then listed unreconciled at `management`, and a repeat is not dispatched. With two machines, one removed and one not, the same question arises. A caller acts differently on each: a repeat after failed is dispatched as a new attempt, and after unknown it is not.

**Counterargument.** The per-machine diagnostics say what happened, and a fresh request removes what is left under either reading.

**Least change.** ENR-008: "The outcome is succeeded only if every removal succeeded; failed only if no enrollment was closed and no machine removed; otherwise unknown. A diagnostic names each machine and how its removal ended."

**Precedent.** OPS-022.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity settled by the counting OPS-022 already gives for a setup and a retirement.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/enrollment.yaml:202-203; "A failed or unknown removal is reported as such" at enrollment.yaml:210-211. No rule maps several removals and a closed enrollment to one outcome; OPS-022 covers only set-up.v1 and retire.v1 (conventions/operations.yaml:463). spec/definitions.md:234-235 gives failed as "has not occurred, in whole or in part" and unknown as "occurred only in part", which points to reading B. The divergence is live: the coordinator's own build order takes reading A, docs/work-orders/049a1-build-enrollment-core.md:235-237 "it ends failed where any removal failed, and unknown where none failed", marked "The admin's reading, to apply and report". Not on the review list or in the known lists.

**Verifier's corrections.** None to class or unsafe mark. The reviewer's least change agrees with OPS-004 and the definitions; order 049a1's reading does not where the same invocation closed the enrollment, and should change with it (see extra). Coordinator, as stated.


## FR-059

**The name a new node takes ignores tombstones and templates**

L05/F13 · ambiguity · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-087.

**Promises.** PLC-026, LIF-041, OBJ-002, ENR-006, ENR-007

**Where.** spec/promises/placement.yaml:394

**Quote.** "It is named by the part of the hostname before the first dot if that is valid under OBJ-001 and unused by any existing Resource under OBJ-002, otherwise under OBJ-003."

**Situation.** An author's Resource `gpu` was retired; its tombstone holds the kind and name, and no existing Resource uses the name. Machine `gpu.lan` passes ENR-006's checks. PLC-026 picks `gpu`. Reading A: LIF-041 applies to any request that would instantiate an instance, so admit-machine.v1 reports conflict, and root must lift an unrelated Resource's tombstone to admit a machine. Reading B: ENR-006 says the request is "dispatched only if all applicable checks hold" and lists eight; accept-machine.v1 is called, and the node is then either registered past the tombstone or fails after the provider accepted the machine. Also "any existing Resource" may or may not count a Resource template, which OBJ-002 counts.

**Counterargument.** A Resource whose name equals a machine's first hostname label, retired and not lifted, is uncommon, and LIF-041 exempts some engine-made objects in spirit.

**Least change.** PLC-026: "if that is valid under OBJ-001, no object of the kind Resource that exists, template or instance, has it, and no tombstone holds it; otherwise under OBJ-003".

**Precedent.** Kubernetes metadata.generateName, which gives a generated name where the chosen one cannot be had.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity; the fallback to a generated name already exists.

**Verifier.** confirmed, low. Quote verbatim at spec/promises/placement.yaml:394-395. LIF-041 reaches any "request [that] would instantiate an instance" and exempts only "engine-made first-start and default Conduits" (conventions/lifecycle.yaml:857, 872); OBJ-002 warns that "A name held by no existing object may still be held by a tombstone; LIF-041 then gives conflict" (spec/promises/objects.yaml:39-40). ENR-006 lists eight checks with "only if" (enrollment.yaml:131). Order 049a1 repeats PLC-026's words and adds nothing (docs/work-orders/049a1-build-enrollment-core.md:213-214). Not in the known lists.

**Verifier's corrections.** By the letter reading A holds ('only if' is necessary, not sufficient), so this is closer to a harsh decided outcome than to two equal readings: root must lift an unrelated Resource's tombstone to admit a machine. The template half is weak: "under OBJ-002" already means template or instance (objects.yaml:32-33). The fix is right and uses the existing fallback.


## FR-060

**A provider's own server that ends on its own, while the engine runs on, is covered by no promise**

L05/F14 · gap · verifier: weakened, low · decides: coordinator · new

**Promises.** PRV-026, PRV-005, PRV-002, PRV-010, IFC-017, OBS-004

**Where.** spec/promises/providers.yaml:877

**Quote.** "and whose run on the engine's own runner, under PRV-002 or PRV-003, ended when the engine or that part stopped, and has not been asked for since."

**Situation.** The machine provider's stdio server, a child of the engine, crashes; the engine keeps running, and its runner's restart policy (DL16) restarts nothing. PRV-026 runs a provider again only where its run ended with the engine or a part of it. IFC-017 speaks of "a request or a hook" and names two runs asked for again, neither a provider's. PRV-005 decides only a provider that is not ready or lacks the extension; this provider is ready. Reading A: the engine runs it again before the next call it must make, as after a restart. Reading B: OBS-004 forbids that; the regular report counts as not received, every other node is omitted, and every run, stop and placement that needs a provider reports unavailable until an actor sets the provider up again, with the provider's `Running` condition as the only sign. A restart of the engine heals itself, and a crash of one child does not.

**Counterargument.** PRV-010 and DL16 leave restarts to the runner's own policy, and OBS-004 read to the letter gives reading B.

**Least change.** PRV-026's `when`: "ended when the engine or that part stopped, or ended on its own"; PRV-005: "where the provider's server is not running and is not run again, the call is not dispatched and reports unavailable, naming the provider".

**Precedent.** The kubelet dials its container runtime's socket again when the runtime restarts; Terraform starts a provider plugin anew for each operation.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a gap closed by extending PRV-026's own rule; no new mechanism.

**Verifier.** weakened, low. Quote verbatim at spec/promises/providers.yaml:877-879. The text decides reading B. spec/interface/implementation.yaml:482-487: "Where that runner has no process for the instance's run, as once the process has ended ... nothing is sent ... until a further setup asks for the run again, under PRV-007". spec/conventions/observation.yaml:63-68: "stays until an actor asks for an operation that removes it ... and not because the run is seen to have stopped". The runner's own restart policy is DL16's (spec/implementation-defined.yaml:182) under PRV-010. PRV-015 covers a call that "ends with no result" (providers.yaml:476).

**Verifier's corrections.** Not a gap in whether the engine runs the provider again: it does not. What remains is small: PRV-005, PRV-007 and PRV-008 name only 'not ready' and the missing extension among the causes of unavailable, not a provider whose process has ended (ruling 52). The proposed change to PRV-026's `when` makes the engine act without a request where OBS-004 forbids it: that is behavior, the owner's or ruling 71 point 4, not rulings 37 and 46.


## FR-061

**The enrollment's labels go through the provider and back, though the engine holds them and fixes them itself**

L05/F15 · needless-complexity · verifier: weakened, low · decides: owner · new

**Promises.** ENR-002, ENR-006, ENR-010, PRV-014, ENR-001

**Where.** spec/promises/enrollment.yaml:50

**Quote.** "The provider binds the enrollment's labels to the machine so the machine cannot change them."

**Situation.** Root gives labels in enroll-machine.v1; the enrollment holds them (ENR-001). The provider must store them against the machine by a means of its own, "such as tags", and report them back; check 4 of ENR-006 then requires the report to hold exactly the three descriptive labels plus those, and ENR-010 fixes the node's labels as the three reported "plus only the enrollment's labels", which the engine already has. The report also names the enrollment, so the engine can look them up. Every machine provider therefore needs a label store and an exact echo; one without tags, such as an SSH inventory, must build it, and a provider that reports a label of its own, a zone for one, makes every admission report conflict. The echo protects nothing: a later report changes no label (ENR-010).

**Counterargument.** Binding at the provider lets the provider's own network policy use the labels, as tags drive access lists, and a pending machine's panel shows labels the provider vouches for. That use is the provider's own and needs no promise.

**Least change.** ENR-002: drop the sentence quoted; enroll.v1 keeps `labels` as information the provider may use. ENR-006 check 4: "The report holds kubernetes.io/hostname, kubernetes.io/os and kubernetes.io/arch." ENR-010: "plus the labels the enrollment holds".

**Precedent.** Kubernetes: the administrator's labels are set on the Node object in the API server, and the NodeRestriction admission plugin keeps a kubelet from setting them; they are not echoed through the cloud provider.

**Who decides.** owner. It removes a duty from every machine provider's contract, and batch 1's choice C7 on labels is logged for the owner to strike (ruling 68 point 3).

**Verifier.** weakened, low. Quote verbatim at spec/promises/enrollment.yaml:50-51. The mechanics are as stated: check 4 requires exactly the three labels plus the enrollment's (enrollment.yaml:136-137), ENR-010 fixes "plus only the enrollment's labels" (enrollment.yaml:246), which the enrollment already holds (enrollment.yaml:15). But binding by the provider is ruled, not only batch 1's C7: docs/proposals/ruling-52/README.md:25 "labels named by the enrollment and bound by the provider", from decision 3 of docs/proposals/enrollment/README.md:214-216, written when labels were still reported status (docs/proposals/enrollment/README.md:163-166).

**Verifier's corrections.** Known mark: a ruled decision challenged with a new reason, namely that ENR-010 has since made the echo redundant for the node's labels. Basis must cite ruling 52 point 4, which the change amends; C7 covers only 'the three plus the enrollment's'. Owner decides, as stated. The argument is sound for the engine's labels; the binding still serves the provider's own network policy, which needs no promise.


## Further points the verifier noticed

Not verified by a second reader.

- **Order 049a1's reading of refuse-machine.v1's outcome contradicts the definitions where the enrollment closed** (docs/work-orders/049a1-build-enrollment-core.md:235). "it ends failed where any removal failed" is applied after the named enrollment has already closed in the same invocation. spec/definitions.md:234-235 and OPS-003/OPS-004 give failed only where no part of the effect occurred and unknown for a partial effect. The order is checkpointed for `s3/m2`, so the build will carry reading A unless F12's fix changes it. Not binding text, but it is the only place the outcome is decided today.
- **`local` is bound to no machine id, so place.v1 by id cannot name it except by a hostname lookup that no promise states** (spec/promises/providers.yaml:469). PRV-015 has the provider name a machine "by id"; `local` has no `machine {provider, id, hostname}` in status and "represents the engine's hostname alone" (spec/promises/placement.yaml:129-131, 396-398). Under a registered machine provider the engine must map the returned id to `local` through the latest report's hostname; the staged vector row says "never select by hostname alone" (docs/work-orders/038b-staged/049b.md:33). A builder who binds by id only can never place on `local` through any registered provider. Adjacent to the queued item at docs/guides/full-review.md:135 and to F3; F3's fix removes it.

## The verifier's notes

Checked at commit 2eb8f258 (2026-11-12), read-only. All fifteen quotes are verbatim at the cited lines; the only citation slip is F10's `where` (the layer is at enrollment.yaml:105, the quote at 112-114).

Totals: 8 confirmed (F1, F3, F4, F7, F8, F9, F10, F12, F13 is the ninth: 9 confirmed), 6 weakened (F2, F5, F6, F11, F14, F15), none refuted outright, none wholly known. One high (F1), seven medium, seven low.

Things the coordinator should know when acting on these:
- F1 and F7 pull on the same rule (ENR-006 check 3) in opposite directions. F1's fix makes a hostname unusable until the retired node is unregistered; that makes a machine-provider swap (F7) cost one more step per node. Decide them together.
- F3 and F4 are one change to place.v1; F4's candidate list subsumes F3.
- Order 049a1 (checkpointed, not merged, no Record yet) already holds readings for F5, F12 and part of F1 (it adds the machine id to `machine`). It is not one of the designated known-open records, so I did not mark those findings known on its account, but F5(a) is also item 28 of the review list, which is.
- F15 amends ruling 52 point 4, not just a coordinator choice.
- The reviewer's assessment figures recount correctly (63 promises; `then` 3,962 words, `observable` 11,046), but 'no vector listed on any of them' describes the prescribed form of the field (CONTRIBUTING.md:122-123), not coverage.

What I read: the three slice files in full; the brief; AGENTS.md 'What holds'; spec/README.md (form of a promise, conformance, the limits on pending actions and on secrets); the known-open lists in AUDIT, cold-readers, full-review and IN_FORCE topics 2, 10 and 11; the review list in full; G5's findings table; the cited promises LIF-017, 024 to 026, 028, 035, 036, 040, 041, OPS-003, 004, 009, 012, 014, 022, ACC-032, 033, OBJ-001 to 003, IFC-017, OBS-004, DL5, DL16; the provider and enrollment contracts in spec/core/operations.yaml; order 038b in full and order 049a1 steps 2 to 7 and its report list; the staged 049b patch for place.v1 and list-machines.v1.

What I did not do: read spec/definitions.md line by line (searched it by term); read the ledger beyond searches for hostname, fingerprint, state directory, place.v1 and machine provider; read the SSH runtime provider's code to see whether it would in fact be fooled in F1 (the spec-level finding does not depend on it). Nothing was built or run.
