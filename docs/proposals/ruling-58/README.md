# Ruling 58: how the repository is condensed, and the form of ruling records

Status: **ruled by the owner on 2026-11-10** (plan-audit ruling 58). This is the record of a
ruling. Where it and the specification differ, the specification governs.

Ruling 57 revoked the rule against editing historical documents and left open the form of ruling
records and how duplicated content is removed. The [condensation audit](../condensation/README.md)
proposed a plan in ten points, the first of which ruling 57 had already settled; the coordinator put
the other nine and a standing rule. The owner: "I like everything and all your recommendations!"

## The decisions

1. **Decisions the owner could strike.** Every coordinator decision, editor choice and work-order
   decision logged before ruling 57 is accepted, as ruling 39 did, except items 15 to 19 of the
   [review list](../first-build/to-review-with-the-owner.md) and the open decisions of increment 4,
   which stay open to the owner until reviewed. Decisions that changed an earlier ruling become
   recorded amendments.
2. **The form of ruling records.** `docs/AUDIT.md` keeps one line per ruling (number, date, a gist
   of 25 words or fewer, the record, and what amended it). `docs/proposals/ruling-log.md` holds
   verbatim what has no other home: rulings 1 to 37, the rulings before the plan audit, and the table
   of later amendments. Each ruling keeps one record file in a fixed form: status; what was put,
   with the options; the owner's answer verbatim; the decisions; what it amends; the promises it
   produced. No merged register re-keys the decision rows that the specification cites.
3. **A closed increment's log** becomes rows of one settlements register (finding, decision,
   promises, vector, rejected alternative and why), and the log leaves the tree. The open
   increment keeps its log until it closes.
4. **A merged work order's file** leaves the tree once its row is in the work-order record.
5. **Evidence.** Dated results, the method as run, the prompts and the counted artifacts stay as
   v0's evidence; raw run records leave the tree with links to their last version. Increment 4's
   runs stay until the S1 rerun. Draft ruling 47's "reported with their dated observations" is read
   this way.
6. **History inside `spec/`.** The change narrative in the specification's comments and READMEs is
   condensed to what each file holds, its standing, its standards, its open issues and its register
   of removed IDs, checked by script so that the parsed data is identical and every retired ID stays
   named; no promise changes. Specification files carry no change narrative from now on.
7. **The independent builds' assessment of 2026-11-06** is replaced by the state now, marked as the
   coordinator's until G11; the confirmed assessment moves verbatim into ruling 39's record.
8. **The Project investigations' papers** give way to [a brief](../projects/brief.md); the papers
   leave the tree once the Project rulings are recorded.
9. **The index of removed files** has one row per removed directory or group of files, with what it
   was, what it is source material for and its last version, at a pinned commit.
10. **Condense at close**, a standing rule: when a batch, an increment or an investigation closes,
    its working papers condense into the registers and the records, so duplication does not grow
    back.

## What follows

Codex work orders in the plan's sequence, each reviewed by the coordinator for lost content; the
`AGENTS.md` and `CONTRIBUTING.md` rule that ruling 57 revoked is removed first.
