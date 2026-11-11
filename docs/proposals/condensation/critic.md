Not binding. The critic of the condensation plan, kept as returned; its execution issues are the coordinator's to handle in the work orders.

severity major

Sound in shape and mostly accurate. I spot-checked its line ranges, word counts, file counts and the check_roadmap rules against HEAD a98ffe0, and nearly all of them hold. It is not safe to run as written, for five reasons. (1) It proposes "ruling 57", but ruling 57 is already recorded at HEAD. That ruling already settles the plan's point 1 and leaves open only the form of ruling records and how duplicated content is removed. (2) The meaning it gives the manifest's safety_rows tags is false, so the new E15 check would fail from WO-030 onward. (3) Open items in the removed editor logs are never checked; only choices are carried. (4) WO-031 links to anchors and files that later orders create, so check_docs fails. (5) The line numbers will drift at C0, and PR 14 conflicts with the AUDIT restructure. It is also too timid on secrets and multi-machine: about 52k and 24k words of reasoning stay, each said twice. Each of these is fixable without changing the plan's architecture.

MAJOR
1. Ruling 57 already exists. It is commit a98ffe0, row 57 at docs/AUDIT.md:611, and its record is docs/proposals/ruling-57/README.md. The owner said: "I revoke the rule saying you cannot edit those docs". The ruling sets the relevance standard and records "Open: the form of ruling records, and how duplicated content is removed". The plan's Step 0 would put "ruling 57" again and overwrite that record. Its point 1, options (b) and (c), asks again a question the owner has answered.
   Fix:
   - Renumber the plan's ruling to 58.
   - Drop point 1. Present its extra limits ("surviving text is not reworded", the never-removed list, condensing in place) as the coordinator's procedure under ruling 57, or as a narrowing for the owner to confirm. They are stricter than the ruling, which says documents "may be edited, condensed or removed like any other document that does not bind".
   - Move the standing condense-at-close rule into its own point, since it answers 57's open question on duplicated content.
   - Renumber every "Accepted by ruling 57", "logged before ruling 57" and "amendments accepted by ruling 57" (E1, E2, E20, E24).
   - Write "57 or 58 one-line rows", not 56. Add later-amendment rows for 45-57: ruling 57 itself amends ruling 39 point 11 and the AGENTS rule.
   - The AGENTS.md and CONTRIBUTING.md edits that ruling 57 requires ("What follows") are still not done. The AGENTS.md:110-112 rule still loads into every worker's context. Do them now, before C0, not in WO-031.

2. The safety_rows premise is false (conformance/manifest.yaml; E15, E36).
   - The manifest uses 21 distinct values: registration (83), snapshot-context (61), S1-plain-ends (44), archive-installation (40), first-start*, loader-* and s1, among others.
   - Only S1-plain-ends, S1-key-scope, S1-reconcile and S1-unknown-write are trace names in increment-2/plan.md §3 (:551-584).
   - safety_rows is defined in no README, schema or decision. simplification/research/map-constraints.md:22-23 says it is "my reconstruction" and that "the specification has no safety field".
   - Consequences:
     - E15's check ("every safety_rows value names a row of s1-safety-cases.md") fails on about 17 values and about 240 tags in WO-030. check_schemas then fails in every later order, and "report it; do not fix it" conflicts with step (e).
     - E36's claims that the file "is the meaning of the tags", and the list behind draft ruling 47's "every safety vector passes" and A68's done-when, decide something nobody has decided.
   - Fix: copy §3 as the proposed traces only. Record the tag mismatch under Known to be open and A79. Add the check only after the tags are reconciled, and have the owner or coordinator decide what "safety vector" means.

3. No sweep of open items in the logs that are removed.
   - E35 carries only "choices". A settled-check exists only for open-decisions "Left for round 5" (E17), core-and-schemas "left undone" (E20) and the secrets drafts (E18).
   - Logs on the removal list have their own open or needs-a-decision sections:
     - ruling-41: https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-core.md#L208, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G1.md#L209, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G2.md#L228 and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G2.md#L238, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G3.md#L202, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G4.md#L170, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G5.md#L120, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G6.md#L203, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G7.md#L151, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-integration.md#L150 and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-integration.md#L342.
     - ruling-42: https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-conventions.md#L119, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-core.md#L154, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-integration.md#L157 and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-integration.md#L310, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-language.md#L133, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-machines.md#L144, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-secrets.md#L107.
     - ruling-43: https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-core.md#L185, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-integration.md#L192 and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-integration.md#L374, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-interface.md#L156, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-providers.md#L138, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-secrets.md#L150.
     - [docs-audit/log-gaps:164](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/log-gaps.md#L164), twelve or more items "each point below needs a decision". Examples: GQ7 (what a request naming a nonexistent object reports), GP9, GP15, GQ10, GP4, GP17.
     - log-increment-1:695-706 holds more items than the two extracted.
   - AUDIT:226 says only "Most are settled since".
   - Fix: a verifier maps every item under these headings to its settlement (ruling, record or promise), or carries it to "Known to be open". Run this before each deletion.

4. WO-031 links to things later orders create, so check_docs fails. WO-031 writes AUDIT and ruling-log.md links to:
   - ruling-41/README#planners-choices, created by E16 in WO-034;
   - closeout-results#how-to-run-the-open-checks, created by E8 in WO-035;
   - conformance/README "Planned work" and "Evidence of vector strength so far", created by E13 in WO-035 (A44's Source and A79);
   - the measurement-plan addendum, created by E11 in WO-036 (the Waiting row "points here");
   - language-shape/project/brief.md, created by E23 in WO-037.
   E10 (WO-035) also cites WORK_ORDERS record rows that do not exist until E30 (WO-040).
   Fix: create these destinations, or at least their headings, in WO-030 or WO-031, or link today's locations and repoint them later.

5. Line numbers will drift at C0. They were computed at a98ffe0, but C0 is the later merge of ruling 58, which adds an AUDIT row and edits AGENTS and CONTRIBUTING.
   - Every cited AUDIT span after line 611 shifts: :612-697, :698-721, :755-775.
   - So do AGENTS lines after 110 and CONTRIBUTING lines after :56.
   - "Copy, do not paraphrase", checked against C0 with the same numbers, will not catch a wrong span.
   - Fix: cite by heading plus first and last sentence, or recompute every span at C0 when the orders are written.

6. PR 14 conflicts with the AUDIT restructure. It is branch docs/organization-publication-plan, based on 171e4cf, 47 commits behind main. It adds:
   - a long AUDIT row for ruling 47;
   - a later-amendments row for ruling 40;
   - a new AUDIT section "Organization and publication after v0" (A75, A76);
   - a "Publication" row in the ROADMAP order;
   - the removal of the "Held for an open pull request" line;
   - edits to CONTRIBUTING and docs/README.
   The sequence never mentions it. After WO-031 its diff no longer applies. Either merge or rebase PR 14 before C0, or give rebase instructions: a one-line row, and the amendment in ruling-log.md. E2's new Waiting row "How v0 is released (REPOSITORY §5)" may duplicate what PR 14 decides.

MODERATE
7. Ruling 38 point 11. The owner said: "Ok, note this for codex to look at in the audit". The note is "To look at in particular (ruling 38, point 11)", AUDIT:268-286. The plan keeps point 4's rule verbatim in AUDIT for exactly that reason, but moves point 11's note to cold-readers.md and a stub. Its claim to change "neither ruling 38 point 4 nor 39 point 15" leaves point 11 out. Keep the note in AUDIT, or list it as amended.

8. Too timid on secrets and multi-machine.
   - Secrets: secrets/README is condensed only to about 29k words of 39.4k, and docs/research/SECRETS.md (23.3k) is kept whole. That is about 52k words of secrets reasoning.
   - Multi-machine: proposal 14.7k plus research/MULTI_MACHINE.md 10.8k, about 25k.
   - These are the largest remaining historical texts, and each pair duplicates itself, which is the open question ruling 57 left.
   - The "spec names it" constraint (spec/README:550) is one the plan imposes on itself, since WO-032 rewrites that paragraph anyway.
   - Suggest: keep, for each S-question, the options, the recommendation and why each alternative was rejected (about 8-10k). Remove the research packets with archive rows once the proposals carry their precedents.

9. Ruling 57's standard is "preserved … in spec/, or in a live document". For research packets that feed contracts not yet written, the plan keeps only an archive row pointing to C0:
   - CONDUITS:795-833 (L151-L154);
   - ESTABLISHMENT (L85, L86, L92, L96);
   - PI_RUNTIME (L77, L79, L81, L87, L94);
   - PI_CONNECTORS;
   - OBSERVABILITY §3.
   Either extract their conclusions into LANGUAGE_DEFECTS or AUDIT rows, as E2 does for A66 and A50, or have the owner confirm that pinned source-material rows are enough.

10. Known to be open leaves things out.
   - AUDIT:236-252 still holds open remainders:
     - MAN-010 repeated-argument binding and the isRepeated gap (round-6/DECISIONS; waits on S7 and A63);
     - PLC-024's portable record field (A39);
     - OPS-028, to revisit with A65.
   - AUDIT:189-203 holds scope statements (later work is not in v0; policies and composites are not in v0) that need a home in Scope guard or Current direction.
   - Do not assert that log-increment-1's patternProperties item is still open; check it against the core's own matching (1-04).

11. The AUDIT size of 7.3k is not reachable. The 37 open action-item rows alone are about 3.96k words (AUDIT:790-862). Adding Identifiers, Purpose (674), Layered claim (874), Current direction (439), about 1.7k of ruling one-liners, Known to be open (900), Who decides what (600) and Waiting gives about 11-12k. The read-first path then comes to about 50k, not 46k, unless the action-item status notes are cut.

12. Sequencing with ongoing work.
   - Increment 4 is open. The freeze plus twelve sequential PRs touch conformance/README, reference-model/README, FIRST_BUILD, log-increment-4 (E9), the review list, AUDIT and ROADMAP. The plan does not say whether increment-4 orders pause, and it takes order numbers 030-041.
   - E35 is a judgement over about 170k words of logs with a 20% sample. Split WO-034 by ruling.
   - Steps (a) and (b) are commits, which workers may not make; the admin makes them, and squash merges flatten them.

MINOR
- s1_drafts.rs cannot be "pointed at reference-model/examples/s1". The test asserts [agents, filesystem, history, project], and examples/s1 is a single project. conformance/fixtures/s1 holds copies that differ (filesystem and history conduit.yaml). Remove the test.
- The new path-mention check will report false positives:
  - check_roadmap.py:22 and :24 (`experiments/NNN-name/`, and `experiments/006-across-machines/`, a number held for a directory not yet made);
  - JSON pointers in core/src/schemas.rs:728 and :739.
  It also needs to know about check_schemas.py:187-215, which names nine more log paths than the :83 and :221 the plan lists. Give the check an allow-list.
- The retired-ID reuse check in E15 is scheduled in WO-030, but the registers it reads are created in WO-032.
- Permalink check:
  - Four existing links at 0a9a323 (spec/README) point to a commit that is not an ancestor of main. A fresh clone lacks it, so treat a missing object as a notice.
  - C0 must be the squash commit on origin/main.
- 005: "The next run writes dated output" means changing probe.rs:799. That contradicts "they do not change" and needs cargo test.
- stale-init.json is under packages/, so step (e) must run cargo test in packages/agent-claude-code.
- E27 counts are wrong:
  - LANGUAGE_DEFECTS has 18 closed rows (13+5), not 23.
  - The 74 ruled rows are language and convention rows, not type-layer rows.
  - The plan's figures sum to 172, not 167.
- E1 copies the AUDIT:545-552 preamble verbatim, and it is stale ("ruling 49 on 2026-11-10 … 47 still reserved"). Only the labelling sentence must stay verbatim.
- WORK_ORDERS.md:131-134 moved each record into its order's file because parallel orders conflicted (002 and 004). E30's standing rule puts records back into one file. Keep the per-file Record while an order is in flight; the admin writes the row serially when the file is removed.
- ruling-45/log-roadmap "The coordinator's decisions" 1-10, with reasons, are missing from E35's sources, and AUDIT "Who decides what" links them. Add them to ruling-45/README.
- The AGENTS.md PR-history cut to "two sentences" must keep:
  - the standing instruction (ruling 36/38: sessions writing the language commit and push to the PR branch at each milestone);
  - squash merges with a message written by hand;
  - order branches named wo/NNN-name.
- Step 13's "skeptic items (decisions 1-10, rationale 1-12, evidence 1-10, links 1-12)" are not written anywhere the cold reader can see. Commit them with the order.
- The admin's auto-memory, outside the repository, cites increment-3-handoff.md and other paths that will be removed. Update it after the pass, since it loads into every admin session.

Verified as correct: AUTHORING :77-119 and :790-798; AUDIT :165-186 and :396-404; the D19 numbers; 027 Record :81-85; packages/agent-codex/README:94; the lib.rs:145 and manifest:6883 dependencies; A79 being the next free number; the AGENTS word count (2,115); 980 experiment files; results.txt and results-rerun.txt being byte-identical; the current promise form check passing (455 promises, no duplicates); and project_digest using canonical JSON, so WO-032's comment-only edits change no digest.
