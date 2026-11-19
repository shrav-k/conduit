# Work order 211: A78's stage 2: its claim, measures, pass conditions and stop rules, registered before its first turn

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order changes two documents. It has no
network access and needs none. It needs no cargo command; if you run one, run one at a time. Do
not run `conduit`, `codex`, `claude`, the dogfood harness or anything else against a store, and
sign in nowhere. Never print a whole large file: read `ROADMAP.md` and
`experiments/028-a78-dogfood/README.md` with `grep -n` and `sed -n`. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never read or print a key,
a token or any credential.

## What and why

**What it registers.** [A78](../../ROADMAP.md#experiments-and-probes) has Conduit run its own
work orders through Agents of the `conduit-core` Fleet
([ruling 70](../proposals/ruling-70/README.md); [ruling 61](../proposals/ruling-61/README.md), B12).
Stage 1 ran order 042 on one slot, and experiment 028 registers orders 2 and 3.
[Order 210](210-a78-stage-2-fleet-documents.md) wrote stage 2's documents: the store initialized
again once order 050n merges the Agent packages 0.2.0, with two pairs of a Codex worker and a Codex
verifier and one Claude reader, `reader-1`, on Claude Opus, each keeping its client's login in a
home of its own (order 050c), naming the owner's primary account of its client (order 050f) and
signed in once by the owner through `conduit sign-in` ([order 051b-2](051b2-cli-sign-in.md)). 210
registers nothing, and says that a dated amendment registers stage 2's claim, measures and pass
conditions before its first turn ([ruling 45](../proposals/ruling-45/README.md)), and records the
coordinator's reading of what `reader-1`'s reach waits for, or the owner's answer. This order is
that amendment. It also sets stage 2's stop rules, says what stage 2 cannot show, and brings A78's
row of the roadmap up to date. [Ruling 89](../proposals/ruling-89/README.md) has since set the
first dogfood milestone: the Agents of the owner's primary Claude and Codex accounts signed in
through Conduit on the Mac, running Conduit's work orders one turn at a time (point 3), the Claude
reader slot joining only once D58's guard is in the harness (point 5). Stage 2 is that milestone's
run, and this registration follows the ruling.

**What stage 2 runs.** Root sends Conduit's own work orders as turns: each order's worker and
verifier on one pair's turn Conduits, in that pair's slot clone, and the fresh read on Claude
Opus before the merge ([ruling 79](../proposals/ruling-79/README.md)) as a turn of `reader-1`
through Conduit, in a clone of its own. Until [order 203](203-new-conversation.md)'s
`new-conversation.v1` ships with the packages 0.2.1, a fresh conversation restarts the engine. The
harness sends one turn at a time on a store of 0.2.0, or of 0.2.1 if it is first initialized with
them. Turns at once are registered later, by their own amendment. Until `reader-1` joins, the four
Codex Agents carry the stage and each fresh read runs outside Conduit, as the method runs it.

**What it does not do.** It does not change stage 2's documents, the harness (which stays outside
the tree), the method or A77's registration, nor orders 2 and 3's claim, measures or pass
conditions, except that, if they run in stage 2, their setup is stage 2's and their conditions are
read as stage 2's conditions 1 to 7 are, by this amendment.
By the coordinator's reading, orders after A78's order 3 go through Conduit before order 203 on
the owner's word that ruling 89, point 3, records, one turn at a time, and lifting the stops it
marks as the owner's stays the owner's. That reading leaves
[IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed) saying that A78's orders 2
and 3 run through Conduit and the plain runner carries the rest (ruling 80, point 7): the
coordinator reconciles that line by a change of its own, merged before this order, and this order
qualifies only A78's row of the roadmap (decision 5). Every choice here is the coordinator's, open
to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)):

- reading ruling 89's milestone 1 as the owner's word for orders after order 3, and its "milestone
  1 may begin with its Codex slots" as letting the store be initialized before D58's guard, with
  `reader-1` added once the guard holds and signed in before its own first turn, as items 2 and 3
  of order 210's list before stage 2's first turn allow, item 3 still binding the Codex Agents;
- two more pass conditions beside stage 1's seven: no sign-in asked again of an Agent the owner
  signed in, and every read fresh;
- the coordinator's reading of 028's "its reach over the worker's tree waits for ... (D59)" as a
  Conduit or a directory of `reader-1` naming a worker's slot, which `reader-1` does not have;
- a turn of `reader-1` is the pre-merge Opus read of the method and never A77's paired read,
  since a paired reader is given only the verifier's inputs and runs no command that writes, and
  `reader-1` runs the order's checks;
- stage 2's claims are judged over its first five orders, each fixed as stage 2's when it is
  dispatched, so that no order is picked after its outcome is known;
- five stop rules, each a pause that takes A78 off nothing
  ([D54](../proposals/full-review/decisions.md)).

**Amended before dispatch.** On 2026-11-19, before this order was dispatched, the coordinator
amended the end of decision 2's block, its reading of condition 4's guard of the reader's clone. The
block said that no `__pycache__` is excepted because the harness sets `PYTHONDONTWRITEBYTECODE` for
its Agents, so none is written. That overstated: the Agent packages pass their clients only six
names of the environment (`HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `USER`) and the client's own
home variable, so `PYTHONDONTWRITEBYTECODE` does not reach Python that an Agent runs, which can
still write one. The harness, as changed before stage 2's first turn, removes a new `__pycache__/`
that holds only `.pyc` files after each turn, before the guard reads, and notes it. The guard now
reads as the harness reads it, its exceptions stated once: a rewritten Cargo.lock, restored; a new
`target/` or `packages/NAME/dist/`; and that `__pycache__/`; any other new ignored path breaks it.
The amendment is the coordinator's, open to the owner's strike.

## What to read

- [experiments/028-a78-dogfood/README.md](../../experiments/028-a78-dogfood/README.md): "What it
  cannot show", the whole of "Orders 2 and 3: registered before order 2 runs", above all its
  Measures, Pass conditions and "The Claude verifier slot", and the last section, "Stage 2: the
  multi-slot Fleet, written before 0.2.0", which order 210 added.
- Stage 2's instances, which order 210 added in `stage-2/fleet/` of experiment 028: the five
  Agents and root's five turn Conduits.
- [IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed): its paragraphs on who
  reads each order, on the allowances, and on Conduit building Conduit.
- [Ruling 89](../proposals/ruling-89/README.md), points 3 to 7, and A77's amendment of 2026-11-19
  pausing its pairing, in [work orders](../implementation/WORK_ORDERS.md).
- A78's row in [ROADMAP.md](../../ROADMAP.md#experiments-and-probes), by `grep -n '^| A78 |'`.

## Before you change anything

From the repository root, check that you find what this order was written against, and stop,
change nothing and report if any line prints otherwise:

```sh
git ls-files --error-unmatch docs/work-orders/211-a78-stage-2-registration.md >/dev/null; echo "order $?"
git ls-files --error-unmatch experiments/028-a78-dogfood/stage-2/fleet/instances.yaml >/dev/null; echo "210 $?"
git ls-files --error-unmatch docs/proposals/ruling-89/README.md >/dev/null; echo "89 $?"
git status --short | wc -l
R=experiments/028-a78-dogfood/README.md
grep -c '^## Stage 2: the multi-slot Fleet, written before 0.2.0$' $R
grep -c '^## Stage 2: registered before its first turn$' $R
grep -c "It is added only once D58.s guards are in place" $R
tail -n 1 $R | grep -c 'holds for every result of stage 2\.$'
grep -c '^| A78 |' ROADMAP.md
awk '/^## 15/,/^## 16/' IN_FORCE.md | grep -c 'the plain runner carries the rest (70 point 5, 80 point 7)\.$'
```

They print `order 0`, `210 0`, `89 0`, `0`, `1`, `0`, `1`, `1`, `1` and `0`. The third `grep` finds
the ruling-84 correction of "The Claude verifier slot". The last shows that the coordinator's
change reconciling IN_FORCE.md's topic 15 with this order has merged: that change rewrites the
line's last sentence, on A78's orders 2 and 3 and the plain runner, or adds one after it. If it
prints 1, that change has not merged: stop and report. If another change has moved the README's
last line, stop and report.

## The decisions it applies

Each block below marked "word for word" is written so, with DATE replaced by the date you write
it, as YYYY-MM-DD. Decision 2's block is word for word because a pre-registration's claim, pass
conditions and stop rules are the decision itself, and the sections around them fix what those
mean. Do not rewrap a block: its lines are at most 100 characters, and each link sits
whole on one line, because `check_docs.py` checks a link only when its text and its target share
a line. Where a decision lists what a part must say, write it in your own words, in the README's
style: the terms the registration of orders 2 and 3 uses, lines of at most 100 characters, every
link whole on one line, and no private value. The README is two directories below the root, as
this order is, so its links are written as they serve from the README.

1. **The README's opening.** In `experiments/028-a78-dogfood/README.md`, after the line that ends
   "for orders 4 and 5 are registered before order 4 runs.", add these two lines, word for word,
   inside the same paragraph.

```markdown
Stage 2, on the store initialized again with the Agent packages 0.2.0, or 0.2.1 if it is first
initialized with them, is [registered below](#stage-2-registered-before-its-first-turn).
```

2. **The registration is the README's new last section.** After order 210's section, the README's
   last, add one blank line and then this block, word for word, with the two sections between the
   markers written as decisions 3 and 4 say, each in place of its marker line.

```markdown
## Stage 2: registered before its first turn

Registered on DATE by work order 211, before stage 2's first turn
([ruling 45](../../docs/proposals/ruling-45/README.md)), after the section above wrote stage 2's
documents. It holds for every order of stage 2: an order that root dispatches to go through
Conduit on the store initialized again with the Agent packages 0.2.0, or 0.2.1 if it is first
initialized with them, one turn at a time either way, fixed as stage 2's at its dispatch. Stage 2
ends when the owner ends it or when a dated amendment lets turns run at once; a turn sent while
another is in flight is not stage 2's. An upgrade of the store's Agent packages, or a new
initialization, ends stage 2 unless a dated amendment merged before it says otherwise. This text
changes only by a dated amendment, merged before the first turn it changes. Its choices are the
coordinator's, each open to the owner's strike
([ruling 48](../../docs/proposals/ruling-48/README.md)).

### Stage 2's orders

Root sends each order through the turn Conduits of
[stage 2's instances](stage-2/fleet/instances.yaml): the worker and the verifier of one pair work
in that pair's slot clone, and `reader-1` gives the order its fresh read on Claude Opus before it
merges ([ruling 79](../../docs/proposals/ruling-79/README.md)), in a clone of its own. Each Agent
keeps the login that the owner gave it once, through `conduit sign-in`, in a home of its own.

- If order 2 or 3 has not run when stage 2 starts, stage 2's first orders are those orders. They
  each get two verdicts: A78's, under the seven conditions of
  [their registration](#orders-2-and-3-registered-before-order-2-runs), and stage 2's, under this
  section's nine. The setup recorded below serves as theirs, and their conditions are read as
  stage 2's conditions 1 to 7 are read below.
- [Ruling 89](../../docs/proposals/ruling-89/README.md), point 3, sets the first dogfood
  milestone: the Agents of the owner's primary accounts, signed in through Conduit on the Mac, run
  Conduit's work orders one turn at a time. The coordinator reads it as the owner's word for an
  order after order 3 to go through Conduit before
  [order 203](../../docs/work-orders/203-new-conversation.md), one turn at a time. Such an order
  is an order of stage 2, judged here, and does not count toward A78's five, whose orders 4 and 5
  a coordinator Agent sends.
- No order that changes this experiment's directory, or the harness, is sent through Conduit in
  stage 2. An order's text and its report schema are on `main` before its first turn.
- Each order of stage 2 keeps the directory, the redactions and the result that orders 2 and 3
  keep ([What counts](#what-counts-and-what-each-outcome-changes)). The account values that its
  sign-ins and account checks return are private values: the records export carries them, and
  they are replaced in the export by the placeholders of the section above before it is copied.
  Its result opens with "Not binding.", then the first paragraph of What it cannot show and the
  first paragraph of [What stage 2 cannot show](#what-stage-2-cannot-show), then its verdict.

### `reader-1`, D58, D59 and A77

- `reader-1` is signed in and sent turns only once D58's guard is in the harness, as the setup
  below records it ([ruling 89](../../docs/proposals/ruling-89/README.md), point 5), and after the
  Claude scratch-home sign-in of
  [027's amendment](../027-a5-sign-in/README.md#amendment-of-2026-11-19-the-live-check-as-it-runs)
  has passed; if that sign-in failed or ended a fault, only once the owner, told of it, has
  answered. Until then the four Codex Agents carry stage 2, and each order's fresh read runs
  outside Conduit as the method runs it. Stage 2's store may be initialized before then, as
  item 2 of the section above's list allows: it is then rendered without `reader-1` and
  `turn-reader-1`, and both are added from stage 2's documents once the guard holds. Item 3 of
  that list then holds for the four Codex Agents before stage 2's first turn, and for `reader-1`
  before its own first turn.
- `reader-1` is [the Claude verifier slot](#the-claude-verifier-slot). The coordinator reads its
  "reach over the worker's tree", which waits for the owner's answer on D59, as a Conduit or a
  directory of `reader-1` that names a worker's slot. `reader-1` has neither: the harness brings
  each order's branch into its own clone. Its own tools, like every unconfined Agent's, still
  reach the worker's slot. If the owner strikes this reading, `reader-1` is sent no turn until
  the owner has answered D59.
- A turn of `reader-1` is the fresh Opus read before the merge, as
  [the method](../../docs/implementation/METHOD_AS_RUN.md#6-the-opus-reader-and-fixes-by-hand)
  gives it, sent through Conduit. It is never A77's paired read: a paired reader is given only
  what the Codex verifier is given and runs no command that writes, and `reader-1` runs the
  order's checks, so the Opus reader of A77's paired read is started outside Conduit, as A77
  registers it. A77 pairs no order until order 050n merges (its amendment of 2026-11-19), and
  stage 2 starts after it: an order of stage 2 may then be paired, and its paired Opus read is
  still started outside Conduit. How an order's A77 record gives a turn of `reader-1`, and which
  verifier an order on the second pair has there, are A77's to say, by a dated amendment of its
  own section.

### Stage 2's claim

For each of its orders, stage 2 claims what [the claim](#claim) of orders 2 and 3 claims, on the
store initialized again with the Agent packages 0.2.0, or 0.2.1 if it is first initialized with
them, one turn at a time either way: the four Codex Agents and, once it joins, `reader-1`, each
signed in once by the owner and keeping its client's login in a home of its own. It adds two
claims:

1. the owner's one sign-in of each Agent carries the stage: no turn waits on a sign-in asked
   again;
2. the fresh read on Claude Opus before each merge can be a turn of `reader-1` through Conduit:
   fresh, valid against its schema, and leaving its clone unchanged. Only an order whose fresh
   read was a turn of `reader-1` shows it.

It claims nothing that orders 2 and 3 do not: nothing about access (D57), Agents kept apart (D60)
or speed (D54). It claims nothing about turns at once, which stage 2 does not send, and nothing
about how well `reader-1` reads beside the reader it replaces, since no read of `reader-1` is
paired with another reader's.

SETUP-SECTION

MEASURES-SECTION

### Stage 2's pass conditions

An order of stage 2 passes when all nine hold, and a condition not shown fails it, as for orders
2 and 3. Conditions 1 to 7 are [those of orders 2 and 3](#pass-conditions), read for stage 2 so:
a turn is a turn on any Agent of stage 2; in condition 1, the plain turn beside a turn of
`reader-1` is a plain Claude Code run on the same machine and account; in condition 2, set-up
seconds run until each of root's turn tools on the store answers; in condition 3, a report of
`reader-1` is valid against the reader's schema that the setup records; and in condition 4, the
guards include D58's guards on the harness's own git directory and files, and the guard of the
reader's clone, which holds when the turn left its HEAD and its branch unmoved, the harness's
`git status` of the clone is empty and no ignored path is new since the turn's start, except
these: a rewritten Cargo.lock, restored and noted; a new `target/` or a package's
`packages/NAME/dist/`, noted; and a new `__pycache__/` that holds only `.pyc` files, which the
harness removes after the turn, before the guard reads, and notes. Any other new ignored path
breaks the guard.

8. **No sign-in is asked again of an Agent the owner signed in.** No sign-in action is raised, for
   an Agent of stage 2 that the owner has signed in, from the order's first restart to its last
   end, the window counting from that Agent's last sign-in by the owner, unless the result shows,
   by a log line it cites, that the provider ended that login itself. An action raised by a
   setup's account check that failed or ended unknown counts, and the result names it apart.
9. **Every read is fresh.** Each turn of `reader-1` and each verifier's turn, but a repair turn,
   is the first of a new conversation of its Agent: it is preceded on that Agent by a restart of
   the engine or, where installed, by a `new-conversation.v1` that succeeded, as the harness's log
   shows it. A read that is not fresh is not ruling 79's read, and nothing merges on it.

Stage 2's claims hold when each of its first five orders passes. An order that fell back to the
plain runner before its first turn stays among the five, its turns counted as lost under
condition 1, or as not shown, with the reason. Each order has its result. Once
the fifth has its result, or stage 2 ends with fewer, a result for the stage gives the verdict,
and with fewer than five orders it says that the claims are not shown. With no turn of
`reader-1` among the five, it says that claim 2 is not shown.

### Stage 2's stop rules

A stop pauses stage 2: the harness sends no further turn through Conduit, the coordinator tells
the owner that day with the rule and its evidence, and the orders in flight go on on the plain
runner, where each lost turn runs again (D54). A stop does not take A78 off the core chain; that
stays the owner's (D54). The coordinator lifts a stop once its cause is a row of the first build's
open log and the fix, or why none is needed, is in the order's result, except where a rule leaves
the lift to the owner.

1. **A guard answers broken.** That includes D58's guards on the harness's own git directory and
   files, the guard of the reader's clone, and a write that the sweep sees by an Agent outside its
   own slot or clone and its own state directory. The lift is the owner's when the write reached
   another Agent's state directory, a login, the main checkout or a remote.
2. **An Agent acts under an account it does not name.** A setup's account check answers another
   account, as the records give it. The lift is the owner's.
3. **A sign-in is asked again.** When an Agent the owner signed in raises a sign-in action, that
   Agent is sent no turn until the owner signs it in again through `conduit sign-in`, and the
   other Agents go on. A second time on any Agent stops stage 2. Only the owner signs an Agent in.
4. **Two turns in a row are lost** to a fault of the engine, an Agent package or the harness, as
   condition 1 reads a lost turn, or a turn is recorded as not having occurred (condition 5).
5. **The engine does not come back.** A restart on the store does not bring back root's turn
   tools within twice condition 2's bound, or the store does not start.

An allowance's end is a pause, not a stop. When the ChatGPT Codex allowance runs out, the Codex
Agents' turns wait for it; when the Claude allowance runs low, A77's paired reads stop first and
dispatch next, and nothing merges unread
([IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed)). No API key is used, and
no Agent names one. A read of `reader-1` that is lost, or still invalid after its repair turn, is
run outside Conduit as the method runs the fresh read, so that nothing merges unread.

### What stage 2 cannot show

Each Agent keeps its client's login in a home of its own, which keeps the accounts apart for the
clients but is no wall against an Agent that runs code
([ruling 84](../../docs/proposals/ruling-84/README.md), point 2): under one user, any Agent of
stage 2, `reader-1` among them, can read or use another's login, and each one's own tools reach
every slot. Each can also read what the owner's environment and shell profile hold, an API-key
variable among them, a reach the owner accepts for the dogfood
([ruling 89](../../docs/proposals/ruling-89/README.md), point 4), and stage 2 shows nothing about
whether an Agent used it. A sign-in that carried the stage shows that it did so on this store,
for this stage's length, not across a package's upgrade or the install of 0.2.1 (A78's P4, not
registered). An account check that answered the named account shows what the client said, not
which account the provider charged. One turn at a time shows nothing about turns at once: two
pairs show five Agents kept in one store and two slot clones used in turn, not two orders run
together. And no read of `reader-1` is paired with another reader's, so stage 2 shows nothing
about how well it reads.

### What stage 2 leaves unregistered

- Turns at once. Once order 203's `new-conversation.v1` is installed with the packages 0.2.1,
  root's turn Conduits gain it by a change of stage 2's documents, and a dated amendment, merged
  before the first turn sent while another is in flight, registers what that adds: the restarts
  and waits it removes, both pairs at once, and whether a store of 0.2.0 takes the Agents of
  0.2.1 without new sign-ins (A78's P4).
- A third pair, a second account, which comes after the S1 rerun's pin and only on the owner's
  word ([ruling 84](../../docs/proposals/ruling-84/README.md), point 3; ruling 89, point 7), the
  Linux node as a node of the Fleet, and a tree granted through the Pool (orders 4 and 5, D59):
  each by its own registration or amendment, before the first turn it changes.
```

3. **Stage 2's setup** replaces the line `SETUP-SECTION`. It is a section headed
   `### Stage 2's setup, recorded before its first turn`, whose first paragraph says that the
   coordinator records each item below in this section, by a dated amendment merged before stage
   2's first turn, and that the first result of stage 2 repeats each, with the command that
   shows it where one does. Then one bullet for each of these items:
   - the commit of `main` the dogfood engine runs, and that it holds the merges of orders 050n
     and 051b-2, each shown by `git merge-base --is-ancestor`, and the two ancestor checks of
     orders 2 and 3's setup (`b1f6f8ca` and `60126251`); and whether the order that removes the
     `reconcile.v1` caveat (ruling 78, point 6) is in that commit, or the harness calls no
     `reconcile.v1`; and the call timeout the engine is served with;
   - each Agent archive's version and digest, and the commit it was built from;
   - stage 2's documents as rendered: the lines in which they differ from the files in
     `stage-2/fleet/` other than by a filled placeholder, 0 if none, and the Agents' deadlines;
     and, if the store was first rendered without `reader-1`, when it and `turn-reader-1` were
     added;
   - that the store was initialized again with no Agent of stage 1, and when;
   - D58's guard: the commit or digest of the harness that holds it, and that the harness reads
     each order's text, report schema and refs from a git directory of its own, never through a
     slot's or the reader's clone's, runs each clone's guards with that git directory and the
     clone only as the working tree, runs nothing from a clone, and runs a guard that answered
     unknown again before the next turn; and when that held, before `reader-1` was added and
     signed in;
   - the outcome of the Claude scratch-home sign-in of experiment 027's amendment, by a link to
     its result, or the owner's answer to its failure, before `reader-1` is signed in;
   - each sign-in, made one at a time by the owner: the kind of sign-in login used (as
     `sign-in-login.v1` names it), never a URL, code or credential; its seconds from its
     `sign-in.v1` dispatch to its end in the records; whether the account check answered the
     account the Agent names, never the account's value; that the default homes of Codex and
     Claude Code kept their login files unchanged, by size, time and inode, never by content; and
     the everyday status of each client before and after, through a filter that prints only
     whether a login is held and whether the expected account is named, never a value. Opening a
     Keychain item stays off limits;
   - the version of Codex and of Claude Code each Agent runs, as `codex --version` and
     `claude --version` of the binary each Agent resolves give it, with the resolved path recorded
     as a placeholder;
   - the harness as run: the commit or digest of its scripts, and the lines its log and its guard
     files write for each turn, as orders 2 and 3's setup lists them and for each measure that
     decision 4 adds; the brief and the report schema that `reader-1` is given, each by digest. A
     measure that no line gives is not shown;
   - three restarts of the engine on stage 2's store after the sign-ins, with no turn sent, each
     timed as set-up seconds are, each followed by every account check answering the account its
     Agent names and no sign-in action pending, and their median, with condition 2's bound amended
     before stage 2's first turn by the rule that orders 2 and 3's setup gives, should any of the
     three exceed it; and one more such restart once `reader-1` is signed in, if it joins later,
     with the bound amended by the same rule, by a dated amendment merged before `reader-1`'s
     first turn.

4. **Stage 2's measures** replaces the line `MEASURES-SECTION`. It is a section headed
   `### Stage 2's measures`, whose first paragraph says that each measure of orders 2 and 3, per
   turn and per order, is taken for every order of stage 2 as their section "Measures" takes
   it, each number beside the command that gives it, and that stage 2 adds those below, each a
   bullet under "Per turn", "Per order" or "Per stage":
   - per turn: the slot clone or the reader's clone it ran in; the seconds it waited, once ready,
     for the turn in flight to end; the seconds the harness spent readying the clone before it
     (its lock, fetch, branch and clean), apart from the restart's; and for a turn of `reader-1`,
     its verdict, its findings by level, and the answer of its clone's guard;
   - per order: the pair it ran on; each read run outside Conduit in place of `reader-1`'s, with
     why; each sign-in action raised, by Agent, with its raising part (sign-in or setup), the
     restart it followed, and the check's answer (another account, no account, failed, unknown);
     each setup's account check, by Agent and answer (the named account, another, none, failed or
     unknown), as the records give it, or not shown; after each restart and before any turn, the
     sign-in actions pending, by Agent, as the harness's check of them gives it; each client's
     version at the order's first turn, read as the setup reads it, beside the setup's; and each
     stop and pause, with its rule, its evidence and when it was lifted;
   - per stage, in each result, over stage 2's orders so far: the restarts and their seconds, the
     waits for a turn in flight and the turns sent, shown as what a new conversation without a
     restart would remove and judged against nothing; and the seconds of each turn of `reader-1`
     and its tokens where the client reports them, compared with nothing.

5. **A78's row of the roadmap.** In `ROADMAP.md`, A78's row keeps one line. In its status cell,
   which begins `in progress: order 042`, after the clause on orders 2 and 3 that ends "the next
   waiting for the dogfood engine's re-pin to `main`;", add one clause in the row's style. It says
   that stage 2's documents (order 210) and its registration (order 211, dated DATE) are in
   experiment 028, with a link to
   `experiments/028-a78-dogfood/README.md#stage-2-registered-before-its-first-turn`; that stage 2 is
   the store initialized again with the Agent packages 0.2.0, or 0.2.1 if it is first initialized
   with them, after order 050n, with five Agents that the owner signs in once, two Codex pairs and
   a Claude reader on Claude Opus, each with a home of its own, sending one turn at a time until a
   dated amendment lets turns run at once; that it adds two pass conditions (no sign-in asked
   again, every read fresh) and five stop rules; and that it is the first dogfood milestone's run,
   so orders after order 3 go through it one turn at a time and the Claude reader joins once D58's
   guard is in the harness (ruling 89, points 3 and 5). In the row's next sentence, which begins
   with ruling 80's link and "point 7", add "until stage 2 starts" at the end of its clause
   "orders 2 and 3 run through Conduit one at a time while the plain runner carries the other
   orders", so that it no longer contradicts the clause you added. Change nothing else
   in the row or the file.

6. **Nothing else.** Change no other line of the README. Its additions are decision 1's line and
   decision 2's section, and it loses no line.

## Scope

Change only `experiments/028-a78-dogfood/README.md`, as decisions 1 to 4 say, and A78's row of
`ROADMAP.md`, as decision 5 says. Change nothing under `experiments/028-a78-dogfood/inputs/` or
`experiments/028-a78-dogfood/stage-2/`, and do not change `experiments/README.md`, `IN_FORCE.md`,
`docs/implementation/`, `tools/` or any order's file. Anything else you find that needs changing
goes in your report.

## Done when

From the repository root, each passes:

```sh
python3 tools/check_docs.py             # Errors: 0
python3 tools/check_private_paths.py    # Errors: 0
python3 tools/check_roadmap.py          # Errors: 0
python3 tools/check_rulings.py          # Errors: 0
git diff --check                        # prints nothing
git status --short                      # exactly: M ROADMAP.md and M experiments/028-a78-dogfood/README.md
```

Then each of these prints what is beside it:

```sh
R=experiments/028-a78-dogfood/README.md
S='/^## Stage 2: registered before its first turn$/,0'
grep -c '^## Stage 2: registered before its first turn$' $R          # 1
grep '^## ' $R | tail -n 1                                           # ## Stage 2: registered before its first turn
git diff -U0 $R | grep -c '^-[^-]'                                    # 0 (the README loses no line)
grep -c '(#stage-2-registered-before-its-first-turn)\.$' $R           # 1
awk "$S" $R | awk 'length > 100' | wc -l                              # 0
awk "$S" $R | grep -c "$(printf '\100')"                              # 0 (no at sign)
awk "$S" $R | grep -c -e 'SETUP-SECTION' -e 'MEASURES-SECTION' -e 'DATE'   # 0
awk "$S" $R | grep '^### ' | sed 's/^### //' | tr '\n' '|'
git diff -U0 ROADMAP.md | grep -c '^[-+]| A78 |'                      # 2
grep '^| A78 |' ROADMAP.md | grep -c '028-a78-dogfood/README.md#stage-2-registered-before-its-first-turn'   # 1
grep '^| A78 |' ROADMAP.md | grep -c 'DATE'                           # 0
grep '^| A78 |' ROADMAP.md | grep -c 'carries the other orders until stage 2 starts'   # 1
```

The line that lists the section's headings prints, on one line:

```text
Stage 2's orders|`reader-1`, D58, D59 and A77|Stage 2's claim|Stage 2's setup, recorded before its first turn|Stage 2's measures|Stage 2's pass conditions|Stage 2's stop rules|What stage 2 cannot show|What stage 2 leaves unregistered|
```

The coordinator records stage 2's setup by the amendment the registration names, once order 050n
has merged; this order registers what can be fixed before. The admin then has a fresh, read-only
Opus reader read the diff before it merges (ruling 79). Do not stage anything. Report each choice
you make as your own, the wording you chose for decisions 3 to 5, and every check's last line.
