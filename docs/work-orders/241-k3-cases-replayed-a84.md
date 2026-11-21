# Work order 241: K3 brought up to date and replayed (A84): the plan-and-apply reference's nine cases moved to today's fixtures, and their 60 requests sent to a serving engine

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no conformance case changes. This order
needs no network: it reaches no host and fetches no crate, and the only sockets it opens are the
loopback ports its test binds. Every crate it builds is in the local cache already. Run one
cargo command at a time.
Never print a whole large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential, even one a test made. Do not run
`conduit` outside a test, `npm`, `node`, `codex`, `claude`, the dogfood harness or
`tools/gate.py`, and sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`.
Every file you write is public: no absolute local path, user or machine name, or private
repository name.

## What and why

**The gap.** Orders 069, 069b, 154 and 164 wrote K3,
[the plan-and-apply reference](../implementation/plan-and-apply/README.md), with nine golden
cases, by hand. No program produced them, and A84, which sends their requests to the engine
([FR-299](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/P03-plan-apply.md#fr-299);
[ROADMAP](../../ROADMAP.md), row A84), never ran. They are also stale:

- They pin `example.org/s1/agents` 0.2.0 at `sha256:479ef470…6c71`, order 044's archive. Orders
  048a-1, 202 and 050n rebuilt it; it is now `sha256:09e98548…4159`
  ([digests.json](../../conformance/fixtures/install-archives/digests.json)). Order 050n's `.v2`
  Types gained `directory`, `effort` and `turnDeadlineSeconds`, each with a default, and `signIn`.
- Their `.v2` configs write `apiKey: null`, an input the Types lost with order 202
  ([ruling 83](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-83/README.md));
  the schemas are closed, so a register that sends it is refused.

Orders 245 to 249 build plan and apply to these cases. This order makes them valid against
today's `spec/` and fixtures, and shows on the real engine whether their requests end as stated.

**Counts.** Root's two `operator` Conduits are in K3's T1 count already: its 19 file lines come
from experiment 023's flow states, whose t1 lines F10 and F16 are `operator.from agents#turn`,
one line each
([023, "Where each line goes"](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/023-paper-count/results-2026-11-11.md#where-each-line-goes)).
No count changes; decision 2 adds the note. Case 01 already prints both `ownTools` lines (A84's
row asks for them).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
as existing practice: golden cases replayed against the real system, as Terraform's provider
acceptance tests apply each step to a real API and check the state it leaves, with a declared
list of known differences, as `by-rule-outcomes.txt` keeps A83's. Not taken: the planner first
(245 would match cases no one has run); the cases moved into `conformance/` (they are author
expectations, not vectors).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); rule 1): serves 2 (access
can be inspected and revoked: each case's open Conduits and tombstones are read back) and 6 (no
effect repeats implicitly: each request is sent once, with its case's key). None at risk: no
promise, engine source, package or conformance case changes.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (4-hour limit; aim for 3
hours), a Codex verifier and a Claude reader, and the gate (DEC-033). It runs beside order 240
(decision 2). By
[ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md),
each decision says what must hold; how is yours. The names and decision 1's reading of "the
Types' defaults" are the coordinator's, open to the owner's strike.

**Stop rule.** Stop, change nothing further, and report where:

- the harness needs an engine source change or a crate the shell does not already depend on;
- the filesystem cannot be scripted without changing an archive's bytes;
- case 01's step 1 (the register) does not succeed, and no case change of decision 5 makes it:
  record it in the results first, with its diagnostic and promise;
- a case change needs a choice that `spec/` does not make: report the case, the choice and the
  options; do not choose (a replay mismatch that `spec/` does not decide is not a stop:
  decision 5 lists it);
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The cases on today's fixtures.** In `files`, `requests`, `engine`, `printed`, `recorded` and
   `kept` of all nine cases:
   - every pin of `example.org/s1/agents` 0.2.0 names `sha256:09e98548…4159`; every other pin
     stays, and each equals its archive's digest in `digests.json` or in the
     [S1 fixtures' table](../../conformance/fixtures/s1/README.md#computed-digests);
   - each `.v2` Agent's config in `files`, in the register documents and in each template row
     holds `account` alone: `apiKey` goes, and `accountServices`, `peerMessaging` and `model`,
     which equal their Type's defaults, are left to them, as experiment 023 counted only
     `config.account`. Each instance's `resolved` config and each inspection view shows what
     inspection gives: every default, `directory`, `effort` and `turnDeadlineSeconds` among them
     (SCH-009);
   - every derived value follows: the plan text, its digest and record path, each key
     `plan:HEX:ATTEMPT:STEP`, `kept.files`, `plans` and `journals`, the next case's `applied`,
     the `result` lines of `printed`, and the table "Case command and output counts". Derive
     them with decision 3's test, which names each wrong value and the value it computes, or with
     `shasum -a 256`; add no script.
2. **The reference's text**, `docs/implementation/plan-and-apply/README.md`. It stays "Not
   binding", and every heading stays, since other documents link to its anchors.
   - The three clauses "as for S1's `apiKey`" go; the rule they illustrate (a permitted null is
     not a reference) stays.
   - T1's agents install line in "S1 commands and counts" gives the new digest in full.
   - Where O7, O8 and "Commands" give the engine context, or how the CLI learns the engine's
     version, to order 052a, they name order 243. The safety-case section stays as it is.
   - Below the table of "S1 commands and counts under ruling 76", a note that holds the words
     "root's two `operator` Conduits": T1's 19 file lines include them, one flow line each
     (SCT-009), listed apart as root's Conduits
     ([the measurement plan](../implementation/first-build/measurement-plan.md), "Grants made by
     hand"); the cases leave them out; written inline or by reference, each is one line (which
     form is the owner's to choose, for order 250). No count in the table changes.
   - "No program has produced these expected plans" stays, followed by a sentence that order
     241 sent their requests to the engine, linking experiment 029's results. "Fixture agents
     0.2.0 raise none" changes if decision 5 shows a sign-in.
   - The closing paragraph gains one sentence: what this order re-derived, and that
     `plan_apply_cases_are_consistent` checks it.
   - If the start check's `DEC-008` line prints `0` (order 240 merged), each reading of ruling
     76 points 2 to 5 that `spec/README.md` now states cites its section "What the short form's
     meaning covers" in place of the ruling, and O4's last two sentences become one: `spec/`
     now matches CON-008 (order 240). If it prints `1`, those lines stay.
3. **The consistency test**, `plan_apply_cases_are_consistent`, in a new
   `engine/crates/shell/tests/plan_apply_cases.rs`; it runs no engine. Read each case with the
   loader's YAML reader (`boundary::value`); where its profile refuses a case, the case's YAML
   changes to fit, with no change of meaning. For each case it checks:
   - `recorded.digest` is the SHA-256 of `recorded.text`, and `recorded.path` is
     `.conduit/plans/HEX.txt` or null; `printed` starts with `recorded.text`, and each of its
     `result N OUTCOME JSON` lines is step N's `result` as its safe projection in sorted-key
     compact JSON (the README's "Read, compare, print, keep, send", step 6);
   - each request's key is `plan:HEX:ATTEMPT:STEP`, with STEP its place;
   - `kept.files` holds the SHA-256 of each file of a case that keeps state;
   - in chain order (01 to 07, 09, then 08), 01's `applied` is null and each later `applied`
     equals the previous `kept`;
   - each pin equals its archive's digest, as decision 1 says;
   - "Case command and output counts" agrees with the cases: requests, plan lines, stdout lines.

   On today's cases it fails on the agents pin.
4. **The replay**, `plan_apply_cases_replay_in_chain_order`, in the same file, with the harness
   in a new `engine/crates/shell/tests/plan_apply_support/mod.rs` that 245 to 249 reuse: reading
   a case, the served engine, the scripted filesystem, the map, the projections and the
   comparison.
   - **The engine.** A `Shell` on a temporary store, first started, served by `server::serve` on a
     loopback port, its `ServingExecutor` around the test's executor. Each request goes to the
     MCP endpoint as root, its key in `_meta` under `io.github.shrav-k.conduit/idempotencyKey`
     and Conduit's extension declared, as `tests/s2.rs`'s `call` sends one (IFC-023, IFC-036).
   - **Packages.** Before each case, the installs of K3's schedule (T1, T2 and T5 in "S1
     commands and counts"), by `conduit install`'s route: `management.install.v1` sent as root
     with `boundary::install_arguments(ARCHIVE#HEX)` (`src/author.rs`, as `tests/by_rule.rs`'s
     `install` sends it), from the archives under `conformance/fixtures/`, unchanged. The
     installed packages then equal the case's `engine.packages`. An install the engine refuses
     is one mismatch line `CASE install - PROMISE`, logged with its diagnostic; the replay goes on.
   - **No npm, no client.** The filesystem implementation's runs and calls are answered by a
     test `Executor`, as `tests/changes.rs`'s `Observed` is one (not `tests/agents.rs`'s
     `scripted_filesystem`, which rewrites a snapshot's implementation item). Every other
     dispatch is answered as `MissingExecutor` answers it. No process runs `npm` or `node`.
   - **Order.** The cases in chain order, each case's requests in order. Every request is sent,
     whatever earlier results were, except one whose arguments need an ID or name that no earlier
     result or read gave: it is not sent, and counts as a mismatch.
   - **The map.** The cases' IDs, generated names (`root-pool-abc01`), invocation IDs and
     revisions (per object) are symbols. Each is bound one to one at its first appearance: the
     README's base symbols (system 001 to root's Conduits 007) by kind and name from `list.v1`
     after first start, the rest from the result that returns them. A symbol bound to two values,
     or two symbols to one, is a mismatch.
   - **Compared.** Each value a result states equals the engine's, through the map; a member the
     case omits is not compared. `view` and `tombstones` are compared in the README's compact
     notation ("Case format and sequence"), projected from `inspect.v1` and `list.v1`. After each
     case, read as root through `list.v1` and `inspect.v1`, the objects (kind, name, ID, state,
     held pins), each open Conduit's operations and the tombstones equal its `kept`'s.
   - **Known mismatches.** Each is one line of a new
     `engine/crates/shell/tests/expected/plan-apply-mismatches.txt`: `CASE STEP POINTER PROMISE`,
     with POINTER a JSON Pointer into the case (`/requests/3/result/outcome`, `/kept/tombstones`)
     and PROMISE the deciding promise, or `-`. The test fails on a mismatch not listed and on a
     listed one that does not occur. Head the file as `by-rule-outcomes.txt` is headed: a line
     changes only in the pull request that changes what the engine does or what the case states.
5. **Each mismatch settled.** For each mismatch of the first replay, name the promise that
   decides. Where `spec/` says the case is wrong, change the case and its derived values. Where
   the engine departs from `spec/`, or `spec/` does not decide, keep the case, list the line,
   and log it in the results with both values. Change no engine source. The coordinator expects
   one, which the replay settles: by PND-005 and the signIn rule in `promises/contracts.yaml`
   ("A string at the account place is the expected account"), set-up of a `.v2` fixture Agent,
   whose account-hook has no implementation, raises a sign-in action. If so, case 01's set-up
   results carry it, and `printed` gains `panel action ID: sign-in required` lines (ruling 76
   point 4).
6. **Experiment 029 and the register.**
   - `experiments/029-a84-plan-apply/README.md`, written before the replay first runs: A84's
     question; what runs; the pass (A84's "Done when"); the measures, fixed: per case, sent,
     matched, and the first mismatch with its promise, taken on the first replay after
     decision 1 and again on the last; and the limits: the scripted filesystem, fixture Agents
     with no implementation, one machine, the commit named.
   - `results-YYYY-MM-DD.md` (the run's date, Pacific time): the commit, the command, the two
     tables, each case change the replay prompted with its promise, and each logged mismatch.
   - A row in `experiments/README.md`.
   - ROADMAP's row A84: "Needs" drops "order 051a merged"; "Feeds" becomes orders 245 to 249;
     "Done when" drops "case 01's missing `ownTools` line is fixed first"; "State" becomes
     `done on YYYY-MM-DD:` with the two matched counts out of 60 and a link to the results.

## What to read

- The reference's sections from "Read, compare, print, keep, send" to "S1 commands and counts";
  the nine cases, by key with `sed -n`.
- `conformance/fixtures/install-archives/`, `conformance/fixtures/s1/README.md`, and the `.v2`
  Types in `conformance/fixtures/s1/packages/agents-0.2.0/contracts.json`.
- `spec/`: PND-005, the signIn rule in `promises/contracts.yaml`, SCH-009, SCT-009, IFC-023,
  IFC-036, OPS-008, LIF-013, LIF-037, LIF-040, ACC-033, FLEET-005.
- In `engine/crates/shell/`: the tests named in decision 4, `by_rule.rs`, `common/mod.rs`;
  `src/boundary.rs`, `src/server.rs`, `src/lib.rs` (`Executor`), `src/author.rs` ("install").
- FR-299 with its verifier's counterargument (`git show
  077fa47e:docs/proposals/full-review/findings/P03-plan-apply.md`, lines 279-308); ROADMAP's
  row A84 and "Proposing an experiment".

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
git grep -l 479ef470 -- docs/implementation/plan-and-apply | wc -l | tr -d ' '
git grep -l apiKey -- docs/implementation/plan-and-apply | wc -l | tr -d ' '
grep -c '"agents-0.2.0.tar.gz": "sha256:09e98548' conformance/fixtures/install-archives/digests.json
cat docs/implementation/plan-and-apply/cases/*.yaml | grep -c '^- tool:'
test -e engine/crates/shell/tests/plan_apply_cases.rs; echo "test $?"
test -e experiments/029-a84-plan-apply; echo "029 $?"
grep -A8 '^## DEC-008' docs/decisions/log.md | grep -c 'accepted, not done' | sed 's/^/DEC-008 /'
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `10`, `10`, `1`, `60`, `test 1`, `029 1`, then `DEC-008 1` on `main` at
`cc3605a3` or `DEC-008 0` once order 240 has merged (either goes on; decision 2 says what each
means), `engine 0` and `0`.

## Scope

Change `docs/implementation/plan-and-apply/` (the README and the nine cases); add
`engine/crates/shell/tests/plan_apply_cases.rs`, `engine/crates/shell/tests/plan_apply_support/`
and `engine/crates/shell/tests/expected/plan-apply-mismatches.txt`; add
`experiments/029-a84-plan-apply/`; and change `experiments/README.md` and ROADMAP's row A84.
Change nothing else: not `spec/`, `conformance/`, `packages/`, `tools/`, `IN_FORCE.md`,
`docs/implementation/HANDOVER.md`, any other engine file, `Cargo.toml` or `Cargo.lock`.

## Done when

Write decision 6's README first, then decision 3's and 4's tests, and run them on the unchanged
cases: report each failing line. Then make decisions 1, 2, 5 and 6 until all pass. Each line
marked "fails today" prints otherwise on `main` at `cc3605a3`; all pass after. If the replay's
`kept` comparison does not pass by hour 3, compare only each request's outcome and `list.v1`'s
open Conduits and tombstones (FR-299's cheaper stage 1), and log per case what is left
uncompared.

```sh
(cd engine && cargo test --release -p conduit-shell --test plan_apply_cases 2>&1 | grep -c '^test result: ok\. 2 passed')  # 1; fails today (0)
git grep -l -e 479ef470 -e apiKey -- docs/implementation/plan-and-apply | wc -l | tr -d ' '  # 0; fails today (10)
git grep -l 09e985481081192fc23ffd518273c21245765afcb8db5a6ad1b467c156664159 -- docs/implementation/plan-and-apply/cases | wc -l | tr -d ' '  # 9; fails today (0)
grep -c "root's two \`operator\` Conduits" docs/implementation/plan-and-apply/README.md  # 1; fails today (0)
ls experiments/029-a84-plan-apply/README.md experiments/029-a84-plan-apply/results-2026-*.md 2>/dev/null | wc -l | tr -d ' '  # 2; fails today (0)
grep -c '029-a84-plan-apply' experiments/README.md  # 1 or more; fails today (0)
grep -c 'after order 051a' ROADMAP.md  # 0; fails today (1)
test -f engine/crates/shell/tests/expected/plan-apply-mismatches.txt; echo $?  # 0; fails today (1)
```

These pass, as on `main`; run the shell's whole suite once, last:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_roadmap.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(docs/implementation/plan-and-apply/|engine/crates/shell/tests/(plan_apply_cases\.rs|plan_apply_support/|expected/plan-apply-mismatches\.txt)|experiments/(029-a84-plan-apply/|README\.md)|ROADMAP\.md)' | wc -l | tr -d ' '  # 0
(cd engine && cargo test --release -p conduit-shell)  # every test passes
```

Your report gives: decision 6's measures, per case, on the first and the last replay; each case
change with its promise, and each logged mismatch; the failing line of the replay with one
case's stated outcome changed, then undone; how the filesystem was scripted; your own choices; and
every check's last line.

Stage nothing.

## What follows

Named here and not in this order:

- The reference's 18 safety cases and the convergence check that decision D26 of the full review
  made 052a's done-when: no order of the chain holds them yet.
- A change to a case alone runs no engine row in the gate; the gate's engine paths (order 238)
  should take the cases' directory.
- Root's `operator` Conduits in the cases, and the real Agent packages' contracts: order 250.
- Each mismatch logged against the engine: an engine order, which removes its line.
- If order 240 merges after the start check, decision 2's last bullet is applied at rebase.

## Record

Run on 2026-11-21 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-2's first turn** (10:57 to 13:09) brought the nine cases up to today's fixtures and
passed the consistency test. It then stopped at case 01, as the stop rule says:
- the contracts-only agents fixture offers `turn.v1` with no implementation, so registration
  fails under TPL-004;
- its first replay sent 10 of 60 requests and matched none.

Root's decision, open to the owner's strike, was to keep K3 on its fixture and give the fixture's
Agents a scripted implementation. A new agents 0.2.1 fixture selects one, and the harness
supplies its command. K3 does not move to the real Agent packages: that is order 250's, and it
touches the owner's open item 1c.

**Worker-2's second turn** (13:10 to 13:47) sent all 60 requests to a serving engine. All 60
outcomes and all nine kept-state comparisons match. 51 replies match every stated field. Nine
reply-field differences are logged as engine gaps for order 252:
- under IFC-025, setup replies leave out `pendingAction`;
- under NAM-008, unused-package notes are missing.

The changed-outcome check failed as expected. The full shell suite passed.

**`turn-verifier-2.turn.v1`** (13:47 to 13:52) confirmed the replay and the nine differences. Its
one must was that embedded pins went unchecked. Worker-2 fixed it, with a regression, and also
applied the verifier's should and two nits (13:52 to 13:57).

**The Claude reader** (13:58 to 14:05) approved with two shoulds and two nits. Root applied them:
- the reference and case 01 now say that the 0.2.1 fixture's Agent command is the harness's;
- the fixture README dates the 0.2.1 row and says how its archive was built.

Two of the order's "fails today" lines now print other values, because of root's fixture
decision and the added wrong-pin test:
- `test result: ok. 2 passed` gives 0, since the file now has three tests;
- the old agents digest `09e98548...` is in no case, since the cases pin 0.2.1.

The gate's run on the squash is the coordinator's, at the merge.
