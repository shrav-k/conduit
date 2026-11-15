# Work order 161: A78's stage 1: its evidence and the engine's records of order 042 enter the tree

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order needs no network access: use none, and
reach no other machine. It runs no cargo command. Do not run `conduit`, the dogfood harness or
anything else against a store: no store is in your tree, and none is needed. Never print a whole
large file. Read `harness.md`, `plan.md` and the Fleet's documents in parts, with `sed -n`. Read the
export and `measures.jsonl` only with `jq`, `grep -c` and `wc`, print only the fields you need, and
never print a whole record. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 10 of [its orders](../proposals/full-review/orders.md)) found
that the evidence and the means of A78 are outside the repository
([FR-342](../proposals/full-review/findings/P06-dogfooding.md#fr-342)). Ruling 70 ranks A78 first.

- A78 has no directory under `experiments/` and no dated result, and its register row links no
  file.
- The harness and the `conduit-core` Fleet's documents are outside the tree.
- The coordinator's A78 plan, which ruling 74's record cites, has no file.
- The engine's own records of order 042's turns are in the dogfood store, and order 050 will
  initialize that store again. That new store, not the re-pin, destroys the records
  ([FR-528](../proposals/full-review/findings/G6-outside-the-tree.md#fr-528), as its verifier
  corrected it).

The review asks for one directory for A78, holding the method as run, a dated results file, the
Fleet's documents with placeholder paths and the records export
([FR-526](../proposals/full-review/findings/G6-outside-the-tree.md#fr-526), part 2). The results
report the measures of [FR-531](../proposals/full-review/findings/G6-outside-the-tree.md#fr-531)
for order 042 as an observation, not as a pass or a fail: no measure was fixed before 042 ran
(ruling 45).

Under the coordinator's D56 ([decisions](../proposals/full-review/decisions.md)), the export was
made with the pinned engine and with `serve` stopped, before order 050 makes a new store. It stayed
outside the tree until the private-path checker and the secret scan were in, and order 151 brought
them in. This order:

- brings the export and the other inputs into the tree;
- writes the experiment's README and its dated results;
- links A78's row to them;
- moves the handover's list "What worked (log)" into the results.

"What worked" and "What failed or cost" are the two A78 marks of
[FR-532](../proposals/full-review/findings/G6-outside-the-tree.md#fr-532). Order 155 closed "What
failed or cost". It logged the frictions as findings 4-21 to 4-27 of increment 4's log, and those
rows and 4-28 are the ones FR-526 part 2 asks for.

This order does not:

- pre-register A78's claim, pass conditions or measures for the orders from 046 on (that is the
  review's order 12);
- log a finding;
- write the method of the runners (the review's order 27, under D62);
- touch the dogfood engine.

## The decisions it applies

1. **The directory is `experiments/028-a78-dogfood/`, and its inputs are the coordinator's.**
   - 028 is the number that the review's [changes to the experiments](../proposals/full-review/experiments.md)
     assign to A78, under "Numbers, assigned once". Order 159 used 027 for A5 under the same list.
   - Before dispatch, the coordinator placed seven untracked files in
     `experiments/028-a78-dogfood/inputs/`. They stay where they are, byte for byte, and are
     committed with the results. The export is a raw run record: it leaves the tree at the next
     condensation, and the results link its last version
     ([keeping evidence](../../experiments/README.md#keeping-evidence)).
   - The harness's code stays outside the tree. The tree holds the method as run, with every
     private value a placeholder (D62).

   The seven inputs:

   - `inputs/records-2026-11-12.jsonl` is the export, dated by the store's last write.
     - The coordinator made it on 2026-11-15 with the dogfood's pinned engine, `38c0b4f`, by
       running `conduit records --store` on the dogfood store with `serve` stopped. By the
       coordinator's account, the command wrote nothing to standard error.
     - It holds 344 CloudEvents lines: 104 `object-change`, 60 `access-decision`,
       60 `invocation-dispatched`, 60 `invocation-ended` and 60 `request-ended`.
     - By the coordinator's account, it is byte-identical to the harness's final export, made after
       order 042's last run, and holds no absolute path, user name or credential.
     - Its records hold IDs, operation names, times, attempts, outcomes and access decisions. None
       holds a request's inputs.
   - `inputs/measures.jsonl` holds the harness's measures, four lines, one for each run of the
     harness on order 042. A run sends one or more turns. In each line:
     - `turns` counts the turns the run sent;
     - `rounds` is the last round the run reached;
     - `start` says where the run began;
     - `exit` says how it ended;
     - `invocation` is the harness's own name for the run, the time it began with no time zone. It
       is not a Conduit invocation ID.
   - `inputs/fleet/conduit.yaml`, `inputs/fleet/instances.yaml` and `inputs/fleet/templates.yaml`
     are the `conduit-core` Fleet's documents as templates. They are not the copies the harness
     rendered. `@SLOT_1@` in `instances.yaml` is the templates' own placeholder, which the
     harness's setup filled with the slot's path.
   - `inputs/harness.md` is the harness's README. The coordinator replaced its private paths with
     `<dogfood>`, `<main checkout>` and `<slot-1>`.
   - `inputs/plan.md` is the coordinator's A78 plan that ruling 74's record cites, scrubbed with
     `<dogfood>` and `<github url>`. Other text in angle brackets in it, such as `<schema>`,
     `<cause>` and `<turnDeadlineSeconds>`, is the plan's own notation.

   Before anything else, run `shasum -a 256` over the seven files and keep the output for your
   report. Do not change, move or delete any of them. Stop and report the file, the line and the
   check, without changing the file, if any of these happens:

   - an input is missing;
   - an input fails a check of "Done when";
   - an input disagrees with what this decision says.

   The coordinator corrects the inputs.
2. **`README.md` gives the method as run.** Write it from the inputs, in about 100 lines at most.
   - **Opening.** The title is `# 028 — A78: Conduit runs its own orders`, and the line after it
     is "Not binding." Then the README says three things:
     - this is A78's directory, with a link to the roadmap's register;
     - stage 1 is orders 1 to 3, which root sends through turns on Agents of the `conduit-core`
       Fleet (ruling 70; ruling 61, B12);
     - A78's claim, pass conditions and measures for later orders are not registered here yet.
   - **The setup as run.**
     - The engine was pinned at `38c0b4f`. That commit holds order 074 and neither 070 nor 077.
       Give the commands that show it: `git merge-base --is-ancestor 6591a064 38c0b4f` (074)
       succeeds, and the same command fails for `1a34ccf9` (070) and for `575a1bcd` (077).
     - The `.v2` Agent packages were installed from the branch `wo/076-agents-v2-folder-grant`.
       They stay unmerged until order 050. Link order 076's file in `docs/work-orders/`, whose
       Record says where that branch stands. Give the branch's commit only if an input names it.
     - Give the Agents `worker-1` and `verifier-1`, their deadlines and their slots, as the Fleet's
       documents and `harness.md` give them.
     - List every placeholder that decision 1 names. For each, give what it stands for as the input
       says (or "not stated"), its class (path, name or other) and the files that use it.
   - **The harness as run.** Give one line for each step of a turn, in `harness.md`'s order:
     - how a turn is sent;
     - the restart for each fresh conversation;
     - the check of each report against its schema, and the one repair turn;
     - the guards after each turn;
     - the records exports;
     - the measures file.

     Link `inputs/harness.md` for the exact commands. Copy none of them, and add no step, command
     or value that `harness.md` does not have.
   - **The plan.** `inputs/plan.md` is the plan that ruling 74's record cites as "The coordinator's
     plan for A78 under ruling 70". Link that record. Say what the plan plans in two to four
     sentences, in its own words where possible.
   - **The export.** Say how and when it was made, as decision 1 says, and give its counts with the
     command beside each.
   - **Files.** Give a table of every file in the directory, one line for each. Say that each input
     keeps the words it was written with before it was committed.
   - **What it cannot show.** This follows the corrections of FR-531's verifier and D57:
     - The records say which requests were made, by whom, and how each ended. They say no more.
     - What an Agent did in its slot with its own tools is not a Conduit request, so it is not
       in the records (CTR-014, PRV-006, ACC-027).
     - In stage 1 every caller is root, so the records do not test attribution among callers.
     - An order run through Conduit is not evidence about access unless the results count a
       refusal, an open or a close.
     - The reports of 042's worker and verifier are not in this directory.
3. **`results-2026-11-12.md` is an observation of order 042.** Keep it to about 160 lines at most.
   Its name carries the last day of the run, as FR-526 part 2 names it.
   - **Opening.** The title is `# 028 — A78: order 042, observed`. The line after it reads: "Not
     binding. An observation, not a pass or a fail: no measure was fixed before order 042 ran
     (ruling 45)." The file then says three things:
     - 042 ran on 2026-11-11 and 12;
     - the export was made on 2026-11-15;
     - work order 161 wrote the file on DATE. Put the date you write it in place of DATE.

     Give times as the records give them, in UTC. The Record's and the handover's dates are local.
   - **Sources (D13).** Every number has beside it the command that gives it from this directory's
     files, run from the repository root: `wc -l`, `grep -c` or `jq`.
     - For a number in a table, put the command in a fenced block right after the table.
     - Attribute a fact taken from the handover or from 042's Record to that source, and never
       present it as measured.
     - Where the export or `measures.jsonl` gives the same fact, give both.
   - **Outcomes.** Use the outcomes the records give. Do not write that an effect did not occur
     ([ruling 78](../proposals/ruling-78/README.md), point 6), and give no verdict against REC-001.
   - **What worked.** Give every fact of the two handover bullets that decision 6 removes,
     attributed to "the handover of 2026-11-12" (`docs/implementation/HANDOVER.md` at `632881df`,
     lines 140-147):
     - the smoke test and its six checks;
     - 042's four rounds and its valid reports;
     - the one guard that tripped;
     - its first worker turn of 11966 s;
     - its merge on 2026-11-12 (pull request 264).

     Then add one line saying that what failed or cost is logged in increment 4's log as findings
     4-21 to 4-27.
   - **The records.** Give the number of lines of each type, and the times of the first and the
     last record.
   - **The turns.** Give one table row for each `invocation-dispatched` record whose `operation`
     ends in `#turn.v1`. Each row gives:
     - the invocation's full ID and the attempt;
     - the target and the caller;
     - the dispatch time;
     - the time of the turn's `invocation-ended` record, or "no end recorded";
     - the seconds between the two times;
     - the outcome, and the reason if a record gives one;
     - what the turn belongs to: a round and a role of order 042, the smoke test, or
       "unattributed".

     Attribute a turn only on evidence in this directory, and name the evidence. The evidence is:
     - the lines of `measures.jsonl`, with each run's `turns`, `rounds` and `start`;
     - `harness.md`'s order of the turns in a round;
     - 042's Record.

     Match the runs to the turns by their order and by the gaps between the runs' start times,
     which do not depend on a time zone. Mark a turn matched this way "inferred", and give the
     reason. A turn's `access-decision` record names, in `tool`, the tool root called, which is
     named for the Conduit the turn came through. `fleet/instances.yaml` says which Agent each turn
     Conduit reaches. Name an Agent only from such a pair, and say which pair. The Record gives at
     least nine turn invocations for 042: four worker turns, four verifier turns, and a failed
     turn of round 3 that was run again (FR-528, as corrected).
   - **FR-531's measures for order 042.**
     - (a) Compare three counts: the turns the harness counts (each run's `turns` in
       `measures.jsonl`, and their sum), the turns the Record gives, and the turn invocations in the
       export. For each run and each turn of the Record, say whether the export holds it, and how you
       matched it.
     - (b) For each matched turn, say whether the export gives its caller, Agent, start, end and
       outcome. Name each field that the records do not give.
     - (c) For each turn whose outcome is not `succeeded`, give its outcome, and its reason if a
       record gives one. Then say what followed it on the same target: a later attempt of the same
       invocation, a new request under another invocation ID, or a later `reconcile.v1`
       invocation. For a `reconcile.v1` invocation, say whether its records name the turn it
       settled. Also say whether anything in the records links the later request to the earlier
       one. This is reported, not judged (FR-531's verifier).
     - (d) Redactions made before committing the export, by class (path, name, other): none,
       since the coordinator states that it is byte-identical to the harness's final export. The
       README lists the other inputs' placeholders.
     - (e) What Conduit refused, opened and closed (D57). Count the `access-decision` records whose
       `permitted` is false, and the invocations of the core's `open.v1` and `close.v1` that
       succeeded. Give each count over 042's interval, from 042's first turn to the last record,
       and over the whole export. A zero says that 042 tested no access.
   - **The harness's measures.** Give a table of the four lines of `measures.jsonl`, with the keys
     as the file names them.

   If a turn is in the Record or in `measures.jsonl` but not in the export, write it as "not
   found in the export". Report it as a candidate finding against REC-001 for the coordinator to
   log. Do not log it.
4. **A78's register row links the results.** In `ROADMAP.md`, replace the State cell of A78's row
   (line 71) with this text. Nothing else in the row changes.

```markdown
in progress: order 042, root's first order through Conduit (2026-11-11), its `.v2` Agents installed from order 076's branch, merged on 2026-11-12 (pull request 264), observed from the engine's own records of its turns in [experiment 028](experiments/028-a78-dogfood/README.md) ([results of 2026-11-12](experiments/028-a78-dogfood/results-2026-11-12.md)); the next waits for the dogfood engine's re-pin to `main`; orders 4 and 5 await appointment (batch 2)
```

5. **The experiments index.** In `experiments/README.md`, add this row after 027's (line 43). Put
   the date you write it in place of DATE.

```markdown
| [028 — A78: Conduit runs its own orders](028-a78-dogfood/README.md) | A78: work orders run through Conduit's own Agents, each friction logged as a finding of the first build. Stage 1: orders 1 to 3, sent by root. | Order 042 observed on DATE by work order 161, from the engine's records of its turns; [results](028-a78-dogfood/results-2026-11-12.md). |
```

6. **The handover.** In `docs/implementation/HANDOVER.md`, replace lines 138 to 147 with the text
   below. Those lines are "What worked (log):", the blank line after it and its two bullets. Keep
   the one blank line before the paragraph that begins "What failed or cost".

```markdown
What worked, the smoke test's six checks and order 042's four rounds, is recorded in
[A78's results for order 042](../../experiments/028-a78-dogfood/results-2026-11-12.md), beside the
engine's own records of 042's turns and [the harness's method as run](../../experiments/028-a78-dogfood/README.md).
```

## Scope

Create `experiments/028-a78-dogfood/README.md` and `experiments/028-a78-dogfood/results-2026-11-12.md`.
Change only these:

- A78's State cell in `ROADMAP.md`;
- one row of `experiments/README.md`;
- in `docs/implementation/HANDOVER.md`, only the passage that decision 6 names.

Change nothing under `experiments/028-a78-dogfood/inputs/`, and nothing else. That includes
increment 4's log, ruling 74's record and `.gitleaks.toml`.

## Done when

From the repository root, each passes:

```sh
D=experiments/028-a78-dogfood
R=$D/inputs/records-2026-11-12.jsonl
shasum -a 256 $D/inputs/*.* $D/inputs/fleet/*      # the same digests as before you began
wc -l < $R                                         # 344
jq -r .type $R | sort | uniq -c                    # 60, 60, 60, 104, 60, in that order
jq -r 'select(.type=="invocation-dispatched" and (.data.operation|endswith("#turn.v1"))) | .data.invocation' $R | while read -r i; do grep -q "$i" $D/results-2026-11-12.md || echo "$i"; done   # prints nothing
grep -c '^Not binding\. An observation, not a pass or a fail' $D/results-2026-11-12.md   # 1
grep -c '^Not binding\.' $D/README.md              # 1
wc -l $D/README.md $D/results-2026-11-12.md        # about 100 and about 160 at most
grep '^| A78 |' ROADMAP.md | grep -c '028-a78-dogfood/results-2026-11-12.md'      # 1; on main, 0
grep -c '028-a78-dogfood/README.md' experiments/README.md                          # 1
grep -c 'What worked (log)' docs/implementation/HANDOVER.md                        # 0
grep -c '028-a78-dogfood/results-2026-11-12.md' docs/implementation/HANDOVER.md    # 1
for n in 11966 1273 264; do grep -q $n $D/results-2026-11-12.md || echo $n; done   # prints nothing
grep -n 'did not occur' $D/README.md $D/results-2026-11-12.md                      # prints nothing
grep -rnE '/(Users|home)/[A-Za-z0-9._-]+|[A-Za-z]:\\+Users\\+|\.ts\.net|SHA256:[A-Za-z0-9+/]{43}|100\.(6[4-9]|[7-9][0-9]|1[01][0-9]|12[0-7])\.[0-9]{1,3}\.[0-9]{1,3}' $D | cut -d: -f1,2   # prints nothing
grep -rcE 'sk-[A-Za-z0-9_-]{20,}|gh[pousr]_[A-Za-z0-9]{20,}|github_pat_|xox[abpr]-|AKIA[0-9A-Z]{16}|lin_api_|BEGIN [A-Z ]*PRIVATE KEY' $D | grep -v ':0$'   # prints nothing
python3 tools/check_roadmap.py
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
git status --short                                 # the three changed files and experiments/028-a78-dogfood/
```

With the inputs in place, `check_roadmap.py` fails until decisions 4 and 5 are done, and the
A78-row grep prints 0 on `main`.

Do not stage anything. In your report, give:

- each choice you made yourself;
- the digests from before you began and from the end;
- every turn you could not attribute or match;
- any candidate finding;
- every check's last line.

`git diff --check` and `check_private_paths.py` read tracked files only. So the admin then stages
the directory, inputs included, and runs `python3 tools/check_private_paths.py` and
`git diff --cached --check`. The admin also shows the `docs` and `secrets` jobs passing on the pull
request. A fresh, read-only Opus reader reads the result before it merges, beside the Codex
verifier (ruling 79, point 2).
