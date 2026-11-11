# Work order 034: the ledger of rulings, the condensed tracker, and records that say whether they hold

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

[Ruling 67](../proposals/ruling-67/README.md) settled how rulings are kept: the specification binds
behavior; [IN_FORCE.md](../../IN_FORCE.md) states every other decision in force; a ledger says what
was ruled, when, in the owner's words, and each ruling's status; a record says why, and opens with
whether it still holds. IN_FORCE.md is written. This order writes the ledger, condenses
`docs/AUDIT.md` (step 2 of [the condensation plan](../proposals/condensation/plan.md), its
extractions E1 and E2, with the plan's [critic](../proposals/condensation/critic.md)), adds the
records' status lines and the check. Read ruling 67, ruling 58 and the plan's sections
`target_structure`, E1 and E2 first.

The plan was written before rulings 57 to 67 were numbered. Where it says "ruling 57" for the
acceptance of earlier logged decisions, read ruling 58, point 1; its "WO-031" is this order.
**C0**, the commit every "last version" link of the condensation points to, is
`f1cb473493fbb7b8667686d6fdf639ae96579ec3`.

## What to do

1. **The ledger, `docs/proposals/ruling-log.md`** (new), in ruling 67's form, point 3:
   - First line under the title: "History. What holds now is [IN_FORCE.md](../../IN_FORCE.md); the
     specification alone binds behavior." Then ruling 67's precedence, in one paragraph.
   - One table: `| # | Date | Ruling | The owner's words | Status |`, one row per ruling, 1 to 67.
     *Ruling* is a gist of 25 words or fewer linking the record (or "no record" for rulings 1 to 6,
     9 to 12, 16, 26 and 34 to 36, whose only record is their row). *The owner's words* are
     verbatim and whole when they run to 40 words or fewer, copied from the record or the AUDIT
     row that quotes them; otherwise "In the record." Never excerpt. For rulings 1 to 37, where no
     words are marked as quoted, write "(the coordinator's summary)". *Status* uses only ruling 67's
     words: in force, done, directive (A-NN), open (points), amended by N (points), replaced by N,
     withdrawn; combine with semicolons where points differ. Take each "amended by" from the later
     ruling's own list of what it amends and from the tables of later amendments, and only from
     those; the admin's draft in the section below is a starting point to verify, not to copy.
   - Appendices, headed as history, verbatim, as E1 lists them: the preamble (AUDIT's ruling-log
     preamble), "Rulings 1 to 37 as given" (AUDIT's rows 1 to 37), the multi-system options under
     ruling 5, "Rulings before the plan audit", the tables of later amendments with rows added for
     rulings 45 to 67 from each record's list of what it amends, the round-5 changes, and the
     record-specific ID prefixes. Above the amendments tables: "The ledger's status column says
     what is current; these tables are kept as written."
2. **`docs/AUDIT.md`**, condensed as E2 says, with these changes from ruling 67:
   - Its "Ruling log" section becomes three lines: where the ledger is, where IN_FORCE.md is, and
     the precedence. Keep the headings `## Ruling log` and `### Later amendments` as stubs, so
     links to them resolve.
   - "Waiting for the owner" holds only what is waiting today: the metrics proposal
     (`docs/proposals/metrics/README.md`); what counts as a safety vector, which ruling 47's gate
     ("Every safety vector passes") needs (finding WO030-F1 of work order 030's record); A20, after
     the bypass probe A7; ruling 62's C6 to C12, after the S1 rerun; items 15 to 19 of the review
     list and increment 4's open decisions (ruling 58, point 1); four earlier rulings whose status
     is unclear (ruling 5's option (b), ruling 7's
     PL8 on fork-from-checkpoint, ruling 46's fallback of two Claude Code instances against ruling
     47's gate, and ruling 1's reference clones after increments 1 to 3); and the readiness gate's G8
     and G11. Rulings 59 to 66 settled the Projects rows, ruling 63 the counting rules and ruling 65 the
     public name: remove those rows. Do not add items of your own; report them.
   - "Known to be open" as E2 lists it, plus: findings 032-G1 and 032-G2 (the two passages of
     `spec/promises/secrets.yaml`'s head that no promise holds, in work order 032's record) and
     finding 4-18 (the core's speed, in increment 4's log).
   - Every heading that a tracked file links stays, as a stub if needed.
3. **Status lines.** Every `docs/proposals/ruling-NN/README.md` opens, under its title, with one
   line in ruling 67's form: `Status: <the ledger's status>. Holds now: <IN_FORCE.md's topic, by
   link, and the promises, where a promise holds it>. This record says why.` Replace the record's
   existing status sentence with it, keeping the date and "ruled by the owner" in it. A record that
   holds several rulings (for example `placement/`, `naming/`, `conventions/`,
   `schemas-and-interface/`, `reaudit/`) gets one line naming each. Change nothing else in a
   record: never the owner's words or the decisions.
4. **The check**, in `tools/check_docs.py` or a new `tools/check_rulings.py` run beside it, with a
   unit test under `tools/tests/`: one ledger row per ruling from 1 to the highest `ruling-NN`
   directory; each status cell uses only ruling 67's words; every `ruling-NN/README.md` has a status
   line as its first paragraph; no line of IN_FORCE.md's decisions holds a date. If you add a new
   tool, add it to AGENTS.md's list of checks and to `.github/workflows/checks.yml` if that file
   exists.

## The admin's draft of the ledger

A draft of the ledger's table, written from a reading of every ruling, follows this order under
"From the admin session". It is a starting point: verify every row against the record and the AUDIT
row it summarizes (the gist, the date, the owner's words character for character, the status
against each later record's list of what it amends), correct what is wrong, and report each
correction. Remove its bracketed marks such as `[10]`; they pointed at questions now carried in
`docs/AUDIT.md`.

## Scope

Create: `docs/proposals/ruling-log.md`, the check and its test. Change: `docs/AUDIT.md`, the status
sentence of each ruling or batch record under `docs/proposals/`, `AGENTS.md` (its list of checks
only), `.github/workflows/checks.yml` (one step, if you add a tool), and links elsewhere that point
into sections you move. Do not change `spec/`, IN_FORCE.md, an owner's quoted words or a ruling's
decisions. Report anything else that needs changing.

## Done when

The four items are done, and every extraction of E1 and E2 is present and faithful to its C0 source
(say how you checked). From the repository root: `python3 tools/check_docs.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (run it with the
interpreter that has PyYAML) and `git diff --check` pass. Report the words of `docs/AUDIT.md` before
and after. Do not stage anything.

## Record

Run on 2026-11-10 and 2026-11-11, in parallel with orders 033 and 035: the worker in 22 minutes,
then a fresh verifier in 7. The worker wrote the ledger of 67 rulings with its appendices,
condensed `docs/AUDIT.md` from 24,882 words to 8,414, gave 49 records their status lines, and added
`tools/check_rulings.py` with its tests; 17 extractions compared equal to their C0 sources. The
verifier found that three directive statuses named work orders instead of the action item that
carries them, and one history row mislabelled round 2's prefixes. The admin added action item A80
for the rest of the condensation plan, marked ruling 64's point 4 done (order 033 merged) and ruling
67's directives done with this order and 035, made the check require an action item on a
directive, and corrected the row. The worker also found that the plan's A79 had since gone to the
short-form spike (finding 034-F1); the lean-vector work stays under A39. The admin applied order
033's rename to this order's text and added the new check to continuous integration at merge.
