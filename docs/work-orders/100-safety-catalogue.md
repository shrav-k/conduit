# Work order 100 (SC): the safety catalogue

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This is a paper order: it builds no Rust and writes no vector, so its
`vectors` list is empty. It has no network access. Never print a whole large file: use `head`,
`tail`, `grep`, `sed -n` or a count. The promise files are large: read them one file at a time,
in parts, and write each file's rows before you read the next (decision 10).

## What and why

An order of the proof lane under [ruling 71](../proposals/ruling-71/README.md). Two of ruling
47's conditions for v0 rest on a list that does not exist yet:

- "Every safety vector passes". [Ruling 68](../proposals/ruling-68/README.md), point 2, defines a
  safety vector as one "whose failure would be an unsafe finding as the tracker defines
  'unsafe'", carrying a field that names the safety promise it tests. It asks that each of S1's
  proposed safety traces have such a vector or be listed as missing.
- "Model checking shows the safety promises hold" (A40). G5 of
  [the readiness gate](../implementation/INDEPENDENT_BUILDS.md#readiness-gate) asks that every
  safety promise of the shared conventions hold under model checking or be listed with the
  reason. Ruling 39 found that "No list exists of the safety promises of the conventions".

This order writes that list for the whole of `spec/`, with S1's traces mapped to it. Later orders
take from it the promise each safety vector names (SV1 to SV3) and the promises each model
checks (MC2 on).

Read:

- the paragraph of [docs/AUDIT.md](../AUDIT.md) that defines unsafe (search for "mark as
  unsafe");
- round 5's unsafe rows in [its decisions](../proposals/reaudit/round-5/DECISIONS.md), M01 to
  M03, M32 and M45 to M50, as calibration for decision 2;
- [IN_FORCE.md](../../IN_FORCE.md), topic 13, and ruling 68, point 2;
- [spec/README.md](../../spec/README.md) and [spec/definitions.md](../../spec/definitions.md);
- every promise file: `spec/promises/*.yaml`, `spec/conventions/*.yaml` and
  `spec/interface/implementation.yaml`;
- [conformance/s1-safety-cases.md](../../conformance/s1-safety-cases.md).

## The decisions it applies

1. **Where it lives.** `docs/implementation/SAFETY_CATALOGUE.md`, which holds a placeholder
   now: replace it. It does not bind.
   It sits outside `spec/` and `conformance/` because classifying a promise adds no behavior,
   and the field of ruling 68 is written into the vectors by later orders. This is the lane
   coordinator's decision, logged in this order's Record. The catalogue links to no file under
   `docs/work-orders/`, since an order's file leaves the tree once it merges.
2. **The test.** A promise is a safety promise when an engine that kept every other promise but
   broke this one could, in some case, reach one of the outcomes the tracker calls unsafe. The
   engine includes what the promises bind beside it: a gate, a provider or an implementation.
   Another promise guarding the same outcome does not excuse it, so two promises can both be
   safety promises. The four kinds:
   - **access**: access is granted that should not be. Something is dispatched, delivered,
     listed or disclosed to a caller that may not reach it, including after the caller's access
     ended, through a diagnostic, or past a gate;
   - **secret**: a secret's value, or a credential or identity material, reaches a party not
     entitled to it;
   - **effect**: an effect is lost or repeated, or an unknown outcome is treated as known. This
     includes an effect, a fetch, a run or model work that nothing authorized;
   - **result**: a wrong outcome or output is reported for a request that was decided or
     dispatched, or a record states wrongly who did what.

   A promise that only decides whether a document is valid, or how a diagnostic is worded, is
   not a safety promise on that account. Refusing what is valid is never unsafe. Admitting what
   is invalid is unsafe only when what it admits reaches one of the four outcomes, and then the
   promise takes that outcome's kind; S1-archive-escape and S1-plain-filter are examples. Round
   5's unsafe rows show where the line fell before.
3. **The reason.** Each safety promise gets one sentence of reason. It quotes the words of the
   promise that the unsafe breach falls under and names their field: `when`, `then` or
   `observable`. A later order uses that to judge whether a vector tests the unsafe slice of
   the promise or another slice.
4. **Every promise once.** Every promise ID in the files above appears in the catalogue exactly
   once as the first cell of a table row. The cell is the bare ID with one space on each side,
   as `| ACC-001 |`, with no backticks and no link. It goes under one of three headings, in this
   order:
   - `## Safety promises`: one table per promise file, with its kind and reason;
   - `## For the coordinator`: one table, with the question that decision 2 leaves open and the
     two readings;
   - `## The others`: one table, with a short reason from this list: form; naming; diagnostic
     wording; progress or timing; provider choice; implementation-defined; or other, with the
     reason stated.

   The removed IDs that the files' comments list are not promises and are left out. No other
   table in the file has a promise ID as its first cell.
5. **The safety tables.** One table per promise file, in the order of the Contents table of
   `spec/README.md`, with the columns: ID; layer; level; kind; why a breach is unsafe; S1 traces.
   The last column names each trace of decision 6 whose requiring promises list the ID, or `—`.
6. **S1's traces.** A section lists every trace of `conformance/s1-safety-cases.md` in the
   file's order, by its vector name:
   - the 6 rows of the E05 table;
   - the 26 rows of the remaining table;
   - the two adjacent publication checks of the paragraph after the E05 table. `E05-handle` has
     the requiring promises IFC-011 and IFC-033, and `E05-narrowed` has IFC-033, TPL-011 and
     TPL-013.

   Its columns are: trace; the safety promises among its requiring promises, with their kinds;
   whether the trace itself is a safety trace, that is, whether its failure would be unsafe;
   and a flag. Flag every safety trace whose requiring promises include no safety promise, and
   every trace that you judge not to be a safety trace, with the reason. List the flagged
   traces again among your report's own choices. The three filtered replay variants of finding
   6-1-02, in the file's last paragraph, are noted as adjacent boundaries and are not listed as
   traces.
7. **G5's list.** Ruling 39 named "the safety promises of the seven conventions": the seven
   files under `spec/conventions/`. Promises whose layer is `convention` also sit in other
   files. The count line gives both numbers: safety promises in the seven files, and safety
   promises of the convention layer anywhere. It names the first as G5's list as ruling 39
   worded it, and the second as the wider reading, for the owner.
8. **Not yet written.** A short section lists the lines of IN_FORCE.md marked *spec pending*
   that, once written, would add or change a safety promise. Give each by its topic and its
   citation, with one sentence on why. No text outside `spec/` is classified.
9. **No change to `spec/`.** A defect found while reading goes in the report's findings with
   the promise IDs, and the catalogue does not work around it. That covers a gap, an
   ambiguity, a contradiction, or a procedure that conflicts with a safety promise, which the
   tracker counts as unsafe. The next order, MC2, will model OPS-008 to OPS-013, REV-002,
   REV-011 and REV-012. Report each of these nine that you do not class as a safety promise,
   with the reason.
10. **How to work.** Classify one promise file at a time: read it in parts, write its rows in
    the catalogue, then go on to the next. Reread decision 2 before each file, so that the last
    file is judged as the first was.
11. **The header.** The file opens with these:
    - "Not binding."
    - That the classification is the coordinator's, open to the owner to strike (IN_FORCE.md,
      topic 15).
    - The short hash of the commit it was written from (`git rev-parse --short HEAD`).
    - That the proof lane updates it when a promise changes.
    - A count line: promises read; safety; for the coordinator; others; safety promises by
      kind; and decision 7's two numbers.
    - The command that recounts the rows under each heading, from "Done when".
12. **The index.** The catalogue's row in [docs/README.md](../README.md) is already in place;
    do not change it.

## Scope

Change only `docs/implementation/SAFETY_CATALOGUE.md`.

Do not change any other file: not `spec/`, `conformance/`, `engine/`, `packages/`,
`experiments/`, `docs/README.md`, IN_FORCE.md, ROADMAP.md, docs/AUDIT.md, or any record or
ruling. Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
grep -n -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" docs/implementation/SAFETY_CATALOGUE.md
grep -n -E "[[:space:]]+$" docs/implementation/SAFETY_CATALOGUE.md
diff <(grep -hoE '^- id: [A-Z]+-[0-9]+' spec/promises/*.yaml spec/conventions/*.yaml spec/interface/implementation.yaml | sed 's/^- id: //' | sort) \
     <(grep -oE '^\| [A-Z]+-[0-9]+ \|' docs/implementation/SAFETY_CATALOGUE.md | grep -oE '[A-Z]+-[0-9]+' | sort)
```

The last one shows that every promise ID is the first cell of exactly one row. This command
recounts the rows under each heading, and the header cites it:

```bash
awk '/^## /{s=$0} /^\| [A-Z]+-[0-9]+ \|/{n[s]++} END{for(k in n) print n[k], k}' docs/implementation/SAFETY_CATALOGUE.md
```

Report, as numbers:

- the promises read, by file;
- the safety promises, by kind and by layer, and decision 7's two numbers;
- the rows for the coordinator;
- the traces listed, and those flagged.

Do not stage anything.

## Record

Run on 2026-11-11 beside orders 101 to 105: the worker in 25 minutes (1,526 s), then a fresh
verifier in 8 minutes (454 s). That verifier found five promises classed as not safety that its
test makes safety promises: INS-005, CON-014, POOL-009, OBJ-003 and INS-014. A second round of 3
minutes (187 s) moved them. A fresh verifier in 4 minutes (262 s) then found three more, on the
same grounds: SCT-005, POOL-016 and PLC-002. The lane coordinator moved those three after the
second round:

- SCT-005 as access: a translated body that differed would grant access the author did not
  write;
- POOL-016 and PLC-002 as result: a Resource placed on another machine serves a request from the
  wrong machine.

Neither verifier could rerun the tools' unit tests in its read-only sandbox. The first could not
run the bash comparison of IDs there either, so it compared them in memory instead.

The catalogue classes 502 promises:

- 394 safety promises: access 193, secret 39, effect 119, result 43;
- 1 for the coordinator: IFC-007, whether the metadata sent to a plain server discloses too much;
- 107 others.

By layer, the safety promises are 133 of the language, 217 of the conventions and 44 of the
interface. G5's list, the safety promises in the seven convention files, holds 128. The wider
reading, the convention layer anywhere, holds 217; that choice is for the owner.

Of S1's 34 traces, 27 are safety traces. Each of them has at least one safety promise among its
requiring promises. The other 7 are flagged as publication only: the six E05 schema traces and
E05-handle. Which traces already have a vector is not this order's question; SV2's reviewed table
answers it, since ruling 68 says the manifest's group tags are not safety tags.

The worker read the promise files one at a time as decision 10 asked, except for one slip at
revisions.yaml, which it corrected before going on. Its findings are for the admin's lane:

- SC-F1: `conformance/s1-safety-cases.md` is stale. Its bootstrap trace expects two root
  Conduits, not three. Its plain-ends trace expects failed for -32602 where IFC-010 now gives
  unknown for an effectful operation. Its closing paragraphs still call settled questions open.
- SC-F2: `spec/definitions.md`'s row for plain still gives failed for invalid parameters without
  limiting it to read-only operations, which could license an unsafe retry.
- SC-F3: OPS-009, OPS-034 and IFC-019 are ambiguous on a filtered repeat of a plain operation
  that succeeded with content but declared no outputs.
- SC-F4: other summaries in `spec/definitions.md` are stale: the diagnostic, the target, the
  manifest's count of document kinds, the core package's root Conduits, and the sign-in
  elicitation.

The coordinator checked every total and the comparison of IDs before the merge, and ran
check_docs, check_rulings, set_names, check_schemas and check_roadmap, and the private-path grep,
all clean.

2026-11-11, later: the admin decided the one question for the coordinator. IFC-007 is wire
shape, not an access-safety promise: the selected implementation is granted the Resource's
configuration and the call, IFC-030 withholds the consumer's proof, and the secrets promises
withhold secret material. The decision is open to the owner to strike. The lane coordinator
moved IFC-007 to the others, in work order 105's pull request. The counts are now 394 safety, 0
for the coordinator and 108 others.
