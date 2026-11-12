# Work order 116 (MC8): G5's account of the safety promises

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This is a paper order: it builds no Rust and writes no vector, so its
`vectors` list is empty. It has no network access. Never print a whole large file: use `head`,
`tail`, `grep`, `sed -n` or a count.

**Precondition.** Experiment 012's README must hold MC7b's accounting: a section for work order
113, and one for 115. If it does not, stop, change nothing, and say so in your report.

## What and why

An order of the proof lane under [ruling 71](../proposals/ruling-71/README.md). G5 of
[the readiness gate](../implementation/INDEPENDENT_BUILDS.md#readiness-gate) asks that every
safety promise of the shared conventions hold under model checking "in the stated
configurations", or be listed with the reason it cannot. Ruling 75, E1, counts all 217
convention-layer safety promises of [the safety catalogue](../implementation/SAFETY_CATALOGUE.md),
wherever they sit. Ruling 47 makes "the safety promises hold under loss, reordering and
partition" a condition of v0.

Work orders 105 and 107 to 115 built the models in
[experiment 012](../../experiments/012-model-checking/README.md). This order gathers what they
found into one account of the 217, and states plainly where each promise stands.

Read:

- the safety catalogue, its convention-layer rows above all;
- experiment 012's README: every model's section, its accounting, its properties, its results
  and its findings;
- the Records of work orders 105 and 107 to 115, in `docs/work-orders/`;
- ruling 75, A1 and E1, and IN_FORCE.md topic 13.

## The decisions it applies

1. **Where it lives.** A new file, `experiments/012-model-checking/G5.md`, with one line in the
   experiment's README that links it, under the README's opening. It does not bind.
2. **The table.** Under a heading `## The account`, the table has exactly one row for each of the
   217 convention-layer safety promises in the catalogue, in the catalogue's order. Its first
   cell is the bare ID with one space on each side, as in the catalogue. No other table in the
   file starts a row with a bare promise ID. The columns are:
   - **ID**;
   - **in the seven files**: yes for a promise in `spec/conventions/*.yaml`;
   - **model**: the module and property that check it, or `—`;
   - **standing**: one value from decision 3;
   - **note**: any partial exclusion, quoted from the model's own accounting row, and anything
     else a reader needs.
3. **The standings.** Take each from the model's own accounting row, and judge it on the model as
   the promises are written, with the readings the order applied. The values:
   - `holds`: the model checks it, and the correct model holds it;
   - `holds under ruling 75 A1; refuted as written: <finding>`: the promises as written break it,
     and the fenced reading that ruling 75 A1 put in force holds it. Example: OPS-018 under
     MC2-001;
   - `contradiction: <finding>`, `gap: <finding>` or `ambiguity: <finding>`: a finding against
     the promises, of that class, touches it;
   - `property over-broad: <finding>`: the order's property asked more than the promise, the
     promise itself was not refuted, and the note names the control that holds. Examples:
     MC3-001 for CON-013, and MC3-002 and MC3-004 for ACC-034, where control Q holds;
   - `static`: a static decision, to be shown by vectors;
   - `not modelled`: with the model's reason, such as a slice that the README places elsewhere;
   - `left to MC9`: PND-001 to PND-012, ten promises, which wait for batch 2.

   Where a promise has more than one finding, give each in the note. Where your reading of a
   model differs from the expected set below, follow the README and report the difference.
4. **The expected set of findings** against the promises, with their standing as of this order.
   Orders 077, 081 and 082 are written but not yet run, so write each as "to be written" or "to
   be taken", "not yet run". Their Records will show when they are applied.
   - **MC2-001**, and its reproductions **MC7a-002** and **MC6's unfenced control**: ruled by
     ruling 75, A1, and to be written by order 077.
   - **MC4-001** and **MC4-009**: to be fixed by order 081.
   - **MC4-002**: to be stated by order 081 as the limit that ruling 42, D11, leaves. Whether to
     fence it is a question on the owner's list.
   - **MC6-001** and **MC6-002**: to be taken into the specification by order 081.
   - **MC7b-001** and **MC7b-002**: for order 082, now being drafted.

   This list governs. Findings that concern only an order's wording or a model's own
   expectations are listed apart, as not findings against the promises.
5. **MC2's promises.** MC2's README has no accounting list. Take its promises and properties from
   order 105 and the README:
   - at most once: OPS-018, OPS-008, OPS-012 and OPS-024;
   - no wrong outcome: OPS-012 and OPS-014;
   - unknown stays unknown: OPS-013 and IMP-007;
   - a repeat is answered and not dispatched: OPS-009 and OPS-032;
   - different contents conflict: OPS-010;
   - revision checked: REV-002 and REV-011;
   - one change at a time: REV-012.

   OPS-011 is shown only by a reachability witness, which keeps keyless requests apart; give it
   `holds`, and say so in the note. OPS-030 is left out by the README, which excludes
   unauthorized reconciliation; give it `not modelled`, with that reason. MC7a's REV-010
   supersedes MC2's.
6. **Where two models touch one promise**, the later order's entry stands, and the note names the
   earlier. Example: MC6 models the eight promises that MC3 left to the lifecycle model.
7. **Sections beside the table.**
   - **Counts**, first:
     - the 217, by standing;
     - the 125 of them in the seven files, by standing;
     - the refuted or contested promises, with their findings.
   - **Outside the 217.** Ruling 39's seven files also hold three language-layer safety
     promises: LIF-002, LIF-007 and IMP-003. They are not in the 217, which ruling 75, E1,
     counts by layer. Name them, and say that G5's account as ruled does not include them.
   - **Configurations.** One row per model, giving the faults it ran under (loss, reordering,
     duplication, partition, crash, a lost machine, timers) and its bounds, from its README.
   - **Findings**, as decision 4 gives them, with the configuration and the length of each first
     trace.
   - **What G5 still needs.** It says plainly what remains:
     - MC9 for pending actions;
     - the orders not yet run;
     - any promise left unchecked.

     It decides nothing.
8. **No change to `spec/`, to any model, or to the catalogue.** Where a model's list and the
   catalogue disagree, such as a promise in no model's list, report it as a finding of class gap,
   and give it `not modelled`, with the reason "in no model's list".

## Scope

Change only `experiments/012-model-checking/G5.md` (new) and one line of
`experiments/012-model-checking/README.md`.

Do not change any other file: not the models, `spec/`, `conformance/`, `engine/`, `packages/`,
`docs/`, the catalogue, `experiments/README.md`, ROADMAP.md, IN_FORCE.md, or any record or ruling.
Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Then run these three commands with `bash`; each prints nothing when it passes. The first finds
private paths, the second trailing spaces, and the third checks that the table under
`## The account` has exactly one row for each of the catalogue's 217 convention-layer safety
promises:

```bash
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" experiments/012-model-checking/G5.md
grep -n -E "[[:space:]]+$" experiments/012-model-checking/G5.md
diff <(awk '/^## Safety promises/{s=1} /^## For the coordinator/{s=0} s && /^\| [A-Z]+-[0-9]+ \| convention \|/' docs/implementation/SAFETY_CATALOGUE.md | grep -oE '^\| [A-Z]+-[0-9]+' | grep -oE '[A-Z]+-[0-9]+') \
     <(awk '/^## The account/{s=1; next} /^## /{s=0} s && /^\| [A-Z]+-[0-9]+ \|/' experiments/012-model-checking/G5.md | grep -oE '^\| [A-Z]+-[0-9]+' | grep -oE '[A-Z]+-[0-9]+')
```

The third compares in order, so the table must follow the catalogue's order. Report the counts of
decision 7. Do not stage anything.
