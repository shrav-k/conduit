# Work order 249: `--prune`, `--adopt` and the cascade flags of `conduit plan` and `conduit apply`, built

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`;
leave `vectors` empty, since no case changes. This order needs no network: it reaches no host and
fetches no crate, and the only sockets it opens are the loopback ports its tests bind. Every crate it builds is in the local cache already; if a build asks for one
that is not, stop and report, and fetch nothing. Run one cargo command at a time. Never print a
whole large file, diff, log or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential. Do not run `codex`, `claude`, the dogfood harness
or `tools/gate.py`, nor `conduit` outside a test but for the `--help` line under "Done when", and
sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is
public: write no absolute local path, no user or machine name and no private repository name.

**The git rule.** On this repository run only `git merge-base`, `git log`, `git show`,
`git status`, `git diff`, `git add -N` and `git reset -q`.

## What and why

**The gap.** [spec/README.md](../../spec/README.md#what-the-short-forms-meaning-covers) names seven
flags of plan and apply. Order 247 leaves each act that needs one printed as a stop with
`requires=`; order 248 builds `--replace`, `--reopen` and `--recreate`. This order builds
`--prune`, `--adopt` and the cascade flags `--with-resources` and `--with-agents`, so that a fresh
author who reads spec/README finds no flag it names unbuilt. S1's schedule in
[K3](../implementation/plan-and-apply/README.md#s1-commands-and-counts-under-ruling-76) uses none
of the four, so no S1 count moves.
[Batch 1](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/README.md#the-work-orders)
gave the two to 052a and the cascades to 051a as commands; spec/README now names them as flags.

**What K3 settles, and what it leaves.** K3 decides these acts: rows 7, 10 and 14 to 16 of
[the difference table](../implementation/plan-and-apply/README.md#the-difference-table), `--adopt`
under [ruling 76's readings](../implementation/plan-and-apply/README.md#approval-and-command-readings-under-ruling-76),
and removal, the cascade's pattern and its cleanup under
[Replacement, removal and cascades](../implementation/plan-and-apply/README.md#replacement-removal-and-cascades).
It leaves three things, which decisions 3 to 5 settle: no printed line for an adoption; a cascade
pattern with only the one-time Conduit under `affects` and no `unregister.v1`, where rows 6 and 14
and "Print every Conduit retirement closes (CON-008), including outside the file" ask for both;
and what a plan without the cascade flag does with a one-time Conduit a failed cascade left. K3
names cases for these but writes none; this order writes shell tests.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): serves
2 and 6. Test 2: the cascade's one-time Conduit is registered, opened, used and lifted in the
printed plan; one a failed cascade leaves is closed by the next plan, never left open unseen,
which decision 5 and its test guard. Test 6: a retirement or cascade that did not finish is
retried only by a command that names its flag again (LIF-026), and adoption sends no write. No
promise or case changes.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
as existing practice: `kubectl apply --prune` removes only what an earlier apply made, as
`--prune` does from this working copy's ledger; Terraform's and Pulumi's `import` take an existing
object into state unchanged, as `--adopt` does; `kubectl delete --cascade=foreground` retires
dependents before their owner, as LIF-021 does. Not taken: a `--prune` with no operand or by label
(ruling 76 point 2); batch 1's cascade commands; an adoption that writes the file over the engine.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (4-hour limit; aim for under
2 hours), a Codex verifier and a Claude reader who run only what settles a doubt, and the gate
([DEC-030](../decisions/log.md#dec-030-2026-11-20-how-an-order-runs-and-merges),
[DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). It runs beside order 248,
which also changes the planner's flag rows and `help.rs`: 248 merges first, and this branch is
rebased onto it, keeping both orders' rows and tests. Whether the S1 rerun's pin waits for this
order is the owner's. By
[ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md),
each decision says what must hold; how is yours. The line forms and test names are the
coordinator's, open to the owner's strike.

## The decisions

1. **The operands** (ruling 76 point 2; K3's reading B). Each of the four flags is repeatable and
   names one object by its generated name. An operand that names no object, that names objects of
   more than one kind its rows act on, or that no row of this plan needs, stops the plan: status 2,
   nothing sent and nothing kept. Use the stop form orders 245 to 248 give an unused operand, or
   else one in K3's stop grammar for all four flags, listed under `own_choices`. An object that
   neither the file nor the ledger holds is never pruned, adopted or cascaded. Each flag used is
   printed once in `flags [...]` and recorded with the plan.
2. **`--prune NAME`** (rows 15, 16 and 7).
   - Row 15, an instance that is not a Pool or Fleet owning members: its retirement through the
     tool ACC-033 gives for it, class `narrows`; then, once it is removed, `unregister.v1` of its
     inline template `N--t` at its inspected revision with `expectedId`, class `removes`; each step
     with `flag=--prune NAME`. Its `affects` lines as decision 4 says, outside the file included.
   - Row 16, a template that is not inline, or `N--t` whose instance is gone: `unregister.v1` at
     its revision with `expectedId`, under `--prune NAME`.
   - Row 7, an instance this working copy retired under `--prune NAME` whose shutdown is
     unconfirmed: a further retirement (LIF-026) when the command names `--prune NAME` again, then
     the unregister; without it, row 7's stop. Never `unregister.v1` to recover (LIF-017). Where 248
     has built row 7 for `--replace`, extend that code.
   - A prune lifts no tombstone. Adding the instance back takes `--recreate` (row 11), and a
     Conduit its retirement closed takes `--reopen`.
3. **`--adopt NAME`** (row 10).
   - The plan prints one read step, `N TOOL KIND/NAME expected=ID neither flag=--adopt NAME
     OBJ-002`, TOOL being the `inspect.v1` tool ACC-033 gives, and after it `detail adopt KIND/NAME
     id=ID revision=REVISION resolved=JSON pins=JSON`: the inspected resolved definition and held
     pins in sorted-key compact JSON, each absolute path as `{"pathSha256":"HEX"}`, as K3 prints
     `ownTools`.
   - Apply sends that inspection and compares its ID, revision, resolved definition and pins with
     the printed ones. Where they are equal, the ledger takes the object as inspected, so a later
     difference from the file is the file's change (rows 21 to 26), never a control's (row 23).
     Where any differs, verification failed: status 1, `not sent N..M`, the ledger unchanged.
   - Adoption sends no write for the object. The rows below then decide it in the same plan, each
     act under its own flag, so a file that differs from the adopted object still stops with
     `requires=--replace NAME`.
   - Rows 6 and 7 come first: an engine-made object or a retired instance is never adopted.
4. **`--with-resources NAME` and `--with-agents NAME`** (row 14; LIF-020, LIF-021; ruling 59 A3;
   spec/README's cascade sentence as order 240 left it).
   - Without the flag, row 14's stop, `stop Pool/NAME absent=file requires=--with-resources NAME
     LIF-020`, or the Fleet form.
   - With it, K3's six steps in K3's form, numbered in the plan's sequence: register the one-time
     Conduit from core `retire-pool` (`retire-fleet` for a Fleet), consumer root, target the
     container, `as` the one-time name `cascade-TARGET` as K3 forms it; inspect it; open it; call
     `retire-with-resources.v1` (`retire-with-agents.v1`) through it; list its tombstone; lift it.
     Send no close: CON-008 closes and removes it within the cascade (K3's O4).
   - The cases' `affects bounds=[...] closed-conduits=[...]` line still prints, naming what the
     plan's own close steps close. After it, each retirement or cascade step S that closes
     Conduits, a pruned one included, adds `affects step=S closes=[...] CON-008`, naming in byte
     order each Conduit S closes: for a cascade, the one-time Conduit, root's FLEET-005 defaults to
     the container, and each Conduit a member is an end of, outside the file included. A cascade
     step adds `affects step=S retires=[...] LIF-021` too, naming the members.
   - Once the container and its members are removed, `unregister.v1` of each inline template of
     the container and of its members, at its inspected revision with `expectedId`, class
     `removes`, under the same flag.
   - Members take no row of their own, but rows 1 to 6 reach them: a member with an unreconciled
     invocation, or a setting reported unknown, stops the cascade (row 5).
   - The closes of dropped Conduits (row 13) come first; the one-time Conduit's registration and
     open are the one exception to narrowing before widening. Only its tombstone is lifted.
5. **A cascade that did not finish** (K3's cleanup paragraph).
   - The kept state holds the one-time Conduit's kind, name and ID, with its container and flag,
     from its registration until its tombstone is lifted.
   - A plan under the same flag cleans up first, before any new cascade: inspect by the kept ID;
     close with that ID in `expectedId` only where the observation shows the same object; list;
     lift only the tombstone whose retained object ID is the kept ID; skip what is gone. Where the
     name now names another object, stop on the identity conflict, and close and lift nothing.
   - A plan without the flag inspects and closes a surviving one-time Conduit the same way, as a
     Conduit the file does not name (ruling 76 point 1), and leaves its tombstone for the flag.
     While the container is still dropped, row 14's stop sends nothing, as every stopped plan does.
6. **The fresh author's texts.** In `help.rs`, the usage lines of plan and apply carry the four
   flags as K3's table does, and no help text says any of them is unbuilt or always stops. Where
   `engine/CLI.md` (order 246) names any of the four in its first paragraph's `not yet built`
   sentence, drop them, and drop the sentence if it names nothing else; change nothing else there.
   Remove the four from the `is not built yet` refusal of orders 245 and 247 and from the tests
   that assert it (245's `plan_refuses_unbuilt_flags`, and 247's or 248's if any), deleting such a
   test once it covers no flag.
7. **The tests**, by these names; no other test's name holds these prefixes. Run the end-to-end
   ones through a serving engine with the harness and fixtures of orders 241 and 247 (the S1
   fixture packages and the scripted filesystem, no npm), from case 02's kept state unless said.
   Where a test needs a state the fixtures cannot make, test 245's planner or 247's apply on read
   state given as values, adding the narrowest seam in the shell crate if none exists.
   - `plan_apply_prune_retires_and_unregisters`: the file drops `history` and the Conduits to it.
     Without `--prune history`: row 15's stop, exit 2, and `conduit inspect`'s objects unchanged.
     With it: exit 0, no `history` or `history--t` in the engine or the kept state, `history`'s
     tombstone kept; restoring it in the file then stops with `requires=--recreate history`.
   - `plan_apply_prune_removes_templates_and_retries`: row 16 for a dropped `templates.conduits`
     member; row 7's further retirement with the flag, and its stop without.
   - `plan_apply_prune_refuses_unused_operands`: for each flag, an operand naming no object, one no
     row needs, and one outside the project: status 2, nothing sent or kept.
   - `plan_apply_adopt_takes_over_after_verifying`: `conduit submit` registers a framed copy of a
     Resource the file holds, with its `N--t`. Without the flags, row 10's stop. With `--adopt` for
     both: only the two inspections sent, exit 0, `conduit inspect`'s objects unchanged, and the
     next plan has no step. A changed configuration in the file then stops with
     `requires=--replace`.
   - `plan_apply_adopt_stops_when_verification_differs`: an ID, a revision, a resolved definition
     and a pin that differ at the step each fail as decision 3 says.
   - `plan_apply_with_resources_cascades_a_dropped_pool`: the file drops Pool `team`, its
     Resources and the Conduits to them. Without the flag, row 14's stop, nothing sent. With it,
     K3's six lines with `cascade-team`, `affects` naming root's default to `team`; after apply no
     `team`, `shared`, `history`, inline template of theirs, `cascade-team` or tombstone of it;
     the tombstones of `team`, `shared`, `history` and the four dropped Conduits kept; both Agents
     ready.
   - `plan_apply_with_resources_stops_and_cleans_up`: row 5's stop for a member with an
     unreconciled invocation; decision 5's cleanup before a new cascade, its identity conflict, and
     its flag-free close.
   - `plan_apply_with_agents_cascades_a_dropped_fleet`: the same for `fleet: s1` and both Agents;
     after apply `team`, `shared` and `history` are ready.

## What to read

K3's sections named above, its "Ordering and access", "Stops, unknown outcomes and repeats",
"Kept state and privacy" and "Text grammar and statuses", rows 5 to 16 and O4; spec/README's
"What the short form's meaning covers"; CON-008, LIF-020, LIF-021, LIF-026, LIF-040 and LIF-041;
`retire-pool` and `retire-fleet` in `spec/core/templates.yaml`; and the shell code and tests of
orders 245, 247 and, if merged, 248.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
git log --oneline HEAD | grep -c 'Work order 247: .*built through Conduit'
git log --oneline HEAD | grep -c 'Work order 248: .*built through Conduit'
grep -c 'close the Conduit, then root lifts' spec/README.md
grep -c -- '--with-agents' spec/README.md
grep -rhoE 'fn plan_apply_(prune|adopt|with_resources|with_agents)_[a-z_]+' engine/crates/shell | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`; `1`, once order 247 has merged (on `cc3605a3` it prints `0`); `0` or `1`,
whether order 248 has merged (either goes on; with `0` the branch is rebased onto it later);
`0`, once order 240 has fixed spec/README's cascade sentence (on `cc3605a3` it prints `1`); `1`;
`0`; `engine 0`; and `0`.

## Scope

Change `engine/crates/shell/`: the planner, apply and kept-state code orders 245 and 247 added,
`src/main.rs`'s arguments, `src/help.rs`, and tests (the eight of decision 7, in the file where
order 247's end-to-end cases run or a new file beside it, and helpers in `tests/common/`). Change
`engine/STANDING.md` only as `PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --write`
rewrites it, and `engine/CLI.md` only as decision 6 says. Change nothing else: not `spec/`,
`conformance/`, K3's README or cases, `IN_FORCE.md`, the decision log, the core, the loader,
`src/server.rs` or `src/server/`, `packages/`, any `Cargo.toml` or `Cargo.lock`. Put anything
else that needs changing in your report.

## Done when

Write decision 7's tests first, run them on the unchanged code and report each failing line;
then change the code until all pass. Each line here fails on `main` at `cc3605a3` ("fails
today") and passes after.

```sh
(cd engine && cargo test --release -p conduit-shell -- plan_apply_prune_ 2>&1 | grep -cE '^test [^ ]*plan_apply_prune_[a-z_]+ \.\.\. ok$')  # 3; fails today (0)
(cd engine && cargo test --release -p conduit-shell -- plan_apply_adopt_ 2>&1 | grep -cE '^test [^ ]*plan_apply_adopt_[a-z_]+ \.\.\. ok$')  # 2; fails today (0)
(cd engine && cargo test --release -p conduit-shell -- plan_apply_with_resources_ 2>&1 | grep -cE '^test [^ ]*plan_apply_with_resources_[a-z_]+ \.\.\. ok$')  # 2; fails today (0)
(cd engine && cargo test --release -p conduit-shell -- plan_apply_with_agents_ 2>&1 | grep -cE '^test [^ ]*plan_apply_with_agents_[a-z_]+ \.\.\. ok$')  # 1; fails today (0)
(cd engine && cargo build --release -p conduit-shell --bin conduit 2>&1 | tail -1) && engine/target/release/conduit apply --help 2>&1 | grep -oE -- '--(prune|adopt|with-resources|with-agents)' | sort -u | wc -l | tr -d ' '  # 4; fails today (0)
```

These pass, as on `main`; run the engine's whole suite once, last:

```sh
grep -niE 'not (yet )?built|order 249' engine/crates/shell/src/help.rs | wc -l | tr -d ' '  # 0
grep -i 'not yet built' $(ls engine/CLI.md 2>/dev/null) | grep -cE -- '--(prune|adopt|with-resources|with-agents)'  # 0
grep -rn 'not built yet' engine/crates/shell/src | wc -l | tr -d ' '  # 0 once rebased onto 248
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy -p conduit-shell --all-targets -- -D warnings)
git add -N engine/crates/shell     # so the checks see a new test file
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py              # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py     # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check
git diff --check                                                   # prints nothing
git diff --name-only HEAD | grep -vE '^engine/crates/shell/|^engine/CLI\.md$|^engine/STANDING\.md$' | wc -l | tr -d ' '  # 0
git reset -q                                                       # stage nothing
(cd engine && cargo test --release)                                # every test passes, cases 01 to 09's among them
```

Your report gives, per decision, what you built; one cascade's plan text in full; each new
test's failing line, then its passing line; whether 248 had merged and what of it you extended;
any engine answer K3 did not expect; `own_choices`; and every check's last line.

## Stop rule

Stop, change nothing further, and report the gap and the options it leaves, without choosing,
where:

- a start-check line prints otherwise;
- the engine refuses, or answers otherwise than K3 expects, a request these decisions send, such
  as the one-time Conduit's registration from `retire-pool`, which no test sends through serve
  today: report the request, the answer and the promise;
- spec/README, as order 240 left it, and K3 disagree on a cascade step: report both;
- a decision cannot be built without changing a line of cases 01 to 09 or a test of orders 245
  to 248, other than decision 6's removal of these four flags from an unbuilt-flag test;
- a correct result needs a change outside the scope, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## What this leaves

Report this list, corrected by what you built:

- K3 still says these flags have no case and no order, and its cascade pattern lacks decision
  4's lines: for a later order (order 250's plan does not yet hold it).
- A one-time Conduit a failed cascade leaves stays open until the next plan closes it (decision
  5); closing it at once would send a request the plan did not print. That is the owner's.
- `--withhold` is batch 2's; `conduit fleet list` and `fleet inspect` are order 251's.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with orders 245, 271, 247 and 248 stacked while they waited in the merge queue.

**Worker-1's turn** (11:14 to 12:07) gave plan and apply the four remaining approval flags on 247's
planner-owned actions and 248's operand table: prune retires a dropped, owned instance through its
managing Conduit and then unregisters its inline template; adoption checks the observed ID,
revision, resolved definition and pins; the Pool and Fleet cascades run through a one-time
Conduit whose identity the kept state records, with recovery under the recorded approval. The
eight tests, which failed first, pass, with apply's and 248's suites and order 241's 60-request
replay.

**Two blind reads at once** (12:19 to 12:35): `turn-verifier-1.turn.v1` found six musts, the
Claude reader a should and two nits. One fix turn (12:36 to 13:18) applied all nine, each must
reproduced first: adoption verifies the definition it printed and grants ledger ownership only; a
cascade's approval survives an interruption around its steps; a partial cascade keeps its effects
and retries retirement; prune keeps its new tombstones in the ledger; inspection's unknown
settings, served since order 270, govern over an older unknown invocation; a stale temporary
Conduit's state is cleared. Left for a later order: a failed cascade's reply cannot establish
every member's and grant's outcome, which an exact ledger would need from the engine.

The planner test that expected inspection to omit `status.settings` is order 245's, corrected
before 245's merge. The gate's run on the squash is the coordinator's, at the merge.
