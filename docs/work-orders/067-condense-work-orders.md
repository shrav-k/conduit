# Work order 067: merged work orders leave the tree for their record

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Ruling 58, point 4: a merged work order's file leaves the tree once its row is in the work-order
record. Action item A80 carries it; it is step 11 of [the condensation plan](../proposals/condensation/plan.md),
its extraction E30 (read it, and the plan's [critic](../proposals/condensation/critic.md)). The plan
names orders 001 to 029; since then orders 030 to 036k and 060 to 066 have merged too, and are in
scope. **C0** is `f1cb473493fbb7b8667686d6fdf639ae96579ec3`.

## What to do

1. **The record**, `docs/implementation/WORK_ORDERS.md`, section "The record": one row per merged
   order 001 to 036k and 060 to 066, as E30 says: worker minutes and rounds; verifier minutes and its
   must, should and nit counts; what the admin changed; tests or vectors at merge; findings; the
   order's own decisions, one line each in the order's own bolded words, with its reason where given;
   and any observation in its Record that no finding or package README carries, verbatim. Take each
   fact from the order's Record section and nowhere else; where a Record lacks a fact, write "not
   recorded". State once that the admin's own time was not recorded. Carry the two verbatim passages
   E30 requires (028's decision 5 and 027's Record on Codex's collaboration tools), the live
   correction to `packages/agent-codex/README.md`, and E30's lessons into "What this round taught".
2. **Then remove** the merged orders' files and their own `.schema.json` files, and only those. Keep:
   `004-i2-01-vectors-and-first-start.schema.json` (every order's report schema), `verify.md`,
   `verify.schema.json`, `README.md`, the directory `036b-staged/` (later orders apply its patches),
   any `*-declared.yaml`, and every order not yet merged (037, 038a, 039, 040 and this one). Before
   deleting a file, check that what its Record says is in its row.
3. **Repoint every link and mention** of a removed order, by path and by bare file name, to its row
   in the record (an anchor per row, or the section) where the sentence cites what the order did or
   decided, or to its C0 permalink where it cites the order's text, for example the spec batch 1
   plan's references to orders. Links in records keep their words.
4. **`docs/work-orders/README.md`** becomes the index of what is in the directory now, pointing to
   the record for what merged.
5. **Report** the rows written, the files removed, the words before and after, and every link
   repointed by class.

## Scope

Change: `docs/work-orders/`, `docs/implementation/WORK_ORDERS.md`, `packages/agent-codex/README.md`
(E30's correction), `docs/AUDIT.md` (A77's done-when: "each order's row", only), and links
elsewhere that point into what this order removes. Do not change `spec/`, `engine/`, `IN_FORCE.md`'s
decisions, a ruling's decisions or an owner's words. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py` and `git diff --check` pass. Do not stage anything.
