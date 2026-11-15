# P07-engine-build: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 2 and 3, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** The first build is real and disciplined: a strict loader, a core that is pure by inspection, and a shell that ran real Claude Code and Codex through S1, with 494 cases passing and documents that are candid about their limits. It is an S1 engine. None of the ten v0 capabilities I was asked about is on main: the https address is on a side branch, runtime-provider calls and enrollment are checkpointed with musts (enrollment on a base that predates four core merges), and sign-in, pending actions, secrets through a store, declare-lost, plan and apply, metrics and appointment are unbuilt, the last two with no promises yet. By count, 342 of 519 promises are named in engine code and 292 have a case; by size, what is parked or planned for batch 1 alone is 19,000 to 22,000 lines against 35,245 merged, before batch 2, secret delivery, package sources, control dispatch and metrics. My judgment is that the engine is about half of a v0 reference engine, and less by scenarios: S2, S3, S5 and S10 each need something main lacks. The largest risk is not the missing features but that the engine is its own oracle: the vectors drive and assert its internal state, citations are bundles of up to 74 promises checked only for presence, 227 promises have no case, and the specification's own by-rule examples are never run through the core, so today's evidence cannot show a promise unmet and no second builder could use it. The second risk is structural: every commit rewrites the whole state, with every record, each invocation's held context (one measured at 373,365 bytes) and package files as JSON number arrays, and state is untyped JSON read through 210 string keys; every further order builds on both. One defect is safety-relevant now: a fence is recorded when a kill signal is sent, not when the run is seen to have ended. I would fix that, move the store to an existing transactional one, write every new vector at the public boundary with its deciding promise, and merge or drop the four parked branches before starting more. Two decisions are the owner's: whether that consolidation goes ahead of batch 1's remaining core orders, and which capabilities the v0 engine must hold.

**What the analyst read.** Tree: the repository root at 2eb8f258 (2026-11-12), clean. Nothing was built or run, so no behavior was observed: where I say the engine accepts or lacks something, the evidence is the absence of code or of a case, not a run.

Read in full: docs/guides/full-review.md; docs/implementation/HANDOVER.md; AGENTS.md 'What holds'; engine/CONFORMANCE.md (all 397 lines, every DL row); docs/implementation/FIRST_BUILD.md; docs/proposals/first-build/README.md, log-increment-4.md, to-review-with-the-owner.md and increment-2/decisions.md (long table cells cut at about 500 characters); engine/crates/shell/src/store.rs; engine/crates/core/src/lib.rs lines 1 to 620; order 070's What and why, scope, done-when and Record; order 042's Record.

Read in part: engine/README.md lines 1 to 330 of 757; core access.rs (85-165, 425-900), requests.rs (37-97, 372-425, 715-830, 1020-1160, 1445-1700), retirement.rs (186-420), lifecycle.rs (150-180, 499-640), records.rs (40-200); shell lib.rs (60-100, 340-540, 600-700), server.rs (30-180, 400-425), runner.rs (46-54, 195-232); conformance runner lib.rs (745-760, 960-982); conformance/README.md (intro, assertions, planned work, increment 2's done-when); ROADMAP.md (order of the work, register, scenarios; cells cut at 1,500 characters); docs/proposals/spec-batch-1/README.md 'The work orders'; docs/implementation/PROVIDERS.md status and 'What main has now'; docs/AUDIT.md Purpose, Layered claim and the rows A14, A34, A35, A38, A39, A45, A50, A63; INDEPENDENT_BUILDS.md gate rows; promises PKG-023, PND-009, PND-010, SEC-002, SEC-007, SEC-010, SEC-013, SEC-023, OPS-005, OPS-008, OPS-014 and DL2, DL10.

Measured by script or grep: every promise ID in spec/ (519: 226 language, 245 convention, 48 interface, recounted); conformance/manifest.yaml parsed whole; all 494 case.yaml parsed for input kinds and expectation keys (one, loader/strings, is unreadable by PyYAML by design); promise IDs named per crate; decision and cite call sites; function lengths; whole-state copies; diff statistics of seven unmerged branches against their merge bases (branch contents not read).

Not read: the loader's source beyond greps; core schemas.rs, packages.rs, cascade.rs, inventory.rs, revisions.rs, differential.rs past line 60, and most of registration.rs; runner.rs beyond the stop and drop paths; all shell tests; the increment 3 closeout beyond its headings and 'not shown' lines; the increment 4 run reports and measurement plan; the packages' code; the scenario plans except lines found by grep. The brief's '53 MB state file' is not in the tree or in any commit message (git grep, git log --grep): I report the mechanism that produces a file of that order, not the figure.


## FR-344

**What of v0 is not built: none of the ten named capabilities is on main**

P07 I1 · fact · verifier: supported · decides: none

The merged engine runs S1 on one machine: loopback http, the built-in runner, npm packages over stdio, one OS user. Of 519 promises, 342 are named anywhere in engine source (comments and tests included, so an upper bound) and 292 have at least one case. Named in code by family: ENR 0 of 12, SCT 0 of 15, PND 2 of 13 (both stubs), SEC 3 of 24, PRV 11 of 30, PLC 10 of 21.

Capability | Principal promises | On main | Vectors | Order that would build it
Sign-in | PND-005, PND-010 to PND-013, IFC-016, IFC-049, CTR-013, DL18 | No. sign-in.v1 is a stub answering unavailable, 'not built yet, order 048a' | None for PND; CTR-013 has a case | 048a, 048b, 050, 051b: none written
Pending actions | PND-001 to PND-009, DL19 | No. answer.v1 is a stub ('order 047'); no pending model | None (PND 0 of 13) | 047 (after 046), 051b: none written
Enrollment and machines | ENR-001 to ENR-012, PND-008, PLC-023, PLC-024, PRV-015 | No. 049a1 is on a branch: 3,980 engine lines, one must and one should left, based before 041b, 070, 077 and 042 and overlapping main in 16 files; the spec's engine bindings are still staged patches | None (ENR 0 of 12) | 049a1 to finish, then 049b, 051c, A63-MDIS, SSH-M: unwritten
declare-lost.v1 | PLC-017, PLC-011, PLC-024, LIF-034 | Routing and root's protection only; a call answers JSON-RPC -32603 with no record | Refusal slices only (9 cases mention it) | None assigned ('none yet: ROADMAP.md S3, AUDIT.md A35'); S3's plan proposes S3-L1
Secrets read through a store | SEC-005, SEC-006, SEC-009 to SEC-024; the engine's reads are SEC-010, SEC-013, SEC-023 | No. The core emits no get-secret read; only SEC-004, SEC-007, SEC-008 are named; the file-store package (857 lines) exists and nothing calls it | SEC-007 (1 case), SEC-008 (4) | SEC-D, a proposal in S2's plan, after 080: unwritten
Metrics | None: spec pending, 'the metrics batch after batch 2' | No | None | None
Plan and apply | None bind (tooling, ruling 76); rests on SCT-001 to SCT-016, LIF-037, OPS-020 | No. The CLI has 12 commands and neither; the short form is not in the loader | None; the reference's golden cases bind nothing | 043a, 043b, 051a, 052a, 052b: none written
The https address | DL13, IFC-042, IFC-022 | No: loopback http only. 079 is merged to s3/m2 alone (1,404 engine lines) | Shell tests on that branch | 079, done on s3/m2, merges after the rerun's pin
Runtime-provider calls | PRV-007, PRV-009, PRV-019, PRV-021, PRV-026, PRV-030, IFC-017 | The core emits the intents; the shell answers them Missing (inspect unknown, run failed, stop unknown). 080 is on a branch with three musts | Core side yes (PRV-007 in 10 cases); shell none | 080 round 3, into s3/m2
Appointment | None: spec pending, batch 2 (make.v1, keep, withhold, acting for a Fleet) | No | None | Batch 2, not begun, after S1's bar

Also unbuilt and not on the list I was given: control dispatch (OPS-020, REV-006, REV-007, REV-009, REV-015, TPL-010): serve refuses a store in which a consumer other than root would be presented a control, and S5 is that scenario; the short form (SCT, on the rerun path, 043a and 043b); refusal tiers (ACC-056, IFC-050; order 046, first on the critical path); https fetching and package sources (PSD1 to PSD3, proposals).

Size: about 11,000 engine lines wait on four branches, and batch 1's unwritten orders are estimated by their own plan at 8,200 to 11,400 lines, against 35,245 merged. 049a1 came in at 3,980 engine lines against an estimate of 1,200 to 1,500.

**Evidence.** Promise count: grep -hoE '^- id: [A-Z]+-[0-9]+' spec/promises/*.yaml spec/conventions/*.yaml spec/interface/*.yaml | wc -l gives 519. Named in code: grep -rhoE '[A-Z]{3,5}-[0-9]{3}' over engine/crates/{core,loader,shell}/{src,tests}, intersected with the 519, gives 342. Cases: conformance/manifest.yaml parsed with /usr/bin/python3: 494 cases, 292 promises with at least one.
Stubs: engine/crates/core/src/requests.rs:1046-1058 ('not built yet, order {order}').
declare-lost and operation lists: engine/crates/shell/src/server.rs:42-66; engine/CONFORMANCE.md:130 'Declare-lost.v1 and control dispatch remain unfinished public invocation paths'; :138 'without committing a transition'.
Secrets: grep -rn 'secret-provider' engine/crates/*/src gives nothing; grep 'get-secret|secrets' engine/crates/core/src/lifecycle.rs gives nothing.
Metrics: grep -rni metric engine/crates/*/src gives nothing; IN_FORCE.md:148.
CLI: engine/crates/shell/src/help.rs:1 'const COMMANDS: [(&str, &str, &str); 12]'; grep -rln sectioned engine/crates/*/src gives nothing.
https: engine/CONFORMANCE.md:34 'accepts an http engine address on loopback only'; git log origin/main..origin/s3/m2 shows 9b8f3326 only.
Provider calls: engine/crates/shell/src/lib.rs:83-85 and 671-690; engine/CONFORMANCE.md:41 'Runtime provider calls and remote connections are unserved'.
Controls: engine/crates/shell/src/server.rs:148-168 (preflight) and :141.
Branches: git diff --shortstat $(git merge-base HEAD B) B -- engine/ for B = origin/s3/m2 (1,404 insertions), origin/wo/080-shell-runtime-provider-calls (4,570 including 079's), origin/wo/049a1-build-enrollment-core (3,980), origin/wo/071-conformance-manifest-pieces (2,411).
Estimates: docs/proposals/spec-batch-1/README.md:544-640 and :590 'About 1.2-1.5k lines'. Orders unwritten: ls docs/work-orders shows no 043, 046, 047, 048, 049b, 050 (only 050a), 051, 052 or 053.
Merged size: find engine -name '*.rs' | xargs wc -l gives 35,245.

**Waits.** I6 (the scope question) and the summary's distance judgment rest on this table.

**Verifier.** supported. Recounted at 2eb8f258. 519 promises (grep '^- id:' over spec/promises, spec/conventions, spec/interface). The analyst's command gives 342; manifest parsed with /usr/bin/python3: 494 cases, 292 promises with a case. Family counts reproduce (ENR 0/12, SCT 0/15, PND 2/13, SEC 3/24). Stubs: engine/crates/core/src/requests.rs:1046-1058. Operation lists and declare-lost: engine/crates/shell/src/server.rs:42-66; preflight :148-168. No 'secret-provider', 'sectioned' or 'metric' in engine/crates/*/src. CLI: engine/crates/shell/src/help.rs:1 (12 commands). Provider calls default to Missing: engine/crates/shell/src/lib.rs:83-85, :667-690; engine/CONFORMANCE.md:41. Branch diffstats under engine/ reproduce exactly: s3/m2 1,404; wo/080 4,570; wo/049a1 3,980; wo/071 2,411 (and 648 deletions). git log origin/main..origin/s3/m2 is 9b8f3326 only. Unwritten orders confirmed by ls docs/work-orders. Estimates sum to 8.2k-11.35k (docs/proposals/spec-batch-1/README.md:545-645). 9 case files mention declare-lost. 35,245 lines in 64 .rs files.

**Verifier's counterargument.** The table is sound. Its size comparison flatters the remaining work: the estimates it sums have no figure for 052a, 052b and 053, include package and spec lines (050, 043b), and the one order that can be checked against its estimate overran by far more than the item says (see corrections). So '19,000 to 22,000 lines' is a floor, not a range.

**Verifier's corrections.** (1) 342 counts a text fixture: 25 of those IDs occur only in engine/crates/loader/tests/expected/invalid-verdicts.txt (example file names). In Rust source under the same directories the count is 317 (319 with the conformance crate; 298 in non-test core source). (2) 049a1 is not order 049a: its own order file says it is 'the first half of batch 1's order 049a', with 049a2 (PND-008's pending actions, ENR-012's removal question) to follow 047 and 048a (docs/work-orders/049a1-build-enrollment-core.md:27-35). So 3,980 lines stand against half of a 1.2-1.5k estimate, and the enrollment row should list 049a2 as unwritten. (3) The Publication row is ROADMAP.md:35, not :34. (4) The CORE LANE line is spec-batch-1/README.md:470 and 'Hot files' is :668.


## FR-345

**The largest risk: the engine is its own oracle, so its evidence cannot show a promise unmet and no second engine could use it**

P07 I2 · assessment · verifier: partly · decides: coordinator

The first build's second thesis is that the language fully determines what documents mean, tested by 'the core cites a promise for every decision'. What exists tests something weaker.

1. Citations are bundles. The core has 84 decision sites in 14,109 lines. One decision, register-project-atomically, cites 74 promises; resolve-snapshot-context 21; dispatch-invocation 12; end-invocation a fixed 11 whatever branch chose the outcome. The runner checks only that each decision names at least one known ID and that each required promise is somewhere in the step's union. That detects a decision with no citation. It cannot detect behavior that no promise requires, or a wrong deciding promise. The conformance README says as much: 'executable citation coverage, not a proof that every citation is sufficient'.

2. The vectors are written in this core's shape. Inputs are its Input variants; 399 of 1,708 steps reach their situation through fixtures that patch internal state; expected objects are exact arrays carrying internal keys (resolved_implementations 290 times, nested 292, local_name 215, first_start 88). Another engine would have to copy the internal model to run them. A39's done-when is 'Any engine can run them through an adapter'; LD4 wants the conventions shown 'through vectors' for two builds.

3. Coverage is thinner than the case count suggests: 227 of 519 promises have no case; of S1's own 308, 82 have none; the specification's 41 by-rule examples are checked only to pass the schemas, never to be rejected by the core, and eight of their rules are named nowhere in the engine.

What works: where the answer is a reason, the citation is precise (38 access verdicts and 58 registration faults each carry one rule), DL choices are kept apart, and exact arrays do catch a missing or extra record. What does not: outcomes, and anything outside S1. The danger is drift: each order adds cases in the internal shape (041a needed a scripted transform over about 170 case files), expectations follow the engine, and the specification stops being what is tested. I7 and I14 are the changes.

**Evidence.** Decision sites: script over engine/crates/core/src/*.rs (non-test) matching decision( and cite( gives 84; promise counts per site 0 (computed) to 74.
engine/crates/core/src/registration.rs:2091-2166 (74 IDs between those lines by grep -o count); engine/crates/core/src/requests.rs:1480-1488 'end-invocation' with 11 IDs; engine/crates/core/src/packages.rs:642.
Runner: engine/crates/conformance/src/lib.rs:750-759 ('decision lacks a requiring known promise') and :965-982 (cited = union over decisions; error only if a required rule is absent).
conformance/README.md:697 'This is executable citation coverage, not a proof that every citation is sufficient.'; :74 'Arrays have exact cardinality and order'.
Vector survey (PyYAML over 493 parsed case.yaml, 1,708 steps): input kinds lifecycle-fixture 176, request-fixture 133, access-fixture 90, restore-state 11; fixture patching at engine/crates/conformance/src/lib.rs:463-560 ('access::patch(&mut next, &fixture["patches"])').
Coverage: manifest parsed: 292 of 519 promises with a case; s1_coverage 308 entries, 226 with vectors, 82 without.
By-rule examples: ls spec/schemas/examples/invalid-by-rule | wc -l gives 41 (30 rule IDs); engine/crates/loader/tests/examples.rs:249 'every_invalid_by_rule_example_passes_the_schemas'; CTR-009, PLC-028, PRV-001, PRV-004, PRV-016, SEC-001, SEC-022, SEC-024 are absent from all engine source and from the manifest.
Precise citations: grep -c 'Verdict::reason(\|Verdict::restriction(' access.rs gives 38; grep -c 'fault("' registration.rs gives 58.
docs/AUDIT.md:464 (A39), docs/implementation/INDEPENDENT_BUILDS.md:112 and :114 (G2, G4).

**Waits.** G2 and G4 cannot be counted in this form; A34 (two builds) and A39's adapter wait on it; I7 and I14 act on it.

**Verifier.** partly. Holds: 85 decision/cite call sites in non-test core by my script (analyst 84); register-project-atomically 74 IDs (engine/crates/core/src/registration.rs:2091), resolve-snapshot-context 21 (packages.rs:642), dispatch-invocation 12 (requests.rs:1285), end-invocation 11 (requests.rs:1480-1488). Runner checks as described: engine/crates/conformance/src/lib.rs:750-759 and :965-982. conformance/README.md:697 and :74 quoted correctly. Fixture steps reproduce exactly: lifecycle-fixture 176, request-fixture 133, access-fixture 90, restore-state 11. Coverage reproduces: 292 with a case, 227 without; s1_coverage 308 = 226 + 82; 211 outside S1, 58 with a case. 41 by-rule files, 30 rule IDs; only tests are engine/crates/loader/tests/examples.rs:249 and :269. The eight IDs are absent from every file under engine/crates. 38 Verdict sites and 58 fault sites recount exactly. A39 and G2/G4 quoted correctly (docs/AUDIT.md:464; docs/implementation/INDEPENDENT_BUILDS.md:112, :114).

**Verifier's counterargument.** The evidence carries 'vectors are in this core's shape and a second engine could not run them', and more strongly than the analyst says: 338 of 494 cases start from the snapshot recipe, and the shared snapshot 'is a supplied fixture state, not a state the engine can reach' (conformance/README.md:65-66); 576 steps assert internal state. It does not carry 'cannot show a promise unmet': 240 steps assert a diagnostic ruleId, 905 carry required_promises, and exact arrays of records and intents do fail on a wrong result; the analyst's own 'what works' paragraph concedes this. The accurate claim is narrower: the evidence cannot show behavior no promise requires, a wrong deciding promise for an outcome, or anything about 227 promises. The item also ignores the project's own registered answer to exactly this risk: mutation testing of the model against the vectors, 'Whether the vectors actually pin behavior' (ROADMAP.md:128, A44; G6 at INDEPENDENT_BUILDS.md:116). Running A44 on the core now would measure the oracle problem directly; I14 measures a small corner of it. One real instance of the oracle following the engine exists and was not cited: order 070's Record corrected the differential reference to match the lazy path (docs/work-orders/070-build-state-copies.md:352-361).

**Verifier's corrections.** 494 case files parse, with 1,717 steps (not 493 and 1,708). CTR-009 is not absent from the manifest: it is an s1_coverage entry with no vector and a stated reason (conformance/manifest.yaml:6098-6102); the other seven are absent. Ten by-rule IDs have no case, not eight: PKG-030 and PLC-025 are named in core code but have no case. By case tags 234 of S1's 308 are covered, not 226: eight s1_coverage entries list no vector while a case tags them (OBJ-001, OBJ-016, OBJ-019, OBJ-022, OBJ-023, CON-031, LIF-032, REV-003).


## FR-346

**Unsafe candidate: a fence is recorded when a kill signal is sent, not when the run has ended**

P07 I3 · assessment · verifier: partly · decides: coordinator

OPS-014 lets an actor record that an unknown effect did not occur only once every attempt is fenced, 'once no copy of its dispatch remains in flight, or the implementation confirms that the attempt can no longer act'; a repeat may then be dispatched as a new attempt. For a built-in run the fence is set when the runner is dropped. ProcessGroup::stop sends SIGKILL to the process group and returns without waiting; Runner::drop submits builtin-run-stopped in the next statement; the core then marks every built-in attempt to that instance fenced, citing OPS-014. CONFORMANCE.md says of this: 'it must await an actual end before this evidence is sound'.

Two facts make it more than a nicety. A process that left the group is not signalled at all, which the same page states as a limit, and the dogfood smoke test saw exactly that: Codex's terminal children outlived the group kill. And ruling 70 runs Agents that execute code unconfined. So an engine following its own rules can call an attempt fenced while a child of it can still act, accept 'did not occur', and dispatch the retry: an effect repeated, which the tracker classes unsafe. It is a defect of the build, not of OPS-014.

Fix, in the shell only: submit the observation after the group is seen empty (after reaping, a signal-0 probe of the group fails), and where the runner cannot know that no descendant escaped, submit nothing, so the attempt stays unfenced and OPS-030 keeps answering conflict. Existing paradigm: systemd counts a unit stopped when its control group is empty, not when the signal is sent; where no such containment exists, as on macOS, make no claim. Cost: a small order and one test with a child that calls setsid. It also belongs in increment 4's log with the escaped children, which ruling 70 asks be logged and which are not yet.

**Evidence.** spec/conventions/operations.yaml, OPS-014 'then' clause.
engine/crates/shell/src/runner.rs:46-54 ('libc::kill(-self.pid, libc::SIGKILL);' then return); :208-225 (Drop for Runner: 'run.group.stop();' followed by submit of {"kind":"builtin-run-stopped"}).
engine/crates/core/src/requests.rs:75-94 (marks 'fenced' true for each builtin_run attempt; decision 'fence-stopped-builtin-run' citing OPS-014).
engine/CONFORMANCE.md:41 'The shell submits that observation only on Runner drop, immediately after signalling SIGKILL rather than observing process exit; it must await an actual end before this evidence is sound.'; :40 'deliberately escaping that group is outside this runner's confinement'.
docs/implementation/HANDOVER.md:150-151 'Codex's terminal (PTY) children escaped the run's process-group kill'; :153 'None is yet a finding in increment 4's log'.

**Waits.** Should be fixed before more orders run through Conduit's own Agents (A78) and before S4's lost-response runs rely on the fence.

**Verifier.** partly. Code as described: engine/crates/shell/src/runner.rs:46-54 (kill(-pid, SIGKILL), no wait); :208-225 (group.stop() then submit builtin-run-stopped); engine/crates/core/src/requests.rs:75-94 (marks builtin_run attempts fenced, cites OPS-014). engine/CONFORMANCE.md:41 states it: 'it must await an actual end before this evidence is sound'; :40 states the escape. docs/implementation/HANDOVER.md:149-150 records Codex's PTY children escaping; :152 says none is logged. The spec's fence event for a run is 'once the engine's own runner has seen its process end, or the runtime provider reports it stopped' (spec/conventions/operations.yaml:361-363).

**Verifier's counterargument.** Three things weaken it. (1) It is not only a build defect. Signal-versus-seen is: the build does not meet 'has seen its process end'. But the route that matters, an escaped descendant, is not excluded by OPS-014 at all: an engine that waits for the run's process to end keeps the promise while a setsid child still acts. The fence's stated limits are the onward call and the remote that never answers (operations.yaml:349-352, :373-376; docs/proposals/ruling-75/README.md:121-124); a local descendant is a third, unstated. That half is a language finding (gap, unsafe candidate) and its natural home is a dated later answer under ruling 75, as the first two limits were. (2) The signal-versus-seen gap alone opens no practical window: the fence is recorded only on Runner drop, at engine shutdown; a group member cannot survive SIGKILL, and no retry can be dispatched until the engine restarts. (3) The proposed fix, 'submit nothing where no descendant can be ruled out', means no built-in run is ever fenced by a stop on macOS, so every unanswered attempt stays unknown for good. OPS-014 allows that, but it removes the only run-end fence the build has and changes what S4 and S6 can show. The item also misses that run-end fences are already planned: S6-END (experiments/017-s6-revoke-in-flight/plan.md:79, :123-132) and S4-LATE (experiments/015-s4-partial-setup/plan.md:154-167). The better change is to write 'await observed end' into S6-END, pull that part forward if A78 needs it, and put the descendant question to the language. Existing paradigm stands: systemd's cgroup-empty test on Linux; on macOS state the limit.

**Verifier's corrections.** 'which the tracker classes unsafe' misapplies the test: unsafe is for a finding by which an engine that keeps every promise misbehaves (docs/AUDIT.md:50). That fits the escaped-descendant route, not the missing wait. The limit is known and stated, so only the link to the dogfood observation and the priority are new. Decider: coordinator for the build fix; the descendant limit is a language question logged for the owner.


## FR-347

**Performance cliffs that remain after order 070**

P07 I4 · fact · verifier: supported · decides: none

070 removed the copies made on every commit and cut the shell's tests from 1900 s to 100 s. These remain.

1. One file, rewritten whole. Each transition serializes all of State (objects, every record, every invocation, every installed file) and fsyncs it. Cost grows with history, not with the change.
2. Package files are in that file as JSON arrays of numbers: the bytes field is a plain Vec&lt;u8>, about 3.6 bytes of JSON per byte. 070 left it by design ('a format version 2, for a separate order'). This is the mechanism behind a test store of tens of megabytes; the figure 53 MB is not in the tree.
3. Installing: the shell turns the archive's bytes into a JSON value with one element per byte, clones and reparses the input, then rereads and reparses state.json from disk for the committed retry. Opening a store parses the file to a generic value and then again to State.
4. Each invocation keeps the whole target and its contract, and keeps them after it ends. D19 measured one entry at 373,365 bytes. 'No key expires and invocation state is never compacted.' OPS-008 lists what must be kept (key, ID, state, digest, operation, caller, instance, Conduit, target and a control's values); the held target is not in it, so this is a build choice.
5. Registration copies the whole state at six sites, not four: two more came with order 042, after 070. A register.v1 runs registration twice (trial, then real), so five whole-state copies; a replace.v1 six. 070 measured register-project at 0.95 s on S1.
6. Per commit, still proportional to the whole: every object cloned for the tool-list comparison; every consumer's tools republished on both sides; every retained output reserialized to measure retention; state directories rescanned.
7. Per call: the server republishes every tool of the caller to learn whether the called one is unfinished. Objects and invocations are vectors of JSON searched linearly (27 scans of invocations and 80 of objects in the core).
8. After 070, listing root's 42 tools takes 0.38 s and one inspect.v1 0.35 s on S1's small store. The differential vector run takes about 2.4 hours and is not in CI. A fresh conversation in dogfooding costs an engine restart of 171 to 191 s, measured on the engine pinned before 070.

There is also no migration path: format version 1 only, 112 unwrap or expect calls in the non-test core against untyped state, and the practice so far is to initialize the store again.

**Evidence.** docs/work-orders/070-build-state-copies.md Record ('1900 s to 100 s ... listing root's tools 14.7 s to 0.38 s (median), inspect.v1 13.2 s to 0.35 s, and register-project 4.0 s to 0.95 s ... Registration's four permitted copies remain'); :304-305 'Package files stay in `state.json`. Keeping them by digest outside it would be a format version 2'.
engine/crates/shell/src/store.rs:245-268 (write_state: serde_json::to_vec of the whole State, then atomic_write with fsync); :199-218 (read: Value::deserialize then from_value).
engine/crates/core/src/archives.rs:33-35 'File { bytes: Vec&lt;u8>, mode: u32 }' under derive(Serialize); :40-44 InstalledPackage.files.
engine/crates/shell/src/lib.rs:398 'json!({"status":"complete","bytes":bytes})'; :409-410; :416 and :382 'store.read()?.state'.
engine/crates/core/src/requests.rs:1236 ('"held_target":target' in the invocation); :1454-1461 (finish removes only previous and retained).
docs/proposals/first-build/README.md:106-110 ('invocation entry stayed at 373,365 bytes'); engine/CONFORMANCE.md:30; spec/conventions/operations.yaml OPS-008.
Whole-state copies: engine/crates/core/src/records.rs:73, :110; engine/crates/core/src/registration.rs:1776, :1987, :2497, :2546; git blame gives 57813f06 (order 042) for :1987 and :2546; trial and real calls at records.rs:80 and :258.
Per commit: engine/crates/shell/src/lib.rs:517-520, :478, :434-440, :526; engine/crates/core/src/access.rs:378-411.
Per call: engine/crates/shell/src/server.rs:411-418 and :106-147. Scans: grep -c 'invocations.iter()' and 'objects.iter()' over core/src.
docs/implementation/HANDOVER.md:209 ('about 2.4 hours'), :148 ('171 to 191 s'), :153 ('still pinned at `38c0b4f`, before 070'); grep differential .github/workflows/*.yml gives nothing.
Unwraps: script over non-test core gives 112.

**Waits.** I8 (the store) and I15 (measuring it in dogfooding).

**Verifier.** supported. 070's Record figures match (docs/work-orders/070-build-state-copies.md:363-367); package files left in state by design (:303-305). Whole-state write and double parse: engine/crates/shell/src/store.rs:245-268 and :196-216. Member::File { bytes: Vec&lt;u8> } under Serialize: engine/crates/core/src/archives.rs:33-35. Install path: engine/crates/shell/src/lib.rs:397-398 (bytes into json!), :409-417 (reparse input, store.read() again). Held context kept: requests.rs:1236; finish removes only previous and retained (:1444-1461). D19 figure: docs/proposals/first-build/README.md:106-110 (dated 2026-11-09). Six whole-state copies: records.rs:73, :110; registration.rs:1776, :1987, :2497, :2546; git blame gives 57813f06 for :1987 and :2546. Per call: server.rs:411-418. Format version 1 only: store.rs:13, :212. 112 unwrap/expect in non-test core reproduces. No 'differential' in .github/workflows. '53 MB' is in no tracked file (git grep).

**Verifier's counterargument.** As a list of facts it holds. Two cautions on what it is used for. The costs after 070 are small on S1's store by the Record's own figures (0.38 s, 0.35 s, 0.95 s), and the one large figure, the 171-191 s restart, was measured before 070 with no cause established in the tree; it may be setup and run restarts (engine/CONFORMANCE.md:140-141) and not the store. So the list shows mechanisms, not a measured cliff on main. Second, increment 4's log still says 'registration's four copies' for finding 4-18 (docs/proposals/first-build/log-increment-4.md:29); the six-copy count is a real correction to a record.

**Verifier's corrections.** Item 6 overstates: a consumer whose dependencies are unchanged has its tools visited once on the before view only; both sides are published only where dependencies changed (engine/crates/core/src/access.rs:395-407). Still proportional to all consumers per commit. Scans by my count: 26 of invocations and 77 of objects (iter and iter_mut), against 27 and 80. The 3.6 bytes per byte figure is right (average 2.57 digits plus a comma).


## FR-348

**Does the engine get a short consolidation before batch 1's remaining core orders are written on top of it?**

P07 I5 · question-for-owner · verifier: partly · decides: owner

In plain words: the next orders on the critical path (046 refusals, 047 pending actions, 048a sign-in) each add a thousand or more lines to the same three files and dozens of cases in the engine's internal format. Two things get more expensive with every one of them: the store that rewrites everything on each commit, and vectors that only this engine can run.

A. No change. Continue batch 1 as planned, rerun S1, judge the bar, consolidate after.
B. The store only, now: package files and ended invocations' held context leave state.json (I8). Everything else as planned.
C. The store now, and from order 046 on every new vector is written at the public boundary with its deciding promise (I7). Typed state and splitting the largest functions wait until after the rerun's pin.
D. Full consolidation now, including typed state and migrating the 494 existing cases, before any further core order.

Recommendation: C. Neither part changes behavior, a vector's verdict or S1's author counts, so the bar is untouched. The store is paid for on every commit and restart of the dogfood engine, which ruling 70 made the first priority. The vector rule costs nothing to adopt for new cases and stops the debt growing. D would delay the rerun the owner's bar waits on, for work that can follow the pin. A leaves 047 and 048a to be built and verified against evidence that cannot show them wrong.

It changes: the roadmap's step 3 order (two orders ahead of 046); engine/CONFORMANCE.md's DL2 and DL7 rows; conformance/README.md. Nothing in spec/ or IN_FORCE.md.

**Evidence.** docs/implementation/HANDOVER.md:192 ('The order is the roadmap's step 3: batch 1, then the S1 rerun, then the bar') and the queue's rows 9 and 12; docs/proposals/spec-batch-1/README.md:544-581 (046, 047, 048a sizes) and :660 ('Hot files: conformance/manifest.yaml, case.schema.json, the runner's lib.rs, registration.rs and requests.rs'); :505-512 (041a's migration of about 170 case files). The costs are in I2 and I4.

**Waits.** The writing of 046, 047 and 048a; re-pinning the dogfood engine; order 071's round 3 (it rewrites the runner and could carry the vector rule).

**Verifier.** partly. Queue and order: docs/implementation/HANDOVER.md:193, rows 9 and 12. Sizes: docs/proposals/spec-batch-1/README.md:545-581. Hot files: :668. 041a's transform over about 170 case files: :500-512. 23 case files assert installed_packages (grep over conformance/**/case.yaml); none mentions held_target.

**Verifier's counterargument.** Three objections. (1) It is not the owner's. The coordinator 'plans and decides, writes the orders' (ruling 48; docs/AUDIT.md:129-131), DL2 and DL7 rows and the runner are the coordinator's (ruling 46 point 4; 070-build-state-copies.md:41-42), and 068, 070 and 071 were inserted ahead of batch 1 orders with no ruling. The brief says not to ask the owner what a coordinator may settle (docs/guides/full-review.md:104-106). Decide it, log it for the owner to strike. (2) Option C contradicts the analyst's own I15, which makes the store work conditional on a measurement that has not been taken ('otherwise ... I8 is no longer optional'). Nothing on main after 070 shows the store costs enough to go ahead of 046. The order that the evidence supports is: re-pin, measure, then decide the store. (3) 'Neither part changes a vector's verdict' is true only if the core's State keeps its shape. I8 as worded moves files out of State, and 23 case files assert installed_packages.

**Verifier's corrections.** 'Each add a thousand or more lines to the same three files' is not what the plan says: 046 is 0.6-0.75k, 048a 0.8-1.1k, and 047's 1-1.5k is mostly a new pending.rs. Recent overruns make the concern fair, but the cited figures do not. A better option than A to D: adopt the vector rule for new cases as a coordinator decision now (no order needed beyond the runner check), run I14 and the re-pin measurement, and take the two cheap storage steps only if the measurement shows growth.


## FR-349

**Which capabilities must the engine hold before v0 is brought for publication?**

P07 I6 · question-for-owner · verifier: partly · decides: owner

In plain words: none of the ten capabilities in I1 is merged, three have no promises yet, and building all that is ruled roughly doubles the engine. The documents disagree on the target: the roadmap says 'The acceptance scenario is S1 and S2 passing there', A50's done-when is 'It runs the scenarios S1 to S10', and publication needs 'complete experiment evidence'.

A. Everything ruled: batch 1, batch 2 (Fleet authority, appointment, keep and withhold, peer engines), the metrics batch, and S1 to S10.
B. What the three central claims need, in this order. The relationship: S1 and S2, which need secrets through a store, sign-in and pending actions. Providers behind contracts: S10, which needs the engine to call all four provider contracts. Two machines: S3, which needs enrollment, the https address and declare-lost. Appointment, peer engines and metrics stay ruled and specified when their batch comes, and are stated limits of the v0 engine.
C. S1 and S2 only, the rest stated as limits.

Recommendation: B. It is what tests the layered claim and ruling 40, which S3 and S10 exist for; C would publish without the provider and two-machine evidence the purpose rests on; A puts appointment and metrics, which have no promises yet and no scenario that needs them, ahead of capabilities that S2, S3 and S10 cannot run without. It also keeps rulings few: B needs no new behavior, only an order and a list of limits.

It changes: the roadmap's order after the bar and the 'Publication' row; IN_FORCE.md topics 9, 10 and 13 where batch 2 and metrics are scheduled; spec/README.md's limits of this version. This question may overlap one from the analyst of the roadmap; it is put here because the table in I1 is its evidence.

**Evidence.** ROADMAP.md:153 ('The acceptance scenario is S1 and S2 passing there'); docs/AUDIT.md:469 (A50: 'It runs the scenarios S1 to S10'); ROADMAP.md:34 (Publication: 'A v0 engine and complete experiment evidence'); IN_FORCE.md:98, :148, :171 (batch 2 and metrics, spec pending); ROADMAP.md:141 and :148 (S3 'Central to the claim since ruling 40'; S10). I1's table.

**Waits.** Batch 2's place in the queue; the metrics batch; what A75 counts as 'the required experiments'; the order of SEC-D, 080, 049a1 and S3-L1.

**Verifier.** partly. The capability facts hold (I1). The three quotes are accurate: ROADMAP.md:152-153, docs/AUDIT.md:469, ROADMAP.md:35. But v0's gate is ruled and written down, and the item does not cite it: IN_FORCE.md:261-269 ('v0 is the reference engine (47). To pass, it must meet all of the following: S1 to S10 run end to end, with their vectors passing ...'); the owner-accepted list in docs/proposals/ruling-47/README.md:33-43. Metrics: 'Metrics are not part of v0's gate' (IN_FORCE.md:148; docs/proposals/ruling-72/README.md:22). Fleet authority: 'Fleet authority ships in v0' (IN_FORCE.md:171; docs/proposals/ruling-61/README.md:17, B1 'Yes').

**Verifier's counterargument.** The premise 'the documents disagree on the target' does not survive. The roadmap's 'acceptance scenario is S1 and S2' is A14, the week of use (docs/AUDIT.md:476), a different item from the publication gate. The gate is S1 to S10 on the reference engine, by ruling 47 and IN_FORCE topic 13. So the question is already answered, and the options misstate the status quo: A includes metrics, which ruling 72 put outside the gate; B and C would amend ruling 47's accepted list (S4 to S9) and ruling 61 B1 (appointment in v0) while claiming 'B needs no new behavior, only an order and a list of limits' and that it keeps rulings few. It would in fact need a ruling that reverses two. B's capability list is also incomplete against the ruled gate: S5 needs control dispatch and S7 and S10 need package sources, which the analyst lists as unbuilt in I1. What is left for anyone is ordering, which is the roadmap's and mostly settled (batch 2 merges after the bar, ruling 71 point 2; scenarios S2 to S9 on the same codebase). If the coordinator believes appointment should leave v0, that is a proposal to amend 61 B1, to be put as that and with A78's orders 4 and 5 named as what it would strand.

**Verifier's corrections.** Drop this as an owner question, or restate it as one narrow proposal: 'amend ruling 61 B1 so that appointment follows v0', with the fact that no scenario pre-registration needs batch 2 (experiments/016-s5-control-contention/plan.md:157; experiments/019-s8-filtered-search/plan.md:161; experiments/020-s9-start-and-repair/plan.md:159) as its evidence. 'Three have no promises yet' is right (metrics, appointment, plan and apply as tooling).


## FR-350

**From the next core order on, write vectors at the public boundary and assert the one deciding promise**

P07 I7 · recommendation · verifier: partly · decides: coordinator

Three rules, adopted for new cases only.

1. A new case's inputs are what a client or a provider can send (first start, install, register, a request, a completion, a setup report, cancel, recover) and its assertions are what a client can observe (the result's reason or outcome, its diagnostics' ruleId, the records, inspection and the tool list). No new fixture that patches state. The 494 existing cases are tagged internal and left alone.
2. Every outcome names one deciding promise, as every reason already must: OPS-005 requires 'an error diagnostic ... whose ruleId is the ID of the promise that decided it'. The core's Decision gains a single decided-by beside its bundle, and the runner compares it. Start with the places that have none today (I10): a retirement's outcome, a completion's mapping to failed or unknown, a setup part's unavailable.
3. The specification's by-rule examples become a core test (I14).

Existing paradigm: Kubernetes conformance runs against the API server and never reads etcd; the JSON Schema Test Suite and the WebAssembly specification tests are data any engine consumes. Conduit's own A39 asks for the same.

Cost: one runner order, which can ride on 071's round 3 since 071 is already rewriting the manifest; some cases get longer because they must reach a state by requests. It buys: an adapter for a second engine becomes thin; G2 and G4 become countable; a wrong deciding promise fails a case; order 046's refusal work lands in the form that will last. What it does not do is migrate the old cases; that is a later, optional order.

**Evidence.** spec/conventions/operations.yaml OPS-005 ('every reason carries an error diagnostic under IFC-050 whose `ruleId` is the ID of the promise that decided it'); docs/AUDIT.md:464 (A39 done-when); engine/crates/core/src/lib.rs:219-227 (Decision: action, promises, implementation_choices, facts, affected); engine/crates/conformance/src/lib.rs:965-982; docs/implementation/HANDOVER.md queue row 5 (071, one must left); I2's evidence.

**Waits.** Order 046's vectors (about 14 new, 45 changed) are the first that would follow it; 071's round 3.

**Verifier.** partly. OPS-005 quoted correctly (spec/conventions/operations.yaml:79-80). Decision's fields: engine/crates/core/src/lib.rs:220-227. Runner union check: engine/crates/conformance/src/lib.rs:965-982. A39: docs/AUDIT.md:464. 071's state: docs/implementation/HANDOVER.md row 5. 046's vector counts: docs/proposals/spec-batch-1/README.md:545-555.

**Verifier's counterargument.** Rule 1 is right in direction and matches A39. Rule 2 contradicts it: Decision is an internal core type, so a 'decided-by' field compared by the runner is one more internal thing a second engine must copy. The public form already exists in the specification: IFC-050 requires every reason to carry an error diagnostic with the deciding ruleId and every engine-decided failed or unknown outcome to carry a diagnostic naming its cause (spec/interface/implementation.yaml:1333-1345), and order 046 builds exactly that ('a diagnostic on every reason and on engine-decided outcomes'). IFC-050 is named nowhere in the engine today and has no case. So the rule should be: new cases assert the public diagnostic's ruleId (the SARIF convention the results already follow), and no new Decision field. Rule 1 also needs a stated exception: recovery, crash and restore states cannot be reached by client requests, and a ban on the snapshot recipe makes every new case replay first start and registration, a cost conformance/README.md:462-476 already measured. Riding on 071's round 3 is the wrong vehicle: 071 is checkpointed with a must in its piece merger, and the handover's lesson is not to widen finishing rounds (HANDOVER.md:302-304). Make it a short separate order after 071 merges, or state it in 046's order text.

**Verifier's corrections.** Decider is right (coordinator; the case form is the admin's by decisions 3, 4 and 7, conformance/README.md:3-4). Nothing in IN_FORCE.md is contradicted (IN_FORCE.md:43, :260).


## FR-351

**Store format version 2: an existing transactional store, files by digest, and only OPS-008's fields kept for an ended invocation**

P07 I8 · recommendation · verifier: partly · decides: coordinator

Replace the one JSON file with what comparable systems use.

1. Package files leave the state and live in a directory keyed by digest, referenced from state by digest. This is the content-addressed store of containerd, npm's cache and Nix. The shell already keeps a hash-keyed artifact cache and an archive-catalog directory, so the idea is present twice.
2. An ended invocation keeps what OPS-008 lists and drops the held target and contract. The one later reader, record redaction, needs the schema reference, not the object.
3. State goes into an embedded transactional store, one transaction per transition, with records as an append-only table: k3s runs Kubernetes on SQLite, containerd and etcd on bbolt. SQLite is the default choice; a pure-Rust store is the alternative if a C dependency is unwanted. If that is too much for one order, an append-only log of transitions with a periodic snapshot is the smaller step and the same paradigm.

No specification change: DL2 and DL7 are implementation-defined and their rows are the coordinator's (ruling 46, point 4); OPS-008's list is kept whole, so the owner's reservation under A14 ('dropping any of it would change OPS-008') is not touched. Cost: one order in shell/store.rs and a small core change at finish; the killed-child crash tests must be requalified; a one-way migration from version 1, or a stated re-initialization. It buys: commit cost independent of history and of installed bytes, a restart that does not reparse megabytes, and a D19 projection for the week's use that is not dominated by a build choice. Typing the state (structs for object, Conduit and invocation in place of 210 string keys) is the natural second order and can wait for the rerun's pin.

**Evidence.** engine/CONFORMANCE.md:18 (DL2 storage: 'the whole core State, including records, in one JSON file'), :25 (DL7: 'retain a virtual directory with bytes/modes in `State.installed_packages`; no compressed cache'), :37 ('hash-keyed store cache'), :251 ('store/archive-catalog/DIGEST').
spec/conventions/operations.yaml OPS-008 'then' (the kept fields); docs/AUDIT.md:476 (A14).
engine/crates/core/src/requests.rs:419-420 (record_content reads v["held_target"]) and :1676 (complete validates outputs against it while in progress).
docs/work-orders/070-build-state-copies.md:304-306; :41-42 ('The DL2 storage row ... changes to say how; that is the coordinator's decision, under ruling 46, point 4').
I4's evidence for the costs.

**Waits.** Re-pinning the dogfood engine to main; A14's projection; any order that adds state (047 pending actions, 049a1 machines).

**Verifier.** partly. DL2 and DL7 rows as quoted: engine/CONFORMANCE.md:18, :25; hash-keyed cache :37; archive-catalog/DIGEST :250. 070 left package files for 'a format version 2' (docs/work-orders/070-build-state-copies.md:303-305) and names the DL2 row as the coordinator's (:41-42). OPS-008's list: spec/conventions/operations.yaml:155-160; the held target is not in it. A14's reservation: docs/AUDIT.md:476.

**Verifier's counterargument.** One factual error and one understated cost. Error: record redaction is not the only later reader of an ended invocation's held context. Reconciling an invocation that ended unknown reads its held contract (engine/crates/core/src/requests.rs:1296-1300; revisions.rs:20, :56, :103, which is REV-010's control reconciliation); setup and retirement read a child's contract for 'effect' (lifecycle.rs:1110, :1154); cascades read the parent's held_target (cascade.rs:163, :256). Dropping target and contract 'at finish' would break reconciliation of unknown outcomes. It is safe only once an invocation is succeeded, failed or reconciled, or if the two facts read later ('changes', 'effect') are kept. Cost: the core's contract is whole State in, whole State out (engine/crates/core/src/lib.rs:311-324) over vectors of JSON searched linearly. A transactional store does not make commit cost independent of history unless the core emits changes or the shell diffs two states; that is a change to the core's boundary and to every vector that asserts state, not 'one order in shell/store.rs'. And the benefit on main is unmeasured (see I4). Better staging, all coordinator's: (a) change only how the shell serializes Member::File (by digest beside state.json, where archive-catalog/DIGEST already is), leaving the core's State and the 23 cases that assert installed_packages untouched; (b) drop held target and contract for terminal invocations only; (c) measure under I15; (d) decide SQLite or a transition log after that. Paradigms named are apt (containerd's content store, k3s on SQLite).

**Verifier's corrections.** 'The one later reader, record redaction' is wrong, as above. 'A restart that does not reparse megabytes' assumes the restart cost is parsing; the 171-191 s figure predates 070 and its cause is not in the tree. No ruling is contradicted; IN_FORCE.md:260 ('It uses existing crates') supports an existing store.


## FR-352

**About 11,000 lines of engine work wait on four branches, the last two core merges had no CI engine run, and the check that guards lazy publication is manual**

P07 I9 · assessment · verifier: partly · decides: coordinator

Parked: 079 (the https address, merged only to s3/m2), 080 (provider calls, three musts), 049a1 (enrollment, one must and one should after five rounds), 071 (the manifest in pieces, one must). Together about 10,960 inserted lines under engine/, against 35,245 merged. 049a1's five verifier rounds ran on a base from before 041b, 070, 077 and 042; it overlaps main in 16 files, among them access.rs, lifecycle.rs, registration.rs, requests.rs and retirement.rs, and 070 changed how the core reads state. Those rounds do not verify what a rebase will produce.

On main: engine test shards depend on a job that also runs every provider package, so a flaky SSH-R test skips them; 077 merged 'on local evidence' and 042 after it. The differential build, which is the independent check on 070's lazy publication and therefore on what each consumer is shown, is in no workflow and takes about 2.4 hours by hand. The access rule now exists in three copies that must agree: a state form, a view form and the differential reference.

The dogfood engine is pinned before 070, and the Agent packages it runs (050a, 076, 078) are unmerged.

What I would change, all the coordinator's: fix CI first, as the handover says, and make the engine's shards independent of the packages' job so a provider test cannot hide a core regression; run the differential vectors nightly; then merge or drop the four branches, oldest base first, before any new engine order starts. For 049a1, treat the rebase as a new round with a fresh verifier, not as a conflict resolution. The pattern to stop is starting engine orders faster than they merge: the plan itself calls the core lane serial.

**Evidence.** git diff --shortstat against each merge base, engine/ only: s3/m2 1,404; wo/080 4,570 (stacked on 079, so 3,166 its own); wo/049a1 3,980; wo/071 2,411. git rev-list --count c8e9aa20..HEAD -- engine/ gives 7 (041b, 068, 072, 074, 070, 077, 042); comm of changed files gives 16 in common.
docs/implementation/HANDOVER.md:198-205 (CI red since #260, 'the engine shards are then skipped'), :210 ('Merged (#263, `575a1bcd`) on local evidence'), queue rows 3 to 5, :153.
.github/workflows/checks.yml:187-189 ('engine-tests: ... needs: rust-dir') and :94 (the matrix covers 'engine/Cargo.toml packages/*/Cargo.toml experiments/*/Cargo.toml'); grep differential .github/workflows/*.yml gives nothing.
engine/crates/core/src/access.rs:98-112 (subset) and :114-142 (subset_in_view), :276-289 and :460-469 (two current_path); engine/crates/core/src/differential.rs:1-2 ('Deliberately retains whole-state copies and independent normalization/lookup').
docs/proposals/spec-batch-1/README.md:487 ('CORE LANE (serial: the orders share registration.rs, access.rs, requests.rs and the runner's lib.rs)').

**Waits.** S3's engine work (all of it is on these branches); any new core order; the dogfood re-pin.

**Verifier.** partly. Branch sizes reproduce (I1). 7 engine commits since c8e9aa20: 041b, 068, 072, 074, 070, 077, 042 (git log c8e9aa20..HEAD -- engine/). Files changed on both sides: 13 under engine/, 3 under conformance/, 4 under spec/core (16 for engine and conformance, 20 in all). CI: engine-tests 'needs: rust-dir' (.github/workflows/checks.yml:187-189), and rust-dir's matrix covers engine, packages and experiments (:94). No 'differential' in either workflow. Duplicate rule forms: engine/crates/core/src/access.rs:98-142, :276-289, :460-469; differential.rs:1-2. Core lane serial: docs/proposals/spec-batch-1/README.md:470.

**Verifier's counterargument.** The title's second claim is wrong and the main recommendation collides with a logged decision. Only 077 is recorded as merged with the engine shards skipped (docs/implementation/HANDOVER.md:210; docs/AUDIT.md:74-75). The same page says the failing test 'passed on 042's run' (HANDOVER.md:203), so 042's shards ran. On the recommendation: 079, 080 and 049a1 are parked on s3/m2 on purpose, 'merged after the rerun's pin' (HANDOVER.md row 11 and :277), so that the build S1's rerun pins does not move. 'Merge or drop the four branches before any new engine order starts' would either reverse that or put 046, the head of the critical path, behind S3 work. The sound version: fix CI and split the engine's archive leg from the package legs; merge 071, the only one bound for main; keep s3/m2 but rebase it onto main after each core merge instead of leaving it to age; treat 049a1's rebase as a new round with a fresh verifier (agreed). Nightly differential run is right and feasible if sharded as the engine tests are.

**Verifier's corrections.** 'The last two core merges had no CI engine run' should read 'one of the last two (077)'. 'Three copies of the access rule' should read: the bound-subset and current-path checks exist in a state form and a view form, and the differential build keeps an independent publication computation; the access decision itself is not triplicated.


## FR-353

**Where the core decides with no promise cited, or cites one that does not say what it does**

P07 I10 · fact · verifier: partly · decides: coordinator

Sampled in access.rs, requests.rs, registration.rs, lifecycle.rs and retirement.rs.

Precise: every access refusal returns one rule (38 sites) in OPS-023's order; every registration fault carries one (58 sites); implementation choices are labelled DL apart from promises.

No promise at the point of decision:
- What a retirement's outcome is (succeeded, unknown or failed, from its parts) and whether shutdown is left unknown or pending: retirement's settle has no decision and no citation. The file's one decision cites 9 to 13 promises once at the start; its remaining 230 non-test lines name none.
- How a completion becomes an outcome: a plain protocol error is failed if the operation is read-only and unknown otherwise; an error result likewise; outputs that fail the schema make it unknown. No citation at any branch; all end in end-invocation's fixed list of 11.
- A setup part marked unavailable because its dependencies did not succeed, and the default conflict: lifecycle.rs has 51 outcome literals and 14 cite calls.
- The routing table for ACC-033: 14 operations routed under one comment.

Cited, but the promise does not say it:
- answer.v1 and sign-in.v1 answer unavailable citing PND-009 and PND-010 ('not built yet').
- An https fetch answers invalid citing PKG-023, whose text says 'Every engine accepts https and file'.
Both break the core's own first rule: 'Unsupported cases and broken embedding inputs are errors, never Conduit reasons.'

Not decided at all:
- SEC-002, a language-layer must: a secret reference to a Resource whose type does not offer secret-provider.v1 is to be rejected. The string appears nowhere in the engine; registration resolves a secret reference as any Resource.
- Eight rules that have by-rule examples in the specification are named nowhere in engine code and have no case: CTR-009, PLC-028, PRV-001, PRV-004, PRV-016, SEC-001, SEC-022, SEC-024.

Outside the core: the shell answers an unfinished operation -32603 with no transition and so no record; and it reads a decision's action name to decide whether to read a file.

**Evidence.** engine/crates/core/src/access.rs:600-686 and :716-893 (Verdict::reason sites); :507-598 (routing; comment at :515).
engine/crates/core/src/retirement.rs:314-343 (settle: 'let outcome = if ... "succeeded" ... "unknown" ... "failed"'; object["shutdown"]); :177-190 (the one decision); grep of promise IDs in the file shows none after line 250; test module begins at :481.
engine/crates/core/src/requests.rs:1562 ('let mut outcome = "unknown";'), :1598-1602, :1608, :1681; :1480-1488.
engine/crates/core/src/lifecycle.rs:592 and :605; counts by grep.
engine/crates/core/src/requests.rs:1046-1058; :1131-1149 ('https fetching and package-source providers are unfinished'); spec/promises/packages.yaml PKG-023 observable ('Every engine accepts `https` and `file`'); engine/crates/core/src/lib.rs:3.
spec/promises/secrets.yaml SEC-002; grep -rn 'secret-provider' engine/crates/*/src gives nothing; engine/crates/core/src/registration.rs:636 and :657.
ls spec/schemas/examples/invalid-by-rule against the IDs named in engine source and in the manifest.
engine/crates/shell/src/server.rs:415-418 ('operation {op} is unfinished'); engine/crates/shell/src/lib.rs:386-390.
DL labels: archives.rs:213, lib.rs:703, cascade.rs:126, schemas.rs:1148, requests.rs:1041.

**Waits.** The two stubs and the https answer go with their orders (047, 048a, PSD3); the uncited outcomes are the first targets of I7's rule 2; SEC-002 and the eight rules are I14's expected failures.

**Verifier.** partly. Holds: 38 Verdict sites and 58 fault sites; retirement's settle decides outcome and shutdown with no decision (engine/crates/core/src/retirement.rs:314-343) and nothing after :250 names a promise before the test module at :481; plain protocol error, error result and failed output schema map to outcomes without a citation (requests.rs:1562, :1595-1608); setup part 'unavailable' and default 'conflict' (lifecycle.rs:592, :605); stubs cite PND-009 and PND-010 for 'not built yet' (requests.rs:1046-1058) and https cites PKG-023, whose text says 'Every engine accepts https and file' (spec/promises/packages.yaml:278); lib.rs:3 quoted correctly; SEC-002 is a language-layer must with no trace in the engine and no example; shell answers unfinished operations itself (server.rs:415-418) and reads a decision's action name (shell/src/lib.rs:386-390).

**Verifier's counterargument.** Several sub-claims are off, and two items are known decisions presented as findings. The stubs are a logged coordinator decision with an exit: C11, 'lift, answer and sign-in answer unavailable ... No stub remains at the rerun' (docs/proposals/spec-batch-1/README.md:310-313), and engine/CONFORMANCE.md:128-129 states them. SEC-002 and the eight rules sit outside S1's 308 (SEC-002 is not in s1_coverage), so they are unbuilt families, which I1 already says, not places where the core decides without a citation. The ACC-033 routing example does not carry: ACC-033 is itself a table, and the function that implements it names it in its doc comment and body (access.rs:506, :515). The more useful statement of the gap is that IFC-050, the promise that requires a deciding ID on every reason and a cause on every engine-decided failed or unknown outcome, is named nowhere in engine source and has no case; the uncited outcomes are its unbuilt half, due with order 046.

**Verifier's corrections.** Retirement's one decision cites 6 promises for unregister or 9 for retire, not '9 to 13' (retirement.rs:177-190). 'No citation at any branch' of completion is too broad: two branches cite (requests.rs:1564-1594: repair-parameter-headers-once with IFC-046 and IMP-007; capability-error-performed-nothing with OPS-007 and IFC-008 or IFC-047). CTR-009 is in the manifest with a reason. lifecycle.rs has 14 cite calls (confirmed); I count 35 lines with outcome literals, the analyst 51 occurrences; unverified.


## FR-354

**What CONFORMANCE.md admits as unmet or partly met**

P07 I11 · fact · verifier: supported · decides: coordinator

The page is organized by implementation-defined item and by work order, names 27 distinct promise IDs in 397 lines, and still opens by saying it covers 'increment 2 and shell orders 020/021/022'. Mapping its statements to promises (line in the page):

- OPS-014, fences and late answers: late answers after the shell's call timeout never reach the core (20); the built-in fence rests on a signal, not an observed end; spontaneous exit, runs ended across restart, provider stop reports and composite parts are not connected (41). See I3.
- PRV-007, PRV-009, PRV-021, IFC-017: 'Runtime provider calls and remote connections are unserved' (41, 269).
- PKG-023, NAM-012, PRV-017: https fetching and package sources report invalid; no removal or mirror (25, 56, 247).
- REC-001: an unfinished operation's call is answered 'without committing a transition' (138). PLC-017, OPS-020: 'Declare-lost.v1 and control dispatch remain unfinished' (130).
- PND-009, PND-010: stubs answering unavailable (128-129).
- Records delivery (DL8): no live feed or cursor (26, 57).
- OPS-008, OPS-034, DL10: 'No key expires and invocation state is never compacted'; no timer, idle stores checked only on opening (30). SUB-011: only the nesting limit is stated (29).
- DL13, IFC-042, IFC-022, PRV-022: loopback http only, 'ca.crt checks nothing at this address' (34); no gate proof, TLS or authorization server (60); a credential valid 100 years (33).
- MAN-010 and the package form: npm and stdio only, the hash 'does not pin the dependency graph', declared engines unchecked, S1 run on Node 21.5.0 against a server declaring 24 or later (37).
- ACC-027, PRV-025, PRV-029: 'This is no OS sandbox' (38); 'Same-user access remains mediated' (39); the shared login and other runs' credentials (203-204); Codex's home instructions reach every Codex Agent, 'a stated leak' (206-208); its collaboration tools stay on (360-372).
- IFC-038: no ttlMs chosen (94); model refresh at turn boundaries unqualified (106-107).
- LIF-011, OBS-002: no form for an ended condition (392-395). TPL-009: SIGKILL at once, 'no graceful-stop interval' (280). SUB-010: a served submission takes absolute values only, settled by ruling 52 (298-302).
- Ten open rows under 'Choices still required': DL1, DL3, DL5, DL7, DL8, DL10, DL12, DL13, DL16, DL17 (53-62); for DL17, 'No distributed configuration or guarantee is demonstrated by this core'.

Not mentioned: DL18 and DL19 have no row; enrollment, the pending-action model, the short form and metrics do not appear; line 269 says secrets have 'the unserved boundaries listed above' and nothing above lists them.

No document in the tree says, promise by promise, met, partly met or unbuilt. The manifest covers S1's 308 with a case or a reason and is silent on the other 211, of which 58 have a case. Other stale figures: the engine README lists State without tombstones and bound_targets and first-start without trustDomain; the conformance README says 217 of 308 have vectors (226 now) and gives the directory as about 8 MB (18 MB now).

**Evidence.** engine/CONFORMANCE.md at the lines given in the body; :3-4 (header); grep -oE '[A-Z]{3,5}-[0-9]{3}' engine/CONFORMANCE.md | sort -u | wc -l gives 28 including the string SHA-256; grep -c 'DL18\|DL19' gives 0; spec/implementation-defined.yaml:244 and :252 (DL18, DL19).
conformance/manifest.yaml parsed: s1_coverage 308 entries (226 with vectors, 82 without); 211 promises outside it, 58 with a case.
engine/README.md:43-44 against engine/crates/core/src/lib.rs:65-84 (tombstones, bound_targets); README.md:56 against lib.rs:201-206 (trustDomain).
conformance/README.md:702 ('217 have vectors; 91 have explicit reasons'), :469; du -sh conformance gives 18M.

**Waits.** A per-promise conformance table is what a published reference engine needs (LD5: 'a recorded conformance document'); it can be generated from the manifest once I7's deciding promise exists.

**Verifier.** supported. Every line reference I opened says what the item reports: engine/CONFORMANCE.md:3-4 (header), :19-41 (DL rows), :53-62 (ten open rows: DL1, DL3, DL5, DL7, DL8, DL10, DL12, DL13, DL16, DL17), :94-107 (no ttlMs; model refresh unqualified), :128-130, :138, :199-208, :247, :269-270, :279-280, :294-302, :360-372, :392-395. 27 distinct promise IDs (28 with the string SHA-256). No DL18 or DL19 row; both exist at spec/implementation-defined.yaml:244 and :252. Manifest counts reproduce. engine/README.md:43-44 omits tombstones and bound_targets against engine/crates/core/src/lib.rs:65-84.

**Verifier's counterargument.** The conclusion that no document gives a per-promise standing is right and matters for A50's 'recorded conformance document' (LD5). One nuance: generating that table does not need I7's Decision field; the manifest plus IFC-050's public ruleId would do.

**Verifier's corrections.** The 8 MB figure is not stale: conformance/README.md:469 gives it as the size 'Before this order', a dated measurement. Stated limits the list leaves out: the 4 MiB HTTP body limit (:118); the 1,024-notice queue whose overflow closes a legacy session (:100-106); 'No real Agent client is demonstrated by these tests' (:150-151); the 180-second per-turn deadline that 'poisons that session' (:159-160), which describes the 0.1.1 packages and no longer matches the .v2 Agents the dogfood store runs. The promises' own 'vectors' field is empty in 517 of 519 promises, so the specification itself reports no coverage; only the manifest does.


## FR-355

**The engine is still a strict loader, a pure core and a runnable shell; the core is pure by grep and nothing enforces it**

P07 I12 · fact · verifier: supported · decides: none

Four crates in one workspace: loader (2,879 lines of source), core (14,109), conformance runner (2,277) and shell (8,561, with 3,993 of tests beside). The dependency direction is right: the core depends on the loader and on no I/O crate; the shell depends on the core.

The core has no file, network, process, environment, thread or clock access. Its standard-library use is collections, a result alias, one PathBuf used for string work, RefCell, and one OnceLock. chrono is used only to parse a supplied timestamp. Time, IDs, revisions and suffixes arrive as Evidence; maps are BTreeMap throughout, so order is deterministic. From the loader the core calls only functions that take bytes or documents.

Two qualifications. The core embeds spec/names.yaml at build time and caches three names in a process-wide static, which CONFORMANCE.md states ('a rename requires rebuilding'). And nothing mechanical keeps it pure: there is no clippy configuration of disallowed methods, no forbid attribute and no CI check, so the property rests on review of each order.

The shell holds no scenario-specific code (no S1, filesystem or history special case) and one tool-specific one: npm is the only registry type its runner can fetch. It does decide two things a caller can see: it answers unfinished operations itself with no record (I10), and its serve command sets up registered instances and restarts stopped runs before listening, which is tooling policy rather than a promise.

**Evidence.** engine/Cargo.toml:5 (members); engine/crates/core/Cargo.toml:10-25 (dependencies: serde, serde_json, conduit-loader, jsonschema, semver, sha2, serde_json_canonicalizer, fluent-uri, percent-encoding, flate2, tar, chrono, url).
grep -nE 'std::(fs|time|net|process|env|io|thread|sync)|SystemTime|Instant|tokio|rand::|getrandom|now\(\)|Utc::|Local::|println!|HashMap' engine/crates/core/src/*.rs returns only lib.rs:29 (OnceLock) and lib.rs:33 (include_bytes of spec/names.yaml); chrono at records.rs:472 and :476 ('parse_from_rfc3339').
engine/crates/core/src/lib.rs:1-3 (module comment), :86-95 (Evidence).
Loader calls from the core: read_bytes, compile, formats::ecma_regex, MAX_DEPTH, read_submitted_files (engine/crates/loader/src/project.rs:84-92, 'Pure'), Specification::from_documents (archives.rs:86). File reads in the loader are at read.rs:102 and project.rs:136-150 only.
find engine -name 'clippy.toml' -o -name 'deny.toml' gives nothing; grep 'forbid' engine/crates/*/src/lib.rs gives nothing relevant.
engine/CONFORMANCE.md:39 ('a rename requires rebuilding'), :140-141 (serve sets up and restarts); engine/crates/shell/src/runner.rs:469 and :720 (npm).
Line totals: find engine/crates/&lt;crate>/src -name '*.rs' | xargs cat | wc -l.

**Waits.** A one-line clippy disallowed-methods list for the core crate would turn the property from reviewed to enforced; coordinator's, small.

**Verifier.** supported. Members and dependency direction: engine/Cargo.toml; engine/crates/core/Cargo.toml:10-25 (no I/O crate; jsonschema with default features off). Line totals reproduce: loader 2,879; core 14,109; conformance 2,277; shell 8,561 with 3,993 under tests/. The analyst's grep returns only lib.rs:29 (OnceLock); chrono only parses at records.rs:472, :476. Loader calls from the core are compile, read_bytes, formats::ecma_regex, MAX_DEPTH; the loader's file reads are read.rs:102 and project.rs:136-150 only. No clippy.toml or deny.toml; no forbid attribute. npm is the one registry type (engine/crates/shell/src/runner.rs:469-472). No docker, ssh, tailscale, keychain or linear in engine/crates/*/src.

**Verifier's counterargument.** The purity conclusion holds. The method has a blind spot the analyst did not notice: the pattern std::(fs|io|...) cannot match grouped imports, and the core has one, 'use std::{collections::..., io::Read}' (engine/crates/core/src/archives.rs:9-12), used for in-memory gzip and tar. It is harmless, but it shows why an enforced lint is worth more than the grep; the suggested disallowed-methods list is the right small order. 'Strict loader' is asserted, not examined: no item looks at the loader's strictness beyond the by-rule test.

**Verifier's corrections.** The core's standard-library use also includes io::Read (archives.rs:11), Cow and OnceCell (access.rs:6). url's to_file_path (requests.rs:1143) is pure but differs by platform, a small determinism caveat for a Windows node. The shell ships one S1-named helper binary (engine/crates/shell/src/bin/s1-archive.rs), a fixture builder, not engine behavior. Shell 'source' of 8,561 lines includes 3,243 lines of tests kept in src (tests.rs, server_tests.rs).


## FR-356

**The largest files, and the code an independent builder would find hardest to compare against**

P07 I13 · fact · verifier: supported · decides: none

Largest files (lines): core registration.rs 2,569; core requests.rs 2,275; shell tests.rs 1,627; shell server_tests.rs 1,616; shell runner.rs 1,570; core packages.rs 1,471; core lifecycle.rs 1,280; core access.rs 1,212; core schemas.rs 1,169; the runner's lib.rs 1,098. The whole engine is 35,245 lines in 64 files.

Hardest to compare, in order:
1. requests.rs invoke_inner, one function of 716 lines: identification, access, records of the decision, preflights, repeats, the two stubs, a trial of replacement, installation and dispatch, threading one mutable JSON route through all of it.
2. registration.rs prepare (554 lines) and resolve_instance (317): resolution, defaults, tombstone matching and the registration decision that cites 74 promises.
3. lifecycle.rs advance (280) and report (196), and retirement.rs: state machines over string phases held inside the invocation ('entry', 'declarations', 'hook-pending', 'inspect-run', 'stop'), with part outcomes assigned in branches and few citations (I10). CONFORMANCE.md needs two prose sections to say what they do.
4. access.rs evaluate_path (359 lines): well cited, but the order of its checks is the meaning (OPS-023), and the same rule exists in a state form, a view form and the differential reference.
5. The state itself. Objects, records and invocations are untyped JSON. The core reads them through 210 distinct string keys and defines 38 types; 40 keys are read at exactly one place. A misspelled key reads as null and compares unequal without error. No document or schema describes the stored object, yet the vectors assert it.

An independent builder working from spec/ would produce typed objects and could not line them up with this engine's state or its vectors without reading this code, which the independence rules forbid.

**Evidence.** wc -l engine/crates/*/src/*.rs engine/crates/*/src/*/*.rs engine/crates/*/tests/*.rs (34,585 for those globs; 35,245 with tests/common and examples by find).
Function lengths by script over 'fn ' positions: requests.rs:715 invoke_inner 716; registration.rs:1625 prepare 554; access.rs:697 evaluate_path 359; lib.rs:326 step_body 354; records.rs:246 execute 320; registration.rs:1052 resolve_instance 317; lifecycle.rs:499 advance 280; lifecycle.rs:862 report 196.
Phases: engine/crates/core/src/lifecycle.rs:511-512, :568; engine/crates/core/src/retirement.rs:197-198, :267, :273.
engine/CONFORMANCE.md:258-271 and :318-325 (prose on retirement parts).
Untyped state: engine/crates/core/src/lib.rs:65-84 ('pub objects: Vec&lt;Value>, pub records: Vec&lt;Value>, ... pub invocations: Vec&lt;Value>'); cat core/src/*.rs | grep -oE '\["[a-zA-Z_.-]+"\]' | sort -u | wc -l gives 210; struct and enum definitions counted by grep give 38.
Copies of the access rule: access.rs:98-142, :276-289, :460-469; differential.rs.

**Waits.** Typed state and splitting invoke_inner are the second half of I5's option C, after the rerun's pin.

**Verifier.** supported. File sizes reproduce exactly (wc -l). Function lengths by my own script: invoke_inner 716 (requests.rs:715), prepare 554 (registration.rs:1625), evaluate_path 358 (access.rs:697), step_body 353 (lib.rs:326), resolve_instance 317 (registration.rs:1052), records::execute 317 (records.rs:246), advance 280 (lifecycle.rs:499). State is Vec&lt;Value> for objects, records and invocations (engine/crates/core/src/lib.rs:65-84). 210 distinct bracketed string keys and 38 struct or enum definitions reproduce.

**Verifier's counterargument.** The ranking is fair. The list omits the function a second engine's adapter would have to reproduce: the runner's run_case, 649 lines, the second longest in the engine (engine/crates/conformance/src/lib.rs:417). That belongs at the top of 'hardest to compare against' because it, not the core, defines what a vector means.

**Verifier's corrections.** 210 is an upper bound on state keys: the count includes keys used to index requests, documents, replies and arguments, and the test modules. I count 60 keys that occur exactly once in that form, not 40; the analyst's method for 40 is not stated.


## FR-357

**New probe: run the specification's by-rule examples through the core (the cheapest falsifier in this area)**

P07 I14 · experiment-change · verifier: partly · decides: coordinator

Register row: new, in ROADMAP.md 'Experiments and probes'; it serves A38 and G4 and belongs in step 3. No engine change is needed to run it.

Claim it tests: the first build's second thesis, 'The language fully determines what a set of documents means', and increment 1's own statement that each file under invalid-by-rule 'passes the schemas, since its rule comes with the core'. Today the only test of those 41 files is that the schemas accept them.

Pass condition, fixed before it runs: each of the 41 files under spec/schemas/examples/invalid-by-rule, submitted by the public route its kind takes (register.v1 for a project document, install.v1 for a package's), is rejected, and the first error diagnostic's ruleId equals the rule in the file's name. Each whole project under spec/schemas/examples/valid registers.

Measures: files rejected under the named rule; files rejected under another rule, listed as pairs; files accepted; valid projects rejected. Each accepted file is a build defect or, if the core cannot decide it from the text, a finding against the promise.

Expected before running: at least the eight files whose rules no engine code names (CTR-009, PLC-028, PRV-001, PRV-004, PRV-016, SEC-001, SEC-022, SEC-024) fail, and SEC-002 has no example at all, which is itself a finding. Cost: one small order, half a day. A failure is unambiguous and needs no judgment, which is what makes it the fastest falsifier here. The passing set then becomes permanent cases under I7's rules.

**Evidence.** docs/implementation/FIRST_BUILD.md:47 ('Every document under `invalid-by-rule/` passes the schemas, since its rule comes with the core'); :34 (thesis 2).
engine/crates/loader/tests/examples.rs:249 (the only test over the directory besides the SCH-016 check at :269).
ls spec/schemas/examples/invalid-by-rule | wc -l gives 41; the eight rule IDs are absent from grep over engine/crates and from conformance/manifest.yaml.
ROADMAP.md:89-103 (how an experiment is proposed: the row it serves, what it must show, how each measure is counted).

**Waits.** Nothing blocks it. Its result sizes the gap in I2 and gives order 046 and later orders their first public-boundary cases.

**Verifier.** partly. Basis holds: docs/implementation/FIRST_BUILD.md:34 (thesis 2) and :47; the only tests over the directory are engine/crates/loader/tests/examples.rs:249 and :269; 41 files, 30 rule IDs; eight IDs absent from engine/crates; SEC-002 has no example. ROADMAP.md:89-103 gives the proposal rules.

**Verifier's counterargument.** Worth running, but the pass condition as written would fail for reasons that say nothing about the engine, and a failure is not 'unambiguous'. (1) 'The first error diagnostic's ruleId equals the rule' contradicts the build's stated DL9 choice: no portable order among diagnostics of one reason (engine/CONFORMANCE.md:28). The condition should be that some error diagnostic carries the rule and the reason is the one OPS-023 gives. (2) The files are single documents by design ('One document, valid against the schemas, that must be rejected', e.g. spec/schemas/examples/invalid-by-rule/SEC-001-secret-reference-beside-a-string.yaml:1), not projects or packages. Each needs a scaffold to reach register.v1 or install.v1, and a rejection under another rule may be the scaffold's doing. That takes judgment per file and more than half a day. (3) 'Each whole project under valid/ registers' fails by construction: valid/ holds sectioned-project and sectioned-package, which need the unbuilt short form, and first-start, which is not a project. These need named exclusions before the run. (4) Its expected failures are mostly families already known to be unbuilt (PRV, SEC), so its information is in the other 22 rule IDs, 20 of which already have cases with their own fixtures. For the larger question it is meant to size (I2), early mutation testing of the core against the vectors (A44) is the stronger falsifier and is already registered.

**Verifier's corrections.** Fix before it runs: 'some error diagnostic names the rule'; a stated scaffold per kind; exclusions for sectioned-* and first-start; a third outcome class 'rejected earlier by the scaffold'. Decider: coordinator, as stated.


## FR-358

**A78 (Conduit runs its own orders): add engine measures and a pass condition on them**

P07 I15 · experiment-change · verifier: partly · decides: coordinator

Register row: A78, ROADMAP.md line 71. Its done-when today is 'Five orders run through Conduit, each friction logged as a finding of the first build'.

Claim it tests: that the engine can be in daily use for real work (ruling 70), which is also the only evidence available for A14's projection of kept state over a week. That projection rests today on one vector's single invocation of 373,365 bytes.

Change. Preconditions: the dogfood engine is re-pinned to main (it is before 070), and the four frictions already seen are entered in increment 4's log as findings, with the unsound fence of I3. Measures, recorded per order and defined before the next run: bytes of the state file at the order's start and end; bytes added per invocation; median and maximum commit time, by the method of the ignored test commit_latency_on_s1; engine restart time; turns that ended unknown for a cause inside the engine or the Agent package, each named. Added pass condition: over the five orders, restart time and median commit time at the fifth are within twice their value at the first; otherwise the store is logged as a finding of the first build and I8 is no longer optional.

Why: the friction list so far is anecdotal, and the one number recorded, 171 to 191 s to restart for each fresh conversation, came from an engine that no longer matches main. Without the measures, five orders can 'pass' while each gets slower.

**Evidence.** ROADMAP.md:71 (A78 row: 'Five orders run through Conduit, each friction logged as a finding of the first build'; 'the next waits for the dogfood engine's re-pin to `main`').
docs/implementation/HANDOVER.md:141-155 (the four frictions; 'None is yet a finding in increment 4's log'; 'still pinned at `38c0b4f`, before 070'; '171 to 191 s').
engine/crates/shell/tests/agents.rs (commit_latency_on_s1 prints 'state_bytes={} objects_bytes={} package_file_bytes={}').
docs/AUDIT.md:476 (A14: 'the state kept for every invocation, measured by the core (D19 of round 6), is projected for the week's workload'); docs/proposals/first-build/README.md:104-110.

**Waits.** Orders 046 and 047, which the handover plans to run through Conduit next.

**Verifier.** partly. A78's row and done-when: ROADMAP.md:71. Frictions, pin and restart figure: docs/implementation/HANDOVER.md:141-155. The ignored test and its printed measures: engine/crates/shell/tests/agents.rs:393-395, :443-447. A14 and D19: docs/AUDIT.md:476; docs/proposals/first-build/README.md:104-110.

**Verifier's counterargument.** Measuring is right; the pass condition is not well formed. (1) A first-to-fifth comparison straddles two breaks the handover already schedules: 'After 050, the store is initialized again' (HANDOVER.md:157-158), and orders 4 and 5 wait for appointment in batch 2 (ROADMAP.md:71), which merges after the bar. The fifth order may be months away and on a fresh store. Measure a slope inside one store instead: bytes added per invocation, and commit time against state bytes, with the condition set on those. (2) The dogfood harness is outside the repository (HANDOVER.md:123), so measures taken there end up as '(log)' facts. The engine should emit them (a line per commit or a command), so that the evidence is reproducible in the tree. (3) It is at odds with I5's option C, which does the store before this measurement exists. Keep I15 and let it decide I8. (4) 'Within twice' is arbitrary; say so and fix it before the run, or report the series with no threshold and let the coordinator judge.

**Verifier's corrections.** Decider is right: the register is not binding and ruling 61 B12 fixes how A78 runs, not what it measures. Entering the four frictions and the fence in increment 4's log is owed regardless (ruling 70).


## FR-359

**Experiment 022 and S10: a provider suite passes only when the engine, not just the harness, is the caller**

P07 I16 · experiment-change · verifier: partly · decides: coordinator

Register row: Experiment 022, ROADMAP.md line 76, and S10 in the scenarios table. Its done-when today: 'The harness passes its own tests, and each suite runs on both of its contract's implementations with a dated report'.

Claim it tests (ruling 40): a provider is existing infrastructure behind a contract that any tool can implement, with no special case in the engine. As built, about 18,000 lines of provider and kit code under packages/ are exercised by a harness that stands in for the engine, while the engine on main cannot call any of them: it answers every provider call Missing. A pass therefore shows that a provider meets the harness. Nothing yet shows that the engine and a provider meet each other.

Change to the pass condition, fixed before the suites' dated runs: each suite passes twice per implementation, once driven by the harness and once with the engine's shell issuing the same operations through its own provider-call path (after order 080), with identical results per case; and a search of engine/crates finds no provider's name and no tool-specific branch. Measures: per suite and implementation, cases passed with each caller; every divergence listed and classed as a gap in the contract, a defect of the engine or a defect of the harness; the count of tool-specific identifiers under engine/crates (today one: npm, in the built-in runner).

Why now: S10's own falsifier is that any document edit needed to swap a provider fails swappability. If engine and providers first meet in S10, integration faults will be found at the most expensive point and will be hard to tell from faults in the claim. This puts the meeting one step earlier at the cost of one driver in the harness.

**Evidence.** ROADMAP.md:76 (Experiment 022 row) and :148 (S10).
Package sizes by find and wc -l: runtime-provider-kit 6,253; provider-ssh-runtime 6,731; provider-local-process 2,125; provider-local-directory 1,108; provider-local-machine 981; provider-file 857.
engine/crates/shell/src/lib.rs:83-85 ('fn provider_call ... Ok(Execution::Missing)') and :671-690; engine/CONFORMANCE.md:41.
docs/implementation/PROVIDERS.md:77 ('ADDR/079 and DSP/080 are order files, not implemented paths') and :81 ('No MCP provider driver or per-tool report ... core cases alone cannot establish P3').
engine/crates/shell/src/runner.rs:469-472 ('unsupported registryType'), :720 (npm_package); grep for docker, ssh, tailscale, keychain, linear in engine/crates/*/src gives nothing.

**Waits.** Order 080's merge; the suites' dated runs under A63; S10's pre-registration.

**Verifier.** partly. Experiment 022's row: ROADMAP.md:76. Package sizes reproduce exactly (6,253; 6,731; 2,125; 1,108; 981; 857, about 18,000). The engine answers provider calls Missing: engine/crates/shell/src/lib.rs:83-85, :667-690; engine/CONFORMANCE.md:41. npm is the only tool-specific identifier in engine/crates. The PROVIDERS.md lines cited are from a status table taken at an earlier commit and partly stale (it says 'No store package exists' and that the tree 'still clones State', docs/implementation/PROVIDERS.md:80, :84).

**Verifier's counterargument.** The gap is real: nothing yet shows the engine and a provider meeting. The remedy as written is not workable and partly not the coordinator's. (1) An engine cannot issue 'the same operations' as a contract suite. The core emits only the intents it decides; a suite's negative cases (malformed input, calls out of order) cannot be produced through it, so 'identical results per case' is unattainable for part of every suite. (2) The harness is standalone by design ('depends on no repository crate', experiments/022-provider-contracts/README.md:3-6); a driver standing in for the caller is the existing paradigm for provider contracts (csi-sanity for CSI drivers, critest for CRI runtimes), with orchestrator-driven tests kept apart. (3) S10's P1 already makes the engine the caller end to end (experiments/021-s10-swapped-providers/README.md:40-41), and S10's pre-registration is owner-accepted (docs/proposals/ruling-75/README.md:79-80), so any change to S10's conditions is the owner's; 022's are the lane's. A cheaper way to move the meeting earlier, in the consumer-driven-contract pattern: validate every provider-call intent the engine emits in its vectors against the contract's input schema; feed the harness's recorded provider outputs to core cases as completions; and add one engine-driven smoke per runtime provider (set up, run, inspect, stop) to order 080's done-when.

**Verifier's corrections.** Keep the measure 'count of tool-specific identifiers under engine/crates' (today one: npm); it is cheap and tests ruling 40 directly. Drop 'each suite passes twice with identical results'. Decider: lane coordinator for 022, admin for 080's done-when, owner only if S10's pre-registration changes.


## What the verifier says the analyst missed

- **v0's gate is already ruled, and the analysis never cites it** IN_FORCE.md:261-269 and the owner-accepted list at docs/proposals/ruling-47/README.md:33-43 say what v0 must pass: S1 to S10 end to end with vectors, S1 and S2 with real Agents, every safety vector, the author measurements, model checking, subscription terms. Metrics are outside the gate (IN_FORCE.md:148) and Fleet authority ships in v0 (IN_FORCE.md:171). The summary's 'which capabilities the v0 engine must hold' is therefore not open. Measured against the ruled gate the engine is further off than 'about half': S4 to S9 also need control dispatch (S5), run-end fences (S4, S6) and package sources (S7), none on main.
- **Mutation testing (A44, G6) is the registered measure of the oracle risk and is not mentioned** ROADMAP.md:128 lists 'Mutation testing of the model against the vectors: whether the vectors actually pin behavior' and G6 gates the independent builds on it (docs/implementation/INDEPENDENT_BUILDS.md:116). Running it on the core now would put a number on I2 (surviving mutants per file and per promise) with no new vector rule. The model-checking lane and tools/check_schemas.py against the loader are the two independent oracles that do exist; the analysis credits neither.
- **Run-end fences are already a proposed order, and the descendant escape is a language gap** S6-END ('shell reports built-in runs stopped on their own exit, after a stop part, and after restart', experiments/017-s6-revoke-in-flight/plan.md:123-132) and S4-LATE (experiments/015-s4-partial-setup/plan.md:154-167) cover what I3 calls a new small order. Separately, OPS-014's event is 'has seen its process end' (spec/conventions/operations.yaml:361-363); nothing in spec/ speaks of a run's descendants, so an engine keeping the promise can fence while an escaped child acts. Under ruling 70's unconfined Agents that is a third limit of the fence beside ruling 75 A1's two, and it is not stated.
- **Merged specification text with no engine work on main** Order 081's one-run-per-instance promises PRV-019, PRV-021 and PRV-030 are named in no engine source and have no case; the same is true of IFC-050 and ACC-056 (refusal tiers and explicit results) and PKG-032. The analyst's table files PRV-019, PRV-021 and PRV-030 under provider calls and reports 'core side yes' on the strength of PRV-007 alone. A list of promises changed by 077, 081 and 082 against what the core names would show how far main's engine lags main's spec.
- **Estimates understate by a large factor, which the distance judgment should carry** 049a1 is half of order 049a (docs/work-orders/049a1-build-enrollment-core.md:27-35) and came in at 3,980 engine lines against 1.2-1.5k for the whole order, after five rounds with a must still open. Applied to batch 1's remaining 8.2-11.4k, and with 052a, 052b, 053, S3-L1, S3-L2, S6-END, SEC-D and the PSD orders unestimated or outside that sum, 'about half' is optimistic. The round counts (049a1 five, 070 four, 042 four plus a fix round) say the same about order size.
- **The vectors' starting state and the runner itself** 338 of 494 cases start from a snapshot of internal state, and the shared one is 'not a state the engine can reach' (conformance/README.md:65-66). The runner's run_case is 649 lines (engine/crates/conformance/src/lib.rs:417). Both bear on I2 and I13 more than the points cited there. The coverage index also disagrees with itself: eight S1 promises are tagged by a case while s1_coverage lists no vector for them, and the promises' own 'vectors' field is empty in 517 of 519.
- **Increments and tasks were assigned and are not assessed** The assignment named docs/implementation/FIRST_BUILD.md's increments 1 to 4 and tasks T1 to T7. No item says where each increment's done-when stands. The relevant facts: increment 4 is open because both author runs failed the bar (FIRST_BUILD.md:100-105); order 053's gate for the rerun is 'no "not built" stub remains' (docs/proposals/spec-batch-1/README.md:659-663), which ties the two stubs in I10 to the critical path; and finding 4-18's log entry still says four registration copies where the code has six (docs/proposals/first-build/log-increment-4.md:29).
- **The parked branches are parked by a logged decision** s3/m2 'merges after the rerun's pin' (docs/implementation/HANDOVER.md row 11 and :277) so that S1's rerun build does not move. I9 recommends merging or dropping the branches before any new engine order without naming that decision or what reversing it costs the rerun.

## The verifier's notes

Method: every count was recomputed at 2eb8f258 in the repository root with read-only commands (grep, wc, git diff --shortstat, git blame, PyYAML under /usr/bin/python3 over the manifest and all 494 case files, and small scripts for decision sites, function lengths and unwraps). Nothing was built or run; I did not confirm that 494 cases pass, and I did not read the contents of the four unmerged branches beyond diffstats and file lists. Not verified: the '40 keys read once' and '51 outcome literals' counts, and the cause of the 171-191 s restart.

Overall: the analyst's numbers are reliable. Of roughly sixty figures I recounted, the material errors are four: 342 named promises (317 in Rust source), 'the last two core merges had no CI engine run' (one), '9 to 13' promises on retirement's decision (6 or 9), and 'the one later reader' of held context (reconciliation also reads it). The fact items I1, I4, I11, I12 and I13 stand with small corrections.

The judgments are weaker than the facts in four ways. First, neither owner question should go to the owner as written: I6 is answered by rulings 47, 61 B1 and 72, which the item does not cite, and its recommended option would reverse two of them while claiming to need no ruling; I5 is a build-order decision the coordinator has made before without a ruling (068, 070, 071). That leaves this area with no question for the owner except, possibly, a dated later answer under ruling 75 stating a third limit of the fence. Second, the recommendations pull against each other: I5 does the store now while I15 makes the store conditional on a measurement not yet taken; I7's second rule adds an internal field while its first rule bans internal assertions. Third, three items present known and planned things as new: the fence limit (stated in CONFORMANCE.md:41, planned in S6-END), the stubs (decision C11), and vector weight (the README's lean-vectors section). Fourth, the headline risk is overstated: the evidence can show a promise unmet where a case asserts a reason, a ruleId or an exact array; what it cannot show is unrequired behavior, a wrong deciding promise for an outcome, or anything about 227 promises, and it cannot be run by a second engine.

What I would keep for the review's record: the capability table (with 049a2 added and the gate taken from IN_FORCE.md:261-269); the six-copy and held-context facts; the vector-shape finding, strengthened by the snapshot count; CI independence for the engine shards and a nightly differential run; the purity lint; a corrected I14; I15 with a slope measure emitted by the engine. What I would change: decide the vector rule as coordinator and express it through IFC-050's public ruleId; stage the store work behind the measurement and confine its first step to the shell's serialization; fold the fence fix into S6-END and log the descendant escape as a language finding; and bring A44 forward as the falsifier for the oracle risk.
