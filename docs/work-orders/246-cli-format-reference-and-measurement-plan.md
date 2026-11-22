# Work order 246: `engine/CLI.md`, the CLI's format reference with nothing of S1's design, and the S1 rerun's measurement plan with the fresh author's pack

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order changes
Markdown alone. It needs no network and builds nothing; its one cargo command is the start
check's. Never print a whole large file, diff or log. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a
token or any credential. Do not run `conduit`, `codex`, `claude`, the dogfood harness or
`tools/gate.py`, and sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`,
and run the tool tests with `/usr/bin/python3`, which has PyYAML. Every file you write is
public: no absolute local path, user or machine name, or private repository name.

## What and why

**The gap.** v0's author bar is S1's rerun, by the coordinator and by a fresh Codex author, each
against the baseline A70
([IN_FORCE.md, topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement)).
The fresh author receives "the specification, the tasks and the authoring format's reference
shipped with the CLI, stated before the run"
([ruling 53](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-53/README.md),
point 9). No such reference exists. `conduit --help` gives usage lines only. The one reference
for plan and apply, [K3](../implementation/plan-and-apply/README.md), is written from S1's
tasks: its cases are T1 to T7 and its schedule and counts are S1's, so it cannot go in the pack.
And the [measurement plan](../implementation/first-build/measurement-plan.md) still gives the
2026-11-10 runs' binary ("as work order 025 leaves it"), Codex 0.167.0 and
`reference-model/examples/s1/`, and states no pack.

On 2026-11-21 the owner held the rerun and asked for the chain. This is the chain's docs order.

**Where the guide lives: `engine/CLI.md`**, beside `engine/README.md` and `CONFORMANCE.md`.
`spec/README.md`'s path for an author of documents says "This specification is the format
reference; the guide to a tool's commands belongs to that tool", and ruling 53 has the reference
"shipped with the CLI". Not taken: `docs/guides/plan-and-apply.md`, the chain's plan, since
`docs/guides/` holds the owner's setup guides; a section of `engine/README.md`, which walks through
S1. The cost: a change under `engine/` runs the gate's engine row.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): none
served and none at risk. No promise, case, engine source or package changes.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, as existing practice: a command line ships a reference beside its help, as git's
`Documentation/`, kubectl's reference and Terraform's pages for `plan` and `apply` do. Not taken:
`--help` alone, which cannot hold a text grammar, statuses or kept state; K3 itself, for the
reason above. Rule 7, one place per fact: the guide gives the commands' forms and cites `spec/`
for each rule `spec/` states; what it shares with K3 is under "What this leaves".

**When it runs.** After 241, 243, 263 (DEC-048's install) and 245 (`conduit plan`, which
corrected K3), so after 265 and 267; beside 247 to 249. Through Conduit's dogfood: one Codex
worker turn (aim for under 1.5 hours), a Codex verifier and a Claude reader
([ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md)),
and the gate
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). By
[ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md),
each decision says what must hold; how is yours. The file name, the headings and the pack's
wording are the coordinator's, open to the owner's strike.

## The decisions

1. **The sources, and which wins.** Plan and apply as K3 states them at your base, but where K3
   and `spec/README.md`'s "What the short form's meaning covers" differ, `spec/` binds. `init`, the
   engine context and the package cache as 243 built them, and `install`, with what it caches, as
   263 did, in the shell's code and K3's O7 and O8; where a text and the code differ, the code wins.
   Report each difference and the source you followed. Where a command or flag the guide describes
   is not in the shell's help table at your base, the guide's first paragraph names it in one
   sentence holding `not yet built`; with none missing, no such sentence.

2. **What `engine/CLI.md` holds**, each part once, in an order of your choosing:
   - What it is: not binding, `spec/` binds; the format is `spec/`'s, starting from
     `spec/README.md`'s path for an author of documents and `spec/promises/sections.yaml`; each
     command's own help is `conduit COMMAND --help`.
   - The engine context: what `conduit serve` records and where, the variable that moves that
     place, `--store DIR` as the override, and the stop with status 2 on a missing or stale
     context, as 243 prints it.
   - `conduit init`: both roles as 243 built them: bare, what it writes, the skeleton shown
     exactly, and what a repeat does; with `--store DIR`, that it initializes that store and
     writes nothing outside it.
   - `conduit install`: the closure walk, leaves first, one request per missing package; the
     places, in order (the cache, an absolute `file:` fetch, each `--mirror DIR`, the per-user
     `config.json`), an https or relative fetch still unbuilt, `(unbuilt)` in `tried=`; `--dry-run`;
     the form with no archive; the named archive's exact pin, up or down, never a dependency's or a
     `fetch`; its `present`, `install`, `pin`, `stop` and `not sent` lines and their fields, `from=`
     and `needed-by=` among them; the package cache: where, what it holds, that every working copy
     shares it, that install keeps each archive it reads whose digest matches, and that plan and
     apply only read it.
   - `conduit plan` and `conduit apply`: usage as the help table gives it where the command is
     there, and otherwise as K3's command table gives it; a flag K3 lists that the help usage
     lacks goes in the `not yet built` sentence; PROJECT_DIR's default; what each reads and
     sends (plan: `list.v1` and `inspect.v1` alone, writing nothing; apply: the same plan
     printed and kept, then sent step by step with no prompt, the command being the approval);
     plan never required; a CLI and an engine of different versions stop before anything is
     sent.
   - The approval flags: the seven, each repeatable, each with one generated name as its operand,
     and how a generated name is formed (SCT-006, SCT-007, SCT-009, SCT-013); what an unused,
     ambiguous or contradictory operand does; a table of the changes to a project and the flag
     each needs, from none for adding and narrowing to the stops that no flag permits,
     condensed from K3's difference table with each row's promise; narrowing before widening;
     that a plan stopped for a missing flag sends nothing, narrowing included; a cascade's
     printed pattern.
   - The text: the first and last lines; the step line and its fields; `(step N)`; the
     `detail`, `affects`, `difference`, `flags` and `stop` lines, the stop line's keys and the
     lines' order; the classes, as a table; apply's `result`, `not sent` and
     `panel action ID: sign-in required` lines. Examples only of K3's generic forms (a cascade,
     a no-op, a stop line), with names of your own.
   - Statuses: a table with rows `| 0 |`, `| 1 |` and `| 2 |` for plan and apply, `init`'s where
     243 gives it its own, and `install`'s as 263 gives them.
   - Stops, repeats and unknown outcomes: no automatic resend and no rollback; the key
     `plan:<hex digest>:<attempt>:<step>` and the attempt counter; an unknown outcome
     reconciled by the author first; an interrupted apply resumed from its journal.
   - Kept state: `.conduit/plans/HEX.txt`, `.conduit/state.yaml` and `.conduit/results/HEX.yaml`;
     what writes none; `.conduit/` not committed; not an authorization boundary; no absolute
     path, store path or credential in it, with the `pathSha256` fingerprint; the context and
     the cache outside every project.

   Each usage line for a command in the shell's help table is as `conduit COMMAND --help`
   prints it at your base.

3. **Nothing of S1, and nothing outside `spec/`.** No S1 name, case, schedule, count or design:
   no `example.org/s1` address, no name of an object of S1's or of K3's cases (`agent-a`,
   `shared--t`, `Pool/team` and the like), no T1 to T7, no example taken from a case. No
   ruling, decision, order, record or file outside `spec/` is cited, and no Markdown link leads
   out of the file, so that it reads the same in the pack: promises by ID, `spec/` files as
   plain paths. Definitions, Conduits and `packages` values in it are in flow style, as
   `spec/README.md` asks of guides; the skeleton is shown as `init` writes it. A name that
   `spec/names.yaml` assigns, such as `apiVersion`'s value, is written as it is there:
   `tools/set_names.py --check` reads `engine/`'s Markdown too.

4. **Links to it**, `../engine/CLI.md` in `docs/README.md` and `../../../engine/CLI.md` in K3.
   In `docs/README.md`, the `engine/` row links it, and the `implementation/plan-and-apply/` row
   calls K3 the design of plan and apply with its golden cases, not "the CLI reference", linking the
   guide as the authors' reference. In K3's opening paragraph, the sentence "What ships to the
   rerun's fresh author is the rerun preparation's decision ..." becomes one sentence: authors get
   the guide; this page, written from S1's tasks, is not in the fresh author's pack. K3's title
   becomes "Plan and apply: the design and golden cases", and its "the proposed CLI reference"
   becomes "the proposed design of plan and apply". Nothing else in K3 changes.

5. **The measurement plan.** In `docs/implementation/first-build/measurement-plan.md`:
   - "Who and what" gives the rerun's: the binary built at the rerun's pin, named in the results by
     commit; Claude Code 2.1.318 and Codex 0.167.1, each through the Agent package the pin ships;
     Node 21.5.0; each version read again after each run and given in the results;
     `engine/examples/s1/` as the example the author does not copy. No line there names order 025's
     binary, Codex 0.167.0 or `reference-model/`. "Two authors, apart" says the fresh author works
     from the pack below, linking `#the-fresh-authors-pack`, in place of "with only the
     specification, S1's tasks and what the engine ships".
   - "Provider work, counted apart" names the packages the pin ships as the results list them,
     by address, version and digest, in place of "the five packages ... in 0.1.0 and 0.2.0".
   - After the opening paragraph, one dated sentence: amended on the day you run by work order
     246, before the rerun; the runs of 2026-11-10 used the binary as work order 025 left it
     and Codex 0.167.0.
   - Under "The S1 rerun", a new last subsection, `### The fresh author's pack` (a straight
     apostrophe), in these words, with links to `../FIRST_BUILD.md#the-tasks`, to
     `../../../engine/CLI.md`, and to ruling 53 pinned as above:

     > The fresh author reads only these, each as at the rerun's pin: `spec/`; the tasks, T1 to
     > T7 of FIRST_BUILD.md, with the lab's paths; the CLI's format reference, `engine/CLI.md`;
     > and the text of `conduit --help` and of each command's `--help` (ruling 53, point 9).
     > The lab also holds what the tasks use: the binary, the archives they install, the
     > mirror directory T2's install reads, a store, and the files and directories the tasks
     > name. Nothing of S1's design goes in: not `engine/examples/s1/`, `conformance/`,
     > `docs/` (the plan-and-apply cases are S1's tasks), `experiments/`, the decision log or
     > the rulings, nor this page. Before the start,
     > a fresh reader checks the pack for S1's design; the results record what it found and
     > what changed.

   In `docs/implementation/FIRST_BUILD.md`, increment 4's row: "with only the specification and
   S1's tasks" becomes "with only the pack the measurement plan states", linking that
   subsection. Nothing else there changes.

## What this leaves

Report this list, corrected by what you did:

- 247 to 249 bring `engine/CLI.md` to what they build and drop their command or flag from its
  `not yet built` sentence; 250 checks that none is left, the guide against the final
  `--help`, and the guide for S1 content.
- If the owner's answer to 240's item 3 keeps the `sectioned-package` example in `spec/`, 250
  adds to the pack that it is left out and that the results say so.
- The text grammar, statuses and kept state are now in K3 and in the guide (rule 7). Once 250
  has run, K3's sections can cite the guide: a later order.
- `conduit --help` and `engine/README.md`'s author section naming the guide: 250's, with the
  README's move to plan and apply. IN_FORCE.md topic 13 and the handover's row 9: 250's.
- The 2026-11-10 pack also held the two Agent packages' READMEs; the rule above leaves them out
  unless the owner adds them.

## Stop rule

Stop, change nothing further, and report the gap and the options, without choosing, where: a
start-check line prints otherwise (a crate missing from the offline cache among them); K3 and
`spec/` differ on a point `spec/` does not settle; 243's `init` or 263's `install` contradicts
`spec/README.md`'s path for an author of documents, its sentence on pins among them; a part of
decision 2 cannot be written without S1's design; or a correct result needs a change outside
the scope, such as `help.rs`, `spec/` or K3's cases. A choice no promise governs is yours: list
it under `own_choices`.

## What to read

K3, all but "Case format and sequence", "Safety cases for 052a's convergence check" and "S1 commands
and counts under ruling 76"; `spec/README.md`'s "Reading paths" and "Limits of this version";
SCT-006, SCT-007, SCT-009 and SCT-013 in `spec/promises/sections.yaml`; the shell's `help.rs`, 243's
changes to `main.rs` and its tests, 263's `install.rs`, `install/pin.rs` and
`tests/install_closure.rs`; the measurement plan; FIRST_BUILD.md's increment 4 row and "The tasks".

## Before you change anything

From the repository root; stop, change nothing and report if a line prints otherwise. Line 8
holds once 245 has merged; under other markers, the coordinator corrects it before dispatch.

```sh
git merge-base --is-ancestor b2374c84 HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
test -d experiments/029-a84-plan-apply; echo "241 $?"
git grep -l 479ef47083c8 -- docs/implementation/plan-and-apply | wc -l | tr -d ' '
grep -c '^O8: Open' docs/implementation/plan-and-apply/README.md
grep -c 'CONDUIT_CONFIG_DIR' engine/crates/shell/src/help.rs
grep -c -- '--mirror' engine/crates/shell/src/help.rs
grep -c 'ACC-033 rows 6 and 6A' docs/implementation/plan-and-apply/README.md
test -e engine/CLI.md; echo "new $?"
sed -n '/^## Who and what/,/^## What the author writes/p' docs/implementation/first-build/measurement-plan.md | grep -cE 'order 025|0\.167\.0|reference-model/'
grep -c 'with only the specification and S1' docs/implementation/FIRST_BUILD.md
(cd engine && h=$(rustc -vV | sed -n 's/^host: //p') && RUSTUP_AUTO_INSTALL=0 cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "crates $?"
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py 2>&1 | tail -1
```

They print `main 0`, `0`, `241 0`, `0`, `0`, a number above 0, `2` and `1` (`0`
before 245); then `new 1`, `3`, `1`, `crates 0` and `Errors: 0`.

## Scope

Create `engine/CLI.md`. Change `docs/implementation/first-build/measurement-plan.md`,
`docs/implementation/FIRST_BUILD.md` (decision 5's sentence), K3's README (decision 4's title
and sentences) and `docs/README.md` (two rows). Change nothing else: not `engine/crates/`,
`engine/README.md`, `spec/`, `conformance/`, K3's cases, IN_FORCE.md, ROADMAP.md, the
handover, AGENTS.md or another order.

## Done when

Each line in the first group prints otherwise on `main` once 245 has merged ("fails today")
and passes after.

```sh
G=engine/CLI.md; M=docs/implementation/first-build/measurement-plan.md
test -f $G; echo "guide $?"  # guide 0; fails today (guide 1)
for t in 'conduit plan' 'conduit apply' 'conduit init' 'conduit install' 'conduit serve' '--store' '--reopen' '--recreate' '--prune' '--replace' '--adopt' '--with-resources' '--with-agents' '.conduit/plans/' '.conduit/state.yaml' '.conduit/results/' 'requires=' 'expected=' '(step ' 'not sent' 'panel action' 'no changes' 'stopped' 'ready' 'adds' 'narrows' 'widens' 'replaces' 'removes' 'neither' 'plan:' 'pathSha256' '--mirror' '--dry-run' 'config.json' 'needed-by='; do grep -qF -- "$t" $G 2>/dev/null || echo "missing $t"; done | wc -l | tr -d ' '  # 0; fails today (36)
for c in init install plan; do u=$(PYTHONDONTWRITEBYTECODE=1 python3 -c 'import re,sys; print(re.search(r"\(\s*\"%s\",\s*\"([^\"]*)\"" % sys.argv[1], open("engine/crates/shell/src/help.rs").read())[1])' "$c"); cat $G 2>/dev/null | grep -F "conduit $c" | grep -qF -- "$u" || echo "usage $c"; done | wc -l | tr -d ' '  # 0; fails today (3)
cat $G 2>/dev/null | grep -cE '^\| [012] \|'  # 3 or more; fails today (0)
test -f $G && ! /usr/bin/grep -qiE 'example\.org/s1|\bS1\b|\bT[1-7]\b|agent-[ab]\b|turn-[ab]\b|(shared|history|workspace|team)--|/(shared|history|workspace|team)\b|--[a-z-]+ (shared|history|workspace|team)\b|cascade-team|/lab/|cases/|plan-and-apply|\bK3\b|ruling|IN_FORCE|docs/|\]\([^#)]' $G; echo "clean $?"  # clean 0; fails today (clean 1)
grep -c 'engine/CLI\.md' docs/README.md docs/implementation/plan-and-apply/README.md  # 2 and 1; fails today (0 and 0)
grep -c "rerun preparation's decision" docs/implementation/plan-and-apply/README.md  # 0; fails today (1)
grep -c 'CLI reference' docs/README.md docs/implementation/plan-and-apply/README.md  # 0 and 0; fails today (1 and 2)
sed -n '/^## Who and what/,/^## What the author writes/p' $M | grep -cE 'order 025|0\.167\.0|reference-model/'  # 0; fails today (3)
for t in '0.167.1' 'engine/examples/s1/' 'work order 246' "### The fresh author's pack"; do grep -qF -- "$t" $M || echo "missing $t"; done | wc -l | tr -d ' '  # 0; fails today (4)
sed -n "/^### The fresh author's pack$/,/^## /p" $M | grep -cE 'engine/CLI\.md|conduit --help'  # 1 or more; fails today (0)
sed -n "/^### The fresh author's pack$/,/^## /p" $M | grep -c 'mirror directory'  # 1; fails today (0)
grep -c 'the five packages' $M  # 0; fails today (1)
grep -c 'what the engine ships' $M  # 0; fails today (1)
grep -c 'with only the specification and S1' docs/implementation/FIRST_BUILD.md  # 0; fails today (1)
grep -c 'measurement-plan.md#the-fresh-authors-pack' docs/implementation/FIRST_BUILD.md  # 1; fails today (0)
```

These pass, as on `main`:

```sh
git add -N engine/CLI.md  # so the checks see the new file
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py 2>&1 | tail -1           # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/set_names.py --check | tail -1         # ends Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py | tail -1       # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1  # OK, once
git diff --check  # prints nothing
git diff --name-only HEAD | grep -vE '^(engine/CLI\.md|docs/README\.md|docs/implementation/(FIRST_BUILD\.md|first-build/measurement-plan\.md|plan-and-apply/README\.md))$' | wc -l | tr -d ' '  # 0
wc -l < engine/CLI.md  # report it
git reset -q  # stage nothing
```

Your report also gives the guide's parts and line count, the `not yet built` sentence, if any,
and the measurement plan's new lines.

## Record

Not yet run.
