# Work order 276: one tar reader in the shell, for install's dependency order and its metadata read alike (order 263's finding)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no conformance case changes. This order
needs no network: it reaches no host and fetches no crate. Every crate it builds is in the local
cache already: run cargo offline, and if a build asks for a crate that is not there, stop and
report, and fetch nothing. Run one cargo command at a time. Never print a whole large file,
case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Do not run `conduit` outside a test, `npm`, `node`,
`codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each
`python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user or machine name, or private repository name.

## What and why

**The finding.** Order [263](263-install-closure-and-exact-pins.md)'s Claude reader found two
tar readers in the shell, left to a later order. Read on `main` at `d5958765`:

- *The metadata path*: `archive_metadata.rs`'s `unpack` and `resolve`, a copy of the core's
  admission reader in `engine/crates/core/src/archives.rs`, equal from `type ArchiveResult` to
  the end of `unpack`. `boundary::install_manifest` reads with it every archive that
  `conduit install` is given or finds, as `install_package` (`--package`) and
  `named_install_arguments` do.
- *The install path*: `install/archive.rs`'s `authored_members`, with its own `Member`, `resolve`
  and `components`. `dependencies` reads with it the written order of a manifest's `packages`,
  when it lists two or more, for `Candidate::open`, from `Sources::named` and `Sources::find`.

Two parsers of one format can disagree on one archive; the two paths' tar reads must accept and
refuse the same archives. Today no command shows a difference: both callers of
`Candidate::open` run `install_manifest` on the same bytes first, so the install path reads only
what the metadata path accepted, and the core admits each archive again with its own copy. Each
difference below is a refusal that the install path's reader would miss if it ever read first.

**The differences**, read from both readers; the test settles each. Each row's archive has a
root `conduit.yaml` that lists two packages, so the install path reads it. "Refuses" is the
metadata path's error, `PKG-022: ...` or `MAN-014: ...`; the install path's errors carry no
rule. "Kept, by" names what keeps the metadata path's behaviour: a promise; the standards that
[the schema profile](../../spec/schema-profile.md) names for archives, POSIX tar and RFC 1952
gzip; or DL7, which leaves the tar formats and extensions to the engine, as the DL7 rows of
[`engine/CONFORMANCE.md`](../../engine/CONFORMANCE.md) declare them.

| # | The archive | Metadata path | Install path | Kept, by |
| --- | --- | --- | --- | --- |
| 1 | The gzip member's CRC or length is wrong | refuses, PKG-022 | reads it: it stops at the tar's first zero block, before the gzip trailer | metadata; RFC 1952, PKG-022 |
| 2 | Bytes after the last gzip member that are no gzip member | refuses, PKG-022 | never reads them | metadata; RFC 1952, PKG-022 |
| 3 | The tar ends after a member with no zero block, or one | refuses, PKG-022 | reads it | metadata; POSIX ends an archive with two zero blocks |
| 4 | After the two zero blocks, a nonzero byte or a partial block | refuses, PKG-022 | ignores it | metadata; DL7, as the core reads it |
| 5 | A member's data runs past the end | refuses, PKG-022 | the tar crate's error | metadata; POSIX, PKG-022 |
| 6 | A header that is not USTAR: v7, or GNU's magic | refuses, PKG-022 | reads it | metadata; DL7: "POSIX USTAR, without GNU/PAX extensions" |
| 7 | A PAX (`x`, `g`) or GNU (`L`, `K`) extension entry | refuses, PKG-022 | skips it | metadata; DL7, as row 6 |
| 8 | A FIFO, device or contiguous-file entry | refuses, PKG-022 | skips it | metadata; DL7: files, directories, symbolic links and hard links |
| 9 | A regular file's header whose mode is not octal | refuses, PKG-022 | ignores modes | metadata; POSIX; DL7 keeps each file's mode |
| 10 | A path that is not UTF-8 | refuses, PKG-022 | refuses, no rule | metadata; DL7: "UTF-8 paths" |
| 11 | A member path that is absolute or has a `..` segment | refuses, MAN-014 | drops the `/`, applies `..`, refuses only above the root | metadata; MAN-014 |
| 12 | A symbolic link, absolute or leading outside, that nothing reads through | refuses, MAN-014 | reads it | metadata; MAN-014 |
| 13 | A hard link to an absolute path | refuses, MAN-014 | drops the `/` | metadata; MAN-014 |
| 14 | A link cycle that nothing reads through | refuses, PKG-022 | reads it | metadata; DL7: "All paths/targets are checked" |
| 15 | 41 link expansions from `conduit.yaml`, and 40 | refuses 41, PKG-022; reads 40 | the same, in its own words | metadata; DL7: "At most 40 link expansions" |
| 16 | `conduit.yaml` links through a missing component, or a file, before `..` | refuses, PKG-022 | applies `..` by name and reads the target | metadata; MAN-011 and SUB-010: a link that leads nowhere |
| 17 | A member below a file | refuses, PKG-022 | reads it | metadata; DL7: "a non-directory parent" |
| 18 | A non-directory where a directory has members | refuses, PKG-022 | replaces it | metadata; DL7: "replacing a nonempty directory" |
| 19 | A member `.` that is not a directory | refuses, PKG-022 | keeps it at the empty path | metadata; DL7, as row 17: the root is every member's parent |
| 20 | A well-formed archive whose manifest lists `example.org/b` before `example.org/a` | reads it | reads it, in that order | both |

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): none
at risk. No command accepts, refuses or prints anything new, and no promise, case or core source
changes.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: one parser of a format where a decision rests on it. Android's
"Master Key" bug (CVE-2013-4787) was two readers of one APK, the signature check and the
installer, taking different entries of the same name. Not taken: two readers, the second given
the first's checks (two parsers still); the core's reader made public so that the shell keeps no
copy (a change to `conduit-core`'s public surface; "What follows" names it); a new reader (a
third).

**When it runs.** On `main` from `d5958765`. Through Conduit's dogfood: one Codex worker turn
(4-hour limit; aim for under an hour), a blind Codex verifier and Claude reader, one fix turn,
the gate (DEC-046, DEC-033). By ruling 89, each decision says what must hold; how is yours. The
table's readings, row 4's among them, and the test's name are the coordinator's, open to the
owner's strike.

**Stop rule.** Stop, change nothing further, and report where:

- the test shows a difference that no row lists, or a row on which the install path refuses what
  the metadata path accepts, or reads another manifest or order: give the archive and both
  results; choose nothing;
- keeping the metadata path's behaviour needs a change to what it, the core or a command
  accepts, refuses or prints;
- an existing test fails, or would need changing;
- the copied region of `archive_metadata.rs` cannot stay the core's, but for visibility;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **One reader** (DL7, PKG-022, MAN-014). The shell reads a package archive's members with one
   reader, the `unpack` and `resolve` of `archive_metadata.rs`, whose copied region stays the
   core's but for visibility. `install/archive.rs` keeps `dependencies` and its written-order
   read, and loses its own reader: `Member`, `authored_members`, `resolve` and `components`. Its
   read of each of `MANIFEST_NAMES` resolves through the shared `resolve`. One way: reach
   `archive_metadata` from `install` as `pub(crate)`; another, `install_manifest` returns the
   members beside the manifest, so each archive is read once. Yours.
2. **What each path does** (the table). On every row the install path does what the metadata
   path does today: it accepts what that accepts, and refuses what that refuses, with an error
   of the same `RULE: message` form and words. `dependencies` still reads nothing when the
   manifest lists fewer than two packages, and its error "cannot determine the archive
   manifest's written order" stands. Nothing that the metadata path, `install_package`, the core
   or a command prints changes.
3. **Tests.** Write this first and run it on the unchanged code; report each row's two results
   and the failing line.
   - `install_and_metadata_paths_read_archives_alike`, in `install/archive.rs`'s tests: one case
     per row of the table, and two for row 15, at 41 and 40 expansions. Each archive's
     root `conduit.yaml` lists `example.org/b` then `example.org/a`, each with a version and a
     well-formed digest. Each is given to `crate::boundary::install_manifest` and to
     `dependencies`, with the JSON its manifest holds: both accept, `dependencies` giving
     `example.org/b` then `example.org/a`; or both refuse, each error beginning with the row's
     rule and `: `. Write a header that the tar crate's builder refuses (an absolute or `..`
     path, a mode that is not octal, v7 or GNU magic) into the header's bytes, as
     `engine/crates/conformance/examples/make_archives.rs` writes its traversal fixtures, with
     the checksum set last. Today it fails on each row where the paths differ.
   - No other test changes.

## What to read

- PKG-022 and PKG-023 in `spec/promises/packages.yaml`; MAN-011, MAN-013 and MAN-014 in
  `spec/promises/manifest.yaml`; SUB-010; DL7 in `spec/implementation-defined.yaml`; the
  "Package archives" row of `spec/schema-profile.md`; the DL7 rows of `engine/CONFORMANCE.md`;
  `conformance/fixtures/install-archives/README.md`.
- In `engine/crates/shell/src/`: `install/archive.rs`, `archive_metadata.rs`, `boundary.rs`
  (`install_manifest`, `install_package`, `named_install_arguments`) and `install.rs`
  (`Candidate::open`, `Sources::named`, `Sources::find`); `unpack` in
  `engine/crates/core/src/archives.rs`; `make_archives.rs`'s header writer;
  `engine/crates/shell/tests/install_closure.rs`, by test with `sed -n`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor d5958765 HEAD; echo "main $?"
s=engine/crates/shell/src
grep -cE '^(enum Member \{|fn (authored_members|resolve|components)\()' $s/install/archive.rs
grep -cF 'archive::dependencies(&bytes, manifest)?' $s/install.rs
grep -c 'boundary::install_manifest(' $s/install.rs
git grep -l 'tar::Archive::new' -- $s | wc -l | tr -d ' '
r() { sed -n '/^type ArchiveResult/,/^    Ok(files)$/p' "$1" | sed -E 's/^pub(\([a-z]+\))? //' | cksum; }
test "$(r engine/crates/core/src/archives.rs)" = "$(r $s/archive_metadata.rs)"; echo "copy $?"
git grep -l 'fn install_and_metadata_paths_read_archives_alike' -- engine | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `4`, `1`, `2`, `3`, `copy 0`, `0`, `engine 0` and `0`.

## Scope

Change `engine/crates/shell/src/install/archive.rs` for decisions 1 to 3;
`archive_metadata.rs` only in visibility, its header comment and what lies outside its copied
region; `boundary.rs` and `install.rs` only where decision 1 reaches the reader or hands it the
members; and `engine/STANDING.md` only by the tool. Change nothing else: not the core (its
`archives.rs` among it), the loader, `runner.rs` (its npm unpack reads npm's tarball, not a
package archive), `spec/`, `conformance/`, `docs/`, the shell's integration tests, `Cargo.toml`
or `Cargo.lock`. Report anything else that needs changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last line, the whole engine suite,
once, last.

```sh
ok='^test result: ok\. 1 passed'
s=engine/crates/shell/src
(cd engine && cargo test --offline --release -p conduit-shell --lib install_and_metadata_paths_read_archives_alike 2>&1 | grep -c "$ok")  # 1; today 0
grep -cE '^(enum Member \{|fn (authored_members|resolve|components)\()' $s/install/archive.rs  # 0; today 4
git grep -l 'tar::Archive::new' -- $s | wc -l | tr -d ' '  # 2, archive_metadata.rs and runner.rs; today 3
```

```sh
s=engine/crates/shell/src
r() { sed -n '/^type ArchiveResult/,/^    Ok(files)$/p' "$1" | sed -E 's/^pub(\([a-z]+\))? //' | cksum; }
test "$(r engine/crates/core/src/archives.rs)" = "$(r $s/archive_metadata.rs)"; echo "copy $?"  # copy 0
(cd engine && cargo test --offline --release -p conduit-shell --test install_closure 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^engine/(crates/shell/src/(install/archive|archive_metadata|boundary|install)\.rs|STANDING\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Your report gives, per decision, what changed and where; the test's row results on the unchanged
code and its failing line, then its passing line; your own choices; and every check's last line.

Stage nothing. The verifier's turn follows; then the gate, a read-only Claude reader, and the
merge.

## What follows

Named here and not in this order:

- The shell's reader stays a copy of the core's, kept equal by the guard above. The core's
  `unpack` and `resolve` made public, so that the shell holds no copy, is a later order's.
- Found while writing: the metadata path refuses a root manifest name that leads nowhere (row 16)
  as PKG-022, where the core's admission reports SUB-010 and MAN-013, as the case
  `archives-manifest-missing-parent` has it; `conduit install` prints the metadata path's. Its
  words are outside this order, which changes nothing printed.
- DL7's row in `engine/CONFORMANCE.md` does not say that the engine refuses bytes after the two
  zero blocks (row 4); it could.
- The paths still differ after the tar read, in which manifest they take:
  `archive_metadata::manifest` refuses two root manifests, or one of several documents, where
  `dependencies` takes the first of `MANIFEST_NAMES` and the first document. A later order's;
  decision 1's second way, `install_manifest` returning the members, would carry it.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main`.

**Worker-2's turn** (08:29 to 08:38) made `install/archive.rs` read through
`archive_metadata.rs`'s `unpack` and `resolve`, now `pub(crate)`, and removed its own `Member`,
`authored_members`, `resolve` and `components`, so the shell has one tar reader; dependency
ordering keeps its written-order read and gives the metadata reader's exact refusals. The table
test fed the same 21 archives to both callers, failing first where the two readers differed; it
passes, with the 11 install-closure tests and nine targeted CLI tests. The shared reader matches
the core's apart from visibility.

**Two blind reads at once** (08:39 to 08:45): `turn-verifier-2.turn.v1` and the Claude reader
both passed it with no findings. The gate's run on the squash is the coordinator's, at the merge.
