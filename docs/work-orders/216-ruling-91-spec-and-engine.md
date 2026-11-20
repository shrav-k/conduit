# Work order 216: ruling 91 in the specification and the engine: a late answer of another account holds the instance, and the `carried: false` message names no method (written ahead)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` the case you add, with the promises it tests and
`passes` true, and each other case whose expectations change. This order needs no network: it
reaches no host and fetches no crate, and the only sockets it opens are the loopback ports that
the engine's own tests bind, as every run of the suite does. Run one cargo command at a time.
Never print a whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Do not run `conduit` against a
store, `codex`, `claude` or the dogfood harness, and sign in nowhere.

## What and why

**What it writes.** [Ruling 91](../proposals/ruling-91/README.md) holds two decisions of the owner,
and action item A88 in [the tracker](../AUDIT.md#p1--prove-the-language-can-be-rebuilt-and-its-promises-kept)
asks for both in `spec/` and the engine in one order, the spec change first:

1. **A late answer of another account holds the instance** (decision 1, the second read's S2 of
   [order 048a-5gh's Record](048a5gh-build-sign-in-undo-and-expiry.md#record)). When an
   `account-hook.v1` part of `sign-in.v1` that was in progress as DL18 ended the invocation's
   wait then reports an account other than the expected one and than the one the check before
   `sign-in-hook.v1` found, no sign-out runs, the invocation stays unknown, and the instance is
   held under PND-011's `SignInUndone` condition until a check finds the expected account or none.
   Today nothing holds it, and it could take a turn under the wrong account.
2. **The `carried: false` message names no method** (decision 2, the delta pass's m1). IFC-049's
   message for such a login, and the core `carried` description, keep "at it or over a
   connection Conduit manages" and no longer name remote desktop, SSH, pasting the code or
   forwarding the port.

The two lines of [IN_FORCE.md](../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach)
marked *Spec pending, ruling 91* say what to write; this order writes it and they stop being
spec pending. Where the ruling's record words a promise or a description (the coordinator's
reading there, open to the owner's strike), this order gives the words and you write them as
given. Where it words none, this order marks its own words as the coordinator's reading, open to
the owner's strike ([ruling 48](../proposals/ruling-48/README.md)).

**When it runs.** Written ahead, to run after the first dogfood milestone
([ruling 89](../proposals/ruling-89/README.md), point 3), not on its path, and after
[order 215](215-build-sign-in-follow-up.md): both change `engine/crates/core/src/sign_in.rs` and
`requests.rs`, so this order starts on `main` once 215 has merged, and its start check stops
unless 215's Record is on `main`. It is an order of A78's stage 2, sent through Conduit under
[order 211's registration](211-a78-stage-2-registration.md) (its section "Stage 2's orders",
and [experiment 028's](../../experiments/028-a78-dogfood/README.md#stage-2s-orders)): it changes
nothing under `experiments/` and not the harness, which stays outside the tree; its text and its
report schema are on `main` before its first turn; it runs on the Mac; and it is cut for one
worker turn and one verifier turn. None of this changes what you do. By
[ruling 89](../proposals/ruling-89/README.md), orders are less prescriptive: each decision says
what must hold and why, the Done-when checks it, and how the engine is built is yours.

## The decisions it applies

Write the specification first (decisions 1 to 3), in the terms of
[spec/definitions.md](../../spec/definitions.md). Then write the case and each engine test, and run
them on the unchanged engine: each fails there, and you report its failing line. Then change the
engine until they pass.

1. **Decision 1's promise words, as ruling 91's record words them (the coordinator's reading
   there, open to the owner's strike).** Write each as given, rewrapping lines as the file wraps
   them:
   - `spec/promises/pending.yaml`, PND-011's `observable`, after the sentence that ends "with a
     diagnostic naming PND-011 and the condition.", a new sentence: "The engine does the same
     when an account-hook.v1 part of sign-in.v1 that was in progress when DL18 ended the
     invocation's wait then reports an account other than the expected one and than the one the
     check before sign-in-hook.v1 found: no sign-out-hook.v1 part runs, the invocation stays
     unknown (OPS-013), and the condition's cause says that the check answered after the wait
     ended."
   - PND-010's `observable`: "DL18 ending without completion ends unknown and no hook runs
     later." becomes "DL18 ending without completion ends unknown and no hook runs later; an
     account check in progress then that later reports an account other than the expected one
     and than the one found before sign-in-hook.v1 holds the instance under PND-011."
   - `spec/core/operations.yaml`, `sign-in.v1`'s description: "Failed undo holds model work."
     becomes "A failed undo, or an account other than the expected one and than the one the
     first check found, reported by a check that answers after DL18, holds every request to the
     instance but the parts of sign-in.v1, of a setup and of its retirement (PND-011)." This also
     takes up the amendment that [order 171b's Record](171b-sign-in-return-and-holds.md#record)
     asked of that sentence.

   DL18's document in `spec/implementation-defined.yaml`, PND-005 and PND-012 do not change
   (the ruling's "What the late answer leaves alone").

2. **Decision 1 where the ruling's record words none (the coordinator's reading, open to the
   owner's strike).** Two stated texts say what a wrong account does and would be false or incomplete
   after decision 1:
   - `spec/definitions.md`, the row "account check", whose last sentence "A failed undo holds
     every request to the instance but the parts of sign-in.v1, of a setup and of its retirement
     (PND-005, PND-010, PND-011)." becomes "A failed undo, or an account other than the expected
     one and than the one found before sign-in-hook, reported by a check that answers after DL18
     ended its wait, holds every request to the instance but the parts of sign-in.v1, of a setup
     and of its retirement (PND-005, PND-010, PND-011)."
   - `spec/README.md`, the limits under "Pending actions, sign-in and the network": "A wrong
     account is undone, and a failed undo holds every request to that instance" becomes "A wrong
     account is undone, and a failed undo, or an account other than the expected one and the one
     first found, reported by a check after the sign-in's wait has ended, holds every request to
     that instance", the rest of the sentence unchanged.

   By the same reading, polling ends at DL18 (PND-010: "until an account appears or DL18 ends"),
   so the texts that say a polled other account is undone and the sign-in fails speak of answers
   within the wait, and they stand unchanged: PND-011's `then`, `sign-in.v1`'s description where a
   wrong account is "found then", the row's sentence on polling and the limits' opening "A wrong
   account is undone". Leave them, and say so in your report.

   Change no other promise for decision 1. If a promise outside the three of decision 1 must
   change for it to hold, or if writing these words needs a choice that ruling 91 does not make,
   stop, change nothing more and report.

3. **Decision 2's words, as ruling 91's record words them (the coordinator's reading there, open
   to the owner's strike).**
   - `spec/interface/implementation.yaml`, IFC-049's `then`: "at it or over a connection Conduit
     manages, by remote desktop or by SSH, pasting the code or forwarding the port." becomes "at
     it or over a connection Conduit manages."
   - `spec/schemas/contracts.yaml`, the core `carried` description: "at it or over a connection
     Conduit manages, by remote desktop or by SSH, pasting the code or forwarding the port, and no
     process" becomes "at it or over a connection Conduit manages, and no process".

   No promise's `when`, level or observable changes, and no case asserts the words (IFC-050). The
   limits in `spec/README.md`, "Pending actions, sign-in and the network", keep their routes
   ("by remote desktop or by SSH, pasting the code or forwarding the port"), since ruling 80,
   point 8, is not amended. No package holds a copy of the message or the description: on `main`
   at `fc1c54a6` the words are in the two `spec/` files, in `engine/crates/core/src/sign_in.rs`
   and in documents that do not bind. If you find a copy in a package, leave it, and report it
   for a package order, since changing it would need a package release.

4. **The case: a late answer of another account is held.** Add
   `conformance/conventions/requests/sign-in-late-other-account/case.yaml`, with the id
   `requests-sign-in-late-other-account`, tagging PND-011 and PND-010, and its entry in
   `conformance/conventions/requests/index.yaml`. Model it on
   `requests-sign-in-other-account-undone` and `requests-sign-in-polling`. It shows: a
   `sign-in.v1` relays its URL; a retry dispatches a poll; that poll completes, at a step whose
   time is at or past the relay's DL18 expiry, with an account other than the expected one and
   than the first check's; `sign-in.v1` has ended unknown, no `sign-out-hook.v1` intent is sent,
   and the instance's `SignInUndone` is False; a request that starts the Agent's model work then
   reports unavailable with a diagnostic naming PND-011; and a new `sign-in.v1`, whose first check
   finds the expected account, succeeds and sets the condition True. The case fails on the
   unchanged engine: report the runner's failing line. If the runner cannot express a step the
   case needs without a change to `engine/crates/conformance/`, stop and report.

5. **The engine holds the instance on that late answer (decision 1).** On `main`, a poll in
   flight when the wait's expiry passes keeps `sign-in.v1` in progress, since the shell's wait-end
   input ends only a `sign-in.v1` that waits for a retry (`waits_on_person`). When that poll
   answers another account, `poll_answered`'s arm for another account with the wait over ends
   `sign-in.v1` unknown under PND-010 and DL18, with no sign-out and no condition. That arm is
   the late answer the ruling decides. By the coordinator's reading, the engine's ending there,
   unknown under DL18, is the invocation's DL18 ending that the spec words call "stays unknown";
   this order does not move that ending to the wait-end input. What must hold:
   - the arm still ends `sign-in.v1` unknown and dispatches no `sign-out-hook.v1` part;
   - it also sets the instance's `SignInUndone` to False, with a message saying that an account
     check of `sign-in.v1` answered another account after the sign-in's wait had ended, so no
     sign-out ran, and that requests other than the parts of `sign-in.v1`, of a setup and of its
     retirement report unavailable under PND-011 until an account check finds the expected
     account or none. `spec/` fixes no reason for the condition: name one (for instance
     `CheckAfterWait`), not `SignOutFailed` or `SignOutUnknown`, since no sign-out ran, and report
     it;
   - the diagnostic of a request held under PND-011 (in `requests.rs`, today "since a sign-out
     after an account check did not succeed") is true for this hold too: it names the
     condition's own cause, or is worded so that it holds for both causes (for instance "since
     an account check found an account other than the expected one that may still be signed
     in"), so that it does not say a sign-out ran when none did (ruling 52: a refusal as
     explicit as it can be for an agent caller);
   - every request PND-011's observable holds is then held, and only a later account check
     within `sign-in.v1` or a setup that reports the expected account or none clears it, as for
     a failed undo today;
   - nothing else changes but that diagnostic: no sign-in action is raised or answered, and a
     late answer after a cancel or a restart of `sign-in.v1` stays a late completion, ignored as
     order 215 builds it.

   The test: a core test in which a poll in flight past the wait's expiry answers another
   account, asserting the ending, the absence of a `sign-out-hook.v1` intent, the condition and
   its reason, and that a request starting model work is then held with a diagnostic that does
   not say a sign-out ran; and in which the same poll answering the first check's account leaves
   no condition. On the unchanged engine it fails.

   Today the same arm's neighbours read no expiry: a poll past the expiry that answers the
   expected account completes `sign-in.v1` succeeded. The ruling's reading says that a late
   answer of the expected account or of none changes nothing, but it reads the late answer as
   coming after `sign-in.v1` has ended, which the engine does not do while a poll is in flight.
   That is not this order's to settle: leave those arms as built, and report what they do. The
   coordinator brings it back to the owner if it needs a choice.

6. **The engine writes decision 2's message.** `relay_message` in
   `engine/crates/core/src/sign_in.rs` writes the `carried: false` sentence, and the test
   `sign_in_relay_message_and_wait` quotes it. Change the test's expected words first, to "...
   complete the login on the node {node}'s machine, at it or over a connection Conduit manages."
   with the rest of the message unchanged, and run it on the unchanged engine (it fails); then
   change the message (it passes).

7. **The rechecks the ruling asks for.** Each is extended or recorded as standing, with why:
   - the limits in `spec/README.md`, "Pending actions, sign-in and the network": the sentence
     decision 2 names is extended there; the routes' sentence stands (decision 3);
   - [IN_FORCE.md](../../IN_FORCE.md), topic 9: the line "A sign-in to another account is undone
     under PND-011; the attempt fails." gains, before its period, "; one that an account check in
     progress as the wait ended reports after it is not undone, ends unknown, and holds the
     instance (91 point 1; under PND-011's SignInUndone condition, the coordinator's reading, open
     to the owner's strike)";
   - IN_FORCE.md's line for ruling 80 point 8, which cites IFC-049's message and the `carried`
     description as holding the point: it gains a sentence that, since ruling 91 point 2, the
     message and the description name the connection Conduit manages and no route, and the
     limits in spec/README.md hold the routes;
   - the two lines marked *Spec pending, ruling 91* become lines marked **In spec/**, each naming
     where its decision is held (decision 1: PND-011, PND-010, `sign-in.v1`'s description, the
     row "account check" and the limits; decision 2: IFC-049 and the `carried` description). Keep
     the owner's words and the coordinator's readings in them, still marked *Coordinator's*,
     open to the owner's strike, and drop only what said the change was still to come. Name this
     order as the one that wrote them.

   In your report, say for each recheck what you changed, or why it stands.

8. **What follows from the changes.** Update each row of `engine/CONFORMANCE.md` that the change
   makes false (the rows for PND-011 and DL18 among the ones to read); run
   `python3 tools/promise_standing.py --write` for `engine/STANDING.md`; and write
   `docs/work-orders/216-declared.yaml`, in the form of [209's](209-declared.yaml), declaring
   the added case and each other expectation that changes, so that `tools/vector_delta.py`
   reports 0 undeclared. On `main` at `fc1c54a6` no case reaches the arm of decision 5, so the
   coordinator expects the added case alone; where another case's expectations must change,
   change and declare it too, and list it first among your findings, with the reason. Do not
   touch [the tracker](../AUDIT.md) or the roadmap: the coordinator marks A88 at the merge.

## What to read

- [Ruling 91's record](../proposals/ruling-91/README.md), its section "The decisions", and the
  two lines of IN_FORCE.md topic 9 marked *Spec pending, ruling 91*.
- PND-005, PND-010, PND-011 and PND-012 in `spec/promises/pending.yaml`; IFC-049 and IFC-050 in
  `spec/interface/implementation.yaml`; DL18 in `spec/implementation-defined.yaml`; OPS-013 in
  `spec/conventions/operations.yaml`; `sign-in.v1` in `spec/core/operations.yaml`; the `carried`
  description in `spec/schemas/contracts.yaml`; the row "account check" in `spec/definitions.md`.
- `engine/crates/core/src/sign_in.rs`: `part_answered`, `poll_answered`, `poll_check`,
  `wait_over`, `waits_on_person`, `sign_in_wait_ended`, `set_sign_in_undone_false`,
  `clear_sign_in_undone`, `request_is_held_for_sign_in_undone`, `relay_message`, and the tests
  `sign_in_undone_while_polling`, `sign_in_undone_condition_and_hold`,
  `sign_in_wait_ends_unknown_at_its_expiry` and `sign_in_relay_message_and_wait` as models.
- The cases `requests-sign-in-other-account-undone` and `requests-sign-in-polling`, with
  `sed -n` or a short script, and the DL18 and PND-011 rows of `engine/CONFORMANCE.md`.

## Before you change anything

From the repository root, check that you find what this order was written against, and stop,
change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor fc1c54a6 HEAD; echo "main $?"
f=$(find docs/work-orders docs/archive/work-orders -name '215-*.md' 2>/dev/null | head -n 1); test -n "$f"; echo "215 $?"
grep -c '^Not yet run\.$' "$f"
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'DL18 ending without completion ends unknown and no hook runs later\.'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'with a diagnostic naming PND-011 and the condition\. Only a later account-hook\.v1 part'
tr -s ' \n' '  ' < spec/core/operations.yaml | grep -c 'Failed undo holds model work\.'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'at it or over a connection Conduit manages, by remote desktop or by SSH, pasting the code or forwarding the port\.'
tr -s ' \n' '  ' < spec/schemas/contracts.yaml | grep -c 'at it or over a connection Conduit manages, by remote desktop or by SSH, pasting the code or forwarding the port, and no process'
tr -s ' \n' '  ' < spec/README.md | grep -c 'A wrong account is undone, and a failed undo holds every request'
grep -c 'A failed undo holds every request to the instance but the parts of sign-in.v1, of a setup and of its retirement (PND-005, PND-010, PND-011)\.' spec/definitions.md
grep -c 'by remote desktop or by SSH' engine/crates/core/src/sign_in.rs
grep -c -e 'fn poll_answered' -e 'fn set_sign_in_undone_false' -e 'fn wait_over' engine/crates/core/src/sign_in.rs
grep -c 'Spec pending, ruling 91' IN_FORCE.md
test -e conformance/conventions/requests/sign-in-late-other-account; echo "case $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `215 0`, `0` (order 215's Record is written, so 215 has merged), `1`, `1`,
`1`, `1`, `1`, `1`, `1`, `2`, `3`, `2`, `case 1` and `0`. If order 215 has changed the arm that
decision 5 names, so that a poll answering another account past the expiry no longer ends
`sign-in.v1` unknown with no sign-out, stop and report.

## Scope

Change, under `spec/`, only the texts that decisions 1 to 3 name: PND-010's and PND-011's
`observable` in `spec/promises/pending.yaml`, `sign-in.v1`'s description in
`spec/core/operations.yaml`, IFC-049's `then` in `spec/interface/implementation.yaml`, the
`carried` description in `spec/schemas/contracts.yaml`, the row "account check" in
`spec/definitions.md` and the one sentence of `spec/README.md`. Change `engine/crates/core/src/`,
the rows of `engine/CONFORMANCE.md` that decision 8 names, `engine/STANDING.md` only by the tool,
the lines of `IN_FORCE.md` that decision 7 names, `conformance/conventions/requests/index.yaml`
and any case that decision 8 declares. Create only
`conformance/conventions/requests/sign-in-late-other-account/` (its `case.yaml`, and a fixture
file there if the case needs one) and `docs/work-orders/216-declared.yaml`. Change nothing under
`packages/`, `experiments/`, `tools/` or `engine/crates/conformance/`, nor `engine/Cargo.lock`, any
`Cargo.toml`, `docs/AUDIT.md` or `ROADMAP.md`. Anything else you find that needs changing goes in
your report.

## Done when

The case and each test of decisions 5 and 6 failed on the unchanged engine, and you report each
failing line. Then, each passes:

```sh
(cd engine && cargo fmt --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd engine && cargo test --release -p conduit-core <name>)        # once per test of decisions 5 and 6: 1 passed
(cd engine && cargo run --release -p conduit-conformance -- .. requests-sign-in-late-other-account)  # 1 cases passed
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, the vectors among it
git add -N conformance/conventions/requests/sign-in-late-other-account docs/work-orders/216-declared.yaml  # so the checks see the new files
python3 tools/check_docs.py                                      # Errors: 0
python3 tools/check_rulings.py                                   # passes
python3 tools/set_names.py --check                               # passes: no name changes
python3 tools/check_schemas.py                                   # Errors: 0
python3 tools/check_roadmap.py                                   # passes
python3 tools/check_model_account.py                             # passes
python3 tools/promise_standing.py --check                        # Errors: 0
python3 tools/check_private_paths.py                             # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/216-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; exit 0
git diff --check                                                 # prints nothing
git diff --name-only HEAD -- packages experiments tools engine/crates/conformance engine/Cargo.lock docs/AUDIT.md ROADMAP.md | wc -l | tr -d ' '  # 0
cat spec/interface/implementation.yaml spec/schemas/contracts.yaml | tr -s ' \n' '  ' | grep -c 'remote desktop'  # 0
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'at it or over a connection Conduit manages\.'  # 1
tr -s ' \n' '  ' < spec/schemas/contracts.yaml | grep -c 'at it or over a connection Conduit manages, and no process'  # 1
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'than the one the check before sign-in-hook\.v1 found: no sign-out-hook\.v1 part runs, the invocation stays unknown (OPS-013)'  # 1
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'DL18 ending without completion ends unknown and no hook runs later; an account check in progress then'  # 1
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'DL18 ending without completion ends unknown and no hook runs later\.'  # 0
tr -s ' \n' '  ' < spec/core/operations.yaml | grep -c 'A failed undo, or an account other than the expected one and than the one the first check found, reported by a check that answers after DL18'  # 1
tr -s ' \n' '  ' < spec/core/operations.yaml | grep -c 'Failed undo holds model work\.'  # 0
tr -s ' \n' '  ' < spec/definitions.md | grep -c 'A failed undo, or an account other than the expected one and than the one found before sign-in-hook, reported by a check that answers after DL18 ended its wait, holds every request'  # 1
grep -c 'A failed undo holds every request to the instance but the parts of sign-in.v1, of a setup and of its retirement (PND-005, PND-010, PND-011)\.' spec/definitions.md  # 0
tr -s ' \n' '  ' < spec/README.md | grep -c "A wrong account is undone, and a failed undo, or an account other than the expected one and the one first found, reported by a check after the sign-in's wait has ended, holds every request to that instance"  # 1
tr -s ' \n' '  ' < spec/README.md | grep -c 'A wrong account is undone, and a failed undo holds every request'  # 0
tr -s ' \n' '  ' < spec/README.md | grep -c 'by remote desktop or by SSH, pasting the code or forwarding the port'  # 1: the routes stand
grep -c 'Spec pending, ruling 91' IN_FORCE.md                    # 0
grep -c 'by remote desktop or by SSH' engine/crates/core/src/sign_in.rs  # 0
grep -c 'at it or over a connection Conduit manages\."' engine/crates/core/src/sign_in.rs  # 2: the message and its test
git reset -q -- conformance/conventions/requests/sign-in-late-other-account docs/work-orders/216-declared.yaml  # stage nothing
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML; where `python3` has PyYAML, it serves as
well. Your report gives each test's name, its failing line on the unchanged engine and its passing
line; the case's failing and passing lines; the condition's reason you chose; what the late
expected and no-account arms of decision 5 do as built; each recheck of decision 7 and what you
did; the vector delta's counts; every choice you made yourself; and every check's last line.
Stage nothing. The coordinator then has a fresh, read-only Opus reader read the diff
([ruling 79](../proposals/ruling-79/README.md)), runs `tools/gate.py` and merges.

## Record

Not yet run.
