# Ruling 67: decisions in force, a ledger, and records that say why

Status: in force (coordinator under delegation, strikeable) (delegated by the owner on 2026-11-10, decided by the coordinator; the owner may strike or change any point). Holds now: [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed). This record says why.

The owner may strike or change any point. This is the record of a ruling. Where it and the
specification differ, the specification governs.

The owner raised that the rulings were growing: "We should discuss how to dedup and condense the
rulings, it's starting to get large, and will only become more so". The coordinator proposed two
documents, the decisions in force by topic and a compact chronological ledger. The owner agreed
provisionally and asked for research: "I also provisionally agree on rulings, but I feel maybe some
research into other projects or Claude best practices might be good here as well". The coordinator
had both researched ([research](research.md)) and a critic review the result. The owner then
delegated the choice: "I trust whatever strategy you will come up with to condense these rulings.
Continue uninterrupted and present me with the rulings when I ask in the morning".

## The decisions

1. **Four documents, each answering one question, with one order of precedence.** The
   specification says what Conduit does, and alone binds behavior. [IN_FORCE.md](../../../IN_FORCE.md),
   *decisions in force*, says what else has been decided and still holds: every decision in force
   that `spec/` does not hold, and every accepted language decision not yet written there, marked
   *spec pending*. The ledger, `docs/proposals/ruling-log.md`, says what was ruled, when, in the owner's words,
   and each ruling's status. A ruling's record says why. A line of IN_FORCE.md that differs from
   the owner's words in a ruling is a defect: the owner's words govern until the line is fixed. A
   *spec pending* line tells the session that writes `spec/` what to write; builds and vectors
   follow `spec/` until it is written, so a ruling still changes no behavior until it is in `spec/`.
2. **IN_FORCE.md.** One line per decision, grouped by topic in the terms of `spec/definitions.md`,
   each written whole, with its scope and its exceptions in the line, citing its rulings by number
   and point (for example "ruling 59, A3"). A decision `spec/` already holds gets one pointer, never
   a restatement; a topic `spec/` wholly holds gets one pointer line. No new identifiers: rulings
   keep their numbers, which `spec/` cites (ruling 58, point 2). No dates, no state of the work and
   no one-off directives. When a later ruling changes a decision, its line is rewritten whole, not
   extended. Decisions the coordinator logged that the owner may still strike say so in their line.
   Questions the owner holds open have a line saying so, so that no session settles them.
3. **The ledger** is `docs/proposals/ruling-log.md`, with one row per ruling, 1 onward: number;
   date; a gist of 25 words or fewer linking the record; the owner's words, verbatim and whole when
   they run to 40 words or fewer, otherwise a pointer to the record that holds them (never an
   excerpt); and a status: *in force*, *done* (a one-time act carried out), *directive* with the
   action item that carries it, *open* with the points still open, *amended by N* with the points,
   *replaced by N*, or *withdrawn*. Rulings 1 to 37 keep the coordinator's summaries, with the
   owner's words where marked (ruling 39, point 15). The verbatim history ruling 58 kept (rulings 1
   to 37 as given, the rulings before the plan audit and the tables of later amendments) stays
   verbatim, as appendices headed as history; the status column holds what is current. The ruling
   log in `docs/AUDIT.md` becomes a pointer to the ledger and to IN_FORCE.md. This amends ruling
   58, point 2.
4. **Records say why, and say first whether they still hold.** Each record opens with its status
   line: its status in the ledger's words, and where it holds now (IN_FORCE.md's topic, or the
   promises). A record that holds several rulings gives each. The owner's words and the decisions as
   ruled are never rewritten: a change goes in the later ruling's record, and only the earlier
   record's status line changes. The rest of a record may be condensed (rulings 57 and 58).
5. **Fewer rulings.** A new ruling number is given only when the owner decides something new. An
   answer that confirms or clarifies a ruling without changing a decision is added to that ruling's
   record, dated, and to its ledger row; ruling 66's sixth decision is the pattern.
6. **One change carries a ruling everywhere.** A ruling's change writes, together: its record, its
   ledger row, the status line and ledger row of every ruling it amends or replaces, the lines of
   IN_FORCE.md it changes, and any action item. When a specification batch lands, its *spec pending*
   lines become pointers. Condense at close (ruling 58, point 10) applies to the research behind a
   ruling as to any working paper.
7. **What agents read.** `AGENTS.md`, which every Claude session loads and every Codex session reads,
   keeps only what every session needs, and stays near 150 lines: no per-ruling summaries, no
   history of pull requests, no dated state of the work. Its first reading list names IN_FORCE.md by
   link, never by import. `spec/README.md`'s list of accepted rulings not yet written becomes one
   pointer to IN_FORCE.md's *spec pending* lines, so the list exists once.
8. **A check.** `tools/check_docs.py`, or a step beside it in `tools/`, checks: one ledger row per
   ruling from 1 to the last record; the status words; a status line at the top of every
   `ruling-NN` record; and no date in IN_FORCE.md's lines.

## What it amends

Ruling 58, point 2: the ledger leaves `docs/AUDIT.md` for `docs/proposals/ruling-log.md` and carries
the owner's words; IN_FORCE.md and the records' status lines are added. Ruling 58's other points
stand.

## What was declined

- **Line identifiers** for IN_FORCE.md (for example `SCOPE-2`): they would clash with the
  specification's promise prefixes and add a scheme beside ruling numbers.
- **Restating `spec/`** in IN_FORCE.md: two copies of behavior drift apart.
- **Quoting excerpts of long answers**: choosing which of the owner's words to keep risks misquoting.
- **Gating adoption on an evaluation** with fresh sessions: one run per case would be noise, and git
  makes the change reversible. A smoke check with fresh sessions may be run later as evidence.
- **Typed relations per point with a parser for them**: one status per ruling, with points in free
  text, is checked more simply.
