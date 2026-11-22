# Work order 252: two gaps order 241's replay found: a setup's result names the sign-in action it raised (IFC-025), and register and replace report unused packages (NAM-008)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no conformance case changes. This order
needs no network: it reaches no host and fetches no crate, and the only sockets it opens are the
loopback ports its tests bind. Every crate it builds is in the local cache already: run cargo
offline, and if a build asks for a crate that is not there, stop and report, and fetch nothing.
Run one cargo command at a time. Never print a whole large file, case, diff or log: use `head`,
`tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential,
even one a test made. Do not run `conduit` outside a test, `npm`, `node`, `codex`, `claude`, the
dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user or
machine name, or private repository name.

## What and why

**The findings.** Order [241](241-k3-cases-replayed-a84.md) sent the 60 requests of the nine
plan-and-apply cases to a serving engine. All 60 outcomes matched; nine replies differed in a
field of a successful result, each now a line of
`engine/crates/shell/tests/expected/plan-apply-mismatches.txt`, with both values in
`experiments/029-a84-plan-apply/results-2026-11-21.md`:

1. Case 01 steps 9 and 11, `s1.set-up.v1` of the two `.v2` Agents: each setup raised a sign-in
   action, which inspection listed (PND-005, TPL-012, PND-002), but the setup's result held no
   `io.github.shrav-k.conduit/pendingAction`.
2. Seven successful replies, `register.v1` at 02 step 1, 04 step 6, 05 step 5, 06 step 5 and 08
   step 2, and `replace.v1` at 05 step 1 and 06 step 1: each manifest lists
   `example.org/s1/agents`, and from case 04 also `example.org/s1/history`, which no document of
   the request refers to; the cases expect a NAM-008 note per address, and `diagnostics` was
   `[]`.

**What `spec/` settles.**

1. IFC-025: "io.github.shrav-k.conduit/pendingAction names the action IDs raised or holding the
   request (PND-005, PND-007)" and "is a list of action IDs, including when it names one
   action". A setup that raised a sign-in action is a request that raised it. PND-012 has its
   record name the action, and the engine's does (`lifecycle::record_setup_sign_in`); its result
   does not. `requests::finish` adds to `_meta` only the notices of PND-007. A `sign-in.v1` whose
   first account check finds another account raises the action too (PND-010) and has the same
   gap (`sign_in::link_raised_action`); this order closes these two, and "What follows" names a
   third.
2. NAM-008: "The submission is accepted, and the unused package is reported", and "The result
   lists the unused address as a diagnostic that did not prevent registration". `replace.v1`
   resolves its template, contracts and manifest "as `register.v1` resolves a template", and
   both declare `diagnostics` among their outputs. The resolver makes the note
   (`packages.rs`, `unused satisfied package`), the registration keeps it, and the
   register-project boundary returns it (`records::registration_boundary`); but the request
   route's outputs are fixed: `json!({"objects":created,"diagnostics":[]})` for `register.v1`
   and `json!({"revision":new["revision"],"diagnostics":[]})` for `replace.v1`, in
   `records.rs`.
3. NAM-008 fixes no wording. The engine writes `project: unused satisfied package ADDRESS`; the
   cases write `Unused package ADDRESS; submission accepted.`, which
   [the plan-and-apply reference](../implementation/plan-and-apply/README.md#case-format-and-sequence)
   calls "a golden-case choice". Decision 3 takes the cases' words.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 5,
every management action is available to an agent under a grant: an Agent or an Interface that
sends a setup learns from its result, not from a further inspection, that the instance waits on a
person, and which action; 6, nothing repeats an effect implicitly: a caller told of the sign-in
action has no reason to send the setup again. None at risk: no promise or conformance case
changes, and the result gains only what its record already holds.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes: each is the least change that keeps a promise already written, by existing practice. A
response names what the request created, as HTTP's 201 names the created resource in `Location`
(RFC 9110, section 15.3.2); a build that accepts an unused dependency still reports it, as
rustc's `unused_crate_dependencies` lint, where enabled, warns and still builds. Not taken: the
harness reading the action from inspection (IFC-025 puts it in the result); the cases dropping
their notes (NAM-008 asks for them); the cases taking the engine's words, since `project:` is the
resolver's internal scope label and the cases' words say what happened (ruling 52).

**When it runs.** Order 241 must have merged first: the start check stops otherwise. Through
Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for under 2 hours), a Codex verifier
and a Claude reader, who run only what settles a doubt, and the gate, which runs everything
(DEC-030, DEC-033). By ruling 89, each decision says what must hold; how is yours. The test names,
the wording of decision 3 and the reading that takes in `sign-in.v1` are the coordinator's, open
to the owner's strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- a promise's wording needs a choice that `spec/` does not make: report the promise, the choice
  and the options; do not choose;
- a correct result needs a change outside the scope: a shell source, the loader, the replay
  harness (`plan_apply_support/`), a case, `conformance/`, a `Cargo.toml` or a lockfile;
- after decisions 1 to 3, the replay shows a difference that is not among the nine lines, or one
  of the nine remains: report the case, step, pointer and both values; change no case;
- `vector_delta.py` shows a change, or a vector fails: report the case and step with the expected
  and the observed reply;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

### A setup's result names the sign-in action it raised

1. **The raising request names the action** (IFC-025, PND-005, PND-010, PND-012). Where an
   invocation of `set-up.v1` (its account check, TPL-012) or of `sign-in.v1` (its first account
   check, PND-010) raised a sign-in action, the result the engine gives when it ends, in
   `t.reply` and in its `request_replies` entry alike, holds
   `io.github.shrav-k.conduit/pendingAction`: a list that holds that action's ID, beside any
   notice IDs, the ID its `invocation-ended` record names, whatever the outcome and whichever
   route ends it. A request that raised no action gains no key, and one that found an action
   already unanswered raised none (PND-001). A held request (PND-005) and a repeat under OPS-009
   answer as today. One way: where `requests::finish` adds the notices, add the action held in
   the invocation's `setup.raised` or `sign_in.raised`.

### Register and replace report unused packages

2. **The results carry the registration's notes** (NAM-008, IFC-025). A `register.v1` or
   `replace.v1` request that succeeds holds in `structuredContent.diagnostics` the diagnostics its
   registration reported, none of them an error, in the order reported, in place of `[]`, as
   `registration_boundary` returns them. Among them is one NAM-008 note for each address that the
   request's manifest lists, installed at its version and digest, and that no document of the
   request's project refers to. A repeat under OPS-009 returns the outputs kept, notes among
   them. A rejected request reports as today.
3. **The note's words** (NAM-008; ruling 52). The note for the request's own manifest has
   `ruleId` `NAM-008`, `level` `note` and `message.text` exactly
   `Unused package ADDRESS; submission accepted.`, where ADDRESS is the package's address. A note
   that the resolver makes for another scope keeps its text.

### Tests and documents

4. **Tests.** Write these first and run them on the unchanged code; report each failing line.
   - `setup_result_names_the_sign_in_action_it_raised`, in `conduit-core`'s unit tests (the
     module is yours): a `set-up.v1` of an instance whose Type states `signIn` with a string
     account, whose `account-hook.v1` part reports no account, raises the sign-in action and ends
     with `pendingAction` `[ID]` in `t.reply` and in its `request_replies` entry, ID being the one
     its `invocation-ended` record names; a setup whose account check finds the expected account
     ends with no such key. Today the key is absent.
   - `sign_in_result_names_the_action_it_raised`, in `sign_in.rs`'s tests, on
     `account_part_fixture` as `sign_in_cancel_and_restart_link_the_action_raised` is: the first
     account check reports another account and raises the action; when `sign-in.v1` then ends
     because its `sign-in-hook.v1` part ends failed, as in
     `sign_in_explains_an_uncertain_first_check_before_relay`, its result holds `pendingAction`
     `[ID]`. Today absent.
   - `register_and_replace_results_report_unused_packages`, in `conduit-core`'s unit tests: a
     `register.v1` sent as a request, and a `replace.v1`, each with a manifest that lists one
     installed package that no document uses and one that a document uses, succeed with
     `diagnostics` holding exactly one item: `ruleId` `NAM-008`, `level` `note`, `message.text`
     `Unused package ADDRESS; submission accepted.` for the unused one; with both used,
     `diagnostics` is `[]`. Today `[]` in each case.
   - The replay: delete the nine lines of `plan-apply-mismatches.txt` and keep its header; on the
     unchanged code, `plan_apply_cases_replay_in_chain_order` fails, naming the nine; report that
     line. After decisions 1 to 3 it passes.
   - An existing test changes only where it asserts what decisions 1 to 3 change: an empty
     `diagnostics` from a request whose manifest lists a package no document uses, a setup's or a
     sign-in's `_meta` compared whole, or the old note text. Change no other test, and nothing in
     `plan_apply_support/` or the cases.
5. **The documents.**
   - `experiments/029-a84-plan-apply/results-2026-11-21.md` gains a last section,
     `## After order 252`, which gives the replay's date, the commit, the command, and the
     per-case table of the "Last replay" section, sent and matched, for this run: 60 of 60 sent,
     60 of 60 matched. The earlier sections stay as they are.
   - ROADMAP's row A84: its "State" cell gains one sentence holding `60/60 after order 252`, with
     a link to that section.
   - `engine/CONFORMANCE.md`: at the end of the DL19 row, "From order 252, a `set-up.v1` or
     `sign-in.v1` that raises the sign-in action names it in the `pendingAction` key of its
     result, as its ending record does."; and in the section "I4-01: operations for changing
     S1", after the sentence that ends "an intervening change remains a core conflict.", "From
     order 252, a `register.v1` or `replace.v1` that succeeds returns its NAM-008 notes in
     `diagnostics`." Keep `From order 252,` on one line in each.
   - Run `python3 tools/promise_standing.py --write` where an engine source names a promise ID
     it did not.

## What to read

- IFC-025 and its observable, in `spec/interface/implementation.yaml`; PND-001, PND-002, PND-005,
  PND-007, PND-010 and PND-012 in `spec/promises/pending.yaml`; NAM-008 in
  `spec/promises/naming.yaml`; `register.v1` and `replace.v1` in `spec/core/operations.yaml`.
- In `engine/crates/core/src/`: `requests.rs` (`finish`, `ended_reply`), `lifecycle.rs`
  (`raise_setup_sign_in`, `record_setup_sign_in`, `settle`), `sign_in.rs` (the raise after
  `FirstCheck::Other`, `end_sign_in`, `link_raised_action` and its tests), `records.rs` (the
  `register.v1` and `replace.v1` dispatch, `replacement`, `registration_boundary`),
  `packages.rs` (the NAM-008 note) and `registration.rs` (`prepare`'s success reply).
- `engine/crates/shell/tests/plan_apply_cases.rs`, the mismatch list, and case 01's steps 9 and
  11 and case 05's steps 1 and 5, by key with `sed -n`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor fa1a7cad HEAD; echo "main $?"
m=engine/crates/shell/tests/expected/plan-apply-mismatches.txt
test -f engine/crates/shell/tests/plan_apply_cases.rs && test -f "$m"; echo "241 $?"
grep -cvE '^(#|$)' "$m"
grep -cE '^01-t1 (9|11) /requests/(8|10)/result/pendingAction IFC-025$' "$m"
grep -cE '^0[2-8]-[a-z0-9-]+ [0-9]+ /requests/[0-9]+/result/diagnostics NAM-008$' "$m"
grep -cF 'outputs = json!({"objects":created,"diagnostics":[]});' engine/crates/core/src/records.rs
grep -cF 'outputs = json!({"revision":new["revision"],"diagnostics":[]});' engine/crates/core/src/records.rs
grep -c 'unused satisfied package' engine/crates/core/src/packages.rs
grep -c 'From order 252,' engine/CONFORMANCE.md
git grep -lE 'fn (setup_result_names_the_sign_in_action_it_raised|sign_in_result_names_the_action_it_raised|register_and_replace_results_report_unused_packages)\(' -- engine | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `241 0`, `9`, `2`, `7`, three lines `1`, two lines `0`, `engine 0` and
`0`. `241 1` means order 241 has not merged: stop. Other counts in the mismatch list mean 241
changed it before merging: stop and report the list.

## Scope

Change, for decisions 1 to 3 and their tests, `engine/crates/core/src/requests.rs`,
`lifecycle.rs`, `sign_in.rs`, `records.rs` and `packages.rs`, and the test modules of other
`engine/crates/core/src/` files and of `engine/crates/shell/tests/` only as decision 4 allows;
`engine/crates/shell/tests/expected/plan-apply-mismatches.txt`, by removing the nine lines;
`experiments/029-a84-plan-apply/results-2026-11-21.md`, ROADMAP's row A84 and
`engine/CONFORMANCE.md`, as decision 5 says; and `engine/STANDING.md` only by the tool. Change
nothing else: not `spec/`, `conformance/`, `docs/`, `packages/`, `tools/`, a shell or loader
source, `plan_apply_support/`, `Cargo.toml` or `Cargo.lock`. Put anything else that needs
changing in your report.

## Done when

Each line of the first group prints otherwise on `main` once order 241 has merged ("today") and
passes after. The second group are guards, which pass today and must still pass; the replay among
them passes today only because the nine lines are listed, and must pass with none. Run the last
line, the whole engine suite, once, last.

```sh
ok='^test result: ok\. 1 passed'
(cd engine && cargo test --offline --release -p conduit-core --lib setup_result_names_the_sign_in_action_it_raised 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-core --lib sign_in_result_names_the_action_it_raised 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-core --lib register_and_replace_results_report_unused_packages 2>&1 | grep -c "$ok")  # 1; today 0
grep -cvE '^(#|$)' engine/crates/shell/tests/expected/plan-apply-mismatches.txt  # 0; today 9
grep -c 'From order 252,' engine/CONFORMANCE.md  # 2; today 0
grep -c '^## After order 252' experiments/029-a84-plan-apply/results-2026-11-21.md  # 1; today 0
grep -c '60/60 after order 252' ROADMAP.md  # 1; today 0
```

```sh
(cd engine && cargo test --offline --release -p conduit-shell --test plan_apply_cases 2>&1 | grep -c '^test result: ok\. 3 passed')  # 1, with the list emptied
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_roadmap.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/[a-z_]+\.rs|engine/crates/shell/tests/([a-z_]+\.rs|expected/plan-apply-mismatches\.txt)|engine/(CONFORMANCE|STANDING)\.md|experiments/029-a84-plan-apply/results-2026-11-21\.md|ROADMAP\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Your report gives, per decision, what changed and where; each new test's failing line on the
unchanged code, then its passing line; the replay's failing line on the unchanged code with the
list emptied, then its passing line; each existing test you changed, and why; your own choices;
and every check's last line.

Stage nothing. The verifier's turn follows; then the gate, a read-only Claude reader, and the
merge.

## What follows

Named here and not in this order:

- Whether a failed setup's result names its computed setup action (PND-006), whose ID is the
  invocation's, which `_meta` names already; and whether a repeat under OPS-009 names the actions
  its first answer raised, notices among them. Both are readings of IFC-025 for a later order.
- Found while writing this order: a call whose contract reports a lapsed login, or whose
  implementation sends URL elicitation outside `sign-in.v1`, raises the sign-in action after its
  result is made (`requests::finish_login_lapse`), so its record names the action and its result
  does not; the same reading of IFC-025, for a later order.
- Found while writing this order: the resolver also makes NAM-008 notes for an installed
  package's own manifest, labelled by an internal scope (`snapshot-N`), and a registration
  returns them; whether NAM-008 reaches past the request's own project is a follow-up.
- Orders 245 to 249 build plan and apply on the cases, now with no known difference.

## Record

Run on 2026-11-21 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-2's turn** (15:24 to 15:47) closed both gaps:
- a `set-up.v1` or `sign-in.v1` that raises a sign-in action now names it in its result's
  `pendingAction`, the same ID the ending record holds;
- a successful `register.v1` or `replace.v1` now returns its registration's unused-package
  notes.

Order 241's live replay now matches all 60 requests with no mismatch. The whole engine suite
passed once.

**`turn-verifier-2.turn.v1`** (15:48 to 15:53) found one must: the legacy register-project
reply that the CLI prints had changed too. Worker-2 put it back byte for byte (15:53 to 15:57).
162 core tests, the regressions and the replay pass.

**The Claude reader** (15:58 to 16:03) passed it. Its nit, recorded here: `finish` now writes
`pendingAction` into the `invocation-ended` record itself. So a set-up ended by
`lifecycle::cancel`, which never wrote the action before, now names the action it raised. That
fits PND-012 and decision 1's "whichever route ends it". Records for set-ups and sign-ins ended
by `settle`, `recover`, `end_sign_in` or `cancel` otherwise hold what they held.

The gate's run on the squash is the coordinator's, at the merge.
