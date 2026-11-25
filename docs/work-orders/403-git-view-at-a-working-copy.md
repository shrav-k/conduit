# Work order 403: a working copy's root holds a Git view for the Agent's own Git, which the package never reads (DEC-171, DEC-180)

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

Beyond this checkout's own checks below, run Git only on repositories a test makes under its
temporary directory, under order 229's rule for Git in tests with decision 7's two exceptions.
Never run `git commit`, `git push` or a fetch outside a repository a test makes, nor
`git config --global`.

## What and why

**The owner's words.** [DEC-171](../decisions/log.md#dec-171-2026-11-23-the-coordinator-hands-off-through-written-state-its-kits-rust-replacements-follow-the-guard):
the kit -> "Order Rust replacements first"; when they run -> "After the guard chain
(Recommended)".
[DEC-180](../decisions/log.md#dec-180-2026-11-24-parallel-tracks-on-the-accounts): how the new
seats work -> "Parallel tracks (Recommended)"; and "We should also likely rewrite everything we
can into the engine now using the new accounts we have access to", which gives the kit's
replacements to pairs 3 and 4 now. The working copy is the `git` package's (DEC-021).
[DEC-018](../decisions/log.md#dec-018-2026-11-20-what-an-agent-reaches-enforcement-in-v0-after-s2):
reader-1 -> "for now to assess conduit we should allow reads and freedom for agents and discuss
the difference between these two". This order replaces one piece of the kit's slot build: an
Agent's own Git in its copy.

**Today.** The coordinator clones each order's base into a slot by hand and runs no Git there after
Agents worked there, since an Agent can plant hooks or configuration in its `.git` (order 229's
gap). `working-copy.v1` ([order 229](229-git-working-copy.md)) serves a copy at a detached commit
from a private Git directory (PRV-029), but root holds no `.git`. So an Agent's own Git there finds
no repository, or an enclosing tree's, and `git status`, `git diff --check`, checks on `HEAD` and
the guards that ask Git fail. Every order's checks run such lines, so the slots cannot yet be copies
(order 405).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, a
shared thing is owned once and never copied to an agent: the view lends the copy's private
objects by reference and copies no repository; 7, everything an agent does through Conduit is
attributable: no result is taken from the view, so nothing the Agent's own Git does there
reaches a record or a patch. Decisions 1 and 2 keep both.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes, by
existing practice. GitHub's [actions/checkout](https://github.com/actions/checkout) leaves a real
repository for a job's own Git, and `persist-credentials: false` keeps the token out of it.
[`git clone --shared`](https://git-scm.com/docs/git-clone) lends objects through
`objects/info/alternates`, as `changes.v1`'s scratch repository already does. Git's
[`protocol.allow`](https://git-scm.com/docs/git-config) refuses transports, and Git refuses an empty
identity. The package's Git directory stays private (order 229). Not taken: Conduit operations for
status and diff (a new template, PKG-031); a `git worktree` (the Agent's Git would write the
package's index and refs); a clone (a full one copies every object again, a shallow one lacks
history); refusing ref changes or hiding other objects (`git add -N` needs the view writable; an
Agent reads the source today, DEC-018).

**When it runs.** On `main`, now (DEC-180); order 404 follows. Through Conduit: one Codex worker
turn, the blind Codex verifier and the Claude reader at once (text an Agent meets, DEC-189), one fix
turn, the gate (DEC-050, DEC-033). By ruling 89, each decision says what must hold; how is yours. If
one turn cannot finish it, 403b takes decision 3's two refusals, their README words and
`git_view_refuses_commit_and_transport`, and 403a the rest. The coordinator's choices, open to the
owner's strike ([DEC-029](../decisions/log.md#dec-029-2026-11-20-who-decides-and-who-does)): the
view in root's `.git`, its objects by alternates, its index the checkout's and its configuration the
package's Git's; the two refusals; no view for a 0.1.0 copy; 0.2.0; the README's words; the test
names; and the reading that PRV-029 binds the engine, so the package may name its own objects
directory in the view (implementation-defined: any process of the engine's user can read it).

**Shared files.** Order 404 changes setup and `tests/mcp.rs` after this one. For other files,
the later of two orders rebases and keeps both.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), a file outside Scope that must change, or
Git on the run's `PATH` that does not refuse decision 3's commit or push in the view setup wrote
(give the command and its last line). Where this order's wording contradicts a promise it does
not change, the promise governs: apply it and list it under `own_choices`, with every choice no
decision governs.

## The decisions

Read first, in `packages/git`: `State::setup`, `Git::command` and `Git::checkout`; `take` and
`capture_paths` in `src/patch.rs`; `git_entry` and `list` in `src/files.rs`; `Fixture` and the
tests decision 7 names; the README; then PRV-029, PKG-025 and PKG-031. Wrap each insertion at
its file's width.

1. **The view, written at setup** (order 229's decision 3 stands otherwise). Before the rename
   that publishes the checkout at root, setup writes in it a `.git` directory, the view:
   - `HEAD`: the base commit's 40 digits and a newline, detached.
   - `index`: the private checkout's index, byte for byte.
   - `objects/info/alternates`: the absolute path of the instance's private
     `repository.git/objects` and a newline. The view holds no object of its own.
   - `info/attributes`: the private Git directory's, `* !filter !diff`.
   - `config`, one config text in which no section name stands alone as a string literal:
     `core.repositoryformatversion = 0`, `core.bare = false`, `core.logAllRefUpdates = false`;
     the ten values `Git::command` passes with `-c`, as written there, from `core.hooksPath` to
     the empty `credential.helper`; `protocol.allow = never`; empty `user.name` and
     `user.email`.
   - empty `refs/heads/` and `refs/tags/`, and the directories Git needs under `objects/`. No
     other file.

   No Git the package runs uses the view as its Git directory, index or configuration. A failed
   or interrupted setup removes the view with the staged checkout. The private Git directory is
   as today.
2. **The package never reads it.** After the rename, no operation or hook but retirement, which
   removes it with root following no link, opens, reads or runs anything under root's `.git`,
   and no Git the package runs names it. The file operations refuse or hide `.git`, and
   `changes.v1` skips it, as today. The Agent's Git there adds, removes or rewrites no file of
   the private Git directory; it may freshen a borrowed object's time. A setup repeated at the
   same root and commit changes nothing, the view included.
3. **What the Agent's own Git does there** (AGENTS.md; DEC-018). Run in root, it works on the
   view: `status`, `diff`, `log`, `show`, `ls-files`, `cat-file`, `rev-parse` and `merge-base`,
   and `add -N` and `reset -q -- PATH` on its index. Until order 413 commits and pushes through
   Conduit, an Agent changes no ref, `HEAD` or configuration there, and commits and pushes
   nothing (AGENTS.md: "Do not commit, push, open a pull request or switch branches"). By the
   view's configuration Git itself refuses a commit ("empty ident name (for <>) not allowed":
   the empty `user.*` outranks a global one, not the identity variables or `author.*` and
   `committer.*`) and every fetch and push, to a URL or a path ("transport 'file' not
   allowed"). Neither binds an Agent that sets its own identity or configuration (DEC-018):
   they stop a habit, not a will. A ref change is not refused. None of it reaches a result:
   `changes.v1` gives the change of root's files from the base, whatever the view holds.
4. **A copy that 0.1.0 made** keeps no view: a setup repeated at its root and commit returns
   early and changes nothing, as today. It gets one only when retired and set up again.
5. **0.2.0.** `packages/git/Cargo.toml`'s version and `Cargo.lock`'s own line, `conduit.yaml`'s
   `version`, the README's first sentence and the engine test's pin, so that a store holding
   0.1.0 is not refused this archive at the same version (PKG-025). No operation is added
   (PKG-031).
6. **The README.**
   - Its first sentence names `0.2.0`; "Root's own `.git` is skipped." becomes "Root's own
     `.git`, the Git view below, is skipped."
   - Before `## D15 inventory`, this section:

     ```text
     ## The Git view

     Setup also writes root's `.git`, a Git view, so that an Agent's own Git works in the copy: status, diff, log, `rev-parse`, `merge-base`, and `add -N` and `reset` on the view's own index. Its `HEAD` is the base, detached, and its index is the checkout's. It borrows the private Git directory's objects through `objects/info/alternates` and holds no ref, remote, hook or object of its own. Its configuration holds the values the package's own Git runs with, so `git status` lists the paths that `changes.v1` returns, and an empty identity and `protocol.allow = never`, so Git refuses a commit, a fetch and a push there unless the Agent's environment or configuration names an identity or allows a transport.

     The package never reads the view: nothing written there reaches a result, and `changes.v1` takes the change from root's files against the private base. No trusted process runs Git in the view, fetches from it or pushes to it: Git would read its configuration and run its hooks. A copy that 0.1.0 made has no view; retire it and set it up again.
     ```
   - The row `D15(c) Persistent state`, after "lock and private Git home there.", adds: "Setup
     writes the Git view at root's `.git` once; the package never reads it, and retirement
     removes it with root."
7. **Tests**, in `packages/git/tests/mcp.rs`, written first; run each on the otherwise unchanged
   package and report its line. "The Agent's Git" is `git` in root with `Scratch::git`'s environment
   and `GIT_CEILING_DIRECTORIES` the scratch path, unless a test says otherwise. The tests keep
   order 229's rule for Git in tests, that variable added, but for two exceptions:
   `git_view_writes_reach_no_result` commits in root's view, and
   `git_view_refuses_commit_and_transport` runs its Agent's Git with a global configuration file
   under its temporary directory and no identity variables, and runs `git push` and `git fetch` only
   to repositories there. The view refuses both.
   - `git_view_is_written_at_setup` (decisions 1, 2 and 4): a repository of two commits, set up
     at the second. Root's `.git` holds as files `HEAD`, `config`, `index`, `info/attributes`
     and `objects/info/alternates`, and no other; `HEAD` is the base and a newline; the
     alternates name the state directory's `repository.git/objects`; the index has the private
     one's bytes. The Agent's Git gives: `rev-parse --show-toplevel` root; `rev-parse HEAD` the
     base; `status --porcelain=v1 -uall` nothing; `log --format=%H` the base, then the first
     commit; `merge-base --is-ancestor FIRST HEAD` exit 0; `remote` and `for-each-ref` nothing.
     A repeated setup leaves every file under root's `.git` with its bytes. Then root's `.git`
     is removed, as a 0.1.0 copy has none, and setup repeated: it succeeds, and root has no
     `.git`.
   - `git_view_status_lists_what_changes_returns` (decisions 1 and 3): in root, `tracked.txt`
     gains a trailing space, `delete.txt` is removed, `mode.sh` becomes 0644, and `new.txt` and
     `ignored/extra` are written. The Agent's `status --porcelain=v1 -z -uall --no-renames`
     gives exactly `changes.v1`'s paths, `??` for each `added`, ` D` for each `deleted` and
     ` M` for each `modified`, and not `ignored/extra`; `diff --check` exits non-zero, naming
     `tracked.txt`; after `add -N new.txt`, `diff --name-only HEAD` lists the four paths; after
     `reset -q -- new.txt`, the status is as before. `changes.v1` gives the same result before
     those index changes and after.
   - `git_view_refuses_commit_and_transport` (decision 3): its global configuration names a user.
     `commit -am` exits non-zero with "empty ident name", and `HEAD` stays the base;
     `push BARE HEAD:refs/heads/x`, to a bare repository, and `fetch SOURCE` each exit non-zero with
     "transport 'file' not allowed", and the bare repository holds no ref.
   - `git_view_writes_reach_no_result` (decisions 2 and 3): `tracked.txt` is edited; the Agent's
     Git commits it, makes a branch and a tag, and, last, sets `core.fsmonitor` to a script
     that writes a marker outside root. Then `changes.v1` gives `base` the setup's commit and
     `files` `tracked.txt` modified, and its patch, by `git apply --index` on a clean clone at
     the base, gives root's files. The state directory's `repository.git` keeps every file's
     name and bytes from before the commit; retirement removes root with its `.git`; no marker
     exists.
   - Changed, in place of each assertion that root has no `.git`: in
     `setup_makes_the_copy_at_the_commit` and the engine's `working_copy_through_the_engine`,
     root's `.git/HEAD` holds the base and a newline, and the engine's project pins `0.2.0`; in
     `changes_rebuild_the_copy_from_its_base`, every file under root's `.git` keeps the name and
     bytes it had before the `changes.v1` call. `planted_git_entries_are_never_read_or_run`
     stays as it is and passes: it writes its configuration and hooks over the view's. Any
     other existing test changes only where it asserts that root has no `.git`. Report each.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
git merge-base --is-ancestor 1431aeb9 HEAD; echo "base $?"  # base 0
sed -n '/^## DEC-171, /,/^## DEC-172, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
k packages/git/conduit.yaml 'version: 0.1.0' 'version: 0.2.0'  # 1 0
k packages/git/Cargo.toml 'version = "0.1.0"'  # 1
k packages/git/Cargo.lock 'name = "conduit-git" version = "0.1.0"'  # 1
k packages/git/src/repository.rs 'git.checkout(&config.commit, &stage, &index)?;' '"credential.helper=",' 'protocol.allow'  # 1 1 0
k packages/git/src/patch.rs '"../../../repository.git/objects\n",'  # 1
k packages/git/tests/mcp.rs 'assert!(!f.root.join(".git").exists());' 'async fn planted_git_entries_are_never_read_or_run()' 'fn git_view_'  # 2 1 0
k engine/crates/shell/tests/working_copy.rs 'assert!(!root.join(".git").exists());' '"version":"0.1.0"'  # 1 1
k packages/git/README.md '`example.org/conduit/git` 0.1.0' 'own `.git` is skipped.' '## The Git view'  # 1 1 0
k AGENTS.md 'Do not commit, push, open a pull request or switch branches'  # 1
(cd packages/git && grep -rhoE '"(push|pull|remote|ls-remote|credential|submodule)"' src | wc -l | tr -d ' ')  # 0
git --version | grep -cE '^git version (2\.(3[2-9]|[4-9][0-9])|[3-9])\.'  # 1
command -v cargo git | wc -l | tr -d ' '  # 2
```

Where a line prints otherwise, stop, change nothing and report it. A `repository.rs` line other
than `1 1 0` means setup or the package's Git moved since this order was written.

## Scope

Change, as the decisions say: `packages/git/src/`; `packages/git/tests/mcp.rs`;
`packages/git/README.md`; the version lines of `packages/git/Cargo.toml`, `Cargo.lock` and
`conduit.yaml`; and in `engine/crates/shell/tests/working_copy.rs` the one assertion and the pin.
Change an existing test only under decision 7. Change nothing else, `contracts.yaml`,
`tests/support/mod.rs`, `spec/` and `docs/` among them (DEC-171 stays not done); no changelog
fragment. Write outside the tree only in temporary directories the tests make.

## Done when

The first group prints otherwise today and passes after; the second, guards, passes today and
after. Run each new test by name, once, then the package's tests once, last.

```sh
export PYTHONDONTWRITEBYTECODE=1
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
ok='^test result: ok\. 1 passed'
for t in git_view_is_written_at_setup git_view_status_lists_what_changes_returns git_view_refuses_commit_and_transport git_view_writes_reach_no_result; do (cd packages/git && cargo test --offline --release --test mcp -- --exact "$t" 2>&1 | grep -c "$ok"); done  # 1 four times; 0 today
k packages/git/conduit.yaml 'version: 0.1.0' 'version: 0.2.0'  # 0 1; 1 0 today
k packages/git/Cargo.toml 'version = "0.2.0"'  # 1; 0 today
k packages/git/Cargo.lock 'name = "conduit-git" version = "0.2.0"'  # 1; 0 today
k packages/git/README.md '`example.org/conduit/git` 0.2.0' 'own `.git`, the Git view below, is skipped.' '## The Git view' 'No trusted process runs Git in the view' 'Setup writes the Git view at root'  # 1 1 1 1 1; 0 0 0 0 0 today
k packages/git/tests/mcp.rs 'assert!(!f.root.join(".git").exists());'  # 0; 2 today
k engine/crates/shell/tests/working_copy.rs 'assert!(!root.join(".git").exists());' '"version":"0.2.0"'  # 0 1; 1 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
ok='^test result: ok\. 1 passed'
(cd packages/git && grep -rhoE '"(push|pull|remote|ls-remote|credential|submodule)"' src | wc -l | tr -d ' ')  # 0
(cd packages/git && cargo fmt --check); echo "fmt $?"  # fmt 0
(cd packages/git && cargo clippy --offline --all-targets -- -D warnings >/dev/null 2>&1); echo "clippy $?"  # clippy 0
(cd engine && cargo fmt --all -- --check); echo "engine fmt $?"  # engine fmt 0
(cd engine && cargo test --offline --release -p conduit-shell --test working_copy 2>&1 | grep -c "$ok")  # 1
git add -N packages/git/src
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, twice
python3 tools/set_names.py --check | grep -oE 'Errors: [0-9]+$'  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/git/(Cargo\.(toml|lock)|conduit\.yaml|README\.md|src/[a-z_]+\.rs|tests/mcp\.rs)|engine/crates/shell/tests/working_copy\.rs)$' | wc -l | tr -d ' '  # 0
git reset -q -- packages/git/src  # stage nothing
(cd packages/git && cargo test --offline --release 2>&1 | grep -E '^(test result|error)' | cut -d. -f1 | sort -u)  # test result: ok, the package's tests, once, last
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML. Your report gives, per decision, what
changed and where; each new test's line on the unchanged package, then its passing line; each
existing test you changed, and why; your own choices; and every check's last line. Stage
nothing.

## What follows

- Order 404 rewrites the view with the copy; 405 makes the slots copies; 413 commits in the
  private directory (DEC-034) and keeps every object the view borrows.
- A confined Agent (DEC-018) needs the alternates' path readable and the view owned by its user.
- DEC-171's status is the coordinator's once its replacement orders merge.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` at its start. Drafted and
checked by the coordinator's Workflow; built and read through Conduit.

**Worker-2's turn**, on the primary ChatGPT account, built the Git view at setup, kept the package
from ever reading it, and raised the git package to 0.2.0, in 19 minutes.

**Two blind reads.** The Codex verifier, `root-verifier-2.turn.v1`, passed it with no finding. The
Claude reader, `root-reader-a.turn.v1`, on the Team seat, passed it with three shoulds and two
nits, all in the package's README: say that an Agent changes no ref, `HEAD` or configuration in
the view; narrow `reset` to `reset -- PATH`; say that the view's alternates name the instance's
private objects by absolute path, readable because the Agent runs unconfined in v0; and name the
author or committer identity that lifts the commit refusal. The coordinator made these README
edits (ruling 87), since the pair's working copy had gone to the next order; the test-marker nit
stays as the order prescribed it. The gate's run on the squash is the coordinator's, at the
merge.
