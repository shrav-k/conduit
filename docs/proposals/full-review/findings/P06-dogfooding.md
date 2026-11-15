# P06-dogfooding: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 2 and 3, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** Dogfooding stage 1 did what ruling 70 asked of it: it tested how a turn is carried, and it found real faults (4-20 and the six frictions). It did not test what Conduit is for. In order 042 the only thing Conduit decided was that root may take turns on two Agents; every limit on the worker and the verifier was a harness check made after the fact, and the run was less confined, less diagnosable and less strictly reported than the plain `codex exec` runner the project already had. The frictions landed where the layered claim says behavior belongs: five of six are faults of the Agent Type's contract, the runner or the harness, and none needs a new language concept. The exception matters: the fence that ruling 75 A1 and order 077 added (OPS-014) is established by evidence that the smoke test showed false, because a turn's processes outlive both the wrapper's answer and the run's process-group stop. "Root sends turns to Agents" fits a work-order pipeline once three things move into the `.v2` turn contract before 0.2.0 publishes (a conversation the caller chooses, a report in the client's own schema option, a structured failure) and the wrapper owns its client's process tree; the queue and the rounds rightly stay outside, driven by an Interface. The order of risk is wrong: the worker already reaches everything a dedicated macOS user is meant to protect, so that user should come before the next Conduit-run order and the confining runtime provider before any coordinator Agent. Ruling 61 B12's per-order tree grant no longer has anything to grant since ruling 75 B1 made the folder a fixed input, so orders 4 and 5 need redefining: I recommend shared things behind Conduits, order 4 by an appointed Interface and order 5 by an Agent. A turn of hours as one blocking call tied to the caller's connection will not compose into an Agent driving Agents; a probe should settle that before order 5. A78 has no experiment directory, no pre-registered measures and none of its frictions logged; its evidence is in private notes.

**What the analyst read.** Tree: commit 2eb8f258 (2026-11-12), read-only; nothing built or run.

Read in full: docs/guides/full-review.md; docs/implementation/HANDOVER.md; AGENTS.md "What holds" and "Where things go"; the records of rulings 61, 70, 74 and 75 and ruling 72's later answers; docs/proposals/first-build/log-increment-4.md (20 rows); order 042's Record; order 076; packages/agent-codex and packages/agent-claude-code at the pinned commit (README, contracts.yaml, conduit.yaml); spec/conventions/implicit.yaml; the promises IFC-009, IFC-010, IFC-016, OPS-007, OPS-012, OPS-013, OPS-014, OPS-024, OPS-027, ACC-019, CTR-014, NAM-024, PRV-006 (part), PRV-008, PRV-009, PRV-010, PRV-021, PRV-029; spec/implementation-defined.yaml DL1 to DL19 (heads); docs/AUDIT.md "Purpose", "Preservation tests", "Layered claim", "Waiting for the owner", "Known to be open"; ROADMAP.md's order of the work, register header, rows A5, A6, A77, A78, A81, A82 and "Proposing an experiment".

Read in part: order 050a (decisions, step 7, what the admin checked, scope; not all 495 lines); orders 074, 077 and 078 (purpose and Record); docs/implementation/WORK_ORDERS.md (dispatch, checking, "Running the orders through Conduit"); docs/proposals/projects/brief.md (A78's flow only); engine/CONFORMANCE.md rows DL3, DL10, DL16; engine/crates/shell/src/runner.rs (process group, Drop, dispatch timeout); engine/crates/core/src/requests.rs (fence); packages/provider-local-process/README.md (stop).

Read outside the pinned tree, by read-only `git show`: the unmerged `.v2` Agent packages on origin/wo/076-agents-v2-folder-grant (228b5dea) and origin/wo/078-agents-codex-path-item (5f97ca38): contracts.yaml, conduit.yaml, README and parts of src/lib.rs. The dogfood ran these, and the pinned tree holds only `.v1` (0.1.1).

Not read, because they are outside the repository: the dogfood harness, the conduit-core Fleet's documents and store, the private handoff note and coordinator's log, and any trace of 042's turns or of the smoke test. Every fact marked "(log)" rests on HANDOVER.md's account alone. Not read: the engine's source beyond the files named; the scenario plans; rulings other than those named; IN_FORCE.md beyond the lines grepped.

Not verified: Codex's and Claude Code's own retry settings and structured-output options against the pinned clients (stated from general knowledge, marked in the items); why the 042 turn failed where plain workers went on; whether verifier-1 named a directory (inferred from the guard); whether batch 2's Interface tokens exist in any form today.


## FR-331

**Stage 1 tested how a turn is carried, not what Conduit is for, and was weaker than the plain runner on four counts**

P06 I1 · assessment · verifier: supported · decides: none

From the repository's account, the only thing Conduit decided in order 042 was that root may take turns on two Agents. The worker and the verifier used their clients' own tools in a directory, and CTR-014 says of that reach that no Conduit grants it, no restriction limits it and no record shows its use. Every limit of the pipeline was a harness check made after the turn: HEAD unmoved, the verifier changed nothing, nothing pushed, the main checkout untouched.

Against the plain runner the project already had, the Conduit run gave up four things:
1. Confinement. Plain: `codex exec -s workspace-write`, writing in the tree and cargo's cache, with no network unless the order needs it. Conduit: `--sandbox danger-full-access`, as the owner's user.
2. A read-only verifier. Plain: a read-only sandbox. Conduit: full access, checked afterwards by a guard (inferred: the guard would not exist otherwise).
3. The report. Plain: `--output-schema`, enforced by the client. Conduit: text that the harness parses and validates, with one repair turn.
4. Diagnosis. Plain: `--json` keeps every event. Conduit: the wrapper keeps no event and discards the client's stderr, and the Type keeps inputs and outputs out of the records.

What it gained: one attributable record per turn, and real findings about turns (4-20 and the six frictions).

This was a ruled first step (ruling 70: permissive, to test the communication framework) and it did that job. The error to avoid is counting such an order as evidence for the purpose (what an agent may reach, under which limits, and who answers for it). While Agents are unconfined under the engine's user, a Conduit on anything local limits nothing: the Agent reaches the thing, the engine's store and root's token directly. The passage in WORK_ORDERS.md that describes each tree as a directory Resource behind a Conduit, "S1 in daily use", describes a design that did not run.

**Evidence.** docs/implementation/HANDOVER.md:121-126 ("A harness outside the repository ... validates each report against the order's schema (one repair turn), and checks guards after each turn: the slot's HEAD unmoved, the verifier changed nothing, nothing pushed, the main checkout untouched"). docs/implementation/WORK_ORDERS.md:52-56 (dispatch command), :65-68 ("`workspace-write` lets Codex write in its tree and nowhere else, with no network"), :71 ("`--json` keeps every event"), :77 ("a second, fresh Codex session in a read-only sandbox"), :146-152 ("Each order's working tree is a directory Resource ... That is S1 in daily use"). spec/promises/contracts.yaml:346-350 (CTR-014: "no Conduit grants it, no restriction limits it, and no record shows its use ... On the engine's own runner nothing confines it"). spec/README.md:423-425 (a process under the same OS user reads root's token and every state directory). Branch, not the pinned tree: `git show origin/wo/076-agents-v2-folder-grant:packages/agent-codex/src/lib.rs` line 224 ("danger-full-access"), 464 (`.stderr(Stdio::null())`); same ref, contracts.yaml:84-89 ("They reach every file, process, network address, credential and signed-in account that user can reach: the engine's store, root's client token"). packages/agent-codex/contracts.yaml:25-27 (`records: inputs: false, outputs: false`); packages/agent-codex/README.md:43 ("The wrapper retains no prompt, raw event stream or stderr log"). docs/AUDIT.md:253-257 (purpose).

**Waits.** Frames I3, I4, I6 and I7: what the next orders must measure, and why confinement comes before delegation.

**Verifier.** supported. Recounted at 2eb8f258. Plain runner: docs/implementation/WORK_ORDERS.md:50-55 (`-s workspace-write`, `--output-schema`, `--json`), :65, :71, :77; docs/implementation/HANDOVER.md:57-63. Conduit run: HANDOVER.md:121-126 (two `codex.v2` Agents, harness validates the report with one repair turn, four guards after each turn). Branch wrapper, `git show origin/wo/076-agents-v2-folder-grant:packages/agent-codex/src/lib.rs`: lines 221-225 give `danger-full-access` when `.v2` names a directory, 464 sends stderr to null, the loop carries the comment "Never persist raw client streams", and `arguments()` holds no `--output-schema`. Same branch, contracts.yaml:115 `records: {inputs: false, outputs: false}`. spec/promises/contracts.yaml:348-352 (CTR-014), spec/README.md:423-426, WORK_ORDERS.md:144-148 (the directory-Resource design).

**Verifier's counterargument.** Ruling 70 chose this on purpose ("overly permissive in order to test conduit"), and the purpose itself makes enforcement "a future, optional part" with restrictions applying by default only to what passes through Conduit (docs/AUDIT.md:264-266). So the four losses are not a breach; the valid point is narrower: 042 passed nothing through Conduit except root's turns, so it is no evidence about access. That part cannot be fully checked: the conduit-core Fleet's documents are not in the tree, so which Conduits the two Agents held is known only from HANDOVER.md's prose.

**Verifier's corrections.** Line drift only: the design passage is WORK_ORDERS.md:144-148 (not 146-152); CTR-014's observable is contracts.yaml:348-352. Plain `net` orders also have the network (HANDOVER.md:57-58), so the network difference holds only for orders that are not `net`.


## FR-332

**The fence of OPS-014 is recorded on evidence the smoke test showed false: a turn's processes outlive the answer and the stop (draft row 4-23)**

P06 I2 · assessment · verifier: partly · decides: coordinator

Class: defect of the build, and a gap in `spec/`. Unsafe by the tracker's test (an effect can overlap its own retry), within reach the language already calls unmediated.

Three places report an end that has not happened:
1. The wrapper ends a failed, cancelled or timed-out turn by killing the client's process alone. The client's children are not signalled.
2. Its error answer fences the attempt: OPS-014 says an answer that ends the call fences it, and the core does so.
3. The engine's runner sends SIGKILL to the run's process group and records `builtin-run-stopped` at once; the core then marks every attempt sent to that run fenced. CONFORMANCE.md's DL16 row admits it "must await an actual end before this evidence is sound". In the smoke test Codex's terminal children, each in its own session, outlived that kill. This is the ordinary behavior of the first real consumer, not a hostile case.

So a turn can be recorded failed by a reconciliation, and a repeat dispatched under OPS-024, while processes of the first turn still write in the directory. OPS-024's assurance ("the new attempt cannot meet an earlier dispatch still on its way") does not cover them. The same overlap arises without any reconcile: the next turn, on any key, runs beside the leftovers. The dogfood engine's pin predates 077, so this fence code was not in the engine that ran 042; it will be after the re-pin.

Fault of: the Agent package (it does not own its client's process tree), the engine's runner (it reports before it knows), and `spec/` (OPS-014 names two limits of the fence, from ruling 75 A1, and not this one; no promise says how far a run extends).

Least fix, on existing paradigms:
- Wrapper: start the client as leader of a process group; stop the group when a turn fails, is cancelled or passes its deadline, and when the run stops; find what left the group by a marker variable that every descendant inherits, as Jenkins' process-tree killer does; name in the result what could not be stopped.
- Runner and `provider-local-process`: record the stop only after the group is seen empty; otherwise the state is unknown, as PRV-008 already says for a provider that cannot tell.
- `spec/`: OPS-014 states the third limit beside A1's two: the fence covers Conduit's dispatch, not a process an Agent's own tools started.
- Durable: one OS user per slot and kill by user, as Nix's pool of build users does; or cgroups on Linux, as systemd's `KillMode=control-group` does. That comes with ruling 75 B2.

Draft row:
| 4-23 | defect of the build; gap | PRV-008, OPS-014, OPS-024, DL16 | In the smoke test (log), Codex's terminal children, each in a session of its own, outlived the SIGKILL of the run's process group. Three places then report an end that has not happened. The engine's runner records `builtin-run-stopped` as soon as it has signalled, without seeing the process end (CONFORMANCE.md, DL16, says so), and the core marks every attempt sent to that run fenced. The `.v2` wrapper ends a failed, cancelled or timed-out turn by killing the client's process alone, and its answer fences the attempt under OPS-014. So a turn can be recorded failed, and a repeat dispatched under OPS-024, while processes the first turn started still write in the directory. `provider-local-process` states the same limit for `setsid`. | The harness lists and stops what a turn leaves behind (log). | Proposed, for the coordinator: the wrapper starts its client as the leader of a process group, stops the group when a turn fails, is cancelled or passes its deadline and when the run stops, finds what left the group by a marker variable that every descendant inherits, as Jenkins' process-tree killer does, and names in its result what it could not stop; the runner records the stop only after it has seen the group empty, and otherwise reports the run's state unknown (PRV-008). In `spec/`, OPS-014 states the limit beside ruling 75 A1's two: the fence covers Conduit's dispatch, not a process that an Agent's own tools started. That sentence changes what the language says, so it goes to the review list. |

**Evidence.** spec/conventions/operations.yaml:352-354 ("An answer that ends an attempt's call at the implementation fences it: a result, an error or a JSON-RPC error"), :361-363 ("An attempt sent to a run is fenced once the engine's own runner has seen its process end, or the runtime provider reports it `stopped`"), :524-526 (OPS-024). engine/crates/shell/src/runner.rs:46-50 (`libc::kill(-self.pid, libc::SIGKILL)`), :208-219 (Drop: `run.group.stop()` then submit `builtin-run-stopped`; `grep -n builtin-run-stopped` finds this one site). engine/crates/core/src/requests.rs:60-72 (an answering result ends the call), :75-90 (`mark["fenced"] = json!(true)`, decision `fence-stopped-builtin-run`, OPS-014). engine/CONFORMANCE.md:41 ("immediately after signalling SIGKILL rather than observing process exit; it must await an actual end before this evidence is sound"). packages/provider-local-process/README.md:90-91 ("Stop reaches the process group only. A descendant that calls `setsid` escapes it, as with the engine's own runner"). docs/implementation/HANDOVER.md:149-150 (PTY children escaped; the harness sweeps). Branch: `git show origin/wo/076-agents-v2-folder-grant:packages/agent-codex/src/lib.rs` lines 461 (`kill_on_drop`), 639-644 (`shutdown`: `s.child.kill()` then `wait`, no group). spec/promises/providers.yaml:287-288, 294-295 (PRV-008). Pin: `git merge-base --is-ancestor 575a1bcd 38c0b4f` fails (077 not in the dogfood engine).

**Waits.** The re-pin of the dogfood engine to `main` brings this fence into the engine that runs orders; fix the wrapper and the runner first, or state the limit. The OPS-014 sentence goes on the review list for the owner to strike.

**Verifier.** partly. Holds: the wrapper kills only its client (branch lib.rs:461 `kill_on_drop`, 639-644 `s.child.kill()`; no process group or `setsid` anywhere in the file). The core fences on an answering completion (engine/crates/core/src/requests.rs:61-66, 1528-1533) and marks every built-in attempt fenced on `builtin-run-stopped` (:75-95). The runner sends SIGKILL to the group and submits that observation in `Drop` (engine/crates/shell/src/runner.rs:46-50, 208-219; the only submit site). engine/CONFORMANCE.md:41 says so. OPS-014's text is spec/conventions/operations.yaml:349-363. `git merge-base --is-ancestor 575a1bcd 38c0b4f` exits 1.

**Verifier's counterargument.** 1. The runner half is narrower and already known. CONFORMANCE.md:41: "The shell submits that observation only on Runner drop", that is, at engine shutdown; it is a declared conformance gap, and OPS-014 itself is right ("has seen its process end"). 2. "Showed false" is an inference: the smoke test ran on a pin without 077's fence; it showed children escaping the kill, not a fence recorded wrongly. 3. The proposed runner fix does not close the hole shown. A child that called `setsid` has left the group, so "group seen empty" is true while it lives, and on the stop path the wrapper dies with the group and cannot sweep. The less inventive step is what packages/provider-local-process/README.md:84-88 already does: TERM, a grace period, then KILL, confirm, else report unknown; the wrapper stops its client's tree on TERM. 4. A marker-variable scan reads the environment of every process of the user, against the standing rule (HANDOVER.md:282-284). 5. The spec gap is thin: OPS-014 already says "the fence covers Conduit's own dispatch, and each hop handles its own idempotency" (operations.yaml:351-352) and CTR-014 says own-tools reach is unrecorded. OPS-014's observable is about 70 lines; state the limit in the Type's contract instead. 6. Today the poison rule (lib.rs:495-497) stops a repeat from starting work in the same run, so overlap needs a new run; I8's un-poisoning must not land before tree ownership.

**Verifier's corrections.** Say "only on Runner drop" in the row. "Each in a session of its own" is inferred from "terminal (PTY) children" (HANDOVER.md:149). For a cancelled turn OPS-014 says cancellation without confirmation fences nothing (operations.yaml:359-361); whether the wrapper's late error then counts is untested. The log's classes are single; pick "defect of the build".


## FR-333

**Order of risk: the dedicated macOS user now, and the confining provider before any coordinator Agent?**

P06 I3 · question-for-owner · verifier: partly · decides: owner

On 2026-11-12 you kept the dedicated macOS user where ruling 75 B3 puts it, before a coordinator Agent drives workers, with `main`'s branch protection as the guard until then. I ask again because of four things not in front of you then.

1. The risk does not step up when an Agent starts driving. A stage 1 worker already reaches everything the dedicated user is meant to protect: your home, the keychain and GitHub login, SSH keys and so the Windows computer, the other lanes' worktrees, the private handoff note, the engine's store and root's token. Who writes the prompt changes little.
2. Branch protection is not a guard against that. Ruling 74 says so itself: it "does not stop an Agent that uses the owner's `gh` login from opening and merging a pull request".
3. The harness's guards run git inside a repository whose `.git` the checked Agent can write (configuration and hooks), so their evidence is under the Agent's control.
4. The plain runner confines the same work more than Conduit does today (I1).

What the smoke test showed, read with order 050a: Claude Code in `bypassPermissions` asks for no approval before writing inside `.git`. It shows a missing guard, not an Agent acting on its own initiative. The exposure is the same either way.

What a dedicated user buys: your account, credentials, checkouts and second machine are out of reach, and with no GitHub credential in that user the "nothing pushed" guard is true by construction. What it does not buy: Agents on the engine's own runner still run as the engine's user, so they still read the store, root's token and each other's sessions. Only ruling 75 B2's confining runtime provider closes that.

Options:
A. Keep the order as answered: unconfined as your user until a coordinator Agent drives.
B. The dedicated macOS user before the next order run through Conduit (046), holding no GitHub credential and no SSH key; B2's confining provider before any coordinator Agent.
C. No further Conduit-run orders until the confining provider exists.

Recommendation: B. Cost: one setup session of yours, as for `conduit-run` under ruling 77 part 1 (a user, its tools, one Codex sign-in with the primary account); the coordinator reaches it as it reaches `conduit-run`. C costs the dogfood weeks for little more safety than B. A leaves the largest exposure in the project open for the longest time.

It changes: a dated later answer in ruling 75's record (B3's words already allow it earlier), IN_FORCE.md topic 15 (the line citing 75 B3), and the handover's next steps. Nothing in `spec/`.

**Evidence.** docs/proposals/ruling-75/README.md:47-48 (B3), :125-128 (later answer: "The dedicated macOS user stays where B3 puts it ... Until then, `main`'s branch protection ... is the guard"). docs/proposals/ruling-74/README.md:26-30 ("It does not stop an Agent that uses the owner's `gh` login from opening and merging a pull request"). docs/work-orders/050a-agents-v2-own-tools.md:103-105 ("Not checked yet: ... whether bypassPermissions still prompts for a write inside `.git`"). docs/implementation/HANDOVER.md:147. Branch README, `git show origin/wo/076-agents-v2-folder-grant:packages/agent-codex/README.md` lines 142-149 (reach: store, root's token, the keychain, other worktrees sharing one `.git`). spec/README.md:423-425. IN_FORCE.md:314. docs/implementation/HANDOVER.md:174-183 (ruling 77 part 1 as precedent).

**Waits.** Orders 046 and 047 through Conduit wait on it if B is chosen; the lane's order for the confining provider follows it.

**Verifier.** partly. Holds: the later answer of 2026-11-12 (docs/proposals/ruling-75/README.md:125-128); ruling 74's words that protection "does not stop an Agent that uses the owner's `gh` login" (docs/proposals/ruling-74/README.md:28-29); the reach list (branch README:142-149, spec/README.md:423-426); that a dedicated user leaves the store and root's token readable. The owner decides, and the item says which answer it would change.

**Verifier's counterargument.** 1. "Not in front of you then" fails for points 1 and 2. Ruling 74's record put both to the owner on 2026-11-11 (lines 8-9: "it can push with the owner's credentials"; decision 1), and ruling 70 is the owner's own choice of permissive. Point 4 follows from ruling 70. Only point 3 is new, and it cannot be checked: the harness is not in the tree. The brief says to raise a known item again only with a new situation; this re-asks, after two days, with arguments and no new event. 2. B's cost is understated. Slots under another user are unreadable to the coordinator's harness, so the harness must change first; A22 (the keychain with no person present) is open and bears on any Claude Agent there; and order 050 re-initializes the store and has the owner sign each slot in again (HANDOVER.md:157-158), so "before 046" costs two owner sessions. Better timing: move to the dedicated user at 050's re-initialization. 3. B2's provider cannot run an Agent on `main` yet: "Runtime provider calls and remote connections are unserved" (engine/CONFORMANCE.md:41), and order 080 is checkpointed for `s3/m2`, merged after the rerun's pin. 4. A cheaper step already waits: ruling 77 part 2 takes the coordinator's key out of the owner's sudo-capable Windows account (HANDOVER.md:180-183), cutting the worst path with no new ruling. 5. An option not listed: dogfood under the existing `conduit-run` user; it would amend ruling 76 point 6 (A78 stays on the Mac).

**Verifier's corrections.** Reword the opening: points 1 and 2 were stated in ruling 74. Give B's real cost and offer "at order 050" as its timing.


## FR-334

**What should A78's orders 4 and 5 test, now that the folder is a fixed input and not a grant?**

P06 I4 · question-for-owner · verifier: partly · decides: owner

Two rulings no longer fit together. Ruling 61 B12 (2026-11-10) has a coordinator Agent "granting each order's tree through its Fleet's Conduits". Rulings 70 and 75 B1 (2026-11-11) then made the tree a directory the Agent's template names, fixed for the Agent's life (CTR-014), reached by the Agent's own tools outside any Conduit. So a coordinator Agent has no tree to grant per order, and a grant it did make would limit nothing while workers are unconfined.

A second problem is separate. B12 has the coordinator Agent drive workers "through turns". A worker's turn is one blocking call of up to four hours (I5), so the coordinator's own turn must stay open for the whole pipeline, and if its client fails the worker's turn is cancelled with it.

Options:
A. Keep B12's words: build trees as directory Resources, and a shell as a Resource (ruling 70's option b), for the dogfood workers. Tests the access model fully; costs a new provider and slower Agents, and reverses ruling 70's choice of (a).
B. Drop the grant: the coordinator Agent only takes turns on workers. Cheapest; dogfooding then never tests access.
C. Keep the folder private to the Agent (its own tools, confined by ruling 75 B2's provider) and put what is shared behind Conduits: the queue of orders and reports, `main` read-only, and the pull request to GitHub with the token held by the engine. Order 4 is dispatched by an appointed Interface with its own token (ruling 61 B4 allows an Interface to act for a Fleet); order 5 by a coordinator Agent that opens and closes those Conduits per order.

Recommendation: C. It is the purpose's own line ("Private and shared stay separate ... anything shared goes through a Resource"). It puts grants where a Conduit can actually limit something. And it separates two tests that B12 runs at once: bounded authority through a Fleet (order 4, with a program as the actor) and an Agent as driver (order 5, after the probe in I7).

It changes: ruling 61 B12, so it needs a ruling (batch it with I3); IN_FORCE.md topic 15; A78's row in ROADMAP.md; batch 2's content for A78. Nothing changes before the S1 rerun.

**Evidence.** docs/proposals/ruling-61/README.md:28 (B12), :20 (B4: "Any Interface root appoints, each with its own token"). docs/proposals/ruling-75/README.md:39-46 (B1, B2). spec/promises/contracts.yaml:340 ("The place is fixed for the Agent's life"), :346-348. IN_FORCE.md:314 ("The coordinator Agent grants each order's tree through its Fleet's Conduits and drives workers through turns root opened"). ROADMAP.md:71 (A78's row). docs/proposals/projects/brief.md:611-624 (the flow B12 was written for: `trees.make.v1` per order, the worker working through `tree.*`). docs/AUDIT.md:283-284 ("Private and shared stay separate"). docs/implementation/WORK_ORDERS.md:152-154 (the pull request behind a Conduit to GitHub, planned with S2). spec/conventions/operations.yaml:616-618 (OPS-027).

**Waits.** Batch 2's scope for A78 (appointment) and the definition of orders 4 and 5; not urgent, since batch 2 follows the S1 rerun and the bar.

**Verifier.** partly. Holds: B12's words (docs/proposals/ruling-61/README.md:28); ruling 75 B1 (docs/proposals/ruling-75/README.md:39-42); "The place is fixed for the Agent's life" (spec/promises/contracts.yaml:342); IN_FORCE.md:314 still carries both sentences; neither ruling 70 nor 75 lists ruling 61 under what it amends; the flow B12 was written for has the worker work through `tree.*` (docs/proposals/projects/brief.md:611-624). The tension is real, and it is the owner's.

**Verifier's counterargument.** 1. Option C has no stated cost and is empty without confinement. The pull request behind a Conduit needs a provider that holds GitHub's token, planned with S2 (WORK_ORDERS.md:149-150), and ruling 75 C1 has such a package run "where no unconfined Agent runs" (ruling-75/README.md:52-54). With unconfined workers on the engine's machine, the Conduits to the queue and to `main` limit nothing either, by I1's own argument. So C tests access only after I3's answer, and the item should say so. 2. Moving order 4 to an Interface halves the evidence for "Agents can operate the system" (preservation test 5), which B12 exists to give, and an Interface with a token is close to what the harness already is. 3. Nothing waits on this before batch 2. It could be a paragraph in batch 2's plan with the owner's question asked there; if asked now, batch it with I3 as one ruling.

**Verifier's corrections.** State C's dependencies (a GitHub provider, client tokens for Interfaces other than root, which the batch 1 plan marks optional and off the rerun path at docs/proposals/spec-batch-1/README.md:639, and confinement). The pull-request passage is WORK_ORDERS.md:149-150.


## FR-335

**A turn of hours as one blocking call is fragile now and will not compose into an Agent driving Agents (draft row 4-26)**

P06 I5 · assessment · verifier: partly · decides: coordinator

Order 042's first worker turn ran 11966 s as a single call. What holds it up:
- `spec/` names no deadline at all. DL3 says "No promise limits how long an invocation is in progress". That is right for the language; the limits are the Type contract's and the engine's.
- The Type's `turnDeadlineSeconds` is fixed per Agent (4 hours for the worker), so a one-minute repair turn that hangs also waits four hours.
- The engine's call timeout is one value for every call and must be set longer than the longest turn, so a hung provider call also waits hours.
- When the engine's timeout fires it reports the call missing and does not cancel it. The handover already plans that fix.
- The caller's stream must stay open throughout. OPS-027: a dropped stream cancels the invocation; the wrapper then ends the conversation.
- Nothing shows progress, and a failure in the last minute leaves hours of work behind one unknown outcome.

Fault of: the Agent Type's contract and the engine's conformance choices, not the language. The existing paradigm for work that outlasts its call is already in the specification as an option: the protocol's tasks (IFC-016), which is also how A2A serves long agent work (send, receive a task, poll or resubscribe). A per-call deadline set by the caller, capped by the server, is gRPC's.

For stage 2 this is the main lesson. A coordinator Agent that takes a blocking four-hour turn on a worker must hold its own turn open for the whole pipeline; if its client dies, OPS-027 cancels the worker's turn and the package ends that conversation. Convention 7 also means no event can wake a coordinator Agent later: an actor must ask. So the choice is between nested blocking turns, turns served as tasks, or a program (an Interface) as the loop. That should be settled by a probe (I7), not by raising deadlines again, which is how 4-12 was closed.

Draft row:
| 4-26 | gap | DL3, DL10, OPS-007, OPS-027, IFC-016; `turnDeadlineSeconds` | Order 042's first worker turn ran 11966 s as one call (log), under a 4-hour `turnDeadlineSeconds` fixed for the Agent's life and an engine call timeout that is one value for every call and must be set longer. `spec/` names no deadline (DL3). For the whole turn the caller's stream must stay open: OPS-027 cancels the invocation when it drops, and the wrapper then ends the conversation. When the engine's own timeout fires it reports the call missing and does not cancel it. Nothing shows progress meanwhile, and a failure in the last minute leaves hours of work behind an unknown outcome. This reopens part of 4-12, which closed on the deadline input. | The deadlines were raised: 4 hours for the worker, 1 hour for the verifier. | Proposed, for the coordinator: the engine cancels a call whose deadline fires (planned); a turn may state a shorter deadline of its own, as a gRPC call does. Before A78's order 5, a probe decides whether a turn of hours is served as an invocation that outlasts its call, by the protocol's tasks (IFC-016), before an Agent holds such a turn open on another. |

**Evidence.** `grep -rn -i deadline spec/` prints nothing; `grep -rli conversation spec/` finds 0 files. spec/implementation-defined.yaml:25-31 (DL3). spec/conventions/operations.yaml:616-618 (OPS-027: "A connection over Streamable HTTP that the engine sees drop while the request is in progress closes its stream, and so cancels the invocation"). spec/interface/implementation.yaml:386-399 (IFC-016, level must with "may be used"). engine/crates/shell/src/runner.rs:1022-1043 (one `call_timeout` for every dispatch; on timeout the completion is `missing`). engine/CONFORMANCE.md:35 ("dispatch timeout reports missing"). docs/implementation/HANDOVER.md:122 (4-hour and 1-hour deadlines), :138 (11966 s), :155-157 (planned: cancel the provider call when a dispatch deadline fires). docs/proposals/first-build/log-increment-4.md:23 (4-12, closed on `turnDeadlineSeconds`). packages/agent-codex/README.md:46-47 ("Active cancellation ends the client and poisons the conversation"). docs/work-orders/050a-agents-v2-own-tools.md:93-95 (Claude Code aborts an MCP call after 300 seconds without a `timeout` entry).

**Waits.** A78's order 5 (a coordinator Agent) waits on the probe; the engine's cancel-on-deadline fix is already queued before the rerun.

**Verifier.** partly. Holds: 11966 s (HANDOVER.md:138); one `call_timeout` for every dispatch, with `missing` and no cancel on timeout (engine/crates/shell/src/runner.rs:1022-1045; engine/CONFORMANCE.md:35); the Type's description requires the engine's timeout to be longer (branch contracts.yaml:97-99); OPS-027 (spec/conventions/operations.yaml:616-618); the wrapper cancels its session on cancellation (branch main.rs:35-47); DL3 (spec/implementation-defined.yaml:25-31); IFC-016 (spec/interface/implementation.yaml:386-395); `grep -rn -i deadline spec/` prints nothing.

**Verifier's counterargument.** 1. "Will not compose" is a prediction. A coordinator's client can hold a long call: the `.v2` wrapper sets the client's tool timeout to the turn deadline (review-list item 23). Fragile is shown; impossible is not. 2. Tasks are not available to choose. No `tasks/` code exists under engine/crates and CONFORMANCE.md is silent on them; IFC-016 is "an official extension outside the baseline revision", used only where the caller's client declares it, and whether Codex or Claude Code do is unknown. So the probe carries an engine order. 3. A cheaper existing paradigm is missing: MCP's baseline progress notifications. The engine sends none (docs/work-orders/050a-agents-v2-own-tools.md:95) and neither spec/ nor CONFORMANCE.md mentions them. Relayed progress gives liveness and resets idle timers (finding 4-20) with no new machinery. 4. Finding 4-12 closed "with the owner's agreement" and review-list items 21 and 23 were confirmed by the owner; reopening is not only the coordinator's.

**Verifier's corrections.** spec/ does name the limit, as implementation-defined: DL10, "the time allowed for a call" (spec/implementation-defined.yaml:83-84). Add to the row that the deadline is now set in three places (Type input, client tool timeout, engine call timeout and session idle limit).


## FR-336

**What to build next for dogfooding, and what to stop**

P06 I6 · recommendation · verifier: partly · decides: coordinator

Build, in this order:
1. A docs pass: log rows 4-21 to 4-26 (I2, I5, I8 to I11); give A78 an experiment directory with its measures fixed before the next order (I7); bring the conduit-core documents and the harness into the repository, with no private detail.
2. One package order on the `.v2` branch, before 0.2.0 publishes. The window matters: NAM-024 forbids a new default once `.v2` is out, so a later Type input means `.v3`. It holds four changes to the turn's contract:
   - a conversation the caller chooses (new, or a named one to continue), with its ID returned;
   - a report schema passed to the client's own structured-output option and returned as structured output, with no repair turn;
   - a failure as fields (class, with a `network` class; the turn's number; whether the client started) and a bounded tail of the client's error events for diagnosis;
   - the wrapper owning its client's process tree (I2).
   Order 050a deferred the first two by name ("an input for a new conversation, an output schema").
3. One engine order: the runner records a stop only after it has seen the group empty; a call whose deadline fires is cancelled.
4. Re-pin the dogfood engine (it is 57 commits behind the reviewed tree); with the owner's answer to I3, move it under the dedicated user; run 046 and 047 against the parity measures.
5. In the lane: ruling 75 B2's confining runtime provider, wrapping the sandbox the clients themselves use (Seatbelt through `sandbox-exec` on macOS; bubblewrap on Linux, which ruling 77's setup already installs). It makes "the folder is the grant" true for writes and puts the engine's store and root's token out of an Agent's reach. It would be the first enforced restriction in the project (preservation test 9).
6. The three probes of I7, before order 5.

Stop:
- restarting the engine to get a fresh conversation;
- counting an order "run through Conduit" as evidence about access; record instead what Conduit refused, opened and closed for it;
- adding harness guards that read state the checked Agent controls;
- raising deadlines as the answer to long turns;
- keeping the dogfood's facts in private notes;
- planning the coordinator Agent on nested blocking turns before the probe.

Cost: about three admin orders and one or two lane orders (my estimate, not measured). It buys: no restart (171 to 191 s for each fresh conversation, at least eight in 042 if each turn began one, so about 24 minutes an order by that arithmetic); parity with the plain runner; a stop and a fence that mean what they say; and dogfood results that can be read from the repository.

None of this adds a language concept. Each change sits in the Type contract, the runner or a provider, where the layered claim puts behavior.

**Evidence.** spec/promises/naming.yaml:421-425 (NAM-024: optional additions only; an added Type property states no default). docs/work-orders/050a-agents-v2-own-tools.md:446-448 ("Do not add ... an input for a new conversation, an output schema ... Order 050 and later changes add what is needed, on this branch"). docs/implementation/HANDOVER.md:127-129 (the `.v2` packages stay unmerged until 050, because of NAM-024), :148 (171 to 191 s), :153-154 (pinned at `38c0b4f`). `git rev-list --count 38c0b4f..2eb8f258` gives 57. docs/proposals/ruling-75/README.md:43-46 (B2: "The first confining provider is to wrap an existing whole-process sandbox"). docs/implementation/HANDOVER.md:181-183 (bubblewrap in ruling 77's part 2). docs/AUDIT.md:306 (preservation test 9). docs/AUDIT.md:379-382 (layered claim: Type contracts and runtime profile).

**Waits.** Step 4's move to a dedicated user waits on I3. Steps 1 to 3 wait on nothing. The package order must land before order 050 publishes 0.2.0.

**Verifier.** partly. Holds: 050a deferred "an input for a new conversation, an output schema" to this branch (docs/work-orders/050a-agents-v2-own-tools.md:446-449); the pin is 57 commits behind (`git rev-list --count 38c0b4f..2eb8f258`); ruling 75 B2's words (ruling-75/README.md:43-46); 8 turns at 171 to 191 s is about 24 minutes.

**Verifier's counterargument.** 1. The window is overstated. NAM-024 lets a later release add "an optional input of an operation" and a Type property with no default (spec/promises/naming.yaml:421-425); only a default needs `.v3`. 2. Un-poisoning a failed conversation is not the coordinator's before the rerun. That rule drove the fresh author's recovery, 128 lines at T7 (docs/proposals/language-shape/short-form/synthesis.md:622), and 0.2.0 is on the rerun path; ruling 71 point 4 excludes what could move S1's counts. 3. "The first enforced restriction" is wrong as the language stands: "This version reports mediated for every restriction, whatever the consumer's runtime provider is" (spec/conventions/access.yaml:451-452) and CTR-014 says the same of own tools. Reporting it needs a change to spec/, the owner's. The engine on `main` cannot call a runtime provider yet (CONFORMANCE.md:41; order 080). 4. Bringing the harness in meets "Code is Rust" (AGENTS.md, Rules): the runners are scripts, so it is a rewrite or an exception from the owner. 5. "One package order" changes two packages that duplicate one contract and one source (the two `turn.v1` documents are identical; `diff` of the two lib.rs shows the `CLAUDE` constant and two feature names). Claude's wrapper runs one long-lived process with `--no-session-persistence`, so a per-turn schema and "continue a named conversation" do not carry over from Codex. 6. A kept tail of client error events reverses 050a's rule that "No client text reaches the result, an error or stderr" (:324). 7. 24 minutes is small beside one 3.3-hour turn, and was measured before order 070.

**Verifier's corrections.** Split step 2: conversation input, report schema and failure fields are the coordinator's (71 point 4); the poison rule goes to the owner or after the rerun. Bubblewrap is in ruling 77's part 2, not yet run (HANDOVER.md:180).


## FR-337

**A78's register row: stages, a pass condition per order, counted measures, and three probes**

P06 I7 · experiment-change · verifier: partly · decides: coordinator

Row: A78 in ROADMAP.md's register. Today its done-when is "Five orders run through Conduit, each friction logged as a finding of the first build". After one order, no friction is logged, no measure was fixed beforehand and no result is in the repository.

Claim it should test: work orders run through Conduit with every limit the plain runner gives, and with at least one limit that only Conduit gives.

Stages (replacing "orders 1 to 3 by root, 4 and 5 by a coordinator Agent"; the last two need the owner's answer to I4):
- Stage 1, done: 042, by root, unconfined.
- Stage 2: orders 2 and 3 (046, 047), by root, with the fixed turn contract and, if the owner agrees, under the dedicated user.
- Stage 3: order 4, dispatched by an appointed Interface with its own token (batch 2).
- Stage 4: order 5, by a coordinator Agent, after the confining provider and probe P2.

Pass condition for each order, fixed before it runs:
1. Parity with the plain runner: nothing written outside the slot; the verifier writes nothing; the report arrives as schema-valid structured output with no repair turn; every failed turn names its class; no engine restart between turns; no process of a turn alive after its failed end or after the run's stop.
2. No guard trips, and none is unknown at merge.
3. Every unknown outcome is reconciled, with what was checked.

Measures, counted per order in a dated result under a new directory (`experiments/024-a78-dogfood`, the next unused number):
- turns by role; fresh conversations and the seconds each cost;
- failed turns by class; unknown outcomes and how each was settled;
- harness interventions: repair turns, restarts, process sweeps, guard trips, guard unknowns;
- wall-clock per round, against the plain runner's for the last five orders of the same kind;
- lines of the Fleet's documents written and changed;
- requests Conduit refused, and Conduits opened and closed, for the order. Zero means the order tested no access, and the result says so.

Three probes, each a new row (the next A-numbers when accepted), each with scripted or signed-in clients as the row states:
- P1, network loss. Cut the network for 10, 30 and 120 s at two points (while the model streams; during a local command), under the `.v2` wrapper and under plain `codex exec` with the runner's flags. Pass: the same result under both launches at each point, and a failed turn names `network`. It settles whether "a plain Codex reconnected" was the wrapper or the moment.
- P2, a lost caller. During a long turn, drop the caller's stream, then kill the caller. Measure: whether the worker's turn ends; whether its conversation is still usable; whether a repeat with the same key returns the report (IFC-016, OPS-009), given that the Type keeps outputs out of the records. It decides blocking turns against tasks before order 5.
- P3, the process tree. A turn starts a background job, a terminal job and a double-forked job; end it by deadline, by cancellation and by stopping the run. Pass: none alive afterwards, or the result names the survivors, and no stop or fence is recorded before that.

**Evidence.** ROADMAP.md:71 (A78's row and done-when), :96-100 ("Each experiment states, before it runs, what it must show and how each measure is counted (ruling 45)"; directory numbering). `ls experiments/` ends at 023-paper-count; no directory for A78. `grep -c '^| 4-' docs/proposals/first-build/log-increment-4.md` gives 20, and none of the six frictions is among them (only 4-20 cites A78, found in planning). docs/implementation/HANDOVER.md:152-153 ("None is yet a finding in increment 4's log; ruling 70 asks that each be"). ROADMAP.md:70 (A77: "admin time was not recorded, so no complete spend comparison is available"). docs/implementation/HANDOVER.md:143-146 (the network loss, and "Plain `codex exec` workers reconnected by themselves").

**Waits.** The measures and the directory should exist before 046 runs through Conduit. Stages 3 and 4 wait on the owner's answer to I4. P2 gates order 5.

**Verifier.** partly. Holds: A78's row and done-when (ROADMAP.md:71); the rule that measures are fixed before a run (ROADMAP.md:96-97); `ls experiments/` runs 004 to 023 with no directory for A78, and no remote branch holds an `experiments/024*`; `grep -c '^| 4-' docs/proposals/first-build/log-increment-4.md` gives 20, and only 4-20 mentions A78; HANDOVER.md:152-153.

**Verifier's counterargument.** 1. Pass condition 1 is not provable as written. "Nothing written outside the slot" and "the verifier writes nothing" are checked by a harness running as the same user as the Agent it checks, which I10 itself calls evidence under the Agent's control. Until the dedicated user, record them as observed, not as a pass. 2. The measures miss what the assignment named: the verifier's independence. Both slots share one home and login until order 050 (packages/agent-codex/README.md:39-40), and Codex resumes a thread in a fresh process, so the thread is kept in that home; add "what the verifier could read of the worker's conversation". 3. P2's premise mixes two things. `records.outputs` governs records (spec/conventions/records.yaml:50-53); a repeat is answered from retained results, which the shell keeps up to 64 MiB and 7 days (engine/CONFORMANCE.md:30). 4. P2 cannot "decide blocking turns against tasks" while the engine has no tasks code; it can only measure the blocking route. 5. Stages 3 and 4 rewrite ruling 61 B12; the item says so, but then the row cannot be edited before the owner answers I4.

**Verifier's corrections.** Add to P3 a stop by TERM with a grace period, to tell an escape from a kill without warning. Mark stages 3 and 4 as proposed, pending I4.


## FR-338

**The network loss: three faults of the Agent package, none of convention 7 (draft row 4-21)**

P06 I8 · assessment · verifier: partly · decides: coordinator

In play: IMP-007 (the engine dispatches nothing again without the caller's repeat), IFC-010 (any error of a plain implementation on an operation marked `startsModelWork: true` is unknown), OPS-014 (reconciling it), and the package's own rule "session cannot restart implicitly".

Fault of the Agent package, in three parts:
1. Its class table has no network class, so the cause read `other`. The package itself calls the table "an initial list tested with synthetic failures, not real failures".
2. It keeps no client event and sends the client's stderr to null, so why Codex ended the turn cannot be established afterwards.
3. It refuses every later turn of the conversation. No promise asks that: `spec/` has no rule on conversations, and IMP-007 binds the engine's dispatch, not a later explicit turn that continues the thread.

Convention 7 and resilience do not conflict. A client's reconnect inside one explicit turn is not a second dispatch, and order 050a already made the wrapper ignore Codex's retried stream errors. The caller sending a new turn is explicit. Only the package's reading turns one failed turn into a dead conversation.

Not established: that the wrapper is less resilient than plain `codex exec`. The wrapper fails a turn only where Codex itself reports `turn.failed` or exits, as the plain run would. The plain workers may simply have been running a local command during the loss. Probe P1 in I7 settles it. If Codex's own retry limits were the cause, raising them for `.v2` through its configuration is the least fix (setting names and defaults to be checked against the pinned client; I have not verified them).

A smaller point: because the wrapper is `plain`, even a turn whose client never started is unknown and must be reconciled, and stays listed on the Agent's status until it is. A field saying whether the client started lets the caller act without guessing.

Draft row:
| 4-21 | defect of the build | IMP-007, IFC-010, OPS-014; `turn.v1` | On 2026-11-12, in order 042's round 3, a brief loss of the Mac's network ended worker-1's turn after 55 s (log): "client turn 1 failed: client failure: other; session cannot restart implicitly". The outcome is unknown, as IFC-010 gives for any error of a plain implementation on an operation marked `startsModelWork: true`. Three faults of the Agent package meet here. Its class table has no class for a network failure, so the cause reads `other`. It keeps no client event and discards the client's stderr, so why Codex ended the turn cannot be told afterwards. And it refuses every later turn of that conversation, which no promise asks: IMP-007 forbids the engine dispatching again without the caller's repeat, and `spec/` has no rule on conversations. Plain `codex exec` workers running then went on (log); whether they were calling the model during the loss is not recorded, so the comparison is not established. | The harness restarted the engine and ran the round again as a new request. One guard tripped in the failed turn (4-25). | Proposed, for the coordinator: in the `.v2` wrappers before 0.2.0 publishes, a `network` class; the failed result carries its class, the turn's number and whether the client had started as fields, not only as text; a failed turn leaves the conversation open to an explicit later turn that names it (4-24); the wrapper keeps a bounded tail of the client's error events, without prompt or model text, for the diagnosis. Codex's own limits on retries are raised for `.v2` if a probe shows they were the cause. No change to `spec/`. |

**Evidence.** docs/implementation/HANDOVER.md:143-146. docs/work-orders/042-build-tombstones-and-inventory.md:52-54 ("the Agent package answered \"client failure: other\"; a dogfood finding"). spec/conventions/implicit.yaml:59-76 (IMP-007). spec/interface/implementation.yaml:225-230 (IFC-010). spec/conventions/operations.yaml:325-326 (an unreconciled invocation stays listed under LIF-013). docs/work-orders/050a-agents-v2-own-tools.md:274-277 ("An `error` event does not end a turn ... a turn that lasts hours would otherwise fail at its first reconnect"), :306-316 (class table: usage-limit, rate-limit, authentication, context-window, other), :318 ("a first list, not checked against real failures"). Branch: `git show origin/wo/076-agents-v2-folder-grant:packages/agent-codex/src/lib.rs` lines 494-496 ("session failed at turn {k}: {cause}; implicit restart refused"), 464 (stderr null); same ref, conduit.yaml (`wrapper-v2` is `plain: true`). packages/agent-codex/README.md:43-44.

**Waits.** Folds into the package order of I6, before 0.2.0. Probe P1 before any claim about plain Codex's resilience is relied on.

**Verifier.** partly. Holds, each read in the branch wrapper: the class table has four classes and `other` (lib.rs:385-417); stderr goes to null and no event is kept (464, and the loop's comment); every later turn is refused (495-497); an `error` event does not end a turn and only `turn.failed`, EOF or a bad exit does (the `run_turn` loop), as 050a ordered (docs/work-orders/050a-agents-v2-own-tools.md:273-277). IFC-010 gives unknown (spec/interface/implementation.yaml:225-230). IMP-007 binds the engine's dispatch (spec/conventions/implicit.yaml:59-76). The README's "initial list tested with synthetic failures" is at branch README:229. The caution about plain Codex is fair.

**Verifier's counterargument.** 1. Fault 3 cost nothing in this incident: the pipeline wants a fresh conversation each round, so the round would have restarted the engine anyway. It matters for S1, and there it is not the coordinator's to change before the rerun (see I6, point 2). 2. The refusal has a reason the item does not weigh. Claude's wrapper runs with `--no-session-persistence`; after a failure its context is gone, so a later turn would silently be a new conversation. Refusing is the explicit answer (ruling 52). 3. A kept tail of client error text reverses 050a's rule against client text in results (:324, and the code comment on authentication material). A `network` class by the same fixed-substring method is the least fix.

**Verifier's corrections.** The row's date is not established: the Record says "2026-11-11 and 12" (docs/work-orders/042-build-tombstones-and-inventory.md:49). "Restarted the engine and ran the round again as a new request" is inferred; the Record says "was run again" (:54), and nothing says whether the unknown turn was reconciled.


## FR-339

**A fresh conversation costs an engine restart because the turn's contract has no conversation (draft row 4-24)**

P06 I9 · assessment · verifier: supported · decides: coordinator

In play: `turn.v1` (its only input is `message`), the package's one conversation per run, its rule that a failed turn ends the conversation, ruling 61 B12 ("nothing retires a slot"), and CTR-014 (the directory is fixed for the Agent's life). No promise of `spec/` is in play: it has no word for a conversation, rightly, since the layered claim puts an Agent's conduct in its Type's contract.

Fault of: the Agent package's contract, with the harness doing outside Conduit what that contract should offer. Inside the language the only routes to a new run are retiring the Agent or making another, and B12 rules both out for a slot. Fixed slots are themselves right: each Agent home signs in on its own, so a fresh Agent per order would need a person's sign-in per order. So the fresh context has to come from the conversation, not from a new instance.

The pipeline's shape (a fresh conversation for each role and round, a verifier that has not seen the worker's context) is then expressible in the call itself and visible in the record, instead of being a side effect of killing the engine.

Least fix: the turn takes an optional input that asks for a new conversation or names one to continue, and returns the conversation's ID. That is A2A's `contextId`, `codex exec resume` and Claude Code's `--resume`. NAM-024 allows an optional input on `turn.v1`; to keep `.v1`'s documents byte for byte, as order 050a requires and S1 relies on, offer it as a `turn.v2` on the `.v2` Types instead. The handover already names this follow-up (`newConversation`); the point here is its priority: before the next order, not after.

The 171 to 191 s were measured on an engine pinned before order 070, so the restart itself may be far cheaper after the re-pin. Measure it again, but do not rely on it.

Draft row:
| 4-24 | gap | `turn.v1`; NAM-024; ruling 61, B12 | The pipeline needs a fresh conversation for each role and round. `turn.v1` takes only `message`, a run holds one conversation, a failed turn ends it, and B12 retires no slot. The harness therefore restarts the engine for each fresh conversation: 171 to 191 s each in order 042's round 4 (log), on an engine pinned before order 070. `spec/` says nothing of conversations, rightly: they belong to the Agent Type's contract. | The harness restarts the engine. | Proposed, for the coordinator: the `.v2` Types offer a turn whose optional input asks for a new conversation or names one to continue, and whose result gives the conversation's ID, as A2A's `contextId` and `codex exec resume` do; as an optional input of `turn.v1`, which NAM-024 allows, or as a `turn.v2` that leaves `.v1`'s documents byte for byte. The restart is measured again after the re-pin. |

**Evidence.** packages/agent-codex/contracts.yaml:39-47 (`turn.v1` input: `message` only, `additionalProperties: false`). docs/implementation/HANDOVER.md:124 ("restarts the engine for each fresh conversation"), :148, :153-156 ("a fresh conversation without a restart (`newConversation` on `turn.v1`, on 050a's branch)"). docs/proposals/ruling-61/README.md:28 ("nothing retires a slot"). docs/proposals/ruling-75/README.md:85-86 ("every Agent home signs in on its own"). spec/promises/naming.yaml:421-422 ("may add optional things, such as an optional input of an operation"). docs/work-orders/050a-agents-v2-own-tools.md:77-78 ("`.v1` stays byte for byte"), :472-474 (the byte-prefix check). `git merge-base --is-ancestor 38c0b4f 1a34ccf9` succeeds (the pin is before 070). docs/AUDIT.md:381 (Type contracts: "How each type behaves").

**Waits.** Orders 046 and 047 through Conduit should not run on the restart route; folds into the package order of I6.

**Verifier.** supported. `turn.v1` takes only `message`, `additionalProperties: false` (packages/agent-codex/contracts.yaml:42-47). The wrapper holds one thread per run and refuses after a failure (branch lib.rs:479-497). "restarts the engine for each fresh conversation" (HANDOVER.md:124); 171 to 191 s (:148); the planned `newConversation` (:155-156). "nothing retires a slot" (docs/proposals/ruling-61/README.md:28). `grep -rli conversation spec/` finds no file. NAM-024 allows an optional input (spec/promises/naming.yaml:421-422). `git merge-base --is-ancestor 38c0b4f 1a34ccf9` exits 0. 050a's byte-prefix check is at :471-473.

**Verifier's counterargument.** A separate operation in the capability ("start a new conversation", not marked as model work) may fit the purpose better than an input on the turn: a Conduit could then grant turns without the right to discard an Agent's context, which is an access question. NAM-024 allows "a further operation of a capability". The priority ("before the next order") rests on 24 minutes an order measured before 070; the stronger reason is that the pipeline's shape becomes visible in the record.

**Verifier's corrections.** "The only routes to a new run are retiring the Agent or making another" needs one more: a further setup starts a run that has stopped (engine/CONFORMANCE.md:40, "Explicit subsequent setup may start a stopped run"), which is what the restart exploits; no operation stops a run short of retirement. "Names one to continue" works for Codex (`resume`), not for Claude's wrapper as built (`--no-session-persistence`). The handover's plan, `newConversation` on `turn.v1`, would fail 050a's byte-prefix check; say which route is taken.


## FR-340

**The write inside `.git`: a stated limit, with guards that trust what the Agent can write (draft row 4-22)**

P06 I10 · assessment · verifier: partly · decides: coordinator

In play: CTR-014 and PRV-006 (an Agent's own tools are neither enforced nor mediated; on the engine's own runner nothing confines them), ACC-027, ruling 70 point 2, and the `.v2` Type's own description, which says so.

What happened, read with order 050a: the admin had left unchecked "whether bypassPermissions still prompts for a write inside `.git`". The smoke test answered it: no approval is asked. So "with no prompt" means without a permission prompt. It is the absence of the client's last guard, not an Agent acting unasked. I am fairly confident of this reading; the handover's sentence alone would allow the other.

Fault of: neither the language nor the package. Both state the limit. The fault is in the harness, which relies on that state:
- Its guards (HEAD unmoved, the verifier changed nothing) run git inside a repository whose `.git/config` and hooks the checked Agent can write. Git runs configured programs on ordinary commands, so the evidence is under the Agent's control. Today both sides are one OS user, so nothing is escalated; once a dedicated user exists, a guard run by another user inside the slot would execute what the Agent planted.
- Ruling 75's later answer names branch protection as the guard until B3. Ruling 74 says that protection does not stop an Agent using the owner's GitHub login.

Least fix: the harness reads a slot only by fetching from it into a repository of its own, which is how a CI service reads an untrusted checkout and what git's `safe.directory` rule exists for; it never runs git inside the slot. The durable fix is confinement (I3).

Draft row:
| 4-22 | gap | CTR-014, ACC-027, PRV-006 | In the smoke test (log), `claude-code.v2` in the permission mode `bypassPermissions` wrote inside its slot's `.git`, and the client asked for no approval; order 050a had left that unchecked. No promise is broken: CTR-014 says nothing confines an Agent's own tools on the engine's own runner. The gap is in what stands guard meanwhile. The harness's guards (HEAD unmoved, the verifier changed nothing) run git inside a repository whose configuration and hooks the Agent they check can write, and `main`'s branch protection does not stop an Agent that uses the owner's GitHub login (ruling 74, point 1). | Nothing; ruling 75's later answer of 2026-11-12 keeps the dedicated macOS user where B3 puts it. | Proposed, for the coordinator: the harness reads a slot only by fetching from it into a repository of its own, as a CI service reads an untrusted checkout, and never runs git inside the slot. For the owner: when the dedicated user and the confining runtime provider come (ruling 75, B2 and B3). |

**Evidence.** docs/implementation/HANDOVER.md:133-134 ("`claude-code.v2` runs with no prompt", listed as a passed check), :147 ("wrote inside the slot's `.git` with no prompt: unconfined, as ruling 70 allows"). docs/work-orders/050a-agents-v2-own-tools.md:103-105. docs/proposals/ruling-75/README.md:125-128. docs/proposals/ruling-74/README.md:26-30. spec/promises/contracts.yaml:345-350; spec/promises/providers.yaml:199-204 (PRV-006: own tools "neither enforced nor mediated, whatever its runtime provider"). docs/proposals/ruling-70/README.md:33-35 ("Nothing confines them. The Type's contract says so"). Branch: `git show origin/wo/076-agents-v2-folder-grant:packages/agent-codex/src/lib.rs` lines 186, 285 (`bypassPermissions`).

**Waits.** The harness change waits on nothing. The owner's part is I3.

**Verifier.** partly. Holds: the reading that "no prompt" means no permission prompt. 050a left unchecked "whether bypassPermissions still prompts for a write inside `.git`" (docs/work-orders/050a-agents-v2-own-tools.md:103-105), and the handover lists "`claude-code.v2` runs with no prompt" among the smoke test's passes (HANDOVER.md:133-134). No promise is broken: CTR-014 (spec/promises/contracts.yaml:348-352), PRV-006 (spec/promises/providers.yaml:199-204). The two rulings' words on branch protection are quoted correctly.

**Verifier's counterargument.** 1. The fault laid on the harness cannot be established: the harness is outside the repository, so how its guards run git is a guess. 2. Under one user the point is moot, as the item admits: an Agent that would plant a hook can as easily move HEAD back. Ruling 74 says the guards are for "destructive mistakes", and the project's stated model is mistakes and injection, not compromise (docs/proposals/projects/brief.md:1260). 3. The least fix does not work as written. The worker commits nothing and the guard is "HEAD unmoved" (HANDOVER.md:20, 125), so the work product is uncommitted; a fetch brings none of it. The fix that fits is git run with the harness's own repository directory and the slot as the work tree, or plain file reads.

**Verifier's corrections.** The full review's brief says "unprompted" (docs/guides/full-review.md:170-171) where ruling 75's record says "with no prompt"; fix the brief's word. Class: this is a stated limit observed, with a harness weakness; "gap" in the log means a situation no promise decides.


## FR-341

**The guard that tripped on GitHub's refs: a harness check that reported "cannot tell" as a breach (draft row 4-25)**

P06 I11 · assessment · verifier: supported · decides: coordinator

In play: no promise. The check is the harness's own, kept under ruling 74 point 1 ("the harness checks after each turn that nothing was pushed"). Ruling 52's rule applies to it as to any result: say what happened and why.

Fault of: the harness. It has two answers where three are needed. The language itself keeps "unknown" as the one word for "cannot tell"; the guard should do the same.

The deeper cause is the same as I10's. The guard needs GitHub only because an unconfined worker holds the owner's GitHub login: its clone "cannot push" by configuration, not by lacking a credential. So the turn loop depended on three networks (the model provider, GitHub, loopback) where only the first is inherent. Under a dedicated user with no GitHub credential, "nothing pushed" holds by construction and the guard goes.

Draft row:
| 4-25 | defect of the build | none; ruling 52 | In order 042's failed turn (4-21), the harness's guard that nothing was pushed could not read GitHub's refs and tripped. It reported "cannot tell" as a breach. The guard needs the network only because a worker holds the owner's GitHub login: its clone cannot push by configuration, not by lack of a credential. | The round was run again. | Proposed, for the coordinator: each guard answers held, broken or unknown, and an unknown is checked again before the next turn is sent. Under a dedicated user that holds no GitHub credential the guard is not needed. |

**Evidence.** docs/work-orders/042-build-tombstones-and-inventory.md:53-54 ("one guard tripped in that failed turn, when GitHub's refs could not be read"). docs/implementation/HANDOVER.md:136-137 ("no guard tripped except once, in the outage's failed turn, when GitHub's refs could not be read"), :123 ("a separate clone that cannot push"). docs/proposals/ruling-74/README.md:28-30. spec/definitions.md:377 ("unknown | The one word for \"cannot tell\""). Branch README, `git show origin/wo/076-agents-v2-folder-grant:packages/agent-codex/README.md` lines 147-148 ("It can push through the keychain, whatever HOME is").

**Waits.** Nothing waits; the guard disappears if I3's option B is chosen.

**Verifier.** supported. "one guard tripped in that failed turn, when GitHub's refs could not be read" (docs/work-orders/042-build-tombstones-and-inventory.md:54; HANDOVER.md:136-137). The clone "cannot push" by arrangement (HANDOVER.md:123) while the Agent "can push through the keychain, whatever HOME is" (branch README:147-148). Ruling 74 point 1 keeps the check (docs/proposals/ruling-74/README.md:29-30). spec/definitions.md:377 (unknown as the one word for "cannot tell").

**Verifier's counterargument.** "Reported cannot tell as a breach" is an inference from "tripped"; it is the natural reading, but the harness's output is not in the tree. A three-valued guard is the right least fix and is the coordinator's.

**Verifier's corrections.** "The guard is not needed" under a dedicated user would drop a guard that ruling 74 point 1 keeps ("the harness checks after each turn that nothing was pushed"); that is the owner's, and the row should say so. "Defect of the build" is loose for a program outside the build; the log may still hold it under ruling 70.


## FR-342

**A78's evidence and its means are outside the repository; the reviewed tree cannot reproduce the dogfood**

P06 I12 · fact · verifier: supported · decides: coordinator

Established at 2eb8f258:
- The `.v2` Agent Types that ran 042 are not in the tree. `packages/agent-codex` and `packages/agent-claude-code` hold 0.1.1 with `codex.v1` and `claude-code.v1` only. `.v2` lives on three unmerged branches (050a, 076, 078), held until order 050, which is not written.
- The harness is "outside the repository". The conduit-core Fleet's documents are not in the tree either: `conduit-core` appears only in prose, and `worker-1` and `verifier-1` only in the handover and 042's Record.
- The dogfood engine is pinned at `38c0b4f`: it has order 074, and has neither 070 nor 077. It is 57 commits behind the reviewed tree.
- A78 has no directory under `experiments/` and no dated result. The smoke test's six checks, 042's turn times and every friction are marked "(log)": the coordinator's private notes.
- Increment 4's log has 20 rows; none records a dogfood friction. A78's done-when asks for each to be logged. Its other clause stands at one order of five.
- 042's Record is 16 lines and gives no count of repair turns, restarts or reconciliations.
- IMP-001, IMP-002, IMP-003, IMP-007 and IMP-008 each carry `vectors: []` in the promise file; the file's own index line says vectors test slices of two of them.

Consequence: the activity that ruling 70 makes the first priority is the one whose configuration, tooling and results a reader of the repository cannot check. AGENTS.md's own rule is that evidence goes in its own directory under `experiments/`, with dated results.

**Evidence.** packages/agent-codex/conduit.yaml:3-7 (`version: 0.1.1`, exports `codex.v1`, `conversation.v1`, `turn.v1`). `git branch -r | grep -E '050a|076|078'` lists origin/wo/050a-agents-v2-own-tools, origin/wo/076-agents-v2-folder-grant, origin/wo/078-agents-codex-path-item. docs/implementation/HANDOVER.md:123-129, :131-150 ("(log)"), :153-154, :217 (050 not written). `grep -rn 'worker-1\|verifier-1' --include='*.md' --include='*.yaml' .` finds only HANDOVER.md:122, the 042 order file:50 and three lines of docs/proposals/projects/brief.md about a different, hypothetical Agent. `git merge-base --is-ancestor 6591a064 38c0b4f` succeeds (074 in the pin); the same test for 575a1bcd fails (077 not in it); `git rev-list --count 38c0b4f..2eb8f258` gives 57. `ls experiments/`: 004 to 023, none for A78. `grep -c '^| 4-' docs/proposals/first-build/log-increment-4.md` gives 20. ROADMAP.md:71. spec/conventions/implicit.yaml:29, 44, 57, 76, 94 (`vectors: []`); spec/README.md:249. AGENTS.md:79-80 ("Evidence: its own directory under `experiments/`, with dated results").

**Waits.** Step 1 of I6 and the directory of I7 close it.

**Verifier.** supported. Each recounted: packages at 0.1.1 with `.v1` only (packages/agent-codex/conduit.yaml:3-7; packages/agent-claude-code/conduit.yaml:3-7); `.v2` on origin/wo/050a, 076 and 078; `conduit-core` in prose only and `worker-1`, `verifier-1` only at HANDOVER.md:122 and the 042 order file:50 (plus three lines of brief.md about another Agent); 074 in the pin, 070 and 077 not, 57 commits behind; no experiment directory; 20 log rows; the Record is 16 lines with its heading.

**Verifier's counterargument.** The consequence drawn is fair, with one cost unstated: the runners are scripts and AGENTS.md's rule is that code is Rust, so bringing the harness in is a rewrite or an exception.

**Verifier's corrections.** The last bullet misleads. `vectors: []` is the state of 517 of 519 promises (`grep -rn 'vectors: \[\]' spec/ | wc -l`); coverage is kept in conformance/manifest.yaml, which names IMP-002 and IMP-007 six times each. It says nothing about dogfooding; drop it.


## FR-343

**What is working: the frictions landed in the layers the layered claim assigns them to**

P06 I13 · assessment · verifier: partly · decides: none

Dogfooding is, so far, mild evidence for the layered claim and for ruling 40, not against them.

- Of the six frictions, five are faults of the Agent Type's contract (conversation, failure classes, deadline), of the engine's own runner (the stop) or of the harness (two guards). The layered claim puts exactly these in "Type contracts" and the "runtime profile", outside the language. `spec/` has no word for a conversation or a deadline, and did not need one.
- No friction asked for a new family, a new kind of Conduit or a new outcome. Every fix proposed here is an optional input, a provider or a build defect.
- The one convention-layer finding is the fence (I2), and its fix is a sentence of limit and two build defects, not new machinery.
- Convention 7 held up. An unknown outcome settled by looking at the tree is how a work order is in fact recovered (the runner's "rounds after the fact"), and OPS-014 already has the actor record what it checked.
- The pipeline's queue and rounds belong outside the language: by convention 7 nothing wakes an Agent, so an actor must drive each step, and "a system outside" acting through an Interface is the definition of one. The harness is that Interface. What is wrong is only that it acts as root, with unbounded access, and holds limits that should be Conduit's or a provider's.
- Finding 4-20 (a session lost under a long call) was a real engine fault found only by planning real use.

The caution: this evidence covers the turn. It says nothing yet about connection and access, the only place Conduit claims to innovate, because stage 1 exercised none of it (I1).

**Evidence.** docs/AUDIT.md:372-382 (the layered claim's table: "Type contracts: How each type behaves: an Agent's conditions ... Owned by the type's author"; "Runtime profile: Engine choices: outage handling, retries, cursors, process layout"), :384-388 (ruling 40's provider contracts). `grep -rli conversation spec/` finds 0 files; `grep -rn -i deadline spec/` prints nothing. spec/conventions/operations.yaml:309-310 (OPS-014: "An actor records, with what it checked"). docs/implementation/HANDOVER.md:68-71 (rounds after the fact: `START_ROUND`, `VERIFY_FIRST`). spec/definitions.md:60 ("Interface: ... where a person, or a system outside, acts on the system"). spec/conventions/implicit.yaml:31-44 (IMP-002). docs/proposals/first-build/log-increment-4.md:31 (4-20).

**Verifier.** partly. Holds: no friction asks for a new family, kind of Conduit or outcome; `spec/` has no word "conversation" or "deadline"; 4-20 was found by planning real use (docs/proposals/first-build/log-increment-4.md:31); the harness acts as root's Interface.

**Verifier's counterargument.** 1. The evidence is selected. Stage 1 passed only turns through Conduit (I1), so frictions could land only in the turn, the package and the harness. That is no evidence for the layered claim; the item's own caution should be its conclusion. 2. "Convention 7 held up" is not shown. No reconciliation is recorded for 042's unknown turn; it "was run again" (042 order file:54), which sidesteps the repeat and the fence altogether. For the first real consumer, recovery was a new request. 3. There is evidence against clean layers. One deadline had to be set in three layers (the Type's input, the client's tool timeout, the engine's call timeout and session idle limit: review-list items 21 and 23, finding 4-20). And a driver needs two vendors' turns to agree, which the two packages meet by duplicating one contract and one source file. By LD1 (docs/AUDIT.md:406) that is a shared contract, not a matter for one Type; A2A's task and context are the existing paradigm for it.

**Verifier's corrections.** "Rounds after the fact" is HANDOVER.md:64-67, not 68-71. Finding 4-06's settlement (sub-agents have no shell) no longer holds for `.v2` with a directory: "their subprocesses inherit this permissive reach" (branch README:150-151).


## What the verifier says the analyst missed

- **The verifier's independence, which the assignment named, is not analyzed** The pipeline needs "a verifier that must not see the worker's context". worker-1 and verifier-1 are separate Agents, but both run as one OS user and share one home and login until order 050 (packages/agent-codex/README.md:39-40; branch README:146-147), and the Codex wrapper resumes a thread by ID in a fresh process, so the thread is kept in that shared home. An unconfined verifier can read it. The plain runner has the same weakness (ruling 70's record: the read-only sandbox reads any file its user can), so this is parity, not a regression, but no measure in I7 covers it. Coordinator's: add the measure; separate homes come with 050.
- **The engine on `main` cannot call a runtime provider, so the confining provider cannot run an Agent yet** engine/CONFORMANCE.md:41: "Runtime provider calls and remote connections are unserved". Order 080 builds them, is checkpointed with three musts, and is bound for `s3/m2`, merged after the rerun's pin (HANDOVER.md:212, 219). I3's option B and I6's step 5 both rest on ruling 75 B2's provider and state no such dependency. It also is not in the lane's queue (no mention in docs/implementation/PROVIDERS.md).
- **A confining provider would not change what inspection reports in this version** ACC-027: "This version reports mediated for every restriction, whatever the consumer's runtime provider is" (spec/conventions/access.yaml:451-452); CTR-014 and PRV-006 say own-tools reach is "neither enforced nor mediated, whatever the Agent's runtime provider". So building the provider confines in fact and shows nothing in the language. If dogfooding is to test preservation test 9, the owner must decide when the report may say "enforced"; that is a language change the analyst's plan needs and does not raise.
- **Ruling 71 point 4 withholds the poison rule from the coordinator until the S1 rerun** The rule that a failed turn ends its conversation drove the fresh author's recovery in increment 4 (finding 4-12), 128 lines at T7 (docs/proposals/language-shape/short-form/synthesis.md:622). The 0.2.0 packages are on the rerun path (docs/proposals/spec-batch-1/README.md:664). Changing the rule could move S1's author counts, which ruling 71 point 4 excludes from the coordinator's delegation. I6, I8 and I9 give it to the coordinator without saying so. It also interacts with I2: the poison rule is what now stops a new turn running beside a failed turn's leftover processes in the same run.
- **Cheaper steps on the order of risk that need no new ruling, and one option not listed** Ruling 77 part 2 is waiting and takes the coordinator's key out of the owner's sudo-capable Windows account (HANDOVER.md:180-183; ruling-77/README.md:37-38), which cuts the Mac worker's path to the second machine. Order 050 already re-initializes the store and has the owner sign each slot in (HANDOVER.md:157-158), so the dedicated user costs one owner session if it is set up then, two if set up before 046. A further option for the owner: dogfood under the existing `conduit-run` user, which amends ruling 76 point 6.
- **MCP's progress notifications, the baseline paradigm for a long call, are not considered** The engine sends no progress during a call (docs/work-orders/050a-agents-v2-own-tools.md:95), and neither spec/ nor engine/CONFORMANCE.md mentions progress notifications. Relaying them would give a caller liveness during a turn of hours and reset idle timers (finding 4-20) without the tasks extension, which the engine does not implement at all (no `tasks/` code under engine/crates). Whether the engine relays progress is a behavior spec/ does not decide: a gap to log, the coordinator's unless it adds a feature.
- **Finding 4-06's settlement no longer holds for `.v2` Agents** The owner confirmed (review-list item 15) that a Codex Agent's sub-agents are "a stated route, not a bypass" because a spawned sub-agent had no shell and a read-only sandbox. With `.v2` and a directory, the branch README says "their subprocesses inherit this permissive reach" (lines 150-151). A dogfood worker can therefore run parallel, unconfined model work that no turn names. It bears on attribution, on the process tree (I2) and on the order of risk (I3), and belongs in increment 4's log.
- **What became of 042's unknown turn is not recorded, and the fence does not gate the route the harness took** The Record says the failed turn "was run again" (docs/work-orders/042-build-tombstones-and-inventory.md:54). If that was a new request with a new key, OPS-012, OPS-014 and OPS-024 were never exercised, and an unknown invocation stays listed on worker-1's status under LIF-013. The first real consumer recovered by a new request, not by reconcile and repeat. That says the same-key repeat is not how a pipeline of model work recovers, which is finding 4-12's reconcile question again, closed with "v0 keeps `reconcile.v1` as it is".

## The verifier's notes

Overall: the analyst's facts are sound. Almost every count, quote and line I reopened held at 2eb8f258, with line drift of a few lines at most. The diagnosis of stage 1 (I1), the repository's blindness to A78 (I12), and the restart and guard findings (I9, I11) stand. The weaknesses are in the fixes and in who decides them.

Four things should change before these items are used:
- **I2's runner fix does not close the hole it describes.** A child that left the process group is invisible to "group seen empty". TERM, a grace period, then KILL, as `provider-local-process` already does, is the existing paradigm.
- **Un-poisoning a failed conversation is not the coordinator's before the S1 rerun.** It moved the fresh author's count by 128 lines, so ruling 71 point 4 excludes it.
- **The confinement plan has two unstated dependencies.** The engine on `main` cannot call a runtime provider, and this version of spec/ reports every restriction as mediated whatever the provider.
- **I3 re-asks the owner a question answered two days earlier.** Two of its four "new" points are in ruling 74's own record. Its cost is understated, and a cheaper timing exists at order 050.

I13's "evidence for the layered claim" is a selection effect, and its "convention 7 held up" has no record behind it. I10's least fix cannot read uncommitted work.

Coverage. Read in full: docs/guides/full-review.md; docs/implementation/HANDOVER.md; the records of rulings 61, 70, 71, 74 and 75; spec/conventions/implicit.yaml; the Record of order 042; AGENTS.md. Read in part: increment 4's log (all 20 rows, long cells cut at 1800 characters); OPS-012 to OPS-014, OPS-024, OPS-027 and OPS-034; IFC-010 and IFC-016; NAM-024; CTR-014; ACC-027; PRV-006 and PRV-008; DL3 and DL10; engine/CONFORMANCE.md lines 30-41; runner.rs and core requests.rs around the cited lines; order 050a's cited passages; WORK_ORDERS.md lines 40-160; docs/AUDIT.md's purpose, tests, layered claim and who-decides; ROADMAP.md's rows A77 to A82 and its experiment rules; the review list's items 15 to 29; rulings 72, 76 and 77 around the cited points. On the unmerged branch origin/wo/076-agents-v2-folder-grant (read with `git show`, not part of the pinned tree): lib.rs lines 170-647, contracts.yaml lines 1-115, README lines 95-175, conduit.yaml's head, main.rs by grep. Not read: the harness, the Fleet's documents and the private notes (none is in the repository); orders 076 and 078; the Claude package beyond its README and a diff of its contracts and source; the engine's tests. Nothing was built or run. Claims about what Codex, Claude Code, A2A, Jenkins, Nix or systemd do are from general knowledge, not checked here.
