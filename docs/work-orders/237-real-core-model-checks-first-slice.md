# Work order 237: model checking of the real core, first slice: `conduit_core::step` under message loss and reordering, with experiment 012's O, W and U as invariants

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host, fetches no crate and opens no socket. Every crate it builds is in the
local cache already; if a resolution or a build asks for one that is not, stop and report, and
fetch nothing. Your turn has a 4-hour limit; plan to finish in about two. Run one cargo command at
a time. Never print a whole large file, diff, lockfile, trace or log: use `head`, `tail`, `grep`,
`sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential. Do not run `conduit`,
`codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere. Every file you
write is public: write no absolute local path, no user or machine name and no private repository
name.

## What and why

**What.** A test crate in the engine's workspace that runs the engine's real pure core,
`conduit_core::step`, as the state machine of a bounded explorer, Stateright 0.31.0, the tool
experiment 012 uses. One caller and one implementation stand-in talk to it; messages between the
engine and the implementation, on one machine, are lost and reordered. Three of 012's safety
properties are its invariants: at most once (O), no wrong outcome (W) and unknown stays unknown
(U). A refutation prints a counterexample that a seed reproduces.

**Why.** On 2026-11-20 the owner chose, for v0's model checking, "A: check the real core (Rec.)",
put as "Deterministic simulation and small exhaustive checks of the real engine core under loss,
reordering and partitions; 012's checks reused as invariants; 3-5 orders, after S2 and the runtime
stream, before v0's gate"
([DEC-005](../decisions/log.md#dec-005-2026-11-20-v0s-gate-publication-and-stewardship)), and
froze experiment 012
([DEC-026](../decisions/log.md#dec-026-2026-11-20-proof-cases-experiments-measurement)). 012's
models are "pure reference models of the rules promises state, independent of engine code": a
claim about the text, not about an engine. v0 must pass "model checking of the safety promises"
(ROADMAP.md, "What v0 must pass"). This is the first of those three to five orders.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1): 6,
served: O and U check that the real core repeats no effect and settles no unknown outcome by
itself, in every order of lost and reordered messages the bounds allow (OPS-018, OPS-013). None
at risk: no promise, engine source or case changes.

**Should this exist** (ruling 92 point 3). Yes: the owner chose it. Each part exists already: the
core's own `step` is the state machine, so no second model of the rules is written; Stateright
explores it, as in 012 and experiment 004; driving a deterministic implementation under injected
message faults is deterministic simulation testing (FoundationDB's simulator, TigerBeetle's
VOPR). Not taken, the coordinator's, open to the owner's strike: an explorer of our own
(Stateright exists, and 012 records its limits); `turmoil` or `madsim` (they simulate a Tokio
runtime and network, and the core has neither); `loom` or `shuttle` (thread interleavings, not
lost or reordered messages); building on 012's crate (frozen); a TLA+ or Quint model (a second
copy of the rules, which option A avoids).

**When it runs.** DEC-005 places these orders after S2 and the runtime-provider stream; the
coordinator dispatches this one then. Nothing in it depends on them, so its start check holds on
today's `main`. It runs through Conduit's dogfood: one Codex worker turn, a Codex verifier and a
Claude reader who run only what settles a doubt, then the gate, which runs the whole suite
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). Each decision says what must
hold; how is yours
([ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md)).
The names, bounds and stand-ins are the coordinator's, open to the owner's strike.

## The decisions

1. **Where it lives.** A new member of the engine's workspace, `engine/crates/model-check`,
   package `conduit-model-check`, `publish = false`: the engine's own test, which the gate's engine
   suite and CI's engine jobs run whenever the core changes. Its dependencies: `conduit-core` by
   path, `serde_json`, `stateright = "=0.31.0"` (012's pin) and, for decision 3,
   `conduit-conformance` by path. Nothing in the engine depends on it. Experiment 012 changes not
   at all, and nothing depends on it.

2. **The lockfile.** Add the member and let cargo resolve offline, once: the start check's `cargo
   metadata --offline --format-version 1 --filter-platform "$h"`, without `--locked`. Unfiltered,
   it asks for `wasm-streams`, not in the cache. On `main` at `c1d09afc` that adds stateright's
   packages and moves seven target-only packages to the cache's versions: `js-sys`,
   `wasm-bindgen`, `wasm-bindgen-futures`, `wasm-bindgen-macro`, `wasm-bindgen-macro-support`,
   `wasm-bindgen-shared` and `web-sys`. Neither the Mac's nor Linux's dependency graph holds them
   (the coordinator's probe of 2026-11-21). If any other package's version changes, stop.

3. **The real core is the state machine.** Every engine-side action is one `conduit_core::step(`
   call, written in full; the harness decides nothing the engine decides. A `step` error fails
   the run and is not a verdict. Evidence is deterministic: a fixed time, and IDs the harness
   derives from the path and never reuses. The starting state is the one the conformance runner
   builds for a case's first request: `requests-unknown-write`'s for the plain configuration
   (`file.write.v1` on a plain server) and `requests-repeat-failed-earlier-unanswered`'s for the
   deduplicating one (Conduit-aware, `deduplicates: true`). To reuse the runner's fixture code,
   add one public function to `engine/crates/conformance/src/` that returns a case's context, the
   state `conduit_core::step` receives at a given step (after the fixture's patches and updates)
   and that step's core input. No case's result changes.

4. **Actors and the script** (MC2's, cut to one key). One caller sends that request, one key and
   one contents, and repeats it at most twice. One implementation stand-in, written in the
   harness. In the plain configuration it applies each call it receives and answers succeeded or,
   at most once per run, with an error and no effect. In the deduplicating one it applies at most
   once per key, answers a repeat with its first answer, and at most once per run answers failed
   with no effect (012's MC2 rerun). The engine stops waiting for the current attempt (the shell's
   `missing` completion) at most twice per run, whatever the links hold. The discovery that the
   core asks of an aware implementation before its call may be answered at once; it is not a fault
   target in this slice.

5. **The fault model.** Two directed links, engine to implementation and back, on one machine.
   Each is a sequence that keeps positions: a delivery takes position 0, a loss removes any one
   message, and a reordering delivers a later position. Parts, each budget one: T (no fault), L
   (one loss), R (one reordering), LR (one of each). The plain configuration runs T and L: it
   never has two messages on a link, so R adds nothing. The deduplicating one runs all four. At
   most three messages per link; exceeding that fails the run rather than pruning. No duplication,
   crash, restart, cut or lost machine in this slice.

6. **The invariants, reused from 012.** Port O, W and U from
   `experiments/012-model-checking/src/operations.rs` (`ONCE`, `RIGHT` and `UNKNOWN`, as MC2 ran
   them on 2026-11-15 under order 160), keeping their names and promise IDs and citing the source.
   They read only a ledger the harness keeps outside the core. Per invocation it holds each
   dispatch (from the core's dispatch intents), each effect the stand-in applies, each answer it
   sends, and each outcome the core reports, with the effect count then. Outcomes come from what
   the core emits (its records and replies), not from its private state. This slice has no
   reconciliation, so W's OPS-014 half and U's reconcile exception stay idle. U counts an
   operation as deduplicating only where OPS-012 allows a new attempt: `deduplicates: true` and
   not plain.
   Each full search also has witnesses, Stateright `sometimes` properties, and a witness not
   reached fails the run: an effect; each enabled fault used; each outcome the configuration can
   give; a repeat answered without a dispatch. In the deduplicating configuration there are two
   more: a new attempt after unknown and, under L, R and LR (012 had it under L and R), OPS-012's
   case of a new attempt answered failed while an earlier one has no answer.

7. **Controls.** Each invariant is shown able to fail through the real core. A control breaks what
   the core relies on, in a stand-in or a check, and never changes the core. All three use the
   deduplicating configuration in part T:
   - O's control: the implementation applies every call it receives, though the contract says it
     deduplicates.
   - W's control: the implementation applies its first call and answers failed.
   - U's control: the check reads the operation as not deduplicating.

   Each control has a full search, which gives its row of the table, and a one-thread
   breadth-first search that stops at its target's first discovery and prints the trace
   (decision 9).

8. **Searches, bounds and time.** A full search is breadth-first, with at most two checker threads,
   a ceiling of at most 200,000 unique states and a 300-second deadline. A search that reaches
   either fails and gives no verdict (012's rule); `truncated_search_gives_no_verdict` shows it.
   The tests are top-level functions in `tests/operations.rs`, named as in Done when. Refutations
   run one thread. Simulations use Stateright's `spawn_simulation` with `UniformChooser` on one
   thread: three fixed seeds for each correct configuration in its largest part (plain L,
   deduplicating LR), each bounded by a state count. On the Mac, release build and warm, each
   search takes under 30 seconds and the crate's whole test run under 120. If a search runs over,
   cut the caller's repeats or the engine's stops before a fault, and report what was cut. CI runs
   the same tests in its engine jobs; the coordinator reads their time at the first CI run after
   the merge, expected under 6 minutes. A search may merge states it treats as equal, by hashing a
   projection of the core's state. If it does, the crate's README names what the projection drops
   and why no later step of this slice reads it.

9. **The counterexample printout.** A refutation prints one header line, `<property name>
   refuted: <configuration>/<part>, <search>, <n> actions`, where the configuration is `plain`,
   `deduplicating`, `o-control`, `w-control` or `u-control`, and the search is `bfs` or
   `simulation seed <N>`. One numbered line per action follows, such as
   `3. lost engine -> implementation [position 0]`, then the invocation's ledger history. One test
   runs a control's search twice, and one seeded simulation of a control twice, and asserts that
   the two printouts are the same.

10. **Documents.** `engine/crates/model-check/README.md` says what is checked and what is not
    (bounded; safety only; the real core with stand-ins, not the shell's transport), how to run
    it, the table below as registered, and the dated results: per configuration and part, each
    verdict, unique states, depth and seconds, and each control's trace length. `engine/README.md`
    names the crate in its list of crates at the top, and its "Choices" gain an entry for
    stateright, saying why and what was not taken. `tools/set_names.py --check` scans `engine/`:
    write no name that `spec/names.yaml` defines; read it, as the tests do.

## The registered verdicts

Fixed before any run. A run that differs is reported beside the table, which stays.

| Configuration | Parts | O | W | U |
| --- | --- | --- | --- | --- |
| plain | T, L | holds | holds | holds |
| deduplicating | T, L, R, LR | holds | holds | holds |
| o-control | T | refuted | holds | holds |
| w-control | T | holds | refuted | holds |
| u-control | T | holds | holds | refuted |

## Stop rule

Stop, change nothing further, and report what you found, with the options and without choosing,
where:
- a resolution or build asks for a crate not in the cache, or decision 2's lock check fails;
- a correct configuration refutes O, W or U: change nothing in the core, report the first
  one-thread trace, and class it as 012 does (the core against a promise, the promise's text, or
  the property over-broad);
- a correct result needs a change under `spec/`, `conformance/` or `experiments/`, or to the
  core's or the shell's sources;
- a witness is not reached within decision 8's bounds: report the counts, and keep the witness.

A choice no decision makes is yours: list it under `own_choices`.

## What to read

- `experiments/012-model-checking/README.md`: "Run it" (how Stateright 0.31.0 can replace a first
  trace, and how 012 keeps it), "Fault model and model requirements", "MC2 operation identities",
  "MC2 rerun under ruling 78" and its limits; `src/operations.rs` and `src/harness.rs`.
- OPS-008, OPS-009, OPS-012, OPS-013, OPS-018 and OPS-024 in `spec/conventions/operations.yaml`;
  IMP-007; IFC-010, for a plain server's error.
- `engine/README.md`, "The core's boundary"; `engine/crates/core/src/lib.rs` (`step`, `Input`,
  `Transition`) and `requests.rs` (`complete`, the dispatch intents);
  `engine/crates/conformance/src/steps/request_fixture.rs` and the two cases of decision 3.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor c1d09afc HEAD; echo "main $?"
test -e engine/crates/model-check; echo "crate $?"
grep -c 'model-check' engine/Cargo.toml
grep -c 'name = "stateright"' engine/Cargo.lock
ls conformance/conventions/requests/unknown-write/case.yaml conformance/conventions/requests/repeat-failed-earlier-unanswered/case.yaml | wc -l | tr -d ' '
grep -c '^pub const \(ONCE\|RIGHT\|UNKNOWN\):' experiments/012-model-checking/src/operations.rs
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
(cd experiments/012-model-checking && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "stateright $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `crate 1`, `0`, `0`, `2`, `3`, `engine 0`, `stateright 0` and `0`. Another
`stateright` line means stateright or one of its crates is not in the cache.

## Scope

Create `engine/crates/model-check/`. Change `engine/Cargo.toml` (its members alone),
`engine/Cargo.lock` (by cargo, decision 2), `engine/README.md` (decision 10) and
`engine/crates/conformance/src/` for decision 3's one function. Change nothing else: not `spec/`,
`conformance/`, `experiments/` (012 above all), `packages/`, `tools/`, `.github/`, `docs/`,
`ROADMAP.md`, the core's, the loader's or the shell's crate, or another `Cargo.toml`. Put
anything else that needs changing in your report.

## Done when

Each line in the first group prints otherwise on `main` at `c1d09afc` ("today") and passes after.

```sh
grep -c '"crates/model-check"' engine/Cargo.toml  # 1; today 0
grep -A1 '^name = "stateright"$' engine/Cargo.lock | grep -c '^version = "0.31.0"$'  # 1; today 0
test -f engine/crates/model-check/README.md; echo "readme $?"  # readme 0; today readme 1
grep -c 'conduit-model-check' engine/README.md  # 1 or more; today 0
grep -ci 'stateright' engine/README.md  # 1 or more; today 0
grep -rhoE --include='*.rs' 'conduit_core::step(_owned)?\(' engine/crates/model-check/ 2>/dev/null | wc -l | tr -d ' '  # 1 or more; today 0
(cd engine && cargo test --release -p conduit-model-check --test operations -- --list 2>/dev/null | grep -cE '^(plain_holds|deduplicating_holds|seeded_simulations_hold|at_most_once_control_refutes|no_wrong_outcome_control_refutes|unknown_stays_unknown_control_refutes|printouts_reproduce|truncated_search_gives_no_verdict): test$')  # 8; today 0
o=$(mktemp); (cd engine && cargo test --release -p conduit-model-check -- --nocapture --test-threads=1) >"$o" 2>&1; echo "tests $?"  # tests 0; today an error, no such package
grep -cE ' refuted: [a-z-]+/[A-Z]+, (bfs|simulation seed [0-9]+), [0-9]+ actions$' "$o"; rm -f "$o"  # 3 or more; today 0
```

Then run `git add -N engine/crates/model-check` so the checks see the new files. These pass, as
on `main`; run the whole suite once, last, then `git reset -q`, and stage nothing:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
h=$(cd engine && rustc -vV | sed -n 's/^host: //p'); (cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "offline $?"  # offline 0
python3 -c 'import tomllib,subprocess as s;p=lambda t:{(x["name"],x["version"]) for x in tomllib.loads(t)["package"]};o=p(s.run(["git","show","HEAD:engine/Cargo.lock"],capture_output=True,text=True).stdout);n=p(open("engine/Cargo.lock").read());print(sorted({a for a,_ in o-n}-{"js-sys","web-sys","wasm-bindgen","wasm-bindgen-futures","wasm-bindgen-macro","wasm-bindgen-macro-support","wasm-bindgen-shared"}))'  # []
python3 tools/set_names.py --check | grep -o 'Errors: [0-9]*'  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
python3 tools/check_model_account.py | tail -1  # Errors: 0
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --porcelain -uall | cut -c4- | grep -vE '^(engine/crates/model-check/|engine/crates/conformance/src/|engine/Cargo\.(toml|lock)$|engine/README\.md$)' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, once, last
```

Your report gives, per decision, what you built; the lock's added and moved packages; how the
starting states are built; per configuration and part, each verdict, unique states, depth and
seconds; each control's header line and trace length; each simulation's seeds and counts; any
projection decision 8 names; the crate's test time on the Mac; `own_choices`; and every check's
last line. The coordinator runs `tools/gate.py` and merges after the verifier's and the reader's
reads.

## What later slices add

Named here and not in this order; three to five orders in all (DEC-005):

- **Restarts**: the engine's (`Recover`) and the implementation's, queues kept, and a duplicated
  dispatch (012's parts C and D).
- **Two machines**: a cut and its repair between the engine and an implementation on another
  node, and a lost machine (012's parts P and M, MC4), with S3's enrollment and placement.
- **More promises**, each reusing 012's property: reconciliation (OPS-014), revisions (REV-002,
  REV-012), setup and retirement hooks (OPS-021), access over time (MC3).
- **The account**: how a real-core standing is recorded for G5, since 012's `account.yaml` is
  frozen but for DEC-033's marking; put to the owner with options when the second slice is
  written.
- The ROADMAP row DEC-005 names, "model checking of the real core", which the coordinator adds
  when this order merges.

## Record

Written on 2026-11-21 from the owner's "A: check the real core (Rec.)" of 2026-11-20 (DEC-005).
Not yet run.
