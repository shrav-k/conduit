# Work order 215: the sign-in chain's follow-up: a cancel reaches every part, the raised action is linked, and the expiry timer's start is tested (written ahead)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; list under `vectors` each case whose expectations change. This
order needs no network: it reaches no host and fetches no crate, and the only sockets it opens are
the loopback ports that the engine's own tests bind, as every run of the suite does. Run one cargo
command at a time. Never print a whole large file, case, fixture or log: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Do not run
`conduit` against a store, `codex`, `claude` or the dogfood harness, and sign in nowhere.

## What and why

**What it fixes.** The sign-in chain left its last findings to "one sign-in follow-up order after
the dogfood, which the coordinator writes":
[order 048a-5gh's Record](048a5gh-build-sign-in-undo-and-expiry.md#record) carries its reader's
R4 to R7 (the verifier's V4 to V7), R10 and the second read's S2, and
[order 048a-5ef's Record](048a5ef-build-sign-in-relay-and-polling.md#record) its second read's
R3. The readiness review's delta pass, as the coordinator's notes (private) record, added its
1-N4, the embedded `names.yaml` prefix; 1-N2, the sentence on a mid-call Ctrl-C; and m1, the
engine message's SSH wording. The coordinator read each finding at its source and in the code on
`main`. Six are defects where `spec/` already says what must happen and the build does otherwise,
or a missing test of a promised behaviour; this order fixes those six (decisions 1 to 6), and
decision 1 carries 1-N2, a sentence of `engine/README.md` that its fix bears on. The others are
not in it:

- S2, a late poll's other account left signed in, is a behaviour that `spec/` does not settle:
  DL18 says no hook runs after the wait ends, and nothing says whether the instance is then held.
  The coordinator puts it to the owner.
- 1-N4 is a build choice, not a behaviour: `conduit sign-in` reads `metaPrefix` from the source
  tree at run time, as `serve`, `install` and `submit` read `spec/` from it, so embedding that one
  value would leave the command no more movable and give the prefix a second source. The
  coordinator decides it apart.
- m1, the engine message's sentence that a person seated at the node's machine completes the login
  "by remote desktop or by SSH", is `spec/`'s wording (IFC-049's text in
  `spec/interface/implementation.yaml` and `spec/schemas/contracts.yaml`), so it is the owner's;
  the coordinator puts it to the owner.
- A repeat of a sign-in that succeeded after a relay already answers with `signInLogin`, as
  OPS-009 asks: 048a-5ef's first read's R3 was fixed in that order, in the repeat path of
  `invoke_inner`, with its test `succeeded_repeat_keeps_sign_in_login`. Nothing is owed.

**When it runs.** Written ahead, before order 214 merges. Order 214 also changes
`engine/crates/core/src/requests.rs`, so this order starts on `main` once 214 has merged, and its
start check stops unless 214's Record is on `main`. It is the first order that A78's stage 2 sends
through Conduit ([ruling 89](../proposals/ruling-89/README.md), point 3, its first dogfood
milestone), under [order 211's registration](211-a78-stage-2-registration.md) (its section "Stage 2's orders"):
it changes nothing under `experiments/` and not the harness, which stays outside the tree; its text
and its report schema are on `main` before its first turn; it runs on the Mac; and it is cut for one
worker turn and one verifier turn. If A78's orders 2 and 3 have not run when stage 2 starts, the
registration makes this order A78's order 2 as well, judged under both. None of this changes what
the worker does. By [ruling 89](../proposals/ruling-89/README.md), orders are less prescriptive:
each decision says what must hold and why, the Done-when checks it, and how it is built is the
worker's.

## The decisions it applies

Each of decisions 1 to 6 names its promise, what `main` does, what must hold and the test that
shows it. Write each test first and run it on the unchanged engine. Each test of decisions 1 to 5
fails there, and you report its failing line. Decision 6 adds a missing test, not a fix: `main`
already starts the timer, so its test passes on the unchanged engine once its seam exists, and its
failing line is the one from the mutation run that the Done-when gives. The decisions are the
coordinator's reading of the promises named, open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)).

1. **A cancel of `sign-in.v1` reaches each of its parts in progress (OPS-027; 048a-5gh's R4,
   widened to every part).** On `main`, `requests::cancel` sends a `cancel` intent for the
   `sign-in.v1` invocation alone and finishes it: a part whose call is in flight (the account
   check before the hook, a poll, the sign-in hook before it answers, a keyed hook's
   `server/discover`, or the undo's sign-out part) is sent no cancel and stays in progress, and
   the waiting sign-in-hook part, whose call has answered with its URL, stays in progress until a
   restart ends it. OPS-027: where the engine implements an invocation in parts, it passes the
   cancellation on to each part in progress, by the protocol's means for a call that it sent, and
   a cancelled part's invocation ends when the cancellation is sent, unknown. So, on a cancel of
   `sign-in.v1`:
   - each part in progress whose call is in flight gets a `cancel` intent for its own invocation
     and attempt and ends unknown with an OPS-027 cause, as `lifecycle::cancel` ends a setup's
     parts;
   - the waiting sign-in-hook part, which has no call in flight, ends unknown with no intent, as
     `end_hook_wait` ends it at every other ending;
   - a later completion of any of them is a late completion, ignored as today;
   - the undo's own consequences stay as order 048a-5gh built them: `SignInUndone` False with
     `SignOutUnknown`, and the attempt kept in the action's evidence.

   The DL18 row of `engine/CONFORMANCE.md` then says that whenever `sign-in.v1` ends, a cancel of
   its request included, its waiting hook part ends unknown, and drops "a cancelled sign-in leaves
   it in progress until a restart ends it (DL6)". The case `requests-keyed-hook-dispatch` changes:
   at its step 22, as the runner counts steps from 1, the cancel of `$sign3` comes while its
   account check `$check3` waits on discovery, so the step's intents gain that part's cancel, and
   step 23's completion of `$check3` becomes a late one, so the end of `$check3` that step 23's
   `record_facts` hold is now step 22's. The test: a core test that cancels a `sign-in.v1` with
   its account check in flight, with its sign-out part in flight, and with its hook part
   waiting, and asserts each time the intents and every part's state.

   `engine/README.md`'s section on `conduit sign-in` ends "Interrupting it leaves the sign-in
   waiting until that wait ends." That holds only at the prompt: an interrupt while a call is in
   flight drops the request, which cancels `sign-in.v1` (the delta pass's 1-N2). Make the
   sentence say both: at the prompt the sign-in waits until that wait ends; while a call is in
   flight it is cancelled, each part in progress with it, and a new `conduit sign-in` may follow
   at once.

2. **A URL request from a part other than the sign-in hook raises nothing (IFC-010, PND-005; R5).**
   On `main`, an account check, a poll or a sign-out part of `sign-in.v1` that answers
   `input_required` with a URL-mode request ends through `finish_login_lapse` as IFC-016's trigger:
   it raises the instance's sign-in action under PND-005, with a cause saying the request came
   "outside sign-in.v1". IFC-010: from any part of `sign-in.v1` other than `sign-in-hook.v1`, a
   URL-mode input request ends that part unknown, and an account check that so ends follows PND-010
   and PND-011; PND-005 raises for a URL request outside `sign-in.v1` only. PND-010's clause that a
   failed or unknown first check "raises nothing that PND-005 does not raise for a contract that
   reports a missing or lapsed login" does not reach further: PND-005's plain form of a lapsed
   login is a URL request "under IFC-016", and IFC-016 raises only for a URL elicitation at a point
   other than a part of `sign-in.v1`. So the clause allows the raise for a keyed lapse diagnostic,
   which stays as built, and not for a URL request from a part. Such a part therefore ends unknown
   under IFC-010, with a cause that says it was a part of `sign-in.v1`, raises nothing and records
   no `pendingAction`, and `sign-in.v1` goes on as after any part that ended unknown: the first
   check proceeds to the hook, a poll counts as no account yet, and a sign-out leaves
   `SignInUndone` False. A setup's parts and every call that is no part of `sign-in.v1` are
   unchanged. The test: with no unanswered sign-in action, an account check's URL answer, and a
   sign-out part's, each leave the instance's pending actions as they were, and each part's cause
   names IFC-010.

3. **No keyed hook of `sign-in.v1` is sent once its instance is retired (LIF-005, LIF-015; R6).**
   On `main`, `complete_hook_discovery` sends a keyed hook's `tools/call` once its discovery lists
   Conduit's extension and its parent is still in progress, without looking at its target. A
   `retire.v1` dispatched while the discovery was in flight has made the instance retired from that
   moment (LIF-005), and LIF-015's observable lists the only requests that address a retired
   instance and can be dispatched, of which no part of `sign-in.v1` is one. LIF-015's then, which
   reports conflict, is for a request to set up or change a retired instance, and a part of
   `sign-in.v1` is not one; so the part is not dispatched, and it reports unavailable in IFC-037's
   unsent form, which `spec/` does not fix otherwise. So, for a part of `sign-in.v1`, where the
   instance is retired or gone when its discovery answers, as `part_answered` already tells it,
   the call is not sent. A
   call never sent has a known outcome, so the part does not end unknown: it reports unavailable as
   the `hook-without-extension` arm of `complete_hook_discovery` reports a keyed hook that IFC-037
   leaves unsent (its attempt fenced, a rejection and a reply, with a reason that names LIF-015 and
   LIF-005 and says the call was not sent), and its end reaches `sign-in.v1` through
   `lifecycle::child_answered`, as that arm's does. The `drop-hook-of-ended-parent` path is not the
   model: it returns without telling the parent, which here is still in progress. `sign-in.v1` then
   ends by the paths that arm's unavailable part takes today, adding no new ending: an account
   check's by the instance's retirement (PND-003), a sign-out part's by the undo's end, whose
   `SignInUndone` False keeps the reason it gets today for a sign-out IFC-037 left unsent
   (`SignOutUnknown`), accepted here since the other account was not signed out and no new reason
   is added. A setup's or a retirement's keyed hooks are unchanged: a retirement's own hooks
   address a retired instance by right. The test: a keyed account check, and a keyed sign-out part,
   whose instance retires between their discovery and its answer: no `tools/call` intent follows,
   the part has ended unavailable with that reason, and `sign-in.v1` has ended failed, under
   PND-003 for the account check and under PND-011 by the undo's end for the sign-out part.

4. **The cancel and restart endings name the action that `sign-in.v1` raised (PND-012; R7).** On
   `main`, `end_sign_in` writes the action that the first check raised into the
   `invocation-ended` record (`link_action`), but `requests::cancel` and `requests::recover` end
   `sign-in.v1` through `finish` alone, so its ending record names no action. PND-012: the record
   of a request that raises an action names the action's ID. So both endings link it as
   `end_sign_in` does. The test: a first check that finds another account raises the action; the
   `invocation-ended` record of a cancel, and of a restart, of that `sign-in.v1` holds the action
   under `pendingAction`.

5. **A failed or unknown first check is explained when `sign-in.v1` ends before any relay
   (PND-011; 048a-5ef's second read's R3).** On `main`, `part_answered` keeps the first check's
   outcome in SI's `check` where it ended failed or unknown, and only `relay_message` reads it.
   Where `sign-in.v1` ends before any URL is relayed (no `sign-in-hook.v1` resolved, the hook
   ending failed, unknown or with a reason, its answer refused under IFC-049, or the instance
   retired during the hook), the ending's diagnostic names only that ending's cause. PND-011: a
   failed or unknown account hook completes no sign-in, and its uncertainty is explained. So each
   such ending's diagnostic also says that the account check before `sign-in-hook.v1` ended
   failed or unknown, as it did, so whether the account is signed in is not known (PND-011); for
   instance `The account check before sign-in-hook.v1 ended {check}, so whether the account is
   signed in is not known (PND-011).` Its outcome and `ruleId` are the ending's own, and the
   endings after a relay are unchanged, since the relay's message has said it. The cases assert
   no words (IFC-050), so none changes. The test: the first check ends failed and the hook ends
   failed, and the ending's text holds the sentence; the first check finds no account and the
   hook ends failed, and it does not.

6. **The shell's expiry timer is started by a relayed sign-in (PND-010, DL18; R10).** `call_tool`
   in `engine/crates/shell/src/server.rs` starts `end_sign_in_wait_at` for each output it
   relays. The function has its test (`sign_in_wait_end_is_submitted_at_expiry`), but no test
   reaches the call: with `drop(end_sign_in_wait_at(` and its arguments removed, every test
   passes. PND-010: DL18 ending without completion ends `sign-in.v1` unknown, and no hook runs
   later. The test: a shell test that relays a sign-in through `serve`, as
   `sign_in_relays_through_the_interface_and_polls_to_signed_in` does, sends no retry, and
   sees `sign-in.v1` end unknown with the decision `sign-in-wait-ended` holding `"ended": true`,
   with no such input submitted by the test. The test waits for that ending with a bound, a
   timeout of a few seconds past the seam's shortened expiry, as the shell tests bound their
   waits with `tokio::time::timeout`, and fails rather than hangs when the ending does not come.
   The relay's expiry is ten minutes after the step's
   time, so the test needs a seam: add the smallest one you judge sound, in the shell, under
   `#[cfg(test)]` or as a constructor only tests use, that changes no production path; DL18's ten
   minutes do not change. Name the seam in your report.

7. **The declaration.** Write `docs/work-orders/215-declared.yaml`, in the form of
   [209's](209-declared.yaml), one shape for each expectation that changes, each naming the
   decision that changes it, so that `tools/vector_delta.py` reports 0 undeclared. Decision 1's
   two steps (22 and 23) of `requests-keyed-hook-dispatch` are the coordinator's expectation. Where another
   case's expectations must change, change and declare it too, and list it first among your
   findings, with the decision and the reason.

8. **Stop and report, change nothing more,** if a decision cannot hold without a word of `spec/`
   changed (the owner's), or if decision 3 or 5 needs an ending that `sign-in.v1` does not have
   today.

## What to read

- OPS-027 in `spec/conventions/operations.yaml`; IFC-010 in `spec/interface/implementation.yaml`;
  LIF-005 and LIF-015 in `spec/conventions/lifecycle.yaml`; PND-005, PND-010, PND-011 and PND-012
  in `spec/promises/pending.yaml`.
- `engine/crates/core/src/sign_in.rs`: `part_answered`, `poll_answered`, `unexplained_check`,
  `undo_part_answered`, `hook_ended`, `refuse_hook_answer`, `end_hook_wait`, `end_sign_in`,
  `sign_in_wait_ended`, `link_action`, and the tests `sign_in_hook_not_dispatched_while_retiring`
  and `sign_in_wait_ends_unknown_at_its_expiry` as models.
- `engine/crates/core/src/requests.rs`: `complete`, `complete_hook_discovery`,
  `finish_login_lapse`, `cancel`, `recover` and `setup_sign_in_hook`;
  `engine/crates/core/src/lifecycle.rs`: `cancel`, the model for decision 1.
- `engine/crates/shell/src/server.rs`: `end_sign_in_wait_at` and its call in `call_tool`;
  `engine/crates/shell/src/server_tests.rs`: `sign_in_wait_end_is_submitted_at_expiry`, the
  `SignInScript` tests and their helpers.
- DL18's and DL19's rows in `engine/CONFORMANCE.md`, and
  `conformance/conventions/requests/keyed-hook-dispatch/case.yaml`, steps 21 to 23 as the runner counts them, with `sed -n`
  or a short script.

## Before you change anything

From the repository root, check that you find what this order was written against, and stop,
change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 201005c7 HEAD; echo "main $?"
f=$(find docs/work-orders docs/archive/work-orders -name '214-*.md' 2>/dev/null | head -n 1); test -n "$f"; echo "214 $?"
grep -c '^Not yet run\.$' "$f"
grep -c -e 'fn complete_hook_discovery' -e '"drop-hook-of-ended-parent"' -e 'fn finish_login_lapse' -e '"forward-cancellation"' engine/crates/core/src/requests.rs
grep -c -e 'fn undo_part_answered' -e 'fn end_sign_in(' -e 'fn unexplained_check' -e 'fn link_action' engine/crates/core/src/sign_in.rs
grep -c 'drop(end_sign_in_wait_at(' engine/crates/shell/src/server.rs
grep -c 'a cancelled sign-in leaves it in progress until a restart ends it' engine/CONFORMANCE.md
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `214 0`, `0` (order 214's Record is written, so 214 has merged), `4`, `4`,
`1`, `1` and `0`.

## Scope

Change `engine/crates/core/src/`, `engine/crates/shell/src/server_tests.rs`,
`engine/crates/shell/src/server.rs` only for decision 6's seam, the DL18 row of
`engine/CONFORMANCE.md` and any of its rows that a decision makes false,
the one sentence of `engine/README.md` that decision 1 names,
`engine/STANDING.md` only by `python3 tools/promise_standing.py --write` where `--check` asks for
it, `conformance/conventions/requests/keyed-hook-dispatch/case.yaml` and any case that decision 7
declares, and the new `docs/work-orders/215-declared.yaml`. Change nothing under `spec/`,
`packages/`, `experiments/` or `tools/`, nor `engine/Cargo.lock` or any `Cargo.toml`. Anything else
you find that needs changing goes in your report.

## Done when

Each test of decisions 1 to 5 failed on the unchanged engine, and you report its failing line;
decision 6's test passed there, and its failing line is the mutation run's that follows.
Then copy `server.rs` aside (`cp engine/crates/shell/src/server.rs engine/target/server.rs.fixed`),
delete from `call_tool` the statement `drop(end_sign_in_wait_at(...));`, run decision 6's test
(1 failed), restore with `cp` (not `mv`, so that cargo rebuilds), check with `cmp`, and run it
again (1 passed). Then, each passes:

```sh
(cd engine && cargo fmt --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd engine && cargo test --release -p conduit-core <name>)        # once per core test: 1 passed
(cd engine && cargo test --release -p conduit-shell --lib <name>) # decision 6's test: 1 passed
(cd engine && cargo run --release -p conduit-conformance -- .. requests-keyed-hook-dispatch)  # 1 cases passed
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, the vectors among it
git add -N docs/work-orders/215-declared.yaml                    # and any other new file, so the checks see it
python3 tools/promise_standing.py --check                        # Errors: 0
python3 tools/check_docs.py                                      # Errors: 0
python3 tools/check_private_paths.py                             # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/215-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; exit 0
git diff --check                                                 # prints nothing
git diff --name-only HEAD -- spec packages experiments tools engine/Cargo.lock | wc -l | tr -d ' '  # 0
git reset -q -- docs/work-orders/215-declared.yaml               # and each other new file: stage nothing
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML; where `python3` has PyYAML, it serves as
well. Your report gives each test's name, its failing line on the unchanged engine and its passing
line; decision 6's seam; the vector delta's counts; every choice you made yourself; and every
check's last line. Stage nothing. The coordinator then has a fresh, read-only Opus reader read the
diff (ruling 79), runs `tools/gate.py` and merges.

## Record

Run on 2026-11-20 through Conduit itself, the first order the dogfood's own Agents built. Root
called the store's turn tools by `conduit call`: `turn-worker-1.turn.v1` (07:28 to 07:47 PST), a
Codex Agent in its own home; `turn-verifier-1.turn.v1` (07:48 to 07:58), read-only, which found two
musts; and `turn-worker-1.turn.v1` again in the same conversation (07:59 to 08:01), which fixed
both: `engine/CONFORMANCE.md`'s keys-per-hook entry now says that a non-hook part's URL request
ends that part unknown under IFC-010 and raises no sign-in action, and the report lists
`requests-keyed-hook-dispatch` among its vectors. The results were copied back from the Agents'
directory into a trusted worktree. The run is outside A78 stage 2's registration: a dogfood probe,
as the owner chose, whose gaps the coordinator's notes (private) record.

`tools/gate.py` on the two commits: `Gate: PASS. 18 passed; not run: rust unreached.`

A fresh read of the diff after the gate (ruling 92) found decisions 1 to 7 applied, decision 8 not
triggered, and no must. It reran the core tests, `relayed_sign_in_starts_the_expiry_timer`, the
case `requests-keyed-hook-dispatch`, the vector delta (10 changes, 0 flips, 0 undeclared, 0
errors) and `promise_standing --check` (Errors: 0), and saw the expiry test fail with decision 6's
timer start removed. Its one should, S1, is applied: `sign_in::cancel_parts` records each part it
cancels in the parent's `sign_in.parts` as unknown, named for the phase (`account`, `poll`,
`sign-out` or `sign-in`), as `lifecycle::cancel` records its parts; the core suite passed (153).
The gate's run on the squash is the coordinator's at the merge.
