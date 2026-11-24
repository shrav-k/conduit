# Work order 278: the engine keeps each installed archive as distributed, outside `state.json`, and checks `platforms` at install (DEC-049, the engine's first part)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; list under
`vectors` the two cases you add, with the promises each tests. This order needs no network: it
reaches no host and fetches no crate. Every crate it builds is in the local cache already: run
cargo offline, and if a build asks for a crate that is not there, stop and report, and fetch
nothing. Run one cargo command at a time, and not `tools/gate.py`. Never print a whole large
file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Run no `conduit` outside a test, no
`npm`, `node`, `codex` or `claude`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user or
machine name, or private repository name.

## What and why

**The owner's answer** ([DEC-049](../decisions/log.md#dec-049-2026-11-21-per-platform-programs-in-one-archive)):
per-platform programs in one archive. Order 274, stacked under this one, wrote the spec. Its
engine side does not fit one worker turn, so it splits in two: this order, the install side, and
a second order, the run side ("What follows").

**Today**, read on `main` at `8dfec047` with order 274 stacked:

1. The engine keeps an installed package as `InstalledPackage.files`
   (`engine/crates/core/src/archives.rs`), its unpacked members, each `Member::File` holding its
   `bytes`. The shell writes the whole State to `state.json` on every commit (`store.rs`,
   `write_state`), each of those bytes a JSON number. The archive's own bytes are dropped after
   `unpack`; repacking the members would not give the bytes PKG-004's digest is over. PRV-007,
   as 274 wrote it, has the engine keep each installed archive exactly as distributed.
2. 274's schema admits `platforms` (MAN-009) and rejects its malformed forms. Nothing checks that
   the program beside it is a relative path (MAN-010), or that an archive holds each listed
   pair's program (MAN-013).
3. `conduit install` keeps each archive in the person's cache as `packages/HEX` (K3's O7);
   order 263's mirrors use that layout.

**The coordinator's decisions**, open to the owner's strike: one copy per package, its members'
source, in the store's `packages/HEX`, the cache's layout; none of their bytes in `state.json`;
a missing or altered copy stops the store opening; older stores keep their members.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, a
shared thing is owned once: one copy of each archive, the source of its members now and of what
a provider carries in the second order; 6, nothing repeats an effect implicitly: restoring reads
a copy and installs nothing. None at risk: no promise or record changes; replies change only by
decisions 5 and 6's rejections.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: Go's module cache keeps each module's `.zip` as downloaded beside its
extracted tree, checks it against `go.sum`, and serves as a proxy over `file://`; containerd's
content store and the OCI image layout keep each blob as one file named by its digest. Not taken:
members in `state.json` beside the copy (each byte a JSON number, rewritten each commit); an
unpacked tree for the copy (two sources; repacking changes the digest); members as files by
digest (a second layout); a v0 removal command.

**When it runs.** On `main` from `8dfec047` (#550), with order 274 stacked before its merge.
Through Conduit: one Codex worker turn (a 4-hour limit; aim for under 2 hours), the blind Codex
verifier and Claude reader at once, one fix turn for both, the gate (DEC-046, DEC-050, DEC-033).
By ruling 89, each decision says what must hold; how is yours. The words, the copy's place and the
test and case names are the coordinator's, open to the owner's strike.

**Shared files.** Order 272 changes `engine/CONFORMANCE.md`; 276 changes the shell's
`archive_metadata.rs`, `boundary.rs` and `install/archive.rs`, which this order leaves alone.
The later to merge rebases and reruns `promise_standing.py --write`.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- a wording needs a choice that this order and `spec/` do not make: give the options;
- a correct result needs a change outside the scope: the loader, `engine/crates/conformance/src/`,
  `runner.rs`, an existing case, `packages/`, `tools/`, a `Cargo.toml` or a lockfile;
- an existing case or test fails, or `vector_delta.py` shows a change beyond the two added cases:
  give it, with the expected and observed values;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

### The kept copy

1. **The copy** (PRV-007, PKG-004, DL7). Where the engine installs a package (`changed` `true`),
   by `install.v1` or by `register --package`'s `install-archive` input, the store holds,
   durably and before the commit that lists the package, the file `packages/HEX` in the store's
   directory, HEX being the digest's 64 lowercase hex digits: the archive's bytes exactly as the
   install verified them (PKG-026), mode 0600, in a directory of mode 0700: the person's cache's
   layout, one file per digest. A rejected install and a repeat (PKG-027) write none. A copy a
   failed commit leaves may remain; the next install rewrites it.
2. **No package bytes in `state.json`** (DL2, DL7). For a package with a copy, `state.json` holds
   none of its members' bytes. Whenever the shell reads its state (`Store::read`, at open and on
   DL6's retries), it restores them from the copy with the core's `unpack`, which made them at
   install (not `archive_metadata.rs`'s copy of it), so the State in memory, `files` with bytes
   and modes, is as today: the runner, the core and every case see no difference. Every reader
   of `state.json` still reads it, `author::state` (the install preflight's `State`) among them.
3. **A missing or altered copy fails closed** (DL2; ruling 52). Where a package's copy is absent,
   or its SHA-256 is not the package's digest, the store does not open, as for a `state.json` it
   cannot read. The error names the package's address and version, the copy's path, and `absent`
   or the digest found beside the one expected. A copy that is not a regular file of mode 0600
   stops it too, its error giving the kind or mode found. Putting the archive back, as from the
   person's cache, opens it again. Nothing repairs or removes a copy.
4. **Stores written before this order** (DL2). A `state.json` that holds a package's members, as
   every store written before this order does, opens with them, and each later commit writes them
   back there unchanged; the engine makes no copy for such a package. Whether the store's format
   version changes is yours, provided such a store opens.

### `platforms` at install

5. **MAN-010's program** (MAN-009, MAN-010). An implementation item, of a package or a submitted
   project, that states `platforms` beside a `command` whose program is not a relative path (it
   begins with `/`, or holds no `/`) is rejected with one error: `ruleId` `MAN-010`, `level`
   `error`, `message.text` exactly `NAME: platforms is stated beside PROGRAM, which is not a
   relative path`, NAME being the item's name and PROGRAM its program as written. The other
   forms MAN-010 rejects (beside `package` or `remote`, empty, a pair twice, not `<os>/<arch>`)
   are the schema's, as 274 wrote it.
6. **MAN-013's programs** (MAN-013, PKG-023). Installing an archive is rejected, `reason`
   `invalid`, where an item of its manifest states `platforms` beside a relative program and,
   for a listed pair, the archive holds at `platforms/<os>/<arch>/` followed by that program, its
   links followed as `resolve` reads a member, no regular file whose mode has the owner's execute
   bit (0o100). Each such pair gives one error, items in name order and pairs as listed:
   `ruleId` `MAN-013`, `level` `error`, `message.text` exactly `ARCHIVE: NAME lists platform
   PAIR, and PATH is not an executable regular file in the archive`, ARCHIVE being the archive's
   name as other archive diagnostics give it, NAME the item's, PATH the path above. An archive
   holding every listed program installs as before.

### Tests, cases and documents

7. **Tests.** Write these first and run them on the unchanged code; report each failing line.
   Place them in `engine/crates/shell/` (the module is yours).
   - `install_keeps_the_archive_outside_state_json`: a `management.install.v1` through a `Shell`
     on a new store, as `t5_public_install_is_live_and_existing_instances_keep_versions` sends
     one, of an archive built in the test that holds a 65,536-byte member: it succeeds;
     `packages/HEX` holds exactly the archive's bytes, mode 0600; `state.json` grew by fewer than
     65,536 bytes; reopened, the store gives the same `installed_packages`, bytes and modes. The
     same on a second new store by an `install-archive` input.
     Today there is no copy, and `state.json` grows by more than 65,536 bytes.
   - `a_missing_or_altered_kept_archive_stops_the_store_opening`: after that install, with one
     byte of the copy changed, opening the store fails, its error holding the package's address,
     the copy's path and both digests; with the copy removed, the path and `absent`; with the
     archive's bytes put back, it opens. Today it fails at the copy's first read.
   - `members_kept_in_state_json_before_order_278_still_load`, a guard passing today too: a
     store whose `state.json` holds a package's members, as today's engine writes them, opens
     with them, the next commit writes them unchanged, and no copy appears.
   - An existing test changes only where it asserts what decisions 1 to 6 change: a member's
     bytes read from `state.json`. Change no other test.
8. **Two portable cases** (from `{"recipe":"empty"}`), each with its entry in its directory's
   `index.yaml`; take the form from `sectioned-archive-install`, and for resolving a submitted
   project from `contracts-secret-store-only`. Each install input gives a `directory` file URL,
   as `registration-agent-v2-packages` does.
   - `archives-platform-programs`, in `conformance/language/archives/platform-programs/`,
     promises `MAN-013`, `MAN-009` and `PKG-023`. Three archives by `make_archives.rs`, which
     gains a member mode of 0o755 and keeps every existing fixture's bytes, each manifest's one
     item `command: [bin/server]` with `platforms: [darwin/arm64, linux/arm64]`:
     `platforms-missing.tar.gz` holds only the `darwin/arm64` program, `platforms-not-executable`
     both with the `linux/arm64` one at 0o644, `platforms-complete` both at 0o755. The first two
     installs are rejected, each with one MAN-013 error for `linux/arm64` with decision 6's text;
     the third is accepted and lists the package with both programs at mode 493. Record their
     digests in `digests.json`, and a paragraph in the fixtures' README.
   - `packages-platforms-program`, in `conformance/language/packages/platforms-program/`,
     promises `MAN-010` and `MAN-009`. One `resolve-project` per form, each rejected with a
     diagnostic naming the item: `platforms` beside the program `/usr/bin/server` and beside the
     program name `server`, each with decision 5's error; beside `package`; empty; `linux/arm64`
     twice; `linux-arm64`.

   Run each case on the unchanged engine first and report its failing step: the first two
   installs, and the first two resolves, are accepted today. Declare both in
   `docs/work-orders/278-declared.yaml`, in the form of [256's](256-declared.yaml), and run
   `python3 tools/promise_standing.py --write`.
9. **`engine/CONFORMANCE.md`.** At the end of the row "DL7: storage and submission": "From order
   278, the engine keeps each archive it installs exactly as distributed, as `packages/HEX` in
   its store, restores the package's members from it when it reads
   its state, and writes none of their bytes to `state.json`; a package installed before keeps
   its members there." In the DL7 row of the unfinished items, "installed bytes and virtual
   files are persisted in State." becomes "From order 278, installed archives are kept beside
   `state.json`, which holds none of their members' bytes; earlier installs keep theirs there."
   Keep `From order 278,` on one line in each.

## What to read

MAN-009, MAN-010, MAN-013, PRV-007, PKG-004, PKG-023, PKG-026, PKG-027; DL2, DL6 and DL7;
`implementation-item.v1`; K3's O7. Core `archives.rs` (`InstalledPackage`,
`install_with_confirmation`, `unpack`, `resolve`) and `packages.rs` (`expand_manifest`). Shell
`store.rs` (`read`, `load`, `write_state`), `lib.rs` (the archive completion, `persist`),
`author.rs` (`state`), `install.rs` (`preflight`), `user_config.rs` (`cache_archive`) and
`runner.rs`'s `materialize_package`, the members' reader.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 8dfec047 HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/manifest.yaml 'the item, the pair and the path.'
j spec/promises/manifest.yaml '`platforms` beside `package` or `remote`'
j spec/promises/providers.yaml 'as long as the package is installed'
grep -c 'platforms: \[command\]' spec/schemas/manifest.yaml
grep -cF 'pub files: BTreeMap<String, Member>,' engine/crates/core/src/archives.rs
grep -cF 'fn unpack(bytes: &[u8])' engine/crates/core/src/archives.rs
grep -cF 'with bytes/modes in `State.installed_packages`' engine/CONFORMANCE.md
grep -cF 'installed bytes and virtual files are persisted in State.' engine/CONFORMANCE.md
grep -c 'From order 278,' engine/CONFORMANCE.md
test -e conformance/language/archives/platform-programs || test -e conformance/language/packages/platforms-program; echo "cases $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, then `1` eight times, `0`, `cases 1`, `engine 0` and `0`. A `0` among the
first four counts means order 274 is not stacked: stop.

## Scope

Change, for decisions 1 to 6, `engine/crates/core/src/archives.rs`, `packages.rs` and `lib.rs`,
and `engine/crates/shell/src/store.rs`, `lib.rs`, `author.rs` and `install.rs`; for decision 7,
the test modules of `engine/crates/shell/`, a new test file there among them; for decision 8,
`make_archives.rs`, `conformance/fixtures/install-archives/` (the three archives, `digests.json`,
`README.md`), the two case directories, `conformance/language/archives/index.yaml` and
`conformance/language/packages/index.yaml`, and `docs/work-orders/278-declared.yaml`; for decision
9, `engine/CONFORMANCE.md`; and `engine/STANDING.md` only by the tool. Change nothing else: not
`spec/`, another case, other `docs/`, `packages/`, `tools/`, the loader,
`engine/crates/conformance/src/`, `runner.rs`, `Cargo.toml` or `Cargo.lock`. Report what else
needs changing.

## Done when

Each line of the first group prints otherwise on `main` with 274 stacked ("today") and passes
after. The second group are guards, which pass today and must still pass; run its last cargo
line, the whole engine suite, once, last.

```sh
ok='^test result: ok\. 1 passed'
(cd engine && cargo test --offline --release -p conduit-shell install_keeps_the_archive_outside_state_json 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-shell a_missing_or_altered_kept_archive_stops_the_store_opening 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. archives-platform-programs 2>&1 | tail -1)  # 1 cases passed; today it ends no cases matched the requested prefix
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. packages-platforms-program 2>&1 | tail -1)  # 1 cases passed; today the same
grep -c 'From order 278,' engine/CONFORMANCE.md  # 2; today 0
test -f docs/work-orders/278-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n="conformance/language/archives/platform-programs conformance/language/packages/platforms-program docs/work-orders/278-declared.yaml"
git add -N $n conformance/fixtures/install-archives  # so the checks see the new files; add any new test file too
(cd engine && cargo test --offline --release -p conduit-shell members_kept_in_state_json_before_order_278_still_load 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1
(cd engine && cargo test --offline --release -p conduit-shell --test install_closure 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo run --offline --release -q -p conduit-conformance --example make_archives -- --check; echo "archives $?")  # archives 0
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell -p conduit-conformance --all-targets -- -D warnings)
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/278-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(archives|packages|lib)\.rs|engine/crates/shell/(src/(store|lib|author|install|tests)\.rs|tests/[a-z_]+\.rs)|engine/crates/conformance/examples/make_archives\.rs|conformance/fixtures/install-archives/(platforms-[a-z-]+\.tar\.gz|digests\.json|README\.md)|conformance/language/(archives|packages)/(index\.yaml|platform-programs/.+|platforms-program/.+)|docs/work-orders/278-declared\.yaml|engine/(CONFORMANCE|STANDING)\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- $n conformance/fixtures/install-archives  # stage nothing
```

Report, per decision, what changed and where; each new test's and case's failing line on the
unchanged code, then its passing line; `state.json`'s growth in the first test, before and after;
each existing test you changed, and why; and every check's last line.

Stage nothing. The two blind reads follow, then one fix turn, the gate and the merge.

## What follows

- **DEC-049's second engine order**, the run side: placement only on a node whose
  `kubernetes.io/os` and `kubernetes.io/arch` are a listed pair (PLC-007, PKG-019); the pair's
  program on the engine's own runner (PRV-007, MAN-010); PRV-001's rejection of a Type listing
  `run.v2` without `runtime-provider.v1`; `run.v2`, never `run.v1`, to a provider whose Type
  lists it, with `program` and `archive` (`digest`, `size`, `urls` first a `file` URL of this
  order's copy), and the refusal naming item, provider and cause; `run.v2`'s reports, records
  (REC-002 as 274 amended it) and URL check (PRV-028, IFC-017); `check_schemas.py`'s PKG-020 and
  PKG-030 rules counting it; portable cases through the runner's provider steps. It may split at
  the dispatch.
- A package installed before this order has no copy; the second order's refusal names one.
  Moving its members out needs its archive, as from the person's cache: a later order's.
- Then, as order 274 lists them, the kit's and ssh-runtime's order, the packages' (DEC-051)
  and the vendor client's.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with order 274 stacked while it waited in the merge queue.

**Worker-2's turns** (09:17 to 09:49, three stops, each right) built the install side of
DEC-049: each installed archive is kept as distributed in `packages/HEX` beside `state.json`,
written atomically before the commit, for `install.v1` and `register --package` alike; members
are restored from the verified copy, and a copy that is absent, altered, of the wrong kind or
mode stops the store naming the package and the path; stores written before this order open
unchanged; `platforms` is validated at install (MAN-009, MAN-010, MAN-013). The stops, and the
coordinator's decisions, open to the owner's strike: decision 7's absolute growth limit could not
hold, since an install also keeps about 200 KB of invocation data, so the check measures the
members' share (a real archive's growth minus a minimal one's) and that no member's bytes are in
`state.json`; the loader rejects schema-invalid manifests before `resolve-project`, so those are
asserted with `load` steps; a case takes three manifests, so the fourth moved into
`archives-platform-programs`.

**Two blind reads at once** (09:51 to 09:55): `turn-verifier-2.turn.v1` found three musts (an
archive directory a later open would reject, platform names outside MAN-010's vocabulary, and
the growth check on one install path only), the Claude reader a should (a hand-written copy of
`State`'s serializer that could drop a later field) and a nit. One fix turn (09:57 to 10:10)
applied all five, each must reproduced first. Both cases, 181 core tests, the store and install
tests and the vectors test pass. The invocation data an install keeps is left for a later order.

The gate's run on the squash is the coordinator's, at the merge.
