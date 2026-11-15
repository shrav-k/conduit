# Full review of 2026-11-14

Answered on 2026-11-14 by [ruling 78](../ruling-78/README.md): every question as recommended.

Not binding. The [specification](../../../spec/README.md) alone binds, and [IN_FORCE.md](../../../IN_FORCE.md)
holds every other decision in force. This is the full review the owner asked for at the handover of
2026-11-12 ([the brief](../../guides/full-review.md)), carried out by the coordinator on 2026-11-14 on
commit `2eb8f258` (2026-11-12), read only. Its scope is the whole repository and the progress so far:
faults in the language, the approach, and the experiments to alter. It decides nothing by itself:
the owner's answers to [its questions](questions.md) become one ruling, and until then every
decision in force stands. What the coordinator decides alone is logged in [decisions.md](decisions.md)
for the owner to strike.

| Part | What it holds |
| --- | --- |
| [questions.md](questions.md) | The 17 questions for the owner, each with lettered options, a recommendation, what it changes and what waits on it; then what is not asked, and what is asked later |
| [decisions.md](decisions.md) | The coordinator's decisions D1 to D88 (the approach, the pipeline, the experiments) and fixes F1 to F144 (the language), each logged to strike |
| [orders.md](orders.md) | The work orders the review adds, in the order they are dispatched |
| [experiments.md](experiments.md) | The changes to the register's rows, each with its claim, pass condition and measures |
| [findings/](findings/README.md) | All 574 findings and items, [FR-001](findings/L01-access.md#fr-001) to [FR-574](findings/GL7-in-force-rest.md#fr-574), each with its adversarial verifier's verdict |

## In short

The core is sound and worth keeping: access as open Conduits decided per request, a forward-only lifecycle, providers behind contracts modelled on existing tools. The largest group of the 44 unsafe findings, of 263, sits where the engine cannot know what happened, and the main fixes remove machinery. The newest text has no vectors, binding rules hide in `observable`, and no standing rule limits growth. The pipeline produces and records well, but a clean verifier is not proof of done. The second lane worked beside the path to the S1 rerun, one line can decide S1's bar, and v0 has no supportable date. Model checking paid best, but most scenario conditions put no claim at risk and the falsifier has not run.

## The questions

Answer each by number and letter; "as recommended" may cover any or all. Questions 1 to 4 concern
the approach and the experiments, 5 to 17 the language; the three marked unsafe (5, 9 and 10) close a way in which
an engine that keeps every promise could still do harm. Nothing on the path to the S1 rerun waits on
questions 3, 10, 11, 12, 13, 15 or 17; question 16 decides only whether order 043b, on that path, builds `manages:`.

1. [The order of the work until the S1 rerun's build is pinned](questions.md#1-the-order-of-the-work-until-the-s1-reruns-build-is-pinned). Recommended: 1(b) and 2(a), one coordinator until the pin, and the pin waits for the six sign-in orders.
2. [S1's bar](questions.md#2-s1s-bar). Recommended: (b), both runs must take fewer lines than the baseline on setup and on change and repair; T3 and the per-task limit drop out.
3. [v0's gate and scope](questions.md#3-v0s-gate-and-scope). Recommended: 3.1 (b), 3.2 (a), 3.3 (a): features closed at today's rulings; cuts modelled where a promise spans two machines; the corrected claim drafted for publication, closing A20.
4. [One amendment to the accepted plans of S2 to S10](questions.md#4-one-amendment-to-the-accepted-plans-of-s2-to-s10). Recommended: (a), accept the table: the coordinator picks S2's server, S3 cuts with the Mac's firewall, S7 splits; nothing is dropped.
5. [May a reconciliation store a control's values or make an instance ready on an actor's word?](questions.md#5-may-a-reconciliation-store-a-controls-values-or-make-an-instance-ready-on-an-actors-word-unsafe) (unsafe). Recommended: A, a reconciliation only records the outcome and what was checked; to change anything, send the control or setup again.
6. [Keep the fence of ruling 75 A1, or drop the record "did not occur"](questions.md#6-keep-the-fence-of-ruling-75-a1-or-drop-the-record-did-not-occur). Recommended: C, drop the fence and "did not occur"; an unknown stays unknown until recorded as having occurred.
7. [What a call that changes nothing reports when no answer comes](questions.md#7-what-a-call-that-changes-nothing-reports-when-no-answer-comes). Recommended: A, it ends failed, says that no answer came, and a repeat is sent.
8. [What a setup, retirement or cascade reports after a restart in its middle](questions.md#8-what-a-setup-retirement-or-cascade-reports-after-a-restart-in-its-middle). Recommended: A, the outcome its parts give as stored, as a cancellation already does.
9. [What the engine's own panel is, and who may see it](questions.md#9-what-the-engines-own-panel-is-and-who-may-see-it-unsafe) (unsafe). Recommended: A, the engine's own terminal or window, never returned to a request; only root's person sees it in v0.
10. [What an actor other than root may do with install.v1](questions.md#10-what-an-actor-other-than-root-may-do-with-installv1-unsafe) (unsafe). Recommended: A, nothing in v0: install.v1 is reserved to root.
11. [Whether closing a secret's Conduit stops the engine's own use of the value](questions.md#11-whether-closing-a-secrets-conduit-stops-the-engines-own-use-of-the-value). Recommended: A, yes: the next request reads again, is refused and reports unavailable.
12. [A kept store under a changed built-in core package](questions.md#12-a-kept-store-under-a-changed-built-in-core-package). Recommended: C, at each start the engine brings root's own core-made Conduits up to the built-in lists, recorded; no other list changes.
13. [Upgrading or swapping the machine provider](questions.md#13-upgrading-or-swapping-the-machine-provider). Recommended: B, bind each node to the provider's Type, so a same-Type replacement finds its nodes again.
14. [Telling the user that a consumer runs inside the engine's domain of trust](questions.md#14-telling-the-user-that-a-consumer-runs-inside-the-engines-domain-of-trust). Recommended: B, each run's status says whether it is inside the engine's domain of trust.
15. [The shared conventions: say what the layer holds, or move rules out](questions.md#15-the-shared-conventions-say-what-the-layer-holds-or-move-rules-out). Recommended: A, correct the claim's words to what the layer holds; move no promise.
16. [Cutting surface that nothing uses](questions.md#16-cutting-surface-that-nothing-uses). Recommended: A, cut handles, `manages:` and the provider's label echo; the labels once the engine matches them itself.
17. [Four of the coordinator's fixes that change something you accepted](questions.md#17-four-of-the-coordinators-fixes-that-change-something-you-accepted). Recommended: (a), accept all four: the store fails closed for every request, `serve` sets nothing up at start, `local` is not put to a provider, and the writing rules say which text binds.

Also asked later, not now: what changing a running Agent may cost (finding 4-01), after the S1 rerun
with the bar; whether an Agent's own requests during a turn answer to the turn's caller, with ruling
62 C6 to C12 after the rerun; and whether a Resource may start model work, with batch 2's text
([the list](questions.md#asked-later-not-now)).

## What the review determined

### The language

1. **The core is sound; keep it.** Every permission is an open Conduit, each request is decided anew, the lifecycle only moves forward, and runtime providers follow the Container Runtime Interface, so a builder can follow most rules without asking; neither the schemas nor the core operations drew an unsafe finding ([FR-113](findings/L10-core-operations.md#fr-113) to [FR-120](findings/L10-core-operations.md#fr-120), [FR-132](findings/L12-schemas.md#fr-132) to [FR-139](findings/L12-schemas.md#fr-139)). What the text says of the Model Context Protocol matches the adopted revision closely, and the one wide gap is that earlier revisions, which Codex still speaks, are admitted while the promises describe only the adopted revision's mechanisms ([FR-539](findings/GL1-protocol.md#fr-539) to [FR-545](findings/GL1-protocol.md#fr-545)).

2. **The unsafe findings gather where the engine cannot know what happened.** The first pass marked 44 of its 263 findings unsafe: 27 confirmed, 16 weakened and 1 already known, and on all 16 weakened ones the verifiers removed or narrowed the mark. About 19 of the 44 concern a lost answer or a run that must be stopped, and about 13 of those are confirmed, several of them found twice: a reconciliation that stores old values or opens a Conduit on an actor's word, a fence (the evidence that a lost call can no longer arrive, needed before it is recorded as failed) that trusts what the engine cannot see, a resend after an error, and repeats that a server cannot tell apart ([FR-010](findings/L02-lifecycle.md#fr-010), [FR-011](findings/L02-lifecycle.md#fr-011), [FR-022](findings/L03-operations.md#fr-022), [FR-033](findings/L04-interface.md#fr-033), [FR-180](findings/X03-model-checking.md#fr-180), [FR-181](findings/X03-model-checking.md#fr-181), [FR-217](findings/X07-retries-fence.md#fr-217) to [FR-221](findings/X07-retries-fence.md#fr-221), [FR-231](findings/X08-one-run.md#fr-231), [FR-253](findings/X10-queued-and-kept.md#fr-253)).

3. **So the risk lies most of all in the machinery for uncertainty, and the main fixes remove machinery rather than add it.** The core of connection and access drew few unsafe findings, the clearest being what a bound that names a Pool covers, found twice ([FR-001](findings/L01-access.md#fr-001), [FR-121](findings/L11-core-rest.md#fr-121)); the coordinator's fix has the engine store each of its own acts whole or not at all, as a database transaction does ([FR-556](findings/GL3-restart-mid-act.md#fr-556), [FR-557](findings/GL3-restart-mid-act.md#fr-557)), and, if the owner agrees, a reconciliation will only record what happened ([FR-010](findings/L02-lifecycle.md#fr-010)). The second pass's four unsafe marks add no new unsafe fault: two repeat first-pass faults, and the verifiers took the mark off the other two ([FR-548](findings/GL2-vectors.md#fr-548), [FR-549](findings/GL2-vectors.md#fr-549), [FR-557](findings/GL3-restart-mid-act.md#fr-557), [FR-568](findings/GL5-stated-limits.md#fr-568)).

4. **The newest parts are the least checked.** Nine more of the 44 sit in sign-in, secrets, enrollment, install and "the engine's own panel", which is where login codes and machine fingerprints are shown, although no promise defines it ([FR-048](findings/L05-providers.md#fr-048), [FR-062](findings/L06-secrets-pending.md#fr-062) to [FR-065](findings/L06-secrets-pending.md#fr-065), [FR-100](findings/L09-packages.md#fr-100), [FR-141](findings/L13-framing.md#fr-141), [FR-192](findings/X04-stated-limits.md#fr-192), [FR-193](findings/X04-stated-limits.md#fr-193)). The files for pending actions and sign-in (13 promises), enrollment (12) and the short form (15), all written on 2026-11-11, have no vector at all.

5. **The form of a promise hides what binds.** The `observable` field holds three times the words of `then` (69,486 against 22,622) and carries rules stated nowhere else, one of which keeps a browser login's return out of every result, yet nothing says whether it binds ([FR-140](findings/L13-framing.md#fr-140), [FR-153](findings/X01-catalogue-layers.md#fr-153), [FR-441](findings/E05-falsifier.md#fr-441)). One rule can be written in up to five places, and the copies already differ ([FR-116](findings/L10-core-operations.md#fr-116), [FR-009](findings/L01-access.md#fr-009), [FR-032](findings/L03-operations.md#fr-032), [FR-131](findings/L11-core-rest.md#fr-131)).

6. **The text grows, and no standing rule limits it.** Since 2026-11-06 promises rose from 434 to 519 and `then` text from 13,517 to 22,622 words, and since the ruled rewrite of 2026-11-07, 67 promises were added and none removed ([FR-470](findings/G2-size-brake.md#fr-470), [FR-473](findings/G2-size-brake.md#fr-473)). The growth is in the convention layer (179 to 245, against 220 to 226 for the language layer), mostly in topic files that rulings placed there, since the seven conventions' own files grew only from 127 to 132, so "the few behaviors every part must agree on" no longer describes that layer, though the seven conventions themselves stay few ([FR-145](findings/L13-framing.md#fr-145), [FR-160](findings/X01-catalogue-layers.md#fr-160), [FR-440](findings/E05-falsifier.md#fr-440)).

7. **The record of decisions drifts where it restates.** `IN_FORCE.md` is a sound index into `spec/`, and the second pass found the lines it checked, for the older rulings and five topics, faithful apart from five small differences, none unsafe; but about 25 lines restate the text instead of pointing to it, and that is where it drifts, some lines weaker than the owner's words ([FR-167](findings/X02-inforce-vs-spec.md#fr-167) to [FR-179](findings/X02-inforce-vs-spec.md#fr-179), [FR-570](findings/GL7-in-force-rest.md#fr-570) to [FR-574](findings/GL7-in-force-rest.md#fr-574)). Work-order Records keep history well but track open items badly: one unsafe finding was held nowhere but a Record ([FR-253](findings/X10-queued-and-kept.md#fr-253)).

### The approach

1. **The pipeline produces a lot and records it well; keep it.** In four days 125 orders were written and 116 merged to main, each with a Record an auditor can read ([FR-267](findings/P01-work-orders-lanes.md#fr-267)). The Codex verifier is a productive reader: the Records credit it with 246 faults in merged orders, counted as must-level although the Records of orders 020 to 066 give no severity, and with a fault in most first passes ([FR-491](findings/G4-who-verifies.md#fr-491), [FR-495](findings/G4-who-verifies.md#fr-495)).

2. **A clean verifier is not proof that an order is done.** A second fresh verifier pass found new must-level faults in 16 of 31 lane orders, about seven orders showed a defect after a clean final verifier, fifteen Records say the verifier could not build, and 17 of the coordinator's 25 own fixes merged unread, 6 of the 8 that were read needing more work ([FR-264](findings/P01-work-orders-lanes.md#fr-264), [FR-494](findings/G4-who-verifies.md#fr-494), [FR-496](findings/G4-who-verifies.md#fr-496)). A verifier on another model closes only one of three gaps, because most escapes need what only running things shows (CI that gates, with a macOS leg) or a read of the coordinator's own text; smaller orders are what make rounds end ([FR-268](findings/P01-work-orders-lanes.md#fr-268), [FR-491](findings/G4-who-verifies.md#fr-491), [FR-493](findings/G4-who-verifies.md#fr-493)).

3. **The second lane added output beside the path to the S1 rerun, not on it.** Merged orders a day rose from 26.8 to 56.8, many of them paper or finishing orders, and its model checking was the best gain of those days; but engine merges a day fell from 11.6 to 5.3, the engine calls none of the lane's providers, none has passed a contract case, and three incidents reached main through shared refs and CI ([FR-265](findings/P01-work-orders-lanes.md#fr-265), [FR-303](findings/P04-providers-built.md#fr-303), [FR-360](findings/P08-progress.md#fr-360), [FR-368](findings/P08-progress.md#fr-368)). The Records show neither that the lane slowed the coordinator nor that it brought the rerun closer, and it stopped with the handover.

4. **Dogfooding has tested how a turn is carried, not what Conduit is for.** In order 042 Conduit decided only that root may send turns to two Agents, as ruling 70 intended, and five of the six frictions found fall where the layered claim puts them ([FR-331](findings/P06-dogfooding.md#fr-331), [FR-343](findings/P06-dogfooding.md#fr-343)). It still found a real fault: in a smoke test, Codex's child processes outlived the run's stop, which matters for the fence ([FR-332](findings/P06-dogfooding.md#fr-332)).

5. **S1's bar caught a real fault, but now one line can decide it.** It exposed a canonical form that cost 5 and 10 times the baseline's 64 lines and led to the short form; but under ruling 71 point 3, ruling 76's forecast passes on exactly four tasks, the least that passes, so one more line on T5 fails a run, the same project in block layout fails, and the fresh author's only short-form example is S1 itself ([FR-278](findings/P02-s1-bar.md#fr-278), [FR-281](findings/P02-s1-bar.md#fr-281), [FR-289](findings/P02-s1-bar.md#fr-289), [FR-366](findings/P08-progress.md#fr-366)). Neither author's mistakes concerned lines, though both met volume as the first cost ([FR-277](findings/P02-s1-bar.md#fr-277), [FR-282](findings/P02-s1-bar.md#fr-282)).

6. **v0 is further off than the amount of work suggests.** There are real assets: an engine that ran S1 end to end with real Claude Code and Codex Agents, 519 promises and 494 vectors. But none of the ten v0 capabilities the review checked is on main, one of ruling 47's six rows is met, 18 orders on the path to the S1 rerun are unwritten (12 by the batch plan's numbering), and at least 41 of the coordinator's engine, specification and vector orders lie between the pin and the gate, so no date for v0 can be supported ([FR-344](findings/P07-engine-build.md#fr-344), [FR-364](findings/P08-progress.md#fr-364), [FR-365](findings/P08-progress.md#fr-365)).

7. **The means of the work and the dogfood evidence live outside the tree.** The specification, the engine, the models and the baseline can be checked from the tree, but not the order runners (13 lines of prose), the dogfood harness, A78's plan and measurements, or the engine's own records of order 042's run, which sit in a store due to be initialized again ([FR-525](findings/G6-outside-the-tree.md#fr-525), [FR-527](findings/G6-outside-the-tree.md#fr-527), [FR-528](findings/G6-outside-the-tree.md#fr-528), [FR-537](findings/G6-outside-the-tree.md#fr-537)). No Record from 037 to 150 names the worker's model, and this review's own first pass was kept only in temporary space until this record took it in ([FR-535](findings/G6-outside-the-tree.md#fr-535)). One item says the tree records no close for the keys printed on 2026-11-11 ([FR-530](findings/G6-outside-the-tree.md#fr-530)); it does: ruling 75's record holds your answer, so nothing is asked again.

### The experiments

1. **Model checking paid best; keep it for rules whose safety depends on the order of events.** Twelve orders in one day found eight faults in the promises with seconds of machine time ([FR-455](findings/G1a-model-checking-register.md#fr-455)). But "holds" means only that a rule, as modelled, survived an exhaustive search of one small scripted setup, usually with one fault at a time and for safety only: 79 of the 114 rows marked as holding never saw a network cut, and no model has run since five later specification commits, among them orders 077, 081 and 082, changed at least 34 rows, 49 since the text the first five models read ([FR-180](findings/X03-model-checking.md#fr-180) to [FR-191](findings/X03-model-checking.md#fr-191), [FR-447](findings/G1a-model-checking-register.md#fr-447), [FR-448](findings/G1a-model-checking-register.md#fr-448)).

2. **Many rows marked as holding could not have failed.** The second pass read four model files: of their 33 such rows, 24 rest on a property the search cannot refute, such as a constant or a value the model copied from its own expectation, and none of MC7b's 14 properties relates two steps ([FR-456](findings/G1b-model-properties.md#fr-456), [FR-458](findings/G1b-model-properties.md#fr-458)). Of the 42 static rows with a vector, 13 have no step that would fail if the promise broke, and the second pass recommends grading every row before any model runs again ([FR-462](findings/G1b-model-properties.md#fr-462), [FR-469](findings/G1b-model-properties.md#fr-469)).

3. **The vectors are good evidence for document verdicts and weak elsewhere.** They caught a real regression when order 042 merged, a fault of the merge and not of the text. But 99 of the 292 promises they cite are named in no step, up to 59 cases still pin error forms that newer promises forbid, a lag that order 046 is to close, so until then an engine that keeps the promises fails them, a wrong own-tools report passes all 494, and 338 cases start from a snapshot in the first build's own shape, so the engine is its own oracle and a second engine could not run them as they stand ([FR-345](findings/P07-engine-build.md#fr-345), [FR-546](findings/GL2-vectors.md#fr-546), [FR-547](findings/GL2-vectors.md#fr-547), [FR-550](findings/GL2-vectors.md#fr-550)).

4. **The pre-registrations are honest, but most of their conditions put no claim at risk.** S2 to S10 were accepted before any run, state their limits and are unchanged since except for a link, but they register things to do and record, not claims with a result that would refute them ([FR-401](findings/E02-s5-s6-s7.md#fr-401), [FR-502](findings/G5-round6-evidence.md#fr-502)). S2 cannot fail on its purpose-level claim, S3's owed-stop condition passes with a provider that stops nothing, S7 can pass with its claim false, S10's accepted swap allows the very edits that the roadmap's words "the same documents" would count as failure, and at most 6 of the 23 measures in S2 to S7 can be counted by a third party as written ([FR-378](findings/E01-s2-s3-s4.md#fr-378), [FR-380](findings/E01-s2-s3-s4.md#fr-380), [FR-393](findings/E02-s5-s6-s7.md#fr-393), [FR-406](findings/E03-s8-s9-s10.md#fr-406), [FR-504](findings/G5-round6-evidence.md#fr-504)).

5. **The falsifier has not run.** Six cold-reading rounds found faults every time, but nobody has scored whether `spec/` alone fixes what documents mean; the chosen test, in which fresh readers predict held-out verdicts from `spec/` alone, is now the coordinator's row A83, needs no owner time, and runs after the two specification orders for the unsafe text ([FR-436](findings/E05-falsifier.md#fr-436), [FR-438](findings/E05-falsifier.md#fr-438), [FR-439](findings/E05-falsifier.md#fr-439)). Its verdict part, as registered, names 56 of the 519 promises, so a pass only suggests, and a failure comes to the owner with options because no consequence is fixed in advance; nor did model checking answer the partition candidate, since in the one partition model the cut case broke the one-run rule and became a stated limit, so S3 stays required ([FR-442](findings/E05-falsifier.md#fr-442), [FR-447](findings/G1a-model-checking-register.md#fr-447)).

6. **The probes of the riskiest new parts have produced nothing yet.** The second machine has no experimental result though about 6,700 lines of SSH provider code and tests exist, no provider has passed a contract case, and sign-in is written in full but not built ([FR-303](findings/P04-providers-built.md#fr-303), [FR-321](findings/P05-second-machine.md#fr-321)). Under one operating-system user a sign-in test cannot show that one Agent's login is safe from another, and each result must say so ([FR-422](findings/E04-signin-isolation.md#fr-422)).

## What the coordinator does without asking

Each is logged in [decisions.md](decisions.md) for the owner to strike; a struck item is withdrawn.

- **Verification (D1 to D5, D10).** A script runs every check before anyone reads. One fresh reader on the most capable model reads every order. A Codex reader adds a second view on risky orders and reads every coordinator fix. Nothing merges unread, and dispatch stops first when the Claude allowance runs low.
- **The path to the pin (D15 to D17).** Order 071 finishes first. 049a1, 080 and 144 wait for the pin. Two specification orders fix the unsafe text before batch 1 builds on it.
- **The S1 rerun (D19 to D25).** There is one pre-registration, in order 053. Every measure the review adds is report-only, and one script counts both sides. Batch 2 merges after your judgment, pass or fail.
- **Model checking (D28 to D35, D40).** "Holds" is kept for properties that can fail: on four models, the 114 rows fall to at most 86. The recount and MC2 on the fence's new text come before the pin. Every model finding ends in a vector or a stated limit. A new promise enters `spec/` only with a case that can fail.
- **Providers (D42 to D45, D48).** SSH-R's drainer is removed. An owed stop is still carried out before any start, and otherwise the run's own machine ends the run. The packaged local doubles are frozen, no further provider is built until one runs end to end, and the kit's fixed lifetime becomes a renewed lease. Whether a remote run must survive an engine restart comes to you after A74, or before another provider is built on that gate.
- **Dogfooding (D54 to D58).** A78 stays on the core chain, with its overhead measured. The dogfood engine's stop faults are fixed before the rerun. The store's records are exported before order 050 replaces the store, and the harness never runs code from a slot.
- **Experiments (D64, D80, D81).** Each accepted scenario gets a dated amendments section, with one row on your review list. A5 becomes a staged sign-in test that gates nothing. A blind read of `spec/` is registered and comes to you only if it fails.
- **Documents and changed decisions (D84, D44, D45, D67, D88).** The empty shortcut table is filled. Three logged decisions change: order 145's drainer, the packaged local providers and S4's T1 repair. Review-list item 18, which you confirmed, stands.

**The language.** The coordinator makes 144 fixes to the language, each logged for you to strike. By table: access and refusals 10, operations, outcomes and reconciliation 21, lifecycle and revisions 14, the interface 16, providers, runs and machines 18, secrets, pending actions and sign-in 21, objects, naming, schemas and packages 22, the form, the layers, definitions and the limits 16, IN_FORCE.md 6.

Twenty of them are unsafe:
- F1: A Pool named in a bound covers that Pool alone.
- F11: The engine's own work is stored whole, and reconciled opens and setups make their checks.
- F12: Only evidence that no copy is still in flight fences an attempt (withdrawn under question 6's B or the recommended C; its conflict refusal is an interim fix asked with question 6).
- F13: A later attempt's failed does not stand while an earlier attempt is unanswered (an interim fix, asked with question 6).
- F14: A repeat is recognized by the invocation's ID, not by the key alone.
- F15: The engine fails closed when it cannot write its store.
- F16: Competing changes are decided against the state they read.
- F17: What a restore from an older copy undoes is stated as a limit.
- F32: `expectedId` on two more verbs, and a revision never repeats under one name.
- F46: After a revision error, only a call that changes nothing is sent again.
- F47: A handed-over directory is registered for an established actor.
- F61: A run is its whole process tree.
- F62: No restart follows a held stop that was carried out.
- F63: A runtime provider is told when an instance is gone.
- F64: A hostname is free again only once its retired node is unregistered.
- F65: Three failed fingerprint tries close the enrollment.
- F79: A sign-in action's expiry lifts no hold.
- F80: The sign-in holds cover the instance, a Resource included.
- F81: A sign-in that starts from another account raises the hold.
- F122: What an observable says binds, and the rule on a login's return moves into binding text.

## The work orders

[orders.md](orders.md) lists them in order, under the recommended answer to question 1. The first five:

- **#1 The review's record enters the tree**: both passes, scrubbed, before session scratch can be emptied at a restart ([FR-535](findings/G6-outside-the-tree.md#fr-535)).
- **#2 Private names and secrets checked**: four more path patterns and a secret scan in CI, before any report or export is committed ([FR-534](findings/G6-outside-the-tree.md#fr-534)).
- **#3 The engine's CI shards stand alone**: engine tests stop waiting on package legs, and CI builds every crate ([FR-493](findings/G4-who-verifies.md#fr-493), [FR-271](findings/P01-work-orders-lanes.md#fr-271)).
- **#4 Who verifies**: your preference of 2026-11-14 recorded as a later answer under ruling 48, and the pipeline's documents brought to it ([FR-500](findings/G4-who-verifies.md#fr-500)).
- **#5 Records for the five unmerged branches**: so that 071 can resume from its last report and merge first ([FR-529](findings/G6-outside-the-tree.md#fr-529)).

## Changes to the experiments

[experiments.md](experiments.md) holds each row in full.

- **A83, new (026):** blind verdicts from spec/ alone. It is a new row that feeds A45 and A59; it is not A45's pilot narrowed. It includes the two checks the language critic asked for (resolving by hand, and placing layers blind) and runs on corrected text. Coordinator; whether it runs before or after the pin follows question 1.
- **A84, new (029):** plan and apply from the 60 existing requests, after order 051a and before 052a-1. Coordinator.
- **Experiment 012:** reopened as in progress. The recount and MC2 run before the pin, and the rest is placed around it. "Holds" counts only for properties the search could refute. Coordinator.
- **A40:** status and roadmap wording corrected now. The narrowing and the done-when (option A) are yours in question 3.
- **A44 (031):** mutation testing keyed by promise, on the vectors alone, with the first pass at the pin. Coordinator.
- **A5 (027):** the staged sign-in test that ruling 49 already defines. It gates nothing, and stage 2 needs about 30 to 40 owner minutes. Coordinator.
- **A6:** one fact left: does a client resend after a lost response? Twelve runs on a new small stub; S4's replay stays as ruled. Coordinator.
- **A7 (030):** three regimes and a fixed list of attempts; the coordinator drafts the claim. Closing D5 (the tracker's question on v0's authorization scope, not decision D5) and A20 on it, and the G8 scope, are yours ([FR-482](findings/G3-native-probes.md#fr-482)).
- **A10:** closed as done; the paths to wake move to A7.
- **A22:** the keychain in the dedicated user's sessions, run when that user is created. Coordinator, with your approval.
- **A73 (025):** desk check now; a run only if a 1Password store is ordered. Your S10 answer stands. Coordinator.
- **A74:** observational, with hypotheses aimed at the kit's gate and at units under a cut; S3's stage 0. Coordinator.
- **A77:** a pass condition over 20 orders, plus blind paired reads of one diff. Coordinator; you are told.
- **A78 (028):** one merged row: bounded overhead, a records export per order, and a Claude verifier slot at order 050. Coordinator; what orders 4 and 5 test stays yours.
- **A81 (024):** a replay by hand, before any return code is written. Coordinator.
- **A82:** two legs (the listener and the paste) and the placement line. Coordinator.
- **A70:** a second baseline, not run before the rerun and report-only if run later. Coordinator.
- **Experiment 022:** a pass condition per suite, with cases written before the providers are read. Coordinator.
- **S1 rerun:** report-only measures in order 053's plan. Coordinator.
- **Round 6's triggers (E07, E08, and measures D09 and D20):** measures so that your triggers can fire. Coordinator.
- **S2 to S10:** dated, labelled tightenings only, including S9's fix for setup at start. The rest is in question 4.

## The findings

Every finding and item is in [the findings](findings/README.md), one part per reader or analyst,
each with the adversarial verifier's verdict and corrections; where a verifier corrected
something, the correction governs. IDs run from [FR-001](findings/L01-access.md#fr-001) to [FR-574](findings/GL7-in-force-rest.md#fr-574): [FR-001](findings/L01-access.md#fr-001) to [FR-263](findings/X10-queued-and-kept.md#fr-263) are the first
pass on the language, [FR-264](findings/P01-work-orders-lanes.md#fr-264) to [FR-445](findings/E05-falsifier.md#fr-445) the first pass on the approach and the experiments, and
[FR-446](findings/G1a-model-checking-register.md#fr-446) to [FR-574](findings/GL7-in-force-rest.md#fr-574) the second pass.

The language, both passes (299 findings):

| Class | Confirmed | Weakened | Already known | Refuted | All |
| --- | ---: | ---: | ---: | ---: | ---: |
| Defect | 39 | 21 | 4 | 0 | 64 |
| Ambiguity | 37 | 17 | 1 | 2 | 57 |
| Gap | 61 | 39 | 4 | 0 | 104 |
| Contradiction | 22 | 18 | 3 | 0 | 43 |
| Needless complexity | 12 | 17 | 2 | 0 | 31 |
| All | 171 | 112 | 14 | 2 | 299 |

48 were marked unsafe by their reader: by a verifier's verdict, 27 confirmed, 18 weakened and 3
already known. A weakened finding is real but smaller, or of another class, than its reader said;
several verifiers narrowed or removed the unsafe mark in their corrections. 53 were marked as the
owner's to decide; the questions above carry those that survived the skeptics, and the rest are
answered, deferred or the coordinator's, as each question's notes say.

The approach and the experiments, both passes (275 items): 78 supported and 197 supported in part
by their verifiers, none unsupported. A "supported in part" item holds in its facts but not in all
of its conclusion or its numbers; the questions and decisions above are written from the
verifiers' corrections.

## What was not checked


- **Nothing was built, and no vector, model or engine test was run.** The passes read and counted with read-only scripts, and one reader also put the examples through a schema validator in memory. The first proposed runs are the MC2 rerun on order 077's text and a probe of the two client facts that order 050 needs, both before the pin; the blind-verdict read (A83) follows the two specification orders for the unsafe text.
- **`spec/definitions.md` row by row.** About 25 of its roughly 330 rows were checked, and some contradict the promises ([FR-149](findings/L13-framing.md#fr-149), [FR-150](findings/L13-framing.md#fr-150)). The coordinator's fix corrects those rows and adds a checker for undefined terms, but the sweep of every row has no order yet; it suits a mechanical, read-only work order with a fresh verifier.
- **The core contracts, member by member.** No pass checked the 37 core operations and `spec/core/results.yaml` against every promise that names one of their members, and the mismatches found so far were met by chance. It has no order yet and suits the same kind of mechanical work order.
- **The layer of every promise, by two readers.** Goes to the blind-verdict read, part L: two readers classify all 519 promises without seeing the recorded layer.
- **The example project resolved by hand, and the examples nobody read.** People opened at most about 35 of the 130 files in `spec/schemas/examples`. Goes to the blind-verdict read: in part R two readers resolve the valid project and first start, and part E covers the 41 by-rule files and three valid ones.
- **Most model properties.** The 81 holds rows of the operations, access, placement, lifecycle and controls models were not graded (access only in part). The grading the second pass asks for is done at each model's next run, not before it, as the verifier of [FR-469](findings/G1b-model-properties.md#fr-469) advised (decision D31, order #6 and experiment 012's registrations): each row is registered against its quoted unsafe slice, and every mutant is also run with interleaving faults off, so a property refuted on that one schedule loses "holds". No one reads the 81 rows by hand first.
- **Most vectors.** Apart from the 60 sampled cases and those opened for the 42 static rows ([FR-462](findings/G1b-model-properties.md#fr-462)), the 494 cases were seen only through counts across the whole suite. They go to the mutation-testing pass on the pinned tag (A44), the audit of the cases that name the 42 static rows, and the 121 cases of the blind-verdict read.
- **Older rulings, item by item.** Rulings 23, 28, 29, 31, 32, 33 and 37 were compared with `IN_FORCE.md` only through the ledger and each record's decision table. Not yet assigned.
- **Outside texts not fetched.** The protocol's earlier revisions, which existing servers and Codex still speak, were judged only from the adopted revision's own account; the SPIFFE standards were compared from memory, and the RFCs read through summaries. What holds at those revisions is written down as the first build does it, fix F144 ([FR-539](findings/GL1-protocol.md#fr-539)); only dropping them from v0 would be the owner's, and no one proposes it.
- **The allowances.** Nothing measures the Claude or Codex allowance that limits the order of the work. A77's revised row records the coordinator's spend per order, but nothing yet measures the Codex allowance.

## How the review was run

The brief ([full-review.md](../../guides/full-review.md)) set three focuses: faults in the language,
the approach, and the experiments to alter. The coordinator ran it as three multi-agent workflows on
a read-only checkout of `2eb8f258`. Every reader and analyst was followed by an adversarial verifier
that reopened each cited line, tried to refute each finding from the text, searched the records for
a ruling or a known point that already settles it, and checked who decides. A completeness critic
closed each focus and named what no one had covered.

| Pass | What it did | Agents | Result |
| --- | --- | --- | --- |
| First pass, the language | 13 readers read every file of `spec/` in slices; 10 more read across it (the catalogue and its layers, IN_FORCE.md against `spec/`, model checking, the stated limits, the short form, authority, retries and the fence, one run, records and loss, the queued items) | 23 readers, 23 verifiers, 1 critic | 263 findings: 165 confirmed, 91 weakened, 7 already known, none refuted |
| First pass, the approach and the experiments | 8 analysts on the approach (work orders and lanes, S1's bar, plan and apply, the providers, the second machine, dogfooding, the engine, progress) and 5 on the experiments (S2 to S10, sign-in and isolation, the falsifier) | 13 analysts, 13 verifiers, 1 critic | 182 items: 55 supported, 127 supported in part, none unsupported |
| Second pass | 13 analysts on what the critics found uncovered (model checking twice, a brake on the language's size, the native probes, who verifies, round 6's evidence against the proposed cuts, what is outside the tree, the protocol and adopted standards, a sample of 60 vectors, a restart in the middle of each engine act, a second Resource of a core Type, the stated limits, the rest of IN_FORCE.md), each with a verifier; then 12 drafters merged the owner's decisions into the questions below, each challenged by a skeptic; 5 more wrote the register rows, the map of the coordinator's logged decisions and the fix orders | 55 | See each section |
| The record | 8 writers drafted this record's sections from the verified passes, each followed by an adversarial checker; then 5 more checkers read the assembled record (consistency across its files, fidelity to the rulings, the front page's facts, whether the owner can answer from the text alone, and the public-repository rules), and the coordinator applied their edits | 21 | This record |

Models. The first pass and most of the second ran on Claude Fable 5.2, the owner's choice for
review work. When that model's allowance ran out on 2026-11-14 the owner asked for the review to be
finished on Claude Opus 5.5, which ran the rest of the second pass (its verifiers and skeptics, and
the drafts and lists that had not finished) and the record's writers and checkers. Each part of [the findings](findings/README.md) says
which pass it came from.

Rules every agent kept: the reviewed tree left unchanged; nothing built or run, except that the protocol reader fetched the
published Model Context Protocol, its server.json schema and four RFCs, its verifier fetched pages of the protocol again, and one
reader put the examples through a schema validator in memory; no environment printed and no credential read. Six agents also
wrote small helper files to their own session's scratch space, outside the tree and against the rule to create nothing; each says
so in its part (L10, L13, P01, P03, X02 and X03), and none of those files is in this record.
