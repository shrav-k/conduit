# Work order 295: `conduit value set`, `list` and `unset`: an installation value typed at the terminal, shown as typed and saved on y, as root's value Resource (DEC-076, the CLI's first part)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty. It needs no network beyond the loopback ports its tests bind: run cargo offline;
if a crate is missing, stop and report, and fetch nothing. Run one cargo command at a time, and
not `tools/gate.py` or the whole engine suite. Never print a whole large file, case, diff or log:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential, even one a test made. Run no `conduit` outside a test, no `codex` or `claude`,
and sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write
is public: no absolute local path, user, machine or account name, or private repository name;
every value in a test is fake.

## What and why

**The owner's answer** ([DEC-076](../decisions/log.md#dec-076-2026-11-22-installation-values-are-valuev1-resources)):
the durable form of private values -> "value.v1 Resources (Recommended)"; `conduit value set`
takes a value shown as typed and saves it only on y. [Order 283](283-installation-values-as-value-resources.md)
wrote the spec, order 288 the engine's part; this order the commands, and a second the plan's.

**Today**, read on order 288's branch over `main` at `a28e7fef` (#566):

- `conduit value` is an unknown command (`main.rs`); `help.rs` has 17 rows.
- A value Resource is a document `{kind: Resource, from: CORE#value, owner: system, arguments:
  {value, node}}` only root registers (ACC-057); `inspect.v1`'s `values` maps each place a value
  filled, a JSON Pointer, to `{kind: Resource, id}` (INS-013).
- `sign_in::Person` asks on stderr and reads a line, echo on; `author::route` finds root's
  Conduit to an owner (`system` for a value); `server::client` sends as root.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing repeats an effect implicitly: `set` changes no instance, each keeping its value until it is
replaced (OBJ-010), and lifts only a value's tombstone, told before the y; 5, every management
action is available to an agent: each command is root's requests, which an agent acting as root
sends over MCP or with `conduit call`. At risk: 3, a shared thing is owned once: one live value
Resource per name, and `list` shows who holds an earlier one.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: `gh variable set NAME` takes its value at a prompt; an immutable ConfigMap
changes by delete and recreate; Kubernetes keeps a claim a pod uses from removal
(`pvc-protection`). Not taken (the coordinator's design notes): the value as an argument
(`--body`), which other processes and shell history see; input hidden while typed, which the
owner's rule forbids for values; guessing a credential by its pattern. Nor a script's standard
input, which saves with no y (DEC-076).

**When it runs.** Stacked on order 288 (`wo/288-value-engine`), which is merging, on `main` from
`a28e7fef`. Through Conduit: one Codex worker turn, the blind Codex
verifier and Claude reader at once (DEC-071), one fix turn, the gate (DEC-046, DEC-033). By ruling
89, each decision says what must hold; how is yours. The coordinator's, open to the owner's
strike: every line's words, the statuses, no script input, the unset refusal, set-up and the split.

**Shared files.** Order 292 changes `help.rs` and `engine/CLI.md`; order 248
(`wo/248-replace-reopen-recreate`) `help.rs`, `plan/read.rs` and `tests/plan.rs`; order 249
(`wo/249-prune-adopt-cascades`) those three, `lib.rs`, `main.rs` and `tests/plan_apply_support/`.
Whichever merges later rebases.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), a file outside Scope,
or a start-check line printing otherwise. Where this order's wording contradicts a promise it does
not change, the promise governs: follow it and list it under `own_choices`, beside every choice no
decision governs.

## The decisions

1. **The commands.** `conduit value set NAME [--node NODE]`, `conduit value list [--show]` and
   `conduit value unset NAME`, each taking `--store DIR` or the engine context as `conduit submit`
   does, sending only root's requests through the serving engine. A new public module `value.rs`,
   its entry point taking the arguments, a `sign_in::Person`, a terminal flag and stdout; `main.rs`
   dispatches `value` as it does `sign-in`, passing `sign_in::Terminal` and
   `std::io::IsTerminal`'s answer. The value is never an argument.
   Status 0: saved, unchanged, listed or unset; 2: a usage error, a refusal or a declined save,
   nothing sent; 1: an engine read or transport failure, or a request that did not succeed.
2. **`set` checks before it reads.** Each refusal is one stderr line, status 2, with nothing read,
   asked or sent:
   - `conduit value set: {NAME:?} is not a name.v1; nothing was read or sent`;
   - `conduit value set: root sees no node named {NODE:?}; its nodes are [N,...]; nothing was read or sent`;
   - `conduit value set: Resource/NAME exists with Type TYPE, not value.v1; choose another name; nothing was read or sent`;
   - `conduit value set: Tombstone id=T of Resource/NAME id=OLD, owned by Pool/POOL, holds the name and is not a value's; choose another name, or lift it as root with access.lift.v1 first; nothing was read or sent`,
     where `management.inspect.v1` of `{kind: Resource, id: OLD}` fails or shows another Type;
   - with standard input not a terminal: `conduit value set: reads the value from your terminal, and standard input is not a terminal; a script registers a value Resource as root with management.register.v1 (MCP or conduit call); nothing was read or sent`.
3. **At the terminal** (the owner's input rule), on stderr through `Person`, in this order:
   - told: `A value is not secret: the engine keeps it as configuration, conduit inspect and conduit value list --show show it, and conduit records exports it. A password or token goes on a secret Resource, named by a secret reference (SEC-001), never in a value.`
   - where NAME is held, told: `NAME holds "OLD" (WHERE) as Resource/NAME id=ID and fills [KIND/NAME POINTER,...]; each keeps "OLD" until it is replaced (OBJ-010)`,
     WHERE being `not tied to a node` or `tied to node N`;
   - where a value's tombstone holds the name, told: `Tombstone id=T of Resource/NAME id=OLD, retired at TIME, holds the name; saving lifts it`;
   - asked, echo on, so the value shows as typed: `Type the value of NAME, then Enter: `;
   - told: `NAME = "VALUE" (WHERE)`, and where held otherwise `, replacing "OLD" (WHERE)`, each
     string as `serde_json` writes it, so spaces and quotes show;
   - the value and node held already and set up (state `ready`): stdout `unchanged Resource/NAME id=ID: the engine holds this value and node; nothing was sent`,
     status 0, nothing asked; held but `registered`: asked `Resource/NAME id=ID holds this value and node but is not set up; set it up as root in the engine at URL? [y/N] `;
   - otherwise asked: `Save it as root in the engine at URL? [y/N] `; where held, before `[y/N]`, `This retires Resource/NAME id=OLD, lifts its tombstone and registers the new value.`; where a tombstone holds the name, `This lifts Tombstone id=T and registers the new value.`
     `y` or `yes`, in any case, saves, as `consent` reads them; any other answer or the input's end
     gives `conduit value set: not saved: the answer was not y; nothing was sent`, status 2.
   - The value is the line typed less its line end, saved as a JSON string. Each of these is status
     2, nothing sent: `conduit value set: no value was read: standard input ended; nothing was sent`;
     `conduit value set: the value is empty; nothing was sent`; `conduit value set: the value holds a control character at character N; a value is one line of text; nothing was sent`
     (U+0000 to U+001F and U+007F to U+009F; N counts from 1).
4. **Saving**, on stdout. Each request follows the one before's success and prints after its
   reply as `N TOOL SUBJECT outcome=OUTCOME`, then, where it did not succeed, the reply's JSON:
   - a new value: `management.register.v1 Resource/NAME` with the one document, then
     `system.set-up.v1 Resource/NAME`, as `conduit submit` sets up what it registers;
   - where a value's tombstone holds the name: `access.lift.v1 Tombstone/NAME` first;
   - where NAME is held: `system.retire.v1 Resource/NAME`, `management.list.v1 Tombstone/NAME` to
     find its tombstone, `access.lift.v1 Tombstone/NAME`, then the two above;
   - held with this value and node but not set up: `system.set-up.v1 Resource/NAME` alone;
   - last: `saved Resource/NAME id=ID node=N` (`node=-` where none). No stdout line holds the value.
   - After one that did not succeed: `not sent K..M` for those left, the stderr line
     `conduit value set: N TOOL did not succeed; nothing more was sent`, then status 1, the line
     ending, where `system.set-up.v1` failed, `; Resource/NAME id=ID is registered but not set up; conduit value set NAME again sets it up`;
     otherwise after a retirement, `; Resource/NAME id=OLD is retired and its new value is not registered; instances keep what they hold; conduit value set NAME again registers it`;
     otherwise after a lift, `; Tombstone id=T is lifted and Resource/NAME is not registered; conduit value set NAME again registers it`.
     Nothing is sent again or undone.
5. **`list`**: one compact sorted-key JSON line per value Resource root sees, by name and ID:
   `{"fills":[{"kind":KIND,"name":NAME,"place":POINTER}],"id":ID,"live":true,"name":NAME,"node":NODE}`,
   `node` a node's name or null, `fills` each instance, neither template nor retired, whose
   `inspect.v1` `values` names that ID, by kind, name and place. `--show` adds `"value"`; without
   it no line holds one. An ID a source map names that no live value Resource has gets a line with
   `"live":false`, and `name` and `node` as `management.inspect.v1` of `{kind: Resource, id: ID}`
   shows them, each null where that fails. With no line:
   stderr `conduit value list: root sees no value Resource and no instance filled by one`, status 0.
   An inspection that does not succeed: its reply as a JSON line, stderr
   `conduit value list: inspect.v1 of KIND/NAME did not succeed, so fills may be incomplete`, status 1.
6. **`unset`.** With no value Resource named NAME: `conduit value unset: root sees no value Resource named NAME; conduit value list names those it sees; nothing was sent`.
   With one whose ID a source map names, as `list` finds it: `conduit value unset: Resource/NAME fills [KIND/NAME POINTER,...]; each keeps its value until it is replaced or retired, so a value an instance names is not retired; retire or replace each, then unset again; nothing was sent`.
   Both status 2. Otherwise, with nothing asked, as for `conduit retire`: `system.retire.v1
   Resource/NAME`, printed as decision 4 prints it, then `unset Resource/NAME id=ID: retired; its tombstone stays until conduit value set NAME lifts it`,
   status 0.
7. **Help and the guide.** `help.rs` gains a `value` row: arguments `set NAME [--node NODE] |
   list [--show] | unset NAME`, example `value set example-account`, and the description "Keep
   installation values, such as an account, as value Resources root registers, which documents
   name under values:. set shows the value as typed, echoes it and saves it only on y, never from
   an argument; a script registers one as root. list gives each value's name, node and the
   instances it fills, the value only with --show. unset retires a value no instance names. A
   value is not secret: inspect and records show it; a password or token goes on a secret
   Resource, named by a secret reference (SEC-001)." `engine/CLI.md` gains `## Installation
   values` before `## Plan and apply`: decisions 2 to 6. No `spec/` change, so no CHANGES line.

## The tests

The shell's CLI tests only, written first and run on the stack's code, each failing line
reported, in a new `engine/crates/shell/tests/value.rs` on the plan and apply harness
(`plan_apply_support`: a serving engine, `Engine::call` as root, the release binary). Tests 1 to 4
call `value.rs`'s entry point with a scripted `Person` and standard input a terminal; tests 5 to
7 run the binary for `list`, `unset` and `plan`. "Nothing sent": the records show no request
after the reads. Each test checks its value text is on no stdout line, `--show`'s excepted.

1. `value_set_shows_the_value_echoes_it_and_saves_only_on_y`: answer `n` gives decision 3's lines
   in order and the decline, status 2, nothing sent; `y` gives decision 4's two numbered lines and
   `saved`, and `config.value` is the typed text. Input ending at the first question: the no-value
   line.
2. `value_set_replaces_by_retire_lift_and_register`: a value filling an instance's
   `/config/account`, both registered through `Engine::call`. A new value and `y` give the held
   line naming that place, the replacing echo, the longer question, five numbered lines in
   decision 4's order and `saved` with a new ID; the instance keeps the old value and its map the
   old ID, and no tombstone of the name is left. The same value again: `unchanged`, one question,
   nothing sent.
3. `value_set_refuses_before_it_reads_a_value`: decision 2's five refusals, each asking nothing,
   the fourth on the tombstone of a Resource of another Type, the fifth also through the binary
   with standard input piped; then an empty and a control-character value; each sends nothing,
   status 2.
4. `value_set_sets_up_a_registered_value_and_tells_a_failed_set_up`: a value registered through
   `Engine::call` and not set up: `set` with that value gives the not-set-up question, and `y`
   sends only `system.set-up.v1`, then `saved`; again, `unchanged`. A set-up the test makes fail
   (how is yours) gives the set-up ending, status 1, and a rerun sets it up.
5. `value_list_names_nodes_and_fills_and_values_only_with_show`: `acct`, filling an instance, and
   `home` tied to node `local`: `list` prints their two lines exactly, `--show` with `value`.
   After `acct` is replaced as in test 2, `acct`'s line has no fills and a line with
   `"live":false` and `"name":"acct"` holds the old ID.
6. `value_unset_refuses_while_an_instance_names_it`: `unset acct` is refused naming the instance
   and place, nothing sent; `unset home` prints its two lines, status 0, and `list` drops it;
   `set home` then tells the tombstone line and its question, and sends `access.lift.v1` first;
   `unset absent` gives its line, status 2.
7. `plan_is_unchanged_by_values_outside_the_project`: on case 01's input (`first_case`), after
   `set extra` saves a value, `conduit plan` prints case 01's plan as `tests/plan.rs` checks it.

`tests/cli.rs`'s `help_describes_every_command_without_store_access` gains `value`; no other
assertion changes.

## What to read

Order 283's decisions 7 to 9 and 12, order 288's 1, 4 and 5; ACC-057, OBJ-010, LIF-040, LIF-041,
ACC-053. In the shell: `main.rs`, `help.rs`, `author.rs`, `sign_in.rs`, `server.rs`, `plan/read.rs`,
`tests/plan_apply_support/` and `tests/plan.rs`.

## Before you change anything

From the repository root; stop, change nothing and report if a line prints otherwise:

```sh
git merge-base --is-ancestor a28e7fef HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
grep -c 'From order 288,' engine/CONFORMANCE.md
grep -c '^  name: value.v1$' spec/core/types.yaml
grep -c 'const COMMANDS: \[(&str, &str, &str); 17\]' engine/crates/shell/src/help.rs
grep -c '"value",' engine/crates/shell/src/help.rs
grep -c 'pub trait Person' engine/crates/shell/src/sign_in.rs
test -e engine/crates/shell/src/value.rs; echo "module $?"
test -e engine/crates/shell/tests/value.rs; echo "tests $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
```

With 288 stacked they print `main 0`, `0`, `1`, `1`, `1`, `0`, `1`, `module 1`, `tests 1` and
`engine 0`; on `main` alone the third prints `0`.

## Scope

Change, in `engine/crates/shell/`, the new `src/value.rs`, `lib.rs` for its `pub mod`, `main.rs`,
`help.rs`, and `plan/read.rs` only where test 7 needs it; `tests/value.rs`, `tests/cli.rs` for its one verb and
`tests/plan_apply_support/` for a helper; `engine/CLI.md`; `engine/STANDING.md` only by
`promise_standing.py --write`. Nothing else: not `spec/`, a case, `conformance/`, the core, the
loader, `packages/`, the planner's other files, another test's assertions or a Cargo file.

## Done when

The first group fails on the stack today and passes after; the second, guards, pass today and
after. Run the vectors line once, last; never the whole engine suite.

```sh
(cd engine && cargo test --offline --release -p conduit-shell --test value -- --list 2>/dev/null | grep -cE '^(value_set_shows_the_value_echoes_it_and_saves_only_on_y|value_set_replaces_by_retire_lift_and_register|value_set_refuses_before_it_reads_a_value|value_set_sets_up_a_registered_value_and_tells_a_failed_set_up|value_list_names_nodes_and_fills_and_values_only_with_show|value_unset_refuses_while_an_instance_names_it|plan_is_unchanged_by_values_outside_the_project): test$')  # 7; today 0
(cd engine && cargo test --offline --release -p conduit-shell --test value 2>&1 | grep -c '^test result: ok\. 7 passed')  # 1; today 0
grep -c '"value",' engine/crates/shell/src/help.rs  # 1; today 0
grep -c 'a secret reference (SEC-001)' engine/crates/shell/src/help.rs  # 1; today 0
grep -c '^## Installation values$' engine/CLI.md  # 1; today 0
```

```sh
export PYTHONDONTWRITEBYTECODE=1
for t in cli plan apply plan_apply_cases; do (cd engine && cargo test --offline --release -p conduit-shell --test "$t" 2>&1 | grep -cE '^test result: ok\. [0-9]+ passed; 0 failed'); done  # 1 for each
(cd engine && cargo test --offline --release -p conduit-core --lib 2>&1 | grep -c '^test result: ok\.')  # 1: the core's tests
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
n=$(git ls-files -o --exclude-standard -- engine); [ -z "$n" ] || git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/shell/src/(value|lib|main|help|plan/read)\.rs|engine/crates/shell/tests/(value|cli|plan_apply_support/[a-z_]+)\.rs|engine/(CLI|STANDING)\.md)$' | wc -l | tr -d ' '  # 0
[ -z "$n" ] || git reset -q -- $n  # stage nothing
(cd engine && CARGO_NET_OFFLINE=true CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors 2>&1 | grep -c '^test result: ok\.')  # 1: every vector passes
```

Report, per decision, what changed and where; each test's failing then passing line; the lines
tests saw for decisions 3, 4 and 5; your choices; each check's last line. Stage nothing.

## What follows

- **The CLI's second part** (M, stacked on this): `conduit plan` and `apply` with `values:`, from
  the coordinator's design notes; it writes its lines and, open to strike, its reading of DEC-076's
  "every stop names `conduit value set`".
- **Records redacted by value name** (DEC-076), in neither part: its design comes first, as an
  export must redact every value an instance ever held.
- A number or boolean value, once an input needs one; then order 288's second part, the dogfood
  order (DEC-068's copies retire) and DEC-076's Status line at the last merge.

## Record

Run on 2026-11-22 and 2026-11-23 through Conduit itself, in Pacific time, stacked on engine order
288 (merged since). Built in stage 2 and read and fixed in stage 3 of experiment 028; it is no
order of stage 3.

**Stage 2's worker-2** (2026-11-22, 17:33 to 17:39) stopped rightly on a contradiction: the
order lifted only "a value's" tombstone, but a removed ID cannot be inspected (LIF-035, OPS-029).
The coordinator ruled, open to the owner's strike, that `set` lifts only the tombstone of the value
it itself just retired, matched by ID, and refuses a name held only by a tombstone with the root
lift command; and, from the fresh read, that `set` reads only from a terminal (no `--file`). The
second turn (17:39 to about 18:00) built `conduit value set`, `list` and `unset`; seven CLI tests
pass.

**Two blind reads in stage 3** (2026-11-23, 10:50 to 10:53). The Codex verifier,
`root-verifier-1.turn.v1`, had one must (a Resource template holding the name skipped the
preflight) and a should; the Claude reader (`reader-1`) raised the tombstone ruling as a must,
settled by that ruling, with two shoulds and a nit. One fix turn by stage 3's worker-1 (10:55 to
11:19) applied the other five; the checks pass. The gate's run on the squash is the coordinator's,
at the merge.
