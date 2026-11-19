# Work order 204b: MC9a's state space measured, before its registration is revised

**This order runs on the second machine, the Linux node (WSL Ubuntu 24.04, x86_64, 24 CPUs).**
Nothing in it needs Linux: the coordinator places it there, beside order 204's runs, and schedules
it on the node only between the sign-in chain's orders. It changes no engine code.

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since this order adds and changes no case. It has
no network access and needs none: the crate's toolchain and its one direct dependency must already
be in the node's caches, and if they are not, stop and report it. Run one cargo command at a time,
except the census runs that decision 9 lets run together. Never print a whole large file or log:
`experiments/012-model-checking/README.md` has 7,638 lines and `src/pending.rs` 3,835, so read
them with `grep -n`, `sed -n`, `head`, `tail` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Do not run `claude`, `codex` or any client, and sign in nowhere.

## What and why

[Order 204](204-mc9a-model-run.md) wrote MC9a's model, `src/pending.rs`, and stopped under its
decision 9: the sign-in part's Correct/Base row reaches the registered ceiling of 400,000 unique
states, and in a reader's probe it still truncated at the harness's maximum of 1,000,000 (26.2 s
wall, peak 4,919 MiB). Its readers found model defects, R204-4 to R204-7, which its fixer fixed on
the branch `wo/204-mc9a-model-run`, and order 204 was not merged. Its Record, on that branch, is
the source of every figure in this paragraph. A bound changes only "as a dated revision before
that run", in the registration's own words, so before anyone revises MC9a this order measures the
space the revision must fit: how many states each sign-in row has, which of the model's choices
multiply them, how much of the count is the ledger's order alone, and how fast the harness checks
states at this size. It records the measurement beside the registration, and it evaluates no
property: nothing it records is a verdict.

**Who decided what.** Every decision below is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): that a measuring order comes before the
revision; that it carries order 204's model and Record to `main`, with the model's checking tests
ignored until the revision; the census, its caps, its ablations and projections, its output lines
and the record's form; that the measurement reads no property; and leaving the revision, the
bound it chooses and the run proper to the coordinator (decision 11). Apply them as written and
report them under `decisions` as the coordinator's.

**What it carries from order 204's branch.** From commit `49b96ec4`, the branch's head, three
files and nothing else of its history:

- `experiments/012-model-checking/src/pending.rs`, the model with the fixes for R204-4 to R204-7,
  byte for byte, then changed only by this order's additions (decisions 2, 5 and 6): no line of it
  is removed or edited.
- the line `pub mod pending;` of `experiments/012-model-checking/src/lib.rs`.
- order 204's Record, at the foot of [order 204's file](204-mc9a-model-run.md), as its fixer
  wrote it. It records an order not merged and says so; it states what order 204's readers saw of
  MC9a's outcomes in their probes, which the revision must name.

It carries none of order 204's logs, its `run.py` or its four checkpoint commits. Once this order
merges, the branch `wo/204-mc9a-model-run` holds nothing that `main` lacks, and the model run
proper is a later order on `main`, written after the revision (decision 11).

**Read first:** `AGENTS.md`; this order; in `experiments/012-model-checking/README.md`, lines 1 to
60 (the harness and its rules) and, in the MC9a section, "MC9a nodes, scripts and bounds" and
"MC9a configurations and fault windows" (`grep -n '^### MC9a'`, then `sed -n` in parts of at most
200 lines); `src/harness.rs` whole (880 lines); in `src/pending.rs` after decision 2's checkout,
lines 1 to 185 and the `sign_in` module's types, `local_actions` and `local_step`
(`grep -n 'fn local_actions\|fn local_step\|pub struct Data\|enum Local\|const STAGES'`); and
order 204's decisions 5 to 10 and its Record, in its file once decision 2 has carried the Record.
You need not read `spec/`.

## The decisions it applies

### Before any code

1. **The base and the node.** From the repository root of your worktree, run these before
   anything else. Each prints what its comment says. If one prints anything else, change nothing
   and report which: the coordinator dispatches this order only from a base on which each prints
   that, and makes commit `49b96ec4` available in the node's clone before it dispatches.

   ```sh
   git status --porcelain | grep -c .                                       # 0
   git merge-base --is-ancestor c097ec81 HEAD; echo $?                      # 0
   git diff --quiet 75944df4 HEAD -- experiments/012-model-checking/src experiments/012-model-checking/Cargo.toml experiments/012-model-checking/Cargo.lock; echo $?   # 0
   git diff --quiet 7eed5a4d HEAD -- experiments/012-model-checking/README.md; echo $?   # 0
   git diff --quiet 6ce1f97b HEAD -- docs/work-orders/204-mc9a-model-run.md; echo $?     # 0
   git cat-file -t 49b96ec4                                                 # commit
   git rev-parse 49b96ec4:experiments/012-model-checking/src/pending.rs     # b032690efc55205519bbcd1c741a0ef1fbba90ec
   git rev-parse 49b96ec4:experiments/012-model-checking/src/lib.rs         # 657a4c6e7a352f3f05882e97558961d91f02eda9
   git rev-parse 49b96ec4:docs/work-orders/204-mc9a-model-run.md           # e1c30a54254e7aea638dfc9027142af266a199f6
   git diff 6ce1f97b 49b96ec4 -- docs/work-orders/204-mc9a-model-run.md | grep -c '^-[^-]'   # 0
   uname -m                                                                 # x86_64
   nproc                                                                    # 24
   ```

   `c097ec81` is `main` when this order was written; `75944df4` is order 201's merge, after which
   nothing in the experiment's `src/` or Cargo files has changed; `7eed5a4d` is the merge of MC9a's
   amendment of 2026-11-19, the last change to the README; `6ce1f97b` is the base onto which
   order 204's branch was last rebased. Then this, whose output you write down as SPEC and do not
   stop on: it lists the files under `spec/` changed since order 204's last base. The measurement
   counts the states of the model as order 204 wrote it to the registration, whatever `spec/` now
   says; the record names these files, since a changed promise that MC9a reads makes its rows "text
   changed, not rerun" until MC9a is registered again (D29), which is the coordinator's to judge.
   Order 207 has already changed `spec/promises/pending.yaml`, a text MC9a reads: the measurement
   counts the pre-207 model, and the revision must also register against order 207's text.

   ```sh
   git diff --name-only 6ce1f97b HEAD -- spec | tr '\n' ' '; echo
   ```

   Then, from `experiments/012-model-checking/`:

   ```sh
   rustup toolchain list | grep -c '^1\.97\.1-x86_64-unknown-linux-gnu'    # 1
   rustup component list --toolchain 1.97.1 --installed | grep -c -e '^rustfmt' -e '^clippy'   # 2
   cargo fetch --locked --offline; echo $?                                  # 0
   ```

   Write `git rev-parse --short=8 HEAD` down as BASE; decision 10 records it.

2. **Carry order 204's three files.** From the repository root:

   ```sh
   git checkout 49b96ec4 -- experiments/012-model-checking/src/pending.rs experiments/012-model-checking/src/lib.rs docs/work-orders/204-mc9a-model-run.md
   git reset -q
   ```

   (`git reset -q` unstages them: stage nothing.) Then, in `src/pending.rs`, give each of the six
   tests that run a model check, `sign_in_table`, `actions_table`, `sign_in_first_refutations`,
   `actions_first_refutations`, `difference_traces` and `finding_witnesses`, the line

   ```rust
   #[ignore = "MC9a waits for its dated revision after order 204b's measurement"]
   ```

   directly after its `#[test]`. `configuration_and_property_inventory` stays a plain test. These
   six would truncate or compute MC9a's verdicts in every ordinary run, the merge's local gate
   among them; the run proper removes the six lines. Change no other line of order 204's code.

### The census

3. **`src/measure.rs`, a state census that reads no property.** One new file, beginning
   `//! A state census: distinct states counted over a harness's transitions, reading no property.`,
   and one line `pub mod measure;` in `src/lib.rs`, between `pub mod mc2;` and
   `pub mod operations;`. It imports only `crate::harness`, `stateright` and `std`, and changes
   nothing in the harness. Its test names `crate::records::record` by its full path, with no `use`,
   and that is the one reference outside the three crates. It holds:

   - `pub struct Limits { pub states: usize, pub seconds: u64 }`;
   - `pub enum Stop { Exhausted, StateCap, TimeCap }`, whose output words are `exhausted`,
     `state cap` and `time cap`;
   - `pub struct Census` with at least `pub states: usize`, `pub stop: Stop`, `pub seconds: f64`,
     `pub classes: Vec<usize>`, `pub projections: Vec<usize>` and
     `pub kinds: std::collections::BTreeMap<String, (u64, u64)>`;
   - `pub fn census<P: Protocol>(model: &Harness<P>, prune: &dyn Fn(&System<P>, &mut Vec<Action<P::Local>>), class: &dyn Fn(&System<P>) -> usize, projections: &[&dyn Fn(&System<P>) -> u64], limits: Limits) -> Census`.

   `census` explores depth first, with an explicit stack, from `Model::init_states`, through
   `Model::actions` and `Model::next_state` alone: the harness's own transitions, every fault and
   budget as the configuration has them. It never calls `Model::properties`, a property's
   condition or any checker. At each state it lists the enabled actions, calls `prune` on the list
   (a measurement's restriction; the identity for a registered row), and follows each action that
   remains. A state is identified by its fingerprint, the `u64` that `DefaultHasher::new()` gives
   for it, kept in a `HashSet<u64>` made `with_capacity(limits.states)`; a state is counted once,
   when its fingerprint is first inserted. For each newly counted state it adds one to
   `classes[class(state)]`, growing the vector as needed, and inserts each projection's value into
   that projection's own set, also made `with_capacity(limits.states)`; `projections[i]` is the size
   of set `i` at the end. For each followed action it adds one to `kinds[kind].0`, and one to
   `kinds[kind].1` when the action leads to a newly counted state, where `kind` is: for `Deliver`,
   `Drop` and `Duplicate`, the action's name, the source and destination nodes' first letters as
   `Q->E`, and the message's variant name; for `Local`, `Local`, the node's first letter and the
   local action's variant name; for the others, the action's variant name; each part separated by
   one space (so `Deliver Q->E Request`, `Local E Expire`, `Crash`). Names come from the `Debug`
   text, cut at the first `(`, `{` or space. It stops with `StateCap` when the count reaches
   `limits.states`, with `TimeCap` when `limits.seconds` have passed (checked at least every 65,536
   new states), and otherwise with `Exhausted`. `seconds` is the elapsed time.

   The fingerprint is a measurement's: two states share one with probability below
   n²/2^65 for n states, which the record states. A state count found with a cap's stop is a lower
   bound. The order of a depth-first search changes the second number of `kinds`, and no other
   count of an exhausted census: the first count is independent of the search's order only where
   the census exhausted; where it stopped at a cap, both counts depend on it.

   Its `#[cfg(test)] mod tests` holds one plain test, `census_counts_as_the_checker`: it prints
   `MC9a measurement test census_counts_as_the_checker` first, then, for each `Part` of
   `crate::records::record::Part::ALL` in order, counts the Correct model with `census` (the
   identity `prune`, `class` always 0, no projection, limits 1,000,000 states and 60 seconds) and
   with `check_all`, prints
   `MC9a calibration Records|Correct/<Part>|<census states>|<checker states>`, and asserts the
   census exhausted and the two counts equal. MC7b's dated results recorded those rows; this test
   is what lets the record compare a census with the harness's counts. If it fails, stop and report
   it: no census result is then recorded.

4. **Where the MC9a census lives.** In `src/pending.rs`, one `#[cfg(test)] mod census` at the end
   of the `sign_in` module and one at the end of the `actions` module, so that each can read its
   part's private state. They may import `crate::measure` besides what the file already imports.
   Every test in them is `#[ignore = "measurement for order 204b; run through its run script"]`,
   prints `MC9a measurement test <name>` before anything else (libtest's `test <name> ... ` has no
   newline under `--nocapture`), and runs each of its rows in turn. No test there calls
   `assert_refuted`, `assert_reached`, `assert_holds` or a property's condition, and none prints a
   `Report`, its `broken` or its `missing_witnesses`.

5. **The sign-in census**, in `sign_in::census`. Every census of a registered row uses
   `model(v, c)` unchanged. `class` is Q's script index, `s.nodes[Q].data.q.script.stage`, from 0
   to 8: index 0 is the README's stage 1, indices 1 and 2 are stage 2's two requests, indices 3 to
   7 are stages 3 to 7, and 8 is the script's end. For each census it prints one line

   ```text
   MC9a census Sign-in|<Row>|<states>|<stop>|<seconds, 1 decimal>
   ```

   and, for a census of `census_correct`, `census_without` or `census_fixed`, one line
   `MC9a stages Sign-in|<Row>|<n0>,<n1>,<n2>,<n3>,<n4>,<n5>,<n6>,<n7>,<n8>`, each `n` the count of
   counted states at that index (0 where none). `<Row>` is `<Variant>/<Configuration>` by their
   `Debug` names, and for an ablation `Correct/Base, <ablation>`. The tests:

   - `census_correct`: Correct in each configuration of `Configuration::ALL`, in order, with limits
     200,000,000 states and 600 seconds. After Base's census line it also prints, for each kind in
     the `BTreeMap`'s order, `MC9a kind Sign-in|Correct/Base|<kind>|<transitions>|<to a new state>`.
   - `census_without`: Correct/Base with one choice removed, in this order, each with limits
     200,000,000 states and 300 seconds. Each `prune` removes from the list only the actions it
     names:
     - `without expiry`: every `Local(E, Expire(_))`;
     - `page as e only`: every `Local(H, CompletePage(_, Other))`;
     - `page as O only`: each `Local(H, CompletePage(id, LowerE))` where the list also holds
       `Local(H, CompletePage(id, Other))` (W2's page, which offers only `e`, keeps it);
     - `one input request`: every `Local(E, OfferSecond(_, true))`;
     - `sign-out succeeds`: every `Local(H, Undo(_, false))`;
     - `no lapse`: every `Local(H, Turn(_, false))`;
     - `bracket in order`: `Local(Q, Send(X_SIGN))` while `R_SIGN` is unsent, and
       `Local(Q, Send(FINAL_SIGN))` while `RETIRE` is unsent;
     - `internal steps first`: where the list holds any `Local(E, PassReturn(_))`,
       `Local(E, Poll(_, _))` or `Local(H, ApplyReturn(_))`, every action but the first of those in
       the list's order. This one is a measure only: it drops interleavings that no registered
       sentence drops, and no revision may adopt it without registering it as such.
   - `census_fixed`: Correct/Base with `choices fixed`, the six removals `without expiry`,
     `page as e only`, `one input request`, `sign-out succeeds`, `no lapse` and `bracket in order`
     together; then the same with one of them given back, in this order: `choices fixed, with
     expiry`, `choices fixed, with either page`, `choices fixed, with a second input request`,
     `choices fixed, with a failed sign-out`, `choices fixed, with a lapse`, `choices fixed, with
     either bracket order`; then `choices fixed, internal steps first`. Limits 200,000,000 states
     and 300 seconds each.
   - `census_projections`: Correct/Base once more, labelled `Correct/Base, projections`, with
     limits 100,000,000 states and 600 seconds and three projections, each the fingerprint
     (`DefaultHasher::new()`) of: `without the ledger`, the state's `nodes`, `links`, `cut` and
     `used`; `ledger unordered`, those four with the ledger's events as a multiset, each event
     fingerprinted alone and the fingerprints sorted; and `ledger alone`, the ledger. After its
     census line it prints, for each in that order,
     `MC9a projection Sign-in|Correct/Base|<projection>|<distinct values>`. The first is the floor
     that no change to the ledger's form alone can go below; the second, what a ledger that kept
     its entries but not their order would leave.
   - `census_mutants_own` and `census_mutants_base`: each mutant of `BROKEN`, in order, in its own
     configuration, and then CopyAsNew, ConflictForever, ReplayState, SavedReturn and VolatileHold
     in Base, as order 204's `sign_in_table` orders them; limits 200,000,000 states and 180 seconds
     each. These run only if `census_correct`'s Correct/Base line ends in `exhausted`; otherwise
     they are not run, and the record says so.
   - `throughput_400k` and `throughput_1m`: the harness itself at this model's size. Each takes
     `model(Correct, Base)`, sets its `config.unique_state_ceiling` to 400,000 or 1,000,000 and its
     `config.timeout` to 600 seconds (a measurement's configuration, not a registered one), and
     calls `check_all` inside `std::panic::catch_unwind` with `AssertUnwindSafe`, timing the call.
     Where it panics, it asserts that the message contains `run truncated` and prints
     `MC9a throughput Sign-in|Correct/Base|<ceiling>|<threads>|<seconds, 3 decimals>|truncated`;
     where it returns, it prints the same line ending `|exhausted at <unique states>` and prints
     nothing else of the report. `<threads>` is `crate::harness::threads()`.

6. **The actions census**, in `actions::census`: one test, `census_rows`, over order 204's 17
   actions rows in `actions_table`'s order (Correct in each configuration, each mutant of `BROKEN`
   in its own configuration, then VolatileScope, AnswerTwice and VolatileAnswers in Base, as
   `actions_rows` gives them), each with limits 200,000,000 states and 180 seconds, printing
   `MC9a census Actions|<Variant>/<Configuration>|<states>|<stop>|<seconds, 1 decimal>`. No
   stages, kinds or projections.

7. **The measurement reads no outcome.** Nothing this order runs prints or records whether a
   property holds or a witness is reached: the census never evaluates one, and a throughput run's
   check stops at its ceiling before any verdict is read, or, where it does not, prints the count
   alone. Do not run order 204's six ignored tests, alone or with `--include-ignored`, and do not
   run `cargo test` with `--ignored` except through decision 8's script, which names one test.

### Running it on the Linux node

8. **The run script.** Write this to `experiments/012-model-checking/target/mc9a-census/run.py`
   (git ignores `target/`) and run every test of decisions 3, 5 and 6 through it, from
   `experiments/012-model-checking/`, as
   `/usr/bin/python3 -I target/mc9a-census/run.py <test path> [threads]`, where the test path is
   `measure::tests::census_counts_as_the_checker`, `pending::sign_in::census::<name>` or
   `pending::actions::census::census_rows`, and threads, default 2, sets `CONDUIT_MC_THREADS`
   (it matters only to the throughput runs). Build first, uncapped, with
   `CARGO_BUILD_JOBS=4 cargo test --release --no-run`.

   ```python
   import os, resource, subprocess, sys, time
   test = sys.argv[1]
   threads = sys.argv[2] if len(sys.argv) > 2 else "2"
   cap = 12 << 30
   def limit():
       resource.setrlimit(resource.RLIMIT_AS, (cap, cap))
   env = dict(os.environ, CONDUIT_MC_THREADS=threads, CARGO_BUILD_JOBS="4")
   ignored = [] if test.startswith("measure::") else ["--ignored"]
   args = (["cargo", "test", "--release", "--lib", "--"] + ignored
           + ["--exact", "--nocapture", "--test-threads=1", test])
   log = f"target/mc9a-census/{test.rsplit('::', 1)[-1]}-{threads}.log"
   start = time.monotonic()
   with open(log, "w") as out:
       code = subprocess.run(args, stdout=out, stderr=subprocess.STDOUT, env=env,
                             preexec_fn=limit).returncode
   wall = time.monotonic() - start
   peak = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss // 1024
   summary = f"{test} threads {threads}: exit {code}, {wall:.1f} s wall, peak {peak} MiB"
   print(summary)
   with open(log, "a") as out:
       out.write(summary + "\n")
   sys.exit(1 if code or peak >= 8192 else 0)
   ```

9. **The runs, in this order.** Each must exit 0 and its log's `test result` line must read
   `ok. 1 passed; 0 failed`.

   1. `measure::tests::census_counts_as_the_checker`.
   2. `census_correct`, `census_without` and `census_fixed`, which may run at the same time, three
      processes started together and waited for (each census is one thread; each holds about
      2.3 GiB for its fingerprints). Nothing else of this order runs while they do; the
      coordinator, not the worker, keeps the node free of other orders during decision 9.
   3. `census_projections`, alone (four sets: about 4.5 GiB).
   4. `census_mutants_own` and `census_mutants_base`, together, where decision 5 says they run.
   5. `pending::actions::census::census_rows`.
   6. `throughput_400k` with threads 2, then `throughput_400k` with threads 1, then
      `throughput_1m` with threads 2, each alone.

   Expect at most about three hours if every census reaches its cap, and less where they exhaust;
   the caps bound every run. A run that fails for memory (an allocation failure in its log), prints a
   peak of 8,192 MiB or more, or panics in the model (for instance the harness's queue bound,
   "model exceeded its stated queue bound") is this order's stop condition: keep the code, write
   no record, and report the run, its row and the line it printed. A queue-bound panic is a fault
   of the model or of its registered bound, and goes in `findings`. A census that reaches a cap is
   not a failure: it is recorded as such. Run each test once; if one must run again, the record is
   written from the last run's log alone.

### The record

10. **The README**, `experiments/012-model-checking/README.md`, gains, after its last line,
    "stands as written.", one blank line and one section, each line of prose at most 100
    characters and each link whole on one line. Nothing above it changes, line 3 included: MC9a
    is still registered and not run.

    `### MC9a state-space measurement — DATE`, DATE being `date -u +%F` on the day of the runs. In
    order:

    - A paragraph: "Measured DATE under work order 204b on the Linux node (WSL Ubuntu 24.04, x86_64,
      24 CPUs), from `main` `BASE`, on the model that order 204 wrote, as its branch left it at
      `49b96ec4` with the fixes for its readers' findings R204-4 to R204-7, which order 204b
      carried to `main`. This is not a run of MC9a: the census reads no property, and each
      harness run below stops at its ceiling before any verdict is read or reports its count
      alone, so nothing in this section is a verdict, and the registration above is unchanged.
      Order 204's Record states what
      its readers saw of MC9a's outcomes before this measurement." Then one sentence naming SPEC:
      either "No file under `spec/` changed between `6ce1f97b` and `BASE`." or "These files under
      `spec/` changed between `6ce1f97b` and `BASE`, with order 207: <the names, each in backticks>;
      the census counts the model as written to the registration, before order 207."
    - A paragraph on the method: the census of `src/measure.rs`, depth first over the harness's
      own transitions, distinct states counted by a 64-bit fingerprint, with no ceiling but each
      run's caps; a count with the stop `state cap` or `time cap` is a lower bound; for n states,
      two fingerprints coincide with probability below n²/2^65; and the calibration, in one
      sentence from the log: "On the eight Correct rows of MC7b's records model, the census
      counted the states that the checker counts: N of 8 equal.", N from the `MC9a calibration`
      lines.
    - **Census.** A table with the header `| Part | Row | States | Stop | Seconds |` and the
      alignment row `| --- | --- | ---: | --- | ---: |`, one row for each `MC9a census` line of
      the logs, in decision 9's order of runs and each log's order:
      `| <Part> | <Row> | <states> | <stop> | <seconds> |`, every value copied from that line. Then
      a list naming what each ablation removes, in decision 5's words, and a sentence saying that
      `internal steps first` is a measure only. Where the mutants did not run, a sentence: "The
      sign-in mutants were not measured: Correct/Base did not exhaust within its caps."
    - **Stages.** A sentence mapping the index to the README's stages as decision 5 does, then a
      table with the header `| Row | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |` and the alignment row
      `| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |`, one row for each
      `MC9a stages` line: `| <Row> | <n0> | ... | <n8> |`.
    - **Projections of Correct/Base.** A table with the header `| Projection | Distinct values |`,
      one row for each `MC9a projection` line, then the `Correct/Base, projections` census row's
      stop, in a sentence.
    - **Transitions by kind, Correct/Base.** A table with the header
      `| Kind | Transitions | To a new state |`, one row for each `MC9a kind` line, and a sentence
      saying that the second count depends on the search's order and stating whichever applies to
      Correct/Base's stop: the first count is independent of the search's order only where the
      census exhausted; where it stopped at a cap, both counts depend on it.
    - **The harness at this size.** A table with the header
      `| Ceiling | Threads | Seconds | Ended | Peak MiB |`, one row for each `MC9a throughput`
      line, its peak from the summary line of the same log, and a sentence saying that these runs
      use a measurement's configuration (600 seconds, and the ceiling shown), not a registered
      one.
    - **Runs.** A table with the header `| Test | Threads | Wall seconds | Peak MiB |`, one row for
      each log's summary line.

    No line of the section states a count, a time or a peak that the logs do not hold, and none
    says which bound MC9a should take: that is the revision's (decision 11).

11. **What this order does not do.** It does not revise MC9a's registration or any bound, table,
    property or mutant of it; does not run MC9a or write order 204's results; and changes no engine
    code, no `spec/`, no case, no harness, no other model, and not `Cargo.toml`, `Cargo.lock`,
    `account.yaml`, `G5.md`, `ROADMAP.md`, `IN_FORCE.md` or any order but order 204's file, which
    gains its Record (decision 2). From this record the coordinator writes MC9a's dated revision
    (for instance a split of the sign-in script, a fixed choice per row, a different ledger, or a
    different bound and the harness change it needs), registered before the next run, and then the
    order that runs MC9a from its controls on, which removes decision 2's six `#[ignore]` lines.

## Scope

Change only:

- `experiments/012-model-checking/src/pending.rs` (new on `main`, carried and added to:
  decisions 2, 4, 5 and 6);
- `experiments/012-model-checking/src/lib.rs` (two lines: `pub mod measure;` and
  `pub mod pending;`);
- `experiments/012-model-checking/src/measure.rs` (new: decision 3);
- `experiments/012-model-checking/README.md` (decision 10);
- `docs/work-orders/204-mc9a-model-run.md` (its Record, carried: decision 2).

Report anything else that needs changing.

## Done when

From `experiments/012-model-checking/` on the Linux node, one at a time, each passes:

```sh
CARGO_BUILD_JOBS=4 cargo fmt --check
CARGO_BUILD_JOBS=4 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=4 cargo test --release          # last test result line: ok, 0 failed
```

Decision 9's runs each exit 0, after the last of them. Then each prints what its comment says:

```sh
grep -h -c '^test result: ok\. 1 passed; 0 failed' target/mc9a-census/*.log | sort -u   # 1
grep -h '^MC9a calibration ' target/mc9a-census/*.log | awk -F'|' '$3 == $4' | grep -c .   # 8
grep -h '^MC9a census Sign-in|Correct/' target/mc9a-census/census_correct-2.log | grep -c -v ', '   # 6
grep -h '^MC9a census Sign-in|Correct/Base, ' target/mc9a-census/census_without-2.log | grep -c .   # 8
grep -h '^MC9a census Sign-in|Correct/Base, ' target/mc9a-census/census_fixed-2.log | grep -c .     # 8
grep -h '^MC9a stages Sign-in|' target/mc9a-census/*.log | grep -c .                     # 22
grep -h '^MC9a projection Sign-in|' target/mc9a-census/*.log | grep -c .                 # 3
grep -h '^MC9a census Actions|' target/mc9a-census/*.log | grep -c .                     # 17
grep -h '^MC9a throughput Sign-in|' target/mc9a-census/*.log | grep -c .                 # 3
grep -h -i -E 'holds|refuted|reached|broken|missing' target/mc9a-census/*.log | grep -c -v -e '^test ' -e 'run truncated'   # 0
grep -c '^pub mod measure;$' src/lib.rs; grep -c '^pub mod pending;$' src/lib.rs        # 1 and 1
grep -c '#\[ignore = "MC9a waits for its dated revision after order 204b' src/pending.rs   # 6
grep -E '^\s*(pub )?use ' src/measure.rs src/pending.rs | grep -v -e 'crate::harness' -e 'crate::measure' -e 'stateright' -e 'std::' -e 'super::' | grep -c .   # 0
```

The stage count is 6 Correct rows, 8 of `census_without` and 8 of `census_fixed`. The mutants'
logs, where they ran, hold 14 and 5 `MC9a census Sign-in|` lines. From the repository root, order
204's code is carried and only added to, its Record carried whole, and the record matches the logs:

```sh
git show 49b96ec4:experiments/012-model-checking/src/pending.rs | diff - experiments/012-model-checking/src/pending.rs | grep -c '^<'   # 0
git diff 49b96ec4 -- experiments/012-model-checking/src/lib.rs | grep -c '^-[^-]'       # 0
git diff --quiet 49b96ec4 -- docs/work-orders/204-mc9a-model-run.md; echo $?            # 0
```

```sh
/usr/bin/python3 -I - <<'EOF'
import glob, re
logs = {p: open(p, encoding="utf-8").read() for p in sorted(glob.glob(
    "experiments/012-model-checking/target/mc9a-census/*.log"))}
text = open("experiments/012-model-checking/README.md", encoding="utf-8").read()
part = text[text.index("### MC9a state-space measurement — "):]
lines = [l for log in logs.values() for l in log.splitlines()]
def count(prefix, form):
    found = [l[len(prefix):].split("|") for l in lines if l.startswith(prefix)]
    return len(found), sum(part.count(form(f)) == 1 for f in found)
c = count("MC9a census ", lambda f: f"| {f[0]} | {f[1]} | {f[2]} | {f[3]} | {f[4]} |\n")
s = count("MC9a stages Sign-in|", lambda f: "| " + f[0] + " | " + " | ".join(f[1].split(",")) + " |\n")
p = count("MC9a projection Sign-in|", lambda f: f"| {f[1]} | {f[2]} |\n")
k = count("MC9a kind Sign-in|", lambda f: f"| {f[1]} | {f[2]} | {f[3]} |\n")
t = count("MC9a throughput Sign-in|", lambda f: f"| {f[1]} | {f[2]} | {f[3]} | {f[4]} |")
runs = [re.fullmatch(r"(\S+) threads (\d+): exit 0, ([0-9.]+) s wall, peak (\d+) MiB",
                     log.splitlines()[-1]) for log in logs.values()]
r = sum(bool(m) and part.count(f"| {m[1]} | {m[2]} | {m[3]} | {m[4]} |\n") == 1 for m in runs)
print(f"census {c[0]} {c[1]}; stages {s[0]} {s[1]}; projections {p[0]} {p[1]}; "
      f"kinds {k[0]} {k[1]}; throughput {t[0]} {t[1]}; runs {len(runs)} {r}")
EOF
```

It prints each pair equal: `census C C; stages 22 22; projections 3 3; kinds K K; throughput 3 3;
runs N N`, with C the number of census lines (40, or 59 with the mutants) and N the number of
logs. Then, from the repository root, with the new files seen through an intent-to-add that is
undone after, each passes:

```sh
F=experiments/012-model-checking/README.md
git add -N experiments/012-model-checking/src/pending.rs experiments/012-model-checking/src/measure.rs
python3 tools/check_docs.py                      # last line: Errors: 0
python3 tools/check_roadmap.py                   # last line: Errors: 0
python3 tools/check_model_account.py             # last line: Errors: 0
python3 tools/check_private_paths.py             # last line: Errors: 0
git diff --check                                 # prints nothing
git reset -q experiments/012-model-checking/src/pending.rs experiments/012-model-checking/src/measure.rs
git status --short                               # the five files of Scope, and nothing else
git diff --numstat -- $F | awk '{print $2}'      # 0
grep -c '^### MC9a' $F                           # 13
grep -c '^### MC9a state-space measurement — 20[0-9-]*$' $F   # 1
sed -n 3p $F | grep -c 'MC9a registered 2026-11-18 under work order 201, not run\.$'   # 1
git diff -U0 -- $F experiments/012-model-checking/src | grep '^+' | /usr/bin/python3 -c 'import sys; print(sum(chr(64) in l for l in sys.stdin))'   # 0
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" --exclude-dir=target experiments/012-model-checking   # prints nothing
```

Do not stage anything. Report under `summary` the base, the date, SPEC, Correct/Base's count and
stop, every census line's row, count and stop, the projections, the throughput lines and each
run's wall time and peak; under `findings` any queue-bound panic or other stop of decision 9, and
anything the census showed of the model that a registered sentence does not say (a choice the
model makes that the registration does not state, for instance), each citing its line; under
`own_choices` every detail of decisions 3 to 6 you had to choose; and every check's last line.
A fresh Codex verifier and then a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2, as rulings 85 and 87 apply it), and the
merge waits on the local gate ([ruling 80](../proposals/ruling-80/README.md), point 10). The admin
writes this order's Record at the foot of this file when it merges.

## Record

Run on 2026-11-19, from about 17:00 to 18:28 UTC, on the Linux node from `main` `d91ef7b2` (the
sign-in chain written ahead to ruling 88's shape), in one round: the worker (4874 s) and a fresh
Codex verifier (311 s). The change is the five files of Scope. `src/pending.rs` is order 204's file
at `49b96ec4` with additions only: the actions census, `census_rows`, and the sign-in census, its
ablations, projections and throughput runs, and the six ignore lines of decision 2. `src/lib.rs`
gains the two module lines, `src/measure.rs` is new (the depth-first census, its 64-bit
fingerprints, caps, stages, kinds and projections, and the calibration test), the README gains the
dated section "MC9a state-space measurement — 2026-11-19", and order 204's file is carried
byte-equal. The worker ran decision 9's nine runs on the node, each exit 0. Correct/Base stopped at
its time cap at 98,138,450 states, a lower bound; the sign-in mutants were therefore not run, and
the section says so. The worker's own choices: one stack frame per state on the active path, with a
cursor through its retained actions, so the harness's list order is kept, and the state cap taking
precedence when both caps are reached together; Debug-derived labels for the kinds; the ablations as
ordered action filters, `page as O only` snapshotting the attempts with an `Other` choice before
filtering; projection fields fingerprinted as tuples, the events' fingerprints sorted with
repetitions kept; the default panic output for throughput, accepting either string payload; and a
type-only clippy change to the projection vector after the measurements, so the logs come from the
build before it. It wrote the three carried blobs directly because the node's index was read-only;
the result is byte-identical. Its one finding, R204b-1, an ambiguity: stage 4 leaves the scheduling
of the return, the account's update and polling open; the model splits applying the return into its
own H transition and gives E an explicit `OfferSecond(false)` step before the final poll, which the
registration does not state as a separate step.

The verifier found one must and four shoulds: the model can issue `rs2` and complete the invocation
before Q sends `v2`, a branch the worker's report omitted (the must); the `with a failed sign-out`
row cannot show a failed sign-out's cost under `page as e only`; the collision bound is stated
without its assumption of uniform independent fingerprints; the order's at-sign scan of the added
lines misses the untracked carried source, where seven at signs stand, all Rust pattern bindings
carried from `49b96ec4`; and the worker's report links to the node's absolute paths.

A fresh read-only Opus reader then verified the order (ruling 79, point 2, as ruling 87 applies
it), on the branch before its rebase. On the Mac it ran `check_docs`, `check_roadmap`,
`check_model_account` and `check_private_paths` (Errors: 0 each), `git diff --check
origin/main...HEAD` (clean), the order's greps on the README (13, 1 and 1), a width check of the new
section (no line over 100 characters), and a scan of the added lines for private paths, names and
at signs (none in the README or `measure.rs`; the seven in `pending.rs` are the carried bindings);
the diff held the five files of Scope, `pending.rs` differed from `49b96ec4` by added hunks only,
and order 204's file was byte-equal. On the node it ran `cargo fmt --check` and `cargo clippy
--offline --all-targets -- -D warnings` (exit 0 each), `cargo test --offline --release` (64 passed,
0 failed, 15 ignored), the order's greps over the logs and sources (each as specified), and the
order's README-to-logs script, which printed `census 40 40; stages 22 22; projections 3 3; kinds 16
16; throughput 3 3; runs 9 9`. It reran the calibration, `census_rows` and `census_fixed` into a
separate target directory, leaving the record's logs untouched; each reproduced its logged lines,
apart from seconds, on the committed build. It found no must and two shoulds. It confirmed the
verifier's must as a model fact, by tracing the code, but took it as a should: no census line
separates the branch, the code is order 204's, which decision 2 forbids editing, and no done-when
or recorded value is wrong. It confirmed the second, and refuted the other three as defects: the
bound is the standard birthday union bound, in the sentence decisions 3 and 10 prescribe, and the
calibration matched 8 of 8; the at signs are pattern bindings, syntax `main` already holds, and
`check_private_paths` reports no error (the scan's gap is the order's text, not the work); and the
added lines hold no private path, so only this Record must keep to repository-relative references.
It noted for the coordinator's revision that each time-capped row's count tracks its cap at about
160,000 states a second (about 98 million at 600 s, 48 to 51 million at 300 s), and that the search
took a single `Expire` transition in Correct/Base, so the capped ablation rows measure throughput,
not each choice's share; only the exhausted rows compare choices: `page as e only` (51,620,210),
`internal steps first` (3,795,818) and every `census_fixed` row.

- R204b-r1: the carried model lets the invocation end on v1's return after `rs2` is issued, with
  `v2` never sent. `PassReturn` passes v1, H takes the return and its result sets `can_offer`,
  `OfferSecond(true)` issues `rs2` while the program still waits, `ApplyReturn` installs the
  account, the initial poll needs only `return_passed` and on `e` completes the invocation, and the
  ending's reply clears Q's input, so `SendRetry(v2)` is never enabled. The registration says only
  that the caller then retries with v2 and that its result comes at the invocation's end.
- R204b-r2: under `page as e only` a failed sign-out cannot occur, so the row `choices fixed, with a
  failed sign-out` cannot add a state, and the record listed it without saying so; and the
  throughput sentence left out decision 10's "not a registered one".

The fixer rebased the branch onto `origin/main` at `791fe1f2` (order 048a-5ef refreshed for its
stacked dispatch) with the work-order identity, without conflict; the commits since the base touch
no file of this order. It applied each finding:

- R204b-r1: no code changes, as the reader asked. This Record names it as a second ambiguity beside
  R204b-1: the model lets the invocation end on v1's return after `rs2` is issued, with `v2` never
  sent. The coordinator's dated revision of MC9a then states whether `v2` must precede completion,
  or registers the branch.
- R204b-r2: after the ablation list in the README's Census part, the section adds "With `page as e
  only` kept, no poll finds `O`, so no sign-out is dispatched: `choices fixed, with a failed
  sign-out` equals `choices fixed`, as its census and stages lines show." The throughput sentence
  now reads "These runs use a measurement's configuration (600 seconds and the ceiling shown), not a
  registered one." Both are prose only: no table line changes, so the README-to-logs script's
  pairs are unaffected.

The fixer then ran, in the worktree on the Mac (aarch64), on the files of this commit before it was
made: from `experiments/012-model-checking/`, `cargo fmt --check` and `cargo clippy --offline
--all-targets -- -D warnings` (exit 0 each), `cargo test --offline --release` (64 passed, 0 failed,
15 ignored), and, one at a time, the calibration, `census_fixed` and `census_rows`, each naming its
one test (each 1 passed): the calibration's 8 lines all equal, and the 25 census lines and 8 stages
lines of the other two each matched the README's row exactly, so the counts reproduce off the node.
The nine logs stay on the node, so the README-to-logs script was not rerun; the fix changes no
table line. From the repository root, `python3 tools/check_docs.py`, `check_roadmap.py`,
`check_model_account.py` and `check_private_paths.py` (Errors: 0 each), and `git diff --check` and
`git diff --check origin/main...HEAD` (clean); no line of the new section or of this Record is over
100 characters. The gate's run on this order's squash, which this merge waits on, is the
coordinator's at the merge.
