# Round 3, reader 4: the whole repository, read cold

Read at commit `b1d3fb4` on 2026-11-06, on the branch `docs/plan-audit`, with a clean working
tree. Paths in this report are relative to the root of the repository.

## Summary

Twenty-three findings.

| Severity | Count | Findings |
| --- | --- | --- |
| `misleading` | 9 | 3D-1, 3D-2, 3D-4, 3D-5, 3D-6, 3D-12, 3D-13, 3D-14, 3D-15 |
| `stale` | 4 | 3D-7, 3D-8, 3D-9, 3D-11 |
| `missing` | 2 | 3D-3, 3D-10 |
| `hygiene` | 1 | 3D-18 |
| `minor` | 7 | 3D-16, 3D-17, 3D-19, 3D-20, 3D-21, 3D-22, 3D-23 |

The entry documents do their main job. From `README.md` alone a stranger learns what the
project is, that only `spec/` binds, that there is no engine, no reference model and no
conformance vector, and that a third round of the re-audit is in progress. The counts in
`spec/README.md` are exact. The findings fall in three groups:

- The entry documents say that accepted behavior changes only by a logged ruling of the
  owner. Since ruling 37 the binding text has changed by choices of "the coordinator", who is
  nowhere introduced, and one file of the core package still credits those choices to the
  owner (3D-1 to 3D-4).
- Five status statements are ahead of, or behind, what the records show (3D-5 to 3D-9), and
  what waits for the owner is spread over at least five files (3D-10).
- `docs/OVERVIEW.md` was not read again after the last pass of round 2, which added later
  work, the rule on installing and the rule on a retired consumer (3D-11 to 3D-17).

Read in full: `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`,
`docs/OVERVIEW.md`, `docs/AUDIT.md`, `spec/README.md`, `spec/definitions.md`,
`spec/names.yaml`, `docs/proposals/reaudit/README.md`, the three files of round 2 named
`README.md`, `DECISIONS.md` and `log-reader-4-fixes.md`, the README of round 3,
`docs/proposals/docs-audit/README.md`, the two build documents under `docs/implementation/`,
`docs/protocol/README.md`, `docs/archive/README.md`, `experiments/README.md` and the READMEs of
Experiments 003 and 004.

Sampled: 83 promises by identifier, printed from the YAML (every promise the overview cites,
and those the last pass of round 2 added); the header comment of each file under `spec/` that
a finding touches; the status header of every record under `docs/proposals/`; the first lines
of every Markdown file under `docs/` and `experiments/`, for its note; `log-tail.md` from its
line 100 to its end, the headings of `log-closing.md`, the opening of each round-2 reader
report and the summary of `mcp-citations.md`; the head and the table of
`docs/LANGUAGE_DEFECTS.md`; the docstrings of the four tools; `git log` for authors and dates.

Not read: the bodies of the protocol topics, the research packets, the archived files, the
authoring-syntax packet and the two architecture documents; the logs of round 1; the body of
the core-and-schemas record; most promise files end to end; the schemas and the examples,
apart from the two documents the overview quotes.

Not verified: `cargo test --release` for Experiment 004 was not run, because it writes build
output. What `check_schemas.py` does when a package is missing was not tried. Nothing was
checked against the Model Context Protocol. Of the external links, four GitHub addresses were
fetched and no other.

This report is named in `docs/proposals/reaudit/round-3/README.md` as plain text. No document
links to it yet.

## The four checks

Run from the root, in the order `AGENTS.md` gives. Each exited with status 0.

```text
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 121 Markdown files and 1551 local links (offline).
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x432, apiVersion x228, metaPrefix x88. Errors: 0

$ python3 tools/check_schemas.py
Python 3.9.6 at <path of the system interpreter>; PyYAML 6.0.1; jsonschema 4.20.0
Profile: 27 keywords, 5 formats. Format checkers available for all of them.
1. Read 100 files holding 225 documents as strict YAML.
2. Checked 58 Schema contracts against the profile, and every `$ref`.
3. Validated 137 documents under spec/schemas/, spec/core/ and examples/valid/.
   (59 lines, one for each invalid example, each beginning "rejected")
4. 59 invalid documents in 59 files were each rejected by a schema, or rejected when read.
5. Core package: 99 documents exported, 0 documents deliberately private and 1 implementation,
   which is also an export. Checked 434 references for the kind they name; 1 name packages
   that are not in this repository and were not checked. Checked 3 controls, the 5 operations
   that take no `revision`, the tables of the two READMEs, and 799 citations of promises in
   103 files. The example packages keep 0 documents private.
   (17 lines, one for each set under invalid-by-rule, each beginning "rejected")
6. Applied 16 rules of this tool, which come from 14 promises, to the core package and 3
   example directories. 17 sets of documents in 17 files under examples/invalid-by-rule/ were
   each rejected by the rule they are named for; each of their 30 documents is valid against
   the schemas.
   SCH-019: validated 10 defaults of the core package and the valid examples against their
   schemas; 0 were not validated, because a `$ref` in the schema leads outside this repository.
Errors: 0

$ git diff --check
(no output)
```

The first line of `check_schemas.py` prints the absolute path of the interpreter it ran under.
It is elided above. The `python3` first on the path lacked the two packages, and the tool ran
itself again under the system interpreter, as `AGENTS.md` says it does. The lines of steps 5
and 6 are wrapped here; the tool prints each as one line.

`python3 tools/check_doc_migration.py`, the record of one past migration, was also run. It
printed "Errors: 0" and exited with status 0.

## Findings

### How the binding text changes, and who decides

**3D-1. `misleading`. The entry documents say that accepted behavior changes only by a logged
ruling. Since ruling 37 it has changed without one.**

- Where: `README.md:41-42`; `AGENTS.md:55-56`; `CONTRIBUTING.md:22-24`; `docs/README.md:115-117`.
- Quote, `README.md`: "Accepted behavior changes only by a ruling of the project's owner,
  logged in the [plan audit]". `AGENTS.md`: "It changes only by a logged ruling."
- What the tree shows: the ruling log ends at ruling 37. Before the docs audit converted the
  older text, the specification had 376 promises (`docs/proposals/docs-audit/README.md`,
  "Applied"). It has 424 now. The 48 added, and the many changed, came from the docs audit
  and from round 2, by choices that no row of the log records.
  `docs/proposals/reaudit/round-2/DECISIONS.md:7`: "Each row is the coordinator's choice and
  can be struck by the owner." Row 9 of its section on the closing pass: "This is the one
  addition of this pass that is a new mechanism."
- Why it misleads: a reader of the four entry documents believes that every rule in `spec/`
  traces to a row of the ruling log. The tracker does say, in ruling 37, that findings are
  "resolved without waiting for a ruling on each". None of the four places that state the
  rule mentions the exception.
- Least change: one sentence where each states the rule: since ruling 37 a finding of a cold
  reader is settled by the coordinator, the choice is logged in the round's record, and the
  owner may strike it.

**3D-2. `misleading`. One file of the core package credits to the owner rules that round 2
added.**

- Where: `spec/core/operations.yaml:1` and `:11`; `spec/README.md:8-10`; `README.md:26`.
- Quote, `spec/core/operations.yaml:11`: "Read with these rules, each of which the record
  lists as a choice the owner accepted:". `spec/README.md`, of the word accepted on a part:
  it "says that the owner ruled on what it holds."
- What the tree shows: the sentence at line 11 is unchanged since the file was accepted
  (commit `42a0a14`). The list under it has since gained rules from round 2: that
  `reconcile.v1` is in the capability of `access`; that an instance is set up "by one
  invocation of `set-up.v1`"; that a control's `changes` is "a map from the name of an input
  to the pointer"; and two points on `start-later-work.v1`. The record it names,
  `docs/proposals/core-and-schemas/README.md`, contains none of the phrases "one invocation",
  "later work" or "map from the name of an input". `spec/core/README.md:3-17` is careful: it
  says which pass changed what, and that the last pass "added one export".
- Why it misleads: the contract `start-later-work.v1` sits in a file headed "Accepted by the
  owner on 2026-11-05", under a sentence that says the owner accepted each rule listed. The
  records show the coordinator chose it on 2026-11-06.
- Least change: reword line 11 to say that the rules are those of the record as round 2
  changed them, with the two logs; and in `spec/README.md:8-10` say that "accepted" dates the
  owner's ruling on the part, and that the header of each file lists what later passes
  changed under ruling 37.

**3D-3. `missing`. Nothing says who the coordinator, the editors and the readers are.**

- Where: `docs/proposals/reaudit/round-2/DECISIONS.md:7` and `:69-76`; the logs of both
  rounds; `docs/proposals/docs-audit/GAPS.md:9`.
- Quote: "Each row is the coordinator's choice"; "Editor P owns every promise file";
  "Written 2026-11-06 by Editor S".
- What the tree shows: the word "coordinator" does not occur in `README.md`, `AGENTS.md`,
  `CONTRIBUTING.md`, `docs/README.md` or `spec/README.md`, and in `docs/AUDIT.md` only in an
  unrelated historical table. No record defines it. `docs/AUDIT.md:11-12` says that the first
  audit was made "by a Claude session", and `docs/implementation/INDEPENDENT_BUILDS.md:37`
  speaks of "the sessions that have been writing the language". Nothing connects those to the
  coordinator, the editors or the "cold readers".
- Why a reviewer needs it: these parties changed the binding text (3D-1). A reviewer has to
  know whether each is a person or a model session, and which, to weigh "Each can be struck
  by the owner".
- Least change: one short paragraph in `docs/AUDIT.md`, beside the paragraph on the external
  audit, that says who or what the coordinator, an editor and a cold reader are.

**3D-4. `misleading`. The record of round 3 gives ruling 37 a test that the ruling log does
not hold.**

- Where: `docs/proposals/reaudit/round-3/README.md:15-16`.
- Quote: "Findings are resolved under ruling 37: without a ruling on each, with every choice
  logged in this directory, and only what changes what Conduit is for goes back to the owner."
- What the tree shows: row 37 of the ruling log says only that findings "are to be resolved
  without waiting for a ruling on each". The phrases "what Conduit is for" and "goes back to
  the owner" occur nowhere else in the repository.
- Why it misleads: the test decides what the owner sees. It reads as the owner's, and no
  record shows that it is.
- Least change: say whose test it is. If it is the owner's, add it to the ruling log by the
  owner's hand; if it is the coordinator's reading of ruling 37, say so in that sentence.

### Status claims

**3D-5. `misleading`. "Checked against the published protocol" is said of text that was
changed after the check.**

- Where: `docs/AUDIT.md:80`; `docs/proposals/reaudit/round-3/README.md:27-29`.
- Quote, `docs/AUDIT.md`: "What the interface says about the Model Context Protocol was
  checked against the published protocol, and corrected where it departed".
- What the tree shows: `mcp-citations.md` checked "The promises IFC-001 to IFC-033" as they
  stood before the last pass. That pass then changed fourteen interface promises and added
  IFC-034 and IFC-035. `docs/proposals/reaudit/round-2/log-tail.md:216-218`: "Not verified:
  nothing was checked against the Model Context Protocol in this pass. What the interface now
  says of the protocol rests on the report of the check and on the decisions on it." The
  report also says that "2 cannot be confirmed"; the specification now assumes one of them
  and says so in IFC-025.
- Why it misleads: a reader takes it that the interface as it stands was checked.
- Least change: add to both sentences that the corrections, IFC-034 and IFC-035 were written
  after the check and have not been checked, and that two claims could not be confirmed.

**3D-6. `misleading`. Two records state as settled a question that a third lists for the
owner.**

- Where: `docs/proposals/reaudit/round-3/README.md:3-4`; `docs/proposals/reaudit/README.md:118`;
  against `docs/proposals/docs-audit/README.md:73` and
  `docs/implementation/INDEPENDENT_BUILDS.md:111`.
- Quote, round 3: "The readiness condition asks for two consecutive rounds with no finding in
  the language, the conventions or the interface." Round 1: "The readiness gate asks for two
  clean rounds." The docs audit, under "Left for the owner": "Whether readiness condition G7
  is judged on two clean rounds of the re-audit."
- What the tree shows: G7 reads "two consecutive sweeps of the language and the shared
  conventions find no new defect". It does not name the interface, and its evidence is the
  defect register, which says of itself "Findings against `spec/` are not added here". The
  words of the round-3 record are the done-when of action A59, not G7.
- Least change: in both records say "the done-when of A59" in place of "the readiness
  condition" and "the readiness gate"; and name the open question on G7 in the readiness row
  of "Where things stand".

**3D-7. `stale`. The record of round 1 says twice that round 2 is in progress.**

- Where: `docs/proposals/reaudit/README.md:7-8` and `:118-119`.
- Quote: "Round 2, with four readers, is in progress and has [its own record]"; "In progress:
  [round 2]".
- What the tree shows: `docs/proposals/reaudit/round-2/README.md:3`: "Status: done,
  2026-11-06".
- Least change: "is done" in both places, and a link to round 3 beside it.

**3D-8. `stale`. The documentation map says that round 2 is its last row.**

- Where: `docs/README.md:66`.
- Quote: "Round 2 is the last row of this table."
- What the tree shows: the last row, line 70, is round 3.
- Least change: "Rounds 2 and 3 are the last two rows of this table."

**3D-9. `stale`. Round 2 has seven unsafe findings in one place and eight in three.**

- Where: `docs/proposals/reaudit/round-2/DECISIONS.md:3-4` and `:13`; against
  `docs/proposals/reaudit/round-2/README.md:5-6`, `docs/AUDIT.md:80` and `:349`.
- Quote, the decisions: "reader 3, 40, of which 7 unsafe", under the heading "Unsafe findings
  (reader 3)". The record: "The three who read only `spec/` reported 125 findings, 8 of them
  unsafe."
- What the tree shows: reader 3 counts "7 UNSAFE". The report of reader 1 does not use the
  word. The table under the heading has eight rows, and the eighth is R1-1.1, a finding of
  reader 1 that the coordinator placed there.
- Why it matters: "reported ... 8 of them unsafe" is not what the readers reported.
- Least change: in the record and the tracker, "7 of them marked unsafe by their reader, and
  one more treated as unsafe"; and drop "(reader 3)" from the heading.

**3D-10. `missing`. There is no one list of what waits for the owner, and the tracker's
pointer to round 2 lands on a page that has none.**

- Where: `docs/AUDIT.md:93-97`.
- Quote: "What only the owner can settle is listed for the owner: in the [section below], in
  the [record of the documents audit] and in the [record of round 2]."
- What the tree shows: the third link is `docs/proposals/reaudit/round-2/README.md`, which
  names one point, on secrets. The eight points of round 2 are in
  `log-reader-4-fixes.md`, under "Left for the owner". Others are only in a status cell of the
  tracker: whether G1 is met (the readiness row), and five standing inconsistencies and a
  branch (A21). The choices that "can be struck by the owner" are in `DECISIONS.md`, the two
  later logs, `GAPS.md` and the consolidation record.
- Why a reviewer needs it: what is still open to the owner is the first thing a reviewer of
  the state asks, and it takes five files to assemble.
- Least change: point the third link at `log-reader-4-fixes.md#left-for-the-owner`, and add
  to "What happens next" one list of every open point, each with a link to its record.

### `docs/OVERVIEW.md` against the specification

**3D-11. `stale`. The overview was not read again after the last pass of round 2, and its
date cannot show that.**

- Where: `docs/OVERVIEW.md:3-5`.
- Quote: "written from [spec/README.md] and [spec/definitions.md] as they stood on
  2026-11-06".
- What the tree shows: the file was last changed in commit `cd2861d`, the closing pass. Two
  later commits of the same day took the specification from 410 promises to 424.
  `log-tail.md:10`: "Nothing under `docs/` was edited apart from this log". The fixes log of
  round 2 had asked for it, at line 133: "the overview needs one more reading after it".
  ACC-039, ACC-040, IFC-034, IFC-035, CON-031, IMP-010 to IMP-012, POOL-017, SUB-010, LIF-030
  and OPS-029 are all absent from the specification at `cd2861d`.
- Least change: read it once more against the specification as it stands, and date it by the
  commit it was read against. Findings 3D-12 to 3D-17 are what that reading would find.

**3D-12. `misleading`. Which tools the engine presents.**

- Where: `docs/OVERVIEW.md:281-282`, and `:92-95`.
- Quote: "the engine presents to each consumer, as tools, the operations its open Conduits
  list (IFC-012)"; "A request for an operation that no open Conduit from that consumer to that
  target lists is refused (ACC-001)."
- The specification: IFC-012 now reads "exactly the operations its open Conduits list, and
  the two tools of IFC-024 and IFC-035". IFC-024 says of the first: "It is presented whether
  or not any Conduit of which the instance is the consumer is open, and a call to it needs no
  Conduit." IFC-035 says the like of the second.
- Why it misleads: the section's claim in bold is "Every permission is an open Conduit". A
  reader does not learn that two operations of the core package are called with none.
- Least change: add "and two tools of the core package that need no Conduit:
  `retrieve-result.v1`, for one's own results, and `start-later-work.v1`".

**3D-13. `misleading`. What an open Conduit permits once its consumer is retired, and what
retiring an end does to a Conduit.**

- Where: `docs/OVERVIEW.md:92-93`, and the section "One lifecycle".
- Quote: "While a Conduit is open it permits exactly the operations it lists, on its target
  (ACC-003)."
- The specification: the observable of ACC-003 now adds "From the moment its consumer is
  retired, an open Conduit permits less: only the requests of the consumer's retirement, under
  ACC-039." CON-008: "A Conduit whose target is retired, or either of whose ends is declared
  lost, is closed at once. A Conduit whose consumer is retired is closed once the consumer's
  shutdown is confirmed." The overview says how a Conduit is closed by request, and nowhere
  that retiring an instance closes its Conduits.
- Least change: one sentence in "One lifecycle" that gives CON-008 and ACC-039.

**3D-14. `misleading`, by omission. A package listed in a manifest is not thereby installed.**

- Where: `docs/OVERVIEW.md:269-273`.
- Quote: "A project's manifest lists each package it uses with exactly one version and that
  version's digest. A document reaches an export by a qualified reference".
- The specification: the observable of PKG-006: "A registration fetches no package and
  installs none, whatever its manifest lists: a package is installed only by a request for
  `install.v1`". The definitions have a row for "install". The overview does not use the word.
- Why it misleads: a reader concludes that listing a package makes its exports reachable.
- Least change: one sentence: a package is made available only by `install.v1`, an operation
  of `management`; registering installs none, and reports a package that is missing.

**3D-15. `misleading`, by omission. Later work is absent.**

- Where: `docs/OVERVIEW.md:226-228` and `:254-256`.
- Quote: "A request that passes the engine's checks is dispatched, which begins an
  invocation."; "Model work starts only as the effect of an operation whose contract is marked
  as starting it, dispatched through an open Conduit that lists it (IMP-001). Setup never
  starts it."
- The specification: an operation marked `arrangesLaterWork: true` may arrange work that
  starts after its invocation has ended. The implementation that holds the work asks for each
  start with `start-later-work.v1`, and the engine dispatches it (ACC-019, ACC-040, IMP-006).
  `spec/definitions.md:178`: "A start is an invocation that the consumer did not request".
- Why it misleads: the overview's account has every invocation begin with a consumer's
  request. The one path by which model work starts without one is not mentioned, in the
  paragraph that says when model work starts.
- Least change: two sentences after line 256, from the definitions' row for "later work".

**3D-16. `minor`. "Exactly one of seven ways" has an eighth case.**

- Where: `docs/OVERVIEW.md:228`.
- Quote: "Every request ends in exactly one of seven ways".
- The specification: `spec/definitions.md:206-208`: "The result of a repeat of an invocation
  that has not ended is not one of the seven: it carries neither an outcome nor a reason, says
  `inProgress`". The same file still opens that section with "Every request ends in exactly
  one of these seven ways", so the two sentences also disagree inside `spec/definitions.md`.
- Least change: "ends in one of seven ways, apart from a repeat of an invocation that is
  still in progress".

**3D-17. `minor`. A claim of conformance also names a version.**

- Where: `docs/OVERVIEW.md:307`.
- Quote: "A claim of conformance names one of three levels".
- The specification: `spec/README.md:138`: "A claim of conformance names a version of this
  specification and one of three levels."
- Least change: add "a version of the specification and".

### Hygiene

**3D-18. `hygiene`. The commits of this branch carry addresses made from a local machine
name.**

- Where: Git history, not the tree. `git log --format='%ae' | sort | uniq -c`.
- What it shows: the 17 commits from `76a8f10` to `b1d3fb4`, which are every commit of this
  branch since it left `main`, have an author and a committer address whose host part is a
  machine name ending `.local` (15 commits) or a home-network name (2 commits). Three earlier
  merge commits carry a personal mail address. Of the rest, 386 use the hosting service's
  no-reply address and 3 are an outside contributor's. The repository is public: its address
  answers without sign-in, and the branch is level with its remote.
- Why it matters: a machine name and a personal address are published with every clone.
- Least change: set the no-reply address in the Git configuration before the next commit.
  Rewriting the 17 commits is the owner's call, since the branch is already pushed.

Nothing else was found under this heading. The tree holds no home-directory path, no user
name, no secret and no stray or generated file, and the ignore rules cover the only build
output. The one absolute path in the tree is the interpreter path that `check_schemas.py`
prints, quoted in five records under `docs/proposals/`; it names no user.

### Minor

**3D-19. `minor`. Who made the external audit is stated as fact, and a log says it is
unconfirmed.**

- Where: `docs/AUDIT.md:11-12`.
- Quote: "The external audit was made on that day, at the owner's request, by a Claude
  session that had not written the plans it read."
- What the tree shows: `docs/proposals/reaudit/round-2/log-reader-4-fixes.md:110-114`: "No
  document in the repository says this. ... The owner should confirm the clause or strike it."
- Least change: the owner confirms it, or the sentence says "as far as the records show".

**3D-20. `minor`. "Model checking ... supports the at-most-once promises" says more than the
experiment does.**

- Where: `docs/AUDIT.md:341`, the status of A13.
- Quote: "model checking in Experiment 004 supports the at-most-once promises".
- What the tree shows: `experiments/004-stateright-evaluation/README.md`: "It models one
  promise group to evaluate a tool. It is not a Conduit runtime, a reference model or
  conformance evidence", and "When this was run, the rules were read from the protocol topics,
  which are now superseded." The promises it names were written, and twice revised, after the
  run. A40 says it correctly: "open: tool evaluated".
- Least change: "a model written from the older text, in Experiment 004, kept the
  at-most-once rule in two rule sets; the promises as written are not yet model-checked".

**3D-21. `minor`. The contributor guide accepts "a common package envelope", a term of the
older design.**

- Where: `CONTRIBUTING.md:155`.
- Quote: "One main repository, a common package envelope and Apache-2.0 are accepted."
- What the tree shows: the phrase occurs otherwise only in superseded or archived files
  (`docs/archive/reviews/REPOSITORY-2026-11-02.md`, `docs/authoring/README.md`,
  `docs/implementation/ORGANIZATION.md`). The specification has a package and a manifest, and
  `spec/schemas/envelope.yaml` is about documents. The sentence after it, "Existing task
  authorizations remain valid", refers to nothing a newcomer can find.
- Least change: "One main repository and Apache-2.0 are accepted", and drop or explain the
  sentence on task authorizations.

**3D-22. `minor`. The pinned range for the first of the six dropped behaviors starts two
lines late.**

- Where: `docs/AUDIT.md:430`.
- Quote: "[MODEL.md, lines 218 to 221]".
- What the tree shows: at commit `7927d68` the sentence that states the behavior begins on
  line 216: "Derive the initial public result contract by default from declared public output
  bindings and constituent contracts". Line 218 starts in the middle of it. The other five
  ranges hold the text the table describes.
- Least change: "lines 216 to 221", and `#L216-L221` in the link.

**3D-23. `minor`. The decisions and the log of the last pass count its open points
differently.**

- Where: `docs/proposals/reaudit/round-2/DECISIONS.md:149-150`; `log-tail.md:141`.
- Quote, the decisions: "The pass listed eleven further points". The log: "The last pass
  listed fourteen points here. The coordinator settled eleven of them".
- Least change: in the decisions, "listed fourteen points, of which eleven are settled here".

## What a newcomer can rely on

Each of these was checked against the tree and found accurate.

- **What exists.** There is no engine, no reference model and no conformance vector. The
  directories `conformance/`, `conformance-held-out/` and `reference-model/` do not exist, and
  all 424 `vectors` lists are empty. Every entry document says so.
- **What binds.** `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md` and
  `spec/README.md` state one rule: the specification alone binds, and a behavior missing from
  it is a gap in it. The same one-sentence description of Conduit is in all five and in the
  overview, and the four outside `spec/` give the same reading order.
- **The counts of `spec/README.md`.** 424 promises, and the count of each of the 22 files;
  15 implementation-defined items; 99 exported documents, which are 21, 5, 6, 2, 11 and 2
  contracts, 7 templates and 45 schemas; 45 Schema contracts in 9 files; 7, 59 and 17 example
  files. All were recounted and agree.
- **The tracker's counts.** The defect register holds 167 rows: 48, 39, 30 and 50 by layer;
  3 closed and 84 ruled in the first two layers; 49 implementation-defined and 1 ruled in the
  runtime profile; 30 open for type contracts. The later-amendments table is right on fifteen
  items and five kinds of contract. The tally under A21 is right, and the three line numbers
  under "Known inconsistencies" that were checked are right.
- **The four checks** pass, and print what `AGENTS.md` and `CONTRIBUTING.md` say they print.
- **The records are reachable.** Every Markdown file in the repository is linked from at
  least one other, and every record under `docs/proposals/` from `docs/README.md` or from the
  README of its directory. The promise counts 395, 406, 410, 421 and 424 can be followed
  through `DECISIONS.md` and the logs of round 2, and the header comment of each promise file
  names the pass that changed each promise.
- **Who decided is stated in the records themselves.** `DECISIONS.md` and `GAPS.md` each say
  that a row is the coordinator's choice and that the owner can strike it. The findings above
  are about the entry documents and one header, not about these records.
- **The older material is marked.** Every file under `docs/protocol/`, `docs/authoring/`,
  `docs/proposals/authoring-syntax/` and `docs/research/`, the three superseded files under
  `docs/implementation/`, `docs/REVIEW.md`, `docs/VOCABULARY.md` and Experiment 002 carries a
  note under its title that it does not bind. The index of the archive carries the note for
  the archive.
- **The overview's worked example** is quoted exactly: both blocks occur character for
  character in the two files of the example project. Apart from 3D-12 to 3D-17, each sentence
  of the overview that cites a promise agrees with that promise as it stands.
- **The evidence is described honestly where it is kept.** Experiment 004 says that it is a
  tool evaluation and not conformance evidence, Experiment 002 that nothing in it was run, and
  Experiment 003 holds the 20 cases its README claims. `spec/README.md` says of the checks
  that "None of them is a conformance vector".
- **Links.** The checker finds no broken local link. The four GitHub addresses that were
  fetched, among them the pull request and one pinned file, answered without sign-in.
