# Work order 256: a late account check changes nothing, and the sign-in freeze ends in sequence

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` the three cases you add, with the promises each
tests. This order needs no network: it reaches no host and fetches no crate. Every crate it builds
is in the local cache already: run cargo offline, and if a build asks for a crate that is not
there, stop and report, and fetch nothing. Run one cargo command at a time. Never print a whole
large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Do not run `conduit` outside a test,
`npm`, `node`, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere.
Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no
absolute local path, user or machine name, or private repository name.

## What and why

**The owner's answers** ([DEC-043](../decisions/log.md#dec-043-2026-11-21-the-sign-in-freeze-ends-in-sequence),
citing DEC-023): late account checks -> "Late answers change nothing (Recommended)", as
decisions 1 to 3 write it; the freeze's end -> "Lift in sequence (Recommended)": the late checks
and the login Resource (DEC-042) now, the carried return when a remote node's need (A81) is taken
up, the Fleet-wide sign-in command closed, and the ROADMAP row DEC-023 asked for (decision 6).

**Today.** PND-010 states only the late other account, which holds the instance (PND-011);
OPS-013 needs no change. A `sign-in.v1` whose poll is in flight at DL18 ends when that poll
answers; order 216 read that ending as the DL18 ending, and this order keeps the reading. In
`poll_answered` only `Poll::Other` reads `wait_over`: a late `Poll::Expected` ends succeeded,
answering the action and lifting the hold; a late `Poll::NoneYet` relays another input request
whose expiry has passed; and, found while writing, a late `Poll::NotKnown` ends failed, against
PND-010's "ends unknown". `part_answered` clears a False `SignInUndone` for a late poll finding the
expected account or none, though IN_FORCE.md topic 9 says such an answer "changes and clears
nothing". A `sign-in-hook.v1` part ending unknown after a successful sign-in, a note of the review,
is by design (`end_hook_wait`).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing repeats an effect implicitly: a late answer no longer turns an ended wait into a sign-in
or a prompt nobody asked for; 5, every management action is available to an agent: the caller
learns from the result what to send next. None at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: in the OAuth device grant, once the device code expires the token
endpoint answers `expired_token`, "the device authorization session has concluded", and the
client starts a new request (RFC 8628, section 3.5). Not taken: writing the built behaviour in,
which changes an ended outcome after the fact (OPS-013); leaving it unstated and unpinned.

**When it runs.** On `main` from #507, which logged DEC-043. Through Conduit's dogfood: one Codex
worker turn (a 4-hour limit; aim for under 2 hours), a Codex verifier and a Claude reader, who run
only what settles a doubt, and the gate (DEC-030, DEC-033). By ruling 89, each decision says what
must hold; how is yours. The words, names and ROADMAP row, and the readings that a late check
clears no condition and that one with a reason ends unknown, are the coordinator's, open to the
owner's strike.

**Stop rule.** Stop, change nothing further, and report where:

- a wording needs a choice that DEC-043 and `spec/` do not make: report it with the options;
- a correct result needs a change outside the scope: a shell source, `engine/crates/conformance/`,
  the loader, a package, a `Cargo.toml` or a lockfile;
- `vector_delta.py` shows a change other than the three added cases: report the case and step
  with the expected and the observed reply; change no other case;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The spec text** (DEC-043). Change no other promise: DL18's document, PND-005, PND-012 and
   OPS-013 stand.
   - PND-010's `observable`, after "...holds the instance under PND-011.", gains: "Any other later
     answer of that check changes nothing: the invocation stays unknown (OPS-013), no URL action
     is relayed, no sign-in action is raised or answered, no hold is lifted and no condition is
     cleared; where it reports the expected account, the diagnostic names PND-010 and says that
     the wait ended and that, if the login landed, a new sign-in.v1 completes on its first account
     check with no URL action."
   - PND-011's `observable`: "Only a later account-hook.v1 part within sign-in.v1 or setup that
     reports the expected account or none clears that condition;" becomes "Only a later
     account-hook.v1 part within sign-in.v1 or setup that reports the expected account or none,
     and that does not answer after DL18 ended the wait of its sign-in.v1, clears that condition; a
     check that answers after that end completes nothing either (PND-010);".
   - `sign-in.v1`'s description in `spec/core/operations.yaml`: "No held request resumes and no
     later hook runs after DL18." becomes "No held request resumes and no later hook runs after
     DL18, and a check that answers after it changes nothing but another account's hold under
     PND-011 (PND-010)."
   - `spec/CHANGES.md`, under `## [Unreleased]`:

     ```text
     - Changed: PND-010, PND-011 and `sign-in.v1`'s description: an account check that answers after DL18 ended the sign-in's wait changes nothing but another account's hold, and a late expected account's diagnostic names a new sign-in.v1 (DEC-043).
     ```

2. **A late expected account changes nothing** (PND-010, OPS-013). Where a poll of `sign-in.v1`
   answers at or after its wait's expiry (`wait_over`) with the expected account, compared
   ignoring ASCII case, `sign-in.v1` ends unknown under PND-010 with one diagnostic whose
   `message.text` ends with this cause, after the prefix `outcome_diagnostic` writes:

   ```text
   The account check answered after the sign-in's URL action's wait had ended (DL18) and found the expected account, answering no sign-in action and lifting no hold; if the login landed, a new sign-in.v1 completes on its first account check, with no URL to open.
   ```

   It sends no intent, answers no action, lifts no hold and clears no `SignInUndone`; order 252's
   `pendingAction` stays.
3. **Every other late answer changes nothing** (PND-010, PND-005, PND-011). A late poll that
   reports no account, the first check's account or no readable account, or ends failed, unknown
   or with a reason, ends `sign-in.v1` unknown under PND-010, the diagnostic's text ending with:

   ```text
   The account check answered after the sign-in's URL action's wait had ended (DL18) without the expected account, raising and answering no sign-in action, and no hook runs for it later; send sign-in.v1 again to sign in.
   ```

   It relays nothing, sends no intent, raises and answers no action, lifts no hold and clears no
   `SignInUndone`. A late other account and a poll before the expiry answer as today. One way:
   in `poll_answered`, read `wait_over` once, before the match, ending through `end_sign_in` and
   pushing `sign-in-wait-ended` as the late other arm does; in `part_answered`, skip
   `clear_sign_in_undone` for a late poll.
4. **Tests**, in `sign_in.rs`, set up as `sign_in_late_other_account_holds_instance` is, with an
   unanswered sign-in action, PND-005's hold and a False `SignInUndone` on the instance. Write
   them first, run them on the unchanged code, and report each failing line.
   - `sign_in_late_expected_account_changes_nothing`: the poll answering the expected account, in
     another ASCII case, at the expiry ends `sign-in.v1` unknown with one PND-010 cause ending in
     decision 2's text, no intent, and `pending_actions`, `sign_in_holds` and the instance's
     conditions unchanged; one second before the expiry it ends succeeded. Today: succeeded.
   - `sign_in_late_no_account_changes_nothing`: the poll answering at the expiry with `account`
     null, with the first check's account in another ASCII case, with outcome `failed`, with
     `unknown`, and with a reason (such as `unavailable`): each ends `sign-in.v1` unknown, its
     cause ending in decision 3's text, no `input_required`, no intent, and the same three
     unchanged. Today the first four leave it in progress with another input request, and the
     last ends it failed.
   - Change an existing test only where it asserts what decisions 2 and 3 change:
     `sign_in_late_other_account_holds_instance`'s late `FIRST-ACCOUNT` now ends unknown.
5. **Three cases** in `conformance/conventions/requests/`, each with its `index.yaml` entry,
   sharing steps 1 to 15 of `requests-sign-in-late-other-account` but for the first check's
   answer; the late poll answers at
   `2026-11-12T12:10:00Z`, the relay's expiry. Each fails on the unchanged engine; report the
   runner's failing line.
   - `requests-sign-in-late-expected-account` (PND-010, PND-005): the first check finds
     `first-account`, raising the action; the poll reports `PRIMARY-ACCOUNT`; `sign-in.v1` ends
     unknown with decision 2's text and no intent; the action stays pending and a request that
     starts model work is held naming it; a new `sign-in.v1` whose first check reports
     `primary-account` succeeds with no `sign-in-hook.v1` part and answers it.
   - `requests-sign-in-late-no-account` (PND-010): the first check reports `account` null,
     raising nothing; the poll reports null; `sign-in.v1` ends unknown with decision 3's text, no
     `input_required` and no intent; inspection lists no sign-in action and no `SignInUndone`; a
     request that starts model work is dispatched.
   - `requests-sign-in-late-first-account` (PND-010, PND-011): the first check finds
     `first-account`; the poll reports `FIRST-ACCOUNT`; `sign-in.v1` ends unknown with decision
     3's text and no `sign-out-hook.v1` intent; inspection lists the one action and no
     `SignInUndone`; a request that starts model work reports unavailable with PND-005's
     diagnostic and none naming PND-011.

   Declare them in `docs/work-orders/256-declared.yaml`, in the form of [242's](242-declared.yaml).
6. **The documents.**
   - `engine/CONFORMANCE.md`, the DL18 row: "The expected account still ends the invocation
     succeeded. A late poll that counts as no account yet still offers another input request with
     the original expiry, already reached." becomes "From order 256, any other late answer ends
     `sign-in.v1` unknown under PND-010 and changes nothing else, a late expected account's
     diagnostic naming a new `sign-in.v1` (`requests-sign-in-late-expected-account`,
     `requests-sign-in-late-no-account`, `requests-sign-in-late-first-account`)." Keep
     `From order 256,` on one line. Run `python3 tools/promise_standing.py --write`.
   - `IN_FORCE.md`, two lines, no bullet added. Topic 9, the **In spec/** line on ruling 91
     point 1: "and a late answer of the expected account or of none changes and clears nothing;"
     becomes "and a late answer of the expected account or of none changes and clears nothing, as
     DEC-043 confirms and order 256 writes and builds;". Topic 13, the line on orders 048a-7,
     048a-8 and 051b-3: after "and a browser login on the Mac needs no return (85 point 5).", add
     "Since DEC-043 ends DEC-023's freeze in sequence, they and order 050o wait for a remote
     node's need (A81), and the Fleet-wide sign-in command is closed (ROADMAP.md, the row
     Sign-in)."
   - `ROADMAP.md`, "The order of the work", after the row `In parallel`:

     ```text
     | Sign-in | The sign-in orders DEC-023 froze, lifted in sequence (DEC-043): the late account checks (order 256) and the Pool-owned login Resource (DEC-042) now; the carried return, orders 048a-7, 048a-8, 050o and 051b-3, once a remote node's need is taken up (A81). The Fleet-wide sign-in command is closed: accounts, not Agents, are what is signed in. Until the carried return, or the device login's F83 and 051c, a Codex Agent signs in through Conduit only on the engine's machine (IN_FORCE.md topic 9). | A81 | For the carried return, a remote node's need | now: order 256 and the login Resource; the carried return later |
     ```

   - Leave `docs/decisions/log.md`: the coordinator sets DEC-023's and DEC-043's Status at the
     merge. Experiment 012 leaves PND-010 and PND-011 to MC9, so its account stands.

## What to read

PND-005, PND-010 and PND-011; DL18; `sign-in.v1`; in `sign_in.rs`, `part_answered`,
`poll_answered`, `wait_over`, `end_sign_in`, `signed_in`, `sign_in_wait_ended` and the tests
decision 4 names; the case `requests-sign-in-late-other-account`, by step with `sed -n`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor f7b1af11 HEAD; echo "main $?"
sed -n '/^## DEC-043/,/^## DEC-044/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/pending.yaml 'than the one found before sign-in-hook.v1 holds the instance under PND-011.'
j spec/promises/pending.yaml 'Only a later account-hook.v1 part within sign-in.v1 or setup that reports the expected account or none clears that condition;'
j spec/core/operations.yaml 'No held request resumes and no later hook runs after DL18.'
j spec/promises/pending.yaml 'Any other later answer of that check changes nothing'
grep -c 'Poll::Other if wait_over' engine/crates/core/src/sign_in.rs
grep -c -e 'The expected account still ends the invocation succeeded\.' -e 'still offers another input request with the original expiry' engine/CONFORMANCE.md
grep -c -e 'changes and clears nothing;' -e 'and a browser login on the Mac needs no return (85 point 5)\.' IN_FORCE.md
grep -c -e '^| In parallel |' -e '^| Sign-in |' ROADMAP.md
ls -d conformance/conventions/requests/sign-in-late-* | wc -l | tr -d ' '
git grep -lE 'fn sign_in_late_(expected|no)_account_changes_nothing\(' -- engine | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `1`, `1`, `1`, `0`, `1`, `1`, `2`, `1`, `1`, `0`, `engine 0` and `0`.

## Scope

Change `spec/promises/pending.yaml`, `spec/core/operations.yaml` and `spec/CHANGES.md` as decision
1 says; `engine/crates/core/src/sign_in.rs`, and `requests.rs` only if decisions 2 and 3 need it;
`conformance/conventions/requests/index.yaml`; `engine/CONFORMANCE.md`, `IN_FORCE.md` and
`ROADMAP.md` as decision 6 says; and `engine/STANDING.md` only by the tool. Create only the three
case directories and `docs/work-orders/256-declared.yaml`. Change nothing else: not another case,
`docs/decisions/`, `packages/`, `tools/`, `experiments/`, a shell or loader source,
`engine/crates/conformance/`, `Cargo.toml` or `Cargo.lock`. Report anything else that needs
changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last cargo line, the whole engine suite,
once, last.

```sh
ok='^test result: ok\. 1 passed'
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
(cd engine && cargo test --offline --release -p conduit-core --lib sign_in_late_expected_account_changes_nothing 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-core --lib sign_in_late_no_account_changes_nothing 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-sign-in-late- 2>&1 | tail -1)  # 4 cases passed; today 1 cases passed
j spec/promises/pending.yaml 'Any other later answer of that check changes nothing: the invocation stays unknown (OPS-013)'  # 1; today 0
j spec/promises/pending.yaml 'clears that condition; a check that answers after that end completes nothing either (PND-010);'  # 1; today 0
j spec/core/operations.yaml "changes nothing but another account's hold under PND-011 (PND-010)."  # 1; today 0
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- Changed: PND-010, PND-011 .*(DEC-043)\.$'  # 1; today 0
grep -c 'From order 256,' engine/CONFORMANCE.md  # 1; today 0
grep -c -e 'The expected account still ends the invocation succeeded\.' -e 'still offers another input request with the original expiry' engine/CONFORMANCE.md  # 0; today 1
grep -c -e 'as DEC-043 confirms and order 256 writes and builds;' -e "Since DEC-043 ends DEC-023's freeze in sequence" IN_FORCE.md  # 2; today 0
grep -c '^| Sign-in | .*(DEC-043)' ROADMAP.md  # 1; today 0
test -f docs/work-orders/256-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n='conformance/conventions/requests/sign-in-late-'
git add -N "${n}expected-account" "${n}no-account" "${n}first-account" docs/work-orders/256-declared.yaml  # so the checks see the new files
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)
test "$(git show HEAD:IN_FORCE.md | grep -cE '^\s*[-*] ')" -eq "$(grep -cE '^\s*[-*] ' IN_FORCE.md)"; echo "bullets $?"  # bullets 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_roadmap.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_model_account.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/256-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(promises/pending|core/operations)\.yaml|spec/CHANGES\.md|engine/crates/core/src/(sign_in|requests)\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/conventions/requests/(index\.yaml|sign-in-late-(expected-account|no-account|first-account)/[^/]+)|docs/work-orders/256-declared\.yaml|IN_FORCE\.md|ROADMAP\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- "${n}expected-account" "${n}no-account" "${n}first-account" docs/work-orders/256-declared.yaml  # stage nothing
```

Report, per decision, what changed and where; each new test's and case's failing line on the
unchanged code, then its passing line; the test you changed; and every check's last line.

Stage nothing. The verifier's turn follows; then the gate, a read-only Claude reader, and the
merge.

## What follows

Named here and not in this order:

- The carried return, after A81's need; the login Resource's orders (DEC-042).
- Found while writing: a poll in flight at DL18 that never answers keeps `sign-in.v1` in
  progress, since the wait-end input ends only a sign-in waiting for a retry; whether DL18's end
  should end it then, as PND-010's words read, is a later order's.

## Record

Not yet run.
