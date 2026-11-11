# Work order 062: the batch records' logs leave the tree, and the older records condense

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Action item A80 carries the rest of [the condensation plan](../proposals/condensation/plan.md)
(ruling 58; read its sections `target_structure`, `removal_list`, `extractions` and `sequence`, and
its [critic](../proposals/condensation/critic.md)). This order is the second half of the plan's step
5, "ruling and batch records" (removal list C): the logs of `core-and-schemas/` and `docs-audit/`,
the secrets record condensed in place (E18), the older records condensed in place by deleting
sections only (E20), and the move of the independent builds' assessment (E31).

The plan predates later work. Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger,
the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to
extract into them is present, and extract only what is missing. The plan's line numbers refer to
**C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line
in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

## What to do

1. **Extract first**: E35's "Choices that set behavior" in `core-and-schemas/README.md` and
   `docs-audit/README.md` from their logs; E20's spot-check of the 22 "left undone" items of
   `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-documents.md` and [`log-promises.md`](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-promises.md), any unsettled one reported for the
   tracker's "Known to be open"; E18, `secrets/README.md` condensed in place at the same path,
   dropping only the sections the plan names, and "[Promises that would follow](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)" only after mapping
   each drafted SEC-n to `spec/promises/secrets.yaml` or principles-review's dropped list (any draft
   sentence carried by neither is reported); E20 for the thirteen older records, the
   vocabulary-audit, multi-machine and the READMEs the plan lists, deleting sections only, with the
   plan's keep lists; E31, the "State on 2026-11-06" of `docs/implementation/INDEPENDENT_BUILDS.md`
   moved verbatim, row by row, into `ruling-39/README.md` as "The assessment point 8 confirmed", and
   replaced in INDEPENDENT_BUILDS by the state now for G8, G9 and G10, marked as the coordinator's
   until G11 (ruling 58, point 7).
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

Change: `docs/proposals/core-and-schemas/`, `docs-audit/`, `secrets/`, the records E20 names, `ruling-39/README.md` (the assessment, appended), `docs/implementation/INDEPENDENT_BUILDS.md`, `docs/archive/README.md`, and links and path mentions elsewhere that point into what this order removes. Do not change `spec/` (except a link target in a comment), `IN_FORCE.md`'s decisions, a ruling's decisions or an owner's words. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py` (it checks the C0 permalinks against Git
history), `python3 tools/check_rulings.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass. Do
not stage anything.

## Record

Run on 2026-11-11. The first dispatch stopped at the order's guard, as written, on five files whose
only changes since C0 were links repointed by earlier orders; the admin let such files go and
dispatched it again. The worker then took 14 minutes and a fresh verifier 6, then a second round.
It removed eight files, the logs of `core-and-schemas/` and `docs-audit/`, condensed the secrets
record and the older records in place by deleting sections only, and moved the independent builds'
assessment of 2026-11-06 verbatim into ruling 39's record, replacing it with the state now, the
coordinator's until G11: about 42,700 fewer words. The verifier found that the new G9 required both
builds to run before the gate let them start, and that bare file names and section references to
removed text had been missed; the second round removed the circular condition and linked 399
references to their C0 text, the records' words unchanged.
