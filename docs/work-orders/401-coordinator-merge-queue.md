# Work order 401: the coordinator's merge queue, `wo queue`, in Rust in experiment 028 (DEC-174)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
is empty, since this order adds no case. It needs no network; if a crate is missing from the
cache, fetch nothing and report it. Run one cargo command at a time, and not `tools/gate.py`.
Never print a whole large file, case, diff or list: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Run no `conduit` outside a
test, no `codex` or `claude`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user,
machine or account name, or private repository name.

Run no `gh`. Your tests run git only in repositories they make in a temporary directory, with
git's global and system configuration off, and a fixture binary plays `gh`, the gate and the
repository's Python tools.

## What and why

**The owner's words.**
[DEC-174](../decisions/log.md#dec-174-2026-11-23-the-coordinators-rust-tooling-lives-in-experiment-028):
where the coordinator's Rust tooling lives -> "Inside experiment 028 (Recommended)".
[DEC-171](../decisions/log.md#dec-171-2026-11-23-the-coordinator-hands-off-through-written-state-its-kits-rust-replacements-follow-the-guard):
the kit -> "Order Rust replacements first". DEC-132: "Packages only (Recommended)", so the
coordinator's tooling stays Rust. DEC-180: "Parallel tracks (Recommended)": pairs 3 and 4 take
the kit's replacements now. A merge is squashed, waits on the local gate and not on CI, and
carries the work-order identity (DEC-030, DEC-089).

**Today.** The coordinator merges through a private kit outside the tree, a marked stopgap of
lock, queue and batch scripts. They rebase each branch just before its merge, settle known
conflicts, regenerate STANDING, squash, gate, push, merge with `gh` and skip a failed branch's
dependents. On 2026-11-23 batches 29 to 36 ran at once, so orders 302 to 304 failed on a moved
`main`. An order with no message was reset onto `main` with its changes staged. Order 278's
markers were committed mid-rebase. Batches waited on one session's files.

As order 400's What follows asks, this order regenerates STANDING and stops on any other
conflict. Order 401b adds the word merge, the changelog's keep-both merge and the fragment step.
The kit's pick of the replayed order text, and its merge of the expected failures by case id, go.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1). None is
at stake: no promise, case, engine source or package changes. The queue merges only a tree the
gate ran on, and only commits by the identity it checks. It reports by name, and does not undo,
a merge that lands on a `main` moved since its last fetch.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice. GitHub's merge queue, bors and Prow's Tide test each change on the tip
where it will land before they merge it. The lock is `flock(2)`, as Cargo locks its caches. Not
taken: GitHub's merge queue, whose checks are CI's (DEC-030); bors' batches; `git rerere`, which
replays a person's resolution; merge drivers (DEC-177); merging through Conduit, orders 412 to
414.

**When it runs.** On `wo/400-changelog-fragments` at `5ee79ece69801d16f01d144e8cc386c190a79c82`,
where order 400 is built and recorded, or on `main`: it needs nothing of 400's. Through Conduit:
one Codex worker turn, the blind Codex verifier and the Claude reader at once, one fix turn, the
gate (DEC-050, DEC-033). By ruling 89, each decision says what must hold; how is yours. The
coordinator's choices, open to the owner's strike (DEC-029): the commands, the configuration's
keys, the steps, the ledger's fields, the split with 401b, dropping the kit's picks, the exit
statuses and the fixture.

**Shared files.** Only the Files table of `experiments/028-a78-dogfood/README.md`: the later of
two orders that change it rebases and keeps both.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first the last line that `tools/gate.py` prints, `experiments/033-usage-guard-probe/probe/`
(manifest, lockfile, toolchain) and `experiments/022-provider-contracts/Cargo.toml`. In quoted
text, write each "[text] (link to `target`)" as a real link to that target.

1. **The crate** (DEC-174, DEC-132). `experiments/028-a78-dogfood/coordinator/` is its own
   workspace: package `a78-coordinator`, edition 2024, `publish = false`, one binary `wo` whose
   logic sits in a library, and the probe's `rust-toolchain.toml`. Its one dependency is
   `serde_json`, pinned and locked as the probe has it. Tests make temporary directories with the
   standard library. The fixture `queue-fixture`, at `tests/fixtures/queue_fixture.rs`, is a
   `[[bin]]`, as experiment 022 declares its scripted provider. The 028 README's Files table
   gains, before the `NNN-name/` row:

   ```text
   | [coordinator/README.md] (link to `coordinator/README.md`) | The coordinator's Rust tooling, the binary `wo` (DEC-174), and its merge queue |
   ```
2. **The configuration.** Each `wo queue` command reads `--config FILE`, a JSON object kept
   outside the tree (HOW-WE-WORK rule 2). It refuses an unknown key, and a missing one that is
   not optional. Paths are absolute, and nothing expands `~` or a variable.
   - `clone`, the main clone; `worktrees`, the trusted worktrees' directory, an entry being named
     by its worktree's directory name; `state`, the queue's own directory.
   - `messages`: merge messages, `NAME.txt`. One is well formed when its first line, the title,
     is not empty, its second is blank, a body follows with a line that is not blank, and it ends
     with a blank line and its trailer lines, at least one, each beginning with a `trailers`
     prefix.
   - `remote` and `base`, such as `origin` and `main`; `identity`, a `name` and an `email`.
   - `python`: the argv prefix that runs the repository's Python tools.
   - `gate`: `argv`, in which `{base}` becomes PINNED; `env`, names and values; an optional
     `cache`, an `address` (`host:port`) and an `env` added only when the address accepts a TCP
     connection within 2 s.
   - `gh`, the argv prefix of GitHub's CLI; `pr_footer`, text after each pull request's body;
     `trailers`, line prefixes.
   - `legacy_lock`, optional: the stopgap's lock directory.
3. **The queue file and the ledger**: JSON lines in `state`, each appended by one write. A reader
   skips a last line with no newline. Neither stores a state: it is computed, as DEC-177 computes
   counts.
   - An entry's order text, for a branch `wo/ID-...` whose ID, up to the first `-`, begins with
     a digit, is the one file `docs/work-orders/ID-*.md` at its head; another branch has none.
     The Record rule: that file exists and its last `## Record` section is not exactly
     `Not yet run.`. Texts written ahead are not checked.
   - `wo queue add NAME [--onto COMMIT] [--after NAME]...` appends to `queue.jsonl` the `name`,
     its worktree's `branch`, `onto`, `after` and `added`. It refuses, with exit 2 and why: a name
     already added; an unknown `--after`; an `--onto` that is not a full commit ID in the clone;
     an entry that fails `check`; and one that fails the Record rule.
   - `ledger.jsonl` gets a line as each step ends and an `end` line per attempt: `name`,
     `attempt` (one more per retry), `step`, `outcome` (`ok` or `failed`; at the end `merged`,
     `failed` or `skipped`), `started` and `ended` (UTC, RFC 3339), a one-line `why`, `pinned`,
     `head`, `backup`, and `gate_record`, the copied record's path once the attempt's gate has
     run, else null. A merged `end` line adds `pr` and `commit`.
   - `wo queue retry NAME` appends a `retry` line, opening the next attempt. It refuses a name
     whose current attempt has no `end` line or has merged.
   - An entry's state is its current attempt's `end` outcome. With none, it is `pending` while
     the attempt has no step line, else `running` while a run holds the lock and `stopped` when
     none does.
   - `wo queue status` prints a line per entry, in the file's order: name, branch, state, last
     step, pull request, commit, gate record, and what git shows now, unfetched: whether
     `REMOTE/BASE` holds a merged entry's commit, or another's branch head. It prints no file's
     contents.
4. **`wo queue run [--no-wait]`** takes an exclusive `std::fs::File::lock` on
   `state/queue.lock` until it exits. Where `legacy_lock` is set, it then makes that directory,
   as the kit's lock script does, with an `owner` file naming `wo queue` and its process ID, and
   removes it as it exits; one whose `owner` names `wo queue` was left by a run that died, and is
   removed. While either lock is held elsewhere, it waits and says so once, naming the lock's
   path and, for the kit's, its `owner` line; under `--no-wait` it exits 3. Before any entry, it
   ends `failed` each attempt with step lines and no `end` line, why "the run that held it
   stopped", and counts it failed. It runs pending entries in the file's order, one at a time,
   reading both files again after each, so an entry added or retried meanwhile runs in the same
   pass, and exits when none is pending. An entry runs only once each `--after` entry has merged;
   otherwise its `end` line is `skipped`, naming that entry.
5. **One entry, step by step**, to the first failure; the step's name is the ledger's `step`.
   - `check`: the message is well formed; it is read here, once, and later steps use that text.
     The worktree is a directory directly under `worktrees` whose `.git` is a file with a
     `gitdir:` line naming a directory under `clone`'s `.git/worktrees/`, read before any git
     runs there. It is clean, untracked files included, with no rebase, merge or cherry-pick in
     progress, on the entry's branch.
   - `fetch`: `git fetch REMOTE`. The commit `REMOTE/BASE` names is PINNED, used and never the
     ref until the entry ends.
   - `backup`: `refs/wo-backup/NAME/ATTEMPT` names HEAD; one already there fails the step and is
     never written over.
   - `rebase`: a merge commit in PINNED..HEAD (ONTO..HEAD) fails it: such a branch is never
     rebased. Then `git -c merge.conflictstyle=diff3 rebase PINNED`, or `--onto PINNED ONTO`;
     decision 6 settles each stop, then `git rebase --continue`. Where it cannot, run
     `git rebase --abort`; HEAD must equal the backup, and the step fails naming each path and
     why. A path is added only when no line of it begins `<<<<<<< `, `||||||| ` or `>>>>>>> `,
     and nothing else is added or committed while a rebase is stopped.
   - `standing`: `PYTHON tools/promise_standing.py --write`; a changed `engine/STANDING.md` is
     committed alone, "wip: STANDING regenerated on main".
   - `decisions`: `PYTHON tools/check_decisions.py PINNED WORKTREE` must end `Errors: 0`;
     otherwise fail with its `error:` lines, at most 20, each cut to 200 characters.
   - `identity`: each commit in PINNED..HEAD has the identity as author and committer.
   - `record`: the entry passes the Record rule at HEAD.
   - `squash`, with no fetch: PINNED is an ancestor of HEAD, and HEAD's tree differs from
     PINNED's, else "nothing to merge". The squash commit, with HEAD's tree, PINNED its only
     parent, the message `check` read and the identity, is made before the branch moves to it,
     as `git commit-tree` can; if any part fails, HEAD stays the rebased head. Print the count of
     changed paths and at most 60 names.
   - `gate`: the configured gate in the worktree, its output in `state/gate-NAME-ATTEMPT.txt`.
     The record its last line names after `Record: ` is copied into `state/gate-records/`,
     named `NAME-ATTEMPT-` and its own file name, never over a file. A nonzero exit or no
     record fails the step: print the output's lines that begin `FAIL`, cut to 200 characters,
     and its last line.
   - `push`: `git push --force-with-lease REMOTE HEAD:refs/heads/BRANCH`.
   - `pull-request`: the branch's open pull request, which `gh pr list --head BRANCH` finds, or
     else `gh pr create --base BASE --head BRANCH --title TITLE --body BODY`, numbered by the
     last path segment of the URL it prints. BODY is the message's body, less the blank line and
     trailer lines that end it, then `pr_footer`.
   - `main-unmoved`: `git fetch REMOTE`; `REMOTE/BASE` must still name PINNED. Otherwise fail
     with "main moved during the gate", and the pull request stays open.
   - `merge`: `gh pr merge N --squash --match-head-commit SHA`, SHA being HEAD's commit ID,
     `--subject "TITLE (#N)" --body` with the message after its blank line, trailers kept; then
     fetch. `REMOTE/BASE` must name a commit whose only parent is PINNED, with HEAD's tree and
     that subject; otherwise fail with "merged commit differs", and undo nothing.
   - `tidy`: remove each directory named `target` one to three levels under the worktree,
     never following a symbolic link or entering `.git`.

   A failed entry's worktree stays as its step left it. Every git command runs with
   `-c core.hooksPath=/dev/null -c commit.gpgsign=false -c rebase.updateRefs=false`. Every
   command the queue starts gets `PYTHONDONTWRITEBYTECODE=1`, `GIT_EDITOR=true`, so no step opens
   an editor, and the identity as `GIT_AUTHOR_*` and `GIT_COMMITTER_*`; the queue passes the rest
   of its environment on and never prints it.
6. **A stopped rebase.** Ours is the side rebased onto; theirs is the replayed commit (`--theirs`
   in a rebase). A conflicted path without all three stages in `git ls-files -u`, such as one
   side's change to a file the other deleted, stops, naming the path and the kind.
   `engine/STANDING.md` takes theirs; once every other path is settled,
   `PYTHON tools/promise_standing.py --write` regenerates it and it is added. Every other
   conflicted path stops, naming it and its first hunk's first line: `spec/`, order texts and
   `conformance/expected-failures.yaml` among them. No hunk keeps one side over a changed other.
7. **Output and exit status.** One line as each step starts, `NAME STEP`. One line per outcome:
   `MERGED NAME as COMMIT SUBJECT`, `FAILED NAME at STEP: WHY` or
   `SKIPPED NAME: AFTER did not merge`. Last, `queue done: merged [...] failed [...] skipped
   [...]`. Exit 0 when every entry run merged or none was pending, 1 when one failed or was
   skipped, 2 for a configuration or usage error or a refusal, 3 for a lock under `--no-wait`.
8. **Tests**, in `tests/queue.rs`. Each makes, in a temporary directory, a bare `origin`, a main
   clone and worktrees under a `worktrees` directory, with git's global and system configuration
   off and the identity set. `queue-fixture` plays `gh` on the bare repository: `pr list`;
   `pr create`, printing a URL that ends in a number; `pr merge`, which checks
   `--match-head-commit` against the branch's head on `origin` and writes the squash onto its
   `main` with `git commit-tree` and `git update-ref`. It plays the gate, `pass`, `fail` and
   `move-main` (which pushes to `origin`'s `main`, then passes), each writing
   `target/gate/HEAD.json` and ending as the gate's last line does. It plays the Python tools:
   `promise_standing.py --write` writes `engine/STANDING.md` from the `spec/` paths and their
   line counts, and removes the message where the tree holds `drop-message`;
   `check_decisions.py` ends `Errors: 1` where the tree holds `decisions-fail`, else
   `Errors: 0`.
   - `queue_merges_in_order`: two entries; `main` moves after both branch. Each merges in order
     as one commit on `origin`'s `main`, with its subject, its message's body and its branch's
     rebased tree. The ledger has a line per step and two `merged` lines; each line from the gate
     on names a record that exists.
   - `queue_onto_replays_own_commits`: an entry stacked on another branch, added with `--onto`
     that branch's head, lands only its own commits' changes.
   - `queue_skips_dependents`: A's gate fails; B, `--after` A, is skipped, naming A; C merges;
     exit 1.
   - `queue_add_refuses`: a missing message; an empty title, a second line not blank, no body or
     no trailer line; a `wo/ID-...` branch with no order text, or one whose Record is
     `Not yet run.`; a duplicate, an unknown `--after` and a short `--onto`, each leaving
     `queue.jsonl` as it was. A branch of another name that adds a text written ahead is added.
     After `add`, a removed message fails at `check`, HEAD unchanged, and a reverted Record at
     `record`.
   - `queue_resolves_standing`: `main` and the branch both change `engine/STANDING.md`. It merges
     with the fixture's STANDING for the merged tree.
   - `queue_other_conflict_stops`: a conflict in `conformance/expected-failures.yaml`, then in a
     Rust source, then a spec file `main` deleted and the branch changed, fails at `rebase`,
     naming the path, and the kind for the last; no rebase is in progress, HEAD equals the
     backup, the tree is clean.
   - `queue_squash_keeps_head`: an entry whose change `main` already holds fails at `squash`,
     "nothing to merge", HEAD the rebased head and nothing staged. One whose tree holds
     `drop-message` merges with the message `check` read.
   - `queue_decisions_check_stops`, `queue_refuses_foreign_identity` and
     `queue_refuses_when_main_moved`: `decisions-fail`, another author's commit and `move-main`
     fail at `decisions`, `identity` and `main-unmoved`, and nothing merges.
   - `queue_refuses_a_clone`: an entry directory that is a clone of its own fails at `check`,
     its configuration and refs unchanged.
   - `queue_single_runner`: while a run holds the lock, and while `legacy_lock` exists with
     another's `owner`, `run --no-wait` exits 3 and no entry starts. While a run holds the lock,
     a `mkdir` of `legacy_lock` fails and its `owner` names the run; it is gone once the run
     exits. One whose `owner` names `wo queue`, with no run, is removed.
   - `queue_status_and_retry`: `status` gives each state, never the message's body. After a
     failed gate, `retry` and a passing gate merge the entry as attempt 2, and `status` shows it
     merged and on `main`. An attempt with step lines and no `end` line shows `stopped`; the next
     run ends it `failed`, its backup ref unchanged, and `retry` merges it as the next attempt.
9. **README.** The crate's `README.md` has a `` ## `wo queue` `` section: the commands, the
   steps and what each checks, decision 6's rules, and the configuration's keys with an example
   whose values are `<...>` placeholders only. It says `wo queue` replaces the coordinator's
   private batch, lock and queue scripts.

## Start check

From the repository root:

```sh
c=experiments/028-a78-dogfood/coordinator
p=experiments/033-usage-guard-probe/probe
git status --short | wc -l | tr -d ' '  # 0
test -e $c; echo "crate $?"  # crate 1
sed -n '/^## DEC-174, /,/^## DEC-175, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
grep -cE '^(serde_json = "=1\.0\.154"|\[\[bin\]\])$' $p/Cargo.toml  # 4
grep -c "parser.add_argument('--base'" tools/gate.py  # 1
grep -c 'Record: {relative_record}' tools/gate.py  # 2
grep -c '^| `NNN-name/`, from order 2 on |' experiments/028-a78-dogfood/README.md  # 1
```

Where a line prints otherwise, stop, change nothing and report it.

## Scope

Change only `experiments/028-a78-dogfood/coordinator/`, which this order makes (`Cargo.toml`,
`Cargo.lock`, `rust-toolchain.toml`, `src/`, `tests/` and `README.md`), and one row of the Files
table in `experiments/028-a78-dogfood/README.md`. Change nothing else: not `tools/`, `spec/`,
`conformance/`, `engine/`, `packages/`, `docs/`, `.github/` or another experiment. Write outside
the tree only in temporary directories your tests make. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after. The second is guards, which pass today
and after; its cargo lines run once the crate exists.

```sh
c=experiments/028-a78-dogfood/coordinator
ok='^test result: ok\. 1 passed'
for t in queue_merges_in_order queue_onto_replays_own_commits queue_skips_dependents queue_add_refuses queue_resolves_standing queue_other_conflict_stops queue_squash_keeps_head queue_decisions_check_stops queue_refuses_when_main_moved queue_refuses_foreign_identity queue_refuses_a_clone queue_single_runner queue_status_and_retry; do (cd $c && cargo test --offline --locked --test queue "$t" -- --exact 2>&1 | grep -c "$ok"); done  # 1 thirteen times; nothing today (no crate)
cat $c/README.md 2>/dev/null | grep -c '^## `wo queue`$'  # 1; 0 today
grep -cF '| [coordinator/README.md](coordinator/README.md) | ' experiments/028-a78-dogfood/README.md  # 1; 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
c=experiments/028-a78-dogfood/coordinator
(cd $c && cargo fmt --check); echo "fmt $?"  # fmt 0
(cd $c && cargo clippy --offline --locked --all-targets -- -D warnings 2>&1 | grep -c '^error')  # 0
(cd $c && cargo test --offline --locked 2>&1 | grep -E '^test result:' | grep -vc '^test result: ok\.')  # 0
git add -N $c
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, twice
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^experiments/028-a78-dogfood/(coordinator/.+|README\.md)$' | wc -l | tr -d ' '  # 0
git reset -q -- $c  # stage nothing
```

Your report gives, per decision, what changed and where; each test's passing line; the dependency
and its lockfile entries; each of your own choices; and every check's last line. Stage nothing.

## What follows

- The coordinator writes the configuration outside the tree, the gate's argv as the kit passes
  it with `--base {base}`, then queues each merge with `wo queue add` and one `wo queue run` in
  the background. The kit's batch, lock and queue scripts retire after its last batch; until
  then the queue and the kit share the kit's lock.
- A conflict the queue cannot settle fails its entry; the coordinator settles it in the worktree
  and runs `wo queue retry`. Order 401b adds the word merge, the changelog's keep-both merge and
  the fragment step. The kit's one-offs, a rebase continuer and a script that drops one expected
  failure, get no replacement. The case-id merge ends with the expected failures' own fragments
  (DEC-190, logged after this base).
- `wo status` is to compute the coordinator's state from this ledger (DEC-183, logged after this
  base).
- DEC-174's status is the coordinator's once this order merges.

## Record

Run on 2026-11-24, in Pacific time, on order 400's tip (changelog fragments, queued to merge).

**Built outside the Fleet.** This order changes experiment 028's directory, which no order sent
through the Fleet may change (DEC-057), so the coordinator ran it in a Workflow of its own: one
Claude worker in a worktree of its own, two blind Claude reads, one fix pass. No Agent of the
dogfood took a turn on it.

**The worker** built the crate, its 13 tests and its README, and every Start check and Done-when
line printed what the order expects.

**Two blind reads.** One read it as a merge queue and asked for two musts and two shoulds: a retry
of an `--onto` entry replayed commits from `main`, not only the entry's own; the queue inherited
the caller's Git configuration, `rerere` included; the identity step missed committers rewritten
by the rebase; and a torn last ledger line poisoned every later read. The other read it against
the order and passed it, with a should on the README's account of the daily use. One fix pass
answered all five, each with a test that failed first (18 integration tests now). The gate's run
on the squash is the coordinator's, at the merge.

**At the merge.** The gate's tool tests refused the tree: its table of what each Rust crate reads
(`RUST_READS` in `tools/gate.py`) had no row for the new crate, whose queue fixture builds a
`spec/` of its own and so matches the table's test for a crate that reads `spec/`. The
coordinator added the row (ruling 87), so the crate's tests also run when `spec/` changes.
