# Work order 258: the engine's login Resource: one sign-in for its Agents, turns in parallel, one at a time after a lapse

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; list under
`vectors` the six cases you add, with the promises each tests. It needs no network: run cargo
offline; if a crate is missing, stop and report, and fetch nothing. Run one cargo command at a time.
Never print a whole large file, case, diff or log. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential, even one a test made. Do not run `conduit` outside a test, `codex`, `claude`, the
dogfood harness or `tools/gate.py`, and sign in nowhere; no test starts a real client: scripted
clients and fake values only. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you
write is public: no absolute local path, user or machine name, or private repository name.

## What and why

**The owner's answers.** [DEC-042](../decisions/log.md#dec-042-2026-11-21-model-logins-are-pool-owned-resources):
"Pool-owned login Resource (Recommended)". [DEC-046](../decisions/log.md#dec-046-2026-11-21-the-fleet-grows-on-every-account-by-measurement):
"One login per account, shared": turns in parallel, a refresh failure that fails a turn raising
one sign-in action, the login then taking one turn at a time until lifted. Order 257 wrote them into
the spec; this order builds them.

**Today.** The engine knows only an instance's own login (`sign_in.rs`, `sign_in_holds`).
`lifecycle::bindings` reads a Resource reference at a run pointer as a secret. Notices are
PND-007's alone. PRV-020 covers `runtime` alone. A run gets no state directory but its own.

**The form a plain turn reports a lapse in** (decided here with order 259). PND-005 names two: a
keyed result, failed or unknown with an error diagnostic naming PND-005, and a plain
implementation's URL request (IFC-016). The `.v2` turn stays plain "in 0.2.0 and after it"
(IN_FORCE.md topic 9) and PKG-020's `keyed` lists hooks alone, so a keyed turn needs the owner's
word. The least form is the one the engine already reads, a URL request, with one IFC-016 clause
(decision 9). 259's packages send exactly this `tools/call` result, only for a lapsed login, never
a usage limit:

```json
{"resultType":"input_required","inputRequests":{"sign-in":{"method":"elicitation/create","params":{"mode":"url","message":"<fixed: the client, its login lapsed>","url":"<fixed https page: the vendor's sign-in help>"}}}}
```

No other member, account, token, path or login URL; the turn starts no login (a Codex re-login
revokes the shared one); the item stays plain. The engine ends the turn unknown, relays neither
value (IFC-016) and acts as decisions 5 and 6 say.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): serves 3,
one login per account per machine, a Pool's, its path passed, never a copy; 6, a held call reports
unavailable, never queued. At risk: 2, the login reaches an Agent only by the engine's own runner,
and PRV-020 keeps the Resource, so its logout, until its Agents are gone; 5, only root answers the
notice and signs R in, as PND-010 keeps Resources root's. Each refusal says what to send next and
who sends it, naming only what the caller may inspect (ruling 52).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: OpenAI's CI page has one `auth.json` serve a "serialized workflow
stream", here only after a lapse (DEC-046); MCP's URL-mode elicitation asks a person to act out
of band, as for an authorization. Not taken: a keyed turn (topic 9); a diagnostic read from a
plain result, a second plain form; a lease per account per turn (DEC-046); queuing (test 6).

**When it runs.** Order 257 must have merged: the start check stops otherwise. Through Conduit's
dogfood: one Codex worker turn (a 4-hour limit), a Codex verifier and a Claude reader, blind and at
once, one fix turn, the gate. Hot files: with 255, core `requests.rs`, `lifecycle.rs`, `sign_in.rs`,
`explain.rs`, `packages.rs` and `engine/CONFORMANCE.md` (255's DL10 and DL2, this DL16 and DL19);
with 262, `requests.rs` (262 `complete`, this at most its lapse arms) and CONFORMANCE (DL8); with
261, `spec/CHANGES.md`, `implementation.yaml`, the requests `index.yaml`; with 245 to 249, only
`engine/STANDING.md`. The later to merge rebases and reruns `promise_standing.py --write`. How is
yours (ruling 89). The form, texts, case IDs, notice evidence, naming a running call only to a
caller who may inspect its Agent, and failing setup off R's node are the coordinator's, open to the
owner's strike.

**Stop rule.** Stop, change nothing further, and report where 257's text and this order differ,
or a result needs a choice neither makes (give both, with options); a result needs a change
outside the scope; `vector_delta.py` shows a change beyond the six added cases, or a test decision
10 does not name fails (name it); or the start check prints otherwise. A
choice no decision governs is yours: list it under `own_choices`.

## The decisions

R is a login Resource, A and B Agents of it; `{R}` is `Resource/<name> (<ID>)`.

1. **`loginResource`** (CTR-013). Beside `packages.rs`'s `toolsDirectory` check: `place` leads to
   a required member of schema `resource-reference.v1` that no listed control's `changes` holds,
   compared as for the account pointer; else a CTR-013 error names the Type, `place` and why.
   The family and `signIn` rules are `type-spec.v1`'s (SUB-005). No `account-hook.v1`,
   `sign-in-hook.v1` or `sign-out-hook.v1` goes to an Agent of R.
2. **At submission** (CTR-013, MAN-017). A login place referring to a Resource whose Type states
   no `signIn`, with no string at its account pointer, or retired, is rejected with a CTR-013
   error naming the place and that Resource. MAN-017 accepts an item's `environment` pointer to
   the login place; in `packageArguments` or `headers` it stays rejected.
3. **The Pool authorizes** (ACC-033). Registering, under row 1A, 2 or 3, an Agent whose login
   place refers to R also needs the actor's Conduit to R's owning Pool listing the row's
   operation, unless that Pool is in the submission; otherwise it reports refused.
4. **The directory reaches each run** (PRV-029). For a variable an item's `environment` binds to
   the login place, `lifecycle::run_inputs` gives the run intent R's ID (one way:
   `inputs.loginDirectories`), never a secret to read; no record holds a path. `runner.rs`'s
   `start` sets it to R's `state_directory`, as it sets the state variable, never a helper's. Where
   the Agent or R names a `runtime`, or their `placement` differs, the Agent's setup fails before
   its run is asked for, with an error naming PRV-029 and `{R}` and saying that only the engine's
   own runner passes a login Resource's directory, to a run on its node.
5. **The login's hold is R's** (PND-005, PND-011, IFC-016).
   - A call of A reporting a missing or lapsed login, in either form, ends as today (a URL request:
     unknown, its URL and message in no record or reply) and raises R's sign-in action unless one is
     unanswered, its evidence naming A's call. It holds neither R nor any Agent of R. The cause,
     under today's trigger's rule: "The implementation reports a missing or lapsed login" (keyed) or
     "The implementation asked for a URL elicitation outside sign-in.v1, which reports a missing or
     lapsed login" (URL), then ", for the Agent's login Resource {R}, whose sign-in action is
     pending and whose Agents are not held; if its login lapsed, root's sign-in.v1 for that Resource
     signs in all its Agents." For a null account, today's not-set cause, "the instance's expected
     account" read "the expected account of the Agent's login Resource {R}", and no action or
     notice.
   - While R itself is held under PND-005 (by its own triggers, never an Agent's report), or R's
     `SignInUndone` is False, an operation that starts model work on an Agent of R reports
     unavailable, not dispatched, R's unanswered action, if any, in `pendingAction`: "{tool} reports
     unavailable under PND-005: the Agent's login Resource {R} awaits sign-in to its expected
     account, so the request was not dispatched; send it again once root's sign-in.v1 for that
     Resource has signed it in." PND-011's text names R likewise.
   - Root's `sign-in.v1` for R, succeeding, answers the action and lifts the hold of all.
6. **The fallback** (PND-005, PND-009; DEC-046).
   - Where decision 5's call started model work, one notice for R is also raised, unless one is
     unanswered: subjects R and A; evidence A, the call's request and invocation; expiry under
     DL19. The result and `invocation-ended` record name R's action, then the notice, in
     `pendingAction`; the cause gains " Until root answers notice <ID> acknowledged, the login
     takes one call that starts model work at a time."
   - While R's newest such notice is not answered acknowledged, an operation that starts model work
     on an Agent of R, while one on any of them is in progress, reports unavailable, the notice in
     `pendingAction`: "{tool} reports unavailable under PND-005: a call on the login Resource {R}
     reported its login lapsed, so its Agents' calls that start model work run one at a time, and
     <running> is in progress; the request was not dispatched. Send it again once that call ends.
     Root lifts this by answering notice <ID> acknowledged." <running> is "invocation <ID> of
     Agent/<name>" where the caller may inspect that Agent (PND-002), else "a call of an Agent of
     it". With R's hold too, both are reported, the hold's first.
   - An expired notice lifts nothing: the next call held back raises a new one. Only root answers
     it, through R's owning Pool (`management` where retired); another caller it is raised at is
     refused under PND-009, told only root answers it. Root's `acknowledged` lifts the rule. One
     way: the notice's pending entry holds `login`, R's ID, and no `fleet`.
7. **`sign-in.v1` for A** (PND-010) reports invalid after access, before any part: "sign-in.v1
   reports invalid under PND-010: the Agent signs in through its login Resource {R}; root sends
   sign-in.v1 for that Resource, which signs in all its Agents."
8. **Retiring R** (PRV-020, LIF-021). `retirement::preflight` reports conflict under PRV-020
   while an Agent of R exists, retired or not, naming each as it names an instance that names a
   runtime provider; so `retire-with-resources.v1` reports R's part conflict, holding the Pool back.
9. **IFC-016** (`spec/interface/implementation.yaml`, wrapped at its width): "raises PND-005 only
   for a Type stating signIn with a string account, and reaches" becomes "raises PND-005 only for
   a Type stating signIn with a string account, or for the login Resource of an Agent where that
   Resource has one (CTR-013, PND-005), and reaches", and the next "Without that declaration"
   becomes "Without either". `spec/CHANGES.md`, under `## [Unreleased]`, after its last item:

   ```text
   - Changed: IFC-016, a URL request outside sign-in.v1 from an Agent of a login Resource raises PND-005 for that Resource, the form in which a plain turn reports a lapsed login (DEC-042, DEC-046).
   ```

10. **Tests.** Write them first, run them on the unchanged code and report each failing line. In
    core: `login_resource_type_rules` (decisions 1, 2); `login_resource_run_binding` (4, its
    failures among it); `login_resource_lapse_and_hold` (A's keyed lapse: one action, on R, B not
    held; a second lapse adding none; R's setup finding no account, then R's `SignInUndone`, each
    holding A and B; root's sign-in lifting both; 7; no hook to A); `login_resource_plain_url_lapse`
    (the result above, from a plain item); `login_resource_one_call_at_a_time` (6: with the hold,
    both <running> forms, expiry, a Fleet Interface's refusal, root's lift, no notice for a call
    that starts no model work); `login_resource_retirement_conflict` (8, alone and in the cascade).
    In `runner.rs`, `login_directory_reaches_each_agent_run`: A and B each get R's directory in the
    bound variable and their own state directory, a helper neither; it starts no process.
11. **Six cases**, each with its `index.yaml` entry and failing on the unchanged engine, declared
    in `docs/work-orders/258-declared.yaml` in the form of [242's](242-declared.yaml):
    - `contracts-login-resource-misplaced` (CTR-013), `language/contracts/`, as
      `contracts-own-tools-control`: a place not required, not a `resource-reference.v1`, or held
      by a control's `changes`, each rejected; a valid one accepted.
    - `registration-login-place` (CTR-013, MAN-017), `language/registration/`, as
      `registration-runtime-retired`: bound in `environment`, it registers; in
      `packageArguments`, rejected (MAN-017); referring to a Resource with no `signIn`, a null
      account, or retired, rejected (CTR-013).
    - `access-login-resource-pool` (ACC-033), `conventions/access/`: an Agent of R registered
      into a live Fleet via `management` and the Fleet alone is refused; with R's Pool, accepted.
    - `requests-login-resource-lapse` (PND-005, PND-010), `conventions/requests/`, from
      `requests-sign-in-lapse-and-conflict`: R's setup finds no account; A's and B's turns report
      unavailable naming R's action; `sign-in.v1` for A, invalid naming R; root's for R succeeds on
      its account check; A's keyed turn reports a lapse, raising R's action; B's turn is dispatched.
    - `requests-login-resource-one-at-a-time` (PND-005, PND-009, IFC-016), beside it: A's plain turn
      sends the URL request above, ending unknown, naming the action and notice; B's turn runs,
      unheld, and A's meanwhile reports unavailable naming the notice, dispatched once B's ends;
      after the expiry a held call names a new notice; the Fleet Interface's answer is refused;
      after root's, both run at once.
    - `fleets-retire-login-resource` (PRV-020, LIF-021), `conventions/fleets/e3b/`: `retire.v1`
      of R and the Pool's `retire-with-resources.v1` each report R's conflict naming A; once A is
      retired and unregistered, the cascade retires both.
12. **`engine/CONFORMANCE.md`.** The row "DL16: instance state directories" ends "From order 258, a
    login Resource's directory also reaches its Agents' runs (PRV-029).", row DL19 "From order 258,
    an Agent's lapse raises its login Resource's sign-in action, holding no Agent, and its Agents
    then start model work one call at a time until root answers its notice; only the Resource's own
    hold holds them all (PND-005, PND-009)." Keep `From order 258,` on one line. Run
    `promise_standing.py --write`.

## What to read

257's promises, IFC-016, PND-002 and PND-007; `runner.rs`'s `start` and its tests.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 48d4f36e HEAD; echo "main $?"
for d in 042 046; do sed -n "/^## DEC-$d/,/^## DEC-/p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/contracts.yaml 'states signIn or loginResource.'  # from order 257
j spec/promises/providers.yaml 'no other run but this:'
j spec/promises/manifest.yaml 'the login place of that type'
j spec/promises/pending.yaml "An Agent's report holds no instance"
j spec/interface/implementation.yaml 'raises PND-005 only for a Type stating signIn with a string account, and reaches no consumer inline'
git grep -l -e loginResource -e login-resource -e login-place -- engine/crates conformance | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, two lines `1`, five lines `1`, `0`, `engine 0` and `0`.

## Scope

Change `engine/crates/core/src/` for decisions 1 to 8 and tests; the shell's `runner.rs` alone;
`spec/interface/implementation.yaml` and `spec/CHANGES.md` (decision 9); each new case's
`index.yaml`; `engine/CONFORMANCE.md` (decision 12); `engine/STANDING.md` by the tool. Create only
the six case directories and `docs/work-orders/258-declared.yaml`. Change nothing else: not
another promise, case or shell source, the loader, `engine/crates/conformance/`, `packages/`,
`tools/`, `docs/decisions/`, `IN_FORCE.md` or a Cargo file. Report anything else.

## Done when

The first group prints otherwise on `main` today and passes after; the second, guards, passes
today and must still pass; run its last cargo line, the whole engine suite, once, last.

```sh
ok='^test result: ok\. 1 passed'
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
for t in login_resource_type_rules login_resource_run_binding login_resource_lapse_and_hold login_resource_plain_url_lapse login_resource_one_call_at_a_time login_resource_retirement_conflict; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # six lines 1; today 0
(cd engine && cargo test --offline --release -p conduit-shell --lib login_directory_reaches_each_agent_run 2>&1 | grep -c "$ok")  # 1; today 0
for p in contracts-login-resource-misplaced registration-login-place access-login-resource-pool requests-login-resource- fleets-retire-login-resource; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # 1, 1, 1, 2 and 1 cases passed; today FAIL no cases matched the requested prefix
j spec/interface/implementation.yaml 'or for the login Resource of an Agent where that Resource has one'  # 1; today 0
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- Changed: IFC-016, .*(DEC-042, DEC-046)\.$'  # 1; today 0
grep -c 'From order 258,' engine/CONFORMANCE.md  # 2; today 0
test -f docs/work-orders/258-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
new=$(git status --short -uall | sed -n 's/^?? //p'); git add -N $new  # so the checks see them
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_model_account.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/258-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(interface/implementation\.yaml|CHANGES\.md)|engine/crates/(core/src/[a-z_]+|shell/src/runner)\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/.*(/index\.yaml|login-(resource|place)[^/]*/.+)|docs/work-orders/258-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- $new  # stage nothing
```

Report, per decision, what changed and where; each new test's and case's failing line, then its
passing line; and every check's last line. Stage nothing.

## What follows

- Order 259: the packages, sending the form above; each client's lapse detection is 259's.
- A keyed turn, which could end a lapse failed, needs the owner's word (IN_FORCE.md topic 9).
- From 257: whether R's `sign-in.v1` holds its Agents while a re-sign-in has deleted the login.
- A login on another node than its Agent's waits with the carried return (A81).

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with order 257 stacked as one commit while 257 waited in the merge queue. It follows 257
as built, which the coordinator amended during its build (open to the owner's strike): the
IFC-016 clause this order's decision 9 carried moved into 257; PRV-007 has the runner give an
Agent's login-place variable the Resource's directory; IFC-025 lets a lapse report name the
Resource's unanswered action and notice; a login notice is answered through the Resource's Pool;
the reporting Agent is told it may resend.

**Worker-1's first turn** (01:21 to 01:34) stopped before any behaviour change: decision 11's
ACC-033 access case needs an actor that may register an Agent without the login Resource's
Pool's grant, which no public request can reach, since only root registers an Agent (ACC-057)
and root always holds every Pool's register Conduit (FLEET-005, ACC-024). The coordinator
decided that the refusal is tested by a seeded core unit test and the portable case is dropped.

**Worker-1's second turn** (01:35 to 02:17) built the order, tests first; eight focused tests
and five portable cases pass. Its whole-suite run failed only while the machine's disk was full.

**Two blind reads at once** (02:20 to 02:25), the owner's answer for A77's window:
`turn-verifier-1.turn.v1` found two musts and two shoulds; the Claude reader passed it with
three nits. One fix turn (02:25 to 02:34), each must reproduced first:
- a retirement could prepare a hook run that passed the login directory to an Agent that setup
  had refused under PRV-029; the same check now guards every run that receives it;
- the sign-in action's evidence named the reporting Agent without attribution, so an Interface
  that may inspect the login Pool saw facts about an Agent it may not; lapse evidence is now the
  Agent's and account evidence the Resource's;
- the notice cites the triggering request's record; the runner's regression now exercises the
  Resource's resolution and the helper's environment; a held call's message names its rule
  once; a PRV-029 setup failure names the condition that failed.

178 core tests and eight runner tests pass. From this order on, workers leave the whole suite to
the gate. The gate's run on the squash is the coordinator's, at the merge.
