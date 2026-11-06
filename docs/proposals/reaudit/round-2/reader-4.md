# Re-audit round 2, reader 4: the repository read cold

How I read it: with no prior context, at commit `2db5b6a` on 2026-11-06, starting at `README.md` and following its reading order (`spec/README.md`, `spec/definitions.md`, `docs/AUDIT.md`), then `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md` and `docs/OVERVIEW.md` in full, then by sampling: the three READMEs under `spec/`, four promise files in full (`first-start`, `pools`, `access`, and the promises the overview cites from thirteen other files), `spec/names.yaml`, `spec/implementation-defined.yaml`, `spec/core/requirements.yaml`, `spec/core/templates.yaml`, all seven valid example files, the first 40 lines of every Markdown file under `docs/`, the records `conventions`, `operations`, `reaudit` and `docs-audit` in full, and every experiment README. I did not read every promise, and I did not run `tools/check_doc_migration.py` or `cargo test`.

Checks, as run from the repository root before this file was written (each exited 0):

```text
python3 tools/check_docs.py          Errors: 0   (after "Checked 109 Markdown files and 1421 local links (offline).")
python3 tools/set_names.py --check   Names: corePackage x401, apiVersion x208, metaPrefix x63. Errors: 0
python3 tools/check_schemas.py       Errors: 0   (first line: "Python 3.9.6 at /Applications/Xcode.app/Contents/Developer/usr/bin/python3; PyYAML 6.0.1; jsonschema 4.20.0")
```

`git diff --check` also reports nothing, and `git status` is clean.

## A. The first ten minutes

What works. `README.md` answers most of the questions in 65 lines. What the project is and why: lines 3 to 16. What exists and what does not: the table at lines 22 to 27, which says plainly "It holds no engine" and "Does not exist". What binds: lines 29 to 39, one rule stated once ("The specification in spec/ alone binds. Nothing else in this repository does"). Where to start and in what order: lines 41 to 49. The commands for the checks are in the first document of the reading order (`spec/README.md:145-163`), and what not to touch is one hop away (`AGENTS.md:66-70`). "Promise", "conformance vector", "core package" and "engine" are each defined where they first appear. I found no statement in `README.md` that a later document contradicts. The problems are these.

1. **The word "audit" names five things, and the first use comes before any of them is explained.** `README.md:24`: "Three readers who were given only the specification have audited it once; the plan audit says what followed." Here "audited" is the re-audit and "plan audit" is the tracker, `docs/AUDIT.md`, which is not explained until `README.md:49`. A stranger then meets the consolidation audit, the re-audit, the docs audit, and at `docs/AUDIT.md:5` "an external audit of the plans", which is the audit the tracker is named after and whose text is nowhere in the repository (ruling 1, `docs/AUDIT.md:179`, begins "The audit is the basis for action items and priorities"). Smallest change: at `README.md:24` write "the tracker (docs/AUDIT.md, called the plan audit) says what followed", and add one sentence at `docs/AUDIT.md:5` saying who made the external audit and that its findings survive only as the Findings section.

2. **"Draft" and "accepted" are used of the same files with no sentence relating them.** `spec/README.md:3` says "Status: draft, begun 2026-11-05. Incomplete, and not a release", and every row of its Contents table (lines 35 to 82) says "Draft". `spec/core/README.md:3` and `spec/schemas/README.md:3` each open "Status: **accepted by the owner on 2026-11-05**". The tracker has the missing sentence, for its own word: `docs/AUDIT.md:55-56`, "ruled means decided by the owner and written as promises, with vectors still to come". A reader who goes from `spec/README.md` to `spec/core/README.md` cannot tell whether the core package is a draft or accepted. Smallest change: one sentence at `spec/README.md:3`, such as "Draft means that the owner has ruled on the content and it is written, that no vector tests it, and that it is not a release."

3. **The state of the work is one step behind.** `README.md:22` and `docs/AUDIT.md:71` both say "State on 2026-11-05". The repository's last two commits are dated 2026-11-06, `docs/proposals/reaudit/round-2/README.md:3` says "Status: in progress, 2026-11-06", and neither entry document says that a second round is under way. The third document of the reading order, the tracker, is stale in five places (section C). A stranger therefore learns the state as it was two commits ago.

4. **Two of the five families never appear in the front door.** `README.md:10-16` describes "agents, the shared things those agents use ... and the relationships between them". The words Interface and Pool do not occur in `README.md`, and `spec/README.md` does not define the families either. The first place a reader of the stated order learns what an Interface is is `spec/definitions.md:21`. The overview does not help: its row is "Interface | An actor: it makes requests on its own behalf" (`docs/OVERVIEW.md:28`), which does not say that it is where a person or an outside system acts. Smallest change: one clause in `README.md:10-12` naming all five, with the definition's words for Interface and Pool.

5. **"Type contracts" is used before it is defined.** `README.md:35`: "kept as source material for the type contracts still to be written". The definition is at `spec/README.md:24-29`. Minor; a five-word gloss would do.

6. **The stated order is about 1,150 lines before the plain-language tour.** Steps 1 to 3 are `spec/README.md` (193 lines), `spec/definitions.md` (286 lines of table rows, most citing several promise IDs) and `docs/AUDIT.md` (674 lines). The overview, 286 lines of prose with one worked example, is under "Then, as needed". A newcomer cannot do steps 1 to 3 in ten minutes, and can read `README.md` plus the overview in that time. This is a judgment about the path, not an error: the order given is the right one for someone who will work on the specification. Smallest change: a sentence under `README.md:43` saying that a reader who wants the idea first may read the overview before step 2.

## B. Consistency between the entry documents

What agrees. All seven documents give the same one-sentence description of Conduit. All say that `spec/` alone binds and that a behavior missing from it is a gap in it. The reading order is the same wherever it is stated (`README.md:45-49`, `AGENTS.md:10-16`, `CONTRIBUTING.md:7-10`, `docs/README.md:10-13`). The four-line checks block is identical in `AGENTS.md:80-85` and `CONTRIBUTING.md:78-83`.

Every number I counted is right except one (finding 5):

| Stated | Where | Counted |
| --- | --- | --- |
| 395 promises, and the count of each of the 22 files | `spec/README.md:37-84` | 395; every per-file count matches |
| Layers: none stated as totals, but "one holds a promise of the layer interface" and so on | `spec/README.md:99-103` | 212 language, 151 convention, 32 interface; `packages.yaml` holds the one interface promise outside `interface/`, and `implicit.yaml`, `lifecycle.yaml` and `interface/implementation.yaml` hold 4, 2 and 1 language promises |
| 14 implementation-defined items | `spec/README.md:82` | 14 (DL1 to DL14) |
| 7 valid files in 3 directories, 55 invalid, 11 invalid by rule | `spec/README.md:71-73` | 7 in 3, 55, 11 |
| 98 exports, 1 implementation, 45 schema contracts in 9 files, 20 + 5 + 6 + 2 + 11 + 2 + 7 in `core/` | `spec/README.md:52-70`, `spec/core/README.md:21-36` | Confirmed by `check_schemas.py`, step 5; the sums are 53 + 45 = 98 |
| 167 defects: 48, 39, 30, 50; of 87, 3 closed and 84 ruled; of 50, 49 on the list and 1 ruled | `docs/AUDIT.md:75` | 167 rows; 45 + 3 language, 39 convention, 30 type, 49 + 1 runtime |
| 37 rulings, 37 re-audit decisions, 26 follow-up decisions, 26 consolidation findings, 14 convention decisions, 13 docs-audit decisions, 19 behaviors and 6 dropped | `docs/AUDIT.md`, `docs/README.md:63-68` | All match (the re-audit record lists 38 rows; U11 says it is "not treated as a decision") |
| "Four checkers in tools/" | `README.md:27` | 4 files |

The disagreements:

1. **The list of checks differs between the specification and the build documents.** `spec/README.md:149-153` lists three tools, the first being `python3 tools/check_docs.py`. `docs/implementation/ENGINE_BUILD_BRIEF.md:32` says "The two checkers the specification refers to, check_schemas.py and set_names.py", and `docs/implementation/INDEPENDENT_BUILDS.md:58-59` says the same. On the build base `spec/README.md` would tell a builder to run a tool that is not shipped, or, if it is shipped, one that must fail, because "Files under spec/ also link to records under docs/, which the build base does not hold" (`INDEPENDENT_BUILDS.md:59-60`). Smallest change: say in `spec/README.md:155` that the first check is for the whole repository and not for a checkout that holds `spec/` alone.

2. **"The other two tools need only Python 3" is false for one of them.** `AGENTS.md:89` and `CONTRIBUTING.md:87`. `tools/check_docs.py:47-49` lists its files with `git ls-files`, so it needs Git and a Git working tree. Detail and the traceback are under F2.

3. **`AGENTS.md` uses "runtime" for the engine, which the definitions rule out.** `AGENTS.md:20`: "The language is the core and the runtime fulfills its promise." `spec/definitions.md:200` defines runtime as "Something long-lived that an instance keeps ... It is not the engine", and `AGENTS.md:44-46` tells contributors to write new text in the terms in force. `docs/AUDIT.md:148` has the same sentence, but there it quotes ruling 6. Smallest change: "the engine keeps its promises" in `AGENTS.md:20`.

4. **"The Python checkers in tools/ are the only exception" is not true of the tree.** `AGENTS.md:62-63` and `CONTRIBUTING.md:40-42`. `experiments/003-language-probes/probe.py` is tracked Python, and `README.md:27` lists it as code. The experiment's own README says so honestly (line 3: "This probe is Python and predates the Rust-only rule"). Smallest change: "the only exceptions are the Python checkers in tools/ and Experiment 003, which predates the rule".

5. **One stale count.** `docs/AUDIT.md:229`, in the table of later amendments: "The implementation-defined list has twelve entries. | Thirteen: ruling 32 added DL13." The list has fourteen: `spec/implementation-defined.yaml:75` is DL14, added by the conversion of the older text, and `spec/README.md:82` says "14 items".

6. **The tracker and the specification disagree on whether the docs-audit decisions are applied.** `docs/AUDIT.md:74`: "Decisions DA1 to DA13 are being applied (A61). Nineteen accepted behaviors that only the older text stated are being converted into promises". `spec/README.md:84-92`: the decisions of the docs audit, "which converted the behaviors that the older text accepted and the specification did not state, have been applied". The specification is right: commit `bdacd83` did it, and `docs/proposals/docs-audit/log-gaps.md` records 376 promises becoming 395.

7. **The readiness gate's first condition may be met while the tracker says none is.** `docs/AUDIT.md:76`: "None of G1 to G11 is met." G1 is "a contents list names every normative file and nothing normative lives outside it" (`INDEPENDENT_BUILDS.md`, readiness gate), and action A32, whose done-when is G1, says that what remained was the conversion that is now done (`docs/AUDIT.md:313`). Either G1 is met, or the one behavior left unconverted (secrets, GP3) is what holds it open. The tracker should say which.

## C. `docs/AUDIT.md` as the tracker

What works. What was decided is recoverable: 37 numbered rulings in order, a table of which later rulings amended which earlier ones (lines 217 to 231), an identifier table that disarms the reused letters (lines 13 to 50), and a link from each batch to its record. The counts in "Where things stand" are right apart from the one above. Historical sections are marked as such. The six behaviors that seem to have been dropped are put to the owner with links pinned to a commit (lines 400 to 412), which is exactly what a reviewer wants to see.

What is stale. The tracker was last changed in commit `70ed7b0`. Two commits have landed since: `bdacd83`, which converted the older text's behaviors into the specification, and `2db5b6a`, which opened round 2. Neither is reflected.

1. **`docs/AUDIT.md:74`, "Documents outside the specification".** "being applied" and "being converted" describe work that is present in the tree (B6). Should read: applied; eighteen of nineteen converted; GP3 waits for the owner.

2. **A32, `docs/AUDIT.md:313`, "in progress".** "What remains is the conversion of the accepted behaviors that only the older text stated". That conversion is done. What remains, if anything, is GP3.

3. **A61, `docs/AUDIT.md:329`, "in progress: audited; decisions DA1 to DA13 being applied".** The three logs under `docs/proposals/docs-audit/` record the edits made for them, file by file. The action's done-when has a second half, "a cold reader can tell from the entry points what binds", which this report answers yes to. The status should say that the decisions are applied and that the action waits on round 2.

4. **A59, `docs/AUDIT.md:327`, ends "Next: round 2".** Round 2 is in progress (`docs/proposals/reaudit/round-2/README.md:3`). The tracker does not mention it or link to it, and nor does anything else (E4).

5. **The later-amendments table, `docs/AUDIT.md:229`.** Thirteen items, where there are fourteen (B5).

6. **A11, `docs/AUDIT.md:332`, "open", and Q3, line 489, "open: A11".** The action is "State what the language requires of caller authentication, then choose the mechanism from the native probes." The first half is in the specification: IFC-022 ("The engine establishes which instance the consumer is for that request"), IFC-030, and item DL13 "Identifying consumers", from re-audit decisions U10 and F2. A bare "open" hides that. Suggested: "in progress: the requirement is IFC-022 and DL13; the mechanism waits for probe A4".

7. **A21, `docs/AUDIT.md:338`, cannot be evaluated.** Its done-when is "Each item under known inconsistencies is fixed or dropped". That section (lines 603 to 626) is headed "Historical ... Not maintained. Action A21 tracks what is left of it", and its items are not marked. Several are plainly overtaken: "Templates in Experiment 001 hard-code predicted instance names" (Experiment 001 is removed), "About 30 links point into Experiments 001 to 003 and will break" (the 001 links are now pinned to a commit). A reader cannot tell which of the eleven items A21 still covers. Smallest change: strike or mark each item, or close A21 and say why.

What I could not tell.

8. **What happens next.** There is no such paragraph. The experiment program gives an order of seven steps (lines 449 to 459) and does not say which step the work is at. From the statuses I infer: finish round 2, fix what it finds, run rounds until two are clean (A59), then vectors (A39) and the reference model (A38). The tracker should say this in three lines under "Where things stand".

9. **Who wrote the ruling log, and from what.** "All rulings are the owner's, given on 2026-11-05, in this order" (line 175). Thirty-seven rulings in one day, each a paragraph in the tracker's voice. A reviewer will want to know whether each row is the owner's words or an editor's summary of a conversation, and whether the owner has read the rows. One sentence would settle it.

10. **Things in the log I needed the records to decode.** Ruling 7 cites "PL1 and PL3 to PL8", and the identifier table (line 31) explains "PL-1 to PL-10": decisions and rules of one record differ only by a hyphen. Ruling 1 cites "the audit" (A1 above). Rulings 13 to 15 use declarations, plugin and grant, which the amendments table covers. Otherwise the log reads without having been present.

11. **The purpose section speaks in words the definitions retire.** `docs/AUDIT.md:109`: "**The grant is an object.**" `spec/definitions.md:284`: "grant | Not used." The note at lines 86 to 89 tells the reader how to translate, and the docs audit left the rewording to the owner. It remains the third thing a newcomer reads in the tracker, straight after the definitions told them the word is not used. I report it because a cold reader trips on it; the decision is the owner's.

## D. `docs/OVERVIEW.md` against the specification

The worked example is exact. Lines 119 to 140 are byte-identical to `spec/schemas/examples/valid/project/templates.yaml:72-93`, and lines 146 to 156 to `instances.yaml:41-51` (checked with `diff`). The four bullets under it hold: `rate-limit.v1` is a filter (`narrows: true` in `files-package/contracts.yaml:196`), `shared-host.v1` is the core requirement PLC-013 describes, and the instance is in the state registered until opened (LIF-002, INS-016).

Statements checked and supported, with the promise that supports each: one owner, a Pool, never changed (POOL-001); ownership permits nothing (ACC-028); one Resource per entry (POOL-002); a Resource made on its own (POOL-004); one placement for an Agent and a Resource, a location is a Resource owned by `system`, none for the other three (PLC-001, PLC-008, PLC-006); one consumer, one target, one capability (CON-001); a Conduit permits exactly what it lists (ACC-003); no listing Conduit, so refused (ACC-001); a Conduit document naming existing instances (CON-012); a restriction only narrows (ACC-008); the right to open is a Conduit to `access` (ACC-006); a bound names Pools, targets and operations (ACC-029); passed on only as it is or narrower (ACC-030); the first-start objects and the two Conduits of `root` (FST-001, FST-002); no access for another Interface (ACC-023); closing ends all access, never reopened (ACC-011, LIF-006); all or nothing for a submission (SUB-003); nothing set up or opened by registering (INS-016); one forward-only lifecycle, none for a template (LIF-001, LIF-006, LIF-012); the three states (LIF-002, LIF-003, LIF-005); unregistering (LIF-017); conditions beside the state (LIF-011); the three parts of an inspection (LIF-013); the access decision before any implementation (ACC-002); one invocation per caller, target and identity (OPS-008); nothing dispatched again without a repeat (IMP-007); a stale revision reports conflict (REV-002); model work only by a marked operation through a listing Conduit, never from setup (IMP-001, IMP-005); what is recorded (REC-001); one version and a digest per package (NAM-006); tools are the operations of open Conduits (IFC-012). That is thirty-one statements.

Statements the specification does not support as written:

1. **`docs/OVERVIEW.md:215-216`: "An unknown outcome stays unknown until an actor reconciles it (OPS-014)."** The specification makes this conditional. OPS-012: a repeat for an invocation whose outcome is unknown "is dispatched as a new attempt only if the operation's contract declares `deduplicates: true` and the implementation is not marked `plain`". OPS-013: the outcome stays unknown "and no new attempt at it is dispatched under OPS-012". So there are two ways out of unknown, and the overview gives one. The cited promise is also the wrong one: OPS-014 is reconciling; OPS-013 is "stays unknown". Smallest change: "stays unknown until an actor reconciles it, or until a repeat of an operation that deduplicates is dispatched as a new attempt (OPS-012 to OPS-014)".

2. **`docs/OVERVIEW.md:192-193`: "nothing removes it without an explicit request (OBS-004)."** OBS-004 has an exception: "other than an implementation restoring its own runtime, which OBS-005 governs". Smallest change: add "except that an implementation may keep its own runtime alive where its type allows (OBS-005)".

3. **`docs/OVERVIEW.md:95-96` leaves out the half of ACC-027 that a reviewer most needs.** The overview: "Inspecting a Conduit reports, for each restriction, whether it is enforced or only mediated". ACC-027 goes on: "In this version every restriction is reported as mediated", because "This version specifies no isolation plugin". As written, the overview reads as though enforcement exists. This bears on the security claim, so it is more than an omission. Smallest change: one sentence, "In this version nothing is enforced: every restriction is reported as mediated."

4. **`docs/OVERVIEW.md:45-46`: "Its document has `from`, which names the template, and `arguments`, which supply the inputs."** `arguments` may be left out (OBJ-013; `spec/definitions.md:35`, "may have `metadata` and `arguments`"), and the example project has instance documents with none (`instances.yaml:35-39` and `:55-61`). The sentence also omits the standard arguments (`owner`, `placement`, `consumer`, `target`), and the worked example sixty lines later depends on two of them. Smallest change: "has `from`, the standard arguments of its family such as `consumer` and `target`, and may have `arguments`".

5. **`docs/OVERVIEW.md:88-90` cites CON-002 for two things, and it states one.** "one Conduit is then made with each instance of that template, and opening it is a step of that instance's setup (CON-002)". CON-002 covers the first half. The second is GRA-022 and CON-004. Minor.

6. **`docs/OVERVIEW.md:28`: the Interface row does not say what an Interface is** (A4). The definitions do: "where a person, or a system outside, acts on the system".

7. **`docs/OVERVIEW.md:273-275` names two conformance levels where the specification has three.** `spec/README.md:131-140` adds the Implementation level, which the conversion commit introduced after the overview was last edited (`70ed7b0`, against `bdacd83`). The sentence begins "An engine claims", so it is not false, but a reader would conclude there are two.

None of these contradicts a promise outright except the first. The overview's header is honest about its date ("as they stood on 2026-11-05"); it should be re-dated when these are fixed.

## E. Documents that would mislead a stranger

What works. Every one of the seven topic files under `docs/protocol/` opens "**Superseded. Nothing in this file binds.**" and says what it is kept for. All thirteen proposal records carry "This is the record of a ruling. The specification in spec/ governs where they differ. Amended by:" with the rulings listed, and the two I read in full put a note beside each row a later ruling changed. The five files of `docs/proposals/authoring-syntax/`, `docs/authoring/README.md`, the three older files of `docs/implementation/`, `docs/REVIEW.md`, `docs/VOCABULARY.md` and the research packets each carry a banner that matches the text under it. This is thorough work, and the following are the exceptions.

1. **`experiments/002-runtime-plugins/README.md` is the weakest-marked document in the repository.** It is 559 lines (57 KB) of text that says "accepted" throughout. Its banner (lines 3 to 6) says only "Superseded as a plan", does not say that nothing in it binds, does not say that its words are the older ones, and ends "Its acceptance cases are mapped onto promises and conformance vectors and it is then removed", in the present tense, for work that is open (A23). Line 8 follows with "Status: first-proof layout, core authority/outage rules ... accepted". Line 12 has a link labelled "specification" whose target is `docs/OVERVIEW.md`, which is expressly not the specification. Smallest change: replace lines 3 to 6 with the banner used on the authoring-syntax files, and relabel the link "overview".

2. **`docs/archive/README.md` contradicts its own first line twice.** Line 3: "Nothing here binds." Lines 5 to 6: "the current review remains the review entry point and defers to it." Line 21: "Current review is the only active question queue." `docs/REVIEW.md:3` says "Superseded as a work queue by the plan audit", and `docs/AUDIT.md:7-8` says "It is the one tracker". Line 15 also says the two reviews are "linked to the active reference". Smallest change: delete the clause at lines 5 to 6 after the semicolon, and replace line 21's first sentence with "There is no active question queue here; the tracker is the plan audit."

3. **Six archive files state in the present tense that the protocol topics are the active authority or that the review queue is current, with no note on the file.** `docs/archive/collaborator-proposals/README.md:18` ("The protocol reference is authoritative; current review tracks outstanding work"), `collaborator-proposals/REVIEW.md:3-5`, `reviews/CONCEPTUAL.md:3-4` ("C1–C3 remain accepted in the active protocol reference"), `reviews/IMPLEMENTATION.md:3-4` ("Use current review for outstanding work"), `reviews/AUTHORING-2026-11-04.md:8-9` and `migrations/2026-11-protocol-framing.md:5-6` ("it is the only active queue"). Only the archive's index says these are superseded, and many documents link straight to the files. `AGENTS.md:66` forbids editing the archive, and the docs audit chose to mark the index alone (DA5). Research packets, which are also not to be edited, each got a one-line note. The same line on these six files would close the gap; if the owner prefers not to touch them, `AGENTS.md:40-43` should say that archive files carry no note of their own.

4. **Four documents that nothing links to.** `docs/proposals/reaudit/round-2/README.md`: no file in the repository contains the string `round-2` outside that directory. And the three logs `docs/proposals/docs-audit/log-entry-points.md`, `log-gaps.md` and `log-older-text.md`: the record's README links the two reports and neither the logs nor `GAPS.md`. `tools/check_docs.py` checks that links resolve, not that a document is reachable, so it does not catch this. Smallest change: link round 2 from `docs/AUDIT.md` (A59) and from `docs/proposals/reaudit/README.md`; add an "Applied" section to `docs/proposals/docs-audit/README.md` that links `GAPS.md` and the three logs.

5. **`docs/proposals/docs-audit/README.md` never says its decisions were applied.** Its table (lines 28 to 42) is headed "Decisions taken to fix it" and has no status. Its conclusions (lines 13 to 21) are in the present tense: "The repository is not ready for a cold read. The front door never sends a reader to spec/." That was true before commits `d159ba3` and `70ed7b0` and is false now. A stranger who lands here reads a current verdict.

6. **`docs/proposals/reaudit/README.md` reads as the state of the re-audit and is the record of round 1.** Its title says "round 1, decisions accepted". "What comes next" (lines 103 to 108) lists four steps as though pending; three are done and the fourth is under way. Line 220: "The specification now has 355 promises"; it has 395. Smallest change: retitle the section "What came next", mark each step, and add a line pointing at round 2.

7. **`AGENTS.md:71-74` gives every agent a standing instruction that belongs to one working session.** "The owner's standing instruction of 2026-11-05 is that this work is tracked on pull request #6: commit and push to its branch, docs/plan-audit, at each milestone." The same bullet begins "Commit or push only when asked". An outside reviewer's agent that reads this file as its instructions is told both, and the second is specific enough to act on. Smallest change: scope it ("For the sessions that write the language: ...") or move it to the tracker.

8. **`experiments/004-stateright-evaluation/README.md:3-4` cites a superseded document as the authority for the code.** "Exploratory Rust code under the exploratory-code ruling", linked to `docs/REVIEW.md`. The ruling is number 3 in the log of `docs/AUDIT.md`. Minor.

9. **The conventions record's list of amendments misses one.** `docs/proposals/conventions/README.md:51`, decision OL3: an instance may be unregistered "Only when it is retired and its shutdown is confirmed". LIF-017 adds "or it was retired because its location was declared lost" (re-audit decision A2, ruling 32). The "Amended by" entry for ruling 32 (lines 13 to 16) lists four changes and not this one. The record says the specification governs, so nothing binds wrongly; the list is simply incomplete.

10. **`docs/research/naming/README.md` is the one research packet without the "Not binding" line** (lines 1 to 5), and it calls the accepted naming record "the naming proposal". Minor.

## F. Hygiene and tooling

What works. All three checks pass and exit 0, and so does `git diff --check`. I exported the tracked files with `git archive` into an empty directory and ran the checks there: `set_names.py --check` and `check_schemas.py` pass on tracked files alone, so neither depends on anything untracked or ignored. There are no personal absolute paths, no TODO or FIXME markers, no temporary or editor files, and no `.DS_Store`. `git status` is clean, the branch `docs/plan-audit` is level with its remote, and the only ignored path is `experiments/004-stateright-evaluation/target/`. `.gitignore` is right. Experiment 004 has a lockfile and a pinned toolchain, as `AGENTS.md` requires. The last fifteen commits read as a coherent story, each subject line a full sentence saying what the commit did; the merge of pull request 5 keeps the outside reviewer's commit under their name.

1. **`tools/check_docs.py` fails with a traceback outside a Git checkout, and the documents say it needs only Python 3.** In the exported tree:

   ```text
   fatal: not a git repository (or any of the parent directories): .git
   subprocess.CalledProcessError: Command '['git', 'ls-files', '--cached', '--others', '--exclude-standard', '-z']' returned non-zero exit status 128.
   ```

   A reviewer who downloads the repository as an archive cannot run the first check. `AGENTS.md:89` and `CONTRIBUTING.md:87`: "The other two tools need only Python 3." Smallest change: correct the sentence to say that `check_docs.py` needs Git and a checkout. Better: have the tool fall back to walking the tree when `git ls-files` fails.

2. **`check_schemas.py` passed here because of what happens to be installed on this machine.** The `python3` on the path (Homebrew 3.13.2) has neither PyYAML nor jsonschema. The tool re-ran itself under `/usr/bin/python3`, which is Xcode's Python 3.9.6 with PyYAML 6.0.1 and jsonschema 4.20.0, and says so in its first line. The documents describe this correctly. But on a machine where neither interpreter has the packages the tool stops with "NOTHING WAS CHECKED" and exit 2 (`tools/check_schemas.py:73-83`; read, not run), and no document gives the install command or the versions it was last run with. `experiments/003-language-probes/requirements.txt` pins its dependencies; `tools/` has nothing. The fallback path is also specific to macOS. Smallest change: one line in the Checks sections, "python3 -m pip install pyyaml jsonschema (run with PyYAML 6.0.1 and jsonschema 4.20.0)".

3. **About 770 KB of superseded text against 550 KB of specification.** `docs/protocol/` is 266 KB, `docs/proposals/authoring-syntax/` 321 KB (`AUTHORING.md` alone is 138 KB), `docs/implementation/` 161 KB of which all but two files are superseded, and `experiments/002-runtime-plugins/README.md` 57 KB. Each is marked (but see E1) and each is kept for a stated reason, so this is not an error. It is the main cost a stranger pays: a text search for any term returns more superseded hits than binding ones. Nothing to change today; worth a ruling on when the protocol topics and the authoring-syntax packet move to the archive.

4. **Two details of the owner's machine and organization are in entry documents.** `AGENTS.md:68-70`: "the sibling directories ../agentwatch and ../pi-reference ... reference clones on the owner's machine". `docs/AUDIT.md:659` names a private repository and says "Its links will likely fail for readers outside that organization". The docs audit left the second to the owner. Neither breaks anything.

5. **Nothing tells a stranger that `tools/check_doc_migration.py` is safe to skip.** `README.md:27` counts it among "Four checkers", and `CONTRIBUTING.md:105-115` explains it at length, including that it "cannot fail on later changes" and needs full Git history. It is not in the checks block, which is right. One clause in `README.md:27` ("three that are run before each commit, and the record of one past migration") would stop a newcomer wondering why the fourth is not listed.

6. **No file tracked by Git looks accidental.** `CLAUDE.md` is eleven bytes that include `AGENTS.md`, which is deliberate. The three JSON files under `docs/research/` and `docs/archive/` are retrieval and migration records that their documents cite.

## G. The top problems, ranked

1. **Bring the tracker up to the last two commits.** `docs/AUDIT.md` lines 74, 229, 313, 327 and 329: decisions DA1 to DA13 applied, eighteen of nineteen behaviors converted, fourteen items on the implementation-defined list, round 2 in progress with a link to `proposals/reaudit/round-2/README.md`. Change "State on 2026-11-05" at line 71 and at `README.md:22` to the day it is done. This is the third document every reader is sent to, and it is the one stale thing they are certain to hit.

2. **Make round 2 and the docs-audit logs reachable.** Link `round-2/README.md` from `docs/AUDIT.md` (A59) and `docs/proposals/reaudit/README.md`; in `docs/proposals/docs-audit/README.md` add a short section saying the decisions are applied and linking `GAPS.md` and the three logs.

3. **Correct what the documents say the checks need.** `AGENTS.md:87-89` and `CONTRIBUTING.md:85-87`: `check_docs.py` needs Git and a checkout; give the install line and versions for PyYAML and jsonschema.

4. **Fix three sentences in the overview.** `docs/OVERVIEW.md:215-216` (unknown outcomes; OPS-012 and OPS-013), `:192-193` (the exception in OBS-004), `:95-96` (every restriction is mediated in this version). Then the smaller ones: the Interface row at line 28, `arguments` at lines 45 to 46, the third conformance level at lines 273 to 275.

5. **Give `experiments/002-runtime-plugins/README.md` the standard banner** and relabel the link at line 12 that calls the overview the specification.

6. **Remove the two present-tense claims in `docs/archive/README.md`** (lines 5 to 6 and 21), and decide whether the six archive files that call the protocol topics the active reference get a one-line note or `AGENTS.md` says they have none.

7. **Scope or move the standing instruction at `AGENTS.md:71-74`** so that an outside agent is not told to commit and push to pull request 6.

8. **Add "What happens next" to the tracker,** three lines under "Where things stand": the rounds still to run, what follows two clean rounds, and which step of the experiment program's order the work is at. In the same edit: mark the items of "Known inconsistencies" or close A21; note on A11 that IFC-022 and DL13 state the requirement; say whether G1 is met.

9. **Say once how "draft" relates to "accepted".** One sentence at `spec/README.md:3`. In the same file, line 155: say that `check_docs.py` is for the whole repository, so that the build brief's "two checkers" and the specification's three agree.

10. **Disarm "audit" and introduce all five families in `README.md`.** At line 24 name the tracker as the tracker; at lines 10 to 12 add Interface and Pool in the definitions' words; at `docs/AUDIT.md:5` say what the external audit was. In `AGENTS.md:20` write "engine" for "runtime", and in `AGENTS.md:62-63` and `CONTRIBUTING.md:40-42` admit Experiment 003 as the second exception.

## Is it ready to be handed to an outside reviewer today?

Nearly, and not quite. The things that are hard to get right are right: one statement of what binds, repeated without variation in seven entry documents; one reading order; every count in the specification's contents table and in the tracker's defect summary correct when counted; three checks that pass; every superseded topic, record and research packet I sampled under a banner that matches its text; an overview whose example is copied exactly from a checked file; a clean tree and a readable history. What blocks it is narrow and is all outside `spec/`. The tracker, which is the third document every reader is told to read, is two commits behind: it describes as under way work that the specification says is done, gives a stale count, and does not mention the round of review now in progress, whose own page nothing links to. A reviewer who arrives from an archive download cannot run the first check, and the documents say they can. And the overview makes three statements that the promises qualify, one of which, on enforcement, a reviewer assessing the security claim would be misled by. Items 1 to 4 of section G are a few hours of editing; with those done I would hand it over, and I would do items 5 to 7 in the same pass because each is a single small edit.
