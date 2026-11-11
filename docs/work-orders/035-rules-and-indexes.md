# Work order 035: the contributor rules, the readers' guides, the roadmap and the indexes

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Step 2 of [the condensation plan](../proposals/condensation/plan.md) (read it, and its
[critic](../proposals/condensation/critic.md)), the part that does not touch `docs/AUDIT.md` or the
ruling records, which work order 034 does in parallel: its extractions E3, E19, E24 and E33, and the
map of `docs/README.md`. Also four stale statements the coordinator found while condensing the
rulings under [ruling 67](../proposals/ruling-67/README.md).

The plan was written before rulings 57 to 67 were numbered. Where it says "ruling 57" for the
acceptance of earlier logged decisions, read ruling 58, point 1; where it says "Accepted by ruling
57 (point 2)", write "Accepted by ruling 58 (point 1)". **C0** is
`f1cb473493fbb7b8667686d6fdf639ae96579ec3`.

## What to do

1. **E19, `CONTRIBUTING.md`**: as the plan lists, including the section "Writing promises", the
   removal procedure, and the rule that a specification head comment holds what the file holds,
   its standing, reading notes, the standards it follows, open issues and its register of removed
   IDs, and no change narrative (ruling 58, point 6; the register form is `Removed: OLD → NEW
   (ruling NN).` or `OLD: nothing replaces it (ruling NN).`). Add ruling 67's rule for a new
   ruling: one change writes its record, its ledger row in `docs/proposals/ruling-log.md`, the
   status lines and rows of the rulings it amends, IN_FORCE.md's lines, and any action item; an
   answer that changes no decision is added to the existing record, dated.
2. **E3, `docs/guides/cold-readers.md`**, as the plan lists.
3. **E24's one-line notes**, as the plan lists, with the ruling numbers corrected as above.
4. **E33, `ROADMAP.md`**: step 3's state in two lines; A77's and A71's states corrected from their
   records; A79 cited. Also: A78's row, whose trigger "once increment 3 runs" ruling 61 (B12)
   replaced: work orders go through Conduit after Fleet authority (batch 2), each worker an Agent
   of the `conduit-core` Fleet with a coordinator Agent appointed by root.
   And the sentence of "the order of the work" that has S1 and S2 also pass "in both independent
   builds": ruling 47 made the independent builds inform the owner's judgment without gating v0;
   say so, citing ruling 47.
5. **`docs/README.md`** becomes a map: what to read first (AGENTS.md's list), where decisions are
   (IN_FORCE.md, the ledger, the records), where the state of the work is (`docs/AUDIT.md`), and an
   index of the records that remain under `docs/proposals/`, one line each, including
   `ruling-67/`, `metrics/` and the ledger. Drop rows for what has left the tree. The ledger does
   not exist in your tree yet (order 034 writes it): name it as the code path
   `docs/proposals/ruling-log.md`, not as a link, here and in item 1; the admin links it at merge.
6. **The counting rules of ruling 63** go into
   `docs/proposals/first-build/increment-4/measurement-plan.md`, beside the rules it already
   applies, in ruling 63's words, with a link to the record.

## Scope

Change: `CONTRIBUTING.md`, `docs/guides/cold-readers.md`, `ROADMAP.md`, `docs/README.md`,
`docs/proposals/first-build/increment-4/measurement-plan.md`, and the files E24 names (one line
each). Do not change `docs/AUDIT.md`, `AGENTS.md`, IN_FORCE.md, `spec/`, a ruling record or the
ledger: order 034 changes those. Report anything else that needs changing.

## Done when

Every extraction is present and faithful to its C0 source (say how you checked). From the
repository root: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass.
Report the words of each changed file before and after. Do not stage anything.
