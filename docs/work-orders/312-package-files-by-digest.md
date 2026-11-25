# Work order 312: an earlier store's packages move out of `state.json`, and runs of one digest share one read-only program copy (DEC-148, after order 278)

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
copy or read no store but one a test makes in a temporary directory: never the dogfood's. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute
local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-148](../decisions/log.md#dec-148-2026-11-23-package-files-stored-once-by-digest-run-leftovers-removed):
where installed packages' bytes live -> "In the store, by digest (Recommended)", landing in "each
installed archive is kept once by its digest in the store's directory, and a run's program copy
once per digest, read-only and checked before each run; the state keeps an index"; run leftovers
-> "Remove at run end (Recommended)".
[DEC-149](../decisions/log.md#dec-149-2026-11-23-the-store-is-slimmed-now): "Yes, order them now
(Recommended)", in three engine orders, of which this is "package files by digest (DEC-148)".
[DEC-138](../decisions/log.md#dec-138-2026-11-23-the-engines-state-is-a-resource-store-of-the-system-pool):
"this entire approach is suspect and holds state in a way that's not configurable or
transferrable". [DEC-143](../decisions/log.md#dec-143-2026-11-23-the-dogfood-store-moves-after-the-slimming-at-a-gate):
the dogfood store moves "After slimming, at a gate (Recommended)".

**Today**, read on `main` at `4749a0da`, with orders 278, 285 and 311 merged:

1. [Order 278](278-kept-archives-and-platforms-at-install.md) keeps each archive the engine
   installs as `packages/HEX` in the store, 0600 in a 0700 directory, before the commit that
   lists it (`store.rs`, `keep_archive`; shell `lib.rs`, the `install-archive` decision before
   `persist`); `state.json` omits such a package's `files` (`write_state`), and each read
   restores them from the copy through the shared tar reader (`read_disk`, `read_archive`),
   failing closed on a copy that is absent, altered, or of the wrong kind or mode. Order 285's
   `run.v2` takes its `archive` from that copy (`archive_descriptor`).
2. A package installed before 278 keeps its members in `state.json` for the store's life:
   `read_disk` restores only packages whose `files` is absent, `write_state` writes `files` for
   every package whose digest is not kept, and nothing ever keeps an archive for one
   (`members_kept_in_state_json_before_order_278_still_load` guards it). 278's "What follows"
   left the move to a later order, since it needs the archive. Measured on the dogfood store on
   2026-11-23, by sizes alone: six such packages are 100.8 MB of a 707 MB `state.json`, rewritten
   on every commit; and `run.v2` of such a package is refused, `archive_descriptor` answering "no
   kept archive".
3. Every run of a relative program writes the package's members into a new `programs/<uuid>`
   (`runner.rs`, `materialize_package`, called from `command`), writable, shared with no other run
   and checked by nothing. Measured the same day: 102 copies, 522 MB.
4. A keep that fails is answered as a state write failing before its rename (shell `lib.rs`,
   the closure before `persist`). One test covers it
   (`an_archive_directory_with_special_mode_bits_cannot_receive_an_install`): through an
   `install-archive` input, it checks the error, that no copy is made and that a reopened store
   lists no package. None checks `state.json`'s bytes, memory against disk, the public
   `management.install.v1`, or a keep that succeeds before the state write fails.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, a
shared thing is owned once: each installed archive is kept once, earlier installs too, and one
program copy serves every run of its digest; 6, nothing repeats an effect implicitly: the move
runs before recovery, installs and sends nothing, and a run executes the same bytes as today.
None at risk: no reply, record, inspection or case changes.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, for what 278 left: an earlier store's members, every run's copy and one untested failure.
By existing practice: Kubernetes' storage version migration rewrites each stored object in the
current form once, the old form readable until it is done; the Nix store makes each path once
per hash, read-only (files `0444` or `0555`, directories `0555`), shared by every user of that
hash, and `nix-store --verify --check-contents` checks it against the recorded hash. 278 already
does, and this order drops: the kept archive at `packages/HEX`, its durability before the commit,
`state.json` without a kept package's members, the restore through the shared tar reader, the
fail-closed read, and the opening of an earlier store; the descriptor `run.v2` reads is 285's.
Not taken: format version 2 with a member index in the state (278's omission and restore give
the same State, and no reader needs an index); failing the open of a store with a package found
nowhere (278's decision 4 has such a store open); the install invocation's `fetch` file as a
place (gone or unchecked by now; `conduit install` keeps its archives in the per-user cache); a
clone of the shared copy for each run (APFS `clonefile` or a reflink keeps per-run isolation, but
a copy per run is what DEC-148 removes).

**When it runs.** On `main` from `4749a0da` (#612), whether or not order 313 has merged.
Through Conduit: one Codex worker turn, the blind Codex verifier and Claude reader at once, one
fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what must hold;
how is yours. The coordinator's, open to the owner's strike (DEC-029): the places and their order,
that a package found nowhere stays and the store opens, the copy's name and modes, the stale
copy's handling, the messages' words and the test names. It fits one turn, about the size of
order 311's. If it does not, it splits by file and nowhere else: 312a is decisions 1 to 4 and 8
(`store.rs`, `lib.rs`, `main.rs`) with their tests and the DL7 edits of decision 10; 312b is
decisions 5 to 7 (`runner.rs`) with theirs and the DL16, paragraph and README edits. Do 312a
first, and report the part not done.

**Shared files.** Order 313 removes a run's leftovers and changes `runner.rs`, and may merge
first: read the code as it stands. Other orders change `engine/CONFORMANCE.md`; whichever merges
later rebases and reruns `promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first PRV-007, PKG-004, PKG-022, PKG-026, PKG-027 and IMP-007, and the rows "DL2: storage",
"DL7: storage and submission" and "DL16: packages and commands" of `engine/CONFORMANCE.md`; in the
shell, `store.rs` whole, `lib.rs` (`Config`, `Shell::open`, the keep before `persist`), `main.rs`
where it opens the store, `install.rs` (`Sources::read`), `user_config.rs` (`directory`,
`cache_archive`) and `runner.rs` (`Runner::new`, `command`, `materialize_package` and the tests
from `installed_command_state`); and `engine/crates/shell/tests/kept_archives.rs`.

### An earlier store's members move out

1. **The places** (DL7, PKG-026). For a package whose members `state.json` holds, its archive is
   looked for, in this order: the store's `packages/HEX`, HEX being the digest's 64 lowercase
   hex digits, as 278 names it; then `HEX` in each directory of a new `Config` member,
   `archive_places`, empty by default. `main.rs`, where it opens the store, sets it to the
   per-user package cache and the mirrors of the per-user `config.json`, as `Sources::read`
   finds them with no `--mirror`; a per-user directory that cannot be found, a `config.json` that
   cannot be read or parsed, or a mirror that is not a directory is left out and never fails the
   open. No test reads the per-user directory: each names its own places. A candidate counts only
   where it is a regular file, not a symbolic link (read with `symlink_metadata`), its SHA-256 is
   the package's digest, and the shared tar reader that `read_archive` uses unpacks it to members
   exactly equal to the ones `state.json` holds: each path, kind, mode, link target and file's
   bytes. Any other outcome at a place, among them metadata or bytes that cannot be read and a
   file of the digest that the reader refuses (not gzip, a truncated tar, a header not USTAR),
   is that place's outcome under decision 3, and the move tries the next place; none fails the
   open. A file of the package's digest that cannot be unpacked is the same bytes in every place,
   so the package then stays in `state.json`.
2. **The move** (DL2, PRV-007, IMP-007). At each open, under the lease, after `state.json` is read
   and before the authority, recovery or any commit, a function of `Store`'s, given the places,
   takes each package not yet kept, in the state's order: where a place holds a candidate, it
   keeps those bytes as `keep_archive` does, durably; then, if it kept any, it writes
   `state.json` once by `write_state`, which omits their members. Until that rename `state.json`
   is the earlier file, unchanged, so a move cut off at any point starts again at the next open,
   finding the archives it kept at `packages/HEX`. The State in memory is the same before and
   after, and memory equals disk. A keep or a write that fails fails the open with its error. The
   move writes no record and changes no object, invocation, allocated ID or output time; whether
   it adds a line to `commit-measures.jsonl` is yours. A store with no such package is not
   written by the move (`Shell::open`'s `recover` commit writes `state.json` at every open, as
   today; that write is not the move's). A move cut off after some of its keeps, or failing after
   the state's rename (its directory sync), leaves a store the next open reads and finishes.

   **One digest, every entry** (DL2, DL7). Omission is by digest (`kept_archives`,
   `write_state`), so a digest counts as kept only where every entry holding it has members
   exactly equal to its archive's. One function says so, used wherever a digest becomes kept: by
   the move, by `read_disk` for a store holding one entry of a digest without members and another
   with them, and by the install's keep. Where one entry of a digest differs, no entry of it
   moves; at open, a digest restored from `packages/HEX` whose inline entry differs is not
   counted kept, so every later write keeps every entry's members, the restored ones too. No
   entry's members are lost, and memory equals disk after each write.
3. **A package found nowhere** (ruling 52). It keeps its members in `state.json`, as 278's
   decision 4 has it, and the store opens. The move gives one line for each:
   `package ADDRESS VERSION (DIGEST) keeps its members in state.json: ` and then each place tried,
   its path and outcome, `absent`, `a symbolic link`, `not a regular file`, `unreadable: ERROR`,
   `digest found sha256:...`, `cannot be unpacked: ERROR`, `members differ at PATH`, with the
   first path that differs, or `held back: another entry of this digest differs at PATH`
   (decision 2). Where a place had the digest but it cannot be unpacked or an entry's members
   differ, the line ends: `no archive can move it; keep the store as it is and report it`;
   otherwise: `put the archive, named by its SHA-256 hex, in the package cache or a mirror, and
   open the store again`. The shell keeps the lines, readable by a public
   method of `Shell`'s; `conduit serve` writes each to standard error as it starts, after
   `conduit serve: `. Each later open tries again.
4. **One way** (DL2). After the move, an engine built before order 278 cannot read the store,
   since `InstalledPackage.files` has no default, as it already cannot read one holding a package
   278 kept. The format version stays 1, as 278 left it; every engine from 278 reads the store.

### One read-only program copy per digest

5. **The copy** (DL16, DEC-148). A run of an installed package's relative program runs it from
   `programs/HEX` in the store, one directory per digest, made from the package's members as
   today's `materialize_package` makes `programs/<uuid>`: built under a temporary name in
   `programs/` and renamed into place, its links resolving in `programs/HEX` (today's link is
   absolute into its root, which would point into the temporary name). It is read-only: each
   file has its member's mode without its write bits (`mode & 0o555`), and each directory, its
   root among them, `0o555`. Every run of that digest uses it; another digest has its own. A
   package whose digest is not `sha256:` and 64 lowercase hex digits fails the run, naming it.
   `platforms` and the program path are as today, under `programs/HEX`.
6. **Checked before each run** (DL16, DEC-148). Before each run that uses a copy, the runner
   checks it against the package's members: each path, its kind, a link's target, a file's bytes
   and decision 5's mode, and nothing more in the tree, following no link. Where the check fails,
   the copy is renamed to `programs/.stale-<uuid>`, never removed while a run may use it, and a
   new one is made; the run uses the new one. Where none can be made, the run fails as a failed
   materialization fails today, naming the package's address, version and digest. Making and
   checking a copy are serialized within the runner (as `cache_lock` serializes the npm cache;
   how is yours). The check holds between runs, not during one: decision 6 reads DEC-148's
   "checked before each run" so (DL16: "This is no OS sandbox").
7. **Leftovers** (DEC-148). No run's end removes `programs/HEX`. `Runner::new`, before any run is
   live, removes `programs/.stale-*` and the temporary names decision 5 builds under, making them
   writable first, unless a sweep at open already removes them. Where order 313 has merged (the
   start check's last count is 1), its removal at a run's end no longer applies to the shared
   copy; whether its sweep at open keeps or removes `programs/HEX` is yours, since the next run
   makes it again: report which. The `programs/<uuid>` copies earlier engines left are 313's.

### A failed keep, and the documents

8. **A failed keep leaves the store as it was** (DL2, PRV-007). No behaviour changes: decision 9's
   test guards today's. Where it fails on the unchanged code, fix it within Scope and report the
   failing line.
9. **Tests.** In a new `engine/crates/shell/src/package_store_tests.rs`, declared in `tests.rs` as
   `held_copy_tests.rs` is, with `tests.rs`'s helpers and runner tests' helpers made visible to it
   where needed. Write them first, run each on the otherwise unchanged code and report its failing
   line, or its first compiler error where it cannot compile there. Each builds its own archives
   (USTAR, gzip, as `kept_archives.rs` packs them) and writes an earlier store's form as
   `members_kept_in_state_json_before_order_278_still_load` does. A test that makes a read-only
   copy makes it writable again before its temporary directory is dropped, so none is left
   behind.
   - `an_earlier_stores_members_move_out_once` (DL2, DL7): an earlier store of two packages, one
     archive in a place the test names, the other at `packages/HEX` as a cut-off move leaves it.
     It opens: `state.json` holds neither package's `files`; both copies are 0600 in a 0700
     directory; `installed_packages` in memory equals the earlier file's; objects, records,
     invocations, allocated IDs and output times equal it; no line is given. Then, on the moved
     store, with the place removed, the move itself, called after `Store::open` and its read and
     before any recovery, with a `WriteFault::Write` armed on the state write: it returns with
     nothing moved, the fault is still armed (no state write began), and each `packages/` file
     keeps its inode and modification time (no keep). The `recover` commit's write at
     `Shell::open` is today's and is not asserted here. Today the members stay in
     `state.json`.
   - `a_package_found_nowhere_keeps_its_members_and_the_store_opens` (decision 3): three packages:
     one placed, one in no place, and one whose place holds an archive of its digest while the
     earlier `state.json` holds one member's bytes changed. The store opens; the first moves; the
     other two stay in `state.json`, each with decision 3's line naming the address, version,
     digest, each place and its outcome, `absent` and `members differ at PATH` among them. With
     the missing archive then placed, the next open moves it. Today none moves and no line exists.
   - `a_cut_off_move_leaves_state_json_as_it_was` (decision 2; one method, each case on its own
     store): an earlier store of two packages with their archives placed, the move called after
     `Store::open`:
     - a `WriteFault::Rename` on the state write: the move fails; `state.json`'s bytes are
       unchanged; both kept copies stay;
     - the second keep fails after the first is durable (a directory made at the second's
       `packages/HEX` makes its rename fail, or a test-only fault of yours): the move fails;
       `state.json`'s bytes are unchanged; only the first copy is kept; the directory is then
       removed;
     - a `WriteFault::DirectorySync`, after the state's rename: the move fails; `state.json`
       omits both packages' members.

     After each, the next `Shell::open` opens, with the places removed but, in the second case,
     the second package's: every member of both packages equals the earlier file's, in memory
     and as restored from `packages/`, and memory equals disk; objects, records, invocations,
     allocated IDs and output times equal the earlier file's; the open asks for no external work
     (no intent, no install or dispatch), and a further open gives no line. Today there is no
     move to call.
   - `a_shared_digest_moves_only_when_every_entry_matches` (decision 2): (a) an earlier store
     with two entries of one digest, one whose members equal the placed archive's and one with
     one member's bytes changed; (b) the same two, the matching one already without members and
     its archive at `packages/HEX`, the other inline. Each opens; then one commit that changes
     an unrelated object (yours) runs, and the store is opened again. In both, after each step,
     both entries' members in memory equal the earlier file's, the differing entry's members are
     in `state.json`, and memory equals disk; decision 3's line names the digest,
     `members differ at PATH` for one entry and `held back:` for the other. With the differing
     member made equal in the earlier file of (a), the next open moves both. Today (a) moves
     nothing, and (b)'s commit drops the differing entry's members from `state.json`.
   - `an_unusable_candidate_gives_way_to_a_later_one` (decisions 1 and 3): places in order, each
     holding `HEX` as a symbolic link to the valid archive, a directory, a file of another digest,
     and a file whose metadata or bytes cannot be read (mode 0, or a place directory without
     search permission); the valid archive is in a fifth place. The store opens and the package
     moves from the fifth, no line given. A second package whose digest is that of bytes that are
     not gzip, placed: the store opens, it stays in `state.json`, and its line gives
     `cannot be unpacked:` and the `no archive can move it` ending. Today none moves.
   - `runs_of_one_digest_share_one_read_only_copy` (decisions 5 and 6), in a runner as
     `relative_command_uses_installed_bytes_and_keeps_their_mode` makes one: two runs' programs of
     one package are in the same `programs/HEX`, which holds exactly the members, files at their
     modes less write bits and directories at 0555, and runs; a package of a second digest gets a
     second directory; `programs/` holds those two entries alone. Today each run gets its own
     `programs/<uuid>`.
   - `an_altered_program_copy_is_made_again_before_the_next_run` (decisions 6 and 7): after a
     run, the test does as a run's process could: gives a file and its directory write
     permission and changes the file's bytes, adds a file and removes another. The next run's
     program is in `programs/HEX`, whose tree equals the members again; the altered copy is under
     `programs/.stale-*`; a new `Runner` removes it. Today it fails at its first assertion that
     two runs share a directory.
   - `a_failed_keep_leaves_the_store_as_it_was` (decision 8), each on a new store after first
     start with no package installed, as `kept_archives.rs`'s `start` makes one (a store holding a
     kept package could not reread its state once `packages` is broken). With `packages` made a
     regular file before an install: through an `install-archive` input,
     the install fails, `state.json`'s bytes are unchanged, no package is listed, memory equals
     disk; through `management.install.v1` from a file URL, its reply, records and
     `installed_packages` equal those of the same install on another store with a
     `WriteFault::Write` injected in place of the broken `packages`, but for IDs, times and the
     error's text, and memory equals disk. With a `WriteFault::Rename` on the state write after a
     keep that succeeds: no package is listed, memory equals disk, and the same install then
     succeeds and the store reopens with it. It passes today too, a guard.
   - An existing test changes only where it builds an installed package with a digest that is not
     `sha256:` and 64 hex digits, or asserts a program copy's mode or its `programs/<uuid>`
     place: `relative_command_uses_installed_bytes_and_keeps_their_mode` (its 0o755 becomes
     0o555), `installed_relative_command_executes_through_parent_symlink_target` and
     `installed_bare_command_uses_permitted_path_not_package_files` among them; each that makes a
     copy may also gain the step that makes it writable before its directory is dropped. Report
     each.
10. **`engine/CONFORMANCE.md` and `engine/README.md`,** at their width. Quoted text below names
    links as `[text] (link to target)`, so the docs guard does not read them from this file;
    write each as a real Markdown link to that target where you insert it.
    - In the row "DL7: storage and submission", "a package installed before keeps its members
      there." becomes "a package installed before keeps its members there until an open finds its
      archive. From order 312, each open, before recovery, moves such a package's members out:
      where the store's `packages/HEX`, the per-user package cache or a mirror named in the
      per-user `config.json` holds a file of the package's digest whose members are those
      `state.json` holds, the engine keeps it as `packages/HEX` and then rewrites `state.json`
      without them, which an engine before order 278 cannot read; a package whose archive is found
      nowhere keeps its members, and `conduit serve` names it and each place tried as it starts
      (DEC-148)." Its evidence cell gains, after its last item,
      `, [package_store_tests.rs] (link to `crates/shell/src/package_store_tests.rs`)`.
    - In the unfinished row "DL7", "earlier installs keep theirs there." becomes "earlier installs
      keep theirs there until an open finds their archive (order 312)."
    - In the row "DL16: packages and commands", "Relative command paths are materialized from
      retained installed-package files under the store with their modes;" becomes "From order
      312, relative command paths run from one read-only copy of the package's files per digest,
      `programs/HEX` under the store, at their modes less write permission, checked against the
      package's members before each run and made again where the check fails;". Its evidence cell
      gains the same link as DL7's. Keep `From order 312,` on one line in each row.
    - In the paragraph "The archive catalog's virtual directory is store/archive-catalog/DIGEST;",
      "retained files stay in State and the runner materializes selected relative programs as
      before;" becomes "the runner materializes selected relative programs once per digest,
      read-only, as the row "DL16: packages and commands" says;".
    - In `engine/README.md`, "relative paths use retained installed-package files;" becomes
      "relative paths run from one read-only copy of the package's files per digest, checked
      before each run;".

    Then run `python3 tools/promise_standing.py --check`, and `--write` where it fails.

## Start check

From the repository root:

```sh
k() { f=$1; shift; for p in "$@"; do grep -cF -- "$p" "$f"; done | paste -sd' ' -; }
git status --short -- engine conformance spec tools | wc -l | tr -d ' '  # 0
grep -c '^## DEC-148, ' docs/decisions/log.md  # 1
sed -n '/^## DEC-148, /,/^## DEC-149, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
grep -c '^pub(crate) const VERSION: u32 = 1;$' engine/crates/shell/src/store.rs  # 1
k engine/crates/shell/src/store.rs 'pub fn keep_archive(' 'fn read_archive(' 'files: (!kept.contains(&entry.package.digest)).then_some(&entry.files),' 'self.kept_archives.borrow_mut().insert(digest.into());' 'kept.insert(entry.package.digest.clone());'  # 1 1 1 1 1
k engine/crates/shell/src/runner.rs 'fn materialize_package(' '.join(Uuid::new_v4().to_string());' '"npm", "npm-downloads", "work", "programs", "run-groups"'  # 1 2 1
cat engine/crates/shell/src/lib.rs engine/crates/shell/src/main.rs | grep -c 'archive_places'  # 0
grep -c 'let recovered = shell.commit(recover)?;' engine/crates/shell/src/lib.rs  # 1
k engine/crates/shell/tests/kept_archives.rs 'fn members_kept_in_state_json_before_order_278_still_load' 'fn an_archive_directory_with_special_mode_bits_cannot_receive_an_install'  # 1 1
k engine/CONFORMANCE.md 'From order 312,' 'a package installed before keeps its members there.' 'Relative command paths are materialized from retained installed-package files under the store with their modes;' 'earlier installs keep theirs there.' 'retained files stay'  # 0 1 1 1 1
grep -c 'relative paths use retained installed-package files;' engine/README.md  # 1
test -e engine/crates/shell/src/package_store_tests.rs; echo "new $?"  # new 1
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"  # engine 0
git log --oneline -E --grep='^Work order 313' HEAD | wc -l | tr -d ' '  # 0 or 1: 1 means order 313 has merged (decision 7)
```

Where a line prints otherwise, stop, change nothing and report it. A sixth line other than `1 2 1`
means the runner's copy has moved, as order 313 may move it: read it as it stands, and stop only
where `materialize_package` or its caller is gone.

## Scope

Change, in the shell, `store.rs` and `lib.rs` for decisions 1 to 4 and 8, `main.rs` for decisions
1 and 3, `runner.rs` for decisions 5 to 7 and its tests under decision 9, `tests.rs` (its `mod`
line and visibility of its helpers) and `package_store_tests.rs` (new); `engine/CONFORMANCE.md`'s
rows and paragraph and `engine/README.md`'s sentence under decision 10; and `engine/STANDING.md`
by the tool alone. Change an existing test only under decision 9. Change nothing else: not
`spec/`, `conformance/`, `docs/` (DEC-148 stays not done), the core, the shell's other sources,
`engine/crates/shell/tests/`, the loader, the conformance crate, `packages/`, `tools/`, a
`Cargo.toml` or `Cargo.lock`. No promise changes, so `spec/CHANGES.md` gains no line. Report
anything else that needs changing.

## Done when

The first group prints otherwise today and passes after; the second, guards, passes today and
after. Run each existing test you changed by name, once, too. Run the vectors test once, last;
never the whole engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in an_earlier_stores_members_move_out_once a_package_found_nowhere_keeps_its_members_and_the_store_opens a_cut_off_move_leaves_state_json_as_it_was a_shared_digest_moves_only_when_every_entry_matches an_unusable_candidate_gives_way_to_a_later_one runs_of_one_digest_share_one_read_only_copy an_altered_program_copy_is_made_again_before_the_next_run; do (cd engine && cargo test --offline --release -p conduit-shell --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 seven times; 0 today
grep -c 'From order 312,' engine/CONFORMANCE.md  # 2; 0 today
grep -c 'until an open finds their archive (order 312)' engine/CONFORMANCE.md  # 1; 0 today
cat engine/CONFORMANCE.md engine/README.md | grep -c 'retained installed-package files'  # 0; 2 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-shell a_failed_keep_leaves_the_store_as_it_was 1  # 1, once written; on the unchanged code too
s conduit-shell memory_equals_disk_after 5  # 1
s conduit-shell killed_after_ 2  # 1
s conduit-shell held_copy_ 3  # 1
s conduit-shell commit_measures_one_line_per_commit 1  # 1
(cd engine && cargo test --offline --release -p conduit-shell --lib runner:: 2>&1 | grep -c '^test result: ok\.')  # 1
for t in kept_archives local_process provider_calls install_closure; do (cd engine && cargo test --offline --release -p conduit-shell --test "$t" 2>&1 | grep -c '^test result: ok\.'); done  # 1 four times
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. archives- 2>&1 | tail -1)  # the same "N cases passed" as on the unchanged code, run first
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
n=engine/crates/shell/src/package_store_tests.rs; git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, twice
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/shell/src/(store|lib|main|runner|tests|package_store_tests)\.rs|engine/(CONFORMANCE|STANDING|README)\.md)$' | wc -l | tr -d ' '  # 0
git reset -q -- $n  # stage nothing
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors 2>&1 | grep -c '^test result: ok\.')  # 1: every vector passes, once, last
```

Your report gives, per decision, what changed and where; each new test's failing line on the
unchanged code (for the guard, its passing line there), then its passing line; the lines decision
3 gives in the second test; whether order 313 had merged, and what decision 7 did with its sweep;
if the work split, which part is done; each existing test you changed, and why; your own
choices; and every check's last line. Stage nothing.

## What follows

- **DEC-148's run leftovers**, order 313, if it has not merged: it removes a run's directories at
  its end and sweeps at open, and leaves `packages/` and the shared `programs/HEX` to this order.
- **The dogfood store** (DEC-143) is moved at its first open under this order, one way, so it
  opens with this engine only at a stage boundary or by a dated amendment: before it, the
  coordinator stops `serve`, copies the store directory with its modes, and checks that each of
  its six installed digests is in the per-user package cache or a mirror; after it, reads the
  open's lines and `stateBytes` in `commit-measures.jsonl`, by numbers.
- **DEC-148's "the state keeps an index"**: 278 omits a kept package's members and restores them
  from the copy, so the State in memory still holds every member's bytes. Whether that meets the
  owner's words, or the State should keep an index and the runner read the copy, is the
  coordinator's to put to the owner.
- DEC-148's and DEC-149's Status are the coordinator's to set once their orders merge.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` at its start. Redrafted after
order 278 merged and built and read on the team seats (DEC-179, DEC-180, DEC-203).

**The draft.** The drafter, `root-drafter.turn.v1`, on the Team seat, redrafted it to keep only
what 278 lacks; verifier-5 checked it adversarially (one must: one digest's members are kept only
where every entry holding it matches its archive) and the drafter revised it.

**Worker-5's turns**, on the Business seat, the fifth pair's first order: 312a (the one-way move of
an older store, the shared-digest rule, the failed-keep coverage), then 312b.

**Two blind reads.** The Codex verifier, `root-verifier-5.turn.v1`, ran out of its hour on the
first attempt under the machine's load and ended unknown; its second attempt, with targeted tests,
found no production defect and one should (the new package tests failed intermittently at the
default concurrency). The Claude reader, `root-reader-a.turn.v1`, found the decisions applied,
with four shoulds and three nits, most on what an operator is told when the move or the cleanup
fails. One fix turn by worker-5 answered all of them. The gate's run on the squash is the
coordinator's, at the merge.
