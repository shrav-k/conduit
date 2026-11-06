# Round 4, reader 4: the whole repository, read cold

Read at commit `f583469` on 2026-11-06, on the branch `docs/plan-audit`, with a clean working
tree when the read began. Paths are relative to the root of the repository. Nothing was edited
but this file.

## Summary

Twenty-six findings.

| Severity | Count | Findings |
| --- | --- | --- |
| `misleading` | 8 | 4D-1, 4D-2, 4D-3, 4D-4, 4D-5, 4D-17, 4D-18, 4D-19 |
| `missing` | 5 | 4D-6, 4D-7, 4D-8, 4D-11, 4D-12 |
| `stale` | 5 | 4D-9, 4D-13, 4D-14, 4D-16, 4D-21 |
| `hygiene` | 2 | 4D-22, 4D-23 |
| `minor` | 6 | 4D-10, 4D-15, 4D-20, 4D-24, 4D-25, 4D-26 |

The entry documents still do their main job. From `README.md` alone a stranger learns what the
project is, that only `spec/` binds, that there is no engine, no reference model and no
conformance vector, and that a fourth round of the re-audit is in progress. Every count in
`spec/README.md` is exact. The findings fall in four groups:

- The tracker's new section on who decides what draws a cleaner line than the records do.
  The records show choices that no ruling covers before ruling 37, and choices of editors and
  of readers whose reports are not kept after it. Three entry documents still credit to the
  owner what the records give to the coordinator (4D-1 to 4D-7).
- The list "Waiting for the owner" misses several things that the records leave for the
  owner, and three of its pointers do not land on what they claim (4D-8 to 4D-11).
- Round 3 left no record of what was done about its fourth reader's findings outside `spec/`,
  and a few statuses and counts are behind (4D-12 to 4D-16).
- `docs/OVERVIEW.md` omits three things without which a reader comes away with a false
  picture of who applies a restriction, of what limits the root Interface, and of whether the
  engine ever dispatches for no actor (4D-17 to 4D-21).

Read in full: `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`,
`docs/OVERVIEW.md`, `docs/AUDIT.md`, `spec/README.md`, `spec/definitions.md`,
`spec/names.yaml`, the record of round 1
(`docs/proposals/reaudit/README.md`), the `README.md` and `DECISIONS.md` of rounds 2 and 3,
the `README.md` of round 4, `log-reader-4-fixes.md` of round 2, the report of the fourth
reader of round 3, `docs/proposals/docs-audit/README.md` and `GAPS.md`, the two build
documents under `docs/implementation/`, `docs/protocol/README.md`, `docs/archive/README.md`,
`experiments/README.md` and the README of Experiment 004.

Sampled: about 70 promises by identifier, printed from the YAML (every promise the overview
cites, and those on restrictions, the root Interface and later work); the head comment of four
files under `spec/`; `spec/schemas/README.md` apart from its table of what a schema cannot
express; the first third of `spec/core/README.md` and a search of the rest for status words;
the status header of every record under `docs/proposals/`; the first lines of every Markdown
file under `docs/` and `experiments/`, for its note; the result, choices and
read-through sections of `log-closing.md`, `log-tail.md` and `log-fixes.md`; the summary and
the corrections of `mcp-citations.md`; the head of `docs/LANGUAGE_DEFECTS.md`; the docstrings
of the tools; `git log` for dates, addresses and trailers.

Not read: the bodies of the protocol topics, the research packets, the archived files, the
authoring-syntax packet and the two architecture documents; the logs of round 1 beyond their
counts; the body of the core-and-schemas record; most promise files end to end; the schemas.

Not verified: `cargo test --release` for Experiment 004 was not run, because it writes build
output. Nothing was checked against the Model Context Protocol. Of the external addresses,
five on the hosting service were asked for their status, pull request 6 was queried, and no
other was fetched. The files `mcp-recheck.md` and `reader-3.md` appeared in this directory
while I was reading; they are not part of this read.

## The four checks

Run from the root, in the order `AGENTS.md` gives, before this file existed. Each exited with
status 0.

```text
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 128 Markdown files and 1629 local links (offline).
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x436, apiVersion x234, metaPrefix x96. Errors: 0

$ python3 tools/check_schemas.py
Python 3.9.6 at <path of the system interpreter>; PyYAML 6.0.1; jsonschema 4.20.0
Profile: 27 keywords, 5 formats. Format checkers available for all of them.
1. Read 105 files holding 231 documents as strict YAML.
2. Checked 58 Schema contracts against the profile, and every `$ref`.
3. Validated 138 documents under spec/schemas/, spec/core/ and examples/valid/.
   (61 lines, one for each invalid example, each beginning "rejected")
4. 61 invalid documents in 61 files were each rejected by a schema, or rejected when read.
5. Core package: 99 documents exported, 0 documents deliberately private and 1 implementation,
   which is also an export. Checked 439 references for the kind they name; 1 name packages
   that are not in this repository and were not checked. Checked 3 controls, the 5 operations
   that take no `revision`, the tables of the two READMEs, and 852 citations of promises in
   108 files. The example packages keep 0 documents private.
   (20 lines, one for each set under invalid-by-rule, each beginning "rejected")
6. Applied 19 rules of this tool, which come from 17 promises, to the core package and 3
   example directories. 20 sets of documents in 20 files under examples/invalid-by-rule/ were
   each rejected by the rule they are named for; each of their 33 documents is valid against
   the schemas.
   SCH-019: validated 10 defaults of the core package and the valid examples against their
   schemas; 0 were not validated, because a `$ref` in the schema leads outside this repository.
Errors: 0

$ git diff --check
(no output)
```

The first line of `check_schemas.py` prints the absolute path of the interpreter it ran under.
It is elided above. The lines of steps 5 and 6 are wrapped here; the tool prints each as one
line. `python3 tools/check_doc_migration.py`, the record of one past migration, was also run:
it printed "Errors: 0" and exited with status 0.

## Findings

### Who decided what

**4D-1. `misleading`. The tracker says that until ruling 37 every change was the owner's
ruling. The records show changes that no ruling covers.**

- Where: `docs/AUDIT.md:105`, under "Who decides what since ruling 37"; `README.md:41-43`.
- Quote: "Until ruling 37 every change of accepted behavior was a ruling of the owner, logged
  below."
- What the records show, each before ruling 37:
  `docs/proposals/reaudit/README.md:39`, of about 115 findings, "Fixed in the consolidation
  pass with no new ruling"; `:226`, "coordinator then made three further changes", one of
  which is that a Pool template declares no Conduits; `:256`, of five pairs of promises, "each
  was corrected without a new one". `docs/proposals/reaudit/CONSOLIDATION.md:124` has a
  section "Choices made by the coordinator while planning", the first of which gives every
  invocation an ID of the engine's own. `docs/proposals/consolidation/README.md:63-64`: "the
  owner did not rule on them one by one". `docs/proposals/core-and-schemas/README.md:431-432`,
  of the choices made while ruling 37 itself was applied: "each is for the owner to strike or
  keep." The tracker's own status of A56 says the same of the consolidation choices.
- Why it misleads: a reviewer takes it that the 376 promises of ruling 37 hold nothing but
  what the owner ruled. They also hold choices of the coordinator and of editors, logged and
  not ruled on one by one.
- Least change: reword the sentence to say that until ruling 37 every decision put to the
  owner was ruled by the owner, and that the passes which applied the rulings made choices of
  their own, with links to the five places above; and add those places to "Waiting for the
  owner".

**4D-2. `misleading`. The tracker names two deciders since ruling 37. The records show a
third, and readers whose reports are not kept.**

- Where: `docs/AUDIT.md:112-115` and the table at `:125-127`.
- Quote: "Every promise added or changed since then [...] rests on such a decision"; of an
  editor, "decides nothing that needs a new decision"; of a cold reader, "writes a report."
- What the records show: `docs/proposals/reaudit/round-2/log-closing.md:104-106` has a table
  "Choices made where a reader gave options or no wording", headed "Each can be struck by the
  owner." It holds the editor's own choices, among them the four promises that the pass added
  (TPL-013, PKG-026, PKG-027, PKG-028) and the reasons that several requests report.
  `round-2/DECISIONS.md:108-109`: "The choices the closing pass made itself are in its log
  and stand unless the owner strikes them." The same log, at `:14`, says "Two further
  readers, who were given the text and not this log, read it cold", and its read-through
  tables credit 23 rows to "Reader A" and 14 to "Reader B". `log-tail.md:14` has "a further
  reader", with 12 rows. None of the three wrote a report that is kept, and their rows are
  outside the 125 findings that the tracker gives for round 2.
- Why it misleads: the section exists so that a reviewer can tell the owner's rulings from
  the coordinator's decisions. A third class, an editor's choice that the coordinator let
  stand, is described as not existing; and "four cold readers" understates who read the
  specification in round 2.
- Least change: in the table, say that an editor's own choices are logged under "Choices
  made" in each log and stand unless struck, and that the closing and last passes of round 2
  used three further readers whose findings are only in those logs.

**4D-3. `misleading`. `AGENTS.md` states as the owner's instruction what the tracker lists as
the coordinator's unconfirmed reading.**

- Where: `AGENTS.md:78-82`; against `docs/AUDIT.md:143` and row 36 of the ruling log.
- Quote, `AGENTS.md`: "the owner's instruction of 2026-11-05 is that this work is tracked on
  [pull request #6] [...], and those sessions commit and push to its branch, `docs/plan-audit`,
  at each milestone."
- What the tracker says: ruling 36, "The owner asked for the work to be tracked on a pull
  request". The list for the owner: "its reading of the instruction of 2026-11-05 to keep a
  record: that it commits and pushes to pull request 6 at each milestone."
- Why it misleads: the one standing permission to commit and push reads as the owner's words.
  The tracker says it is a reading that waits for the owner.
- Least change: in `AGENTS.md`, end the owner's instruction at "tracked on pull request #6",
  and say that committing and pushing at each milestone is the coordinator's reading of it,
  which the owner has not confirmed.

**4D-4. `misleading`. The front page still gives "accepted by the owner" the meaning that
`spec/README.md` withdrew.**

- Where: `README.md:26`; against `spec/README.md:7-18`.
- Quote, `README.md`: "That means the owner has ruled on what they say, and they are still
  part of the draft."
- The specification: "'accepted' covers less than the part as it now stands"; "no text that
  they changed or added is a choice the owner accepted."
- Why it misleads: finding 3D-2 of round 3 named this line. The specification's README was
  corrected and the front page was not, so the first document says the opposite of the second.
- Least change: "That means the owner ruled on the part as it stood at the ruling named. What
  later passes changed is the coordinator's, and `spec/README.md` says which."

**4D-5. `misleading`. The index and `AGENTS.md` say that every record holds what the owner
ruled.**

- Where: `docs/README.md:46-49`, the heading "Records of rulings" and its first sentence;
  `AGENTS.md:38`.
- Quote, `docs/README.md`: "Each record says what was proposed, what the owner ruled and why."
  `AGENTS.md`: "The records under `docs/proposals/` say what the owner ruled and why."
- What the tree shows: the last four rows of that table (the documents audit and rounds 2 to
  4) hold decisions that the tracker gives to the coordinator. Their "Ruling" column reads
  "37" or "30, 37", which are the rulings that asked for the work, not rulings on its content.
- Least change: add to both sentences "or, for the documents audit and rounds 2 to 4 of the
  re-audit, what the coordinator decided under ruling 37".

**4D-6. `missing`. Two records do not say whose decisions they hold.**

- Where: `docs/proposals/docs-audit/README.md:28-31`;
  `docs/proposals/reaudit/round-2/README.md:7-8`.
- Quote, the documents audit: "Decisions taken to fix it"; "Each decision below follows a
  ruling already made". Round 2: "resolved under ruling 37, without a ruling on each, and
  every choice is logged."
- What the tree shows: neither page uses the word "coordinator" for its decisions. The
  tracker rests "nothing outside `spec/` binds" on "decision DA1 of the documents audit"
  (`docs/AUDIT.md:83`, `:385`, `:569`), and DA1's own record does not say who took it. The
  records of rounds 3 and 4 do say it: "as the coordinator's and not the owner's".
- Least change: one clause in each: "Each is the coordinator's and not the owner's, and the
  owner can strike any of them."

**4D-7. `missing`. A coordinator's decision that changes something the owner accepted is
marked as such in one case only.**

- Where: `docs/AUDIT.md:141`; the table "Later amendments" at `docs/AUDIT.md:291-303`; the
  "Amended by" list at the head of each record under `docs/proposals/`.
- Quote, `docs/AUDIT.md:141`, of finding 3B-2 alone: "which bears on a change the owner made
  in ruling 37".
- What the tree shows: at least five more decisions of the same standing.
  The scope of an operation identity: `docs/proposals/operations/README.md:15-16` gives, as
  the last amendment, ruling 32, "an identity is scoped to one caller and one target"; OPS-008
  now adds "and the instance that sent the request" (round 2, R3-3).
  The order of reasons: ruling 32, decision Q5, "otherwise refused, then invalid, then
  conflict, then unavailable"; OPS-023 now puts unavailable in the place of refused where the
  access decision cannot be made (round 3, 3A-1).
  A control's `changes`: the owner's change X1 of ruling 37 has "`changes: [/facts]`", a list
  (`docs/proposals/core-and-schemas/APPLY.md`); it is now a map (round 2, R2-G2).
  A runtime's `optional`: decision D4 of ruling 31 was accepted with "`optional: true`"
  (`docs/proposals/schemas-and-interface/README.md:73`); a runtime now carries none (round 2).
  The conformance levels: decision PD5 of ruling 24 accepted "Two levels"; there are three
  since row GP16 of `docs/proposals/docs-audit/GAPS.md`.
  No "Amended by" list names the documents audit or a round of the re-audit, and "Later
  amendments" names them only for the two items added to the implementation-defined list.
  The two latest records of rulings, `docs/proposals/reaudit/README.md` and
  `docs/proposals/core-and-schemas/README.md`, have no "Amended by" list at all, and do not
  carry the sentence that the specification governs where they differ.
- Why a reviewer needs it: the trace runs one way. From a promise, the head comment of its
  file leads to the pass that changed it. From a ruling or its record, nothing says that a
  later decision, which the owner has not seen, changed what was accepted.
- Least change: a row for each in "Later amendments", with "by the coordinator, round N",
  and one line at the end of the "Amended by" list of the five records concerned.

### The list "Waiting for the owner"

**4D-8. `missing`. The list leaves out things that the records leave for the owner.**

- Where: `docs/AUDIT.md:133-152`.
- Quote: "One list of everything that only the owner can settle."
- Not in the list:
  - The conversions of the documents audit. `docs/proposals/docs-audit/GAPS.md:9-10`: "Every
    row is a choice made by the coordinator [...]. The owner can strike any of them." The row
    on the coordinator's decisions points at rounds 2 and 3 only. Row GP16 of that file added
    the third conformance level.
  - The choices made while ruling 37 was applied:
    `docs/proposals/core-and-schemas/README.md:429-432`, "each is for the owner to strike or
    keep."
  - The four corrections that the check against the protocol set aside.
    `docs/proposals/reaudit/round-2/mcp-citations.md:125-127`: "Corrections B, C, D and E
    touch behavior or state a departure from the protocol, so each is for the owner to rule
    on. None of them has been applied." The coordinator then settled and applied all four
    (`round-2/DECISIONS.md:131-147`). The report carries no note that this happened.
  - Two points of the documents audit:
    `docs/proposals/docs-audit/log-entry-points.md:89` and `:92`, the status of action A25 and
    question Q8, "one for the owner to confirm", and a state for each readiness condition,
    "An owner call."
  - The choices before ruling 37 that finding 4D-1 lists.
- Least change: add a row for each, with its pointer; and put one line under the summary of
  `mcp-citations.md` that says where the four corrections were decided.

**4D-9. `stale`. Three pointers in the list do not land on what they claim.**

- Where: `docs/AUDIT.md:146`, `:147` and `:143`.
- Quote: "The rename of the conformance level "Host" to "Engine". | [Documents audit]";
  "Whether a domain is still needed before the engine builds [...] | Rulings 33 to 35";
  "The coordinator's reading of ruling 37, above [...] | This section; `AGENTS.md`".
- What the tree shows: `docs/proposals/docs-audit/README.md` does not contain the word
  "Host". The rename is recorded in `docs/proposals/packages-and-start/README.md:25-27`,
  "fixed without a ruling of its own", and the row does not say that it is already made:
  `spec/README.md` has said "Engine" since consolidation pass 1. The question on the domain
  is recorded in `docs/proposals/docs-audit/log-entry-points.md:90`, and rulings 33 to 35 do
  not raise it. The reading of ruling 37 is at `docs/AUDIT.md:129-131`, in the section before.
- Least change: point the first at the packages-and-start record and say "made, and not
  ruled on"; point the second at the log; write "the section above" in the third.

**4D-10. `minor`. One item of the list misstates the decision it names.**

- Where: `docs/AUDIT.md:141`.
- Quote: "a result with no outputs being an error result".
- The specification: IFC-025, "the result of an operation that declares none is never marked
  as withheld, and where its outcome is succeeded its `isError` is not true". The decision is
  that a result which carries none of the outputs its operation declares has `isError` true.
- Least change: "a result that carries none of its operation's declared outputs being an
  error result".

**4D-11. `missing`. Ruling 37 asks for a cold read by Codex after these rounds. Nothing
tracks it.**

- Where: `docs/AUDIT.md:287`, against "What happens next" at `docs/AUDIT.md:89-101` and the
  action items.
- Quote, ruling 37: "the repository is to be made ready for a cold read by Codex after these
  rounds."
- What the tree shows: outside the ruling log, the tracker names Codex only for the builds,
  the acceptance scenario and the probes. None of `README.md`, `AGENTS.md`,
  `CONTRIBUTING.md`, `docs/README.md` or the round records names it. Action A61 is done when
  "a cold reader" can tell what binds, and the tracker defines a cold reader as "A separate
  session that the coordinator starts". A reviewer cannot tell whether the read that the
  owner asked for is planned, replaced by the fourth readers, or forgotten.
- Least change: a fourth step in "What happens next", or a clause in the done-when of A61.

### Records and statuses

**4D-12. `missing`. Round 3 has no record of what was done about its fourth reader's findings
outside `spec/`.**

- Where: `docs/proposals/reaudit/round-3/DECISIONS.md:76`.
- Quote: "The findings outside `spec/` were fixed when the report came in (commit
  `d3ae339`)."
- What the tree shows: `log-fixes.md` covers 3D-2, 3D-5 and 3D-11 to 3D-17. For 3D-1, 3D-3,
  3D-4, 3D-6 to 3D-10 and 3D-18 to 3D-23 the only account is the message of that commit,
  which names no finding. Round 2 has a log for the same reader, `log-reader-4-fixes.md`,
  with one row for each finding and a section "Left for the owner". I checked the fourteen
  against the tree and each is fixed or is in the list for the owner; a reviewer has to do
  the same to know it.
- Least change: a short table in `round-3/DECISIONS.md`, one row for each of the fourteen:
  what was changed, or "left for the owner".

**4D-13. `stale`. The tracker's account of the documents outside `spec/` stops one round
back.**

- Where: `docs/AUDIT.md:401`, the status of A61; `docs/AUDIT.md:81`;
  `docs/proposals/docs-audit/README.md:59-62`.
- Quote, A61: "The fourth reader of round 3 reads the whole repository cold again, and the
  action is done when that reader reports nothing that misleads".
- What the tree shows: that reader has reported: 23 findings, 9 of them `misleading`
  (`docs/proposals/reaudit/round-3/reader-4.md`). The row at `:81` and the record of the
  documents audit name the fourth reader of round 2 and no later one. The same tracker says
  "Round 3 is read" in three places (`:80`, `:91`, `:399`) where the record of round 3 says
  "Status: done".
- Least change: in A61, "The fourth reader of round 3 reported 23 findings, 9 that mislead;
  they are fixed or listed for the owner. The fourth reader of round 4 reads it again"; and
  add round 3 to the other two places.

**4D-14. `stale`. The count of the round-2 whole-repository reader is 47, not 57.**

- Where: `docs/proposals/reaudit/round-3/README.md:11`.
- Quote: "| Findings of the reader of the whole repository | 57 | 23 |".
- What the tree shows: `docs/proposals/reaudit/round-2/reader-4.md` has 6, 7, 11, 7, 10 and 6
  numbered findings in its sections A to F, which is 47. Section G, "The top problems,
  ranked", has 10 items that restate them, and `log-reader-4-fixes.md` maps each to findings
  of A to F under the heading "The ranked list". Counting G gives 57.
- Why it matters: the table is the evidence that the repository is converging. The fall is
  from 47 to 23.
- Least change: "47".

**4D-15. `minor`. The counts of round 1 do not reconcile.**

- Where: `docs/proposals/reaudit/README.md:18`; `docs/AUDIT.md:80` and `:399`;
  `docs/proposals/reaudit/round-4/README.md:6-7`.
- Quote, round 1: "About 70: 24 groups of duplicates, 25 contradictions, 12 stale wordings,
  10 misplaced, 10 to split". The tracker: "about 170 problems". Round 4: "about 170
  findings, then 125, then 44."
- What the tree shows: the five figures of reader 1 sum to 81, and its report has 24, 25, 12,
  10 and 10 numbered items. With the 69 of reader 2 and the 50 of reader 3 the three reports
  hold about 200. The "about 170" matches the handling table (115, about 55 and 3), which is
  a count by class. Rounds 2 and 3 are sums of the reports, so the series compares unlike
  figures.
- Least change: "About 80" for reader 1, and one clause that says what "about 170" counts.

**4D-16. `stale`. The tracker's sentence on the protocol check names one of four passes that
changed the interface after it.**

- Where: `docs/AUDIT.md:80`.
- Quote: "The last pass of round 2 then changed fourteen of those promises and added IFC-034
  and IFC-035, to follow the report; the text as changed has not been checked against the
  protocol again."
- What the tree shows: the head comment of `spec/interface/implementation.yaml` lists three
  later passes: the follow-up to that pass (IFC-006, IFC-009, IFC-016, IFC-034), the fix pass
  of round 3 (IFC-006, IFC-021, IFC-024, IFC-025, IFC-028) and its follow-up (IFC-024).
  `round-4/README.md:17-19` has it right: "after the changes of rounds 2 and 3".
- Least change: "Later passes of rounds 2 and 3 changed more of them; none of the changed
  text has been checked against the protocol again."

### `docs/OVERVIEW.md` against the specification

Each sentence of the overview that cites a promise agrees with that promise, apart from the
second of these. The findings are about what is left out.

**4D-17. `misleading`, by omission. The overview does not say who applies a restriction.**

- Where: `docs/OVERVIEW.md:108-118`, and `:236-237`.
- Quote: "A filter is carried by one Conduit and narrows what may pass through it."; "Mediated
  means that the restriction applies to requests that pass through the engine."; "The engine
  makes its own access decision before any implementation is invoked (ACC-002)."
- The specification: ACC-008, "The implementation applies the restrictions it is sent under
  IFC-019". IFC-019, "It applies every restriction in the list to what the request does and
  to what it returns." The engine decides only whether the Conduit lists the operation, and
  checks the bound and the shared-host requirement itself. The target's implementation must
  state a filter's requirement in `supports` (CON-022), an implementation marked `plain`
  supports none (PKG-020), and a request that carries a restriction which the implementation
  does not support reports unavailable (ACC-015).
- Why it misleads: a reader concludes that the engine evaluates filters and policies. In the
  specification the implementation of the target does, and what an author may put on a
  Conduit depends on what that implementation supports.
- Least change: two sentences after line 112: the engine sends each request's restrictions to
  the target's implementation, which applies them and must state their requirements in
  `supports`; a plain implementation supports none.

**4D-18. `misleading`. The overview's rule for a policy has no exception for the root
Interface, and the section says nothing else of what sets `root` apart.**

- Where: `docs/OVERVIEW.md:109-112`, and `:126-132`.
- Quote: "It applies to each request whose caller is one of those actors, where any are
  named, and whose target is one of those targets, where any are named (ACC-031)."
- The specification: ACC-035, a policy must not apply "to a request whose caller is the root
  Interface"; a policy that gives only `targets` "does not apply to a request that the root
  Interface makes of one of them". ACC-024: a request to close a Conduit whose consumer is
  the root Interface "reports refused". LIF-022: no first-start object is retired,
  unregistered or renamed. ACC-032: `declare-lost.v1` and `purge.v1` are dispatched only when
  the root Interface asks.
- Why it misleads: the sentence as written covers `root`. A reader also takes it that the two
  Conduits of `root` can be closed and that `root` can be retired like any instance, since
  "One lifecycle" gives no exception.
- Least change: one sentence in "Conduits and access": no policy applies to a request of
  `root`, no request closes a Conduit of `root` or retires a first-start object, and two
  operations are reserved to `root`.

**4D-19. `misleading`, by omission. The engine itself dispatches one operation for no actor,
and the overview's two lists of exceptions leave it out.**

- Where: `docs/OVERVIEW.md:221-225`, and `:269`.
- Quote: "nothing acts to remove it without an explicit request (OBS-004). There is one
  exception: an implementation may restore its own runtime, where its type's contract allows
  it."; "Not every invocation begins with a consumer's request."
- The specification: OBS-007, where the type's contract does not allow the restoring, "the
  runtime is stopped under its `stop` operation [...]. The engine dispatches that stop for no
  actor: the request carries no caller". The definitions' row for "caller" says "One request
  has no caller".
- Why it misleads: the overview gives later work as the one way an invocation begins without
  a request, and the restoring that a contract allows as the one exception to "nothing acts".
  A reader concludes that the engine never dispatches on its own. It does, in this one case,
  through no Conduit.
- Least change: one sentence after line 225: where the type's contract does not allow the
  restoring, the engine stops the runtime itself, for no actor (OBS-007).

**4D-20. `minor`. Smaller omissions in "One lifecycle" and in reconciliation.**

- Where: `docs/OVERVIEW.md:210-215`, and `:257-261`.
- Quote: "Those of which it is the consumer are closed once its shutdown is confirmed"; "the
  Pool that owns it where it is a Resource, and `management` otherwise (ACC-033)."
- The specification: CON-008 also closes a Conduit at once where "either of whose ends is
  declared lost". The overview uses "declared lost" and "shutdown" and explains neither:
  declaring a location lost retires the location and every instance placed on it (PLC-017).
  ACC-033 also sends the reconciliation to `management` where the Pool is retired, and where
  the instance has been unregistered.
- Least change: "or at once where it is declared lost"; one clause each for what declaring
  lost and shutdown are; "or where that Pool is retired".

**4D-21. `stale`. The overview's "last read" line is one commit behind the specification.**

- Where: `docs/OVERVIEW.md:4-6`.
- Quote: "last read against the specification on 2026-11-06, as the fix pass of [round 3 of
  the re-audit] left it."
- What the tree shows: the overview was last changed in commit `a67890d`, the fix pass.
  Commit `f583469`, the follow-up, then changed eleven files of promises and added OPS-031,
  and did not touch the overview. I found no sentence of the overview that the
  follow-up contradicts; the line is true and reads as more than it says.
- Least change: date the line by the commit, or add "the follow-up to that pass was not read
  against it".

### Hygiene

**4D-22. `hygiene`. The tracker names two repositories that do not answer publicly, and the
list for the owner counts one.**

- Where: `docs/AUDIT.md:765`, the fourth row under "Removed on 2026-11-05";
  `docs/research/PI_RUNTIME.md:64`; against `docs/AUDIT.md:150`.
- Quote, the list: "The private repository named among the [reference repositories]".
- What it shows: the repository at `docs/AUDIT.md:756` is the one the tracker calls
  proprietary. A second repository of the same organization is named nine lines below, in the
  next table, with no note. On 2026-11-06 the hosting service answered "not found" without
  sign-in for both, and "found" for the two other reference repositories I asked for. I
  cannot tell a private repository from one that does not exist.
- Least change: make the row of the list plural, so that the owner's decision covers both.

**4D-23. `hygiene`. The commit messages of this branch carry a link to a private session.**

- Where: Git history, not the tree. `git log --format=%b | grep -c Claude-Session`.
- What it shows: 21 of the 24 commits on this branch since it left `main` end with a trailer
  "Claude-Session:" and an address of a hosted session. An outside reader cannot open it, and
  it is published with every clone. The tracker already cites one such trailer as a source
  (`docs/proposals/reaudit/round-2/log-reader-4-fixes.md:112-113`). The known item on commit
  addresses has also grown since finding 3D-18: 21 of the 24 commits now carry an address
  made from a machine or home-network name, four of them made after that finding.
- Least change: the owner decides whether the trailer stays; if it does, one sentence in the
  tracker that says what it is and that the address is not public.

### Minor

**4D-24. `minor`. "Every packet predates the specification" is not true of one packet.**

- Where: `docs/README.md:98`; `docs/research/README.md:3`.
- Quote: "every packet predates the specification".
- What the tree shows: `docs/research/naming/README.md` was retrieved on 2026-11-05 for
  rulings 25 and 27, after the specification was begun. The log of round 2 says so of this
  packet: the usual line "is not true of this one" (`log-reader-4-fixes.md`, row E10).
- Least change: "every packet but the naming research".

**4D-25. `minor`. The record of round 4 does not name the commit that its readers read.**

- Where: `docs/proposals/reaudit/round-4/README.md:3-4`.
- Quote: "as it stands after [round 3] and its fix pass."
- What the tree shows: the records of rounds 2 and 3 name `bdacd83` and `b1d3fb4`. Round 3
  ended with a follow-up after its fix pass, so "after its fix pass" fits two commits. I read
  `f583469`.
- Least change: "at commit `f583469`".

**4D-26. `minor`. Inside `spec/`, the changes of the documents audit and of round 2 are not
credited as the changes of round 3 are.**

- Where: `spec/schema-profile.md:3-5`; the head comments of the promise files, for example
  `spec/promises/conduits.yaml:7-21` against `:22-25`.
- Quote, the schema profile: "Plan-audit rulings 23, 24, 32, 33 and 37 of 2026-11-05,
  decision DA3 of the [docs audit], and the [decisions of the second round of the re-audit]."
- What the tree shows: the status line sets the owner's rulings and the coordinator's
  decisions in one list. In the head comments, the passes of round 3 are described as resting
  "on the coordinator's decisions [...] which the owner has not ruled on", and the passes of
  the documents audit and of round 2 carry no such words. `spec/README.md:11-18` covers all
  of them in general terms, so a reader of one file alone is the one misled.
- Least change: the same six words after each mention of the documents audit and of round 2.

## What a newcomer can rely on

Each of these was checked against the tree and found accurate.

- **What exists.** No engine, no reference model and no conformance vector. The directories
  `conformance/`, `conformance-held-out/` and `reference-model/` do not exist, and all 437
  `vectors` lists are empty. Every entry document says so, and none presents a check of this
  repository as a test of behavior.
- **What binds.** `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md` and
  `spec/README.md` state one rule and one reading order. The same sentence describes Conduit
  in all of them and in the overview.
- **The counts of `spec/README.md`.** 437 promises, and the count of each of the 22 files;
  221, 181 and 35 by layer; 15 implementation-defined items; 99 exported documents, which are
  21, 5, 6, 2, 11 and 2 contracts, 7 templates and 45 schemas in 9 files; 7, 61 and 20
  example files. All were recounted and agree.
- **The count of promises at each step.** 312, 339, 353, 355, 369, 376, 395, 406, 410, 421,
  424, 436 and 437 can each be found in a log or a decisions file, in that order, and the
  tracker's 376 at ruling 37 is the figure of the log of that batch.
- **The findings of rounds 2 and 3.** 125 is 36, 49 and 40 as the decisions give them, and
  the reports of readers 1 and 3 state 36 and 40; 44 is 16, 13 and 15, with 1, 1 and 2 marked
  unsafe, as the three reports state. The round-3 whole-repository report holds 23.
- **The check against the protocol is described as it is.** The tracker says which promises
  were checked, on what date, with what result (26, 3 and 2 of 31, as the report's own count
  has it), and that the text changed since has not been checked. The specification's README
  says that none of the four checks is a conformance vector.
- **The four checks** pass, and print what `AGENTS.md` and `CONTRIBUTING.md` say they print.
- **The records are reachable.** Every Markdown file is linked from at least one other,
  `CLAUDE.md` apart, and every directory under `docs/proposals/` has a row in
  `docs/README.md`. Each record of a ruling up to ruling 31, and the consolidation record,
  says under its status that it is a record and that the specification governs.
- **The specification says who decided.** `spec/README.md`, `spec/definitions.md`,
  `spec/core/README.md`, `spec/schemas/README.md` and the head of every file of the core
  package say that what changed after ruling 37 is the coordinator's and not the owner's.
- **The older material is marked.** Every file under `docs/protocol/`, `docs/authoring/`,
  `docs/proposals/authoring-syntax/` and `docs/research/`, the three superseded files under
  `docs/implementation/`, `docs/REVIEW.md`, `docs/VOCABULARY.md` and Experiment 002 carries a
  note under its title. The index of the archive carries the note for the archive.
- **The overview's worked example** is quoted exactly from the two files of the example
  project.
- **The evidence is described honestly where it is kept.** Experiment 004 says that it is a
  tool evaluation and not conformance evidence, and its pinned toolchain and dependency are
  the ones its README names. Experiment 003 holds the 20 cases it claims. Experiment 002 says
  that nothing in it was run.
- **The pointers of "Waiting for the owner"**, apart from the three of finding 4D-9, land on
  a passage that says what the row says.
- **The tree is clean.** No home-directory path, user name, mail address or secret; no stray
  or generated file; the ignore rules cover the only build output. The one absolute path is
  the interpreter path that `check_schemas.py` prints, quoted in five records.
- **The pull request.** Pull request 6 is open as a draft, and its head is the commit I read.
