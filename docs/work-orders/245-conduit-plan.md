# Work order 245: `conduit plan`, the read-only half of plan and apply: the project, the kept state and the engine compared by K3's difference table, and K3's plan printed

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network; its only sockets are its
tests' loopback ports. Run cargo offline, one command at a time, and fetch nothing. Never print a
whole large file, diff, log or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential, even one a test made. Do not run `conduit` outside
a test, `codex`, `claude`, `npm`, the dogfood harness or `tools/gate.py`, and sign in nowhere.
Every file you write is public: write no absolute local path, no user or machine name and no
private repository name.

## What and why

**Why.** The S1 rerun measures the simplified authoring path, and today's engine has none of it:
`conduit plan` and `conduit apply` answer `unknown command`. On 2026-11-21 the owner held the
rerun ("Hold it; write the chain now"); orders 240 to 251 build the path. This order is batch 1's
052a, first part
([batch 1's plan](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/README.md#the-work-orders)):
the read-only half, so an author sees exactly what a change will do before 247's apply sends it.

**What the sources settle.** `spec/README.md`, "What the short form's meaning covers": plan and
apply are tooling composed of existing requests, the plan is printed, and every dangerous act
needs a named flag. [Ruling 76](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-76/README.md)
point 5: plan is available and never required. K3,
[the plan-and-apply reference](../implementation/plan-and-apply/README.md) as order 241 left it,
gives the read path, the difference table (rows 1 to 27), the grammar, the statuses, the kept
state's form and the golden cases. K3 does not bind; `spec/` does, where they differ.

**Corrections to the chain's plan** (the coordinator's, open to the owner's strike):
- The version stop is here, as K3's O7 gives it to the planner; 247 keeps it too.
- Row 8, and row 7's flag for this working copy's retirement, read the journal 247 writes:
  they are 247's (decision 6).
- Plan prints no digest line (K3); the digest is the SHA-256 of its text.
- Plan routes from `list.v1` and `inspect.v1`, not from `state.json` as `author::route` does
  (K3 forbids a private snapshot).
- Batch 1 made K3's 18 safety cases 052a's done-when; the ones a plan shows are here
  (decision 9), the rest are 247's and 248's.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); ruling 92, point 1): 2,
every access can be inspected and revoked: the plan prints every widening and closing apply would
send, before anything is sent; 6, nothing repeats an effect implicitly: plan sends no mutation,
writes no file, and apply never requires it. No promise or case changes.

**Should this exist** (point 3). Yes, as existing practice: `terraform plan` compares the
configuration, the kept state and the real state; `kubectl diff` and `pulumi preview` show a
change first. Not taken: a saved plan apply must be given (ruling 76 point 5); a server-side dry
run, an engine operation K3 and ruling 53 point 4 rule out.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (4 hours; the whole suite
once, at the end), a Codex verifier, a Claude reader and the gate (DEC-030, DEC-033). It is the
chain's largest order, about 1.8k to 2.8k changed lines with tests, and batch 1's estimates ran
1.5 to 2 times low. If the turn stops short, report the rows and tests done: the coordinator
moves the rest to 247 or to a finishing order
([ruling 87](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-87/README.md),
point 1). By ruling 89, each decision says what must hold; how is yours. The module path and
test names are the coordinator's, open to the owner's strike.

**Stop rule.** Change nothing under `spec/`, `conformance/` or `docs/`, K3 and its cases among
them. Stop, change nothing further, and report where:

- K3 and `spec/` disagree, or two cases, or a case and K3's prose, conflict on a line
  (decision 8): report the row, the case and the options; do not choose;
- a case's expected text would have to change to pass (none is regenerated from the planner's
  output);
- a correct result needs an engine operation, a change of the core's behavior, the loader, the
  store's files, or more of 243's context or cache than decisions 2 and 3 read;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The command.** `conduit plan [PROJECT_DIR] [--store DIR]`. PROJECT_DIR defaults to the
   current directory; the engine is order 243's current context unless `--store` overrides it
   (ruling 76 point 3). Plan sends only `list.v1` and `inspect.v1`, through serve, as root, and
   writes no file: no `.conduit/`, nothing in the package cache. Its `help.rs` row, in 243's form,
   has K3's description ("Read the project and engine as root, and print the requests apply would
   send.") and no approval flag. Plan still recognizes each of the seven
   (`--reopen`, `--recreate`, `--prune`, `--replace`, `--adopt`, `--with-resources`,
   `--with-agents`): given one, it sends nothing, exits 2 and writes one stderr line,
   `conduit plan: --FLAG is not built yet; plan without it prints the stop that needs it`.
2. **The version stop** (K3 O7). Before its first request, plan compares its own version with
   the serving engine's. With 243's context: the context's `version` against plan's own
   `conduit --version` line, byte for byte. With `--store`: `serverInfo.version` from
   `initialize` against the crate version, which catches only a crate-version change (the Record
   says so). Where they differ, it sends nothing, exits 2 and names both on stderr.
3. **The file's value** (K3, "Read, compare, print, keep, send", step 1).
   - The project is read as `submit` reads it, through `boundary::project` and the loader, so
     order 242 translates its sectioned documents. A loader rejection exits 2, its diagnostics on
     stderr, nothing on stdout.
   - Releases come from 243's package cache by digest: each package the manifest pins, through
     every dependency the archives' manifests list (the core package aside, NAM-009), and each
     release an object holds that row 19 compares. Bytes are used only where their SHA-256 is the
     digest; the core's contents are its own build's. Plan never fetches and never reads package
     contents from the engine's store.
   - Resolution is the core's own code (`conduit_core::packages::Catalog`), not a second
     resolver. An existing core function may be made public, unchanged, such as an Agent's
     `ownTools` (CTR-014), and install's steps from archive bytes to a `Snapshot` may move,
     unchanged, into one public core function that `install` calls.
   - Every place `secret-reference.v1` governs is checked during resolution, before any step is
     printed (row 4; K3 O5).
4. **The ledger's value.** Plan reads `.conduit/state.yaml` and never writes it. Its form is K3's
   format `1`, the form of the cases' `applied` and `kept`: keys `format`, `files`, `objects`,
   `tombstones`, `plans` and `journals`, each object a row `[kind, name, id, revision, state,
   definition, pins]`, read with the loader's YAML reader (no new crate). That is the file's form;
   247 writes it so. The cases' `journals` values are fixture notation (K3): plan checks that
   `journals` maps plan digests and reads none of its values. A missing file is an empty ledger.
   Another format, an unknown key, a short row or a bad ID exits 2 with a stderr line naming the
   file and the place, and is never read as a new working copy. Plan reads nothing under
   `.conduit/results/`.
5. **The engine's value** (K3 step 2; "Root's authority and protected Conduits"). Send
   `management.list.v1 {}` as root, then inspect each listed object through the tool ACC-033
   requires, with `expectedId` from the list: a Resource whose owning Pool is not retired,
   through root's Conduit to that Pool; an Agent whose owning Fleet is not retired, through
   root's Conduit to that Fleet; everything else through `management`, members of retired
   owners included (ACC-033 rows 6, 6A and 7). The tool is root's local name (IFC-012). Learn
   it, and each instance's owner, only from what the engine returns (root's inventory, or a
   Pool's or Fleet's own `list.v1`), never from `state.json`. Read all that K3's step 2 lists
   (OPS-036, LIF-013, LIF-040). A read that does not succeed, or a transport failure, exits 1,
   naming the tool and the object on stderr, with nothing on stdout.
6. **The comparison** (K3, "The difference table"). Apply the rows and the two paragraphs above
   them as written: the first row that holds decides; rows 1 to 6 first, with row 5's look-ahead;
   every stop printed; a replacing row wins; the rule for inline templates and cascade members
   holds.
   - Acts that need no flag become steps: row 9 (create), row 13 (close a dropped Conduit), row
     18 (a template updated in place), rows 20 and 21 where access narrows ("Which tombstones
     apply lifts with no further flag"), and the `difference` lines of rows 5, 19 and 23.
   - An act a flag would permit prints its row's stop with `requires=`, and no step (248, 249).
     Row 11 always stops here, the renamed Conduit's `tombstone=(Conduit/OLD) requires=--reopen
     NEW` included.
   - Row 8, and row 7's flag for a retirement this working copy made, read the journal of an
     earlier apply, and are 247's. Until then row 7 prints its LIF-024 stop for an instance this
     working copy did not retire. An instance row 8 would decide where no place differs, and a
     retirement this working copy made, both print row 27's `stop KIND/NAME state=STATE
     rule=none -`; where a place differs, rows 17 to 26 decide.
   - The planning step is a pure function of the three values, with no I/O, in a library module,
     `conduit_shell::plan`, which 247 to 249 reuse.
7. **The order of steps** is K3's "Ordering and access": template steps before the first close or
   retirement; narrowing before widening; ties by phase, dependency, then kind and name in byte
   order; a batched registration in written order; setups Pool, Resource, Fleet, Agent,
   Interface, then name; each read just before the activation it verifies.
8. **The text and statuses** (K3, "Text grammar and statuses"), byte for byte: `plan`, the step
   lines with their `detail` lines (`ownTools` and `targetOwnTools` among them), the sorted `stop`
   lines, `affects`, `difference`, `flags`, then `ready`, `no changes` or `stopped`, in K3's
   encoding (no path; UTF-8, LF, one final newline). A stopped plan takes case 09's form, and row
   4's stop K3's (`plan`, its stop lines, `flags []`, `stopped`). Where K3 gives a line only by a
   case (the `detail document=`, `manifest=`, `objects=`, `defaults=` and `held-...` lines, a
   registration's primary object, case 07's list after a close, row 19's retained-pin line,
   case 07's `remains ready`), the cases fix its bytes: choose a rule that fits every case and
   list it under `own_choices` with those cases. Stdout is exactly the plan text; its digest is
   the SHA-256 of those bytes. Each diagnostic is one stderr line naming the place and the
   remedy, never a secret's value (SEC-003). Exit 0 for a valid plan, with steps or no changes;
   2 for a stopped plan, usage, local input or state; 1 for a failed read (decision 5).
9. **Unit tests**, in `conduit_shell::plan`'s `tests` module, with no engine:
   - `row_01_` to `row_27_`, one per row: each builds its three values in K3's compact notation
     and asserts the exact lines; row 8's asserts decision 6's fallback. Where one of K3's safety
     cases 1 to 5, 9, 11, 13, 15 or 18 shows a row, its test includes it and names it in a comment.
   - `ordering_narrows_before_widens`: decision 7's ties, and T4's narrowing of Agent B's Conduit
     (row 21) before any widening.
   - `own_tools_lines`: `{"stated":false}`; a stated absolute directory as `{"pathSha256":…}`;
     `targetOwnTools` after an `operations=` line holding an operation marked
     `startsModelWork: true`.
   - `kept_state_is_read_or_refused`: decision 4's refusals, and a missing file.
10. **Engine tests**, in `engine/crates/shell/tests/plan.rs`, through order 241's harness (a
    served store, fixture packages, the scripted filesystem; no npm, no network), against the
    cases' `recorded.text` as they stand.
    - `plan_case_01_text_and_digest`: on case 01's pre-state (first start, its packages installed
      by `conduit install`) and `files`, plan prints exactly `recorded.text`; the SHA-256 of
      stdout is `recorded.digest`; exit 0.
    - `plan_cases_02_03_07_09_on_replayed_state`: 241's harness replays the chain to each case's
      pre-state and writes its `files`, and its `applied` as `.conduit/state.yaml`, IDs mapped,
      and fills a temporary `CONDUIT_CONFIG_DIR`'s `packages/` with the archives of K3's schedule
      to that case (T1, T2, T5), as `conduit install` keeps them; plan, given that directory,
      prints its `recorded.text` under the same map, exiting 0, 0, 0 and 2.
    - `plan_reads_only`: on case 07's pre-state, every record plan adds is root's access
      decision or invocation of `list.v1` or `inspect.v1` (REC-001), and objects' revisions and
      lifecycle, tombstones, the project directory and the cache are unchanged; on case 01's, no
      `.conduit/`.
    - `plan_secret_literal_leaves_no_trace`: a Type with a secret-reference place given a
      literal: exit 2, stdout exactly `plan`, the `SEC-001` stop, `flags []`, `stopped`, and the
      literal's bytes in neither stdout nor stderr.
    - `plan_stops_on_a_cache_miss`: row 2 end to end: a pinned archive removed from the cache
      prints `archive=absent requires=conduit-install PKG-004`, its bytes changed print
      `archive=mismatch`, both exit 2, and plan goes on once `conduit install` puts it back.
    - `plan_stops_on_another_engine_version`: decision 2, by a context whose `version` differs:
      both versions named, nothing sent.
    - `plan_refuses_unbuilt_flags`: each flag exits 2 with decision 1's line, sending nothing.

## What to read

- [K3](../implementation/plan-and-apply/README.md) as 241 left it: every section from "Commands"
  to "Safety cases", O1, O5, O7 and O8; cases 01, 02, 03, 07 and 09.
- Orders 241 to 243 and their Records: the harness and its ID map, the translator, the context
  and the cache.
- `spec/README.md`, "What the short form's meaning covers" and "Fleets, discovery and
  tombstones"; ACC-024 and ACC-033; LIF-013, LIF-037, LIF-040 and LIF-041; OPS-036; CTR-014;
  SEC-001 and SEC-003; PKG-004, PKG-006, PKG-021 and PKG-025; FLEET-005; SCT-003, SCT-004, SCT-006.
- Shell: `main.rs`, `help.rs`, `author.rs`, `boundary.rs`, `server.rs` (`client`). Core:
  `packages.rs` (`Catalog`), `inventory.rs` (`own_tools`), `archives.rs` (unpacking).

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 5ed7e1ec HEAD; echo "main $?"
git log --format=%s HEAD | grep -E '^Work order (241|242|243): .*built through Conduit' | grep -oE '^Work order 24[123]:' | sort -u | wc -l | tr -d ' '
test -d experiments/029-a84-plan-apply; echo "241 $?"
grep -c 09e985481081192fc23ffd518273c21245765afcb8db5a6ad1b467c156664159 docs/implementation/plan-and-apply/cases/01-t1.yaml
grep -c 'apiKey: null' docs/implementation/plan-and-apply/cases/01-t1.yaml
grep -c '"plan"' engine/crates/shell/src/main.rs engine/crates/shell/src/help.rs
test -e engine/crates/shell/tests/plan.rs; echo "plan test $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "offline $?"
git status --short | wc -l | tr -d ' '
```

The order goes on only once 241, 242 and 243 have merged, each "built through Conduit". It then
prints `main 0`; `3`; `241 0` (241's results); `1` or more (241 moved case 01's agents pin); `0`
(241 dropped `apiKey`); two lines ending `:0`; `plan test 1`; `offline 0`; `0`. On `main` at
`5ed7e1ec` lines 2 to 5 print `0`, `241 1`, `0` and `9`: not ready.

## Scope

Change these files:
- in `engine/crates/shell/src/`, `main.rs`, `help.rs`, `lib.rs` and a new `plan` module, and any
  other shell source only to share an existing function unchanged;
- in `engine/crates/shell/tests/`, the new `plan.rs`, plus `common/` and 241's harness file, only
  to share the replay with what it checks unchanged;
- in `engine/crates/core/src/`, only to make an existing function public, unchanged, or to move
  install's archive-to-`Snapshot` steps into one (decision 3);
- `engine/STANDING.md`, only as `python3 tools/promise_standing.py --write` rewrites it.

Change nothing else: not `spec/`, `conformance/`, `docs/` (K3 and its cases), `experiments/`,
`packages/`, the loader, a `Cargo.toml` or `Cargo.lock`. No other test changes what it checks.
Report anything else that needs changing.

## Done when

The first group prints otherwise on `main` at `5ed7e1ec` ("today") and passes after. The second
are guards; run its last line, the whole suite, once, last.

```sh
ok='^test result: ok\.'
(cd engine && cargo test --offline --release -p conduit-shell --lib plan::tests::row_ 2>&1 | grep -c "$ok 27 passed")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-shell --lib plan::tests 2>&1 | grep -cE '^test plan::tests::(ordering_narrows_before_widens|own_tools_lines|kept_state_is_read_or_refused) \.\.\. ok$')  # 3; today 0
(cd engine && cargo test --offline --release -p conduit-shell --test plan 2>&1 | grep -cE '^test (plan_case_01_text_and_digest|plan_cases_02_03_07_09_on_replayed_state|plan_reads_only|plan_secret_literal_leaves_no_trace|plan_stops_on_a_cache_miss|plan_stops_on_another_engine_version|plan_refuses_unbuilt_flags) \.\.\. ok$')  # 7; today 0
grep -c '"plan"' engine/crates/shell/src/help.rs  # 1; today 0
```

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell -p conduit-core --all-targets -- -D warnings)
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py  # Errors: 0
python3 tools/promise_standing.py --check  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 0 changes; 0 undeclared; 0 errors; exit 0
git diff --check  # prints nothing
git status --short --untracked-files=all | sed 's/^...//' | grep -vE '^engine/(crates/(shell/(src|tests)/|core/src/)|STANDING\.md$)' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Your report gives, per decision, what changed; for each row, its test and any safety case in it;
each new test's failing line on the unchanged code, then its passing line; each core function
made public or moved, and why; each line where K3's text and the output differed before the code
was fixed; your own choices, under `own_choices`; and every check's last line.

Stage nothing. Orders 247 to 251 share `main.rs` and `help.rs` and rebase on this one.

## What follows

Not in this order: 246, the CLI format reference for the fresh author; 247, apply, the kept
state in decision 4's form, the journal, row 8, row 7's flag, the version stop kept and safety
cases 6, 7, 8 and 17; 248 and 249, the seven flags' acts, removing decision 1's refusals, row
26's affected bounds and lost sign-ins, and safety case 10; 250, a grep for anything "not built".

## Record

Not yet run.
