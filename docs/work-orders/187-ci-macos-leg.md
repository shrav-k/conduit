# Work order 187: a macOS leg in CI

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
because some packages' tests bind loopback ports; use it for nothing else, and reach no other
machine. Run one cargo command at a time, and never print a whole large file or log. End every
process that a test of yours starts. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 30 of [its orders](../proposals/full-review/orders.md))
found that CI checks nothing on macOS
([FR-271](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-271), point 3;
[FR-493](../proposals/full-review/findings/G4-who-verifies.md#fr-493), gate 2). Every job in
[checks.yml](../../.github/workflows/checks.yml) runs on `ubuntu-latest`, as its lines 1 to 3
say. For an order built on Windows, the Mac run is made by hand: the changed crate's tests, fmt
and clippy pass on the Mac before merge
([the method as run](../implementation/METHOD_AS_RUN.md#5-on-the-windows-computer), line 219).
That run caught faults that a clean verifier had passed: orders 131 and 132 did not compile on the
Mac, and 140 failed 20 of its 33 tests there. FR-271's verifier added the engine's loader, core and
conformance suites, since nine engine or specification orders ran at least partly on Windows and
were run on the Mac by hand. FR-493's verifier corrected what the leg buys: the manual run caught
all three before merge, so the leg stops no fault that reached `main`. It turns a step the admin
takes by hand into one a machine takes, so that no reader is asked to find what CI can.

That verifier also asked that the leg say what it rests on. Ruling 64, point 3, allows CI only on
GitHub's standard runners, which are free for a public repository, and nothing that costs credits
([IN_FORCE.md, topic 14](../../IN_FORCE.md#14-publication-and-organization)). GitHub's standard
macOS runners cost a public repository nothing, and `gh repo view --json visibility` gave
`PUBLIC` on 2026-11-15. So the leg is free while the repository is public, and the workflow says
so.

This order adds two jobs that run on macOS: one for each crate under `packages/`, and one for the
engine's loader, core and conformance tests. That is what the row asks, and no more: its done-when
is that `gh pr checks` lists a passing macOS job for each crate under `packages/` and for those
three engine suites. Both jobs feed the `rust` gate, the check that D11
([decisions](../proposals/full-review/decisions.md)) will ask the owner to require on `main`.
[Order 152](152-ci-engine-shards-stand-alone.md) gave the engine its own build job and had the
discovery job, `rust directories`, find crates two levels down, and
[order 174](174-vector-changes-declared.md) added a step to the `docs` job.
[Order 185](185-promise-standing-generated.md) adds another step to `docs`, and order 189, written
beside this one, adds a Linux job that repeats changed tests. Build on `checks.yml` as it is on
`main`, and leave whatever of theirs has merged as it is. The macOS jobs stay separate from 189's
job.

## The decisions it applies

1. **A standard runner, free only while the repository is public.** Both new jobs run on
   `macos-latest`, GitHub's standard macOS runner. Never use a label that ends in `-large` or
   `-xlarge`: those runners are billed. Lines 1 to 3 of `checks.yml` become:

   ```yaml
   # The repository's checks, as AGENTS.md lists them, on every pull request and every push to main.
   # Every job runs on a standard GitHub-hosted runner: `ubuntu-latest`, or `macos-latest` for the
   # macOS jobs. Only a public repository runs them with no quota and no charge, and ruling 64,
   # point 3, allows nothing that costs credits. Nothing here uses a larger runner, a paid service
   # or a secret.
   ```

2. **Discovery lists the packages for macOS.** The `find` step of `rust-dirs` (lines 174 to 194)
   keeps every line it has. After its last line, `echo "matrix=$matrix" >> "$GITHUB_OUTPUT"`, it
   gains these lines, at the step's indentation:

   ```bash
   packages=$(echo "$matrix" | jq -c '[.[] | select(.dir | startswith("packages/")) | {dir}]')
   if [ "$(echo "$packages" | jq length)" -eq 0 ]; then
     echo "No crate was found under packages/, so the macOS jobs would test none."
     exit 1
   fi
   echo "packages=$packages" >> "$GITHUB_OUTPUT"
   ```

   The list holds every package entry, whatever its `build` and `lint` say: a package listed in
   `macos_only` would be built on macOS. Under `outputs` (lines 168 to 169), `rust-dirs` gains
   `packages: ${{ steps.find.outputs.packages }}` beside `matrix`. The comment above the job
   (lines 158 to 163) gains one sentence at its end: "The output `packages`, the entries under
   packages/, is the matrix of the macOS package job." The `matrix` output and the Linux
   matrix do not change.

3. **One macOS job for each package.** A new job, `macos-package`, goes after `engine-doc` and
   before the comment above `rust`. It is set up as `rust-dir` is at lines 196 to 209, with these
   differences: `name: macos (${{ matrix.dir }})`, `runs-on: macos-latest`, and
   `include: ${{ fromJSON(needs.rust-dirs.outputs.packages) }}`. It keeps `needs: rust-dirs`,
   `timeout-minutes: 90`, `fail-fast: false` and `working-directory: ${{ matrix.dir }}`. Its
   steps, in this order, each named as `rust-dir` names its own:
   - the checkout, as in `rust-dir`, with `persist-credentials: false`;
   - "Install the toolchain that rust-toolchain.toml pins", as at lines 214 to 219, but with
     `rustup component add clippy` in place of `rustup component add rustfmt clippy`;
   - "Restore the cargo cache", as at lines 220 to 230, with its `id: cache` and the same paths,
     key and restore key. On this runner `runner.os` is `macOS`, so a macOS cache never meets a
     Linux one;
   - `cargo clippy --all-targets -- -D warnings`, with no `if`;
   - `cargo test --release`, with `if: ${{ !cancelled() }}`;
   - the save of decision 5.

   It runs no `cargo fmt`: rustfmt reads only the source, so its result is the same on every
   system, and `rust (directory)` already checks it on Linux for every package.

4. **One macOS job for the engine's loader, core and conformance tests.** A new job,
   `macos-engine`, goes right after `macos-package`, with
   `name: "macos (engine: loader, core, conformance)"` (quoted, because a colon followed by a
   space inside an unquoted YAML value is read as a mapping, and the workflow would not load),
   `runs-on: macos-latest`, `timeout-minutes: 90` and `working-directory: engine`. It has no
   `needs`. Its steps, in this order:
   - the checkout, with `persist-credentials: false`;
   - "Install the toolchain that rust-toolchain.toml pins", running
     `rustup toolchain install --no-self-update --profile minimal`, `rustup show active-toolchain`
     and `cargo --version`;
   - "Restore the cargo cache", with `id: cache` and the same paths, key and restore key as
     `engine-build`'s step (lines 271 to 281);
   - one step named and run as
     `cargo test --release -p conduit-loader -p conduit-core -p conduit-conformance`;
   - the save of decision 5.

   `conduit-conformance` depends only on the loader and the core
   (`engine/crates/conformance/Cargo.toml`), so the shell is neither built nor tested. The
   command runs the three crates' unit, integration and doc tests, and the conformance vectors
   are among them. It sets no `CONDUIT_VECTOR_THREADS`, so the runner's processor count sets the
   vectors' threads, as on Linux. If the vectors run out of memory there, a later round sets the
   variable to 2 on that step alone. The job runs no fmt, no clippy and no nextest: the row asks
   for tests, and `engine build` lints the whole engine on Linux.

5. **macOS caches are saved from `main` alone.** Each macOS job restores its cache on every run.
   Its last step, "Save the cargo cache", uses `actions/cache/save@v6` with the paths of its
   restore step and `key: ${{ steps.cache.outputs.cache-primary-key }}`, under
   `if: ${{ !cancelled() && github.event_name == 'push' && steps.cache.outputs.cache-hit != 'true' }}`.
   The workflow runs on `push` for `main` only, so only `main` saves. The reason: GitHub keeps
   at most 10 GB of caches for a repository, and on 2026-11-15 the repository held 43 caches at
   about that limit, with `main`'s own taking about half. A pull request's cache serves only that
   pull request. The thirteen macOS caches are expected to weigh about as much as `main`'s Linux
   caches for the same crates, so saving them from every pull request would push `main`'s out. A
   pull request can still restore `main`'s macOS caches. The comment above the two jobs says what
   they are and why they save only from `main`:

   ```yaml
   # The Mac run that orders built on Windows had by hand (the full review's FR-271, point 3, and
   # FR-493, gate 2), as two macOS jobs: each package's clippy and release tests, and the release
   # tests of the engine's loader, core and conformance crates. Formatting reads only the source
   # and is checked on Linux. Their caches are saved from pushes to main alone (work order 187).
   ```

   The Linux jobs' caches do not change.

6. **The `rust` gate needs both.** `rust` (lines 385 to 401) adds `macos-package` and
   `macos-engine` to its `needs`, and keeps the five it has. Its step's `env` gains
   `MACOS_PACKAGES: ${{ needs.macos-package.result }}` and
   `MACOS_ENGINE: ${{ needs.macos-engine.result }}`, and its `echo` line ends with
   `; macos packages: $MACOS_PACKAGES; macos engine: $MACOS_ENGINE`. Its `test` line ends with
   `&& test "$MACOS_PACKAGES" = success && test "$MACOS_ENGINE" = success`. So a macOS job that
   fails, is skipped or is cancelled fails `rust`, as any other Rust job does, and the gate's line
   names each result (ruling 52). The comment above the job (lines 383 to 384) begins "One result
   for all Rust directories, all engine shards, the doctests and the macOS jobs." If order 189 has
   merged, leave its job and its result in `rust` as it put them.

7. **What stays off macOS.** The row and its two findings name the packages and the engine's
   three suites, and nothing else.
   - No experiment joins. Experiment 005 is the only probe in `macos_only` (line 176). It could
     build on macOS, but it stays listed without being built, as now, and so does its Linux entry.
     Neither finding names it, and an experiment is evidence kept as it ran (line 160). A later
     order may add it; this one does not.
   - The engine's shell crate, `engine build`, the four nextest shards and `engine doctests` stay
     on Linux alone.
   - Apart from decisions 1, 2 and 6, nothing that exists in the workflow changes.
     `.github/workflows/nightly.yml` does not change either.

8. **A crate that fails on macOS is fixed inside itself.** You run every package's clippy and
   tests, and the engine's three crates' tests, on the Mac (see Done when). Where one fails on
   macOS, make the least fix inside that crate's directory, to its tests or its code, so that it
   checks and does the same thing on both systems. Say what you changed and why. The rules:
   - Do not mark a test ignored, remove it, limit it to one system or narrow what it asserts. A
     test that expects one system's message or path may accept each system's, where the code
     gives each.
   - Add no retry (order 068: no retries anywhere) and no repetition
     ([D12](../proposals/full-review/decisions.md): never repetition alone). A test's own wait
     that is too short for a slower machine may be lengthened; say from what to what, and why.
   - Change nothing under `spec/`, `conformance/` or `experiments/`, and no engine crate but the
     loader, the core and the conformance runner. No promise changes, so rule 8 of
     [CONTRIBUTING.md](../../CONTRIBUTING.md) asks nothing of this order. No case changes, so it
     writes no `187-declared.yaml` (order 174).
   - Stop and report, giving the test's name and the line that states its failure, if a test
     cannot pass on macOS because of what it checks (for example, a facility macOS lacks), if a
     conformance case fails on macOS, or if the fix would change what the crate does on Linux.
     The coordinator decides.

   The runner's macOS version may differ from the Mac's. A failure that only one of them shows is
   still a failure. Where a macOS job fails on CI while its tests pass on the Mac, the admin gives
   the next round that test's name and the lines that state its failure, and the same rules
   apply.

9. **The method as run** is updated in the same change (D13 of the
   [decisions](../proposals/full-review/decisions.md)). In
   `docs/implementation/METHOD_AS_RUN.md`, line 219, which begins "Orders built on Windows pass",
   becomes these four lines. Line 220, which begins "failed to compile on macOS", does not change:

   ```text
   Orders built on Windows passed the changed crate's tests, fmt and clippy on the Mac before merge.
   Since 187, CI's macOS jobs run each package's clippy and release tests and the engine's loader,
   core and conformance tests, and Linux checks fmt; only the experiments, the engine's shell and
   the engine's clippy still run on the Mac by hand. 131
   ```

   Lines 286 and 287, the first of which begins with the job name `engine tests (1/4)` and the
   second "Rust job groups feed the", become these four lines. Line 288, which begins "engine
   jobs depend on its build alone", does not change:

   ```text
   `engine tests (1/4)` through `(4/4)` use that archive; `engine doctests` runs doctests. Since 187,
   `macos (directory)` runs clippy and release tests for each package on `macos-latest`, and
   `macos (engine: loader, core, conformance)` runs those three crates' release tests there. Each
   of these Rust jobs feeds the `rust` gate: failed, skipped or cancelled jobs fail it. Since 152 the
   ```

   Order 185 turns lines 281 to 283 of the same paragraph into four lines. If it merges first,
   these lines move down by one: find them by their words. If it, or order 189, has changed these
   lines' words first, keep its words and put these sentences in beside them, at most 100
   characters a line.

## Scope

- `.github/workflows/checks.yml`: lines 1 to 3; the job `rust-dirs` (its comment, its `outputs`
  and the end of its `find` step); two new jobs, `macos-package` and `macos-engine`, with their
  comment, between `engine-doc` and the comment above `rust`; and the job `rust` with its
  comment.
- `docs/implementation/METHOD_AS_RUN.md`: line 219 and lines 286 to 287.
- Only where decision 8 needs a fix: files inside the directory of each package that fails on
  macOS, under `packages/`, or inside `engine/crates/loader/`, `engine/crates/core/` or
  `engine/crates/conformance/`.

Change nothing else: no other job or workflow, nothing under `spec/`, `conformance/` or
`experiments/`, no case or fixture, and no declaration.

## Done when

Run this order on the Mac, not on the Windows computer: its checks are the macOS runs. Work from
the repository root. Keep scratch files under `target/wo187/`, which Git ignores:

```sh
mkdir -p target/wo187/empty
git show HEAD:.github/workflows/checks.yml > target/wo187/base.yml
```

Write this check, unchanged, to `target/wo187/check.py`, and commit nothing of it:

```python
import sys
import yaml

new = yaml.safe_load(open(sys.argv[1]))["jobs"]
base = yaml.safe_load(open(sys.argv[2]))["jobs"]
macos = ("macos-package", "macos-engine")
pkg, eng, gate = new["macos-package"], new["macos-engine"], new["rust"]
assert pkg["runs-on"] == eng["runs-on"] == "macos-latest", "runner"
assert pkg["name"] == "macos (${{ matrix.dir }})", "package job name"
assert eng["name"] == "macos (engine: loader, core, conformance)", "engine job name"
assert pkg["needs"] in ("rust-dirs", ["rust-dirs"]) and "needs" not in eng, "needs"
assert pkg["strategy"]["matrix"]["include"] == "${{ fromJSON(needs.rust-dirs.outputs.packages) }}", "matrix"
assert new["rust-dirs"]["outputs"]["packages"] == "${{ steps.find.outputs.packages }}", "output"


def find(jobs):
    return next(s["run"] for s in jobs["rust-dirs"]["steps"] if s.get("id") == "find")


assert find(new).startswith(find(base)) and find(new) != find(base), "find step"
runs = {job: [s.get("run") for s in new[job]["steps"]] for job in macos}
assert runs["macos-package"].count("cargo clippy --all-targets -- -D warnings") == 1, "clippy"
assert runs["macos-package"].count("cargo test --release") == 1, "package tests"
assert runs["macos-engine"].count(
    "cargo test --release -p conduit-loader -p conduit-core -p conduit-conformance") == 1, "engine tests"
assert not any(r and r.startswith("cargo fmt") for job in macos for r in runs[job]), "fmt"
for job in macos:
    saves = [s for s in new[job]["steps"] if s.get("uses", "").startswith("actions/cache/save")]
    assert len(saves) == 1 and "github.event_name == 'push'" in saves[0]["if"], job + " cache save"
assert set(base["rust"]["needs"]) | set(macos) == set(gate["needs"]), "gate needs"
assert all("needs." + job + ".result" in str(gate["steps"]) for job in macos), "gate results"
gate_run = "".join(s.get("run", "") for s in gate["steps"])
assert all('test "$' + v + '" = success' in gate_run for v in ("MACOS_PACKAGES", "MACOS_ENGINE")), "gate test"
for job in base:
    if job not in ("rust-dirs", "rust"):
        assert new[job] == base[job], job + " changed"
assert set(new) == set(base) | set(macos), "jobs"
print("macOS jobs: ok")
```

Then run these one at a time:

```sh
/usr/bin/python3 target/wo187/check.py .github/workflows/checks.yml target/wo187/base.yml   # prints macOS jobs: ok
/usr/bin/python3 target/wo187/check.py target/wo187/base.yml target/wo187/base.yml          # fails: KeyError: 'macos-package'
/usr/bin/python3 -c 'import yaml; print(next(s["run"] for s in yaml.safe_load(open(".github/workflows/checks.yml"))["jobs"]["rust-dirs"]["steps"] if s.get("id") == "find"))' > target/wo187/find.sh
: > target/wo187/output
GITHUB_OUTPUT=target/wo187/output bash -e target/wo187/find.sh > /dev/null
sed -n 's/^packages=//p' target/wo187/output | jq -r '.[].dir' | sort > target/wo187/found
ls packages/*/Cargo.toml | sed 's,/Cargo.toml$,,' | sort | diff - target/wo187/found        # prints nothing
(cd target/wo187/empty && GITHUB_OUTPUT=../empty-output bash -e ../find.sh); echo "exit $?"  # ends with decision 2's line, then exit 1
```

The second command is the check that fails with the change removed. With `main`'s workflow,
the check stops at the first line that reads a macOS job. Report its last line.

Then, in each directory under `packages/` that holds a `Cargo.toml` (twelve on `main`), run these
one at a time, from inside that directory:

```sh
cargo clippy --all-targets -- -D warnings
cargo test --release
```

From `engine/`:

```sh
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-loader -p conduit-core -p conduit-conformance
```

Since order 071, the runner uses every thread the machine offers unless `CONDUIT_VECTOR_THREADS`
names a number, and at 24 threads it ran out of memory. Hence the 4.

Where decision 8 led to a fix, the run before it is that fix's check that fails with the change
removed: report the failing test's name and the line that stated its failure, then the passing
run after the fix. If decision 8 changed a package, also run `cargo fmt --check` in that package.
If it changed an engine crate, also run, from `engine/` and one at a time, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `CONDUIT_VECTOR_THREADS=4 cargo test --release`.

Then, from the repository root, each passes:

```sh
tr '\n' ' ' < docs/implementation/METHOD_AS_RUN.md | grep -c 'Each of these Rust jobs feeds the `rust` gate'   # prints 1
tr '\n' ' ' < docs/implementation/METHOD_AS_RUN.md | grep -c 'only the experiments, the engine.s shell and the engine.s clippy still run on the Mac by hand'   # prints 1
grep -c 'Those five' docs/implementation/METHOD_AS_RUN.md   # prints 0
python3 tools/check_docs.py                                 # Errors: 0
python3 tools/check_private_paths.py                        # Errors: 0
git diff --check
git status --short
```

`git status --short` lists only files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own. Name every fix that decision 8 led to, with its crate, its
reason and its failing line before the fix, and give the last line of every check, including the
failing run's `KeyError` line.

The admin then pushes the branch and opens its pull request. Once CI has run, from the repository
root, with `PR` set to the pull request's number:

```sh
gh pr checks "$PR" --json name,bucket --jq '.[] | select(.name | startswith("macos (")) | "\(.bucket) \(.name)"' | sort
gh pr checks "$PR" --json name,bucket --jq '.[] | select(.name == "rust") | .bucket'
```

The first command prints thirteen lines, each beginning `pass`:
`pass macos (engine: loader, core, conformance)`, and `pass macos (packages/NAME)` for each of
the twelve packages. The second prints `pass`. The other Linux jobs pass as before. The Record
gives each macOS job's time and the run's total. A fresh Codex verifier and a fresh read-only Opus
reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).

## Record

Run on 2026-11-15 on the Mac from `main` `f383edaf`: the worker (1,201 s) and a fresh Codex
verifier (348 s), which found nothing to change in one round and could not run CI. The first lines
of `checks.yml` now say that every job runs on a standard runner, free only while the repository
is public. `rust directories` gains the output `packages`, the entries under `packages/`, and
fails, saying why, if it finds none. Two new jobs run on `macos-latest`: `macos (packages/NAME)`
runs clippy and the release tests of each of the twelve packages, and
`macos (engine: loader, core, conformance)` runs those three crates' release tests, with no thread
count set for the vectors. Each restores its cache on every run and saves it only on a push to
`main`, under a key that names the system. The `rust` gate needs both, and its line names their
results. `METHOD_AS_RUN.md` says what CI now runs on macOS and what still runs on the Mac by hand.
No crate needed a fix: on the Mac, the twelve packages' clippy and release tests (268 tests) and
the engine's three crates' tests (149, at `CONDUIT_VECTOR_THREADS=4`) passed.

A fresh read-only Opus reader then verified the order (ruling 79, point 2). It reran
`check_docs.py`, `check_private_paths.py` and `git diff --check`, and parsed the workflow with
PyYAML. It ran the order's check, which printed `macOS jobs: ok` on the branch and stopped at
`KeyError: 'macos-package'` on `main`'s file, and the discovery step, which listed the twelve
packages and, in an empty directory, printed decision 2's line and exited 1. It ran the vectors
test at 3 threads: it passed in 156 s, at a peak of about 4.7 GiB resident. It found no must and
three shoulds. Two were applied before merge: the comment above `rust` is re-wrapped to 100
characters, and `macos-package` has `rust-dir`'s comment on why it runs inside each directory.
The third changes nothing before CI, as decision 4 plans: the runner has 3 vCPUs and 7 GB, so if
`macos (engine: loader, core, conformance)` runs out of memory there, a later round sets
`CONDUIT_VECTOR_THREADS` to 2 on that step alone. The fix also found one test likely to fail on
the runner. `exported_type_template_passes_semantic_registration_rules`, in
`packages/provider-local-process`, runs `tools/check_schemas.py` under `python3`, which needs
PyYAML. The Linux runner's `python3` has it, since that job passes, and no Rust job installs it.
On the Mac the test passed only because `/usr/bin/python3` has PyYAML installed. The pull
request's CI, with each macOS job's time and the run's total, is for the admin to add: the
thirteen jobs run at most five at a time, and the first run builds without a cache.

A second fresh Opus reader read the coordinator's fix (ruling 79, point 2) and found no must. The
coordinator applied its two shoulds: the macOS package job installs Python with PyYAML and
jsonschema, as the docs job does, since `provider-local-process`'s template test runs
`tools/check_schemas.py`, which would otherwise fail on the runner; that test's failure message now
gives the checker's standard output as well as its standard error (ruling 52); and this Record
names the thread count of the local engine run.

The pull request's first CI run showed the leg's worth at once: `provider-local-machine`'s three
MCP tests failed on the macOS runner while passing on the Mac. The coordinator made the test's
assertion give the provider's reply (ruling 52), which named the cause: "OS hostname must contain 1
to 63 characters after normalization; no truncation". GitHub's macOS runners have longer hostnames,
which the provider refuses by design. The macOS package job now sets a short hostname before its
tests, with a comment that says why.
