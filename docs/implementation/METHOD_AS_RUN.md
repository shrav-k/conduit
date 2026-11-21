# The work-order method as run

Not binding.

This page gives the steps by which the coordinator writes, dispatches, checks and merges an order,
as run on 2026-11-15, with each command. The runners are scripts outside the tree, beside [the
private handoff note](HANDOVER.md), which holds every private value. The tree keeps this page
because a copy that does not run would drift and the tree's code is Rust
([D62](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/decisions.md), logged for the owner to strike). A merge changing how
orders are dispatched or checked updates this page in the same pull request
([D13](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/decisions.md)). Until the S1 rerun's pin, one coordinator runs it
([ruling 78, point 1](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-78/README.md)); the lane uses the same runners when it
restarts at the pin ([Restarting the lane
coordinator](HANDOVER.md#restarting-the-lane-coordinator)).

## Variables

| Variable | Stands for | Private | Default |
| --- | --- | --- | --- |
| `<WO_DIR>` | the coordinator's working directory beside the repository, holding the worktrees, the runners and the outputs | yes | in the private note |
| `<MAC_CHECKOUT>` | the main checkout of the repository on the Mac, from which worktrees are made | yes | in the private note |
| `<MAC_HOME>` | the home directory of the Mac's user | yes | in the private note |
| `<KEYS_DIR>` | the directory beside the runners holding the SSH configuration for the Windows computer | yes | in the private note |
| `<WIN_HOST>` | the SSH host alias of the run account on the Windows computer | yes | in the private note |
| `<REMOTE_DIR>` | the directory in the Windows run account's home that holds the remote runner, its clone and its outputs | yes | in the private note |
| `<NNN>` | the order's number | no | none |
| `<ORDER_NAME>` | the order's file name without .md, such as 162-dispatch-deadline-cancels | no | none |
| `<BASE_COMMIT>` | the main commit the order started from | no | none |
| `<CHECKS>` | the checks the coordinator names for that order, usually the order's Done when | no | none |
| `<FINDINGS_FILE>` | a file in the coordinator's private notes holding a reader's structured result | yes | in the private note |
| `<FIXER_REPORT>` | the fixer's structured report, pasted | no | none |
| `SCHEMA` | the report schema's name under docs/work-orders, without .schema.json | no | the order's own name |
| `EXTRA` | a file appended to the order's prompt under From the admin session | no | none |
| `PREP` | a script run with the new worktree's path before the worker starts, to place untracked inputs | no | none |
| `ROUNDS` | the most rounds, the first included | no | 2 |
| `BASE` | the ref the order starts from; required by the continue runner | no | origin/main |
| `JOBS` | cargo's build jobs per worker | no | 6 (8 on the Windows computer) |
| `TT` | test threads per worker | no | 4 (6 on the Windows computer) |
| `APIKEY` | 1 to use an OpenAI API key from the environment instead of the ChatGPT allowance | no | none |
| `START_ROUND` | the continue runner resumes from verify-report-N.json | no | 1 |
| `VERIFY_FIRST` | 1 when the worker ran but its verifier did not; the continue runner verifies first | no | none |
| `HOME` | the caller's home directory, for cargo's directory and the compile cache | no | none |

A private value is never written in the tree, in an order, in a Record or in a pull request.

## 1. Write the order

The form used by orders 162 to 169 is `docs/work-orders/NNN-name.md`: a preamble (read AGENTS.md;
report schema; what the network is for; never print the environment or a whole large file; one cargo
command at a time), What and why, The decisions it applies with sources, Scope, Done when with
commands, and at merge the Record. See [An order](WORK_ORDERS.md#an-order) for the design. Name its
own schema or the shared `004-i2-01-vectors-and-first-start` with `SCHEMA=` at dispatch. Merge the
order file to `main` first, on `orders/NNN` (#245, #258): runners read order files from
`origin/main` whatever the base.

## 2. Dispatch on the Mac

Call the runner below; omit `net` for an offline order. `NNN-name` means `<ORDER_NAME>` throughout;
`NNN` is `<NNN>`. Optional caller settings are in the table, including `EXTRA` and `PREP`.
```sh
SCHEMA=004-i2-01-vectors-and-first-start "<WO_DIR>/run-order.sh" NNN-name net
```
The runner makes the output directory, fetches, adds the worktree, optionally runs `PREP`, pins
`BASE` to a commit, then extracts the orders. Commands in order:
```sh
mkdir -p "<WO_DIR>/out-NNN"
git -C "<MAC_CHECKOUT>" fetch -q origin
git -C "<MAC_CHECKOUT>" worktree add -q --no-track -b wo/NNN-name "<WO_DIR>/NNN" "$BASE"
bash "$PREP" "<WO_DIR>/NNN"
git -C "<MAC_CHECKOUT>" rev-parse "$BASE"
rm -rf "<WO_DIR>/out-NNN/work-orders"
git -C "<MAC_CHECKOUT>" archive origin/main docs/work-orders | tar -x -C "<WO_DIR>/out-NNN" --strip-components=1
```
`PREP` runs only when supplied. A failed worktree add, preparation or base resolution exits 1. A
stacked order sets `BASE` to the unfinished branch. A leftover branch fails the worktree add; the
runner does not delete or reuse it. Checks for an existing `SCHEMA` or a load limit are not recorded
in the runner. The resolved `BASE` stays fixed across later fetches. `--no-track` avoids the shared
Git configuration race. For `net`, it exports `SCCACHE_DIR=$HOME/.cache/conduit-sccache` and
`SCCACHE_CACHE_SIZE=40G` and starts the shared cache server, ignoring a failure:
```sh
/opt/homebrew/bin/sccache --start-server >/dev/null 2>&1
```
Below, `W` is `<WO_DIR>/NNN`, `R` is `<WO_DIR>/out-NNN`, `O` is its `work-orders` directory and
`NAME` is `NNN-name`; `suffix` is empty for round 1, then `-N`; `prompt` is `R/order-prompt.md`,
then `R/round-N-prompt.md`. For `net`, `OFF` is `CARGO_NET_OFFLINE=false`, `NET` is `-c
sandbox_workspace_write.network_access=true` and `CACHE` is
`RUSTC_WRAPPER=/opt/homebrew/bin/sccache`; otherwise `OFF` is `CARGO_NET_OFFLINE=true` and both are
empty. `SCRUB` holds `-u` and each caller variable name matching
`KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL`, removed before either Codex starts. `APIKEY=1` keeps only
`OPENAI_API_KEY` of those and supplies `API` below, else empty: the owner's 2026-11-11 API
exception, capped at $50 that night, not a new leave to use a key. No credential is read or written
here. The provider flags are:
```sh
-c 'model_providers.openai-api={name="OpenAI API",base_url="https://api.openai.com/v1",env_key="OPENAI_API_KEY",wire_api="responses"}' -c 'model_provider="openai-api"'
```
The worker command is:
```sh
env "${SCRUB[@]}" $OFF CARGO_BUILD_JOBS=$JOBS RUST_TEST_THREADS=$TT NEXTEST_TEST_THREADS=$TT "${CACHE[@]}" codex exec --ignore-user-config --strict-config --disable apps "${API[@]}" \
  -s workspace-write --add-dir "$HOME/.cargo" "${NET[@]}" -C "$W" -c 'model_reasoning_effort="high"' \
  --output-schema "$O/${SCHEMA:-$NAME}.schema.json" -o "$R/worker-report$suffix.json" --json \
  - < "$prompt" > "$R/worker-events$suffix.jsonl" 2> "$R/worker-stderr$suffix.txt"
```
- `--ignore-user-config` excludes the owner's configuration and MCP servers; sign-in still uses the
  owner's subscription (48 point 2). `--strict-config` rejects an unknown key: A70 lost a task to an
  override silently ignored (mistake 2).
- `--disable apps` excludes ChatGPT-login apps (48 point 1). `-s workspace-write` permits writes in
  the tree; `--add-dir` also permits cargo's cache. Offline crates come from that cache. `net`
  allows fetching crates and loopback binds, refused offline in order 010's second round.
- `-C` selects the worktree. `-` closes standard input after the prompt: A70 once waited five
  minutes on an open input. The prompt holds the archived order and, if `EXTRA` is supplied, its
  contents under `## From the admin session` (080, 081).
- `--output-schema` and `-o` make the report a file. `--json` keeps events, read only when the
  report needs explaining: events omit some commands (A70, surprise 7), so check claims in files.

Every flag in WORK_ORDERS.md's former command is passed. `JOBS` caps `CARGO_BUILD_JOBS`; `TT` caps
both `RUST_TEST_THREADS` and `NEXTEST_TEST_THREADS`. No model is passed: it is the client's default
on the day of the run. The effort is `high` for worker and verifier (FR-527). A nonzero worker exit
stops without a verifier. After a successful worker, the checkpoint is:
```sh
git -C "$W" add -A && git -C "$W" -c user.name="Conduit work order" -c user.email=work-orders@conduit.invalid commit -q -m "wip: worker output of order NNN (checkpoint, squashed on merge)"
```
Round `N`'s checkpoint is the same command with the message `wip: round N of order NNN`. Both commit
as `"Conduit work order" <work-orders@conduit.invalid>`, the identity section 5 sets on Windows, so
no checkpoint names the Mac's user or host. A failed checkpoint does not stop the verifier.

## 3. The Codex verifier and the rounds

The verifier receives the archived [standing order](../work-orders/verify.md), the order path, the
worker's report and the full order text, in that order. Its prompt names the fixed base and `git
diff BASE..HEAD` for all rounds, and `git show HEAD` for the latest checkpoint. It is not shown
`EXTRA` or an earlier round's findings: only the order's file from `origin/main`, the latest worker
report and the commits since the base. It runs:
```sh
env "${SCRUB[@]}" codex exec --ignore-user-config --strict-config --disable apps "${API[@]}" \
  -s read-only -C "$W" -c 'model_reasoning_effort="high"' \
  --output-schema "$O/verify.schema.json" -o "$R/verify-report$suffix.json" --json \
  - < "$R/verify-prompt$suffix.md" > "$R/verify-events$suffix.jsonl" 2> "$R/verify-stderr$suffix.txt"
```
The verifier tries to refute each change against the decisions and cited texts; it reports must,
should or nit in `verify.schema.json`. `read-only` protects the tree; other flags are above. A must
or should starts another worker and fresh verifier, up to `ROUNDS` in all; nits do not. The next
prompt repeats the order and `EXTRA`, names the earlier commits and the latest one, and includes the
musts and shoulds. It asks for in-scope fixes, a report of out-of-scope fixes, rerun checks and
`RUST_TEST_THREADS=2` for timing-sensitive failures under load. A failing verifier is logged but
does not stop the runner. A missing or unreadable JSON report, or missing `findings`, counts as 0
findings; no schema validation is recorded in the runner. It can thus end as if there were no
findings, as [experiment 028's harness](../../experiments/028-a78-dogfood/inputs/harness.md)
observes. A reviewer reading cold found the first round's most consequential faults: a contradiction
across nine promises, and a wrong claim about Codex in the coordinator's own notes.

After the last round with a must left, or an Opus reader finding one, the coordinator reads the
findings and the branch. The Record names one of three outcomes (D5):

- A fault of the same kind as an earlier pass means the mechanism is wrong: a new order replaces it,
  as 150 replaced the handoff of 146 and 149.
- A new, local, small fault is fixed by the coordinator, then read by a fresh Opus reader.
- Otherwise, a finishing order names each finding and its kind and stacks on the branch; the pair
  merges together: 114 on 111, 115 on 113, 124 on 122, 125 on 123, 130 on 129, 140 on 137,
  148 on 142; 149 and 150 on 146 (#260).

There is no fixed stop at a fourth pass. Applied shoulds also get another fresh Opus reader, as the
Records of [153](../work-orders/153-model-account-checked.md#record),
[156](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/156-runner-stopped-after-end.md#record) and
[168](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/168-tidy-after-153-155.md#record) show.

For an order of A77's sample whose round 1 is a Codex worker's and whose group still lacks five
paired orders, while the Claude allowance allows, a fresh Claude Opus reader also reads round 1's
checkpoint, given only what the Codex verifier is given and running nothing that writes; neither
sees the other's report. Such an order is dispatched with `ROUNDS=1`. The coordinator judges both
reports' findings with their readers' names removed, and the accepted musts and shoulds of both
start round 2 through the continue runner. Each of A77's orders keeps a record in
`docs/work-orders/NNN-a77.yaml`.
[A77's measures](WORK_ORDERS.md#a77s-measures-registered-before-the-first-paired-read) give the
rules.

## 4. Continue from a report

The continue runner is called `round-order.sh`. It uses the existing worktree and output directory:
```sh
SCHEMA=004-i2-01-vectors-and-first-start BASE="<BASE_COMMIT>" START_ROUND=2 ROUNDS=3
"<WO_DIR>/round-order.sh" NNN-name net
```
`BASE` is required: the original starting commit, not the latest checkpoint (the first runner's
`base=` line). It fetches, resolves that base and re-extracts `origin/main`'s orders as section 2
does, adding no worktree and running no `PREP`. It rebuilds the order prompt, with any resupplied
`EXTRA`, and uses section 2's worker and verifier. `START_ROUND` defaults to 1, reading
`<WO_DIR>/out-NNN/verify-report.json`; `START_ROUND=N` above 1 reads `verify-report-N.json` there.
`VERIFY_FIRST=1` first verifies the existing worker output into the unsuffixed `verify-report.json`,
before selecting `START_ROUND`'s report, so that combination does not verify a suffixed report. The
coordinator may write the report itself: to continue a checkpoint (070, 077 and 081 after the
Windows pause) or to fix a merge (042 after its rebase); 079 needed `VERIFY_FIRST=1`.

## 5. On the Windows computer

Call `run-order-win.sh`; it passes a resolved Mac base, schema, rounds, jobs and threads remotely:
```sh
SCHEMA=004-i2-01-vectors-and-first-start "<WO_DIR>/run-order-win.sh" NNN-name net
```
The remote copy runs under WSL. The SSH target and its run account use `<WIN_HOST>` and
`<KEYS_DIR>/ssh_config`; the runner records no separate account name. Its clone is
`~/<REMOTE_DIR>/repo`. The wrapper copies the remote runner and optional `EXTRA`, and fetches orders
through the clone's origin. It forwards no `PREP` or `APIKEY`: such orders run on the Mac. Commands:
```sh
mkdir -p "<WO_DIR>/out-NNN"
git -C "<MAC_CHECKOUT>" fetch -q origin
git -C "<MAC_CHECKOUT>" rev-parse "${BASE:-origin/main}"
scp -q -F "<KEYS_DIR>/ssh_config" "<WO_DIR>/run-order-remote.sh" "<WIN_HOST>:<REMOTE_DIR>/run-order.sh"
scp -q -F "<KEYS_DIR>/ssh_config" "$EXTRA" "<WIN_HOST>:<REMOTE_DIR>/extra-NNN.md"
ssh -F "<KEYS_DIR>/ssh_config" <WIN_HOST> "cd ~/<REMOTE_DIR> && git -C repo config user.name 'Conduit work order' && git -C repo config user.email work-orders@conduit.invalid && git -C repo fetch -q origin '+refs/heads/*:refs/remotes/origin/*' && chmod +x run-order.sh && $XR SCHEMA=$SCHEMA BASE=$BASEC ROUNDS=${ROUNDS:-2} JOBS=${JOBS:-8} TT=${TT:-6} nohup ./run-order.sh NNN-name net > out-NNN.log 2>&1 < /dev/null & echo remote-started"
```
`BASEC` is the resolved Mac commit; `XR` is empty or the remote assignment
`EXTRA=$HOME/<REMOTE_DIR>/extra-NNN.md` (the home expands remotely). Omit `net` remotely when
omitted locally. Copy or SSH failure exits 1. The remote worker's command and rustup directory
allowance are not recorded in the runner supplied here; [075's
Record](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/075-spec-own-tools-reach.md#record) says the runner now permits writes to
rustup's directory. Every 30 seconds the wrapper reads the log, prints only new lines, and stops on
`done rounds` or `failed`. After 20 failed contacts in a row (ten minutes) it exits 1 and fetches
nothing. Its header promises a Mac copy of the remote `out-NNN.log`; no command writes one.
Otherwise it fetches the branch, restores a local review tree and copies JSON reports and prompts:
```sh
ssh -F "<KEYS_DIR>/ssh_config" <WIN_HOST> "cat ~/<REMOTE_DIR>/out-NNN.log 2>/dev/null"
GIT_SSH_COMMAND="ssh -F <KEYS_DIR>/ssh_config" git -C "<MAC_CHECKOUT>" fetch -q "<WIN_HOST>:<REMOTE_DIR>/repo" "+wo/NNN-name:refs/remotes/win/wo/NNN-name"
git -C "<WO_DIR>/NNN" reset -q --hard refs/remotes/win/wo/NNN-name
git -C "<MAC_CHECKOUT>" branch -f wo/NNN-name refs/remotes/win/wo/NNN-name
scp -q -F "<KEYS_DIR>/ssh_config" "<WIN_HOST>:<REMOTE_DIR>/out-NNN/*.json" "<WO_DIR>/out-NNN/" 2>/dev/null
scp -q -F "<KEYS_DIR>/ssh_config" "<WIN_HOST>:<REMOTE_DIR>/out-NNN/*.md" "<WO_DIR>/out-NNN/" 2>/dev/null
git -C "<MAC_CHECKOUT>" worktree add -q "<WO_DIR>/NNN" wo/NNN-name
```
The reset runs only for an existing local tree; otherwise the branch is set, then the tree added.
Orders built on Windows passed the changed crate's tests, fmt and clippy on the Mac before
merge. Since order 193b the gate of [section 7](#7-merge-and-the-record) runs them there,
for each Rust directory the branch reaches and the engine. Since order 223, CI's macOS jobs
(187) run nightly and on request. 131
failed to compile on macOS (rustix's `mknodat`) and expected Linux's error message; 137 and 140 used
`chmod --` and `find -printf` (fixed in #241). Windows' sandbox could not use the pinned toolchain
for 075's spec suites ([075's Record](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/075-spec-own-tools-reach.md#record)) and timed
out 072's S1 HTTP test ([072's Record](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/072-shell-list-operation.md#record)). See [The
second machine](HANDOVER.md#the-second-machine) for the owner's pause and resume.

## 6. The Opus reader and fixes by hand

Once checks pass, a fresh read-only Claude Opus reader verifies each order before merge, and each
coordinator's fix the same way ([ruling 79, points 2 and 4](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md)). When
the Claude allowance runs low, A77's paired reads stop first and dispatch next
([ruling 80](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-80/README.md), point 6), nothing merges unread, and the owner is
told.
Each reader starts with no memory of the work and gives a structured verdict and findings.

The order reader gets its worktree, base and order, `git -C <WO_DIR>/<NNN> diff origin/main...HEAD`
and `git -C <WO_DIR>/<NNN> status --short`, the worker's and verifier's JSON reports (not events or
stderr), `<CHECKS>` and fix-removal checks on a scratch copy. The brief says "edit, create or delete
no file in the repository", "never print environment variables and never read credential files" and
"run one cargo command at a time"; it bars Git writes and network beyond test loopback, and builds
write only to `target/` or scratch. It asks: "Check hard: every decision of the order applied fully
and correctly; nothing outside its scope; the sources it cites read right ...; no test that cannot
fail; every result, refusal and message as explicit as ruling 52 asks; no private detail (local
paths, user names, account names, machine or network names); and anything else a careful reader
would block on", and "Return a verdict (ready to merge, or not) and every finding as must or should,
with where, the problem, the evidence and the exact fix." The fix reader gets `git -C <WO_DIR>/<NNN>
show HEAD`, the findings in `<FINDINGS_FILE>`, `<FIXER_REPORT>`, `<WO_DIR>/out-<NNN>.log` and
`<WO_DIR>/out-<NNN>/*.json`, and `<CHECKS>`; it checks every fix, that nothing else changed or
broke, the Record's accuracy and privacy, and that tests fail with the fix removed.

Where the coordinator gives a reader's findings to an Opus fixer, it gets `<WO_DIR>/<NNN>`, the
order and `<FINDINGS_FILE>`, applies every must and should within the scope, adds the Record in
156's form, runs `<CHECKS>` and commits with `git add -A` and a message beginning "admin:"; it does
not push, rebase, switch branches or touch any other worktree. `<FIXER_REPORT>` is its report.

A coordinator's last-finding fix rests on the admin's leave under [ruling
48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md), "the admin finishes the work itself", formerly in
WORK_ORDERS.md and now held here under D5; a fresh Opus reader reads the fix. [Ruling 71 point
4](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-71/README.md) is a different leave: settle a small finding that adds a
feature and log it for the owner to strike. Until the S1 rerun it excludes anything that could move
S1's author counts: CLI flags, short-form keys, templates and defaults ([IN_FORCE.md, topic
15](../../IN_FORCE.md#15-how-the-work-is-governed)). The handover's former step 9 cited that point
for both. Its examples were the last race of 122 and 124 (#207), 128's mips byte order, 136's
redaction of a library-built error, 137 and 140's macOS fixes (#241), and 141's extension (#249).

## 7. Merge and the Record

The admin reads the diff, report and every worker choice, then squashes with the guard:
```sh
git fetch origin && git rebase origin/main && git merge-base --is-ancestor origin/main HEAD && git reset --soft origin/main && git -c user.name="Conduit work order" -c user.email=work-orders@conduit.invalid commit
git diff --name-only HEAD^ HEAD
```
The squash commits with the checkpoints' identity, not the handover's plain `git commit`.
Check the second command's files against scope; shared remote refs can move between
commands. Then, on the Mac, with the tree clean, run the gate
([ruling 80](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-80/README.md), point 10 and its later answers), which
derives the private names from the Mac at each run:
```sh
python3 tools/gate.py --derive-private-names
```
It compares the branch with `origin/main` as last fetched, and never fetches, pushes or
calls GitHub. It fails a commit since `origin/main` whose author or committer is not the
work-order identity, as a plain rebase leaves one, and names the commit. For what the
branch changes it runs the checks of CI's `checks` workflow on the Mac: the `docs` job's
Python checks in that job's order,
with the private names it derives (the Mac's account, host and computer names, git's
global identity, the repositories beside this one and their remotes, and the tailnet's
peers, less those shorter than 4 characters, in the origin remote's URL or on its list of
public words) given to `tools/check_private_paths.py` alone and never printed;
the vector changes against the declarations the branch adds or changes and `git diff --check`.
The tool tests run with `--all` or when a change touches `tools/`, `.github/workflows/`, or a
Rust source, `Cargo.toml` or `Cargo.lock` under `packages/` or `experiments/`. The gate runs
`cargo fmt --check`,
clippy with `-D warnings` and `cargo test --release` in each package that the branch
changes, that depends by path on a directory it changes, or that reads a path it changes,
and the tests alone of each such experiment. Experiment 011 runs with `--all` or when its own
directory changes. The engine's fmt, clippy and release tests run
when `engine/`, `spec/`, `conformance/` or an Agent package changes. Gitleaks scans the branch's
commits when the release that checks.yml pins is on the `PATH`. Cargo runs offline.
`--target-cache DIR` uses one target directory per Rust directory, rebuilds the tree's own
crates and reuses registry crates. The gate holds `DIR/gate.lock` for the run. The gate prints
one line for each gate, PASS, FAIL or NOT RUN with its command
and why, and a last line that says PASS only when every required gate passed, and writes
its record to `target/gate/<HEAD>.json`. `--plan` runs `rust discovery` alone and lists
the other gates without running them, and exits 1 when discovery does not pass; `--all`
runs every Rust directory and the engine. A FAIL is fixed on the branch, which is squashed
again, and the gate runs again.

The merge waits on the gate, not on GitHub CI. Only after the gate's PASS, push
`wo/NNN-name`, open its pull request with the gate's last line and its record's gates in
the description, and squash-merge with a message written by hand, its subject in the form
of orders merged since 151: `Work order NNN: title (#PR)`. The record cited names the
pushed commit as its head, and says that the tree was clean, that the branch contained
`origin/main` and that the private names were configured; the pull request copies the
gate's `Private names:` lines, which give counts and sources and no name.
Where it says that gitleaks did not run, no secret scan ran before the push: GitHub's push
protection stops only the patterns it knows, and the pull request's `secrets` job scans
after. The gate's log of the vector changes lists each flip for the reader ([reviewing vector
changes](../../conformance/README.md#reviewing-vector-changes)).
[checks.yml](../../.github/workflows/checks.yml) still runs on pull requests and pushes to
`main`, as a second opinion and the Linux leg that nothing waits on (ruling 80). A run that
fails after a merge is fixed in a follow-up, an order or a coordinator's fix read by a
fresh Opus reader, whose Record names the run. Its jobs:
`secrets` scans added commits; `docs` runs ten Python checks (docs, names, schemas, roadmap,
rulings, model account with history, promise standing, private paths and, on pull requests,
vector changes against declarations and the decision framework), tools' unit tests and PR
`git diff --check`.
`rust directories` discovers crates;
`rust (directory)` runs fmt/clippy for packages, release tests for packages and experiments,
excluding the macOS-only probe. Since 189, on a pull request, `changed packages` lists each package
crate whose directory, or the directory of one of its path dependencies, the pull request changes,
and `package tests repeated (directory)` runs that crate's release tests five more times on Linux,
all five even after a failure; when it lists none, and on a push, the repeats are skipped. `engine
build` runs fmt, clippy and builds a nextest archive; `engine tests (1/4)` through `(4/4)` use that
archive; `engine doctests` runs doctests. These Rust jobs feed the `rust` gate: failed, skipped or
cancelled jobs fail it, except that the repeats may be skipped when `changed packages` lists no
crate. Since 152 the engine jobs depend on its build alone, so a package or experiment cannot skip
them. Once a day and on request, [nightly.yml](../../.github/workflows/nightly.yml) runs the
engine's tests with npm and the vectors on the differential build. Since 223, it also runs the
macOS jobs from order 187. `macos (directory)` runs clippy and release tests for each package on
`macos-latest`. `macos (engine: loader, core, conformance)` runs those three crates' release tests
there. 068 built CI (#198); since 070
(#240) engine shards take 1 to 8 minutes, down from 12 to 38. No check is required on `main` by a
branch setting. The admin decides what to fix, reorder or put to the owner.

The Record goes at the order's foot. Begin: "Run on DATE on the Mac from `main` COMMIT: the worker
(N s) and a fresh Codex verifier (N s), ...". Give rounds, findings, each Opus reader and what it
found, coordinator changes and own decisions, and findings left for later. [081's
Record](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/081-spec-one-run-per-instance.md#record) was the model; the fixer's brief now
uses [156's Record](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/156-runner-stopped-after-end.md#record). Orders 037 to 045,
049a1, 050a and 067 onward still await condensation into [the record](WORK_ORDERS.md#the-record)
(A80); 067 moved 001 to 036k and 060 to 066 there.

Each round's checkpoint is a local commit on `wo/NNN-name`. For an order run on the Windows
computer, it is also on the `win/` ref fetched to the Mac. Only the squash is pushed at merge, so
the round-by-round diffs are discarded after the merge, and only the Records describe the rounds
([D63](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/decisions.md)). An order stopped unmerged is pushed with its
checkpoints, as 049a1, 080 and 144 are.

## Load

The lane coordinator runs one heavy, engine-building order per machine; the admin ran at most two
engine orders on Windows until 070 merged. On the Mac dispatch only below load 8, with `JOBS` and
`TT` capped. No input reads the load: check by hand with `sysctl -n vm.loadavg`, whose first number
is the one-minute load. Under load, rerun timing-sensitive shell tests with `RUST_TEST_THREADS=2`.

## Failure modes

- Two orders started at once raced for Git's configuration; the runner now uses `--no-track`
  ([FR-525](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/G6-outside-the-tree.md#fr-525)).
- The Windows checkpoint lacked a git identity ([069b's
  Record](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/069b-spec-plan-apply-ruling-76.md#record)); the wrapper now sets it.
- Rustup could not write its Windows sandbox directory ([075's
  Record](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/075-spec-own-tools-reach.md#record), which says the runner allows it now);
  the allowance is not recorded in the runner supplied.
- The Windows script exited 127 after Codex finished; the admin committed and verified on the Mac
  ([079's Record](../work-orders/079-shell-https-address.md#record), which does not name the missing
  command). A remedy is not recorded in the runner.
- Two of 081's dispatches failed at once: missing `SCHEMA` and a leftover branch
  ([FR-532](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/G6-outside-the-tree.md#fr-532)). A schema precheck is
  not recorded in the runner; a leftover branch still fails the worktree add and stops dispatch.
- The read-only Codex verifier reruns no build or test
  ([156](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/156-runner-stopped-after-end.md#record),
  [167](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/167-a5-probe-signals-tested.md#record)); an automatic remedy is not recorded
  in the runner. The Opus readers of 156, [161](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/161-a78-stage-1-evidence.md#record)
  and 167 reran checks (section 6).
- A failed verifier or unreadable report can count as no findings, as section 3 says ([harness
  account](../../experiments/028-a78-dogfood/inputs/harness.md)); schema validation and an automatic
  stop are not recorded in the runner.

An order run through Conduit (A78) uses the harness in [Experiment
028](../../experiments/028-a78-dogfood/README.md), which keeps `run-order.sh`'s interface and log
lines.
