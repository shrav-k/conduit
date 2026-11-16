# Work order 186: the scenario plans brought to the tree

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order changes 21 documents and no code. It
has no network access and needs none. It needs no cargo command; if you run one, run one at a
time. Never print a whole large file: read the plans, the READMEs, `docs/AUDIT.md` and the
handover with `grep -n` and `sed -n`. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 35 of [its orders](../proposals/full-review/orders.md))
found the plans of S2 to S10, experiments 013 to 021, out of step with what was decided after
the owner accepted them on 2026-11-11 under [ruling 75](../proposals/ruling-75/README.md), D:

- Four plans, S3's, S7's, S8's and S10's, the tracker's waiting list and the handover still make
  counted runs wait for the owner to confirm the admin's reading of ruling 75 D4, and the other
  five plans still call that reading provisional. The owner confirmed it on 2026-11-12, in
  [ruling 75's later answers](../proposals/ruling-75/README.md#later-answers), and IN_FORCE.md's
  topic 13 holds D4 "as its later answer reads it"
  ([FR-388](../proposals/full-review/findings/E01-s2-s3-s4.md#fr-388),
  [FR-404](../proposals/full-review/findings/E02-s5-s6-s7.md#fr-404) as its verifier corrected
  it, [FR-419](../proposals/full-review/findings/E03-s8-s9-s10.md#fr-419) item 1, with the lines
  its verifier added).
- S6's plan gives S6-WIRE a "T4 restart case", which the accepted S6-T4 does not have, and with
  it a dependency on S9's shell order S9-CUT2
  ([FR-397](../proposals/full-review/findings/E02-s5-s6-s7.md#fr-397), point 7, which its
  verifier makes the coordinator's).
- No accepted README has changed since acceptance except by the line that points to its plan,
  while rulings and the coordinator's logged decisions changed how they read. A third party
  cannot tell which text governs a count, and ruling 45 asks that it be fixed before the run.
  Some passages are now wrong: S3's fault preparation, which ruling 77's lockdown made impossible
  to follow; S3's G4 and V7, which wait for enrollment promises that now exist; and "What it
  needs first" in S5 and S6, which calls implemented work missing
  ([FR-390](../proposals/full-review/findings/E01-s2-s3-s4.md#fr-390) as its verifier corrected
  it, [FR-404](../proposals/full-review/findings/E02-s5-s6-s7.md#fr-404)).

[Ruling 78](../proposals/ruling-78/README.md), point 4, answered the review's question 4. It
amends S2's plain server and measure D20, S3's fault method, O4's command, P4 and runs, and S7's
tasks and run, and it applies the coordinator's other changes in the review's table for S2 to
S10, each dated and open to the owner's strike; its points 5 and 6 amend S5's P5, T5, V4 and
`occurred: false` branches (IN_FORCE.md, topic 13). The coordinator's decision D64 gives each
accepted README one dated section, "Amendments since acceptance", beneath its accepted text,
makes each section one row on the owner's review list, and has the same order clear the waits
the owner has already answered
([the review's decisions](../proposals/full-review/decisions.md#the-experiments-documents-and-the-register),
D64 to D73). Ruling 78 has since answered two more waits, S3-OF and S2's plain server, so this
order clears them too. S3's hold on MC4-002, which D64 also names, is order 42 of the review's
list, work order 184, and stays out of this order. Order 168 edited S4's plan, and order 155
(the review's order 11, on which this one depends) has merged: build on the files as they are on
`main`.

## The decisions it applies

Line numbers are those of `main` at `ab07fd47`. "S2's :78" means line 78 of S2's plan,
`experiments/013-s2-shared-login/plan.md`, and so on for S3 to S10 in experiments 014 to 021; a
README's lines are named as such. If a line has moved, find it by its words. Text in a quoted
block below is written word for word, without the quote marks, with DATE replaced by the date
you write it, as YYYY-MM-DD. In the plans, edit in place: keep a line's own breaks, and a
sentence that replaces another stays where the old one was. In the READMEs' new section and the
review list's new section, wrap prose at most 100 characters a line, with each link whole on one
line, because `check_docs.py` checks a link only when its text and its target share a line; a
table row stays on one line. The plans and READMEs are two directories below the root, as this
order is, so the links in the quoted blocks are written as they serve from there. In a line you
edit, change only what the decision says, and keep the line's own apostrophes and quotation
marks.

1. **The answered D4 waits are struck.** The owner's confirmation of 2026-11-12 is stated where
   the plans asked for it, and nothing waits on it any longer.
   - In each of the nine plans, the sentence "Provisional until the owner confirms." that ends
     the line beginning, or the item containing, "Planned on the admin's reading of ruling 75 D4
     (2026-11-11)" becomes the block below: S2's :278, S3's :241, S4's :275, S5's :186, S6's
     :202, S7's :160, S8's :183, S9's :186 and S10's :224. The same words close S10's A4, on
     A73's "or else", at :231; that line stays.

     > The owner confirmed this reading on 2026-11-12 ([ruling 75's later answers](../../docs/proposals/ruling-75/README.md#later-answers)).

   - In the opening summaries, the text after each line's leading quote mark and dash becomes:
     - S3's :12: Owner: confirm A74 unmeasured-cell treatment; D4's reading was confirmed on
       2026-11-12.
     - S4's :12: Admin C: D4 reading recorded below, confirmed by the owner on 2026-11-12.
     - S7's :16: **Owner:** provide author's repository/npm scope; D4's reading was confirmed on
       2026-11-12.
     - S8's :15: D4's reading for counted real-client runs was confirmed by the owner on
       2026-11-12.
     - S10's :31: **Owner:** ruling 77's `tailscale serve` setup after the Mac's tailnet rename,
       with ADDR and DSP, gates Docker T3/T5 and R2; D4's reading was confirmed on 2026-11-12.
   - In the order tables, each "Starts" cell loses "; the owner confirms D4's reading": S3's :86
     to :89, S7's :74 and :76, S8's :83 and :84, S10's :101 and :102.
   - In the prose:
     - S3's :190: "Pin inputs before timing; counted runs wait: the owner confirms D4's reading."
       becomes "Pin inputs before timing."
     - S3's :231 and :232: "S3-OF's answer, remaining owner holds and "the owner confirms D4's
       reading" on counted runs determines R1 → R2 → R3 → R4. No other order waits on D4."
       becomes "S3-OF's answer and remaining owner holds determine R1 → R2 → R3 → R4."
     - S3's :249 and :250, the item "D4" under "For the owner", becomes:

       > - **D4:** the owner confirmed on 2026-11-12 the admin's reading recorded above (A3), so
       >   S3-R1–R4 do not wait on it.

     - S7's :150 and :151: "Counted R/N also wait: the owner confirms D4's reading. Only those
       runs wait on it; no A22/A73/batch-2 wait." becomes "No A22/A73/batch-2 wait."
     - S7's :164 and :165, the item "D4" under "For the owner", becomes:

       > - **D4:** the owner confirmed on 2026-11-12 the admin's reading recorded above (A2), so
       >   S7-R/N do not wait on it.

       Its last clause, "no scripted substitute for real consuming Agents is selected", goes with
       it: under ruling 78, point 4, S7's outside author works against a scripted consumer
       (decision 7).
     - S8's :172 and :173: "R1 → R2; only the counted runs wait: the owner confirms D4's
       reading." becomes "R1 → R2."
     - S8's :187: "Counted R1/R2 wait: the owner confirms D4's reading; preparation does not
       wait." becomes "The owner confirmed D4's reading on 2026-11-12 (A2 above), so R1/R2 do not
       wait on it."
     - S10's :210 and :211: "Only counted runs wait: the owner confirms D4's reading. T3/T5 and
       R2's provider-run engine access separately waits for" becomes "T3/T5 and R2's
       provider-run engine access waits for".
     - S10's :236 and :237: "Counted R1/R2 wait: the owner confirms D4's reading. Only those runs
       wait; all independent preparation proceeds." becomes "The owner confirmed D4's reading on
       2026-11-12 (A2 above), so R1/R2 do not wait on it." The sentence on ruling 77 that follows
       stays.
   - `docs/AUDIT.md` :176, the row of "Waiting for the owner" that begins "The scenario plans'
     other questions": its first cell loses "the admin's reading of ruling 75 D4, which the
     counted runs of S3, S7, S8 and S10 wait on; ", and its second cell loses the link to S10's
     plan, so that it ends with the links to S7's and S8's plans joined by "and".
   - `docs/implementation/HANDOVER.md` :192 and :193: ", and the reading of ruling 75 D4 that
     counted runs wait on" goes, so that "A74's unmeasured cells" is followed by the link to S3's
     plan. Nothing else in the handover changes.

2. **S3-OF and S2's plain server are answered.** [Ruling 78](../proposals/ruling-78/README.md),
   point 4, answered both on 2026-11-14, so the lines that still put them to the owner say so.
   S3-H's method for option (E), and the server the coordinator picks, belong to the orders that
   build them.
   - S3's :18, in the opening summary, after the leading quote mark and dash: "Owner step S3-OF:
     choose the cut/heal/loss method before S3-H's fault implementation and the runs." becomes:
     S3-OF, answered on 2026-11-14 by ruling 78, point 4: option (E), the Mac's own firewall.
     The owner adds its one administrator rule once, before S3-H's fault implementation.
   - S3's :71, the row S3-OF: its "What it does" cell becomes "Add once the administrator rule on
     the Mac under which the coordinator's locked-down script cuts the network (S3-OF, answered
     by ruling 78, point 4: option (E))"; its "Promises" cell becomes "README T4/T5/T6, rulings 77
     and 78"; its "Size" cell becomes "one rule, once". Its other cells stay.
   - Under S3's "For the owner", just before the item that begins "**S3-OF — how should S3
     cut/heal the network and lose the machine?**" (:259), add this item:

     > - **S3-OF, answered on 2026-11-14:** [ruling 78](../../docs/proposals/ruling-78/README.md),
     >   point 4, chose option (E): the Mac's own firewall cuts the network, run by the
     >   coordinator's locked-down script under one administrator rule that the owner adds once;
     >   two test cuts decide whether it works, and a path it misses gets a relay on the Mac. The
     >   owner stops the distro at each loss with `wsl --terminate`. S3-H's fault implementation
     >   follows (E). The question below is kept as it was put, with its options (A) to (C) and
     >   the recommendation that (E) replaced.

   - S2's :78, the row S2-PL: its "Depends on" cell, "the plain server's choice, with the owner",
     becomes "the plain server's choice, by the coordinator (ruling 78, point 4)".
   - S2's :210 to :212: the paragraph's first sentence, from "**The plain server for the runs**
     is chosen before them, with the owner," to "(the admin's requirements, 2026-11-12).", becomes
     the block below. The rest of the paragraph, from "It must have an OSI-approved license",
     stays, rewrapped only as far as needed.

     > **The plain server for the runs** is chosen before them by the coordinator, and logged for
     > the owner to strike ([ruling 78](../../docs/proposals/ruling-78/README.md), point 4,
     > 2026-11-14). It runs under the owner's account against the test workspace, unconfined on
     > the Mac as the engine's own runner starts it (the admin's requirements, 2026-11-12).

   - S2's :313: "**The plain server:** chosen with the owner before S2's runs, under the
     requirements above;" becomes "**The plain server:** chosen by the coordinator before S2's
     runs, under the requirements above, and logged for the owner to strike (ruling 78, point 4,
     2026-11-14);". The rest of the item stays.
   - The lane's dated checkpoints at S2's :25 to :29 and S3's :30 to :34 are records of
     2026-11-12 and stay as they are; decision 4's paragraph follows them.

3. **S6's plan loses the T4 restart.** S6-T4 in S6's README (lines 112 to 114) retires a consumer
   while one of its invocations is active, with no restart. Decision D69 removes the plan's
   restart case with its dependency, so in S6's plan:
   - :80, the row S6-WIRE: its "Depends on" cell loses ", S9-CUT2 for T4 restart".
   - :137 and :138: ", and S9-CUT2 because T4's restart case uses its acknowledged lab repair
     cut" goes, so that the sentence reads "Wait S6-END and S4-LATE."
   - :186: "S4-LATE/S4-TASK2/S9-CUT2/DSP/DUTY1/2" becomes "S4-LATE/S4-TASK2/DSP/DUTY1/2".
   - :189 and :190: "WIRE follows S9-CUT2 and the other shared shell work." becomes "WIRE follows
     the shared shell work before it in that order."

   S6-END, S4-LATE and the rest of S6-WIRE stay: what ruling 78 means for them ("under point 6,
   S6-END goes and S4-LATE shrinks") belongs to the orders that build them. S4's :266, "Shell:
   DSP → S4-LATE → S4-TASK2 → S5-WIRE → S9-CUT1 → S9-CUT2 → S6-WIRE.", is the order of merges into
   shared files, which S6's :189 now points to without naming S9-CUT2, and stays, as do S9's own
   lines on S9-CUT2.

4. **Each plan says that the README's amendments govern.** In each of the nine plans, as the
   last paragraph of the section "Status", just before the heading that follows it (S2's :31,
   S3's :36, S4's :31, S5's :22, S6's :22, S7's :28, S8's :28, S9's :22, S10's :45), add this
   paragraph, with a blank line before it:

   > **Since acceptance (DATE, work order 186):** what has changed since the owner accepted the
   > README is listed in the README's last section, "Amendments since acceptance". Where this
   > plan differs from that section, the section governs, until the order that builds the
   > passage rewrites it.

   This covers what decisions 1 to 3 leave in the plans, such as S6's provider runs over the
   tailnet, S7's run rows from before the split, S8's V1 to V3 with no dependency on the filter
   contract, S4's two logged decisions on T1 and S10's A4: the orders that build those passages
   rewrite them.

5. **Each README points to its amendments, and no accepted line changes.** In each README, right
   after the line that begins "Its order-level plan is" (S2's README :35, S3's :34, S4's :35,
   S5's :32, S6's :32, S7's :31, S8's :31, S9's :32, S10's :31), add one line, in the same
   paragraph:

   > What has changed since acceptance is listed in the last section, "Amendments since acceptance".

   This order changes or removes no line of a README: every line it writes there is an addition.

6. **Each README ends with the section "Amendments since acceptance".** After the last line of
   its section "Limits", add one blank line, this block, one blank line and the README's lines
   from decision 7, as one list:

   > ## Amendments since acceptance
   >
   > The text above keeps what the owner accepted on 2026-11-11 under
   > [ruling 75](../../docs/proposals/ruling-75/README.md), D. Each line below is dated and says
   > whose it is. A line from a ruling amends the text above as that ruling says. A line marked
   > the coordinator's is open to the owner's strike: it changes no pass condition above, and what
   > it adds to a run is reported apart from the counted result.
   > [Ruling 78](../../docs/proposals/ruling-78/README.md), point 4, applied the coordinator's
   > changes in the table under question 4 of
   > [the full review's questions](../../docs/proposals/full-review/questions.md); the decisions
   > that these lines cite are in
   > [the full review's decisions](../../docs/proposals/full-review/decisions.md).

7. **The lines of each section.** Each is one list item, in the order given. They are the
   coordinator's text, drawn from ruling 78, from question 4's table and from the decisions and
   findings each names; add nothing to them.

   S2, `experiments/013-s2-shared-login/README.md`, three lines:

   > - **2026-11-14, [ruling 78](../../docs/proposals/ruling-78/README.md), point 4 (the owner):**
   >   the coordinator, not the owner, chooses the plain server for S2's runs: a pinned open-source
   >   server that meets the requirements under "The plain server for the runs" in
   >   [the plan](../../experiments/013-s2-shared-login/plan.md), logged for the owner to strike.
   >   Measure D20's list of places that can reach a credential also gains another Agent's home and
   >   client login. This amends ruling 75 D1 for S2.
   > - **2026-11-14, the coordinator's (decision D65):** an expected result is written for each task
   >   step; D05's cases are kept in one shared case file and run on both rows that are not plain,
   >   with one row made to send an onward request; P5 is judged against a snapshot of each Agent's
   >   access; D20's attempts are made by a scripted probe, run as each Agent's process, that reads
   >   only planted markers and never a real credential, and the real client's attempt is left to
   >   A7; for E07, each author writes down first, from inspection, which Conduits Agent A may open,
   >   and a miss is a finding, not a fail; each D01 row's cost is counted in lines; M1's trials and
   >   prompts are fixed before the runs; and one E17 case is added and reported apart: a call in
   >   flight while its Conduit closes.
   > - **2026-11-14, the coordinator's
   >   ([FR-390](../../docs/proposals/full-review/findings/E01-s2-s3-s4.md#fr-390)):** two decisions
   >   logged in [the plan](../../experiments/013-s2-shared-login/plan.md) change how this text
   >   reads, and each is open to the owner's strike there: what "adapter" means for D01's middle
   >   row (under "S2-L2"), and that the plain row runs only in the owner's runs, against the test
   >   workspace (under "The plain row"). D05's cases are listed in the contracts package's
   >   [README](../../packages/scenario-s2-linear-restrictions/README.md#d05-restriction-cases).

   S3, `experiments/014-s3-two-machines/README.md`, five lines:

   > - **2026-11-14, [ruling 78](../../docs/proposals/ruling-78/README.md), point 4 (the owner): the
   >   cut.** The Mac's own firewall cuts the network, run by the coordinator's locked-down script
   >   under one administrator rule that the owner adds once; two test cuts decide whether it works,
   >   and a path it misses gets a relay on the Mac. This replaces the Linux-side firewall rule and
   >   the coordinator's restoring script of "Fault preparation, apart from author tasks", which the
   >   lockdown of [ruling 77](../../docs/proposals/ruling-77/README.md) left impossible to follow
   >   ([FR-390](../../docs/proposals/full-review/findings/E01-s2-s3-s4.md#fr-390)). It answers S3-OF
   >   in the plan, and the table under question 4 of the full review's questions gives its details.
   > - **2026-11-14, ruling 78, point 4 (the owner): the loss.** At each loss the owner stops the
   >   distro with `wsl --terminate`, in place of the `wsl --shutdown` of "Fault preparation" and of
   >   O4's answer, which also stopped Docker Desktop's distro. Root still declares the node lost, and
   >   the provider reporting it gone stays a separate case.
   > - **2026-11-14, ruling 78, point 4 (the owner): a last phase.** Each run ends with a phase in
   >   which the machine is declared lost while a test run is alive, so that P4's owed stop is seen
   >   to end a run and can fail. The table under question 4 of the full review's questions
   >   describes the phase: a test server that ignores the end of its input, witnessed by the stop's
   >   ledger entry; a call that the old run makes to the engine after the declaration, which must
   >   be refused; and two runs that heal before sshd's 120 seconds and two after. With the two
   >   lines above, this amends ruling 75 D1 for S3's fault method, O4's command, P4 and runs.
   > - **2026-11-14, the coordinator's (decision D66), each reported apart:** the documents' changes
   >   from their one-machine form; the seconds from each cut to "unavailable", against SSH's 30,
   >   sshd's 120 and the machine provider's bound once it is written (the gate's 45 seconds applies
   >   on loopback only); the unit's state before and after each heal, with "nothing left to stop"
   >   reported; three ordering cells beside the accepted four, once the MC2 rerun has run and a
   >   scripted cut exists; the revocation axis, kept and labelled an exercise of reconnection; D20
   >   at each phase; a start that waits for an owed stop, tested in an undeclared cut with no pass
   >   condition; the cost of adding a machine; and a disposable Linux node on the engine's machine,
   >   used to rehearse and never as evidence.
   > - **2026-11-14, the coordinator's
   >   ([FR-390](../../docs/proposals/full-review/findings/E01-s2-s3-s4.md#fr-390)):** G4, under
   >   "What `spec/` does not yet decide", and V7, under "Vectors expected", wait for enrollment's
   >   promises. They are written: ENR-001 to ENR-012, in
   >   [spec/promises/enrollment.yaml](../../spec/promises/enrollment.yaml), and V7 cites those it
   >   tests when its case is written. The engine's enrollment bindings, which orders 049a and 049b
   >   land, are still spec pending in topic 10 of [IN_FORCE.md](../../IN_FORCE.md).

   S4, `experiments/015-s4-partial-setup/README.md`, two lines:

   > - **2026-11-14, the coordinator's (decision D67):** measure D02 is written as a twelve-row
   >   table; M2 gets a rubric, applied by a judge who has not seen the harness; the lifecycle model
   >   (MC6) is asked whether a Resource can outlive its retired Pool before E18's fixture is built;
   >   E20 is labelled an observation of a stated limit; and S4's one lost-response replay on a
   >   second account stays. Beside S4 and not part of it, a small stub like experiment 008's takes a
   >   lost response and a call never answered, three runs of each per client (A6's twelve runs), on
   >   the primary accounts, with the owner told first, after the pin
   >   ([ruling 78](../../docs/proposals/ruling-78/README.md), point 1).
   > - **2026-11-14, the coordinator's (decision D67,
   >   [FR-296](../../docs/proposals/full-review/findings/P03-plan-apply.md#fr-296)):** T1's repair
   >   cannot be a control call. After the blocked setup the hook's instance is not ready, and a
   >   request to an instance that is not ready is not dispatched unless it is a management verb,
   >   which a control is not (CON-024). So the author corrects the file and replaces the instance.
   >   This changes the plan's two logged decisions on T1, "T1's blocked part" and "How M2 counts
   >   T1's repair", which order 50 of
   >   [the full review's orders](../../docs/proposals/full-review/orders.md) rewrites. Whether apply
   >   ever calls a control is the owner's, asked with finding 4-01 after the S1 rerun.

   S5, `experiments/016-s5-control-contention/README.md`, three lines:

   > - **2026-11-14, [ruling 78](../../docs/proposals/ruling-78/README.md), points 5 and 6 (the
   >   owner):** P5, S5-T5 and V4 follow them. A reconciliation records the outcome and what was
   >   checked; it stores no control's values, makes nothing ready and leaves the settings reported
   >   unknown, and the way on is to send again: the control, naming the current revision, or a
   >   further setup. No actor records that an unknown effect did not occur, so S5-T5's
   >   `occurred: false` branches change with the later order that writes points 5 and 6 into
   >   `spec/`. This amends ruling 75 D1 for these passages; they are marked spec pending in
   >   [IN_FORCE.md](../../IN_FORCE.md) until that order lands.
   > - **2026-11-14, the coordinator's (decision D68):** M1 names the right next step for each of its
   >   five conflict classes; M2's branches are listed; the fixture's own contract checks that a
   >   readback equals what was sent; and a branch in which an Agent's control lands between plan and
   >   apply is reported apart, and runs only if batch 2 has merged.
   > - **2026-11-14, the coordinator's
   >   ([FR-404](../../docs/proposals/full-review/findings/E02-s5-s6-s7.md#fr-404)):** "What it needs
   >   first" says that controls and control reconciliation are outside the engine's implemented
   >   slice. Order 041b has since implemented control reconciliation, which applies the held input
   >   values to the current revision
   >   ([engine/CONFORMANCE.md](../../engine/CONFORMANCE.md#consumer-server-order-022)), and ruling
   >   78, point 5, narrows it as the first line here says. Control dispatch is still unfinished
   >   there.

   S6, `experiments/017-s6-revoke-in-flight/README.md`, two lines:

   > - **2026-11-14, the coordinator's (decision D69,
   >   [FR-397](../../docs/proposals/full-review/findings/E02-s5-s6-s7.md#fr-397)):** M1's count of
   >   forbidden dispatches after a close or a retirement is expected to be zero; M2's retained-data
   >   checks are a fixed list, written before the runs; the plan's "T4 restart case", which S6-T4
   >   above does not have, is removed with its dependency on S9's shell order; S6 keeps to one local
   >   machine, as "Runs" says; a child process that outlives a stop is a measure here, with no pass
   >   condition, and as a gap it goes to the language list and to a PRV-030 case in experiment
   >   022's runtime-provider suite; and the fixture's ledger is kept outside the state directory
   >   that removal deletes.
   > - **2026-11-14, the coordinator's
   >   ([FR-404](../../docs/proposals/full-review/findings/E02-s5-s6-s7.md#fr-404)):** "What it needs
   >   first" says that the conformance document leaves `retire-with-resources.v1` and DL1's order
   >   unfinished. Both are implemented: DL1's row in
   >   [engine/CONFORMANCE.md](../../engine/CONFORMANCE.md) states the order, independent members in
   >   stored order and the members of a cycle in ascending ID order, and that Pool and Fleet
   >   cascades are implemented, and `retire-with-resources.v1` is among the operations the engine
   >   runs publicly. What remains is to qualify them, as the plan's S6-DUTY1 and S6-DUTY2 do.

   S7, `experiments/018-s7-outside-package/README.md`, two lines:

   > - **2026-11-14, [ruling 78](../../docs/proposals/ruling-78/README.md), point 4 (the owner):** S7
   >   is split. The outside author works against a scripted consumer, so that part needs neither the
   >   owner's presence nor the second accounts. Real Agents on the second accounts then use the same
   >   packages, and the coordinator, not the outside author, sets them up. This amends ruling 75 D1
   >   for S7's tasks and run.
   > - **2026-11-14, the coordinator's (decision D70):** "intervention" is defined and counted by
   >   kind; the author's pack is frozen and hashed, and its log of files read is kept; places of
   >   doubt, files read outside `spec/` and time are measured; measure D01's middle row is the
   >   adapter as S2's plan logs it, or two rows with the reason; a second server and a self-check run
   >   after the counted tasks and are reported apart; and a limit is stated: a fresh session stands
   >   in for ruling 47's outside author.

   S8, `experiments/019-s8-filtered-search/README.md`, one line:

   > - **2026-11-14, the coordinator's (decision D71):** the claim is stated in two parts, the
   >   contract and the convention; an expectation for counted small adapters is stated before the
   >   run and is not a pass condition; the suite must fail seeded leaks; a hand-written expected
   >   table is P3's independent oracle; scripted exact-protocol checks run beside the Agents' tasks;
   >   and one observation goes to A7: whether a refused Agent reads the corpus with its own tools.
   >   V1 to V3 wait for the filter contract, as "Vectors expected" says.

   S9, `experiments/020-s9-start-and-repair/README.md`, one line:

   > - **2026-11-14, the coordinator's (decision D72):** once the change of
   >   [ruling 78](../../docs/proposals/ruling-78/README.md), point 17 (F39), lands after the pin, so
   >   that the engine sets nothing up at start, S9-T4's branch (a) and M2 expect no setup or hook
   >   dispatched at start for a registered or failed instance (OBS-004), and a ready instance's
   >   restart is recorded but not judged until `spec/` states it. M2 gets a rubric; the dispatches
   >   before the first request and the seconds until the engine listens are measured; and every
   >   recorded request names the caller that sent it.

   S10, `experiments/021-s10-swapped-providers/README.md`, one line:

   > - **2026-11-14, the coordinator's (decision D73):** a predicted set of edits for each swap,
   >   written from a paper count before the run, is reported beside measure D16 and is not a pass
   >   list, so an unlisted edit stays a deviation; D16's ledger records each edited document's kind
   >   and counts `lift.v1` requests; D15 names its six implementations; a ledger lists the behavior
   >   that differs across a swap; stage R's conditions 2 to 6 in
   >   [FR-410](../../docs/proposals/full-review/findings/E03-s8-s9-s10.md#fr-410) are run as checks
   >   of the invariants D16 states; and the plan's reading of A73's "or else", its item A4 for the
   >   admin, is the coordinator's and open to the owner's strike.

8. **The owner's review list gains one row for each section.** In
   `docs/proposals/first-build/to-review-with-the-owner.md`, between the paragraph that ends
   "Items 15 to 29 stand as decided." (lines 74 to 78) and "## What the owner ruled, for
   reference", add this block, so that one blank line stands before it and one after it:

   > ## The scenario plans' amendments, from the full review
   >
   > Ruling 78, point 4, applied on 2026-11-14 the coordinator's changes in the full review's table
   > for S2 to S10, each dated and open to the owner's strike. Each scenario's README lists them,
   > with the coordinator's corrections, in its last section, "Amendments since acceptance"; the
   > lines there that come from a ruling are the owner's. The coordinator's lines of each section
   > are one row here, as decision D64 asks.
   >
   > | # | Decision | Why it matters | Where |
   > | --- | --- | --- | --- |
   > | 30 | **The coordinator's lines in S2's amendments since acceptance** (decision D65 and FR-390) | They change how the runs are observed, counted and prepared, not a pass condition, and what they add is reported apart | S2's README |

   Rows 31 to 38 follow in the same form, with the same "Why it matters": S3 (decision D66 and
   FR-390), S4 (decision D67 and FR-296), S5 (decision D68 and FR-404), S6 (decision D69 and
   FR-404), S7 (decision D70), S8 (decision D71), S9 (decision D72) and S10 (decision D73). Make
   "Ruling 78" a link to ruling 78's record and "decision D64" a link to the full review's
   decisions, and make each "Where" cell a link whose text is the scenario's README, as written,
   to that README at the anchor of its new section. Write each path relative to
   `docs/proposals/first-build/`. If the list's last row is no longer 29 when you start, number
   from the row after it and say so in your report.

9. **What stays.**
   - The accepted text of each README, its opening "For the owner" summary included, whose
     "proposed" labels the summary's first line converts. The README lines that give D4 as the
     reason for scripted clients (S4's README :247, S5's :230, S6's :239, S9's :253) are accepted
     text, not waits.
   - In the plans, everything decisions 1 to 4 do not name: the sections "What `main` has now",
     the lines on ruling 77, S3's hold on MC4-002 (:19 and :258, order 42 of the review's list,
     work order 184), S4's two logged decisions on T1 (order 50 of that list), S10's A4, and
     every owner question still open: A74's unmeasured cells, S7-NO and S8-A.
   - `docs/AUDIT.md` and the handover beyond decision 1's two edits; `ROADMAP.md`, whose S3 and S7
     rows already cite ruling 78; `IN_FORCE.md`, `docs/implementation/PROVIDERS.md`,
     `engine/CONFORMANCE.md`, the second-machine guide, `spec/`, `conformance/`, `engine/`,
     `packages/`, the full review's own files and every other order's file. This order changes no
     promise, vector or case, so CONTRIBUTING.md's rule 8 asks nothing of it and it needs no
     declared file under order 174.

## Scope

Change only these 21 files, and in each only what the decisions name:

- the nine plans: `experiments/013-s2-shared-login/plan.md` and
  `experiments/014-s3-two-machines/plan.md` (decisions 1, 2 and 4),
  `experiments/017-s6-revoke-in-flight/plan.md` (1, 3 and 4), and
  `experiments/015-s4-partial-setup/plan.md`, `experiments/016-s5-control-contention/plan.md`,
  `experiments/018-s7-outside-package/plan.md`, `experiments/019-s8-filtered-search/plan.md`,
  `experiments/020-s9-start-and-repair/plan.md` and
  `experiments/021-s10-swapped-providers/plan.md` (1 and 4);
- the nine READMEs beside them, `experiments/013-s2-shared-login/README.md` to
  `experiments/021-s10-swapped-providers/README.md` (decisions 5 to 7);
- `docs/AUDIT.md` and `docs/implementation/HANDOVER.md` (decision 1);
- `docs/proposals/first-build/to-review-with-the-owner.md` (decision 8).

Work order 182 also changes `docs/AUDIT.md` and the handover, and work order 184 changes S3's
plan at :19 and :258. If either has merged when you start, build on `main` as it is, find each
line of this order by its words, and say in your report which lines had moved.

## Done when

From the repository root, each passes:

```sh
python3 tools/check_docs.py             # Errors: 0
python3 tools/check_roadmap.py          # Errors: 0
python3 tools/check_private_paths.py    # Errors: 0
git diff --check                        # prints nothing
git diff --name-only | wc -l            # 21
git diff --name-only | grep -v -E '^(docs/AUDIT\.md|docs/implementation/HANDOVER\.md|docs/proposals/first-build/to-review-with-the-owner\.md|experiments/0(1[3-9]|2[01])-[a-z0-9-]+/(README|plan)\.md)$'   # prints nothing
git diff --numstat -- 'experiments/*/README.md' | awk '$2 != 0' | wc -l   # 0: no README line changed or removed
```

Then each of these prints the number beside it. The commands that start with `tr` read the text
with its line breaks as spaces. Those marked "on main" print another number with the change
removed (on `main` at `ab07fd47`).

```sh
git grep -n -E 'confirms? D4|S9 only\. Provisional|provisional D4|waits? on D4|D4, which the counted runs|D4 that counted runs' -- experiments docs/AUDIT.md docs/implementation/HANDOVER.md | wc -l   # 0; on main, 36
git grep -l 'The owner confirmed this reading on 2026-11-12' -- experiments | wc -l   # 9; on main, 0
grep -c 'Provisional until the owner confirms' experiments/021-s10-swapped-providers/plan.md   # 1: A4 stays
sed -n '/^## Waiting for the owner/,/^## Known to be open/p' docs/AUDIT.md | grep -c '021-s10-swapped-providers'   # 0; on main, 1
git grep -n -E 'T4 restart|restart case|S9-CUT2' -- experiments/017-s6-revoke-in-flight/plan.md | wc -l   # 0; on main, 4
grep -c 'S9-CUT2' experiments/015-s4-partial-setup/plan.md experiments/020-s9-start-and-repair/plan.md   # 1 and 2, as on main
grep -c -e 'choice, with the owner' -e 'before them, with the owner' -e 'chosen with the owner before' experiments/013-s2-shared-login/plan.md   # 0; on main, 3
grep -c -e 'Owner step S3-OF' -e 'cut, heal and loss method (A/B/C below)' experiments/014-s3-two-machines/plan.md   # 0; on main, 2
grep -c 'S3-OF, answered on 2026-11-14' experiments/014-s3-two-machines/plan.md   # 2; on main, 0
git grep -l 'work order 186' -- 'experiments/*/plan.md' | wc -l   # 9; on main, 0
for f in experiments/01[3-9]-*/README.md experiments/02[01]-*/README.md; do grep '^## ' $f | tail -1; done | grep -c '^## Amendments since acceptance$'   # 9; on main, 0
for f in experiments/01[3-9]-*/README.md experiments/02[01]-*/README.md; do grep -c '^What has changed since acceptance is listed in the last section' $f; done | grep -c '^1$'   # 9; on main, 0
S='/^## Amendments since acceptance$/,0'
for f in experiments/01[3-9]-*/README.md experiments/02[01]-*/README.md; do echo "$(dirname $f | cut -c13-15) $(awk "$S" $f | grep -c '^- \*\*2026-11-14')"; done | tr '\n' ' '   # 013 3 014 5 015 2 016 3 017 2 018 2 019 1 020 1 021 1; on main, each 0
for f in experiments/01[3-9]-*/README.md experiments/02[01]-*/README.md; do awk "$S" $f | awk 'length > 100'; done | wc -l   # 0
awk "$S" experiments/014-s3-two-machines/README.md | tr -s ' \n' '  ' | grep -c 'with .wsl --terminate., in place of the .wsl --shutdown.'   # 1; on main, 0
awk "$S" experiments/014-s3-two-machines/README.md | tr -s ' \n' '  ' | grep -c 'after the declaration, which must be refused'   # 1; on main, 0
awk "$S" experiments/016-s5-control-contention/README.md | tr -s ' \n' '  ' | grep -c 'it stores no control.s values, makes nothing ready'   # 1; on main, 0
awk "$S" experiments/017-s6-revoke-in-flight/README.md | tr -s ' \n' '  ' | grep -c 'which S6-T4 above does not have'   # 1; on main, 0
awk "$S" experiments/018-s7-outside-package/README.md | tr -s ' \n' '  ' | grep -c 'the coordinator, not the outside author, sets them up'   # 1; on main, 0
grep -c '^| 3[0-8] | \*\*The coordinator.s lines in S[0-9]*.s amendments since acceptance\*\*' docs/proposals/first-build/to-review-with-the-owner.md   # 9; on main, 0
```

`check_docs.py` checks that each new link and anchor resolves, the review list's links to the
READMEs' new sections among them. The admin then has a fresh, read-only Opus reader read the
diff before it merges ([ruling 79](../proposals/ruling-79/README.md)). Do not stage anything.
Report each choice you make as your own, and every check's last line.
