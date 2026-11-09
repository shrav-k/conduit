# Ruling 45: log of the roadmap

Date: 2026-11-09. In [ruling 45](README.md) the owner asked the coordinator to "do a pass over
the docs and decide the cleanest way to display the roadmap", since the five experiments of pull
request 10 were each already planned and could not be found. This is A72 of the
[plan audit](../../AUDIT.md), whose done-when reads: "A reader can tell from the entry points,
without reading this tracker, what is next and whether an experiment they have in mind is
already planned."

The pass first mapped every place that stated a fact of the roadmap, walked a cold reader's
questions from the README, listed the identifiers that collide, and read how other projects show
planned work. Three designs were then written and judged: keep the roadmap in a section of the
plan audit, give it a page of its own, or extend the experiments index. The coordinator chose the
page of its own, with parts of the other two. This log records what moved and the coordinator's
decisions. The owner can strike any of them.

Files: `ROADMAP.md` (new), `docs/AUDIT.md`, `README.md`, `AGENTS.md`, `CONTRIBUTING.md`,
`docs/README.md`, `experiments/README.md`, `spec/README.md` (one sentence),
`docs/implementation/INDEPENDENT_BUILDS.md` (one line), `tools/check_roadmap.py` (new) and this
log. No promise changed. Nothing under `docs/archive/` or `docs/research/` was edited, and the
record of ruling 45 was not edited.

## What moved

Each block was moved, not copied: the plan audit keeps only a heading with a link where a link
from elsewhere may lead to it. Line numbers are those of `docs/AUDIT.md` at `d46a52b`.

| Lines of `docs/AUDIT.md` | What | Where it is now | Changed on the way |
| --- | --- | --- | --- |
| 105-137 | "What happens next", items 1 to 6, each marked done | Not moved: rulings 39 to 45 are rows of the [ruling log](../../AUDIT.md#ruling-log), with the records they link; round 5 (item 3) is in the row "How it was checked" of [where things stand](../../AUDIT.md#where-things-stand), in A59's status and in [round 5's record](../reaudit/round-5/README.md); D06 and D12 of round 6 (item 5) are in [waiting for the owner](../../AUDIT.md#waiting-for-the-owner), among what is settled; and the merges of pull requests 7, 8, 9 and 11, and the closing of 10, are in the history of pull requests in `AGENTS.md`, which now records the merge of 8 as `2ca84f3`. The heading stays, as a [pointer](../../AUDIT.md#what-happens-next) | The list is gone, so its numbers, which records of earlier rounds cite, are no longer reused |
| 138-147 | "What happens next", items 7 and 8: the first build, and the next read | [Order of the work](../../../ROADMAP.md#the-order-of-the-work), step 3 and the row "Read" | Step 3 also names the loader first (A43), as ruling 45 has it, and says that it is X4 of pull request 10 |
| 783 | A68's link to the program | Stays in the plan audit, linking the [roadmap](../../../ROADMAP.md#round-6s-evidence-and-measurements) | Its action names the strict loader in Rust first (A43); its status links step 3 and [waiting for the owner](../../AUDIT.md#waiting-for-the-owner) |
| 784-785 | The rows of A70 and A71 | [Experiments and probes](../../../ROADMAP.md#experiments-and-probes) | Re-cut into the register's columns; every fact of the action, done-when and status cells is kept |
| 794, part of A63's status | What Experiment 005 measured of the keychain | Already in [Experiment 005's record](../../../experiments/005-macos-keychain/README.md#what-it-means-for-a-macos-keychain-provider-package), which A63's status now links; the untested daemon is A22's | Removed from A63, since it was a fourth summary |
| 794, part of A63's status | Round 6's measurements of the providers: D15, D16, D01 and a package whose server repeats a named argument (MAN-010) | [Round 6's evidence and measurements](../../../ROADMAP.md#round-6s-evidence-and-measurements), rows S2, S7 and S10 | The S10 row takes A63's fuller words on D15 and D16; a new S7 row holds D01 and MAN-010's package |
| 794, part of A63's status | The probe of 1Password with no person present (X1 of pull request 10) | A new item, A73, in [the register](../../../ROADMAP.md#experiments-and-probes) | Its needs say that the case of the desktop app comes to the owner before it runs, as A63 said |
| 800-805 | The rows of A4, A5, A6, A7, A10 and A8 | [Experiments and probes](../../../ROADMAP.md#experiments-and-probes) | Each question is taken from the [standing probes](../../archive/plan-audit-history.md#standing-probes-and-comparisons) of the plan audit's history, which is not edited; each source is written "probe E1 of the plan audit" |
| 812 | The row of A19, superseded | [Done, removed, dropped or not adopted](../../../ROADMAP.md#done-removed-dropped-or-not-adopted) | Its done-when is folded into what it asked |
| 814 | The row of A22 | [Experiments and probes](../../../ROADMAP.md#experiments-and-probes) | None |
| 824-826 | The heading and date of the program | [The experiment program](../../../ROADMAP.md#the-experiment-program); the heading stays in the plan audit as a [pointer](../../AUDIT.md#experiment-program), since the history and records link it | "Amended by later rulings, among them 16, 40, 41, 44 and 45" added |
| 828-844 | The claim and the table of methods | [The experiment program](../../../ROADMAP.md#the-experiment-program) | Cold-reader sweeps cite A59 in place of A42, which closes the triage; multi-machine runs cite S3, A74 and A40 |
| 846-862 | The scenarios S1 to S10 and the acceptance paragraph | [Scenarios](../../../ROADMAP.md#scenarios) | A Step column: S1 in step 3, S10 in step 4, S2 to S9 in step 5. One sentence links the question [open in the order](../../../ROADMAP.md#open-in-the-order) and settles nothing |
| 864-880 | The order, steps 1 to 8 | [Order of the work](../../../ROADMAP.md#the-order-of-the-work), one row each, with their numbers and words | Items, needs and a state for each step |
| 882-883 | The note on the vocabulary | [The experiment program](../../../ROADMAP.md#the-experiment-program) | None |
| 885-887 | The work in parallel throughout | [Order of the work](../../../ROADMAP.md#the-order-of-the-work), row "In parallel" | The keychain's daemon case and A74 added |
| 889-890 | Ruling 45's rule that each experiment states, before it runs, what it must show and how each measure is counted | [Proposing an experiment](../../../ROADMAP.md#proposing-an-experiment), its only copy | None |
| 892-896 | The independence rules for the builds | [The experiment program](../../../ROADMAP.md#the-experiment-program) | The link is repointed from the root |
| 898-900 | The removal of Experiments 001 to 003 | [Done, removed, dropped or not adopted](../../../ROADMAP.md#done-removed-dropped-or-not-adopted) | None |
| 902-919 | Round 6's evidence and measurements | [Round 6's evidence and measurements](../../../ROADMAP.md#round-6s-evidence-and-measurements); the heading stays in the plan audit as a [pointer](../../AUDIT.md#round-6s-evidence-and-measurements), since round 6's decisions link it | A first line says that every E and D in it is round 6's; the S1 row names A70, E17 names A71, and the S7 and S10 rows are as above |

Not from the plan audit: the experiment across two machines, which ruling 43 accepted with the
[multi-machine proposal](../multi-machine/README.md#11-an-experiment-across-two-machines) and
which nothing tracked, is A74 in the register. The proposal keeps its text. X2 of pull request
10, which ruling 45 dropped, and the candidates of the collaborator's pull request 1 that were
not adopted, which the archive's
[review](../../archive/collaborator-proposals/REVIEW.md#additional-roadmap-candidates) holds,
have rows among what is done, dropped or not adopted.

## Other edits

| File | What changed |
| --- | --- |
| `docs/AUDIT.md` | The preamble says what this tracker records, that the roadmap holds the order and the program, and that the two hold the state of the work. Identifiers: the E row says that the plan audit's standing probes, E1 to E5, E8, E10 and E11, are action items with rows in the roadmap, written "probe E10 of the plan audit", and that its earlier ideas E0, E6, E7, E9 and E12 were superseded by action items whose rows are in the plan audit; the S row links the roadmap; rows for X1 to X5 of pull request 10 and for H1 to H10 of A74. How to use: where the roadmap is, and that whoever starts an item writes `in progress` with the branch or pull request. Where things stand: dated 2026-11-09, with a first row that links the roadmap; the specification row names ruling 45's six core terms; the row "How it was checked" says that the text ruling 45 changed has not been read cold either; the code row names the new checker and the count of `check_schemas.py`'s rules that its run reports, 28 from 27 promises. "For the next reader" links the roadmap. "Who decides what" names this log. "Rulings 1 to 45". The ruling log's dates reach ruling 45. Priorities: one sentence, that the order of the work is the roadmap. Action items: one sentence, that the experiments and probes have their rows in the roadmap, and among what is done there once done. A59's status ends with a link to the row "Read" in place of a wording of the order. A68, A63 and A72 as above and below. Waiting for the owner: the row of A68 lists the inputs to the discussion, and the row of A20 links A7's row in place of stating its state |
| `README.md` | Dated 2026-11-09. A row "Roadmap" in "What exists". The rows "How it got here" and "Reference model" point to the roadmap in place of their two wordings of the next milestone. The roadmap is named among what does not bind, and as row 4 of "Start here". The code row counts five tools, four of them checks. The sentence on the tracker links where the work stands and what waits for the owner each to its own section |
| `AGENTS.md` | The first build is step 3 of the roadmap, in place of a third wording of it. "Read first" adds the roadmap, and "Also not binding" names it. Evidence: look for the item in the register first, and follow its rules. The history of pull requests adds the merge of 8, as `2ca84f3`, then 9, 10 and 11 and this pass. The checks add `check_roadmap.py` |
| `CONTRIBUTING.md` | The reading order adds the roadmap, and "What binds" names it as not binding. "The state of the work is tracked in the plan audit, and its order and its experiments and probes in the roadmap." Evidence: check the register first. The checks add `check_roadmap.py`, with what it needs and covers |
| `docs/README.md` | The reading order adds the roadmap. "Learn the state of the work" starts at the roadmap, and a row "Propose an experiment or a probe" is added. The map lists `ROADMAP.md` as not binding, the AUDIT row says what the plan audit now holds, the tools row adds the checker, the experiments row points to the register, and the row of ruling 45 links this log |
| `experiments/README.md` | A paragraph: the register lists every experiment and probe, this index those with a directory; check the register before proposing, and follow its rules, which say how a directory is numbered. The program's link goes to the roadmap's scenarios. "Probe E10" is "Probe E10 of the plan audit (A22)" |
| `spec/README.md` | The latest change is ruling 45, of 2026-11-09, which added six core terms; before it, ruling 44 settled round 6. Nothing else under `spec/` changed |
| `docs/implementation/INDEPENDENT_BUILDS.md` | One line under the status: the gate and the builds are step 7 of the roadmap |
| `tools/check_roadmap.py` | New, below |

## The coordinator's decisions

Each is the coordinator's, under ruling 45's "decide the cleanest way to display the roadmap",
and the owner can strike any of them.

1. **A page at the root holds the roadmap.** `ROADMAP.md` holds the order of the work, the
   register of experiments and probes, how to propose one, and the experiment program that
   ruling 9 accepted, with its scenarios and round 6's evidence. It says first that it does not
   bind. A72's done-when asks for a reader who does not read the tracker, so a section of the
   tracker would have met it only by a reading of those words; a root file is also the name a
   search for "roadmap" finds, where it found only the archived proposal of pull request 1.
2. **The statements that there is one tracker are reworded.** The plan audit's preamble, line
   34 of `CONTRIBUTING.md` and the AUDIT row of `docs/README.md` now say that the plan audit and
   the roadmap between them hold the state of the work: the plan audit the rulings, the owner's
   queue and the action items that are not experiments or probes, and the roadmap the order and
   the row of each experiment and probe.
3. **The rows of the experiments and probes moved.** A4 to A8, A10, A19, A22, A70 and A71 have
   their only rows in the roadmap, so that "is this already planned?" is answered in one table.
   The other action items stay in the plan audit. Every item keeps its number.
4. **Two new items, and a number held.** A73 is the probe of 1Password with no person present,
   which ruling 45 put "later (A63)"; A74 is the experiment across two machines that ruling 43
   accepted, with its hypotheses H1 to H10. The directory `experiments/006-across-machines/`,
   which the accepted proposal names, is held for A74: a new directory takes the next unused
   number, counting the numbers the roadmap holds.
5. **The state of step 2.** "Done as a step, as the coordinator's note": every area is written
   as promises (A33), and ruling 44 put the build next. Its items are not done: some questions
   and repairs, such as A17 and A20, are still open, none has a vector, nothing is
   model-checked and the next sweep comes after step 3; A13's done-when, "ruled, model-checked
   and swept clean", is not met. Step 2's items are grouped by kind, not by status, so that a
   change of status does not leave a label stale.
6. **Rows without a number.** "In parallel", which is ruling 9's work in parallel with what was
   added to it since, and "Read", "After" and "Later versions", which are new, carry no number,
   so the eight steps keep theirs. Ruling 9 accepted seven; step 4 and S10 came with the
   provider layer of ruling 40, and the page says so. The next read is a row of its own after
   step 3, not part of it. A50 is cited in step 5 with the question of what runs S2 to S9
   left open, below the table, for the owner with A68: the program says the reference engine
   (A50), and ruling 44 runs S1 through the reference model (A38). This pass settles neither.
7. **A63 trimmed.** Its status keeps what is said nowhere else: it needs A62, where its
   providers come in the order, and links to Experiment 005, A22, A73 and round 6's table. Each
   sentence removed is in Experiment 005's record, A73 or the table, as above.
8. **The inputs to the discussion of the first build.** The row of A68 under
   [waiting for the owner](../../AUDIT.md#waiting-for-the-owner) lists them: the scope that pull
   request 10 proposed, as ruling 45 corrected it; the success criteria that round 6's
   [critique](../reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage)
   suggests for a first author trial; and the question in decision 6.
9. **A checker.** `tools/check_roadmap.py`, in Python's standard library like the other
   checkers, fails when an action item from A1 to the highest has no row or two rows across the
   plan audit, the roadmap and the history, when either page cites an item with no row, when an
   item of the roadmap sits outside its register, when an item not yet done, superseded or
   dropped is not in the order's Items column, when a directory under `experiments/` is missing
   from its index or the roadmap or shares a number, when an entry point does not link the
   roadmap, or when a state does not begin with an allowed word. It cannot tell whether a state
   is true: the coordinator updates the states in the pull request that changes the work.
10. **Left alone.** The assessment of the readiness gate of 2026-11-06 in
    `docs/implementation/INDEPENDENT_BUILDS.md`, which the plan audit already lists as not made
    again, gains only a line that points to step 7. The table of standing probes in the plan
    audit's history keeps the statuses ruling 39 left, since the archive is not edited; the E row
    of the identifiers says where the live rows are. The records that link the program's old
    anchors still resolve, to the headings kept as pointers, and were not edited. A22's state
    says `in progress` with no branch or pull request: it began before the rule that asks for
    one, and its state links Experiment 005, which carries the work.

Some facts still stand in two places, and no check covers them; the coordinator keeps each pair
in step. A68's action and step 3 both name the pieces of the first build. A63's candidates and
the rows of A22 and A73 both name the stores the probes serve. The index of experiments and
the roadmap's rows of Experiments 001 to 005 both say what each did. The priority P1 says that
the native probes run beside it, as the row "In parallel" does.

## Checks

Run from the root of the repository on the result:

```sh
python3 tools/check_docs.py          # Errors: 0
python3 tools/set_names.py --check   # Errors: 0
python3 tools/check_schemas.py       # Errors: 0
python3 tools/check_roadmap.py       # Errors: 0
git diff --check                     # no output
```

`check_roadmap.py` reported 41 rows of action items in the plan audit, 12 in the roadmap and 21
in the history, which together hold A1 to A74 once each; 50 items not finished, each cited by
the order of the work; and 28 states checked.
