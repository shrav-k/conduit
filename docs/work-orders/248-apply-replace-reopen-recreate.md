# Work order 248: `--replace`, `--reopen` and `--recreate` in plan and apply: replacement under stable names, the tombstones it lifts, and the bounds it prints

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no conformance case changes. This order
needs no network: it reaches no host and fetches no crate, and its only sockets are the loopback
ports the engine's own tests bind. Every crate it builds is in the local cache already; if a build
asks for one that is not, stop and report, and fetch nothing. Run one cargo command at a time.
Never print a whole large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Do not run `conduit` outside a
test, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere. Every file
you write is public: write no absolute local path, no user or machine name and no private
repository name.

## What and why

**What.** S1's T4 to T7 ([experiment 007](../../experiments/007-s1-baseline/README.md)) widen
one grant and narrow another, move the filesystem pin, change the shared directory, and restore
a revoked grant. After orders 245 and 247, plan and apply print each as a stop naming its flag
and send nothing. This order makes the three flags work, so that
[K3's cases](../implementation/plan-and-apply/cases/) 04, 05, 06 and 08 pass end to end. It is the
stable-name half of batch 1's 052b
([the plan](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/README.md#the-work-orders));
243 has its install and init halves, 244 its diagnostics; `render` was dropped.

**What settles it.** `spec/README.md`, "What the short form's meaning covers": each dangerous act
takes a named flag; narrowing needs none; "Replacement keeps names stable (SCT-004, SCT-013):
retire and remove the old instance before making its replacement under the same name, lifting
its tombstone wherever necessary"; "Plans print affected bounds" (ACC-029). K3,
[the CLI reference](../implementation/plan-and-apply/README.md), gives the rows, steps,
readings and text; what they leave open this order decides (the coordinator's, open to the
owner's strike).

**Corrections to the chain's plan:** rows 7, 12, 18, 20 and 21 are on no order's list and act
under these flags, so they are here. K3 has no printed form for an operand that stops, a lost
bound, a lost sign-in or a Conduit outside the file: decision 7 gives them. IN_FORCE.md topic 6
still gives the bounds to "CLI order 052b" and 250 leaves it, so decision 9 changes it.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); ruling 92, point 1): 2,
a revoked grant stays revoked until a flag that names it reopens it, and no edit lifts a
tombstone; 6, no lift, reopening or resend that the command did not name; 5, the plan shows each
bound that stops or starts covering a replaced object. Test 2 is at risk from an implicit lift;
decisions 1, 3, 4 and 6 guard it. Decision 4's `lifts-caused` lift is the one no flag names;
the plan prints it.

**Should this exist** (point 3). Yes, as existing practice: Terraform's `-replace=ADDRESS` and
Pulumi's `--replace URN` name one object and show in the plan; destroy-before-create is
Terraform's default, which ruling 55 A3 chose. `--reopen` and `--recreate` are Conduit's own:
LIF-041's tombstone blocks remaking a revoked grant, which those tools lack. Not taken:
create-before-destroy (OBJ-002 holds the name); an approval with no operand (ruling 76, point 2).

**When it runs.** Through Conduit's dogfood, shell lane (worker-2): one Codex worker turn (a
4-hour limit), a Codex verifier and a Claude reader who run only what settles a doubt, and the
gate (DEC-030, DEC-033). By ruling 89, each decision says what must hold; how is yours. Order
249 runs beside this one and rebases onto it.

**Size.** L: 1.0 to 1.6k changed lines with tests (the chain's plan; it may run 1.5 to 2 times
low). If the turn runs short, finish decisions 1 to 4 and 7 to 9 with the four golden cases
first, then 6, then 5, and report what is left.

## The decisions

1. **Operands** (ruling 76, point 2). `plan` and `apply` accept `--replace NAME`, `--reopen NAME`
   and `--recreate NAME`, each repeatable, each with one generated name. A flag with no name is a
   usage error: status 2, before any read. A flag and name given twice count once. An operand is
   used where the plan computed without it would stop with `requires=` that flag and name; it
   then permits that row's steps. An unused operand, or one whose stops fall on objects of two
   kinds, stops the plan (status 2, nothing sent or kept) in decision 7's form. So
   `--reopen shared`, naming a Resource, is unused, and a name given to two flags stops on the
   unused one. Keep the operand table in one place, so that 249 adds its flags as rows. The help
   lists the three as K3's table does (`[--recreate OBJECT]`). 245's "is not built yet" refusal
   goes for these three and stays for `--prune`, `--adopt` and the cascade flags until 249.
2. **`--replace NAME`** (rows 18, 21, 22, 25, 26; K3's
   [replacement section](../implementation/plan-and-apply/README.md#replacement-removal-and-cascades)).
   - A Conduit: close, list, lift, register, inspect, open.
   - Another instance: where row 18 holds for its inline template `N--t`, replace it at its
     inspected revision; retire it through its owner; once that succeeds, list, lift, and
     register it with each incident Conduit the file holds; inspect and set it up; then inspect
     and open each incident Conduit. Those its retirement closed (CON-008), root's `N--operator`
     among them, print under `closed-conduits=` and are made again under the same flag.
   - Every template step comes before the first close or retirement, and every narrowing before
     every widening (B's read before A's write in case 04; K3's safety case 8).
   - Row 21 takes the same steps with `flag=-`. A project template's update takes no flag of its
     own (row 18); where its inline instance is replaced, the step carries that `--replace`.
   - An owner change (`/owner`) is a replacement: retire through the old owner, register and set
     up through the new. No request moves an object (FLEET-002, OBJ-011).
   - Without the flag each object stops as its row says; the plan sends no part.
3. **Lifts with no flag** (LIF-041, ACC-053; K3's "Which tombstones apply lifts with no further
   flag"). After a removal, the plan's `management.list.v1 Tombstone/*` step reads fresh
   tombstones. Apply lifts only one whose cause is that removal's invocation, or one such a
   tombstone caused, with `expected=(step N)` and `flag=-`, except row 20's lift, which carries
   the nested instance's `--replace NAME` as K3's row 20 says; it never guesses an ID. Where the
   list holds none, or two for one object, or a lift's `remaining` keeps one that a later step
   would match, apply stops: verification failed, status 1, `not sent N..M`. A lift that removes
   caused tombstones prints `detail lifts-caused=[...]`, as case 05 does.
4. **`--reopen NAME` and `--recreate NAME`** (rows 11 and 12; case 08; K3's safety cases 1 to 4).
   A tombstone this command did not cause, which a registration or open of the plan would match,
   is lifted only under the flag naming the object to be made: `--reopen` for a Conduit,
   `--recreate` for another instance. So recreating `shared` after another actor retired it needs
   `--reopen` for each of its Conduits too, though one lift removes them all; that step's flag
   field joins the spellings with commas, as K3's grammar does for a batched registration. A
   tombstone a lift removes that no step would match is printed in `lifts-caused` (the
   coordinator's reading, open to the owner's strike: `lift.v1` removes caused tombstones by
   contract, ACC-053). Row 12 (no tombstone): register, inspect, set up or open, under the same
   flags.
5. **A nested instance** (row 20; LIF-009). A declared Conduit or a Pool entry's Resource whose
   `from` template changed, inside an instance not replaced: close it or retire it through its
   Pool; list; lift the tombstone that leaves where the new one would match; then
   `register-replacement.v1`, inspect, open or set up. The steps carry `--replace NAME` naming the
   nested instance as inspection shows it, or `flag=-` where a declared Conduit narrows as row 21
   says. Later lines name the result `Kind/(step N).object expected=(step N)`, with the detail
   `nested=member`.
6. **Interrupted and failed replacements** (K3's
   [stops section](../implementation/plan-and-apply/README.md#stops-unknown-outcomes-and-repeats);
   LIF-026). The coordinator's reading of "An interrupted apply's own journal may establish the
   same apply and cause": a later command finishes an interrupted replacement only where it names
   the flag the journal recorded; its own-cause lifts then take `flag=-`. Without it, the plan
   stops on the object with `requires=` that flag and sends nothing, so B stays closed rather than
   A opening (a window without access, which `spec/README.md` accepts). A retirement under
   `--replace` whose shutdown is unconfirmed is retired again only under the same flag, never by
   `unregister.v1` (row 7; LIF-017).
7. **The printed forms K3 leaves open** (the coordinator's, open to the owner's strike), in K3's
   [grammar](../implementation/plan-and-apply/README.md#text-grammar-and-statuses):
   - `stop Flag/FLAG operand=NAME use=none -`, and
     `stop Flag/FLAG operand=NAME use=ambiguous objects=[KIND/NAME,...] -` (FLAG spelled as in
     `flags`, e.g. `Flag/--replace`).
   - `affects bounds=[Conduit/HOLDER:KIND/NAME,...]`: each bound that stops covering a replaced
     instance. A `targets` item keeps the old ID; a `pools` item stops covering only on an owner
     change (ACC-029; ruling 59 A7). On an owner change, bounds that start covering it go in a
     `bounds-added=[...]` item of the same `affects` line, after `closed-conduits=`, printed only
     where not empty. Bounds outside the file count.
   - `signed-out=[Agent/NAME,...]`, the line's last item, only where not empty: each replaced
     Agent whose Type states `signIn`, since its replacement starts with a new home.
   - A Conduit outside the file whose consumer or target is a replaced object:
     `stop Conduit/NAME absent=file place=/target CON-008`, or `place=/consumer`, with no
     `requires=`: no flag permits it, as K3 says.

   Add them to K3 and, where 246 has merged, to `docs/guides/plan-and-apply.md` (no S1 name).
   Every form the cases print stays.
8. **The cases.** Cases 04, 05, 06 and 08, as order 241 left them, are fixed inputs. Change one
   only where it contradicts K3's prose or `spec/`, and report the sentence it follows; a change
   no sentence decides is a stop.
9. **IN_FORCE.md**, topic 6: the line's second sentence becomes "Replacement plans print the
   bounds that stop covering, in `conduit plan` and `conduit apply` (order 248); the guidance is
   in spec/README.md." No other line changes.

## The tests

In a new `engine/crates/shell/tests/apply_replace.rs`, through the golden-case harness of orders
241 and 247 (release binary, serving engine, fixture packages, scripted filesystem, no npm). In
each, a stopped plan sends only list and inspect and writes nothing (compare `list.v1` and
`.conduit/` before and after).

- `golden_case_04_t4`, `golden_case_05_t5`, `golden_case_06_t6`, `golden_case_08_t7_restore`:
  on the case's engine state and `applied`, `conduit apply` with its flag prints `printed` byte
  for byte, sends its `requests`, keeps `recorded` and `kept`, and exits with `exit`;
  `conduit plan` with the flag prints the recorded text and writes nothing.
- `unflagged_replacements_stop_and_send_nothing`: cases 04, 05 and 06 with no flag print their
  rows' stops (22's `place=/from`, 26's `pin=`, 25's `setting=/config/root`) and exit 2.
- `operands_name_one_object_or_stop`: each stop of decision 1, one command apiece.
- `recreate_and_reopen_after_an_outside_retirement`: after case 06, root retires `shared` with
  `conduit retire`. `apply` stops for `shared` (`--recreate`) and each of its Conduits
  (`--reopen`); `--recreate shared` alone still stops for the Conduits; all three restore them,
  `shared` with a new ID under its name. Then an outside retirement and lift give row 12's stops,
  and the same flags restore.
- `replacement_prints_bounds_and_an_owner_change`: two Conduits to `access` applying `bound.v1`,
  one with `shared` in `targets`, one with its Pool in `pools`, and a second Pool, which a third
  Conduit to `access` names in its bound's `pools`. Replacing `shared` lists only the first in
  `bounds=`. Moving it to the second Pool stops with
  `place=/owner requires=--replace shared OBJ-011`; with the flag it retires through the old Pool,
  sets up through the new, and lists the first two in `bounds=` and the third in `bounds-added=`.
- `replacement_stops_on_a_conduit_outside_the_file`: a Conduit to `shared` registered and opened
  outside the working copy; `apply --replace shared` prints decision 7's stop and exits 2.
- `agent_replacement_prints_the_sign_in_loss`: an annotation of `agent-a` changed;
  `--replace agent-a` retires it through its Fleet, remakes its Conduits under the flag, and
  prints `signed-out=[Agent/agent-a]` and its `ownTools` line.
- `interrupted_replacement_resumes_only_under_its_flag`: case 04 interrupted after step 2, as
  247's resume test interrupts, or by replaying steps 1 and 2 and the journal 247 writes then. A
  bare `apply` stops and sends nothing; `apply --replace agent-a--project` ends as case 04 does.
- `nested_instance_replaced_under_a_new_name`: decision 5, for one declared Conduit.

Where the scripted filesystem can leave a retirement's shutdown unconfirmed, add
`unconfirmed_retirement_is_retired_again_under_its_flag`; otherwise report that row 7 has no
test. Write the tests first and run them on 247's code: report each failing line.

## What to read

K3's README, all of it, and cases 04, 05, 06 and 08; `spec/README.md`, "What the short form's
meaning covers" and "Fleets, discovery and tombstones"; LIF-009, LIF-017, LIF-026, LIF-035,
LIF-040, LIF-041, ACC-029, ACC-053, OBJ-002, OBJ-011, FLEET-002, CON-008; `bound.v1`,
`retire.v1`, `register-replacement.v1` and `lift.v1` in `spec/core/`; the shell's plan and apply
code from 245 and 247, their tests and harness, and `help.rs`.

## Stop rule

Stop, change nothing further, and report the gap and its options, without choosing, where:

- a start-check line prints otherwise, a crate missing from the cache among them;
- a case's expected text, or a decision here, needs a choice that neither K3 nor `spec/` makes;
- a correct result needs a change to `engine/crates/core`, `engine/crates/loader`, `spec/`,
  `conformance/`, a `Cargo.toml` or the lockfile, or a request that is not a core operation;
- decision 6's interrupted state cannot be reached without a sleep or retry beyond the harness's.

A choice no decision governs is yours (`own_choices`). Where 245 or 247 built part of a row
named here, keep it, add what is missing, and say which.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
cat docs/implementation/plan-and-apply/cases/0[4568]-*.yaml | grep -cE '479ef470|apiKey'
grep -c '"plan",' engine/crates/shell/src/help.rs
grep -c '"apply",' engine/crates/shell/src/help.rs
test -e engine/crates/shell/tests/apply_replace.rs; echo "new $?"
test -e docs/guides/plan-and-apply.md; echo "246 $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

On `main` at `cc3605a3` they print `main 0`, `128`, `0`, `0`, `new 1`, `246 1`, `engine 0` and
`0`. This order starts only once orders 241, 243, 245 and 247 have merged. It then prints
`main 0`; `0`, since 241 moved the cases to the agents fixture `09e98548…` and `.v2`
configurations with no `apiKey`; `1` for 245's `plan`; `1` for 247's `apply`; `new 1`; `246 0`
or `246 1` (either goes on); `engine 0`; and `0`.

## Scope

Change `engine/crates/shell/src/` (plan, apply, the operand table, `help.rs`),
`engine/crates/shell/tests/apply_replace.rs` and the harness for a case ID alone, K3's README
(decision 7) and its cases 04, 05, 06 and 08 (decision 8 alone), `docs/guides/plan-and-apply.md`
where 246 has merged, IN_FORCE.md (decision 9), and `engine/STANDING.md`, only as
`python3 tools/promise_standing.py --write` rewrites it. Change nothing else (not `spec/`,
`conformance/`, core, loader, `engine/README.md` or a Cargo file); report what else needs it.

## Done when

Each line marked "fails today" prints otherwise on `main` at `cc3605a3`; all pass after. Run
`git add -N engine/crates/shell/tests/apply_replace.rs` first, so that the checks see it.

```sh
(cd engine && cargo test --release -p conduit-shell --test apply_replace -- --list 2>/dev/null | grep -cE '^(golden_case_04_t4|golden_case_05_t5|golden_case_06_t6|golden_case_08_t7_restore|unflagged_replacements_stop_and_send_nothing|operands_name_one_object_or_stop|recreate_and_reopen_after_an_outside_retirement|replacement_prints_bounds_and_an_owner_change|replacement_stops_on_a_conduit_outside_the_file|agent_replacement_prints_the_sign_in_loss|interrupted_replacement_resumes_only_under_its_flag|nested_instance_replaced_under_a_new_name): test$')  # 12; fails today (0)
(cd engine && cargo test --release -p conduit-shell --test apply_replace golden_case_ 2>&1 | grep -c '^test result: ok\. 4 passed')  # 1; fails today (0)
(cd engine && cargo test --release -p conduit-shell --test apply_replace 2>&1 | grep -cE '^test result: ok\. [0-9]+ passed; 0 failed')  # 1; fails today (0)
grep -c -- '\[--recreate OBJECT\]' engine/crates/shell/src/help.rs  # 2, one per row; fails today (0)
grep -cE 'use=(none|ambiguous)' docs/implementation/plan-and-apply/README.md  # 2 or more; fails today (0)
grep -cE 'signed-out=|bounds-added=' docs/implementation/plan-and-apply/README.md  # 2 or more; fails today (0)
grep -c 'remains for CLI order 052b' IN_FORCE.md  # 0; fails today (1)
```

These pass, as on `main`; run the whole suite once, last, then `git reset -q`, and stage
nothing:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy -p conduit-shell --all-targets -- -D warnings)
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
python3 tools/promise_standing.py --check  # Errors: 0
git diff --check  # prints nothing
git diff --name-only HEAD | grep -vE '^(engine/crates/shell/(src|tests)/|docs/implementation/plan-and-apply/|docs/guides/plan-and-apply\.md$|IN_FORCE\.md$|engine/STANDING\.md$)' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite
```

Your report gives, per decision, what changed and which rows 245 or 247 had built; each test's
failing line on 247's code, then its passing line; each case change with its sentence; whether
246 had merged and where decision 7's forms went; `own_choices`; and every check's last line.

## What follows

- Order 249: `--prune`, `--adopt` and the cascade flags, as rows of decision 1's table.
- Order 250: T4 to T7 in the short form through these flags, with the real Agent packages'
  contracts; S1 replaces no Agent, so no `signed-out=` line prints there.
- Whether apply may call a control in place of replacing for an edited setting (row 25;
  decision D67, finding 4-01): the owner's, after the S1 rerun.
- K3's O2, the same-ID race, stays open; so does naming an object by kind, should `use=ambiguous` arise.

## Record

Not yet run.
