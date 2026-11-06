# Round 2, reader 4: log of the fixes

Editor's log of 2026-11-06. It records what was changed to carry out the findings of the
[fourth reader's report](reader-4.md), which read the whole repository cold. Finding numbers
are that report's: a section letter and the number of the finding in it.

Accepted by the owner in [ruling 39](../../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

Rules followed. Nothing under `spec/` was edited, because three other readers were reading it.
The four reader reports were not edited. The rows of the ruling log and the preservation tests
in `docs/AUDIT.md` were not rewritten. Nothing was committed. Each change is the smallest one
the report proposes, unless the row says otherwise. Where a finding needs the owner, the text
is left and the finding is listed under [Left for the owner](#left-for-the-owner).

## A. The first ten minutes

| Finding | What was done | Where |
| --- | --- | --- |
| A1 "Audit" names five things | The front page now says "the tracker (docs/AUDIT.md, called the plan audit)" and no longer says the specification was "audited". The tracker has a new paragraph under its opening: what the external audit was, that its report is not kept as a file, that it survives as the findings and the sections marked Historical, that ruling 1 means it, and what the later audits are called. One clause of it, on who made the audit, is not stated anywhere else in the repository; see [Left for the owner](#left-for-the-owner). | `README.md`, the Specification row; `docs/AUDIT.md`, second paragraph |
| A2 "Draft" and "accepted" of the same files | Done outside `spec/` only. The front page now relates the two words: draft means not complete and not a release, with no promise tested by a vector; "accepted by the owner" on a part means the owner has ruled on what it says, and the part is still in the draft. The sentence the report asks for at `spec/README.md:3` is not written. | `README.md`, the Specification row. `spec/README.md` is left for the editors of `spec/` |
| A3 The state is one step behind | "State on 2026-11-05" is now 2026-11-06 in both tables, and both say that round 2 is in progress. | `README.md`, "What exists"; `docs/AUDIT.md`, "Where things stand" |
| A4 Two families missing from the front door | The opening now names all five families, each in the words of `spec/definitions.md`. The overview's Interface row says where an Interface is. | `README.md`, third paragraph; `docs/OVERVIEW.md`, "The five families" |
| A5 "Type contracts" before its definition | Glossed: "the type contracts, which say how each type behaves and are still to be written". | `README.md`, "What binds" |
| A6 The order is long before the plain-language tour | One sentence under "Read these in order": the order is the one for work on the specification, and a reader who wants the idea first can read the overview first. The order itself is unchanged (decision DA10). | `README.md`, "Start here" |

## B. Consistency between the entry documents

| Finding | What was done | Where |
| --- | --- | --- |
| B1 Three checks in the specification, two in the build documents | Done outside `spec/` only. Both build documents now say that `spec/README.md` lists a third check, `check_docs.py`, that it is for the whole repository, and that it is not for the build base. The sentence the report asks for at `spec/README.md:155` is not written. | `docs/implementation/ENGINE_BUILD_BRIEF.md`, the `tools/` row; `docs/implementation/INDEPENDENT_BUILDS.md`, the paragraph on `tools/`. `spec/README.md` is left for the editors of `spec/` |
| B2 "The other two tools need only Python 3" was false | The tool was changed so that the sentence is true (F1), and the Checks sections now say what each check needs. | `tools/check_docs.py`; `AGENTS.md` and `CONTRIBUTING.md`, "Checks" |
| B3 "Runtime" used for the engine | "The language is the core and the engine keeps its promises." Ruling 6 in the log keeps its own words. | `AGENTS.md`, "The goal" |
| B4 "The only exception" | "The only exceptions are the Python checkers in `tools/` and Experiment 003, which is Python and predates the rule." | `AGENTS.md`, "Rules"; `CONTRIBUTING.md`, "Where work belongs" |
| B5 A stale count of the implementation-defined list | The row of the later-amendments table now says fourteen: ruling 32 added DL13, and decision DA3 of the documents audit added DL14. Checked against `spec/implementation-defined.yaml` and the conversion log. | `docs/AUDIT.md`, "Later amendments" |
| B6 "Being applied" where the specification says applied | The row now says that decisions DA1 to DA13 are applied, that eighteen of the nineteen behaviors are converted, and that the nineteenth, on secrets, is left for the owner. | `docs/AUDIT.md`, "Where things stand" |
| B7 Whether G1 is met | Partly. The row now says that no condition is recorded as met, what G1 has (the contents list, and decision DA1), and what is open under it (the behavior on secrets). It does not say that G1 is met: that is the owner's to say, as is the gate itself (G11). | `docs/AUDIT.md`, "Where things stand", the readiness gate row |

## C. The tracker

| Finding | What was done | Where |
| --- | --- | --- |
| C1 "Documents outside the specification" | As B6. | `docs/AUDIT.md`, "Where things stand" |
| C2 A32 | The status now says that eighteen of nineteen behaviors are converted, with the list and the log, and that what remains is the one on secrets (GP3), which waits for the owner. | `docs/AUDIT.md`, action A32 |
| C3 A61 | The status now says that the decisions are applied, that the fourth reader of round 2 could tell from the entry points what binds, where this log is, and that the action waits for round 2 to finish. | `docs/AUDIT.md`, action A61 |
| C4 A59 ends "Next: round 2" | Now "Round 2 is in progress, with four readers", with a link to its record. | `docs/AUDIT.md`, action A59 |
| C5 The later-amendments table | As B5. | `docs/AUDIT.md`, "Later amendments" |
| C6 A11 and Q3 say only "open" | A11 is now "in progress": the requirement is IFC-022, IFC-030 and item DL13, from re-audit decisions U10 and F2, and the mechanism waits for probe A4. Q3 points at it as in progress. Each promise was read before it was cited. | `docs/AUDIT.md`, action A11 and finding Q3 |
| C7 A21 cannot be evaluated | Each of the eleven items under "Known inconsistencies" was checked against the tree, and a sentence in italics after it says what was found. Three are overtaken, two partly, five stand as written in files that are marked as not binding or are not edited, and one waits for the owner. The items themselves are not reworded. A21 stays in progress and says this; it is not closed, because dropping an item is the owner's call. | `docs/AUDIT.md`, "Known inconsistencies" and action A21 |
| C8 No "what happens next" | A short section of three steps under "Where things stand": round 2 finishes; rounds are run until two in a row are clean; the vectors (A39) and the reference model (A38) follow. It says the work is at step 2 of the experiment program's order. Every statement in it is taken from an action's done-when, ruling 37 or the experiment program. | `docs/AUDIT.md`, "What happens next" |
| C9 Who wrote the ruling log, and from what | Not done. Only the owner can say whether each row is the owner's words or a summary, and whether the owner has read the rows. | Left for the owner |
| C10 PL1 against PL-1 | The identifier table now says that PL1 to PL8 and IN1 to IN10, with no hyphen, are decisions, and that PL-1 to PL-10, with a hyphen, are the earlier wording of the placement rules. Checked against both records. "The audit" in ruling 1 is covered by A1. | `docs/AUDIT.md`, "Identifiers" |
| C11 The purpose section says "the grant is an object" | Not done. The docs audit left the rewording to the owner, and the note above the section still says how to read it. | Left for the owner |

## D. The overview against the specification

Each promise was read under `spec/` before the sentence was changed, and each sentence now
says what the promise says. The header is re-dated to 2026-11-06.

| Finding | What was done | Where |
| --- | --- | --- |
| D1 An unknown outcome | Now: a repeat is dispatched as a new attempt only if the operation's contract declares `deduplicates: true` and the implementation is not marked `plain` (OPS-012); where no new attempt is dispatched, the outcome stays unknown until it is reconciled (OPS-013); reconciling is an actor recording that the effect occurred or did not (OPS-014). | `docs/OVERVIEW.md`, "Operations, outcomes and reasons" |
| D2 The exception in OBS-004 | Now: nothing acts to remove the difference without an explicit request, with one exception, an implementation restoring its own runtime where its type's contract allows it; the handle stays valid and nothing is dispatched because of it (OBS-005). | `docs/OVERVIEW.md`, "One lifecycle" |
| D3 Enforced and mediated | Now its own paragraph: what each word means, that this version specifies no isolation plugin, and that every restriction is therefore reported as mediated (ACC-027). | `docs/OVERVIEW.md`, "Conduits and access" |
| D4 `arguments` may be left out | Now: an instance document has `from` and the standard arguments of its family, which are named, and may have `arguments` (OBJ-013). | `docs/OVERVIEW.md`, "Documents, templates and instances" |
| D5 CON-002 cited for two things | The first half cites CON-002, and the second GRA-022 and CON-004. | `docs/OVERVIEW.md`, "Conduits and access" |
| D6 The Interface row | "Where a person, or a system outside, acts on the system", from the definitions. | `docs/OVERVIEW.md`, "The five families" |
| D7 Two conformance levels where there are three | Now names all three and who claims each, as `spec/README.md` does. | `docs/OVERVIEW.md`, "Three layers, and what is outside them" |

## E. Documents that would mislead a stranger

| Finding | What was done | Where |
| --- | --- | --- |
| E1 Experiment 002's banner | The banner is replaced by the one the authoring-syntax files carry, with three sentences of its own: nothing in it was implemented or run; two of its statements no longer hold; it stays until action A23, which is open, maps its cases. The link that called the overview the specification now says what that file was then and is now. Every word the banner names was checked to occur in the file. | `experiments/002-runtime-plugins/README.md`, the banner under the title and the status paragraph |
| E2 The archive's index contradicts its first line | The clause that the current review "remains the review entry point" is removed. "Linked to the active reference" now says the protocol topics were then the active reference. The last paragraph now says there is no active queue here and that the earlier queue is superseded. The index was already edited once for the same reason, under decision DA5. | `docs/archive/README.md` |
| E3 Six archive files with no note | The six files are not edited: `AGENTS.md` forbids editing the archive, and decision DA5 marked the index alone. The report's other option is taken. `AGENTS.md` now says that archived files carry no note of their own and that such statements describe the time they were written, and the archive's index says the same. | `AGENTS.md`, "What binds and what does not"; `docs/archive/README.md`, last paragraph. A note on each file is left for the owner |
| E4 Four documents that nothing links to | Round 2's record is linked from the tracker (A59, "Where things stand" and "What happens next"), from the re-audit record and from the documentation map, which has a new row for it. The conversion list and the three logs are linked from a new "Applied" section of the docs-audit record and from the documentation map. Round 2's record now links the four reports and this log. | `docs/AUDIT.md`; `docs/proposals/reaudit/README.md`; `docs/README.md`; `docs/proposals/docs-audit/README.md`; `docs/proposals/reaudit/round-2/README.md` |
| E5 The docs-audit record never says its decisions were applied | Its status now says that the thirteen decisions are applied. "What they concluded" is put in the past tense, under a sentence that says it is the state the readers found and not the state now. The new "Applied" section names the decisions each log covers. | `docs/proposals/docs-audit/README.md` |
| E6 The re-audit record reads as the current state | A paragraph under the status says that the page is the record of round 1, links round 2, and says that its counts are as they stood when each pass ended. "What comes next" is now "What came next", with the state of each step. "The specification now has 355 promises" is now "had", with a pointer to the contents table. No link pointed at the old heading. | `docs/proposals/reaudit/README.md` |
| E7 A standing instruction for one kind of session | Scoped, not removed: the instruction to commit and push to pull request 6 is now said to be for the sessions the owner has asked to write the language, and not for a reviewer or any other session. | `AGENTS.md`, "Rules" |
| E8 Experiment 004 cites a superseded document | It now cites ruling 3 of the ruling log in the plan audit. | `experiments/004-stateright-evaluation/README.md`, status line |
| E9 The conventions record misses an amendment | The entry for ruling 32 now lists it: an instance retired because its location was declared lost can be unregistered without its shutdown being confirmed (re-audit decision A2, LIF-017), which widens OL3. | `docs/proposals/conventions/README.md`, "Amended by" |
| E10 The naming research has no "Not binding" line | One line under the title: not binding; evidence for rulings 25 and 27; what it calls the naming proposal is now the accepted record; the rules in force are in `spec/promises/naming.yaml`. The packet's own text is not edited. The line used on the other packets says the research predates the specification, which is not true of this one, so the line is worded for it. | `docs/research/naming/README.md` |

## F. Hygiene and tooling

| Finding | What was done | Where |
| --- | --- | --- |
| F1 `check_docs.py` fails outside a Git checkout | The tool now finds the Markdown files in one of two ways and says which in its first line of output. In a Git working tree whose top is the repository it asks Git, as before. Anywhere else it walks the directory, skipping `.git/`, `target/` and what a `.gitignore` file plainly names. `--walk` asks for the second way in a Git working tree too. Files are read as UTF-8 whatever the locale. It still uses only the standard library, and its last line is still "Errors: N". Both ways were run; see [Checks](#checks). | `tools/check_docs.py`; `AGENTS.md` and `CONTRIBUTING.md`, "Checks" |
| F2 `check_schemas.py` passed because of what is installed | The Checks sections now give the install line, `python3 -m pip install pyyaml jsonschema`, and the versions it was last run with, PyYAML 6.0.1 and jsonschema 4.20.0. They describe what the tool does when a package is missing. That was checked by reading the tool and by running it both ways: with neither package it stops, says that nothing was checked and exits with status 2; with PyYAML alone it runs steps 1, 2, 5 and 6 without one rule, says that steps 3 and 4 and that rule were not run, and exits with status 3. The tool itself is not changed, and no requirements file is added. | `AGENTS.md` and `CONTRIBUTING.md`, "Checks" |
| F3 More superseded text than specification | Nothing to change, as the report says. When the protocol topics and the authoring-syntax packet move to the archive needs a ruling. | Left for the owner |
| F4 Two details of the owner's machine and organization | Not done. The docs audit left the private repository's name to the owner, and the sibling directories are an instruction of the owner's. | Left for the owner |
| F5 Nothing says the fourth tool is safe to skip | "Four Python tools in `tools/`: three checks that are run before each commit, and the record of one past migration." The documentation map says the same. | `README.md`, the Code row; `docs/README.md`, the `tools/` row |
| F6 No accidental file | Nothing to change. | — |

## G. The ranked list

| Item | Carried out by |
| --- | --- |
| G1 Bring the tracker up to the last two commits | B5, B6, C1 to C5, A3 |
| G2 Make round 2 and the docs-audit logs reachable | E4, E5 |
| G3 Correct what the documents say the checks need | B2, F1, F2 |
| G4 Fix the overview | D1 to D7 |
| G5 Experiment 002's banner | E1 |
| G6 The archive's index, and the six archive files | E2, E3 |
| G7 Scope the standing instruction | E7 |
| G8 "What happens next", A21, A11 and G1 | C8, C7, C6, B7 |
| G9 "Draft" against "accepted", and the three checks, in `spec/README.md` | Not done in `spec/README.md`. Covered from outside it by A2 and B1 |
| G10 "Audit", the five families, "engine" for "runtime", Experiment 003 | A1, A4, B3, B4 |

## Left for the owner

- **Who made the external audit (A1).** The new paragraph in `docs/AUDIT.md` says it was made
  "at the owner's request, by a Claude session that had not written the plans it read". No
  document in the repository says this. It comes from the notes of the session that opened the
  tracker and from the trailer of commit `76a8f10`. The owner should confirm the clause or
  strike it.
- **Whether readiness condition G1 is met (B7).** The tracker states what G1 has and what is
  open under it, and leaves the judgment.
- **Who wrote the rows of the ruling log, and whether the owner has read them (C9).**
- **The purpose section's "the grant is an object" (C11).**
- **Whether to drop the five items of "Known inconsistencies" that stand as written, and the
  stale branch `implementation/rust-runtime-plugins` (C7).**
- **Whether the six archive files get a one-line note of their own (E3).**
- **When the protocol topics and the authoring-syntax packet move to the archive (F3).**
- **The sibling directories named in `AGENTS.md`, and the private repository named in
  `docs/AUDIT.md` (F4).**

For the editors of `spec/`, once the readers have finished:

- `spec/README.md:3`: one sentence that relates "draft" to "accepted by the owner" (A2, G9).
- `spec/README.md:155`: say that `check_docs.py` is for the whole repository and not for a
  checkout that holds `spec/` alone (B1, G9). The paragraph can also say that the tool now
  works without Git.
- The overview was checked against the promises as they stood when this log was written. The
  fix pass of round 2 changes some of them, so the overview needs one more reading after it.

## Checks

Run from the root of the repository after the last edit. Each exited with status 0.

```text
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 115 Markdown files and 1480 local links (offline).
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x401, apiVersion x208, metaPrefix x63. Errors: 0

$ python3 tools/check_schemas.py        (first and last lines)
Python 3.9.6 at /Applications/Xcode.app/Contents/Developer/usr/bin/python3; PyYAML 6.0.1; jsonschema 4.20.0
Errors: 0

$ git diff --check
(no output)
```

`git diff --check` does not read a file that Git does not track yet, which this log was when
the checks ran. It was searched for trailing spaces separately, and has none.

The second way of `check_docs.py` was run on a copy of the repository made without its `.git`
directory, outside the repository:

```text
$ python3 tools/check_docs.py
Files: found by walking the directory, because this directory is not a Git working tree. Skipped: .git/, target/ and what .gitignore plainly names.
Checked 115 Markdown files and 1480 local links (offline).
Errors: 0
```

Three more runs on copies, each with the same count of files and links and no error:

- With no `git` command on the path, the first line gave the reason "the `git` command was
  not found".
- With the copy placed inside another Git working tree, the reason was "this directory is not
  the top of a Git working tree".
- With `--walk` in the repository itself.

What the walk skips was tested on a copy by adding files with a broken link. A file under
`target/`, a file under a second `target/` elsewhere, a file whose name `.gitignore` lists and
a file in a directory `.gitignore` lists were all skipped. A file matched only by a wildcard
line was read and its broken link reported, because a wildcard line is not read.

`set_names.py --check` and `check_schemas.py` gave the same result on the copy as in the
repository.
