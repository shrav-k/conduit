# Work order 182: stale state lines in the handover, tracker, roadmap and engine pages

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none,
and it runs no cargo command. Never print a whole large file or log: read `docs/AUDIT.md`,
`engine/CONFORMANCE.md`, `conformance/README.md` and the other long pages by line ranges or with
`grep`. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 36 of [its orders](../proposals/full-review/orders.md)) found
lines in the coordinator's pages that no longer say what the tree holds: the handover, the
tracker, the roadmap, the engine's conformance page and README, and a few pages beside them. The
order carries [FR-276](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-276),
[FR-377](../proposals/full-review/findings/P08-progress.md#fr-377),
[FR-537](../proposals/full-review/findings/G6-outside-the-tree.md#fr-537) and
[FR-354](../proposals/full-review/findings/P07-engine-build.md#fr-354); the verifiers' corrections
of [FR-344](../proposals/full-review/findings/P07-engine-build.md#fr-344) (049a1 is only the first
half of order 049a) and [FR-347](../proposals/full-review/findings/P07-engine-build.md#fr-347)
(registration copies the whole state at six sites, not four);
[FR-317](../proposals/full-review/findings/P04-providers-built.md#fr-317), items 1 and 2; the
rewording that [FR-303](../proposals/full-review/findings/P04-providers-built.md#fr-303)'s
verifier asks for, with D45 of [the review's decisions](../proposals/full-review/decisions.md);
and [FR-330](../proposals/full-review/findings/P05-second-machine.md#fr-330), point 5. Each line
misleads a coordinator taking over, or a cold reader, and
[ruling 52](../proposals/ruling-52/README.md) asks every statement to be as explicit as it can be.
Every edit is the coordinator's, and none changes behavior. No file under `spec/` changes; in
`IN_FORCE.md`, one line gains a promise's citation, and no decision changes.

Orders 161, 168, 170 and 173 have edited these pages since the review. This order carries only
the lines still stale on `main` at `ab07fd47`. It leaves these, each for the reason given:

- Ruling 75 D4's reading, still listed as waiting in the tracker (line 176), the handover (lines
  192 to 194) and the scenario plans: order 186, the review's order 35, strikes it with the plans'
  other lines ([FR-388](../proposals/full-review/findings/E01-s2-s3-s4.md#fr-388)).
- G5.md's "Orders not yet run" (FR-377, item 5): order 168 marked that part of G5.md as the
  account of 2026-11-11.
- The promise files' `vectors: []` (FR-377, item 7): the review's order 55 keeps it, under F127.
- The handover's five coordinator fixes (FR-276): the method as run now gives them as examples,
  which is what its verifier said they were.
- Whether sign-in is on the rerun path (FR-377's verifier): the tracker and the handover agree
  since ruling 78, point 1.
- The rest of FR-537, which assesses and finds nothing stale; FR-354's 8 MB, which its verifier
  calls a dated measurement; and `engine/CONFORMANCE.md`'s 180-second turn deadline, which is true
  of the 0.1.1 Agent packages on `main`.
- FR-354's remark that enrollment, the short form and metrics do not appear in
  `engine/CONFORMANCE.md`: order 185, the review's order 43, generates each promise's standing,
  built or not.
- The status lines of rulings 40 and 41, "PRV-001 to PRV-026": order 034 wrote them after PRV-027
  and PRV-028 existed, so they are not plainly stale.

Three written orders touch the same files, on other lines. Order 180 changes the link anchors on
lines 24 and 31 of `experiments/README.md`, order 177 row 4-36 of increment 4's log, and order
181 line 290 of `IN_FORCE.md`. Orders written beside this one do too: order 183 adds a section to
`engine/CONFORMANCE.md`, order 184 changes S3's plan, and order 186 strikes the D4 lines of the
tracker, the handover and the plans. Build on each file as it is on `main` when you start, and
change only what the decisions below name.

## The decisions it applies

1. **How each edit is made.**
   - Quoted old text is as `main` holds it at `ab07fd47`, at the lines named. If an order merged
     since has moved it, find the same text. If the text itself has changed, apply the nearest
     sound reading and say so in your report.
   - A blockquote gives the new text exactly. Where a decision says that words in it are a link,
     make them a Markdown link to the target named, written relative to the file that holds it.
     Every target named below is a link from this order's own directory.
   - Keep every Markdown line that you write or rewrap at most 100 characters. Rewrap only the
     sentence, bullet or paragraph that a decision changes, at its own indent. A table row stays
     on one line, `IN_FORCE.md` keeps its one line per item, and so does the list of
     `docs/AUDIT.md` that holds line 235, whose bullets are one line each on `main`.
   - Nothing else in any file changes. Add no "(log)" mark: the review's order 37 closes them.
2. **The handover**, `docs/implementation/HANDOVER.md` (FR-276, FR-317 item 1, FR-303, FR-344,
   FR-330 point 5). Order 071 merged on 2026-11-15 (pull request 281).
   - Lines 76 and 77, in the bullet "Its queue": "Tell it when 071 merges (SV1 unblocked) and when
     the checkpoint ends." becomes "Tell it that 071 has merged (#281), so SV1 is unblocked, and
     when the checkpoint ends."
   - Line 128: "can run now: 071 and 080 are checkpointed and the computer is idle;" becomes "can
     run now: 080 is checkpointed and the computer is idle;". The bullet's "(log)" stays.
   - Row 5 of the queue (line 161). Its Item cell does not change. Its State cell becomes:

     > Merged on 2026-11-15 (#281, `2318ec59`) after a third round on the Mac; at the handover it
     > was checkpointed after two rounds on Windows, with one must left (its Record)

     "its Record" is a link to [071's Record](071-conformance-manifest-pieces.md#record). Its Next
     step cell becomes "Tell the lane, when it restarts at the pin, that SV1 is unblocked".
   - Row 6 (line 162): in its State cell, "with two musts named in its Record" becomes "with two
     musts named in its Record on that branch; the order's file on `main` has none".
   - Row 11 (line 167): its State cell, "049b, 051c, SSH-M; `s3/m2` merges after the rerun's
     pin", becomes "049a2, the second half of batch 1's order 049a (PND-008's pending actions for
     enrollment and ENR-012's removal question), after 047 and 048a; 049b, 051c, SSH-M; `s3/m2`
     merges after the rerun's pin". Its Next step stays "After 049a1".
     [049a1's order file](049a1-build-enrollment-core.md), lines 29 to 36, splits 049a this way.
   - Row 15 (line 171): its State cell loses "; A78's findings in increment 4's log". Order 155
     logged them as findings 4-21 to 4-27, as lines 95 to 99 of the handover say.
   - The bullet on lines 219 and 220, which begins "Providers: option (b)", becomes:

     > - Providers: option (b) of order 120, packaged local providers that are to pass the contract
     >   suites beside the second providers (PROVIDERS.md, decision B), as D45 of the full
     >   review's decisions changed it. No further work goes to `provider-local-machine`;
     >   `provider-local-process` stays as order 080's one-machine tracer and
     >   `provider-local-directory` only as a suite partner; no adapter wraps the built-in runner,
     >   whose built-ins are tested by running the suites' cases as data through the engine's own
     >   tests. The four contract suites hold no case yet (experiment 022), so no provider has
     >   passed one. "Self-contained" is read as a crate's own lockfile and toolchain.

     "PROVIDERS.md" is a link to [the providers' plan](../implementation/PROVIDERS.md), "D45 of the
     full review's decisions" to [the decisions](../proposals/full-review/decisions.md), and
     "experiment 022" to [its README](../../experiments/022-provider-contracts/README.md).
   - After the bullet on line 227, which begins with the quotation "Windows computer is up
     again", add:

     > - Ruling 72, decision 3: "No rented machine stands in", in IN_FORCE.md, topic 10, and in
     >   the ruling's record, is the coordinator's rendering. The owner's words, "We can use the
     >   windows machine whenever needed", answered a question about a fallback machine (FR-330,
     >   point 5). Struck, the sentence leaves topic 10 and decision 3.

     "IN_FORCE.md, topic 10" is a link to
     [that topic](../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), "the
     ruling's record" to [ruling 72's record](../proposals/ruling-72/README.md), and "FR-330" to
     the finding.
   - Nothing else in the handover changes: not lines 192 to 194 (D4, order 186), not line 22 or
     the queue's other rows and opening paragraph, which no finding names, and not any "(log)"
     mark.
3. **The tracker**, `docs/AUDIT.md` (FR-377 item 4, FR-344, FR-347, FR-276).
   - Line 60, the row "First build": "Order 070 removed finding 4-18's whole-state copies, except
     registration's four (the shell's library tests: 1900 s to 100 s)." becomes "Order 070 removed
     finding 4-18's whole-state copies, except registration's four (the shell's library tests:
     1900 s to 100 s); order 042 then added two, so registration copies the whole state at six
     sites."
   - Line 61, the row "Secrets and machines": "its core, order 049a1, for the side branch `s3/m2`,
     is checkpointed unmerged after five rounds." becomes "the first half of its core, order 049a1,
     for the side branch `s3/m2`, is checkpointed unmerged after five rounds; the second half,
     049a2 (PND-008's pending actions for enrollment and ENR-012's removal question), follows
     orders 047 and 048a and is not written."
   - Line 62, the row "Readiness gate": its first link stays, and so does its last sentence,
     "Round 6 was unsafe; the next cold read judges G7." Between them, the three sentences from
     "The confirmed assessment" to "coordinator's until G11." become:

     > The coordinator assessed G8 to G10 on 2026-11-11, under ruling 58 point 7, and found none
     > met (the assessment); it stays the coordinator's until G11. G1 to G7 have no current
     > assessment: the last, confirmed before rulings 40 to 43, met none and partly G1 and G3,
     > and is historical.

     "the assessment" is a link to
     [it](../implementation/INDEPENDENT_BUILDS.md#current-assessment-of-g8-g9-and-g10).
   - Line 174, the row on ruling 77's setup: "can run now: orders 071 and 080 are checkpointed and
     the computer is idle" becomes "can run now: order 080 is checkpointed and the computer is
     idle".
   - Line 235, the bullet on increment 4: after the sentence on 4-18, which ends with the link to
     order 070's Record, add the sentence "Order 042 then added two such copies, so registration
     copies the whole state at six sites."
   - Nothing else changes, "At the handover of 2026-11-12" and line 176 (D4) included.
4. **The roadmap**, `ROADMAP.md` (FR-377 item 2 and its verifier). Each State still begins with
   an allowed word, as `tools/check_roadmap.py` requires.
   - Step 4 (line 29): its State cell, "later", becomes:

     > later: from the S1 rerun's pin, when a lane runs again (ruling 78, point 1). Begun on
     > 2026-11-11: A63's runtime kit, contract harness and first packaged providers, SSH-R among
     > them, are merged, as the providers' status gives them, and S10's plan is written (order
     > 119); the four contract suites hold no case yet

     "ruling 78" is a link to [its record](../proposals/ruling-78/README.md), and "the providers'
     status" to [that section](../implementation/PROVIDERS.md#status).
   - Step 5 (line 30): its State cell, "later", becomes:

     > later: from the S1 rerun's pin, when a lane runs again (ruling 78, point 1). Begun on
     > 2026-11-11: the plans of S2 to S9 are written (orders 117 to 119, revised by orders 121
     > and 134), and S2's Linear stub, Conduit-aware server and adapter, S4's effect servers
     > (order 143) and the scripted client SCR1 are merged; none of S2 to S9 has run

     "ruling 78" is the same link.
   - A79 (line 72): in its State cell, between "not an author run; " and "follow-up on
     2026-11-11", insert "rerun on 2026-11-11, after orders 039 to 041a: 14 exact states passed and
     0 failed, and the 14 probes without `as` passed (its evidence); ". "its evidence" is a link to
     [the run's output](../../experiments/011-short-form-spike/evidence/run-2026-11-11.txt), which
     `experiments/011-short-form-spike/README.md` lines 94 to 95 describe.
   - No other row changes; order 180 changes the rows of experiments 012 and 004.
5. **IN_FORCE.md, topic 11** (FR-317 item 2). PRV-030 came from experiment 012's MC4-009, written
   into `spec/` by [order 081](081-spec-one-run-per-instance.md), and `IN_FORCE.md` cites it
   nowhere. At the end of line 262, the item that begins "Providers as packages", "44 6-1-05)."
   becomes "44 6-1-05), and PRV-030 (experiment 012, MC4-009; order 081)." The range "PRV-001 to
   PRV-028" stays: PRV-029 is cited in topic 9 with its own rulings, and rulings 40 to 44 did not
   make PRV-030. Nothing else in the file changes.
6. **INDEPENDENT_BUILDS.md** (FR-377 item 3), in `docs/implementation/`.
   - Lines 7 to 9: "`conformance/` and the branches named below do not exist yet; their names are
     proposals. `engine/` holds the code of the first build (ruling 46), which is not one of these
     builds." becomes "The branches named below do not exist yet; their names are proposals.
     `conformance/` holds the vectors the first build runs, and `engine/` the code of the first
     build (ruling 46), which is not one of these builds." Rewrap that paragraph only.
   - Lines 78 and 79: "`spec/`, `tools/`, `docs/` and `experiments/` exist. The others are proposed
     and do not exist yet." becomes "`spec/`, `tools/`, `docs/`, `experiments/`, `conformance/`,
     `engine/` and `packages/` exist on `main`; the others are proposed and do not exist yet."
   - In the table, the Exists cell of `conformance/` (line 85) and of `packages/` (line 88)
     becomes "Yes", and that of `engine/` (line 87) becomes "Yes, holding the first build (ruling
     46)". No other cell changes.
7. **The pipeline's pages** (FR-276).
   - `docs/implementation/METHOD_AS_RUN.md`, lines 157 and 158: "124 on 122, 125 on 123, 130 on
     129, 148 on 142; 149 and 150 on 146 (#260)." becomes "114 on 111, 115 on 113, 124 on 122, 125
     on 123, 130 on 129, 140 on 137, 148 on 142; 149 and 150 on 146 (#260)." The Records of 114,
     115 and 140 each say that it was stacked on that order's last commit.
   - `docs/implementation/PROVIDERS.md`, lines 44 and 45: "with two musts left for a finishing
     order, named in its Record." becomes "with two musts left for a finishing order, named in its
     Record on that branch; the order's file on `main` has none."
   - `docs/work-orders/README.md`. The table under "Orders still in the directory" lists 32 of the
     107 order files on `main`. Make it list, once each, every file in the directory whose name
     begins with a digit and ends in `.md`, in the order that Python's `sorted` gives their names.
     Each row keeps the form of the rows there now. Its first cell is the file's name up to its
     first hyphen, as a link to the file. Its second cell is the row's text unchanged, for a file
     that has a row now; for any other file, it is the file's first line with everything up to
     and including the first colon and the space after it removed. Between the heading and the
     table, add the line "One row for each order file in this directory, in file-name order."
     Then lines 59 and 60, "While an order is in flight, its admin Record stays in its file; at
     merge, the admin writes its row serially in the record before removing that order and its
     own schema.", become:

     > An order keeps its Record at the foot of its file after it merges, until a condensation
     > (A80) carries it into the record: the admin then writes its row there serially, and removes
     > the order and its own schema.

     "the record" is a link to [the record](../implementation/WORK_ORDERS.md#the-record), whose
     lines 134 to 136 say the same. Nothing else in the README changes.
8. **Registration's six copies** (FR-347's verifier). Order 070 left four whole-state copies in
   registration; order 042, merged after it, added two (`engine/crates/core/src/records.rs` lines
   73 and 110, `engine/crates/core/src/registration.rs` lines 1776, 1987, 2497 and 2546).
   - `docs/guides/cold-readers.md`, lines 73 and 74: "and order 070 removed the whole-state copies
     of 4-18 except registration's four." becomes "and order 070 removed the whole-state copies of
     4-18 except registration's four, to which order 042 then added two."
   - `docs/proposals/first-build/log-increment-4.md`, row 4-18 (line 29): at the end of its last
     cell, after the link to order 070's Record and its full stop, add " Order 042, merged after
     it, added two more whole-state copies in registration, so six remain (FR-347)." "FR-347" is a
     link to the finding. No other row changes.
9. **The measurement plan** (FR-377's verifier),
   `docs/proposals/first-build/increment-4/measurement-plan.md`. It was written before the rerun,
   so it gains a dated note and loses nothing. In the bullet on lines 113 to 115, after "That
   comparison choice remains to be stated explicitly.", add:

   > Note of 2026-11-15: it has since been stated. Ruling 71, point 3, has both runs pass, the
   > coordinator's and the fresh Codex author's, each against the baseline (A70), and ruling 78,
   > point 2, scores each run in ruling 45's two groups.

   "Ruling 71" is a link to [its record](../proposals/ruling-71/README.md), and "ruling 78" to
   [its record](../proposals/ruling-78/README.md). Rewrap that bullet only, at its two-space
   indent.
10. **The engine's conformance page**, `engine/CONFORMANCE.md` (FR-354).
    - Lines 3 to 7: "This document states the choices implemented through increment 2 and shell
      orders 020/021/022, under the admin's decisions and order 020, and distinguishes them from
      choices the shell must still make." becomes "This document states the choices implemented
      in increment 2, under the admin's decisions, and in the shell orders that followed it from
      order 020 on, and distinguishes them from choices the shell must still make." The links on
      "decisions" and "order 020" stay. Rewrap that paragraph only.
    - In "Choices still required", after the row DL17 (line 62), add two rows. "requests.rs" in
      each is a link to [the core's requests](../../engine/crates/core/src/requests.rs), as in the
      row "DL2: contents"; lines 1049 to 1057 there hold both stubs.

      > | DL18 | Not built: `sign-in.v1` answers unavailable under PND-010, with the text "not built yet, order 048a" (requests.rs), so no URL action waits and no wait is chosen. |
      > | DL19 | Not built: `answer.v1` answers unavailable under PND-009, with the text "not built yet, order 047" (requests.rs), and the core holds no pending action, so no expiry is chosen. |

    - Lines 269 and 270: "Runtime providers, remote connections and secrets still have the
      unserved boundaries listed above; this order qualifies the built-in runner on S1, not those
      providers." becomes:

      > Runtime providers and remote connections still have the unserved boundaries listed above,
      > in the row "DL16: executor coverage". Secrets are unserved too: the engine reads no secret
      > through a secret store, so its reads under SEC-010, SEC-013 and SEC-023 are not built, and
      > nothing calls the file secret store in `packages/provider-file`. This order qualifies the
      > built-in runner on S1, not those providers.

      Rewrap from the line where the change begins to the paragraph's end. No line of
      `engine/crates/` names `secret-provider`, and the core names only SEC-004, SEC-007 and
      SEC-008.
11. **The engine's and the vectors' READMEs** (FR-354).
    - `engine/README.md`, lines 51 and 52: "`State` holds started, objects, records, allocated_ids,
      record_gaps, invocations, installed_packages and preinstalled_snapshots." becomes "`State`
      holds started, objects, records, allocated_ids, tombstones, bound_targets, record_gaps,
      invocations, installed_packages and preinstalled_snapshots.", the fields in the order of
      `engine/crates/core/src/lib.rs` lines 75 to 93. Rewrap that paragraph only.
    - `engine/README.md`, line 64: the Values cell of `first-start`, "evidence, machine", becomes
      "evidence, machine, trustDomain" (`lib.rs` lines 209 to 214).
    - `conformance/README.md`, lines 606 to 608: "The inventory currently has 217 rows with vectors
      and 91 with reasons, including loader tests not yet ported, missing boundary cases and shell
      work." becomes "When increment 2 ended, the inventory had 217 rows with vectors and 91 with
      reasons; on 2026-11-15 it has 226 with vectors and 82 with reasons. The reasons include
      loader tests not yet ported, missing boundary cases and shell work." The counts are what the
      second command under Done when prints. If it prints others, write those with the date of
      your run, use them in the last of the checks under Done when that print 1, and say so in
      your report. Rewrap that paragraph only.
    - `conformance/README.md`, lines 855 and 856, in increment 2's done-when: "217 have vectors;
      91 have explicit reasons." becomes "217 then had vectors and 91 explicit reasons; the
      inventory's section above gives the later count." "the inventory's section above" is a link
      to [that section](../../conformance/README.md#s1-whole-and-promise-inventory-i2-09).
12. **The experiments' index**, `experiments/README.md` (FR-537).
    - Line 31, experiment 012's Current evidence cell: "No Conduit promise is checked yet." becomes
      "Models of the safety promises followed: MC2 to MC7b on 2026-11-11 (orders 105 and 107 to
      115), MC2's rerun on 2026-11-15 (order 160), and G5's account of the convention-layer
      safety promises (order 116)." "G5's account" is a link to
      [G5.md](../../experiments/012-model-checking/G5.md).
      Leave the cell's link to the results as `main` has it when you start.
    - Lines 59 and 60: "the harness and its self-test of 2026-11-11. No Conduit promise is checked
      yet." becomes "the harness and its self-test of 2026-11-11, then models MC2 to MC7b of the
      safety promises and G5's account. Each model is of the rules the promises state, not of an
      engine's code, and a bounded check shows no unbounded safety." "G5's account" is the same
      link. Rewrap that bullet only.
13. **Ten headings that read as current** (FR-377 item 6). Each scenario plan, and the providers'
    plan, names in its status lines the commit it was written against and any it was revised
    against, and its section "What `main` has now" describes `main` then: S2's says that 042 and
    070 are absent. In each of these ten files, the heading "## What `main` has now" becomes
    "## What `main` had at the commits named above", and nothing else in the file changes for this
    decision:
    `docs/implementation/PROVIDERS.md` (line 71), and `plan.md` in each of
    `experiments/013-s2-shared-login/`, `014-s3-two-machines/`, `015-s4-partial-setup/`,
    `016-s5-control-contention/`, `017-s6-revoke-in-flight/`, `018-s7-outside-package/`,
    `019-s8-filtered-search/`, `020-s9-start-and-repair/` and `021-s10-swapped-providers/`. No
    link in the tree names the old heading's anchor.

## Scope

Change only these files, and in each only what the decisions name:

- `docs/implementation/HANDOVER.md` (decision 2);
- `docs/AUDIT.md` (decision 3);
- `ROADMAP.md` (decision 4);
- `IN_FORCE.md`, line 262 (decision 5);
- `docs/implementation/INDEPENDENT_BUILDS.md` (decision 6);
- `docs/implementation/METHOD_AS_RUN.md`, `docs/implementation/PROVIDERS.md` and
  `docs/work-orders/README.md` (decisions 7 and 13);
- `docs/guides/cold-readers.md` and `docs/proposals/first-build/log-increment-4.md` (decision 8);
- `docs/proposals/first-build/increment-4/measurement-plan.md` (decision 9);
- `engine/CONFORMANCE.md` (decision 10);
- `engine/README.md` and `conformance/README.md` (decision 11);
- `experiments/README.md` (decision 12);
- the nine scenario plans of decision 13.

Change nothing under `spec/`, no case, vector or code, no ruling's record, no order's file, and
none of the review's files. No case changes, so no declaration is written, and no promise
changes, so A24's rule in `CONTRIBUTING.md` does not apply.

## Done when

From the repository root, each command passes. This prints nothing:

```sh
git grep -n -F -e 'built and passing the contract suites' -e 'Tell it when 071 merges' -e '071 and 080 are checkpointed' -e "A78's findings in increment 4's log" -e 'No Conduit promise is checked yet' -e 'shell orders 020/021/022' -e 'currently has 217' -e 'What `main` has now' -e 'While an order is in flight, its admin Record' -e '`conformance/` and the branches named below' -- docs/implementation docs/AUDIT.md docs/work-orders/README.md engine conformance experiments
```

This prints `226 82`, or the counts that decision 11 then writes:

```sh
/usr/bin/python3 -c "import yaml; c = yaml.safe_load(open('conformance/manifest.yaml'))['s1_coverage']; w = sum(1 for r in c if r.get('vectors')); print(w, len(c) - w)"
```

This prints the number of order files twice, then `True []`. On `main` before your change it
prints `False` in third place, so it can fail:

```sh
python3 - <<'EOF'
import pathlib, re, subprocess
d = pathlib.Path("docs/work-orders")
pat = r"^\| \[([^\]]+)\]\(([^)]+\.md)\) \| (.+) \|$"
files = sorted(p.name for p in d.glob("[0-9]*.md"))
rows = re.findall(pat, (d / "README.md").read_text(), re.M)
head = subprocess.run(["git", "show", "HEAD:docs/work-orders/README.md"],
                      capture_output=True, text=True, check=True).stdout
old = {f: w for _, f, w in re.findall(pat, head, re.M)}
links = [f for _, f, _ in rows]
bad = [f for n, f, w in rows if n != f.split("-")[0]
       or w != old.get(f, (d / f).read_text().split("\n", 1)[0].split(": ", 1)[1])]
print(len(files), len(links), links == files, bad)
EOF
```

Each of these prints 1:

```sh
tr -s ' \n' '  ' < docs/implementation/HANDOVER.md | grep -c -F 'The four contract suites hold no case yet'
tr -s ' \n' '  ' < docs/implementation/HANDOVER.md | grep -c -F '"No rented machine stands in"'
tr -s ' \n' '  ' < docs/implementation/HANDOVER.md | grep -c -F 'Tell it that 071 has merged (#281), so SV1 is unblocked'
grep -c -F 'Merged on 2026-11-15 (#281' docs/implementation/HANDOVER.md
grep -c -F '049a2, the second half of batch 1' docs/implementation/HANDOVER.md
grep -c -F 'order 042 then added two, so registration' docs/AUDIT.md
grep -c -F 'the second half, 049a2' docs/AUDIT.md
tr -s ' \n' '  ' < docs/AUDIT.md | grep -c -F 'G1 to G7 have no current assessment'
tr -s ' \n' '  ' < docs/AUDIT.md | grep -c -F 'Order 042 then added two such copies'
grep -c -F 'rerun on 2026-11-11, after orders 039 to 041a' ROADMAP.md
grep -c -F '44 6-1-05), and PRV-030 (experiment 012, MC4-009; order 081).' IN_FORCE.md
tr -s ' \n' '  ' < docs/implementation/INDEPENDENT_BUILDS.md | grep -c -F '`engine/` and `packages/` exist on `main`'
tr -s ' \n' '  ' < docs/implementation/METHOD_AS_RUN.md | grep -c -F '114 on 111, 115 on 113, 124 on 122, 125 on 123, 130 on 129, 140 on 137, 148 on 142'
tr -s ' \n' '  ' < docs/implementation/PROVIDERS.md | grep -c -F 'named in its Record on that branch'
tr -s ' \n' '  ' < docs/guides/cold-readers.md | grep -c -F 'to which order 042 then added two'
grep -c -F 'added two more whole-state copies in registration' docs/proposals/first-build/log-increment-4.md
tr -s ' \n' '  ' < docs/proposals/first-build/increment-4/measurement-plan.md | grep -c -F 'Note of 2026-11-15: it has since been stated.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c -F 'Secrets are unserved too'
tr -s ' \n' '  ' < engine/README.md | grep -c -F 'allocated_ids, tombstones, bound_targets, record_gaps'
grep -c -F '| first-start | evidence, machine, trustDomain |' engine/README.md
tr -s ' \n' '  ' < conformance/README.md | grep -c -F '217 then had vectors and 91 explicit reasons'
tr -s ' \n' '  ' < experiments/README.md | grep -c -F 'Models of the safety promises followed'
tr -s ' \n' '  ' < conformance/README.md | grep -c -F 'it has 226 with vectors and 82 with reasons'
```

Each of these prints 2, 3, 10 and 2, in turn:

```sh
grep -c -E "^\| [45] \| .* \| later: from the S1 rerun's pin" ROADMAP.md
grep -c -E '^\| `(conformance|engine|packages)/` \| [^|]+ \| Yes' docs/implementation/INDEPENDENT_BUILDS.md
git grep -l -F '## What `main` had at the commits named above' -- docs/implementation experiments | wc -l
grep -c -E '^\| DL1[89] \| Not built' engine/CONFORMANCE.md
```

This prints nothing: no line you wrote, outside a table row, `IN_FORCE.md` and `docs/AUDIT.md`,
is longer than 100 characters:

```sh
git diff -U0 -- . ':!IN_FORCE.md' ':!docs/AUDIT.md' | grep '^+[^+]' | grep -v '^+|' | awk 'length > 101'
```

Then each of these passes, with the last line given:

```sh
python3 tools/check_docs.py          # Errors: 0
python3 tools/check_roadmap.py       # Errors: 0
python3 tools/check_rulings.py       # Errors: 0
python3 tools/check_private_paths.py # Errors: 0
git diff --check
git status --short                   # the 24 files of Scope, and nothing else
```

The counts command runs under `/usr/bin/python3` because it needs PyYAML. Do not stage anything.
In your report, mark each choice you made yourself as your own, give the output of the README
check and of the counts command, and give the last line of every check.

The admin then stages the changes and runs `python3 tools/check_private_paths.py` with the
private names set, and `git diff --cached --check`. On the pull request, CI's `docs` job runs the
checks again. A fresh Codex verifier and a fresh read-only Opus reader read the result before it
merges ([ruling 79](../proposals/ruling-79/README.md), point 2).

## Record

Run on 2026-11-15 on the Mac from `main` `2f33a53e`: the worker (316 s) and a fresh Codex verifier
(255 s), which found nothing to change in one round. The order changed the 24 files of its Scope
and nothing else. The handover says that 071 merged (#281) and that SV1 is unblocked, names
049a2 as the second half of 049a, drops row 15's phrase on A78's findings (order 155 logged them
as 4-21 to 4-27), rewords the providers' bullet as D45 changed it, with the four contract suites
holding no case yet, and records ruling 72's "No rented machine stands in" as the coordinator's
rendering. The tracker gives registration's six whole-state copies, 049a2, the readiness gate's
assessment of G8 to G10 alone, and only 080 as checkpointed in the row on ruling 77's setup.
`ROADMAP.md`'s steps 4 and 5 say what has begun, and A79 gains its rerun of 2026-11-11.
`IN_FORCE.md`'s topic 11 cites PRV-030. `INDEPENDENT_BUILDS.md` says that `conformance/`,
`engine/` and `packages/` exist. The method as run gains the stacked pairs 114 on 111, 115 on 113
and 140 on 137. The order README's table has one row for each of the 112 order files, and it
says that an order keeps its Record in its file until A80's condensation carries it into the
record. The cold readers' guide and row 4-18 of increment 4's log give the six copies, and the
measurement plan gains a dated note on rulings 71 and 78. `engine/CONFORMANCE.md`'s opening
names the shell orders from 020 on, its rows DL18 and DL19 say what is not built, and it says
that secrets are unserved. `engine/README.md` gives `State`'s fields and `trustDomain`, and
`conformance/README.md` the counts of 2026-11-15: 226 rows with vectors and 82 with reasons.
Experiment 012's lines in the experiments' index name its models MC2 to MC7b, MC2's rerun and
G5's account. Ten headings now read "What `main` had at the commits named above". The review's
order 36 also asks that `grep` find no "PRV-001 to PRV-028" in `IN_FORCE.md`. That check is not
met, on purpose: decision 5 keeps the range, since PRV-029 is cited in topic 9 with its own
rulings and rulings 40 to 44 did not make PRV-030, and it adds PRV-030 with its source instead.

A fresh read-only Opus reader then verified the order (ruling 79, point 2). It ran every command
of Done when (the counts command printed `226 82`, the README script `112 112 True []`), with
`check_docs`, `check_roadmap`, `check_rulings`, `check_private_paths` and `check_model_account`
(each Errors: 0) and `git diff --check origin/main...HEAD`. Once the checkpoint commit exists,
the README script compares HEAD's README with itself, and the line-length check finds no
unstaged change to read. So it reran both against `origin/main`, and both passed. It checked
that the changed files are the 24 of Scope, read each new statement against the tree and found
no must. Three of its four shoulds were applied before merge. The secrets sentence says that no
code under `engine/` calls the file secret store, since the package's own tests call it. The
handover's row 6 and `PROVIDERS.md` say that 144's file on `main` has no Record. This Record
says why order 36's PRV check is not met. The fourth should is for the admin: a check in
`tools/check_docs.py` that the README's table lists each order file, since nothing checks it
now. It is outside this order's Scope, so until it is written the admin adds a row for each
order written.

A second fresh Opus reader read the coordinator's fix (ruling 79, point 2) and found no must. Its one
should was applied: the README check's follow-up is now in the handover's row 15, beside the docs
pass.
