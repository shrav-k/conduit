# Work order 404: `move.v1` moves a working copy in place to another commit, drops its change, keeps its ignored files and may leave a patch applied (DEC-171, DEC-180)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network, and every crate it builds is
cached: run cargo offline, and if a crate is missing, stop and report, and fetch nothing. Run one
cargo command at a time, and not `tools/gate.py` or the whole engine suite. Never print a whole
large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Run no `conduit` outside a test, no
`codex` or `claude`, and sign in nowhere: every implementation a test meets is scripted. Open,
list or sweep no store but one a test makes in a temporary directory: never the dogfood's. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute
local path, user, machine or account name, or private repository name.

Besides the checks below, run Git only on repositories a test makes under its temporary
directory, under order 229's rule for Git in tests and order 403's for the Agent's Git in a view.
Never run `git commit`, `git push` or a fetch outside a repository a test makes, nor
`git config --global`.

## What and why

**The owner's words.** [DEC-171](../decisions/log.md#dec-171-2026-11-23-the-coordinator-hands-off-through-written-state-its-kits-rust-replacements-follow-the-guard):
the kit -> "Order Rust replacements first"; when they run -> "After the guard chain
(Recommended)".
[DEC-180](../decisions/log.md#dec-180-2026-11-24-parallel-tracks-on-the-accounts): how the new
seats work -> "Parallel tracks (Recommended)"; and "We should also likely rewrite everything we
can into the engine now using the new accounts we have access to".
[DEC-021](../decisions/log.md#dec-021-2026-11-20-how-packages-are-organized): GitHub unit ->
"github + git (study's rec)": the working copy is the `git` package's. This order replaces the
kit's second slot step: moving a pair's copy to the next base.

**Today.** Between orders the coordinator rebuilds each pair's slot by hand: a fresh clone at the
next base, the order text uncommitted, and no `target/`. `working-copy.v1`
([order 229](229-git-working-copy.md), with order 403's view) serves the commit its configuration
names; another takes retirement and a new setup, which drops `target/` too. The reader's view, the
base with the worker's patch uncommitted, is built by hand too.

**A turn in progress.** The package cannot see an Agent's turn, an invocation of another instance:
Conduit's keys (IFC-006) name no consumer's invocation. So the move changes nothing unless the
copy is as its caller last read it (decision 3), which refuses the move after a turn whose writes
`changes.v1` shows. Order 408's `wo slot` checks its ledger for a turn of the pair before it reads.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2, only a
Conduit listing `move.v1` reaches it, and `read` and `read-write` keep their lists (PKG-031); 3,
the copy stays its Resource's; 6, a refused move changes nothing; 7, the move's record keeps its
inputs (`records`).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes, by
existing practice. GitLab's runners reuse a job's working copy and clean it by
[Git clean flags](https://docs.gitlab.com/ci/runners/configure_runners/#git-clean-flags), and
[`git clean`](https://git-scm.com/docs/git-clean) without `-x` keeps ignored files. The
expectations are [`git push --force-with-lease`](https://git-scm.com/docs/git-push)'s and HTTP's
[`If-Match`](https://www.rfc-editor.org/rfc/rfc9110#name-if-match): a change applies only to the
state its caller saw. Not taken: a control setting `/commit` (CTR-006), since "control dispatch
remain unfinished" ([CONFORMANCE](../../engine/CONFORMANCE.md)) and a control applies no patch;
a new setup (no `target/`); Git with root as its work tree (the package never gives Git a path
into the served copy, order 229); a new directory renamed over root (not the one the record names).

**When it runs.** On `main` once order 403 merges, or on 403's commit stacked on `main` at
`6d3a1f14`. Through Conduit: one Codex worker turn, the blind Codex verifier and the
Claude reader at once (README text, DEC-189), one fix turn, the gate (DEC-050, DEC-033). By
ruling 89, each decision says what must hold; how is yours. If one turn cannot finish it, 404b
takes decision 4 (`patch`, `move_applies_a_patch_uncommitted` and the README's patch sentence)
and 404a the rest, in one merge, so 0.3.0 has one archive (PKG-025). The coordinator's choices,
open to the owner's strike
([DEC-029](../decisions/log.md#dec-029-2026-11-20-who-decides-and-who-does)): the names, inputs
and texts; expectations in place of a turn check; what stays; the record of a move; the unstaged
patch; root in place; 0.3.0; the README's words.

**Shared files.** Order 403's, which it follows; and `engine/crates/shell/tests/working_copy.rs`
with order 416, in progress, which rewrites `Cli::serve`'s wait: whichever merges later rebases
and keeps both. The move step adds no fixed wait.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), a file outside Scope that must change, or
an entry that can stay in root only by being copied (give the case). Where this order's wording
contradicts a promise it does not change, the promise governs: apply it and list it under
`own_choices`, with every choice no decision governs.

## The decisions

Read first, in `packages/git`: `src/`, `tests/schema_drift.rs`, `Fixture` and order 403's tests,
the README; then PKG-025, PKG-031 and order 403's decisions. Wrap each insertion at its file's
width.

1. **The operation and its template.** `contracts.yaml` gains the Operation `move.v1` after
   `changes.v1`, and `copy.v1`'s list, the Type's `implementation` map and the server's
   `implements` each gain it last. A Conduit template `manage` follows `read-write`, with
   `read-write`'s four operations, in its order, then `#move.v1`; `read` and `read-write` keep
   their lists (PKG-031). `exports` follows `contracts.yaml`. Description: "Move the copy in place
   to another commit: drop its change, keep its ignored files and apply an optional patch
   uncommitted." Inputs, with
   `additionalProperties: false`: `commit` and `expectedBase`, each a string of pattern
   `^[0-9a-f]{40}$`, and `expectedPatchSha256`, of `^[0-9a-f]{64}$`, all required; `patch`, a
   string. Outputs: `base` and `files`, as `changes.v1` declares them. `effect`: "Replaces the
   copy's files with the commit's and keeps its ignored files." `lib.rs` serves the same
   schemas. No `changes` (no control), no new dependency.
2. **What the move leaves.** It imports the repository's objects as setup does, copying those it
   lacks, and checks `commit` as setup does; `commit` may be the base. Then:
   - Root is the same directory, by the record's device and inode, with its mode.
   - Each entry that `changes.v1` leaves out as ignored, and that the base does not track or
     hold a tracked path under, stays, a directory whole, with its bytes, modes and inodes, never
     copied; unless the new tree (the commit's, the patch applied) holds a path at it, under it or
     at one of its ancestors, or that tree's `.gitignore` files do not ignore it. A directory
     holding one stays too.
   - All else goes, the base's files and the change, and the new tree takes its place. No link
     is followed; nothing outside root, the move's staging and the instance's state directory
     changes.
   - Root's `.git` is replaced whole: removed as retirement removes root, reading no file in it,
     then written as setup writes the view, at `commit` (amending order 403's decision 2). No
     Git the package runs has root as its work tree or the view as its directory.
3. **The expectations.** The move fails, changing nothing, at the first of these: the record's
   base is not `expectedBase`, with "move refused: the copy is not at expectedBase; nothing was
   changed"; taking the copy's change as `changes.v1` does fails, as that fails; the SHA-256 of
   that patch's UTF-8 bytes is not `expectedPatchSha256`, with "move refused: the copy's change is
   not the one expectedPatchSha256 names; read changes.v1 again; nothing was changed". A commit
   missing from the repository, or holding a Git link, fails with setup's text, changing nothing.
   The instance's lock is held throughout.
4. **The patch.** A `patch`, at most 8 MiB, is applied to the commit's files in the private
   checkout, as `git apply --index` applies `changes.v1`'s patch to a clean checkout, before root
   changes. One that does not apply, or names a `.git` segment in any case, fails with "move
   refused: the patch does not apply to commit; nothing was changed". It stays unstaged: the
   view's index is the commit's, so the Agent's `git status` lists its paths, `??` for each added
   file.
5. **What it returns and records.**
   - It returns `base`, the commit, and `files`, as `changes.v1` then gives them.
   - The engine's record keeps its inputs, the patch included, not its outputs (`records`).
   - The package's record names the new base once root and the view are at it, and keeps
     setup's commit: a setup repeated at the same root and that commit succeeds and changes
     nothing, after a move too; one at another commit fails as today. A record without setup's
     commit, as 0.1.0 and 0.2.0 wrote it, takes its base as setup's commit.
   - Once decisions 3 and 4 pass, before root changes, the record names the move: its inputs,
     the patch by its SHA-256, and each entry it keeps, by path, device and inode. Only a move
     that finishes drops it, and no move, even one that ends part way by a failure or a kill,
     takes a kept entry out of root. While it stands, `changes.v1` and other moves fail with "a
     move to COMMIT ended part way; repeat it with the same inputs; nothing was changed", COMMIT
     its SHA; the same move takes no change, keeps each recorded entry that still has its device
     and inode, and finishes. A move removes its staging when it ends; a killed move's is removed
     by the next move or retirement, never anything the instance does not own.
6. **0.3.0.** `packages/git/Cargo.toml`'s version and `Cargo.lock`'s own line, `conduit.yaml`'s
   `version`, the README's first sentence and the engine test's pin (PKG-025).
7. **The README.**
   - Its first sentence names `0.3.0`. After "The `read-write` template adds `file.write.v1`."
     add: "The `manage` template adds `move.v1`, below."
   - Before `## The Git view`, this section:

     ```text
     ## Moving the copy

     `move.v1` moves the copy in place to another commit of its repository. Give it the `base` that `changes.v1` last returned and the SHA-256, in lowercase hexadecimal, of that call's `patch`: unless the copy is still as that read gave it, the move fails and changes nothing, so it drops no change made before it that its caller has not read. It drops the copy's change and keeps each untracked entry that the ignore rules before the move and after it (the commit's, with any patch applied) both ignore and the moved copy does not track, such as `target/`; any other ignored entry goes too. An optional `patch`, in `changes.v1`'s form, is applied unstaged. Root stays the same directory, with a new Git view at the commit. The record keeps the move's inputs, the patch among them.

     The package cannot see an Agent's turn: move a copy only when no Agent works in it. A move that ends part way is finished by the same move; until then `changes.v1` and other moves are refused. The Resource's `commit` stays setup's.
     ```
   - In the row `D15(c) Persistent state`, "Setup writes the Git view at root's `.git` once;"
     becomes "Setup writes the Git view at root's `.git`, and each move writes it again;". In
     `D15(d)`, "checks both Conduits, applies a patch and retires" becomes "checks its three
     Conduits, applies a patch, moves the copy and retires".
8. **Tests**, in `packages/git/tests/mcp.rs`, written first; run each on the otherwise unchanged
   package and report its line. "Read" is a `changes.v1` call, whose `base` and patch's SHA-256
   are the expectations. "Unchanged": every file under root, `.git` included, keeps its name,
   bytes and mode, and `changes.v1` gives what it gave. The Agent's Git is order 403's.
   - `move_drops_the_change_and_keeps_ignored_files` (decisions 2, 3, 5): set up at the base; a
     new commit changes `tracked.txt`, removes `delete.txt`, adds `next.txt`, and un-ignores and
     adds `lib/ignored/file`. In root: `tracked.txt` edited, `new.txt` written, `.gitignore` given
     `*.tmp` and `scratch.tmp` written, `opaque.txt` made a link to a file outside root,
     `ignored/cache/tool` written with mode 0755, `lib/ignored/stale` written; the Agent's Git
     makes a branch. Read, note root's and `ignored/`'s inodes, and move to the new commit. Root,
     with its inode, holds `.git`, the commit's files and `ignored/cache/tool`, with its bytes,
     mode and `ignored/`'s inode; `lib/ignored/` holds only `file`; the outside file keeps its
     bytes. The Agent's Git gives the commit for `rev-parse HEAD`, and nothing for
     `status --porcelain=v1 -uall` or `for-each-ref`. The reply and `changes.v1` give `base` the
     commit and no `files`. No `.conduit-` entry stays beside root.
   - `move_applies_a_patch_uncommitted` (decision 4): two copies at the base, each with its own
     state and root. The first gets `tracked.txt` edited, `delete.txt` removed and `new.txt` and
     `ignored/extra` written, and gives a patch. The second, read, moves to the base with it: its
     files, `.git` aside, equal the first's but for `ignored/`, its `changes.v1` gives the first's
     `files`, and the Agent's Git gives nothing for `diff --cached --name-only` and `?? new.txt`,
     ` D delete.txt` and ` M tracked.txt` for `status --porcelain=v1 -uall`. A commit then edits
     `tracked.txt`; a move there, read again, with that patch fails with "the patch does not
     apply", and the second copy is unchanged.
   - `move_is_refused_unless_the_copy_is_as_read` (decision 3): set up, read, then write
     `tracked.txt`, as a turn would. Moves fail with "read changes.v1 again" (the read's
     expectations), "not at expectedBase" (another base) and, read again, "commit is not in the
     repository" (a missing commit), root unchanged each time; a read and a move then succeed.
   - `move_keeps_its_commit_across_setup_and_retirement` (decision 5): set up, read and move to a
     second commit. Setup repeated at the base succeeds and leaves root, the view's `HEAD`
     included, at the second; setup at the second fails with "another root or commit";
     retirement removes root and the private Git directory, leaving nothing beside root.
   - `move_that_ends_part_way_is_finished` (decision 5): set up; in root write
     `ignored/cache/tool` and `locked/file`, then make `locked/` 0500. Read and move to a new
     commit: it fails. `changes.v1`, and a move to the base with the read's expectations, fail
     with "ended part way". Restore `locked/`'s mode: the same move succeeds, `ignored/` keeps its
     inode and `ignored/cache/tool` its bytes, `locked/` is gone, and `changes.v1` gives the
     commit and no `files`. No fault variable is added.
   - Changed: `contracts_match_the_capability_and_advertised_tools`, for decision 1; and
     `working_copy_through_the_engine`, which pins `0.3.0`, adds a Conduit `copy-manage` from
     `#manage` and, before retirement, commits again in its source and moves the copy there by
     `copy-manage.move.v1` with its `changes.v1` result's expectations: it succeeds with that
     `base`, `hello.txt` and `.git/HEAD` follow, and `copy-write.move.v1` is not presented. Any
     other test changes only where it counts operations or templates. Report each.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
git merge-base --is-ancestor 6d3a1f14 HEAD; echo "base $?"  # base 0
sed -n '/^## DEC-171, /,/^## DEC-172, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
k packages/git/conduit.yaml 'version: 0.2.0' 'version: 0.3.0' 'move.v1'  # 1 0 0
k packages/git/Cargo.toml 'version = "0.2.0"'  # 1
k packages/git/Cargo.lock 'name = "conduit-git" version = "0.2.0"'  # 1
k packages/git/contracts.yaml 'move.v1' 'name: manage' "operations: ['#file.read.v1', '#directory.list.v1', '#file.write.v1', '#changes.v1']"  # 0 0 1
k packages/git/src/repository.rs 'git.write_view(&config.commit, &stage, &index)?;' 'return if root == copy.root && config.commit == copy.base {'  # 1 1
k packages/git/tests/mcp.rs 'async fn git_view_' 'fn move_'  # 4 0
k packages/git/tests/schema_drift.rs 'assert_eq!(mapping.len(), 6);'  # 1
k engine/crates/shell/tests/working_copy.rs '"version":"0.2.0"' '("copy-read", "read"), ("copy-write", "read-write")' 'move.v1'  # 1 1 0
k packages/git/README.md '`example.org/conduit/git` 0.2.0' '## The Git view' 'The package never reads the view' '## Moving the copy' 'Setup writes the Git view at root'  # 1 1 1 0 1
git --version | grep -cE '^git version (2\.(3[2-9]|[4-9][0-9])|[3-9])\.'  # 1
command -v cargo git | wc -l | tr -d ' '  # 2
```

Where a line prints otherwise, stop, change nothing and report it. Without order 403, the version,
`write_view`, `git_view_` and Git view lines print 0.

## Scope

Change, as the decisions say: in `packages/git`, `src/`, `contracts.yaml`, `tests/mcp.rs`,
`tests/schema_drift.rs`, the README, `conduit.yaml` and the version lines of `Cargo.toml` and
`Cargo.lock`; and `engine/crates/shell/tests/working_copy.rs`. Change nothing else,
`tests/support/mod.rs`, `spec/`, `docs/` and `experiments/` among them (DEC-171 stays not done);
no changelog fragment. Write outside the tree only in temporary directories the tests make.

## Done when

The first group prints otherwise today and passes after; the second, guards, passes both times.
Run each new test by name, once, then the package's tests once, last.

```sh
export PYTHONDONTWRITEBYTECODE=1
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
ok='^test result: ok\. 1 passed'
for t in move_drops_the_change_and_keeps_ignored_files move_applies_a_patch_uncommitted move_is_refused_unless_the_copy_is_as_read move_keeps_its_commit_across_setup_and_retirement move_that_ends_part_way_is_finished; do (cd packages/git && cargo test --offline --release --test mcp -- --exact "$t" 2>&1 | grep -c "$ok"); done  # 1 five times; 0 today
k packages/git/conduit.yaml 'version: 0.2.0' 'version: 0.3.0' 'move.v1' 'manage'  # 0 1 2 1; 1 0 0 0 today
k packages/git/Cargo.toml 'version = "0.3.0"'  # 1; 0 today
k packages/git/Cargo.lock 'name = "conduit-git" version = "0.3.0"'  # 1; 0 today
k packages/git/contracts.yaml 'name: move.v1' 'name: manage' "operations: ['#file.read.v1', '#directory.list.v1', '#file.write.v1', '#changes.v1', '#move.v1']"  # 1 1 1; 0 0 0 today
k packages/git/README.md '`example.org/conduit/git` 0.3.0' '## Moving the copy' 'The package cannot see an Agent' 'The `manage` template adds `move.v1`, below.' 'and each move writes it again;'  # 1 1 1 1 1; 0 0 0 0 0 today
k engine/crates/shell/tests/working_copy.rs '"version":"0.3.0"' '"version":"0.2.0"'  # 1 0; 0 1 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
ok='^test result: ok\. 1 passed'
(cd packages/git && grep -rhoE '"(push|pull|remote|ls-remote|credential|submodule)"' src | wc -l | tr -d ' ')  # 0
(cd packages/git && cargo fmt --check); echo "fmt $?"  # fmt 0
(cd packages/git && cargo clippy --offline --all-targets -- -D warnings >/dev/null 2>&1); echo "clippy $?"  # clippy 0
(cd engine && cargo fmt --all -- --check); echo "engine fmt $?"  # engine fmt 0
(cd engine && cargo test --offline --release -p conduit-shell --test working_copy 2>&1 | grep -c "$ok")  # 1, its move step included
git add -N packages/git/src
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, twice
python3 tools/set_names.py --check | grep -oE 'Errors: [0-9]+$'  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/git/(Cargo\.(toml|lock)|conduit\.yaml|contracts\.yaml|README\.md|src/[a-z_]+\.rs|tests/(mcp|schema_drift)\.rs)|engine/crates/shell/tests/working_copy\.rs)$' | wc -l | tr -d ' '  # 0
git reset -q -- packages/git/src  # stage nothing
(cd packages/git && cargo test --offline --release 2>&1 | grep -E '^(test result|error)' | cut -d. -f1 | sort -u)  # test result: ok, the package's tests, once, last
```

Your report gives, per decision, what changed and where; each new test's line on the unchanged
package, then its passing line; each existing test you changed, and why; your own choices; and
every check's last line. Stage nothing.

## What follows

- Order 405 declares the slots as copies, root's Conduit from `#manage`; 408's `wo slot` checks
  its ledger, reads, moves, and writes the order text by `file.write.v1`.
- An engine refusal while an Agent consuming the copy has model work in progress needs a promise
  of its own: a spec order, not yet planned.
- Order 413 commits in the private directory (DEC-034).

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` (with order 403's Git view)
at its start. Drafted and checked by the coordinator's Workflow; built and read on the team seats
(DEC-179, DEC-180, DEC-203).

**Worker-5's turn**, on the Business seat, built `move.v1` and its handling of a move that ends
part way, raising the git package to 0.3.0.

**Two blind reads.** The Codex verifier, `root-verifier-5.turn.v1`, passed it with no finding. The
Claude reader, `root-reader-a.turn.v1`, on the Team seat, found the move doing what the order
asks, with two shoulds and three nits on what an operator is told: a move that failed after its
record was saved returned only a bare cause; the README named no way out of a move that can never
finish; and three refusals did not say what to do next. One fix turn by worker-5 answered all
five. The gate's run on the squash is the coordinator's, at the merge.
