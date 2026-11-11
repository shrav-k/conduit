# Work order 060: the re-audit's working papers leave the tree

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Action item A80 carries the rest of [the condensation plan](../proposals/condensation/plan.md)
(ruling 58; read the plan's sections `target_structure`, `removal_list`, `extractions` and
`sequence`, and its [critic](../proposals/condensation/critic.md)). This order is the plan's step 4,
"reaudit": its removal list A, with extractions E4, E5 and E35 for the re-audit's logs. Rulings 57
and 58 allow it: a document that does not bind may be removed once every piece of relevant Conduit
information in it is preserved in `spec/` or a live document; git keeps every version.

The plan predates later work. Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger,
the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to
extract into them is present, and extract only what is missing. The plan's line numbers refer to
**C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line
in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

## What to do

1. **Extract first**, as removal list A and E4, E5 and E35 say: E4 into `round-6/DECISIONS.md`'s
   "For the owner" (anchor kept); E5's condensations of `reaudit/README.md`, `round-5/README.md`,
   `round-5/log-fixes.md` and `round-6/log-integration.md`; E35's "Choices that set behavior"
   sections in `reaudit/README.md` and `round-6/README.md`, one line per choice that names a
   rejected alternative, a candidate for later, or a reading a promise does not state, copied
   verbatim where it fits, with a count per log.
2. **Then remove** the files removal list A names, and only those. Before deleting each, check it
   is unchanged since C0 (`git diff --quiet f1cb473 HEAD -- <path>`); if one changed, stop and report.
3. **Repoint every link** into a removed file or section, in three classes as the plan's sequence
   says: to the new home of text this order moved; to a live destination for a sentence that says
   where future work goes (list each for the admin); otherwise to its C0 permalink,
   `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>`
   (`/tree/` for a directory).
4. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group (what
   it was, what it is source material for, its last version at C0).
5. **Report** the words removed and kept, the extraction counts, every link repointed by class, and
   `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and
   archive rows.

## Scope

Change: `docs/proposals/reaudit/`, `docs/archive/README.md`, and links and path mentions elsewhere
(comments in `tools/` and `engine/` included: a plain path becomes the C0 permalink) that point into
what this order removes. Do not change `spec/`, `IN_FORCE.md`, the ledger, a ruling record's
decisions or an owner's words. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py` (it checks the C0 permalinks against Git
history), `python3 tools/check_rulings.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass. Do
not stage anything.
