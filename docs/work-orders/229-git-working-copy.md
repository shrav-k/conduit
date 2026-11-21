# Work order 229: dogfood gap G-5, the `git` package's first slice: a Pool-owned working copy, its `read` and `read-write` Conduits, and its changes as a patch

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate; its only sockets are the loopback ports the
engine's own tests bind. Run one cargo command at a time. Never print a whole large file, case,
fixture or log. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential. Do not run
`conduit` against a store outside a test, `codex`, `claude` or the dogfood harness, and sign in
nowhere.

**The git rule.** Beyond this order's checks (`git merge-base`, `git --version`,
`git status`, `git diff`, `git add -N`, `git reset`), run git only on repositories a test makes
under its temporary directory: never on this checkout or another of the user's; never
`git push`, a remote fetch or `git config --global`. Every git a test runs has its environment
cleared but `PATH`, with `HOME` a temporary directory, `GIT_CONFIG_NOSYSTEM=1`,
`GIT_CONFIG_GLOBAL=/dev/null`, and author and committer `Conduit test <test@conduit.invalid>`.

## What and why

**The gap** (the dogfood-gaps stream, step G-5; [ruling 92](../proposals/ruling-92/README.md),
point 9; [DEC-027](../decisions/log.md#dec-027-2026-11-20-conduit-builds-conduit-directly-through-conduit)).
Running order 215 directly through Conduit showed it has no working copy. The coordinator
readied each Agent's directory by hand with git, and recloned it after Agents ran there. The
worker and the verifier shared one directory, the verifier read-only by message alone. The
result came back by copying files into a trusted worktree, since an unconfined Agent can plant
hooks or configuration in its directory's `.git`.

**The owner's package decision**
([DEC-021](../decisions/log.md#dec-021-2026-11-20-how-packages-are-organized)): a `git` package
for the working copy, and a `github` package for hosted operations (G-6, later).

**This slice** is the working copy alone: a Resource that a Pool owns, set up from a local
repository and a commit; standard `read` and `read-write` Conduit templates, so a verifier gets
a read Conduit to a worker's copy; and an operation returning the copy's changes as a patch
against its base, which the root Interface applies instead of copying files out of an Agent's
directory. Fetch, push, the network and the token are later slices (see "What this leaves").

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
3, a shared thing is owned once and never copied to an agent (the copy is the Pool's; its
changes leave as a result); 2, access can be inspected and revoked (a verifier's reach is a read
Conduit); 7, attributable (a write is a request through a named Conduit).

**Should this exist** (point 3). Each part is an existing paradigm: a checkout at a commit
owned by the pipeline, not the job (`actions/checkout`, Tekton's `git-clone`, `git-sync`); read
and read-write views of one tree (S1's `directory-read` and `directory-read-write`; PKG-031); a
patch for `git apply`. The package runs `git` and adds no version control (ruling 40).

Not taken (the coordinator's, open to the owner's strike): `mcp-server-git` (Python; its diffs
leave out untracked and binary files; works in the `.git` the Agent can change); `gix` or
`git2` (not in the offline cache); the git directory under root (the gap above).

**When it runs.** Through Conduit's dogfood: one Codex worker turn (4-hour limit; aim for under
2 hours), a Codex verifier and a Claude reader ([ruling 79](../proposals/ruling-79/README.md)),
who run only what settles a doubt, and the gate
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). By
[ruling 89](../proposals/ruling-89/README.md), each decision says what must hold; how is yours.
The names are the coordinator's, open to the owner's strike.

## The decisions

1. **The package.** `packages/git/`, in provider-file's form (its own `Cargo.lock`,
   `rust-toolchain.toml` at 1.97.1, `.gitignore`, `conduit.yaml`, `contracts.yaml`, `README.md`,
   `tests/package.rs`, `tests/schema_drift.rs`, and `src/bin/package.rs` building
   `dist/git.tar.gz` and its digest). Cargo package `conduit-git`, binary `bin/conduit-git`, never
   taken for git; address `example.org/conduit/git`, version 0.1.0; Unix only, as
   `provider-local-directory`. Start the lock from a copy of provider-file's: resolving offline
   then adds no crate.
2. **The contracts.**
   - Type `working-copy.v1`, family Resource: offers `#copy.v1`, lists no control, requires core
     `setup-hook.v1` and `retire-hook.v1`, implements all six operations by one item `#server`,
     `command: [bin/conduit-git]`, not `plain`, so every call carries Conduit's keys (IFC-006),
     and states `records: {inputs: true, outputs: false}`. Its schema is `{repository, commit,
     root}`, all required, no other key: `repository` an absolute path (`^/`) to a local git
     repository, `commit` 40 lowercase hex digits, `root` the absolute path of the copy.
   - Capability `copy.v1`, four operations, each schema closed: `file.read.v1` `{path}` gives
     `{content}`; `directory.list.v1` `{path}` gives `{entries: [{name, kind}]}`, `kind` one of
     `file`, `directory`, `symlink`, `other`, sorted by `name`'s bytes; `file.write.v1` `{path,
     content}`, whose `effect` creates or replaces the file at path in the copy; `changes.v1`
     `{}` gives `{base, files: [{path, change}], patch}`, `change` one of `added`, `modified`,
     `deleted`, with no effect.
   - Conduit templates under PKG-031, as the S1 fixture `release-adds-optional` exports them:
     `read` lists `file.read.v1`, `directory.list.v1` and `changes.v1`; `read-write` all four.
   - A `path` is `.` (root), or relative with no empty, `.`, `..` or `.git` (any case) segment.
3. **Setup** (`setup-hook.v1`).
   - The git directory is in the instance's state directory (PRV-029, `CONDUIT_STATE_DIR`),
     never under root, and root holds no `.git` entry.
   - Root must not exist or be an empty directory; its parent must exist; no part of its path
     may be a symbolic link. The copy is root checked out at `commit`, detached: every tracked
     file of its tree, with modes and links as git checks them out.
   - The repository is only fetched from, by its local path; no command runs in it, and no
     network or protocol but local is allowed.
   - Every git the program runs ignores system and global configuration and uses no hook,
     template directory, credential helper, prompt or pager; its configuration comes only from
     the package's own git directory, never from root. Its `HOME` and `XDG_CONFIG_HOME` name a
     directory in the state directory, so no per-user ignore or attributes file is read. git is
     found on the run's `PATH`, 2.32 or later; an older one fails, naming the version.
   - A setup at the root and commit of a copy already made succeeds and changes nothing; one
     after a failed or interrupted attempt starts again, removing only what that attempt made.
     Any other failure is outcome failed with a fixed message naming the cause (root not empty,
     commit not in the repository, repository unreadable, git missing or too old, no state
     directory), and makes nothing under root.
   - A configuration the schema refuses fails before anything is made. The hooks come through
     no Conduit: their `_meta` holds no `conduit` key (IFC-006).
4. **Reads and writes** act only inside root, following no symbolic link at any segment; a path
   through a link, or naming a file of the wrong kind, fails, naming the path. A read takes
   UTF-8 of at most 8 MiB; a write takes UTF-8 content of at most 8 MiB, creates missing parent
   directories, and replaces a regular file, never a directory or a link; a failed write leaves
   the earlier file whole. Each call's configured `root` must be the copy this instance made,
   as its state directory records, or the call fails and touches nothing. Calls to one copy run
   one at a time.
5. **Changes** (`changes.v1`) is the difference between `commit` (`base`) and root's files as
   they stand, tracked and untracked, minus what root's ignore files exclude (an edit to an
   ignore file is itself in the patch). `patch` is in the form of `git diff --binary
   --full-index --no-renames` against base: `git apply --index` on a clean checkout of base
   gives root's files exactly, contents, executable bits, links, binary files and a file a
   `.gitattributes` marks `-diff` included. No attribute in root changes a byte (the git
   directory's `info/attributes` can unset `text`, `eol`, `filter`, `ident` and
   `working-tree-encoding`). `files` lists each changed path, sorted; with no
   change both are empty. It changes nothing under root (no index, no lock). It fails, naming
   the cause, with no partial patch, where a directory below root, not excluded by the ignore
   files, holds a `.git` entry (named; root's own `.git` is skipped and never in the patch),
   the patch is not UTF-8 or exceeds 8 MiB, or a path is not UTF-8.
6. **Retirement** (`retire-hook.v1`) removes root with what it holds, and the git directory,
   following no symbolic link, so a link's target outside root stays. A repeated retirement
   succeeds.
7. **The README**, in provider-file's form with its D15 inventory, states the configuration;
   Pool ownership as the engine test writes it (`read` for the root Interface or a verifier,
   `read-write` for a worker, whose `directory` may name the copy's `root`); the git it needs,
   which the archive's digest does not pin (MAN-010); to take `changes.v1` before retiring; the
   ignore-file limit; no network, remote or commits; and the trust model. No configuration or
   hook is read from root, but in v0 an Agent runs unconfined under the owner's user, so a read
   Conduit narrows what Conduit grants, not what the Agent's own tools reach
   ([DEC-018](../decisions/log.md#dec-018-2026-11-20-what-an-agent-reaches-enforcement-in-v0-after-s2)).

## The tests

**Package tests**, `packages/git/tests/mcp.rs`. They drive `bin/conduit-git` over stdio with
`_meta` as the engine sends it, as provider-file's `tests/mcp.rs` does. Each run's environment
is cleared except `PATH` and `CONDUIT_STATE_DIR`, which is a temporary directory. Each test,
the engine's too, canonicalizes its temporary directory, as `/var` and `/tmp` are links on
macOS. The tests keep to the git rule. At least these tests, by these names:

- `setup_makes_the_copy_at_the_commit`: in a repository of two commits, a setup at the first
  gives root its files and modes, no `.git` entry, and the git directory under the state
  directory; a repeated setup changes nothing, and `changes.v1` gives an empty patch, `base` the
  commit. A root not empty, a commit not in the repository, 39 hex digits, a repository that is
  not one, and a relative root each fail and make nothing.
- `reads_lists_and_writes_stay_inside_root`: a write reads back the same; a list is sorted; the
  paths `..`, absolute, `.git`, `.GIT`, an empty segment and a link inside root to a directory
  outside it each fail and write nothing outside root; so do a non-UTF-8 file and one of 8 MiB
  and a byte.
- `changes_rebuild_the_copy_from_its_base`: writes, deletes, a mode change and a new link made
  directly in root, as an Agent would; a binary file; a file marked `-diff`; a `.gitattributes`
  written in root with `* text eol=crlf` beside a file with CRLF line endings; a file the base's
  `.gitignore` ignores. The patch, applied by `git apply --index` to a fresh clone at the base,
  gives root's files; `files` and `base` are right; the ignored file is not in it.
- `planted_git_entries_are_never_read_or_run`: root gets a `.git` directory whose `config` sets
  `core.hooksPath`, `core.fsmonitor` and `core.pager` to a script that writes a marker outside
  root, with that script as every hook, and a `sub/.git`. A write, a read, a list, `changes.v1`
  (failed, naming `sub`; with `sub/.git` removed, succeeded) and a retirement leave no marker,
  and no patch names a `.git` path.
- `retire_removes_the_copy_and_nothing_outside`: root and the git directory are gone, a link's
  target outside root stays, a second retirement succeeds, and a new setup at root succeeds.

The file keeps provider-file's discovery, extension and filter refusal tests.
`tests/schema_drift.rs` checks the contracts against `spec/` as provider-file's does, and the
exports and each template's operations; `tests/package.rs` checks the archive.

**Engine test**, `engine/crates/shell/tests/working_copy.rs`, one test,
`working_copy_through_the_engine`. It packs `packages/git` as `tests/secrets.rs` packs
provider-file, makes a repository of one commit, and drives the CLI as `tests/serve.rs` does,
with serve's `HOME` a temporary directory and a project whose Pool owns a `working-copy.v1`
Resource `copy`, and the root Interface's two Conduits to it: `copy-read` from
`example.org/conduit/git#read` and `copy-write` from `#read-write`. It shows that after setup
root holds the commit's files and no `.git`, and the copy's state directory in the store holds
the git directory; that `copy-write.file.write.v1` writes; that `copy-read` presents no
`file.write.v1` (`conduit tools` lists none, a call is refused and writes nothing); that
`copy-read.changes.v1`'s patch, applied by `git apply --index` to a fresh clone at the commit,
gives root's files; and that retiring `Resource/copy` removes root.

**The gate.** `tools/gate.py` gains the `RUST_READS` row
`('packages/git', 'spec/', 'tests/schema_drift.rs', '"../../spec'),` and `'packages/git/'` in
`ENGINE_PREFIXES` and `ENGINE_WHY`, since the engine test reads it;
`tools/tests/test_gate.py` gains `('packages/git/x', set(), True)` beside provider-file's case.

## What this leaves

Report this list, corrected by what you built: the dogfood's project using the copy (each
worker's own copy, the verifier's read Conduit and own directory, the root Interface applying
the patch; the coordinator's, after the merge); fetch and push, and the Pool-owned token by
secret reference (SEC-010); commits and branches; a verifier's own copy with the worker's patch
applied, to run checks; a build cache kept across copies; an Agent confined to its copy
(DEC-018); SHA-256 repositories, submodules and Windows; the `github` package (G-6).

## Stop rule

Stop, change nothing further, and report the gap and the options it leaves, without choosing,
where: a crate is not in the cache; a correct result needs a change under `spec/` or to an
engine source, a case, an engine `Cargo.toml` or `engine/Cargo.lock`; the engine does not
dispatch a keyed setup or retire hook to a Resource's command item, or gives that run no
`CONDUIT_STATE_DIR`; a project cannot take a package's `read` template by `from:`; the Mac's git
is older than 2.32; or a test would run git outside its temporary directory.

A choice that no promise governs is yours: list it under `own_choices`.

## What to read

`packages/provider-file`; `engine/crates/shell/tests/secrets.rs` and `serve.rs`;
`engine/examples/s1/instances.yaml`; the hooks in `spec/core/operations.yaml`; IFC-006, PKG-031
and PRV-029.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 59cf7ce9 HEAD; echo "main $?"
test -e packages/git; echo "package $?"
test -e engine/crates/shell/tests/working_copy.rs; echo "test $?"
grep -c 'packages/git' tools/gate.py tools/tests/test_gate.py
grep -c "'packages/provider-file/')" tools/gate.py
git --version | grep -cE '^git version 2\.(3[2-9]|[4-9][0-9])\.'
d=$(mktemp -d) && cp packages/provider-file/Cargo.toml packages/provider-file/Cargo.lock packages/provider-file/rust-toolchain.toml "$d"/ && mkdir "$d/src" && echo 'fn main() {}' > "$d/src/main.rs" && (cd "$d" && cargo metadata --offline --locked --format-version 1 >/dev/null 2>&1); echo "offline $?"; rm -rf "$d"
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform aarch64-apple-darwin >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `package 1`, `test 1`, two lines ending `:0`, `1`, `1`, `offline 0`,
`engine 0` and `0`; another `offline` or `engine` line means a crate is not in the cache.

## Scope

Create `packages/git/` and `engine/crates/shell/tests/working_copy.rs`; change `tools/gate.py`
and `tools/tests/test_gate.py` as above, and `engine/STANDING.md` only by
`promise_standing.py --write` if its check asks. Change nothing else: not `spec/`,
`conformance/`, `experiments/`, `docs/`, `.github/`, `IN_FORCE.md`, `ROADMAP.md`, another
package, an engine source, `engine/CONFORMANCE.md`, a `Cargo.toml` outside `packages/git/`, or
`engine/Cargo.lock`. Put anything else that needs changing in your report.

## Done when

Each line marked "fails today" prints otherwise on `main` at `59cf7ce9`; all pass after.

```sh
(cd packages/git && cargo metadata --offline --locked --format-version 1 >/dev/null); echo "offline $?"  # offline 0; fails today (1)
(cd packages/git && cargo fmt --check)  # passes; fails today
(cd packages/git && cargo clippy --all-targets -- -D warnings)  # passes; fails today
(cd packages/git && cargo clippy --all-targets --target x86_64-unknown-linux-gnu -- -D warnings)  # passes, as CI's ubuntu job builds it; fails today
(cd packages/git && cargo test --release)  # passes; fails today
(cd packages/git && cargo test --release --test mcp -- --list 2>/dev/null | grep -cE '^(setup_makes_the_copy_at_the_commit|reads_lists_and_writes_stay_inside_root|changes_rebuild_the_copy_from_its_base|planted_git_entries_are_never_read_or_run|retire_removes_the_copy_and_nothing_outside): test$')  # 5; fails today (prints nothing)
(cd packages/git && grep -rhoE '"(push|pull|remote|ls-remote|credential|submodule)"' src | wc -l | tr -d ' ')  # 0; fails today (prints nothing)
(cd engine && cargo test --release -p conduit-shell --test working_copy)  # 1 passed; fails today (no such test target)
grep -c "('packages/git', 'spec/'" tools/gate.py  # 1; fails today (0)
grep -c "'packages/git/'" tools/gate.py  # 1; fails today (0)
grep -c "'packages/git/x'" tools/tests/test_gate.py  # 1; fails today (0)
```

Then run `git add -N packages/git engine/crates/shell/tests/working_copy.rs` so that the checks
see the new files. These pass, as on `main`; run the whole suite once, last, then
`git reset -q`, and stage nothing:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
python3 tools/check_docs.py  # Errors: 0
python3 tools/set_names.py --check  # passes
python3 tools/check_schemas.py  # Errors: 0
python3 tools/promise_standing.py --check  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
python3 -m unittest discover -s tools/tests  # OK
git diff --check  # prints nothing
git diff --name-only HEAD | grep -vE '^(packages/git/|engine/crates/shell/tests/working_copy\.rs$|tools/gate\.py$|tools/tests/test_gate\.py$|engine/STANDING\.md$)' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite
```

Your report gives, per decision, what you built and its tests; each failed setup's message; the
engine test's `changes.v1` result; how git is kept off root's configuration and hooks;
`own_choices`; the corrected list of what this leaves; every check's last line.

## Record

Not yet run.
