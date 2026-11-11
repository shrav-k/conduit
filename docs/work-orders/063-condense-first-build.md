# Work order 063: the first build's closed increments condense

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Action item A80 carries the rest of [the condensation plan](../proposals/condensation/plan.md)
(ruling 58; read its sections `target_structure`, `removal_list`, `extractions` and `sequence`, and
its [critic](../proposals/condensation/critic.md)). This order is the plan's step 6, "first build":
removal list B, with extractions E6 to E10, E12 and E13. Increments 1 to 3 are closed, so their
logs condense into one settlements register (ruling 58, point 3). Increment 4 is open: its log and
runs stay (its runs leave only after the S1 rerun), and only E9 touches it.

The plan predates later work. Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger,
the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to
extract into them is present, and extract only what is missing. The plan's line numbers refer to
**C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line
in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

Since the plan: `reference-model/` is `engine/` (order 033); work order 030 already did E14 and
E36 and the `s1_drafts.rs` choice; ruling 63 settled the rerun's counting rules, which order 035 put
in the measurement plan, so E11 adds only what ruling 63 did not settle and marks the rest settled;
the plan's A79 is now the short-form spike, so its lean-vector work goes under A39 (finding 034-F1).

## What to do

1. **Extract first**, as the plan's E6 to E10, E12 and E13 say: the settlements register in
   `docs/proposals/first-build/README.md` (one row per finding of increments 1 to 3: ID, decision,
   promises, vector, rejected alternative and why; D19 with its caveat verbatim; the two open rows;
   the two "noted for later" items); E7 into `increment-2/decisions.md`; E8's section "How to run
   the open checks" in the closeout results, with the sentinel method verbatim; E9 into increment 4's
   log and A68's done-when; E10's corrected mapping in `to-review-with-the-owner.md`; E12, "The
   core's boundary" in `engine/README.md`, rebuilt from the current code and not copied; E13's
   "Planned work" in `conformance/README.md`, with "Evidence of vector strength so far" verbatim.
   Check that what increment-2/plan.md's sections 4 to 7 hold is carried by `conformance/README.md`,
   `increment-2/decisions.md` or the code; report what is not, for the tracker's "Known to be open".
2. **Then remove** the files the list names, and only those. Before deleting each, check that
   since C0 it changed only in link targets or path mentions, which orders 033, 034, 035 and 060
   repointed (`git diff --word-diff f1cb473 HEAD -- <path>` shows only those); if anything else
   changed, stop and report that file, and go on with the others.
3. **Repoint every link and path mention** into a removed file or section, in the plan's three
   classes: to the new home of text this order moved; to a live destination for a sentence that says
   where future work goes (list each for the admin); otherwise to its C0 permalink,
   `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>`
   (`/tree/<C0>/<directory>` for a directory, with no trailing slash). Links in the ledger
   (`docs/proposals/ruling-log.md`) and in records are repointed too: their targets may change,
   their words may not.
4. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group.
5. **Report** the words removed and kept, the extraction counts, every link repointed by class, and
   `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and
   archive rows.


## Scope

Change: `docs/proposals/first-build/` (not `log-increment-4.md` beyond E9, and not increment 4's
runs), `docs/AUDIT.md` (A68's and A39's cells and "Known to be open" only), `docs/implementation/FIRST_BUILD.md`
(the owner's bar, E32, if it is not already there), `engine/README.md` (E12), `conformance/README.md`
(E13), `docs/archive/README.md`, and links and path mentions elsewhere that point into what this
order removes, comments in `tools/` and `engine/` included. Do not change `spec/` beyond a link
target in a comment, `IN_FORCE.md`'s decisions, a ruling's decisions or an owner's words. Report
anything else.

## Done when

From the repository root: `python3 tools/check_docs.py` (it checks the C0 permalinks against Git
history), `python3 tools/check_rulings.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass. Do
not stage anything.
