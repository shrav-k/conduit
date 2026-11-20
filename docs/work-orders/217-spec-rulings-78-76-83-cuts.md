# Work order 217: `spec/` brought into line with what rulings 78, 76 and 83 removed: handles, `manages:`, the fence, a reconciliation's stored values, the S1 example, and closing a dropped Conduit without `--prune`

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` each case you add, remove or change, with its
promises and `passes`. This order needs no network: it reaches no host and fetches no crate, and
the only sockets it opens are the loopback ports the engine's own tests bind. Run one cargo command
at a time. Never print a whole large file, case, fixture or log: use `head`, `tail`, `grep`,
`sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential. Do not run `conduit`
against a store, `codex`, `claude` or the dogfood harness, and sign in nowhere.

## What and why

`spec/` alone binds ([AGENTS.md](../../AGENTS.md#what-holds)), so an engine or a vector written to
it today builds what the owner cut. The rulings consolidation's section 2.1 listed these cuts; the
owner chose to write them in one specification order now. Each item below names its ruling's own
words, which this order does not restate (ruling 92 point 7), and the
[IN_FORCE.md](../../IN_FORCE.md) lines marked *spec pending* for it; once `spec/` holds the item,
those lines go.

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
2, every access is a relationship that can be inspected and revoked (a scope is ordinary Conduits;
a dropped Conduit is closed); 3, a shared thing is never copied to an Agent (no API key in the
example); 6, nothing repeats an effect implicitly (an unknown stays unknown; no "did not occur"
re-sends a key); and 7, everything is attributable (a reconciliation records what was checked and
nothing else). **Should this exist** (point 3): no existing tool writes Conduit's own binding text.

**When it runs.** An order of A78's stage 2, sent through Conduit under
[order 211's registration](211-a78-stage-2-registration.md) ("Stage 2's orders"), as ruling 92
point 4 reads it: one worker turn by `conduit` call, then the local gate, then one fresh read; the
round has no verifier. It changes nothing under `experiments/028-a78-dogfood/`
and not the harness. By [ruling 89](../proposals/ruling-89/README.md), each item says what must hold;
how you write it is yours, in the terms of [spec/definitions.md](../../spec/definitions.md).

**Stop rule.** Where an item needs a choice that its ruling does not make, stop there, change
nothing further, and report the item, the choice and the options the ruling leaves. Do not choose.

## The one choice the rulings leave, answered before dispatch

Ruling 78 point 6 says no actor records that an unknown effect did not occur; it does not say what
`reconcile.v1`'s required boolean input `occurred` becomes. The options: (a) `occurred` stays and
`false` reports invalid, naming OPS-014; (b) `occurred` is removed, and a reconciliation records
that the effect occurred; (c) `false` reports conflict under OPS-030, as an unfenced one does today.

Answer: none yet.

If the line above still reads "none yet", the start check stops the order. The coordinator writes
the owner's answer there before dispatch, and item 3 follows it. With the answer, the coordinator
adds to "Done when" one line that fails today and passes only on a build that follows it: for (b),
`grep -c 'occurred' spec/core/operations.yaml` printing 0; for (a) or (c), the expected class of
the changed case that records `occurred: false`.

## The items

1. **Handles are cut** ([78 point 16](../proposals/ruling-78/README.md), its list of what goes and
   changes). IFC-011, `handle.v1` (`spec/core/conduit.yaml`) and its schema keyword
   (`spec/schemas/contracts.yaml`) go; CTR-002, CTR-012 and IFC-033 change, so that an
   implementation's ID is a plain string; the rows "handle" and "hold (a handle)" in
   `spec/definitions.md`, `spec/core/README.md`, the comment in `spec/core/results.yaml`, the
   `files-package` example and the two fixtures that use `handle.v1` (`e05-handle`,
   `e05-secret-omitted`) follow. A case whose promises are all removed goes; one that still tests
   a kept promise is changed. `e05-handle` keeps its ID and stays, changed to test IFC-033 alone,
   since `engine/crates/conformance/tests/vectors.rs` requires it and is out of scope. If the engine
   holds a check for `handle.v1`, it goes. IN_FORCE: the line "*Spec pending, ruling 78*: handles
   are cut."
2. **The `manages:` shorthand is cut** (78 point 16). SCT-012, `manages.v1`, its reserved-key
   entries in `spec/schemas/sectioned.yaml` and SCT-012's two invalid examples go; SCT-006, SCT-013,
   SCT-014, OBJ-019 and OBJ-022 change, and SCT-013's invalid example that holds `manages:` keeps
   testing SCT-013 without it. A scope is written as `consumes` items naming the core's scope
   templates (`fleet-manage`, `fleet-manage-own`, `fleet-view`, `pool-manage`, `pool-view`); Fleet-held
   Pools in `conduits:` wait for batch 2 and are not written here. Every other mention of the key,
   of `manages.v1` or of a generated name `N--manages--T` under `spec/` follows, the row `manages`
   in `spec/definitions.md` and `spec/schemas/README.md` among them; ordinary prose ("the target
   that manages the instance") stays. IN_FORCE: "*Spec pending, ruling 78*: the `manages:`
   shorthand is cut." (topic 4) and "*Spec pending, ruling 78*: there is no `manages:` shorthand."
   (topic 9). If a scope cannot be written as `consumes` items under `sectioned.v1` once SCT-012 is
   gone, without a rule the ruling does not state, stop (the stop rule).
3. **No fence, and no "did not occur"** (78 point 6, the list after "A later order drops"). OPS-014
   drops "did not occur" and the fence, and says how to move on after a hook in the ruling's words
   (its work done by hand and recorded, or its instance replaced); OPS-018, OPS-024, OPS-030,
   LIF-028, REV-010 and `reconcile.v1` lose their fence text; `reconcile.v1`'s `occurred` follows
   the answer above, and its description and `effect` (`spec/core/operations.yaml`, from "Record,
   with what was checked") with it; the rows "reconcile", "conflict", "fenced" and "attempt, new
   attempt" in `spec/definitions.md` and `spec/core/README.md` follow, and so does every other
   mention of a fence or of "did not occur" under `spec/`. F13, as order 172a wrote it into OPS-012,
   stays. The cases that record "did not occur" (find them with
   `grep -rln -e '"occurred": false' -e 'occurred: false' conformance`) and S1-reconcile
   (`conformance/s1-safety-cases.md` and its case tags) change to the answer; none is removed, and
   if one would be left testing nothing, stop (the stop rule). The engine's fence marks go
   (`attempt_fences`, `fence_attempt`, `fenced` and their journal events in
   `engine/crates/core/src/requests.rs`, and the fence's text in `explain.rs`). The packages'
   own provider fences (`provider-local-process`, `provider-ssh-runtime`) are theirs: leave them.
   IN_FORCE: under "*Spec pending, rulings 78 and 80*", the bullet "No actor records that an
   unknown effect did not occur".
4. **A reconciliation records only, for a control** (78 point 5, its list after "A later order
   keeps"). REV-010 keeps only the outcome, REV-009 keeps the settings reported unknown, and REV-013,
   OPS-031, `reconcile.v1`'s `revision` input and the values kept for it go; the cases
   `reconcile-revision-in-progress`, `reconcile-revision-succeeded` and
   `fleets/e3b/reconcile-control` shrink, and the engine stores no control's values on a
   reconciliation. For a setup or an open, IN_FORCE says LIF-003, LIF-028, ACC-044 and
   `reconcile.v1` hold the point already (order 172b): check that, and report anything left.
   IN_FORCE: the bullet "A reconciliation records the outcome and what was checked."
5. **The S1 example is replaced** (78 point 2; [83 point 1](../proposals/ruling-83/README.md):
   no `apiKey`). `spec/schemas/examples/valid/sectioned-project/s1.yaml`, which is S1 itself, is
   replaced by another flow-style short-form project in the same directory, with its
   `conduit.yaml` pins made to match. Its shape is yours, within the ruling's constraints, as the
   *Coordinator's* reading, open to the owner's strike: it is not S1 (none of S1's names, `s1`,
   `team`, `shared`, `history`, `agent-a`, `agent-b`, `console`, `project`, no package under
   `example.org/s1/`, and not S1's structure of Agents sharing a Resource); no `config` holds an
   `apiKey`, and an `account`, if any, is a plain string or null, not an address; and a scope is
   written as `consumes` items naming a core scope template, such as
   `github.com/shrav-k/conduit/core#fleet-view`. Name its file for its project.
   `check_schemas.py` alone validates it (the engine's by-rule test skips this directory,
   `engine/crates/shell/tests/by_rule.rs`); the directory's name
   stays, since `spec/README.md` and `spec/schemas/README.md` link it. IN_FORCE: in topic 13, the
   sentence "*Spec pending, ruling 78*: the short-form example in `spec/` that is S1 itself is
   replaced by another (78 point 2)."
6. **Closing a dropped Conduit takes no flag** ([76 point 1](../proposals/ruling-76/README.md)).
   `--prune` stays for removing another object, since 76 amends ruling 53 point 4 "for removal
   only". In `spec/README.md`, "What the short form's meaning covers", the sentence "Narrowing never
   waits." becomes, as set wording: "Narrowing never waits and never needs a flag: a Conduit the
   file no longer names is closed on apply, and so is a narrowing of a Conduit's operations, and
   the plan prints and records each closing; `--prune` is for removing another object the file no
   longer names." IN_FORCE topic 4: the line beginning "*Spec pending, with a revision of order
   069's reference and order 052a*" keeps 76 points 2 to 5 only, and the **In spec/** line before
   it gains "Narrowing never needs a flag (76 point 1)."

**Not in this order: the labels a machine provider stores** (78 point 16). The ruling cuts them
"once the coordinator's fix for FR-050 has the engine match them itself"; that fix (`place.v1`'s
input `candidates`) is not on `main`. ENR-002, ENR-006, ENR-010, PRV-014 and IN_FORCE's line on
them stay as they are.

**What follows, for every item.** In each changed file's head comment "Removed IDs (not reused)",
add IFC-011, SCT-012, OPS-031 and REV-013 in the file's form, with "nothing replaces it (ruling
78)", and register them where `spec/README.md` keeps removed IDs. Each IN_FORCE line named above
goes, and so does any header left with nothing pending under it. "*Spec pending, rulings 78 and
80*" keeps its pending mark, since its bullet "On a plain server a retry is a new request" is
neither **In spec/** nor written by this order; if `spec/` already holds that bullet, report the
promise that does. Update the rows of
`engine/CONFORMANCE.md`, `conformance/s1-promises.yaml`, `conformance/s1-safety-cases.md` and
`conformance/expected-failures.yaml` that the changes make false or that name a removed promise;
run `python3 tools/promise_standing.py --write`; and in
`experiments/012-model-checking/account.yaml`, by [order 153](153-model-account-checked.md)'s
decision 7, mark a changed convention promise's model-run row `text changed, not rerun` with one
sentence naming this order appended to its `note`, and remove a removed promise's row. Write
`docs/work-orders/217-declared.yaml` in the form of [209's](209-declared.yaml), declaring every
case added, removed or changed. Where a change to `spec/` makes an engine test fail, change the
engine until it passes, without loosening a check that does not concern these items.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor d97fbe76 HEAD; echo "main $?"
grep -c '^Answer: none yet\.$' docs/work-orders/217-spec-rulings-78-76-83-cuts.md
grep -c 'candidates' spec/core/operations.yaml
grep -c -e 'id: IFC-011$' -e 'id: SCT-012$' -e 'id: OPS-031$' -e 'id: REV-013$' spec/interface/implementation.yaml spec/promises/sections.yaml spec/conventions/operations.yaml spec/conventions/revisions.yaml | cut -d: -f2 | tr '\n' ' '; echo
grep -c 'Narrowing never waits\.' spec/README.md
grep -c 'apiKey' spec/schemas/examples/valid/sectioned-project/s1.yaml
grep -c -e 'fence_attempt' engine/crates/core/src/requests.rs
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `0` (the answer is written), `0` (FR-050's fix has not landed), `1 1 1 1`,
`1`, `2`, a number above 0, and `0`. If `candidates` prints above 0, FR-050's fix has landed:
stop and report, so the coordinator adds the label cut.

## Scope

Change `spec/`, `conformance/` (cases, fixtures, indexes, `manifest.yaml`, `s1-promises.yaml`,
`s1-safety-cases.md`, `expected-failures.yaml`), `engine/crates/core/src/`, `engine/CONFORMANCE.md`,
`engine/STANDING.md` only by the tool, `experiments/012-model-checking/account.yaml` as above, the
IN_FORCE.md lines named above, and create `docs/work-orders/217-declared.yaml`. If the engine's
loader or shell must change (`engine/crates/loader/` or `engine/crates/shell/`), change it and say
why. Change nothing under `packages/`, `tools/`, `engine/crates/conformance/`, `docs/` outside the
declaration, any other `experiments/` path, `engine/Cargo.lock`, any `Cargo.toml`,
`docs/AUDIT.md` or `ROADMAP.md`. Documents that do not bind and quote a removed item (the
plan-and-apply reference, `docs/OVERVIEW.md`, guides) go in your report, not in this change.

## Done when

Each line below marked "fails today" prints otherwise on `main` at `d97fbe76`; all pass after.

```sh
(cd engine && cargo fmt --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, the vectors among it
git add -N spec conformance docs/work-orders/217-declared.yaml  # so the checks see new files
python3 tools/check_docs.py                     # Errors: 0
python3 tools/check_rulings.py                  # passes
python3 tools/set_names.py --check              # passes: no assigned name changes
python3 tools/check_schemas.py                  # Errors: 0
python3 tools/check_roadmap.py                  # passes
python3 tools/check_model_account.py            # passes
python3 tools/promise_standing.py --check       # Errors: 0
python3 tools/check_private_paths.py            # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/217-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; exit 0
git diff --check                                # prints nothing
git diff --name-only HEAD -- packages tools engine/crates/conformance engine/Cargo.lock docs/AUDIT.md ROADMAP.md experiments/028-a78-dogfood | wc -l | tr -d ' '  # 0
grep -rc -e 'id: IFC-011$' -e 'id: SCT-012$' -e 'id: OPS-031$' -e 'id: REV-013$' spec | awk -F: '{s+=$2} END {print s}'  # 0; fails today (4)
grep -rl -e 'handle\.v1' -e 'manages\.v1' -e 'manages:' -e '--manages--' -e '`manages`' spec | wc -l | tr -d ' '  # 0; fails today
ls spec/schemas/examples/invalid | grep -c '^SCT-012-'   # 0; fails today (2)
grep -rliE '\bfenc(e|ed|es|ing)\b' spec | wc -l | tr -d ' '  # 0; fails today (6)
grep -rli 'did not occur' spec | wc -l | tr -d ' '  # 0; fails today (6)
grep -c -e 'attempt_fences' -e 'fn fence_attempt' engine/crates/core/src/requests.rs  # 0; fails today
grep -rl -e 'apiKey' -e 'example.org/s1/' -e 'agent-a' spec/schemas/examples/valid/sectioned-project | wc -l | tr -d ' '  # 0; fails today
tr -s ' \n' '  ' < spec/README.md | grep -c 'Narrowing never waits and never needs a flag: a Conduit the file no longer names is closed on apply'  # 1; fails today
grep -c -e 'handles are cut\.' -e 'the `manages:` shorthand is cut\.' -e 'there is no `manages:` shorthand\.' -e 'that is S1 itself is replaced' -e 'No actor records that an unknown effect did not occur, and no attempt is fenced' -e 'For a control, REV-009, REV-010, REV-013, OPS-031' IN_FORCE.md  # 0; fails today (6)
grep -c 'a machine provider no longer stores the labels' IN_FORCE.md  # 1: the labels stay
git reset -q -- spec conformance docs/work-orders/217-declared.yaml  # stage nothing
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML. Your report gives, per item: what changed,
the promises changed and removed, the cases added, removed or changed and why, and each sentence
you worded yourself (under `own_choices`); the vector delta's counts; each binding text you left
that still quotes a removed item, with why; each document that does not bind that does; and every
check's last line. Stage nothing. The coordinator runs `tools/gate.py`, then has a fresh, read-only
Opus reader read the diff ([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4), and
merges.

## Record

Not yet run.
