# Work order 189: changed package tests run five more times on Linux

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
Its only cargo commands are the `cargo locate-project` and `cargo metadata --no-deps` that the
workflow's own steps run, which build nothing and fetch nothing; run them one at a time, as those
steps do. Run no `cargo build` and no `cargo test`: decision 2's loop is checked with a stand-in
for cargo. Never print a whole large file or log. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

The full review of 2026-11-14 (order 44 of [its orders](../proposals/full-review/orders.md))
found that a test that fails at times on CI's Linux runner passes once for the worker and on the
Mac, and so reaches `main`
([FR-493](../proposals/full-review/findings/G4-who-verifies.md#fr-493), gate 7, as its verifier
corrected it;
[FR-271](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-271), point 4). The job
`rust-dir` in `.github/workflows/checks.yml` runs each crate's `cargo test --release` once (lines
238 to 240), so a test that fails in some runs passes CI in most.

That is what happened. Since 2026-11-12 the package jobs failed 17 times in CI on tests that
passed on the Mac (the message of `255b3206`, pull request 266). Pull request 266 fixed three
tests, and `8873bedf` (pull request 268) fixed two more places where the first of those faults
could still strike. Every fault was in the tests, and none in the providers:

- `provider-ssh-runtime`: a flock belongs to the open file description, which a child forked by
  another test thread shares until its exec. So a lock that a test had just released could stay
  held for an instant (both pull requests).
- `runtime-provider-kit`: with one tracing dispatcher registered, a new call site takes its
  interest from the thread that reaches it first. So the secrets test's capture could be empty.
- `provider-local-process`: the backend proves a run's group gone only once its last member is
  reaped, an instant after the processes die.

This order adds the check that gate 7 asks for. On a pull request that changes a package crate,
directly or through a path dependency, that crate's tests run five more times on Linux, and a
failed run fails the `rust` gate. On any other pull request, and on a push to `main`, the repeats
are skipped. The review's row sets five runs. FR-271 had said twenty, and its verifier answered
that repeats on an idle runner will not reproduce a race that needs load. Five runs catch a test
that fails often, such as one run in three, but a rarer fault can still pass them. The
coordinator's D12 ([decisions](../proposals/full-review/decisions.md)) says a lock handoff is
tested deterministically, "never repetition alone". This job is a safety net beside such tests,
not a substitute for them. Once the owner makes `rust` a required check (D11), the repeats also
gate the merge.

## The decisions it applies

1. **`changed packages` lists the package crates that a pull request changes.** Add a job
   `changed-packages`, named `changed packages`. Put it after `engine-doc` and before the comment
   above `rust` (line 383). It has `needs: rust-dirs` and nothing else, runs on `ubuntu-latest`
   with `timeout-minutes: 5`, and has no `if`. It runs on every event, so that the gate can always
   read its result. Its checkout sets `fetch-depth: 2`, so that `HEAD^1`, the base of a pull
   request's merge commit, is present, and `persist-credentials: false`.

   After its checkout it has one step, `id: select`, with `shell: bash`. GitHub then runs it as
   `bash --noprofile --norc -eo pipefail`, which is how the checks below run it. Without that
   line it would run as `bash -e`, without pipefail. The step gets four values through `env`,
   never written into its script: `EVENT_NAME` (`github.event_name`), `RUST_DIRS` (the `matrix`
   output of `rust-dirs`), `DIFF_BASE: HEAD^1` and `DIFF_HEAD: HEAD`. It sets two outputs, and
   the job exports them as `dirs` and `any`:
   - When `EVENT_NAME` is not `pull_request`, it prints
     `Not a pull request (<event>): the repeated runs are skipped.` and sets `dirs=[]` and
     `any=false`.
   - The candidates are the entries of `RUST_DIRS` whose `dir` begins `packages/` and whose
     `build` is true. Today these are the 12 crates under `packages/`. `rust-dirs` finds the
     crates (lines 158 to 194), and this step does not search for them again.
   - The changed paths are those that `git diff --no-renames --name-only "$DIFF_BASE" "$DIFF_HEAD"`
     prints. With `--no-renames`, a moved file counts at both its old and its new place.
   - A candidate is listed when a changed path lies under its directory, or under the directory
     of a path dependency that it names. Its path dependencies are those of every kind (normal,
     dev and build) for which
     `cargo metadata --no-deps --format-version 1 --manifest-path <dir>/Cargo.toml` gives a
     `path`. Run that command for every candidate, from the repository root, as `rust-dirs` runs
     `cargo locate-project`. Cargo gives each `path` as an absolute path. Strip the repository
     root (`$PWD/`) from it. If a path does not begin with that root, fail the step with a line
     that names the path. A path dependency inside the crate's own directory adds nothing. Today
     there are two: `packages/runtime-provider-kit`, for `provider-local-process` and
     `provider-ssh-runtime`, and `experiments/013-s2-shared-login/linear-stub`, a dev-dependency
     of `scenario-s2-linear` and `scenario-s2-linear-adapter`. Only direct path dependencies
     count, and neither of today's two has one of its own. Compare paths as plain prefixes
     ending in `/`, never as patterns.
   - For each listed crate, in byte order, it prints `<dir>: N changed file(s) in its directory`
     when N is at least 1. Then, for each such dependency, it prints
     `<dir>: its path dependency <path> changed`, with the path from the repository root. After
     all listed crates it prints `Repeating the tests of N package crate(s) five times each.` It
     sets `dirs` to the listed directories as a compact JSON array in byte order, each once, and
     sets `any=true`. When no crate is listed, it prints
     `No package crate changed, nor a path dependency of one: the repeated runs are skipped.` and
     sets `dirs=[]` and `any=false`.
   - The step fails, and sets neither output, when any command whose output it uses fails:
     `git diff` on a base that is not there, `cargo metadata` on a manifest that cannot be read,
     and `jq`. A failure never counts as "no change". Bash's `-e` does not see a command that
     runs in a `for` loop's word list. So take each such output by assignment (`x=$(...)`), then
     loop over the variable.
   - A change outside every package and path dependency lists nothing. This holds even under
     `spec/`, which the packages' schema-drift tests read: those tests use no lock, timer or
     process, and `rust-dir` still runs them once.
   - Use nothing that the Mac's `/bin/bash` 3.2 lacks, such as `mapfile` or associative arrays,
     so that the checks below can run the step there. `jq` is on the runner and on the Mac.

2. **`package tests repeated` runs each listed crate's tests five more times.** Add a job
   `package-repeats`, named `package tests repeated (${{ matrix.dir }})`. It has
   `needs: changed-packages` and nothing else, and runs only
   `if: ${{ needs.changed-packages.outputs.any == 'true' }}`. GitHub Actions rejects a matrix
   with no entries, so when no crate is listed the job is skipped rather than run empty. It never
   needs `rust-dir`. [Order 152](152-ci-engine-shards-stand-alone.md) stopped one package's
   failure from skipping the engine's shards; in the same way, one crate's failed single run must
   not skip another crate's repeats. The job has `runs-on: ubuntu-latest`,
   `timeout-minutes: 60`, `fail-fast: false` and
   `matrix: dir: ${{ fromJSON(needs.changed-packages.outputs.dirs) }}`. It sets
   `working-directory: ${{ matrix.dir }}` as `rust-dir` does (lines 207 to 209). Each package's
   whole job took under two minutes in CI's run for `2c0dc6bd` on `main`, so 60 minutes leaves
   wide room. Its steps:
   - checkout with `persist-credentials: false`;
   - `rust-dir`'s toolchain step (lines 214 to 219), without `rustup component add rustfmt
     clippy`: formatting and clippy stay `rust-dir`'s;
   - `actions/cache/restore@v6` with `rust-dir`'s paths, `key` and `restore-keys` (lines 220 to
     230), and no save step, so that the cache stays the one `rust-dir` writes;
   - one step, `id: repeat`, named `cargo test --release, five more times`, with `shell: bash`
     (which adds `-o pipefail`). It gets two values through `env`: `CRATE_DIR: ${{ matrix.dir }}`
     and `CARGO_TERM_COLOR: never`. The workflow's `env` sets colour to `always` (line 21), and
     cargo's own `error:` lines would then carry colour codes that a plain match misses.

   The step runs `cargo test --release`, the command on line 240, exactly five times in the
   crate's directory, with no other flag. It runs all five whatever any one of them gives. It
   never retries: a failed run is counted and never run again
   ([068](068-test-gate-and-ci.md), decision 1: "no retries anywhere"). For run K:
   - the run's output goes to the log, between `::group::run K of 5: cargo test --release` and
     `::endgroup::`, and to a file under `$RUNNER_TEMP`;
   - then one line follows: `run K of 5: passed`, or `run K of 5: failed (exit S)`, where S is
     cargo's own exit status, not `tee`'s;
   - after a failed run's line, the step prints each line of that run's output that names a
     failed test (`test <name> ... FAILED`) or begins `error: `. Cargo's line that names the test
     target to rerun begins that way, and so does its line for a crate that did not compile. If
     no line matches, it prints `run K of 5: no failed test was named; see its output above`.

   When every run passed, the last line is `<dir>: all 5 repeated runs passed`, and the step exits
   0. When F runs failed, the last line is
   `::error title=repeated package tests::<dir>: F of 5 repeated runs failed (run numbers: a, b)`,
   with the failed runs' numbers in order, and the step exits 1. GitHub shows that line as an
   annotation on the pull request ([ruling 52](../proposals/ruling-52/README.md): a result as
   explicit as it can be).

   The loop uses `cargo test`, not nextest's `--stress-count`, which FR-271's verifier proposed.
   Two of the three faults that `255b3206` fixed, and the two places that `8873bedf` fixed,
   needed two test threads in one process: a lock held by a child that another test's thread
   forked, and a call site's interest taken from whichever thread reached it first. nextest runs
   each test in a process of its own, where neither fault can happen. Running the command that
   `rust-dir` runs repeats the conditions in which these tests failed. The comment above the two
   new jobs says so, and cites FR-493, gate 7.

3. **The `rust` gate takes in both jobs.** Add `changed-packages` and `package-repeats` to
   `rust`'s `needs` (line 387). Its step gets three more values through `env`:
   `CHANGED: ${{ needs.changed-packages.result }}`,
   `ANY_CHANGED: ${{ needs.changed-packages.outputs.any }}` and
   `REPEATS: ${{ needs.package-repeats.result }}`. The gate passes only when all of these hold:
   - everything it requires today passes;
   - `CHANGED` is `success`;
   - either `ANY_CHANGED` is `true` and `REPEATS` is `success`, or `ANY_CHANGED` is `false` and
     `REPEATS` is `skipped`.

   Any other combination fails the gate. Among them are a repeat that failed or was cancelled, a
   repeat skipped although a crate was listed, and `changed packages` failed or skipped. The
   `echo` line (line 400) gains this ending:
   `; changed packages: $CHANGED (any: $ANY_CHANGED); repeated package tests: $REPEATS`.
   The comment above the job (lines 383 and 384) says that the repeats may be skipped only when
   no crate was listed. No other job's `needs` changes: `secrets`, `docs`, `rust-dirs`,
   `rust-dir`, `engine-build`, `engine-tests` and `engine-doc` stay as they are.

4. **Linux only, and apart from orders 187 and 188.** The repeats run on `ubuntu-latest` alone,
   where the faults showed. Order 187, written beside this one, adds a macOS leg to the same
   workflow in jobs of its own, and changes the same `rust` job and the same lines of the method
   as run. Order 188 (the differential vectors, nightly) may change the nightly sentence in those
   lines. Keep them apart: add no macOS runner here, and change none of 187's jobs. If 187 has
   merged when you start, keep its jobs and its entries in the gate as they are. Add this order's
   two jobs and conditions beside them, and say so in your report. In the gate's checks below,
   also set to `success` each further result that the gate then reads.

5. **The method as run says what the jobs do.** Under D13, a merge that changes how orders are
   checked updates [the method as run](../implementation/METHOD_AS_RUN.md) in the same pull
   request. Make two changes in its account of CI, at lines 280 to 291, the end of the paragraph
   that begins at line 276:
   - After "excluding the macOS-only probe.", insert: "Since 189, on a pull request, `changed
     packages` lists each package crate whose directory, or the directory of one of its path
     dependencies, the pull request changes, and `package tests repeated (directory)` runs that
     crate's release tests five more times on Linux, all five even after a failure; when it lists
     none, and on a push, the repeats are skipped."
   - Replace "Those five Rust job groups feed the `rust` gate: failed, skipped or cancelled jobs
     fail it." with "These Rust jobs feed the `rust` gate: a failed, skipped or cancelled job
     fails it, except that the repeats may be skipped when `changed packages` lists no crate."

   The new text gives no count of jobs, so order 187 need not change it again. If 187 or 188 has
   merged and changed these lines, keep its words. Make the two changes beside them, using the
   same words for the exception if the gate's sentence now reads differently, and say so in your
   report. Rewrap only lines 280 to 291, to at most 100 characters a line. Nothing else in the
   file changes.

## Scope

Change only these two files:

- `.github/workflows/checks.yml`: the two new jobs, and the `rust` job's `needs`, step `env`,
  `echo` line, test and comment;
- `docs/implementation/METHOD_AS_RUN.md`: the lines of decision 5.

Change no test, no package, no other job and no other document. This order changes no promise,
case or fixture, so it writes no declaration, and CONTRIBUTING.md's rule 8 asks nothing of it.

## Done when

Run every command from the repository root, one at a time. Git ignores `target/`.

Read the scripts back out of the workflow:

```sh
mkdir -p target/o189/fakebin target/o189/tmp
/usr/bin/python3 - .github/workflows/checks.yml target/o189 <<'PY'
import sys, yaml
jobs = yaml.safe_load(open(sys.argv[1]))["jobs"]
def run(job, step):
    return next(s["run"] for s in jobs[job]["steps"] if s.get("id") == step)
scripts = {"find": run("rust-dirs", "find"), "select": run("changed-packages", "select"),
           "repeat": run("package-repeats", "repeat"), "gate": jobs["rust"]["steps"][-1]["run"]}
for name, text in scripts.items():
    open(f"{sys.argv[2]}/{name}.sh", "w").write(text)
print("written:", " ".join(sorted(scripts)))
PY
```

It prints `written: find gate repeat select`.

The jobs' shape:

```sh
cat > target/o189/shape.py <<'PY'
import sys, yaml
jobs = yaml.safe_load(open(sys.argv[1]))["jobs"]
repeats = jobs["package-repeats"]
def needs(name):
    n = jobs[name].get("needs", [])
    return sorted([n] if isinstance(n, str) else n)
def step(job, sid):
    return next(s for s in jobs[job]["steps"] if s.get("id") == sid)
print("needs:", needs("changed-packages"), needs("package-repeats"), needs("rust-dir"), needs("engine-tests"), needs("engine-doc"))
print("if:", repeats["if"])
print("matrix:", repeats["strategy"]["matrix"]["dir"], repeats["strategy"]["fail-fast"], repeats["runs-on"])
print("actions:", sorted({s["uses"].split("@")[0] for s in repeats["steps"] if "uses" in s}))
print("select:", jobs["changed-packages"]["steps"][0]["with"]["fetch-depth"], step("changed-packages", "select").get("shell"))
print("repeat:", step("package-repeats", "repeat").get("shell"), step("package-repeats", "repeat")["env"]["CARGO_TERM_COLOR"])
print("rust also needs:", sorted(set(needs("rust")) - {"rust-dirs", "rust-dir", "engine-build", "engine-tests", "engine-doc"}))
print("needing the new jobs:", sorted(n for n in jobs if n != "rust" and {"changed-packages", "package-repeats"} & set(needs(n))))
PY
/usr/bin/python3 target/o189/shape.py .github/workflows/checks.yml; echo "exit $?"
```

It prints these lines and `exit 0`. If order 187 has merged, `rust also needs` also lists 187's
jobs.

```text
needs: ['rust-dirs'] ['changed-packages'] ['rust-dirs'] ['engine-build'] ['engine-build']
if: ${{ needs.changed-packages.outputs.any == 'true' }}
matrix: ${{ fromJSON(needs.changed-packages.outputs.dirs) }} False ubuntu-latest
actions: ['actions/cache/restore', 'actions/checkout']
select: 2 bash
repeat: bash never
rust also needs: ['changed-packages', 'package-repeats']
needing the new jobs: ['package-repeats']
```

Decision 1, on five merged commits: `8873bedf` (pull request 268), `255b3206` (pull request
266), `2689cbc5` (order 141, the kit alone), `9f55b1e6` (order 135, the Linear stub alone) and
`2f33a53e` (order 181, no package):

```sh
: > target/o189/dirs.out
GITHUB_OUTPUT=target/o189/dirs.out /bin/bash --noprofile --norc -eo pipefail target/o189/find.sh > /dev/null; echo "exit $?"
sed -n 's/^matrix=//p' target/o189/dirs.out > target/o189/matrix.json
for c in 8873bedf 255b3206 2689cbc5 9f55b1e6 2f33a53e; do : > target/o189/$c.out; EVENT_NAME=pull_request DIFF_BASE="$c^" DIFF_HEAD=$c RUST_DIRS="$(cat target/o189/matrix.json)" GITHUB_OUTPUT=target/o189/$c.out /bin/bash --noprofile --norc -eo pipefail target/o189/select.sh > target/o189/$c.txt; echo "$c exit $?"; done
for c in 8873bedf 255b3206 2689cbc5 9f55b1e6 2f33a53e; do echo "== $c"; cat target/o189/$c.txt; sort target/o189/$c.out; done
```

Each command exits 0, and the last prints exactly the lines below. These outputs were reached
from `main` at `2f33a53e`. If a package has been added or has gained a path dependency since,
say so and give the lines you got.

```text
== 8873bedf
packages/provider-ssh-runtime: 1 changed file(s) in its directory
Repeating the tests of 1 package crate(s) five times each.
any=true
dirs=["packages/provider-ssh-runtime"]
== 255b3206
packages/provider-local-process: 1 changed file(s) in its directory
packages/provider-local-process: its path dependency packages/runtime-provider-kit changed
packages/provider-ssh-runtime: 3 changed file(s) in its directory
packages/provider-ssh-runtime: its path dependency packages/runtime-provider-kit changed
packages/runtime-provider-kit: 1 changed file(s) in its directory
Repeating the tests of 3 package crate(s) five times each.
any=true
dirs=["packages/provider-local-process","packages/provider-ssh-runtime","packages/runtime-provider-kit"]
== 2689cbc5
packages/provider-local-process: its path dependency packages/runtime-provider-kit changed
packages/provider-ssh-runtime: its path dependency packages/runtime-provider-kit changed
packages/runtime-provider-kit: 4 changed file(s) in its directory
Repeating the tests of 3 package crate(s) five times each.
any=true
dirs=["packages/provider-local-process","packages/provider-ssh-runtime","packages/runtime-provider-kit"]
== 9f55b1e6
packages/scenario-s2-linear: its path dependency experiments/013-s2-shared-login/linear-stub changed
packages/scenario-s2-linear-adapter: its path dependency experiments/013-s2-shared-login/linear-stub changed
Repeating the tests of 2 package crate(s) five times each.
any=true
dirs=["packages/scenario-s2-linear","packages/scenario-s2-linear-adapter"]
== 2f33a53e
No package crate changed, nor a path dependency of one: the repeated runs are skipped.
any=false
dirs=[]
```

A push, a base that is not there, and a crate whose manifest is not there:

```sh
: > target/o189/push.out; EVENT_NAME=push DIFF_BASE="8873bedf^" DIFF_HEAD=8873bedf RUST_DIRS="$(cat target/o189/matrix.json)" GITHUB_OUTPUT=target/o189/push.out /bin/bash --noprofile --norc -eo pipefail target/o189/select.sh; echo "exit $?"; sort target/o189/push.out
: > target/o189/bad.out; EVENT_NAME=pull_request DIFF_BASE=no-such-commit DIFF_HEAD=8873bedf RUST_DIRS="$(cat target/o189/matrix.json)" GITHUB_OUTPUT=target/o189/bad.out /bin/bash --noprofile --norc -eo pipefail target/o189/select.sh > /dev/null 2>&1; echo "exit $?"; grep -c '^any=' target/o189/bad.out
: > target/o189/nometa.out; EVENT_NAME=pull_request DIFF_BASE="2f33a53e^" DIFF_HEAD=2f33a53e RUST_DIRS='[{"dir":"packages/no-such-crate","build":true,"lint":true}]' GITHUB_OUTPUT=target/o189/nometa.out /bin/bash --noprofile --norc -eo pipefail target/o189/select.sh > /dev/null 2>&1; echo "exit $?"; grep -c '^any=' target/o189/nometa.out
```

The first prints `Not a pull request (push): the repeated runs are skipped.`, `exit 0`,
`any=false` and `dirs=[]`. The second and the third each print an exit status other than 0, then
`0`. In the third, `cargo metadata` fails on the missing manifest, and that failure must fail the
step: it must not count as a crate with no path dependency.

Decision 2, with a stand-in for cargo that records its arguments and fails the runs it is told
to:

```sh
cat > target/o189/fakebin/cargo <<'SH'
#!/bin/sh
# A stand-in for cargo: records its arguments and fails the runs that O189_NAMED and O189_QUIET list.
echo "$*" >> "$O189_CALLS"
count=$(( $(cat "$O189_COUNT") + 1 ))
echo "$count" > "$O189_COUNT"
case " $O189_NAMED " in *" $count "*)
  echo "test held_lock_is_released ... FAILED"
  echo 'error: test failed, to rerun pass `--test configuration`'
  exit 101 ;;
esac
case " $O189_QUIET " in *" $count "*) exit 101 ;; esac
echo "test result: ok. 1 passed; 0 failed"
SH
chmod +x target/o189/fakebin/cargo
cat > target/o189/run-repeat.sh <<'SH'
# Runs decision 2's step in packages/provider-file with the stand-in for cargo.
# Arguments: the runs that fail naming a test, the runs that fail naming none, the output file.
root=$(pwd)
echo 0 > "$root/target/o189/count"
: > "$root/target/o189/calls"
cd packages/provider-file
O189_NAMED="$1" O189_QUIET="$2" O189_COUNT="$root/target/o189/count" O189_CALLS="$root/target/o189/calls" \
  PATH="$root/target/o189/fakebin:$PATH" CRATE_DIR=packages/provider-file RUNNER_TEMP="$root/target/o189/tmp" \
  /bin/bash --noprofile --norc -eo pipefail "$root/target/o189/repeat.sh" > "$root/target/o189/$3"
SH
/bin/sh target/o189/run-repeat.sh "2 4" "" named.txt; echo "exit $?"; sort target/o189/calls | uniq -c
grep -c '^run [135] of 5: passed$' target/o189/named.txt
grep -c '^run [24] of 5: failed (exit 101)$' target/o189/named.txt
grep -A2 '^run [24] of 5: failed (exit 101)$' target/o189/named.txt
tail -n 1 target/o189/named.txt
/bin/sh target/o189/run-repeat.sh "" "3" quiet.txt; echo "exit $?"; sort target/o189/calls | uniq -c
grep -A1 '^run 3 of 5: failed (exit 101)$' target/o189/quiet.txt
tail -n 1 target/o189/quiet.txt
/bin/sh target/o189/run-repeat.sh "" "" none.txt; echo "exit $?"; sort target/o189/calls | uniq -c
tail -n 1 target/o189/none.txt
```

Each `run-repeat.sh` call is followed by its exit and `5 test --release`, so cargo ran five
times, with those arguments only, even after failures. The first call gives `exit 1`, then `3`
and `2`. Its `grep -A2` prints exactly these lines:

```text
run 2 of 5: failed (exit 101)
test held_lock_is_released ... FAILED
error: test failed, to rerun pass `--test configuration`
--
run 4 of 5: failed (exit 101)
test held_lock_is_released ... FAILED
error: test failed, to rerun pass `--test configuration`
```

Its last line is
`::error title=repeated package tests::packages/provider-file: 2 of 5 repeated runs failed (run numbers: 2, 4)`.
The second gives `exit 1`, then `run 3 of 5: failed (exit 101)` with
`run 3 of 5: no failed test was named; see its output above` on the next line, then
`::error title=repeated package tests::packages/provider-file: 1 of 5 repeated runs failed (run numbers: 3)`.
The third gives `exit 0` and `packages/provider-file: all 5 repeated runs passed`.

Decision 3, the gate over the results it can be given:

```sh
while read -r changed any repeats expected; do [ "$any" = - ] && any=; DIRS=success CHECKS=success BUILD=success SHARDS=success DOCTESTS=success CHANGED=$changed ANY_CHANGED=$any REPEATS=$repeats /bin/bash -e target/o189/gate.sh > /dev/null; echo "$changed $any $repeats: exit $? (expected $expected)"; done <<'EOF'
success true success 0
success false skipped 0
success true failure 1
success true cancelled 1
success true skipped 1
success false success 1
failure - skipped 1
skipped - skipped 1
EOF
```

Each line's exit equals its expected value.

Then show that the change is needed. Take `main`'s workflow from `2f33a53e`, not `HEAD`: in a
second round, `HEAD` is the runner's checkpoint of your first.

```sh
git show 2f33a53e:.github/workflows/checks.yml > target/o189/checks-main.yml
/usr/bin/python3 -c 'import sys, yaml; print(yaml.safe_load(open(sys.argv[1]))["jobs"]["rust"]["steps"][-1]["run"])' target/o189/checks-main.yml > target/o189/gate-main.sh
DIRS=success CHECKS=success BUILD=success SHARDS=success DOCTESTS=success CHANGED=success ANY_CHANGED=true REPEATS=failure /bin/bash -e target/o189/gate-main.sh > /dev/null; echo "exit $?"
/usr/bin/python3 target/o189/shape.py target/o189/checks-main.yml > target/o189/shape-main.txt 2>&1; echo "exit $?"; tail -n 1 target/o189/shape-main.txt
```

The first prints `exit 0`: `main`'s gate passes a failed repeat, which decision 3's gate fails.
The second prints `exit 1` and `KeyError: 'package-repeats'`.

Then the documents:

```sh
python3 tools/check_docs.py                              # Errors: 0
python3 tools/check_private_paths.py                     # Errors: 0
git diff --check
tr -s ' \n' '  ' < docs/implementation/METHOD_AS_RUN.md | grep -c 'Those five Rust job groups'   # prints 0
tr -s ' \n' '  ' < docs/implementation/METHOD_AS_RUN.md | grep -c -F 'except that the repeats may be skipped when `changed packages` lists no crate'   # prints 1
tr -s ' \n' '  ' < docs/implementation/METHOD_AS_RUN.md | grep -c -F 'runs that crate'"'"'s release tests five more times on Linux'   # prints 1
git status --short
```

`git status --short` lists only the two files in scope. Do not stage anything. In your report,
mark each choice you made yourself as your own, and give the last line of every check and each
line this section asks you to compare.

Then the admin shows the jobs on GitHub. On this order's pull request:

- `changed packages` passes and prints
  `No package crate changed, nor a path dependency of one: the repeated runs are skipped.`;
- `package tests repeated` is skipped;
- `rust` passes.

The admin then opens a draft pull request against `main`, from a scratch branch on this order's
branch, that adds one comment line to `packages/runtime-provider-kit/tests/runtime.rs`. On it:

- `changed packages` lists the same three crates as for `2689cbc5` above, with
  `packages/runtime-provider-kit: 1 changed file(s) in its directory`;
- three `package tests repeated` legs each end `all 5 repeated runs passed`;
- `rust` passes.

The admin closes that draft unmerged, deletes its branch, and gives each leg's minutes in the
Record. A fresh read-only Opus reader then reads the diff before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).
