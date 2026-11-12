# Work order 077 (S75a): failed is reconciled only once fenced, and the safety catalogue's findings

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` each case you add or change, with the promises it
tests and whether it passes. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file, fixture or log: use
`head`, `tail`, `grep`, `sed -n` or a count. Run one cargo command at a time. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a credential.

## What and why

[Ruling 75](../proposals/ruling-75/README.md), point A1, answers finding MC2-001 of work order 105.
Model checking found that OPS-014 and OPS-024 together break OPS-018 on a plain implementation, in
seven actions. In short:

1. the engine marks an attempt unknown while its dispatch is still on its way;
2. an actor reads no effect and reconciles the invocation as failed;
3. the caller's repeat is dispatched as a new attempt;
4. both dispatches land, and one keyed effect is applied twice.

The owner's decision, in the record's words:

> an actor may record that an attempt's effect did not occur only once the attempt is fenced.
> Either no copy of its dispatch remains in flight, or the implementation confirms that the
> attempt can no longer act. This is the fenced control of experiment 012.

[IN_FORCE.md](../../IN_FORCE.md) holds it as *spec pending* in topic 7, at line 151, which begins
"- *Spec pending*: an actor may record that an unknown invocation's effect did not occur
(OPS-014) only once its attempt is fenced".
[Experiment 012](../../experiments/012-model-checking/README.md) holds the evidence:

- its dated results of 2026-11-11 (lines 322 to 344), in which the Fenced rows (lines 338 to 344)
  hold all five safety properties in every fault part and reach every witness. The table at lines
  215 to 244 is the verdict registered before the run, not a result;
- the control's definition (lines 246 to 251). Its model assumes that "implementation application
  is atomic" (lines 249 and 250), and its fence certifies that no effect occurred only where a
  dispatch was lost (lines 209 and 210). Step 1 says what this leaves out;
- the seven-action trace (lines 420 to 430), and "What follows, and limits" (lines 531 to 568).

Order 105's Record (`docs/work-orders/105-mc2-operation-identity.md`) sums it up.

Steps of a case are zero-based in this order, as in a JSON Pointer `/steps/N`.
`tools/vector_delta.py` and the runner's errors print the same step as N + 1.

Today both the text and the engine allow the trace:

- OPS-014's `then` (`spec/conventions/operations.yaml` line 294) records failed on the actor's
  word, and its observable says that "an effect may land after the actor has recorded that it did
  not occur" (the sentence at lines 319 to 324).
- In the case `requests-reconcile-false`, `/steps/3` ends the write's attempt with the completion
  `missing`, `/steps/5` records failed, and `/steps/7` dispatches attempt 2 under OPS-024. The
  shell sends `missing` when its call times out or fails at the transport
  (`engine/crates/shell/src/runner.rs` lines 1031 to 1045), that is, while the dispatch may still
  be on its way.

This order writes the fence into the specification, builds it in the engine's core so that the
vectors pass, and adds vectors that show it. It also settles the four findings that order 100
reported from the safety catalogue (its Record, `docs/work-orders/100-safety-catalogue.md` lines
213 to 222):

- **SC-F1**: `conformance/s1-safety-cases.md` is stale. Its bootstrap trace expects two root
  Conduits, not three; its plain-ends trace expects failed for -32602, where IFC-010 now gives
  unknown for an effectful operation; and its closing paragraphs call settled questions open.
- **SC-F2**: the row for plain in `spec/definitions.md` gives failed for invalid parameters
  without limiting it to reads, which could license an unsafe retry.
- **SC-F3**: OPS-009, OPS-034 and IFC-019 are ambiguous on a filtered repeat of a plain call that
  succeeded with content, of an operation that declares no outputs.
- **SC-F4**: other summaries in `spec/definitions.md` are stale: the diagnostic, the target, the
  manifest's count of document kinds, the core package's root Conduits and the sign-in
  elicitation.

**Who decided what.** The fence is the owner's (75 A1). Where a step below marks a decision
*coordinator's*, it is the admin's own reading or settlement of a finding, made under IN_FORCE.md
topic 15, "Who decides" ("Two or three coordinators settle the rest", 48 and 71 point 1), and it
stays open to the owner to strike. Apply it as written, and report it under `decisions` as
coordinator's. The admin logs it; you write no record.

**Orders in flight.** Line numbers in this order are those of `main` at 9729f21. Three orders
running now share files with this one:

- **Order 041b** builds reconciling an Agent through its Fleet, and REV-010. It rewrites the
  reconciliation's preflight in `engine/crates/core/src/requests.rs`: it replaces the error
  `control reconciliation requires REV-010 lifecycle work`, and its worker's output adds a
  conflict under REV-014 just after OPS-030's, where step 8 puts the fence's check. It also adds
  about 14 cases to `conformance/manifest.yaml` and to `s1_coverage`, and edits
  `engine/CONFORMANCE.md`. This order starts only after 041b has merged. Before you change
  anything, run `grep -n 'requires REV-010 lifecycle work' engine/crates/core/src/requests.rs`.
  If it prints a line, 041b is not in your tree: stop, and report that.
- **Order 038b** writes enrollment into `spec/`. It edits other rows of `spec/definitions.md`,
  `spec/core/README.md` and the head comment and other contracts of `spec/core/operations.yaml`,
  ACC-032 in `spec/conventions/access.yaml`, IFC-049 in `spec/interface/implementation.yaml`, and
  lines of IN_FORCE.md in topics 9 and 10. Keep each of your edits in those files to the lines it
  needs, so that either order rebases easily.
- **Order 074** adds a row, "DL10: MCP session idle limit", to `engine/CONFORMANCE.md`, and its
  scope is `engine/crates/shell/`, which this order does not change. This order names the rows of
  `engine/CONFORMANCE.md` by their first cell.

Where a line has moved in your tree, find it by the words quoted. Lines under `engine/` and in
`conformance/manifest.yaml` will have moved, since 041b merges first; if 038b has merged too, so
will lines of the files it edits.

**Size: M-L.** The specification's text, a core change with its tests, and a few cases.

Read first:

- ruling 75's record, point A1; in IN_FORCE.md, topic 7 (lines 132 to 151), topic 13 (lines 254
  to 283), the sign-in lines of topic 9 that hold ruling 75's points C1 to C7 (lines 193 to 196),
  topic 15's "Who decides" and its *Coordinator's* line (lines 303 to 308), and line 22: "An
  unknown outcome stays unknown, and nothing runs a background reconciler or wakes on its own";
- experiment 012's README, its MC2 sections (lines 165 to 261 and 303 to 568);
- the promises:
  - in `spec/conventions/operations.yaml`: OPS-003, OPS-007 to OPS-014, OPS-018, OPS-021 to
    OPS-025, OPS-027 and OPS-029 to OPS-034;
  - IMP-007 (`spec/conventions/implicit.yaml`), LIF-028 (`spec/conventions/lifecycle.yaml` line
    554), REV-010 (`spec/conventions/revisions.yaml` line 126), and ACC-008, ACC-015, ACC-017 and
    ACC-055 (`spec/conventions/access.yaml`);
  - IFC-010, IFC-016, IFC-019, IFC-025, IFC-046 and IFC-050
    (`spec/interface/implementation.yaml`), PRV-009 and PRV-025 (`spec/promises/providers.yaml`),
    PLC-017 (`spec/promises/placement.yaml` line 211), CON-022, PKG-020, and DL3 and DL10
    (`spec/implementation-defined.yaml`);
- `reconcile.v1` in `spec/core/operations.yaml` (its description at lines 464 to 491 and its
  `effect` at lines 509 to 514), and its summary in `spec/core/README.md`, the item that begins
  "**Reconciling.**" (lines 176 to 204);
- `spec/definitions.md`: "How a request ends" (lines 225 to 258), and the rows that steps 4 to 6
  name;
- [CONTRIBUTING.md](../../CONTRIBUTING.md), "Writing promises": a rewrite keeps everything that
  the change does not name, lines stay under 97 characters, and `spec/` carries no change
  narrative;
- `docs/implementation/SAFETY_CATALOGUE.md`: its operations table (lines 465 to 500), its
  paragraph on filtered replays (lines 782 to 785) and its note on the stale source (lines 787 to
  793);
- `conformance/s1-safety-cases.md`, all 79 lines, and `conformance/README.md`, "Assertions" and
  "Reviewing vector changes";
- the cases `requests-reconcile-false`, `requests-reconcile-true`, `requests-restart-interrupted`,
  `requests-cancellation` and `requests-cancel-confirmed`, under
  `conformance/conventions/requests/`;
- in the core, `engine/crates/core/src/requests.rs`, found by these words (lines on 9729f21):
  - the repeat decision in `invoke_inner`, which sets `redispatch` and returns conflict with
    `"OPS-034"` (lines 791 to 830), and the invocation's `"needs_result"` (line 1056);
  - the reconciliation's preflight, from the comment
    `// Validate reconciliation's state after authority/schema, before allocating or dispatching.`
    (lines 936 to 966), as 041b leaves it;
  - the recording of a reconciliation, `if let Some(reconciled) = reconciled`, with its decision
    `"reconcile-checked-effect"` (lines 1098 to 1123);
  - `pub(crate) fn complete(` (from line 1240), with its late branch, the decision
    `"ignore-late-completion"` (lines 1255 to 1265); `pub(crate) fn cancel(` (from line 1410); and
    `pub(crate) fn recover(` (from line 1486);
- `Input::BuiltinRunStopped` in `engine/crates/core/src/lib.rs` (lines 363 to 382);
- in `engine/CONFORMANCE.md`, the rows whose first cells are "DL2: attempts and late results",
  "DL3: waiting, implemented portion", "DL10: process and call deadlines", "DL16: run state and
  restart" and "DL16: executor coverage".

## What to do

1. **The fence, in OPS-014** (75 A1). Apply this decision as written; do not reopen it.
   - Recording that the effect occurred makes the outcome succeeded, as today, with no condition.
     Recording that it did not occur makes the outcome failed only once the invocation is fenced.
     Before then the request is not dispatched (step 3).
   - Define fenced by the ruling's two branches: no copy of the dispatch remains in flight, or the
     implementation confirms that the attempt can no longer act. Step 2 says what establishes
     each.
   - **Every attempt** (coordinator's reading of "the attempt"). An invocation may have several
     attempts (OPS-012, OPS-024), and it is fenced only when each of them is. A deduplicating new
     attempt under OPS-012 can be dispatched while an earlier attempt is still on its way, and
     that earlier one can land after failed is recorded.
   - The ruling makes no exception for a deduplicating implementation: write none.
   - Rewrite the sentence of OPS-014's observable that runs from line 319, "What an actor checks
     of an invocation dispatched to an instance on a machine that is cut off", to line 324, "did
     not occur." Line 318 ends the sentence before it: leave that sentence whole. Keep the
     sentence's facts: what the actor checks may be out of date until the machine is back and the
     run is inspected (PRV-009); the work may still run there (OPS-027); and a gate closes a
     server's stream only where the server answers with one (PRV-025). Its conclusion changes:
     the fence now keeps the actor from recording failed meanwhile.
   - **What the fence leaves out.** The fence speaks of the engine's dispatch and of what the
     implementation confirms. An implementation that has answered, or whose process has ended,
     may still have a request of its own on its way to another service, as a plain server that
     passes the call on to a remote API may. That effect can land after failed is recorded, and
     the repeat that OPS-024 then dispatches can apply it again. Experiment 012's model does not
     cover this, since it assumes that an implementation applies an effect atomically. Keep one
     sentence in OPS-014's observable that states this limit, in place of the clause that ends
     the sentence you rewrite. Write no rule to close it, which would go beyond 75 A1, and report
     it under `findings` for the owner.
   - Nothing is automatic. Once an invocation is fenced, the engine records nothing, wakes on
     nothing and reconciles nothing (IN_FORCE.md line 22, OPS-013): an actor still reconciles it.

2. **What fences an attempt.** In OPS-014's observable, write what fences an attempt after each
   way its outcome can become unknown, using only what the specification already has. Apply these
   readings, each coordinator's, within the limit that step 1 keeps:
   - **An answer fences.** A response that ends the attempt's call at the implementation (a
     result, an error or a JSON-RPC error) means that no copy of the call remains on its way. A
     response that arrives after the outcome became unknown fences the attempt too, and changes
     no outcome (OPS-013). A response that leaves the work running fences nothing: the creation of
     a task under IFC-016, whose answer is what the task ended with (IFC-010), and an
     `input_required` result. Nor does an error after which the call is sent once more under
     IFC-046: the call sent again is still on its way until it is answered.
   - **No answer fences nothing.** An attempt whose outcome became unknown because the engine
     stopped waiting for its result (OPS-007, DL3, DL10), because the connection it travelled on
     dropped (OPS-027), because it was cancelled with no confirmation (OPS-027), or because the
     engine restarted while it was in progress (IMP-007), stays unfenced until another event in
     this list fences it.
   - **The run's end fences.** An attempt sent to a run is fenced once that run has stopped: the
     engine's own runner has seen its process end, or the runtime provider reports it `stopped`
     (PRV-009) in a report made after the attempt was dispatched. A report about a machine that is
     cut off fences nothing until the machine is back and the run is inspected (PRV-025, OPS-027).
     Declaring a node lost fences nothing: under PLC-017, a run left on a machine that comes back
     may still run. A server reached by `remote` has no run.
   - **The implementation's confirmation**, only by a means the specification already has. Name
     the means. For example, check whether the protocol's own text says that a task the
     implementation reports cancelled (IFC-016, IFC-010) can no longer act. Define no new message,
     key or operation (ruling 40). Where no means exists, say so in the text and report it as a
     finding.
   - **What the engine carries out itself.** An invocation or a part whose implementation is
     `engine` is fenced once it has ended. A setup, a retirement, an open or a close is fenced once
     every part it dispatched is fenced (LIF-028, OPS-022). A hook is reconciled as an invocation
     of its own (OPS-021).

   Then state the consequence plainly. An attempt that nothing in this list fences, such as one
   sent to a plain server reached by `remote` that never answered, is never fenced: its invocation
   can be recorded as having occurred, or it stays unknown (OPS-013).

3. **The refusal before the fence** (75 A1; the reason chosen is coordinator's). Extend OPS-030's
   `when` with a request that records that the effect did not occur, of an invocation whose
   outcome is unknown and that is not fenced. It is not dispatched, and reports conflict:
   - refused means only that a request is not permitted, and "held" is none of the seven ways a
     request ends (`spec/definitions.md` lines 227 to 237); holding the request until the fence
     would have the engine wake on its own (IN_FORCE.md line 22);
   - conflict is what OPS-030 already reports for an invocation still in progress.

   In OPS-030's observable: the invocation's state is unchanged, and no record of a reconciliation
   is added. The error diagnostic's `ruleId` is OPS-030. Its message says, as IFC-050 and ruling 52
   ask, that the invocation is not yet fenced, what would fence it (step 2), and that recording
   that the effect occurred needs no fence. OPS-023 still puts refused (OPS-014, ACC-055) and
   invalid (OPS-029, OPS-031) before it. The same request made again later is decided afresh
   (OPS-025).

4. **What relies on the fence** (75 A1). Make each of these agree with steps 1 to 3, citing
   OPS-014 and saying no more than the fence needs:
   - OPS-024: a failed outcome that a reconciliation recorded was recorded only once the
     invocation was fenced, so the new attempt cannot meet an earlier one still on its way;
   - OPS-018: its observable names the fence as what keeps the effect at most once across a
     reconciliation and a repeat, and points to OPS-014 for the limit that step 1 keeps;
   - `reconcile.v1` in `spec/core/operations.yaml`: its description, at lines 475 and 476 ("A
     request that names an invocation whose outcome is not unknown reports conflict") and lines
     480 and 481 ("Recording that the effect did not occur gives the outcome failed"), and its
     `effect` (lines 509 to 514, "Changes the invocation's outcome from unknown to succeeded or
     failed");
   - LIF-028's `then` (`spec/conventions/lifecycle.yaml` lines 563 and 564, "Recording that it
     did not occur gives the outcome failed"), REV-010's `then` and observable
     (`spec/conventions/revisions.yaml` lines 135 and 136, and 147 to 149, "A reconciliation that
     records that the effect did not occur"), and, in `spec/core/README.md`, lines 190 and 191
     ("Recording that the effect did not occur gives the outcome failed and changes nothing
     else") and lines 196 and 197 (the conflict under OPS-030);
   - in `spec/definitions.md`: the two rows for reconcile (lines 42 and 221), the row for attempt
     (line 214), a new row for fenced beside them in "Requests and operations" that points to
     OPS-014, and the conflict row of "How a request ends" (line 236), if its examples should name
     this case. The row at line 221 also names only a Pool or `management` as the target, where
     ACC-033 names the Fleet that owns an Agent too (`spec/core/README.md` line 182): make it
     agree, as a stale summary.

   Then search `spec/` for "did not occur", "reconcil" and "failed", and change any other text that
   has a reconciliation record failed with no fence. In your report, list every place you changed,
   and every place you checked and left.

5. **SC-F2 and SC-F4: stale summaries in `spec/definitions.md`.** A summary says what its promises
   say, and the promise governs. Check each row against the promises it cites, and write no new
   meaning:
   - plain (line 286): an error, -32602 among them, gives failed only for an operation whose
     contract states no `effect`, that is not a control and whose contract is not marked
     `startsModelWork: true`, and unknown for any other (IFC-010). Today "and for invalid
     parameters" reads as failed for any operation, which would let OPS-024 dispatch a write again;
   - diagnostic (line 104): "one that reports refused carries none" contradicts IFC-025 and
     IFC-050, under which a refused result carries ACC-056's diagnostic;
   - target (line 165): a target may also be a Fleet (OBJ-013);
   - manifest (line 384): "the ten kinds of document" are eleven (`spec/schemas/document.yaml`
     line 2);
   - core package (line 388): "the two Conduits of `root`" are three, to `management`, `access`
     and `system` (FST-002);
   - the sign-in elicitation: the finding names no row. Check the rows that describe it, at lines
     204 (the sign-in hooks), 270 (sign-in) and 274 (account check), against PND-010, PND-011,
     IFC-016 and IFC-049. Say which you changed and why, or that you found none stale. Write none
     of ruling 75's points C1 to C7, which stay *spec pending* (IN_FORCE.md lines 193 to 196).

   Check too the rows of "Core terms" (lines 11 to 53) that summarize the same things: plain
   (line 44), manifest (line 51) and core package (line 52).

6. **SC-F3: a filtered repeat of a plain call with content.** Today:
   - OPS-009's `when` takes a repeat to which a filter applies, of an operation that declares no
     outputs and that OPS-024 does not dispatch again (`spec/conventions/operations.yaml` lines
     188 to 190). Its `then` gives "its outputs and any plain content" (line 193). Its observable
     counts that content as outputs "for this promise and OPS-034" (line 195) and, where a filter
     applies, gives "only the outcome already recorded" (lines 212 to 214);
   - OPS-034's `when` covers only "an operation that declares outputs" (line 758);
   - under IFC-019 only an implementation applies a filter, so the engine cannot know that a
     filter permits the kept content now.

   Settle it with the reading that keeps OPS-018 and IFC-019, and report it under `own_choices` as
   your reading; the admin logs the settlement as coordinator's:
   - nothing is dispatched again, unless OPS-024 dispatches the repeat because the operation
     states no `effect`, is not a control and starts no model work, so no effect is applied twice
     (OPS-018);
   - the answer holds none of the content kept for the invocation, which no filter has judged now
     (IFC-019, ACC-008).

   Within that reading, the repeat either reports conflict under OPS-034 or is answered from its
   outcome alone under OPS-009. Choose by which keeps the result explicit (ruling 52). The admin
   expects conflict under OPS-034: OPS-009 already counts the content as outputs for OPS-034, a
   result of succeeded without that content reads as a call that returned nothing, and the core
   already answers so (in `requests.rs`, the conflict with `"OPS-034"` in the repeat decision,
   where a plain invocation counts as needing its result through `"needs_result"`). If you find a
   reason for the other answer, apply it, give the reason, and make the core agree.

   First establish where a filter can apply to such a repeat at all. CON-022 admits a Conduit's
   filter only where the implementation supports it, and a plain implementation supports none
   (PKG-020), so a filter reaches the repeat only as one that an onward request carries (ACC-017),
   which ACC-015 makes unavailable for a dispatch. Say which promise answers that repeat, in
   OPS-023's order, and write it there.

   Then make every text that states the answer agree with your reading, changing each only where
   the reading requires:
   - OPS-009, OPS-034 and ACC-008 (its observable, lines 152 to 157);
   - in `spec/definitions.md`, the repeat row (line 213) and the paragraph after the table of
     "How a request ends" (lines 253 to 257, "A repeat of an invocation that has not ended");
   - IFC-016's observable (`spec/interface/implementation.yaml` lines 401 to 407, "or, for an
     operation that declares no outputs, is answered with the outcome alone under OPS-009");
   - the head comment of `spec/core/operations.yaml` (lines 80 to 91; line 87, "and is answered
     from its outcome, where it declares none (OPS-009)");
   - `spec/core/README.md` lines 297 to 302 ("Where a filter applies to a repeat of an invocation
     that succeeded");
   - the catalogue's paragraph on filtered replays (`docs/implementation/SAFETY_CATALOGUE.md`
     lines 782 to 785, "success with an effect and no outputs returns its recorded outcome under
     OPS-009").

   Change IFC-019 and IFC-025 only if they must.

7. **SC-F1: `conformance/s1-safety-cases.md`.** It lists proposed traces and binds nothing.
   Correct it against the promises. Keep its title, its tables' columns and every trace's name,
   which the catalogue and the experiments cite:
   - `bootstrap` (line 45): root has three open declared Conduits, to `management`, `access` and
     `system` (FST-002), beside the five ready objects of FST-001;
   - `S1-plain-ends` (line 62): -32602 gives failed for an operation that states no `effect`, is
     not a control and starts no model work, and unknown for any other (IFC-010);
   - `S1-reconcile` (line 65): add the fence. Failed is recorded only once the invocation is
     fenced, and a reconciliation to failed before then reports conflict (OPS-014, OPS-030);
   - the closing paragraph (lines 72 to 79): IFC-010 settles the -32602 question. Check the
     sentence on IFC-046 against IFC-046, and the filtered replays against step 6's reading. Drop
     "No study result closes those known questions" where it no longer holds;
   - the last sentence before the remaining traces (lines 34 to 36, "The known root `$defs`
     collision"): IFC-033 now reserves and rejects the root `$defs` keys, so that collision is
     decided (the catalogue, lines 789 to 791).

8. **The core** (`engine/crates/core/`). Build the fence, so that the cases of step 9 pass. Each
   decision that the core emits for it cites OPS-014, or OPS-030 for the refusal.
   - Keep, for each attempt of an invocation, whether it is fenced. A dispatch starts an attempt
     unfenced.
   - A `complete` whose completion is a response of step 2's first kind fences the attempt it
     names, whether it ends the invocation or arrives late. Today a late one answers
     `{"ignored":true}` with the decision `"ignore-late-completion"`, citing OPS-013, OPS-018 and
     IMP-007: keep that reply and the outcome, record the fence in the state, and add OPS-014 to
     that decision.
   - A `complete` whose completion is `missing`, a `cancel` with no confirmation, and `recover`
     fence nothing.
   - `builtin-run-stopped` (`Input::BuiltinRunStopped` in `lib.rs`) fences each unfenced attempt
     that was dispatched to that object's built-in run before the stop was observed. No case input
     carries it, so test it in the core's own tests or in
     `engine/crates/conformance/tests/core_boundary.rs`. Add no runner input.
   - The reconciliation's preflight: after the check that reports OPS-030 for an outcome that is
     not unknown, a request with `occurred: false` whose invocation is not fenced reports conflict,
     with step 3's diagnostic. Where 041b's conflict under REV-014 applies to the same request
     too, say which rule your diagnostic names, and why.
   - Where the core cannot yet tell that an attempt is fenced, as for a runtime provider's report
     or a part of a setup, it treats the attempt as not fenced, so that it never records failed
     early. An invocation with no fence mark, as in a store written before this change, reads as
     not fenced. List each such case in your report, for a build order.
   - The shell is out of scope. Report under `findings` what it must do so that the fence can be
     reached in the running engine, checking each of these in `engine/crates/shell/src/runner.rs`:
     - it drops a response that comes after its call's timeout: `dispatch_later` removes the
       pending request once the timeout gives `missing` (lines 1046 to 1048);
     - it submits `builtin-run-stopped` only in `impl Drop for Runner` (from line 208; the
       submission at line 219), when the shell itself shuts down, and not when a run exits on its
       own, which the next `inspect-run` reports instead (the row "DL16: run state and restart" of
       `engine/CONFORMANCE.md`);
     - after the engine is killed and restarts, the runs that ended with it (the same row) are
       never reported stopped through `builtin-run-stopped`, and `recover` fences nothing;
     - where no run exists for the target, `dispatch_later` returns `Execution::Missing` (lines
       1019 and 1020), and a call to a server reached by `remote` is not served (the row "DL16:
       executor coverage"). Nothing was sent, yet the core sees only `missing` and the attempt
       stays unfenced.
   - In `engine/CONFORMANCE.md`, in the rows "DL2: attempts and late results", "DL3: waiting,
     implemented portion" and "DL16: executor coverage", say what the core now does with a late
     response and with a stopped built-in run, and what the shell does not yet do.

9. **The vectors.** Write each expectation by hand from the promises, never by copying the
   engine's output, and declare every change.
   - **New cases**, under `conformance/conventions/requests/`, built from the initial state, files
     and first step of `requests-reconcile-false` as your tree has it. A case may name that case's
     fixtures by relative path. Between them, the new cases show:
     1. a keyed plain write whose attempt ends `missing`, after which root's reconciliation with
        `occurred: false` reports conflict with OPS-030's error diagnostic, dispatches nothing,
        records no reconciliation and leaves the outcome unknown;
     2. a late `complete` of that attempt, which changes no outcome (OPS-013) and dispatches
        nothing;
     3. the same reconciliation, made again, after which the outcome is failed, and the caller's
        repeat is dispatched as attempt 2 (OPS-024);
     4. the same conflict after a `cancel` with no confirmation (OPS-027), and after
        `restore-state` (IMP-007).

     Give the first case the ID `requests-reconcile-fenced`, in `reconcile-fenced/`.
     `requests-reconcile-true` already shows that recording that the effect occurred needs no
     fence: it stays unchanged.
   - **Changed cases.** Change no other case unless the new text requires it, and then only the
     steps it requires:
     - `requests-reconcile-false`: its `/steps/5` records failed after `/steps/3`'s `missing`,
       which the fence now refuses. Keep its purpose, "S1-reconcile: evidence establishes failed",
       and change as few steps as you can. For example, `/steps/3` can end the attempt with an
       answer that IFC-010 reads as unknown for a plain write, such as a result whose `isError` is
       true, which fences it. Say what you chose.
       `docs/work-orders/036b-staged/vector-impact-038a.md` line 62 lists this case's steps 2, 3
       and 6 (zero-based) for later work on diagnostics. If you change `/steps/3`, report that
       line as stale; the file is out of scope.
     - `requests-restart-interrupted` and `requests-cancellation`: their late `complete`
       (`/steps/4` and `/steps/3`) now fences the attempt, so their expectations should not
       change. Report any that does.
   - **The coverage index.** Add each new case to `conformance/manifest.yaml`, or to its pieces if
     order 071 has landed in your tree, in the form of `requests-reconcile-false`'s entry, with
     `safety_rows` naming `S1-reconcile`. Add it to the `s1_coverage` list of each promise it
     tests. In `spec/README.md` line 7, set the count of portable vectors to the number of entries
     in the manifest's `cases` in your finished tree (413 on 9729f21, before 041b's cases).
   - **The declaration.** Write `docs/work-orders/077-declared.yaml`, in the form of
     `docs/work-orders/041a-declared.yaml`. Give one shape for each intended change, none broader
     than its change: an `added` shape for each new case, and, for each changed case, shapes that
     name its exact paths, of these kinds:
     - `input`, for a change under a step's `input` or to the case's `files` or `initial`;
     - `flip`, for a change to a step's `expect/accepted`, `expect/stateUnchanged`, or
       `expect/reply`'s `reason`, `outcome`, `accepted` or `tool`, and `expectation` for any other
       change under a step's `expect`;
     - `metadata`, for a change to a case's `promises`, `title` or other top-level field;
     - `fixture`, for a change to a file that a case names. It is reported once for every existing
       case that names the file, so give a new case its own `requests.json` rather than adding
       entries to `reconcile-false/requests.json`, unless that case needs them.

     Run `python3 tools/vector_delta.py HEAD --declared docs/work-orders/077-declared.yaml` until
     it reports no undeclared change.

10. **IN_FORCE.md and the catalogue.**
    - This order writes in full the *spec pending* line of topic 7 at line 151, which begins
      "- *Spec pending*: an actor may record that an unknown invocation's effect did not occur
      (OPS-014) only once its attempt is fenced". Make it a pointer to the promises that now hold
      it, in that file's form for a line headed **In spec/**, citing "75 A1; experiment 012,
      MC2-001". You may instead fold those IDs and that citation into the line that begins
      "  - Unknown outcomes and reconcile: OPS-012 to OPS-014, OPS-031" (line 137), and remove
      line 151. Line 136, which begins "  - Idempotency keys and repeats:", is not that line.
      Change no other line.
    - In `docs/implementation/SAFETY_CATALOGUE.md`, the note on the stale source (lines 787 to
      793, "The source has stale expectations to reconcile") no longer holds once step 7 is done:
      rewrite or remove it. Change the paragraph on filtered replays only as step 6 says. Where a
      promise you change no longer holds a row's quoted text, change that row's quote and keep its
      kind. Check the rows of OPS-009, OPS-014, OPS-018, OPS-024, OPS-030 and OPS-034 above all.

11. **Before you finish, check**:
    - every request that is not dispatched names exactly one of refused, invalid, conflict and
      unavailable, in OPS-023's order (CONTRIBUTING.md, "Writing promises", rule 5);
    - each way an outcome becomes unknown has its fence, or the text says that nothing fences it;
    - each promise you rewrote keeps everything that the change does not name, and every ID it
      cites exists;
    - no text in `spec/` still lets failed be recorded before the fence, and none tells a caller
      that a repeat after a reconciliation can apply the effect again, beyond the limit that step
      1 keeps;
    - what the fence touches outside the scope, for your report only: S3's V2
      (`experiments/014-s3-two-machines/README.md` line 166), S4's T6 and V5
      (`experiments/015-s4-partial-setup/README.md` lines 117 and 163), and experiment 012, whose
      model "as the promises state it" no longer matches the text.

## Scope

Change:

- in `spec/`:
  - `spec/conventions/operations.yaml`: OPS-009, OPS-014, OPS-018, OPS-024, OPS-030 and OPS-034,
    and any other promise that step 4's search finds;
  - `spec/conventions/lifecycle.yaml` (LIF-028) and `spec/conventions/revisions.yaml` (REV-010);
  - `spec/conventions/access.yaml` (ACC-008) and `spec/interface/implementation.yaml` (IFC-016,
    IFC-019, IFC-025), for step 6 only, and only where its reading requires;
  - `spec/core/operations.yaml`: `reconcile.v1`'s description and `effect`, and, only where step
    6's reading requires, the head comment's sentences on repeats (lines 80 to 91);
  - `spec/core/README.md`: the item "**Reconciling.**", and, only where step 6's reading
    requires, the sentences on filtered repeats (lines 297 to 302);
  - `spec/definitions.md`;
  - `spec/README.md`: the count of portable vectors at line 7 only;
  - elsewhere in `spec/`, only text that step 4's search finds letting a reconciliation record
    failed with no fence, each place listed in your report;
- `IN_FORCE.md`: the line that step 10 names, and, if you fold it there, the line that begins
  "  - Unknown outcomes and reconcile:";
- `conformance/s1-safety-cases.md`;
- `conformance/conventions/requests/`: the new cases and their fixtures, and `reconcile-false/`
  only as step 9 says;
- `conformance/manifest.yaml`, or its pieces, for the new cases;
- `docs/work-orders/077-declared.yaml`, new;
- `engine/crates/core/` and its tests, `engine/crates/conformance/tests/core_boundary.rs` for
  step 8's test, and, in `engine/CONFORMANCE.md`, the rows "DL2: attempts and late results",
  "DL3: waiting, implemented portion" and "DL16: executor coverage";
- `docs/implementation/SAFETY_CATALOGUE.md`: the note on the stale source, the paragraph on
  filtered replays as step 6 says, and the quotes that step 10 names.

Do not change:

- `engine/crates/shell/`, `engine/crates/loader/`, `engine/crates/conformance/src/` or
  `conformance/case.schema.json`: this order adds no runner input;
- `spec/names.yaml`, `tools/` or `packages/`;
- `experiments/`: experiment 012's dated results stand, and the pre-registrations of S2 to S10
  are for your report only;
- any other line of IN_FORCE.md, or anything it still marks *spec pending*, ruling 75's points B
  and C among it;
- a record: the ledger, the rulings' records, `docs/AUDIT.md`, `ROADMAP.md`, `docs/proposals/`,
  and the files of other work orders, `docs/work-orders/036b-staged/` among them.

Report anything else that needs changing.

## Done when

Every item of steps 1 to 11 is written, or its absence is reported with the reason.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/check_private_paths.py`, if your tree has it (order 068 adds it)
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py HEAD --declared docs/work-orders/077-declared.yaml`, with that
  interpreter. Its last line reads `Vector delta: N changes; F flips; 0 undeclared; 0 errors.`, and
  it exits 0 where F is 0. A declared flip still exits 1: list each flip with its path, its before
  and after, and the promise that requires it.
- `git diff --check`

Run these with `bash`. Each prints nothing. The first shows that no line you wrote in a promise
file has 97 characters or more; the second, that every promise ID your changes to `spec/` cite
exists; the last two cover the files that are not yet tracked, which `git diff --check` does not
see. (The brackets keep this order free of the paths it looks for.)

```bash
git diff -U0 -- spec/conventions spec/interface spec/promises | grep -E '^\+[^+]' | awk 'length > 97'
for id in $(git diff -U0 -- spec | grep -E '^\+[^+]' | grep -oE '\b[A-Z]{3,5}-[0-9]{3}\b' | grep -vx 'SHA-256' | sort -u); do grep -rqE "^- id: $id$" spec || echo "$id"; done
grep -rn -E '[[:space:]]+$' conformance/conventions/requests docs/work-orders/077-declared.yaml
grep -rn -E '/[U]sers/|/[h]ome/|/private/[t]mp/|/tmp/[c]laude|\.codex/' conformance/conventions/requests docs/work-orders/077-declared.yaml
```

`git status --porcelain` lists only paths in the scope; report any other line, and why.

In `engine/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`. The tests read `spec/`, so run it once more after your last change to
  `spec/`.

Report the totals passed, failed and ignored of each full run, and its wall time, as numbers only.
If a shell test fails, rerun it alone. Its timing-sensitive tests can fail under load (finding
4-18): rerun those with `-- --test-threads=2`. Tests that bind loopback ports may fail in your
sandbox: say so, name each, and the admin reruns them.

Report:

- in `decisions`, each promise and definition row you changed, with the ruling or finding behind
  it: 75 A1, or SC-F1 to SC-F4 with the promise that governs; mark each coordinator's reading of
  steps 1 to 3 as coordinator's;
- each way an outcome becomes unknown, with what fences it (step 2), and each case that the core
  treats as not fenced for now (step 8);
- under `own_choices`: step 6's reading, with where a filter can reach such a repeat; what you
  chose for `requests-reconcile-false`; the rule your diagnostic names where REV-014 applies too;
  and every other choice you made yourself;
- under `findings`: the limit that step 1 keeps, for the owner; what the shell must do (step 8);
  any fence that the text needs and no means supports; the stale line of
  `vector-impact-038a.md`, if any; what step 11 found outside the scope; and anything else that
  needs changing;
- in `vectors`, each case added or changed;
- each check's last line.

Do not stage anything.

## Record

Run on 2026-11-11 and 12. The first worker, on the Windows computer, ran about 5.5 hours and was
stopped when the owner paused that computer; its partial work was checkpointed, rebased onto main
and continued on the Mac for three more rounds, with the owner's acceptance of the fence's two
limits (ruling 75's later answer of 2026-11-12: the fence covers Conduit's own dispatch, not a
server's onward call; a remote call that never answers stays unknown and the way on is a new
request). Round 3's verifier found no fault in scope but one out-of-scope vector from order 041b,
fleets/e3b/reconcile-control, that reconciled an attempt to failed without a fence. The admin
decided to keep that vector's purpose (who may reconcile) by establishing the fence first, and
extended the scope to it; round 4's worker found the repair also needed the vector's companion
requests.json, kept to the stated scope, and prepared a patch that passed every vector in a copy.
The admin applied it as an admin-side fix (ruling 71): the attempt is fenced by an observed late
answer before the original reconcile. On the rebased tree all 446 vectors pass, the vector delta
shows 123 declared changes and none undeclared, and the repository checks pass.
