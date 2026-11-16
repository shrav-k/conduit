# Work order 188: the vectors on the differential build, nightly and on request

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none:
no command it runs binds a port, and the feature it builds adds no crate to `engine/Cargo.lock`.
If cargo stops because it would fetch a crate, report that and stop. Run one cargo command at a
time, and never print a whole large file or log. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

The full review of 2026-11-14 (order 31 of [its orders](../proposals/full-review/orders.md))
found that the differential build is in no workflow, so that each statement that the vectors pass
on it rests on a run by hand
([FR-533](../proposals/full-review/findings/G6-outside-the-tree.md#fr-533), with its verifier's
corrections), and asked for a nightly vector run on that build
([FR-493](../proposals/full-review/findings/G4-who-verifies.md#fr-493), gate 8). Both findings
leave the decision to the coordinator.

[Order 070](070-build-state-copies.md) added the cargo feature `differential` to `conduit-core`
(`engine/crates/core/Cargo.toml` line 28), and features of the same name that turn it on to
`conduit-conformance` (`engine/crates/conformance/Cargo.toml` line 22) and `conduit-shell`. Under
it, `step_owned` (`engine/crates/core/src/lib.rs` lines 326 to 333) keeps the state it was given
and, after each transition, calls `differential::check` (`engine/crates/core/src/differential.rs`
line 134). That compares each consumer's tool list on the states before and after, and the change
notifications, with an independent computation that order 070 kept from before it made
publication lazy. A difference is an error that names the consumer (lines 153 and 203), and the
case fails. It is the check on order 070's lazy publication.

No workflow builds the feature: neither `.github/workflows/checks.yml` nor `nightly.yml` names it,
and [order 176](176-core-purity-enforced.md) left it so (its decision 4). The runs by hand are in
the Records: 440 vectors on both builds in order 070's, 491 in
[order 042's](042-build-tombstones-and-inventory.md#record), and 494 in the handover
(`docs/implementation/HANDOVER.md` line 157), which puts the differential run at about 2.4 hours
on the Mac. That run came before [order 071](071-conformance-manifest-pieces.md#record) made the
runner run cases on several threads. Order 071's Record, from the Windows computer (WSL), measured
the normal build at four threads in about a third of one thread's time, and at 24 threads the
runner ran out of memory. No differential run is recorded since. `main` at `2f33a53e` holds 501
cases.

`.github/workflows/nightly.yml` already runs once a day, at 06:17 UTC, and on request
(`workflow_dispatch`, lines 8 to 11). Its one job, `npm`, runs the engine's tests that fetch the
pinned filesystem server. This order gives it a second job, which runs the vectors on the
differential build, and says where that run is described. The review's order 3, on which this one
depends, is [order 152](152-ci-engine-shards-stand-alone.md), merged. The review places this order
beside the path to the pin, so that the pinned build has this evidence.

## The decisions it applies

1. **A second job in `nightly.yml`, not a new workflow.** The file already has both triggers.
   GitHub runs `workflow_dispatch` only for a workflow file that is on the default branch (its
   documentation; not checked here). Because this file is on it, the admin can start the branch's
   version by hand before merge, which a new file would not allow until it reached `main`. The
   schedule, `permissions`, `concurrency`, `env` and the `npm` job do not change.
2. **The job.** Add it after the `npm` job, at the end of the file, as written here, with one blank
   line before its comment:

   ```yaml
     # The conformance vectors on the differential build (work order 070). Four threads, stated
     # rather than taken from the processors: order 071 ran the vectors at four, and at 24 they
     # ran out of memory.
     differential:
       name: differential vectors
       runs-on: ubuntu-latest
       timeout-minutes: 300
       defaults:
         run:
           working-directory: engine
       steps:
         - uses: actions/checkout@v7
           with:
             persist-credentials: false
         - name: Install the toolchain that rust-toolchain.toml pins
           run: |
             rustup toolchain install --no-self-update --profile minimal
             rustup show active-toolchain
             cargo --version
         # The cache the checks workflow saves for engine/ on main; this job only reads it.
         - name: Restore the cargo cache
           uses: actions/cache/restore@v6
           with:
             path: |
               ~/.cargo/registry/index/
               ~/.cargo/registry/cache/
               ~/.cargo/git/db/
               engine/target/
             key: cargo-${{ runner.os }}-engine-${{ hashFiles('engine/Cargo.lock', 'engine/rust-toolchain.toml') }}
             restore-keys: cargo-${{ runner.os }}-engine-
         - name: cargo build --release -p conduit-conformance --features differential
           run: cargo build --release -p conduit-conformance --features differential
         - name: CONDUIT_VECTOR_THREADS=4 cargo run --release -p conduit-conformance --features differential
           env:
             CONDUIT_VECTOR_THREADS: "4"
           run: cargo run --release -p conduit-conformance --features differential
   ```

   - It restores the cache that `checks.yml` saves for `engine/` on `main`, as `npm` does (lines
     46 to 56), and saves none.
   - The build is a step of its own, so that the log gives the run's time apart from the build's.
     Each step's name is its command, as the `npm` job's last step is.
   - With no argument the runner takes the repository it was built in
     (`engine/crates/conformance/src/main.rs` lines 3 to 6) and runs every case.
   - Four threads: order 071's measure and recommendation, and the processors of `ubuntu-latest`
     for a public repository (four, with 16 GB of memory, by GitHub's documentation; not checked
     here). The number is stated, not taken from the processors, so that another runner image
     cannot change the memory the run needs.
   - `timeout-minutes: 300`: one thread on the Mac took about 2.4 hours, and GitHub ends a job on
     a hosted runner after six hours (its documentation; not checked here), so a run too slow for
     the night fails at the job's own limit first.
3. **The Mac's peak decides between four threads and two.** If the timed run of "Done when"
   peaks above 12 GiB (`maximum resident set size` above 12884901888 bytes), use 2 in place of 4
   in the step's `env` and in its name, and in decision 6's command. Replace the job's comment with
   these three lines:

   ```yaml
     # The conformance vectors on the differential build (work order 070). Two threads, stated
     # rather than taken from the processors: at four the Mac's peak passed 12 GiB, and at 24
     # order 071's runs ran out of memory.
   ```

   Say so in your report. Otherwise change nothing. The Mac's peak stands in for the runner's,
   which no one can measure before the manual run. The manual run is the test (see "Done when").
4. **The header says what both jobs are.** Replace lines 1 to 4 of `nightly.yml` with these five
   lines:

   ```yaml
   # Two runs kept out of the checks on each pull request, once a day and on request: the engine's
   # tests that fetch the pinned @modelcontextprotocol/server-filesystem from npm, which need the
   # network, and the conformance vectors on the differential build, which take far longer than the
   # checks. Each job runs on `ubuntu-latest`, a standard GitHub-hosted runner, which is free for a
   # public repository; neither uses a larger runner, paid service or secret.
   ```

5. **What stays.**
   - The runner and its output do not change. A failed case already ends the run with a `FAIL`
     line that gives the counts, then one line for each failing case with its promises and its
     error (`engine/crates/conformance/src/lib.rs` lines 339 to 362, `src/main.rs` lines 44 to
     47), and a failed differential check names the consumer. On success the run step's last line
     is `N cases passed`, or `P cases passed; M expected failures` while
     `conformance/expected-failures.yaml` (order 163) lists M cases (`main.rs` lines 34 to 42).
     That line is the evidence the review's order asks for. The output already says what failed
     and why ([ruling 52](../proposals/ruling-52/README.md), point 2), so nothing is added to it.
   - Only the vectors run on the differential build. The shell's tests under the feature, which
     order 070 also ran, and Clippy with the feature are not part of this order.
   - `checks.yml` does not change. The new job runs on no pull request, gates no merge and is not
     among the checks that the coordinator's D11
     ([decisions](../proposals/full-review/decisions.md)) asks the owner to require.
   - Nothing under `engine/`, `spec/` or the cases changes. No promise or vector changes, so
     neither CONTRIBUTING.md's rule 8 nor a declaration file applies.
6. **`conformance/README.md` says what the build is and where it runs.** In "Running", after the
   paragraph that ends "before any matching case runs." (line 127), and before the paragraph on
   the pieces tool, add this text, with its links as written (each relative to the README):

   ````text
   The `differential` feature adds to the core an independent computation of each consumer's tool
   list and of the notifications its changes send, which
   [order 070](../docs/work-orders/070-build-state-copies.md) kept from before it made publication
   lazy. Every transition is then checked against that computation, and a difference fails the case
   with an error that names the consumer. From `engine/`, the vectors run on that build with:

   ```sh
   CONDUIT_VECTOR_THREADS=4 cargo run --release -p conduit-conformance --features differential
   ```

   That run is far slower than the normal build's, so the checks on a pull request leave it out:
   [nightly.yml](../.github/workflows/nightly.yml) runs it once a day and on request.
   ````

   Keep lines to 100 characters or fewer. Nothing else in the file changes.
7. **The method as run says so.** A merge that changes how orders are checked updates that page
   in the same pull request (D13 of the same decisions). In
   `docs/implementation/METHOD_AS_RUN.md`, in the paragraph on CI (lines 280 to 291 on `main` at
   `2f33a53e`), the sentence "A nightly job runs the engine with npm." (lines 288 and 289) becomes
   this, with its link as written:

   ```text
   Once a day and on request, [nightly.yml](../../.github/workflows/nightly.yml) runs the engine's
   tests with npm and the vectors on the differential build.
   ```

   Rewrap only the lines that the new sentence moves, from its first line to the paragraph's end,
   at no more than 100 characters a line. The lines before it do not change. Orders 185, 187 and
   189 may change other sentences of the same paragraph (185 changes lines 281 to 283). Find the
   sentence by its words, not its line number, and change nothing else in the paragraph.

## Scope

Change only these three files:

- `.github/workflows/nightly.yml`: the header (decision 4) and the new job (decisions 2 and 3);
- `conformance/README.md`: decision 6's paragraph and command, in "Running";
- `docs/implementation/METHOD_AS_RUN.md`: decision 7's sentence and the rewrap after it.

`engine/crates/core/src/differential.rs` changes only for the planted fault below, and ends byte
for byte as on `HEAD`. Change no other file: not `checks.yml`, no `Cargo.toml` or `Cargo.lock`,
no engine source, no case and nothing under `spec/`. Orders 185, 187 and 189 may also change
`docs/implementation/METHOD_AS_RUN.md`. Whichever merges later is rebased by the admin, who keeps
both changes.

## Done when

From `engine/`, one command at a time:

```bash
cargo build --release -p conduit-conformance --features differential
CONDUIT_VECTOR_THREADS=4 target/release/conduit-conformance .. access- | tail -n 1
```

The second prints `18 cases passed`. Run the built runner directly, not through cargo, so that the
timing below measures the runner alone and no compiler warning enters its output.

Then show that the job's command runs the check, and that without the feature it does not. Make
the first statement of `check` in `crates/core/src/differential.rs` (line 134)
`return Err("planted differential fault".into());`. Expect the compiler to warn that the rest is
unreachable. This is the one change outside scope, and the last command below undoes it. Then,
one at a time:

```bash
cargo build --release -p conduit-conformance --features differential
CONDUIT_VECTOR_THREADS=4 target/release/conduit-conformance .. access- 2> target/planted.txt
head -n 1 target/planted.txt
grep -c 'planted differential fault' target/planted.txt
cargo build --release -p conduit-conformance
CONDUIT_VECTOR_THREADS=4 target/release/conduit-conformance .. access- | tail -n 1
git show HEAD:engine/crates/core/src/differential.rs > crates/core/src/differential.rs
```

The second exits 1, the third prints `FAIL 0 cases passed; 18 failed` and the fourth prints `18`.
The sixth prints `18 cases passed`: built without the feature, the runner never reaches the fault.
If any `access-` case passes with the fault in place, report which, and its steps.

Then run every case on the differential build, timed, from a fresh build. The run may take more
than an hour; run it in the foreground and let it finish:

```bash
cargo build --release -p conduit-conformance --features differential
CONDUIT_VECTOR_THREADS=4 /usr/bin/time -l target/release/conduit-conformance .. > target/differential-vectors.txt
tail -n 1 target/differential-vectors.txt
```

The last command prints `N cases passed`. N is the number this command prints from the repository
root (501 on `main` at `2f33a53e`):

```sh
/usr/bin/python3 -c 'import pathlib,yaml; print(sum(len(yaml.safe_load(p.read_text())["cases"]) for p in pathlib.Path("conformance").rglob("index.yaml")))'
```

While `conformance/expected-failures.yaml` lists M cases, it prints
`P cases passed; M expected failures` instead, with P + M = N.

Report the `real` seconds and the `maximum resident set size` that `time` prints, and apply
decision 3. `/usr/bin/time -l` is macOS's. On Linux use `/usr/bin/time -v`, whose
`Maximum resident set size` is in kilobytes, so decision 3's limit becomes 12582912. If any case
fails, report the `FAIL` line and each failing case's line, not the whole output. Change nothing
for it, and finish the rest of the order: the admin logs it as a finding of the first build, and
the manual run below waits for its fix.

From the repository root, run each:

```sh
/usr/bin/python3 -c 'import sys,yaml; j=yaml.safe_load(sys.stdin)["jobs"]["differential"]; r=[s for s in j["steps"] if "conduit-conformance" in s.get("run","")]; print(j["runs-on"], j["timeout-minutes"], j["defaults"]["run"]["working-directory"], [(s["run"], s.get("env")) for s in r])' < .github/workflows/nightly.yml
/usr/bin/python3 -c 'import subprocess,yaml; a=yaml.safe_load(open(".github/workflows/nightly.yml")); b=yaml.safe_load(subprocess.check_output(["git","show","HEAD:.github/workflows/nightly.yml"])); print(sorted(a["jobs"]), a["jobs"]["npm"]==b["jobs"]["npm"], all(a[k]==b[k] for k in b if k!="jobs"))'
grep -c 'secrets\.' .github/workflows/nightly.yml
grep -c -F 'CONDUIT_VECTOR_THREADS=4 cargo run --release -p conduit-conformance --features differential' conformance/README.md
tr -s ' \n' '  ' < conformance/README.md | grep -c -F 'Every transition is then checked against that computation, and a difference fails the case with an error that names the consumer'
tr -s ' \n' '  ' < docs/implementation/METHOD_AS_RUN.md | grep -c -F "tests with npm and the vectors on the differential build. 068 built CI"
tr -s ' \n' '  ' < docs/implementation/METHOD_AS_RUN.md | grep -c -F 'A nightly job runs the engine with npm'
python3 tools/check_docs.py
python3 tools/set_names.py --check
python3 tools/check_private_paths.py
git diff --check
git status --short
```

The first prints exactly this line:

```text
ubuntu-latest 300 engine [('cargo build --release -p conduit-conformance --features differential', None), ('cargo run --release -p conduit-conformance --features differential', {'CONDUIT_VECTOR_THREADS': '4'})]
```

The second prints `['differential', 'npm'] True True`: the `npm` job and everything outside the
jobs are as on `HEAD`. PyYAML reads the key `on` as `True`, and the check allows for that. The
third prints `0`, the next three print `1` each, and the seventh prints `0`. The third and seventh
exit 1, as `grep -c` does when it prints 0, and that is expected. If decision 3 chose two threads,
the first check shows `'2'` in place of `'4'`, and the fourth check's pattern reads
`CONDUIT_VECTOR_THREADS=2`. `check_docs.py`, `set_names.py --check` and
`check_private_paths.py` end with no error, `git diff --check` prints nothing, and
`git status --short` lists only the three files in scope. Then show that the first check fails
with the change removed by running it on `main`'s file:

```sh
git show HEAD:.github/workflows/nightly.yml | /usr/bin/python3 -c 'import sys,yaml; j=yaml.safe_load(sys.stdin)["jobs"]["differential"]; r=[s for s in j["steps"] if "conduit-conformance" in s.get("run","")]; print(j["runs-on"], j["timeout-minutes"], j["defaults"]["run"]["working-directory"], [(s["run"], s.get("env")) for s in r])'
```

It must exit 1 with `KeyError: 'differential'` on its last line. Do not stage anything. In your
report, mark each choice you made yourself as your own. Give the two `access-` counts, the
`FAIL` line, the full run's last line, its `real` seconds and peak memory, the thread count you
kept, and the last line of every check.

The admin then pushes `wo/188-differential-vectors-nightly`, opens its pull request and starts the
branch's workflow by hand with `gh workflow run nightly.yml --ref wo/188-differential-vectors-nightly`.
The workflow's concurrency group makes the run wait while a scheduled run is in progress. This
manual run is the review's done-when for its order 31. The `differential vectors` job passes, and
its run step's log ends with `N cases passed` (or `P cases passed; M expected failures`), with N,
or P + M, as above: 500 or more. If the job ends for memory (exit 137, or a runner lost) or at its
300 minutes, the order is not done. The admin then applies decision 3's two-thread form in the same
places, says so in the Record and runs the job again. If it ends the same way at two threads, the
admin stops and reports it. The Record cites the passing run by its address and gives the job's
duration, that last line and the `npm` job's result beside it (D9;
[FR-533](../proposals/full-review/findings/G6-outside-the-tree.md#fr-533)). On the pull request
the admin also shows the `docs` and `rust` checks passing. A fresh Codex verifier and a fresh
read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).

## Record

Run on 2026-11-15 on the Mac from `main` `0ed26306`: the worker (6,666 s) and a fresh Codex
verifier (594 s), which found no must or should in one round but could not run the hosted
workflow. `nightly.yml` gains its second job, `differential vectors`, after `npm`, word for word as
decision 2 gives it: on `ubuntu-latest`, with at most 300 minutes, restoring without saving the
cache that `checks.yml` saves for `engine/`, then a build step and the run step at
`CONDUIT_VECTOR_THREADS=4`. Its header now says what both jobs are. The schedule, the manual
trigger, `permissions`, `concurrency`, `env` and the `npm` job are unchanged, and neither job reads
a secret. `conformance/README.md` says, in "Running", what the `differential` feature checks and
gives its command, and the paragraph on CI in `METHOD_AS_RUN.md` says that `nightly.yml` runs the
vectors on the differential build once a day and on request. Nothing under `engine/`, `spec/` or
the cases changed. On the Mac both `access-` runs printed `18 cases passed`; with the planted fault
the differential build printed `FAIL 0 cases passed; 18 failed` and 18 planted-fault errors, and
`differential.rs` was restored byte for byte. In the full timed run, `/usr/bin/time -l` failed in
the sandbox (`time: sysctl kern.clockrate: Operation not permitted`) after the runner printed
`501 cases passed` (real 3062.73 s), and printed no peak. The peak, 5,336,973,312 bytes (4.97 GiB,
under decision 3's 12 GiB), was measured on a second run through `getrusage(RUSAGE_CHILDREN)`
(3011.01 s), which also printed `501 cases passed`; that second run was the worker's own choice.
Four threads were kept.

A fresh read-only Opus reader then verified the order (ruling 79, point 2). It ran
`check_docs.py`, `check_private_paths.py` and `set_names.py --check`, each ending "Errors: 0", and
`git diff --check`, which was clean, and parsed the workflow with PyYAML 6.0.1 under
`/usr/bin/python3`. Every Done-when check gave the order's value. Since the checkpoint commit
exists, it compared the second against `origin/main` rather than `HEAD`, and on `main`'s file the
first stopped at `KeyError: 'differential'`; the README and `METHOD_AS_RUN.md` greps each gave the
opposite count on `main`, so each can fail. It ran the run step's command on the `access-` cases,
which printed `18 cases passed`, and read the workflow's run against the worker's: the same binary,
features, thread count and cases, since with no argument the runner takes the repository it was
built in (`main.rs` lines 3 to 6), and its last line is the count (lines 34 to 42). It did not
repeat the full run or the planted fault. It found no fault in the change. Its must is the order's
own last step, the manual run, left to the admin. Its should, that this Record give the timed run
as it went rather than say that `time` printed the peak, is applied above.

This fix reran `check_docs.py`, `check_private_paths.py` and `set_names.py --check`, each ending
"Errors: 0", `git diff --check`, the YAML parse, every Done-when check from the root (the second
against `origin/main`) and the case count, 501 with no expected failure. From `engine/` it reran
the order's `access-` runs and the planted fault: the differential build printed `18 cases passed`
(186 s); with the fault, `FAIL 0 cases passed; 18 failed` and 18 planted-fault lines; the normal
build, with the fault still in place, `18 cases passed` (8 s). `differential.rs` was then restored
byte for byte as on `main`. It did not repeat the full run.

The order is done only once the admin has pushed the branch, opened its pull request and run
`gh workflow run nightly.yml --ref wo/188-differential-vectors-nightly`, and the
`differential vectors` job has passed with its run step's log ending `501 cases passed`. This
Record then gains the run's address, the job's duration, that last line and the `npm` job's result
beside it (D9; [FR-533](../proposals/full-review/findings/G6-outside-the-tree.md#fr-533)), and the
pull request shows the `docs` and `rust` checks passing. If the job exits 137, loses its runner or
reaches its 300 minutes, the admin applies decision 3's two-thread form, says so here and runs it
again.

The admin pushed the branch and ran `gh workflow run nightly.yml --ref
wo/188-differential-vectors-nightly` on 2026-11-16: [run
43821570026](https://github.com/shrav-k/conduit/actions/runs/43821570026), on the branch at
`4492dd35`. `differential vectors` passed in 103 min 16 s; its run step's log ends `501 cases
passed`. `engine with npm` passed beside it in 15 min 8 s. Under
[ruling 80](../proposals/ruling-80/README.md), point 10, the merge waited on the coordinator's local
gate, not on the pull request's CI checks, which run but are not waited on.
