# Git working copy

`example.org/conduit/git` 0.3.0 is a Unix package for the engine's own runner. One
`working-copy.v1` Resource serves a copy of a local SHA-1 Git repository at a
detached commit. Install `dist/git.tar.gz` with its SHA-256 digest and pin the
version and digest in the project manifest. Build the archive with
`cargo run --offline --locked --release --bin package` in this directory. The
host must provide Git 2.32 or newer on the runner's `PATH`. The archive's digest
pins the package's binary and contracts. Host Git is an external dependency
under MAN-010.

Give the Resource template a `config` with three fields:

| Field | Value |
| --- | --- |
| `repository` | Absolute path to the local source repository. |
| `commit` | Full 40-digit lowercase hexadecimal commit ID. |
| `root` | Absolute destination with an existing parent. The destination must be absent or an empty directory. Its path cannot contain symbolic links. |

The Pool owns the Resource. For example, use a Pool instance named `workspace`
and a Resource instance named `copy` with `owner: workspace`. A root Interface
or verifier can consume a Conduit with `from: example.org/conduit/git#read` to
`Resource/copy`. A worker can consume `#read-write` at the same package address
and use the copy's `root` as its working directory. The `read` template exposes
`file.read.v1`, `directory.list.v1` and `changes.v1`. The `read-write` template
adds `file.write.v1`. The `manage` template adds `move.v1`, below.
File and directory paths are relative to the copy. Only
the spelling `.` names its root. Empty segments, traversal segments and
case-insensitive `.git` segments are refused. File reads and writes are limited
to 8 MiB of UTF-8.

Take `changes.v1` before retiring the Resource. It returns the base commit, a
sorted list of changed paths and a patch for `git apply --index` on a clean
checkout of the same base. The patch includes tracked changes and untracked
files that Git's ignore rules include. Review the ignore files before relying
on the patch to transfer a newly created file. Changes to the ignore files
themselves can appear in the patch. A visible nested `.git` entry fails the
operation. Root's own `.git`, the Git view below, is skipped. The package
captures files and links through the opened copy, stages the private snapshot,
and applies the candidate patch to a clean private checkout. A patch that does
not reproduce the captured bytes, Git executable modes, and symlinks fails.
For example, an LF edit to a file that the base requires Git to check out with
CRLF can be inexpressible.
The patch limit is 8 MiB. Retirement removes the owned copy and its private
Git directory through opened directory descriptors.

The package supports local setup and patch extraction. Network remotes,
pushing, creating commits, submodules, SHA-256 repositories and Windows are
outside this release. The package imports loose and packed Git objects from
the source and its local alternate object stores without running Git in
the source repository. Git uses the package's own repository and home.
Checkout honors built-in base
attributes such as line endings. Custom filters, configuration and hooks
planted in the copy are never used. In v0, an Agent runs unconfined under the
owner's OS user. A read Conduit limits requests made through Conduit. The Agent's own tools keep
the access of that OS user.
[DEC-018](../../docs/decisions/log.md#dec-018-2026-11-20-what-an-agent-reaches-enforcement-in-v0-after-s2)

## Moving the copy

`move.v1` moves the copy in place to another commit of its repository. Give it
the `base` that `changes.v1` last returned and the SHA-256, in lowercase
hexadecimal, of that call's `patch`: unless the copy is still as that read gave
it, the move fails and changes nothing, so it drops no change made before it
that its caller has not read. It drops the copy's change and keeps each
untracked entry that the ignore rules before the move and after it (the
commit's, with any patch applied) both ignore and the moved copy does not
track, such as `target/`; any other ignored entry goes too. An optional
`patch`, at most 8 MiB and in `changes.v1`'s form, is applied unstaged.
Root stays the same directory, with a new Git view at the commit. The record
keeps the move's inputs, the patch among them.

The package cannot see an Agent's turn: move a copy only when no Agent works
in it. A move that ends part way leaves the copy as its record names; repeat
that move with the same inputs to finish it. Until then `changes.v1` and other
moves are refused. File reads, directory listings and file writes still work
on the partly moved copy. The finishing move drops anything written to the
copy meanwhile, without checking the expectations again. If the move cannot
finish, retire the Resource and set it up again.

The Resource's `commit` stays setup's: setting it up again leaves the copy
where its last move put it, and another commit is refused.

## The Git view

Setup also writes root's `.git`, a Git view, so that an Agent's own Git works in the copy:
status, diff, log, `rev-parse`, `merge-base`, and `add -N PATH` and `reset -- PATH` on the view's
own index. Its `HEAD` is the base, detached, and its index is the checkout's. It holds no ref,
remote, hook or object of its own: it borrows the objects of the private Git directory through
`objects/info/alternates`, which names the absolute path of `repository.git/objects` in the
instance's state directory (PRV-029). The Agent's Git reads objects there, which it can because
in v0 it runs as the owner's OS user (DEC-018); a confined Agent would need that path readable.
Its configuration holds the values the package's own Git runs with, so `git status` lists the
paths that `changes.v1` returns, and an empty identity and `protocol.allow = never`, so Git
refuses a commit, a fetch and a push there unless the Agent's environment or configuration names
an author or committer identity, or allows a transport.

An Agent changes no ref, `HEAD` or configuration in the view, and commits or pushes nothing there;
commits and pushes will go through Conduit in a later release. The refusals stop a habit, not a
will: a ref change or a checkout is not refused. A checkout or a reset of `HEAD` rewrites root's
files, and `changes.v1` returns those files' change from the base.

The package never reads the view: nothing written there reaches a result,
and `changes.v1` takes the change from root's files against the private base.
No trusted process runs Git in the view, fetches from it or pushes to it:
Git would read its configuration and run its hooks. A copy that 0.1.0 made
has no view; retire it and set it up again.

## D15 inventory

| Item | Implementation and evidence |
| --- | --- |
| D15(a) Package-owned behavior | `src/lib.rs` negotiates IFC-037 and validates keyed requests and configuration. `src/objects.rs` imports local Git objects. `src/repository.rs` owns checkout state and retirement. `src/patch.rs` captures and verifies changes. `src/files.rs` handles bounded reads, directory listing and atomic writes. The stdio tests exercise successful and refused operations. |
| D15(b) Delegated behavior | Host Git checks out the private commit, evaluates ignore rules against a snapshot, produces the binary-capable diff, and applies each candidate for verification. Unix descriptors and an advisory lock come from rustix. rmcp supplies MCP over stdio. tar, flate2 and sha2 build and digest the archive. |
| D15(c) Persistent state | The runner supplies `CONDUIT_STATE_DIR` under PRV-029. The package keeps the Git directory, owner record, lock and private Git home there. Setup writes the Git view at root's `.git`, and each move writes it again; the package never reads it, and retirement removes it with root. A pending checkout can be cleaned and retried. Retirement removes the served copy and Git directory. |
| D15(d) Operator work and recovery | Supply host Git, a readable local repository, a pinned commit and a suitable destination. Repair the reported path or permissions and retry setup. The engine integration test installs the archive, sets up a Pool-owned copy, checks its three Conduits, applies a patch, moves the copy and retires the Resource. |
