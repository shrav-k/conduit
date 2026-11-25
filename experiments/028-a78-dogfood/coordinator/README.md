# The coordinator's Rust tooling

The coordinator's tooling lives here, inside experiment 028
([DEC-174](../../../docs/decisions/log.md#dec-174-2026-11-23-the-coordinators-rust-tooling-lives-in-experiment-028)),
and is Rust (DEC-132). Work order 401 made it. The crate is `a78-coordinator`, edition 2024 and
unpublished, with its own workspace, lockfile and the toolchain that experiment 033's probe pins.
Its one dependency is `serde_json = "=1.0.154"`, locked as that probe locks it. The binary `wo`
is a thin front over the library. The `queue-fixture` binary, `tests/fixtures/queue_fixture.rs`,
is for the tests alone: it plays GitHub's CLI on a bare repository, the gate and the repository's
Python tools.

```sh
cargo test --offline --locked
```

The tests make every repository they use in a temporary directory, with git's global and system
configuration off. They run no `gh`, no gate and no Python.

## `wo queue`

`wo queue` replaces the coordinator's private batch, lock and queue scripts: the kit's stopgap
outside the tree. It merges one work order's branch at a time. Each branch is rebased on the
commit `REMOTE/BASE` names when its entry starts, called PINNED below. Then the queue squashes
it, gates it, pushes it and merges its pull request, and it checks what landed. A merge is
squashed, waits on the local gate and not on CI, and carries the work-order identity (DEC-030,
DEC-089). The queue merges only a tree the gate ran on and only commits by the identity it
checks. A merge that lands on a `main` moved since its last fetch is reported by name and never
undone.

### How the coordinator uses it

The coordinator writes the configuration outside the tree, with the gate's argv as the kit passes
it, `--base {base}` included. It queues each merge with `wo queue add NAME`, adding `--after` for
an order that needs another merged first and `--onto` for a branch stacked on another, then
starts one `wo queue run` in the background; `wo queue status` shows where each entry stands. A
conflict the queue cannot settle fails its entry at `rebase`, with HEAD back at the backup: the
coordinator settles it in that worktree and runs `wo queue retry NAME`, which the running pass, or
the next run, takes up. A failed gate is retried the same way once its cause is fixed. While the
kit's batch, lock and queue scripts still run, `legacy_lock` names the kit's lock directory, so
that the queue and the kit share the kit's lock; the scripts retire after the kit's last batch.

### Commands

Each command reads `--config FILE`.

- `wo queue add NAME [--onto COMMIT] [--after NAME]...` appends an entry to `queue.jsonl`. The
  entry holds the name, its worktree's branch, `onto`, `after` and the time it was added. It
  refuses, with exit 2 and the reason, in six cases: a name already added, an unknown
  `--after`, an `--onto` that is not a full commit ID in the clone, an `--onto` that is not an
  ancestor of the branch's head, an entry that fails `check`, and one that fails the Record rule.
- `wo queue run [--no-wait]` runs every pending entry; see the run below.
- `wo queue status` prints one line per entry, in the file's order:
  `NAME branch=B state=S attempt=N step=LAST pr=N commit=C gate_record=PATH`, with `-` for a value
  not yet known. The line ends with what git shows now, without a fetch. For a merged entry that
  is `REMOTE/BASE_holds_commit=yes|no`, for its commit. For any other entry it is
  `REMOTE/BASE_holds_head=yes|no`, for its branch's head in the clone. Status prints no file's
  contents.
- `wo queue retry NAME` appends a `retry` line, which opens the entry's next attempt. It refuses a
  name whose current attempt has no `end` line, and one that has merged.

### The files

`state` holds `queue.jsonl` and `ledger.jsonl`. Each is JSON lines, each line appended by one
write, and a reader skips a last line with no newline. A writer holds the file's lock (`flock(2)`)
while it appends, and first cuts such a torn last line, which no reader saw, saying so on
standard error; the line it appends then starts a line of its own. Neither stores a state.

- A step line or `end` line of the ledger has these fields: `name`, `attempt` (one more per
  retry), `step`, `outcome` (`ok` or `failed`; at the end `merged`, `failed` or `skipped`),
  `started` and `ended` (UTC, RFC 3339) and a one-line `why`. It also holds `pinned`, `head`,
  `backup` (the backup ref's name) and `gate_record`. `gate_record` is the copied record's path
  once the attempt's gate has run, and null before. A merged `end` line adds `pr` and `commit`.
- An entry's state is its current attempt's `end` outcome. An attempt with no `end` line is
  `pending` while it has no step line. With step lines, it is `running` while a run holds the
  lock, and `stopped` when none does.
- `state` also holds `queue.lock`, each gate's output as `gate-NAME-ATTEMPT.txt` and the copied
  records under `gate-records/`.

The order text is found by the branch's name. For a branch `wo/ID-...` whose ID, up to the first
`-`, begins with a digit, the order text is the one file `docs/work-orders/ID-*.md` at its head.
Another branch has none. The Record rule: that file exists, and its last `## Record` section is
not exactly `Not yet run.`. A text without a `## Record` section fails the rule. Texts written
ahead are not checked.

### The run

`wo queue run` takes an exclusive lock on `state/queue.lock` (`std::fs::File::lock`, which is
`flock(2)`) and holds it until it exits. Where `legacy_lock` is set, it then makes that directory
as the kit's lock script does. The directory gets an `owner` file,
`wo queue pid PID lock QUEUE_LOCK`, naming the queue's lock the run holds, and the run removes it
as it exits. A lock directory whose `owner` begins `wo queue` was left by a run that died, and is
removed, when the queue's lock it names is this run's own or can be taken, or when it names none.
While a run under another `state` holds the lock it names, the directory is held, as the kit's is.
While either lock is held elsewhere, the run waits and says so once. It names the lock's path and,
for the kit's lock, its `owner` line. Under `--no-wait` it exits 3 instead. The run tries the
queue's lock for 100 ms before it counts it held, since `status` and `retry` take it for an
instant to see whether a run holds it.

Before any entry, the run ends as `failed` each attempt that has step lines and no `end` line,
with the reason "the run that held it stopped", and counts it as failed. Then it runs the pending
entries in the file's order, one at a time. It reads both files again after each entry, so an
entry added or retried meanwhile runs in the same pass. It exits when none is pending. An entry
runs only once each of its `--after` entries has merged; otherwise its `end` line is `skipped`,
naming that entry.

### The steps

An entry runs step by step, to its first failure, and the ledger's `step` is the step's name. A
failed entry's worktree stays as its step left it.

| Step | What it does and checks |
| --- | --- |
| `check` | The message `NAME.txt` is well formed. It is read here, once, and later steps use that text. The worktree is a directory directly under `worktrees`. Its `.git` is a file whose `gitdir:` line names a directory under the clone's `.git/worktrees/`, and this is read before any git runs there. The worktree is clean, untracked files included, has no rebase, merge, cherry-pick or revert in progress, and is on the entry's branch. |
| `fetch` | `git fetch REMOTE`. The commit `REMOTE/BASE` names is PINNED, and the steps use PINNED, never the ref, until the entry ends. |
| `backup` | `refs/wo-backup/NAME/ATTEMPT` names HEAD. If that ref exists already, the step fails, and the ref is never written over. |
| `rebase` | For an `--onto` entry, ONTO here is the newest of the entry's ONTO, the PINNED of each of its earlier attempts whose rebase passed, and PINNED, that is an ancestor of HEAD: once an attempt has rebased the branch, the entry's ONTO is no longer one, and a retry replays from the base that attempt left. With none, the step fails, since the entry's own commits cannot be told from others. A merge commit in PINNED..HEAD (ONTO..HEAD) fails the step: such a branch is never rebased. Then `git -c merge.conflictstyle=diff3 rebase PINNED`, or `--onto PINNED ONTO`. Each stop is settled as the next section says, then `git rebase --continue`. A stop it cannot settle runs `git rebase --abort`. HEAD must then equal the backup, and the step fails naming each path and why. |
| `standing` | `PYTHON tools/promise_standing.py --write`. A changed `engine/STANDING.md` is committed alone, as "wip: STANDING regenerated on main". A change to any other path fails the step. |
| `decisions` | `PYTHON tools/check_decisions.py PINNED WORKTREE` must exit 0 and end `Errors: 0`. Otherwise the step fails with the tool's `error:` lines, at most 20, each cut to 200 characters. |
| `identity` | Each commit in PINNED..HEAD has the configured identity as author and committer. So does each of the branch's own commits as each of the entry's backups holds it, in BASE..BACKUP, BASE being what the rebase replays from: the rebase runs with the identity as committer, so it rewrites a foreign committer away, on this attempt or an earlier one. |
| `record` | The entry passes the Record rule at HEAD. |
| `squash` | No fetch. PINNED is an ancestor of HEAD, and HEAD's tree differs from PINNED's; otherwise the step fails with "nothing to merge". `git commit-tree` makes the squash commit, with HEAD's tree, PINNED as its only parent, the message `check` read and the identity. Only then does the branch move to it. If any part fails, HEAD stays the rebased head. The step prints the count of changed paths and at most 60 of their names. |
| `gate` | The configured gate runs in the worktree, and its output goes to `state/gate-NAME-ATTEMPT.txt`. The record that its last line names after `Record: ` is copied into `state/gate-records/` as `NAME-ATTEMPT-` followed by its own file name, never over an existing file. The step fails on a nonzero exit, on no record, or if HEAD moved during the gate. It then prints the output's lines that begin `FAIL`, cut to 200 characters, and its last line. |
| `push` | `git push --force-with-lease REMOTE HEAD:refs/heads/BRANCH`. |
| `pull-request` | The branch's open pull request, which `gh pr list --head BRANCH --state open --json number` finds. If there is none, `gh pr create --base BASE --head BRANCH --title TITLE --body BODY` opens one, numbered by the last path segment of the URL it prints. BODY is the message's body, without the blank line and the trailer lines that end it, followed by `pr_footer`. |
| `main-unmoved` | `git fetch REMOTE`, and `REMOTE/BASE` must still name PINNED. Otherwise the step fails with "main moved during the gate", and the pull request stays open. |
| `merge` | `gh pr merge N --squash --match-head-commit HEAD --subject "TITLE (#N)" --body REST`, where REST is the message after its blank line, trailers kept, then a fetch. `REMOTE/BASE` must name a commit whose only parent is PINNED, with HEAD's tree and that subject. Otherwise the step fails with "merged commit differs", and nothing is undone. |
| `tidy` | Removes each directory named `target` one to three levels under the worktree, never following a symbolic link or entering `.git`. A tidy that fails is recorded as failed, but the entry still ends `merged`, since its merge landed. |

Every git command runs with `-c core.hooksPath=/dev/null -c commit.gpgsign=false -c
rebase.updateRefs=false -c rerere.enabled=false -c rerere.autoUpdate=false`. Rerere is off
whatever the caller's configuration says, since a recorded resolution replays a person's pick of
a side. Every command the queue starts gets `PYTHONDONTWRITEBYTECODE=1`, and
`GIT_EDITOR=true`, so that no step opens an editor. It also gets the identity as `GIT_AUTHOR_*`
and `GIT_COMMITTER_*`. The rest of the queue's environment passes on, and the queue never prints
it.

### A stopped rebase

Ours is the side rebased onto; theirs is the replayed commit (`--theirs` in a rebase).

- A conflicted path without all three stages in `git ls-files -u` stops the rebase, naming the
  path and the kind. One example is one side's change to a file the other side deleted, which
  shows as "deleted by ours, changed by theirs".
- `engine/STANDING.md` takes theirs. Once every other path is settled,
  `PYTHON tools/promise_standing.py --write` regenerates it, and it is added.
- Every other conflicted path stops the rebase, naming the path and its first hunk's line and
  first line. That includes `spec/`, order texts and `conformance/expected-failures.yaml`.
- A path is added only when no line of it begins `<<<<<<< `, `||||||| ` or `>>>>>>> `. Nothing
  else is added or committed while a rebase is stopped, and no hunk keeps one side over a changed
  other.

Order 401b adds the word merge, the changelog's keep-both merge and the fragment step. The kit's
pick of the replayed order text, and its merge of the expected failures by case ID, are not
carried over.

### Output and exit status

The run prints one line as each step starts, `NAME STEP`, and one line per outcome:
`MERGED NAME as COMMIT SUBJECT`, `FAILED NAME at STEP: WHY` (with indented lines naming each
thing) or `SKIPPED NAME: AFTER did not merge`. Its last line is
`queue done: merged [...] failed [...] skipped [...]`.

| Exit | When |
| --- | --- |
| 0 | Every entry run merged, or none was pending |
| 1 | An entry failed or was skipped |
| 2 | A configuration or usage error, or a refusal |
| 3 | A lock was held elsewhere under `--no-wait` |

### The configuration

The configuration is a JSON object kept outside the tree (HOW-WE-WORK rule 2). An unknown key is
refused, at any level, and so is a missing key that is not optional. Paths are absolute, and
nothing expands `~` or a variable.

| Key | Holds |
| --- | --- |
| `clone` | The main clone |
| `worktrees` | The trusted worktrees' directory; an entry is named by its worktree's directory name |
| `state` | The queue's own directory, made when missing |
| `messages` | The merge messages, `NAME.txt` |
| `remote`, `base` | Such as `origin` and `main` |
| `identity` | `name` and `email` |
| `python` | The argv prefix that runs the repository's Python tools |
| `gate` | `argv`, in which `{base}` becomes PINNED; `env`, names and values; an optional `cache`: an `address` (`host:port`) and an `env` added only when the address accepts a TCP connection within 2 s |
| `gh` | The argv prefix of GitHub's CLI |
| `pr_footer` | Text after each pull request's body |
| `trailers` | The trailer lines' prefixes, at least one |
| `legacy_lock` | Optional: the stopgap's lock directory |

A message is well formed when it meets all of these:

- its first line, the title, is not empty;
- its second line is blank;
- a body follows, with at least one line that is not blank;
- it ends with a blank line and its trailer lines, at least one, each beginning with a `trailers`
  prefix.

An example, every value a placeholder:

```json
{
  "clone": "<absolute path of the main clone>",
  "worktrees": "<absolute path of the trusted worktrees' directory>",
  "state": "<absolute path of the queue's own directory>",
  "messages": "<absolute path of the merge messages' directory>",
  "remote": "<remote's name>",
  "base": "<base branch's name>",
  "identity": {"name": "<identity's name>", "email": "<identity's email>"},
  "python": ["<argv prefix that runs the repository's Python tools>"],
  "gate": {
    "argv": ["<the gate's argv, with {base} where the pinned commit goes>"],
    "env": {"<name>": "<value>"},
    "cache": {"address": "<host:port>", "env": {"<name>": "<value>"}}
  },
  "gh": ["<argv prefix of GitHub's CLI>"],
  "pr_footer": "<text after each pull request's body>",
  "trailers": ["<a trailer line's prefix>"],
  "legacy_lock": "<absolute path of the kit's lock directory>"
}
```
