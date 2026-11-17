# Work order 171b: SO-A, part 2: a sign-in return kept nowhere, and the sign-in holds

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. List in `vectors` every case you add or change, with the promises it
tests and whether it passes. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it. Use it for nothing else, and reach no other
machine. Run one cargo command at a time. Never print a whole large file, fixture or log: use
`head`, `tail`, `grep`, `sed -n`, `wc` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

This is the second part of the full review's order 19 (SO-A in
[its orders](../proposals/full-review/orders.md), "The two specification orders the build must
not overtake"). Order 171a, the first part, wrote the form of a promise and `access`, with
[ruling 80](../proposals/ruling-80/README.md)'s point 1. This order writes sign-in, in
`spec/promises/pending.yaml` and the sign-in parts of IFC-049. It starts once 171a has merged. It
lands before orders 046a, 047 and 048a-2 are written (row 19's place). The review's order 24,
MC9a, is registered against its text when 048a-2 is drafted. Ruling 78, point 1, part 2, keeps
sign-in before the pin, so this part does not move after it.

1. **A return is kept nowhere, in a promise of its own**
   ([FR-140](../proposals/full-review/findings/L13-framing.md#fr-140), its verifier's correction 1;
   fix
   [F122](../proposals/full-review/decisions.md#the-form-the-layers-definitions-and-the-limits)
   of the coordinator's decisions; unsafe).
   - The rule that a browser login's return appears in no result, notice, log or record stands
     today only in the observables of PND-013 (`pending.yaml`, lines 267 to 270) and IFC-049
     (line 1419).
   - Order 171a's form already makes it bind. F122 also gives it a promise ID and a case of its
     own, and this order does that.
2. **A sign-in action's expiry lifts no hold**
   ([FR-062](../proposals/full-review/findings/L06-secrets-pending.md#fr-062), as its verifier
   narrowed it; fix
   [F79](../proposals/full-review/decisions.md#secrets-pending-actions-and-sign-in) of the
   decisions; unsafe).
   - PND-005 holds model work "until answered", and PND-003 counts an expiry as an answer.
   - So a login finished late under the wrong account is never checked. The action then expires,
     and the Agent works under that account.
   - F79 makes the hold the instance's. It ends only when `sign-in.v1` answers that the instance
     is signed in.
3. **The hold falls on the instance, a Resource included**
   ([FR-063](../proposals/full-review/findings/L06-secrets-pending.md#fr-063) =
   [FR-192](../proposals/full-review/findings/X04-stated-limits.md#fr-192); fix F80; unsafe).
   - A failed undo holds "all model work", and so does PND-005's hold. A Resource has no model
     work. So a ready Resource that is signed in to the wrong account goes on serving every Agent
     that has a Conduit to it.
   - F80 holds every request to the instance except the parts of `sign-in.v1`, of a setup and of
     its retirement.
4. **A sign-in that starts from another account raises the hold**
   ([FR-068](../proposals/full-review/findings/L06-secrets-pending.md#fr-068), with its
   verifier's alternative; fix F81).
   - While `sign-in.v1` polls, the account its first check found counts as no account yet.
   - That first account, where it is not the expected one, also raises the instance's sign-in
     action. This is a fourth trigger of PND-005, so no work runs under an account the engine
     has seen to be wrong.
   - PND-011's recovery stays.
5. **A request without the URL capability**
   ([FR-040](../proposals/full-review/findings/L04-interface.md#fr-040) =
   [FR-075](../proposals/full-review/findings/L06-secrets-pending.md#fr-075); fix F82).
   - Such a request is decided after access and before any part. It gets -32021, reports invalid
     and is recorded, as IFC-036 records one.
   - PND-010's "not invalid" becomes the wire form that order 038a corrected.

**F84 is not in this order.**
[FR-067](../proposals/full-review/findings/L06-secrets-pending.md#fr-067) found that PND-005's
trigger "its contract reports a missing or lapsed login" (line 143) has no form an engine can
see. Fix F84 gives it two halves: an implementation that carries Conduit's keys ends the call
failed or unknown with an error diagnostic naming PND-005, and a plain implementation shows a
lapse only by asking for a URL (IFC-016). Each half settles part of a question that the targeted
review below owns: whether the hooks and wrappers are keyed or plain, and how a login that lapses
during a turn reaches the engine. On order 078's unmerged branch, the `.v2` Codex Type's turn
wrapper states `plain: true`.

- The plain half decides what such a wrapper can report.
- The keyed half, written in place of the trigger, as FR-067's least change has it, leaves
  URL-mode elicitation as a plain implementation's only route, which is the plain half by another
  road. Written beside the trigger, it fixes what a keyed wrapper reports, which the same
  question decides.

So F84 is left whole to that review, its plain-implementation clause among it. PND-005's third
trigger keeps its words, and IN_FORCE.md's "A lapsed login raises PND-005" (line 209) stays.

**Who decided what.** F79 to F82 and F84 are the coordinator's, each logged in
[decisions.md](../proposals/full-review/decisions.md) for the owner to strike. The review's
questions list the findings of F79 to F81 and F84 among the sign-in fixes under "Not asked now"
([questions.md](../proposals/full-review/questions.md#not-asked-now-about-these-subjects)), with
the hold on a Resource on the review list.
[Ruling 78](../proposals/ruling-78/README.md#the-decisions) struck none, and
[ruling 80](../proposals/ruling-80/README.md#the-decisions) names none. F122 is the coordinator's
too (order 171a). These are also the coordinator's, open to the owner's strike: leaving F84 to
the targeted review; the class that decision 9 gives PND-014, as the
[safety catalogue](../implementation/SAFETY_CATALOGUE.md)'s classing is (its lines 5 and 6); and
the quotes it gives the catalogue's rows of PND-011 and PND-013. Ruling 80 decides nothing in
this order. Its point 8 is order 050's (below), and its point 10 changes only how the admin
merges this order (at the end).

**The targeted review, and what it owns.** Later on 2026-11-16 the owner set a targeted review
with one target: whether the Agent packages 0.2.0's sign-in and home surface can be frozen under
NAM-024. That surface is `signIn` and its logins, `account` or `apiKey`, the hooks, each Agent's
home, the wrappers' fields of the review's orders 16 and 17, and the connection Conduit manages
for a Claude sign-in. The owner held orders 048a-1, 048a-2, 048b-join, 050 and 051b, unwritten,
until that review has its record and the owner has answered it. The review reads `spec/` as it
stands, so nothing this order writes may settle a question the review owns:

- **Keyed or plain hooks.** F84 stays out (above). PND-010's sentence on a plain hook and
  PND-013's first check, which reads whether the hook carries Conduit's keys, do not change.
- **The connection Conduit manages** ([ruling 80](../proposals/ruling-80/README.md#the-decisions),
  point 8; IN_FORCE.md line 216, spec pending for order 050). This order writes nothing about it,
  and the text of PND-013 and PND-014 neither settles nor rules out such a connection.
  - PND-014 governs only a value that `signInReturn` carries, or that the engine holds to pass to
    `sign-in-hook.v1`. It names no connection, tunnel, SSH session, forward or remote desktop.
  - Point 8's "never reads, holds or relays the credential" is about a Claude Agent's sign-in.
    PND-013 still has the engine carry the return of a login that states `carried: true` to its
    hook (rulings 73 and 75 C5), and this order keeps that.
  - The limits in `spec/README.md`, lines 431 to 438, which point 8 changes, IFC-049's sentence
    for `carried: false` (line 1370) and the core schema's description of `carried`
    (`spec/schemas/contracts.yaml`, line 424) stay as they are: order 050 writes point 8.
- **How Codex opens a browser, and its fix.** Nothing here touches it.
- **The reason a second `sign-in.v1` of one instance is refused**: conflict (F85), or unavailable,
  as PND-010's rule on a fixed port has it (lines 173 to 176). PND-010's `then` does not change.
- **F95 to F97.** This order writes none of them. The word "session" stays in PND-010 and IFC-049
  (F96).
- **The device login.** PND-010's device leg and PND-005's evidence of a device user code stay.
- **The freeze of 0.2.0.** Nothing under `packages/` changes.

**Rulings 78 and 80: the lines this order leaves.** This order writes none of these lines of
IN_FORCE.md, each marked *spec pending*, and each stays marked:

- **Line 206**, the engine's own panel: PND-004's audience, the device leg of PND-010, a
  definition and the panel's paragraph in "Pending actions, sign-in and the network" (78 point
  9). The order written with 051c writes it.
- **Line 216**, the connection Conduit manages for a Claude Agent's sign-in, and the limits it
  changes (80 point 8). Order 050 writes it.
- **Line 217**, a call that changes nothing ends failed on an unanswered input request under
  IFC-016 (78 point 7). SO-E, point 7's order, writes it.
- **Line 45**, the form: rule 1, the rule on an observable and the `vectors` field (78 point
  17). SO-H writes it.

This order changes only PND-004's sentence on a return. It leaves PND-004's audience sentence,
the device leg of PND-010, IFC-016 and the paragraphs of the limits that lines 206 and 216 name.

**Cases.** PND-014 is this order's one new promise ID, so it enters with a case that can fail
([CONTRIBUTING.md, rule 8](../../CONTRIBUTING.md#writing-promises)). It is a convention-layer
safety promise, and no model checks it yet, so its row in the model-checking account says why
(decision 9). PND-005, PND-010, PND-011, PND-004 and IFC-049 keep their IDs, so rule 8 asks no
case of them. Their vectors come with the sign-in orders 048a-1 and 048a-2, which build sign-in
and its fixture Type, as row 19's size cell says. Both are held until the targeted review.

The engine cannot pass the new case yet:

- its core request has no member for `signInReturn` and denies unknown members (`AccessRequest`
  in `engine/crates/core/src/access.rs`, from line 16);
- `sign-in.v1` is the stub that names order 048a (`engine/crates/core/src/requests.rs`, line
  1050).

So the case is listed in
[conformance/expected-failures.yaml](../../conformance/README.md#expected-failures), which order
163 made.

**Before you start.** Check that order 171a and ruling 80 are on your tree. From the repository
root, each of these prints 1:

```bash
tr -s ' \n' '  ' < spec/README.md | grep -c 'Where `observable` and `then` differ, `then` governs.'
test -d conformance/conventions/access/close-root-outside-bound && echo 1
grep -c 'Spec pending, ruling 80, order 050' IN_FORCE.md
```

If one does not, stop and report it. Then run `python3 tools/promise_standing.py --check` and
`python3 tools/check_model_account.py` once, before any change, and keep their output for your
report. Orders 172a and 172b (SO-B) may merge before or after this one. They change other rows of
`spec/definitions.md`, other lines of IN_FORCE.md, other rows of the account and the same list in
`conformance/expected-failures.yaml`, and so `engine/STANDING.md`. Whichever lands second rebases
and writes `engine/STANDING.md` again with the tool.

**Size: M.** About 80 changed lines of `spec/` in four files, one new promise and one new case,
three rows of the safety catalogue, one row of the account, five counts, and the generated
`engine/STANDING.md`.

Read first:

- the findings linked above, each with its verifier's corrections, which govern;
- F79 to F85, F95 to F97 and F122 in [decisions.md](../proposals/full-review/decisions.md), and
  D30 there;
- IN_FORCE.md, lines 202 to 217 and line 295;
- `spec/promises/pending.yaml` whole;
- IFC-006, IFC-016, IFC-036 and IFC-049 in `spec/interface/implementation.yaml`;
- in `spec/README.md`, "Contents" (lines 192 to 260) and "Pending actions, sign-in and the
  network" (lines 402 to 453);
- the rows return, sign-in and account check in `spec/definitions.md`;
- in `conformance/README.md`, "Assertions", "The coverage index and the case schema in pieces",
  "Expected failures" and "Reviewing vector changes";
- the case `requests-reconcile-invalid` (`conformance/conventions/requests/reconcile-invalid/`),
  its steps 1 and 2 and its `requests.json`;
- in the safety catalogue, lines 1 to 30 and the section on `spec/promises/pending.yaml` (from
  line 357); the PND rows of `experiments/012-model-checking/account.yaml`; and
  `experiments/012-model-checking/G5.md`, lines 1 to 20, line 85 and lines 222 to 240;
- [order 181](181-uncatalogued-promises-classified.md), decisions 2, 3 and 5 to 7, for the form of
  a catalogue row, an account row and the counts;
- in AGENTS.md, what `promise_standing.py` needs.

## The decisions it applies

Line numbers are those of `main` at `d4c8cf8b`. Order 171a may move some of them in
`spec/README.md` and `spec/interface/implementation.yaml`, and orders 172a and 172b some in
IN_FORCE.md, `spec/definitions.md` and the account, so find each by its words. Rewrap only as
far as a change needs, at the field's four-space indent, with every line under 97 characters.
Keep everything a decision does not name (CONTRIBUTING.md, "Writing promises", rules 1 and 7).
`spec/` carries no change narrative. Each block below that is indented to sit in its list item is
written without that indent.

1. **PND-014: the return is kept nowhere** (F122; FR-140's verifier). In
   `spec/promises/pending.yaml`, add this promise after PND-013. No PND-014 has ever been used:
   the file's head comment registers no removed ID, and `git grep PND-014` finds none.

   ```yaml
   - id: PND-014
     layer: convention
     level: must not
     when: >-
       An Interface's request or retry carries io.github.shrav-k.conduit/signInReturn, whether
       or not PND-013 accepts it as a return, or the engine holds a return's value to pass to
       sign-in-hook.v1.
     then: >-
       The value appears in a result, structuredContent, response _meta, notification,
       diagnostic, log or record, is among the values REC-002 keeps, reaches an Agent, or stays
       in the engine's memory after the sign-in-hook.v1 retry that passes it on has ended or,
       where none passes it on, after the request that carried it has ended; or a diagnostic
       holds its code, state, error_description or error_uri, or another URL value taken from it.
     observable: >-
       Only the package and login that started the sign-in receive an accepted value, in the
       engine's sign-in-hook.v1 retry, a part of sign-in.v1, in that retry's own _meta (IFC-006,
       PND-013); that retry's record holds neither the value nor signInReturn, and a restart
       loses the value. A refused return's diagnostic names PND-013, the failed check and its
       cause, and holds none of the value. A request that carries signInReturn where PND-013
       takes no return reports invalid, with a diagnostic naming PND-013 and the key, and holds
       no value.
     vectors: []
   ```

   The result's `error` member, a registered RFC 6749 code or `other` (PND-013), is not the value
   and stays allowed. Add nothing to PND-014 on how a value reaches the program waiting on a
   login, and nothing on a connection over which a person completes a sign-in.
2. **The copies point to PND-014.**
   - **PND-013's observable** (lines 267 to 270). "No diagnostic holds a return, code, state, URL
     value, error_description or error_uri. The value appears in no result, structuredContent,
     response _meta, notification, diagnostic, log or record. It is none of the values REC-002
     keeps. The engine holds it in memory only until the hook retry ends; restart loses it."
     becomes "The value is kept nowhere (PND-014)."
   - **PND-004's observable** (lines 91 and 92). "PND-013 alone governs it, and only signInReturn
     carries it." becomes "PND-013 and PND-014 alone govern it, and only signInReturn carries
     it." Nothing else in PND-004 changes.
   - **IFC-049's observable** (line 1419). "It appears in no result, notification or record."
     becomes "It is kept nowhere (PND-014)." The next sentence, "PND-013 alone accepts it, only
     from the Interface that received the sign-in, in its bound session and invocation.", stays.
   - **`spec/definitions.md`, line 273, the row return.** "never kept (PND-013)" becomes "never
     kept (PND-013, PND-014)".
   - **IN_FORCE.md.**
     - Line 202: "PND-001 to PND-013" becomes "PND-001 to PND-014".
     - Line 212: "the browser login's return, PND-013 and IFC-049" becomes "the browser login's
       return, PND-013, PND-014 and IFC-049".
     - Put no date in either line, since `check_rulings.py` refuses one there.
   - **`spec/README.md`.**
     - Line 220: "Draft; 13 promises; no vectors yet" becomes "Draft; 14 promises; no vectors
       yet". "No vectors yet" is the recorded S1 coverage of 2026-11-10 (line 194), and it stays.
     - Lines 254 and 255: "519 in all: 226 of the layer `language`, 245 of the layer
       `convention`" becomes "520 in all: 226 of the layer `language`, 246 of the layer
       `convention`". `check_schemas.py`'s step 8 compares these figures with the files.
3. **PND-005: a fourth trigger, and a hold that is the instance's** (F79; F80, second half; F81).
   - **Its `when`** (lines 141 to 143) becomes: "An instance whose Type states signIn has a string
     at its account pointer, and setup's account-hook reports no account or a different account;
     or the account-hook.v1 part that sign-in.v1 dispatches before sign-in-hook.v1 reports a
     different account; or its implementation sends URL-mode elicitation outside sign-in.v1; or
     its contract reports a missing or lapsed login." The last trigger keeps its words: F84 is the
     targeted review's.
   - **Its `then`** (lines 145 to 150) changes in two places.
     - "A sign-in action is raised for the instance, with expiry." becomes "A sign-in action is
       raised for the instance, with expiry, unless one is unanswered (PND-001)."
     - The sentence "Until answered, operations that start model work on that instance are not
       dispatched and report unavailable with the action ID in
       io.github.shrav-k.conduit/pendingAction." becomes:

       > The instance is then held until sign-in.v1 answers that it is signed in (PND-010,
       > PND-011): an operation that starts model work on an Agent, and any request to a Resource
       > other than the parts of sign-in.v1, of a setup and of its retirement, is not dispatched
       > and reports unavailable with the ID of the instance's unanswered sign-in action in
       > io.github.shrav-k.conduit/pendingAction. The hold is the instance's, not the action's:
       > an expiry lifts nothing.

     The sentences on evidence and return attempts stay.
   - **Its observable** (lines 152 to 156) does not change. Add no sentence on how a plain or a
     keyed implementation reports a missing or lapsed login.
4. **PND-011: a sign-in that starts from another account, and a failed undo** (F81; F80, first
   half, in FR-192's words).
   - **Its `then`** (lines 207 to 214). "a different account is undone by a sign-out-hook.v1 part"
     becomes "the account that the check before sign-in-hook.v1 found counts as no account yet;
     any other account is undone by a sign-out-hook.v1 part". Nothing else in the `then` changes.
   - **Its observable** (lines 216 and 217). "If sign-out does not succeed, the engine reports
     SignInUndone=False with its cause and holds back all model work." becomes:

     > If sign-out does not succeed, the engine reports SignInUndone=False with its cause and,
     > until that condition clears, dispatches to that instance no request other than the parts
     > of sign-in.v1, of a setup and of its retirement: every other request to it, an operation
     > that starts its model work among them, reports unavailable, with a diagnostic naming
     > PND-011 and the condition.

     The next sentence, "Only a later account-hook.v1 part within sign-in.v1 or setup that
     reports the expected account or none clears that condition", stays.
5. **PND-010: the first check's account, and the missing capability** (F81; F82). In PND-010's
   observable:
   - After "that initial mismatch does not undo the account or end sign-in.v1." (lines 183 and
     184), insert "Another account found there raises the instance's sign-in action under
     PND-005, unless one is unanswered, and counts as no account yet while sign-in.v1 polls
     (PND-011)."
   - Line 189: "Missing URL capability uses -32021 with requiredCapabilities, not invalid."
     becomes "A request that does not declare URL elicitation is decided after the access
     decision and before any part: it gets the protocol's error -32021 with
     requiredCapabilities, not the JSON-RPC error for invalid parameters, and reports invalid
     under IFC-049."
   - Nothing else in PND-010 changes: not its `then`, not its device leg, not its sentence on a
     plain hook, and not "the session and invocation bound by requestState".
6. **IFC-049: where the missing capability is decided** (F82). In
   `spec/interface/implementation.yaml`, IFC-049's `then`, insert after "sending no elicitation."
   (line 1361): "It does so after the access decision and before any part is dispatched; the
   request is not dispatched and reports invalid, recorded so, and the error holds in its data
   io.github.shrav-k.conduit/reason with the value invalid, as IFC-036 has such an error hold
   it." IFC-036 does not change: its cases are decided before access, and this one is not. The
   message's sentence for `carried: false` (line 1370) stays.
7. **The copies of the holds.**
   - **`spec/definitions.md`, line 274, the row sign-in.** It gains a last sentence: "The hold is
     the instance's: an expiry lifts nothing, and a held Resource is sent only the parts of
     sign-in.v1, of a setup and of its retirement (PND-005)."
   - **`spec/definitions.md`, line 278, the row account check.**
     - "Before sign-in-hook, no account or another account proceeds to that hook; during polling
       after its URL elicitation, another account is undone and sign-in fails with the action
       pending." becomes "Before sign-in-hook, no account or another account proceeds to that
       hook, and another account raises the sign-in action; during polling after its URL
       elicitation, the account found before sign-in-hook counts as no account yet, any other
       account is undone and sign-in fails with the action pending."
     - "Failed undo holds model work (PND-005, PND-010, PND-011)." becomes "A failed undo holds
       every request to the instance but the parts of sign-in.v1, of a setup and of its
       retirement (PND-005, PND-010, PND-011)."
   - **`spec/README.md`, lines 413 and 414.** "A wrong account is undone and failed undo holds
     model work until a later account check in sign-in or setup clears it (PND-005, PND-011)."
     becomes "A wrong account is undone, and a failed undo holds every request to that instance
     but the parts of sign-in, of a setup and of its retirement until a later account check in
     sign-in or setup clears it (PND-011). A sign-in action holds its instance until sign-in
     answers that it is signed in, whatever the action's expiry (PND-005)." The rest of the
     paragraph, the section's first paragraph and its paragraph of lines 431 to 438 do not
     change.
8. **The case for PND-014** (rule 8).
   - **Where it goes.** Add `conformance/conventions/requests/sign-in-return-kept-nowhere/`.
     - It holds `case.yaml` (the case `requests-sign-in-return-kept-nowhere`) and
       `requests.json`. Its `promises` are `["PND-014", "PND-013"]`, and its `title` is "A
       sign-in return on a request that takes none reports invalid and is kept nowhere".
     - Copy `layers`, `profile`, `initial` and `files` from `requests-reconcile-invalid`, with
       `requests` pointing at the new file.
     - Its first step is that case's first step, the `register-project` step, unchanged.
     - Its second step, which the runner prints as step 2, is a `request-fixture` step reading
       index 0.
     - Use only input kinds and expectations that exist. Add nothing under
       `conformance/case-schema/`, and leave `conformance/case.schema.json` unchanged.
   - **The request.** Index 0 has the model's `patches`, and a `call` of kind `invoke` with its
     own evidence. In it, root (`…0005`), on its own behalf, asks `management.inspect.v1` with the
     arguments `{"object": {"kind": "Pool", "name": "workspace"}}`, as
     `conformance/conventions/records/expected-id/requests.json` writes an inspection.
     - The request carries signInReturn with a value that is plainly not a secret, at an
       arbitrary port, not one that a client's login uses:

       ```json
       {"value": "http://127.0.0.1:49152/callback?code=not-a-code&state=not-a-state", "via": "paste"}
       ```

     - Write it in the call's `request` as the member `signInReturn`, beside `identity`, `tool`
       and `arguments`. That follows how the runner writes the key
       `io.github.shrav-k.conduit/serving` as the member `serving`. The member name binds
       nothing.
     - PND-013 makes such a request no return: after the access decision it reports invalid,
       with a diagnostic naming PND-013 and the key.
   - **What step 2 expects,** as step 2 of the model case does for its invalid request:
     - `reply` holds what the model's reply holds, `_meta` with
       `io.github.shrav-k.conduit/reason` invalid, `isError` true and its `content`, with one
       entry in `structuredContent.diagnostics`, whose `ruleId` is PND-013 and `level` error;
     - `required_promises` is `["OPS-023", "PND-013"]`;
     - `state` `{"started": true}`, as the model step has it, with no external intent;
     - the model's two records, `access-decision` and `request-ended`, with this case's own IDs
       and target;
     - `absent` holds `/reply/_meta/io.github.shrav-k.conduit~1signInReturn`,
       `/reply/_meta/io.github.shrav-k.conduit~1outcome`, `/reply/structuredContent/signInReturn`
       and, for both records, `/records/N/data/signInReturn` and
       `/records/N/data/io.github.shrav-k.conduit~1signInReturn`.

     Do not put `/reply/structuredContent` itself in `absent`: an invalid reply carries its
     diagnostics there. A diagnostic's message is library prose and is not compared
     ("Assertions").
   - **Its index entry.** Add one entry, `id`, `case`, `promises` and `safety_rows: []` in that
     order, to `conformance/conventions/requests/index.yaml`, as order 157 did. Write it in a
     `cases` list in `conformance/manifest.yaml`, then run the pieces tool from `engine/`. The
     tool moves the entry into its piece and leaves `manifest.yaml` as it was.
   - **The expected failure.** Add one entry to `conformance/expected-failures.yaml`, in byte order
     of `id` among the entries there, keeping the file's form:
     - `"id": "requests-sign-in-return-kept-nowhere"`;
     - `"reason": "The core's request has no member for signInReturn and reads no return; the
       sign-in orders 048a-1 and 048a-2 build PND-013 and PND-014."`
9. **PND-014's class, and the rows that quote what this order moves** (D30;
   [FR-448](../proposals/full-review/findings/G1a-model-checking-register.md#fr-448)). Order 181
   left no convention-layer promise outside the safety catalogue, and each row of the account
   takes its class from the catalogue. PND-014 holds the slice that the catalogue's row of
   PND-013 quotes today, so by the catalogue's own test it is a safety promise of kind secret.
   Decisions 2 and 4 also remove the text that the rows of PND-013 and PND-011 quote, so each is
   requoted and keeps its kind, as orders 081 and 082 did.
   - **The catalogue,** `docs/implementation/SAFETY_CATALOGUE.md`:
     - After line 9, add:

       ```text
       Work order 171b added the row of PND-014, which it wrote, and requoted the rows of PND-011
       and PND-013, whose quoted text it changed.
       ```

     - Lines 11 to 14 become:

       ```text
       Promises read: 520; safety: 412; for the coordinator: 0; others: 108. Safety by kind: access 199,
       secret 45, effect 125, result 43. Safety by layer: language 135, convention 232, interface 45.
       Safety in the seven convention files: 128 (G5’s list as ruling 39 worded it); safety of the
       convention layer anywhere: 232 (the wider reading, for the owner).
       ```

     - Line 359, "Read: 13 promises.", becomes "Read: 14 promises."
     - PND-011's row (line 371) becomes:

       ```text
       | PND-011 | convention | must | effect | `observable`: “dispatches to that instance no request other than the parts of sign-in.v1, of a setup and of its retirement” — running model work or serving a Resource’s requests after an unsuccessful account undo spends work under an account the instance did not authorize. | — |
       ```

     - PND-013's row (line 372) becomes the first row below, and PND-014's row, the second,
       follows it:

       ```text
       | PND-013 | convention | must | secret | `observable`: “Only the package and login that started it receive it” — passing a return to another package or login hands the login’s authorization code to a program that did not start the sign-in. | — |
       | PND-014 | convention | must not | secret | `then`: “The value appears in a result, structuredContent, response _meta, notification, diagnostic, log or record” — publishing a sign-in return discloses the login’s authorization code to the readers of those surfaces. | — |
       ```

     Nothing else in the catalogue changes. If another row quotes text that this order changes,
     leave it and report it.
   - **The account,** `experiments/012-model-checking/account.yaml`: after PND-013's row (lines
     561 to 564), before `- id: PND-006`, add:

     ```yaml
     - id: PND-014
       class: safety
       standing: left to MC9
       note: Written by work order 171b from the rule that PND-013's observable held (F122). FR-448
         names MC9a, order 24 of the full review, for PND-013. Classed by work order 171b.
     ```

     No other row changes: PND-005, PND-010, PND-011 and PND-013 are `left to MC9`, with no
     digest. PND-004 and IFC-049 are of the interface layer and have no row.
   - **The count elsewhere** (D30). In IN_FORCE.md, line 295, "Of the 231 convention-layer safety
     promises" becomes "Of the 232 convention-layer safety promises". In
     `experiments/012-model-checking/README.md`, line 5, "all 231 convention-layer safety
     promises" becomes "all 232 convention-layer safety promises". Rewrap neither line, and change
     nothing else in either file.
   - **G5.md,** `experiments/012-model-checking/G5.md`:
     - In the paragraph of lines 5 to 12, the text from "Work order 181 classed" to the
       paragraph's end becomes the first text below. Rewrap the paragraph to at most 100
       characters a line.
     - Line 14: "**231 convention-layer safety promises**" becomes "**232 convention-layer safety
       promises**".
     - Line 85: "## Outside the 231" becomes "## Outside the 232".
     - In the bullet on the 14 (from line 226), "No model has run on any of them, and no
       conformance case names one." becomes "No model has run on any of them, and only work order
       171b's case names one of them, PND-013."
     - After that bullet, before the paragraph that begins "The catalogue and the source
       accounting agree", add a last bullet, the second text below.

   The paragraph's new end, in G5.md:

   ```text
   Work order 181 classed as safety the 14 convention-layer promises that the catalogue lacked.
   It changed the count below, the heading on what lies outside that count and the MC9 bullet
   to match, and added the bullet on those 14 under What G5 still needs. Work order 171b
   classed PND-014, which it wrote, brought that count and [that heading](#outside-the-232) to
   232, and added the last bullet. The rest of this file is the account of 2026-11-11.
   ```

   The last bullet, in G5.md:

   ```text
   - **PND-014, which work order 171b wrote and classed:** the rule, held until then in
     PND-013's observable, that a sign-in return is kept nowhere (F122 of
     [the review's decisions](../../docs/proposals/full-review/decisions.md)). No model has run on
     it. It is `left to MC9` with PND-013, for MC9a. Its case,
     `requests-sign-in-return-kept-nowhere`, names both, and the first build does not pass it yet.
   ```

   Nothing else in G5.md changes.
10. **The declaration.** Write `docs/work-orders/171b-declared.yaml` in the form of
    `docs/work-orders/081-declared.yaml`, with this one declaration and nothing more:

    ```yaml
    - name: a sign-in return is kept nowhere
      kind: added
      cases: [requests-sign-in-return-kept-nowhere]
    ```
11. **The standing table.** Once the case, its index entry and the expected failure are in place,
    run `python3 tools/promise_standing.py --write` from the repository root, as AGENTS.md asks of
    a new promise, a case's `promises` and the expected failures. It rewrites
    `engine/STANDING.md`. PND-013's row changes to `expected failure`, PND-014's row appears with
    the same standing, and in Totals the column `convention` changes: `expected failure` by plus
    two, `no case, no reason` by minus one, and `all` by plus one. No other row changes. Never
    edit the file by hand.

## Scope

Change only these files, and in each only the lines the decisions name and the rewrapping they
need:

- `spec/promises/pending.yaml`
- `spec/interface/implementation.yaml`
- `spec/definitions.md`
- `spec/README.md`
- `IN_FORCE.md`: lines 202, 212 and 295
- `docs/implementation/SAFETY_CATALOGUE.md`
- `experiments/012-model-checking/account.yaml`
- `experiments/012-model-checking/G5.md`
- `experiments/012-model-checking/README.md`: line 5
- `conformance/conventions/requests/index.yaml`
- `conformance/expected-failures.yaml`
- `engine/STANDING.md`, written only by the tool, as decision 11 says

Add `conformance/conventions/requests/sign-in-return-kept-nowhere/` and
`docs/work-orders/171b-declared.yaml`. `conformance/manifest.yaml` ends unchanged. Change nothing
under `engine/` apart from `engine/STANDING.md`, and nothing under `packages/`. In PND-004 change
only its sentence on a return. Leave alone: PND-010's `then`, its device leg and its sentences
on a plain hook and on the session; PND-005's observable; IFC-016, IFC-036, REC-002 and IFC-049's
sentence for `carried: false`; the paragraph of lines 431 to 438 of `spec/README.md`; the lines
of IN_FORCE.md listed above, and line 209; and every other promise.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_model_account.py
python3 tools/promise_standing.py --check
python3 tools/check_private_paths.py
git diff --check
python3 tools/vector_delta.py HEAD --declared docs/work-orders/171b-declared.yaml
```

What each must show:

- `check_schemas.py`'s step 7 checks 520 promises, one more than before. Its step 8 compares
  "520 promises in 27 files, 226 of the layer `language`, 246 of the layer `convention` and 48 of
  the layer `interface`; 0 errors."
- `check_model_account.py` prints `Rows: 246 (safety 232, not safety 14, uncatalogued 0)` and an
  empty `Uncatalogued IDs: `, its `left to MC9` count is one more than before your change (12
  on `main` at `d4c8cf8b`, so 13 after it), and it ends `Errors: 0`. If an order that merged
  after `d4c8cf8b` changed the account, report the counts you see.
- `promise_standing.py --check` ends `engine/STANDING.md is current; promise rows: 520.` and
  `Errors: 0`. Its `Standings:` line shows two more `expected failure` and one fewer
  `no case, no reason` than the run before your change. It fails until decision 11's `--write`
  has run.
- `vector_delta.py` ends `Vector delta: 1 changes; 0 flips; 0 undeclared; 0 errors.` Run it and
  the two Python checks below with `/usr/bin/python3` where `python3` lacks PyYAML.

The catalogue. Each prints what its comment says:

```bash
awk '/^## /{s=$0} /^\| [A-Z]+-[0-9]+ \|/{n[s]++} END{for(k in n) print n[k], k}' docs/implementation/SAFETY_CATALOGUE.md | sort   # 108 ## The others, then 412 ## Safety promises
awk -F' [|] ' '/^## /{s=$0} s=="## Safety promises" && /^[|] [A-Z]+-[0-9]+ [|]/{print "kind", $4; print "layer", $2}' docs/implementation/SAFETY_CATALOGUE.md | sort | uniq -c   # access 199, effect 125, result 43, secret 45; convention 232, interface 45, language 135
/usr/bin/python3 -c '
import re, yaml
text = open("docs/implementation/SAFETY_CATALOGUE.md", encoding="utf-8").read()
pairs = re.findall(r"^### (spec/\S+\.yaml)\n\nRead: (\d+) promises\.", text, re.M)
differ = [f for f, n in pairs if int(n) != sum(1 for p in yaml.safe_load(open(f, encoding="utf-8")) if isinstance(p, dict) and "id" in p)]
print(len(pairs), "sections;", len(differ), "differ:", " ".join(differ))
'   # 27 sections; 0 differ:
/usr/bin/python3 -c '
import re, yaml
text = open("docs/implementation/SAFETY_CATALOGUE.md", encoding="utf-8").read()
rows = {m[0]: (m[1], m[2]) for m in re.findall(r"^\| (PND-01[134]) \|[^`]*`(\w+)`: “([^”]+)”", text, re.M)}
spec = {p["id"]: p for p in yaml.safe_load(open("spec/promises/pending.yaml", encoding="utf-8"))}
for i, (field, quote) in sorted(rows.items()):
    print(i, field, quote in " ".join(spec[i][field].split()))
'   # PND-011 observable True, PND-013 observable True, PND-014 then True
```

Each of these prints nothing:

```bash
git grep -n -e 'holds back all model work' -e 'olds model work' -e 'not invalid' -e 'appears in no result' -e 'PND-013 alone governs' -- spec
git status --short -- engine ':(exclude)engine/STANDING.md' packages conformance/manifest.yaml conformance/case.schema.json conformance/case-schema
```

Each of these prints 1:

```bash
grep -c '^- id: PND-014$' spec/promises/pending.yaml
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c "The hold is the instance's, not the action's: an expiry lifts nothing."
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'until sign-in.v1 answers that it is signed in'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'counts as no account yet while sign-in.v1 polls (PND-011)'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'reports unavailable, with a diagnostic naming PND-011 and the condition'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'PND-013 and PND-014 alone govern it'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'or its contract reports a missing or lapsed login.'
tr -s ' \n' '  ' < spec/README.md | grep -c '520 in all: 226 of the layer `language`, 246 of the layer `convention`'
tr -s ' \n' '  ' < spec/README.md | grep -c "An SSH forward of a redirect's port is the person's own and carries nothing through Conduit."
grep -c 'Draft; 14 promises; no vectors yet' spec/README.md
grep -c 'PND-001 to PND-014' IN_FORCE.md
grep -c 'Of the 232 convention-layer safety promises' IN_FORCE.md
grep -c 'Spec pending, ruling 80, order 050' IN_FORCE.md
grep -c 'all 232 convention-layer safety promises' experiments/012-model-checking/README.md
grep -c '^## Outside the 232$' experiments/012-model-checking/G5.md
```

Each of these prints 0:

```bash
grep -c -i 'plain implementation' spec/promises/pending.yaml
grep -c '231' experiments/012-model-checking/G5.md
```

In this loop's output, the last two numbers of each line are equal: the words that the targeted
review's questions turn on occur as often as before.

```bash
for f in spec/promises/pending.yaml spec/interface/implementation.yaml spec/README.md; do for w in session SSH tunnel forward; do printf '%s %s %s %s\n' "$f" "$w" "$(git show "HEAD:$f" | grep -o -i -w "$w" | wc -l | tr -d ' ')" "$(grep -o -i -w "$w" "$f" | wc -l | tr -d ' ')"; done; done
```

From `engine/`, run each of these, one at a time:

```bash
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-return
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

What each must show:

- The pieces tool prints `Nothing to move.`
- The runner ends `0 cases passed; 1 expected failures`. Its `EXPECTED FAILURE` line names
  `requests-sign-in-return-kept-nowhere`, and its error names step 2 and comes from the core
  refusing the `signInReturn` member. That shows that the case runs, and that it fails where the
  engine does not keep the promise. If the error names step 1, the case is broken: fix it.
- The two test commands pass. The 4 keeps the runner within memory, as order 071 found.

Do not stage anything. In your report:

- mark each choice you made yourself as your own, and decision 9's class and quotes as the
  coordinator's;
- say whether orders 172a and 172b were on your tree;
- name the pointers you put in `absent`;
- give the `EXPECTED FAILURE` line with its error;
- give the output of `promise_standing.py --check` and of `check_model_account.py`'s first line
  and standings, from before your change and after it;
- give anything decision 9 asks you to report;
- give the last line of every check.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. A
fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The merge waits on the admin's local
gate on the squash, not on GitHub's CI, which still runs on the pull request and does not block
([ruling 80](../proposals/ruling-80/README.md#the-decisions), point 10). That gate is
`python3 tools/gate.py` once order 193b has merged, and until then the checks it stands in for,
the engine's build, its four test shards and its doctests among them.

## Record

Run on 2026-11-16, from about 23:13 to 23:35 UTC, on the Mac from `main` `41c620aa`: the worker
(1,109 s) and a fresh Codex verifier (172 s), which found no must or should in one round. The
verifier reran the repository's checks read-only, but not the Cargo checks. Orders 172a and 172b
were not on the worker's tree. PND-014, a convention-layer `must not`, now holds the rule that a
sign-in return is kept nowhere (F122): the value of `signInReturn` appears in no result,
`structuredContent`, response `_meta`, notification, diagnostic, log or record, is none of the
values REC-002 keeps, reaches no Agent, and leaves the engine's memory once the retry that passes it
on, or else the request that carried it, has ended. PND-013's and IFC-049's observables, PND-004's
sentence on a return and the row return of `spec/definitions.md` now point to it. PND-005 has a
fourth trigger, the account-hook.v1 part before sign-in-hook.v1 reporting a different account (F81),
raises an action only where none is unanswered, and holds the instance until `sign-in.v1` answers
that it is signed in: an operation that starts model work on an Agent, and any request to a Resource
other than the parts of `sign-in.v1`, of a setup and of its retirement, reports unavailable, and an
expiry lifts nothing (F79, F80). Its third trigger keeps its words: F84 is left whole to the
targeted review. In PND-011 the account the first check found counts as no account yet while
`sign-in.v1` polls, and a failed undo holds every request to the instance but those parts, with a
diagnostic naming PND-011 (F81, F80). PND-010's observable and IFC-049's `then` decide a request
that does not declare URL elicitation after the access decision and before any part: -32021 with
`requiredCapabilities`, reported and recorded invalid (F82). The rows sign-in and account check,
`spec/README.md`'s paragraph on a wrong account and its counts, and IN_FORCE.md's lines 202, 212 and
296 (the Scope's 295) follow. The case `requests-sign-in-return-kept-nowhere` (PND-014, PND-013)
sends `signInReturn` on an inspection and expects invalid with a diagnostic naming PND-013 and seven
absent pointers. It is an expected failure: the runner stops at step 2, where the core refuses the
unknown field `signInReturn`. In the coordinator's classing, the safety catalogue gains PND-014 as
`secret` and requotes PND-011 and PND-013, the account gains PND-014 as `left to MC9`, and the count
of convention-layer safety promises goes from 231 to 232 in IN_FORCE.md, experiment 012's README and
G5.md. The worker chose the request's evidence time, 2026-11-16T12:30:00Z, and its IDs, and ran
Cargo offline. `engine/STANDING.md` was written by the tool.

A fresh read-only Opus reader then verified the order at `38408f5d` (ruling 79, point 2). Under
`env -i` it ran `check_docs`, `check_rulings`, `set_names.py --check`, `check_schemas` (520
promises: 226 language, 246 convention, 48 interface), `check_model_account` (Rows: 246, safety 232,
not safety 14, uncatalogued 0), `promise_standing.py --check`, `check_private_paths` and
`check_roadmap`, each `Errors: 0`, `vector_delta.py origin/main HEAD --declared-changed` (1 changes;
0 flips; 0 undeclared; 0 errors) and `git diff --check`, which was clean. In `engine/`, fmt, clippy
and `cargo test --release` passed, the pieces tool printed `Nothing to move.`, and the runner gave
the expected failure at step 2. On a scratch copy the case failed at step 2 with its
expected-failure entry removed, and with `signInReturn` also dropped from the request it failed on
the unexpected `outcome` member, so it can fail. It found every decision applied word for word, the
change within the 15 files of Scope, and nothing settled that topic 9's *Open* line leaves to the
targeted review: PND-005's third trigger, PND-010's and PND-013's `then`, IFC-049's sentence for
`carried: false`, `spec/schemas/contracts.yaml` and the README's limits are unchanged, and the words
session, SSH, tunnel and forward occur as often as before. It found one must, a rebase onto `main`
`121020ec`, where order 172a's entries of the expected failures and its totals conflicted, and four
shoulds:

- The description of `sign-in.v1` in `spec/core/operations.yaml` still reads "Retries poll for the
  account; a wrong account found then is undone and the invocation fails with the action still
  pending. Failed undo holds model work." It contradicts PND-011 and understates F80's hold. "Done
  when"'s grep missed it because "holds model" and "work." fall on two lines. The file is outside
  this order's Scope, so it is left for SO-H's sweep, with the amendment the reader gave: "Retries
  poll for the account; the account the first check found counts as no account yet, and any other
  wrong account found then is undone and the invocation fails with the action still pending. A
  failed undo holds every request to the instance but the parts of sign-in.v1, of a setup and of its
  retirement." A later order's greps should join lines first (`tr -s ' \n' '  '`).
- G5.md's MC9 bullet put PND-008 "in the last bullet", which is now PND-014's. It now says "in the
  bullet on the 14 that work order 181 classed", and the blank line before PND-014's bullet is gone,
  so the list is tight again.
- PND-005's held request reported only unavailable and an action ID, which an Agent can neither see
  nor answer, so it was not told why (ruling 52, point 2). At the coordinator's decision its `then`
  now adds, after the `pendingAction` key, "with a diagnostic naming PND-005 and that the instance
  awaits sign-in to its expected account, without naming that account (PND-004)". This concerns the
  held request, not how a lapse is reported, so F84 is untouched; PND-005 has no digest in the
  account.
- The case asserts the value's absence only at its seven pointers: maps are required-field
  projections and a diagnostic's message is not compared, so an engine that echoed the value in a
  message, another member or a log would still pass. No change was made here. When the core gains
  the member, orders 048a-1 and 048a-2 are to add an expectation that searches the reply, every
  record and the engine's log for the value and its code and state strings. Until then PND-014 is
  not counted as covered by a safety vector.

The reader also noted, without raising them, that PND-014's "reaches an Agent" copies IFC-049's
existing words and is the targeted review's to read, that PND-011 does not say whether the first
account is compared ignoring ASCII case, and that on `main` four catalogue quotes, of PKG-031,
PLC-026, PRV-015 and ACC-056, already fail to match their promises. The coordinator's choices that
this order opens to the owner's strike are leaving F84 to the targeted review, PND-014's class, the
requotes of PND-011 and PND-013, and the diagnostic that PND-005's held request names.

The fixer rebased the branch onto `121020ec`. In `conformance/expected-failures.yaml` it kept order
172a's two entries and this order's, four in all in byte order, and it wrote `engine/STANDING.md`
again with `promise_standing.py --write`: convention `expected failure` 3 and `no case, no reason`
87, totals 3, 150 and 520. `main` then moved three times, to `852a6d52` (order 172b), `20e9b29d`
(order 195) and `f08a73ea` (order 193a), and the branch was rebased onto each. Order 172b's row
setup action sits beside this order's rows return and sign-in in `spec/definitions.md`, so the three
rows were merged by hand, each keeping its own order's text. The expected failures merged clean, six
entries in byte order, and the tool wrote `engine/STANDING.md` again: convention `expected failure`
5 and `no case, no reason` 85, totals 293, 5, 74, 148 and 520, with PND-013 and PND-014
`expected failure`. Orders 195 and 193a touched none of this order's files. Then the shoulds above
were applied. On the result, under `env -i`, `check_docs`, `check_rulings`, `set_names.py --check`,
`check_schemas` (step 8: 520 promises, 226, 246 and 48), `check_model_account` (Rows: 246, safety
232, not safety 14, uncatalogued 0; `left to MC9` 13), `promise_standing.py --check` (current; 520
promise rows), `check_private_paths` and `check_roadmap` each end `Errors: 0`;
`vector_delta.py origin/main HEAD --declared-changed` ends
`Vector delta: 1 changes; 0 flips; 0 undeclared; 0 errors.`; and
`git diff --check origin/main...HEAD` is clean. Every command of "Done when" prints its value, with
`origin/main` in place of `HEAD` where a command compares against `HEAD`, and the catalogue still
quotes each PND promise word for word. With lines joined, the old words of the holds remain in
`spec/` only in `spec/core/operations.yaml`. In `engine/`, `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings` pass, the pieces tool prints `Nothing to move.`, the
runner ends `0 cases passed; 1 expected failures`, the case failing at step 2 on the core's refusal
of `signInReturn`, and `CONDUIT_VECTOR_THREADS=4 cargo test --release` passes, 242 tests in 28
suites.
