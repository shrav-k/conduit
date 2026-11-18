# Work order 046c (E5c): the outcomes the engine decides name their promise

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. List in `vectors` every case you add or change, with the promises
it tests and whether it passes. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it. Use it for nothing else, and reach no other
machine. Run one cargo command at a time. Never print a whole large file, fixture or log: use
`head`, `tail`, `grep`, `sed -n`, `wc` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

**Where it sits.** Batch 1's core chain is 046a, 046c, 047, 048a-2, 048b-join, 050 and 053. Batch
1's plan has one order 046, "refusals and explicitness"
([the plan](../proposals/spec-batch-1/README.md), its entry 046 under "The work orders"), which
[order 117](117-plan-s2-s3.md) split into 046a and 046b. The full review's queue
([orders.md](../proposals/full-review/orders.md), line 100) gives 046a IFC-050's unbuilt half
([FR-353](../proposals/full-review/findings/P07-engine-build.md#fr-353); D37 of
[the decisions](../proposals/full-review/decisions.md#the-pipeline-and-verification)): an outcome
the engine decides as failed or unknown carries a diagnostic naming its cause, at the places
FR-353 names. [Order 046a](046a-build-refusal-tiers-and-diagnostics.md) built IFC-050 for every
reason and moved its decision 6, this half, whole to this order, which comes after 046a and
before 047 (see "Who decided what").

**What it builds.** One behavior: an outcome that the engine decides as failed or unknown says
which promise decided it and why, in its reply and in every repeat answered from it (IFC-050:
"An engine-decided failed or unknown outcome carries a diagnostic naming its cause"). A setup's
part held back under POOL-013 says so too. The negative assertions that pin such an outcome's old
form go ([FR-547](../proposals/full-review/findings/GL2-vectors.md#fr-547), F59, the half that
046a left), and one portable case asserts the new form.

**What it leaves, and to whom.**

- **OPS-012's unknown**, where a new attempt is answered failed while an earlier attempt has no
  answer or was answered succeeded. It is the engine's too, though no implementation stated it.
  The build records failed there today, and the cases `requests-repeat-failed-earlier-succeeded`
  and `requests-repeat-failed-earlier-unanswered` are expected failures until the full review's
  order 22 builds it, with its diagnostic under IFC-050. This order adds no row for it.
- **The https fetch** that answers invalid citing PKG-023 (D37's second sentence): order 046d,
  as order 046a says.
- **F2's build and vectors, and F3's other vectors**: an order of the admin's, not yet numbered,
  as order 046a says.

**Who decided what.** The split is the coordinator's: 046a's decision 6 is a behavior of its own
(FR-264: an order holds one behavior), so it moved whole to this order, which comes after 046a
and before 047. The rule each site names is the coordinator's too, under rulings 46 point 4 and
52, each open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)), and each
stated in decision 1's table with the reason the specification gives. Among them:

- **OPS-022 names every setup or retirement that ends unsucceeded, cancelled or not; OPS-027
  names only what a cancel itself ends.** OPS-022's `when` is the end of a `set-up.v1` or
  `retire.v1` invocation, and its `then` decides the outcome from the parts. OPS-027's `when` is
  an invocation cancelled while in progress, and its observable has the engine pass the
  cancellation on to a setup's or a retirement's parts, and then gives the outcome away: "The
  outcome of a setup or a retirement is then OPS-022's, and that of `retire-with-resources.v1`
  LIF-021's." IFC-050 asks for "the deciding promise's ID", so OPS-022 names every
  `lifecycle::settle` and `retirement::settle` that ends unsucceeded, and LIF-021 every end of a
  cascade, cancelled or not. Where the invocation was cancelled, the cause's words say that it was
  cancelled and name the first part not marked optional that did not succeed, so the cancel stays
  explicit (ruling 52; decision 1, "The causes"). OPS-027 names the invocations it decides
  itself: a hook child that `lifecycle::cancel` or `retirement::cancel` ends, and what
  `requests::cancel` ends unknown. An earlier draft named OPS-027 wherever a cancel ended a setup
  or a retirement; that departs from OPS-027's own sentence, and where a coordinator's choice
  and the specification differ, the specification governs
  ([ruling 67](../proposals/ruling-67/README.md)).
- **IMP-007 for what `requests::recover` ends**: IMP-007's observable says "An invocation that
  was in progress across a restart has the outcome unknown".
- **LIF-021 for a cascade's outcome, cancelled or not**: LIF-021's observable says "The cascade
  outcome is failed, with diagnostics naming every remaining member", and that once every
  member's shutdown is confirmed the request "takes that invocation's outcome", the container's
  retirement. OPS-027's observable gives a cancelled `retire-with-resources.v1` the outcome
  LIF-021's, and LIF-021's says "Cancellation under OPS-027 passes to retirements in progress and
  dispatches no further retirement".
- **POOL-013 for a held-back part**: POOL-013's `then` has an entry's part "begun only once the
  parts for the entries it refers to have succeeded", so a part held back for want of them is
  that rule's.
- IFC-036 for the shell's own errors is order 046a's (its decision 4), since those are reasons.

The coordinator confirms three of these choices, each open to the owner's strike:

- F3's other vectors go with F2's to the admin's order, not yet numbered ("What it leaves").
- A cascade cancelled with members remaining reports the cancel first, then one LIF-021
  diagnostic for each remaining member (decision 1, "The cascade"). The cancel's diagnostic
  names LIF-021, not OPS-027, by the first bullet above.
- At the two settle sites, the message that says the invocation was cancelled also names the
  first part not marked optional that did not succeed (decision 1, "The causes"). It names
  OPS-022, not OPS-027, by the first bullet above.

The rest of the decisions below are the coordinator's as well, the wording of each cause among
them. Apply them as written, and report them under `decisions` as the coordinator's.

**When this order starts.** Order 046a must be merged to `main` first: it brings `explain.rs`,
`explain::first_text`, the runner's checks, the portable-case rule and
`conformance/conventions/explain/`. If it is not merged, stop and report that, changing nothing.
Orders 048a-1 and 050 are written beside this one; whichever lands second keeps both changes and
writes `engine/STANDING.md` again with the tool. This order does not depend on S0b.

**Read first:**

1. In `spec/`: IFC-050, IFC-025, IFC-008, IFC-009, IFC-010, IFC-046 and IFC-047
   (`spec/interface/implementation.yaml`), OPS-007, OPS-012, OPS-022 and OPS-027
   (`spec/conventions/operations.yaml`), IMP-007 (`spec/conventions/implicit.yaml`), LIF-021
   (`spec/conventions/lifecycle.yaml`), POOL-013 (`spec/promises/pools.yaml`), and
   `diagnostic.v1` (`spec/core/results.yaml`).
2. FR-353 and FR-547 with their verifiers' corrections, which govern, and D37 and F59.
3. Order 046a, decisions 3, 4, 7, 8, 10 and 11, as merged, and `engine/crates/core/src/explain.rs`.
4. Under `engine/crates/core/src/`:
   - `requests.rs`: `envelope` (line 112), the repeat answered from an ended outcome (about lines
     1019 to 1032), `finish` (line 1445), `complete` (line 1507, its mapping from line 1562),
     `cancel` (line 1697, its `finish` at line 1749) and `recover` (line 1788, its `finish` at
     line 1800);
   - `lifecycle.rs`: `settle` (line 350, its `finish` at line 390), a part held back (line 605),
     and `cancel` (line 1126: it sets `cancelled` at line 1136, ends a hook child at line 1160,
     cancels a child setup by calling itself, and ends with `settle` at line 1194);
   - `retirement.rs`: `settle` (line 314, its `finish` at line 355) and `cancel` (line 448: it
     sets `cancelled` at line 454, ends a child at line 469, and calls `stop`, which reaches
     `settle` in this transition or a later one);
   - `cascade.rs`: the three calls of `requests::finish` (lines 252, 274 and 294), the LIF-021
     diagnostics written after the second (lines 275 to 279), and `cancel` (line 314, which sets
     `cancelled` at line 320).
5. The cases `s1-whole` (`conformance/conventions/s1/whole/`) and `explain-agent-results`
   (`conformance/conventions/explain/agent-results/`).

Line numbers are those of `main` at `9af1c29f`, before order 046a. If a line has moved, find it
by its words. `META/outcome` is the `_meta` key `outcome` under the prefix that `spec/names.yaml`
sets, written as every case writes it.

**Size: M**: about 0.2 to 0.35k changed lines of Rust with tests, one new case, and one
mechanical edit of 25 cases.

## What to do

1. **Outcomes the engine decides** (FR-353; D37; IFC-050).
   - `requests::finish` takes one more argument, `cause: Option<Vec<Value>>`: the diagnostics of
     an outcome the engine decided, one or more. Where it is `Some`, `envelope` puts them in
     `structuredContent.diagnostics`, and `explain::first_text` (order 046a, decision 4) writes
     their messages into the first text block. Every caller that passes succeeded, and every
     outcome an implementation stated under IFC-009, passes `None`.
   - **A repeat answered from an ended outcome.** Where `cause` is `Some` and the outcome is not
     succeeded, `finish` keeps the list with the ended invocation, as the member `cause`. A
     repeat answered from a stored failed or unknown outcome (`requests.rs`, about lines 1019 to
     1032) gives that stored `cause` to `envelope`, so its reply carries the same diagnostics and
     first text block as the reply that ended the invocation (IFC-050). An invocation that holds
     no `cause`, because an implementation stated its outcome or because it ended before this
     order, is answered without one. Where `requests::complete` restores `previous` (about line
     1628, where an implementation answers a redispatch with a refused or unavailable reason whose
     restriction is one of the filters), it restores `previous`'s `cause` too, or removes `cause`
     where `previous` holds none. Report any check that the new member trips.
   - An outcome is the engine's where no implementation stated it. Its diagnostic is
     `{"ruleId": RULE, "level": "error", "message": {"text": "{operation} ended {outcome} under {RULE}: {cause}"}}`,
     where `operation` is the invocation's qualified operation, unless a row below gives the
     diagnostics another form. RULE at each call site, with the specification's reason:

     | Call site | Outcome | RULE | Why that rule |
     | --- | --- | --- | --- |
     | `complete`: -32021 that names Conduit's extension, of an implementation that carries Conduit's keys | failed | IFC-008 | its observable: the error says the implementation performed nothing, so the outcome is failed |
     | `complete`: -32021 that names an undeclared capability, of an implementation that carries Conduit's keys | failed | IFC-047 | it has such an implementation answer so only where it performed nothing |
     | `complete`: -32020, for an operation outside IFC-046's resend class (it states an effect, is a control or starts model work) | unknown | IFC-046 | its `then`: "any other invocation it ends unknown"; the engine identifies only -32020 as a version error (below) |
     | `complete`: a plain implementation's error, `isError` result, `input_required` or unrecognized `resultType`, or no result at all, where IFC-010 gives unknown | unknown | IFC-010 | its `then` gives each of these unknown |
     | `complete`: a result of an implementation that carries Conduit's keys with no outcome, both an outcome and a reason, or an `isError` that disagrees; no result at all from such an implementation; a protocol error of such an implementation, other than those above and IFC-046's resend; a declared output that fails validation | unknown | OPS-007 | its `when` names each of these |
     | `lifecycle::settle` (`finish` at line 390), where the outcome is not succeeded, whether or not the setup invocation's `cancelled` is true | failed or unknown | OPS-022 | it decides a setup's outcome from its parts, and OPS-027's observable gives it a cancelled setup's outcome |
     | `retirement::settle` (`finish` at line 355), where the outcome is not succeeded, whether or not the retirement invocation's `cancelled` is true, in whichever transition the settle runs | failed or unknown | OPS-022 | it decides a retirement's outcome from its parts, and OPS-027's observable gives it a cancelled retirement's outcome |
     | `lifecycle::cancel` (line 1160) and `retirement::cancel` (line 469): a hook child that the cancel ends | unknown | OPS-027 | its `then`: a cancelled invocation's outcome is unknown |
     | `requests::cancel` (line 1749), where it ends the invocation unknown | unknown | OPS-027 | the same |
     | `cascade.rs`, line 274, where a member remains or the cascade's `cancelled` is true | failed | LIF-021 | its observable: the cascade outcome is failed, with diagnostics naming every remaining member; OPS-027's observable gives a cancelled cascade's outcome to LIF-021 |
     | `cascade.rs`, lines 252 and 294, where the outcome is failed or unknown, whether or not the cascade's `cancelled` is true | the outcome given | LIF-021 | the cascade takes the container's retirement's outcome, or fails without it |
     | `requests::recover` (line 1800), an invocation in progress at the restart, cancelled or not | unknown | IMP-007 | its observable: an invocation in progress across a restart has the outcome unknown |

     So a setup or retirement that ends unsucceeded has one rule, OPS-022 at its settle, cancelled
     or not, and a cascade one rule, LIF-021 at each of its three sites, cancelled or not; where
     `cancelled` is true, the cause's words say so ("The causes", below). `lifecycle::cancel` and
     `retirement::cancel` pass no cause of the parent's themselves: the parent's outcome is
     finished at its settle, by the row above. A child setup that `lifecycle::cancel` cancels by
     calling itself has `cancelled` true too, so its own settle names OPS-022, with a cause that
     says it was cancelled.

     A plain implementation's error or `isError` result where IFC-010 gives failed, for an
     operation with no `effect`, not a control and not `startsModelWork`, is the
     implementation's own failure: it passes `None`, as FR-547's verifier reads IFC-050 for the
     cases `read-is-error`, `failed-read-repeat` and `read-capability-error`. A plain
     implementation's -32021 is such an error: IFC-047's observable binds a plain implementation
     to no promise, and IFC-010 reads its -32021 as any other error, so the IFC-008 and IFC-047
     rows never apply to it. A confirmed cancel that `requests::cancel` ends failed
     (`requests.rs`, lines 1733 to 1739) is an outcome the implementation's confirmation states
     under IFC-009, as FR-547's verifier reads `cancel-confirmed`: it passes `None`. The IFC-046
     row comes before the IFC-010 and OPS-007 rows: a plain or other implementation's -32020 for
     an operation outside IFC-046's resend class (it states an effect, is a control or starts
     model work) is IFC-046's. The engine identifies only -32020: `Completion::ProtocolError`
     carries a bare code, and IFC-046 and IFC-001 name no code for a version error, so a protocol
     error with any other code is read by IFC-010 or OPS-007, as now. Telling a version error
     apart, and the resend at another revision, is unbuilt: report it as a finding, and list it
     as unfinished in decision 7's section. An error answering the call sent once more under
     IFC-046 is read as IFC-010 and OPS-007 read any other (IFC-046's observable), so a plain
     read's second -32020 is IFC-010's failed and passes `None`, as
     `requests-read-parameter-headers` step 4 in `KEEP`.
   - **The cascade** (`cascade.rs`, line 274). Compute `remaining` before `requests::finish`, as
     now, and pass the whole list as `cause` in every case: where the cascade's `cancelled` is
     true, first one LIF-021 diagnostic in the form above, whose cause says that the cascade was
     cancelled and dispatched no further retirement; then one LIF-021 diagnostic for each
     remaining member, with the message the code writes now. A cascade cancelled with no member
     left holds that first diagnostic alone. The rewrite of
     `t.reply["structuredContent"]` and of the stored request reply after `finish` (lines 275 to
     279) goes, so that the ended invocation keeps the list and a repeat of that cascade
     retirement carries it.
   - **The causes.** The cause of OPS-022 at the two settle sites names the first part not
     marked optional that did not succeed, by its name and its outcome or reason; where the
     invocation's `cancelled` is true, it first says that the invocation was cancelled, so the
     message says both that it was cancelled and which part did not succeed. The cause of
     LIF-021 at cascade lines 252 and 294 likewise first says that the cascade was cancelled
     where its `cancelled` is true. Each other cause comes from a
     third table, `pub const OUTCOME_CAUSES: &[(&str, &str)]` in `conduit_core::explain`, keyed
     by rule, with one sentence each, in the form of order 046a's decision 3. `explain` reads it
     for an outcome; a rule it does not hold is an error of the build, never a silent default:
     `explain` returns `Err` naming the rule.

2. **A part held back** (`lifecycle.rs`, line 605, POOL-013). The part that reports unavailable
   because an entry it depends on did not succeed gets `diagnostics` in the setup's parts (the
   invocation's `setup.parts` and the object's `status.setup.parts`), holding one diagnostic with
   `ruleId` POOL-013 whose message names the entries it waited for. The part's default conflict
   keeps its CON-004 diagnostic. The setup's reply holds no parts (IFC-025): add none to it.

3. **FR-547: the negative assertions beside an outcome the engine decides go** (F59; FR-547's
   least change, the half that order 046a left). From the repository root, run this once, before
   decision 4 adds its case. It changes only the steps whose expected reply's `_meta` states the
   outcome failed or unknown, outside the ten that `KEEP` lists, and rewrites each case in the
   form every one of them has now, JSON with two-space indentation and a final newline. It reads
   the `_meta` prefix from `spec/names.yaml`:

   ```bash
   python3 - <<'EOF'
   import json, pathlib
   PREFIX = next(line.split(":", 1)[1].strip()
                 for line in pathlib.Path("spec/names.yaml").read_text().splitlines()
                 if line.startswith("metaPrefix:"))
   OUTCOMES = {"failed", "unknown"}
   # Steps whose outcome passes no cause (decision 1), (case, step from 1): they stay as they are.
   KEEP = {("requests-aware-failed", 3), ("requests-aware-unknown", 3),
           ("requests-repeat-failed-earlier-answered", 3),
           ("requests-repeat-failed-earlier-answered", 5),
           ("requests-failed-read-repeat", 3), ("requests-read-is-error", 3),
           ("requests-read-invalid-parameters", 3), ("requests-read-parameter-headers", 4),
           ("requests-read-capability-error", 3), ("requests-cancel-confirmed", 3)}
   for path in sorted(pathlib.Path("conformance").rglob("case.yaml")):
       case = json.loads(path.read_text())
       before = json.dumps(case, indent=2, ensure_ascii=False) + "\n"
       for number, step in enumerate(case["steps"], 1):
           expect = step["expect"]
           reply = expect.get("reply")
           if not isinstance(reply, dict) or (case["id"], number) in KEEP:
               continue
           meta = reply.get("_meta")
           meta = meta if isinstance(meta, dict) else {}
           if meta.get(PREFIX + "outcome") not in OUTCOMES:
               continue
           content = reply.get("content")
           if (isinstance(content, list) and content and isinstance(content[0], dict)
                   and content[0].get("text") in OUTCOMES):
               del content[0]["text"]
           absent = expect.get("absent")
           if isinstance(absent, list) and "/reply/structuredContent" in absent:
               absent.remove("/reply/structuredContent")
               if not absent:
                   del expect["absent"]
       after = json.dumps(case, indent=2, ensure_ascii=False) + "\n"
       if after != before:
           path.write_text(after)
   EOF
   ```

   **The steps the script keeps** (`KEEP`). Ten steps end with an outcome that decision 1 gives
   `None`, so their reply holds no diagnostic and their first text block is the outcome alone.
   The script changes nothing in them, and their bare word and their absent
   `/reply/structuredContent` stay, so that a build that adds a diagnostic there fails:
   - an outcome an implementation stated under IFC-009: `requests-aware-failed` step 3,
     `requests-aware-unknown` step 3 and `requests-repeat-failed-earlier-answered` steps 3 and 5;
   - a plain read's failure under IFC-010: `requests-failed-read-repeat` step 3,
     `requests-read-is-error` step 3, `requests-read-invalid-parameters` step 3,
     `requests-read-parameter-headers` step 4 and `requests-read-capability-error` step 3;
   - a confirmed cancel: `requests-cancel-confirmed` step 3.

   `requests-repeat-failed-earlier-succeeded` step 6 and `requests-repeat-failed-earlier-unanswered`
   step 5 are not kept: their unknown is OPS-012's, the engine's, and carries a diagnostic once
   order 22 builds it. Both cases stay expected failures.

   Each case keeps every outcome it asserts. Computed on a scratch copy of `9af1c29f` with order
   046a's decision 8 and its hand edit applied, the script changes 30 steps in 25 cases that pin
   the first text block to a bare `unknown`, and removes `/reply/structuredContent` from `absent`
   in 32 steps of the same 25 cases; no `absent` it touches ends empty. Every outcome it edits is
   unknown: each failed outcome among the steps that pin the old form is one of `KEEP`'s. Edit none of these cases by
   hand. If `main` has moved and the counts differ, report the counts you see.

4. **One portable case**, `explain-outcome-causes`, in
   `conformance/conventions/explain/outcome-causes/case.yaml`, written as order 046a's decision
   11 writes its cases (JSON with two-space indentation and a final newline; members `id`,
   `layers`, `promises`, `title`, `profile`, `initial`, `files` and `steps`, in that order;
   `layers` `["convention", "interface"]`; `profile` `first-build`; `initial`
   `{"recipe": "empty"}`), with promises IFC-050, IFC-010 and IFC-025 and title "An outcome the
   engine decides names its deciding rule". It follows order 046a's decision 10, and no later
   order adds it to `conformance/internal-cases.yaml`. `files` is `s1-whole`'s, each path
   rewritten relative to the new case's directory.

   Every step copies an existing step's input whole or with only its `completion` replaced, so
   no step is new: each keeps the evidence of the `s1-whole` step it copies, and the steps keep
   `s1-whole`'s order (its evidence times are not monotonic, and the case keeps them as they are).
   - Steps 1 to 13 build the state: exactly as steps 1 to 13 of `explain-agent-results` (order
     046a, decision 11), each `s1-whole`'s step of that number, cut as that decision says.
   - Step 14: `s1-whole`'s step 21 input and `capture` whole, agent-b asking
     `project.file.read.v1`, which captures `$call6`. Expect what `s1-whole`'s step 21 expects,
     cut as in steps 1 to 13.
   - Step 15: `s1-whole`'s step 22 input, with its `completion` replaced by
     `{"kind": "missing"}`: no result at all, which IFC-010 gives as unknown. Expect a reply:
     META/outcome `unknown` and META `invocation` `"$call6"`, `isError` true, `content`
     `[{"type": "text"}]`, and `structuredContent.diagnostics` `[{"ruleId": "IFC-010", "level":
     "error"}]`; and `external_intents` as `s1-whole`'s step 22 expects.
   - Step 16: `s1-whole`'s step 23 input and `capture` whole, agent-b asking
     `project.file.write.v1`, which captures `$call8`. Expect what `s1-whole`'s step 23 expects,
     cut as in steps 1 to 13.
   - Step 17: `s1-whole`'s step 24 input, with its `completion` replaced by `{"kind": "result",
     "result": {"isError": true, "content": [{"type": "text", "text": "write failed"}]}}`.
     `file.write.v1` states an effect, so IFC-010 gives unknown. Expect a reply: META/outcome
     `unknown` and META `invocation` `"$call8"`, `isError` true, `content` `[{"type": "text"},
     {"type": "text", "text": "write failed"}]`, and `structuredContent.diagnostics`
     `[{"ruleId": "IFC-010", "level": "error"}]`; and `external_intents` as `s1-whole`'s step 24
     expects.

   Index it in `conformance/conventions/explain/index.yaml` as order 046a indexed its cases: write
   its entry (`id`, `case` `conventions/explain/outcome-causes/case.yaml`, `promises` and
   `safety_rows: []`, in that order) in a `cases` list in `conformance/manifest.yaml`, then run,
   from `engine/`, `cargo run --release -p conduit-conformance --example pieces` with no flag. It
   adds the entry to the piece and leaves `manifest.yaml` byte for byte as it was. Change no
   `s1_coverage` row.

   If a step expects what the build does not give, check the step against this order and the
   specification first. Fix the step only where it is wrong against them; otherwise fix the
   build. Never write an expectation from the engine's output.

5. **The standing table.** Once the cases and the index are in place, run
   `python3 tools/promise_standing.py --write` from the repository root. It rewrites
   `engine/STANDING.md`. Never edit that file by hand.

6. **Unit tests**, in the core, in `explain.rs` or beside the code they test:
   - each row of decision 1's table; for the IFC-010 row, one test each for a plain
     implementation's error, `isError` result, `input_required`, unrecognized `resultType` and
     no result at all; for the OPS-007 row, one with no result at all from an implementation that
     carries Conduit's keys;
   - the cancel rows: a setup cancelled in its work phase and one cancelled before it, each
     settled with OPS-022 and its message saying that it was cancelled and naming the part; a
     setup that fails with no cancel, settled with OPS-022 and a message that names no cancel; a
     retirement cancelled while its stop is in progress, settled in a later transition, with
     OPS-022 and a message saying that it was cancelled; and a hook child that
     `lifecycle::cancel` ends, unknown, with OPS-027;
   - the cascade: cancelled with no member left, whose reply holds one LIF-021 diagnostic saying
     that it was cancelled; cancelled with a member left, that diagnostic first, then the
     member's LIF-021; cancelled in its container phase, ended at line 252 with LIF-021 and a
     message saying that it was cancelled; not cancelled with two members left, two LIF-021
     diagnostics; and a repeat of the last, answered from the ended invocation, which carries
     the same two;
   - the four that pass `None`: a plain read's error that IFC-010 gives as failed (a -32021
     among them), a plain read's second -32020 answering the call sent once more under IFC-046,
     a confirmed cancel that `requests::cancel` ends failed, and an aware implementation's
     result that states failed under IFC-009, whose reply holds no diagnostic and whose first
     text block is `failed` alone;
   - a held-back part's POOL-013 diagnostic;
   - a repeat answered from an engine-decided unknown outcome carries the stored cause, and a
     repeat answered from an outcome an implementation stated carries none;
   - an engine-decided failed outcome, redispatched, answered with a filtered reason, then
     repeated, carries the stored diagnostic;
   - every entry of `OUTCOME_CAUSES` is one non-empty sentence ending with a period, and
     `explain` returns `Err` for an outcome whose rule it does not hold.

   A shell or core test that asserts a reply, record or message this order changes may change
   that assertion alone. Report each such test.

7. **`engine/CONFORMANCE.md`.** In the section "Refusals and reasons (order 046a)", remove the
   unfinished item for an outcome the engine decides (046c). Add a section "Outcomes the engine
   decides (order 046c)": what this order built, and, as unfinished, OPS-012's unknown (the full
   review's order 22) and IFC-046's version error other than -32020, with the resend at another
   revision (decision 1).

8. **The declaration.** `docs/work-orders/046c-declared.yaml` is written: three shapes, one for
   the new case and two for decision 3's script, each with its cases listed. Change it only
   where `main` has moved and the script changes other cases, and then only their lists, and
   report it.

9. **Before you finish, check:**
   - every outcome the engine decides as failed or unknown in the vectors carries a diagnostic
     whose `ruleId` is its row's rule;
   - no outcome that an implementation stated gains a diagnostic, and the ten steps of `KEEP`
     pass as they are;
   - a cancelled setup or retirement names OPS-022 and a cancelled cascade LIF-021, each with a
     cause that says it was cancelled; OPS-027 is named only by a hook child that a cancel ends
     and by what `requests::cancel` ends unknown, never at a settle or at a cascade's end.

## Scope

Change:

- `engine/crates/core/`: `src/explain.rs`, `requests.rs`, `lifecycle.rs`, `retirement.rs` and
  `cascade.rs` at the sites the decisions name, and every caller of `requests::finish`, which
  gains an argument; report any other core file you change, and why;
- `engine/CONFORMANCE.md`, and `engine/STANDING.md` through the tool alone;
- `conformance/`: the 25 cases of decision 3, through its script; the new
  `conventions/explain/outcome-causes/` and its entry in `conventions/explain/index.yaml`;
- `docs/work-orders/046c-declared.yaml`, only as decision 8 allows;
- `experiments/011-short-form-spike/`, only if its tests need it: it depends on the core.

Do not change:

- `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, or any record (`docs/proposals/`,
  `docs/AUDIT.md`, `ROADMAP.md`, `docs/implementation/`): report any defect as a finding;
- `conformance/case.schema.json`, `conformance/case-schema/`, `conformance/internal-cases.yaml`
  and `conformance/expected-failures.yaml`;
- `conformance/manifest.yaml`, which ends byte for byte as it was;
- any case outside decisions 3 and 4, by hand or by tool;
- `engine/crates/conformance/` and `engine/crates/shell/`, but for tests that assert what this
  order changes;
- `access.rs`, the refusals and reasons of order 046a, and the repeat index in `requests.rs`
  beyond the stored `cause` of decision 1.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_rulings.py
python3 tools/set_names.py --check
python3 tools/check_schemas.py
python3 tools/check_roadmap.py
python3 tools/check_model_account.py
python3 tools/promise_standing.py --check
python3 tools/check_private_paths.py
python3 -m unittest discover -s tools/tests
git diff --check
python3 tools/vector_delta.py HEAD --declared docs/work-orders/046c-declared.yaml
```

Run the Python checks that need PyYAML with `/usr/bin/python3` where `python3` lacks it.
`check_model_account.py` and `promise_standing.py --check` must end `Errors: 0`; the second
fails until decision 5's `--write` has run. Computed on a scratch copy of `9af1c29f` with order
046a's decision 8 and its hand edit applied, `vector_delta.py` ends
`Vector delta: 63 changes; 0 flips; 0 undeclared; 0 errors.`: 62 from decision 3's script and
the new case. On `main` with order 046a merged, `0 flips; 0 undeclared; 0 errors.` must hold;
report the count, and say why where it is not 63.

This prints `0 0 10`: the old form left beside an outcome the engine decides, and the ten steps
of `KEEP` that keep it.

```bash
python3 - <<'EOF'
import json, pathlib
P = next(l.split(":", 1)[1].strip() for l in pathlib.Path("spec/names.yaml").read_text().splitlines()
         if l.startswith("metaPrefix:"))
K = {("requests-aware-failed", 3), ("requests-aware-unknown", 3),
     ("requests-repeat-failed-earlier-answered", 3), ("requests-repeat-failed-earlier-answered", 5),
     ("requests-failed-read-repeat", 3), ("requests-read-is-error", 3),
     ("requests-read-invalid-parameters", 3), ("requests-read-parameter-headers", 4),
     ("requests-read-capability-error", 3), ("requests-cancel-confirmed", 3)}
a = b = k = 0
for p in pathlib.Path("conformance").rglob("case.yaml"):
    case = json.loads(p.read_text())
    for n, s in enumerate(case["steps"], 1):
        e = s["expect"]; r = e.get("reply"); ab = e.get("absent") or []
        if not isinstance(r, dict):
            continue
        m = r.get("_meta") if isinstance(r.get("_meta"), dict) else {}
        if m.get(P + "outcome") not in ("failed", "unknown"):
            continue
        ct = r.get("content")
        bare = (isinstance(ct, list) and bool(ct) and isinstance(ct[0], dict)
                and ct[0].get("text") in ("failed", "unknown"))
        if (case["id"], n) in K:
            k += bare and "/reply/structuredContent" in ab
            continue
        a += bare
        b += "/reply/structuredContent" in ab
print(a, b, k)
EOF
```

Each of these prints 1:

```bash
grep -c 'pub const OUTCOME_CAUSES' engine/crates/core/src/explain.rs
grep -c 'Outcomes the engine decides (order 046c)' engine/CONFORMANCE.md
grep -c '"explain-outcome-causes"' conformance/conventions/explain/index.yaml
```

Each of these prints 0:

```bash
grep -c 'explain-' conformance/internal-cases.yaml
grep -c 'structuredContent"\] = json!({"diagnostics"' engine/crates/core/src/cascade.rs
```

This prints nothing: `git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml spec tools packages`.

From `engine/`, one at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. explain-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

What each must show:

- The pieces tool prints `Nothing to move.`
- The `explain-` run prints a `PASS` line for each of the four cases and ends `4 cases passed`.
- The two test commands pass. Report the totals passed, failed and ignored of the full run. If a
  shell test fails, rerun it alone; rerun a timing-sensitive one with `-- --test-threads=2`.

Then show that the case can fail. From `engine/`, one at a time:

1. `cp crates/core/src/requests.rs target/requests.rs.046c`
2. Make the IFC-010 row of decision 1 pass `None`. Run
   `cargo run --release -p conduit-conformance -- .. explain-outcome-causes`. It must fail,
   printing a line that holds `step 15:` and `/reply/structuredContent`.
3. `cp target/requests.rs.046c crates/core/src/requests.rs`, and run the same command a last
   time. It must pass.

Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not
rebuild. Then `cmp crates/core/src/requests.rs target/requests.rs.046c` exits 0.

In `experiments/011-short-form-spike/`: `cargo test --release`.

Do not stage anything. In your report:

- the cases added and changed, by path, and the counts that decision 3's script and
  `vector_delta.py` gave;
- each test of decision 6 by name, and each existing test whose assertion you changed;
- the causes you wrote for `OUTCOME_CAUSES`, and any rule you added to it;
- the line the run that had to fail printed;
- every choice you made yourself, marked as yours;
- every finding against the specification;
- the last line of every check.

`git diff --check` and `check_private_paths.py` read tracked files only. The admin then stages the
new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. Before
the merge the admin runs `python3 tools/gate.py`, and merges on its PASS
([ruling 80](../proposals/ruling-80/README.md)); GitHub's CI runs on the pull request too, and the
merge does not wait for it.

## Record

Run on 2026-11-18, from about 03:16 to 04:42 UTC, on the Mac from `main` `2d209c09` (order 050g's
merge), in three rounds: the worker (1600 s) and a fresh Codex verifier (449 s), then the worker
again (1278 s) and a second fresh Codex verifier (406 s), then the worker a third time (1013 s) and
a third fresh Codex verifier (397 s). The change is the five core files of Scope and the `None` of
`records.rs`'s successful `finish`, `engine/CONFORMANCE.md` and `engine/STANDING.md`, the new case
`explain-outcome-causes` (17 steps) and its index entry, and decision 3's edit of 25 cases, which
removed 30 bare text assertions and 32 `structuredContent` absences. `requests::finish` takes the
`cause`, keeps it with the ended invocation, and a repeat answered from a stored failed or unknown
outcome, and `complete`'s restoration of `previous`, carry it. Each call site names decision 1's
rule; `OUTCOME_CAUSES` holds eight entries (IFC-008, IFC-010, IFC-046, IFC-047, IMP-007, LIF-021,
OPS-007 and OPS-027); a part held back names POOL-013 in `setup.parts` and `status.setup.parts`;
decision 6's tests are present. The two OPS-012 cases stay expected failures for the full review's
order 22, and the unbuilt -32022 handling and resend is listed as unfinished. The worker's own
choices: each diagnostic names the caller's subject, a core reference as the caller spelled it or
the container named by a cascade's tool, else the tool; a cascade's LIF-021 message holds the
container retirement's recorded OPS-022 message, or the preflight reason that held it back; and
the cascade and filtered repeat tests check `ended_reply` directly, since OPS-024 redispatches
every failed repeat, which the worker reports as the nearest sound reading.

The first verifier found two musts: the diagnostics named no subject, which IFC-050 asks for, and
a cascade whose container retirement was held back did not say why. Round 2 added the named subject
and the preflight cause. The subject changes decision 1's message form to
`{operation} for {subject} ended {outcome} under {RULE}: {cause}`; the admin's round 2 passed the
first verifier's must on, and the coordinator holds the departure, open to the owner's strike. The
second verifier found two musts: a cascade dropped the cause of a dispatched container retirement,
and a provider call named only its tool, where the caller named a file. Round 3 put the child's
cause in the cascade's message and named the path of `file.read.v1`, `file.write.v1` and
`directory.list.v1`. The third verifier found three musts: a `directory.read.v1` call named no
path, an invocation in progress from before this change lost its subject at recovery, and the
cascade rewrote the message of each remaining member. Its should asked for a per-vector check of
the deciding rules. The first two verifiers' runner should, that an outcome diagnostic with no
`message` can pass, is left to a follow-up order with scope in `engine/crates/conformance/`.
Rounds 2 and 3 applied the verifiers' other shoulds: the POOL-013 test with one met and one unmet
dependency, the unfinished item naming -32022, and the LIF-031 test fixture.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), on a scratch copy of
the branch at `020a6751`, before its rebase. It ran every check of "Done when": the Python checks
(Errors: 0 each, STANDING.md current with 520 rows), the tool tests (123, OK), `git diff --check`
(clean), `vector_delta.py` against HEAD (0 changes) and against `origin/main` (63 changes; 0 flips;
0 undeclared; 0 errors), decision 3's script on the merge base (the 25 cases byte for byte), the
KEEP script (`0 0 10`), the greps, fmt, clippy, pieces (`Nothing to move.`), the `explain-` run (4
cases passed), the vectors test (2 passed) and the full release run (316 passed, 0 failed, 4
ignored), the mutation (`step 15: /reply/structuredContent: missing required field`, then PASS and
`cmp` 0) and the spike (9 passed). For item 9 it put a placeholder rule in all 32 edited vector
steps in a probe copy: each actual `ruleId` matched decision 1's row. It upheld none of the third
verifier's musts as a must: the two on subjects follow from a subject rule the order does not ask
for. It found no must and three shoulds:

- S1: decision 1 gives each remaining member of a cascade "the message the code writes now", and
  the build wrapped it in the outcome form.
- S2: `OUTCOME_CAUSES` gave IFC-010 the cause "The plain implementation gave no complete result
  establishing the outcome.", false for a complete `isError` result on an operation that states an
  effect.
- S3: the named subject departs from decision 1's form, and for provider calls it matched the bare
  local names of one package's operations in the core, which left `directory.read.v1` out.

The fixer rebased the branch onto `origin/main` at `408d5f17` (order 048a-1, amended) with the work
order's identity, without conflict, and applied each finding:

- S1: each remaining member's diagnostic is again `ruleId` LIF-021, `level` error, with the whole
  message `Shutdown is unconfirmed for {kind}/{name} ({id}).`, and the tests
  `cascade_cancel_lists_cancellation_before_any_remaining_member` and
  `cascade_repeat_replays_diagnostics_for_all_remaining_members` expect the bare sentence.
- S2: IFC-010's cause reads "The plain implementation's answer does not establish whether the
  operation acted."
- S3, the reader's option (a): the filesystem branch of the named subject goes, so a provider call
  names `tool {tool}`; the core-reference and cascade subjects stay. The sentence "Filesystem
  outcome diagnostics name the path ..." of `engine/CONFORMANCE.md` goes. The test is renamed
  `plain_file_write_outcome_names_the_tool_and_omits_the_arguments`, and asserts
  `example.org/files#file.write.v1 for tool project.file.write.v1 ended unknown` and that the reply
  holds neither the content nor `hello.txt`. With it goes the third verifier's `directory.read.v1`
  must; its recovery must is still not upheld: an invocation persisted before this change has no
  stored subject, and its IMP-007 diagnostic takes decision 1's form. The reader's spec finding,
  that contracts carry no marker for a subject that may be disclosed, is a record outside this
  order's scope: the coordinator files it.

The fixer then ran, on the files of this commit before it was made: the Python checks of "Done when"
(Errors: 0 each, STANDING.md current), the tool tests (129, OK, the tests `main` gained since),
`git diff --check` and `git diff --check origin/main...HEAD` (clean), `vector_delta.py` against HEAD
(0 changes) and against `origin/main` (63 changes; 0 flips; 0 undeclared; 0 errors), the KEEP script
(`0 0 10`), the greps (1, 1, 1, 0, 0), the protected paths (unchanged), fmt and clippy (clean),
pieces (`Nothing to move.`), the `explain-` run (4 cases passed), the full release run with
`RUST_TEST_THREADS=4` (316 passed, 0 failed, 4 ignored, the vectors test's 2 among them), the
mutation (`step 15: /reply/structuredContent: missing required field`, then PASS and `cmp` 0) and
the spike (9 passed). The gate's run on this order's squash is the coordinator's at the merge. The
coordinator rules on the decision that S3 left open: it accepts the named subject's message form
`{operation} for {subject} ended {outcome} under {RULE}: {cause}` for core references, cascades and
every other dispatched call (`tool {tool}`) as a departure from decision 1's form, open to the
owner's strike, and the filesystem branch stays deleted (option (a)). At the merge the coordinator
adds the departure to its list for review with the owner, as row 40, and files the reader's spec
finding as finding 4-37 in increment 4's log.
