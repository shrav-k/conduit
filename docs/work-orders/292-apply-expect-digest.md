# Work order 292: `conduit apply --expect sha256:HEX` sends only the plan whose digest it is given, and `conduit plan` prints that digest (DEC-065)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network: its only sockets are the
loopback ports the shell's tests bind, and every crate it builds is cached. Run cargo offline; if
a crate is missing, stop and report, and fetch nothing. Run one cargo command at a time, and not
`tools/gate.py` or the whole engine suite. Never print a whole large file, case, diff or log: use
`head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`,
never print an environment variable's value, and never read or print a key, a token or any
credential, even one a test made. Run no `conduit` outside a test, no `codex` or `claude`, and
sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is
public: no absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's answer** ([DEC-065](../decisions/log.md#dec-065-2026-11-22-apply-checks-the-approved-plans-digest)):
must apply send exactly the plan approved -> "Digest check (Recommended)". `conduit apply
--expect sha256:HEX` recomputes the plan from live state and sends nothing when its digest differs
from the one given; no plan file is required
([ruling 76](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-76/README.md),
point 5, stands). This is DEC-065's CLI order, after orders 245, 247 and 248.

**Today**, read on the stack below. `Plan::digest()` (`plan/mod.rs`) is `sha256:` and the SHA-256
of `Plan::text()`; [order 247](247-conduit-apply.md)'s apply names `.conduit/plans/HEX.txt` and
each key `plan:HEX:ATTEMPT:STEP` by it. `conduit plan` prints that text and nothing else, so no
digest reaches the person. No plan has a JSON form: there is no `--output json`. `Options::parse`
(`plan/read.rs`) takes PROJECT_DIR, `--store` and [order 248](248-apply-replace-reopen-recreate.md)'s
approval table, and calls `--expect` an unexpected argument.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing repeats or adds an effect implicitly: with `--expect`, apply sends only the steps the
person read; 2, every access can be inspected: the plan read is the plan sent. None at risk:
without `--expect` nothing changes, and no promise, case or vector changes.

**Should this exist** ([rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes, as existing
practice: HTTP's conditional request, `If-Match` with the entity tag the client saw, refused with
412 when the current one differs (RFC 9110), and Terraform's refusal of a stale saved plan.
Conduit's own part is none: the digest is K3's. Not taken: a saved plan file (`terraform plan
-out`), which ruling 76 point 5 rules out; a JSON form with canonical bytes of its own (RFC 8785),
a second digest beside the one the keys use; a prompt (ruling 55, A4).

**When it runs.** Stacked on orders 247 (`wo/247-conduit-apply`) and 248
(`wo/248-replace-reopen-recreate`), which are merging now, on `main` from `d0187884` (#564). The
coordinator builds the stack; the start check stops otherwise. Through Conduit: one Codex worker
turn (aim for under 2 hours), a blind Codex verifier and Claude reader, one fix turn, the gate
(DEC-046, DEC-033). By ruling 89, each decision says what must hold; how is yours. The
coordinator's, open to the owner's strike: the digest on stderr, every line's words, status 2 and
the test names.

**Shared files.** Order 249 (`wo/249-prune-adopt-cascades`) also changes `plan/read.rs`'s options,
`plan/apply.rs` (`execute`), `plan/mod.rs`, `help.rs`, K3 and `engine/STANDING.md`; whichever
merges later rebases.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step that needs a change outside Scope (give its failing line), a file outside
Scope, or a start-check line that prints otherwise. Where this order's wording contradicts a
promise it does not change, the promise governs: follow it and list it under `own_choices`,
beside every choice no decision governs.

## The decisions

Wrap each insertion at its file's width.

1. **The digest's bytes** (K3's [kept state](../implementation/plan-and-apply/README.md#kept-state-and-privacy)).
   One digest, `Plan::digest()`: `sha256:` and the lowercase hex SHA-256 of the plan text's UTF-8
   bytes, with LF line ends and one final newline, from the line `plan` through the status line
   (`ready`, `no changes` or `stopped`). They are exactly `conduit plan`'s stdout and
   `.conduit/plans/HEX.txt`'s bytes, so `conduit plan > plan.txt; shasum -a 256 plan.txt` gives
   HEX. Result, `verify`, `not sent` and panel lines, and stderr, are outside it. Each approval
   flag given shapes it: a used one on its step and the `flags` line, an unused or ambiguous one
   as a stop line; the text itself is unchanged. `--expect` is not in it, nor any other command
   spelling. So plan and apply, given the same project, store and approval flags, print the same
   bytes.
2. **`conduit plan` prints it.** After each plan text it prints, whatever its status and on both
   of its paths (the SEC-001 stop before connecting, and the main one), `conduit plan` flushes
   stdout and writes one line to stderr:
   `conduit plan: digest sha256:HEX; conduit apply --expect sha256:HEX, given the same project, store and approval flags, sends nothing unless the plan it computes has this digest`.
   Stdout and the status are unchanged: stdout stays the hashed bytes, as the cases' `printed`
   fixes it, which a trailing stdout line would break. Where no plan is printed (a usage error, a
   version mismatch, a failed read) no digest line is. Apply prints no digest line.
3. **The operand.** `conduit apply --expect sha256:HEX` takes one operand, anywhere among the
   arguments. `Options::parse` checks it before any file or engine read. Each of these exits 2,
   prints nothing on stdout and writes this one stderr line:
   - no operand, or one starting with `-`: `conduit apply: --expect needs sha256:HEX, the digest conduit plan prints; nothing was read or sent`;
   - an operand other than `sha256:` and 64 lowercase hex digits: `conduit apply: --expect {VALUE:?} is not sha256: and 64 lowercase hex digits; nothing was read or sent`, `{VALUE:?}` being the operand as Rust's `{:?}` writes it, quotes included;
   - given twice: `conduit apply: --expect is given twice; give one digest; nothing was read or sent`;
   - given to plan: `conduit plan: --expect is apply's; conduit plan prints the digest to give it; nothing was read`.
4. **The check.** Apply computes and prints the plan as it does without the flag. On both of its
   paths, the first thing after printing, before the stopped and no-op branches, every write (the
   `recovered_update` ledger save among them) and every send, it compares `Plan::digest()` with
   the operand. Equal: everything goes on as without `--expect`; a stopped plan still stops.
   Different:
   - stdout gets `not sent 1..N` where the plan has N steps, once, and nothing where it has none;
   - stderr gets one line: `conduit apply: the plan computed now has digest sha256:NEW, not sha256:OLD as --expect gives; nothing was sent or kept; stdout holds that plan; the digest covers what it is computed from: the engine, the project files, .conduit/ and the approval flags given; no earlier plan is kept, so the lines that changed are not named;`, a space, and the tail for the plan computed now:
     - ready: `review it, then rerun this command with --expect sha256:NEW to send it`;
     - stopped: `it is stopped, so --expect sha256:NEW sends nothing either: resolve each stop line, run conduit plan with the approval flags apply will be given, and give apply the digest it prints`;
     - no changes: `it has no changes, so there is nothing to send`;
   - status 2, a stopped plan's (K3: no mutation, detected before sending); nothing is written
     under `.conduit/`, and no request is sent after the reads.

   What changed: the CLI holds only the approved plan's digest, so it prints the whole plan
   computed now and names no line. The race between apply's reads and its first send remains:
   K3's O2 stays open.
5. **Help.** Apply's row in `help.rs` gains `[--expect sha256:HEX]` after its approval flags, and
   its description "With --expect sha256:HEX, sends nothing unless the plan computed now has that
   digest." Plan's description gains "Prints the plan's digest on stderr, for apply --expect."
6. **The documents.**
   - `engine/CLI.md`: apply's usage line gains `[--expect sha256:HEX]`. A new section,
     `### Apply the plan you reviewed`, under "Plan and apply" after its opening paragraphs,
     gives decision 1's bytes and its `shasum` line, decision 2's line, decisions 3 and 4's lines
     and statuses, and that no plan file is read or needed. The status table's 2 row adds "a
     digest other than `--expect`'s". The opening sentence naming what is not yet built drops
     `conduit apply` and each flag `help.rs` gives apply, so that it does not contradict the new
     section. Apply's own future wording ("Once `conduit apply` ships, it will", "The planned
     apply command will", "prints" and "stops") becomes the present tense; a sentence the built
     apply does not keep stays, and the report names it. Orders 247 and 248 left these (order
     246, "What this leaves"); 250 only checks that none is left.
   - K3's README: in "Commands", apply's row gains `[--expect sha256:HEX]`; "There are no
     additional options." becomes "The one other option is apply's `--expect sha256:HEX`."; after
     the sentence that `conduit plan` is never required, add "`conduit apply --expect sha256:HEX`
     sends nothing unless the plan it computes has that digest, which `conduit plan` prints on
     stderr (DEC-065; the CLI reference)." with DEC-065 linked to its log entry and "the CLI
     reference" to the new section of `engine/CLI.md`. The status table's 2 row adds "a digest
     other than `--expect`'s".
   - No `spec/` change, so no CHANGES line.

## The tests

The shell's CLI tests only: a new `engine/crates/shell/tests/apply_expect.rs`, on the harness of
orders 241, 247 and 248 (`plan_apply_support`: the release binary, a serving engine, fixture
packages, the scripted filesystem). Take each digest from plan's stderr line or the refusal.
"Sends nothing" means `apply.rs`'s `non_read_requests` is as before and `.conduit/` is byte for
byte as before, absent where it was. Write the tests first, run them on the stack's code and
report each failing line.

1. `plan_prints_its_digest_on_stderr`: on case 01's input, `conduit plan` prints `recorded.text`,
   exits 0, writes nothing, and its stderr is decision 2's line naming case 01's
   `recorded.digest`. `conduit plan --replace nothing` (an unused operand: a stopped plan) exits 2
   and names the SHA-256 of its stdout; so does the SEC-001 stop of a secret literal, built as
   `plan_secret_literal_leaves_no_trace` builds it.
2. `apply_sends_the_plan_whose_digest_it_was_given`: case 01: `conduit apply --expect D`, with D
   from plan, meets every check `apply_case_01_t1` makes (through `applied_case` given the extra
   arguments), and the kept plan and the keys are named by D.
3. `apply_refuses_a_plan_the_project_changed_and_sends_nothing`: case 01: plan gives D1; then an
   edit of the project that changes the plan and still applies (yours; report it). `apply
   --expect D1` prints what `conduit plan` prints then, followed by `not sent 1..N`, writes
   decision 4's line naming D2 and D1 with the ready tail, exits 2 and sends nothing. Then `apply
   --expect D2` sends it and exits 0.
4. `apply_refuses_a_plan_the_engine_changed_and_sends_nothing`: case 01: plan gives D1; then one
   object the plan registers is registered under its name through the harness's root client
   (`Engine::call`). By row 10 of K3's difference table the plan now stops on `--adopt`. `apply
   --expect D1` refuses as in test 3 with the stopped tail and sends nothing beyond that outside
   registration; the D2 it names is the digest `conduit plan` prints afterwards.
5. `apply_expect_on_a_no_op`: case 03, on replayed state as `apply_case_03_t3_noop` runs it:
   `apply --expect` with case 03's `recorded.digest` prints `plan\nno changes\n` and exits 0;
   with case 01's `recorded.digest` it prints the same, no `not sent` line and the refusal with
   the no-changes tail, and exits 2. Neither sends or writes anything.
6. `expect_covers_the_approval_flags`: case 04, on replayed state as `golden_case_04_t4` runs it:
   `conduit plan --replace agent-a--project` gives D; `apply --expect D` without the flag
   refuses with the stopped tail (its plan stops on the flag: another digest) and sends nothing;
   `apply --replace agent-a--project --expect D` exits 0, its plan kept under D.
7. `expect_usage_errors_read_nothing`: with no engine context and an empty project directory, as
   `plan.rs` runs its flag refusals: `apply --expect`; `apply --expect` given case 01's HEX
   without `sha256:`, uppercased, and cut to 63 digits; `apply --expect D --expect D`; and `plan
   --expect D`. Each exits 2, prints nothing on stdout and exactly decision 3's line, not a
   context error.

No existing test's assertions change: without `--expect`, stdout and statuses stay as they are.
If one fails, fix the code.

## What to read

DEC-065; K3's "Commands", "Read, compare, print, keep, send", "Kept state and privacy", "Text
grammar and statuses" and O2; `engine/CLI.md`'s "Plan and apply"; the decisions of orders 247
and 248. In the shell: `plan/read.rs` (`Options`, `run`), `plan/apply.rs` (`execute`),
`plan/mod.rs` (`Plan::text`, `Plan::digest`) and `help.rs`; in its tests, `plan_apply_support/`,
`apply.rs`, `apply_replace.rs` and `plan.rs`.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor d0187884 HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
grep -c '"apply",' engine/crates/shell/src/help.rs
grep -c -- '\[--recreate OBJECT\]' engine/crates/shell/src/help.rs
grep -c 'pub fn digest' engine/crates/shell/src/plan/mod.rs
test -e engine/crates/shell/tests/apply_replace.rs; echo "248 $?"
test -e engine/crates/shell/tests/apply_expect.rs; echo "new $?"
git grep -l -e '--expect' -- engine/crates/shell engine/CLI.md docs/implementation/plan-and-apply | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
```

With 247 and 248 stacked they print `main 0`, `0`, `1`, `2`, `1`, `248 0`, `new 1`, `0` and
`engine 0`. On `main` at `d0187884` the third and fourth print `0` and the sixth `248 1`: the
stack is not built.

## Scope

Change `engine/crates/shell/src/plan/` (`read.rs`, `apply.rs`, and `mod.rs` only where decision 2
or 4 needs it), `engine/crates/shell/src/help.rs`, the new `tests/apply_expect.rs`,
`tests/plan_apply_support/` for a helper it needs, and `tests/apply.rs` only to move
`applied_case`, `non_read_requests` and the helpers they use into `plan_apply_support/cli.rs` (it
allows dead code), `applied_case` taking the extra arguments and its callers passing none;
`tests/apply_replace.rs` is unchanged, so copy `at_case`'s replay into the new test or the support
module. Also `engine/CLI.md` and K3's README (decision 6), and `engine/STANDING.md` only by
`promise_standing.py --write`. Nothing else: not `spec/`, a case, `conformance/`, the core, the
loader, `packages/`, `main.rs`, another test's assertions, a `Cargo.toml` or `Cargo.lock`.

## Done when

The first group prints otherwise on the stack today and passes after. The second are guards that
pass today and must still pass; run the vectors line once, last. Do not run the whole engine suite.

```sh
(cd engine && cargo test --offline --release -p conduit-shell --test apply_expect -- --list 2>/dev/null | grep -cE '^(plan_prints_its_digest_on_stderr|apply_sends_the_plan_whose_digest_it_was_given|apply_refuses_a_plan_the_project_changed_and_sends_nothing|apply_refuses_a_plan_the_engine_changed_and_sends_nothing|apply_expect_on_a_no_op|expect_covers_the_approval_flags|expect_usage_errors_read_nothing): test$')  # 7; today 0
(cd engine && cargo test --offline --release -p conduit-shell --test apply_expect 2>&1 | grep -c '^test result: ok\. 7 passed')  # 1; today 0
grep -c -- '--expect sha256:HEX' engine/crates/shell/src/help.rs  # 1 or more; today 0
grep -c '^### Apply the plan you reviewed$' engine/CLI.md  # 1; today 0
grep -c 'DEC-065' docs/implementation/plan-and-apply/README.md  # 1; today 0
```

```sh
export PYTHONDONTWRITEBYTECODE=1
for t in plan apply apply_replace plan_apply_cases $(test -e engine/crates/shell/tests/apply_remove.rs && echo apply_remove); do (cd engine && cargo test --offline --release -p conduit-shell --test "$t" 2>&1 | grep -cE '^test result: ok\. [0-9]+ passed; 0 failed'); done  # 1 for each
(cd engine && cargo test --offline --release -p conduit-shell --lib plan:: 2>&1 | grep -cE '^test result: ok\. [0-9]+ passed; 0 failed')  # 1
(cd engine && cargo test --offline --release -p conduit-core --lib 2>&1 | grep -c '^test result: ok\.')  # 1: the core's tests
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
n=$(git ls-files -o --exclude-standard -- engine); [ -z "$n" ] || git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/shell/src/(plan/(read|apply|mod)|help)\.rs|engine/crates/shell/tests/(apply_expect|apply|plan_apply_support/[a-z_]+)\.rs|engine/(CLI|STANDING)\.md|docs/implementation/plan-and-apply/README\.md)$' | wc -l | tr -d ' '  # 0
[ -z "$n" ] || git reset -q -- $n  # stage nothing
(cd engine && CARGO_NET_OFFLINE=true CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors 2>&1 | grep -c '^test result: ok\.')  # 1: every vector passes
```

Your report gives, per decision, what changed and where; each new test's failing line, then its
passing line; test 3's edit and test 4's registration; your own choices; and every check's last
line. Stage nothing.

## What follows

- DEC-065's Status line: the coordinator's, at the merge, naming the pull request.
- `--expect` is optional: S1's counted commands stay a bare `apply` (order 250's pack may name it).
- K3's O2, the race between apply's reads and its sends, stays open.
- The guide's flag wording ("Passing an approval flag to the shipped planner exits 2", "In the
  planned flag handling", "The planned flag handling will", "The planned cascade") belongs to the
  flags 248 and 249 build; no order's scope fixes it yet, and the coordinator places it.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, stacked
on order 248 (merged during the reads as #569).

**Worker-1's turns** (16:20 to 17:39) built DEC-065: `conduit plan` reports on stderr the digest
of its exact stdout, and `conduit apply --expect sha256:HEX` recomputes the plan from live state
and sends nothing, writing nothing, when its digest differs, with a refusal shaped by the plan it
computed (ready, stopped or no changes). The first turn stopped rightly at a file outside Scope:
`main.rs` sends any `--help` to help before parsing. The coordinator ruled, as the least change and
open to the owner's strike, that `conduit apply --expect --help` prints apply's help as any
`--help` does; the turn went on. Seven new tests failed on the stack and pass.

**Two blind reads** (the Codex verifier 17:37 to 17:42, the Claude reader 17:37 to 17:39). The Codex verifier had one must: `engine/CLI.md` said apply
reads no saved plan, but apply reads an earlier kept plan when it recovers an interrupted
replacement. The Claude reader had no must, a should and two nits. One fix turn (17:42 to 18:38)
applied all five. The gate's run on the squash is the coordinator's, at the merge.
